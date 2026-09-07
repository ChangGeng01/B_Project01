//! ops-agent 的真实数据库装配。
//!
//! 本进程只持有 `ep_ops_ro` 的 Ops2 池；Rw/Ro 由 core-server
//! 持有，Worker 由 job-worker 持有，integration-gateway 不持库连接。
//! 能力未装配时以 None 表达，不注入恒成功占位实现。

pub mod db;
pub mod metrics;
pub mod probes;

pub use db::{budget_check, build};
