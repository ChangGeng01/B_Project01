//! 在线变更边界与其上限判定。
//!
//! 规格第 7.4 章逐字：「新增表、新增可空列、新增索引和放宽长度**可在线执行**；
//! 改变列类型、收紧非空约束和重建主键**需要停机窗口**。
//! 单次在线变更的锁持有时间上限为 5 秒，迁移执行时长上限为 30 分钟。」
//!
//! # 判错方向的代价不对称，本模块因此取保守侧
//!
//! 把「需停机」误判成「可在线」，会在生产高峰锁住一张业务表——
//! 而客户是二三十人的小公司，一张表锁住五分钟就是全公司停工。
//! 反过来把「可在线」误判成「需停机」，只是多要一个维护窗口。
//!
//! 因此 [`classify`] 用的是**白名单**：只有明确列在可在线清单里的操作才判可在线，
//! **其余一律判需停机窗口**，包括本模块还不认识的新操作类型。
//! 这条在 [`classify`] 的 `_` 分支上，不是靠调用方记得。

use std::time::Duration;

/// 单次在线变更的锁持有上限。规格第 7.4 章逐字 5 秒。
///
/// 与基线第 3.9 节的迁移会话参数 `lock_timeout = '5s'`、
/// 阶段 13 计划的 `ddl_plans.lock_timeout_ms` 默认 5000 是同一个数。
pub const MAX_LOCK_HOLD: Duration = Duration::from_secs(5);
/// 迁移执行时长上限。规格第 7.4 章逐字 30 分钟。
///
/// 同上，对应 `statement_timeout = '30min'` 与 `statement_timeout_ms` 默认 1800000。
pub const MAX_MIGRATION_DURATION: Duration = Duration::from_secs(30 * 60);
/// 单次停机窗口的上限。规格第 7.4 章逐字 15 分钟：
/// 「单次不超过 15 分钟，超出单次上限的变更拆分为多次排期或改用影子表加切换方案。」
///
/// **注意这个数比 [`MAX_MIGRATION_DURATION`] 小一半。** 直觉上停机窗口该更宽松——
/// 毕竟没人在用——但规格取的是反过来的：在线变更超时只是回退，
/// 停机窗口超时是全公司多停那么久。判成「转停机窗口就有 30 分钟可用」是一处实错。
pub const MAX_MAINTENANCE_WINDOW: Duration = Duration::from_secs(15 * 60);

/// 一次结构变更的操作类型。
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum DdlOperation {
    /// 新增表。
    CreateTable,
    /// 新增可空列。
    AddNullableColumn,
    /// 新增索引。
    CreateIndex,
    /// 放宽长度（例如 varchar(50) 改 varchar(100)）。
    WidenLength,
    /// 改变列类型。
    ChangeColumnType,
    /// 收紧非空约束。
    TightenNotNull,
    /// 重建主键。
    RebuildPrimaryKey,
    /// 新增**非空**列。**规格的可在线清单里只有「新增可空列」**，
    /// 非空列不在其中——加一个非空列要么带默认值重写全表，要么先加可空再收紧，
    /// 两条都不在可在线范围内。单列出来是为了不让它被误读进 `AddNullableColumn`。
    AddNotNullColumn,
    /// 收窄长度。规格只说「放宽长度」可在线，收窄不在其中——
    /// 收窄要校验存量数据，不是纯元数据变更。
    NarrowLength,
    /// 删除列。规格的两张清单都没提，按白名单落到需停机窗口。
    DropColumn,
}

impl DdlOperation {
    pub fn as_db_value(self) -> &'static str {
        match self {
            DdlOperation::CreateTable => "CREATE_TABLE",
            DdlOperation::AddNullableColumn => "ADD_NULLABLE_COLUMN",
            DdlOperation::CreateIndex => "CREATE_INDEX",
            DdlOperation::WidenLength => "WIDEN_LENGTH",
            DdlOperation::ChangeColumnType => "CHANGE_COLUMN_TYPE",
            DdlOperation::TightenNotNull => "TIGHTEN_NOT_NULL",
            DdlOperation::RebuildPrimaryKey => "REBUILD_PRIMARY_KEY",
            DdlOperation::AddNotNullColumn => "ADD_NOT_NULL_COLUMN",
            DdlOperation::NarrowLength => "NARROW_LENGTH",
            DdlOperation::DropColumn => "DROP_COLUMN",
        }
    }
}

/// 变更的执行方式。
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum ExecutionMode {
    /// 可在线执行。
    Online,
    /// 需要停机窗口。
    MaintenanceWindow,
}

/// 规格逐字列出的可在线操作。**只有这四个。**
///
/// 单列成常量而不是写进 `match`，是为了让「可在线的到底有几个」可断言——
/// 有人往里加一个，用例会红。
pub const ONLINE_OPERATIONS: [DdlOperation; 4] = [
    DdlOperation::CreateTable,
    DdlOperation::AddNullableColumn,
    DdlOperation::CreateIndex,
    DdlOperation::WidenLength,
];

/// 无论如何都必须保住在线能力的两个操作。规格第 7.4 章「最低在线变更能力」逐字：
/// 「新增可空列和新增索引**必须保留**为在线变更能力……无法满足该底线时
/// **不得以「在线 DDL」能力通过认证**，交付说明必须明确降级为停机窗口变更。」
///
/// 这两个与另外两个（新增表、放宽长度）的地位不同：后两个若实测超限，
/// 按认证期口径登记进停机窗口清单即可、不判认证失败；**这两个不行**，
/// 它们掉出在线范围就意味着「在线 DDL」这项能力整个不成立。
/// 单列成常量是为了让这条底线有一个可断言的被测对象，而不是散文里的一句话。
pub const MINIMUM_ONLINE_CAPABILITY: [DdlOperation; 2] =
    [DdlOperation::AddNullableColumn, DdlOperation::CreateIndex];

/// 判定一次变更该怎么执行。
///
/// **白名单**：只有 [`ONLINE_OPERATIONS`] 里的判可在线，其余一律需停机窗口。
pub fn classify(op: DdlOperation) -> ExecutionMode {
    if ONLINE_OPERATIONS.contains(&op) {
        ExecutionMode::Online
    } else {
        ExecutionMode::MaintenanceWindow
    }
}

/// 一次在线变更实测之后的处置。
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum OnlineOutcome {
    /// 在上限内完成。
    Completed,
    /// 超限。**自动回退并转入停机窗口**，回退原因、操作对象与耗时记入审计。
    ///
    /// 规格第 7.4 章逐字区分了两个口径：认证期超限要把该操作登记进停机窗口清单、
    /// 此后不属于在线变更范围；**运行期超限自动回退并转停机窗口，
    /// 不判定为认证失败**。本枚举承载的是运行期口径。
    RolledBackToWindow { reason: LimitExceeded },
}

/// 超了哪一项。**分开报**：锁持有超限与执行时长超限的成因不同——
/// 前者通常是并发争用，后者通常是数据量。运维的下一步动作不一样。
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum LimitExceeded {
    LockHold { held: Duration, limit: Duration },
    Duration { elapsed: Duration, limit: Duration },
}

/// 判定一次在线变更的实测结果。
///
/// 先判锁持有再判总时长：锁持有是对**别人**的影响，总时长只是对自己的。
/// 两项同时超时先报锁持有，因为那是需要立刻处理的那一个。
pub fn judge_online_run(lock_held: Duration, elapsed: Duration) -> OnlineOutcome {
    if lock_held > MAX_LOCK_HOLD {
        return OnlineOutcome::RolledBackToWindow {
            reason: LimitExceeded::LockHold {
                held: lock_held,
                limit: MAX_LOCK_HOLD,
            },
        };
    }
    if elapsed > MAX_MIGRATION_DURATION {
        return OnlineOutcome::RolledBackToWindow {
            reason: LimitExceeded::Duration {
                elapsed,
                limit: MAX_MIGRATION_DURATION,
            },
        };
    }
    OnlineOutcome::Completed
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 规格逐字给的四个可在线操作，一个不多一个不少。
    #[test]
    fn exactly_four_operations_are_online() {
        assert_eq!(ONLINE_OPERATIONS.len(), 4, "改这张表必须先改规格第 7.4 章");
        for op in ONLINE_OPERATIONS {
            assert_eq!(classify(op), ExecutionMode::Online, "{op:?} 应可在线");
        }
    }

    /// 规格逐字给的三个需停机操作。
    #[test]
    fn the_three_named_offline_operations() {
        for op in [
            DdlOperation::ChangeColumnType,
            DdlOperation::TightenNotNull,
            DdlOperation::RebuildPrimaryKey,
        ] {
            assert_eq!(classify(op), ExecutionMode::MaintenanceWindow, "{op:?}");
        }
    }

    /// 白名单的要害在这里：**规格没点名的操作一律落到需停机窗口**。
    /// 判错方向的代价不对称——误判成可在线会在生产高峰锁住业务表，
    /// 而客户是二三十人的小公司，一张表锁住五分钟就是全公司停工。
    #[test]
    fn operations_the_spec_never_named_default_to_offline() {
        for op in [
            DdlOperation::AddNotNullColumn,
            DdlOperation::NarrowLength,
            DdlOperation::DropColumn,
        ] {
            assert_eq!(
                classify(op),
                ExecutionMode::MaintenanceWindow,
                "{op:?} 不在规格的可在线清单里，必须落到停机窗口"
            );
        }
    }

    /// 新增可空列可在线，新增非空列不行——两者只差一个词，后果差很远。
    #[test]
    fn nullable_and_not_null_columns_are_not_the_same_operation() {
        assert_eq!(
            classify(DdlOperation::AddNullableColumn),
            ExecutionMode::Online
        );
        assert_eq!(
            classify(DdlOperation::AddNotNullColumn),
            ExecutionMode::MaintenanceWindow
        );
    }

    /// 放宽长度可在线，收窄不行——收窄要校验存量数据，不是纯元数据变更。
    #[test]
    fn widening_is_online_but_narrowing_is_not() {
        assert_eq!(classify(DdlOperation::WidenLength), ExecutionMode::Online);
        assert_eq!(
            classify(DdlOperation::NarrowLength),
            ExecutionMode::MaintenanceWindow
        );
    }

    #[test]
    fn limits_match_the_spec_verbatim() {
        assert_eq!(MAX_LOCK_HOLD.as_secs(), 5);
        assert_eq!(MAX_MIGRATION_DURATION.as_secs(), 1800);
        assert_eq!(MAX_MAINTENANCE_WINDOW.as_secs(), 900);
    }

    /// 停机窗口的单次上限**比在线变更的执行上限更严**，不是更松。
    /// 直觉上停机窗口该更宽松——没人在用——但规格取的是反过来的：
    /// 在线超时只是回退，停机超时是全公司多停那么久。
    /// 判成「转停机窗口就有 30 分钟可用」是一处实错。
    #[test]
    fn the_maintenance_window_is_the_tighter_limit() {
        assert!(
            MAX_MAINTENANCE_WINDOW < MAX_MIGRATION_DURATION,
            "规格第 7.4 章：停机窗口单次不超过 15 分钟"
        );
    }

    /// 最低在线变更能力的两项必须落在可在线清单里。
    /// 这条不是重复 `exactly_four_operations_are_online`——那条判的是「有哪四个」，
    /// 这条判的是「这两个掉出去就不只是少一个操作，是「在线 DDL」整项能力不成立」。
    #[test]
    fn the_two_minimum_capabilities_are_online() {
        for op in MINIMUM_ONLINE_CAPABILITY {
            assert_eq!(
                classify(op),
                ExecutionMode::Online,
                "{op:?} 是规格第 7.4 章的最低在线变更能力，掉出在线范围即不得以「在线 DDL」通过认证"
            );
            assert!(ONLINE_OPERATIONS.contains(&op));
        }
    }

    /// 边界：恰好等于上限不算超。上限是「不得超过」。
    #[test]
    fn exactly_at_the_limit_is_not_exceeded() {
        assert_eq!(
            judge_online_run(MAX_LOCK_HOLD, MAX_MIGRATION_DURATION),
            OnlineOutcome::Completed
        );
    }

    #[test]
    fn each_limit_is_reported_with_its_numbers() {
        let over_lock = judge_online_run(Duration::from_secs(6), Duration::from_secs(1));
        assert_eq!(
            over_lock,
            OnlineOutcome::RolledBackToWindow {
                reason: LimitExceeded::LockHold {
                    held: Duration::from_secs(6),
                    limit: MAX_LOCK_HOLD
                }
            }
        );
        let over_time = judge_online_run(Duration::from_secs(1), Duration::from_secs(1801));
        assert!(matches!(
            over_time,
            OnlineOutcome::RolledBackToWindow {
                reason: LimitExceeded::Duration { .. }
            }
        ));
    }

    /// 两项同时超时先报锁持有：它是对别人的影响，总时长只是对自己的。
    #[test]
    fn lock_hold_is_reported_before_duration() {
        let both = judge_online_run(Duration::from_secs(99), Duration::from_secs(9999));
        assert!(matches!(
            both,
            OnlineOutcome::RolledBackToWindow {
                reason: LimitExceeded::LockHold { .. }
            }
        ));
    }
}
