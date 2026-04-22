/// 业务逻辑层 (Service / Logic Layer)
/// 
/// 这一层只负责处理纯粹的业务数据和核心逻辑，完全独立于 UI 框架 (Slint) 
/// 和操作系统平台 (Windows/Android)。
/// 这样设计的目的是为了实现极高的复用性和可测试性。

pub struct AppService;

impl AppService {
    /// 处理增加计数器的业务逻辑
    /// 
    /// 这里的逻辑非常简单（+1），但在真实项目中，这里可能包含复杂的：
    /// - 数据库访问
    /// - 远端 API 请求
    /// - 复杂算法计算
    pub fn increase_counter(current_value: i32) -> i32 {
        // 这里是纯粹的业务逻辑运算
        current_value + 1
    }
}

// ==========================================
// 单元测试
// 由于业务层与 UI 完全解耦，我们可以轻松地编写单元测试
// ==========================================
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_increase_counter() {
        assert_eq!(AppService::increase_counter(42), 43);
        assert_eq!(AppService::increase_counter(0), 1);
        assert_eq!(AppService::increase_counter(-1), 0);
    }
}

// ==========================================
// ASR 感知层引擎 (STT)
// ==========================================
use std::sync::mpsc;
use std::thread;
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};

pub struct AsrEngine {
    model_dir: String,
}

impl AsrEngine {
    pub fn new(model_dir: &str) -> Self {
        Self {
            model_dir: model_dir.to_string(),
        }
    }

    /// 启动后台录音和识别任务，不阻塞当前线程
    pub fn start(&self, sender: mpsc::Sender<String>) {
        let model_dir = format!("{}/sherpa-onnx-streaming-zipformer-bilingual-zh-en-2023-02-20", self.model_dir);
        
        thread::spawn(move || {
            // [真实逻辑]：构建 sherpa-onnx Recognizer
            // 配置识别器参数
            let encoder = format!("{}/encoder-epoch-99-avg-1.int8.onnx", model_dir);
            let decoder = format!("{}/decoder-epoch-99-avg-1.int8.onnx", model_dir);
            let joiner = format!("{}/joiner-epoch-99-avg-1.int8.onnx", model_dir);
            let tokens = format!("{}/tokens.txt", model_dir);
            
            // 为了防止无报错失败，我们需要确保这些文件真的存在
            if !std::path::Path::new(&encoder).exists() {
                let _ = sender.send("[系统] 模型文件未找到，请确认解压路径".to_string());
                return;
            }

            use sherpa_onnx::{OnlineRecognizer, OnlineRecognizerConfig, OnlineModelConfig};
            use sherpa_onnx::{OnlineTransducerModelConfig};

            let transducer = OnlineTransducerModelConfig {
                encoder: Some(encoder),
                decoder: Some(decoder),
                joiner: Some(joiner),
            };

            let mut model_config = OnlineModelConfig::default();
            model_config.transducer = transducer;
            model_config.tokens = Some(tokens);
            model_config.num_threads = 4;
            model_config.provider = Some("cpu".to_string());
            model_config.debug = false;

            let mut config = OnlineRecognizerConfig::default();
            config.model_config = model_config;

            let recognizer = OnlineRecognizer::create(&config).expect("[系统] 无法初始化语音识别模型");
            let mut stream = recognizer.create_stream();

            // 1. 初始化 cpal
            let host = cpal::default_host();
            let device = match host.default_input_device() {
                Some(device) => device,
                None => {
                    let _ = sender.send("[系统] 未找到麦克风设备".to_string());
                    return;
                }
            };
            
            let _ = sender.send("[系统] 引擎加载完毕，开始倾听...".to_string());

            let default_in_config = device.default_input_config().expect("无法获取默认输入配置");
            
            // 尝试通过宏绕过显式类型或者利用 Into 强制拆包
            // 某些 cpal 编译器版本将 SampleRate 视为 type alias = u32，另一些则是 SampleRate(u32)
            // 所以我们可以利用 `config()` 直接拿 u32 或者通过类型推导
            let mut cpal_config = default_in_config.config();
            cpal_config.channels = 1; // sherpa-onnx 需要单声道
            // 强行设死 16000 避免复杂的 cpal sample_rate 结构体冲突，且这是 sherpa 必须的
            // 不过如果是作为识别流输入，我们直接取 cpal_config 的值转换成 i32
            let sample_rate = cpal_config.sample_rate as i32;

            
            let (audio_tx, audio_rx) = mpsc::channel::<Vec<f32>>();
            
            let stream_handle = device.build_input_stream(
                &cpal_config,
                move |data: &[f32], _: &_| {
                    let _ = audio_tx.send(data.to_vec());
                },
                |err| eprintln!("录音发生错误: {}", err),
                None
            ).unwrap();
            
            stream_handle.play().unwrap();

            // 持续从 cpal 取出音频片段并喂给 Recognizer
            let mut last_text = String::new();
            while let Ok(data) = audio_rx.recv() {
                // sherpa-onnx 的流支持指定当前喂入的音频采样率
                stream.accept_waveform(sample_rate, &data);
                
                while recognizer.is_ready(&mut stream) {
                    recognizer.decode(&mut stream);
                }
                
                if let Some(result) = recognizer.get_result(&mut stream) {
                    let text = result.text.trim();
                    
                    if !text.is_empty() && text != last_text {
                        last_text = text.to_string();
                        let _ = sender.send(last_text.clone() + " ");
                    }
                }
                
                // 如果发现端点（一段话说完）
                if recognizer.is_endpoint(&mut stream) {
                    recognizer.reset(&mut stream);
                    last_text.clear();
                    let _ = sender.send("\n".to_string()); // 换行
                }
            }
        });
    }
}
