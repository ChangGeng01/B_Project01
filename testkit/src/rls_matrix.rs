//! RLS 断言矩阵。八个断言函数的函数名按裁定 C-05 逐字冻结。
//!
//! 阶段 1 只交付这八个，不实现阶段 2 与阶段 4 的追加函数：
//! 阶段 2 追加 `assert_replication_role_containment` 与 `assert_recon_context_borrow`，
//! 阶段 4 追加 `matrix_32.rs` 与发布门禁项 `RG-RLS-MATRIX-GREEN`。
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
}
