// ============================================================
// 库根 (Library Root)
// ============================================================
// 职责：
//   1. 触发 build.rs 生成的 Slint UI 代码注入（Layer 1 的输出物）
//   2. 声明各架构层对应的子模块
//      注意：此文件本身不包含任何逻辑，只负责"组装"
// ============================================================

// 将 slint-build 在编译期生成的 Rust 绑定注入到当前 crate 根作用域
// 生成的类型（如 AppWindow）可通过 crate::AppWindow 被子模块引用
slint::include_modules!();

/// Layer 3：业务逻辑层
pub mod logic;

/// Layer 2：控制器层
pub mod controller;

/// Layer 0：平台适配层（WASM / Android 入口）
pub mod platform;
