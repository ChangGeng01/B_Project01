//! 身份域后台任务的装配（阶段 4 任务 #21）。
//!
//! 纪律同目录其他文件：能力缺位以「不注入」表达（unwired-absent）。
//! 与 core-server 的差异：本进程不装配密钥后端，因此只装配卫生与
//! 应急两个用例（TOTP 种子信封的 wrap/unwrap 不在后台任务路径上）。
//! 审计与台账告警两个占位端口的实现体以结构化日志落地（同阶段 2
//! events.rs 先例）：写入本体分别属阶段 3b 与降级台账，不静默丢弃。

use std::sync::Arc;

use ep_adapter_db_pg::{
    PgBreakglassStore, PgChallengeCleanup, PgCredentialStore, PgLegalEntityDirectory,
    PgSessionStore, PgUnitOfWork, PgUserAuthzQuery,
};
use ep_platform_identity::config::BreakglassPolicy;
use ep_platform_identity::ports::{AuditRecorder, OpsAlertRecorder};
use ep_platform_identity::{BreakglassService, HygieneService};
use ep_platform_obs::log::{JsonLogger, Level, LogFields};
use ep_platform_runtime::config::AuthCfg;

use super::db::WorkerDbAssembly;

/// 后台任务装配产物：卫生与应急两用例 + 法人目录（逐法人枚举）。
pub struct WorkerIdentityAssembly {
    pub hygiene: Arc<HygieneService<PgUnitOfWork>>,
    pub breakglass: Arc<BreakglassService<PgUnitOfWork>>,
    pub directory: Arc<PgLegalEntityDirectory>,
}

/// 审计记录的日志实现体：写入本体属阶段 3b，此处不静默丢弃。
pub struct JobAuditRecorder {
    logger: Arc<JsonLogger>,
}

impl AuditRecorder for JobAuditRecorder {
    fn record(&self, kind: &str, detail: &str) {
        self.logger.log(
            Level::Info,
            LogFields::msg(
                "audit-event",
                format!("kind={kind} {detail}：审计写入本体属阶段 3b，本阶段仅登记发生"),
            ),
        );
    }
}

/// platform_ops 台账告警的日志实现体：应急到期失效即开口。
pub struct JobOpsAlert {
    logger: Arc<JsonLogger>,
}

impl OpsAlertRecorder for JobOpsAlert {
    fn alert_breakglass_activated(&self, doc_no: &str, user_id: &str) {
        self.logger.log(
            Level::Warn,
            LogFields::msg(
                "ops-alert",
                format!("应急账号启用 doc_no={doc_no} user={user_id}：台账窗口经降级台账承接"),
            ),
        );
    }
}

/// EP__AUTH__BREAKGLASS__* 两键到应急策略的映射。
fn breakglass_policy_of(auth: &AuthCfg) -> BreakglassPolicy {
    BreakglassPolicy {
        max_session_seconds: u64::from(auth.breakglass.max_session_seconds),
        idle_rotation_days: u32::from(auth.breakglass.idle_rotation_days),
    }
}

/// 后台任务装配。前置条件只有一个：数据库装配已就位（调用侧保证）。
pub fn build(
    db: &WorkerDbAssembly,
    auth: &AuthCfg,
    logger: Arc<JsonLogger>,
) -> Result<WorkerIdentityAssembly, String> {
    let sessions: Arc<dyn ep_platform_identity::ports::SessionStore> = Arc::new(PgSessionStore);
    let challenges_cleanup: Arc<dyn ep_platform_identity::ports::ChallengeCleanup> =
        Arc::new(PgChallengeCleanup);
    let credentials: Arc<dyn ep_platform_identity::ports::CredentialStore> =
        Arc::new(PgCredentialStore);
    let breakglass_store: Arc<dyn ep_platform_identity::ports::BreakglassStore> =
        Arc::new(PgBreakglassStore);
    let authz_query: Arc<dyn ep_platform_identity::ports::UserAuthzQuery> =
        Arc::new(PgUserAuthzQuery::new(db.legal_entities.clone()
            as Arc<dyn ep_platform_tenancy::directory::LegalEntityDirectory>));
    let audit: Arc<dyn AuditRecorder> = Arc::new(JobAuditRecorder {
        logger: logger.clone(),
    });
    let ops_alert: Arc<dyn OpsAlertRecorder> = Arc::new(JobOpsAlert { logger });
    let uow = db.uow_worker.clone();

    let hygiene = Arc::new(HygieneService::new(
        uow.clone(),
        sessions.clone(),
        challenges_cleanup,
    ));
    let breakglass = Arc::new(BreakglassService::new(
        uow,
        breakglass_store,
        sessions,
        credentials,
        authz_query,
        audit,
        ops_alert,
        breakglass_policy_of(auth),
    ));
    Ok(WorkerIdentityAssembly {
        hygiene,
        breakglass,
        directory: db.legal_entities.clone(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use ep_platform_runtime::config::AuthCfg;

    #[test]
    fn breakglass_policy_maps_both_auth_keys() {
        let cfg = AuthCfg::default();
        let p = breakglass_policy_of(&cfg);
        assert_eq!(p.max_session_seconds, 28_800, "默认上限 8 小时");
        assert_eq!(p.idle_rotation_days, 365, "闲置 12 个月轮换登记");
    }
}
