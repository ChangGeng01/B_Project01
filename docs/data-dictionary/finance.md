# Finance 数据字典（F-50 冻结）

> **F-57 状态：`REGISTRY_PENDING_REBASELINE`。** F-50 资金/核销/冲销不变量继续有效；旧表数、迁移数和“可直接开发”状态不得覆盖 F-57 的经营账与法定系统边界。
>
> **激活/owner tasks：Tasks 4、20。** 本分册目前不是 F-57 实现权威；Task 4 完成持久化再基线且 Task 20 完成经营财务闭环激活前不得据此实施。

历史状态（F-57 下无效）：曾标为“可直接开发的文档契约”，但尚未执行迁移。旧阶段 10 finance schema 口径为 23 张表、25 个迁移；其中 24 个为 finance 基础迁移，另 1 个是 `V20261019093130__finance_add_deferred_foreign_keys.sql` 双向延迟外键追补迁移。F-50 改列和约束，不新增 finance 表。

## 对象清单

`aging_bucket_definitions`、`cash_accounts`、`receivable_entries`、`payable_entries`、`advance_receipt_entries`、`advance_payment_entries`、`unbilled_ar_entries`、`overbilling_entries`、`overbilling_settlements`、`receipts`、`payments`、`refunds`、`refund_source_payment_links`、`cash_document_reversals`、四张 settlement link、`cash_ledger_entries`、四张资金单据附件关联表，共 23 张。

## cash_accounts 银行字段与账号盲索引

逻辑字段 `bank_name` 与 `bank_account_no` 均为密级 30 且字段级加密：物理列分别为 `bank_name_enc bytea + bank_name_key_ref text`、`bank_account_no_enc bytea + bank_account_no_key_ref text`，不得保留同名明文列；账号另有 `bank_account_no_tail text` 承载掩码末四位。`platform_core.sensitive_field_registry` 必须登记两行且 `is_field_encrypted=true`；银行名不建盲索引，账号按下述唯一规则建盲索引。列表、详情完整查看、导出、重新认证、审批与审计统一按 F-51 U-A-12。

`bank_account_no_bidx bytea null` 存 `derive_blind_key(legal_entity_id, 'finance.cash_accounts.bank_account_no', plaintext)` 返回的完整 `BlindIndex([u8; 32])`。约束 `ck_cash_accounts_bank_account_no_bidx_len` 固定为 `bank_account_no_bidx IS NULL OR octet_length(bank_account_no_bidx) = 32`；唯一约束 `ux_cash_accounts_legal_entity_id_bank_account_no_bidx` 表达同一法人内银行账号不重复。唯一性是业务规则，不是宽度例外；全库盲索引均为 32 字节。

## cash_accounts 迁移撤销 owner audit target

`reverse_migrated_cash_account` 是 ep-app-finance crate-private owner 用例，只允许 Stage 14 MigrationModuleWriter 在同一事务调用。它锁根并重读未结资金依赖：active 根复用停用守卫，写 `is_active=false,deactivated_at=effect_occurred_at,row_version=before+1`；已 inactive 根保持 is_active、deactivated_at 与 row_version；任一未结到款、付款、退款、返款、资金冲正或其他未闭合资金事实存在即拒绝。该用例不删除账户、不改 opening balance 或历史 cash ledger，也不经公开 cash-account 配置型审批再生成第二套流程；权威批准只认 Stage 14 DATA_MIGRATION 的冲销批准、第二批准人与重新认证证据。

同事务新建一条独立 `platform_audit.audit_events` owner fact，action 固定 `FINANCE_CASH_ACCOUNT_MIGRATION_REVERSED`，object_type/id 指 `finance.cash_accounts` 原 APPLY 根，object_version 等于根 after row_version，reason 固定 DATA_MIGRATION_REVERSED。before/after 各恰有 `{schema_version:1,row_version,is_active,deactivated_at}` 四键；schema_version 是 JSON number 1，row_version 是不带前导零的正十进制 JSON string，JSON number 不接受。after 的版本字符串必须与根/object_version 的规范十进制文本逐字相等；真实停用时 before 等于根当前版本减一，已停用保持时 before/after 与根版本逐值相等。deactivated_at 只为 RFC 3339 string 或 JSON null。REVERSE receipt target 固定为 `(platform_audit.audit_events.event_id,owner_audit_event_id)`；R0 另取 event_id=receipt.id/action=DATA_MIGRATION_REVERSED，并让 after.owner_effect_object_type/id 指 owner event。owner event 与 R0 必须 event id 不同、同法人、同 effect_occurred_at；092600 的静态 projection 固定 `{owner_audit,cash_account_after,R0}`，提交时核 action、JSON exact keys、状态/版本/时点、最终根与未结资金守卫。只改根、只写普通审计、复用旧事件或用 R0 兼任 owner fact 均不成立。本契约复用既有 audit 表，不新增 finance 表、迁移文件或领域事件。

## 四张 settlement link

四表均为仅追加金额事实。公共业务列：

| 列 | 类型 | 可空 | 规则 |
|---|---|---:|---|
| effect_kind | text | 否 | `APPLY` 或 `RELEASE` |
| source_doc_type | text | 否 | 各表封闭来源枚举 |
| source_doc_id | uuid | 否 | 来源单据 |
| root_apply_id | uuid | 否 | 根 `APPLY` 行取自身 id；派生行复制根 id |
| reverses_id | uuid | 是 | 根必空；派生行必填，指向本次直接反向的父行 |
| settled_amount | numeric(18,2) | 否 | 严格大于 0，符号不编码在金额中 |
| settled_at | timestamptz | 否 | 业务分配顺序依据 |
| business_date | date | 否 | 检索与账龄日期 |
| accounting_period_id | uuid | 否 | 历史切片唯一期间依据 |
| refund_source_payment_link_id | uuid | 是 | 退款/返款及其资金冲正产生的效果必填，其他来源为空 |

表专有字段与来源枚举：

| 表 | 所属/资金字段 | source_doc_type |
|---|---|---|
| receivable_settlement_links | `receivable_entry_id`、`funding_origin=DIRECT_CASH\|ADVANCE_AUTO`、`funding_receipt_id?`、`funding_advance_receipt_entry_id?` | `RECEIPT/SALES_INVOICE/CUSTOMER_REFUND/INVOICE_REVERSAL/CASH_DOC_REVERSAL` |
| payable_settlement_links | `payable_entry_id`、`funding_origin=DIRECT_CASH\|ADVANCE_AUTO`、`funding_payment_id?`、`funding_advance_payment_entry_id?` | `PAYMENT/PURCHASE_INVOICE/SUPPLIER_REFUND/INVOICE_REVERSAL/CASH_DOC_REVERSAL` |
| advance_receipt_settlement_links | `advance_receipt_entry_id`、`target_type=RECEIVABLE_ENTRY\|CUSTOMER_REFUND\|CASH_DOC_REVERSAL`、`target_id` | `SALES_INVOICE/CUSTOMER_REFUND/INVOICE_REVERSAL/CASH_DOC_REVERSAL` |
| advance_payment_settlement_links | `advance_payment_entry_id`、`target_type=PAYABLE_ENTRY\|SUPPLIER_REFUND\|CASH_DOC_REVERSAL`、`target_id` | `PURCHASE_INVOICE/SUPPLIER_REFUND/INVOICE_REVERSAL/CASH_DOC_REVERSAL` |

`DIRECT_CASH` 根必须指向同法人原到款/付款且 advance id 为空。`ADVANCE_AUTO` 根必须指向同法人预收/预付；若该 advance 不是期初，funding receipt/payment 必须与其资金根一致。所有派生行复制根行资金字段。advance 两表的 `target_type/target_id` 由 NULL-safe 分支 CHECK 和带法人列的条件复合 FK/约束触发器指向 AR/AP 正向主条目、方向相符的退款/返款或资金冲正单；派生行复制根目标，禁止把多态 `target_id` 当作无约束 UUID。

每表同时建立 `(legal_entity_id,所属条目 id,id)` 与 `(legal_entity_id,所属条目 id,root_apply_id,id)` 候选键；所属列依次为 `receivable_entry_id/payable_entry_id/advance_receipt_entry_id/advance_payment_entry_id`。根真实自 FK 固定为 `(legal_entity_id,所属条目 id,root_apply_id) REFERENCES 本表(legal_entity_id,所属条目 id,id) ON DELETE RESTRICT DEFERRABLE INITIALLY DEFERRED`；父长真实自 FK 固定为 `(legal_entity_id,所属条目 id,root_apply_id,reverses_id) REFERENCES 本表(legal_entity_id,所属条目 id,root_apply_id,id) ON DELETE RESTRICT DEFERRABLE INITIALLY DEFERRED`，约束名逐表为 `fk_<table>_root_apply` 与 `fk_<table>_reverses_parent`。finance owner 在 F-50 collect 前预生成本次效果 id、把 root/effect 键纳入 plan，取得 proof 后首根以单条 INSERT 同时写 `id=root_apply_id,effect_kind=APPLY,reverses_id=NULL`；不得先插 NULL 再 UPDATE。NULL-safe CHECK 强制根为 `APPLY/root=self/reverses=NULL`、派生为 `root<>self/reverses NOT NULL`。`DEFERRABLE INITIALLY DEFERRED` 约束触发器强制父子 effect 相反，不跨法人/台账侧/所属条目/根，直接子行合计不超父行，链无环，且每根 `0 <= ΣAPPLY-ΣRELEASE <= root APPLY amount`。`reverses_id` 只用于父链追溯，绝不用于判断符号。迁移/catalog 测试须证明八条 FK 的列序、`confdeltype='r'`、`condeferrable=true`、`condeferred=true`，并逐表从空表单 INSERT 首根后成功 COMMIT。

## receivable_entries / payable_entries

两表共用下列条目形状，party/订单/原票列按 AR、AP 镜像：

| 列 | 类型 | 可空 | 规则 |
|---|---|---:|---|
| entry_kind | text | 否 | `ORIGINAL\|REVERSAL` |
| source_doc_type | text | 否 | AR: `SALES_INVOICE\|INVOICE_REVERSAL\|MIGRATION_OPENING`；AP: `PURCHASE_INVOICE\|INVOICE_REVERSAL\|MIGRATION_OPENING` |
| sales_invoice_id / purchase_invoice_id | uuid | 条件 | 仅原票 `ORIGINAL` 必填 |
| invoice_reversal_id | uuid | 条件 | 仅 `REVERSAL` 必填，同法人唯一 |
| reverses_entry_id | uuid | 条件 | `ORIGINAL` 必空；`REVERSAL` 必填且指向同法人、同 party 的原票 `ORIGINAL` |
| business_date | date | 否 | 原票/冲销/期初的实际业务日 |
| accounting_period_id | uuid | 否 | 本次追加事实的期间 |
| deferred_from_period_id | uuid | 是 | 顺延来源 |
| due_date | date | 否 | 冲销复制父条目 |
| original_amount | numeric(18,2) | 否 | 大于 0；冲销行取本次冲销价税合计 |
| settled_amount | numeric(18,2) | 否 | `S=ΣAPPLY-ΣRELEASE` 同步投影 |
| open_amount | numeric(18,2) | 否 | `row_open=O-S` 同步投影 |

NULL-safe CHECK 固定三种合法形状：原票 `ORIGINAL` 只填对应蓝字票 id，期初 `ORIGINAL` 三个票/父引用全空，发票 `REVERSAL` 只填 `invoice_reversal_id+reverses_entry_id`。每张 owner 表及本表均提供 `(legal_entity_id,id)` 候选键；原票、冲销头与父条目引用全部使用带法人列的复合 FK，不允许单列 UUID 绕过法人边界。延迟触发器按每个主条目强制：

```text
O = ORIGINAL.original_amount
S = sum(APPLY.settled_amount) - sum(RELEASE.settled_amount)
C = sum(child REVERSAL.original_amount)
0 <= S <= O
ORIGINAL.settled_amount = S
row_open = ORIGINAL.open_amount = O - S
0 <= effective_open = O - S - C <= row_open
```

`REVERSAL` 强制 `settled_amount=0,open_amount=original_amount`，不得指向期初或另一冲销行，且从不进核销候选、账龄或信用占用。经营读取只使用 `effective_open`；两个存储投影不用于倒推历史。

## advance_receipt_entries / advance_payment_entries

预收、预付只保存正向创建额，不追加“反向 advance”。公共业务列如下，斜线左右分别为预收/预付字段：

| 列 | 类型 | 可空 | 规则 |
|---|---|---:|---|
| customer_id / supplier_id | uuid | 否 | 往来方 |
| contract_id | uuid | 是 | 仅预收：合同快照 |
| sales_order_id | uuid | 是 | 仅预收：销售订单引用 |
| receipt_plan_line_id | uuid | 是 | 仅预收：收款计划引用 |
| purchase_order_id | uuid | 是 | 仅预付：采购订单引用 |
| payment_plan_line_id | uuid | 是 | 仅预付：付款计划引用 |
| receipt_id / payment_id | uuid | 是 | 原到款/付款资金根；预收使用 `receipt_id`，预付使用 `payment_id`，物理表不得重复建同名列 |
| source_doc_type | text | 否 | 下述封闭枚举 |
| source_doc_id | uuid | 否 | 创建事件单据或期初导入行 id |
| source_settlement_root_id | uuid | 是 | 只指向同侧 `DIRECT_CASH APPLY` 根 |
| business_date | date | 否 | 创建事件业务日 |
| accounting_period_id | uuid | 否 | 追加事实期间 |
| deferred_from_period_id | uuid | 是 | 顺延来源期间 |
| original_amount | numeric(18,2) | 否 | 大于 0 |
| settled_amount | numeric(18,2) | 否 | `net_consumed` 同步投影 |
| open_amount | numeric(18,2) | 否 | `advance_open` 同步投影 |

预付计划列名固定为 `payment_plan_line_id`，资金根列名固定为 `payment_id`；预收对应 `receipt_plan_line_id` 与 `receipt_id`。

- AR advance 来源只允许 `RECEIPT|INVOICE_REVERSAL|CASH_DOC_REVERSAL|MIGRATION_OPENING`。
- AP advance 来源只允许 `PAYMENT|INVOICE_REVERSAL|CASH_DOC_REVERSAL|MIGRATION_OPENING`。
- 直接款项创建时必须有原款项 id、`source_doc_id` 等于该原款项 id，且无 `source_settlement_root_id`；期初的原款项 id 与来源根皆空，`source_doc_id` 保留导入行 id；`INVOICE_REVERSAL/CASH_DOC_REVERSAL` 只在 `DIRECT_CASH` 根释放后创建，原款项 id 与 `source_settlement_root_id` 都必填。两引用均为带法人列的复合 FK，延迟触发器强制来源根为 `APPLY+DIRECT_CASH` 且 funding 款项等于本条原款项 id。

```text
net_consumed = sum(APPLY.settled_amount) - sum(RELEASE.settled_amount)
advance_open = original_amount - net_consumed
0 <= net_consumed <= original_amount
settled_amount = net_consumed
open_amount = advance_open
```

## receipts / payments 投影

`receipts.settled_total`、`payments.settled_total` 是最终追溯到该原款项资金根的 `DIRECT_CASH+ADVANCE_AUTO` AR/AP 根净额投影；`advance_amount/prepaid_amount` 是该资金根下 advance 条目的 `advance_open` 合计；`refunded_amount` 是未冲正 `refund_source_payment_links` 的合计投影，不是可退事实源。全部投影不接受客户端输入，事务末从效果链/current view/来源链接重读。REGISTERED 行的金额恒等式为 `receipt_amount=settled_total+advance_amount+refunded_amount`、`payment_amount=settled_total+prepaid_amount+refunded_amount`；DRAFT/CANCELLED 不套用，REVERSED 行强制三投影与未冲正退款均为零，且唯一冲正单金额等于原款项金额。

## unbilled_ar_entries / overbilling 历史事实与父链

`unbilled_ar_entries`、`overbilling_entries`、`overbilling_settlements` 均保存同一 `ResolvedPeriod` 的 `accounting_period_id/accounting_period_seq/deferred_from_period_id`；历史查询只比较 seq，不比较 UUID，也不从今天的投影倒推。`v_recon_unbilled_ar` 截至目标 seq 从 append-only 事实重算 `Σ(DEBIT.net_amount)-Σ(CREDIT.net_amount)`。`v_recon_overbilling` 截至目标 seq 重算 `Σ(overbilling_entries.original_amount)-Σ(reverses_id IS NULL settlement.settled_amount)+Σ(reverses_id IS NOT NULL settlement.settled_amount)`，禁止读取 `overbilling_entries.open_amount/status`。

`unbilled_ar_entries` 建 `UNIQUE(legal_entity_id,id,customer_id)` 与 `(legal_entity_id,reverses_id,customer_id)` 长复合父外键。NULL-safe CHECK 强制只有 `SALES_INVOICE_REVERSED` 填 reverses；延迟触发器要求父为未反向的 `SALES_INVOICE_ISSUED`、同客户，`contract_id/sales_order_id` 分别 `IS NOT DISTINCT FROM`，方向相反，且同一父的反向子累计 net/gross 不超父值；自指、反向的反向、成环与错祖先均拒绝。

`overbilling_settlements` 建 `UNIQUE(legal_entity_id,overbilling_entry_id,id)` 与 `(legal_entity_id,overbilling_entry_id,reverses_id)` 长复合父外键。普通三路径行 reverses 为空；只有 PATH_THREE 写销冲回行可非空，且普通约束 `ux_overbilling_settlements_legal_entity_id_reverses_id UNIQUE(legal_entity_id,reverses_id)` 唯一；PostgreSQL 默认 `NULLS DISTINCT` 使其同时允许任意多根空值，不建部分索引。延迟触发器要求父为尚未冲回的 PATH_THREE 根，父子同挂账、数量/金额相同且方向相反，并拒绝自指、成环、跨法人或跨挂账祖先。

## refund_source_payment_links

| 列 | 类型 | 可空 | 规则 |
|---|---|---:|---|
| refund_id | uuid | 否 | 同法人退款单 |
| source_doc_type | text | 否 | `RECEIPT\|PAYMENT` |
| source_doc_id | uuid | 否 | 对应原款项 |
| linked_amount | numeric(18,2) | 否 | 大于 0 |
| advance_consumed_amount | numeric(18,2) | 否 | 非负，只读投影 |
| settlement_released_amount | numeric(18,2) | 否 | 非负，只读投影 |

客户退款只能引用同法人、同客户到款；供应商返款只能引用同法人、同供应商付款。同一退款内 `(source_doc_type,source_doc_id)` 唯一。逐行 `linked_amount = advance_consumed_amount + settlement_released_amount`，整单 `refund_amount = sum(linked_amount)`。两项投影由带 source-link 复合外键的效果行聚合，不接受客户端填写。每个 link 只能消耗该原款项可追溯的 `advance_open` 和根净额，不得借用另一 link 容量。

## 条目与视图

四个 current view 为 `v_receivable_current`、`v_payable_current`、`v_advance_receipt_current`、`v_advance_payment_current`。前两个逐 `ORIGINAL` 主条目返回 `row_open/effective_open`，后两个返回 `advance_open`。核销候选、当前账龄、信用占用、付款上限、门户和导出只读这些口径。

八个 finance 自有 `v_recon_*` view 按会计期间序列从追加事实重算：AR/AP 为 `ΣORIGINAL-ΣREVERSAL-ΣAPPLY+ΣRELEASE`，advance 为 `Σcreation-ΣAPPLY+ΣRELEASE`，unbilled/overbilling 用上节公式，cash 两项为期初加截至期间资金腿净额。不比较 UUID 期间 id，不读今天的投影列倒推历史。最新期累计必须等于 current view 合计。inventory 与 GRNI 不建 SQL view，由 `ReconciliationItemQuery` 在同一 `SnapshotCtx` 调两个 owner snapshot port 后组装；阶段 10 最终只读视图总数 19。

## 跨表强制与 RLS

23 张 finance 表全部带 `legal_entity_id`，启用并强制法人 RLS。同 schema 引用使用 `(legal_entity_id,id)` 复合外键；根/父链、AR/AP 冲销父行、退款来源、资金根与投影守恒均由 PostgreSQL CHECK、复合 FK 和延迟约束触发器承担第一道防线。应用在同事务末重读校验，不得把这些规则降级成“数据库或应用二选一”。
