//! `xtask reproduce` —— 可复现构建证据的比对门禁。
//!
//! 阶段 1 退出条件 13：两次独立构建产出完全相同的二进制哈希与镜像 digest。
//! 构建参数按计划第 5.7 节定死：`SOURCE_DATE_EPOCH` 取该 Git 提交的 committer 时间，
//! `RUSTFLAGS` 含两条 `--remap-path-prefix`，目标固定 `x86_64-unknown-linux-musl`。
//!
//! 本工具不构建，只比对两份证据。构建由流水线阶段 8 在两个不同路径下各跑一次并各留
//! 一份证据文件，本工具读这两份文件判等。这样划分的理由是：门禁跑两次全量构建会把
//! 单条命令的时长推到小时级，而判据本身只需要两份哈希清单。
//!
//! 三态纪律。证据文件缺席时判定未做出，因为本阶段还没有任何构建产物；
//! 证据在而条目为空、条目不齐或两侧不等，一律是不符。空比对不得判通过。

use std::collections::BTreeMap;
use std::fs;
use std::path::Path;

use crate::archcheck::Outcome;
use crate::sign::{sha256_file, IMAGES};

/// 两份证据的落点。同 [`crate::sign`]，落在 `target/` 之下，不新增顶层目录。
pub const EVIDENCE_DIR: &str = "target/reproduce";
pub const EVIDENCE_A: &str = "build-1.evidence";
pub const EVIDENCE_B: &str = "build-2.evidence";

/// 计划第 5.7 节定死的三项构建参数。
pub const FROZEN_TARGET: &str = "x86_64-unknown-linux-musl";
pub const REMAP_BUILD: &str = "--remap-path-prefix=$PWD=/build";
pub const REMAP_CARGO: &str = "--remap-path-prefix=$CARGO_HOME=/cargo";

/// 八个进程二进制。九个镜像取 [`crate::sign::IMAGES`]，两处不各写一份。
pub const BINARIES: [&str; 8] = [
    "archive-writer",
    "backup-writer",
    "core-server",
    "integration-gateway",
    "job-worker",
    "ops-agent",
    "plugin-host",
    "portal-gateway",
];

#[derive(Debug, Default)]
pub struct Report {
    /// 判不符：两份证据都读到了，但不相等或不齐。
    pub problems: Vec<String>,
    /// 判定未做出：证据缺席。任何情况下都不折算为通过。
    pub undecidable: Vec<String>,
    pub notes: Vec<String>,
}

impl Report {
    pub fn outcome(&self) -> Outcome {
        if !self.problems.is_empty() {
            Outcome::Violated
        } else if !self.undecidable.is_empty() {
            Outcome::Undecidable
        } else {
            Outcome::Clean
        }
    }
}

/// 一次构建留下的证据。
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Evidence {
    pub source_date_epoch: String,
    pub rustflags: String,
    pub target: String,
    /// 二进制名 → SHA-256。
    pub binaries: BTreeMap<String, String>,
    /// 镜像名 → digest，形如 `sha256:<64 位十六进制>`。
    pub images: BTreeMap<String, String>,
}

pub fn run(root: &Path) -> Report {
    evaluate(&root.join(EVIDENCE_DIR), Some(root))
}

/// 判据本体。`artifact_root` 给出时另做一次证据与实际二进制的交叉核对；
/// 负样例传 `None`，只判两份证据本身。
pub fn evaluate(dir: &Path, artifact_root: Option<&Path>) -> Report {
    let mut r = Report::default();
    let mut loaded: Vec<(&str, Evidence)> = Vec::new();
    for name in [EVIDENCE_A, EVIDENCE_B] {
        let path = dir.join(name);
        match fs::read_to_string(&path) {
            Err(e) => r.undecidable.push(format!(
                "{} 读不到（{e}）。本阶段尚无两次独立构建的证据，可复现构建判定未做出",
                path.display()
            )),
            Ok(text) => match parse_evidence(&text) {
                Ok(ev) => loaded.push((name, ev)),
                Err(msg) => r.problems.push(format!("{name} 解析失败：{msg}")),
            },
        }
    }
    if loaded.len() < 2 {
        // 只读到一份也不能判等：一份证据自己跟自己永远相同，那正是恒真形态。
        if r.undecidable.is_empty() && r.problems.is_empty() {
            r.problems.push("两份证据不足两份，无从判等".into());
        }
        return r;
    }

    let (a, b) = (&loaded[0].1, &loaded[1].1);
    r.problems.extend(compare(a, b));
    if let Some(root) = artifact_root {
        let (p, u) = cross_check(root, a);
        r.problems.extend(p);
        r.undecidable.extend(u);
    }
    if r.problems.is_empty() && r.undecidable.is_empty() {
        r.notes.push(format!(
            "{} 个二进制与 {} 个镜像两侧全等",
            a.binaries.len(),
            a.images.len()
        ));
    }
    r
}

/// 证据格式：`键=值` 三行，加若干条 `binary <名> <哈希>` 与 `image <名> <digest>`。
/// 不用 JSON，因为这份文件要能由构建脚本用 shell 直接追加，也要能人眼比对。
pub fn parse_evidence(text: &str) -> Result<Evidence, String> {
    let mut ev = Evidence::default();
    for (i, raw) in text.lines().enumerate() {
        let line = raw.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let no = i + 1;
        if let Some((key, value)) = line.split_once('=') {
            let value = value.trim().to_string();
            match key.trim() {
                "source_date_epoch" => ev.source_date_epoch = value,
                "rustflags" => ev.rustflags = value,
                "target" => ev.target = value,
                other => return Err(format!("第 {no} 行的键 {other} 不在取值域内")),
            }
            continue;
        }
        let mut parts = line.split_whitespace();
        let kind = parts.next().unwrap_or_default();
        let (Some(name), Some(digest)) = (parts.next(), parts.next()) else {
            return Err(format!("第 {no} 行不是「类别 名字 哈希」三段：{line}"));
        };
        if parts.next().is_some() {
            return Err(format!("第 {no} 行多出第四段：{line}"));
        }
        let slot = match kind {
            "binary" => &mut ev.binaries,
            "image" => &mut ev.images,
            other => return Err(format!("第 {no} 行的类别 {other} 不在取值域内")),
        };
        if slot.insert(name.to_string(), digest.to_string()).is_some() {
            return Err(format!("第 {no} 行的 {name} 在同一份证据中重复"));
        }
    }
    Ok(ev)
}

/// 规则本体：两份证据的构建参数与全部条目逐项相等，且花名册齐备。
pub fn compare(a: &Evidence, b: &Evidence) -> Vec<String> {
    let mut out = Vec::new();

    for (label, x, y) in [
        ("source_date_epoch", &a.source_date_epoch, &b.source_date_epoch),
        ("rustflags", &a.rustflags, &b.rustflags),
        ("target", &a.target, &b.target),
    ] {
        if x.is_empty() || y.is_empty() {
            out.push(format!("{label} 至少一侧为空，构建参数未固定即无从谈可复现"));
        } else if x != y {
            out.push(format!("{label} 两侧不等：\n      一次：{x}\n      二次：{y}"));
        }
    }
    if !a.target.is_empty() && a.target != FROZEN_TARGET {
        out.push(format!("target={} 不是冻结取值 {FROZEN_TARGET}", a.target));
    }
    for flag in [REMAP_BUILD, REMAP_CARGO] {
        if !a.rustflags.is_empty() && !a.rustflags.contains(flag) {
            out.push(format!("rustflags 中没有 {flag}，路径会进二进制"));
        }
    }

    out.extend(roster(&a.binaries, &BINARIES, "二进制"));
    out.extend(roster(&a.images, &IMAGES, "镜像"));
    out.extend(diff_side(a, b, "binary", |e| &e.binaries));
    out.extend(diff_side(a, b, "image", |e| &e.images));
    out
}

/// 花名册齐备判定。条目为空时必须报出：空集合逐项相等恒真，那是最典型的假通过。
fn roster(got: &BTreeMap<String, String>, want: &[&str], label: &str) -> Vec<String> {
    let mut out = Vec::new();
    if got.is_empty() {
        out.push(format!("证据中一条{label}记录都没有，空比对不得判通过"));
        return out;
    }
    for name in want {
        if !got.contains_key(*name) {
            out.push(format!("证据中缺{label} {name}"));
        }
    }
    for name in got.keys() {
        if !want.contains(&name.as_str()) {
            out.push(format!("证据中多出一项{label} {name}，不在应有花名册内"));
        }
    }
    out
}

fn diff_side(
    a: &Evidence,
    b: &Evidence,
    kind: &str,
    pick: fn(&Evidence) -> &BTreeMap<String, String>,
) -> Vec<String> {
    let (x, y) = (pick(a), pick(b));
    let mut out = Vec::new();
    for (name, va) in x {
        match y.get(name) {
            None => out.push(format!("{kind} {name} 只在第一次构建的证据中")),
            Some(vb) if vb != va => out.push(format!(
                "{kind} {name} 两次构建不等：\n      一次：{va}\n      二次：{vb}"
            )),
            Some(_) => {}
        }
    }
    for name in y.keys() {
        if !x.contains_key(name) {
            out.push(format!("{kind} {name} 只在第二次构建的证据中"));
        }
    }
    out
}

/// 证据与实际二进制的交叉核对。二进制不在时是「未做出判定」而不是通过：
/// 一份没人核对过的证据文件，本身不构成可复现构建的证据。
fn cross_check(root: &Path, ev: &Evidence) -> (Vec<String>, Vec<String>) {
    let mut problems = Vec::new();
    let mut undecidable = Vec::new();
    let base = root.join("target").join(FROZEN_TARGET).join("release");
    if !base.is_dir() {
        undecidable.push(format!(
            "{} 不存在，证据未与任何实际二进制核对过，交叉核对未做出判定",
            base.display()
        ));
        return (problems, undecidable);
    }
    for (name, recorded) in &ev.binaries {
        let path = base.join(name);
        if !path.is_file() {
            undecidable.push(format!("{} 不存在，{name} 的证据未经核对", path.display()));
            continue;
        }
        match sha256_file(&path) {
            Err(e) => undecidable.push(format!("{} 读不到：{e}", path.display())),
            Ok(actual) if &actual != recorded => problems.push(format!(
                "{name} 的实际 SHA-256 与证据不符。\n      证据：{recorded}\n      实际：{actual}"
            )),
            Ok(_) => {}
        }
    }
    (problems, undecidable)
}

#[cfg(test)]
mod rule_negative_samples {
    use super::*;
    use std::path::PathBuf;

    /// 夹具里的哈希是造出来的定值，不是真哈希：本规则判的是两侧相等，不是哈希本身。
    fn fake_hash(seed: usize) -> String {
        format!("{seed:064x}")
    }

    fn good_evidence() -> String {
        let mut s = String::from("# 第一次构建\n");
        s.push_str("source_date_epoch=1786396683\n");
        s.push_str(&format!("rustflags={REMAP_BUILD} {REMAP_CARGO}\n"));
        s.push_str(&format!("target={FROZEN_TARGET}\n"));
        for (i, n) in BINARIES.iter().enumerate() {
            s.push_str(&format!("binary {n} {}\n", fake_hash(i)));
        }
        for (i, n) in IMAGES.iter().enumerate() {
            s.push_str(&format!("image {n} sha256:{}\n", fake_hash(i + 100)));
        }
        s
    }

    fn fixture(tag: &str, a: &str, b: &str) -> PathBuf {
        let dir = std::env::temp_dir().join(format!("ep-reproduce-{tag}"));
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(&dir).expect("建夹具目录");
        fs::write(dir.join(EVIDENCE_A), a).expect("写证据一");
        fs::write(dir.join(EVIDENCE_B), b).expect("写证据二");
        dir
    }

    /// 正样例守边界：两份相同的完整证据必须全绿，否则后面的负样例证明不了任何事。
    #[test]
    fn negative_identical_evidence_is_clean() {
        let g = good_evidence();
        let r = evaluate(&fixture("same", &g, &g), None);
        assert_eq!(r.outcome(), Outcome::Clean, "实得：{:?}", r.problems);
    }

    /// 负样例：一个二进制哈希在两次构建之间变了。
    #[test]
    fn negative_binary_hash_differs() {
        let g = good_evidence();
        let idx = BINARIES.iter().position(|n| *n == "core-server").expect("花名册里有 core-server");
        let b = g.replace(
            &format!("binary core-server {}", fake_hash(idx)),
            &format!("binary core-server {}", fake_hash(0xff)),
        );
        assert_ne!(g, b, "替换必须真的发生，否则这条负样例在自欺");
        let r = evaluate(&fixture("hash-diff", &g, &b), None);
        assert_eq!(r.outcome(), Outcome::Violated);
        assert!(
            r.problems.iter().any(|p| p.contains("binary core-server") && p.contains("两次构建不等")),
            "实得：{:?}",
            r.problems
        );
    }

    /// 负样例：镜像 digest 少一个。
    #[test]
    fn negative_image_missing_on_one_side() {
        let g = good_evidence();
        let b: String = g.lines().filter(|l| !l.starts_with("image ep-migrate ")).collect::<Vec<_>>().join("\n");
        let r = evaluate(&fixture("image-missing", &g, &b), None);
        assert_eq!(r.outcome(), Outcome::Violated);
        assert!(
            r.problems.iter().any(|p| p.contains("image ep-migrate 只在第一次")),
            "实得：{:?}",
            r.problems
        );
    }

    /// 负样例：两份都空。空集合逐项相等恒真，规则必须把它判红而不是判绿。
    #[test]
    fn negative_empty_evidence_must_not_pass() {
        let head = format!(
            "source_date_epoch=1\nrustflags={REMAP_BUILD} {REMAP_CARGO}\ntarget={FROZEN_TARGET}\n"
        );
        let r = evaluate(&fixture("empty", &head, &head), None);
        assert_eq!(r.outcome(), Outcome::Violated);
        assert!(r.problems.iter().any(|p| p.contains("一条二进制记录都没有")));
        assert!(r.problems.iter().any(|p| p.contains("一条镜像记录都没有")));
    }

    /// 负样例：证据整体缺席必须是「判定未做出」，不是通过。
    #[test]
    fn negative_absent_evidence_is_undecidable() {
        let dir = std::env::temp_dir().join("ep-reproduce-absent");
        let _ = fs::remove_dir_all(&dir);
        let r = evaluate(&dir, None);
        assert_eq!(r.outcome(), Outcome::Undecidable);
        assert_eq!(r.undecidable.len(), 2, "两份证据各报一条");
        assert!(r.undecidable[0].contains("判定未做出"));
    }

    /// 负样例：构建参数没按第 5.7 节固定。
    #[test]
    fn negative_frozen_build_parameters() {
        let g = good_evidence();
        let bad = g.replace(FROZEN_TARGET, "x86_64-unknown-linux-gnu");
        let r = evaluate(&fixture("target", &bad, &bad), None);
        assert_eq!(r.outcome(), Outcome::Violated);
        assert!(r.problems.iter().any(|p| p.contains("不是冻结取值")));

        let no_remap = g.replace(REMAP_CARGO, "");
        let r = evaluate(&fixture("remap", &no_remap, &no_remap), None);
        assert!(r.problems.iter().any(|p| p.contains(REMAP_CARGO)));
    }

    /// 负样例：证据格式。四段、未知类别、重复条目三种都必须报出。
    #[test]
    fn negative_evidence_parse() {
        assert!(parse_evidence("binary a b c").is_err(), "多出第四段");
        assert!(parse_evidence("blob a b").is_err(), "未知类别");
        assert!(parse_evidence("binary a x\nbinary a y").is_err(), "同名重复");
        assert!(parse_evidence("what=1").is_err(), "未知键");
        assert!(parse_evidence("# 只有注释\n").is_ok());
    }
}
