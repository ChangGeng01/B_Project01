//! 对账的值类型。取值逐项照裁定 A-06 的三张表与契约段。
//!
//! A-06 撤销过两个取值，这里**只留改后的**，且各配一条计数用例：
//! `ReconCategory` 的 `CROSS_MODULE_LINK` 撤销（跨 schema 单目标引用改建真实外键），
//! 因此该枚举**恰两个取值**，不是三个。

use crate::ReconRun;
use ep_foundation::id::Id;

/// 对账检查的类别。**恰两个取值**——A-06 撤销了原第三个 `CROSS_MODULE_LINK`。
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum ReconCategory {
    /// 子账与总账勾稽。
    SubledgerVsLedger,
    /// 强制不变量。
    Invariant,
}

impl ReconCategory {
    pub fn as_db_value(self) -> &'static str {
        match self {
            ReconCategory::SubledgerVsLedger => "SUBLEDGER_VS_LEDGER",
            ReconCategory::Invariant => "INVARIANT",
        }
    }

    /// 全部取值。供计数用例与登记表比对用。
    pub const ALL: [ReconCategory; 2] =
        [ReconCategory::SubledgerVsLedger, ReconCategory::Invariant];
}

/// 一次对账运行的种类。
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum ReconRunKind {
    Daily,
    PeriodClose,
    RecoveryAcceptance,
}

impl ReconRunKind {
    pub fn as_db_value(self) -> &'static str {
        match self {
            ReconRunKind::Daily => "DAILY",
            ReconRunKind::PeriodClose => "PERIOD_CLOSE",
            ReconRunKind::RecoveryAcceptance => "RECOVERY_ACCEPTANCE",
        }
    }

    pub const ALL: [ReconRunKind; 3] = [
        ReconRunKind::Daily,
        ReconRunKind::PeriodClose,
        ReconRunKind::RecoveryAcceptance,
    ];
}

/// 一次对账运行的状态。
///
/// **三个取值，`FAILED` 已由裁定 F-14 撤销。** 规格第 10.2 章把五类终止成因
/// 全部归入「未完成」，阶段 14 的降级 kind 也只有对应未完成的一项；
/// 保留 `FAILED` 就要一次配齐产生条件、关账状态机的新出边、降级承接方与改规格四件，
/// 四件今天一件都没有。归因改由 [`TerminationCause`] 承担。
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum ReconRunStatus {
    Running,
    Completed,
    Unfinished,
}

impl ReconRunStatus {
    pub fn as_db_value(self) -> &'static str {
        match self {
            ReconRunStatus::Running => "RUNNING",
            ReconRunStatus::Completed => "COMPLETED",
            ReconRunStatus::Unfinished => "UNFINISHED",
        }
    }

    pub const ALL: [ReconRunStatus; 3] = [
        ReconRunStatus::Running,
        ReconRunStatus::Completed,
        ReconRunStatus::Unfinished,
    ];
}

/// 一次运行未跑完的终止成因。五个取值取阶段 9 计划逐字，
/// 由裁定 F-14 补为 `platform_core.recon_runs` 的一列。
///
/// 补这一列有两条理由。其一，`FAILED` 撤销之后，「运行本身出了什么事」
/// 需要一个能写的地方——否则「无从归因的中断」并进 `UNFINISHED` 之后就没了归因。
/// 其二，规格要求台账条目载明「已完成批次与**终止原因**」，
/// 而 `termination_cause` 今天只长在 `ledger.period_close_requests` 上，
/// `DAILY` 与 `RECOVERY_ACCEPTANCE` 两类运行无处可写。
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum TerminationCause {
    BatchTimeout,
    ResourceLimit,
    /// 执行进程异常退出。
    ///
    /// **谁替死掉的进程写这一值，卷内没有答案**——没有看门狗、没有超时清理。
    /// 裁定 F-14 明写这是卷内既有缺口（`ledger.period_close_requests` 上同样存在），
    /// 不是本 crate 能补的。留着这个取值是因为规格逐字把它列为五类成因之一。
    ProcessExit,
    ConnectionRecycled,
    /// 快照不可用，含建立失败与建立后失效。
    SnapshotInvalid,
}

impl TerminationCause {
    pub fn as_db_value(self) -> &'static str {
        match self {
            TerminationCause::BatchTimeout => "BATCH_TIMEOUT",
            TerminationCause::ResourceLimit => "RESOURCE_LIMIT",
            TerminationCause::ProcessExit => "PROCESS_EXIT",
            TerminationCause::ConnectionRecycled => "CONNECTION_RECYCLED",
            TerminationCause::SnapshotInvalid => "SNAPSHOT_INVALID",
        }
    }

    pub const ALL: [TerminationCause; 5] = [
        TerminationCause::BatchTimeout,
        TerminationCause::ResourceLimit,
        TerminationCause::ProcessExit,
        TerminationCause::ConnectionRecycled,
        TerminationCause::SnapshotInvalid,
    ];
}

/// 差异事项的状态。**三态，`WAIVED` 已由裁定 F-14 撤销**。
///
/// 规格与 PRD 对对账差异全文没有「豁免」语义——差异是修数据修掉的，
/// 不是标状态标掉的（见裁定 F-13 结论二）。裁定 F-10 已在另一张表上判过同形，
/// 理由是那样的取值「落地后只能靠测试代码手工塞一个 UUID」。
/// `REPAIRING` 与 `REPAIRED` 保留，它们在规格里有语义依据；首版仍无生产者，
/// 由建表迁移把 CHECK 的取值域收为 `OPEN` 一值，次版开处置流时再放开。
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum DiscrepancyState {
    Open,
    Repairing,
    Repaired,
}

impl DiscrepancyState {
    pub fn as_db_value(self) -> &'static str {
        match self {
            DiscrepancyState::Open => "OPEN",
            DiscrepancyState::Repairing => "REPAIRING",
            DiscrepancyState::Repaired => "REPAIRED",
        }
    }

    pub const ALL: [DiscrepancyState; 3] = [
        DiscrepancyState::Open,
        DiscrepancyState::Repairing,
        DiscrepancyState::Repaired,
    ];

    /// 是否已了结。**只有 `Repaired` 算了结**（`WAIVED` 已由 F-14 撤销）。
    ///
    /// # 首版恒假，因此不得拿它当闸门
    ///
    /// 按裁定 F-13，三个非 `Open` 取值在首版没有生产者，这个方法因此**恒返回假**。
    /// 关账闸门一度用 `!is_settled()` 过滤计数——那个过滤恒真，
    /// 计数只增不减，一个期间出过一条差异就再也关不上。已改。
    /// 拦关账的判据是**本次校验的校验项结论**，见 [`crate::gate`]。
    ///
    /// 这条判定直接决定关账能不能受理（规格第 10.2 章「差异清零前不得关账」）。
    /// 把 `Repairing` 误判成已了结，会让一个正在修、还没修完的差异放行关账——
    /// 而关账之后那个期间就不再接受任何凭证写入，差异就永远修不掉了。
    pub fn is_settled(self) -> bool {
        matches!(self, DiscrepancyState::Repaired)
    }
}

/// 一条差异事项。金额三项**以字符串承载**，不用浮点。
///
/// 理由与 `ep-platform-audit` 的 JCS 那处同源：本卷金额是 `numeric(18,2)`，
/// 最大到 10^18 量级，超出双精度能精确表示的范围。用 `f64` 承载会在这里
/// 悄悄舍入，而差异金额恰恰是**用来判断差异是否为零**的那个数——
/// 舍入让一个非零差异变成零，就等于把一条真差异放行进关账。
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ReconDiscrepancy {
    pub check_code: String,
    /// 差异主体的定位信息，落库为 jsonb。
    pub subject_ref: String,
    pub expected_amount: String,
    pub actual_amount: String,
    pub difference_amount: String,
    pub state: DiscrepancyState,
}

/// 分批窗口。
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct BatchWindow {
    pub batch_no: u32,
    pub batch_size: u32,
    pub offset: u64,
}

impl BatchWindow {
    /// 第 `batch_no` 批（从 0 起）的窗口。
    ///
    /// `offset` 用 `u64` 而 `batch_no` 与 `batch_size` 用 `u32`：
    /// 两个 `u32` 相乘会溢出，而一次对账的总行数可以远超 `u32`。
    /// 这里显式先转再乘——写成 `batch_no * batch_size` 再转，
    /// 在批数大时会在转之前就已经绕回去了。
    pub fn nth(batch_no: u32, batch_size: u32) -> Self {
        Self {
            batch_no,
            batch_size,
            offset: u64::from(batch_no) * u64::from(batch_size),
        }
    }
}

/// 一次对账运行的结果。
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ReconRunOutcome {
    /// 本次运行的标识。**上一轮把这个字段静默删掉了**，与 A-06 逐字不一致
    /// 且没留下任何说明；本轮补回。标记类型 [`crate::ReconRun`] 落在本 crate 内，
    /// 不进 foundation——那张标记表是冻结的，且它收的是跨模块引用的标记。
    pub run_id: Id<ReconRun>,
    pub status: ReconRunStatus,
    pub discrepancy_count: u32,
    /// **本次运行真正跑到结论的检查项。** A-06 没有这个字段，本轮加。
    ///
    /// 没有它，`Completed` 不蕴含「该跑的都跑了」：一个根本没注册的校验项
    /// 既不在未完成清单里、也不产生差异行，于是 `Completed` 加空差异加闸门放行。
    /// 这是「未覆盖 ≠ 通过」最直接的破口，而它在阶段 9a 交付本体那一刻**必然发生**——
    /// 那时十五项里有十一项还不存在。
    pub executed_check_codes: Vec<String>,
    /// 没跑到底的检查项。`Unfinished` 时与 `termination_cause` 至少其一非空。
    pub unfinished_check_codes: Vec<String>,
    /// 运行本身的终止成因。裁定 F-14 撤销 `FAILED` 之后由它承担归因。
    pub termination_cause: Option<TerminationCause>,
}

impl ReconRunOutcome {
    /// 运行刚开始那一刻的结果。
    ///
    /// **`Running` 是一条活路径**——裁定 F-14 已把 `recon_runs` 的登记
    /// 由 `APPEND_ONLY` 改为 `IMMUTABLE_COLUMNS`，可变列取
    /// `status`、`batch_done`、`finished_at`、`termination_cause` 四列，
    /// 于是 `RUNNING` 到终态的更新走得通，而证据列（法人、运行类别、期间、
    /// 快照标识、开始时刻与制品标识两列）仍不可改。
    pub fn running(run_id: Id<ReconRun>) -> Self {
        Self {
            run_id,
            status: ReconRunStatus::Running,
            discrepancy_count: 0,
            executed_check_codes: Vec::new(),
            unfinished_check_codes: Vec::new(),
            termination_cause: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A-06 撤销了 `CROSS_MODULE_LINK`，类别恰两个。
    /// 这条守的是「撤销的取值被人不小心加回来」。
    #[test]
    fn category_has_exactly_two_values_after_a06() {
        assert_eq!(
            ReconCategory::ALL.len(),
            2,
            "A-06 撤销了 CROSS_MODULE_LINK，跨 schema 单目标引用改建真实外键"
        );
        assert_eq!(ReconRunKind::ALL.len(), 3);
        assert_eq!(ReconRunStatus::ALL.len(), 3, "FAILED 已由裁定 F-14 撤销");
        assert_eq!(DiscrepancyState::ALL.len(), 3, "WAIVED 已由裁定 F-14 撤销");
        assert_eq!(
            TerminationCause::ALL.len(),
            5,
            "规格第 10.2 章逐字五类终止成因"
        );
    }

    #[test]
    fn db_values_are_screaming_snake() {
        assert_eq!(
            ReconCategory::SubledgerVsLedger.as_db_value(),
            "SUBLEDGER_VS_LEDGER"
        );
        assert_eq!(
            ReconRunKind::RecoveryAcceptance.as_db_value(),
            "RECOVERY_ACCEPTANCE"
        );
        assert_eq!(ReconRunStatus::Unfinished.as_db_value(), "UNFINISHED");
        assert_eq!(DiscrepancyState::Repaired.as_db_value(), "REPAIRED");
        assert_eq!(TerminationCause::ProcessExit.as_db_value(), "PROCESS_EXIT");
    }

    /// 只有已修复算了结。把 `REPAIRING` 当成了结会放行关账，
    /// 而关账之后那个期间不再接受凭证写入，差异就永远修不掉了。
    ///
    /// **首版三个取值里只有 `OPEN` 有生产者**（裁定 F-13 结论四），
    /// 因此这个方法在首版恒假；关账拦截不走它，走的是本次校验的校验项结论。
    #[test]
    fn only_repaired_is_settled() {
        assert!(!DiscrepancyState::Open.is_settled());
        assert!(!DiscrepancyState::Repairing.is_settled());
        assert!(DiscrepancyState::Repaired.is_settled());
    }

    /// `WAIVED` 撤销之后，取值域里不得再出现它——包括 DB 取值这一侧。
    /// 规格与 PRD 对对账差异全文没有「豁免」语义，一个没有语义依据的取值
    /// 落地后只能靠测试代码手工塞一个凭据，那是裁定 F-10 判过的形态。
    #[test]
    fn no_waived_value_survives_anywhere() {
        let vals: Vec<&str> = DiscrepancyState::ALL
            .iter()
            .map(|s| s.as_db_value())
            .collect();
        assert!(!vals.contains(&"WAIVED"), "取值域里仍有 WAIVED：{vals:?}");
    }

    /// 五个终止成因的 DB 取值互异且齐备。
    #[test]
    fn termination_causes_are_distinct() {
        let mut v: Vec<&str> = TerminationCause::ALL
            .iter()
            .map(|c| c.as_db_value())
            .collect();
        v.sort_unstable();
        v.dedup();
        assert_eq!(v.len(), 5);
    }

    #[test]
    fn batch_offsets_advance_by_size() {
        assert_eq!(BatchWindow::nth(0, 1000).offset, 0);
        assert_eq!(BatchWindow::nth(1, 1000).offset, 1000);
        assert_eq!(BatchWindow::nth(3, 250).offset, 750);
    }

    /// 大批数不得溢出。两个 u32 相乘会绕回，而一次对账的总行数可以远超 u32。
    #[test]
    fn large_batch_numbers_do_not_overflow() {
        let w = BatchWindow::nth(5_000_000, 1_000);
        assert_eq!(w.offset, 5_000_000_000, "先转 u64 再乘，不得先乘后转");
    }
}
