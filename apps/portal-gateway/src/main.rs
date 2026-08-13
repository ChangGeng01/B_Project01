//! portal-gateway — 8090 HTTP、不建数据库连接、经回环探测 core-server 的
//! 健康端点、门户侧新建 trace 并回带 X-Correlation-Id、优雅停机。
//!
//! 阶段 4 起另承载门户 Cookie → 核心会话令牌的转发换算（见
//! `session.rs`）；本阶段没有门户业务页面、没有脱敏投影。

mod config;
mod session;
mod upstream;
mod wiring;

use std::process::ExitCode;
use std::sync::Arc;

use axum::middleware::from_fn_with_state;
use ep_platform_obs::log::{JsonLogger, Level, LogFields};
use ep_platform_runtime::boot;
use ep_platform_runtime::http::middleware::{catch_panic, observe};
use ep_platform_runtime::http::{portal_system_router, SystemState};
use ep_platform_runtime::lifecycle::{Lifecycle, EXIT_PANIC};
use ep_platform_runtime::selfcheck::baseline_registry;
use ep_platform_runtime::serving::Serving;
use ep_platform_runtime::{http, BuildInfo, ProcessKind};

use config::{PortalConfig, DEFAULTS};
use upstream::PortalState;

const PROCESS: ProcessKind = ProcessKind::PortalGateway;

fn main() -> ExitCode {
    let p = match boot::prepare::<PortalConfig>(PROCESS, DEFAULTS, |c| &c.log, |c| &c.runtime) {
        Ok(p) => p,
        Err(code) => return code,
    };
    let logger = p.logger.clone();
    p.runtime.block_on(serve(p.cfg, logger, p.layers, p.check))
}

async fn serve(
    cfg: PortalConfig,
    logger: Arc<JsonLogger>,
    layers: String,
    check_only: bool,
) -> ExitCode {
    let mut lifecycle = Lifecycle::new(PROCESS);
    boot::enter_configuring(&mut lifecycle, &logger);
    boot::enter_selfchecking(&mut lifecycle, &logger);

    let metrics = boot::metrics(&logger);
    // 门户不持 SQL 会话，四项 SQL 自检一律标 NotApplicable，因此不注入探针。
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
    let addr = match http::parse_addr(&cfg.http.bind_addr) {
        Ok(a) => a,
        Err(e) => {
            logger.log(Level::Error, LogFields::msg("startup", format!("{e}")));
            return ExitCode::from(EXIT_PANIC);
        }
    };

    let portal_state = PortalState {
        system: state.clone(),
        upstream_base_url: cfg.portal.upstream_base_url.into(),
    };
    let router = portal_system_router()
        .merge(upstream::router())
        .fallback(http::system::fallback)
        // 门户 Cookie → 核心会话令牌换算（阶段 4 任务 #23）：
        // 在路由层之内完成，系统端点不受影响。
        .route_layer(axum::middleware::from_fn(session::forward_session))
        .route_layer(from_fn_with_state(state.clone(), observe))
        .layer(from_fn_with_state(state.clone(), catch_panic))
        .with_state(portal_state);

    let mut serving = Serving::new();
    serving.spawn_http(addr, router, &logger).await;
    logger.log(
        Level::Info,
        LogFields::msg(
            "startup",
            format!("已就绪，状态 {}", state.state().as_str()),
        ),
    );
    serving
        .wait_and_drain(&state, &logger, cfg.http.shutdown_drain_ms)
        .await
}
