> **F-57 状态：`HISTORICAL_DO_NOT_EXECUTE`。** 本文只保留历史任务正文，现行工作由 F-57 **Task 20** 承接。当前权威集合为 [F-57 总体设计](../../specs/2026-08-23-f57-governed-automation-fabric-design.md)、[F-57 需求追踪矩阵](../../reviews/2026-08-23-f57-requirements-traceability.md)、[F-57 权威替代登记](../../reviews/2026-08-23-f57-authority-supersession-register.md)与 [F-57 实施计划](../2026-08-23-f57-governed-automation-fabric-implementation.md)；F-57 是设计/计划权威，不是产品已实现声明，本地模型实现延期。

## 阶段 10：财务内核二 —— 往来与发票

> **F-50/F-51 完整替换边界。** invoice 最终 17 表/18 个本目录迁移，finance 23 表/25 个本目录迁移，另有 inventory、portal、procure 各 1 个目标晚建外键追补，合计 40 张法人 RLS 表、46 个迁移、19 个只读视图；公开端点固定为 49 个（23 写、26 读），自有 contract traits 为 16 个（F-10 新增只读 `ReceiptPlanBillingQuery`），另消费阶段 7 已冻结的 2 个内部 owner port；本阶段事件为 13 个，活跃自有错误码为 61 个（FINANCE 31 + INVOICE 30），另传播 MDM 1 个与 PORTAL 2 个。此前与本卷现行表结构、金额方向、余额口径、冲销链、税额模型、锁序或 owner 端口相冲突的段落全部失效；第 3—11 节已经逐项改写为唯一可实现正文，不依赖本段替代声明解释旧结构。

本阶段交付 invoice 与 finance 两个模块码的契约层、领域层、应用层与数据库结构，覆盖销项发票申请与开具登记、进项发票台账与采购发票登记及三单匹配、销项作废以及销项与进项两个方向的红字冲销登记、应收应付台账与账龄、预收预付台账、到款与付款登记与核销、客户退款与供应商返款、资金账户与资金腿明细、应收账款未开票过渡科目子账、待处理超量开票子账与三条结清路径、往来与预收预付的期初余额导入，以及规格第 17.3 章与 PRD 第 6.13.1 合计十个勾稽项的对账视图。

按跨阶段裁定 A-10，进项发票台账归 invoice 模块，因此 `invoice.purchase_invoices` 与 `invoice.purchase_invoice_lines` 两张表、采购发票登记用例与三单匹配一并归本阶段，采购阶段不建表也不写台账，与基线第 1.2 节 invoice 覆盖销项与进项发票台账一致。

本阶段不定义任何借贷方向、科目、取价与拆分规则。全部账务处理指向规格第 5.2 章财务规则条目的事件-分录表及其后七个规则块，本文只写它在哪一步被调用、以什么参数被调用、结果落到哪张表。

---

### 0. 本阶段的口径前提与显式假设
#### 0.0 本阶段在 T0 贯通线上的最小切片

在阶段 3b-1 结束后、阶段 5 全量开工之前插入一条不新增任何范围的最薄贯通线 T0，判据是一条合同从建单走到管理层看到一个数。本阶段向 T0 贡献五项最小切片，全部取自本阶段既有交付物，不新增端点或第二套契约：一条 `invoice.invoice_number_registry` 号码登记、一张 `invoice.sales_invoices` 销项发票头及至少一条 `invoice.sales_invoice_lines`（允许单行样本，但必须走正式头行模型）、`finance.receivable_entries` 的一条应收正向主条目、`finance.receipts` 与 `finance.receivable_settlement_links` 的一笔到款与一次全额核销、`finance.cash_accounts` 的一个银行账户建档，以及 `invoice.tax_rate_options` 的建表与种子及 `ep_contract_invoice::TaxRateOptionQuery` 的 `default_rate` 与 `list` 两个方法。税率字典仍是唯一出处。承载用例为 `issue_sales_invoice`、`register_receipt` 与 `maintain_cash_account`，最小端点仍为 `POST /api/v1/invoice/sales-invoices`、`POST /api/v1/finance/receipts` 与 `POST /api/v1/finance/cash-accounts`；税率查询不设端点。发票申请单在 T0 中只走单审批节点。

这些切片对应的 invoice 迁移至少包括 `V20261019090000__invoice_create_tax_rate_options.sql`、`V20261019090100__invoice_create_invoice_applications.sql`、`V20261019090200__invoice_create_invoice_application_link_tables.sql`、`V20261019090300__invoice_create_invoice_number_registry.sql`、`V20261019090400__invoice_create_sales_invoices.sql`、`V20261019090500__invoice_create_sales_invoice_lines.sql`、`V20261019091000__invoice_create_invoice_receipt_plan_links.sql` 与 `V20261019091500__invoice_backfill_seed_tax_rate_options.sql`；finance 迁移至少包括 `V20261019091800__finance_create_cash_accounts.sql`、`V20261019091900__finance_create_receivable_entries.sql`、`V20261019092600__finance_create_receipts.sql`、`V20261019093100__finance_create_settlement_link_tables.sql` 与 `V20261019093200__finance_create_cash_ledger_entries.sql`。RLS 与索引迁移按 schema 整目录执行。T0 样本必须经正常用例同时写入号码登记、发票头和至少一行，不得由夹具直写头表。

T0 明确不要求的部分在本阶段一律不提前：不用 `ep-datagen` 的基准规模数据集，只用最小样本；不要求分支覆盖，进项发票与三单匹配、销项作废及两向红字冲销、预收预付自动核销、超量开票三条结清路径、退款与返款、资金单据冲正、账龄与期初导入一概不进 T0；不要求四端，只要求桌面端；不要求十项勾稽全绿，只要求应收一项在最小样本上差额为零。第 0.5 节列出的八个反向依赖点在 T0 中一个都不出现，因为 T0 不含交付确认、不含采购侧、不含退货。

T0 通过后，本阶段其余全部内容改为在这条已贯通的骨架上加厚，即在已经跑通的销项发票、应收条目、到款与核销之上追加上一段列举的各项，不再有第二次首次贯通的动作。M7 相应保留为全分支闭环的判定点，不再是黄金业务闭环的首次贯通点。

#### 0.1 过账时机：同步在业务事务内生成凭证

本阶段的决定：由本阶段单据直接触发的九类会产生凭证事件（销项开票、进项发票登记、销项冲销、进项冲销、到款、付款、退款/返款、资金单据冲正、超量开票三路径统一结清事件），其总账凭证、子账台账条目与核销关系在同一个业务数据库事务内同步写入，凭证经 `ep-contract-ledger` 的过账端口生成，本阶段不向 Outbox 投递任何需要异步过账的条目。该决定与跨阶段裁定 C-28 一致：全部凭证一律与业务事件同事务生成，Outbox 只承载派生、通知、检索与报表数据集。

理由有三条。其一，PRD 第 6.4.8、6.7.7、6.8.8 三张异常表都把“凭证生成失败或借贷不平”的系统行为定为“提交失败并进入死信与人工修复”，提交失败只有在凭证与业务写入同事务时才成立。其二，核销明细行的逐行上限校验必须读到主条目锁后 `effective_open`，异步过账会让紧邻的两次登记读到过期容量。其三，规格第 5.2 章要求同一业务事件的子账条目与凭证共用同一个会计期间字段，同事务写入使该要求成为结构性事实而不是运行期约定。

与规格第 10.2 章关账受理前提二的关系：受理前提二的判定语句按跨阶段裁定 C-28 固定为一句话，本阶段与阶段 4、阶段 9 三处逐字一致，即该法人该期间内，`platform_msg.outbox_events` 中 `status` 属于 PENDING 或 DISPATCHING、`posting_date` 落在该期间起止之间、且 `event_type` 命中 `ledger.posting_trigger_event_types` 的条目数为零，且 `platform_msg.dead_letters` 中 `state` 属于 OPEN 或 REPAIRING、同样命中该注册表的条数为零。`posting_date` 为空的平台事件一律不计入，理由是它们不产生凭证。判定所用视图固定为 `ledger.v_pending_posting_backlog`，两个错误码固定为 `LEDGER.PERIOD_CLOSE_REQUEST.PENDING_POSTING_BACKLOG` 与 `LEDGER.PERIOD_CLOSE_REQUEST.UNREPAIRED_DEAD_LETTERS`，视图与错误码均由阶段 9a 提供。

本阶段发布的 13 个事件中有 9 个在 `ledger.posting_trigger_event_types` 中有登记行；`finance.overbilling_entry.settled.v1` 也是会产生凭证的统一结清事件，不能漏登。`portal.supplier_invoice_upload.accepted.v1` 不产生另一张凭证，因而不新增 posting-trigger。登记行按裁定 A-21 由阶段 9a 的种子迁移一次写入，本阶段不新增任何回填迁移，见第 5.8 节末表。这 9 个事件的凭证已在业务事务内生成，Outbox 条目只驱动派生传播，即站内通知、报表投影、检索索引与客户 360 视图；但按上述判定语句，它们在被消费完毕之前仍计入该计数，因此关账受理需等待其消费结束，而不是等待任何异步过账。

与顺延入账的关系：会计期间在业务事务内由 `AccountingPeriodResolver` 一次解析并同时写入凭证与全部子账条目，因此规格第 10.2 章“受理时点在途写事务”这一集合天然覆盖本阶段的在途提交，等待该集合结束后建立的快照必然包含这些凭证。

本节口径已由跨阶段裁定 C-28 定死，不再是待整合议题。本阶段不存在也不预留任何异步过账路径，第 5.8 节的全部事件消费方均不做过账，核销上限校验只读已落库 current view 的 `effective_open`，不引入待过账占用量。

#### 0.2 F-50 已关闭：自动核销预收预付的分录腿

F-50 第 3.4 节已冻结唯一口径，本项不再是规格缺口或开工前置。销项开票时，自动核销额 `A` 在同一张开票凭证的基础腿外增加借预收账款、贷应收账款；采购发票登记时，在同一张采购发票凭证的基础腿外增加借应付账款、贷预付账款。两类来源统一提交计量项 `advance_auto_applied_amount = A`，不新增事件、凭证来源类型或第二张凭证。

finance 在同一事务内按同一法人、往来方、合同及有效收付款计划，依 `created_at ASC, id ASC` 消费可追溯开放预收/预付；每个分段同时为预收/预付追加 `APPLY`，并为本次应收/应付 `ORIGINAL` 主条目追加 `funding_origin = ADVANCE_AUTO` 的 `APPLY` 根行。两侧分段金额与资金根必须逐项一致，`A` 不得超过锁后 `effective_open` 与候选 `advance_open` 合计的较小值。锁序、期间归属、历史切片与事务末勾稽逐字按 F-50 第 3.4、5、10 节执行，差额非零整体回滚。

#### 0.3 规格缺口：资金账户期初余额

PRD 第 6.2.2 的资金账户字段表没有期初余额，而第 6.2.4 的资金腿明细视图要求展示期初余额，规格第 17.3 章又要求资金流水台账按账户的余额合计等于银行存款科目余额。上线首期若科目有期初余额而资金腿明细没有，该项勾稽在首期即为非零差额。

本阶段的显式假设：`finance.cash_accounts` 增加 `opening_balance` 与 `opening_balance_period_id` 两列，建档时录入一次，建档后不可修改；同一法人下全部银行存款类账户的期初余额合计必须等于总账银行存款科目的期初余额，现金类账户同理，该等式由本阶段的对账视图逐期校验。

按跨阶段裁定 A-24，本项目不设独立的数据迁移阶段，期初与历史数据按模块归属分落三处：总账期初余额归阶段 9a 的期初余额批次；库存期初归阶段 8 的 `MIGRATION_STOCK_ADJUSTMENT` 来源类型；应收应付预收预付期初与资金账户期初归本阶段，前者经第 4.12 节的期初导入通道，后者经本节两列在建档时一次录入。四个通道的写入一律不生成凭证，两侧的平衡由第 3.3 节的对账视图在首个会计期间校验。

#### 0.4 被阻塞的业务决策项与本阶段的冻结取值

下表逐条对应 PRD 第 6.16 节的 F 编号与附录乙的 U-D 组编号，另含 U-A-12 一条。本阶段不代替决策人决策，F 与 U-D 两组的决策人为财务负责人，U-A-12 的决策人为安全负责人与产品负责人；每一条都给出冻结取值，否则表结构与校验无法落地。冻结取值一律以配置项、配置发布对象或登记行承载，切换时不改表结构的标注为低代价。

| 编号 | 冻结取值 | 承载方式 | 切换代价 |
|---|---|---|---|
| F-01 / U-D-03 | **已由 F-50 关闭。** `UNIFIED_20` 为无代码的 20 位 ASCII 数字号码；`LEGACY_CODE_NUMBER` 为 10/12 位 ASCII 数字代码加 8 位号码。`invoice_medium` 与 `number_scheme` 分离，完整标识经 `invoice.invoice_number_registry` 在同法人跨四类蓝/红票唯一 | 中央登记表、数据库生成 `identifier_key`、复合 FK 与延迟 owner 约束 | 已冻结，不再是冻结取值 |
| F-02 / U-D-04 | **已冻结。** 税率取自 `invoice.tax_rate_options`，出厂预置 0.130000、0.090000、0.060000、0.030000、0.010000、0.000000；销项、进项均为至少一行的多行模型，税率只在行上，同票允许多税率，头三项金额只做行汇总 | `invoice.sales_invoice_lines`、`invoice.purchase_invoice_lines`、行税额验证器与 `TaxRateOptionQuery::default_rate/list`；出厂种子仍由第 3.6 节现有迁移承载 | 已冻结，未来改口径必须正式设计变更 |
| F-03 / U-D-05 | **已由 F-50 关闭。** 金额 `numeric(18,2)`、税率 `numeric(9,6)`，税额按 half-up（中点远离零）到两位；普通行容差默认且最大 0.02，价税合计严格相等；纯税额冲销只走 F-50 第 6.4 节特例 | 配置项 `EP__INVOICE__TAX__AMOUNT_TOLERANCE` 与统一行验证器 | 已冻结，不再是冻结取值 |
| F-04 / U-D-06 | 剩余可开比例的计算基数为合同金额；比例列类型 `numeric(9,6)`；累计比例校验容差 0.000001 | 配置项 `EP__INVOICE__RATIO__TOLERANCE` | 基数改为订单金额合计为中，需改取数与回滚公式 |
| F-05 / U-D-07 | 申请金额不可人工改写，等于 `round(开票比例 * 合同金额, 2)` | 领域规则 | 低 |
| F-06 / U-D-08 | 开票内容为自由文本，长度上限 500 | 表列 CHECK | 改为逐行对应为中 |
| F-07 | 开具登记不强制上传影像附件 | 配置项 `EP__INVOICE__ISSUE__REQUIRE_IMAGE_ATTACHMENT`，默认 false | 低 |
| F-08 / U-D-09 | **已由 F-50 关闭。** 红字发票法定号码必填；销项、进项均允许按原票行分次部分红冲，累计数量和三项金额不得超原行，销项状态为 `ISSUED/PARTIALLY_RED_REVERSED/VOIDED/RED_REVERSED` | 统一冲销头行、`source_effect_seq` 与延迟累计约束；比例按本次冲销金额逐次回滚并在末次归尾 | 已冻结，不再是实现方选择 |
| F-09 / U-D-10 | 作废与红字冲销登记复用开票的高风险控制，即重新认证加审批 | 领域规则，不设开关；规格第 12.1 章的高风险控制不得由配置关闭 | 低 |
| F-10 / U-D-11 | 账龄分档为 0 至 30、31 至 60、61 至 90、91 至 180、181 至 360、361 以上，六档 | 按裁定 C-08，本阶段先写入临时表 `finance.aging_bucket_definitions`，只出厂预置一套六档且不提供配置发布入口；阶段 11 交付 `reporting.aging_bucket_profiles` 与 `reporting.aging_bucket_lines` 后迁移并删除本表，按法人分套的形态随阶段 11 一并交付，此后取用入口唯一为 `ep_contract_reporting::AgingBucketQuery::buckets` | 低 |
| F-11 / U-D-12 | 到期日取值优先级为：关联收付款计划行的到期日；缺失时取发票开具日期加往来方档案上的约定账期天数；仍缺失时取发票开具日期 | 领域服务 `DueDateResolver`，账期天数经 `ep-contract-mdm` 读取 | 低 |
| F-12 / U-D-13 | 可核销范围限定为同一法人同一往来方，不允许跨客户或跨供应商核销 | 配置项 `EP__FINANCE__SETTLEMENT__CROSS_PARTY_ALLOWED`，默认 false | 放开为中，需改越权测试集与账龄归属 |
| F-13 / U-D-14 | 到款登记不需重新认证也不需审批；客户退款与供应商返款需重新认证加审批；资金账户档案的新增、修改与停用需审批不需重新认证 | 到款审批与资金账户审批两项恢复为配置项 `EP__FINANCE__RECEIPT__REQUIRES_APPROVAL` 与 `EP__FINANCE__CASH_ACCOUNT__REQUIRES_APPROVAL`，见第 7 节；退款与返款的重新认证为领域规则，不设开关，理由是规格第 12.1 章第 1050 行把付款与财务过账列为必须重新认证的六类高风险操作，该项不得由配置关闭 | 低 |
| F-14 / U-D-02 | 新增资金单据冲正登记单，见第 4.7 节，是本阶段为闭合该缺口而新增的单据类型 | 表 `finance.cash_document_reversals` 与同名用例 | 若财务负责人另选路径，改动为一张表加一个用例，中等 |
| F-15 / U-D-15 | 可退上限见第 4.8 节的算法 | 领域服务 `RefundCapCalculator` | 低 |
| F-16 | 批量导入单次上限 2000 行，逐行独立事务落库，失败行不回滚已成功行，逐行返回原因 | 两个配置项 `EP__INVOICE__IMPORT__MAX_ROWS` 与 `EP__INVOICE__IMPORT__ON_ROW_FAILURE`，见第 7 节 | 改为整体回滚为中，需把导入改为单事务并放弃逐行幂等 |
| F-17 / U-A-12 | **已由 F-51 关闭。** `bank_name` 与 `bank_account_no` 均登记为密级 30 并字段级加密；列表和普通详情的账号只显示末四位，完整账号需 `finance.cash_account.bank_account_no.read_full`、重新认证并写审计；银行名只向具备 `finance.cash_account.bank_name.read` 字段权限的主体显示；任何含银行名或账号的导出均需重新认证与审批 | 两行登记落 `platform_core.sensitive_field_registry`，密文列、账号 tail 与 32 字节盲索引见第 3.2.2、3.6 节；字段授权落 `platform_authz.field_permissions` | 已冻结；变更须走正式安全设计变更 |
| F-18 / U-D-16 | 同一法人允许多个现金账户并存 | 无唯一约束 | 收紧为唯一为低 |
| F-19 | 按共享基线第 11.5 节，本阶段不另取值 | 共享基线 | 无 |
| F-20 | 文案集中在 `docs/error-codes.md`，代码只引用常量 | 共享基线第 10.2 节 | 无 |
| F-21 | 交叉引用按本文正文写明的 PRD 节号，不写节名 | 本文 | 无 |

被阻塞判定：本阶段不因上表任一条被阻塞，全部有可执行取值。风险最高的是 F-08 与 F-16，两者若在阶段结束后才改，会触及已落库数据的语义，需要数据回填。原第 0.2 节自动核销分录缺口已由 F-50 第 3.4 节关闭，第 31 条现为正常验收项，不再是开工前置。
#### 0.5 反向依赖点的处置

本阶段是八个反向依赖点的提供方。原裁定通则第三条允许调用方先注入以 Noop 前缀命名的空实现、由本阶段替换、并把调用方的验收项顺延到本阶段，该通则已删除。删除理由有二：在判定类与记账类端口上，返回零值或恒定业务分支不是缺省而是一个会被记进账的错误答案，且 fail-open 与 fail-closed 的选择被下放到 wiring 里的一行；顺延使调用方阶段的退出条件不再证明任何闭环事实。取而代之的硬规则是三选一，逐个端口择其一并在下表写明。三档一律不得出现返回零值、空集合、固定业务分支或恒定成功的实现，发布装配中不得注入任何占位类型。

| 端口 | 调用方 | 处置 | 落地口径 |
|---|---|---|---|
| UnbilledArPort | 阶段 6 的交付确认与销售退货 | 同批交付 | 按总览第 1.5 节第八条其三与第十条，本端口与 `finance.unbilled_ar_entries` 均不在 T0 内，两者的真实实现与阶段 6 第三批的交付确认用例同批施工、同批验收；接线到位之前阶段 6 的交付确认用例不建立过渡科目腿的调用点，不存在取 None 或取零值的形态，三腿在接线当次一起真实执行 |
| ReceivableExposureQuery | 阶段 6 的信用敞口入口 | 同批交付 | 按总览第 1.5 节第八条其三与第十条，本端口不在 T0 内；两桶取数与 `ep_contract_sales::CreditExposureQueryPort` 的组装与阶段 6 第三批同批施工、同批验收；接线之前阶段 6 的信用校验按端口不可用处理，返回 INFRASTRUCTURE 且可重试，不得按零敞口放行 |
| InvoiceReversalStatusQuery | 阶段 6 的销售退货前置校验 | 同批交付 | 按总览第 1.5 节第八条其三采纳阶段 6 的写法、撤销本阶段原写的整条推迟：本端口与上两行三者一并落在阶段 6 第三批，与本阶段的 invoice 与 finance 端口同批施工、同批验收，本端口不在 T0 内；接线之前阶段 6 只实现未开票分支并对已开票行硬阻断，不注入任何替身；错误码 `SALES.SALES_RETURN.INVOICE_NOT_CREDIT_NOTED` 与本端口同批生效 |
| ReceiptInvoiceMatchQueryPort | 阶段 7 的采购退货 | 整条推迟 | 阶段 7 首次施工只落未开票内核且不发布临时完整系统；已登记与混合分支、本端口、`PurchaseCreditNotePort` 及固定锁序在本阶段同批接线后一次形成可发布闭环 |
| PurchaseCreditNotePort | 阶段 7 的采购退货 | 整条推迟 | 红字进项发票登记随上一行的已登记分支一并在本阶段落地，阶段 7 不建该调用点 |
| OverbillingMatchPort | 阶段 7 的收货用例 | 同批交付 | 超量开票挂账只由本阶段的采购发票登记产生，本阶段交付之前 `finance.overbilling_entries` 恒为空、路径一没有任何可匹配对象，因此阶段 7 的收货用例在该窗口内不接本端口即为正确行为；本阶段交付本端口时一并完成收货用例的接线 |
| PayableLedgerQuery | 阶段 7 的付款申请占用校验 | 整条推迟 | 按总览第 1.5 节第八条其一采纳阶段 7 的写法、撤销本阶段原写的降级窗口写法：付款申请的 `INVOICE_PAYMENT` 分支、其占用写入路径与 `PROCURE.PAYMENT_REQUEST.AMOUNT_EXCEEDS_OPEN_BALANCE` 判定连同其用例整条推到本阶段，与本端口同批交付；阶段 7 只受理 `PREPAYMENT` 一类并对 `INVOICE_PAYMENT` 硬阻断，不以采购订单金额等更宽口径替代；本阶段不开也不关任何 `PORT_NOT_IMPLEMENTED` 降级窗口 |
| SupplierStatementQuery | 阶段 7 的供应商门户对账 | 整条推迟 | 供应商门户的收付款对账查询入口整条推迟到本阶段，阶段 7 不建该入口 |

本表只改各阶段内部的工作次序，不改任何阶段的范围归属，也不改任何迁移文件的版本号。本阶段不承接来自阶段 6 与阶段 7 的任何顺延验收项，第 9 节退出条件不再逐条复述顺延清单。

---

### 1. 交付物清单

本阶段结束时，下列各项在单台服务器上可运行、可演示、可用自动化用例判定。

1. 两个模块的三层 crate 全部编译通过并接入 `core-server`：`ep-contract-invoice`、`ep-domain-invoice`、`ep-app-invoice`、`ep-contract-finance`、`ep-domain-finance`、`ep-app-finance`。
2. 本阶段 46 个迁移（含 `db/migrations/invoice/`、`finance/` 主目录及 `inventory/`、`portal/`、`procure/` 各一支追补）可按全局版本序离线执行到最新版本，且可按各文件头 `-- rollback:` 段回退到本阶段起点。
3. 40 张业务表与 19 个只读视图在 `ep` 库中建立，其中 invoice 17 张、finance 23 张；19 个视图为 8 个 finance 自有 SQL 对账视图、4 个业务查询视图、3 个受治理数据集视图，加四个 `finance.v_*_current` 经营余额视图。存货与 GRNI 两个子账项只经 snapshot owner port 运行时组装，不伪装成 SQL view。全部带法人列的表已 `ENABLE` 且 `FORCE` 行级安全并挂上统一策略。
4. 第 5 节逐个展开后固定为 49 个 HTTP method/path operation，在 `/api/v1/invoice/**` 与 `/api/v1/finance/**` 下可用，含 OpenAPI 描述文件 `docs/openapi/invoice.v1.yaml` 与 `docs/openapi/finance.v1.yaml`。原始计划的表实际只有 47 个 operation，后续具名增加 `POST /api/v1/invoice/purchase-invoices` 与 `POST /api/v1/finance/opening-balances/actions/import` 后为 49；此前未与表核对的扩大总数作废，不代表漏列任何未命名端点，也不得据此扩范围。当前两份 YAML 已按第 5 节完整展开为 invoice 16 个、finance 33 个 operation，均固定 `x-scope: stage10-full-surface`；handler、路由注册与契约测试一律以这两份完整文件为输入，不再保留受影响路径种子或“实现前再补契约”的步骤。
5. 16 个自有对外契约 trait 在 `ep-contract-finance` 与 `ep-contract-invoice` 中定义并有实现注册到 `apps/core-server/src/wiring/` 与 `apps/job-worker/src/wiring/` 两个目录，供 sales、procure、clm、portal、reporting、ledger 六个模块调用，清单见第 5.9 节。其中 8 个 trait 按第 0.5 节的三档处置与其调用方同批接线，F-10 的 `ReceiptPlanBillingQuery` 则与两条合同终止影响规则同批接线；阶段 6 与阶段 7 不注入任何空实现，本阶段也不承接任何顺延验收。本阶段的采购发票、进项红字、付款及勾稽组装另消费其它 owner 已冻结的 `SupplierInvoiceUploadWritebackPort`、`GrniEffectWritebackPort`、`GrniSubledgerBalancePort`、`PaymentRequestQueryPort`、`PaymentRequestWritebackPort`、`PayableReservationReadPort`、`PurchaseOrderInvoicingPort`、`InventoryVariancePort`、`StockValueSubledgerBalancePort`、`PurchaseInvoiceCaptureReversalBasisQuery`、`AccountingPeriodResolver`、`PostingPort` 与 `CrossModuleLockCoordinator`；这些外部 trait 只消费、逐个真实接线，不计入 16 个自有 trait。
6. 13 个领域事件登记到 `docs/event-catalog.md` 并可从 `platform_msg.outbox_events` 中查得，其中 9 个在 `ledger.posting_trigger_event_types` 中有登记行，按裁定 A-21 该登记行由阶段 9a 的种子迁移写入且每行只填 `event_type`；本阶段只在 CI 中由 `xtask configdoc` 与 `docs/event-catalog.md` 逐字比对，不进启动自检，不作为关账受理的前置校验，也不交付回填迁移。
7. 规格第 17.3 章与 PRD 第 6.13.1 合计 10 个勾稽项全部完整实现，可在应用内按法人与会计期间查询并展示子账侧、总账侧与差额三列。其中 8 项来自 finance 自有 SQL view；存货与已收货未收票两项按裁定 B-08 与 G-01 由 `ReconciliationItemQuery` 在同一 `SnapshotCtx` 内分别调用阶段 8 的 `StockValueSubledgerBalancePort` 与阶段 7 的 `GrniSubledgerBalancePort`，不建不能调用 Rust DI 的伪 view。
8. 一条可重复执行的端到端脚本 `testkit/scenarios/stage10_ar_ap_closed_loop.rs`，覆盖规格第 8 章闭环第 6、7、9、10、11 步，并串起规格第 17.2 章十五类必测分支中的第二、五、六、八、九、十三类。该脚本在 T0 已跑通的最小路径上加厚，开票与到款两段直接复用 T0 的步骤函数而不重写；该脚本的步骤函数再供阶段 9b 的 `testkit/scenarios/golden_loop_14_steps.rs` 复用，黄金业务闭环十四步的整体端到端验收落在阶段 9b，本阶段不承担。
9. `ep-datagen` 增加往来与发票子集生成器，可在基准规模下产出销项发票 6 万张、进项发票 4 万张、应收明细条目 6 万条、应付明细条目 4 万条、到款单 3 万张、付款单 2 万张、资金腿明细 6 万条，用于附录 A.1 的应收账龄分析与应付账龄分析两项报表实测。
10. `docs/error-codes.md` 登记本阶段活跃自有错误码精确 61 个（FINANCE 31 + INVOICE 30），另传播 MDM 1 个与 PORTAL 2 个；15 个被 F-50 替代的旧码只留历史标记，不得返回。`docs/data-dictionary/invoice.md` 与 `docs/data-dictionary/finance.md` 给出完整对象清单；`docs/data-dictionary.md` 的单据类型码一节增补 PINV 一码，见裁定 C-26。
11. 三个新增指标接入 ops-agent 暴露端点。
12. invoice 与 finance 两个模块的四端界面：目录为 `clients/desktop/src/modules/invoice/`、`clients/desktop/src/modules/finance/`、`clients/mobile/src/modules/invoice/`、`clients/mobile/src/modules/finance/`，按裁定 A-23 由本阶段而不是阶段 13 交付。
13. 三个受治理数据集视图 `invoice.v_purchase_invoices_dataset`、`finance.v_receivable_ledger_entries`、`finance.v_payable_ledger_entries` 已发布并授予 `ep_analyst_ro`，列签名同步给阶段 11，见裁定 A-18。
14. 四个主数据探针与历史成交提供者：`InvoiceReferenceCounter`、`FinanceReferenceCounter`（`crates/application/invoice/src/probe/` 与 `crates/application/finance/src/probe/`）与 `InvoiceSalesTradeHistoryProvider`、`InvoicePurchaseTradeHistoryProvider`，注册到阶段 5 提供的 `MasterReferenceCounterRegistry` 与 `TradeHistoryProviderRegistry`，见裁定 A-15。
15. 往来与预收预付的期初导入通道：用例 `crates/application/finance/src/usecase/import_opening_balances.rs` 与端点 `POST /api/v1/finance/opening-balances/actions/import`，见裁定 A-24 与第 4.12 节。
16. 两个模块全部路由的能力域码与动作类别常量已在 `crates/contract/invoice/src/capability.rs` 与 `crates/contract/finance/src/capability.rs` 声明，`xtask configdoc` 通过，见裁定 A-20。

---

### 2. crate 与进程归属

#### 2.1 新增 crate

| crate | 目录 | 进程 | 职责 |
|---|---|---|---|
| ep-contract-invoice | crates/contract/invoice | 装配进 core-server、job-worker | 发票申请、销项发票、进项发票、冲销登记的命令与查询 DTO、事件类型、能力域码与动作类别常量、供 clm、sales、procure、reporting 调用的六个 trait，含 `ReceiptInvoiceMatchQueryPort`、`PurchaseCreditNotePort`、`TaxRateOptionQuery` 与 F-10 `ReceiptPlanBillingQuery` |
| ep-domain-invoice | crates/domain/invoice | 同上 | 发票申请单与销项发票聚合、剩余可开比例值对象、税额勾稽规则、冲销互斥规则 |
| ep-app-invoice | crates/application/invoice | core-server 承载全部用例，job-worker 承载批量导入任务与影响规则装配 | 开具登记、冲销登记、批量导入、采购发票登记与三单匹配、进项红字发票登记、进项发票台账、`ReceiptPlanBillingQuery` 实现、`ContractTerminationSalesInvoiceImpactRule`、`InvoiceReferenceCounter` 与两个历史成交提供者；依赖 `ep-contract-costing::PurchaseInvoiceCaptureReversalBasisQuery`，为进项红字锁读当前 live 成本叶片并构造逐叶归因，不读 costing schema |
| ep-contract-finance | crates/contract/finance | 装配进 core-server、job-worker、portal-gateway 的调用方 | 应收应付预收预付台账查询 DTO，以及供 invoice、sales、procure、portal、reporting、crm、ledger 调用的 10 个 trait；方法数按第 5.9.1 节 ABI 快照机械核对，不以 trait 数推断 |
| ep-domain-finance | crates/domain/finance | 同上 | 台账条目聚合、核销分配算法、账龄分桶、可退上限、超量开票余额推进 |
| ep-app-finance | crates/application/finance | core-server 承载全部用例，job-worker 承载对账取数 | 到款、付款、退款、冲正、资金账户、对账视图 |

#### 2.2 改动的既有 crate

| crate | 改动 | 归属阶段 |
|---|---|---|
| ep-foundation | 增加 `TaxRate`（复用 `Rate` 的 newtype）、`IssueRatio`（复用 `Rate`）、`SettlementAmount`（复用 `Money` 且约束非负）三个 newtype；增加 `AccountingPeriodRef` 的 `is_deferred` 标记；必要性按基线第 12 节通则第六条在提交说明中逐项举证使用位 | 阶段 1 建立，本阶段追加 |
| ep-adapter-db-pg | 增加 `invoice` 与 `finance` 两个仓储子模块，按 schema 分文件，一个仓储只访问自己模块的 schema，共 36 个仓储实现 | 阶段 1 建立，本阶段追加 |
| ep-platform-sequence | 注册 9 个新的单据类型码：`INVA` 发票申请、`SINV` 销项发票、`IRVS` 冲销登记、`RCPT` 到款、`PAYM` 付款登记、`RFND` 退款与返款、`CDRV` 资金冲正、`OBST` 超量开票结清、`PINV` 进项发票；九码同时登记到 `docs/data-dictionary.md` 的单据类型码一节，由 `xtask configdoc --check-doc-type-codes` 校验全局唯一，见裁定 C-26 | 阶段 2 建立，本阶段追加类型码 |
| ep-platform-authz | 注册 14 个对象类型与 12 个动作 | 阶段 2 建立，本阶段追加注册项 |
| ep-testkit | 增加 `CashAccountFixture`、`InvoiceApplicationBuilder`、`SalesInvoiceBuilder`、`ReceiptBuilder`、`PaymentBuilder`、`RefundBuilder`、`ReceivableEntryProbe`、`ReconciliationProbe` 八个构造器与探针 | 阶段 1 建立，本阶段追加 |
| ep-datagen | 增加往来与发票子集 | 阶段 1 建立，本阶段追加 |
| ep-app-clm | 追加 `ContractTerminationReceiptPlanImpactRule`；只依赖 `ep-platform-impact` 与 `ep-contract-invoice::ReceiptPlanBillingQuery`，不直接读 invoice schema | 阶段 6 建立，本阶段完成 F-10 接线与验收 |
| ep-app-sales | 本阶段在 `src/usecase/confirm_delivery.rs` 接入真实 `UnbilledArPort::record_on_delivery`，在 `src/usecase/register_sales_return.rs` 接入 `InvoiceReversalStatusQuery` 与 `UnbilledArPort::record_on_sales_return`，在 `src/query/credit_exposure.rs` 用 `ReceivableExposureQuery` 组装三桶信用暴露；发布装配增量唯一落在 `apps/core-server/src/wiring/stage10_sales.rs`，契约/真库验收唯一落在 `crates/application/sales/tests/stage10_finance_contract.rs`，三处都不得读 `finance.`/`invoice.` schema 或接入占位实现 | 阶段 6 建立，本阶段拥有这三个接线增量及其验收 |
| ep-app-procure | 本阶段在 `src/usecase/post_goods_receipt.rs` 接入 `OverbillingMatchPort` 的候选/写入两腿，在 `src/usecase/post_purchase_return.rs` 接入 `ReceiptInvoiceMatchQueryPort` 与 `PurchaseCreditNotePort` 的已开票/混合退货分支，在 `src/usecase/submit_payment_request.rs` 接入 `PayableLedgerQuery` 的 `INVOICE_PAYMENT` 分支，并在 `src/writeback/payment_request.rs` 与 `src/writeback/purchase_order_invoicing.rs` 实现阶段 7 冻结的两个 owner writeback trait；core-server 接线唯一落在 `apps/core-server/src/wiring/stage10_procure.rs`，R-PROC-05 的快照接线落在 `apps/job-worker/src/wiring/stage10_procure.rs`，契约/真库验收唯一落在 `crates/application/procure/tests/stage10_invoice_finance_contract.rs` | 阶段 7 建立，本阶段拥有上述整条推迟与同批接线增量及其验收 |
| ep-app-portal | 本阶段在 `src/usecase/query_supplier_reconciliation.rs` 一处实现已登记采购发票、付款记录、应付未核销余额三个投影，只调 `SupplierStatementQuery` 与 `PayableLedgerQuery`，不读 invoice/finance schema；core-server 接线唯一落在 `apps/core-server/src/wiring/stage10_portal.rs`，契约/脱敏/数据范围验收唯一落在 `crates/application/portal/tests/stage10_supplier_reconciliation.rs` | 阶段 7 建立，本阶段拥有三个整条推迟端点的应用层增量及其验收 |
| apps/portal-gateway | 在 `src/routes/reconciliation.rs` 启用阶段 7 留给本阶段的三个 GET 路由，仍全部转发已冻结的 `portal.settlement_query.v1`，不新增 operation 通配符或数据库连接；路由集合与管道 allowlist 增量测试落在 `apps/portal-gateway/tests/stage10_reconciliation_routes.rs` | 阶段 7 建立，本阶段拥有三个路由的启用增量及其验收 |
| apps/core-server | `apps/core-server/src/wiring/` 对第 5.9 节 16 个自有 trait 逐一注入真实实现，该计数不含外部 owner trait；另按使用位注入 `StockValueSubledgerBalancePort`、`InventoryVariancePort`、`GrniSubledgerBalancePort`、`GrniEffectWritebackPort`、`PaymentRequestQueryPort`、`PaymentRequestWritebackPort`、`PayableReservationReadPort`、`PurchaseOrderInvoicingPort`、`SupplierInvoiceUploadWritebackPort`、`PurchaseInvoiceCaptureReversalBasisQuery`、`AccountingPeriodResolver`、`PostingPort` 与 `CrossModuleLockCoordinator`。装配测试按类型集合逐项相等而不把两类 trait 混成一个易漏的总数；路由注册第 5 节唯一清单的 49 个端点，两个 wiring 目录中不出现任何 Noop、Stub、Fake、Dummy 前缀的发布占位类型 | 本阶段追加 |
| apps/job-worker | 注册批量导入任务处理器，并把 `CLM_TERM_RECEIPT_PLAN` 与 `CLM_TERM_SALES_INVOICE` 两个真实规则注册进既有 `ImpactRegistry`，注入真实 `ReceiptPlanBillingQuery`，使累计注册数由 4 增至 6；另为阶段 7 的 R-PROC-05 注入真实 `OverbillingMatchPort`，只调用其 `settlement_segments_by_receipt_lines(&dyn SnapshotCtx,...)` 快照方法，不读取 finance schema。不得为 `clm.contract.terminated.v1` 新增第二个消费者。按裁定 A-06 本阶段不实现也不注册任何 `ReconCheck`，原定的 `FIN_CROSS_MODULE_LINK` 是纯存在性项，其 `category` 取值 `CROSS_MODULE_LINK` 已随该类别整体撤销，本项不再存在。跨模块单目标引用的存在性由基线第 3.3 节的复合真实外键强制；子账条目与其来源凭证的期间一致由 `ep-contract-ledger::AccountingPeriodResolver::resolve` 在同一 `&mut dyn Tx` 内的记忆化保证，见第 5 节相应段落 | 本阶段追加 |

本阶段不新增进程、不新增 schema、不新增模块码、不新增错误分类。`ep-domain-finance` 与 `ep-domain-invoice` 不依赖对方；两个模块之间只由 `ep-app-invoice` 依赖 `ep-contract-finance`，`ep-app-finance` 不依赖 `ep-contract-invoice`。按裁定 G-01，`ep-app-finance` 另依赖 `ep-contract-inventory` 与 `ep-contract-procure` 两个契约，用于注入两个子账余额端口的实现，其中对 `ep-contract-procure` 的依赖先于本裁定已存在（见第 6.1 节 register_payment 行的付款申请已付金额回写），本裁定只新增对 `ep-contract-inventory` 一条边。进项成本血缘只追加 `ep-app-invoice → ep-contract-costing`，由 `PurchaseInvoiceCaptureReversalBasisQuery` 读取/锁住 live 叶片，绝不产生 app→app 或 invoice→costing schema SQL。F-10 只追加 `ep-app-clm → ep-contract-invoice` 与 `ep-app-clm/ep-app-invoice → ep-platform-impact` 三条允许边；任何 app 都不依赖另一个 app，也不跨 schema 直读。上述边均为 app 到 contract/platform，落在基线第 1.3 节允许项内，承接方为 `xtask archcheck` 的层位判定，本阶段不另立任何按 crate 逐项比对的期望依赖清单。

---

### 3. 数据库变更

全部表遵守共享基线第 3 节与第 4 节。以下每张表只列出公共列之外的专有列，公共列按基线第 4 节的九列固定顺序在前。仅追加表不带 `row_version`、`updated_at`、`updated_by`；只有存在真实反向父链的表才另带 `reverses_id` 并建立下文明确的同法人复合自外键，号码登记、分摊事实等没有反向父链的仅追加表不得为了公共形状增加恒空列。

本阶段所有单目标引用均建立数据库真实外键。双方带法人列时统一使用 `(legal_entity_id,ref_id) -> target(legal_entity_id,id) ON DELETE RESTRICT`，并在目标表显式建立 `UNIQUE (legal_entity_id,id)` 候选键；业务用户列指向 `platform_authz.user_legal_entity_grants(legal_entity_id,user_id)`；`reauth_ref` 以真实单列外键指向 `platform_core.reauth_challenges(id)`，写入事务另校验证据用户、法人、设备、摘要和有效期。只有带判别列的封闭多态引用、精确命名的 `approval_ref`，以及已封闭的 `linked_purchase_return_id` 例外不建外键。指向后建目标的外键由第 3.6 节精确追补迁移建立，禁止永久保留“逻辑引用”。

#### 3.1 invoice schema

> **F-50 当前结构说明：** 第 3.1.3—3.1.8 节全部是可直接建表的当前规范，与 `docs/data-dictionary/invoice.md` 及 F-50 第 6—7 节一致：号码只存中央登记表，蓝票与冲销都有独立逐行表，冲销行有两套真实来源 FK，头表无税率，且不存在 `reversed_*` 权威缓存、`is_credit_note` 或单次冲销唯一约束。

##### 3.1.0 F-50 当前进项发票/红字物理覆盖（规范性）

以下是本节采购相关建表的当前规范摘要，并由后文逐表定义完整展开：

- `invoice.purchase_invoices` 保留公共列、单据号、供应商、可空采购订单、发票日期、记账日期、期间、三项只读头汇总、`cost_kind`、中央 `invoice_number_registry_id` 与生成凭证引用；状态只能是 `REGISTERED/PARTIALLY_REVERSED/REVERSED`。头表没有税率、号码副本、`is_credit_note`、`reversed_by_id` 或任何 `reversed_*` 权威累计。
- `invoice.purchase_invoice_lines` 固定含 `purchase_invoice_id,line_no,purchase_order_id,purchase_order_line_id,goods_receipt_id?,goods_receipt_line_id?,cost_kind,item_id?,quantity,net_unit_price,tax_rate,net_amount,tax_amount,gross_amount` 及四个 `NOT NULL DEFAULT 0` 服务端结果 `accrual_reversal_amount,price_variance_in_stock_amount,price_variance_released_amount,overbilling_amount`；不得另建未拆分的总价差列或超量布尔标志作为权威值。同一头全部行 `cost_kind` 必须同值；同头行号唯一，并建立 `(legal_entity_id,purchase_invoice_id,id)` 候选唯一键。金额、税率、头行汇总与价税等式逐字按 F-50 第 6.1—6.4 节。
- `invoice.invoice_reversals` 固定使用 `direction,reversal_kind,source_sales_invoice_id,source_purchase_invoice_id,linked_purchase_return_id,invoice_number_registry_id` 与三项只读汇总。两种原票 id 按方向恰一非空；`linked_purchase_return_id uuid NULL` 只有 `INPUT+RED_LETTER` 可非空，采购退货用例调用时必填，独立进项更正、全部销项与 `VOID` 必须为空。该字段不建一条会制造循环建表依赖的伪 FK；`V20261019090930__procure_add_invoice_foreign_keys.sql` 以退货侧与红字侧都可触发的双向 `DEFERRABLE INITIALLY DEFERRED` 效果图在数据库提交点强制真实归属，invoice owner 仍在同一事务验证所指退货同法人、同供应商，物料类核对已开票退货段，直接费用/直运类核对原成本归集链与累计可冲上限。
- `invoice.invoice_reversal_lines` 使用两组原票/原行 id、`source_effect_seq,quantity_effect_kind,pricing_effect_kind,quantity,tax_rate,net_amount,tax_amount,gross_amount`。活动来源组全填、非活动组全空由 NULL-safe CHECK 强制，复合 FK 使用默认 `MATCH SIMPLE`，并由延迟触发器保证行所指原票与头逐字相同；不存在单次冲销唯一约束。`source_effect_seq NOT NULL` 在每条来源行内从 1 无缺口递增，两套 `(legal_entity_id,source_*_invoice_line_id,source_effect_seq)` 唯一约束保证活动方向必被覆盖。另一延迟约束触发器按序重放同一来源行：累计数量/金额不得超原行，非末次 `ORIGINAL_UNIT_PRICE` 必须等于标准重算值；只有恰好耗尽数量、此前无 `ADJUSTED` 且金额等于原行定标值减此前原价冲销定标值时才吸收末次尾差。跳号、非末次偏差或已有 `ADJUSTED` 后伪装尾差均由数据库拒绝。

`linked_purchase_return_id IS NOT NULL` 且原票为物料类时，凭证来源固定为 `PURCHASE_INVOICE_LINKED_RETURN_REVERSED`，不得出现库存计量项；对应物理退货只由采购模块以 `PURCHASE_RETURN_INVENTORY` 生成一张凭证。数量红字先逐父重开 GRNI，物理退货随后逐条等量等额消费，二者必须处于同一写事务。原票为直接费用/直运类时来源固定为 `PURCHASE_INVOICE_DIRECT_EXPENSE_REVERSED`，只冲应付、进项税与原成本，不写 GRNI 或本方库存，也不生成物理库存凭证。

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
| customer_id | uuid | 否 | 与法人组成复合外键指向 `mdm.customers(legal_entity_id,id)` |
| contract_id | uuid | 否 | 与法人组成复合外键指向 `clm.contracts(legal_entity_id,id)` |
| application_date | date | 否 | |
| issue_content | text | 否 | 长度上限 500 |
| issue_ratio | numeric(9,6) | 否 | `ck_invoice_applications_ratio_positive` 大于 0 |
| remaining_ratio | numeric(9,6) | 否 | `ck_invoice_applications_remaining_range` 大于等于 0 且小于等于 `issue_ratio` |
| contract_amount | numeric(18,2) | 否 | 提交时从 clm 快照带出并固化，避免合同变更后比例基数漂移 |
| application_amount | numeric(18,2) | 否 | 等于 `round(issue_ratio * contract_amount, 2)` |
| expected_receipt_date | date | 否 | `ck_invoice_applications_expected_date` 不早于 `application_date` |
| approval_ref | uuid | 是 | 指向 platform_flow 的审批实例 |
| remark | text | 是 | 长度上限 2000 |

另有两张关联表，按共享基线第 3.2 节的 `<a>_<b>_links` 命名：`invoice.invoice_application_sales_order_links`，列为 `invoice_application_id`、`sales_order_id`，两列分别以同法人复合外键指向申请单与 `sales.sales_orders`；`invoice.invoice_application_receipt_plan_links`，列为 `invoice_application_id`、`receipt_plan_line_id`，两列分别以同法人复合外键指向申请单与 `clm.contract_payment_schedules`。全部取 `ON DELETE RESTRICT`。

##### 3.1.3 invoice.sales_invoices（销项发票）

| 列 | 类型 | 可空 | 约束与说明 |
|---|---|---|---|
| doc_no | text | 否 | `ux_sales_invoices_legal_entity_id_doc_no`；类型码 SINV |
| status | text | 否 | `ck_sales_invoices_status` 取值 `ISSUED/PARTIALLY_RED_REVERSED/VOIDED/RED_REVERSED` |
| invoice_application_id | uuid | 否 | 与 `legal_entity_id` 组成复合 FK 指向申请单，`ON DELETE RESTRICT` |
| customer_id | uuid | 否 | 与法人组成复合外键指向 `mdm.customers(legal_entity_id,id)` |
| contract_id | uuid | 否 | 与法人组成复合外键指向 `clm.contracts(legal_entity_id,id)` |
| invoice_number_registry_id | uuid | 否 | 与 `legal_entity_id` 组成复合 FK 指向中央号码登记；法人内唯一，号码不在本表复制 |
| issue_date | date | 否 | |
| posting_date | date | 否 | 记账日期，取 `issue_date` |
| accounting_period_id | uuid | 否 | 由 ledger 端口在业务事务内解析；与法人组成复合外键指向 `ledger.accounting_periods(legal_entity_id,id)` |
| deferred_from_period_id | uuid | 是 | 非空表示该事件发生过顺延；与法人组成复合外键指向 `ledger.accounting_periods(legal_entity_id,id)` |
| issued_ratio | numeric(9,6) | 否 | 大于 0 |
| issue_content | text | 否 | 长度上限 500 |
| net_amount | numeric(18,2) | 否 | 服务端只读行汇总，大于 0 |
| tax_amount | numeric(18,2) | 否 | 服务端只读行汇总，大于等于 0 |
| gross_amount | numeric(18,2) | 否 | 服务端只读行汇总，大于 0 且等于前两项之和 |
| receivable_entry_id | uuid | 否 | 与法人组成复合外键指向 `finance.receivable_entries(legal_entity_id,id)`，由 `V20261019092430__invoice_add_finance_foreign_keys.sql` 建立，且 `DEFERRABLE INITIALLY DEFERRED` |
| voucher_id | uuid | 否 | 与法人组成复合外键指向 `ledger.vouchers(legal_entity_id,id)` |
| import_batch_id | uuid | 是 | 非空表示由批量导入产生 |
| reauth_ref | uuid | 否 | 真实单列外键指向 `platform_core.reauth_challenges(id)`；事务校验用户、法人、设备、摘要与有效期 |
| approval_ref | uuid | 否 | 审批实例引用 |

本表建立 `(legal_entity_id,id)` 候选唯一键供冲销头复合 FK 使用；`invoice_number_registry_id` 另建法人内唯一约束。头表没有 `invoice_kind/invoice_medium/number_scheme/invoice_code/invoice_no/tax_rate` 或 `reversed_*` 权威列。三项头金额由行汇总，`gross_amount=net_amount+tax_amount` 做数据库 CHECK；事务提交前的受控写入口再重读至少一行、三项行合计、号码 owner 与应收/凭证引用，任一不一致整笔回滚。

##### 3.1.3a invoice.sales_invoice_lines（销项发票行）

| 列 | 类型 | 可空 | 约束与说明 |
|---|---|---|---|
| sales_invoice_id | uuid | 否 | 与 `legal_entity_id` 组成复合 FK 指向销项头，`ON DELETE RESTRICT` |
| line_no | int | 否 | 大于 0；同一发票内唯一 |
| sales_order_id | uuid | 否 | 与法人组成复合外键指向 `sales.sales_orders(legal_entity_id,id)` |
| sales_order_line_id | uuid | 否 | 与法人、销售订单组成复合外键指向 `sales.sales_order_lines`，并保证属于同一订单 |
| item_kind | text | 否 | `PRODUCT/MATERIAL` |
| item_id | uuid | 否 | 与 `item_kind` 组成 `PRODUCT/MATERIAL` 封闭多态引用，由 invoice owner 校验同法人目标；不建伪外键 |
| uom_code | text | 否 | 长度 1 至 64 |
| quantity | numeric(18,6) | 否 | 大于 0 |
| net_unit_price | numeric(18,6) | 否 | 大于 0 |
| tax_rate | numeric(9,6) | 否 | 0 至 1 闭区间 |
| net_amount | numeric(18,2) | 否 | 大于 0 |
| tax_amount | numeric(18,2) | 否 | 非负；按 half-up 与 0.02 上限容差校验 |
| gross_amount | numeric(18,2) | 否 | 大于 0 且精确等于 `net_amount+tax_amount` |

建立 `UNIQUE(sales_invoice_id,line_no)` 与 `(legal_entity_id,sales_invoice_id,id)` 候选唯一键。每张销项发票至少一行；税率只在行上，同票允许多税率。红冲后的剩余数量、净额、税额与价税合计由统一冲销行按 `source_effect_seq` 聚合，不在本表保存 `reversed_*` 缓存。

##### 3.1.3b invoice.invoice_number_registry（中央法定号码登记，仅追加）

| 列 | 类型 | 可空 | 约束与说明 |
|---|---|---|---|
| invoice_medium | text | 否 | CHECK 仅 `ELECTRONIC/PAPER` |
| number_scheme | text | 否 | CHECK 仅 `UNIFIED_20/LEGACY_CODE_NUMBER` |
| invoice_code | text | 条件 | `UNIFIED_20` 必空；旧制必填且为 10 或 12 位 ASCII 数字 |
| invoice_no | text | 否 | `UNIFIED_20` 为 20 位 ASCII 数字；旧制为 8 位 ASCII 数字 |
| identifier_key | text | 否 | `GENERATED ALWAYS AS (...) STORED`，确定格式见 F-50 第 7.2 节 |
| owner_type | text | 否 | CHECK 仅 `SALES_BLUE/PURCHASE_BLUE/OUTPUT_RED/INPUT_RED` |
| owner_id | uuid | 否 | 唯一业务头 id |

所有关键列和生成结果都显式 `NOT NULL`。制式 CHECK 逐项使用 `IS NULL/IS NOT NULL`，不得让 SQL `UNKNOWN` 放行；生成表达式只读本行制式、代码与号码。固定唯一键为 `(legal_entity_id,identifier_key)`、`(legal_entity_id,owner_type,owner_id)`，另建 `(legal_entity_id,id)` 候选键。蓝票或红票头以 `(legal_entity_id,invoice_number_registry_id)` 复合 FK 引用；`DEFERRABLE INITIALLY DEFERRED` 约束触发器在提交前双向核对 owner 类型、owner id 与头引用。该表按 APPEND_ONLY 登记并启用/强制法人 RLS；`VOID` 不插行。

##### 3.1.4 invoice.invoice_reversals（销项作废及销项/进项红字冲销登记单）

| 列 | 类型 | 可空 | 约束与说明 |
|---|---|---|---|
| doc_no | text | 否 | `ux_invoice_reversals_legal_entity_id_doc_no`；类型码 IRVS |
| status | text | 否 | `ck_invoice_reversals_status` 取值固定为 REGISTERED |
| direction | text | 否 | `ck_invoice_reversals_direction` 取值 OUTPUT、INPUT |
| reversal_kind | text | 否 | `ck_invoice_reversals_kind` 取值 VOID、RED_LETTER；INPUT+VOID 不开放 |
| source_sales_invoice_id | uuid | 条件 | OUTPUT 必填、INPUT 必空；与 `legal_entity_id` 组成复合真实 FK |
| source_purchase_invoice_id | uuid | 条件 | INPUT 必填、OUTPUT 必空；与 `legal_entity_id` 组成复合真实 FK |
| linked_purchase_return_id | uuid | 是 | 仅 INPUT+RED_LETTER 可非空；采购退货内部端口调用时必填，独立更正必空；不建循环伪 FK，由第 3.6 节第 19090930 号迁移的双向延迟效果图在数据库提交点约束 |
| register_date | date | 否 | |
| posting_date | date | 否 | |
| accounting_period_id | uuid | 否 | 与法人组成复合外键指向 `ledger.accounting_periods(legal_entity_id,id)` |
| deferred_from_period_id | uuid | 是 | 与法人组成复合外键指向 `ledger.accounting_periods(legal_entity_id,id)` |
| invoice_number_registry_id | uuid | 条件 | RED_LETTER 必填、VOID 必空；同法人复合 FK 指向中央号码登记 |
| net_amount | numeric(18,2) | 否 | 只读行汇总；非负 |
| tax_amount | numeric(18,2) | 否 | 只读行汇总；非负 |
| gross_amount | numeric(18,2) | 否 | 只读行汇总；`gross_amount = net_amount + tax_amount` 且大于零 |
| reason | text | 否 | 长度上限 2000 |
| voucher_id | uuid | 否 | 与法人组成复合外键指向 `ledger.vouchers(legal_entity_id,id)` |
| overbilling_entry_id | uuid | 是 | 方向为 INPUT 且本次冲销用于结清超量开票时非空；由 `V20261019092430__invoice_add_finance_foreign_keys.sql` 补建指向 `finance.overbilling_entries(legal_entity_id,id)` 的真实复合外键 |
| reauth_ref | uuid | 是 | 真实单列外键指向 `platform_core.reauth_challenges(id)`，事务校验证据归属 |
| approval_ref | uuid | 是 | |

`source_sales_invoice_id` 与 `source_purchase_invoice_id` 用逐项 `IS NULL/IS NOT NULL` 的 XOR CHECK，不写可能返回 UNKNOWN 的裸比较。`linked_purchase_return_id` 的 CHECK 同样逐项判空；invoice owner 在统一锁序的同一事务内校验退货同法人、同供应商与红字行覆盖范围，第 19090930 号迁移的双向延迟效果图再以数据库约束封住绕过应用的直写。红字允许多次部分冲销，不建立原票到冲销头的一对一唯一约束；同一采购退货可以按不同原票各有一张红字，但同一 `(linked_purchase_return_id,source_purchase_invoice_id)` 组只能有一张且必须完整覆盖该组本次退货分段。销项 VOID 仍由锁后状态守卫保证全额一次。累计权威来源只有下表冲销行聚合。

U-D-19 的现行路径固定如下：`INPUT + VOID` 是非法组合，提交时返回已登记的 `INVOICE.INVOICE_REVERSAL.EFFECT_KIND_INVALID`，数据库 CHECK 同样拒绝。若供应商已自行作废本方已登记的进项发票但尚未提供红字票，供应商必须重新开具正确蓝票，或提供一张可登记且引用原发票的合法红字票；在取得其中一种合法更正凭据并完成登记前，原进项发票保持已登记状态，不自动冲销。本方不得用内部作废、更正凭证、资金冲正或采购退货伪装承接该事实。

##### 3.1.4a invoice.invoice_reversal_lines（统一冲销行）

| 列 | 类型 | 可空 | 约束与说明 |
|---|---|---|---|
| invoice_reversal_id | uuid | 否 | 同法人复合 FK 指向冲销头，`ON DELETE RESTRICT` |
| source_effect_seq | int | 否 | 活动来源行内从 1 无缺口递增 |
| source_sales_invoice_id | uuid | 条件 | 与对应 source line id 同空同非空 |
| source_sales_invoice_line_id | uuid | 条件 | OUTPUT 必填、INPUT 必空；三列默认 `MATCH SIMPLE` 复合 FK 指向销项原行候选键 |
| source_purchase_invoice_id | uuid | 条件 | 与对应 source line id 同空同非空 |
| source_purchase_invoice_line_id | uuid | 条件 | INPUT 必填、OUTPUT 必空；三列默认 `MATCH SIMPLE` 复合 FK 指向进项原行候选键 |
| quantity_effect_kind | text | 否 | REDUCE、NONE |
| pricing_effect_kind | text | 否 | ORIGINAL_UNIT_PRICE、ADJUSTED |
| quantity | numeric(18,6) | 否 | REDUCE 大于零；NONE 精确为零 |
| tax_rate | numeric(9,6) | 否 | 必须等于活动原行税率 |
| net_amount | numeric(18,2) | 否 | 非负 |
| tax_amount | numeric(18,2) | 否 | 非负 |
| gross_amount | numeric(18,2) | 否 | 大于零且精确等于前两项之和 |

两组来源以 NULL-safe all-or-none/XOR CHECK 强制，延迟头行一致性触发器拒绝错票挂行。分别建立 `(legal_entity_id,source_sales_invoice_line_id,source_effect_seq)` 与 `(legal_entity_id,source_purchase_invoice_line_id,source_effect_seq)` 唯一约束。第二个 `DEFERRABLE INITIALLY DEFERRED` 约束触发器锁定活动原行并按序重放：要求序号恰为 `1..n`、累计数量/净额/税额/价税合计均不超原行；非末次 `ORIGINAL_UNIT_PRICE` 必须按原单价与唯一税额规则定标。只有恰好耗尽数量、此前从未出现 ADJUSTED 且金额等于原行定标值减此前全部 ORIGINAL_UNIT_PRICE 定标值时，才允许当前行吸收可重算末次尾差。NULL、跳号、重复、非末次偏差与 ADJUSTED 后伪装末次尾差全部在 PostgreSQL 层拒绝。

##### 3.1.5 invoice.invoice_receipt_plan_links（销项发票与合同收款计划行的勾稽）

本表是逐期净已开金额的唯一权威，只追加、不更新历史分摊。公共列外的完整业务列为：

| 列 | 类型 | 可空 | 约束与说明 |
|---|---|---|---|
| contract_id | uuid | 否 | 发票所属合同快照；与法人组成复合外键指向 `clm.contracts`，且与正向行的销项发票一致 |
| receipt_plan_line_id | uuid | 否 | 与法人、合同组成复合外键指向 `clm.contract_payment_schedules`，不得只校验 UUID 存在 |
| receipt_plan_period_no | int | 否 | 开票时经 clm owner 契约校验后固化，大于 0；反向行必须复制根行值 |
| sales_invoice_id | uuid | 否 | 与 `legal_entity_id` 组成复合 FK 指向原销项发票 |
| allocation_kind | text | 否 | `ISSUE/VOID/RED_LETTER`；方向只由本列显式表达，不另设布尔方向列 |
| invoice_reversal_id | uuid | 条件 | `ISSUE` 必空；`VOID/RED_LETTER` 必填，与法人组成复合 FK 指向同一原票的冲销头 |
| root_allocation_id | uuid | 否 | `ISSUE` 根行取自身 id；反向行指向被反向的 `ISSUE` 根行 |
| linked_net_amount | numeric(18,2) | 否 | 严格大于 0 |
| linked_gross_amount | numeric(18,2) | 否 | 严格大于 0，且不小于净额 |

NULL-safe CHECK 固定根/反向行形状。为使根不可能跨原票、合同或期次挂接，本表建立候选键 `UNIQUE(legal_entity_id,contract_id,sales_invoice_id,receipt_plan_line_id,receipt_plan_period_no,id)`，并以具名真实自外键 `fk_invoice_receipt_plan_links_root_allocation (legal_entity_id,contract_id,sales_invoice_id,receipt_plan_line_id,receipt_plan_period_no,root_allocation_id) REFERENCES invoice.invoice_receipt_plan_links(legal_entity_id,contract_id,sales_invoice_id,receipt_plan_line_id,receipt_plan_period_no,id) ON DELETE RESTRICT DEFERRABLE INITIALLY DEFERRED` 固定根链；不得以单列 `root_allocation_id` FK 或应用校验代替。invoice owner 在 proof 前只预生成根 id，取得 proof 后首根用一条 `INSERT` 同时写 `id=root_allocation_id`，绝不先插 NULL 再回填；延迟自 FK 使空表首根可以在 COMMIT 合法自引用。`DEFERRABLE INITIALLY DEFERRED` 约束触发器校验同法人、同合同、同原票、同期次，并强制每个根行的 `VOID+RED_LETTER` 累计反向净额和价税合计均不超正向分摊。发票开具按各期追加 `ISSUE`；作废按原分摊全额追加 `VOID`；分次红冲按本次实际回滚分摊追加 `RED_LETTER`，末次吸收已批准舍入尾差。

`ReceiptPlanBillingQuery::billing_by_period(tx, ctx, contract_id) -> Result<BTreeMap<i32, Money>, AppError>` 只读本表，按 `receipt_plan_period_no` 返回 `sum(ISSUE.linked_gross_amount)-sum((VOID|RED_LETTER).linked_gross_amount)`。缺席键由调用方视为零，任一返回值小于零均是勾稽故障而非业务状态。`clm` 不保存已开金额副本；开票与冲销都不写 `clm.contract_payment_schedules`，也不投递已开金额回写 Outbox。

收款计划行的取数唯一经 `ep_contract_clm::ContractPaymentScheduleQuery::schedules(tx, ctx, contract_id)`，按裁定 C-20 该查询由阶段 6 提供，收付款计划行的唯一表为 `clm.contract_payment_schedules`，`ep_contract_finance::ReceivablePlanPort` 已撤销，本阶段不派生第二套收付款计划。

##### 3.1.6 invoice.invoice_import_batches（批量导入批次）

列为 `doc_no`、`status`（`ck_invoice_import_batches_status` 取值 PENDING、RUNNING、SUCCEEDED、PARTIALLY_FAILED、FAILED）、`total_rows int`、`succeeded_rows int`、`failed_rows int`、`file_object_id uuid` 与 `result_object_id uuid`（均以同法人复合外键指向 `platform_file.attachment_objects(legal_entity_id,id)`）、`started_at`、`finished_at`、`reauth_ref`（真实单列外键指向 `platform_core.reauth_challenges(id)` 并做事务归属校验）、`approval_ref`（审批实例白名单），加公共列。
##### 3.1.7 invoice.purchase_invoices（进项发票，单据类，类型码 PINV）

本表按裁定 A-10 归 invoice 模块，采购阶段不建表也不写台账。

| 列 | 类型 | 可空 | 约束与说明 |
|---|---|---|---|
| doc_no | text | 否 | `ux_purchase_invoices_legal_entity_id_doc_no`；类型码 PINV |
| status | text | 否 | `ck_purchase_invoices_status` 取值 REGISTERED、PARTIALLY_REVERSED、REVERSED |
| supplier_id | uuid | 否 | 与法人组成复合外键指向 `mdm.suppliers(legal_entity_id,id)` |
| purchase_order_id | uuid | 是 | 与法人组成复合外键指向 `procure.purchase_orders(legal_entity_id,id)`；行只涉及一张采购订单时取该 id，跨订单时为空，逐行订单引用仍必填 |
| supplier_invoice_upload_id | uuid | 是 | 门户受理来源以同法人复合外键指向 `portal.supplier_invoice_uploads(legal_entity_id,id)`；手工登记为空 |
| invoice_date | date | 否 | |
| posting_date | date | 否 | |
| accounting_period_id | uuid | 否 | 由 ledger 端口在业务事务内解析；同法人复合外键指向 `ledger.accounting_periods` |
| deferred_from_period_id | uuid | 是 | 同法人复合外键指向 `ledger.accounting_periods` |
| cost_kind | text | 否 | 由全部同值行推导，取 INVENTORY_TYPE、DIRECT_EXPENSE_TYPE；混合值拒绝 |
| invoice_number_registry_id | uuid | 否 | 同法人复合 FK 指向 `invoice.invoice_number_registry`；号码不在本表复制 |
| net_amount | numeric(18,2) | 否 | 只读行汇总；大于零 |
| tax_amount | numeric(18,2) | 否 | 只读行汇总；非负 |
| gross_amount | numeric(18,2) | 否 | 只读行汇总；大于零且等于前两项之和 |
| payable_entry_id | uuid | 否 | 与法人组成复合外键指向 `finance.payable_entries(legal_entity_id,id)`，由 `V20261019092430__invoice_add_finance_foreign_keys.sql` 建立，且 `DEFERRABLE INITIALLY DEFERRED` |
| voucher_id | uuid | 否 | 与法人组成复合外键指向 `ledger.vouchers(legal_entity_id,id)` |

本表没有头税率、供应商号码副本、`is_credit_note`、`reversed_by_id` 或 `reversed_*` 权威累计。号码唯一性只由中央登记表的非空生成键和 owner 约束承担。自动核销额 `advance_auto_applied_amount` 是从 finance 效果链重读的响应/事件只读结果，不作为客户端输入或第二个可写累计。索引为 `ix_purchase_invoices_legal_entity_id_created_at`、`ix_purchase_invoices_legal_entity_id_purchase_order_id`、`ix_purchase_invoices_legal_entity_id_posting_date`。

##### 3.1.8 invoice.purchase_invoice_lines（进项发票行）

| 列 | 类型 | 可空 | 约束与说明 |
|---|---|---|---|
| purchase_invoice_id | uuid | 否 | 同法人复合外键，`ON DELETE RESTRICT` |
| line_no | int | 否 | `ux_purchase_invoice_lines_invoice_id_line_no` |
| purchase_order_id | uuid | 否 | 与法人组成复合外键指向 `procure.purchase_orders(legal_entity_id,id)` |
| purchase_order_line_id | uuid | 否 | 与法人、采购订单组成复合外键指向 `procure.purchase_order_lines` |
| goods_receipt_id | uuid | 是 | 与收货行 id 同空或同非空；非空时以同法人复合外键指向 `procure.goods_receipts`，直接费用类必须为空 |
| goods_receipt_line_id | uuid | 是 | 与收货头 id 同空或同非空；非空时以法人、收货头组成复合外键指向 `procure.goods_receipt_lines`，直接费用类必须为空 |
| cost_kind | text | 否 | INVENTORY_TYPE、DIRECT_EXPENSE_TYPE；同一头全部行必须同值 |
| item_id | uuid | 是 | 物料类必填并以同法人复合外键指向 `mdm.materials(legal_entity_id,id)`；直接费用类可空 |
| quantity | numeric(18,6) | 否 | 大于零 |
| net_unit_price | numeric(18,6) | 否 | 大于零 |
| tax_rate | numeric(9,6) | 否 | 0 到 1 闭区间 |
| net_amount | numeric(18,2) | 否 | 大于零 |
| tax_amount | numeric(18,2) | 否 | 非负；按 half-up 与 0.02 上限容差校验 |
| gross_amount | numeric(18,2) | 否 | 大于零且精确等于 `net_amount + tax_amount` |
| accrual_reversal_amount | numeric(18,2) | 否 | 默认 0；暂估回冲金额，由 `GrniEffectWritebackPort::decrease_for_purchase_invoice` 的逐行结果服务端汇总，不由库存价差算法反推 |
| price_variance_in_stock_amount | numeric(18,2) | 否 | 默认 0；有符号在库价差，只由库存端口返回 |
| price_variance_released_amount | numeric(18,2) | 否 | 默认 0；有符号已出库价差，只由库存端口返回 |
| overbilling_amount | numeric(18,2) | 否 | 默认 0；非负超量未匹配净额，非零时生成 `finance.overbilling_entries` |

最后四项全是服务端只读结果，客户端、插件与 Excel 均不得提交；直接费用行四项全为零。物料已匹配段满足 `matched_invoice_net_amount - accrual_reversal_amount = price_variance_in_stock_amount + price_variance_released_amount`，未匹配净额只进入 `overbilling_amount`。同头行号唯一，另建 `(legal_entity_id,purchase_invoice_id,id)` 候选唯一键供冲销行真实复合 FK 使用。索引 `ix_purchase_invoice_lines_legal_entity_id_goods_receipt_line_id` 支撑 `ReceiptInvoiceMatchQueryPort` 按收货行判定是否已开票。

##### 3.1.9 附件关联表

`invoice.invoice_application_attachments`、`invoice.sales_invoice_attachments`、`invoice.purchase_invoice_attachments`、`invoice.invoice_reversal_attachments`，列均为 `owner_id`、`attachment_object_id`、`purpose`、`sort_no` 加公共列；`owner_id` 与法人组成复合外键分别指向 `invoice_applications`、`sales_invoices`、`purchase_invoices`、`invoice_reversals`，`attachment_object_id` 与法人组成复合外键指向 `platform_file.attachment_objects(legal_entity_id,id)`，均取 `ON DELETE RESTRICT`。四表均以 `(legal_entity_id,owner_id,attachment_object_id)` 唯一并以 `(legal_entity_id,owner_id,sort_no)` 唯一；`purchase_invoice_attachments` 必须由既有 `V20261019091200__invoice_create_attachment_link_tables.sql` 同批创建，Stage 14 的 `invoice.purchase_invoice_bundle` 静态投影只认此具名关系，不得省略附件或复用其他 owner 的关联表。

#### 3.2 finance schema

> **F-50 当前结构说明：** 第 3.2.3—3.2.14 节是可直接建表的现行规范。AR/AP 条目用 `ORIGINAL/REVERSAL`，四张关系表用显式 `APPLY/RELEASE`、根/父链与资金来源；`settled_amount/open_amount` 只是事务内同步投影，经营读取只暴露 `effective_open/advance_open`。

##### 3.2.1 finance.aging_bucket_definitions（账龄分档配置）

列为 `code`、`display_name`、`from_days int`、`to_days int`（`to_days` 为空表示最后一档开区间）、`sort_no`、`is_active`、`deactivated_at`，加公共列。约束 `ck_aging_bucket_definitions_range` 表达 `from_days >= 0` 且 `to_days` 为空或大于 `from_days`。

本表按裁定 C-08 是临时表：账龄分档的唯一出处为阶段 11 的 `reporting.aging_bucket_profiles` 与 `reporting.aging_bucket_lines`。本阶段只由第 3.6 节的种子迁移出厂预置一套六档，不提供任何维护端点，也不建配置发布对象与 ConfigItemApplier，按法人分套的形态随阶段 11 一并交付。迁数据与删表两个迁移文件按裁定通则第五条一律放在 `db/migrations/reporting/` 目录下，文件名为 `V20261020091600__reporting_backfill_migrate_aging_buckets_from_finance.sql` 与 `V20261020091700__reporting_drop_finance_aging_bucket_definitions.sql`，两个文件均由阶段 11 提供，本阶段一个都不提供，也不在 `db/migrations/finance/` 下建删表文件。本阶段的账龄查询在阶段 11 到位后改经 `ep_contract_reporting::AgingBucketQuery::buckets(tx, ctx, legal_entity_id, ledger_side)`，不保留第二套口径。

##### 3.2.2 finance.cash_accounts（资金账户档案）

| 列 | 类型 | 可空 | 约束与说明 |
|---|---|---|---|
| code | text | 否 | `ux_cash_accounts_legal_entity_id_code` |
| account_name | text | 否 | `ux_cash_accounts_legal_entity_id_account_name`；长度上限 200 |
| account_type | text | 否 | `ck_cash_accounts_type` 取值 BANK、CASH；建档后不可修改 |
| ledger_account_id | uuid | 否 | 与法人组成复合外键指向 `ledger.accounts(legal_entity_id,id)`；建档后不可修改 |
| bank_name_enc | bytea | 是 | BANK 时必填；逻辑字段 `bank_name` 按法人密钥域加密，不保留同名明文列 |
| bank_name_key_ref | text | 是 | 与 `bank_name_enc` 同生共死，记录密钥标识与版本 |
| bank_account_no_enc | bytea | 是 | BANK 时必填；按规格第 7.8 章法人密钥域字段级加密存储，不保留同名明文列 |
| bank_account_no_key_ref | text | 是 | 记录密钥标识与版本，与 `bank_account_no_enc` 同生共死 |
| bank_account_no_tail | text | 是 | 明文后 4 位，供列表与导出脱敏展示 |
| bank_account_no_bidx | bytea | 是 | 固定 32 字节盲索引，写入与查询从同一登记取 scope=30，取值为 `derive_blind_key(legal_entity_id, 'finance.cash_accounts.bank_account_no@30', plaintext)` 返回的完整 `BlindIndex([u8; 32])`，裸 FQN 拒绝；`ck_cash_accounts_bank_account_no_bidx_len` 强制 `bank_account_no_bidx IS NULL OR octet_length(bank_account_no_bidx) = 32`；唯一约束 `ux_cash_accounts_legal_entity_id_bank_account_no_bidx` 建在其上，用于查重而不落明文；按裁定 B-04 直接复用阶段 2 的唯一计算入口，本阶段不自建哈希或截断路径 |
| owner_user_id | uuid | 否 | 责任人；与法人组成复合外键指向 `platform_authz.user_legal_entity_grants(legal_entity_id,user_id)` |
| opening_balance | numeric(18,2) | 否 | 默认 0，见第 0.3 节 |
| opening_balance_period_id | uuid | 是 | 与法人组成复合外键指向 `ledger.accounting_periods(legal_entity_id,id)` |
| is_active | boolean | 否 | 默认 true |
| deactivated_at | timestamptz | 是 | |
| has_cash_flow | boolean | 否 | 默认 false，首次产生资金腿时置 true，用于 PRD 第 6.2.5 的修改拦截 |
| remark | text | 是 | 长度上限 2000 |

`bank_name_enc` 与 `bank_account_no_enc` 承载的两个逻辑字段均为密级 30。按裁定 C-06，密级的唯一登记表是 `platform_core.sensitive_field_registry`，本阶段在 `db/migrations/finance/` 追加一支 backfill 迁移登记两行，见第 3.6 节；`platform_authz` 只写 `field_permissions` 的字段级授权行，不承载密级。列命名按裁定 A-28/F-51 的全库唯一一套，即 `<语义>_enc bytea` 加 `<语义>_key_ref text`，账号另有掩码尾数与查重盲索引，银行名不建 tail 或盲索引。所有 `_bidx` 列一律固定 32 字节；本表的唯一约束来自“同一法人内银行账号不重复”的业务规则，不是盲索引宽度例外。

本表没有 `status=PENDING_APPROVAL`，也没有币种列：首版固定人民币，启停只由 `is_active` 表达。`EP__FINANCE__CASH_ACCOUNT__REQUIRES_APPROVAL=true`（默认）时，新增、修改、启用和停用的公开请求只在流程引擎保存不可变命令快照并返回 `202 + approval_ref`，不得提前插入或修改 `finance.cash_accounts`；审批通过回调在新事务内重跑权限、字段、唯一性、科目类型、row_version 与职责分离校验后才原子应用，驳回只终结审批实例。开关为 false 时同一公开请求直接应用并返回 201/200。两个分支共用同一领域命令与幂等键，幂等重放不得产生第二个审批实例或第二次变更。

所有延后执行的 finance 审批统一使用阶段 3 的 `platform_flow.approval_command_snapshots`，绝不把业务命令放入 `process_instances.variables`。快照明文的唯一 Rust 形状如下；其中四个 `*Request` 类型逐字段由本阶段两份完整 OpenAPI 的同名 schema 生成，禁止另写手工影子 DTO。`PaymentCreateRequest/RefundCreateRequest` 随待审批头 id 一并封存，解决付款核销行等审批期间尚未形成财务效果的输入保存问题。

```rust
pub const FINANCE_APPROVAL_COMMAND_SCHEMA_V1: &str = "finance-approval-command-v1";

pub enum FinanceApprovalPayloadV1 {
    CashAccountCreate { request: CashAccountCreateRequest },
    CashAccountUpdate { cash_account_id: Id<CashAccount>, request: CashAccountPatchRequest },
    CashAccountSetActive { cash_account_id: Id<CashAccount>, row_version: i64, is_active: bool },
    ReceiptRegister { request: ReceiptCreateRequest },
    PaymentRegister {
        payment_id: Id<Payment>,
        reauth_ref: Id<ReauthChallenge>,
        request: PaymentCreateRequest,
    },
    RefundRegister {
        refund_id: Id<Refund>,
        reauth_ref: Id<ReauthChallenge>,
        request: RefundCreateRequest,
    },
}
pub struct FinanceApprovalCommandEnvelopeV1 {
    pub legal_entity_id: Id<LegalEntity>,
    pub requested_by: Id<User>,
    pub idempotency_key: String,
    pub request_hash: [u8; 32],
    pub payload: FinanceApprovalPayloadV1,
}
```

写快照时以确定性 CBOR 编码 envelope，`command_digest=SHA-256(canonical_bytes)`，再使用法人与 `process_instance_id/owner_module/scenario/action/schema_version` 组成的 AAD 经法人密钥域加密到 `command_enc + command_key_ref`；`process_instances.variables` 只允许保存 `snapshot_id/owner_module/scenario/action/subject_id?` 五个非敏感路由字段。银行名称、账号、核销行、金额和备注不得以明文进入 variables、日志、指标、审计 before/after 或 Outbox。审批回调按 `approval_ref` 锁流程实例与快照，要求状态 PENDING，解密后重算 digest、校验 schema version、request_hash、法人、申请人、variant 与 scenario/action 一致，再重跑当前权限、row_version、余额、账户、期间和职责分离守卫；业务写入与快照转 CONSUMED 同事务提交。驳回/过期只把快照转 REJECTED/EXPIRED；付款头回 DRAFT，退款头进 CANCELLED，资金账户和到款因尚无业务行而保持零写入。未知版本、摘要/AAD 不符或快照缺失一律失败关闭并产生不含命令明文的安全事件。

##### 3.2.3 finance.receivable_entries（应收明细条目）

| 列 | 类型 | 可空 | 约束与说明 |
|---|---|---|---|
| customer_id | uuid | 否 | 与法人组成复合外键指向 `mdm.customers(legal_entity_id,id)` |
| contract_id | uuid | 是 | 与法人组成复合外键指向 `clm.contracts(legal_entity_id,id)` |
| sales_order_id | uuid | 是 | 与法人组成复合外键指向 `sales.sales_orders(legal_entity_id,id)` |
| entry_kind | text | 否 | `ORIGINAL/REVERSAL` |
| source_doc_type | text | 否 | `SALES_INVOICE/INVOICE_REVERSAL/MIGRATION_OPENING` |
| sales_invoice_id | uuid | 条件 | 仅 `ORIGINAL+SALES_INVOICE` 必填；其他形态必空 |
| invoice_reversal_id | uuid | 条件 | 仅 `REVERSAL+INVOICE_REVERSAL` 必填，且同法人唯一 |
| reverses_entry_id | uuid | 条件 | `ORIGINAL` 必空；`REVERSAL` 必填，与 `legal_entity_id` 组成复合自外键指向 `ORIGINAL` 主条目 |
| business_date | date | 否 | 原票取开具日，冲销取登记日，期初取导入业务日 |
| accounting_period_id | uuid | 否 | 与法人组成复合外键指向 `ledger.accounting_periods(legal_entity_id,id)` |
| deferred_from_period_id | uuid | 是 | 与法人组成复合外键指向 `ledger.accounting_periods(legal_entity_id,id)` |
| due_date | date | 否 | `REVERSAL` 复制所指主条目；原票按第 0.4 节 F-11 |
| original_amount | numeric(18,2) | 否 | 严格大于 0；原票/期初为正向创建额，冲销为本次冲销价税合计 |
| settled_amount | numeric(18,2) | 否 | 默认 0，仅是核销效果净额 `S=ΣAPPLY-ΣRELEASE` 的同步投影 |
| open_amount | numeric(18,2) | 否 | 默认等于 `original_amount`，仅是 `row_open=original_amount-settled_amount` 的同步投影 |

NULL-safe CHECK 只允许三种形状：`ORIGINAL+SALES_INVOICE` 必须且只能填写 `sales_invoice_id`，`ORIGINAL+MIGRATION_OPENING` 三个引用列全空，`REVERSAL+INVOICE_REVERSAL` 必须且只能填写 `invoice_reversal_id` 与 `reverses_entry_id`。建立 `(legal_entity_id,id)` 候选键、同法人 `sales_invoice_id` 与 `invoice_reversal_id` 的条件唯一约束；原票、冲销头与父条目引用分别使用 `(legal_entity_id,sales_invoice_id)`、`(legal_entity_id,invoice_reversal_id)`、`(legal_entity_id,reverses_entry_id)` 复合 FK 指向 owner 表或本表的 `(legal_entity_id,id)`，不得只靠全局 UUID 或应用校验。其中 `receivable_entries -> sales_invoices` 与反向的 `sales_invoices -> receivable_entries` 两条外键都必须声明 `DEFERRABLE INITIALLY DEFERRED`，使同一事务内的非空双向引用在提交点统一校验。延迟约束触发器要求冲销父行为同法人、同客户的 `ORIGINAL+SALES_INVOICE`，不得指向期初或另一冲销行；并在提交前按主条目强制：

```text
S = sum(APPLY.settled_amount) - sum(RELEASE.settled_amount)
C = sum(REVERSAL.original_amount)
0 <= S <= O
settled_amount = S
open_amount = row_open = O - S
0 <= effective_open = O - S - C <= row_open
```

`REVERSAL` 行强制 `settled_amount=0` 且 `open_amount=original_amount`，从不进入核销候选或账龄。权威事实是条目、冲销子行与核销效果，不得用两个投影列倒推历史期间。

##### 3.2.4 finance.payable_entries（应付明细条目）

结构、候选键、复合自外键、延迟触发器与 3.2.3 镜像。差异列为 `supplier_id`、`purchase_order_id`、条件非空的 `purchase_invoice_id`，前三者分别以同法人复合外键指向 `mdm.suppliers`、`procure.purchase_orders`、`invoice.purchase_invoices`；`source_doc_type` 只允许 `PURCHASE_INVOICE/INVOICE_REVERSAL/MIGRATION_OPENING`。NULL-safe CHECK 固定：`ORIGINAL+PURCHASE_INVOICE` 必须且只能填写进项原票，`ORIGINAL+MIGRATION_OPENING` 三个引用列全空，`REVERSAL+INVOICE_REVERSAL` 必须且只能填写 `invoice_reversal_id` 与 `reverses_entry_id`。进项原票、冲销头和父条目同样使用带 `legal_entity_id` 的复合 FK；其中 `payable_entries -> purchase_invoices` 与反向的 `purchase_invoices -> payable_entries` 两条外键都必须声明 `DEFERRABLE INITIALLY DEFERRED`。父行必须是同法人、同供应商的 `ORIGINAL+PURCHASE_INVOICE`；业务日期与期间列分别按 3.2.3 同形，并以同法人复合外键指向 `ledger.accounting_periods`；`row_open/effective_open` 与冲销累计公式逐字同构。

##### 3.2.5 finance.advance_receipt_entries（预收台账条目）

| 列 | 类型 | 可空 | 说明 |
|---|---|---|---|
| customer_id | uuid | 否 | 与法人组成复合外键指向 `mdm.customers(legal_entity_id,id)` |
| contract_id | uuid | 是 | 与法人组成复合外键指向 `clm.contracts(legal_entity_id,id)` |
| sales_order_id | uuid | 是 | 与法人组成复合外键指向 `sales.sales_orders(legal_entity_id,id)` |
| receipt_plan_line_id | uuid | 是 | 与法人组成复合外键指向 `clm.contract_payment_schedules(legal_entity_id,id)` |
| source_doc_type | text | 否 | `RECEIPT/INVOICE_REVERSAL/CASH_DOC_REVERSAL/MIGRATION_OPENING` |
| source_doc_id | uuid | 否 | 创建本条目的业务单据或期初导入行 id |
| receipt_id | uuid | 是 | 资金根；非期初的直接到款及由直接资金释放创建的预收必填 |
| source_settlement_root_id | uuid | 是 | 发票冲销或退款冲正将 `DIRECT_CASH` 根转为预收时必填；其他创建形态必空 |
| business_date | date | 否 | 创建事件业务日 |
| accounting_period_id | uuid | 否 | 与法人组成复合外键指向 `ledger.accounting_periods(legal_entity_id,id)` |
| deferred_from_period_id | uuid | 是 | 与法人组成复合外键指向 `ledger.accounting_periods(legal_entity_id,id)` |
| original_amount | numeric(18,2) | 否 | 挂账金额，大于 0 |
| settled_amount | numeric(18,2) | 否 | 默认 0；只是 `net_consumed=ΣAPPLY-ΣRELEASE` 投影 |
| open_amount | numeric(18,2) | 否 | 默认等于 `original_amount`；只是 `advance_open=original_amount-net_consumed` 投影 |

NULL-safe CHECK 固定四种来源：`RECEIPT` 要求 `source_doc_id=receipt_id` 且来源根为空；`MIGRATION_OPENING` 只保留导入行 `source_doc_id`、资金单与来源根均为空；`INVOICE_REVERSAL/CASH_DOC_REVERSAL` 仅在 `DIRECT_CASH` 根释放后新建预收，要求原 `receipt_id` 与 `source_settlement_root_id` 同时非空。资金单和来源根分别用 `(legal_entity_id,receipt_id)`、`(legal_entity_id,source_settlement_root_id)` 复合 FK 指向同法人到款与应收核销根，延迟触发器再强制来源根为 `APPLY+DIRECT_CASH` 且 `funding_receipt_id=receipt_id`。事务末强制 `0 <= net_consumed <= original_amount`、`settled_amount=net_consumed`、`open_amount=advance_open=original_amount-net_consumed`。本表不产生反向条目；恢复和消耗都在 3.2.14 的效果链追加。

##### 3.2.6 finance.advance_payment_entries（预付台账条目）

与 3.2.5 镜像，差异列为 `supplier_id`、`purchase_order_id`、`payment_plan_line_id`、`payment_id`；前三个业务归属列分别以同法人复合外键指向 `mdm.suppliers`、`procure.purchase_orders`、`procure.purchase_order_payment_plans`，`payment_id` 指向同法人付款单；`source_doc_type` 只允许 `PAYMENT/INVOICE_REVERSAL/CASH_DOC_REVERSAL/MIGRATION_OPENING`。四种 NULL-safe 形状、期间复合外键、资金根、来源应付核销根、两个投影列与 `advance_open` 公式全部同构：直接 `PAYMENT` 要求 `source_doc_id=payment_id`，冲销来源要求原 `payment_id+source_settlement_root_id`，期初只保留导入行 id。

##### 3.2.7 finance.unbilled_ar_entries（应收账款未开票过渡科目子账，仅追加）

| 列 | 类型 | 可空 | 说明 |
|---|---|---|---|
| customer_id | uuid | 否 | 与法人组成复合外键指向 `mdm.customers(legal_entity_id,id)` |
| contract_id | uuid | 是 | 与法人组成复合外键指向 `clm.contracts(legal_entity_id,id)` |
| sales_order_id | uuid | 是 | 与法人组成复合外键指向 `sales.sales_orders(legal_entity_id,id)` |
| direction | text | 否 | `ck_unbilled_ar_entries_direction` 取值 DEBIT、CREDIT |
| source_event | text | 否 | `ck_unbilled_ar_entries_source_event` 取值 DELIVERY_CONFIRMED、SALES_INVOICE_ISSUED、SALES_INVOICE_REVERSED、SALES_RETURN |
| source_doc_type | text | 否 | 与 `source_event` 共同限定为交付、销项发票、冲销或销售退货的封闭来源类型 |
| source_doc_id | uuid | 否 | 封闭多态来源 id，由 owner 用例校验同法人及业务归属，不建伪外键 |
| business_date | date | 否 | |
| accounting_period_id | uuid | 否 | 与法人组成复合外键指向 `ledger.accounting_periods(legal_entity_id,id)` |
| accounting_period_seq | integer | 否 | 取同一 `ResolvedPeriod` 的单调序号；历史勾稽只比较该序号，不比较 UUID |
| deferred_from_period_id | uuid | 是 | 与法人组成复合外键指向 `ledger.accounting_periods(legal_entity_id,id)` |
| net_amount | numeric(18,2) | 否 | 大于等于 0，方向由 `direction` 表达；只有 F-50 纯税冲销可为 0 |
| gross_amount | numeric(18,2) | 否 | 严格大于 0，且 `gross_amount >= net_amount`；常规行由用例另行强制 `net_amount>0` |
| voucher_id | uuid | 否 | 与法人组成复合外键指向 `ledger.vouchers(legal_entity_id,id)` |
| reverses_id | uuid | 是 | 仅 `SALES_INVOICE_REVERSED` 非空；长复合父外键与延迟触发器见下文 |

表级 NULL-safe CHECK 固定为 `net_amount >= 0 AND gross_amount > 0 AND gross_amount >= net_amount`，不得恢复单列 `net_amount > 0` 的数据库约束；普通交付、蓝票与销售退货仍由各自用例守卫强制净额大于零，只有合法纯税红字/VOID 冲销可写 `net_amount=0,gross_amount>0`。另建 `UNIQUE(legal_entity_id,id,customer_id)`，父引用固定为 `(legal_entity_id,reverses_id,customer_id) -> (legal_entity_id,id,customer_id)`；NULL-safe CHECK 强制 `source_event=SALES_INVOICE_REVERSED` 当且仅当 `reverses_id IS NOT NULL`，其他来源必须为空。`DEFERRABLE INITIALLY DEFERRED` 约束触发器要求父行是 `SALES_INVOICE_ISSUED`、父 `reverses_id IS NULL`，父子 `contract_id/sales_order_id` 逐项 `IS NOT DISTINCT FROM`、方向相反、子业务日期/期间不早于父效果序，且同一父的全部反向子行累计 `net_amount/gross_amount` 分别不超父值；自指、反向的反向、成环、错客户、错合同或错订单全部拒绝。只带 `(legal_entity_id,reverses_id)` 的短外键作废，应用事务校验不能替代该数据库祖先约束。

双向口径由同一现有视图 `finance.v_unbilled_ar_net` 表达，不新增视图：`net_balance=sum(DEBIT net_amount)-sum(CREDIT net_amount)` 只供当前过渡科目/总账勾稽；`gross_balance=sum(DEBIT gross_amount)-sum(CREDIT gross_amount)` 只供当前信用暴露。`ReceivableExposureQuery` 返回 `delivered_unbilled_gross_amount=greatest(gross_balance,0)`，不读 net_balance。历史勾稽只能从 `unbilled_ar_entries` 按 `accounting_period_seq<=target` 重算，不能读取这个 current view。DEBIT 由交付确认与红字/VOID 冲销产生，CREDIT 由开具与销售退货产生；每条写入命令必须同时携带同一业务事件的 net/gross，不得从税率二次推算。表数 23、finance 本目录迁移数 25、视图数 19 与事件数均不变。

##### 3.2.8 finance.overbilling_entries（待处理超量开票挂账）

| 列 | 类型 | 可空 | 说明 |
|---|---|---|---|
| supplier_id | uuid | 否 | 与法人组成复合外键指向 `mdm.suppliers(legal_entity_id,id)` |
| purchase_order_id | uuid | 否 | 与法人组成复合外键指向 `procure.purchase_orders(legal_entity_id,id)` |
| purchase_invoice_id | uuid | 否 | 与 `legal_entity_id` 组成复合 FK 指向 `invoice.purchase_invoices` |
| material_id | uuid | 否 | 与法人组成复合外键指向 `mdm.materials(legal_entity_id,id)` |
| warehouse_id | uuid | 是 | 与法人组成复合外键指向 `mdm.warehouses(legal_entity_id,id)`，路径一匹配时回填 |
| overbilled_quantity | numeric(18,6) | 否 | 大于 0 |
| unit_price | numeric(18,6) | 否 | 已登记发票的不含税单价 |
| original_amount | numeric(18,2) | 否 | 挂账不含税金额 |
| settled_amount | numeric(18,2) | 否 | |
| open_amount | numeric(18,2) | 否 | |
| settled_quantity | numeric(18,6) | 否 | |
| open_quantity | numeric(18,6) | 否 | |
| status | text | 否 | `ck_overbilling_entries_status` 取值 OPEN、PARTIALLY_SETTLED、SETTLED |
| business_date | date | 否 | 等于采购发票登记日期 |
| accounting_period_id | uuid | 否 | 与法人组成复合外键指向 `ledger.accounting_periods(legal_entity_id,id)` |
| accounting_period_seq | integer | 否 | 取同一 `ResolvedPeriod` 的单调序号；历史勾稽只比较该序号 |
| deferred_from_period_id | uuid | 是 | 与法人组成复合外键指向 `ledger.accounting_periods(legal_entity_id,id)` |

本表带 `row_version`，因为余额可更新。守恒 CHECK 四条：金额两条与数量两条，形式同 3.2.3。

##### 3.2.9 finance.overbilling_settlements（超量开票结清记录，仅追加）

列为 `overbilling_entry_id`（同法人复合外键）、`settlement_path`（`ck_overbilling_settlements_path` 取值 PATH_ONE_RECEIPT_MATCH、PATH_TWO_RED_INVOICE、PATH_THREE_WRITE_OFF）、`settled_quantity numeric(18,6)`、`settled_amount numeric(18,2)`、`source_doc_type/source_doc_id`（三条路径的封闭多态来源，由 owner 校验）、`business_date`、`accounting_period_id/accounting_period_seq/deferred_from_period_id`（前后两项以同法人复合外键指向 `ledger.accounting_periods`，seq 取同一 `ResolvedPeriod`）、`voucher_id`（同法人复合外键指向 `ledger.vouchers`）、`reauth_ref`（真实单列外键指向 `platform_core.reauth_challenges` 并校验证据归属）、`approval_ref`（审批实例白名单）、`reverses_id`（只在冲回 PATH_THREE_WRITE_OFF 时非空），加公共列。

本表建立 `UNIQUE(legal_entity_id,overbilling_entry_id,id)`；父引用固定为 `(legal_entity_id,overbilling_entry_id,reverses_id) -> (legal_entity_id,overbilling_entry_id,id)`，不能用只带法人与 id 的短外键让冲回挂到另一张超量挂账。NULL-safe CHECK 强制普通 PATH_ONE/PATH_TWO/PATH_THREE 结清行 `reverses_id IS NULL`，冲回行只能取 `settlement_path=PATH_THREE_WRITE_OFF` 且 `reverses_id IS NOT NULL`；普通约束 `CONSTRAINT ux_overbilling_settlements_legal_entity_id_reverses_id UNIQUE(legal_entity_id,reverses_id)` 利用 PostgreSQL 默认 `NULLS DISTINCT` 允许任意多根空值、同时禁止同一非空父被冲回两次，不建部分索引。`DEFERRABLE INITIALLY DEFERRED` 约束触发器再要求父行是尚未冲回的 PATH_THREE_WRITE_OFF 根、父 `reverses_id IS NULL`、父子同挂账/数量/金额且业务方向相反，并拒绝自指、成环、跨法人或跨挂账祖先。历史余额以 `reverses_id IS NULL` 为减少、非空为恢复，不能从当前 status 倒推。

路径三的 `reauth_ref` 与 `approval_ref` 非空，对应规格第 17.2 章第十三类分支“转当期主营业务成本的路径按第 12.1 章财务过账类高风险操作完成重新认证”。

##### 3.2.10 finance.receipts（到款单）

| 列 | 类型 | 可空 | 说明 |
|---|---|---|---|
| doc_no | text | 否 | 类型码 RCPT |
| status | text | 否 | `ck_receipts_status` 取值 DRAFT、REGISTERED、CANCELLED、REVERSED |
| customer_id | uuid | 否 | 与法人组成复合外键指向 `mdm.customers(legal_entity_id,id)` |
| receipt_date | date | 否 | |
| posting_date | date | 否 | 取 `receipt_date` |
| accounting_period_id | uuid | 是 | 状态为 REGISTERED 后非空；同法人复合外键指向 `ledger.accounting_periods` |
| deferred_from_period_id | uuid | 是 | 同法人复合外键指向 `ledger.accounting_periods` |
| receipt_amount | numeric(18,2) | 否 | 大于 0 |
| settled_total | numeric(18,2) | 否 | 默认 0；最终追溯到本到款资金根的 `DIRECT_CASH+ADVANCE_AUTO` 应收根净额合计投影 |
| advance_amount | numeric(18,2) | 否 | 默认 0；`receipt_id` 指向本到款的预收条目 `advance_open` 合计投影 |
| cash_account_id | uuid | 否 | 同法人复合外键指向 `finance.cash_accounts` |
| is_manual_settlement_order | boolean | 否 | 默认 false，人工指定核销顺序时为 true |
| refunded_amount | numeric(18,2) | 否 | 默认 0；仅为未冲正退款来源链接的同步汇总投影，不是可退上限事实 |
| voucher_id | uuid | 是 | 同法人复合外键指向 `ledger.vouchers` |
| reversed_by_id | uuid | 是 | 同法人复合外键指向冲正单；目标晚建，在 `finance_create_cash_document_reversals` 迁移内回补 |
| remark | text | 是 | |

三个汇总列均须在事务末从效果链/advance 当前视图/退款来源链接重读一致，不接受客户端填写。`ck_receipts_amount_identity` 对 REGISTERED 行表达 `receipt_amount = settled_total + advance_amount + refunded_amount`；DRAFT/CANCELLED 尚未形成资金效果，不套用该等式。REVERSED 行由延迟触发器强制三项投影全零、未冲正退款为零，且唯一 `cash_document_reversals.reversed_amount=receipt_amount`。

##### 3.2.11 finance.payments（付款登记单）

结构与 3.2.10 镜像，但 `status` 的 CHECK 明确取五值 `DRAFT、PENDING_REAUTH_APPROVAL、REGISTERED、CANCELLED、REVERSED`；差异列为 `supplier_id`（同法人复合外键指向 `mdm.suppliers`）、`payment_request_id`（同法人复合外键指向 `procure.payment_requests`）、`payment_date`、`payment_amount`、`prepaid_amount`、`reauth_ref`（真实单列外键指向 `platform_core.reauth_challenges` 并校验证据归属）、`approval_ref`（审批实例白名单）。`settled_total`、`prepaid_amount`、`refunded_amount` 分别是最终追溯到本付款资金根的 `DIRECT_CASH+ADVANCE_AUTO` 应付根净额、该资金根下预付 `advance_open` 与未冲正返款链接的同步投影；REGISTERED 行强制 `payment_amount = settled_total + prepaid_amount + refunded_amount`，REVERSED 行的归零与冲正金额约束镜像。`DRAFT/PENDING_REAUTH_APPROVAL/CANCELLED` 的三项投影均为零，期间、凭证与全部效果均为空；`PENDING_REAUTH_APPROVAL` 另强制 `reauth_ref/approval_ref` 均非空。审批通过回调锁后重跑付款申请余额、账户、期间和核销快照，再在同一事务写核销/预付效果、资金腿、凭证并进入 REGISTERED；驳回或撤回只回 DRAFT，不得留下任何财务效果。

##### 3.2.12 finance.refunds（客户退款单与供应商返款单）

| 列 | 类型 | 可空 | 说明 |
|---|---|---|---|
| doc_no | text | 否 | 类型码 RFND |
| status | text | 否 | `ck_refunds_status` 取值 PENDING_REAUTH_APPROVAL、REGISTERED、CANCELLED、REVERSED；公开创建直接持久化为待审批，不暴露无后续动作的 DRAFT |
| refund_type | text | 否 | `ck_refunds_type` 取值 CUSTOMER_REFUND、SUPPLIER_REFUND |
| party_id | uuid | 否 | 与 `refund_type` 组成客户/供应商封闭多态引用，由 owner 校验同法人目标 |
| return_doc_type | text | 否 | `ck_refunds_return_doc_type` 取值 SALES_RETURN、PURCHASE_RETURN |
| return_doc_id | uuid | 否 | 与 `return_doc_type` 组成销售/采购退货封闭多态引用，由 owner 校验同法人、party 与状态 |
| invoice_reversal_id | uuid | 是 | 退货部分已开票时必填；同法人复合外键指向 `invoice.invoice_reversals` |
| register_date | date | 否 | |
| posting_date | date | 否 | |
| accounting_period_id | uuid | 是 | 同法人复合外键指向 `ledger.accounting_periods` |
| deferred_from_period_id | uuid | 是 | 同法人复合外键指向 `ledger.accounting_periods` |
| refund_amount | numeric(18,2) | 否 | 大于 0 |
| cash_account_id | uuid | 否 | 同法人复合外键指向 `finance.cash_accounts` |
| reason | text | 否 | 长度上限 2000 |
| voucher_id | uuid | 是 | 同法人复合外键指向 `ledger.vouchers` |
| reversed_by_id | uuid | 是 | 同法人复合外键指向 `finance.cash_document_reversals`，由目标建表迁移回补 |
| reauth_ref | uuid | 是 | 真实单列外键指向 `platform_core.reauth_challenges`，事务校验证据归属 |
| approval_ref | uuid | 是 | |

关联原款项只由 `finance.refund_source_payment_links` 承载。其完整业务列为 `refund_id`、`source_doc_type(RECEIPT|PAYMENT)`、`source_doc_id`、`linked_amount numeric(18,2)`、`advance_consumed_amount numeric(18,2)`、`settlement_released_amount numeric(18,2)`；后两项非负且只是引用本 link id 的预收/预付 `APPLY` 与应收/应付 `RELEASE` 效果汇总投影。条件复合 FK 强制客户退款只引用同法人同客户到款，供应商返款只引用同法人同供应商付款；同退款内 `(source_doc_type,source_doc_id)` 唯一。事务末强制 `linked_amount=advance_consumed_amount+settlement_released_amount`且 `refund_amount=sum(linked_amount)`，客户端不得填写两个投影。

`PENDING_REAUTH_APPROVAL/CANCELLED` 的期间、凭证与全部释放/消耗效果均为空；`PENDING_REAUTH_APPROVAL` 强制 `reauth_ref/approval_ref` 均非空，可保存调用方请求的来源及 `linked_amount`，但两个服务端投影固定为零。审批通过回调必须重新锁定每个来源并按第 4.8 节重算容量，随后才在同一事务写效果、资金腿、凭证并进入 REGISTERED；审批期间余额变化导致容量不足时整笔拒绝，不得按旧快照放款。审批驳回、撤回或审批回调业务校验失败均原子进入 CANCELLED 并记录固定原因与 approval_ref，不提供修改、重新提交或用户取消端点；用户要更正只能发起新退款，原单留作审计。

##### 3.2.13 finance.cash_document_reversals（资金单据冲正登记单）

本表是本阶段为闭合 F-14 与 U-D-02 新增的单据。列为 `doc_no`（类型码 CDRV）、`status`（固定 REGISTERED）、`source_doc_type(RECEIPT|PAYMENT|REFUND)` 与 `source_doc_id`（封闭多态原资金单引用，由条件复合约束/触发器校验类型、同法人与金额）、`register_date`、`posting_date`、`accounting_period_id/deferred_from_period_id`（同法人复合外键指向 `ledger.accounting_periods`）、`reversed_amount numeric(18,2)`、`reason text`、`voucher_id`（同法人复合外键指向 `ledger.vouchers`）、`reauth_ref`（真实单列外键指向 `platform_core.reauth_challenges` 并校验证据归属）、`approval_ref`（审批实例白名单），加公共列。唯一约束必须是 `(legal_entity_id,source_doc_type,source_doc_id)`，防止不同资金表恰好同 id 时误伤；本迁移同时回补 receipts/payments/refunds 的 `(legal_entity_id,reversed_by_id)` 复合外键。冲正行自身不可再被冲正。

##### 3.2.14 四张核销关系表（仅追加）

四表的公共业务列和类型固定如下：

| 列 | 类型 | 可空 | 约束与说明 |
|---|---|---|---|
| effect_kind | text | 否 | `APPLY/RELEASE`；这是金额方向的唯一判据 |
| source_doc_type | text | 否 | 各表下述封闭枚举 |
| source_doc_id | uuid | 否 | 产生本效果的业务单据 |
| root_apply_id | uuid | 否 | 根 `APPLY` 行取自身 id；派生行复制根 id |
| reverses_id | uuid | 是 | 根行必空；派生行必填且指向本次直接反向的父行，不编码金额方向 |
| settled_amount | numeric(18,2) | 否 | 严格大于 0，不存负数效果 |
| settled_at | timestamptz | 否 | 全部锁定后的业务 LIFO 顺序字段 |
| business_date | date | 否 | 业务检索与账龄日期 |
| accounting_period_id | uuid | 否 | 历史切片唯一期间归属；同法人复合外键指向 `ledger.accounting_periods` |
| refund_source_payment_link_id | uuid | 是 | 退款/返款及其资金冲正产生的每条效果必填，以同法人复合外键指向 `finance.refund_source_payment_links`；其他来源必空 |

表专有列和封闭枚举如下：

| 表 | 所属条目与资金列 | source_doc_type |
|---|---|---|
| finance.receivable_settlement_links | `receivable_entry_id NOT NULL`、`funding_origin DIRECT_CASH\|ADVANCE_AUTO NOT NULL`、`funding_receipt_id NULL`、`funding_advance_receipt_entry_id NULL` | `RECEIPT/SALES_INVOICE/CUSTOMER_REFUND/INVOICE_REVERSAL/CASH_DOC_REVERSAL` |
| finance.payable_settlement_links | `payable_entry_id NOT NULL`、`funding_origin DIRECT_CASH\|ADVANCE_AUTO NOT NULL`、`funding_payment_id NULL`、`funding_advance_payment_entry_id NULL` | `PAYMENT/PURCHASE_INVOICE/SUPPLIER_REFUND/INVOICE_REVERSAL/CASH_DOC_REVERSAL` |
| finance.advance_receipt_settlement_links | `advance_receipt_entry_id NOT NULL`、`target_type RECEIVABLE_ENTRY\|CUSTOMER_REFUND\|CASH_DOC_REVERSAL NOT NULL`、`target_id NOT NULL` | `SALES_INVOICE/CUSTOMER_REFUND/INVOICE_REVERSAL/CASH_DOC_REVERSAL` |
| finance.advance_payment_settlement_links | `advance_payment_entry_id NOT NULL`、`target_type PAYABLE_ENTRY\|SUPPLIER_REFUND\|CASH_DOC_REVERSAL NOT NULL`、`target_id NOT NULL` | `PURCHASE_INVOICE/SUPPLIER_REFUND/INVOICE_REVERSAL/CASH_DOC_REVERSAL` |

`DIRECT_CASH` 根必须有同法人原到款/付款且 advance id 为空；`ADVANCE_AUTO` 根必须有同法人预收/预付 id，该期初条目没有现金单据时 funding receipt/payment 可空，否则必须与 advance 资金根一致。所有派生行复制根行的 `funding_origin` 和资金列。advance 两表的 `target_type/target_id` 用 NULL-safe 分支 CHECK 与同法人条件复合 FK/约束触发器分别指向 AR/AP 正向主条目、方向相符的退款/返款或资金冲正单，派生行必须复制根目标；单列多态 UUID 不构成外键。自动核销每个分段的 AR/AP `ADVANCE_AUTO APPLY` 与 advance `APPLY` 金额、条目和资金根必须一一对应。

每张表同时建立 `(legal_entity_id,所属条目 id,id)` 与 `(legal_entity_id,所属条目 id,root_apply_id,id)` 两个候选键；“所属条目 id”在四表依次是 `receivable_entry_id/payable_entry_id/advance_receipt_entry_id/advance_payment_entry_id`。四条根真实自 FK 固定为 `(legal_entity_id,所属条目 id,root_apply_id) REFERENCES 本表(legal_entity_id,所属条目 id,id) ON DELETE RESTRICT DEFERRABLE INITIALLY DEFERRED`，四条父长真实自 FK 固定为 `(legal_entity_id,所属条目 id,root_apply_id,reverses_id) REFERENCES 本表(legal_entity_id,所属条目 id,root_apply_id,id) ON DELETE RESTRICT DEFERRABLE INITIALLY DEFERRED`；约束名逐表固定为 `fk_<table>_root_apply` 与 `fk_<table>_reverses_parent`，不得降成短 FK、非延迟 FK 或 trigger-only 校验。finance owner 在 proof 前预生成本次效果 id 并纳入 settlement root/effect 计划键，取得 proof 后首根仅用一条 `INSERT` 同时写 `id=root_apply_id,effect_kind=APPLY,reverses_id=NULL`；空表首根依靠延迟自 FK 在 COMMIT 合法自引用，禁止先写 NULL、再 UPDATE 回填。NULL-safe CHECK 强制根行必为 `APPLY/root=self/reverses=NULL`，派生行必为 `root<>self/reverses NOT NULL`。`DEFERRABLE INITIALLY DEFERRED` 约束触发器在提交前同时强制：父子 effect 相反；不跨法人、台账侧、所属条目或根；直接子行反向合计不超父行；链无环；且每根均满足 `0 <= ΣAPPLY-ΣRELEASE <= root_apply.settled_amount`。事务末再重读 3.2.3—3.2.6 投影和退款来源链接守恒；应用断言不替代上述 PostgreSQL 约束。

##### 3.2.15 finance.cash_ledger_entries（资金腿明细，仅追加）

| 列 | 类型 | 可空 | 说明 |
|---|---|---|---|
| cash_account_id | uuid | 否 | 同法人复合外键指向 `finance.cash_accounts` |
| direction | text | 否 | `ck_cash_ledger_entries_direction` 取值 IN、OUT |
| amount | numeric(18,2) | 否 | 大于 0 |
| source_doc_type | text | 否 | `ck_cash_ledger_entries_source_type` 取值 RECEIPT、PAYMENT、REFUND、CASH_DOC_REVERSAL |
| source_doc_id | uuid | 否 | 与 `source_doc_type` 组成封闭多态资金来源，由 owner 校验同法人原单 |
| business_date | date | 否 | |
| accounting_period_id | uuid | 否 | 同法人复合外键指向 `ledger.accounting_periods` |
| deferred_from_period_id | uuid | 是 | 同法人复合外键指向 `ledger.accounting_periods` |
| voucher_id | uuid | 否 | 同法人复合外键指向 `ledger.vouchers` |
| reverses_id | uuid | 是 | 仅资金冲正腿非空，`(legal_entity_id,reverses_id)` 真实自外键指向本表 `(legal_entity_id,id)`；延迟约束保证父子同账户、同资金根、金额相等且方向相反 |

PRD 第 6.1.4 要求系统不提供新增资金流水入口，本表在 API 层不暴露任何写端点，只由四个用例经仓储写入，静态检查见第 8.5 节。该约束另有两条既有机制承载，即路由表上不存在任何指向本表的写端点，以及第 3.6 节向 `platform_core.append_only_registry` 登记后挂上的仅追加触发器。

##### 3.2.16 附件关联表

`finance.receipt_attachments`、`finance.payment_attachments`、`finance.refund_attachments`、`finance.cash_document_reversal_attachments`。各表的 `owner_id` 与法人组成复合外键指向对应资金单，`attachment_object_id` 与法人组成复合外键指向 `platform_file.attachment_objects(legal_entity_id,id)`，均取 `ON DELETE RESTRICT`。

#### 3.3 视图与运行时勾稽组装

十个勾稽项中只有八个 finance 自有项落 SQL view；存货与已收货未收票由 Rust owner port 在运行时取数，数据库不能调用 DI，因而全库禁止创建 `finance.v_recon_inventory`、`finance.v_recon_grni` 或同义包装 view。`ReconciliationItemQuery::items` 在执行器传入的同一个 `SnapshotCtx` 内先读八个 finance view，再分别调用阶段 8 的 `ep_contract_inventory::StockValueSubledgerBalancePort` 与阶段 7 的 `ep_contract_procure::GrniSubledgerBalancePort`；实现类型固定为 `InventorySubledgerBalanceQuery` 与 `GrniSubledgerBalanceQuery`，分别由 owner 阶段定义并实现，本阶段只组合与注入。另有四个业务查询视图、三个受治理数据集视图，以及四个当前经营余额视图，合计 19 个。当前视图与历史切片分离：经营查询只读 current view，关账只按目标期间累计追加事件；任何 view/port 都不得用今天的 `open_amount/status` 倒推历史。

| 视图 | 子账侧取数 | 归属 |
|---|---|---|
| finance.v_recon_receivable | 截至目标期间 `Σ(ORIGINAL) - Σ(REVERSAL) - Σ(APPLY-RELEASE)` | 本阶段完整实现；历史切片 |
| finance.v_recon_payable | 截至目标期间 `Σ(ORIGINAL) - Σ(REVERSAL) - Σ(APPLY-RELEASE)` | 本阶段完整实现；历史切片 |
| finance.v_recon_advance_receipt | 截至目标期间 `Σ(创建额) - Σ(APPLY-RELEASE)` | 本阶段完整实现；历史切片 |
| finance.v_recon_advance_payment | 截至目标期间 `Σ(创建额) - Σ(APPLY-RELEASE)` | 本阶段完整实现；历史切片 |
| finance.v_recon_unbilled_ar | 截至目标 `accounting_period_seq` 从 append-only `unbilled_ar_entries` 重算 `Σ(DEBIT.net_amount)-Σ(CREDIT.net_amount)` | 本阶段完整实现；不得读当前 `v_unbilled_ar_net` 代替历史切片 |
| finance.v_recon_overbilling | 截至目标 `accounting_period_seq` 重算 `Σ(overbilling_entries.original_amount)-Σ(非 reversal settlement.settled_amount)+Σ(reversal settlement.settled_amount)` | 本阶段完整实现；不得读今天的 `open_amount/status` |
| finance.v_recon_cash_bank | `opening_balance` 加 `finance.cash_ledger_entries` 的方向净额，限 `account_type` 为 BANK | 本阶段完整实现 |
| finance.v_recon_cash_on_hand | 同上，限 `account_type` 为 CASH | 本阶段完整实现 |

上表恰为八行且是 `V20261019093600__finance_create_reconciliation_views.sql` 允许创建的完整集合。运行时再追加两个非 view 项：`ReconciliationItemCode::Inventory` 调 `StockValueSubledgerBalancePort::balance(snapshot,legal_entity_id,accounting_period_id,accounting_period_seq)`，`ReconciliationItemCode::Grni` 调 `GrniSubledgerBalancePort::balance` 的同形方法；二者返回值与八行 view 一并按第 5.9.1 节冻结枚举顺序组装恰十项。

四个当前经营余额视图固定为：`finance.v_receivable_current`、`finance.v_payable_current`、`finance.v_advance_receipt_current`、`finance.v_advance_payment_current`。前两者逐正向主条目返回 `row_open` 与 `effective_open`，后两者逐条目返回 `advance_open`；最新期间的四个历史累计值必须分别等于四个 current view 的合计。

另有四个业务查询视图：`finance.v_unbilled_ar_net`、`finance.v_receivable_aging`、`finance.v_payable_aging`、`finance.v_cash_account_period_balance`。账龄视图不做物化，理由是共享基线第 3.2 节禁用物化视图。

全部视图不带 `SECURITY DEFINER`，因此继承调用连接的 RLS 会话变量。
按裁定 A-18，本阶段另发布三个受治理数据集视图，dataset code、视图名与 grain 固定如下，任何阶段不得改名。

| dataset code | 视图 | grain |
|---|---|---|
| invoice_purchase_invoices | invoice.v_purchase_invoices_dataset | DOCUMENT |
| finance_receivable_ledger_entries | finance.v_receivable_ledger_entries | ENTRY |
| finance_payable_ledger_entries | finance.v_payable_ledger_entries | ENTRY |

三个视图必须包含 `legal_entity_id`、`security_level`、`data_scope_tags` 三列，并在同一迁移中执行 `GRANT SELECT ON <视图> TO ep_analyst_ro`，不授予 `ep_app_rw` 之外的任何写权限。列名与类型签名必须与阶段 11 的 `reporting.dataset_fields` 登记一致，由阶段 11 的 `reporting-dataset-signature-matched` 按降级口径校验，即签名不符时关闭相关报表入口并开一个降级窗口，不以退出码 78 阻断进程启动。阶段 11 原先登记的 `procure_purchase_invoices` 与 `procure.v_purchase_invoices_dataset` 一行由本阶段的 `invoice_purchase_invoices` 与 `invoice.v_purchase_invoices_dataset` 取代，提供方由采购阶段改为本阶段。

#### 3.4 RLS 策略

上述 40 张表全部带 `legal_entity_id`，全部按共享基线第 3.8 节的统一模板生成策略，模板由迁移生成器产出，本阶段不写变体。策略名一律 `rls_<table>_le`。

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
| ix_invoice_reversals_legal_entity_id_source_sales_invoice_id | invoice.invoice_reversals | 销项原票的多次部分冲销聚合与状态判定 |
| ix_invoice_reversals_legal_entity_id_source_purchase_invoice_id | invoice.invoice_reversals | 进项原票的多次部分冲销聚合与状态判定 |
| ix_purchase_invoices_legal_entity_id_purchase_order_id | invoice.purchase_invoices | 三单匹配与门户按采购订单反查已登记发票 |
| ix_purchase_invoices_legal_entity_id_posting_date | invoice.purchase_invoices | 进项台账按记账日期与会计期间取数 |
| ix_purchase_invoice_lines_legal_entity_id_goods_receipt_line_id | invoice.purchase_invoice_lines | `ReceiptInvoiceMatchQueryPort::lock_candidates_for_receipt_lines`、`match_state` 与 `match_states` 按收货行发现原票行并读取锁后开票容量 |

上述追加查询索引分别集中在 invoice 与 finance 的一支独立非事务索引迁移中，以 `CREATE INDEX CONCURRENTLY` 依次创建；两支文件必须位于各自 `<schema>/concurrent/` 目录且不得混入约束、列变更或其他事务 DDL。新建空表的主键、唯一约束与基线索引仍随建表迁移使用普通 `CREATE INDEX`，不得为此拆入 concurrent 文件。迁移会话按共享基线第 3.9 节固定 `lock_timeout` 与 `statement_timeout`。

#### 3.6 迁移编号与顺序

执行顺序由单一全局 Runner 按文件版本号全序排定，本阶段 invoice 目录建表文件的版本号一律早于 finance 目录中引用它们的文件。

invoice 目录：

1. V20261019090000__invoice_create_tax_rate_options.sql
2. V20261019090100__invoice_create_invoice_applications.sql
3. V20261019090200__invoice_create_invoice_application_link_tables.sql
4. V20261019090300__invoice_create_invoice_number_registry.sql
5. V20261019090400__invoice_create_sales_invoices.sql
6. V20261019090500__invoice_create_sales_invoice_lines.sql
7. V20261019090600__invoice_create_invoice_reversals.sql
8. V20261019090700__invoice_create_invoice_reversal_lines.sql
9. V20261019090800__invoice_create_purchase_invoices.sql
10. V20261019090900__invoice_create_purchase_invoice_lines.sql
11. V20261019091000__invoice_create_invoice_receipt_plan_links.sql
12. V20261019091100__invoice_create_invoice_import_batches.sql
13. V20261019091200__invoice_create_attachment_link_tables.sql
14. V20261019091300__invoice_enable_row_level_security.sql
15. `concurrent/V20261019091400__invoice_create_indexes.sql`
16. V20261019091500__invoice_backfill_seed_tax_rate_options.sql
17. V20261019091600__invoice_create_dataset_views.sql

号码登记必须先于任何引用它的发票头；销项与冲销行表紧跟各自头表；进项头行排在冲销头行之后以满足统一来源模型。第 1 与第 16 个文件在 T0 期间执行，第 16 个文件出厂预置六档税率；已撤销的 `WITHDRAWN__invoice_backfill_migrate_tax_rates_from_mdm.sql` 不得创建。第 17 个文件建立 `invoice.v_purchase_invoices_dataset` 并在同一文件内执行 `GRANT SELECT` 给 `ep_analyst_ro`。

第 11 个文件必须在首次建表时直接创建第 3.1.5 节冻结的长候选键、`fk_invoice_receipt_plan_links_root_allocation` 真实自 FK 和延迟效果图触发器；自 FK 的删除动作必须为 `ON DELETE RESTRICT`，且 `condeferrable=true,condeferred=true`。迁移空表正例先预生成一个 id，再用单条 INSERT 写 `id=root_allocation_id` 的 ISSUE 根并成功 COMMIT；错法人、错合同、错原票、错计划行或错期次的反向行必须由长根 FK/延迟图拒绝。rollback 先删除约束触发器和函数，再随表删除自 FK/候选键，不留下同义短 FK。第 13 个文件一次创建第 3.1.9 节四张附件关联表，包含 `purchase_invoice_attachments`；rollback 逐表反序删除，catalog test 必须逐字断言四个关系名及两组唯一键，禁止少建采购发票附件表。

invoice 建表段内另有三支跨目录追补，版本号夹在其目标建表与下一分钟迁移之间：`V20261019090830__portal_add_invoice_foreign_keys.sql` 放在 `db/migrations/portal/`，为 `portal.supplier_invoice_uploads.accepted_purchase_invoice_id` 补同法人复合外键；`V20261019090910__inventory_add_invoice_foreign_keys.sql` 放在 `db/migrations/inventory/`，为 `inventory.variance_splits.source_doc_id` 补到 `invoice.purchase_invoices` 的同法人复合外键，并以 `(legal_entity_id,source_doc_id,source_doc_line_id)` 补到 `invoice.purchase_invoice_lines(legal_entity_id,purchase_invoice_id,id)` 的头行一致性复合外键，两条均为 `ON DELETE RESTRICT DEFERRABLE INITIALLY DEFERRED`；`V20261019090930__procure_add_invoice_foreign_keys.sql` 放在 `db/migrations/procure/`，以 `(legal_entity_id,purchase_invoice_id,purchase_invoice_line_id) -> invoice.purchase_invoice_lines(legal_entity_id,purchase_invoice_id,id)` 为 `procure.purchase_return_lines` 补 `ON DELETE RESTRICT` 长复合外键，并为 `procure.payable_reservations.purchase_invoice_id` 补同法人复合外键。三支都先做同法人孤儿预检，非零立即失败，不留 `NOT VALID` 尾项；inventory 价差写入口只在其中间一支成功后启用。

第 19090930 号迁移还必须 `CREATE OR REPLACE procure.assert_purchase_return_effect_graph_consistent()`，保留阶段 7 原有退货/库存/GRNI 图并加入发票图；它在 `procure.purchase_returns`、`procure.purchase_return_lines`、`invoice.invoice_reversals`、`invoice.invoice_reversal_lines` 四表安装或替换约束触发器，共同形成双向 `DEFERRABLE INITIALLY DEFERRED` 图，因此从退货侧改状态或从红字侧直插都必在 COMMIT 排队校验。每个 POSTED 退货按 `original_purchase_invoice_id UUID bytes ASC` 分组后必须恰有一张同法人、`direction=INPUT`、`reversal_kind=RED_LETTER`、`linked_purchase_return_id=本退货 id` 的红字头；红字原票供应商、退货供应商、原票头行、退货期间/记账日与红字期间/记账日逐项一致。MATERIAL_RECEIPT 的已开票段以原发票行 `goods_receipt_line_id` 对应退货行，红字 `REDUCE` 行、GRNI `PURCHASE_CREDIT_NOTE/INCREASE` 与紧随其后的 `PURCHASE_RETURN/DECREASE` 必须逐父、逐数量、逐金额一一覆盖；DROP_SHIP 行则以其 `(purchase_invoice_id,purchase_invoice_line_id)` 长键对应原 `DIRECT_EXPENSE_TYPE` 行及同一销售退货/直运采购链，红字来源固定为 `PURCHASE_INVOICE_DIRECT_EXPENSE_REVERSED` 且不得有 inventory/GRNI 效果。两类都要求红字行集合与本次 billed 分组双向无缺无多，逐行 quantity/net/tax/gross 与服务端退货分段一致，按原票行累计冲销不超剩余量额；任何 linked 红字都必须反向命中一张已 POSTED 退货及上述唯一分组，禁止孤立、跨退货或额外 linked 红字。该规则不建立“一张退货全局只能一张红字”的错误唯一约束，而是每个 `(purchase_return_id,original_purchase_invoice_id)` 恰一张。

第 19090930 号迁移的真库 direct-SQL 负例固定覆盖：孤立 `linked_purchase_return_id`、非 POSTED 退货、错法人/供应商/原票/原票行/期间/记账日、非 `INPUT+RED_LETTER`、同一分组缺票或多票、缺行/多行/跨退货行、数量或净税价税不等、物料 GRNI 父链不等额、DROP_SHIP 错直运祖先或伪造 inventory/GRNI；每例都必须在 COMMIT 被延迟图拒绝。回退顺序固定为先删除本文件在 invoice 两表与 procure 退货头/行四表安装或替换的约束触发器，再恢复 `V20261018091400__procure_create_purchase_return_line_serials.sql` 交付的旧函数体及原六表约束触发器，最后删除本文件新增的退货行长外键与 payable reservation 外键；不得 drop 仍被恢复后触发器引用的函数。

finance 目录：

1. V20261019091700__finance_create_aging_bucket_definitions.sql
2. V20261019091800__finance_create_cash_accounts.sql
3. V20261019091900__finance_create_receivable_entries.sql
4. V20261019092000__finance_create_payable_entries.sql
5. V20261019092100__finance_create_advance_receipt_entries.sql
6. V20261019092200__finance_create_advance_payment_entries.sql
7. V20261019092300__finance_create_unbilled_ar_entries.sql
8. V20261019092400__finance_create_overbilling_entries.sql
9. V20261019092500__finance_create_overbilling_settlements.sql
10. V20261019092600__finance_create_receipts.sql
11. V20261019092700__finance_create_payments.sql
12. V20261019092800__finance_create_refunds.sql
13. V20261019092900__finance_create_refund_source_payment_links.sql
14. V20261019093000__finance_create_cash_document_reversals.sql
15. V20261019093100__finance_create_settlement_link_tables.sql
16. V20261019093200__finance_create_cash_ledger_entries.sql
17. V20261019093300__finance_create_attachment_link_tables.sql
18. V20261019093400__finance_enable_row_level_security.sql
19. `concurrent/V20261019093500__finance_create_indexes.sql`
20. V20261019093600__finance_create_reconciliation_views.sql
21. V20261019093700__finance_backfill_seed_aging_buckets.sql
22. V20261019093800__finance_create_dataset_views.sql
23. V20261019093900__finance_backfill_append_only_registry.sql
24. V20261019094000__finance_backfill_sensitive_field_registry.sql

Stage 14 历史迁移撤销的资金账户 owner fact 复用既有 `platform_audit.audit_events`；`V20261019091800__finance_create_cash_accounts.sql` 不新增审计列，finance 迁移文件数、表数与事件数均不变。运行期 action、状态形状、receipt target 与 R0 分离契约以第 4.2 节“历史迁移撤销的资金账户 owner 审计事实”和 Stage 14 第 092600 号静态分支为唯一口径。

第 20 个文件只创建第 3.3 节表内八个 `finance.v_recon_*`，不得创建 `v_recon_inventory/v_recon_grni`；其中 unbilled 与 overbilling 两个 view 必须使用 `accounting_period_seq<=target_seq` 的 append-only 事件公式，SQL 文本静态扫描禁止引用 `v_unbilled_ar_net`、`overbilling_entries.open_amount` 或 `overbilling_entries.status`。第 20 个文件的 rollback 恰删除这八个 view。存货与 GRNI 由 `ReconciliationItemQuery` 经两个 snapshot port 运行时追加，迁移中不存在调用 Rust 的占位对象。

第 15 个文件必须在四张 settlement link 首次建表时逐表创建两条候选键、具名根自 FK 与父长自 FK；八条自 FK 全部逐字为 `ON DELETE RESTRICT DEFERRABLE INITIALLY DEFERRED`，不能留到未编号追补。四张空表各有一个独立正例：预生成 id 后以单条 INSERT 写 `id=root_apply_id,effect_kind=APPLY,reverses_id=NULL` 并成功 COMMIT；错所属条目、错根、错父、短 FK 可侥幸命中的跨链 UUID 均须失败。rollback 按约束触发器/函数、父 FK、根 FK、候选键、表的依赖逆序执行。

finance 建表段另有两支精确追补：`V20261019092430__invoice_add_finance_foreign_keys.sql` 放在 `db/migrations/invoice/`，在 `finance.overbilling_entries` 建成后一次补 `sales_invoices.receivable_entry_id`、`purchase_invoices.payable_entry_id` 与 `invoice_reversals.overbilling_entry_id`；前两条与 finance 侧反向外键成对，四条 invoice↔AR/AP 外键全部显式 `DEFERRABLE INITIALLY DEFERRED`。`V20261019093130__finance_add_deferred_foreign_keys.sql` 放在 `db/migrations/finance/`，在四张核销关系表建成后补 `advance_receipt_entries.receipt_id/source_settlement_root_id` 与 `advance_payment_entries.payment_id/source_settlement_root_id` 的同法人复合外键。两支同样先做孤儿与法人错配预检，回退只删除本文件新增外键。

第 22 个文件按裁定 A-18 建立 `finance.v_receivable_ledger_entries` 与 `finance.v_payable_ledger_entries` 并授予 `ep_analyst_ro`。第 23 个文件按裁定 B-02 向 `platform_core.append_only_registry` 登记本阶段的两张表 `finance.unbilled_ar_entries` 与 `finance.cash_ledger_entries`，两行的 `mode` 取 `APPEND_ONLY`、`mutable_columns` 取 `'{}'`；文件内先插这两行登记，再依次调用 `platform_core.attach_table_guards('finance','unbilled_ar_entries')` 与 `platform_core.attach_table_guards('finance','cash_ledger_entries')`，顺序不得颠倒，挂接函数读登记表取可变列白名单，先挂接后登记取不到 `mutable_columns`；`finance.receivable_entries`、`finance.payable_entries`、`finance.advance_receipt_entries`、`finance.advance_payment_entries`、`finance.overbilling_entries` 五张表带核销金额与状态机，属可更新表，一律不登记。该文件读 finance 写 platform_core，其主要创建对象是 finance 两张仅追加表上的触发器与其登记行，按裁定通则第五条放在 `db/migrations/finance/` 目录下，版本号晚于阶段 2 建立 `platform_core.append_only_registry` 的迁移。登记与触发器的一致性由 `db/checks/append_only_consistency.sql` 经 `xtask sqlcheck` 断言。

第 24 个文件按裁定 A-28、C-06 与 F-51 向 `platform_core.sensitive_field_registry` 登记 `finance.cash_accounts.bank_name` 与 `bank_account_no` 两行：两行 `category=ACCOUNT`、`security_level=30`、`is_field_encrypted=true`、`normalization=TRIM_NFKC`、`release_ref=MIGRATION:V20261019094000`；银行名行 `blind_index=NONE`、`blind_index_column` 为空、`mask_style=NONE`，账号行 `blind_index=EXACT`、`blind_index_column=bank_account_no_bidx`、`mask_style=KEEP_LAST_4`。该文件读 finance 写 platform_core，按通则第五条仍放在 `db/migrations/finance/`。`db/checks/11` 断言两组 `_enc bytea + _key_ref text` 存在、同名明文列均不存在，并核验账号 tail 与 32 字节盲索引约束。

按裁定 A-21，本阶段两个目录一律不建 `backfill_posting_trigger_event_types` 文件，`ledger.posting_trigger_event_types` 的全部登记行由阶段 9a 的种子迁移一次写入且每行只填 `event_type`，本阶段只在 CI 中由 `xtask configdoc` 与 `docs/event-catalog.md` 逐字比对，不进启动自检，也不在关账受理前置校验中复用任何断言，逐条对照见第 5.8 节末表。全部 backfill 与 seed 迁移的 `created_by` 一律取 `ep_foundation::SYSTEM_PRINCIPAL_ID`，即 `00000000-0000-7000-8000-000000000001`，按裁定 A-02，不得自选取值。

每个文件头带 `-- rollback:` 段。建表类文件的回退为对应 `drop table`；seed 与 backfill 文件的回退为按 `code` 删除出厂预置行，或按 `schema_name` 与 `table_name` 删除本次登记的行，即 `append_only_registry` 两行与 `sensitive_field_registry` 两行，第 23 号另 drop `finance.unbilled_ar_entries` 与 `finance.cash_ledger_entries` 两张表上对应的 `assert_append_only` 触发器；`enable_row_level_security` 与 `create_indexes` 的回退为逐条 `drop policy` 与 `drop index`；`create_dataset_views` 的回退为 `drop view` 加 `revoke`。本阶段没有改列类型与收紧非空的迁移，因此全部迁移可在线执行。

---

### 4. 领域模型与关键算法

#### 4.1 核心类型

`ep-domain-invoice` 的聚合与值对象：

- `InvoiceApplication`：聚合根，持有 `IssueRatio issue_ratio`、`IssueRatio remaining_ratio`、`Money contract_amount`、`InvoiceApplicationStatus`。
- `SalesInvoice`：聚合根，持有 `InvoiceNumberRegistryId`、`NonEmptyVec<SalesInvoiceLine>`、三项行汇总与 `SalesInvoiceStatus`；不持有头税率、号码副本或 `reversed_*` 累计。
- `InvoiceReversal`：聚合根，持有 `ReversalDirection`、`ReversalKind`、按方向恰一的原票 id、可空采购退货 id、红字号码登记 id 与 `NonEmptyVec<InvoiceReversalLine>`；分次累计从冲销行按 `source_effect_seq` 重放。
- `InvoiceLineAmounts::validate(tolerance)`：行级净额、税额、价税合计与舍入校验的唯一入口；头金额只求和、不二次舍入。
- `ReceiptPlanAllocationEffect`：`ISSUE/VOID/RED_LETTER` 的仅追加分摊效果，持有合同、期次快照、根分摊、净额与价税合计。

`ep-domain-finance` 的聚合与值对象：

- `ReceivableEntry` 与 `PayableEntry`：共用 `OpenItem<Side>`，持有 `entry_kind=ORIGINAL|REVERSAL`、条件原票/冲销/父条目引用、`original_amount`、`settled_amount`、`row_open`、`effective_open`、`due_date`与两个日期；核销候选只是 `ORIGINAL && effective_open>0`。
- `AdvanceEntry<Side>`：预收/预付正向创建额及资金根，持有 `net_consumed`与 `advance_open`；不建反向 advance 类型。
- `SettlementEffect<Side>`：持有 `effect_kind`、`root_apply_id`、`reverses_id`、`funding_origin`（仅 AR/AP）、资金根、可空退款来源 link 与正金额；符号不由父引用判定。
- `Receipt`、`Payment`、`Refund`、`CashDocumentReversal`：四个资金类单据聚合。
- `CashAccount`：档案聚合。
- `OverbillingEntry`：超量开票挂账聚合。
- `SettlementPlan`：核销分配结果，是一个不可变值对象，含 `lines: Vec<SettlementLine>`、`settled_total`、`residual`。
- `AgingBucketSet` 与 `AgingSnapshot`。

域层方法返回新的状态/效果集合；已过账金额事实只追加，可变的投影必须与同事务权威效果聚合一致。

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

销项状态为 ISSUED、PARTIALLY_RED_REVERSED、VOIDED、RED_REVERSED。VOID 只允许从 ISSUED 且尚无任何冲销行时全额一次进入 VOIDED；红字可由 ISSUED 或 PARTIALLY_RED_REVERSED 分次登记，按 `source_sales_invoice_id` 聚合逐行剩余金额，尚未全额归零进入/保持 PARTIALLY_RED_REVERSED，全部行三项金额归零才进入 RED_REVERSED。VOIDED 与 RED_REVERSED 为终态。并发由锁定原票头行、逐来源行 `source_effect_seq` 唯一和延迟累计触发器串行化，不存在单次冲销唯一约束。

##### 到款单

DRAFT 到 REGISTERED（通过 PRD 第 6.7.2 与 6.7.3 校验且凭证生成成功）、DRAFT 到 CANCELLED、REGISTERED 到 REVERSED（经资金单据冲正登记）。REVERSED 与 CANCELLED 为终态。守卫为：REGISTERED 到 REVERSED 要求该到款单未被任何客户退款单引用，或引用它的退款单已先行冲正。

到款审批开关不增加本表状态：`EP__FINANCE__RECEIPT__REQUIRES_APPROVAL=true` 时，公开请求先只在流程引擎保存不可变登记命令并返回 `202 + approval_ref`，不创建 receipt 行和任何财务效果；审批通过回调重跑同一登记用例后直接原子创建 REGISTERED 行，驳回不创建业务单据。开关为 false 时公开请求直接执行同一用例并返回 201。

##### 付款登记单

DRAFT 到 PENDING_REAUTH_APPROVAL、PENDING_REAUTH_APPROVAL 到 REGISTERED（重新认证通过且审批链通过且凭证生成成功）、PENDING_REAUTH_APPROVAL 到 DRAFT（驳回或撤回）、DRAFT 到 CANCELLED、REGISTERED 到 REVERSED。守卫为：转 REGISTERED 前重跑该付款申请单的累计已登记金额上限校验，避免审批期间被其他登记占满。

##### 退款与返款单

公开创建在同一事务核销重新认证证据并直接持久化为 PENDING_REAUTH_APPROVAL；审批通过后到 REGISTERED，审批驳回、撤回或回调重校验失败后到 CANCELLED；REGISTERED 可经资金单据冲正到 REVERSED。重新认证与审批一律必经，不设开关；CANCELLED/REVERSED 为终态，首版没有退款修改、重提或用户取消入口，因此不建立可观察 DRAFT 态。

##### 超量开票挂账

OPEN 到 PARTIALLY_SETTLED 到 SETTLED，三条结清路径任一条都推进该状态机；SETTLED 为终态但可由路径三的成本冲回退回 PARTIALLY_SETTLED，对应规格第 5.2 章“已按路径三转成本的部分，需先经审批冲回原成本再按路径一入账”。冲回由一条 `reverses_id` 非空的 `finance.overbilling_settlements` 行表达。

##### 资金账户

`is_active` 在 true 与 false 之间双向流转，停用只影响新单据的下拉可选范围，不影响历史引用，对应 PRD 第 6.2.3。

##### 历史迁移撤销的资金账户 owner 审计事实

`reverse_migrated_cash_account` 是 `ep-app-finance` 的 crate-private 用例，只允许 Stage 14 的 `MigrationModuleWriter::apply_reversal` 在同一 `&mut dyn Tx` 内调用；它不注册 HTTP 路由，不复用公开 `actions/deactivate` 的配置型审批快照，而是要求同一批次已有 Stage 14 的 DATA_MIGRATION 冲销批准、第二批准人与重新认证证据，然后复用资金账户停用的领域守卫和写模型。用例按 cash account id 锁根并重读未结资金依赖：`is_active=true` 时只允许写 `is_active=false`、`deactivated_at=effect_occurred_at`、`row_version+1`；`is_active=false` 时保持原 row_version 与 deactivated_at 不变。存在未结到款、付款、退款、返款、资金冲正或其他不能由既有 owner 通道闭合的资金事实时拒绝；不删除账户、不改 opening balance 或历史 cash ledger。事务只捕获一次 `effect_occurred_at`，另生成与 REVERSE receipt id 不同的 `owner_audit_event_id`。

同事务写一条独立、不可变的 `platform_audit.audit_events` owner fact：`event_id=owner_audit_event_id`、`action='FINANCE_CASH_ACCOUNT_MIGRATION_REVERSED'`、`object_type='finance.cash_accounts'`、`object_id=原 APPLY 根 id`、`object_version=根的 after row_version`、`reason='DATA_MIGRATION_REVERSED'`、`occurred_at=effect_occurred_at`。`before`、`after` 各自必须恰有 `{schema_version:1,row_version,is_active,deactivated_at}` 四键；按审计链全库编码规则，row_version 必须是不带前导零的正十进制 JSON 字符串而非 JSON number，is_active 为 JSON boolean，deactivated_at 为 RFC 3339 字符串或 JSON null。真实停用时 092600 逐字比较 `after.row_version=cash_accounts.row_version::text`、`before.row_version=(cash_accounts.row_version-1)::text`、after is_active=false 及 after deactivated_at=effect_occurred_at；已停用保持时 before/after 状态、版本字符串、时点与根逐值相等。该 owner fact 与 R0 分离：REVERSE receipt 固定 `target_object_type='platform_audit.audit_events'`、`target_id=owner_audit_event_id`；R0 才使用 `event_id=receipt.id`、`action='DATA_MIGRATION_REVERSED'`，且 `after.owner_effect_object_type='platform_audit.audit_events'`、`after.owner_effect_id=owner_audit_event_id`。两条审计必须同法人、同 effect_occurred_at 且 event_id 不同；根变更、owner fact、R0、writer receipt 与记录转 REVERSED 同事务提交。Stage 14 第 092600 号延迟分支锁根后逐项核 action、根、before/after、最终状态/版本、独立 R0 与 receipt target；旧审计、错 action/root/version、row_version 数字/非规范字符串、复用 R0、只停用根或只写审计均在 COMMIT 拒绝。

#### 4.3 核销分配算法

输入：`side`（AR 或 AP）、`party_id`、`legal_entity_id`、`amount`、可选的人工指定行列表。输出：`SettlementPlan`。

步骤：

1. 候选集取数。按 `side` 从对应 current view 只取该法人该往来方 `entry_kind=ORIGINAL AND effective_open>0` 的条目，排序为 `due_date ASC, doc_no ASC, entry_id ASC`。冲销条目永不进候选。首版禁止跨往来方核销；人工指定只能改同法人同往来方候选的顺序，并置 `is_manual_settlement_order=true`。
2. 若有人工指定行，候选集改为人工指定的顺序与条目集合，`is_manual_settlement_order` 置 true，该事实按 PRD 第 6.14.4 写入审计。
3. 逐条分配。`residual` 初值为 `amount`；对候选集每一条取 `line=min(residual,locked_effective_open)`；`line` 为零则跳过；写 `APPLY` 根行后递减 `residual`。全部条目及现有根/效果锁定完成后才计算，并在提交前重读 `effective_open`。
4. 剩余部分。循环结束后 `residual` 大于零的，作为转预收或转预付金额返回。
5. 行数上限。`lines.len()` 超过 `EP__FINANCE__SETTLEMENT__MAX_LINES` 时返回 `VALIDATION`，默认 200，与共享基线第 5.1 节的批量上限一致。

边界条件与对应错误：

- `amount` 小于等于零：`FINANCE.RECEIPT.AMOUNT_NOT_POSITIVE`。
- 候选集为空：合法，全额转预收或转预付。
- 人工指定某行金额超过锁后有效未核销余额：`FINANCE.SETTLEMENT.AMOUNT_EXCEEDS_EFFECTIVE_OPEN`，`details` 定位到 `lines[i]` 并回带当前有效未核销余额。
- 人工指定行合计超过 `amount`：`FINANCE.SETTLEMENT.TOTAL_EXCEEDS_DOCUMENT_AMOUNT`，`details` 回带两个数值与差额。
- 人工指定的条目不属该法人或不属该往来方：按共享基线第 5.5 节返回 404 与 `PLATFORM.AUTHZ.NOT_FOUND_OR_DENIED`。

金额一律以 `Decimal` 全精度参与 `min` 比较，写库前一次性 `round(2)`。由于全部输入已是 2 位小数，`min` 不引入新精度，因此不产生尾差。

##### 4.3.1 付款申请核销与 owner 回写

`register_payment` 的唯一实现位于 `crates/application/finance/src/usecase/register_payment.rs`。审批通过回调进入该用例后，先完成纯输入解析、权限/重新认证、UUID 预生成及唯一的 `AccountingPeriodResolver::resolve`，再以 `PaymentRequestQueryPort::for_payment` 无锁取申请头行，并以 `PayableLedgerQuery::entries` 把请求的应付主条目映射到其原进项发票。`INVOICE_PAYMENT` 必须逐行得到唯一 `payment_request_line_id ↔ purchase_invoice_id ↔ payable ORIGINAL`，`PREPAYMENT` 则不得带发票/AP 核销行；供应商、申请类型、状态、账户、行引用或金额任一不一致均在锁前只拒绝，不写业务事实。

无锁 collect 把原付款申请及行、资金账户、全部 AP ORIGINAL、预付、settlement 根/效果、每个发票的 `PayableReservation` 键及本次预生成的付款/效果键编入一份 `F50LockPlan`；然后严格执行 `collect → lock_all → reload → seal`。reload 重复 `for_payment`、应付条目映射与候选集合发现，并在 seal 前核对规范化 id/row_version 集合；集合漂移只走 SQLSTATE 40001。取得唯一 `TransactionLockProof` 前不计算核销额、不释放占用、不占号，也不写 finance/procure/ledger 事实。

proof 后先调 `PayableReservationReadPort::after_lock(tx,ctx,&f50_lock_proof,purchase_invoice_ids)` 取锁后占用，再从已锁 AP 容量计算并追加本次真实 `APPLY`。应用层私有变量 `invoice_releases: BTreeMap<Id<PurchaseInvoice>, Money>` 必须恰按发票汇总本事务新增的有效 AP `APPLY`，键按 UUID bytes ASC，每个值为正，总和等于本次真实核销额；不得用 HTTP 声明额、申请额或预计分配代替。随后恰调一次 `PaymentRequestWritebackPort::register_payment(tx,ctx,&f50_lock_proof,PaymentRequestPaymentWriteback { payment_request_id, payment_id, expected_row_version, allocations })`：`INVOICE_PAYMENT` 的每个 `InvoicePaymentAllocationWriteback.paid_amount_delta` 逐值取自 `invoice_releases`，并与唯一申请行/发票映射相等；`PREPAYMENT` 只构造 `PrepaymentAllocationWriteback`，其 delta 逐值等于本次实际创建的预付资金分配。返回的头行 paid/remaining、status 与 row_version 必须与锁后重算结果逐值相等，失配按内部不变量令整笔回滚。该 owner 回写完成占用释放后，finance 再调一次 `PayableReservationReadPort::after_lock` 并逐发票强制 `effective_open_after >= reserved_after`；此后才可生成凭证并进入 `finish → Outbox → 通知命令 → 审计终结批`。

资金冲正的应付释放也使用同一规则：从原付款持久化分配图重建正值、按申请行稳定排序的 allocations，追加 AP `RELEASE` 后、终结前恰调一次 `PaymentRequestWritebackPort::reverse_payment(tx,ctx,&f50_lock_proof,PaymentRequestPaymentReversalWriteback { payment_request_id, original_payment_id, cash_reversal_id, expected_row_version, allocations })`。其发票金额必须逐值等于本次 AP `RELEASE`，不得按原申请头金额或付款头汇总比例分摊。已 `CLOSED` 申请的占用不恢复，其余状态与占用恢复完全服从阶段 7 第 4.5 节唯一 ABI/语义；只有资金单状态条件更新的唯一胜者可调用，幂等重放不再调用。

#### 4.4 剩余可开比例的推进与回滚

`remaining_ratio` 初值等于 `issue_ratio`。开具登记扣减 `issued_ratio`；每张销项发票的已回滚比例从其冲销行只读汇总，不保存第二个可写累计。作废本次回增该票尚未回滚的全部 `issued_ratio`；分次红冲本次回增 `round(issued_ratio * current_reversal_net / original_invoice_net, 6)`，末次金额全额冲尽时取 `issued_ratio - already_rolled_back_ratio` 一次吸收比例尾差。

守卫为 `issued_ratio <= remaining_ratio + tolerance`，`tolerance` 取 `EP__INVOICE__RATIO__TOLERANCE`。扣减后若 `remaining_ratio` 的绝对值小于 `tolerance` 则归零，避免尾数使状态卡在 PARTIALLY_ISSUED。

回滚守卫：回增后 `remaining_ratio <= issue_ratio + tolerance`，超出即 `BUSINESS_CONFLICT` 与 `INVOICE.INVOICE_APPLICATION.RATIO_ROLLBACK_OVERFLOW`，按规格第 15.2 章进入死信。

累计开票比例的唯一公式为对所有已开票求 `sum(issued_ratio-already_rolled_back_ratio)`：`ISSUED` 通常贡献全额，`PARTIALLY_RED_REVERSED` 贡献未回滚部分，`VOIDED/RED_REVERSED` 终态贡献零。不得用“状态为 ISSUED 才计入”把部分红冲票整张排除。

#### 4.5 账龄分桶

输入：current view 中 `ORIGINAL` 条目的 `due_date`、`effective_open`、评估基准日（默认为服务器自然日，按共享基线第 3.4 节取 `(now() AT TIME ZONE 'Asia/Shanghai')::date`）、`AgingBucketSet`。`AgingBucketSet` 的取数在本阶段来自临时表 `finance.aging_bucket_definitions`，阶段 11 交付 `reporting.aging_bucket_profiles` 与 `reporting.aging_bucket_lines` 之后改经 `ep_contract_reporting::AgingBucketQuery::buckets(tx, ctx, legal_entity_id, ledger_side)`，本表随阶段 11 的删表迁移撤销，分档口径不设第二套，见裁定 C-08。

`overdue_days = 评估基准日 - due_date`，小于零时归入第一档。逐档判定 `from_days <= overdue_days` 且（`to_days` 为空或 `overdue_days <= to_days`）。当前账龄基数一律为 current view 的 `effective_open`，只分组 `ORIGINAL` 主条目；不读行投影作为经营口径，也不把 `REVERSAL` 行单独进账龄。

账龄不依赖 `accounting_period_id`，因此顺延入账不改变账龄，对应规格第 5.2 章子账与凭证共用同一期间归属条款的最后一句。这一点在领域属性测试中作为不变量断言。

#### 4.6 超量开票三条结清路径

三条路径共用同一个余额推进函数 `OverbillingEntry::settle(quantity, amount, path) -> Result<(OverbillingEntry, OverbillingSettlement)>`，守卫为 `quantity <= open_quantity` 且 `amount <= open_amount`。

路径一由收货用例经契约 `OverbillingMatchPort::match_on_receipt` 触发。入参为采购订单、物料、仓库、本次收货数量；返回本次可反向匹配的数量与单价。库存模块按返回的单价与数量同源写数量账与金额账，本阶段按同一金额记 `finance.overbilling_settlements`，凭证由 ledger 端口按规格第 5.2 章超量开票路径一生成，本文不复述分录。匹配数量以该采购订单已挂账的 `open_quantity` 为上限，对应规格第 5.2 章路径一。

路径二由进项方向的冲销登记触发，`invoice.invoice_reversals.overbilling_entry_id` 非空时把本次红字发票不含税金额结清到对应挂账。

路径三由 `POST /api/v1/finance/overbilling-entries/{id}/actions/settle-by-write-off` 触发，属规格第 12.1 章财务过账类高风险操作，需重新认证与审批。

路径三之后的收货：先由 `POST .../actions/reverse-write-off` 经审批冲回原成本，产生 `reverses_id` 非空的结清记录并把挂账退回 PARTIALLY_SETTLED，再走路径一。该顺序在领域层由守卫强制：`open_quantity` 为零而收货侧仍请求匹配时返回 `FINANCE.OVERBILLING_ENTRY.WRITTEN_OFF_REQUIRES_REVERSAL`。

#### 4.7 资金单据冲正

冲正只反向资金事实，不修改原凭证、不反结账，也不无条件镜像原凭证。先按第 6.2 节唯一锁序锁定原款项、退款来源链接、AR/AP 主条目、advance 条目与根/效果，锁后按当前可追溯去向计算。

到款/付款冲正对原金额 `R` 计算当前有效应收/应付核销 `S` 和开放预收/预付 `V`。存在未冲正的下游退款/返款时先以 `FINANCE.CASH_DOCUMENT.DOWNSTREAM_REFUND_EXISTS` 拒绝；否则必须 `R=S+V`，不等则以 `FINANCE.CASH_DOCUMENT.TRACEABILITY_MISMATCH` 整笔失败。对 `S` 每根按当前 `root_net_settled` 追加 `RELEASE`；`ADVANCE_AUTO` 根先向原 advance 追加 `RELEASE`，再把恢复额与原开放 `V` 以 `CASH_DOC_REVERSAL/APPLY` 消耗，不遗留新开放 advance。

退款/返款冲正按每个 `refund_source_payment_link j` 独立处理，不合并资金池：先用 `RELEASE` 恢复原退款消耗的 advance `Y_j`；再对原退款产生的 AR/AP `RELEASE` 以每个主条目的递减 `locked_effective_open` 容量追加父指向明确的 `APPLY=A_j`。无法重新核销的 `E_j` 按原根转入/恢复 advance，然后只对该资金根的 `Q_j=Y_j+E_j` 重跑自动核销得 `Z_j`与 `V_j`。每个 j 分别强制 `X_j=A_j+E_j`、`Q_j=Z_j+V_j`，全部效果保留同一来源 link 和资金根。

finance 只在上述终态成立后构造 `CashReversalPostingSplit { ar_ap_amount, advance_amount }`：到款/付款取 `{S,V}`，退款/返款取 `{Σ(A_j+Z_j),ΣV_j}`。同一锁内构造唯一 `CashDocumentRef`：`reversal=SourceDocumentRef { object_type:"CASH_DOCUMENT_REVERSAL", id:cash_document_reversal.id, doc_no:cash_document_reversal.doc_no }`；`original_doc_type` 按原表映射为 `Receipt|Payment`，原表为 refunds 时再按 `refund_type` 映射为 `CustomerRefund|SupplierRefund`；`original_doc_id=source_doc_id`；`original_voucher_id=锁读原单.voucher_id`；`posting_date=本冲正单 posting_date`；补记时 `backdate_authorization` 取本请求已校验的授权证据，否则为空；同步用例尚未产生来源事件，`source_event_id=None`。任何字段不得从 HTTP、Excel 或插件接收，原单没有非空凭证即以 `FINANCE.CASH_DOCUMENT_REVERSAL.SOURCE_NOT_REGISTERED` 失败。

随后恰调用一次 `PostingPort::post_reversal(tx,ctx,source,split)`；只有 `PostingOutcome::Posted` 可继续，返回的期间必须等于本事务唯一 `ResolvedPeriod`。`IdempotentReplay` 只允许在同一 API 幂等记录已成功且响应存档可验证时返回既有整单结果；新事务首次执行遇到该结果视为孤立凭证图并失败关闭，`Skipped` 永远非法。ledger 追加一条反向资金腿和一张按当前去向拆分的冲正凭证；finance 校验返回 `voucher_id` 后才把效果、资金腿、凭证、冲正单与原单 `REVERSED` 状态在同一事务提交。

#### 4.8 可退金额上限

请求必须显式给出一条或多条原款项分配，并先强制 `refund_amount=sum(requested_linked_amount)`；合计不等返回 `FINANCE.REFUND.SOURCE_ALLOCATION_MISMATCH`。不先把多个上限折成一个可借用的全局 cap。

对每条来源 link 锁后独立计算可追溯容量：先消耗同一原到款/付款资金根下开放的 `advance_open`，余额再按 `root.settled_at DESC,root.id DESC`和根内 `settled_at DESC,id DESC` 两级 LIFO，只对该原款项资助且当前 `root_net_settled>0` 的 AR/AP 根追加 `RELEASE`。任一 link 自身容量不足即返回 `FINANCE.REFUND.SOURCE_CAP_EXCEEDED`，不得用其他 link 剩余容量补足。

退货业务证据另作独立守卫：已开票部分以关联红字冲销本次/累计可退价税合计为上限，未开票部分以 sales/procure owner 返回的退货金额为上限；任一守卫失败整笔拒绝。登记后每条 advance `APPLY` 和 AR/AP `RELEASE` 都保存 `refund_source_payment_link_id`，然后重读强制逐 link 与整单守恒；`refunded_amount` 只是此链的同步投影。

该规则为冻结取值，见第 0.4 节 F-15。

#### 4.9 应收账款未开票过渡科目的双向净额

开具登记一律追加一条 CREDIT 方向的 `finance.unbilled_ar_entries`，`net_amount` 取原票净额、`gross_amount` 取原票价税合计，不做与交付确认的逐笔匹配。invoice issued/reversed v1 现有事件已携带 gross，不增事件或版本。

销项红字冲销按本次冲销头的 `net_amount/gross_amount` 追加 DEBIT；销项 VOID 按原票全额两列追加 DEBIT。分次红冲分别累计校验 net/gross 不超原票，纯税冲销允许 net=0、gross>0。未开票销售退货通过 `record_on_sales_return` 追加 CREDIT，命令从 `sales_return_lines.line_amount_with_tax` 携 gross，不可只冲 net。

因此该子账同时维护两个双向余额：net 余额只对会计勾稽，gross 余额只对信用三桶；两者均是 `sum(DEBIT)-sum(CREDIT)` 且不设关账归零要求。

#### 4.10 会计期间归属

本阶段不实现期间解析，只调用 `ep-contract-ledger::AccountingPeriodResolver::resolve(legal_entity_id, posting_date, tx) -> ResolvedPeriod { accounting_period_id, accounting_period_seq, deferred_from_period_id }`。该方法在同一个 `&mut dyn Tx` 内记忆化，同一事务中第二次调用返回同一取值，因此凭证与全部子账条目共用同一次解析结果是结构性事实而不是一句纪律；共用该结果的子账条目包括台账条目、核销关系行、资金腿明细、超量开票记录与采购暂估回冲效果行。事务句柄的类型按裁定 A-01 固定为 `&mut dyn Tx`，`Tx` 与只读快照上下文 `SnapshotCtx` 均取自 `ep_foundation::port`，由阶段 1 冻结；本阶段全部跨模块契约方法的事务参数一律写成 `&mut dyn Tx`，只读对账取数一律写成 `&dyn SnapshotCtx`，不使用具体连接类型。

记账日期的取值与校验在本阶段：默认取登记时点服务器自然日；允许早于该日并按 PRD 第 6.1.4 提示为补记并写审计；晚于该日一律 `VALIDATION` 并定位字段。

提交响应固定回带 `accounting_period`（编码与名称）与 `is_deferred`，供界面按 PRD 第 6.1.4 显式标注，缺失即视为实现缺陷。

##### 4.10.1 销项、进项原票的非空引用与凭证生成顺序

两种原票头都禁止“先写空引用、过账后 UPDATE 回填”。用例在 coordinator 前只预生成发票 id、号码 owner id 与应收/应付 `ORIGINAL` 条目 id；期间 resolve 后立即 collect 完整 `F50LockPlan` 并按第 6.2 节一次 `lock_all → reload → seal`。只有取得 `TransactionLockProof` 后才允许登记号码、完成服务端金额/容量计算，并以预生成发票 id 写 AR/AP 主条目和自动核销效果；所有 owner mutator 接收同一 proof 且不得补锁。`sales_invoices ↔ receivable_entries`、`purchase_invoices ↔ payable_entries` 的双向法人复合外键一律 `DEFERRABLE INITIALLY DEFERRED`，因此这一中间状态只可存在于同一未提交事务中，提交时两端必须同时存在且互指正确。

取得自动核销额和全部计量项后调用 `PostingPort::post`，source document id 使用预生成发票 id。只有 `Posted { voucher_id,... }` 可以继续首次落原票头；`IdempotentReplay` 必须按 source key 重读一份已完整存在、两端 id 与本次预生成值逐项相同的发票图并直接返回，若图缺失或不一致就是不变量故障；`Skipped` 对原票登记永远非法。拿到非空 `voucher_id` 和期间结果后，才一次插入带非空 `receivable_entry_id/payable_entry_id` 与 `voucher_id` 的发票头、至少一行、收款计划/未开票应收等从属效果，并在事务末重读号码 owner、头行合计、双向外键、台账、凭证与未开票应收。任一步失败连同先写的号码、台账、GRNI 与凭证一起回滚；不得把任一 NOT NULL 列改为可空，也不得为回填开放历史 UPDATE。

采购原票的成本归因在首次过账即逐行固定为 NewRoot，不由阶段 11 事后猜测。物料票每个非零 `purchase_invoice_line.price_variance_released_amount` 都提交一条 `PostingAttribution { source_document_line_id: purchase_invoice_line_id, measure_key: price_variance_released_amount, amount: abs(line_amount), capture_kind: CostPostingVariance(EstimatePriceDiffIssued), dimensions: 该原票行权威维度, reverses_capture_entry_id: None }`；直接费用票每个非零行同形提交 `measure_key: direct_expense_amount,capture_kind: CostDirectExpense`。同一 measure 的逐行 attribution 金额恰等于该 measure 绝对值，零行不伪造 attribution，所有父引用为空并服从 JOURNAL_MAP 的 `Line/NewRootOnly`。本阶段不得把多行压成头级 nil UUID，也不得预留“红字时按原角色猜父”的捷径。

#### 4.11 采购发票登记与三单匹配

本节按裁定 A-10 归本阶段。用例为 `crates/application/invoice/src/usecase/register_purchase_invoice.rs`，端点见第 5.2 节。

步骤：

1. 先完成纯输入解析、权限/重新认证与幂等受理，再由 `AccountingPeriodResolver::resolve` 一次解析期间（含首个零期间同事务建立），随后预生成采购发票 id、应付 `ORIGINAL` id、全部发票行 id、每个发票行各一个候选 `overbilling_entry_id` 和号码 owner id。超量挂账 id 无论最终金额是否为零都预生成，只是零值行在 proof 后不落库；这使得 lock plan 不依赖锁前金额计算。期间解析是 coordinator 前唯一允许的平台前置写；本步不占用发票号码、不计算三单金额/容量/GRNI/库存结果，也不写任何模块业务事实。凭证、应付与预付效果、超量挂账、价差、采购暂估回冲效果行最终全部使用该同一期间。
2. resolve 后立即只做无锁标识发现：以全部 `purchase_order_line_ids` 调用一次 `PurchaseOrderInvoicingPort::targets`，该方法只返回订单 id、行 id、供应商 id 与采购类型；以 `PayableLockCandidateScope::RegisterPurchaseInvoice(PayableRegistrationLockScope { payable_entry_id, purchase_invoice_id, supplier_id, purchase_order_id, contract_id, posting_date, preallocated_overbilling_entry_ids })` 调用一次 `PayableRegistrationPort::lock_candidates`；再经 GRNI/inventory owner 候选入口枚举收货行、GRNI 根与现有效果、库存维度键。把返回的采购订单/行、AP ORIGINAL、预付、settlement 根/效果、超量挂账/累计键及其余 owner 键按 `ep_contract_ledger::f50_lock::F50LockPlan` 的封闭类别构造完整 collected plan。随后只调用一次 `CrossModuleLockCoordinator::lock_all`，由其按 F-50 全局类别序取得全部全局 owner 锁；全局类别锁完后、seal 之前，恰调一次 `PurchaseOrderInvoicingPort::lock_targets_after_global` 按 order id/line id 升序锁定不在全局类别中的 procure owner 辅助行，该方法仍只返回相同标识，再用完全相同入参重调其余 owner 候选入口组成 reloaded plan。两次只比较 `F50LockPlan` 中的规范化 id/维度键集合，禁止把 quantity、amount、status 或 row_version 塞进 plan；集合逐值相等才调 `seal_after_reload` 得到唯一 `TransactionLockProof`，漂移只走 SQLSTATE 40001 整事务重试。取得 proof 前禁止金额/容量/GRNI/库存计算、号码占用和任何模块业务事实写；seal 后本用例及任何被调端口都不得扩 plan、补锁或取得第二份 proof。
3. 取得 proof 后先恰调一次 `PurchaseOrderInvoicingPort::states_after_seal(tx,ctx,&f50_lock_proof,purchase_order_line_ids)`；owner 首句验证 proof 后只重读已锁订单头行，按 order/line id 升序恰返回一项/输入行的 `ordered_quantity/invoiced_quantity/status/row_version`，不得补锁。随后才在该锁后快照上完成三单匹配，依次比对采购订单行、收货行与本次发票行的数量与金额；本模块自有号码/业务键负责防止同一发票重复登记，采购侧开放暂估数量与金额不得由缓存替代。对有收货匹配的物料行，按阶段 7 第 3.2.10 节唯一 Rust 代码块构造带预生成发票行 id 的非空 `GrniInvoiceMatch[]`，在同一事务调用 `ep_contract_procure::GrniEffectWritebackPort::decrease_for_purchase_invoice(tx,ctx,&f50_lock_proof,...)`；没有符合条件的物料匹配行时不调用。该端口先 `assert_covers`，只重读已锁 GRNI 根/效果并按 `PURCHASE_INVOICE/DECREASE` 追加，不执行 `FOR UPDATE`。返回按冻结五字段键升序的 `GrniWritebackEffect[]` 与总暂估回冲金额；本用例逐项验证方向、来源映射及汇总后保存在待落结构。调用失败、空成功结果、累计超根或集合变化整笔回滚；本模块不得直接写 procure schema。
4. 对全部已匹配发票行分别计算 `total_variance_amount = matched_invoice_net_amount - accrual_reversal_amount`，再按发票行 id 升序组装唯一一次文档级 `VarianceSplitCommand`：`source=SourceDocumentRef { doc_type:PURCHASE_INVOICE,doc_id:预生成发票 id,doc_no }`，`period/label` 与本次登记相同；每条 `VarianceSplitLine` 的 `source_line` 取预生成发票行 id/line_no，`posting_line_key` 恒为 `<purchase_invoice_line_id>:1`，并带锁后仓库/物料快照、匹配数量与有符号总价差。调用 `ep_contract_inventory::InventoryVariancePort::split_variance(tx,ctx,&f50_lock_proof,command)`；该端口只验证 proof、重读已锁行并写效果，不补锁。结果键集合必须与输入完全相等；有输入时无论价差是否全零，整张发票都返回一个 VALUE_ADJUST movement，且每个输入键都有非空 `variance_split_id`，只有 `value_entry_id` 可空。逐键取得尚有库存与已出库两部分数量/金额、after 快照并保存在同一待落行结构，两个金额之和必须逐值等于输入总价差；负价差的在库部分不得使存货余额低于零，超出部分归已出库价差。暂估回冲先由采购子账给出，库存价差算法不得反向推导它。空集合时不调用端口；缺行、多行、错键、错来源或错期间均按不变量故障整笔回滚。由于发票父图尚未首次插入，本次 owner port 不要求库存模块 SELECT 父发票；调用身份、预生成 id 形状与头行自洽在端口校验，父图存在性由两条递延复合外键在提交时验证。按裁定 C-13，库存状态相关拆分归阶段 8，ledger 只做分录映射与借贷平衡。
5. 组装第 5.9.1 节的 `RegisterPurchaseInvoicePayable`，并调用 `PayableRegistrationPort::register_purchase_invoice(tx,ctx,&f50_lock_proof,cmd)`；finance 先 `assert_covers`，只重读 proof 覆盖的 AP/预付/核销/超量行。它以预生成发票 id 写入应付 `ORIGINAL`，按同一合同自动核销预付并返回 `advance_auto_applied_amount`；开票数量超过累计收货数量的部分把服务端定标的未匹配净额登记为 `overbilling_entries`，并返回与输入行一一对应的 id/开放量/额。双向法人复合外键延迟到提交校验；该部分不产生 GRNI 回冲，零值不建挂账。返回 payable id、超量键集或金额与命令不一致一律按不变量故障回滚，invoice 不直接写 finance schema。
6. 用同一事务调用 `PostingPort::post`。物料类来源固定为 `PURCHASE_INVOICE_INVENTORY`，基础计量项逐项提交 `gross_amount`、`input_tax_amount`、`accrual_reversal_amount`、`price_variance_in_stock_amount`、`price_variance_released_amount`、`overbilling_amount`，自动核销非零时另交 `advance_auto_applied_amount`；平衡式为 `gross = input_tax + accrual_reversal + price_variance_in_stock + price_variance_released + overbilling`，两类价差为有符号金额。直接费用类来源固定为 `PURCHASE_INVOICE_DIRECT_EXPENSE`，提交 `gross_amount`、`input_tax_amount`、`direct_expense_amount` 与可选自动核销额，平衡式为 `gross = input_tax + direct_expense`。`PostingInput.attributions` 必须逐原票行携带第 4.10.1 节冻结的 `price_variance_released_amount/CostPostingVariance(EstimatePriceDiffIssued)` 或 `direct_expense_amount/CostDirectExpense` NewRoot 归因，父为空且逐行合计等于对应 measure；不得头级合并。只接受第 4.10.1 节定义的合法结果；不得在此之前插入缺少凭证引用的原票头。
7. 取得 `Posted` 的非空凭证 id 后，一次插入含 `payable_entry_id`、`voucher_id` 的采购发票头与全部待落行；四个服务端结果列即使不适用也写零。`IdempotentReplay` 只返回已经完整存在且逐值匹配的图，`Skipped` 或孤立凭证均按不变量故障失败。
8. 发票头行完整落库后、任何幂等/Outbox/审计终结之前，恰调一次 `PurchaseOrderInvoicingPort::record_invoice(tx,ctx,&f50_lock_proof,PurchaseOrderInvoiceWriteback { purchase_invoice_id, lines })`。`lines` 非空，按 `purchase_order_line_id UUID bytes ASC,purchase_invoice_line_id UUID bytes ASC`，每项只带本发票行实际登记的正数 `quantity_delta`，不得携带调用方 expected version，也不得在此重新查找或锁定采购订单行；owner 在 proof 验证后重读步骤 3 已锁状态并作条件更新。返回集合必须与输入订单行恰好相等，并逐行验证 `invoiced_quantity`、line/order status 及两级 row_version；物料行只推进已开数量与 `is_type_locked`，直接费用行达到订购数量时必须闭合行及必要的订单头状态。任一结果漂移整笔回滚；invoice 不直接写 procure schema。
9. 若来源为供应商门户上传，则经 `ep_contract_portal::SupplierInvoiceUploadWritebackPort::accept` 在同一事务把上传记录 `UPLOADED → ACCEPTED` 并写 `accepted_purchase_invoice_id`；端口失败整笔回滚。本阶段不直接写 portal schema；`return_upload` 仍只由阶段 7 的内部退回端点调用。
10. 事务末先重读完整发票头行图及其号码 owner、双向延迟外键、头行合计、采购订单已开数量/状态、应付/预付、GRNI/库存价差、超量挂账与凭证期间；逐条确认每个 variance split 的 `(legal_entity_id,source_doc_id,source_doc_line_id)` 都命中本事务刚落的同法人发票头行。全部相等后依次执行幂等 `finish`、把 `invoice.purchase_invoice.registered.v1` 及前述跨模块待写事件刷新到 Outbox、写同事务通知命令，最后写审计终结批。payload 见第 5.8 节，`accrual_reversal_amount` 必须等于步骤 3 的返回总额；审计之后不得再执行任何数据库语句或跨模块端口。集成测试必须覆盖“先写 split、后写父图可以提交”以及“父头缺失、父行缺失、错法人、错头行四种提交时递延约束失败且全部事实回滚”。

进项红字发票由阶段 7 的采购退货用例经 `ep_contract_invoice::PurchaseCreditNotePort::register_credit_note(tx,ctx,&f50_lock_proof,cmd)` 触发，也允许本阶段的发票更正入口直接触发；两者都在调用方事务内执行并返回 `PurchaseCreditNoteView`。采购退货外层用例把其完整退货/红字/GRNI/库存/AP/settlement 标识纳入同一个 `F50LockPlan` 并把 coordinator 返回的同一 proof 传入；独立更正由 invoice 最外层用例按相同规则取得一次 proof。两个外层还必须把本次红字源行的采购订单行纳入同一流程：collect 阶段调 `PurchaseOrderInvoicingPort::targets`，全局类别锁完后、seal 前调 `lock_targets_after_global`，两次只比较规范化 id 键；seal 后再调 `states_after_seal` 读取数量、状态与 row_version，proof 前调用该方法或把这些值塞进 plan 都失败关闭。端口首句 `assert_covers`，其内部及下游只能锁后重读，不能补锁或另取 proof。`RegisterPurchaseCreditNote` 固定含可空的 `linked_purchase_return_id`，由采购退货用例调用时（含物料、直接费用与直运）必填且必须与同事务采购退货头逐字一致，独立更正时必须为空。事务最前只解析一次期间。只有物料类红字行 `quantity_effect_kind=REDUCE` 且原进项发票对应部分曾写出 `PURCHASE_INVOICE/DECREASE` 时，才按阶段 7 第 3.2.10 节唯一 Rust 代码块构造非空 `GrniCreditNoteReversal[]` 并调用 `GrniEffectWritebackPort::increase_for_purchase_credit_note(tx,ctx,&f50_lock_proof,...)` 逐父追加 `PURCHASE_CREDIT_NOTE/INCREASE`；调用方验证返回按冻结五字段键升序、逐项 `direction=Increase`、来源行映射及 `total_amount=sum(effects.amount)`，空成功结果按不变量故障回滚。没有符合条件的重开行时不调用该写端口。GRNI 重开金额按原父效果与撤销数量在服务端计算，不取红字票面金额。`NONE+ADJUSTED` 的折让、纯金额或纯税额更正不写 GRNI；`REDUCE+ADJUSTED` 只重开对应数量的原暂估。直接费用/直运红字不调用 GRNI 端口。

`register_credit_note` 成功且本次存在 `quantity_effect_kind=REDUCE` 行时，外层在任何 `finish`/Outbox/审计终结前恰调一次 `PurchaseOrderInvoicingPort::reverse_invoice(tx,ctx,&f50_lock_proof,PurchaseOrderInvoiceReversalWriteback { original_purchase_invoice_id, invoice_reversal_id: view.invoice_reversal_id, lines })`。每个 `PurchaseOrderInvoiceLineEffect` 以原进项发票行 id 为 `purchase_invoice_line_id`，只带该行实际减少的正数数量，不携调用方 expected version；`NONE` 行不进列表，全为 `NONE` 时不调用。owner 验证 proof 后以锁后状态作条件更新，返回的已开数量、行/头状态与 row_version 逐值核对；直接费用行因冲减重开 OPEN、全闭合订单因此恢复 SUPPLIER_CONFIRMED 的语义只在 procure owner 内实现，invoice 不直接写 procure schema。

本端口的唯一可抄 Rust 契约只在第 5.9.2 节定义；本节不复制第二份代码。F-50 第 6.5 节只冻结其业务输入规则，开发时必须直接引用第 5.9.2 节的 `RegisterPurchaseCreditNote/InvoiceReversalLineInput/PurchaseCreditNoteView/PurchaseCreditNotePort`；`00c-gap-ruling.md` A-11 的旧命令、旧行 DTO、旧返回 DTO 与 Noop 接线均已被替代。

`lines` 非空且每行只能填写进项来源行；`source_effect_seq` 由服务端锁后分配，不在命令内。锁后原票 `row_version` 与 `expected_original_row_version` 不等时返回既有状态冲突码。期间不作为命令字段：端口复用同一事务内记忆化的 `AccountingPeriodResolver`，返回三项期间值必须与采购调用方先前解析结果逐值相等。`voucher_id` 永远非空；无 GRNI 重开时列表为空、汇总为零，有重开时汇总等于列表金额之和。

凭证来源与计量项不得由调用方选择。链接实物退货固定用 `PURCHASE_INVOICE_LINKED_RETURN_REVERSED`，提交 `gross_amount`、`input_tax_amount`、`grni_reopened_amount`、`linked_return_price_difference_amount = red_net_amount - grni_reopened_amount` 与可选 `released_settlement_amount`；`red_net_amount` 是服务端控制总额而不是 MeasureKey，价差统一映射到主营业务成本，绝不写库存。独立物料更正固定用 `PURCHASE_INVOICE_INVENTORY_REVERSED`，提交价税合计、进项税、GRNI 重开、在库价差冲回、已出库价差冲回、超量挂账结清与可选释放额。直接费用更正固定用 `PURCHASE_INVOICE_DIRECT_EXPENSE_REVERSED`，提交价税合计、进项税、直接费用冲回与可选释放额。三个来源各自的平衡方程与腿表以 `docs/data-dictionary/ledger.md` 为准。

独立物料/直接费用红字不得把原始 capture 的旧角色当成当前父角色。取得 F-50 proof 后、组装红字 `PostingInput` 前，invoice 用例按每个非零实际冲回控制额的原票行构造阶段 11 唯一契约 `PurchaseInvoiceCaptureLineRef`：直接费用取 `PurchaseInvoiceOriginalCaptureKind::DirectExpense`，物料已出库价差取 `EstimatePriceDiffIssued`；输入非空、无重复，按 `(original_purchase_invoice_line_id UUID bytes ASC, original_capture_kind ordinal DirectExpense<EstimatePriceDiffIssued)`，调用一次 `PurchaseInvoiceCaptureReversalBasisQuery::lock_available(tx,ctx,lines)`。结果必须与输入一一同序；每组专用 `PurchaseInvoiceCostLiveFragment` 按 `(root_entry_id UUID bytes ASC,live_entry_id UUID bytes ASC)`，`available_amount: PositiveMoney`，并携原 root 当前 live 叶片的 `effect_sign: DebitCost|CreditCost`、`ReturnCostRole` 与完整 `DimensionSnapshot`。query 只能返回与该原 root 同 effect sign 的 current live leaves，不能复用缺符号的通用 `CostLiveFragment`。该 owner 锁读发生在全部 F-50 类别和本地辅助锁之后，只锁 costing live leaf；返回后本事务不再取得任何新锁。缺组、多组、重复 leaf、错原票来源/kind、同组混合 effect sign、非法 role、空容量或总开放额不足均令整笔回滚，invoice 永不直读 `costing.cost_entries`。

每个 `(原票行,original_capture_kind)` 先把本次 signed 控制冲回额映射到原效果符号：控制额大于零只接受全部 leaf `effect_sign=DebitCost`，控制额小于零只接受全部 leaf `effect_sign=CreditCost`；红字最终成本效果必须与该符号相反，零控制额不得发 query 或 attribution。然后令绝对分数 `R=abs(control)`，只在同符号 live fragments 间按开放额比例做整数分 largest-remainder：先以公开访问器 `available_amount.as_money()` 取得每片严格正的 `Money c_i`，令 `C=Σc_i`，要求 `0<R<=C`；取 `base_i=floor(R*c_i/C)` 后按小数余数降序补足剩余分，余数相同按 `(root_entry_id UUID bytes ASC,live_entry_id UUID bytes ASC)` 升序，且每片最终 `allocation_amount: Money` 必须满足 `0<allocation_amount<=c_i`、合计恰为 R。禁止浮点、银行家舍入、数组原顺序、随机 tie-break 或直接构造私有 `PositiveMoney`。每片产生一条 attribution，`source_document_line_id` 取本次预生成红字行 id、`amount=allocation_amount`、`dimensions` 逐字复制该 live leaf、`reverses_capture_entry_id=Some(live_entry_id)`，不得跨 leaf 合并或改写维度；对应 measure 保留控制额符号并取 `sign(control)*allocation_amount`，所以 signed measure 合计逐分等于控制额，而 `PostingAttribution.amount: Money` 经前述 `allocation_amount>0` 不变量始终严格为正。

静态 measure/capture 映射逐字冻结如下。`PURCHASE_INVOICE_INVENTORY_REVERSED` 把 owner 控制总额 `released_variance_reversed_amount` 按 live role 拆为 `released_variance_reversed_cogs_amount`（`MainOperatingCost → CostPostingVariance(RedLetterDiff)`）与 `released_variance_reversed_direct_expense_amount`（`DirectExpenseCost → CostDirectExpense`），二者带相同符号且和严格等于控制总额；`PURCHASE_INVOICE_DIRECT_EXPENSE_REVERSED` 把 owner 控制总额 `direct_expense_amount` 拆为 `direct_expense_reversed_direct_expense_amount`（`DirectExpenseCost → CostDirectExpense`）与 `direct_expense_reversed_cogs_amount`（`MainOperatingCost → CostInventoryCogs`），二者之和严格等于控制总额。每条 attribution 的 `amount` 取分配绝对值且精确指向 live leaf，MeasureKey 只从上述四个静态键选择，绝不把 role 字符串拼成动态键。`PURCHASE_INVOICE_LINKED_RETURN_REVERSED.linked_return_price_difference_amount` 是本次红字净额减 GRNI 重开的新发生有符号差额，固定 `CostPostingVariance(RedLetterDiff)/Line/NewRootOnly`、父为空，不调用本 query，也不冒充原 `price_variance_released_amount` 的冲回。

`PurchaseCreditNoteView` 固定增加 `grni_reopened_effects: Vec<PurchaseCreditNoteGrniReopen>` 与 `grni_reopened_amount`，每项含红字行、收货行、GRNI 效果、数量与金额。链接实物采购退货时，调用方必须在同一事务逐条等数量、等金额消费这些新效果；实物库存出库只由采购退货的 `PURCHASE_RETURN_INVENTORY` 凭证写，禁止两边重复。该物理凭证的 GRNI 腿取原暂估金额，库存腿取锁后当前账面价值；部分退货按移动平均价、全数退清取退货前库存金额余额全额并使数量/金额/单价同时归零，两者差额由 `return_carrying_difference_amount` 进 COGS。独立价格/税额更正不产生 GRNI；独立数量撤销且货物仍在本方时重开 GRNI 并按独立物料更正规则写价差。`is_for_overbilling_settlement` 为真时同时结清对应挂账；原发票未产生 GRNI 回冲的超量部分不得伪造 GRNI 增加。红字、GRNI、AP `REVERSAL/RELEASE`、库存/成本调整、凭证、审计与 Outbox 任一失败全部回滚。

#### 4.12 往来与预收预付的期初余额导入

本节按裁定 A-24 归本阶段。用例为 `crates/application/finance/src/usecase/import_opening_balances.rs`，端点为 `POST /api/v1/finance/opening-balances/actions/import`，请求体为 `{ledger_side, accounting_period_id, rows[]}`。

`rows` 按 `ledger_side` 写入 `finance.receivable_entries`、`finance.payable_entries`、`finance.advance_receipt_entries`、`finance.advance_payment_entries` 四张表之一，`source_doc_type` 一律取 `MIGRATION_OPENING`，`sales_invoice_id`、`purchase_invoice_id`、`receipt_id`、`payment_id` 四列在期初条目上取空。

本通道与资金账户期初、总账期初、库存期初四者一律不生成凭证：期初对应的总账侧由阶段 9a 的期初余额批次承担，两侧的平衡由第 3.3 节的八个 finance SQL 勾稽 view 加存货/GRNI 两个 snapshot owner port 在同一 `SnapshotCtx` 中组装的十项结果在首个会计期间校验。逐行独立事务落库，失败行不回滚已成功行，逐行返回原因，与第 0.4 节 F-16 的批量导入口径一致。

#### 4.13 F-10 合同终止影响规则

本阶段一次交付两条真实规则：`CLM_TERM_RECEIPT_PLAN` 的实现类型固定为 `ContractTerminationReceiptPlanImpactRule`，位于 `crates/application/clm/src/impact/contract_termination_receipt_plan.rs`；`CLM_TERM_SALES_INVOICE` 的实现类型固定为 `ContractTerminationSalesInvoiceImpactRule`，位于 `crates/application/invoice/src/impact/contract_termination_sales_invoice.rs`。两者都实现 `ep_platform_impact::ImpactRule`，`upstream_event_type()` 都固定返回 `clm.contract.terminated.v1`，`target_module` 分别为 `ModuleCode::Clm` 与 `ModuleCode::Invoice`。阶段 10 把两者注册到既有 `ImpactRegistry` 后，真实累计注册数必须由 4 精确增至 6；七条目录仍是编译期常量，不得以空实现补第七条，也不得为合同终止新增发票或合同侧消费者。

`ReceiptPlanBillingQuery` 定义在 `ep-contract-invoice`，唯一方法固定为 `billing_by_period(tx: &mut dyn Tx, ctx: &SecurityContext, contract_id: Id<Contract>) -> Result<BTreeMap<i32, Money>, AppError>`。实现只读 `invoice.invoice_receipt_plan_links`，按期次汇总 `ISSUE` 正向分摊减 `VOID/RED_LETTER` 反向分摊；缺席键视为零，返回负数视为 `INVARIANT_VIOLATION`，不转成“未开票”。`clm` 不存已开金额副本，不接受同步写端口或 Outbox 回写，合同履约页、变更守卫与终止规则共用这一个读取面。

`ContractTerminationReceiptPlanImpactRule::assess` 只经 clm 仓储取同法人、同合同、状态 ACTIVE 的收款计划，再调一次 `billing_by_period`；只有返回金额等于零的期次按 `period_no ASC,id ASC` 产出 `AUTO_CANCEL`，金额大于零即被占用。`dispose` 锁住期次后重读同一金额：仍 ACTIVE 且为零时置 `VOIDED`、写 `void_reason="合同终止 <合同编号>"` 与审计，返回 `Completed { reason: "RECEIPT_PLAN_AUTO_VOIDED" }`；已 VOIDED 返回 `AlreadySatisfied { reason: "RECEIPT_PLAN_ALREADY_VOIDED" }`；竞态中净额已大于零时不回退开票事实，返回 `AlreadySatisfied { reason: "RECEIPT_PLAN_NOW_BILLED" }`。此规则不产生人工决策码。

`ContractTerminationSalesInvoiceImpactRule::assess` 只经 invoice 仓储查询同法人、同合同且状态为 ISSUED 或 PARTIALLY_RED_REVERSED 的销项发票，按 `id UUID bytes ASC` 每票产出一项 `MANUAL_DECISION`；VOIDED 与已全额红冲的票不命中。初次 `dispose` 不改票、不自动动账，只返回 `ImpactDisposeOutcome::NeedsManualDecision { reason: "SALES_INVOICE_REQUIRES_DECISION" }`，平台按 INVOICE → FINANCE_MANAGER 固定映射分配人工项。人工提交只接受三个 `decision_code`，均要求非空 `decision_reason`，不得解析理由文本决定分支：`VOID_SALES_INVOICE` 要求 `decision_result_doc_id` 非空且指向一张 `direction=OUTPUT,reversal_kind=VOID,source_sales_invoice_id=目标票` 的冲销登记，并要求目标锁后为 VOIDED；`RED_LETTER_SALES_INVOICE` 同样要求结果 id 指向该目标票的 OUTPUT+RED_LETTER 冲销登记，并要求累计冲销后目标已全额红冲；两者校验通过后返回 `AlreadySatisfied`。`KEEP_SALES_INVOICE` 表示明确不冲，要求目标仍为 ISSUED 或 PARTIALLY_RED_REVERSED，且 `decision_result_doc_id` 必须为空，规则不改发票并返回 `Completed { reason: "SALES_INVOICE_KEEP_APPROVED" }`。缺码、错码、结果 id 形状错误、结果单异票、部分红字尚未全额或状态与码不匹配均拒绝并保持 PENDING；三条路径都只能调用本阶段既有作废/红字动作或明确保留，不新增自由动账入口。

---

### 5. API 契约

全部端点遵守共享基线第 5 节：路径前缀 `/api/v1`，字段 snake_case，封套固定，写请求必带 `Idempotency-Key`，请求头集合固定，分页与排序按第 5.3 节。以下只列出各端点的专有部分。

本节是公开 API 的唯一完整清单。合并写在同一表格单元格中的 list/detail 或成对 action 必须按不同 HTTP method/path 分别计数，机械展开结果冻结如下；没有表外隐藏端点：

| 小节 | operation 总数 | 写（POST/PATCH） | 读（GET） |
|---|---:|---:|---:|
| 5.1 发票申请 | 7 | 5 | 2 |
| 5.2 发票开具与冲销 | 9 | 4 | 5 |
| 5.3 资金账户 | 7 | 4 | 3 |
| 5.4 到款、付款、退款与冲正 | 10 | 7 | 3 |
| 5.5 台账、账龄与对账 | 16 | 3 | 13 |
| **合计** | **49** | **23** | **26** |

OpenAPI、路由注册与契约测试必须从下列端点表生成同一 method/path 集合，逐项集合相等：少一个、多一个、同路径重复 method、只写 path 不写 operation，或总数不是 49，均阻断阶段退出。

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

#### 5.2 发票开具与冲销

| 方法与路径 | 请求 | 响应 | 主要错误码 | 权限 |
|---|---|---|---|---|
| POST /api/v1/invoice/sales-invoices | `invoice_application_id`、`identifier: InvoiceIdentifierInput`、`issue_date`、`posting_date`、`issue_content`、`issued_ratio`、非空 `lines[]: SalesInvoiceLineInput`、`attachment_object_ids[]`；必带 `Idempotency-Key` 与 `X-Reauth-Token`。请求不得含头税率或头金额 | 发票视图，含服务端头金额汇总、逐行、`invoice_number_registry_id`、`advance_auto_applied_amount`、`voucher_id`、`accounting_period`、`is_deferred`、`receivable_entry_id` | F-50 的 `INVOICE.INVOICE_LINE.*`、`INVOICE.INVOICE_NUMBER.*`，以及既有比例/日期/重新认证错误 | invoice.sales_invoice:issue |
| GET /api/v1/invoice/sales-invoices 与 /{id} | 过滤 `customer_id`、`status`、`issue_date`、`accounting_period_id`、`invoice_no` | 分页列表与详情 | | invoice.sales_invoice:read |
| POST /api/v1/invoice/sales-invoices/actions/import-batch | `file_object_id`、`template_version = sales-invoice-register-v2`；必带 `X-Reauth-Token` | 后台任务回执 `{ task_id, batch_id }` | INVOICE.IMPORT_BATCH.ROW_LIMIT_EXCEEDED、INVOICE.IMPORT.TEMPLATE_VERSION_UNSUPPORTED、INVOICE.IMPORT.GROUP_HEADER_MISMATCH | invoice.sales_invoice:issue 加 invoice.sales_invoice:import |
| GET /api/v1/invoice/invoice-import-batches/{id} | | 批次详情，含逐行结果对象引用 | | invoice.sales_invoice:read |
| POST /api/v1/invoice/invoice-reversals | `direction`、`reversal_kind = VOID|RED_LETTER`、按方向恰一的 `source_sales_invoice_id/source_purchase_invoice_id`、`register_date`、`posting_date`、`reason`、来源业务动作引用、`overbilling_entry_id?`、`attachment_object_ids[]`、`row_version`；`RED_LETTER` 必须另含 `identifier: InvoiceIdentifierInput` 与非空 `lines[]: InvoiceReversalLineInput`，`VOID` 禁止这两项并由服务端生成全额行；公开请求不得含 `linked_purchase_return_id`，链接采购退货只由内部 `PurchaseCreditNotePort` 在采购退货共享事务调用；必带 `Idempotency-Key` 与 `X-Reauth-Token` | 冲销单视图，含服务端汇总、逐行、可空只读 `linked_purchase_return_id`、可空 `invoice_number_registry_id`、`released_settlement_amount`、`voucher_id` 及回滚后的业务状态 | F-50 的 `INVOICE.INVOICE_REVERSAL.*`、`INVOICE.INVOICE_NUMBER.*` 与 `FINANCE.SETTLEMENT.*` | invoice.invoice_reversal:create |
| POST /api/v1/invoice/purchase-invoices | 手工登记：`supplier_id`、`identifier: InvoiceIdentifierInput`、`invoice_date`、`posting_date`、非空 `lines[]: PurchaseInvoiceLineInput`、`attachment_object_ids[]`；门户受理：`supplier_invoice_upload_id`、`posting_date`、`attachment_object_ids[]`，且禁止再提交 identifier/head/lines。两种形态恰一成立，均不得含头税率、头金额或暂估/价差/超量/自动核销结果字段；同一发票全部行 `cost_kind` 必须同值 | 采购发票视图，含服务端头汇总、逐行 `accrual_reversal_amount/price_variance_in_stock_amount/price_variance_released_amount/overbilling_amount`、对应头级求和、`advance_auto_applied_amount`、`invoice_number_registry_id`、`voucher_id`、期间与 `payable_entry_id` | F-50 的 `INVOICE.INVOICE_LINE.*`、`INVOICE.INVOICE_NUMBER.*`、`PORTAL.SUPPLIER_INVOICE_UPLOAD.*`，以及既有收货/订单/日期错误 | invoice.purchase_invoice_ledger:create |
| GET /api/v1/invoice/purchase-invoices 与 /{id} | 过滤 `supplier_id`、`status`、`invoice_date`、`accounting_period_id`、`purchase_order_id`、`invoice_no` | 分页列表与详情，含发票行、应付明细条目与凭证追溯；取数为本模块自有表 `invoice.purchase_invoices` 与 `invoice.purchase_invoice_lines`，不经 `ep-contract-procure` | | invoice.purchase_invoice_ledger:read |

`POST /api/v1/invoice/sales-invoices` 是规格附录 A.1“应收发票生成”这一度量项的被测端点。
发票打印按裁定 A-08 在阶段 5 交付的三个端口上增量实现，即 `ep_foundation::port::doc::DocTemplatePort::render` 与 `PdfRenderPort::render_pdf`，本阶段只产出像素级套打所需的 `PrintLayout` 取值，不新增任何渲染 trait，也不自建第二条渲染路径。

#### 5.3 资金账户

| 方法与路径 | 说明 | 权限 |
|---|---|---|
| POST /api/v1/finance/cash-accounts | 建档，按配置进审批，配置项见第 7 节；`account_type` 与 `ledger_account_id` 的匹配校验，错误码 FINANCE.CASH_ACCOUNT.ACCOUNT_TYPE_LEDGER_MISMATCH | finance.cash_account:create |
| GET /api/v1/finance/cash-accounts 与 /{id} | 列表按 `code asc`；账号固定返回脱敏后 4 位，详情仅在具备 `finance.cash_account.bank_account_no.read_full`、完成重新认证并写审计后返回完整值；`bank_name` 仅对具备 `finance.cash_account.bank_name.read` 者返回 | finance.cash_account:read；两个完整字段另按前述字段权限判定 |
| PATCH /api/v1/finance/cash-accounts/{id} | `has_cash_flow` 为 true 时拒绝修改 `legal_entity_id` 与 `ledger_account_id`，错误码 FINANCE.CASH_ACCOUNT.LEDGER_ACCOUNT_LOCKED | finance.cash_account:update |
| POST /api/v1/finance/cash-accounts/{id}/actions/deactivate 与 actions/activate | 停用与启用 | finance.cash_account:update |
| GET /api/v1/finance/cash-accounts/{id}/cash-ledger-entries | 资金腿明细视图，`meta` 额外返回 `opening_balance`、`period_in`、`period_out`、`closing_balance` 四个数值 | finance.cash_account:read |

#### 5.4 到款、付款、退款与冲正

| 方法与路径 | 请求要点 | 主要错误码 | 权限 |
|---|---|---|---|
| GET /api/v1/finance/settlement-proposals | 查询参数 `side`（AR 或 AP）、`party_id`、`amount`；返回按默认核销顺序预填的行与转预收或转预付金额 | FINANCE.SETTLEMENT.PARTY_REQUIRED | finance.receivable_entry:read 或 finance.payable_entry:read |
| POST /api/v1/finance/receipts | `customer_id`、`receipt_date`、`posting_date`、`receipt_amount`、`cash_account_id`、`settlement_lines[]`（每项含应收正向主条目 id、`settled_amount` 与请求快照 `expected_effective_open`）、`is_manual_settlement_order`、`attachment_object_ids[]`、`remark` | FINANCE.RECEIPT.AMOUNT_NOT_POSITIVE、FINANCE.RECEIPT.CASH_ACCOUNT_DEACTIVATED、FINANCE.RECEIPT.DATE_IN_FUTURE、FINANCE.SETTLEMENT.AMOUNT_EXCEEDS_EFFECTIVE_OPEN、FINANCE.SETTLEMENT.EFFECTIVE_OPEN_CHANGED、FINANCE.SETTLEMENT.TOTAL_EXCEEDS_DOCUMENT_AMOUNT | finance.receipt:create |
| POST /api/v1/finance/receipts/{id}/actions/cancel | 仅 DRAFT | FINANCE.RECEIPT.INVALID_TRANSITION | finance.receipt:cancel |
| GET /api/v1/finance/receipts 与 /{id} | 过滤 `customer_id`、`status`、`receipt_date`、`accounting_period_id` | | finance.receipt:read |
| POST /api/v1/finance/payments | `payment_request_id`、`supplier_id`、`payment_date`、`posting_date`、`payment_amount`、`cash_account_id`、`settlement_lines[]`（每项含应付正向主条目 id、`settled_amount` 与请求快照 `expected_effective_open`）、`remark`；必带 `X-Reauth-Token`；只返回 `202` 的 `PENDING_REAUTH_APPROVAL` 付款单，审批通过回调才原子写财务效果并转 REGISTERED | FINANCE.PAYMENT.EXCEEDS_REQUEST_AMOUNT、FINANCE.PAYMENT.REQUEST_NOT_APPROVED、PLATFORM.AUTHZ.REAUTH_REQUIRED，其余核销错误同到款且统一使用 F-50 `effective_open` 码 | finance.payment:create |
| POST /api/v1/finance/payments/{id}/actions/withdraw 与 actions/cancel | 撤回与取消 | | finance.payment:cancel |
| POST /api/v1/finance/refunds | `refund_type`、`party_id`、来源业务动作引用、`invoice_reversal_id?`、`source_payments[]`（每项仅 `source_doc_type`、`source_doc_id`、`linked_amount`）、`register_date`、`posting_date`、`refund_amount`、`cash_account_id`、`reason`；禁止提交预收消耗额、核销释放额或凭证拆分；必带 `Idempotency-Key`、`X-Reauth-Token`，只返回 `202` 的 `PENDING_REAUTH_APPROVAL` 单据，审批通过回调重算来源容量后才登记效果 | FINANCE.REFUND.SOURCE_ALLOCATION_MISMATCH、FINANCE.REFUND.SOURCE_CAP_EXCEEDED 及既有来源/账户错误 | finance.refund:create |
| POST /api/v1/finance/cash-document-reversals | `source_doc_type`、`source_doc_id`、`register_date`、`posting_date`、`reason`；禁止提交往来腿、预收预付腿或科目；必带 `Idempotency-Key` 与 `X-Reauth-Token` | FINANCE.CASH_DOCUMENT.DOWNSTREAM_REFUND_EXISTS、FINANCE.CASH_DOCUMENT.TRACEABILITY_MISMATCH、FINANCE.CASH_DOCUMENT.POSTING_SPLIT_MISMATCH、LEDGER.CASH_REVERSAL.SPLIT_INVALID 及既有来源/重复冲正错误 | finance.cash_document_reversal:create |

#### 5.5 台账、账龄与对账

| 方法与路径 | 说明 | 权限 |
|---|---|---|
| GET /api/v1/finance/receivable-entries 与 /{id} | 应收台账；经营余额只返回 `effective_open`，详情核销关系返回 `effect_kind`、`funding_origin`、`root_apply_id`、`reverses_id` 与来源单据，不再返回旧 `origin` | finance.receivable_entry:read |
| GET /api/v1/finance/payable-entries 与 /{id} | 应付台账；经营余额只返回 `effective_open`，详情核销关系使用与应收镜像的显式 effect/root 语义 | finance.payable_entry:read |
| GET /api/v1/finance/advance-receipt-entries 与 /api/v1/finance/advance-payment-entries | 预收预付台账，只读，无写端点，对应 PRD 第 6.11.3 最后一段 | finance.advance_entry:read |
| GET /api/v1/finance/receivable-agings 与 /api/v1/finance/payable-agings | 当前账龄只按 `effective_open` 汇总；查询参数 `group_by` 取 `customer`、`contract`、`sales_order`、`bucket`；下钻用 `filter[bucket_code]=eq:` 加 `expand=entries`。历史期间报表必须调用截至期间事件切片，不读今天的 `open_amount` | 按方向分别为 finance.receivable_entry:read / finance.payable_entry:read |
| GET /api/v1/finance/unbilled-ar-entries | 已交付未开票只读查询视图，`meta` 返回净额与方向 | finance.receivable_entry:read |
| GET /api/v1/finance/overbilling-entries 与 /{id} | 待处理超量开票查询视图，字段逐字取 `supplier_id,purchase_order_id,purchase_invoice_id,material_id,warehouse_id,overbilled_quantity,unit_price,original_amount,settled_amount,open_amount,settled_quantity,open_quantity,status,business_date,accounting_period_id,deferred_from_period_id,row_version`；`status` 只取 `OPEN|PARTIALLY_SETTLED|SETTLED`。可选 `latest_effective_settlement_path` 从未被冲回的最新 settlement 派生，只读且不参与状态机；不得返回旧 `remaining_amount`、五态状态或头级 `write_off_voucher_id`。`meta` 返回该法人该期间的挂账合计 | finance.overbilling_entry:read |
| POST /api/v1/finance/overbilling-entries/{id}/actions/settle-by-write-off | 路径三，必带 `X-Reauth-Token` 并进审批 | finance.overbilling_entry:write_off |
| POST /api/v1/finance/overbilling-entries/{id}/actions/reverse-write-off | 冲回路径三，必带 `X-Reauth-Token` 并进审批 | finance.overbilling_entry:reverse |
| GET /api/v1/finance/reconciliations | 查询参数 `accounting_period_id` 与可选 `item`；返回十项的子账侧、总账侧与差额三列，十项的子账侧均已接入，存货与已收货未收票两项经 `ep_contract_inventory::StockValueSubledgerBalancePort` 与 `ep_contract_procure::GrniSubledgerBalancePort` 两个模块端口取数，见第 3.3 节；差额非零时附差异事项引用；本端点不提供任何调整入口，对应 PRD 第 6.13.2 | finance.reconciliation:read |
| GET /api/v1/finance/cash-ledger-entries | 全法人资金腿明细，按账户筛选 | finance.cash_account:read |
| POST /api/v1/finance/opening-balances/actions/import | 往来与预收预付期初导入，请求体 `{ledger_side, accounting_period_id, rows[]}`，见第 4.12 节；逐行独立事务，不生成凭证；错误码 FINANCE.OPENING_BALANCE.PERIOD_NOT_FIRST、FINANCE.OPENING_BALANCE.ROW_LIMIT_EXCEEDED、FINANCE.OPENING_BALANCE.PARTY_NOT_FOUND | `ledger_side` 为 RECEIVABLE/PAYABLE 时分别要求 finance.receivable_entry:create / finance.payable_entry:create；两类 ADVANCE 要求 finance.advance_entry:create |

移动端按规格第 6.2 章矩阵只提供本节全部 GET 端点，POST 端点在移动端由前端隐藏，服务端不做端别拒绝，理由是端别限制属客户端能力矩阵而不是服务端授权。

#### 5.6 幂等语义

全部 POST 与 PATCH 按共享基线第 5.4 节执行。本阶段的补充约定：`request_hash` 的计算不包含 `X-Reauth-Token`，理由是重新认证凭证单次有效，重放时该头必然不同，若纳入哈希会把合法重放判成 `PAYLOAD_MISMATCH`。该约定需回写共享基线第 5.4 节。

批量导入的幂等作用域为逐行：每行的幂等键取批次幂等键加行号派生的 UUIDv5，因此重跑同一批次不产生重复发票。

#### 5.7 权限要求

对象类型注册 14 个：`invoice.invoice_application`、`invoice.sales_invoice`、`invoice.invoice_reversal`、`invoice.purchase_invoice_ledger`、`finance.cash_account`、`finance.receipt`、`finance.payment`、`finance.refund`、`finance.cash_document_reversal`、`finance.receivable_entry`、`finance.payable_entry`、`finance.advance_entry`、`finance.overbilling_entry`、`finance.reconciliation`。

动作注册 12 个：create、read、update、submit、cancel、issue、import、reverse、settle、write_off、export、approve。

字段级权限注册 2 个：`finance.cash_account.bank_name.read`、`finance.cash_account.bank_account_no.read_full`。

判定顺序按共享基线第 11.3 节，即法人、对象、记录、字段与密级四级，显式拒绝优先。不可见记录一律 404 与 `PLATFORM.AUTHZ.NOT_FOUND_OR_DENIED`。

职责分离：发票申请的申请人不可自审；开票的发起人不可自审；付款登记的发起人不可自审；超量开票路径三的发起人不可自审。四项由 platform_authz 的审批授权判定承担，本阶段只声明该要求并在集成测试中验证。

#### 5.8 领域事件

13 个事件，命名按共享基线第 6.1 节的四段式，登记到 `docs/event-catalog.md` 后才实现。既有 12 个中受 F-50 影响的 payload 按头行/来源链接更新；新增 `portal.supplier_invoice_upload.accepted.v1` 由正式进项发票受理事务产生。信封字段不增不减，`security_level` 取单据取值，`data_scope_tags` 携带客户或供应商与合同两类标签。期间字段逐事件冻结：`invoice.invoice_application.submitted.v1`、`invoice.invoice_application.approved.v1` 与 `invoice.invoice_import_batch.completed.v1` 的 `posting_date/accounting_period_id` 固定为 null，原因是它们不对应单一过账事务；9 个 posting-trigger 事件取各自同事务 `ResolvedPeriod`；`portal.supplier_invoice_upload.accepted.v1` 复用同一受理事务内采购发票的 `posting_date/accounting_period_id`，只用于追溯且不因此产生第二张凭证。禁止给审批/批次事件伪造日期，也禁止从导入批次成功行中任选一行的期间。

| 事件类型 | 触发点 | payload 要点 | 消费方 |
|---|---|---|---|
| invoice.invoice_application.submitted.v1 | 申请提交进入审批 | 申请单 ID、合同、客户、开票比例、申请金额 | notify、reporting |
| invoice.invoice_application.approved.v1 | 审批链全通过 | 申请单 ID、剩余可开比例 | notify |
| invoice.sales_invoice.issued.v1 | 开具登记提交成功 | 发票 ID、申请单 ID、客户、合同、不含税金额、税额、价税合计、`advance_auto_applied_amount`、应收条目 ID、凭证 ID | notify、reporting、search、客户 360 |
| invoice.purchase_invoice.registered.v1 | 采购发票登记提交成功 | `purchase_invoice_id`、`doc_no`、`supplier_id`、`purchase_order_id`、`cost_kind`、`net_amount`、`tax_amount`、`gross_amount`、`accrual_reversal_amount`、`price_variance_in_stock_amount`、`price_variance_released_amount`、`overbilling_amount`、`advance_auto_applied_amount`、`payable_entry_id`、`voucher_id`、`lines` | notify、reporting、门户投影 |
| invoice.sales_invoice.reversed.v1 | 销项作废或红冲登记成功 | 冲销单 ID、原发票 ID、处理类型、红字金额、回滚后的剩余可开比例 | notify、reporting、search |
| invoice.purchase_invoice.reversed.v1 | 进项红字冲销登记成功 | 冲销单 ID、原采购发票 ID、处理类型、红字金额、是否用于超量开票结清 | notify、reporting |
| invoice.invoice_import_batch.completed.v1 | 批量导入任务结束 | 批次 ID、总行数、成功行数、失败行数、结果对象引用 | notify |
| finance.receipt.registered.v1 | 到款登记成功 | 到款单 ID、客户、到款金额、核销合计、转预收金额、资金账户、凭证 ID | notify、reporting、客户 360 |
| finance.payment.registered.v1 | 付款登记成功 | 付款单 ID、供应商、付款金额、核销合计、转预付金额、付款申请单 ID、凭证 ID | notify、reporting、门户投影 |
| finance.refund.registered.v1 | 退款或返款登记成功 | 退款单 ID、类型、往来方、金额、关联退货单、关联冲销单、凭证 ID | notify、reporting |
| finance.cash_document.reversed.v1 | 资金单据冲正登记成功 | 冲正单 ID、原单据类型与 ID、冲正金额、凭证 ID | notify、reporting |
| finance.overbilling_entry.settled.v1 | 三条结清路径任一条完成 | `overbilling_entry_id`、`settlement_path`、`settled_quantity`、`settled_amount`、`open_amount`、`open_quantity`、`status`、`voucher_id`、`posting_date`、`accounting_period_id` | notify、reporting |
| portal.supplier_invoice_upload.accepted.v1 | 正式进项发票受理事务内把上传记录迁到 ACCEPTED | 上传记录 ID、采购发票 ID、供应商、法人、受理时间 | portal 投影、notify |

上述事件的消费方均不做过账，见第 0.1 节。事件的 `aggregate_type` 按共享基线第 6.1 节取 `<schema>.<表名>`，采购发票登记事件取 `invoice.purchase_invoices`。

按裁定 A-21，登记表、登记接口与全部登记行均归阶段 9a：`ledger.posting_trigger_event_types` 的 13 行由阶段 9a 的种子迁移一次写入，每行只填 `event_type`，原有的 `ledger_event_kind` 与 `registered_by_module` 两列已删，本阶段不得再引用，也不新增任何回填迁移。按总览第 1.5 节第三条，`PostingTriggerRegistry::assert_registered` 与错误码 `LEDGER.POSTING_TRIGGER_EVENT_TYPE.REGISTRY_MISMATCH` 整项删除，本阶段在启动自检、`--check` 与关账受理三处都不调用该方法，理由是规格第 10.2 章逐字枚举关账受理只有两项前提，在计划层新增第三项受理前提是计划凌驾规格。登记表一致性的承接方定死为两条，即 `xtask configdoc` 从 `docs/event-catalog.md` 生成阶段 9a 的第 16 号种子迁移并在 CI 中逐字比对，以及阶段 3b 的 `event-catalog-consistent` 自检项且不通过时停止派发未登记事件类型；本阶段下表九个事件的一致性即由这两条承接，运行期不再有退出码 78 这条路径，关账受理前提仍为规格第 10.2 章的两条。

| event_type |
|---|
| invoice.sales_invoice.issued.v1 |
| invoice.purchase_invoice.registered.v1 |
| invoice.sales_invoice.reversed.v1 |
| invoice.purchase_invoice.reversed.v1 |
| finance.receipt.registered.v1 |
| finance.payment.registered.v1 |
| finance.refund.registered.v1 |
| finance.cash_document.reversed.v1 |
| finance.overbilling_entry.settled.v1 |

#### 5.9 对外契约 trait

16 个自有 trait 定义在两个 contract crate 中：`ep-contract-finance` 恰为 10 个，`ep-contract-invoice` 恰为 6 个。实现注册在 `apps/core-server/src/wiring/` 与 `apps/job-worker/src/wiring/` 两个目录，调用方按共享基线第 1.3 节只依赖 contract，不依赖 application。业务事务内的同步查询与写入一律接收调用方的 `&mut dyn Tx`；只有关账/对账执行器已经建立的 REPEATABLE READ 快照读取接收 `&dyn SnapshotCtx`，不得把二者互换或在实现内另开事务。两个类型的准确路径为 `ep_foundation::port::tx::{Tx, SnapshotCtx}`，按裁定 A-01 由阶段 1 冻结。外部 owner trait 的精确消费集合见第 1 节第 5 条与第 2.2 节 `apps/core-server` 行；它们由各 owner contract 定义、在下文指定使用位注入，不计入本节 16 个自有 trait，也不允许 Stage 10 复制第二份 trait/DTO。

| trait | 所在 crate | 调用方 | 语义 |
|---|---|---|---|
| PayableRegistrationPort | ep-contract-finance | ep-app-invoice | `lock_candidates(tx,ctx,&PayableLockCandidateScope)` 是登记进项蓝票/红字前的 finance owner 无锁候选发现；`register_purchase_invoice(tx,ctx,f50_lock_proof:&TransactionLockProof,cmd)` 写应付明细、预付自动核销与超量开票挂账。proof 必须覆盖本次 AP/advance/settlement/overbilling 全集并与调用方事务同源，finance 只锁后重读、不得补锁。按裁定 A-10 调用方由 ep-app-procure 收窄为 ep-app-invoice |
| OverbillingMatchPort | ep-contract-finance | ep-app-procure 的收货用例与 R-PROC-05 | `candidate_entry_ids_for_receipt` 在 collect/reload 两阶段无锁返回候选挂账 id；`match_on_receipt` 只在同一 proof 覆盖的锁后集合上结清路径一，返回按收货行分组的计价/结清段；`settlement_segments_by_receipt_lines` 在执行器提供的同一 `SnapshotCtx` 上为 R-PROC-05 返回同源历史段。三个方法由同一真实实现提供并按运行入口注入 core-server、job-worker；阶段 7 不跨 schema SQL，也不注入任何替身 |
| UnbilledArPort | ep-contract-finance | ep-app-sales | 交付确认与未开票销售退货的子账腿。`record_on_delivery(tx,ctx,DeliveryUnbilledArCommand { delivery_confirmation_id,customer_id,posting_date,accounting_period_id,accounting_period_seq,deferred_from_period_id,voucher_id,direction:UnbilledArDirection::Debit,net_amount,gross_amount })`；`record_on_sales_return(tx,ctx,SalesReturnUnbilledArCommand { sales_return_id,customer_id,posting_date,accounting_period_id,accounting_period_seq,deferred_from_period_id,voucher_id,direction:UnbilledArDirection::Credit,net_amount,gross_amount })`。两方法使用调用方同一 `&mut dyn Tx`，只在 Stage 6 非零收入分支中以 `PostingOutcome::Posted` 的非空 voucher 调用；合法全零 `Skipped` 分支不调用、不写零额行。表行一次插入便满足 NOT NULL/APPEND_ONLY，不先空后回填 |
| ReceivableExposureQuery | ep-contract-finance | ep-app-sales | 返回 `ReceivableExposureView { receivable_open_amount, delivered_unbilled_gross_amount }`；前者是应收未收价税合计，取 `Σ(v_receivable_current.effective_open)`；后者取 `greatest(v_unbilled_ar_net.gross_balance,0)`。两项均不使用 net_balance，供 sales 组装三桶含税暴露；同批真实接线且不注入替身 |
| ReceivableLedgerQuery | ep-contract-finance | ep-app-reporting、ep-app-finance 的 `ReceiptsSectionProvider` | 应收台账与核销关系只读查询；ep-app-crm 只聚合 `Customer360SectionProvider`，不直接依赖本 trait |
| PayableLedgerQuery | ep-contract-finance | ep-app-procure、ep-app-reporting | 应付台账与核销关系只读查询，方法按裁定 C-15 固定为 `open_balance(tx, ctx, purchase_invoice_id: Id<PurchaseInvoice>) -> Result<Money, AppError>`；方法名为兼容名，返回值唯一取该原票 `v_payable_current.effective_open`；阶段 7 的 `PayableQueryPort` 作废 |
| SupplierStatementQuery | ep-contract-finance | ep-app-portal | 供应商收付款对账查询的取数，方法按裁定 C-15 固定为 `statement(tx, ctx, supplier_id: Id<Supplier>, period: PeriodRange) -> Result<SupplierStatementView, AppError>`，当前余额取 `effective_open`、历史取期间事件切片，返回未脱敏结构，脱敏在门户侧完成；阶段 7 的 `PayableStatementQueryPort` 作废 |
| CashAccountQuery | ep-contract-finance | ep-app-procure、ep-app-reporting | 资金账户与资金腿明细只读查询 |
| AgingQuery | ep-contract-finance | ep-app-reporting | 应收账龄与应付账龄两张基础表的取数；分档定义不由本 trait 承载，取用入口见第 4.5 节 |
| ReconciliationItemQuery | ep-contract-finance | ep-app-ledger 中 9b 段实现的子账与总账勾稽 `ReconCheck` | 按法人与会计期间返回十项勾稽的子账侧合计，结构为 `ReconciliationItemView`；按裁定 B-08 与 G-01 该 `ReconCheck` 由 ep-platform-recon 的执行器驱动，执行器不直接依赖本 crate |
| SalesInvoiceQuery | ep-contract-invoice | ep-app-clm、ep-app-sales、ep-app-reporting | 销项发票与收款计划勾稽的只读查询，方法按裁定 C-16 固定为 `by_sales_order_line(tx, ctx, sales_order_line_id) -> Result<Vec<SalesInvoiceRef>, AppError>` |
| ReceiptPlanBillingQuery | ep-contract-invoice | ep-app-clm 的合同变更/履约投影与 `ContractTerminationReceiptPlanImpactRule` | `billing_by_period(tx,ctx,contract_id) -> Result<BTreeMap<i32,Money>,AppError>`；只读 invoice 分摊权威，按期次返回 `ISSUE-VOID-RED_LETTER` 净已开金额；clm 无副本、无写端口、无 Outbox 回写 |
| InvoiceReversalStatusQuery | ep-contract-invoice | ep-app-sales、ep-app-procure | 方法按裁定 C-16 固定为 `is_fully_credit_noted(tx, ctx, sales_order_line_id, quantity: Quantity) -> Result<CreditNoteStatus, AppError>`，供销售退货与采购退货的前置校验，对应 PRD 第 6.5.4；阶段 6 的 `InvoiceStatusPort` 作废；接线次序见第 0.5 节，与阶段 6 第三批同批接线同批验收，阶段 6 不注入任何替身 |
| ReceiptInvoiceMatchQueryPort | ep-contract-invoice | ep-app-procure | `lock_candidates_for_receipt_lines` 在 collect/reload 只返回物料收货行对应的原进项票/行 id，不算容量或金额；`match_state/match_states` 必须接收已 seal 的同一 proof 才返回物料退货锁后快照，`billed_allocations_for_purchase_invoice_lines` 以同一 proof 为已持久化原票行的 DROP_SHIP 退货返回同形快照。三种容量查询都逐原进项票/行返回 row_version 与数量、单价、税率、净税价税，供采购退货按原票分组调用 `PurchaseCreditNotePort`。其承载的采购退货已登记分支按第 0.5 节整条推迟到本阶段，阶段 7 不建该调用点也不注入任何替身，本阶段首次接线 |
| PurchaseCreditNotePort | ep-contract-invoice | ep-app-procure | `lock_candidates(tx,ctx,original_purchase_invoice_id,source_line_ids)` 无锁返回原票/原行及组合后的 finance owner 键；`register_credit_note(tx,ctx,f50_lock_proof:&TransactionLockProof,cmd)` 返回服务端生成的 `grni_reopened_effects` 与汇总金额。采购退货外层用例在 collect/reload 各调一次候选方法，取得一次 proof 后把同一引用传给红字、GRNI、库存、AP/settlement 全部 mutator；该分支按第 0.5 节同批首次接线，不注入替身 |
| TaxRateOptionQuery | ep-contract-invoice | ep-app-sales、ep-app-clm、ep-app-procure | 按裁定 C-11 提供 `default_rate(tx, ctx, legal_entity_id, item_id: uuid::Uuid) -> Result<Rate, AppError>` 与 `list(tx, ctx, legal_entity_id) -> Result<Vec<TaxRateOption>, AppError>`，是税率字典的唯一取用入口 |
十项勾稽中的存货与已收货未收票两项，其子账侧不由本节的 trait 承载，取数经 `ep_contract_inventory::StockValueSubledgerBalancePort` 与 `ep_contract_procure::GrniSubledgerBalancePort` 两个外部端口，两个端口分别由阶段 8 与阶段 7 在其自身阶段定义并实现，由 `ReconciliationItemQuery` 的实现在组装时调用，注入行由本阶段写入 `apps/core-server/src/wiring/` 与 `apps/job-worker/src/wiring/` 两个目录，见裁定 G-01。

进项红字的 current live 成本父链同样不新增本节 trait：唯一 owner 契约是阶段 11 `ep-contract-costing` 的 `PurchaseInvoiceCaptureReversalBasisQuery::lock_available(&self, tx:&mut dyn Tx, ctx:&SecurityContext, lines:Vec<PurchaseInvoiceCaptureLineRef>) -> Result<Vec<PurchaseInvoiceCaptureReversalBasis>,AppError>`。其 `PurchaseInvoiceOriginalCaptureKind::{DirectExpense,EstimatePriceDiffIssued}`、专用 `PurchaseInvoiceCostLiveFragment { root_entry_id,live_entry_id,available_amount:PositiveMoney,effect_sign:PurchaseInvoiceCostEffectSign::{DebitCost,CreditCost},role:ReturnCostRole,dimensions:DimensionSnapshot }` 以阶段 11 唯一代码块为规范源；不能复用只表达正向 live 成本的通用 `CostLiveFragment`，因为原票已出库价差可为负。Stage 10 只消费和接线，既不复制 DTO，也不把该外部 trait 计入 finance 10 + invoice 6。

##### 5.9.1 唯一 Rust ABI：ep-contract-finance

下面代码块是 `ep-contract-finance` 对外 ABI 的唯一规范源。`marker` 中的类型是本 contract crate 自有的零字段身份标记；跨模块稳定标记只从 foundation 的 22 项冻结清单导入，不在本 crate 重定义。所有金额/数量/单价仍使用 foundation 强类型，不以 `Decimal`、`f64` 或 HTTP 字符串代替。

```rust
use chrono::NaiveDate;
use ep_foundation::{AppError, Id, Money, Quantity, UnitPrice};
use ep_foundation::id::marker::{
    AccountingPeriod, Contract, Customer, DeliveryConfirmation, GoodsReceiptLine,
    LegalEntity, Material, PurchaseInvoice, PurchaseInvoiceLine, PurchaseOrder,
    SalesOrder, Supplier, Warehouse,
};
use ep_foundation::port::tx::{SnapshotCtx, TransactionLockProof, Tx};
use ep_foundation::security::SecurityContext;

pub mod marker {
    pub struct AdvancePaymentEntry;
    pub struct CashAccount;
    pub struct CashLedgerEntry;
    pub struct OverbillingEntry;
    pub struct OverbillingSettlement;
    pub struct PayableEntry;
    pub struct Payment;
    pub struct ReceivableEntry;
    pub struct Receipt;
}
use marker::*;

pub struct PeriodRange {
    pub from: NaiveDate,
    pub to: NaiveDate, // 两端均包含，且 from <= to
}

pub enum LedgerSide { Receivable, Payable }
pub enum LedgerEntryKind { Original, Reversal }
pub enum SettlementEffectDirection { Apply, Release }
pub enum UnbilledArDirection { Debit, Credit }
pub enum CashAccountType { Bank, Cash }
pub enum CashDirection { In, Out }
pub enum OverbillingStatus { Open, PartiallySettled, Settled }
pub enum OverbillingSettlementPath {
    PathOneReceiptMatch,
    PathTwoRedInvoice,
    PathThreeWriteOff,
}
pub enum FinanceSettlementLockSide { Payable, AdvancePayment }
pub enum FinanceReversalAccumulatorLockKind { Overbilling }

pub struct FinanceSettlementLockKey {
    pub side: FinanceSettlementLockSide,
    pub id: uuid::Uuid,
}

pub struct FinanceReversalAccumulatorLockKey {
    pub kind: FinanceReversalAccumulatorLockKind,
    pub source_id: uuid::Uuid,
}

pub struct PayableRegistrationLockScope {
    pub payable_entry_id: Id<PayableEntry>,
    pub purchase_invoice_id: Id<PurchaseInvoice>,
    pub supplier_id: Id<Supplier>,
    pub purchase_order_id: Option<Id<PurchaseOrder>>,
    pub contract_id: Option<Id<Contract>>,
    pub posting_date: NaiveDate,
    pub preallocated_overbilling_entry_ids: Vec<Id<OverbillingEntry>>,
}

pub struct PurchaseCreditNoteFinanceLockScope {
    pub original_purchase_invoice_id: Id<PurchaseInvoice>,
    pub source_purchase_invoice_line_ids: Vec<Id<PurchaseInvoiceLine>>,
}

pub enum PayableLockCandidateScope {
    RegisterPurchaseInvoice(PayableRegistrationLockScope),
    RegisterPurchaseCreditNote(PurchaseCreditNoteFinanceLockScope),
}

pub struct PayableRegistrationLockCandidates {
    pub ar_ap_original_entry_ids: Vec<Id<PayableEntry>>,
    pub advance_payment_entry_ids: Vec<Id<AdvancePaymentEntry>>,
    pub settlement_roots: Vec<FinanceSettlementLockKey>,
    pub settlement_effects: Vec<FinanceSettlementLockKey>,
    pub overbilling_entry_ids: Vec<Id<OverbillingEntry>>,
    pub finance_reversal_accumulators: Vec<FinanceReversalAccumulatorLockKey>,
}

pub struct PayableOverbillingLine {
    pub overbilling_entry_id: Id<OverbillingEntry>,
    pub purchase_invoice_line_id: Id<PurchaseInvoiceLine>,
    pub material_id: Id<Material>,
    pub warehouse_id: Option<Id<Warehouse>>,
    pub overbilled_quantity: Quantity,
    pub invoice_unit_price: UnitPrice,
    pub original_amount: Money,
}

pub struct RegisterPurchaseInvoicePayable {
    pub payable_entry_id: Id<PayableEntry>,
    pub purchase_invoice_id: Id<PurchaseInvoice>,
    pub supplier_id: Id<Supplier>,
    pub purchase_order_id: Option<Id<PurchaseOrder>>,
    pub contract_id: Option<Id<Contract>>,
    pub business_date: NaiveDate,
    pub due_date: NaiveDate,
    pub accounting_period_id: Id<AccountingPeriod>,
    pub accounting_period_seq: i32,
    pub deferred_from_period_id: Option<Id<AccountingPeriod>>,
    pub gross_amount: Money,
    pub overbilling_lines: Vec<PayableOverbillingLine>,
}

pub struct RegisteredOverbillingEntry {
    pub overbilling_entry_id: Id<OverbillingEntry>,
    pub purchase_invoice_line_id: Id<PurchaseInvoiceLine>,
    pub open_quantity: Quantity,
    pub open_amount: Money,
}

pub struct PayableRegistrationResult {
    pub payable_entry_id: Id<PayableEntry>,
    pub advance_auto_applied_amount: Money,
    pub overbilling_entries: Vec<RegisteredOverbillingEntry>,
}

#[async_trait::async_trait]
pub trait PayableRegistrationPort: Send + Sync {
    async fn lock_candidates(
        &self,
        tx: &mut dyn Tx,
        ctx: &SecurityContext,
        scope: &PayableLockCandidateScope,
    ) -> Result<PayableRegistrationLockCandidates, AppError>;

    async fn register_purchase_invoice(
        &self,
        tx: &mut dyn Tx,
        ctx: &SecurityContext,
        f50_lock_proof: &TransactionLockProof,
        cmd: RegisterPurchaseInvoicePayable,
    ) -> Result<PayableRegistrationResult, AppError>;
}

pub struct MatchOverbillingOnReceipt {
    pub goods_receipt_line_id: Id<GoodsReceiptLine>,
    pub purchase_order_id: Id<PurchaseOrder>,
    pub material_id: Id<Material>,
    pub warehouse_id: Id<Warehouse>,
    pub receipt_quantity: Quantity,
    pub business_date: NaiveDate,
    pub accounting_period_id: Id<AccountingPeriod>,
    pub accounting_period_seq: i32,
    pub deferred_from_period_id: Option<Id<AccountingPeriod>>,
}

pub struct ReceiptLineOverbillingSettlementSegment {
    pub goods_receipt_line_id: Id<GoodsReceiptLine>,
    pub overbilling_entry_id: Id<OverbillingEntry>,
    pub overbilling_settlement_id: Id<OverbillingSettlement>,
    pub matched_quantity: Quantity,
    pub invoice_unit_price: UnitPrice,
    pub settled_amount: Money,
    pub voucher_id: uuid::Uuid,
    pub accounting_period_id: Id<AccountingPeriod>,
    pub accounting_period_seq: i32,
    pub deferred_from_period_id: Option<Id<AccountingPeriod>>,
}

pub struct ReceiptLineOverbillingMatch {
    pub goods_receipt_line_id: Id<GoodsReceiptLine>,
    pub matched_quantity: Quantity,
    pub segments: Vec<ReceiptLineOverbillingSettlementSegment>,
}

#[async_trait::async_trait]
pub trait OverbillingMatchPort: Send + Sync {
    async fn candidate_entry_ids_for_receipt(
        &self,
        tx: &mut dyn Tx,
        ctx: &SecurityContext,
        purchase_order_id: Id<PurchaseOrder>,
        material_id: Id<Material>,
        warehouse_id: Id<Warehouse>,
    ) -> Result<Vec<Id<OverbillingEntry>>, AppError>;

    async fn match_on_receipt(
        &self,
        tx: &mut dyn Tx,
        ctx: &SecurityContext,
        f50_lock_proof: &TransactionLockProof,
        cmd: MatchOverbillingOnReceipt,
    ) -> Result<ReceiptLineOverbillingMatch, AppError>;

    async fn settlement_segments_by_receipt_lines(
        &self,
        snapshot: &dyn SnapshotCtx,
        legal_entity_id: Id<LegalEntity>,
        goods_receipt_line_ids: &[Id<GoodsReceiptLine>],
    ) -> Result<Vec<ReceiptLineOverbillingSettlementSegment>, AppError>;
}

pub struct DeliveryUnbilledArCommand {
    pub delivery_confirmation_id: Id<DeliveryConfirmation>,
    pub customer_id: Id<Customer>,
    pub posting_date: NaiveDate,
    pub accounting_period_id: Id<AccountingPeriod>,
    pub accounting_period_seq: i32,
    pub deferred_from_period_id: Option<Id<AccountingPeriod>>,
    pub voucher_id: uuid::Uuid,
    pub direction: UnbilledArDirection,
    pub net_amount: Money,
    pub gross_amount: Money,
}

pub struct SalesReturnUnbilledArCommand {
    pub sales_return_id: uuid::Uuid,
    pub customer_id: Id<Customer>,
    pub posting_date: NaiveDate,
    pub accounting_period_id: Id<AccountingPeriod>,
    pub accounting_period_seq: i32,
    pub deferred_from_period_id: Option<Id<AccountingPeriod>>,
    pub voucher_id: uuid::Uuid,
    pub direction: UnbilledArDirection,
    pub net_amount: Money,
    pub gross_amount: Money,
}

#[async_trait::async_trait]
pub trait UnbilledArPort: Send + Sync {
    async fn record_on_delivery(
        &self,
        tx: &mut dyn Tx,
        ctx: &SecurityContext,
        cmd: DeliveryUnbilledArCommand,
    ) -> Result<(), AppError>;

    async fn record_on_sales_return(
        &self,
        tx: &mut dyn Tx,
        ctx: &SecurityContext,
        cmd: SalesReturnUnbilledArCommand,
    ) -> Result<(), AppError>;
}

pub struct ReceivableExposureView {
    pub receivable_open_amount: Money,
    pub delivered_unbilled_gross_amount: Money,
}

#[async_trait::async_trait]
pub trait ReceivableExposureQuery: Send + Sync {
    async fn exposure(
        &self,
        tx: &mut dyn Tx,
        ctx: &SecurityContext,
        customer_id: Id<Customer>,
    ) -> Result<ReceivableExposureView, AppError>;
}

pub struct LedgerCursor {
    pub business_date: NaiveDate,
    pub doc_no: String,
    pub entry_id: uuid::Uuid,
}

pub struct ReceivableLedgerQueryInput {
    pub legal_entity_id: Id<LegalEntity>,
    pub customer_id: Option<Id<Customer>>,
    pub contract_id: Option<Id<Contract>>,
    pub sales_order_id: Option<Id<SalesOrder>>,
    pub entry_ids: Option<Vec<Id<ReceivableEntry>>>,
    pub period: Option<PeriodRange>,
    pub after: Option<LedgerCursor>,
    pub limit: u16,
}

pub struct ReceivableLedgerEntryView {
    pub entry_id: Id<ReceivableEntry>,
    pub customer_id: Id<Customer>,
    pub contract_id: Option<Id<Contract>>,
    pub sales_order_id: Option<Id<SalesOrder>>,
    pub entry_kind: LedgerEntryKind,
    pub source_doc_type: String,
    pub source_doc_id: uuid::Uuid,
    pub doc_no: String,
    pub business_date: NaiveDate,
    pub due_date: NaiveDate,
    pub accounting_period_id: Id<AccountingPeriod>,
    pub original_amount: Money,
    pub effective_open: Money,
}

pub struct ReceivableLedgerPage {
    pub items: Vec<ReceivableLedgerEntryView>,
    pub next: Option<LedgerCursor>,
}

pub struct SettlementEffectView {
    pub effect_id: uuid::Uuid,
    pub entry_id: uuid::Uuid,
    pub direction: SettlementEffectDirection,
    pub amount: Money,
    pub business_date: NaiveDate,
    pub source_doc_type: String,
    pub source_doc_id: uuid::Uuid,
    pub source_doc_no: String,
}

#[async_trait::async_trait]
pub trait ReceivableLedgerQuery: Send + Sync {
    async fn entries(
        &self,
        tx: &mut dyn Tx,
        ctx: &SecurityContext,
        query: ReceivableLedgerQueryInput,
    ) -> Result<ReceivableLedgerPage, AppError>;

    async fn settlement_effects(
        &self,
        tx: &mut dyn Tx,
        ctx: &SecurityContext,
        receivable_entry_id: Id<ReceivableEntry>,
    ) -> Result<Vec<SettlementEffectView>, AppError>;
}

pub struct PayableLedgerQueryInput {
    pub legal_entity_id: Id<LegalEntity>,
    pub supplier_id: Option<Id<Supplier>>,
    pub contract_id: Option<Id<Contract>>,
    pub purchase_order_id: Option<Id<PurchaseOrder>>,
    pub entry_ids: Option<Vec<Id<PayableEntry>>>,
    pub period: Option<PeriodRange>,
    pub after: Option<LedgerCursor>,
    pub limit: u16,
}

pub struct PayableLedgerEntryView {
    pub entry_id: Id<PayableEntry>,
    pub supplier_id: Id<Supplier>,
    pub contract_id: Option<Id<Contract>>,
    pub purchase_order_id: Option<Id<PurchaseOrder>>,
    pub purchase_invoice_id: Option<Id<PurchaseInvoice>>,
    pub entry_kind: LedgerEntryKind,
    pub source_doc_type: String,
    pub source_doc_id: uuid::Uuid,
    pub doc_no: String,
    pub business_date: NaiveDate,
    pub due_date: NaiveDate,
    pub accounting_period_id: Id<AccountingPeriod>,
    pub original_amount: Money,
    pub effective_open: Money,
}

pub struct PayableLedgerPage {
    pub items: Vec<PayableLedgerEntryView>,
    pub next: Option<LedgerCursor>,
}

#[async_trait::async_trait]
pub trait PayableLedgerQuery: Send + Sync {
    async fn open_balance(
        &self,
        tx: &mut dyn Tx,
        ctx: &SecurityContext,
        purchase_invoice_id: Id<PurchaseInvoice>,
    ) -> Result<Money, AppError>;

    async fn entries(
        &self,
        tx: &mut dyn Tx,
        ctx: &SecurityContext,
        query: PayableLedgerQueryInput,
    ) -> Result<PayableLedgerPage, AppError>;

    async fn settlement_effects(
        &self,
        tx: &mut dyn Tx,
        ctx: &SecurityContext,
        payable_entry_id: Id<PayableEntry>,
    ) -> Result<Vec<SettlementEffectView>, AppError>;
}

pub struct SupplierStatementInvoiceLine {
    pub purchase_invoice_id: Id<PurchaseInvoice>,
    pub doc_no: String,
    pub business_date: NaiveDate,
    pub due_date: NaiveDate,
    pub gross_amount: Money,
    pub effective_open: Money,
}

pub struct SupplierStatementPaymentLine {
    pub payment_id: Id<Payment>,
    pub doc_no: String,
    pub business_date: NaiveDate,
    pub amount: Money,
}

pub struct SupplierStatementView {
    pub supplier_id: Id<Supplier>,
    pub period: PeriodRange,
    pub opening_effective_open: Money,
    pub invoiced_amount: Money,
    pub paid_amount: Money,
    pub closing_effective_open: Money,
    pub invoices: Vec<SupplierStatementInvoiceLine>,
    pub payments: Vec<SupplierStatementPaymentLine>,
}

#[async_trait::async_trait]
pub trait SupplierStatementQuery: Send + Sync {
    async fn statement(
        &self,
        tx: &mut dyn Tx,
        ctx: &SecurityContext,
        supplier_id: Id<Supplier>,
        period: PeriodRange,
    ) -> Result<SupplierStatementView, AppError>;
}

pub struct CashAccountView {
    pub cash_account_id: Id<CashAccount>,
    pub code: String,
    pub account_name: String,
    pub account_type: CashAccountType,
    pub ledger_account_id: uuid::Uuid,
    pub is_active: bool,
    pub opening_balance: Money,
    pub opening_balance_period_id: Option<Id<AccountingPeriod>>,
}

pub struct CashLedgerQueryInput {
    pub legal_entity_id: Id<LegalEntity>,
    pub cash_account_id: Option<Id<CashAccount>>,
    pub period: PeriodRange,
    pub after: Option<LedgerCursor>,
    pub limit: u16,
}

pub struct CashLedgerEntryView {
    pub cash_ledger_entry_id: Id<CashLedgerEntry>,
    pub cash_account_id: Id<CashAccount>,
    pub direction: CashDirection,
    pub amount: Money,
    pub source_doc_type: String,
    pub source_doc_id: uuid::Uuid,
    pub doc_no: String,
    pub business_date: NaiveDate,
    pub accounting_period_id: Id<AccountingPeriod>,
    pub voucher_id: uuid::Uuid,
}

pub struct CashLedgerPage {
    pub items: Vec<CashLedgerEntryView>,
    pub next: Option<LedgerCursor>,
}

#[async_trait::async_trait]
pub trait CashAccountQuery: Send + Sync {
    async fn account(
        &self,
        tx: &mut dyn Tx,
        ctx: &SecurityContext,
        cash_account_id: Id<CashAccount>,
    ) -> Result<CashAccountView, AppError>;

    async fn ledger_entries(
        &self,
        tx: &mut dyn Tx,
        ctx: &SecurityContext,
        query: CashLedgerQueryInput,
    ) -> Result<CashLedgerPage, AppError>;
}

pub struct AgingQueryInput {
    pub legal_entity_id: Id<LegalEntity>,
    pub side: LedgerSide,
    pub as_of_date: NaiveDate,
    pub bucket_profile_code: Option<String>,
    pub after: Option<LedgerCursor>,
    pub limit: u16,
}

pub struct AgingItemView {
    pub ledger_entry_id: uuid::Uuid,
    pub counterparty_id: uuid::Uuid,
    pub contract_id: Option<Id<Contract>>,
    pub sales_order_id: Option<Id<SalesOrder>>,
    pub purchase_order_id: Option<Id<PurchaseOrder>>,
    pub source_doc_type: String,
    pub source_doc_id: uuid::Uuid,
    pub doc_no: String,
    pub due_date: NaiveDate,
    pub overdue_days: i32,
    pub bucket_code: String,
    pub bucket_sort_no: i32,
    pub effective_open: Money,
}

pub struct AgingSnapshot {
    pub side: LedgerSide,
    pub as_of_date: NaiveDate,
    pub bucket_profile_code: String,
    pub items: Vec<AgingItemView>,
    pub anomalies: Vec<AgingItemView>,
    pub next: Option<LedgerCursor>,
}

#[async_trait::async_trait]
pub trait AgingQuery: Send + Sync {
    async fn snapshot(
        &self,
        tx: &mut dyn Tx,
        ctx: &SecurityContext,
        query: AgingQueryInput,
    ) -> Result<AgingSnapshot, AppError>;
}

pub enum ReconciliationItemCode {
    Receivable,
    Payable,
    AdvanceReceipt,
    AdvancePayment,
    UnbilledAr,
    Overbilling,
    CashBank,
    CashOnHand,
    Inventory,
    Grni,
}

pub struct ReconciliationItemView {
    pub item_code: ReconciliationItemCode,
    pub legal_entity_id: Id<LegalEntity>,
    pub accounting_period_id: Id<AccountingPeriod>,
    pub subsidiary_amount: Money,
}

#[async_trait::async_trait]
pub trait ReconciliationItemQuery: Send + Sync {
    async fn items(
        &self,
        snapshot: &dyn SnapshotCtx,
        legal_entity_id: Id<LegalEntity>,
        accounting_period_id: Id<AccountingPeriod>,
        accounting_period_seq: i32,
    ) -> Result<Vec<ReconciliationItemView>, AppError>;
}
```

`PayableRegistrationPort::lock_candidates` 是 finance owner 唯一的计划发现入口，只执行普通 `SELECT`，不得取行锁/advisory lock、计算金额或容量、生成业务事实。`RegisterPurchaseInvoice` scope 的 `preallocated_overbilling_entry_ids` 在解析请求后由调用方预生成，必须无重复；返回的 `ar_ap_original_entry_ids` 恰含预生成 payable id，`overbilling_entry_ids` 恰含这组预生成 id，其余字段只含同法人、同供应商、同合同/采购订单且在 `posting_date` 有效的现存预付及其完整核销祖先图。`RegisterPurchaseCreditNote` scope 的来源行数组必须非空、无重复，返回原进项票的唯一 AP ORIGINAL、相关预付、完整 settlement 根/效果以及该票现存 overbilling。两个 scope 的返回均按以下唯一顺序排列且无重复：AP、advance、overbilling 各按 UUID bytes ASC；`settlement_roots/effects` 按 `side` 声明顺序再按 id ASC；`finance_reversal_accumulators` 按 `kind` 声明顺序再按 `source_id` ASC，其中当前唯一 kind 为 `Overbilling` 且 source 集合逐项等于 `overbilling_entry_ids`。无现存 advance/settlement/overbilling 时对应数组为空，不伪造行。外层在 coordinator 前及全类别锁完后用完全相同 scope 各调一次，并分别构造 collected/reloaded plan；两次规范化结果不等时只允许 `seal_after_reload` 以 SQLSTATE 40001 中止重试。`register_purchase_invoice` 收到的 payable/contract/overbilling 身份必须逐项等于 sealed scope；`PayableOverbillingLine.overbilling_entry_id` 由调用方预生成而非 finance 隐式换号，finance 只在 proof 后决定零值不建或写入对应行。

`OverbillingMatchPort` 的三个方法使用同一真实 finance owner 实现，不允许 job-worker 自建影子 DTO 或查询 finance 表。`candidate_entry_ids_for_receipt` 只读同法人、同采购订单、同物料、`warehouse_id IS NULL OR warehouse_id=请求仓库` 且 `open_quantity>0` 的候选，按 `overbilling_entry_id UUID bytes ASC` 返回；零命中合法为空。收货外层在 collect 与全部类别锁后的 reload 各调一次，把规范化 id 全部映射为 `FinanceReversalAccumulatorLockKind::Overbilling`/超量挂账所需键，集合漂移只由 seal 走 40001。`match_on_receipt` 首句以 `CrossModuleLockCoordinator::assert_covers(tx,ctx,f50_lock_proof,required_subset)` 验证 proof，然后只重读 proof 已覆盖行，不执行 `FOR UPDATE` 或中途补锁。返回 `segments` 按 `overbilling_entry_id UUID bytes ASC,overbilling_settlement_id UUID bytes ASC`，`matched_quantity=sum(segments.matched_quantity)`，每段 `settled_amount` 等于本端口服务端定标金额，零命中合法返回 `matched_quantity=0,segments=[]`。路径一所需凭证与 `finance.overbilling_settlements` 由 finance 实现在本方法内同事务生成，所以每个返回段的 settlement/voucher/期间都非空；后续库存或采购写入失败时整笔随调用方事务回滚。快照方法不接 proof：空入参直接返回空数组且不访问数据库；非空结果按 `goods_receipt_line_id UUID bytes ASC,overbilling_entry_id UUID bytes ASC,overbilling_settlement_id UUID bytes ASC` 排序，仅返回尚未被 `reverses_id` 冲回的有效路径一段，R-PROC-05 以该集合与库存计价段逐键比较。

`UnbilledArPort` 两个方法只承接 Stage 6 已经获得 `PostingOutcome::Posted` 的非零收入分支。`record_on_delivery` 固定 `Debit`，`record_on_sales_return` 固定 `Credit`；两者都要求 `net_amount>0`、`gross_amount>0`、`gross_amount>=net_amount`、非 nil `voucher_id` 及与调用方唯一 `ResolvedPeriod` 逐值相等的三项期间字段，反向错、零额、空 voucher 或期间不等均在插入前失败关闭。Stage 6 合法全零分支只接受 `PostingOutcome::Skipped`，交付头/退货头 `voucher_id=None`，不调用本 trait、不写零额 `unbilled_ar_entries`；其中销售退货即使合法库存回收金额非零，只要 revenue/gross 同为零也仍不调用本 trait。

其余 finance ABI 的集合语义冻结如下：`PeriodRange` 两端包含，`from>to` 为 `VALIDATION`；分页 `limit` 仅允许 1 至 200；两个 ledger query 的 `entry_ids=Some(ids)` 要求 1 至 200 个互异 id，并与其余过滤条件取交集，`None` 表示不按 id 过滤，禁止用 `Some([])` 表达全量。ledger 页按 `business_date DESC,doc_no DESC,entry_id UUID bytes DESC`，其 `next` 是最后一行的稳定游标；核销效果按 `business_date ASC,effect_id UUID bytes ASC`；供应商对账的发票、付款分别按 `business_date ASC,doc_no ASC,id ASC`，`closing_effective_open=opening_effective_open+invoiced_amount-paid_amount` 且还必须等于期间末权威 `effective_open`；资金腿按 `business_date DESC,doc_no DESC,id DESC`。账龄的 `bucket_profile_code=None` 取同法人、同 side 的唯一 active default，`Some(code)` 取具名 active profile；不存在、重复默认、断档或重叠原样传播阶段 11 的具名错误，不私自回退另一套分档。正常项按 `bucket_sort_no ASC,due_date ASC,doc_no ASC,ledger_entry_id ASC`，负 `effective_open` 只进 `anomalies`；返回的 `bucket_profile_code` 是实际采用的 code，维度 id 逐项来自同一 ORIGINAL 台账行。查询无数据一律返回空集合/零合计，不返回 404；指定对象本应存在却不存在、跨法人或不可见统一 `PLATFORM.AUTHZ.NOT_FOUND_OR_DENIED`。`ReconciliationItemQuery::items` 恰返回上述枚举顺序的十个互异项目，全部行法人和期间等于请求；缺项、重复项、额外项或负的不允许余额以对账框架内部契约错误失败关闭。finance 只返回 `subsidiary_amount`，`ledger_amount/difference` 禁止进入本 trait 或 DTO。

##### 5.9.2 唯一 Rust ABI：ep-contract-invoice

下面代码块是 `ep-contract-invoice` 对外 ABI 的唯一规范源，并同时定义 `InvoiceIdentifierInput`、`InvoiceReversalLineInput` 与 `PurchaseCreditNotePort` 所引用的全部类型。第 4.11 节只规定业务算法，不再复制第二份端口代码。

```rust
use std::collections::BTreeMap;
use chrono::NaiveDate;
use ep_foundation::{AppError, Id, Money, Quantity, Rate, UnitPrice};
use ep_foundation::id::marker::{
    AccountingPeriod, Contract, GoodsReceiptLine, LegalEntity, PurchaseInvoice,
    PurchaseInvoiceLine, SalesOrderLine, Supplier,
};
use ep_foundation::port::tx::{TransactionLockProof, Tx};
use ep_foundation::security::SecurityContext;

pub mod marker {
    pub struct InvoiceNumberRegistry;
    pub struct InvoiceReversal;
    pub struct InvoiceReversalLine;
    pub struct SalesInvoice;
    pub struct SalesInvoiceLine;
    pub struct TaxRateOptionRecord;
}
use marker::*;

pub enum InvoiceMedium { Electronic, Paper }
pub enum NumberScheme { Unified20, LegacyCodeNumber }
pub enum QuantityEffectKind { Reduce, None }
pub enum PricingEffectKind { OriginalUnitPrice, Adjusted }
pub enum SalesInvoiceStatus { Issued, PartiallyRedReversed, Voided, RedReversed }

pub struct InvoiceIdentifierInput {
    pub invoice_medium: InvoiceMedium,
    pub number_scheme: NumberScheme,
    pub invoice_code: Option<String>,
    pub invoice_no: String,
}

pub struct InvoiceReversalLineInput {
    pub source_sales_invoice_line_id: Option<Id<SalesInvoiceLine>>,
    pub source_purchase_invoice_line_id: Option<Id<PurchaseInvoiceLine>>,
    pub quantity_effect_kind: QuantityEffectKind,
    pub pricing_effect_kind: PricingEffectKind,
    pub quantity: Quantity,
    pub tax_rate: Rate,
    pub net_amount: Money,
    pub tax_amount: Money,
    pub gross_amount: Money,
}

pub struct SalesInvoiceRef {
    pub sales_invoice_id: Id<SalesInvoice>,
    pub sales_invoice_line_id: Id<SalesInvoiceLine>,
    pub sales_order_line_id: Id<SalesOrderLine>,
    pub doc_no: String,
    pub status: SalesInvoiceStatus,
    pub issue_date: NaiveDate,
    pub invoiced_quantity: Quantity,
    pub credit_noted_quantity: Quantity,
    pub remaining_quantity: Quantity,
    pub original_gross_amount: Money,
    pub remaining_gross_amount: Money,
}

pub struct CreditNoteStatus {
    pub requested_quantity: Quantity,
    pub invoiced_quantity: Quantity,
    pub credit_noted_quantity: Quantity,
    pub is_fully_credit_noted: bool,
    pub pending_invoices: Vec<SalesInvoiceRef>,
}

#[async_trait::async_trait]
pub trait SalesInvoiceQuery: Send + Sync {
    async fn by_sales_order_line(
        &self,
        tx: &mut dyn Tx,
        ctx: &SecurityContext,
        sales_order_line_id: Id<SalesOrderLine>,
    ) -> Result<Vec<SalesInvoiceRef>, AppError>;
}

#[async_trait::async_trait]
pub trait ReceiptPlanBillingQuery: Send + Sync {
    async fn billing_by_period(
        &self,
        tx: &mut dyn Tx,
        ctx: &SecurityContext,
        contract_id: Id<Contract>,
    ) -> Result<BTreeMap<i32, Money>, AppError>;
}

#[async_trait::async_trait]
pub trait InvoiceReversalStatusQuery: Send + Sync {
    async fn is_fully_credit_noted(
        &self,
        tx: &mut dyn Tx,
        ctx: &SecurityContext,
        sales_order_line_id: Id<SalesOrderLine>,
        quantity: Quantity,
    ) -> Result<CreditNoteStatus, AppError>;
}

pub struct ReceiptInvoiceMatchLockCandidate {
    pub goods_receipt_line_id: Id<GoodsReceiptLine>,
    pub original_purchase_invoice_id: Id<PurchaseInvoice>,
    pub original_purchase_invoice_line_id: Id<PurchaseInvoiceLine>,
}

pub struct BilledReturnAllocation {
    pub original_purchase_invoice_id: Id<PurchaseInvoice>,
    pub original_purchase_invoice_row_version: i64,
    pub original_purchase_invoice_line_id: Id<PurchaseInvoiceLine>,
    pub posting_date: NaiveDate,
    pub returnable_quantity: Quantity,
    pub original_unit_price: UnitPrice,
    pub tax_rate: Rate,
    pub returnable_net_amount: Money,
    pub returnable_tax_amount: Money,
    pub returnable_gross_amount: Money,
}

pub struct ReceiptInvoiceMatchState {
    pub goods_receipt_line_id: Id<GoodsReceiptLine>,
    pub unbilled_returnable_quantity: Quantity,
    pub billed_allocations: Vec<BilledReturnAllocation>,
}

pub enum PurchaseCreditNoteSettlementLockSide { Payable, AdvancePayment }
pub enum PurchaseCreditNoteFinanceAccumulatorLockKind { Overbilling }

pub struct PurchaseCreditNoteSettlementLockKey {
    pub side: PurchaseCreditNoteSettlementLockSide,
    pub id: uuid::Uuid,
}

pub struct PurchaseCreditNoteFinanceAccumulatorLockKey {
    pub kind: PurchaseCreditNoteFinanceAccumulatorLockKind,
    pub source_id: uuid::Uuid,
}

pub struct PurchaseCreditNoteLockCandidates {
    pub original_purchase_invoice_id: Id<PurchaseInvoice>,
    pub original_purchase_invoice_line_ids: Vec<Id<PurchaseInvoiceLine>>,
    pub payable_original_entry_ids: Vec<uuid::Uuid>,
    pub advance_payment_entry_ids: Vec<uuid::Uuid>,
    pub settlement_roots: Vec<PurchaseCreditNoteSettlementLockKey>,
    pub settlement_effects: Vec<PurchaseCreditNoteSettlementLockKey>,
    pub invoice_reversal_accumulator_source_line_ids: Vec<Id<PurchaseInvoiceLine>>,
    pub finance_reversal_accumulators: Vec<PurchaseCreditNoteFinanceAccumulatorLockKey>,
    pub overbilling_entry_ids: Vec<uuid::Uuid>,
}

#[async_trait::async_trait]
pub trait ReceiptInvoiceMatchQueryPort: Send + Sync {
    async fn lock_candidates_for_receipt_lines(
        &self,
        tx: &mut dyn Tx,
        ctx: &SecurityContext,
        goods_receipt_line_ids: &[Id<GoodsReceiptLine>],
    ) -> Result<Vec<ReceiptInvoiceMatchLockCandidate>, AppError>;

    async fn match_state(
        &self,
        tx: &mut dyn Tx,
        ctx: &SecurityContext,
        f50_lock_proof: &TransactionLockProof,
        goods_receipt_line_id: Id<GoodsReceiptLine>,
    ) -> Result<ReceiptInvoiceMatchState, AppError>;

    async fn match_states(
        &self,
        tx: &mut dyn Tx,
        ctx: &SecurityContext,
        f50_lock_proof: &TransactionLockProof,
        goods_receipt_line_ids: &[Id<GoodsReceiptLine>],
    ) -> Result<Vec<ReceiptInvoiceMatchState>, AppError>;

    async fn billed_allocations_for_purchase_invoice_lines(
        &self,
        tx: &mut dyn Tx,
        ctx: &SecurityContext,
        f50_lock_proof: &TransactionLockProof,
        source_line_ids: &[Id<PurchaseInvoiceLine>],
    ) -> Result<Vec<BilledReturnAllocation>, AppError>;
}

pub struct RegisterPurchaseCreditNote {
    pub supplier_id: Id<Supplier>,
    pub original_purchase_invoice_id: Id<PurchaseInvoice>,
    pub linked_purchase_return_id: Option<uuid::Uuid>,
    pub identifier: InvoiceIdentifierInput,
    pub posting_date: NaiveDate,
    pub expected_original_row_version: i64,
    pub is_for_overbilling_settlement: bool,
    pub lines: Vec<InvoiceReversalLineInput>,
}

pub struct PurchaseCreditNoteGrniReopen {
    pub invoice_reversal_line_id: Id<InvoiceReversalLine>,
    pub original_purchase_invoice_line_id: Id<PurchaseInvoiceLine>,
    pub goods_receipt_line_id: Id<GoodsReceiptLine>,
    pub grni_effect_id: uuid::Uuid,
    pub quantity: Quantity,
    pub amount: Money,
}

pub struct PurchaseCreditNoteView {
    pub invoice_reversal_id: Id<InvoiceReversal>,
    pub doc_no: String,
    pub net_amount: Money,
    pub tax_amount: Money,
    pub gross_amount: Money,
    pub linked_purchase_return_id: Option<uuid::Uuid>,
    pub grni_reopened_effects: Vec<PurchaseCreditNoteGrniReopen>,
    pub grni_reopened_amount: Money,
    pub voucher_id: uuid::Uuid,
    pub accounting_period_id: Id<AccountingPeriod>,
    pub accounting_period_seq: i32,
    pub deferred_from_period_id: Option<Id<AccountingPeriod>>,
}

#[async_trait::async_trait]
pub trait PurchaseCreditNotePort: Send + Sync {
    async fn lock_candidates(
        &self,
        tx: &mut dyn Tx,
        ctx: &SecurityContext,
        original_purchase_invoice_id: Id<PurchaseInvoice>,
        source_line_ids: &[Id<PurchaseInvoiceLine>],
    ) -> Result<PurchaseCreditNoteLockCandidates, AppError>;

    async fn register_credit_note(
        &self,
        tx: &mut dyn Tx,
        ctx: &SecurityContext,
        f50_lock_proof: &TransactionLockProof,
        cmd: RegisterPurchaseCreditNote,
    ) -> Result<PurchaseCreditNoteView, AppError>;
}

pub struct TaxRateOption {
    pub tax_rate_option_id: Id<TaxRateOptionRecord>,
    pub tax_rate: Rate,
    pub display_name: String,
    pub sort_no: i32,
}

#[async_trait::async_trait]
pub trait TaxRateOptionQuery: Send + Sync {
    async fn default_rate(
        &self,
        tx: &mut dyn Tx,
        ctx: &SecurityContext,
        legal_entity_id: Id<LegalEntity>,
        item_id: uuid::Uuid,
    ) -> Result<Rate, AppError>;

    async fn list(
        &self,
        tx: &mut dyn Tx,
        ctx: &SecurityContext,
        legal_entity_id: Id<LegalEntity>,
    ) -> Result<Vec<TaxRateOption>, AppError>;
}
```

`ReceiptInvoiceMatchQueryPort::lock_candidates_for_receipt_lines` 是 invoice owner 给物料采购退货外层的唯一 proof 前发现入口，只执行无锁标识查询，不读取或返回数量、金额、row_version、状态或任何容量结论。空入参不访问数据库并返回空数组；非空入参不得重复，结果枚举这些同法人收货行曾对应的全部进项原票行，不以今天的剩余量/状态过滤，按 `goods_receipt_line_id UUID bytes ASC,original_purchase_invoice_id UUID bytes ASC,original_purchase_invoice_line_id UUID bytes ASC` 排序且无重复。采购外层在 collect 与全部全局类别锁完后的 reload 各调用一次；DROP_SHIP 则直接使用既有退货行已持久化的 `(purchase_invoice_id,purchase_invoice_line_id)`。两路都按原票 id 分组调用 `PurchaseCreditNotePort::lock_candidates` 并构造 collected/reloaded plan，规范化集合漂移只由 seal 走 SQLSTATE 40001。`match_state`、`match_states` 与 `billed_allocations_for_purchase_invoice_lines` 都必须传 seal 后的同一 `TransactionLockProof`；实现首句验证 proof 覆盖请求收货行或来源行及其全部原票头行，随后只重读已锁图来计算 returnable 数量/金额与 row_version，不执行 `FOR UPDATE`、advisory lock 或补 plan。proof 缺失、异事务/法人或覆盖不全失败关闭，proof 前不得调用这三种容量查询。

`billed_allocations_for_purchase_invoice_lines` 专用于 DROP_SHIP/直接费用退货：空入参不访问数据库并返回空数组；非空入参必须无重复且每个 id 都来自同法人、同一已 seal 退货图，成功时恰返回一项/输入来源行，并按 `posting_date ASC,original_purchase_invoice_id UUID bytes ASC,original_purchase_invoice_line_id UUID bytes ASC` 排序。每项完整返回原票 id、原票行 id、统一的原票 row_version、正的可退数量、原单价、税率及可退 net/tax/gross；没有正容量、错供应商/直运祖先或输入行不可见时失败而不以缺项伪装成功。结果 source line 键集合必须与输入逐项相等，调用方再按原票分组且每票恰生成一张新红字。

invoice 查询的确定性与空集语义冻结如下：`SalesInvoiceQuery::by_sales_order_line` 按 `issue_date ASC,sales_invoice_id UUID bytes ASC,sales_invoice_line_id UUID bytes ASC` 返回该订单行的全部蓝票行及现有冲销后的剩余量/额，无票合法返回空数组；`InvoiceReversalStatusQuery` 使用同一顺序把请求数量分配到发票行，`invoiced_quantity` 只计请求范围内实际已开部分，未开票部分不要求红冲。无票时返回 `is_fully_credit_noted=true,pending_invoices=[]`；凡请求范围内已开部分仍有未冲数量，返回 false 且 `pending_invoices` 只含未闭合行并保持上述顺序。`ReceiptPlanBillingQuery` 的 BTreeMap 键只能是正整数期次且金额不得为负，空合同分摊合法返回空 map。`ReceiptInvoiceMatchQueryPort::match_states` 的空入参不访问数据库并返回空数组；非空入参不得重复，返回恰好一行/输入 id 并按 `goods_receipt_line_id UUID bytes ASC`。每行 `billed_allocations` 按 `posting_date ASC,original_purchase_invoice_id UUID bytes ASC,original_purchase_invoice_line_id UUID bytes ASC`，同一原票各项 `original_purchase_invoice_row_version` 必须一致；每项数量严格大于零，`net+tax=gross`，金额/税率/原单价是锁后原票行剩余可冲快照，累计不得超过该行剩余量额。`unbilled_returnable_quantity + sum(billed_allocations.returnable_quantity)` 等于该收货行锁后可退数量。采购退货先消费 unbilled，不足部分按该稳定顺序消费 allocation，再按 `original_purchase_invoice_id` 分组；每组以同一个 row_version 调一次 `PurchaseCreditNotePort`，禁止把多张原票行塞进一个命令。`RegisterPurchaseCreditNote.identifier` 是本次新红字票的法定号码，不是原蓝票号；Stage 7 采购退货 POST 的应用层请求固定含 `credit_note_identifiers: Vec<PurchaseCreditNoteIdentifierInput>`，其元素固定为 `{ original_purchase_invoice_id: Id<PurchaseInvoice>, identifier: ep_contract_invoice::InvoiceIdentifierInput }`。列表按原票 id UUID bytes ASC、id 唯一，且必须与锁后实际消费的 distinct billed 原票键集恰好相等；未开票-only 必须为空，缺、多、重复都按 `VALIDATION` 零写入拒绝。该类型只属于 `ep-app-procure`/HTTP 边界，不放入 `ep-contract-procure`，从而不制造 contract→contract 依赖；procure 只按 id 把对应的新号码 move 进该组的命令，不猜号、不复用原号、不查 invoice schema。`TaxRateOptionQuery::list` 只返回启用项，按 `sort_no ASC,tax_rate ASC,id ASC`；`default_rate` 返回主数据默认值且该值必须存在于同法人启用清单，否则按内部不变量失败关闭，不擅自回退 13% 或零税率。

`PurchaseCreditNotePort::lock_candidates` 同样只读无锁，不占号码、不计算剩余量/金额、不写 invoice 或 finance 事实。`source_line_ids` 必须非空且无重复，每个 id 都必须属于请求的同法人原进项票；输出 `original_purchase_invoice_id` 必须逐字等于入参，`original_purchase_invoice_line_ids` 与 `invoice_reversal_accumulator_source_line_ids` 都恰为输入集合按 UUID bytes ASC 规范化后的结果。ep-app-invoice 经注入的 finance owner 候选入口组合 AP original、advance、settlement 与 overbilling，不跨 schema SQL；`payable_original_entry_ids`、`advance_payment_entry_ids`、`overbilling_entry_ids` 各按 UUID bytes ASC，`settlement_roots/effects` 按 `side` 声明顺序再按 id ASC，`finance_reversal_accumulators` 按 `kind` 声明顺序再按 `source_id` ASC，且 `Overbilling` source 集合逐项等于 `overbilling_entry_ids`。原票没有相关 finance 图时相应数组为空，但原票头/请求行与 invoice accumulator 仍必须完整返回；指定原票或行不存在/不可见不得伪装成空集合。采购退货或独立进项更正外层在 coordinator 前及全部类别锁后用相同入参各调一次，分别构造 collected/reloaded plan；两次规范化集合不等只允许 `seal_after_reload` 走 SQLSTATE 40001，取得 proof 前禁止冲销容量计算。`register_credit_note` 首句验证 proof 覆盖该 DTO 全集，随后按命令中的 `expected_original_row_version` 重读并校验原票版本，不补锁。

两个 contract crate 的共同失败语义：`tx.legal_entity_id()`/`snapshot.legal_entity_id()`、显式 `legal_entity_id` 与 `ctx.legal_entity_id` 必须相同；指定记录不存在、跨法人、密级/数据范围不可见均统一 `PLATFORM.AUTHZ.NOT_FOUND_OR_DENIED`，不得用空集合掩盖指定对象越权。数量/金额/期间/分页或输入集合形态非法使用本卷既有 `VALIDATION` 码；锁后余额、版本或业务容量变化使用本卷既有 `BUSINESS_CONFLICT` 码；collect/reload 的规范化候选键集合变化只由协调器以 SQLSTATE 40001 中止并走全局事务重试，不降格为业务冲突继续半锁事务；头行、期间、合计、排序键或 owner 图不守恒按内部不变量失败并整笔回滚，不新增模糊的 `PORT_FAILED` 码。`PayableRegistrationPort::register_purchase_invoice`、`OverbillingMatchPort::match_on_receipt` 与 `PurchaseCreditNotePort::register_credit_note` 的 application 实现必须在首条业务 SQL 前调用权威 `ep_contract_ledger::f50_lock::CrossModuleLockCoordinator::assert_covers`，验证同一 `TransactionLockProof` 的 HMAC、tx_id、法人和 plan digest 及所需集合；缺类、缺 id、异事务、异法人、摘要错误或 proof 重放均失败关闭，端口不得为缺口补锁。`TransactionLockProof` 是 foundation 的业务无关不透明类型，两个 Stage 10 contract crate 不得依赖 `ep-contract-ledger`；只有 application 实现与最外层编排依赖 coordinator。所有写端口都使用调用方事务，成功返回前不执行幂等 `finish`、Outbox、通知或终末审计；这些统一由最外层业务用例按第 6.1 节收口。

##### 5.9.3 契约编译、快照与装配门禁

1. `crates/contract/finance/tests/api_snapshot.rs` 与 `crates/contract/invoice/tests/api_snapshot.rs` 对公开 `pub use`、16 个 trait、全部方法参数/返回值、enum 变体和 DTO 字段做逐字快照；其中快照必须显式包含 `PayableRegistrationPort::lock_candidates`、`OverbillingMatchPort` 的三个方法、`ReceiptInvoiceMatchQueryPort` 的一个候选方法加三个 proof 后查询方法，以及 `PurchaseCreditNotePort::lock_candidates`，方法增加不改变 trait 数。变更快照必须走正式设计变更，不能由开发者顺手接受。
2. `tests/trybuild/stage10_contracts/` 至少编译阶段 6、7、11、12 四个 consumer fixture；正例必须只依赖 foundation 加对应 contract crate 并可构造每个命令/读取每个结果，Stage 7 fixture 还须按 `PurchaseOrderInvoicingPort::targets → lock_targets_after_global → seal → states_after_seal → record_invoice/reverse_invoice` 编译唯一调用形状；invoice-costing fixture 必须只经 `ep-contract-costing::PurchaseInvoiceCaptureReversalBasisQuery` 构造两种 `PurchaseInvoiceOriginalCaptureKind`，并穷举 `PurchaseInvoiceCostEffectSign::{DebitCost,CreditCost}` 与两个 `ReturnCostRole` 组装专用 `PurchaseInvoiceCostLiveFragment`，且只以 `available_amount.as_money()` 取得容量并把严格正的 `Money` 交给 `PostingAttribution.amount`。反例固定拒绝具体数据库连接、漏传或自行构造 `TransactionLockProof`、proof 前/漏 proof 调 `ReceiptInvoiceMatchQueryPort` 的三种容量查询或 `PurchaseOrderInvoicingPort::states_after_seal`、在 `PurchaseOrderInvoiceTarget/F50LockPlan` 塞 quantity/status/row_version、在 `PurchaseOrderInvoiceLineEffect` 传 `expected_line_row_version`、直接构造私有 `PositiveMoney`、把 `PositiveMoney` 赋给 `PostingAttribution.amount`、用通用 `CostLiveFragment` 代替专用含符号 fragment、自行构造动态 measure key、遗漏 live parent 或从 invoice 导入 costing 数据库行、`&dyn SnapshotCtx` 调业务写端口、`&mut dyn Tx` 调两个快照方法、跨类型 `Id<T>`、旧 `PayableQueryPort/PayableStatementQueryPort/InvoiceStatusPort` 与任何未定义 DTO。
3. object-safety 测试逐个构造 `&dyn <Trait>`，机械计数必须等于 finance 10 + invoice 6 = 16；`cargo metadata` 断言两个 contract crate 不依赖任何 domain/application/adapter crate。
4. core-server 装配测试对全部业务事务方法、四个无锁候选入口（Payable、Overbilling receipt、ReceiptInvoiceMatch、PurchaseCreditNote）及 `PurchaseInvoiceCaptureReversalBasisQuery` 注入真实实现；job-worker 装配测试至少注入 `OverbillingMatchPort::settlement_segments_by_receipt_lines` 与 `ReconciliationItemQuery` 的真实实现。两个 wiring 目录全量扫描不得出现 `Noop/Stub/Fake/Dummy` 发布实现，测试专用 recording 类型只能位于 testkit/test cfg。
5. Stage 7 的收货与 R-PROC-05 契约测试使用同一组真实数据库事实，逐键证明写方法返回段、finance settlement 行、快照读方法返回段与 inventory 计价段四者相等；采购退货测试另断言 `lock_candidates_for_receipt_lines` 在 collect/reload 各恰调用一次且结果不含量额/版本，seal 前 `match_state`、`match_states`、`billed_allocations_for_purchase_invoice_lines` 与 `PurchaseOrderInvoicingPort::states_after_seal` 四种锁后读取的调用次数均为零，seal 后只以同一 proof 调用对应分支并返回锁后事实。采购订单回写夹具还断言 `targets/lock_targets_after_global` 只返回四项标识、两次键集合相等，writeback command 不含 expected version。两组都以依赖图和 SQL 常量扫描断言 `ep-app-procure` 不出现 `finance.`/`invoice.` schema SQL、Stage 10 不在 procure 侧注入空实现。任何一项失败即阻断 Stage 10 退出，不能登记顺延项。

---

### 6. 并发与事务边界

#### 6.1 事务清单

下表逐个用例给出事务边界。全部业务事务隔离级别 `READ COMMITTED`，按共享基线第 8.4 节。

| 用例 | 一个事务内包含的写入 | 事务外 |
|---|---|---|
| issue_sales_invoice | 销项发票头行；发票申请单状态；`invoice.invoice_receipt_plan_links` 的 `ISSUE` 正向分摊；`finance.receivable_entries` 的 `ORIGINAL` 主条目；`finance.unbilled_ar_entries` CREDIT；预收与应收两侧 `ADVANCE_AUTO/APPLY` 效果及投影；ledger 凭证；幂等 finish；业务事件 Outbox；同事务通知命令；审计终结批 | 附件正文；站内通知投递。不写 clm，不投递任何已开金额回写 Outbox |
| register_invoice_reversal | 冲销头行；原票状态；申请单本次比例回滚；`invoice.invoice_receipt_plan_links` 的 `VOID/RED_LETTER` 反向分摊；AR/AP 追加指向原主条目的 `REVERSAL`；需要时追加两级 LIFO `RELEASE` 及可追溯 advance 创建/恢复效果；销项的 unbilled DEBIT；超量路径二；凭证；幂等 finish；业务事件 Outbox；同事务通知命令；审计终结批 | 附件正文；站内通知投递。不写 clm，不投递已开金额回写 Outbox |
| register_receipt | 到款单；核销关系行；应收条目更新；预收条目新增；资金腿明细；凭证；幂等 finish；Outbox；同事务通知命令；审计终结批 | 附件；通知投递 |
| register_payment | 付款登记单；核销关系行；应付条目更新；预付条目新增；资金腿明细；以同一 F-50 proof 先释放逐发票 `invoice_releases`，再经 `PaymentRequestWritebackPort::register_payment` 回写申请行/头已付金额与状态，并经 `PayableReservationReadPort::after_lock` 终检占用上限；凭证；幂等 finish；Outbox；同事务通知命令；审计终结批 | 附件；通知投递；重新认证校验在事务外先行完成 |
| register_refund | 退款单；逐来源 `refund_source_payment_links`；每个 link 下的 advance `APPLY` 和 AR/AP `RELEASE`；三项投影重读；资金腿；凭证；幂等 finish；Outbox；同事务通知命令；审计终结批 | 附件；通知投递。`refunded_amount` 只从链接投影，不单独回写为事实 |
| register_cash_document_reversal | 冲正单；按锁后当前去向追加的 `APPLY/RELEASE`；advance 创建或效果恢复/消耗；若原单为付款，以同一 proof 经 `PaymentRequestWritebackPort::reverse_payment` 逐申请行回减已付额并按阶段 7 规则恢复占用；资金腿反向；唯一动态拆分凭证；原单 REVERSED；投影与勾稽重读；幂等 finish；Outbox；同事务通知命令；审计终结批 | 通知投递。不修改历史金额事实，不写“反向 advance 条目” |
| settle_overbilling_by_write_off | 结清记录；挂账更新；凭证；幂等 finish；Outbox；同事务通知命令；审计终结批 | 通知投递 |
| match_overbilling_on_receipt | 结清记录；挂账更新；凭证附加腿。本用例由收货用例在其事务内调用，不自开事务 | |
| register_purchase_invoice | 采购发票头表与行表；三单匹配；经 `GrniEffectWritebackPort` 追加 `PURCHASE_INVOICE/DECREASE` 并取得暂估回冲金额；经 `InventoryVariancePort::split_variance` 拆分价差；行表汇总回写；`register_payable_on_purchase_invoice` 的应付条目与预付自动核销；超量开票挂账；经 `PurchaseOrderInvoicingPort::record_invoice` 推进采购订单行已开数量与直接费用订单状态；来源为门户上传时经门户侧写端口迁到 `ACCEPTED`；凭证；幂等 finish；Outbox；同事务通知命令；审计终结批，以上全部共用一个 `ResolvedPeriod` 与同一 proof 并同事务 | 附件正文读写；站内通知投递 |
| register_payable_on_purchase_invoice | 应付条目；预付自动核销；超量开票挂账。本用例由本阶段的 register_purchase_invoice 用例在其事务内调用，不自开事务 | |
| register_purchase_credit_note | 进项红字头行；经 `GrniEffectWritebackPort` 对原发票的开放 DECREASE 逐父追加 `PURCHASE_CREDIT_NOTE/INCREASE`；AP 追加指向原 `ORIGINAL` 的 `REVERSAL`；需要时追加应付 `RELEASE` 与预付效果；库存/成本调整；超量路径二；数量冲减非空时经 `PurchaseOrderInvoicingPort::reverse_invoice` 回减采购订单行已开数量并重开必要状态；凭证；登记 Outbox 与审计待写项。本用例由阶段 7 的采购退货用例经 `PurchaseCreditNotePort` 在其事务内调用，不自开事务、不提前刷新；调用方最后执行幂等 finish、Outbox、通知命令、审计终结批 | |
| record_unbilled_ar_on_delivery / on_sales_return | 仅 Stage 6 非零收入且 `PostingOutcome::Posted` 的分支分别在调用方原事务追加 `finance.unbilled_ar_entries` DEBIT/CREDIT，不自开事务；合法全零 `Skipped` 分支不调用、不写零额行 | |
| import_opening_balances | 四张台账表之一的期初条目，`source_doc_type` 取 MIGRATION_OPENING。逐行独立事务，不生成凭证，不写 Outbox | 通知 |
| maintain_cash_account | 资金账户行；幂等 finish；审计终结批 | 审批在事务外 |

一个 HTTP 请求内不开启多个写事务，按共享基线第 10.3 节。批量导入按行拆事务，是后台任务而不是单个 HTTP 请求，不违反该纪律。

所有写用例采用同一收口顺序：先按 F-50 冻结锁序与各算法既定引用顺序完成业务事实、子账、凭证和同步投影，再执行幂等 `finish`，再刷新本模块及嵌套 owner port 登记的全部 Outbox 待写项，再写确需同事务落库的通知命令，最后调用 `AuditWriter::append_terminal` 批量落审计；不存在的类别跳过但后缀不得调换。`append_terminal` 封印 `Tx`，其后任何 SQL `query/execute`、仓储写入或跨模块端口调用都必须以内置不变量错误拒绝并令整个事务回滚，唯一允许的后继动作是 `UnitOfWork::commit`。共享跨模块契约测试 `audit_terminal_seals_tx` 以采购发票、退款和付款三条多端口路径为夹具：审计后分别尝试本地仓储、`GrniEffectWritebackPort`、`InventoryVariancePort`、`PostingPort` 与采购写回端口，均应失败、审计后零新增行、事务整体回滚；正常路径由 recording transaction 断言审计批是 commit 前最后一批数据库执行。

#### 6.2 锁策略

- 唯一协调器为 `ep_contract_ledger::f50_lock::CrossModuleLockCoordinator`，业务类别与顺序只存在于其 `F50LockPlan`。具体实现 `LedgerF50LockCoordinator` 位于 ep-app-ledger；Stage 10 contract 只接收 foundation 的不透明 `TransactionLockProof`，不依赖 ledger contract，也不自建第二个 coordinator/guard 类型。
- coordinator 前允许且仅允许纯输入解析、权限与重新认证、UUID 预生成、幂等受理、`AccountingPeriodResolver::resolve`（含首个零期间同事务建立）及无锁候选标识收集；期间解析是唯一允许的平台前置写，不属于 F-50 业务锁图。resolve 后必须立即执行 `collect → lock_all → reload → seal`；取得 proof 前禁止金额/容量/GRNI/库存计算、号码占用和任何模块业务事实写。
- `lock_all` 按 F-50 封闭类别依次锁定原款项/退款单 → 退款来源链接 → 采购退货单 → 原发票头 → 原发票行 → 收货行 → GRNI 根/效果 → 库存余额/金额 → 应收/应付正向主条目 → 预收/预付条目 → 核销根 → 核销效果行 → 超量挂账/结清 → 冲销行与累计；没有的类别跳过，每类 id 采用 UUID bytes 升序，GRNI 同根按 `root_effect_id,id`。全局类别全部完成后、seal 前，只允许调用 plan 已声明的 owner lock-only 方法锁定不在全局类别中的聚合辅助行；Stage 10 当前唯一具名方法为 `PurchaseOrderInvoicingPort::lock_targets_after_global`，它只按 order/line id 升序锁行并返回与 proof 前 `targets` 相同的标识形状，不读取数量、金额、status、row_version，不改状态、不扩全局 plan。随后重调全部无锁候选入口形成 reloaded plan；只比较权威 `F50LockPlan` 的规范化 id/维度键，集合漂移由 `seal_after_reload` 以 40001 中止并整事务重试，禁止扩 plan 塞入 version 或 balance。seal 后才可调用带 proof 的 `PurchaseOrderInvoicingPort::states_after_seal` 读取已锁数量/状态/版本；plan 不得追加 id，也不得取得第二份 proof。
- `PayableRegistrationPort`、`OverbillingMatchPort::match_on_receipt`、`PurchaseCreditNotePort`、`PaymentRequestWritebackPort::{register_payment,reverse_payment}`、`PurchaseOrderInvoicingPort::{record_invoice,reverse_invoice}` 以及它们调用的 GRNI、inventory、AP/AR/advance/settlement/reservation mutator 首句都以同一 coordinator 执行 `assert_covers(tx,ctx,f50_lock_proof,required_subset)`；验证 HMAC、tx_id、法人、plan digest 与类别/id 覆盖后只重读已锁行或执行已冻结 row_version 条件更新。任何 owner 仓储在 proof 存在的事务里执行新的 `SELECT ... FOR UPDATE`、advisory lock 或补锁都按内部不变量失败并整笔回滚。
- 本次将新增的四张 settlement link 根/效果 id 与 `invoice_receipt_plan_links` 根 id 均可在 coordinator 前预生成，但不得插行。前者必须分别进入 collected/reloaded plan 的 `SettlementRoot/SettlementEffect` 键且在 proof 后以单 INSERT 建首根；后者不扩充 `F50LockPlan`，其并发边界复用计划内的原销项票/原票行及 invoice 冲销累计键，invoice owner 取得同一 proof 后才写 `id=root_allocation_id`。两类数据库自 FK均为延迟校验，但延迟不构成 proof 前写入许可，也不得用先 NULL 后回填规避一次锁全集。
- `PurchaseInvoiceCaptureReversalBasisQuery::lock_available` 是唯一具名的 F-50 图外后置锁读：它只在 proof 已 seal、所有 F-50 全局类别与 procure 辅助锁均完成后，按 `(original_purchase_invoice_line_id,original_capture_kind,root_entry_id,live_entry_id)` 固定全序锁 costing live leaf；此后直到 commit 任何路径都不得再取得 F-50 或其它新锁。该 query 不写事实、不扩 `F50LockPlan`，仅向红字 Posting attribution 返回 current role/open/dimensions；除此例外，四个 F-50 owner 在 proof 后仍严禁新增 `FOR UPDATE`、advisory lock 或补锁。
- 不进入 F-50 跨模块图的发票申请草稿等单聚合动作仍可在 owner 内按 id 升序加行锁并校验 `row_version`；一旦进入开具、冲销、收付款、退款、采购发票、进项红字、采购退货或超量结清，即完全服从上述 proof，不再叠加本地锁序。资金账户 `has_cash_flow` 的幂等置位保持 `UPDATE ... WHERE id = $1 AND has_cash_flow = false`，但引用该账户的资金事务必须把账户及相关资金腿类别纳入 plan。
- 核销与超量候选只可在 collect 阶段无锁读取并把全部候选 id 放入 plan；锁后按 proof 覆盖集合重算有效容量。集合或容量漂移返回 `BUSINESS_CONFLICT` 与 `FINANCE.SETTLEMENT.EFFECTIVE_OPEN_CHANGED`，由界面重取；不得用已作废的 `OPEN_AMOUNT_CHANGED` 别名，也不得先算分配再补锁。

#### 6.3 幂等与 Outbox

写入幂等由共享基线第 5.4 节的 `platform_msg.idempotency_keys` 承担，与业务写入同事务；成功结果的 `finish` 必须晚于全部业务/子账/凭证/投影，早于 Outbox、同事务通知命令和审计终结批。

本阶段发布的 13 个事件在幂等 `finish` 之后统一写入 `platform_msg.outbox_events`，信封的 `posting_date` 与 `accounting_period_id` 取本次解析结果；Outbox 刷新后只允许同事务通知命令与审计终结批。消费者为 notify、reporting 投影与 search 索引，均不做过账，见第 0.1 节。

消费端幂等由 `platform_msg.inbox_consumptions` 的唯一约束保证。重投退避按共享基线第 6.2 节。

#### 6.4 失败重试与补偿

- 序列化失败 40001 与死锁 40P01 由数据访问层重试 3 次，退避 50、150、450 毫秒，只对尚未产生外部可见副作用的事务重试。本阶段全部用例在提交前不产生外部副作用，因此全部可重试。
- 守恒 CHECK 违例不重试，直接映射为 `BUSINESS_CONFLICT` 并按规格第 15.2 章写入死信，`incident_no` 回带给界面。这一路径覆盖 PRD 第 6.7.7、6.8.8、6.11.3、6.12.7 的负数未核销余额行。
- ledger 端口返回借贷不平时不重试，整事务回滚并写死信，对应 PRD 第 6.4.8 最后一行。
- 批量导入行级失败不回滚已成功行，失败行写入结果对象并计入 `failed_rows`，批次状态置 PARTIALLY_FAILED。
- 本阶段不使用补偿事务：全部跨模块写入都在同一数据库事务内经 owner 契约完成。收款计划净已开金额不是跨模块写腿；它由 `invoice.invoice_receipt_plan_links` 实时聚合，因而不存在 clm 回写、金额 Outbox、补偿用例或第二份金额投影。

#### 6.5 必测并发场景在本阶段的落点

共享基线第 8.4 节固定的六组必测并发场景中，本阶段承担三组：同一单据的乐观锁冲突（发票申请单）、同一采购订单的并发发票匹配与暂估回冲（本阶段同时承担发票侧与超量开票挂账侧，因为采购发票登记按裁定 A-10 已归本阶段）、Outbox 同一事件的重复投递不少于 3 次。另新增三组本阶段专有场景：同一应收条目被两笔到款并发核销、同一预收条目被开票自动核销与客户退款并发消费、同一收货行被两张采购发票并发匹配且只有一张成功登记。

---

### 7. 配置项

全部按共享基线第 7 节：前缀 `EP__`，层级双下划线，`deny_unknown_fields`，敏感项不入配置。

| 键 | 类型 | 默认值 | 生效方式 |
|---|---|---|---|
| EP__INVOICE__TAX__AMOUNT_TOLERANCE | decimal | 0.02 | 启动时读取，变更需重启；取值写入 `platform_ops` 台账 |
| EP__INVOICE__RATIO__TOLERANCE | decimal | 0.000001 | 同上 |
| EP__INVOICE__ISSUE__REQUIRE_IMAGE_ATTACHMENT | bool | false | 同上 |
| EP__INVOICE__IMPORT__MAX_ROWS | u32 | 2000 | 同上 |
| EP__INVOICE__IMPORT__ON_ROW_FAILURE | enum CONTINUE 或 ABORT | CONTINUE | 同上 |
| EP__INVOICE__VOID__MAX_DAYS_AFTER_ISSUE | optional u32 | null（不限） | 同上；正整数只收紧作废登记时间窗，0 非法 |
| EP__FINANCE__SETTLEMENT__CROSS_PARTY_ALLOWED | bool | false | 同上 |
| EP__FINANCE__SETTLEMENT__MAX_LINES | u32 | 200 | 同上 |
| EP__FINANCE__RECEIPT__REQUIRES_APPROVAL | bool | false | 同上 |
| EP__FINANCE__CASH_ACCOUNT__REQUIRES_APPROVAL | bool | true | 同上 |
| EP__FINANCE__BANK_ACCOUNT__MASK_TAIL_DIGITS | u8 | 4 | 同上 |
| EP__FINANCE__RECON__MAX_PERIODS_PER_QUERY | u8 | 12 | 同上，限制对账视图单次查询的期间跨度 |

不进配置文件而进事务数据库并经配置发布通道的运行期业务参数有三项：税率可选值（`invoice.tax_rate_options`，其出厂预置按第 0.4 节 F-02 与第 3.6 节由 invoice 目录第 13 号种子迁移在 T0 期间写入，此后的增删改经发布通道）、发票申请与开票的审批链、到款与付款的提醒规则。账龄分档在本阶段不经发布通道，只出厂预置一套六档，见第 3.2.1 节。按共享基线第 7.1 节最后一段。发布通道按裁定 A-27 一律使用阶段 3b 交付的最小发布通道，本阶段不自建第二套。

本阶段不新增启动自检项，也不把任何判读业务数据行的比对挂进启动自检。按裁定 C-25，启动自检项一律按注册名标识而不用序号；共享基线第 7.3 节原有的 `current-period-open` 已整项撤销，本阶段不引用该项。按总览第 1.5 节第十二条，首个会计期间由阶段 9a 的 `AccountingPeriodResolver::resolve` 第二步的零期间分支建立，即该法人 `ledger.accounting_periods` 无任何行时按 `posting_date` 所属自然月建立该期间并置 OPEN，建立动作在同一业务事务内完成，该分支属阶段 9a 交付并落在阶段 9a 的 T0 切片内，本阶段的销项发票与到款登记在首次过账时即经该分支取得期间。本阶段与登记表相关的比对按第 5.8 节只挂 CI 一处。本阶段在 `--check` 模式下额外输出两个法人的账龄分档与税率字典行数，只作报告不作判定，也不据以拒绝启动。

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
8. 销项发票状态机的全部合法流转与非法流转，含 ISSUED/PARTIALLY_RED_REVERSED 上的分次红字、累计全额红冲、VOID 只从无冲销的 ISSUED 发起，以及作废与红字互斥。
9. 开具日期与记账日期晚于基准日的拒绝，补记的接受。
10. 累计开票比例：`ISSUED`、分次部分红冲、末次全额归尾与 VOID 分别按 `issued_ratio-already_rolled_back_ratio` 计入，部分红冲票不得整张排除。

财务侧：

11. 核销分配算法：候选为空、单条全额、单条部分、多条跨越、恰好用尽、有剩余、行数超限，共 7 个分支。
12. 人工指定顺序的三种失败：行超额、合计超额、条目不属该往来方。
13. 账龄分桶：六档各一个用例，加未到期、恰好到期日、跨最后一档开区间三个边界。
14. `OpenItem` 守恒：`ORIGINAL/REVERSAL` 三种合法来源形状，`S/C/row_open/effective_open` 全部等式，冲销行不进候选，以及任一负数/超原额的拒绝。
15. 退款来源守恒：请求分配合计错误、单 link advance 先消耗、本 link 根容量不足、禁止借用另一 link、退货证据上限及逐 link/整单投影等式。
16. 超量开票余额推进：三条路径各一个用例，加路径三后请求路径一的拒绝、路径三冲回后路径一的接受。
17. 冲正：银行/现金腿全额反向，往来与预收预付按锁后当前可追溯去向动态拆分；覆盖原去向未变、预收预付已被后续消费、退款后再冲正及拆分容量变化，并逐一断言四种原单到 `OriginalCashDocumentType` 的映射、reversal object_type/id/doc_no、original_voucher_id、posting_date、补记授权与 `source_event_id=None`，逐项验证资金根守恒。
18. 到款单与付款单的金额恒等式：REGISTERED 生命周期内，当前可追溯核销合计加当前开放预收/预付再加未冲正退款/返款恒等于原款项金额；自动核销、退款及退款冲正后仍成立，资金冲正后各投影归零且冲正金额等于原金额。
19. 资金账户：账户类型与科目匹配的四种组合、已产生资金流水后的修改拒绝、停用后不可选；迁移撤销另覆盖 active→inactive 与 inactive 保持两支、未结资金拒绝、row_version/时点形状，以及固定 `FINANCE_CASH_ACCOUNT_MIGRATION_REVERSED` owner audit 的四键 before/after、根引用与独立 R0 target。
20. 状态机：到款、付款、退款、冲正四个单据各自的合法与非法流转。
21. 采购发票三单匹配：数量与订单相等、少于订单、超过累计收货三个分支；`overbilling_amount` 为零与非零；两种同票 `cost_kind` 及混合值整单拒绝；暂估回冲、在库价差、已出库价差、超量挂账与自动核销五类服务端结果均只出现在响应/事件，不接受客户端输入；逐原票行断言非零 `price_variance_released_amount` 与 `direct_expense_amount` 分别生成 `CostPostingVariance(EstimatePriceDiffIssued)`、`CostDirectExpense` 的 Line/NewRoot attribution，父为空且逐行合计守恒；原票头不能在 `PostingOutcome` 前插入，也不存在空 `voucher_id/payable_entry_id` 后补路径。
22. 采购发票红字登记：部分、分次至全额、用于超量开票结清、终态后继续冲销及累计超过剩余金额的拒绝分支；内部端口输入/输出逐字段等于第 4.11 节 exact contract，版本失配拒绝，响应逐行返回服务端 `source_effect_seq` 与 GRNI 重开效果。largest-remainder 另覆盖零/单 leaf/多 leaf、余数相等、1 分尾差、容量不足、输入乱序、正 `DebitCost` 根、负 `CreditCost` 根、混合符号拒绝，以及同一原 root 经受控更正分裂成 `MainOperatingCost+DirectExpenseCost` 两种 live role；断言四个静态 measure key、capture kind、当前维度与 exact `live_entry_id` 父逐项正确，signed measure 保持控制额符号、attribution amount 恒正，两个拆分和分别等于 `released_variance_reversed_amount/direct_expense_amount`，linked-return 差额仍为父空 NewRoot。
23. `CLM_TERM_RECEIPT_PLAN`：两期正向分摊、部分红字、末次红字与 VOID 后，`billing_by_period` 逐期净额精确等于 `ISSUE-VOID-RED_LETTER`；只有净额为零的 ACTIVE 期次自动 VOIDED，正数占用与锁后竞态返回 `AlreadySatisfied`，负数直接报勾稽故障；同时断言 clm/Outbox 无金额副本。
24. `CLM_TERM_SALES_INVOICE`：ISSUED/PARTIALLY_RED_REVERSED 命中、终态不命中、初次 `NeedsManualDecision`；VOID、全额 RED_LETTER、KEEP 三码逐项验证 `decision_result_doc_id` 的必填/必空与归属、非空 reason、错码/异票/部分红字拒绝，以及三态返回值。

#### 8.2 领域属性测试

用 proptest，覆盖规格第 17.3 章中本阶段承担的不变量，是共享基线第 8.1 节要求的五组之外的追加组。

| 属性 | 断言 |
|---|---|
| AR/AP 当前守恒 | 对任意正向主条目、分次冲销与多层 `APPLY/RELEASE` 链，恒有 `0<=S<=O`、`row_open=O-S`、`0<=effective_open=O-S-C<=row_open`；冲销行不成为核销候选 |
| 到款/付款资金根守恒 | 对任意 REGISTERED 到款/付款及其自动核销、退款/返款与退款冲正序列，资金根下当前 AR/AP 净核销加 `advance_open` 加未冲正退款/返款恒等于原款项金额；REVERSED 后三者归零且冲正金额等于原金额 |
| 预收/预付守恒 | 对任意多层 effect 链，`net_consumed=ΣAPPLY-ΣRELEASE`且 `advance_open=original_amount-net_consumed`始终在 `0..=original_amount` |
| 开票比例守恒 | 对任意开具与冲销序列，该申请单下有效发票的开票比例合计加剩余可开比例恒等于申请单开票比例，容差内成立 |
| 资金腿守恒 | 对任意到款、付款、退款、冲正序列，账户期末余额恒等于期初余额加收方向合计减付方向合计 |
| 过渡科目双向净额 | 对任意交付确认与开票的交错序列，净额恒等于已交付未开票减已开票未交付 |
| 冲正守恒 | 对任意到款/付款/退款顺序，冲正按锁后去向满足 `R=S+V` 或逐 link 的 `X=A+E,Q=Z+V`，全部根净额、`effective_open`、`advance_open` 均不负，不要求镜像历史投影 |
| 账龄不随顺延改变 | 对任意条目，改变 `accounting_period_id` 不改变账龄分桶结果 |

#### 8.3 集成测试

使用真实 PostgreSQL 16，每个用例独占一个 `ep_test_<nanoid>` 库，禁止用内存库替代。场景清单如下。

1. 开具登记全链路：申请单从 DRAFT 走到 FULLY_ISSUED，应收条目、过渡科目条目、收款计划勾稽、凭证五处逐项核对；真实 PostgreSQL 直接证明两对 invoice↔AR/AP 复合外键延迟到提交且终态双向互指，原票头从未以空 `voucher_id/receivable_entry_id/payable_entry_id` 插入；强制 `PostingPort::post` 失败、`Skipped`、孤立 `IdempotentReplay` 时整个号码/台账/GRNI 图为零残留。同组迁移夹具分别从五张空表写入 `invoice_receipt_plan_links` 的 `id=root_allocation_id` ISSUE 根，以及四张 settlement link 的 `id=root_apply_id` APPLY 根，均要求单条 INSERT 后 COMMIT 成功；再逐表以错法人/原票/期次/所属条目/根/父长键证明短 UUID 即使存在也不能挂接。Stage 6 consumer 契约测试各覆盖交付确认/销售退货的非零 `Posted+非空 voucher+恰一 unbilled` 与全零 `Skipped+voucher=None+零 unbilled` 分支，并以错方向、零额直调、nil voucher 三个负例证明 finance 端口在插入前失败。
2. 红字冲销后重新开具：对应规格第 8 章第 7 步与第 17.2 章“作废与红字冲销后按同一发票申请单重新开具成功，累计开票比例校验不被误阻断”。
3. 作废与红冲互斥：两次登记，第二次被拒且写入审计。
4. 分次到款：三笔到款核销同一张发票，`effective_open` 逐次下降至零，账龄与候选同源变化。
5. 一笔到款核销多张发票：五张发票，核销顺序按到期日升序、同日按单据编号升序，与规格第 5.2 章核销顺序规则块逐项比对。
6. 人工指定核销顺序：与默认顺序不同，审计事件中可查得该事实。
7. 到款金额大于可核销应收：对应规格第 17.2 章第五类必测分支，超出部分挂预收，子账与总账两侧金额一致，后续开票自动核销并进入账龄。
8. 付款金额大于可核销应付：对应第六类必测分支；`INVOICE_PAYMENT` 另逐发票断言本事务新增 AP `APPLY` 汇总恰等于传给 `PaymentRequestWritebackPort::register_payment` 的 `invoice_releases/paid_amount_delta`，申请行/头 paid、remaining、status、row_version 与返回值一致，释放 reservation 后的锁后终检仍满足 `effective_open_after>=reserved_after`。`PREPAYMENT` 只允许对应的预付 allocation，禁止伪造发票释放。
9. 预收自动核销：对应第二类必测分支，每个分段的 advance `APPLY` 与 AR `ADVANCE_AUTO/APPLY` 根金额及资金根一一对应，`advance_open/effective_open` 与凭证一致。
10. 预付自动核销：由采购发票登记触发，同上。
11. 已收款后的销售退货并退款：对应第八类必测分支，含红冲前置、退款登记、资金账户明细与银行存款科目余额一致。
12. 已付款后的采购退货并收回货款：对应第九类必测分支。
13. 超量开票挂账与三条结清路径：对应第十三类必测分支，逐条路径验证挂账余额归零、凭证借贷相等、关账拦截与解除。
14. 超量开票路径三之后到货：验证必须先冲回成本再走路径一。
15. 资金单据冲正：到款、付款、客户退款、供应商返款各覆盖 `CashDocumentRef` 精确构造与当前去向拆分；预收已被后续开票消费时按 `R=S+V` 正常冲正，仅未冲正下游退款或追溯不守恒时拒绝；退款冲正验证 `Y/X/A/E/Q/Z/V` 逐 link 不混池。付款冲正从原付款持久化分配图逐行构造 AP `RELEASE`，并以同一 proof 恰调一次 `PaymentRequestWritebackPort::reverse_payment`；传入 delta 必须逐值等于 RELEASE，非 CLOSED 申请按 Stage 7 唯一规则恢复 reservation，CLOSED 不恢复，重放不得二次调用。负例覆盖错 object_type、错 original_doc_type/id/voucher、遗漏补记授权、首次执行返回 IdempotentReplay/Skipped，均零提交。
16. 会计期间顺延：在一次关账受理之后提交一笔到款，凭证与全部子账条目落入其后最早的可入账期间且 `deferred_from_period_id` 非空，两条检索路径均可查得，对应规格第 10.2 章的注入用例其一。
17. 勾稽取数十项：正常态差额为零；逐项注入差额后差额非零并生成差异事项引用；差额清零后恢复。存货与已收货未收票两项经 `ep_contract_inventory::StockValueSubledgerBalancePort` 与 `ep_contract_procure::GrniSubledgerBalancePort` 两个模块端口取数，其注入方式为改变阶段 8 与阶段 7 的子账侧样例数据。其余八项的注入方式为直接对台账条目做受控 UPDATE，仅在测试库上执行。另固定历史切片用例：M1 分别写 unbilled 与 overbilling 原始事实并保存 M1 结果，M2 再写发票抵销、路径一/二/三结清及路径三冲回，随后重跑 M1 必须逐值等于保存结果，M2 才反映新增效果。
18. 批量导入：2000 行成功、含 3 行失败的部分失败、重跑同批次不产生重复发票。
19. 幂等：第 5 节全部 23 个写 operation 各一次重放，返回首次结果并带 `Idempotent-Replay: true`；载荷不同时返回 409。同一真实库另以 `reverse_migrated_cash_account` 覆盖 active→inactive、inactive 保持与未结资金拒绝，断言 owner audit 与 R0 是两个同法人同 occurred_at 的事件、receipt target 指 owner event；错 action/root/before/after/version/time、复用 R0 event id、只改根或只写 audit 的 direct-SQL 夹具由 Stage 14 第 092600 号延迟分支在 COMMIT 拒绝，失败时根、审计、receipt 与 migration record 零部分写入。
20. 法人越权矩阵：并入独立测试目标 `tests/rls_matrix`，覆盖读取、写入、更新、删除、聚合、排序、报表投影与错误信息泄漏八类，本阶段追加 40 张表与 19 个视图的条目，另覆盖资金账户银行账号字段级权限的两种上下文；八类断言函数名与 32 组矩阵按裁定 C-05 由阶段 1、阶段 2 与阶段 4 分段提供，本阶段只追加条目，不重复实现同名函数。
21. 高风险操作：开票、付款、超量开票路径三三项验证重新认证缺失时拒绝、审批未完成时拒绝、申请人自审时拒绝。
22. 并发：第 6.5 节列出的六组各一个用例，用两个连接交叉提交。
23. 采购发票登记全链路：采购订单、收货、发票三单匹配通过，应付条目、暂估回冲、价差拆分、超量开票挂账与凭证五处逐项核对；proof 前 `PurchaseOrderInvoicingPort::targets/lock_targets_after_global` 只返回相同的订单/行/供应商/类型标识，seal 后才以同一 proof 调 `states_after_seal` 取得数量、状态与版本，再调用 `record_invoice`；逐行 delta 与本次真实发票行相等且命令不含 expected version，物料行只推进已开数量/类型锁，直接费用行闭合行及必要的订单头。原票场景同时证明每条非零已出库价差/直接费用都是对应原票行的 NewRoot attribution，含负 `price_variance_released_amount` 的 CreditCost root。随后先经受控成本更正把一条原 capture 拆成 MainOperatingCost 与 DirectExpenseCost 两个不同维度的同符号 live leaf，再登记独立红字；只允许在 F-50 seal 后调用一次 `PurchaseInvoiceCaptureReversalBasisQuery::lock_available`，largest-remainder 按绝对额与冻结 tie-break 分配到 exact leaf，四个静态 measure 的角色、capture kind、effect sign、维度、父 id 与两组 signed 控制总额逐项一致，容量不足、符号混合或查询集合漂移均零写入。采购退货的物料分支只在 seal 后以 proof 调 `match_states`，DROP_SHIP 只在 seal 后以同一 proof 调 `billed_allocations_for_purchase_invoice_lines`；两路都按返回的 `BilledReturnAllocation` 原票分组，HTTP `credit_note_identifiers` 的键集必须与锁后实际消费的 distinct 原票集合恰好相等且每项是未复用的新红字法定号码，再逐组经 `PurchaseCreditNotePort` 登记红字、冲回应付，并在存在 `REDUCE` 行时以同一 proof 调 `PurchaseOrderInvoicingPort::reverse_invoice` 回减已开数量；纯 `NONE` 不调用。linked-return 的 `linked_return_price_difference_amount` 始终新建父空 COGS root，不调用 basis query。另直接绕过服务写入第 19090930 号迁移列出的孤立/错祖先/缺多组/错量额/错 GRNI 或直运效果图，逐例必须在 COMMIT 由双向延迟图拒绝且整事务零提交。
24. 期初余额导入：应收、应付、预收、预付四个方向各一批，`source_doc_type` 为 MIGRATION_OPENING，导入后首个会计期间的八个 `finance.v_recon_*` view 加两个 snapshot owner port 组装出的十项差额为零；失败行不回滚已成功行。
25. 合同终止收款计划：同合同 ACTIVE 三期在 `billing_by_period` 中分别为缺席/零/正数，只前两期生成处置并置 VOIDED；锁后净额由零变正返回 `RECEIPT_PLAN_NOW_BILLED`，重复处置不重复审计；两期 ISSUE/部分 RED/全额 RED/VOID 证明净额与合同履约页同源，clm 和 Outbox 均无金额副本。
26. 合同终止销项发票人工处置：分别经既有 VOID、分次至全额 RED_LETTER、KEEP 三路闭合，结果单 id 与目标票逐笔核对；空 reason、结果 id 错形、只部分红字和未知码均保持 PENDING。两条规则注册后 `ImpactRegistry` 累计恰为 6，且 Outbox 对 `clm.contract.terminated.v1` 仍只有平台单一消费者。

#### 8.4 端到端测试

后端 E2E 用 Rust 集成测试直接打 HTTP 接口，四端 UI 用 Playwright 驱动桌面 WebView。

- E2E-10-01：闭环第 6 步到第 11 步的连贯路径，从发票申请提交到退款登记，全程在应用内完成，中途不出现外部补齐环节。
- E2E-10-02：直运订单分支下的退款与返款走同一张单据、同一套字段与同一套勾稽校验，对应 PRD 第 6.12.6。
- E2E-10-03：移动端按规格第 6.2 章矩阵为仅查看，验证移动端可查得本阶段全部单据与台账、且提交入口不可达。
- E2E-10-04：供应商门户的收付款对账查询取数与内部应付台账同源，脱敏后返回，对应 PRD 第 4.9.6。
- E2E-10-05：关账受理后提交到款并观察界面显式标注顺延期间，对应 PRD 第 6.1.4。
- E2E-10-06：采购订单到收货到采购发票登记再到付款的连贯路径，三单匹配、暂估回冲、价差拆分、`PurchaseOrderInvoicingPort::record_invoice` 的已开数量/直接费用状态推进、应付核销、逐发票 `PaymentRequestWritebackPort::register_payment` 与 reservation 终检全程在应用内完成，对应裁定 A-10；同一场景再以资金冲正证明 `reverse_payment` 按原分配回减且不二次释放。

按裁定 A-23，本阶段自交本模块四端界面，测试计划相应追加：invoice 与 finance 两个模块在规格第 6.2 章能力矩阵中取值为完整或简化的能力域，其桌面端用例用 Playwright 与 tauri-driver 驱动，移动端用例用 XCUITest 与 Espresso 驱动；取值为 VIEW_ONLY 的能力域只测只读视图；取值为 NOT_APPLICABLE 的不建入口。E2E-10-03 的移动端仅查看断言并入该组用例。

#### 8.5 静态检查

- `ep-domain-invoice` 与 `ep-domain-finance` 中不出现 sqlx、reqwest、tokio 的 IO 模块、`std::fs`、`std::net`、`SystemTime::now`、`rand` 符号。
- `ep-app-invoice` 与 `ep-app-finance` 的用例函数中不出现 reqwest 与文件写入符号。
- `ep-app-invoice` 的 SQL 常量、仓储与查询代码不得出现 `costing.cost_entries`、`costing.revenue_entries` 或同义表名；`cargo metadata` 只允许 `ep-app-invoice → ep-contract-costing`，禁止依赖 `ep-app-costing`/costing adapter。四个红字角色拆分 MeasureKey 只能来自 ledger 静态枚举，代码中不得拼接 measure key 字符串。
- `ep-app-clm` 的收款计划影响规则不出现 `invoice.` 表名，`ep-app-invoice` 的销项发票影响规则不出现 `clm.` 表名；跨模块事实只经 `ReceiptPlanBillingQuery`。
- `invoice.invoice_receipt_plan_links` 是唯一净已开金额事实源；代码与迁移中不存在 clm 已开金额列、同步写端口、已开金额 Outbox 事件/消费者或第二套聚合。契约快照机械断言 `billing_by_period` 的返回类型恰为 `BTreeMap<i32,Money>`，聚合式恰为 `ISSUE-VOID-RED_LETTER`。
- 两个 schema 上不存在 `DELETE` 语句。
- `finance.cash_ledger_entries` 只被四个用例的仓储写入，由一段基于 `cargo metadata` 与调用图的自检脚本断言。
- `ep-app-invoice` 不依赖 `ep-app-finance`，反向亦然。
- `xtask sqlcheck` 对本阶段全部迁移的 token 化 SQL 做基线禁用索引扫描，拒绝所有带谓词的 `CREATE INDEX/CREATE UNIQUE INDEX`；并从 PostgreSQL catalog 断言 `finance.overbilling_settlements` 只有普通约束 `ux_overbilling_settlements_legal_entity_id_reverses_id UNIQUE(legal_entity_id,reverses_id)` 承担非空父唯一性，默认 `NULLS DISTINCT`，不存在任何同义部分索引。
- `xtask sqlcheck` 与 PostgreSQL catalog 快照逐表断言四张 settlement link 的八条根/父自 FK及 `invoice.invoice_receipt_plan_links` 的长根自 FK均为真实 foreign key、`confdeltype='r'`、`condeferrable=true`、`condeferred=true`，引用列顺序与第 3.1.5/3.2.14 节逐字一致；任一短 FK、缺所属条目/原票/期次列、非延迟或非 RESTRICT 均失败。
- 第 19090930 号迁移的 SQL 快照必须同时包含退货行长复合 FK、payable reservation FK、`CREATE OR REPLACE procure.assert_purchase_return_effect_graph_consistent()`、退货头/行与红字头/行四表的 `DEFERRABLE INITIALLY DEFERRED` 触发器，以及恢复阶段 7 原函数/原六表触发器的完整 rollback；少任一侧触发器或 rollback 只删不恢复都失败。

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
| 采购发票登记 | 普通交易提交 P95 在 3 秒内 | POST /api/v1/invoice/purchase-invoices |

上述十项在基准数据集上必须无顺序扫描，阶段结束时提交对应查询的 `EXPLAIN (ANALYZE, BUFFERS)` 证据，按共享基线第 3.10 节最后一段。

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

1. 40 张表与 19 个视图在空库上从零迁移成功，再按 `-- rollback:` 段全部回退成功，两次执行的迁移历史表状态一致；对象目录明确不存在 `finance.v_recon_inventory/v_recon_grni`，且 `invoice.purchase_invoice_attachments` 与其他三张具名 invoice 附件表均存在、父属 FK 与两组唯一键列序逐字正确。catalog 另断言 `ux_overbilling_settlements_legal_entity_id_reverses_id` 是普通两列 UNIQUE constraint 且无任何部分索引；四张 settlement link 的八条根/父自 FK与 `invoice_receipt_plan_links` 的一条长根自 FK均为 `ON DELETE RESTRICT DEFERRABLE INITIALLY DEFERRED`，五张空表首根各以单 INSERT 自引用并成功 COMMIT；第 19090930 号迁移向前后分别具备/恢复第 3.6 节冻结的双向退货—红字图与原六表退货图，完整 up/down/up 后触发器集合逐项相等。
2. `--check` 模式在两个法人上通过，含全部带法人列的表已 `ENABLE` 且 `FORCE` 行级安全的自检项。
3. `docs/openapi/invoice.v1.yaml` 与 `docs/openapi/finance.v1.yaml` 已从初版局部路径种子扩为 `stage10-full-surface`；两文件合并后与第 5 节 49 个 method/path operation 集合逐项相等（23 写、26 读），且描述与实现字段名一致，由契约测试机械断言。缺项、增项、重复 operation、计数不是 49，或任一文件不是完整表面，本条均失败。
4. 第 8.1 节列出的 24 组单元测试分支全部通过。
5. 第 8.2 节列出的 8 组领域属性测试在 1000 次随机用例下全部通过。
6. 第 8.3 节列出的 26 组集成测试全部通过。
7. 规格第 17.2 章十五类必测分支中的第二、五、六、八、九、十三类在本阶段的用例中通过，第十三类的三条结清路径逐条通过。
8. 十个勾稽项全部在基准数据集上差额为零；逐项注入差额后取数结果差额非零、可下钻、可追溯，清零后恢复为零，其中待处理超量开票一项以关账前余额非零的方式注入，对应规格第 10.2 章的发布验收口径；存货与已收货未收票两项经 `ep_contract_inventory::StockValueSubledgerBalancePort` 与 `ep_contract_procure::GrniSubledgerBalancePort` 两个端口实现接入并同样通过该判定。另做 M1/M2 历史稳定性：M1 的 unbilled/overbilling 原始事实在 M2 发生开票、结清或冲回后，重跑 M1 必须逐值不变；静态 SQL 断言两项历史 view 不读 current view、`open_amount` 或 `status`。
9. 规格第 10.2 章的顺延入账注入用例在本阶段的到款登记上通过：凭证与全部子账条目落入同一个顺延后的期间，两条检索路径均可查得。
10. 法人越权测试集 `tests/rls_matrix` 追加本阶段条目后全部通过，八类判据无一泄漏。
11. 三项高风险操作的重新认证与审批控制通过身份与访问控制测试，认证方式、待签内容摘要、时间与设备可在审计证据中查得。
12. PRD 第 6.14.4 列出的 11 类动作全部写入审计，同一事实不只落日志，由审计探针逐项断言。
13. 第 8.6 节十项性能度量在基准数据集上达标，且十项对应查询的执行计划无顺序扫描。
14. 覆盖率达到第 8.7 节的分档门槛，工作区整体不低于 80%。
15. `docs/error-codes.md` 中阶段 10 的活跃自有定义行经 registry 机械去重后恰为 61（FINANCE 31 + INVOICE 30），与 `ep-foundation::error::codes` 常量表一致且无重复码；两份 OpenAPI 传播的 MDM 1 码与 PORTAL 2 码不计入自有码。`docs/event-catalog.md` 的 13 个事件与实现一致；`docs/data-dictionary.md` 的单据类型码一节含本阶段九码且 `xtask configdoc --check-doc-type-codes` 通过。`xtask openapi-schema-contract` 另逐字段比对 `OverbillingEntry` 与第 3.2.8 节投影：字段名、可空性和三态 enum 任一漂移即 CI 失败；负例固定覆盖旧 `remaining_amount`、`SETTLED_BY_*|WRITTEN_OFF` 与 `write_off_voucher_id`。
16. 第 11.3 节列出的共享基线四处回写完成：第 5.4 节幂等 `request_hash` 排除 `X-Reauth-Token`、第 9.2 节新增三个指标、第 11 节新增资金账户期初余额与资金单据冲正两项决定、第 3.5 节确认本阶段未引入新的精度语义。
17. E2E-10-01 至 E2E-10-06 六个用例通过，其中 E2E-10-01 全程在应用内完成。
18. 严重与高危缺陷为零，中危缺陷登记并给出规避方案与责任人，按规格第 17.2 章发布缺陷门禁的口径。
19. invoice 与 finance 两个模块在规格第 6.2 章能力矩阵中取值为完整或简化的能力域，其四端界面已实现并通过 Playwright 与 tauri-driver 的桌面用例、XCUITest 与 Espresso 的移动用例；取值为 VIEW_ONLY 的能力域只实现只读视图；取值为 NOT_APPLICABLE 的不实现入口。
20. 本模块数据集视图 `invoice.v_purchase_invoices_dataset`、`finance.v_receivable_ledger_entries`、`finance.v_payable_ledger_entries` 已发布并授予 `ep_analyst_ro`，列签名已同步给阶段 11。本条在本阶段的被测输入只有本阶段自己的迁移产物与列签名快照，视图已建、授权已授、签名已同步三项均可在本阶段静态判定。阶段 11 的 `reporting-dataset-signature-matched` 在三者上的校验，其被测输入由阶段 11 交付，按基线第 12 节通则第六条取整条推迟一档：本阶段不注册该自检项、不调用它，本条也不断言其结论；三者按第 3.3 节降级口径的通过判定由阶段 11 第 9 节退出条件第 25 条承接，本节不得以任何形态留下以该自检项为被测输入的断言。
21. 本阶段全部路由的能力域码与动作类别常量已声明在 `crates/contract/invoice/src/capability.rs` 与 `crates/contract/finance/src/capability.rs`，`xtask configdoc` 通过；第 5.9 节 trait 表机械展开恰为 16 个且逐个有真实实现，其中包含 `ReceiptPlanBillingQuery`。第 1 节第 5 条与第 2.2 节点名的所有外部 owner trait 只消费、不计入这 16 个；装配门禁对 16 个自有实现和外部消费集合分别做类型集合相等，禁止以一个混合总数通过。
22. 本模块的 `InvoiceReferenceCounter`、`FinanceReferenceCounter`、`InvoiceSalesTradeHistoryProvider`、`InvoicePurchaseTradeHistoryProvider` 已实现并注册到阶段 5 提供的两个注册表。
23. `finance.cash_accounts` 的银行账号查重经 `derive_blind_key` 与 `BlindIndex([u8; 32])` 实现，`ck_cash_accounts_bank_account_no_bidx_len` 拒绝非空且非 32 字节值，唯一约束名为 `ux_cash_accounts_legal_entity_id_bank_account_no_bidx`；全库无第二套账号哈希、截断或宽度配置实现。迁移撤销只走第 4.2 节 crate-private owner 用例；active/已 inactive 两支的根状态、版本、时点与 `FINANCE_CASH_ACCOUNT_MIGRATION_REVERSED` owner audit 逐项成立，owner event 与 R0 分离且 REVERSE receipt target 指前者，未结资金、错审计形状或任一半套写入均不得通过 Stage 14 第 092600 号提交点。
24. `platform_core.append_only_registry` 已登记本阶段两张表 `finance.unbilled_ar_entries` 与 `finance.cash_ledger_entries`，两行的 `mode` 取 `APPEND_ONLY`、`mutable_columns` 取 `'{}'`，五张可更新台账表未登记，`db/checks/append_only_consistency.sql` 经 `xtask sqlcheck` 通过。
25. 本阶段九个事件在 `ledger.posting_trigger_event_types` 中的登记行由阶段 9a 的种子迁移写入且每行只填 `event_type`，本阶段不含任何 `backfill_posting_trigger_event_types` 迁移；本模块九个事件与 `docs/event-catalog.md` 逐字比对通过，该比对由 CI 的 `xtask configdoc` 承担，本项不点名任何运行期断言方法，进程启动路径与关账受理路径上都不存在与本项相关的判定，也不存在退出码 78。
26. 八个反向依赖点按第 0.5 节的三档处置逐条落地并端到端通过：`UnbilledArPort`、`ReceivableExposureQuery` 与 `InvoiceReversalStatusQuery` 三者与阶段 6 第三批同批接线、同批验收，`OverbillingMatchPort` 在交付时一并接入阶段 7 的收货用例，`ReceiptInvoiceMatchQueryPort`、`PurchaseCreditNotePort`、`SupplierStatementQuery` 与 `PayableLedgerQuery` 承接四条整条推迟的分支，其中 `PayableLedgerQuery` 一条含阶段 7 第 4.5 节的付款申请 `INVOICE_PAYMENT` 分支与其占用写入路径，本阶段按原形态实现并端到端通过，同一张采购发票被两张付款申请并发引用时可串行化。外部 owner 的 `PaymentRequestWritebackPort::{register_payment,reverse_payment}` 与 `PurchaseOrderInvoicingPort::{states_after_seal,record_invoice,reverse_invoice}` 也在第 4.3/4.11 节具名调用点用同一 proof 真实接线，逐发票 delta、订单已开数量/直接费用状态与 reservation 终检均通过；`PurchaseInvoiceCaptureReversalBasisQuery::lock_available` 在 proof 后按固定叶片锁序真实接线，独立物料/直接费用红字均按 current live role、维度与 exact leaf 父拆静态 measure，initial invoice 与 linked-return 则分别保持逐行 NewRoot 与父空新差额。两个 wiring 目录下的全部文件中不出现任何占位实现类型，本阶段不开也不关任何降级窗口，`SALES.SALES_RETURN.INVOICE_NOT_CREDIT_NOTED` 判定与交付确认三腿的过渡科目净额断言在本阶段一次真实通过。
27. `invoice.tax_rate_options` 的建表迁移 `V20261019090000__invoice_create_tax_rate_options.sql` 与种子迁移 `V20261019091500__invoice_backfill_seed_tax_rate_options.sql` 已在 T0 期间执行且六档出厂预置可查，全库取默认税率只有 `ep_contract_invoice::TaxRateOptionQuery::default_rate` 一条路径，任何阶段不提供税率桩，本阶段不含任何从 mdm 迁移税率的回填迁移。
28. 期初导入通道在四个方向上各通过一次，导入后首个会计期间的八个 finance SQL 勾稽 view 与两个 snapshot owner port 组装出的十项差额全部为零，且该通道不产生任何凭证。
29. 本阶段按裁定 A-06 不实现也不注册任何 `ReconCheck`，原定的 `FIN_CROSS_MODULE_LINK` 整条删除。判定方式为本阶段新增与修改的代码内不出现 `ep_platform_recon::ReconCheck` 的实现体与 `ReconRegistry::register` 的调用。本阶段对内部对账的贡献只有取数一侧，即第 3.3 节的八个 SQL view、两个 snapshot owner port 与 `ReconciliationItemQuery` 的十项组装；其比较由阶段 9b 的子账与总账勾稽 `ReconCheck` 驱动，该项的验收见本节第 8 条，本条不重复判定。
30. `platform_core.sensitive_field_registry` 中存在 `finance.cash_accounts.bank_name` 与 `bank_account_no` 两行，两行 `is_field_encrypted` 均为真且 `security_level=30`；`db/checks/11` 返回零行，即两组 `_enc bytea + _key_ref text` 存在、同名明文列均不存在，账号另有 tail、EXACT 盲索引与固定名称的 `octet_length(bank_account_no_bidx)=32` CHECK。列表、详情完整查看及含银行字段导出逐项覆盖 F-51 U-A-12 的权限、重新认证、审批和审计。
31. 第 0.2 节与 F-50 第 3.4 节的自动核销规则已实现：销项开票同凭证借预收、贷应收，采购发票登记同凭证借应付、贷预付；计量项、双侧效果行、逐资金根追溯、锁后上限、期间切片和事务末勾稽全部通过真实 PostgreSQL 用例，且不存在第二张凭证或台账单边写入。
32. 第 0.0 节列出的五项 T0 最小切片在 T0 判定时已经跑通并保持可回归，即最小销项发票、最小应收条目、一笔到款与一次全额核销、一个资金账户建档、税率字典建表与种子及 `TaxRateOptionQuery` 五项在 `ep-datagen` 最小样本上通过且应收一项勾稽差额为零，销项发票的 `tax_rate` 取自 `TaxRateOptionQuery::default_rate`；本阶段的全部其余交付物在该骨架上加厚，`testkit/scenarios/stage10_ar_ap_closed_loop.rs` 复用 T0 的开票与到款两段步骤函数，全卷不存在第二条首次贯通路径。
33. 门户发票上传记录的 `UPLOADED → ACCEPTED` 一路在本阶段一次真实通过：以一条阶段 7 交付的 `UPLOADED` 上传记录为输入执行 `register_purchase_invoice`，登记成功后该记录 `status` 为 `ACCEPTED` 且 `accepted_purchase_invoice_id` 等于本次采购发票的 `id`；回写与采购发票落库在同一事务内，注入登记失败后该记录仍为 `UPLOADED` 且无孤立的采购发票行。该回写经第 4.11 节第 9 步的写端口完成，本阶段代码内不出现对 portal schema 任何表的直接写入，判定方式为本阶段新增与修改的代码内不出现 `portal.` 前缀表名的写语句。阶段 7 推迟到本阶段的 E2E-T-03 受理路径随本条一并判定。
34. 规格第 21.4 章要求的专业签字已取得并留档：会计与税务在本阶段签字，签字人资格证据随版本留档；签字缺失或不通过时本阶段不得退出，整改后重新测试并重新签字，不得以未记录的方式豁免（规格第 22 章第 12 条）。本条由裁定 F-42 新增，此前四份计划的退出条件中无任何签字项。
35. GRNI 追加效果链通过真实 PostgreSQL 验收：收货 100 元在 M1 写 `GOODS_RECEIPT/INCREASE=100`，M2 分次进票写 `PURCHASE_INVOICE/DECREASE=40` 与 60 后，各期间余额依次为 100、60、0；独立数量红字 30 在 M3 写 `PURCHASE_CREDIT_NOTE/INCREASE=30` 后余额为 30，而 `NONE+ADJUSTED` 的纯价格/税额红字不写 GRNI。未收票采购退货直接写 `PURCHASE_RETURN/DECREASE`；已收票实物退货必须同事务先写红字 INCREASE、再逐父等额写退货 DECREASE，序列 `+100,-100,+30,-30` 终值为零；同单混合已开票与未开票数量两段合计等于退货数量。链接实物退货的红字凭证不写库存/成本腿，退货凭证与红字凭证合计后 GRNI 净变动为零。部分效果尾差在末次被吸收。同一根两张发票并发只有一个合法串行结果；跨法人根、错收货行、同方向父子、累计数量或金额超根/父均被数据库或事务末约束拒绝；失败时采购发票或红字、GRNI 效果、库存价差、应付、凭证、审计与 Outbox 全部回滚。`GrniSubledgerBalanceQuery` 只读 procure 表、按 period seq 截至聚合，后续事件不改变旧期间结果，且 GRNI 子账与 `ACCOUNTS_PAYABLE_ACCRUED` 总账差额始终为零。红冲行另以真实 PostgreSQL 直写覆盖 `source_effect_seq` NULL/重复/跳号、非末次 `ORIGINAL_UNIT_PRICE` 金额偏差与已有 `ADJUSTED` 后伪装末次尾差，均须由数据库约束拒绝。
36. `ContractTerminationReceiptPlanImpactRule` 与 `ContractTerminationSalesInvoiceImpactRule` 已以 `CLM_TERM_RECEIPT_PLAN`、`CLM_TERM_SALES_INVOICE` 两个 code 注册，`ImpactRegistry` 累计真实注册数恰为 6；`ReceiptPlanBillingQuery::billing_by_period(...)->BTreeMap<i32,Money>` 只聚合 `invoice.invoice_receipt_plan_links` 的 `ISSUE-VOID-RED_LETTER`，合同终止只以逐期 `amount>0` 判定占用，收款计划自动作废、销项票 VOID/RED_LETTER/KEEP 三个具名决策码、各自 `decision_result_doc_id` 形状和统一三态结果均通过第 8.1、8.3 节真实用例。clm 不存在已开金额列或写端口，Outbox 不存在已开金额回写事件/消费者，两个 wiring 目录无 ImpactRule 替身，且不存在第二个合同终止消费者或第二套净已开金额聚合。

---

### 10. 与规格和 PRD 的对应

#### 10.1 规格条目

| 规格章节 | 本阶段实现的内容 |
|---|---|
| 5.2 财务条目 | 应收台账、应付台账、发票申请与审批与开具登记、与合同收付款计划及订单勾稽、按发票核销到款、分次到款与分次付款、一笔款项核销多张发票、客户退款与供应商返款、预收台账与预付台账、银行与现金账户档案、三类事件的资金腿明细视图、资金流水不得独立登记 |
| 5.2 数电发票与税务条目 | 销项发票开具登记的八个字段、回写申请单状态与剩余可开比例、销项作废以及销项与进项两个方向的红字冲销登记、回滚后按剩余可开比例重新开具、单一税种增值税、按实际开票结果登记的语义、人工录入与批量导入两条路径 |
| 5.2 财务规则条目事件-分录表 | 开票、采购发票登记、到款、付款、退款、红字冲销与作废六类事件的调用与落库，其中采购发票登记的单据本体按裁定 A-10 也在本阶段；采购发票事件的应付腿与超量开票腿；交付确认与销售退货两类事件在过渡科目上的子账腿，交付确认腿经 `UnbilledArPort::record_on_delivery` 由阶段 6 调用 |
| 5.2 到款与付款的核销顺序规则块 | 默认按单据到期日升序、同日按单据编号升序；人工指定写入审计 |
| 5.2 超量开票的三条结清路径规则块 | 挂账登记与三条路径的完整实现，含路径三之后到货的先冲回再入账顺序 |
| 5.2 总账功能与期末处理块 | 记账日期的取值与校验；凭证与子账共用同一会计期间字段；顺延只改变期间归属不改变取价；两个日期与两条检索路径 |
| 7.7 法人行级隔离机制 | 40 张表的统一 RLS 策略、无 `BYPASSRLS`、跨法人查询按法人逐个设置变量 |
| 7.8 密钥域 | 银行名与银行账号均按法人密钥域字段级加密存储，分别使用 `_enc + _key_ref` 且不保留同名明文列；展示、完整查看、导出与审计按 F-51 U-A-12 |
| 8 黄金业务闭环第 6、7、9、10、11 步 | 发票申请、发票开具登记与冲销、到款登记、付款登记、退货相关的退款与返款 |
| 10.2 主系统规则 | 待处理超量开票科目的子账侧口径与关账拦截的可达解除路径；本阶段不产生异步过账条目的声明 |
| 12.1 与 12.2 | 开票、付款、财务过账三类高风险操作的重新认证与审批；申请人不可自审 |
| 12.5 审计 | PRD 第 6.14.4 的 11 类动作与业务变更同事务写入审计 |
| 15.1 错误分类 | 五类分类中本阶段涉及的四类；错误封套的七个必含要素 |
| 15.2 可靠任务 | 守恒违例、凭证不平、勾稽差额三类进入死信与人工修复 |
| 16 与附录 A.1、A.2 | 第 8.6 节的十项度量 |
| 17.2 财务内核测试 | 应收应付核销的三项、十五类必测分支中的第二、五、六、八、九、十三类 |
| 17.3 强制不变量 | 应收应付核销守恒；子账与总账勾稽十项全部；已过账凭证不可覆盖由仅追加表与冲正路径承载 |

#### 10.2 PRD 节

| PRD 节 | 本阶段实现的功能 |
|---|---|
| 6.1.4 | 记账日期字段与三条校验；提交结果展示实际落入的会计期间并显式标注顺延；资金流水不得独立登记 |
| 6.1.5 | 登记语义的六条含义，含批量导入不放宽任何校验 |
| 6.2 全节 | 资金账户档案的九个字段、两条状态流转、资金腿明细视图的四列、五种异常 |
| 6.3 全节 | 发票申请单的十个字段、三项校验、审批要求、11 条状态流转、剩余可开比例、五种异常 |
| 6.4 全节 | 开具登记的十二个字段、四项系统处理、五项输出变化、批量导入、销项发票状态机、七种异常 |
| 6.5 全节 | 作废与红字冲销的八个字段、四项系统处理、与销售退货的先后关系、五种异常 |
| 6.6 全节 | 进项发票台账的四项内容、采购发票登记与三单匹配、进项方向的冲销登记、供应商价格调整与金额税额更正的登记路径、三种异常；台账两张表由本阶段建立，见裁定 A-10 |
| 6.7 全节 | 到款登记的九个字段、核销明细行与核销顺序的六条规则、剩余款项与预收账款、五项输出、状态机、七种异常 |
| 6.8 全节 | 付款登记的读取信息、十个字段、核销与分次付款的五条规则、高风险控制、六项输出、状态机、七种异常 |
| 6.9 全节 | 应收台账的九类信息、核销守恒、核销关系的四条规则、账龄的五条规则、查询与权限 |
| 6.10 全节 | 应付台账同构内容；已收货未收票视图完整实现，子账侧经阶段 7 定义并实现的 `ep_contract_procure::GrniSubledgerBalancePort` 取数，本阶段只注入；待处理超量开票视图 |
| 6.11 全节 | 预收台账与预付台账的六条与五条规则、三条用户可见校验、无人工新增与调整入口 |
| 6.12 全节 | 两类退款单的区分、十一个字段、四项校验、五项输出、状态机、直运情形、五种异常 |
| 6.13 全节 | 十个勾稽项的对账视图全部、三条用户可见规则、八条本节内部勾稽 |
| 6.14 全节 | 四类错误的应用、四类不平的死信处置、幂等与重复提交、11 类审计 |
| 6.16 | 21 条已关闭事项的现行规范值、承载方式与未来正式变更代价，见第 0.4 节 |
| 4.7.3 与 4.7.4 | 付款登记完成后回写付款申请的已付金额与状态，经 `ep-contract-procure` 写端口 |
| 4.9.6 | 供应商门户收付款对账查询的取数来源 |
| 8.3.4 | 应收账龄与应付账龄两张基础表的数据来源 |
| 11.3 | 同步等待上限 8 秒，超过转后台任务，本阶段只有批量导入与账龄大范围导出触及该线 |

#### 10.3 本阶段明确不做的事

按 PRD 第 6.15 节全部八条，不扩大也不收窄。另外，本阶段不实现凭证生成本身、不实现科目表与期间管理、不实现库存金额账与任何取价、不实现收货单据本身、不实现销售退货与采购退货单据、不实现合同收付款计划，这六项分别属其他阶段。采购发票登记与三单匹配按裁定 A-10 已归本阶段，不再列入不做清单；三单匹配所需的收货与开放暂估经 `GrniEffectWritebackPort` 锁内读取并回写，暂估回冲金额取其返回值，价差拆分才经 `InventoryVariancePort::split_variance` 取得，本阶段不自行取价。

---

### 11. 风险与预留

#### 11.1 技术风险

| 风险 | 影响 | 应对 |
|---|---|---|
| 本阶段是八个反向依赖点的提供方，其中三项与阶段 6 第三批同批接线、一项在交付时接入阶段 7 的收货用例、四项承接整条推迟的分支 | 本阶段延期会连带推迟阶段 6 的交付确认过渡科目腿、信用敞口两桶与销售退货已开票分支，以及阶段 7 的采购退货已开票分支、供应商门户对账与付款申请的 `INVOICE_PAYMENT` 分支 | 按第 0.5 节的三档处置排期，三项同批接线与阶段 6 第三批一并列入联调冒烟；整条推迟的四项在其调用方阶段以硬阻断示明，不留任何返回固定分支的实现，也不开任何降级窗口；第 9 节退出条件第 26 条对八项逐条判定 |
| F-50 第 3.4 节自动核销规则实现偏离 | 预收预付与应收应付会出现台账、凭证或历史切片单边变化 | 计量项固定为 `advance_auto_applied_amount`；销项同凭证借预收贷应收、进项同凭证借应付贷预付；双侧效果行逐资金根一致，分录层与勾稽层均设真实数据库测试，任一偏离阻断阶段退出 |
| 收款期次分摊反向累计在并发红冲下可能短暂越界 | `billing_by_period` 若读到负数会让合同守卫得到错误答案 | 冲销事务锁原票与正向分摊根；延迟约束强制同根 `VOID+RED_LETTER` 不超 `ISSUE`；查询对任一负数返回勾稽故障而不当作零 |
| 分次红冲与同时核销可争用同一 `effective_open` | 可能产生负有效余额或释放不足 | 按统一锁序锁主条目、根、效果与冲销行；锁后按 `L=max(0,current_reversal_gross-effective_open_before)` 两级 LIFO 释放，数据库和事务末同时校验 |
| F-16 逐行落库的冻结取值 | 若改为整体回滚，逐行幂等键设计作废 | 导入器把逐行处理与批次编排分离，切换只改编排层 |
| 账龄查询在 6 万条应收条目与 12 个期间跨度下可能触及 10 秒线 | 附录 A.1 的两项报表度量不达标 | 账龄计算下推到数据库聚合而不是应用侧循环；分组键固定为四个；单次查询期间跨度由 `EP__FINANCE__RECON__MAX_PERIODS_PER_QUERY` 限制；不达标时按规格第 16 章执行性能整改，不放宽通过线 |
| 守恒 CHECK 违例在高并发下成为主要失败源 | 用户看到较多 `BUSINESS_CONFLICT` | 按统一锁序取得全部锁后重读权威效果与余额，冲突时回带最新余额，界面可一键重取；冲突次数进指标 `ep_finance_settlement_conflicts_total`，超阈值时只调整候选预取、不得改变锁后计算语义 |
| 银行账号字段级加密与查重的组合 | 确定性盲索引会暴露同法人同字段内的相等关系，并存在字典攻击面 | 按裁定 B-04 直接使用阶段 2 提供的 `derive_blind_key` 与固定 32 字节 `BlindIndex`，密钥自法人数据加密密钥域派生且不落库；该列只用于业务唯一约束，不提供通用检索入口；本阶段不实现任何自有哈希、截断或宽度配置 |

#### 11.2 为后续阶段预留的扩展点

1. `ep-contract-finance` 的 `ReceivableExposureQuery` 端口返回应收未收金额与已交付未开票金额两项，销售阶段在 `ep_contract_sales::CreditExposureQueryPort` 内消费该结果并补上在途订单金额，对外唯一入口为销售侧端口，对应规格第 5.2 章客户信用额度校验条目的三部分构成，见裁定 C-14。
2. inventory 与 GRNI 两项不建立 SQL view；本阶段在 `ReconciliationItemQuery` 中注入阶段 8 的 `InventorySubledgerBalanceQuery`（`ep_contract_inventory::StockValueSubledgerBalancePort` 的实现）与阶段 7 的 `GrniSubledgerBalanceQuery`（`ep_contract_procure::GrniSubledgerBalancePort` 的实现），并在同一 `SnapshotCtx` 上组装。后续版本新增子账来源时，在该来源模块的 `ep-contract-<m>` 增加一个同形 snapshot port、由来源模块定义并实现、由本模块在组装处接入并在两个 wiring 目录注册，不伪造跨模块 SQL view，见裁定 B-08 与 G-01。
3. `finance.receivable_entries.source_doc_type` 与 `finance.payable_entries.source_doc_type` 首版已分别封闭为原票、`INVOICE_REVERSAL`、`MIGRATION_OPENING`；新增其他 AR/AP 来源必须同时扩展条目形状 CHECK、历史切片与对账测试，不得只放开文本枚举。
4. `invoice.sales_invoice_lines` 已是现行必备表；后续增加新明细类型必须保持税率只在行上、头三项只求和。`invoice.invoice_receipt_plan_links` 是独立的逐期分摊效果链，不可替代发票行，也不可被 clm 金额副本替代。
5. `finance.cash_ledger_entries.source_doc_type` 为可扩展枚举，后续版本引入银企直连流水时新增取值即可，但资金流水不得独立登记的约束在首版必须保持。
6. 对账视图的十项统一为同一子账 DTO `ReconciliationItemView { item_code, legal_entity_id, accounting_period_id, subsidiary_amount }`，由 `ReconciliationItemQuery::items` 在快照上一次返回；阶段 9b 的对账组件另经 `TotalAccountBalanceProvider` 取得总账侧并产生 `ledger_amount/difference`，两字段不反向进入 finance contract。
7. 报表阶段的应收账龄与应付账龄两张基础表直接消费 `finance.v_receivable_aging` 与 `finance.v_payable_aging`，不另建一套口径；分档定义按裁定 C-08 在阶段 11 迁到 `reporting.aging_bucket_profiles` 与 `reporting.aging_bucket_lines`，届时两个视图的分档取数改经 `ep_contract_reporting::AgingBucketQuery`。
8. 门户阶段的供应商对账查询消费 `ep-contract-finance::SupplierStatementQuery`，脱敏投影在门户侧完成，本阶段不返回任何脱敏后的数据结构，避免两套口径。

#### 11.3 需回写共享基线的项

1. 第 5.4 节：`request_hash` 的计算排除 `X-Reauth-Token`。
2. 第 9.2 节：新增三个指标 `ep_finance_settlement_conflicts_total`、`ep_finance_reconciliation_difference_amount`（gauge，标签 `item`、`legal_entity_id`）、`ep_invoice_import_rows_total`（counter，标签 `outcome`）。
3. 第 11 节：新增两项全局取值，即资金账户期初余额的存在与勾稽要求、资金类单据的冲正登记路径（临时闭合 U-D-02）。
4. 第 3.5 节：确认本阶段全部金额列均为 `numeric(18,2)`、比例列为 `numeric(9,6)`、数量列为 `numeric(18,6)`，本阶段未引入任何新的精度语义。
