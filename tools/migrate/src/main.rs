//! ep-migrate — 迁移执行 CLI。
//!
//! 交付物 D-03。本阶段只交付两样东西：五个子命令 apply、status、check、gen-rls、
//! open-window 的参数解析，以及 0、2、3、4、5、78 六个退出码的约定。五个子命令
//! 的实现体按裁定 C-01 与 C-02 由阶段 2 交付。
//!
//! 本二进制没有任何返回 0 的迁移路径：`--help` 与 `--version` 之外的每一次调用
//! 都以非零退出码收尾，并在 stderr 上写清楚是哪道判据拦下的、哪些判据本阶段
//! 根本没有被测对象。

mod cli;
mod exit;
mod manifest;
mod options;
mod preflight;
mod sha256;
mod usage;

use std::process::ExitCode;

use cli::Parsed;
use exit::{MigrateExit, Outcome};

/// 纯函数形态的入口：给定参数与环境变量取值，算出结论。测试直接调它，不必起进程。
fn decide(args: &[String], env_db_url: Option<&str>) -> Outcome {
    match cli::parse(args) {
        Err(e) => Outcome::Failed(
            MigrateExit::UsageError,
            format!("{}\n用 ep-migrate --help 看用法。", e.0),
        ),
        Ok(Parsed::Print(text)) => Outcome::Done(text),
        Ok(Parsed::Run(inv)) => preflight::run(&inv, env_db_url),
    }
}

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let env_db_url = std::env::var(cli::DB_URL_ENV).ok();
    let outcome = decide(&args, env_db_url.as_deref());
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
        assert_eq!(decide(&args(&["--help"]), None).exit(), MigrateExit::Success);
        assert_eq!(decide(&args(&["--version"]), None).exit(), MigrateExit::Success);
    }

    #[test]
    fn unknown_subcommand_is_usage_error() {
        assert_eq!(decide(&args(&["frobnicate"]), None).exit(), MigrateExit::UsageError);
    }

    #[test]
    fn no_subcommand_never_succeeds() {
        assert_eq!(decide(&[], None).exit(), MigrateExit::UsageError);
    }
}
