//! ep-platform-outbox — Outbox 写入与消费、幂等键、死信、重投、投递统计。
//!
//! 阶段 3a 交付：幂等键仓储的三态判定纯逻辑（[`idempotency`]）；
//! 幂等键表 `platform_msg.idempotency_keys` 的 SQL 实现体在
//! ep-adapter-db-pg 的 `platform_msg/` 目录（platform 不依赖 adapter，
//! 判定逻辑沉本 crate，SQL 落适配层，同阶段 2 tenancy 口径）。
//! Outbox 写入与消费、死信、退避等其余能力随阶段 3b-1 补齐。
//!
//! 编译期断言：本 crate 的依赖方向由 `xtask archcheck` 逐条断言，
//! 允许的上游见技术基线第 1.3 节；退出条件 29 按 Cargo.toml 直读
//! 判定本 crate 无 identity/authz 依赖。

pub mod idempotency;

pub use idempotency::{hash_hex, judge, ExistingKeyRow, KeyState};
