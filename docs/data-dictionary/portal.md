# Portal 数据字典（F-50 冻结）

> **F-57 状态：`REGISTRY_PENDING_REBASELINE`。** 旧供应商门户数据可复用；客户门户、双门户 exact allowlist、独立 audience、动态权限和 generation 须按 F-57 追加，旧“可直接开发”状态不再有效。
>
> **激活/owner tasks：Tasks 5、22。** 本分册目前不是 F-57 实现权威；Task 5 完成门户持久化再基线且 Task 22 完成客户/供应商门户激活前不得据此实施。

历史状态（F-57 下无效）：曾标为“可直接开发的文档契约”，但尚未执行迁移。旧阶段 7 口径为 procure 23 表 + portal 8 表 = 31 张业务表、33 个本阶段迁移、31 张法人 RLS 表；第 33 支是 portal 目标建成后的 procure 真实外键追补。

## Portal 对象清单

`supplier_portal_users`、`delivery_notices`、`delivery_notice_lines`、`delivery_notice_line_serials`、`delivery_notice_attachments`、`supplier_invoice_uploads`、`supplier_invoice_upload_lines`、`supplier_invoice_upload_attachments`。

## supplier_invoice_uploads

业务头保存 `supplier_id`、`invoice_medium`、`number_scheme`、`invoice_code`、`invoice_no`、数据库生成的 `identifier_key` 与 `active_identifier_slot`、`issued_on`、引用业务单据、服务端汇总 `net_amount/tax_amount/gross_amount`、`status`、`return_reason`、`accepted_purchase_invoice_id`、`submitted_by_portal_user_id`。删除单一 `tax_rate`。状态为 `UPLOADED|RETURNED|ACCEPTED`，后两态终态；状态形状固定为 `UPLOADED` 两个终态字段均空、`RETURNED` 只有非空白退回原因、`ACCEPTED` 只有正式进项发票引用。

上传号码只在 portal staging 内防同供应商重复；ACCEPTED 前不占 `invoice.invoice_number_registry`。`active_identifier_slot` 在 `UPLOADED|ACCEPTED` 时等于 `identifier_key`、在 `RETURNED` 时为 NULL，以普通 `UNIQUE(legal_entity_id,supplier_id,active_identifier_slot)` 表达活动号码唯一；依赖 PostgreSQL 16 默认 `NULLS DISTINCT` 保留多张退回历史，不使用部分索引。

## supplier_invoice_upload_lines

业务列：`supplier_invoice_upload_id`、`line_no`、`purchase_order_id`、`purchase_order_line_id`、`goods_receipt_id`、`goods_receipt_line_id`、`cost_kind`、`item_id`、`quantity`、`net_unit_price`、`tax_rate`、`net_amount`、`tax_amount`、`gross_amount`。至少一行，同头行号唯一，金额规则同 invoice。

## Owner port

`SupplierInvoiceUploadId` 是 `ep-contract-portal` 局部 opaque ID，不加入 foundation marker 清单。

```rust
#[async_trait::async_trait]
pub trait SupplierInvoiceUploadWritebackPort: Send + Sync {
    async fn accept(&self, tx: &mut dyn Tx, ctx: &SecurityContext,
        upload_id: SupplierInvoiceUploadId,
        purchase_invoice_id: PurchaseInvoiceId) -> Result<(), AppError>;

    async fn return_upload(&self, tx: &mut dyn Tx, ctx: &SecurityContext,
        upload_id: SupplierInvoiceUploadId,
        reason: NonEmptyText,
        expected_row_version: i64) -> Result<(), AppError>;
}
```

端口由 `ep-app-portal` 实现；invoice 受理事务调用 `accept`。财务退回入口为 `POST /api/v1/portal/supplier-invoice-uploads/{id}/actions/return`，请求只含 `reason,row_version`，handler 必须把 `row_version` 原样传为 `expected_row_version`；owner 在锁内同时比较法人、供应商、`UPLOADED` 状态与版本，状态或版本失配统一返回 `PORTAL.SUPPLIER_INVOICE_UPLOAD.STATE_CHANGED`。端口外不得丢弃版本或另做一套竞争性先查后写。
