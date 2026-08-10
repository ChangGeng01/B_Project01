## 阶段 7：采购、门户与收货

本阶段的范围是采购需求、采购订货与分批订货、供应商采购扩展档案、供应商门户、收货登记与入库单、采购退货、付款申请与审批。本阶段不实现采购发票登记、进项红字冲销、应付台账、付款登记与供应商返款，这五项属财务与发票阶段，本阶段只按契约衔接；进项发票台账的两张表 `invoice.purchase_invoices` 与 `invoice.purchase_invoice_lines` 由阶段 10 在 invoice schema 建立，本阶段不建表也不写台账。本阶段不实现库存数量账与金额账的写入算法，也不实现事件到分录的映射；本阶段不自行取价，取价一律归库存模块，总账只做分录映射与借贷平衡，本阶段按规格第 5.2 章财务规则条目与 PRD 第 5 节的分工调用其契约。本阶段不发布任何受治理数据集视图，采购发票数据集由阶段 10 在 invoice schema 发布。

本计划遵守共享技术基线。凡基线已给出取值的一律直接引用，不重新决定。本阶段新增的决定与假设集中在第 11.2 小节，并在正文各处以「本阶段新增决定」或「假设」标注。
本阶段在贯通线 T0 之后开工。T0 是阶段 3b-1 结束后、阶段 5 全量开工之前插入的一条不新增范围的最薄贯通线，其前置为阶段 1、2、3a、4 与 3b-1，固定链为 1 → 2 → 3a → 4 → 3b-1 → T0 → 5 → 9a → 8 → 6 → 7 → 10 → 11 → 9b → 14 共十五个环节，本阶段是其中第十一环，切片取自阶段 5、6、9a、10、11 五个阶段，判据是一条合同从建单走到管理层看到一个数。采购不在 T0 的切片清单内，本阶段不向 T0 贡献任何切片，也不因 T0 提前交付任何采购能力。本阶段的全部工作是在 T0 已经贯通的骨架上加厚：会计期间解析、凭证生成、销项发票与到款登记在 T0 上已经跑通，本阶段沿这条骨架追加采购订货、收货、采购退货与付款申请四段，另加门户这一条外部入口。骨架上已经成立的判据不在本阶段重新论证，本阶段只对新增的分支给判据。M7 相应改为全分支闭环而不是闭环的首次贯通，本阶段的验收措辞一律不写首次跑通。

---

### 1. 交付物清单

本阶段结束时，下列可运行物存在并可在单台服务器的认证部署形态上启动与验证。

1. core-server 进程内新增两组 HTTP 路由并可用：内部采购路由 `/api/v1/procure/*`，门户受控能力路由 `/api/v1/portal/*`。两组路由共用 core-server 的安全上下文、行级隔离、幂等、审计与 Outbox 机制，不新增第二套。
2. portal-gateway 进程可对外承载供应商门户站点，路由前缀 `/portal/v1`，实现门户会话、限流、水印与呈现层裁剪，全部取数与写入经 core-server 的受控能力 API，本进程不建立任何事务数据库连接。
3. job-worker 进程内新增四类消费者与一个定时任务：合同生效派生采购需求的 Outbox 消费者、采购退货生成供应商质量记录的消费者、采购与门户单据的检索索引与门户投影刷新消费者（产出 `foundation::port::search::SearchDocument` 并经 `SearchIndexPort` 写入）、采购与门户单据的站内通知投递消费者，以及库存不足触发采购需求的扫描定时任务（默认关闭，理由见第 11.2 小节）。
4. `procure` 与 `portal` 两个 schema 的全部表、约束、索引与行级安全策略，经 refinery 迁移可离线执行并可按迁移文件头的回退说明回退。
5. 六个新增 crate：`ep-contract-procure`、`ep-domain-procure`、`ep-app-procure`、`ep-contract-portal`、`ep-domain-portal`、`ep-app-portal`。
6. `ep-testkit` 中新增的采购与门户构造器，以及两个记录型桩（`RecordingStockPostingPort` 记录 `ep_contract_inventory::InventoryPostingPort` 的调用，`RecordingLedgerPostingPort` 记录 `ep_contract_ledger::PostingPort` 的调用），用于契约测试的入参断言与故障注入。库存与总账的真实实现分别在阶段 8 与阶段 9a 已合入，两个记录型桩只出现在测试装配，发布装配一律注入真实实现。发票与财务两个模块的四个端口在本阶段之后交付，本阶段一律不注入替身，四个调用点在本阶段的代码中不存在：`ReceiptInvoiceMatchQueryPort` 与 `PurchaseCreditNotePort` 所支撑的采购退货发票已登记分支按第 4.4 小节整条推迟到阶段 10；`PayableLedgerQuery` 所支撑的付款申请 `INVOICE_PAYMENT` 分支按第 4.5 小节整条推迟到阶段 10；`SupplierStatementQuery` 所支撑的三个门户对账端点按第 5.7 小节随该端口在阶段 10 同批交付。两个 wiring 目录下的全部文件中不出现任何以 `Noop` 前缀命名的注入行。
7. `ep-datagen` 中新增的采购侧基准数据生成器，产出附录 A.3 规模中的采购订单行 10 万条与其对应的收货、退货与付款申请分布。
8. 一份可执行的端到端用例集合，覆盖规格第 8 章闭环第 4 步、第 5 步收货腿、第 10 步的申请与审批腿、第 11 步的采购退货腿，以及规格第 19 章阶段 3 门户条目要求的采购订单与交期确认、送货通知、发票上传、收付款对账查询四项闭环用例。
9. 三份登记文档的增量：`docs/event-catalog.md` 新增 14 个事件类型，`docs/error-codes.md` 新增 35 个 PROCURE 与 PORTAL 段错误码，其中含第 4.4 小节推迟窗口的硬阻断码 `PROCURE.PURCHASE_RETURN.INVOICE_STAGE_PENDING`（平台段错误码由阶段 1 登记，本阶段只引用），`docs/data-dictionary.md` 新增 30 张表并在单据类型码一节补齐本阶段的八个类型码。
10. 采购模块的四端界面：`clients/desktop/src/modules/procure/` 与 `clients/mobile/src/modules/procure/` 两个目录；供应商门户站点以浏览器承载，由 portal-gateway 交付，不进 `clients/`。

---

### 2. crate 与进程归属

#### 2.1 新增 crate

| crate | 目录 | 职责 | 依赖 |
|---|---|---|---|
| ep-contract-procure | crates/contract/procure | 采购模块对外公开的命令、查询、事件类型与 DTO；供其他模块调用的 trait，含 `PaymentRequestWritebackPort`、`PurchaseOrderInvoicingPort`、`GoodsReceiptQueryPort`、`PurchaseRequisitionIntakePort`、`PurchaseReturnLinkPort`、`GrniSubledgerBalancePort`（子账侧余额端口，定义在 `src/port/subledger_balance.rs`，见裁定 G-01）；`src/capability.rs` 中为每个用例声明 `<USECASE_SCREAMING>_DOMAIN` 与 `<USECASE_SCREAMING>_ACTION` 一对常量 | 仅 ep-foundation |
| ep-domain-procure | crates/domain/procure | 采购需求、采购订单、收货单、采购退货单、付款申请、供应商准入与质量记录六类聚合；数量守恒、可退数量、累计下达数量、占用金额四组不变量；业务端口 trait | ep-foundation、ep-contract-procure |
| ep-app-procure | crates/application/procure | 采购各用例、事务边界、授权调用、审计与 Outbox 写入、与库存与总账两个模块契约的编排；`src/probe/` 下的 `ProcureReferenceCounter` 与 `ProcureTradeHistoryProvider`；六个 `ReconCheck` 实现（R-PROC-01 至 R-PROC-05 与 R-PORT-01）；`GrniSubledgerBalanceQuery`，即 `ep_contract_procure::GrniSubledgerBalancePort` 的实现类型，位于 `crates/application/procure/src/projection/subledger_balance.rs` | ep-foundation、ep-platform-*、ep-domain-procure、ep-contract-* |
| ep-contract-portal | crates/contract/portal | 门户受控能力的命令、查询与 DTO；门户投影的字段白名单类型；`PortalCapability` 枚举；`src/capability.rs` 中为每个门户用例声明 `<USECASE_SCREAMING>_DOMAIN` 与 `<USECASE_SCREAMING>_ACTION` 一对常量 | 仅 ep-foundation |
| ep-domain-portal | crates/domain/portal | 门户账号绑定、送货通知、发票上传记录三类聚合；能力白名单与供应商数据范围两组不变量 | ep-foundation、ep-contract-portal |
| ep-app-portal | crates/application/portal | 门户五项能力的受控用例、投影组装与脱敏裁剪、门户操作审计写入 | ep-foundation、ep-platform-*、ep-domain-portal、ep-contract-* |

依赖方向逐条自检：`ep-domain-procure` 不依赖 `ep-contract-inventory` 与 `ep-contract-ledger`，跨模块调用一律经 `ep-app-procure`；`ep-app-portal` 不依赖 `ep-app-procure`，门户对采购单据的读写经 `ep-contract-procure` 的 trait，实现在 `apps/core-server/src/wiring/` 目录下注入。这两条是本阶段最容易被违反的两条，由阶段 1 交付的 `xtask archcheck` 按层位断言：前者落禁止项第一条 `domain-no-cross-module`，后者落禁止项第二条 `app-no-peer-app`，被测输入是 `cargo metadata --no-deps` 建出的层位图；本阶段不另立按 crate 逐项比对期望依赖清单的自检脚本（裁定 F-05 通则甲-3）。

#### 2.2 改动的既有 crate

| crate | 改动 |
|---|---|
| ep-adapter-db-pg | 新增 `src/repo/procure/` 与 `src/repo/portal/` 两个目录，按表分文件，每个仓储只访问自己模块的 schema |
| ep-testkit | 新增 `SupplierFixture`、`PurchaseOrderBuilder`、`GoodsReceiptBuilder`、`PurchaseReturnBuilder`、`PaymentRequestBuilder`、`PortalUserFixture`、`DeliveryNoticeBuilder`；新增库存与总账两个契约的记录型桩 |
| ep-datagen | 新增 `--module procure` 分支 |
| apps/core-server | 路由注册、权限对象类型注册；wiring 注入，其中发票与财务两个模块的端口一律不注入任何替身且其调用点在本阶段的代码中不存在，`PurchaseReturnLinkPort` 由本阶段首次接线，以及本模块按裁定 A-15 向 `MasterReferenceCounterRegistry` 与 `TradeHistoryProviderRegistry` 的注册 |
| apps/portal-gateway | 站点、会话、限流、水印、五项能力的呈现层与转发 |
| apps/job-worker | 四个消费者与一个定时任务的注册；本阶段六个 `ReconCheck`（R-PROC-01 至 R-PROC-05 与 R-PORT-01）向 `ReconRegistry` 的注册 |
| ep-platform-obs | 注册本阶段新增的三个指标 |

#### 2.3 进程归属

| 能力 | 承载进程 |
|---|---|
| 采购全部用例、门户受控能力 API、收货与退货的同事务过账编排 | core-server |
| 门户站点、门户会话、门户限流、门户水印与呈现层 | portal-gateway |
| 合同派生需求的 Outbox 消费、质量记录生成、门户投影与检索索引刷新、站内通知投递、库存不足扫描定时任务 | job-worker |
| 本阶段新增指标的暴露 | ops-agent |

本阶段不新增进程，不改动进程的监听地址、数据库连接池上限、系统账户与 cgroup slice。

---

### 3. 数据库变更

#### 3.1 通用约定

本节全部表按基线第 4 节带齐九个公共列（`id`、`legal_entity_id`、`security_level`、`data_scope_tags`、`row_version`、`created_at`、`created_by`、`updated_at`、`updated_by`），下文的列清单只列该表特有的列，不重复公共列。单据类表另带 `doc_no text not null` 与 `status text not null`，二者也不在下文重复列出，只列出 `status` 的 CHECK 取值。

本阶段八张单据类表与单据类型码的对应固定如下，类型码登记入 `docs/data-dictionary.md` 的单据类型码一节，由 `xtask configdoc --check-doc-type-codes` 校验全局唯一且与 `ep-platform-sequence` 的常量表逐项一致。

| 表 | 类型码 |
|---|---|
| procure.purchase_requisitions | PR |
| procure.purchase_orders | PO |
| procure.goods_receipts | GR |
| procure.receipt_rejections | RJ |
| procure.purchase_returns | PRT |
| procure.payment_requests | PAYR |
| portal.delivery_notices | DN |
| portal.supplier_invoice_uploads | SIU |

全部表带 `legal_entity_id`，因此全部按基线第 3.8 节的模板生成四条行级安全语句，模板由迁移生成器统一产出，不手写变体。策略名一律 `rls_<table>_le`。

跨 schema 引用一律不建数据库外键，只留逻辑引用列，存在性由 `ep-app-procure` 与 `ep-app-portal` 在写入前经对方模块契约校验一次。本阶段不再为跨模块逻辑引用另建周期性存在性核对项：按基线第 3.3 节，未登记的跨模块逻辑引用只做写入时校验，对账框架只承担金额与数量守恒判据，不兼职做存在性巡检。同一 schema 内的引用建真实外键，`ON DELETE RESTRICT`。

金额列一律 `numeric(18,2)`，单价列一律 `numeric(18,6)`，数量列一律 `numeric(18,6)`，税率列一律 `numeric(9,6)`。批次列与序列号列在物料未启用相应管理时取固定值 `'-'`，按基线第 11.4 节。

密级取值：采购需求、采购订单、收货单、采购退货单、送货通知、发票上传取 20；付款申请取 30。数据范围标签的取值集合为 `dept:<部门码>`、`supplier:<供应商编码>`、`contract:<合同编号>`、`project:<项目编号>`、`sales_order:<订单编号>`，标签不承载任何敏感值。

#### 3.2 procure schema 的表

##### 3.2.1 procure.supplier_admissions（供应商准入结论）

| 列 | 类型 | 可空 | 说明 |
|---|---|---|---|
| supplier_id | uuid | 否 | 逻辑引用 mdm 的供应商档案 |
| admission_status | text | 否 | CHECK 取值 `PENDING`、`REJECTED`、`ADMITTED`、`SUSPENDED`、`TERMINATED` |
| concluded_on | date | 是 | 准入结论日期 |
| reviewer_user_id | uuid | 是 | 审核人 |
| valid_until | date | 是 | 准入有效期止日 |
| reason | text | 是 | CHECK 长度不超过 2000 |
| portal_enabled | boolean | 否 | 默认 false，与 mdm 的门户开通标记同步，取值来源以 mdm 为准 |

约束与索引：`pk_supplier_admissions`；`ux_supplier_admissions_legal_entity_id_supplier_id`；`ix_supplier_admissions_legal_entity_id_created_at`；`ix_supplier_admissions_legal_entity_id_valid_until`。本表不是单据类也不是档案类，不带 `doc_no` 与 `code`。

##### 3.2.2 procure.supplier_quality_records（供应商质量记录）

| 列 | 类型 | 可空 | 说明 |
|---|---|---|---|
| supplier_id | uuid | 否 | 逻辑引用 |
| source_type | text | 否 | CHECK 取值 `PURCHASE_RETURN`、`RECEIPT_REJECTION`、`MANUAL` |
| source_doc_id | uuid | 是 | 同 schema 内引用，不建外键，因为三类来源指向不同表 |
| source_doc_no | text | 是 | 冗余存编号，供只读展示 |
| material_id | uuid | 是 | 逻辑引用 |
| reason_code | text | 否 | 退货原因字典码，字典归平台配置 |
| quantity | numeric(18,6) | 是 | 涉及数量 |
| occurred_on | date | 否 | 发生日期 |
| conclusion | text | 是 | 处理结论，CHECK 长度不超过 2000 |

索引：`pk_`；`ix_supplier_quality_records_legal_entity_id_created_at`；`ix_supplier_quality_records_legal_entity_id_supplier_id_occurred_on`。

供应商风险记录不在本 schema 承载。`procure.supplier_risk_records` 已撤销，风险记录的唯一出处是 `mdm.supplier_risk_records`，本阶段的读写一律经阶段 5 提供的 `ep_contract_mdm::SupplierRiskRecordPort::append` 与 `::list`。

##### 3.2.3 procure.purchase_requisitions（采购需求，单据类）

采购需求按 PRD 第 4.3.2 小节的字段表实现为单行单据，不设明细行表，理由见第 11.2 小节的假设 A1。

| 列 | 类型 | 可空 | 说明 |
|---|---|---|---|
| source_type | text | 否 | CHECK 取值 `CONTRACT`、`SALES_ORDER`、`PROJECT_TASK`、`STOCK_SHORTAGE` |
| source_doc_id | uuid | 是 | 逻辑引用 |
| source_doc_line_id | uuid | 是 | 逻辑引用 |
| source_doc_no | text | 是 | 只读展示用 |
| source_idempotency_key | text | 否 | 来源侧幂等键，四类来源各自的去重依据 |
| suggested_purchase_type | text | 否 | CHECK 取值 `MATERIAL`、`DIRECT_EXPENSE` |
| material_id | uuid | 是 | 物料类必填，逻辑引用 |
| expense_item_code | text | 是 | 直接费用类必填 |
| required_quantity | numeric(18,6) | 否 | CHECK 大于零 |
| ordered_quantity | numeric(18,6) | 否 | 默认 0，累计已下达数量 |
| expected_arrival_date | date | 否 | CHECK 不早于 `created_at` 的服务器自然日 |
| contract_id / sales_order_id / project_id | uuid | 是 | 直接费用类至少一项非空，由 CHECK 表达 |
| suggested_supplier_id | uuid | 是 | 逻辑引用 |
| is_drop_ship | boolean | 否 | 默认 false，来源为直运订单时为 true 且 `suggested_purchase_type` 固定为 `DIRECT_EXPENSE` |
| close_reason | text | 是 | 关闭原因 |
| closed_at | timestamptz | 是 | |

`status` CHECK 取值 `PENDING`、`PARTIALLY_ORDERED`、`ORDERED`、`CLOSED`。表级 CHECK：`ck_purchase_requisitions_ordered_qty_le_required`（`ordered_quantity <= required_quantity`）；`ck_purchase_requisitions_type_fields`（物料类必填 `material_id`，直接费用类必填 `expense_item_code` 且三个归集字段至少一项非空）；`ck_purchase_requisitions_drop_ship_type`（`is_drop_ship` 为真时类型必须为 `DIRECT_EXPENSE`）。

索引：`pk_`；`ux_purchase_requisitions_legal_entity_id_doc_no`；`ix_purchase_requisitions_legal_entity_id_created_at`；`ux_purchase_requisitions_legal_entity_id_source_idempotency_key`；`ix_purchase_requisitions_legal_entity_id_status_expected_arrival_date`。

##### 3.2.4 procure.purchase_orders（采购订单，单据类）

| 列 | 类型 | 可空 | 说明 |
|---|---|---|---|
| supplier_id | uuid | 否 | 逻辑引用 |
| purchase_type | text | 否 | CHECK 取值 `MATERIAL`、`DIRECT_EXPENSE` |
| order_date | date | 否 | |
| payment_terms_code | text | 是 | 付款条件字典码 |
| total_untaxed_amount | numeric(18,2) | 否 | 默认 0，由行汇总维护 |
| total_tax_amount | numeric(18,2) | 否 | 默认 0 |
| total_gross_amount | numeric(18,2) | 否 | 默认 0 |
| reschedule_round | integer | 否 | 默认 0，改期协商轮次计数 |
| approval_ref | uuid | 是 | 流程实例引用 |
| issued_at | timestamptz | 是 | |
| close_reason | text | 是 | |
| closed_at / voided_at | timestamptz | 是 | |
| is_type_locked | boolean | 否 | 默认 false，首次收货登记或首次采购发票登记后置为 true |

`status` CHECK 取值 `DRAFT`、`PENDING_APPROVAL`、`REJECTED`、`ISSUED`、`PENDING_SUPPLIER_CONFIRM`、`SUPPLIER_RESCHEDULE_PROPOSED`、`SUPPLIER_CONFIRMED`、`PARTIALLY_RECEIVED`、`COMPLETED`、`CLOSED`、`VOIDED`。

索引：`pk_`；`ux_purchase_orders_legal_entity_id_doc_no`；`ix_purchase_orders_legal_entity_id_created_at`；`ix_purchase_orders_legal_entity_id_supplier_id_status`；`ix_purchase_orders_legal_entity_id_status_order_date`。第四条索引直接支撑门户的待确认列表与 A.1 度量项「采购订单与交期待确认列表加载」，须在基准数据集上给出无顺序扫描的 `EXPLAIN` 证据。

##### 3.2.5 procure.purchase_order_lines

| 列 | 类型 | 可空 | 说明 |
|---|---|---|---|
| purchase_order_id | uuid | 否 | 同 schema 外键 `fk_purchase_order_lines_purchase_orders` |
| line_no | integer | 否 | 行号 |
| purchase_requisition_id | uuid | 是 | 同 schema 外键 |
| material_id | uuid | 是 | 逻辑引用 |
| expense_item_code | text | 是 | |
| quantity | numeric(18,6) | 否 | CHECK 大于零 |
| unit_price_untaxed | numeric(18,6) | 否 | CHECK 大于等于零 |
| tax_rate | numeric(9,6) | 否 | 取值来自税率字典，唯一出处按裁定 C-11 与总览第 1.5 节第五条为 `invoice.tax_rate_options`，唯一取用入口为 `ep_contract_invoice::TaxRateOptionQuery` 的 `default_rate` 与 `list`，该表的建表与种子两条迁移及该查询由阶段 10 在 T0 期间交付，属阶段 10 的 T0 切片第五项；本阶段取默认税率一律经 ep-contract-invoice，不经 ep-contract-mdm，不自建税率字典，也不存在任何税率桩 |
| agreed_delivery_date | date | 否 | CHECK 不早于订单日期，由应用层校验并在写入时冗余 `order_date` 以支撑表级 CHECK |
| order_date | date | 否 | 冗余自订单头，仅为表级 CHECK 与索引服务 |
| warehouse_id | uuid | 是 | 物料类必填，逻辑引用 |
| contract_id / sales_order_id / project_id | uuid | 是 | 直接费用类至少一项非空 |
| received_quantity | numeric(18,6) | 否 | 默认 0 |
| returned_quantity | numeric(18,6) | 否 | 默认 0 |
| invoiced_quantity | numeric(18,6) | 否 | 默认 0，由发票模块经契约回写，本阶段只建列与回写入口 |
| line_status | text | 否 | CHECK 取值 `OPEN`、`FULLY_RECEIVED`、`CLOSED`、`VOIDED` |

表级 CHECK：`ck_purchase_order_lines_delivery_date`（`agreed_delivery_date >= order_date`）；`ck_purchase_order_lines_type_fields`。索引：`pk_`；`ux_purchase_order_lines_purchase_order_id_line_no`；`ix_purchase_order_lines_legal_entity_id_created_at`；`ix_purchase_order_lines_legal_entity_id_material_id_line_status`。

##### 3.2.6 procure.purchase_order_line_batches（交货批次行）

列为 `purchase_order_line_id uuid not null`（同 schema 外键）、`batch_no integer not null`、`batch_quantity numeric(18,6) not null`（CHECK 大于零）、`agreed_delivery_date date not null`、`received_quantity numeric(18,6) not null default 0`、`batch_status text not null`（CHECK 取值 `OPEN`、`FULLY_RECEIVED`、`CLOSED`）。索引：`pk_`；`ux_purchase_order_line_batches_purchase_order_line_id_batch_no`；`ix_..._legal_entity_id_created_at`。批次数量合计等于该行订单数量由领域层断言，不由数据库 CHECK 表达，理由是该断言跨行。

##### 3.2.7 procure.purchase_order_payment_plans（预计付款计划）

列为 `purchase_order_id uuid not null`（同 schema 外键）、`plan_no integer not null`、`planned_date date not null`、`planned_amount numeric(18,2) not null`、`plan_note text null`。索引：`pk_`；`ux_purchase_order_payment_plans_purchase_order_id_plan_no`；`ix_..._legal_entity_id_created_at`。

##### 3.2.8 procure.goods_receipts（收货单，单据类）

| 列 | 类型 | 可空 | 说明 |
|---|---|---|---|
| supplier_id | uuid | 否 | 逻辑引用 |
| purchase_order_id | uuid | 否 | 同 schema 外键 |
| delivery_notice_id | uuid | 是 | 逻辑引用 portal.delivery_notices，跨 schema 不建外键 |
| posting_date | date | 否 | 该业务事件的记账日期，取值即收货日期，CHECK 不晚于登记时点的服务器自然日 |
| accounting_period_id | uuid | 是 | 过账时由 `ep_contract_ledger::AccountingPeriodResolver::resolve` 解析并写入，草稿态为空 |
| voucher_id | uuid | 是 | 过账时由 `ep_contract_ledger::PostingPort::post` 返回并写入 |
| has_over_receipt | boolean | 否 | 默认 false |
| over_receipt_reason | text | 是 | `has_over_receipt` 为真时必填，由 CHECK 表达 |
| over_receipt_approval_ref | uuid | 是 | |
| posted_at | timestamptz | 是 | |

`status` CHECK 取值 `DRAFT`、`PENDING_APPROVAL`、`POSTED`、`PARTIALLY_RETURNED`、`FULLY_RETURNED`。`PENDING_APPROVAL` 是本阶段对 PRD 第 4.5.5 小节状态机的补充，理由是 PRD 第 4.5.4 小节要求超收转审批而状态机漏列该态，见第 11.2 小节假设 A8。

索引：`pk_`；`ux_goods_receipts_legal_entity_id_doc_no`；`ix_goods_receipts_legal_entity_id_created_at`；`ix_goods_receipts_legal_entity_id_purchase_order_id_status`；`ix_goods_receipts_legal_entity_id_posting_date`。

##### 3.2.9 procure.goods_receipt_lines

| 列 | 类型 | 可空 | 说明 |
|---|---|---|---|
| goods_receipt_id | uuid | 否 | 同 schema 外键 |
| line_no | integer | 否 | |
| purchase_order_line_id | uuid | 否 | 同 schema 外键 |
| purchase_order_line_batch_id | uuid | 是 | 同 schema 外键 |
| delivery_notice_line_id | uuid | 是 | 跨 schema 逻辑引用 |
| material_id | uuid | 否 | 逻辑引用 |
| warehouse_id | uuid | 否 | 逻辑引用 |
| quantity | numeric(18,6) | 否 | CHECK 大于零 |
| batch_no | text | 否 | 默认 `'-'` |
| returned_quantity | numeric(18,6) | 否 | 默认 0 |
| order_unit_price_untaxed | numeric(18,6) | 否 | 登记时固化的采购订单不含税单价，作为库存契约判定暂估入账单价的入参，采购侧不据此取价 |

表级 CHECK：`ck_goods_receipt_lines_returned_le_quantity`。索引：`pk_`；`ux_goods_receipt_lines_goods_receipt_id_line_no`；`ix_goods_receipt_lines_legal_entity_id_created_at`；`ix_goods_receipt_lines_legal_entity_id_purchase_order_line_id`；`ix_goods_receipt_lines_legal_entity_id_material_id_batch_no`。

##### 3.2.10 procure.goods_receipt_line_costings（收货行入账分配，仅追加）

本表只固化收货行的数量与金额分配关系，不固化单价。收货入账单价的权威出处是 `inventory.stock_value_entries.applied_unit_price`，采购侧一律经 `ep_contract_inventory::InventoryPricingLookupPort::original_unit_price_by_source_line(tx, ctx, source_doc_line_id)` 回查，`source_doc_line_id` 取该收货行标识；规格第 5.2 章退货回冲的取价三分支所称的「该退货明细行关联的收货单原入账单价」由该回查提供。本表是仅追加表，按基线第 4 节不带 `row_version`、`updated_at`、`updated_by`，改带 `reverses_id uuid null`。

| 列 | 类型 | 可空 | 说明 |
|---|---|---|---|
| goods_receipt_line_id | uuid | 否 | 同 schema 外键 |
| allocation_kind | text | 否 | CHECK 取值 `ESTIMATED`、`OVERBILL_REVERSE_MATCH`，领域侧对应 `CostingType` |
| quantity | numeric(18,6) | 否 | CHECK 大于零 |
| amount | numeric(18,2) | 否 | 入账金额，由库存契约在过账时返回，按四舍五入且中值远离零 round 到 2 位 |
| source_purchase_invoice_line_id | uuid | 是 | `OVERBILL_REVERSE_MATCH` 时非空，逻辑引用 `invoice.purchase_invoice_lines` |
| reverses_id | uuid | 是 | 冲销引用 |

取值由库存契约在过账时返回，`ep-app-procure` 不自行计算取价。同一收货行可产生一条或两条分配记录，两条时其数量合计等于该行 `quantity`，该断言由领域层与 `ReconCheck` R-PROC-05 双重校验。索引：`pk_`；`ix_goods_receipt_line_costings_legal_entity_id_created_at`；`ix_goods_receipt_line_costings_legal_entity_id_goods_receipt_line_id`。

##### 3.2.11 procure.goods_receipt_line_serials

列为 `goods_receipt_line_id uuid not null`（同 schema 外键）、`serial_no text not null`（CHECK 长度不超过 64）。索引：`pk_`；`ix_goods_receipt_line_serials_legal_entity_id_created_at`；`ix_goods_receipt_line_serials_legal_entity_id_serial_no`。本表不建序列号唯一约束，理由是同一序列号在退货后可再次收货，法人内唯一的在库判定由库存模块的序列号台账承担。

##### 3.2.12 procure.receipt_rejections（拒收记录，单据类）

列为 `supplier_id uuid not null`、`purchase_order_id uuid not null`（同 schema 外键）、`purchase_order_line_id uuid not null`（同 schema 外键）、`delivery_notice_id uuid null`、`rejected_quantity numeric(18,6) not null`（CHECK 大于零）、`reason_code text not null`、`reason_text text null`、`rejected_on date not null`。`status` CHECK 取值 `REGISTERED`。本单据不产生库存流水与凭证。索引：`pk_`；`ux_receipt_rejections_legal_entity_id_doc_no`；`ix_receipt_rejections_legal_entity_id_created_at`。

##### 3.2.13 procure.purchase_returns（采购退货单，单据类）

| 列 | 类型 | 可空 | 说明 |
|---|---|---|---|
| supplier_id | uuid | 否 | 逻辑引用 |
| return_scenario | text | 否 | CHECK 取值 `MATERIAL_RECEIPT`、`DROP_SHIP` |
| sales_return_id | uuid | 是 | `DROP_SHIP` 时必填，逻辑引用销售模块，由 CHECK 表达 |
| sales_return_doc_no | text | 是 | 只读展示用 |
| warehouse_id | uuid | 是 | `MATERIAL_RECEIPT` 时必填 |
| posting_date | date | 否 | 该业务事件的记账日期，取值即退货日期 |
| accounting_period_id | uuid | 是 | 过账时由 `ep_contract_ledger::AccountingPeriodResolver::resolve` 解析并写入 |
| voucher_id | uuid | 是 | 过账时由 `ep_contract_ledger::PostingPort::post` 返回并写入 |
| reason_code | text | 否 | 退货原因字典码 |
| is_invoice_registered | boolean | 是 | 系统判定，过账时由 `ep_contract_invoice::ReceiptInvoiceMatchQueryPort::match_state` 返回并固化，用户不可填写 |
| approval_ref | uuid | 是 | |
| posted_at / voided_at | timestamptz | 是 | |

`status` CHECK 取值 `DRAFT`、`PENDING_APPROVAL`、`REJECTED`、`POSTED`、`VOIDED`。索引：`pk_`；`ux_purchase_returns_legal_entity_id_doc_no`；`ix_purchase_returns_legal_entity_id_created_at`；`ix_purchase_returns_legal_entity_id_supplier_id_posting_date`。

##### 3.2.14 procure.purchase_return_lines

列为 `purchase_return_id uuid not null`（同 schema 外键）、`line_no integer not null`、`goods_receipt_line_id uuid null`（同 schema 外键，`MATERIAL_RECEIPT` 时必填）、`goods_receipt_line_costing_id uuid null`（同 schema 外键，指向被回冲的那一条入账分配，一行退货关联多次收货时按各自分配逐条录入）、`purchase_invoice_line_id uuid null`（`DROP_SHIP` 时指向原直接费用类归集的发票行，逻辑引用 `invoice.purchase_invoice_lines`）、`material_id uuid null`、`quantity numeric(18,6) not null`（CHECK 大于零）、`batch_no text not null default '-'`、`reversal_unit_price numeric(18,6) null`（过账时由库存契约返回并固化）、`reversal_amount numeric(18,2) null`。索引：`pk_`；`ux_purchase_return_lines_purchase_return_id_line_no`；`ix_..._legal_entity_id_created_at`；`ix_purchase_return_lines_legal_entity_id_goods_receipt_line_id`。

##### 3.2.15 procure.purchase_return_line_serials

列为 `purchase_return_line_id uuid not null`（同 schema 外键）、`serial_no text not null`。索引同 3.2.11 的模式。

##### 3.2.16 procure.payment_requests（付款申请，单据类）

| 列 | 类型 | 可空 | 说明 |
|---|---|---|---|
| supplier_id | uuid | 否 | 逻辑引用 |
| payment_type | text | 否 | CHECK 取值 `INVOICE_PAYMENT`、`PREPAYMENT` |
| requested_amount | numeric(18,2) | 否 | CHECK 大于零 |
| planned_payment_date | date | 否 | CHECK 不早于申请日期 |
| requested_on | date | 否 | 申请日期 |
| payee_account_ref | uuid | 否 | 指向 mdm 供应商档案上的收款账户行标识，本表只存引用，不复制账号明文 |
| request_note | text | 是 | 长度不超过 2000 |
| paid_amount | numeric(18,2) | 否 | 默认 0，由财务模块经契约回写 |
| approval_ref | uuid | 是 | |
| close_reason | text | 是 | |
| withdrawn_at / closed_at / voided_at | timestamptz | 是 | |

`status` CHECK 取值 `DRAFT`、`PENDING_APPROVAL`、`REJECTED`、`WITHDRAWN`、`APPROVED`、`PARTIALLY_PAID`、`FULLY_PAID`、`CLOSED`、`VOIDED`。表级 CHECK：`ck_payment_requests_paid_le_requested`。索引：`pk_`；`ux_payment_requests_legal_entity_id_doc_no`；`ix_payment_requests_legal_entity_id_created_at`；`ix_payment_requests_legal_entity_id_supplier_id_status`；`ix_payment_requests_legal_entity_id_status_planned_payment_date`。第四条与第五条支撑财务的待付款队列取数。

本表不存银行账号明文，只存 `payee_account_ref`，因此不承载规格第 7.8 章的行内敏感字段，字段级密级与解密路径仍在 mdm 侧判定。

##### 3.2.17 procure.payment_request_lines

列为 `payment_request_id uuid not null`（同 schema 外键）、`line_no integer not null`、`ref_type text not null`（CHECK 取值 `PURCHASE_INVOICE`、`CONTRACT`、`PURCHASE_ORDER`）、`ref_id uuid not null`（`PURCHASE_INVOICE` 时逻辑引用 `invoice.purchase_invoices`）、`ref_doc_no text null`、`requested_amount numeric(18,2) not null`（CHECK 大于零）。索引：`pk_`；`ux_payment_request_lines_payment_request_id_line_no`；`ix_..._legal_entity_id_created_at`；`ix_payment_request_lines_legal_entity_id_ref_type_ref_id`。

##### 3.2.18 procure.payable_reservations（应付占用汇总）

本表是 PRD 第 4.7.5 小节「同一张采购发票被多张未关闭的付款申请重复引用时按各申请已占用金额合计校验」的可串行化落点。

列为 `purchase_invoice_id uuid not null`（逻辑引用 `invoice.purchase_invoices`）、`reserved_amount numeric(18,2) not null default 0`（CHECK 大于等于零）、`open_request_count integer not null default 0`。约束：`ux_payable_reservations_legal_entity_id_purchase_invoice_id`。索引：`pk_`；`ix_payable_reservations_legal_entity_id_created_at`。

##### 3.2.19 附件关联表

按基线第 4 节命名，五张表结构一致，列为 `owner_id uuid not null`（同 schema 外键指向各自主表）、`attachment_object_id uuid not null`（跨 schema 逻辑引用 `platform_file.attachment_objects`）、`purpose text not null`、`sort_no integer not null`。

- `procure.purchase_order_attachments`
- `procure.goods_receipt_attachments`
- `procure.purchase_return_attachments`
- `procure.payment_request_attachments`
- `procure.receipt_rejection_attachments`

每张表的索引为 `pk_`、`ux_<table>_owner_id_attachment_object_id`、`ix_<table>_legal_entity_id_created_at`。

#### 3.3 portal schema 的表

##### 3.3.1 portal.supplier_portal_users（门户账号与供应商的授权绑定）

本表不承载身份主体本身。门户账号的目录、口令、MFA、会话与设备登记归 `platform_identity`，本表只存绑定与能力授权，理由见第 11.2 小节假设 A5。

| 列 | 类型 | 可空 | 说明 |
|---|---|---|---|
| identity_principal_id | uuid | 否 | 逻辑引用 platform_identity 的外部主体 |
| supplier_id | uuid | 否 | 逻辑引用 mdm |
| capabilities | text[] | 否 | 默认 `'{}'`，取值只允许五项能力码，由 CHECK 表达 |
| binding_status | text | 否 | CHECK 取值 `INVITED`、`PENDING_REVIEW`、`ACTIVE`、`SUSPENDED`、`DISABLED` |
| invited_by | uuid | 是 | |
| invited_at / activated_at / disabled_at | timestamptz | 是 | |
| self_registered | boolean | 否 | 默认 false，标记该绑定来自受限自助注册 |

五项能力码为 `ORDER_CONFIRM`、`DELIVERY_NOTICE`、`INVOICE_UPLOAD`、`SETTLEMENT_QUERY`、`PROFILE_MAINTAIN`。CHECK 表达为 `capabilities <@ ARRAY['ORDER_CONFIRM','DELIVERY_NOTICE','INVOICE_UPLOAD','SETTLEMENT_QUERY','PROFILE_MAINTAIN']::text[]`。

一行代表一个法人下的一条授权，跨法人授权由多行表达，本表因此不出现跨法人引用。索引：`pk_`；`ux_supplier_portal_users_legal_entity_id_identity_principal_id`；`ix_supplier_portal_users_legal_entity_id_created_at`；`ix_supplier_portal_users_legal_entity_id_supplier_id_binding_status`。

##### 3.3.2 portal.delivery_notices（送货通知，单据类）

列为 `supplier_id uuid not null`、`purchase_order_id uuid not null`（跨 schema 逻辑引用）、`purchase_order_doc_no text not null`、`expected_arrival_date date not null`、`carrier_name text null`、`waybill_no text null`、`remark text null`、`submitted_by_portal_user_id uuid not null`（同 schema 外键）、`voided_at timestamptz null`。`status` CHECK 取值 `SUBMITTED`、`PARTIALLY_RECEIVED`、`RECEIVED`、`VOIDED`。索引：`pk_`；`ux_delivery_notices_legal_entity_id_doc_no`；`ix_delivery_notices_legal_entity_id_created_at`；`ix_delivery_notices_legal_entity_id_supplier_id_status`；`ix_delivery_notices_legal_entity_id_purchase_order_id`。

##### 3.3.3 portal.delivery_notice_lines

列为 `delivery_notice_id uuid not null`（同 schema 外键）、`line_no integer not null`、`purchase_order_line_id uuid not null`（跨 schema 逻辑引用）、`purchase_order_line_batch_id uuid null`、`material_id uuid not null`、`quantity numeric(18,6) not null`（CHECK 大于零）、`batch_no text not null default '-'`、`received_quantity numeric(18,6) not null default 0`。表级 CHECK：`ck_delivery_notice_lines_received_le_quantity`。索引：`pk_`；`ux_delivery_notice_lines_delivery_notice_id_line_no`；`ix_..._legal_entity_id_created_at`；`ix_delivery_notice_lines_legal_entity_id_purchase_order_line_id`。

##### 3.3.4 portal.delivery_notice_line_serials

列为 `delivery_notice_line_id uuid not null`（同 schema 外键）、`serial_no text not null`。

##### 3.3.5 portal.supplier_invoice_uploads（发票上传记录，单据类）

| 列 | 类型 | 可空 | 说明 |
|---|---|---|---|
| supplier_id | uuid | 否 | |
| invoice_no | text | 否 | 长度不超过 64 |
| invoice_code | text | 是 | 长度不超过 64 |
| issued_on | date | 否 | |
| ref_type | text | 否 | CHECK 取值 `PURCHASE_ORDER`、`GOODS_RECEIPT` |
| ref_id | uuid | 否 | 跨 schema 逻辑引用 |
| ref_doc_no | text | 否 | |
| untaxed_amount | numeric(18,2) | 否 | |
| tax_rate | numeric(9,6) | 否 | |
| tax_amount | numeric(18,2) | 否 | |
| gross_amount | numeric(18,2) | 否 | 表级 CHECK `gross_amount = untaxed_amount + tax_amount` |
| return_reason | text | 是 | 财务退回时填写 |
| accepted_purchase_invoice_id | uuid | 是 | 逻辑引用 `invoice.purchase_invoices`，受理后由阶段 10 的 invoice 模块经契约回写 |
| submitted_by_portal_user_id | uuid | 否 | 同 schema 外键 |

`status` CHECK 取值 `UPLOADED`、`RETURNED`、`ACCEPTED`。约束 `ux_supplier_invoice_uploads_legal_entity_id_supplier_id_invoice_code_invoice_no`，`invoice_code` 为空时以空串参与唯一，写入前由应用层归一，理由是 NULL 在唯一约束中不参与比较会放过重复上传。索引：`pk_`；`ux_supplier_invoice_uploads_legal_entity_id_doc_no`；`ix_..._legal_entity_id_created_at`；`ix_supplier_invoice_uploads_legal_entity_id_status_issued_on`。

##### 3.3.6 附件关联表

`portal.delivery_notice_attachments` 与 `portal.supplier_invoice_upload_attachments`，结构同第 3.2.19 小节。

#### 3.4 迁移编号与顺序

迁移文件放在 `db/migrations/procure/` 与 `db/migrations/portal/`，迁移历史统一落在全局唯一的 `platform_core.schema_history`。执行顺序由单一全局 Runner 按文件版本号全序排定，不存在任何模块顺序声明文件，本阶段两个目录的文件版本号一律晚于其全部被引用对象的建表迁移。

本阶段占用两段迁移时间窗，段内按下表顺序执行。每个文件只做一件事，每个文件头必须带 `-- rollback:` 段。

| 序 | 文件名 | 回退说明 |
|---|---|---|
| 1 | V202611030901__procure_create_supplier_admissions.sql | drop table |
| 2 | V202611030902__procure_create_supplier_quality_records.sql | drop table |
| 3 | V202611030903__procure_create_purchase_requisitions.sql | drop table |
| 4 | V202611030904__procure_create_purchase_orders.sql | drop table |
| 5 | V202611030905__procure_create_purchase_order_lines.sql | drop table |
| 6 | V202611030906__procure_create_purchase_order_line_batches.sql | drop table |
| 7 | V202611030907__procure_create_purchase_order_payment_plans.sql | drop table |
| 8 | V202611030908__procure_create_goods_receipts.sql | drop table |
| 9 | V202611030909__procure_create_goods_receipt_lines.sql | drop table |
| 10 | V202611030910__procure_create_goods_receipt_line_serials.sql | drop table |
| 11 | V202611030911__procure_create_goods_receipt_line_costings.sql | drop table |
| 12 | V202611030912__procure_create_receipt_rejections.sql | drop table |
| 13 | V202611030913__procure_create_purchase_returns.sql | drop table |
| 14 | V202611030914__procure_create_purchase_return_lines.sql | drop table |
| 15 | V202611030915__procure_create_purchase_return_line_serials.sql | drop table |
| 16 | V202611030916__procure_create_payment_requests.sql | drop table |
| 17 | V202611030917__procure_create_payment_request_lines.sql | drop table |
| 18 | V202611030918__procure_create_payable_reservations.sql | drop table |
| 19 | V202611030919__procure_create_purchase_order_attachments.sql | drop table |
| 20 | V202611030920__procure_create_goods_receipt_attachments.sql | drop table |
| 21 | V202611030921__procure_create_purchase_return_attachments.sql | drop table |
| 22 | V202611030922__procure_create_payment_request_attachments.sql | drop table |
| 23 | V202611030923__procure_create_receipt_rejection_attachments.sql | drop table |
| 24 | V202611030924__procure_backfill_append_only_registry.sql | delete 本次插入的登记行 |
| 25 | V202611031201__portal_create_supplier_portal_users.sql | drop table |
| 26 | V202611031202__portal_create_delivery_notices.sql | drop table |
| 27 | V202611031203__portal_create_delivery_notice_lines.sql | drop table |
| 28 | V202611031204__portal_create_delivery_notice_line_serials.sql | drop table |
| 29 | V202611031205__portal_create_delivery_notice_attachments.sql | drop table |
| 30 | V202611031206__portal_create_supplier_invoice_uploads.sql | drop table |
| 31 | V202611031207__portal_create_supplier_invoice_upload_attachments.sql | drop table |

三十一个文件中三十个为新增表、第 24 号为登记回填，全部落在基线第 3.9 节的在线变更范围内，索引一律 `CREATE INDEX CONCURRENTLY`，迁移会话固定 `SET lock_timeout = '5s'` 与 `SET statement_timeout = '30min'`。第 24 号文件按裁定 B-02 向 `platform_core.append_only_registry` 插入一行，`schema_name` 取 `procure`、`table_name` 取 `goods_receipt_line_costings`、`mode` 取 `APPEND_ONLY`、`mutable_columns` 取空数组，`created_by` 取 `foundation::SYSTEM_PRINCIPAL_ID`，仅追加触发器按该登记行挂接，回退为删除该行。该文件同时读写 `platform_core` 与 `procure` 两个 schema，其主要创建对象是 `procure.goods_receipt_line_costings` 上的仅追加触发器与其登记行，按裁定通则第五条放在 `db/migrations/procure/` 目录下，版本号晚于阶段 2 建立 `platform_core.append_only_registry` 的迁移。本阶段按裁定 A-21 不新增 `ledger.posting_trigger_event_types` 的回填迁移，`procure.goods_receipt.posted.v1` 与 `procure.purchase_return.posted.v1` 两行登记由阶段 9a 的种子迁移一次写入，见第 6.5 小节。

模块属主角色为 `ep_mod_procure` 与 `ep_mod_portal`，两者在迁移中建立表与索引的归属，运行期读写仍只由 `ep_app_rw` 承担。

---

### 4. 领域模型与关键算法

#### 4.1 核心类型

`ep-domain-procure` 的聚合与值对象。

```
model/purchase_requisition.rs   PurchaseRequisition
model/purchase_order.rs         PurchaseOrder（含 Line、LineBatch 两个实体）
model/goods_receipt.rs          GoodsReceipt（含 Line、LineCosting、LineSerial）
model/purchase_return.rs        PurchaseReturn（含 Line、LineSerial）
model/payment_request.rs        PaymentRequest（含 Line）
model/supplier_admission.rs     SupplierAdmission
value/purchase_type.rs          PurchaseType { Material, DirectExpense }
value/requisition_source.rs     RequisitionSource { Contract, SalesOrder, ProjectTask, StockShortage }
value/receivable_progress.rs    ReceiptProgress { ordered, received, returned }
value/return_scenario.rs        ReturnScenario { MaterialReceipt, DropShip }
value/costing_type.rs           CostingType { Estimated, OverbillReverseMatch }，落库列名 allocation_kind
value/reservation.rs            PayableReservation
rule/quantity_balance.rs        累计下达、累计收货、累计退货三组不变量
rule/cost_dimension.rs          直接费用类归集字段必填其一
rule/supplier_gate.rs           供应商准入与资质闸门
```

`ep-domain-portal` 的聚合与值对象。

```
model/supplier_portal_user.rs   SupplierPortalUser
model/delivery_notice.rs        DeliveryNotice（含 Line、LineSerial）
model/supplier_invoice_upload.rs SupplierInvoiceUpload
value/portal_capability.rs      PortalCapability 五项能力码
rule/data_scope.rs              门户数据范围：交易对手等于本供应商，且法人在授权集合内
rule/field_whitelist.rs         五项能力各自的返回字段白名单
```

全部金额用 `foundation::Money`，单价用 `foundation::UnitPrice`，数量用 `foundation::Quantity`，比例用 `foundation::Rate`。领域层不取当前时间，一律经 `foundation::Clock`；不生成标识符，一律经 `foundation::IdGen`。

#### 4.2 状态机

##### 4.2.1 采购需求

| 当前状态 | 目标状态 | 触发 | 守卫条件 |
|---|---|---|---|
| （无） | PENDING | 四类来源生成 | 来源单据行存在且未关闭；同一 `source_idempotency_key` 不存在 |
| PENDING | PARTIALLY_ORDERED | 采购订单下达 | `0 < ordered_quantity < required_quantity` |
| PENDING | ORDERED | 采购订单下达 | `ordered_quantity = required_quantity` |
| PARTIALLY_ORDERED | ORDERED | 采购订单下达 | 同上 |
| PENDING / PARTIALLY_ORDERED | CLOSED | 采购员手工关闭或来源单据作废 | 手工关闭必填 `close_reason`；来源作废由 Outbox 消费者触发并写审计 |
| ORDERED | CLOSED | 采购员手工关闭 | 必填 `close_reason` |

`ordered_quantity` 的增减只在采购订单进入 `ISSUED` 与离开 `ISSUED` 及其后各态时发生。订单作废时按行回退，订单提前关闭时不回退，理由是已下达数量已经发生。

##### 4.2.2 采购订单

状态与流转按 PRD 第 4.4.4 小节，守卫条件如下。

| 流转 | 守卫条件 |
|---|---|
| DRAFT → PENDING_APPROVAL | 供应商准入状态为 ADMITTED；供应商资质未整体过期；直接费用类三个归集字段至少一项非空；物料类每行 `warehouse_id` 非空；每行累计下达数量不超过关联需求数量；交货批次行数量合计等于该行订单数量 |
| PENDING_APPROVAL → ISSUED | 审批链全部节点通过；审批链为空时提交即进入本态 |
| PENDING_APPROVAL → REJECTED | 任一节点驳回 |
| REJECTED → DRAFT | 采购员修改 |
| ISSUED → PENDING_SUPPLIER_CONFIRM | 下达成功后自动进入 |
| PENDING_SUPPLIER_CONFIRM → SUPPLIER_CONFIRMED | 门户确认 |
| PENDING_SUPPLIER_CONFIRM → SUPPLIER_RESCHEDULE_PROPOSED | 门户提出改期，只允许改交期字段 |
| SUPPLIER_RESCHEDULE_PROPOSED → SUPPLIER_CONFIRMED | 采购主管接受改期，同时更新行交期并写审计 |
| SUPPLIER_RESCHEDULE_PROPOSED → PENDING_SUPPLIER_CONFIRM | 采购主管拒绝改期，`reschedule_round` 加一 |
| SUPPLIER_CONFIRMED → PARTIALLY_RECEIVED | 存在已过账收货且累计收货数量小于订单数量 |
| SUPPLIER_CONFIRMED / PARTIALLY_RECEIVED → COMPLETED | 物料类累计收货数量等于订单数量；直接费用类由发票模块经契约标记发票登记完毕 |
| 任一态 → CLOSED | 采购主管填写原因提前关闭；剩余未收数量不再收货 |
| DRAFT / PENDING_SUPPLIER_CONFIRM / SUPPLIER_RESCHEDULE_PROPOSED → VOIDED | 无任何收货登记、无已登记采购发票、无已审批付款申请三条同时成立 |

已下达订单的变更（数量、单价、交期、仓库）经 `actions/revise` 表达，守卫条件为该行已收货部分对应的数量与单价不允许变更，变更后订单回到 `PENDING_SUPPLIER_CONFIRM`。变更前后取值由 `platform_audit` 的 `before` 与 `after` 承载，本阶段不建变更历史表。

##### 4.2.3 收货单

| 流转 | 守卫条件 |
|---|---|
| DRAFT → PENDING_APPROVAL | `has_over_receipt` 为真且超收转审批开关为开 |
| DRAFT → POSTED | 无超收，或超收但转审批开关为关；且通过第 4.3 小节的过账前置校验 |
| PENDING_APPROVAL → POSTED | 审批链通过 |
| PENDING_APPROVAL → DRAFT | 审批驳回 |
| POSTED → PARTIALLY_RETURNED | 累计退货数量大于零且小于本次收货数量 |
| POSTED / PARTIALLY_RETURNED → FULLY_RETURNED | 累计退货数量等于本次收货数量 |

`POSTED` 之后单据本体不可修改也不可删除，只有 `status`、`row_version` 与三个累计数量列随退货过账变化，更正只能通过采购退货或红字路径实现。

##### 4.2.4 采购退货单

按 PRD 第 4.6.4 小节，守卫条件如下。`DRAFT → PENDING_APPROVAL` 要求物料类每行的本次退货数量不超过该收货行可退数量，直运场景要求 `sales_return_id` 指向存在且未作废的销售退货单。`PENDING_APPROVAL → POSTED` 要求过账前置校验通过，含结存充足性校验。审批链为空时提交即过账。`POSTED` 为终态，登记有误只能另行登记更正单据。

##### 4.2.5 付款申请

按 PRD 第 4.7.4 小节。`DRAFT → PENDING_APPROVAL` 的守卫条件为供应商状态非 `TERMINATED` 与供应商收款账户非空两条；`payment_type` 取 `INVOICE_PAYMENT` 时另加一条，即申请金额加该发票已占用金额不超过其未核销余额，该类型连同这条守卫按第 4.5 小节整条推迟到阶段 10，因此本阶段受理的付款申请只有 `PREPAYMENT` 一类。`PENDING_APPROVAL → APPROVED` 要求审批链全部节点通过且申请人不在任一节点上。`APPROVED → PARTIALLY_PAID → FULLY_PAID` 由财务的付款登记经 `PaymentRequestWritebackPort` 回写驱动。`APPROVED / PARTIALLY_PAID → CLOSED` 由采购主管填写原因触发。

##### 4.2.6 供应商准入

按 PRD 第 4.8.3 小节。`PENDING → ADMITTED` 的触发点是 mdm 侧供应商档案的生效审批通过事件，`ep-app-procure` 消费该事件并写入准入结论与有效期，这一衔接解决 PRD 第 2.4 节与第 4.8 节的双归属分歧，取值见第 11.2 小节假设 A4。`ADMITTED → SUSPENDED / TERMINATED`、`SUSPENDED → ADMITTED / TERMINATED` 由供应商管理员在应用内触发。

状态对业务的约束在领域层表达为三条守卫：`SUSPENDED` 时禁止新建采购需求指定该供应商、禁止新建与提交采购订单，已下达订单的收货、退货与付款照常；`TERMINATED` 时另禁止新建付款申请并同步把该供应商的全部门户绑定置为 `DISABLED`；资质整体过期时禁止采购订单提交，已下达订单不受影响。

##### 4.2.7 送货通知与发票上传

按 PRD 第 4.9.4 小节与第 4.9.5 小节。送货通知的 `SUBMITTED → VOIDED` 守卫条件为未被任何收货单引用，即 `received_quantity` 全行为零。发票上传的 `UPLOADED → ACCEPTED` 由发票模块经契约回写，`UPLOADED → RETURNED` 由财务填写退回原因触发。

#### 4.3 收货过账算法

收货过账是本阶段最关键的算法，它把采购单据、库存两账与总账凭证三者绑在同一个事务内。步骤如下。

1. 取锁。按 `purchase_order_line_id` 升序对本次涉及的全部采购订单行执行 `SELECT ... FOR UPDATE`，再对涉及的交货批次行与送货通知行按同一升序取锁。固定升序是死锁避免手段，不依赖数据库重试。
2. 前置校验。逐行校验采购订单状态为 `SUPPLIER_CONFIRMED` 或 `PARTIALLY_RECEIVED`；订单未关闭未作废；物料启用批次管理时批次号非空、未启用时取 `'-'`；物料启用序列号管理时序列号条数等于该行数量；引用的送货通知属于同一供应商同一采购订单且未作废；`posting_date` 不晚于登记时点的服务器自然日，取值为 `(now() AT TIME ZONE 'Asia/Shanghai')::date` 的比较。
3. 超收判定。本行实收数量大于该订单行剩余待收数量时置 `has_over_receipt`，按超收转审批开关决定是进入 `PENDING_APPROVAL` 还是继续过账。剩余待收数量的定义为订单行数量减累计收货数量加累计退货数量。采购订单行的订单数量不因超收自动调整。
4. 解析会计期间并调用库存契约取价与写两账。先在事务最前调用 `ep_contract_ledger::AccountingPeriodResolver::resolve` 解析一次会计期间，随后在同一 `&mut dyn Tx` 上调用 `ep_contract_inventory::InventoryPostingPort::post_inbound(tx, ctx, InboundPosting { .. })`，入参为法人、供应商、采购订单、逐行的物料、仓库、批次、序列号、数量与该行固化的 `order_unit_price_untaxed`，以及 `posting_date` 与解析出的会计期间。库存模块按规格第 5.2 章采购收货事件与超量开票的三条结清路径中的路径一判定本次收货中哪一部分走暂估、哪一部分走反向匹配，写入数量账与金额账，返回逐行 `stock_movement_id` 与逐条入账分配。取价由库存模块承担，本阶段不复述借贷与取价。
5. 落入账分配。把第 4 步返回的分配写入 `procure.goods_receipt_line_costings`，只写数量、金额与 `allocation_kind`，不写单价，并断言同一收货行的分配数量合计等于该行数量。
6. 调用总账契约生成凭证。在同一 `&mut dyn Tx` 上调用 `ep_contract_ledger::PostingPort::post(tx, ctx, PostingInput { source_kind: VoucherSourceKind::PURCHASE_RECEIPT, posting_date, source_document, measures })`，measures 取第 4 步返回的暂估金额与反向匹配金额，返回 `voucher_id`。总账只做分录映射与借贷平衡，不提供任何取价方法。数量账、金额账与总账存货腿的取值同源于第 4 步的分配，因此规格第 17.3 章的两账一致与子账总账勾稽在写入时点即成立。
7. 回写。更新采购订单行与交货批次行的 `received_quantity`，推进 `line_status` 与订单 `status`；更新被引用送货通知行的 `received_quantity` 与通知 `status`；把订单的 `is_type_locked` 置为 true。
8. 写审计。经 `ep-platform-audit` 写入 `GOODS_RECEIPT_POSTED` 审计事件，`before` 为空、`after` 为收货单快照，敏感字段按掩码规则处理。
9. 写 Outbox。写入 `procure.goods_receipt.posted.v1`，信封携带 `posting_date` 与 `accounting_period_id`，供关账受理前提枚举。

边界条件：第 4 步与第 6 步任一失败即整个事务回滚，接口返回明确失败并给出 `incident_no`，不写死信，理由是事务未提交因而不存在不一致；PRD 第 4.5.6 小节所称的「库存或财务侧写入不一致」在本设计下只可能由事后对账检出，其处置走死信与人工修复。第 3 步的超收转审批发生在第 4 步之前，因此审批期间不产生任何账务效果。

#### 4.4 采购退货过账算法

1. 取锁。按 `goods_receipt_line_id` 升序对涉及的收货行与其入账分配行取锁。
2. 分支判定。本阶段只实现进项发票未登记分支，`is_invoice_registered` 恒为假。`ep_contract_invoice::ReceiptInvoiceMatchQueryPort::match_state` 的调用点、发票已登记分支的判定与第 6 步的红字发票登记整条推迟到阶段 10，与该端口和 `PurchaseCreditNotePort` 同批交付，本阶段的代码中不出现这两个端口的调用点，也不注入任何替身。推迟窗口内的安全网是一条硬阻断：`invoice` 模块一旦启用，采购退货提交一律返回 `PROCURE.PURCHASE_RETURN.INVOICE_STAGE_PENDING` 并拒绝过账，直到该分支接线；`invoice` 模块由阶段 10 建表与启用，因此本阶段不存在可触发该阻断的数据。该判定任何阶段都由系统作出，用户不可干预。
3. 可退数量校验。本次退货数量不超过该收货行的 `quantity - returned_quantity`；批次与序列号必须为该收货行原登记的取值。
4. 结存充足性前置校验。物料类退货为出库方向，调用 `ep_contract_inventory::AvailabilityQueryPort::on_hand(tx, ctx, legal_entity_id, warehouse_id, material_id, batch_no)` 校验本次出库数量不超过当前结存数量，不满足即阻断提交并返回当前结存数量。这是 PRD 第 5.6.4 小节的提交时校验在采购侧的落点。
5. 调用库存契约取价与写两账（直运场景跳过）。会计期间在事务最前由 `ep_contract_ledger::AccountingPeriodResolver::resolve` 解析一次，随后在同一 `&mut dyn Tx` 上调用 `ep_contract_inventory::InventoryPostingPort::post_outbound(tx, ctx, OutboundPosting { reason: MovementReason::PurchaseReturn, source: SourceRef { doc_type: PURCHASE_RETURN, .. }, lines })`，库存模块按规格第 5.2 章采购退货事件与退货回冲的取价三分支返回逐行回冲单价、回冲金额与 `stock_movement_id`。零结存回退分支所需的「收货单原入账单价」经 `ep_contract_inventory::InventoryPricingLookupPort::original_unit_price_by_source_line` 以被回冲的收货行标识回查，一行退货关联多次收货时逐条回查。直运场景不调用库存契约，按规格第 5.2 章直运订单的退货与成本冲回执行，不产生库存流水。
6. 生成凭证。在同一 `&mut dyn Tx` 上调用 `ep_contract_ledger::PostingPort::post(tx, ctx, PostingInput { source_kind: VoucherSourceKind::PURCHASE_RETURN, posting_date, source_document, measures })`，measures 取第 5 步的回冲金额，返回 `voucher_id`。总账不提供任何取价方法。红字发票金额这一项 measures 与其取数来源随发票已登记分支在阶段 10 一并接入，本阶段既不传该项也不留占位取值。直运场景在本阶段不调用总账契约，理由见本小节边界条件。
7. 回写。更新收货行 `returned_quantity` 与收货单 `status`，更新采购订单行 `returned_quantity`。
8. 写审计与 Outbox，事件 `procure.purchase_return.posted.v1`。
9. 由 job-worker 消费该事件生成一条 `procure.supplier_quality_records`，来源类型 `PURCHASE_RETURN`，退货原因进入质量记录。该步在事务外经 Outbox 完成，理由是质量记录不参与任何守恒判据。

边界条件：直运场景要求 `sales_return_id` 非空且指向未作废的销售退货单，二者逐笔勾稽。本阶段的直运采购退货只登记单据、与销售退货单勾稽并写审计与 Outbox，不产生库存流水，也不产生账务效果；同一笔直接费用类成本的分次冲回与累计冲回金额不超过原归集金额这条上限，随发票已登记分支在阶段 10 补齐，届时由 `PurchaseCreditNotePort::register_credit_note` 在 invoice 模块内判定并在超限时返回业务冲突，`ep-app-procure` 直接透传错误码。供应商不接受退回而不冲回成本的情形，由第 11.2 小节假设 A3 说明其触发点。

#### 4.5 付款申请占用算法

同一张采购发票被多张未关闭付款申请引用时的占用校验必须可串行化。本阶段只交付 `procure.payable_reservations` 的建表迁移与 `ep-domain-procure` 侧的占用不变量，占用的写入路径与下列算法随 `payment_type` 取 `INVOICE_PAYMENT` 的分支在阶段 10 与 `ep_contract_finance::PayableLedgerQuery` 同批交付，本阶段的发布装配不注入该端口的任何替身。算法固定如下，阶段 10 按此实现，本阶段不改其形态。

1. 对本次申请涉及的每一个 `purchase_invoice_id`，在 `procure.payable_reservations` 上执行 `INSERT ... ON CONFLICT (legal_entity_id, purchase_invoice_id) DO UPDATE SET reserved_amount = payable_reservations.reserved_amount + $delta, open_request_count = payable_reservations.open_request_count + 1 RETURNING reserved_amount`。该语句在冲突分支上取得行锁，使同一发票的并发申请串行化。
2. 用返回的 `reserved_amount` 与 `ep_contract_finance::PayableLedgerQuery::open_balance(tx, ctx, purchase_invoice_id)` 返回的未核销余额比较，超出即返回 `PROCURE.PAYMENT_REQUEST.AMOUNT_EXCEEDS_OPEN_BALANCE`，并在 `details` 中列出各发票的未核销余额与已占用金额。本步与其所在分支同批推迟到阶段 10，本阶段既不实现本步，也不以任何更宽的口径替代它，更不注入替身：`invoice.purchase_invoices` 由阶段 10 建表，本阶段不存在可被引用的采购发票。
3. 申请被驳回、撤回、作废、关闭或付清时按同一方式做减法释放，`open_request_count` 归零时保留行不删除，理由是业务 schema 上禁止 `DELETE`。

预付款类型不产生占用，理由是其关联对象是合同或采购订单而不是应付明细。

#### 4.6 采购需求的四类来源

| 来源 | 触发路径 | 幂等键构成 |
|---|---|---|
| 合同派生 | 合同生效派生事件的 Outbox 消费者调用 `PurchaseRequisitionIntakePort::intake` | `CONTRACT:{contract_id}:{contract_line_id}:{contract_version}` |
| 销售订单 | 采购员在应用内经 `actions/raise-from-sales-order-line` 发起 | `SALES_ORDER:{sales_order_line_id}` |
| 项目任务 | 项目模块经同一 `PurchaseRequisitionIntakePort::intake` 调用 | `PROJECT_TASK:{project_task_id}` |
| 库存不足 | job-worker 定时任务扫描后调用同一端口 | `STOCK_SHORTAGE:{warehouse_id}:{material_id}:{scan_date}` |

端口签名固定为 `PurchaseRequisitionIntakePort::intake(tx, ctx, cmd: PurchaseRequisitionIntake)`，`PurchaseRequisitionIntake` 含 `source_module: ModuleCode`、`source_doc_id`、`source_doc_line_id`、`material_id`、`quantity`、`required_on`、`unique_key` 七项，`unique_key` 即上表的幂等键构成，落到 `procure.purchase_requisitions.source_idempotency_key` 列。阶段 12 的 `project.project_task.requisition_requested.v1` 下游同样经该端口，不另起第二个入口。

四类来源共用同一个用例与同一张唯一约束 `ux_purchase_requisitions_legal_entity_id_source_idempotency_key`，重复触发按幂等键返回已有需求，不产生第二条。合同派生失败按规格第 15.2 章进入死信与人工修复并写入审计，修复后重投由同一唯一约束保证不产生重复需求。

库存不足来源的扫描在补货阈值未配置的物料上不生成需求，也不产生告警，实现方式为扫描语句直接跳过阈值为空的组合。

#### 4.7 门户投影与字段白名单

门户五项能力各自的返回字段是显式白名单，白名单在 `ep-domain-portal::rule::field_whitelist` 中以常量表达，不提供通用查询接口。裁剪发生在 `ep-app-portal` 的投影组装阶段，即在 core-server 内完成，portal-gateway 不做字段裁剪，只做呈现层的水印与展示控制。

| 能力 | 返回字段白名单 |
|---|---|
| 采购订单与交期确认 | 订单编号、订单日期、订单状态、行号、物料编码、物料名称、数量、计量单位、不含税单价、税率、约定交期、交货批次行的批次号与批次数量与批次交期。不返回仓库、合同、销售订单、项目、内部备注、内部附件、审批留痕 |
| 送货通知 | 通知编号、状态、关联订单编号与行号、发货数量、批次号、序列号、预计到货日期、承运方、运单号 |
| 发票上传 | 上传编号、状态、发票号码、发票代码、开具日期、关联单据编号、不含税金额、税率、税额、价税合计、退回原因 |
| 收付款对账查询 | 本阶段返回采购订单编号与金额、收货单编号与收货日期与数量两组；已登记采购发票编号与金额、付款记录的付款日期与付款金额、应付未核销余额合计三组随阶段 10 的 `SupplierStatementQuery` 同批加入白名单，本阶段的白名单中不出现这三组字段，也不返回空值占位。不返回账龄分档、成本、毛利、其他供应商、客户与销售侧任何字段 |
| 自身档案维护 | 本供应商的资质、价格、交期三类档案的可维护字段，以及变更申请的编号与审核状态 |

数据范围在 `ep-app-portal` 的每一个用例入口以两条断言表达：请求的目标单据其交易对手供应商等于该门户账号绑定的供应商；请求的 `X-Legal-Entity-Id` 落在该账号的授权法人集合内。两条任一不成立返回 404 与 `PLATFORM.AUTHZ.NOT_FOUND_OR_DENIED`，不区分不存在与无权。行级安全策略仍以 `app.legal_entity_id` 为唯一判据，供应商维度的裁剪由上述断言与投影查询条件承担，不新增第二套行级策略。

---

### 5. API 契约

全部端点遵守基线第 5 节：封套、分页、排序、过滤、幂等键、错误码结构、鉴权头与版本化一律沿用，不新增第二套。下文只列各端点特有的部分。

全部写请求必须带 `Idempotency-Key`，幂等作用域为法人、用户、端点、键值四元组，重复请求且 `request_hash` 相同时返回首次结果并带 `Idempotent-Replay: true`。本阶段不引入任何业务侧的第二套幂等机制，第 4.6 小节的来源幂等键是采购需求这一个对象的去重依据，不替代 `Idempotency-Key`。

#### 5.1 采购需求

| 方法与路径 | 说明 | 权限对象与动作 | 主要错误码 |
|---|---|---|---|
| GET /api/v1/procure/purchase-requisitions | 列表，默认排序 `created_at desc, id desc`，默认筛选期间最近 3 个自然月 | procure.purchase_requisition:read | — |
| GET /api/v1/procure/purchase-requisitions/{id} | 详情 | procure.purchase_requisition:read | PLATFORM.AUTHZ.NOT_FOUND_OR_DENIED |
| POST /api/v1/procure/purchase-requisitions/actions/raise-from-sales-order-line | 采购员按销售订单行发起需求 | procure.purchase_requisition:create | PROCURE.PURCHASE_REQUISITION.SOURCE_LINE_CLOSED |
| POST /api/v1/procure/purchase-requisitions/{id}/actions/close | 手工关闭，必填 `close_reason` | procure.purchase_requisition:close | PROCURE.PURCHASE_REQUISITION.ILLEGAL_STATUS_TRANSITION |

集合级 actions 路径（不带 `{id}`）是本阶段新增的路径约定，基线第 5.1 节只定义了带 `{id}` 的形态与批量形态。理由是采购需求禁止手工新建，因此不能用 `POST /purchase-requisitions` 表达，而该动作又不属于任何已有需求。该约定登记入基线第 5.1 节的修订建议。

请求体示例（`raise-from-sales-order-line`）：`{"sales_order_line_id":"…","suggested_purchase_type":"MATERIAL","required_quantity":"120.000000","expected_arrival_date":"2026-11-20","suggested_supplier_id":null}`。响应 `data` 为需求单视图。

#### 5.2 采购订单

| 方法与路径 | 说明 | 权限动作 | 主要错误码 |
|---|---|---|---|
| POST /api/v1/procure/purchase-orders | 创建草稿，请求体含头与行与交货批次行 | create | PROCURE.PURCHASE_ORDER.SUPPLIER_NOT_ADMITTED、PROCURE.PURCHASE_ORDER.COST_DIMENSION_REQUIRED |
| PATCH /api/v1/procure/purchase-orders/{id} | 草稿或已驳回态的修改，带 `row_version` | update | PLATFORM.CONCURRENCY.STALE_VERSION |
| GET /api/v1/procure/purchase-orders | 列表 | read | — |
| GET /api/v1/procure/purchase-orders/{id} | 详情 | read | — |
| GET /api/v1/procure/purchase-orders/{id}/lines | 行列表 | read | — |
| POST /api/v1/procure/purchase-orders/{id}/actions/submit-for-approval | 提交，审批链为空时直接下达 | submit | PROCURE.PURCHASE_ORDER.SUPPLIER_QUALIFICATION_EXPIRED、PROCURE.PURCHASE_ORDER.BATCH_QUANTITY_MISMATCH、PROCURE.PURCHASE_REQUISITION.ORDERED_QUANTITY_EXCEEDED |
| POST /api/v1/procure/purchase-orders/{id}/actions/revise | 已下达订单的数量、单价、交期、仓库变更 | update | PROCURE.PURCHASE_ORDER.RECEIVED_LINE_NOT_REVISABLE |
| POST /api/v1/procure/purchase-orders/{id}/actions/accept-reschedule | 接受供应商改期 | update | PROCURE.PURCHASE_ORDER.ILLEGAL_STATUS_TRANSITION |
| POST /api/v1/procure/purchase-orders/{id}/actions/reject-reschedule | 拒绝供应商改期 | update | 同上 |
| POST /api/v1/procure/purchase-orders/{id}/actions/close | 提前关闭，必填原因 | close | 同上 |
| POST /api/v1/procure/purchase-orders/{id}/actions/void | 作废 | void | PROCURE.PURCHASE_ORDER.VOID_NOT_ALLOWED |

`submit-for-approval` 是附录 A.1 度量项「采购订单提交」的度量端点，`route` 标签取模板路径。

#### 5.3 收货与拒收

| 方法与路径 | 说明 | 权限动作 | 主要错误码 |
|---|---|---|---|
| POST /api/v1/procure/goods-receipts | 创建收货单草稿，可引用送货通知带出行 | create | PROCURE.GOODS_RECEIPT.ORDER_NOT_RECEIVABLE |
| PATCH /api/v1/procure/goods-receipts/{id} | 草稿修改 | update | PLATFORM.CONCURRENCY.STALE_VERSION |
| POST /api/v1/procure/goods-receipts/{id}/actions/post | 过账，执行第 4.3 小节算法 | post | PROCURE.GOODS_RECEIPT.OVER_RECEIPT_APPROVAL_REQUIRED、PROCURE.GOODS_RECEIPT.SERIAL_NO_DUPLICATED、PROCURE.GOODS_RECEIPT.SERIAL_COUNT_MISMATCH、PROCURE.GOODS_RECEIPT.BATCH_NO_REQUIRED、PROCURE.GOODS_RECEIPT.POSTING_DATE_IN_FUTURE |
| GET /api/v1/procure/goods-receipts | 列表 | read | — |
| GET /api/v1/procure/goods-receipts/{id} | 详情，含入账分配与凭证号 | read | — |
| POST /api/v1/procure/receipt-rejections | 拒收登记 | create | PROCURE.GOODS_RECEIPT.ORDER_NOT_RECEIVABLE |

`actions/post` 是附录 A.1 度量项「入库过账」的度量端点。响应 `data` 含入库单号、入库明细、逐行剩余待收数量与凭证号，与 PRD 第 4.5.3 小节的输出一致。

#### 5.4 采购退货

| 方法与路径 | 说明 | 权限动作 | 主要错误码 |
|---|---|---|---|
| POST /api/v1/procure/purchase-returns | 创建草稿 | create | PROCURE.PURCHASE_RETURN.RECEIPT_NOT_POSTED、PROCURE.PURCHASE_RETURN.SALES_RETURN_LINK_REQUIRED |
| PATCH /api/v1/procure/purchase-returns/{id} | 草稿修改 | update | PLATFORM.CONCURRENCY.STALE_VERSION |
| POST /api/v1/procure/purchase-returns/{id}/actions/submit-for-approval | 提交 | submit | PROCURE.PURCHASE_RETURN.QUANTITY_EXCEEDS_RETURNABLE、PROCURE.PURCHASE_RETURN.BATCH_OR_SERIAL_NOT_IN_RECEIPT |
| POST /api/v1/procure/purchase-returns/{id}/actions/post | 过账，执行第 4.4 小节算法；审批链为空时由 submit 内部直接调用 | post | PROCURE.PURCHASE_RETURN.NEGATIVE_STOCK_BLOCKED |
| POST /api/v1/procure/purchase-returns/{id}/actions/void | 作废未过账单据 | void | PROCURE.PURCHASE_RETURN.ILLEGAL_STATUS_TRANSITION |
| GET /api/v1/procure/purchase-returns 与 /{id} | 列表与详情 | read | — |

`actions/post` 是附录 A.1 度量项「退货登记」在采购侧的度量端点。

#### 5.5 付款申请

| 方法与路径 | 说明 | 权限动作 | 主要错误码 |
|---|---|---|---|
| POST /api/v1/procure/payment-requests | 创建草稿 | create | PROCURE.PAYMENT_REQUEST.SUPPLIER_TERMINATED、PROCURE.PAYMENT_REQUEST.PAYEE_ACCOUNT_MISSING |
| PATCH /api/v1/procure/payment-requests/{id} | 草稿修改 | update | PLATFORM.CONCURRENCY.STALE_VERSION |
| POST /api/v1/procure/payment-requests/{id}/actions/submit-for-approval | 提交并占用 | submit | PROCURE.PAYMENT_REQUEST.AMOUNT_EXCEEDS_OPEN_BALANCE、PROCURE.PAYMENT_REQUEST.DUPLICATE_INVOICE_RESERVATION、PLATFORM.AUTHZ.SEGREGATION_OF_DUTIES |
| POST /api/v1/procure/payment-requests/{id}/actions/withdraw | 产生审批结论前撤回并释放占用 | submit | PROCURE.PAYMENT_REQUEST.ILLEGAL_STATUS_TRANSITION |
| POST /api/v1/procure/payment-requests/{id}/actions/close | 提前关闭并释放剩余占用 | close | 同上 |
| POST /api/v1/procure/payment-requests/{id}/actions/void | 作废 | void | 同上 |
| GET /api/v1/procure/payment-requests 与 /{id} | 列表与详情 | read | — |

`submit-for-approval` 是附录 A.1 度量项「付款申请提交」的度量端点。付款申请的提交与审批不属于规格第 12.1 章的六类高风险操作，因此不要求 `X-Reauth-Token`；付款登记属高风险操作，其重新认证在财务阶段的付款登记端点上执行。

#### 5.6 供应商采购扩展档案

| 方法与路径 | 说明 |
|---|---|
| GET /api/v1/procure/supplier-admissions 与 /{id} | 列表与详情 |
| POST /api/v1/procure/supplier-admissions/{id}/actions/suspend | 暂停，必填原因 |
| POST /api/v1/procure/supplier-admissions/{id}/actions/resume | 恢复 |
| POST /api/v1/procure/supplier-admissions/{id}/actions/terminate | 终止，同步停用门户绑定 |
| GET 与 POST /api/v1/procure/supplier-quality-records | 查询与手工补录 |
| GET 与 POST /api/v1/procure/supplier-risk-records | 查询与登记，取数与写入一律经 `ep_contract_mdm::SupplierRiskRecordPort::list` 与 `::append`，本模块不建风险记录表；处理状态的维护入口在 mdm 侧的供应商档案，本阶段不提供 PATCH 端点 |

准入结论的建立与通过不提供独立端点，由 mdm 供应商档案生效审批的事件驱动，见第 4.2.6 小节。

#### 5.7 门户受控能力 API（core-server）

路径前缀 `/api/v1/portal`。这组端点只接受 `X-Client: portal` 且会话主体为门户账号的请求，其他客户端取值一律返回 403 与 `PORTAL.PORTAL_USER.CAPABILITY_NOT_GRANTED`。

| 方法与路径 | 能力码 | 说明 |
|---|---|---|
| GET /api/v1/portal/purchase-orders | ORDER_CONFIRM | 待确认与全部订单列表，字段按白名单 |
| GET /api/v1/portal/purchase-orders/{id} | ORDER_CONFIRM | 详情 |
| POST /api/v1/portal/purchase-orders/{id}/actions/confirm | ORDER_CONFIRM | 确认订单与交期 |
| POST /api/v1/portal/purchase-orders/{id}/actions/propose-reschedule | ORDER_CONFIRM | 提出改期，请求体只含逐行建议交期与原因 |
| GET 与 POST /api/v1/portal/delivery-notices | DELIVERY_NOTICE | 查询与提交送货通知 |
| POST /api/v1/portal/delivery-notices/{id}/actions/void | DELIVERY_NOTICE | 未被引用时作废 |
| GET 与 POST /api/v1/portal/supplier-invoice-uploads | INVOICE_UPLOAD | 查询与上传发票元数据 |
| GET /api/v1/portal/reconciliation/purchase-orders | SETTLEMENT_QUERY | 对账：采购订单 |
| GET /api/v1/portal/reconciliation/goods-receipts | SETTLEMENT_QUERY | 对账：收货记录 |
| GET /api/v1/portal/reconciliation/purchase-invoices | SETTLEMENT_QUERY | 对账：已登记采购发票，随阶段 10 同批交付，本阶段不注册该路由 |
| GET /api/v1/portal/reconciliation/payments | SETTLEMENT_QUERY | 对账：付款记录，随阶段 10 同批交付，本阶段不注册该路由 |
| GET /api/v1/portal/reconciliation/payable-balance | SETTLEMENT_QUERY | 对账：应付未核销余额合计，随阶段 10 同批交付，本阶段不注册该路由 |
| GET /api/v1/portal/supplier-profile | PROFILE_MAINTAIN | 自身资质、价格、交期档案 |
| POST /api/v1/portal/supplier-profile/actions/submit-change | PROFILE_MAINTAIN | 提交档案变更，经 `ep_contract_mdm::SupplierSelfServiceCommand::submit_profile_change` 生成待审批变更申请，返回申请编号回执 |

五个对账端点的取数来源固定如下：采购订单与收货记录取本模块自有表，本阶段交付；已登记采购发票、付款记录与应付未核销余额合计一律经 `ep_contract_finance::SupplierStatementQuery::statement` 与 `ep_contract_finance::PayableLedgerQuery::open_balance` 取数，两个端口由阶段 10 交付，因此这三个端点连同其路由、投影与白名单字段随两个端口在阶段 10 同批交付，本阶段既不注册路由也不注入替身，不建第二套余额口径，也不直读 invoice 与 finance 两个 schema。资质文件的上传经 `SupplierSelfServiceCommand::upload_qualification`。

门户端点的分页参数与内部一致，`page_size` 上限收窄为 50，理由是门户在附录 A.1 沿用常规交互通过线而其取数经一次内部转发，收窄上限是保住该通过线的手段。该收窄是本阶段新增决定。

门户端点不提供任何导出入口，因此规格第 12.4 章浏览器门户端的导出审批项以「无导出入口」满足，该判定须由安全负责人在发布前确认，列入第 11 节的阻塞项。

#### 5.8 门户站点 API（portal-gateway）

路径前缀 `/portal/v1`，与第 5.7 小节一一对应，另加三个会话端点。

| 方法与路径 | 说明 |
|---|---|
| POST /portal/v1/session/login | 转发到 core-server 的门户身份认证入口，成功后下发仅 HttpOnly、Secure、SameSite=Strict 的会话 cookie |
| POST /portal/v1/session/logout | 注销 |
| GET /portal/v1/session/current | 当前账号、绑定供应商与授权法人集合 |

portal-gateway 在每个请求上做四件事：会话校验（结果按第 7 节的配置缓存不超过 30 秒）、按账号与按来源地址的限流、把会话解析出的门户主体与法人写入到 core-server 请求的 `Authorization`、`X-Legal-Entity-Id`、`X-Device-Id`、`X-Client: portal` 四个头、在响应上叠加水印所需的呈现层参数。门户请求在 portal-gateway 新建 trace，不接受外部传入的 `traceparent`，公网侧关联标识放入 `X-Correlation-Id`，按基线第 9.3 节。

---

### 6. 并发与事务边界

#### 6.1 事务边界总表

| 用例 | 事务内包含 | 事务外经 Outbox |
|---|---|---|
| 采购需求生成 | 需求单写入、幂等键唯一约束、审计、Outbox 条目 | 站内通知、检索索引 |
| 采购订单提交 | 订单头与行与批次行写入、需求累计下达数量回写、流程实例启动、审计、Outbox | 站内通知、门户待确认投影刷新、检索索引 |
| 收货过账 | 采购单据写入、会计期间解析、库存数量账与金额账写入与取价、入账分配写入、总账凭证生成、订单行与批次行与送货通知回写、审计、Outbox | 站内通知、门户投影刷新、检索索引 |
| 采购退货过账 | 退货单写入、库存两账写入与取价、总账凭证生成、收货行与订单行回写、审计、Outbox | 供应商质量记录生成、站内通知、门户投影刷新 |
| 付款申请提交 | 申请头与行写入、流程实例启动、审计、Outbox | 站内通知 |
| 门户订单确认与改期 | 订单状态与交期更新、审计、Outbox | 站内通知给采购主管、门户投影刷新 |
| 门户送货通知提交 | 通知头与行写入、审计、Outbox | 站内通知给仓管员、待收货列表投影刷新 |
| 门户发票上传 | 上传记录写入、附件元数据关联、审计、Outbox | 站内通知给财务、待登记队列投影刷新 |
| 门户档案变更提交 | 经 `SupplierSelfServiceCommand::submit_profile_change` 在同一事务内创建变更申请、审计、Outbox | 站内通知给采购负责人 |

每个用例一个事务，一个 HTTP 请求内不开启第二个写事务。事务内禁止外部 HTTP 调用、文件正文读写、通知发送与长时计算。附件正文的上传经 `platform_file` 的上传流水线在业务事务之外完成，业务事务只写附件关联表中的元数据引用。

#### 6.2 隔离级别与超时

业务事务一律 `READ COMMITTED`。本阶段不引入 `REPEATABLE READ` 事务，第 8.6 小节的六个 `ReconCheck` 由 `ep-platform-recon` 的执行器在其提供的快照上执行，快照上下文类型为 `ep_foundation::port::SnapshotCtx`。事务预算沿用基线第 10.3 节：业务事务不超过 5 秒，读写池 `statement_timeout` 10 秒，`lock_timeout` 3 秒，`idle_in_transaction_session_timeout` 15 秒。

#### 6.3 锁策略

| 场景 | 锁 | 顺序 |
|---|---|---|
| 收货过账 | 采购订单行、交货批次行、送货通知行的 `FOR UPDATE` | 按各自主键升序，先订单行、后批次行、再通知行 |
| 采购退货过账 | 收货行、其入账分配行、采购订单行的 `FOR UPDATE` | 按主键升序，先收货行、后分配行、再订单行 |
| 付款申请提交与释放 | `payable_reservations` 的行锁，经 `INSERT ... ON CONFLICT DO UPDATE` 取得 | 按 `purchase_invoice_id` 升序 |
| 采购需求累计下达 | 需求行 `FOR UPDATE` | 按主键升序 |
| 门户订单确认与本方改期处理 | 订单头 `FOR UPDATE` | 单行 |

固定升序取锁是死锁避免的第一手段。序列化失败 40001 与死锁 40P01 由数据访问层重试 3 次，退避 50、150、450 毫秒，只对尚未产生任何外部可见副作用的事务重试，重试次数进 `ep_db_tx_retries_total`，标签为 pool 与 sqlstate，该指标由阶段 2 注册与填充。

#### 6.4 乐观锁

采购需求、采购订单及其行与批次行、收货单、采购退货单、付款申请、供应商准入、门户绑定、送货通知、发票上传均带 `row_version`，更新一律按基线第 3.7 节的写法，受影响行数为 0 即返回 409 与 `PLATFORM.CONCURRENCY.STALE_VERSION` 并回带当前版本号与最后修改人。`procure.goods_receipt_line_costings` 为仅追加表，不带 `row_version`。

#### 6.5 幂等与 Outbox

写请求的 `Idempotency-Key` 与业务写入同事务，存储在 `platform_msg.idempotency_keys`，保留 7 天。本阶段的全部领域事件与业务状态、审计事件写入同一事务，事务提交前不发起任何外部调用。消费端幂等由 `platform_msg.inbox_consumptions` 的唯一约束保证，消费副作用与该行插入同事务。

本阶段的 Outbox 事件信封一律携带 `posting_date` 与 `accounting_period_id` 两个字段：收货过账与采购退货过账事件取其单据上的实际取值，其余事件取空值。`procure.goods_receipt.posted.v1` 与 `procure.purchase_return.posted.v1` 两个事件在 `ledger.posting_trigger_event_types` 中的登记行按裁定 A-21 由阶段 9a 的种子迁移一次写入，本阶段不新增回填迁移。该登记行按裁定 A-21 与总览第 1.5 节第三条每行只填 `event_type`，原有的 `ledger_event_kind` 与 `registered_by_module` 两列已删除，本阶段不得再引用；`ep_contract_ledger::PostingTriggerRegistry::assert_registered` 与错误码 `LEDGER.POSTING_TRIGGER_EVENT_TYPE.REGISTRY_MISMATCH` 整项撤销，本阶段不做启动自检、不做 `--check` 静态断言，也不向阶段 9b 的关账受理追加任何前置校验，关账受理前提仍为两条。登记表一致性的承接方只有两条：`xtask configdoc` 从 `docs/event-catalog.md` 生成阶段 9a 的第 14 号种子迁移并在 CI 中逐字比对，以及阶段 3b 的 `event-catalog-consistent` 自检项，该项取 Degrading 且不通过时停止派发未登记事件类型。本模块的 14 个事件与 `docs/event-catalog.md` 逐字比对通过即为达标。两个事件在 PENDING 或 DISPATCHING 状态下进入关账受理前提二的统计，`posting_date` 为空的其余事件不计入该统计。

#### 6.6 失败重试与补偿

| 失败点 | 处置 |
|---|---|
| 总账或库存契约在同一事务内返回错误 | 整个事务回滚，接口返回明确失败与 `incident_no`，不产生死信，不产生部分写入 |
| 合同派生采购需求的 Outbox 消费失败 | 按基线第 6.2 节的 8 次退避重投，全部失败置为 `DEAD` 并写入死信；死信按 `legal_entity_id` 与 `posting_date` 可枚举 |
| 供应商质量记录生成失败 | 同上，重投；该失败不影响退货本身的账务效果 |
| 门户投影与检索索引刷新失败 | 同上，重投；超过规格第 7.9 章的 15 分钟传播窗口按该章告警并转人工 |
| 站内通知投递失败 | 同上，重投；不改变任何业务状态 |
| 事后对账检出采购侧与库存或财务侧不一致 | 由 `ep-platform-recon` 的执行器按本阶段实现的 `ReconCheck` 生成对账差异事项，按规格第 15.2 章进入死信与人工修复，并按规格第 10.2 章拦截关账 |

本阶段不引入补偿事务。收货与退货的多腿写入在同一数据库事务内，因此不存在需要逆序补偿的部分成功状态；门户与内部之间的写入也不跨事务，门户的写请求在 core-server 内是一个完整事务。

#### 6.7 必测并发场景

基线第 8.4 节的六组必测场景中，本阶段承担第一组与第六组的采购侧实例，并新增四组本阶段特有的场景。

1. 同一采购订单的乐观锁冲突：两个采购员并发 `revise` 同一订单。
2. 同一采购订单行的并发收货：两个仓管员并发过账，累计收货数量不得超过订单数量加超收放行量，且两次过账各自产生一张凭证与一组库存流水。
3. 同一送货通知的并发引用：两张收货单并发引用同一通知行，累计引用数量不得超过通知数量。
4. 同一采购发票被两张付款申请并发占用：该场景随付款申请的 `INVOICE_PAYMENT` 分支在阶段 10 执行，判据不变，即占用合计不得超过未核销余额、被拒的一方返回 `PROCURE.PAYMENT_REQUEST.AMOUNT_EXCEEDS_OPEN_BALANCE`。
5. 同一收货行的并发退货：累计退货数量不得超过收货数量，且不得出现负结存。
6. `procure.goods_receipt.posted.v1` 的重复投递不少于 3 次：业务效果、外发事件与审计记录只允许产生一次。

---

### 7. 配置项

本阶段新增的配置项全部走基线第 7.1 节的五层来源与优先级，结构体开启 `deny_unknown_fields`，未知键拒绝启动。运行期可变的业务参数不进配置文件，改存事务数据库并经配置发布通道签名发布，下文分两组列出。配置发布通道由阶段 3b 交付，本阶段的业务参数一律经该通道发布，不自建第二套。

#### 7.1 进程配置（`/etc/ep/config.toml` 与环境变量）

| 键名 | 类型 | 默认值 | 承载进程 | 生效方式 |
|---|---|---|---|---|
| EP__PORTAL__SESSION__MAX_AGE_SECONDS | u32 | 7200 | portal-gateway | 重启生效 |
| EP__PORTAL__SESSION__IDLE_TIMEOUT_SECONDS | u32 | 900 | portal-gateway | 重启生效 |
| EP__PORTAL__SESSION__VALIDATION_CACHE_TTL_SECONDS | u32 | 30 | portal-gateway | 重启生效 |
| EP__PORTAL__RATE_LIMIT__REQUESTS_PER_MINUTE | u32 | 120 | portal-gateway | 重启生效 |
| EP__PORTAL__RATE_LIMIT__BURST | u32 | 40 | portal-gateway | 重启生效 |
| EP__PORTAL__CORE_API__BASE_URL | String | http://127.0.0.1:8080 | portal-gateway | 重启生效 |
| EP__PORTAL__CORE_API__TIMEOUT_MS | u32 | 8000 | portal-gateway | 重启生效 |
| EP__PORTAL__UPLOAD__MAX_ATTACHMENT_BYTES | u64 | 52428800 | portal-gateway 与 core-server | 重启生效 |
| EP__PORTAL__SELF_REGISTRATION__ENABLED | bool | false | portal-gateway 与 core-server | 重启生效 |
| EP__PORTAL__WATERMARK__ENABLED | bool | true | portal-gateway | 重启生效 |
| EP__PROCURE__RECEIPT__MAX_LINES | u16 | 200 | core-server | 重启生效 |
| EP__PROCURE__RETURN__MAX_LINES | u16 | 200 | core-server | 重启生效 |
| EP__PROCURE__REQUISITION__STOCK_SHORTAGE_SCAN_ENABLED | bool | false | job-worker | 重启生效 |
| EP__PROCURE__REQUISITION__STOCK_SHORTAGE_SCAN_INTERVAL_MINUTES | u32 | 60 | job-worker | 重启生效 |

会话有效期 2 小时与空闲 15 分钟低于基线第 11.6 节对内部会话的 8 小时与 30 分钟，理由是门户是公网暴露面，规格第 21.17 章把该暴露面登记为风险。该差异是本阶段新增决定，回写基线第 11.6 节。

会话校验缓存 30 秒意味着门户账号被停用后最多 30 秒仍可访问。门户五项能力均不属于规格第 12.1 章的六类高风险操作，因此该延迟不违反高风险操作即时撤销的要求。该判断是本阶段新增决定。

单附件上限 50 MB 低于规格第 6.5 章的 5 GB 默认上限，只作用于门户上传通道，理由同上。内部通道不受影响。

`EP__PORTAL__SELF_REGISTRATION__ENABLED` 默认关闭。受限自助注册在代码上完整实现（注册频率限制、邀请码校验、待审核账号只能访问自身注册状态三项），但默认不开放，开放条件是 U-F-11 的决策落地。

#### 7.2 业务参数（存库，经配置发布通道发布）

| 参数键 | 类型 | 出厂默认 | 说明 |
|---|---|---|---|
| procure.receipt.over_receipt_tolerance_ratio | Rate | 0.000000 | 超收容差比例，对应 U-F-04 |
| procure.receipt.over_receipt_requires_approval | bool | true | 超出容差是否转审批，对应 U-F-04 |
| procure.order.reschedule_round_limit | u16 | 0 | 改期协商轮次上限，0 表示不限，对应 U-F-12 |
| procure.order.block_on_qualification_expired | bool | true | 资质整体过期是否阻断下单，对应 U-F-14 |
| procure.order.requisition_required_for_material | bool | true | 物料类订单是否必须关联需求，对应 U-F-01 |
| procure.order.requisition_required_for_direct_expense | bool | false | 直接费用类订单是否必须关联需求，对应 U-F-01 |
| procure.return.approval_required | bool | true | 采购退货是否需要审批链，对应 U-F-05 |
| portal.settlement.show_aging | bool | false | 门户对账是否展示应付账龄，对应 U-F-13 |

八个参数的出厂默认值是本阶段的临时取值，业务决策落地后按配置发布通道改值即可，不需要改代码、改表结构或改迁移，切换代价为一次配置发布。

---

### 8. 测试计划

#### 8.1 单元测试

位于 `ep-domain-procure` 与 `ep-domain-portal` 内，不触网、不触库、不触文件系统、不取真实时间，`Clock` 一律注入 `FixedClock`。覆盖的分支如下。

采购需求：四类来源各自的生成守卫；来源单据行已关闭的拒绝；同一 `source_idempotency_key` 的去重；累计下达数量三档状态迁移的边界（等于零、介于之间、等于需求数量）；手工关闭与来源作废触发关闭两条路径；直运来源强制 `DIRECT_EXPENSE` 且不允许改为物料类。

采购订单：物料类与直接费用类各自的必填字段矩阵；交货批次行数量合计等于行数量的断言，含合计大于与合计小于两个反例；约定交期不早于订单日期；累计下达数量不超过需求数量；十一态状态机的全部合法迁移与至少同等数量的非法迁移拒绝；作废三条前置条件的四种组合；已收货行不可变更数量与单价；供应商四态对下单的四种约束。

收货单：超收、短收、平收三分支；批次管理与序列号管理开启与关闭的四种组合；序列号条数等于数量的断言；`posting_date` 晚于服务器自然日的拒绝；入账分配数量合计等于行数量的断言，含一条分配与两条分配两种形态；过账后不可修改。

采购退货：可退数量计算（收货数量减累计退货数量）的边界；物料类与直运两个场景的必填字段矩阵；批次与序列号必须来自原收货行；一行退货关联多次收货时按各自分配逐条取价的入参构造；累计冲回不超过原归集金额的断言。

付款申请：发票付款与预付款两类型的必填矩阵；占用金额加已占用不超过未核销余额的边界；九态状态机；已付金额不超过申请金额的断言；撤回、驳回、作废、关闭四条释放路径。

门户：五项能力码的白名单裁剪，逐能力逐字段断言不出现禁止字段；数据范围两条断言的四种组合（供应商匹配与否、法人在授权集合与否）；送货通知累计数量不超过订单行剩余待收数量；送货通知被引用后不可作废；发票上传的价税合计等式与同号重复判定。

领域属性测试（proptest）覆盖本阶段承担的四组不变量，对应规格第 17.3 章：

1. 采购订单行的累计收货数量减累计退货数量在任意合法操作序列后不小于零，且不超过订单数量加放行的超收量。
2. 收货行的累计退货数量在任意合法操作序列后不超过该行收货数量。
3. 收货行入账分配的数量合计在任意合法操作序列后恒等于该行收货数量。
4. 同一采购发票的占用金额合计在任意申请提交与释放序列后恒等于当前未关闭申请行的金额合计，且不小于零。

#### 8.2 集成测试

位于各 crate 的 `tests/` 与 `apps/core-server/tests/`、`apps/portal-gateway/tests/`。使用真实 PostgreSQL 16，每个用例独占一个 `ep_test_<nanoid>` 数据库，用例结束即删库。不使用内存库或 mock 替代数据库。

场景清单：

1. 迁移正向执行与按 `-- rollback:` 段回退，三十一个文件逐个验证，其中第 24 号登记回填文件验证正向插入与回退删除两个方向，并断言 `db/checks/append_only_consistency.sql` 返回零行。
2. 三十张表的行级安全策略生效：变量缺失时不可见不可写；跨法人上下文读、写、更新、聚合、排序六类操作均不返回也不写入他法人数据。
3. 采购需求四类来源的端到端生成，含合同派生事件的重复投递 3 次只产生一条需求。
4. 采购订单从草稿到下达到供应商确认的全链路，含审批链为空与非空两种配置。
5. 分批订货两种形态：需求侧分多次下达为多张订单，累计不超过需求数量；订单侧一行多个交货批次，按批次逐次收货。
6. 收货过账的同事务性验证：注入总账契约失败与库存契约失败两种故障，断言采购单据、入账分配、库存流水、凭证四处均无写入，且 `platform_msg.outbox_events` 无新增条目。
7. 收货过账对采购订单行、交货批次行、送货通知行三处累计数量的回写正确性。
8. 超收三分支：容差内直接过账；超出容差转审批后过账；拒收只登记记录不产生库存流水与凭证。
9. 采购退货的发票未登记分支：按原暂估单价原额冲回，取价由库存契约返回，本阶段断言入参完整且返回值被原样用于凭证 measures 与单据回写。发票已登记分支及其两条取价路径按第 4.4 小节随阶段 10 交付，本阶段不构造该分支的数据。
10. 一行退货关联多次收货：三次收货各自单价不同，一次退货跨三条分配，断言逐条取价与逐条回冲。
11. 直运采购退货：不产生库存流水，与销售退货单逐笔勾稽，分次冲回累计不超过原归集金额。
12. 付款申请的占用并发：该场景随付款申请的 `INVOICE_PAYMENT` 分支在阶段 10 执行，本阶段不以任何桩替代余额取数。
13. 付款申请状态随付款登记回写的全链路，含分次付款与提前关闭：本阶段由集成测试直接调用本模块自有的 `PaymentRequestWritebackPort` 驱动，不依赖财务模块，也不顺延。
14. 供应商四态对采购需求、采购订单、付款申请、收货、退货五类操作的约束矩阵，共二十格逐格断言。
15. 门户五项能力的受控访问：以 A 供应商的门户账号访问 B 供应商的订单、收货、发票、付款四类对象，一律返回 404 与 `PLATFORM.AUTHZ.NOT_FOUND_OR_DENIED`；以门户账号访问任一内部端点一律被拒。
16. 门户返回字段白名单：对五项能力的响应做全字段快照（insta），任何新增字段导致快照失败，防止字段泄漏被静默引入。
17. 门户订单确认与改期的完整协商回合，含本方接受与本方拒绝两条路径与订单进入收货流程后拒绝门户操作。
18. 门户送货通知从提交、被部分引用、被全部引用到不可作废的全链路。
19. 门户发票上传的重复号码拒绝、退回后重传、受理后置终态三条路径；第三条路径在本阶段只断言 `ACCEPTED` 终态下的门户查询裁剪与状态取值，其终态由集成测试直接写入 `portal.supplier_invoice_uploads` 构造，受理回写本身随 E2E-T-03 的被受理一路在阶段 10 交付，本阶段不写受理入口，也不注入任何替身。
20. 门户对账查询的取数与内部应付台账同源：该场景随三个门户对账端点与 `SupplierStatementQuery` 在阶段 10 同批执行；本阶段执行的是门户对账查询在采购订单与收货两组字段上的取数与裁剪。
21. portal-gateway 的会话、限流与转发：限流触发返回 429 并记入运维中心，未登记设备与非门户客户端取值被拒。
22. 幂等：本阶段全部写端点各执行一次重复提交，断言返回首次结果并带 `Idempotent-Replay: true`；键相同而载荷不同时返回 409。

外部电子签章不在本阶段范围内，本阶段不引入任何 wiremock 打桩。

第 9 项、第 12 项与第 20 项分别依赖阶段 10 的 `ReceiptInvoiceMatchQueryPort` 与 `PurchaseCreditNotePort`、`PayableLedgerQuery`、`SupplierStatementQuery`。三处一律不接替身，也不登记任何顺延验收：第 9 项在本阶段只执行发票未登记分支，已登记分支的用例代码与断言随两个端口在阶段 10 同批交付；第 12 项整条推迟到阶段 10；第 20 项随三个门户对账端点在阶段 10 同批交付。本阶段执行第 8.2 小节二十二个场景中的二十个，第 12 项与第 20 项不在其内，两项在第 9 节退出条件第 25 条逐条列名。

#### 8.3 端到端测试

后端 E2E 用 Rust 集成测试直接打 HTTP 接口，桌面端用 Playwright 驱动 WebView 与 tauri-driver，门户 Web 用 Playwright 驱动浏览器，移动端按规格第 6.2 章矩阵中「采购与供应商协同」一行取值为简化的范围执行 XCUITest 与 Espresso。本阶段的界面代码落在 `clients/desktop/src/modules/procure/` 与 `clients/mobile/src/modules/procure/`，门户站点由 portal-gateway 承载，三处均由上述用例覆盖。

用例清单：

- E2E-P-01 采购订货：从审批生效合同派生的采购需求出发，下达采购订单，门户确认交期，覆盖规格第 8 章第 4 步。
- E2E-P-02 分批订货：需求侧分两次下达，订单侧一行分三个交货批次，逐批次收货，对应规格第 17.2 章基础分支中的分批订货。
- E2E-P-03 收货暂估：按采购订单不含税单价暂估入库，断言收货单输出的凭证号可查、入账分配为 `ESTIMATED`、库存两账同源。发票侧的回冲与价差在财务阶段联测。
- E2E-P-04 发票数量少于收货数量的收货侧：一次收货登记部分数量的发票后再次收货，断言未匹配部分的暂估继续留存。本阶段断言收货侧的入账分配与订单行累计数量，财务侧判据在财务阶段。
- E2E-P-05 一张发票跨多次收货的收货侧：三次收货形成三条入账分配，供发票侧逐次回冲。
- E2E-P-06 未收票采购退货：收货后未登记发票即退货，断言分支判定为「发票未登记」、按原暂估单价原额冲回、库存两账同步减少。
- E2E-P-07 超量开票路径一的收货侧：在存在待处理超量开票余额的采购订单上登记收货，断言入账分配中出现 `OVERBILL_REVERSE_MATCH` 类型且该部分不再按订单单价挂暂估。
- E2E-P-08 直接费用类采购：创建直接费用类采购订单，断言不产生收货入口、三个归集字段必填其一、订单在发票登记完毕后进入已完成。
- E2E-P-09 直运订单闭环的采购侧：直运订单派生的采购需求强制为直接费用类且不可改为物料类，全程不产生库存数量流水。
- E2E-P-10 直运订单退货：登记与销售退货单勾稽的采购退货，断言不产生库存流水、勾稽双向可达；直接费用类成本的分次冲回与其累计上限判定随发票已登记分支在阶段 10 补齐。
- E2E-P-11 付款申请与审批：提交预付款类付款申请，申请人不可自审，审批链不可越权跳过，审批通过后进入财务待付款队列，覆盖规格第 8 章第 10 步的申请与审批部分；发票付款类的申请随第 4.5 小节的分支在阶段 10 补齐。
- E2E-P-12 供应商准入到停用的全生命周期：档案生效审批置为已准入，暂停后禁止新建订单但已下达订单可继续收货与付款，终止后门户绑定同步停用。
- E2E-T-01 门户采购订单与交期确认闭环。
- E2E-T-02 门户送货通知闭环，含被收货引用与作废两条路径。
- E2E-T-03 门户发票上传闭环：本阶段执行上传与被退回两条路径，被受理一路整条推迟到阶段 10，即 `UPLOADED → ACCEPTED` 的回写与 `accepted_purchase_invoice_id` 的落值随承接该回写的端口在阶段 10 同批交付；本阶段按本文件对未交付端口的既有纪律不注册受理入口，也不注入任何替身。
- E2E-T-04 门户收付款对账查询闭环：本阶段覆盖采购订单与收货两组字段的查询与裁剪，与内部台账同源的判定随三个对账端点在阶段 10 补齐。

E2E-T-01 至 E2E-T-04 逐条对应规格第 19 章阶段 3 门户条目的四项闭环用例。

#### 8.4 数据保护控制测试

按规格第 17.2 章数据保护控制测试的浏览器门户端条目执行四项：脱敏投影按第 8.2 小节第 16 项的字段白名单快照验证；水印在门户全部页面上可见且含账号与时间；导出审批以「无导出入口」判定通过并记录该判定；操作审计逐项验证门户五项能力的全部写操作与读操作均产生 `platform_audit.audit_events` 记录且哈希链连续。

#### 8.5 法人越权测试

`tests/rls_matrix` 中新增本阶段的三十张表，覆盖读取、写入、更新、删除、聚合、排序、报表投影与错误信息泄漏八类，另新增门户维度的两类：以门户账号跨供应商访问，以门户账号跨授权法人访问。该测试目标属发布门禁项。

#### 8.6 对账与不变量校验

本阶段在 `ep-app-procure` 实现六个 `ep_platform_recon::ReconCheck`，并按裁定 A-06 全部在 `apps/job-worker/src/wiring/` 目录下经 `ReconRegistry::register` 注册，由 ep-platform-recon 的执行器按法人逐轮遍历、在其提供的快照上分批执行，差额非零即生成对账差异事项并按规格第 10.2 章拦截关账。裁定 A-06 固定的四个注册方十五个校验项中，本阶段承担六个。六个 check 的 `code()` 取值即下表编号，`blocks_period_close()` 一律为真，`category()` 一律取 `ReconCategory::Invariant`，落库文本为 `INVARIANT`：本阶段的六条判据全部是金额与数量守恒判据，跨模块逻辑引用的存在性不在对账框架内核对，见第 3.1 小节。

| 编号 | 判据 |
|---|---|
| R-PROC-01 | 采购订单行的 `received_quantity` 等于该行全部已过账收货行的数量合计 |
| R-PROC-02 | 收货行的 `returned_quantity` 等于关联该行的全部已过账退货行数量合计，且不超过该行 `quantity` |
| R-PROC-03 | 采购需求的 `ordered_quantity` 等于关联采购订单行的数量合计，且不超过 `required_quantity` |
| R-PROC-04 | `payable_reservations.reserved_amount` 等于该发票被未关闭付款申请行占用的金额合计；付款申请的 `paid_amount` 不超过 `requested_amount` |
| R-PROC-05 | 收货行的入账分配数量合计等于该行 `quantity`，且分配金额合计等于逐条 `quantity` 乘以经 `InventoryPricingLookupPort::original_unit_price_by_source_line` 回查的单价按 2 位 round 后的合计 |
| R-PORT-01 | 送货通知行的 `received_quantity` 等于引用该行的已过账收货行数量合计，且不超过通知行 `quantity` |

上述六条是本阶段自有的采购侧守恒判据。规格第 17.3 章的库存数量守恒、两账一致与子账总账勾稽由库存与总账两阶段的语句承担，本阶段不重复定义，但 E2E 用例在其可用时一并执行。

#### 8.7 性能相关项

本阶段对应附录 A.1 的八个度量项，各自的度量端点在第 5 节已列出。

| 度量项 | 端点 | 通过线 |
|---|---|---|
| 采购订单提交 | POST /api/v1/procure/purchase-orders/{id}/actions/submit-for-approval | 普通交易提交 P95 在 3 秒内 |
| 入库过账 | POST /api/v1/procure/goods-receipts/{id}/actions/post | 同上 |
| 付款申请提交 | POST /api/v1/procure/payment-requests/{id}/actions/submit-for-approval | 同上 |
| 退货登记（采购侧） | POST /api/v1/procure/purchase-returns/{id}/actions/post | 同上 |
| 采购订单与交期待确认列表加载 | GET /portal/v1/purchase-orders | 常规交互 P95 在 2 秒内，门户口径从反向代理门户入口计起 |
| 收付款对账查询 | GET /portal/v1/reconciliation/purchase-orders 与 /goods-receipts | 同上；另三个对账端点的度量随其在阶段 10 交付 |
| 门户首页加载、供应商自身档案查看与维护 | GET /portal/v1/session/current 与 /portal/v1/supplier-profile | 同上 |
| 采购订单与交期确认、送货通知提交、发票上传 | 三个门户提交端点 | 普通交易提交 P95 在 3 秒内；发票上传的通过线只覆盖到提交回执可见 |

本阶段的性能责任是提交 `EXPLAIN` 证据：附录 A.1 清单内的上述查询在附录 A.3 基准数据集上不得出现顺序扫描。时延通过线的正式判定在阶段 4 统一执行，本阶段不冻结取值。

#### 8.8 覆盖率门槛

| 范围 | 行覆盖率下限 |
|---|---|
| `ep-domain-procure` 的 `rule/` 与 `ep-domain-portal` 的 `rule/`，以及第 8.6 小节六条判据的实现代码 | 85% |
| `ep-domain-procure`、`ep-domain-portal` 其余部分与 `ep-app-procure`、`ep-app-portal` | 80% |
| `ep-contract-procure`、`ep-contract-portal`、`ep-adapter-db-pg` 的两个仓储目录、`apps/portal-gateway` 新增部分 | 70% |
| 本阶段新增与修改代码整体 | 80% |
| 工作区整体 | 不低于 80%，本阶段合入后不得下降 |

工具为 cargo-llvm-cov，阈值由 `codecov.toml` 的路径规则表达，路径规则与本阶段的 crate 清单一一对应。`#[ignore]` 必须带 issue 编号且存活不超过本阶段。

---

### 9. 退出条件

下列条目全部达成才算本阶段完成，每条可客观判定。

1. 三十一个迁移文件（三十个建表文件与第 24 号 `append_only_registry` 登记回填文件）在空库上按文件版本号全序执行成功，并按其 `-- rollback:` 段逆向回退成功，回退后 `procure` 与 `portal` 两个 schema 无残留对象，`platform_core.append_only_registry` 中无本阶段残留登记行。
2. 三十张表全部 `ENABLE` 且 `FORCE` 行级安全，策略名与基线模板一致，运行期账号不具备 `BYPASSRLS` 与 `SUPERUSER`。`--check` 模式的 `rls-enabled-and-forced` 自检项对这三十张表通过。
3. `tests/rls_matrix` 的十类断言在本阶段三十张表上全部通过，含门户跨供应商与跨授权法人两类。
4. 第 5 节列出的端点中，除第 5.7 小节标注随阶段 10 交付的三个对账端点外全部可用，逐个端点的封套、分页、排序白名单、过滤算子、幂等语义与错误码与基线一致，由一组契约测试逐端点断言。
5. 第 8.2 小节的二十二个集成场景中本阶段执行二十个并全部通过，第 12 项与第 20 项随阶段 10 的对应端口同批执行，两项在第 25 条列名。
6. 第 8.3 小节的十六个 E2E 用例全部通过，其中 E2E-T-01 至 E2E-T-04 对应规格第 19 章阶段 3 门户条目的四项闭环用例；E2E-T-03 在本阶段只执行上传与被退回两条路径，其被受理一路随阶段 10 的受理回写同批执行，该路径在第 25 条列名。
7. 第 8.1 小节的四组领域属性测试各运行不少于 1000 个用例且无反例。
8. 第 6.7 小节的六个并发场景中本阶段执行五个并全部通过，第 4 项随付款申请的 `INVOICE_PAYMENT` 分支在阶段 10 执行；`procure.goods_receipt.posted.v1` 的重复投递 3 次业务效果、外发事件与审计记录各只产生一次。
9. 第 8.6 小节的六个 `ReconCheck` 已在 `ep-app-procure` 实现并在 `apps/job-worker/src/wiring/` 目录下经 `ReconRegistry::register` 注册，注入任一差额后对账差异事项生成且关账请求被拒绝，差额清零后关账可通过。
10. 第 8.4 小节的四项数据保护控制测试通过，其中导出审批以「无导出入口」判定并已取得安全负责人的书面确认。
11. 第 8.8 小节的覆盖率门槛全部达标，`cargo llvm-cov --fail-under-lines` 在 CI 上通过。
12. 依赖方向自检脚本通过：`ep-domain-procure` 与 `ep-domain-portal` 不出现 sqlx、reqwest、tokio 的 IO 模块、`std::fs`、`std::net`、`SystemTime::now`、`rand` 六类符号；`ep-app-procure` 与 `ep-app-portal` 之间无相互依赖；除 `apps/*/src/wiring/` 目录外无 `use ep_adapter_db_pg::` 出现。
13. 文件规模纪律通过：本阶段新增文件无一超过 800 行，函数无一超过 50 行，嵌套深度无一超过 4 层。
14. `docs/event-catalog.md` 已登记本阶段 14 个事件类型，`docs/error-codes.md` 已登记 35 个 PROCURE 与 PORTAL 段错误码且与 `ep-foundation::error::codes` 常量表一致（平台段错误码由阶段 1 登记，本阶段不重复登记），`docs/data-dictionary.md` 已登记 30 张表，三处由 CI 校验一致。
15. 附录 A.1 清单内本阶段的八个度量端点在附录 A.3 基准数据集上给出 `EXPLAIN` 证据，无顺序扫描。
16. portal-gateway 以独立系统账户 `ep-portal` 启动，进程内无任何事务数据库连接，该判定由 `/scripts/` 下的部署校验脚本以 `pg_stat_activity` 断言一次，不做每次启动的自检；反向代理上门户站点与员工站点使用独立站点、独立证书与独立访问策略，核对结论已写入部署记录。
17. 本阶段新增的三个指标可在 ops-agent 的 127.0.0.1:9101 上读到，标签基数符合基线第 9.2 节的纪律。
18. 第 7.2 小节的八个业务参数已通过配置发布通道发布一次，改值不需要重启进程与改表结构。
19. 本模块在规格第 6.2 章能力矩阵中取值为完整或简化的能力域，其四端界面已实现并通过 Playwright 与 tauri-driver 的桌面用例、XCUITest 与 Espresso 的移动用例；取值为 VIEW_ONLY 的能力域只实现只读视图；取值为 NOT_APPLICABLE 的不实现入口。供应商门户站点以浏览器承载，其用例由 Playwright 驱动。
20. 本阶段全部路由的能力域码与动作类别常量已在 `crates/contract/procure/src/capability.rs` 与 `crates/contract/portal/src/capability.rs` 声明，`xtask configdoc` 通过。
21. `ProcureReferenceCounter` 与 `ProcureTradeHistoryProvider` 已实现并在两个 wiring 目录注册到 `MasterReferenceCounterRegistry` 与 `TradeHistoryProviderRegistry`，覆盖未终态采购需求、采购订单、收货、采购退货、付款申请与采购订单行、收货行的历史成交。
22. 已在 `crates/contract/procure/src/port/subledger_balance.rs` 定义 `GrniSubledgerBalancePort`（`Send + Sync`，带 `#[async_trait::async_trait]`），方法为 `async fn balance(&self, snapshot: &dyn SnapshotCtx, legal_entity_id: Id<LegalEntity>, accounting_period_id: Id<AccountingPeriod>) -> Result<Money, AppError>`，返回该法人该会计期间的已收货未收票暂估合计；实现类型 `GrniSubledgerBalanceQuery` 位于 `crates/application/procure/src/projection/subledger_balance.rs`，`impl` 与类型同 crate。本阶段不写任何注入行，注入由阶段 10 在 `apps/core-server/src/wiring/` 与 `apps/job-worker/src/wiring/` 两个目录写入（裁定 G-01；`ep_contract_finance::SubledgerBalanceProvider` 一名全卷作废，本阶段不引用）。
23. `PurchaseReturnLinkPort::link_drop_ship_return` 已实现并在两个 wiring 目录首次接线，阶段 6 在本阶段之前未注入任何替身，直运退货勾稽端到端通过。
24. 八个单据类型码 PR、PO、GR、RJ、PRT、PAYR、DN、SIU 已登记入 `docs/data-dictionary.md` 的单据类型码一节与 `ep-platform-sequence` 的常量表，`xtask configdoc --check-doc-type-codes` 通过。
25. 两个 wiring 目录下的全部文件中不出现任何以 `Noop` 前缀命名的注入行，也不出现 `ReceiptInvoiceMatchQueryPort`、`PurchaseCreditNotePort`、`PayableLedgerQuery` 与 `SupplierStatementQuery` 四个端口的调用点。本阶段推迟到阶段 10 的五项在此逐条列名：采购退货的发票已登记分支与红字发票登记（第 4.4 小节）、付款申请的 `INVOICE_PAYMENT` 分支与占用写入路径（第 4.5 小节）、三个门户对账端点 `/portal/v1/reconciliation/purchase-invoices` 与 `/payments` 与 `/payable-balance`（第 5.7 小节）、集成场景第 12 项与第 20 项、E2E-T-03 的被受理一路即发票上传 `UPLOADED → ACCEPTED` 的回写与 `accepted_purchase_invoice_id` 的落值（第 4.2.7 小节与第 8.3 小节）。五项的实现与验收在阶段 10 同批执行，本阶段不为其登记任何顺延验收；承接该受理回写的端口的确切类型名与所属 crate 由阶段 7 与阶段 10 在落码前同批裁定，本阶段不预设该名，也不为此留任何占位。
26. `platform_core.append_only_registry` 中存在 `procure.goods_receipt_line_costings` 一行，其 `mode` 为 `APPEND_ONLY`、`mutable_columns` 为空数组，仅追加触发器已按该行挂接，`xtask sqlcheck` 执行 `db/checks/append_only_consistency.sql` 返回零行。
27. 严重与高危缺陷全部关闭，中危缺陷已登记并给出规避方案与责任人。

---

### 10. 与规格和 PRD 的对应

#### 10.1 规格条目

| 规格章节 | 本阶段实现的部分 |
|---|---|
| 第 5.2 章 采购与 SRM | 供应商准入与资质档案的采购侧承载；由合同派生、销售订单、项目任务、库存不足四个来源触发的采购需求；采购订单与分批订货；收货；退货；付款申请；供应商价格、交期、质量、风险的记录（价格、交期与风险存于 mdm，质量存于 procure）；物料类与直接费用类的两条路径与直运订单归入直接费用类 |
| 第 5.2 章 财务规则条目 | 只作为调用方：收货按采购收货事件与超量开票路径一先调用库存契约取价与写两账、再调用总账契约生成凭证；退货按采购退货事件与退货回冲的取价三分支同样先库存后总账。取价归库存模块，总账只做分录映射与借贷平衡，本阶段既不取价也不实现分录 |
| 第 5.5 章 供应商门户 | 五项门户能力、独立进程与独立系统账户、只访问脱敏投影与受控能力 API、不持有事务数据库账号与文件存储目录凭据 |
| 第 6.2 章 | 「采购与供应商协同」一行的四端取值为完整、完整、简化、简化；门户以浏览器承载不纳入四端等价判定 |
| 第 6.5 章 | 门户附件正文经大文件通道，其时延不计入门户提交通过线 |
| 第 7.7 章 | 三十张表的行级隔离以 `app.legal_entity_id` 为唯一判据；不使用 `BYPASSRLS`；跨法人查询按法人逐个设置变量分别查询 |
| 第 7.9 章 | 采购与门户单据进入内置搜索索引时产出 `foundation::port::search::SearchDocument`，携带来源对象 ID、版本、法人 ID、密级与数据范围标签，经 `SearchIndexPort` 由 job-worker 写入；删除与更正的传播窗口为 15 分钟 |
| 第 8 章 第 4 步 | 采购订货与分批订货，供应商门户确认订单与交期 |
| 第 8 章 第 5 步 | 收货登记生成入库单，驱动库存两账与暂估；收货数量差异必须在应用内登记；直接费用类不产生收货 |
| 第 8 章 第 10 步 | 付款申请的提交与审批部分；付款登记本身在财务阶段 |
| 第 8 章 第 11 步 | 采购退货按出库方向登记；直运货物退回供应商另登记一笔采购退货并与销售退货单勾稽 |
| 第 12.1 章 | 外部供应商用户的邀请开通与受限自助注册（后者默认关闭）；付款申请的提交与审批不属六类高风险操作 |
| 第 12.2 章 | 申请人不可自审、审批链不可越权跳过在采购订单、采购退货、付款申请三处生效；门户账号不得被授予任何内部角色 |
| 第 12.4 章 | 浏览器门户端的脱敏投影、水印、操作审计强制执行，导出审批以无导出入口满足 |
| 第 12.5 章 | 采购与门户的全部写操作、审批、门户操作写入审计事件，与业务变更同事务 |
| 第 13.1 章 | portal-gateway 的独立进程、独立系统账户与独立站点；本阶段不定义任何 cgroup 配额与让路次序，门户请求的过载处置一律走 portal-gateway 已有的限流与超时路径 |
| 第 15.1 章 | 五类错误分类的使用；存在性泄漏统一按 404 处理 |
| 第 15.2 章 | 合同派生需求失败、门户投影传播失败进入死信与人工修复 |
| 第 16 章 与 附录 A.1 | 八个度量端点；门户交互沿用常规交互通过线，门户提交沿用普通交易提交通过线 |
| 第 17.2 章 | 财务内核测试十五类必测分支中本阶段承担采购侧的第一、十、十一、十二、十三、四、七、十五类；数据保护控制测试的浏览器门户端条目；派生存储越权与删除传播测试中采购与门户对象的部分 |
| 第 17.3 章 | 本阶段承担采购侧的六条守恒判据（第 8.6 小节），并为库存数量守恒与两账一致提供不产生负结存的提交时前置校验 |
| 第 19 章 阶段 3 | 采购与 SRM 条目、供应商门户条目的四项闭环用例 |
| 第 21.17 章 | 门户暴露面的三项遏制：独立进程与账户、会话与限流取值收窄、受限自助注册默认关闭 |

#### 10.2 PRD 条目

| PRD 节 | 本阶段实现的功能 |
|---|---|
| 4.1 | 七类角色的权限对象与动作注册 |
| 4.2 | 物料类与直接费用类的七项对比全部落地；三条类型选择规则；类型锁定 |
| 4.3.1 至 4.3.4 | 四类来源、字段与校验、四态状态机、三条异常提示 |
| 4.4.1 至 4.4.6 | 两种分批订货形态、十三项输入字段校验、四项系统处理与输出、十一态状态机、变更留痕、四条异常提示 |
| 4.5.1 至 4.5.6 | 收货触发与操作者、九项字段校验、五项系统处理与输出、短收超收拒收三分支、四态加一态状态机、三条异常提示 |
| 4.6.1 至 4.6.5 | 两类适用场景、九项字段校验、五项系统处理与输出、五态状态机、三条异常提示 |
| 4.7.1 至 4.7.5 | 两类触发情形、九项字段校验、五项系统处理与输出、九态状态机、三条异常提示 |
| 4.8.1 至 4.8.3 | 准入状态与质量记录两类由本阶段承载；资质、价格、交期与风险记录四类经 mdm 契约读写，风险记录经 `SupplierRiskRecordPort`；准入四步流程；五态状态机与三条业务约束 |
| 4.9.1 至 4.9.7 | 五项能力白名单、七条访问与数据约束、订单与交期确认、送货通知、发票上传、收付款对账查询、自身档案维护 |
| 4.10 | 五条权限与职责分离规则 |
| 4.11 | 四条异常处理与错误反馈规则 |
| 4.12 | 四条验收要点 |
| 2.4.4 | 门户提交的档案变更经 `ep_contract_mdm::SupplierSelfServiceCommand` 生成待审批变更申请，本阶段提供门户侧入口与回执 |
| 5.5.2 | 通用库存侧输入与校验的五项字段与四步校验顺序，在收货与退货两处调用 |
| 5.5.3 | 采购收货入库的库存侧输入与系统处理，本阶段提供来源单据与调用编排 |
| 5.5.6 | 采购退货出库的库存侧输入与系统处理，同上 |
| 5.6.4 | 出库方向的提交时结存充足性校验在采购退货处落点 |
| 5.6.5 | 库存不足触发采购建议的建议单本体由本阶段承载，阈值字段与判定时点待决 |
| 6.8.2 | 财务在付款登记环节读取的六项付款申请信息，由 `ep-contract-procure` 的查询端口提供 |
| 6.8.6 | 付款申请的已付金额与状态回写，由 `PaymentRequestWritebackPort` 提供 |
| 6.10.4 | 门户对账查询的数据来源为财务的应付台账与核销关系，本阶段只做投影与裁剪 |
| 10.5.2 | 本阶段产生的提醒事项：审批待办到达、审批结果、死信与人工任务三类的采购与门户实例 |
| 11.2 | 门户并发计入合计 20 人上限 |

---

### 11. 风险与预留

#### 11.1 技术风险

风险一：收货与退货的同事务多腿写入使事务变长。收货过账在一个事务内串联采购单据写入、总账取价与凭证生成、库存两账写入三段，行数多时可能逼近 5 秒的业务事务预算与 3 秒的普通交易提交通过线。遏制手段是把单次收货明细行上限定为 200 行（`EP__PROCURE__RECEIPT__MAX_LINES`），超出转后台任务并由站内通知回执；并在阶段 4 的容量测试上以基准数据集实测。若实测不达标，收窄行数上限而不是拆事务，理由是拆事务会破坏第 17.3 章的两账一致与子账总账勾稽在写入时点即成立这一性质。

风险二：跨模块的同事务契约调用把三个模块的失败耦合在一起。库存或总账任一实现出现长事务或锁等待，采购侧的提交一并失败。遏制手段是 `InventoryPostingPort`、`AvailabilityQueryPort`、`InventoryPricingLookupPort`、`PostingPort` 与 `AccountingPeriodResolver` 五个端口的方法签名不含任何 IO 之外的等待语义、不做外部调用、不做长时计算，并在契约测试中断言其单次调用的语句数与耗时上界。库存与总账的真实实现已分别在阶段 8 与阶段 9a 合入，本阶段的契约测试对真实实现直接执行一遍，不停留在桩上。

风险三：门户是首版唯一的公网暴露面。遏制手段有三项：独立进程与独立系统账户、会话与限流取值收窄、受限自助注册默认关闭。原列的 cgroup 份额与突发上限一项按总览第 6.3 节 R10 删除，理由是它在一台 20 人规模的服务器上不构成运行期保证，过载处置改由 portal-gateway 的限流与超时承担。残余风险按规格第 21.17 章保留，门户与核心之间只有进程与系统账户边界而不是机器边界这一点不因本阶段的措施改变。

风险四：门户字段白名单一旦遗漏即构成数据外发。遏制手段是第 8.2 小节第 16 项的全字段快照测试，任何新增字段都会导致快照失败，必须显式更新快照并经评审。U-F-10 未决之前，白名单以本阶段第 4.7 小节的取值为准，发布前必须取得安全负责人批准，这是本阶段唯一的阻塞项。

风险五：`payable_reservations` 是本阶段引入的第二处金额状态，与财务侧的应付未核销余额存在漂移可能。遏制手段是 R-PROC-04 这条 `ReconCheck` 按周期核对，并在申请的每一次状态迁移上以同一事务内的加减维护，不做异步同步。

风险六：本阶段的八个业务参数取的是临时值，其中超收容差与转审批阈值（U-F-04）直接影响收货的拦截行为，若客户在实施期改值，历史已过账收货不重算。该性质须在交付说明中写明。

#### 11.2 本阶段的假设与新增决定

下列条目在规格与 PRD 中没有取值或存在分歧，本阶段显式作出取值并说明理由与切换代价。

假设 A1：采购需求是单行单据。理由是 PRD 第 4.3.2 小节的字段表只有单个物料与单个数量，没有明细行的结构。切换代价为新增一张 `procure.purchase_requisition_lines` 表与一次数据回填迁移，属中等代价，因此在整合期确认。

假设 A2：收货与采购退货的采购单据、库存两账与总账凭证在同一个数据库事务内同步写入，不经 Outbox 异步过账。理由有三条：规格第 17.3 章要求存货金额账合计等于总账存货科目余额；PRD 第 4.5.3 小节把凭证号列为收货登记的输出；PRD 第 4.5.6 小节要求库存或财务侧写入不一致时界面返回明确失败。三条同时成立只有同事务一种实现。由此产生的推论是：规格第 10.2 章关账受理前提下不存在收货与退货的异步过账路径，受理前提二统计的是这两个事件的未投递条目而不是未生成的凭证。本阶段仍在 Outbox 信封上携带 `posting_date` 与 `accounting_period_id`，两个事件在 `ledger.posting_trigger_event_types` 中的登记行按裁定 A-21 由阶段 9a 的种子迁移写入；`PostingTriggerRegistry::assert_registered` 按总览第 1.5 节第三条整项撤销，本阶段不做启动自检、不做 `--check` 静态断言，也不向关账受理追加前置校验，该统计的可枚举性由 `xtask configdoc` 在 CI 中对第 14 号种子迁移与 `docs/event-catalog.md` 的逐字比对以及阶段 3b 的 `event-catalog-consistent` 保证。总账与库存的契约端口按 A-01 接受 `&mut dyn Tx`，已由阶段 1 提供。

假设 A3：采购退货在「采购发票已登记」分支下调用 `ep_contract_invoice::PurchaseCreditNotePort::register_credit_note`，进项红字发票由 invoice 模块登记，采购侧只提供 `RegisterPurchaseCreditNote` 所需的供应商、原采购发票、退货单标识、过账日期与逐行的原发票行、收货行、数量、净额、税额。理由是 PRD 第 4.6.2 小节的字段表没有红字发票字段，而规格第 5.2 章采购退货事件要求按红字发票价税合计入账。该端口由阶段 10 交付，本阶段不注入任何替身，也不写该端口的调用点，发票已登记分支连同红字发票登记按第 4.4 小节整条推迟到阶段 10 与该端口同批交付。同一小节的「供应商不接受退回而不冲回成本」对应 U-C-09，该事项属 PRD 待决且规格未强制，本阶段不代拍置位方与撤销规则，只取一条临时取值：采购侧不置位，理由是它影响的是成本归集查询而不是采购单据。切换代价是在采购退货过账用例内增加一次置位调用，不改本阶段的表结构与迁移。

假设 A4（对应 U-C-08）：供应商的资质证照、价格资料、交期资料与风险记录唯一存储在 mdm 的供应商档案及其版本与子表上，`procure` 只存准入结论与质量记录两类；风险记录按裁定 C-10 一律经 `ep_contract_mdm::SupplierRiskRecordPort::append` 与 `::list` 读写，`procure.supplier_risk_records` 已撤销，见第 3.2.2 小节。理由是 PRD 第 2.4.1 小节与第 2.4.3 小节已把这四类定义为供应商档案的字段与子表，而 PRD 第 4.8.1 小节只是从采购视角复述。切换代价为把四张表从 mdm 迁到 procure 并改门户提交的写入目标，属中等代价。准入结论存于 procure 的理由是它只被采购侧读取且带自己的状态机。

假设 A5：门户账号的身份主体、口令、MFA、会话与设备登记归 `platform_identity`，`portal.supplier_portal_users` 只存账号与供应商与法人的授权绑定及五项能力白名单。理由是规格第 12.1 章把外部供应商用户列为身份能力，重复建目录会产生第二套认证面。

假设 A6：集合级 actions 路径（`POST /api/v1/<module>/<resource-plural>/actions/<verb>`，不带 `{id}`）是合法形态。理由是采购需求禁止手工新建但允许由销售订单行发起，该动作不属于任何已有需求。该形态回写基线第 5.1 节。

假设 A7：门户会话有效期 2 小时、空闲 15 分钟、会话校验缓存 30 秒、单附件 50 MB、列表页大小上限 50。五项均低于内部取值，理由是公网暴露面。回写基线第 11.6 节。

假设 A8：收货单状态机新增 `PENDING_APPROVAL` 一态。理由是 PRD 第 4.5.4 小节要求超收转审批而第 4.5.5 小节的状态机漏列该态。

假设 A9（对应 U-F-01）：物料类采购订单必须关联采购需求，直接费用类允许不关联需求直接创建。理由是规格第 8 章限定采购需求只有四个来源，若直接费用类也强制关联则服务类采购无入口。该取值由第 7.2 小节的两个业务参数承载，改值不需改代码。

假设 A10（对应 U-F-02）：库存不足触发采购需求所依据的补货阈值字段挂在物料与仓库的组合上，由库存或主数据模块承载，采购侧只消费其查询端口。本阶段的扫描任务默认关闭，因此本阶段不被该未决事项阻塞；开启条件是该阈值字段落地与 U-F-02 决策。

假设 A11（对应 U-F-04、U-F-05、U-F-12、U-F-13、U-F-14）：五项待决按第 7.2 小节的业务参数取临时值，本阶段不被阻塞，切换代价为一次配置发布。

假设 A12（对应 U-F-10）：门户返回字段白名单按第 4.7 小节取值，发布前必须由安全负责人批准。这是本阶段唯一的阻塞项，未获批准不得进入阶段 4 的发布门禁。

假设 A13（对应 U-F-11）：受限自助注册完整实现但默认关闭，开启条件是安全负责人对准入条件、审核人与防滥用措施的决策。本阶段不被阻塞。

假设 A14（对应 U-F-03）：本阶段不实现采购需求的合并与拆分，一张采购订单可关联多条同法人同供应商的需求（多对一由订单行上的 `purchase_requisition_id` 表达），但不产生合并后的新需求单。理由是合并键与回写方式未决，而多对一的表达已足够支撑需求侧分批与订单侧分批两种形态。切换代价为新增一张合并关系表。

假设 A15（对应 U-A-07、U-A-08、U-A-11）：退货原因、风险类型、付款条件三个字典与本阶段四条审批链的出厂配置由平台配置承载，本阶段只登记字典键与审批链标识，不定义取值内容。审批链为空时全部提交动作直接进入下一态，这一降级路径必须可用，理由是出厂即无可用审批配置时闭环不得中断。

假设 A16（对应 U-F-06、U-F-07、U-F-08、U-F-09）：四项均属 PRD 待决且规格未强制，本阶段不代拍，只给临时取值，四项都不阻塞本阶段。U-F-06 取直接费用类的合同、销售订单、项目三个归集字段至少一项非空，由 `ck_purchase_requisitions_type_fields` 与 `ck_purchase_order_lines_type_fields` 两条 CHECK 表达，切换代价是放宽这两条 CHECK 并由阶段 11 把三项全空的行归入未分摊差异。U-F-07 取首次收货登记或首次采购发票登记之前允许改采购类型、其后由 `is_type_locked` 锁定，切换代价是把该列的置位时点提前到订单下达，属一处守卫改动。U-F-08 取第 4.2.6 小节的五态与三条守卫，切换代价是改 `admission_status` 的 CHECK 取值与三条守卫分支。U-F-09 取质量记录的字段按第 3.2.2 小节、由采购退货过账经 Outbox 消费者自动生成，拒收与手工两类来源经第 5.6 小节的补录端点登记，风险类型字典按假设 A15 归平台配置，切换代价是增改生成消费者与字段列。

#### 11.3 为后续阶段预留的扩展点

1. `procure.purchase_order_lines` 上已预留 `invoiced_quantity` 列与 `PurchaseOrderInvoicingPort` 端口，供发票阶段回写累计已开票数量与把直接费用类订单推进到已完成。
2. `procure.goods_receipt_line_costings` 的 `allocation_kind` 以文本加 CHECK 表达，超量开票路径二与路径三若需要在收货侧留痕，只需扩展 CHECK 取值而不需改类型。
3. `procure.purchase_returns.is_invoice_registered` 由 invoice 契约回填，`purchase_return_lines.reversal_unit_price` 由库存契约回填，两列的填值方按取价归库存、匹配判定归发票的分层固定，采购侧任何情况下都不自行取价。
4. `portal.supplier_portal_users.capabilities` 为文本数组加 CHECK，后续版本恢复客户门户与经销商门户时，能力码集合可扩展而不改表结构；但门户能力是封闭白名单，扩展必须先在规格第 5.5 章登记。
5. `procure.payable_reservations` 的行结构可直接承载后续的付款计划占用，只需新增一个占用类型列。
6. 门户投影的字段白名单以常量表达并有全字段快照测试兜底，后续新增门户能力时该测试是唯一必须更新的地方，构成可审查的暴露面变更清单。
7. 本阶段不建任何物化视图，门户对账查询与内部台账同源直查。若阶段 4 的容量测试显示该查询击穿通过线，扩展点是在 `reporting` 侧建立只读投影，而不是在 `portal` 侧建第二套余额口径。
