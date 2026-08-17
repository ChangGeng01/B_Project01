//! 订阅许可的状态判定，与「现在是不是受限运行」这个另一个问题。
//!
//! # 一件必须先说清的事：本模块有两个函数而不是一个
//!
//! 计划第 3.4.11 节按裁定 A-05 冻结的枚举是四个变体：
//! `Valid`、`ExpiringSoon`、`Expired`、`Revoked`。
//! 规格第 3.4 章的四态是另外四个：生效、临期告警、**宽限期**、**受限运行**。
//!
//! 两组不是同一组。规格逐字「到期日后进入 **30 天宽限期：全部功能可用**」，
//! 「宽限期结束后进入受限运行：业务写入、审批、集成出站和自动化任务停止」。
//! 而 `Expired` 一个变体同时盖住了这两段——前 30 天全部功能可用，之后业务写入停止，
//! 后果完全相反。
//!
//! 所以 [`classify_subscription`] **不足以回答「该不该停业务写入」**，
//! 那个问题由 [`in_restricted_run`] 单独回答。这不是把一个函数拆成两个，
//! 是本来就是两件事：前者照 A-05 冻结的契约给状态值，后者照规格第 3.4 章给后果。
//!
//! 反过来说，如果只落 [`classify_subscription`] 一个函数，
//! 调用方只有三条路可走，三条都错：
//! 一、见 `Expired` 即停写——比规格早 30 天停掉一家二三十人公司的开单；
//! 二、见 `Expired` 不停写——受限运行这一态在整个系统里永远不发生；
//! 三、调用方自己算 `today - valid_to > 30`——那是阶段 4 计划明禁的
//! 「自建第二套许可判定」。
//! 三条路都错，说明缺的是一个函数，不是缺一个约定。
//!
//! # 本模块不取时钟
//!
//! `today` 是入参。规格逐字要求判定基准是**可信时间**——
//! 「取以下三者中的最晚值：许可证签发日期、平台本地许可状态存储中按日落盘的
//! 最近一次时间戳、最近一次导入的签名续期或撤销文件的签发时间」，
//! 且「本地系统时间早于可信时间时按可信时间判定状态」。
//! 把时钟藏进实现，等于把一条本该防时钟回拨的判据做成可被 `date -s` 绕过的判据。
//!
//! **但本 crate 保证不了传进来的确实是可信时间，而且现状比这更糟**：
//! 三项输入里的第二项（按日落盘的时间戳）在 `platform_core.license_grants` 上
//! 没有对应列，job-worker 的任务清单（计划第 3.2.4 节，十二项，封闭列举）
//! 里也没有按日落盘的任务。这一条登记在 crate 文档的未覆盖段。

use chrono::NaiveDate;

/// 许可状态。**逐字照裁定 A-05 与计划第 3.4.11 节冻结的四个变体**，
/// 不增、不改名、不重排。
///
/// 注意本枚举**没有** `as_db_value`：`license_grants` 表上没有 status 列，
/// 它是由 `revoked_at`、`valid_to` 与可信时间派生出来的量，不是存储量。
/// 给一个派生量配 DB 取值，下一个阶段很容易据此加一列缓存状态，
/// 而缓存下来的状态就是一条可以与事实不符、且不会当场报错的判据。
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum LicenseStatus {
    Valid,
    ExpiringSoon,
    Expired,
    Revoked,
}

impl LicenseStatus {
    pub const ALL: [LicenseStatus; 4] = [
        LicenseStatus::Valid,
        LicenseStatus::ExpiringSoon,
        LicenseStatus::Expired,
        LicenseStatus::Revoked,
    ];
}

/// 判定所需的全部事实。做成结构体而不是三个散参，
/// 是为了让「判定只吃这三样」在类型上可见——模块开关与功能开关**不在其中**，
/// 计划第 3.4.11 节的判定顺序句里也没有它们。
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct LicenseFacts {
    /// 撤销日期。非空即已撤销。
    pub revoked_at: Option<NaiveDate>,
    /// 许可证到期日。库侧 `license_grants.valid_to` 是 `date`。
    pub valid_to: NaiveDate,
    /// 判定基准日。**必须是可信时间**，见模块文档。
    pub today: NaiveDate,
}

/// 临期告警窗口。
///
/// **两处文本给了两个数，这里取计划侧的 30。**
/// 规格第 3.4 章逐字「到期日前 60 天进入临期告警」；
/// 计划第 3.4.11 节逐字「临期窗口取 30 天，属本阶段临时取值，切换只改配置」。
///
/// 取 30 的理由只有一条：计划是本阶段的落码依据，且它自陈临时。
/// **不是因为裁定 A-05 定了 30**——A-05 全文不含任何天数，
/// 别把这个数挂到 A-05 名下。
///
/// 「切换只改配置」这句在本阶段**没有承载物**：本阶段配置项全表里不存在任何
/// `EP__PLATFORM__LICENSE__*` 键，`docs/config-reference.md` 全文 `license` 零命中。
/// 所以它现在是一个 Rust 常量，改判要改这里加两条边界用例。
///
/// 两个数的差别只落在告警提前量上：规格明写临期告警期间「全部功能可用」，
/// 60 与 30 在任何一天都不改变「该不该停业务写入」。
pub const EXPIRING_SOON_WINDOW_DAYS: i64 = 30;

/// 宽限期长度。规格第 3.4 章逐字「到期日后进入 30 天宽限期：全部功能可用」。
///
/// 这个 30 与 [`EXPIRING_SOON_WINDOW_DAYS`] 的 30 **同值不同源**，
/// 一个来自规格一个来自计划，改判时不会一起动。合成一个常量会把两件事绑死。
pub const GRACE_PERIOD_DAYS: i64 = 30;

/// 判定订阅许可的状态。
///
/// 判定顺序照计划第 3.4.11 节逐字：「`revoked_at` 非空为 `Revoked`；
/// `valid_to` 早于当前日期为 `Expired`；距 `valid_to` 不足临期窗口为 `ExpiringSoon`；
/// 否则 `Valid`。」四段**有序**，`Revoked` 短路优先。
///
/// # 名字里的 `subscription` 不是修饰，是范围
///
/// 规格第 3.4 章的节标题逐字是「**订阅**许可生命周期与离线计量」，
/// 开篇第一句逐字「**订阅授权**在完全离线……环境下同样必须可运转」。
/// 这四态只对订阅授权定义。而规格第 3.2 章逐字「同时支持永久授权与订阅授权」，
/// 第 3.5 章逐字「永久授权部分继续可用：**平台不因维护订阅到期而停机**，
/// 已交付功能保持可用」。
///
/// **该缺口已由裁定 F-17 处置：永久授权与年度维护订阅两项首版不交付**，
/// 本部署为自有企业内部使用、两项不存在，故不建 `grant_type` 列
/// （按裁定 F-15 的判别式，`PERPETUAL` 的凭证载体、状态机出边与后果承接方三样全缺，
/// 该撤不该预留；落码形式另照 `adapter/kms` 的先例——预留值不建枚举变体）。
///
/// 名字里的 `subscription` 因此不再是「两种里的一种」，
/// 而是**本部署唯一存在的授权形态**。
pub fn classify_subscription(f: &LicenseFacts) -> LicenseStatus {
    if f.revoked_at.is_some() {
        return LicenseStatus::Revoked;
    }
    if f.valid_to < f.today {
        return LicenseStatus::Expired;
    }
    if (f.valid_to - f.today).num_days() < EXPIRING_SOON_WINDOW_DAYS {
        return LicenseStatus::ExpiringSoon;
    }
    LicenseStatus::Valid
}

/// 现在是不是受限运行。
///
/// 规格第 3.4 章两条路进入受限运行：
/// 一、「**宽限期结束后**进入受限运行」——即 `today` 超过 `valid_to` 加 30 天；
/// 二、「撤销……导入后立即生效，平台**跳过临期告警与宽限期**直接进入受限运行」。
///
/// **这个函数与 [`classify_subscription`] 在宽限期那 30 天里给出不同答案，
/// 这正是它存在的理由。** 到期后第 1 天：状态是 `Expired`，而受限运行是 `false`，
/// 规格逐字「全部功能可用」。到期后第 31 天：状态仍是 `Expired`，受限运行变 `true`。
/// 同一个 `Expired` 两种后果——所以调用方不得据 [`LicenseStatus`] 决定停不停写。
///
/// 三处区间闭合口径规格全部未定义，本实现逐条取宽的一侧并在此登记：
/// 到期日**当天**不算过期；宽限期**第 30 天当天**仍在宽限期内，第 31 天起受限。
/// 取宽是因为判紧的每一天都是一天不可逆的业务中断，判松只多一天。
pub fn in_restricted_run(f: &LicenseFacts) -> bool {
    if f.revoked_at.is_some() {
        return true;
    }
    (f.today - f.valid_to).num_days() > GRACE_PERIOD_DAYS
}

#[cfg(test)]
mod tests {
    use super::*;

    fn d(y: i32, m: u32, day: u32) -> NaiveDate {
        NaiveDate::from_ymd_opt(y, m, day).expect("夹具日期应合法")
    }

    fn facts(valid_to: NaiveDate, today: NaiveDate) -> LicenseFacts {
        LicenseFacts {
            revoked_at: None,
            valid_to,
            today,
        }
    }

    /// 四个变体各要有一组事实能取到它。
    /// 一个列在枚举里却永远取不到的变体本身就是缺陷——本卷已在八档退避表上栽过一次。
    #[test]
    fn every_license_status_is_reachable() {
        let vt = d(2026, 1, 1);
        let reached: Vec<LicenseStatus> = vec![
            classify_subscription(&facts(vt, d(2025, 1, 1))), // 远早于到期
            classify_subscription(&facts(vt, d(2025, 12, 20))), // 距到期 12 天
            classify_subscription(&facts(vt, d(2026, 1, 2))), // 已过期
            classify_subscription(&LicenseFacts {
                revoked_at: Some(d(2025, 6, 1)),
                valid_to: vt,
                today: d(2025, 1, 1),
            }),
        ];
        assert_eq!(
            reached,
            vec![
                LicenseStatus::Valid,
                LicenseStatus::ExpiringSoon,
                LicenseStatus::Expired,
                LicenseStatus::Revoked,
            ]
        );
    }

    /// `ALL` 要既够数又互异。只断长度守不住——
    /// 写成 `[Valid, Valid, Expired, Revoked]` 时长度仍是 4，
    /// 而 `Revoked` 就此从任何遍历型用例里消失。
    #[test]
    fn all_is_complete_and_distinct() {
        assert_eq!(LicenseStatus::ALL.len(), 4);
        for (i, a) in LicenseStatus::ALL.iter().enumerate() {
            for b in &LicenseStatus::ALL[i + 1..] {
                assert_ne!(a, b, "ALL 里有重复变体");
            }
        }
    }

    /// 撤销短路：即便还远没到期，撤销也压过一切。
    /// 规格逐字「跳过临期告警与宽限期直接进入受限运行」。
    #[test]
    fn revocation_short_circuits_everything() {
        let f = LicenseFacts {
            revoked_at: Some(d(2025, 6, 1)),
            valid_to: d(2030, 1, 1),
            today: d(2025, 6, 2),
        };
        assert_eq!(classify_subscription(&f), LicenseStatus::Revoked);
        assert!(in_restricted_run(&f), "撤销立即进入受限运行，不走宽限期");
    }

    /// 三处区间闭合，规格全部未定义，本实现取的口径钉在这里。
    #[test]
    fn boundary_closures_are_pinned() {
        let vt = d(2026, 1, 1);
        // 到期日当天不算过期。
        assert_eq!(
            classify_subscription(&facts(vt, vt)),
            LicenseStatus::ExpiringSoon
        );
        // 到期日次日起过期。
        assert_eq!(
            classify_subscription(&facts(vt, d(2026, 1, 2))),
            LicenseStatus::Expired
        );
        // 距到期恰好 30 天仍是 Valid，29 天才进临期——「不足临期窗口」是严格小于。
        assert_eq!(
            classify_subscription(&facts(vt, d(2025, 12, 2))),
            LicenseStatus::Valid,
            "距到期 30 天，不满足「不足 30 天」"
        );
        assert_eq!(
            classify_subscription(&facts(vt, d(2025, 12, 3))),
            LicenseStatus::ExpiringSoon,
            "距到期 29 天"
        );
    }

    /// 本模块的要害：**宽限期里 `Expired` 与「受限运行」不是一回事**。
    ///
    /// 到期后第 1 天到第 30 天，状态是 `Expired` 而 `in_restricted_run` 为假，
    /// 规格逐字「到期日后进入 30 天宽限期：全部功能可用」。
    /// 谁把 `Expired` 直接当成「停业务写入」，就会比规格早 30 天
    /// 停掉一家二三十人公司的开单。
    #[test]
    fn expired_does_not_mean_restricted_during_the_grace_period() {
        let vt = d(2026, 1, 1);
        for (today, want_restricted) in [
            (d(2026, 1, 2), false),  // 宽限期第 1 天
            (d(2026, 1, 31), false), // 宽限期第 30 天
            (d(2026, 2, 1), true),   // 第 31 天，宽限期结束
        ] {
            let f = facts(vt, today);
            assert_eq!(
                classify_subscription(&f),
                LicenseStatus::Expired,
                "{today} 的状态应为 Expired"
            );
            assert_eq!(
                in_restricted_run(&f),
                want_restricted,
                "{today} 的受限运行判定不符"
            );
        }
    }

    /// 未到期的一律不受限——包括临期告警期间。
    /// 规格逐字临期告警「全部功能可用」。
    #[test]
    fn nothing_before_expiry_is_restricted() {
        let vt = d(2026, 1, 1);
        for today in [d(2025, 1, 1), d(2025, 12, 3), vt] {
            assert!(!in_restricted_run(&facts(vt, today)), "{today} 不该受限");
        }
    }

    /// 判定只吃入参里的 `today`，不吃进程时钟。
    /// 同一组 `valid_to` 传两个不同的 `today` 必须得到不同结论——
    /// 实现里若偷取了系统时间，这条会红。
    #[test]
    fn the_judgement_reads_today_from_the_argument() {
        let vt = d(2026, 1, 1);
        assert_ne!(
            classify_subscription(&facts(vt, d(2025, 1, 1))),
            classify_subscription(&facts(vt, d(2026, 6, 1))),
        );
    }

    /// 两个 30 同值不同源，不得合并成一个常量。
    #[test]
    fn the_two_thirty_day_windows_are_separate_constants() {
        assert_eq!(EXPIRING_SOON_WINDOW_DAYS, 30, "计划第 3.4.11 节");
        assert_eq!(GRACE_PERIOD_DAYS, 30, "规格第 3.4 章");
        let vt = d(2026, 1, 1);
        // 若有人把临期窗口改成规格的 60，这条会红，而宽限期用例不动——
        // 正是「两处改判互不牵连」要保的形态。
        assert_eq!(
            classify_subscription(&facts(vt, d(2025, 11, 20))),
            LicenseStatus::Valid,
            "距到期 42 天：30 天口径下是 Valid，60 天口径下是 ExpiringSoon"
        );
    }
}
