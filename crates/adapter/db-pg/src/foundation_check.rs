//! 数据基座启动自检五项中 SQL 四项的取数实现（02 计划 §7.1、E-11）。
//!
//! 规格要求以 `DataFoundationCheck` trait 交付并返回结构化结论：判定逻辑
//! （版本比对、参数下限、越权清单）在 ep-platform-runtime 的自检项里，
//! 本 trait 只回答「库里现在是什么」，四方法按自检项切分，一次取数一项。
//! 装配侧（apps 的 wiring）把本 trait 的取值适配成运行期 `SqlProbe` 端口形态。
//!
//! 五项的第五项 `secrets-resolvable` 不经数据库取数：其两段判定分别经
//! KMS 后端与法人目录组合得出，由装配侧实现运行期的 `SecretsProbe` 端口，
//! 因此不在本 trait 之内。
//!
//! 取数约定：服务端整型参数（`max_connections` 等）是 INT4，驱动只对 INT8
//! 走专用解码，一律附加 `::bigint` 显式转换；系统目录查询只读
//! `pg_class`、`pg_namespace`、`pg_attribute` 与 `pg_roles`（02 计划 §7.1
//! 对 `rls-enabled-and-forced` 的限定）。

use std::sync::Arc;

use ep_foundation::error::codes::PLATFORM_SYSTEM_INTERNAL_ERROR;
use ep_foundation::error::AppError;
use ep_foundation::id::Id;
use ep_foundation::port::tx::{Tx, UnitOfWork};
use ep_foundation::principal::SYSTEM_PRINCIPAL_ID;
use ep_foundation::security::context::{RequestId, TraceId};
use ep_foundation::security::SecurityContext;

use crate::conn::DbValue;
use crate::tx::{PgTx, PgUnitOfWork};

/// `database-reachable` 的被测取值。形态与运行期 `ServerSettings` 逐项对应。
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct CheckServerSettings {
    pub server_version: String,
    pub timezone: String,
    pub max_connections: u32,
    pub max_wal_senders: u32,
    pub max_replication_slots: u32,
}

/// 迁移历史表的一行。历史表无 schema 列，schema 归属由装配侧按
/// 二进制内嵌清单回填，见 ep-platform-runtime 的 `migrations` 模块。
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct CheckMigrationRow {
    pub version: u64,
    pub name: String,
    pub applied_on: String,
    pub checksum: String,
}

/// 一张带法人列的表的行级安全状态。
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct CheckTableRls {
    pub schema: String,
    pub table: String,
    pub enabled: bool,
    pub forced: bool,
}

/// `rls-enabled-and-forced` 的被测取值。
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct CheckRlsState {
    pub legal_entity_tables: Vec<CheckTableRls>,
    pub current_role_bypassrls: bool,
    pub current_role_superuser: bool,
}

/// `runtime-role-privileges-bounded` 的被测取值。
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct CheckRolePrivileges {
    /// 当前角色具备 CREATE 权限的 schema 清单，非空即越界。
    pub schemas_with_create: Vec<String>,
    pub rolcreaterole: bool,
    pub rolcreatedb: bool,
}

/// 数据基座自检的取数端口。四方法对应四项 Blocking 自检，
/// 任一取数失败以 `AppError` 上抛，由自检项按失败处置。
#[async_trait::async_trait]
pub trait DataFoundationCheck: Send + Sync {
    async fn server_settings(&self) -> Result<CheckServerSettings, AppError>;
    async fn migration_rows(&self) -> Result<Vec<CheckMigrationRow>, AppError>;
    async fn rls_state(&self) -> Result<CheckRlsState, AppError>;
    async fn role_privileges(&self) -> Result<CheckRolePrivileges, AppError>;
}

const SERVER_SETTINGS_STMT: &str = "select current_setting('server_version'), \
     current_setting('TimeZone'), \
     current_setting('max_connections')::bigint, \
     current_setting('max_wal_senders')::bigint, \
     current_setting('max_replication_slots')::bigint";

const MIGRATION_ROWS_STMT: &str = "select version, name, applied_on, checksum \
     from platform_core.schema_history order by version asc";

/// 带 `legal_entity_id` 列的普通表及其 RLS 开关。排除系统 schema。
const RLS_TABLES_STMT: &str = "select n.nspname, c.relname, c.relrowsecurity, \
     c.relforcerowsecurity from pg_class c \
     join pg_namespace n on n.oid = c.relnamespace \
     join pg_attribute a on a.attrelid = c.oid \
     where c.relkind = 'r' and a.attname = 'legal_entity_id' and not a.attisdropped \
     and n.nspname not in ('pg_catalog', 'information_schema') \
     order by n.nspname, c.relname";

const ROLE_RLS_FLAGS_STMT: &str =
    "select rolbypassrls, rolsuper from pg_roles where rolname = current_user";

const SCHEMAS_WITH_CREATE_STMT: &str = "select n.nspname from pg_namespace n \
     where has_schema_privilege(current_user, n.nspname, 'CREATE') \
     and n.nspname not in ('pg_catalog', 'information_schema') order by n.nspname";

const ROLE_CREATE_FLAGS_STMT: &str =
    "select rolcreaterole, rolcreatedb from pg_roles where rolname = current_user";

/// 数据基座自检的 PostgreSQL 实现。装配时绑定 Ops 池的工作单元。
pub struct PgDataFoundationCheck {
    uow: Arc<PgUnitOfWork>,
}

impl PgDataFoundationCheck {
    pub fn new(uow: Arc<PgUnitOfWork>) -> Self {
        Self { uow }
    }

    /// 自检取数是系统行为：主体固定系统账号，request/trace 取固定常量，
    /// 与法人目录同口径。
    fn system_ctx() -> SecurityContext {
        SecurityContext::system(
            Id::from_uuid(SYSTEM_PRINCIPAL_ID),
            RequestId::new("data-foundation-check").expect("固定取值长度合法"),
            TraceId::new(&"0".repeat(32)).expect("固定取值形态合法"),
        )
    }
}

fn downcast(tx: &mut dyn Tx) -> Result<&mut PgTx, AppError> {
    tx.as_any_mut().downcast_mut::<PgTx>().ok_or_else(|| {
        AppError::new(
            PLATFORM_SYSTEM_INTERNAL_ERROR,
            "数据基座自检必须在 PostgreSQL 事务内执行",
        )
    })
}

fn bad_shape(what: &'static str) -> AppError {
    AppError::new(
        PLATFORM_SYSTEM_INTERNAL_ERROR,
        format!("数据库返回的{what}形态与自检取数预期不符"),
    )
}

fn text_of(row: &[DbValue], idx: usize) -> Result<String, AppError> {
    match row.get(idx) {
        Some(DbValue::Text(s)) => Ok(s.clone()),
        _ => Err(bad_shape("文本列")),
    }
}

fn int_of(row: &[DbValue], idx: usize) -> Result<u32, AppError> {
    match row.get(idx) {
        Some(DbValue::Int64(n)) => u32::try_from(*n).map_err(|_| bad_shape("整型参数列")),
        _ => Err(bad_shape("整型参数列")),
    }
}

fn bool_of(row: &[DbValue], idx: usize) -> Result<bool, AppError> {
    match row.get(idx) {
        Some(DbValue::Bool(b)) => Ok(*b),
        _ => Err(bad_shape("布尔列")),
    }
}

fn single_row(rows: Vec<Vec<DbValue>>, what: &'static str) -> Result<Vec<DbValue>, AppError> {
    if rows.len() != 1 {
        return Err(bad_shape(what));
    }
    rows.into_iter().next().ok_or_else(|| bad_shape(what))
}

#[async_trait::async_trait]
impl DataFoundationCheck for PgDataFoundationCheck {
    async fn server_settings(&self) -> Result<CheckServerSettings, AppError> {
        let ctx = Self::system_ctx();
        self.uow
            .transact(&ctx, |tx| {
                Box::pin(async move {
                    let pg = downcast(tx)?;
                    let rows = pg.query(SERVER_SETTINGS_STMT, &[]).await.map_err(|e| {
                        AppError::new(
                            PLATFORM_SYSTEM_INTERNAL_ERROR,
                            format!("读取服务端参数失败：{}", e.message),
                        )
                    })?;
                    let row = single_row(rows, "服务端参数行")?;
                    Ok(CheckServerSettings {
                        server_version: text_of(&row, 0)?,
                        timezone: text_of(&row, 1)?,
                        max_connections: int_of(&row, 2)?,
                        max_wal_senders: int_of(&row, 3)?,
                        max_replication_slots: int_of(&row, 4)?,
                    })
                })
            })
            .await
    }

    async fn migration_rows(&self) -> Result<Vec<CheckMigrationRow>, AppError> {
        let ctx = Self::system_ctx();
        self.uow
            .transact(&ctx, |tx| {
                Box::pin(async move {
                    let pg = downcast(tx)?;
                    let rows = pg.query(MIGRATION_ROWS_STMT, &[]).await.map_err(|e| {
                        AppError::new(
                            PLATFORM_SYSTEM_INTERNAL_ERROR,
                            format!("读取迁移历史失败：{}", e.message),
                        )
                    })?;
                    rows.iter()
                        .map(|row| {
                            Ok(CheckMigrationRow {
                                version: match row.first() {
                                    Some(DbValue::Int64(v)) => {
                                        u64::try_from(*v).map_err(|_| bad_shape("迁移版本号列"))?
                                    }
                                    _ => return Err(bad_shape("迁移版本号列")),
                                },
                                name: text_of(row, 1)?,
                                applied_on: text_of(row, 2)?,
                                checksum: text_of(row, 3)?,
                            })
                        })
                        .collect()
                })
            })
            .await
    }

    async fn rls_state(&self) -> Result<CheckRlsState, AppError> {
        let ctx = Self::system_ctx();
        self.uow
            .transact(&ctx, |tx| {
                Box::pin(async move {
                    let pg = downcast(tx)?;
                    let pg_err = |e: AppError, what: &'static str| {
                        AppError::new(
                            PLATFORM_SYSTEM_INTERNAL_ERROR,
                            format!("读取{what}失败：{}", e.message),
                        )
                    };
                    let tables = pg
                        .query(RLS_TABLES_STMT, &[])
                        .await
                        .map_err(|e| pg_err(e, "表级 RLS 状态"))?;
                    let legal_entity_tables = tables
                        .iter()
                        .map(|row| {
                            Ok(CheckTableRls {
                                schema: text_of(row, 0)?,
                                table: text_of(row, 1)?,
                                enabled: bool_of(row, 2)?,
                                forced: bool_of(row, 3)?,
                            })
                        })
                        .collect::<Result<Vec<_>, AppError>>()?;
                    let flags = pg
                        .query(ROLE_RLS_FLAGS_STMT, &[])
                        .await
                        .map_err(|e| pg_err(e, "角色 RLS 标志"))?;
                    let row = single_row(flags, "角色 RLS 标志行")?;
                    Ok(CheckRlsState {
                        legal_entity_tables,
                        current_role_bypassrls: bool_of(&row, 0)?,
                        current_role_superuser: bool_of(&row, 1)?,
                    })
                })
            })
            .await
    }

    async fn role_privileges(&self) -> Result<CheckRolePrivileges, AppError> {
        let ctx = Self::system_ctx();
        self.uow
            .transact(&ctx, |tx| {
                Box::pin(async move {
                    let pg = downcast(tx)?;
                    let pg_err = |e: AppError, what: &'static str| {
                        AppError::new(
                            PLATFORM_SYSTEM_INTERNAL_ERROR,
                            format!("读取{what}失败：{}", e.message),
                        )
                    };
                    let schemas = pg
                        .query(SCHEMAS_WITH_CREATE_STMT, &[])
                        .await
                        .map_err(|e| pg_err(e, "schema 授权清单"))?;
                    let schemas_with_create = schemas
                        .iter()
                        .map(|row| text_of(row, 0))
                        .collect::<Result<Vec<_>, _>>()?;
                    let flags = pg
                        .query(ROLE_CREATE_FLAGS_STMT, &[])
                        .await
                        .map_err(|e| pg_err(e, "角色管理标志"))?;
                    let row = single_row(flags, "角色管理标志行")?;
                    Ok(CheckRolePrivileges {
                        schemas_with_create,
                        rolcreaterole: bool_of(&row, 0)?,
                        rolcreatedb: bool_of(&row, 1)?,
                    })
                })
            })
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::fake::FakeConn;
    use crate::metrics::NoopDbMetrics;
    use crate::retry::RetryPolicy;

    fn uow_with(conn: FakeConn) -> Arc<PgUnitOfWork> {
        Arc::new(PgUnitOfWork::with_fake_conns(
            vec![Box::new(conn)],
            "ops",
            RetryPolicy::standard(),
            Arc::new(NoopDbMetrics),
        ))
    }

    #[tokio::test]
    async fn server_settings_decodes_five_columns() {
        let mut conn = FakeConn::new();
        conn.push_rows(vec![vec![
            DbValue::Text("16.4".into()),
            DbValue::Text("UTC".into()),
            DbValue::Int64(64),
            DbValue::Int64(4),
            DbValue::Int64(3),
        ]]);
        let check = PgDataFoundationCheck::new(uow_with(conn));
        let got = check.server_settings().await.expect("五列均可解码");
        assert_eq!(
            got,
            CheckServerSettings {
                server_version: "16.4".into(),
                timezone: "UTC".into(),
                max_connections: 64,
                max_wal_senders: 4,
                max_replication_slots: 3,
            }
        );
    }

    #[tokio::test]
    async fn migration_rows_keep_history_column_verbatim() {
        let mut conn = FakeConn::new();
        conn.push_rows(vec![
            vec![
                DbValue::Int64(20260901090000),
                DbValue::Text("platform_core_create_schema".into()),
                DbValue::Text("2026-09-01T09:00:00Z".into()),
                DbValue::Text("123456789".into()),
            ],
            vec![
                DbValue::Int64(20260901104500),
                DbValue::Text("platform_ops_create_degradation_windows".into()),
                DbValue::Text("2026-09-01T10:45:00Z".into()),
                DbValue::Text("987654321".into()),
            ],
        ]);
        let check = PgDataFoundationCheck::new(uow_with(conn));
        let rows = check.migration_rows().await.expect("历史行可解码");
        assert_eq!(rows.len(), 2);
        assert_eq!(rows[0].version, 20260901090000);
        assert_eq!(rows[1].checksum, "987654321");
    }

    #[tokio::test]
    async fn rls_state_combines_table_rows_and_role_flags() {
        let mut conn = FakeConn::new();
        conn.push_rows(vec![vec![
            DbValue::Text("mdm".into()),
            DbValue::Text("legal_entities_ext".into()),
            DbValue::Bool(true),
            DbValue::Bool(true),
        ]]);
        conn.push_rows(vec![vec![DbValue::Bool(false), DbValue::Bool(false)]]);
        let check = PgDataFoundationCheck::new(uow_with(conn));
        let state = check.rls_state().await.expect("两段取数合并");
        assert_eq!(state.legal_entity_tables.len(), 1);
        assert!(!state.current_role_bypassrls && !state.current_role_superuser);
    }

    #[tokio::test]
    async fn role_privileges_collect_schemas_and_flags() {
        let mut conn = FakeConn::new();
        conn.push_rows(vec![vec![DbValue::Text("platform_core".into())]]);
        conn.push_rows(vec![vec![DbValue::Bool(false), DbValue::Bool(true)]]);
        let check = PgDataFoundationCheck::new(uow_with(conn));
        let p = check.role_privileges().await.expect("两段取数合并");
        assert_eq!(p.schemas_with_create, vec!["platform_core".to_string()]);
        assert!(!p.rolcreaterole && p.rolcreatedb);
    }

    #[tokio::test]
    async fn malformed_shape_is_an_error_not_a_default() {
        let mut conn = FakeConn::new();
        // 布尔列位上放了文本：形态不符必须报错，不得落回默认值。
        conn.push_rows(vec![vec![
            DbValue::Text("16.4".into()),
            DbValue::Text("UTC".into()),
            DbValue::Text("64".into()),
            DbValue::Int64(4),
            DbValue::Int64(3),
        ]]);
        let check = PgDataFoundationCheck::new(uow_with(conn));
        assert!(check.server_settings().await.is_err());
    }
}
