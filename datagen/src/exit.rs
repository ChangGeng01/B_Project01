//! 退出码域。
//!
//! 单列一个模块并禁止在别处写字面量，理由与 `ep-migrate` 的 `exit.rs` 同：
//! 退出码是脚本唯一能可靠读到的判定位，散落在各处的字面量必然漂移。
//!
//! 取值向仓库既有约定看齐，不另立一套：`xtask` 用 2 表参数错误、70 表本阶段未交付，
//! 65 与 74 取 sysexits 的 EX_DATAERR 与 EX_IOERR。没有一个取值表示「跑了但没产出」——
//! 生成器要么产出完整样本档并返回 0，要么以非零码说明是哪一道拦下的。

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Exit {
    /// 样本档已完整产出。
    Ok,
    /// 参数错误：调用方改命令行即可。
    Usage,
    /// 产出物不满足档位声明的形状，或含不可编码的取值。生成器自身的缺陷。
    ShapeViolation,
    /// 已登记但本阶段未交付：改命令行没用，要等交付。
    NotDelivered,
    /// 写出目标失败。
    IoError,
}

impl Exit {
    pub const ALL: [Exit; 5] = [
        Exit::Ok,
        Exit::Usage,
        Exit::ShapeViolation,
        Exit::NotDelivered,
        Exit::IoError,
    ];

    pub const fn code(self) -> u8 {
        match self {
            Exit::Ok => 0,
            Exit::Usage => 2,
            Exit::ShapeViolation => 65,
            Exit::NotDelivered => 70,
            Exit::IoError => 74,
        }
    }

    pub const fn label(self) -> &'static str {
        match self {
            Exit::Ok => "成功",
            Exit::Usage => "参数错误",
            Exit::ShapeViolation => "产出物形状不符",
            Exit::NotDelivered => "本阶段未交付",
            Exit::IoError => "写出失败",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 退出码必须两两不同，否则脚本无法凭它区分成因。
    #[test]
    fn codes_are_pairwise_distinct() {
        let mut codes: Vec<u8> = Exit::ALL.iter().map(|e| e.code()).collect();
        codes.sort_unstable();
        codes.dedup();
        assert_eq!(codes.len(), Exit::ALL.len());
    }

    /// 负样例方向：只有 `Ok` 可以是 0，任何失败态取 0 都会被这条拦下。
    #[test]
    fn only_ok_is_zero() {
        for e in Exit::ALL {
            assert_eq!(e.code() == 0, e == Exit::Ok, "{e:?} 的退出码取了 0");
        }
    }
}
