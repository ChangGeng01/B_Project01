//! `xtask coverage` —— 覆盖率分档门禁。
//!
//! 阶段 1 退出条件 12：覆盖率四档全部达标，且有一个人为降低覆盖率的负样例使流水线变红。
//! 分档算法照计划第 5.8 节：产出 lcov 后按文件路径匹配 `codecov.toml` 的路径规则，
//! 归入 A 档（85%）与 B 档（70%），另有整体档（80%）与增量档（80%）。
//! cargo-llvm-cov 只支持全局阈值，分档因此自行实现。
//!
//! 三态纪律，本模块最吃紧的一条。判定工具缺席、`codecov.toml` 缺席、增量档的比对基
//! 取不到，三者一律是「判定未做出」，退出码与「不达标」不同，且**任何情况下都不产出
//! 一个编造的覆盖率数字**。读不到被测对象时报未覆盖，不报通过。

use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::Path;
use std::process::Command;

use crate::archcheck::Outcome;

/// 分档路径规则的唯一来源。退出条件 2 点名它按目录前缀表达，
/// 因此新增 crate 不会静默逃出分档。
pub const RULES_DOC: &str = "codecov.toml";
/// lcov 落点。存在则直接读，不存在则调 cargo-llvm-cov 现产。
pub const LCOV_PATH: &str = "target/coverage/lcov.info";

pub const TIER_A_THRESHOLD: f64 = 85.0;
pub const TIER_B_THRESHOLD: f64 = 70.0;
pub const OVERALL_THRESHOLD: f64 = 80.0;
pub const DIFF_THRESHOLD: f64 = 80.0;

/// 增量档的比对基。按第一个能解析出来的取，都取不到即判定未做出。
const DIFF_BASES: [&str; 2] = ["origin/main", "main"];

/// 未达标时每个文件最多列出的缺失行号，避免一条错误信息刷屏。
const MAX_LISTED_LINES: usize = 20;

#[derive(Debug, Default)]
pub struct Report {
    /// 判不符：覆盖率读到了，低于门槛。
    pub problems: Vec<String>,
    /// 判定未做出：工具、规则或被测数据缺席。绝不折算为通过。
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

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Tier {
    A,
    B,
}

impl Tier {
    pub fn threshold(self) -> f64 {
        match self {
            Tier::A => TIER_A_THRESHOLD,
            Tier::B => TIER_B_THRESHOLD,
        }
    }

    pub fn label(self) -> &'static str {
        match self {
            Tier::A => "A 档（强制不变量相关与平台内核）",
            Tier::B => "B 档（其余）",
        }
    }
}

/// A 档路径前缀清单。B 档是补集，因此不单独登记。
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct TierRules {
    pub a_prefixes: Vec<String>,
}

/// 一个文件的行覆盖：行号 → 命中次数。
#[derive(Clone, Debug)]
pub struct FileCoverage {
    pub path: String,
    pub lines: BTreeMap<u32, u64>,
}

impl FileCoverage {
    fn covered(&self) -> usize {
        self.lines.values().filter(|h| **h > 0).count()
    }

    fn missing(&self) -> Vec<u32> {
        self.lines.iter().filter(|(_, h)| **h == 0).map(|(l, _)| *l).collect()
    }
}

pub fn run(root: &Path) -> Report {
    let mut r = Report::default();

    let rules = match fs::read_to_string(root.join(RULES_DOC)) {
        Err(e) => {
            r.undecidable.push(format!(
                "读不到 {RULES_DOC}（{e}）。分档路径规则没有来源，四档判定一条都未做出",
            ));
            return r;
        }
        Ok(text) => match parse_rules(&text) {
            Ok(v) => v,
            Err(msg) => {
                r.problems.push(format!("{RULES_DOC} 解析失败：{msg}"));
                return r;
            }
        },
    };

    let lcov = match load_lcov(root) {
        Ok(text) => text,
        Err(msg) => {
            r.undecidable.push(msg);
            return r;
        }
    };
    let files = match parse_lcov(&lcov) {
        Ok(v) => v,
        Err(msg) => {
            r.problems.push(format!("lcov 解析失败：{msg}"));
            return r;
        }
    };

    let diff = match diff_lines(root) {
        Ok(v) => Some(v),
        Err(msg) => {
            r.undecidable.push(msg);
            None
        }
    };

    let mut inner = evaluate(&files, &rules, diff.as_ref());
    r.problems.append(&mut inner.problems);
    r.undecidable.append(&mut inner.undecidable);
    r.notes.append(&mut inner.notes);
    r
}

/// 判据本体。四档各自判定，任一档不达标即不符；被测集合为空一律判未做出。
pub fn evaluate(
    files: &[FileCoverage],
    rules: &TierRules,
    diff: Option<&BTreeMap<String, BTreeSet<u32>>>,
) -> Report {
    let mut r = Report::default();
    if files.is_empty() {
        r.undecidable
            .push("lcov 中没有任何文件记录，四档判定未做出。空扫描不是通过".into());
        return r;
    }

    for tier in [Tier::A, Tier::B] {
        let picked: Vec<&FileCoverage> =
            files.iter().filter(|f| classify(&f.path, rules) == tier).collect();
        let total: usize = picked.iter().map(|f| f.lines.len()).sum();
        if total == 0 {
            r.undecidable.push(format!(
                "{} 下没有任何可执行行，该档判定未做出。分档规则或 lcov 至少有一处不对",
                tier.label()
            ));
            continue;
        }
        let covered: usize = picked.iter().map(|f| f.covered()).sum();
        let rate = percent(covered, total);
        if rate < tier.threshold() {
            r.problems.push(format!(
                "{} 行覆盖率 {rate:.2}% 低于门槛 {:.0}%（{covered}/{total}）{}",
                tier.label(),
                tier.threshold(),
                below_threshold_detail(&picked, tier.threshold()),
            ));
        } else {
            r.notes.push(format!("{} {rate:.2}%（门槛 {:.0}%）", tier.label(), tier.threshold()));
        }
    }

    let total: usize = files.iter().map(|f| f.lines.len()).sum();
    let covered: usize = files.iter().map(FileCoverage::covered).sum();
    let overall = percent(covered, total);
    if overall < OVERALL_THRESHOLD {
        r.problems.push(format!(
            "整体档行覆盖率 {overall:.2}% 低于门槛 {OVERALL_THRESHOLD:.0}%（{covered}/{total}）"
        ));
    } else {
        r.notes.push(format!("整体档 {overall:.2}%（门槛 {OVERALL_THRESHOLD:.0}%）"));
    }

    match diff {
        None => r.undecidable.push("增量行集合取不到，增量档判定未做出".into()),
        Some(d) => {
            let note = diff_tier(files, d, &mut r.problems);
            r.notes.push(note);
        }
    }
    r
}

/// 增量档。与 lcov 取交集，只判可执行行；交集为空时如实说明本次没有被测行，
/// 既不判不达标也不冒充达标——这与「取不到比对基」是两件事，后者归 undecidable。
fn diff_tier(
    files: &[FileCoverage],
    diff: &BTreeMap<String, BTreeSet<u32>>,
    problems: &mut Vec<String>,
) -> String {
    let mut total = 0usize;
    let mut covered = 0usize;
    let mut misses: Vec<String> = Vec::new();
    for f in files {
        let Some(changed) = diff.get(&f.path) else { continue };
        let mut file_missing: Vec<u32> = Vec::new();
        for line in changed {
            let Some(hits) = f.lines.get(line) else { continue };
            total += 1;
            if *hits > 0 {
                covered += 1;
            } else {
                file_missing.push(*line);
            }
        }
        if !file_missing.is_empty() {
            misses.push(format!("      {} 缺失行 {}", f.path, join_lines(&file_missing)));
        }
    }
    if total == 0 {
        return "增量档：本次比对基与 HEAD 之间没有任何被覆盖率统计的新增或修改行，该档无被测对象"
            .into();
    }
    let rate = percent(covered, total);
    if rate < DIFF_THRESHOLD {
        problems.push(format!(
            "增量档行覆盖率 {rate:.2}% 低于门槛 {DIFF_THRESHOLD:.0}%（{covered}/{total}）\n{}",
            misses.join("\n")
        ));
    }
    format!("增量档 {rate:.2}%（门槛 {DIFF_THRESHOLD:.0}%，{covered}/{total}）")
}

fn below_threshold_detail(files: &[&FileCoverage], threshold: f64) -> String {
    let mut out = String::new();
    for f in files {
        if f.lines.is_empty() {
            continue;
        }
        let rate = percent(f.covered(), f.lines.len());
        if rate >= threshold {
            continue;
        }
        out.push_str(&format!(
            "\n      {} {rate:.2}%，缺失行 {}",
            f.path,
            join_lines(&f.missing())
        ));
    }
    out
}

fn join_lines(lines: &[u32]) -> String {
    let shown: Vec<String> = lines.iter().take(MAX_LISTED_LINES).map(u32::to_string).collect();
    if lines.len() > MAX_LISTED_LINES {
        format!("{}…（共 {} 行）", shown.join(","), lines.len())
    } else {
        shown.join(",")
    }
}

fn percent(covered: usize, total: usize) -> f64 {
    if total == 0 {
        return 0.0;
    }
    covered as f64 * 100.0 / total as f64
}

pub fn classify(path: &str, rules: &TierRules) -> Tier {
    if rules.a_prefixes.iter().any(|p| path.starts_with(p.as_str())) {
        Tier::A
    } else {
        Tier::B
    }
}

/// `codecov.toml` 中只读一处：`[coverage.tier_a]` 的 `paths` 数组。
/// 空数组视为解析失败，因为「A 档没有任何路径」会让 A 档判定恒真。
pub fn parse_rules(text: &str) -> Result<TierRules, String> {
    let mut in_section = false;
    let mut collecting = false;
    let mut buf = String::new();
    for line in text.lines() {
        let line = line.split('#').next().unwrap_or_default().trim();
        if line.starts_with('[') {
            in_section = line == "[coverage.tier_a]";
            continue;
        }
        if !in_section {
            continue;
        }
        if let Some((key, value)) = line.split_once('=') {
            if key.trim() != "paths" {
                continue;
            }
            collecting = true;
            buf.push_str(value.trim());
        } else if collecting {
            buf.push_str(line);
        }
        if collecting && buf.contains(']') {
            break;
        }
    }
    if !collecting {
        return Err("没找到 [coverage.tier_a] 段下的 paths".into());
    }
    let inner = buf
        .trim()
        .strip_prefix('[')
        .and_then(|s| s.split(']').next())
        .ok_or_else(|| format!("paths 不是数组形态：{buf}"))?;
    let prefixes: Vec<String> = inner
        .split(',')
        .map(|s| s.trim().trim_matches('"').trim().to_string())
        .filter(|s| !s.is_empty())
        .collect();
    if prefixes.is_empty() {
        return Err("A 档路径清单为空，分档判定会恒真".into());
    }
    Ok(TierRules { a_prefixes: prefixes })
}

/// lcov 只取 `SF:` 与 `DA:` 两种记录，其余（分支、函数）本阶段门槛不涉及。
pub fn parse_lcov(text: &str) -> Result<Vec<FileCoverage>, String> {
    let mut out: Vec<FileCoverage> = Vec::new();
    let mut current: Option<FileCoverage> = None;
    for (i, raw) in text.lines().enumerate() {
        let line = raw.trim();
        let no = i + 1;
        if let Some(path) = line.strip_prefix("SF:") {
            if current.is_some() {
                return Err(format!("第 {no} 行又开了一个 SF，上一条没有 end_of_record"));
            }
            current = Some(FileCoverage { path: normalize(path), lines: BTreeMap::new() });
        } else if let Some(da) = line.strip_prefix("DA:") {
            let Some(f) = current.as_mut() else {
                return Err(format!("第 {no} 行的 DA 不属于任何 SF"));
            };
            let (l, h) = da
                .split_once(',')
                .ok_or_else(|| format!("第 {no} 行的 DA 不是「行号,次数」：{da}"))?;
            let l: u32 = l.trim().parse().map_err(|_| format!("第 {no} 行的行号非法：{l}"))?;
            // 次数列可能带校验和后缀，取第一段。
            let h = h.split(',').next().unwrap_or("0");
            let h: u64 = h.trim().parse().map_err(|_| format!("第 {no} 行的命中次数非法：{h}"))?;
            f.lines.insert(l, h);
        } else if line == "end_of_record" {
            let Some(f) = current.take() else {
                return Err(format!("第 {no} 行的 end_of_record 没有对应的 SF"));
            };
            out.push(f);
        }
    }
    if current.is_some() {
        return Err("最后一条 SF 没有 end_of_record".into());
    }
    Ok(out)
}

/// lcov 里的路径可能是绝对路径。统一成相对工作区根的形态，否则前缀匹配一条都不命中。
fn normalize(path: &str) -> String {
    let p = path.replace('\\', "/");
    for anchor in ["/crates/", "/apps/", "/tools/", "/testkit/", "/datagen/", "/xtask/"] {
        if let Some(i) = p.find(anchor) {
            return p[i + 1..].to_string();
        }
    }
    p.trim_start_matches("./").to_string()
}

fn load_lcov(root: &Path) -> Result<String, String> {
    let path = root.join(LCOV_PATH);
    if path.is_file() {
        return fs::read_to_string(&path).map_err(|e| format!("{} 读不到：{e}", path.display()));
    }
    // 工具不在时不得编造任何覆盖率数字，一律报未做出判定。
    let probe = Command::new(std::env::var("CARGO").unwrap_or_else(|_| "cargo".into()))
        .args(["llvm-cov", "--version"])
        .current_dir(root)
        .output();
    match probe {
        Ok(o) if o.status.success() => {}
        _ => {
            return Err(format!(
                "{} 不存在，且本机没有 cargo-llvm-cov：覆盖率工具不可用，判定未做出。\
                 本工具不会以任何方式估算覆盖率",
                path.display()
            ))
        }
    }
    let out = Command::new(std::env::var("CARGO").unwrap_or_else(|_| "cargo".into()))
        .args([
            "llvm-cov",
            "--workspace",
            "--locked",
            "--offline",
            "--lcov",
            "--output-path",
            LCOV_PATH,
        ])
        .current_dir(root)
        .output()
        .map_err(|e| format!("cargo llvm-cov 启动失败：{e}，判定未做出"))?;
    if !out.status.success() {
        return Err(format!(
            "cargo llvm-cov 返回 {}，判定未做出：{}",
            out.status,
            String::from_utf8_lossy(&out.stderr).trim()
        ));
    }
    fs::read_to_string(&path).map_err(|e| format!("{} 读不到：{e}", path.display()))
}

/// 增量行集合。取不到比对基是「判定未做出」，与「比对基存在但没有改动行」不同。
fn diff_lines(root: &Path) -> Result<BTreeMap<String, BTreeSet<u32>>, String> {
    let base = DIFF_BASES
        .iter()
        .find(|b| {
            Command::new("git")
                .args(["rev-parse", "--verify", "--quiet", b])
                .current_dir(root)
                .output()
                .map(|o| o.status.success())
                .unwrap_or(false)
        })
        .ok_or_else(|| {
            format!(
                "{} 一个都解析不出来，增量档的比对基取不到，该档判定未做出",
                DIFF_BASES.join(" 与 ")
            )
        })?;
    let out = Command::new("git")
        .args(["diff", "--unified=0", &format!("{base}...HEAD")])
        .current_dir(root)
        .output()
        .map_err(|e| format!("git diff 启动失败：{e}，增量档判定未做出"))?;
    if !out.status.success() {
        return Err(format!(
            "git diff 返回 {}，增量档判定未做出：{}",
            out.status,
            String::from_utf8_lossy(&out.stderr).trim()
        ));
    }
    Ok(parse_diff(&String::from_utf8_lossy(&out.stdout)))
}

/// 只取新增与修改行的行号，即 `@@` 头右侧的区间。
pub fn parse_diff(text: &str) -> BTreeMap<String, BTreeSet<u32>> {
    let mut out: BTreeMap<String, BTreeSet<u32>> = BTreeMap::new();
    let mut file: Option<String> = None;
    for line in text.lines() {
        if let Some(p) = line.strip_prefix("+++ ") {
            file = if p.trim() == "/dev/null" {
                None
            } else {
                Some(normalize(p.trim().trim_start_matches("b/")))
            };
            continue;
        }
        let Some(rest) = line.strip_prefix("@@ ") else { continue };
        let Some(f) = file.clone() else { continue };
        let Some(plus) = rest.split_whitespace().find(|t| t.starts_with('+')) else { continue };
        let spec = plus.trim_start_matches('+');
        let (start, count) = match spec.split_once(',') {
            Some((s, c)) => (s.parse::<u32>().ok(), c.parse::<u32>().ok()),
            None => (spec.parse::<u32>().ok(), Some(1)),
        };
        let (Some(start), Some(count)) = (start, count) else { continue };
        let entry = out.entry(f).or_default();
        for l in start..start + count {
            entry.insert(l);
        }
    }
    out
}

#[cfg(test)]
mod parse_negative_samples {
    use super::*;

    #[test]
    fn negative_rules_parse() {
        let ok = parse_rules("[coverage.tier_a]\npaths = [\"crates/foundation/\"]\n").expect("可解析");
        assert_eq!(ok.a_prefixes, ["crates/foundation/"]);
        // 多行数组同样受理。
        let multi = parse_rules("[coverage.tier_a]\npaths = [\n  \"crates/foundation/\",\n  \"crates/platform/\",\n]\n")
            .expect("可解析");
        assert_eq!(multi.a_prefixes.len(), 2);
        // 空清单会让 A 档恒真，必须报错而不是接受。
        assert!(parse_rules("[coverage.tier_a]\npaths = []\n").is_err());
        assert!(parse_rules("[coverage.tier_b]\npaths = [\"x\"]\n").is_err(), "段名不对");
    }

    #[test]
    fn negative_lcov_parse() {
        let v = parse_lcov("SF:/w/crates/foundation/src/a.rs\nDA:1,3\nDA:2,0\nend_of_record\n")
            .expect("可解析");
        assert_eq!(v.len(), 1);
        assert_eq!(v[0].path, "crates/foundation/src/a.rs", "绝对路径要归一到相对根");
        assert_eq!(v[0].covered(), 1);
        assert_eq!(v[0].missing(), [2]);
        assert!(parse_lcov("DA:1,1\n").is_err(), "DA 不属于任何 SF");
        assert!(parse_lcov("SF:a\nDA:1,1\n").is_err(), "少 end_of_record");
    }

    #[test]
    fn negative_diff_parse() {
        let d = parse_diff("+++ b/crates/foundation/src/a.rs\n@@ -1,0 +5,3 @@\n@@ -9 +12 @@\n");
        let lines = &d["crates/foundation/src/a.rs"];
        assert_eq!(lines.iter().copied().collect::<Vec<_>>(), [5, 6, 7, 12]);
        // 纯删除的块右侧计数为 0，不产生任何新增行。
        let none = parse_diff("+++ b/x.rs\n@@ -3,2 +2,0 @@\n");
        assert!(none["x.rs"].is_empty());
    }

    #[test]
    fn negative_classify() {
        let rules = TierRules { a_prefixes: vec!["crates/foundation/".into()] };
        assert_eq!(classify("crates/foundation/src/id/marker.rs", &rules), Tier::A);
        assert_eq!(classify("crates/platform/runtime/src/lib.rs", &rules), Tier::B);
        // 前缀不是子串：名字里含 foundation 的别处路径不该被吸进 A 档。
        assert_eq!(classify("apps/core-server/src/foundation_wiring.rs", &rules), Tier::B);
    }
}

#[cfg(test)]
mod rule_negative_samples {
    use super::*;

    fn rules() -> TierRules {
        TierRules { a_prefixes: vec!["crates/foundation/".into()] }
    }

    /// 造一个文件：`hit` 行命中，`miss` 行未命中。
    fn file(path: &str, hit: u32, miss: u32) -> FileCoverage {
        let mut lines = BTreeMap::new();
        for i in 0..hit {
            lines.insert(i + 1, 1);
        }
        for i in 0..miss {
            lines.insert(hit + i + 1, 0);
        }
        FileCoverage { path: path.into(), lines }
    }

    fn no_diff() -> BTreeMap<String, BTreeSet<u32>> {
        BTreeMap::new()
    }

    /// 正样例守边界：四档都达标时必须全绿。
    #[test]
    fn negative_all_tiers_green() {
        let files = [file("crates/foundation/src/a.rs", 90, 10), file("crates/platform/x/src/b.rs", 80, 20)];
        let r = evaluate(&files, &rules(), Some(&no_diff()));
        assert_eq!(r.outcome(), Outcome::Clean, "实得：{:?}", r.problems);
    }

    /// 负样例：人为降低 A 档覆盖率到门槛以下，整条规则必须判红并点名缺失行。
    #[test]
    fn negative_tier_a_below_threshold() {
        let files = [file("crates/foundation/src/a.rs", 84, 16), file("crates/platform/x/src/b.rs", 90, 10)];
        let r = evaluate(&files, &rules(), Some(&no_diff()));
        assert_eq!(r.outcome(), Outcome::Violated);
        let msg = r.problems.iter().find(|p| p.contains("A 档")).expect("须点名 A 档");
        assert!(msg.contains("84.00%"));
        assert!(msg.contains("crates/foundation/src/a.rs"), "须列出未达标文件");
        assert!(msg.contains("缺失行 85"), "须列出缺失行号");
    }

    /// 负样例：B 档单独不达标，A 档达标时也必须判红。
    #[test]
    fn negative_tier_b_below_threshold() {
        let files = [file("crates/foundation/src/a.rs", 99, 1), file("crates/platform/x/src/b.rs", 60, 40)];
        let r = evaluate(&files, &rules(), Some(&no_diff()));
        assert!(r.problems.iter().any(|p| p.contains("B 档")));
    }

    /// 负样例：两档各自达标而整体档不达标——分档不能替整体档背书。
    #[test]
    fn negative_overall_below_threshold() {
        // B 档恰好压线 70%，A 档 86%，两档都过而整体 71.45% 不过。
        let files = [file("crates/foundation/src/a.rs", 86, 14), file("crates/platform/x/src/b.rs", 700, 300)];
        let r = evaluate(&files, &rules(), Some(&no_diff()));
        assert!(r.problems.iter().any(|p| p.contains("整体档")), "实得：{:?}", r.problems);
        assert!(!r.problems.iter().any(|p| p.contains("A 档")));
    }

    /// 负样例：新增行没测到，前三档全绿也要判红。
    #[test]
    fn negative_diff_tier_below_threshold() {
        let files = [file("crates/foundation/src/a.rs", 90, 10), file("crates/platform/x/src/b.rs", 90, 10)];
        let mut diff = BTreeMap::new();
        // 该文件 91..100 是未命中行，全部算作本次新增。
        diff.insert("crates/foundation/src/a.rs".to_string(), (91u32..=100).collect::<BTreeSet<u32>>());
        let r = evaluate(&files, &rules(), Some(&diff));
        assert_eq!(r.outcome(), Outcome::Violated);
        let msg = r.problems.iter().find(|p| p.contains("增量档")).expect("须点名增量档");
        assert!(msg.contains("0.00%"));
        assert!(msg.contains("缺失行 91"));
    }

    /// 负样例：lcov 为空时必须是「判定未做出」，不是通过。空扫描不得判绿。
    #[test]
    fn negative_empty_lcov_is_undecidable() {
        let r = evaluate(&[], &rules(), Some(&no_diff()));
        assert_eq!(r.outcome(), Outcome::Undecidable);
        assert!(r.problems.is_empty());
        assert!(r.undecidable[0].contains("空扫描不是通过"));
    }

    /// 负样例：A 档路径下一个文件都没有时不得判达标。
    #[test]
    fn negative_empty_tier_is_undecidable() {
        let files = [file("crates/platform/x/src/b.rs", 90, 10)];
        let r = evaluate(&files, &rules(), Some(&no_diff()));
        assert_eq!(r.outcome(), Outcome::Undecidable);
        assert!(r.undecidable.iter().any(|u| u.contains("A 档")));
    }

    /// 负样例：增量行取不到与增量行为空是两件事，前者判未做出，后者如实说明。
    #[test]
    fn negative_diff_unavailable_differs_from_diff_empty() {
        let files = [file("crates/foundation/src/a.rs", 90, 10), file("crates/platform/x/src/b.rs", 90, 10)];
        let unavailable = evaluate(&files, &rules(), None);
        assert_eq!(unavailable.outcome(), Outcome::Undecidable);
        assert!(unavailable.undecidable.iter().any(|u| u.contains("增量档判定未做出")));

        let empty = evaluate(&files, &rules(), Some(&no_diff()));
        assert_eq!(empty.outcome(), Outcome::Clean);
        assert!(empty.notes.iter().any(|n| n.contains("该档无被测对象")));
    }
}
