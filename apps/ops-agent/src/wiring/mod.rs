//! ops-agent 的装配。
//!
//! ops-agent 持有 ep_ops_ro 池（上限 2），因此四项 SQL 自检对它成立；
//! 但 `ep-adapter-db-pg` 尚未提供 `SqlProbe` 实现，故不注入，自检报 PENDING。

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

    // 负样例断言的是「未注入即未覆盖」这条规则本身。
    #[tokio::test]
    async fn unwired_probe_yields_pending_not_passed() {
        let p = ProcessKind::OpsAgent;
        let report = baseline_registry(p, String::new(), 1_000, sql_probe()).run_all(p, "0.1.0").await;
        let item = report.items.iter().find(|i| i.name == "database-reachable").expect("项必须在报告里");
        assert_eq!(item.outcome, Outcome::Pending);
    }
}
