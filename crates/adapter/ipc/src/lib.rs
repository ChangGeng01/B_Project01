//! ep-adapter-ipc — 进程间接口的客户端与服务端，Unix domain socket 承载。
//!
//! 帧格式为 4 字节大端长度前缀加 JSON 体，单帧上限 1 MiB（阶段 1 计划第 6.3 节）。
//! 本阶段只实现 `system.ping` 与 `system.version` 两个方法，方法本体由 apps 注入；
//! 基线第 2 节的四类上报由阶段 14 连同方法名一次定义，这里不预留方法名。
//!
//! 依赖方向：本 crate 只依赖 ep-foundation，不依赖任何其他 adapter，
//! 也不依赖观测层——落 spool 时被丢弃的条数如实返回给调用方去记 ERROR。

pub mod client;
pub mod forward;
pub mod frame;
pub mod message;
pub mod server;
pub mod spool;

pub use client::{ClientError, IpcClient};
pub use forward::{ForwardOutcome, Forwarder, Pending, ReplayOutcome};
pub use frame::{FrameError, DEFAULT_MAX_FRAME_BYTES};
pub use message::{error_body, IpcErrorBody, IpcRequest, IpcResponse, PROTOCOL_VERSION};
pub use server::{IpcMethod, IpcServer, MethodTable, ServerError, SOCKET_MODE};
pub use spool::{AppendOutcome, Spool, SpoolError};
