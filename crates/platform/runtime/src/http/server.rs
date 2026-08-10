//! HTTP 服务端骨架与优雅停机。
//!
//! 直接建在第三方库上：技术基线第 1.3 节写明工作区内既无也不新增 HTTP 系
//! `ep-adapter-*`，因此这里不是「缺一层适配」，是刻意不设那一层。

use std::net::SocketAddr;

use axum::Router;
use tokio::net::TcpListener;

use crate::config::ConfigError;

#[derive(Debug)]
pub enum ServeError {
    Bind { addr: String, detail: String },
    Serve(String),
    BadAddr { addr: String, detail: String },
}

impl std::fmt::Display for ServeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ServeError::Bind { addr, detail } => write!(f, "监听 {addr} 失败：{detail}"),
            ServeError::Serve(d) => write!(f, "HTTP 服务异常终止：{d}"),
            ServeError::BadAddr { addr, detail } => write!(f, "监听地址 {addr} 不合法：{detail}"),
        }
    }
}

impl std::error::Error for ServeError {}

impl From<ConfigError> for ServeError {
    fn from(e: ConfigError) -> Self {
        ServeError::Serve(e.to_string())
    }
}

pub fn parse_addr(addr: &str) -> Result<SocketAddr, ServeError> {
    addr.parse::<SocketAddr>()
        .map_err(|e| ServeError::BadAddr { addr: addr.to_string(), detail: e.to_string() })
}

/// 绑定并提供服务，直到 `shutdown` 完成。
///
/// 先绑定再返回监听地址，供随机端口的测试取用；绑定失败必须报错而不是重试，
/// 端口被占用时静默重试会让两份进程同时以为自己在服务。
pub async fn serve<F>(router: Router, addr: SocketAddr, shutdown: F) -> Result<(), ServeError>
where
    F: std::future::Future<Output = ()> + Send + 'static,
{
    let listener = TcpListener::bind(addr)
        .await
        .map_err(|e| ServeError::Bind { addr: addr.to_string(), detail: e.to_string() })?;
    axum::serve(listener, router)
        .with_graceful_shutdown(shutdown)
        .await
        .map_err(|e| ServeError::Serve(e.to_string()))
}

/// 与 [`serve`] 同一条路径，但把实际绑定到的地址交给调用方。
pub async fn bind(addr: SocketAddr) -> Result<(TcpListener, SocketAddr), ServeError> {
    let listener = TcpListener::bind(addr)
        .await
        .map_err(|e| ServeError::Bind { addr: addr.to_string(), detail: e.to_string() })?;
    let local = listener
        .local_addr()
        .map_err(|e| ServeError::Bind { addr: addr.to_string(), detail: e.to_string() })?;
    Ok((listener, local))
}

pub async fn serve_on<F>(listener: TcpListener, router: Router, shutdown: F) -> Result<(), ServeError>
where
    F: std::future::Future<Output = ()> + Send + 'static,
{
    axum::serve(listener, router)
        .with_graceful_shutdown(shutdown)
        .await
        .map_err(|e| ServeError::Serve(e.to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn loopback_addresses_parse() {
        assert_eq!(parse_addr("127.0.0.1:8080").unwrap().port(), 8080);
    }

    // 负样例断言的是地址解析这条规则本身：不合法的监听地址必须报错，
    // 不得回落到一个默认端口——回落会让进程听在没人预期的端口上。
    #[test]
    fn malformed_address_is_rejected_without_falling_back() {
        assert!(parse_addr("127.0.0.1").is_err());
        assert!(parse_addr("").is_err());
        assert!(parse_addr("localhost:8080").is_err(), "只接受 IP 字面量，不做名字解析");
    }
}
