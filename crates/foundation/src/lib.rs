//! ep-foundation — 稳定通用类型。
//!
//! 本 crate 不依赖工作区内任何 crate（技术基线第 1.3 节第一条允许项）。
//! 第 1.4 节冻结的跨阶段共享类型全部落在这里，签名与取值全阶段唯一。

pub mod capability;
pub mod error;
pub mod id;
pub mod module;
pub mod port;
pub mod principal;
pub mod security;

pub use error::{AppError, ErrorCode};
pub use id::Id;
pub use module::ModuleCode;
