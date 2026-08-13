//! 降级窗口台账端口（裁定 A-26、D-15；02 计划 §7.1 与 §11 预留点九）。
//!
//! 权威存储是 `platform_ops.degradation_windows`（阶段 2 第 104500 号迁移已建表，
//! 列形态与阶段 14 计划第 3.1 节表 3 完全一致）；数据库侧实现落在
//! ep-adapter-db-pg 的 `platform_ops` 同名目录（db-pg-one-schema-per-file），
//! 本模块只声明端口、kind 枚举与开窗请求形态，不出现任何 SQL。
//!
//! 阶段 1 按 A-26 预留的 `// TODO(stage-2): write degradation ledger` 由本端口补上：
//! 阶段 14 的 `offsite-sink-requirements` 第八个子判定失败时经
//! [`DegradationLedger::open`] 登记 `OFFSITE_SINK_NOT_CONFIGURED` 窗口，
//! 该窗口不可抑制；`WRITER_NOT_IN_SERVICE` 的登记方同样是阶段 14。
//! 本阶段只交付端口与三个初始 kind 取值，不预支阶段 14 的触发路径。
//!
//! 台账与指标双出（规格第 15.3 章）：数据库表由实现侧写入，
//! `ep_degradation_windows_open` gauge 由实现侧在 open/close 后刷新，
//! 两处不得只有其一。

use ep_foundation::error::AppError;
use ep_foundation::id::marker::{AccountingPeriod, LegalEntity};
use ep_foundation::id::Id;

/// 降级 kind 的三个初始取值（E-16）。取值名与表上
/// `ck_degradation_windows_kind` 的 CHECK 逐字一致；其余取值由阶段 14 扩展，
/// 扩展时同步放宽 CHECK，不在本枚举之外另起形态。
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum DegradationKind {
    /// 异地汇出未配置。不可抑制，登记方为阶段 14。
    OffsiteSinkNotConfigured,
    /// 写出进程未投入运行或连续若干周期无上报，登记方为阶段 14。
    WriterNotInService,
    /// 跨模块与平台能力缺位的唯一登记形态；开窗时由 `subject` 记下
    /// 该端口或平台能力的完整类型名。
    PortNotImplemented,
}

impl DegradationKind {
    /// 入库取值，与 CHECK 约束逐字一致。
    pub const fn as_str(self) -> &'static str {
        match self {
            DegradationKind::OffsiteSinkNotConfigured => "OFFSITE_SINK_NOT_CONFIGURED",
            DegradationKind::WriterNotInService => "WRITER_NOT_IN_SERVICE",
            DegradationKind::PortNotImplemented => "PORT_NOT_IMPLEMENTED",
        }
    }

    /// 本阶段定义的全部取值，顺序即登记顺序。
    pub const ALL: [DegradationKind; 3] = [
        DegradationKind::OffsiteSinkNotConfigured,
        DegradationKind::WriterNotInService,
        DegradationKind::PortNotImplemented,
    ];
}

/// 一次开窗请求。字段集合对齐表上的必填列；`id`、时间戳与 `row_version`
/// 由实现侧生成，不进请求形态。
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct WindowOpenRequest {
    pub kind: DegradationKind,
    /// 开窗对象的完整类型名（端口名或平台能力名），可空。
    pub subject: Option<String>,
    /// 作用域键，标注窗口作用在哪个对象上，最长 200 字符。
    pub scope_key: String,
    /// 两个 scope 列只作标注不作策略判据。
    pub scope_legal_entity_id: Option<Id<LegalEntity>>,
    pub scope_accounting_period_id: Option<Id<AccountingPeriod>>,
    /// 开窗依据，最长 2000 字符。
    pub basis: String,
    /// 关窗条件的客观描述，最长 2000 字符。
    pub closing_condition: String,
    /// 是否允许抑制。`OFFSITE_SINK_NOT_CONFIGURED` 一律取假。
    pub is_suppressible: bool,
}

/// 关窗的定位五元组，与唯一约束 `ux_degradation_windows_kind_scope_closed`
/// 的判据列一一对应（closed_at 由实现侧置位，不在定位之内）。
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct WindowRef {
    pub kind: DegradationKind,
    pub subject: Option<String>,
    pub scope_legal_entity_id: Option<Id<LegalEntity>>,
    pub scope_accounting_period_id: Option<Id<AccountingPeriod>>,
}

/// 降级窗口台账。实现必须满足：
/// - `open` 与 `close` 幂等：同一活动窗口重复开窗由表上唯一约束拒绝并
///   映射为已登记的冲突类错误码；重复关一个已关闭的窗口不报错；
/// - 每次 open/close 成功后刷新 `ep_degradation_windows_open` gauge，
///   取值与 `open_count` 一致（IT-41）。
#[async_trait::async_trait]
pub trait DegradationLedger: Send + Sync {
    /// 开一个窗口。同一 kind、subject 与作用域下已有未关闭窗口时，
    /// 由表上唯一约束拒绝。
    async fn open(&self, req: WindowOpenRequest) -> Result<(), AppError>;

    /// 关闭定位五元组下的活动窗口；无活动窗口时视为已完成。
    async fn close(&self, window: WindowRef) -> Result<(), AppError>;

    /// 当前未关闭窗口的总数，供 gauge 与运行期巡检取用。
    async fn open_count(&self) -> Result<usize, AppError>;
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 三个初始取值逐字对齐表上 CHECK，制品中不得出现已撤销的
    /// `WRITER_ROLE_CONTAINMENT_MISSING` 一名（E-16）。
    #[test]
    fn three_initial_kinds_match_the_check_constraint() {
        assert_eq!(
            DegradationKind::ALL.map(|k| k.as_str()),
            [
                "OFFSITE_SINK_NOT_CONFIGURED",
                "WRITER_NOT_IN_SERVICE",
                "PORT_NOT_IMPLEMENTED",
            ]
        );
    }

    /// 装配侧以 `Arc<dyn _>` 注入，trait 必须对象安全。
    #[test]
    fn trait_is_object_safe() {
        fn _ledger(_x: std::sync::Arc<dyn DegradationLedger>) {}
    }

    #[test]
    fn open_request_carries_required_columns() {
        let req = WindowOpenRequest {
            kind: DegradationKind::PortNotImplemented,
            subject: Some("ep_foundation::port::doc::DocStore".to_string()),
            scope_key: "core-server".to_string(),
            scope_legal_entity_id: None,
            scope_accounting_period_id: None,
            basis: "端口未装配".to_string(),
            closing_condition: "装配该端口后关闭".to_string(),
            is_suppressible: false,
        };
        assert_eq!(req.clone(), req, "开窗请求可克隆且可判等");
    }
}
