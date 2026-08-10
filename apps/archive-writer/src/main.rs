//! archive-writer — 无监听、spool 目录、IPC 客户端、15 分钟周期心跳占位、优雅停机。
//!
//! core-server 不可用时把心跳帧落 spool，恢复后按顺序补写并在成功后截断；
//! spool 超上限丢最旧并记 ERROR，绝不阻塞写出。
//!
//! 本阶段不实现事务日志归档、附件写出与审计证据写出。

mod config;
mod wiring;

use std::process::ExitCode;
use std::sync::Arc;
use std::time::Duration;

use ep_adapter_ipc::{Forwarder, IpcClient, Pending, Spool};
use ep_platform_obs::log::{JsonLogger, Level, LogFields};
use ep_platform_runtime::boot;
use ep_platform_runtime::http::SystemState;
use ep_platform_runtime::lifecycle::Lifecycle;
use ep_platform_runtime::selfcheck::baseline_registry;
use ep_platform_runtime::serving::Serving;
use ep_platform_runtime::{BuildInfo, ProcessKind};

use config::{ArchiveWriterConfig, DEFAULTS, SHUTDOWN_DRAIN_MS};

const PROCESS: ProcessKind = ProcessKind::ArchiveWriter;

/// 心跳周期。按阶段 1 计划第 3.2 节的 15 分钟周期心跳占位，与规格第 13.3 章的写出节奏同源。
pub const HEARTBEAT_PERIOD: Duration = Duration::from_secs(900);
/// 单次上报的等待上限。心跳不该把停机拖到 drain 上限之外。
const CALL_TIMEOUT: Duration = Duration::from_secs(3);

fn main() -> ExitCode {
    let p = match boot::prepare::<ArchiveWriterConfig>(PROCESS, DEFAULTS, |c| &c.log, |c| &c.runtime) {
        Ok(p) => p,
        Err(code) => return code,
    };
    let logger = p.logger.clone();
    p.runtime.block_on(serve(p.cfg, logger, p.layers, p.check))
}

async fn serve(cfg: ArchiveWriterConfig, logger: Arc<JsonLogger>, layers: String, check_only: bool) -> ExitCode {
    let mut lifecycle = Lifecycle::new(PROCESS);
    boot::enter_configuring(&mut lifecycle, &logger);
    boot::enter_selfchecking(&mut lifecycle, &logger);

    let metrics = boot::metrics(&logger);
    // 不持 SQL 会话：四项 SQL 自检对本进程一律 NotApplicable。
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

    let spool = Spool::new(cfg.spool.dir.clone(), cfg.spool.max_bytes);
    if let Err(e) = spool.ensure_dir() {
        // spool 目录建不出来，写出进程就没有落盘退路，不能宣称就绪。
        serving.mark_failed(format!("spool 目录不可用：{e}"));
    }
    let forwarder = Forwarder::new(
        IpcClient::new(cfg.ipc.socket_path.clone(), cfg.ipc.max_frame_bytes, CALL_TIMEOUT),
        spool,
    );

    let signal = serving.signal();
    let hb_logger = logger.clone();
    serving.spawn(async move {
        heartbeat(forwarder, signal, hb_logger).await;
    });

    logger.log(
        Level::Info,
        LogFields::msg("startup", format!("已就绪，心跳周期 {} 秒", HEARTBEAT_PERIOD.as_secs())),
    );
    serving.wait_and_drain(&state, &logger, SHUTDOWN_DRAIN_MS).await
}

/// 心跳循环。首帧立即发一次，之后按周期发；停机信号到即返回。
async fn heartbeat(
    forwarder: Forwarder,
    signal: ep_platform_runtime::shutdown::Shutdown,
    logger: Arc<JsonLogger>,
) {
    use ep_adapter_ipc::{ForwardOutcome, ReplayOutcome};

    let mut stop = Box::pin(signal.wait());
    loop {
        let pending = Pending {
            method: "system.ping".into(),
            payload: serde_json::json!({ "process": PROCESS.name(), "kind": "heartbeat" }),
        };
        let (replay, forward) = forwarder.send(&pending).await;
        match replay {
            ReplayOutcome::Nothing => {}
            ReplayOutcome::Replayed { count } => logger.log(
                Level::Info,
                LogFields::msg("spool", format!("恢复后补写 {count} 条并截断")),
            ),
            ReplayOutcome::Partial { ok, remaining, reason } => logger.log(
                Level::Warn,
                LogFields::msg("spool", format!("补写 {ok} 条后中断，剩余 {remaining} 条：{reason}")),
            ),
            ReplayOutcome::Broken { reason } => {
                logger.log(Level::Error, LogFields::msg("spool", format!("spool 不可用：{reason}")))
            }
        }
        match forward {
            ForwardOutcome::Sent => {}
            ForwardOutcome::Spooled { evicted, reason } => {
                if evicted > 0 {
                    logger.log(
                        Level::Error,
                        LogFields::msg("spool", format!("spool 超上限，丢弃最旧 {evicted} 条")),
                    );
                }
                logger.log(Level::Warn, LogFields::msg("spool", format!("心跳落盘：{reason}")));
            }
            ForwardOutcome::Lost { reason } => {
                logger.log(Level::Error, LogFields::msg("spool", format!("心跳既发不出也落不下：{reason}")))
            }
        }

        tokio::select! {
            _ = &mut stop => break,
            _ = tokio::time::sleep(HEARTBEAT_PERIOD) => {}
        }
    }
    logger.log(Level::Info, LogFields::msg("shutdown", "心跳循环已停止"));
}
