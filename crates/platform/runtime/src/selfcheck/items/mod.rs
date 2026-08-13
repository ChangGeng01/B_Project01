//! 基线十项的注册。名字与分级的唯一出处是技术基线第 7.3 节与裁定 C-25。

pub mod basic;
pub mod sql;

use std::sync::Arc;

use crate::process::ProcessKind;
use crate::selfcheck::probe::SqlProbe;
use crate::selfcheck::registry::{SelfCheckItem, SelfCheckRegistry, Severity};
use crate::selfcheck::secrets::{SecretsProbe, SecretsResolvable};
use ep_platform_obs::degradation::DegradationLedger;

use basic::{ClockSkewWithinLimit, ConfigParsed, PendingItem};
use sql::{
    DatabaseReachable, MigrationVersionMatched, RlsEnabledAndForced, RuntimeRolePrivilegesBounded,
    SqlContext,
};

/// 基线十项的注册名与分级，前八 Blocking、后两 Degrading。
/// 本阶段注册前九项，`offsite-sink-requirements` 整条推迟到阶段 14 一次登记，
/// 因此这里既不登记也不留占位——留一个恒 Pending 的占位等于新增一条永不减少的项。
pub const BASELINE_ITEMS: [(&str, Severity); 10] = [
    ("config-parsed", Severity::Blocking),
    ("database-reachable", Severity::Blocking),
    ("migration-version-matched", Severity::Blocking),
    ("rls-enabled-and-forced", Severity::Blocking),
    ("runtime-role-privileges-bounded", Severity::Blocking),
    ("secrets-resolvable", Severity::Blocking),
    ("file-store-writable", Severity::Blocking),
    ("clock-skew-within-limit", Severity::Blocking),
    ("audit-chain-verifiable", Severity::Degrading),
    ("offsite-sink-requirements", Severity::Degrading),
];

/// 本阶段注册的九项，顺序即 [`BASELINE_ITEMS`] 去掉最后一项。
pub const REGISTERED_THIS_STAGE: usize = 9;

/// 构造一个进程的基线注册表。
///
/// `probe` 为 None 时 SQL 类四项报 Pending 而不是通过，见 [`sql::SqlContext`]。
/// `secrets` 与 `ledger` 为阶段 2 接入 `secrets-resolvable` 两段式判定的注入点，
/// 见 [`crate::selfcheck::secrets`]；两者都为 None 时该项报 Pending（未覆盖）。
pub fn baseline_registry(
    process: ProcessKind,
    layers: String,
    clock_skew_max_ms: u32,
    probe: Option<Arc<dyn SqlProbe>>,
    secrets: Option<Arc<dyn SecretsProbe>>,
    ledger: Option<Arc<dyn DegradationLedger>>,
) -> SelfCheckRegistry {
    let ctx = Arc::new(SqlContext::new(process, probe));
    let mut reg = SelfCheckRegistry::new();
    let items = vec![
        SelfCheckItem::new(
            "config-parsed",
            "配置解析成功且无未知键",
            Severity::Blocking,
            Arc::new(ConfigParsed { layers }),
        ),
        SelfCheckItem::new(
            "database-reachable",
            "事务数据库可达且服务端参数满足最低要求",
            Severity::Blocking,
            Arc::new(DatabaseReachable(ctx.clone())),
        ),
        SelfCheckItem::new(
            "migration-version-matched",
            "迁移清单与二进制期望一致",
            Severity::Blocking,
            Arc::new(MigrationVersionMatched {
                ctx: ctx.clone(),
                expected: env!("EP_MIGRATION_MANIFEST_SHA256"),
                dir_present: env!("EP_MIGRATION_DIR_PRESENT") == "1",
            }),
        ),
        SelfCheckItem::new(
            "rls-enabled-and-forced",
            "带法人列的表均已 ENABLE 且 FORCE 行级安全",
            Severity::Blocking,
            Arc::new(RlsEnabledAndForced(ctx.clone())),
        ),
        SelfCheckItem::new(
            "runtime-role-privileges-bounded",
            "运行期账号不具备 DDL、角色管理与策略管理权限",
            Severity::Blocking,
            Arc::new(RuntimeRolePrivilegesBounded(ctx)),
        ),
        SelfCheckItem::new(
            "secrets-resolvable",
            "机密全部可解引用且各法人密钥域齐备",
            Severity::Blocking,
            Arc::new(SecretsResolvable {
                process,
                probe: secrets,
                ledger,
            }),
        ),
        SelfCheckItem::new(
            "file-store-writable",
            "文件存储路径可写且权限受限",
            Severity::Blocking,
            Arc::new(PendingItem { owner: "阶段 3b" }),
        ),
        SelfCheckItem::new(
            "clock-skew-within-limit",
            "系统时钟与授时源偏差小于阈值",
            Severity::Blocking,
            Arc::new(ClockSkewWithinLimit {
                max_ms: clock_skew_max_ms,
            }),
        ),
        SelfCheckItem::new(
            "audit-chain-verifiable",
            "审计链最近一段可读且段根签名可验证",
            Severity::Degrading,
            Arc::new(PendingItem { owner: "阶段 3b" }),
        ),
    ];
    for item in items {
        // 名字是编译期常量，重名只可能是本函数自己写错，因此这里不吞错。
        reg.register(item).expect("基线自检项不得重名");
    }
    reg
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::selfcheck::registry::Outcome;

    #[test]
    fn registration_order_matches_the_baseline_roster() {
        let reg = baseline_registry(
            ProcessKind::CoreServer,
            "内置默认".into(),
            1_000,
            None,
            None,
            None,
        );
        let want: Vec<&str> = BASELINE_ITEMS
            .iter()
            .take(REGISTERED_THIS_STAGE)
            .map(|(n, _)| *n)
            .collect();
        assert_eq!(reg.names(), want);
    }

    #[test]
    fn offsite_sink_requirements_is_not_registered_this_stage() {
        let reg = baseline_registry(
            ProcessKind::CoreServer,
            String::new(),
            1_000,
            None,
            None,
            None,
        );
        assert!(
            !reg.names().contains(&"offsite-sink-requirements"),
            "本阶段整条推迟，不留占位"
        );
        assert_eq!(reg.len(), REGISTERED_THIS_STAGE);
    }

    #[tokio::test]
    async fn processes_without_sql_session_mark_sql_items_not_applicable() {
        for p in [
            ProcessKind::PortalGateway,
            ProcessKind::PluginHost,
            ProcessKind::ArchiveWriter,
            ProcessKind::BackupWriter,
        ] {
            let report = baseline_registry(p, String::new(), 1_000, None, None, None)
                .run_all(p, "0.1.0")
                .await;
            for name in [
                "database-reachable",
                "migration-version-matched",
                "rls-enabled-and-forced",
                "runtime-role-privileges-bounded",
            ] {
                let item = report
                    .items
                    .iter()
                    .find(|i| i.name == name)
                    .expect("项必须在报告里");
                assert_eq!(
                    item.outcome,
                    Outcome::NotApplicable,
                    "{} 的 {name}",
                    p.name()
                );
            }
        }
    }

    #[tokio::test]
    async fn sql_items_without_a_probe_are_pending_not_passed() {
        let p = ProcessKind::CoreServer;
        let report = baseline_registry(p, String::new(), 1_000, None, None, None)
            .run_all(p, "0.1.0")
            .await;
        let item = report
            .items
            .iter()
            .find(|i| i.name == "database-reachable")
            .unwrap();
        assert_eq!(item.outcome, Outcome::Pending, "探针未装配时绝不判通过");
        assert!(item.detail.contains("未覆盖"));
    }
}
