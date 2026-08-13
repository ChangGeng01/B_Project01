//! 阶段 4 任务 #21 的两个后台任务：身份卫生（过期会话与过期
//! 重认证挑战清理）与应急维护（应急账号到期失效与闲置轮换）。
//!
//! 事务法人上下文取系统主体：逐法人枚举（不 OR 展开），每个法人
//! 一个系统上下文执行一轮，RLS 边界与在线路径一致。

use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use std::time::Duration;

use chrono::Utc;
use ep_adapter_db_pg::PgUnitOfWork;
use ep_foundation::id::marker::LegalEntity;
use ep_foundation::id::Id;
use ep_foundation::security::context::{RequestId, TraceId};
use ep_foundation::security::SecurityContext;
use ep_platform_identity::{BreakglassService, HygieneService};
use ep_platform_tenancy::directory::LegalEntityDirectory;

use crate::scheduler::{Job, JobOutcome};

/// 身份卫生周期：每分钟一轮（过期撤销与批量写合并同节拍）。
pub const HYGIENE_INTERVAL: Duration = Duration::from_secs(60);
/// 应急维护周期：五分钟一轮（到期失效与闲置轮换无需分钟级敏感）。
pub const BREAKGLASS_INTERVAL: Duration = Duration::from_secs(300);

/// 后台任务的固定追踪标识：32 位十六进制零串。
const JOB_TRACE_ID: &str = "00000000000000000000000000000000";

/// 逐法人系统上下文：请求标识按任务名区分，追踪标识取固定零串。
fn job_ctx(le: Id<LegalEntity>, request: &str) -> Result<SecurityContext, String> {
    let request_id = RequestId::new(request).map_err(|e| e.message.clone())?;
    let trace_id = TraceId::new(JOB_TRACE_ID).map_err(|e| e.message.clone())?;
    Ok(SecurityContext::system(le, request_id, trace_id))
}

/// 任务一：过期会话批量撤销 + 过期重认证挑战置 EXPIRED。
pub struct SessionHygieneJob {
    hygiene: Arc<HygieneService<PgUnitOfWork>>,
    directory: Arc<dyn LegalEntityDirectory>,
}

impl SessionHygieneJob {
    pub fn new(
        hygiene: Arc<HygieneService<PgUnitOfWork>>,
        directory: Arc<dyn LegalEntityDirectory>,
    ) -> Self {
        Self { hygiene, directory }
    }
}

impl Job for SessionHygieneJob {
    fn name(&self) -> &'static str {
        "identity-hygiene"
    }

    fn interval(&self) -> Duration {
        HYGIENE_INTERVAL
    }

    fn run(&self) -> Pin<Box<dyn Future<Output = JobOutcome> + Send + '_>> {
        Box::pin(async move {
            let entities = self
                .directory
                .list_active()
                .await
                .map_err(|e| e.message.clone())?;
            let now = Utc::now();
            let mut affected = 0u64;
            for entity in entities {
                let ctx = job_ctx(entity.id, "job-worker-hygiene")?;
                affected += self
                    .hygiene
                    .expire_sessions(&ctx, now)
                    .await
                    .map_err(|e| e.message.clone())?;
                affected += self
                    .hygiene
                    .expire_challenges(&ctx, now)
                    .await
                    .map_err(|e| e.message.clone())?;
            }
            Ok(affected)
        })
    }
}

/// 任务二：应急账号到期失效（会话撤销/凭据 REVOKED）与闲置轮换登记。
pub struct BreakglassMaintenanceJob {
    breakglass: Arc<BreakglassService<PgUnitOfWork>>,
    directory: Arc<dyn LegalEntityDirectory>,
}

impl BreakglassMaintenanceJob {
    pub fn new(
        breakglass: Arc<BreakglassService<PgUnitOfWork>>,
        directory: Arc<dyn LegalEntityDirectory>,
    ) -> Self {
        Self {
            breakglass,
            directory,
        }
    }
}

impl Job for BreakglassMaintenanceJob {
    fn name(&self) -> &'static str {
        "breakglass-maintenance"
    }

    fn interval(&self) -> Duration {
        BREAKGLASS_INTERVAL
    }

    fn run(&self) -> Pin<Box<dyn Future<Output = JobOutcome> + Send + '_>> {
        Box::pin(async move {
            let entities = self
                .directory
                .list_active()
                .await
                .map_err(|e| e.message.clone())?;
            let now = Utc::now();
            let mut affected = 0u64;
            for entity in entities {
                let ctx = job_ctx(entity.id, "job-worker-breakglass")?;
                affected += self
                    .breakglass
                    .expire_due(&ctx, now)
                    .await
                    .map_err(|e| e.message.clone())?;
                affected += self
                    .breakglass
                    .rotate_idle(&ctx, now)
                    .await
                    .map_err(|e| e.message.clone())?;
            }
            Ok(affected)
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ep_foundation::security::context::AccountKind;

    #[test]
    fn intervals_follow_the_documented_cadence() {
        assert_eq!(HYGIENE_INTERVAL, Duration::from_secs(60));
        assert_eq!(BREAKGLASS_INTERVAL, Duration::from_secs(300));
    }

    #[test]
    fn job_ctx_yields_a_system_principal_per_legal_entity() {
        let le = Id::<LegalEntity>::from_uuid(uuid::Uuid::from_u128(9));
        let ctx = job_ctx(le, "job-worker-hygiene").expect("固定取值形态合法");
        assert_eq!(ctx.account_kind, AccountKind::System);
        assert_eq!(ctx.legal_entity_id, le);
    }
}
