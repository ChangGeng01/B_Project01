//! 卫生域：过期会话与过期登录挑战的批量失效（job-worker 入口）。
//!
//! 两个后台任务的用例面：任务 1 调 [`HygieneService::expire_sessions`]
//! 与 [`HygieneService::expire_challenges`]；应急账号到期失效与轮换
//! 在 [`crate::breakglass::BreakglassService`]（expire_due/rotate_idle）。

use std::sync::Arc;

use chrono::{DateTime, Utc};
use ep_foundation::error::AppError;
use ep_foundation::port::tx::UnitOfWork;
use ep_foundation::security::context::SecurityContext;

use crate::ports::{ChallengeCleanup, SessionStore};

/// 清理用例：由 job-worker 周期性调用，事务法人上下文取系统主体。
pub struct HygieneService<U: UnitOfWork> {
    uow: Arc<U>,
    sessions: Arc<dyn SessionStore>,
    challenges: Arc<dyn ChallengeCleanup>,
}

impl<U: UnitOfWork> HygieneService<U> {
    pub fn new(
        uow: Arc<U>,
        sessions: Arc<dyn SessionStore>,
        challenges: Arc<dyn ChallengeCleanup>,
    ) -> Self {
        Self {
            uow,
            sessions,
            challenges,
        }
    }

    /// 过期会话：expires_at/idle_expires_at 过界者批量撤销（理由 EXPIRED）。
    pub async fn expire_sessions(
        &self,
        ctx: &SecurityContext,
        now: DateTime<Utc>,
    ) -> Result<u64, AppError> {
        let sessions = self.sessions.clone();
        self.uow
            .transact(ctx, move |tx| {
                Box::pin(async move { sessions.expire_overdue(tx, now).await })
            })
            .await
    }

    /// 过期登录挑战：超期挑战清理（数量为实现体语义）。
    pub async fn expire_challenges(
        &self,
        ctx: &SecurityContext,
        now: DateTime<Utc>,
    ) -> Result<u64, AppError> {
        let challenges = self.challenges.clone();
        self.uow
            .transact(ctx, move |tx| {
                Box::pin(async move { challenges.expire_overdue(tx, now).await })
            })
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::context_build::pre_auth_context;
    use crate::testutil::{lock, mem, InMemoryUow, MemSessionStore};
    use crate::testutil_extra::MemChallengeCleanup;
    use crate::types::{AccountKind, AccountStatus, SessionRow, UserAccountRow};
    use chrono::Duration;
    use ep_foundation::id::marker::{LegalEntity, UserAccount};
    use ep_foundation::id::Id;

    fn ctx() -> SecurityContext {
        let account = UserAccountRow {
            id: Id::from_uuid(uuid::Uuid::from_u128(1)),
            account_kind: AccountKind::Employee,
            login_name: "hygiene".into(),
            employee_no: None,
            display_name: "H".into(),
            home_legal_entity_id: Id::<LegalEntity>::from_uuid(uuid::Uuid::from_u128(2)),
            status: AccountStatus::Active,
            clearance_level: 20,
            security_level: 30,
            is_mfa_required: false,
            created_at: Utc::now(),
        };
        pre_auth_context(&account, "DEV-01", "hygiene000", &"0".repeat(32)).expect("合法")
    }

    fn svc(h: &crate::testutil::MemHandle) -> HygieneService<InMemoryUow> {
        HygieneService::new(
            Arc::new(InMemoryUow),
            Arc::new(MemSessionStore(h.clone())),
            Arc::new(MemChallengeCleanup(h.clone())),
        )
    }

    #[tokio::test]
    async fn expired_sessions_are_revoked_in_bulk() {
        let h = mem();
        let s = svc(&h);
        let user = Id::<UserAccount>::from_uuid(uuid::Uuid::from_u128(1));
        let now = Utc::now();
        let le = Id::<LegalEntity>::from_uuid(uuid::Uuid::from_u128(2));
        // 一条绝对过期、一条空闲过期、一条存活。
        for (exp, idle) in [
            (now - Duration::seconds(1), now + Duration::hours(1)),
            (now + Duration::hours(1), now - Duration::seconds(1)),
            (now + Duration::hours(1), now + Duration::hours(1)),
        ] {
            lock(&h).sessions.push(SessionRow {
                id: uuid::Uuid::now_v7(),
                user_id: user,
                user_device_row_id: uuid::Uuid::nil(),
                token_hash: vec![0],
                active_legal_entity_id: le,
                issued_at: now,
                expires_at: exp,
                idle_expires_at: idle,
                last_seen_at: now,
                revoked_at: None,
                revoke_reason: None,
                is_breakglass: false,
            });
        }
        let n = s.expire_sessions(&ctx(), now).await.expect("清理");
        assert_eq!(n, 2, "绝对过期与空闲过期各撤一条");
        assert_eq!(
            lock(&h)
                .sessions
                .iter()
                .filter(|x| x.revoked_at.is_none())
                .count(),
            1
        );
    }

    #[tokio::test]
    async fn expired_challenges_call_is_delegated() {
        let h = mem();
        let s = svc(&h);
        let _ = s.expire_challenges(&ctx(), Utc::now()).await.expect("清理");
        assert_eq!(lock(&h).challenges_expired_calls, 1, "清理调用落到实现体");
    }
}
