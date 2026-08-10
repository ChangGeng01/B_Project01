//! 指标注册表。
//!
//! 阶段 1 登记且只登记 `docs/metrics-catalog.md` 第 3 节的六项，别名一概不设：
//! 同义名是重复登记的主要来源。

pub mod histogram;
pub mod registry;

pub use registry::{MetricDef, MetricError, MetricKind, MetricsRegistry, REGISTERED};
