//! 任务调度器骨架。阶段 4 任务 #21 接入首批两个真实任务（身份
//! 卫生与应急维护，见 crate::jobs）。
//!
//! 不为 Outbox 消费预留任何钩子：没有消费者的钩子无从验证，只有维护成本。
//! 空转的退避从 200 毫秒起、翻倍到 2 秒封顶，为的是零任务时不空烧 CPU。

use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use std::time::{Duration, Instant};

use ep_platform_obs::log::{JsonLogger, Level, LogFields};
use ep_platform_runtime::shutdown::Shutdown;

pub const MIN_BACKOFF: Duration = Duration::from_millis(200);
pub const MAX_BACKOFF: Duration = Duration::from_secs(2);

/// 一次任务执行的结果：受影响行数或失败原因。
pub type JobOutcome = Result<u64, String>;

/// 一个可调度的任务：名字、周期与执行入口三件事随首个真实任务定型。
/// 执行体串行调度（同一任务不会自重叠），失败只记警不中断调度。
pub trait Job: Send + Sync {
    fn name(&self) -> &'static str;
    /// 两次执行的间隔；到期判定按调度循环的节拍累计。
    fn interval(&self) -> Duration;
    fn run(&self) -> Pin<Box<dyn Future<Output = JobOutcome> + Send + '_>>;
}

/// 已注册任务表。
#[derive(Default)]
pub struct JobRegistry {
    jobs: Vec<Arc<dyn Job>>,
}

impl JobRegistry {
    pub fn new() -> Self {
        Self::default()
    }

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

/// 执行到期任务一轮；返回本轮实际执行的任务数。
async fn run_due(registry: &JobRegistry, next_run: &mut [Instant], logger: &JsonLogger) -> usize {
    let now = Instant::now();
    let mut ran = 0usize;
    for (idx, job) in registry.jobs.iter().enumerate() {
        if now < next_run[idx] {
            continue;
        }
        next_run[idx] = now + job.interval();
        ran += 1;
        match job.run().await {
            Ok(affected) => logger.log(
                Level::Info,
                LogFields::msg(
                    "scheduler",
                    format!("任务 {} 完成，影响 {} 行", job.name(), affected),
                ),
            ),
            Err(reason) => logger.log(
                Level::Warn,
                LogFields::msg(
                    "scheduler",
                    format!("任务 {} 失败：{reason}，下一周期重试", job.name()),
                ),
            ),
        }
    }
    ran
}

/// 调度循环。零任务时按退避空转；有任务时按最小节拍轮询到期项，
/// 收到停机信号即返回。
pub async fn run(registry: JobRegistry, signal: Shutdown, logger: Arc<JsonLogger>) {
    logger.log(
        Level::Info,
        LogFields::msg(
            "scheduler",
            format!(
                "调度器启动，已注册任务 {} 个：{}",
                registry.len(),
                registry.names().join("、")
            ),
        ),
    );
    let mut next_run: Vec<Instant> = registry.jobs.iter().map(|_| Instant::now()).collect();
    let mut backoff = MIN_BACKOFF;
    let mut stop = Box::pin(signal.wait());
    loop {
        // 零任务时每一轮都是空转，退避一路上涨到上限；有任务时保持最小节拍。
        let due = registry.is_empty();
        tokio::select! {
            _ = &mut stop => break,
            _ = tokio::time::sleep(backoff) => {
                if due {
                    backoff = next_backoff(backoff);
                } else {
                    let _ = run_due(&registry, &mut next_run, &logger).await;
                    backoff = MIN_BACKOFF;
                }
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
