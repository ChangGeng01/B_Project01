## 阶段 10：财务内核二 —— 往来与发票

本阶段交付 invoice 与 finance 两个模块码的契约层、领域层、应用层与数据库结构，覆盖销项发票申请与开具登记、销项与进项两个方向的作废与红字冲销登记、应收应付台账与账龄、预收预付台账、到款与付款登记与核销、客户退款与供应商返款、资金账户与资金腿明细、应收账款未开票过渡科目子账、待处理超量开票子账与三条结清路径，以及规格第 17.3 章与 PRD 第 6.13.1 合计十个勾稽项中归属本阶段的八项对账视图。

本阶段不定义任何借贷方向、科目、取价与拆分规则。全部账务处理指向规格第 5.2 章财务规则条目的事件-分录表及其后七个规则块，本文只写它在哪一步被调用、以什么参数被调用、结果落到哪张表。

---

### 0. 本阶段的口径前提与显式假设

#### 0.1 过账时机：同步在业务事务内生成凭证

本阶段的决定：由本阶段单据直接触发的七类事件（开票、红字冲销与作废、到款、付款、退款、超量开票路径二结清、超量开票路径三结清），其总账凭证、子账台账条目与核销关系在同一个业务数据库事务内同步写入，凭证经 `ep-contract-ledger` 的过账端口生成，本阶段不向 Outbox 投递任何需要异步过账的条目。

理由有三条。其一，PRD 第 6.4.8、6.7.7、6.8.8 三张异常表都把“凭证生成失败或借贷不平”的系统行为定为“提交失败并进入死信与人工修复”，提交失败只有在凭证与业务写入同事务时才成立。其二，核销明细行的逐行上限校验必须读到该单据当前的未核销余额，异步过账会让紧邻的两次登记读到过期余额。其三，规格第 5.2 章要求同一业务事件的子账条目与凭证共用同一个会计期间字段，同事务写入使该要求成为结构性事实而不是运行期约定。

与规格第 10.2 章关账受理前提二的关系：本阶段发布的领域事件一律不进入过账消费者订阅清单，因此“该法人记账日期属于该会计期间的待消费过账条目数”不因本阶段而非零。本阶段的 Outbox 条目只驱动派生传播，即站内通知、报表投影、检索索引与客户 360 视图。

与顺延入账的关系：会计期间在业务事务内由 `AccountingPeriodResolver` 一次解析并同时写入凭证与全部子账条目，因此规格第 10.2 章“受理时点在途写事务”这一集合天然覆盖本阶段的在途提交，等待该集合结束后建立的快照必然包含这些凭证。

本节属于接缝口径，需要整合员在 14 个阶段之间统一确认。若整合结论改为异步过账，本阶段的改动范围是：把七个用例的凭证生成腿改为向 Outbox 投递、把 PRD 三张异常表的“提交失败”改为“提交成功后进入死信”、并为核销上限校验引入待过账占用量，代价为中等偏高。

#### 0.2 规格缺口：自动核销预收预付的分录腿

规格第 5.2 章事件-分录表的开票事件只列出借应收账款、贷应交税费销项税额与应收账款未开票过渡科目三腿，采购发票事件同样没有预付账款腿；而到款事件与付款事件都要求“后续开票时按同一合同的收付款计划自动核销预收账款”“后续采购发票登记时自动核销预付账款”。若自动核销只动台账不动总账，预收台账余额下降而预收账款科目余额不变，规格第 17.3 章的预收与预付两项勾稽必然破裂。

本阶段的显式假设：自动核销作为该事件凭证的附加分录腿写入同一张凭证，销项方向为借预收账款、贷应收账款，进项方向为借应付账款、贷预付账款，金额为本次自动核销金额。该假设需回写规格第 5.2 章事件-分录表的开票事件与采购发票事件的分支与附加规则列。在回写完成前，实现按本假设执行，`ep-contract-ledger` 的过账端口按“附加腿”参数接收，不需要新增事件类型。

#### 0.3 规格缺口：资金账户期初余额

PRD 第 6.2.2 的资金账户字段表没有期初余额，而第 6.2.4 的资金腿明细视图要求展示期初余额，规格第 17.3 章又要求资金流水台账按账户的余额合计等于银行存款科目余额。上线首期若科目有期初余额而资金腿明细没有，该项勾稽在首期即为非零差额。

本阶段的显式假设：`finance.cash_accounts` 增加 `opening_balance` 与 `opening_balance_period_id` 两列，建档时录入一次，建档后不可修改；同一法人下全部银行存款类账户的期初余额合计必须等于总账银行存款科目的期初余额，现金类账户同理，该等式由本阶段的对账视图逐期校验。历史存量往来与存量资金的批量录入通道属历史数据导入阶段，本阶段只提供字段、校验与勾稽。

#### 0.4 被阻塞的业务决策项与本阶段的临时取值

下表逐条对应 PRD 第 6.16 节的 F 编号与附录乙四的 U-D 编号。本阶段不代替财务负责人决策，但每一条都给出临时取值，否则表结构与校验无法落地。临时取值一律以配置项或配置发布对象承载，切换时不改表结构的标注为低代价。

| 编号 | 临时取值 | 承载方式 | 切换代价 |
|---|---|---|---|
| F-01 / U-D-03 | 发票号码 `text` 且 `char_length <= 64`，法人内唯一；发票代码 `text` 且 `char_length <= 32`，可空，发票种类为数电时必须为空，为纸质时必填 | 表列与 CHECK 约束，发票种类为 `invoice_kind` 列 | 低，改 CHECK 与校验函数 |
| F-02 / U-D-04 | 税率取自 `invoice.tax_rate_options`，出厂预置 0.130000、0.090000、0.060000、0.030000、0.010000、0.000000；一张发票单税率、单行金额，不做多行明细 | 配置发布对象写入 `invoice.tax_rate_options` | 税率集合为低；多行明细为中，需新增 `invoice.sales_invoice_lines` 与分摊逻辑 |
| F-03 / U-D-05 | 舍入按共享基线第 3.5 节；容差判据为 `abs(tax_amount - round(net_amount * tax_rate, 2)) <= tolerance`，`tolerance` 默认 0.02 | 配置项 `EP__INVOICE__TAX__AMOUNT_TOLERANCE` | 低 |
| F-04 / U-D-06 | 剩余可开比例的计算基数为合同金额；比例列类型 `numeric(9,6)`；累计比例校验容差 0.000001 | 配置项 `EP__INVOICE__RATIO__TOLERANCE` | 基数改为订单金额合计为中，需改取数与回滚公式 |
| F-05 / U-D-07 | 申请金额不可人工改写，等于 `round(开票比例 * 合同金额, 2)` | 领域规则 | 低 |
| F-06 / U-D-08 | 开票内容为自由文本，长度上限 500 | 表列 CHECK | 改为逐行对应为中 |
| F-07 | 开具登记不强制上传影像附件 | 配置项 `EP__INVOICE__ISSUE__REQUIRE_IMAGE_ATTACHMENT`，默认 false | 低 |
| F-08 / U-D-09 | 红字发票号码必填；首版只允许全额红冲，红字不含税金额、税额、价税合计必须分别等于原发票对应值 | 领域规则 | 允许部分红冲为中偏高，需把销项发票状态机拆出部分红冲态并改比例回滚公式 |
| F-09 / U-D-10 | 作废与红字冲销登记复用开票的高风险控制，即重新认证加审批 | 配置项 `EP__INVOICE__REVERSAL__REQUIRES_REAUTH`，默认 true | 低 |
| F-10 / U-D-11 | 账龄分档为 0 至 30、31 至 60、61 至 90、91 至 180、181 至 360、361 以上，六档 | 配置发布对象写入 `finance.aging_bucket_definitions`，按法人可配 | 低 |
| F-11 / U-D-12 | 到期日取值优先级为：关联收付款计划行的到期日；缺失时取发票开具日期加往来方档案上的约定账期天数；仍缺失时取发票开具日期 | 领域服务 `DueDateResolver`，账期天数经 `ep-contract-mdm` 读取 | 低 |
| F-12 / U-D-13 | 可核销范围限定为同一法人同一往来方，不允许跨客户或跨供应商核销 | 配置项 `EP__FINANCE__SETTLEMENT__CROSS_PARTY_ALLOWED`，默认 false | 放开为中，需改越权测试集与账龄归属 |
| F-13 / U-D-14 | 到款登记不需重新认证也不需审批；客户退款与供应商返款需重新认证加审批；资金账户档案的新增、修改与停用需审批不需重新认证 | 三个配置项，见第 7 节 | 低 |
| F-14 / U-D-02 | 新增资金单据冲正登记单，见第 4.7 节，是本阶段为闭合该缺口而新增的单据类型 | 表 `finance.cash_document_reversals` 与同名用例 | 若财务负责人另选路径，改动为一张表加一个用例，中等 |
| F-15 / U-D-15 | 可退上限见第 4.8 节的算法 | 领域服务 `RefundCapCalculator` | 低 |
| F-16 | 批量导入单次上限 2000 行，逐行独立事务落库，失败行不回滚已成功行，逐行返回原因 | 两个配置项 | 改为整体回滚为中，需把导入改为单事务并放弃逐行幂等 |
| F-17 | 银行账号列表与导出显示后 4 位，详情需字段级权限 `finance.cash_account.bank_account_no.read_full`；银行账号纳入敏感字段清单，字段级密级 30 | 字段级权限与密级 | 低 |
| F-18 / U-D-16 | 同一法人允许多个现金账户并存 | 无唯一约束 | 收紧为唯一为低 |
| F-19 | 按共享基线第 11.5 节，本阶段不另取值 | 共享基线 | 无 |
| F-20 | 文案集中在 `docs/error-codes.md`，代码只引用常量 | 共享基线第 10.2 节 | 无 |
| F-21 | 交叉引用按本文正文写明的 PRD 节号，不写节名 | 本文 | 无 |

被阻塞判定：本阶段不因上述任一条被阻塞，全部有可执行的临时取值。风险最高的是 F-08 与 F-16，两者若在阶段结束后才改，会触及已落库数据的语义，需要数据回填。

---

### 1. 交付物清单

本阶段结束时，下列各项在单台服务器上可运行、可演示、可用自动化用例判定。

1. 两个模块的三层 crate 全部编译通过并接入 `core-server`：`ep-contract-invoice`、`ep-domain-invoice`、`ep-app-invoice`、`ep-contract-finance`、`ep-domain-finance`、`ep-app-finance`。
2. `db/migrations/invoice/` 与 `db/migrations/finance/` 两个迁移目录可离线执行到最新版本，且可按各文件头 `-- rollback:` 段回退到本阶段起点。
3. 34 张业务表与 14 个只读视图在 `ep` 库中建立，其中 invoice 11 张、finance 23 张；全部带法人列的表已 `ENABLE` 且 `FORCE` 行级安全并挂上统一策略。
4. 56 个 HTTP 端点在 `/api/v1/invoice/**` 与 `/api/v1/finance/**` 下可用，含 OpenAPI 描述文件 `docs/openapi/invoice.v1.yaml` 与 `docs/openapi/finance.v1.yaml`。
5. 12 个对外契约 trait 在 `ep-contract-finance` 与 `ep-contract-invoice` 中定义并有默认实现注册到 `apps/core-server/src/wiring.rs`，供 sales、procure、inventory、clm、portal、reporting 六个模块调用。
6. 11 个领域事件登记到 `docs/event-catalog.md` 并可从 `platform_msg.outbox_events` 中查得。
7. 规格第 17.3 章与 PRD 第 6.13.1 合计 10 个勾稽项中的 8 项对账视图可在应用内按法人与会计期间查询并展示子账侧、总账侧与差额三列，另 2 项（存货、应付账款暂估）的子账侧由其他阶段提供，本阶段只提供视图外壳与总账侧取数。
8. 一条可重复执行的端到端脚本 `testkit/scenarios/stage10_ar_ap_closed_loop.rs`，覆盖规格第 8 章闭环第 6、7、9、10、11 步，并串起规格第 17.2 章十五类必测分支中的第二、五、六、八、九、十三类。
9. `ep-datagen` 增加往来与发票子集生成器，可在基准规模下产出销项发票 6 万张、应收明细条目 6 万条、应付明细条目 4 万条、到款单 3 万张、付款单 2 万张、资金腿明细 6 万条，用于附录 A.1 的应收账龄分析与应付账龄分析两项报表实测。
10. `docs/error-codes.md` 增补本阶段错误码（第 5 节列出的全部错误码，合计 80 个上下），`docs/data-dictionary/invoice.md` 与 `docs/data-dictionary/finance.md` 两份数据字典。
11. 三个新增指标接入 ops-agent 暴露端点。

---

### 2. crate 与进程归属

#### 2.1 新增 crate

| crate | 目录 | 进程 | 职责 |
|---|---|---|---|
| ep-contract-invoice | crates/contract/invoice | 装配进 core-server、job-worker | 发票申请、销项发票、冲销登记的命令与查询 DTO、事件类型、供 clm 与 procure 调用的 trait |
| ep-domain-invoice | crates/domain/invoice | 同上 | 发票申请单与销项发票聚合、剩余可开比例值对象、税额勾稽规则、冲销互斥规则 |
| ep-app-invoice | crates/application/invoice | core-server 承载全部用例，job-worker 承载批量导入任务 | 开具登记、冲销登记、批量导入、进项发票台账组装 |
| ep-contract-finance | crates/contract/finance | 装配进 core-server、job-worker、portal-gateway 的调用方 | 应收应付预收预付台账查询 DTO、供 sales、procure、inventory、portal 调用的 trait |
| ep-domain-finance | crates/domain/finance | 同上 | 台账条目聚合、核销分配算法、账龄分桶、可退上限、超量开票余额推进 |
| ep-app-finance | crates/application/finance | core-server 承载全部用例，job-worker 承载对账取数 | 到款、付款、退款、冲正、资金账户、对账视图 |

#### 2.2 改动的既有 crate

| crate | 改动 | 归属阶段 |
|---|---|---|
| ep-foundation | 增加 `TaxRate`（复用 `Rate` 的 newtype）、`IssueRatio`（复用 `Rate`）、`SettlementAmount`（复用 `Money` 且约束非负）三个 newtype；增加 `AccountingPeriodRef` 的 `is_deferred` 标记 | 阶段 1 建立，本阶段追加 |
| ep-adapter-db-pg | 增加 `invoice` 与 `finance` 两个仓储子模块，按 schema 分文件，一个仓储只访问自己模块的 schema，共 34 个仓储实现 | 阶段 1 建立，本阶段追加 |
| ep-platform-sequence | 注册 8 个新的单据类型码：`INVA` 发票申请、`SINV` 销项发票、`IRVS` 冲销登记、`RCPT` 到款、`PAYM` 付款登记、`RFND` 退款与返款、`CDRV` 资金冲正、`OBST` 超量开票结清 | 阶段 2 建立，本阶段追加类型码 |
| ep-platform-authz | 注册 14 个对象类型与 12 个动作 | 阶段 2 建立，本阶段追加注册项 |
| ep-testkit | 增加 `CashAccountFixture`、`InvoiceApplicationBuilder`、`SalesInvoiceBuilder`、`ReceiptBuilder`、`PaymentBuilder`、`RefundBuilder`、`ReceivableEntryProbe`、`ReconciliationProbe` 八个构造器与探针 | 阶段 1 建立，本阶段追加 |
| ep-datagen | 增加往来与发票子集 | 阶段 1 建立，本阶段追加 |
| apps/core-server | `wiring.rs` 注入 12 个契约实现，路由注册 56 个端点 | 本阶段追加 |
| apps/job-worker | 注册批量导入任务处理器与对账取数语句集 | 本阶段追加 |

本阶段不新增进程、不新增 schema、不新增模块码、不新增错误分类、不新增依赖方向。`ep-domain-finance` 与 `ep-domain-invoice` 不依赖对方，跨模块只由 `ep-app-invoice` 依赖 `ep-contract-finance`，反向不成立。

---

### 3. 数据库变更

全部表遵守共享基线第 3 节与第 4 节。以下每张表只列出公共列之外的专有列，公共列按基线第 4 节的九列固定顺序在前。仅追加表不带 `row_version`、`updated_at`、`updated_by`，改带 `reverses_id`。

#### 3.1 invoice schema

##### 3.1.1 invoice.tax_rate_options（配置字典，档案类）

| 列 | 类型 | 可空 | 约束 |
|---|---|---|---|
| code | text | 否 | `ck_tax_rate_options_code_len` 长度 1 至 64；`ux_tax_rate_options_legal_entity_id_code` |
| tax_rate | numeric(9,6) | 否 | `ck_tax_rate_options_range` 取值在 0 与 1 之间闭区间 |
| display_name | text | 否 | 长度上限 200 |
| sort_no | int | 否 | 默认 0 |
| is_active | boolean | 否 | 默认 true |
| deactivated_at | timestamptz | 是 | |

##### 3.1.2 invoice.invoice_applications（发票申请单）

| 列 | 类型 | 可空 | 约束与说明 |
|---|---|---|---|
| doc_no | text | 否 | `ux_invoice_applications_legal_entity_id_doc_no`；类型码 INVA |
| status | text | 否 | `ck_invoice_applications_status` 取值 DRAFT、PENDING_APPROVAL、APPROVED、PARTIALLY_ISSUED、FULLY_ISSUED、CANCELLED |
| customer_id | uuid | 否 | 跨模块逻辑引用 mdm，不建外键 |
| contract_id | uuid | 否 | 跨模块逻辑引用 clm，不建外键 |
| application_date | date | 否 | |
| issue_content | text | 否 | 长度上限 500 |
| issue_ratio | numeric(9,6) | 否 | `ck_invoice_applications_ratio_positive` 大于 0 |
| remaining_ratio | numeric(9,6) | 否 | `ck_invoice_applications_remaining_range` 大于等于 0 且小于等于 `issue_ratio` |
| contract_amount | numeric(18,2) | 否 | 提交时从 clm 快照带出并固化，避免合同变更后比例基数漂移 |
| application_amount | numeric(18,2) | 否 | 等于 `round(issue_ratio * contract_amount, 2)` |
| expected_receipt_date | date | 否 | `ck_invoice_applications_expected_date` 不早于 `application_date` |
| approval_ref | uuid | 是 | 指向 platform_flow 的审批实例 |
| remark | text | 是 | 长度上限 2000 |

另有两张关联表，按共享基线第 3.2 节的 `<a>_<b>_links` 命名：`invoice.invoice_application_sales_order_links`，列为 `invoice_application_id`、`sales_order_id`；`invoice.invoice_application_receipt_plan_links`，列为 `invoice_application_id`、`receipt_plan_line_id`。两表的第二列均为跨模块逻辑引用，不建外键。

##### 3.1.3 invoice.sales_invoices（销项发票）

| 列 | 类型 | 可空 | 约束与说明 |
|---|---|---|---|
| doc_no | text | 否 | `ux_sales_invoices_legal_entity_id_doc_no`；类型码 SINV |
| status | text | 否 | `ck_sales_invoices_status` 取值 ISSUED、VOIDED、RED_REVERSED |
| invoice_application_id | uuid | 否 | 同 schema 外键 `fk_sales_invoices_invoice_applications`，`ON DELETE RESTRICT` |
| customer_id | uuid | 否 | 逻辑引用 |
| contract_id | uuid | 否 | 逻辑引用 |
| invoice_kind | text | 否 | `ck_sales_invoices_kind` 取值 DIGITAL、PAPER |
| invoice_no | text | 否 | `ux_sales_invoices_legal_entity_id_invoice_no`；长度上限 64 |
| invoice_code | text | 是 | 长度上限 32；`ck_sales_invoices_code_by_kind` 表达 DIGITAL 必须为空、PAPER 必须非空 |
| issue_date | date | 否 | |
| posting_date | date | 否 | 记账日期，取 `issue_date` |
| accounting_period_id | uuid | 否 | 由 ledger 端口在业务事务内解析 |
| deferred_from_period_id | uuid | 是 | 非空表示该事件发生过顺延 |
| issued_ratio | numeric(9,6) | 否 | 大于 0 |
| issue_content | text | 否 | 长度上限 500 |
| net_amount | numeric(18,2) | 否 | `ck_sales_invoices_net_positive` 大于 0 |
| tax_rate | numeric(9,6) | 否 | |
| tax_amount | numeric(18,2) | 否 | 大于等于 0 |
| gross_amount | numeric(18,2) | 否 | `ck_sales_invoices_gross_sum` 等于 `net_amount + tax_amount` |
| reversed_net_amount | numeric(18,2) | 否 | 默认 0，全额红冲时等于 `net_amount`，为 F-08 改判部分红冲预留 |
| voucher_id | uuid | 否 | 逻辑引用 ledger.vouchers |
| import_batch_id | uuid | 是 | 非空表示由批量导入产生 |
| reauth_ref | uuid | 否 | 重新认证凭证引用 |
| approval_ref | uuid | 否 | 审批实例引用 |

`gross_amount` 的等式做成数据库 CHECK 而不是只在应用层校验，理由是它是规格第 17.3 章应收勾稽的输入，静默不等会在关账时才暴露。

##### 3.1.4 invoice.invoice_reversals（作废与红字冲销登记单，覆盖销项与进项两个方向）

| 列 | 类型 | 可空 | 约束与说明 |
|---|---|---|---|
| doc_no | text | 否 | `ux_invoice_reversals_legal_entity_id_doc_no`；类型码 IRVS |
| status | text | 否 | `ck_invoice_reversals_status` 取值固定为 REGISTERED |
| direction | text | 否 | `ck_invoice_reversals_direction` 取值 OUTPUT、INPUT |
| reversal_type | text | 否 | `ck_invoice_reversals_type` 取值 VOID、RED_LETTER |
| source_invoice_id | uuid | 否 | 方向为 OUTPUT 时指向 `invoice.sales_invoices.id`，建同 schema 外键；方向为 INPUT 时为跨模块逻辑引用 procure 的采购发票，不建外键；两种情形由 `ck_invoice_reversals_source_by_direction` 与应用层校验共同保证 |
| register_date | date | 否 | |
| posting_date | date | 否 | 取 `register_date` |
| accounting_period_id | uuid | 否 | |
| deferred_from_period_id | uuid | 是 | |
| red_invoice_no | text | 是 | `reversal_type` 为 RED_LETTER 时必填，长度上限 64 |
| red_net_amount | numeric(18,2) | 是 | RED_LETTER 时必填 |
| red_tax_rate | numeric(9,6) | 是 | |
| red_tax_amount | numeric(18,2) | 是 | |
| red_gross_amount | numeric(18,2) | 是 | 等于前两者之和 |
| reason | text | 否 | 长度上限 2000 |
| voucher_id | uuid | 否 | 逻辑引用 |
| overbilling_entry_id | uuid | 是 | 方向为 INPUT 且本次冲销用于结清超量开票时非空，对应规格第 5.2 章超量开票路径二 |
| reauth_ref | uuid | 是 | |
| approval_ref | uuid | 是 | |

冲销登记单不设草稿态持久化：重新认证与审批在提交前完成，审批期间的中间态由 platform_flow 的审批实例承载，不落在本表；提交结果只有 REGISTERED 或直接失败两种。因此本表的 `status` 只有 REGISTERED 一个取值，“作废与红字冲销互斥且只允许一次”可以直接由唯一约束 `ux_invoice_reversals_legal_entity_id_source_invoice_id` 保证，不需要部分索引，与共享基线第 3.10 节禁用部分索引的规定不冲突。保留 `status` 列是为满足共享基线第 4 节对单据类表的固定要求，其 CHECK 约束把取值锁死为 REGISTERED。

##### 3.1.5 invoice.invoice_receipt_plan_links（销项发票与合同收款计划行的勾稽）

列为 `sales_invoice_id`（同 schema 外键）、`receipt_plan_line_id`（跨模块逻辑引用 clm）、`linked_net_amount numeric(18,2)`、`linked_gross_amount numeric(18,2)`、`is_reversed boolean`，加公共列。红冲或作废登记时追加一条 `is_reversed` 为 true 的反向行，不更新原行，理由是勾稽记录属于证据链。

##### 3.1.6 invoice.invoice_import_batches（批量导入批次）

列为 `doc_no`、`status`（`ck_invoice_import_batches_status` 取值 PENDING、RUNNING、SUCCEEDED、PARTIALLY_FAILED、FAILED）、`total_rows int`、`succeeded_rows int`、`failed_rows int`、`file_object_id uuid`（逻辑引用 platform_file）、`result_object_id uuid`、`started_at`、`finished_at`、`reauth_ref`、`approval_ref`，加公共列。

##### 3.1.7 附件关联表

`invoice.invoice_application_attachments`、`invoice.sales_invoice_attachments`、`invoice.invoice_reversal_attachments`，列均为 `owner_id`、`attachment_object_id`、`purpose`、`sort_no` 加公共列，按基线第 4 节。

#### 3.2 finance schema

##### 3.2.1 finance.aging_bucket_definitions（账龄分档配置）

列为 `code`、`display_name`、`from_days int`、`to_days int`（`to_days` 为空表示最后一档开区间）、`sort_no`、`is_active`、`deactivated_at`，加公共列。约束 `ck_aging_bucket_definitions_range` 表达 `from_days >= 0` 且 `to_days` 为空或大于 `from_days`。

##### 3.2.2 finance.cash_accounts（资金账户档案）

| 列 | 类型 | 可空 | 约束与说明 |
|---|---|---|---|
| code | text | 否 | `ux_cash_accounts_legal_entity_id_code` |
| account_name | text | 否 | `ux_cash_accounts_legal_entity_id_account_name`；长度上限 200 |
| account_type | text | 否 | `ck_cash_accounts_type` 取值 BANK、CASH；建档后不可修改 |
| ledger_account_id | uuid | 否 | 逻辑引用 ledger 科目；建档后不可修改 |
| bank_name | text | 是 | BANK 时必填，长度上限 200 |
| bank_account_no_cipher | bytea | 是 | BANK 时必填；按规格第 7.8 章法人密钥域字段级加密存储 |
| bank_account_no_tail | text | 是 | 明文后 4 位，供列表与导出脱敏展示 |
| bank_account_no_hash | bytea | 是 | 法人内加盐哈希，唯一约束 `ux_cash_accounts_legal_entity_id_bank_account_no_hash` 建在其上，用于查重而不落明文 |
| owner_user_id | uuid | 否 | 责任人 |
| opening_balance | numeric(18,2) | 否 | 默认 0，见第 0.3 节 |
| opening_balance_period_id | uuid | 是 | |
| is_active | boolean | 否 | 默认 true |
| deactivated_at | timestamptz | 是 | |
| has_cash_flow | boolean | 否 | 默认 false，首次产生资金腿时置 true，用于 PRD 第 6.2.5 的修改拦截 |
| remark | text | 是 | 长度上限 2000 |

`bank_account_no_cipher` 的字段级密级为 30，字段级密级覆盖的登记由 platform_authz 承载。

##### 3.2.3 finance.receivable_entries（应收明细条目）

| 列 | 类型 | 可空 | 约束与说明 |
|---|---|---|---|
| customer_id | uuid | 否 | |
| contract_id | uuid | 是 | |
| sales_order_id | uuid | 是 | |
| sales_invoice_id | uuid | 否 | 跨模块逻辑引用 invoice，不建外键 |
| source_doc_type | text | 否 | `ck_receivable_entries_source_type` 取值 SALES_INVOICE |
| business_date | date | 否 | 原始业务日期，等于发票开具日期 |
| accounting_period_id | uuid | 否 | |
| deferred_from_period_id | uuid | 是 | |
| due_date | date | 否 | 见第 0.4 节 F-11 |
| original_amount | numeric(18,2) | 否 | `ck_receivable_entries_original_positive` 大于 0；等于发票价税合计 |
| settled_amount | numeric(18,2) | 否 | 默认 0 |
| open_amount | numeric(18,2) | 否 | |
| is_reversed | boolean | 否 | 默认 false，红冲或作废登记后置 true 并把 `original_amount` 冲回 |

三条 CHECK 直接对应规格第 17.3 章的应收应付核销守恒：`ck_receivable_entries_settled_nonneg` 表达 `settled_amount >= 0`；`ck_receivable_entries_settled_le_original` 表达 `settled_amount <= original_amount`；`ck_receivable_entries_open_identity` 表达 `open_amount = original_amount - settled_amount`。负数未核销余额因此在数据库层就不可能落库，PRD 第 6.7.7 与 6.12.7 要求的死信路径由约束违例触发。

##### 3.2.4 finance.payable_entries（应付明细条目）

结构与 3.2.3 对称，差异列为 `supplier_id`、`purchase_order_id`、`purchase_invoice_id`（跨模块逻辑引用 procure）、`source_doc_type` 取值 PURCHASE_INVOICE。三条守恒 CHECK 同构。

##### 3.2.5 finance.advance_receipt_entries（预收台账条目）

| 列 | 类型 | 可空 | 说明 |
|---|---|---|---|
| customer_id | uuid | 否 | |
| contract_id | uuid | 是 | |
| sales_order_id | uuid | 是 | |
| receipt_plan_line_id | uuid | 是 | 逻辑引用 clm |
| receipt_id | uuid | 否 | 同 schema 外键指向 `finance.receipts` |
| business_date | date | 否 | 等于到款日期 |
| accounting_period_id | uuid | 否 | |
| deferred_from_period_id | uuid | 是 | |
| original_amount | numeric(18,2) | 否 | 挂账金额，大于 0 |
| settled_amount | numeric(18,2) | 否 | |
| open_amount | numeric(18,2) | 否 | |

守恒 CHECK 三条同构，直接支撑 PRD 第 6.11.3 的两条校验。

##### 3.2.6 finance.advance_payment_entries（预付台账条目）

与 3.2.5 对称，差异列为 `supplier_id`、`purchase_order_id`、`payment_plan_line_id`、`payment_id`。

##### 3.2.7 finance.unbilled_ar_entries（应收账款未开票过渡科目子账，仅追加）

| 列 | 类型 | 可空 | 说明 |
|---|---|---|---|
| customer_id | uuid | 否 | |
| contract_id | uuid | 是 | |
| sales_order_id | uuid | 是 | |
| direction | text | 否 | `ck_unbilled_ar_entries_direction` 取值 DEBIT、CREDIT |
| source_event | text | 否 | `ck_unbilled_ar_entries_source_event` 取值 DELIVERY_CONFIRMED、SALES_INVOICE_ISSUED、SALES_INVOICE_REVERSED、SALES_RETURN |
| source_doc_type | text | 否 | |
| source_doc_id | uuid | 否 | |
| business_date | date | 否 | |
| accounting_period_id | uuid | 否 | |
| deferred_from_period_id | uuid | 是 | |
| net_amount | numeric(18,2) | 否 | 大于 0，方向由 `direction` 表达 |
| voucher_id | uuid | 否 | |
| reverses_id | uuid | 是 | |

净额双向口径由视图 `finance.v_unbilled_ar_net` 表达：按法人、会计期间取 `sum(case direction when 'DEBIT' then net_amount else -net_amount end)`，正数为已交付未开票、负数为已开票未交付，与规格第 17.3 章一致。DEBIT 由交付确认与红字冲销登记产生，CREDIT 由开具登记与销售退货产生。

##### 3.2.8 finance.overbilling_entries（待处理超量开票挂账）

| 列 | 类型 | 可空 | 说明 |
|---|---|---|---|
| supplier_id | uuid | 否 | |
| purchase_order_id | uuid | 否 | 逻辑引用 procure |
| purchase_invoice_id | uuid | 否 | 逻辑引用 procure |
| material_id | uuid | 否 | 逻辑引用 mdm |
| warehouse_id | uuid | 是 | 逻辑引用 inventory，路径一匹配时回填 |
| overbilled_quantity | numeric(18,6) | 否 | 大于 0 |
| unit_price | numeric(18,6) | 否 | 已登记发票的不含税单价 |
| original_amount | numeric(18,2) | 否 | 挂账不含税金额 |
| settled_amount | numeric(18,2) | 否 | |
| open_amount | numeric(18,2) | 否 | |
| settled_quantity | numeric(18,6) | 否 | |
| open_quantity | numeric(18,6) | 否 | |
| status | text | 否 | `ck_overbilling_entries_status` 取值 OPEN、PARTIALLY_SETTLED、SETTLED |
| business_date | date | 否 | 等于采购发票登记日期 |
| accounting_period_id | uuid | 否 | |
| deferred_from_period_id | uuid | 是 | |

本表带 `row_version`，因为余额可更新。守恒 CHECK 四条：金额两条与数量两条，形式同 3.2.3。

##### 3.2.9 finance.overbilling_settlements（超量开票结清记录，仅追加）

列为 `overbilling_entry_id`（同 schema 外键）、`settlement_path`（`ck_overbilling_settlements_path` 取值 PATH_ONE_RECEIPT_MATCH、PATH_TWO_RED_INVOICE、PATH_THREE_WRITE_OFF）、`settled_quantity numeric(18,6)`、`settled_amount numeric(18,2)`、`source_doc_type`、`source_doc_id`、`business_date`、`accounting_period_id`、`deferred_from_period_id`、`voucher_id`、`reauth_ref`、`approval_ref`、`reverses_id`，加公共列。

路径三的 `reauth_ref` 与 `approval_ref` 非空，对应规格第 17.2 章第十三类分支“转当期主营业务成本的路径按第 12.1 章财务过账类高风险操作完成重新认证”。

##### 3.2.10 finance.receipts（到款单）

| 列 | 类型 | 可空 | 说明 |
|---|---|---|---|
| doc_no | text | 否 | 类型码 RCPT |
| status | text | 否 | `ck_receipts_status` 取值 DRAFT、REGISTERED、CANCELLED、REVERSED |
| customer_id | uuid | 否 | |
| receipt_date | date | 否 | |
| posting_date | date | 否 | 取 `receipt_date` |
| accounting_period_id | uuid | 是 | 状态为 REGISTERED 后非空 |
| deferred_from_period_id | uuid | 是 | |
| receipt_amount | numeric(18,2) | 否 | 大于 0 |
| settled_total | numeric(18,2) | 否 | 默认 0 |
| advance_amount | numeric(18,2) | 否 | 默认 0 |
| cash_account_id | uuid | 否 | 同 schema 外键指向 `finance.cash_accounts` |
| is_manual_settlement_order | boolean | 否 | 默认 false，人工指定核销顺序时为 true |
| refunded_amount | numeric(18,2) | 否 | 默认 0，供第 4.8 节可退上限使用 |
| voucher_id | uuid | 是 | |
| reversed_by_id | uuid | 是 | 指向冲正单 |
| remark | text | 是 | |

`ck_receipts_amount_identity` 表达 `receipt_amount = settled_total + advance_amount`，对应 PRD 第 6.13.3 的到款单勾稽。

##### 3.2.11 finance.payments（付款登记单）

结构与 3.2.10 对称，差异列为 `supplier_id`、`payment_request_id`（逻辑引用 procure 的付款申请单）、`payment_date`、`payment_amount`、`prepaid_amount`、`reauth_ref`、`approval_ref`。`ck_payments_amount_identity` 表达 `payment_amount = settled_total + prepaid_amount`。

##### 3.2.12 finance.refunds（客户退款单与供应商返款单）

| 列 | 类型 | 可空 | 说明 |
|---|---|---|---|
| doc_no | text | 否 | 类型码 RFND |
| status | text | 否 | `ck_refunds_status` 取值 DRAFT、REGISTERED、CANCELLED、REVERSED |
| refund_type | text | 否 | `ck_refunds_type` 取值 CUSTOMER_REFUND、SUPPLIER_REFUND |
| party_id | uuid | 否 | 客户或供应商 |
| return_doc_type | text | 否 | `ck_refunds_return_doc_type` 取值 SALES_RETURN、PURCHASE_RETURN |
| return_doc_id | uuid | 否 | 逻辑引用，必填，对应 PRD 第 6.12.3 |
| invoice_reversal_id | uuid | 是 | 退货部分已开票时必填 |
| register_date | date | 否 | |
| posting_date | date | 否 | |
| accounting_period_id | uuid | 是 | |
| deferred_from_period_id | uuid | 是 | |
| refund_amount | numeric(18,2) | 否 | 大于 0 |
| cash_account_id | uuid | 否 | 同 schema 外键 |
| reason | text | 否 | 长度上限 2000 |
| voucher_id | uuid | 是 | |
| reversed_by_id | uuid | 是 | |
| reauth_ref | uuid | 是 | |
| approval_ref | uuid | 是 | |

关联原款项由 `finance.refund_source_payment_links` 承载，列为 `refund_id`、`source_doc_type`（RECEIPT 或 PAYMENT）、`source_doc_id`、`linked_amount`，加公共列，支持一笔退款关联多笔原款项。

##### 3.2.13 finance.cash_document_reversals（资金单据冲正登记单）

本表是本阶段为闭合 F-14 与 U-D-02 新增的单据。列为 `doc_no`（类型码 CDRV）、`status`（`ck_cash_document_reversals_status` 取值固定为 REGISTERED，理由同第 3.1.4 节）、`source_doc_type`（RECEIPT、PAYMENT、REFUND）、`source_doc_id`、`register_date`、`posting_date`、`accounting_period_id`、`deferred_from_period_id`、`reversed_amount numeric(18,2)`、`reason text`、`voucher_id`、`reauth_ref`、`approval_ref`，加公共列。唯一约束 `ux_cash_document_reversals_legal_entity_id_source_doc_id` 保证一张资金单据只能被冲正一次。

##### 3.2.14 四张核销关系表（仅追加）

| 表 | 语义 | 专有列 |
|---|---|---|
| finance.receivable_settlement_links | 应收明细条目被核销 | `receivable_entry_id`（同 schema 外键）、`source_doc_type`（RECEIPT、ADVANCE_RECEIPT、CUSTOMER_REFUND、CASH_DOC_REVERSAL）、`source_doc_id`、`settled_amount numeric(18,2)`、`settled_at timestamptz`、`origin`（MANUAL、AUTO_ADVANCE、REFUND、REVERSAL）、`accounting_period_id`、`business_date`、`reverses_id` |
| finance.payable_settlement_links | 应付明细条目被核销 | 同构，`payable_entry_id`，`source_doc_type` 取值 PAYMENT、ADVANCE_PAYMENT、SUPPLIER_REFUND、CASH_DOC_REVERSAL |
| finance.advance_receipt_settlement_links | 预收条目被核销或被退款 | `advance_receipt_entry_id`、`target_type`（RECEIVABLE_ENTRY、CUSTOMER_REFUND）、`target_id`、`settled_amount`、`settled_at`、`accounting_period_id`、`business_date`、`reverses_id` |
| finance.advance_payment_settlement_links | 预付条目被核销或被返款 | 同构 |

`settled_amount` 一律为正数，冲回由追加 `reverses_id` 非空的反向行表达，反向行的 `settled_amount` 同样为正数并在聚合时按 `reverses_id` 抵消。理由是仅追加表不允许更新，也不允许负金额混入账龄基数。

##### 3.2.15 finance.cash_ledger_entries（资金腿明细，仅追加）

| 列 | 类型 | 可空 | 说明 |
|---|---|---|---|
| cash_account_id | uuid | 否 | 同 schema 外键 |
| direction | text | 否 | `ck_cash_ledger_entries_direction` 取值 IN、OUT |
| amount | numeric(18,2) | 否 | 大于 0 |
| source_doc_type | text | 否 | `ck_cash_ledger_entries_source_type` 取值 RECEIPT、PAYMENT、REFUND、CASH_DOC_REVERSAL |
| source_doc_id | uuid | 否 | |
| business_date | date | 否 | |
| accounting_period_id | uuid | 否 | |
| deferred_from_period_id | uuid | 是 | |
| voucher_id | uuid | 否 | |
| reverses_id | uuid | 是 | |

PRD 第 6.1.4 要求系统不提供新增资金流水入口，本表在 API 层不暴露任何写端点，只由四个用例经仓储写入，静态检查见第 8.5 节。

##### 3.2.16 附件关联表

`finance.receipt_attachments`、`finance.payment_attachments`、`finance.refund_attachments`、`finance.cash_document_reversal_attachments`。

#### 3.3 视图

十个勾稽项对应十个对账视图，本阶段建其中八个的完整实现，另两个只建外壳。另有四个业务查询视图，合计 14 个。

| 视图 | 子账侧取数 | 归属 |
|---|---|---|
| finance.v_recon_receivable | `sum(open_amount)` from `finance.receivable_entries` | 本阶段完整实现 |
| finance.v_recon_payable | `sum(open_amount)` from `finance.payable_entries` | 本阶段完整实现 |
| finance.v_recon_advance_receipt | `sum(open_amount)` from `finance.advance_receipt_entries` | 本阶段完整实现 |
| finance.v_recon_advance_payment | `sum(open_amount)` from `finance.advance_payment_entries` | 本阶段完整实现 |
| finance.v_recon_unbilled_ar | `finance.v_unbilled_ar_net` | 本阶段完整实现 |
| finance.v_recon_overbilling | `sum(open_amount)` from `finance.overbilling_entries` | 本阶段完整实现 |
| finance.v_recon_cash_bank | `opening_balance` 加 `finance.cash_ledger_entries` 的方向净额，限 `account_type` 为 BANK | 本阶段完整实现 |
| finance.v_recon_cash_on_hand | 同上，限 `account_type` 为 CASH | 本阶段完整实现 |
| finance.v_recon_inventory | 存货金额账 | 外壳，子账侧由库存阶段提供契约查询 |
| finance.v_recon_grni | 已收货未收票暂估 | 外壳，子账侧由采购阶段提供契约查询 |

另有四个业务查询视图：`finance.v_unbilled_ar_net`、`finance.v_receivable_aging`、`finance.v_payable_aging`、`finance.v_cash_account_period_balance`。账龄视图不做物化，理由是共享基线第 3.2 节禁用物化视图。

全部视图不带 `SECURITY DEFINER`，因此继承调用连接的 RLS 会话变量。

#### 3.4 RLS 策略

上述 34 张表全部带 `legal_entity_id`，全部按共享基线第 3.8 节的统一模板生成策略，模板由迁移生成器产出，本阶段不写变体。策略名一律 `rls_<table>_le`。

两个配置字典表 `invoice.tax_rate_options` 与 `finance.aging_bucket_definitions` 同样带法人列并建策略，不列入基线第 3.8 节的四类免建策略表。

#### 3.5 索引

每张表的基线三条索引按共享基线第 3.10 节自动生成。以下为本阶段追加的查询索引，逐条给出被支撑的查询。

| 索引 | 表 | 支撑的查询 |
|---|---|---|
| ix_receivable_entries_legal_entity_id_customer_id_due_date | finance.receivable_entries | 核销候选检索与默认核销顺序，规格第 5.2 章核销顺序规则块 |
| ix_receivable_entries_legal_entity_id_accounting_period_id | finance.receivable_entries | 对账视图与账龄按期间取数 |
| ix_receivable_entries_legal_entity_id_sales_invoice_id | finance.receivable_entries | 红冲登记时按发票定位应收条目 |
| ix_payable_entries_legal_entity_id_supplier_id_due_date | finance.payable_entries | 同上，应付侧 |
| ix_payable_entries_legal_entity_id_accounting_period_id | finance.payable_entries | 同上 |
| ix_payable_entries_legal_entity_id_purchase_invoice_id | finance.payable_entries | 进项红冲与付款申请占用校验 |
| ix_advance_receipt_entries_legal_entity_id_contract_id | finance.advance_receipt_entries | 开票时按同一合同自动核销预收 |
| ix_advance_payment_entries_legal_entity_id_contract_id | finance.advance_payment_entries | 采购发票登记时自动核销预付 |
| ix_unbilled_ar_entries_legal_entity_id_accounting_period_id | finance.unbilled_ar_entries | 过渡科目净额对账 |
| ix_overbilling_entries_legal_entity_id_purchase_order_id | finance.overbilling_entries | 路径一按采购订单反向匹配 |
| ix_cash_ledger_entries_legal_entity_id_cash_account_id_business_date | finance.cash_ledger_entries | 资金腿明细视图与资金勾稽 |
| ix_receivable_settlement_links_legal_entity_id_receivable_entry_id | finance.receivable_settlement_links | 核销关系双向追溯 |
| ix_receivable_settlement_links_legal_entity_id_source_doc_id | finance.receivable_settlement_links | 从到款单反查核销明细 |
| ix_payable_settlement_links_legal_entity_id_payable_entry_id | finance.payable_settlement_links | 同上 |
| ix_payable_settlement_links_legal_entity_id_source_doc_id | finance.payable_settlement_links | 同上 |
| ix_sales_invoices_legal_entity_id_invoice_application_id | invoice.sales_invoices | 剩余可开比例回滚与申请单勾稽 |
| ix_sales_invoices_legal_entity_id_customer_id_issue_date | invoice.sales_invoices | 销项发票台账列表 |
| ix_invoice_reversals_legal_entity_id_source_invoice_id | invoice.invoice_reversals | 冲销互斥判定 |

全部索引在迁移中以 `CREATE INDEX CONCURRENTLY` 创建，迁移会话按共享基线第 3.9 节固定 `lock_timeout` 与 `statement_timeout`。

#### 3.6 迁移编号与顺序

`db/migrations/order.toml` 已声明 invoice 先于 finance，本阶段不改该顺序。

invoice 目录：

1. V202611030900__invoice_create_tax_rate_options.sql
2. V202611030905__invoice_create_invoice_applications.sql
3. V202611030910__invoice_create_invoice_application_link_tables.sql
4. V202611030915__invoice_create_sales_invoices.sql
5. V202611030920__invoice_create_invoice_reversals.sql
6. V202611030925__invoice_create_invoice_receipt_plan_links.sql
7. V202611030930__invoice_create_invoice_import_batches.sql
8. V202611030935__invoice_create_attachment_link_tables.sql
9. V202611030940__invoice_enable_row_level_security.sql
10. V202611030945__invoice_create_indexes.sql
11. V202611030950__invoice_backfill_seed_tax_rate_options.sql

finance 目录：

1. V202611031000__finance_create_aging_bucket_definitions.sql
2. V202611031005__finance_create_cash_accounts.sql
3. V202611031010__finance_create_receivable_entries.sql
4. V202611031015__finance_create_payable_entries.sql
5. V202611031020__finance_create_advance_receipt_entries.sql
6. V202611031025__finance_create_advance_payment_entries.sql
7. V202611031030__finance_create_unbilled_ar_entries.sql
8. V202611031035__finance_create_overbilling_entries.sql
9. V202611031040__finance_create_overbilling_settlements.sql
10. V202611031045__finance_create_receipts.sql
11. V202611031050__finance_create_payments.sql
12. V202611031055__finance_create_refunds.sql
13. V202611031060__finance_create_refund_source_payment_links.sql
14. V202611031065__finance_create_cash_document_reversals.sql
15. V202611031070__finance_create_settlement_link_tables.sql
16. V202611031075__finance_create_cash_ledger_entries.sql
17. V202611031080__finance_create_attachment_link_tables.sql
18. V202611031085__finance_enable_row_level_security.sql
19. V202611031090__finance_create_indexes.sql
20. V202611031095__finance_create_reconciliation_views.sql
21. V202611031100__finance_backfill_seed_aging_buckets.sql

每个文件头带 `-- rollback:` 段。建表类文件的回退为对应 `drop table`；两个 backfill 文件的回退为按 `code` 删除出厂预置行；`enable_row_level_security` 与 `create_indexes` 的回退为逐条 `drop policy` 与 `drop index`。本阶段没有改列类型与收紧非空的迁移，因此全部迁移可在线执行。

---

### 4. 领域模型与关键算法

#### 4.1 核心类型

`ep-domain-invoice` 的聚合与值对象：

- `InvoiceApplication`：聚合根，持有 `IssueRatio issue_ratio`、`IssueRatio remaining_ratio`、`Money contract_amount`、`InvoiceApplicationStatus`。
- `SalesInvoice`：聚合根，持有 `InvoiceNumber`、`InvoiceKind`、`TaxLine { net: Money, rate: Rate, tax: Money, gross: Money }`、`SalesInvoiceStatus`。
- `InvoiceReversal`：聚合根，持有 `ReversalDirection`、`ReversalType`、可选红字金额。
- `TaxLine::validate(tolerance)`：勾稽校验，是唯一的税额校验入口。

`ep-domain-finance` 的聚合与值对象：

- `ReceivableEntry` 与 `PayableEntry`：共用泛型聚合 `OpenItem<Side>`，持有 `original_amount`、`settled_amount`、`open_amount`、`due_date`、`business_date`。
- `AdvanceEntry<Side>`：预收预付共用。
- `Receipt`、`Payment`、`Refund`、`CashDocumentReversal`：四个资金类单据聚合。
- `CashAccount`：档案聚合。
- `OverbillingEntry`：超量开票挂账聚合。
- `SettlementPlan`：核销分配结果，是一个不可变值对象，含 `lines: Vec<SettlementLine>`、`settled_total`、`residual`。
- `AgingBucketSet` 与 `AgingSnapshot`。

全部聚合方法返回新实例，不做原地修改，符合项目编码规范的不可变要求。

#### 4.2 状态机

##### 发票申请单

状态取值 DRAFT、PENDING_APPROVAL、APPROVED、PARTIALLY_ISSUED、FULLY_ISSUED、CANCELLED。

| 起点 | 终点 | 守卫条件 |
|---|---|---|
| DRAFT | PENDING_APPROVAL | 通过 PRD 第 6.3.3 三项校验；合同状态为已生效；累计开票比例加本次不超过 1 加容差 |
| DRAFT | CANCELLED | 无已开具发票 |
| PENDING_APPROVAL | APPROVED | 审批链全节点通过且审批人不等于申请人 |
| PENDING_APPROVAL | DRAFT | 审批驳回，或申请人在首节点处理前撤回 |
| APPROVED | PARTIALLY_ISSUED | 一次开具登记后 `remaining_ratio > 0` |
| APPROVED | FULLY_ISSUED | 一次开具登记后 `remaining_ratio = 0` |
| APPROVED | CANCELLED | 该申请单下无任何开具登记 |
| PARTIALLY_ISSUED | FULLY_ISSUED | 后续开具使 `remaining_ratio = 0` |
| PARTIALLY_ISSUED | APPROVED | 该申请单下全部已开具发票被作废或红冲，`remaining_ratio` 回增至 `issue_ratio` |
| FULLY_ISSUED | PARTIALLY_ISSUED | 部分已开具发票被作废或红冲 |
| FULLY_ISSUED | APPROVED | 全部已开具发票被作废或红冲 |
| CANCELLED | 无 | 终态 |

守卫条件“该申请单下无任何开具登记”按 `invoice.sales_invoices` 中该申请单的行数为零判定，含已作废与已红冲的行，因此曾经开过票的申请单永远不能取消，对应 PRD 第 6.3.6 最后一行。

##### 销项发票

ISSUED 到 VOIDED、ISSUED 到 RED_REVERSED，两者互斥，各自只允许一次，VOIDED 与 RED_REVERSED 为终态。守卫为：`invoice.invoice_reversals` 中不存在以该发票为 `source_invoice_id` 的行，由唯一约束兜底。

##### 到款单

DRAFT 到 REGISTERED（通过 PRD 第 6.7.2 与 6.7.3 校验且凭证生成成功）、DRAFT 到 CANCELLED、REGISTERED 到 REVERSED（经资金单据冲正登记）。REVERSED 与 CANCELLED 为终态。守卫为：REGISTERED 到 REVERSED 要求该到款单未被任何客户退款单引用，或引用它的退款单已先行冲正。

##### 付款登记单

DRAFT 到 PENDING_REAUTH_APPROVAL、PENDING_REAUTH_APPROVAL 到 REGISTERED（重新认证通过且审批链通过且凭证生成成功）、PENDING_REAUTH_APPROVAL 到 DRAFT（驳回或撤回）、DRAFT 到 CANCELLED、REGISTERED 到 REVERSED。守卫为：转 REGISTERED 前重跑该付款申请单的累计已登记金额上限校验，避免审批期间被其他登记占满。

##### 退款与返款单

DRAFT 到 REGISTERED，或 DRAFT 到 PENDING_REAUTH_APPROVAL 再到 REGISTERED（按配置项决定是否经重新认证），DRAFT 到 CANCELLED，REGISTERED 到 REVERSED。

##### 超量开票挂账

OPEN 到 PARTIALLY_SETTLED 到 SETTLED，三条结清路径任一条都推进该状态机；SETTLED 为终态但可由路径三的成本冲回退回 PARTIALLY_SETTLED，对应规格第 5.2 章“已按路径三转成本的部分，需先经审批冲回原成本再按路径一入账”。冲回由一条 `reverses_id` 非空的 `finance.overbilling_settlements` 行表达。

##### 资金账户

`is_active` 在 true 与 false 之间双向流转，停用只影响新单据的下拉可选范围，不影响历史引用，对应 PRD 第 6.2.3。

#### 4.3 核销分配算法

输入：`side`（AR 或 AP）、`party_id`、`legal_entity_id`、`amount`、可选的人工指定行列表。输出：`SettlementPlan`。

步骤：

1. 候选集取数。按 `side` 从 `finance.receivable_entries` 或 `finance.payable_entries` 取该法人该往来方 `open_amount > 0` 且 `is_reversed = false` 的条目，排序按规格第 5.2 章核销顺序规则块，即 `due_date asc, doc_no asc`。跨往来方核销按配置项禁止，配置为允许时候选集放宽到该法人全部往来方，并强制 `is_manual_settlement_order = true`。
2. 若有人工指定行，候选集改为人工指定的顺序与条目集合，`is_manual_settlement_order` 置 true，该事实按 PRD 第 6.14.4 写入审计。
3. 逐条分配。`residual` 初值为 `amount`；对候选集每一条取 `line = min(residual, entry.open_amount)`；`line` 为零则跳过；`residual` 减 `line`；`residual` 为零则停止。
4. 剩余部分。循环结束后 `residual` 大于零的，作为转预收或转预付金额返回。
5. 行数上限。`lines.len()` 超过 `EP__FINANCE__SETTLEMENT__MAX_LINES` 时返回 `VALIDATION`，默认 200，与共享基线第 5.1 节的批量上限一致。

边界条件与对应错误：

- `amount` 小于等于零：`FINANCE.RECEIPT.AMOUNT_NOT_POSITIVE`。
- 候选集为空：合法，全额转预收或转预付。
- 人工指定某行金额超过该条目 `open_amount`：`FINANCE.SETTLEMENT.LINE_EXCEEDS_OPEN_AMOUNT`，`details` 定位到 `lines[i]` 并回带该条目 `open_amount`。
- 人工指定行合计超过 `amount`：`FINANCE.SETTLEMENT.TOTAL_EXCEEDS_DOCUMENT_AMOUNT`，`details` 回带两个数值与差额。
- 人工指定的条目不属该法人或不属该往来方：按共享基线第 5.5 节返回 404 与 `PLATFORM.AUTHZ.NOT_FOUND_OR_DENIED`。

金额一律以 `Decimal` 全精度参与 `min` 比较，写库前一次性 `round(2)`。由于全部输入已是 2 位小数，`min` 不引入新精度，因此不产生尾差。

#### 4.4 剩余可开比例的推进与回滚

`remaining_ratio` 初值等于 `issue_ratio`。开具登记扣减 `issued_ratio`，作废或红冲回增被冲销发票的 `issued_ratio`。

守卫为 `issued_ratio <= remaining_ratio + tolerance`，`tolerance` 取 `EP__INVOICE__RATIO__TOLERANCE`。扣减后若 `remaining_ratio` 的绝对值小于 `tolerance` 则归零，避免尾数使状态卡在 PARTIALLY_ISSUED。

回滚守卫：回增后 `remaining_ratio <= issue_ratio + tolerance`，超出即 `BUSINESS_CONFLICT` 与 `INVOICE.INVOICE_APPLICATION.RATIO_ROLLBACK_OVERFLOW`，按规格第 15.2 章进入死信。

累计开票比例校验的取数范围只含有效状态：`invoice.invoice_applications` 中状态不为 CANCELLED 的申请单，加 `invoice.sales_invoices` 中状态为 ISSUED 的发票。已作废与已红冲的发票不参与累计，对应 PRD 第 6.3.3 与规格第 8 章第 7 步。

#### 4.5 账龄分桶

输入：条目的 `due_date`、`open_amount`、评估基准日（默认为服务器自然日，按共享基线第 3.4 节取 `(now() AT TIME ZONE 'Asia/Shanghai')::date`）、`AgingBucketSet`。

`overdue_days = 评估基准日 - due_date`，小于零时归入第一档。逐档判定 `from_days <= overdue_days` 且（`to_days` 为空或 `overdue_days <= to_days`）。基数一律为 `open_amount`，不使用 `original_amount`，对应 PRD 第 6.9.3。

账龄不依赖 `accounting_period_id`，因此顺延入账不改变账龄，对应规格第 5.2 章子账与凭证共用同一期间归属条款的最后一句。这一点在领域属性测试中作为不变量断言。

#### 4.6 超量开票三条结清路径

三条路径共用同一个余额推进函数 `OverbillingEntry::settle(quantity, amount, path) -> Result<(OverbillingEntry, OverbillingSettlement)>`，守卫为 `quantity <= open_quantity` 且 `amount <= open_amount`。

路径一由收货用例经契约 `OverbillingMatchPort::match_on_receipt` 触发。入参为采购订单、物料、仓库、本次收货数量；返回本次可反向匹配的数量与单价。库存模块按返回的单价与数量同源写数量账与金额账，本阶段按同一金额记 `finance.overbilling_settlements` 并由 ledger 端口生成借存货、贷待处理超量开票科目两腿。匹配数量以该采购订单已挂账的 `open_quantity` 为上限，对应规格第 5.2 章路径一。

路径二由进项方向的冲销登记触发，`invoice.invoice_reversals.overbilling_entry_id` 非空时把本次红字发票不含税金额结清到对应挂账。

路径三由 `POST /api/v1/finance/overbilling-entries/{id}/actions/settle-by-write-off` 触发，属规格第 12.1 章财务过账类高风险操作，需重新认证与审批。

路径三之后的收货：先由 `POST .../actions/reverse-write-off` 经审批冲回原成本，产生 `reverses_id` 非空的结清记录并把挂账退回 PARTIALLY_SETTLED，再走路径一。该顺序在领域层由守卫强制：`open_quantity` 为零而收货侧仍请求匹配时返回 `FINANCE.OVERBILLING_ENTRY.WRITTEN_OFF_REQUIRES_REVERSAL`。

#### 4.7 资金单据冲正

冲正是本阶段为 F-14 与 U-D-02 新增的路径，语义是按同一事件生成一张反向凭证并把子账逐行反向，不做反结账，符合规格第 5.2 章“已过账凭证只以红字冲销或更正凭证追加更正”。

步骤：

1. 锁定原单据行与其全部核销关系行。
2. 逐条核销关系追加一条 `reverses_id` 指向原行的反向行，同时把对应台账条目的 `settled_amount` 减去该行金额，`open_amount` 相应回增。三条守恒 CHECK 在此处兜底。
3. 原单据产生的预收或预付条目：若该条目已被后续核销，冲正被拒绝并返回 `FINANCE.CASH_DOCUMENT_REVERSAL.ADVANCE_ALREADY_CONSUMED`，要求先冲正后续单据；未被核销的按 `reverses_id` 追加反向条目并把 `original_amount` 视为已冲回。
4. 追加一条方向相反的资金腿明细。
5. 经 ledger 端口生成红字凭证，会计期间按冲正单的记账日期重新解析，允许与原单据落在不同期间，这与规格第 5.2 章顺延不回迁一致。
6. 原单据置 REVERSED。

冲正单不可再被冲正，由唯一约束保证。

#### 4.8 可退金额上限

`RefundCapCalculator::cap(refund) -> Money`，取两个上界的较小值。

上界一，来自原款项：对 `finance.refund_source_payment_links` 中每一笔原到款单或原付款登记单，可退余额等于该单金额减已退金额，求和。

上界二，来自退货：关联退货单据的退货金额；退货部分已开票的，取关联红字冲销单的红字价税合计；未开票的部分取退货单该部分金额，由 `ep-contract-sales` 与 `ep-contract-procure` 的退货查询提供。

超出上限返回 `FINANCE.REFUND.AMOUNT_EXCEEDS_CAP`，`details` 回带两个数值。登记成功后回写各原款项单的 `refunded_amount`。

该规则为临时取值，见第 0.4 节 F-15。

#### 4.9 应收账款未开票过渡科目的双向净额

开具登记一律追加一条 CREDIT 方向、金额为 `net_amount` 的 `finance.unbilled_ar_entries`，不做与交付确认的逐笔匹配。理由是规格第 5.2 章开票事件对该科目只有一条贷记腿，没有匹配条件；开票先于交付确认时形成贷方余额，交付确认时由交付确认事件借记冲回，两者的抵消是科目层面的净额而不是条目层面的配对。

红字冲销与作废登记按销项方向追加一条 DEBIT 方向、金额等于原发票 `net_amount` 的条目，对应事件-分录表“恢复其余额”。

因此该科目的子账侧净额恒等于 `sum(DEBIT) - sum(CREDIT)`，正数为已交付未开票、负数为已开票未交付，与规格第 17.3 章的净额双向口径逐字对应，且不设关账归零要求。

#### 4.10 会计期间归属

本阶段不实现期间解析，只调用 `ep-contract-ledger::AccountingPeriodResolver::resolve(legal_entity_id, posting_date, tx) -> ResolvedPeriod { period_id, deferred_from_period_id }`。该端口在业务事务内调用一次，结果同时写入凭证与全部子账条目，包括台账条目、核销关系行、资金腿明细与超量开票记录。

记账日期的取值与校验在本阶段：默认取登记时点服务器自然日；允许早于该日并按 PRD 第 6.1.4 提示为补记并写审计；晚于该日一律 `VALIDATION` 并定位字段。

提交响应固定回带 `accounting_period`（编码与名称）与 `is_deferred`，供界面按 PRD 第 6.1.4 显式标注，缺失即视为实现缺陷。

---

### 5. API 契约

全部端点遵守共享基线第 5 节：路径前缀 `/api/v1`，字段 snake_case，封套固定，写请求必带 `Idempotency-Key`，请求头集合固定，分页与排序按第 5.3 节。以下只列出各端点的专有部分。

#### 5.1 发票申请

| 方法与路径 | 请求 | 响应 | 主要错误码 | 权限 |
|---|---|---|---|---|
| POST /api/v1/invoice/invoice-applications | `customer_id`、`contract_id`、`sales_order_ids[]`、`receipt_plan_line_ids[]`、`application_date`、`issue_content`、`issue_ratio`、`expected_receipt_date`、`remark` | 申请单视图，含 `doc_no`、`application_amount`、`remaining_ratio` | INVOICE.INVOICE_APPLICATION.CONTRACT_NOT_EFFECTIVE、INVOICE.INVOICE_APPLICATION.CUMULATIVE_RATIO_EXCEEDED、INVOICE.INVOICE_APPLICATION.EXPECTED_DATE_BEFORE_APPLICATION_DATE | invoice.invoice_application:create |
| GET /api/v1/invoice/invoice-applications | 过滤 `customer_id`、`contract_id`、`status`、`application_date`；排序白名单 `application_date`、`created_at`、`doc_no` | 分页列表 | | invoice.invoice_application:read |
| GET /api/v1/invoice/invoice-applications/{id} | | 详情，含已开具发票列表与剩余可开比例 | | 同上 |
| PATCH /api/v1/invoice/invoice-applications/{id} | 仅 DRAFT 可改；带 `row_version` | 申请单视图 | PLATFORM.CONCURRENCY.STALE_VERSION | invoice.invoice_application:update |
| POST /api/v1/invoice/invoice-applications/{id}/actions/submit-for-approval | `row_version` | 申请单视图 | INVOICE.INVOICE_APPLICATION.INVALID_TRANSITION | invoice.invoice_application:submit |
| POST /api/v1/invoice/invoice-applications/{id}/actions/withdraw | `row_version` | 申请单视图 | INVOICE.INVOICE_APPLICATION.APPROVAL_ALREADY_STARTED | invoice.invoice_application:submit |
| POST /api/v1/invoice/invoice-applications/{id}/actions/cancel | `row_version`、`reason` | 申请单视图 | INVOICE.INVOICE_APPLICATION.ISSUED_INVOICE_EXISTS | invoice.invoice_application:cancel |

审批放行不在本阶段暴露端点，走 platform_flow 的统一审批端点，本阶段只注册审批回调。

#### 5.2 销项发票开具与冲销

| 方法与路径 | 请求 | 响应 | 主要错误码 | 权限 |
|---|---|---|---|---|
| POST /api/v1/invoice/sales-invoices | `invoice_application_id`、`invoice_kind`、`invoice_no`、`invoice_code`、`issue_date`、`posting_date`、`issue_content`、`issued_ratio`、`net_amount`、`tax_rate`、`tax_amount`、`gross_amount`、`attachment_object_ids[]`；必带 `X-Reauth-Token` | 发票视图，含 `voucher_id`、`accounting_period`、`is_deferred`、`receivable_entry_id` | INVOICE.SALES_INVOICE.RATIO_EXCEEDS_REMAINING、INVOICE.SALES_INVOICE.GROSS_AMOUNT_MISMATCH、INVOICE.SALES_INVOICE.TAX_AMOUNT_OUT_OF_TOLERANCE、INVOICE.SALES_INVOICE.INVOICE_NO_DUPLICATED、INVOICE.SALES_INVOICE.ISSUE_DATE_IN_FUTURE、INVOICE.SALES_INVOICE.POSTING_DATE_IN_FUTURE、INVOICE.SALES_INVOICE.CODE_REQUIRED_FOR_PAPER、INVOICE.SALES_INVOICE.CODE_FORBIDDEN_FOR_DIGITAL、PLATFORM.AUTHZ.REAUTH_REQUIRED | invoice.sales_invoice:issue |
| GET /api/v1/invoice/sales-invoices 与 /{id} | 过滤 `customer_id`、`status`、`issue_date`、`accounting_period_id`、`invoice_no` | 分页列表与详情 | | invoice.sales_invoice:read |
| POST /api/v1/invoice/sales-invoices/actions/import-batch | `file_object_id`；必带 `X-Reauth-Token` | 后台任务回执 `{ task_id, batch_id }` | INVOICE.IMPORT_BATCH.ROW_LIMIT_EXCEEDED、INVOICE.IMPORT_BATCH.TEMPLATE_MISMATCH | invoice.sales_invoice:issue 加 invoice.sales_invoice:import |
| GET /api/v1/invoice/invoice-import-batches/{id} | | 批次详情，含逐行结果对象引用 | | invoice.sales_invoice:read |
| POST /api/v1/invoice/invoice-reversals | `direction`、`reversal_type`、`source_invoice_id`、`register_date`、`posting_date`、`red_invoice_no`、`red_net_amount`、`red_tax_rate`、`red_tax_amount`、`red_gross_amount`、`reason`、`overbilling_entry_id`、`attachment_object_ids[]`；按配置带 `X-Reauth-Token` | 冲销单视图，含 `voucher_id`、被回滚后的申请单状态与剩余可开比例 | INVOICE.INVOICE_REVERSAL.SOURCE_ALREADY_REVERSED、INVOICE.INVOICE_REVERSAL.TYPE_MUTUALLY_EXCLUSIVE、INVOICE.INVOICE_REVERSAL.RED_AMOUNT_MISMATCH、INVOICE.INVOICE_REVERSAL.SOURCE_INVOICE_NOT_REGISTERED、INVOICE.INVOICE_REVERSAL.RECEIPT_PLAN_ISSUED_AMOUNT_NEGATIVE | invoice.invoice_reversal:create |
| GET /api/v1/invoice/purchase-invoices 与 /{id} | 进项发票台账只读投影，取数经 `ep-contract-procure` 加本阶段应付条目 | 分页列表与详情，含应付明细条目与凭证追溯 | | invoice.purchase_invoice_ledger:read |

`POST /api/v1/invoice/sales-invoices` 是规格附录 A.1“应收发票生成”这一度量项的被测端点。

#### 5.3 资金账户

| 方法与路径 | 说明 |
|---|---|
| POST /api/v1/finance/cash-accounts | 建档，按配置进审批；`account_type` 与 `ledger_account_id` 的匹配校验，错误码 FINANCE.CASH_ACCOUNT.ACCOUNT_TYPE_LEDGER_MISMATCH |
| GET /api/v1/finance/cash-accounts 与 /{id} | 列表按 `code asc`；`bank_account_no` 返回脱敏后 4 位，具备 `finance.cash_account.bank_account_no.read_full` 字段级权限时详情返回完整值 |
| PATCH /api/v1/finance/cash-accounts/{id} | `has_cash_flow` 为 true 时拒绝修改 `legal_entity_id` 与 `ledger_account_id`，错误码 FINANCE.CASH_ACCOUNT.LEDGER_ACCOUNT_LOCKED |
| POST /api/v1/finance/cash-accounts/{id}/actions/deactivate 与 actions/activate | 停用与启用 |
| GET /api/v1/finance/cash-accounts/{id}/cash-ledger-entries | 资金腿明细视图，`meta` 额外返回 `opening_balance`、`period_in`、`period_out`、`closing_balance` 四个数值 |

#### 5.4 到款、付款、退款与冲正

| 方法与路径 | 请求要点 | 主要错误码 | 权限 |
|---|---|---|---|
| GET /api/v1/finance/settlement-proposals | 查询参数 `side`（AR 或 AP）、`party_id`、`amount`；返回按默认核销顺序预填的行与转预收或转预付金额 | FINANCE.SETTLEMENT.PARTY_REQUIRED | finance.receivable_entry:read 或 finance.payable_entry:read |
| POST /api/v1/finance/receipts | `customer_id`、`receipt_date`、`posting_date`、`receipt_amount`、`cash_account_id`、`settlement_lines[]`、`is_manual_settlement_order`、`attachment_object_ids[]`、`remark` | FINANCE.RECEIPT.AMOUNT_NOT_POSITIVE、FINANCE.RECEIPT.CASH_ACCOUNT_DEACTIVATED、FINANCE.RECEIPT.DATE_IN_FUTURE、FINANCE.SETTLEMENT.LINE_EXCEEDS_OPEN_AMOUNT、FINANCE.SETTLEMENT.TOTAL_EXCEEDS_DOCUMENT_AMOUNT | finance.receipt:create |
| POST /api/v1/finance/receipts/{id}/actions/cancel | 仅 DRAFT | FINANCE.RECEIPT.INVALID_TRANSITION | finance.receipt:cancel |
| GET /api/v1/finance/receipts 与 /{id} | 过滤 `customer_id`、`status`、`receipt_date`、`accounting_period_id` | | finance.receipt:read |
| POST /api/v1/finance/payments | `payment_request_id`、`supplier_id`、`payment_date`、`posting_date`、`payment_amount`、`cash_account_id`、`settlement_lines[]`、`remark`；必带 `X-Reauth-Token` | FINANCE.PAYMENT.EXCEEDS_REQUEST_AMOUNT、FINANCE.PAYMENT.REQUEST_NOT_APPROVED、PLATFORM.AUTHZ.REAUTH_REQUIRED，其余同到款 | finance.payment:create |
| POST /api/v1/finance/payments/{id}/actions/withdraw 与 actions/cancel | 撤回与取消 | | finance.payment:cancel |
| POST /api/v1/finance/refunds | `refund_type`、`party_id`、`return_doc_type`、`return_doc_id`、`invoice_reversal_id`、`source_payments[]`、`register_date`、`posting_date`、`refund_amount`、`cash_account_id`、`reason` | FINANCE.REFUND.RETURN_DOC_REQUIRED、FINANCE.REFUND.INVOICE_REVERSAL_REQUIRED、FINANCE.REFUND.AMOUNT_EXCEEDS_CAP、FINANCE.REFUND.CASH_ACCOUNT_DEACTIVATED | finance.refund:create |
| POST /api/v1/finance/cash-document-reversals | `source_doc_type`、`source_doc_id`、`register_date`、`posting_date`、`reason`；必带 `X-Reauth-Token` | FINANCE.CASH_DOCUMENT_REVERSAL.SOURCE_ALREADY_REVERSED、FINANCE.CASH_DOCUMENT_REVERSAL.ADVANCE_ALREADY_CONSUMED、FINANCE.CASH_DOCUMENT_REVERSAL.SOURCE_NOT_REGISTERED | finance.cash_document_reversal:create |

#### 5.5 台账、账龄与对账

| 方法与路径 | 说明 |
|---|---|
| GET /api/v1/finance/receivable-entries 与 /{id} | 应收台账；详情含九类信息与核销关系列表，核销关系标注 `origin` |
| GET /api/v1/finance/payable-entries 与 /{id} | 应付台账 |
| GET /api/v1/finance/advance-receipt-entries 与 /api/v1/finance/advance-payment-entries | 预收预付台账，只读，无写端点，对应 PRD 第 6.11.3 最后一段 |
| GET /api/v1/finance/receivable-agings 与 /api/v1/finance/payable-agings | 账龄汇总；查询参数 `group_by` 取 `customer`、`contract`、`sales_order`、`bucket`；下钻用 `filter[bucket_code]=eq:` 加 `expand=entries` |
| GET /api/v1/finance/unbilled-ar-entries | 已交付未开票只读查询视图，`meta` 返回净额与方向 |
| GET /api/v1/finance/overbilling-entries 与 /{id} | 待处理超量开票查询视图，`meta` 返回该法人该期间的挂账合计 |
| POST /api/v1/finance/overbilling-entries/{id}/actions/settle-by-write-off | 路径三，必带 `X-Reauth-Token` 并进审批 |
| POST /api/v1/finance/overbilling-entries/{id}/actions/reverse-write-off | 冲回路径三，必带 `X-Reauth-Token` 并进审批 |
| GET /api/v1/finance/reconciliations | 查询参数 `accounting_period_id` 与可选 `item`；返回十项的子账侧、总账侧与差额三列，其中存货与已收货未收票两项在其子账侧契约接入前只返回总账侧并标注未接入；差额非零时附差异事项引用；本端点不提供任何调整入口，对应 PRD 第 6.13.2 |
| GET /api/v1/finance/cash-ledger-entries | 全法人资金腿明细，按账户筛选 |

移动端按规格第 6.2 章矩阵只提供本节全部 GET 端点，POST 端点在移动端由前端隐藏，服务端不做端别拒绝，理由是端别限制属客户端能力矩阵而不是服务端授权。

#### 5.6 幂等语义

全部 POST 与 PATCH 按共享基线第 5.4 节执行。本阶段的补充约定：`request_hash` 的计算不包含 `X-Reauth-Token`，理由是重新认证凭证单次有效，重放时该头必然不同，若纳入哈希会把合法重放判成 `PAYLOAD_MISMATCH`。该约定需回写共享基线第 5.4 节。

批量导入的幂等作用域为逐行：每行的幂等键取批次幂等键加行号派生的 UUIDv5，因此重跑同一批次不产生重复发票。

#### 5.7 权限要求

对象类型注册 14 个：`invoice.invoice_application`、`invoice.sales_invoice`、`invoice.invoice_reversal`、`invoice.purchase_invoice_ledger`、`finance.cash_account`、`finance.receipt`、`finance.payment`、`finance.refund`、`finance.cash_document_reversal`、`finance.receivable_entry`、`finance.payable_entry`、`finance.advance_entry`、`finance.overbilling_entry`、`finance.reconciliation`。

动作注册 12 个：create、read、update、submit、cancel、issue、import、reverse、settle、write_off、export、approve。

字段级权限注册 1 个：`finance.cash_account.bank_account_no.read_full`。

判定顺序按共享基线第 11.3 节，即法人、对象、记录、字段与密级四级，显式拒绝优先。不可见记录一律 404 与 `PLATFORM.AUTHZ.NOT_FOUND_OR_DENIED`。

职责分离：发票申请的申请人不可自审；开票的发起人不可自审；付款登记的发起人不可自审；超量开票路径三的发起人不可自审。四项由 platform_authz 的审批授权判定承担，本阶段只声明该要求并在集成测试中验证。

#### 5.8 领域事件

11 个事件，命名按共享基线第 6.1 节的四段式，登记到 `docs/event-catalog.md` 后才实现。信封字段不增不减，`posting_date` 与 `accounting_period_id` 取本次解析结果，`security_level` 取单据取值，`data_scope_tags` 携带客户或供应商与合同两类标签。

| 事件类型 | 触发点 | payload 要点 | 消费方 |
|---|---|---|---|
| invoice.invoice_application.submitted.v1 | 申请提交进入审批 | 申请单 ID、合同、客户、开票比例、申请金额 | notify、reporting |
| invoice.invoice_application.approved.v1 | 审批链全通过 | 申请单 ID、剩余可开比例 | notify |
| invoice.sales_invoice.issued.v1 | 开具登记提交成功 | 发票 ID、申请单 ID、客户、合同、不含税金额、税额、价税合计、应收条目 ID、凭证 ID | notify、reporting、search、客户 360 |
| invoice.sales_invoice.reversed.v1 | 销项作废或红冲登记成功 | 冲销单 ID、原发票 ID、处理类型、红字金额、回滚后的剩余可开比例 | notify、reporting、search |
| invoice.purchase_invoice.reversed.v1 | 进项作废或红冲登记成功 | 冲销单 ID、原采购发票 ID、处理类型、红字金额、是否用于超量开票结清 | notify、reporting |
| invoice.invoice_import_batch.completed.v1 | 批量导入任务结束 | 批次 ID、总行数、成功行数、失败行数、结果对象引用 | notify |
| finance.receipt.registered.v1 | 到款登记成功 | 到款单 ID、客户、到款金额、核销合计、转预收金额、资金账户、凭证 ID | notify、reporting、客户 360 |
| finance.payment.registered.v1 | 付款登记成功 | 付款单 ID、供应商、付款金额、核销合计、转预付金额、付款申请单 ID、凭证 ID | notify、reporting、门户投影 |
| finance.refund.registered.v1 | 退款或返款登记成功 | 退款单 ID、类型、往来方、金额、关联退货单、关联冲销单、凭证 ID | notify、reporting |
| finance.cash_document.reversed.v1 | 资金单据冲正登记成功 | 冲正单 ID、原单据类型与 ID、冲正金额、凭证 ID | notify、reporting |
| finance.overbilling_entry.settled.v1 | 三条结清路径任一条完成 | 挂账 ID、路径、结清数量与金额、剩余余额、凭证 ID | notify、reporting |

上述事件的消费方均不做过账，见第 0.1 节。

#### 5.9 对外契约 trait

12 个 trait，定义在两个 contract crate 中，实现注册在 `apps/core-server/src/wiring.rs`。调用方按共享基线第 1.3 节只依赖 contract，不依赖 application。

| trait | 所在 crate | 调用方 | 语义 |
|---|---|---|---|
| PayableRegistrationPort | ep-contract-finance | ep-app-procure | 采购发票登记的应付明细条目写入、预付自动核销、超量开票挂账，在调用方事务内执行 |
| OverbillingMatchPort | ep-contract-finance | ep-app-inventory 或 ep-app-procure 的收货用例 | 规格第 5.2 章超量开票路径一的反向匹配，返回可匹配数量与单价 |
| UnbilledArPort | ep-contract-finance | ep-app-sales、ep-app-inventory | 交付确认与销售退货在应收账款未开票过渡科目上的子账腿写入 |
| CreditExposureQuery | ep-contract-finance | ep-app-sales | 返回该客户的应收未收金额与已交付未开票金额两项，供信用额度占用计算 |
| ReceivableLedgerQuery | ep-contract-finance | ep-app-reporting、ep-app-crm | 应收台账与核销关系只读查询 |
| PayableLedgerQuery | ep-contract-finance | ep-app-reporting | 应付台账与核销关系只读查询 |
| SupplierStatementQuery | ep-contract-finance | ep-app-portal | 供应商收付款对账查询的取数，返回未脱敏结构，脱敏在门户侧完成 |
| CashAccountQuery | ep-contract-finance | ep-app-procure、ep-app-reporting | 资金账户与资金腿明细只读查询 |
| AgingQuery | ep-contract-finance | ep-app-reporting | 应收账龄与应付账龄两张基础表的取数 |
| ReconciliationItemQuery | ep-contract-finance | ep-app-ledger 的关账前强制校验、job-worker 的内部对账组件 | 按法人与会计期间返回八项勾稽的子账侧合计，结构为 `ReconciliationItemView` |
| SalesInvoiceQuery | ep-contract-invoice | ep-app-clm、ep-app-sales、ep-app-reporting | 销项发票与收款计划勾稽的只读查询 |
| InvoiceReversalStatusQuery | ep-contract-invoice | ep-app-sales、ep-app-procure | 判定某张发票是否已完成红冲或作废，供销售退货与采购退货的前置校验，对应 PRD 第 6.5.4 |

---

### 6. 并发与事务边界

#### 6.1 事务清单

下表逐个用例给出事务边界。全部业务事务隔离级别 `READ COMMITTED`，按共享基线第 8.4 节。

| 用例 | 一个事务内包含的写入 | 事务外 |
|---|---|---|
| issue_sales_invoice | 销项发票行；发票申请单行更新与状态迁移；`invoice.invoice_receipt_plan_links`；`finance.receivable_entries` 新增；`finance.unbilled_ar_entries` CREDIT 行；预收自动核销的两张链接表与预收条目更新；ledger 端口生成凭证；`platform_audit` 审计事件；Outbox 条目 | 附件正文读写；站内通知；clm 收款计划已开票金额回写经 `ep-contract-clm` 在同一事务内的写端口完成，若 clm 只提供事件驱动回写则改由 Outbox 串接 |
| register_invoice_reversal | 冲销登记单；销项发票状态迁移或进项侧标记；发票申请单回滚；`invoice.invoice_receipt_plan_links` 反向行；`finance.receivable_entries` 或 `finance.payable_entries` 的冲回；`finance.unbilled_ar_entries` DEBIT 行；超量开票路径二结清；凭证；审计；Outbox | 同上 |
| register_receipt | 到款单；核销关系行；应收条目更新；预收条目新增；资金腿明细；凭证；审计；Outbox | 附件；通知 |
| register_payment | 付款登记单；核销关系行；应付条目更新；预付条目新增；资金腿明细；付款申请单已付金额回写经 `ep-contract-procure` 写端口；凭证；审计；Outbox | 附件；通知；重新认证校验在事务外先行完成 |
| register_refund | 退款单；`finance.refund_source_payment_links`；预收或预付条目更新，或应收应付条目核销；原款项单 `refunded_amount` 回写；资金腿明细；凭证；审计；Outbox | 附件；通知 |
| register_cash_document_reversal | 冲正单；全部反向核销行；台账条目回增；预收预付反向条目；资金腿反向明细；原单据置 REVERSED；凭证；审计；Outbox | 通知 |
| settle_overbilling_by_write_off | 结清记录；挂账更新；凭证；审计；Outbox | 通知 |
| match_overbilling_on_receipt | 结清记录；挂账更新；凭证附加腿。本用例由收货用例在其事务内调用，不自开事务 | |
| register_payable_on_purchase_invoice | 应付条目；预付自动核销；超量开票挂账。本用例由采购发票登记用例在其事务内调用，不自开事务 | |
| record_unbilled_ar_on_delivery | `finance.unbilled_ar_entries` DEBIT 行。由交付确认用例在其事务内调用 | |
| maintain_cash_account | 资金账户行；审计 | 审批在事务外 |

一个 HTTP 请求内不开启多个写事务，按共享基线第 10.3 节。批量导入按行拆事务，是后台任务而不是单个 HTTP 请求，不违反该纪律。

#### 6.2 锁策略

- 台账条目更新一律 `SELECT ... FOR UPDATE`，加锁顺序固定按 `id` 升序，避免两笔到款交叉核销同两条应收条目时死锁。
- 发票申请单在开具与冲销时 `SELECT ... FOR UPDATE`，并同时校验 `row_version`，双保险：行锁保证串行，版本号保证客户端看到的是最新剩余可开比例。
- 超量开票挂账在三条路径上一律 `SELECT ... FOR UPDATE`。
- 资金账户不加行锁，`has_cash_flow` 的置位用 `UPDATE ... WHERE id = $1 AND has_cash_flow = false`，幂等。
- 核销候选集的读取不加锁，逐行分配后在写入前对被选中的条目重新 `FOR UPDATE` 并复核 `open_amount`，复核失败返回 `BUSINESS_CONFLICT` 与 `FINANCE.SETTLEMENT.OPEN_AMOUNT_CHANGED`，回带最新余额，由界面重取。理由是先读后锁可把锁持有时间压到毫秒级，而 20 并发下冲突概率低，冲突时重取的代价远小于长事务持锁。

#### 6.3 幂等与 Outbox

写入幂等由共享基线第 5.4 节的 `platform_msg.idempotency_keys` 承担，与业务写入同事务。

本阶段发布的 11 个事件写入 `platform_msg.outbox_events`，信封的 `posting_date` 与 `accounting_period_id` 取本次解析结果。消费者为 notify、reporting 投影与 search 索引，均不做过账，见第 0.1 节。

消费端幂等由 `platform_msg.inbox_consumptions` 的唯一约束保证。重投退避按共享基线第 6.2 节。

#### 6.4 失败重试与补偿

- 序列化失败 40001 与死锁 40P01 由数据访问层重试 3 次，退避 50、150、450 毫秒，只对尚未产生外部可见副作用的事务重试。本阶段全部用例在提交前不产生外部副作用，因此全部可重试。
- 守恒 CHECK 违例不重试，直接映射为 `BUSINESS_CONFLICT` 并按规格第 15.2 章写入死信，`incident_no` 回带给界面。这一路径覆盖 PRD 第 6.7.7、6.8.8、6.11.3、6.12.7 的负数未核销余额行。
- ledger 端口返回借贷不平时不重试，整事务回滚并写死信，对应 PRD 第 6.4.8 最后一行。
- 批量导入行级失败不回滚已成功行，失败行写入结果对象并计入 `failed_rows`，批次状态置 PARTIALLY_FAILED。
- 本阶段不使用补偿事务，理由是全部跨模块写入都在同一数据库事务内经契约端口完成，不存在需要 Saga 的跨事务步骤。唯一的例外是 clm 收款计划回写，若 clm 只提供事件驱动接口，该腿改为 Outbox 串接并由死信兜底，届时需新增一条补偿用例，见第 11 节。

#### 6.5 必测并发场景在本阶段的落点

共享基线第 8.4 节固定的六组必测并发场景中，本阶段承担三组：同一单据的乐观锁冲突（发票申请单）、同一采购订单的并发发票匹配与暂估回冲（超量开票挂账侧）、Outbox 同一事件的重复投递不少于 3 次。另新增两组本阶段专有场景：同一应收条目被两笔到款并发核销、同一预收条目被开票自动核销与客户退款并发消费。

---

### 7. 配置项

全部按共享基线第 7 节：前缀 `EP__`，层级双下划线，`deny_unknown_fields`，敏感项不入配置。

| 键 | 类型 | 默认值 | 生效方式 |
|---|---|---|---|
| EP__INVOICE__TAX__AMOUNT_TOLERANCE | decimal | 0.02 | 启动时读取，变更需重启；取值写入 `platform_ops` 台账 |
| EP__INVOICE__RATIO__TOLERANCE | decimal | 0.000001 | 同上 |
| EP__INVOICE__ISSUE__REQUIRE_IMAGE_ATTACHMENT | bool | false | 同上 |
| EP__INVOICE__REVERSAL__REQUIRES_REAUTH | bool | true | 同上 |
| EP__INVOICE__IMPORT__MAX_ROWS | u32 | 2000 | 同上 |
| EP__INVOICE__IMPORT__ON_ROW_FAILURE | enum CONTINUE 或 ABORT | CONTINUE | 同上 |
| EP__FINANCE__SETTLEMENT__CROSS_PARTY_ALLOWED | bool | false | 同上 |
| EP__FINANCE__SETTLEMENT__MAX_LINES | u32 | 200 | 同上 |
| EP__FINANCE__RECEIPT__REQUIRES_APPROVAL | bool | false | 同上 |
| EP__FINANCE__REFUND__REQUIRES_REAUTH | bool | true | 同上 |
| EP__FINANCE__CASH_ACCOUNT__REQUIRES_APPROVAL | bool | true | 同上 |
| EP__FINANCE__CASH_DOCUMENT_REVERSAL__REQUIRES_REAUTH | bool | true | 同上 |
| EP__FINANCE__BANK_ACCOUNT__MASK_TAIL_DIGITS | u8 | 4 | 同上 |
| EP__FINANCE__RECON__MAX_PERIODS_PER_QUERY | u8 | 12 | 同上，限制对账视图单次查询的期间跨度 |

不进配置文件而进事务数据库并经配置发布通道的运行期业务参数：账龄分档（`finance.aging_bucket_definitions`）、税率可选值（`invoice.tax_rate_options`）、发票申请与开票的审批链、到款与付款的提醒规则。按共享基线第 7.1 节最后一段。

本阶段不新增启动自检项。共享基线第 7.3 节第 13 项“每个法人存在当前自然月的打开会计期间”由 ledger 阶段承担，本阶段在 `--check` 模式下额外输出两个法人的账龄分档与税率字典行数，只作报告不作判定。

---

### 8. 测试计划

#### 8.1 单元测试

位于两个 domain crate 内，不触库不触网不取真实时间。覆盖的分支逐条列出。

发票侧：

1. `TaxLine::validate` 的三种结果：等式成立、在容差内、超出容差；边界取容差恰好相等。
2. `gross_amount` 不等于 `net_amount + tax_amount` 的拒绝。
3. 发票种类与代码必填的四种组合：数电有代码、数电无代码、纸质有代码、纸质无代码。
4. `remaining_ratio` 扣减的三种结果：仍大于零、恰好归零、超出剩余。
5. `remaining_ratio` 回增的三种结果：回到部分开具、回到已审批、回增溢出。
6. 归零容差：扣减后剩余为 0.0000005 时归零，为 0.000002 时不归零。
7. 发票申请单状态机全部 11 条流转与 6 条非法流转。
8. 销项发票状态机 2 条合法流转与 4 条非法流转，含重复冲销与作废红冲互斥。
9. 开具日期与记账日期晚于基准日的拒绝，补记的接受。
10. 累计开票比例的取数范围：已作废与已红冲发票不计入。

财务侧：

11. 核销分配算法：候选为空、单条全额、单条部分、多条跨越、恰好用尽、有剩余、行数超限，共 7 个分支。
12. 人工指定顺序的三种失败：行超额、合计超额、条目不属该往来方。
13. 账龄分桶：六档各一个用例，加未到期、恰好到期日、跨最后一档开区间三个边界。
14. `OpenItem` 守恒：已核销超过原额、未核销为负、恒等式成立三个断言。
15. 可退上限：两个上界分别成为最小值的两种情形，加已部分退款后的剩余上限。
16. 超量开票余额推进：三条路径各一个用例，加路径三后请求路径一的拒绝、路径三冲回后路径一的接受。
17. 冲正：核销关系逐行反向的金额守恒、预收已被消费时的拒绝、资金腿方向反转。
18. 到款单与付款单的金额恒等式：核销合计加转预收等于到款金额。
19. 资金账户：账户类型与科目匹配的四种组合、已产生资金流水后的修改拒绝、停用后不可选。
20. 状态机：到款、付款、退款、冲正四个单据各自的合法与非法流转。

#### 8.2 领域属性测试

用 proptest，覆盖规格第 17.3 章中本阶段承担的不变量，是共享基线第 8.1 节要求的五组之外的追加组。

| 属性 | 断言 |
|---|---|
| 核销守恒 | 对任意随机生成的条目集合与任意随机到款金额序列，逐笔核销后每条条目满足 `0 <= settled_amount <= original_amount` 且 `open_amount = original_amount - settled_amount` |
| 到款金额守恒 | 对任意到款，核销行合计加转预收金额恒等于到款金额 |
| 预收守恒 | 预收条目的已核销金额不超过挂账金额，未核销余额非负 |
| 开票比例守恒 | 对任意开具与冲销序列，该申请单下有效发票的开票比例合计加剩余可开比例恒等于申请单开票比例，容差内成立 |
| 资金腿守恒 | 对任意到款、付款、退款、冲正序列，账户期末余额恒等于期初余额加收方向合计减付方向合计 |
| 过渡科目双向净额 | 对任意交付确认与开票的交错序列，净额恒等于已交付未开票减已开票未交付 |
| 冲正可逆 | 对任意资金单据，登记后再冲正，全部被触及台账条目的 `settled_amount` 与 `open_amount` 恢复到登记前取值 |
| 账龄不随顺延改变 | 对任意条目，改变 `accounting_period_id` 不改变账龄分桶结果 |

#### 8.3 集成测试

使用真实 PostgreSQL 16，每个用例独占一个 `ep_test_<nanoid>` 库，禁止用内存库替代。场景清单如下。

1. 开具登记全链路：申请单从 DRAFT 走到 FULLY_ISSUED，应收条目、过渡科目条目、收款计划勾稽、凭证五处逐项核对。
2. 红字冲销后重新开具：对应规格第 8 章第 7 步与第 17.2 章“作废与红字冲销后按同一发票申请单重新开具成功，累计开票比例校验不被误阻断”。
3. 作废与红冲互斥：两次登记，第二次被拒且写入审计。
4. 分次到款：三笔到款核销同一张发票，未核销余额逐次下降至零，账龄基数同步变化。
5. 一笔到款核销多张发票：五张发票，核销顺序按到期日升序、同日按单据编号升序，与规格第 5.2 章核销顺序规则块逐项比对。
6. 人工指定核销顺序：与默认顺序不同，审计事件中可查得该事实。
7. 到款金额大于可核销应收：对应规格第 17.2 章第五类必测分支，超出部分挂预收，子账与总账两侧金额一致，后续开票自动核销并进入账龄。
8. 付款金额大于可核销应付：对应第六类必测分支。
9. 预收自动核销：对应第二类必测分支，核销后预收余额、应收未核销余额与台账三处一致。
10. 预付自动核销：由采购发票登记触发，同上。
11. 已收款后的销售退货并退款：对应第八类必测分支，含红冲前置、退款登记、资金账户明细与银行存款科目余额一致。
12. 已付款后的采购退货并收回货款：对应第九类必测分支。
13. 超量开票挂账与三条结清路径：对应第十三类必测分支，逐条路径验证挂账余额归零、凭证借贷相等、关账拦截与解除。
14. 超量开票路径三之后到货：验证必须先冲回成本再走路径一。
15. 资金单据冲正：到款冲正、付款冲正、退款冲正各一个用例，含预收已被消费时的拒绝。
16. 会计期间顺延：在一次关账受理之后提交一笔到款，凭证与全部子账条目落入其后最早的可入账期间且 `deferred_from_period_id` 非空，两条检索路径均可查得，对应规格第 10.2 章的注入用例其一。
17. 对账视图八项：正常态差额为零；逐项注入差额后差额非零并生成差异事项引用；差额清零后恢复。注入方式为直接对台账条目做受控 UPDATE，仅在测试库上执行。
18. 批量导入：2000 行成功、含 3 行失败的部分失败、重跑同批次不产生重复发票。
19. 幂等：全部 8 个写端点各一次重放，返回首次结果并带 `Idempotent-Replay: true`；载荷不同时返回 409。
20. 法人越权矩阵：并入独立测试目标 `tests/rls_matrix`，覆盖读取、写入、更新、删除、聚合、排序、报表投影与错误信息泄漏八类，本阶段追加 34 张表与 14 个视图的条目，另覆盖资金账户银行账号字段级权限的两种上下文。
21. 高风险操作：开票、付款、超量开票路径三三项验证重新认证缺失时拒绝、审批未完成时拒绝、申请人自审时拒绝。
22. 并发：第 6.5 节列出的五组各一个用例，用两个连接交叉提交。

#### 8.4 端到端测试

后端 E2E 用 Rust 集成测试直接打 HTTP 接口，四端 UI 用 Playwright 驱动桌面 WebView。

- E2E-10-01：闭环第 6 步到第 11 步的连贯路径，从发票申请提交到退款登记，全程在应用内完成，中途不出现外部补齐环节。
- E2E-10-02：直运订单分支下的退款与返款走同一张单据、同一套字段与同一套勾稽校验，对应 PRD 第 6.12.6。
- E2E-10-03：移动端按规格第 6.2 章矩阵为仅查看，验证移动端可查得本阶段全部单据与台账、且提交入口不可达。
- E2E-10-04：供应商门户的收付款对账查询取数与内部应付台账同源，脱敏后返回，对应 PRD 第 4.9.6。
- E2E-10-05：关账受理后提交到款并观察界面显式标注顺延期间，对应 PRD 第 6.1.4。

#### 8.5 静态检查

- `ep-domain-invoice` 与 `ep-domain-finance` 中不出现 sqlx、reqwest、tokio 的 IO 模块、`std::fs`、`std::net`、`SystemTime::now`、`rand` 符号。
- `ep-app-invoice` 与 `ep-app-finance` 的用例函数中不出现 reqwest 与文件写入符号。
- 两个 schema 上不存在 `DELETE` 语句。
- `finance.cash_ledger_entries` 只被四个用例的仓储写入，由一段基于 `cargo metadata` 与调用图的自检脚本断言。
- `ep-app-invoice` 不依赖 `ep-app-finance`，反向亦然。

#### 8.6 性能相关项

对应规格附录 A.1 的度量项，在附录 A.3 基准数据集与附录 A.4 负载模型下实测。

| 度量项 | 通过线 | 本阶段的被测端点 |
|---|---|---|
| 应收发票生成 | 普通交易提交 P95 在 3 秒内 | POST /api/v1/invoice/sales-invoices |
| 发票申请提交 | 同上 | POST /api/v1/invoice/invoice-applications/{id}/actions/submit-for-approval |
| 发票作废或红字冲销登记 | 同上 | POST /api/v1/invoice/invoice-reversals |
| 到款登记 | 同上 | POST /api/v1/finance/receipts |
| 付款登记 | 同上 | POST /api/v1/finance/payments |
| 退款与返款登记 | 同上 | POST /api/v1/finance/refunds |
| 应收账龄分析 | 常用报表 P95 在 10 秒内 | GET /api/v1/finance/receivable-agings |
| 应付账龄分析 | 同上 | GET /api/v1/finance/payable-agings |
| 收付款对账查询 | 门户交互 P95 在 2 秒内 | 门户经 `ep-contract-finance` 的供应商对账查询 |

上述九项在基准数据集上必须无顺序扫描，阶段结束时提交对应查询的 `EXPLAIN (ANALYZE, BUFFERS)` 证据，按共享基线第 3.10 节最后一段。

#### 8.7 覆盖率门槛

本阶段绝大部分代码属规格第 17.3 章强制不变量相关代码，因此按 85% 一档执行。

| 路径 | 行覆盖率下限 |
|---|---|
| crates/domain/finance、crates/domain/invoice | 90%，本阶段在规格之上自加，理由是这两个 crate 全部是不变量断言与算法 |
| crates/application/finance、crates/application/invoice | 85% |
| crates/contract/finance、crates/contract/invoice | 70% |
| ep-adapter-db-pg 中本阶段新增的仓储实现 | 80% |
| 本阶段新增与修改代码整体 | 85% |

工具为 cargo-llvm-cov，阈值写入 `codecov.toml` 的路径规则。`#[ignore]` 用例在本阶段结束时必须为零。

---

### 9. 退出条件

以下每一条都可由自动化用例或可核对的产物判定，全部达成才算本阶段完成。

1. 34 张表与 14 个视图在空库上从零迁移成功，再按 `-- rollback:` 段全部回退成功，两次执行的迁移历史表状态一致。
2. `--check` 模式在两个法人上通过，含全部带法人列的表已 `ENABLE` 且 `FORCE` 行级安全的自检项。
3. 56 个端点全部有 OpenAPI 描述且描述与实现的字段名逐项一致，由契约测试断言。
4. 第 8.1 节列出的 20 组单元测试分支全部通过。
5. 第 8.2 节列出的 8 组领域属性测试在 1000 次随机用例下全部通过。
6. 第 8.3 节列出的 22 组集成测试全部通过。
7. 规格第 17.2 章十五类必测分支中的第二、五、六、八、九、十三类在本阶段的用例中通过，第十三类的三条结清路径逐条通过。
8. 十个勾稽项中本阶段承担的八项在基准数据集上差额为零；逐项注入差额后对账视图差额非零、可下钻、可追溯，清零后恢复为零，其中待处理超量开票一项以关账前余额非零的方式注入，对应规格第 10.2 章的发布验收口径。
9. 规格第 10.2 章的顺延入账注入用例在本阶段的到款登记上通过：凭证与全部子账条目落入同一个顺延后的期间，两条检索路径均可查得。
10. 法人越权测试集 `tests/rls_matrix` 追加本阶段条目后全部通过，八类判据无一泄漏。
11. 三项高风险操作的重新认证与审批控制通过身份与访问控制测试，认证方式、待签内容摘要、时间与设备可在审计证据中查得。
12. PRD 第 6.14.4 列出的 11 类动作全部写入审计，同一事实不只落日志，由审计探针逐项断言。
13. 第 8.6 节九项性能度量在基准数据集上达标，且九项对应查询的执行计划无顺序扫描。
14. 覆盖率达到第 8.7 节的分档门槛，工作区整体不低于 80%。
15. `docs/error-codes.md` 的新增错误码与 `ep-foundation::error::codes` 常量表一致，CI 校验通过，无重复码；`docs/event-catalog.md` 的 11 个新增事件与实现一致。
16. 共享基线的四处回写完成：第 5.4 节幂等 `request_hash` 排除 `X-Reauth-Token`、第 9.2 节新增三个指标、第 11 节新增资金账户期初余额与资金单据冲正两项决定、第 0.2 节的规格缺口已提交规格修订建议。
17. E2E-10-01 至 E2E-10-05 五个用例通过，其中 E2E-10-01 全程在应用内完成。
18. 严重与高危缺陷为零，中危缺陷登记并给出规避方案与责任人，按规格第 17.2 章发布缺陷门禁的口径。

---

### 10. 与规格和 PRD 的对应

#### 10.1 规格条目

| 规格章节 | 本阶段实现的内容 |
|---|---|
| 5.2 财务条目 | 应收台账、应付台账、发票申请与审批与开具登记、与合同收付款计划及订单勾稽、按发票核销到款、分次到款与分次付款、一笔款项核销多张发票、客户退款与供应商返款、预收台账与预付台账、银行与现金账户档案、三类事件的资金腿明细视图、资金流水不得独立登记 |
| 5.2 数电发票与税务条目 | 销项发票开具登记的八个字段、回写申请单状态与剩余可开比例、销项与进项两个方向的作废与红字冲销登记、回滚后按剩余可开比例重新开具、单一税种增值税、按实际开票结果登记的语义、人工录入与批量导入两条路径 |
| 5.2 财务规则条目事件-分录表 | 开票、到款、付款、退款、红字冲销与作废五类事件的调用与落库；采购发票事件的应付腿与超量开票腿；交付确认与销售退货两类事件在过渡科目上的子账腿 |
| 5.2 到款与付款的核销顺序规则块 | 默认按单据到期日升序、同日按单据编号升序；人工指定写入审计 |
| 5.2 超量开票的三条结清路径规则块 | 挂账登记与三条路径的完整实现，含路径三之后到货的先冲回再入账顺序 |
| 5.2 总账功能与期末处理块 | 记账日期的取值与校验；凭证与子账共用同一会计期间字段；顺延只改变期间归属不改变取价；两个日期与两条检索路径 |
| 7.7 法人行级隔离机制 | 34 张表的统一 RLS 策略、无 `BYPASSRLS`、跨法人查询按法人逐个设置变量 |
| 7.8 密钥域 | 银行账号按法人密钥域字段级加密存储 |
| 8 黄金业务闭环第 6、7、9、10、11 步 | 发票申请、发票开具登记与冲销、到款登记、付款登记、退货相关的退款与返款 |
| 10.2 主系统规则 | 待处理超量开票科目的子账侧口径与关账拦截的可达解除路径；本阶段不产生异步过账条目的声明 |
| 12.1 与 12.2 | 开票、付款、财务过账三类高风险操作的重新认证与审批；申请人不可自审 |
| 12.5 审计 | PRD 第 6.14.4 的 11 类动作与业务变更同事务写入审计 |
| 15.1 错误分类 | 五类分类中本阶段涉及的四类；错误封套的七个必含要素 |
| 15.2 可靠任务 | 守恒违例、凭证不平、勾稽差额三类进入死信与人工修复 |
| 16 与附录 A.1、A.2 | 第 8.6 节的九项度量 |
| 17.2 财务内核测试 | 应收应付核销的三项、十五类必测分支中的第二、五、六、八、九、十三类 |
| 17.3 强制不变量 | 应收应付核销守恒；子账与总账勾稽十项中的八项；已过账凭证不可覆盖由仅追加表与冲正路径承载 |

#### 10.2 PRD 节

| PRD 节 | 本阶段实现的功能 |
|---|---|
| 6.1.4 | 记账日期字段与三条校验；提交结果展示实际落入的会计期间并显式标注顺延；资金流水不得独立登记 |
| 6.1.5 | 登记语义的六条含义，含批量导入不放宽任何校验 |
| 6.2 全节 | 资金账户档案的九个字段、两条状态流转、资金腿明细视图的四列、五种异常 |
| 6.3 全节 | 发票申请单的十个字段、三项校验、审批要求、11 条状态流转、剩余可开比例、五种异常 |
| 6.4 全节 | 开具登记的十二个字段、四项系统处理、五项输出变化、批量导入、销项发票状态机、七种异常 |
| 6.5 全节 | 作废与红字冲销的八个字段、四项系统处理、与销售退货的先后关系、五种异常 |
| 6.6 全节 | 进项发票台账的四项内容、进项方向的冲销登记、供应商价格调整与金额税额更正的登记路径、三种异常 |
| 6.7 全节 | 到款登记的九个字段、核销明细行与核销顺序的六条规则、剩余款项与预收账款、五项输出、状态机、七种异常 |
| 6.8 全节 | 付款登记的读取信息、十个字段、核销与分次付款的五条规则、高风险控制、六项输出、状态机、七种异常 |
| 6.9 全节 | 应收台账的九类信息、核销守恒、核销关系的四条规则、账龄的五条规则、查询与权限 |
| 6.10 全节 | 应付台账同构内容；已收货未收票视图外壳；待处理超量开票视图 |
| 6.11 全节 | 预收台账与预付台账的六条与五条规则、三条用户可见校验、无人工新增与调整入口 |
| 6.12 全节 | 两类退款单的区分、十一个字段、四项校验、五项输出、状态机、直运情形、五种异常 |
| 6.13 全节 | 十个勾稽项中的八项对账视图、三条用户可见规则、八条本节内部勾稽 |
| 6.14 全节 | 四类错误的应用、四类不平的死信处置、幂等与重复提交、11 类审计 |
| 6.16 | 21 条待决项的临时取值与承载方式，见第 0.4 节 |
| 4.7.3 与 4.7.4 | 付款登记完成后回写付款申请的已付金额与状态，经 `ep-contract-procure` 写端口 |
| 4.9.6 | 供应商门户收付款对账查询的取数来源 |
| 8.3.4 | 应收账龄与应付账龄两张基础表的数据来源 |
| 11.3 | 同步等待上限 8 秒，超过转后台任务，本阶段只有批量导入与账龄大范围导出触及该线 |

#### 10.3 本阶段明确不做的事

按 PRD 第 6.15 节全部八条，不扩大也不收窄。另外，本阶段不实现凭证生成本身、不实现科目表与期间管理、不实现库存金额账、不实现采购发票单据与收货匹配、不实现销售退货与采购退货单据、不实现合同收付款计划，这六项分别属其他阶段。

---

### 11. 风险与预留

#### 11.1 技术风险

| 风险 | 影响 | 应对 |
|---|---|---|
| 第 0.1 节的同步过账口径若被整合结论推翻 | 七个用例的凭证腿需改为异步，PRD 三张异常表的行为描述需同步修订，核销上限校验需引入待过账占用量 | 把凭证生成收敛到一个 `PostingGateway` 抽象，用例只调用该抽象，改造面限制在一个文件；在阶段中期的整合评审上单列该议题 |
| 第 0.2 节的自动核销分录腿是本阶段的假设 | 若规格回写为其他形式，预收预付两项勾稽的实现需改 | 分录腿以 `AdditionalLeg` 参数传给 ledger 端口，不写死在本阶段；测试断言写在勾稽层而不是分录层 |
| clm 收款计划已开票金额的回写方式未定 | 若 clm 只提供事件驱动接口，开票用例的事务边界被打破，需要补偿路径 | 在契约层同时定义同步写端口与事件两种形态，装配时择一；若走事件，追加一条补偿用例与一项死信监控 |
| F-08 只允许全额红冲的临时取值 | 若财务负责人要求部分红冲，销项发票状态机需拆出部分红冲态，比例回滚公式需改，已落库数据需回填 | 已在 `invoice.sales_invoices` 上预留 `reversed_net_amount` 列，全额红冲时等于 `net_amount`，改为部分红冲时不需要加列 |
| F-16 逐行落库的临时取值 | 若改为整体回滚，逐行幂等键设计作废 | 导入器把逐行处理与批次编排分离，切换只改编排层 |
| 账龄查询在 6 万条应收条目与 12 个期间跨度下可能触及 10 秒线 | 附录 A.1 的两项报表度量不达标 | 账龄计算下推到数据库聚合而不是应用侧循环；分组键固定为四个；单次查询期间跨度由 `EP__FINANCE__RECON__MAX_PERIODS_PER_QUERY` 限制；不达标时按规格第 16 章执行性能整改，不放宽通过线 |
| 守恒 CHECK 违例在高并发下成为主要失败源 | 用户看到较多 `BUSINESS_CONFLICT` | 先读后锁的复核路径在冲突时回带最新余额，界面可一键重取；冲突次数进指标 `ep_finance_settlement_conflicts_total`，超阈值时调整核销候选集的预取策略 |
| 银行账号字段级加密与查重的组合 | 加盐哈希的盐若按法人固定，存在字典攻击面 | 盐取自法人数据加密密钥域派生，不落库；哈希只用于唯一约束，不用于检索 |

#### 11.2 为后续阶段预留的扩展点

1. `ep-contract-finance` 的 `CreditExposureQuery` 端口返回应收未收金额与已交付未开票金额两项，销售阶段的客户信用额度校验直接消费，第三项在途订单金额由销售阶段自持，对应规格第 5.2 章客户信用额度校验条目的三部分构成。
2. `finance.v_recon_inventory` 与 `finance.v_recon_grni` 两个视图外壳已建，库存与采购阶段只需实现子账侧契约查询即可接入，不改视图结构。
3. `finance.receivable_entries.source_doc_type` 与 `finance.payable_entries.source_doc_type` 是可扩展枚举，为后续版本的其他应收应付来源留位，首版只有一个取值。
4. `invoice.sales_invoices` 不设明细行表，但 `invoice.invoice_receipt_plan_links` 已按多行结构建立，将来支持一张发票多行明细时只需新增 `invoice.sales_invoice_lines` 并把税额校验从头表下移到行表。
5. `finance.cash_ledger_entries.source_doc_type` 为可扩展枚举，后续版本引入银企直连流水时新增取值即可，但资金流水不得独立登记的约束在首版必须保持。
6. 对账视图的十项统一为同一 DTO 结构 `ReconciliationItemView { item_code, legal_entity_id, accounting_period_id, subsidiary_amount, ledger_amount, difference }`，内部对账组件与关账前强制校验直接消费该结构，不需要为每项写一套取数。
7. 报表阶段的应收账龄与应付账龄两张基础表直接消费 `finance.v_receivable_aging` 与 `finance.v_payable_aging`，不另建一套口径。
8. 门户阶段的供应商对账查询消费 `ep-contract-finance::SupplierStatementQuery`，脱敏投影在门户侧完成，本阶段不返回任何脱敏后的数据结构，避免两套口径。

#### 11.3 需回写共享基线的项

1. 第 5.4 节：`request_hash` 的计算排除 `X-Reauth-Token`。
2. 第 9.2 节：新增三个指标 `ep_finance_settlement_conflicts_total`、`ep_finance_reconciliation_difference_amount`（gauge，标签 `item`、`legal_entity_id`）、`ep_invoice_import_rows_total`（counter，标签 `outcome`）。
3. 第 11 节：新增两项全局取值，即资金账户期初余额的存在与勾稽要求、资金类单据的冲正登记路径（临时闭合 U-D-02）。
4. 第 3.5 节：确认本阶段全部金额列均为 `numeric(18,2)`、比例列为 `numeric(9,6)`、数量列为 `numeric(18,6)`，本阶段未引入任何新的精度语义。
