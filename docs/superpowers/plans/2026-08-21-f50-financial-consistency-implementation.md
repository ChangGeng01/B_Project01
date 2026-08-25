# F-50 财务一致性与发票模型实施计划

> **F-57 状态：`SUPERSEDED_DO_NOT_EXECUTE`。** 本文件只保留为历史设计输入，不得单独或续跑执行；其中仍适用的财务约束已并入 F-57 总体设计、需求追踪矩阵及 F-57 实施计划。任何实现必须从 `2026-08-23-f57-governed-automation-fabric-implementation.md` Task 1 开始。

> **For agentic workers:** REQUIRED SUB-SKILL: Use `superpowers:executing-plans` to implement this plan task-by-task. 每个任务完成测试并提交后才能进入下一任务；禁止把本计划拆成互相看不到同一事务边界的并行实现。

**Goal:** 在不引入自由分录、万能发票表或通用事件溯源平台的前提下，实现 F-50 已冻结的核销方向、退款逐资金根追溯、动态资金冲正、当前/历史余额、销项与进项多行多税率、中央号码、分次红冲、更正凭证及历史成交资格，并通过 45 项验收。

**Architecture:** 保留 `ledger`、`finance`、`invoice`、`portal`、`mdm` 模块边界。跨模块写入只经 owner contract 和同一个 `&mut dyn Tx`；金额事实只追加，当前投影事务内同步维护并重读；数据库约束是第一道防线，应用断言是第二道防线。当前经营余额由四个 current view 提供，历史关账由四个截至期间 recon view 提供。

**Tech Stack:** Rust workspace、Axum HTTP、PostgreSQL 16、SQL migrations、`rust_decimal::Decimal`、OpenAPI 3.1、Excel v2 模板、proptest、真实 PostgreSQL 集成测试、现有 `xtask` 静态门禁。

**Spec:** `docs/superpowers/specs/2026-08-21-f50-financial-consistency-design.md`

---

## Global Constraints

- 本计划只在用户另行授权后执行；本次文档冻结不开始业务代码。
- 旧 `origin=REFUND`、`min(已核销, 红字金额)`、固定镜像原凭证的资金冲正、按 `reverses_id` 推断符号、不分方向的原票单列、头级单税率、单次红冲唯一约束与当前 `open_amount` 倒推历史全部禁止。
- 所有人民币金额为 `numeric(18,2)`/`Decimal`，税率为 `numeric(9,6)`；普通税额使用 half-up，容差最大 `0.02`，价税等式无容差。
- 同一业务命令只开一个写事务；finance、invoice、ledger、portal 的 owner 端口共享同一 `&mut dyn Tx`。事务内不做网络、文件正文、通知投递或 OCR。
- 所有资金、发票、采购退货、GRNI 与库存联动事务共用 F-50 设计 §10.0.1 的唯一 `CrossModuleLockCoordinator` 与锁序：原款项/退款单 → 退款来源链接 → 采购退货单 → 原发票头/行 → 收货行 → GRNI 根/效果 → 库存 availability/value/coverage/qty/serial → AR/AP 正向主条目 → payable reservation → 预收/预付 → 核销根/效果 → invoice/finance 冲销累计。先无锁收集完整 `F50LockPlan`，一次锁全集，锁后重载得到同一 plan，再签发绑定 tx/法人/plan digest 的 `TransactionLockProof`；所有 mutator 首条 SQL 前验证 proof。集合漂移走既有 SQLSTATE 40001 三次整事务重试，禁止跨 schema 自行锁、锁到一半补锁、持 lease 写入或仅靠调用顺序注释。
- 关系、凭证、冲销、号码均只追加或受限状态推进。不得更新或删除已过账金额事实。
- RLS、复合外键、生成列、延迟约束、并发、期间切片一律使用真实 PostgreSQL；内存替身不能代替验收。
- `SupplierInvoiceUploadId` 使用 `ep-contract-portal` 内的 opaque marker/newtype，不扩展 foundation 冻结的 22 个跨模块 marker。
- 负向用例必须精确断言 `docs/error-codes.md` 已登记的 code/category/HTTP/retryable；禁止只断言 4xx 或数据库报错。

## Frozen Counts

| 范围 | 最终值 |
|---|---:|
| 阶段 7 业务表 / 本阶段迁移 / 法人 RLS 表 | 31 / 33 / 31；含目标晚建 portal 外键追补，阶段 10/12 的后续晚绑定文件按各自阶段计数 |
| 阶段 7 事件 | 15；新增 `portal.supplier_invoice_upload.returned.v1`，accepted 由阶段 10 事务产生 |
| 阶段 7 HTTP | 既有清单 + 1 个内部 return 入口 |
| 阶段 9 ledger 表 / 迁移 / ledger RLS | 14 / 18 / 13；只有 `posting_trigger_event_types` 无法人列 |
| 阶段 9 `VoucherSourceKind` / 单据类型码 | 18 / 5；采购发票红字与退货四项按 ledger 数据字典重排，含 `CORRECTION` / `CORR` |
| 阶段 9 HTTP / 审计动作 / Ledger 事件 / Ledger 错误码 | 既有 31 + 3 = 34 / 15 / 9 / 36（32 自有既有码 + F-50 四码；重新认证与自审拒绝传播平台码） |
| 阶段 9 posting-trigger event types | 13，不因更正凭证增加 |
| 阶段 10 invoice / finance / 总表 | 16 / 23 / 39 |
| 阶段 10 invoice / finance 本目录迁移 / 阶段总迁移 | 18 / 25 / 46；总数另含 inventory、portal、procure 各一支目标晚建外键追补 |
| 阶段 10 法人 RLS 表 / 只读视图 | 39 / 21 |
| 阶段 10 HTTP / 自有 contract traits | 49（23 写、26 读）/ 16；第 16 个为 `ReceiptPlanBillingQuery`，另消费阶段 7 的 `SupplierInvoiceUploadWritebackPort` 与 `GrniEffectWritebackPort` 2 个 owner port，两者不计入自有 trait |
| 阶段 10 事件 | 13；既有 12 个 payload 更新，加 portal accepted 1 个 |
| 阶段 10 活跃自有错误码 | 61：FINANCE 31 + INVOICE 30；另传播 MDM 1 + PORTAL 2；新增的第 31 个 FINANCE 码为付款状态机非法迁移；门户上传重复码由阶段 7 自身返回 |
| F-50 新登记错误码 / 验收场景 | 32 / 45 |

阶段 10 的 21 个视图固定为：原 10 个 recon 视图、原 4 个业务查询视图、原 3 个数据集视图，加 `finance.v_receivable_current`、`v_payable_current`、`v_advance_receipt_current`、`v_advance_payment_current` 四个 current view。四个 `v_recon_*` 往来视图改为截至期间事件切片，不再表示“今天的 open”。

## Superseded Error Codes

下列旧码保留在历史文档但新实现不得返回：

```text
FINANCE.CASH_DOCUMENT_REVERSAL.ADVANCE_ALREADY_CONSUMED
FINANCE.REFUND.AMOUNT_EXCEEDS_CAP
FINANCE.SETTLEMENT.LINE_EXCEEDS_OPEN_AMOUNT
FINANCE.SETTLEMENT.OPEN_AMOUNT_CHANGED
INVOICE.IMPORT_BATCH.TEMPLATE_MISMATCH
INVOICE.INVOICE_REVERSAL.RED_AMOUNT_MISMATCH
INVOICE.INVOICE_REVERSAL.SOURCE_ALREADY_REVERSED
INVOICE.INVOICE_REVERSAL.TYPE_MUTUALLY_EXCLUSIVE
INVOICE.PURCHASE_INVOICE.GROSS_AMOUNT_MISMATCH
INVOICE.PURCHASE_INVOICE.INVOICE_NO_DUPLICATED
INVOICE.SALES_INVOICE.CODE_FORBIDDEN_FOR_DIGITAL
INVOICE.SALES_INVOICE.CODE_REQUIRED_FOR_PAPER
INVOICE.SALES_INVOICE.GROSS_AMOUNT_MISMATCH
INVOICE.SALES_INVOICE.INVOICE_NO_DUPLICATED
INVOICE.SALES_INVOICE.TAX_AMOUNT_OUT_OF_TOLERANCE
```

其余阶段 10 已登记 FINANCE/INVOICE 码原样保留。F-50 的 24 个阶段 10 自有码（FINANCE 11、INVOICE 13）加入，并为付款撤回/取消状态机补一条显式非法迁移码后，活跃自有码精确为 61。

---

### Task 1: Freeze shared types, errors, config, and static gates

**Files:**
- Modify: `crates/foundation/src/error/codes.rs`
- Modify: `crates/platform/runtime/src/config/sections.rs`
- Modify: `xtask/src/archcheck/source.rs`
- Modify: `xtask/src/errorcodes.rs`
- Modify: `xtask/src/configdoc.rs`
- Test: `xtask/tests/f50_registry.rs`
- Test: `xtask/tests/fixtures/archcheck/voucher_direct_insert.rs`

- [ ] **Step 1: Write failing registry tests.** Assert all 32 F-50 codes exist once, the two config keys have the documented type/default/range, the 15 superseded codes are not returned by any new module source, and direct voucher SQL outside ledger fails `archcheck`.
- [ ] **Step 2: Run the focused tests.** Run `cargo test -p ep-xtask --test f50_registry`. Expected: FAIL because constants/config/static rule do not yet exist.
- [ ] **Step 3: Add exact constants and config.** Add the 32 codes verbatim. Add `invoice.tax.amount_tolerance: Decimal = 0.02` with `0.00..=0.02` startup validation and `mdm.trade_history.include_ineffective: bool = false`.
- [ ] **Step 4: Add the voucher-write gate.** Permit voucher table writes only in ledger repository implementations reached by `post`, `post_reversal`, or `post_correction`; the fixture must fail with a stable rule name.
- [ ] **Step 5: Verify.** Run `cargo test -p ep-xtask --test f50_registry && cargo xtask errorcodes && cargo xtask configdoc && cargo xtask archcheck`. Expected: PASS.
- [ ] **Step 6: Commit.** `git commit -m "feat(foundation): freeze F-50 registries and gates"`

### Task 1A: Implement the one cross-module lock coordinator before any F-50 mutator

**Files:**
- Modify: `crates/foundation/src/port/tx.rs`
- Create: `crates/contract/ledger/src/f50_lock.rs`
- Create: `crates/application/ledger/src/f50_lock.rs`
- Create: `crates/application/{finance,invoice,procure,inventory}/src/f50_lock_slice.rs`
- Modify: `apps/core-server/src/wiring/f50_lock.rs`
- Modify: `apps/job-worker/src/wiring/f50_lock.rs`
- Test: `crates/application/ledger/tests/f50_lock_coordinator_pg.rs`
- Test: `tests/compile_fail/f50_lock_proof/`

- [ ] **Step 1: Freeze the ABI.** Implement `TransactionLockProof` exactly as F-50 design §10.0.1 and the exact `F50LockPlan`/key enums, `F50LockLease`, `F50LockSlicePort`, `CrossModuleLockCoordinator` signatures. Foundation proof remains business-free and Debug-redacted; no contract crate depends on another contract crate.
- [ ] **Step 2: Register slice owners with phase-safe and release-complete gates.** Finance, procure, invoice and inventory each implement the ledger-owned SPI in their own application crate. `lock_all` rejects missing, duplicate or wrong registration only for owners implied by this plan's non-empty categories, so stages 8/6/7 can test their real slices without future Noop owners. After stage 10, core-server/job-worker `--check` separately requires all four owner slots exactly once for the complete release. Each category only touches its owner schema; the coordinator calls the now twenty categories in the frozen global order, including `InventorySourceDocument` immediately before the five existing inventory balance/state categories.
- [ ] **Step 3: Issue a transaction-bound proof.** Normalize/sort/dedupe the plan, lock every category, require an exact lock-after-reload plan match, and HMAC `F50_LOCK_V1 + TxId + legal_entity_id + plan_digest + sealed`. A lease is never accepted by a mutator. Set drift invokes the adapter's fixed SQLSTATE 40001 abort so the central 50/150/450 ms policy reruns the whole closure.
- [ ] **Step 4: Make proof verification mandatory.** Every affected owner implementation receives `&TransactionLockProof`, calls `assert_covers` before its first SQL, and only reloads already locked rows. Add compile-fail cases for omitting the proof and runtime cases for forged bytes, lease-as-proof, wrong tx, wrong entity, missing category, missing key and post-seal plan mutation; all must produce zero business, Outbox and audit rows.
- [ ] **Step 5: Prove category concurrency.** Cover empty categories, absent inventory balance/reservation rows via advisory locks, two owners contending on the same graph, collection drift, reversed caller invocation order and process restart. Only a valid serial order or whole-transaction retry is accepted; no partial fact graph may commit.
- [ ] **Step 6: Commit.** `git commit -m "feat(ledger): coordinate F-50 cross-module locks"`

### Task 2: Add ledger correction-voucher schema

**Files:**
- Create: `db/migrations/ledger/V20261015090700__ledger_create_correction_vouchers.sql`
- Create: `db/migrations/ledger/V20261015090800__ledger_create_correction_voucher_lines.sql`
- Modify: `db/migrations/ledger/V20261015091700__ledger_backfill_append_only_registry.sql`
- Modify: `db/migrations/ledger/V20261015091600__ledger_create_dataset_views.sql`
- Create: `db/checks/14_f50_ledger_constraints.sql`
- Test: `testkit/tests/f50_ledger_schema_pg.rs`

- [ ] **Step 1: Write failing PostgreSQL tests.** Cover 14 ledger tables, 13 RLS tables, same-legal-entity source voucher FK, line uniqueness, positive one-sided amount, balanced totals, immutable posted rows, and cross-entity rejection.
- [ ] **Step 2: Run.** `cargo test -p ep-testkit --test f50_ledger_schema_pg`. Expected: FAIL on missing tables.
- [ ] **Step 3: Create the two tables.** Use F-50 dictionary columns, `CORR` doc numbers, `(legal_entity_id,id)` composite references, `APPEND_ONLY` guards and no free-form account input from HTTP.
- [ ] **Step 4: Update migration roster.** Preserve the existing 16 filenames and insert the two new timestamps above; final ledger migration count is 18.
- [ ] **Step 5: Verify schema.** Run the focused test and `cargo xtask sqlcheck`. Expected: PASS and exact counts 14/18/13.
- [ ] **Step 6: Commit.** `git commit -m "feat(ledger): add correction voucher schema"`

### Task 3: Implement `post_correction` and dynamic `post_reversal`

**Files:**
- Create: `crates/contract/ledger/src/correction.rs`
- Create: `crates/contract/ledger/src/reversal.rs`
- Modify: `crates/contract/ledger/src/lib.rs`
- Create: `crates/domain/ledger/src/correction_voucher.rs`
- Modify: `crates/domain/ledger/src/lib.rs`
- Create: `crates/application/ledger/src/posting/correction.rs`
- Create: `crates/application/ledger/src/posting/reversal.rs`
- Modify: `crates/application/ledger/src/lib.rs`
- Test: `crates/application/ledger/tests/f50_posting.rs`

- [ ] **Step 1: Freeze the contracts in tests.** The signatures are:

```rust
pub struct CashReversalPostingSplit {
    pub ar_ap_amount: Decimal,
    pub advance_amount: Decimal,
}

pub trait PostingPort {
    fn post(&self, tx: &mut dyn Tx, ctx: &SecurityContext, input: PostingInput)
        -> Result<PostingOutcome, AppError>;
    fn post_reversal(&self, tx: &mut dyn Tx, ctx: &SecurityContext,
        source: CashDocumentRef, split: CashReversalPostingSplit)
        -> Result<PostingOutcome, AppError>;
    fn post_correction(&self, tx: &mut dyn Tx, ctx: &SecurityContext,
        input: CorrectionPostingInput)
        -> Result<PostingOutcome, AppError>;
}
```

- [ ] **Step 2: Run failing tests.** `cargo test -p ep-app-ledger --test f50_posting`. Expected: FAIL on missing methods.
- [ ] **Step 3: Implement validation.** `post_reversal` accepts only registered cash source types, nonnegative split and exact original cash-leg total. `post_correction` accepts only posted source vouchers and the same-side matrix `MAIN_OPERATING_COST↔DIRECT_EXPENSE_COST`, balanced pairs, cumulative per-source-line cap, and nonempty frozen current-live cost capture allocations whose unique IDs all belong to the source voucher line and whose amounts sum exactly to the approved line. Revenue/cross-side correction and callback-time reallocation are rejected.
- [ ] **Step 4: Add source/event/catalog values.** Add `VoucherSourceKind::Correction`, `CORR`, audit action `CORRECTION_VOUCHER_POSTED`, and `ledger.correction_voucher.posted.v1`; posting-trigger event count remains 13.
- [ ] **Step 5: Verify.** Run ledger unit/property tests and focused integration tests. Expected: 18 source kinds fully covered, four new LEDGER codes mapped exactly.
- [ ] **Step 6: Commit.** `git commit -m "feat(ledger): support controlled correction and cash reversal splits"`

### Task 3A: Implement the frozen JOURNAL_MAP and GRNI/return pairing

**Files:**
- Modify: `crates/contract/ledger/src/posting.rs`
- Modify: `crates/domain/ledger/src/rule/journal_map.rs`
- Modify: `crates/domain/ledger/src/source_kind.rs`
- Modify: `db/migrations/procure/V20261018091000__procure_create_goods_receipt_line_costings.sql`
- Modify: `crates/contract/procure/src/port/grni_effect_writeback.rs`
- Modify: `crates/application/procure/src/usecase/post_purchase_return.rs`
- Modify: `crates/application/invoice/src/usecase/register_purchase_invoice.rs`
- Modify: `crates/application/invoice/src/usecase/register_purchase_credit_note.rs`
- Test: `crates/domain/ledger/tests/journal_map.rs`
- Test: `testkit/tests/grni_purchase_return_pg.rs`

- [ ] **Step 1: Write failing map tests.** Assert 17 AccountRole, 18 VoucherSourceKind and every `(source_kind,measure_key)` in `docs/data-dictionary/ledger.md` exactly once; every `JournalRule` has nonempty `legs`, every legal combination balances, every duplicate/unknown/missing/unbalanced combination fails.
- [ ] **Step 2: Replace the impossible tuple model.** Implement `JournalRule { source_kind, measure_key, requiredness, legs }`; make request MeasureKey unique; split `bank_amount` and `cash_on_hand_amount`; do not accept control totals as measures.
- [ ] **Step 3: Apply the 18-source reordering.** Delete the four superseded purchase reverse/return values, add `PURCHASE_INVOICE_INVENTORY_REVERSED`, `PURCHASE_INVOICE_DIRECT_EXPENSE_REVERSED`, `PURCHASE_INVOICE_LINKED_RETURN_REVERSED`, `PURCHASE_RETURN_INVENTORY`; remove `resolved_source_kind`; keep the total at 18.
- [ ] **Step 4: Harden GRNI storage.** Add identity `effect_seq`, NULL-safe root idempotency, deferred parent/root limits at transaction commit, and a quantity-positive amount-zero root case. Parent order uses effect_seq, never `created_at`.
- [ ] **Step 5: Implement purchase invoice and return accounting.** Purchase invoice debits AP accrued for GRNI principal and only posts signed price variance to inventory/COGS. Every credit note triggered by a purchase return persists `linked_purchase_return_id`; inventory lines use `PURCHASE_INVOICE_LINKED_RETURN_REVERSED`, reopen GRNI and post their difference only to COGS, after which the single physical return voucher consumes GRNI and credits inventory. Direct-expense/direct-ship lines use `PURCHASE_INVOICE_DIRECT_EXPENSE_REVERSED`, reverse AP/tax/original cost and never touch GRNI or local inventory. Persist only the physical voucher in `purchase_returns.physical_return_voucher_id`; construct the posted event with that nullable id plus the sorted, de-duplicated `PurchaseCreditNoteView.voucher_id` array. Cover the exact unbilled-material, billed/mixed-material and direct-expense/direct-ship payload shapes.
- [ ] **Step 6: Enforce the one cross-module lock order.** Use Task 1A's coordinator, not local SQL: collect the complete plan, obtain lease, lock/reload, seal to `TransactionLockProof`, and pass it into every GRNI/inventory/AR-AP/settlement mutator. The plan must include payable-reservation keys adjacent to AP originals and inventory availability keys before balance keys. Set drift retries the entire transaction; no incremental reverse-order locks.
- [ ] **Step 7: Verify.** Run ledger map/property tests and real-PostgreSQL GRNI tests for temporary accrual 100/invoice 120, `+100,-100,+30,-30`, zero amount roots, positive/negative differences, duplicate roots and concurrency. Expected: all balances and recon differences are zero.
- [ ] **Step 8: Commit.** `git commit -m "feat(finance): freeze journal map and GRNI return pairing"`

### Task 4: Expose the three correction-voucher endpoints

**Files:**
- Create: `crates/application/ledger/src/usecase/create_correction_voucher.rs`
- Create: `crates/application/ledger/src/query/list_correction_vouchers.rs`
- Create: `crates/application/ledger/src/query/get_correction_voucher.rs`
- Create: `apps/core-server/src/http/ledger/correction_vouchers.rs`
- Modify: `apps/core-server/src/wiring/mod.rs`
- Modify: `docs/openapi/ledger.v1.yaml`
- Test: `apps/core-server/tests/f50_correction_voucher_http.rs`

- [ ] **Step 1: Write failing HTTP tests.** Cover `POST /api/v1/ledger/correction-vouchers`, `GET /api/v1/ledger/correction-vouchers`, and `GET /api/v1/ledger/correction-vouchers/{id}` with RLS, reauth, approval, idempotency and no free-form entry route.
- [ ] **Step 2: Run.** `cargo test -p core-server --test f50_correction_voucher_http`. Expected: FAIL with routes missing.
- [ ] **Step 3: Implement the use cases and routes.** Creation accepts source voucher id, reason, posting date and controlled correction lines; list/detail are read-only and redact by ordinary voucher permissions.
- [ ] **Step 4: Verify exact stage counts.** 34 ledger endpoints, 15 audit actions, 9 ledger events, 38 ledger errors.
- [ ] **Step 5: Commit.** `git commit -m "feat(ledger): expose correction voucher workflow"`

### Task 5: Replace settlement links with explicit APPLY/RELEASE roots

**Files:**
- Modify: `db/migrations/finance/V20261019093100__finance_create_settlement_link_tables.sql`
- Create: `db/checks/15_f50_settlement_roots.sql`
- Create: `crates/domain/finance/src/settlement/effect.rs`
- Create: `crates/domain/finance/src/settlement/balance.rs`
- Create: `crates/application/finance/src/repository/settlement_links.rs`
- Test: `testkit/tests/f50_settlement_constraints_pg.rs`
- Test: `crates/domain/finance/tests/f50_settlement_properties.rs`

- [ ] **Step 1: Write failing direct-write tests.** Cover root self-reference, derived opposite effect, parent cap, root net range, same entity/side/entry/root and cycle rejection.
- [ ] **Step 2: Run.** `cargo test -p ep-testkit --test f50_settlement_constraints_pg`. Expected: FAIL on old columns.
- [ ] **Step 3: Implement the four table shapes.** Add `effect_kind`, `root_apply_id`, `reverses_id`, positive `settled_amount`; add `funding_origin` only to AR/AP links; remove old `origin`.
- [ ] **Step 4: Add mandatory DB enforcement.** Use composite self-FKs, NULL-safe CHECK and `DEFERRABLE INITIALLY DEFERRED` constraint triggers; application re-read remains secondary.
- [ ] **Step 5: Verify properties.** `root_net = ΣAPPLY-ΣRELEASE`, `0 <= root_net <= root amount`, no signed amounts.
- [ ] **Step 6: Commit.** `git commit -m "feat(finance): model settlement effects explicitly"`

### Task 6: Implement current balances and effective-open consumers

**Files:**
- Modify: `db/migrations/finance/V20261019093600__finance_create_reconciliation_views.sql`
- Create: `crates/contract/finance/src/current_balance.rs`
- Create: `crates/application/finance/src/query/current_balance.rs`
- Modify: `crates/application/finance/src/lib.rs`
- Test: `crates/application/finance/tests/f50_current_balances_pg.rs`

- [ ] **Step 1: Write failing tests** for `row_open`, `effective_open`, `advance_open`, migration openings and reversal rows excluded from candidates.
- [ ] **Step 2: Create four current views.** Compute current balances from ORIGINAL/REVERSAL and APPLY/RELEASE facts; projections must equal aggregates.
- [ ] **Step 3: Freeze the query contract.** Expose `effective_open` for AR/AP and `advance_open` for advances; do not expose an ambiguous `open_balance` field to cross-module consumers.
- [ ] **Step 4: Verify.** Run the focused tests and assert total stage-10 view count 21.
- [ ] **Step 5: Commit.** `git commit -m "feat(finance): expose one current balance contract"`

### Task 7: Implement refund registration per source-payment link

**Files:**
- Modify: `db/migrations/finance/V20261019092900__finance_create_refund_source_payment_links.sql`
- Create: `crates/domain/finance/src/refund/allocation.rs`
- Create: `crates/application/finance/src/usecase/register_refund.rs`
- Create: `crates/application/finance/src/repository/refund_source_links.rs`
- Test: `crates/application/finance/tests/f50_refund_sources_pg.rs`

- [ ] **Step 1: Write failing tests.** A refund split over receipts A/B must conserve each link, retain `refund_source_payment_link_id` on every effect and reject cross-root borrowing.
- [ ] **Step 2: Add per-link projections and FKs.** Enforce `linked = advance_consumed + settlement_released`, plus whole-refund sum.
- [ ] **Step 3: Implement order.** Per link consume traceable advance first, then release only roots funded by that source using locked LIFO.
- [ ] **Step 4: Map errors.** Request sum mismatch uses `SOURCE_ALLOCATION_MISMATCH`; locked capacity failure uses `SOURCE_CAP_EXCEEDED`.
- [ ] **Step 5: Verify customer and supplier mirrors.** Run focused tests with two source payments and concurrent refund attempts.
- [ ] **Step 6: Commit.** `git commit -m "feat(finance): preserve refund funding-root traceability"`

### Task 8: Implement dynamic cash-document reversal

**Files:**
- Create: `crates/domain/finance/src/cash_reversal/split.rs`
- Create: `crates/application/finance/src/usecase/reverse_cash_document.rs`
- Modify: `crates/contract/finance/src/lib.rs`
- Test: `crates/application/finance/tests/f50_cash_reversal_pg.rs`

- [ ] **Step 1: Write failing order tests.** Include receipt→red reversal→cash reversal, refund→full red→refund reversal, advance refund→new invoice→refund reversal, and supplier mirrors.
- [ ] **Step 2: Implement `R=S+V`.** Reject un-reversed downstream refunds; release only current root net and consume traceable advance.
- [ ] **Step 3: Implement per-source refund reversal.** Compute `Y_j/X_j/A_j/E_j/Q_j/Z_j/V_j` independently, never pool roots, and re-run auto-settlement after A within the same locked transaction.
- [ ] **Step 4: Construct one ledger command.** Only finance constructs `CashReversalPostingSplit`; HTTP/Excel/plugin cannot supply split amounts or accounts.
- [ ] **Step 5: Verify total and per-link conservation plus single voucher.** Expected: all order permutations end in the same balances.
- [ ] **Step 6: Commit.** `git commit -m "feat(finance): split cash reversals by locked current use"`

### Task 9: Implement historical AR/AP and advance slices

**Files:**
- Modify: `db/migrations/finance/V20261019093600__finance_create_reconciliation_views.sql`
- Create: `crates/contract/finance/src/as_of_balance.rs`
- Create: `crates/application/finance/src/query/as_of_balance.rs`
- Modify: `crates/application/ledger/src/recon/mod.rs`
- Test: `crates/application/finance/tests/f50_as_of_balances_pg.rs`

- [ ] **Step 1: Write failing M1–M4 tests.** Later refunds, reversals and cash reversals must not change earlier period outputs; `MIGRATION_OPENING` must appear in first-period current/history and remain settleable.
- [ ] **Step 2: Rewrite four recon views.** Group by actual `accounting_period_id` and period sequence; cumulative ORIGINAL−REVERSAL−APPLY+RELEASE for AR/AP, creation−APPLY+RELEASE for advances. Never compare UUID ids.
- [ ] **Step 3: Wire period close.** The four recon checks accept period P and compare the P slice to the P-end general-ledger balance.
- [ ] **Step 4: Verify latest= current.** Latest cumulative rows equal the four current views exactly.
- [ ] **Step 5: Commit.** `git commit -m "feat(finance): reconstruct historical balances from append-only effects"`

### Task 10: Add invoice line, reversal line, and central number schema

**Files:**
- Create: `db/migrations/invoice/V20261019090300__invoice_create_invoice_number_registry.sql`
- Modify: `db/migrations/invoice/V20261019090400__invoice_create_sales_invoices.sql`
- Create: `db/migrations/invoice/V20261019090500__invoice_create_sales_invoice_lines.sql`
- Modify: `db/migrations/invoice/V20261019090600__invoice_create_invoice_reversals.sql`
- Create: `db/migrations/invoice/V20261019090700__invoice_create_invoice_reversal_lines.sql`
- Modify: `db/migrations/invoice/V20261019090800__invoice_create_purchase_invoices.sql`
- Modify: `db/migrations/invoice/V20261019090900__invoice_create_purchase_invoice_lines.sql`
- Modify: `db/migrations/invoice/V20261019091300__invoice_enable_row_level_security.sql`
- Modify: `db/migrations/invoice/concurrent/V20261019091400__invoice_create_indexes.sql`
- Test: `testkit/tests/f50_invoice_schema_pg.rs`

- [ ] **Step 1: Write failing real-DB tests.** Cover 16 tables/17 migrations/16 invoice RLS, required line, multi-tax, NULL-safe reversal source XOR, wrong-head line, registry format/owner and cross-table duplicate races. Also cover `linked_purchase_return_id`: only `INPUT+RED_LETTER` may set it; OUTPUT, VOID and independent input correction must reject/nonpersist it. For reversal lines, directly write NULL/duplicate/gapped `source_effect_seq`, a non-final `ORIGINAL_UNIT_PRICE` amount deviation, and a forged final rounding residual after an earlier `ADJUSTED`; PostgreSQL must reject all five classes without relying on the application service.
- [ ] **Step 2: Create the final 17-file migration roster.** Use exactly the timestamps listed above plus unchanged old files at 900/905/910/925/930/935/950/960.
- [ ] **Step 3: Remove old columns and constraints.** Remove head `tax_rate`, number copies, `red_tax_rate`, `is_credit_note`, `reversed_by_id` and one-reversal uniqueness; add nullable `invoice_reversals.linked_purchase_return_id` as the persisted cross-schema logical reference.
- [ ] **Step 4: Add database constraints.** Reversal line FKs use default `MATCH SIMPLE` plus explicit all-or-none/XOR CHECK and deferred head-line trigger; `source_effect_seq` is NOT NULL and has direction-specific `(legal_entity_id,source_*_invoice_line_id,source_effect_seq)` uniqueness. A second deferred constraint trigger locks the active source line, requires `1..n` sequence continuity, replays cumulative quantity/amount limits, and verifies original-price/tail-rounding classification so an earlier `ADJUSTED` makes a later residual deviation invalid. Registry uses generated NOT NULL key and deferred two-way owner trigger. A NULL-safe header CHECK permits non-null `linked_purchase_return_id` only for `INPUT+RED_LETTER`; owner ports enforce same entity/supplier and exact returned-line coverage in the shared transaction.
- [ ] **Step 5: Verify.** Run focused DB tests and `cargo xtask sqlcheck`; expected exact counts and no NULL bypass.
- [ ] **Step 6: Commit.** `git commit -m "feat(invoice): add line-based invoices and central number registry"`

### Task 11: Implement decimal tax and common invoice contracts

**Files:**
- Create: `crates/contract/invoice/src/input.rs`
- Create: `crates/domain/invoice/src/amounts.rs`
- Create: `crates/domain/invoice/src/tax.rs`
- Create: `crates/domain/invoice/src/identifier.rs`
- Modify: `crates/contract/invoice/src/lib.rs`
- Modify: `crates/domain/invoice/src/lib.rs`
- Test: `crates/domain/invoice/tests/f50_amounts.rs`

- [ ] **Step 1: Write failing contract/rounding tests.** Include `0.05 × 10% -> 0.01`, diff 0.02 accepted/0.03 rejected, exact gross equation, 0% tax, pure-tax exception and both numbering schemes.
- [ ] **Step 2: Add the four exact input types.** Implement `InvoiceIdentifierInput`, `SalesInvoiceLineInput`, `PurchaseInvoiceLineInput`, `InvoiceReversalLineInput` exactly as F-50 §6.5; no head amount or head tax fields.
- [ ] **Step 3: Implement one validator.** HTTP, plugin and Excel call the same pure validator before any repository access.
- [ ] **Step 4: Verify no binary float.** Static source check must reject `f32/f64` in invoice/finance amount modules.
- [ ] **Step 5: Commit.** `git commit -m "feat(invoice): freeze decimal line and identifier contracts"`

### Task 12: Implement sales and purchase invoice registration

**Files:**
- Create: `crates/application/invoice/src/usecase/register_sales_invoice.rs`
- Create: `crates/application/invoice/src/usecase/register_purchase_invoice.rs`
- Create: `crates/application/invoice/src/repository/invoice_number_registry.rs`
- Create: `crates/application/invoice/src/repository/invoices.rs`
- Test: `crates/application/invoice/tests/f50_register_invoice_pg.rs`

- [ ] **Step 1: Write failing tests.** Multi-line 13%/6% sales, 13%/0% purchase, number race, unauthorized duplicate detail, head-line sum rollback, mixed purchase `cost_kind` rejection, and absence of all server-owned result fields from write inputs. Purchase responses must expose split in-stock/released variance rather than an undifferentiated total-variance field.
- [ ] **Step 2: Implement the single transaction without nullable placeholders.** Pre-generate the invoice UUID and AR/AP `ORIGINAL` entry UUID; use Task 1A to collect, lock, reload and seal the complete invoice/receipt/GRNI/inventory/AP/reservation/advance/settlement plan **before** deriving any amount or invoking a mutator. Pass the same `TransactionLockProof` to GRNI, variance, payable/overbilling and settlement owners; those owners only reload and never supplement locks. Allocate the central number owner, derive every line/head amount and GRNI/variance/overbilling result, and use locked same-entity/party/contract advance candidates in `created_at ASC,id ASC`. Insert the AR/AP `ORIGINAL` plus paired advance `APPLY`/`ADVANCE_AUTO` effects with the pre-generated invoice id; the reciprocal invoice↔AR/AP composite foreign keys are `DEFERRABLE INITIALLY DEFERRED`, so this is valid only inside the same transaction and must be complete at commit. Compute `advance_auto_applied_amount=A`, then call `PostingPort::post` with the pre-generated invoice id before inserting an invoice head. Only `Posted` may continue with first-write inserts; `IdempotentReplay` must re-read and return the already complete invoice graph with exactly matching ids; `Skipped` or replay without the complete graph is an invariant failure. With the returned non-null voucher/period values, insert the invoice head/lines, unbilled/link rows and owner writeback as one deferred-FK-consistent set. Persist the four non-null purchase results `accrual_reversal_amount`、`price_variance_in_stock_amount`、`price_variance_released_amount`、`overbilling_amount` (zero when inapplicable) and `advance_auto_applied_amount=A`. Re-read sums, number owner, both non-null back-pointers and all subledger/ledger balances, then write event/outbox; any mismatch rolls back everything. Never insert a null `voucher_id`/AR/AP id and never open an UPDATE path merely to backfill them.
- [ ] **Step 3: Ensure owner-safe duplicate mapping.** Authorized users may receive existing document link; others receive empty details.
- [ ] **Step 4: Verify payloads include line arrays, registry id and posting results.** Both invoice responses/events include `advance_auto_applied_amount`; purchase additionally includes the four exact server result fields and never emits an undifferentiated total-variance field or an overbilling boolean. Run integration tests including F-50 case 18 and supplier mirror; assert paired effects preserve funding roots, the invoice period carries both effects and original advance periods remain unchanged.
- [ ] **Step 5: Commit.** `git commit -m "feat(invoice): register multi-line sales and purchase invoices"`

### Task 13: Implement VOID and multi-step red reversals

**Files:**
- Create: `crates/domain/invoice/src/reversal/effects.rs`
- Create: `crates/domain/invoice/src/reversal/remaining.rs`
- Create: `crates/application/invoice/src/usecase/void_sales_invoice.rs`
- Create: `crates/application/invoice/src/usecase/register_invoice_reversal.rs`
- Create: `crates/application/finance/src/settlement/release_for_reversal.rs`
- Test: `crates/application/invoice/tests/f50_reversal_pg.rs`

- [ ] **Step 1: Write failing tests.** Cover effect combinations, pure tax, adjusted then fake final original-price line, partial repeats, quantity/amount overrun, VOID/red race and amount exhaustion with audit quantity remaining. Add linked purchase-return tests: every public HTTP/plugin/Excel request carrying `linked_purchase_return_id` is rejected; the internal purchase-return port requires it; same entity/supplier/line coverage, unbilled/billed/mixed segmentation, `+100,-100,+30,-30`, zero-price GRNI roots and transaction rollback on either half failing all hold.
- [ ] **Step 2: Implement the exact internal credit-note contract and remaining per source line.** Declare `RegisterPurchaseCreditNote`、`PurchaseCreditNoteGrniReopen`、`PurchaseCreditNoteView` 与 `PurchaseCreditNotePort` exactly as F-50 §6.5, including nullable `linked_purchase_return_id`, identifier, posting date, `expected_original_row_version`, both effect kinds, tax rate/gross, non-null voucher/period fields and itemized plus summed GRNI reopen results; add the required `f50_lock_proof: &TransactionLockProof` method parameter, do not recreate the old A-11 DTO or expose `source_effect_seq` as input. Persist the command's nullable link; before this mutator, the transaction owner must collect every involved payment/refund, return, invoice, receipt, GRNI, inventory, reservation and settlement key, use Task 1A to seal the plan, and pass the same proof to every owner. Lock/reload original header/lines, compare the original version, validate any linked return through the procure owner port, aggregate registered reversal lines, and assign each active source line's next contiguous `source_effect_seq`; replay the same original-price/tail-rounding classifier as the deferred database trigger before insert. State is monetary/tax finality.
- [ ] **Step 3: Implement release formula.** `L=max(0,current_gross-effective_open_before)` with remaining reversible cap and two-level LIFO; every DIRECT_CASH segment creates a traceable advance.
- [ ] **Step 4: Post one balanced voucher.** Carry `released_settlement_amount=L`; no bank/cash leg in invoice reversal. Linked inventory returns use `PURCHASE_INVOICE_LINKED_RETURN_REVERSED`, reopen GRNI and post only `linked_return_price_difference_amount` to COGS—never inventory; the procure half alone posts the single `PURCHASE_RETURN_INVENTORY` voucher, debiting original GRNI, crediting locked current carrying value, and sending the signed difference to COGS. Partial returns use moving average; a return that exhausts quantity takes the full pre-return inventory balance so quantity, amount, and unit price all become zero. Linked direct-expense/direct-ship returns use `PURCHASE_INVOICE_DIRECT_EXPENSE_REVERSED` and do not call GRNI or inventory.
- [ ] **Step 5: Verify 45-matrix cases 1–5, 9–12, 22–36, 44.** Expected: exact states, amounts and error codes.
- [ ] **Step 6: Commit.** `git commit -m "feat(invoice): support constrained repeated line reversals"`

### Task 14: Upgrade supplier invoice upload and freeze writeback port

**Files:**
- Modify: `db/migrations/portal/V20261018092900__portal_create_supplier_invoice_uploads.sql`
- Create: `db/migrations/portal/V20261018093000__portal_create_supplier_invoice_upload_lines.sql`
- Create: `db/migrations/portal/V20261018093100__portal_create_supplier_invoice_upload_attachments.sql`
- Create: `crates/contract/portal/src/supplier_invoice_upload.rs`
- Create: `crates/application/portal/src/supplier_invoice_upload_writeback.rs`
- Modify: `crates/contract/portal/src/lib.rs`
- Modify: `crates/application/portal/src/lib.rs`
- Test: `crates/application/portal/tests/f50_supplier_invoice_upload_pg.rs`

- [ ] **Step 1: Write failing schema and port tests.** Assert 31/33/31 counts, multi-line upload, head sum, same supplier/entity, accepted/returned transitions, illegal status-field shapes and optimistic version. A real PostgreSQL test must prove the generated `active_identifier_slot` plus ordinary unique constraint accepts multiple RETURNED history rows but rejects a second active UPLOADED/ACCEPTED row；迁移测试同时断言第 33 支 portal 外键追补已执行且不存在部分索引。
- [ ] **Step 2: Declare the local id and port.** `SupplierInvoiceUploadId` is an opaque `Id<LocalMarker>` in this contract crate. Implement the exact `accept` and `return_upload` signatures from F-50 §6.6 with `ctx: &SecurityContext`; `return_upload` must receive `expected_row_version: i64`.
- [ ] **Step 3: Implement owner logic.** `ep-app-portal` owns state changes. Upload does not occupy central invoice number before acceptance. Generate `identifier_key` and `active_identifier_slot` in PostgreSQL, enforce the three state shapes with NULL-safe CHECK and use ordinary `UNIQUE(legal_entity_id,supplier_id,active_identifier_slot)`; do not add a partial index or a client-computed normalization path.
- [ ] **Step 4: Freeze the attachment migration at 1208** so the final ordered roster has 32 files without a duplicate version; the pre-implementation stage plan's former 1207 attachment slot is historical and must not be created.
- [ ] **Step 5: Verify and commit.** `git commit -m "feat(portal): make supplier invoice uploads line based"`

### Task 15: Connect portal acceptance to purchase-invoice registration

**Files:**
- Modify: `crates/application/invoice/src/usecase/register_purchase_invoice.rs`
- Modify: `apps/core-server/src/wiring/mod.rs`
- Create: `apps/core-server/src/http/portal/supplier_invoice_uploads.rs`
- Modify: `docs/openapi/portal.v1.yaml`
- Test: `apps/core-server/tests/f50_supplier_invoice_acceptance.rs`

- [ ] **Step 1: Write failing transaction tests.** Acceptance creates invoice head/lines/number and portal ACCEPTED together; duplicate number or concurrent return leaves no formal invoice.
- [ ] **Step 2: Use one transaction and the owner port.** When `supplier_invoice_upload_id` is supplied, request lines are forbidden; the use case locks and copies upload head/lines, then calls `accept` after invoice writes but before commit.
- [ ] **Step 3: Add return endpoint.** `POST /api/v1/portal/supplier-invoice-uploads/{id}/actions/return` accepts only `reason,row_version`, maps `row_version` unchanged to `expected_row_version`, and uses the same owner port for the locked version/state transition; suppliers cannot call it.
- [ ] **Step 4: Emit ownership-correct events.** RETURNED is stage 7 event; ACCEPTED is written by the stage 10 acceptance transaction. Update stage event totals 15 and 13.
- [ ] **Step 5: Commit.** `git commit -m "feat(invoice): atomically accept supplier invoice uploads"`

### Task 16: Make HTTP, plugin, and Excel share the v2 invoice contract

**Files:**
- Create: `apps/core-server/src/http/invoice/line_inputs.rs`
- Create: `crates/application/invoice/src/import/v2.rs`
- Create: `crates/contract/invoice/src/plugin_schema.rs`
- Modify: `docs/openapi/invoice.v1.yaml`
- Modify: `docs/openapi/finance.v1.yaml`
- Test: `crates/application/invoice/tests/f50_import_v2.rs`
- Test: `apps/core-server/tests/f50_invoice_contract_parity.rs`

- [ ] **Step 1: Write parity tests.** The same logical document through JSON, plugin payload and each v2 Excel template yields identical normalized contract input and identical errors.
- [ ] **Step 2: Implement templates.** Only `sales-invoice-register-v2`, `purchase-invoice-register-v2`, `invoice-reversal-register-v2`; group by `document_key`, require unique 1-based line_no and identical repeated heads.
- [ ] **Step 3: Reject legacy shape.** Any head tax/head amount or old template returns the exact F-50 codes; do not ignore columns.
- [x] **Step 4: Use the frozen complete public surface of 49 operations.** Registry has no public endpoint. The two OpenAPI files already carry `x-scope: stage10-full-surface` and exactly match the stage 10 §5 method/path set (23 writes, 26 reads); implementation and parity tests consume them directly and must continue to assert no missing, extra or duplicate method/path and no legacy inflated endpoint count.
- [ ] **Step 5: Commit.** `git commit -m "feat(invoice): unify HTTP plugin and Excel line contracts"`

### Task 17: Update historical trade selection

**Files:**
- Create: `crates/contract/mdm/src/trade_history.rs`
- Create: `crates/application/mdm/src/trade_history/aggregate.rs`
- Create: `crates/application/sales/src/trade_history.rs`
- Create: `crates/application/procure/src/trade_history.rs`
- Create: `crates/application/invoice/src/trade_history/sales.rs`
- Create: `crates/application/invoice/src/trade_history/purchase.rs`
- Modify: `crates/application/sales/src/lib.rs`
- Modify: `crates/application/procure/src/lib.rs`
- Test: `crates/application/mdm/tests/f50_trade_history.rs`

- [ ] **Step 1: Write failing four-provider tests.** Cover active, original-price partial reduction, ADJUSTED partial, terminal, and monetary-zero/quantity-positive records.
- [ ] **Step 2: Extend `TradeHistoryItem`.** Add remaining business quantity, remaining net/tax/gross, `is_visible_by_default`, `is_selectable_as_price_source`; MDM never parses provider status strings.
- [ ] **Step 3: Implement revalidation.** A price selection command reloads the source and returns `PRICE_SOURCE_NO_LONGER_ELIGIBLE` if changed.
- [ ] **Step 4: Replace config use.** Remove `INCLUDE_VOIDED`; use `INCLUDE_INEFFECTIVE` only for visibility, never selectability.
- [ ] **Step 5: Commit.** `git commit -m "feat(mdm): separate trade visibility from price eligibility"`

### Task 18: Rewire all effective-open consumers and source-action order

**Files:**
- Modify: `crates/application/sales/src/lib.rs`
- Modify: `crates/application/procure/src/lib.rs`
- Modify: `crates/application/portal/src/lib.rs`
- Modify: `crates/application/reporting/src/lib.rs`
- Modify: `crates/contract/finance/src/current_balance.rs`
- Test: `testkit/tests/f50_balance_consumer_contract.rs`

- [ ] **Step 1: Write a single contract fixture.** One invoice, settlement and partial reversal must return the same effective balance through settlement candidate, credit exposure, payment cap, customer/supplier portal, aging, report, Excel and reconciliation.
- [ ] **Step 2: Replace all ambiguous consumers.** No consumer reads mutable `open_amount` directly; all call current balance contract or governed current view.
- [ ] **Step 3: Fix source-action order.** Sales return, purchase return and contract termination persist source facts first; invoice correction is conditional downstream work and references the source action.
- [ ] **Step 4: Verify no second balance.** Run focused integration and `rg` guard for forbidden current consumers.
- [ ] **Step 5: Commit.** `git commit -m "feat(workflows): consume effective balances and source actions consistently"`

### Task 19: Upgrade T0 and benchmark data generation

**Files:**
- Modify: `datagen/src/t0_min.rs`
- Create: `datagen/src/finance_invoice.rs`
- Modify: `testkit/src/lib.rs`
- Test: `datagen/tests/f50_t0.rs`

- [ ] **Step 1: Write failing T0 test.** Generated minimum must contain registry owner, one sales header, at least one line, exact head sums, AR ORIGINAL and one settlement root.
- [ ] **Step 2: Update minimum and scale generators.** Scale invoices are multi-line/multi-rate and may contain legal partial reversals; no head-only fixture helper remains.
- [ ] **Step 3: Verify deterministic seeds.** Same seed yields identical ids, amounts and registry keys.
- [ ] **Step 4: Run T0.** `cargo test -p ep-datagen --test f50_t0`. Expected: PASS without direct fixture SQL.
- [ ] **Step 5: Commit.** `git commit -m "test(datagen): generate valid F-50 invoice and settlement graphs"`

### Task 20: Add concurrency, security, and reconciliation end-to-end gates

**Files:**
- Create: `testkit/tests/f50_concurrency_pg.rs`
- Create: `testkit/tests/f50_rls_pg.rs`
- Create: `testkit/scenarios/f50_financial_consistency.rs`
- Create: `testkit/tests/f50_financial_consistency.rs`
- Modify: `testkit/tests/rls_matrix.rs`
- Modify: `apps/job-worker/src/wiring/mod.rs`

- [ ] **Step 1: Implement the 45 acceptance cases one-for-one.** Test names start `f50_01_...` through `f50_45_...`; no omitted or combined case；第 45 项逐个覆盖第 10.1 节全部错误码的端到端负例。
- [ ] **Step 2: Add lock-set drift tests.** Concurrent reversal/refund/cash reversal/purchase return/purchase invoice must commit in one legal serial order or retry the whole transaction; cover missing category, missing id, wrong owner, forged proof, proof from another transaction, dependency set growth and payable reservation versus AP reduction. No partial lock extension or pre-proof business SQL may occur.
- [ ] **Step 3: Add RLS/leakage tests.** All 39 stage-10 tables, 13 ledger RLS tables and 31 stage-7 RLS tables use the shared matrix; duplicate-number detail is permission-sensitive.
- [ ] **Step 4: Add closed-period replay.** Re-run M1/M2 after M4 and compare byte-stable period results; inject discrepancy and assert close blocked without auto-adjustment.
- [ ] **Step 5: Run.** `cargo test -p ep-testkit --test f50_concurrency_pg --test f50_rls_pg && cargo test -p ep-testkit --test f50_financial_consistency`. Expected: PASS.
- [ ] **Step 6: Commit.** `git commit -m "test(f50): cover the complete financial consistency matrix"`

### Task 21: Final integration, documentation sync, and release evidence

**Files:**
- Modify: `docs/data-dictionary/ledger.md`
- Modify: `docs/data-dictionary/finance.md`
- Modify: `docs/data-dictionary/invoice.md`
- Modify: `docs/data-dictionary/portal.md`
- Modify: `docs/openapi/ledger.v1.yaml`
- Modify: `docs/openapi/finance.v1.yaml`
- Modify: `docs/openapi/invoice.v1.yaml`
- Modify: `docs/openapi/portal.v1.yaml`
- Modify: `docs/event-catalog.md`
- Modify: `docs/error-codes.md`
- Modify: `docs/config-reference.md`
- Modify: affected stage plans only if implementation exposes a proven mismatch; never silently diverge from F-50.

- [ ] **Step 1: Run full validation.** `cargo fmt --check`, `cargo clippy --workspace --all-targets -- -D warnings`, `cargo test --workspace`, `cargo xtask archcheck`, `cargo xtask sqlcheck`, `cargo xtask errorcodes`, `cargo xtask configdoc`, `cargo xtask eventcatalog`.
- [ ] **Step 2: Run static forbidden-phrase checks.** Active implementation must have zero uses of old origin/refund direction, old min formula, fixed full voucher reversal, head tax rate, single source invoice id, one-reversal uniqueness and direct current open balance consumers.
- [ ] **Step 3: Recount mechanically.** Assert the Frozen Counts table exactly; the document gate must reject unresolved placeholder markers and approximate count language in current F-50 artifacts.
- [ ] **Step 4: Re-run 45 cases and T0.** Archive JUnit, PostgreSQL version, migration roster, RLS matrix and event/error/config reports into release evidence.
- [ ] **Step 5: Request review.** Use `superpowers:requesting-code-review`, resolve findings with `superpowers:receiving-code-review`, then rerun the entire gate.
- [ ] **Step 6: Finish the branch.** Use `superpowers:verification-before-completion` and `superpowers:finishing-a-development-branch`; only after all evidence is green mark implementation complete.
- [ ] **Step 7: Final commit.** `git commit -m "feat(f50): complete financial consistency and invoice model"`

---

## Plan Self-Review

- **Coverage:** Tasks 2–4 close correction vouchers and dynamic ledger posting; 5–9 close settlement/refund/current/history; 10–16 close invoice/number/portal/contracts; 17–18 close history and consumers; 19–20 close T0 and all 45 acceptance cases; 21 closes documentation and release gates.
- **Placeholder scan:** This plan contains no unresolved decision markers. Counts, routes, trait names, migration names, view names, errors and event ownership are frozen above.
- **Type consistency:** Money/Rate/IDs, `Tx`, `SecurityContext`, error envelopes, event envelopes and RLS context reuse existing project foundations; `RequestCtx` is not a project type and must not be introduced, while the one new portal ID remains owner-contract-local.
- **Authority:** If any implementation detail appears inconsistent, stop and amend the F-50 design and this plan in the same reviewed document change before code continues. Do not choose an undocumented third interpretation.
