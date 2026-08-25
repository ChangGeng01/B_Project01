# Ledger 数据字典（开发就绪冻结）

> **F-57 状态：`REGISTRY_PENDING_REBASELINE`。** 当前产品只认证平衡的内部经营分录、科目映射、试算、子账对账和经营期间控制；法定账簿/税务/工资/法定年结外接。旧“唯一实现口径”与更宽法定总账含义已被取代。
>
> **激活/owner tasks：Tasks 4、20。** 本分册目前不是 F-57 实现权威；Task 4 完成持久化再基线且 Task 20 完成经营账闭环激活前不得据此实施。

历史状态（F-57 下无效）：曾标为“可直接开发的文档契约”，但尚未执行迁移、编译或数据库测试。本文与阶段 9 第 9.3—9.4 节、F-50 只保留为旧实现口径输入，不能据此开工。

## 1. 对象总数、公共列与 RLS

`ledger` schema 固定为 14 张表、2 个视图、18 个迁移。14 张表中只有 `ledger.posting_trigger_event_types` 不带 `legal_entity_id`；其余 13 张全部使用总数据字典公共列并 `ENABLE ROW LEVEL SECURITY`、`FORCE ROW LEVEL SECURITY`。仅追加表去掉 `row_version/updated_at/updated_by`，并由 append-only registry、数据库权限及触发器共同禁止 UPDATE/DELETE。

| # | 表 | 分类 | 专有列与核心约束 |
|---:|---|---|---|
| 1 | `ledger.accounts` | 档案 | `code,name,category,balance_direction,account_level,parent_account_id,is_postable,is_active,deactivated_at`；法人内 code 唯一；层级只允许 1/2；二级必须指向同法人一级科目 |
| 2 | `ledger.accounting_periods` | 业务/会计 | `period_code,fiscal_year,period_no,start_date,end_date,status,is_fiscal_year_last,closed_at,closed_by_close_request_id`；自然月 CHECK 强制 `start=make_date(year,no,1)`、`end=next_month-1 day`、`period_code=YYYYMM`、`is_fiscal_year_last=(period_no=12)`；法人内 `period_code`、`(fiscal_year,period_no)`、`start_date` 各自唯一；最终 OPEN/CLOSED 形状及 CLOSED↔同期间 PASSED request 由三表延迟图证明 |
| 3 | `ledger.event_account_bindings` | 业务 | `account_role,account_id,release_package_id`；法人内每个 AccountRole 恰一绑定；只能绑定启用且可过账科目 |
| 4 | `ledger.opening_balance_batches` | 单据 | `doc_no,status,accounting_period_id,source,migration_batch_no,total_debit_amount,total_credit_amount,confirmed_at,approval_ref`；来源 `MANUAL/MIGRATION_BATCH`；`status<>'CONFIRMED' OR total_debit_amount=total_credit_amount` |
| 5 | `ledger.opening_balance_batch_lines` | 业务 | `opening_balance_batch_id,line_no,account_id,debit_amount,credit_amount`；批次内行号与科目各自唯一；借贷恰一大于零；PENDING/CONFIRMED 非空且行合计等于头，CONFIRMED 后头行明细均不可变 |
| 6 | `ledger.vouchers` | 仅追加/会计 | `doc_no,accounting_period_id,business_date,deferred_from_period_id,source_kind,source_sequence_no,source_document_type,source_document_id,source_document_no,source_event_id,total_debit_amount,total_credit_amount,line_count,reverses_id`；借贷相等、行数至少 2；来源单据幂等唯一；`UNIQUE(legal_entity_id,id)`、普通 `UNIQUE(legal_entity_id,reverses_id)`；只有资金冲正头与 `HISTORICAL_MIGRATION` sequence 2 镜像可带父凭证 |
| 7 | `ledger.voucher_lines` | 仅追加/会计 | `voucher_id,line_no,account_id,account_role,direction,amount,measure_key,accounting_period_id,business_date,reverses_id`；金额大于 0；借贷方向二值；期间与日期和头一致；`UNIQUE(legal_entity_id,id)`、`UNIQUE(legal_entity_id,voucher_id,id)`、普通 `UNIQUE(legal_entity_id,reverses_id)` |
| 8 | `ledger.correction_vouchers` | 仅追加/单据 | `doc_no,source_voucher_id,reason,posting_date,accounting_period_id,generated_voucher_id,initiated_by,posted_at`；类型码 `CORR`；同法人真实 FK；generated voucher 法人内唯一 |
| 9 | `ledger.correction_voucher_lines` | 仅追加 | `correction_voucher_id,pair_no,line_role,line_no,source_voucher_line_id,generated_voucher_line_id,account_id,account_role,direction,amount,memo`；pair 恰为 `REVERSE_ORIGINAL|TARGET` 两行，同额反向；每条证据唯一映射一条生成凭证行；历史只累计 REVERSE_ORIGINAL 且不超过原行金额 |
| 10 | `ledger.account_period_balances` | 业务/会计 | `account_id,accounting_period_id,opening_balance_amount,is_opening_fixed,period_debit_amount,period_credit_amount`；法人+科目+期间唯一 |
| 11 | `ledger.close_serialization_slots` | 业务 | `active_close_request_id,active_slot_key`；key 由指针空性生成；法人首次发起关账或年结时建唯一行，作为二者共享串行化点并与 active request 双向长 FK |
| 12 | `ledger.period_close_requests` | 单据 | `doc_no,accounting_period_id,status,reauth_ref,approval_ref,approved_by,accepted_at,inflight_xids,inflight_wait_completed_at,snapshot_id,snapshot_established_at,conclusion,concluded_at,refusal_reasons,completed_batch_count,termination_cause,cancellation_reauth_ref,cancellation_approval_ref,cancelled_by,active_slot_key,passed_accounting_period_id`；后两列为状态生成证据键；九态形状、独立取消证据、active slot、PASSED period 全由提交点图证明 |
| 13 | `ledger.year_end_closings` | 单据 | `doc_no,fiscal_year,accounting_period_id,status,sequence_no,reauth_ref,approval_ref,approved_by,pl_carry_voucher_id,retained_earnings_voucher_id,executed_at,failure_code,concluded_at,profit_loss_nonzero_account_count_before,profit_loss_net_balance_before_amount`；法人+年度+sequence 唯一；五态、末期、0/1/2 凭证与余额图延迟证明 |
| 14 | `ledger.posting_trigger_event_types` | 全局登记 | 仅 `id,event_type,created_at,created_by`；event_type 唯一；无 `legal_entity_id`、不建 RLS，须登记到 `unpoliced_table_registry` |

必须建立的同法人候选键与复合 FK 包括 `(legal_entity_id,id)`、头行关系、期间关系、科目关系、更正头到原凭证/生成凭证，以及更正证据到原分录行和唯一生成分录行。`voucher_lines` 另建 `(legal_entity_id,voucher_id,id)` 候选键。第 7 号迁移的通用 `DEFERRABLE INITIALLY DEFERRED` 约束触发器先对所有凭证强制 `line_count`、借贷行合计、行期间与日期等于头；普通、年结、受控更正与历史 APPLY 凭证的头行 `reverses_id` 均为空。资金冲正和历史 REVERSE 头行均非空：前者父头必须是未冲正的四类普通现金凭证且子父 source_kind 相同；后者父头必须是同一迁移记录的未冲正 `HISTORICAL_MIGRATION/DATA_MIGRATION_RECORD` sequence 1，子为同一 type/id/no 的 sequence 2。两类父图每行都恰被一条子行完整覆盖，account/account_role/measure_key/amount/line_no 相同而 direction 反向。头、行上的普通 `UNIQUE(legal_entity_id,reverses_id)` 分别禁止同一原凭证或原行再次冲正；父行缺失、跨法人、错凭证、部分金额或自由改腿均整笔回滚。

Stage 14 的 092600 把 `ck_vouchers_source_kind` 扩为 19 项，并把 sequence CHECK 精确冻结为：普通来源/CORRECTION 只取 1，YEAR_END 与 HISTORICAL_MIGRATION 只取 1/2；再替换通用图函数以加入历史完整镜像和 migration receipt 双向证明。历史 sequence 1 必须由 `ledger/historical_voucher` 同法人 record 的 APPLY receipt 以 `target_object_type='ledger.vouchers',target_id=voucher.id` 唯一指向且命中预留；sequence 2 必须由同记录 REVERSE receipt 指向、receipt 指回 APPLY 并有 R0，`business_date=platform_core.business_day(REVERSE receipt.owner_effect_at)`。record/batch/receipt 侧反向验证同一头；不得出现孤立历史凭证或孤立 receipt。092600 仅在历史 voucher/receipt/record/R0 全空时允许回退该分支。

第 9 号迁移的延迟 CORR 图触发器强制每个 pair 恰两行、同一 source line、同额反向，`REVERSE_ORIGINAL` 镜像原 account/role 并反向；源与 `TARGET` 角色都只允许 `MAIN_OPERATING_COST|DIRECT_EXPENSE_COST` 且必须不同，收入与任何跨侧组合不可达。生成行 measure_key 分别固定 `correction_reverse_original|correction_target`。证据与 `generated_voucher_line_id` 的 line/account/role/direction/amount 一一相等，所有生成行都且只属于头的 generated voucher，历史累计只计 REVERSE_ORIGINAL 且不超原行。第 5 号迁移另以延迟图约束强制 PENDING/CONFIRMED 期初批次非空、行借贷合计逐侧等于头且相等，并以即时守卫冻结 CONFIRMED 头和全部行。应用校验均不替代数据库约束。`vouchers`、`voucher_lines`、`correction_vouchers`、`correction_voucher_lines` 四表对运行期角色撤销 UPDATE/DELETE。

第 11、12 号迁移冻结关账三表证据图。slot 的 `active_slot_key=CASE WHEN active_close_request_id IS NULL THEN NULL ELSE 1 END`；request 的 `active_slot_key=1` 仅当 status 为 `ACCEPTED|VALIDATING`，`passed_accounting_period_id=accounting_period_id` 仅当 status 为 `PASSED`。四条双向长 FK 分别为 slot `(legal_entity_id,active_slot_key,active_close_request_id)`↔request `(legal_entity_id,active_slot_key,id)`，以及 period `(legal_entity_id,id,closed_by_close_request_id,closed_at)`↔request `(legal_entity_id,passed_accounting_period_id,id,concluded_at)`；全部 `ON DELETE RESTRICT DEFERRABLE INITIALLY DEFERRED`，候选键都不是部分索引。`assert_period_close_state_graph_consistent()` 在 accounting_periods/request/slot 三表提交点按“法人 slot→期间→request UUID bytes”锁序验证九态完整证据、`completed_batch_count>=0`、时间前缀、OPEN 无关闭证据、active request 与 slot 双向一致、CLOSED 只且必须指同期间 PASSED request 且 `closed_at=concluded_at`。只有 CANCELLED 可且必须带 `cancellation_reauth_ref/cancellation_approval_ref/cancelled_by`，三项来自既有 `LEDGER_PERIOD_CLOSE + action=CANCEL` 的独立批准动作，原申请 reauth/approval 不得覆盖复用，事件 cancelled_at 取 concluded_at；其中 reauth 与 actor 分别以真实 `ON DELETE RESTRICT` FK 指向 challenge 与同法人用户授权，approval ref 走具名平台证明白名单。第 2 号临时关闭形状 CHECK 在图安装时删除；9b 只允许按 13→12→11 逆序回退，第 12 号 down 仅允许空事实并先拆三表触发器/函数/外向 FK、最后恢复该 CHECK，第 11 号再在后置对象全无且无 active 指针时 `DROP TABLE ... RESTRICT`。

第 13 号迁移冻结年结图。`failure_code` 仅允许 `PERIOD_NOT_POSTABLE|ROLE_UNBOUND`；FAILED 必有 failure_code/concluded_at 且不得有执行、凭证或控制字段，基础设施失败回滚并保留 APPROVED。终态转移按“法人 slot→期间→account UUID bytes”锁序验证：EXECUTED 必须期间仍为 OPEN 且本法人 slot 为空；期间非 OPEN 或本法人任一期间仍有 active close request 时只能固化 PERIOD_NOT_POSTABLE，只有期间 OPEN、slot 为空且本次所需角色绑定异常时才可固化 ROLE_UNBOUND，两者同时成立时前者优先，不能只写一个合法枚举伪造失败；期间被 closing 引用后，独立即时 guard 冻结其年月/末期身份且不改写第 12 号状态 guard。两个执行前控制字段仅 EXECUTED 非空且由锁后余额、年结凭证腿与最终余额反推：`count=0` 当且仅当 `net=0` 且两凭证空；`count>0` 当且仅当 sequence 1 的 `pl_carry_voucher_id` 非空；sequence 2 的 `retained_earnings_voucher_id` 非空当且仅当 `net<>0`。因此多个非零损益科目净和为零时仍有第一张、没有第二张。第一张恰逐项反向清零 count 个非零 PROFIT_LOSS 科目，net 非零才带 PROFIT_THIS_YEAR 腿；第二张若存在以 `abs(net)` 清零 PROFIT_THIS_YEAR 并转入 RETAINED_EARNINGS_UNDISTRIBUTED。两头固定 `source_kind=YEAR_END_PL_CLOSING`、`source_document_type=YEAR_END_CLOSING`、source id/no=closing id/doc_no、event/reverses/deferred 均空、期间=closing 期间、business_date=末期 end_date、sequence=1/2；孤立、错槽、错期间、错金额或自由腿均在延迟提交点拒绝。三个延迟 trigger 分别挂在 closing、voucher 头和 voucher_lines；行触发器按 voucher_id 锁读头来判定 YEAR_END 来源，不引用 line 上不存在的 source_kind。

两个视图：

- `ledger.v_account_period_balances`：按法人、科目、期间输出期初、本期借方、本期贷方、期末；同时输出 `security_level/data_scope_tags`，不是物化视图。
- `ledger.v_pending_posting_backlog`：按调用方当前法人和期间统计会产生凭证的 Outbox PENDING/DISPATCHING 与 dead-letter OPEN/REPAIRING；事件集合只取登记表。

## 2. AccountRole（17 项）

| Rust/数据库值 | 会计含义 |
|---|---|
| `ACCOUNTS_RECEIVABLE_UNBILLED` | 应收账款未开票过渡科目 |
| `ACCOUNTS_RECEIVABLE` | 应收账款 |
| `ADVANCE_FROM_CUSTOMER` | 预收账款 |
| `MAIN_OPERATING_REVENUE` | 主营业务收入 |
| `MAIN_OPERATING_COST` | 主营业务成本 |
| `INVENTORY` | 存货 |
| `ACCOUNTS_PAYABLE` | 应付账款 |
| `ACCOUNTS_PAYABLE_ACCRUED` | 应付账款暂估（GRNI） |
| `ADVANCE_TO_SUPPLIER` | 预付账款 |
| `OVERBILLING_SUSPENSE` | 待处理超量开票 |
| `TAX_PAYABLE_OUTPUT` | 应交税费—销项税额 |
| `TAX_PAYABLE_INPUT` | 应交税费—进项税额 |
| `BANK_DEPOSIT` | 银行存款 |
| `CASH_ON_HAND` | 库存现金 |
| `DIRECT_EXPENSE_COST` | 直接费用类成本 |
| `PROFIT_THIS_YEAR` | 本年利润 |
| `RETAINED_EARNINGS_UNDISTRIBUTED` | 未分配利润 |

客户只能把上述角色绑定到本法人科目，不能新增角色或改变映射规则。

## 3. VoucherSourceKind（19 项）

| 来源 | 唯一职责 |
|---|---|
| `DELIVERY_CONFIRMED` | 交付收入及可选销货成本 |
| `SALES_INVOICE_ISSUED` | 销项蓝票及本票预收自动核销 |
| `SALES_INVOICE_REVERSED` | 销项作废/红字及核销释放 |
| `PURCHASE_INVOICE_INVENTORY_REVERSED` | 独立物料进项更正，不链接实物退货 |
| `PURCHASE_INVOICE_DIRECT_EXPENSE_REVERSED` | 直接费用/直运进项更正或退货 |
| `PURCHASE_INVOICE_LINKED_RETURN_REVERSED` | 链接实物采购退货的进项红字；绝不写库存 |
| `RECEIPT_REGISTERED` | 客户到款 |
| `PAYMENT_REGISTERED` | 供应商付款 |
| `CUSTOMER_REFUND` | 客户退款 |
| `SUPPLIER_REFUND` | 供应商返款 |
| `PURCHASE_RECEIPT` | 收货暂估及超量挂账反向匹配入库 |
| `PURCHASE_INVOICE_INVENTORY` | 物料采购发票、GRNI 回冲、价差、超量挂账及预付自动核销 |
| `PURCHASE_INVOICE_DIRECT_EXPENSE` | 直接费用采购发票及预付自动核销 |
| `SALES_RETURN` | 销售退货收入冲回及可选库存成本回冲 |
| `PURCHASE_RETURN_INVENTORY` | 物料实物采购退货；已/未开票/混合均同一来源 |
| `OVERBILLING_WRITTEN_OFF` | 超量挂账审批转当期成本 |
| `YEAR_END_PL_CLOSING` | 年度损益结转，只走年结专用入口 |
| `CORRECTION` | 受控科目重分类，只走 `post_correction` |
| `HISTORICAL_MIGRATION` | Stage 14 已批准历史迁移凭证；只走 ledger migration writer，APPLY/REVERSE 固定 sequence 1/2 |

旧的单一进项冲销来源、按开票状态拆分的三类采购退货来源与采购退货动态 `resolved_source_kind` 均已删除；F-50 后为 18 项，Stage 14 的 092600 再追加 `HISTORICAL_MIGRATION`，终态 19 项。

## 4. JOURNAL_MAP 类型

```rust
struct JournalRule {
    source_kind: VoucherSourceKind,
    measure_key: MeasureKey,
    requiredness: Requiredness,
    legs: &'static [JournalLeg],
}

struct JournalLeg {
    account_role: AccountRole,
    direction: Direction,
    capture_policy: CapturePolicy,
}

enum CaptureKind {
    CostInventoryCogs,
    CostDirectExpense,
    CostPostingVariance(CaptureVarianceReason),
    RevenueDeliveryOrder,
    RevenueDeliveryMilestone,
    RevenueSalesReturn,
}
enum CaptureVarianceReason {
    EstimatePriceDiffIssued,
    PurchaseReturnDiff,
    RedLetterDiff,
    OverInvoiceToCost,
}
enum CaptureDetailGrain { Head, Line }
enum CaptureParentRequirement {
    NewRootOnly,
    ReverseCurrentLiveOnly,
    NewRootForPositiveReverseCurrentLiveForNegative,
}
enum CapturePolicy {
    None,
    Required {
        capture_kind: CaptureKind,
        detail_grain: CaptureDetailGrain,
        parent_requirement: CaptureParentRequirement,
    },
}
```

`(source_kind,measure_key)` 在 `JOURNAL_MAP` 中唯一，每条规则必须有 1..n 个腿。正金额按表中方向，负金额取绝对值并把该规则的全部腿同时反向，零金额不生成腿。调用方请求内 `MeasureKey` 也必须唯一；不产分录的控制总额禁止塞进 `measures`。每个成本或收入角色腿必须恰有一个 `Required` 策略，其余角色腿必须为 `None`；下表“—”逐字表示该规则全部腿为 `None`，非空策略以 `角色: CaptureKind/DetailGrain/ParentRequirement` 唯一绑定到该角色腿。

## 5. 完整计量项—分录腿映射

下表的 `D`/`C` 分别为借/贷；逗号分隔的腿属于同一条 `JournalRule`。

| 来源 | MeasureKey | 腿 | 归集策略 |
|---|---|---|---|
| DELIVERY_CONFIRMED | `revenue_amount` | D AR_UNBILLED, C REVENUE | REVENUE: `RevenueDeliveryOrder/Line/NewRootOnly` |
| DELIVERY_CONFIRMED | `cogs_amount` | D COGS, C INVENTORY | COGS: `CostInventoryCogs/Line/NewRootOnly` |
| SALES_INVOICE_ISSUED | `gross_amount` | D AR | — |
| SALES_INVOICE_ISSUED | `net_amount` | C AR_UNBILLED | — |
| SALES_INVOICE_ISSUED | `output_tax_amount` | C OUTPUT_TAX | — |
| SALES_INVOICE_ISSUED | `advance_auto_applied_amount` | D CUSTOMER_ADVANCE, C AR | — |
| SALES_INVOICE_REVERSED | `gross_amount` | C AR | — |
| SALES_INVOICE_REVERSED | `net_amount` | D AR_UNBILLED | — |
| SALES_INVOICE_REVERSED | `output_tax_amount` | D OUTPUT_TAX | — |
| SALES_INVOICE_REVERSED | `released_settlement_amount` | D AR, C CUSTOMER_ADVANCE | — |
| PURCHASE_INVOICE_INVENTORY | `gross_amount` | C AP | — |
| PURCHASE_INVOICE_INVENTORY | `input_tax_amount` | D INPUT_TAX | — |
| PURCHASE_INVOICE_INVENTORY | `accrual_reversal_amount` | D AP_ACCRUED | — |
| PURCHASE_INVOICE_INVENTORY | `price_variance_in_stock_amount` | D INVENTORY | — |
| PURCHASE_INVOICE_INVENTORY | `price_variance_released_amount` | D COGS | COGS: `CostPostingVariance(EstimatePriceDiffIssued)/Line/NewRootOnly` |
| PURCHASE_INVOICE_INVENTORY | `overbilling_amount` | D OVERBILLING_SUSPENSE | — |
| PURCHASE_INVOICE_INVENTORY | `advance_auto_applied_amount` | D AP, C SUPPLIER_ADVANCE | — |
| PURCHASE_INVOICE_DIRECT_EXPENSE | `gross_amount` | C AP | — |
| PURCHASE_INVOICE_DIRECT_EXPENSE | `input_tax_amount` | D INPUT_TAX | — |
| PURCHASE_INVOICE_DIRECT_EXPENSE | `direct_expense_amount` | D DIRECT_EXPENSE_COST | DIRECT_EXPENSE_COST: `CostDirectExpense/Line/NewRootOnly` |
| PURCHASE_INVOICE_DIRECT_EXPENSE | `advance_auto_applied_amount` | D AP, C SUPPLIER_ADVANCE | — |
| PURCHASE_INVOICE_INVENTORY_REVERSED | `gross_amount` | D AP | — |
| PURCHASE_INVOICE_INVENTORY_REVERSED | `input_tax_amount` | C INPUT_TAX | — |
| PURCHASE_INVOICE_INVENTORY_REVERSED | `grni_reopened_amount` | C AP_ACCRUED | — |
| PURCHASE_INVOICE_INVENTORY_REVERSED | `stock_variance_reversed_amount` | C INVENTORY | — |
| PURCHASE_INVOICE_INVENTORY_REVERSED | `released_variance_reversed_cogs_amount` | C COGS | COGS: `CostPostingVariance(RedLetterDiff)/Line/ReverseCurrentLiveOnly` |
| PURCHASE_INVOICE_INVENTORY_REVERSED | `released_variance_reversed_direct_expense_amount` | C DIRECT_EXPENSE_COST | DIRECT_EXPENSE_COST: `CostDirectExpense/Line/ReverseCurrentLiveOnly` |
| PURCHASE_INVOICE_INVENTORY_REVERSED | `overbilling_settlement_amount` | C OVERBILLING_SUSPENSE | — |
| PURCHASE_INVOICE_INVENTORY_REVERSED | `released_settlement_amount` | D SUPPLIER_ADVANCE, C AP | — |
| PURCHASE_INVOICE_DIRECT_EXPENSE_REVERSED | `gross_amount` | D AP | — |
| PURCHASE_INVOICE_DIRECT_EXPENSE_REVERSED | `input_tax_amount` | C INPUT_TAX | — |
| PURCHASE_INVOICE_DIRECT_EXPENSE_REVERSED | `direct_expense_reversed_direct_expense_amount` | C DIRECT_EXPENSE_COST | DIRECT_EXPENSE_COST: `CostDirectExpense/Line/ReverseCurrentLiveOnly` |
| PURCHASE_INVOICE_DIRECT_EXPENSE_REVERSED | `direct_expense_reversed_cogs_amount` | C COGS | COGS: `CostInventoryCogs/Line/ReverseCurrentLiveOnly` |
| PURCHASE_INVOICE_DIRECT_EXPENSE_REVERSED | `released_settlement_amount` | D SUPPLIER_ADVANCE, C AP | — |
| PURCHASE_INVOICE_LINKED_RETURN_REVERSED | `gross_amount` | D AP | — |
| PURCHASE_INVOICE_LINKED_RETURN_REVERSED | `input_tax_amount` | C INPUT_TAX | — |
| PURCHASE_INVOICE_LINKED_RETURN_REVERSED | `grni_reopened_amount` | C AP_ACCRUED | — |
| PURCHASE_INVOICE_LINKED_RETURN_REVERSED | `linked_return_price_difference_amount` | C COGS | COGS: `CostPostingVariance(RedLetterDiff)/Line/NewRootOnly` |
| PURCHASE_INVOICE_LINKED_RETURN_REVERSED | `released_settlement_amount` | D SUPPLIER_ADVANCE, C AP | — |
| PURCHASE_RECEIPT | `accrual_amount` | D INVENTORY, C AP_ACCRUED | — |
| PURCHASE_RECEIPT | `overbilling_settlement_amount` | D INVENTORY, C OVERBILLING_SUSPENSE | — |
| PURCHASE_RETURN_INVENTORY | `grni_consumed_amount` | D AP_ACCRUED | — |
| PURCHASE_RETURN_INVENTORY | `inventory_return_amount` | C INVENTORY | — |
| PURCHASE_RETURN_INVENTORY | `return_carrying_difference_amount` | D COGS | COGS: `CostPostingVariance(PurchaseReturnDiff)/Line/NewRootOnly` |
| RECEIPT_REGISTERED | `bank_amount` | D BANK | — |
| RECEIPT_REGISTERED | `cash_on_hand_amount` | D CASH | — |
| RECEIPT_REGISTERED | `settlement_amount` | C AR | — |
| RECEIPT_REGISTERED | `advance_amount` | C CUSTOMER_ADVANCE | — |
| PAYMENT_REGISTERED | `settlement_amount` | D AP | — |
| PAYMENT_REGISTERED | `advance_amount` | D SUPPLIER_ADVANCE | — |
| PAYMENT_REGISTERED | `bank_amount` | C BANK | — |
| PAYMENT_REGISTERED | `cash_on_hand_amount` | C CASH | — |
| CUSTOMER_REFUND | `advance_consumed_amount` | D CUSTOMER_ADVANCE | — |
| CUSTOMER_REFUND | `settlement_released_amount` | D AR | — |
| CUSTOMER_REFUND | `bank_amount` | C BANK | — |
| CUSTOMER_REFUND | `cash_on_hand_amount` | C CASH | — |
| SUPPLIER_REFUND | `bank_amount` | D BANK | — |
| SUPPLIER_REFUND | `cash_on_hand_amount` | D CASH | — |
| SUPPLIER_REFUND | `advance_consumed_amount` | C SUPPLIER_ADVANCE | — |
| SUPPLIER_REFUND | `settlement_released_amount` | C AP | — |
| SALES_RETURN | `revenue_amount` | D REVENUE, C AR_UNBILLED | REVENUE: `RevenueSalesReturn/Line/ReverseCurrentLiveOnly` |
| SALES_RETURN | `inventory_return_cogs_amount` | D INVENTORY, C COGS | COGS: `CostInventoryCogs/Line/ReverseCurrentLiveOnly` |
| SALES_RETURN | `inventory_return_direct_expense_amount` | D INVENTORY, C DIRECT_EXPENSE_COST | DIRECT_EXPENSE_COST: `CostDirectExpense/Line/ReverseCurrentLiveOnly` |
| OVERBILLING_WRITTEN_OFF | `overbilling_amount` | D COGS, C OVERBILLING_SUSPENSE | COGS: `CostPostingVariance(OverInvoiceToCost)/Head/NewRootForPositiveReverseCurrentLiveForNegative` |

缩写只用于本表展示：AR_UNBILLED=`ACCOUNTS_RECEIVABLE_UNBILLED`，AR=`ACCOUNTS_RECEIVABLE`，CUSTOMER_ADVANCE=`ADVANCE_FROM_CUSTOMER`，REVENUE=`MAIN_OPERATING_REVENUE`，COGS=`MAIN_OPERATING_COST`，AP=`ACCOUNTS_PAYABLE`，AP_ACCRUED=`ACCOUNTS_PAYABLE_ACCRUED`，SUPPLIER_ADVANCE=`ADVANCE_TO_SUPPLIER`，OUTPUT_TAX=`TAX_PAYABLE_OUTPUT`，INPUT_TAX=`TAX_PAYABLE_INPUT`，BANK=`BANK_DEPOSIT`，CASH=`CASH_ON_HAND`。

## 6. 必填、可选、互斥与平衡方程

“必填”表示键必须出现；允许为零的键仍须出现，零值在展开时跳过。未列为可选的其他键一律非法。

| 来源 | 必填 MeasureKey | 可选 MeasureKey | 约束/方程 |
|---|---|---|---|
| DELIVERY_CONFIRMED | `revenue_amount` | `cogs_amount` | `revenue_amount>0`；`cogs_amount>=0` |
| SALES_INVOICE_ISSUED | `gross_amount`,`net_amount`,`output_tax_amount` | `advance_auto_applied_amount` | `gross_amount=net_amount+output_tax_amount`；前三项非负且 gross>0；`0<=advance_auto_applied_amount<=gross_amount` |
| SALES_INVOICE_REVERSED | `gross_amount`,`net_amount`,`output_tax_amount` | `released_settlement_amount` | `gross_amount=net_amount+output_tax_amount`；前三项非负且 gross>0；`0<=released_settlement_amount<=gross_amount` |
| PURCHASE_INVOICE_INVENTORY | `gross_amount`,`input_tax_amount`,`accrual_reversal_amount`,`price_variance_in_stock_amount`,`price_variance_released_amount`,`overbilling_amount` | `advance_auto_applied_amount` | `gross_amount=input_tax_amount+accrual_reversal_amount+price_variance_in_stock_amount+price_variance_released_amount+overbilling_amount`；两项 `price_variance_*` 可为负，其余必填项非负且 gross>0；`0<=advance_auto_applied_amount<=gross_amount` |
| PURCHASE_INVOICE_DIRECT_EXPENSE | `gross_amount`,`input_tax_amount`,`direct_expense_amount` | `advance_auto_applied_amount` | `gross_amount=input_tax_amount+direct_expense_amount`；必填项非负且 gross>0；`0<=advance_auto_applied_amount<=gross_amount` |
| PURCHASE_INVOICE_INVENTORY_REVERSED | `gross_amount`,`input_tax_amount`,`grni_reopened_amount`,`stock_variance_reversed_amount`,`released_variance_reversed_cogs_amount`,`released_variance_reversed_direct_expense_amount`,`overbilling_settlement_amount` | `released_settlement_amount` | 两个 released 键按原已出库价差 capture 的当前 live 成本角色拆分，二者之和严格等于 invoice owner 的 `released_variance_reversed_amount` 控制总额；`gross_amount=input_tax_amount+grni_reopened_amount+stock_variance_reversed_amount+released_variance_reversed_cogs_amount+released_variance_reversed_direct_expense_amount+overbilling_settlement_amount`；stock 与两个 released 键可为负但同一原效果拆分后各 live slice 保持其本次冲回方向，其余必填项非负且 gross>0；`0<=released_settlement_amount<=gross_amount` |
| PURCHASE_INVOICE_DIRECT_EXPENSE_REVERSED | `gross_amount`,`input_tax_amount`,`direct_expense_reversed_direct_expense_amount`,`direct_expense_reversed_cogs_amount` | `released_settlement_amount` | 两个成本键按原直接费用 capture 的当前 live 角色拆分，二者之和严格等于 invoice owner 的 `direct_expense_amount` 控制总额；`gross_amount=input_tax_amount+direct_expense_reversed_direct_expense_amount+direct_expense_reversed_cogs_amount`；必填项非负且 gross>0；`0<=released_settlement_amount<=gross_amount` |
| PURCHASE_INVOICE_LINKED_RETURN_REVERSED | `gross_amount`,`input_tax_amount`,`grni_reopened_amount`,`linked_return_price_difference_amount` | `released_settlement_amount` | `linked_return_price_difference_amount=red_net_amount-grni_reopened_amount`；`gross_amount=input_tax_amount+grni_reopened_amount+linked_return_price_difference_amount`；差额可负，其余必填项非负且 gross>0；`0<=released_settlement_amount<=gross_amount`；不得出现任何库存键 |
| PURCHASE_RECEIPT | `accrual_amount`,`overbilling_settlement_amount` | 无 | 两项非负；均为零时 `PostingPort=Skipped`，但 GRNI 数量根仍存在 |
| PURCHASE_RETURN_INVENTORY | `grni_consumed_amount`,`inventory_return_amount`,`return_carrying_difference_amount` | 无 | `grni_consumed_amount` 取原 GRNI；`inventory_return_amount` 取锁后当前账面价值，部分退货按移动平均价、全数退清取退货前金额余额全额并使库存数量/金额/单价同时归零；`return_carrying_difference_amount=inventory_return_amount-grni_consumed_amount`，前两项非负，差额可负 |
| RECEIPT_REGISTERED | `bank_amount`,`cash_on_hand_amount`,`settlement_amount`,`advance_amount` | 无 | 四项非负；`bank_amount`/`cash_on_hand_amount` 恰一大于零；`bank_amount+cash_on_hand_amount=settlement_amount+advance_amount` |
| PAYMENT_REGISTERED | `bank_amount`,`cash_on_hand_amount`,`settlement_amount`,`advance_amount` | 无 | 四项非负；`bank_amount`/`cash_on_hand_amount` 恰一大于零；`settlement_amount+advance_amount=bank_amount+cash_on_hand_amount` |
| CUSTOMER_REFUND | `bank_amount`,`cash_on_hand_amount`,`advance_consumed_amount`,`settlement_released_amount` | 无 | 四项非负；`bank_amount`/`cash_on_hand_amount` 恰一大于零；`advance_consumed_amount+settlement_released_amount=bank_amount+cash_on_hand_amount` |
| SUPPLIER_REFUND | `bank_amount`,`cash_on_hand_amount`,`advance_consumed_amount`,`settlement_released_amount` | 无 | 四项非负；`bank_amount`/`cash_on_hand_amount` 恰一大于零；`bank_amount+cash_on_hand_amount=advance_consumed_amount+settlement_released_amount` |
| SALES_RETURN | `revenue_amount` | `inventory_return_cogs_amount`,`inventory_return_direct_expense_amount` | 三项均非负且 revenue>0；两项成本键按原交付成本 capture 的当前 live 角色拆分，二者之和必须与 owner 在同事务持有的 `inventory_return_amount` 控制总额逐分相等；直运或原交付实际成本为零时两键均可不传 |
| OVERBILLING_WRITTEN_OFF | `overbilling_amount` | 无 | `overbilling_amount>0` |
| YEAR_END_PL_CLOSING | 无 | 无 | 普通 `post` 必须拒绝；年结服务按实际损益科目余额生成受控多行凭证 |
| CORRECTION | 无 | 无 | 普通 `post` 必须拒绝；只由 `post_correction` 根据已批准的原凭证行生成 |
| HISTORICAL_MIGRATION | 无 | 无 | 普通 `post` 必须拒绝且 `JOURNAL_MAP` 零行；只由 Stage 14 migration writer 生成平衡 APPLY 或完整镜像 REVERSE |

本节与第 5 节均只使用完整、可直接生成 Rust 枚举的 `MeasureKey`，不得再定义短名或别名。`red_net_amount` 只是 invoice 服务按本次已登记红字行汇总得到的控制总额，不进入 `measures`，ledger 必须以 `gross_amount-input_tax_amount` 复核它后再校验链接退货差额方程。销售退货的 `inventory_return_amount` 同样只是 inventory owner 返回的控制总额，不是 `MeasureKey`；sales owner 必须按当前 live 成本叶片的角色形成上述两个静态键，ledger 不允许运行时动态选择科目角色。

## 7. 受控特殊入口

公开 `PostingPort` 只有 `post`、`post_reversal`、`post_correction` 三个凭证写入口。

- `post` 只接受前 16 个普通来源，不接受 `YEAR_END_PL_CLOSING/CORRECTION/HISTORICAL_MIGRATION`。
- `post_reversal` 的金额拆分由 finance 在同一事务锁后计算，ledger 一次构造原凭证的完整反向图；业务层不能传自由科目，也不能在凭证已过账后追加行。生成头必须引用未被冲正的四类普通现金凭证，每条生成行一一引用原凭证内的父行，完整复制 line/account/role/measure/amount 并只反转 direction；头行唯一约束与延迟经济镜像约束共同禁止第二次、部分或错腿冲正。
- `post_correction` 只做已批准的成本同侧科目归类更正，首版唯一矩阵为 `MAIN_OPERATING_COST↔DIRECT_EXPENSE_COST`；收入侧只有一个角色，明确拒绝 `MAIN_OPERATING_REVENUE`。入口不改变资金、税额、库存、应收应付或源业务事实；每个输入自动生成 `REVERSE_ORIGINAL|TARGET` pair，并用 `generated_voucher_line_id` 把两条证据一一绑定到生成凭证行。审批快照冻结逐项成本 capture allocation，回调只能重验原集合而不得重算或换维度；pair 形状、生成行镜像、同侧角色矩阵与累计更正上限由数据库延迟约束和应用事务末重读双重保证。

历史迁移只在 `ep-app-ledger::migration` 模块内定义私有 `HistoricalMigrationPostingPrivate`，唯一实现者是 `LedgerMigrationWriter`，其 `post_historical_migration` 与 `reverse_migrated_historical_voucher` 只准同模块的 `MigrationModuleWriter for LedgerMigrationWriter` 调用；trait/struct/方法均无 `pub` 或 `pub(crate)`，Rust 可见性直接禁止复用且不新增 archcheck。APPLY DTO 精确为 `{data_migration_record_id,target_voucher_id,batch_no,posting_date,lines[]}`，每行精确为 `{account_id,account_role,direction,amount}`；target 必须用 VALIDATED UUIDv7 预留，行至少两项、account 不重复、amount>0、角色恰绑定该启用可过账科目，服务按 account UUID bytes 排序并生成 line_no/id，measure 固定 `historical_migration`，借贷相等且不生成归集。REVERSE DTO 精确为 `{data_migration_record_id,target_voucher_id,original_voucher_id}`；日期取数据库时钟的 `business_day`，锁读原图后逐腿复制并反向，source tuple 复用同一 record/batch、sequence=2，不接受自由日期、期间、科目、金额或腿。两入口与 Stage 14 receipt/R0/record 状态同一事务。

## 8. 开发门禁

CI 必须机械验证：14 表/2 视图、13 张 RLS、17 个 AccountRole、19 个 VoucherSourceKind、16 个 map-backed 来源的每个 `(source,measure)` 一条且 `legs` 非空、必填/可选不相交、请求键无重复、全部合法计量组合借贷平衡、所有非法或不平衡组合被拒绝、正负价差均通过、19 个来源全部有具名测试；年结、更正和历史迁移只验证专用入口且在 `JOURNAL_MAP` 零行。每个成本/收入腿必须恰有一个与第 5 节逐字相等的 capture policy，其他腿必须为 None；缺/多 attribution、错 kind/grain/parent、销售退货两成本键与 owner 控制总额不等都阻断。真实 PostgreSQL COMMIT 负例必须覆盖通用凭证 count/sum/period/date、资金冲正第二次/部分/错腿、HISTORICAL_MIGRATION tuple/sequence/receipt 双向图与第二次/部分/错腿镜像、CORR pair/生成行错配与超额、收入或跨侧 CORR、期初批次空图/错合计/确认后改写、关账九态/独立取消证据/request↔slot/PASSED↔CLOSED 双向图，以及年结五态/0-1-2 凭证/逐科目归零/source/pre-image；任一负例均整笔拒绝且无部分请求、slot、期间、凭证、余额、迁移 receipt/R0 或 Outbox。任何计数或映射不一致都阻断阶段退出。
