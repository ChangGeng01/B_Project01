//! 四项 SQL 类自检的判定逻辑。
//!
//! 三种前提各自有不同的结论，不得混同：
//! 进程不持 SQL 会话 → NotApplicable；持有但探针未装配 → Pending（未覆盖）；
//! 探针取数失败 → Fail。把「读不到」讲成「通过」是本卷已修过四次的缺陷。

use std::sync::Arc;

use crate::process::ProcessKind;
use crate::selfcheck::probe::{manifest_sha256, SqlProbe};
use crate::selfcheck::registry::{SelfCheckRun, Verdict};

/// 服务端最低要求，出处是技术基线第 7.3 节 `database-reachable` 一条。
pub const MIN_MAX_CONNECTIONS: u32 = 52;
pub const MIN_MAX_WAL_SENDERS: u32 = 4;
pub const MIN_MAX_REPLICATION_SLOTS: u32 = 3;
pub const REQUIRED_SERVER_MAJOR: &str = "16.";

/// 四项 SQL 自检共用的前置判定。
pub struct SqlContext {
    process: ProcessKind,
    probe: Option<Arc<dyn SqlProbe>>,
}

impl SqlContext {
    pub fn new(process: ProcessKind, probe: Option<Arc<dyn SqlProbe>>) -> Self {
        Self { process, probe }
    }

    /// 返回 Err 时是已定结论，调用方直接上报，不再往下判。
    fn probe(&self) -> Result<&Arc<dyn SqlProbe>, Verdict> {
        if !self.process.holds_sql_session() {
            return Err(Verdict::NotApplicable(format!(
                "{} 不持有常规数据库连接，SQL 类自检项不成立",
                self.process.name()
            )));
        }
        self.probe.as_ref().ok_or_else(|| {
            Verdict::Pending(
                "未装配 SQL 探针：判定逻辑已就位，取数实现由 ep-adapter-db-pg 提供，本项未覆盖".into(),
            )
        })
    }
}

macro_rules! probe_or_return {
    ($ctx:expr) => {
        match $ctx.probe() {
            Ok(p) => p.clone(),
            Err(verdict) => return verdict,
        }
    };
}

pub struct DatabaseReachable(pub Arc<SqlContext>);

#[async_trait::async_trait]
impl SelfCheckRun for DatabaseReachable {
    async fn run(&self) -> Verdict {
        let probe = probe_or_return!(self.0);
        let s = match probe.server_settings().await {
            Ok(s) => s,
            Err(e) => return Verdict::Fail(format!("数据库不可达：{e}")),
        };
        let mut bad = Vec::new();
        if !s.server_version.starts_with(REQUIRED_SERVER_MAJOR) {
            bad.push(format!("服务端版本 {} 不是 16.x", s.server_version));
        }
        if s.timezone != "UTC" {
            bad.push(format!("timezone 为 {} 而非 UTC", s.timezone));
        }
        if s.max_connections < MIN_MAX_CONNECTIONS {
            bad.push(format!("max_connections 为 {} 低于 {MIN_MAX_CONNECTIONS}", s.max_connections));
        }
        if s.max_wal_senders < MIN_MAX_WAL_SENDERS {
            bad.push(format!("max_wal_senders 为 {} 低于 {MIN_MAX_WAL_SENDERS}", s.max_wal_senders));
        }
        if s.max_replication_slots < MIN_MAX_REPLICATION_SLOTS {
            bad.push(format!(
                "max_replication_slots 为 {} 低于 {MIN_MAX_REPLICATION_SLOTS}",
                s.max_replication_slots
            ));
        }
        if bad.is_empty() {
            Verdict::Pass(format!("PostgreSQL {}，参数满足最低要求", s.server_version))
        } else {
            Verdict::Fail(bad.join("；"))
        }
    }
}

pub struct MigrationVersionMatched {
    pub ctx: Arc<SqlContext>,
    /// 编译期常量，由 build.rs 对 `db/migrations/` 算定。
    pub expected: &'static str,
    /// 构建时迁移目录是否存在。不存在时即使比对相等也要在 detail 里写明。
    pub dir_present: bool,
}

#[async_trait::async_trait]
impl SelfCheckRun for MigrationVersionMatched {
    async fn run(&self) -> Verdict {
        let probe = probe_or_return!(self.ctx);
        let rows = match probe.migration_rows().await {
            Ok(r) => r,
            Err(e) => return Verdict::Fail(format!("读迁移历史失败：{e}")),
        };
        let actual = manifest_sha256(&rows);
        if actual != self.expected {
            return Verdict::Fail(format!(
                "迁移清单不一致：库内 {} 条算得 {actual}，二进制期望 {}",
                rows.len(),
                self.expected
            ));
        }
        let note = if self.dir_present { "" } else { "（构建时 db/migrations/ 目录不存在，清单为空集）" };
        Verdict::Pass(format!("迁移清单一致，库内 {} 条{note}", rows.len()))
    }
}

pub struct RlsEnabledAndForced(pub Arc<SqlContext>);

#[async_trait::async_trait]
impl SelfCheckRun for RlsEnabledAndForced {
    async fn run(&self) -> Verdict {
        let probe = probe_or_return!(self.0);
        let state = match probe.rls_state().await {
            Ok(s) => s,
            Err(e) => return Verdict::Fail(format!("读 RLS 状态失败：{e}")),
        };
        let mut bad: Vec<String> = state
            .legal_entity_tables
            .iter()
            .filter(|t| !t.enabled || !t.forced)
            .map(|t| format!("{}.{} enabled={} forced={}", t.schema, t.table, t.enabled, t.forced))
            .collect();
        if state.current_role_bypassrls {
            bad.push("当前角色具备 BYPASSRLS".into());
        }
        if state.current_role_superuser {
            bad.push("当前角色是 SUPERUSER".into());
        }
        if bad.is_empty() {
            Verdict::Pass(format!("{} 张带法人列的表均已 ENABLE 且 FORCE", state.legal_entity_tables.len()))
        } else {
            Verdict::Fail(bad.join("；"))
        }
    }
}

pub struct RuntimeRolePrivilegesBounded(pub Arc<SqlContext>);

#[async_trait::async_trait]
impl SelfCheckRun for RuntimeRolePrivilegesBounded {
    async fn run(&self) -> Verdict {
        let probe = probe_or_return!(self.0);
        let p = match probe.role_privileges().await {
            Ok(p) => p,
            Err(e) => return Verdict::Fail(format!("读角色权限失败：{e}")),
        };
        let mut bad = Vec::new();
        if !p.schemas_with_create.is_empty() {
            bad.push(format!("在 {} 上具备 CREATE", p.schemas_with_create.join("、")));
        }
        if p.rolcreaterole {
            bad.push("具备 CREATEROLE".into());
        }
        if p.rolcreatedb {
            bad.push("具备 CREATEDB".into());
        }
        if bad.is_empty() {
            Verdict::Pass("运行期账号不具备 DDL、角色管理与策略管理权限".into())
        } else {
            Verdict::Fail(bad.join("；"))
        }
    }
}
