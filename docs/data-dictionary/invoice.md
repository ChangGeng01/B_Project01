# Invoice 数据字典（F-50 冻结）

> **F-57 状态：`REGISTRY_PENDING_REBASELINE`。** F-50 发票不变量继续有效；旧表数、迁移数和“可直接开发”状态不能作为 F-57 执行授权。
>
> **激活/owner tasks：Tasks 4、20。** 本分册目前不是 F-57 实现权威；Task 4 完成持久化再基线且 Task 20 完成发票/经营财务闭环激活前不得据此实施。

历史状态（F-57 下无效）：曾标为“可直接开发的文档契约”，但尚未执行迁移。旧阶段 10 invoice schema 口径为 17 张表、18 个迁移、17 张法人 RLS 表；其中 17 个为 invoice 基础迁移，另 1 个是 `V20261019092430__invoice_add_finance_foreign_keys.sql` 双向延迟外键追补迁移。

## 对象清单

`tax_rate_options`、`invoice_applications`、`invoice_application_receipt_plan_links`、`invoice_application_sales_order_links`、`sales_invoices`、`sales_invoice_lines`、`invoice_reversals`、`invoice_reversal_lines`、`invoice_number_registry`、`invoice_receipt_plan_links`、`invoice_import_batches`、`purchase_invoices`、`purchase_invoice_lines`、`invoice_application_attachments`、`sales_invoice_attachments`、`purchase_invoice_attachments`、`invoice_reversal_attachments`。

四张附件关联表统一为 `owner_id`、`attachment_object_id`、`purpose`、`sort_no` 加公共列；`owner_id` 与法人组成复合外键分别指向 application、sales invoice、purchase invoice、reversal 头，附件复合外键指向 `platform_file.attachment_objects(legal_entity_id,id)`，全部 `ON DELETE RESTRICT`。每表固定 `UNIQUE(legal_entity_id,owner_id,attachment_object_id)` 与 `UNIQUE(legal_entity_id,owner_id,sort_no)`。`V20261019091200__invoice_create_attachment_link_tables.sql` 必须一次创建四表；Stage 14 采购发票 bundle 投影只使用具名 `invoice.purchase_invoice_attachments`。

## 公共金额规则

- 蓝字原票至少一行；税率只在行。
- 金额 `numeric(18,2)`，税率 `numeric(9,6)`，数量/单价 `numeric(18,6)`。
- `gross_amount = net_amount + tax_amount` 无容差；普通税额 half-up，容差最大 0.02。
- 头 `net/tax/gross` 只由行求和；写契约没有头税率或头金额。

## sales_invoice_lines

业务列：`sales_invoice_id`、`line_no`、`sales_order_id`、`sales_order_line_id`、`item_kind`、`item_id`、`uom_code`、`quantity`、`net_unit_price`、`tax_rate`、`net_amount`、`tax_amount`、`gross_amount`。同头行号唯一；建立 `(legal_entity_id,sales_invoice_id,id)` 候选唯一键。

`sales_invoices` 的 `advance_auto_applied_amount` 不是客户端输入或票面金额字段，而是本次登记事务按 F-50 第 3.4 节锁后计算的只读结果；公开创建响应、详情响应与 `invoice.sales_invoice.issued.v1` 必须返回它，不适用时为 `0.00`。它与同一凭证的借预收、贷应收两腿以及 finance 双侧 `APPLY` 效果逐资金根一致。

## invoice_receipt_plan_links

本表是逐期净已开金额的唯一追加事实，业务列为 `contract_id`、`receipt_plan_line_id`、`receipt_plan_period_no`、`sales_invoice_id`、`allocation_kind=ISSUE|VOID|RED_LETTER`、条件可空 `invoice_reversal_id`、`root_allocation_id`、`linked_net_amount` 与 `linked_gross_amount`。建立候选键 `UNIQUE(legal_entity_id,contract_id,sales_invoice_id,receipt_plan_line_id,receipt_plan_period_no,id)`，以及唯一权威根自 FK `fk_invoice_receipt_plan_links_root_allocation (legal_entity_id,contract_id,sales_invoice_id,receipt_plan_line_id,receipt_plan_period_no,root_allocation_id) REFERENCES invoice.invoice_receipt_plan_links(legal_entity_id,contract_id,sales_invoice_id,receipt_plan_line_id,receipt_plan_period_no,id) ON DELETE RESTRICT DEFERRABLE INITIALLY DEFERRED`；单列 root FK、短原票 FK或只靠应用校验均禁止。

ISSUE 根为 `id=root_allocation_id,invoice_reversal_id=NULL`，VOID/RED_LETTER 复制根的法人、合同、原票、计划行与期次并指向对应冲销头。invoice owner 在 F-50 proof 前只预生成根 id，proof 后以单条 INSERT 写首根，禁止先 NULL 后回填。延迟图限制每根累计反向净额/价税合计不超 ISSUE。迁移/catalog 测试必须证明根 FK 的列序、`confdeltype='r'`、`condeferrable=true`、`condeferred=true`，并从空表单条写入自引用 ISSUE 根后成功 COMMIT；错法人、合同、原票、计划行或期次均失败。

## purchase_invoice_lines

业务列固定为：`purchase_invoice_id`、`line_no`、`purchase_order_id`、`purchase_order_line_id`、`goods_receipt_id`、`goods_receipt_line_id`、`cost_kind`、`item_id`、`quantity`、`net_unit_price`、`tax_rate`、`net_amount`、`tax_amount`、`gross_amount`、`accrual_reversal_amount`、`price_variance_in_stock_amount`、`price_variance_released_amount`、`overbilling_amount`。最后四项是服务端只读结果，全部 `NOT NULL DEFAULT 0`；两项 `price_variance_*` 是可正可负的 `numeric(18,2)`，其余为非负 `numeric(18,2)`。不得另建未拆分的总价差列或超量布尔标志作为权威值。

`goods_receipt_id` 与 `goods_receipt_line_id` 由逐项显式判空的 CHECK 强制同空或同非空；`DIRECT_EXPENSE_TYPE` 必须同空，`INVENTORY_TYPE` 可按已收货匹配或超量段取同非空/同空。`INVENTORY_TYPE` 必须有 `item_id`；全部行的 `cost_kind` 必须相同，头表 `cost_kind` 由服务端从行推导并在事务末核对，从而一张进项发票只选择 `PURCHASE_INVOICE_INVENTORY` 或 `PURCHASE_INVOICE_DIRECT_EXPENSE` 一个凭证来源。建立 `(legal_entity_id,purchase_invoice_id,id)` 候选唯一键及同头行号唯一键。

四项结果逐行满足：已匹配暂估回冲只取 `GrniEffectWritebackPort` 返回值；`matched_invoice_net_amount - accrual_reversal_amount = price_variance_in_stock_amount + price_variance_released_amount`；未匹配超量净额进入 `overbilling_amount`，不得伪造 GRNI。直接费用行的四项结果固定为零，其不含税额直接作为 `direct_expense_amount` 交给 ledger。头级同名结果是行值求和的只读投影，不另建可写权威累计。

`purchase_invoices` 的 `advance_auto_applied_amount` 同样是锁后只读结果；公开创建响应、详情响应与 `invoice.purchase_invoice.registered.v1` 必须返回，不适用时为 `0.00`，并与同凭证借应付、贷预付两腿及 finance 双侧效果逐资金根一致。客户端、插件和 Excel 都不得提交上述服务端结果字段。

## invoice_reversals 与 lines

头使用 `direction`、`reversal_kind`、`source_sales_invoice_id`、`source_purchase_invoice_id`、`linked_purchase_return_id uuid NULL`、三项只读汇总及可空 registry id。两种 source invoice id 按方向恰一非空；`VOID` registry 为空，`RED_LETTER` 必填。

`linked_purchase_return_id` 的数据库 CHECK 固定为：字段非空时 `direction='INPUT' AND reversal_kind='RED_LETTER'` 必须为真；全部 `OUTPUT`、`VOID` 与独立进项更正必须为空。非空表示该红字由采购退货用例触发，凭证来源再由原进项行 `cost_kind` 唯一派生：物料类用 `PURCHASE_INVOICE_LINKED_RETURN_REVERSED`，直接费用/直运类用 `PURCHASE_INVOICE_DIRECT_EXPENSE_REVERSED`；两类红字都不得出现库存 MeasureKey。invoice owner 在同一事务内经 procure owner 端口验证所指退货同法人、同供应商；`V20261019090930__procure_add_invoice_foreign_keys.sql` 另在退货头/行与红字头/行四表安装或替换双向 `DEFERRABLE INITIALLY DEFERRED` 约束触发器，使任一侧 direct SQL 也在 COMMIT 重验同法人、供应商、原票头行、期间/记账日、分组唯一、行覆盖、quantity/net/tax/gross 与 GRNI/直运来源图。物料类红字行必须恰好覆盖该退货的已开票分段，直接费用/直运类必须对应同一原成本归集链且累计不超可冲上限；任何 linked 红字都必须反向命中一张 POSTED 退货，禁止孤立、额外或跨退货链接。该字段不建立会制造循环建表依赖的伪物理 FK，也不施加“一张退货全局只能链接一张红字”的错误唯一约束；唯一粒度是每个 `(purchase_return_id,original_purchase_invoice_id)` 恰一张。

该链接字段不是公开 HTTP、插件或 Excel 的写字段；公开进项红字只允许独立更正并拒绝客户端提交此字段。只有采购退货用例经内部 `PurchaseCreditNotePort` 在同一事务写入，公开响应可将它作为只读追溯字段返回。

行上固定 `NOT NULL` 的关键列是 `invoice_reversal_id`、`source_effect_seq`、`quantity_effect_kind`、`pricing_effect_kind`、`quantity`、`tax_rate`、`net_amount`、`tax_amount`、`gross_amount`；两组来源 invoice/line id 是按方向条件可空，不能声明为全部 `NOT NULL`。来源组以 NULL-safe XOR/all-or-none CHECK、默认 `MATCH SIMPLE` 三列复合 FK 和延迟头行一致性触发器约束。允许组合：`REDUCE+ORIGINAL_UNIT_PRICE`、`REDUCE+ADJUSTED`、`NONE+ADJUSTED`；禁止 `NONE+ORIGINAL_UNIT_PRICE`。

`source_effect_seq` 是每条销项或进项来源行各自从 1 开始、无缺口严格递增的冲销效果序号。分别建立 `(legal_entity_id,source_sales_invoice_line_id,source_effect_seq)` 与 `(legal_entity_id,source_purchase_invoice_line_id,source_effect_seq)` 唯一约束；非活动方向的 line id 为空，活动方向必被其中一个约束覆盖。invoice owner 先按统一锁序锁来源行，再分配下一序号。`DEFERRABLE INITIALLY DEFERRED` 约束触发器在提交前按该序号重放同一来源行全部冲销：校验序号连续、四项累计不超原行，并逐条复算 `ORIGINAL_UNIT_PRICE` 的标准金额。只有本行耗尽剩余数量、此前序号从未出现 `ADJUSTED`、且三项金额恰等于原行定标值减此前全部 `ORIGINAL_UNIT_PRICE` 定标值时，才允许本行偏离标准金额吸收末次尾差；已有 `ADJUSTED` 后伪装末次尾差、重复/跳号或任意非末次偏差均由数据库拒绝。该触发器与应用层锁后重算是双重约束，不是二选一。

链接实物退货时，只有 `REDUCE` 数量部分逐父追加 `PURCHASE_CREDIT_NOTE/INCREASE` 重开原 GRNI；`NONE+ADJUSTED` 不写 GRNI，`REDUCE+ADJUSTED` 只按数量重开暂估金额。采购模块必须在同一事务逐条等量等额追加 `PURCHASE_RETURN/DECREASE`，随后以唯一物理来源 `PURCHASE_RETURN_INVENTORY` 按锁后当前账面价值贷库存；部分退货取移动平均价，全数退清取退货前金额余额全额并使库存数量/金额/单价归零，原 GRNI 与库存账面退货额的差额进 COGS。红字与物理凭证合计 GRNI 为零且库存只减少一次。

## invoice_number_registry

| 列 | 类型 | 可空 | 规则 |
|---|---|---:|---|
| invoice_medium | text | 否 | `ELECTRONIC|PAPER` |
| number_scheme | text | 否 | `UNIFIED_20|LEGACY_CODE_NUMBER` |
| invoice_code | text | 条件 | 统一 20 位为空；旧制为 10/12 位 ASCII 数字 |
| invoice_no | text | 否 | 统一制 20 位；旧制 8 位 ASCII 数字 |
| identifier_key | text | 否 | `GENERATED ALWAYS AS (...) STORED` |
| owner_type | text | 否 | `SALES_BLUE|PURCHASE_BLUE|OUTPUT_RED|INPUT_RED` |
| owner_id | uuid | 否 | 指向唯一业务头 |

`legal_entity_id`、`invoice_medium`、`number_scheme`、`invoice_no`、`identifier_key`、`owner_type`、`owner_id` 全部 `NOT NULL`。数据库 CHECK 分别把三组枚举限定为表中列值；制式 CHECK 逐项使用 `IS NULL/IS NOT NULL`，明确要求统一制代码为空且号码为 20 位 ASCII 数字、旧制代码非空且为 10/12 位 ASCII 数字且号码为 8 位 ASCII 数字，不依赖 SQL `UNKNOWN` 拒绝非法行。

唯一键为 `(legal_entity_id,identifier_key)` 与 `(legal_entity_id,owner_type,owner_id)`；另设 `(legal_entity_id,id)` 候选键。业务头用同法人复合 FK 引用，延迟触发器双向核对 owner。号码/代码保留前导零；无权重复响应不带原记录详情。
