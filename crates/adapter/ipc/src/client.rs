//! IPC 客户端。写出进程用它把心跳与上报送到 core-server。
//!
//! 连接不做重连循环：重连策略属调用方的编排（写出进程在 core 不可用时落 spool），
//! 藏在客户端里会让「落 spool」这条路径永远测不到。

use std::path::{Path, PathBuf};
use std::time::Duration;

use serde_json::Value;
use tokio::net::UnixStream;

use crate::frame::{read_frame, write_frame, FrameError};
use crate::message::{IpcRequest, IpcResponse};

#[derive(Debug)]
pub enum ClientError {
    Connect { path: PathBuf, detail: String },
    Frame(FrameError),
    Decode(String),
    Timeout(Duration),
    /// 对端返回了一条错误响应。这不是传输失败，调用方要能分辨。
    Remote(crate::message::IpcErrorBody),
    /// 响应的 id 与请求不匹配，说明串帧了。
    IdMismatch { want: String, got: String },
}

impl std::fmt::Display for ClientError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ClientError::Connect { path, detail } => write!(f, "连接 {} 失败：{detail}", path.display()),
            ClientError::Frame(e) => write!(f, "帧处理失败：{e}"),
            ClientError::Decode(d) => write!(f, "响应不可解析：{d}"),
            ClientError::Timeout(d) => write!(f, "等待响应超过 {} 毫秒", d.as_millis()),
            ClientError::Remote(e) => write!(f, "对端返回错误 {}：{}", e.code, e.message),
            ClientError::IdMismatch { want, got } => write!(f, "响应 id 不匹配，期望 {want} 实际 {got}"),
        }
    }
}

impl std::error::Error for ClientError {}

pub struct IpcClient {
    path: PathBuf,
    max_frame_bytes: u32,
    timeout: Duration,
}

impl IpcClient {
    pub fn new(path: impl Into<PathBuf>, max_frame_bytes: u32, timeout: Duration) -> Self {
        Self { path: path.into(), max_frame_bytes, timeout }
    }

    pub fn path(&self) -> &Path {
        &self.path
    }

    /// 发一次请求并等一条响应。每次调用一条新连接，形态最简单也最可判。
    pub async fn call(&self, method: &str, payload: Value) -> Result<Value, ClientError> {
        let req = IpcRequest::new(method, payload);
        let body = serde_json::to_vec(&req).map_err(|e| ClientError::Decode(e.to_string()))?;
        let fut = self.exchange(&req.id, body);
        match tokio::time::timeout(self.timeout, fut).await {
            Ok(r) => r,
            Err(_) => Err(ClientError::Timeout(self.timeout)),
        }
    }

    async fn exchange(&self, id: &str, body: Vec<u8>) -> Result<Value, ClientError> {
        let mut stream = UnixStream::connect(&self.path)
            .await
            .map_err(|e| ClientError::Connect { path: self.path.clone(), detail: e.to_string() })?;
        write_frame(&mut stream, &body, self.max_frame_bytes).await.map_err(ClientError::Frame)?;
        let raw = read_frame(&mut stream, self.max_frame_bytes).await.map_err(ClientError::Frame)?;
        let resp: IpcResponse =
            serde_json::from_slice(&raw).map_err(|e| ClientError::Decode(e.to_string()))?;
        if resp.id != id {
            return Err(ClientError::IdMismatch { want: id.to_string(), got: resp.id });
        }
        if !resp.ok {
            return Err(ClientError::Remote(resp.error.unwrap_or(crate::message::IpcErrorBody {
                code: "PLATFORM.SYSTEM.INTERNAL_ERROR".into(),
                category: "INFRASTRUCTURE".into(),
                message: "对端返回失败但未给出错误体".into(),
            })));
        }
        Ok(resp.payload.unwrap_or(Value::Null))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::server::{IpcMethod, IpcServer, MethodTable};
    use std::sync::Arc;

    struct Echo;

    #[async_trait::async_trait]
    impl IpcMethod for Echo {
        async fn call(&self, payload: Value) -> Result<Value, String> {
            Ok(payload)
        }
    }

    async fn with_server<F, Fut>(name: &str, body: F)
    where
        F: FnOnce(PathBuf) -> Fut,
        Fut: std::future::Future<Output = ()>,
    {
        let dir = std::env::temp_dir().join(format!("ep-ipc-c-{}-{name}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        let path = dir.join("s.sock");
        let server = IpcServer::new(&path, 1024, MethodTable::new().with("system.echo", Arc::new(Echo)));
        let listener = server.bind().unwrap();
        let (tx, rx) = tokio::sync::oneshot::channel::<()>();
        let handle = tokio::spawn(async move {
            server
                .serve(listener, async move {
                    let _ = rx.await;
                })
                .await;
        });
        body(path).await;
        let _ = tx.send(());
        let _ = handle.await;
        std::fs::remove_dir_all(&dir).ok();
    }

    #[tokio::test]
    async fn request_and_response_round_trip_over_the_socket() {
        with_server("ok", |path| async move {
            let client = IpcClient::new(&path, 1024, Duration::from_secs(2));
            let got = client.call("system.echo", serde_json::json!({"n": 1})).await.unwrap();
            assert_eq!(got["n"], 1);
        })
        .await;
    }

    // 负样例断言的是「未知方法返回错误」在客户端一侧同样可分辨。
    #[tokio::test]
    async fn unknown_method_surfaces_as_a_remote_error() {
        with_server("unknown", |path| async move {
            let client = IpcClient::new(&path, 1024, Duration::from_secs(2));
            let err = client.call("system.nope", Value::Null).await.unwrap_err();
            match err {
                ClientError::Remote(b) => assert_eq!(b.code, "PLATFORM.ROUTE.NOT_FOUND"),
                other => panic!("应为远端错误，实际 {other}"),
            }
        })
        .await;
    }

    #[tokio::test]
    async fn missing_socket_is_a_connect_error_not_a_silent_success() {
        let client = IpcClient::new("/nonexistent/ep.sock", 1024, Duration::from_millis(200));
        let err = client.call("system.ping", Value::Null).await.unwrap_err();
        assert!(matches!(err, ClientError::Connect { .. }));
    }
}
