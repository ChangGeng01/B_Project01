//! job-worker — 8081 健康与指标、任务调度器骨架与零个已注册任务、
//! 200 毫秒到 2 秒的退避轮询空转、优雅停机。
//!
//! 本阶段不消费 Outbox、不投递通知、不做对账。

mod config;
mod scheduler;
mod wiring;

use std::process::ExitCode;
use std::sync::Arc;

use axum::middleware::from_fn_with_state;
use ep_platform_obs::log::{JsonLogger, Level, LogFields};
use ep_platform_runtime::boot;
use ep_platform_runtime::http::middleware::{catch_panic, observe};
use ep_platform_runtime::http::{minimal_router, SystemState};
use ep_platform_runtime::lifecycle::{Lifecycle, EXIT_PANIC};
use ep_platform_runtime::selfcheck::baseline_registry;
use ep_platform_runtime::serving::Serving;
use ep_platform_runtime::{http, BuildInfo, ProcessKind};

use config::{WorkerConfig, DEFAULTS};

const PROCESS: ProcessKind = ProcessKind::JobWorker;

fn main() -> ExitCode {
    let p = match boot::prepare::<WorkerConfig>(PROCESS, DEFAULTS, |c| &c.log, |c| &c.runtime) {
        Ok(p) => p,
        Err(code) => return code,
    };
    let logger = p.logger.clone();
    p.runtime.block_on(serve(p.cfg, logger, p.layers, p.check))
}

async fn serve(cfg: WorkerConfig, logger: Arc<JsonLogger>, layers: String, check_only: bool) -> ExitCode {
    let mut lifecycle = Lifecycle::new(PROCESS);
    boot::enter_configuring(&mut lifecycle, &logger);
    boot::enter_selfchecking(&mut lifecycle, &logger);

    let metrics = boot::metrics(&logger);
    let registry = baseline_registry(PROCESS, layers, cfg.selfcheck.clock_skew_max_ms, wiring::sql_probe());
    if check_only {
        return boot::check_exit(&registry.run_all(PROCESS, BuildInfo::current().version).await);
    }
    let report = match boot::selfcheck(&registry, PROCESS, &mut lifecycle, &metrics, &logger).await {
        Ok(r) => r,
        Err((report, code)) => {
            println!("{}", report.to_json());
            return code;
        }
    };

    let state = SystemState::new(PROCESS, BuildInfo::current(), lifecycle, report, metrics, logger.clone());
    let addr = match http::parse_addr(&cfg.http.bind_addr) {
        Ok(a) => a,
        Err(e) => {
            logger.log(Level::Error, LogFields::msg("startup", format!("{e}")));
            return ExitCode::from(EXIT_PANIC);
        }
    };

    let router = minimal_router()
        .fallback(http::system::fallback)
        .route_layer(from_fn_with_state(state.clone(), observe))
        .layer(from_fn_with_state(state.clone(), catch_panic))
        .with_state(state.clone());

    let mut serving = Serving::new();
    serving.spawn_http(addr, router, &logger).await;
    serving.spawn(scheduler::run(scheduler::JobRegistry::new(), serving.signal(), logger.clone()));

    logger.log(Level::Info, LogFields::msg("startup", format!("已就绪，状态 {}", state.state().as_str())));
    serving.wait_and_drain(&state, &logger, cfg.http.shutdown_drain_ms).await
}
