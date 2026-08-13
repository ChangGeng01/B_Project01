//! ep-migrate — 迁移执行 CLI。
//!
//! 交付物 D-03。阶段 1 交付五个子命令的参数解析与 0、2、3、4、5、78 六个退出码
//! 的约定（均已冻结）；阶段 2（任务 #13）补齐五个子命令的实现体：
//! apply（事务执行器 + concurrent/ 非事务执行器）、status、check（db/checks
//! 十三个编号断言）、gen-rls（按 apply_le_rls 模板生成，不连库）、open-window。
//!
//! 结构性冲突记录：计划 §3.3 指定 refinery 0.8 Runner，但 refinery 的版本号
//! 是 i32（历史表 DDL version INT4），装不下本项目 V<YYYYMMDDHHMMSS> 的
//! 14 位时间戳版本号，属计划内部结构性矛盾；经 leader 批准，本工具自建
//! refinery 语义兼容 Runner，细节与兼容判据见 history.rs 模块头。
//!
//! 本二进制没有「悄悄返回 0」的路径：每一次调用要么完成动作并打印正文，
//! 要么带着六个退出码之一与一段说明失败。

mod apply;
mod bootstrap;
mod checks;
mod cli;
mod concurrent;
mod dbconn;
mod exec;
mod exit;
mod genrls;
mod history;
mod manifest;
mod options;
mod preflight;
mod sha256;
mod usage;
mod versions;
mod window;

use std::process::ExitCode;

use cli::Parsed;
use exit::{MigrateExit, Outcome};
use preflight::Stage;

/// 纯函数形态的入口：给定参数与环境变量取值，算出结论。测试直接调它，不必起进程。
fn decide(
    args: &[String],
    env_db_url: Option<&str>,
    env_db_dsn: Option<&str>,
    env_versions_path: Option<&str>,
    env_approval_ref: Option<&str>,
) -> Outcome {
    match cli::parse(args) {
        Err(e) => Outcome::Failed(
            MigrateExit::UsageError,
            format!("{}\n用 ep-migrate --help 看用法。", e.0),
        ),
        Ok(Parsed::Print(text)) => Outcome::Done(text),
        Ok(Parsed::Run(inv)) => match preflight::stage(&inv, env_db_url, env_db_dsn) {
            Stage::Settled(outcome) => outcome,
            Stage::NeedsDb(url) => exec::run(&inv, &url, env_versions_path, env_approval_ref),
        },
    }
}

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let env_db_url = std::env::var(cli::DB_URL_ENV).ok();
    let env_db_dsn = std::env::var(cli::DB_DSN_ENV).ok();
    let (env_versions_path, env_approval_ref) = exec::env_values();
    let outcome = decide(
        &args,
        env_db_url.as_deref(),
        env_db_dsn.as_deref(),
        env_versions_path.as_deref(),
        env_approval_ref.as_deref(),
    );
    match &outcome {
        Outcome::Done(text) => println!("{text}"),
        Outcome::Failed(e, msg) => {
            eprintln!("{msg}");
            eprintln!("退出码 {}（{}）。", e.code(), e.label());
        }
    }
    ExitCode::from(outcome.exit().code())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn args(s: &[&str]) -> Vec<String> {
        s.iter().map(|x| x.to_string()).collect()
    }

    #[test]
    fn help_and_version_succeed() {
        assert_eq!(
            decide(&args(&["--help"]), None, None, None, None).exit(),
            MigrateExit::Success
        );
        assert_eq!(
            decide(&args(&["--version"]), None, None, None, None).exit(),
            MigrateExit::Success
        );
    }

    #[test]
    fn unknown_subcommand_is_usage_error() {
        assert_eq!(
            decide(&args(&["frobnicate"]), None, None, None, None).exit(),
            MigrateExit::UsageError
        );
    }

    #[test]
    fn no_subcommand_never_succeeds() {
        assert_eq!(
            decide(&[], None, None, None, None).exit(),
            MigrateExit::UsageError
        );
    }

    #[test]
    fn gen_rls_end_to_end_without_db() {
        // gen-rls 是五个子命令里唯一无活库即可完成的完整路径。
        let out = decide(
            &args(&["gen-rls", "--schema=mdm", "--table=parties"]),
            None,
            None,
            None,
            None,
        );
        assert_eq!(out.exit(), MigrateExit::Success);
        match out {
            Outcome::Done(text) => assert!(text.contains("rls_parties_le")),
            _ => panic!(),
        }
    }
}
