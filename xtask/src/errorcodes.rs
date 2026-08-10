//! `xtask errorcodes` —— 错误码表与代码常量表逐项比对。
//!
//! 阶段 1 退出条件 7：十三条错误码在 `docs/error-codes.md` 与代码常量表中一致，
//! 重复码或缺失码即构建失败。
//!
//! 代码侧按文本解析 `codes.rs` 而不是 `use ep_foundation`，与 archcheck 的
//! `foundation-no-single-owner` 同一纪律：不给 xtask 加一条对 ep-foundation 的依赖边。

use std::collections::BTreeMap;
use std::fs;
use std::path::Path;

const DOC: &str = "docs/error-codes.md";
const CODE: &str = "crates/foundation/src/error/codes.rs";
/// 登记表所在小节。文案表在第 3 节，不参与本比对。
const DOC_SECTION: &str = "## 2. PLATFORM 段";

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Entry {
    pub code: String,
    pub category: String,
    pub http: String,
    pub retryable: String,
}

#[derive(Debug)]
pub struct Report {
    pub problems: Vec<String>,
    pub compared: usize,
}

impl Report {
    pub fn ok(&self) -> bool {
        self.problems.is_empty()
    }
}

pub fn run(root: &Path) -> Report {
    let mut problems = Vec::new();

    let doc = match read_doc(root) {
        Ok(v) => v,
        Err(e) => return Report { problems: vec![e], compared: 0 },
    };
    let code = match read_code(root) {
        Ok(v) => v,
        Err(e) => return Report { problems: vec![e], compared: 0 },
    };

    problems.extend(duplicates(&doc, DOC));
    problems.extend(duplicates(&code, CODE));

    let doc_map: BTreeMap<&str, &Entry> = doc.iter().map(|e| (e.code.as_str(), e)).collect();
    let code_map: BTreeMap<&str, &Entry> = code.iter().map(|e| (e.code.as_str(), e)).collect();

    for (name, d) in &doc_map {
        match code_map.get(name) {
            None => problems.push(format!("{DOC} 登记了 {name}，代码常量表中没有")),
            Some(c) if c != d => problems.push(format!(
                "{name} 两侧取值不一致。\n      文档：{} / {} / {}\n      代码：{} / {} / {}",
                d.category, d.http, d.retryable, c.category, c.http, c.retryable
            )),
            Some(_) => {}
        }
    }
    for name in code_map.keys() {
        if !doc_map.contains_key(name) {
            problems.push(format!("代码常量表有 {name}，{DOC} 中没有登记"));
        }
    }

    Report { problems, compared: doc_map.len().max(code_map.len()) }
}

fn duplicates(entries: &[Entry], where_: &str) -> Vec<String> {
    let mut seen: BTreeMap<&str, usize> = BTreeMap::new();
    for e in entries {
        *seen.entry(e.code.as_str()).or_insert(0) += 1;
    }
    seen.into_iter()
        .filter(|(_, n)| *n > 1)
        .map(|(c, n)| format!("{where_} 中 {c} 出现 {n} 次；重复码即失败"))
        .collect()
}

/// 只读第 2 节的表体，避开第 3 节的文案表。
fn read_doc(root: &Path) -> Result<Vec<Entry>, String> {
    let text = fs::read_to_string(root.join(DOC)).map_err(|_| format!("读不到 {DOC}"))?;
    let section = text
        .split_once(DOC_SECTION)
        .ok_or_else(|| format!("{DOC} 中找不到小节标题「{DOC_SECTION}」"))?
        .1;
    let section = section.split("\n## ").next().unwrap_or(section);
    let entries: Vec<Entry> = section
        .lines()
        .map(str::trim)
        .filter(|l| l.starts_with("| PLATFORM."))
        .filter_map(|l| {
            let c: Vec<&str> = l.trim_matches('|').split('|').map(str::trim).collect();
            (c.len() >= 4).then(|| Entry {
                code: c[0].to_string(),
                category: c[1].to_string(),
                http: c[2].to_string(),
                retryable: c[3].to_string(),
            })
        })
        .collect();
    if entries.is_empty() {
        return Err(format!("{DOC} 第 2 节没有解析出任何登记行"));
    }
    Ok(entries)
}

/// 解析 `codes! { 常量名 => "码", 分类, HTTP, 可重试; }` 的宏调用体。
fn read_code(root: &Path) -> Result<Vec<Entry>, String> {
    let text = fs::read_to_string(root.join(CODE)).map_err(|_| format!("读不到 {CODE}"))?;
    let body = text
        .split_once("\ncodes! {")
        .ok_or_else(|| format!("{CODE} 中找不到 codes! 宏调用"))?
        .1;
    let body = body.split("\n}").next().unwrap_or(body);
    let entries: Vec<Entry> = body
        .lines()
        .map(str::trim)
        .filter(|l| l.contains("=>") && l.ends_with(';'))
        .filter_map(parse_code_line)
        .collect();
    if entries.is_empty() {
        return Err(format!("{CODE} 的 codes! 体内没有解析出任何条目"));
    }
    Ok(entries)
}

fn parse_code_line(line: &str) -> Option<Entry> {
    let rhs = line.split_once("=>")?.1.trim_end_matches(';');
    let parts: Vec<&str> = rhs.split(',').map(str::trim).collect();
    if parts.len() != 4 {
        return None;
    }
    Some(Entry {
        code: parts[0].trim_matches('"').to_string(),
        category: camel_to_screaming(parts[1]),
        http: parts[2].to_string(),
        retryable: parts[3].to_string(),
    })
}

/// 代码里分类是 `BusinessConflict`，文档里是 `BUSINESS_CONFLICT`。
fn camel_to_screaming(name: &str) -> String {
    let mut out = String::new();
    for (i, c) in name.chars().enumerate() {
        if c.is_ascii_uppercase() && i > 0 {
            out.push('_');
        }
        out.push(c.to_ascii_uppercase());
    }
    out
}

#[cfg(test)]
mod negative_samples {
    use super::*;

    #[test]
    fn negative_camel_to_screaming() {
        assert_eq!(camel_to_screaming("BusinessConflict"), "BUSINESS_CONFLICT");
        assert_eq!(camel_to_screaming("Validation"), "VALIDATION");
        assert_eq!(camel_to_screaming("PermissionDenied"), "PERMISSION_DENIED");
    }

    #[test]
    fn negative_parse_code_line() {
        let e = parse_code_line(
            r#"PLATFORM_ROUTE_NOT_FOUND => "PLATFORM.ROUTE.NOT_FOUND", PermissionDenied, 404, false;"#,
        )
        .expect("可解析");
        assert_eq!(e.code, "PLATFORM.ROUTE.NOT_FOUND");
        assert_eq!(e.category, "PERMISSION_DENIED");
        assert_eq!(e.http, "404");
        assert_eq!(e.retryable, "false");
        // 列数不对的行不得被当成条目，宁可解析不出来也不要错配。
        assert!(parse_code_line("FOO => \"X\", Bar;").is_none());
    }

    #[test]
    fn negative_duplicate_detection() {
        let e = |c: &str| Entry {
            code: c.into(),
            category: "VALIDATION".into(),
            http: "400".into(),
            retryable: "false".into(),
        };
        assert!(duplicates(&[e("A"), e("B")], "x").is_empty());
        let dup = duplicates(&[e("A"), e("A")], "x");
        assert_eq!(dup.len(), 1);
        assert!(dup[0].contains("出现 2 次"));
    }
}
