//! `ModuleLicenseQuery` —— 各阶段取用许可判定的唯一入口。
//!
//! 签名逐字照裁定 A-05 与阶段 3 计划第 3.4.11 节冻结的三个方法。
//!
//! **本轮不提供任何实现体。** 三个方法都要读 `platform_core` 的三张表，
//! 而本轮只做脱库可判定的一半。不留 `todo!()`、不留占位实现——
//! 一个会 panic 的占位实现比没有实现更危险，因为它编译得过、也注册得进去。

use crate::{LicenseStatus, ModuleState};
use ep_foundation::module::ModuleCode;

/// 模块许可判定的取用入口。
///
/// 计划第 3.11.2 节逐字：「各阶段只读该 trait 判定模块状态与功能开关，**不直接读许可表**。」
///
/// # `license_status` 的返回值不足以决定停不停业务写入
///
/// 见 [`crate::status`] 的模块文档：`Expired` 同时盖住规格的宽限期与受限运行两态，
/// 前者「全部功能可用」、后者停业务写入。调用方要判停写，
/// 取的是 [`crate::in_restricted_run`] 而不是本方法的返回值。
pub trait ModuleLicenseQuery: Send + Sync {
    fn module_state(&self, module: ModuleCode) -> ModuleState;
    fn is_feature_enabled(&self, feature_code: &str) -> bool;
    fn license_status(&self) -> LicenseStatus;
}

#[cfg(test)]
mod tests {
    use super::*;

    struct Fixture;

    impl ModuleLicenseQuery for Fixture {
        fn module_state(&self, _module: ModuleCode) -> ModuleState {
            ModuleState::InstalledEnabled
        }
        fn is_feature_enabled(&self, _feature_code: &str) -> bool {
            true
        }
        fn license_status(&self) -> LicenseStatus {
            LicenseStatus::Valid
        }
    }

    /// 签名锁。裁定 A-05 逐字冻结了这三个签名，下面三行把它们钉成函数指针类型：
    /// 任一方法**改名、改参数个数或类型、改返回类型**，这三行就编译不过。
    ///
    /// 不写成 `fn _obj_safe(_: &dyn ModuleLicenseQuery) {}`——那一条是恒真的。
    /// 对象安全只被极窄的一类改动破坏（泛型方法、返回 `Self` 之类），
    /// 改方法名、加一个参数、换返回类型它一概照绿，等于没锁。
    #[test]
    fn the_three_frozen_signatures_still_hold() {
        let _: fn(&Fixture, ModuleCode) -> ModuleState = Fixture::module_state;
        let _: fn(&Fixture, &str) -> bool = Fixture::is_feature_enabled;
        let _: fn(&Fixture) -> LicenseStatus = Fixture::license_status;
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
        assert_eq!(q.license_status(), LicenseStatus::Valid);
    }
}
