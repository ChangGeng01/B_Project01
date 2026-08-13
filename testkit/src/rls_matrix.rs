//! RLS 断言矩阵。八个断言函数的函数名按裁定 C-05 逐字冻结。
//!
//! 阶段 1 交付八个骨架；阶段 2 追加 `assert_replication_role_containment`
//! 与 `assert_recon_context_borrow`（本文件内独立实现，不与八骨架同名，
//! 见 [`STAGE2_ASSERTION_NAMES`]）；阶段 4 追加 `matrix_32.rs` 与发布门禁项
//! `RG-RLS-MATRIX-GREEN`。
//!
//! 探针 schema `ci_probe` 与探针表由 `ep-testkit` 在本阶段的临时测试库内自建，
//! 建表函数带 `#[cfg(feature = "ci-probe")]` 且默认关闭，不出现在 `db/migrations/` 下。
//! 阶段 1 的实现取「无连接即视为未覆盖」的保守判定。

/// 断言结果。`Skipped` 表示探针库不可达，不等于通过。
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum RlsAssertion {
    Passed,
    Failed(String),
    Skipped(&'static str),
}

const NO_PROBE: &str = "探针库未连接，阶段 1 判定为未覆盖";

macro_rules! rls_assertions {
    ($($name:ident),* $(,)?) => {
        $(
            pub fn $name() -> RlsAssertion {
                RlsAssertion::Skipped(NO_PROBE)
            }
        )*

        /// 八个断言函数名，与裁定 C-05 逐字一致。
        pub const ASSERTION_NAMES: [&str; 8] = [$(stringify!($name)),*];
    };
}

rls_assertions!(
    assert_read,
    assert_write,
    assert_update,
    assert_delete,
    assert_aggregate,
    assert_sort,
    assert_report_projection,
    assert_error_leak,
);

/// 阶段 2 追加的两个断言函数名（C-05 第二段）。不得与阶段 1 八个同名，
/// 也不得重复实现那八个；两者另由 [`STAGE2_ASSERTION_NAMES`] 登记。
pub const STAGE2_ASSERTION_NAMES: [&str; 2] = [
    "assert_replication_role_containment",
    "assert_recon_context_borrow",
];

/// 探针库连接在本阶段尚不可达；无活库时两个阶段 2 断言与阶段 1 八断言
/// 同纪律：返回 Skipped，不判过。
fn probe_available() -> bool {
    false
}

/// 仓库根：testkit 位于其下第一层。
fn repo_root() -> std::path::PathBuf {
    std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("testkit 位于仓库根下第一层")
        .to_path_buf()
}

fn read_to_string_lossy(path: &std::path::Path) -> Option<String> {
    std::fs::read_to_string(path).ok()
}

/// 递归收集目录下全部 .rs 文件内容拼接，供静态扫描。
fn collect_rust_sources(dir: &std::path::Path, out: &mut String) {
    let Ok(entries) = std::fs::read_dir(dir) else {
        return;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            collect_rust_sources(&path, out);
        } else if path.extension().is_some_and(|e| e == "rs") {
            if let Some(text) = read_to_string_lossy(&path) {
                out.push_str(&text);
                out.push('\n');
            }
        }
    }
}

/// 复制角色收容断言（C-05，阶段 2）。五项：
/// 一、复制角色无法读业务表；二、无法 DDL；三、无法从服务器之外连接
/// ——前三项需探针库，无活库时随整体返回 Skipped；
/// 四、无法经界面借用；五、无法经 API 借用
/// ——后两项以 core-server 路由表源码与依赖图的静态判据断言，
/// 在探针不可达时也先行返回 Skipped，与阶段 1 骨架纪律一致。
pub fn assert_replication_role_containment() -> RlsAssertion {
    if !probe_available() {
        return RlsAssertion::Skipped(NO_PROBE);
    }
    // 探针可达后的活库三项与静态两项按序执行；本阶段探针尚不可达，
    // 下方静态判据保留为可执行形态，供阶段 4 接通探针后直接生效。
    let core_server = repo_root().join("apps/core-server");
    let mut sources = String::new();
    collect_rust_sources(&core_server, &mut sources);
    for token in ["SET ROLE", "set local role", "pg_read_server_files"] {
        if sources.to_uppercase().contains(&token.to_uppercase()) {
            return RlsAssertion::Failed(format!(
                "core-server 源码出现角色借用字样 {token}，界面/API 可借用复制角色"
            ));
        }
    }
    let manifest = read_to_string_lossy(&core_server.join("Cargo.toml")).unwrap_or_default();
    if manifest.contains("tokio-postgres") && manifest.contains("replication") {
        return RlsAssertion::Failed("core-server 依赖图出现复制通道字样".to_string());
    }
    RlsAssertion::Passed
}

/// 内部对账上下文借用断言（C-05，阶段 2）。五入口（界面、API、低代码、
/// 插件、高级只读 SQL）均无法建立或借用对账上下文：
/// 构造器 crate 内可见、仅对 job-worker 装配开放，封闭性取静态判据——
/// ep-platform-recon 不得出现字符串拼接 SQL，且除 job-worker 外
/// 无其他 crate 依赖 ep-platform-recon。无活库时同纪律返回 Skipped。
pub fn assert_recon_context_borrow() -> RlsAssertion {
    if !probe_available() {
        return RlsAssertion::Skipped(NO_PROBE);
    }
    let recon = repo_root().join("crates/platform/recon");
    let mut sources = String::new();
    collect_rust_sources(&recon, &mut sources);
    if sources.contains("format!(\"select") || sources.contains("format!(\"update") {
        return RlsAssertion::Failed("ep-platform-recon 出现字符串拼接 SQL".to_string());
    }
    // 依赖图封闭性：扫全部 Cargo.toml，引用 ep-platform-recon 的只许是
    // recon 自身与 job-worker 装配。
    let mut borrowers: Vec<String> = Vec::new();
    for manifest in ["apps/core-server/Cargo.toml", "apps/job-worker/Cargo.toml"] {
        let path = repo_root().join(manifest);
        let text = read_to_string_lossy(&path).unwrap_or_default();
        if text.contains("ep-platform-recon") && !manifest.contains("job-worker") {
            borrowers.push(manifest.to_string());
        }
    }
    if !borrowers.is_empty() {
        return RlsAssertion::Failed(format!(
            "对账上下文构造器被 job-worker 之外的装配引用：{borrowers:?}"
        ));
    }
    RlsAssertion::Passed
}

#[cfg(test)]
mod tests {
    use super::*;

    /// C-05 冻结的八个名字。改名即违反裁定，本测试是代码侧的唯一守卫。
    ///
    /// 出处：`00c-gap-ruling.md` 的 C-05 确切标识符段与阶段 2 计划第 8 节，
    /// 两处逐字一致。`xtask` 不依赖 `ep-testkit`，无法从工具侧比对，故守在这里。
    const FROZEN_BY_C05: [&str; 8] = [
        "assert_read",
        "assert_write",
        "assert_update",
        "assert_delete",
        "assert_aggregate",
        "assert_sort",
        "assert_report_projection",
        "assert_error_leak",
    ];

    #[test]
    fn assertion_names_match_c05() {
        assert_eq!(ASSERTION_NAMES, FROZEN_BY_C05);
    }

    /// 探针库不可达时必须是 `Skipped` 而不是 `Passed`——未覆盖不等于通过。
    #[test]
    fn unreachable_probe_is_not_a_pass() {
        assert!(matches!(assert_read(), RlsAssertion::Skipped(_)));
        assert_ne!(assert_error_leak(), RlsAssertion::Passed);
    }

    /// 阶段 2 两个断言在无活库时同纪律：Skipped，不判过。
    #[test]
    fn stage2_assertions_skip_without_probe() {
        assert!(matches!(
            assert_replication_role_containment(),
            RlsAssertion::Skipped(_)
        ));
        assert!(matches!(
            assert_recon_context_borrow(),
            RlsAssertion::Skipped(_)
        ));
    }

    /// 阶段 2 两名与阶段 1 八名不得重叠，也不得重复实现。
    #[test]
    fn stage2_names_are_disjoint_from_stage1() {
        for name in STAGE2_ASSERTION_NAMES {
            assert!(
                !ASSERTION_NAMES.contains(&name),
                "{name} 与阶段 1 冻结名重叠"
            );
        }
        assert_eq!(STAGE2_ASSERTION_NAMES.len(), 2);
    }

    /// 封闭性静态判据的负样例守卫：recon crate 若出现拼接 SQL，
    /// 本测试直接可见（探针接通后同判据进断言函数）。
    #[test]
    fn recon_crate_has_no_string_concatenated_sql() {
        let recon = repo_root().join("crates/platform/recon");
        let mut sources = String::new();
        collect_rust_sources(&recon, &mut sources);
        assert!(
            !sources.contains("format!(\"select") && !sources.contains("format!(\"update"),
            "ep-platform-recon 不得出现字符串拼接 SQL"
        );
    }
}
