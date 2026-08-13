//! 前置判定阶梯（同步段）。
//!
//! 顺序是：环境段（78）→ 窗口出示登记（未出示不再落 3，首装自举与非空库
//! 窗口闸判据移入执行段，见 bootstrap.rs 与 apply.rs）→ 版本闸（5）→ 清单闸（4）。
//! 库侧判据——连接可达、窗口真 OPEN、库侧版本比对、合规断言——都需要读库，
//! 放在执行段（exec.rs 起）承接；本阶梯只判调用方出示的凭据与本机可读的文件。
//!
//! 阶梯放行后返回 [`Stage::NeedsDb`]，由 main 交执行段连库执行；`gen-rls`
//! 不连库，直接在阶梯内完成。凡是读不到被测对象的判据，报告里写「未覆盖」，
//! 不写「通过」。

use std::path::Path;

use crate::cli::{Invocation, Subcommand};
use crate::dbconn;
use crate::exit::{MigrateExit, Outcome};
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

/// 环境与各闸逐条的结论，随放行说明或失败说明一起打印。
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

/// 阶梯的两种结局：就地了结（失败或 gen-rls 完成），或放行去执行段。
#[derive(Debug)]
pub enum Stage {
    Settled(Outcome),
    /// 前置阶梯全部放行，括号内是解析出的连接串。
    NeedsDb(String),
}

impl Stage {
    /// 供测试断言用的退出码视图：NeedsDb 没有退出码，返回 None。
    #[cfg(test)]
    pub fn exit(&self) -> Option<MigrateExit> {
        match self {
            Stage::Settled(o) => Some(o.exit()),
            Stage::NeedsDb(_) => None,
        }
    }
}

fn fail(exit: MigrateExit, msg: String) -> Stage {
    Stage::Settled(Outcome::Failed(exit, msg))
}

/// 跑完整条同步阶梯。
pub fn stage(inv: &Invocation, env_db_url: Option<&str>, env_db_dsn: Option<&str>) -> Stage {
    let mut ladder = Ladder::new();

    // 环境段一：连接串可解析（gen-rls 不需要）。
    let mut url: Option<String> = None;
    if inv.sub.needs_db() {
        match dbconn::resolve_db_url(inv, env_db_url, env_db_dsn) {
            Err(outcome) => return Stage::Settled(outcome),
            Ok(resolved) => {
                ladder.note(
                    "db-url-resolvable",
                    Verdict::Passed,
                    "已取到连接串；连通性判定在执行段连库时做。",
                );
                url = Some(resolved);
            }
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

    // 窗口出示闸：只有 apply 受约束；窗口是否真 OPEN 由执行段读库判定。
    // 未出示窗口不再在本阶梯落 3：目标库无历史表时执行段进首装自举
    // （bootstrap.rs，02 计划第 12 节偏离登记十四），非空库的窗口闸判据
    // 同样移入执行段（无待执行退出码 0，有待执行落 3），因为是否首装
    // 必须探测库后才能判定，未过闸前不触库的口径自此限缩为不探历史表。
    if inv.sub == Subcommand::Apply {
        match &inv.window_id {
            None => ladder.note(
                "migration-window-presented",
                Verdict::NotCovered,
                "未出示 --window-id：目标库无历史表时执行段进首装自举，\
                 否则由执行段读库判窗口闸。",
            ),
            Some(id) => ladder.note(
                "migration-window-presented",
                Verdict::Passed,
                &format!("已出示 {id}；执行段将读 migration_windows 判定 OPEN。"),
            ),
        }
    }

    // 版本闸一：工具自身版本。
    match &inv.expect_tool_version {
        None => ladder.note(
            "tool-version-matched",
            Verdict::NotCovered,
            "调用方未出示 --expect-tool-version。",
        ),
        Some(expected) if expected != TOOL_VERSION => {
            return fail(
                MigrateExit::VersionMismatch,
                format!("版本不一致：调用方期望 {expected}，本制品自身版本为 {TOOL_VERSION}。"),
            );
        }
        Some(_) => ladder.note("tool-version-matched", Verdict::Passed, TOOL_VERSION),
    }
    // 版本闸二：库侧 schema_history 与期望版本清单的比对需要读库，归执行段。
    ladder.note(
        "schema-history-version-matched",
        Verdict::NotCovered,
        "库侧版本比对需要读库，在执行段执行。",
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

    // gen-rls 不连库，就地完成。
    if inv.sub == Subcommand::GenRls {
        let schema = inv.rls_schema.as_deref().expect("CLI 闸已保证 --schema");
        let table = inv.rls_table.as_deref().expect("CLI 闸已保证 --table");
        return Stage::Settled(crate::genrls::run(schema, table, inv.out.as_deref()));
    }

    // open-window 的 reason 必填是参数判据，提前拦，免得连库后才落 2。
    if inv.sub == Subcommand::OpenWindow && inv.reason.is_none() {
        return fail(
            MigrateExit::UsageError,
            "open-window 必须给出 --reason（A-09 请求要素 reason ≤ 2000）。".to_string(),
        );
    }

    let url = url.expect("needs_db 的子命令在上面已解析出连接串");
    let _ = ladder.render(); // 阶梯明细在失败路径带出；放行路径由执行段产出正文
    Stage::NeedsDb(url)
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
        let out = stage(&inv(&["status"]), None, None);
        assert_eq!(out.exit(), Some(MigrateExit::EnvSelfCheckFailed));
    }

    #[test]
    fn env_var_supplies_db_url_and_stage_releases() {
        // 环境变量给了连接串，阶梯放行去执行段（不再落 NOT_DELIVERED）。
        let out = stage(&inv(&["status"]), Some("postgres://h/ep"), None);
        assert!(matches!(out, Stage::NeedsDb(u) if u == "postgres://h/ep"));
    }

    #[test]
    fn missing_migrations_dir_fails_env_selfcheck() {
        let out = stage(&inv(&["check", "--db-url=postgres://h/ep"]), None, None);
        match out {
            Stage::Settled(Outcome::Failed(e, msg)) => {
                assert_eq!(e, MigrateExit::EnvSelfCheckFailed);
                assert!(msg.contains("migrations-dir-readable"), "{msg}");
            }
            other => panic!("默认迁移目录不存在时不得放行：{other:?}"),
        }
    }

    #[test]
    fn apply_without_window_id_releases_to_exec_stage() {
        // 首装自举口径（偏离登记十四）：未出示窗口不再在本阶梯落 3，
        // 是否首装由执行段探历史表判定。
        let dir = probe_dir("window");
        let a = inv(&[
            "apply",
            "--db-url=postgres://h/ep",
            &format!("--migrations-dir={}", dir.display()),
        ]);
        assert!(matches!(stage(&a, None, None), Stage::NeedsDb(_)));

        // 对照面：出示窗口后同样放行去执行段读库判 OPEN。
        let b = inv(&[
            "apply",
            "--db-url=postgres://h/ep",
            &format!("--migrations-dir={}", dir.display()),
            "--window-id=w-1",
        ]);
        assert!(matches!(stage(&b, None, None), Stage::NeedsDb(_)));
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
        assert_eq!(
            stage(&a, None, None).exit(),
            Some(MigrateExit::VersionMismatch)
        );

        let b = inv(&[
            "check",
            "--db-url=postgres://h/ep",
            &format!("--migrations-dir={}", dir.display()),
            &format!("--expect-tool-version={TOOL_VERSION}"),
        ]);
        assert!(matches!(stage(&b, None, None), Stage::NeedsDb(_)));
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
        assert!(
            matches!(stage(&a, None, None), Stage::NeedsDb(_)),
            "哈希相符时不得落 4"
        );

        fs::write(
            dir.join("platform_core/V001__probe.sql"),
            "-- rollback: drop table ci_probe.probe_records;\ncreate table t2();\n",
        )
        .expect("篡改探针文件");
        assert_eq!(
            stage(&a, None, None).exit(),
            Some(MigrateExit::ChecksumMismatch)
        );
        fs::remove_dir_all(&dir).ok();
    }

    #[test]
    fn gen_rls_completes_without_db() {
        let out = stage(
            &inv(&["gen-rls", "--schema=mdm", "--table=parties"]),
            None,
            None,
        );
        match out {
            Stage::Settled(Outcome::Done(text)) => {
                assert!(text.contains("create policy"), "gen-rls 应直接产出策略语句");
            }
            other => panic!("gen-rls 不连库，应就地完成：{other:?}"),
        }
    }

    #[test]
    fn open_window_requires_reason_before_db() {
        let out = stage(
            &inv(&["open-window", "--db-url=postgres://h/ep"]),
            None,
            None,
        );
        assert_eq!(out.exit(), Some(MigrateExit::UsageError));
    }
}
