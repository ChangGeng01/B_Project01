//! `xtask sbom` —— SBOM 与供应链门禁。
//!
//! 阶段 1 退出条件 14：SBOM 生成成功，`cargo deny` 与依赖漏洞扫描零严重与高危，
//! 许可证清单通过；另含一个断言 SBOM 中不出现 `ep-bench` 与 `ep-release-gate`
//! 两个包名的负样例，与阶段 14 的发布门禁项 `RG-TOOLS-EXCLUDED` 同名同判据。
//!
//! 该负样例按退出条件 14 原文以**手写 SBOM 夹具**构造，不以真实构建产物构造。
//! 这一支的必要性见裁定登记 00c 附录庚二 X-3：两个 crate 当前确实在工作区内，
//! 与退出条件 14「本阶段工作区内不存在」一句冲突，X-3 尚未裁定。手写夹具这一支
//! 在两种裁定下都成立——两包在或不在，规则本身都被真的执行过一次并判红。
//! 本模块因此既不去删那两个 crate，也不假装它们不存在。
//!
//! 三态纪律。SBOM 尚未产出、`deny.toml` 缺席、cargo-deny 不在，三者一律是
//! 「判定未做出」；SBOM 在而含被排除包名、许可证越界，才是不符。空 SBOM 不得判通过。

use std::collections::BTreeSet;
use std::fs;
use std::path::Path;
use std::process::Command;

use crate::archcheck::Outcome;

/// SBOM 落点。同 [`crate::sign`]，落在 `target/` 之下，不新增顶层目录。
pub const SBOM_PATH: &str = "target/sbom/ep-workspace.cdx.json";
/// 许可证允许清单的唯一来源。缺席时许可证一档判定未做出，不自造一份允许清单。
pub const DENY_DOC: &str = "deny.toml";

/// 与阶段 14 发布门禁项同名。改名要两处同批改。
pub const RULE_TOOLS_EXCLUDED: &str = "RG-TOOLS-EXCLUDED";

/// 按总览 B-11 排除出制品的两个包。位于 `tools/bench/` 与 `tools/release-gate/`。
pub const EXCLUDED_PACKAGES: [&str; 2] = ["ep-bench", "ep-release-gate"];

#[derive(Debug, Default)]
pub struct Report {
    /// 判不符：SBOM 读到了，但不满足判据。
    pub problems: Vec<String>,
    /// 判定未做出：被测对象或判定工具缺席。绝不折算为通过。
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

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Component {
    pub name: String,
    pub version: String,
    /// 许可证标识，可能有多个（`OR` 关系在 CycloneDX 里就是多条）。
    pub licenses: Vec<String>,
}

pub fn run(root: &Path) -> Report {
    let mut r = Report::default();

    let sbom_path = root.join(SBOM_PATH);
    let text = match fs::read_to_string(&sbom_path) {
        Ok(t) => t,
        Err(e) => {
            r.undecidable.push(format!(
                "{} 读不到（{e}）。本阶段尚未产出制品与 SBOM，供应链判定未做出，不得据此判通过",
                sbom_path.display()
            ));
            return r;
        }
    };

    let allowed = match fs::read_to_string(root.join(DENY_DOC)) {
        Ok(t) => match parse_allowed_licenses(&t) {
            Ok(v) => Some(v),
            Err(msg) => {
                r.problems.push(format!("{DENY_DOC} 解析失败：{msg}"));
                return r;
            }
        },
        Err(_) => None,
    };

    let mut inner = evaluate(&text, allowed.as_deref());
    r.problems.append(&mut inner.problems);
    r.undecidable.append(&mut inner.undecidable);
    r.notes.append(&mut inner.notes);

    let (p, u) = cargo_deny(root);
    r.problems.extend(p);
    r.undecidable.extend(u);
    r
}

/// 判据本体。以 SBOM 文本为被测对象，因此负样例直接喂一份手写 SBOM 即可。
pub fn evaluate(sbom_text: &str, allowed_licenses: Option<&[String]>) -> Report {
    let mut r = Report::default();
    let components = match parse_sbom(sbom_text) {
        Ok(v) => v,
        Err(msg) => {
            r.problems.push(format!("SBOM 解析失败：{msg}"));
            return r;
        }
    };
    if components.is_empty() {
        r.problems
            .push("SBOM 中一个组件都没有。空 SBOM 会让下面每一条断言恒真，按不符处理".into());
        return r;
    }

    r.problems.extend(tools_excluded(&components));
    match allowed_licenses {
        None => r.undecidable.push(format!(
            "{DENY_DOC} 不存在，许可证允许清单没有来源，许可证一档判定未做出；\
             本工具不自造允许清单"
        )),
        Some(allowed) => r.problems.extend(license_roster(&components, allowed)),
    }
    r.notes.push(format!("SBOM 含 {} 个组件", components.len()));
    r
}

/// `RG-TOOLS-EXCLUDED`：两个只在研发期使用的 crate 不得进入制品的 SBOM。
pub fn tools_excluded(components: &[Component]) -> Vec<String> {
    let mut out = Vec::new();
    for excluded in EXCLUDED_PACKAGES {
        for c in components.iter().filter(|c| c.name == excluded) {
            out.push(format!(
                "[{RULE_TOOLS_EXCLUDED}] SBOM 中出现 {} {}，该包按总览 B-11 不随产品交付",
                c.name, c.version
            ));
        }
    }
    out
}

/// 许可证清单：每个组件都要有许可证，且都在允许清单内。
pub fn license_roster(components: &[Component], allowed: &[String]) -> Vec<String> {
    let allow: BTreeSet<&str> = allowed.iter().map(String::as_str).collect();
    let mut out = Vec::new();
    for c in components {
        if c.licenses.is_empty() {
            out.push(format!(
                "{} {} 没有许可证标识，许可证清单不完整",
                c.name, c.version
            ));
            continue;
        }
        // 多个标识是 OR 关系，命中任一即可。
        if c.licenses.iter().any(|l| allow.contains(l.as_str())) {
            continue;
        }
        out.push(format!(
            "{} {} 的许可证 {} 不在 {DENY_DOC} 的允许清单内",
            c.name,
            c.version,
            c.licenses.join(" OR ")
        ));
    }
    out
}

/// 只读 CycloneDX 的 `components` 数组，三个字段：name、version、licenses。
pub fn parse_sbom(text: &str) -> Result<Vec<Component>, String> {
    let doc: serde_json::Value = serde_json::from_str(text).map_err(|e| e.to_string())?;
    match doc.get("bomFormat").and_then(|v| v.as_str()) {
        Some("CycloneDX") => {}
        Some(other) => return Err(format!("bomFormat 为 {other}，只受理 CycloneDX")),
        None => return Err("没有 bomFormat 字段，不是一份可判定的 SBOM".into()),
    }
    let items = doc
        .get("components")
        .and_then(|v| v.as_array())
        .ok_or_else(|| "没有 components 数组".to_string())?;

    let mut out = Vec::new();
    for (i, item) in items.iter().enumerate() {
        let name = item
            .get("name")
            .and_then(|v| v.as_str())
            .ok_or_else(|| format!("第 {} 个组件没有 name", i + 1))?;
        let version = item.get("version").and_then(|v| v.as_str()).unwrap_or("");
        out.push(Component {
            name: name.to_string(),
            version: version.to_string(),
            licenses: licenses_of(item),
        });
    }
    Ok(out)
}

/// CycloneDX 的许可证有两种写法：`license.id` 与 `expression`，两种都取。
fn licenses_of(item: &serde_json::Value) -> Vec<String> {
    let Some(list) = item.get("licenses").and_then(|v| v.as_array()) else {
        return Vec::new();
    };
    let mut out = Vec::new();
    for entry in list {
        if let Some(id) = entry
            .get("license")
            .and_then(|l| l.get("id"))
            .and_then(|v| v.as_str())
        {
            out.push(id.to_string());
        } else if let Some(expr) = entry.get("expression").and_then(|v| v.as_str()) {
            // 表达式按 OR 拆开；AND 组合本工具不拆，整串留给允许清单比对。
            out.extend(expr.split(" OR ").map(|s| s.trim().to_string()));
        }
    }
    out
}

/// `deny.toml` 的 `[licenses] allow = [...]`。解析方式与 [`crate::coverage::parse_rules`] 同款。
pub fn parse_allowed_licenses(text: &str) -> Result<Vec<String>, String> {
    let mut in_section = false;
    let mut collecting = false;
    let mut buf = String::new();
    for line in text.lines() {
        let line = line.split('#').next().unwrap_or_default().trim();
        if line.starts_with('[') {
            in_section = line == "[licenses]";
            continue;
        }
        if !in_section {
            continue;
        }
        if let Some((key, value)) = line.split_once('=') {
            if key.trim() != "allow" {
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
        return Err("没找到 [licenses] 段下的 allow".into());
    }
    let inner = buf
        .trim()
        .strip_prefix('[')
        .and_then(|s| s.split(']').next())
        .ok_or_else(|| format!("allow 不是数组形态：{buf}"))?;
    let list: Vec<String> = inner
        .split(',')
        .map(|s| s.trim().trim_matches('"').trim().to_string())
        .filter(|s| !s.is_empty())
        .collect();
    if list.is_empty() {
        return Err("允许清单为空，许可证判定会恒真".into());
    }
    Ok(list)
}

/// 依赖漏洞与禁用依赖扫描。工具不在时判定未做出，不折算为零严重零高危。
fn cargo_deny(root: &Path) -> (Vec<String>, Vec<String>) {
    let cargo = std::env::var("CARGO").unwrap_or_else(|_| "cargo".into());
    let probe = Command::new(&cargo)
        .args(["deny", "--version"])
        .current_dir(root)
        .output();
    match probe {
        Ok(o) if o.status.success() => {}
        _ => {
            return (
                Vec::new(),
                vec![
                    "本机没有 cargo-deny：依赖漏洞、禁用依赖与许可证三项扫描未做出判定，\
                     不得据此判零严重零高危"
                        .into(),
                ],
            )
        }
    }
    let out = Command::new(&cargo)
        .args([
            "deny",
            "--offline",
            "check",
            "advisories",
            "bans",
            "licenses",
        ])
        .current_dir(root)
        .output();
    match out {
        Err(e) => (
            Vec::new(),
            vec![format!("cargo deny 启动失败：{e}，扫描判定未做出")],
        ),
        Ok(o) if o.status.success() => (Vec::new(), Vec::new()),
        Ok(o) => (
            vec![format!(
                "cargo deny 返回 {}：\n{}",
                o.status,
                String::from_utf8_lossy(&o.stderr).trim()
            )],
            Vec::new(),
        ),
    }
}

#[cfg(test)]
mod parse_negative_samples {
    use super::*;

    #[test]
    fn negative_sbom_parse() {
        assert!(parse_sbom("{}").is_err(), "没有 bomFormat");
        assert!(
            parse_sbom(r#"{"bomFormat":"SPDX","components":[]}"#).is_err(),
            "格式不受理"
        );
        assert!(
            parse_sbom(r#"{"bomFormat":"CycloneDX"}"#).is_err(),
            "没有 components"
        );
        assert!(
            parse_sbom(r#"{"bomFormat":"CycloneDX","components":[{"version":"1"}]}"#).is_err(),
            "组件没有 name"
        );
    }

    #[test]
    fn negative_license_extraction() {
        let doc = r#"{"bomFormat":"CycloneDX","components":[
            {"name":"a","version":"1","licenses":[{"license":{"id":"MIT"}}]},
            {"name":"b","version":"2","licenses":[{"expression":"MIT OR Apache-2.0"}]},
            {"name":"c","version":"3"}
        ]}"#;
        let c = parse_sbom(doc).expect("可解析");
        assert_eq!(c[0].licenses, ["MIT"]);
        assert_eq!(c[1].licenses, ["MIT", "Apache-2.0"]);
        assert!(
            c[2].licenses.is_empty(),
            "没有许可证要如实为空，不得默认一个"
        );
    }

    #[test]
    fn negative_allow_list_parse() {
        let v = parse_allowed_licenses("[licenses]\nallow = [\"MIT\", \"Apache-2.0\"]\n")
            .expect("可解析");
        assert_eq!(v, ["MIT", "Apache-2.0"]);
        assert!(
            parse_allowed_licenses("[licenses]\nallow = []\n").is_err(),
            "空清单会恒真"
        );
        assert!(
            parse_allowed_licenses("[bans]\nallow = [\"MIT\"]\n").is_err(),
            "段名不对"
        );
    }
}

#[cfg(test)]
mod rule_negative_samples {
    use super::*;

    fn allow() -> Vec<String> {
        vec!["MIT".to_string(), "Apache-2.0".to_string()]
    }

    /// 手写 SBOM 夹具。退出条件 14 明写负样例一律由手写夹具构造，
    /// 不以真实构建产物构造，也不因两包缺席而把断言留成恒真。
    fn fixture(extra: &str) -> String {
        format!(
            r#"{{"bomFormat":"CycloneDX","specVersion":"1.5","components":[
                {{"name":"ep-foundation","version":"0.1.0","licenses":[{{"license":{{"id":"MIT"}}}}]}},
                {{"name":"serde","version":"1.0.0","licenses":[{{"expression":"MIT OR Apache-2.0"}}]}}
                {extra}
            ]}}"#
        )
    }

    /// 正样例守边界：干净的 SBOM 必须全绿，否则下面的负样例证明不了任何事。
    #[test]
    fn negative_clean_sbom_is_green() {
        let r = evaluate(&fixture(""), Some(&allow()));
        assert_eq!(r.outcome(), Outcome::Clean, "实得：{:?}", r.problems);
    }

    /// 负样例：夹具中人为写入两个被排除的包名，规则本体必须判其不通过。
    #[test]
    fn negative_excluded_tool_packages_in_sbom() {
        let extra = r#",
            {"name":"ep-bench","version":"0.1.0","licenses":[{"license":{"id":"MIT"}}]},
            {"name":"ep-release-gate","version":"0.1.0","licenses":[{"license":{"id":"MIT"}}]}"#;
        let r = evaluate(&fixture(extra), Some(&allow()));
        assert_eq!(r.outcome(), Outcome::Violated);
        for name in EXCLUDED_PACKAGES {
            assert!(
                r.problems
                    .iter()
                    .any(|p| p.contains(RULE_TOOLS_EXCLUDED) && p.contains(name)),
                "{name} 未被点名，实得：{:?}",
                r.problems
            );
        }
    }

    /// 负样例：只有其中一个包也要判红，不能要求两个同时出现才报。
    #[test]
    fn negative_single_excluded_package() {
        let extra = r#",
            {"name":"ep-release-gate","version":"0.1.0","licenses":[{"license":{"id":"MIT"}}]}"#;
        let p = tools_excluded(&parse_sbom(&fixture(extra)).expect("可解析"));
        assert_eq!(p.len(), 1);
        assert!(p[0].contains("ep-release-gate"));
    }

    /// 负样例：许可证越出允许清单。
    #[test]
    fn negative_license_outside_allow_list() {
        let extra = r#",
            {"name":"gpl-thing","version":"9","licenses":[{"license":{"id":"GPL-3.0"}}]}"#;
        let r = evaluate(&fixture(extra), Some(&allow()));
        assert!(r
            .problems
            .iter()
            .any(|p| p.contains("GPL-3.0") && p.contains("允许清单")));
    }

    /// 负样例：组件没有许可证标识，同样不通过。
    #[test]
    fn negative_missing_license() {
        let extra = r#", {"name":"nolicense","version":"1"}"#;
        let r = evaluate(&fixture(extra), Some(&allow()));
        assert!(r
            .problems
            .iter()
            .any(|p| p.contains("nolicense") && p.contains("没有许可证标识")));
    }

    /// 负样例：空 SBOM 会让排除断言恒真，必须判不符而不是通过。
    #[test]
    fn negative_empty_sbom_must_not_pass() {
        let r = evaluate(
            r#"{"bomFormat":"CycloneDX","components":[]}"#,
            Some(&allow()),
        );
        assert_eq!(r.outcome(), Outcome::Violated);
        assert!(r.problems[0].contains("恒真"));
    }

    /// 负样例：没有允许清单时是「判定未做出」，不是许可证通过。
    #[test]
    fn negative_absent_allow_list_is_undecidable() {
        let r = evaluate(&fixture(""), None);
        assert_eq!(r.outcome(), Outcome::Undecidable);
        assert!(r.undecidable[0].contains("判定未做出"));
        assert!(r.problems.is_empty(), "排除断言仍然跑过且通过了");
    }
}
