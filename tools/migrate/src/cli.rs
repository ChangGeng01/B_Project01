//! 参数解析。手写而不引 clap，与 `xtask/src/main.rs` 保持同一形态，也省掉一条
//! 随制品交付的依赖。
//!
//! 划分口径写在这里，免得后面各处再猜：选项「给了但取值形态不合法」算参数错误
//! （退出码 2，调用方能改），选项「压根没给且环境变量也没有」算环境自检失败
//! （退出码 78，要改的是这台机器的配置）。

use std::collections::BTreeSet;
use std::path::PathBuf;

use crate::options::apply_option;

/// 五个子命令，按裁定 C-02 冻结。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Subcommand {
    Apply,
    Status,
    Check,
    GenRls,
    OpenWindow,
}

pub const SUBCOMMANDS: [&str; 5] = ["apply", "status", "check", "gen-rls", "open-window"];

impl Subcommand {
    pub fn parse(s: &str) -> Option<Subcommand> {
        match s {
            "apply" => Some(Subcommand::Apply),
            "status" => Some(Subcommand::Status),
            "check" => Some(Subcommand::Check),
            "gen-rls" => Some(Subcommand::GenRls),
            "open-window" => Some(Subcommand::OpenWindow),
            _ => None,
        }
    }

    pub const fn name(self) -> &'static str {
        match self {
            Subcommand::Apply => "apply",
            Subcommand::Status => "status",
            Subcommand::Check => "check",
            Subcommand::GenRls => "gen-rls",
            Subcommand::OpenWindow => "open-window",
        }
    }

    /// 本子命令允许出现的选项。不在表内即参数错误，避免把选项写错了还照跑。
    pub const fn options(self) -> &'static [&'static str] {
        match self {
            Subcommand::Apply => &[
                "db-url",
                "migrations-dir",
                "history-schema",
                "history-table",
                "window-id",
                "expect-tool-version",
                "expect-manifest-sha256",
            ],
            Subcommand::Status => &[
                "db-url",
                "history-schema",
                "history-table",
                "format",
                "expect-tool-version",
            ],
            Subcommand::Check => &[
                "db-url",
                "migrations-dir",
                "history-schema",
                "history-table",
                "expect-tool-version",
                "expect-manifest-sha256",
            ],
            Subcommand::GenRls => &["schema", "table", "out"],
            Subcommand::OpenWindow => &["db-url", "ttl-minutes", "reason"],
        }
    }

    /// 是否需要一个数据库连接串。`gen-rls` 只按模板出 SQL，不连库。
    pub const fn needs_db(self) -> bool {
        !matches!(self, Subcommand::GenRls)
    }

    /// 是否需要读迁移目录。
    pub const fn needs_migrations_dir(self) -> bool {
        matches!(self, Subcommand::Apply | Subcommand::Check)
    }
}

/// `status --format` 的取值。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StatusFormat {
    Text,
    Json,
    /// 制品清单。IT-01 点名的取值，本阶段只保证它能被解析。
    Manifest,
}

/// 迁移历史表的 schema 与表名。计划第 4.3 节要求本阶段就在 CLI 骨架中固定这两
/// 个参数，使自检项 `migration-version-matched` 在空库上同样成立。
pub const DEFAULT_HISTORY_SCHEMA: &str = "platform_core";
pub const DEFAULT_HISTORY_TABLE: &str = "schema_history";
pub const DEFAULT_MIGRATIONS_DIR: &str = "db/migrations";
/// 迁移窗口默认与上限存活分钟数，取阶段 2 计划假设四。
pub const DEFAULT_TTL_MINUTES: u32 = 60;
pub const MAX_TTL_MINUTES: u32 = 240;
/// 未在命令行给出连接串时读取的环境变量，与配置模型的双下划线映射同形。
pub const DB_URL_ENV: &str = "EP__DB__URL";

/// 解析出来的一次调用。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Invocation {
    pub sub: Subcommand,
    pub db_url: Option<String>,
    pub migrations_dir: PathBuf,
    pub history_schema: String,
    pub history_table: String,
    pub window_id: Option<String>,
    pub expect_tool_version: Option<String>,
    pub expect_manifest_sha256: Option<String>,
    pub format: StatusFormat,
    pub ttl_minutes: u32,
    pub reason: Option<String>,
    pub rls_schema: Option<String>,
    pub rls_table: Option<String>,
    pub out: Option<PathBuf>,
}

/// 解析的三种结局。
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Parsed {
    /// 请求的是用法或版本，本身就是一次完成的调用。
    Print(String),
    Run(Box<Invocation>),
}

/// 参数错误，带一句给运维看的原因。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParseError(pub String);

pub(crate) fn err<T>(msg: impl Into<String>) -> Result<T, ParseError> {
    Err(ParseError(msg.into()))
}

/// 把 `--k=v` 与 `--k v` 两种写法归一成 (键, 值)。
fn take_option(
    args: &[String],
    idx: &mut usize,
) -> Result<(String, String), ParseError> {
    let raw = &args[*idx];
    let body = raw.strip_prefix("--").expect("调用方已确认以 -- 开头");
    if let Some((k, v)) = body.split_once('=') {
        *idx += 1;
        if k.is_empty() {
            return err(format!("选项名为空：{raw}"));
        }
        return Ok((k.to_string(), v.to_string()));
    }
    let key = body.to_string();
    match args.get(*idx + 1) {
        Some(v) if !v.starts_with("--") => {
            *idx += 2;
            Ok((key, v.clone()))
        }
        _ => err(format!("选项 --{key} 缺少取值")),
    }
}

/// 解析命令行。`args` 不含程序名。
pub fn parse(args: &[String]) -> Result<Parsed, ParseError> {
    if args.iter().any(|a| a == "-h" || a == "--help") {
        let sub = args.first().and_then(|a| Subcommand::parse(a));
        return Ok(Parsed::Print(crate::usage::usage(sub)));
    }
    if args.iter().any(|a| a == "-V" || a == "--version") {
        return Ok(Parsed::Print(crate::usage::version()));
    }

    let Some(head) = args.first() else {
        return err(format!(
            "缺少子命令；可用：{}。用 --help 看用法。",
            SUBCOMMANDS.join("、")
        ));
    };
    if head.starts_with('-') {
        return err(format!("未知全局选项 {head}；用 --help 看用法。"));
    }
    let Some(sub) = Subcommand::parse(head) else {
        return err(format!(
            "未知子命令 {head}；可用：{}。",
            SUBCOMMANDS.join("、")
        ));
    };

    let mut inv = Invocation {
        sub,
        db_url: None,
        migrations_dir: PathBuf::from(DEFAULT_MIGRATIONS_DIR),
        history_schema: DEFAULT_HISTORY_SCHEMA.to_string(),
        history_table: DEFAULT_HISTORY_TABLE.to_string(),
        window_id: None,
        expect_tool_version: None,
        expect_manifest_sha256: None,
        format: StatusFormat::Text,
        ttl_minutes: DEFAULT_TTL_MINUTES,
        reason: None,
        rls_schema: None,
        rls_table: None,
        out: None,
    };

    let mut seen: BTreeSet<String> = BTreeSet::new();
    let mut idx = 1usize;
    while idx < args.len() {
        let cur = &args[idx];
        if !cur.starts_with("--") {
            return err(format!("多余的位置参数 {cur}；本工具只收 --key=value 形态。"));
        }
        let (key, value) = take_option(args, &mut idx)?;
        if !sub.options().contains(&key.as_str()) {
            return err(format!(
                "子命令 {} 不接受选项 --{key}；它接受：{}。",
                sub.name(),
                sub.options()
                    .iter()
                    .map(|o| format!("--{o}"))
                    .collect::<Vec<_>>()
                    .join("、")
            ));
        }
        if !seen.insert(key.clone()) {
            return err(format!("选项 --{key} 重复出现。"));
        }
        apply_option(&mut inv, &key, &value)?;
    }

    if sub == Subcommand::GenRls {
        if inv.rls_schema.is_none() {
            return err("子命令 gen-rls 必须给出 --schema。");
        }
        if inv.rls_table.is_none() {
            return err("子命令 gen-rls 必须给出 --table。");
        }
    }

    Ok(Parsed::Run(Box::new(inv)))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn args(s: &[&str]) -> Vec<String> {
        s.iter().map(|x| x.to_string()).collect()
    }

    fn run(s: &[&str]) -> Result<Invocation, ParseError> {
        match parse(&args(s))? {
            Parsed::Run(inv) => Ok(*inv),
            Parsed::Print(_) => panic!("这组参数不该走用法分支"),
        }
    }

    #[test]
    fn five_subcommands_all_parse() {
        for name in SUBCOMMANDS {
            assert!(
                Subcommand::parse(name).is_some(),
                "子命令 {name} 必须可解析"
            );
        }
        assert_eq!(SUBCOMMANDS.len(), 5);
    }

    #[test]
    fn apply_defaults_freeze_history_table_params() {
        let inv = run(&["apply", "--db-url=postgres://h/ep"]).expect("可解析");
        assert_eq!(inv.history_schema, "platform_core");
        assert_eq!(inv.history_table, "schema_history");
        assert_eq!(inv.migrations_dir, PathBuf::from("db/migrations"));
    }

    #[test]
    fn status_format_manifest_parses() {
        let inv = run(&["status", "--db-url=postgres://h/ep", "--format=manifest"]).expect("可解析");
        assert_eq!(inv.format, StatusFormat::Manifest);
    }

    #[test]
    fn space_separated_form_parses() {
        let inv = run(&["open-window", "--db-url", "postgres://h/ep", "--ttl-minutes", "30"])
            .expect("可解析");
        assert_eq!(inv.ttl_minutes, 30);
    }

    #[test]
    fn gen_rls_requires_schema_and_table() {
        assert!(run(&["gen-rls", "--schema=mdm"]).is_err(), "缺 --table 必须报错");
        assert!(run(&["gen-rls", "--table=t"]).is_err(), "缺 --schema 必须报错");
        let inv = run(&["gen-rls", "--schema=mdm", "--table=parties"]).expect("可解析");
        assert_eq!(inv.rls_schema.as_deref(), Some("mdm"));
        assert_eq!(inv.rls_table.as_deref(), Some("parties"));
    }

    // 以下为负样例：每条断言的是解析规则本身会红，而不是某个辅助函数。

    #[test]
    fn negative_unknown_subcommand() {
        assert!(parse(&args(&["migrate"])).is_err(), "阶段 1 的旧名 migrate 已并入 apply，必须被拒");
        assert!(parse(&args(&["verify"])).is_err());
        assert!(parse(&args(&["manifest"])).is_err());
    }

    #[test]
    fn negative_no_subcommand() {
        assert!(parse(&[]).is_err());
    }

    #[test]
    fn negative_option_not_allowed_for_this_subcommand() {
        assert!(
            run(&["gen-rls", "--schema=mdm", "--table=t", "--db-url=postgres://h/ep"]).is_err(),
            "gen-rls 不连库，不该接受 --db-url"
        );
        assert!(
            run(&["status", "--db-url=postgres://h/ep", "--window-id=w1"]).is_err(),
            "只有 apply 出示迁移窗口"
        );
    }

    #[test]
    fn negative_duplicate_option() {
        assert!(run(&["status", "--format=text", "--format=json"]).is_err());
    }

    #[test]
    fn negative_missing_value() {
        assert!(run(&["status", "--db-url"]).is_err());
    }

    #[test]
    fn negative_positional_argument() {
        assert!(run(&["status", "extra"]).is_err());
    }

    #[test]
    fn negative_bad_values() {
        assert!(run(&["status", "--db-url=mysql://h/ep"]).is_err(), "非 pg 连接串");
        assert!(run(&["status", "--db-url=postgres://h/ep", "--format=xml"]).is_err());
        assert!(run(&["apply", "--db-url=postgres://h/ep", "--expect-manifest-sha256=dead"]).is_err());
        assert!(
            run(&["apply", "--db-url=postgres://h/ep", "--history-table=Schema_History"]).is_err(),
            "大写标识符必须被拒"
        );
        assert!(run(&["open-window", "--db-url=postgres://h/ep", "--ttl-minutes=0"]).is_err());
        assert!(run(&["open-window", "--db-url=postgres://h/ep", "--ttl-minutes=241"]).is_err());
        assert!(run(&["open-window", "--db-url=postgres://h/ep", "--ttl-minutes=x"]).is_err());
        assert!(run(&["apply", "--db-url=postgres://h/ep", "--window-id="]).is_err());
    }

    #[test]
    fn help_and_version_do_not_run_anything() {
        assert!(matches!(parse(&args(&["--help"])), Ok(Parsed::Print(_))));
        assert!(matches!(parse(&args(&["apply", "--help"])), Ok(Parsed::Print(_))));
        assert!(matches!(parse(&args(&["--version"])), Ok(Parsed::Print(_))));
    }
}
