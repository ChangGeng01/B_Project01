//! 身份域持久化端口与审计/事件占位调用面。
//!
//! platform 不依赖 adapter：全部 SQL 实现体落 ep-adapter-db-pg
//! （platform_core/identity*.rs 与 platform_authz/user_grants.rs），
//! 与阶段 2 tenancy 的 downcast 纪律同构。

use chrono::{DateTime, Utc};
use ep_foundation::error::AppError;
use ep_foundation::id::marker::{
    Customer, Department, LegalEntity, Position, Project, UserAccount,
};
use ep_foundation::id::Id;
use ep_foundation::port::tx::Tx;
use ep_foundation::security::context::{ClientKind, DutyClass};

use crate::types::{
    AccountKind, BreakglassAction, BreakglassRow, BreakglassStatus, CredentialKind, CredentialRow,
    CredentialStatus, DeviceRow, LockoutRow, LoginAttemptOutcome, SessionRow, UserAccountRow,
};

/// 建号入参。
#[derive(Clone, Debug)]
pub struct NewAccount {
    pub account_kind: AccountKind,
    pub login_name: String,
    pub employee_no: Option<String>,
    pub display_name: String,
    pub home_legal_entity_id: Id<LegalEntity>,
    pub clearance_level: u8,
    pub is_mfa_required: bool,
}

/// 建凭据入参。载体按 kind 由迁移 CHECK 强制。
#[derive(Clone, Debug)]
pub struct NewCredential {
    pub user_id: Id<UserAccount>,
    pub credential_kind: CredentialKind,
    pub verifier: Option<String>,
    pub public_key: Option<String>,
    pub credential_handle: Option<String>,
    pub secret_ref: Option<String>,
}

/// 设备登记入参。
#[derive(Clone, Debug)]
pub struct NewDevice {
    pub user_id: Id<UserAccount>,
    pub device_id: String,
    pub client: ClientKind,
    pub public_key: Option<String>,
    pub attestation_ref: Option<String>,
    pub restricted_legal_entity_id: Option<Id<LegalEntity>>,
}

/// 会话插入入参（令牌摘要入库，明文不出现）。
#[derive(Clone, Debug)]
pub struct NewSession {
    pub user_id: Id<UserAccount>,
    pub user_device_row_id: uuid::Uuid,
    pub token_hash: Vec<u8>,
    pub active_legal_entity_id: Id<LegalEntity>,
    pub issued_at: DateTime<Utc>,
    pub expires_at: DateTime<Utc>,
    pub idle_expires_at: DateTime<Utc>,
    pub is_breakglass: bool,
}

/// 登录尝试流水入参（仅追加）。登录名以 SHA-256 摘要落库。
#[derive(Clone, Debug)]
pub struct NewLoginAttempt {
    pub user_id: Option<Id<UserAccount>>,
    pub login_name_hash: Vec<u8>,
    pub outcome: LoginAttemptOutcome,
    pub client: ClientKind,
    pub source_addr: String,
    pub occurred_at: DateTime<Utc>,
}

/// 应急账号启用申请入参。
#[derive(Clone, Debug)]
pub struct NewBreakglass {
    pub doc_no: String,
    pub user_id: Id<UserAccount>,
    pub requested_by: Id<UserAccount>,
    pub reason: String,
    pub allowed_action_set: Vec<BreakglassAction>,
}

/// 用户授权集合：会话建立时读一次冻结进 SecurityContext（04 §4.3 步 9 之前）。
/// 读取面经 platform_authz 六张用户维度表与 authz_config_versions 版本号。
#[derive(Clone, Debug, Default)]
pub struct UserAuthzSet {
    /// 生效角色码（effective_from/to 命中今日且角色 lifecycle ACTIVE）。
    pub role_codes: Vec<String>,
    /// 所授角色的 duty_class 去重集合（业务角色无 duty 不入集）。
    pub duty_classes: Vec<DutyClass>,
    /// 所授角色是否含六类高风险操作权限项之一。
    pub has_high_risk_permission: bool,
    /// 授权法人清单（granted_from/to 命中今日）。
    pub legal_entity_ids: Vec<Id<LegalEntity>>,
    pub department_ids: Vec<Id<Department>>,
    pub position_ids: Vec<Id<Position>>,
    pub project_ids: Vec<Id<Project>>,
    pub customer_ids: Vec<Id<Customer>>,
    /// 记录级共享：(object_type, scope_ref_id)。
    pub record_shares: Vec<(String, uuid::Uuid)>,
    /// user_role_grants.data_scope_tags 的并集。
    pub data_scope_tags: Vec<String>,
    /// 该法人 EFFECTIVE 的 authz_config_versions.version_no。
    pub snapshot_version: u64,
}

impl UserAuthzSet {
    /// 强制 MFA 判据第二支：持任一 duty_class 非空的有效角色授予。
    pub fn has_duty_role(&self) -> bool {
        !self.duty_classes.is_empty()
    }
}

#[async_trait::async_trait]
pub trait AccountStore: Send + Sync {
    async fn find_by_login_name(
        &self,
        tx: &mut dyn Tx,
        login_name: &str,
    ) -> Result<Option<UserAccountRow>, AppError>;
    async fn get(
        &self,
        tx: &mut dyn Tx,
        id: Id<UserAccount>,
    ) -> Result<Option<UserAccountRow>, AppError>;
    async fn insert(&self, tx: &mut dyn Tx, new: NewAccount) -> Result<Id<UserAccount>, AppError>;
    /// 状态迁移（CAS 语义：仅当当前状态为 from 时生效；None 不限制起态）。
    async fn transition_status(
        &self,
        tx: &mut dyn Tx,
        id: Id<UserAccount>,
        from: Option<crate::types::AccountStatus>,
        to: crate::types::AccountStatus,
    ) -> Result<bool, AppError>;
}

#[async_trait::async_trait]
pub trait CredentialStore: Send + Sync {
    /// 用户全部 ACTIVE 凭据。
    async fn list_active(
        &self,
        tx: &mut dyn Tx,
        user_id: Id<UserAccount>,
    ) -> Result<Vec<CredentialRow>, AppError>;
    /// 指定 kind 的 ACTIVE 凭据（至多一行语义由用例保证）。
    async fn active_of_kind(
        &self,
        tx: &mut dyn Tx,
        user_id: Id<UserAccount>,
        kind: CredentialKind,
    ) -> Result<Option<CredentialRow>, AppError>;
    async fn insert(&self, tx: &mut dyn Tx, new: NewCredential) -> Result<uuid::Uuid, AppError>;
    async fn set_status(
        &self,
        tx: &mut dyn Tx,
        credential_id: uuid::Uuid,
        status: CredentialStatus,
    ) -> Result<bool, AppError>;
    /// 撤销用户全部凭据（停用与应急失效的级联面）。
    async fn revoke_all_for_user(
        &self,
        tx: &mut dyn Tx,
        user_id: Id<UserAccount>,
    ) -> Result<u64, AppError>;
    /// WebAuthn 断言成功后推进签名计数（仅向前）。
    async fn bump_sign_count(
        &self,
        tx: &mut dyn Tx,
        credential_id: uuid::Uuid,
        new_count: i64,
    ) -> Result<bool, AppError>;
}

/// 口令历史（仅追加表）：重置口令时校 history_size 代不重复。
#[async_trait::async_trait]
pub trait PasswordHistoryStore: Send + Sync {
    /// 最近 n 代 verifier（按 created_at 倒序）。
    async fn recent_verifiers(
        &self,
        tx: &mut dyn Tx,
        user_id: Id<UserAccount>,
        n: usize,
    ) -> Result<Vec<String>, AppError>;
    async fn append(
        &self,
        tx: &mut dyn Tx,
        user_id: Id<UserAccount>,
        verifier: String,
        created_by: Id<UserAccount>,
    ) -> Result<(), AppError>;
}

#[async_trait::async_trait]
pub trait DeviceStore: Send + Sync {
    async fn find_active(
        &self,
        tx: &mut dyn Tx,
        user_id: Id<UserAccount>,
        device_id: &str,
    ) -> Result<Option<DeviceRow>, AppError>;
    async fn list(
        &self,
        tx: &mut dyn Tx,
        user_id: Id<UserAccount>,
    ) -> Result<Vec<DeviceRow>, AppError>;
    async fn insert(&self, tx: &mut dyn Tx, new: NewDevice) -> Result<uuid::Uuid, AppError>;
    /// 远程注销（级联撤会话由用例编排）。
    async fn revoke(&self, tx: &mut dyn Tx, device_row_id: uuid::Uuid) -> Result<bool, AppError>;
}

#[async_trait::async_trait]
pub trait SessionStore: Send + Sync {
    async fn insert(&self, tx: &mut dyn Tx, new: NewSession) -> Result<uuid::Uuid, AppError>;
    /// 按令牌 SHA-256 摘要取活跃会话（认证中间件主路径，归集成任务消费）。
    async fn find_active_by_digest(
        &self,
        tx: &mut dyn Tx,
        token_hash: &[u8],
    ) -> Result<Option<SessionRow>, AppError>;
    /// 用户未撤销且未过期的会话（issued_at 升序）。
    async fn list_active_for_user(
        &self,
        tx: &mut dyn Tx,
        user_id: Id<UserAccount>,
    ) -> Result<Vec<SessionRow>, AppError>;
    async fn revoke(
        &self,
        tx: &mut dyn Tx,
        session_id: uuid::Uuid,
        reason: &str,
    ) -> Result<bool, AppError>;
    async fn revoke_all_for_user(
        &self,
        tx: &mut dyn Tx,
        user_id: Id<UserAccount>,
        reason: &str,
    ) -> Result<u64, AppError>;
    /// 按设备级联撤销会话（远程注销的级联面），返回处置行数。
    async fn revoke_by_device(
        &self,
        tx: &mut dyn Tx,
        user_id: Id<UserAccount>,
        device_row_id: uuid::Uuid,
        reason: &str,
    ) -> Result<u64, AppError>;
    /// 滑动续期批量写：60 秒粒度合并后的到期时刻一次刷多行。
    async fn extend_idle(
        &self,
        tx: &mut dyn Tx,
        session_ids: &[uuid::Uuid],
        idle_expires_at: DateTime<Utc>,
    ) -> Result<u64, AppError>;
    /// job-worker：到期会话置 revoked（reason EXPIRED），返回处置行数。
    async fn expire_overdue(&self, tx: &mut dyn Tx, now: DateTime<Utc>) -> Result<u64, AppError>;
}

/// 锁定计数（FOR UPDATE 行锁语义在实现体）。
#[async_trait::async_trait]
pub trait LockoutStore: Send + Sync {
    /// 取行并加行锁；无行即建零值行。
    async fn lock_for_update(
        &self,
        tx: &mut dyn Tx,
        user_id: Id<UserAccount>,
    ) -> Result<LockoutRow, AppError>;
    /// 记一次失败并按策略推进窗口与锁定，返回更新后状态。
    async fn record_failure(
        &self,
        tx: &mut dyn Tx,
        row: &LockoutRow,
        max_failures: u32,
        window_seconds: u64,
        duration_seconds: u64,
        now: DateTime<Utc>,
    ) -> Result<LockoutRow, AppError>;
    /// 登录成功后重置计数。
    async fn reset(&self, tx: &mut dyn Tx, user_id: Id<UserAccount>) -> Result<(), AppError>;
}

#[async_trait::async_trait]
pub trait LoginAttemptStore: Send + Sync {
    async fn append(&self, tx: &mut dyn Tx, new: NewLoginAttempt) -> Result<(), AppError>;
}

#[async_trait::async_trait]
pub trait BreakglassStore: Send + Sync {
    async fn insert(&self, tx: &mut dyn Tx, new: NewBreakglass) -> Result<uuid::Uuid, AppError>;
    async fn get(&self, tx: &mut dyn Tx, id: uuid::Uuid)
        -> Result<Option<BreakglassRow>, AppError>;
    /// 条件状态迁移：仅当现状态等于 from 时落 to，返回是否生效。
    async fn transition(
        &self,
        tx: &mut dyn Tx,
        id: uuid::Uuid,
        from: BreakglassStatus,
        to: BreakglassStatus,
    ) -> Result<bool, AppError>;
    /// 批准：写 approved_by、approval_ref 并迁移 APPROVED。
    async fn approve(
        &self,
        tx: &mut dyn Tx,
        id: uuid::Uuid,
        approved_by: Id<UserAccount>,
        approval_ref: &str,
    ) -> Result<bool, AppError>;
    /// 启用：写 activated_at 与 expires_at 并迁移 ACTIVE。
    async fn activate(
        &self,
        tx: &mut dyn Tx,
        id: uuid::Uuid,
        activated_at: DateTime<Utc>,
        expires_at: DateTime<Utc>,
    ) -> Result<bool, AppError>;
    /// 关闭：写 closed_at 并迁移 CLOSED。
    async fn close(
        &self,
        tx: &mut dyn Tx,
        id: uuid::Uuid,
        now: DateTime<Utc>,
    ) -> Result<bool, AppError>;
    /// 到期失效：写 rotation_result 并迁移 EXPIRED/CLOSED 终态。
    async fn finalize_with_rotation(
        &self,
        tx: &mut dyn Tx,
        id: uuid::Uuid,
        to: BreakglassStatus,
        rotation_result: &str,
        now: DateTime<Utc>,
    ) -> Result<bool, AppError>;
    /// ACTIVE 且 expires_at 已到（job-worker 到期失效）。
    async fn list_due_active(
        &self,
        tx: &mut dyn Tx,
        now: DateTime<Utc>,
    ) -> Result<Vec<BreakglassRow>, AppError>;
    /// CLOSED/EXPIRED 且 rotated_at 早于 cutoff 或为空（闲置轮换）。
    async fn list_idle_for_rotation(
        &self,
        tx: &mut dyn Tx,
        cutoff: DateTime<Utc>,
    ) -> Result<Vec<BreakglassRow>, AppError>;
    /// 闲置轮换登记：写 rotated_at 与 rotation_result（仅终态行）。
    async fn mark_rotated(
        &self,
        tx: &mut dyn Tx,
        id: uuid::Uuid,
        rotation_result: &str,
        now: DateTime<Utc>,
    ) -> Result<bool, AppError>;
}

/// platform_authz 六张用户维度表的读取面（实现体在 db-pg platform_authz/）。
#[async_trait::async_trait]
pub trait UserAuthzQuery: Send + Sync {
    /// 读授权集合。`home_legal_entity_id` 为属籍法人（快照版本号的
    /// 定位键），由调用方从账号行携带，避免读取面跨 schema 取
    /// user_accounts。
    async fn load_user_authz(
        &self,
        tx: &mut dyn Tx,
        user_id: Id<UserAccount>,
        home_legal_entity_id: Id<LegalEntity>,
    ) -> Result<UserAuthzSet, AppError>;
    /// 用户所授角色的 duty_class 去重集合（应急批准人 SECURITY/AUDIT 判据）。
    async fn user_duty_classes(
        &self,
        tx: &mut dyn Tx,
        user_id: Id<UserAccount>,
    ) -> Result<Vec<DutyClass>, AppError>;
    /// 用户名下未结束的审批待办数（transfer 前置校验）。
    async fn count_open_high_risk_requests(
        &self,
        tx: &mut dyn Tx,
        user_id: Id<UserAccount>,
    ) -> Result<u64, AppError>;
    /// 已安装法人清单（me/legal-entities 逐法人探测的枚举源）。
    async fn installed_legal_entities(
        &self,
        tx: &mut dyn Tx,
    ) -> Result<Vec<Id<LegalEntity>>, AppError>;
    /// 逐法人探测：设置法人上下文后 user_legal_entity_grants 是否可见。
    async fn probe_legal_entity_grant(
        &self,
        tx: &mut dyn Tx,
        user_id: Id<UserAccount>,
        legal_entity_id: Id<LegalEntity>,
    ) -> Result<bool, AppError>;
}

/// job-worker：过期重新认证挑战清理（authz 域表，清理由本任务承担）。
#[async_trait::async_trait]
pub trait ChallengeCleanup: Send + Sync {
    /// ISSUED/VERIFIED 且 expires_at 已过的挑战置 EXPIRED。
    async fn expire_overdue(&self, tx: &mut dyn Tx, now: DateTime<Utc>) -> Result<u64, AppError>;
}

/// 审计事件记录的占位调用面：写入本体归阶段 3b
/// （platform_audit.audit_events 表随 3b 建立）。本 crate 只冻结调用面，
/// wiring 侧实现体以结构化日志记录（同阶段 2 events.rs 先例），不静默丢弃。
pub trait AuditRecorder: Send + Sync {
    fn record(&self, kind: &str, detail: &str);
}

/// Outbox 待发出的事件记录面。platform_msg.outbox_events 表属阶段 3b，
/// 本阶段以结构化日志记下本应发出的事件（同阶段 2 record_pending_emit 先例）。
pub trait PendingEventRecorder: Send + Sync {
    fn record_pending(&self, event_type: &str, subject: &str);
}

/// platform_ops 台账告警开口：应急账号启用即写（降级窗口端口复用面）。
/// 台账本体归阶段 2 已交付的 degradation ledger；此处以告警 kind 记窗口。
pub trait OpsAlertRecorder: Send + Sync {
    fn alert_breakglass_activated(&self, doc_no: &str, user_id: &str);
}
