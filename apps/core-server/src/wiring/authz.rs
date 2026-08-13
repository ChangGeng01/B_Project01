//! 授权域的装配桥接（阶段 4 任务 #23，04 计划 §2.3/§4.4/§7）。
//!
//! ep-platform-authz 的本地端口在这里映射到平台观测面与数据库面：
//! - `AuthzMetricsSink` → obs `MetricsRegistry`（六项 ep_authz_* 与
//!   准入两项端到端填充）；
//! - `DegradationWindowOpener` → obs `DegradationLedger` 的接线位
//!   （AUTHZ_SNAPSHOT_CHECKSUM_MISMATCH 尚不在阶段 2 三取值之内，
//!   按 A-26 纪律由阶段 14 随 CHECK 放宽一并扩展，本阶段只留接线位
//!   与 WARN 日志，不擅自扩 kind）；
//! - 快照轮询任务：`EP__AUTHZ__SNAPSHOT__POLL_INTERVAL_MS` 间隔轮询
//!   `authz_config_versions` 的 EFFECTIVE 版本号。
//!
//! `AdmissionGate` 的构建与驱动归身份域装配（`wiring/identity.rs`，
//! 登录算法九步第一步取席位），本文件不重复构建，避免双闸门；
//! 准入两项指标经同一 obs registry 端到端填充。
//!
//! 行级安全对运行角色强制生效（含 FORCE），单事务只见事务法人一个
//! 法人的行：轮询按法人目录逐法人逐事务驱动，聚合后整体替换快照
//! （与 `SnapshotReloader::reload_once` 的单事务全法人假设在 RLS 下
//! 不可直接驱动的差异已在任务 #23 汇报登记）。

use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;

use ep_adapter_db_pg::{PgAuthzConfigVersionQuery, PgUnitOfWork};
use ep_foundation::error::AppError;
use ep_foundation::id::marker::LegalEntity;
use ep_foundation::id::Id;
use ep_foundation::port::tx::UnitOfWork;
use ep_foundation::security::context::{RequestId, TraceId};
use ep_foundation::security::SecurityContext;
use ep_platform_authz::metrics::AuthzMetricsSink;
use ep_platform_authz::snapshot::{
    AuthzConfigVersionQuery, AuthzSnapshotHolder, DegradationWindowOpener, EntityAuthzData,
};
use ep_platform_authz::types::ObjectScopeBinding;
use ep_platform_obs::log::{JsonLogger, Level, LogFields};
use ep_platform_obs::{DegradationLedger, MetricsRegistry};
use ep_platform_runtime::config::AuthzCfg;
use ep_platform_runtime::selfcheck::registry::Verdict;
use ep_platform_runtime::selfcheck::SelfCheckRun;
use ep_platform_tenancy::directory::LegalEntityDirectory;

use super::db::DbAssembly;

/// 轮询事务的固定请求标识。
const POLL_REQUEST_ID: &str = "authz-snapshot-poll";
/// 轮询事务的追踪标识：32 位十六进制零串。
const POLL_TRACE_ID: &str = "00000000000000000000000000000000";

/// `AuthzMetricsSink` 到 obs `MetricsRegistry` 的填充桥接。
/// 标签取值与登记表逐项一致：legal_entity_id、operation_type、
/// outcome、reason。
pub struct ObsAuthzMetricsSink {
    registry: Arc<MetricsRegistry>,
}

impl ObsAuthzMetricsSink {
    pub fn new(registry: Arc<MetricsRegistry>) -> Self {
        Self { registry }
    }
}

impl AuthzMetricsSink for ObsAuthzMetricsSink {
    fn observe_decision(&self, legal_entity_id: &str, allowed: bool, seconds: f64) {
        let outcome = if allowed { "allow" } else { "deny" };
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
            &[("outcome", outcome)],
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

/// checksum 不符的降级窗口开口。阶段 2 的 `DegradationKind` 只有三个
/// 初始取值，AUTHZ_SNAPSHOT_CHECKSUM_MISMATCH 属阶段 14 扩展面（A-26）：
/// 扩展 kind 与放宽表上 CHECK 必须同批，本阶段不擅自扩枚举，只把台账
/// 引用与开窗参数留在接线位，以 WARN 日志留痕。
pub struct ChecksumMismatchOpener {
    #[allow(dead_code)]
    ledger: Arc<dyn DegradationLedger>,
    logger: Arc<JsonLogger>,
}

impl ChecksumMismatchOpener {
    pub fn new(ledger: Arc<dyn DegradationLedger>, logger: Arc<JsonLogger>) -> Self {
        Self { ledger, logger }
    }
}

impl DegradationWindowOpener for ChecksumMismatchOpener {
    fn open_checksum_mismatch_window(
        &self,
        legal_entity_id: Id<LegalEntity>,
        _expected: &str,
        _actual: &str,
    ) {
        // 接线位：阶段 14 扩展 DegradationKind 后在此经 self.ledger.open
        // 登记 AUTHZ_SNAPSHOT_CHECKSUM_MISMATCH 窗口（scope 取法人，
        // 不可抑制）。摘要取值一律不进日志。
        self.logger.log(
            Level::Warn,
            LogFields::msg(
                "authz-snapshot",
                format!(
                    "法人 {legal_entity_id} 授权快照校验和不符，沿用上一版分片；\
                     降级窗口登记待阶段 14 扩展 kind 后接通"
                ),
            ),
        );
    }
}

/// 逐法人轮询的快照重载器。消费 ep-platform-authz 的三个端口与
/// 法人目录，聚合各法人分片后整体替换快照持有者。
pub struct SnapshotPoller {
    holder: Arc<AuthzSnapshotHolder>,
    query: Arc<dyn AuthzConfigVersionQuery>,
    opener: Arc<dyn DegradationWindowOpener>,
    directory: Arc<dyn LegalEntityDirectory>,
    uow: Arc<PgUnitOfWork>,
    poll_interval: Duration,
    logger: Arc<JsonLogger>,
}

/// 一次轮询的结果摘要，供自检与测试取用。
#[derive(Clone, PartialEq, Eq, Debug, Default)]
pub struct PollOutcome {
    /// 本次轮询见到 EFFECTIVE 版本的法人数。
    pub loaded_entities: usize,
    /// 校验和不符沿用旧分片的法人数。
    pub checksum_mismatches: usize,
    /// 对象范围绑定登记行数。
    pub bindings: usize,
}

impl SnapshotPoller {
    pub fn new(
        holder: Arc<AuthzSnapshotHolder>,
        query: Arc<dyn AuthzConfigVersionQuery>,
        opener: Arc<dyn DegradationWindowOpener>,
        directory: Arc<dyn LegalEntityDirectory>,
        uow: Arc<PgUnitOfWork>,
        poll_interval: Duration,
        logger: Arc<JsonLogger>,
    ) -> Self {
        Self {
            holder,
            query,
            opener,
            directory,
            uow,
            poll_interval,
            logger,
        }
    }

    fn poll_ctx(le: Id<LegalEntity>) -> SecurityContext {
        let request = RequestId::new(POLL_REQUEST_ID)
            .unwrap_or_else(|_| RequestId::new("platform-endpoint").expect("固定取值合法"));
        let trace = TraceId::new(POLL_TRACE_ID).expect("零串合法");
        SecurityContext::system(le, request, trace)
    }

    /// 单次轮询：枚举法人目录，逐法人逐事务读 EFFECTIVE 版本与分片，
    /// 聚合后整体替换；任一步失败整体上抛，由调用方记 WARN 不中断循环。
    pub async fn poll_once(&self) -> Result<PollOutcome, AppError> {
        let entities = self.directory.list_active().await?;
        let mut next: HashMap<Id<LegalEntity>, Arc<EntityAuthzData>> = HashMap::new();
        let mut bindings: Vec<ObjectScopeBinding> = Vec::new();
        let mut outcome = PollOutcome::default();
        for entity in &entities {
            let le = entity.id;
            let ctx = Self::poll_ctx(le);
            let (entity_bindings, data) = self.load_entity_slice(&ctx, le).await?;
            outcome.bindings = entity_bindings.len().max(outcome.bindings);
            bindings = entity_bindings;
            if let Some(data) = data {
                outcome.loaded_entities += 1;
                next.insert(le, data);
            } else {
                outcome.checksum_mismatches += 1;
                if let Some(old) = self.holder.entity(le) {
                    next.insert(le, old);
                }
            }
        }
        self.holder.replace(next, bindings);
        Ok(outcome)
    }

    /// 单法人事务：读 EFFECTIVE 版本行与对象绑定，版本或校验和变化
    /// 时重读分片。`data` 为 None 表示校验和不符或无 EFFECTIVE 版本；
    /// 前者已经开窗留痕，后者在首次装配时属预期态（回填迁移未跑）。
    async fn load_entity_slice(
        &self,
        ctx: &SecurityContext,
        le: Id<LegalEntity>,
    ) -> Result<(Vec<ObjectScopeBinding>, Option<Arc<EntityAuthzData>>), AppError> {
        let query = self.query.clone();
        let old = self.holder.entity(le);
        let opener = self.opener.clone();
        self.uow
            .transact(ctx, move |tx| {
                Box::pin(async move {
                    let bindings = query.object_bindings(tx).await?;
                    let versions = query.effective_versions(tx).await?;
                    let Some(version) = versions.iter().find(|v| v.legal_entity_id == le) else {
                        return Ok((bindings, None));
                    };
                    if let Some(old) = &old {
                        if old.version_no == version.version_no && old.checksum == version.checksum
                        {
                            return Ok((bindings, Some(old.clone())));
                        }
                    }
                    let data = query.load_entity(tx, le, version.version_no).await?;
                    if data.checksum != version.checksum {
                        opener.open_checksum_mismatch_window(
                            le,
                            version.checksum.as_ref(),
                            data.checksum.as_ref(),
                        );
                        return Ok((bindings, None));
                    }
                    Ok((bindings, Some(Arc::new(data))))
                })
            })
            .await
    }

    /// 轮询循环：错误记 WARN 后按间隔继续，不因单次失败退出。
    pub async fn run_forever(self: Arc<Self>) {
        loop {
            match self.poll_once().await {
                Ok(_) => {}
                Err(e) => self.logger.log(
                    Level::Warn,
                    LogFields::msg(
                        "authz-snapshot",
                        format!("快照轮询本轮失败，沿用上一版快照：{}", e.code),
                    ),
                ),
            }
            tokio::time::sleep(self.poll_interval).await;
        }
    }
}

/// 授权域装配产物：快照持有者、轮询器与指标桥接。holder 与
/// metrics 是装配事实的持有点（判定面接入阶段经此消费），
/// 并非占位实现，dead_code 容忍在此是显式决定。
#[allow(dead_code)]
pub struct AuthzAssembly {
    pub holder: Arc<AuthzSnapshotHolder>,
    pub poller: Arc<SnapshotPoller>,
    /// `AuthzMetricsSink` 到 obs registry 的填充桥接：判定面接入时
    /// （阶段 5+）经此端到端填充 ep_authz_* 指标。
    pub metrics: Arc<dyn AuthzMetricsSink>,
}

/// `authz-snapshot-loadable` 自检项（Blocking，kebab-case）：驱动
/// 一次逐法人快照加载。checksum 不符不阻断启动：轮询器已经开窗
/// 留痕（接线位，kind 待阶段 14 扩展），自检如实报 Degraded。
pub struct SnapshotLoadableCheck {
    poller: Arc<SnapshotPoller>,
}

impl SnapshotLoadableCheck {
    pub fn new(poller: Arc<SnapshotPoller>) -> Self {
        Self { poller }
    }
}

#[async_trait::async_trait]
impl SelfCheckRun for SnapshotLoadableCheck {
    async fn run(&self) -> Verdict {
        match self.poller.poll_once().await {
            Ok(outcome) if outcome.checksum_mismatches > 0 => Verdict::Degraded(format!(
                "加载 {} 个法人分片，{} 个校验和不符沿用旧版",
                outcome.loaded_entities, outcome.checksum_mismatches
            )),
            Ok(outcome) => Verdict::Pass(format!(
                "加载 {} 个法人分片，对象绑定 {} 行",
                outcome.loaded_entities, outcome.bindings
            )),
            Err(e) => Verdict::Fail(format!("快照加载失败：{}", e.message)),
        }
    }
}

/// 构建授权域装配。数据库装配在场即可构建：降级台账取既有载体，
/// 快照读取走 ro 池。
pub fn build(
    db: &DbAssembly,
    authz: &AuthzCfg,
    registry: Arc<MetricsRegistry>,
    logger: Arc<JsonLogger>,
) -> Arc<AuthzAssembly> {
    let metrics: Arc<dyn AuthzMetricsSink> = Arc::new(ObsAuthzMetricsSink::new(registry));
    let holder = Arc::new(AuthzSnapshotHolder::empty());
    let query: Arc<dyn AuthzConfigVersionQuery> = Arc::new(PgAuthzConfigVersionQuery::new());
    let opener: Arc<dyn DegradationWindowOpener> = Arc::new(ChecksumMismatchOpener::new(
        db.ledger.clone(),
        logger.clone(),
    ));
    let poller = Arc::new(SnapshotPoller::new(
        holder.clone(),
        query,
        opener,
        db.legal_entities.clone(),
        db.uow_ro.clone(),
        Duration::from_millis(u64::from(authz.snapshot.poll_interval_ms)),
        logger,
    ));
    Arc::new(AuthzAssembly {
        holder,
        poller,
        metrics,
    })
}

#[allow(dead_code)]
const _: fn() = || {
    fn assert_send_sync<T: Send + Sync>() {}
    assert_send_sync::<AuthzAssembly>();
    assert_send_sync::<ObsAuthzMetricsSink>();
    assert_send_sync::<ChecksumMismatchOpener>();
    assert_send_sync::<SnapshotPoller>();
};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn poll_interval_defaults_to_two_seconds() {
        let cfg = AuthzCfg::default();
        assert_eq!(
            Duration::from_millis(u64::from(cfg.snapshot.poll_interval_ms)),
            Duration::from_secs(2)
        );
    }
}
