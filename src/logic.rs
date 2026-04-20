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
