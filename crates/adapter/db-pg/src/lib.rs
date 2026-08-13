//! ep-adapter-db-pg — 首版唯一交付并认证的 PostgreSQL 16 实现。
//!
//! 含 RLS 会话变量注入与清除、流复制以外的全部 SQL。
//! 驱动为 sqlx（runtime-tokio）：五具名池与会话语义全部走 sqlx，
//! 历史迁移的数据库通道由 tools/migrate 侧的 refinery 承担，不在本 crate。
//!
//! 模块分工：
//! - [`budget`]：五具名池种类与连接预算（C-04 四类型之三）；
//! - [`conn`]：连接抽象、SQLSTATE 分类与错误码映射、金额写前断言；
//! - [`fake`]：纯逻辑测试用假连接；
//! - [`foundation_check`]：启动自检五项中 SQL 四项的取数实现；
//! - [`platform_core`]：`platform_core` schema 的仓储（迁移窗口守卫等）；
//! - [`platform_msg`]：`platform_msg` schema 的仓储（阶段 3a 幂等键存储）；
//! - [`platform_ops`]：`platform_ops` schema 的仓储（降级窗口台账）；
//! - [`metrics`]：指标出口 trait（注册表在 ep-platform-obs，桥接归装配侧）；
//! - [`pool`]：五具名池构建与 after_connect/after_release 钩子；
//! - [`retry`]：事务重试策略与判定（C-04 四类型之一）；
//! - [`session`]：四条 RLS 会话变量（C-04 四类型之一）；
//! - [`tx`]：`PgTx`/`PgSnapshot`/`PgUnitOfWork` 真实事务接通。

pub mod budget;
pub mod conn;
pub mod fake;
pub mod foundation_check;
pub mod metrics;
pub mod platform_authz;
pub mod platform_core;
pub mod platform_msg;
pub mod platform_ops;
pub mod pool;
pub mod retry;
pub mod session;
pub mod tx;

pub use budget::{BudgetViolation, ConnectionBudget, PoolKind, PoolSpec};
pub use conn::{DbConn, DbValue, PgError};
pub use foundation_check::{
    CheckMigrationRow, CheckRlsState, CheckRolePrivileges, CheckServerSettings, CheckTableRls,
    DataFoundationCheck, PgDataFoundationCheck,
};
pub use metrics::{DbMetrics, NoopDbMetrics, RecordingDbMetrics};
pub use platform_authz::config_store::PgAuthzConfigWriteStore;
pub use platform_authz::snapshot_query::PgAuthzConfigVersionQuery;
pub use platform_authz::user_grants::PgUserAuthzQuery;
pub use platform_core::guard::PgMigrationWindowGuard;
pub use platform_core::identity_accounts::{
    PgAccountStore, PgCredentialStore, PgPasswordHistoryStore,
};
pub use platform_core::identity_breakglass::{PgBreakglassStore, PgChallengeCleanup};
pub use platform_core::identity_sessions::{
    PgDeviceStore, PgLockoutStore, PgLoginAttemptStore, PgSessionStore,
};
pub use platform_core::key_domain::{
    DataKeyInsert, DataKeyRow, KeyDomainRow, PgKeyDomainStore, RotationRows,
};
pub use platform_core::sensitive_fields::{
    PgSensitiveFieldRegistry, SensitiveFieldFilter, SensitiveFieldRow,
};
pub use platform_core::tenancy::{
    PgDepartmentClosure, PgDepartmentClosureMaintainer, PgLegalEntityDirectory,
};
pub use platform_core::windows::{OpenedWindow, PgMigrationWindowStore};
pub use platform_msg::idempotency::PgIdempotencyStore;
pub use platform_ops::PgDegradationLedger;
pub use pool::{register_process_name, PgPools, PoolBuildCfg, PoolTimeouts, RoResourceLimits};
pub use retry::RetryPolicy;
pub use session::SessionContext;
pub use tx::{PgSnapshot, PgTx, PgUnitOfWork};
