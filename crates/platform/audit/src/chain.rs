//! 按法人与自然日分段的哈希链。
//!
//! 链的形状取自技术基线第 9.4 节与阶段 3 计划第 3.4.2 节：
//! 段内第 n 条的 `prev_hash` 是第 n−1 条的 `hash`，段首条取 32 字节全零；
//! `hash = SHA-256(JCS(除 hash 外的全部列))`。
//!
//! 本模块只做纯计算，不碰数据库：段行的取锁、`seq` 的分配、批量插入
//! 都在调用方的业务事务内完成（计划第 3.4.2 节七步中的第 3、5、6 步）。
//! 这样分是为了让链的算法可以脱库单测——它是整条证据链的地基，
//! 一处算错要到很久以后验证时才显形，那时已经查不回来。

use sha2::{Digest, Sha256};

use crate::jcs::{canonicalize, JcsError};

/// 摘要长度。SHA-256，取自规格第 12.3 章。
pub const HASH_LEN: usize = 32;

/// 段首条的 `prev_hash`：32 字节全零。
pub const GENESIS_PREV_HASH: [u8; HASH_LEN] = [0u8; HASH_LEN];

#[derive(Debug, PartialEq, Eq)]
pub enum ChainError {
    /// 规范化失败，原因见内层。
    Canonicalize(JcsError),
    /// 链断了：第 `index` 条的 `prev_hash` 不等于上一条的 `hash`。
    /// `index` 是段内序号，从 0 起。
    BrokenLink {
        index: usize,
        expected: String,
        found: String,
    },
    /// 某条事件的 `hash` 与按其内容重算的结果不符——内容被改过。
    HashMismatch {
        index: usize,
        expected: String,
        found: String,
    },
}

impl std::fmt::Display for ChainError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ChainError::Canonicalize(e) => write!(f, "规范化失败：{e}"),
            ChainError::BrokenLink {
                index,
                expected,
                found,
            } => write!(
                f,
                "段内第 {index} 条的 prev_hash 断链：应为 {expected}，实为 {found}"
            ),
            ChainError::HashMismatch {
                index,
                expected,
                found,
            } => write!(
                f,
                "段内第 {index} 条的 hash 与其内容不符：按内容应为 {expected}，记录为 {found}"
            ),
        }
    }
}

impl std::error::Error for ChainError {}

impl From<JcsError> for ChainError {
    fn from(e: JcsError) -> Self {
        ChainError::Canonicalize(e)
    }
}

/// 一环。`prev_hash` 与 `hash` 都是 32 字节摘要。
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct ChainLink {
    pub prev_hash: [u8; HASH_LEN],
    pub hash: [u8; HASH_LEN],
}

/// 小写十六进制。`prev_hash` 进哈希输入时按此形态承载（计划第 3.4.2 节）。
pub fn to_hex(bytes: &[u8]) -> String {
    let mut s = String::with_capacity(bytes.len() * 2);
    for b in bytes {
        s.push_str(&format!("{b:02x}"));
    }
    s
}

/// 由「除 hash 外的全部列」算出本条的 hash。
///
/// 入参是已经把 `prev_hash` 以小写十六进制字符串填好的那个 JSON 对象——
/// 本函数不替调用方拼字段，因为列集是技术基线第 9.4 节冻结的 19 列，
/// 由谁来拼这件事必须留在能看到那张表的地方，不能藏在算法里。
pub fn hash_of(canonical_input: &serde_json::Value) -> Result<[u8; HASH_LEN], ChainError> {
    let bytes = canonicalize(canonical_input)?;
    let mut h = Sha256::new();
    h.update(&bytes);
    Ok(h.finalize().into())
}

/// 段根哈希的输入摘要（计划第 3.4.3 节阶段 B）：
/// `SHA-256(JCS({segment_id, legal_entity_id, event_day, anchor_seq, root_hash, event_count}))`。
///
/// 单列一个函数而不是复用 [`hash_of`]，是因为两者的输入含义不同：
/// 一个是事件本身，一个是段的锚定记录。混用会让「签的是哪一份东西」变得含糊，
/// 而签名的对象含糊正是这类证据体系最常见的漏洞。
pub fn anchor_digest(anchor_record: &serde_json::Value) -> Result<[u8; HASH_LEN], ChainError> {
    hash_of(anchor_record)
}

/// 校验一段内的链是否首尾相连、且每条的 hash 与其内容相符。
///
/// 入参每一项是 `(该条除 hash 外的全部列, 记录在库里的 prev_hash, 记录在库里的 hash)`。
/// `expected_genesis` 给段首条应有的 `prev_hash`——段首取全零，
/// 若是从段中某一点开始验证则给该点之前那条的 hash。
///
/// **两类错分开报**：断链说明有人删了或插了一条，哈希不符说明有人改了一条内容。
/// 合成一种错会让排查方向从一开始就偏。
pub fn verify_segment(
    events: &[(serde_json::Value, [u8; HASH_LEN], [u8; HASH_LEN])],
    expected_genesis: [u8; HASH_LEN],
) -> Result<(), ChainError> {
    let mut expected_prev = expected_genesis;
    for (index, (input, prev_hash, hash)) in events.iter().enumerate() {
        if *prev_hash != expected_prev {
            return Err(ChainError::BrokenLink {
                index,
                expected: to_hex(&expected_prev),
                found: to_hex(prev_hash),
            });
        }
        let recomputed = hash_of(input)?;
        if recomputed != *hash {
            return Err(ChainError::HashMismatch {
                index,
                expected: to_hex(&recomputed),
                found: to_hex(hash),
            });
        }
        expected_prev = *hash;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    /// 造一条事件的哈希输入。`prev_hash` 按纪律以小写十六进制承载。
    fn input(action: &str, prev: &[u8; HASH_LEN]) -> serde_json::Value {
        json!({
            "action": action,
            "object_type": "contract",
            "object_version": 1,
            "prev_hash": to_hex(prev),
        })
    }

    /// 建一条 n 环的链，返回可直接喂给 verify_segment 的三元组。
    fn build(n: usize) -> Vec<(serde_json::Value, [u8; HASH_LEN], [u8; HASH_LEN])> {
        let mut prev = GENESIS_PREV_HASH;
        let mut out = Vec::new();
        for i in 0..n {
            let v = input(&format!("act.{i}"), &prev);
            let h = hash_of(&v).expect("应可算");
            out.push((v, prev, h));
            prev = h;
        }
        out
    }

    #[test]
    fn a_well_formed_chain_verifies() {
        let chain = build(5);
        assert!(verify_segment(&chain, GENESIS_PREV_HASH).is_ok());
    }

    #[test]
    fn same_input_gives_same_hash() {
        let a = hash_of(&input("x", &GENESIS_PREV_HASH)).expect("应可算");
        let b = hash_of(&input("x", &GENESIS_PREV_HASH)).expect("应可算");
        assert_eq!(a, b, "同一份输入必须产出同一摘要");
    }

    /// 负样例：改内容。哈希不符，且报的是 HashMismatch 不是 BrokenLink——
    /// 两类错的排查方向不同，报错必须把方向指对。
    #[test]
    fn tampering_with_content_is_caught_as_hash_mismatch() {
        let mut chain = build(3);
        chain[1].0 = input("篡改后的动作", &chain[1].1);
        let err = verify_segment(&chain, GENESIS_PREV_HASH).expect_err("必须查出");
        match err {
            ChainError::HashMismatch { index, .. } => assert_eq!(index, 1),
            other => panic!("应报内容被改，实报 {other}"),
        }
    }

    /// 负样例：删一条。链断在被删那条的后一条上。
    #[test]
    fn deleting_an_event_breaks_the_link() {
        let mut chain = build(4);
        chain.remove(1);
        let err = verify_segment(&chain, GENESIS_PREV_HASH).expect_err("必须查出");
        match err {
            ChainError::BrokenLink { index, .. } => assert_eq!(index, 1),
            other => panic!("应报断链，实报 {other}"),
        }
    }

    /// 负样例：段首条的 prev_hash 不是全零。
    #[test]
    fn first_event_must_start_from_genesis() {
        let chain = build(2);
        let wrong = [1u8; HASH_LEN];
        let err = verify_segment(&chain, wrong).expect_err("必须查出");
        assert!(matches!(err, ChainError::BrokenLink { index: 0, .. }));
    }

    /// 空段合法：一个法人在某一天没有任何审计事件是正常的，
    /// 不该被当成链损坏——把「没有」判成「坏了」会让运维每天追查不存在的问题。
    #[test]
    fn empty_segment_is_valid() {
        assert!(verify_segment(&[], GENESIS_PREV_HASH).is_ok());
    }

    /// 规范化失败要如实穿透，不得吞掉后当成「链没问题」。
    #[test]
    fn canonicalize_failure_propagates() {
        let bad = json!({"amount": 1234.56, "prev_hash": to_hex(&GENESIS_PREV_HASH)});
        assert!(matches!(hash_of(&bad), Err(ChainError::Canonicalize(_))));
    }

    #[test]
    fn hex_is_lowercase_and_fixed_width() {
        assert_eq!(to_hex(&[0u8, 15, 16, 255]), "000f10ff");
        assert_eq!(to_hex(&GENESIS_PREV_HASH).len(), HASH_LEN * 2);
    }
}
