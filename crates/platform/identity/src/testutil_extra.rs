//! 测试基座（续）：应急账号、授权读取面与挑战清理的内存实现。

#![cfg(test)]

use chrono::{DateTime, Utc};
use ep_foundation::error::AppError;
use ep_foundation::id::marker::LegalEntity;
use ep_foundation::id::Id;
use ep_foundation::port::tx::Tx;

use crate::ports::{
    BreakglassStore, ChallengeCleanup, NewBreakglass, UserAuthzQuery, UserAuthzSet,
};
use crate::testutil::{lock, MemHandle};
use crate::types::{BreakglassRow, BreakglassStatus};

pub struct MemBreakglassStore(pub MemHandle);

#[async_trait::async_trait]
impl BreakglassStore for MemBreakglassStore {
    async fn insert(&self, _tx: &mut dyn Tx, new: NewBreakglass) -> Result<uuid::Uuid, AppError> {
        let id = uuid::Uuid::now_v7();
        lock(&self.0).breakglass.push(BreakglassRow {
            id,
            doc_no: new.doc_no,
            status: BreakglassStatus::Draft,
            user_id: new.user_id,
            requested_by: new.requested_by,
            approved_by: None,
            reason: new.reason,
            approval_ref: None,
            allowed_action_set: new.allowed_action_set,
            activated_at: None,
            expires_at: None,
            closed_at: None,
            rotated_at: None,
            rotation_result: None,
        });
        Ok(id)
    }
    async fn get(
        &self,
        _tx: &mut dyn Tx,
        id: uuid::Uuid,
    ) -> Result<Option<BreakglassRow>, AppError> {
        Ok(lock(&self.0)
            .breakglass
            .iter()
            .find(|b| b.id == id)
            .cloned())
    }
    async fn transition(
        &self,
        _tx: &mut dyn Tx,
        id: uuid::Uuid,
        from: BreakglassStatus,
        to: BreakglassStatus,
    ) -> Result<bool, AppError> {
        let mut st = lock(&self.0);
        let Some(b) = st.breakglass.iter_mut().find(|b| b.id == id) else {
            return Ok(false);
        };
        if b.status != from {
            return Ok(false);
        }
        b.status = to;
        Ok(true)
    }
    async fn approve(
        &self,
        _tx: &mut dyn Tx,
        id: uuid::Uuid,
        approved_by: Id<ep_foundation::id::marker::UserAccount>,
        approval_ref: &str,
    ) -> Result<bool, AppError> {
        let mut st = lock(&self.0);
        let Some(b) = st.breakglass.iter_mut().find(|b| b.id == id) else {
            return Ok(false);
        };
        if b.status != BreakglassStatus::PendingApproval {
            return Ok(false);
        }
        b.status = BreakglassStatus::Approved;
        b.approved_by = Some(approved_by);
        b.approval_ref = Some(approval_ref.to_string());
        Ok(true)
    }
    async fn activate(
        &self,
        _tx: &mut dyn Tx,
        id: uuid::Uuid,
        activated_at: DateTime<Utc>,
        expires_at: DateTime<Utc>,
    ) -> Result<bool, AppError> {
        let mut st = lock(&self.0);
        let Some(b) = st.breakglass.iter_mut().find(|b| b.id == id) else {
            return Ok(false);
        };
        if b.status != BreakglassStatus::Approved {
            return Ok(false);
        }
        b.status = BreakglassStatus::Active;
        b.activated_at = Some(activated_at);
        b.expires_at = Some(expires_at);
        Ok(true)
    }
    async fn close(
        &self,
        _tx: &mut dyn Tx,
        id: uuid::Uuid,
        now: DateTime<Utc>,
    ) -> Result<bool, AppError> {
        let mut st = lock(&self.0);
        let Some(b) = st.breakglass.iter_mut().find(|b| b.id == id) else {
            return Ok(false);
        };
        if b.status != BreakglassStatus::Active {
            return Ok(false);
        }
        b.status = BreakglassStatus::Closed;
        b.closed_at = Some(now);
        Ok(true)
    }
    async fn finalize_with_rotation(
        &self,
        _tx: &mut dyn Tx,
        id: uuid::Uuid,
        to: BreakglassStatus,
        rotation_result: &str,
        now: DateTime<Utc>,
    ) -> Result<bool, AppError> {
        let mut st = lock(&self.0);
        let Some(b) = st.breakglass.iter_mut().find(|b| b.id == id) else {
            return Ok(false);
        };
        if b.status != BreakglassStatus::Active {
            return Ok(false);
        }
        b.status = to;
        b.rotated_at = Some(now);
        b.rotation_result = Some(rotation_result.to_string());
        Ok(true)
    }
    async fn list_due_active(
        &self,
        _tx: &mut dyn Tx,
        now: DateTime<Utc>,
    ) -> Result<Vec<BreakglassRow>, AppError> {
        Ok(lock(&self.0)
            .breakglass
            .iter()
            .filter(|b| {
                b.status == BreakglassStatus::Active && b.expires_at.is_some_and(|e| e <= now)
            })
            .cloned()
            .collect())
    }
    async fn list_idle_for_rotation(
        &self,
        _tx: &mut dyn Tx,
        cutoff: DateTime<Utc>,
    ) -> Result<Vec<BreakglassRow>, AppError> {
        Ok(lock(&self.0)
            .breakglass
            .iter()
            .filter(|b| {
                matches!(
                    b.status,
                    BreakglassStatus::Closed | BreakglassStatus::Expired
                ) && b.rotated_at.is_none_or(|r| r < cutoff)
            })
            .cloned()
            .collect())
    }
    async fn mark_rotated(
        &self,
        _tx: &mut dyn Tx,
        id: uuid::Uuid,
        rotation_result: &str,
        now: DateTime<Utc>,
    ) -> Result<bool, AppError> {
        let mut st = lock(&self.0);
        let Some(b) = st.breakglass.iter_mut().find(|b| b.id == id) else {
            return Ok(false);
        };
        if !matches!(
            b.status,
            BreakglassStatus::Closed | BreakglassStatus::Expired
        ) {
            return Ok(false);
        }
        b.rotated_at = Some(now);
        b.rotation_result = Some(rotation_result.to_string());
        Ok(true)
    }
}

pub struct MemUserAuthzQuery(pub MemHandle);

#[async_trait::async_trait]
impl UserAuthzQuery for MemUserAuthzQuery {
    async fn load_user_authz(
        &self,
        _tx: &mut dyn Tx,
        _user_id: Id<ep_foundation::id::marker::UserAccount>,
        _home_legal_entity_id: Id<ep_foundation::id::marker::LegalEntity>,
    ) -> Result<UserAuthzSet, AppError> {
        let st = lock(&self.0);
        Ok(UserAuthzSet {
            duty_classes: st.duties.clone(),
            has_high_risk_permission: st.high_risk,
            legal_entity_ids: st.legal_entities.clone(),
            ..UserAuthzSet::default()
        })
    }
    async fn user_duty_classes(
        &self,
        _tx: &mut dyn Tx,
        _user_id: Id<ep_foundation::id::marker::UserAccount>,
    ) -> Result<Vec<ep_foundation::security::context::DutyClass>, AppError> {
        Ok(lock(&self.0).duties.clone())
    }
    async fn count_open_high_risk_requests(
        &self,
        _tx: &mut dyn Tx,
        _user_id: Id<ep_foundation::id::marker::UserAccount>,
    ) -> Result<u64, AppError> {
        Ok(lock(&self.0).open_high_risk_requests)
    }
    async fn installed_legal_entities(
        &self,
        _tx: &mut dyn Tx,
    ) -> Result<Vec<Id<LegalEntity>>, AppError> {
        Ok(lock(&self.0).installed_les.clone())
    }
    async fn probe_legal_entity_grant(
        &self,
        _tx: &mut dyn Tx,
        _user_id: Id<ep_foundation::id::marker::UserAccount>,
        legal_entity_id: Id<LegalEntity>,
    ) -> Result<bool, AppError> {
        Ok(lock(&self.0).granted_les.contains(&legal_entity_id))
    }
}

pub struct MemChallengeCleanup(pub MemHandle);

#[async_trait::async_trait]
impl ChallengeCleanup for MemChallengeCleanup {
    async fn expire_overdue(&self, _tx: &mut dyn Tx, _now: DateTime<Utc>) -> Result<u64, AppError> {
        lock(&self.0).challenges_expired_calls += 1;
        Ok(0)
    }
}
