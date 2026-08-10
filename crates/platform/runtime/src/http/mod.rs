//! HTTP 层：封套、系统端点、中间件栈、服务端与回环客户端。

pub mod client;
pub mod envelope;
pub mod headers;
pub mod middleware;
pub mod server;
pub mod state;
pub mod system;

pub use envelope::{ApiError, Detail, Envelope, ErrorBody};
pub use middleware::{Gate, SyncLimit};
pub use server::{bind, parse_addr, serve, serve_on, ServeError};
pub use state::SystemState;
pub use system::{
    core_system_router, minimal_router, ops_health_router, portal_system_router, Shared,
};
