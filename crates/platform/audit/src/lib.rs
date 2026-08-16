//! ep-platform-audit —— 审计事件的哈希链、分段与链验证。
//!
//! 审计事件是法律与合规证据（技术基线第 9.4 节）：与业务变更在**同一数据库事务**
//! 写入，进入按法人与自然日的哈希链，每 5 分钟或每 1000 条对段根做一次
//! ECDSA P-256 签名并立即写入审计证据存储，只追加不覆盖。
//!
//! 本 crate 只承担其中**可以脱库判定的那一半**：
//! [`jcs`] 规范化、[`chain`] 哈希链与验证、[`segment`] 分段键与分组次序。
//! 取段锁、分配 `seq`、批量插入、更新段行这四件在调用方的业务事务内完成，
//! 签名与写证据在段锁之外的两个独立阶段完成（计划第 3.4.3 节 A、B、C 三段）。
//!
//! 这样切的理由：链的算法是整条证据体系的地基，**一处算错要到很久以后
//! 验证时才显形，那时已经查不回来**。脱库可测就能在写完当天就把它测穿。

pub mod chain;
pub mod jcs;
pub mod segment;

pub use chain::{
    anchor_digest, hash_of, to_hex, verify_segment, ChainError, ChainLink, GENESIS_PREV_HASH,
    HASH_LEN,
};
pub use jcs::{canonicalize, JcsError};
pub use segment::{group_by_day_ascending, SegmentKey, SHANGHAI_OFFSET_SECONDS};
