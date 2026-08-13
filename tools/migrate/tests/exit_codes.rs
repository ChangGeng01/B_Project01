//! IT-01 与 IT-02：起真进程，实测退出码。
//!
//! 六个退出码各至少一个用例，且每个用例都配一个对照面，证明拦下它的是那道判据
//! 本身而不是别的什么。阶段 2 实现体已补齐：需要读库的子命令在无活库的测试机上
//! 落 78（环境自检项 db-reachable），gen-rls 不连库则落 0；库侧判据（窗口是否真
//! OPEN、库侧版本比对、合规断言）需要活库，本文件不为它们伪造被测对象。
//! 首装自举口径（偏离登记十四）生效后，退出码 3 需探库后才能触发，本文件
//! 以 EP_TEST_PG_URL 门控的活库用例补齐该码与自举全链的实证。

use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};

const BIN: &str = env!("CARGO_BIN_EXE_ep-migrate");
const DB: &str = "--db-url=postgres://localhost/ep_probe";

fn run(args: &[&str]) -> Output {
    Command::new(BIN)
        .args(args)
        // 清掉环境变量，免得开发机上恰好设了它而让 db-url-resolvable 的用例失真。
        .env_remove("EP__DB__URL")
        .output()
        .expect("能起 ep-migrate 进程")
}

fn code(args: &[&str]) -> i32 {
    run(args)
        .status
        .code()
        .expect("进程正常退出而不是被信号杀掉")
}

fn stderr(out: &Output) -> String {
    String::from_utf8_lossy(&out.stderr).into_owned()
}

/// 探针迁移目录。不进 db/migrations/，只活在临时目录里。
fn probe_dir(tag: &str) -> PathBuf {
    let mut p = std::env::temp_dir();
    p.push(format!(
        "ep-migrate-it-{tag}-{}-{}",
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_nanos())
            .unwrap_or_default()
    ));
    fs::create_dir_all(p.join("platform_core")).expect("建探针目录");
    fs::write(
        p.join("platform_core/V202609010900__probe.sql"),
        "-- rollback: drop table ci_probe.probe_records;\ncreate table ci_probe.probe_records();\n",
    )
    .expect("写探针迁移文件");
    p
}

fn dir_opt(dir: &Path) -> String {
    format!("--migrations-dir={}", dir.display())
}

/// 用 `--expect-manifest-sha256` 反推当前目录的实算哈希：先故意给一个必然不符的
/// 值，从 stderr 的「实算清单哈希为 ...」里取回来。这样测试不重复实现一遍算法，
/// 断言的是 CLI 自己算出来的东西。
fn actual_manifest_hash(dir: &Path) -> String {
    let wrong = "0".repeat(64);
    let out = run(&[
        "check",
        DB,
        &dir_opt(dir),
        &format!("--expect-manifest-sha256={wrong}"),
    ]);
    assert_eq!(out.status.code(), Some(4), "{}", stderr(&out));
    let text = stderr(&out);
    let marker = "实算清单哈希为 ";
    let start = text.find(marker).expect("错误说明里必须报出实算值") + marker.len();
    text[start..start + 64].to_string()
}

#[test]
fn exit_0_success() {
    assert_eq!(code(&["--help"]), 0);
    assert_eq!(code(&["--version"]), 0);
    assert_eq!(code(&["apply", "--help"]), 0);

    let out = run(&["--help"]);
    let text = String::from_utf8_lossy(&out.stdout);
    for name in ["apply", "status", "check", "gen-rls", "open-window"] {
        assert!(text.contains(name), "用法必须列出 {name}");
    }
    for c in ["0", "2", "3", "4", "5", "78"] {
        assert!(text.contains(c), "用法必须列出退出码 {c}");
    }
}

#[test]
fn exit_2_usage_error() {
    // 未知子命令。
    assert_eq!(code(&["frobnicate"]), 2);
    // 阶段 1 的旧名已按 C-02 并入，必须不再存在。
    for old in ["migrate", "verify", "manifest"] {
        assert_eq!(code(&[old]), 2, "旧子命令 {old} 必须已消失");
    }
    // 缺子命令。
    assert_eq!(code(&[]), 2);
    // 子命令不接受的选项。
    assert_eq!(code(&["gen-rls", "--schema=mdm", "--table=t", DB]), 2);
    // 取值形态不合法。
    assert_eq!(code(&["status", DB, "--format=xml"]), 2);
    assert_eq!(code(&["open-window", DB, "--ttl-minutes=241"]), 2);
    assert_eq!(code(&["apply", DB, "--expect-manifest-sha256=dead"]), 2);
    // 必填缺失。
    assert_eq!(code(&["gen-rls", "--schema=mdm"]), 2);
    // 选项缺值。
    assert_eq!(code(&["status", "--db-url"]), 2);

    // 对照面：同样的子命令写对了参数就不再落 2。
    assert_ne!(code(&["gen-rls", "--schema=mdm", "--table=t"]), 2);
    assert_ne!(code(&["status", DB, "--format=manifest"]), 2);
}

#[test]
fn exit_3_window_gate_requires_live_db() {
    // 首装自举口径（02 计划第 12 节偏离登记十四）生效后，apply 未出示窗口时
    // 必须先探历史表才能判定是否首装：无活库机器上探测失败落 78，
    // 退出码 3 只能在活库上对非空库触发（见 live_exit_3_* 用例）。
    let dir = probe_dir("window");
    let out = run(&["apply", DB, &dir_opt(&dir)]);
    assert_eq!(out.status.code(), Some(78), "{}", stderr(&out));
    assert!(stderr(&out).contains("db-reachable"), "{}", stderr(&out));
    // 对照面：出示窗口的调用同样要连库才能判 OPEN，无活库落 78 而非 3。
    let out = run(&["apply", DB, &dir_opt(&dir), "--window-id=w-1"]);
    assert_eq!(out.status.code(), Some(78), "{}", stderr(&out));
    // 对照面：不是 apply 的子命令不受窗口闸约束。
    assert_ne!(code(&["check", DB, &dir_opt(&dir)]), 3);
    fs::remove_dir_all(&dir).ok();
}

#[test]
fn exit_4_checksum_mismatch() {
    let dir = probe_dir("checksum");
    let good = actual_manifest_hash(&dir);

    // 对照面：哈希相符时不落 4。
    let ok = run(&[
        "check",
        DB,
        &dir_opt(&dir),
        &format!("--expect-manifest-sha256={good}"),
    ]);
    assert_eq!(ok.status.code(), Some(78), "{}", stderr(&ok));

    // 篡改探针目录中的一个文件后比对失败（退出条件 5 原文）。
    fs::write(
        dir.join("platform_core/V202609010900__probe.sql"),
        "-- rollback: drop table ci_probe.probe_records;\ncreate table ci_probe.probe_records(x int);\n",
    )
    .expect("篡改探针文件");
    assert_eq!(
        code(&[
            "check",
            DB,
            &dir_opt(&dir),
            &format!("--expect-manifest-sha256={good}")
        ]),
        4
    );
    fs::remove_dir_all(&dir).ok();
}

#[test]
fn exit_5_version_mismatch() {
    let dir = probe_dir("version");
    assert_eq!(
        code(&["check", DB, &dir_opt(&dir), "--expect-tool-version=9.9.9"]),
        5
    );
    // 对照面：出示本制品自身的版本就不再落 5。
    let self_version = {
        let out = run(&["--version"]);
        let text = String::from_utf8_lossy(&out.stdout).into_owned();
        text.lines()
            .next()
            .and_then(|l| l.strip_prefix("ep-migrate "))
            .expect("--version 第一行形如 ep-migrate <版本>")
            .trim()
            .to_string()
    };
    let out = run(&[
        "check",
        DB,
        &dir_opt(&dir),
        &format!("--expect-tool-version={self_version}"),
    ]);
    assert_eq!(out.status.code(), Some(78), "{}", stderr(&out));
    fs::remove_dir_all(&dir).ok();
}

#[test]
fn exit_78_env_selfcheck_failed() {
    let dir = probe_dir("env");

    // 其一，连接串两处都没有。
    let a = run(&["status"]);
    assert_eq!(a.status.code(), Some(78), "{}", stderr(&a));
    assert!(stderr(&a).contains("db-url-resolvable"), "{}", stderr(&a));

    // 其二，默认迁移目录 db/migrations 尚不存在。
    let b = run(&["check", DB]);
    assert_eq!(b.status.code(), Some(78), "{}", stderr(&b));
    assert!(
        stderr(&b).contains("migrations-dir-readable"),
        "{}",
        stderr(&b)
    );

    // 其三，前置阶梯全过之后，执行段连不上库也落 78（环境自检项 db-reachable）。
    let c = run(&["check", DB, &dir_opt(&dir)]);
    assert_eq!(c.status.code(), Some(78), "{}", stderr(&c));
    assert!(stderr(&c).contains("db-reachable"), "{}", stderr(&c));

    // 对照面：环境变量给了连接串就不再落 db-url-resolvable，改落 db-reachable。
    let d = Command::new(BIN)
        .args(["status"])
        .env("EP__DB__URL", "postgres://localhost/ep_probe")
        .env_remove("EP__DB__DSN")
        .output()
        .expect("能起进程");
    assert_eq!(d.status.code(), Some(78));
    assert!(stderr(&d).contains("db-reachable"), "{}", stderr(&d));

    fs::remove_dir_all(&dir).ok();
}

/// 需要读库的子命令在无活库机器上不许静默返回 0；gen-rls 不连库，必须真干活。
#[test]
fn db_subcommands_never_exit_zero_without_live_db() {
    let dir = probe_dir("nonzero");
    let d = dir_opt(&dir);
    let db_cases: Vec<Vec<&str>> = vec![
        vec!["apply", DB, &d, "--window-id=w-1"],
        vec!["status", DB],
        vec!["status", DB, "--format=manifest"],
        vec!["status", DB, "--format=json"],
        vec!["check", DB, &d],
        vec!["open-window", DB, "--ttl-minutes=30", "--reason=升级窗口"],
    ];
    for case in db_cases {
        let out = run(&case);
        assert_eq!(
            out.status.code(),
            Some(78),
            "{case:?} 无活库必须以 78 报 db-reachable，实得 {:?}\n{}",
            out.status.code(),
            stderr(&out)
        );
        let text = stderr(&out);
        assert!(text.contains("db-reachable"), "{case:?}: {text}");
    }

    // gen-rls 不连库：实现体已补齐，必须落 0 并产出策略语句。
    for case in [
        vec!["gen-rls", "--schema=mdm", "--table=parties"],
        vec![
            "gen-rls",
            "--schema=mdm",
            "--table=parties",
            "--out=/dev/null",
        ],
    ] {
        let out = run(&case);
        assert_eq!(
            out.status.code(),
            Some(0),
            "{case:?} 不连库应就地完成，实得 {:?}\n{}",
            out.status.code(),
            stderr(&out)
        );
    }
    let out = run(&["gen-rls", "--schema=mdm", "--table=parties"]);
    let text = String::from_utf8_lossy(&out.stdout);
    assert!(
        text.contains("create policy"),
        "gen-rls 正文必须含策略语句：{text}"
    );
    fs::remove_dir_all(&dir).ok();
}

/// 六个码在同一轮里各出现一次，缺一个即不通过。
/// 退出码 3 在首装自举口径生效后必须有活库才能触发（窗口闸读库判定）：
/// 设了 EP_TEST_PG_URL 时用活库独占库触发 3 凑齐六码；未设时断言离线
/// 可达的五码并留痕，不为凑码伪造被测对象。
#[test]
fn all_six_exit_codes_are_reachable() {
    let dir = probe_dir("all-six");
    let d = dir_opt(&dir);
    let good = actual_manifest_hash(&dir);
    let bad_hash = "0".repeat(64);

    let mut observed = vec![code(&["--version"]), code(&["frobnicate"])];
    let live = LiveAdmin::from_env();
    match &live {
        Some(admin) => {
            // 非空库（已有历史表）且未出示窗口且有待执行迁移 → 3。
            let db = admin.create_db();
            admin
                .psql(
                    &db,
                    "create schema platform_core; create table \
                     platform_core.schema_history (version bigint primary key, \
                     name varchar(255), applied_on varchar(255), checksum varchar(255));",
                )
                .expect("建最小非空库结构");
            let out = run(&["apply", &admin.db_url_opt(&db), &d]);
            assert_eq!(out.status.code(), Some(3), "{}", stderr(&out));
            observed.push(3);
            admin.drop_db(&db);
        }
        None => {
            eprintln!("留痕：未设 EP_TEST_PG_URL，退出码 3 本轮未触发（需活库）");
            // 无窗口 apply 在无活库机器上落探测失败 78。
            observed.push(code(&["apply", DB, &d]));
        }
    }
    observed.push(code(&[
        "check",
        DB,
        &d,
        &format!("--expect-manifest-sha256={bad_hash}"),
    ]));
    observed.push(code(&["check", DB, &d, "--expect-tool-version=9.9.9"]));
    observed.push(code(&["status"]));
    match &live {
        Some(_) => assert_eq!(observed, vec![0, 2, 3, 4, 5, 78]),
        None => assert_eq!(observed, vec![0, 2, 78, 4, 5, 78]),
    }
    assert_ne!(good, bad_hash);
    fs::remove_dir_all(&dir).ok();
}

// ---------------------------------------------------------------------------
// 活库用例：门控在 EP_TEST_PG_URL（同 ep-testkit 约定）。未设即留痕跳过；
// 独占库名 ep_test_<后缀>，用例收尾删库，与 workspace 其余活库测试同约定。
// 库的管理动作经 psql 完成：本文件是二进级集成测试，不重复实现客户端。
// ---------------------------------------------------------------------------

const LIVE_URL_ENV: &str = "EP_TEST_PG_URL";

struct LiveAdmin {
    host: String,
    port: String,
    user: String,
    password: Option<String>,
    admin_db: String,
}

impl LiveAdmin {
    /// 未设 EP_TEST_PG_URL 或形态不解析时返回 None：调用方据此跳过。
    fn from_env() -> Option<Self> {
        let url = std::env::var(LIVE_URL_ENV).ok()?;
        let rest = url
            .strip_prefix("postgres://")
            .or_else(|| url.strip_prefix("postgresql://"))?;
        let (userinfo, rest) = rest.split_once('@')?;
        let (hostport, database) = rest.split_once('/')?;
        let (host, port) = hostport.split_once(':')?;
        let (user, password) = match userinfo.split_once(':') {
            Some((u, p)) => (u.to_string(), Some(p.to_string())),
            None => (userinfo.to_string(), None),
        };
        Some(Self {
            host: host.to_string(),
            port: port.to_string(),
            user,
            password,
            admin_db: database.trim_end_matches('/').to_string(),
        })
    }

    fn psql(&self, db: &str, sql: &str) -> Result<String, String> {
        let mut cmd = Command::new("psql");
        cmd.args([
            "-h",
            &self.host,
            "-p",
            &self.port,
            "-U",
            &self.user,
            "-d",
            db,
            "-v",
            "ON_ERROR_STOP=1",
            "-qtAc",
            sql,
        ]);
        if let Some(pw) = &self.password {
            cmd.env("PGPASSWORD", pw);
        }
        let out = cmd.output().map_err(|e| format!("起 psql 失败：{e}"))?;
        let stdout = String::from_utf8_lossy(&out.stdout).into_owned();
        let stderr_txt = String::from_utf8_lossy(&out.stderr).into_owned();
        if !out.status.success() {
            return Err(format!("psql 失败：{stderr_txt}"));
        }
        Ok(stdout.trim().to_string())
    }

    fn create_db(&self) -> String {
        let name = format!(
            "ep_test_{}_{}",
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .map(|d| d.as_nanos())
                .unwrap_or_default(),
            std::process::id()
        );
        self.psql(&self.admin_db, &format!("create database {name}"))
            .expect("独占建库必须成功");
        name
    }

    fn drop_db(&self, name: &str) {
        self.psql(
            &self.admin_db,
            &format!("drop database {name} with (force)"),
        )
        .expect("独占库必须删除成功");
    }

    fn db_url_opt(&self, db: &str) -> String {
        let creds = match &self.password {
            Some(p) => format!("{}:{p}@", self.user),
            None => format!("{}@", self.user),
        };
        format!(
            "--db-url=postgres://{creds}{}:{}/{}",
            self.host, self.port, db
        )
    }

    fn role_exists(&self, role: &str) -> bool {
        self.psql(
            &self.admin_db,
            &format!("select 1 from pg_roles where rolname = '{role}'"),
        )
        .map(|s| s == "1")
        .unwrap_or(false)
    }
}

/// 非空库未出示窗口且有待执行迁移：退出码 3（窗口闸的活库实证）。
#[test]
fn live_exit_3_on_nonempty_db_without_window() {
    let Some(admin) = LiveAdmin::from_env() else {
        eprintln!("跳过：未设 {LIVE_URL_ENV}，需运行中的 PostgreSQL");
        return;
    };
    let db = admin.create_db();
    admin
        .psql(
            &db,
            "create schema platform_core; create table \
             platform_core.schema_history (version bigint primary key, \
             name varchar(255), applied_on varchar(255), checksum varchar(255));",
        )
        .expect("建最小非空库结构");
    let dir = probe_dir("live3");
    let out = run(&["apply", &admin.db_url_opt(&db), &dir_opt(&dir)]);
    assert_eq!(out.status.code(), Some(3), "{}", stderr(&out));
    assert!(
        stderr(&out).contains("PLATFORM.DB.MIGRATION_WINDOW_CLOSED"),
        "{}",
        stderr(&out)
    );
    // 对照面：同一库出示一个不存在的窗口同样落 3（库中没有 OPEN 窗口）；
    // 锁表尚不存在时读库判据落 78，两种库侧形态都不是 0。
    let out = run(&[
        "apply",
        &admin.db_url_opt(&db),
        &dir_opt(&dir),
        "--window-id=00000000-0000-7000-8000-00000000ffff",
    ]);
    assert!(
        matches!(out.status.code(), Some(3) | Some(78)),
        "{}",
        stderr(&out)
    );
    fs::remove_dir_all(&dir).ok();
    admin.drop_db(&db);
}

/// 空库首装自举全链：无预施、无 --window-id 的 apply 自举建表并执行全部
/// 探针迁移；窗口行形态逐项实证；重复 apply 走正常比对路径退出码 0
/// 且不再自举（窗口行不新增）。需本机已执行过 db/bootstrap 的角色脚本
/// （簇级 ep_mod_* 角色存在），否则留痕跳过。
#[test]
fn live_empty_db_bootstrap_apply_and_repeat() {
    let Some(admin) = LiveAdmin::from_env() else {
        eprintln!("跳过：未设 {LIVE_URL_ENV}，需运行中的 PostgreSQL");
        return;
    };
    if !admin.role_exists("ep_mod_platform_core") {
        eprintln!("跳过：本机无引导角色 ep_mod_platform_core，首装自举用例需先跑 db/bootstrap");
        return;
    }
    let db = admin.create_db();
    // 探针迁移目录：一个无害文件，建表落在自举建出的 platform_core schema。
    let dir = std::env::temp_dir().join(format!(
        "ep-migrate-it-bootstrap-{}-{}",
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_nanos())
            .unwrap_or_default()
    ));
    fs::create_dir_all(dir.join("platform_core")).expect("建探针目录");
    fs::write(
        dir.join("platform_core/V202609010900__live_bootstrap_probe.sql"),
        "-- rollback: drop table platform_core.live_bootstrap_probe;\n\
         create table platform_core.live_bootstrap_probe ();\n",
    )
    .expect("写探针迁移文件");

    // 首跑：无预施、无 --window-id，退出码 0，正文带首装自举说明。
    let out = run(&["apply", &admin.db_url_opt(&db), &dir_opt(&dir)]);
    assert_eq!(out.status.code(), Some(0), "{}", stderr(&out));
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(stdout.contains("首装自举"), "{stdout}");
    assert!(stdout.contains("INITIAL_INSTALL"), "{stdout}");

    // 窗口行形态：OPEN 态、固定审批引用、系统主体开窗、应用版本已回写。
    let row = admin
        .psql(
            &db,
            "select state || '|' || approval_ref || '|' || opened_by::text \
             || '|' || array_length(applied_versions, 1) \
             from platform_core.migration_windows",
        )
        .expect("窗口行可读");
    assert_eq!(
        row, "OPEN|INITIAL_INSTALL|00000000-0000-7000-8000-000000000001|1",
        "首装窗口行形态不符：{row}"
    );
    // 锁表唯一行与历史表台账均已建出。
    let lock = admin
        .psql(&db, "select id from platform_core.migration_window_lock")
        .expect("锁行可读");
    assert_eq!(lock, "1");
    let hist = admin
        .psql(&db, "select count(*) from platform_core.schema_history")
        .expect("历史表可读");
    assert_eq!(hist, "1");

    // 重复 apply：正常比对路径，退出码 0，不再自举，窗口行不新增。
    let out = run(&["apply", &admin.db_url_opt(&db), &dir_opt(&dir)]);
    assert_eq!(out.status.code(), Some(0), "{}", stderr(&out));
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(stdout.contains("无待执行迁移"), "{stdout}");
    assert!(
        !stdout.contains("首装自举"),
        "重复 apply 不得再自举：{stdout}"
    );
    let windows = admin
        .psql(&db, "select count(*) from platform_core.migration_windows")
        .expect("窗口表可读");
    assert_eq!(windows, "1", "重复 apply 不得再开窗口");

    fs::remove_dir_all(&dir).ok();
    admin.drop_db(&db);
}
