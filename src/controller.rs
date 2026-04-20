// ============================================================
// Layer 2：控制器层 (Controller / Glue Code Layer)
// ============================================================
// IPO 模型：
//   INPUT  → UI 层上报的用户交互事件 + 当前 UI 状态数据
//   PROCESS→ 将事件路由到业务逻辑层（Layer 3），接收计算结果
//   OUTPUT → 将新状态写回 UI 层（Layer 1），驱动界面刷新
//
// 约束（MECE 边界）：
//   ✗ 禁止自行执行任何业务计算（如直接写 counter + 1）
//   ✗ 禁止调用任何平台 API（如 android::init）
//   ✓ 是 Layer 1（UI）与 Layer 3（业务）之间唯一的中间人
//   ✓ 所有计算必须委托给 crate::logic 模块处理
// ============================================================

/// 初始化 UI 并运行事件循环
///
/// 此函数是所有平台适配层（Layer 0）的统一调用目标：
/// - 桌面端：由 `main.rs` 的 `fn main()` 调用
/// - WASM 端：由 `platform.rs` 的 `wasm_entry()` 调用
/// - Android 端：由 `platform.rs` 的 `android_main()` 调用
///
/// # 返回值
/// 返回 `slint::PlatformError`，由各平台入口点决定如何处理错误。
pub fn run() -> Result<(), slint::PlatformError> {
    use slint::ComponentHandle; // 必须导入该 trait 才能使用 as_weak() 和 run()
    
    // ── INPUT：实例化 UI 组件（使用 Layer 1 编译期生成的 AppWindow 类型）──
    let ui = crate::AppWindow::new()?;

    // ── PROCESS：绑定 UI 事件到业务逻辑路由 ──
    //
    // 模式：每一个 UI 事件绑定块的内部结构严格遵循 IPO：
    //   INPUT  → 从 UI 读取当前状态（get_*）
    //   PROCESS→ 委托给 Layer 3 计算新状态（crate::logic::*）
    //   OUTPUT → 将新状态写回 UI（set_*）

    let ui_handle = ui.as_weak();
    ui.on_request_increase_value(move || {
        let ui = ui_handle.unwrap();

        // INPUT：读取当前计数值
        let current_val = ui.get_counter();

        // PROCESS：委托 Layer 3 执行业务规则（本层不参与计算）
        let new_val = crate::logic::AppService::increase_counter(current_val);

        // OUTPUT：将结果写回 UI
        ui.set_counter(new_val);
    });

    // ── OUTPUT：启动 Slint 事件循环 ──
    // 在 WASM 环境下，Slint 内部已做特殊处理，不会阻塞浏览器主线程
    // 在桌面/Android 环境下，此调用阻塞直到窗口关闭
    ui.run()
}
