//! `xtask configdoc` —— 登记文件与代码的一致性，共四段判据（第三段单据类型码走 `--check-doc-type-codes` 独立入口）。
//!
//! 一、配置键。退出条件 9：全部配置键在 `docs/config-reference.md` 中有条目，代码与文档
//!     逐键一致。代码侧由 `crates/platform/runtime/src/config/sections.rs` 的分段结构体与
//!     八个进程各自的配置根结构体合成完整键路径，两侧双向比对。
//! 二、指标名。退出条件 24：`docs/metrics-catalog.md` 的指标名唯一性校验，六个指标与代码
//!     注册表逐项一致，`ep_db_retries_total` 与 `ep_tx_retry_total` 两个名字不出现在任何
//!     登记文件与代码中。
//! 三、单据类型码，即 `--check-doc-type-codes`，入口见 [`run_doc_type_codes`]。
//! 四、路由能力元组。总览 A-20 与阶段 4 退出条件 17：每个 `/api/v1/` 路由都能解析到一个
//!     `(CapabilityDomain, ActionClass)` 元组，缺失即构建失败。判据落在**声明完整性**上，
//!     与「运行期是否真的按该元组判定」是两件事——后者今天在注册处被 `_capability` 丢弃，
//!     属附录辛第 26 条，本段不判、也不得被读作已判。
//!
//! 第四段的文本级判定规则与其三处已知限制：
//! 一、只扫 `apps/` 下**含 `.route(` 的**源码文件。守卫里的路径比较（形如
//!     `if path != "/api/v1/..."`）不构成路由注册，按此规则自然排除。
//! 二、`/api/v1/system/` 前缀豁免。依据是阶段 1 计划端点表权限列逐字「无，仅回环」，
//!     同计划另有一条 e2e 用例断言该前缀不对外暴露。
//! 三、关联判据取「相邻两个路径字面量之间须出现一次 `CapabilityDomain::`」，末条取到文本末。
//!     该规则认的是**同一声明块内的邻接**，不做语法树解析；若日后有人把元组写到
//!     下一条路径之后，本判据会漏。这一条是本段的已知上限，不写成「已覆盖」。
//!
//! 第三段的逐项比对按退出条件 23 与技术基线第 12 节通则第六条整条推迟到阶段 3a，
//! 因此它落进 [`Report::deferred`] 而不是 [`Report::uncovered`]：推迟是通则给出的三种
//! 合法处置之一，与「判据已实现但本次没有被测输入」不是一件事，两者不并成一段，
//! 也不参与同一个退出码。推迟段重新生效的触发谓词是「该节登记表出现第一行」，
//! 由本工具自身可观测，不写成阶段号。
//!
//! 代码侧一律文本解析，不 use 任何 ep-* crate。

use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};

const CONFIG_DOC: &str = "docs/config-reference.md";
const SECTIONS_RS: &str = "crates/platform/runtime/src/config/sections.rs";
const METRICS_DOC: &str = "docs/metrics-catalog.md";
const METRICS_RS: &str = "crates/platform/obs/src/metrics/registry.rs";
const DICT_DOC: &str = "docs/data-dictionary.md";
const SEQUENCE_SRC: &str = "crates/platform/sequence/src";
const APPS: &str = "apps";

/// 单据类型码一节的两个标题，出处是 `docs/data-dictionary.md` 与退出条件 23。
const DICT_SECTION: &str = "## 5. 单据类型码";
const DICT_TABLE: &str = "### 5.1 登记表";

// 退出条件 24 后半句「`ep_db_retries_total` 与 `ep_tx_retry_total` 两个名字不出现在
// 任何登记文件与代码中」本模块不判。实测 `crates/platform/obs/src/metrics/registry.rs`
// 第 296 与 297 行正以 `ep_db_retries_total` 作为「未登记指标一律拒绝」的负样例字面量，
// 按字面实现该判据会把一条正确的负样例判成违反。判据与既有代码的这处冲突已作为待裁定项
// 上报，裁定前不擅自改任何一侧，也不写一条会误伤的规则。

#[derive(Debug)]
pub struct Report {
    pub problems: Vec<String>,
    /// 未覆盖：判据已实现但本次运行没有被测输入。不得读作通过。
    pub uncovered: Vec<String>,
    /// 已裁定整条推迟的判据，附重新生效的触发谓词。不参与退出码判定。
    pub deferred: Vec<String>,
    /// 已比对的条目数，配置键与指标名合计。
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
    let mut compared = 0usize;

    compared += check_config_keys(root, &mut problems, &mut uncovered);
    compared += check_metrics(root, &mut problems, &mut uncovered);
    compared += check_route_capabilities(root, &mut problems, &mut uncovered);

    Report {
        problems,
        uncovered,
        deferred: Vec::new(),
        compared,
    }
}

// ---------------------------------------------------------------------------
// 一、配置键
// ---------------------------------------------------------------------------

fn check_config_keys(
    root: &Path,
    problems: &mut Vec<String>,
    uncovered: &mut Vec<String>,
) -> usize {
    let doc = match fs::read_to_string(root.join(CONFIG_DOC)) {
        Ok(t) => t,
        Err(_) => {
            problems.push(format!("读不到 {CONFIG_DOC}"));
            return 0;
        }
    };
    let doc_keys = doc_config_keys(&doc);
    if doc_keys.is_empty() {
        problems.push(format!("{CONFIG_DOC} 第 2 节没有解析出任何配置键"));
        return 0;
    }
    for (k, n) in duplicates(&doc_keys) {
        problems.push(format!(
            "{CONFIG_DOC} 中配置键 {k} 出现 {n} 次；登记表内不得重复"
        ));
    }

    let structs = match struct_fields(root, SECTIONS_RS) {
        Ok(s) if !s.is_empty() => s,
        _ => {
            uncovered.push(format!(
                "未覆盖：{SECTIONS_RS} 里没有解析出任何配置分段结构体"
            ));
            return 0;
        }
    };
    let roots = app_config_roots(root, &structs);
    if roots.is_empty() {
        uncovered.push(format!(
            "未覆盖：{APPS}/*/src/config.rs 里没有解析出任何配置根结构体"
        ));
        return 0;
    }

    let mut code_keys: Vec<String> = Vec::new();
    for (section, ty) in &roots {
        expand(section, ty, &structs, &mut Vec::new(), &mut code_keys);
    }
    code_keys.sort();
    code_keys.dedup();

    for key in &code_keys {
        if !doc_keys.iter().any(|d| key_matches(d, key)) {
            problems.push(format!("代码里有配置键 {key}，{CONFIG_DOC} 中没有登记"));
        }
    }
    for d in &doc_keys {
        if !code_keys.iter().any(|k| key_matches(d, k)) {
            problems.push(format!(
                "{CONFIG_DOC} 登记了 {d}，代码的配置结构体里没有对应字段"
            ));
        }
    }

    // 已删除的两个键不得再引入，出处是 config-reference 第 6 节。
    for removed in removed_keys(&doc) {
        if code_keys.iter().any(|k| key_matches(&removed, k)) {
            problems.push(format!("配置键 {removed} 已随裁定删除，任何阶段不得再引入"));
        }
    }
    code_keys.len()
}

/// 第 2 节各表的第一格。`<池>` 一类占位段原样保留，由 [`key_matches`] 当通配段处理。
fn doc_config_keys(doc: &str) -> Vec<String> {
    doc.lines()
        .map(str::trim)
        .filter(|l| l.starts_with('|'))
        .filter_map(|l| l.trim_matches('|').split('|').next())
        .map(|c| c.trim().trim_matches('`').trim().to_string())
        .filter(|c| c.contains('.') && !c.contains(' '))
        .collect()
}

/// 第 6 节列的已删除键。
fn removed_keys(doc: &str) -> Vec<String> {
    let Some(section) = doc.split("## 6. ").nth(1) else {
        return Vec::new();
    };
    let section = section.split("\n## ").next().unwrap_or(section);
    section
        .lines()
        .map(str::trim)
        .filter(|l| l.starts_with("- `"))
        // 已证实的读取缺陷（00c F-68 结论三登记，暂不改行为）：本行只取每条 bullet 的
        // 第一个反引号词元，而第 6 节有三条 bullet 一行列多个键（`config-reference.md`
        // 的 admission 五键、notify/portal 三键、ops 三键），其余 8 个键从未被本判据读到。
        // 正确读法是取首个「：」之前的全部词元——「：」之后的是取代它的现行键，不得取。
        // 不当轮改行为的理由：这 8 个键里有 5 个今天已被「代码里有、文档没登记」那一向报过，
        // 另 3 个（portal.core_api.base_url、ops.crosscheck_statement_timeout_ms、
        // ops.crosscheck.timeout_seconds）根本不在代码里，两向都不会报——改本读法对它们也无增益，
        // 改后是重复计数而非新发现；且 `config-reference.md` 第 6 节逐字写明
        // 「『已随裁定删除』指文档登记面，代码侧的移除随同批实现裁定」，
        // 照本判据的措辞把它们报成「不得再引入」与该逐字相抵。按文档先行，
        // 先定这批键在代码侧的去留口径，再同批改本读取规则与措辞。
        .filter_map(|l| l.split('`').nth(1))
        .map(str::to_string)
        .collect()
}

/// 文档键与代码键比对。文档键里形如 `<池>` 的段是通配段，匹配任意一段。
fn key_matches(doc_key: &str, code_key: &str) -> bool {
    let d: Vec<&str> = doc_key.split('.').collect();
    let c: Vec<&str> = code_key.split('.').collect();
    d.len() == c.len()
        && d.iter()
            .zip(c.iter())
            .all(|(a, b)| (a.starts_with('<') && a.ends_with('>')) || a == b)
}

/// 从一个 `.rs` 文件里取全部 `pub struct X { pub f: T, … }` 的字段表。
fn struct_fields(
    root: &Path,
    rel: &str,
) -> Result<BTreeMap<String, Vec<(String, String)>>, String> {
    let text = fs::read_to_string(root.join(rel)).map_err(|_| format!("读不到 {rel}"))?;
    Ok(parse_structs(&text))
}

fn parse_structs(text: &str) -> BTreeMap<String, Vec<(String, String)>> {
    let mut out = BTreeMap::new();
    let mut current: Option<(String, Vec<(String, String)>)> = None;
    for line in text.lines().map(str::trim) {
        if let Some((name, fields)) = current.as_mut() {
            if line == "}" {
                out.insert(name.clone(), std::mem::take(fields));
                current = None;
                continue;
            }
            if let Some(rest) = line.strip_prefix("pub ") {
                if let Some((f, t)) = rest.split_once(':') {
                    let ty = t.trim().trim_end_matches(',').trim().to_string();
                    fields.push((f.trim().to_string(), ty));
                }
            }
            continue;
        }
        let Some(rest) = line.strip_prefix("pub struct ") else {
            continue;
        };
        let Some(name) = rest.strip_suffix('{').map(str::trim) else {
            continue;
        };
        if !name.is_empty() {
            current = Some((name.to_string(), Vec::new()));
        }
    }
    out
}

/// 八个进程的配置根结构体，取其中类型落在分段表里的字段作为顶层段。
fn app_config_roots(
    root: &Path,
    structs: &BTreeMap<String, Vec<(String, String)>>,
) -> Vec<(String, String)> {
    let mut out: Vec<(String, String)> = Vec::new();
    let Ok(entries) = fs::read_dir(root.join(APPS)) else {
        return out;
    };
    let mut paths: Vec<PathBuf> = entries.filter_map(Result::ok).map(|e| e.path()).collect();
    paths.sort();
    for p in paths {
        let Ok(text) = fs::read_to_string(p.join("src/config.rs")) else {
            continue;
        };
        for (_, fields) in parse_structs(&text) {
            for (name, ty) in fields {
                if structs.contains_key(&ty) && !out.iter().any(|(n, t)| n == &name && t == &ty) {
                    out.push((name, ty));
                }
            }
        }
    }
    out.sort();
    out
}

/// 把一个段展开成完整键路径。`path` 承担环路防护：同一类型在一条路径上只展开一次。
fn expand(
    prefix: &str,
    ty: &str,
    structs: &BTreeMap<String, Vec<(String, String)>>,
    path: &mut Vec<String>,
    out: &mut Vec<String>,
) {
    let Some(fields) = structs.get(ty) else {
        out.push(prefix.to_string());
        return;
    };
    if path.iter().any(|t| t == ty) {
        out.push(prefix.to_string());
        return;
    }
    path.push(ty.to_string());
    for (name, field_ty) in fields {
        expand(&format!("{prefix}.{name}"), field_ty, structs, path, out);
    }
    path.pop();
}

fn duplicates(items: &[String]) -> Vec<(String, usize)> {
    let mut seen: BTreeMap<&str, usize> = BTreeMap::new();
    for i in items {
        *seen.entry(i.as_str()).or_insert(0) += 1;
    }
    seen.into_iter()
        .filter(|(_, n)| *n > 1)
        .map(|(k, n)| (k.to_string(), n))
        .collect()
}

// ---------------------------------------------------------------------------
// 二、指标名
// ---------------------------------------------------------------------------

fn check_metrics(root: &Path, problems: &mut Vec<String>, uncovered: &mut Vec<String>) -> usize {
    let doc = match fs::read_to_string(root.join(METRICS_DOC)) {
        Ok(t) => t,
        Err(_) => {
            problems.push(format!("读不到 {METRICS_DOC}"));
            return 0;
        }
    };
    let doc_names: Vec<String> = doc
        .lines()
        .map(str::trim)
        .filter(|l| l.starts_with("| `ep_"))
        .filter_map(|l| l.split('`').nth(1))
        .map(str::to_string)
        .collect();
    if doc_names.is_empty() {
        problems.push(format!("{METRICS_DOC} 第 3 节没有解析出任何指标名"));
        return 0;
    }
    for (name, n) in duplicates(&doc_names) {
        problems.push(format!(
            "{METRICS_DOC} 中指标名 {name} 出现 {n} 次；重名即失败"
        ));
    }

    match fs::read_to_string(root.join(METRICS_RS)) {
        Err(_) => uncovered.push(format!(
            "未覆盖：读不到 {METRICS_RS}，指标登记表两侧比对无被测输入"
        )),
        Ok(code) => {
            let code_names = registered_metrics(&code);
            if code_names.is_empty() {
                uncovered.push(format!("未覆盖：{METRICS_RS} 里没有解析出任何注册项"));
            } else {
                for (name, n) in duplicates(&code_names) {
                    problems.push(format!("{METRICS_RS} 中 {name} 注册了 {n} 次"));
                }
                for d in &doc_names {
                    if !code_names.contains(d) {
                        problems.push(format!("{METRICS_DOC} 登记了 {d}，代码注册表中没有"));
                    }
                }
                for c in &code_names {
                    if !doc_names.contains(c) {
                        problems.push(format!("代码注册了 {c}，{METRICS_DOC} 中没有登记"));
                    }
                }
            }
        }
    }

    doc_names.len()
}

/// `REGISTERED` 数组里的 `name: "…"` 行。
fn registered_metrics(code: &str) -> Vec<String> {
    let Some(body) = code.split("pub const REGISTERED").nth(1) else {
        return Vec::new();
    };
    let body = body.split("\n];").next().unwrap_or(body);
    body.lines()
        .map(str::trim)
        .filter(|l| l.starts_with("name: \""))
        .filter_map(|l| l.split('"').nth(1))
        .map(str::to_string)
        .collect()
}

fn files_with_ext(dir: &Path, ext: &str) -> Vec<PathBuf> {
    let mut out = Vec::new();
    let Ok(entries) = fs::read_dir(dir) else {
        return out;
    };
    let mut paths: Vec<_> = entries.filter_map(Result::ok).map(|e| e.path()).collect();
    paths.sort();
    for p in paths {
        if p.is_dir() {
            out.extend(files_with_ext(&p, ext));
        } else if p.extension().is_some_and(|e| e == ext) {
            out.push(p);
        }
    }
    out
}

// ---------------------------------------------------------------------------
// 三、单据类型码
// ---------------------------------------------------------------------------

/// `xtask configdoc --check-doc-type-codes` 的入口。
///
/// 阶段 1 只判一件事：`docs/data-dictionary.md` 的单据类型码一节存在。
/// 逐项一致且无重复这一条按退出条件 23 整条推迟；推迟不等于恒真，因此它以
/// [`Report::deferred`] 单列打印，且一旦该节登记表出现第一行就自动转为真判定。
pub fn run_doc_type_codes(root: &Path) -> Report {
    let mut problems = Vec::new();
    let mut uncovered = Vec::new();
    let mut deferred = Vec::new();

    let doc = match fs::read_to_string(root.join(DICT_DOC)) {
        Ok(t) => t,
        Err(_) => {
            return Report {
                problems: vec![format!("读不到 {DICT_DOC}")],
                uncovered,
                deferred,
                compared: 0,
            }
        }
    };
    for title in [DICT_SECTION, DICT_TABLE] {
        if !doc.contains(title) {
            problems.push(format!("{DICT_DOC} 中找不到小节标题「{title}」"));
        }
    }
    if !problems.is_empty() {
        return Report {
            problems,
            uncovered,
            deferred,
            compared: 0,
        };
    }

    let codes = dict_type_codes(&doc);
    if codes.is_empty() {
        deferred.push(format!(
            "{DICT_DOC} 的 {DICT_TABLE} 当前 0 行，逐项一致且无重复按退出条件 23 整条推迟；\
             该表出现第一行即自动生效，不以阶段号为触发谓词"
        ));
        return Report {
            problems,
            uncovered,
            deferred,
            compared: 0,
        };
    }

    for (code, n) in duplicates(&codes) {
        problems.push(format!(
            "{DICT_DOC} 中类型码 {code} 出现 {n} 次；类型码全局唯一"
        ));
    }
    let in_code = sequence_type_codes(root);
    if in_code.is_empty() {
        uncovered.push(format!(
            "未覆盖：{DICT_DOC} 已登记 {} 个类型码，但 {SEQUENCE_SRC} 下找不到任何类型码常量，比对做不了",
            codes.len()
        ));
        return Report {
            problems,
            uncovered,
            deferred,
            compared: codes.len(),
        };
    }
    for c in &codes {
        if !in_code.contains(c) {
            problems.push(format!(
                "{DICT_DOC} 登记了类型码 {c}，ep-platform-sequence 的常量表中没有"
            ));
        }
    }
    for c in &in_code {
        if !codes.contains(c) {
            problems.push(format!(
                "ep-platform-sequence 有类型码 {c}，{DICT_DOC} 中没有登记"
            ));
        }
    }
    Report {
        problems,
        uncovered,
        deferred,
        compared: codes.len(),
    }
}

/// 5.1 登记表的第一格。
fn dict_type_codes(doc: &str) -> Vec<String> {
    let Some(section) = doc.split(DICT_TABLE).nth(1) else {
        return Vec::new();
    };
    let section = section.split("\n### ").next().unwrap_or(section);
    section
        .lines()
        .map(str::trim)
        .filter(|l| l.starts_with('|'))
        .filter_map(|l| l.trim_matches('|').split('|').next())
        .map(|c| c.trim().trim_matches('`').trim().to_string())
        .filter(|c| is_type_code(c))
        .collect()
}

/// 类型码形态：2 至 6 位大写字母，如 CT、SO、SR、DC、PINV。
fn is_type_code(s: &str) -> bool {
    (2..=6).contains(&s.chars().count()) && s.chars().all(|c| c.is_ascii_uppercase())
}

/// 代码侧取 ep-platform-sequence 里合类型码形态的字符串字面量。
/// 按形态取而不是按某个常量名取：常量名由阶段 3a 定，本阶段不替它取名。
fn sequence_type_codes(root: &Path) -> Vec<String> {
    let mut out = Vec::new();
    for path in files_with_ext(&root.join(SEQUENCE_SRC), "rs") {
        let Ok(text) = fs::read_to_string(&path) else {
            continue;
        };
        for line in text.lines() {
            let mut rest = line;
            while let Some(start) = rest.find('"') {
                let after = &rest[start + 1..];
                let Some(end) = after.find('"') else { break };
                let lit = &after[..end];
                if is_type_code(lit) {
                    out.push(lit.to_string());
                }
                rest = &after[end + 1..];
            }
        }
    }
    out.sort();
    out.dedup();
    out
}

// ---------------------------------------------------------------------------
// 四、路由能力元组
// ---------------------------------------------------------------------------

/// 系统端点前缀，按阶段 1 计划端点表权限列逐字「无，仅回环」豁免。
const SYSTEM_ROUTE_PREFIX: &str = "/api/v1/system/";

/// 从一份源码文本里取出全部路由路径及其是否带能力元组。
///
/// 判定规则见模块文档第四段。以文本为被测对象，因此负样例直接喂一份手写源码即可。
fn scan_route_capabilities(text: &str) -> Vec<(String, bool)> {
    let mut hits: Vec<(usize, String)> = Vec::new();
    let needle = "\"/api/v1/";
    let mut from = 0usize;
    while let Some(rel) = text[from..].find(needle) {
        let at = from + rel;
        let lit_start = at + 1;
        match text[lit_start..].find('"') {
            None => break,
            Some(end_rel) => {
                let path = text[lit_start..lit_start + end_rel].to_string();
                hits.push((at, path));
                from = lit_start + end_rel + 1;
            }
        }
    }

    let mut out = Vec::new();
    for i in 0..hits.len() {
        let (start, ref path) = hits[i];
        if path.starts_with(SYSTEM_ROUTE_PREFIX) {
            continue;
        }
        let end = hits.get(i + 1).map_or(text.len(), |(next, _)| *next);
        let has = text[start..end].contains("CapabilityDomain::");
        out.push((path.clone(), has));
    }
    out
}

/// 递归收集 `apps/` 下的 `.rs` 文件。
fn rs_files(dir: &Path, out: &mut Vec<PathBuf>) {
    let Ok(entries) = fs::read_dir(dir) else {
        return;
    };
    for e in entries.flatten() {
        let p = e.path();
        if p.is_dir() {
            rs_files(&p, out);
        } else if p.extension().is_some_and(|x| x == "rs") {
            out.push(p);
        }
    }
}

fn check_route_capabilities(
    root: &Path,
    problems: &mut Vec<String>,
    uncovered: &mut Vec<String>,
) -> usize {
    let mut files = Vec::new();
    rs_files(&root.join(APPS), &mut files);
    files.sort();

    let mut compared = 0usize;
    let mut scanned_files = 0usize;
    for f in files {
        let Ok(text) = fs::read_to_string(&f) else {
            continue;
        };
        // 只扫真的注册路由的文件；守卫里的路径比较不构成注册。
        if !text.contains(".route(") {
            continue;
        }
        scanned_files += 1;
        let rel = f.strip_prefix(root).unwrap_or(&f).display().to_string();
        for (path, has) in scan_route_capabilities(&text) {
            compared += 1;
            if !has {
                problems.push(format!(
                    "{rel} 的路由 {path} 解析不到 (CapabilityDomain, ActionClass) 元组；\
                     按总览 A-20 与阶段 4 退出条件 17，缺失即构建失败"
                ));
            }
        }
    }

    if scanned_files == 0 {
        uncovered.push(format!(
            "未覆盖：{APPS} 下没有含 .route( 的源码文件，路由能力元组判据无被测输入"
        ));
    } else if compared == 0 {
        uncovered.push(format!(
            "未覆盖：扫了 {scanned_files} 个含路由注册的文件，但没有解析出任何非系统 /api/v1/ 路由"
        ));
    }
    compared
}

#[cfg(test)]
mod negative_samples {
    use super::*;

    fn repo_root() -> PathBuf {
        Path::new(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .expect("xtask 在工作区根之下")
            .to_path_buf()
    }

    /// 判据本体：带元组的条目判过，缺元组的条目判不过。
    /// 以文本为被测对象，故负样例直接喂手写源码。
    #[test]
    fn a_route_without_a_capability_tuple_is_caught() {
        let good = r#"
            .route(
                "/api/v1/platform/things",
                get(list),
                (CapabilityDomain::PlatformAdminLowcodeOps, ActionClass::Read),
            )
        "#;
        assert_eq!(
            scan_route_capabilities(good),
            vec![("/api/v1/platform/things".to_string(), true)]
        );

        let bad = r#"
            .route("/api/v1/platform/things", get(list))
        "#;
        assert_eq!(
            scan_route_capabilities(bad),
            vec![("/api/v1/platform/things".to_string(), false)]
        );
    }

    /// 系统端点按阶段 1 计划端点表权限列「无，仅回环」豁免，不进被测面。
    #[test]
    fn system_routes_are_exempt() {
        let text = r#".route("/api/v1/system/echo", post(echo))"#;
        assert!(scan_route_capabilities(text).is_empty());
    }

    /// 相邻两条之间各判各的：前一条有元组不能替后一条顶过。
    #[test]
    fn a_tuple_does_not_cover_the_next_route() {
        let text = r#"
            ("/api/v1/platform/a", get(a), (CapabilityDomain::MdmMasterData, ActionClass::Read)),
            ("/api/v1/platform/b", get(b)),
        "#;
        assert_eq!(
            scan_route_capabilities(text),
            vec![
                ("/api/v1/platform/a".to_string(), true),
                ("/api/v1/platform/b".to_string(), false),
            ]
        );
    }

    /// 仓库现状：配置键与指标名两侧齐备，本子命令应全绿。
    #[test]
    fn the_repository_passes() {
        let r = run(&repo_root());
        assert!(r.problems.is_empty(), "两侧应逐项一致：{:#?}", r.problems);
        assert!(r.uncovered.is_empty(), "{:#?}", r.uncovered);
        assert_eq!(r.outcome(), Outcome::Clean);
        assert!(r.compared > 40, "比对条目数看着不对：{}", r.compared);
    }

    /// 负样例断言「未覆盖不得判通过」：输入全缺时不得是 Clean。
    #[test]
    fn negative_missing_inputs_are_not_clean() {
        let r = run(Path::new("/nonexistent-root-for-configdoc"));
        assert_ne!(r.outcome(), Outcome::Clean);
    }

    /// 负样例断言通配段这条规则本身：`<池>` 只匹配一段，不匹配跨段。
    #[test]
    fn negative_placeholder_matches_exactly_one_segment() {
        assert!(key_matches(
            "db.timeout.<池>.lock_ms",
            "db.timeout.rw.lock_ms"
        ));
        assert!(!key_matches(
            "db.timeout.<池>.lock_ms",
            "db.timeout.lock_ms"
        ));
        assert!(!key_matches(
            "db.timeout.<池>.lock_ms",
            "db.timeout.rw.x.lock_ms"
        ));
        assert!(!key_matches("http.max_body_bytes", "http.max_body_byte"));
    }

    /// 负样例断言键展开这条规则本身：漏一个字段即两侧不再相等。
    #[test]
    fn negative_a_dropped_field_breaks_the_key_set() {
        let structs = parse_structs(
            "pub struct DbCfg {\npub host: String,\npub pool: DbPoolCfg,\n}\npub struct DbPoolCfg {\npub rw_max: u16,\n}\n",
        );
        let mut keys = Vec::new();
        expand("db", "DbCfg", &structs, &mut Vec::new(), &mut keys);
        assert_eq!(keys, vec!["db.host", "db.pool.rw_max"]);

        let structs = parse_structs("pub struct DbCfg {\npub host: String,\n}\n");
        let mut keys = Vec::new();
        expand("db", "DbCfg", &structs, &mut Vec::new(), &mut keys);
        assert_eq!(keys, vec!["db.host"], "少一个字段就必须少一个键");
    }

    /// 负样例断言文档取键这条规则本身：表头与非键单元格不得被当成键。
    #[test]
    fn negative_doc_key_extraction_skips_headers() {
        assert_eq!(
            doc_config_keys("| 键 | 类型 |\n| http.bind_addr | string |"),
            vec!["http.bind_addr"]
        );
        assert!(doc_config_keys("| core-server | 8080 |").is_empty());
    }

    /// 负样例断言指标名比对这条规则本身：两侧各改一处，两个方向都要报。
    #[test]
    fn negative_metric_roster_is_compared_both_ways() {
        assert_eq!(
            registered_metrics("pub const REGISTERED: [MetricDef; 1] = [\n MetricDef {\n name: \"ep_build_info\",\n },\n];"),
            vec!["ep_build_info"]
        );
        assert!(registered_metrics("没有 REGISTERED 数组").is_empty());
        let dup = duplicates(&["a".to_string(), "a".to_string(), "b".to_string()]);
        assert_eq!(dup, vec![("a".to_string(), 2)]);
    }

    /// 单据类型码：登记表已有 43 行，按本文件 `run_doc_type_codes` 自陈的谓词
    /// 「该表出现第一行即自动生效，不以阶段号为触发谓词」，比对已经生效。
    /// 代码侧常量表尚未建立，故现行真值是「判定未做出」而不是「推迟」，更不是通过。
    #[test]
    fn doc_type_codes_comparison_is_active_and_uncovered() {
        let r = run_doc_type_codes(&repo_root());
        assert!(r.problems.is_empty(), "{:#?}", r.problems);
        assert!(r.deferred.is_empty(), "登记表已有行，不得再落进推迟段");
        assert_eq!(r.uncovered.len(), 1, "{:#?}", r.uncovered);
        assert!(
            r.uncovered[0].contains("比对做不了"),
            "未覆盖必须写出取不到的是什么"
        );
        assert_eq!(
            r.outcome(),
            Outcome::Uncovered,
            "未覆盖不得判 Clean：常量表建起来之前这一段一律不算通过"
        );
    }

    /// 推迟分支不因上面那条改断言而失去覆盖：在临时根目录里放一份登记表为空的
    /// 数据字典，`run_doc_type_codes` 必须真的走进推迟分支——只断言取行结果为空
    /// 是不够的，那只证明了触发条件成立，没证明分支被走到。
    #[test]
    fn deferred_branch_is_still_reachable_on_an_empty_table() {
        let root = std::env::temp_dir().join(format!(
            "ep-configdoc-deferred-branch-{}",
            std::process::id()
        ));
        fs::create_dir_all(root.join("docs")).expect("建临时 docs 目录");
        fs::write(
            root.join(DICT_DOC),
            format!("{DICT_SECTION}\n\n{DICT_TABLE}\n\n（本合成样例的登记表为空）\n"),
        )
        .expect("写合成数据字典");

        let r = run_doc_type_codes(&root);
        assert!(r.problems.is_empty(), "{:#?}", r.problems);
        assert_eq!(r.deferred.len(), 1, "登记表为空时必须落进推迟段");
        assert!(
            r.deferred[0].contains("出现第一行"),
            "推迟项必须写出可观测的触发谓词：{}",
            r.deferred[0]
        );
        assert_eq!(r.compared, 0);

        fs::remove_dir_all(&root).ok();
    }

    /// 负样例断言该节存在这条规则本身：标题被改掉即报。
    #[test]
    fn negative_a_missing_doc_type_section_fails() {
        let doc = fs::read_to_string(repo_root().join(DICT_DOC)).expect("读得到数据字典");
        assert!(doc.contains(DICT_SECTION) && doc.contains(DICT_TABLE));
        // 登记表已装入 43 个值（data-dictionary.md 第 5.1 节），比对因此已生效。
        let codes = dict_type_codes(&doc);
        assert_eq!(codes.len(), 43, "5.1 登记表应为 43 行：{codes:#?}");
        assert!(duplicates(&codes).is_empty(), "类型码全局唯一");
        // 再加一行时取行判定仍生效，新行必须被取到。
        let with_row = doc.replace(
            DICT_TABLE,
            &format!("{DICT_TABLE}\n\n| 类型码 | 名称 |\n|---|---|\n| ZZZ | 合成样例 |\n"),
        );
        let after = dict_type_codes(&with_row);
        assert_eq!(after.len(), 44, "新增行必须被取到");
        assert!(after.contains(&"ZZZ".to_string()));
        // 标题被改掉时该节存在这条规则本身要报——这是本测试名字所指的那条规则。
        // 只断言 `String::replace` 生效是空洞的，要真跑 run_doc_type_codes 看它报不报。
        let root = std::env::temp_dir().join(format!(
            "ep-configdoc-missing-section-{}",
            std::process::id()
        ));
        fs::create_dir_all(root.join("docs")).expect("建临时 docs 目录");
        fs::write(
            root.join(DICT_DOC),
            doc.replace(DICT_SECTION, "## 已改名的小节"),
        )
        .expect("写合成数据字典");
        let r = run_doc_type_codes(&root);
        assert!(
            r.problems.iter().any(|p| p.contains(DICT_SECTION)),
            "小节标题被改掉必须报出来：{:#?}",
            r.problems
        );
        fs::remove_dir_all(&root).ok();
        assert!(is_type_code("PINV") && !is_type_code("So") && !is_type_code("S"));
    }
}
