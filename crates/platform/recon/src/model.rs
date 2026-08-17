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
/// `Unfinished` 与 `Failed` 是两回事，**不得合并**：前者是跑完了但有检查项没跑到底
/// （例如单批超时终止），后者是运行本身出错。关账受理时前者要列出没跑到底的检查项、
/// 后者要看错误——处置路径不同。
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum ReconRunStatus {
    Running,
    Completed,
    Unfinished,
    Failed,
}

impl ReconRunStatus {
    pub fn as_db_value(self) -> &'static str {
        match self {
            ReconRunStatus::Running => "RUNNING",
            ReconRunStatus::Completed => "COMPLETED",
            ReconRunStatus::Unfinished => "UNFINISHED",
            ReconRunStatus::Failed => "FAILED",
        }
    }

    pub const ALL: [ReconRunStatus; 4] = [
        ReconRunStatus::Running,
        ReconRunStatus::Completed,
        ReconRunStatus::Unfinished,
        ReconRunStatus::Failed,
    ];
}

/// 差异事项的状态。四态，与 `platform_core.recon_discrepancies.state` 的 CHECK 一致。
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum DiscrepancyState {
    Open,
    Repairing,
    Repaired,
    Waived,
}

impl DiscrepancyState {
    pub fn as_db_value(self) -> &'static str {
        match self {
            DiscrepancyState::Open => "OPEN",
            DiscrepancyState::Repairing => "REPAIRING",
            DiscrepancyState::Repaired => "REPAIRED",
            DiscrepancyState::Waived => "WAIVED",
        }
    }

    pub const ALL: [DiscrepancyState; 4] = [
        DiscrepancyState::Open,
        DiscrepancyState::Repairing,
        DiscrepancyState::Repaired,
        DiscrepancyState::Waived,
    ];

    /// 是否已了结。**只有 `Repaired` 与 `Waived` 算了结。**
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
        matches!(self, DiscrepancyState::Repaired | DiscrepancyState::Waived)
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
    /// 豁免凭据。`Waived` 时必须非空——见 [`validate_waiver`]。
    pub approval_ref: Option<String>,
}

/// 豁免必须带审批凭据。
///
/// 这条在库层有 CHECK 兜底，这里再判一次不是重复：库层的 CHECK 只在写入那一刻拦，
/// 而调用方在构造这条记录、决定要不要豁免的时候就该被拦住——
/// 让一个注定写不进去的记录一路走到数据库才被拒，错误信息离出错的地方太远。
pub fn validate_waiver(d: &ReconDiscrepancy) -> Result<(), &'static str> {
    if d.state == DiscrepancyState::Waived && d.approval_ref.as_deref().is_none_or(str::is_empty) {
        return Err("豁免差异必须带审批凭据（approval_ref），不得空豁免");
    }
    Ok(())
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
    /// 没跑到底的检查项。`Unfinished` 时非空，这是它与 `Failed` 的区别所在。
    pub unfinished_check_codes: Vec<String>,
}

impl ReconRunOutcome {
    /// 运行刚开始那一刻的结果。
    ///
    /// **`Running` 这个取值在生产上大概率取不到**：`platform_core.recon_runs`
    /// 按裁定 B-02 登记为仅追加表且可变列白名单为空数组，而仅追加表的
    /// `BEFORE UPDATE OR DELETE` 触发器一律 raise——`RUNNING` 到终态的那次更新
    /// 上线即被无条件拒绝，实现方只能绕过 `RUNNING` 直插终态。
    /// 这一处矛盾已登记入裁定文件附录辛，本构造函数留着是因为 A-06 的取值域里有它、
    /// 且关账闸门对它有一条分支；**不要据此认为它是一条活路径**。
    pub fn running(run_id: Id<ReconRun>) -> Self {
        Self {
            run_id,
            status: ReconRunStatus::Running,
            discrepancy_count: 0,
            executed_check_codes: Vec::new(),
            unfinished_check_codes: Vec::new(),
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
        assert_eq!(ReconRunStatus::ALL.len(), 4);
        assert_eq!(DiscrepancyState::ALL.len(), 4);
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
        assert_eq!(DiscrepancyState::Waived.as_db_value(), "WAIVED");
    }

    /// 只有已修复与已豁免算了结。把 REPAIRING 当成了结会放行关账，
    /// 而关账之后那个期间不再接受凭证写入，差异就永远修不掉了。
    #[test]
    fn only_repaired_and_waived_are_settled() {
        assert!(!DiscrepancyState::Open.is_settled());
        assert!(!DiscrepancyState::Repairing.is_settled());
        assert!(DiscrepancyState::Repaired.is_settled());
        assert!(DiscrepancyState::Waived.is_settled());
    }

    fn discrepancy(state: DiscrepancyState, approval: Option<&str>) -> ReconDiscrepancy {
        ReconDiscrepancy {
            check_code: "SUB_VS_LED_AR".to_string(),
            subject_ref: "{}".to_string(),
            expected_amount: "100.00".to_string(),
            actual_amount: "90.00".to_string(),
            difference_amount: "10.00".to_string(),
            state,
            approval_ref: approval.map(str::to_string),
        }
    }

    /// 空豁免必须被拒：既不给凭据、又要豁免，等于无人负责地抹掉一条差异。
    #[test]
    fn waiver_without_approval_is_rejected() {
        assert!(validate_waiver(&discrepancy(DiscrepancyState::Waived, None)).is_err());
        // 空串同样不算凭据——这是最容易漏的一种「有值但没用」。
        assert!(validate_waiver(&discrepancy(DiscrepancyState::Waived, Some(""))).is_err());
        assert!(validate_waiver(&discrepancy(DiscrepancyState::Waived, Some("appr-1"))).is_ok());
    }

    /// 非豁免态不要求凭据。
    #[test]
    fn non_waived_states_do_not_need_approval() {
        for s in [
            DiscrepancyState::Open,
            DiscrepancyState::Repairing,
            DiscrepancyState::Repaired,
        ] {
            assert!(validate_waiver(&discrepancy(s, None)).is_ok(), "{s:?}");
        }
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
