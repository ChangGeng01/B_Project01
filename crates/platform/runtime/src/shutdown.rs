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

/// 等一个外部停机请求。安装失败是致命的：装不上处理器的进程无法优雅停机，
/// 必须让调用方知道，而不是静默地永远等下去。
///
/// 平台分叉按裁定 F-08 第 4.2 节配套第 1 条。**这是平台分叉，不是双平台维护**
/// （F-09-2 第三条：零 Linux 开发的效力范围）——Unix 分支保留其现有实现即可，
/// 不为它新增测试、不为它跑 CI、不因它阻塞任何 Windows 侧改动。
///
/// 两侧返回的 [`StopReason`] 取值刻意保持不变：`Sigterm` 与 `Sigint` 两个名字
/// 在本平台已无信号可指，但它们是**状态机的取值**，改名会连带改
/// `lifecycle.rs` 的状态机与其全部用例，而语义（外部要求停机／用户中断）两平台一致。
/// 裁定 F-08 第八节把该改名单列为「纯改名」一项，不在本次传输边界抽出的范围内。
#[cfg(unix)]
pub async fn wait_for_signal() -> Result<StopReason, std::io::Error> {
    use tokio::signal::unix::{signal, SignalKind};
    let mut term = signal(SignalKind::terminate())?;
    let mut int = signal(SignalKind::interrupt())?;
    tokio::select! {
        _ = term.recv() => Ok(StopReason::Sigterm),
        _ = int.recv() => Ok(StopReason::Sigint),
    }
}

/// Windows 侧：本平台没有 SIGTERM 与 SIGINT。
///
/// 本函数只承接**控制台直跑模式**——按 F-08 第 4.2 节配套第 1 条，
/// 该模式为开发与集成测试保留。服务模式下的停机请求走另一条路：
/// 由服务宿主层的 `HandlerEx` 收到服务控制管理器的停止控制码后，
/// 经 [`ShutdownTx`] 投递，**不经过本函数**。
///
/// 两个事件的对应关系逐条记明，不含糊：
/// `ctrl_c` 与 `ctrl_break` 都表示用户在控制台中断，映射到 [`StopReason::Sigint`]；
/// `ctrl_close`（控制台窗口被关闭）与 `ctrl_shutdown`（系统关机）表示外部要求停机，
/// 映射到 [`StopReason::Sigterm`]。
///
/// **一处如实登记**：`ctrl_close` 与 `ctrl_shutdown` 之后系统给的排空时间由操作系统决定，
/// 远短于配置的 30 秒——这正是 F-08 做不到四登记的那条降级，不在本函数内可解。
///
/// 本分支**未在目标平台跑过**，按 F-08 的纪律记明：它是按接口面写的，不是实测过的。
#[cfg(windows)]
pub async fn wait_for_signal() -> Result<StopReason, std::io::Error> {
    use tokio::signal::windows;
    let mut ctrl_c = windows::ctrl_c()?;
    let mut ctrl_break = windows::ctrl_break()?;
    let mut ctrl_close = windows::ctrl_close()?;
    let mut ctrl_shutdown = windows::ctrl_shutdown()?;
    tokio::select! {
        _ = ctrl_c.recv() => Ok(StopReason::Sigint),
        _ = ctrl_break.recv() => Ok(StopReason::Sigint),
        _ = ctrl_close.recv() => Ok(StopReason::Sigterm),
        _ = ctrl_shutdown.recv() => Ok(StopReason::Sigterm),
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
