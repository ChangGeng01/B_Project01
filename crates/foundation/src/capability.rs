//! 能力域码与动作类别。
//!
//! `CapabilityDomain` 的 18 项顺序与阶段 13 计划第 4.4 节表格序号一致。
//! `ActionClass` 的五项与该节判定算法的 ViewOnly 分支配套，ViewOnly 只放行 Read。

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, serde::Serialize, serde::Deserialize)]
pub enum CapabilityDomain {
    CrmCustomer360,
    ClmContractEsign,
    SalesOrderFulfillment,
    ProcureSupplierCollab,
    InventoryLedgerScan,
    ServiceWorkorderEquipment,
    PlatformApprovalNotify,
    ProjectTaskMilestone,
    MdmMasterData,
    PlatformFullTextSearch,
    LedgerPostingClose,
    FinanceSettlementView,
    InvoiceApplyIssue,
    ReportingReportPrint,
    PlatformDocumentAttachment,
    PlatformAdminLowcodeOps,
    PlatformExtensionDynamicCode,
    PortalSupplierWeb,
}

impl CapabilityDomain {
    pub const ALL: [CapabilityDomain; 18] = [
        CapabilityDomain::CrmCustomer360,
        CapabilityDomain::ClmContractEsign,
        CapabilityDomain::SalesOrderFulfillment,
        CapabilityDomain::ProcureSupplierCollab,
        CapabilityDomain::InventoryLedgerScan,
        CapabilityDomain::ServiceWorkorderEquipment,
        CapabilityDomain::PlatformApprovalNotify,
        CapabilityDomain::ProjectTaskMilestone,
        CapabilityDomain::MdmMasterData,
        CapabilityDomain::PlatformFullTextSearch,
        CapabilityDomain::LedgerPostingClose,
        CapabilityDomain::FinanceSettlementView,
        CapabilityDomain::InvoiceApplyIssue,
        CapabilityDomain::ReportingReportPrint,
        CapabilityDomain::PlatformDocumentAttachment,
        CapabilityDomain::PlatformAdminLowcodeOps,
        CapabilityDomain::PlatformExtensionDynamicCode,
        CapabilityDomain::PortalSupplierWeb,
    ];
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, serde::Serialize, serde::Deserialize)]
pub enum ActionClass {
    Read,
    Write,
    Submit,
    Approve,
    Export,
}
