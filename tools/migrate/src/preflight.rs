//! 前置判定阶梯。
//!
//! 顺序是：环境段（78）→ 窗口闸（3）→ 版本闸（5）→ 清单闸（4）→ 能力段（78）。
//!
//! 能力段之所以排在三个闸之后而不是最前面，是因为本阶段五个子命令的实现体全部
//! 归阶段 2，能力段一旦前置，3、4、5 三个码在本阶段就恒不可达——那正是基线第
//! 12 节通则第六条禁止的「以恒不可满足的形态留下判据」。三个闸判的是调用方出
//! 示的凭据，本来就先于「这台机器上能不能干这件事」。
//!
//! 凡是本阶段读不到被测对象的判据，一律在报告里写「未覆盖」，不写「通过」。

use std::path::Path;

use crate::cli::{Invocation, Subcommand, DB_URL_ENV};
use crate::exit::{MigrateExit, Outcome, NOT_DELIVERED};
use crate::manifest::manifest_sha256;

/// 本制品自身的版本，用于版本闸比对。
pub const TOOL_VERSION: &str = env!("CARGO_PKG_VERSION");

/// 一道判据在本次调用里的结论。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Verdict {
    Passed,
    /// 本阶段没有被测对象，或调用方没有出示期望值。不等于通过。
    NotCovered,
}

impl Verdict {
    fn text(self) -> &'static str {
        match self {
            Verdict::Passed => "通过",
            Verdict::NotCovered => "未覆盖",
        }
    }
}

/// 环境段与三个闸逐条的结论，最后随未交付说明一起打印。
struct Ladder {
    lines: Vec<String>,
}

impl Ladder {
    fn new() -> Ladder {
        Ladder { lines: Vec::new() }
    }

    fn note(&mut self, name: &str, verdict: Verdict, detail: &str) {
        self.lines
            .push(format!("  {:<26} {}  {}", name, verdict.text(), detail));
    }

    fn render(&self) -> String {
        self.lines.join("\n")
    }
}

/// 解析连接串：命令行优先，其次环境变量。返回 None 表示两处都没有。
fn resolve_db_url(inv: &Invocation, env_db_url: Option<&str>) -> Option<String> {
    inv.db_url
        .clone()
        .or_else(|| env_db_url.map(|s| s.to_string()))
}

fn fail(exit: MigrateExit, msg: String) -> Outcome {
    Outcome::Failed(exit, msg)
}

/// 跑完整条阶梯。本阶段任何 `Run` 形态的调用都以非零码收尾：要么被某道闸拦下，
/// 要么走到能力段被 `subcommand-implemented` 拦下。没有返回 0 的路径。
pub fn run(inv: &Invocation, env_db_url: Option<&str>) -> Outcome {
    let mut ladder = Ladder::new();

    // 环境段一：连接串可解析。
    if inv.sub.needs_db() {
        match resolve_db_url(inv, env_db_url) {
            None => {
                return fail(
                    MigrateExit::EnvSelfCheckFailed,
                    format!(
                        "环境自检项 db-url-resolvable 不通过：子命令 {} 需要数据库连接串，\
                         命令行未给 --db-url，环境变量 {DB_URL_ENV} 也未设置。",
                        inv.sub.name()
                    ),
                );
            }
            Some(url) if !url.starts_with("postgres") => {
                return fail(
                    MigrateExit::EnvSelfCheckFailed,
                    format!(
                        "环境自检项 db-url-resolvable 不通过：环境变量 {DB_URL_ENV} 的取值\
                         不是 postgresql:// 或 postgres:// 开头的连接串。"
                    ),
                );
            }
            Some(_) => ladder.note(
                "db-url-resolvable",
                Verdict::Passed,
                "已取到连接串；本阶段不建立连接，连通性判定归阶段 2。",
            ),
        }
    }

    // 环境段二：迁移目录可读。
    if inv.sub.needs_migrations_dir() {
        let dir: &Path = inv.migrations_dir.as_path();
        if !dir.is_dir() {
            return fail(
                MigrateExit::EnvSelfCheckFailed,
                format!(
                    "环境自检项 migrations-dir-readable 不通过：迁移目录 {} 不存在或不是目录。",
                    dir.display()
                ),
            );
        }
        if let Err(e) = std::fs::read_dir(dir) {
            return fail(
                MigrateExit::EnvSelfCheckFailed,
                format!(
                    "环境自检项 migrations-dir-readable 不通过：迁移目录 {} 不可读：{e}",
                    dir.display()
                ),
            );
        }
        ladder.note(
            "migrations-dir-readable",
            Verdict::Passed,
            &format!("{}", dir.display()),
        );
    }

    // 窗口闸：只有 apply 受约束。
    if inv.sub == Subcommand::Apply {
        match &inv.window_id {
            None => {
                return fail(
                    MigrateExit::MigrationWindowClosed,
                    "迁移窗口未打开：apply 必须以 --window-id 出示一个已打开的迁移窗口。\
                     窗口由 open-window 开启，登记在 platform_core.migration_windows。"
                        .to_string(),
                );
            }
            Some(id) => ladder.note(
                "migration-window-presented",
                Verdict::NotCovered,
                &format!("已出示 {id}；该窗口是否真的处于 OPEN 需要读库，归阶段 2。"),
            ),
        }
    }

    // 版本闸。
    match &inv.expect_tool_version {
        None => ladder.note(
            "tool-version-matched",
            Verdict::NotCovered,
            "调用方未出示 --expect-tool-version。",
        ),
        Some(expected) if expected != TOOL_VERSION => {
            return fail(
                MigrateExit::VersionMismatch,
                format!(
                    "版本不一致：调用方期望 {expected}，本制品自身版本为 {TOOL_VERSION}。"
                ),
            );
        }
        Some(_) => ladder.note("tool-version-matched", Verdict::Passed, TOOL_VERSION),
    }
    ladder.note(
        "schema-history-version-matched",
        Verdict::NotCovered,
        "库侧 schema_history 版本比对需要读库，归阶段 2。",
    );

    // 清单闸。
    match &inv.expect_manifest_sha256 {
        None => ladder.note(
            "migration-manifest-matched",
            Verdict::NotCovered,
            "调用方未出示 --expect-manifest-sha256。",
        ),
        Some(expected) => match manifest_sha256(inv.migrations_dir.as_path()) {
            Err(e) => {
                return fail(
                    MigrateExit::EnvSelfCheckFailed,
                    format!("环境自检项 migrations-dir-readable 不通过：{e}"),
                );
            }
            Ok(actual) if &actual != expected => {
                return fail(
                    MigrateExit::ChecksumMismatch,
                    format!(
                        "校验和不符：迁移目录 {} 实算清单哈希为 {actual}，调用方期望 {expected}。",
                        inv.migrations_dir.display()
                    ),
                );
            }
            Ok(actual) => ladder.note("migration-manifest-matched", Verdict::Passed, &actual),
        },
    }

    // 能力段。
    fail(
        NOT_DELIVERED,
        format!(
            "环境自检项 subcommand-implemented 不通过：子命令 {} 的实现体按裁定 C-01 与 C-02 \
             由阶段 2 交付，本制品只含参数解析与退出码约定。\n\
             前置阶梯逐条结论：\n{}\n\
             注意：「未覆盖」不等于通过。本次调用没有执行任何迁移动作。",
            inv.sub.name(),
            ladder.render()
        ),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cli::{parse, Parsed};
    use std::fs;
    use std::path::PathBuf;

    fn inv(argv: &[&str]) -> Invocation {
        let args: Vec<String> = argv.iter().map(|s| s.to_string()).collect();
        match parse(&args).expect("这组参数应当可解析") {
            Parsed::Run(i) => *i,
            Parsed::Print(_) => panic!("这组参数不该走用法分支"),
        }
    }

    fn probe_dir(tag: &str) -> PathBuf {
        let mut p = std::env::temp_dir();
        p.push(format!(
            "ep-migrate-preflight-{tag}-{}-{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .map(|d| d.as_nanos())
                .unwrap_or_default()
        ));
        fs::create_dir_all(p.join("platform_core")).expect("建探针目录");
        fs::write(
            p.join("platform_core/V001__probe.sql"),
            "-- rollback: drop table ci_probe.probe_records;\ncreate table t();\n",
        )
        .expect("写探针迁移文件");
        p
    }

    #[test]
    fn missing_db_url_fails_env_selfcheck() {
        let out = run(&inv(&["status"]), None);
        assert_eq!(out.exit(), MigrateExit::EnvSelfCheckFailed);
    }

    #[test]
    fn env_var_supplies_db_url() {
        // 负样例的对照面：环境变量给了连接串就不该再落 db-url-resolvable。
        let out = run(&inv(&["status"]), Some("postgres://h/ep"));
        assert_eq!(out.exit(), MigrateExit::EnvSelfCheckFailed);
        match out {
            Outcome::Failed(_, msg) => {
                assert!(msg.contains("subcommand-implemented"), "应当走到能力段：{msg}");
                assert!(!msg.contains("db-url-resolvable 不通过"));
            }
            Outcome::Done(_) => panic!("本阶段不得返回成功"),
        }
    }

    #[test]
    fn missing_migrations_dir_fails_env_selfcheck() {
        let out = run(&inv(&["check", "--db-url=postgres://h/ep"]), None);
        match out {
            Outcome::Failed(e, msg) => {
                assert_eq!(e, MigrateExit::EnvSelfCheckFailed);
                assert!(msg.contains("migrations-dir-readable"), "{msg}");
            }
            Outcome::Done(_) => panic!("默认迁移目录不存在时不得返回成功"),
        }
    }

    #[test]
    fn apply_without_window_id_is_window_closed() {
        let dir = probe_dir("window");
        let a = inv(&[
            "apply",
            "--db-url=postgres://h/ep",
            &format!("--migrations-dir={}", dir.display()),
        ]);
        assert_eq!(run(&a, None).exit(), MigrateExit::MigrationWindowClosed);

        // 负样例的对照面：出示窗口后不再落 3，改由能力段拦下。
        let b = inv(&[
            "apply",
            "--db-url=postgres://h/ep",
            &format!("--migrations-dir={}", dir.display()),
            "--window-id=w-1",
        ]);
        assert_eq!(run(&b, None).exit(), MigrateExit::EnvSelfCheckFailed);
        fs::remove_dir_all(&dir).ok();
    }

    #[test]
    fn wrong_expected_version_is_version_mismatch() {
        let dir = probe_dir("version");
        let a = inv(&[
            "check",
            "--db-url=postgres://h/ep",
            &format!("--migrations-dir={}", dir.display()),
            "--expect-tool-version=9.9.9",
        ]);
        assert_eq!(run(&a, None).exit(), MigrateExit::VersionMismatch);

        let b = inv(&[
            "check",
            "--db-url=postgres://h/ep",
            &format!("--migrations-dir={}", dir.display()),
            &format!("--expect-tool-version={TOOL_VERSION}"),
        ]);
        assert_eq!(run(&b, None).exit(), MigrateExit::EnvSelfCheckFailed);
        fs::remove_dir_all(&dir).ok();
    }

    #[test]
    fn tampered_probe_dir_is_checksum_mismatch() {
        let dir = probe_dir("checksum");
        let good = manifest_sha256(&dir).expect("可计算");
        let a = inv(&[
            "check",
            "--db-url=postgres://h/ep",
            &format!("--migrations-dir={}", dir.display()),
            &format!("--expect-manifest-sha256={good}"),
        ]);
        assert_eq!(
            run(&a, None).exit(),
            MigrateExit::EnvSelfCheckFailed,
            "哈希相符时不得落 4"
        );

        fs::write(
            dir.join("platform_core/V001__probe.sql"),
            "-- rollback: drop table ci_probe.probe_records;\ncreate table t2();\n",
        )
        .expect("篡改探针文件");
        assert_eq!(run(&a, None).exit(), MigrateExit::ChecksumMismatch);
        fs::remove_dir_all(&dir).ok();
    }

    #[test]
    fn ladder_never_reports_passed_for_uncovered_items() {
        let dir = probe_dir("ladder");
        let a = inv(&[
            "check",
            "--db-url=postgres://h/ep",
            &format!("--migrations-dir={}", dir.display()),
        ]);
        match run(&a, None) {
            Outcome::Failed(e, msg) => {
                assert_eq!(e, MigrateExit::EnvSelfCheckFailed);
                assert!(msg.contains("schema-history-version-matched 未覆盖"), "{msg}");
                assert!(msg.contains("migration-manifest-matched 未覆盖"), "{msg}");
            }
            Outcome::Done(_) => panic!("本阶段不得返回成功"),
        }
        fs::remove_dir_all(&dir).ok();
    }
}
