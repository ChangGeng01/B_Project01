//! RLS 断言矩阵。八个断言函数的函数名与 C-05 逐字一致。
//!
//! 阶段 1 只交付这八个，不实现阶段 2 与阶段 4 的追加函数。
//! 函数体在探针表上生效，探针表由阶段 2 的引导脚本建立；
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

        /// 八个断言函数名，供 `xtask` 逐字比对 C-05。
        pub const ASSERTION_NAMES: [&str; 8] = [$(stringify!($name)),*];
    };
}

rls_assertions!(
    assert_rls_enabled_and_forced,
    assert_cross_entity_select_denied,
    assert_cross_entity_insert_denied,
    assert_cross_entity_update_denied,
    assert_runtime_role_cannot_delete,
    assert_runtime_role_cannot_ddl,
    assert_readonly_role_cannot_write,
    assert_session_vars_cleared_on_return,
);
