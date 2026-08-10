//! ep-xtask — 结构门禁与文档校验工具。只在开发期运行，不进制品。
//!
//! 阶段 1 应交付十一个子命令：archcheck、sqlcheck、codecheck、errorcodes、
//! eventcatalog、configdoc、coverage、sbom、sign、reproduce、e2e。
//! 未实现的子命令一律以退出码 70 明确报「本阶段未交付」，不静默返回 0。

mod archcheck;
mod graph;

use std::path::PathBuf;
use std::process::ExitCode;

const SUBCOMMANDS: [&str; 11] = [
    "archcheck", "sqlcheck", "codecheck", "errorcodes", "eventcatalog",
    "configdoc", "coverage", "sbom", "sign", "reproduce", "e2e",
];

/// 未实现的子命令退出码。与「参数错误」的 2 区分，避免误读为通过。
const EXIT_NOT_DELIVERED: u8 = 70;

fn workspace_root() -> PathBuf {
    // xtask 的 CARGO_MANIFEST_DIR 是 <root>/xtask。
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("xtask 必须位于工作区根之下")
        .to_path_buf()
}

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let Some(cmd) = args.first().map(String::as_str) else {
        eprintln!("用法: cargo xtask <{}>", SUBCOMMANDS.join("|"));
        return ExitCode::from(2);
    };
    if !SUBCOMMANDS.contains(&cmd) {
        eprintln!("未知子命令 {cmd}；可用: {}", SUBCOMMANDS.join("、"));
        return ExitCode::from(2);
    }
    match cmd {
        "archcheck" => run_archcheck(),
        other => {
            eprintln!("子命令 {other} 在本阶段尚未交付（退出码 {EXIT_NOT_DELIVERED}）");
            ExitCode::from(EXIT_NOT_DELIVERED)
        }
    }
}

fn run_archcheck() -> ExitCode {
    let root = workspace_root();
    let report = match archcheck::run(&root) {
        Ok(r) => r,
        Err(e) => {
            eprintln!("archcheck 无法执行: {e}");
            return ExitCode::from(1);
        }
    };

    println!("archcheck 已判定 {} 条规则：", report.checked.len());
    for rule in &report.checked {
        let hits = report.violations.iter().filter(|v| &v.rule == rule).count();
        println!("  {:<28} {}", rule, if hits == 0 { "通过".into() } else { format!("{hits} 处违反") });
    }

    for (rule, why) in &report.undecidable {
        println!("  {rule:<28} 不可判定");
        println!("      {why}");
    }

    if !report.violations.is_empty() {
        eprintln!("\n违反明细（{} 处）：", report.violations.len());
        for v in &report.violations {
            eprintln!("  [{}] {} — {}", v.rule, v.package, v.detail);
        }
    }

    match report.outcome() {
        archcheck::Outcome::Clean => {
            println!("\narchcheck 通过。");
            ExitCode::SUCCESS
        }
        archcheck::Outcome::Undecidable => {
            eprintln!(
                "\narchcheck 有 {} 条规则当前不可判定（退出码 3）。不可判定不等于通过。",
                report.undecidable.len()
            );
            ExitCode::from(3)
        }
        archcheck::Outcome::Violated => {
            eprintln!("\narchcheck 未通过（退出码 1）。");
            ExitCode::from(1)
        }
    }
}
