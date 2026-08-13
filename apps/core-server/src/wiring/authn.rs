//! 认证中间件的装配（阶段 4 任务 #23）。
//!
//! 全部实现体经端口消费身份域既有 SQL 仓储：会话按令牌摘要核验、
//! 法人授权集合读取、设备核验都在同一事务内完成。数据库装配缺位
//! 即整体不注入（unwired-absent），中间件按未装配形态放行，平台
//! 端点随后在各自处理器按 503 NOT_PROVISIONED 处置。

use std::sync::Arc;

use ep_adapter_db_pg::{PgAccountStore, PgDeviceStore, PgSessionStore, PgUserAuthzQuery};
use ep_platform_identity::ports::{AccountStore, DeviceStore, SessionStore, UserAuthzQuery};
use ep_platform_obs::MetricsRegistry;
use ep_platform_runtime::config::AuthSessionCfg;
use ep_platform_tenancy::directory::LegalEntityDirectory;

use super::db::DbAssembly;
use crate::platform::middleware::{AuthnAssembly, PreAuthRateLimiter, SessionTracker};

/// 构建认证中间件载体。数据库装配在场即可构建：会话与授权读取
/// 面不依赖密钥后端。
pub fn build(
    db: &DbAssembly,
    session: &AuthSessionCfg,
    registry: Arc<MetricsRegistry>,
) -> Arc<AuthnAssembly> {
    let sessions: Arc<dyn SessionStore> = Arc::new(PgSessionStore);
    let accounts: Arc<dyn AccountStore> = Arc::new(PgAccountStore);
    let devices: Arc<dyn DeviceStore> = Arc::new(PgDeviceStore);
    let authz_query: Arc<dyn UserAuthzQuery> = Arc::new(PgUserAuthzQuery::new(
        db.legal_entities.clone() as Arc<dyn LegalEntityDirectory>,
    ));
    Arc::new(AuthnAssembly {
        uow: db.uow_rw.clone(),
        sessions,
        accounts,
        devices,
        authz_query,
        limiter: Arc::new(PreAuthRateLimiter::new()),
        tracker: Arc::new(SessionTracker::new(
            u64::from(session.idle_timeout_seconds),
            registry.clone(),
        )),
        metrics: registry,
        sliding_granularity_seconds: u64::from(session.sliding_write_granularity_seconds),
        idle_timeout_seconds: u64::from(session.idle_timeout_seconds),
    })
}
