//! ep-platform-obs — 日志字段约定、指标注册表、降级台账端口与追踪上下文。
//!
//! 本 crate 只依赖 ep-foundation，依赖方向由 `xtask archcheck` 逐条断言。
//! 指标名的唯一出处是 `docs/metrics-catalog.md` 的登记表，代码侧的注册落点
//! 由阶段 1 计划第 13 节新增决定五定在 `src/metrics/registry.rs`。
//! 降级台账端口按裁定 A-26/D-15 落在 `src/degradation.rs`，
//! 数据库实现由 ep-adapter-db-pg 承接（adapter 允许依赖 platform 端口）。

pub mod degradation;
pub mod log;
pub mod metrics;
pub mod trace;

pub use degradation::{DegradationKind, DegradationLedger, WindowOpenRequest, WindowRef};
pub use metrics::registry::MetricsRegistry;
pub use trace::{CorrelationId, TraceContext};
