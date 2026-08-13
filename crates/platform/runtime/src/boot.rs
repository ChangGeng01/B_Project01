//! 八个进程共用的启动骨架。
//!
//! 每个进程的 main 只表达自己的形态差异（听哪个端口、起不起 IPC、有没有
//! 轮询循环），配置加载、自检、状态机迁移与退出码这四件事在这里一次写死，
//! 否则八份 main 会各自漂移出八种退出码语义。

use std::process::ExitCode;
use std::sync::Arc;

use ep_platform_obs::log::{JsonLogger, Level, LogFields};
use ep_platform_obs::MetricsRegistry;

use crate::cli::{Cli, USAGE};
use crate::config::{ConfigError, RuntimeCfg};
use crate::http::state::SystemState;
use crate::lifecycle::{Event, Lifecycle, State, EXIT_CONFIG_OR_SELFCHECK};
use crate::process::{BuildInfo, ProcessKind};
use crate::selfcheck::items::basic::ConfigInvalid;
use crate::selfcheck::registry::{
    Outcome, SelfCheckItem, SelfCheckRegistry, SelfCheckReport, Severity,
};

/// 参数错误的退出码。与 78 区分：78 是配置或自检不通过，2 是命令行本身写错。
pub const EXIT_USAGE: u8 = 2;

/// 解析命令行。`--help` 与参数错误都在这里终结，返回 Err 时调用方直接返回该退出码。
pub fn parse_cli() -> Result<Cli, ExitCode> {
    match Cli::from_env() {
        Ok(cli) if cli.help => {
            println!("{USAGE}");
            Err(ExitCode::SUCCESS)
        }
        Ok(cli) => Ok(cli),
        Err(e) => {
            eprintln!("{e}");
            Err(ExitCode::from(EXIT_USAGE))
        }
    }
}

/// 启动前四件事的结果：配置、日志器、异步运行时、生效层描述。
pub struct Prepared<T> {
    pub cfg: T,
    pub logger: Arc<JsonLogger>,
    pub runtime: tokio::runtime::Runtime,
    pub layers: String,
    pub check: bool,
}

/// 命令行、配置、日志器与运行时四件事，八个进程一模一样，在这里做完。
///
/// 两个取字段的函数参数而不是给配置加 trait：加 trait 会要求八个配置根结构
/// 都实现它，而它们真正的差别恰恰是有没有某个段。
pub fn prepare<T: serde::de::DeserializeOwned>(
    process: ProcessKind,
    defaults: &str,
    log_of: fn(&T) -> &crate::config::LogCfg,
    runtime_of: fn(&T) -> &RuntimeCfg,
) -> Result<Prepared<T>, ExitCode> {
    let cli = parse_cli()?;
    let sources = cli.config_sources(process);
    let cfg: T =
        crate::config::load(defaults, &sources).map_err(|e| config_invalid_exit(process, &e))?;
    let logger = logger(process, &log_of(&cfg).level).map_err(|e| {
        eprintln!("{e}");
        ExitCode::from(EXIT_USAGE)
    })?;
    let runtime = tokio_runtime(runtime_of(&cfg)).map_err(|e| {
        eprintln!("构造异步运行时失败：{e}");
        ExitCode::from(crate::lifecycle::EXIT_PANIC)
    })?;
    let layers = cli.layers_description(&sources);
    Ok(Prepared {
        cfg,
        logger,
        runtime,
        layers,
        check: cli.check,
    })
}

/// 配置不可用时的收尾：出一份只有 `config-parsed` 一项的报告，再以 78 退出。
///
/// 不是「跳过自检直接退出」：部署方拿到的必须是同一种报告结构，
/// 否则升级前置脚本要为这一种情况另写一套解析。
pub fn config_invalid_exit(process: ProcessKind, error: &ConfigError) -> ExitCode {
    let mut reg = SelfCheckRegistry::new();
    reg.register(SelfCheckItem::new(
        "config-parsed",
        "配置解析成功且无未知键",
        Severity::Blocking,
        Arc::new(ConfigInvalid {
            detail: error.to_string(),
        }),
    ))
    .expect("单项注册不可能重名");
    let report = futures_block_on(reg.run_all(process, BuildInfo::current().version));
    println!("{}", report.to_json());
    eprintln!("配置不可用：{error}");
    ExitCode::from(EXIT_CONFIG_OR_SELFCHECK)
}

/// 只为「配置层已经失败、还没有 tokio 运行时」这一种情况准备的最小执行器。
/// 自检项的 run 是 async 的，而这条路径上没有运行时可用。
fn futures_block_on<F: std::future::Future>(fut: F) -> F::Output {
    // 这条路径上的两个自检项都不做 IO，不会 Pending，因此一个当前线程运行时足够。
    tokio::runtime::Builder::new_current_thread()
        .build()
        .expect("构造单线程运行时失败说明进程已无法继续")
        .block_on(fut)
}

/// 按配置构造多线程运行时。`worker_threads` 取 0 表示交给 tokio 按 CPU 配额推导。
pub fn tokio_runtime(cfg: &RuntimeCfg) -> std::io::Result<tokio::runtime::Runtime> {
    let mut builder = tokio::runtime::Builder::new_multi_thread();
    builder
        .enable_all()
        .max_blocking_threads(usize::from(cfg.blocking_threads));
    if cfg.worker_threads > 0 {
        builder.worker_threads(usize::from(cfg.worker_threads));
    }
    builder.build()
}

/// 日志器。级别取值非法必须报错，不静默降级为 info。
pub fn logger(process: ProcessKind, level: &str) -> Result<Arc<JsonLogger>, String> {
    let lvl = Level::parse(level)
        .ok_or_else(|| format!("log.level 取值 {level} 不在 debug/info/warn/error 之内"))?;
    Ok(Arc::new(JsonLogger::new(
        process.name(),
        BuildInfo::current().version,
        lvl,
    )))
}

/// 指标注册表，并填 `ep_build_info`。
pub fn metrics(logger: &JsonLogger) -> Arc<MetricsRegistry> {
    let reg = Arc::new(MetricsRegistry::new());
    let b = BuildInfo::current();
    if let Err(e) = reg.set_gauge(
        "ep_build_info",
        &[("version", b.version), ("git_commit", b.git_commit)],
        1.0,
    ) {
        logger.log(
            Level::Error,
            LogFields::msg("metrics", format!("ep_build_info 写入失败：{e}")),
        );
    }
    reg
}

/// 执行自检并把结论落到状态机与指标上。
///
/// 返回 Err 时进程必须退出，退出码已经包含在内。
pub async fn selfcheck(
    registry: &SelfCheckRegistry,
    process: ProcessKind,
    lifecycle: &mut Lifecycle,
    metrics: &MetricsRegistry,
    logger: &JsonLogger,
) -> Result<SelfCheckReport, (SelfCheckReport, ExitCode)> {
    let report = registry
        .run_all(process, BuildInfo::current().version)
        .await;
    if let Err(e) = metrics.set_gauge(
        "ep_selfcheck_pending_items",
        &[("process", process.name())],
        report.pending_items() as f64,
    ) {
        logger.log(
            Level::Error,
            LogFields::msg("metrics", format!("Pending 计数写入失败：{e}")),
        );
    }

    let event = match report.overall {
        Outcome::Failed => Event::AnyFailed,
        Outcome::Degraded => Event::PassedWithDegradation,
        _ => Event::AllPassed,
    };
    // SelfChecking 之前必须已经过 Configuring；迁移失败说明调用顺序写错了。
    if let Err(e) = lifecycle.fire(event) {
        logger.log(Level::Error, LogFields::msg("lifecycle", format!("{e}")));
        return Err((report, ExitCode::from(EXIT_CONFIG_OR_SELFCHECK)));
    }
    if lifecycle.state() == State::Failed {
        for item in report.items.iter().filter(|i| i.outcome == Outcome::Failed) {
            eprintln!("启动自检不通过：{} — {}", item.name, item.detail);
        }
        return Err((report, ExitCode::from(EXIT_CONFIG_OR_SELFCHECK)));
    }
    Ok(report)
}

/// `--check` 模式的收尾：打印报告并按报告给出退出码。
pub fn check_exit(report: &SelfCheckReport) -> ExitCode {
    println!("{}", report.to_json());
    let code = report.check_exit_code();
    if code != 0 {
        eprintln!("--check 未通过：overall={:?}", report.overall);
    }
    if report.pending_items() > 0 {
        eprintln!(
            "注意：{} 项自检为 PENDING（未覆盖，不计入成败）",
            report.pending_items()
        );
    }
    ExitCode::from(code)
}

/// 把生命周期推到 Configuring。配置加载之前调用。
pub fn enter_configuring(lifecycle: &mut Lifecycle, logger: &JsonLogger) {
    if let Err(e) = lifecycle.fire(Event::Start) {
        logger.log(Level::Error, LogFields::msg("lifecycle", format!("{e}")));
    }
}

/// 配置就绪，推到 SelfChecking。
pub fn enter_selfchecking(lifecycle: &mut Lifecycle, logger: &JsonLogger) {
    if let Err(e) = lifecycle.fire(Event::ConfigLoaded) {
        logger.log(Level::Error, LogFields::msg("lifecycle", format!("{e}")));
    }
}

/// 停机收尾：Draining → Stopped，退出码 0。
pub fn finish_draining(
    state: &SystemState,
    logger: &JsonLogger,
    drained_in_time: bool,
) -> ExitCode {
    if let Err(e) = state.fire(Event::DrainComplete) {
        logger.log(Level::Error, LogFields::msg("lifecycle", format!("{e}")));
    }
    if !drained_in_time {
        logger.log(
            Level::Warn,
            LogFields::msg("shutdown", "drain 超时，强制关闭在途连接"),
        );
    }
    logger.log(Level::Info, LogFields::msg("shutdown", "已停止，退出码 0"));
    ExitCode::SUCCESS
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn usage_exit_code_differs_from_selfcheck_exit_code() {
        // 两个退出码不得合并：2 表示命令行写错，78 表示配置或自检不通过，
        // systemd 只对 78 不重启。
        assert_ne!(EXIT_USAGE, EXIT_CONFIG_OR_SELFCHECK);
    }

    #[test]
    fn invalid_log_level_is_rejected() {
        assert!(logger(ProcessKind::CoreServer, "info").is_ok());
        assert!(logger(ProcessKind::CoreServer, "verbose").is_err());
    }

    #[test]
    fn worker_threads_zero_means_derive_from_quota() {
        let rt = tokio_runtime(&RuntimeCfg {
            worker_threads: 0,
            blocking_threads: 8,
        })
        .unwrap();
        drop(rt);
        let rt = tokio_runtime(&RuntimeCfg {
            worker_threads: 2,
            blocking_threads: 8,
        })
        .unwrap();
        drop(rt);
    }
}
