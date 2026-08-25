# 事件目录

> **F-57 状态：`REGISTRY_PENDING_REBASELINE`。** 既有事件仍是历史兼容输入；F-57 命令、事实、objective/effect/evidence、generation、包、权限和恢复事件尚须由实施计划 Task 1 统一登记，本文件当前不得作为完整闭集。

本文件是全部领域事件类型的唯一登记处。新增事件必须先登记再实现：先在本文件加行，再写产生该事件的代码。写入侧对未登记的 `event_type` 一律拒绝，不静默放行。

## 1. 事件类型命名

四段，形如 `<module>.<aggregate>.<past_participle>.v<major>`，全小写点分。例：`clm.contract.effective.v1`、`sales.delivery.confirmed.v1`、`inventory.stock_movement.posted.v1`、`ledger.voucher.posted.v1`、`finance.payment.registered.v1`。

三条硬约束：

- 事件名一律用已完成时态，禁止用命令式动词。`order.create` 是命令不是事件，`sales.order.created.v1` 才是。
- 模块段取技术基线第 1.2 节的模块码，或平台事件的 `platform`。
- 版本段是主版本。破坏性变更即删除或重命名字段、收紧校验、改变默认值语义、删除枚举取值，必须升主版本并作为新的一行登记，旧行保留。

## 2. 信封

载荷结构固定为信封加业务体，信封字段不得增删。任何事件的差异只在 `payload` 内。

```json
{
  "event_id": "0192f3a1-...-uuidv7",
  "event_type": "sales.delivery.confirmed.v1",
  "event_version": 1,
  "occurred_at": "2026-08-10T02:11:43.512Z",
  "legal_entity_id": "…",
  "aggregate_type": "sales.delivery_confirmations",
  "aggregate_id": "…",
  "aggregate_version": 7,
  "security_level": 20,
  "data_scope_tags": ["dept:sales", "project:P-2026-0007"],
  "posting_date": "2026-08-10",
  "accounting_period_id": "…",
  "correlation_id": "…",
  "causation_id": "…",
  "idempotency_key": "…",
  "actor": { "user_id": "…", "device_id": "…", "on_behalf_of": null },
  "payload": { }
}
```

三处形态与代码共用同一份编解码，不得各自实现：

- `security_level` 取 10、20、30、40 四值并序列化为数字，形态见 `crates/foundation/src/security/level.rs` 的 `SecurityLevel`；未知取值反序列化失败，不静默降级。
- `data_scope_tags` 的元素形态为 `<kind>:<value>`，kind 取 `[a-z0-9_-]`，value 取 `[A-Za-z0-9_-]`，总长上限 128，形态见同目录 `context.rs` 的 `DataScopeTag`。该形态与公共列 `data_scope_tags text[]` 的元素形态同源。
- `aggregate_type` 为 `<module>.<table>` 的小写下划线形态，与 `RecordShare::object_type` 同形。

`posting_date` 与 `accounting_period_id` 两项对不涉及过账的事件取空值，取空值不等于可以省略字段。

## 3. 登记表的组织方式

按模块段分节，节的顺序取技术基线第 1.2 节的模块码顺序，`platform` 段在最前。段名与下表逐字一致，`ModuleCode` 的 15 项取值见 `crates/foundation/src/module.rs`：

`platform`、`mdm`、`crm`、`cpq`、`clm`、`sales`、`procure`、`inventory`、`costing`、`project`、`service`、`finance`、`ledger`、`invoice`、`portal`、`reporting`。

每段一张表，列固定为：事件类型、聚合类型、触发时点、`payload` 关键字段、消费者、`produces_voucher`、登记阶段。消费者一列写「暂无」时必须同时写明预期消费方所属阶段，不允许只写「暂无」。`produces_voucher` 只允许 `true` 或 `false`；阶段 9a 的 `xtask configdoc` 只从取 `true` 的行生成 `ledger.posting_trigger_event_types` 种子，任何正文说明都不能替代该列。

## 4. 登记表

### 4.1 platform 段

| 事件类型 | 聚合类型 | 触发时点 | `payload` 关键字段 | 消费者 | `produces_voucher` | 登记阶段 |
|---|---|---|---|---|---|---|
| `platform.key_domain.provisioned.v1` | platform.key_domains | A-03 密钥域供给成功（201）时 | key_domain_id、legal_entity_id、kek_ref | 阶段 12 销毁核验 | false | 阶段 2 |
| `platform.key_domain.rotated.v1` | platform.key_domains | A-04 数据密钥轮换落库成功时 | key_domain_id、legal_entity_id、purpose、new_version、retiring_data_key_id | 阶段 12 销毁核验 | false | 阶段 2 |
| `platform.migration_window.opened.v1` | platform.migration_windows | A-09 迁移窗口开窗成功（201）时 | window_id、approval_ref、expires_at | 阶段 14 运维台账 | false | 阶段 2 |
| `platform.attachment.published.v1` | platform.attachment_objects | 附件版本内建检查与约定扫描通过、版本原子置 AVAILABLE 时 | attachment_object_id、attachment_version_id、version_no、content_hash、scan_mode | search、业务附件投影 | false | 阶段 3b |
| `platform.notification.push_requested.v1` | platform.notification_deliveries | 站内通知事务发现活跃推送登记且推送开关启用时 | notification_id、delivery_id、recipient_user_id、notice_type | job-worker `platform.push_dispatch` | false | 阶段 3b |
| `platform.config_release.released.v1` | platform.config_release_orders | 发布单、配置包与全部内容项在段二事务成功时 | release_order_id、package_id、package_version、requires_derived_store_rebuild | 阶段 13b 发布传播消费者 | false | 阶段 3b |
| `platform.user_account.deactivated.v1` | platform.user_accounts | 账号停用端点成功，全部会话与设备凭据即时撤销后 | user_id、legal_entity_id | search、账号审计投影 | false | 阶段 4 |
| `platform.user_account.locked.v1` | platform.account_lockouts | 登录失败达锁定阈值，锁定与窗口计数落库后 | user_id、legal_entity_id、locked_until | 账号审计投影 | false | 阶段 4 |
| `platform.user_account.transferred.v1` | platform.user_accounts | 账号移交端点成功，职责归属迁移事务提交后 | from_user_id、to_user_id、legal_entity_id | search、账号审计投影 | false | 阶段 4 |
| `platform.breakglass_activation.closed.v1` | platform.breakglass_activations | 应急账号关闭且凭据轮换完成后 | activation_id、legal_entity_id、rotation_ref | 密钥台账、notify | false | 阶段 4 |
| `platform.authz_policy.published.v1` | platform_authz.authz_config_versions | 已签名授权配置版本原子切为 EFFECTIVE 时 | authz_config_version_id、version_no、spec_hash | 派生存储重建与重新打标 | false | 阶段 4（与阶段 3b 发布通道同批） |
| `platform.custom_record.created.v1` | ext.&lt;object_code&gt; | 自定义对象记录创建事务提交时 | object_code、record_id、definition_version | search、规则触发、notify | false | 阶段 13b |
| `platform.custom_record.updated.v1` | ext.&lt;object_code&gt; | 自定义对象记录更新事务提交时 | object_code、record_id、definition_version、changed_field_codes | search、规则触发、notify | false | 阶段 13b |
| `platform.custom_record.state_changed.v1` | ext.&lt;object_code&gt; | 自定义对象状态机迁移事务提交时 | object_code、record_id、from_state、to_state | search、规则触发、notify | false | 阶段 13b |

### 4.2 mdm 段

阶段 5 的五类档案统一只在审批结论应用或明确生命周期动作完成后发事件；草稿保存、提交审批和撤回不发档案事件。`effective` 表示 CREATION 申请首次生效，`updated` 表示已生效档案的 CHANGE 申请应用，`deactivated` 与 `reactivated` 分别对应停用与再启用。这样五类各四个事件，加导入完成一个，共 21 个 mdm 事件。

| 事件类型 | 聚合类型 | 触发时点 | `payload` 关键字段 | 消费者 | `produces_voucher` | 登记阶段 |
|---|---|---|---|---|---|---|
| `mdm.customer.effective.v1` | mdm.customers | CREATION 变更申请应用且客户首次生效时 | customer_id、code、version_no | `mdm.search_indexer`、notify | false | 阶段 5 |
| `mdm.customer.updated.v1` | mdm.customers | 已生效客户的 CHANGE 申请应用时 | customer_id、code、version_no、changed_fields | `mdm.search_indexer`、notify | false | 阶段 5 |
| `mdm.customer.deactivated.v1` | mdm.customers | 停用申请应用时 | customer_id、code、version_no、reason | `mdm.search_indexer`、notify | false | 阶段 5 |
| `mdm.customer.reactivated.v1` | mdm.customers | 再启用申请应用时 | customer_id、code、version_no | `mdm.search_indexer`、notify | false | 阶段 5 |
| `mdm.supplier.effective.v1` | mdm.suppliers | CREATION 变更申请应用且供应商首次生效时 | supplier_id、code、version_no | `mdm.search_indexer`、notify | false | 阶段 5 |
| `mdm.supplier.updated.v1` | mdm.suppliers | 已生效供应商的 CHANGE 申请应用时 | supplier_id、code、version_no、changed_fields | `mdm.search_indexer`、notify | false | 阶段 5 |
| `mdm.supplier.deactivated.v1` | mdm.suppliers | 停用申请应用时 | supplier_id、code、version_no、reason | `mdm.search_indexer`、notify | false | 阶段 5 |
| `mdm.supplier.reactivated.v1` | mdm.suppliers | 再启用申请应用时 | supplier_id、code、version_no | `mdm.search_indexer`、notify | false | 阶段 5 |
| `mdm.material.effective.v1` | mdm.materials | CREATION 变更申请应用且物料首次生效时 | material_id、code、version_no | `mdm.search_indexer` | false | 阶段 5 |
| `mdm.material.updated.v1` | mdm.materials | 已生效物料的 CHANGE 申请应用时 | material_id、code、version_no、changed_fields | `mdm.search_indexer` | false | 阶段 5 |
| `mdm.material.deactivated.v1` | mdm.materials | 停用申请应用且全部使用探针通过时 | material_id、code、version_no、reason | `mdm.search_indexer`、notify | false | 阶段 5 |
| `mdm.material.reactivated.v1` | mdm.materials | 再启用申请应用时 | material_id、code、version_no | `mdm.search_indexer` | false | 阶段 5 |
| `mdm.product.effective.v1` | mdm.products | CREATION 变更申请应用且产品首次生效时 | product_id、code、version_no、costing_mode、inventory_material_id | `mdm.search_indexer` | false | 阶段 5 |
| `mdm.product.updated.v1` | mdm.products | 已生效产品的 CHANGE 申请应用时 | product_id、code、version_no、changed_fields、costing_mode、inventory_material_id | `mdm.search_indexer` | false | 阶段 5 |
| `mdm.product.deactivated.v1` | mdm.products | 停用申请应用且全部使用探针通过时 | product_id、code、version_no、reason | `mdm.search_indexer`、notify | false | 阶段 5 |
| `mdm.product.reactivated.v1` | mdm.products | 再启用申请应用时 | product_id、code、version_no、costing_mode、inventory_material_id | `mdm.search_indexer` | false | 阶段 5 |
| `mdm.warehouse.effective.v1` | mdm.warehouses | CREATION 变更申请应用且仓库首次生效时 | warehouse_id、code、version_no、default_receiving、default_shipping | `mdm.search_indexer` | false | 阶段 5 |
| `mdm.warehouse.updated.v1` | mdm.warehouses | 已生效仓库的 CHANGE 申请应用时 | warehouse_id、code、version_no、changed_fields | `mdm.search_indexer` | false | 阶段 5 |
| `mdm.warehouse.deactivated.v1` | mdm.warehouses | 停用申请应用且库存余额检查通过时 | warehouse_id、code、version_no、reason | `mdm.search_indexer`、notify | false | 阶段 5 |
| `mdm.warehouse.reactivated.v1` | mdm.warehouses | 再启用申请应用时 | warehouse_id、code、version_no | `mdm.search_indexer` | false | 阶段 5 |
| `mdm.import_batch.completed.v1` | mdm.import_batches | 第二遍逐行落草稿结束并写最终批次统计时 | import_batch_id、object_type、total_rows、drafted_rows、failed_rows、error_report_attachment_object_id | notify | false | 阶段 5 |

### 4.3 cpq 段

| 事件类型 | 聚合类型 | 触发时点 | `payload` 关键字段 | 消费者 | `produces_voucher` | 登记阶段 |
|---|---|---|---|---|---|---|
| `cpq.price_list.effective.v1` | cpq.price_lists | CREATION 申请应用且价目表首次生效时 | price_list_id、code、version_no、effective_from_date、effective_to_date | search、notify | false | 阶段 5 |
| `cpq.price_list.updated.v1` | cpq.price_lists | 已生效价目表的 CHANGE 申请应用时 | price_list_id、code、version_no、changed_fields | search、notify | false | 阶段 5 |
| `cpq.price_list.expired.v1` | cpq.price_lists | 到期扫描把 EFFECTIVE 原子置为 EXPIRED 时 | price_list_id、code、version_no、expired_on | search、notify | false | 阶段 5 |

### 4.4 clm 段

| 事件类型 | 聚合类型 | 触发时点 | `payload` 关键字段 | 消费者 | `produces_voucher` | 登记阶段 |
|---|---|---|---|---|---|---|
| `clm.contract.submitted.v1` | clm.contracts | DRAFT 提交进入 PENDING_APPROVAL 时 | contract_id、contract_version_no、doc_no、flow_instance_id | notify | false | 阶段 6 |
| `clm.contract.rejected.v1` | clm.contracts | 审批或签署拒绝使合同进入 REJECTED 时 | contract_id、contract_version_no、doc_no、reason、approval_ref | notify | false | 阶段 6 |
| `clm.contract.signature_requested.v1` | clm.contracts | 全部审批通过并建立签署请求时 | contract_id、contract_version_no、signature_request_id、provider_code | integration-gateway、notify | false | 阶段 6 |
| `clm.contract.signed.v1` | clm.contracts | 电子签章验签通过或实体用印证据登记完成时 | contract_id、contract_version_no、signed_attachment_object_id、evidence_hash | notify、search | false | 阶段 6 |
| `clm.contract.effective.v1` | clm.contracts | 生效动作全部守卫通过并原子进入 EFFECTIVE 时 | contract_id、contract_version_no、doc_no、customer_id、derivation_batch_no | `clm.derivation`、`project.contract_derivation`、search、notify | false | 阶段 6 |
| `clm.contract.derivation_completed.v1` | clm.contracts | 派生项全部 DONE、合同进入 IN_PERFORMANCE 时 | contract_id、contract_version_no、derivation_id、item_total | notify、search | false | 阶段 6 |
| `clm.contract.completed.v1` | clm.contracts | 履约与结清守卫全部通过、合同进入 COMPLETED 时 | contract_id、contract_version_no、doc_no、completed_at | notify、search | false | 阶段 6 |
| `clm.contract.voided.v1` | clm.contracts | DRAFT 或 REJECTED 且无派生记录时作废 | contract_id、contract_version_no、doc_no、void_reason | notify、search | false | 阶段 6 |
| `clm.contract.terminated.v1` | clm.contracts | IN_PERFORMANCE，或派生失败的 EFFECTIVE，经 TERMINATION 审批进入 TERMINATING 时 | contract_id、contract_version_no、doc_no、terminate_reason、approval_ref、terminated_at | 唯一消费者 `platform.impact_assess` | false | 阶段 6（F-10） |
| `clm.contract.termination_completed.v1` | clm.contracts | 影响面批次 DONE 且 item_done=item_total，合同进入 TERMINATED 时 | contract_id、impact_assessment_id、item_total、completed_at | notify、search | false | 阶段 6（F-10；最终验收阶段 12） |

### 4.5 sales 段

| 事件类型 | 聚合类型 | 触发时点 | `payload` 关键字段 | 消费者 | `produces_voucher` | 登记阶段 |
|---|---|---|---|---|---|---|
| `sales.sales_order.created.v1` | sales.sales_orders | 合同派生或人工建单事务写入新订单时 | sales_order_id、doc_no、contract_id、customer_id、order_type | search、notify | false | 阶段 6 |
| `sales.sales_order.released.v1` | sales.sales_orders | 信用与库存守卫通过、订单进入 RELEASED 时 | sales_order_id、doc_no、contract_id、released_at | search、notify | false | 阶段 6 |
| `sales.sales_order.changed.v1` | sales.sales_orders | 订单变更审批通过、版本快照与需求重算同事务完成时 | sales_order_id、doc_no、from_version_no、to_version_no、change_id | search、notify | false | 阶段 6 |
| `sales.sales_order.closed.v1` | sales.sales_orders | 剩余量按关闭原因原子关闭时 | sales_order_id、doc_no、close_reason_code、closed_at | search、notify | false | 阶段 6 |
| `sales.sales_order.cancelled.v1` | sales.sales_orders | 零交付订单取消并释放销售需求时 | sales_order_id、doc_no、cancel_reason、cancelled_at | search、notify | false | 阶段 6 |
| `sales.delivery.confirmed.v1` | sales.delivery_confirmations | 库存腿、适用的过渡科目腿、凭证或合法全零 Skipped 结果及订单需求回写同事务成功时 | delivery_confirmation_id、doc_no、sales_order_id、customer_id、contract_id、voucher_id?（仅全部会计效果为零时空）、lines（含 quantity、allocation_quantity_before、net_unit_price、is_tax_included、tax_rate、revenue_amount、gross_amount、cogs_amount?） | `clm.milestone_confirm`、service、reporting、search | true | 阶段 6 |
| `sales.sales_return.registered.v1` | sales.sales_returns | 销售退货登记事务已同步完成适用的库存入库、凭证或合法全零 Skipped、未开票应收与销售回写后；本事件只作派生通知，不触发库存或财务补写 | sales_return_id、doc_no、sales_order_id、customer_id、source_ref?、is_drop_ship、voucher_id?（仅全部会计效果为零时空）、lines（每行含 sales_return_line_id、sales_order_line_id、item_kind、item_id、costing_mode、inventory_material_id、quantity、warehouse_id、batch_no、serial_nos、revenue_amount、inventory_return_amount?、stock_movement_id?、delivery_links；每个 link 含 delivery_confirmation_line_id、quantity、assigned_by） | service、reporting、search、notify；均不得写 inventory、ledger、finance 或 sales 权威表 | true | 阶段 6 |
| `sales.sales_return.closed.v1` | sales.sales_returns | REGISTERED 迁到 CLOSED 时 | sales_return_id、doc_no、sales_order_id、source_ref?、closed_at | `service.return_repair_writeback`、notify | false | 阶段 6 |
| `sales.sales_return.cancelled.v1` | sales.sales_returns | DRAFT 或 SUBMITTED 在登记前迁到 CANCELLED 时；REGISTERED 不可取消 | sales_return_id、doc_no、sales_order_id、source_ref?、cancel_reason | `service.return_repair_writeback`、notify | false | 阶段 6 |
| `sales.sales_return.rejected.v1` | sales.sales_returns | SUBMITTED 因审批驳回退回 DRAFT 时 | sales_return_id、doc_no、sales_order_id、source_ref?、reject_reason、approval_ref | `service.return_repair_writeback`、notify | false | 阶段 6 |

### 4.6 procure 段

| 事件类型 | 聚合类型 | 触发时点 | `payload` 关键字段 | 消费者 | `produces_voucher` | 登记阶段 |
|---|---|---|---|---|---|---|
| `procure.purchase_requisition.created.v1` | procure.purchase_requisitions | 四类来源经统一 intake 首次成功创建采购需求时 | purchase_requisition_id、doc_no、source_type、source_doc_id、warehouse_id、material_id、required_quantity | notify、search | false | 阶段 7 |
| `procure.purchase_requisition.closed.v1` | procure.purchase_requisitions | 未结需求由人工或来源失效原子进入 CLOSED 时 | purchase_requisition_id、doc_no、source_type、close_reason、closed_at | notify、search | false | 阶段 7 |
| `procure.purchase_order.submitted.v1` | procure.purchase_orders | 草稿或驳回订单提交且进入审批时 | purchase_order_id、doc_no、supplier_id、approval_ref、total_amount | notify、search | false | 阶段 7 |
| `procure.purchase_order.issued.v1` | procure.purchase_orders | 无审批链直接下达或审批通过后进入 ISSUED/PENDING_SUPPLIER_CONFIRM 时 | purchase_order_id、doc_no、supplier_id、issued_at、lines | notify、portal、search | false | 阶段 7 |
| `procure.purchase_order.reschedule_proposed.v1` | procure.purchase_orders | 供应商提交逐行建议交期并进入 SUPPLIER_RESCHEDULE_PROPOSED 时 | purchase_order_id、doc_no、supplier_id、proposed_lines、reason | notify、portal | false | 阶段 7 |
| `procure.purchase_order.supplier_confirmed.v1` | procure.purchase_orders | 供应商确认原交期或采购主管接受建议交期后进入 SUPPLIER_CONFIRMED 时 | purchase_order_id、doc_no、supplier_id、confirmed_at、lines | notify、portal、search | false | 阶段 7 |
| `procure.goods_receipt.posted.v1` | procure.goods_receipts | 既有收货单、库存两账、GRNI、凭证或合法全零 Skipped 与采购回写同事务成功时 | goods_receipt_id、doc_no、purchase_order_id、supplier_id、voucher_id?（仅全部会计效果为零时空）、lines | notify、portal、search、reporting | true | 阶段 7 |
| `procure.receipt_rejection.registered.v1` | procure.receipt_rejections | 拒收单与采购订单引用、审计同事务登记成功时 | receipt_rejection_id、doc_no、purchase_order_id、supplier_id、reason_code、rejected_quantity | supplier-quality、notify、search | false | 阶段 7 |
| `procure.purchase_return.submitted.v1` | procure.purchase_returns | 采购退货草稿提交并进入审批，或无审批链准备直接过账时 | purchase_return_id、doc_no、supplier_id、approval_ref、return_date、lines | notify | false | 阶段 7 |
| `procure.purchase_return.posted.v1` | procure.purchase_returns | 既有采购退货完成适用分支并原子进入 POSTED 时：物料类须完成库存两账及物理凭证或合法全零 Skipped，已开票/直接费用分段另须按原票完成一至多张链接进项红字；直运类不要求本方库存 | purchase_return_id、doc_no、supplier_id、physical_return_voucher_id?（仅物理三项会计效果全零或非物理场景时空）、purchase_credit_note_voucher_ids[]（按原票 id 分组、去重排序）、lines；数组为空当且仅当本次无 billed allocation | supplier-quality、notify、portal、reporting | true | 阶段 7 |
| `procure.payment_request.submitted.v1` | procure.payment_requests | 付款申请提交、占用与审批实例同事务建立时 | payment_request_id、doc_no、supplier_id、payment_type、requested_amount、approval_ref | notify | false | 阶段 7 |
| `procure.payment_request.approved.v1` | procure.payment_requests | 审批链完成且付款申请原子进入 APPROVED 时 | payment_request_id、doc_no、supplier_id、requested_amount、approved_at、approval_ref | notify、阶段 10 付款候选 | false | 阶段 7 |

### 4.7 inventory 段

| 事件类型 | 聚合类型 | 触发时点 | `payload` 关键字段 | 消费者 | `produces_voucher` | 登记阶段 |
|---|---|---|---|---|---|---|
| `inventory.stock_movement.posted.v1` | inventory.stock_movements | 任一来源用例首次经库存过账端口写 movement 与两账成功时；同摘要来源重放不重复登记 | stock_movement_id、source_document_type/id、direction、reason、business_date、accounting_period_id、lines；line 含 posting_line_key、source_document_line_id/no、warehouse_id、material_id、batch_no、quantity、amount、pricing_branch 并按键升序。IN 为正数量/正入库幅值；OUT 为负数量/负账面出库幅值；VALUE_ADJUST 固定 batch_no=`-`、quantity=0、amount=on_hand_variance_amount、branch=VARIANCE_ON_HAND，issued-only/全零为 0 | reporting | false | 阶段 8 |

`inventory.stock_value_adjusted.v1`、`inventory.stock_value.adjusted.v1` 与 `inventory.stock_movement.value_adjusted.v1` 均为已撤销名称，只为裁定迁移审计保留在本段说明中；它们不是现行登记行、不得产生、不得消费，也不得进入运行期注册表。只影响金额账的价差效果已由 `PostingPort` 同事务调用 `CostCaptureService` 直接捕获，禁止再建异步补记通道。

### 4.8 project 段

| 事件类型 | 聚合类型 | 触发时点 | `payload` 关键字段 | 消费者 | `produces_voucher` | 登记阶段 |
|---|---|---|---|---|---|---|
| `project.project.created.v1` | project.projects | 手工新建项目或合同派生首次建立项目时 | project_id、doc_no、customer_id、source_contract_id、source | search、notify | false | 阶段 12 |
| `project.project.completed.v1` | project.projects | 全部任务终态且项目进入 COMPLETED 时 | project_id、doc_no、completed_at | search、notify | false | 阶段 12 |
| `project.project.closed.v1` | project.projects | 全部任务终态且项目进入 CLOSED 时 | project_id、doc_no、closed_at | search、notify | false | 阶段 12 |
| `project.project_task.derived.v1` | project.project_tasks | 合同派生 NEW/CHANGED 项建立新任务时 | project_id、project_task_id、contract_id、contract_version_no、obligation_key、derivation_unique_key | search、notify | false | 阶段 12 |
| `project.project_task.requisition_requested.v1` | project.project_tasks | 任务原子进入 requisition_link_state=PENDING 且 Outbox 条目同事务写入时 | project_id、project_task_id、source_contract_id（可空）、material_id、quantity、required_on、unique_key（固定 `PROJECT_TASK:{project_task_id}`） | 唯一消费者 `project.requisition_intake` | false | 阶段 12 |

### 4.9 service 段

| 事件类型 | 聚合类型 | 触发时点 | `payload` 关键字段 | 消费者 | `produces_voucher` | 登记阶段 |
|---|---|---|---|---|---|---|
| `service.equipment_record.created.v1` | service.equipment_records | 手工或交付批次创建设备档案时 | equipment_record_id、code、customer_id、product_id、delivery_confirmation_id | search、notify | false | 阶段 12 |
| `service.equipment_record.status_changed.v1` | service.equipment_records | 受控状态字典校验通过并变更设备状态时 | equipment_record_id、from_status_code、to_status_code、reason | search、notify | false | 阶段 12 |
| `service.equipment_record.warranty_updated.v1` | service.equipment_records | 项目经理更新保修信息时 | equipment_record_id、warranty_start_on、warranty_end_on、reason | search、notify | false | 阶段 12 |
| `service.customer_complaint.registered.v1` | service.customer_complaints | 投诉登记为 REGISTERED 时 | complaint_id、doc_no、customer_id、complaint_on | search、notify | false | 阶段 12 |
| `service.customer_complaint.accepted.v1` | service.customer_complaints | 投诉受理并写 accepted_by 时 | complaint_id、doc_no、accepted_by、accepted_at | search、notify | false | 阶段 12 |
| `service.customer_complaint.closed.v1` | service.customer_complaints | 填写处理说明并关闭投诉时 | complaint_id、doc_no、handling_note、closed_at | search、notify | false | 阶段 12 |
| `service.customer_complaint.cancelled.v1` | service.customer_complaints | 项目经理填写原因并取消投诉时 | complaint_id、doc_no、cancel_reason、cancelled_at | search、notify | false | 阶段 12 |
| `service.customer_complaint.escalated.v1` | service.customer_complaints | 投诉唯一升级工单事务成功时 | complaint_id、work_order_id、work_order_doc_no | search、notify | false | 阶段 12 |
| `service.work_order.created.v1` | service.work_orders | 任一入口创建工单或创建返修跟进工单时 | work_order_id、doc_no、customer_id、source_complaint_id、follow_up_of_work_order_id | search、notify | false | 阶段 12 |
| `service.work_order.submitted.v1` | service.work_orders | DRAFT 迁到 PENDING_ACCEPTANCE 时 | work_order_id、doc_no、submitted_at、due_at | search、notify | false | 阶段 12 |
| `service.work_order.assigned.v1` | service.work_orders | 写入受理人并进入 IN_PROGRESS 时 | work_order_id、doc_no、assignee_user_id、accepted_at | search、notify | false | 阶段 12 |
| `service.work_order.customer_confirmation_requested.v1` | service.work_orders | IN_PROGRESS 迁到 PENDING_CUSTOMER_CONFIRM 时 | work_order_id、doc_no、requested_at | search、notify | false | 阶段 12 |
| `service.work_order.processing_resumed.v1` | service.work_orders | PENDING_CUSTOMER_CONFIRM 迁回 IN_PROGRESS 时 | work_order_id、doc_no、resumed_at | search、notify | false | 阶段 12 |
| `service.work_order.completed.v1` | service.work_orders | G1/G2 守卫通过并进入 COMPLETED 时 | work_order_id、doc_no、conclusion_note、completed_at | search、notify | false | 阶段 12 |
| `service.work_order.cancelled.v1` | service.work_orders | G3 守卫通过并进入 CANCELLED 时 | work_order_id、doc_no、cancel_reason、cancelled_at | search、notify | false | 阶段 12 |
| `service.work_order.follow_up_created.v1` | service.work_orders | COMPLETED 原工单创建新的 DRAFT 返修跟进单时 | source_work_order_id、follow_up_work_order_id、follow_up_doc_no | search、notify | false | 阶段 12 |
| `service.work_order_line.registered.v1` | service.work_order_lines | RETURN/EXCHANGE 登记行提交，或 REPAIR 登记行建立时 | work_order_id、work_order_line_id、handling_method、quantity、sales_order_line_id、return_posting_date（REPAIR 为空）、return_warehouse_id（可空）、replacement_delivery_schedule_id | 唯一消费者 `service.return_repair_writeback` | false | 阶段 12 |
| `service.work_order_line.linked.v1` | service.work_order_lines | RETURN/EXCHANGE 权威销售关联建立并进入 LINKED 时 | work_order_id、work_order_line_id、sales_return_id、sales_return_line_id、replacement_delivery_schedule_id | search、notify | false | 阶段 12 |
| `service.work_order_line.completed.v1` | service.work_order_lines | REPAIR 人工完成或退换两侧终态守卫满足时 | work_order_id、work_order_line_id、handling_method、completed_at | search、notify | false | 阶段 12 |
| `service.work_order_line.voided.v1` | service.work_order_lines | 项目经理填写原因并把非终态登记行置 VOIDED 时 | work_order_id、work_order_line_id、void_reason、voided_at | search、notify | false | 阶段 12 |

### 4.10 finance 段（F-50 开发前登记）

| 事件类型 | 聚合类型 | 触发时点 | `payload` 关键字段 | 消费者 | `produces_voucher` | 登记阶段 |
|---|---|---|---|---|---|---|
| `finance.receipt.registered.v1` | finance.receipts | 到款、核销/预收效果、凭证与勾稽同事务成功后 | receipt_id、customer_id、amount、settled_amount、advance_amount、cash_account_id、voucher_id | notify、reporting、客户 360 | true | 阶段 10 |
| `finance.payment.registered.v1` | finance.payments | 付款、核销/预付效果、凭证与勾稽同事务成功后 | payment_id、supplier_id、amount、settled_amount、advance_amount、payment_request_id、voucher_id | notify、reporting、portal | true | 阶段 10 |
| `finance.refund.registered.v1` | finance.refunds | 退款或返款、逐来源链接效果、凭证与勾稽同事务成功后 | refund_id、refund_type、party_id、refund_amount、source_links（link_id、source_doc_type/id、linked_amount、advance_consumed_amount、settlement_released_amount）、voucher_id、posting_date、accounting_period_id | notify、reporting | true | 阶段 10（F-50） |
| `finance.cash_document.reversed.v1` | finance.cash_document_reversals | 到款、付款、退款或返款按锁后去向动态拆分并同事务过账成功后 | reversal_id、source_doc_type/id、amount、ar_ap_amount、advance_amount、voucher_id、posting_date、accounting_period_id | notify、reporting、reconciliation | true | 阶段 10（F-50） |
| `finance.overbilling_entry.settled.v1` | finance.overbilling_entries | 超量开票三条结清路径任一条完成时 | overbilling_entry_id、settlement_path、settled_quantity、settled_amount、open_amount、open_quantity、status、voucher_id、posting_date、accounting_period_id | notify、reporting | true | 阶段 10 |

### 4.11 ledger 段（F-50 开发前登记）

| 事件类型 | 聚合类型 | 触发时点 | `payload` 关键字段 | 消费者 | `produces_voucher` | 登记阶段 |
|---|---|---|---|---|---|---|
| `ledger.voucher.posted.v1` | ledger.vouchers | PostingPort 完成凭证、分录与余额更新的同一业务事务时 | voucher_id、doc_no、source_kind、source_document_type/id、business_date、accounting_period_id、total_debit_amount、total_credit_amount | reporting、audit-search | false | 阶段 9 |
| `ledger.correction_voucher.posted.v1` | ledger.correction_vouchers | 更正凭证头行、生成凭证、余额更新与审计同事务成功后 | correction_voucher_id、source_voucher_id、generated_voucher_id、reason、posting_date、accounting_period_id | reporting、audit-search | false | 阶段 9（F-50） |
| `ledger.period_close.requested.v1` | ledger.period_close_requests | 关账请求、重新认证/审批引用、审计与 Outbox 同事务创建时 | period_close_request_id、accounting_period_id、requested_by、approval_ref | approval、notify | false | 阶段 9 |
| `ledger.period_close.accepted.v1` | ledger.period_close_requests | 关账两项受理前提通过且请求原子进入 ACCEPTED 时 | period_close_request_id、accounting_period_id、accepted_at | 关账执行器、notify | false | 阶段 9 |
| `ledger.period_close.acceptance_rejected.v1` | ledger.period_close_requests | 关账任一受理前提不满足并固化拒绝事项时 | period_close_request_id、accounting_period_id、refusal_reasons、rejected_at | notify、运维中心 | false | 阶段 9 |
| `ledger.period_close.concluded.v1` | ledger.period_close_requests | 关账校验形成 PASSED/FAILED_DISCREPANCY/FAILED_INCOMPLETE 结论并释放 slot 时 | period_close_request_id、accounting_period_id、conclusion、concluded_at、discrepancy_refs | notify、reporting、运维中心 | false | 阶段 9 |
| `ledger.period_close.cancelled.v1` | ledger.period_close_requests | 独立重新认证与 `LEDGER_PERIOD_CLOSE/action=CANCEL` 审批通过，请求原子进入 CANCELLED 并释放适用 slot 时 | period_close_request_id、accounting_period_id、cancellation_reauth_ref、cancellation_approval_ref、cancelled_at（=concluded_at）、cancelled_by | notify、运维中心 | false | 阶段 9 |
| `ledger.year_end_closing.requested.v1` | ledger.year_end_closings | 年结请求、重新认证/审批引用、审计与 Outbox 同事务创建时 | year_end_closing_id、fiscal_year、accounting_period_id、sequence_no、approval_ref | approval、notify | false | 阶段 9 |
| `ledger.year_end_closing.executed.v1` | ledger.year_end_closings | 年结锁后余额、0/1/2 张受控凭证与余额图完整提交并进入 EXECUTED 时；FAILED 不产出本事件 | year_end_closing_id、fiscal_year、accounting_period_id、profit_loss_nonzero_account_count_before、profit_loss_net_balance_before_amount、pl_carry_voucher_id（条件可空）、retained_earnings_voucher_id（条件可空）、executed_at | notify、reporting、audit-search | false | 阶段 9 |

### 4.12 invoice 段（F-50 开发前登记）

| 事件类型 | 聚合类型 | 触发时点 | `payload` 关键字段 | 消费者 | `produces_voucher` | 登记阶段 |
|---|---|---|---|---|---|---|
| `invoice.invoice_application.submitted.v1` | invoice.invoice_applications | 发票申请提交并进入审批时 | invoice_application_id、contract_id、customer_id、issue_ratio、application_amount | notify、reporting | false | 阶段 10 |
| `invoice.invoice_application.approved.v1` | invoice.invoice_applications | 审批链全部通过时 | invoice_application_id、remaining_issue_ratio、approval_ref | notify | false | 阶段 10 |
| `invoice.sales_invoice.issued.v1` | invoice.sales_invoices | 销项头行、号码登记、应收、凭证与勾稽同事务成功后 | sales_invoice_id、invoice_number_registry_id、customer_id、contract_id、net_amount、tax_amount、gross_amount、advance_auto_applied_amount、lines、receivable_entry_id、voucher_id、posting_date、accounting_period_id | notify、reporting、search、客户 360 | true | 阶段 10（F-50） |
| `invoice.purchase_invoice.registered.v1` | invoice.purchase_invoices | 进项头行、号码登记、应付、凭证及可选门户受理同事务成功后 | purchase_invoice_id、invoice_number_registry_id、supplier_id、supplier_invoice_upload_id、net_amount、tax_amount、gross_amount、advance_auto_applied_amount、lines、payable_entry_id、voucher_id、posting_date、accounting_period_id | notify、reporting、portal | true | 阶段 10（F-50） |
| `invoice.sales_invoice.reversed.v1` | invoice.invoice_reversals | 销项作废或红字逐行登记、核销释放、凭证与勾稽同事务成功后 | reversal_id、source_sales_invoice_id、reversal_kind、invoice_number_registry_id、net_amount、tax_amount、gross_amount、released_settlement_amount、lines、voucher_id、posting_date、accounting_period_id | notify、reporting、search | true | 阶段 10（F-50） |
| `invoice.purchase_invoice.reversed.v1` | invoice.invoice_reversals | 进项红字逐行登记、核销释放、凭证与勾稽同事务成功后 | reversal_id、source_purchase_invoice_id、reversal_kind、invoice_number_registry_id、net_amount、tax_amount、gross_amount、released_settlement_amount、lines、voucher_id、posting_date、accounting_period_id | notify、reporting | true | 阶段 10（F-50） |
| `invoice.invoice_import_batch.completed.v1` | invoice.invoice_import_batches | 批量导入任务结束并固化统计与结果文件时 | invoice_import_batch_id、total_rows、succeeded_rows、failed_rows、result_object_id | notify | false | 阶段 10 |

### 4.13 portal 段（F-50 开发前登记）

| 事件类型 | 聚合类型 | 触发时点 | `payload` 关键字段 | 消费者 | `produces_voucher` | 登记阶段 |
|---|---|---|---|---|---|---|
| `portal.delivery_notice.submitted.v1` | portal.delivery_notices | 供应商提交送货通知头行并通过订单剩余量校验时 | delivery_notice_id、doc_no、supplier_id、purchase_order_id、expected_arrival_date、lines | notify、待收货投影 | false | 阶段 7 |
| `portal.supplier_invoice_upload.uploaded.v1` | portal.supplier_invoice_uploads | 供应商发票上传头行、附件关联与防重校验同事务成功时 | upload_id、doc_no、supplier_id、invoice_medium、number_scheme、invoice_code、invoice_no、lines | notify、待登记投影 | false | 阶段 7 |
| `portal.supplier_invoice_upload.accepted.v1` | portal.supplier_invoice_uploads | 正式进项发票与中央号码创建成功、上传状态原子推进为 ACCEPTED 后 | upload_id、supplier_id、purchase_invoice_id、accepted_at | notify、portal projection | false | 阶段 10（F-50；由进项发票受理事务产生） |
| `portal.supplier_invoice_upload.returned.v1` | portal.supplier_invoice_uploads | 财务填写原因并原子推进为 RETURNED 后 | upload_id、supplier_id、reason、returned_at | notify、portal projection | false | 阶段 7（F-50；由 portal owner 用例产生） |

### 4.14 reporting 段

| 事件类型 | 聚合类型 | 触发时点 | `payload` 关键字段 | 消费者 | `produces_voucher` | 登记阶段 |
|---|---|---|---|---|---|---|
| `reporting.report_object.published.v1` | reporting.report_objects | ENTERPRISE 对象审批通过并原子替换 publication 时 | report_object_id、object_kind、version_no、spec_hash、approval_ref | audit-search、依赖失效扫描、配置发布通道 | false | 阶段 11 |
| `reporting.report_object.deactivated.v1` | reporting.report_objects | ENTERPRISE 对象停用时 | report_object_id、version_no、reason | audit-search、依赖失效扫描、配置发布通道 | false | 阶段 11 |
| `reporting.render_task.completed.v1` | reporting.render_tasks | 渲染产物登记成功或任务终态失败时 | render_task_id、doc_no、task_kind、output_format、row_count、attachment_object_id、outcome | notify | false | 阶段 11 |

## 5. 阶段登记说明

阶段 1 登记的事件数为 0。理由不是遗漏：阶段 1 不写任何 Outbox 条目，不消费任何事件，也不为消费预留钩子（阶段 1 计划第 7.7 节）；登记一个无人产生也无人消费的事件只有维护成本没有判据。

阶段 2 按 02 计划 D-13 登记上表三个事件。三个事件的写出属阶段 3b：本阶段不实现 Outbox，只交付业务、审计与 Outbox 共享同一 `Tx` 的接缝（02 计划第 6 节）；接缝就位前，core-server 在三个写端点的成功路径上以结构化日志记下本应发出的事件类型（`apps/core-server/src/platform/events.rs`），不静默丢事件也不冒充已写出。信封字段按本文件第 2 节，`security_level` 取 40，`posting_date` 与 `accounting_period_id` 取空值。

阶段 4 任务 #21（ep-platform-identity）登记 `platform.user_account.deactivated.v1`：账号停用用例（`crates/platform/identity/src/lifecycle.rs` 的 `EVENT_USER_ACCOUNT_DEACTIVATED`）在同事务撤销全部会话与设备凭据后登记发生。该事件与阶段 3b 的真实 Outbox 写入接缝同批交付、同批验收；不提供 `PendingEventRecorder`、结构化日志占位或其他替身。

阶段 4 任务 #23 补登记其余三个事件：`platform.user_account.locked.v1`、`platform.user_account.transferred.v1`、`platform.breakglass_activation.closed.v1`。04 计划正文只点名 deactivated 与 authz_policy.published.v1 两个，后者的发出属阶段 3b 的 activate 路由，不在本阶段登记面；其余三个按基线第 6.1 节四段命名派生（模块段 platform、聚合段取表名单数 snake 形态、动作段取已完成时态），派生依据逐条见 `apps/core-server/src/platform/events.rs` 的常量注释（锁定取 U-B-14，移交取 §5.4，应急关闭取退出条件 14，规格报告第 9 节 D-2）。三个字面量同在 `events.rs`，登记发生点分别在登录失败锁定分支、移交端点成功路径与应急关闭端点成功路径；三者与阶段 3b 的真实 Outbox 写入接缝同批交付，事务内直接写 Outbox，不保留 `record_pending_emit` 或日志占位路径。

后续阶段按其计划的交付物清单向对应段追加行，每一行都要同时满足本文件第 1 节的命名约束与第 2 节的信封约束。

F-54 对阶段配额做了具名收口：阶段 3 的登记集合精确为 `platform.attachment.published.v1`、`platform.notification.push_requested.v1`、`platform.config_release.released.v1` 三项；阶段 13 的登记集合精确为 `platform.custom_record.created.v1`、`platform.custom_record.updated.v1`、`platform.custom_record.state_changed.v1` 三项。旧“阶段 3 十七项”和“阶段 13 十项”没有可逐字对账的剩余名称，均已撤销，不得创建占位事件或未命名配额。B-09 复核再撤销一条不可构造且与同步成本捕获重复的库存金额调整事件；本目录当前共有 **124 条**具名登记行，`xtask eventcatalog` 只比较这 124 条与代码常量集合。

阶段 14 新增平台事件类型固定为 **0**。归档、备份、容量、恢复、降级与历史迁移的部署级状态只写 `platform_ops` 台账、`platform_audit.audit_events` 与既有指标，不进入业务 Outbox，不建立虚构的系统法人来填充信封。历史数据迁移正式应用与整批冲销只复用各属主模块已经登记的领域事件，并在既有来源引用中携带 `migration_batch_no`；迁移工作流状态本身不登记第二套事件。

## 6. 机器判定

- `xtask eventcatalog` 校验本文件与代码中事件类型登记的一致性，阶段 0 已交付：模块段清单与 `ModuleCode` 逐项比对、四段命名与重复登记判定、登记表与代码字面量双向比对。全部一致退出码 0；判据无被测输入时以退出码 3 报「判定未做出」，不冒充通过。
- 写入侧的运行期判定属阶段 3a：信封字段缺任一必填项即拒绝，`event_type` 未在本目录登记即拒绝。
- 启动自检项 `event-catalog-consistent` 由阶段 3b 注册，届时把本文件与运行期登记面的一致性变成启动时可判定的项。

## 7. 维护纪律

- 一个事件类型只能由一个阶段登记，重复登记即构建失败。
- 已登记的事件类型不得改名。语义变化时升主版本并新增一行，旧行保留并标注其停止产生的版本。
- 事件是对外契约。删除一个事件类型与删除一个 API 端点同级，须按破坏性变更处理。
