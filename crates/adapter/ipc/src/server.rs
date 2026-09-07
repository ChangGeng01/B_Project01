//! IPC 服务端。承载物由 [`crate::transport`] 提供：Unix 侧是域套接字，
//! Windows 侧是命名管道（裁定 F-08 第 4.3 节）。本模块只管协议面。
//!
//! 未实现的方法一律返回统一的未知方法错误而不是 panic：这条断言与方法名无关，
//! 后续阶段新增方法不需要改它。

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use std::sync::Arc;

use ep_foundation::error::codes::{PLATFORM_REQUEST_INVALID_PAYLOAD, PLATFORM_ROUTE_NOT_FOUND};
use serde_json::Value;
use tokio::io::{AsyncRead, AsyncWrite};

use crate::transport::{IpcListener, TransportError};

use crate::frame::{read_frame, write_frame, FrameError};
use crate::message::{error_body, IpcRequest, IpcResponse, PROTOCOL_VERSION};

/// 承载物的访问控制取值。取自传输层，各平台语义不同：
/// Unix 是 socket 文件权限位，Windows 由安全描述符承担、该常量无语义。
/// 保留这个名字是为了不改既有引用点。
pub const SOCKET_MODE: u32 = crate::transport::ACCESS_MODE;

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
                    error_body(
                        PLATFORM_REQUEST_INVALID_PAYLOAD,
                        format!("报文不可解析：{e}"),
                    ),
                )
            }
        };
        if req.v != PROTOCOL_VERSION {
            return IpcResponse::failed(
                &req.id,
                error_body(
                    PLATFORM_REQUEST_INVALID_PAYLOAD,
                    format!("协议版本 {} 不受支持", req.v),
                ),
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
            Err(detail) => IpcResponse::failed(
                &req.id,
                error_body(PLATFORM_REQUEST_INVALID_PAYLOAD, detail),
            ),
        }
    }
}

pub struct IpcServer {
    path: PathBuf,
    max_frame_bytes: u32,
    methods: MethodTable,
}

// The protocol loop owns the listener; this private seam lets lifecycle tests
// inject accept failures without changing platform security gates or OS state.
#[async_trait::async_trait]
trait ConnectionListener: Send {
    type Stream: AsyncRead + AsyncWrite + Unpin + Send + 'static;
    async fn accept(&mut self) -> std::io::Result<Self::Stream>;
    fn cleanup(&self);
}

#[async_trait::async_trait]
impl ConnectionListener for IpcListener {
    type Stream = crate::transport::IpcStream;
    async fn accept(&mut self) -> std::io::Result<Self::Stream> {
        IpcListener::accept(self).await
    }
    fn cleanup(&self) {
        IpcListener::cleanup(self);
    }
}

#[derive(Debug)]
pub enum ServerError {
    Bind { path: PathBuf, detail: String },
    Permissions { path: PathBuf, detail: String },
}

impl std::fmt::Display for ServerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ServerError::Bind { path, detail } => {
                write!(f, "绑定 {} 失败：{detail}", path.display())
            }
            ServerError::Permissions { path, detail } => {
                write!(f, "设置 {} 权限失败：{detail}", path.display())
            }
        }
    }
}

impl std::error::Error for ServerError {}

impl IpcServer {
    pub fn new(path: impl Into<PathBuf>, max_frame_bytes: u32, methods: MethodTable) -> Self {
        Self {
            path: path.into(),
            max_frame_bytes,
            methods,
        }
    }

    pub fn path(&self) -> &Path {
        &self.path
    }

    /// 建立监听。返回的是**不透明类型**——按裁定 F-08 第八节，
    /// 该类型不得泄露到 apps，否则换平台时装配层要跟着改。
    /// 残留清理、权限设置与平台差异全在传输层，本函数只做错误归类。
    pub fn bind(&self) -> Result<IpcListener, ServerError> {
        IpcListener::bind(&self.path).map_err(|e| match e {
            TransportError::Access { path, detail } => ServerError::Permissions { path, detail },
            TransportError::Listen { path, detail } | TransportError::Connect { path, detail } => {
                ServerError::Bind { path, detail }
            }
        })
    }

    /// 接受连接直到 `shutdown` 完成，然后等待已接受连接排空。
    /// 外层排空期限到达后丢弃本 future，JoinSet 会取消所有在途连接。
    pub async fn serve<F>(
        self,
        listener: IpcListener,
        shutdown: F,
        on_failure: impl FnOnce(String) + Send,
    ) where
        F: std::future::Future<Output = ()> + Send,
    {
        self.serve_connections(listener, shutdown, on_failure).await;
    }

    async fn serve_connections<L, F>(
        self,
        mut listener: L,
        shutdown: F,
        on_failure: impl FnOnce(String) + Send,
    ) where
        L: ConnectionListener,
        F: std::future::Future<Output = ()> + Send,
    {
        let methods = Arc::new(self.methods);
        let max = self.max_frame_bytes;
        let mut connections = tokio::task::JoinSet::new();
        tokio::pin!(shutdown);
        loop {
            tokio::select! {
                biased;
                _ = &mut shutdown => break,
                _ = connections.join_next(), if !connections.is_empty() => {},
                accepted = listener.accept() => match accepted {
                    Ok(stream) => {
                        let methods = methods.clone();
                        connections.spawn(async move { serve_conn(stream, methods, max).await });
                    }
                    // 必须在等待在途连接前报告失败，否则 idle 连接会把外层
                    // Internal 停机与排空期限一起无限延迟。
                    Err(error) => {
                        on_failure(error.to_string());
                        break;
                    },
                },
            }
        }
        listener.cleanup();
        drop(listener);
        while connections.join_next().await.is_some() {}
    }
}

async fn serve_conn<S>(mut stream: S, methods: Arc<MethodTable>, max: u32)
where
    S: AsyncRead + AsyncWrite + Unpin,
{
    loop {
        let body = match read_frame(&mut stream, max).await {
            Ok(b) => b,
            Err(FrameError::TooLarge { declared, limit }) => {
                let resp = IpcResponse::failed(
                    "",
                    error_body(
                        PLATFORM_REQUEST_INVALID_PAYLOAD,
                        format!("帧长 {declared} 超过上限 {limit}"),
                    ),
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

    struct FailingListener {
        stream: Option<tokio::io::DuplexStream>,
        fail: Arc<tokio::sync::Notify>,
    }

    #[async_trait::async_trait]
    impl ConnectionListener for FailingListener {
        type Stream = tokio::io::DuplexStream;
        async fn accept(&mut self) -> std::io::Result<Self::Stream> {
            if let Some(stream) = self.stream.take() {
                return Ok(stream);
            }
            self.fail.notified().await;
            Err(std::io::Error::other("injected accept failure"))
        }
        fn cleanup(&self) {}
    }

    #[tokio::test]
    async fn accept_failure_notifies_owner_before_draining_idle_connection() {
        use tokio::io::AsyncReadExt;
        let (mut client, stream) = tokio::io::duplex(1024);
        let fail = Arc::new(tokio::sync::Notify::new());
        let listener = FailingListener {
            stream: Some(stream),
            fail: fail.clone(),
        };
        let (failed_tx, failed_rx) = tokio::sync::oneshot::channel();
        let server = IpcServer::new("unused-test-listener", 1024, table());
        let mut serving =
            Box::pin(
                server.serve_connections(listener, std::future::pending(), move |detail| {
                    let _ = failed_tx.send(detail);
                }),
            );
        // Drive acceptance before asking the next accept to fail. The idle
        // stream is a real framed connection waiting for its first request.
        std::future::poll_fn(|cx| {
            use std::future::Future;
            assert!(serving.as_mut().poll(cx).is_pending());
            std::task::Poll::Ready(())
        })
        .await;
        fail.notify_one();
        let failure = tokio::select! {
            detail = tokio::time::timeout(std::time::Duration::from_millis(100), failed_rx) => detail,
            _ = &mut serving => panic!("accepted connection must remain owned during drain"),
        };
        assert_eq!(
            failure
                .expect("listener failure must reach the owner promptly")
                .unwrap(),
            "injected accept failure"
        );
        let start = std::time::Instant::now();
        assert!(
            tokio::time::timeout(std::time::Duration::from_millis(30), &mut serving)
                .await
                .is_err()
        );
        assert!(start.elapsed() >= std::time::Duration::from_millis(30));
        drop(serving); // the owner's bounded drain deadline expires
        let mut byte = [0];
        assert_eq!(
            tokio::time::timeout(
                std::time::Duration::from_millis(100),
                client.read(&mut byte)
            )
            .await
            .unwrap()
            .unwrap(),
            0
        );
    }

    // Host transport is only a harness for the shared connection lifecycle;
    // these tests do not provide Windows DACL/token or runtime authority.
    #[cfg(unix)]
    struct BlockedMethod {
        entered: tokio::sync::Notify,
        release: tokio::sync::Notify,
        dropped: Arc<tokio::sync::Notify>,
    }

    #[cfg(unix)]
    #[async_trait::async_trait]
    impl IpcMethod for BlockedMethod {
        async fn call(&self, _payload: Value) -> Result<Value, String> {
            struct OnDrop(Arc<tokio::sync::Notify>);
            impl Drop for OnDrop {
                fn drop(&mut self) {
                    self.0.notify_one();
                }
            }
            let _guard = OnDrop(self.dropped.clone());
            self.entered.notify_one();
            self.release.notified().await;
            Ok(serde_json::json!({"completed": true}))
        }
    }

    #[cfg(unix)]
    async fn blocked_connection(
        name: &str,
    ) -> (
        Arc<BlockedMethod>,
        tokio::task::JoinHandle<()>,
        tokio::task::JoinHandle<Result<Value, crate::client::ClientError>>,
        tokio::sync::oneshot::Sender<()>,
        PathBuf,
    ) {
        let path =
            std::env::temp_dir().join(format!("ep-drain-{}-{name}.sock", std::process::id()));
        let method = Arc::new(BlockedMethod {
            entered: tokio::sync::Notify::new(),
            release: tokio::sync::Notify::new(),
            dropped: Arc::new(tokio::sync::Notify::new()),
        });
        let server = IpcServer::new(
            &path,
            1024,
            MethodTable::new().with("blocked", method.clone()),
        );
        let listener = server.bind().unwrap();
        let (tx, rx) = tokio::sync::oneshot::channel();
        let handle = tokio::spawn(server.serve(
            listener,
            async {
                let _ = rx.await;
            },
            |detail| panic!("unexpected listener failure: {detail}"),
        ));
        let client = crate::client::IpcClient::new(&path, 1024, std::time::Duration::from_secs(2));
        let request = tokio::spawn(async move { client.call("blocked", Value::Null).await });
        tokio::time::timeout(std::time::Duration::from_secs(1), method.entered.notified())
            .await
            .unwrap();
        (method, handle, request, tx, path)
    }

    #[cfg(unix)]
    #[tokio::test]
    async fn shutdown_drains_accepted_request_before_returning() {
        let (method, mut server, request, shutdown, path) = blocked_connection("drain").await;
        shutdown.send(()).unwrap();
        assert!(
            tokio::time::timeout(std::time::Duration::from_millis(30), &mut server)
                .await
                .is_err(),
            "server must own accepted work until its response completes"
        );
        method.release.notify_one();
        assert_eq!(
            request.await.unwrap().unwrap(),
            serde_json::json!({"completed":true})
        );
        tokio::time::timeout(std::time::Duration::from_secs(1), server)
            .await
            .unwrap()
            .unwrap();
        assert!(!path.exists());
    }

    #[cfg(unix)]
    #[tokio::test]
    async fn dropping_server_cancels_owned_connection_work() {
        let (method, server, request, _shutdown, path) = blocked_connection("abort").await;
        server.abort();
        assert!(server.await.unwrap_err().is_cancelled());
        assert!(
            tokio::time::timeout(
                std::time::Duration::from_millis(100),
                method.dropped.notified()
            )
            .await
            .is_ok(),
            "dropping serve must abort connection futures rather than detach them"
        );
        assert!(request.await.unwrap().is_err());
        std::fs::remove_file(path).ok();
    }

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

    /// 该用例判的是 Unix 侧承载物的权限位与残留清理，被测对象只在该平台存在。
    /// 按裁定 F-09-2 第三条（零 Linux 开发的效力范围），不为 Unix 分支新增测试，
    /// 但既有的这一条保留——删它等于在还没有 Windows 侧替代用例之前先把覆盖面砍掉。
    /// Windows 侧的对应判据（管道 DACL 与名字被占时 fail-closed）随服务宿主层一并落。
    #[cfg(unix)]
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
                .serve(
                    listener,
                    async move {
                        let _ = rx.await;
                    },
                    |detail| panic!("unexpected listener failure: {detail}"),
                )
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
