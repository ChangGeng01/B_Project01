//! AdmissionGate：会话并发准入。
//!
//! 信号量上限 EP__ADMISSION__MAX_CONCURRENT_USERS（默认 20），排队上限
//! EP__ADMISSION__QUEUE_MAX_LEN（默认 40），等待上限
//! EP__ADMISSION__QUEUE_WAIT_TIMEOUT_SECONDS（默认 10s）；超限一律 503
//! PLATFORM.CAPACITY.CONCURRENCY_LIMIT（阶段 1 已登记，不重复登记）。
//! 活跃用户 = EP__ADMISSION__ACTIVE_WINDOW_SECONDS（默认 60s）内
//! 有请求的不同 user_id。

use std::collections::HashMap;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use ep_foundation::error::codes::PLATFORM_CAPACITY_CONCURRENCY_LIMIT;
use ep_foundation::error::AppError;
use ep_foundation::security::SecurityContext;

use crate::metrics::AuthzMetricsSink;

/// 准入配置，四个 EP__ADMISSION__* 键的承载。
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct AdmissionConfig {
    /// 同时在途的会话上限。
    pub max_concurrent_users: usize,
    /// 等待队列长度上限，超限即拒。
    pub queue_max_len: usize,
    /// 排队等待上限，超时即拒。
    pub queue_wait_timeout: Duration,
    /// 活跃用户统计窗口。
    pub active_window: Duration,
}

impl Default for AdmissionConfig {
    fn default() -> Self {
        Self {
            max_concurrent_users: 20,
            queue_max_len: 40,
            queue_wait_timeout: Duration::from_secs(10),
            active_window: Duration::from_secs(60),
        }
    }
}

/// 准入凭证：持有即占用一个并发席位，落释即归还。
pub struct AdmissionPermit {
    _permit: tokio::sync::OwnedSemaphorePermit,
}

impl core::fmt::Debug for AdmissionPermit {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str("AdmissionPermit")
    }
}

/// 会话并发准入闸门。
pub struct AdmissionGate {
    semaphore: Arc<tokio::sync::Semaphore>,
    waiting: AtomicUsize,
    config: AdmissionConfig,
    /// user_id → 最近请求时刻；窗口外条目惰性清理。
    active: Mutex<HashMap<uuid::Uuid, Instant>>,
    metrics: Arc<dyn AuthzMetricsSink>,
}

impl AdmissionGate {
    pub fn new(config: AdmissionConfig, metrics: Arc<dyn AuthzMetricsSink>) -> Self {
        Self {
            semaphore: Arc::new(tokio::sync::Semaphore::new(config.max_concurrent_users)),
            waiting: AtomicUsize::new(0),
            config,
            active: Mutex::new(HashMap::new()),
            metrics,
        }
    }

    pub fn config(&self) -> &AdmissionConfig {
        &self.config
    }

    /// 准入：登记活跃用户后经信号量取席位；队列满或等待超时拒绝。
    pub async fn admit(&self, ctx: &SecurityContext) -> Result<AdmissionPermit, AppError> {
        self.mark_active(ctx);
        if self.waiting.load(Ordering::Relaxed) >= self.config.queue_max_len {
            self.metrics.observe_admission(0.0, false, "queue_full");
            return Err(limit_error("等待队列已满"));
        }
        self.waiting.fetch_add(1, Ordering::Relaxed);
        let started = Instant::now();
        let acquired = tokio::time::timeout(
            self.config.queue_wait_timeout,
            self.semaphore.clone().acquire_owned(),
        )
        .await;
        self.waiting.fetch_sub(1, Ordering::Relaxed);
        let waited = started.elapsed().as_secs_f64();
        match acquired {
            Ok(Ok(permit)) => {
                self.metrics.observe_admission(waited, true, "admitted");
                Ok(AdmissionPermit { _permit: permit })
            }
            Ok(Err(_)) => {
                // 信号量被 close：视同容量耗尽。
                self.metrics.observe_admission(waited, false, "closed");
                Err(limit_error("准入信号量已关闭"))
            }
            Err(_) => {
                self.metrics
                    .observe_admission(waited, false, "wait_timeout");
                Err(limit_error("排队等待超时"))
            }
        }
    }

    /// 活跃用户数：统计窗口内出现过请求的不同 user_id。
    pub fn active_user_count(&self) -> usize {
        let mut map = self.active.lock().unwrap_or_else(|p| p.into_inner());
        prune(&mut map, self.config.active_window);
        map.len()
    }

    fn mark_active(&self, ctx: &SecurityContext) {
        let mut map = self.active.lock().unwrap_or_else(|p| p.into_inner());
        prune(&mut map, self.config.active_window);
        map.insert(ctx.user_id.as_uuid(), Instant::now());
    }
}

fn prune(map: &mut HashMap<uuid::Uuid, Instant>, window: Duration) {
    let now = Instant::now();
    map.retain(|_, at| now.duration_since(*at) <= window);
}

fn limit_error(detail: &str) -> AppError {
    AppError::new(
        PLATFORM_CAPACITY_CONCURRENCY_LIMIT,
        format!("并发准入受限：{detail}"),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::metrics::SilentMetricsSink;
    use crate::types::tests::ctx_with;
    use ep_foundation::security::context::ClientKind;

    fn gate(concurrent: usize, queue: usize, wait_ms: u64) -> AdmissionGate {
        AdmissionGate::new(
            AdmissionConfig {
                max_concurrent_users: concurrent,
                queue_max_len: queue,
                queue_wait_timeout: Duration::from_millis(wait_ms),
                active_window: Duration::from_secs(60),
            },
            Arc::new(SilentMetricsSink),
        )
    }

    #[tokio::test]
    async fn permits_beyond_capacity_queue_then_timeout_rejects() {
        let g = Arc::new(gate(1, 40, 50));
        let ctx = ctx_with(vec!["SALES"], ClientKind::Win);
        let _p1 = g.admit(&ctx).await.expect("第一席位");
        let g2 = g.clone();
        let held = tokio::spawn(async move {
            let ctx2 = ctx_with(vec!["SALES"], ClientKind::Win);
            g2.admit(&ctx2).await
        });
        // 等待排队的任务入队后，持有席位不放，等待超时被拒。
        let err = held.await.expect("任务完成").expect_err("超时拒");
        assert_eq!(err.code, PLATFORM_CAPACITY_CONCURRENCY_LIMIT);
        drop(_p1);
        let _p2 = g.admit(&ctx).await.expect("释放后可再入");
    }

    #[tokio::test]
    async fn queue_full_rejects_immediately() {
        let g = Arc::new(gate(1, 1, 500));
        let ctx = ctx_with(vec!["SALES"], ClientKind::Win);
        let _p1 = g.admit(&ctx).await.expect("第一席位");
        let g2 = g.clone();
        let waiter = tokio::spawn(async move {
            let ctx2 = ctx_with(vec!["SALES"], ClientKind::Win);
            g2.admit(&ctx2).await
        });
        tokio::time::sleep(Duration::from_millis(20)).await;
        // 队列上限 1 且已有排队者：直接拒。
        let err = g.admit(&ctx).await.expect_err("队列满拒");
        assert_eq!(err.code, PLATFORM_CAPACITY_CONCURRENCY_LIMIT);
        drop(_p1);
        assert!(waiter.await.expect("任务完成").is_ok());
    }

    #[tokio::test]
    async fn active_users_count_distinct_within_window() {
        let g = gate(20, 40, 50);
        let ctx = ctx_with(vec!["SALES"], ClientKind::Win);
        let p = g.admit(&ctx).await.expect("准入");
        assert_eq!(g.active_user_count(), 1);
        drop(p);
        let p2 = g.admit(&ctx).await.expect("再次准入");
        assert_eq!(g.active_user_count(), 1, "同一 user_id 只计一次");
        drop(p2);
        assert_eq!(g.config().max_concurrent_users, 20);
    }
}
