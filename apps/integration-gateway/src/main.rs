//! integration-gateway — 8082 健康与指标、出网客户端骨架（超时、退避、熔断）、
//! 出网白名单校验、独立池 5、优雅停机。
//!
//! 本阶段不实现电子签章协议，也不做证据固化，更不发起任何真实出网请求。

mod config;
mod egress;
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

use config::{IntegrationConfig, DEFAULTS};

const PROCESS: ProcessKind = ProcessKind::IntegrationGateway;

fn main() -> ExitCode {
    let p = match boot::prepare::<IntegrationConfig>(PROCESS, DEFAULTS, |c| &c.log, |c| &c.runtime) {
        Ok(p) => p,
        Err(code) => return code,
    };
    let logger = p.logger.clone();
    p.runtime.block_on(serve(p.cfg, logger, p.layers, p.check))
}

async fn serve(
    cfg: IntegrationConfig,
    logger: Arc<JsonLogger>,
    layers: String,
    check_only: bool,
) -> ExitCode {
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

    let mut serving = Serving::new();

    // 出网骨架的一次性演练：白名单已在配置层解析通过，这里再验一遍判定口径
    // 与熔断参数，全程不发起真实请求。演练不通过就不宣称就绪。
    match egress::rehearse(&cfg.egress.allowlist, cfg.egress.breaker) {
        Ok(msg) => logger.log(
            Level::Info,
            LogFields::msg("egress", format!("{msg}，连接超时 {} 毫秒", cfg.egress.connect_timeout_ms)),
        ),
        Err(e) => serving.mark_failed(format!("出网骨架演练不通过：{e}")),
    }

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

    serving.spawn_http(addr, router, &logger).await;
    logger.log(Level::Info, LogFields::msg("startup", format!("已就绪，状态 {}", state.state().as_str())));
    serving.wait_and_drain(&state, &logger, cfg.http.shutdown_drain_ms).await
}
