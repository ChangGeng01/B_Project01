//! integration-gateway — 固定本机 IPC、出网客户端骨架（超时、退避、熔断）、
//! 出网白名单校验、零数据库连接、零 TCP 监听、优雅停机。
//!
//! 本阶段不实现电子签章协议，也不做证据固化，更不发起任何真实出网请求。

mod config;
mod egress;
mod wiring;

use std::process::ExitCode;
use std::sync::Arc;

use ep_adapter_ipc::{IpcServer, INTEGRATION_ENDPOINT};
use ep_platform_obs::log::{JsonLogger, Level, LogFields};
use ep_platform_runtime::boot;
use ep_platform_runtime::http::SystemState;
use ep_platform_runtime::lifecycle::{Lifecycle, EXIT_CONFIG_OR_SELFCHECK};
use ep_platform_runtime::selfcheck::baseline_registry;
use ep_platform_runtime::serving::Serving;
use ep_platform_runtime::{BuildInfo, ProcessKind};

use config::{IntegrationConfig, DEFAULTS, SHUTDOWN_DRAIN_MS};

const PROCESS: ProcessKind = ProcessKind::IntegrationGateway;

fn main() -> ExitCode {
    let p = match boot::prepare::<IntegrationConfig>(PROCESS, DEFAULTS, |c| &c.log, |c| &c.runtime)
    {
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
    if let Err(detail) = cfg.ipc.require_endpoint(INTEGRATION_ENDPOINT) {
        logger.log(Level::Error, LogFields::msg("startup", detail));
        return ExitCode::from(EXIT_CONFIG_OR_SELFCHECK);
    }
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

    let mut serving = Serving::new();

    // 出网骨架的一次性演练：白名单已在配置层解析通过，这里再验一遍判定口径
    // 与熔断参数，全程不发起真实请求。演练不通过就不宣称就绪。
    match egress::rehearse(&cfg.egress.allowlist, cfg.egress.breaker) {
        Ok(msg) => logger.log(
            Level::Info,
            LogFields::msg(
                "egress",
                format!("{msg}，连接超时 {} 毫秒", cfg.egress.connect_timeout_ms),
            ),
        ),
        Err(e) => serving.mark_failed(format!("出网骨架演练不通过：{e}")),
    }

    let ipc = IpcServer::new(
        cfg.ipc.socket_path.clone(),
        cfg.ipc.max_frame_bytes,
        wiring::method_table(state.clone()),
    );
    match ipc.bind() {
        Ok(listener) => {
            let signal = serving.signal();
            logger.log(
                Level::Info,
                LogFields::msg("startup", format!("IPC 监听 {}", ipc.path().display())),
            );
            serving.spawn_critical("integration-gateway IPC 服务端", async move {
                ipc.serve(listener, async move {
                    signal.wait().await;
                })
                .await;
            });
        }
        Err(e) => serving.mark_failed(format!("IPC 服务端不可用：{e}")),
    }
    if serving.startup_succeeded().await {
        logger.log(
            Level::Info,
            LogFields::msg(
                "startup",
                format!("已就绪，状态 {}", state.state().as_str()),
            ),
        );
    }
    serving
        .wait_and_drain(&state, &logger, SHUTDOWN_DRAIN_MS)
        .await
}
