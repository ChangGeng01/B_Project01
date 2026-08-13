//! 身份域的装配（阶段 4 任务 #21）。
//!
//! 纪律同目录其他文件：能力缺位以「不注入」表达（unwired-absent）。
//! 五个前置条件任一不满足即返回失败原因，PlatformState.identity 置
//! None，端点按 503 NOT_PROVISIONED 处置：
//! 一、数据库装配已就位；二、密钥后端已就位；三、存在至少一个
//!    ACTIVE 密钥域承载 TOTP 种子信封（逐法人探测，同自检第二段形态）；
//! 四、Argon2id 参数合法；五、口令策略束构造成功。
//! 审计/事件/告警三个占位端口的实现体以结构化日志落地（同阶段 2
//! events.rs 先例）：写入本体分别属阶段 3b 与降级台账，不静默丢弃。

use std::sync::Arc;
use std::time::Duration;

use ep_adapter_db_pg::{
    PgAccountStore, PgBreakglassStore, PgChallengeCleanup, PgCredentialStore, PgDeviceStore,
    PgLockoutStore, PgLoginAttemptStore, PgPasswordHistoryStore, PgSessionStore, PgUnitOfWork,
    PgUserAuthzQuery,
};
use ep_adapter_kms::BuiltinKmsBackend;
use ep_foundation::port::kms::{KeyDomainId, KmsBackend};
use ep_foundation::security::context::{RequestId, TraceId};
use ep_foundation::security::SecurityContext;
use ep_platform_authz::{AdmissionConfig, AdmissionGate, AuthzMetricsSink};
use ep_platform_identity::config::{
    Argon2Params, BreakglassPolicy, IdentityPolicies, LockoutPolicy, PasswordPolicy, SessionPolicy,
    TotpPolicy, WebauthnPolicy, X509Policy,
};
use ep_platform_identity::ports::{AuditRecorder, OpsAlertRecorder, PendingEventRecorder};
use ep_platform_identity::{
    AccountAdminService, BreakglassService, HygieneService, LifecycleService, LoginService,
    MfaChallengeService, MfaEnrollmentService, PasswordService,
};
use ep_platform_obs::log::{JsonLogger, Level, LogFields};
use ep_platform_obs::MetricsRegistry;
use ep_platform_runtime::config::{AdmissionCfg, AuthCfg};
use ep_platform_tenancy::directory::LegalEntityDirectory;

use super::db::DbAssembly;

/// 装配固定追踪标识：32 位十六进制零串。
const WIRING_TRACE_ID: &str = "00000000000000000000000000000000";

/// 身份域装配产物：五个用例服务与 TOTP 密钥域。
#[allow(dead_code)]
pub struct IdentityAssembly {
    pub login: Arc<LoginService<PgUnitOfWork>>,
    pub lifecycle: Arc<LifecycleService<PgUnitOfWork>>,
    pub account_admin: Arc<AccountAdminService<PgUnitOfWork>>,
    pub breakglass: Arc<BreakglassService<PgUnitOfWork>>,
    pub enrollment: Arc<MfaEnrollmentService<PgUnitOfWork>>,
    /// 过期会话与挑战清理（job-worker 复用同构装配，此处持有供自检）。
    pub hygiene: Arc<HygieneService<PgUnitOfWork>>,
    pub totp_domain: KeyDomainId,
}

/// 授权域六指标到 obs 登记表的桥接（指标名已在 obs 注册，阶段 4 填充）。
pub struct ObsAuthzMetrics {
    registry: Arc<MetricsRegistry>,
}

impl ObsAuthzMetrics {
    pub fn new(registry: Arc<MetricsRegistry>) -> Self {
        Self { registry }
    }
}

impl AuthzMetricsSink for ObsAuthzMetrics {
    fn observe_decision(&self, legal_entity_id: &str, allowed: bool, seconds: f64) {
        let outcome = if allowed { "allowed" } else { "denied" };
        let _ = self.registry.observe(
            "ep_authz_decision_duration_seconds",
            &[("legal_entity_id", legal_entity_id), ("outcome", outcome)],
            seconds,
        );
    }

    fn count_denied(&self, legal_entity_id: &str, reason: &str) {
        let _ = self.registry.inc_counter(
            "ep_authz_denied_total",
            &[("legal_entity_id", legal_entity_id), ("reason", reason)],
            1.0,
        );
    }

    fn count_scope_truncated(&self, legal_entity_id: &str) {
        let _ = self.registry.inc_counter(
            "ep_authz_scope_truncated_total",
            &[("legal_entity_id", legal_entity_id)],
            1.0,
        );
    }

    fn count_reauth_challenge(&self, legal_entity_id: &str, operation: &str) {
        let _ = self.registry.inc_counter(
            "ep_reauth_challenges_total",
            &[
                ("legal_entity_id", legal_entity_id),
                ("operation_type", operation),
            ],
            1.0,
        );
    }

    fn observe_admission(&self, seconds: f64, admitted: bool, reason: &str) {
        let outcome = if admitted { "admitted" } else { "rejected" };
        let _ = self.registry.observe(
            "ep_session_admission_queue_wait_seconds",
            &[("outcome", outcome), ("reason", reason)],
            seconds,
        );
        if !admitted {
            let _ = self.registry.inc_counter(
                "ep_session_admission_rejected_total",
                &[("reason", reason)],
                1.0,
            );
        }
    }
}

/// 审计记录的日志实现体：写入本体属阶段 3b，此处不静默丢弃。
pub struct LogAuditRecorder {
    logger: Arc<JsonLogger>,
}

impl AuditRecorder for LogAuditRecorder {
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

/// Outbox 待发出事件的日志实现体：同阶段 2 record_pending_emit 先例。
pub struct LogPendingEvents {
    logger: Arc<JsonLogger>,
}

impl PendingEventRecorder for LogPendingEvents {
    fn record_pending(&self, event_type: &str, subject: &str) {
        self.logger.log(
            Level::Info,
            LogFields::msg(
                "platform-event",
                format!("事件 {event_type} 主体 {subject}：Outbox 接缝属阶段 3b，本阶段仅登记发生"),
            ),
        );
    }
}

/// platform_ops 台账告警的日志实现体：应急启用即开口（降级窗口端口复用面）。
pub struct LogOpsAlert {
    logger: Arc<JsonLogger>,
}

impl OpsAlertRecorder for LogOpsAlert {
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

/// EP__AUTH__* 配置段到身份域策略束的映射。
pub fn policies_from_cfg(auth: &AuthCfg) -> IdentityPolicies {
    IdentityPolicies {
        password: PasswordPolicy {
            min_length: usize::from(auth.password.min_length),
            min_char_classes: usize::from(auth.password.min_char_classes),
            max_age_days: u32::from(auth.password.max_age_days),
            history_size: usize::from(auth.password.history_size),
        },
        argon2: Argon2Params {
            memory_kib: auth.password.argon2.memory_kib,
            iterations: auth.password.argon2.iterations,
            parallelism: auth.password.argon2.parallelism,
        },
        lockout: LockoutPolicy {
            max_failures: u32::from(auth.lockout.max_failures),
            window_seconds: u64::from(auth.lockout.window_seconds),
            duration_seconds: u64::from(auth.lockout.duration_seconds),
        },
        session: SessionPolicy {
            ttl_seconds: u64::from(auth.session.ttl_seconds),
            idle_timeout_seconds: u64::from(auth.session.idle_timeout_seconds),
            max_per_user: usize::from(auth.session.max_per_user),
            sliding_write_granularity_seconds: u64::from(
                auth.session.sliding_write_granularity_seconds,
            ),
        },
        totp: TotpPolicy {
            skew_steps: u32::from(auth.totp.skew_steps),
        },
        webauthn: WebauthnPolicy {
            rp_id: auth.webauthn.rp_id.clone(),
            origins: auth.webauthn.origins.clone(),
        },
        x509: X509Policy {
            trust_anchor_ref: auth.x509.trust_anchor_ref.clone(),
        },
        breakglass: BreakglassPolicy {
            max_session_seconds: u64::from(auth.breakglass.max_session_seconds),
            idle_rotation_days: u32::from(auth.breakglass.idle_rotation_days),
        },
    }
}

fn admission_cfg_of(cfg: &AdmissionCfg) -> AdmissionConfig {
    AdmissionConfig {
        max_concurrent_users: usize::from(cfg.max_concurrent_users),
        queue_max_len: usize::from(cfg.queue_max_len),
        queue_wait_timeout: Duration::from_secs(u64::from(cfg.queue_wait_timeout_seconds)),
        active_window: Duration::from_secs(u64::from(cfg.active_window_seconds)),
    }
}

/// 逐法人探测第一个 ACTIVE 的 LEGAL_ENTITY 密钥域，承载 TOTP 种子信封。
/// key_domains 挂法人行级策略，跨法人枚举只能逐法人切换上下文（同
/// secrets-resolvable 第二段形态）。无可用域即装配失败（unwired-absent）。
async fn resolve_totp_domain(db: &DbAssembly) -> Result<KeyDomainId, String> {
    let entities = db
        .legal_entities
        .list_active()
        .await
        .map_err(|e| format!("法人目录枚举失败：{}", e.message))?;
    for entity in entities {
        let request = RequestId::new("identity-wiring").map_err(|e| e.message.clone())?;
        let trace = TraceId::new(WIRING_TRACE_ID).map_err(|e| e.message.clone())?;
        let ctx = SecurityContext::system(entity.id, request, trace);
        let row = db
            .key_domains
            .domain_of_kind(&ctx, "LEGAL_ENTITY")
            .await
            .map_err(|e| format!("密钥域探测失败：{}", e.message))?;
        if let Some(domain) = row {
            if domain.state == "ACTIVE" {
                return Ok(KeyDomainId(domain.id));
            }
        }
    }
    Err("无 ACTIVE 密钥域可供 TOTP 种子信封".to_string())
}

/// 完整装配。任一步失败返回原因文本：不注入，不带病运行。
pub async fn build(
    db: &DbAssembly,
    kms: Arc<BuiltinKmsBackend>,
    auth: &AuthCfg,
    admission: &AdmissionCfg,
    registry: Arc<MetricsRegistry>,
    logger: Arc<JsonLogger>,
) -> Result<IdentityAssembly, String> {
    let policies = policies_from_cfg(auth);
    // WebAuthn 登记面配置自检：缺失不阻断 TOTP/口令主链，仅降级登记。
    if let Err(reason) = policies.webauthn.selfcheck() {
        logger.log(
            Level::Warn,
            LogFields::msg("startup", format!("WebAuthn 面未就绪：{reason}")),
        );
    }
    let password_service = Arc::new(
        PasswordService::new(policies.argon2)
            .map_err(|e| format!("Argon2id 参数非法：{}", e.message))?,
    );
    let totp_domain = resolve_totp_domain(db).await?;

    let accounts: Arc<dyn ep_platform_identity::ports::AccountStore> = Arc::new(PgAccountStore);
    let credentials: Arc<dyn ep_platform_identity::ports::CredentialStore> =
        Arc::new(PgCredentialStore);
    let password_history: Arc<dyn ep_platform_identity::ports::PasswordHistoryStore> =
        Arc::new(PgPasswordHistoryStore);
    let devices: Arc<dyn ep_platform_identity::ports::DeviceStore> = Arc::new(PgDeviceStore);
    let sessions: Arc<dyn ep_platform_identity::ports::SessionStore> = Arc::new(PgSessionStore);
    let lockouts: Arc<dyn ep_platform_identity::ports::LockoutStore> = Arc::new(PgLockoutStore);
    let attempts: Arc<dyn ep_platform_identity::ports::LoginAttemptStore> =
        Arc::new(PgLoginAttemptStore);
    let breakglass_store: Arc<dyn ep_platform_identity::ports::BreakglassStore> =
        Arc::new(PgBreakglassStore);
    let challenges_cleanup: Arc<dyn ep_platform_identity::ports::ChallengeCleanup> =
        Arc::new(PgChallengeCleanup);
    let authz_query: Arc<dyn ep_platform_identity::ports::UserAuthzQuery> =
        Arc::new(PgUserAuthzQuery::new(db.legal_entities.clone()
            as Arc<dyn ep_platform_tenancy::directory::LegalEntityDirectory>));
    let audit: Arc<dyn AuditRecorder> = Arc::new(LogAuditRecorder {
        logger: logger.clone(),
    });
    let pending: Arc<dyn PendingEventRecorder> = Arc::new(LogPendingEvents {
        logger: logger.clone(),
    });
    let ops_alert: Arc<dyn OpsAlertRecorder> = Arc::new(LogOpsAlert {
        logger: logger.clone(),
    });
    let kms_dyn: Arc<dyn KmsBackend> = kms.clone();
    let admission_gate = Arc::new(AdmissionGate::new(
        admission_cfg_of(admission),
        Arc::new(ObsAuthzMetrics::new(registry)),
    ));
    let challenges = Arc::new(MfaChallengeService::new(u64::from(auth.reauth.ttl_seconds)));
    let uow = db.uow_rw.clone();

    let login = Arc::new(LoginService::new(
        uow.clone(),
        admission_gate,
        accounts.clone(),
        credentials.clone(),
        devices.clone(),
        sessions.clone(),
        lockouts,
        attempts,
        authz_query.clone(),
        audit.clone(),
        password_service.clone(),
        challenges,
        kms_dyn.clone(),
        totp_domain,
        policies.clone(),
    ));
    let lifecycle = Arc::new(LifecycleService::new(
        uow.clone(),
        accounts.clone(),
        credentials.clone(),
        password_history.clone(),
        devices.clone(),
        sessions.clone(),
        authz_query.clone(),
        pending,
        audit.clone(),
        password_service.clone(),
        policies.clone(),
    ));
    let account_admin = Arc::new(AccountAdminService::new(
        uow.clone(),
        accounts.clone(),
        credentials.clone(),
        password_history,
        password_service,
        policies.clone(),
    ));
    let breakglass = Arc::new(BreakglassService::new(
        uow.clone(),
        breakglass_store,
        sessions.clone(),
        credentials.clone(),
        authz_query,
        audit,
        ops_alert,
        policies.breakglass,
    ));
    let enrollment = Arc::new(MfaEnrollmentService::new(
        uow.clone(),
        accounts,
        credentials.clone(),
        kms_dyn,
        totp_domain,
        policies,
    ));
    let hygiene = Arc::new(HygieneService::new(uow, sessions, challenges_cleanup));
    Ok(IdentityAssembly {
        login,
        lifecycle,
        account_admin,
        breakglass,
        enrollment,
        hygiene,
        totp_domain,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use ep_platform_runtime::config::AuthCfg;

    #[test]
    fn policies_map_every_auth_key_into_the_identity_bundle() {
        let cfg = AuthCfg::default();
        let p = policies_from_cfg(&cfg);
        assert_eq!(p.password.min_length, 12);
        assert_eq!(p.argon2.memory_kib, 65_536);
        assert_eq!(p.lockout.max_failures, 5);
        assert_eq!(p.session.ttl_seconds, 28_800);
        assert_eq!(p.totp.skew_steps, 1);
        assert_eq!(p.x509.trust_anchor_ref, "secret://pki/client_ca#1");
        assert_eq!(p.breakglass.idle_rotation_days, 365);
    }

    #[test]
    fn admission_config_maps_all_four_keys() {
        let cfg = AdmissionCfg::default();
        let got = admission_cfg_of(&cfg);
        assert_eq!(got.max_concurrent_users, 20);
        assert_eq!(got.queue_max_len, 40);
        assert_eq!(got.queue_wait_timeout, Duration::from_secs(10));
        assert_eq!(got.active_window, Duration::from_secs(60));
    }
}
