//! ep-platform-outbox — Outbox 写入与消费、幂等键、死信、重投、投递统计。
//!
//! 阶段 3a 交付：幂等键仓储的三态判定纯逻辑（[`idempotency`]）；
//! 幂等键表 `platform_msg.idempotency_keys` 的 SQL 实现体在
//! ep-adapter-db-pg 的 `platform_msg/` 目录（platform 不依赖 adapter，
//! 判定逻辑沉本 crate，SQL 落适配层，同阶段 2 tenancy 口径）。
//!
//! 本轮补齐投递侧的**纯判定面**：[`delivery`] 的状态机、八档退避与死信状态，
//! [`consumption`] 的消费端去重判定。取件语句、行锁、`available_at` 的落库
//! 仍在适配层——本 crate 一如既往不碰数据库。
//!
//! 为什么先补判定面：Outbox 的三组必过测试（至少一次投递、重复投递去重、
//! 崩溃恢复后不丢不重）里，前两组的判据完全落在这两个模块上，
//! 而它们**脱库可测**。第三组要真实崩溃，留给集成测试。
//!
//! 编译期断言：本 crate 的依赖方向由 `xtask archcheck` 逐条断言，
//! 允许的上游见技术基线第 1.3 节；退出条件 29 按 Cargo.toml 直读
//! 判定本 crate 无 identity/authz 依赖。

pub mod consumption;
pub mod delivery;
pub mod idempotency;

pub use consumption::{ConsumeDecision, ConsumptionKey};
pub use delivery::{
    poll_interval, DeadLetterState, NextStep, OutboxStatus, BACKOFF_SCHEDULE, FETCH_BATCH_LIMIT,
    MAX_RETRIES, POLL_INTERVAL_BUSY, POLL_INTERVAL_IDLE,
};
pub use idempotency::{hash_hex, judge, ExistingKeyRow, KeyState};
