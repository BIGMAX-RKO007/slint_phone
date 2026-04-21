# SLINT_PHONE — Slint 跨平台 GUI 示例项目

> **核心开发理念**：基于 MECE 原则（相互独立、完全穷尽）与 IPO 模型（输入-处理-输出），
> 从全局架构向下切分最小工作包，确保每一层的职责边界清晰、解耦彻底、全平台兼容。

---

## 一、什么是 SLINT_PHONE？

SLINT_PHONE 是一个基于 **Rust + Slint** 的跨平台 GUI Demo 项目，目标是同一套业务代码，
无需修改即可运行于：

| 平台 | 运行方式 | 构建命令 |
|------|---------|---------|
| 🖥️ **Windows 桌面** | 原生可执行文件 | `cargo run` |
| 🌐 **Web 浏览器** | WebAssembly | `wasm-pack build --target web` |
| 📱 **Android 手机** | 原生 APK | `cargo apk run --lib` |

---

## 二、架构总览（MECE 分层）

基于 MECE 原则，整个项目被切分为 **4 层**，每层职责相互独立、合起来覆盖全部功能：

```
┌─────────────────────────────────────────────────────────┐
│                  Layer 0：平台适配层                      │
│         (Platform Adapter / Entry Point Layer)           │
│  职责：感知运行平台，提供对应的程序入口点                    │
│  文件：src/lib.rs (android_main, WASM start), src/main.rs │
├─────────────────────────────────────────────────────────┤
│                  Layer 1：UI 声明层                       │
│              (Declarative UI / View Layer)               │
│  职责：声明界面结构与外观，定义事件回调接口（不含逻辑）        │
│  文件：ui/app-window.slint                               │
├─────────────────────────────────────────────────────────┤
│                  Layer 2：控制器层                        │
│              (Controller / Glue Code Layer)              │
│  职责：连接 UI 事件与业务逻辑，是唯一知道两侧存在的中间人      │
│  文件：src/lib.rs → pub fn main()                        │
├─────────────────────────────────────────────────────────┤
│                  Layer 3：业务逻辑层                      │
│             (Business Logic / Service Layer)             │
│  职责：执行纯粹的数据运算，完全不依赖 UI 框架或平台 API       │
│  文件：src/logic.rs                                      │
└─────────────────────────────────────────────────────────┘
```

> **MECE 校验**：
> - **相互独立**：每层之间只通过明确定义的接口通信（回调函数 / 函数参数），不存在跨层直接调用。
> - **完全穷尽**：从用户点击屏幕到数字改变，所有经过的逻辑必然落在且仅落在其中一层。

---

## 三、每层的 IPO 分析（黑盒思维）

### Layer 0：平台适配层

```
INPUT  → 操作系统/浏览器/Android JNI 的运行时信号（程序启动）
PROCESS→ 根据编译目标（cfg），选择对应的初始化路径：
          · wasm32   → #[wasm_bindgen(start)] 自动触发
          · android  → android_main() + slint::android::init()
          · desktop  → fn main() in main.rs
OUTPUT → 调用统一的 pub fn main()（控制器层入口）
```

**关键约束**：此层禁止包含任何业务逻辑。它唯一的工作是"插电"。

---

### Layer 1：UI 声明层

```
INPUT  → 设计规范（要展示什么，要有什么交互）
PROCESS→ 用 Slint DSL 声明组件结构、属性绑定、回调接口
OUTPUT → 编译期生成 Rust 绑定代码（AppWindow struct），
          暴露：get_counter(), set_counter(), on_request_increase_value()
```

**关键约束**：`.slint` 文件中禁止包含业务计算逻辑（如 `counter + 1` 不应在此处）。
UI 只负责"展示"与"事件上报"，不做决策。

---

### Layer 2：控制器层

```
INPUT  → UI 层上报的事件（用户点击按钮）+ 当前 UI 状态（get_counter()）
PROCESS→ 将 UI 事件路由到业务逻辑层，接收计算结果
OUTPUT → 将新状态写回 UI（set_counter()），驱动界面更新
```

**关键约束**：控制器层禁止自己进行业务计算（禁止写 `current + 1`）。
它是"快递员"，只负责传递，不打开包裹。

---

### Layer 3：业务逻辑层

```
INPUT  → 当前计数值（i32）
PROCESS→ 执行业务规则（当前为 +1，未来可扩展为任意规则）
OUTPUT → 新的计数值（i32）
```

**关键约束**：此层禁止 `use slint::*`，禁止调用任何平台 API。
它是纯函数，给什么数据就输出什么结果，可以脱离 UI 独立单元测试。

---

## 四、目录结构

```
slint_phone/
├── src/
│   ├── main.rs       # [桌面入口] Layer 0 — 桌面平台适配，调用 slint_phone::main()
│   ├── lib.rs        # [核心枢纽] Layer 0+2 — 平台适配 + 控制器逻辑
│   └── logic.rs      # [业务核心] Layer 3 — 纯业务逻辑，无平台依赖
├── ui/
│   └── app-window.slint  # [界面定义] Layer 1 — UI 声明，事件接口定义
├── index.html        # [Web 宿主] Web 平台的 HTML 容器，提供 <canvas id="canvas">
├── build.rs          # [构建脚本] 触发 slint-build 将 .slint 编译为 Rust 代码
└── Cargo.toml        # [依赖配置] 多平台条件依赖管理
```

---

## 五、全平台开发检查清单

在修改任何代码之前，**必须过一遍以下检查**（避免"查了东墙补西墙"）：

### 修改 `logic.rs` 时
- [ ] 函数签名变更是否向后兼容？（影响 Layer 2 的调用方式）
- [ ] 新增逻辑是否有对应的单元测试？（`cargo test`）
- [ ] 是否引入了任何平台特定依赖？（禁止）

### 修改 `lib.rs` 的控制器部分时
- [ ] 事件绑定逻辑是否在三个平台上都能触发？
- [ ] 是否混入了业务逻辑？（应提取到 `logic.rs`）
- [ ] `ui.run()` 的调用方式是否符合 Slint 官方文档？

### 修改 `lib.rs` 的平台适配部分时
- [ ] `#[cfg(...)]` 条件是否覆盖了全部预期平台？（wasm32 / android / desktop）
- [ ] 各平台的初始化顺序是否正确？（Android 必须先 `init(app)` 再 `main()`）
- [ ] Web 平台入口是否符合 `#[cfg_attr(target_arch = "wasm32", wasm_bindgen(start))]` 模式？

### 修改 `.slint` UI 文件时
- [ ] 新增的属性/回调名称是否已在 `lib.rs` 中绑定？
- [ ] UI 中是否混入了业务计算逻辑？（应移到 `logic.rs`）
- [ ] 重命名属性后是否同步更新了 `lib.rs` 中的 `get_*`/`set_*`/`on_*` 调用？

---

## 六、构建与运行

### 桌面端
```bash
cargo run
```

### Web 端
```bash
# 编译 WASM
wasm-pack build --target web --release

# 启动本地服务器（必须通过 HTTP 访问，不能直接打开 HTML 文件）
python -m http.server 8080

# 浏览器访问
# http://localhost:8080/
```

### Android 端
```bash
# 安装 Android 目标工具链（首次）
rustup target add aarch64-linux-android

# 连接设备并运行（需要 cargo-apk）
cargo apk run --target aarch64-linux-android --lib
cargo apk run --target aarch64-linux-android --lib --release

```
方法二：使用 xbuild (跨平台构建工具)
x devices
x run --device <id>
x build --platform android --arch arm64 --format apk --release

---

## 七、关键设计决策记录

| 决策 | 原因 | 影响的平台 |
|------|------|---------|
| `crate-type = ["cdylib", "rlib"]` | `cdylib` 支持 Android/WASM 动态库，`rlib` 支持桌面静态链接 | 全平台 |
| `wasm-bindgen` 仅在 `wasm` 目标引入 | 避免污染桌面/Android 构建 | Web |
| `slint = { features = ["backend-android-activity-06"] }` | Android 特性必须显式启用 | Android |
| 业务逻辑放在 `logic.rs` 而非 `lib.rs` | 解耦，使 `logic.rs` 可独立测试 | 全平台 |
| UI 入口统一为 `pub fn main()` | 三端共用同一控制器逻辑，减少平台分支代码 | 全平台 |

---

*文档版本：v1.0 · 基于 MECE + IPO 架构原则*

---

## 八、架构已知待优化项（MECE 诚实记录）

> **诚实是架构文档的核心**。现有设计存在以下 MECE 不完美之处，
> 已知且已接受，待项目规模扩大后优先重构。

### ⚠️ 问题 1：`lib.rs` 承担了两层职责

**现状**：`lib.rs` 同时包含 Layer 0（平台适配）和 Layer 2（控制器）的代码。
这违反了 MECE 的"相互独立"原则——两层的代码物理上混居在同一文件中。

```
当前（不纯粹）：
lib.rs = [Layer 0：android_main / wasm entry]
       + [Layer 2：pub fn main() 控制器逻辑]

理想（完全解耦）：
platform.rs = [Layer 0：仅平台适配代码]
controller.rs = [Layer 2：仅控制器逻辑]
```

**接受原因**：Demo 项目体量小，拆分收益有限。当控制器逻辑超过 50 行时，
应将 `pub fn main()` 迁移到独立的 `controller.rs`。

---

### ⚠️ 问题 2：Layer 3 缺少 Trait 接口定义

**现状**：`AppService` 是直接实现的具体结构体，没有对应的 Trait 接口。
这意味着 Layer 2 直接依赖了 Layer 3 的具体实现，无法在测试中注入 Mock。

**理想状态**：

```rust
// logic.rs 中应先定义接口
pub trait CounterService {
    fn increase_counter(current: i32) -> i32;
}

// 再提供具体实现
pub struct AppService;
impl CounterService for AppService { ... }
```

**接受原因**：当前功能简单，Mock 测试尚无必要。功能复杂化后立即引入。

---

## 九、构建管道 IPO 分析

> 构建管道本身也是一个完整的 IPO 系统，需要同等对待。

### 桌面构建管道

```
INPUT  → src/*.rs + ui/*.slint + Cargo.toml
PROCESS→ ① slint-build (build.rs) 将 .slint 编译为 Rust 代码
         ② rustc 编译全部 Rust 代码
         ③ 链接器生成可执行文件
OUTPUT → slint_phone.exe（Windows 原生二进制）
```

### Web 构建管道

```
INPUT  → src/lib.rs + ui/*.slint + Cargo.toml
PROCESS→ ① slint-build 将 .slint 编译为 Rust 代码
         ② rustc 交叉编译到 wasm32-unknown-unknown 目标
         ③ wasm-bindgen 生成 JS 胶水代码
         ④ wasm-opt 优化 WASM 体积
OUTPUT → pkg/slint_phone_bg.wasm（WASM 模块）
          pkg/slint_phone.js（JS 胶水 + 类型绑定）
```

### Android 构建管道

```
cargo install --git https://github.com/rust-mobile/xbuild.git
x devices
x run --device <id>
x build --platform android --arch arm64 --format apk --release
  target/x/release/android/<name>.apk

INPUT  → src/lib.rs + ui/*.slint + Cargo.toml + Manifest.toml
PROCESS→ ① slint-build 将 .slint 编译为 Rust 代码
         ② rustc 交叉编译到 aarch64-linux-android 目标
         ③ cargo-apk 包装为 .so 动态库
         ④ Android 构建工具打包 APK 并签名
OUTPUT → *.apk（Android 安装包）
```

> **关键洞察**：三条管道的 INPUT 有 90% 重叠（同一套 Rust+Slint 代码），
> 差异只在 PROCESS 阶段的编译目标和 OUTPUT 格式。
> 这正是"一次编写，处处运行"架构的价值所在。

---

## 十、变更影响矩阵

> 改任何一处之前，查此表，明确连带影响范围。✅ 有影响 ｜ ➖ 无影响

| 修改位置 → 受影响层 | Layer 0 平台适配 | Layer 1 UI 声明 | Layer 2 控制器 | Layer 3 业务逻辑 | Web 构建 | Android 构建 | 桌面构建 |
|---|---|---|---|---|---|---|---|
| `logic.rs` 函数签名 | ➖ | ➖ | ✅ 调用方式改变 | ✅ 本层 | ➖ | ➖ | ➖ |
| `logic.rs` 内部实现 | ➖ | ➖ | ➖ | ✅ 本层 | ➖ | ➖ | ➖ |
| `.slint` 属性/回调名 | ➖ | ✅ 本层 | ✅ get/set/on 调用 | ➖ | ✅ 重新编译 | ✅ 重新编译 | ✅ 重新编译 |
| `lib.rs` 控制器逻辑 | ➖ | ➖ | ✅ 本层 | ➖ | ✅ 重新编译 | ✅ 重新编译 | ✅ 重新编译 |
| `lib.rs` 平台适配 | ✅ 本层 | ➖ | ➖ | ➖ | ✅ 影响 Web 入口 | ✅ 影响 Android 入口 | ✅ 影响桌面入口 |
| `Cargo.toml` 依赖 | ➖ | ➖ | ➖ | ➖ | ✅ 可能破坏 | ✅ 可能破坏 | ✅ 可能破坏 |
| `index.html` | ➖ | ➖ | ➖ | ➖ | ✅ Web 宿主 | ➖ | ➖ |

**使用方法**：找到你要修改的行，所有 ✅ 列都需要在修改后验证。

---

## 十一、各层测试策略

> 每一层应有与其解耦程度相匹配的测试方式。

### Layer 3：业务逻辑层 ← **唯一可纯单元测试的层**

```bash
cargo test
```

- ✅ 完全不需要 UI、不需要平台、不需要网络
- ✅ 可在 CI/CD 中对所有平台并行运行
- 📌 **要求**：每一个公开函数必须有对应的 `#[test]`

### Layer 2：控制器层 ← **集成测试**

- 当前无自动化测试（UI 事件绑定难以 Mock）
- 📌 **验证方式**：每次修改后，在三个平台分别手动触发交互，确认数值变化
- 📌 **未来改进**：引入 Trait 后，可用 Mock Service 进行集成测试

### Layer 1：UI 声明层 ← **视觉回归测试**

- 📌 **验证方式**：截图对比（当前手动）
- 📌 **未来改进**：接入 Slint 的 `slint-testing` 库进行自动化 UI 测试

### Layer 0：平台适配层 ← **端到端测试**

- 📌 **验证方式**：
  - Desktop：`cargo run` → 窗口正常弹出
  - Web：`wasm-pack build` → 浏览器打开 → 开发者工具无 ERROR 级别报错
  - Android：`cargo apk run` → 手机界面正常显示

---

## 十二、新增功能的标准操作流程

> 严格按此顺序操作，确保每步都不污染其他层。

**示例：新增"重置计数器"按钮**

```
Step 1 ── [Layer 3] logic.rs
          定义业务规则：增加 AppService::reset_counter() -> i32
          ↓ 立即编写单元测试，cargo test 通过后再继续

Step 2 ── [Layer 1] app-window.slint
          声明新的 callback request-reset-value()
          添加 Reset 按钮组件，绑定 clicked => root.request-reset-value()

Step 3 ── [Layer 2] lib.rs（控制器部分）
          增加事件绑定：ui.on_request_reset_value(move || { ... })
          调用 logic::AppService::reset_counter()，将结果写回 UI

Step 4 ── [Layer 0] 无需改动
          （功能扩展不应触及平台适配层）

Step 5 ── [全平台验证]
          · cargo run          → 桌面测试 Reset 按钮
          · wasm-pack build    → 浏览器测试
          · cargo apk run      → Android 测试（如适用）
```

**黄金法则**：永远从 Layer 3 开始写，永远从 Layer 0 结束测试。

---

*文档版本：v1.1 · 新增：MECE 诚实记录 / 构建管道 IPO / 变更影响矩阵 / 测试策略 / 扩展流程*

---

## 十三、常见踩坑记录 (Troubleshooting)

### Android 平台：自定义应用图标与应用名不生效的问题

**症状**：你在 `Cargo.toml` 中配置了 `[package.metadata.android]` 下的 `apk_label` 和 `icon` 属性，但打包出 APK 后，安装到手机上不仅显示的是默认的安卓蓝绿小人，应用桌面里显示的名字也全都是代码结构里默认的英文小写（如 `slint_phone`）。

**原因（`cargo-apk` 与 `android-activity` 兼容性巨坑）**：
由于 Slint 的 Android 后端引擎采用的是相对底层的 `android-activity` crate，在此套件接管 JNI 和生命周期后，原版 `cargo-apk` 对打包清单 (`AndroidManifest.xml`) 标签生成机制发生了非常严苛的变化。它要求将应用的显示属性放在更加特定的 `application` 子表中！写在 `[package.metadata.android]` 下的基础 `icon` 参数会被完全**静默丢弃**。

**解决办法**：
必须在 `Cargo.toml` 中提供精确到 `[package.metadata.android.application]` 层级的映射。同时，推荐图标搭配多分辨率文件夹 (mipmap) 加载，以规避不同厂商启动器的图片解码水土不服：

```toml
# 第一步：仅用于指定 res 资源目录所在的相对位置
[package.metadata.android]
resources = "res"

# 第二步：将具体的桌面展示属性独立配置到专用子表中
[package.metadata.android.application]
label = "SlintPhone"           # 控制桌面显示正确的应用名称
icon = "@mipmap/ic_launcher"   # 使用标准安卓引用方法让系统读取 res/mipmap-* 下的 PNG
```

*(注意：请确保你已在项目根目录中建立了 `res/mipmap-mdpi` 乃至 `xxxhdpi` 各大尺寸的高清 `.png` 资源)*
