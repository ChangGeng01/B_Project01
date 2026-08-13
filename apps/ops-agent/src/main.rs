//! ops-agent — 9101 Prometheus 文本（聚合本机各进程的指标端点）、
//! 9102 健康聚合、ep_ops_ro 池 2、优雅停机。
//!
//! 本阶段不读运维台账，也不做降级窗口。

mod config;
mod targets;
mod wiring;

use std::process::ExitCode;
use std::sync::Arc;

use axum::extract::State;
use axum::http::header;
use axum::middleware::from_fn_with_state;
use axum::response::{IntoResponse, Response};
use axum::routing::get;
use axum::Router;
use ep_platform_obs::log::{JsonLogger, Level, LogFields};
use ep_platform_runtime::boot;
use ep_platform_runtime::http::middleware::{catch_panic, observe};
use ep_platform_runtime::http::{ops_health_router, SystemState};
use ep_platform_runtime::lifecycle::{Lifecycle, EXIT_PANIC};
use ep_platform_runtime::selfcheck::baseline_registry;
use ep_platform_runtime::serving::Serving;
use ep_platform_runtime::{http, BuildInfo, ProcessKind};

use config::{OpsConfig, DEFAULTS};

const PROCESS: ProcessKind = ProcessKind::OpsAgent;

fn main() -> ExitCode {
    let p = match boot::prepare::<OpsConfig>(PROCESS, DEFAULTS, |c| &c.log, |c| &c.runtime) {
        Ok(p) => p,
        Err(code) => return code,
    };
    let logger = p.logger.clone();
    p.runtime.block_on(serve(p.cfg, logger, p.layers, p.check))
}

/// 9101 的聚合指标端点。本进程自己的指标也在里面，因此 ops-agent 不需要
/// 再单独暴露一个 `/metrics`。
async fn aggregated_metrics(State(st): State<Arc<SystemState>>) -> Response {
    let local = st.metrics().encode_text();
    let text = targets::render(&local, &targets::scrape_all().await);
    (
        [(
            header::CONTENT_TYPE,
            "text/plain; version=0.0.4; charset=utf-8",
        )],
        text,
    )
        .into_response()
}

async fn serve(
    cfg: OpsConfig,
    logger: Arc<JsonLogger>,
    layers: String,
    check_only: bool,
) -> ExitCode {
    let mut lifecycle = Lifecycle::new(PROCESS);
    boot::enter_configuring(&mut lifecycle, &logger);
    boot::enter_selfchecking(&mut lifecycle, &logger);

    let metrics = boot::metrics(&logger);
    let registry = baseline_registry(
        PROCESS,
        layers,
        cfg.selfcheck.clock_skew_max_ms,
        wiring::sql_probe(),
        None,
        None,
    );
    if check_only {
        return boot::check_exit(
            &registry
                .run_all(PROCESS, BuildInfo::current().version)
                .await,
        );
    }
    let report = match boot::selfcheck(&registry, PROCESS, &mut lifecycle, &metrics, &logger).await
    {
        Ok(r) => r,
        Err((report, code)) => {
            println!("{}", report.to_json());
            return code;
        }
    };

    let state = SystemState::new(
        PROCESS,
        BuildInfo::current(),
        lifecycle,
        report,
        metrics,
        logger.clone(),
    );
    let health_addr = match http::parse_addr(&cfg.http.bind_addr) {
        Ok(a) => a,
        Err(e) => {
            logger.log(Level::Error, LogFields::msg("startup", format!("{e}")));
            return ExitCode::from(EXIT_PANIC);
        }
    };
    let metrics_addr = match http::parse_addr(&cfg.metrics.bind_addr) {
        Ok(a) => a,
        Err(e) => {
            logger.log(Level::Error, LogFields::msg("startup", format!("{e}")));
            return ExitCode::from(EXIT_PANIC);
        }
    };

    let health = ops_health_router()
        .fallback(http::system::fallback)
        .route_layer(from_fn_with_state(state.clone(), observe))
        .layer(from_fn_with_state(state.clone(), catch_panic))
        .with_state(state.clone());
    let scrape = Router::new()
        .route("/metrics", get(aggregated_metrics))
        .fallback(http::system::fallback)
        .route_layer(from_fn_with_state(state.clone(), observe))
        .layer(from_fn_with_state(state.clone(), catch_panic))
        .with_state(state.clone());

    let mut serving = Serving::new();
    serving.spawn_http(metrics_addr, scrape, &logger).await;
    serving.spawn_http(health_addr, health, &logger).await;
    logger.log(
        Level::Info,
        LogFields::msg(
            "startup",
            format!("已就绪，抓取目标 {} 个", targets::TARGETS.len()),
        ),
    );
    serving
        .wait_and_drain(&state, &logger, cfg.http.shutdown_drain_ms)
        .await
}
