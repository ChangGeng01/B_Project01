//! core-server — 8080 HTTP、五个系统端点、IPC 服务端、并发闸门、
//! 同步等待上限、启动自检与优雅停机。
//!
//! crate 名与进程名、systemd 单元名、cgroup slice 名一一对应，
//! 由 `xtask codecheck` 断言。本阶段没有任何业务路由。

mod config;
#[cfg(feature = "ci-probe")]
mod probe;
mod wiring;

use std::process::ExitCode;
use std::sync::Arc;

use axum::middleware::from_fn_with_state;
use ep_adapter_ipc::IpcServer;
use ep_platform_obs::log::{JsonLogger, Level, LogFields};
use ep_platform_runtime::boot;
use ep_platform_runtime::http::middleware::{catch_panic, concurrency_gate, observe, sync_timeout};
use ep_platform_runtime::http::{core_system_router, Gate, SyncLimit, SystemState};
use ep_platform_runtime::lifecycle::{Lifecycle, EXIT_PANIC};
use ep_platform_runtime::selfcheck::baseline_registry;
use ep_platform_runtime::serving::Serving;
use ep_platform_runtime::{http, BuildInfo, ProcessKind};

use config::{CoreConfig, DEFAULTS};

const PROCESS: ProcessKind = ProcessKind::CoreServer;

fn main() -> ExitCode {
    let p = match boot::prepare::<CoreConfig>(PROCESS, DEFAULTS, |c| &c.log, |c| &c.runtime) {
        Ok(p) => p,
        Err(code) => return code,
    };
    let logger = p.logger.clone();
    p.runtime.block_on(serve(p.cfg, logger, p.layers, p.check))
}

async fn serve(cfg: CoreConfig, logger: Arc<JsonLogger>, layers: String, check_only: bool) -> ExitCode {
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

    let mut serving = Serving::new();
    serving.spawn_http(addr, build_router(&cfg, state.clone()), &logger).await;

    // IPC 服务端与 HTTP 同起同停：core.sock 是写出进程唯一的上报入口，
    // 它不可用而 HTTP 可用，会让归档与备份静默地一直落 spool。
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
        Err(e) => serving.mark_failed(format!("IPC 服务端不可用：{e}")),
    }

    logger.log(
        Level::Info,
        LogFields::msg("startup", format!("已就绪，状态 {}", state.state().as_str())),
    );
    serving.wait_and_drain(&state, &logger, cfg.http.shutdown_drain_ms).await
}

fn build_router(cfg: &CoreConfig, state: Arc<SystemState>) -> axum::Router {
    let gate = Gate::new(cfg.http.concurrency_limit, cfg.http.concurrency_wait_ms, state.clone());
    let limit = SyncLimit::new(cfg.http.request_timeout_ms, state.clone());

    let router = core_system_router();
    #[cfg(feature = "ci-probe")]
    let router = router.merge(probe::router(state.clone()));

    // 由外到内是 panic 捕获、并发闸门、同步等待上限、访问日志与指标。
    // panic 捕获在最外层，才盖得住闸门与超时层自身的意外。
    router
        .fallback(http::system::fallback)
        // route_layer 才拿得到 MatchedPath，指标的 route 标签因此是模板路径。
        .route_layer(from_fn_with_state(state.clone(), observe))
        .layer(from_fn_with_state(limit, sync_timeout))
        .layer(from_fn_with_state(gate, concurrency_gate))
        .layer(from_fn_with_state(state.clone(), catch_panic))
        .with_state(state)
}
