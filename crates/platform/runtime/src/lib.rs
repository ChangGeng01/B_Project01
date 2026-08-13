//! ep-platform-runtime — 进程运行时。
//!
//! 职责：进程生命周期状态机、分层配置加载、`SelfCheckRegistry`、健康与就绪端点、
//! 中间件栈与优雅停机。八个进程共用这一套骨架，形态差异留在各自的 apps 里。
//!
//! 依赖方向：本 crate 只依赖 `ep-foundation` 与其他 `ep-platform-*`，
//! 不依赖任何 `ep-adapter-*`（`xtask archcheck` 的 platform-no-adapter 会拦）。
//! IPC 的具体传输实现留在 `ep-adapter-ipc`，由 apps 在 `wiring/` 目录下注入；
//! HTTP 服务端骨架按技术基线第 1.3 节直接建在第三方库上，不设 HTTP 系 adapter。

pub mod boot;
pub mod cli;
pub mod config;
pub mod http;
pub mod incident;
pub mod lifecycle;
pub mod migrations;
pub mod process;
pub mod selfcheck;
pub mod serving;
pub mod shutdown;

pub use cli::Cli;
pub use lifecycle::{Event, Lifecycle, State};
pub use process::{BuildInfo, ProcessKind, ALL_PROCESSES};
pub use selfcheck::{SelfCheckRegistry, SelfCheckReport};
