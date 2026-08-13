//! `xtask ci` —— D-07 流水线的聚合入口，ADR-0005 决定二的落点。
//!
//! 读流水线登记表 `.github/ci/pipeline-stages.tsv`（流水线的唯一真值），逐行调度三类
//! 命令：cargo 以 cargo 直接执行，xtask 以 `cargo xtask` 执行，script 以仓库内脚本
//! 执行。判定逻辑收敛在本模块，不在 workflow 的 YAML 里表达：每条命令的退出码与登记
//! 表的门禁状态列互相核对——登记 delivered 却返回 70、或登记 undelivered 却不返回 70，
//! 一律判不符；返回 3 记为存在不可判定项，不得当作通过；返回 0 记通过。
//!
//! 聚合退出码取最重的一类：不符 1 > 不可判定 3 > 其余 0。D-07 的登记表把 17 条门禁
//! 全部登记为 delivered，因此「登记为 undelivered 且如实返回 70」本阶段没有实例；
//! 该形态退出码与登记表一致、不计不符，聚合码中不单独升级，只在汇总里如实印出。
//!
//! `.github/ci/run-pipeline.sh` 保留为备用调度器：读同一份登记表，行为与本入口逐行
//! 对等，只在需要排除 Rust 入口时启用。两处的退出码纪律同全仓一套：0 通过、1 违反、
//! 3 判定未做出（不得当作通过）、70 未交付。

use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, ExitCode};

/// 登记表的默认落点。可用环境变量 `EP_CI_ROSTER` 覆盖，与 run-pipeline.sh 一致。
pub const DEFAULT_ROSTER: &str = ".github/ci/pipeline-stages.tsv";

/// 参数错误的退出码。与 `main.rs`、`e2e` 的约定一致：2 表调用方参数错误。
const EXIT_USAGE: u8 = 2;

/// 退出码 3：判定未做出。计划第 10 节退出条件 27 明令 CI 不得当作通过。
const RC_UNDECIDABLE: i32 = 3;

/// 退出码 70：xtask 对未交付子命令的固定退出码。登记表的 undelivered 状态应与它相符。
const RC_UNDELIVERED: i32 = 70;

const USAGE: &str = "用法: cargo xtask ci [--stage <阶段号>] [--dry-run]";

/// 命令类别。登记表第三列的三个取值，其余一律解析失败。
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Kind {
    Cargo,
    Xtask,
    Script,
}

impl Kind {
    fn parse(token: &str) -> Result<Kind, String> {
        match token {
            "cargo" => Ok(Kind::Cargo),
            "xtask" => Ok(Kind::Xtask),
            "script" => Ok(Kind::Script),
            other => Err(format!("命令类别 {other} 不在 {{cargo, xtask, script}} 内")),
        }
    }

    fn label(self) -> &'static str {
        match self {
            Kind::Cargo => "cargo",
            Kind::Xtask => "xtask",
            Kind::Script => "script",
        }
    }
}

/// 门禁状态。登记表第五列的两个取值，其余一律解析失败。
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Status {
    Delivered,
    Undelivered,
}

impl Status {
    fn parse(token: &str) -> Result<Status, String> {
        match token {
            "delivered" => Ok(Status::Delivered),
            "undelivered" => Ok(Status::Undelivered),
            other => Err(format!(
                "门禁状态 {other} 不在 {{delivered, undelivered}} 内"
            )),
        }
    }
}

/// 登记表的一条命令。五列逐一落位，列数不对即解析失败。
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Row {
    pub stage: u32,
    pub id: String,
    pub kind: Kind,
    pub argv: Vec<String>,
    pub status: Status,
}

impl Row {
    /// 拼出完整命令行词元。script 类以首词元为相对仓库根的路径，与 run-pipeline.sh 同。
    pub fn command(&self, root: &Path) -> Vec<String> {
        match self.kind {
            Kind::Cargo => std::iter::once("cargo".to_string())
                .chain(self.argv.iter().cloned())
                .collect(),
            Kind::Xtask => ["cargo", "xtask"]
                .iter()
                .map(|s| s.to_string())
                .chain(self.argv.iter().cloned())
                .collect(),
            Kind::Script => {
                // parse_roster 保证 argv 非空，首词元必然存在。
                let (rel, rest) = self.argv.split_first().expect("登记表行的参数列非空");
                std::iter::once("bash".to_string())
                    .chain(std::iter::once(root.join(rel).display().to_string()))
                    .chain(rest.iter().cloned())
                    .collect()
            }
        }
    }

    pub fn cmdline(&self, root: &Path) -> String {
        self.command(root).join(" ")
    }
}

/// 登记表解析的两类失败。「读不到」与「读到了但不对」是两个退出码，不得合并。
#[derive(Debug)]
pub enum RosterError {
    /// 被测对象读不到：判定未做出，退出码 3。
    Unreadable(String),
    /// 读到了但内容失真：登记表本身违反格式，判不符，退出码 1。
    Invalid(String),
}

/// 从文件读登记表。路径经 `EP_CI_ROSTER` 覆盖时同样走这里。
pub fn load_roster(path: &Path) -> Result<Vec<Row>, RosterError> {
    let text = fs::read_to_string(path)
        .map_err(|e| RosterError::Unreadable(format!("登记表 {} 读不到：{e}", path.display())))?;
    parse_roster(&text).map_err(RosterError::Invalid)
}

/// 解析登记表文本：跳过 `#` 注释行与空行，其余每行必须是制表符分隔的五列。
pub fn parse_roster(text: &str) -> Result<Vec<Row>, String> {
    let mut rows = Vec::new();
    for (i, raw) in text.lines().enumerate() {
        let line = raw.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let no = i + 1;
        let fields: Vec<&str> = line.split('\t').collect();
        if fields.len() != 5 {
            return Err(format!(
                "第 {no} 行有 {} 列，登记表要求制表符分隔的五列",
                fields.len()
            ));
        }
        let stage: u32 = fields[0]
            .parse()
            .map_err(|_| format!("第 {no} 行首列 {} 不是阶段号", fields[0]))?;
        let argv: Vec<String> = fields[3].split_whitespace().map(str::to_string).collect();
        if argv.is_empty() {
            return Err(format!("第 {no} 行的命令参数列为空"));
        }
        rows.push(Row {
            stage,
            id: fields[1].to_string(),
            kind: Kind::parse(fields[2]).map_err(|e| format!("第 {no} 行：{e}"))?,
            argv,
            status: Status::parse(fields[4]).map_err(|e| format!("第 {no} 行：{e}"))?,
        });
    }
    if rows.is_empty() {
        return Err("登记表没有任何有效行".into());
    }
    Ok(rows)
}

/// 一条命令对照登记表后的结论。四种形态互不合并。
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Verdict {
    Pass,
    /// 退出码 3：判定未做出，不得当作通过。
    Undecidable,
    /// 登记为 undelivered 且如实返回 70：与登记表一致，不计不符也不计通过。
    Undelivered,
    /// 判不符，携带原因。
    Fail(String),
}

impl Verdict {
    pub fn text(&self) -> &str {
        match self {
            Verdict::Pass => "通过",
            Verdict::Undecidable => "存在不可判定项",
            Verdict::Undelivered => "本阶段未交付",
            Verdict::Fail(reason) => reason,
        }
    }
}

/// 单条判定。先看门禁状态列与退出码 70 是否互相矛盾，再看退出码本身的语义。
///
/// 登记失真比工具失败更重：标 delivered 却返回 70、或标 undelivered 却不返回 70，
/// 与 verify-pipeline-commands.sh 同判不符。
pub fn judge(status: Status, rc: i32) -> Verdict {
    if status == Status::Delivered && rc == RC_UNDELIVERED {
        return Verdict::Fail("登记为已交付却报未交付，登记表失真".into());
    }
    if status == Status::Undelivered && rc != RC_UNDELIVERED {
        return Verdict::Fail(format!("登记为未交付，退出码却是 {rc} 而不是 70"));
    }
    match rc {
        0 => Verdict::Pass,
        RC_UNDECIDABLE => Verdict::Undecidable,
        RC_UNDELIVERED => Verdict::Undelivered,
        other => Verdict::Fail(format!("不符（退出码 {other}）")),
    }
}

/// 逐阶段计数。汇总表与聚合退出码都从它出。
#[derive(Default, Debug)]
pub struct Tally {
    pub pass: usize,
    pub undelivered: usize,
    pub undecidable: usize,
    pub fail: usize,
}

impl Tally {
    fn bump(&mut self, verdict: &Verdict) {
        match verdict {
            Verdict::Pass => self.pass += 1,
            Verdict::Undecidable => self.undecidable += 1,
            Verdict::Undelivered => self.undelivered += 1,
            Verdict::Fail(_) => self.fail += 1,
        }
    }
}

/// 聚合退出码：有不符取 1，否则有不可判定取 3，否则 0。
///
/// 与 run-pipeline.sh 的差别只有一处：未交付不计聚合等级。D-07 的登记表 17 条全部
/// 标 delivered，未交付形态本阶段不存在；将来若出现，其退出码与登记表一致即不升级。
pub fn aggregate_exit(tally: &Tally) -> u8 {
    if tally.fail > 0 {
        1
    } else if tally.undecidable > 0 {
        3
    } else {
        0
    }
}

/// 子命令参数。
#[derive(Default, Clone, Copy, PartialEq, Eq, Debug)]
pub struct Options {
    /// `--stage <n>`：只跑某一个阶段。
    pub stage: Option<u32>,
    /// `--dry-run`：只打印将执行的命令，不执行。
    pub dry_run: bool,
}

pub fn parse_args(args: &[String]) -> Result<Options, String> {
    let mut opts = Options::default();
    let mut i = 0;
    while i < args.len() {
        let arg = args[i].as_str();
        if arg == "--dry-run" {
            opts.dry_run = true;
        } else if arg == "--stage" {
            let value = args
                .get(i + 1)
                .ok_or_else(|| "--stage 后面缺阶段号".to_string())?;
            opts.stage = Some(parse_stage(value)?);
            i += 1;
        } else if let Some(value) = arg.strip_prefix("--stage=") {
            opts.stage = Some(parse_stage(value)?);
        } else {
            return Err(format!("未知参数 {arg}"));
        }
        i += 1;
    }
    Ok(opts)
}

fn parse_stage(value: &str) -> Result<u32, String> {
    value
        .parse()
        .map_err(|_| format!("{value} 不是阶段号（须为正整数）"))
}

/// 子命令入口。`args` 是已剥去 `ci` 本身的剩余参数。
pub fn run(root: &Path, args: &[String]) -> ExitCode {
    let opts = match parse_args(args) {
        Ok(o) => o,
        Err(e) => {
            eprintln!("{e}");
            eprintln!("{USAGE}");
            return ExitCode::from(EXIT_USAGE);
        }
    };

    let roster = env::var_os("EP_CI_ROSTER")
        .map(PathBuf::from)
        .unwrap_or_else(|| root.join(DEFAULT_ROSTER));
    let rows = match load_roster(&roster) {
        Ok(v) => v,
        Err(RosterError::Unreadable(m)) => {
            eprintln!("{m}。判定未做出（退出码 3），不得当作通过。");
            return ExitCode::from(3);
        }
        Err(RosterError::Invalid(m)) => {
            eprintln!("登记表解析失败：{m}。");
            return ExitCode::from(1);
        }
    };

    let rows = match pick_stage(rows, opts.stage) {
        Ok(v) => v,
        Err(m) => {
            eprintln!("{m}。判定未做出（退出码 3），不得当作通过。");
            return ExitCode::from(3);
        }
    };

    if opts.dry_run {
        for row in &rows {
            println!(
                "阶段 {}（{}） {}  {}",
                row.stage,
                row.id,
                row.kind.label(),
                row.cmdline(root)
            );
        }
        println!("dry-run：以上 {} 条命令均未执行。", rows.len());
        return ExitCode::SUCCESS;
    }

    execute(root, &rows)
}

/// `--stage` 过滤。指定的阶段一条命令都没有时不得静默跑空集，报判定未做出。
fn pick_stage(rows: Vec<Row>, stage: Option<u32>) -> Result<Vec<Row>, String> {
    let Some(stage) = stage else {
        return Ok(rows);
    };
    let picked: Vec<Row> = rows.into_iter().filter(|r| r.stage == stage).collect();
    if picked.is_empty() {
        return Err(format!("阶段 {stage} 在登记表内没有任何命令"));
    }
    Ok(picked)
}

fn execute(root: &Path, rows: &[Row]) -> ExitCode {
    let mut tally = Tally::default();
    let mut lines: Vec<String> = Vec::new();
    for row in rows {
        let cmdline = row.cmdline(root);
        println!("::: 阶段 {}（{}） {}", row.stage, row.id, cmdline);
        let verdict = match run_command(row, root) {
            Ok(rc) => judge(row.status, rc),
            Err(e) => Verdict::Fail(e),
        };
        tally.bump(&verdict);
        lines.push(format!(
            "  阶段 {} {}  {}  →  {}",
            row.stage,
            row.id,
            cmdline,
            verdict.text()
        ));
    }
    summarize(&tally, &lines)
}

fn run_command(row: &Row, root: &Path) -> Result<i32, String> {
    let cmd = row.command(root);
    let (prog, rest) = cmd.split_first().expect("登记表行的命令词元非空");
    let status = Command::new(prog)
        .args(rest)
        .current_dir(root)
        .status()
        .map_err(|e| format!("命令 {prog} 启动失败：{e}，判不符"))?;
    status
        .code()
        .ok_or_else(|| "命令被信号中断，没有退出码，判不符".to_string())
}

/// 逐阶段汇总表，风格与 run-pipeline.sh 的输出对齐。
fn summarize(tally: &Tally, lines: &[String]) -> ExitCode {
    println!();
    println!("==== D-07 流水线汇总 ====");
    for line in lines {
        println!("{line}");
    }
    println!(
        "通过 {} 条，未交付 {} 条，不可判定 {} 条，不符 {} 条。",
        tally.pass, tally.undelivered, tally.undecidable, tally.fail
    );

    let code = aggregate_exit(tally);
    match code {
        1 => eprintln!("结论：有门禁判不符，流水线红（退出码 1）。"),
        3 => {
            eprintln!("结论：仍有不可判定项，按计划第 10 节退出条件 27，不得当作通过（退出码 3）。")
        }
        _ => {
            if tally.undelivered > 0 {
                println!(
                    "注：{} 条门禁本阶段未交付，退出码与登记表一致，未计入不符。",
                    tally.undelivered
                );
            }
            println!("结论：本次执行的全部命令与登记表一致。");
        }
    }
    ExitCode::from(code)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 登记表样本：含注释行、空行与三类命令各一条。
    const SAMPLE: &str = "\
# CI 流水线登记表样本。
# 列：阶段号 <TAB> 阶段 id <TAB> 命令类别 <TAB> 命令参数 <TAB> 门禁状态

1\ttoolchain-offline\tcargo\tmetadata --locked --offline\tdelivered
3\tarchcheck\txtask\tarchcheck\tdelivered
11\tdeploy-limits\tscript\tscripts/verify-resource-limits.sh\tdelivered
";

    fn row(stage: u32, id: &str, kind: Kind, argv: &[&str], status: Status) -> Row {
        Row {
            stage,
            id: id.into(),
            kind,
            argv: argv.iter().map(|s| s.to_string()).collect(),
            status,
        }
    }

    /// 临时登记表文件：写入后返回路径，测完自行清理。
    fn temp_roster(name: &str, content: &str) -> PathBuf {
        let path = std::env::temp_dir().join(format!(
            "ep-xtask-ci-test-{}-{name}.tsv",
            std::process::id()
        ));
        fs::write(&path, content).expect("写临时登记表失败");
        path
    }

    fn cleanup(path: &Path) {
        let _ = fs::remove_file(path);
    }

    #[test]
    fn parse_roster_reads_three_kinds() {
        let rows = parse_roster(SAMPLE).expect("样本应可解析");
        assert_eq!(rows.len(), 3, "注释行与空行不得计入");
        assert_eq!(
            rows[0],
            row(
                1,
                "toolchain-offline",
                Kind::Cargo,
                &["metadata", "--locked", "--offline"],
                Status::Delivered
            )
        );
        assert_eq!(rows[1].kind, Kind::Xtask);
        assert_eq!(rows[2].kind, Kind::Script);
    }

    /// 用临时文件走一遍 load_roster：文件内容与字符串解析必须同果。
    #[test]
    fn load_roster_from_temp_file() {
        let path = temp_roster("valid", SAMPLE);
        let rows = load_roster(&path).expect("临时登记表应可读可解析");
        assert_eq!(rows.len(), 3);
        cleanup(&path);
    }

    /// 读不到与读到了但不对必须是两类错误，对应两个退出码。
    #[test]
    fn load_roster_distinguishes_unreadable_from_invalid() {
        let missing = std::env::temp_dir().join(format!(
            "ep-xtask-ci-test-{}-missing.tsv",
            std::process::id()
        ));
        assert!(matches!(
            load_roster(&missing),
            Err(RosterError::Unreadable(_))
        ));

        let bad = temp_roster("invalid-four-columns", "1\tbuild\tcargo\tbuild\n");
        assert!(
            matches!(load_roster(&bad), Err(RosterError::Invalid(_)),),
            "四列不是五列，必须判失真"
        );
        cleanup(&bad);
    }

    #[test]
    fn parse_roster_rejects_malformed_lines() {
        // 阶段号不是数字。
        assert!(parse_roster("x\tbuild\tcargo\tbuild\tdelivered\n").is_err());
        // 命令类别不认识。
        assert!(parse_roster("1\tbuild\tdocker\tbuild\tdelivered\n").is_err());
        // 门禁状态不认识。
        assert!(parse_roster("1\tbuild\tcargo\tbuild\tmaybe\n").is_err());
        // 参数列为空。
        assert!(parse_roster("1\tbuild\tcargo\t\tdelivered\n").is_err());
        // 只有注释行：没有任何有效行。
        assert!(parse_roster("# 只有注释\n").is_err());
    }

    #[test]
    fn command_line_follows_kind() {
        let root = Path::new("/repo");
        let cargo = row(
            1,
            "s",
            Kind::Cargo,
            &["build", "--locked"],
            Status::Delivered,
        );
        assert_eq!(cargo.command(root), ["cargo", "build", "--locked"]);
        let xtask = row(3, "s", Kind::Xtask, &["archcheck"], Status::Delivered);
        assert_eq!(xtask.command(root), ["cargo", "xtask", "archcheck"]);
        let script = row(
            11,
            "s",
            Kind::Script,
            &["scripts/a.sh", "--x"],
            Status::Delivered,
        );
        assert_eq!(
            script.command(root),
            ["bash", "/repo/scripts/a.sh", "--x"],
            "script 首词元是相对仓库根的路径"
        );
    }

    #[test]
    fn judge_cross_checks_status_column() {
        // 登记已交付却返回 70：登记表失真，判不符。
        assert!(matches!(judge(Status::Delivered, 70), Verdict::Fail(_)));
        // 登记未交付却不返回 70：同样判不符，无论返回的是 0 还是 3。
        assert!(matches!(judge(Status::Undelivered, 0), Verdict::Fail(_)));
        assert!(matches!(judge(Status::Undelivered, 3), Verdict::Fail(_)));
        // 登记未交付且如实返回 70：一致，不计不符。
        assert_eq!(judge(Status::Undelivered, 70), Verdict::Undelivered);
    }

    #[test]
    fn judge_follows_exit_code_semantics() {
        assert_eq!(judge(Status::Delivered, 0), Verdict::Pass);
        assert_eq!(
            judge(Status::Delivered, 3),
            Verdict::Undecidable,
            "退出码 3 不得当作通过"
        );
        assert!(matches!(judge(Status::Delivered, 1), Verdict::Fail(_)));
        assert!(matches!(judge(Status::Delivered, 101), Verdict::Fail(_)));
    }

    #[test]
    fn aggregate_exit_takes_heaviest_class() {
        let mut t = Tally::default();
        assert_eq!(aggregate_exit(&t), 0, "空汇总不得报不符");

        t.undelivered = 2;
        assert_eq!(aggregate_exit(&t), 0, "与登记表一致的未交付不升级聚合码");

        t.undecidable = 1;
        assert_eq!(aggregate_exit(&t), 3, "不可判定压过全通过");

        t.fail = 1;
        assert_eq!(aggregate_exit(&t), 1, "不符压过不可判定");
    }

    #[test]
    fn pick_stage_filters_and_rejects_absent_stage() {
        let rows = parse_roster(SAMPLE).expect("样本应可解析");
        let one = pick_stage(rows.clone(), Some(3)).expect("阶段 3 存在");
        assert_eq!(one.len(), 1);
        assert_eq!(one[0].id, "archcheck");
        assert!(
            pick_stage(rows, Some(99)).is_err(),
            "不存在的阶段不得静默跑空集"
        );
    }

    #[test]
    fn parse_args_accepts_documented_forms() {
        let opts = parse_args(&[]).expect("无参数合法");
        assert_eq!(opts, Options::default());

        let opts = parse_args(&["--dry-run".to_string()]).expect("dry-run 合法");
        assert!(opts.dry_run);

        let opts = parse_args(&["--stage".to_string(), "5".to_string()]).expect("两段式合法");
        assert_eq!(opts.stage, Some(5));

        let opts = parse_args(&["--stage=5".to_string()]).expect("等号式合法");
        assert_eq!(opts.stage, Some(5));

        let opts = parse_args(&[
            "--stage".to_string(),
            "5".to_string(),
            "--dry-run".to_string(),
        ])
        .expect("可组合");
        assert_eq!(
            opts,
            Options {
                stage: Some(5),
                dry_run: true
            }
        );
    }

    /// 负样例：未知参数、缺阶段号、阶段号非数字都必须报参数错误，不得静默忽略。
    #[test]
    fn parse_args_rejects_bad_input() {
        assert!(parse_args(&["--fast".to_string()]).is_err());
        let e = parse_args(&["--stage".to_string()]).unwrap_err();
        assert!(e.contains("缺阶段号"), "{e}");
        let e = parse_args(&["--stage=abc".to_string()]).unwrap_err();
        assert!(e.contains("不是阶段号"), "{e}");
    }
}
