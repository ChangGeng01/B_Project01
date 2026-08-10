//! archive-writer 的装配。
//!
//! 本进程不持有运行期应用账号，也不建数据库连接，因此没有任何数据库一侧的
//! 注入点：四项 SQL 自检对它一律 NotApplicable。

use std::sync::Arc;

use ep_platform_runtime::selfcheck::SqlProbe;

pub fn sql_probe() -> Option<Arc<dyn SqlProbe>> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use ep_platform_runtime::selfcheck::{baseline_registry, Outcome};
    use ep_platform_runtime::ProcessKind;

    // 负样例断言的是「本进程不持 SQL 会话」这条规则本身。
    #[tokio::test]
    async fn sql_items_are_not_applicable_for_this_process() {
        let p = ProcessKind::ArchiveWriter;
        assert!(!p.holds_sql_session());
        assert!(sql_probe().is_none());
        let report = baseline_registry(p, String::new(), 1_000, None).run_all(p, "0.1.0").await;
        let item = report.items.iter().find(|i| i.name == "database-reachable").unwrap();
        assert_eq!(item.outcome, Outcome::NotApplicable);
    }
}
