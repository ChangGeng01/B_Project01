//! 空库首装自举（鸡生蛋收口，裁定登记见 02 计划第 12 节偏离登记十四）。
//!
//! ## 鸡生蛋与收口口径
//!
//! `platform_core.migration_windows` 与 `migration_window_lock` 由迁移
//! V20260901093500 创建，而 apply 的窗口闸要求两表先存在且调用方出示窗口；
//! 空库首装因此死锁。活库补验曾以「ep_migrator 预施 090000/090500/093500
//! 三个文件」的临时口径绕过，本模块是其正路实现，临时口径自此作废。
//!
//! 触发条件（且仅在）：apply 未出示 `--window-id` 且目标库无
//! `{history_schema}.{history_table}`（沿用 [`crate::apply::read_history`]
//! 同款 `to_regclass` 探测）。自举在同一连接上以 ep_migrator 身份、单一事务内：
//! 1. 建 platform_core schema（存在性守护，属主与授权由第 1 号迁移归位）；
//!    属主归位前必须先授 ep_mod_platform_core 本 schema 的 CREATE 与 USAGE：
//!    ALTER TABLE ... OWNER TO 要求目标角色对表所在 schema 持有 CREATE，
//!    而第 1 号迁移的 `alter schema ... owner to` 尚未执行，不预授则属主
//!    归位失败；第 1 号迁移归位 schema 属主后该角色自然持有两权，
//!    预授是幂等无害的。
//! 2. 建 migration_windows / migration_window_lock 两表——列集、约束与
//!    V20260901093500 的最终形态逐字一致（本模块常量即从该迁移提取，
//!    单测逐字比对）；属主归位 ep_mod_platform_core，与迁移内
//!    `set role ep_mod_platform_core` 建表的属主形态一致；
//! 3. 建历史表（复用 [`crate::history::create_history_table_sql`]，与运行期
//!    路径同一常量，天然同形态）并同样属主归位；
//! 4. 插入单例锁行（与迁移同款 ON CONFLICT 幂等形态）；
//! 5. 插入一条一次性安装窗口行：state OPEN、approval_ref 取
//!    [`INITIAL_INSTALL_APPROVAL_REF`]、reason 写明首装自举、opened_by 取
//!    系统主体（与 open-window 同口径，见 [`crate::window`]）、ttl 取默认值。
//!
//! 随后 apply 以该窗口 id 走正常流程执行全部迁移。全部迁移成功后窗口不
//! 显式关闭，到期自动按过期口径由既有机制承担（open-window 对过期 OPEN
//! 窗口先关再开）。自举后重复 apply：历史表已存在，走正常比对路径，
//! 无待执行即退出码 0，不再自举。
//!
//! 出示了 `--window-id` 的调用永不走自举：空库上出示的窗口不可能存在，
//! 交由窗口闸以库侧事实拒绝，保住「出示窗口必校验」的纪律。

use tokio_postgres::Client;

use crate::cli::DEFAULT_TTL_MINUTES;
use crate::dbconn::SESSION_PREAMBLE;
use crate::exit::{MigrateExit, Outcome};
use crate::history::{align_history_owner_sql, create_history_table_sql};
use crate::window::SYSTEM_PRINCIPAL;

/// 首装自举窗口行的审批引用：一次性安装的固定取值（A-09 要求审批引用
/// 非空，首装不存在审批单，取本固定值并在偏离登记十四具名）。
pub const INITIAL_INSTALL_APPROVAL_REF: &str = "INITIAL_INSTALL";

/// 首装自举窗口行的 reason 列取值（受 ck_migration_windows_reason_len
/// 的 2000 字上限约束，本常量远低于上限，单测锁定）。
pub const INITIAL_INSTALL_REASON: &str = "空库首装自举：目标库无 platform_core.schema_history，\
ep-migrate 以 ep_migrator 身份建历史表与迁移窗口两表，并自开一次性安装窗口执行全部迁移。\
口径见 02 计划第 12 节偏离登记十四；到期按既有窗口生命周期机制自动关闭。";

/// 窗口表属主归位角色：与 V20260901093500 内 `set role ep_mod_platform_core`
/// 建表的最终属主形态一致。
const WINDOWS_OWNER_ROLE: &str = "ep_mod_platform_core";

/// migration_windows 建表语句。列集与约束逐字取自
/// V20260901093500__platform_core_migration_windows.sql 的最终形态
/// （DO 块内 execute 串的 `''` 转义还原为 `'`），仅外加 IF NOT EXISTS 守护；
/// 形态一致性由单测逐字比对锁定，改任何一处都必须同步另一处。
pub const CREATE_MIGRATION_WINDOWS_TABLE: &str = "create table if not exists \
platform_core.migration_windows (
  id uuid not null,
  security_level smallint not null default 20,
  data_scope_tags text[] not null default '{}',
  row_version bigint not null default 1,
  created_at timestamptz not null default now(),
  created_by uuid not null default '00000000-0000-7000-8000-000000000001',
  updated_at timestamptz not null default now(),
  updated_by uuid not null default '00000000-0000-7000-8000-000000000001',
  state text not null,
  approval_ref text not null,
  reason text not null,
  opened_by uuid not null,
  opened_at timestamptz not null,
  expires_at timestamptz not null,
  closed_by uuid null,
  closed_at timestamptz null,
  close_kind text null,
  applied_versions text[] not null default '{}',
  constraint pk_migration_windows primary key (id),
  constraint ck_migration_windows_state check (state in ('OPEN', 'CLOSED')),
  constraint ck_migration_windows_reason_len check (length(reason) <= 2000),
  constraint ck_migration_windows_expiry check (expires_at > opened_at),
  constraint ck_migration_windows_close_kind check (
    close_kind is null or close_kind in ('MANUAL', 'EXPIRED', 'FAILED'))
)";

/// migration_window_lock 建表语句。逐字取自同一迁移的最终形态：
/// 只有 id smallint 一列，不带公共列，行数由 check (id = 1) 固定为一行。
pub const CREATE_MIGRATION_WINDOW_LOCK_TABLE: &str = "create table if not exists \
platform_core.migration_window_lock (
  id smallint not null,
  constraint pk_migration_window_lock primary key (id),
  constraint ck_migration_window_lock_singleton check (id = 1)
)";

/// 单例锁行：与迁移同款 ON CONFLICT 幂等形态，开窗流程据此加行锁。
const INSERT_LOCK_ROW: &str =
    "insert into platform_core.migration_window_lock (id) values (1) on conflict (id) do nothing";

/// 首装窗口行插入语句。参数形态与 open-window 的插入逐款一致
/// （UUID 以文本绑定并经 ::text::uuid 两步显式转型，见 [`crate::window`]）。
pub const INSERT_BOOTSTRAP_WINDOW: &str = "insert into platform_core.migration_windows \
     (id, state, approval_ref, reason, opened_by, opened_at, expires_at) \
     values ($1::text::uuid, 'OPEN', $2, $3, $4::text::uuid, now(), \
             now() + make_interval(mins => $5)) \
     returning id::text";

/// apply 的入口判定（纯函数，无活库可测）：历史表是否存在 × 是否出示窗口。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ApplyEntry {
    /// 空库且未出示窗口：进首装自举。
    Bootstrap,
    /// 出示了窗口：读库判 OPEN（空库上必然失败，纪律不变）。
    PresentedWindow,
    /// 非空库且未出示窗口：走正常比对路径；有待执行迁移才落窗口闸 3。
    Bare,
}

/// 入口判定的唯一出处。自举只在「无历史表且未出示窗口」一格触发。
pub fn decide_entry(history_exists: bool, window_presented: bool) -> ApplyEntry {
    match (history_exists, window_presented) {
        (false, false) => ApplyEntry::Bootstrap,
        (_, true) => ApplyEntry::PresentedWindow,
        (true, false) => ApplyEntry::Bare,
    }
}

/// 自举事务的 DDL 语句序列（不含窗口行插入，插入带参数单列）。
/// 历史表的 schema 与表名经 CLI 标识符闸校验为小写无引号形态，拼接安全。
pub fn bootstrap_statements(history_schema: &str, history_table: &str) -> Vec<String> {
    vec![
        // 建 schema 需要库上的 CREATE 权限，会话角色 ep_migrator 持有（01_roles.sql）；
        // 属主与三条授权由第 1 号迁移（V20260901090000）在 apply 全序中归位。
        "create schema if not exists platform_core".to_string(),
        // 属主归位前置：ALTER TABLE ... OWNER TO ep_mod_platform_core 要求目标角色
        // 对 schema 持 CREATE（PostgreSQL 硬要求）；此处 schema 属主尚是 ep_migrator，
        // 必须预授。第 1 号迁移随后归位 schema 属主，两权自然延续，无冲突。
        // USAGE 一并预授，保证归位后该角色仍可访问本 schema。
        format!("grant create, usage on schema {history_schema} to {WINDOWS_OWNER_ROLE}"),
        CREATE_MIGRATION_WINDOWS_TABLE.to_string(),
        CREATE_MIGRATION_WINDOW_LOCK_TABLE.to_string(),
        create_history_table_sql(history_schema, history_table),
        // 属主归位：与迁移内 set role 建表的最终属主形态一致（偏离登记十四）。
        format!("alter table platform_core.migration_windows owner to {WINDOWS_OWNER_ROLE}"),
        format!("alter table platform_core.migration_window_lock owner to {WINDOWS_OWNER_ROLE}"),
        align_history_owner_sql(history_schema, history_table),
        INSERT_LOCK_ROW.to_string(),
    ]
}

fn db_failure(detail: String) -> Outcome {
    Outcome::Failed(
        MigrateExit::EnvSelfCheckFailed,
        format!("环境自检项 db-reachable 不通过：{detail}"),
    )
}

/// 首装自举：单一事务内建出最小结构并插入一次性安装窗口行，
/// 成功返回该窗口 id（后续按正常流程回写 applied_versions）。
/// 事务中途失败即整体回滚，目标库回到空库形态，可原样重跑。
pub async fn run_bootstrap(
    client: &mut Client,
    history_schema: &str,
    history_table: &str,
) -> Result<String, Outcome> {
    let window_id = uuid::Uuid::now_v7();
    let tx = client
        .transaction()
        .await
        .map_err(|e| db_failure(format!("开启首装自举事务失败：{e}")))?;
    for stmt in SESSION_PREAMBLE {
        tx.batch_execute(stmt)
            .await
            .map_err(|e| db_failure(format!("自举会话设置失败：{e}")))?;
    }
    for stmt in bootstrap_statements(history_schema, history_table) {
        tx.batch_execute(&stmt)
            .await
            .map_err(|e| db_failure(format!("首装自举 DDL 失败（{stmt}）：{e}")))?;
    }
    tx.execute(
        INSERT_BOOTSTRAP_WINDOW,
        &[
            &window_id.to_string(),
            &INITIAL_INSTALL_APPROVAL_REF,
            &INITIAL_INSTALL_REASON,
            &SYSTEM_PRINCIPAL,
            &(DEFAULT_TTL_MINUTES as i32),
        ],
    )
    .await
    .map_err(|e| db_failure(format!("写入首装安装窗口行失败：{e}")))?;
    tx.commit()
        .await
        .map_err(|e| db_failure(format!("提交首装自举事务失败：{e}")))?;
    Ok(window_id.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::history::HISTORY_TABLE_COLUMNS;
    use std::path::{Path, PathBuf};

    /// 归一化：小写、`''` 还原 `'`、去 IF NOT EXISTS、压缩空白，
    /// 使常量与迁移文件（DO 块内 execute 串）可逐字比对。
    fn normalize(s: &str) -> String {
        let s = s.to_lowercase();
        let s = s.replace("''", "'");
        let s = s.replace("if not exists ", "");
        s.split_whitespace().collect::<Vec<_>>().join(" ")
    }

    /// 仓库内迁移文件路径（相对本 crate 清单目录）。
    fn migration_file(name: &str) -> PathBuf {
        Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("..")
            .join("..")
            .join("db")
            .join("migrations")
            .join("platform_core")
            .join(name)
    }

    #[test]
    fn decide_entry_only_bootstraps_on_empty_db_without_window() {
        // 触发条件：无历史表且未出示窗口，且仅此一格。
        assert_eq!(decide_entry(false, false), ApplyEntry::Bootstrap);
        // 出示窗口的调用永不走自举（空库上出示的窗口必由窗口闸拒）。
        assert_eq!(decide_entry(false, true), ApplyEntry::PresentedWindow);
        assert_eq!(decide_entry(true, true), ApplyEntry::PresentedWindow);
        // 重复 apply：自举后历史表已存在，走正常比对路径，不再自举。
        assert_eq!(decide_entry(true, false), ApplyEntry::Bare);
    }

    #[test]
    fn bootstrap_window_row_shape_is_frozen() {
        // 审批引用为一次性安装固定取值。
        assert_eq!(INITIAL_INSTALL_APPROVAL_REF, "INITIAL_INSTALL");
        // reason 写明首装自举且远低于 2000 字上限（约束 ck_…_reason_len）。
        assert!(INITIAL_INSTALL_REASON.contains("空库首装自举"));
        assert!(INITIAL_INSTALL_REASON.chars().count() <= 2000);
        // 插入语句形态：OPEN 态、五参绑定、UUID 两步转型与 open-window 同口径。
        let sql = normalize(INSERT_BOOTSTRAP_WINDOW);
        assert!(sql.contains("'open'"), "首装窗口行必须取 OPEN 态");
        assert!(sql.contains("$1::text::uuid"), "id 以文本绑定两步转型");
        assert!(
            sql.contains("$4::text::uuid"),
            "opened_by 以文本绑定两步转型"
        );
        assert!(
            sql.contains("make_interval(mins => $5)"),
            "到期时刻按 ttl 计"
        );
        // opened_by 取系统主体（ep_foundation SYSTEM_PRINCIPAL_ID，A-02）。
        assert_eq!(SYSTEM_PRINCIPAL, "00000000-0000-7000-8000-000000000001");
    }

    #[test]
    fn windows_ddl_matches_migration_final_shape() {
        // 与 V20260901093500 逐字一致：列集、约束、默认值逐项比对。
        let migration_sql = std::fs::read_to_string(migration_file(
            "V20260901093500__platform_core_migration_windows.sql",
        ))
        .expect("窗口表迁移文件必须随仓交付");
        let normalized_migration = normalize(&migration_sql);
        assert!(
            normalized_migration.contains(&normalize(CREATE_MIGRATION_WINDOWS_TABLE)),
            "migration_windows 自举 DDL 必须与迁移最终形态逐字一致"
        );
        assert!(
            normalized_migration.contains(&normalize(CREATE_MIGRATION_WINDOW_LOCK_TABLE)),
            "migration_window_lock 自举 DDL 必须与迁移最终形态逐字一致"
        );
        assert!(
            normalized_migration.contains(&normalize(INSERT_LOCK_ROW)),
            "单例锁行插入必须与迁移同款幂等形态"
        );
    }

    #[test]
    fn bootstrap_ownership_matches_migration_set_role() {
        // 迁移以 set role ep_mod_platform_core 建表；自举以属主归位对齐同一形态。
        let migration_sql = std::fs::read_to_string(migration_file(
            "V20260901093500__platform_core_migration_windows.sql",
        ))
        .expect("窗口表迁移文件必须随仓交付");
        assert!(
            migration_sql.contains(&format!("set role {WINDOWS_OWNER_ROLE};")),
            "迁移建表属主口径读取失败，属主一致性断言失去依据"
        );
        let stmts = bootstrap_statements("platform_core", "schema_history");
        let joined = normalize(&stmts.join("; "));
        // 自举预授 schema 权限是属主归位的前置硬要求（模块头第 1 条）。
        assert!(
            joined.contains("grant create, usage on schema platform_core to ep_mod_platform_core"),
            "属主归位前必须预授目标角色 schema 的 CREATE 与 USAGE"
        );
        assert!(
            joined.contains(&format!(
                "alter table platform_core.migration_windows owner to {WINDOWS_OWNER_ROLE}"
            )),
            "migration_windows 属主必须归位"
        );
        assert!(
            joined.contains(&format!(
                "alter table platform_core.migration_window_lock owner to {WINDOWS_OWNER_ROLE}"
            )),
            "migration_window_lock 属主必须归位"
        );
        // 历史表与运行期路径共用同一建表常量与同一属主归位函数，天然同形态。
        assert!(joined.contains(&normalize(&create_history_table_sql(
            "platform_core",
            "schema_history"
        ))));
        assert!(joined.contains(&normalize(&align_history_owner_sql(
            "platform_core",
            "schema_history"
        ))));
        // 历史表四列结构仍按 refinery 语义兼容口径（见 history.rs 模块头）。
        assert!(HISTORY_TABLE_COLUMNS.contains("version BIGINT PRIMARY KEY"));
    }
}
