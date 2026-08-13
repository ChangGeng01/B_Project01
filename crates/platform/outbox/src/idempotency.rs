//! 幂等键三态判定的纯逻辑（阶段 3a，03 计划 §3.4.4 行 922）。
//!
//! 判定与存储介质无关：SQL 实现体（ep-adapter-db-pg 的
//! `platform_msg/idempotency.rs`）只负责事务内插入与读行，
//! 三态结论一律经本模块的 [`judge`] 算出，全仓不得另立第二套判等
//! （裁定 C-07 职责三段：请求头校验归阶段 1、端口定义归阶段 2、
//! 建表与重放实现归阶段 3a）。
//!
//! 三态口径：
//! - 键位不存在 → `FirstCall`，本次请求取得执行权；
//! - 已有行 `IN_PROGRESS` → 不占用 Outcome 变体，以 Err 返回
//!   `PLATFORM.IDEMPOTENCY.IN_PROGRESS`（并发在途去重）；
//! - 已有行 `COMPLETED` 且请求哈希相同 → `Replay` 回放定稿响应；
//!   哈希相异 → `PayloadMismatch`（调用方映射阶段 1 已登记的
//!   `PLATFORM.IDEMPOTENCY.PAYLOAD_MISMATCH`，本批不重复登记）。

use ep_foundation::error::codes::{
    PLATFORM_IDEMPOTENCY_IN_PROGRESS, PLATFORM_SYSTEM_INTERNAL_ERROR,
};
use ep_foundation::error::AppError;
use ep_foundation::port::db::IdempotencyOutcome;

/// `platform_msg.idempotency_keys.state` 列的两个取值。
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum KeyState {
    InProgress,
    Completed,
}

/// 事务内读到的已有键行快照。列投影与表 12 一致。
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ExistingKeyRow {
    /// 键的落库状态。
    pub state: KeyState,
    /// 首次登记时的请求哈希，64 位小写十六进制。
    pub request_hash_hex: String,
    /// 定稿响应。`COMPLETED` 行正常必有；缺席视为数据损坏。
    pub response: Option<(u16, Vec<u8>)>,
}

/// 把 32 字节请求哈希编码成 64 位小写十六进制。
/// 这是 `request_hash` 列的唯一落库形态（表 12 规格），
/// 判等以字符串逐位比对，不做大小写折叠。
pub fn hash_hex(hash: &[u8; 32]) -> String {
    let mut out = String::with_capacity(64);
    for byte in hash {
        out.push_str(&format!("{byte:02x}"));
    }
    out
}

/// 三态判定。`existing` 为 None 表示 `INSERT ... ON CONFLICT DO NOTHING`
/// 之后键位仍不在场（首次调用）；否则按已有行的 state 与哈希关系判。
///
/// `IN_PROGRESS` 判定先于哈希比对：在途行的载荷比较没有意义，
/// 且端口的三变体不为在途留位（新增决定六）。
pub fn judge(
    existing: Option<ExistingKeyRow>,
    request_hash: [u8; 32],
) -> Result<IdempotencyOutcome, AppError> {
    let Some(row) = existing else {
        return Ok(IdempotencyOutcome::FirstCall);
    };
    if row.state == KeyState::InProgress {
        return Err(AppError::new(
            PLATFORM_IDEMPOTENCY_IN_PROGRESS,
            "同一幂等键上已有请求在途",
        ));
    }
    if row.request_hash_hex != hash_hex(&request_hash) {
        return Ok(IdempotencyOutcome::PayloadMismatch);
    }
    match row.response {
        Some((response_status, response_body)) => Ok(IdempotencyOutcome::Replay {
            response_status,
            response_body,
        }),
        None => Err(AppError::new(
            PLATFORM_SYSTEM_INTERNAL_ERROR,
            "幂等键已定稿但缺少响应记录",
        )),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const HASH_A: [u8; 32] = [0x1a; 32];
    const HASH_B: [u8; 32] = [0x2b; 32];
    const HASH_C: [u8; 32] = [0x00; 32];

    fn completed(hash: [u8; 32], response: Option<(u16, Vec<u8>)>) -> ExistingKeyRow {
        ExistingKeyRow {
            state: KeyState::Completed,
            request_hash_hex: hash_hex(&hash),
            response,
        }
    }

    fn in_progress(hash: [u8; 32]) -> ExistingKeyRow {
        ExistingKeyRow {
            state: KeyState::InProgress,
            request_hash_hex: hash_hex(&hash),
            response: None,
        }
    }

    /// 九种组合（3 state × 3 哈希关系）逐格断言，退出条件 6 的纯逻辑半。
    /// state 三态：键位不存在、IN_PROGRESS、COMPLETED；
    /// 哈希关系三态：同哈希、异哈希、定稿响应缺席（COMPLETED 行）
    /// 或任意哈希（前两态的哈希关系不影响结论）。
    #[test]
    fn nine_combinations_of_state_and_hash_relation() {
        // 一、键位不存在：任意哈希一律 FirstCall。
        for hash in [HASH_A, HASH_B, HASH_C] {
            assert_eq!(
                judge(None, hash).expect("不返错"),
                IdempotencyOutcome::FirstCall
            );
        }
        // 二、IN_PROGRESS：任意哈希一律以冲突错误返回，不占 Outcome 变体。
        for hash in [HASH_A, HASH_B, HASH_C] {
            let row = in_progress(HASH_A);
            let err = judge(Some(row), hash).expect_err("在途必须返错");
            assert_eq!(err.code, PLATFORM_IDEMPOTENCY_IN_PROGRESS);
        }
        // 三、COMPLETED 同哈希：回放定稿响应。
        let replay = judge(
            Some(completed(HASH_A, Some((201, b"created".to_vec())))),
            HASH_A,
        )
        .expect("不返错");
        assert_eq!(
            replay,
            IdempotencyOutcome::Replay {
                response_status: 201,
                response_body: b"created".to_vec()
            }
        );
        // 三、COMPLETED 异哈希：载荷不符。
        let mismatch =
            judge(Some(completed(HASH_A, Some((200, Vec::new())))), HASH_B).expect("不返错");
        assert_eq!(mismatch, IdempotencyOutcome::PayloadMismatch);
        // 三、COMPLETED 同哈希但响应缺席：数据损坏，内部错误。
        let err = judge(Some(completed(HASH_A, None)), HASH_A).expect_err("数据损坏必须返错");
        assert_eq!(err.code, PLATFORM_SYSTEM_INTERNAL_ERROR);
    }

    /// 哈希编码形态：64 位小写十六进制，与表 12 的 check 约束同口径。
    #[test]
    fn hash_hex_is_64_lowercase_hex() {
        let hex = hash_hex(&HASH_A);
        assert_eq!(hex.len(), 64);
        assert!(hex.chars().all(|c| c.is_ascii_hexdigit()));
        assert_eq!(hex, "1a".repeat(32));
        assert_eq!(hash_hex(&HASH_C), "0".repeat(64));
    }

    /// 判等按字符串逐位比对：大小写或长度不同即异哈希。
    #[test]
    fn hash_comparison_is_exact_string_match() {
        let mut row = completed(HASH_A, Some((200, Vec::new())));
        row.request_hash_hex = row.request_hash_hex.to_uppercase();
        assert_eq!(
            judge(Some(row), HASH_A).expect("不返错"),
            IdempotencyOutcome::PayloadMismatch,
            "大写形态不是落库形态，必须判异"
        );
    }
}
