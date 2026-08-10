//! IPC 服务端。Unix domain socket 承载，权限 0660。
//!
//! 未实现的方法一律返回统一的未知方法错误而不是 panic：这条断言与方法名无关，
//! 后续阶段新增方法不需要改它。

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use std::sync::Arc;

use ep_foundation::error::codes::{PLATFORM_REQUEST_INVALID_PAYLOAD, PLATFORM_ROUTE_NOT_FOUND};
use serde_json::Value;
use tokio::net::{UnixListener, UnixStream};

use crate::frame::{read_frame, write_frame, FrameError};
use crate::message::{error_body, IpcRequest, IpcResponse, PROTOCOL_VERSION};

/// socket 文件权限。属主与属组由部署方（systemd 单元）设定，
/// 进程自身不做 chown：能 chown 的进程等于要以特权身份运行。
pub const SOCKET_MODE: u32 = 0o660;

#[async_trait::async_trait]
pub trait IpcMethod: Send + Sync {
    async fn call(&self, payload: Value) -> Result<Value, String>;
}

/// 方法表。方法名是编译期常量，注册重名视为编码错误。
#[derive(Default, Clone)]
pub struct MethodTable {
    methods: BTreeMap<&'static str, Arc<dyn IpcMethod>>,
}

impl MethodTable {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with(mut self, name: &'static str, method: Arc<dyn IpcMethod>) -> Self {
        let existed = self.methods.insert(name, method);
        assert!(existed.is_none(), "IPC 方法 {name} 重复注册");
        self
    }

    pub fn names(&self) -> Vec<&'static str> {
        self.methods.keys().copied().collect()
    }

    /// 处理一帧。任何不认识的输入都要变成一条响应，而不是让连接静默断开。
    pub async fn dispatch(&self, body: &[u8]) -> IpcResponse {
        let req: IpcRequest = match serde_json::from_slice(body) {
            Ok(r) => r,
            Err(e) => {
                return IpcResponse::failed(
                    "",
                    error_body(PLATFORM_REQUEST_INVALID_PAYLOAD, format!("报文不可解析：{e}")),
                )
            }
        };
        if req.v != PROTOCOL_VERSION {
            return IpcResponse::failed(
                &req.id,
                error_body(PLATFORM_REQUEST_INVALID_PAYLOAD, format!("协议版本 {} 不受支持", req.v)),
            );
        }
        let Some(method) = self.methods.get(req.method.as_str()) else {
            return IpcResponse::failed(
                &req.id,
                error_body(PLATFORM_ROUTE_NOT_FOUND, format!("未知方法 {}", req.method)),
            );
        };
        match method.call(req.payload).await {
            Ok(payload) => IpcResponse::ok(&req.id, payload),
            Err(detail) => IpcResponse::failed(&req.id, error_body(PLATFORM_REQUEST_INVALID_PAYLOAD, detail)),
        }
    }
}

pub struct IpcServer {
    path: PathBuf,
    max_frame_bytes: u32,
    methods: MethodTable,
}

#[derive(Debug)]
pub enum ServerError {
    Bind { path: PathBuf, detail: String },
    Permissions { path: PathBuf, detail: String },
}

impl std::fmt::Display for ServerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ServerError::Bind { path, detail } => write!(f, "绑定 {} 失败：{detail}", path.display()),
            ServerError::Permissions { path, detail } => {
                write!(f, "设置 {} 权限失败：{detail}", path.display())
            }
        }
    }
}

impl std::error::Error for ServerError {}

impl IpcServer {
    pub fn new(path: impl Into<PathBuf>, max_frame_bytes: u32, methods: MethodTable) -> Self {
        Self { path: path.into(), max_frame_bytes, methods }
    }

    pub fn path(&self) -> &Path {
        &self.path
    }

    /// 绑定并设置权限。上一次进程留下的 socket 文件要先删：
    /// 残留文件会让绑定报「地址已占用」，而实际上没有任何进程在听。
    pub fn bind(&self) -> Result<UnixListener, ServerError> {
        if let Some(parent) = self.path.parent() {
            std::fs::create_dir_all(parent)
                .map_err(|e| ServerError::Bind { path: self.path.clone(), detail: e.to_string() })?;
        }
        match std::fs::remove_file(&self.path) {
            Ok(()) => {}
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => {}
            Err(e) => return Err(ServerError::Bind { path: self.path.clone(), detail: e.to_string() }),
        }
        let listener = UnixListener::bind(&self.path)
            .map_err(|e| ServerError::Bind { path: self.path.clone(), detail: e.to_string() })?;
        use std::os::unix::fs::PermissionsExt;
        std::fs::set_permissions(&self.path, std::fs::Permissions::from_mode(SOCKET_MODE))
            .map_err(|e| ServerError::Permissions { path: self.path.clone(), detail: e.to_string() })?;
        Ok(listener)
    }

    /// 接受连接直到 `shutdown` 完成。停机时删除 socket 文件，不留残留。
    pub async fn serve<F>(self, listener: UnixListener, shutdown: F)
    where
        F: std::future::Future<Output = ()> + Send,
    {
        let methods = Arc::new(self.methods);
        let max = self.max_frame_bytes;
        tokio::pin!(shutdown);
        loop {
            tokio::select! {
                _ = &mut shutdown => break,
                accepted = listener.accept() => match accepted {
                    Ok((stream, _)) => {
                        let methods = methods.clone();
                        tokio::spawn(async move { serve_conn(stream, methods, max).await });
                    }
                    // 单次 accept 失败不拖垮服务端，但也不静默：交给调用方的日志
                    // 看不到这条，于是这里把它作为连接级事实吞掉是不行的——
                    // 因此以错误帧的形式无法表达时，只能中止 accept 循环。
                    Err(_) => break,
                },
            }
        }
        let _ = std::fs::remove_file(&self.path);
    }
}

async fn serve_conn(mut stream: UnixStream, methods: Arc<MethodTable>, max: u32) {
    loop {
        let body = match read_frame(&mut stream, max).await {
            Ok(b) => b,
            Err(FrameError::TooLarge { declared, limit }) => {
                let resp = IpcResponse::failed(
                    "",
                    error_body(PLATFORM_REQUEST_INVALID_PAYLOAD, format!("帧长 {declared} 超过上限 {limit}")),
                );
                let text = serde_json::to_vec(&resp).unwrap_or_default();
                let _ = write_frame(&mut stream, &text, max).await;
                return;
            }
            Err(_) => return,
        };
        let resp = methods.dispatch(&body).await;
        let text = match serde_json::to_vec(&resp) {
            Ok(t) => t,
            Err(_) => return,
        };
        if write_frame(&mut stream, &text, max).await.is_err() {
            return;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct Ping;

    #[async_trait::async_trait]
    impl IpcMethod for Ping {
        async fn call(&self, _payload: Value) -> Result<Value, String> {
            Ok(serde_json::json!({"process": "core-server"}))
        }
    }

    fn table() -> MethodTable {
        MethodTable::new().with("system.ping", Arc::new(Ping))
    }

    #[tokio::test]
    async fn known_method_answers_with_ok() {
        let req = serde_json::to_vec(&IpcRequest::new("system.ping", Value::Null)).unwrap();
        let resp = table().dispatch(&req).await;
        assert!(resp.ok);
        assert_eq!(resp.payload.unwrap()["process"], "core-server");
    }

    // 负样例断言的是「未知方法返回统一错误而不是 panic」这条规则本身。
    #[tokio::test]
    async fn unknown_method_returns_an_error_frame_instead_of_panicking() {
        let req = serde_json::to_vec(&IpcRequest::new("system.nope", Value::Null)).unwrap();
        let resp = table().dispatch(&req).await;
        assert!(!resp.ok);
        let err = resp.error.unwrap();
        assert_eq!(err.code, "PLATFORM.ROUTE.NOT_FOUND");
        assert!(err.message.contains("system.nope"));
    }

    #[tokio::test]
    async fn unparsable_frame_becomes_an_error_response() {
        let resp = table().dispatch(b"not json").await;
        assert!(!resp.ok);
        assert_eq!(resp.error.unwrap().code, "PLATFORM.REQUEST.INVALID_PAYLOAD");
    }

    #[tokio::test]
    async fn wrong_protocol_version_is_rejected() {
        let mut req = IpcRequest::new("system.ping", Value::Null);
        req.v = 2;
        let resp = table().dispatch(&serde_json::to_vec(&req).unwrap()).await;
        assert!(!resp.ok);
    }

    #[tokio::test]
    async fn socket_is_created_with_mode_0660_and_removed_on_stop() {
        use std::os::unix::fs::PermissionsExt;
        let dir = std::env::temp_dir().join(format!("ep-ipc-{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        let path = dir.join("test.sock");
        let server = IpcServer::new(&path, 1024, table());
        let listener = server.bind().expect("绑定应成功");
        let mode = std::fs::metadata(&path).unwrap().permissions().mode() & 0o777;
        assert_eq!(mode, SOCKET_MODE);

        let (tx, rx) = tokio::sync::oneshot::channel::<()>();
        let handle = tokio::spawn(async move {
            server
                .serve(listener, async move {
                    let _ = rx.await;
                })
                .await;
        });
        tx.send(()).unwrap();
        handle.await.unwrap();
        assert!(!path.exists(), "停机后不得残留 socket 文件");
        std::fs::remove_dir_all(&dir).ok();
    }

    #[tokio::test]
    async fn stale_socket_file_does_not_block_bind() {
        let dir = std::env::temp_dir().join(format!("ep-ipc-stale-{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        let path = dir.join("stale.sock");
        std::fs::write(&path, "上次进程留下的残留").unwrap();
        let server = IpcServer::new(&path, 1024, table());
        assert!(server.bind().is_ok());
        std::fs::remove_dir_all(&dir).ok();
    }
}
