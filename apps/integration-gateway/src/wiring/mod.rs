//! integration-gateway 的装配。
//!
//! `ep-adapter-db-pg` 不提供 `SqlProbe` 实现——全仓两处 `impl SqlProbe` 分别在
//! core-server 与 job-worker **各自的 `wiring/probes.rs`**，是那两个 app 自建的
//! `FoundationProbeAdapter`，建在 `PgDataFoundationCheck` 之上。
//!
//! 本进程今天不依赖 `ep-adapter-db-pg`、无池、无该适配器，故不注入，
//! 四项 SQL 自检报 PENDING（未覆盖）而不是 PASSED。
//!
//! **原注写的「与 core-server 同理」已不成立**：core-server 早已自建适配器并注入
//! （`apps/core-server/src/wiring/db.rs` 的 `sql_probe` 逐字「装配成功即 Some，
//! 自检随即产生实质判定」）。本进程与它不同理，是**尚无任何数据库装配**。
//! 该处更正见裁定 F-34。

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
        let p = ProcessKind::IntegrationGateway;
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
