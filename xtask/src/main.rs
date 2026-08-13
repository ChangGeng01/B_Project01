//! ep-xtask — 结构门禁与文档校验工具。只在开发期运行，不进制品。
//!
//! 十二个子命令：archcheck、sqlcheck、codecheck、errorcodes、eventcatalog、
//! configdoc、coverage、sbom、sign、reproduce、e2e 与流水线聚合入口 ci，
//! 本阶段全部交付。
//!
//! 退出码三态互不合并：0 通过、1 有违反、3 判定未做出（被测对象不存在或所需工具缺失）。
//! 「判定未做出」既不得表达为通过也不得表达为违反——阶段 1 计划第 10 节退出条件 27
//! 与技术基线第 12 节通则第六条的硬要求。
//! 退出码 70 保留给将来可能出现的未交付子命令，本阶段无一命中。

mod archcheck;
mod ci;
mod codecheck;
mod configdoc;
mod coverage;
mod e2e;
mod errorcodes;
mod eventcatalog;
mod graph;
mod reproduce;
mod sbom;
mod sign;
mod sqlcheck;

use std::path::PathBuf;
use std::process::ExitCode;

const SUBCOMMANDS: [&str; 12] = [
    "archcheck",
    "sqlcheck",
    "codecheck",
    "errorcodes",
    "eventcatalog",
    "configdoc",
    "coverage",
    "sbom",
    "sign",
    "reproduce",
    "e2e",
    "ci",
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
        "ci" => ci::run(&workspace_root(), &args[1..]),
        "errorcodes" => run_errorcodes(),
        "e2e" => e2e::run(&args[1..]),
        "sqlcheck" => report(cmd, sqlcheck::run(&workspace_root())),
        "codecheck" => report(cmd, codecheck::run(&workspace_root())),
        "eventcatalog" => report(cmd, eventcatalog::run(&workspace_root())),
        "configdoc" => {
            let root = workspace_root();
            if args.iter().any(|a| a == "--check-doc-type-codes") {
                report(cmd, configdoc::run_doc_type_codes(&root))
            } else {
                report(cmd, configdoc::run(&root))
            }
        }
        "coverage" => report(cmd, coverage::run(&workspace_root())),
        "sbom" => report(cmd, sbom::run(&workspace_root())),
        "sign" => report(cmd, sign::run(&workspace_root())),
        "reproduce" => report(cmd, reproduce::run(&workspace_root())),
        other => {
            eprintln!("子命令 {other} 在本阶段尚未交付（退出码 {EXIT_NOT_DELIVERED}）");
            ExitCode::from(EXIT_NOT_DELIVERED)
        }
    }
}

/// 八个子命令的共同判定面。
///
/// 各模块自己判 `outcome()`，本 trait 只把三种结论归一并取出明细与计数——
/// 判定权留在模块里，退出码的映射收在一处。
enum Verdict {
    Clean,
    /// 判定未做出：被测对象不存在，或所需工具不可用。
    /// **不得并进 problems，也不得当作通过。**
    Unmade,
    Violated,
}

trait Judged {
    fn problems(&self) -> &[String];
    fn unmade(&self) -> &[String];
    fn notes(&self) -> &[String] {
        &[]
    }
    fn verdict(&self) -> Verdict;
    fn summary(&self) -> String;
}

/// 前四个子命令的 Report 用 `uncovered` 表未覆盖，各自有一个本地 `Outcome`。
macro_rules! judged_uncovered {
    ($($m:ident => $summary:expr),* $(,)?) => {
        $(impl Judged for $m::Report {
            fn problems(&self) -> &[String] { &self.problems }
            fn unmade(&self) -> &[String] { &self.uncovered }
            fn verdict(&self) -> Verdict {
                match self.outcome() {
                    $m::Outcome::Clean => Verdict::Clean,
                    $m::Outcome::Uncovered => Verdict::Unmade,
                    $m::Outcome::Violated => Verdict::Violated,
                }
            }
            fn summary(&self) -> String { let f: fn(&Self) -> String = $summary; f(self) }
        })*
    };
}

/// 后四个子命令的 Report 用 `undecidable`，共用 `archcheck::Outcome`。
macro_rules! judged_undecidable {
    ($($m:ident),* $(,)?) => {
        $(impl Judged for $m::Report {
            fn problems(&self) -> &[String] { &self.problems }
            fn unmade(&self) -> &[String] { &self.undecidable }
            fn notes(&self) -> &[String] { &self.notes }
            fn verdict(&self) -> Verdict {
                match self.outcome() {
                    archcheck::Outcome::Clean => Verdict::Clean,
                    archcheck::Outcome::Undecidable => Verdict::Unmade,
                    archcheck::Outcome::Violated => Verdict::Violated,
                }
            }
            fn summary(&self) -> String {
                format!("{} 项判定未做出", self.undecidable.len())
            }
        })*
    };
}

judged_uncovered! {
    sqlcheck => |r| format!("{} 条规则，扫过 {} 个迁移文件", r.checked.len(), r.scanned_files),
    codecheck => |r| format!("核过 {} 项", r.checked),
    eventcatalog => |r| format!("比对 {} 个事件类型", r.compared),
    configdoc => |r| format!("比对 {} 个配置键，{} 项按裁定推迟", r.compared, r.deferred.len()),
}

judged_undecidable!(coverage, sbom, sign, reproduce);

/// 三态输出，与 archcheck 同一套：0 通过、1 有违反、3 判定未做出。
///
/// 三者互不合并——「代码违规」「被测对象不存在」「判据没写出来」是三件不同的事，
/// 任何一件不得被读成另一件。技术基线第 12 节通则第六条的硬要求。
fn report(name: &str, r: impl Judged) -> ExitCode {
    println!("{name}：{}。", r.summary());
    for n in r.notes() {
        println!("  {n}");
    }
    if !r.problems().is_empty() {
        eprintln!("\n不符（{} 处）：", r.problems().len());
        for p in r.problems() {
            eprintln!("  {p}");
        }
    }
    if !r.unmade().is_empty() {
        println!("\n判定未做出（{} 项）：", r.unmade().len());
        for u in r.unmade() {
            println!("  {u}");
        }
    }
    match r.verdict() {
        Verdict::Violated => {
            eprintln!("\n{name} 未通过（退出码 1）。");
            ExitCode::from(1)
        }
        Verdict::Unmade => {
            eprintln!("\n{name}：判定未做出不等于通过（退出码 3）。");
            ExitCode::from(3)
        }
        Verdict::Clean => {
            println!("{name} 通过。");
            ExitCode::SUCCESS
        }
    }
}

fn run_errorcodes() -> ExitCode {
    let report = errorcodes::run(&workspace_root());
    println!("errorcodes 比对了 {} 条错误码。", report.compared);
    if report.ok() {
        println!("docs/error-codes.md 与代码常量表逐项一致。");
        return ExitCode::SUCCESS;
    }
    eprintln!("\n不一致（{} 处）：", report.problems.len());
    for p in &report.problems {
        eprintln!("  {p}");
    }
    ExitCode::from(1)
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
        println!(
            "  {:<28} {}",
            rule,
            if hits == 0 {
                "通过".into()
            } else {
                format!("{hits} 处违反")
            }
        );
    }

    if !archcheck::DELEGATED.is_empty() {
        println!("\n已裁定不由本工具判定的判据（裁定 F-03，基线第 12.1 节 delegated 段）：");
        for (rule, taker) in archcheck::DELEGATED {
            println!("  {rule}");
            println!("      {taker}");
        }
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
