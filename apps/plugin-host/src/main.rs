//! plugin-host — `/run/ep/ipc/plugin.sock` IPC 服务端，零数据库连接，无 HTTP。
//!
//! 本阶段不实现 WASM 宿主。

mod config;
mod wiring;

use std::process::ExitCode;
use std::sync::Arc;

use ep_adapter_ipc::IpcServer;
use ep_platform_obs::log::{JsonLogger, Level, LogFields};
use ep_platform_runtime::boot;
use ep_platform_runtime::http::SystemState;
use ep_platform_runtime::lifecycle::Lifecycle;
use ep_platform_runtime::selfcheck::baseline_registry;
use ep_platform_runtime::serving::Serving;
use ep_platform_runtime::{BuildInfo, ProcessKind};

use config::{PluginHostConfig, DEFAULTS, SHUTDOWN_DRAIN_MS};

const PROCESS: ProcessKind = ProcessKind::PluginHost;

fn main() -> ExitCode {
    let p = match boot::prepare::<PluginHostConfig>(PROCESS, DEFAULTS, |c| &c.log, |c| &c.runtime) {
        Ok(p) => p,
        Err(code) => return code,
    };
    let logger = p.logger.clone();
    p.runtime.block_on(serve(p.cfg, logger, p.layers, p.check))
}

async fn serve(
    cfg: PluginHostConfig,
    logger: Arc<JsonLogger>,
    layers: String,
    check_only: bool,
) -> ExitCode {
    let mut lifecycle = Lifecycle::new(PROCESS);
    boot::enter_configuring(&mut lifecycle, &logger);
    boot::enter_selfchecking(&mut lifecycle, &logger);

    let metrics = boot::metrics(&logger);
    // 不持 SQL 会话：四项 SQL 自检对本进程一律 NotApplicable。
    let registry = baseline_registry(
        PROCESS,
        layers,
        cfg.selfcheck.clock_skew_max_ms,
        None,
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
            serving.spawn(async move {
                ipc.serve(listener, async move {
                    signal.wait().await;
                })
                .await;
            });
        }
        // 唯一的服务端起不来就没有任何可服务的东西，不能宣称就绪。
        Err(e) => serving.mark_failed(format!("IPC 服务端不可用：{e}")),
    }

    logger.log(
        Level::Info,
        LogFields::msg(
            "startup",
            format!("已就绪，状态 {}", state.state().as_str()),
        ),
    );
    serving
        .wait_and_drain(&state, &logger, SHUTDOWN_DRAIN_MS)
        .await
}
