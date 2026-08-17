//! `ModuleLicenseQuery` —— 各阶段取用许可判定的唯一入口。
//!
//! 签名逐字照裁定 A-05 与阶段 3 计划第 3.4.11 节冻结的三个方法。
//!
//! **本轮不提供任何实现体。** 三个方法都要读 `platform_core` 的三张表，
//! 而本轮只做脱库可判定的一半。不留 `todo!()`、不留占位实现——
//! 一个会 panic 的占位实现比没有实现更危险，因为它编译得过、也注册得进去。

use crate::ModuleState;
use ep_foundation::module::ModuleCode;

/// 模块许可判定的取用入口。
///
/// 计划第 3.11.2 节逐字：「各阶段只读该 trait 判定模块状态与功能开关，**不直接读许可表**。」
///
/// # 单方法，这是对裁定 A-05 冻结签名的一次改判（裁定 F-18 结论四）
///
/// A-05 冻结的是三个方法。`license_status` 与 `is_feature_enabled` 已由 F-18 撤下，
/// 理由是两者都必须读一张**本部署不建的表**：
/// `license_grants` 与 `feature_flags` 按 F-18 结论三首版不建。
///
/// 留着它们只有两条路，两条都不合规：留一个会 panic 的占位（本卷明禁），
/// 或者返回一个**没有闸门可穿的值**——`Valid` 是「没有许可被判成有许可」这种
/// 不会当场报错的错，`Revoked` 在本部署没有任何消费方、是一个取不到的返回值。
///
/// F-18 据此新立一条判别式：**fail-closed 的前提是存在一道会被真实请求穿过的闸门；
/// 无闸门时唯一合规处置是撤，不是关。**
///
/// `module_state` 留下来是因为它有三个具名消费方：job-worker 的两处过滤点、
/// 阶段 5 的探针判定、阶段 13 的集成测试。对比 `is_feature_enabled` 全仓七处命中
/// 全部是声明、夹具、签名锁与自陈未覆盖——**零个具名调用方**。
pub trait ModuleLicenseQuery: Send + Sync {
    fn module_state(&self, module: ModuleCode) -> ModuleState;
}

#[cfg(test)]
mod tests {
    use super::*;

    struct Fixture;

    impl ModuleLicenseQuery for Fixture {
        fn module_state(&self, _module: ModuleCode) -> ModuleState {
            ModuleState::InstalledEnabled
        }
    }

    /// 签名锁。裁定 F-18 把 trait 收为单方法之后只剩一条，
    /// 但它仍是这里唯一夹具伪装不了的断言——它在编译期。
    ///
    /// 不写成 `fn _obj_safe(_: &dyn ModuleLicenseQuery) {}`——那一条是恒真的。
    /// 对象安全只被极窄的一类改动破坏（泛型方法、返回 `Self` 之类），
    /// 改方法名、加一个参数、换返回类型它一概照绿，等于没锁。
    #[test]
    fn the_frozen_signature_still_holds() {
        let _: fn(&Fixture, ModuleCode) -> ModuleState = Fixture::module_state;
    }

    /// trait 要能做成 trait 对象——apps 侧按计划是注入一个实现，走的是 `dyn`。
    /// 这一条与签名锁是两件事：签名锁守取值，这条守可注入性。
    #[test]
    fn the_trait_is_object_safe_for_injection() {
        let q: &dyn ModuleLicenseQuery = &Fixture;
        assert_eq!(
            q.module_state(ModuleCode::Finance),
            ModuleState::InstalledEnabled
        );
    }
}
