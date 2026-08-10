//! portal-gateway 的装配。
//!
//! 门户进程不建数据库连接，因此这里没有任何数据库一侧的注入点：
//! 四项 SQL 自检对它一律是 NotApplicable，不是「未注入」。

use std::sync::Arc;

use ep_platform_runtime::selfcheck::SqlProbe;

/// 恒为 None，且这一点由 [`ProcessKind::holds_sql_session`] 一侧独立成立：
/// 即便有人在这里注入探针，自检项也会先按「本进程不持 SQL 会话」判 NotApplicable。
pub fn sql_probe() -> Option<Arc<dyn SqlProbe>> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use ep_platform_runtime::ProcessKind;

    // 负样例断言的是「门户不持 SQL 会话」这条规则本身。
    #[test]
    fn portal_gateway_holds_no_sql_session() {
        assert!(!ProcessKind::PortalGateway.holds_sql_session());
        assert!(sql_probe().is_none());
    }
}
