// ============================================================
// Layer 0：平台适配层 —— WASM / Android 入口
// ============================================================
// IPO 模型：
//   INPUT  → 各平台运行时的启动信号（浏览器加载 WASM / Android JNI 回调）
//   PROCESS→ 执行平台专属的初始化逻辑（如 Android 需先初始化 Slint 后端）
//   OUTPUT → 调用 controller::run() 启动统一的应用逻辑
//
// 约束（MECE 边界）：
//   ✗ 禁止包含任何业务计算逻辑
//   ✗ 禁止直接访问或修改 UI 组件
//   ✓ 唯一职责：感知平台 → 完成初始化 → 移交控制权
// ============================================================

// --- Web (WASM) 平台 ---

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

/// Web / WASM 平台入口
///
/// 当浏览器初始化 WASM 模块后，由 wasm-bindgen 自动调用此函数。
/// `index.html` 中只需执行 `init()`，无需手动调用此函数。
///
/// 参考官方模式：`#[cfg_attr(target_arch = "wasm32", wasm_bindgen(start))]`
#[cfg(target_arch = "wasm32")]
#[wasm_bindgen(start)]
pub fn wasm_entry() {
    crate::controller::run().unwrap();
}

// --- Android 平台 ---

/// Android 平台入口
///
/// 由 Android Activity 框架通过 JNI 调用。
/// 初始化顺序约束：必须先执行 `slint::android::init(app)` 完成 Slint
/// Android 后端的 Window 和渲染环境准备，之后才可创建任何 Slint 组件。
#[cfg(target_os = "android")]
#[no_mangle]
fn android_main(app: slint::android::AndroidApp) {
    // Step 1：初始化 Slint Android 后端（平台专属，此层唯一允许的初始化代码）
    slint::android::init(app).unwrap();
    // Step 2：移交控制权给公共控制器
    crate::controller::run().unwrap();
}
