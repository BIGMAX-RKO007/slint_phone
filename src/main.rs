// ============================================================
// Layer 0：平台适配层 —— 桌面端入口
// ============================================================
// IPO 模型：
//   INPUT  → 操作系统启动信号（Windows / macOS / Linux 程序入口）
//   PROCESS→ （无特殊初始化，桌面平台由 Slint 直接支持）
//   OUTPUT → 调用 controller::run() 启动应用
//
// 约束（MECE 边界）：
//   ✗ 禁止包含任何业务逻辑
//   ✗ 禁止直接操作 UI 组件
//   ✓ 此文件只应有 main() 函数，且只有一行有效调用
// ============================================================

// 在 release 模式下，隐藏 Windows 系统的命令行黑窗口
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    slint_phone::controller::run().unwrap();
}
