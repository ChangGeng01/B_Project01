//! 模块安装态状态机。四条边逐行照抄阶段 3 计划第 3.4.11 节。

use ep_foundation::error::codes;

/// 模块安装态。三个取值照 `platform_core.module_registrations.install_state`
/// 的 CHECK 逐字（阶段 3 计划表 25）。
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum ModuleState {
    NotInstalled,
    InstalledEnabled,
    InstalledDisabled,
}

impl ModuleState {
    pub fn as_db_value(self) -> &'static str {
        match self {
            ModuleState::NotInstalled => "NOT_INSTALLED",
            ModuleState::InstalledEnabled => "INSTALLED_ENABLED",
            ModuleState::InstalledDisabled => "INSTALLED_DISABLED",
        }
    }

    pub const ALL: [ModuleState; 3] = [
        ModuleState::NotInstalled,
        ModuleState::InstalledEnabled,
        ModuleState::InstalledDisabled,
    ];
}

/// 对模块发起的动作。
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum ModuleAction {
    /// 安装并启用。**计划把「安装」与「启用」合成了这一条边**，见 [`TRANSITIONS`]。
    InstallAndEnable,
    Disable,
    Enable,
    Uninstall,
}

impl ModuleAction {
    pub const ALL: [ModuleAction; 4] = [
        ModuleAction::InstallAndEnable,
        ModuleAction::Disable,
        ModuleAction::Enable,
        ModuleAction::Uninstall,
    ];
}

/// 合法迁移表。计划第 3.4.11 节逐字四条边：
/// 「`NOT_INSTALLED` 到 `INSTALLED_ENABLED`（安装并启用）、
/// `INSTALLED_ENABLED` 到 `INSTALLED_DISABLED`（停用）、
/// `INSTALLED_DISABLED` 到 `INSTALLED_ENABLED`（再启用）、
/// `INSTALLED_DISABLED` 到 `NOT_INSTALLED`（卸载）。」
///
/// 做成数据而不是 `match` 分支，是为了让「表里有几行」本身可断言，
/// 也为了让 [`transition`] 没有地方写兜底——凡查不到即非法，不存在
/// 「不等就算合法」这种恒真放行的写法。
///
/// # 表里没有的两条边，各自的后果
///
/// **没有 `NOT_INSTALLED → INSTALLED_DISABLED`。** 于是「安装但先不启用」
/// 经状态机达不成，只能先启用再停用。若迁移种子把某模块直接种成
/// `INSTALLED_DISABLED`，那一行的状态是状态机产生不出来的状态。
///
/// **没有 `INSTALLED_ENABLED → NOT_INSTALLED`。** 启用态不可直接卸载，
/// 必须先停用。这一条是保护性的：直接卸载会跳过停用那一步的数据保留判定。
pub const TRANSITIONS: [(ModuleState, ModuleAction, ModuleState); 4] = [
    (
        ModuleState::NotInstalled,
        ModuleAction::InstallAndEnable,
        ModuleState::InstalledEnabled,
    ),
    (
        ModuleState::InstalledEnabled,
        ModuleAction::Disable,
        ModuleState::InstalledDisabled,
    ),
    (
        ModuleState::InstalledDisabled,
        ModuleAction::Enable,
        ModuleState::InstalledEnabled,
    ),
    (
        ModuleState::InstalledDisabled,
        ModuleAction::Uninstall,
        ModuleState::NotInstalled,
    ),
];

/// 非法迁移。
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct ModuleTransitionError {
    pub from: ModuleState,
    pub action: ModuleAction,
}

impl ModuleTransitionError {
    /// 错误码。计划第 3.4.11 节只写了分类「非法迁移返回 `BUSINESS_CONFLICT`」、
    /// 没给码名，本轮按 `docs/error-codes.md` 的「先登记后实现」次序补登了
    /// `PLATFORM.MODULE.TRANSITION_INVALID` 再在这里返回它。
    ///
    /// **不复用 `PLATFORM.KEY_DOMAIN.TRANSITION_INVALID`**：那条已登记的语义是
    /// 密钥域状态机，同形不同物，复用它是误用而不是复用。
    pub fn error_code(self) -> &'static str {
        codes::PLATFORM_MODULE_TRANSITION_INVALID.0
    }
}

impl std::fmt::Display for ModuleTransitionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "模块处于 {} 时不能执行 {:?}；合法迁移共 {} 条，见阶段 3 计划第 3.4.11 节",
            self.from.as_db_value(),
            self.action,
            TRANSITIONS.len()
        )
    }
}

impl std::error::Error for ModuleTransitionError {}

/// 走一次迁移。查不到即非法，**没有兜底分支**。
pub fn transition(
    from: ModuleState,
    action: ModuleAction,
) -> Result<ModuleState, ModuleTransitionError> {
    TRANSITIONS
        .iter()
        .find(|(f, a, _)| *f == from && *a == action)
        .map(|(_, _, to)| *to)
        .ok_or(ModuleTransitionError { from, action })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_table_has_exactly_the_four_edges_the_plan_names() {
        assert_eq!(
            TRANSITIONS.len(),
            4,
            "改这张表必须先改阶段 3 计划第 3.4.11 节"
        );
        for (from, action, to) in TRANSITIONS {
            assert_eq!(transition(from, action), Ok(to));
        }
    }

    /// 逐对穷举：12 组 `(状态, 动作)` 里恰好 4 组合法。
    /// 这一条比「四条边都走得通」严——它同时守住**没有第五条边悄悄可走**。
    #[test]
    fn exactly_four_of_the_twelve_pairs_are_legal() {
        let legal = ModuleState::ALL
            .iter()
            .flat_map(|s| ModuleAction::ALL.iter().map(move |a| (*s, *a)))
            .filter(|(s, a)| transition(*s, *a).is_ok())
            .count();
        assert_eq!(legal, 4, "3 状态 × 4 动作 = 12 组，合法的应恰好 4 组");
    }

    /// 启用态不可直接卸载——必须先停用。
    /// 直接卸载会跳过停用那一步的数据保留判定。
    #[test]
    fn an_enabled_module_cannot_be_uninstalled_directly() {
        assert!(transition(ModuleState::InstalledEnabled, ModuleAction::Uninstall).is_err());
    }

    /// 「安装但先不启用」经状态机达不成——表里没有这条边。
    /// 这不是缺陷登记，是把计划的取值钉住：谁要补这条边，得先改计划。
    #[test]
    fn there_is_no_install_without_enable() {
        // NotInstalled 只认 InstallAndEnable 一个动作。
        let from_not_installed: Vec<ModuleAction> = ModuleAction::ALL
            .iter()
            .copied()
            .filter(|a| transition(ModuleState::NotInstalled, *a).is_ok())
            .collect();
        assert_eq!(from_not_installed, vec![ModuleAction::InstallAndEnable]);
    }

    /// 停用再启用要能回到原态——这是计划退出条件第 23 项的形状。
    #[test]
    fn disable_then_enable_round_trips() {
        let disabled =
            transition(ModuleState::InstalledEnabled, ModuleAction::Disable).expect("停用应合法");
        assert_eq!(disabled, ModuleState::InstalledDisabled);
        assert_eq!(
            transition(disabled, ModuleAction::Enable),
            Ok(ModuleState::InstalledEnabled)
        );
    }

    #[test]
    fn db_values_match_the_check_constraint() {
        assert_eq!(ModuleState::ALL.len(), 3);
        assert_eq!(ModuleState::NotInstalled.as_db_value(), "NOT_INSTALLED");
        assert_eq!(
            ModuleState::InstalledEnabled.as_db_value(),
            "INSTALLED_ENABLED"
        );
        assert_eq!(
            ModuleState::InstalledDisabled.as_db_value(),
            "INSTALLED_DISABLED"
        );
        // 三个取值互异——只断长度守不住写重的情形。
        let mut vals: Vec<&str> = ModuleState::ALL.iter().map(|s| s.as_db_value()).collect();
        vals.sort_unstable();
        vals.dedup();
        assert_eq!(vals.len(), 3, "三个 DB 取值应互异");
    }

    /// 非法迁移要报出已登记的码，且不得是密钥域那条同形码。
    #[test]
    fn illegal_transition_carries_the_registered_code() {
        let err = transition(ModuleState::NotInstalled, ModuleAction::Disable)
            .expect_err("未安装不能停用");
        assert_eq!(err.error_code(), "PLATFORM.MODULE.TRANSITION_INVALID");
        assert_ne!(err.error_code(), "PLATFORM.KEY_DOMAIN.TRANSITION_INVALID");
        assert!(
            err.to_string().contains("NOT_INSTALLED"),
            "文案要点出当前状态"
        );
    }
}
