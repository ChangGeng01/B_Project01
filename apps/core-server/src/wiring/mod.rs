//! core-server 的装配。
//!
//! `xtask archcheck` 的 unwired-absent 规则断言本目录下的全部文件中
//! 不出现任何以 Noop、Stub、Fake、Dummy 为前缀的实现类型或注入行。
//! 因此能力缺位在这里一律以「不注入」表达，由自检项如实报未覆盖，
//! 不以一个返回成功的空实现顶位。

pub mod db;
pub mod ipc;

pub use db::sql_probe;
pub use ipc::method_table;
