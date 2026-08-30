//! `xtask sqlcheck` —— SQL 静态检查。
//!
//! 阶段 1 退出条件 11 点名十项规则，本模块在其上另加基线与裁定各自点名的三项：
//! 迁移版本号全局唯一且严格递增（裁定 00c 第五条）、引导脚本口令字面量禁令
//! （阶段 1 计划第 4.1 节 SQL-020）、`ci_probe` 不进生产迁移目录（第 4.4 节 SQL-030）。
//!
//! 另加第十四项 SQL-031：仅追加登记与守卫挂接的**次序**一致性，见下。
//!
//! **空扫描不判通过。** 被测目录为空时，按技术基线第 12 节
//! 通则第六条，被测输入缺席既不得表达为通过也不得表达为违反，这类规则单列进
//! [`Report::uncovered`] 并由调用方以专用退出码结束。重新生效的触发谓词是
//! 「对应目录下出现第一个 `.sql` 文件」，由本工具自身可观测，不写成阶段号。
//!
//! 解析一律文本级：为待交付的 SQL 引一个解析器等于给门禁工具加一条本阶段无被测对象的
//! 依赖边。已知边界写在 [`strip_comments`] 与 [`statements`] 的注释里，不藏。

use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};

/// 规则清单。左列是规则号，右列是判据一句话。
/// SQL-001 至 SQL-011 判迁移文件，SQL-020 与 SQL-021 判引导目录，SQL-030 判迁移目录整体。
pub const RULES: [(&str, &str); 15] = [
    (
        "SQL-001",
        "业务 schema 上禁止 DELETE 语句，只放行 platform_msg 与 platform_ops",
    ),
    ("SQL-002", "禁止 varchar(n)，一律 text 加 CHECK 长度约束"),
    (
        "SQL-003",
        "禁止 PostgreSQL enum 类型，一律 text 加 CHECK 约束",
    ),
    (
        "SQL-004",
        "禁止 current_date，服务器自然日取 (now() AT TIME ZONE 'Asia/Shanghai')::date",
    ),
    (
        "SQL-005",
        "跨 schema 外键必须是 (legal_entity_id, <ref>_id) 复合形式并 ON DELETE RESTRICT",
    ),
    ("SQL-006", "禁止 ON DELETE CASCADE"),
    ("SQL-007", "迁移文件必须以 -- rollback: 段开头"),
    ("SQL-008", "公共列齐备且按基线第 4 节的顺序排在列表最前"),
    (
        "SQL-009",
        "命名规范：迁移文件名、约束与索引前缀、列名后缀与类型",
    ),
    (
        "SQL-010",
        "迁移单一职责：一个文件创建的对象只属一个 schema，且等于所在目录名",
    ),
    ("SQL-011", "迁移版本号全局唯一且严格递增"),
    (
        "SQL-030",
        "ci_probe 不进生产迁移目录",
    ),
    ("SQL-020", "引导脚本中不得出现口令字面量"),
    ("SQL-021", "引导目录中不得出现约定之外的文件名"),
    (
        "SQL-031",
        "仅追加登记行必有 attach_table_guards 调用，且该调用不得排在登记之前",
    ),
];

/// 迁移目录相对仓库根的位置。三处目录名是阶段 1 计划第 4.1、4.2 节与裁定登记的固定取值。
const MIGRATIONS: &str = "db/migrations";
const BOOTSTRAP: &str = "db/bootstrap";

/// 引导目录的文件名白名单，出处是阶段 1 计划第 4.1 节，逐字五项加一份 README。
const BOOTSTRAP_FILES: [&str; 6] = [
    "00_database.sql",
    "01_roles.sql",
    "02_cluster_params.sql",
    "03_role_defaults.sql",
    "04_pg_hba.fragment",
    "README.md",
];

/// 公共列与其顺序，出处是技术基线第 4 节。
const COMMON_COLUMNS: [&str; 9] = [
    "id",
    "legal_entity_id",
    "security_level",
    "data_scope_tags",
    "row_version",
    "created_at",
    "created_by",
    "updated_at",
    "updated_by",
];

/// 仅追加表不带的三列。三者同进同出：缺一即不是合法的仅追加表形态。
const APPEND_ONLY_OMITTED: [&str; 3] = ["row_version", "updated_at", "updated_by"];

/// 允许执行 DELETE 的两个 schema，出处是技术基线第 3.6 节末条。
const DELETE_ALLOWED_SCHEMAS: [&str; 2] = ["platform_msg", "platform_ops"];

/// 工具自带元数据表，不套用公共列与 text 约定。白名单只有这一项，出处是阶段 1 计划第 4.3 节。
const WHITELIST_TABLE: &str = "platform_core.schema_history";

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Violation {
    pub rule: &'static str,
    pub file: String,
    pub line: usize,
    pub detail: String,
}

#[derive(Debug)]
pub struct Report {
    /// 违反明细，已格式化为可直接打印的行。
    pub problems: Vec<String>,
    /// 未覆盖的规则：判据已实现但本次运行没有被测输入。不得读作通过。
    pub uncovered: Vec<String>,
    /// 本次真正判定过的规则号。
    pub checked: Vec<&'static str>,
    pub scanned_files: usize,
}

/// 退出码三态，与 archcheck 同形：全绿 0、有违反 1、有未覆盖 3。
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Outcome {
    Clean,
    Uncovered,
    Violated,
}

impl Report {
    pub fn outcome(&self) -> Outcome {
        match (self.problems.is_empty(), self.uncovered.is_empty()) {
            (false, _) => Outcome::Violated,
            (true, false) => Outcome::Uncovered,
            (true, true) => Outcome::Clean,
        }
    }
}

pub fn run(root: &Path) -> Report {
    let (mut violations, mut uncovered): (Vec<Violation>, Vec<String>) = (Vec::new(), Vec::new());
    let mut checked: Vec<&'static str> = Vec::new();
    let migrations = sql_files(&root.join(MIGRATIONS));
    if migrations.is_empty() {
        // RULES 的前 12 条（SQL-001..SQL-011 与 SQL-030）与末条 SQL-031 判迁移文件；
        // 没有迁移文件就是这 13 条一律未覆盖。
        //
        // F-81 更正三处：其一，SQL-030 原先根本不在 RULES 里（只作字面量出现在
        // `checked.push` 与 `line_rule` 中），因此它**永远不会**出现在未覆盖名单上——
        // 迁移目录为空时该规则从三态报告里整条蒸发，读报告的人看不出它没跑。
        // 其二，原式 `chain([&RULES[12], &RULES[13]])` 取的是 SQL-021 与 SQL-031，
        // 而 SQL-021 判的是**引导目录**、下面 :182 另有分支处置，于是同一条规则
        // 会同时出现在「已判定」与「判定未做出」两栏——两栏按 main.rs 的纪律
        // 「互不合并、任何一件不得被读成另一件」。其三，索引写死在 chain 里，
        // RULES 增删条目即静默指错；改为按取值语义取，不按下标。
        for (id, what) in RULES
            .iter()
            .take(12)
            .chain(RULES.iter().filter(|(id, _)| *id == "SQL-031"))
        {
            uncovered.push(format!(
                "{id} 未覆盖：{MIGRATIONS} 下没有任何 .sql 文件，判据「{what}」本次无被测输入"
            ));
        }
    } else {
        // take(12) 已含 SQL-030，不再另推字面量（原写法是 SQL-030 缺席 RULES 的根源）。
        checked.extend(RULES.iter().take(12).map(|(id, _)| *id));
        let mut versions: BTreeMap<u64, Vec<String>> = BTreeMap::new();
        for path in &migrations {
            let rel = relative(root, path);
            match fs::read_to_string(path) {
                Ok(text) => {
                    violations.extend(scan_migration(&rel, &text));
                    if let Some(v) = file_version(&rel) {
                        versions.entry(v).or_default().push(rel.clone());
                    }
                }
                // 读不出来的文件不得当作通过：判不了就是违反本条的举证义务。
                Err(e) => violations.push(unreadable("SQL-007", rel, &e)),
            }
        }
        violations.extend(check_versions(&versions));

        let (guard_violations, registered) = check_append_only_guards(root, &migrations);
        if registered == 0 {
            uncovered.push(format!(
                "SQL-031 未覆盖：{MIGRATIONS} 下解析不到任何 append_only_registry 登记行，\
                 判据本次无被测输入"
            ));
        } else {
            checked.push("SQL-031");
            violations.extend(guard_violations);
        }
    }

    let bootstrap_dir = root.join(BOOTSTRAP);
    if !bootstrap_dir.is_dir() {
        uncovered.push(format!(
            "SQL-020 与 SQL-021 未覆盖：目录 {BOOTSTRAP} 不存在"
        ));
    } else {
        checked.push("SQL-021");
        violations.extend(check_bootstrap_names(&bootstrap_dir));
        let files = sql_files(&bootstrap_dir);
        if files.is_empty() {
            uncovered.push(format!("SQL-020 未覆盖：{BOOTSTRAP} 下没有任何 .sql 文件"));
        } else {
            checked.push("SQL-020");
            for path in &files {
                let rel = relative(root, path);
                match fs::read_to_string(path) {
                    Ok(text) => violations.extend(scan_bootstrap(&rel, &text)),
                    Err(e) => violations.push(unreadable("SQL-020", rel, &e)),
                }
            }
        }
    }

    Report {
        problems: violations.iter().map(format_violation).collect(),
        uncovered,
        checked,
        scanned_files: migrations.len(),
    }
}

fn unreadable(rule: &'static str, file: String, e: &std::io::Error) -> Violation {
    Violation {
        rule,
        file,
        line: 0,
        detail: format!("读不到该文件：{e}"),
    }
}

fn format_violation(v: &Violation) -> String {
    let at = if v.line == 0 {
        String::new()
    } else {
        format!(":{}", v.line)
    };
    format!("[{}] {}{} — {}", v.rule, v.file, at, v.detail)
}

fn relative(root: &Path, path: &Path) -> String {
    path.strip_prefix(root)
        .unwrap_or(path)
        .to_string_lossy()
        .replace('\\', "/")
}

/// 递归收集 `.sql` 文件。目录不存在时返回空集，由调用方判「未覆盖」。
fn sql_files(dir: &Path) -> Vec<PathBuf> {
    let mut out = Vec::new();
    let Ok(entries) = fs::read_dir(dir) else {
        return out;
    };
    let mut entries: Vec<_> = entries.filter_map(Result::ok).map(|e| e.path()).collect();
    entries.sort();
    for path in entries {
        if path.is_dir() {
            out.extend(sql_files(&path));
        } else if path.extension().is_some_and(|e| e == "sql") {
            out.push(path);
        }
    }
    out
}

// ---------------------------------------------------------------------------
// 文本层
// ---------------------------------------------------------------------------

/// 去注释，保留行号。已知边界：不识别字符串字面量内的 `--` 与 `/*`。
/// 代价是那种字面量里的内容会被当成注释丢掉，判据因此偏松而不偏紧——
/// 静态门禁宁可漏报也不能因为解析不准而误伤一条合法迁移。
fn strip_comments(text: &str) -> Vec<(usize, String)> {
    let mut out = Vec::new();
    let mut in_block = false;
    for (i, raw) in text.lines().enumerate() {
        let chars: Vec<char> = raw.chars().collect();
        let (mut code, mut j) = (String::new(), 0usize);
        while j < chars.len() {
            let pair = (chars[j], chars.get(j + 1).copied());
            if in_block {
                in_block = pair != ('*', Some('/'));
                j += if in_block { 1 } else { 2 };
            } else if pair == ('-', Some('-')) {
                break;
            } else if pair == ('/', Some('*')) {
                in_block = true;
                j += 2;
            } else {
                code.push(chars[j]);
                j += 1;
            }
        }
        // 连续空白归一并转小写：后面的规则一律按单空格形态匹配，
        // 免得同一条语句因为缩进不同而漏判。
        out.push((
            i + 1,
            code.split_whitespace()
                .collect::<Vec<_>>()
                .join(" ")
                .to_lowercase(),
        ));
    }
    out
}

struct Stmt {
    line: usize,
    /// 已去注释、已转小写、连续空白归一为单个空格。
    norm: String,
}

/// 按分号切句。单引号内的分号不切；已知边界同 [`strip_comments`]。
fn statements(lines: &[(usize, String)]) -> Vec<Stmt> {
    let (mut out, mut buf, mut start, mut quoted) = (Vec::new(), String::new(), 0usize, false);
    for (no, line) in lines {
        for c in line.chars() {
            if c == '\'' {
                quoted = !quoted;
            }
            if c == ';' && !quoted {
                push_stmt(&mut out, start, &buf);
                buf.clear();
                start = 0;
                continue;
            }
            if start == 0 && !c.is_whitespace() {
                start = *no;
            }
            buf.push(c);
        }
        buf.push(' ');
    }
    push_stmt(&mut out, start, &buf);
    out
}

fn push_stmt(out: &mut Vec<Stmt>, line: usize, raw: &str) {
    let norm = raw.split_whitespace().collect::<Vec<_>>().join(" ");
    if !norm.is_empty() && line > 0 {
        out.push(Stmt { line, norm });
    }
}

/// 取顶层括号内的逗号分段，用于拆列定义。深度不平衡时返回空集。
fn top_level_parts(body: &str) -> Vec<String> {
    let (mut parts, mut cur, mut depth, mut quoted) = (Vec::new(), String::new(), 0i32, false);
    for c in body.chars() {
        match c {
            '\'' => quoted = !quoted,
            '(' if !quoted => depth += 1,
            ')' if !quoted => depth -= 1,
            ',' if depth == 0 && !quoted => {
                parts.push(cur.trim().to_string());
                cur.clear();
                continue;
            }
            _ => {}
        }
        cur.push(c);
    }
    if depth != 0 {
        return Vec::new();
    }
    if !cur.trim().is_empty() {
        parts.push(cur.trim().to_string());
    }
    parts
}

/// 取 `create table [if not exists] <name> ( <body> )` 的表名与括号体。
fn create_table_parts(norm: &str) -> Option<(String, String)> {
    let rest = norm.strip_prefix("create table ")?;
    let rest = rest.strip_prefix("if not exists ").unwrap_or(rest);
    let open = rest.find('(')?;
    let close = rest.rfind(')')?;
    if close <= open {
        return None;
    }
    let name = rest[..open].trim().to_string();
    Some((name, rest[open + 1..close].to_string()))
}

/// 取限定名的 schema 段。无点号时返回 None——不给未限定名猜一个 schema。
fn schema_of(qualified: &str) -> Option<&str> {
    qualified.split_once('.').map(|(s, _)| s)
}

fn table_of(qualified: &str) -> &str {
    qualified.rsplit('.').next().unwrap_or(qualified)
}

// ---------------------------------------------------------------------------
// 逐规则判定
// ---------------------------------------------------------------------------

/// 一个迁移文件上的全部规则。`rel` 形如 `db/migrations/<schema>/V…__x.sql`。
pub fn scan_migration(rel: &str, text: &str) -> Vec<Violation> {
    let mut v = Vec::new();
    let lines = strip_comments(text);
    let stmts = statements(&lines);
    let dir_schema = rel.split('/').rev().nth(1).unwrap_or("").to_string();
    let whitelisted = stmts
        .iter()
        .any(|s| s.norm.starts_with("create table") && s.norm.contains(WHITELIST_TABLE));

    v.extend(rule_delete(rel, &lines));
    v.extend(rule_enum(rel, &stmts));
    v.extend(rule_current_date(rel, &lines));
    v.extend(rule_cross_schema_fk(rel, &dir_schema, &stmts));
    v.extend(rule_cascade(rel, &lines));
    v.extend(rule_rollback_header(rel, text));
    // 工具自带元数据表不套用 text 约定与公共列，白名单只有它一项。
    if !whitelisted {
        v.extend(rule_varchar(rel, &lines));
        v.extend(rule_common_columns(rel, &stmts));
    }
    v.extend(rule_naming(rel, &stmts));
    v.extend(rule_single_responsibility(rel, &dir_schema, &stmts));
    v.extend(rule_ci_probe(rel, &lines));
    v
}

fn at(rule: &'static str, rel: &str, line: usize, detail: String) -> Violation {
    Violation {
        rule,
        file: rel.to_string(),
        line,
        detail,
    }
}

/// 逐行谓词型规则的公共形态：命中一行即一条违反，文案固定。
fn line_rule(
    rel: &str,
    lines: &[(usize, String)],
    rule: &'static str,
    hit: fn(&str) -> bool,
    detail: &str,
) -> Vec<Violation> {
    lines
        .iter()
        .filter(|(_, l)| hit(l))
        .map(|(no, _)| at(rule, rel, *no, detail.into()))
        .collect()
}

fn rule_delete(rel: &str, lines: &[(usize, String)]) -> Vec<Violation> {
    let mut v = Vec::new();
    for (no, line) in lines {
        let Some(idx) = line.find("delete from ") else {
            continue;
        };
        let word = line[idx + "delete from ".len()..]
            .split_whitespace()
            .next()
            .unwrap_or("");
        let target = word.trim_matches(|c: char| !c.is_alphanumeric() && c != '_' && c != '.');
        if schema_of(target).is_some_and(|s| DELETE_ALLOWED_SCHEMAS.contains(&s)) {
            continue;
        }
        let allowed = DELETE_ALLOWED_SCHEMAS.join(" 与 ");
        v.push(at(
            "SQL-001",
            rel,
            *no,
            format!(
                "DELETE 目标 {target} 不在 {allowed} 内；业务数据不做物理删除，走 status 状态机"
            ),
        ));
    }
    v
}

fn rule_varchar(rel: &str, lines: &[(usize, String)]) -> Vec<Violation> {
    let why = "出现 varchar；列类型一律 text 加 CHECK 长度约束";
    line_rule(rel, lines, "SQL-002", |l| l.contains("varchar"), why)
}

fn rule_enum(rel: &str, stmts: &[Stmt]) -> Vec<Violation> {
    let why = "出现 PostgreSQL enum 类型；一律 text 加 CHECK 约束";
    stmts
        .iter()
        .filter(|s| s.norm.starts_with("create type ") && s.norm.contains(" as enum"))
        .map(|s| at("SQL-003", rel, s.line, why.into()))
        .collect()
}

fn rule_current_date(rel: &str, lines: &[(usize, String)]) -> Vec<Violation> {
    let why = "出现 current_date；服务器自然日一律取 (now() AT TIME ZONE 'Asia/Shanghai')::date";
    line_rule(rel, lines, "SQL-004", |l| l.contains("current_date"), why)
}

/// 跨 schema 外键。判据取技术基线第 3.3 节：跨 schema 引用一律建复合外键
/// `(legal_entity_id, <ref>_id)` 指向 `(legal_entity_id, id)` 并 `ON DELETE RESTRICT`。
///
/// 阶段 1 计划退出条件 11 把本条写成「跨 schema 外键禁令」，与基线第 3.3 节及
/// 00-overview 的 R14 直接冲突——那两处要求跨 schema 一律建真实外键。本模块按基线取值，
/// 冲突已作为待裁定项上报；若裁定改判为字面禁令，本函数改一处判断即可。
fn rule_cross_schema_fk(rel: &str, dir_schema: &str, stmts: &[Stmt]) -> Vec<Violation> {
    let mut v = Vec::new();
    for s in stmts {
        for (i, _) in s.norm.match_indices("references ") {
            let target = s.norm[i + "references ".len()..]
                .split(|c: char| c.is_whitespace() || c == '(')
                .next()
                .unwrap_or("");
            let Some(target_schema) = schema_of(target) else {
                continue;
            };
            if target_schema == dir_schema {
                continue;
            }
            let composite = s.norm.contains("foreign key (legal_entity_id,")
                && s.norm[i..].contains("(legal_entity_id, id)");
            if !composite {
                v.push(at("SQL-005", rel, s.line, format!("跨 schema 外键指向 {target}，但不是 (legal_entity_id, <ref>_id) 指向 (legal_entity_id, id) 的复合形式")));
            } else if !s.norm.contains("on delete restrict") {
                v.push(at(
                    "SQL-005",
                    rel,
                    s.line,
                    format!("跨 schema 外键指向 {target}，缺 ON DELETE RESTRICT"),
                ));
            }
        }
    }
    v
}

fn rule_cascade(rel: &str, lines: &[(usize, String)]) -> Vec<Violation> {
    let why = "出现 ON DELETE CASCADE；外键一律 ON DELETE RESTRICT";
    line_rule(
        rel,
        lines,
        "SQL-006",
        |l| l.contains("on delete cascade"),
        why,
    )
}

/// 回退说明段。判据是文件的第一行非空内容即 `-- rollback:`，不是「文件里某处有」。
fn rule_rollback_header(rel: &str, text: &str) -> Vec<Violation> {
    let first = text
        .lines()
        .map(str::trim)
        .find(|l| !l.is_empty())
        .unwrap_or("");
    if first.to_lowercase().starts_with("-- rollback:") {
        return Vec::new();
    }
    vec![at(
        "SQL-007",
        rel,
        1,
        format!("首行非空内容是「{first}」，迁移文件必须以 -- rollback: 段开头"),
    )]
}

fn rule_common_columns(rel: &str, stmts: &[Stmt]) -> Vec<Violation> {
    let mut v = Vec::new();
    for s in stmts {
        let Some((name, body)) = create_table_parts(&s.norm) else {
            continue;
        };
        let parts = top_level_parts(&body);
        if parts.is_empty() {
            v.push(at(
                "SQL-008",
                rel,
                s.line,
                format!("{name} 的列定义解析不出来，判不了公共列"),
            ));
            continue;
        }
        let columns: Vec<&str> = parts
            .iter()
            .filter(|p| !starts_with_constraint(p))
            .filter_map(|p| p.split_whitespace().next())
            .collect();
        let has = |c: &&str| columns.contains(c);
        let present: Vec<&str> = COMMON_COLUMNS.iter().copied().filter(has).collect();
        let missing: Vec<&str> = COMMON_COLUMNS
            .iter()
            .copied()
            .filter(|c| !present.contains(c) && !APPEND_ONLY_OMITTED.contains(c))
            .collect();
        if !missing.is_empty() {
            v.push(at(
                "SQL-008",
                rel,
                s.line,
                format!("{name} 缺公共列 {}", missing.join("、")),
            ));
        }
        let omitted: Vec<&str> = APPEND_ONLY_OMITTED
            .iter()
            .copied()
            .filter(|c| !present.contains(c))
            .collect();
        if !omitted.is_empty() && omitted.len() != APPEND_ONLY_OMITTED.len() {
            let miss = omitted.join("、");
            v.push(at(
                "SQL-008",
                rel,
                s.line,
                format!("{name} 缺 {miss}；仅追加表必须三列同缺，不得只缺其一"),
            ));
        }
        // 公共列必须占据列表最前且保持基线第 4 节的相对顺序。
        let head: Vec<&str> = columns.iter().take(present.len()).copied().collect();
        if head.len() == present.len() && head != present {
            v.push(at(
                "SQL-008",
                rel,
                s.line,
                format!(
                    "{name} 的公共列顺序或位置不符。期望前 {} 列依次为 {}，实际为 {}",
                    present.len(),
                    present.join("、"),
                    head.join("、")
                ),
            ));
        }
    }
    v
}

fn starts_with_constraint(part: &str) -> bool {
    const KEYWORDS: [&str; 8] = [
        "constraint",
        "primary",
        "unique",
        "foreign",
        "check",
        "exclude",
        "like",
        "partition",
    ];
    KEYWORDS.iter().any(|k| part.starts_with(k))
}

/// 命名规范。三类判据：迁移文件名、约束与索引名的前缀、列名后缀与类型的对应。
fn rule_naming(rel: &str, stmts: &[Stmt]) -> Vec<Violation> {
    let mut v = Vec::new();
    let file = rel.rsplit('/').next().unwrap_or(rel);
    if file_version(rel).is_none() {
        v.push(at(
            "SQL-009",
            rel,
            0,
            format!("迁移文件名 {file} 不合 V<14 位时间戳>__<名字>.sql"),
        ));
    }

    for s in stmts {
        if let Some((name, body)) = create_table_parts(&s.norm) {
            let table = table_of(&name);
            for part in top_level_parts(&body) {
                if let Some(rest) = part.strip_prefix("constraint ") {
                    let cname = rest.split_whitespace().next().unwrap_or("");
                    if let Some(prefix) = constraint_prefix(rest) {
                        if !cname.starts_with(&format!("{prefix}{table}")) {
                            v.push(at(
                                "SQL-009",
                                rel,
                                s.line,
                                format!("约束名 {cname} 不合 {prefix}{table}… 的命名"),
                            ));
                        }
                    }
                } else if !starts_with_constraint(&part) {
                    v.extend(column_naming(rel, s.line, &part));
                }
            }
        }
        v.extend(object_naming(rel, s));
    }
    v
}

fn constraint_prefix(rest: &str) -> Option<&'static str> {
    const MAP: [(&str, &str); 4] = [
        ("primary key", "pk_"),
        ("unique", "ux_"),
        ("foreign key", "fk_"),
        ("check", "ck_"),
    ];
    MAP.iter()
        .find(|(kw, _)| rest.contains(kw))
        .map(|(_, p)| *p)
}

fn column_naming(rel: &str, line: usize, part: &str) -> Vec<Violation> {
    let mut v = Vec::new();
    let mut it = part.split_whitespace();
    let (Some(name), Some(ty)) = (it.next(), it.next()) else {
        return v;
    };
    let want = if name.ends_with("_at") {
        Some(("timestamptz", "_at"))
    } else if name.ends_with("_date") || name.ends_with("_on") {
        Some(("date", "_date 或 _on"))
    } else if name.starts_with("is_") || name.starts_with("has_") {
        Some(("boolean", "is_ 或 has_"))
    } else {
        None
    };
    if let Some((expect, why)) = want {
        if !ty.starts_with(expect) {
            v.push(at(
                "SQL-009",
                rel,
                line,
                format!("列 {name} 以 {why} 命名，类型必须是 {expect}，实际是 {ty}"),
            ));
        }
    }
    v
}

/// `create [unique] index`、`create policy`、`create sequence` 三类对象名的前缀。
fn object_naming(rel: &str, s: &Stmt) -> Vec<Violation> {
    let mut v = Vec::new();
    let specs: [(&str, &str); 4] = [
        ("create unique index ", "ux_"),
        ("create index ", "ix_"),
        ("create policy ", "rls_"),
        ("create sequence ", "sq_"),
    ];
    for (head, prefix) in specs {
        let Some(rest) = s.norm.strip_prefix(head) else {
            continue;
        };
        let rest = rest.strip_prefix("concurrently ").unwrap_or(rest);
        let rest = rest.strip_prefix("if not exists ").unwrap_or(rest);
        let name = rest.split_whitespace().next().unwrap_or("");
        if !name.starts_with(prefix) {
            v.push(at(
                "SQL-009",
                rel,
                s.line,
                format!("对象名 {name} 必须以 {prefix} 开头"),
            ));
        }
        break;
    }
    v
}

/// 迁移单一职责。判据取 00-overview 的 R14 与通则第五条：一个迁移文件的创建对象
/// 只属一个 schema，且该 schema 等于文件所在目录名。跨 schema 的登记行写入与
/// 触发器挂接经函数调用完成，不产生 CREATE 对象，因此不触本条——这与裁定 00c
/// 给出的三个回填迁移的写法一致。
fn rule_single_responsibility(rel: &str, dir_schema: &str, stmts: &[Stmt]) -> Vec<Violation> {
    let mut v = Vec::new();
    let mut seen: Vec<String> = Vec::new();
    for s in stmts {
        let Some(target) = created_object(&s.norm) else {
            continue;
        };
        let Some(schema) = schema_of(&target) else {
            continue;
        };
        if !seen.iter().any(|x| x == schema) {
            seen.push(schema.to_string());
        }
        if schema != dir_schema {
            v.push(at("SQL-010", rel, s.line, format!("创建 {target}，但文件位于 {dir_schema} 目录下；迁移放在其主要创建对象所属 schema 的目录")));
        }
    }
    if seen.len() > 1 {
        v.push(at(
            "SQL-010",
            rel,
            0,
            format!("同一迁移创建了 {} 两个以上 schema 的对象", seen.join("、")),
        ));
    }
    v
}

/// 取一条 DDL 创建的对象限定名。只认建表、建索引、建视图与建序列四类。
fn created_object(norm: &str) -> Option<String> {
    const HEADS: [&str; 6] = [
        "create table ",
        "create unique index ",
        "create index ",
        "create view ",
        "create materialized view ",
        "create sequence ",
    ];
    for head in HEADS {
        let Some(rest) = norm.strip_prefix(head) else {
            continue;
        };
        let rest = rest.strip_prefix("concurrently ").unwrap_or(rest);
        let rest = rest.strip_prefix("if not exists ").unwrap_or(rest);
        // 索引名不带 schema，其归属看 on 子句的表。
        let target = if head.contains("index") {
            rest.split(" on ").nth(1)?
        } else {
            rest
        };
        return Some(
            target
                .split(|c: char| c.is_whitespace() || c == '(')
                .next()?
                .to_string(),
        );
    }
    None
}

fn rule_ci_probe(rel: &str, lines: &[(usize, String)]) -> Vec<Violation> {
    let why = "生产迁移目录中不得出现 ci_probe；探针表只建在测试库";
    line_rule(rel, lines, "SQL-030", |l| l.contains("ci_probe"), why)
}

/// SQL-031：仅追加登记与守卫挂接的次序一致性。**跨文件判据**，被测面是整个迁移目录。
///
/// # 为什么这一条能静态判，而 `db/checks/append_only_consistency.sql` 不能
///
/// 那份活库脚本比的是 `platform_core.append_only_registry` 与 `pg_trigger`，
/// 两侧都要连库。但触发器并非各迁移里字面 `create trigger` 出来的，而是由
/// `platform_core.attach_table_guards(schema, table)` 在被调用时**读登记表**决定挂哪一个
/// （见 `V20260901090500__platform_core_conventions.sql` 的三分支）。该函数的注释逐字写着
/// 「空库全序下本函数先于登记表被调用，表不存在时尚无任何登记行，**按未登记处理**，
/// 不得因缺表报错」——**于是「attach 调用排在登记 insert 之前」这一情形会静默挂不上守卫**，
/// 而两侧的产生处都在迁移文本里，可静态判。
///
/// # 本条**不覆盖**什么（不得读作 append_only_consistency.sql 已有承接方）
///
/// 一、活库漂移：有人手工 drop 触发器、或迁移只跑了一半，本条看不见；
/// 二、活库脚本的反向分支 `UNREGISTERED_TRIGGER`：那种触发器只可能由本函数按登记创建，
///     静态上不可达，故本条不判该向；
/// 三、结论只及于**迁移文本**的自洽，不等于目标库的实际状态。
///
/// 附录辛第 24 条（该脚本被七处文档指名由 `xtask sqlcheck` 执行，而本工具无 postgres 客户端）
/// **由本条部分承接，不是全部**：活库那一半仍缺执行方，属附录辛第 27 条。
fn check_append_only_guards(root: &Path, migrations: &[PathBuf]) -> (Vec<Violation>, usize) {
    // (schema, table) -> (版本号, 模式, 文件相对路径, 行号)
    let mut regs: BTreeMap<(String, String), (u64, String, String, usize)> = BTreeMap::new();
    // (schema, table) -> (版本号, 文件相对路径, 行号)
    let mut atts: BTreeMap<(String, String), (u64, String, usize)> = BTreeMap::new();

    for path in migrations {
        let rel = relative(root, path);
        let Some(ver) = file_version(&rel) else {
            continue;
        };
        let Ok(text) = fs::read_to_string(path) else {
            continue;
        };
        for st in statements(&strip_comments(&text)) {
            for (sc, tb, mode) in parse_registry_rows(&st.norm) {
                regs.entry((sc, tb))
                    .or_insert((ver, mode, rel.clone(), st.line));
            }
            if let Some((sc, tb)) = parse_attach_call(&st.norm) {
                atts.entry((sc, tb)).or_insert((ver, rel.clone(), st.line));
            }
        }
    }

    let out = compare_guard_maps(&regs, &atts);
    let n = regs.len();
    (out, n)
}

/// SQL-031 的比对本体。**纯函数**：两张表进、违反明细出，不碰文件系统。
///
/// 抽出来是为了让**违反分支**可被直接测到——只在「通过」路径上被测过的判据，
/// 与恒真判据在效果上没有区别。
fn compare_guard_maps(
    regs: &BTreeMap<(String, String), (u64, String, String, usize)>,
    atts: &BTreeMap<(String, String), (u64, String, usize)>,
) -> Vec<Violation> {
    let mut out = Vec::new();
    for ((sc, tb), (rver, mode, rfile, rline)) in regs {
        match atts.get(&(sc.clone(), tb.clone())) {
            None => out.push(Violation {
                rule: "SQL-031",
                file: rfile.clone(),
                line: *rline,
                detail: format!(
                    "{sc}.{tb} 以 {mode} 登记入 append_only_registry，\
                     但全部迁移里没有一处 attach_table_guards('{sc}', '{tb}')；\
                     登记在而守卫不挂，该表的仅追加约束不成立"
                ),
            }),
            Some((aver, afile, aline)) if aver < rver => out.push(Violation {
                rule: "SQL-031",
                file: afile.clone(),
                line: *aline,
                detail: format!(
                    "{sc}.{tb} 的 attach_table_guards 在版本 {aver} 调用，\
                     而其 append_only_registry 登记行在版本 {rver}（{rfile}:{rline}）；\
                     attach 读登记表决定挂哪个触发器，排在登记之前会按未登记处理、\
                     静默挂不上 {mode} 守卫"
                ),
            }),
            Some(_) => {}
        }
    }
    out.sort_by_key(|v| (v.file.clone(), v.line));
    out
}

/// 从一条已归一的语句里解析全部 `append_only_registry` 登记元组。
///
/// 被解析的形态取自现存迁移：`insert into platform_core.append_only_registry
/// (id, schema_name, table_name, mode) values ('<uuid>', '<schema>', '<table>', '<mode>')`，
/// 元组跨行书写，故按**语句**而不是按行解析（[`Stmt::norm`] 已把换行归一为空格）。
/// `norm` 已转小写，因此模式字面量按小写匹配。
fn parse_registry_rows(norm: &str) -> Vec<(String, String, String)> {
    if !norm.starts_with("insert into") || !norm.contains("append_only_registry") {
        return Vec::new();
    }
    let mut out = Vec::new();
    for tuple in norm.split('(').skip(1) {
        let body = match tuple.find(')') {
            Some(k) => &tuple[..k],
            None => tuple,
        };
        let mode = if body.contains("'append_only'") {
            "APPEND_ONLY"
        } else if body.contains("'immutable_columns'") {
            "IMMUTABLE_COLUMNS"
        } else {
            continue;
        };
        // 元组内的带引号取值：uuid、schema、table、mode。取形如标识符且非模式字面量的前两个。
        let quoted: Vec<&str> = body.split('\'').skip(1).step_by(2).collect();
        let mut idents = quoted.iter().filter(|q| {
            !q.is_empty()
                && q.len() <= 63
                && q.chars()
                    .all(|c| c.is_ascii_lowercase() || c == '_' || c.is_ascii_digit())
                && *q != &"append_only"
                && *q != &"immutable_columns"
        });
        let (Some(sc), Some(tb)) = (idents.next(), idents.next()) else {
            continue;
        };
        out.push((sc.to_string(), tb.to_string(), mode.to_string()));
    }
    out
}

/// 解析一条 `attach_table_guards('schema', 'table')` 调用。
fn parse_attach_call(norm: &str) -> Option<(String, String)> {
    let at = norm.find("attach_table_guards")?;
    let rest = &norm[at..];
    let open = rest.find('(')?;
    let quoted: Vec<&str> = rest[open..].split('\'').skip(1).step_by(2).collect();
    Some((quoted.first()?.to_string(), quoted.get(1)?.to_string()))
}

/// 文件名版本号 `V<14 位数字>__<名字>.sql`。
fn file_version(rel: &str) -> Option<u64> {
    let stem = rel.rsplit('/').next()?.strip_suffix(".sql")?;
    let (digits, name) = stem.strip_prefix('V')?.split_once("__")?;
    if digits.len() != 14 || !digits.chars().all(|c| c.is_ascii_digit()) || name.is_empty() {
        return None;
    }
    digits.parse().ok()
}

/// 版本号全局唯一且严格递增。递增性按「目录内文件名字典序即版本序」判定：
/// 版本号唯一时全序由排序结果给出，重号是唯一会破坏全序的情形。
fn check_versions(versions: &BTreeMap<u64, Vec<String>>) -> Vec<Violation> {
    versions
        .iter()
        .filter(|(_, files)| files.len() > 1)
        .map(|(v, files)| {
            let why = format!(
                "版本号 {v} 被 {} 个文件占用；版本号必须全局唯一",
                files.len()
            );
            at("SQL-011", &files.join(" 与 "), 0, why)
        })
        .collect()
}

// ---------------------------------------------------------------------------
// 引导目录
// ---------------------------------------------------------------------------

pub fn scan_bootstrap(rel: &str, text: &str) -> Vec<Violation> {
    let why = "出现口令字面量；口令由安装器从机密库读取后经 ALTER ROLE … PASSWORD 单独注入";
    line_rule(rel, &strip_comments(text), "SQL-020", password_literal, why)
}

/// `password '…'` 形态即口令字面量。`password null` 与不带字面量的语法不算。
fn password_literal(line: &str) -> bool {
    let Some(idx) = line.find("password") else {
        return false;
    };
    line[idx + "password".len()..]
        .trim_start()
        .starts_with('\'')
}

fn check_bootstrap_names(dir: &Path) -> Vec<Violation> {
    let Ok(entries) = fs::read_dir(dir) else {
        return vec![at(
            "SQL-021",
            BOOTSTRAP,
            0,
            "目录读不出来，判不了文件名白名单".into(),
        )];
    };
    let mut names: Vec<String> = entries
        .filter_map(Result::ok)
        .map(|e| e.file_name().to_string_lossy().to_string())
        .filter(|n| !n.starts_with('.'))
        .collect();
    names.sort();
    let convention = BOOTSTRAP_FILES.join("、");
    names
        .iter()
        .filter(|n| !BOOTSTRAP_FILES.contains(&n.as_str()))
        .map(|n| {
            at(
                "SQL-021",
                &format!("{BOOTSTRAP}/{n}"),
                0,
                format!("不在约定文件名内。约定为 {convention}"),
            )
        })
        .collect()
}

#[cfg(test)]
mod negative_samples {
    use super::*;

    /// SQL-031 的解析：登记元组跨行书写，按语句解析取得到。
    #[test]
    fn a_multiline_registry_tuple_is_parsed() {
        let norm = "insert into platform_core.append_only_registry                     (id, schema_name, table_name, mode) values                     ('00000000-0000-7000-8000-000000000102', 'platform_core',                     'login_attempts', 'append_only')";
        assert_eq!(
            parse_registry_rows(norm),
            vec![(
                "platform_core".to_string(),
                "login_attempts".to_string(),
                "APPEND_ONLY".to_string()
            )]
        );
    }

    /// 模式字面量本身不得被当成 schema 或 table。
    #[test]
    fn the_mode_literal_is_not_mistaken_for_an_identifier() {
        let norm = "insert into platform_core.append_only_registry (schema_name,                     table_name, mode) values ('platform_core', 'x', 'immutable_columns')";
        let got = parse_registry_rows(norm);
        assert_eq!(got.len(), 1);
        assert_eq!(got[0].0, "platform_core");
        assert_eq!(got[0].1, "x");
        assert_eq!(got[0].2, "IMMUTABLE_COLUMNS");
    }

    /// 非登记语句不产出元组。
    #[test]
    fn an_unrelated_insert_yields_nothing() {
        assert!(
            parse_registry_rows("insert into platform_core.roles (code) values ('x')").is_empty()
        );
        assert!(parse_registry_rows("select 1").is_empty());
    }

    /// attach 调用的解析。
    #[test]
    fn an_attach_call_is_parsed() {
        assert_eq!(
            parse_attach_call(
                "select platform_core.attach_table_guards('platform_authz', 'sod_rules')"
            ),
            Some(("platform_authz".to_string(), "sod_rules".to_string()))
        );
        assert_eq!(parse_attach_call("select 1"), None);
    }

    /// 违反一：有登记行而全仓没有对应的 attach 调用。
    #[test]
    fn a_registry_row_without_an_attach_call_is_caught() {
        let mut regs = BTreeMap::new();
        regs.insert(
            ("platform_core".to_string(), "audit_events".to_string()),
            (
                20260901093000u64,
                "APPEND_ONLY".to_string(),
                "db/migrations/x.sql".to_string(),
                12usize,
            ),
        );
        let v = compare_guard_maps(&regs, &BTreeMap::new());
        assert_eq!(v.len(), 1, "{v:#?}");
        assert_eq!(v[0].rule, "SQL-031");
        assert!(
            v[0].detail.contains("没有一处 attach_table_guards"),
            "{}",
            v[0].detail
        );
    }

    /// 违反二：attach 调用排在登记 insert 之前——函数按未登记处理，静默挂不上守卫。
    #[test]
    fn an_attach_before_its_registry_row_is_caught() {
        let key = ("platform_core".to_string(), "login_attempts".to_string());
        let mut regs = BTreeMap::new();
        regs.insert(
            key.clone(),
            (
                20261012093000u64,
                "APPEND_ONLY".to_string(),
                "db/migrations/reg.sql".to_string(),
                55usize,
            ),
        );
        let mut atts = BTreeMap::new();
        atts.insert(
            key,
            (
                20260901090500u64,
                "db/migrations/early.sql".to_string(),
                9usize,
            ),
        );
        let v = compare_guard_maps(&regs, &atts);
        assert_eq!(v.len(), 1, "{v:#?}");
        assert!(v[0].detail.contains("静默挂不上"), "{}", v[0].detail);
    }

    /// 次序正确即不报：同一版本内先登记后挂接是合法形态。
    #[test]
    fn an_attach_at_or_after_its_registry_row_is_clean() {
        let key = ("platform_core".to_string(), "login_attempts".to_string());
        let mut regs = BTreeMap::new();
        regs.insert(
            key.clone(),
            (
                20261012093000u64,
                "APPEND_ONLY".to_string(),
                "db/migrations/reg.sql".to_string(),
                55usize,
            ),
        );
        let mut atts = BTreeMap::new();
        atts.insert(
            key,
            (
                20261012093000u64,
                "db/migrations/reg.sql".to_string(),
                63usize,
            ),
        );
        assert!(compare_guard_maps(&regs, &atts).is_empty());
    }

    /// 仓库现状：两条登记行各自有 attach 调用，且次序正确。
    /// 这一条同时证明 SQL-031 今天有被测输入，不是恒过。
    #[test]
    fn the_repository_has_consistent_append_only_guards() {
        let root = Path::new(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .expect("xtask 在工作区根之下")
            .to_path_buf();
        let migrations = sql_files(&root.join(MIGRATIONS));
        assert!(!migrations.is_empty(), "迁移目录不该是空的");
        let (violations, registered) = check_append_only_guards(&root, &migrations);
        assert!(
            registered >= 2,
            "至少应解析到两条登记行，实测 {registered}——解析不到就等于判据恒过"
        );
        assert!(violations.is_empty(), "{violations:#?}");
    }

    const OK_TABLE: &str = r#"-- rollback: drop table sales.sales_orders;
create table sales.sales_orders (
  id uuid not null,
  legal_entity_id uuid not null,
  security_level smallint not null default 20,
  data_scope_tags text[] not null default '{}',
  row_version bigint not null default 1,
  created_at timestamptz not null default now(),
  created_by uuid not null,
  updated_at timestamptz not null default now(),
  updated_by uuid not null,
  doc_no text not null,
  constraint pk_sales_orders primary key (id),
  constraint ck_sales_orders_doc_no_len check (length(doc_no) <= 64)
);
"#;

    const PATH: &str = "db/migrations/sales/V20260810120000__sales_orders.sql";

    fn rules_hit(text: &str) -> Vec<&'static str> {
        let mut r: Vec<&'static str> = scan_migration(PATH, text)
            .into_iter()
            .map(|v| v.rule)
            .collect();
        r.sort_unstable();
        r.dedup();
        r
    }

    /// 正样例：合规迁移一条都不触。没有这条，下面的负样例证明不了规则在判什么。
    #[test]
    fn a_compliant_migration_trips_nothing() {
        assert_eq!(
            rules_hit(OK_TABLE),
            Vec::<&str>::new(),
            "合规样例不得报违反"
        );
    }

    /// 十项负样例，逐条断言规则本身：每条只改 OK_TABLE 的一处，断言恰好触到该规则。
    #[test]
    fn each_rule_has_a_negative_sample() {
        let cases: [(&str, String); 10] = [
            ("SQL-001", format!("{OK_TABLE}delete from sales.sales_orders where id = '1';")),
            ("SQL-002", OK_TABLE.replace("doc_no text not null", "doc_no varchar(64) not null")),
            ("SQL-003", format!("{OK_TABLE}create type sales.order_status as enum ('draft');")),
            ("SQL-004", OK_TABLE.replace("default now()", "default current_date")),
            ("SQL-005", OK_TABLE.replace("  doc_no text not null,", "  doc_no text not null,\n  customer_id uuid not null references mdm.customers (id),")),
            ("SQL-006", OK_TABLE.replace("primary key (id)", "primary key (id) on delete cascade")),
            ("SQL-007", OK_TABLE.replace("-- rollback: drop table sales.sales_orders;", "-- 忘了写回退段")),
            ("SQL-008", OK_TABLE.replace("  legal_entity_id uuid not null,\n", "")),
            ("SQL-009", OK_TABLE.replace("pk_sales_orders", "sales_orders_pkey")),
            ("SQL-010", OK_TABLE.replace("create table sales.sales_orders", "create table ledger.vouchers")),
        ];
        for (rule, text) in cases {
            let hit = rules_hit(&text);
            assert!(hit.contains(&rule), "负样例应触 {rule}，实际触到 {hit:?}");
        }
    }

    #[test]
    fn negative_delete_is_allowed_only_on_two_schemas() {
        let lines =
            strip_comments("delete from platform_msg.idempotency_keys where expires_at < now();");
        assert!(
            rule_delete(PATH, &lines).is_empty(),
            "platform_msg 上的清理是放行项"
        );
        let lines = strip_comments("delete from sales.sales_orders where id = '1';");
        assert_eq!(rule_delete(PATH, &lines).len(), 1);
        // 未限定名判不出 schema，不得当作放行。
        let lines = strip_comments("delete from sales_orders;");
        assert_eq!(rule_delete(PATH, &lines).len(), 1);
    }

    #[test]
    fn negative_cross_schema_fk_must_be_composite() {
        let composite = "create table sales.sales_orders (customer_id uuid, constraint fk_sales_orders_customers foreign key (legal_entity_id, customer_id) references mdm.customers (legal_entity_id, id) on delete restrict)";
        let stmts = statements(&strip_comments(composite));
        assert!(
            rule_cross_schema_fk(PATH, "sales", &stmts).is_empty(),
            "复合形式合规"
        );
        let no_restrict = composite.replace(" on delete restrict", "");
        let stmts = statements(&strip_comments(&no_restrict));
        let hit = rule_cross_schema_fk(PATH, "sales", &stmts);
        assert_eq!(hit.len(), 1);
        assert!(hit[0].detail.contains("ON DELETE RESTRICT"));
        // 同 schema 的单列外键不触本条。
        let same = "create table sales.sales_order_lines (order_id uuid references sales.sales_orders (id))";
        let stmts = statements(&strip_comments(same));
        assert!(rule_cross_schema_fk(PATH, "sales", &stmts).is_empty());
    }

    #[test]
    fn negative_append_only_table_must_omit_all_three_columns() {
        let base = OK_TABLE
            .replace("  row_version bigint not null default 1,\n", "")
            .replace("  updated_at timestamptz not null default now(),\n", "")
            .replace("  updated_by uuid not null,\n", "");
        assert!(
            rules_hit(&base).is_empty(),
            "三列同缺是合法的仅追加表：{:?}",
            rules_hit(&base)
        );
        let half = OK_TABLE.replace("  row_version bigint not null default 1,\n", "");
        assert!(
            rules_hit(&half).contains(&"SQL-008"),
            "只缺 row_version 必须报"
        );
    }

    #[test]
    fn negative_common_column_order_is_asserted_not_just_presence() {
        let swapped = OK_TABLE.replace(
            "  id uuid not null,\n  legal_entity_id uuid not null,\n",
            "  legal_entity_id uuid not null,\n  id uuid not null,\n",
        );
        let v = scan_migration(PATH, &swapped);
        assert!(
            v.iter()
                .any(|x| x.rule == "SQL-008" && x.detail.contains("顺序")),
            "换序必须报，不能只判齐备：{v:?}"
        );
    }

    #[test]
    fn negative_column_suffix_must_match_type() {
        let bad = OK_TABLE.replace(
            "  created_at timestamptz not null default now(),",
            "  created_at date not null,",
        );
        assert!(
            rules_hit(&bad).contains(&"SQL-009"),
            "_at 列必须是 timestamptz"
        );
    }

    #[test]
    fn negative_file_name_and_version_rules() {
        assert_eq!(
            file_version("db/migrations/sales/V20260810120000__x.sql"),
            Some(20_260_810_120_000)
        );
        assert_eq!(file_version("db/migrations/sales/V202608__x.sql"), None);
        assert_eq!(file_version("db/migrations/sales/sales_orders.sql"), None);
        let mut m: BTreeMap<u64, Vec<String>> = BTreeMap::new();
        m.insert(1, vec!["a".into()]);
        assert!(check_versions(&m).is_empty());
        m.insert(2, vec!["b".into(), "c".into()]);
        assert_eq!(check_versions(&m).len(), 1);
    }

    #[test]
    fn negative_bootstrap_password_literal() {
        let hit = scan_bootstrap(
            "db/bootstrap/01_roles.sql",
            "alter role ep_app_rw password 'hunter2';",
        );
        assert_eq!(hit.len(), 1);
        assert_eq!(hit[0].rule, "SQL-020");
        assert!(
            scan_bootstrap("db/bootstrap/01_roles.sql", "create role ep_app_rw login;").is_empty()
        );
        // 注释里的口令同样要被剥掉再判，不能因为在注释里就漏判或误判。
        assert!(scan_bootstrap("db/bootstrap/01_roles.sql", "-- password 'x'").is_empty());
    }

    #[test]
    fn negative_ci_probe_must_not_reach_production_migrations() {
        assert!(
            rules_hit("-- rollback: x\ncreate table ci_probe.probe_records (id uuid);")
                .contains(&"SQL-030")
        );
    }

    /// 空扫描必须落进 uncovered 而不是 checked，这是本模块最重的一条纪律。
    #[test]
    fn negative_empty_scan_is_uncovered_not_clean() {
        let empty = Path::new("/nonexistent-root-for-sqlcheck");
        let r = run(empty);
        assert!(r.problems.is_empty());
        assert!(!r.uncovered.is_empty(), "空扫描必须报未覆盖");
        assert!(r.checked.is_empty());
        assert_eq!(r.outcome(), Outcome::Uncovered, "空扫描不得判 Clean");
    }
}
