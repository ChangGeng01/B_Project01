//! ep-platform-obs — 日志字段约定、指标注册表与追踪上下文。
//!
//! 本 crate 只依赖 ep-foundation，依赖方向由 `xtask archcheck` 逐条断言。
//! 指标名的唯一出处是 `docs/metrics-catalog.md` 的登记表，代码侧的注册落点
//! 由阶段 1 计划第 13 节新增决定五定在 `src/metrics/registry.rs`。

pub mod log;
pub mod metrics;
pub mod trace;

pub use metrics::registry::MetricsRegistry;
pub use trace::{CorrelationId, TraceContext};
