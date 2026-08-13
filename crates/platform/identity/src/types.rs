//! 身份域核心类型。
//!
//! 枚举字面量与九张身份表迁移的 CHECK 约束逐字对应
//! （V202610120900~V202610120940），改字面量即改落库契约。

use chrono::{DateTime, Utc};
use ep_foundation::id::marker::{LegalEntity, UserAccount};
use ep_foundation::id::Id;
use ep_foundation::security::context::ClientKind;

/// `user_accounts.account_kind` 四值。
#[derive(Clone, Copy, PartialEq, Eq, Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum AccountKind {
    Employee,
    Portal,
    Breakglass,
    System,
}

impl AccountKind {
    pub const fn as_str(self) -> &'static str {
        match self {
            AccountKind::Employee => "EMPLOYEE",
            AccountKind::Portal => "PORTAL",
            AccountKind::Breakglass => "BREAKGLASS",
            AccountKind::System => "SYSTEM",
        }
    }

    pub fn parse(raw: &str) -> Option<Self> {
        match raw {
            "EMPLOYEE" => Some(Self::Employee),
            "PORTAL" => Some(Self::Portal),
            "BREAKGLASS" => Some(Self::Breakglass),
            "SYSTEM" => Some(Self::System),
            _ => None,
        }
    }
}

/// `user_accounts.status` 五态。
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum AccountStatus {
    Unactivated,
    Active,
    Locked,
    Suspended,
    Deactivated,
}

impl AccountStatus {
    pub const fn as_str(self) -> &'static str {
        match self {
            AccountStatus::Unactivated => "UNACTIVATED",
            AccountStatus::Active => "ACTIVE",
            AccountStatus::Locked => "LOCKED",
            AccountStatus::Suspended => "SUSPENDED",
            AccountStatus::Deactivated => "DEACTIVATED",
        }
    }

    pub fn parse(raw: &str) -> Option<Self> {
        match raw {
            "UNACTIVATED" => Some(Self::Unactivated),
            "ACTIVE" => Some(Self::Active),
            "LOCKED" => Some(Self::Locked),
            "SUSPENDED" => Some(Self::Suspended),
            "DEACTIVATED" => Some(Self::Deactivated),
            _ => None,
        }
    }

    /// 可登录状态：仅 ACTIVE 放行，其余走 ACCOUNT_INACTIVE 语义。
    pub const fn is_signable(self) -> bool {
        matches!(self, AccountStatus::Active)
    }
}

/// `user_credentials.credential_kind` 五值。
#[derive(Clone, Copy, PartialEq, Eq, Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum CredentialKind {
    Password,
    Totp,
    WebauthnPlatform,
    WebauthnRoaming,
    X509Cert,
}

impl CredentialKind {
    pub const fn as_str(self) -> &'static str {
        match self {
            CredentialKind::Password => "PASSWORD",
            CredentialKind::Totp => "TOTP",
            CredentialKind::WebauthnPlatform => "WEBAUTHN_PLATFORM",
            CredentialKind::WebauthnRoaming => "WEBAUTHN_ROAMING",
            CredentialKind::X509Cert => "X509_CERT",
        }
    }

    pub fn parse(raw: &str) -> Option<Self> {
        match raw {
            "PASSWORD" => Some(Self::Password),
            "TOTP" => Some(Self::Totp),
            "WEBAUTHN_PLATFORM" => Some(Self::WebauthnPlatform),
            "WEBAUTHN_ROAMING" => Some(Self::WebauthnRoaming),
            "X509_CERT" => Some(Self::X509Cert),
            _ => None,
        }
    }

    /// 第二因子集合：PASSWORD 之外的四类。
    pub const fn is_second_factor(self) -> bool {
        !matches!(self, CredentialKind::Password)
    }
}

/// `user_credentials.status` 四态。
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum CredentialStatus {
    Active,
    Suspended,
    Revoked,
    Expired,
}

impl CredentialStatus {
    pub const fn as_str(self) -> &'static str {
        match self {
            CredentialStatus::Active => "ACTIVE",
            CredentialStatus::Suspended => "SUSPENDED",
            CredentialStatus::Revoked => "REVOKED",
            CredentialStatus::Expired => "EXPIRED",
        }
    }

    pub fn parse(raw: &str) -> Option<Self> {
        match raw {
            "ACTIVE" => Some(Self::Active),
            "SUSPENDED" => Some(Self::Suspended),
            "REVOKED" => Some(Self::Revoked),
            "EXPIRED" => Some(Self::Expired),
            _ => None,
        }
    }
}

/// `user_devices.status` 三态。
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum DeviceStatus {
    Pending,
    Active,
    Revoked,
}

impl DeviceStatus {
    pub const fn as_str(self) -> &'static str {
        match self {
            DeviceStatus::Pending => "PENDING",
            DeviceStatus::Active => "ACTIVE",
            DeviceStatus::Revoked => "REVOKED",
        }
    }

    pub fn parse(raw: &str) -> Option<Self> {
        match raw {
            "PENDING" => Some(Self::Pending),
            "ACTIVE" => Some(Self::Active),
            "REVOKED" => Some(Self::Revoked),
            _ => None,
        }
    }
}

/// `login_attempts.outcome` 八值。
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum LoginAttemptOutcome {
    Success,
    CredentialInvalid,
    AccountLocked,
    AccountInactive,
    MfaRequired,
    MfaInvalid,
    DeviceUnregistered,
    AdmissionRejected,
}

impl LoginAttemptOutcome {
    pub const fn as_str(self) -> &'static str {
        match self {
            LoginAttemptOutcome::Success => "SUCCESS",
            LoginAttemptOutcome::CredentialInvalid => "CREDENTIAL_INVALID",
            LoginAttemptOutcome::AccountLocked => "ACCOUNT_LOCKED",
            LoginAttemptOutcome::AccountInactive => "ACCOUNT_INACTIVE",
            LoginAttemptOutcome::MfaRequired => "MFA_REQUIRED",
            LoginAttemptOutcome::MfaInvalid => "MFA_INVALID",
            LoginAttemptOutcome::DeviceUnregistered => "DEVICE_UNREGISTERED",
            LoginAttemptOutcome::AdmissionRejected => "ADMISSION_REJECTED",
        }
    }
}

/// `breakglass_activations.allowed_action_set` 三类取值（CHECK 已限定）。
#[derive(Clone, Copy, PartialEq, Eq, Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum BreakglassAction {
    UnlockOrResetAdmin,
    RestoreControlledConfigRelease,
    TriggerBackupOrRestore,
}

impl BreakglassAction {
    pub const ALL: [BreakglassAction; 3] = [
        BreakglassAction::UnlockOrResetAdmin,
        BreakglassAction::RestoreControlledConfigRelease,
        BreakglassAction::TriggerBackupOrRestore,
    ];

    pub const fn as_str(self) -> &'static str {
        match self {
            BreakglassAction::UnlockOrResetAdmin => "UNLOCK_OR_RESET_ADMIN",
            BreakglassAction::RestoreControlledConfigRelease => "RESTORE_CONTROLLED_CONFIG_RELEASE",
            BreakglassAction::TriggerBackupOrRestore => "TRIGGER_BACKUP_OR_RESTORE",
        }
    }

    pub fn parse(raw: &str) -> Option<Self> {
        match raw {
            "UNLOCK_OR_RESET_ADMIN" => Some(Self::UnlockOrResetAdmin),
            "RESTORE_CONTROLLED_CONFIG_RELEASE" => Some(Self::RestoreControlledConfigRelease),
            "TRIGGER_BACKUP_OR_RESTORE" => Some(Self::TriggerBackupOrRestore),
            _ => None,
        }
    }
}

/// `breakglass_activations.status` 七态。
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum BreakglassStatus {
    Draft,
    PendingApproval,
    Approved,
    Active,
    Expired,
    Closed,
    Rejected,
}

impl BreakglassStatus {
    pub const fn as_str(self) -> &'static str {
        match self {
            BreakglassStatus::Draft => "DRAFT",
            BreakglassStatus::PendingApproval => "PENDING_APPROVAL",
            BreakglassStatus::Approved => "APPROVED",
            BreakglassStatus::Active => "ACTIVE",
            BreakglassStatus::Expired => "EXPIRED",
            BreakglassStatus::Closed => "CLOSED",
            BreakglassStatus::Rejected => "REJECTED",
        }
    }

    pub fn parse(raw: &str) -> Option<Self> {
        match raw {
            "DRAFT" => Some(Self::Draft),
            "PENDING_APPROVAL" => Some(Self::PendingApproval),
            "APPROVED" => Some(Self::Approved),
            "ACTIVE" => Some(Self::Active),
            "EXPIRED" => Some(Self::Expired),
            "CLOSED" => Some(Self::Closed),
            "REJECTED" => Some(Self::Rejected),
            _ => None,
        }
    }

    /// 状态机合法性：DRAFT→PENDING_APPROVAL→{APPROVED,REJECTED}；
    /// APPROVED→ACTIVE→{EXPIRED,CLOSED}。
    pub const fn can_transition_to(self, next: BreakglassStatus) -> bool {
        matches!(
            (self, next),
            (BreakglassStatus::Draft, BreakglassStatus::PendingApproval)
                | (
                    BreakglassStatus::PendingApproval,
                    BreakglassStatus::Approved
                )
                | (
                    BreakglassStatus::PendingApproval,
                    BreakglassStatus::Rejected
                )
                | (BreakglassStatus::Approved, BreakglassStatus::Active)
                | (BreakglassStatus::Active, BreakglassStatus::Expired)
                | (BreakglassStatus::Active, BreakglassStatus::Closed)
        )
    }
}

/// `user_accounts` 读取行。
#[derive(Clone, Debug)]
pub struct UserAccountRow {
    pub id: Id<UserAccount>,
    pub account_kind: AccountKind,
    pub login_name: String,
    pub employee_no: Option<String>,
    pub display_name: String,
    pub home_legal_entity_id: Id<LegalEntity>,
    pub status: AccountStatus,
    pub clearance_level: u8,
    pub security_level: u8,
    pub is_mfa_required: bool,
    pub created_at: DateTime<Utc>,
}

/// `user_credentials` 读取行。
#[derive(Clone, Debug)]
pub struct CredentialRow {
    pub id: uuid::Uuid,
    pub user_id: Id<UserAccount>,
    pub credential_kind: CredentialKind,
    pub verifier: Option<String>,
    pub public_key: Option<String>,
    pub credential_handle: Option<String>,
    pub secret_ref: Option<String>,
    pub sign_count: i64,
    pub status: CredentialStatus,
    pub security_level: u8,
    /// 口令有效期判定（max_age_days）的时间基准。
    pub created_at: DateTime<Utc>,
}

/// `user_devices` 读取行。
#[derive(Clone, Debug)]
pub struct DeviceRow {
    pub id: uuid::Uuid,
    pub user_id: Id<UserAccount>,
    pub device_id: String,
    pub client: ClientKind,
    pub public_key: Option<String>,
    pub attestation_ref: Option<String>,
    pub restricted_legal_entity_id: Option<Id<LegalEntity>>,
    pub status: DeviceStatus,
}

/// `sessions` 读取行。
#[derive(Clone, Debug)]
pub struct SessionRow {
    pub id: uuid::Uuid,
    pub user_id: Id<UserAccount>,
    pub user_device_row_id: uuid::Uuid,
    pub token_hash: Vec<u8>,
    pub active_legal_entity_id: Id<LegalEntity>,
    pub issued_at: DateTime<Utc>,
    pub expires_at: DateTime<Utc>,
    pub idle_expires_at: DateTime<Utc>,
    pub last_seen_at: DateTime<Utc>,
    pub revoked_at: Option<DateTime<Utc>>,
    pub revoke_reason: Option<String>,
    pub is_breakglass: bool,
}

/// `account_lockouts` 读取行（id 为 pk，user_id 唯一索引）。
#[derive(Clone, Debug)]
pub struct LockoutRow {
    pub id: uuid::Uuid,
    pub user_id: Id<UserAccount>,
    pub failure_count: u32,
    pub window_started_at: Option<DateTime<Utc>>,
    pub locked_until: Option<DateTime<Utc>>,
    pub last_failure_at: Option<DateTime<Utc>>,
}

/// `breakglass_activations` 读取行。
#[derive(Clone, Debug)]
pub struct BreakglassRow {
    pub id: uuid::Uuid,
    pub doc_no: String,
    pub status: BreakglassStatus,
    pub user_id: Id<UserAccount>,
    pub requested_by: Id<UserAccount>,
    pub approved_by: Option<Id<UserAccount>>,
    pub reason: String,
    pub approval_ref: Option<String>,
    pub allowed_action_set: Vec<BreakglassAction>,
    pub activated_at: Option<DateTime<Utc>>,
    pub expires_at: Option<DateTime<Utc>>,
    pub closed_at: Option<DateTime<Utc>>,
    pub rotated_at: Option<DateTime<Utc>>,
    pub rotation_result: Option<String>,
}

/// 会话撤销理由的落库字面量。
pub const REVOKE_SESSION_LIMIT_EXCEEDED: &str = "SESSION_LIMIT_EXCEEDED";
/// 停用级联撤销的理由。
pub const REVOKE_ACCOUNT_DEACTIVATED: &str = "ACCOUNT_DEACTIVATED";
/// 设备注销级联撤销的理由。
pub const REVOKE_DEVICE_REVOKED: &str = "DEVICE_REVOKED";
/// 主动登出。
pub const REVOKE_SIGN_OUT: &str = "SIGN_OUT";
/// 应急账号到期失效。
pub const REVOKE_BREAKGLASS_EXPIRED: &str = "BREAKGLASS_EXPIRED";

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn account_literals_match_migration_checks() {
        assert_eq!(AccountKind::Employee.as_str(), "EMPLOYEE");
        assert_eq!(
            AccountKind::parse("BREAKGLASS"),
            Some(AccountKind::Breakglass)
        );
        assert_eq!(
            AccountStatus::parse("UNACTIVATED"),
            Some(AccountStatus::Unactivated)
        );
        assert!(AccountStatus::parse("FROZEN").is_none());
    }

    #[test]
    fn credential_kind_second_factor_excludes_password() {
        assert!(!CredentialKind::Password.is_second_factor());
        for k in [
            CredentialKind::Totp,
            CredentialKind::WebauthnPlatform,
            CredentialKind::WebauthnRoaming,
            CredentialKind::X509Cert,
        ] {
            assert!(k.is_second_factor(), "{k:?} 是第二因子");
        }
    }

    #[test]
    fn login_attempt_outcomes_cover_eight_literals() {
        let all = [
            LoginAttemptOutcome::Success,
            LoginAttemptOutcome::CredentialInvalid,
            LoginAttemptOutcome::AccountLocked,
            LoginAttemptOutcome::AccountInactive,
            LoginAttemptOutcome::MfaRequired,
            LoginAttemptOutcome::MfaInvalid,
            LoginAttemptOutcome::DeviceUnregistered,
            LoginAttemptOutcome::AdmissionRejected,
        ];
        let want = [
            "SUCCESS",
            "CREDENTIAL_INVALID",
            "ACCOUNT_LOCKED",
            "ACCOUNT_INACTIVE",
            "MFA_REQUIRED",
            "MFA_INVALID",
            "DEVICE_UNREGISTERED",
            "ADMISSION_REJECTED",
        ];
        for (i, o) in all.iter().enumerate() {
            assert_eq!(o.as_str(), want[i]);
        }
    }

    #[test]
    fn breakglass_state_machine_is_exactly_six_edges() {
        use BreakglassStatus::*;
        let legal = [
            (Draft, PendingApproval),
            (PendingApproval, Approved),
            (PendingApproval, Rejected),
            (Approved, Active),
            (Active, Expired),
            (Active, Closed),
        ];
        for (from, to) in legal {
            assert!(from.can_transition_to(to), "{from:?}->{to:?} 应合法");
        }
        assert!(!Active.can_transition_to(Draft));
        assert!(!Closed.can_transition_to(Active), "终态不可回迁");
        assert!(!Expired.can_transition_to(Closed), "到期后不再主动关闭");
    }

    #[test]
    fn breakglass_actions_match_the_three_check_values() {
        let want = [
            "UNLOCK_OR_RESET_ADMIN",
            "RESTORE_CONTROLLED_CONFIG_RELEASE",
            "TRIGGER_BACKUP_OR_RESTORE",
        ];
        for (i, a) in BreakglassAction::ALL.iter().enumerate() {
            assert_eq!(a.as_str(), want[i]);
            assert_eq!(BreakglassAction::parse(want[i]), Some(*a));
        }
    }
}
