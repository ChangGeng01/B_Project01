//! ops-agent 的装配。
//!
//! 阶段 1 计划把本进程描述为持有 `ep_ops_ro` 池（上限 2），
//! 故 `ProcessKind::holds_sql_session` 对它返真，四项 SQL 自检对它成立。
//!
//! 但**今天本 crate 不依赖 `ep-adapter-db-pg`、无池构造、无 `SqlProbe` 适配器**
//! （全仓两处 `impl SqlProbe` 都在 core-server 与 job-worker 自己的 wiring 里），
//! 故不注入，四项 SQL 自检报 PENDING。
//!
//! 「声明持库连接、连接预算记 2 条、而无任何数据库装配」这一不一致
//! 由裁定 F-34 登记；本注不再把它写成「适配层尚未提供」——那句话字面为真，
//! 但会让人以为等适配层就行，实际缺的是本进程自己的整套装配。

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
        let report = baseline_registry(p, String::new(), 1_000, sql_probe(), None, None)
            .run_all(p, "0.1.0")
            .await;
        let item = report
            .items
            .iter()
            .find(|i| i.name == "database-reachable")
            .expect("项必须在报告里");
        assert_eq!(item.outcome, Outcome::Pending);
    }
}
