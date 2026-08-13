//! 账号管理域：批量导入（04 §5.2）——200 行上限、逐行独立事务落库、
//! 失败行成批退回（成功行不回滚）。
//!
//! 每行 = 建账号 + 建口令凭据 + 写口令历史，单事务提交；行与行互不影响。
//! 部分失败时端点经 [`partial_failed_error`] 以 409 退回失败明细。

use std::sync::Arc;

use ep_foundation::error::codes::{
    PLATFORM_REQUEST_INVALID_PAYLOAD, PLATFORM_USER_ACCOUNT_BATCH_PARTIAL_FAILED,
};
use ep_foundation::error::AppError;
use ep_foundation::id::marker::LegalEntity;
use ep_foundation::id::Id;
use ep_foundation::port::tx::UnitOfWork;
use ep_foundation::security::context::SecurityContext;

use crate::config::IdentityPolicies;
use crate::password::{check_policy, PasswordService};
use crate::ports::{
    AccountStore, CredentialStore, NewAccount, NewCredential, PasswordHistoryStore,
};
use crate::types::{AccountKind, CredentialKind};

/// 批量导入行数上限（04 §5.2）。
pub const IMPORT_BATCH_MAX_ROWS: usize = 200;

/// 导入单行入参。初始口令仅驻留本结构，哈希后落库，任何日志不得引用。
pub struct ImportAccountRow {
    pub account_kind: AccountKind,
    pub login_name: String,
    pub employee_no: Option<String>,
    pub display_name: String,
    pub home_legal_entity_id: Id<LegalEntity>,
    pub clearance_level: u8,
    pub is_mfa_required: bool,
    pub initial_password: String,
}

/// 失败行明细：行号（1 基）+ 登录名 + 理由。
#[derive(Debug)]
pub struct ImportFailure {
    pub line_no: usize,
    pub login_name: String,
    pub reason: String,
}

/// 导入产物：成功计数与失败明细。
#[derive(Debug)]
pub struct ImportOutcome {
    pub imported: u64,
    pub failures: Vec<ImportFailure>,
}

/// 部分失败时的 409 错误构造面（端点层调用，失败明细并入消息）。
pub fn partial_failed_error(outcome: &ImportOutcome) -> Option<AppError> {
    if outcome.failures.is_empty() {
        return None;
    }
    let detail: Vec<String> = outcome
        .failures
        .iter()
        .map(|f| format!("第 {} 行({})：{}", f.line_no, f.login_name, f.reason))
        .collect();
    Some(AppError::new(
        PLATFORM_USER_ACCOUNT_BATCH_PARTIAL_FAILED,
        format!("批量导入 {} 行失败：{}", detail.len(), detail.join("；")),
    ))
}

/// 账号管理用例。
pub struct AccountAdminService<U: UnitOfWork> {
    uow: Arc<U>,
    accounts: Arc<dyn AccountStore>,
    credentials: Arc<dyn CredentialStore>,
    password_history: Arc<dyn PasswordHistoryStore>,
    password_service: Arc<PasswordService>,
    policies: IdentityPolicies,
}

impl<U: UnitOfWork> AccountAdminService<U> {
    pub fn new(
        uow: Arc<U>,
        accounts: Arc<dyn AccountStore>,
        credentials: Arc<dyn CredentialStore>,
        password_history: Arc<dyn PasswordHistoryStore>,
        password_service: Arc<PasswordService>,
        policies: IdentityPolicies,
    ) -> Self {
        Self {
            uow,
            accounts,
            credentials,
            password_history,
            password_service,
            policies,
        }
    }

    /// 批量导入：逐行独立事务；单行失败不影响其余行（失败行退回）。
    pub async fn import_batch(
        &self,
        ctx: &SecurityContext,
        rows: Vec<ImportAccountRow>,
    ) -> Result<ImportOutcome, AppError> {
        if rows.is_empty() {
            return Err(AppError::new(
                PLATFORM_REQUEST_INVALID_PAYLOAD,
                "批量导入不得为空",
            ));
        }
        if rows.len() > IMPORT_BATCH_MAX_ROWS {
            return Err(AppError::new(
                PLATFORM_REQUEST_INVALID_PAYLOAD,
                format!("批量导入行数上限 {IMPORT_BATCH_MAX_ROWS}"),
            ));
        }
        let mut imported = 0u64;
        let mut failures = Vec::new();
        for (idx, row) in rows.into_iter().enumerate() {
            let login_name = row.login_name.clone();
            match self.import_one(ctx, row).await {
                Ok(()) => imported += 1,
                Err(e) => failures.push(ImportFailure {
                    line_no: idx + 1,
                    login_name,
                    reason: e.message,
                }),
            }
        }
        Ok(ImportOutcome { imported, failures })
    }

    /// 单行导入：策略校验与哈希在事务外（CPU 工作），落库三写单事务。
    async fn import_one(
        &self,
        ctx: &SecurityContext,
        row: ImportAccountRow,
    ) -> Result<(), AppError> {
        if row.login_name.trim().is_empty() {
            return Err(AppError::new(
                PLATFORM_REQUEST_INVALID_PAYLOAD,
                "登录名不得为空",
            ));
        }
        if row.display_name.trim().is_empty() {
            return Err(AppError::new(
                PLATFORM_REQUEST_INVALID_PAYLOAD,
                "显示名不得为空",
            ));
        }
        check_policy(&row.initial_password, &self.policies.password)?;
        let verifier = self.password_service.hash(&row.initial_password)?;
        let (accounts, credentials, history) = (
            self.accounts.clone(),
            self.credentials.clone(),
            self.password_history.clone(),
        );
        let created_by = ctx.user_id;
        self.uow
            .transact(ctx, move |tx| {
                Box::pin(async move {
                    if accounts
                        .find_by_login_name(tx, &row.login_name)
                        .await?
                        .is_some()
                    {
                        return Err(AppError::new(
                            PLATFORM_REQUEST_INVALID_PAYLOAD,
                            "登录名已存在",
                        ));
                    }
                    let user_id = accounts
                        .insert(
                            tx,
                            NewAccount {
                                account_kind: row.account_kind,
                                login_name: row.login_name,
                                employee_no: row.employee_no,
                                display_name: row.display_name,
                                home_legal_entity_id: row.home_legal_entity_id,
                                clearance_level: row.clearance_level,
                                is_mfa_required: row.is_mfa_required,
                            },
                        )
                        .await?;
                    credentials
                        .insert(
                            tx,
                            NewCredential {
                                user_id,
                                credential_kind: CredentialKind::Password,
                                verifier: Some(verifier.clone()),
                                public_key: None,
                                credential_handle: None,
                                secret_ref: None,
                            },
                        )
                        .await?;
                    history.append(tx, user_id, verifier, created_by).await
                })
            })
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::Argon2Params;
    use crate::context_build::pre_auth_context;
    use crate::testutil::{
        lock, mem, InMemoryUow, MemAccountStore, MemCredentialStore, MemHandle,
        MemPasswordHistoryStore,
    };
    use crate::types::AccountStatus;

    fn ctx() -> SecurityContext {
        let account = crate::types::UserAccountRow {
            id: Id::from_uuid(uuid::Uuid::from_u128(0xAD41)),
            account_kind: AccountKind::Employee,
            login_name: "admin".into(),
            employee_no: None,
            display_name: "Admin".into(),
            home_legal_entity_id: Id::<LegalEntity>::from_uuid(uuid::Uuid::from_u128(2)),
            status: AccountStatus::Active,
            clearance_level: 20,
            security_level: 30,
            is_mfa_required: false,
            created_at: chrono::Utc::now(),
        };
        pre_auth_context(&account, "DEV-01", "admin00001", &"0".repeat(32)).expect("合法")
    }

    fn svc(h: &MemHandle) -> AccountAdminService<InMemoryUow> {
        let pws = Arc::new(
            PasswordService::new(Argon2Params {
                memory_kib: 8,
                iterations: 1,
                parallelism: 1,
            })
            .expect("参数合法"),
        );
        AccountAdminService::new(
            Arc::new(InMemoryUow),
            Arc::new(MemAccountStore(h.clone())),
            Arc::new(MemCredentialStore(h.clone())),
            Arc::new(MemPasswordHistoryStore(h.clone())),
            pws,
            IdentityPolicies::default(),
        )
    }

    fn row(login: &str, password: &str) -> ImportAccountRow {
        ImportAccountRow {
            account_kind: AccountKind::Employee,
            login_name: login.to_string(),
            employee_no: None,
            display_name: login.to_string(),
            home_legal_entity_id: Id::<LegalEntity>::from_uuid(uuid::Uuid::from_u128(2)),
            clearance_level: 20,
            is_mfa_required: false,
            initial_password: password.to_string(),
        }
    }

    #[tokio::test]
    async fn batch_bounds_are_enforced() {
        let h = mem();
        let s = svc(&h);
        let err = s.import_batch(&ctx(), vec![]).await.expect_err("空批拒");
        assert_eq!(err.code, PLATFORM_REQUEST_INVALID_PAYLOAD);
        let big: Vec<_> = (0..201)
            .map(|i| row(&format!("u{i:04}"), "Ab1!Ab1!Ab1!"))
            .collect();
        let err = s.import_batch(&ctx(), big).await.expect_err("超上限拒");
        assert_eq!(err.code, PLATFORM_REQUEST_INVALID_PAYLOAD);
        assert!(err.message.contains("200"));
    }

    #[tokio::test]
    async fn partial_failure_commits_good_rows_and_returns_bad() {
        let h = mem();
        let s = svc(&h);
        // 预植重名账号。
        lock(&h).accounts.push(crate::types::UserAccountRow {
            id: Id::from_uuid(uuid::Uuid::from_u128(0x99)),
            account_kind: AccountKind::Employee,
            login_name: "dup".into(),
            employee_no: None,
            display_name: "Dup".into(),
            home_legal_entity_id: Id::<LegalEntity>::from_uuid(uuid::Uuid::from_u128(2)),
            status: AccountStatus::Active,
            clearance_level: 20,
            security_level: 30,
            is_mfa_required: false,
            created_at: chrono::Utc::now(),
        });
        let rows = vec![
            row("ok-one", "Ab1!Ab1!Ab1!"),
            row("dup", "Ab1!Ab1!Ab1!"),
            row("weak", "short"),
        ];
        let out = s.import_batch(&ctx(), rows).await.expect("成批退回不抛");
        assert_eq!(out.imported, 1, "成功行不回滚");
        assert_eq!(out.failures.len(), 2);
        assert_eq!(out.failures[0].line_no, 2);
        assert_eq!(out.failures[0].login_name, "dup");
        assert!(out.failures[0].reason.contains("已存在"));
        assert_eq!(out.failures[1].line_no, 3);
        assert_eq!(lock(&h).accounts.len(), 2, "重名行未建号");
        assert_eq!(lock(&h).credentials.len(), 1, "仅成功行建凭据");
        assert_eq!(lock(&h).password_history.len(), 1, "口令历史随建随记");
        let err = partial_failed_error(&out).expect("部分失败构造 409");
        assert_eq!(err.code, PLATFORM_USER_ACCOUNT_BATCH_PARTIAL_FAILED);
    }

    #[tokio::test]
    async fn full_success_yields_no_failure_error() {
        let h = mem();
        let s = svc(&h);
        let out = s
            .import_batch(
                &ctx(),
                vec![row("neo", "Ab1!Ab1!Ab1!"), row("nia", "Xy9#Xy9#Xy9#")],
            )
            .await
            .expect("全成功");
        assert_eq!(out.imported, 2);
        assert!(out.failures.is_empty());
        assert!(partial_failed_error(&out).is_none());
        let cred = &lock(&h).credentials[0];
        assert_eq!(cred.credential_kind, CredentialKind::Password);
        assert!(
            cred.verifier
                .as_deref()
                .is_some_and(|v| v.starts_with("$argon2id$")),
            "PHC 串落 verifier"
        );
    }
}
