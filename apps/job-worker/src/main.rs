//! job-worker — 8081 健康与指标、任务调度器与两个身份后台任务
//! （阶段 4 任务 #21：过期会话/挑战清理、应急到期失效与轮换）、
//! 200 毫秒到 2 秒的退避轮询空转、优雅停机。
//!
//! 本阶段不消费 Outbox、不投递通知、不做对账。

mod config;
mod jobs;
mod scheduler;
mod wiring;

use std::process::ExitCode;
use std::sync::Arc;

use axum::middleware::from_fn_with_state;
use ep_adapter_db_pg::POOL_GAUGE_REFRESH_INTERVAL;
use ep_platform_obs::log::{JsonLogger, Level, LogFields};
use ep_platform_runtime::boot;
use ep_platform_runtime::http::middleware::{catch_panic, observe};
use ep_platform_runtime::http::{minimal_router, SystemState};
use ep_platform_runtime::lifecycle::{Lifecycle, EXIT_CONFIG_OR_SELFCHECK, EXIT_PANIC};
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

async fn serve(
    cfg: WorkerConfig,
    logger: Arc<JsonLogger>,
    layers: String,
    check_only: bool,
) -> ExitCode {
    let mut lifecycle = Lifecycle::new(PROCESS);
    boot::enter_configuring(&mut lifecycle, &logger);

    let metrics = boot::metrics(&logger);

    // 连接预算求和校验（C-04）：违例即以退出码 78 拒启，不带病运行。
    if let Err(violations) = wiring::budget_check(&cfg.db) {
        for v in &violations {
            logger.log(
                Level::Error,
                LogFields::msg("startup", format!("连接预算违例：{v:?}")),
            );
        }
        return ExitCode::from(EXIT_CONFIG_OR_SELFCHECK);
    }

    // 数据库与系统机密是本进程的启动前置。装配失败不得降成 None/Pending，
    // 否则默认 KMS 未实现也会被假报为自检通过。
    let db = match wiring::build(&cfg.db, &cfg.secrets, &cfg.platform, metrics.clone()) {
        Ok(assembly) => Some(Arc::new(assembly)),
        Err(reason) => {
            logger.log(
                Level::Error,
                LogFields::msg("startup", format!("数据库装配失败，拒绝启动：{reason}")),
            );
            return ExitCode::from(EXIT_CONFIG_OR_SELFCHECK);
        }
    };

    // 身份域后台任务装配（阶段 4 任务 #21）：db 缺位即不注入，
    // 调度器按零任务空转（unwired-absent）。
    let identity = match &db {
        Some(db) => match wiring::identity::build(db, &cfg.auth, logger.clone()) {
            Ok(assembly) => Some(Arc::new(assembly)),
            Err(reason) => {
                logger.log(
                    Level::Warn,
                    LogFields::msg("startup", format!("身份域后台装配未注入：{reason}")),
                );
                None
            }
        },
        None => None,
    };

    boot::enter_selfchecking(&mut lifecycle, &logger);
    let registry = baseline_registry(
        PROCESS,
        layers,
        cfg.selfcheck.clock_skew_max_ms,
        db.as_ref().and_then(|d| d.sql_probe()),
        None,
        db.as_ref().and_then(|d| d.degradation_ledger()),
    );
    if check_only {
        let code = boot::check_exit(
            &registry
                .run_all(PROCESS, BuildInfo::current().version)
                .await,
        );
        if let Some(db) = &db {
            db.pools.close().await;
        }
        return code;
    }
    let report = match boot::selfcheck(&registry, PROCESS, &mut lifecycle, &metrics, &logger).await
    {
        Ok(r) => r,
        Err((report, code)) => {
            println!("{}", report.to_json());
            if let Some(db) = &db {
                db.pools.close().await;
            }
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
            if let Some(db) = &db {
                db.pools.close().await;
            }
            return ExitCode::from(EXIT_PANIC);
        }
    };

    let router = minimal_router()
        .fallback(http::system::fallback)
        .layer(from_fn_with_state(state.clone(), catch_panic))
        .layer(from_fn_with_state(state.clone(), observe))
        .with_state(state.clone());

    let mut serving = Serving::new();
    serving.spawn_http(addr, router, &logger).await;
    if let Some(db) = db.clone() {
        let signal = serving.signal();
        serving.spawn(async move {
            db.pools
                .refresh_gauges_until(POOL_GAUGE_REFRESH_INTERVAL, signal.wait())
                .await;
        });
    }
    // 阶段 3a 装配位：内容项 applier 注册表空骨架。发布执行归
    // 本进程（03 计划 §3.4.12），applier 实现随属主模块阶段注入
    // （见 wiring/release.rs），执行路径接通前在此显式持有。
    let _config_appliers = Arc::new(wiring::release::config_item_applier_registry());
    // 两个身份后台任务（阶段 4 任务 #21）：装配缺位即零任务空转。
    let mut registry = scheduler::JobRegistry::new();
    if let Some(id) = &identity {
        registry.register(Arc::new(jobs::SessionHygieneJob::new(
            id.hygiene.clone(),
            id.directory.clone(),
        )));
        registry.register(Arc::new(jobs::BreakglassMaintenanceJob::new(
            id.breakglass.clone(),
            id.directory.clone(),
        )));
    }
    serving.spawn_critical(
        "job-worker 调度循环",
        scheduler::run(registry, serving.signal(), logger.clone()),
    );

    if serving.startup_succeeded().await {
        logger.log(
            Level::Info,
            LogFields::msg(
                "startup",
                format!("已就绪，状态 {}", state.state().as_str()),
            ),
        );
    }
    let code = serving
        .wait_and_drain(&state, &logger, cfg.http.shutdown_drain_ms)
        .await;
    if let Some(db) = db {
        db.pools.close().await;
    }
    code
}
