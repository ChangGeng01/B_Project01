//! `undecidable-registry-matched` —— 基线第 12.1 节登记表与本工具运行期输出逐行比对。
//!
//! 这是通则第六条第三句「不得静默放行」的机械承接方：判据一旦被降级或标为
//! 不可判定，必须同时出现在文档登记表与工具输出两处，多一条少一条都判违反。
//! 该比对的被测输入只有登记表与工具自身输出两项，均在阶段 1 内已存在。

use std::fs;
use std::path::Path;

use super::deps::Violation;

const RULE: &str = "undecidable-registry-matched";
const DOC: &str = "docs/superpowers/plans/2026-08-10-first-release-dev-plan/00b-technical-baseline.md";
const SECTION: &str = "### 12.1 ";
const SPLIT: &str = "undecidable 段登记";

fn violation(detail: impl Into<String>) -> Violation {
    Violation { rule: RULE, package: DOC.to_string(), detail: detail.into() }
}

/// 登记表的一行：判据名与承接方两列参与比对，其余三列供人读。
#[derive(PartialEq, Eq, Debug)]
pub struct Entry {
    pub name: String,
    pub taker: String,
}

pub fn check(root: &Path, delegated: &[(&str, &str)], undecidable: &[(&str, String)]) -> Vec<Violation> {
    let path = root.join(DOC);
    let Ok(text) = fs::read_to_string(&path) else {
        return vec![violation("读不到基线第 12.1 节所在文件")];
    };
    let Some(section) = text.split_once(SECTION).map(|(_, rest)| rest) else {
        return vec![violation("基线中找不到第 12.1 节登记表")];
    };
    let section = section.split("\n## ").next().unwrap_or(section);
    let Some((head, tail)) = section.split_once(SPLIT) else {
        return vec![violation("第 12.1 节缺少 undecidable 段，两段不得合并成一张表")];
    };

    let mut found = Vec::new();
    found.extend(compare("delegated", &rows(head), &delegated
        .iter()
        .map(|(n, t)| Entry { name: (*n).to_string(), taker: normalize(t) })
        .collect::<Vec<_>>()));
    found.extend(compare("undecidable", &rows(tail), &undecidable
        .iter()
        .map(|(n, _)| Entry { name: (*n).to_string(), taker: String::new() })
        .collect::<Vec<_>>()));
    found
}

/// 抽表体行。表头与分隔行不算数据行。
fn rows(md: &str) -> Vec<Entry> {
    md.lines()
        .map(str::trim)
        .filter(|l| l.starts_with('|') && l.ends_with('|'))
        .map(|l| l.trim_matches('|').split('|').map(str::trim).collect::<Vec<_>>())
        .filter(|cells| cells.len() == 5)
        .filter(|cells| cells[0] != "判据名" && !cells[0].starts_with("---"))
        .map(|cells| Entry { name: cells[0].to_string(), taker: normalize(cells[3]) })
        .collect()
}

/// 比对前抹掉换行与连续空白，使 Rust 源码里的续行不影响等值判定。
fn normalize(s: &str) -> String {
    s.split_whitespace().collect::<Vec<_>>().join("")
}

fn compare(seg: &str, doc: &[Entry], code: &[Entry]) -> Vec<Violation> {
    let mut found = Vec::new();
    for d in doc {
        match code.iter().find(|c| c.name == d.name) {
            None => found.push(violation(format!(
                "{seg} 段登记了 {}，但本工具运行期输出中没有该项；登记表不得多于输出",
                d.name
            ))),
            Some(c) if !c.taker.is_empty() && c.taker != d.taker => found.push(violation(format!(
                "{seg} 段 {} 的承接方与本工具的登记不一致。\n      文档：{}\n      代码：{}",
                d.name, d.taker, c.taker
            ))),
            Some(_) => {}
        }
    }
    for c in code {
        if !doc.iter().any(|d| d.name == c.name) {
            found.push(violation(format!(
                "本工具输出了 {seg} 项 {}，但基线第 12.1 节 {seg} 段没有登记；不得静默放行",
                c.name
            )));
        }
    }
    found
}

#[cfg(test)]
mod negative_samples {
    use super::*;

    const TABLE: &str = "\n| 判据名 | 所在文件与小节 | 理由 | 承接方 | 重新生效或删除条件 |\n\
                         |---|---|---|---|---|\n\
                         | a/necessity | x | y | 承接方甲 | z |\n";

    #[test]
    fn negative_row_extraction() {
        let got = rows(TABLE);
        assert_eq!(got.len(), 1, "表头与分隔行不算数据行");
        assert_eq!(got[0].name, "a/necessity");
        assert_eq!(got[0].taker, "承接方甲");
    }

    #[test]
    fn negative_doc_has_extra_row() {
        let v = compare("delegated", &rows(TABLE), &[]);
        assert_eq!(v.len(), 1, "文档多一条即违反");
    }

    #[test]
    fn negative_code_has_extra_row() {
        let code = [Entry { name: "b".into(), taker: "承接方乙".into() }];
        let v = compare("delegated", &[], &code);
        assert_eq!(v.len(), 1, "工具多输出一条即违反");
    }

    #[test]
    fn negative_taker_drift() {
        let code = [Entry { name: "a/necessity".into(), taker: "承接方乙".into() }];
        let v = compare("delegated", &rows(TABLE), &code);
        assert_eq!(v.len(), 1, "承接方漂移即违反");
    }

    #[test]
    fn negative_whitespace_is_ignored() {
        let code = [Entry { name: "a/necessity".into(), taker: normalize("承接方\n     甲") }];
        assert!(compare("delegated", &rows(TABLE), &code).is_empty(), "续行不算差异");
    }
}
