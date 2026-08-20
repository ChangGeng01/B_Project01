## 阶段 10：财务内核二 —— 往来与发票

本阶段交付 invoice 与 finance 两个模块码的契约层、领域层、应用层与数据库结构，覆盖销项发票申请与开具登记、进项发票台账与采购发票登记及三单匹配、销项与进项两个方向的作废与红字冲销登记、应收应付台账与账龄、预收预付台账、到款与付款登记与核销、客户退款与供应商返款、资金账户与资金腿明细、应收账款未开票过渡科目子账、待处理超量开票子账与三条结清路径、往来与预收预付的期初余额导入，以及规格第 17.3 章与 PRD 第 6.13.1 合计十个勾稽项的对账视图。

按跨阶段裁定 A-10，进项发票台账归 invoice 模块，因此 `invoice.purchase_invoices` 与 `invoice.purchase_invoice_lines` 两张表、采购发票登记用例与三单匹配一并归本阶段，采购阶段不建表也不写台账，与基线第 1.2 节 invoice 覆盖销项与进项发票台账一致。

本阶段不定义任何借贷方向、科目、取价与拆分规则。全部账务处理指向规格第 5.2 章财务规则条目的事件-分录表及其后七个规则块，本文只写它在哪一步被调用、以什么参数被调用、结果落到哪张表。

---

### 0. 本阶段的口径前提与显式假设
#### 0.0 本阶段在 T0 贯通线上的最小切片

在阶段 3b-1 结束后、阶段 5 全量开工之前插入一条不新增任何范围的最薄贯通线 T0，判据是一条合同从建单走到管理层看到一个数。本阶段向 T0 贡献五项最小切片，全部取自本阶段既有交付物，不新增表、不新增端点、不新增契约：`invoice.sales_invoices` 的一张最小销项发票（数电、单税率、单行金额、不带影像附件）、`finance.receivable_entries` 的一条应收明细条目、`finance.receipts` 与 `finance.receivable_settlement_links` 的一笔到款与一次全额核销、`finance.cash_accounts` 的一个银行账户建档、`invoice.tax_rate_options` 的建表与种子及 `ep_contract_invoice::TaxRateOptionQuery` 的 `default_rate` 与 `list` 两个方法。第五项按总览第 1.5 节第五条与第十条定死归本阶段并在 T0 期间交付，理由是 T0 要开一张销项发票、T0 内的合同行也要取默认税率，而该表是税率字典的唯一出处，全卷不设第二个税率来源，也不存在任何税率桩。承载前四项的最小用例为 `issue_sales_invoice`、`register_receipt` 与 `maintain_cash_account` 三个，最小端点为 `POST /api/v1/invoice/sales-invoices`、`POST /api/v1/finance/receipts` 与 `POST /api/v1/finance/cash-accounts` 三个；第五项不设端点，取用只经 `TaxRateOptionQuery`。发票申请单在 T0 中只走单审批节点。

该五项对应的迁移按第 3.6 节的编号逐条列出：第一项取 invoice 目录第 2 号 `V202611030905__invoice_create_invoice_applications.sql`、第 3 号 `V202611030910__invoice_create_invoice_application_link_tables.sql`、第 4 号 `V202611030915__invoice_create_sales_invoices.sql` 与第 8 号 `V202611030925__invoice_create_invoice_receipt_plan_links.sql`；第二项取 finance 目录第 3 号 `V202611031010__finance_create_receivable_entries.sql`；第三项取 finance 目录第 10 号 `V202611031045__finance_create_receipts.sql` 与第 15 号 `V202611031070__finance_create_settlement_link_tables.sql`；第四项取 finance 目录第 2 号 `V202611031005__finance_create_cash_accounts.sql` 与第 16 号 `V202611031075__finance_create_cash_ledger_entries.sql`；第五项取 invoice 目录第 1 号 `V202611030900__invoice_create_tax_rate_options.sql` 与第 13 号 `V202611030950__invoice_backfill_seed_tax_rate_options.sql`。行级安全与索引两支迁移按 schema 整目录一次建齐，不能按切片拆分，因此 invoice 目录第 11 号与第 12 号、finance 目录第 18 号与第 19 号在 T0 期间随两个目录的建表文件一并执行，T0 只在上述五项涉及的表上写入数据。

T0 明确不要求的部分在本阶段一律不提前：不用 `ep-datagen` 的基准规模数据集，只用最小样本；不要求分支覆盖，进项发票与三单匹配、作废与红字冲销、预收预付自动核销、超量开票三条结清路径、退款与返款、资金单据冲正、账龄与期初导入一概不进 T0；不要求四端，只要求桌面端；不要求十项勾稽全绿，只要求应收一项在最小样本上差额为零。第 0.5 节列出的八个反向依赖点在 T0 中一个都不出现，因为 T0 不含交付确认、不含采购侧、不含退货。

T0 通过后，本阶段其余全部内容改为在这条已贯通的骨架上加厚，即在已经跑通的销项发票、应收条目、到款与核销之上追加上一段列举的各项，不再有第二次首次贯通的动作。M7 相应保留为全分支闭环的判定点，不再是黄金业务闭环的首次贯通点。

#### 0.1 过账时机：同步在业务事务内生成凭证

本阶段的决定：由本阶段单据直接触发的八类事件（开票、采购发票登记、红字冲销与作废、到款、付款、退款、超量开票路径二结清、超量开票路径三结清），其总账凭证、子账台账条目与核销关系在同一个业务数据库事务内同步写入，凭证经 `ep-contract-ledger` 的过账端口生成，本阶段不向 Outbox 投递任何需要异步过账的条目。该决定与跨阶段裁定 C-28 一致：全部凭证一律与业务事件同事务生成，Outbox 只承载派生、通知、检索与报表数据集。

理由有三条。其一，PRD 第 6.4.8、6.7.7、6.8.8 三张异常表都把“凭证生成失败或借贷不平”的系统行为定为“提交失败并进入死信与人工修复”，提交失败只有在凭证与业务写入同事务时才成立。其二，核销明细行的逐行上限校验必须读到该单据当前的未核销余额，异步过账会让紧邻的两次登记读到过期余额。其三，规格第 5.2 章要求同一业务事件的子账条目与凭证共用同一个会计期间字段，同事务写入使该要求成为结构性事实而不是运行期约定。

与规格第 10.2 章关账受理前提二的关系：受理前提二的判定语句按跨阶段裁定 C-28 固定为一句话，本阶段与阶段 4、阶段 9 三处逐字一致，即该法人该期间内，`platform_msg.outbox_events` 中 `status` 属于 PENDING 或 DISPATCHING、`posting_date` 落在该期间起止之间、且 `event_type` 命中 `ledger.posting_trigger_event_types` 的条目数为零，且 `platform_msg.dead_letters` 中 `state` 属于 OPEN 或 REPAIRING、同样命中该注册表的条数为零。`posting_date` 为空的平台事件一律不计入，理由是它们不产生凭证。判定所用视图固定为 `ledger.v_pending_posting_backlog`，两个错误码固定为 `LEDGER.PERIOD_CLOSE_REQUEST.PENDING_POSTING_BACKLOG` 与 `LEDGER.PERIOD_CLOSE_REQUEST.UNREPAIRED_DEAD_LETTERS`，视图与错误码均由阶段 9a 提供。

本阶段发布的 12 个事件中有 8 个在 `ledger.posting_trigger_event_types` 中有登记行，按裁定 A-21 该登记行由阶段 9a 的种子迁移一次写入，本阶段不新增任何回填迁移，见第 5.8 节末表。这 8 个事件的凭证已在业务事务内生成，Outbox 条目只驱动派生传播，即站内通知、报表投影、检索索引与客户 360 视图；但按上述判定语句，它们在被消费完毕之前仍计入该计数，因此关账受理需等待其消费结束，而不是等待任何异步过账。

与顺延入账的关系：会计期间在业务事务内由 `AccountingPeriodResolver` 一次解析并同时写入凭证与全部子账条目，因此规格第 10.2 章“受理时点在途写事务”这一集合天然覆盖本阶段的在途提交，等待该集合结束后建立的快照必然包含这些凭证。

本节口径已由跨阶段裁定 C-28 定死，不再是待整合议题。本阶段不存在也不预留任何异步过账路径，第 5.8 节的全部事件消费方均不做过账，核销上限校验只读已落库的未核销余额，不引入待过账占用量。

#### 0.2 规格缺口：自动核销预收预付的分录腿

规格第 5.2 章事件-分录表的开票事件与采购发票事件，其分支与附加规则列没有覆盖到款事件与付款事件所要求的“后续开票时按同一合同的收付款计划自动核销预收账款”与“后续采购发票登记时自动核销预付账款”。若自动核销只动台账不动总账，预收台账余额下降而总账对应科目余额不变，规格第 17.3 章的预收与预付两项勾稽必然破裂。

本阶段把该缺口登记为规格回写项，不在本计划里给出借贷方向、科目与金额口径，也不授权实现按任何假设先行。回写落点为规格第 5.2 章事件-分录表的开票事件与采购发票事件两行的分支与附加规则列，决策人为财务负责人，截止点为本阶段开工日，即预收预付自动核销的分录腿第一次接线之前。回写完成前本阶段只冻结机制：自动核销的分录腿与该事件凭证写在同一张凭证内，经 `ep-contract-ledger` 过账端口的附加腿参数传入，不新增事件类型；分录内容一律以回写后的规格第 5.2 章为准。该回写是本阶段的阻塞前置，判定见第 9 节退出条件第 31 条。

#### 0.3 规格缺口：资金账户期初余额

PRD 第 6.2.2 的资金账户字段表没有期初余额，而第 6.2.4 的资金腿明细视图要求展示期初余额，规格第 17.3 章又要求资金流水台账按账户的余额合计等于银行存款科目余额。上线首期若科目有期初余额而资金腿明细没有，该项勾稽在首期即为非零差额。

本阶段的显式假设：`finance.cash_accounts` 增加 `opening_balance` 与 `opening_balance_period_id` 两列，建档时录入一次，建档后不可修改；同一法人下全部银行存款类账户的期初余额合计必须等于总账银行存款科目的期初余额，现金类账户同理，该等式由本阶段的对账视图逐期校验。

按跨阶段裁定 A-24，本项目不设独立的数据迁移阶段，期初与历史数据按模块归属分落三处：总账期初余额归阶段 9a 的期初余额批次；库存期初归阶段 8 的 `MIGRATION_STOCK_ADJUSTMENT` 来源类型；应收应付预收预付期初与资金账户期初归本阶段，前者经第 4.12 节的期初导入通道，后者经本节两列在建档时一次录入。四个通道的写入一律不生成凭证，两侧的平衡由第 3.3 节的对账视图在首个会计期间校验。

#### 0.4 被阻塞的业务决策项与本阶段的临时取值

下表逐条对应 PRD 第 6.16 节的 F 编号与附录乙的 U-D 组编号，另含 U-A-12 一条。本阶段不代替决策人决策，F 与 U-D 两组的决策人为财务负责人，U-A-12 的决策人为安全负责人与产品负责人；每一条都给出临时取值，否则表结构与校验无法落地。临时取值一律以配置项、配置发布对象或登记行承载，切换时不改表结构的标注为低代价。

| 编号 | 临时取值 | 承载方式 | 切换代价 |
|---|---|---|---|
| F-01 / U-D-03 | 发票号码 `text` 且 `char_length <= 64`，法人内唯一；发票代码 `text` 且 `char_length <= 32`，可空，发票种类为数电时必须为空，为纸质时必填 | 表列与 CHECK 约束，发票种类为 `invoice_kind` 列 | 低，改 CHECK 与校验函数 |
| F-02 / U-D-04 | 税率取自 `invoice.tax_rate_options`，出厂预置 0.130000、0.090000、0.060000、0.030000、0.010000、0.000000；**按裁定 F-45 决定二与 F-10 B-8**：一张发票**允许多税率**，每个行明细各自带税率；**允许多行明细**，须新增 `invoice.sales_invoice_lines` 与分摊逻辑。本行原写「一张发票单税率、单行金额，不做多行明细」，已被上述两条裁定两面作废 | 出厂预置由第 3.6 节 invoice 目录第 13 号种子迁移 `V202611030950__invoice_backfill_seed_tax_rate_options.sql` 在 T0 期间写入，见第 0.0 节第五项；出厂预置之后的增删改按裁定 A-27 经配置发布对象由阶段 3b 的发布通道写入 `invoice.tax_rate_options`。按裁定 C-11 与总览第 1.5 节第五条，该表是税率字典的唯一出处，唯一取用入口为 `ep_contract_invoice::TaxRateOptionQuery` 的 `default_rate` 与 `list` 两个方法，任何阶段不得另设税率桩 | 税率集合为低；多行明细为中，需新增 `invoice.sales_invoice_lines` 与分摊逻辑 |
| F-03 / U-D-05 | 舍入按共享基线第 3.5 节；容差判据为 `abs(tax_amount - round(net_amount * tax_rate, 2)) <= tolerance`，`tolerance` 默认 0.02 | 配置项 `EP__INVOICE__TAX__AMOUNT_TOLERANCE` | 低 |
| F-04 / U-D-06 | 剩余可开比例的计算基数为合同金额；比例列类型 `numeric(9,6)`；累计比例校验容差 0.000001 | 配置项 `EP__INVOICE__RATIO__TOLERANCE` | 基数改为订单金额合计为中，需改取数与回滚公式 |
| F-05 / U-D-07 | 申请金额不可人工改写，等于 `round(开票比例 * 合同金额, 2)` | 领域规则 | 低 |
| F-06 / U-D-08 | 开票内容为自由文本，长度上限 500 | 表列 CHECK | 改为逐行对应为中 |
| F-07 | 开具登记不强制上传影像附件 | 配置项 `EP__INVOICE__ISSUE__REQUIRE_IMAGE_ATTACHMENT`，默认 false | 低 |
| F-08 / U-D-09 | 红字发票号码必填；首版只允许全额红冲，红字不含税金额、税额、价税合计必须分别等于原发票对应值 | 领域规则 | 允许部分红冲为中偏高，需把销项发票状态机拆出部分红冲态并改比例回滚公式 |
| F-09 / U-D-10 | 作废与红字冲销登记复用开票的高风险控制，即重新认证加审批 | 领域规则，不设开关；规格第 12.1 章的高风险控制不得由配置关闭 | 低 |
| F-10 / U-D-11 | 账龄分档为 0 至 30、31 至 60、61 至 90、91 至 180、181 至 360、361 以上，六档 | 按裁定 C-08，本阶段先写入临时表 `finance.aging_bucket_definitions`，只出厂预置一套六档且不提供配置发布入口；阶段 11 交付 `reporting.aging_bucket_profiles` 与 `reporting.aging_bucket_lines` 后迁移并删除本表，按法人分套的形态随阶段 11 一并交付，此后取用入口唯一为 `ep_contract_reporting::AgingBucketQuery::buckets` | 低 |
| F-11 / U-D-12 | 到期日取值优先级为：关联收付款计划行的到期日；缺失时取发票开具日期加往来方档案上的约定账期天数；仍缺失时取发票开具日期 | 领域服务 `DueDateResolver`，账期天数经 `ep-contract-mdm` 读取 | 低 |
| F-12 / U-D-13 | 可核销范围限定为同一法人同一往来方，不允许跨客户或跨供应商核销 | 配置项 `EP__FINANCE__SETTLEMENT__CROSS_PARTY_ALLOWED`，默认 false | 放开为中，需改越权测试集与账龄归属 |
| F-13 / U-D-14 | 到款登记不需重新认证也不需审批；客户退款与供应商返款需重新认证加审批；资金账户档案的新增、修改与停用需审批不需重新认证 | 到款审批与资金账户审批两项恢复为配置项 `EP__FINANCE__RECEIPT__REQUIRES_APPROVAL` 与 `EP__FINANCE__CASH_ACCOUNT__REQUIRES_APPROVAL`，见第 7 节；退款与返款的重新认证为领域规则，不设开关，理由是规格第 12.1 章第 1050 行把付款与财务过账列为必须重新认证的六类高风险操作，该项不得由配置关闭 | 低 |
| F-14 / U-D-02 | 新增资金单据冲正登记单，见第 4.7 节，是本阶段为闭合该缺口而新增的单据类型 | 表 `finance.cash_document_reversals` 与同名用例 | 若财务负责人另选路径，改动为一张表加一个用例，中等 |
| F-15 / U-D-15 | 可退上限见第 4.8 节的算法 | 领域服务 `RefundCapCalculator` | 低 |
| F-16 | 批量导入单次上限 2000 行，逐行独立事务落库，失败行不回滚已成功行，逐行返回原因 | 两个配置项 `EP__INVOICE__IMPORT__MAX_ROWS` 与 `EP__INVOICE__IMPORT__ON_ROW_FAILURE`，见第 7 节 | 改为整体回滚为中，需把导入改为单事务并放弃逐行幂等 |
| F-17 / U-A-12 | 银行账号按规格第 7.8 章强制纳入行内敏感字段并做字段级加密，该项不待决；待决的只有三问，即开户银行是否同列敏感字段清单、列表与详情与导出三场景的脱敏形态、导出是否触发重新认证。三问的临时取值为开户银行在本阶段不单列登记行、三场景同取 `KEEP_LAST_4` 且后 4 位取自 `bank_account_no_tail`、导出是否重新认证一律指向阶段 4 的重新认证判定函数；详情看全值需字段级权限 `finance.cash_account.bank_account_no.read_full`，字段级密级 30 | 一行登记落 `platform_core.sensitive_field_registry`，见第 3.2.2 节与第 3.6 节的 backfill 迁移；字段级授权行落阶段 4 的 `platform_authz.field_permissions` | 改该登记行的 `mask_style` 与授权行为低，不改表结构；若改判开户银行也做字段级加密，按裁定 A-28 的切换路径在一次变更内完成三件事，为中 |
| F-18 / U-D-16 | 同一法人允许多个现金账户并存 | 无唯一约束 | 收紧为唯一为低 |
| F-19 | 按共享基线第 11.5 节，本阶段不另取值 | 共享基线 | 无 |
| F-20 | 文案集中在 `docs/error-codes.md`，代码只引用常量 | 共享基线第 10.2 节 | 无 |
| F-21 | 交叉引用按本文正文写明的 PRD 节号，不写节名 | 本文 | 无 |

被阻塞判定：本阶段不因上表任一条被阻塞，全部有可执行的临时取值。风险最高的是 F-08 与 F-16，两者若在阶段结束后才改，会触及已落库数据的语义，需要数据回填。第 0.2 节登记的规格回写项不在本表内，它是本阶段唯一的阻塞前置，判定见第 9 节退出条件第 31 条。
#### 0.5 反向依赖点的处置

本阶段是八个反向依赖点的提供方。原裁定通则第三条允许调用方先注入以 Noop 前缀命名的空实现、由本阶段替换、并把调用方的验收项顺延到本阶段，该通则已删除。删除理由有二：在判定类与记账类端口上，返回零值或恒定业务分支不是缺省而是一个会被记进账的错误答案，且 fail-open 与 fail-closed 的选择被下放到 wiring 里的一行；顺延使调用方阶段的退出条件不再证明任何闭环事实。取而代之的硬规则是三选一，逐个端口择其一并在下表写明。三档一律不得出现返回零值、空集合、固定业务分支或恒定成功的实现，发布装配中不得注入任何占位类型。

| 端口 | 调用方 | 处置 | 落地口径 |
|---|---|---|---|
| UnbilledArPort | 阶段 6 的交付确认与销售退货 | 同批交付 | 按总览第 1.5 节第八条其三与第十条，本端口与 `finance.unbilled_ar_entries` 均不在 T0 内，两者的真实实现与阶段 6 第三批的交付确认用例同批施工、同批验收；接线到位之前阶段 6 的交付确认用例不建立过渡科目腿的调用点，不存在取 None 或取零值的形态，三腿在接线当次一起真实执行 |
| ReceivableExposureQuery | 阶段 6 的信用敞口入口 | 同批交付 | 按总览第 1.5 节第八条其三与第十条，本端口不在 T0 内；两桶取数与 `ep_contract_sales::CreditExposureQueryPort` 的组装与阶段 6 第三批同批施工、同批验收；接线之前阶段 6 的信用校验按端口不可用处理，返回 INFRASTRUCTURE 且可重试，不得按零敞口放行 |
| InvoiceReversalStatusQuery | 阶段 6 的销售退货前置校验 | 同批交付 | 按总览第 1.5 节第八条其三采纳阶段 6 的写法、撤销本阶段原写的整条推迟：本端口与上两行三者一并落在阶段 6 第三批，与本阶段的 invoice 与 finance 端口同批施工、同批验收，本端口不在 T0 内；接线之前阶段 6 只实现未开票分支并对已开票行硬阻断，不注入任何替身；错误码 `SALES.SALES_RETURN.INVOICE_NOT_CREDIT_NOTED` 与本端口同批生效 |
| ReceiptInvoiceMatchQueryPort | 阶段 7 的采购退货 | 整条推迟 | 阶段 7 只实现采购发票未登记分支并对已开票收货行硬阻断，已登记分支与本端口在本阶段同批交付 |
| PurchaseCreditNotePort | 阶段 7 的采购退货 | 整条推迟 | 红字进项发票登记随上一行的已登记分支一并在本阶段落地，阶段 7 不建该调用点 |
| OverbillingMatchPort | 阶段 7 的收货用例 | 同批交付 | 超量开票挂账只由本阶段的采购发票登记产生，本阶段交付之前 `finance.overbilling_entries` 恒为空、路径一没有任何可匹配对象，因此阶段 7 的收货用例在该窗口内不接本端口即为正确行为；本阶段交付本端口时一并完成收货用例的接线 |
| PayableLedgerQuery | 阶段 7 的付款申请占用校验 | 整条推迟 | 按总览第 1.5 节第八条其一采纳阶段 7 的写法、撤销本阶段原写的降级窗口写法：付款申请的 `INVOICE_PAYMENT` 分支、其占用写入路径与 `PROCURE.PAYMENT_REQUEST.AMOUNT_EXCEEDS_OPEN_BALANCE` 判定连同其用例整条推到本阶段，与本端口同批交付；阶段 7 只受理 `PREPAYMENT` 一类并对 `INVOICE_PAYMENT` 硬阻断，不以采购订单金额等更宽口径替代；本阶段不开也不关任何 `PORT_NOT_IMPLEMENTED` 降级窗口 |
| SupplierStatementQuery | 阶段 7 的供应商门户对账 | 整条推迟 | 供应商门户的收付款对账查询入口整条推迟到本阶段，阶段 7 不建该入口 |

本表只改各阶段内部的工作次序，不改任何阶段的范围归属，也不改任何迁移文件的版本号。本阶段不承接来自阶段 6 与阶段 7 的任何顺延验收项，第 9 节退出条件不再逐条复述顺延清单。

---

### 1. 交付物清单

本阶段结束时，下列各项在单台服务器上可运行、可演示、可用自动化用例判定。

1. 两个模块的三层 crate 全部编译通过并接入 `core-server`：`ep-contract-invoice`、`ep-domain-invoice`、`ep-app-invoice`、`ep-contract-finance`、`ep-domain-finance`、`ep-app-finance`。
2. `db/migrations/invoice/` 与 `db/migrations/finance/` 两个迁移目录可离线执行到最新版本，且可按各文件头 `-- rollback:` 段回退到本阶段起点。
3. 36 张业务表与 17 个只读视图在 `ep` 库中建立，其中 invoice 13 张、finance 23 张；17 个视图为 10 个对账视图、4 个业务查询视图与 3 个受治理数据集视图。全部带法人列的表已 `ENABLE` 且 `FORCE` 行级安全并挂上统一策略。
4. 58 个 HTTP 端点在 `/api/v1/invoice/**` 与 `/api/v1/finance/**` 下可用，含 OpenAPI 描述文件 `docs/openapi/invoice.v1.yaml` 与 `docs/openapi/finance.v1.yaml`。相对原计划新增两个：`POST /api/v1/invoice/purchase-invoices` 与 `POST /api/v1/finance/opening-balances/actions/import`。
5. 15 个对外契约 trait 在 `ep-contract-finance` 与 `ep-contract-invoice` 中定义并有实现注册到 `apps/core-server/src/wiring/` 与 `apps/job-worker/src/wiring/` 两个目录，供 sales、procure、clm、portal、reporting、ledger 六个模块调用，清单见第 5.9 节。其中 8 个 trait 按第 0.5 节的三档处置与其调用方同批接线，阶段 6 与阶段 7 不注入任何空实现，本阶段也不承接任何顺延验收。
6. 12 个领域事件登记到 `docs/event-catalog.md` 并可从 `platform_msg.outbox_events` 中查得，其中 8 个在 `ledger.posting_trigger_event_types` 中有登记行，按裁定 A-21 该登记行由阶段 9a 的种子迁移写入且每行只填 `event_type`；本阶段只在 CI 中由 `xtask configdoc` 与 `docs/event-catalog.md` 逐字比对，不进启动自检，不作为关账受理的前置校验，也不交付回填迁移。
7. 规格第 17.3 章与 PRD 第 6.13.1 合计 10 个勾稽项的对账视图全部完整实现，可在应用内按法人与会计期间查询并展示子账侧、总账侧与差额三列。其中存货与已收货未收票两项按裁定 B-08 与 G-01 由本阶段注入阶段 8 的 `InventorySubledgerBalanceQuery`（`ep_contract_inventory::StockValueSubledgerBalancePort` 的实现）与阶段 7 的 `GrniSubledgerBalanceQuery`（`ep_contract_procure::GrniSubledgerBalancePort` 的实现）两个端口实现后接入，不再是外壳。
8. 一条可重复执行的端到端脚本 `testkit/scenarios/stage10_ar_ap_closed_loop.rs`，覆盖规格第 8 章闭环第 6、7、9、10、11 步，并串起规格第 17.2 章十五类必测分支中的第二、五、六、八、九、十三类。该脚本在 T0 已跑通的最小路径上加厚，开票与到款两段直接复用 T0 的步骤函数而不重写；该脚本的步骤函数再供阶段 9b 的 `testkit/scenarios/golden_loop_14_steps.rs` 复用，黄金业务闭环十四步的整体端到端验收落在阶段 9b，本阶段不承担。
9. `ep-datagen` 增加往来与发票子集生成器，可在基准规模下产出销项发票 6 万张、进项发票 4 万张、应收明细条目 6 万条、应付明细条目 4 万条、到款单 3 万张、付款单 2 万张、资金腿明细 6 万条，用于附录 A.1 的应收账龄分析与应付账龄分析两项报表实测。
10. `docs/error-codes.md` 增补本阶段错误码（第 5 节列出的全部错误码，含采购发票登记与期初导入两组，合计 90 个上下），`docs/data-dictionary/invoice.md` 与 `docs/data-dictionary/finance.md` 两份数据字典；`docs/data-dictionary.md` 的单据类型码一节增补 PINV 一码，见裁定 C-26。
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
| ep-contract-invoice | crates/contract/invoice | 装配进 core-server、job-worker | 发票申请、销项发票、进项发票、冲销登记的命令与查询 DTO、事件类型、能力域码与动作类别常量、供 clm、sales、procure、reporting 调用的五个 trait，含 `ReceiptInvoiceMatchQueryPort`、`PurchaseCreditNotePort`、`TaxRateOptionQuery` |
| ep-domain-invoice | crates/domain/invoice | 同上 | 发票申请单与销项发票聚合、剩余可开比例值对象、税额勾稽规则、冲销互斥规则 |
| ep-app-invoice | crates/application/invoice | core-server 承载全部用例，job-worker 承载批量导入任务 | 开具登记、冲销登记、批量导入、采购发票登记与三单匹配、进项红字发票登记、进项发票台账、`InvoiceReferenceCounter` 与两个历史成交提供者 |
| ep-contract-finance | crates/contract/finance | 装配进 core-server、job-worker、portal-gateway 的调用方 | 应收应付预收预付台账查询 DTO、供 invoice、sales、procure、portal、reporting、crm、ledger 调用的 trait |
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
| apps/core-server | `apps/core-server/src/wiring/` 目录注入 15 个本阶段契约实现，另注入阶段 8 的 `InventorySubledgerBalanceQuery` 与阶段 7 的 `GrniSubledgerBalanceQuery` 两个外部端口实现，合计 17 个注入行；路由注册 58 个端点；其中 8 个注入行按第 0.5 节的三档处置与调用方同批接线，两个 wiring 目录中不出现任何 Noop、Stub、Fake、Dummy 前缀的占位类型 | 本阶段追加 |
| apps/job-worker | 注册批量导入任务处理器；按裁定 A-06 本阶段不实现也不注册任何 `ReconCheck`，原定的 `FIN_CROSS_MODULE_LINK` 是纯存在性项，其 `category` 取值 `CROSS_MODULE_LINK` 已随该类别整体撤销，本项不再存在。跨模块单目标引用的存在性由基线第 3.3 节的复合真实外键强制；子账条目与其来源凭证的期间一致由 `ep-contract-ledger::AccountingPeriodResolver::resolve` 在同一 `&mut dyn Tx` 内的记忆化保证，见第 5 节相应段落 | 本阶段追加 |

本阶段不新增进程、不新增 schema、不新增模块码、不新增错误分类、不新增依赖方向。`ep-domain-finance` 与 `ep-domain-invoice` 不依赖对方；两个模块之间只由 `ep-app-invoice` 依赖 `ep-contract-finance`，`ep-app-finance` 不依赖 `ep-contract-invoice`。按裁定 G-01，`ep-app-finance` 另依赖 `ep-contract-inventory` 与 `ep-contract-procure` 两个契约，用于注入两个子账余额端口的实现，其中对 `ep-contract-procure` 的依赖先于本裁定已存在（见第 6.1 节 register_payment 行的付款申请已付金额回写），本裁定只新增对 `ep-contract-inventory` 一条边；三条边方向均为 app 到 contract，落在基线第 1.3 节允许项内，承接方为 `xtask archcheck` 的层位判定，本阶段不另立任何按 crate 逐项比对的期望依赖清单。

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

收款计划行的取数唯一经 `ep_contract_clm::ContractPaymentScheduleQuery::schedules(tx, ctx, contract_id)`，按裁定 C-20 该查询由阶段 6 提供，收付款计划行的唯一表为 `clm.contract_payment_schedules`，`ep_contract_finance::ReceivablePlanPort` 已撤销，本阶段不派生第二套收付款计划。

##### 3.1.6 invoice.invoice_import_batches（批量导入批次）

列为 `doc_no`、`status`（`ck_invoice_import_batches_status` 取值 PENDING、RUNNING、SUCCEEDED、PARTIALLY_FAILED、FAILED）、`total_rows int`、`succeeded_rows int`、`failed_rows int`、`file_object_id uuid`（逻辑引用 platform_file）、`result_object_id uuid`、`started_at`、`finished_at`、`reauth_ref`、`approval_ref`，加公共列。
##### 3.1.7 invoice.purchase_invoices（进项发票，单据类，类型码 PINV）

本表按裁定 A-10 归 invoice 模块，采购阶段不建表也不写台账。

| 列 | 类型 | 可空 | 约束与说明 |
|---|---|---|---|
| doc_no | text | 否 | `ux_purchase_invoices_legal_entity_id_doc_no`；类型码 PINV |
| status | text | 否 | `ck_purchase_invoices_status` 取值 REGISTERED、REVERSED |
| supplier_id | uuid | 否 | 跨模块逻辑引用 mdm |
| purchase_order_id | uuid | 是 | 跨模块逻辑引用 procure |
| invoice_no | text | 否 | 供应商发票号；`ux_purchase_invoices_legal_entity_id_supplier_id_invoice_no` |
| invoice_date | date | 否 | |
| posting_date | date | 否 | |
| accounting_period_id | uuid | 否 | 由 ledger 端口在业务事务内解析 |
| deferred_from_period_id | uuid | 是 | |
| tax_rate | numeric(9,6) | 否 | |
| net_amount | numeric(18,2) | 否 | |
| tax_amount | numeric(18,2) | 否 | |
| gross_amount | numeric(18,2) | 否 | |
| cost_kind | text | 否 | `ck_purchase_invoices_cost_kind` 取值 INVENTORY_TYPE、DIRECT_EXPENSE_TYPE |
| is_credit_note | boolean | 否 | 默认 false |
| reversed_by_id | uuid | 是 | |
| voucher_id | uuid | 是 | 逻辑引用 ledger.vouchers |

索引 `ix_purchase_invoices_legal_entity_id_created_at`、`ix_purchase_invoices_legal_entity_id_purchase_order_id`、`ix_purchase_invoices_legal_entity_id_posting_date`。

##### 3.1.8 invoice.purchase_invoice_lines（进项发票行）

| 列 | 类型 | 可空 | 约束与说明 |
|---|---|---|---|
| purchase_invoice_id | uuid | 否 | 同 schema 外键，`ON DELETE RESTRICT` |
| line_no | int | 否 | `ux_purchase_invoice_lines_invoice_id_line_no` |
| purchase_order_line_id | uuid | 是 | 跨模块逻辑引用 procure |
| goods_receipt_line_id | uuid | 是 | 跨模块逻辑引用 procure |
| material_id | uuid | 是 | 跨模块逻辑引用 mdm |
| quantity | numeric(18,6) | 否 | |
| net_unit_price | numeric(18,6) | 否 | |
| net_amount | numeric(18,2) | 否 | |
| tax_amount | numeric(18,2) | 否 | |
| accrual_reversal_amount | numeric(18,2) | 是 | 暂估回冲金额，由 `InventoryVariancePort::split_variance` 返回后回填 |
| price_variance_amount | numeric(18,2) | 是 | 价差金额，来源同上 |
| is_overbilling | boolean | 否 | 默认 false，为真时在 `finance.overbilling_entries` 生成挂账 |

索引 `ix_purchase_invoice_lines_legal_entity_id_goods_receipt_line_id`，支撑 `ReceiptInvoiceMatchQueryPort` 按收货行判定是否已开票。

##### 3.1.9 附件关联表

`invoice.invoice_application_attachments`、`invoice.sales_invoice_attachments`、`invoice.invoice_reversal_attachments`，列均为 `owner_id`、`attachment_object_id`、`purpose`、`sort_no` 加公共列，按基线第 4 节。

#### 3.2 finance schema

##### 3.2.1 finance.aging_bucket_definitions（账龄分档配置）

列为 `code`、`display_name`、`from_days int`、`to_days int`（`to_days` 为空表示最后一档开区间）、`sort_no`、`is_active`、`deactivated_at`，加公共列。约束 `ck_aging_bucket_definitions_range` 表达 `from_days >= 0` 且 `to_days` 为空或大于 `from_days`。

本表按裁定 C-08 是临时表：账龄分档的唯一出处为阶段 11 的 `reporting.aging_bucket_profiles` 与 `reporting.aging_bucket_lines`。本阶段只由第 3.6 节的种子迁移出厂预置一套六档，不提供任何维护端点，也不建配置发布对象与 ConfigItemApplier，按法人分套的形态随阶段 11 一并交付。迁数据与删表两个迁移文件按裁定通则第五条一律放在 `db/migrations/reporting/` 目录下，文件名为 `V202611031060__reporting_backfill_migrate_aging_buckets_from_finance.sql` 与 `V202611031065__reporting_drop_finance_aging_bucket_definitions.sql`，两个文件均由阶段 11 提供，本阶段一个都不提供，也不在 `db/migrations/finance/` 下建删表文件。本阶段的账龄查询在阶段 11 到位后改经 `ep_contract_reporting::AgingBucketQuery::buckets(tx, ctx, legal_entity_id, ledger_side)`，不保留第二套口径。

##### 3.2.2 finance.cash_accounts（资金账户档案）

| 列 | 类型 | 可空 | 约束与说明 |
|---|---|---|---|
| code | text | 否 | `ux_cash_accounts_legal_entity_id_code` |
| account_name | text | 否 | `ux_cash_accounts_legal_entity_id_account_name`；长度上限 200 |
| account_type | text | 否 | `ck_cash_accounts_type` 取值 BANK、CASH；建档后不可修改 |
| ledger_account_id | uuid | 否 | 逻辑引用 ledger 科目；建档后不可修改 |
| bank_name | text | 是 | BANK 时必填，长度上限 200 |
| bank_account_no_enc | bytea | 是 | BANK 时必填；按规格第 7.8 章法人密钥域字段级加密存储，不保留同名明文列 |
| bank_account_no_key_ref | text | 是 | 记录密钥标识与版本，与 `bank_account_no_enc` 同生共死 |
| bank_account_no_tail | text | 是 | 明文后 4 位，供列表与导出脱敏展示 |
| bank_account_no_bidx | bytea | 是 | 盲索引，取值为 `derive_blind_key(legal_entity_id, 'finance.cash_accounts.bank_account_no', plaintext)`，唯一约束 `ux_cash_accounts_legal_entity_id_bank_account_no_bidx` 建在其上，用于查重而不落明文；按裁定 B-04 直接复用阶段 2 提供的 `derive_blind_key` 与 `BlindIndex`，本阶段不自建第二套哈希 |
| owner_user_id | uuid | 否 | 责任人 |
| opening_balance | numeric(18,2) | 否 | 默认 0，见第 0.3 节 |
| opening_balance_period_id | uuid | 是 | |
| is_active | boolean | 否 | 默认 true |
| deactivated_at | timestamptz | 是 | |
| has_cash_flow | boolean | 否 | 默认 false，首次产生资金腿时置 true，用于 PRD 第 6.2.5 的修改拦截 |
| remark | text | 是 | 长度上限 2000 |

`bank_account_no_enc` 承载的逻辑列 `bank_account_no` 的字段级密级为 30。按裁定 C-06，密级的唯一登记表是 `platform_core.sensitive_field_registry`，本阶段在 `db/migrations/finance/` 追加一支 backfill 迁移向该表登记一行，见第 3.6 节；`platform_authz` 只写 `field_permissions` 的字段级授权行，不承载密级。列命名按裁定 A-28 的全库唯一一套，即 `<语义>_enc bytea` 加 `<语义>_key_ref text`，需要保留掩码尾数的再加 `<语义>_tail text`，需要查重的再加 `<语义>_bidx bytea`。

##### 3.2.3 finance.receivable_entries（应收明细条目）

| 列 | 类型 | 可空 | 约束与说明 |
|---|---|---|---|
| customer_id | uuid | 否 | |
| contract_id | uuid | 是 | |
| sales_order_id | uuid | 是 | |
| sales_invoice_id | uuid | 否 | 跨模块逻辑引用 invoice，不建外键 |
| source_doc_type | text | 否 | `ck_receivable_entries_source_type` 取值 SALES_INVOICE、MIGRATION_OPENING，后者由第 4.12 节的期初导入写入 |
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

结构与 3.2.3 对称，差异列为 `supplier_id`、`purchase_order_id`、`purchase_invoice_id`（跨 schema 逻辑引用 `invoice.purchase_invoices`，按裁定 A-10 该表已归 invoice 模块）、`source_doc_type` 取值 PURCHASE_INVOICE、MIGRATION_OPENING。三条守恒 CHECK 同构。

##### 3.2.5 finance.advance_receipt_entries（预收台账条目）

| 列 | 类型 | 可空 | 说明 |
|---|---|---|---|
| customer_id | uuid | 否 | |
| contract_id | uuid | 是 | |
| sales_order_id | uuid | 是 | |
| receipt_plan_line_id | uuid | 是 | 逻辑引用 clm |
| receipt_id | uuid | 是 | 同 schema 外键指向 `finance.receipts`；期初导入产生的条目无来源资金单据，取空 |
| source_doc_type | text | 否 | `ck_advance_receipt_entries_source_type` 取值 RECEIPT、MIGRATION_OPENING |
| business_date | date | 否 | 等于到款日期 |
| accounting_period_id | uuid | 否 | |
| deferred_from_period_id | uuid | 是 | |
| original_amount | numeric(18,2) | 否 | 挂账金额，大于 0 |
| settled_amount | numeric(18,2) | 否 | |
| open_amount | numeric(18,2) | 否 | |

守恒 CHECK 三条同构，直接支撑 PRD 第 6.11.3 的两条校验。

##### 3.2.6 finance.advance_payment_entries（预付台账条目）

与 3.2.5 对称，差异列为 `supplier_id`、`purchase_order_id`、`payment_plan_line_id`、`payment_id`；`payment_id` 同样可空，`source_doc_type` 的 `ck_advance_payment_entries_source_type` 取值 PAYMENT、MIGRATION_OPENING。

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

PRD 第 6.1.4 要求系统不提供新增资金流水入口，本表在 API 层不暴露任何写端点，只由四个用例经仓储写入，静态检查见第 8.5 节。该约束另有两条既有机制承载，即路由表上不存在任何指向本表的写端点，以及第 3.6 节向 `platform_core.append_only_registry` 登记后挂上的仅追加触发器。

##### 3.2.16 附件关联表

`finance.receipt_attachments`、`finance.payment_attachments`、`finance.refund_attachments`、`finance.cash_document_reversal_attachments`。

#### 3.3 视图

十个勾稽项对应十个对账视图，本阶段全部完整实现。其中八项的子账侧取自本阶段自有表；存货与已收货未收票两项按裁定 B-08 与 G-01 由本阶段注入阶段 8 的 `ep_contract_inventory::StockValueSubledgerBalancePort` 与阶段 7 的 `ep_contract_procure::GrniSubledgerBalancePort` 两个端口实现后接入，实现类型名固定为 `InventorySubledgerBalanceQuery` 与 `GrniSubledgerBalanceQuery`，分别由阶段 8 与阶段 7 在其自身阶段定义端口并实现，本阶段只写注入行。另有四个业务查询视图与三个受治理数据集视图，合计 17 个。

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
| finance.v_recon_inventory | 存货金额账，经 `InventorySubledgerBalanceQuery` | 本阶段完整实现，实现体由阶段 8 在 `ep-contract-inventory` 的 `StockValueSubledgerBalancePort` 上交付，本阶段只注入 |
| finance.v_recon_grni | 已收货未收票暂估，经 `GrniSubledgerBalanceQuery` | 本阶段完整实现，实现体由阶段 7 在 `ep-contract-procure` 的 `GrniSubledgerBalancePort` 上交付，本阶段只注入 |

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

上述 36 张表全部带 `legal_entity_id`，全部按共享基线第 3.8 节的统一模板生成策略，模板由迁移生成器产出，本阶段不写变体。策略名一律 `rls_<table>_le`。

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
| ix_purchase_invoices_legal_entity_id_purchase_order_id | invoice.purchase_invoices | 三单匹配与门户按采购订单反查已登记发票 |
| ix_purchase_invoices_legal_entity_id_posting_date | invoice.purchase_invoices | 进项台账按记账日期与会计期间取数 |
| ix_purchase_invoice_lines_legal_entity_id_goods_receipt_line_id | invoice.purchase_invoice_lines | `ReceiptInvoiceMatchQueryPort::match_state` 与 `match_states` 按收货行判定是否已开票 |

全部索引在迁移中以 `CREATE INDEX CONCURRENTLY` 创建，迁移会话按共享基线第 3.9 节固定 `lock_timeout` 与 `statement_timeout`。

#### 3.6 迁移编号与顺序

执行顺序由单一全局 Runner 按文件版本号全序排定，本阶段 invoice 目录建表文件的版本号一律早于 finance 目录中引用它们的文件。

invoice 目录：

1. V202611030900__invoice_create_tax_rate_options.sql
2. V202611030905__invoice_create_invoice_applications.sql
3. V202611030910__invoice_create_invoice_application_link_tables.sql
4. V202611030915__invoice_create_sales_invoices.sql
5. V202611030920__invoice_create_invoice_reversals.sql
6. V202611030921__invoice_create_purchase_invoices.sql
7. V202611030922__invoice_create_purchase_invoice_lines.sql
8. V202611030925__invoice_create_invoice_receipt_plan_links.sql
9. V202611030930__invoice_create_invoice_import_batches.sql
10. V202611030935__invoice_create_attachment_link_tables.sql
11. V202611030940__invoice_enable_row_level_security.sql
12. V202611030945__invoice_create_indexes.sql
13. V202611030950__invoice_backfill_seed_tax_rate_options.sql
14. V202611030960__invoice_create_dataset_views.sql

第 6 与第 7 两个文件按裁定 A-10 排在 `invoice.invoice_reversals` 之后。第 1 与第 13 两个文件按裁定 C-11 与总览第 1.5 节第五条在 T0 期间执行，第 13 个文件出厂预置六档税率；原第 14 个文件 `V202611030955__invoice_backfill_migrate_tax_rates_from_mdm.sql` 一并撤销，理由是阶段 5 的 `mdm.classification_items` 中不存在 TAX_RATE_PRESET 取值，该迁移无源可迁。第 14 个文件按裁定 A-18 建立 `invoice.v_purchase_invoices_dataset` 并在同一文件内执行 `GRANT SELECT` 给 `ep_analyst_ro`。

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
22. V202611031105__finance_create_dataset_views.sql
23. V202611031110__finance_backfill_append_only_registry.sql
24. V202611031115__finance_backfill_sensitive_field_registry.sql

第 22 个文件按裁定 A-18 建立 `finance.v_receivable_ledger_entries` 与 `finance.v_payable_ledger_entries` 并授予 `ep_analyst_ro`。第 23 个文件按裁定 B-02 向 `platform_core.append_only_registry` 登记本阶段的两张表 `finance.unbilled_ar_entries` 与 `finance.cash_ledger_entries`，两行的 `mode` 取 `APPEND_ONLY`、`mutable_columns` 取 `'{}'`；文件内先插这两行登记，再依次调用 `platform_core.attach_table_guards('finance','unbilled_ar_entries')` 与 `platform_core.attach_table_guards('finance','cash_ledger_entries')`，顺序不得颠倒，挂接函数读登记表取可变列白名单，先挂接后登记取不到 `mutable_columns`；`finance.receivable_entries`、`finance.payable_entries`、`finance.advance_receipt_entries`、`finance.advance_payment_entries`、`finance.overbilling_entries` 五张表带核销金额与状态机，属可更新表，一律不登记。该文件读 finance 写 platform_core，其主要创建对象是 finance 两张仅追加表上的触发器与其登记行，按裁定通则第五条放在 `db/migrations/finance/` 目录下，版本号晚于阶段 2 建立 `platform_core.append_only_registry` 的迁移。登记与触发器的一致性由 `db/checks/append_only_consistency.sql` 经 `xtask sqlcheck` 断言。

第 24 个文件按裁定 A-28 与 C-06 向 `platform_core.sensitive_field_registry` 登记一行，十一列取值为 `schema_name` 取 finance、`table_name` 取 cash_accounts、`column_name` 取逻辑列名 bank_account_no 且不带 `_enc` 后缀、`category` 取 ACCOUNT、`security_level` 取 30、`is_field_encrypted` 取 true、`blind_index` 取 EXACT、`blind_index_column` 取 bank_account_no_bidx、`mask_style` 取 KEEP_LAST_4、`normalization` 取 TRIM_NFKC、`release_ref` 取 `MIGRATION:V202611031115`。该文件同样读 finance 写 platform_core，其主要创建对象是 finance 侧敏感字段的登记行，按裁定通则第五条放在 `db/migrations/finance/` 目录下，版本号晚于阶段 2 建立 `platform_core.sensitive_field_registry` 的迁移。`db/checks/11` 按 `is_field_encrypted` 分支断言，本行取真，因此断言物理表上存在 `bank_account_no_enc` 列且类型为 `bytea` 且不存在同名明文列 `bank_account_no`。

按裁定 A-21，本阶段两个目录一律不建 `backfill_posting_trigger_event_types` 文件，`ledger.posting_trigger_event_types` 的全部登记行由阶段 9a 的种子迁移一次写入且每行只填 `event_type`，本阶段只在 CI 中由 `xtask configdoc` 与 `docs/event-catalog.md` 逐字比对，不进启动自检，也不在关账受理前置校验中复用任何断言，逐条对照见第 5.8 节末表。全部 backfill 与 seed 迁移的 `created_by` 一律取 `ep_foundation::SYSTEM_PRINCIPAL_ID`，即 `00000000-0000-7000-8000-000000000001`，按裁定 A-02，不得自选取值。

每个文件头带 `-- rollback:` 段。建表类文件的回退为对应 `drop table`；seed 与 backfill 文件的回退为按 `code` 删除出厂预置行，或按 `schema_name` 与 `table_name` 删除本次登记的行，即 `append_only_registry` 两行与 `sensitive_field_registry` 一行，第 23 号另 drop `finance.unbilled_ar_entries` 与 `finance.cash_ledger_entries` 两张表上对应的 `assert_append_only` 触发器；`enable_row_level_security` 与 `create_indexes` 的回退为逐条 `drop policy` 与 `drop index`；`create_dataset_views` 的回退为 `drop view` 加 `revoke`。本阶段没有改列类型与收紧非空的迁移，因此全部迁移可在线执行。

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

DRAFT 到 PENDING_REAUTH_APPROVAL 再到 REGISTERED，重新认证与审批一律必经，不设开关；另有 DRAFT 到 CANCELLED 与 REGISTERED 到 REVERSED 两条。

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

输入：条目的 `due_date`、`open_amount`、评估基准日（默认为服务器自然日，按共享基线第 3.4 节取 `(now() AT TIME ZONE 'Asia/Shanghai')::date`）、`AgingBucketSet`。`AgingBucketSet` 的取数在本阶段来自临时表 `finance.aging_bucket_definitions`，阶段 11 交付 `reporting.aging_bucket_profiles` 与 `reporting.aging_bucket_lines` 之后改经 `ep_contract_reporting::AgingBucketQuery::buckets(tx, ctx, legal_entity_id, ledger_side)`，本表随阶段 11 的删表迁移撤销，分档口径不设第二套，见裁定 C-08。

`overdue_days = 评估基准日 - due_date`，小于零时归入第一档。逐档判定 `from_days <= overdue_days` 且（`to_days` 为空或 `overdue_days <= to_days`）。基数一律为 `open_amount`，不使用 `original_amount`，对应 PRD 第 6.9.3。

账龄不依赖 `accounting_period_id`，因此顺延入账不改变账龄，对应规格第 5.2 章子账与凭证共用同一期间归属条款的最后一句。这一点在领域属性测试中作为不变量断言。

#### 4.6 超量开票三条结清路径

三条路径共用同一个余额推进函数 `OverbillingEntry::settle(quantity, amount, path) -> Result<(OverbillingEntry, OverbillingSettlement)>`，守卫为 `quantity <= open_quantity` 且 `amount <= open_amount`。

路径一由收货用例经契约 `OverbillingMatchPort::match_on_receipt` 触发。入参为采购订单、物料、仓库、本次收货数量；返回本次可反向匹配的数量与单价。库存模块按返回的单价与数量同源写数量账与金额账，本阶段按同一金额记 `finance.overbilling_settlements`，凭证由 ledger 端口按规格第 5.2 章超量开票路径一生成，本文不复述分录。匹配数量以该采购订单已挂账的 `open_quantity` 为上限，对应规格第 5.2 章路径一。

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

开具登记一律追加一条 CREDIT 方向、金额为 `net_amount` 的 `finance.unbilled_ar_entries`，不做与交付确认的逐笔匹配。理由是规格第 5.2 章开票事件对该科目的处理不设与交付确认的匹配条件；开票先于交付确认与其反向情形的冲转口径按该章执行，两者的抵消是科目层面的净额而不是条目层面的配对。

红字冲销与作废登记按销项方向追加一条 DEBIT 方向、金额等于原发票 `net_amount` 的条目，对应事件-分录表“恢复其余额”。

因此该科目的子账侧净额恒等于 `sum(DEBIT) - sum(CREDIT)`，正数为已交付未开票、负数为已开票未交付，与规格第 17.3 章的净额双向口径逐字对应，且不设关账归零要求。

#### 4.10 会计期间归属

本阶段不实现期间解析，只调用 `ep-contract-ledger::AccountingPeriodResolver::resolve(legal_entity_id, posting_date, tx) -> ResolvedPeriod { period_id, deferred_from_period_id }`。该方法在同一个 `&mut dyn Tx` 内记忆化，同一事务中第二次调用返回同一取值，因此凭证与全部子账条目共用同一次解析结果是结构性事实而不是一句纪律；共用该结果的子账条目包括台账条目、核销关系行、资金腿明细与超量开票记录。事务句柄的类型按裁定 A-01 固定为 `&mut dyn Tx`，`Tx` 与只读快照上下文 `SnapshotCtx` 均取自 `ep_foundation::port`，由阶段 1 冻结；本阶段全部跨模块契约方法的事务参数一律写成 `&mut dyn Tx`，只读对账取数一律写成 `&dyn SnapshotCtx`，不使用具体连接类型。

记账日期的取值与校验在本阶段：默认取登记时点服务器自然日；允许早于该日并按 PRD 第 6.1.4 提示为补记并写审计；晚于该日一律 `VALIDATION` 并定位字段。

提交响应固定回带 `accounting_period`（编码与名称）与 `is_deferred`，供界面按 PRD 第 6.1.4 显式标注，缺失即视为实现缺陷。
#### 4.11 采购发票登记与三单匹配

本节按裁定 A-10 归本阶段。用例为 `crates/application/invoice/src/usecase/register_purchase_invoice.rs`，端点见第 5.2 节。

步骤：

1. 三单匹配在该用例内执行，依次比对采购订单行、收货行与本次发票行的数量与金额。收货行的已开票状态经本模块自有表判定，不回问采购模块。
2. 暂估回冲与价差拆分经 `ep_contract_inventory::InventoryVariancePort::split_variance(tx, ctx, VarianceSplitCommand{..})` 取得尚有库存部分与已出库部分的金额，写入 `invoice.purchase_invoice_lines.accrual_reversal_amount` 与 `price_variance_amount`。按裁定 C-13，取价一律归阶段 8，本阶段不自行取价，ledger 侧只做分录映射与借贷平衡。
3. 应付腿经本阶段自身的 `register_payable_on_purchase_invoice` 用例写入 `finance.payable_entries`，并按同一合同自动核销预付，见第 0.2 节的附加分录腿假设。
4. 开票数量超过累计收货数量的部分按 `is_overbilling` 标记，并在 `finance.overbilling_entries` 生成挂账，后续走第 4.6 节的三条结清路径。
5. 会计期间在事务最前由 `AccountingPeriodResolver::resolve` 一次解析，凭证与全部子账条目共用该结果。
6. 事件 `invoice.purchase_invoice.registered.v1` 写入 Outbox，payload 见第 5.8 节。
7. 若本次登记的来源是供应商门户上传的发票，即 `portal.supplier_invoice_uploads` 中一条 `status` 为 `UPLOADED` 的记录，则在同一事务内补一步回写：把该上传记录的 `status` 迁到 `ACCEPTED`，并把 `accepted_purchase_invoice_id` 回写为本次生成的 `invoice.purchase_invoices.id`，对应阶段 7 第 3.3.5 节的列定义与第 4.2.7 节的 `UPLOADED → ACCEPTED` 一路。本阶段不直接写 portal schema 的任何表，该回写一律经一个写端口完成，端口与 `ep-contract-procure` 既有的 `PaymentRequestWritebackPort` 同构：写端口由被写方模块的 contract crate 定义 trait、由被写方模块的 app crate 实现，调用方只依赖 contract，本阶段由 `ep-app-invoice` 在 `register_purchase_invoice` 的事务内调用，依赖方向为 app 到 contract，落在基线第 1.3 节允许项内，承接方同为 `xtask archcheck` 的层位判定。端口调用失败即整笔登记回滚，不存在采购发票已落库而上传记录仍为 `UPLOADED` 的中间态。补一个写端口，其确切类型名与所属 crate 由阶段 7 与阶段 10 在落码前同批定，本文不预先命名；本次登记与上传记录的对应关系按该端口的入参形态在同批裁定中一并确定，本文不预设请求体新增字段，第 5.2 节的端点请求列相应不变。来源不是门户上传的采购发票登记不触发本步，端口不被调用。`UPLOADED → RETURNED` 一路由哪个端点承载同属尚未裁定项，与本步写端口的类型名和 crate 归属由阶段 7 与阶段 10 在落码前同批定，本文不代为指定。

进项红字发票由阶段 7 的采购退货用例经 `ep_contract_invoice::PurchaseCreditNotePort::register_credit_note(tx, ctx, cmd: RegisterPurchaseCreditNote)` 触发，在调用方事务内执行，返回 `PurchaseCreditNoteView`；`is_for_overbilling_settlement` 为真时同时结清对应挂账，即第 4.6 节的路径二。收货与发票的匹配状态经 `ep_contract_invoice::ReceiptInvoiceMatchQueryPort` 的 `match_state` 与 `match_states` 两个方法对外提供，返回 `ReceiptInvoiceMatchState`。两个 trait 与四个 DTO 按裁定 A-11 由本阶段在 `ep-contract-invoice` 定义、在 `ep-app-invoice` 实现；按第 0.5 节采购退货的采购发票已登记分支整条推迟到本阶段，阶段 7 不建该调用点也不注入任何替身，两个 wiring 目录中的注入行由本阶段首次写入。

#### 4.12 往来与预收预付的期初余额导入

本节按裁定 A-24 归本阶段。用例为 `crates/application/finance/src/usecase/import_opening_balances.rs`，端点为 `POST /api/v1/finance/opening-balances/actions/import`，请求体为 `{ledger_side, accounting_period_id, rows[]}`。

`rows` 按 `ledger_side` 写入 `finance.receivable_entries`、`finance.payable_entries`、`finance.advance_receipt_entries`、`finance.advance_payment_entries` 四张表之一，`source_doc_type` 一律取 `MIGRATION_OPENING`，`sales_invoice_id`、`purchase_invoice_id`、`receipt_id`、`payment_id` 四列在期初条目上取空。

本通道与资金账户期初、总账期初、库存期初四者一律不生成凭证：期初对应的总账侧由阶段 9a 的期初余额批次承担，两侧的平衡由第 3.3 节的十个 `finance.v_recon_*` 视图在首个会计期间校验。逐行独立事务落库，失败行不回滚已成功行，逐行返回原因，与第 0.4 节 F-16 的批量导入口径一致。

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
| POST /api/v1/invoice/invoice-reversals | `direction`、`reversal_type`、`source_invoice_id`、`register_date`、`posting_date`、`red_invoice_no`、`red_net_amount`、`red_tax_rate`、`red_tax_amount`、`red_gross_amount`、`reason`、`overbilling_entry_id`、`attachment_object_ids[]`；必带 `X-Reauth-Token` | 冲销单视图，含 `voucher_id`、被回滚后的申请单状态与剩余可开比例 | INVOICE.INVOICE_REVERSAL.SOURCE_ALREADY_REVERSED、INVOICE.INVOICE_REVERSAL.TYPE_MUTUALLY_EXCLUSIVE、INVOICE.INVOICE_REVERSAL.RED_AMOUNT_MISMATCH、INVOICE.INVOICE_REVERSAL.SOURCE_INVOICE_NOT_REGISTERED、INVOICE.INVOICE_REVERSAL.RECEIPT_PLAN_ISSUED_AMOUNT_NEGATIVE | invoice.invoice_reversal:create |
| POST /api/v1/invoice/purchase-invoices | `supplier_id`、`purchase_order_id`、`invoice_no`、`invoice_date`、`posting_date`、`tax_rate`、`net_amount`、`tax_amount`、`gross_amount`、`cost_kind`、`is_credit_note`、`lines[]`（含 `purchase_order_line_id`、`goods_receipt_line_id`、`material_id`、`quantity`、`net_unit_price`、`net_amount`、`tax_amount`）、`attachment_object_ids[]` | 采购发票视图，含 `doc_no`、`voucher_id`、`accounting_period`、`is_deferred`、逐行的 `accrual_reversal_amount` 与 `price_variance_amount`、`payable_entry_id` | INVOICE.PURCHASE_INVOICE.INVOICE_NO_DUPLICATED、INVOICE.PURCHASE_INVOICE.RECEIPT_LINE_ALREADY_INVOICED、INVOICE.PURCHASE_INVOICE.QUANTITY_EXCEEDS_RECEIPT、INVOICE.PURCHASE_INVOICE.AMOUNT_MISMATCH_WITH_ORDER、INVOICE.PURCHASE_INVOICE.GROSS_AMOUNT_MISMATCH、INVOICE.PURCHASE_INVOICE.POSTING_DATE_IN_FUTURE | invoice.purchase_invoice_ledger:create |
| GET /api/v1/invoice/purchase-invoices 与 /{id} | 过滤 `supplier_id`、`status`、`invoice_date`、`accounting_period_id`、`purchase_order_id`、`invoice_no` | 分页列表与详情，含发票行、应付明细条目与凭证追溯；取数为本模块自有表 `invoice.purchase_invoices` 与 `invoice.purchase_invoice_lines`，不经 `ep-contract-procure` | | invoice.purchase_invoice_ledger:read |

`POST /api/v1/invoice/sales-invoices` 是规格附录 A.1“应收发票生成”这一度量项的被测端点。
发票打印按裁定 A-08 在阶段 5 交付的三个端口上增量实现，即 `ep_foundation::port::doc::DocTemplatePort::render` 与 `PdfRenderPort::render_pdf`，本阶段只产出像素级套打所需的 `PrintLayout` 取值，不新增任何渲染 trait，也不自建第二条渲染路径。

#### 5.3 资金账户

| 方法与路径 | 说明 |
|---|---|
| POST /api/v1/finance/cash-accounts | 建档，按配置进审批，配置项见第 7 节；`account_type` 与 `ledger_account_id` 的匹配校验，错误码 FINANCE.CASH_ACCOUNT.ACCOUNT_TYPE_LEDGER_MISMATCH |
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
| POST /api/v1/finance/refunds | `refund_type`、`party_id`、`return_doc_type`、`return_doc_id`、`invoice_reversal_id`、`source_payments[]`、`register_date`、`posting_date`、`refund_amount`、`cash_account_id`、`reason`；必带 `X-Reauth-Token` 并进审批 | FINANCE.REFUND.RETURN_DOC_REQUIRED、FINANCE.REFUND.INVOICE_REVERSAL_REQUIRED、FINANCE.REFUND.AMOUNT_EXCEEDS_CAP、FINANCE.REFUND.CASH_ACCOUNT_DEACTIVATED | finance.refund:create |
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
| GET /api/v1/finance/reconciliations | 查询参数 `accounting_period_id` 与可选 `item`；返回十项的子账侧、总账侧与差额三列，十项的子账侧均已接入，存货与已收货未收票两项经 `ep_contract_inventory::StockValueSubledgerBalancePort` 与 `ep_contract_procure::GrniSubledgerBalancePort` 两个模块端口取数，见第 3.3 节；差额非零时附差异事项引用；本端点不提供任何调整入口，对应 PRD 第 6.13.2 |
| GET /api/v1/finance/cash-ledger-entries | 全法人资金腿明细，按账户筛选 |
| POST /api/v1/finance/opening-balances/actions/import | 往来与预收预付期初导入，请求体 `{ledger_side, accounting_period_id, rows[]}`，见第 4.12 节；逐行独立事务，不生成凭证；错误码 FINANCE.OPENING_BALANCE.PERIOD_NOT_FIRST、FINANCE.OPENING_BALANCE.ROW_LIMIT_EXCEEDED、FINANCE.OPENING_BALANCE.PARTY_NOT_FOUND |

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

12 个事件，命名按共享基线第 6.1 节的四段式，登记到 `docs/event-catalog.md` 后才实现。信封字段不增不减，`posting_date` 与 `accounting_period_id` 取本次解析结果，`security_level` 取单据取值，`data_scope_tags` 携带客户或供应商与合同两类标签。

| 事件类型 | 触发点 | payload 要点 | 消费方 |
|---|---|---|---|
| invoice.invoice_application.submitted.v1 | 申请提交进入审批 | 申请单 ID、合同、客户、开票比例、申请金额 | notify、reporting |
| invoice.invoice_application.approved.v1 | 审批链全通过 | 申请单 ID、剩余可开比例 | notify |
| invoice.sales_invoice.issued.v1 | 开具登记提交成功 | 发票 ID、申请单 ID、客户、合同、不含税金额、税额、价税合计、应收条目 ID、凭证 ID | notify、reporting、search、客户 360 |
| invoice.purchase_invoice.registered.v1 | 采购发票登记提交成功 | `purchase_invoice_id`、`doc_no`、`supplier_id`、`purchase_order_id`、`cost_kind`、`net_amount`、`tax_amount`、`gross_amount`、`accrual_reversal_amount`、`price_variance_in_stock_amount`、`price_variance_released_amount`、`voucher_id`、`lines` | notify、reporting、门户投影 |
| invoice.sales_invoice.reversed.v1 | 销项作废或红冲登记成功 | 冲销单 ID、原发票 ID、处理类型、红字金额、回滚后的剩余可开比例 | notify、reporting、search |
| invoice.purchase_invoice.reversed.v1 | 进项作废或红冲登记成功 | 冲销单 ID、原采购发票 ID、处理类型、红字金额、是否用于超量开票结清 | notify、reporting |
| invoice.invoice_import_batch.completed.v1 | 批量导入任务结束 | 批次 ID、总行数、成功行数、失败行数、结果对象引用 | notify |
| finance.receipt.registered.v1 | 到款登记成功 | 到款单 ID、客户、到款金额、核销合计、转预收金额、资金账户、凭证 ID | notify、reporting、客户 360 |
| finance.payment.registered.v1 | 付款登记成功 | 付款单 ID、供应商、付款金额、核销合计、转预付金额、付款申请单 ID、凭证 ID | notify、reporting、门户投影 |
| finance.refund.registered.v1 | 退款或返款登记成功 | 退款单 ID、类型、往来方、金额、关联退货单、关联冲销单、凭证 ID | notify、reporting |
| finance.cash_document.reversed.v1 | 资金单据冲正登记成功 | 冲正单 ID、原单据类型与 ID、冲正金额、凭证 ID | notify、reporting |
| finance.overbilling_entry.settled.v1 | 三条结清路径任一条完成 | 挂账 ID、路径、结清数量与金额、剩余余额、凭证 ID | notify、reporting |

上述事件的消费方均不做过账，见第 0.1 节。事件的 `aggregate_type` 按共享基线第 6.1 节取 `<schema>.<表名>`，采购发票登记事件取 `invoice.purchase_invoices`。

按裁定 A-21，登记表、登记接口与全部登记行均归阶段 9a：`ledger.posting_trigger_event_types` 的 13 行由阶段 9a 的种子迁移一次写入，每行只填 `event_type`，原有的 `ledger_event_kind` 与 `registered_by_module` 两列已删，本阶段不得再引用，也不新增任何回填迁移。按总览第 1.5 节第三条，`PostingTriggerRegistry::assert_registered` 与错误码 `LEDGER.POSTING_TRIGGER_EVENT_TYPE.REGISTRY_MISMATCH` 整项删除，本阶段在启动自检、`--check` 与关账受理三处都不调用该方法，理由是规格第 10.2 章逐字枚举关账受理只有两项前提，在计划层新增第三项受理前提是计划凌驾规格。登记表一致性的承接方定死为两条，即 `xtask configdoc` 从 `docs/event-catalog.md` 生成阶段 9a 的第 14 号种子迁移并在 CI 中逐字比对，以及阶段 3b 的 `event-catalog-consistent` 自检项且不通过时停止派发未登记事件类型；本阶段下表八个事件的一致性即由这两条承接，运行期不再有退出码 78 这条路径，关账受理前提仍为规格第 10.2 章的两条。

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

#### 5.9 对外契约 trait

15 个 trait，定义在两个 contract crate 中，实现注册在 `apps/core-server/src/wiring/` 与 `apps/job-worker/src/wiring/` 两个目录。调用方按共享基线第 1.3 节只依赖 contract，不依赖 application。全部方法的事务句柄一律为 `&mut dyn Tx`，只读对账取数为 `&dyn SnapshotCtx`，两个类型取自 `ep_foundation::port`，按裁定 A-01 由阶段 1 冻结。

| trait | 所在 crate | 调用方 | 语义 |
|---|---|---|---|
| PayableRegistrationPort | ep-contract-finance | ep-app-invoice | 采购发票登记的应付明细条目写入、预付自动核销、超量开票挂账，在调用方事务内执行；按裁定 A-10 采购发票登记归本阶段，调用方由 ep-app-procure 收窄为 ep-app-invoice |
| OverbillingMatchPort | ep-contract-finance | ep-app-procure 的收货用例 | 规格第 5.2 章超量开票路径一的反向匹配，返回可匹配数量与单价；接线次序见第 0.5 节，本阶段交付本端口时一并完成阶段 7 收货用例的接线，交付之前 `finance.overbilling_entries` 恒为空，阶段 7 不接本端口即为正确行为，也不注入任何替身 |
| UnbilledArPort | ep-contract-finance | ep-app-sales | 交付确认与销售退货在应收账款未开票过渡科目上的子账腿写入；交付确认腿的方法按裁定 A-09 固定为 `record_on_delivery(tx, ctx, DeliveryUnbilledArCommand { delivery_confirmation_id, customer_id, posting_date, accounting_period_id, direction: DEBIT, net_amount })`，写 `finance.unbilled_ar_entries`；接线次序见第 0.5 节，与阶段 6 第三批同批接线同批验收，阶段 6 不注入任何替身，使用方由 ep-app-sales 与 ep-app-inventory 收窄为 ep-app-sales |
| ReceivableExposureQuery | ep-contract-finance | ep-app-sales | 返回 `ReceivableExposureView { receivable_open_amount, delivered_unbilled_amount }` 两项，供 `ep_contract_sales::CreditExposureQueryPort` 组装对外唯一的信用敞口入口；按裁定 C-14，`CreditExposureQuery` 与 `CustomerCreditExposurePort` 两个旧名作废；接线次序见第 0.5 节，与阶段 6 第三批同批接线同批验收，阶段 6 不注入任何替身 |
| ReceivableLedgerQuery | ep-contract-finance | ep-app-reporting、ep-app-crm | 应收台账与核销关系只读查询 |
| PayableLedgerQuery | ep-contract-finance | ep-app-procure、ep-app-reporting | 应付台账与核销关系只读查询，方法按裁定 C-15 固定为 `open_balance(tx, ctx, purchase_invoice_id: Id<PurchaseInvoice>) -> Result<Money, AppError>`；阶段 7 的 `PayableQueryPort` 作废 |
| SupplierStatementQuery | ep-contract-finance | ep-app-portal | 供应商收付款对账查询的取数，方法按裁定 C-15 固定为 `statement(tx, ctx, supplier_id: Id<Supplier>, period: PeriodRange) -> Result<SupplierStatementView, AppError>`，返回未脱敏结构，脱敏在门户侧完成；阶段 7 的 `PayableStatementQueryPort` 作废 |
| CashAccountQuery | ep-contract-finance | ep-app-procure、ep-app-reporting | 资金账户与资金腿明细只读查询 |
| AgingQuery | ep-contract-finance | ep-app-reporting | 应收账龄与应付账龄两张基础表的取数；分档定义不由本 trait 承载，取用入口见第 4.5 节 |
| ReconciliationItemQuery | ep-contract-finance | ep-app-ledger 中 9b 段实现的子账与总账勾稽 `ReconCheck` | 按法人与会计期间返回十项勾稽的子账侧合计，结构为 `ReconciliationItemView`；按裁定 B-08 与 G-01 该 `ReconCheck` 由 ep-platform-recon 的执行器驱动，执行器不直接依赖本 crate |
| SalesInvoiceQuery | ep-contract-invoice | ep-app-clm、ep-app-sales、ep-app-reporting | 销项发票与收款计划勾稽的只读查询，方法按裁定 C-16 固定为 `by_sales_order_line(tx, ctx, sales_order_line_id) -> Result<Vec<SalesInvoiceRef>, AppError>` |
| InvoiceReversalStatusQuery | ep-contract-invoice | ep-app-sales、ep-app-procure | 方法按裁定 C-16 固定为 `is_fully_credit_noted(tx, ctx, sales_order_line_id, quantity: Quantity) -> Result<CreditNoteStatus, AppError>`，供销售退货与采购退货的前置校验，对应 PRD 第 6.5.4；阶段 6 的 `InvoiceStatusPort` 作废；接线次序见第 0.5 节，与阶段 6 第三批同批接线同批验收，阶段 6 不注入任何替身 |
| ReceiptInvoiceMatchQueryPort | ep-contract-invoice | ep-app-procure | 按裁定 A-11 提供 `match_state` 与 `match_states` 两个方法，返回 `ReceiptInvoiceMatchState`；其承载的采购退货已登记分支按第 0.5 节整条推迟到本阶段，阶段 7 不建该调用点也不注入任何替身，本阶段首次接线 |
| PurchaseCreditNotePort | ep-contract-invoice | ep-app-procure | 按裁定 A-11 提供 `register_credit_note(tx, ctx, cmd: RegisterPurchaseCreditNote) -> Result<PurchaseCreditNoteView, AppError>`，采购退货在采购发票已登记分支下由本端口登记红字进项发票；该分支按第 0.5 节整条推迟到本阶段，阶段 7 不建该调用点也不注入任何替身，本阶段首次接线 |
| TaxRateOptionQuery | ep-contract-invoice | ep-app-sales、ep-app-clm、ep-app-procure | 按裁定 C-11 提供 `default_rate(tx, ctx, legal_entity_id, item_id: uuid::Uuid) -> Result<Rate, AppError>` 与 `list(tx, ctx, legal_entity_id) -> Result<Vec<TaxRateOption>, AppError>`，是税率字典的唯一取用入口 |
十项勾稽中的存货与已收货未收票两项，其子账侧不由本节的 trait 承载，取数经 `ep_contract_inventory::StockValueSubledgerBalancePort` 与 `ep_contract_procure::GrniSubledgerBalancePort` 两个外部端口，两个端口分别由阶段 8 与阶段 7 在其自身阶段定义并实现，由 `ReconciliationItemQuery` 的实现在组装时调用，注入行由本阶段写入 `apps/core-server/src/wiring/` 与 `apps/job-worker/src/wiring/` 两个目录，见裁定 G-01。

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
| register_purchase_invoice | 采购发票头表与行表；三单匹配结果；经 `InventoryVariancePort::split_variance` 得到的暂估回冲与价差金额回写行表；`register_payable_on_purchase_invoice` 的应付条目与预付自动核销；超量开票挂账；来源为门户上传时经门户侧写端口把 `portal.supplier_invoice_uploads` 迁到 `ACCEPTED` 并回写 `accepted_purchase_invoice_id`，见第 4.11 节第 7 步；凭证；审计；Outbox | 附件正文读写；站内通知 |
| register_payable_on_purchase_invoice | 应付条目；预付自动核销；超量开票挂账。本用例由本阶段的 register_purchase_invoice 用例在其事务内调用，不自开事务 | |
| register_purchase_credit_note | 采购发票红字行；原采购发票 `reversed_by_id` 回写；应付条目冲回；超量开票路径二结清。本用例由阶段 7 的采购退货用例经 `PurchaseCreditNotePort` 在其事务内调用，不自开事务 | |
| record_unbilled_ar_on_delivery | `finance.unbilled_ar_entries` DEBIT 行。由阶段 6 的 confirm_delivery 用例经 `UnbilledArPort::record_on_delivery` 在其事务内调用，不自开事务 | |
| import_opening_balances | 四张台账表之一的期初条目，`source_doc_type` 取 MIGRATION_OPENING。逐行独立事务，不生成凭证，不写 Outbox | 通知 |
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

本阶段发布的 12 个事件写入 `platform_msg.outbox_events`，信封的 `posting_date` 与 `accounting_period_id` 取本次解析结果。消费者为 notify、reporting 投影与 search 索引，均不做过账，见第 0.1 节。

消费端幂等由 `platform_msg.inbox_consumptions` 的唯一约束保证。重投退避按共享基线第 6.2 节。

#### 6.4 失败重试与补偿

- 序列化失败 40001 与死锁 40P01 由数据访问层重试 3 次，退避 50、150、450 毫秒，只对尚未产生外部可见副作用的事务重试。本阶段全部用例在提交前不产生外部副作用，因此全部可重试。
- 守恒 CHECK 违例不重试，直接映射为 `BUSINESS_CONFLICT` 并按规格第 15.2 章写入死信，`incident_no` 回带给界面。这一路径覆盖 PRD 第 6.7.7、6.8.8、6.11.3、6.12.7 的负数未核销余额行。
- ledger 端口返回借贷不平时不重试，整事务回滚并写死信，对应 PRD 第 6.4.8 最后一行。
- 批量导入行级失败不回滚已成功行，失败行写入结果对象并计入 `failed_rows`，批次状态置 PARTIALLY_FAILED。
- 本阶段不使用补偿事务，理由是全部跨模块写入都在同一数据库事务内经契约端口完成，不存在需要 Saga 的跨事务步骤。唯一的例外是 clm 收款计划回写，若 clm 只提供事件驱动接口，该腿改为 Outbox 串接并由死信兜底，届时需新增一条补偿用例，见第 11 节。

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
21. 采购发票三单匹配：数量与订单相等、少于订单、超过累计收货、无采购订单的自由发票四个分支；`is_overbilling` 的置位与不置位；`cost_kind` 两种取值对暂估回冲的影响。
22. 采购发票红字登记：全额红冲、用于超量开票结清、原发票已被红冲的拒绝三个分支。

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
17. 对账视图十项：正常态差额为零；逐项注入差额后差额非零并生成差异事项引用；差额清零后恢复。存货与已收货未收票两项经 `ep_contract_inventory::StockValueSubledgerBalancePort` 与 `ep_contract_procure::GrniSubledgerBalancePort` 两个模块端口取数，其注入方式为改变阶段 8 与阶段 7 的子账侧样例数据。其余八项的注入方式为直接对台账条目做受控 UPDATE，仅在测试库上执行。
18. 批量导入：2000 行成功、含 3 行失败的部分失败、重跑同批次不产生重复发票。
19. 幂等：全部 10 个写端点各一次重放，返回首次结果并带 `Idempotent-Replay: true`；载荷不同时返回 409。
20. 法人越权矩阵：并入独立测试目标 `tests/rls_matrix`，覆盖读取、写入、更新、删除、聚合、排序、报表投影与错误信息泄漏八类，本阶段追加 36 张表与 17 个视图的条目，另覆盖资金账户银行账号字段级权限的两种上下文；八类断言函数名与 32 组矩阵按裁定 C-05 由阶段 1、阶段 2 与阶段 4 分段提供，本阶段只追加条目，不重复实现同名函数。
21. 高风险操作：开票、付款、超量开票路径三三项验证重新认证缺失时拒绝、审批未完成时拒绝、申请人自审时拒绝。
22. 并发：第 6.5 节列出的六组各一个用例，用两个连接交叉提交。
23. 采购发票登记全链路：采购订单、收货、发票三单匹配通过，应付条目、暂估回冲、价差拆分、超量开票挂账与凭证五处逐项核对；采购退货经 `PurchaseCreditNotePort` 登记红字进项发票并冲回应付。
24. 期初余额导入：应收、应付、预收、预付四个方向各一批，`source_doc_type` 为 MIGRATION_OPENING，导入后首个会计期间的十个 `finance.v_recon_*` 视图差额为零；失败行不回滚已成功行。

#### 8.4 端到端测试

后端 E2E 用 Rust 集成测试直接打 HTTP 接口，四端 UI 用 Playwright 驱动桌面 WebView。

- E2E-10-01：闭环第 6 步到第 11 步的连贯路径，从发票申请提交到退款登记，全程在应用内完成，中途不出现外部补齐环节。
- E2E-10-02：直运订单分支下的退款与返款走同一张单据、同一套字段与同一套勾稽校验，对应 PRD 第 6.12.6。
- E2E-10-03：移动端按规格第 6.2 章矩阵为仅查看，验证移动端可查得本阶段全部单据与台账、且提交入口不可达。
- E2E-10-04：供应商门户的收付款对账查询取数与内部应付台账同源，脱敏后返回，对应 PRD 第 4.9.6。
- E2E-10-05：关账受理后提交到款并观察界面显式标注顺延期间，对应 PRD 第 6.1.4。
- E2E-10-06：采购订单到收货到采购发票登记再到付款的连贯路径，三单匹配、暂估回冲、价差拆分与应付核销全程在应用内完成，对应裁定 A-10。

按裁定 A-23，本阶段自交本模块四端界面，测试计划相应追加：invoice 与 finance 两个模块在规格第 6.2 章能力矩阵中取值为完整或简化的能力域，其桌面端用例用 Playwright 与 tauri-driver 驱动，移动端用例用 XCUITest 与 Espresso 驱动；取值为 VIEW_ONLY 的能力域只测只读视图；取值为 NOT_APPLICABLE 的不建入口。E2E-10-03 的移动端仅查看断言并入该组用例。

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

1. 36 张表与 17 个视图在空库上从零迁移成功，再按 `-- rollback:` 段全部回退成功，两次执行的迁移历史表状态一致。
2. `--check` 模式在两个法人上通过，含全部带法人列的表已 `ENABLE` 且 `FORCE` 行级安全的自检项。
3. 58 个端点全部有 OpenAPI 描述且描述与实现的字段名逐项一致，由契约测试断言。
4. 第 8.1 节列出的 22 组单元测试分支全部通过。
5. 第 8.2 节列出的 8 组领域属性测试在 1000 次随机用例下全部通过。
6. 第 8.3 节列出的 24 组集成测试全部通过。
7. 规格第 17.2 章十五类必测分支中的第二、五、六、八、九、十三类在本阶段的用例中通过，第十三类的三条结清路径逐条通过。
8. 十个勾稽项全部在基准数据集上差额为零；逐项注入差额后对账视图差额非零、可下钻、可追溯，清零后恢复为零，其中待处理超量开票一项以关账前余额非零的方式注入，对应规格第 10.2 章的发布验收口径；存货与已收货未收票两项经 `ep_contract_inventory::StockValueSubledgerBalancePort` 与 `ep_contract_procure::GrniSubledgerBalancePort` 两个端口实现接入并同样通过该判定。
9. 规格第 10.2 章的顺延入账注入用例在本阶段的到款登记上通过：凭证与全部子账条目落入同一个顺延后的期间，两条检索路径均可查得。
10. 法人越权测试集 `tests/rls_matrix` 追加本阶段条目后全部通过，八类判据无一泄漏。
11. 三项高风险操作的重新认证与审批控制通过身份与访问控制测试，认证方式、待签内容摘要、时间与设备可在审计证据中查得。
12. PRD 第 6.14.4 列出的 11 类动作全部写入审计，同一事实不只落日志，由审计探针逐项断言。
13. 第 8.6 节十项性能度量在基准数据集上达标，且十项对应查询的执行计划无顺序扫描。
14. 覆盖率达到第 8.7 节的分档门槛，工作区整体不低于 80%。
15. `docs/error-codes.md` 的新增错误码与 `ep-foundation::error::codes` 常量表一致，CI 校验通过，无重复码；`docs/event-catalog.md` 的 12 个新增事件与实现一致；`docs/data-dictionary.md` 的单据类型码一节含本阶段九码且 `xtask configdoc --check-doc-type-codes` 通过。
16. 第 11.3 节列出的共享基线四处回写完成：第 5.4 节幂等 `request_hash` 排除 `X-Reauth-Token`、第 9.2 节新增三个指标、第 11 节新增资金账户期初余额与资金单据冲正两项决定、第 3.5 节确认本阶段未引入新的精度语义。
17. E2E-10-01 至 E2E-10-06 六个用例通过，其中 E2E-10-01 全程在应用内完成。
18. 严重与高危缺陷为零，中危缺陷登记并给出规避方案与责任人，按规格第 17.2 章发布缺陷门禁的口径。
19. invoice 与 finance 两个模块在规格第 6.2 章能力矩阵中取值为完整或简化的能力域，其四端界面已实现并通过 Playwright 与 tauri-driver 的桌面用例、XCUITest 与 Espresso 的移动用例；取值为 VIEW_ONLY 的能力域只实现只读视图；取值为 NOT_APPLICABLE 的不实现入口。
20. 本模块数据集视图 `invoice.v_purchase_invoices_dataset`、`finance.v_receivable_ledger_entries`、`finance.v_payable_ledger_entries` 已发布并授予 `ep_analyst_ro`，列签名已同步给阶段 11。本条在本阶段的被测输入只有本阶段自己的迁移产物与列签名快照，视图已建、授权已授、签名已同步三项均可在本阶段静态判定。阶段 11 的 `reporting-dataset-signature-matched` 在三者上的校验，其被测输入由阶段 11 交付，按基线第 12 节通则第六条取整条推迟一档：本阶段不注册该自检项、不调用它，本条也不断言其结论；三者按第 3.3 节降级口径的通过判定由阶段 11 第 9 节退出条件第 25 条承接，本节不得以任何形态留下以该自检项为被测输入的断言。
21. 本阶段全部路由的能力域码与动作类别常量已声明在 `crates/contract/invoice/src/capability.rs` 与 `crates/contract/finance/src/capability.rs`，`xtask configdoc` 通过。
22. 本模块的 `InvoiceReferenceCounter`、`FinanceReferenceCounter`、`InvoiceSalesTradeHistoryProvider`、`InvoicePurchaseTradeHistoryProvider` 已实现并注册到阶段 5 提供的两个注册表。
23. `finance.cash_accounts` 的银行账号查重经 `derive_blind_key` 与 `BlindIndex` 实现，唯一约束名为 `ux_cash_accounts_legal_entity_id_bank_account_no_bidx`，全库无第二套账号哈希实现。
24. `platform_core.append_only_registry` 已登记本阶段两张表 `finance.unbilled_ar_entries` 与 `finance.cash_ledger_entries`，两行的 `mode` 取 `APPEND_ONLY`、`mutable_columns` 取 `'{}'`，五张可更新台账表未登记，`db/checks/append_only_consistency.sql` 经 `xtask sqlcheck` 通过。
25. 本阶段八个事件在 `ledger.posting_trigger_event_types` 中的登记行由阶段 9a 的种子迁移写入且每行只填 `event_type`，本阶段不含任何 `backfill_posting_trigger_event_types` 迁移；本模块八个事件与 `docs/event-catalog.md` 逐字比对通过，该比对由 CI 的 `xtask configdoc` 承担，本项不点名任何运行期断言方法，进程启动路径与关账受理路径上都不存在与本项相关的判定，也不存在退出码 78。
26. 八个反向依赖点按第 0.5 节的三档处置逐条落地并端到端通过：`UnbilledArPort`、`ReceivableExposureQuery` 与 `InvoiceReversalStatusQuery` 三者与阶段 6 第三批同批接线、同批验收，`OverbillingMatchPort` 在交付时一并接入阶段 7 的收货用例，`ReceiptInvoiceMatchQueryPort`、`PurchaseCreditNotePort`、`SupplierStatementQuery` 与 `PayableLedgerQuery` 承接四条整条推迟的分支，其中 `PayableLedgerQuery` 一条含阶段 7 第 4.5 节的付款申请 `INVOICE_PAYMENT` 分支与其占用写入路径，本阶段按原形态实现并端到端通过，同一张采购发票被两张付款申请并发引用时可串行化；两个 wiring 目录下的全部文件中不出现任何占位实现类型，本阶段不开也不关任何降级窗口，`SALES.SALES_RETURN.INVOICE_NOT_CREDIT_NOTED` 判定与交付确认三腿的过渡科目净额断言在本阶段一次真实通过。
27. `invoice.tax_rate_options` 的建表迁移 `V202611030900__invoice_create_tax_rate_options.sql` 与种子迁移 `V202611030950__invoice_backfill_seed_tax_rate_options.sql` 已在 T0 期间执行且六档出厂预置可查，全库取默认税率只有 `ep_contract_invoice::TaxRateOptionQuery::default_rate` 一条路径，任何阶段不提供税率桩，本阶段不含任何从 mdm 迁移税率的回填迁移。
28. 期初导入通道在四个方向上各通过一次，导入后首个会计期间的十个 `finance.v_recon_*` 视图差额为零，且该通道不产生任何凭证。
29. 本阶段按裁定 A-06 不实现也不注册任何 `ReconCheck`，原定的 `FIN_CROSS_MODULE_LINK` 整条删除。判定方式为本阶段新增与修改的代码内不出现 `ep_platform_recon::ReconCheck` 的实现体与 `ReconRegistry::register` 的调用。本阶段对内部对账的贡献只有取数一侧，即第 5.5 节的十个对账视图与 `ReconciliationItemQuery`，其比较由阶段 9b 的子账与总账勾稽 `ReconCheck` 驱动，该项的验收见本节第 8 条，本条不重复判定。
30. `platform_core.sensitive_field_registry` 中存在 `finance.cash_accounts.bank_account_no` 一行，`is_field_encrypted` 为真，`security_level` 为 30，`blind_index_column` 为 `bank_account_no_bidx`；`db/checks/11` 返回零行，即物理表上存在 `bank_account_no_enc bytea` 且不存在同名明文列 `bank_account_no`，见裁定 A-28。
31. 第 0.2 节登记的规格回写项已完成，即规格第 5.2 章事件-分录表的开票事件与采购发票事件两行的分支与附加规则列已补入自动核销预收账款与预付账款的分录腿，本阶段的附加腿按回写后的分录执行；回写未完成即为本阶段的阻塞前置，不得以假设取值放行。
32. 第 0.0 节列出的五项 T0 最小切片在 T0 判定时已经跑通并保持可回归，即最小销项发票、最小应收条目、一笔到款与一次全额核销、一个资金账户建档、税率字典建表与种子及 `TaxRateOptionQuery` 五项在 `ep-datagen` 最小样本上通过且应收一项勾稽差额为零，销项发票的 `tax_rate` 取自 `TaxRateOptionQuery::default_rate`；本阶段的全部其余交付物在该骨架上加厚，`testkit/scenarios/stage10_ar_ap_closed_loop.rs` 复用 T0 的开票与到款两段步骤函数，全卷不存在第二条首次贯通路径。
33. 门户发票上传记录的 `UPLOADED → ACCEPTED` 一路在本阶段一次真实通过：以一条阶段 7 交付的 `UPLOADED` 上传记录为输入执行 `register_purchase_invoice`，登记成功后该记录 `status` 为 `ACCEPTED` 且 `accepted_purchase_invoice_id` 等于本次采购发票的 `id`；回写与采购发票落库在同一事务内，注入登记失败后该记录仍为 `UPLOADED` 且无孤立的采购发票行。该回写经第 4.11 节第 7 步的写端口完成，本阶段代码内不出现对 portal schema 任何表的直接写入，判定方式为本阶段新增与修改的代码内不出现 `portal.` 前缀表名的写语句。阶段 7 推迟到本阶段的 E2E-T-03 受理路径随本条一并判定。
34. 规格第 21.4 章要求的专业签字已取得并留档：会计与税务在本阶段签字，签字人资格证据随版本留档；签字缺失或不通过时本阶段不得退出，整改后重新测试并重新签字，不得以未记录的方式豁免（规格第 22 章第 12 条）。本条由裁定 F-42 新增，此前四份计划的退出条件中无任何签字项。

---

### 10. 与规格和 PRD 的对应

#### 10.1 规格条目

| 规格章节 | 本阶段实现的内容 |
|---|---|
| 5.2 财务条目 | 应收台账、应付台账、发票申请与审批与开具登记、与合同收付款计划及订单勾稽、按发票核销到款、分次到款与分次付款、一笔款项核销多张发票、客户退款与供应商返款、预收台账与预付台账、银行与现金账户档案、三类事件的资金腿明细视图、资金流水不得独立登记 |
| 5.2 数电发票与税务条目 | 销项发票开具登记的八个字段、回写申请单状态与剩余可开比例、销项与进项两个方向的作废与红字冲销登记、回滚后按剩余可开比例重新开具、单一税种增值税、按实际开票结果登记的语义、人工录入与批量导入两条路径 |
| 5.2 财务规则条目事件-分录表 | 开票、采购发票登记、到款、付款、退款、红字冲销与作废六类事件的调用与落库，其中采购发票登记的单据本体按裁定 A-10 也在本阶段；采购发票事件的应付腿与超量开票腿；交付确认与销售退货两类事件在过渡科目上的子账腿，交付确认腿经 `UnbilledArPort::record_on_delivery` 由阶段 6 调用 |
| 5.2 到款与付款的核销顺序规则块 | 默认按单据到期日升序、同日按单据编号升序；人工指定写入审计 |
| 5.2 超量开票的三条结清路径规则块 | 挂账登记与三条路径的完整实现，含路径三之后到货的先冲回再入账顺序 |
| 5.2 总账功能与期末处理块 | 记账日期的取值与校验；凭证与子账共用同一会计期间字段；顺延只改变期间归属不改变取价；两个日期与两条检索路径 |
| 7.7 法人行级隔离机制 | 36 张表的统一 RLS 策略、无 `BYPASSRLS`、跨法人查询按法人逐个设置变量 |
| 7.8 密钥域 | 银行账号按法人密钥域字段级加密存储，物理列为 `bank_account_no_enc` 加 `bank_account_no_key_ref`，不保留同名明文列；该项属规格强制，不在 U-A-12 的待决范围内 |
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
| 6.16 | 21 条待决项的临时取值与承载方式，见第 0.4 节 |
| 4.7.3 与 4.7.4 | 付款登记完成后回写付款申请的已付金额与状态，经 `ep-contract-procure` 写端口 |
| 4.9.6 | 供应商门户收付款对账查询的取数来源 |
| 8.3.4 | 应收账龄与应付账龄两张基础表的数据来源 |
| 11.3 | 同步等待上限 8 秒，超过转后台任务，本阶段只有批量导入与账龄大范围导出触及该线 |

#### 10.3 本阶段明确不做的事

按 PRD 第 6.15 节全部八条，不扩大也不收窄。另外，本阶段不实现凭证生成本身、不实现科目表与期间管理、不实现库存金额账与任何取价、不实现收货单据本身、不实现销售退货与采购退货单据、不实现合同收付款计划，这六项分别属其他阶段。采购发票登记与三单匹配按裁定 A-10 已归本阶段，不再列入不做清单；三单匹配所需的收货行数据经采购模块的只读查询取得，价差与暂估回冲的金额经 `InventoryVariancePort::split_variance` 取得，本阶段不自行取价。

---

### 11. 风险与预留

#### 11.1 技术风险

| 风险 | 影响 | 应对 |
|---|---|---|
| 本阶段是八个反向依赖点的提供方，其中三项与阶段 6 第三批同批接线、一项在交付时接入阶段 7 的收货用例、四项承接整条推迟的分支 | 本阶段延期会连带推迟阶段 6 的交付确认过渡科目腿、信用敞口两桶与销售退货已开票分支，以及阶段 7 的采购退货已开票分支、供应商门户对账与付款申请的 `INVOICE_PAYMENT` 分支 | 按第 0.5 节的三档处置排期，三项同批接线与阶段 6 第三批一并列入联调冒烟；整条推迟的四项在其调用方阶段以硬阻断示明，不留任何返回固定分支的实现，也不开任何降级窗口；第 9 节退出条件第 26 条对八项逐条判定 |
| 第 0.2 节登记的规格回写项在本阶段开工时可能尚未回写 | 预收预付两项勾稽的分录腿没有权威口径，实现不能放行 | 分录腿以 `AdditionalLeg` 参数传给 ledger 端口，机制先行、内容后填，不写死在本阶段；测试断言写在勾稽层而不是分录层；回写未完成时按第 9 节退出条件第 31 条判为阻塞 |
| clm 收款计划已开票金额的回写方式未定 | 若 clm 只提供事件驱动接口，开票用例的事务边界被打破，需要补偿路径 | 在契约层同时定义同步写端口与事件两种形态，装配时择一；若走事件，追加一条补偿用例与一项死信监控 |
| F-08 只允许全额红冲的临时取值 | 若财务负责人要求部分红冲，销项发票状态机需拆出部分红冲态，比例回滚公式需改，已落库数据需回填 | 已在 `invoice.sales_invoices` 上预留 `reversed_net_amount` 列，全额红冲时等于 `net_amount`，改为部分红冲时不需要加列 |
| F-16 逐行落库的临时取值 | 若改为整体回滚，逐行幂等键设计作废 | 导入器把逐行处理与批次编排分离，切换只改编排层 |
| 账龄查询在 6 万条应收条目与 12 个期间跨度下可能触及 10 秒线 | 附录 A.1 的两项报表度量不达标 | 账龄计算下推到数据库聚合而不是应用侧循环；分组键固定为四个；单次查询期间跨度由 `EP__FINANCE__RECON__MAX_PERIODS_PER_QUERY` 限制；不达标时按规格第 16 章执行性能整改，不放宽通过线 |
| 守恒 CHECK 违例在高并发下成为主要失败源 | 用户看到较多 `BUSINESS_CONFLICT` | 先读后锁的复核路径在冲突时回带最新余额，界面可一键重取；冲突次数进指标 `ep_finance_settlement_conflicts_total`，超阈值时调整核销候选集的预取策略 |
| 银行账号字段级加密与查重的组合 | 盲索引密钥若按法人固定，存在字典攻击面 | 按裁定 B-04 直接使用阶段 2 提供的 `derive_blind_key` 与 `BlindIndex`，密钥自法人数据加密密钥域派生且不落库；盲索引只用于唯一约束，不用于检索；本阶段不实现任何自有哈希 |

#### 11.2 为后续阶段预留的扩展点

1. `ep-contract-finance` 的 `ReceivableExposureQuery` 端口返回应收未收金额与已交付未开票金额两项，销售阶段在 `ep_contract_sales::CreditExposureQueryPort` 内消费该结果并补上在途订单金额，对外唯一入口为销售侧端口，对应规格第 5.2 章客户信用额度校验条目的三部分构成，见裁定 C-14。
2. `finance.v_recon_inventory` 与 `finance.v_recon_grni` 两个视图在本阶段已完整接入，子账侧由本阶段注入阶段 8 的 `InventorySubledgerBalanceQuery`（`ep_contract_inventory::StockValueSubledgerBalancePort` 的实现）与阶段 7 的 `GrniSubledgerBalanceQuery`（`ep_contract_procure::GrniSubledgerBalancePort` 的实现）两个端口实现；后续版本新增子账来源时，在该来源模块的 `ep-contract-<m>` 增加一个同形端口、由该来源模块定义并实现、由本模块在组装处接入并在两个 wiring 目录注册，不改视图结构，见裁定 B-08 与 G-01。
3. `finance.receivable_entries.source_doc_type` 与 `finance.payable_entries.source_doc_type` 是可扩展枚举，为后续版本的其他应收应付来源留位，首版只有一个取值。
4. `invoice.sales_invoices` 不设明细行表，但 `invoice.invoice_receipt_plan_links` 已按多行结构建立，将来支持一张发票多行明细时只需新增 `invoice.sales_invoice_lines` 并把税额校验从头表下移到行表。
5. `finance.cash_ledger_entries.source_doc_type` 为可扩展枚举，后续版本引入银企直连流水时新增取值即可，但资金流水不得独立登记的约束在首版必须保持。
6. 对账视图的十项统一为同一 DTO 结构 `ReconciliationItemView { item_code, legal_entity_id, accounting_period_id, subsidiary_amount, ledger_amount, difference }`，内部对账组件与关账前强制校验直接消费该结构，不需要为每项写一套取数。
7. 报表阶段的应收账龄与应付账龄两张基础表直接消费 `finance.v_receivable_aging` 与 `finance.v_payable_aging`，不另建一套口径；分档定义按裁定 C-08 在阶段 11 迁到 `reporting.aging_bucket_profiles` 与 `reporting.aging_bucket_lines`，届时两个视图的分档取数改经 `ep_contract_reporting::AgingBucketQuery`。
8. 门户阶段的供应商对账查询消费 `ep-contract-finance::SupplierStatementQuery`，脱敏投影在门户侧完成，本阶段不返回任何脱敏后的数据结构，避免两套口径。

#### 11.3 需回写共享基线的项

1. 第 5.4 节：`request_hash` 的计算排除 `X-Reauth-Token`。
2. 第 9.2 节：新增三个指标 `ep_finance_settlement_conflicts_total`、`ep_finance_reconciliation_difference_amount`（gauge，标签 `item`、`legal_entity_id`）、`ep_invoice_import_rows_total`（counter，标签 `outcome`）。
3. 第 11 节：新增两项全局取值，即资金账户期初余额的存在与勾稽要求、资金类单据的冲正登记路径（临时闭合 U-D-02）。
4. 第 3.5 节：确认本阶段全部金额列均为 `numeric(18,2)`、比例列为 `numeric(9,6)`、数量列为 `numeric(18,6)`，本阶段未引入任何新的精度语义。
