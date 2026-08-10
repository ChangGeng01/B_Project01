//! `xtask eventcatalog` —— `docs/event-catalog.md` 与代码中事件类型的一致性。
//!
//! 判据出处是该文件第 5 节「机器判定」第一条与阶段 1 退出条件 10。本模块判三件事：
//!
//! 一、第 3 节的模块段清单与 `crates/foundation/src/module.rs` 的 `ModuleCode` 逐项相等，
//!     `platform` 段在最前。这一条在本阶段就有完整的被测输入，是本子命令唯一必然生效的判据。
//! 二、登记表中的事件类型合第 1 节的四段命名，模块段取自上述清单，且不得重复登记。
//! 三、登记表与代码中出现的事件类型字面量双向比对，两侧各自多出的都要报。
//!
//! **空扫描不判通过。** 阶段 1 登记的事件数为 0，代码里也没有任何事件类型字面量
//! （`docs/event-catalog.md` 第 4 节），第三条判据本阶段没有被测输入。按技术基线第 12 节
//! 通则第六条，它落进 [`Report::uncovered`] 并由调用方以专用退出码结束，不得读作通过；
//! 重新生效的触发谓词是「两侧任一出现第一个事件类型」，由本工具自身可观测。
//!
//! 代码侧一律文本解析，不 use 任何 ep-* crate。

use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};

const DOC: &str = "docs/event-catalog.md";
const MODULE_RS: &str = "crates/foundation/src/module.rs";
/// 代码侧的扫描面，与退出条件 21 给 archcheck 定的五个目录同口径；`xtask/` 自身不在内。
const SCAN_DIRS: [&str; 5] = ["crates", "apps", "testkit", "datagen", "tools"];
/// 平台事件的模块段，出处是 `docs/event-catalog.md` 第 1 节第二条硬约束。
const PLATFORM_SEGMENT: &str = "platform";

#[derive(Debug)]
pub struct Report {
    pub problems: Vec<String>,
    /// 未覆盖：判据已实现但本次运行没有被测输入。不得读作通过。
    pub uncovered: Vec<String>,
    /// 已比对的事件类型条数。
    pub compared: usize,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Outcome {
    Clean,
    Uncovered,
    Violated,
}

impl Report {
    pub fn outcome(&self) -> Outcome {
        if !self.problems.is_empty() {
            Outcome::Violated
        } else if !self.uncovered.is_empty() {
            Outcome::Uncovered
        } else {
            Outcome::Clean
        }
    }
}

pub fn run(root: &Path) -> Report {
    let mut problems = Vec::new();
    let mut uncovered = Vec::new();

    let doc = match fs::read_to_string(root.join(DOC)) {
        Ok(t) => t,
        Err(_) => {
            return Report {
                problems: vec![format!("读不到 {DOC}")],
                uncovered,
                compared: 0,
            }
        }
    };

    // 一、模块段清单。
    let segments = match doc_module_segments(&doc) {
        Ok(v) => v,
        Err(e) => {
            problems.push(e);
            Vec::new()
        }
    };
    match code_module_codes(root) {
        Err(e) => problems.push(e),
        Ok(codes) => {
            let want: Vec<String> =
                std::iter::once(PLATFORM_SEGMENT.to_string()).chain(codes).collect();
            if !segments.is_empty() && segments != want {
                problems.push(format!(
                    "{DOC} 第 3 节的模块段清单与 {MODULE_RS} 的 ModuleCode 不一致。\n      文档：{}\n      代码：{}",
                    segments.join("、"),
                    want.join("、")
                ));
            }
        }
    }

    // 二、登记表自身的命名与重复。
    let registered = doc_events(&doc);
    let mut seen: BTreeMap<&str, usize> = BTreeMap::new();
    for e in &registered {
        *seen.entry(e.as_str()).or_insert(0) += 1;
    }
    for (name, n) in seen.iter().filter(|(_, n)| **n > 1) {
        problems.push(format!("{DOC} 中 {name} 登记了 {n} 次；一个事件类型只能由一个阶段登记"));
    }
    for e in &registered {
        if let Err(why) = check_event_name(e, &segments) {
            problems.push(format!("{DOC} 的登记项 {e} 不合第 1 节命名：{why}"));
        }
    }

    // 三、与代码侧字面量双向比对。
    let in_code = code_events(root);
    if registered.is_empty() && in_code.is_empty() {
        uncovered.push(format!(
            "未覆盖：{DOC} 的登记表为空，{} 五个目录下也没有任何事件类型字面量，双向比对本次无被测输入",
            SCAN_DIRS.join("、")
        ));
    } else {
        for (name, files) in &in_code {
            if !registered.iter().any(|r| r == name) {
                problems.push(format!(
                    "代码中出现事件类型 {name}（{}），{DOC} 中没有登记；未登记的 event_type 一律拒绝",
                    files.join("、")
                ));
            }
        }
        for name in &registered {
            if !in_code.contains_key(name) {
                problems.push(format!("{DOC} 登记了 {name}，代码中没有任何产生它的位置"));
            }
        }
    }

    Report { problems, uncovered, compared: registered.len().max(in_code.len()) }
}

/// 第 3 节那一行 `` `platform`、`mdm`、… `` 的反引号词元，按出现顺序。
fn doc_module_segments(doc: &str) -> Result<Vec<String>, String> {
    let line = doc
        .lines()
        .map(str::trim)
        .find(|l| l.starts_with(&format!("`{PLATFORM_SEGMENT}`、")))
        .ok_or_else(|| format!("{DOC} 第 3 节找不到以 `{PLATFORM_SEGMENT}`、开头的模块段清单行"))?;
    let tokens = backticked(line);
    if tokens.is_empty() {
        return Err(format!("{DOC} 的模块段清单行解析不出任何词元"));
    }
    Ok(tokens)
}

fn backticked(line: &str) -> Vec<String> {
    let mut out = Vec::new();
    let mut rest = line;
    while let Some(start) = rest.find('`') {
        let after = &rest[start + 1..];
        let Some(end) = after.find('`') else { break };
        out.push(after[..end].to_string());
        rest = &after[end + 1..];
    }
    out
}

/// `pub enum ModuleCode { … }` 的变体名，转成事件名里的模块段形态（小写下划线）。
fn code_module_codes(root: &Path) -> Result<Vec<String>, String> {
    let text =
        fs::read_to_string(root.join(MODULE_RS)).map_err(|_| format!("读不到 {MODULE_RS}"))?;
    let body = text
        .split_once("pub enum ModuleCode {")
        .ok_or_else(|| format!("{MODULE_RS} 中找不到 pub enum ModuleCode"))?
        .1;
    let body = body.split('}').next().unwrap_or(body);
    let out: Vec<String> = body
        .lines()
        .map(str::trim)
        .filter(|l| !l.is_empty() && !l.starts_with("//"))
        .map(|l| l.trim_end_matches(',').trim())
        .filter(|l| l.chars().next().is_some_and(char::is_uppercase))
        .map(camel_to_snake)
        .collect();
    if out.is_empty() {
        return Err(format!("{MODULE_RS} 的 ModuleCode 体内没有解析出任何变体"));
    }
    Ok(out)
}

fn camel_to_snake(name: &str) -> String {
    let mut out = String::new();
    for (i, c) in name.chars().enumerate() {
        if c.is_ascii_uppercase() && i > 0 {
            out.push('_');
        }
        out.push(c.to_ascii_lowercase());
    }
    out
}

/// 登记表里的事件类型：表格行的第一格，去掉反引号后至少含两个点。
///
/// 用「至少两个点」而不是「合法四段」来取行，是为了让写坏的登记行也进得来、
/// 被第 1 节命名判据挑出来；用「合法四段」取行会让写坏的行直接消失。
fn doc_events(doc: &str) -> Vec<String> {
    doc.lines()
        .map(str::trim)
        .filter(|l| l.starts_with('|'))
        .filter_map(|l| l.trim_matches('|').split('|').next())
        .map(|c| c.trim().trim_matches('`').trim().to_string())
        .filter(|c| c.matches('.').count() >= 2)
        .collect()
}

/// 第 1 节的四段命名。`segments` 为空时跳过模块段一维，避免上一条判据失败后连锁误报。
fn check_event_name(name: &str, segments: &[String]) -> Result<(), String> {
    let parts: Vec<&str> = name.split('.').collect();
    if parts.len() != 4 {
        return Err(format!("应为四段 <module>.<aggregate>.<past_participle>.v<major>，实际 {} 段", parts.len()));
    }
    for p in &parts[..3] {
        if p.is_empty() || !p.chars().all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '_') {
            return Err(format!("段「{p}」只允许小写字母、数字与下划线"));
        }
    }
    let ver = parts[3];
    let digits = ver.strip_prefix('v').unwrap_or("");
    if digits.is_empty() || !digits.chars().all(|c| c.is_ascii_digit()) {
        return Err(format!("版本段「{ver}」应为 v<主版本号>"));
    }
    if !segments.is_empty() && !segments.iter().any(|s| s == parts[0]) {
        return Err(format!("模块段「{}」不在第 3 节的模块段清单内", parts[0]));
    }
    Ok(())
}

/// 代码侧：扫描面内 `.rs` 文件中的双引号字面量，取合四段形态者。
fn code_events(root: &Path) -> BTreeMap<String, Vec<String>> {
    let mut out: BTreeMap<String, Vec<String>> = BTreeMap::new();
    for dir in SCAN_DIRS {
        for path in rust_files(&root.join(dir)) {
            let Ok(text) = fs::read_to_string(&path) else { continue };
            let rel = path.strip_prefix(root).unwrap_or(&path).to_string_lossy().replace('\\', "/");
            for lit in string_literals(&text) {
                if check_event_name(&lit, &[]).is_ok() {
                    out.entry(lit).or_default().push(rel.clone());
                }
            }
        }
    }
    out
}

fn rust_files(dir: &Path) -> Vec<PathBuf> {
    let mut out = Vec::new();
    let Ok(entries) = fs::read_dir(dir) else { return out };
    let mut paths: Vec<_> = entries.filter_map(Result::ok).map(|e| e.path()).collect();
    paths.sort();
    for p in paths {
        if p.is_dir() {
            out.extend(rust_files(&p));
        } else if p.extension().is_some_and(|e| e == "rs") {
            out.push(p);
        }
    }
    out
}

/// 取双引号字面量。不处理转义与原始字符串，多取少取都只影响候选集，
/// 真正的判定由 [`check_event_name`] 完成。
fn string_literals(text: &str) -> Vec<String> {
    let mut out = Vec::new();
    for line in text.lines() {
        let mut rest = line;
        while let Some(start) = rest.find('"') {
            let after = &rest[start + 1..];
            let Some(end) = after.find('"') else { break };
            out.push(after[..end].to_string());
            rest = &after[end + 1..];
        }
    }
    out
}

#[cfg(test)]
mod negative_samples {
    use super::*;

    fn repo_root() -> PathBuf {
        Path::new(env!("CARGO_MANIFEST_DIR")).parent().expect("xtask 在工作区根之下").to_path_buf()
    }

    /// 仓库现状：模块段一致、登记表为空、代码侧无字面量，因此是「未覆盖」而不是「通过」。
    #[test]
    fn the_repository_is_uncovered_not_clean() {
        let r = run(&repo_root());
        assert!(r.problems.is_empty(), "模块段与命名判据应全绿：{:#?}", r.problems);
        assert_eq!(r.outcome(), Outcome::Uncovered, "空登记表不得判 Clean");
        assert!(r.uncovered[0].contains("双向比对"));
    }

    /// 负样例断言模块段清单这条规则本身：文档少一段即报，且报的是这一条。
    #[test]
    fn negative_module_segment_roster_mismatch_is_caught() {
        let doc = fs::read_to_string(repo_root().join(DOC)).expect("读得到事件目录");
        let segments = doc_module_segments(&doc).expect("清单行可解析");
        let codes = code_module_codes(&repo_root()).expect("ModuleCode 可解析");
        assert_eq!(segments.len(), codes.len() + 1, "文档段数应为 ModuleCode 项数加 platform");
        assert_eq!(segments[0], PLATFORM_SEGMENT);
        assert_eq!(segments[1..].to_vec(), codes);

        let tampered = doc.replace("、`ledger`", "");
        let got = doc_module_segments(&tampered).expect("清单行仍在");
        assert_ne!(got, segments, "删一段必须与代码侧不再相等");
    }

    /// 负样例断言四段命名这条规则本身，逐类给一个反例。
    #[test]
    fn negative_event_names_are_rejected_by_shape() {
        let segs: Vec<String> = ["platform", "sales"].iter().map(|s| s.to_string()).collect();
        assert!(check_event_name("sales.order.created.v1", &segs).is_ok());
        // 命令式、段数不足、模块段未登记、版本段写错，四类都要拒。
        assert!(check_event_name("order.create", &segs).is_err());
        assert!(check_event_name("sales.order.created", &segs).is_err());
        assert!(check_event_name("unknown.order.created.v1", &segs).is_err());
        assert!(check_event_name("sales.order.created.v", &segs).is_err());
        assert!(check_event_name("sales.Order.created.v1", &segs).is_err());
    }

    /// 负样例断言双向比对这条规则本身：两个方向各报一条，不能只报一边。
    #[test]
    fn negative_comparison_reports_both_directions() {
        let doc_only = "| `sales.order.created.v1` | sales.orders | 提交时 | — | 暂无（阶段 6） | 阶段 5 |";
        let found = doc_events(doc_only);
        assert_eq!(found, vec!["sales.order.created.v1"]);
        // 表头行与说明行不得被当成登记项。
        assert!(doc_events("| 事件类型 | 聚合类型 |").is_empty());
        // 写坏的登记行必须进得来，交给命名判据挑出，而不是在取行时消失。
        assert_eq!(doc_events("| `sales.order.created` |"), vec!["sales.order.created"]);
    }

    #[test]
    fn negative_literal_scan_picks_only_four_segment_names() {
        let lits = string_literals(r#"let a = "sales.order.created.v1"; let b = "not an event";"#);
        assert!(lits.contains(&"sales.order.created.v1".to_string()));
        assert!(lits.contains(&"not an event".to_string()));
        assert!(check_event_name("not an event", &[]).is_err());
    }

    #[test]
    fn negative_camel_to_snake() {
        assert_eq!(camel_to_snake("Mdm"), "mdm");
        assert_eq!(camel_to_snake("Reporting"), "reporting");
        assert_eq!(camel_to_snake("PlatformCore"), "platform_core");
    }
}
