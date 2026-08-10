//! 跨模块引用的零大小标记类型，冻结 22 项。
//!
//! 本清单按裁定 A-01 与技术基线第 1.4 节冻结，任何阶段不得增删；
//! 增删须先改基线第 1.4 节并走基线修订，不走普通提交。
//! 本清单不适用第 1.3 节的两条准入判据（必要性与稳定性）。
//! 项数与名字由 `xtask archcheck` 的 foundation-frozen-items 按名逐项断言，
//! 形态（无字段、无方法、无 trait 实现）由 foundation-marker-shape 断言。

pub struct LegalEntity;
pub struct UserAccount;
pub struct Session;
pub struct Department;
pub struct Position;
pub struct Project;
pub struct Customer;
pub struct Supplier;
pub struct Material;
pub struct Product;
pub struct Warehouse;
pub struct Contract;
pub struct ContractLine;
pub struct SalesOrder;
pub struct SalesOrderLine;
pub struct DeliveryConfirmation;
pub struct DeliveryConfirmationLine;
pub struct PurchaseOrder;
pub struct GoodsReceiptLine;
pub struct PurchaseInvoice;
pub struct PurchaseInvoiceLine;
pub struct AccountingPeriod;
