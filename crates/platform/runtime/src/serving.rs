//! 服务期编排：起若干个服务端与后台任务，等信号，收尾。
//!
//! 八个进程的形态差别只在「起哪些东西」，收尾语义必须完全一致——
//! 停机语义分散在八份 main 里，就会长出八种退出码。

use std::net::SocketAddr;
use std::process::ExitCode;
use std::sync::Arc;

use axum::Router;
use ep_platform_obs::log::{JsonLogger, Level, LogFields};
use tokio::task::JoinHandle;

use crate::boot;
use crate::http::{self, SystemState};
use crate::lifecycle::{Event, EXIT_PANIC};
use crate::shutdown::{self, Shutdown, ShutdownTrigger, StopReason};

pub struct Serving {
    trigger: ShutdownTrigger,
    signal: Shutdown,
    tasks: Vec<JoinHandle<()>>,
    failed: Option<String>,
}

impl Default for Serving {
    fn default() -> Self {
        Self::new()
    }
}

impl Serving {
    pub fn new() -> Self {
        let (trigger, signal) = shutdown::channel();
        Self { trigger, signal, tasks: Vec::new(), failed: None }
    }

    /// 停机信号的一个副本，供调用方自己写的后台循环使用。
    pub fn signal(&self) -> Shutdown {
        self.signal.clone()
    }

    /// 起一个 HTTP 服务端。绑定失败在这里就记下，`wait_and_drain` 会据此
    /// 以非零码退出——半个进程起来了却宣称就绪，比起不来更糟。
    pub async fn spawn_http(&mut self, addr: SocketAddr, router: Router, logger: &JsonLogger) {
        match http::bind(addr).await {
            Ok((listener, local)) => {
                logger.log(Level::Info, LogFields::msg("startup", format!("监听 {local}")));
                let signal = self.signal.clone();
                self.tasks.push(tokio::spawn(async move {
                    let _ = http::serve_on(listener, router, async move {
                        signal.wait().await;
                    })
                    .await;
                }));
            }
            Err(e) => {
                logger.log(Level::Error, LogFields::msg("startup", format!("{e}")));
                self.failed = Some(e.to_string());
            }
        }
    }

    pub fn spawn<F>(&mut self, task: F)
    where
        F: std::future::Future<Output = ()> + Send + 'static,
    {
        self.tasks.push(tokio::spawn(task));
    }

    pub fn mark_failed(&mut self, detail: impl Into<String>) {
        self.failed = Some(detail.into());
    }

    /// 等信号，进入 Draining，按 drain 上限收尾，退出码 0。
    pub async fn wait_and_drain(
        self,
        state: &Arc<SystemState>,
        logger: &JsonLogger,
        drain_ms: u32,
    ) -> ExitCode {
        if let Some(detail) = self.failed {
            logger.log(Level::Error, LogFields::msg("startup", format!("启动未完成：{detail}")));
            self.trigger.fire(StopReason::Internal);
            for t in self.tasks {
                let _ = t.await;
            }
            return ExitCode::from(EXIT_PANIC);
        }

        match shutdown::wait_for_signal().await {
            Ok(reason) => {
                logger.log(
                    Level::Info,
                    LogFields::msg("shutdown", format!("收到停机信号 {reason:?}，停止接收新请求")),
                );
                if let Err(e) = state.fire(Event::Sigterm) {
                    logger.log(Level::Error, LogFields::msg("lifecycle", format!("{e}")));
                }
                self.trigger.fire(reason);
            }
            Err(e) => {
                logger.log(Level::Error, LogFields::msg("shutdown", format!("信号处理器安装失败：{e}")));
                return ExitCode::from(EXIT_PANIC);
            }
        }

        let drained = tokio::time::timeout(shutdown::drain_limit(drain_ms), async {
            for t in self.tasks {
                let _ = t.await;
            }
        })
        .await
        .is_ok();

        boot::finish_draining(state, logger, drained)
    }
}
