//! ep-adapter-ipc — 进程间接口的客户端与服务端。承载物由 `transport` 模块提供：
//! Unix 侧是域套接字，Windows 侧是命名管道（裁定 F-08 第 4.3 节）。
//!
//! 帧格式为 4 字节大端长度前缀加 JSON 体，单帧上限 1 MiB（阶段 1 计划第 6.3 节）。
//! 方法本体由 apps 注入；本 crate 只固定协议与跨进程共用的端点名，绝不在
//! 客户端和服务端各抄一份路径。业务 operation 仍由各 app 的方法表封闭登记。
//!
//! 依赖方向：本 crate 只依赖 ep-foundation，不依赖任何其他 adapter，
//! 也不依赖观测层——落 spool 时被丢弃的条数如实返回给调用方去记 ERROR。

pub mod client;
pub mod forward;
pub mod frame;
pub mod message;
pub mod server;
pub mod spool;
pub mod transport;

pub use client::{ClientError, IpcClient};
pub use forward::{ForwardOutcome, Forwarder, Pending, ReplayOutcome};
pub use frame::{FrameError, DEFAULT_MAX_FRAME_BYTES};
pub use message::{error_body, IpcErrorBody, IpcRequest, IpcResponse, PROTOCOL_VERSION};
pub use server::{IpcMethod, IpcServer, MethodTable, ServerError, SOCKET_MODE};
pub use spool::{AppendOutcome, Spool, SpoolError};
pub use transport::{IpcListener, IpcStream, TransportError};

/// 产品侧 IPC 端点的唯一代码常量。Windows 是裁定冻结的命名管道；非 Windows
/// 值只用于开发与测试，不构成生产部署口径。客户端和服务端都必须引用这些常量，
/// 不得各自复制路径字符串。
#[cfg(windows)]
pub const CORE_ENDPOINT: &str = r"\\.\pipe\ep-core";
#[cfg(not(windows))]
pub const CORE_ENDPOINT: &str = "/run/ep/ipc/core.sock";

#[cfg(windows)]
pub const INTEGRATION_ENDPOINT: &str = r"\\.\pipe\ep-integ";
#[cfg(not(windows))]
pub const INTEGRATION_ENDPOINT: &str = "/run/ep/ipc/integration.sock";

#[cfg(windows)]
pub const PLUGIN_ENDPOINT: &str = r"\\.\pipe\ep-plugin";
#[cfg(not(windows))]
pub const PLUGIN_ENDPOINT: &str = "/run/ep/ipc/plugin.sock";
