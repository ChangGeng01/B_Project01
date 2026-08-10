//! 数据库一侧的装配。
//!
//! 阶段 1 的 `ep-adapter-db-pg` 尚未提供 `SqlProbe` 的实现，因此这里不注入。
//! 不注入的后果是四项 SQL 自检报 PENDING（未覆盖），不是 PASSED——
//! 这正是要的结果：读不到被测对象时绝不判通过。适配层交付实现后，
//! 本函数返回 Some，四项自检随即产生实质判定，调用点一行不动。

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

    // 负样例断言的是「未注入即未覆盖」这条规则本身：只要 sql_probe 还返回 None，
    // 四项 SQL 自检就必须是 PENDING，任何一项变成 PASSED 都说明有人加了空实现。
    #[tokio::test]
    async fn unwired_probe_yields_pending_not_passed() {
        let reg = baseline_registry(ProcessKind::CoreServer, String::new(), 1_000, sql_probe());
        let report = reg.run_all(ProcessKind::CoreServer, "0.1.0").await;
        for name in [
            "database-reachable",
            "migration-version-matched",
            "rls-enabled-and-forced",
            "runtime-role-privileges-bounded",
        ] {
            let item = report.items.iter().find(|i| i.name == name).expect("项必须在报告里");
            assert_eq!(item.outcome, Outcome::Pending, "{name}");
        }
    }
}
