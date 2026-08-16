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
//! Windows 一侧的代码在本机（macOS）不参与编译，**未在目标平台跑过**，
//! 按裁定 F-08 的纪律如实记明：它是按接口面写的，不是实测过的。

use std::path::{Path, PathBuf};

/// 传输层错误。与 `ServerError` 分开：这一层只知道「连不上」「建不出来」，
/// 不知道任何协议语义。
#[derive(Debug)]
pub enum TransportError {
    /// 建立监听失败。Windows 侧名字被别人占住也落这一档——
    /// 那是 fail-closed，不是可重试的错误。
    Listen { path: PathBuf, detail: String },
    /// 设置访问控制失败。Unix 是权限位，Windows 是安全描述符。
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
    use tokio::net::windows::named_pipe::{ClientOptions, NamedPipeServer, ServerOptions};

    /// Unix 侧的 0660 在本平台没有对应物，访问控制由安全描述符承担。
    /// 保留同名常量只为让协议面不必分叉；**它不表达任何本平台语义**。
    pub const ACCESS_MODE: u32 = 0;

    pub type IpcStream = NamedPipeServer;

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

        pub async fn accept(&mut self) -> std::io::Result<IpcStream> {
            let server = self
                .pending
                .take()
                .ok_or_else(|| std::io::Error::other("命名管道实例缺席，accept 已不可用"))?;
            server.connect().await?;
            // 先把下一个实例建好再交出当前实例；建不出来就把当前实例还回去，
            // 让下一次 accept 重试，而不是把已连上的这一条也丢掉。
            match ServerOptions::new()
                .reject_remote_clients(true)
                .create(&self.name)
            {
                Ok(next) => {
                    self.pending = Some(next);
                    Ok(server)
                }
                Err(e) => Err(e),
            }
        }

        /// 本平台无需清理：管道实例随最后一个句柄由内核回收，不存在残留文件。
        /// 保留空实现是为了让协议面不必分叉。
        pub fn cleanup(&self) {
            let _ = &self.path;
        }
    }

    pub async fn connect(path: &Path) -> Result<IpcStream, TransportError> {
        // 本平台必须处理 ERROR_PIPE_BUSY：服务端在但当前没有空闲实例时返回该码。
        // 不重试会把「core 在但忙」误报成「core 不可用」并落 spool，
        // 那是一条会让人查错方向的假象。
        const ERROR_PIPE_BUSY: i32 = 231;
        const MAX_ATTEMPTS: u32 = 8;
        let name = path.to_string_lossy().into_owned();
        let mut attempt = 0u32;
        loop {
            match ClientOptions::new().open(&name) {
                Ok(c) => return Ok(c),
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
}

pub use imp::{connect, IpcListener, IpcStream, ACCESS_MODE};
