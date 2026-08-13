//! 进程内共享的运行期状态。
//!
//! 这是唯一一处把「我是谁、我处在什么状态、我的自检报告是什么」聚在一起的地方；
//! 各端点从这里取数，不各自持有一份副本，避免就绪端点与自检端点各说各话。

use std::sync::{Arc, RwLock};

use ep_platform_obs::log::JsonLogger;
use ep_platform_obs::MetricsRegistry;

use crate::incident::IncidentNoGen;
use crate::lifecycle::{Event, IllegalTransition, Lifecycle, State};
use crate::process::{BuildInfo, ProcessKind};
use crate::selfcheck::SelfCheckReport;

pub struct SystemState {
    process: ProcessKind,
    build: BuildInfo,
    started_at: String,
    lifecycle: RwLock<Lifecycle>,
    report: SelfCheckReport,
    metrics: Arc<MetricsRegistry>,
    logger: Arc<JsonLogger>,
    incidents: IncidentNoGen,
}

impl SystemState {
    pub fn new(
        process: ProcessKind,
        build: BuildInfo,
        lifecycle: Lifecycle,
        report: SelfCheckReport,
        metrics: Arc<MetricsRegistry>,
        logger: Arc<JsonLogger>,
    ) -> Arc<Self> {
        Arc::new(Self {
            process,
            build,
            started_at: ep_platform_obs::log::now_rfc3339_micros(),
            lifecycle: RwLock::new(lifecycle),
            report,
            metrics,
            logger,
            incidents: IncidentNoGen::new(process),
        })
    }

    pub fn process(&self) -> ProcessKind {
        self.process
    }

    pub fn build(&self) -> BuildInfo {
        self.build
    }

    pub fn started_at(&self) -> &str {
        &self.started_at
    }

    pub fn report(&self) -> &SelfCheckReport {
        &self.report
    }

    pub fn metrics(&self) -> &Arc<MetricsRegistry> {
        &self.metrics
    }

    pub fn logger(&self) -> &Arc<JsonLogger> {
        &self.logger
    }

    pub fn next_incident_no(&self) -> String {
        self.incidents.next()
    }

    /// 锁中毒说明持锁线程 panic 过；状态本身是 Copy 的枚举，取回内层值继续，
    /// 不把一次 panic 放大成整个进程读不到状态。
    pub fn state(&self) -> State {
        self.lifecycle
            .read()
            .unwrap_or_else(|p| p.into_inner())
            .state()
    }

    pub fn fire(&self, event: Event) -> Result<State, IllegalTransition> {
        self.lifecycle
            .write()
            .unwrap_or_else(|p| p.into_inner())
            .fire(event)
    }

    /// 就绪端点的判据：只有 READY 与 DEGRADED 两个状态算在服务中。
    pub fn is_serving(&self) -> bool {
        matches!(self.state(), State::Ready | State::Degraded)
    }
}
