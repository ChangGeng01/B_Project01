//! 受限运行下哪些动作停、哪些照常。
//!
//! # 为什么是三档而不是黑名单或白名单
//!
//! 规格第 3.4 章给了两张清单：
//! 停止的四项——「业务写入、审批、集成出站和自动化任务停止」；
//! 保持可用的五项——「查询、报表、审计追溯、备份和数据导出保持可用」。
//!
//! **两张清单加起来不封闭。** 签名续期与撤销文件的导入、许可状态查询本身、
//! 配置发布、模块的安装启用停用升级、离线升级包导入、用户登录——一项都没归类。
//!
//! 于是两种常见写法都是错的，而且是本卷反复在清的那两类错：
//!
//! **纯黑名单**（不在停止清单里即放行）是一条**恒真放行**的判据——
//! 它对任何没想到的动作都答「可以」，而这正是「恒真判据等于没有判据」。
//!
//! **纯白名单**（不在可用清单里即停止）会把签名续期文件的导入也拦掉。
//! 而规格逐字「续期通过签名续期文件完成……导入成功后状态立即回到生效」——
//! 受限运行下若不能导入续期文件，**受限运行就是不可逆的**，
//! 一家公司会因为一份没能导进去的续期文件永久停在半停摆状态。
//!
//! 两个方向都错，所以第三档是唯一诚实的答案：**归不了类的就说归不了类**，
//! 由调用方显式处理，不替它选一边。这与 crate 内别处的「未覆盖 ≠ 通过」是同一条纪律。

/// 受限运行下一次动作的处置。
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum RestrictedVerdict {
    /// 规格点名停止的。
    Blocked,
    /// 规格点名保持可用的，含两组豁免。
    Allowed,
    /// **规格两张清单都没归类的。**
    /// 既不放行也不静默拒——调用方必须显式处理，不得按任一恒真规则补齐。
    Unclassified,
}

/// 受限运行下会被问到的动作。
///
/// 逐个变体的归类依据写在变体文档上。**新增变体默认落到 `Unclassified`**，
/// 这一点由 [`restricted_run_verdict`] 的兜底分支保证，不靠加变体的人记得。
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum RestrictedOp {
    // ── 规格第 3.4 章点名停止的四项 ──
    /// 业务写入。
    BusinessWrite,
    /// 审批。
    Approval,
    /// 集成出站。
    IntegrationOutbound,
    /// 自动化任务。
    AutomationTask,

    // ── 规格第 3.4 章点名保持可用的五项 ──
    /// 查询。
    Query,
    /// 报表。
    Report,
    /// 审计追溯。
    AuditTrace,
    /// 备份。
    Backup,
    /// 数据导出。规格另有一条独立加固：
    /// 「任何许可状态下……数据可携带能力不受限制，包括受限运行状态与许可已撤销状态。」
    DataExport,

    // ── 第一组豁免：规格第 12.4 章的记录删除与更正处置（规格逐字「不可停用」）──
    /// 删除事件向派生存储与归档层的传播。
    DeletionPropagation,
    /// 更正事件的传播。
    CorrectionPropagation,
    /// 处置清单生成。
    DisposalListGeneration,
    /// 到期销毁的复核与双人审批。
    ///
    /// **注意它与 [`RestrictedOp::Approval`] 分开**：规格停止的是业务审批，
    /// 而这一条逐字「其审批链路、双人复核与不可篡改审计一并保持可用」。
    DestructionDualApproval,
    /// 可验证销毁证明输出。
    DestructionProof,

    // ── 第二组豁免：规格第 12.1 章的身份与访问安全处置（规格逐字「任何许可状态下」）──
    /// 账号停用。
    AccountDeactivation,
    /// 口令重置。
    PasswordReset,
    /// 凭据轮换。
    CredentialRotation,
    /// 权限回收。
    PermissionRevocation,

    // ── 规格两张清单都没归类的，逐项列名以便被断言 ──
    /// 签名续期文件的导入。**这一项是纯白名单写法的反例**：
    /// 拦掉它，受限运行就不可逆。
    RenewalFileImport,
    /// 签名撤销文件的导入。
    RevocationFileImport,
    /// 许可状态查询本身。
    LicenseStatusQuery,
    /// 配置发布。
    ConfigRelease,
    /// 模块的安装、启用、停用与升级。
    ModuleLifecycleAction,
    /// 离线升级包与安全补丁导入。
    OfflineUpgradeImport,
    /// 用户登录本身。
    Login,
}

/// 规格点名**停止**的四项。单列成常量是为了让「停几项」可断言。
pub const BLOCKED_OPS: [RestrictedOp; 4] = [
    RestrictedOp::BusinessWrite,
    RestrictedOp::Approval,
    RestrictedOp::IntegrationOutbound,
    RestrictedOp::AutomationTask,
];

/// 规格点名**保持可用**的五项，加两组豁免共九项。
pub const ALLOWED_OPS: [RestrictedOp; 14] = [
    // 第 3.4 章五项
    RestrictedOp::Query,
    RestrictedOp::Report,
    RestrictedOp::AuditTrace,
    RestrictedOp::Backup,
    RestrictedOp::DataExport,
    // 第 12.4 章处置五项
    RestrictedOp::DeletionPropagation,
    RestrictedOp::CorrectionPropagation,
    RestrictedOp::DisposalListGeneration,
    RestrictedOp::DestructionDualApproval,
    RestrictedOp::DestructionProof,
    // 第 12.1 章身份四项
    RestrictedOp::AccountDeactivation,
    RestrictedOp::PasswordReset,
    RestrictedOp::CredentialRotation,
    RestrictedOp::PermissionRevocation,
];

/// 判定受限运行下一次动作该怎么处置。
///
/// 只在 [`crate::in_restricted_run`] 为真时才需要问这个函数；
/// 非受限运行下一切照常，不必逐项判。
///
/// # 第一组豁免有反向边界，第二组没有——这个不对称是规格自己定的
///
/// 规格对第一组逐字补了一句：「上述删除、更正与销毁处置产生的写入不计为业务写入，
/// 不受许可状态限制；**其操作范围限定为上述处置事项，
/// 不得用于常规业务写入、业务审批或集成出站**。」
/// 这条反向边界在本函数里的落法是：那三项照旧 [`RestrictedVerdict::Blocked`]。
/// 换句话说**豁免挂在动作上，不挂在调用方身上**——
/// 一次常规业务写入不会因为发起它的是处置流程就变成 `Allowed`，
/// 调用方也不得把一次常规写入贴上处置动作的标签绕过去。
///
/// 规格对第二组**没有**写任何反向边界句，范围词还更宽（「任何许可状态下」）。
/// 本实现不替它补一条对称的限制：那会是自造规格，而且判紧的代价是
/// 安全事件发生时没法回收一个被盗账号的权限——这是本模块最贵的一侧，且不可事后补救。
pub fn restricted_run_verdict(op: RestrictedOp) -> RestrictedVerdict {
    if BLOCKED_OPS.contains(&op) {
        return RestrictedVerdict::Blocked;
    }
    if ALLOWED_OPS.contains(&op) {
        return RestrictedVerdict::Allowed;
    }
    RestrictedVerdict::Unclassified
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_four_blocked_ops_match_the_spec() {
        assert_eq!(BLOCKED_OPS.len(), 4, "改这张表必须先改规格第 3.4 章");
        for op in BLOCKED_OPS {
            assert_eq!(restricted_run_verdict(op), RestrictedVerdict::Blocked);
        }
    }

    #[test]
    fn the_allowed_ops_are_five_plus_two_exemption_groups() {
        assert_eq!(
            ALLOWED_OPS.len(),
            14,
            "第 3.4 章五项加第 12.4 章五项加第 12.1 章四项"
        );
        for op in ALLOWED_OPS {
            assert_eq!(restricted_run_verdict(op), RestrictedVerdict::Allowed);
        }
    }

    /// 两张清单不得相交——一个动作不能既停又通。
    #[test]
    fn the_two_lists_do_not_overlap() {
        for b in BLOCKED_OPS {
            assert!(!ALLOWED_OPS.contains(&b), "{b:?} 同时在两张清单里");
        }
    }

    /// 本模块的要害之一：**纯黑名单会在这里恒真放行**。
    /// 这几项规格两张清单都没归类，必须报归不了类，
    /// 而不是因为「不在停止清单里」就答可以。
    #[test]
    fn unclassified_ops_are_neither_allowed_nor_blocked() {
        for op in [
            RestrictedOp::RenewalFileImport,
            RestrictedOp::RevocationFileImport,
            RestrictedOp::LicenseStatusQuery,
            RestrictedOp::ConfigRelease,
            RestrictedOp::ModuleLifecycleAction,
            RestrictedOp::OfflineUpgradeImport,
            RestrictedOp::Login,
        ] {
            assert_eq!(
                restricted_run_verdict(op),
                RestrictedVerdict::Unclassified,
                "{op:?} 规格两张清单都没归类，不得替它选一边"
            );
        }
    }

    /// 本模块的要害之二：**纯白名单会拦掉续期文件导入**，
    /// 而规格逐字「导入成功后状态立即回到生效」——拦掉它，受限运行就不可逆。
    /// 这一条与上一条不是重复：上一条判形态，这一条点名那个会致命的具体项。
    #[test]
    fn a_pure_whitelist_would_make_restricted_run_irreversible() {
        assert_ne!(
            restricted_run_verdict(RestrictedOp::RenewalFileImport),
            RestrictedVerdict::Blocked,
            "拦掉续期文件导入，受限运行就没有出口了"
        );
    }

    /// 第一组豁免的反向边界：规格逐字「不得用于常规业务写入、业务审批或集成出站」。
    /// 豁免挂在动作上不挂在调用方身上——处置流程发起的一次常规业务写入照样停。
    #[test]
    fn the_disposal_exemption_does_not_leak_into_ordinary_writes() {
        for op in [
            RestrictedOp::BusinessWrite,
            RestrictedOp::Approval,
            RestrictedOp::IntegrationOutbound,
        ] {
            assert_eq!(
                restricted_run_verdict(op),
                RestrictedVerdict::Blocked,
                "{op:?} 是规格为第一组豁免明写的三条反向边界之一"
            );
        }
        // 而处置自己的双人审批照常——它与业务审批是两回事。
        assert_eq!(
            restricted_run_verdict(RestrictedOp::DestructionDualApproval),
            RestrictedVerdict::Allowed
        );
    }

    /// 第二组豁免取宽侧：身份四项在受限运行下一律照常。
    /// 判紧的代价是安全事件发生时回收不了一个被盗账号的权限——
    /// 这是本模块最贵的一侧，且不可事后补救。
    #[test]
    fn the_four_identity_actions_stay_available() {
        for op in [
            RestrictedOp::AccountDeactivation,
            RestrictedOp::PasswordReset,
            RestrictedOp::CredentialRotation,
            RestrictedOp::PermissionRevocation,
        ] {
            assert_eq!(restricted_run_verdict(op), RestrictedVerdict::Allowed);
        }
    }

    /// 数据导出在任何状态下都通——规格另有一条独立加固，
    /// 逐字「包括受限运行状态与许可已撤销状态」。
    #[test]
    fn data_export_is_never_blocked() {
        assert_eq!(
            restricted_run_verdict(RestrictedOp::DataExport),
            RestrictedVerdict::Allowed
        );
    }
}
