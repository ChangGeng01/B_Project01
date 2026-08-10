//! 任务调度器骨架。本阶段已注册任务为零个。
//!
//! 不为 Outbox 消费预留任何钩子：没有消费者的钩子无从验证，只有维护成本。
//! 空转的退避从 200 毫秒起、翻倍到 2 秒封顶，为的是零任务时不空烧 CPU。

use std::sync::Arc;
use std::time::Duration;

use ep_platform_obs::log::{JsonLogger, Level, LogFields};
use ep_platform_runtime::shutdown::Shutdown;

pub const MIN_BACKOFF: Duration = Duration::from_millis(200);
pub const MAX_BACKOFF: Duration = Duration::from_secs(2);

/// 一个可调度的任务。本阶段只有名字：执行入口的形态随第一个真实任务一起定，
/// 提前定一个没有实现者的签名，改起来只会更贵。
pub trait Job: Send + Sync {
    fn name(&self) -> &'static str;
}

/// 已注册任务表。本阶段为空，注册与调度两件事都在，只是没有任务可调。
#[derive(Default)]
pub struct JobRegistry {
    jobs: Vec<Arc<dyn Job>>,
}

impl JobRegistry {
    pub fn new() -> Self {
        Self::default()
    }

    // 注册入口按阶段 1 计划第 7.7 节就位，而本阶段已注册任务为零个，
    // 因此没有调用点。用 expect 而不是 allow：第一个真实任务落地时这条
    // 标注会自己报错，提醒把它删掉，不会一直挂着。
    #[expect(dead_code)]
    pub fn register(&mut self, job: Arc<dyn Job>) {
        self.jobs.push(job);
    }

    pub fn len(&self) -> usize {
        self.jobs.len()
    }

    pub fn is_empty(&self) -> bool {
        self.jobs.is_empty()
    }

    pub fn names(&self) -> Vec<&'static str> {
        self.jobs.iter().map(|j| j.name()).collect()
    }
}

/// 下一次空转退避。取值只依赖上一次退避，便于单独判定。
pub fn next_backoff(current: Duration) -> Duration {
    let doubled = current.saturating_mul(2);
    if doubled > MAX_BACKOFF {
        MAX_BACKOFF
    } else {
        doubled
    }
}

/// 调度循环。零任务时按退避空转，收到停机信号即返回。
pub async fn run(registry: JobRegistry, signal: Shutdown, logger: Arc<JsonLogger>) {
    logger.log(
        Level::Info,
        LogFields::msg(
            "scheduler",
            format!("调度器启动，已注册任务 {} 个：{}", registry.len(), registry.names().join("、")),
        ),
    );
    let mut backoff = MIN_BACKOFF;
    let mut stop = Box::pin(signal.wait());
    loop {
        // 本阶段没有任何可执行任务，每一轮都是空转，因此退避一路涨到上限。
        let due = registry.is_empty();
        tokio::select! {
            _ = &mut stop => break,
            _ = tokio::time::sleep(backoff) => {
                backoff = if due { next_backoff(backoff) } else { MIN_BACKOFF };
            }
        }
    }
    logger.log(Level::Info, LogFields::msg("scheduler", "调度器已停止"));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn registry_starts_empty_this_stage() {
        assert_eq!(JobRegistry::new().len(), 0);
        assert!(JobRegistry::new().names().is_empty());
    }

    #[test]
    fn backoff_doubles_and_stops_at_the_cap() {
        assert_eq!(next_backoff(MIN_BACKOFF), Duration::from_millis(400));
        assert_eq!(next_backoff(Duration::from_millis(1_500)), MAX_BACKOFF);
        assert_eq!(next_backoff(MAX_BACKOFF), MAX_BACKOFF, "封顶后不再增长");
    }

    #[tokio::test]
    async fn loop_returns_on_shutdown_instead_of_spinning_forever() {
        let (trigger, signal) = ep_platform_runtime::shutdown::channel();
        let logger = Arc::new(JsonLogger::new("job-worker", "0.1.0", Level::Warn));
        let handle = tokio::spawn(run(JobRegistry::new(), signal, logger));
        trigger.fire(ep_platform_runtime::shutdown::StopReason::Sigterm);
        tokio::time::timeout(Duration::from_secs(2), handle)
            .await
            .expect("调度循环必须在停机信号后返回")
            .expect("任务不应 panic");
    }
}
