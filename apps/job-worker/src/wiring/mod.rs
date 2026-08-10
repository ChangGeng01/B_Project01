//! job-worker 的装配。
//!
//! `xtask archcheck` 的 unwired-absent 规则断言本目录下的全部文件中
//! 不出现任何以 Noop、Stub、Fake、Dummy 为前缀的实现类型或注入行。
//! 能力缺位一律以「不注入」表达，由自检项如实报未覆盖。

pub mod db;

pub use db::sql_probe;
