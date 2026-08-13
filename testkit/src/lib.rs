//! ep-testkit — 测试夹具与构造器。只作为 dev-dependencies 使用。

pub mod determinism;
pub mod idempotency_mem;
/// 法人越权 32 组矩阵（阶段 4 任务 #23，04 计划 §8.3）。
pub mod matrix_32;
/// 探针 schema 与探针表。按裁定 B-01 由 `ci-probe` feature 保护，默认关闭，
/// 不得进入发布制品（阶段 14 发布门禁项 `RG-CI-PROBE-ABSENT`）。
#[cfg(feature = "ci-probe")]
pub mod probe;
pub mod rls_matrix;
