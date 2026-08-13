//! 退出码域。按裁定 C-02 与阶段 2 计划第 3.3 节固定为六个取值，不多不少。
//!
//! 之所以把退出码单列一个模块并禁止在别处写字面量，是因为退出码是运维脚本
//! 与 systemd oneshot 单元唯一能读到的判据；一旦某处偷偷多出一个第七个码，
//! 上游脚本会把它当成未知失败或者更糟——当成成功。

/// `ep-migrate` 的全部退出码。取值集合冻结，任何阶段不得增删。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MigrateExit {
    /// 0，被请求的动作已完成。
    Success,
    /// 2，参数错误。子命令未知、选项未知、取值形态不合法、必填项缺失。
    UsageError,
    /// 3，迁移窗口未打开。调用方没有出示一个已打开的迁移窗口。
    MigrationWindowClosed,
    /// 4，校验和不符。迁移清单实算哈希与调用方出示的期望值不等。
    ChecksumMismatch,
    /// 5，版本不一致。调用方出示的期望版本与本制品自身的版本不等。
    VersionMismatch,
    /// 78，环境自检失败。本二进制在这台机器上无法执行被请求的动作。
    EnvSelfCheckFailed,
}

impl MigrateExit {
    /// 六个取值的全集。用例逐个覆盖它，缺一个即测试不通过。
    pub const ALL: [MigrateExit; 6] = [
        MigrateExit::Success,
        MigrateExit::UsageError,
        MigrateExit::MigrationWindowClosed,
        MigrateExit::ChecksumMismatch,
        MigrateExit::VersionMismatch,
        MigrateExit::EnvSelfCheckFailed,
    ];

    /// 进程退出码。
    pub const fn code(self) -> u8 {
        match self {
            MigrateExit::Success => 0,
            MigrateExit::UsageError => 2,
            MigrateExit::MigrationWindowClosed => 3,
            MigrateExit::ChecksumMismatch => 4,
            MigrateExit::VersionMismatch => 5,
            MigrateExit::EnvSelfCheckFailed => 78,
        }
    }

    /// 计划原文给这个码的名字。用在用法文本与错误说明里，避免各处自造措辞。
    pub const fn label(self) -> &'static str {
        match self {
            MigrateExit::Success => "成功",
            MigrateExit::UsageError => "参数错误",
            MigrateExit::MigrationWindowClosed => "迁移窗口未打开",
            MigrateExit::ChecksumMismatch => "校验和不符",
            MigrateExit::VersionMismatch => "版本不一致",
            MigrateExit::EnvSelfCheckFailed => "环境自检失败",
        }
    }
}

/// 「本阶段未交付」落在哪个退出码。
///
/// 计划把退出码域写死为六个，其中没有一个专门表达「实现体尚未交付」。既不许
/// 发明第七个码，也不许静默返回 0，只能在六个里挑一个最保守的落法。挑 78 的
/// 理由是：3、4、5 三个码各自断言一项确定的库侧事实，误报会把运维引向错误的
/// 补救动作（去开窗、去查校验和、去回滚版本）；2 会告诉运维「你的参数写错了」，
/// 而参数其实完全正确，运维会反复改参数而永远改不对。78 的语义是本二进制在这
/// 台机器上不具备执行该动作的前提，这正是实现体缺席时的真实情形，因此本阶段
/// 把「子命令实现体是否存在」登记成一个环境自检项 `subcommand-implemented`，
/// 由它不通过而落 78，不是把 78 当兜底桶用。
/// 把「子命令实现体是否存在」登记成环境自检项的历史登记常量；实现体补齐后
/// 生产路径不再引用，仅由单测锁死「未交付不得映射为成功」的判据。
#[cfg(test)]
pub const NOT_DELIVERED: MigrateExit = MigrateExit::EnvSelfCheckFailed;

/// 一次调用的判定结果。要么完成并留下一段给 stdout 的正文，要么带着一个非零
/// 退出码与一段给 stderr 的说明失败。没有第三种形态，也就没有「悄悄返回 0」。
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Outcome {
    Done(String),
    Failed(MigrateExit, String),
}

impl Outcome {
    pub fn exit(&self) -> MigrateExit {
        match self {
            Outcome::Done(_) => MigrateExit::Success,
            Outcome::Failed(e, _) => *e,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exit_codes_match_plan_verbatim() {
        let codes: Vec<u8> = MigrateExit::ALL.iter().map(|e| e.code()).collect();
        assert_eq!(codes, vec![0, 2, 3, 4, 5, 78]);
    }

    #[test]
    fn exit_codes_are_distinct_and_exactly_six() {
        let mut codes: Vec<u8> = MigrateExit::ALL.iter().map(|e| e.code()).collect();
        codes.sort_unstable();
        codes.dedup();
        assert_eq!(codes.len(), 6, "退出码域必须恰好六个互不相同的取值");
    }

    #[test]
    fn not_delivered_never_maps_to_success() {
        assert_ne!(NOT_DELIVERED, MigrateExit::Success);
        assert_eq!(NOT_DELIVERED.code(), 78);
    }
}
