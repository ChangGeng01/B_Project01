//! core-server — 8080 HTTP、五个系统端点、IPC 服务端、并发闸门、
//! 同步等待上限、启动自检与优雅停机。
//!
//! crate 名与进程名、systemd 单元名、cgroup slice 名一一对应，
//! 由 `xtask codecheck` 断言。本阶段没有任何业务路由。

mod config;
mod platform;
#[cfg(feature = "ci-probe")]
mod probe;
mod wiring;

use std::process::ExitCode;
use std::sync::Arc;

use axum::middleware::from_fn_with_state;
use ep_adapter_db_pg::POOL_GAUGE_REFRESH_INTERVAL;
use ep_adapter_ipc::{IpcServer, CORE_ENDPOINT};
use ep_platform_obs::log::{JsonLogger, Level, LogFields};
use ep_platform_runtime::boot;
use ep_platform_runtime::http::headers::header_guard;
use ep_platform_runtime::http::middleware::{catch_panic, concurrency_gate, observe, sync_timeout};
use ep_platform_runtime::http::{core_system_router, Gate, SyncLimit, SystemState};
use ep_platform_runtime::lifecycle::{Lifecycle, EXIT_CONFIG_OR_SELFCHECK, EXIT_PANIC};
use ep_platform_runtime::selfcheck::baseline_registry;
use ep_platform_runtime::selfcheck::{SelfCheckItem, Severity};
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

async fn serve(
    cfg: CoreConfig,
    logger: Arc<JsonLogger>,
    layers: String,
    check_only: bool,
) -> ExitCode {
    if let Err(detail) = cfg.ipc.require_endpoint(CORE_ENDPOINT) {
        logger.log(Level::Error, LogFields::msg("startup", detail));
        return ExitCode::from(EXIT_CONFIG_OR_SELFCHECK);
    }
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

    // 数据库与系统机密是本进程的启动前置。装配失败若降成 None，Blocking
    // 自检会被改写为 Pending，并可能把 KMS NOT_IMPLEMENTED 假报为启动通过。
    // 因此这里必须在任何监听器建立前以 78 失败关闭。
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
    let kms = match &db {
        Some(_) => match wiring::kms::build_kms_backend(&cfg.kms, &cfg.secrets.dir) {
            Ok(backend) => Some(backend),
            Err(reason) => {
                logger.log(
                    Level::Warn,
                    LogFields::msg("startup", format!("密钥后端未注入：{reason}")),
                );
                None
            }
        },
        None => None,
    };

    // 身份域装配（阶段 4 任务 #21）：依赖 db 与 kms 双就位，任一
    // 前置不满足即不注入，身份端点按 503 NOT_PROVISIONED 处置。
    let identity = match (&db, &kms) {
        (Some(db), Some(kms)) => match wiring::identity::build(
            db,
            kms.clone(),
            &cfg.auth,
            &cfg.admission,
            metrics.clone(),
            logger.clone(),
        )
        .await
        {
            Ok(assembly) => Some(Arc::new(assembly)),
            Err(reason) => {
                logger.log(
                    Level::Warn,
                    LogFields::msg("startup", format!("身份域装配未注入：{reason}")),
                );
                None
            }
        },
        _ => None,
    };

    // 认证中间件载体（阶段 4 任务 #23）：只依赖数据库装配；
    // 缺位即不注入，中间件按未装配形态放行（unwired-absent）。
    let authn = db
        .as_ref()
        .map(|d| wiring::authn::build(d, &cfg.auth.session, metrics.clone()));

    // 授权域装配（阶段 4 任务 #23）：快照轮询器、指标桥接与
    // checksum 接线位；只依赖数据库装配，缺位即不注入。
    let authz = db
        .as_ref()
        .map(|d| wiring::authz::build(d, &cfg.authz, metrics.clone(), logger.clone()));

    boot::enter_selfchecking(&mut lifecycle, &logger);
    let mut registry = baseline_registry(
        PROCESS,
        layers,
        cfg.selfcheck.clock_skew_max_ms,
        db.as_ref().and_then(|d| d.sql_probe()),
        db.as_ref()
            .and_then(|d| d.secrets_probe(&cfg.secrets, &cfg.db, &cfg.kms)),
        db.as_ref().and_then(|d| d.degradation_ledger()),
    );
    // 阶段 4 新增自检项（Blocking，kebab-case）：授权快照可加载。
    // 重复名是装配错误，如实记 WARN 不静默吞掉。
    if let Some(assembly) = &authz {
        if let Err(e) = registry.register(SelfCheckItem::new(
            "authz-snapshot-loadable",
            "授权配置快照可加载",
            Severity::Blocking,
            Arc::new(wiring::authz::SnapshotLoadableCheck::new(
                assembly.poller.clone(),
            )),
        )) {
            logger.log(Level::Warn, LogFields::msg("startup", format!("{e}")));
        }
    }

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

    // 阶段 3a 装配位：内容项 applier 注册表。阶段 4 注入三个 AUTHZ
    // applier（见 wiring/release.rs）；注册失败即不注入（unwired-absent），
    // 发布执行路径接通前在此显式持有，不以空实现顶位。
    let _config_appliers = match wiring::release::config_item_applier_registry() {
        Ok(r) => Some(Arc::new(r)),
        Err(reason) => {
            logger.log(
                Level::Warn,
                LogFields::msg("startup", format!("内容项 applier 注册表未注入：{reason}")),
            );
            None
        }
    };

    let platform_state = Arc::new(platform::PlatformState {
        system: state.clone(),
        db: db.clone(),
        kms,
        identity,
        authn,
        authz,
        trusted_proxy_cidrs: cfg.http.trusted_proxy_cidrs.clone().into(),
        window_ttl_max_min: cfg.migration.window_ttl_max_min,
    });
    let authz_poller = platform_state.authz.as_ref().map(|a| a.poller.clone());

    let mut serving = Serving::new();
    serving
        .spawn_http(
            addr,
            build_router(&cfg, state.clone(), platform_state),
            &logger,
        )
        .await;

    if let Some(db) = db.clone() {
        let signal = serving.signal();
        serving.spawn(async move {
            db.pools
                .refresh_gauges_until(POOL_GAUGE_REFRESH_INTERVAL, signal.wait())
                .await;
        });
    }

    // 授权快照轮询任务（EP__AUTHZ__SNAPSHOT__POLL_INTERVAL_MS）：
    // 与 HTTP 同起同停；单轮失败记 WARN 后继续，不退出循环。
    if let Some(poller) = authz_poller {
        let signal = serving.signal();
        serving.spawn_critical("授权快照轮询", async move {
            poller.run_until(signal.wait()).await;
        });
    }

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
            serving.spawn_critical("core IPC 服务端", async move {
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
    let code = serving
        .wait_and_drain(&state, &logger, cfg.http.shutdown_drain_ms)
        .await;
    if let Some(db) = db {
        db.pools.close().await;
    }
    code
}

fn build_router(
    cfg: &CoreConfig,
    state: Arc<SystemState>,
    platform_state: Arc<platform::PlatformState>,
) -> axum::Router {
    let gate = Gate::new(
        cfg.http.concurrency_limit,
        cfg.http.concurrency_wait_ms,
        state.clone(),
    );
    let limit = SyncLimit::new(cfg.http.request_timeout_ms, state.clone());
    let trusted_proxies = platform_state.trusted_proxy_cidrs.clone();

    // 平台路由已在内部应用自己的状态，合并前先各自落态为 Router<()>；
    // fallback 依赖系统状态，须在落态前挂上。
    let router = core_system_router()
        .fallback(http::system::fallback)
        .with_state(state.clone());
    #[cfg(feature = "ci-probe")]
    let router = router.merge(probe::router(state.clone()).with_state(state.clone()));
    let router = router.merge(platform::platform_router(platform_state.clone()));

    // 由外到内是服务端请求元数据/全部 x-ep-* 剥离、统一访问轨迹、panic 捕获、
    // 请求体上限、并发闸门、同步等待上限、四头纯格式校验（第一道）、
    // 认证与法人真实校验（阶段 4 任务 #23，经端口在 wiring 注入）。请求元数据
    // 层必须最外；访问轨迹必须包住其他所有层与 fallback，才能让请求头、
    // 认证、闸门、超时、正文上限、panic 与 404 的提前响应也恰有一条记录；
    // MatchedPath 仍取模板路径，fallback 固定为 <unmatched>。
    router
        .layer(from_fn_with_state(
            platform_state.clone(),
            platform::middleware::authenticate,
        ))
        .layer(from_fn_with_state(
            platform_state.system.clone(),
            header_guard,
        ))
        .layer(from_fn_with_state(limit, sync_timeout))
        .layer(from_fn_with_state(gate, concurrency_gate))
        // F-83：请求体上限。这是唯一被登记的入口保护，机器只有 32GB 内存。
        // 原先 `http.max_body_bytes` 声明了却无人取用，实际生效的是 axum 的
        // 2 MiB 隐含默认——比登记值还大。挂在最外层（catch_panic 之内），
        // 使超限在进入任何 handler 与闸门之前即被拒。usize 化以适配 axum 接口；
        // 该值来自 u64 配置，取饱和转换，避免 32 位平台上的截断。
        .layer(axum::extract::DefaultBodyLimit::max(
            usize::try_from(cfg.http.max_body_bytes).unwrap_or(usize::MAX),
        ))
        .layer(from_fn_with_state(state.clone(), catch_panic))
        .layer(from_fn_with_state(state.clone(), observe))
        .layer(from_fn_with_state(
            trusted_proxies,
            platform::middleware::request_metadata,
        ))
}
