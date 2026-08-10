//! 信号处理与优雅停机。
//!
//! 一个进程里可能有多个服务端（ops-agent 有两个 HTTP 端口，core-server 另有
//! IPC 服务端），因此停机信号做成可以被多方等待的广播，而不是一个只能被
//! 消费一次的 oneshot。

use std::time::Duration;

use tokio::sync::watch;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum StopReason {
    Sigterm,
    Sigint,
    /// 进程自身决定收尾，例如 IPC 服务端不可恢复地失败。
    Internal,
}

#[derive(Clone)]
pub struct Shutdown {
    rx: watch::Receiver<Option<StopReason>>,
}

pub struct ShutdownTrigger {
    tx: watch::Sender<Option<StopReason>>,
}

impl ShutdownTrigger {
    pub fn fire(&self, reason: StopReason) {
        // 接收端全部退出时发送失败，此时无人需要被通知，忽略即可。
        let _ = self.tx.send(Some(reason));
    }
}

pub fn channel() -> (ShutdownTrigger, Shutdown) {
    let (tx, rx) = watch::channel(None);
    (ShutdownTrigger { tx }, Shutdown { rx })
}

impl Shutdown {
    /// 等到停机信号。已经触发过则立即返回。
    pub async fn wait(mut self) -> StopReason {
        if let Some(r) = *self.rx.borrow_and_update() {
            return r;
        }
        loop {
            if self.rx.changed().await.is_err() {
                // 发送端已丢弃：按内部收尾处理，不吊死在这里。
                return StopReason::Internal;
            }
            if let Some(r) = *self.rx.borrow_and_update() {
                return r;
            }
        }
    }
}

/// 监听 SIGTERM 与 SIGINT。安装失败是致命的：装不上信号处理器的进程
/// 无法优雅停机，必须让调用方知道，而不是静默地永远等下去。
pub async fn wait_for_signal() -> Result<StopReason, std::io::Error> {
    use tokio::signal::unix::{signal, SignalKind};
    let mut term = signal(SignalKind::terminate())?;
    let mut int = signal(SignalKind::interrupt())?;
    tokio::select! {
        _ = term.recv() => Ok(StopReason::Sigterm),
        _ = int.recv() => Ok(StopReason::Sigint),
    }
}

/// drain 上限。超时后强制关闭并记 WARN，退出码仍为 0。
pub fn drain_limit(shutdown_drain_ms: u32) -> Duration {
    Duration::from_millis(u64::from(shutdown_drain_ms))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn every_waiter_sees_the_signal() {
        let (trigger, shutdown) = channel();
        let a = tokio::spawn(shutdown.clone().wait());
        let b = tokio::spawn(shutdown.clone().wait());
        trigger.fire(StopReason::Sigterm);
        assert_eq!(a.await.unwrap(), StopReason::Sigterm);
        assert_eq!(b.await.unwrap(), StopReason::Sigterm);
    }

    #[tokio::test]
    async fn waiting_after_the_signal_returns_immediately() {
        let (trigger, shutdown) = channel();
        trigger.fire(StopReason::Sigint);
        assert_eq!(shutdown.wait().await, StopReason::Sigint);
    }

    // 负样例断言的是「不得吊死」这条规则本身：触发端消失时等待方必须返回。
    #[tokio::test]
    async fn dropping_the_trigger_does_not_hang_the_waiter() {
        let (trigger, shutdown) = channel();
        drop(trigger);
        assert_eq!(shutdown.wait().await, StopReason::Internal);
    }

    #[test]
    fn drain_limit_is_taken_from_config_not_hardcoded() {
        assert_eq!(drain_limit(30_000), Duration::from_secs(30));
        assert_eq!(drain_limit(1), Duration::from_millis(1));
    }
}
