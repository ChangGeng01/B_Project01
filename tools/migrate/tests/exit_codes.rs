//! IT-01 与 IT-02 的阶段 1 可执行部分：起真进程，实测退出码。
//!
//! 六个退出码各至少一个用例，且每个用例都配一个对照面，证明拦下它的是那道判据
//! 本身而不是别的什么。库侧判据（空库上 24 个 schema 与历史表的存在性、窗口是否
//! 真的处于 OPEN、库侧版本比对）归阶段 2，本文件不为它们伪造被测对象。

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
    run(args).status.code().expect("进程正常退出而不是被信号杀掉")
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
fn exit_3_migration_window_closed() {
    let dir = probe_dir("window");
    // apply 没有出示窗口。
    assert_eq!(code(&["apply", DB, &dir_opt(&dir)]), 3);
    // 对照面：出示窗口后不再落 3；窗口是否真的 OPEN 归阶段 2，因此落 78 而不是 0。
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
    assert!(stderr(&b).contains("migrations-dir-readable"), "{}", stderr(&b));

    // 其三，前置阶梯全过之后，实现体缺席也落 78 而不是 0。
    let c = run(&["check", DB, &dir_opt(&dir)]);
    assert_eq!(c.status.code(), Some(78), "{}", stderr(&c));
    assert!(stderr(&c).contains("subcommand-implemented"), "{}", stderr(&c));

    // 对照面：环境变量给了连接串就不再落 db-url-resolvable。
    let d = Command::new(BIN)
        .args(["status"])
        .env("EP__DB__URL", "postgres://localhost/ep_probe")
        .output()
        .expect("能起进程");
    assert_eq!(d.status.code(), Some(78));
    assert!(stderr(&d).contains("subcommand-implemented"), "{}", stderr(&d));

    fs::remove_dir_all(&dir).ok();
}

/// 五个子命令一个都不许静默返回 0。
#[test]
fn no_subcommand_ever_exits_zero_in_this_stage() {
    let dir = probe_dir("nonzero");
    let d = dir_opt(&dir);
    let cases: Vec<Vec<&str>> = vec![
        vec!["apply", DB, &d, "--window-id=w-1"],
        vec!["status", DB],
        vec!["status", DB, "--format=manifest"],
        vec!["status", DB, "--format=json"],
        vec!["check", DB, &d],
        vec!["gen-rls", "--schema=mdm", "--table=parties"],
        vec!["gen-rls", "--schema=mdm", "--table=parties", "--out=/dev/null"],
        vec!["open-window", DB, "--ttl-minutes=30", "--reason=升级窗口"],
    ];
    for case in cases {
        let out = run(&case);
        assert_eq!(
            out.status.code(),
            Some(78),
            "{case:?} 必须以 78 报未交付，实得 {:?}\n{}",
            out.status.code(),
            stderr(&out)
        );
        let text = stderr(&out);
        assert!(text.contains("subcommand-implemented"), "{case:?}: {text}");
        assert!(text.contains("阶段 2"), "{case:?}: {text}");
    }
    fs::remove_dir_all(&dir).ok();
}

/// 六个码在同一轮里各出现一次，缺一个即不通过。
#[test]
fn all_six_exit_codes_are_reachable() {
    let dir = probe_dir("all-six");
    let d = dir_opt(&dir);
    let good = actual_manifest_hash(&dir);
    let bad_hash = "0".repeat(64);

    let observed = vec![
        code(&["--version"]),
        code(&["frobnicate"]),
        code(&["apply", DB, &d]),
        code(&["check", DB, &d, &format!("--expect-manifest-sha256={bad_hash}")]),
        code(&["check", DB, &d, "--expect-tool-version=9.9.9"]),
        code(&["status"]),
    ];
    assert_eq!(observed, vec![0, 2, 3, 4, 5, 78]);
    assert_ne!(good, bad_hash);
    fs::remove_dir_all(&dir).ok();
}
