# Procure 数据字典（F-51 冻结）

> **F-57 状态：`REGISTRY_PENDING_REBASELINE`。** GRNI 和采购守恒细节可复用；多来源 demand provider、询比价、动态权限、长链 obligation 和旧表/迁移计数须先按 F-57 再基线。
>
> **激活/owner tasks：Tasks 4、20。** 本分册目前不是 F-57 实现权威；Task 4 完成采购持久化再基线且 Task 20 完成采购闭环激活前不得据此实施。

历史状态（F-57 下无效）：曾标为“可直接开发的文档契约”，但尚未执行迁移。旧阶段 7 procure schema 口径为 23 张表；本分册展开 GRNI 承重对象，并汇总采购订单、收货、采购退货三组跨表祖先与效果图。各表完整字段、普通索引、RLS 与迁移文件曾以阶段 7 第 3 节冻结模型为唯一依据。F-57 再基线前，这些内容只作历史输入，不构成实施授权。

## goods_receipt_line_costings

类别：仅追加会计相关表。它是收货分配和已收货未收票（GRNI）子账的唯一事实源；不带 `row_version`、`updated_at`、`updated_by`，不得 UPDATE/DELETE。单价权威仍在 `inventory.stock_value_entries.applied_unit_price`，本表只存数量、金额与期间效果。

| 列 | 类型 | 可空 | 规则 |
|---|---|---:|---|
| id | uuid | 否 | UUIDv7 主键；另有 `(legal_entity_id,id)` 候选唯一键 |
| legal_entity_id | uuid | 否 | RLS 与复合引用的法人键 |
| security_level | smallint | 否 | 默认 20 |
| data_scope_tags | text[] | 否 | 默认空数组 |
| created_at | timestamptz | 否 | 效果追加时点 |
| created_by | uuid | 否 | 主体或固定系统主体 |
| goods_receipt_line_id | uuid | 否 | 同 schema 外键 |
| source_kind | text | 否 | `GOODS_RECEIPT|PURCHASE_RETURN|PURCHASE_INVOICE|PURCHASE_CREDIT_NOTE` |
| source_doc_line_id | uuid | 否 | 来源业务行；同 schema 用 FK，跨 schema 经 owner 契约校验 |
| direction | text | 否 | `INCREASE|DECREASE`，符号不编码在金额中 |
| quantity | numeric(18,6) | 否 | `>= 0`；与 amount 至少一项大于零 |
| amount | numeric(18,2) | 否 | `>= 0`；与 quantity 至少一项大于零；MidpointAwayFromZero 到 2 位 |
| accounting_period_id | uuid | 否 | 只取 `ResolvedPeriod` |
| accounting_period_seq | integer | 否 | 与 period id 同源；截至期间查询唯一排序轴 |
| posting_date | date | 否 | 收货或采购发票的记账日 |
| effect_seq | bigint | 否 | `GENERATED ALWAYS AS IDENTITY`；父子效果的严格先后序号 |
| root_effect_id | uuid | 否 | 根取预生成 UUIDv7 自身 id，派生行沿用根 id |
| reverses_id | uuid | 条件 | 根为空；派生行指向同根、相反方向的直接父效果 |

根只允许 `GOODS_RECEIPT/INCREASE` 且 `root_effect_id=id`，应用预生成 UUIDv7 后以单条 INSERT 同时写 id/root。候选键为 `UNIQUE(legal_entity_id,id)` 与 `UNIQUE(legal_entity_id,goods_receipt_line_id,root_effect_id,id)`；根 FK `(legal_entity_id,root_effect_id)->(legal_entity_id,id)`、长父 FK `(legal_entity_id,goods_receipt_line_id,root_effect_id,reverses_id)->(legal_entity_id,goods_receipt_line_id,root_effect_id,id)` 均为 `ON DELETE RESTRICT DEFERRABLE INITIALLY DEFERRED`，所以空表首根可在 COMMIT 成立而跨法人、跨收货行或跨根父不能命中。派生来源方向固定：采购退货与进项发票为 DECREASE，进项红字为 INCREASE；父子必须同法人、同收货行、同根、方向相反且 `parent.effect_seq < child.effect_seq`，由延迟约束触发器强制，禁止用可能同值的 `created_at` 判序。每个根在每次事务提交点的净数量、净金额均在 `0..=root`，每个父的反向子效果累计不超过父的开放余额；链无环。

索引与唯一键：

- `ux_goods_receipt_line_costings_legal_entity_id_id (legal_entity_id,id)`；
- `ux_goods_receipt_line_costings_le_source_parent UNIQUE NULLS NOT DISTINCT (legal_entity_id,source_kind,source_doc_line_id,reverses_id)`；首版固定 PostgreSQL 16，必须直接使用该语法，不保留部分索引或数据库版本兼容分支；
- `ix_goods_receipt_line_costings_legal_entity_id_goods_receipt_line_id`；
- `ix_goods_receipt_line_costings_le_period_direction (legal_entity_id,accounting_period_seq,direction)`。

## 采购订单头行批次图

`purchase_orders` 提供 `UNIQUE(legal_entity_id,id)`；`purchase_order_lines` 提供 `UNIQUE(legal_entity_id,purchase_order_id,id)` 并以 `(legal_entity_id,purchase_order_id)` 真实复合外键指向订单头；`purchase_order_line_batches` 冗余不可写 `purchase_order_id`，提供 `UNIQUE(legal_entity_id,purchase_order_id,purchase_order_line_id,id)`，以 `(legal_entity_id,purchase_order_id,purchase_order_line_id)` 长复合外键指向订单行。全部删除动作均为 `ON DELETE RESTRICT`。

即时约束固定为：订单行 `received_quantity>=0`、`0<=returned_quantity<=received_quantity`、`0<=invoiced_quantity<=quantity`；OPEN 要求未收满，FULLY_RECEIVED 要求已收满或合法超收；批次累计非负，OPEN/FULLY_RECEIVED 与是否收满同形。`V20261018090500__procure_create_purchase_order_line_batches.sql` 安装三表 `DEFERRABLE INITIALLY DEFERRED` 图触发器，提交时强制每行至少一批、批次数量合计等于订单数量、批次已收合计等于行已收，以及头部不含税/税额/含税金额分别等于行级两位舍入汇总。普通 FK 全命中但跨订单拼行/批次、头金额漂移或累计/状态漂移均不得提交；合法超收只在收货审批图具有完整证据时允许。

## 采购订单与付款申请的迁移撤销 owner audit target

两类撤销不新增 procure correction 表，也不得用根 after-image 直接充当 Stage 14 REVERSE receipt target。`reverse_migrated_purchase_order` 与 `reverse_migrated_payment_request` 是 ep-app-procure crate-private owner 用例；它们锁根、复用现有 VOID/WITHDRAW/CLOSE 与 reservation 释放守卫，并在同事务各新建一条独立 `platform_audit.audit_events` owner fact。订单 action 固定 `PROCURE_PURCHASE_ORDER_MIGRATION_REVERSED`，付款申请 action 固定 `PROCURE_PAYMENT_REQUEST_MIGRATION_REVERSED`；owner event 的 `object_type/object_id` 分别指 `procure.purchase_orders|procure.payment_requests` 原 APPLY 根，`object_version` 等于根 after row_version，`reason='DATA_MIGRATION_REVERSED'`。两者的 before/after 都恰为 `{schema_version:1,row_version,status}`；schema_version 是 JSON number 1，row_version 是不带前导零的正十进制 JSON string，JSON number 不接受。after 的版本字符串必须与根/object_version 的规范十进制文本逐字相等；真实状态改变时 before 等于根当前版本减一，终态保持时 before/after 与根版本逐值相等。

订单状态映射精确为 DRAFT/PENDING_SUPPLIER_CONFIRM/SUPPLIER_RESCHEDULE_PROPOSED→VOIDED，PENDING_APPROVAL/REJECTED/ISSUED/SUPPLIER_CONFIRMED/PARTIALLY_RECEIVED/COMPLETED→CLOSED，CLOSED/VOIDED 保持。付款申请精确为 DRAFT/REJECTED→VOIDED，PENDING_APPROVAL→WITHDRAWN，APPROVED/PARTIALLY_PAID→CLOSED，WITHDRAWN/CLOSED/VOIDED 保持；FULLY_PAID 或仍有付款效果时拒绝。两类实际变更写相应终态 timestamp=同一 `effect_occurred_at`，保持分支保留原 timestamp；业务原因固定 DATA_MIGRATION_REVERSED。

REVERSE receipt 的 target 固定为 `(platform_audit.audit_events.event_id,owner_audit_event_id)`，不再是业务根。R0 仍取 `event_id=receipt.id` 和 action DATA_MIGRATION_REVERSED，其 after.owner_effect_object_type/id 固定指向 owner audit；两个 event id 必须不同且同法人、同 occurred_at。Stage 14 092600 对订单投影 `{owner_audit,purchase_order_after,R0}`、对付款申请投影 `{owner_audit,payment_request_after,R0}`，并在提交时核 exact action、JSON keys、状态边、版本、根最终 after-image 与依赖守卫；普通状态审计、旧事件、R0 复用和半套状态更新均不成立。本契约复用既有 `platform_audit.audit_events`，Stage 7 仍为 23 张 procure 表、33 个迁移文件和 15 个领域事件。

## 收货祖先与效果图

`goods_receipts` 提供 `UNIQUE(legal_entity_id,purchase_order_id,id)`；`goods_receipt_lines` 冗余不可写 `purchase_order_id` 与可空 `delivery_notice_id`，提供 `UNIQUE(legal_entity_id,goods_receipt_id,id)`、`UNIQUE(legal_entity_id,goods_receipt_id,purchase_order_id,purchase_order_line_id,id)`，并以长复合外键同时锁定收货头、采购订单行和可空批次。notice 头行建立后，`V20261018093200__procure_add_portal_foreign_keys.sql` 再补收货头 `(legal_entity_id,purchase_order_id,delivery_notice_id)` 与收货行 `(legal_entity_id,purchase_order_id,purchase_order_line_id,delivery_notice_id,delivery_notice_line_id)` 两条长复合外键及四表延迟 notice 图，禁止跨通知头或错 PO 行拼接。

`ck_goods_receipts_posting_shape` 固定 DRAFT/PENDING_APPROVAL 三项过账证据全空；POSTED/PARTIALLY_RETURNED/FULLY_RETURNED 的 `accounting_period_id/posted_at` 非空，`voucher_id` 由延迟效果图决定。`V20261018091000__procure_create_goods_receipt_line_costings.sql` 在收货头、行、序列、GRNI、订单行和批次安装同一延迟图：供应商、订单、物料、仓库、单价、批次、序列逐值同源；已过账行必须有精确 `PURCHASE_RECEIPT` inventory 数量/金额段，暂估段逐项对应唯一 GRNI 根，超量开票段逐项对应锁定 match；头行、订单行、批次、notice 累计及状态一致。全单 inventory/GRNI/match 会计金额均为零时 `voucher_id` 必空，任一非零时必须存在同法人、同期间且来源为 `PURCHASE_RECEIPT` 的 voucher；DRAFT/PENDING 不得残留任何效果。

## 采购退货祖先与效果图

`purchase_returns` 提供 `UNIQUE(legal_entity_id,id)` 与 `UNIQUE(legal_entity_id,sales_return_id,id)`；`purchase_return_lines` 冗余不可写 `goods_receipt_id/purchase_order_id/purchase_order_line_id/sales_return_id`，提供 `UNIQUE(legal_entity_id,purchase_return_id,id)`，并以长复合外键锁定退货头、精确收货行、该行 costing 与 DROP_SHIP 的同一销售退货行。`purchase_return_line_serials` 冗余退货头键并以长复合外键指向退货行。NULL-safe CHECK 使 MATERIAL_RECEIPT 与 DROP_SHIP 两组祖先列互斥，原发票头行同空同非空。

`V20261018091400__procure_create_purchase_return_line_serials.sql` 安装退货头行序列、收货头行/costing 六表延迟图：MATERIAL_RECEIPT 必须与原收货、PO、供应商、物料、批次、序列完全同源，POSTED 后 inventory movement 与 GRNI decrease 的来源、父链、数量和金额精确；DROP_SHIP 必须属于头上的同一销售退货且不得产生 inventory、GRNI 或 physical voucher。MATERIAL_RECEIPT 的 `grni_consumed_amount+inventory_return_amount+linked_return_price_difference_amount` 全零时 physical voucher 必空，任一非零时必须存在同法人同期间且来源为 `PURCHASE_RETURN_INVENTORY` 的 voucher。

目标发票表建立后，`V20261019090930__procure_add_invoice_foreign_keys.sql` 补 `(legal_entity_id,purchase_invoice_id,purchase_invoice_line_id) -> invoice.purchase_invoice_lines(legal_entity_id,purchase_invoice_id,id) ON DELETE RESTRICT`，`CREATE OR REPLACE` 上述函数并在 procure 退货头行及 invoice 红字头行两侧安装 `DEFERRABLE INITIALLY DEFERRED` 触发器。提交时原票供应商、采购链、销售退货归属及 linked 红字覆盖必须双向完整；已 POSTED DROP_SHIP 无红字、跨退货/多余红字、错误 costing/供应商/sales-return 即使各普通 FK 均命中也必须整笔拒绝。

## 写入与聚合契约

初始收货只把暂估部分写为 `GOODS_RECEIPT/INCREASE`；即使暂估金额为零，只要数量大于零也必须写根。超量开票反向匹配不进 GRNI。采购发票经 `ep_contract_procure::GrniEffectWritebackPort` 写 `PURCHASE_INVOICE/DECREASE`；只有 `quantity_effect_kind=REDUCE` 的进项红字数量撤销才经同一端口写 `PURCHASE_CREDIT_NOTE/INCREASE`，`NONE+ADJUSTED` 的折让、纯金额或纯税额更正不写 GRNI。采购退货由 procure 自有用例写 `PURCHASE_RETURN/DECREASE`：未开票段直接消费开放 INCREASE，已开票段必须在同一事务先由红字重开，再逐条等数量、等金额消费该红字 INCREASE，事务末净效果为零；invoice 模块不可直写本表。部分效果按根原金额比例舍入，吃完剩余数量的最后一段取全部剩余金额并吸收累计尾差；调用方不得传入 GRNI 金额。跨模块统一锁序保证原采购发票早于 GRNI、GRNI 早于库存；先收集、统一加锁、锁后重载，集合漂移整事务重试。

`GrniSubledgerBalancePort::balance(snapshot, legal_entity_id, accounting_period_id, accounting_period_seq)` 只读本表，按 `accounting_period_seq <= target` 聚合 `INCREASE.amount-DECREASE.amount`。结果不得为负，不访问 invoice schema，不以当前状态倒推历史，也不比较期间 UUID。
