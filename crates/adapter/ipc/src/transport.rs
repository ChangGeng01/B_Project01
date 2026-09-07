//! IPC 的传输边界。协议面与生命周期面在此分开。
//!
//! 立这一层的原因见裁定 F-08 第八节：`client.rs` 与 `server.rs` 的 Unix 绑定
//! **不是纯 `cfg` 能解决的**——两个平台的「监听」根本不是同一种东西。
//! Unix 侧有一个长驻的 `UnixListener`，反复 `accept`；Windows 侧**没有 listener 对象**，
//! 一个 `NamedPipeServer` 实例只服务一个连接，循环里要先建下一个实例再交出当前实例。
//!
//! 因此本模块只承担三件事，协议面一行都不碰：
//! 一、把「监听」抽象成不透明的 [`IpcListener`]，其构造与 `accept` 各平台各实现一次；
//! 二、把「连接」抽象成 [`IpcStream`]，它在两个平台上都实现 `AsyncRead` 与 `AsyncWrite`，
//!     于是 `serve_conn` 的帧循环可以对它泛型，一份代码两平台共用；
//! 三、把客户端的「连上去」抽象成 [`connect`]。
//!
//! **`bind()` 的返回类型不得泄露到 apps**：apps 侧只 `match ipc.bind()`，
//! 拿到的是本模块的不透明类型，因此换平台时 `apps/` 一行不动。
//! 这条是设计约束不是实现细节，改动它等于把平台差异漏进装配层。
//!
//! Windows 一侧已在 macOS 上完成 `x86_64-pc-windows-msvc` 交叉编译，但未在
//! Windows Server 实机运行。更重要的是，DACL 与双向 token 身份核验尚未交付；
//! 本模块在 Windows 上因此默认失败关闭，不能把“编译通过”误报成“安全可用”。

use std::path::{Path, PathBuf};

/// 传输层错误。与 `ServerError` 分开：这一层只知道「连不上」「建不出来」，
/// 不知道任何协议语义。
#[derive(Debug)]
pub enum TransportError {
    /// 建立监听失败。Windows 侧名字被别人占住也落这一档——
    /// 那是 fail-closed，不是可重试的错误。
    Listen { path: PathBuf, detail: String },
    /// 设置访问控制失败。Unix 是权限位；Windows 在完整安全描述符与 peer token
    /// 核验交付前固定走这一错误，绝不回退默认 DACL。
    Access { path: PathBuf, detail: String },
    /// 连接失败。
    Connect { path: PathBuf, detail: String },
}

impl std::fmt::Display for TransportError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TransportError::Listen { path, detail } => {
                write!(f, "监听 {} 失败：{detail}", path.display())
            }
            TransportError::Access { path, detail } => {
                write!(f, "设置 {} 访问控制失败：{detail}", path.display())
            }
            TransportError::Connect { path, detail } => {
                write!(f, "连接 {} 失败：{detail}", path.display())
            }
        }
    }
}

impl std::error::Error for TransportError {}

#[cfg(unix)]
mod imp {
    use super::{Path, PathBuf, TransportError};

    /// socket 文件权限。属主与属组由部署方设定，进程自身不做 chown：
    /// 能 chown 的进程等于要以特权身份运行。
    pub const ACCESS_MODE: u32 = 0o660;

    pub type IpcStream = tokio::net::UnixStream;

    pub struct IpcListener {
        inner: tokio::net::UnixListener,
        path: PathBuf,
    }

    impl IpcListener {
        pub fn bind(path: &Path) -> Result<Self, TransportError> {
            if let Some(parent) = path.parent() {
                std::fs::create_dir_all(parent).map_err(|e| TransportError::Listen {
                    path: path.to_path_buf(),
                    detail: e.to_string(),
                })?;
            }
            // 上一次进程留下的 socket 文件要先删：残留文件会让绑定报「地址已占用」，
            // 而实际上没有任何进程在听。
            match std::fs::remove_file(path) {
                Ok(()) => {}
                Err(e) if e.kind() == std::io::ErrorKind::NotFound => {}
                Err(e) => {
                    return Err(TransportError::Listen {
                        path: path.to_path_buf(),
                        detail: e.to_string(),
                    })
                }
            }
            let inner =
                tokio::net::UnixListener::bind(path).map_err(|e| TransportError::Listen {
                    path: path.to_path_buf(),
                    detail: e.to_string(),
                })?;
            use std::os::unix::fs::PermissionsExt;
            std::fs::set_permissions(path, std::fs::Permissions::from_mode(ACCESS_MODE)).map_err(
                |e| TransportError::Access {
                    path: path.to_path_buf(),
                    detail: e.to_string(),
                },
            )?;
            Ok(Self {
                inner,
                path: path.to_path_buf(),
            })
        }

        pub async fn accept(&mut self) -> std::io::Result<IpcStream> {
            self.inner.accept().await.map(|(s, _)| s)
        }

        /// 停机清理。Unix 侧要删 socket 文件，不留残留。
        pub fn cleanup(&self) {
            let _ = std::fs::remove_file(&self.path);
        }
    }

    pub async fn connect(path: &Path) -> Result<IpcStream, TransportError> {
        tokio::net::UnixStream::connect(path)
            .await
            .map_err(|e| TransportError::Connect {
                path: path.to_path_buf(),
                detail: e.to_string(),
            })
    }
}

#[cfg(windows)]
mod imp {
    use super::{Path, PathBuf, TransportError};
    use std::pin::Pin;
    use std::task::{Context, Poll};
    use tokio::io::{AsyncRead, AsyncWrite, ReadBuf};
    use tokio::net::windows::named_pipe::{
        ClientOptions, NamedPipeClient, NamedPipeServer, ServerOptions,
    };

    /// Unix 侧的 0660 在本平台没有对应物，访问控制由安全描述符承担。
    /// 保留同名常量只为让协议面不必分叉；**它不表达任何本平台语义**。
    pub const ACCESS_MODE: u32 = 0;

    const WINDOWS_IPC_SECURITY_NOT_IMPLEMENTED: &str =
        "NOT_IMPLEMENTED：Windows IPC DACL 与双向 token 身份核验尚未交付";

    /// 该门在安全描述符、客户端 SQOS、服务端冒充核验和客户端服务 token 核验
    /// 同批交付并通过 Windows 实机负例之前必须保持失败。把它改成 `Ok(())` 而不
    /// 同时交付上述控制，会重新开放默认 DACL 的本地越权路径。
    fn require_server_security(path: &Path) -> Result<(), TransportError> {
        Err(TransportError::Access {
            path: path.to_path_buf(),
            detail: WINDOWS_IPC_SECURITY_NOT_IMPLEMENTED.to_string(),
        })
    }

    fn require_client_security(path: &Path) -> Result<(), TransportError> {
        Err(TransportError::Connect {
            path: path.to_path_buf(),
            detail: WINDOWS_IPC_SECURITY_NOT_IMPLEMENTED.to_string(),
        })
    }

    /// 服务端接受到的是 `NamedPipeServer`，客户端打开得到的是
    /// `NamedPipeClient`；二者不能用错误的类型别名强行合并。协议层只需要同一组
    /// `AsyncRead + AsyncWrite` 能力，因此在传输边界内封装这两个具体类型。
    pub enum IpcStream {
        Server(NamedPipeServer),
        Client(NamedPipeClient),
    }

    impl AsyncRead for IpcStream {
        fn poll_read(
            self: Pin<&mut Self>,
            cx: &mut Context<'_>,
            buf: &mut ReadBuf<'_>,
        ) -> Poll<std::io::Result<()>> {
            match self.get_mut() {
                Self::Server(stream) => Pin::new(stream).poll_read(cx, buf),
                Self::Client(stream) => Pin::new(stream).poll_read(cx, buf),
            }
        }
    }

    impl AsyncWrite for IpcStream {
        fn poll_write(
            self: Pin<&mut Self>,
            cx: &mut Context<'_>,
            buf: &[u8],
        ) -> Poll<std::io::Result<usize>> {
            match self.get_mut() {
                Self::Server(stream) => Pin::new(stream).poll_write(cx, buf),
                Self::Client(stream) => Pin::new(stream).poll_write(cx, buf),
            }
        }

        fn poll_flush(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<std::io::Result<()>> {
            match self.get_mut() {
                Self::Server(stream) => Pin::new(stream).poll_flush(cx),
                Self::Client(stream) => Pin::new(stream).poll_flush(cx),
            }
        }

        fn poll_shutdown(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<std::io::Result<()>> {
            match self.get_mut() {
                Self::Server(stream) => Pin::new(stream).poll_shutdown(cx),
                Self::Client(stream) => Pin::new(stream).poll_shutdown(cx),
            }
        }
    }

    /// 本平台没有 listener 对象：一个实例只服务一个连接。
    /// 因此这里自己持有「下一个待连接的实例」，`accept` 交出当前实例前先把下一个建好，
    /// 否则两次连接之间会出现一个没有实例在等的窗口，客户端在该窗口内连会失败。
    pub struct IpcListener {
        pending: Option<NamedPipeServer>,
        name: String,
        path: PathBuf,
    }

    impl IpcListener {
        pub fn bind(path: &Path) -> Result<Self, TransportError> {
            require_server_security(path)?;
            let name = path.to_string_lossy().into_owned();
            // `first_pipe_instance(true)` 使名字已被别人占住时**启动失败**。
            // 这是 fail-closed 不是防护（见裁定 F-08 做不到九）：本平台的管道名字空间
            // 没有创建侧准入控制，任何本地用户都能抢先建同名管道。不设这一项更坏——
            // 第二个进程可为同一名字追加实例并分走一部分连接，那比启动失败难查得多。
            let pending = ServerOptions::new()
                .first_pipe_instance(true)
                .reject_remote_clients(true)
                .create(&name)
                .map_err(|e| TransportError::Listen {
                    path: path.to_path_buf(),
                    detail: e.to_string(),
                })?;
            Ok(Self {
                pending: Some(pending),
                name,
                path: path.to_path_buf(),
            })
        }

        fn create_pending(&self) -> std::io::Result<NamedPipeServer> {
            ServerOptions::new()
                .reject_remote_clients(true)
                .create(&self.name)
        }

        pub async fn accept(&mut self) -> std::io::Result<IpcStream> {
            // 预建下一实例曾失败，或上一实例在 connect 时失败后，pending 会暂时缺席。
            // 每次 accept 都先同步重建，使一次瞬时失败不会永久毒化 listener。
            if self.pending.is_none() {
                self.pending = Some(self.create_pending()?);
            }
            let server = self.pending.take().expect("上方已保证 pending 实例存在");
            server.connect().await?;
            // 尽力预建下一实例；失败时仍必须交付已经连接的当前流。pending 保持 None，
            // 下一次 accept 会在等待连接前同步重建，并在仍失败时如实返回错误。
            if let Ok(next) = self.create_pending() {
                self.pending = Some(next);
            }
            Ok(IpcStream::Server(server))
        }

        /// 本平台无需清理：管道实例随最后一个句柄由内核回收，不存在残留文件。
        /// 保留空实现是为了让协议面不必分叉。
        pub fn cleanup(&self) {
            let _ = &self.path;
        }
    }

    pub async fn connect(path: &Path) -> Result<IpcStream, TransportError> {
        require_client_security(path)?;
        // 本平台必须处理 ERROR_PIPE_BUSY：服务端在但当前没有空闲实例时返回该码。
        // 不重试会把「core 在但忙」误报成「core 不可用」并落 spool，
        // 那是一条会让人查错方向的假象。
        const ERROR_PIPE_BUSY: i32 = 231;
        const MAX_ATTEMPTS: u32 = 8;
        let name = path.to_string_lossy().into_owned();
        let mut attempt = 0u32;
        loop {
            match ClientOptions::new().open(&name) {
                Ok(c) => return Ok(IpcStream::Client(c)),
                Err(e) if e.raw_os_error() == Some(ERROR_PIPE_BUSY) && attempt < MAX_ATTEMPTS => {
                    attempt += 1;
                    tokio::time::sleep(std::time::Duration::from_millis(50)).await;
                }
                Err(e) => {
                    return Err(TransportError::Connect {
                        path: path.to_path_buf(),
                        detail: e.to_string(),
                    })
                }
            }
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use std::time::Duration;

        #[tokio::test]
        async fn accept_keeps_the_connected_stream_when_prebuild_fails() {
            let connected_name = format!(
                r"\\.\pipe\ep-ipc-preserve-connected-{}",
                uuid::Uuid::now_v7()
            );
            let pending = ServerOptions::new()
                .reject_remote_clients(true)
                .create(&connected_name)
                .expect("测试应能创建首个命名管道实例");
            let mut listener = IpcListener {
                pending: Some(pending),
                path: PathBuf::from(&connected_name),
                // 空名字保证当前流连接后，预建下一实例失败。
                name: String::new(),
            };

            let client = tokio::spawn(async move {
                ClientOptions::new()
                    .open(&connected_name)
                    .expect("客户端应能连接首个实例")
            });
            let accepted = tokio::time::timeout(Duration::from_secs(5), listener.accept())
                .await
                .expect("接受首个连接不应超时")
                .expect("下一实例预建失败不得丢弃已连接流");

            assert!(matches!(accepted, IpcStream::Server(_)));
            assert!(
                listener.pending.is_none(),
                "预建失败后应保留可由下一次 accept 恢复的缺席状态"
            );
            let _client = client.await.expect("客户端任务不应 panic");
        }

        #[tokio::test]
        async fn accept_rebuilds_a_missing_pending_instance() {
            let name = format!(
                r"\\.\pipe\ep-ipc-recover-missing-pending-{}",
                uuid::Uuid::now_v7()
            );
            let mut listener = IpcListener {
                pending: None,
                path: PathBuf::from(&name),
                name: name.clone(),
            };

            let client = tokio::spawn(async move {
                const ERROR_FILE_NOT_FOUND: i32 = 2;
                loop {
                    match ClientOptions::new().open(&name) {
                        Ok(client) => return client,
                        Err(error) if error.raw_os_error() == Some(ERROR_FILE_NOT_FOUND) => {
                            tokio::task::yield_now().await;
                        }
                        Err(error) => panic!("连接恢复后的命名管道失败：{error}"),
                    }
                }
            });

            let accepted = tokio::time::timeout(Duration::from_secs(5), listener.accept())
                .await
                .expect("恢复缺失实例不应超时")
                .expect("accept 应同步重建缺失的 pending 实例");
            assert!(matches!(accepted, IpcStream::Server(_)));
            assert!(listener.pending.is_some(), "accept 后应已预建下一实例");
            let _client = client.await.expect("客户端任务不应 panic");
        }
    }
}

pub use imp::{connect, IpcListener, IpcStream, ACCESS_MODE};
