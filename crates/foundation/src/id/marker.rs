//! 跨模块引用的零大小标记类型，现有 22 项。
//!
//! 准入判据见技术基线第 1.3 节：无字段、无方法、无 trait 实现，
//! 只承载类型身份，供 `Id<T>` 在契约层表达跨模块引用。
//! 不设冻结清单，增删走普通提交并由 `xtask archcheck` 断言两条判据。

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
