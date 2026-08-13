//! 工作区依赖图模型。
//!
//! 图从 `cargo metadata --no-deps` 读入，但规则判定只依赖本模块的数据结构，
//! 因此负样例可以直接构造合成图喂给同一批判定函数，无需真造违规 crate。

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use std::process::Command;

/// crate 在依赖方向上的层位。
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Layer {
    Foundation,
    Platform(String),
    Contract(String),
    Domain(String),
    Application(String),
    Adapter(String),
    /// 八个二进制进程，负责装配，可依赖全部。
    Process(String),
    /// 开发期工具与夹具，不进制品或只随制品交付，不受层位约束。
    Tooling(String),
}

impl Layer {
    /// 按 crate 名判定层位。名字是唯一判据，目录一致性由 naming 规则单独断言。
    pub fn of(name: &str, is_process: bool) -> Layer {
        if name == "ep-foundation" {
            return Layer::Foundation;
        }
        for (prefix, wrap) in [
            ("ep-platform-", Layer::Platform as fn(String) -> Layer),
            ("ep-contract-", Layer::Contract as fn(String) -> Layer),
            ("ep-domain-", Layer::Domain as fn(String) -> Layer),
            ("ep-app-", Layer::Application as fn(String) -> Layer),
            ("ep-adapter-", Layer::Adapter as fn(String) -> Layer),
        ] {
            if let Some(rest) = name.strip_prefix(prefix) {
                return wrap(rest.to_string());
            }
        }
        if is_process {
            Layer::Process(name.to_string())
        } else {
            Layer::Tooling(name.to_string())
        }
    }

    pub fn module(&self) -> Option<&str> {
        match self {
            Layer::Contract(m) | Layer::Domain(m) | Layer::Application(m) => Some(m),
            _ => None,
        }
    }
}

#[derive(Clone, Debug)]
pub struct Package {
    pub name: String,
    /// 相对工作区根的目录，如 `crates/platform/tenancy`。
    pub dir: String,
    pub layer: Layer,
    /// 全部直接依赖名，含工作区外的第三方。
    pub deps: Vec<String>,
}

#[derive(Clone, Debug, Default)]
pub struct Workspace {
    pub root: PathBuf,
    pub packages: Vec<Package>,
}

impl Workspace {
    pub fn member_names(&self) -> Vec<&str> {
        self.packages.iter().map(|p| p.name.as_str()).collect()
    }

    pub fn get(&self, name: &str) -> Option<&Package> {
        self.packages.iter().find(|p| p.name == name)
    }

    /// 供负样例构造合成图。
    #[cfg(test)]
    pub fn from_packages(packages: Vec<Package>) -> Workspace {
        Workspace {
            root: PathBuf::new(),
            packages,
        }
    }
}

/// 调用 `cargo metadata --no-deps` 载入工作区。
pub fn load(root: &Path) -> Result<Workspace, String> {
    let out = Command::new(std::env::var("CARGO").unwrap_or_else(|_| "cargo".into()))
        .args(["metadata", "--no-deps", "--format-version", "1"])
        .current_dir(root)
        .output()
        .map_err(|e| format!("cargo metadata 启动失败: {e}"))?;
    if !out.status.success() {
        return Err(format!(
            "cargo metadata 返回 {}: {}",
            out.status,
            String::from_utf8_lossy(&out.stderr).trim()
        ));
    }
    let meta: serde_json::Value =
        serde_json::from_slice(&out.stdout).map_err(|e| format!("metadata 解析失败: {e}"))?;
    parse(root, &meta)
}

fn parse(root: &Path, meta: &serde_json::Value) -> Result<Workspace, String> {
    let members = meta["packages"]
        .as_array()
        .ok_or("metadata 缺少 packages 数组")?;

    // 先按 manifest 路径判定哪些是二进制进程：apps/ 下的成员。
    let mut packages = Vec::new();
    for p in members {
        let name = p["name"].as_str().ok_or("包缺少 name")?.to_string();
        let manifest = Path::new(p["manifest_path"].as_str().ok_or("包缺少 manifest_path")?);
        let dir = manifest
            .parent()
            .and_then(|d| d.strip_prefix(root).ok())
            .map(|d| d.to_string_lossy().replace('\\', "/"))
            .ok_or_else(|| format!("{name} 的目录不在工作区根之下"))?;
        let is_process = dir.starts_with("apps/");
        let deps = p["dependencies"]
            .as_array()
            .map(|ds| {
                ds.iter()
                    .filter_map(|d| d["name"].as_str().map(str::to_string))
                    .collect()
            })
            .unwrap_or_default();
        packages.push(Package {
            layer: Layer::of(&name, is_process),
            name,
            dir,
            deps,
        });
    }
    packages.sort_by(|a, b| a.name.cmp(&b.name));
    Ok(Workspace {
        root: root.to_path_buf(),
        packages,
    })
}

/// 目录名到期望 crate 名的映射，即技术基线第 1.1 节的命名约定。
///
/// `crates/` 下目录名一律不带 `ep-` 前缀，`Cargo.toml` 的 name 带前缀；
/// `apps/` 下的二进制 crate 名即进程名，不带前缀。
pub fn expected_name(dir: &str) -> Option<String> {
    let fixed: BTreeMap<&str, &str> = [
        ("crates/foundation", "ep-foundation"),
        ("testkit", "ep-testkit"),
        ("datagen", "ep-datagen"),
        ("xtask", "ep-xtask"),
    ]
    .into_iter()
    .collect();
    if let Some(n) = fixed.get(dir) {
        return Some((*n).to_string());
    }
    for (prefix, crate_prefix) in [
        ("crates/platform/", "ep-platform-"),
        ("crates/contract/", "ep-contract-"),
        ("crates/domain/", "ep-domain-"),
        ("crates/application/", "ep-app-"),
        ("crates/adapter/", "ep-adapter-"),
        ("tools/", "ep-"),
    ] {
        if let Some(rest) = dir.strip_prefix(prefix) {
            if rest.contains('/') {
                return None;
            }
            return Some(format!("{crate_prefix}{rest}"));
        }
    }
    if let Some(rest) = dir.strip_prefix("apps/") {
        if rest.contains('/') {
            return None;
        }
        return Some(rest.to_string());
    }
    None
}
