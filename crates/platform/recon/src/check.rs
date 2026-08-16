//! `ReconCheck` —— 一个对账校验项的契约。
//!
//! 签名取自裁定 A-06 的代码块，**两处偏离，各有理由**，见 [`BatchOutcome`]
//! 与 [`ReconCheck::batch_size`]。
//!
//! 实现方是阶段 7（六个）、8（两个）、9b（四个）、11（三个），
//! 一律在 job-worker 的 wiring 中经 [`crate::ReconRegistry::register`] 注册。
//! **本 crate 内一个实现体都没有**，也不该有。

use crate::model::{DiscrepancyState, ReconCategory, ReconDiscrepancy};
use ep_foundation::error::AppError;
use ep_foundation::id::marker::{AccountingPeriod, LegalEntity};
use ep_foundation::id::Id;
use ep_foundation::port::tx::SnapshotCtx;

/// 一批的结果。
///
/// # 为什么不是 A-06 写的裸 `Vec<ReconDiscrepancy>`
///
/// 裸 `Vec` 让 `Ok(vec![])` 二义：**「这一批没查出差异」与「已经越过末批」
/// 长得一模一样**。而 `BatchWindow` 不带总数、`ReconCheck` 上也没有别的终批信号，
/// 于是执行器只能猜——猜「空即结束」会在中间某一批恰好干净时提前收工，
/// 把后面的数据整段跳过；猜「空也继续」则永不停批。
///
/// 两个方向都是静默的，且前一个方向正是本卷要清的那类：**关账照常通过，
/// 而那部分数据从来没被校验过**。终批信号只能由知道自己数据边界的那一方给出。
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct BatchOutcome {
    /// 本批检出的差异。
    pub discrepancies: Vec<ReconDiscrepancy>,
    /// 本 check 是否还有下一批。`false` 即本 check 已产生结论。
    pub has_more: bool,
}

/// 一个对账校验项。
#[async_trait::async_trait]
pub trait ReconCheck: Send + Sync {
    /// 落库为 `recon_check_definitions.check_code`，全局唯一，不得为空串。
    fn code(&self) -> &'static str;

    /// 落库为 `category`。恰两个取值——A-06 撤销了 `CROSS_MODULE_LINK`。
    ///
    /// **本 crate 不校验它与 [`ReconCheck::code`] 的一致性。** 两套命名风格并存
    /// 且都是冻结取值（阶段 7 的 `R-PROC-01` 配 `INVARIANT`、阶段 11 的
    /// `COSTING_COST_VS_LEDGER` 配 `SUBLEDGER_VS_LEDGER`），卷内既无命名规约
    /// 也无 code 到 category 的对照表。任何前缀规则要么当场判违规一批，
    /// 要么宽到判不出东西——后者就是一条恒真的假门禁。
    fn category(&self) -> ReconCategory;

    /// 落库为 `is_blocking_period_close`。返真者按阶段 9 计划第 9.4.7 节
    /// 「构成关账前强制校验的范围」。
    ///
    /// **今天十五个实现全部返真**（阶段 7 逐字「一律为真」、阶段 8「均返回 true」、
    /// 阶段 11「均为 true」，9b 四项按同节的等价定义也在范围内）。
    /// 也就是说 `false` 这一侧目前是一个**取不到的登记值**。留着这个方法
    /// 是因为 A-06 冻结了它，且关账覆盖面的期望集要从它取；
    /// 但不要据此写「有些项不阻断关账」的分支——那条分支今天走不到。
    fn blocks_period_close(&self) -> bool;

    /// 本 check 每批处理多少行。
    ///
    /// # 为什么这个方法必须在 trait 上
    ///
    /// A-06 没有它，`BatchWindow` 由执行器构造。但 batch_size 的权威在各注册方
    /// 自己的配置域，且**单位互不通约**：库存侧的默认 2000 逐字「单位为仓库与
    /// 物料的组合数」，总账侧的默认 20000 逐字「单批处理的分录行或科目行数」。
    /// 执行器写死一个数的话，那个数对另一半 check 一定是错的——
    /// 要么把一批撑爆，要么把批数撑到没有意义。
    fn batch_size(&self) -> u32;

    /// 跑一批。
    ///
    /// 返回的差异**一律取 [`DiscrepancyState::Open`]、`approval_ref` 为空**——
    /// 见 [`validate_fresh_discrepancy`]。一条新检出就带着「已豁免」的差异
    /// 会直接从关账闸门下穿过去，而闸门那一侧看不出它是刚生出来的。
    async fn run_batch(
        &self,
        snapshot: &dyn SnapshotCtx,
        legal_entity_id: Id<LegalEntity>,
        accounting_period_id: Id<AccountingPeriod>,
        batch: crate::model::BatchWindow,
    ) -> Result<BatchOutcome, AppError>;
}

/// 对象安全断言。装配侧按 `Arc<dyn ReconCheck>` 注入
/// （A-06 第 319 行：十五个「全部经 `ReconRegistry::register` 在 job-worker 的
/// wiring 中注册」），trait 因此必须能做成 trait 对象。
///
/// 这是本模块唯一一条**夹具伪装不了**的断言——它在编译期。
const _: fn(std::sync::Arc<dyn ReconCheck>) = |_| {};

/// 校验一条**刚检出**的差异事项。
///
/// 新检出的差异只能是 [`DiscrepancyState::Open`] 且不带审批引用。
/// 三个非 `Open` 的取值是运维处置**之后**的态，由处置路径写入，不由校验项产出。
///
/// **判错方向的代价不对称**：一条带 `Waived` 进来的新差异会被关账闸门的
/// 「未了结差异」计数直接跳过（`is_settled` 含 `Repaired` 与 `Waived`），
/// 于是账实不符的期间照常关掉；反过来只是拒收一条差异、当场报错。
pub fn validate_fresh_discrepancy(d: &ReconDiscrepancy) -> Result<(), &'static str> {
    if d.state != DiscrepancyState::Open {
        return Err("新检出的差异事项必须是 OPEN；三个已处置的取值由处置路径写入，不由校验项产出");
    }
    if d.approval_ref.is_some() {
        return Err("新检出的差异事项不得带审批引用");
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::ReconDiscrepancy;

    fn fresh() -> ReconDiscrepancy {
        ReconDiscrepancy {
            check_code: "R-PROC-01".to_string(),
            subject_ref: r#"{"check_code":"R-PROC-01"}"#.to_string(),
            expected_amount: "100.00".to_string(),
            actual_amount: "99.00".to_string(),
            difference_amount: "1.00".to_string(),
            state: DiscrepancyState::Open,
            approval_ref: None,
        }
    }

    /// 新检出的差异必须是 OPEN。
    ///
    /// 逐个取值走一遍，而不是只测 `Waived`：只判 `Waived` 或只判
    /// `is_settled()` 的实现会在 `Repairing` 那一条漏过去，
    /// 而 `Repairing` 同样不该由校验项产出。
    #[test]
    fn only_open_is_a_fresh_discrepancy() {
        assert_eq!(validate_fresh_discrepancy(&fresh()), Ok(()));
        for s in DiscrepancyState::ALL {
            let mut d = fresh();
            d.state = s;
            let got = validate_fresh_discrepancy(&d);
            assert_eq!(
                got.is_ok(),
                s == DiscrepancyState::Open,
                "{s:?} 的判定不对；只判 Waived 或只判 is_settled 的实现会在 Repairing 上漏"
            );
        }
    }

    /// 带审批引用的「新」差异要拒——即便状态写的是 OPEN。
    #[test]
    fn a_fresh_discrepancy_carries_no_approval_ref() {
        let mut d = fresh();
        d.approval_ref = Some("APV-1".to_string());
        assert!(validate_fresh_discrepancy(&d).is_err());
    }
}
