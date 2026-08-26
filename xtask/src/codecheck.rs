//! `xtask codecheck` —— 九个二进制与其进程名、systemd 单元名、cgroup slice 名的对应。
//!
//! 判据出处是阶段 1 计划第 3.2 节末句：「九个二进制 crate 名与进程名、systemd 单元名、
//! cgroup slice 名一一对应，由 `xtask codecheck` 断言」。
//!
//! 「一一对应」在四个维度上不是同一种对应，本模块按技术基线第 2 节进程表的实际取值判：
//! crate 名与进程名、单元名三者逐字相等（`apps/<p>/` 目录名、`Cargo.toml` 的 `name`、
//! `deploy/podman/<p>.container` 三处），而进程与 slice 是多对一——core-server 与
//! integration-gateway 同处 `app-core.slice`，ops-agent 落 `app-edge.slice`，
//! `app-db.slice` 承载的是 PostgreSQL 而不是本仓库的任何二进制。把这一维也判成双射
//! 会与基线第 2 节直接冲突，因此 slice 一维的判据是：每个进程的 slice 取值三处一致
//! （基线进程表、`.container` 的 `Slice=`、`deploy/systemd/system/` 下的实际文件），
//! 且每个 slice 文件都至少有一个容器认领它。
//!
//! 代码侧一律文本解析，不 use 任何 ep-* crate，也不调 `cargo metadata`：
//! 门禁工具不为一条判据增加依赖边，这与 archcheck 同纪律。
//!
//! 裁定 F-08 之后：基线第 2 节那一列已由 `cgroup slice` 改名为「资源单位」，
//! 取值形态随之由 `app-core.slice` 变为 `资源单位 app-core`；单元名一维的被测串
//! 也应由 systemd 单元名换成 Windows 服务名。第三、四维的被测对象
//! （`deploy/podman/` 与 `deploy/systemd/`）按 F-08 第十节第 5 步才被取代，
//! 在那之前它们仍在仓库里、仍应保持一致，故本模块暂不改判据形态，
//! 只在 [`resource_unit`] 处归一两种列取值。第 5 步落地即自动转为未覆盖，
//! 那时按 F-08 第八节重写——不需要任何人记得。

use std::collections::BTreeMap;
use std::fs;
use std::path::Path;

const APPS: &str = "apps";
const QUADLET: &str = "deploy/podman";
const SYSTEMD: &str = "deploy/systemd/system";
const BASELINE: &str =
    "docs/superpowers/plans/2026-08-10-first-release-dev-plan/00b-technical-baseline.md";
/// 基线第 2 节进程表所在小节。
const BASELINE_SECTION: &str = "## 2. 进程清单";
/// 基线第 2 节点名的进程数。变更进程数必须先改基线，本工具据此判计数漂移。
const EXPECTED_PROCESSES: usize = 9;

#[derive(Debug)]
pub struct Report {
    pub problems: Vec<String>,
    /// 未覆盖：被测输入缺席。不得读作通过。
    pub uncovered: Vec<String>,
    /// 已判定的进程数。
    pub checked: usize,
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

    let crates = match app_crates(root) {
        Ok(v) if v.is_empty() => {
            uncovered.push(format!(
                "未覆盖：{APPS}/ 下没有任何 crate，四维对应无被测输入"
            ));
            Vec::new()
        }
        Ok(v) => v,
        Err(e) => {
            uncovered.push(format!("未覆盖：{e}"));
            Vec::new()
        }
    };

    let table = match process_table(root) {
        Ok(t) => t,
        Err(e) => {
            uncovered.push(format!("未覆盖：{e}"));
            BTreeMap::new()
        }
    };

    if crates.is_empty() || table.is_empty() {
        return Report {
            problems,
            uncovered,
            checked: 0,
        };
    }

    if table.len() != EXPECTED_PROCESSES {
        problems.push(format!(
            "{BASELINE} 第 2 节进程表有 {} 行，基线声称 {EXPECTED_PROCESSES} 个进程；计数漂移即不通过",
            table.len()
        ));
    }

    // 一、crate 名与目录名。二、crate 集合与基线进程表逐项相等。
    for c in &crates {
        if c.dir != c.package {
            problems.push(format!(
                "{APPS}/{} 的目录名与 Cargo.toml 的 name「{}」不一致",
                c.dir, c.package
            ));
        }
        if !c.has_main {
            problems.push(format!(
                "{APPS}/{}/src/main.rs 不存在，它不是一个二进制 crate",
                c.dir
            ));
        }
        if let Some(bin) = &c.bin_name {
            if bin != &c.package {
                problems.push(format!(
                    "{APPS}/{} 用 [[bin]] 把二进制名改成了 {bin}，进程名与 crate 名因此不再相等",
                    c.dir
                ));
            }
        }
        if !table.contains_key(&c.package) {
            problems.push(format!(
                "{BASELINE} 第 2 节进程表里没有 {} 这一行",
                c.package
            ));
        }
    }
    for name in table.keys() {
        if !crates.iter().any(|c| &c.package == name) {
            problems.push(format!("基线进程表登记了 {name}，{APPS}/ 下没有对应 crate"));
        }
    }

    // 三、systemd 单元名与 slice 取值。
    let mut claimed: Vec<String> = Vec::new();
    for (name, want_slice) in &table {
        let unit = root.join(QUADLET).join(format!("{name}.container"));
        let Ok(text) = fs::read_to_string(&unit) else {
            problems.push(format!(
                "{QUADLET}/{name}.container 不存在，进程 {name} 没有对应的 systemd 单元"
            ));
            continue;
        };
        match ini_value(&text, "Slice") {
            None => problems.push(format!("{QUADLET}/{name}.container 没有写 Slice=")),
            Some(got) => {
                if &got != want_slice {
                    problems.push(format!(
                        "{name} 的 slice 两侧不一致。基线进程表：{want_slice}；{QUADLET}/{name}.container：{got}"
                    ));
                }
                claimed.push(got);
            }
        }
    }
    // 非本仓库二进制的容器（如 PostgreSQL）同样要认领一个 slice，
    // 否则 app-db.slice 这类没有对应进程的分片会成为无人认领的孤儿。
    for extra in container_names(root) {
        if table.contains_key(&extra) {
            continue;
        }
        let unit = root.join(QUADLET).join(format!("{extra}.container"));
        if let Ok(text) = fs::read_to_string(&unit) {
            if let Some(s) = ini_value(&text, "Slice") {
                claimed.push(s);
            }
        }
    }

    // 四、slice 文件与其 drop-in 目录存在，且每个 slice 文件都被认领。
    let slice_dir = root.join(SYSTEMD);
    let mut slices = slice_files(&slice_dir);
    slices.sort();
    if slices.is_empty() {
        uncovered.push(format!("未覆盖：{SYSTEMD} 下没有任何 .slice 文件"));
    }
    for s in &claimed {
        if !slices.contains(s) {
            problems.push(format!("{SYSTEMD}/{s} 不存在，但有容器把 Slice 指向它"));
        }
        let dropin = slice_dir.join(format!("{s}.d"));
        if !dropin.is_dir() {
            problems.push(format!(
                "{SYSTEMD}/{s}.d/ 不存在，该 slice 没有资源限额 drop-in"
            ));
        }
    }
    for s in &slices {
        if !claimed.contains(s) {
            problems.push(format!(
                "{SYSTEMD}/{s} 没有任何容器认领，它承载不了任何进程"
            ));
        }
    }

    Report {
        problems,
        uncovered,
        checked: crates.len(),
    }
}

#[derive(Clone, Debug)]
struct AppCrate {
    dir: String,
    package: String,
    bin_name: Option<String>,
    has_main: bool,
}

fn app_crates(root: &Path) -> Result<Vec<AppCrate>, String> {
    let dir = root.join(APPS);
    let entries = fs::read_dir(&dir).map_err(|_| format!("读不到目录 {APPS}/"))?;
    let mut paths: Vec<_> = entries.filter_map(Result::ok).map(|e| e.path()).collect();
    paths.sort();
    let mut out = Vec::new();
    for p in paths {
        if !p.is_dir() {
            continue;
        }
        let manifest = p.join("Cargo.toml");
        if !manifest.is_file() {
            continue;
        }
        let name = p
            .file_name()
            .unwrap_or_default()
            .to_string_lossy()
            .to_string();
        let text = fs::read_to_string(&manifest)
            .map_err(|_| format!("读不到 {APPS}/{name}/Cargo.toml"))?;
        out.push(AppCrate {
            dir: name,
            package: toml_string(&text, "[package]", "name").unwrap_or_default(),
            bin_name: toml_string(&text, "[[bin]]", "name"),
            has_main: p.join("src/main.rs").is_file(),
        });
    }
    Ok(out)
}

/// 取 TOML 某个表头之后的第一个 `key = "值"`。只够读 `Cargo.toml` 这几处固定形态，
/// 不做通用 TOML 解析——门禁工具不为这一处引一个解析器依赖。
fn toml_string(text: &str, table: &str, key: &str) -> Option<String> {
    let body = text.split_once(table)?.1;
    let body = body.split("\n[").next().unwrap_or(body);
    for line in body.lines().map(str::trim) {
        let Some((k, v)) = line.split_once('=') else {
            continue;
        };
        if k.trim() != key {
            continue;
        }
        let v = v.trim().trim_end_matches(',');
        return Some(v.trim_matches('"').to_string());
    }
    None
}

/// 基线第 2 节进程表最后一列 -> 该进程所属资源单位对应的 slice 文件名。
///
/// 该列有两种写法，都要认：裁定 F-08 之前是 `app-core.slice`，之后是 `资源单位 app-core`
/// ——Windows 上没有 cgroup slice，基线已按裁定换了列名与取值形态。
/// 本函数把两种写法归一到 slice 文件名，因为**下面第三、四维的被测对象仍是
/// `deploy/podman/` 与 `deploy/systemd/`**：这两处按 F-08 第十节第 5 步才被服务注册脚本
/// 与静态限额文件取代，在那之前它们仍在仓库里，仍应保持三处一致。
///
/// 第 5 步落地那一刻 `deploy/systemd/` 消失，第四维会自动转为「未覆盖」并以退出码 3
/// 报出——那正是要求本模块按 F-08 第八节重写的时点，不需要任何人记得去改。
fn resource_unit(cell: &str) -> Option<String> {
    if cell.ends_with(".slice") {
        return Some(cell.to_string());
    }
    let name = cell.strip_prefix("资源单位 ")?.trim();
    if name.is_empty() {
        return None;
    }
    Some(format!("{name}.slice"))
}

/// 基线第 2 节进程表：进程名 -> 资源单位（归一为 slice 文件名）。
fn process_table(root: &Path) -> Result<BTreeMap<String, String>, String> {
    let text = fs::read_to_string(root.join(BASELINE)).map_err(|_| format!("读不到 {BASELINE}"))?;
    let section = text
        .split_once(BASELINE_SECTION)
        .ok_or_else(|| format!("{BASELINE} 中找不到小节「{BASELINE_SECTION}」"))?
        .1;
    let section = section.split("\n## ").next().unwrap_or(section);
    let mut out = BTreeMap::new();
    for line in section.lines().map(str::trim) {
        if !line.starts_with('|') {
            continue;
        }
        let cells: Vec<&str> = line.trim_matches('|').split('|').map(str::trim).collect();
        if cells.len() != 6 || cells[0] == "进程" || cells[0].starts_with("---") {
            continue;
        }
        let Some(unit) = resource_unit(cells[5]) else {
            continue;
        };
        out.insert(cells[0].to_string(), unit);
    }
    if out.is_empty() {
        return Err(format!("{BASELINE} 第 2 节没有解析出任何进程行"));
    }
    Ok(out)
}

/// 取 systemd 单元文件里 `Key=值` 的值。同名键出现多次时取最后一次，与 systemd 同语义。
fn ini_value(text: &str, key: &str) -> Option<String> {
    let mut found = None;
    for line in text.lines().map(str::trim) {
        if line.starts_with('#') {
            continue;
        }
        if let Some(v) = line.strip_prefix(&format!("{key}=")) {
            found = Some(v.trim().to_string());
        }
    }
    found
}

fn container_names(root: &Path) -> Vec<String> {
    let mut out = Vec::new();
    let Ok(entries) = fs::read_dir(root.join(QUADLET)) else {
        return out;
    };
    for p in entries.filter_map(Result::ok).map(|e| e.path()) {
        if p.extension().is_some_and(|e| e == "container") {
            if let Some(stem) = p.file_stem() {
                out.push(stem.to_string_lossy().to_string());
            }
        }
    }
    out.sort();
    out
}

fn slice_files(dir: &Path) -> Vec<String> {
    let Ok(entries) = fs::read_dir(dir) else {
        return Vec::new();
    };
    entries
        .filter_map(Result::ok)
        .map(|e| e.file_name().to_string_lossy().to_string())
        .filter(|n| n.ends_with(".slice"))
        .collect()
}

#[cfg(test)]
mod negative_samples {
    use super::*;

    /// 正样例：当前仓库的四维对应本身就是被测对象，必须全绿。
    /// 它同时证明下面的负样例改的确实是判定结果，而不是本来就红。
    #[test]
    fn the_repository_itself_passes() {
        let r = run(&repo_root());
        assert!(r.problems.is_empty(), "仓库现状应通过：{:#?}", r.problems);
        assert_eq!(r.checked, EXPECTED_PROCESSES);
        assert_eq!(r.outcome(), Outcome::Clean, "{:#?}", r.uncovered);
    }

    /// 负样例断言的是「未覆盖不得判通过」这条规则本身。
    #[test]
    fn negative_missing_inputs_are_uncovered_not_clean() {
        let r = run(Path::new("/nonexistent-root-for-codecheck"));
        assert!(r.problems.is_empty());
        assert!(!r.uncovered.is_empty());
        assert_eq!(r.checked, 0);
        assert_eq!(r.outcome(), Outcome::Uncovered, "空输入不得判 Clean");
    }

    /// 负样例断言进程表解析这条规则：slice 列必须以 .slice 结尾才算一行进程。
    #[test]
    fn negative_process_table_rows_need_a_slice_column() {
        let root = repo_root();
        let table = process_table(&root).expect("基线第 2 节可解析");
        assert_eq!(table.len(), EXPECTED_PROCESSES);
        assert_eq!(
            table.get("core-server").map(String::as_str),
            Some("app-core.slice")
        );
        // 多对一是基线第 2 节的既定取值，不是缺陷：这两个进程同处一个 slice。
        assert_eq!(
            table.get("integration-gateway").map(String::as_str),
            Some("app-core.slice")
        );
    }

    /// 负样例断言 Slice= 比对这条规则：改一个字即报，且报的是这一条。
    #[test]
    fn negative_a_wrong_slice_value_is_caught() {
        let text = "[Service]\nSlice=app-worker.slice\nRestart=always\n";
        assert_eq!(
            ini_value(text, "Slice").as_deref(),
            Some("app-worker.slice")
        );
        assert_eq!(ini_value("[Service]\nRestart=always\n", "Slice"), None);
        // 注释掉的取值不算数，否则改一处漏一处时会读到已作废的值。
        assert_eq!(ini_value("# Slice=app-core.slice\n", "Slice"), None);
    }

    #[test]
    fn negative_bin_rename_breaks_the_correspondence() {
        let manifest = "[package]\nname = \"core-server\"\n\n[[bin]]\nname = \"epcore\"\n";
        assert_eq!(
            toml_string(manifest, "[package]", "name").as_deref(),
            Some("core-server")
        );
        assert_eq!(
            toml_string(manifest, "[[bin]]", "name").as_deref(),
            Some("epcore")
        );
        assert_eq!(
            toml_string("[package]\nversion = \"0\"\n", "[package]", "name"),
            None
        );
    }

    fn repo_root() -> std::path::PathBuf {
        Path::new(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .expect("xtask 在工作区根之下")
            .to_path_buf()
    }
}

#[cfg(test)]
mod resource_unit_tests {
    use super::resource_unit;

    /// 两种写法都要认：裁定 F-08 前后的列取值形态不同。
    #[test]
    fn both_forms_are_accepted() {
        assert_eq!(
            resource_unit("app-core.slice").as_deref(),
            Some("app-core.slice")
        );
        assert_eq!(
            resource_unit("资源单位 app-core").as_deref(),
            Some("app-core.slice")
        );
        assert_eq!(
            resource_unit("资源单位 app-db").as_deref(),
            Some("app-db.slice")
        );
    }

    /// 负样例：认不出来必须返回 None 而不是猜一个，
    /// 否则整张表会被解析成空表并静默退化为「未覆盖」。
    #[test]
    fn negative_unparsable_cells() {
        assert_eq!(resource_unit("ep-core"), None);
        assert_eq!(resource_unit("资源单位 "), None);
        assert_eq!(resource_unit(""), None);
        assert_eq!(resource_unit("Job Object app-core"), None);
    }
}
