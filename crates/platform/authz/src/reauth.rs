//! ReauthGate：高风险操作的二次认证挑战签发与核销。
//!
//! subject_digest 由服务端按五字段重算——操作类型、法人、单据号、
//! 关键金额或期间、一句话影响——固定键序紧凑 JSON（serde 声明序即键序）、
//! Decimal 定长串、SHA-256。核销走单次消费条件更新语义：
//! status VERIFIED→CONSUMED，以影响行数判定；摘要不符与重复消费同归
//! 条件更新 0 行，映射 PLATFORM.REAUTH.TOKEN_ALREADY_CONSUMED。
//! 移动端四类受限操作在发起挑战处即拒 PLATFORM.HIGH_RISK_REQUEST.CLIENT_NOT_ALLOWED。

use std::sync::Arc;

use ep_foundation::error::codes::{
    PLATFORM_HIGH_RISK_REQUEST_CLIENT_NOT_ALLOWED, PLATFORM_REAUTH_TOKEN_ALREADY_CONSUMED,
};
use ep_foundation::error::AppError;
use ep_foundation::id::marker::{LegalEntity, Session, UserAccount};
use ep_foundation::id::Id;
use ep_foundation::security::SecurityContext;
use sha2::{Digest, Sha256};

use crate::metrics::AuthzMetricsSink;
use crate::types::{hex_encode, HighRiskOperation};

/// 挑战主题五字段。serde 声明序即紧凑 JSON 的键序，不得调换。
#[derive(Clone, PartialEq, Eq, Debug, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ReauthSubject {
    pub operation_type: HighRiskOperation,
    /// 以 UUID 承载，参与固定键序 JSON 摘要。
    pub legal_entity_id: uuid::Uuid,
    pub doc_no: String,
    /// 关键金额或期间：一律以 [`canonical_amount`] 等定长形态传入。
    pub key_amount_or_period: String,
    pub impact_statement: String,
}

/// Decimal 定长串：符号 1 + 整数 16 + 小数点 + 小数 2 = 固定 20 字符。
/// 金额以「分」为单位的整数传入，杜绝浮点漂移。
pub fn canonical_amount(cents: i128) -> String {
    let sign = if cents < 0 { '-' } else { '+' };
    let abs = cents.unsigned_abs();
    format!("{sign}{:016}.{:02}", abs / 100, abs % 100)
}

/// 服务端重算摘要：固定键序紧凑 JSON 的 SHA-256 十六进制。
pub fn subject_digest(subject: &ReauthSubject) -> Result<String, AppError> {
    let json = serde_json::to_string(subject).map_err(|e| {
        AppError::new(
            ep_foundation::error::codes::PLATFORM_SYSTEM_INTERNAL_ERROR,
            format!("挑战主题序列化失败：{e}"),
        )
    })?;
    let mut hasher = Sha256::new();
    hasher.update(json.as_bytes());
    Ok(hex_encode(&hasher.finalize()))
}

/// 挑战状态机：ISSUED →（MFA 通过）VERIFIED →（核销）CONSUMED。
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum ChallengeStatus {
    Issued,
    Verified,
    Consumed,
}

/// 一条挑战的落库形态。
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ChallengeRecord {
    pub challenge_id: uuid::Uuid,
    pub user_id: Id<UserAccount>,
    pub session_id: Id<Session>,
    pub legal_entity_id: Id<LegalEntity>,
    pub operation_type: HighRiskOperation,
    pub subject_digest: String,
    /// 令牌哈希（SHA-256 hex），明文令牌只在签发时返回一次。
    pub token_hash: String,
    pub status: ChallengeStatus,
}

/// 挑战持久化端口。SQL 载体归 ep-adapter-db-pg（platform_authz.reauth_challenges）。
#[async_trait::async_trait]
pub trait ReauthChallengeStore: Send + Sync {
    /// 签发落库（status=ISSUED）。
    async fn insert(&self, record: &ChallengeRecord) -> Result<(), AppError>;
    /// 单次消费条件更新：仅当 status=VERIFIED 且 token_hash 与
    /// subject_digest 同时匹配时置 CONSUMED，返回影响行数。
    async fn verify_and_consume(
        &self,
        challenge_id: uuid::Uuid,
        token_hash: &str,
        subject_digest: &str,
    ) -> Result<u64, AppError>;
}

/// 签发产出。`token` 明文只在此返回一次。
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct IssuedChallenge {
    pub challenge_id: uuid::Uuid,
    pub token: String,
    pub subject_digest: String,
}

/// 二次认证闸门。
pub struct ReauthGate {
    store: Arc<dyn ReauthChallengeStore>,
    metrics: Arc<dyn AuthzMetricsSink>,
}

impl ReauthGate {
    pub fn new(store: Arc<dyn ReauthChallengeStore>, metrics: Arc<dyn AuthzMetricsSink>) -> Self {
        Self { store, metrics }
    }

    /// 发起挑战：移动端四类受限操作即拒；其余签发并返回一次性令牌。
    pub async fn issue(
        &self,
        ctx: &SecurityContext,
        subject: &ReauthSubject,
    ) -> Result<IssuedChallenge, AppError> {
        if subject.operation_type.is_mobile_restricted(ctx.client) {
            return Err(AppError::new(
                PLATFORM_HIGH_RISK_REQUEST_CLIENT_NOT_ALLOWED,
                format!(
                    "移动端禁止发起 {} 类高风险操作",
                    subject.operation_type.as_str()
                ),
            ));
        }
        let digest = subject_digest(subject)?;
        let token_bytes = random_bytes();
        let token = hex_encode(&token_bytes);
        let record = ChallengeRecord {
            challenge_id: uuid::Uuid::now_v7(),
            user_id: ctx.user_id,
            session_id: ctx.session_id,
            legal_entity_id: ctx.legal_entity_id,
            operation_type: subject.operation_type,
            subject_digest: digest.clone(),
            token_hash: sha256_hex(token_bytes.as_slice()),
            status: ChallengeStatus::Issued,
        };
        self.store.insert(&record).await?;
        self.metrics.count_reauth_challenge(
            &ctx.legal_entity_id.to_string(),
            subject.operation_type.as_str(),
        );
        Ok(IssuedChallenge {
            challenge_id: record.challenge_id,
            token,
            subject_digest: digest,
        })
    }

    /// 核销：服务端重算摘要并做单次消费条件更新；
    /// 影响行数为 0（重复消费、摘要不符或未过 MFA）一律按已消费拒。
    pub async fn consume(
        &self,
        ctx: &SecurityContext,
        challenge_id: uuid::Uuid,
        token: &str,
        subject: &ReauthSubject,
    ) -> Result<(), AppError> {
        if subject.operation_type.is_mobile_restricted(ctx.client) {
            return Err(AppError::new(
                PLATFORM_HIGH_RISK_REQUEST_CLIENT_NOT_ALLOWED,
                format!(
                    "移动端禁止核销 {} 类高风险操作",
                    subject.operation_type.as_str()
                ),
            ));
        }
        let digest = subject_digest(subject)?;
        let token_hash = sha256_hex(&hex_decode_or_empty(token));
        let rows = self
            .store
            .verify_and_consume(challenge_id, &token_hash, &digest)
            .await?;
        if rows == 0 {
            return Err(AppError::new(
                PLATFORM_REAUTH_TOKEN_ALREADY_CONSUMED,
                "挑战令牌已消费、未过验证或主题摘要不符",
            ));
        }
        Ok(())
    }
}

fn sha256_hex(bytes: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(bytes);
    hex_encode(&hasher.finalize())
}

fn hex_decode_or_empty(token: &str) -> Vec<u8> {
    crate::types::hex_decode(token).unwrap_or_default()
}

/// 32 字节随机令牌材料。
fn random_bytes() -> Vec<u8> {
    use rand::RngCore;
    let mut bytes = [0u8; 32];
    rand::thread_rng().fill_bytes(&mut bytes);
    bytes.to_vec()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::metrics::SilentMetricsSink;
    use crate::types::tests::ctx_with;
    use ep_foundation::security::context::ClientKind;
    use std::sync::Mutex;

    /// 内存挑战仓：以状态字段模拟条件更新语义。
    #[derive(Default)]
    struct FixtureStore {
        records: Mutex<Vec<ChallengeRecord>>,
    }

    #[async_trait::async_trait]
    impl ReauthChallengeStore for FixtureStore {
        async fn insert(&self, record: &ChallengeRecord) -> Result<(), AppError> {
            self.records
                .lock()
                .unwrap_or_else(|p| p.into_inner())
                .push(record.clone());
            Ok(())
        }
        async fn verify_and_consume(
            &self,
            challenge_id: uuid::Uuid,
            token_hash: &str,
            subject_digest: &str,
        ) -> Result<u64, AppError> {
            let mut records = self.records.lock().unwrap_or_else(|p| p.into_inner());
            for r in records.iter_mut() {
                if r.challenge_id == challenge_id
                    && r.status == ChallengeStatus::Verified
                    && r.token_hash == token_hash
                    && r.subject_digest == subject_digest
                {
                    r.status = ChallengeStatus::Consumed;
                    return Ok(1);
                }
            }
            Ok(0)
        }
    }

    fn subject(op: HighRiskOperation) -> ReauthSubject {
        ReauthSubject {
            operation_type: op,
            legal_entity_id: uuid::Uuid::from_u128(3),
            doc_no: "PAY-2026-0001".into(),
            key_amount_or_period: canonical_amount(123_456),
            impact_statement: "对外付款一百万元".into(),
        }
    }

    fn gate(store: Arc<FixtureStore>) -> ReauthGate {
        ReauthGate::new(store, Arc::new(SilentMetricsSink))
    }

    fn mark_verified(store: &FixtureStore, id: uuid::Uuid) {
        let mut records = store.records.lock().unwrap_or_else(|p| p.into_inner());
        if let Some(r) = records.iter_mut().find(|r| r.challenge_id == id) {
            r.status = ChallengeStatus::Verified;
        }
    }

    #[test]
    fn digest_is_fixed_key_order_and_stable() {
        let s = subject(HighRiskOperation::Payment);
        let json = serde_json::to_string(&s).expect("可序列化");
        assert_eq!(
            json,
            concat!(
                r#"{"operation_type":"PAYMENT","legal_entity_id":"#,
                r#""00000000-0000-0000-0000-000000000003","doc_no":"PAY-2026-0001","#,
                r#""key_amount_or_period":"+0000000000001234.56","#,
                r#""impact_statement":"对外付款一百万元"}"#
            ),
            "键序固定为操作类型/法人/单据号/金额期间/影响"
        );
        assert_eq!(
            subject_digest(&s).expect("可摘要"),
            subject_digest(&s).expect("可摘要")
        );
        assert_eq!(canonical_amount(123_456), "+0000000000001234.56");
        assert_eq!(canonical_amount(-5), "-0000000000000000.05");
    }

    #[tokio::test]
    async fn mobile_clients_are_rejected_at_issue() {
        let store = Arc::new(FixtureStore::default());
        let g = gate(store.clone());
        let ctx = ctx_with(vec!["FINANCE"], ClientKind::Ios);
        let err = g
            .issue(&ctx, &subject(HighRiskOperation::Payment))
            .await
            .expect_err("移动端即拒");
        assert_eq!(err.code, PLATFORM_HIGH_RISK_REQUEST_CLIENT_NOT_ALLOWED);
        assert!(store
            .records
            .lock()
            .unwrap_or_else(|p| p.into_inner())
            .is_empty());
        // 非受限操作可签发。
        let ok = g
            .issue(&ctx, &subject(HighRiskOperation::SensitiveExport))
            .await
            .expect("非受限可签发");
        assert_eq!(ok.token.len(), 64, "32 字节 hex");
    }

    #[tokio::test]
    async fn consume_is_single_use_with_conditional_update() {
        let store = Arc::new(FixtureStore::default());
        let g = gate(store.clone());
        let ctx = ctx_with(vec!["FINANCE"], ClientKind::Win);
        let s = subject(HighRiskOperation::Payment);
        let issued = g.issue(&ctx, &s).await.expect("可签发");
        mark_verified(&store, issued.challenge_id);
        g.consume(&ctx, issued.challenge_id, &issued.token, &s)
            .await
            .expect("首次核销成功");
        let err = g
            .consume(&ctx, issued.challenge_id, &issued.token, &s)
            .await
            .expect_err("二次核销拒");
        assert_eq!(err.code, PLATFORM_REAUTH_TOKEN_ALREADY_CONSUMED);
        // 未过 MFA（状态仍 ISSUED）时条件更新同样 0 行。
        let issued2 = g.issue(&ctx, &s).await.expect("可签发");
        let err = g
            .consume(&ctx, issued2.challenge_id, &issued2.token, &s)
            .await
            .expect_err("未验证拒");
        assert_eq!(err.code, PLATFORM_REAUTH_TOKEN_ALREADY_CONSUMED);
    }

    #[tokio::test]
    async fn digest_recompute_catches_subject_tampering() {
        let store = Arc::new(FixtureStore::default());
        let g = gate(store.clone());
        let ctx = ctx_with(vec!["FINANCE"], ClientKind::Win);
        let s = subject(HighRiskOperation::LedgerPosting);
        let issued = g.issue(&ctx, &s).await.expect("可签发");
        mark_verified(&store, issued.challenge_id);
        let mut tampered = s.clone();
        tampered.key_amount_or_period = canonical_amount(999_999_999);
        let err = g
            .consume(&ctx, issued.challenge_id, &issued.token, &tampered)
            .await
            .expect_err("摘要不符拒");
        assert_eq!(err.code, PLATFORM_REAUTH_TOKEN_ALREADY_CONSUMED);
    }
}
