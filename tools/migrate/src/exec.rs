//! 执行段入口：前置阶梯放行后，建运行时、连库、按子命令分派。
//!
//! 连接失败一律落 78（环境自检项 db-reachable）；各子命令的库侧判据
//! 分别落 3（窗口）、4（校验和）、5（版本）。本模块只做分派与运行时生命周期，
//! 不藏任何业务判据。

use crate::apply;
use crate::checks;
use crate::cli::{Invocation, Subcommand, APPROVAL_REF_ENV, EXPECTED_VERSIONS_PATH_ENV};
use crate::dbconn;
use crate::exit::Outcome;
use crate::window;

/// 同步入口：建当前线程运行时跑完异步段。
pub fn run(
    inv: &Invocation,
    url: &str,
    env_versions_path: Option<&str>,
    env_approval_ref: Option<&str>,
) -> Outcome {
    let runtime = match tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
    {
        Ok(rt) => rt,
        Err(e) => {
            return Outcome::Failed(
                crate::exit::MigrateExit::EnvSelfCheckFailed,
                format!("环境自检项 runtime-available 不通过：无法建立运行时：{e}"),
            );
        }
    };
    runtime.block_on(dispatch(inv, url, env_versions_path, env_approval_ref))
}

async fn dispatch(
    inv: &Invocation,
    url: &str,
    env_versions_path: Option<&str>,
    env_approval_ref: Option<&str>,
) -> Outcome {
    let mut client = match dbconn::connect(url).await {
        Ok(c) => c,
        Err(outcome) => return outcome,
    };
    let result = match inv.sub {
        Subcommand::Apply => apply::run_apply(&mut client, inv, env_versions_path).await,
        Subcommand::Status => apply::run_status(&client, inv).await,
        Subcommand::Check => {
            checks::run_check(&client, std::path::Path::new(checks::DEFAULT_CHECKS_DIR)).await
        }
        Subcommand::OpenWindow => match inv.reason.as_deref() {
            Some(reason) => {
                window::open_window(&mut client, inv.ttl_minutes, reason, env_approval_ref)
                    .await
                    .map(Outcome::Done)
            }
            None => {
                return Outcome::Failed(
                    crate::exit::MigrateExit::UsageError,
                    "open-window 必须给出 --reason（A-09 请求要素 reason ≤ 2000）。".to_string(),
                );
            }
        },
        Subcommand::GenRls => {
            unreachable!("gen-rls 不连库，前置阶梯已直接完成");
        }
    };
    match result {
        Ok(outcome) => outcome,
        Err(outcome) => outcome,
    }
}

/// 环境变量取值收集：把执行段需要的两个环境变量集中在一处读取。
pub fn env_values() -> (Option<String>, Option<String>) {
    (
        std::env::var(EXPECTED_VERSIONS_PATH_ENV).ok(),
        std::env::var(APPROVAL_REF_ENV).ok(),
    )
}
