//! 服务期编排：起若干个服务端与后台任务，等信号，收尾。
//!
//! 八个进程的形态差别只在「起哪些东西」，收尾语义必须完全一致——
//! 停机语义分散在八份 main 里，就会长出八种退出码。

use std::future::poll_fn;
use std::net::SocketAddr;
use std::panic::AssertUnwindSafe;
use std::process::ExitCode;
use std::sync::{Arc, Mutex};
use std::task::Poll;

use axum::Router;
use ep_platform_obs::log::{JsonLogger, Level, LogFields};
use tokio::sync::oneshot;
use tokio::task::JoinSet;

use crate::boot;
use crate::http::{self, SystemState};
use crate::lifecycle::{Event, EXIT_PANIC};
use crate::shutdown::{self, Shutdown, ShutdownTrigger, StopReason};

pub struct Serving {
    trigger: ShutdownTrigger,
    signal: Shutdown,
    tasks: JoinSet<()>,
    initial_polls: Vec<oneshot::Receiver<bool>>,
    failed: Option<String>,
    critical_failure: Arc<Mutex<Option<String>>>,
}

impl Default for Serving {
    fn default() -> Self {
        Self::new()
    }
}

impl Serving {
    pub fn new() -> Self {
        let (trigger, signal) = shutdown::channel();
        Self {
            trigger,
            signal,
            tasks: JoinSet::new(),
            initial_polls: Vec::new(),
            failed: None,
            critical_failure: Arc::new(Mutex::new(None)),
        }
    }

    /// 停机信号的一个副本，供调用方自己写的后台循环使用。
    pub fn signal(&self) -> Shutdown {
        self.signal.clone()
    }

    /// 监听者在排空连接前报告不可恢复故障，立即启动进程的有界排空。
    pub fn critical_failure_handler(
        &self,
        name: impl Into<String>,
    ) -> impl FnOnce(String) + Send + 'static {
        let name = name.into();
        let failure = self.critical_failure.clone();
        let trigger = self.trigger.clone();
        move |detail| {
            let mut first_failure = failure
                .lock()
                .unwrap_or_else(|poisoned| poisoned.into_inner());
            if first_failure.is_none() {
                *first_failure = Some(format!("关键任务 {name} 失败：{detail}"));
            }
            drop(first_failure);
            trigger.fire(StopReason::Internal);
        }
    }

    /// 起一个 HTTP 服务端。绑定失败在这里就记下，`wait_and_drain` 会据此
    /// 以非零码退出——半个进程起来了却宣称就绪，比起不来更糟。
    pub async fn spawn_http(&mut self, addr: SocketAddr, router: Router, logger: &JsonLogger) {
        match http::bind(addr).await {
            Ok((listener, local)) => {
                logger.log(
                    Level::Info,
                    LogFields::msg("startup", format!("监听 {local}")),
                );
                let signal = self.signal.clone();
                self.spawn_critical(format!("HTTP 服务端 {local}"), async move {
                    let _ = http::serve_on(listener, router, async move {
                        signal.wait().await;
                    })
                    .await;
                });
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
        self.tasks.spawn(task);
    }

    /// 起一个决定进程存活性的关键任务。它在全局停机信号之前结束时，
    /// 立即触发内部停机；HTTP/IPC 监听者不能悄悄消失后仍让进程宣称存活。
    pub fn spawn_critical<F>(&mut self, name: impl Into<String>, task: F)
    where
        F: std::future::Future<Output = ()> + Send + 'static,
    {
        let trigger = self.trigger.clone();
        let name = name.into();
        let failure = self.critical_failure.clone();
        let signal = self.signal.clone();
        let (ack, initial_poll) = oneshot::channel();
        self.initial_polls.push(initial_poll);
        self.tasks.spawn(async move {
            let mut ack = Some(ack);
            tokio::pin!(task);
            // 确认来自关键 future 本身的首轮 Pending，而非 wrapper 被调度。
            // 单层任务持有 future，避免超时取消 wrapper 后留下 detached 子任务。
            let outcome = poll_fn(|cx| {
                match std::panic::catch_unwind(AssertUnwindSafe(|| task.as_mut().poll(cx))) {
                    Ok(Poll::Pending) => {
                        if let Some(ack) = ack.take() {
                            let _ = ack.send(true);
                        }
                        Poll::Pending
                    }
                    Ok(Poll::Ready(())) => Poll::Ready(Ok(())),
                    Err(_) => Poll::Ready(Err(())),
                }
            })
            .await;
            if outcome.is_ok() && signal.is_requested() {
                if let Some(ack) = ack.take() {
                    let _ = ack.send(false);
                }
                return;
            }
            let detail = match outcome {
                Ok(()) => format!("关键任务 {name} 提前结束"),
                Err(()) => format!("关键任务 {name} panic"),
            };
            let mut first_failure = failure
                .lock()
                .unwrap_or_else(|poisoned| poisoned.into_inner());
            if first_failure.is_none() {
                *first_failure = Some(detail);
            }
            drop(first_failure);
            trigger.fire(StopReason::Internal);
            if let Some(ack) = ack.take() {
                let _ = ack.send(false);
            }
        });
    }

    pub fn mark_failed(&mut self, detail: impl Into<String>) {
        self.failed = Some(detail.into());
    }

    /// 等所有关键 future 首轮轮询确认 Pending，且所有同步启动步骤成功，
    /// 调用方才可以写出“已就绪”。这不是后续永久健康保证，后续失败仍触发停机。
    /// `wait_and_drain` 会处理失败和收尾；这个判据专门防止绑定/目录探测失败后仍留下
    /// 一条误导运维与自动化的成功日志。
    pub async fn startup_succeeded(&mut self) -> bool {
        while let Some(initial_poll) = self.initial_polls.last_mut() {
            let acknowledged = initial_poll.await.unwrap_or(false);
            self.initial_polls.pop();
            if !acknowledged && self.failed.is_none() {
                self.failed = Some("关键任务未确认进入运行状态".into());
            }
        }
        self.failed.is_none()
            && self
                .critical_failure
                .lock()
                .unwrap_or_else(|poisoned| poisoned.into_inner())
                .is_none()
    }

    /// 等信号，进入 Draining，按 drain 上限收尾，退出码 0。
    pub async fn wait_and_drain(
        mut self,
        state: &Arc<SystemState>,
        logger: &JsonLogger,
        drain_ms: u32,
    ) -> ExitCode {
        let startup_failure = self.failed.clone();
        if let Some(detail) = &startup_failure {
            logger.log(
                Level::Error,
                LogFields::msg("startup", format!("启动未完成：{detail}")),
            );
            self.trigger.fire(StopReason::Internal);
        }

        let mut signal_error = None;
        let reason = if startup_failure.is_some() {
            StopReason::Internal
        } else {
            let internal = self.signal.clone();
            tokio::select! {
                external = shutdown::wait_for_signal() => match external {
                    Ok(reason) => reason,
                    Err(error) => {
                        signal_error = Some(error.to_string());
                        StopReason::Internal
                    }
                },
                internal = internal.wait() => internal,
            }
        };
        let abnormal = reason == StopReason::Internal;
        if let Some(error) = signal_error {
            logger.log(
                Level::Error,
                LogFields::msg("shutdown", format!("信号处理器安装失败：{error}")),
            );
        } else if abnormal {
            let detail = startup_failure.unwrap_or_else(|| {
                self.critical_failure
                    .lock()
                    .unwrap_or_else(|poisoned| poisoned.into_inner())
                    .clone()
                    .unwrap_or_else(|| "关键服务任务提前结束".to_string())
            });
            logger.log(
                Level::Error,
                LogFields::msg("shutdown", format!("{detail}，执行内部停机")),
            );
        } else {
            logger.log(
                Level::Info,
                LogFields::msg(
                    "shutdown",
                    format!("收到停机信号 {reason:?}，停止接收新请求"),
                ),
            );
            if let Err(e) = state.fire(Event::Sigterm) {
                logger.log(Level::Error, LogFields::msg("lifecycle", format!("{e}")));
            }
        }
        if abnormal {
            // 内部故障同样先进入 Draining，不能因要返回非零就跳过资源收尾。
            if let Err(e) = state.fire(Event::Sigterm) {
                logger.log(Level::Error, LogFields::msg("lifecycle", format!("{e}")));
            }
        }
        self.trigger.fire(reason);

        let drained = tokio::time::timeout(shutdown::drain_limit(drain_ms), async {
            while self.tasks.join_next().await.is_some() {}
        })
        .await
        .is_ok();

        if !drained {
            self.tasks.abort_all();
            while self.tasks.join_next().await.is_some() {}
        }

        let drained_code = boot::finish_draining(state, logger, drained);
        if abnormal {
            ExitCode::from(EXIT_PANIC)
        } else {
            drained_code
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test(flavor = "current_thread")]
    async fn instant_critical_completion_cannot_announce_ready() {
        let mut serving = Serving::new();
        serving.spawn_critical("instant", async {});
        assert!(
            !serving.startup_succeeded().await,
            "未轮询关键任务前不得宣称就绪"
        );
    }

    #[tokio::test(flavor = "current_thread")]
    async fn instant_critical_panic_cannot_announce_ready() {
        let mut serving = Serving::new();
        serving.spawn_critical("panic", async { panic!("first poll panic") });
        assert!(
            !serving.startup_succeeded().await,
            "首轮 panic 必须阻止就绪"
        );
    }

    #[tokio::test(flavor = "current_thread")]
    async fn readiness_waits_for_every_inner_future_to_be_polled() {
        use std::sync::atomic::{AtomicUsize, Ordering};
        let polled = Arc::new(AtomicUsize::new(0));
        let mut serving = Serving::new();
        for name in ["first", "second", "third"] {
            let polled = polled.clone();
            serving.spawn_critical(name, async move {
                polled.fetch_add(1, Ordering::SeqCst);
                std::future::pending::<()>().await;
            });
        }
        assert!(serving.startup_succeeded().await);
        assert_eq!(
            polled.load(Ordering::SeqCst),
            3,
            "wrapper 被调度不等于服务 future 已轮询"
        );
    }

    #[tokio::test(flavor = "current_thread")]
    async fn cancelled_readiness_wait_cannot_discard_initial_poll_barrier() {
        use std::future::Future;
        use std::sync::atomic::{AtomicBool, Ordering};
        let polled = Arc::new(AtomicBool::new(false));
        let observed = polled.clone();
        let mut serving = Serving::new();
        serving.spawn_critical("listener", async move {
            observed.store(true, Ordering::SeqCst);
            std::future::pending::<()>().await;
        });
        let mut first_wait = Box::pin(serving.startup_succeeded());
        let mut cx = std::task::Context::from_waker(std::task::Waker::noop());
        assert!(first_wait.as_mut().poll(&mut cx).is_pending());
        drop(first_wait);
        assert!(serving.startup_succeeded().await);
        assert!(
            polled.load(Ordering::SeqCst),
            "取消一次等待不得抹掉尚未确认的关键任务"
        );
    }

    #[tokio::test(flavor = "current_thread")]
    async fn drain_timeout_aborts_critical_future_instead_of_detaching_it() {
        use crate::lifecycle::Lifecycle;
        use crate::selfcheck::{Outcome, SelfCheckReport};
        use crate::{BuildInfo, ProcessKind};
        struct OnDrop(Option<tokio::sync::oneshot::Sender<()>>);
        impl Drop for OnDrop {
            fn drop(&mut self) {
                if let Some(tx) = self.0.take() {
                    let _ = tx.send(());
                }
            }
        }
        let logger = Arc::new(JsonLogger::new("core-server", "test", Level::Info));
        let mut lifecycle = Lifecycle::new(ProcessKind::CoreServer);
        for event in [Event::Start, Event::ConfigLoaded, Event::AllPassed] {
            lifecycle.fire(event).unwrap();
        }
        let state = SystemState::new(
            ProcessKind::CoreServer,
            BuildInfo::current(),
            lifecycle,
            SelfCheckReport {
                process: "core-server",
                version: "test".into(),
                items: vec![],
                overall: Outcome::Passed,
            },
            Arc::new(ep_platform_obs::MetricsRegistry::new()),
            logger.clone(),
        );
        let mut serving = Serving::new();
        let (entered_tx, entered_rx) = tokio::sync::oneshot::channel();
        let (dropped_tx, dropped_rx) = tokio::sync::oneshot::channel();
        serving.spawn_critical("stuck", async move {
            let _drop = OnDrop(Some(dropped_tx));
            entered_tx.send(()).unwrap();
            std::future::pending::<()>().await;
        });
        entered_rx.await.unwrap();
        serving.trigger.fire(StopReason::Sigterm);
        let started = std::time::Instant::now();
        assert_eq!(
            serving.wait_and_drain(&state, &logger, 30).await,
            ExitCode::SUCCESS
        );
        assert!(
            started.elapsed() >= std::time::Duration::from_millis(30),
            "不能在排空期限前强行取消"
        );
        assert!(
            tokio::time::timeout(std::time::Duration::from_millis(100), dropped_rx)
                .await
                .is_ok(),
            "排空超时必须取消关键 future，不能遗留 detached 子任务"
        );
    }

    #[tokio::test]
    async fn critical_task_completion_requests_internal_shutdown() {
        let mut serving = Serving::new();
        let signal = serving.signal();
        serving.spawn_critical("test-listener", async {});

        let reason = tokio::time::timeout(std::time::Duration::from_secs(1), signal.wait())
            .await
            .expect("关键任务结束必须唤醒进程");
        assert_eq!(reason, StopReason::Internal);
    }

    #[tokio::test]
    async fn critical_task_panic_requests_internal_shutdown() {
        let mut serving = Serving::new();
        let signal = serving.signal();
        serving.spawn_critical("panicking-listener", async {
            panic!("boom");
        });

        let reason = tokio::time::timeout(std::time::Duration::from_secs(1), signal.wait())
            .await
            .expect("关键任务 panic 必须唤醒进程");
        assert_eq!(reason, StopReason::Internal);
        let detail = serving
            .critical_failure
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner())
            .clone()
            .expect("必须保存首个关键故障");
        assert!(detail.contains("panic"), "unexpected detail: {detail}");
    }

    #[tokio::test]
    async fn ordinary_background_completion_is_not_a_process_failure() {
        let mut serving = Serving::new();
        let signal = serving.signal();
        serving.spawn(async {});

        assert!(
            tokio::time::timeout(std::time::Duration::from_millis(50), signal.wait())
                .await
                .is_err(),
            "普通后台任务完成不得伪装成关键服务故障"
        );
    }

    #[tokio::test]
    async fn normal_critical_shutdown_keeps_original_reason_without_failure() {
        let mut serving = Serving::new();
        let signal = serving.signal();
        serving.spawn_critical("listener", async move {
            signal.wait().await;
        });
        assert!(serving.startup_succeeded().await);
        serving.trigger.fire(StopReason::Sigterm);
        while serving.tasks.join_next().await.is_some() {}
        assert_eq!(serving.signal().wait().await, StopReason::Sigterm);
        assert!(serving.critical_failure.lock().unwrap().is_none());
    }

    #[tokio::test]
    async fn listener_failure_report_starts_shutdown_before_task_finishes() {
        use crate::lifecycle::Lifecycle;
        use crate::selfcheck::{Outcome, SelfCheckReport};
        use crate::{BuildInfo, ProcessKind};
        struct OnDrop(Option<oneshot::Sender<()>>);
        impl Drop for OnDrop {
            fn drop(&mut self) {
                if let Some(tx) = self.0.take() {
                    let _ = tx.send(());
                }
            }
        }
        let logger = Arc::new(JsonLogger::new("core-server", "test", Level::Info));
        let mut lifecycle = Lifecycle::new(ProcessKind::CoreServer);
        for event in [Event::Start, Event::ConfigLoaded, Event::AllPassed] {
            lifecycle.fire(event).unwrap();
        }
        let state = SystemState::new(
            ProcessKind::CoreServer,
            BuildInfo::current(),
            lifecycle,
            SelfCheckReport {
                process: "core-server",
                version: "test".into(),
                items: vec![],
                overall: Outcome::Passed,
            },
            Arc::new(ep_platform_obs::MetricsRegistry::new()),
            logger.clone(),
        );
        let mut serving = Serving::new();
        let report_failure = serving.critical_failure_handler("IPC listener");
        let (dropped_tx, mut dropped_rx) = oneshot::channel();
        serving.spawn_critical("IPC listener", async move {
            let _guard = OnDrop(Some(dropped_tx));
            report_failure("accept failed".into());
            std::future::pending::<()>().await;
        });
        let reason = tokio::time::timeout(
            std::time::Duration::from_millis(100),
            serving.signal().wait(),
        )
        .await
        .unwrap();
        assert_eq!(reason, StopReason::Internal);
        assert!(!serving.startup_succeeded().await);
        assert!(serving
            .critical_failure
            .lock()
            .unwrap()
            .as_ref()
            .unwrap()
            .contains("accept failed"));
        assert!(matches!(
            dropped_rx.try_recv(),
            Err(oneshot::error::TryRecvError::Empty)
        ));
        let start = std::time::Instant::now();
        assert_eq!(
            serving.wait_and_drain(&state, &logger, 30).await,
            ExitCode::from(EXIT_PANIC)
        );
        assert!(start.elapsed() >= std::time::Duration::from_millis(30));
        dropped_rx
            .await
            .expect("内部监听失败仍须在排空期限结束后取消在途任务");
    }

    #[tokio::test]
    async fn a_synchronous_startup_failure_suppresses_ready_announcement() {
        let mut serving = Serving::new();
        assert!(serving.startup_succeeded().await);
        serving.mark_failed("bind failed");
        assert!(
            !serving.startup_succeeded().await,
            "同步启动失败后不得再写出已就绪日志"
        );
    }
}
