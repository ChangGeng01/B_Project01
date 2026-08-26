> **F-57 状态：`HISTORICAL_DO_NOT_EXECUTE`。** 采购/供应商细节可复用；客户门户、多来源需求和 durable obligation 以 F-57 为准。

## 阶段 7：采购、门户与收货

> **F-50/F-51 范围修订。** 本阶段最终为 31 张业务表/33 个迁移/31 张法人 RLS 表、15 个事件；其中新增 `portal.supplier_invoice_upload_lines` 与 1 个内部退回端点。上传改为多行多税率，回写端口固定为 `ep_contract_portal::SupplierInvoiceUploadWritebackPort`，退回入口固定为 `/api/v1/portal/supplier-invoice-uploads/{id}/actions/return`。`RETURNED` 事件归本阶段，`ACCEPTED` 由阶段 10 受理事务产生。付款上限、门户余额只读 `effective_open`，采购退货先登记来源动作。F-51 U-C-09 另固定直运退货供应商拒收动作，经 PROCURE_MANAGER 提交、FINANCE_MANAGER 重新认证审批后调用 `CostReturnMarkPort`；该动作随阶段 11 真实实现同批启用。正文旧 30/31/30、14 个事件、头级税率、“端口待定”及“保留成本但不写标记”均被替代。

本阶段的范围是采购需求、采购订货与分批订货、供应商采购扩展档案、供应商门户、收货登记与入库单、采购退货、付款申请与审批。本阶段不实现采购发票登记、进项红字冲销、应付台账、付款登记与供应商返款，这五项属财务与发票阶段，本阶段只按契约衔接；进项发票台账的两张表 `invoice.purchase_invoices` 与 `invoice.purchase_invoice_lines` 由阶段 10 在 invoice schema 建立，本阶段不建表也不写台账。本阶段不实现库存数量账与金额账的写入算法，也不实现事件到分录的映射；本阶段不自行取价，取价一律归库存模块，总账只做分录映射与借贷平衡，本阶段按规格第 5.2 章财务规则条目与 PRD 第 5 节的分工调用其契约。本阶段不发布任何受治理数据集视图，采购发票数据集由阶段 10 在 invoice schema 发布。

本计划遵守共享技术基线。凡基线已给出取值的一律直接引用，不重新决定。本阶段新增的决定与假设集中在第 11.2 小节，并在正文各处以「本阶段新增决定」或「假设」标注。
本阶段在贯通线 T0 之后开工。T0 是阶段 3b-1 结束后、阶段 5 全量开工之前插入的一条不新增范围的最薄贯通线，其前置为阶段 1、2、3a、4 与 3b-1，固定链为 1 → 2 → 3a → 4 → 3b-1 → T0 → 5 → 9a → 8 → 6 → 7 → 10 → 11 → 9b → 14 共十五个环节，本阶段是其中第十一环，切片取自阶段 5、6、9a、10、11 五个阶段，判据是一条合同从建单走到管理层看到一个数。采购不在 T0 的切片清单内，本阶段不向 T0 贡献任何切片，也不因 T0 提前交付任何采购能力。本阶段的全部工作是在 T0 已经贯通的骨架上加厚：会计期间解析、凭证生成、销项发票与到款登记在 T0 上已经跑通，本阶段沿这条骨架追加采购订货、收货、采购退货与付款申请四段，另加门户这一条外部入口。骨架上已经成立的判据不在本阶段重新论证，本阶段只对新增的分支给判据。M7 相应改为全分支闭环而不是闭环的首次贯通，本阶段的验收措辞一律不写首次跑通。

---

### 1. 交付物清单

本阶段结束时，下列可运行物存在并可在单台服务器的认证部署形态上启动与验证。

1. core-server 进程内新增两组 HTTP 路由并可用：内部采购路由 `/api/v1/procure/*`，门户受控能力路由 `/api/v1/portal/*`。两组路由共用 core-server 的安全上下文、行级隔离、幂等、审计与 Outbox 机制，不新增第二套。
2. portal-gateway 进程可对外承载供应商门户站点，路由前缀 `/portal/v1`，实现门户会话、限流、水印与呈现层裁剪，全部取数与写入经 core-server 的受控能力 API，本进程不建立任何事务数据库连接。
3. job-worker 进程内新增四类消费者与一个定时任务：合同生效派生采购需求的 Outbox 消费者、采购退货生成供应商质量记录的消费者、采购与门户单据的检索索引与门户投影刷新消费者（产出 `foundation::port::search::SearchDocument` 并经 `SearchIndexPort` 写入）、采购与门户单据的站内通知投递消费者，以及库存不足触发采购需求的扫描定时任务（F-51 冻结为每 60 分钟扫描、部署默认关闭）。该任务注入阶段 6/sales 提供的真实 `SalesAwareReplenishmentPolicyQuery`，不直接访问 `inventory.replenishment_policies` 或重算可用量。另把 `CLM_TERM_PURCHASE_REQUISITION` 的真实 `ImpactRule` 注册到既有 `ImpactRegistry`，使阶段累计真实注册数由 3 增至 4；合同终止仍只由平台 `platform.impact_assess` 消费，不新增采购侧消费者。
4. `procure` 与 `portal` 两个 schema 的全部表、约束、索引与行级安全策略，由 `tools/ep-migrate` 按 ADR-0013 的自建 Runner 离线执行；常规迁移逐文件事务化，`concurrent/` 目录走非事务执行器，并可按迁移文件头的回退说明回退。本阶段不得引入第二套迁移执行器。
5. 七个新增 crate：`ep-contract-procure`、`ep-domain-procure`、`ep-app-procure`、`ep-contract-portal`、`ep-domain-portal`、`ep-app-portal`，以及只先建立 U-C-09 契约切片的 `ep-contract-costing`；后者由阶段 11 原位扩展并交付实现，不重复建 crate。
6. `ep-testkit` 中新增的采购与门户构造器，以及三个记录型桩（`RecordingStockPostingPort` 记录 `ep_contract_inventory::InventoryPostingPort` 的调用，`RecordingStockOnHandQueryPort` 记录 `ep_contract_inventory::StockOnHandQueryPort` 的调用，`RecordingLedgerPostingPort` 记录 `ep_contract_ledger::PostingPort` 的调用），用于契约测试的入参断言与故障注入。库存与总账的真实实现分别在阶段 8 与阶段 9a 已合入，三个记录型桩只出现在测试装配，发布装配一律注入真实实现。发票与财务两个模块的四个端口在本阶段之后交付，本阶段一律不注入替身，四个调用点在本阶段的代码中不存在：`ReceiptInvoiceMatchQueryPort` 与 `PurchaseCreditNotePort` 所支撑的采购退货发票已登记分支按第 4.4 小节整条推迟到阶段 10；`PayableLedgerQuery` 所支撑的付款申请 `INVOICE_PAYMENT` 分支按第 4.5 小节整条推迟到阶段 10；`SupplierStatementQuery` 所支撑的三个门户对账端点按第 5.7 小节随该端口在阶段 10 同批交付。两个 wiring 目录下的全部文件中不出现任何以 `Noop` 前缀命名的注入行。
7. `ep-datagen` 中新增的采购侧基准数据生成器，产出附录 A.3 规模中的采购订单行 10 万条与其对应的收货、退货与付款申请分布。
8. 一份可执行的端到端用例集合，覆盖规格第 8 章闭环第 4 步、第 5 步收货腿、第 10 步的申请与审批腿、第 11 步的采购退货腿，以及规格第 19 章阶段 3 门户条目要求的采购订单与交期确认、送货通知、发票上传、收付款对账查询四项闭环用例。
9. 三份登记文档的增量：`docs/event-catalog.md` 登记本阶段最终 15 个事件类型；`docs/error-codes.md` 的本阶段完整引用固定为 31 个 PROCURE/PORTAL 码，即正文逐字出现的 27 个 `PROCURE.*`、门户通用闸门 `PORTAL.PORTAL_USER.CAPABILITY_NOT_GRANTED`，以及 F-50 先行登记的 `PORTAL.SUPPLIER_INVOICE_UPLOAD.DUPLICATED`、`PORTAL.SUPPLIER_INVOICE_UPLOAD.STATE_CHANGED`、`PORTAL.SUPPLIER_INVOICE_UPLOAD.CONTENT_MISMATCH` 三个上传码；`docs/data-dictionary.md` 登记最终 31 张表并在单据类型码一节补齐本阶段的八个类型码。
10. 采购模块的四端界面：`clients/desktop/src/modules/procure/` 与 `clients/mobile/src/modules/procure/` 两个目录；供应商门户站点以浏览器承载，由 portal-gateway 交付，不进 `clients/`。

---

### 2. crate 与进程归属

#### 2.1 新增 crate

| crate | 目录 | 职责 | 依赖 |
|---|---|---|---|
| ep-contract-procure | crates/contract/procure | 采购模块对外公开的命令、查询、事件类型与 DTO；供其他模块调用的 trait，含 `PaymentRequestQueryPort`、`PaymentRequestWritebackPort`、`PayableReservationReadPort`、`PurchaseOrderInvoicingPort`、`PurchaseRequisitionIntakePort`、`PurchaseReturnLinkPort`、`GrniEffectWritebackPort`（进项发票与进项红字同事务追加 GRNI 效果）与 `GrniSubledgerBalancePort`（截至期间的子账余额端口，二者分别定义在 `src/port/grni_effect_writeback.rs` 与 `src/port/subledger_balance.rs`）。无消费者的 `GoodsReceiptQueryPort` 删除，不创建空 trait；`src/capability.rs` 中为每个用例声明 `<USECASE_SCREAMING>_DOMAIN` 与 `<USECASE_SCREAMING>_ACTION` 一对常量 | 仅 ep-foundation |
| ep-domain-procure | crates/domain/procure | 采购需求、采购订单、收货单、采购退货单、付款申请、供应商准入与质量记录六类聚合；数量守恒、可退数量、累计下达数量、占用金额四组不变量；业务端口 trait | ep-foundation、ep-contract-procure |
| ep-app-procure | crates/application/procure | 采购各用例、事务边界、授权调用、审计与 Outbox 写入、与库存与总账两个模块契约的编排；`src/probe/` 下的 `ProcureReferenceCounter` 与 `ProcureTradeHistoryProvider`；六个 `ReconCheck` 实现（R-PROC-01 至 R-PROC-05 与 R-PORT-01）；`GrniEffectWriteback` 与 `GrniSubledgerBalanceQuery` 两个实现类型，分别位于 `src/writeback/grni_effect.rs` 与 `src/projection/subledger_balance.rs`；`src/impact/contract_termination_purchase_requisition.rs` 中实现 `CLM_TERM_PURCHASE_REQUISITION` | ep-foundation、ep-platform-*（含 `ep-platform-impact`）、ep-domain-procure、ep-contract-* |
| ep-contract-portal | crates/contract/portal | 门户受控能力的命令、查询与 DTO；门户投影的字段白名单类型；`PortalCapability` 枚举；`src/capability.rs` 中为每个门户用例声明 `<USECASE_SCREAMING>_DOMAIN` 与 `<USECASE_SCREAMING>_ACTION` 一对常量 | 仅 ep-foundation |
| ep-domain-portal | crates/domain/portal | 门户账号绑定、送货通知、发票上传记录三类聚合；能力白名单与供应商数据范围两组不变量 | ep-foundation、ep-contract-portal |
| ep-app-portal | crates/application/portal | 门户五项能力的受控用例、投影组装与脱敏裁剪、门户操作审计写入 | ep-foundation、ep-platform-*、ep-domain-portal、ep-contract-* |
| ep-contract-costing | crates/contract/costing | 本阶段只定义 `CostReturnMarkPort::mark_unreversed_return_cost(&mut dyn Tx, &SecurityContext, CostReturnMarkCommand)`；命令固定含 purchase_return_id、sales_return_id、reason、evidence_attachment_ids、submitted_by、reauth_ref、approval_ref。阶段 11 在同一 crate 增补成本/收入契约并实现本 trait；本阶段不提供 Noop | 仅 ep-foundation |

依赖方向逐条自检：`ep-domain-procure` 不依赖 `ep-contract-inventory` 与 `ep-contract-ledger`，跨模块调用一律经 `ep-app-procure`；`ep-app-portal` 不依赖 `ep-app-procure`，门户对采购单据的读写经 `ep-contract-procure` 的 trait，实现在 `apps/core-server/src/wiring/` 目录下注入。这两条是本阶段最容易被违反的两条，由阶段 1 交付的 `xtask archcheck` 按层位断言：前者落禁止项第一条 `domain-no-cross-module`，后者落禁止项第二条 `app-no-peer-app`，被测输入是 `cargo metadata --no-deps` 建出的层位图；本阶段不另立按 crate 逐项比对期望依赖清单的自检脚本（裁定 F-05 通则甲-3）。

#### 2.2 改动的既有 crate

| crate | 改动 |
|---|---|
| ep-adapter-db-pg | 新增 `src/repo/procure/` 与 `src/repo/portal/` 两个目录，按表分文件，每个仓储只访问自己模块的 schema |
| ep-testkit | 新增 `SupplierFixture`、`PurchaseOrderBuilder`、`GoodsReceiptBuilder`、`PurchaseReturnBuilder`、`PaymentRequestBuilder`、`PortalUserFixture`、`DeliveryNoticeBuilder`；新增 `InventoryPostingPort`、`StockOnHandQueryPort` 与 `PostingPort` 三个契约的记录型桩 |
| ep-datagen | 新增 `--module procure` 分支 |
| apps/core-server | 路由注册、权限对象类型注册；wiring 为收货/退货编排注入阶段 8 的真实 `InventoryPostingPort` 与 `StockOnHandQueryPort` 以及阶段 9a 的真实 `PostingPort`，采购退货不得注入或调用扣除了销售未交付需求的 `AvailabilityQueryPort`；发票与财务两个模块的端口一律不注入任何替身且其调用点在本阶段的代码中不存在，`PurchaseReturnLinkPort` 由本阶段首次接线，以及本模块按裁定 A-15 向 `MasterReferenceCounterRegistry` 与 `TradeHistoryProviderRegistry` 的注册 |
| apps/portal-gateway | 站点、会话、限流、水印、五项能力的呈现层与转发 |
| apps/job-worker | 四个消费者与一个定时任务的注册；为库存不足扫描注入阶段 6/sales 的真实 `SalesAwareReplenishmentPolicyQuery`；本阶段六个 `ReconCheck`（R-PROC-01 至 R-PROC-05 与 R-PORT-01）向 `ReconRegistry` 的注册；把 `ContractTerminationPurchaseRequisitionImpactRule` 注册进既有 `ImpactRegistry`，注册后累计恰为 4，不注册第二个合同终止消费者 |
| ep-platform-obs | 不新增指标；采购与门户只填充既有 HTTP、数据库、Outbox、死信与对账指标 |

#### 2.3 进程归属

| 能力 | 承载进程 |
|---|---|
| 采购全部用例、门户受控能力 API、收货与退货的同事务过账编排 | core-server |
| 门户站点、门户会话、门户限流、门户水印与呈现层 | portal-gateway |
| 合同派生需求的 Outbox 消费、质量记录生成、门户投影与检索索引刷新、站内通知投递、库存不足扫描定时任务，以及 `CLM_TERM_PURCHASE_REQUISITION` 规则装配 | job-worker |
| 本阶段使用既有指标的聚合暴露 | ops-agent |

本阶段不新增进程，不改动进程的监听地址、数据库连接池上限、系统账户与资源单位。

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

数据库引用按单一目标与封闭多态两类处理。指向单一目标的列一律建立真实外键：双方带法人列时使用 `(legal_entity_id,ref_id) -> target(legal_entity_id,id) ON DELETE RESTRICT`，目标表显式提供 `UNIQUE (legal_entity_id,id)` 候选键；业务用户列指向 `platform_authz.user_legal_entity_grants(legal_entity_id,user_id)`；门户全局身份主体用单列外键指向 `platform_core.user_accounts(id)`，并在绑定事务内校验账号种类、状态和本法人授权。只有本节明确列出且带种类判别列的封闭多态引用，以及 `approval_ref` 白名单不建外键，写入用例必须按判别值校验目标、法人、业务归属和状态。对账框架只承担金额与数量守恒，不代替引用完整性。

目标晚建的单目标引用也不得永久降级为逻辑引用，按下列精确追补文件落地：`V20261018093200__procure_add_portal_foreign_keys.sql` 在本阶段 portal 表建完后，以带采购订单/订单行祖先列的长复合形状补 `goods_receipts.delivery_notice_id`、`goods_receipt_lines.delivery_notice_line_id` 与 `receipt_rejections.delivery_notice_id`，并安装收货-notice 延迟图；`V20261019090830__portal_add_invoice_foreign_keys.sql` 补 `supplier_invoice_uploads.accepted_purchase_invoice_id`；`V20261019090930__procure_add_invoice_foreign_keys.sql` 补 `purchase_return_lines.purchase_invoice_line_id` 与 `payable_reservations.purchase_invoice_id`，并把 DROP_SHIP 原发票行归属纳入退货延迟图；`V20261021090030__procure_add_project_foreign_keys.sql` 在 `project.projects` 建立后补采购需求/订单行中的 `project_id`。追补前相应写入口尚未启用，禁止留下“应用校验即可”的永久过渡态。

金额列一律 `numeric(18,2)`，单价列一律 `numeric(18,6)`，数量列一律 `numeric(18,6)`，税率列一律 `numeric(9,6)`。批次列与序列号列在物料未启用相应管理时取固定值 `'-'`，按基线第 11.4 节。

密级取值：采购需求、采购订单、收货单、采购退货单、送货通知、发票上传取 20；付款申请取 30。数据范围标签的取值集合为 `dept:<部门码>`、`supplier:<供应商编码>`、`contract:<合同编号>`、`project:<项目编号>`、`sales_order:<订单编号>`，标签不承载任何敏感值。

#### 3.2 procure schema 的表

##### 3.2.1 procure.supplier_admissions（供应商准入结论）

| 列 | 类型 | 可空 | 说明 |
|---|---|---|---|
| supplier_id | uuid | 否 | 与法人组成复合外键指向 `mdm.suppliers(legal_entity_id,id)` |
| admission_status | text | 否 | CHECK 取值 `PENDING`、`REJECTED`、`ADMITTED`、`SUSPENDED`、`TERMINATED` |
| concluded_on | date | 是 | 准入结论日期 |
| reviewer_user_id | uuid | 是 | 审核人；与法人组成复合外键指向 `platform_authz.user_legal_entity_grants(legal_entity_id,user_id)` |
| valid_until | date | 是 | 准入有效期止日 |
| reason | text | 是 | CHECK 长度不超过 2000 |
| portal_enabled | boolean | 否 | 默认 false，与 mdm 的门户开通标记同步，取值来源以 mdm 为准 |

约束与索引：`pk_supplier_admissions`；`ux_supplier_admissions_legal_entity_id_supplier_id`；`ix_supplier_admissions_legal_entity_id_created_at`；`ix_supplier_admissions_legal_entity_id_valid_until`。本表不是单据类也不是档案类，不带 `doc_no` 与 `code`。

##### 3.2.2 procure.supplier_quality_records（供应商质量记录）

| 列 | 类型 | 可空 | 说明 |
|---|---|---|---|
| supplier_id | uuid | 否 | 与法人组成复合外键指向 `mdm.suppliers(legal_entity_id,id)` |
| source_type | text | 否 | CHECK 取值 `PURCHASE_RETURN`、`RECEIPT_REJECTION`、`MANUAL` |
| source_doc_id | uuid | 是 | 与 `source_type` 组成封闭多态引用；`MANUAL` 时为空，另两类由写入用例校验同法人来源单据，不建伪外键 |
| source_doc_no | text | 是 | 冗余存编号，供只读展示 |
| material_id | uuid | 是 | 与法人组成复合外键指向 `mdm.materials(legal_entity_id,id)` |
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
| source_doc_id | uuid | 是 | 与 `source_type` 组成封闭多态来源引用，由 owner 在同一事务校验目标与法人 |
| source_doc_line_id | uuid | 是 | 与 `source_type` 组成封闭多态来源行引用；无行级来源时为空 |
| source_doc_no | text | 是 | 只读展示用 |
| source_idempotency_key | text | 否 | 来源侧幂等键，四类来源各自的去重依据 |
| suggested_purchase_type | text | 否 | CHECK 取值 `MATERIAL`、`DIRECT_EXPENSE` |
| material_id | uuid | 是 | 物料类必填；与法人组成复合外键指向 `mdm.materials(legal_entity_id,id)` |
| warehouse_id | uuid | 是 | `STOCK_SHORTAGE` 来源必填；与法人组成复合外键指向 `mdm.warehouses(legal_entity_id,id)`，并与 `material_id` 共同承载补货策略维度 |
| expense_item_code | text | 是 | 直接费用类必填 |
| required_quantity | numeric(18,6) | 否 | CHECK 大于零 |
| ordered_quantity | numeric(18,6) | 否 | 默认 0，累计已下达数量 |
| expected_arrival_date | date | 否 | CHECK 不早于 `created_at` 的服务器自然日 |
| contract_id / sales_order_id / project_id | uuid | 是 | 直接费用类至少一项非空；三列分别以同法人复合外键指向 `clm.contracts`、`sales.sales_orders`、`project.projects`，其中 project 外键由 `V20261021090030__procure_add_project_foreign_keys.sql` 追补；CONTRACT、SALES_ORDER 的 `contract_id` 必填；PROJECT_TASK 的 `project_id` 必填，`contract_id` 只在该任务确有来源合同时固化，可空 |
| suggested_supplier_id | uuid | 是 | 与法人组成复合外键指向 `mdm.suppliers(legal_entity_id,id)` |
| is_drop_ship | boolean | 否 | 默认 false，来源为直运订单时为 true 且 `suggested_purchase_type` 固定为 `DIRECT_EXPENSE` |
| close_reason | text | 是 | 关闭原因 |
| closed_at | timestamptz | 是 | |
| open_stock_shortage_key | text | 是 | `GENERATED ALWAYS AS (CASE WHEN source_type='STOCK_SHORTAGE' AND status<>'CLOSED' THEN warehouse_id::text\|\|':'\|\|material_id::text ELSE NULL END) STORED`；客户端不可写 |

`status` CHECK 取值 `PENDING`、`PARTIALLY_ORDERED`、`ORDERED`、`CLOSED`。表级 CHECK：`ck_purchase_requisitions_ordered_qty_le_required`（`ordered_quantity <= required_quantity`）；`ck_purchase_requisitions_type_fields`（物料类必填 `material_id`，直接费用类必填 `expense_item_code` 且三个归集字段至少一项非空）；`ck_purchase_requisitions_stock_shortage_fields` 逐项判空并要求 STOCK_SHORTAGE 时 `warehouse_id/material_id` 均非空且类型为 MATERIAL、其他来源 `open_stock_shortage_key` 必为空；`ck_purchase_requisitions_drop_ship_type`（`is_drop_ship` 为真时类型必须为 `DIRECT_EXPENSE`）；`ck_purchase_requisitions_source_owner` 使用 NULL-safe 封闭形状：CONTRACT 要求 `contract_id IS NOT NULL`，SALES_ORDER 要求 `sales_order_id IS NOT NULL AND contract_id IS NOT NULL`，PROJECT_TASK 要求 `project_id IS NOT NULL` 而允许 `contract_id` 为空，STOCK_SHORTAGE 不借 `contract_id/sales_order_id/project_id` 冒充业务来源。旧的“PROJECT_TASK 一律要求合同”形状作废。

索引与唯一键：`pk_`；`ux_purchase_requisitions_legal_entity_id_doc_no`；`ix_purchase_requisitions_legal_entity_id_created_at`；`ux_purchase_requisitions_legal_entity_id_source_idempotency_key`；`ux_purchase_requisitions_legal_entity_id_open_stock_shortage_key`；`ix_purchase_requisitions_legal_entity_id_status_expected_arrival_date`。生成键唯一约束从数据库层保证同一法人、仓库、物料最多一张未关闭自动需求；CLOSED 后生成键为 NULL，允许下一扫描时段创建新需求。

##### 3.2.4 procure.purchase_orders（采购订单，单据类）

| 列 | 类型 | 可空 | 说明 |
|---|---|---|---|
| supplier_id | uuid | 否 | 与法人组成复合外键指向 `mdm.suppliers(legal_entity_id,id)` |
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

索引：`pk_`；`ux_purchase_orders_legal_entity_id_doc_no`；候选键 `UNIQUE(legal_entity_id,id)`；`ix_purchase_orders_legal_entity_id_created_at`；`ix_purchase_orders_legal_entity_id_supplier_id_status`；`ix_purchase_orders_legal_entity_id_status_order_date`。第四条索引直接支撑门户的待确认列表与 A.1 度量项「采购订单与交期待确认列表加载」，须在基准数据集上给出无顺序扫描的 `EXPLAIN` 证据。

##### 3.2.5 procure.purchase_order_lines

| 列 | 类型 | 可空 | 说明 |
|---|---|---|---|
| purchase_order_id | uuid | 否 | 同 schema 外键 `fk_purchase_order_lines_purchase_orders` |
| line_no | integer | 否 | 行号 |
| purchase_requisition_id | uuid | 是 | 同 schema 外键 |
| material_id | uuid | 是 | 与法人组成复合外键指向 `mdm.materials(legal_entity_id,id)` |
| expense_item_code | text | 是 | |
| quantity | numeric(18,6) | 否 | CHECK 大于零 |
| unit_price_untaxed | numeric(18,6) | 否 | CHECK 大于等于零 |
| tax_rate | numeric(9,6) | 否 | 取值来自税率字典，唯一出处按裁定 C-11 与总览第 1.5 节第五条为 `invoice.tax_rate_options`，唯一取用入口为 `ep_contract_invoice::TaxRateOptionQuery` 的 `default_rate` 与 `list`，该表的建表与种子两条迁移及该查询由阶段 10 在 T0 期间交付，属阶段 10 的 T0 切片第五项；本阶段取默认税率一律经 ep-contract-invoice，不经 ep-contract-mdm，不自建税率字典，也不存在任何税率桩 |
| agreed_delivery_date | date | 否 | CHECK 不早于订单日期，由应用层校验并在写入时冗余 `order_date` 以支撑表级 CHECK |
| order_date | date | 否 | 冗余自订单头，仅为表级 CHECK 与索引服务 |
| warehouse_id | uuid | 是 | 物料类必填；与法人组成复合外键指向 `mdm.warehouses(legal_entity_id,id)` |
| contract_id / sales_order_id / project_id | uuid | 是 | 直接费用类至少一项非空；三列分别以同法人复合外键指向 `clm.contracts`、`sales.sales_orders`、`project.projects`，其中 project 外键由 `V20261021090030__procure_add_project_foreign_keys.sql` 追补 |
| received_quantity | numeric(18,6) | 否 | 默认 0 |
| returned_quantity | numeric(18,6) | 否 | 默认 0 |
| invoiced_quantity | numeric(18,6) | 否 | 默认 0，由发票模块经契约回写，本阶段只建列与回写入口 |
| line_status | text | 否 | CHECK 取值 `OPEN`、`FULLY_RECEIVED`、`CLOSED`、`VOIDED` |

表级 CHECK：`ck_purchase_order_lines_delivery_date`（`agreed_delivery_date >= order_date`）；`ck_purchase_order_lines_type_fields`；`ck_purchase_order_lines_progress` 强制 `received_quantity>=0 AND returned_quantity>=0 AND returned_quantity<=received_quantity AND invoiced_quantity>=0 AND invoiced_quantity<=quantity`；`ck_purchase_order_lines_receipt_status` 强制 `line_status='OPEN'` 时 `received_quantity<quantity`、`line_status='FULLY_RECEIVED'` 时 `received_quantity>=quantity`，CLOSED/VOIDED 保留其终态业务含义而不倒改累计。候选键 `UNIQUE(legal_entity_id,purchase_order_id,id)`；`purchase_order_id` 使用 `(legal_entity_id,purchase_order_id) -> purchase_orders(legal_entity_id,id) ON DELETE RESTRICT` 真实复合外键。索引：`pk_`；`ux_purchase_order_lines_purchase_order_id_line_no`；`ix_purchase_order_lines_legal_entity_id_created_at`；`ix_purchase_order_lines_legal_entity_id_material_id_line_status`。合法超收允许 `received_quantity>quantity`，但必须由下述收货图证明超出部分来自带完整审批证据的收货，不得用一个错误的 `received_quantity<=quantity` CHECK 阻断。

##### 3.2.6 procure.purchase_order_line_batches（交货批次行）

列为 `purchase_order_id uuid not null`（冗余祖先键，客户端不可写）、`purchase_order_line_id uuid not null`、`batch_no integer not null`、`batch_quantity numeric(18,6) not null`（CHECK 大于零）、`agreed_delivery_date date not null`、`received_quantity numeric(18,6) not null default 0`、`batch_status text not null`（CHECK 取值 `OPEN`、`FULLY_RECEIVED`、`CLOSED`）。候选键 `UNIQUE(legal_entity_id,purchase_order_id,purchase_order_line_id,id)`，并建 `(legal_entity_id,purchase_order_id,purchase_order_line_id) -> purchase_order_lines(legal_entity_id,purchase_order_id,id) ON DELETE RESTRICT` 长复合外键；`ck_purchase_order_line_batches_progress` 强制 `received_quantity>=0`，`batch_status='OPEN'` 时 `received_quantity<batch_quantity`、`FULLY_RECEIVED` 时 `received_quantity>=batch_quantity`，合法超收仍由收货审批图证明。索引：`pk_`；`ux_purchase_order_line_batches_purchase_order_line_id_batch_no`；`ix_..._legal_entity_id_created_at`。

`V20261018090500__procure_create_purchase_order_line_batches.sql` 在头、行、批次齐备后建立 `procure.assert_purchase_order_graph_consistent()`，并在三表各装一个 `DEFERRABLE INITIALLY DEFERRED` 约束触发器。提交时按订单行与批次 id 稳定锁读，强制每条订单行至少一个批次且 `SUM(batch_quantity)=purchase_order_lines.quantity`、`SUM(batch.received_quantity)=purchase_order_lines.received_quantity`；订单头三项金额分别等于所有行的 `SUM(round(quantity*unit_price_untaxed,2))`、`SUM(round(round(quantity*unit_price_untaxed,2)*tax_rate,2))` 与前两项之和，舍入使用 PostgreSQL numeric 的 MidpointAwayFromZero 行级两位规则。状态进入 PENDING_APPROVAL 及其后任一非 DRAFT 状态时至少一行，DRAFT 也不得保存与现有行汇总不等的头金额。该函数不把合法超收误判为非法；超额来源证据在收货图校验。普通 FK 全部命中但把批次挂到另一订单行、批次数量合计不等、头金额篡改或行/批累计漂移均在提交时拒绝。迁移末尾才启用触发器；回退先删触发器与函数，再删批次表。

##### 3.2.7 procure.purchase_order_payment_plans（预计付款计划）

列为 `purchase_order_id uuid not null`（同 schema 外键）、`plan_no integer not null`、`planned_date date not null`、`planned_amount numeric(18,2) not null`、`plan_note text null`。索引：`pk_`；`ux_purchase_order_payment_plans_purchase_order_id_plan_no`；`ix_..._legal_entity_id_created_at`。

##### 3.2.8 procure.goods_receipts（收货单，单据类）

| 列 | 类型 | 可空 | 说明 |
|---|---|---|---|
| supplier_id | uuid | 否 | 与法人组成复合外键指向 `mdm.suppliers(legal_entity_id,id)` |
| purchase_order_id | uuid | 否 | 同 schema 外键 |
| delivery_notice_id | uuid | 是 | 与法人组成复合外键指向 `portal.delivery_notices(legal_entity_id,id)`，由 `V20261018093200__procure_add_portal_foreign_keys.sql` 追补 |
| posting_date | date | 否 | 该业务事件的记账日期，取值即收货日期，CHECK 不晚于登记时点的服务器自然日 |
| accounting_period_id | uuid | 是 | 过账时由 `ep_contract_ledger::AccountingPeriodResolver::resolve` 解析并写入；与法人组成复合外键指向 `ledger.accounting_periods(legal_entity_id,id)`，草稿态为空 |
| voucher_id | uuid | 是 | 收货过账时由 `ep_contract_ledger::PostingPort::post` 返回并写入；与法人组成复合外键指向 `ledger.vouchers(legal_entity_id,id)`，草稿态为空 |
| has_over_receipt | boolean | 否 | 默认 false |
| over_receipt_reason | text | 是 | `has_over_receipt` 为真时必填，由 CHECK 表达 |
| over_receipt_approval_ref | uuid | 是 | |
| posted_at | timestamptz | 是 | |

`status` CHECK 取值 `DRAFT`、`PENDING_APPROVAL`、`POSTED`、`PARTIALLY_RETURNED`、`FULLY_RETURNED`。另建 NULL-safe `ck_goods_receipts_posting_shape`：`status IN ('DRAFT','PENDING_APPROVAL')` 时 `accounting_period_id`、`voucher_id`、`posted_at` 三列必须全空；`status IN ('POSTED','PARTIALLY_RETURNED','FULLY_RETURNED')` 时 `accounting_period_id/posted_at` 必须全非空，`voucher_id` 由第 11 号迁移的延迟效果图按零金额规则判定。该 CHECK 随 `V20261018090700__procure_create_goods_receipts.sql` 首次建表落地，不另留后补窗口；同文件建立候选键 `UNIQUE(legal_entity_id,purchase_order_id,id)`。`PENDING_APPROVAL` 是本阶段对 PRD 第 4.5.5 小节状态机的补充，理由是 PRD 第 4.5.4 小节要求超收转审批而状态机漏列该态，见第 11.2 小节假设 A8。

索引：`pk_`；`ux_goods_receipts_legal_entity_id_doc_no`；`ix_goods_receipts_legal_entity_id_created_at`；`ix_goods_receipts_legal_entity_id_purchase_order_id_status`；`ix_goods_receipts_legal_entity_id_posting_date`。

##### 3.2.9 procure.goods_receipt_lines

| 列 | 类型 | 可空 | 说明 |
|---|---|---|---|
| goods_receipt_id | uuid | 否 | 同 schema 外键 |
| purchase_order_id | uuid | 否 | 冗余祖先键，客户端不可写；与收货头及订单行组成长复合外键 |
| line_no | integer | 否 | |
| purchase_order_line_id | uuid | 否 | 同 schema 外键 |
| purchase_order_line_batch_id | uuid | 是 | 同 schema 外键 |
| delivery_notice_id | uuid | 是 | 冗余自收货头；与 notice line 同空同非空，客户端不可写 |
| delivery_notice_line_id | uuid | 是 | 与头 notice、订单及订单行组成长复合外键，由 `V20261018093200__procure_add_portal_foreign_keys.sql` 追补 |
| material_id | uuid | 否 | 与法人组成复合外键指向 `mdm.materials(legal_entity_id,id)` |
| warehouse_id | uuid | 否 | 与法人组成复合外键指向 `mdm.warehouses(legal_entity_id,id)` |
| quantity | numeric(18,6) | 否 | CHECK 大于零 |
| batch_no | text | 否 | 默认 `'-'` |
| returned_quantity | numeric(18,6) | 否 | 默认 0 |
| order_unit_price_untaxed | numeric(18,6) | 否 | 登记时固化的采购订单不含税单价，作为库存契约判定暂估入账单价的入参，采购侧不据此取价 |

表级 CHECK：`ck_goods_receipt_lines_returned_le_quantity` 精确强制 `0<=returned_quantity<=quantity`，另有 NULL-safe `ck_goods_receipt_lines_notice_shape` 强制 `delivery_notice_id/delivery_notice_line_id` 同空同非空。候选键为 `UNIQUE(legal_entity_id,goods_receipt_id,id)` 与 `UNIQUE(legal_entity_id,goods_receipt_id,purchase_order_id,purchase_order_line_id,id)`；三条 `ON DELETE RESTRICT` 长复合外键为 `(legal_entity_id,purchase_order_id,goods_receipt_id) -> goods_receipts(legal_entity_id,purchase_order_id,id)`、`(legal_entity_id,purchase_order_id,purchase_order_line_id) -> purchase_order_lines(legal_entity_id,purchase_order_id,id)`、可空 `(legal_entity_id,purchase_order_id,purchase_order_line_id,purchase_order_line_batch_id) -> purchase_order_line_batches(legal_entity_id,purchase_order_id,purchase_order_line_id,id)`。索引：`pk_`；`ux_goods_receipt_lines_goods_receipt_id_line_no`；`ix_goods_receipt_lines_legal_entity_id_created_at`；`ix_goods_receipt_lines_legal_entity_id_purchase_order_line_id`；`ix_goods_receipt_lines_legal_entity_id_material_id_batch_no`。

##### 3.2.10 procure.goods_receipt_line_costings（GRNI 追加效果事实，仅追加）

本表复用既有收货行入账分配表，作为已收货未收票（GRNI）子账的唯一事实源；不另建第二张 GRNI 表，也不固化单价。收货入账单价的权威出处仍是 `inventory.stock_value_entries.applied_unit_price`；采购退货的库存账面金额不回查原入账单价，而由 `InventoryPostingPort::post_outbound` 按锁后当前移动加权账面价值及出清归零规则返回。本表只保存收货暂估及采购退货、进项发票、进项红字对该暂估产生的有方向数量/金额效果。按基线第 4 节，本表不带 `row_version`、`updated_at`、`updated_by`，所有效果只追加。

| 列 | 类型 | 可空 | 说明 |
|---|---|---|---|
| goods_receipt_line_id | uuid | 否 | 同 schema 外键；一条效果链始终落在同一收货行 |
| source_kind | text | 否 | `GOODS_RECEIPT`、`PURCHASE_RETURN`、`PURCHASE_INVOICE`、`PURCHASE_CREDIT_NOTE` |
| source_doc_line_id | uuid | 否 | 与 `source_kind` 组成封闭多态来源行引用；owner 按类型校验同法人目标及业务归属，不建伪外键 |
| direction | text | 否 | `INCREASE` 或 `DECREASE`；金额本身永远为正 |
| quantity | numeric(18,6) | 否 | CHECK 大于等于零；数量与金额至少一项大于零，允许纯金额红字 |
| amount | numeric(18,2) | 否 | CHECK 大于等于零；数量与金额至少一项大于零，MidpointAwayFromZero 到 2 位 |
| accounting_period_id | uuid | 否 | 取同一事务 `AccountingPeriodResolver::resolve` 的返回值 |
| accounting_period_seq | integer | 否 | 同一 `ResolvedPeriod` 的单调序号，用于截至期间聚合，不比较 UUID |
| posting_date | date | 否 | 来源业务事件记账日 |
| effect_seq | bigint | 否 | `GENERATED ALWAYS AS IDENTITY`；仅用于同一效果链的父子严格先后判定 |
| root_effect_id | uuid | 否 | 根行取自身 id；派生行沿用根 id |
| reverses_id | uuid | 是 | 根行为空；派生行指向同根、相反方向且早于本行的直接父效果 |

根行唯一合法形态为 `source_kind=GOODS_RECEIPT`、`direction=INCREASE`、`root_effect_id=id`、`reverses_id IS NULL`。派生行必须 `reverses_id IS NOT NULL`；`PURCHASE_RETURN` 与 `PURCHASE_INVOICE` 固定为 `DECREASE`，`PURCHASE_CREDIT_NOTE` 固定为 `INCREASE`。根自引用固定为 `(legal_entity_id,root_effect_id) -> (legal_entity_id,id) ON DELETE RESTRICT DEFERRABLE INITIALLY DEFERRED`；父引用固定为 `(legal_entity_id,goods_receipt_line_id,root_effect_id,reverses_id) -> (legal_entity_id,goods_receipt_line_id,root_effect_id,id) ON DELETE RESTRICT DEFERRABLE INITIALLY DEFERRED`，不得退化成单列或只带法人的短父 FK。应用先预生成 UUIDv7 `id`，根以一次 INSERT 同时写 `root_effect_id=id`，无需先插 NULL 再更新；延迟根 FK 使空表第一条根能在事务 COMMIT 时合法成立。DEFERRABLE constraint trigger 断言派生行与父行同法人、同收货行、同根、方向相反、`parent.effect_seq < child.effect_seq`，且链无环。禁止用 `created_at` 比较父子先后，同一事务的数据库时钟值允许相同。每个根在每次事务提交点同时满足 `net_quantity=ΣINCREASE.quantity-ΣDECREASE.quantity` 位于 `0..=root.quantity`、`net_amount=ΣINCREASE.amount-ΣDECREASE.amount` 位于 `0..=root.amount`；并按每个直接父限制反向子行累计不超过父行尚未被反向的数量和金额。根与派生效果均不可更新或删除。

为使复合外键可创建，本表增加 `UNIQUE(legal_entity_id,id)` 与 `UNIQUE(legal_entity_id,goods_receipt_line_id,root_effect_id,id)`；来源幂等键固定使用 PostgreSQL 16 的 `UNIQUE NULLS NOT DISTINCT (legal_entity_id,source_kind,source_doc_line_id,reverses_id)`，使 `reverses_id IS NULL` 的根行也不能重复。不得以普通 `UNIQUE`、应用层先查后插或另一套部分索引替代该唯一约束。初始收货只把库存契约返回的暂估部分写成 `GOODS_RECEIPT/INCREASE`；超量开票反向匹配部分不产生 GRNI，其关系仍由 `finance.overbilling_settlements` 与库存金额流水承载。零价且零暂估金额的收货仍必须写 `quantity>0, amount=0` 的 GRNI 根，使后续发票、红字与退货共享同一数量父链；金额为零只使总账跳过零金额腿，不得省略 GRNI 数量事实。

部分发票或退货减少额以开放根为基础：`remaining_quantity` 与 `remaining_amount` 均在锁后按上述有向累计重算；非末次金额取 `round(root.amount * effect_quantity / root.quantity, 2, MidpointAwayFromZero)` 并封顶为剩余金额，吃完剩余数量的末次效果直接取全部剩余金额以吸收舍入尾差。只有进项红字行的 `quantity_effect_kind=REDUCE` 部分，才按原进项发票产生的 `PURCHASE_INVOICE/DECREASE` 效果逐条追加 `PURCHASE_CREDIT_NOTE/INCREASE`；重开数量与金额由原 GRNI 父效果按数量比例在服务端计算，累计不得超过该父效果尚未被反向的余额。`NONE+ADJUSTED` 的折让、纯金额或纯税额更正不改变“货已收但未开票”的数量事实，不写 GRNI 效果；其成本或税额差异只走库存/价差与税额计量项。`REDUCE+ADJUSTED` 只按减少数量重开原暂估金额，红字净额与原暂估金额的差额仍走库存/价差，不得把红字票面金额直接当 GRNI 金额。

采购退货的 GRNI 分流固定如下，不允许调用方自行选择：未开票部分直接对开放的 `GOODS_RECEIPT/INCREASE`（或其后续开放 INCREASE）追加 `PURCHASE_RETURN/DECREASE`；已开票部分先由进项红字在同一事务追加 `PURCHASE_CREDIT_NOTE/INCREASE`，采购用例随后逐条以该新增效果为直接父行追加等数量、等金额的 `PURCHASE_RETURN/DECREASE`。因此“收货 100 → 发票冲减 100 → 红字重开 100 → 实物退货冲减 100”的 GRNI 终值严格为零，四个事件仍各自落在真实会计期间。一次退货同时含已开票与未开票数量时先按收货行拆段，再按上述两条路径分别落效果；任何一段缺少可追溯父效果、金额不相等或事务末根净额越界，整笔退货回滚。

阶段 10 使用的 owner port 签名冻结为：

```rust
#[async_trait::async_trait]
pub trait GrniEffectWritebackPort: Send + Sync {
    async fn lock_candidates(
        &self,
        tx: &mut dyn Tx,
        ctx: &SecurityContext,
        goods_receipt_line_ids: &[uuid::Uuid],
    ) -> Result<Vec<GrniLockCandidate>, AppError>;

    async fn decrease_for_purchase_invoice(
        &self,
        tx: &mut dyn Tx,
        ctx: &SecurityContext,
        f50_lock_proof: &TransactionLockProof,
        purchase_invoice_id: Id<PurchaseInvoice>,
        accounting_period_id: Id<AccountingPeriod>,
        accounting_period_seq: i32,
        posting_date: NaiveDate,
        lines: &[GrniInvoiceMatch],
    ) -> Result<GrniWritebackOutcome, AppError>;

    async fn increase_for_purchase_credit_note(
        &self,
        tx: &mut dyn Tx,
        ctx: &SecurityContext,
        f50_lock_proof: &TransactionLockProof,
        purchase_credit_note_id: uuid::Uuid,
        accounting_period_id: Id<AccountingPeriod>,
        accounting_period_seq: i32,
        posting_date: NaiveDate,
        lines: &[GrniCreditNoteReversal],
    ) -> Result<GrniWritebackOutcome, AppError>;
}

pub struct GrniLockCandidate {
    pub goods_receipt_line_id: uuid::Uuid,
    pub root_effect_id: uuid::Uuid,
    pub effect_id: uuid::Uuid,
}

pub struct GrniInvoiceMatch {
    pub purchase_invoice_line_id: uuid::Uuid,
    pub goods_receipt_line_id: uuid::Uuid,
    pub matched_quantity: Quantity,
}

pub struct GrniCreditNoteReversal {
    pub purchase_credit_note_line_id: uuid::Uuid,
    pub original_purchase_invoice_line_id: uuid::Uuid,
    pub reversed_quantity: Quantity,
}

pub enum GrniEffectDirection { Increase, Decrease }

pub struct GrniWritebackEffect {
    pub source_doc_line_id: uuid::Uuid,
    pub goods_receipt_line_id: uuid::Uuid,
    pub root_effect_id: uuid::Uuid,
    pub parent_effect_id: uuid::Uuid,
    pub effect_id: uuid::Uuid,
    pub direction: GrniEffectDirection,
    pub quantity: Quantity,
    pub amount: Money,
}

pub struct GrniWritebackOutcome {
    pub effects: Vec<GrniWritebackEffect>,
    pub total_amount: Money,
}
```

两种输入 DTO、三个方法与三个返回 DTO 的字段集合、类型和顺序以上述代码块为唯一契约，不接收调用方传入的 GRNI 金额。`lock_candidates` 是无锁、无写的计划收集方法：空收货行数组返回空集合且不访问数据库，非空时返回这些同法人收货行的全部现存 GRNI 效果，固定按 `(goods_receipt_line_id,root_effect_id,effect_id) ASC` 排序且无重复；调用方在协调器前与全部类别锁后各调一次，用两次集合构建 collected/reloaded plan，不能把它当成锁或容量结论。两个 mutator 收到空 `lines` 必须以 `PLATFORM.REQUEST.INVALID_PAYLOAD` 拒绝且零写入，不得把空集合解释成成功；成功结果的 `effects` 必须非空，固定按 `(source_doc_line_id,goods_receipt_line_id,root_effect_id,parent_effect_id,effect_id) ASC` 排序，`total_amount` 必须等于返回 `effects.amount` 的服务端求和。`decrease_for_purchase_invoice` 的每项 `direction=Decrease` 且 `source_doc_line_id=purchase_invoice_line_id`；`increase_for_purchase_credit_note` 的每项 `direction=Increase` 且 `source_doc_line_id=purchase_credit_note_line_id`。两个 mutator 首句必须经 `CrossModuleLockCoordinator::assert_covers` 验证同一 `TransactionLockProof` 覆盖收货行与全部 `(root_effect_id,effect_id)`；随后只锁后重读，不执行新的 `FOR UPDATE`、advisory lock 或中途补锁。期间三项只接受同一个 `ResolvedPeriod` 与本次记账日，调用方不得自算 seq。采购退货属 procure 自有用例：未开票段直接追加 `PURCHASE_RETURN/DECREASE`；已开票段消费 `PurchaseCreditNoteView.grni_reopened_effects`，逐条以其中的 GRNI 效果 id 为父追加等额 `PURCHASE_RETURN/DECREASE`，不重复调用红字重开端口。

`ep-app-procure` 另在 `crates/application/procure/src/f50_lock_slice.rs` 实现 procure owner 的 `F50LockSlicePort(owner=Procure)`，只承接 `PurchaseReturn`、`GoodsReceiptLine`、`GrniEffect` 与 `PayableReservation` 四类：前三类按 F-50 顺序和候选键真实 `FOR UPDATE`；PayableReservation 先按 `(legal_entity_id,purchase_invoice_id)` 升序取得 `payable-reservation:` transaction advisory lock，再锁已存在行，覆盖尚未建行。该 SPI 不计算金额、不追加 GRNI、不改变采购状态；任何 procure mutator 只能验证已 seal proof，不能自行调用 SPI 补锁。

`GrniSubledgerBalanceQuery` 只读本表，在同一 `SnapshotCtx` 内按 `legal_entity_id` 与 `accounting_period_seq <= target_seq` 聚合 `INCREASE.amount-DECREASE.amount`；结果必须大于等于零。查询不得访问 invoice schema，不得读取今天的 `invoiced_quantity` 倒推历史。候选键除 `ux_goods_receipt_line_costings_legal_entity_id_id` 外另建 `UNIQUE(legal_entity_id,goods_receipt_line_id,id)`，供采购退货证明所消费效果确属同一收货行。其余索引：`pk_`；`ux_goods_receipt_line_costings_le_source_parent`；`ix_goods_receipt_line_costings_legal_entity_id_created_at`；`ix_goods_receipt_line_costings_legal_entity_id_goods_receipt_line_id`；`ix_goods_receipt_line_costings_le_period_direction` 列为 `(legal_entity_id,accounting_period_seq,direction)`。

##### 3.2.11 procure.goods_receipt_line_serials

列为 `goods_receipt_line_id uuid not null`（同 schema 外键）、`serial_no text not null`（CHECK 长度不超过 64）。索引：`pk_`；`ix_goods_receipt_line_serials_legal_entity_id_created_at`；`ix_goods_receipt_line_serials_legal_entity_id_serial_no`。本表不建序列号唯一约束，理由是同一序列号在退货后可再次收货，法人内唯一的在库判定由库存模块的序列号台账承担。

`V20261018091000__procure_create_goods_receipt_line_costings.sql` 在收货头、行、序列与 GRNI 表齐备后建立 `procure.assert_goods_receipt_effect_graph_consistent()`，并在 `goods_receipts`、`goods_receipt_lines`、`goods_receipt_line_serials`、`goods_receipt_line_costings`、`purchase_order_lines` 与 `purchase_order_line_batches` 上安装同一套 `DEFERRABLE INITIALLY DEFERRED` 约束触发器，迁移末尾才启用。函数按订单行、批次、收货行和效果 id 稳定锁读并强制以下提交点图：

1. 收货头 `supplier_id` 必须等于采购订单供应商；至少在 POSTED/PARTIALLY_RETURNED/FULLY_RETURNED 三态有一条行。每行通过长复合外键属于该头的同一订单，`material_id/order_unit_price_untaxed/warehouse_id` 等于订单行；批次存在时必须属于同一订单行，不虚构批次表不存在的仓库列；序列集合与库存数量事实逐值一致。
2. 仅三种已过账状态计入累计：每条订单行的 `received_quantity` 等于全部已过账收货行数量合计，每个批次的 `received_quantity` 等于指向该批次的已过账行数量合计。累计超过订单/批次数量时，本次收货头必须 `has_over_receipt=true`、原因非空，且 `over_receipt_approval_ref` 指向同法人、同命令摘要、全部节点通过且申请人未自审的审批证据；未超收时三项必须为 `false/NULL/NULL`。合法审批超收不得被简单的 `received<=ordered` CHECK 拦截。
3. DRAFT/PENDING_APPROVAL 不得已有库存 movement、GOODS_RECEIPT GRNI 根或 voucher。已过账头必须恰有一条同法人 `inventory.stock_movements`，其 `direction/reason/source_doc_type/source_module/source_doc_id` 为 `IN/PURCHASE_RECEIPT/PURCHASE_RECEIPT/procure/goods_receipts.id`，期间与业务日期等于头；其 qty/value/serial 段按 `source_doc_line_id`、物料、仓库、批次、序列与每条收货行精确配对。每个 ESTIMATED_PO_PRICE 段恰有一条 `GOODS_RECEIPT/INCREASE` GRNI 根且 quantity/amount/period/date 与库存段相等；OVERBILL_INVOICE_PRICE 段不得伪造 GRNI 根，其 `overbilling_entry_id` 与匹配段唯一且金额逐值相等。
4. POSTED 要求全部行 `returned_quantity=0`；PARTIALLY_RETURNED 要求至少一行已退且至少一行仍有未退数量；FULLY_RETURNED 要求每行 `returned_quantity=quantity`。三态 `accounting_period_id/posted_at` 非空；定义 `inventory_amount` 为本 movement 全部 value 段金额、`grni_amount` 为本次 GOODS_RECEIPT 根金额、`match_amount` 为 OVERBILL_INVOICE_PRICE 段金额，`voucher_id IS NULL` 当且仅当三项都为零。任一非零时 voucher 必须非空，且为同法人 `source_kind=PURCHASE_RECEIPT`、`source_document_type='GOODS_RECEIPT'`、`source_document_id=goods_receipts.id`、同期间的本次普通凭证；三项全零时携带凭证同样拒绝。

任一普通 FK 都命中但跨订单/批次/收货头拼接、错供应商、错库存来源行、GRNI 与库存段错配、超收无有效审批、状态半图或零金额 voucher 反向形状均在 `SET CONSTRAINTS ALL IMMEDIATE`/COMMIT 失败且零部分效果。回退先删六表触发器与函数，再删 GRNI 表；不得因函数跨 schema 读取 ledger/inventory 而把它降级为应用断言。

##### 3.2.12 procure.receipt_rejections（拒收记录，单据类）

列为 `supplier_id uuid not null`（同法人复合外键指向 `mdm.suppliers`）、`purchase_order_id uuid not null`（同法人复合外键）、`purchase_order_line_id uuid not null`（同法人复合外键）、`delivery_notice_id uuid null`（由 `V20261018093200__procure_add_portal_foreign_keys.sql` 补建指向 `portal.delivery_notices` 的同法人复合外键）、`rejected_quantity numeric(18,6) not null`（CHECK 大于零）、`reason_code text not null`、`reason_text text null`、`rejected_on date not null`。`status` CHECK 取值 `REGISTERED`。本单据不产生库存流水与凭证。索引：`pk_`；`ux_receipt_rejections_legal_entity_id_doc_no`；`ix_receipt_rejections_legal_entity_id_created_at`。

##### 3.2.13 procure.purchase_returns（采购退货单，单据类）

| 列 | 类型 | 可空 | 说明 |
|---|---|---|---|
| supplier_id | uuid | 否 | 与法人组成复合外键指向 `mdm.suppliers(legal_entity_id,id)` |
| return_scenario | text | 否 | CHECK 取值 `MATERIAL_RECEIPT`、`DROP_SHIP` |
| sales_return_id | uuid | 是 | `DROP_SHIP` 时必填；与法人组成复合外键指向 `sales.sales_returns(legal_entity_id,id)`，由 CHECK 表达形状 |
| sales_return_doc_no | text | 是 | 只读展示用 |
| warehouse_id | uuid | 是 | `MATERIAL_RECEIPT` 时必填；与法人组成复合外键指向 `mdm.warehouses(legal_entity_id,id)` |
| posting_date | date | 否 | 该业务事件的记账日期，取值即退货日期 |
| accounting_period_id | uuid | 是 | 过账时由 `ep_contract_ledger::AccountingPeriodResolver::resolve` 解析并写入；与法人组成复合外键指向 `ledger.accounting_periods(legal_entity_id,id)` |
| physical_return_voucher_id | uuid | 是 | 仅物料退货过账时由 `ep_contract_ledger::PostingPort::post` 返回并写入；与法人组成复合外键指向 `ledger.vouchers(legal_entity_id,id)`，直运/直接费用恒空 |
| reason_code | text | 否 | 退货原因字典码 |
| approval_ref | uuid | 是 | |
| posted_at / voided_at | timestamptz | 是 | |

`status` CHECK 取值 `DRAFT`、`PENDING_APPROVAL`、`REJECTED`、`POSTED`、`VOIDED`。NULL-safe `ck_purchase_returns_posting_shape` 强制：非 POSTED 时 `accounting_period_id/physical_return_voucher_id/posted_at` 全空；MATERIAL_RECEIPT POSTED 时 `accounting_period_id/posted_at` 非空而物理 voucher 由延迟效果图按三项金额判定；DROP_SHIP POSTED 时期间与 posted_at 非空、physical voucher 恒空。链接进项红字凭证不复制进本表，按 `invoice.invoice_reversals.linked_purchase_return_id` 反查；过账事务仍把当次返回的全部 id 放进事件。不得持久化头级 `is_invoice_registered`：同一退货可按行、甚至同一收货行拆成已开票与未开票段，单一布尔值没有合法含义。列表或详情需要摘要时，由查询服务根据锁后匹配与已落效果导出只读 `invoice_match_summary=NONE|PARTIAL|ALL`，该值不是数据库列、不是命令入参，也不参与后续分支判定。候选键为 `UNIQUE(legal_entity_id,id)` 与 DROP_SHIP 归属使用的 `UNIQUE(legal_entity_id,sales_return_id,id)`；索引：`pk_`；`ux_purchase_returns_legal_entity_id_doc_no`；`ix_purchase_returns_legal_entity_id_created_at`；`ix_purchase_returns_legal_entity_id_supplier_id_posting_date`。

##### 3.2.14 procure.purchase_return_lines

列为 `purchase_return_id uuid not null`、`line_no integer not null`、`goods_receipt_id uuid null`、`purchase_order_id uuid null`、`purchase_order_line_id uuid null`（三项为 MATERIAL_RECEIPT 的不可写冗余祖先键）、`goods_receipt_line_id uuid null`（MATERIAL_RECEIPT 时必填）、`goods_receipt_line_costing_id uuid null`（指向被回冲且确属同一收货行的入账分配）、`purchase_invoice_id uuid null`、`purchase_invoice_line_id uuid null`（DROP_SHIP 原直接费用发票头行，由 `V20261019090930__procure_add_invoice_foreign_keys.sql` 补长复合外键）、`sales_return_id uuid null`（冗余自退货头）、`sales_return_line_id uuid null`（仅 DROP_SHIP）、`material_id uuid null`（同法人复合外键指向 `mdm.materials`）、`quantity numeric(18,6) not null`（CHECK 大于零）、`batch_no text not null default '-'`、`reversal_unit_price numeric(18,6) null`（过账时由库存契约返回并固化）、`reversal_amount numeric(18,2) null`。候选键 `UNIQUE(legal_entity_id,purchase_return_id,id)`；真实 `ON DELETE RESTRICT` 长复合外键为 `(legal_entity_id,purchase_return_id) -> purchase_returns(legal_entity_id,id)`、MATERIAL_RECEIPT 的 `(legal_entity_id,goods_receipt_id,purchase_order_id,purchase_order_line_id,goods_receipt_line_id) -> goods_receipt_lines(legal_entity_id,goods_receipt_id,purchase_order_id,purchase_order_line_id,id)` 与 `(legal_entity_id,goods_receipt_line_id,goods_receipt_line_costing_id) -> goods_receipt_line_costings(legal_entity_id,goods_receipt_line_id,id)`，以及 DROP_SHIP 的 `(legal_entity_id,sales_return_id,sales_return_line_id) -> sales.sales_return_lines(legal_entity_id,sales_return_id,id)`。NULL-safe 形状 CHECK 要求 MATERIAL_RECEIPT 的 receipt/order/material 字段全非空而 invoice/sales-return 字段全空；DROP_SHIP 的 sales-return 字段全非空而 receipt/order/material 字段全空，`purchase_invoice_id/purchase_invoice_line_id` 同空同非空且在 DRAFT/PENDING_APPROVAL 可空、POSTED 必须非空。索引：`pk_`；`ux_purchase_return_lines_purchase_return_id_line_no`；`ux_purchase_return_lines_purchase_return_sales_line`；`ix_..._legal_entity_id_created_at`；`ix_purchase_return_lines_legal_entity_id_goods_receipt_line_id`。

##### 3.2.15 procure.purchase_return_line_serials

列为 `purchase_return_id uuid not null`（冗余祖先键，客户端不可写）、`purchase_return_line_id uuid not null`、`serial_no text not null`。以 `(legal_entity_id,purchase_return_id,purchase_return_line_id) -> purchase_return_lines(legal_entity_id,purchase_return_id,id) ON DELETE RESTRICT` 长复合外键锁定同一退货头行；索引同 3.2.11 的模式。

`V20261018091400__procure_create_purchase_return_line_serials.sql` 在退货头行/序列齐备后建立 `procure.assert_purchase_return_effect_graph_consistent()`，并在 `purchase_returns`、`purchase_return_lines`、`purchase_return_line_serials`、`goods_receipts`、`goods_receipt_lines` 与 `goods_receipt_line_costings` 上安装 `DEFERRABLE INITIALLY DEFERRED` 约束触发器，迁移末尾才启用。提交点固定以下图：PENDING_APPROVAL/POSTED 至少一行；头供应商、场景、仓库或 sales return 与所有行同源。MATERIAL_RECEIPT 行的收货头、采购订单、供应商、订单行、物料、批次和序列逐值一致，所选 costing 通过长键确属该收货行；DROP_SHIP 行必须通过长键属于头上的同一 `sales_return_id`，且销售退货行客户/订单/产品与原直运采购链一致。非 POSTED 时 `reversal_unit_price/reversal_amount` 及本退货的 GRNI、inventory、physical voucher 效果全空。

MATERIAL_RECEIPT POSTED 必须恰有一条 `OUT/PURCHASE_RETURN/PURCHASE_RETURN/procure` 且 `source_doc_id=purchase_returns.id` 的库存 movement；每条退货行恰与其 qty/value/serial 段按 source line、物料、仓库、批次、序列、数量一一配对，`reversal_unit_price/reversal_amount` 等于对应库存结果。每条本次 `PURCHASE_RETURN/DECREASE` GRNI 效果必须以该退货行作 `source_doc_line_id`、以本行所选同收货行 costing 链的开放父效果为父，期间与日期等于头，累计不得超父开放量额。定义 `grni_consumed_amount` 为这些 decrease 金额合计、`inventory_return_amount` 为库存 value 金额合计、`return_carrying_difference_amount=inventory_return_amount-grni_consumed_amount`；三项全零时 `physical_return_voucher_id` 必须为空，任一非零时必须非空且指向同法人、同期间、`source_kind=PURCHASE_RETURN_INVENTORY`、`source_document_type='PURCHASE_RETURN'`、`source_document_id=purchase_returns.id` 的本次普通凭证。DROP_SHIP POSTED 不得存在物理 inventory/GRNI/voucher 效果，`physical_return_voucher_id/reversal_*` 恒空；其红字效果由第 19090930 号追补后的 invoice 图校验。

函数同时强制收货行/单与采购订单行的 `returned_quantity` 等于全部 POSTED MATERIAL_RECEIPT 退货累计，并据此保持 goods receipt 的 POSTED/PARTIALLY_RETURNED/FULLY_RETURNED 状态。普通 FK 全命中但错 costing、错收货头/供应商、错 sales return、错库存来源段、半效果图与零金额反向 voucher 均在提交时失败且零部分写入。回退先删六表触发器与函数，再删退货序列表。

目标发票表建成后，`V20261019090930__procure_add_invoice_foreign_keys.sql` 把 `(legal_entity_id,purchase_invoice_id,purchase_invoice_line_id)` 以 `ON DELETE RESTRICT` 真实复合外键指向 `invoice.purchase_invoice_lines(legal_entity_id,purchase_invoice_id,id)`，而不是只按 line id 命中任意发票。该追补先做同法人、头行和 DROP_SHIP 祖先预检，再 `CREATE OR REPLACE procure.assert_purchase_return_effect_graph_consistent()` 扩展上述延迟函数，并在 `procure.purchase_returns`、`procure.purchase_return_lines`、`invoice.invoice_reversals`、`invoice.invoice_reversal_lines` 四表安装或替换 `DEFERRABLE INITIALLY DEFERRED` 约束触发器，保证先写任一侧都在提交时复核完整双向图。原票头供应商必须等于退货供应商，原票行 `cost_kind=DIRECT_EXPENSE_TYPE` 且采购订单/订单行必须是同一条直运采购链；`sales_return_line_id` 必须属于头上同一销售退货并对应该链的原销售订单行。POSTED DROP_SHIP 每条行必须由 `linked_purchase_return_id=本退货` 的进项红字行完整覆盖且累计不超原票开放量，反向凭证来源为 `PURCHASE_INVOICE_DIRECT_EXPENSE_REVERSED`；MATERIAL_RECEIPT 的已开票分段则由 `PURCHASE_INVOICE_LINKED_RETURN_REVERSED` 覆盖，红字重开的 GRNI 父效果与本退货 decrease 一一对应。错原票、错供应商、错 sales return、缺/多/跨退货红字均在提交时整笔失败。追补成功前相关已开票/DROP_SHIP 写入口不启用；回退先删除本文件在 invoice/procure 四表安装的触发器，再恢复 `V20261018091400` 的旧函数体并重建其原六表约束触发器（包括退货头行），最后删除两条追补外键，绝不留下“旧函数已恢复但原触发器缺失”的半回退态。

##### 3.2.16 procure.payment_requests（付款申请，单据类）

| 列 | 类型 | 可空 | 说明 |
|---|---|---|---|
| supplier_id | uuid | 否 | 与法人组成复合外键指向 `mdm.suppliers(legal_entity_id,id)` |
| payment_type | text | 否 | CHECK 取值 `INVOICE_PAYMENT`、`PREPAYMENT` |
| requested_amount | numeric(18,2) | 否 | CHECK 大于零 |
| planned_payment_date | date | 否 | CHECK 不早于申请日期 |
| requested_on | date | 否 | 申请日期 |
| payee_account_ref | uuid | 否 | 与法人组成复合外键指向 `mdm.supplier_payment_profiles(legal_entity_id,id)`，不复制账号明文 |
| request_note | text | 是 | 长度不超过 2000 |
| paid_amount | numeric(18,2) | 否 | 默认 0，由财务模块经契约回写 |
| approval_ref | uuid | 是 | |
| close_reason | text | 是 | |
| withdrawn_at / closed_at / voided_at | timestamptz | 是 | |

`status` CHECK 取值 `DRAFT`、`PENDING_APPROVAL`、`REJECTED`、`WITHDRAWN`、`APPROVED`、`PARTIALLY_PAID`、`FULLY_PAID`、`CLOSED`、`VOIDED`。表级 CHECK：`ck_payment_requests_paid_le_requested`。索引：`pk_`；`ux_payment_requests_legal_entity_id_doc_no`；`ix_payment_requests_legal_entity_id_created_at`；`ix_payment_requests_legal_entity_id_supplier_id_status`；`ix_payment_requests_legal_entity_id_status_planned_payment_date`。第四条与第五条支撑财务的待付款队列取数。

本表不存银行账号明文，只存 `payee_account_ref`，因此不承载规格第 7.8 章的行内敏感字段，字段级密级与解密路径仍在 mdm 侧判定。

##### 3.2.17 procure.payment_request_lines

列为 `payment_request_id uuid not null`（同法人复合外键）、`line_no integer not null`、`ref_type text not null`（CHECK 取值 `PURCHASE_INVOICE`、`CONTRACT`、`PURCHASE_ORDER`）、`ref_id uuid not null`（与 `ref_type` 组成封闭多态引用，由写入用例按类型校验发票/合同/订单的同法人、供应商与状态，不建伪外键）、`ref_doc_no text null`、`requested_amount numeric(18,2) not null`（CHECK 大于零）、`paid_amount numeric(18,2) not null default 0`（CHECK `0 <= paid_amount <= requested_amount`，只由下述 owner writeback 累计/冲回）。头级 `payment_requests.paid_amount` 必须等于本单全部行 `paid_amount` 合计；该跨行等式由同一 owner 事务重读断言和 R-PROC-04 共同守护。索引：`pk_`；`ux_payment_request_lines_payment_request_id_line_no`；`ix_..._legal_entity_id_created_at`；`ix_payment_request_lines_legal_entity_id_ref_type_ref_id`。

##### 3.2.18 procure.payable_reservations（应付占用汇总）

本表是 PRD 第 4.7.5 小节「同一张采购发票被多张未关闭的付款申请重复引用时按各申请已占用金额合计校验」的可串行化落点。

列为 `purchase_invoice_id uuid not null`（由 `V20261019090930__procure_add_invoice_foreign_keys.sql` 补建指向 `invoice.purchase_invoices(legal_entity_id,id)` 的同法人复合外键）、`reserved_amount numeric(18,2) not null default 0`（CHECK 大于等于零）、`open_request_count integer not null default 0`（CHECK 大于等于零），以及基线公共 `row_version/updated_at/updated_by`。约束：`ux_payable_reservations_legal_entity_id_purchase_invoice_id`；表级 CHECK 另断言 `open_request_count=0` 时 `reserved_amount=0`。索引：`pk_`；`ix_payable_reservations_legal_entity_id_created_at`。

##### 3.2.19 附件关联表

按基线第 4 节命名，五张表结构一致，列为 `owner_id uuid not null`（与法人组成复合外键指向各自主表）、`attachment_object_id uuid not null`（与法人组成复合外键指向 `platform_file.attachment_objects(legal_entity_id,id)`）、`purpose text not null`、`sort_no integer not null`。

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
| identity_principal_id | uuid | 否 | 真实单列外键指向 `platform_core.user_accounts(id)`；绑定事务校验 `account_kind='PORTAL'`、账号有效且具备当前法人授权 |
| supplier_id | uuid | 否 | 与法人组成复合外键指向 `mdm.suppliers(legal_entity_id,id)` |
| capabilities | text[] | 否 | 默认 `'{}'`，取值只允许五项能力码，由 CHECK 表达 |
| binding_status | text | 否 | CHECK 取值 `INVITED`、`PENDING_REVIEW`、`ACTIVE`、`SUSPENDED`、`DISABLED` |
| invited_by | uuid | 是 | 与法人组成复合外键指向 `platform_authz.user_legal_entity_grants(legal_entity_id,user_id)` |
| invited_at / activated_at / disabled_at | timestamptz | 是 | |
| self_registered | boolean | 否 | 默认 false，标记该绑定来自受限自助注册 |

五项能力码为 `ORDER_CONFIRM`、`DELIVERY_NOTICE`、`INVOICE_UPLOAD`、`SETTLEMENT_QUERY`、`PROFILE_MAINTAIN`。CHECK 表达为 `capabilities <@ ARRAY['ORDER_CONFIRM','DELIVERY_NOTICE','INVOICE_UPLOAD','SETTLEMENT_QUERY','PROFILE_MAINTAIN']::text[]`。

一行代表一个法人下的一条授权，跨法人授权由多行表达；`identity_principal_id` 的单列外键只证明全局账号存在，写入事务还必须验证该账号具有本法人授权且绑定的 supplier 属于本法人，二者任一不成立即回滚。索引：`pk_`；`ux_supplier_portal_users_legal_entity_id_identity_principal_id`；`ix_supplier_portal_users_legal_entity_id_created_at`；`ix_supplier_portal_users_legal_entity_id_supplier_id_binding_status`。

##### 3.3.2 portal.delivery_notices（送货通知，单据类）

列为 `supplier_id uuid not null`（同法人复合外键指向 `mdm.suppliers`）、`purchase_order_id uuid not null`（同法人复合外键指向 `procure.purchase_orders`）、`purchase_order_doc_no text not null`、`expected_arrival_date date not null`、`carrier_name text null`、`waybill_no text null`、`remark text null`、`submitted_by_portal_user_id uuid not null`（同法人复合外键指向 `portal.supplier_portal_users`）、`voided_at timestamptz null`。`status` CHECK 取值 `SUBMITTED`、`PARTIALLY_RECEIVED`、`RECEIVED`、`VOIDED`。候选键 `UNIQUE(legal_entity_id,purchase_order_id,id)`；索引：`pk_`；`ux_delivery_notices_legal_entity_id_doc_no`；`ix_delivery_notices_legal_entity_id_created_at`；`ix_delivery_notices_legal_entity_id_supplier_id_status`；`ix_delivery_notices_legal_entity_id_purchase_order_id`。

##### 3.3.3 portal.delivery_notice_lines

列为 `delivery_notice_id uuid not null`、`purchase_order_id uuid not null`（冗余祖先键，客户端不可写）、`line_no integer not null`、`purchase_order_line_id uuid not null`、`purchase_order_line_batch_id uuid null`、`material_id uuid not null`（同法人复合外键指向 `mdm.materials`）、`quantity numeric(18,6) not null`（CHECK 大于零）、`batch_no text not null default '-'`、`received_quantity numeric(18,6) not null default 0`。候选键 `UNIQUE(legal_entity_id,purchase_order_id,purchase_order_line_id,delivery_notice_id,id)`；三条 `ON DELETE RESTRICT` 长复合外键分别为 `(legal_entity_id,purchase_order_id,delivery_notice_id) -> delivery_notices(legal_entity_id,purchase_order_id,id)`、`(legal_entity_id,purchase_order_id,purchase_order_line_id) -> procure.purchase_order_lines(legal_entity_id,purchase_order_id,id)`、可空 `(legal_entity_id,purchase_order_id,purchase_order_line_id,purchase_order_line_batch_id) -> procure.purchase_order_line_batches(legal_entity_id,purchase_order_id,purchase_order_line_id,id)`。表级 CHECK：`ck_delivery_notice_lines_received_le_quantity` 精确为 `0<=received_quantity<=quantity`。索引：`pk_`；`ux_delivery_notice_lines_delivery_notice_id_line_no`；`ix_..._legal_entity_id_created_at`；`ix_delivery_notice_lines_legal_entity_id_purchase_order_line_id`。

`V20261018092600__portal_create_delivery_notice_lines.sql` 同批建立 `portal.assert_delivery_notice_graph_consistent()`，并在 notice 头行安装延迟约束触发器：头供应商与单号快照必须等于同一采购订单，行物料与批次必须等于上述长键命中的订单行/批次，`SUM(lines.received_quantity)` 与后续收货链接回写一致；RECEIVED 要求全部行收满，PARTIALLY_RECEIVED 要求至少一行已收且至少一行未满，SUBMITTED 全部为零，VOIDED 不允许再增加累计。迁移末尾才启用，回退先删触发器与函数。

##### 3.3.4 portal.delivery_notice_line_serials

列为 `delivery_notice_line_id uuid not null`（同 schema 外键）、`serial_no text not null`。

##### 3.3.5 portal.supplier_invoice_uploads（发票上传记录，单据类）

| 列 | 类型 | 可空 | 说明 |
|---|---|---|---|
| supplier_id | uuid | 否 | 与法人组成复合外键指向 `mdm.suppliers(legal_entity_id,id)` |
| invoice_medium | text | 否 | CHECK 取值 `ELECTRONIC/PAPER` |
| number_scheme | text | 否 | CHECK 取值 `UNIFIED_20/LEGACY_CODE_NUMBER` |
| invoice_code | text | 是 | `UNIFIED_20` 必空；旧制必为 10 或 12 位 ASCII 数字 |
| invoice_no | text | 否 | 数电恰 20 位、旧制恰 8 位 ASCII 数字 |
| identifier_key | text | 否 | 数据库生成列：`number_scheme \|\| ':' \|\| coalesce(invoice_code,'') \|\| ':' \|\| invoice_no`，客户端不可写 |
| active_identifier_slot | text | 是 | 数据库生成列：`status IN ('UPLOADED','ACCEPTED')` 时等于 `identifier_key`，`RETURNED` 时为 NULL；客户端不可写 |
| issued_on | date | 否 | |
| ref_type | text | 否 | CHECK 取值 `PURCHASE_ORDER`、`GOODS_RECEIPT` |
| ref_id | uuid | 否 | 多态来源例外；owner 按 `ref_type` 在同一事务校验同法人、同供应商与可见状态，不伪造条件外键 |
| ref_doc_no | text | 否 | |
| net_amount | numeric(18,2) | 否 | 服务端逐行汇总，严格大于零 |
| tax_amount | numeric(18,2) | 否 | 服务端逐行汇总，大于等于零 |
| gross_amount | numeric(18,2) | 否 | 服务端逐行汇总，严格等于 `net_amount + tax_amount` |
| return_reason | text | 是 | 财务退回时填写 |
| accepted_purchase_invoice_id | uuid | 是 | 与法人组成复合外键指向 `invoice.purchase_invoices(legal_entity_id,id)`；目标晚建，由 `V20261019090830__portal_add_invoice_foreign_keys.sql` 添加，受理事务经 owner port 回写 |
| submitted_by_portal_user_id | uuid | 否 | 同 schema 外键 |

`status` CHECK 取值 `UPLOADED`、`RETURNED`、`ACCEPTED`。号码制式、代码与号码的组合使用 NULL-safe CHECK；`identifier_key` 与 `active_identifier_slot` 均为 `GENERATED ALWAYS AS ... STORED`，其中前者 `NOT NULL`，不得由不同客户端各自归一。状态形状 CHECK 固定为：`UPLOADED` 时 `return_reason`、`accepted_purchase_invoice_id` 均空；`RETURNED` 时 `return_reason` 为非空白文本且 `accepted_purchase_invoice_id` 为空；`ACCEPTED` 时 `return_reason` 为空且 `accepted_purchase_invoice_id` 非空。活动上传唯一性使用普通唯一约束 `UNIQUE(legal_entity_id,supplier_id,active_identifier_slot)`；PostgreSQL 16 的默认 `NULLS DISTINCT` 使多张 `RETURNED` 历史行互不冲突，不建立基线禁止的部分索引。因此命中时返回 `PORTAL.SUPPLIER_INVOICE_UPLOAD.DUPLICATED`，而 `RETURNED` 后可用相同票面标识创建一张新上传记录。另有 `ux_supplier_invoice_uploads_legal_entity_id_doc_no`、`ix_supplier_invoice_uploads_legal_entity_id_created_at` 与 `ix_supplier_invoice_uploads_legal_entity_id_status_issued_on`。

##### 3.3.6 portal.supplier_invoice_upload_lines

| 列 | 类型 | 可空 | 说明 |
|---|---|---|---|
| supplier_invoice_upload_id | uuid | 否 | 与法人组成复合外键指向上传头，`ON DELETE RESTRICT` |
| line_no | integer | 否 | 从 1 开始；同一上传头唯一 |
| purchase_order_id | uuid | 否 | 与法人组成复合外键指向采购订单 |
| purchase_order_line_id | uuid | 否 | 与法人、采购订单组成复合外键指向该订单行 |
| goods_receipt_id | uuid | 是 | 与收货行同空或同非空；非空时为同法人真实复合外键 |
| goods_receipt_line_id | uuid | 是 | 与收货头、法人组成复合外键指向该收货行 |
| cost_kind | text | 否 | CHECK 取值 `INVENTORY_TYPE/DIRECT_EXPENSE_TYPE`；同一头全部行必须同值 |
| item_id | uuid | 是 | 物料类必填并与法人组成复合外键指向物料；直接费用类可空 |
| quantity | numeric(18,6) | 否 | 严格大于零 |
| net_unit_price | numeric(18,6) | 否 | 严格大于零 |
| tax_rate | numeric(9,6) | 否 | 0 至 1 闭区间，可逐行不同 |
| net_amount | numeric(18,2) | 否 | 严格大于零 |
| tax_amount | numeric(18,2) | 否 | 大于等于零；使用 F-50 唯一 half-up/0.02 校验 |
| gross_amount | numeric(18,2) | 否 | 严格大于零且等于 `net_amount + tax_amount` |

本表使用 `UNIQUE(legal_entity_id,supplier_invoice_upload_id,line_no)`，并为头复合外键建立目标候选键。`DEFERRABLE INITIALLY DEFERRED` 约束触发器在提交前强制每个头至少一行、全部行同 `cost_kind`、头三项金额等于行合计、采购订单/收货/物料引用属于同一法人且互相匹配；直接费用行不得填写收货或物料，物料行必须填写物料。索引另含 `ix_supplier_invoice_upload_lines_legal_entity_id_purchase_order_line_id`、`ix_supplier_invoice_upload_lines_legal_entity_id_goods_receipt_line_id` 与时间序索引。HTTP、门户管道和后续受理只通过同一 F-50 行验证器写入，不存在头级税率或旧 `untaxed_amount` 路径。

##### 3.3.7 附件关联表

`portal.delivery_notice_attachments` 与 `portal.supplier_invoice_upload_attachments`，结构同第 3.2.19 小节。

#### 3.4 迁移编号与顺序

迁移文件放在 `db/migrations/procure/` 与 `db/migrations/portal/`，迁移历史统一落在全局唯一的 `platform_core.schema_history`。执行顺序由单一全局 Runner 按文件版本号全序排定，不存在任何模块顺序声明文件；可在建表时满足的外键直接内联，指向本阶段后建 portal 表的三条外键由第 33 号追补，指向阶段 10/12 后建目标的三组外键由第 3.1 节冻结的后续精确迁移追补。

本阶段占用两段迁移时间窗，段内按下表顺序执行。每个文件只做一件事，每个文件头必须带 `-- rollback:` 段。

| 序 | 文件名 | 回退说明 |
|---|---|---|
| 1 | V20261018090000__procure_create_supplier_admissions.sql | drop table |
| 2 | V20261018090100__procure_create_supplier_quality_records.sql | drop table |
| 3 | V20261018090200__procure_create_purchase_requisitions.sql | drop table |
| 4 | V20261018090300__procure_create_purchase_orders.sql | drop table；建头候选键 |
| 5 | V20261018090400__procure_create_purchase_order_lines.sql | drop table；建累计范围/行状态 CHECK 与头行候选键 |
| 6 | V20261018090500__procure_create_purchase_order_line_batches.sql | 先删三表延迟触发器/函数再 drop table；建订单头行批次金额、数量与累计图 |
| 7 | V20261018090600__procure_create_purchase_order_payment_plans.sql | drop table |
| 8 | V20261018090700__procure_create_goods_receipts.sql | drop table；建 `ck_goods_receipts_posting_shape`，草稿/待审批三项全空，三种已过账态期间/posted_at 非空而 voucher 交延迟效果图判定 |
| 9 | V20261018090800__procure_create_goods_receipt_lines.sql | drop table；建收货头/订单行/批次长复合外键与候选键 |
| 10 | V20261018090900__procure_create_goods_receipt_line_serials.sql | drop table |
| 11 | V20261018091000__procure_create_goods_receipt_line_costings.sql | 先删六表延迟触发器/函数再 drop table；内联根/长父 `ON DELETE RESTRICT DEFERRABLE INITIALLY DEFERRED` 自 FK，建收货祖先、累计、库存/GRNI、零金额 voucher 效果图 |
| 12 | V20261018091100__procure_create_receipt_rejections.sql | drop table |
| 13 | V20261018091200__procure_create_purchase_returns.sql | drop table；建场景/过账/零金额 physical voucher 头形状与候选键 |
| 14 | V20261018091300__procure_create_purchase_return_lines.sql | drop table；建退货→收货/成本效果/销售退货长复合外键和场景形状 |
| 15 | V20261018091400__procure_create_purchase_return_line_serials.sql | 先删六表延迟触发器/函数再 drop table；建退货祖先、库存/GRNI/物理凭证与序列效果图 |
| 16 | V20261018091500__procure_create_payment_requests.sql | drop table |
| 17 | V20261018091600__procure_create_payment_request_lines.sql | drop table |
| 18 | V20261018091700__procure_create_payable_reservations.sql | drop table |
| 19 | V20261018091800__procure_create_purchase_order_attachments.sql | drop table |
| 20 | V20261018091900__procure_create_goods_receipt_attachments.sql | drop table |
| 21 | V20261018092000__procure_create_purchase_return_attachments.sql | drop table |
| 22 | V20261018092100__procure_create_payment_request_attachments.sql | drop table |
| 23 | V20261018092200__procure_create_receipt_rejection_attachments.sql | drop table |
| 24 | V20261018092300__procure_backfill_append_only_registry.sql | delete 本次插入的登记行 |
| 25 | V20261018092400__portal_create_supplier_portal_users.sql | drop table |
| 26 | V20261018092500__portal_create_delivery_notices.sql | drop table |
| 27 | V20261018092600__portal_create_delivery_notice_lines.sql | drop table |
| 28 | V20261018092700__portal_create_delivery_notice_line_serials.sql | drop table |
| 29 | V20261018092800__portal_create_delivery_notice_attachments.sql | drop table |
| 30 | V20261018092900__portal_create_supplier_invoice_uploads.sql | drop table |
| 31 | V20261018093000__portal_create_supplier_invoice_upload_lines.sql | drop table |
| 32 | V20261018093100__portal_create_supplier_invoice_upload_attachments.sql | drop table |
| 33 | V20261018093200__procure_add_portal_foreign_keys.sql | 删除本文件新增的三条长复合外键、四表延迟触发器与函数；补齐收货头/行/拒收对 notice 的同订单祖先外键及收货-notice 图 |

三十三个文件中三十一个为新增空表、第 24 号为登记回填、第 33 号为目标晚建后的外键追补，全部使用常规逐文件事务执行器；新建空表的主键、唯一约束和普通索引随各自建表迁移使用普通 `CREATE INDEX` 创建，文件内不得出现 `CREATE INDEX CONCURRENTLY`。只有未来向已有存量表追加索引时，才另建放在 `<schema>/concurrent/` 下的独立非事务迁移并使用 `CREATE INDEX CONCURRENTLY`；本阶段无需这种文件，故不新增也不重编号。迁移会话固定 `SET lock_timeout = '5s'` 与 `SET statement_timeout = '30min'`。第 24 号文件按裁定 B-02 向 `platform_core.append_only_registry` 插入一行，`schema_name` 取 `procure`、`table_name` 取 `goods_receipt_line_costings`、`mode` 取 `APPEND_ONLY`、`mutable_columns` 取空数组，`created_by` 取 `foundation::SYSTEM_PRINCIPAL_ID`，仅追加触发器按该登记行挂接，回退为删除该行。该文件同时读写 `platform_core` 与 `procure` 两个 schema，其主要创建对象是 `procure.goods_receipt_line_costings` 上的仅追加触发器与其登记行，按裁定通则第五条放在 `db/migrations/procure/` 目录下，版本号晚于阶段 2 建立 `platform_core.append_only_registry` 的迁移。第 33 号在三张被引 portal 表建立后执行，并先对三组源数据做同法人孤儿与祖先预检，非零立即失败，不以 `NOT VALID` 留尾。它建立 `(legal_entity_id,purchase_order_id,delivery_notice_id) -> portal.delivery_notices(legal_entity_id,purchase_order_id,id)` 的收货头和拒收外键，以及收货行 `(legal_entity_id,purchase_order_id,purchase_order_line_id,delivery_notice_id,delivery_notice_line_id) -> portal.delivery_notice_lines(legal_entity_id,purchase_order_id,purchase_order_line_id,delivery_notice_id,id)`；随后建立 `procure.assert_goods_receipt_portal_graph_consistent()` 并在收货头行与 notice 头行装四个 `DEFERRABLE INITIALLY DEFERRED` 触发器，强制 notice 可空图同空同非空、供应商/订单/订单行/批次/物料全同源，已过账收货对每条 notice line 的累计数量恰等于其 `received_quantity`，notice 状态与累计形状一致。普通 FK 全命中但跨 notice 头或错 PO 行的 direct SQL 在提交时拒绝。回退按触发器、函数、三条 FK 的顺序删除。本阶段按裁定 A-21 不新增 `ledger.posting_trigger_event_types` 的回填迁移，`procure.goods_receipt.posted.v1` 与 `procure.purchase_return.posted.v1` 两行登记由阶段 9a 的种子迁移一次写入，见第 6.5 小节。

Stage 14 历史迁移撤销所需的两条 procure owner 审计事实复用既有 `platform_audit.audit_events`，不增加 procure 表、Outbox 事件或迁移文件；`V20261018090300` 与 `V20261018091500` 只提供根表及候选键，运行期动作与 Stage 14 第 092600 号静态证据分支的精确契约见第 4.2.8 节。

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
value/grni_source_kind.rs       GrniSourceKind { GoodsReceipt, PurchaseReturn, PurchaseInvoice, PurchaseCreditNote }
value/effect_direction.rs       EffectDirection { Increase, Decrease }
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
| PENDING / PARTIALLY_ORDERED | CLOSED | 采购员手工关闭、普通来源单据作废，或来源合同终止 | 手工关闭必填 `close_reason`；普通来源作废沿既有处理；来源合同终止只由 `CLM_TERM_PURCHASE_REQUISITION` 的 `ImpactRule::dispose` 触发并写审计，不新增合同终止消费者 |
| ORDERED | CLOSED | 采购员手工关闭 | 必填 `close_reason` |

`ordered_quantity` 的增减只在采购订单进入 `ISSUED` 与离开 `ISSUED` 及其后各态时发生。订单作废时按行回退，订单提前关闭时不回退，理由是已下达数量已经发生。

##### 4.2.2 采购订单

状态与流转按 PRD 第 4.4.4 小节，守卫条件如下。

| 流转 | 守卫条件 |
|---|---|
| DRAFT → PENDING_APPROVAL | 供应商准入状态为 ADMITTED；供应商资质未整体过期；直接费用类三个归集字段至少一项非空；物料类每行 `warehouse_id` 非空；每行累计下达数量不超过关联需求数量；交货批次行数量合计等于该行订单数量 |
| PENDING_APPROVAL → ISSUED | 审批链存在、至少含一个节点、每个节点展开后的有效审批人集合非空、申请人不属于任一节点展开集合，且全部节点均已通过；任一条件不满足均 fail-closed，不得下达 |
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
| ↳ F-63 注 | 本表为**不完全摘录**，采购订单状态域的唯一权威是 F-57 业务执行契约 §14.6（含 F-63 补入的改单回退与红冲重开两条边）；两表不一致时以 §14.6 为准 |
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

按 PRD 第 4.6.4 小节，守卫条件如下。`DRAFT → PENDING_APPROVAL` 要求物料类每行的本次退货数量不超过该收货行可退数量，直运场景要求 `sales_return_id` 指向存在且未作废的销售退货单。`PENDING_APPROVAL → POSTED` 要求审批链存在、至少含一个节点、每个节点展开后的有效审批人集合非空、申请人不属于任一节点展开集合、全部节点均已通过，并通过含结存充足性校验在内的过账前置校验；任何条件不满足均 fail-closed。`POSTED` 为终态，登记有误只能另行登记更正单据。

采购订单与采购退货的出厂审批链均固定为一个 `approver_kind=ROLE, role_code=PROCURE_MANAGER` 的必经节点。提交事务必须先完成链配置读取、全节点展开与职责分离校验，再取得单据写锁并产生第一笔写入：审批链缺失、零节点或任一节点展开为空统一返回既有稳定错误码 `PLATFORM.APPROVAL.NODE_HAS_NO_APPROVER`；申请人命中任一节点展开集合返回 `PLATFORM.SOD.SELF_APPROVAL_FORBIDDEN`。两类拒绝均保持原单据状态与 `row_version` 不变，且采购业务表、库存流水、总账凭证、审批任务、审计写集和 `platform_msg.outbox_events` 均零新增；不得把缺配置解释为自动通过，也不得跳过未通过节点直接执行下达或过账。

##### 4.2.5 付款申请

按 PRD 第 4.7.4 小节。`DRAFT → PENDING_APPROVAL` 的守卫条件为供应商状态非 `TERMINATED` 与供应商收款账户非空两条；`payment_type` 取 `INVOICE_PAYMENT` 时另加一条，即申请金额加该发票已占用金额不超过其未核销余额，该类型连同这条守卫按第 4.5 小节整条推迟到阶段 10，因此本阶段受理的付款申请只有 `PREPAYMENT` 一类。`PENDING_APPROVAL → APPROVED` 要求审批链全部节点通过且申请人不在任一节点上。`APPROVED → PARTIALLY_PAID → FULLY_PAID` 由财务的付款登记经 `PaymentRequestWritebackPort::register_payment` 回写驱动；资金冲正经同 trait 的 `reverse_payment` 把 FULLY_PAID 重开为 APPROVED/PARTIALLY_PAID，已提前 CLOSED 的申请保持 CLOSED。`APPROVED / PARTIALLY_PAID → CLOSED` 由采购主管填写原因触发并只释放未付残余占用。

##### 4.2.6 供应商准入

按 PRD 第 4.8.3 小节。`PENDING → ADMITTED` 的触发点是 mdm 侧供应商档案的生效审批通过事件，`ep-app-procure` 消费该事件并写入准入结论与有效期，这一衔接解决 PRD 第 2.4 节与第 4.8 节的双归属分歧，取值见第 11.2 小节假设 A4。`ADMITTED → SUSPENDED / TERMINATED`、`SUSPENDED → ADMITTED / TERMINATED` 由供应商管理员在应用内触发。

状态对业务的约束在领域层表达为三条守卫：`SUSPENDED` 时禁止新建采购需求指定该供应商、禁止新建与提交采购订单，已下达订单的收货、退货与付款照常；`TERMINATED` 时另禁止新建付款申请并同步把该供应商的全部门户绑定置为 `DISABLED`；资质整体过期时禁止采购订单提交，已下达订单不受影响。

##### 4.2.7 送货通知与发票上传

按 PRD 第 4.9.4 小节与第 4.9.5 小节。送货通知的 `SUBMITTED → VOIDED` 守卫条件为未被任何收货单引用，即 `received_quantity` 全行为零。发票上传的 `UPLOADED → ACCEPTED` 由发票模块经契约回写，`UPLOADED → RETURNED` 由财务填写退回原因触发。

##### 4.2.8 历史迁移撤销的 owner 审计事实

`reverse_migrated_purchase_order` 与 `reverse_migrated_payment_request` 是 `ep-app-procure` 的 crate-private 用例，只允许 Stage 14 的 `MigrationModuleWriter::apply_reversal` 在同一 `&mut dyn Tx` 内调用，不注册 HTTP 路由、不授普通业务调用方能力，也不绕开当前法人、迁移冲销批准、乐观锁和既有业务守卫。两者先按根 id 锁行、重读依赖并执行下述唯一状态分支；事务只捕获一次 `effect_occurred_at`，由服务端另生成与 REVERSE receipt id 不同的 `owner_audit_event_id`。根变更、reservation 释放、owner 审计事实、Stage 14 的 R0、writer receipt 与记录转 REVERSED 必须同事务提交，任一失败零写入。

- 采购订单：DRAFT、PENDING_SUPPLIER_CONFIRM、SUPPLIER_RESCHEDULE_PROPOSED 只走既有 VOID 守卫并到 VOIDED；PENDING_APPROVAL、REJECTED、ISSUED、SUPPLIER_CONFIRMED、PARTIALLY_RECEIVED、COMPLETED 只走既有 CLOSE 守卫并到 CLOSED；CLOSED、VOIDED 保持原终态。非终态分支固定 `row_version+1` 并写对应 `closed_at/voided_at=effect_occurred_at`，终态保持分支的版本和原时点都不变；两支的业务原因固定 `DATA_MIGRATION_REVERSED`。存在未被各自 migration bundle 反向闭合的收货、采购发票或付款占用时，plan 与 owner 用例都必须拒绝。
- 付款申请：DRAFT、REJECTED 只走既有 VOID 到 VOIDED；PENDING_APPROVAL 只走 WITHDRAW 到 WITHDRAWN；APPROVED、PARTIALLY_PAID 只走 CLOSE 到 CLOSED；WITHDRAWN、CLOSED、VOIDED 保持原终态。FULLY_PAID 以及任一仍有付款效果或未释放 reservation 的状态必须先由对应 finance/migration 反向通道闭合，否则拒绝；非终态分支固定 `row_version+1` 并写对应 `withdrawn_at/closed_at/voided_at=effect_occurred_at`，终态保持分支版本和原时点不变，原因同样固定 `DATA_MIGRATION_REVERSED`。

每个用例在状态动作完成后写一条独立、不可变的 `platform_audit.audit_events` owner fact。采购订单 action 精确为 `PROCURE_PURCHASE_ORDER_MIGRATION_REVERSED`，付款申请 action 精确为 `PROCURE_PAYMENT_REQUEST_MIGRATION_REVERSED`；`object_type/object_id` 分别固定为 `procure.purchase_orders|procure.payment_requests` 与原 APPLY 根 id，`object_version` 等于根的 after row_version，`reason='DATA_MIGRATION_REVERSED'`，`occurred_at=effect_occurred_at`。`before`、`after` 各自必须恰有 `{schema_version:1,row_version,status}` 三键；按审计链全库编码规则，row_version 必须是不带前导零的正十进制 JSON 字符串，status 为本节封闭枚举字符串，不得把 row_version 写成 JSON number。真实变更时延迟分支不信任 JSON 转型，而是逐字比较 `after.row_version=root.row_version::text`、`before.row_version=(root.row_version-1)::text` 并匹配上述唯一状态边；终态保持时 before/after 版本字符串与根版本逐值相等。该 owner fact 不是 R0：REVERSE receipt 的 `target_object_type='platform_audit.audit_events'`、`target_id=owner_audit_event_id`；另写的 R0 才使用 `event_id=receipt.id`、`action='DATA_MIGRATION_REVERSED'`，且其 `after.owner_effect_object_type='platform_audit.audit_events'`、`after.owner_effect_id=owner_audit_event_id`。两条审计必须同法人、同 `effect_occurred_at` 且 event_id 不同。Stage 14 第 092600 号延迟分支提交时锁根并逐项核 owner action、根 id、before/after、最终状态/版本、同一时点、独立 R0 与 receipt target；任一旧审计、同一事件冒充两种证据、错动作、错根、错状态边、错版本、row_version 数字/非规范字符串或只写审计未完成 owner 守卫均拒绝。

#### 4.3 收货过账算法

收货过账是本阶段最关键的算法，它把采购单据、库存两账与总账凭证三者绑在同一个事务内。`POST /goods-receipts/{id}/actions/post` 只消费此前由创建端点落库的既有收货头、行与序列号，路径 `{id}` 是唯一来源单据 id；本动作绝不新建或替换收货头行，也不改变既有行 id。进入下述过账事务前先做无锁、无写的路由预判：若 DRAFT 快照已显示超容差且开关要求审批，改走独立的审批提交事务，锁后重算仍成立才写 PENDING_APPROVAL/审批实例/审计并返回 202，不解析期间、不取得 F-50 proof、不产生库存或账务效果；若预判可直过却在下述锁后重算变为需审批，则整个过账事务以 `PROCURE.GOODS_RECEIPT.OVER_RECEIPT_APPROVAL_REQUIRED` 回滚，客户端重取后进入审批路径，不能在已取得过账 proof 的事务中半途改为审批。下述步骤只处理“DRAFT 且锁后无需审批”或“PENDING_APPROVAL 且同一命令摘要的全部审批节点已通过”两种输入。

1. 平台前置、统一锁计划与本地源行锁。完成权限、幂等受理与路径 id 强类型解析；记账日早于服务端事务日时还必须验证 `ledger.backdate`、同命令摘要的 reauth 与 FINANCE_MANAGER 审批并构造唯一 `BackdateAuthorization`，否则该值为 None。随后在事务最前调用一次 `AccountingPeriodResolver::resolve`；零期间分支可能建立期间行，除此 proof 前只允许 coordinator 的零余额锁脚手架、下游 GRNI/库存事实 UUID 预生成与无锁查询，禁止预生成收货头或收货行 id。以既有 `{goods_receipt_id}` 无锁读取头行图，经 `OverbillingMatchPort::candidate_entry_ids_for_receipt` 收集 finance 候选，把既有收货行、将建立的 GRNI 根、`InventorySourceDocument(PurchaseReceipt,goods_receipt_id)`、库存 availability/value/coverage/qty/serial 维度及命中的 overbilling key 组成规范化 `F50LockPlan`，调用 `CrossModuleLockCoordinator::lock_all` 得 lease。全部全局类别锁完后锁既有收货头，再按 `purchase_order_line_id` 升序锁采购订单行、交货批次行与送货通知行；随后从已锁头重新取得既有收货行/序列号并重读全部关系，第二次调用同一 finance 候选方法重建完整 plan，只有 `seal_after_reload` 逐值相等才取得本事务 `TransactionLockProof`。集合漂移走 40001 整事务重试。proof 之前不得计算超收/价格/容量、不得占号码、不得写收货、GRNI、库存、finance 或采购业务事实；本地源行永远在全局类别之后锁，其他采购路径不得反序。
2. 前置校验。锁后收货头必须是上述两个允许输入状态之一，路径 id、法人、供应商、采购订单、送货通知、行 id 与创建时命令摘要逐值不变；逐行校验采购订单状态为 `SUPPLIER_CONFIRMED` 或 `PARTIALLY_RECEIVED`，订单未关闭未作废，收货行确属头采购订单且其批次/通知行确属同一订单行与同一头通知。物料启用批次管理时批次号非空、未启用时取 `'-'`；物料启用序列号管理时序列号条数等于该行数量；头供应商必须等于采购订单供应商与通知供应商；物料、仓库、数量与冻结订单单价必须等于被锁来源快照；`posting_date` 不晚于登记时点的服务器自然日，取值为 `(now() AT TIME ZONE 'Asia/Shanghai')::date` 的比较。
3. 超收与审批终检。本行实收数量大于该订单行剩余待收数量时置 `has_over_receipt`；剩余待收数量定义为订单行数量减累计收货数量加累计退货数量，采购订单行的订单数量不因超收自动调整。DRAFT 锁后命中“超容差且需审批”一律返回 `PROCURE.GOODS_RECEIPT.OVER_RECEIPT_APPROVAL_REQUIRED` 并回滚本事务；PENDING_APPROVAL 必须携带仍有效、绑定同一头行摘要且全节点通过的 `over_receipt_approval_ref`，否则 fail-closed。无需审批或审批已通过才继续第 4 步。
4. 锁后匹配并写库存两账。复用步骤 1 的唯一 `ResolvedPeriod` 与 `TransactionLockProof`；若 finance 模块已启用，在同一 `&mut dyn Tx` 上调用阶段 10 的 `OverbillingMatchPort::match_on_receipt(tx,ctx,&f50_lock_proof,cmd)`，由该端口先验证 proof、只重读已锁候选，并按收货行返回零至多条 `(overbilling_entry_id,matched_quantity,invoice_unit_price)` 分配及同事务结清效果，inventory 不得反向查询 finance。采购编排对每个收货行先按 `ESTIMATED_PO_PRICE`、再按 `OVERBILL_INVOICE_PRICE/overbilling_entry_id UUID bytes` 排序：未匹配余量形成一个暂估段，各匹配分配各形成一个超量段；段数量合计必须等于收货行数量，序列号按同一顺序无重无漏分区，`posting_line_key` 取 `<goods_receipt_line_id>:<segment_seq>`。随后只调用一次 `InventoryPostingPort::post_inbound(tx,ctx,&f50_lock_proof,InboundPosting { reason: InboundReason::PurchaseReceipt, source: SourceDocumentRef { doc_type:PURCHASE_RECEIPT,.. }, period, label, lines })`；每个暂估段传 `InboundPricing::Explicit { branch:EstimatedPoPrice, unit_price:order_unit_price_untaxed }`，每个超量段传 `Explicit { branch:OverbillInvoicePrice, unit_price:invoice_unit_price }`。返回键集合、分支、数量与输入逐项相等，全部行共享结果头的唯一 `stock_movement_id`，否则按不变量故障整笔回滚。finance 尚未启用时其挂账表在架构上不存在可匹配对象，全部数量只形成暂估段，发布装配不注入空实现。
5. 落 GRNI 与超量结清效果。procure owner 先以同一 coordinator 验证 proof 覆盖本次既有收货行、将建立的 GRNI 根与 effect id，再只把第 4 步结果中 `pricing_branch=ESTIMATED_PO_PRICE` 的段写入 `procure.goods_receipt_line_costings`，取 `source_kind=GOODS_RECEIPT`、`direction=INCREASE`，数量与金额分别取 `quantity/inbound_amount`，同时写 `resolved.accounting_period_id()`、`resolved.accounting_period_seq()` 与 `posting_date`，不写单价；每个根在 proof 前已预生成 UUIDv7，单条 INSERT 同时写 `id=root_effect_id`，不得先写空 root、补 UPDATE 或依赖表中已有行。不得在这里取得新锁。`OVERBILL_INVOICE_PRICE` 段不得写 GRNI，其数量与 `inbound_amount` 必须逐项等于 `OverbillingMatchPort` 的锁定分配并由 finance 在同一事务落 `overbilling_settlements`。任一段缺失、多出、键错、数量或金额不等均零提交。
6. 调用总账契约生成凭证。在同一 `&mut dyn Tx` 上调用 `ep_contract_ledger::PostingPort::post(tx, ctx, PostingInput { source_kind: VoucherSourceKind::PURCHASE_RECEIPT, posting_date, backdate_authorization, source_document: SourceDocumentRef { object_type:"PURCHASE_RECEIPT", id:路径收货id, doc_no:锁后收货单号 }, source_event_id:None, measures, attributions:vec![] })`。`source_event_id=None` 是因为 owner 的 posted 事件尚未在本事务后缀产生；PURCHASE_RECEIPT 的两项规则只影响 INVENTORY/AP_ACCRUED/OVERBILLING，不含成本或收入腿，因此 attribution 必须为空而不是遗漏。measures 只取第 4/5 步返回的暂估、超量匹配与结清会计金额。任一会计效果非零时只接受 `Posted`，全部效果为零时只接受 `Skipped`；两种结果回带期间都必须等于唯一 `ResolvedPeriod`，首次执行的 `IdempotentReplay`、非零 `Skipped`、零值 `Posted` 或期间不等均按孤立/不变量故障整笔回滚。`Posted` 取得 Some(voucher_id)，合法全零 `Skipped` 取得 None。总账只做分录映射与借贷平衡，不提供任何取价方法；零价收货仍保留库存数量账、金额为零的库存价值事实与 `quantity>0,amount=0` 的 GRNI 根，不伪造零金额凭证。
7. 权威单据与来源回写。把既有收货头一次更新为 `status=POSTED`、`accounting_period_id=resolved.accounting_period_id()`、`voucher_id=上述 Option`、`posted_at=事务时钟`，同时固化锁后 `has_over_receipt/reason/approval_ref`；不得插入第二个头或替换行 id。随后更新采购订单行与交货批次行的 `received_quantity`，推进 `line_status` 与订单 `status`，更新被引用送货通知行的 `received_quantity` 与通知 `status`，把订单 `is_type_locked` 置 true。收货行不复制 `stock_movement_id`：权威追溯由 inventory 的唯一 `(legal_entity_id,source_doc_type=PURCHASE_RECEIPT,source_doc_id=goods_receipt_id,source_doc_line_id=goods_receipt_line_id,posting_line_key)` 反查全部稳定分段，详情接口只经 `InventoryPricingLookupPort` 组装，禁止另建可漂移的单值 movement 副本。
8. 收口幂等与消息。以第 7 步写回后的权威收货头行快照执行幂等 `finish`，写入 Outbox 事件 `procure.goods_receipt.posted.v1`；payload 的 `voucher_id` 可空且只在全部会计效果为零时空，信封携带 `posting_date` 与 `Posted|Skipped` 回带的 `accounting_period_id`。再写确需同事务落库的通知命令，外部投递均在提交后进行。
9. 写审计终结批。经 `ep-platform-audit` 写入 `GOODS_RECEIPT_POSTED` 审计事件，`before` 为本事务锁定的 DRAFT/PENDING_APPROVAL 快照、`after` 为第 7 步 POSTED 快照且含可空 voucher 与逐行 inventory/GRNI 结果；敏感字段按掩码规则处理。这是 commit 前最后一批数据库执行，其后不得再访问数据库或调用跨模块端口。

边界条件：第 4 至第 7 步任一失败即整个事务回滚，既有收货头保持原状态且 id/行 id 不变，接口返回明确失败并给出 `incident_no`，不写死信，理由是事务未提交因而不存在不一致；PRD 第 4.5.6 小节所称的「库存或财务侧写入不一致」在本设计下只可能由事后对账检出，其处置走死信与人工修复。审批提交是进入本算法前的独立状态事务，审批期间不解析会计期间且不产生库存、GRNI、凭证或 POSTED 事件。

#### 4.4 采购退货过账算法

过账请求中唯一允许随动作提交、且不属于既有退货事实的业务字段是本次红字发票标识。HTTP/application DTO 冻结在 `crates/application/procure/src/usecase/post_purchase_return.rs`；它可以依赖 invoice contract，但不得下沉到 `ep-contract-procure` 形成 contract→contract 依赖：

```rust
pub struct PurchaseCreditNoteIdentifierInput {
    pub original_purchase_invoice_id: Id<PurchaseInvoice>,
    pub identifier: ep_contract_invoice::InvoiceIdentifierInput,
}
pub struct PostPurchaseReturn {
    pub expected_row_version: i64,
    pub credit_note_identifiers: Vec<PurchaseCreditNoteIdentifierInput>,
}
```

`credit_note_identifiers` 必须按 `original_purchase_invoice_id` UUID bytes 升序且 id 唯一；它给每张实际被本次退货消费的原进项发票提供一枚全新的红字法定标识，不得复用原蓝票标识。锁后 distinct billed 原票集合必须与该数组键集合逐值相等：纯未开票退货必须传空，缺项、多项、重复、乱序或原票不属于本退货关系图均以 VALIDATION 拒绝并零写入。

1. 平台前置、收集与统一 proof。`POST /purchase-returns/{id}/actions/post` 只消费创建端点已落库且全部审批节点通过的既有退货头、行与序列号；路径 `{id}` 是唯一退货 id，本动作不得新建头行、替换行 id 或从请求另收来源图。完成权限/重新认证、幂等受理与路径 id 强类型解析；记账日早于服务端事务日时验证 `ledger.backdate`、同命令摘要 reauth 与 FINANCE_MANAGER 审批并构造 `BackdateAuthorization`，否则为 None。随后在事务最前只调用一次 `AccountingPeriodResolver::resolve`；零期间建立与 coordinator 零余额锁脚手架是 proof 前仅有的受控结构写。只预生成红字、GRNI、库存和凭证等下游效果 id。MATERIAL_RECEIPT 只以既有退货行的收货行 id 无锁调用 `ReceiptInvoiceMatchQueryPort::lock_candidates_for_receipt_lines(tx,ctx,goods_receipt_line_ids)`，该方法只返回原票头/行 id；DROP_SHIP 直接取既有退货行已持久化的 `(purchase_invoice_id,purchase_invoice_line_id)`，两路按原票 id 分组后各以来源行 id 调 `PurchaseCreditNotePort::lock_candidates`。同时调用 `GrniEffectWritebackPort::lock_candidates` 并收集 inventory 维度，组成完整 `F50LockPlan`：明确包含既有本退货、原票头行、收货行、全部 GRNI `(root_effect_id,effect_id)`、`InventorySourceDocument(PurchaseReturn,purchase_return_id)`、availability/value/coverage/qty/serial、AP、payable reservation、advance、settlement与冲销累计键。调用唯一 coordinator `lock_all`，其 PurchaseReturn owner 锁住路径头后，再按退货行 id 锁既有行/序列号及采购模块自身不在全局表内的订单辅助行；锁后头必须仍为 PENDING_APPROVAL、审批引用绑定同一命令摘要且全部节点通过。随后从已锁头重取相同行 id，以完全相同入参重调 `lock_candidates_for_receipt_lines`、逐原票 `PurchaseCreditNotePort::lock_candidates`、GRNI 与 inventory 候选入口形成 reloaded plan，再调用 `seal_after_reload`；只有规范化 id/类别集合逐值相等才得到同一 `TransactionLockProof`，集合变化走 40001 整事务重试。proof 之前 `ReceiptInvoiceMatchQueryPort::match_state/match_states/billed_allocations_for_purchase_invoice_lines` 三种容量方法的调用次数必须为零，也禁止分段、row_version/容量/金额计算、红字/GRNI/库存/AP 写入或任何采购状态推进；所有下游查询/mutator 只接收这一个 proof，不得补锁或另取 proof。
2. 分支判定。取得 proof 后，MATERIAL_RECEIPT 经 `ReceiptInvoiceMatchQueryPort::match_states(tx,ctx,&f50_lock_proof,goods_receipt_line_ids)` 逐收货行取得 Stage10 exact `ReceiptInvoiceMatchState { goods_receipt_line_id, unbilled_returnable_quantity, billed_allocations }`；DROP_SHIP 把既有退货行的 `purchase_invoice_line_id` 按 UUID bytes 升序交给 `billed_allocations_for_purchase_invoice_lines(tx,ctx,&f50_lock_proof,source_line_ids)`，成功结果必须恰好一项/输入行。两个方法都先验证 proof 后只重读已锁原票图，不补锁；返回的 `billed_allocations` 统一按 `(posting_date,original_purchase_invoice_id UUID bytes,original_purchase_invoice_line_id UUID bytes)` 升序，逐项含原票 id/行 id、同票唯一 row_version、正的可退数量、原单价、税率与可退 net/tax/gross。乱序、重复、DROP_SHIP 键集不一一覆盖、同原票 row_version 不一致或三额不守恒均视为 owner 契约故障。物料退货先消费开放的未开票数量，不足部分再按数组 FIFO 消费；DROP_SHIP 全量走 billed allocation。整项消费直接复制 owner 返回的三额，部分消费固定以 `round(original_unit_price * consumed_quantity,2,MidpointAwayFromZero)` 算 net、再以同舍入算 tax、gross=net+tax，invoice owner 仍按原票剩余累计做最终上限校验。所得 `unbilled_quantity+Σbilled_consumed_quantity` 必须等于本次退货数量，用户不可干预。该规则可在一张物料退货、一个收货行内形成混合段并跨多张原票，因此不得读取或持久化头级开票布尔值。阶段 7 首次交付时只落未开票实现；发票已登记分支、两个正式端口及下述红字路径与阶段 10 同批接线，接线前 invoice 模块未启用，因而不存在可提交的已开票数据，不设置会在完整系统中继续生效的临时阻断口径。
3. 可退数量校验。本次退货数量不超过该收货行的 `quantity - returned_quantity`；批次与序列号必须为该收货行原登记的取值。
4. 结存充足性前置校验。物料类退货为出库方向，先执行 `let validated_batch_no = BatchNo::try_from(raw_batch_no)?;`，再调用 `ep_contract_inventory::StockOnHandQueryPort::on_hand(tx, ctx, legal_entity_id, warehouse_id, material_id, &validated_batch_no)` 校验本次出库数量不超过当前物理结存数量；禁止把未经验证的字符串或另一个同名变量传入。构造失败在端口调用前按字段校验拒绝，结存不足则阻断提交并返回当前物理结存数量。此处不得调用只提供 `available(...)` 的 `AvailabilityQueryPort`，后者会扣除已确认/已下达销售订单的未交付需求，不是采购退货的物理出库上限。这是 PRD 第 5.6.4 小节的提交时校验在采购侧的落点。
5. 使用同一 proof 按原票分组写红字/GRNI。复用步骤 1 的唯一期间与 proof。物料类未开票段在 procure owner `assert_covers` 后直接按第 3.2.10 节对已锁开放 GRNI 追加 `PURCHASE_RETURN/DECREASE`。把步骤 2 实际消费的 billed allocations 按 `original_purchase_invoice_id` 分组，并按该 id UUID bytes 升序逐组恰调用一次 `PurchaseCreditNotePort::register_credit_note(tx,ctx,&f50_lock_proof,RegisterPurchaseCreditNote { supplier_id, original_purchase_invoice_id, linked_purchase_return_id:Some(路径退货id), identifier:对应请求项.identifier, posting_date, expected_original_row_version:该组唯一row_version, is_for_overbilling_settlement:false, lines })`；`lines` 按原票行 id 升序，由步骤 2 的 allocation 快照构造 `quantity_effect_kind=REDUCE`、`pricing_effect_kind=ORIGINAL_UNIT_PRICE` 及消费数量/net/tax/gross，不接受用户金额。物料组固定产生 `PURCHASE_INVOICE_LINKED_RETURN_REVERSED` 并返回 `grni_reopened_effects`，采购用例立即逐条以新效果为直接父行追加等数量、等金额的 `PURCHASE_RETURN/DECREASE`；直接费用/直运组固定产生 `PURCHASE_INVOICE_DIRECT_EXPENSE_REVERSED`，不调用 GRNI 端口。每个返回必须 `linked_purchase_return_id=本路径id` 且期间等于唯一 ResolvedPeriod；各组返回集合合并后缺组、多组、错原票、错收货行、数量/金额不等，或未开票加全部组的数量不等于退货数量，均整笔回滚。这样一张采购退货可合法得到多张红字凭证，但同一原票在本动作中只能一张。
6. 写库存两账并形成行级权威结果（直运场景跳过）。完成步骤 5 的 GRNI 写入后，在同一 `&mut dyn Tx` 上只调用一次文档级 `InventoryPostingPort::post_outbound(tx,ctx,&f50_lock_proof,OutboundPosting { reason:OutboundReason::PurchaseReturn, source:SourceDocumentRef { doc_type:PURCHASE_RETURN,doc_id:路径id,.. }, period,label,lines })`；每条物料行以 `<purchase_return_line_id>:1` 为 `posting_line_key`、以既有退货行 id/line_no 为 `source_line`，取锁后 MDM 快照并传 `OutboundPricing::ReturnAtMovingAverage`。输出键集合必须完全相等，按键把 `applied_unit_price/outbound_amount` 分别映射为该既有行待写的 `reversal_unit_price/reversal_amount`，所有行 id 必须等于结果头唯一 `stock_movement_id`。库存模块按锁后当前账面价值计量：部分退货按当前移动加权单价，本次出库使结存数量归零时直接取退货前库存金额余额全额，使金额余额与单价同时归零；当前账面价值为零时两项返回零，不回查原入账单价制造负库存金额。直运/直接费用场景不调用库存契约、行级两项 reversal 保持空，也不生成物理采购退货凭证，只由 `PURCHASE_INVOICE_DIRECT_EXPENSE_REVERSED` 红字凭证冲回原归集成本。
7. 生成物理退货凭证。物料场景在同一事务调用 `PostingPort::post`，七字段固定为：`source_kind=PURCHASE_RETURN_INVENTORY`、`posting_date=锁后退货记账日`、`backdate_authorization=步骤1结果`、`source_document={object_type:"PURCHASE_RETURN",id:路径id,doc_no:锁后单号}`、`source_event_id=None`、`measures` 与本段所述 `attributions`；不存在 `source_sequence_no` 字段。计量项固定为 `grni_consumed_amount`、`inventory_return_amount` 与有符号的 `return_carrying_difference_amount = inventory_return_amount - grni_consumed_amount`。映射固定为借 GRNI、贷库存，差额借主营业务成本，负数整腿反向。前两项不含成本/收入腿，不得携 attribution；差额为零时 attribution 为空。差额非零时，先算每条退货行的 `line_difference=reversal_amount-line_grni_consumed_amount`，正负两组各按退货行 UUID bytes 升序做确定性 FIFO 抵消，剩余项必与头级差额同号且金额合计恰等于其绝对值；每个剩余项生成一条 `PostingAttribution { source_document_line_id:purchase_return_line_id, measure_key:ReturnCarryingDifferenceAmount, amount:abs(residual), capture_kind:CostPostingVariance(PurchaseReturnDiff), dimensions:{contract_id:锁后可空合同, sales_order_id:None, sales_order_line_id:None, customer_id:None, project_id:锁后可空项目, product_id:None, material_id:Some(material_id), warehouse_id:Some(warehouse_id)}, reverses_capture_entry_id:None }`。不得按绝对行差直接求和而把正负抵消丢失。三项会计效果任一非零时只接受 `Posted` 并取得 Some(physical_return_voucher_id)，三项全零时只接受 `Skipped` 且物理凭证 id 为 None；两种结果期间都必须等于唯一 `ResolvedPeriod`。首次执行的 `IdempotentReplay`、非零 `Skipped`、零值 `Posted` 或期间错配均按孤立/不变量故障整笔回滚。未开票、已开票和同单混合都至多生成这一张物理凭证；已开票段另有步骤 5 返回的一张或多张红字凭证，两类效果合计 GRNI 净额为零，库存只减少一次。直运/直接费用场景不调用本端口，物理凭证 id 保持空。
8. 权威退货单与来源回写。先按第 6 步输出逐行写入既有物料退货行的 `reversal_unit_price/reversal_amount`，直运行保持空；再把路径退货头一次更新为 `status=POSTED`、`accounting_period_id=resolved.accounting_period_id()`、`physical_return_voucher_id=上述 Option`、`posted_at=事务时钟`，审批引用保持不变。随后更新收货行 `returned_quantity` 与收货单 `status`，更新采购订单行 `returned_quantity`。不得插入第二个退货头、替换行 id 或先置 POSTED 再补行结果；提交点的延迟图约束同时验证退货→收货/订单/供应商/物料/序列号、DROP_SHIP→销售退货/原发票与全部 GRNI/inventory/credit-note/physical-voucher 来源效果。
9. 以第 8 步写回后的权威 POSTED 头行和行结果执行幂等 `finish`，再写 Outbox 事件 `procure.purchase_return.posted.v1` 与确需同事务落库的通知命令，最后写审计终结批。事件构造可空 `physical_return_voucher_id` 与去重、按 UUID 升序的 `purchase_credit_note_voucher_ids[]`，不使用含义不明的单一 `voucher_id`：物理 id 是否为空只由第 7 步三项会计效果是否全零决定，不能再用 MATERIAL/DROP_SHIP 粗判；数组为空当且仅当本次没有 billed allocation。数组中的每项必须来自 `linked_purchase_return_id=本退货 id` 的本次 `PurchaseCreditNoteView`。审计 `before` 为锁定的 PENDING_APPROVAL 快照，`after` 为权威 POSTED 快照及全部行级/GRNI/inventory/红字结果；审计之后不得再执行任何数据库语句。
10. 由 job-worker 消费该事件生成一条 `procure.supplier_quality_records`，来源类型 `PURCHASE_RETURN`，退货原因进入质量记录。该步在事务外经 Outbox 完成，理由是质量记录不参与任何守恒判据。

边界条件：直运场景要求 `sales_return_id` 非空且指向未作废的销售退货单，二者逐笔勾稽。完整首版中，直运采购退货不产生本方库存流水或物理退货凭证，但必须由步骤 5 的链接进项红字冲回应付、进项税与原直接费用，并把其凭证 id 放入事件数组；阶段 10 接线前不存在可提交的已开票/直接费用数据，该分支整条不注册，不得以“无账务效果”临时实现。同一笔直接费用类成本的分次冲回与累计冲回金额不超过原归集金额，由 `PurchaseCreditNotePort::register_credit_note` 在 invoice 模块内判定并在超限时返回业务冲突，`ep-app-procure` 直接透传错误码。供应商拒绝接受直运退回时走第 5.4 节专用动作；不得借普通退货过账或隐藏布尔位冒充 U-C-09 标记。步骤 5 至 8 任一步失败时，既有退货头仍为 PENDING_APPROVAL，行 reversal 字段仍空，所有红字/GRNI/inventory/voucher 与上游累计均零提交。

#### 4.5 付款申请占用算法

同一张采购发票被多张未关闭付款申请引用时的占用校验必须与应付 `effective_open` 在同一 F-50 串行序列内成立。`INVOICE_PAYMENT` 要求所有行 `ref_type=PURCHASE_INVOICE` 且同一申请内 `ref_id` 不重复；`PREPAYMENT` 要求所有行只引用 CONTRACT/PURCHASE_ORDER。二者不得混行。`ux_payment_request_lines_payment_request_ref` 固定为 `(payment_request_id,ref_type,ref_id)` 普通唯一约束，使付款回写能把财务逐发票分配唯一映射到申请行。预付款不产生 payable reservation；发票付款不得把超过应付可核销额的部分转成预付，超额必须改走独立 PREPAYMENT 申请。

唯一公开 ABI 位于 `crates/contract/procure/src/port/payment_request.rs`，字段、方法和返回形状冻结如下；`GoodsReceiptQueryPort` 不在此文件也不再属于公开 roster。

```rust
pub enum PaymentRequestKind { InvoicePayment, Prepayment }
pub enum PaymentRequestRefKind { PurchaseInvoice, Contract, PurchaseOrder }
pub enum PaymentRequestStatus {
    Draft, PendingApproval, Rejected, Withdrawn, Approved,
    PartiallyPaid, FullyPaid, Closed, Voided,
}

pub struct PaymentRequestLineForPayment {
    pub payment_request_line_id: Id<PaymentRequestLine>,
    pub line_no: i32,
    pub ref_kind: PaymentRequestRefKind,
    pub ref_id: uuid::Uuid,
    pub requested_amount: Money,
    pub paid_amount: Money,
}

pub struct PaymentRequestForPayment {
    pub payment_request_id: Id<PaymentRequest>,
    pub supplier_id: Id<Supplier>,
    pub kind: PaymentRequestKind,
    pub status: PaymentRequestStatus,
    pub requested_amount: Money,
    pub paid_amount: Money,
    pub payee_account_ref: uuid::Uuid,
    pub row_version: i64,
    pub lines: Vec<PaymentRequestLineForPayment>,
}

#[async_trait::async_trait]
pub trait PaymentRequestQueryPort: Send + Sync {
    async fn for_payment(
        &self,
        tx: &mut dyn Tx,
        ctx: &SecurityContext,
        payment_request_id: Id<PaymentRequest>,
    ) -> Result<PaymentRequestForPayment, AppError>;
}

pub struct InvoicePaymentAllocationWriteback {
    pub payment_request_line_id: Id<PaymentRequestLine>,
    pub purchase_invoice_id: Id<PurchaseInvoice>,
    pub paid_amount_delta: Money,
}

pub struct PrepaymentAllocationWriteback {
    pub payment_request_line_id: Id<PaymentRequestLine>,
    pub paid_amount_delta: Money,
}

pub enum PaymentRequestAllocationWriteback {
    Invoice(InvoicePaymentAllocationWriteback),
    Prepayment(PrepaymentAllocationWriteback),
}

pub struct PaymentRequestPaymentWriteback {
    pub payment_request_id: Id<PaymentRequest>,
    pub payment_id: Id<Payment>,
    pub expected_row_version: i64,
    pub allocations: Vec<PaymentRequestAllocationWriteback>,
}

pub struct PaymentRequestPaymentReversalWriteback {
    pub payment_request_id: Id<PaymentRequest>,
    pub original_payment_id: Id<Payment>,
    pub cash_reversal_id: Id<CashDocumentReversal>,
    pub expected_row_version: i64,
    pub allocations: Vec<PaymentRequestAllocationWriteback>,
}

pub struct PaymentRequestLinePaymentResult {
    pub payment_request_line_id: Id<PaymentRequestLine>,
    pub paid_amount: Money,
    pub remaining_amount: Money,
    pub row_version: i64,
}

pub struct PaymentRequestPaymentWritebackResult {
    pub payment_request_id: Id<PaymentRequest>,
    pub status: PaymentRequestStatus,
    pub paid_amount: Money,
    pub remaining_amount: Money,
    pub row_version: i64,
    pub lines: Vec<PaymentRequestLinePaymentResult>,
}

#[async_trait::async_trait]
pub trait PaymentRequestWritebackPort: Send + Sync {
    async fn register_payment(
        &self,
        tx: &mut dyn Tx,
        ctx: &SecurityContext,
        f50_lock_proof: &TransactionLockProof,
        cmd: PaymentRequestPaymentWriteback,
    ) -> Result<PaymentRequestPaymentWritebackResult, AppError>;

    async fn reverse_payment(
        &self,
        tx: &mut dyn Tx,
        ctx: &SecurityContext,
        f50_lock_proof: &TransactionLockProof,
        cmd: PaymentRequestPaymentReversalWriteback,
    ) -> Result<PaymentRequestPaymentWritebackResult, AppError>;
}

pub struct PayableReservationSnapshot {
    pub purchase_invoice_id: Id<PurchaseInvoice>,
    pub reserved_amount: Money,
    pub open_request_count: i32,
    pub row_version: Option<i64>,
}

#[async_trait::async_trait]
pub trait PayableReservationReadPort: Send + Sync {
    async fn after_lock(
        &self,
        tx: &mut dyn Tx,
        ctx: &SecurityContext,
        f50_lock_proof: &TransactionLockProof,
        purchase_invoice_ids: &[Id<PurchaseInvoice>],
    ) -> Result<Vec<PayableReservationSnapshot>, AppError>;
}
```

查询集合语义固定：`for_payment` 不存在、跨法人或不可见统一 `PLATFORM.AUTHZ.NOT_FOUND_OR_DENIED`，lines 按 line_no/id 升序且头金额逐项等于行合计。`after_lock` 要求输入排序前无重复，返回恰好一行/输入 id 并按 UUID bytes 升序；合法发票尚无 reservation 行时返回金额/计数为零、`row_version=None`，不是 404。三个 mutator/read 方法都先用 coordinator 验证 proof 覆盖全部 `PayableReservation` key；owner 只锁后读取或更新，不补 advisory/row lock。

提交发票付款申请时先经 `PaymentRequestQueryPort`/`PayableLedgerQuery` 无锁收集 purchase invoice、AP original entry 与 reservation keys，`lock_all` 后重载并 seal。proof 后对每个 invoice id 升序执行真实 upsert：`INSERT ... (reserved_amount=$delta,open_request_count=1,row_version=1,...) ON CONFLICT (...) DO UPDATE SET reserved_amount=payable_reservations.reserved_amount+EXCLUDED.reserved_amount,open_request_count=payable_reservations.open_request_count+1,row_version=payable_reservations.row_version+1,updated_at=now(),updated_by=$actor RETURNING ...`。再以锁后 `effective_open` 断言 `reserved_after <= effective_open_after`；超出返回 `PROCURE.PAYMENT_REQUEST.AMOUNT_EXCEEDS_OPEN_BALANCE` 并整笔回滚。不得用 no-op UPDATE 取锁。

`register_payment` 只接受状态 APPROVED/PARTIALLY_PAID，allocations 非空、line id 唯一、正金额并按 line id 升序。InvoicePayment 的每项 invoice id 必须等于该行 ref_id，且逐发票 `paid_amount_delta` 必须逐项等于财务本次真实 AP settlement APPLY 金额；Prepayment 只接受 Prepayment variant。每行与头的 paid_after 均不得超过 requested；每个真实行更新都 `row_version+1`。发票付款在写 AP APPLY 的同一事务先把 reservation `reserved_amount` 减本次 delta，部分付款不减 `open_request_count`；当所有行恰好付清时，再对每个 invoice 释放其 `requested_amount-paid_amount_after` 的剩余值（正常应为零）并把 count 减一，状态进入 FULLY_PAID。否则状态进入/保持 PARTIALLY_PAID。头 `paid_amount=sum(lines.paid_amount)`，结果 lines 按 line_no/id 升序。

`reverse_payment` 只由资金冲正状态迁移唯一胜者在同一事务调用，allocations 必须逐项等于原付款对这些申请行的已登记分配。对 APPROVED/PARTIALLY_PAID/FULLY_PAID 行按 delta 减少 paid；从 FULLY_PAID 重开时每个 invoice reservation 同额增加且 count 从零加一，状态按新 paid 总额变为 APPROVED 或 PARTIALLY_PAID；原申请已 CLOSED 时只回减 paid、保持 CLOSED 且不恢复 reservation。InvoicePayment 的 AP RELEASE 与 reservation 恢复处于同一 proof/事务，最终仍须 `effective_open_after >= reserved_after`。同一付款/冲正的 finance 状态条件更新、HTTP 幂等记录与本 port 调用在一个事务中；只有唯一状态胜者调用，精确重放直接返回首次结果，未赢得状态迁移不得再调用。即使内部误调，旧 `expected_row_version` 也必须以 `PLATFORM.CONCURRENCY.STALE_VERSION` 零写入拒绝。

驳回、撤回、作废、提前关闭等采购自有终态只由状态条件更新唯一胜者释放：按 invoice id 升序减 `requested_amount-paid_amount` 并将 count 减一；PARTIALLY_PAID 关闭只释放未付残余，不重复释放已付款 delta。计数归零时金额必须同时为零，保留汇总行不删除。所有 reservation 真更新显式 `row_version+1`。

所有会降低 AP `effective_open` 的路径（付款 APPLY、进项红字、供应商返款/资金冲正的再核销以及其 reversal）都必须把对应 `PayableReservation` key 纳入 F-50 plan，锁后经 `PayableReservationReadPort::after_lock` 重验 `effective_open_after >= reserved_after`；付款路径先释放本次真实 payment delta 再做终检。`V20261019093130__finance_add_deferred_foreign_keys.sql` 同批增加名为 `finance.ck_payable_effective_open_covers_procure_reservation()` 的 DEFERRABLE INITIALLY DEFERRED constraint-trigger 终检，由 reservation 的 INSERT/UPDATE 与 payable entry/settlement/reversal 的有效余额变化两侧共同排队，在 COMMIT 按 `(legal_entity_id,purchase_invoice_id)` 去重重算同一不等式。函数固定 `SECURITY DEFINER SET search_path=pg_catalog,finance,invoice,procure`、owner 为迁移 owner、REVOKE PUBLIC EXECUTE；它是绕过应用直写的第二道防线，不代替 F-50。

#### 4.6 采购需求的四类来源

| 来源 | 触发路径 | 幂等键构成 |
|---|---|---|
| 合同派生 | 合同生效派生事件的 Outbox 消费者调用 `PurchaseRequisitionIntakePort::intake` | `CONTRACT:{contract_id}:{contract_line_id}:{contract_version}` |
| 销售订单 | 采购员在应用内经 `actions/raise-from-sales-order-line` 发起 | `SALES_ORDER:{sales_order_line_id}` |
| 项目任务 | 项目模块经同一 `PurchaseRequisitionIntakePort::intake` 调用 | `PROJECT_TASK:{project_task_id}` |
| 库存不足 | job-worker 每 60 分钟扫描后调用同一端口 | `STOCK_SHORTAGE:{warehouse_id}:{material_id}:{scan_slot_utc}`；`scan_slot_utc` 为本次计划触发时点向下取整到 60 分钟的 UTC 值，任务重试不变 |

端口唯一签名、完整命令与回执字段取第 4.6.2 小节代码块；旧“八项且 material 必填”的窄 DTO 作废，因为它无法表达 DirectExpense、warehouse 与回执 target id/doc no。`unique_key` 必须逐字取上表幂等键并落到 `procure.purchase_requisitions.source_idempotency_key`。CONTRACT 来源传权威合同/合同行；SALES_ORDER 路径传订单/行与阶段 6 权威合同 id；PROJECT_TASK 以 project/task 调用并透传可空来源合同；STOCK_SHORTAGE 以 warehouse/material 调用且来源合同为空。owner 在同一建单事务按第 4.6.2 小节重验封闭形状与 Material/DirectExpense 字段矩阵，不得为手工项目任务伪造合同。

四类来源共用同一个用例与同一张唯一约束 `ux_purchase_requisitions_legal_entity_id_source_idempotency_key`，重复触发按幂等键返回已有需求，不产生第二条。合同派生失败按规格第 15.2 章进入死信与人工修复并写入审计，修复后重投由同一唯一约束保证不产生重复需求。

库存不足来源只经 `ep_contract_inventory::ReplenishmentPolicyQuery::list_for_scan(tx,ctx,legal_entity_id,after,limit)` 分页读取按 `(legal_entity_id,warehouse_id,material_id)` 排序的 `ReplenishmentPolicyScanView`，返回字段固定为 `warehouse_id,material_id,reorder_point,target_stock,available_qty`；`limit` 固定允许 `1..=500`，0 或 501 及以上返回 `PLATFORM.REQUEST.INVALID_PAYLOAD`。策略唯一物理存储和 `ReplenishmentPolicyReadPort` 的契约/持久化 owner 均为 inventory；阶段 6/sales 用该读端口和与销售可用量守卫 A2 相同的 `SalesAwareAvailabilityQuery` 组合实现真实 `SalesAwareReplenishmentPolicyQuery`；阶段 7 只在 job-worker 注入并消费该实现，不保存阈值、不直接读 inventory schema，也不重算第二套可用量。

策略满足 `target_stock >= reorder_point >= 0`；两阈值同空的停用行不会由组合查询返回。job-worker 每 60 分钟扫描一次；`available_qty <= reorder_point` 时建议量固定为 `max(target_stock - available_qty, 0)`，建议量为零时跳过且不告警。部署默认关闭。开启后，任务按组合升序取得与销售可用量守卫相同的 `pg_advisory_xact_lock(hashtextextended('sales-availability:'||legal_entity_id||':'||warehouse_id||':'||material_id,0))`，锁后经同一组合端口重读策略与可用量并读取现有未结需求；已有未结需求即跳过，否则以固定 `scan_slot_utc` 幂等键创建。生成 `open_stock_shortage_key` 的唯一约束是绕过应用层也不可重复的最终兜底。

##### 4.6.1 F-10 `CLM_TERM_PURCHASE_REQUISITION` 影响面规则

实现类型固定为 `ContractTerminationPurchaseRequisitionImpactRule`，位于 `crates/application/procure/src/impact/contract_termination_purchase_requisition.rs`，实现阶段 3 的 `ep_platform_impact::ImpactRule`。`code()` 固定返回 `CLM_TERM_PURCHASE_REQUISITION`，`upstream_event_type()` 固定返回 `clm.contract.terminated.v1`，`target_module` 固定为 `ModuleCode::Procure`。它是阶段 7 追加的第四个真实注册项；七条目录始终是编译期常量，阶段 7 结束时 `ImpactRegistry` 的真实累计注册数必须恰为 4，不得用 Noop、空规则或直接 DONE 的实现提前凑七条，也不得另建消费合同终止事件的采购消费者。

`assess` 只经 procure 仓储按同法人、`contract_id` 与 `id UUID bytes ASC` 查询本表，且必须逐项保留三支来源谓词：一，`source_type=CONTRACT` 且 `source_idempotency_key` 以 `CONTRACT:{contract_id}:` 开头；二，`source_type=SALES_ORDER` 且建单时由销售订单行权威关系固化的 `contract_id` 等于本合同；三，`source_type=PROJECT_TASK` 且项目任务确有 `source_contract_id`、因而建单时固化的 `contract_id` 等于本合同。无来源合同的手工 PROJECT_TASK、`STOCK_SHORTAGE`、其他合同、其他法人及 CLOSED 均不命中。PENDING 与 PARTIALLY_ORDERED 各产出一项 `AUTO_CLOSE`，ORDERED 产出 `MANUAL_DECISION`；目标引用固定携带 `target_doc_id=purchase_requisition_id`、`target_doc_no=doc_no`、`target_doc_line_no=null`。平台按 `target_module=PROCURE` 的固定映射把人工项分配给 `PROCURE_MANAGER`，规则不接收任意角色。

`dispose` 在当前 `&mut dyn Tx` 内按需求 id `FOR UPDATE`，复核法人、三支来源合同归属与当前状态，并只返回统一三态结果：PENDING 或 PARTIALLY_ORDERED 时写 `status=CLOSED`、`close_reason="合同终止 <合同编号>"`、`closed_at`，递增 `row_version` 并同事务写审计，返回 `ImpactDisposeOutcome::Completed { reason: "PURCHASE_REQUISITION_AUTO_CLOSED" }`；当前已 CLOSED 时不改行、不补审计，返回 `ImpactDisposeOutcome::AlreadySatisfied { reason: "PURCHASE_REQUISITION_ALREADY_CLOSED" }`；当前为 ORDERED 时不自动撤销已下达采购，返回 `ImpactDisposeOutcome::NeedsManualDecision { reason: "PURCHASE_REQUISITION_ORDERED_REQUIRES_DECISION" }`，不增加 attempts、不退避、不进死信。

ORDERED 人工项只允许两个 `decision_code`，不得解析 `decision_reason` 猜分支：`CLOSE_ORDERED_REQUISITION` 要求操作者先走既有关闭动作、目标锁后已为 CLOSED，且 `decision_result_doc_id` 非空并严格等于本采购需求 id，规则返回 `AlreadySatisfied`；`KEEP_ORDERED_REQUISITION` 要求目标锁后仍为 ORDERED，`decision_result_doc_id` 同样严格等于本采购需求 id，规则不改采购事实并返回 `Completed { reason: "PURCHASE_REQUISITION_KEEP_APPROVED" }`。两码都要求非空 `decision_reason`；缺码、错码、结果 id 为空/异单或状态与码不匹配均拒绝且保持 PENDING。规则不直接推进合同状态，完整七类项全部 DONE 且无 DEAD 后才由平台闭合合同。

##### 4.6.2 采购需求、直运退货与订单开票回写的 exact ABI

下列三个端口原来只有名字或不完整字段，现统一冻结在 `ep-contract-procure`；调用方不得另造近似 DTO。采购需求输入同时承载物料与直接费用，解决原八字段只能表达 material、无法构造合同直运/直接费用需求的问题。

```rust
pub enum PurchaseType { Material, DirectExpense }
pub enum PurchaseRequisitionStatus { Pending, PartiallyOrdered, Ordered, Closed }

pub struct PurchaseRequisitionIntake {
    pub source_module: ModuleCode,
    pub source_doc_id: uuid::Uuid,
    pub source_doc_line_id: uuid::Uuid,
    pub source_doc_no: Option<String>,
    pub source_contract_id: Option<Id<Contract>>,
    pub suggested_purchase_type: PurchaseType,
    pub material_id: Option<Id<Material>>,
    pub warehouse_id: Option<Id<Warehouse>>,
    pub expense_item_code: Option<String>,
    pub quantity: Quantity,
    pub required_on: chrono::NaiveDate,
    pub suggested_supplier_id: Option<Id<Supplier>>,
    pub is_drop_ship: bool,
    pub unique_key: String,
}

pub struct PurchaseRequisitionView {
    pub purchase_requisition_id: Id<PurchaseRequisition>,
    pub doc_no: String,
    pub status: PurchaseRequisitionStatus,
    pub source_doc_id: uuid::Uuid,
    pub source_doc_line_id: uuid::Uuid,
    pub suggested_purchase_type: PurchaseType,
    pub material_id: Option<Id<Material>>,
    pub warehouse_id: Option<Id<Warehouse>>,
    pub expense_item_code: Option<String>,
    pub required_quantity: Quantity,
    pub ordered_quantity: Quantity,
    pub expected_arrival_date: chrono::NaiveDate,
    pub contract_id: Option<Id<Contract>>,
    pub sales_order_id: Option<Id<SalesOrder>>,
    pub project_id: Option<Id<Project>>,
    pub is_drop_ship: bool,
    pub row_version: i64,
}

#[async_trait::async_trait]
pub trait PurchaseRequisitionIntakePort: Send + Sync {
    async fn intake(
        &self,
        tx: &mut dyn Tx,
        ctx: &SecurityContext,
        cmd: PurchaseRequisitionIntake,
    ) -> Result<PurchaseRequisitionView, AppError>;
}

pub struct DropShipReturnLine {
    pub sales_return_line_id: Id<SalesReturnLine>,
    pub sales_order_line_id: Id<SalesOrderLine>,
    pub quantity: Quantity,
}

pub struct PurchaseReturnLinkLineView {
    pub purchase_return_line_id: Id<PurchaseReturnLine>,
    pub sales_return_line_id: Id<SalesReturnLine>,
    pub purchase_order_line_id: Id<PurchaseOrderLine>,
    pub quantity: Quantity,
}

pub struct PurchaseReturnLinkView {
    pub purchase_return_id: Id<PurchaseReturn>,
    pub doc_no: String,
    pub sales_return_id: Id<SalesReturn>,
    pub status: PurchaseReturnStatus,
    pub lines: Vec<PurchaseReturnLinkLineView>,
    pub row_version: i64,
}

#[async_trait::async_trait]
pub trait PurchaseReturnLinkPort: Send + Sync {
    async fn link_drop_ship_return(
        &self,
        tx: &mut dyn Tx,
        ctx: &SecurityContext,
        sales_return_id: Id<SalesReturn>,
        lines: Vec<DropShipReturnLine>,
    ) -> Result<PurchaseReturnLinkView, AppError>;
}

pub struct PurchaseOrderInvoiceTarget {
    pub purchase_order_id: Id<PurchaseOrder>,
    pub purchase_order_line_id: Id<PurchaseOrderLine>,
    pub supplier_id: Id<Supplier>,
    pub purchase_type: PurchaseType,
}

pub struct PurchaseOrderInvoiceState {
    pub purchase_order_id: Id<PurchaseOrder>,
    pub purchase_order_line_id: Id<PurchaseOrderLine>,
    pub supplier_id: Id<Supplier>,
    pub purchase_type: PurchaseType,
    pub ordered_quantity: Quantity,
    pub invoiced_quantity: Quantity,
    pub line_status: PurchaseOrderLineStatus,
    pub line_row_version: i64,
    pub order_status: PurchaseOrderStatus,
    pub order_row_version: i64,
}

pub struct PurchaseOrderInvoiceLineEffect {
    pub purchase_invoice_line_id: Id<PurchaseInvoiceLine>,
    pub purchase_order_line_id: Id<PurchaseOrderLine>,
    pub quantity_delta: Quantity,
}

pub struct PurchaseOrderInvoiceWriteback {
    pub purchase_invoice_id: Id<PurchaseInvoice>,
    pub lines: Vec<PurchaseOrderInvoiceLineEffect>,
}

pub struct PurchaseOrderInvoiceReversalWriteback {
    pub original_purchase_invoice_id: Id<PurchaseInvoice>,
    pub invoice_reversal_id: Id<InvoiceReversal>,
    pub lines: Vec<PurchaseOrderInvoiceLineEffect>,
}

pub struct PurchaseOrderInvoiceLineResult {
    pub purchase_order_id: Id<PurchaseOrder>,
    pub purchase_order_line_id: Id<PurchaseOrderLine>,
    pub invoiced_quantity: Quantity,
    pub line_status: PurchaseOrderLineStatus,
    pub line_row_version: i64,
    pub order_status: PurchaseOrderStatus,
    pub order_row_version: i64,
}

#[async_trait::async_trait]
pub trait PurchaseOrderInvoicingPort: Send + Sync {
    async fn targets(
        &self,
        tx: &mut dyn Tx,
        ctx: &SecurityContext,
        purchase_order_line_ids: &[Id<PurchaseOrderLine>],
    ) -> Result<Vec<PurchaseOrderInvoiceTarget>, AppError>;

    async fn lock_targets_after_global(
        &self,
        tx: &mut dyn Tx,
        ctx: &SecurityContext,
        purchase_order_line_ids: &[Id<PurchaseOrderLine>],
    ) -> Result<Vec<PurchaseOrderInvoiceTarget>, AppError>;

    async fn states_after_seal(
        &self,
        tx: &mut dyn Tx,
        ctx: &SecurityContext,
        f50_lock_proof: &TransactionLockProof,
        purchase_order_line_ids: &[Id<PurchaseOrderLine>],
    ) -> Result<Vec<PurchaseOrderInvoiceState>, AppError>;

    async fn record_invoice(
        &self,
        tx: &mut dyn Tx,
        ctx: &SecurityContext,
        f50_lock_proof: &TransactionLockProof,
        cmd: PurchaseOrderInvoiceWriteback,
    ) -> Result<Vec<PurchaseOrderInvoiceLineResult>, AppError>;

    async fn reverse_invoice(
        &self,
        tx: &mut dyn Tx,
        ctx: &SecurityContext,
        f50_lock_proof: &TransactionLockProof,
        cmd: PurchaseOrderInvoiceReversalWriteback,
    ) -> Result<Vec<PurchaseOrderInvoiceLineResult>, AppError>;
}
```

`PurchaseRequisitionIntakePort` 的 source 形状固定为：CONTRACT 用 `source_doc_id=contract_id/source_doc_line_id=contract_line_id`；SALES_ORDER 用 order/order-line 且 `source_contract_id` 必填；PROJECT_TASK 用 project/task，合同可空；STOCK_SHORTAGE 用 warehouse/material。Material 要求 material 非空且 expense 为空；DirectExpense 反之并要求 contract/sales/project 至少一个权威归集；`is_drop_ship=true` 只允许 DirectExpense。来源幂等重放返回第一次完整 view，输入快照变化返回 payload mismatch，不新建第二单。四类调用者都只以回执的 `purchase_requisition_id/doc_no` 写自己的 target/link，不猜 id。

`link_drop_ship_return` 的 lines 非空、sales return line id 唯一并按 UUID bytes 升序；owner 通过 purchase requisition 来源链为每条销售订单行解析恰一条未终态 DirectExpense purchase order line，零条或多条均以 `PROCURE.PURCHASE_RETURN.SALES_RETURN_LINK_REQUIRED` 失败且零写入。它幂等创建/返回一张 `return_scenario=DROP_SHIP,status=DRAFT,sales_return_id=...` 的采购退货及逐行链接；`UNIQUE(legal_entity_id,sales_return_id)` 在 sales_return_id 非空时自然保证一张销售退货只有一张采购退货，NULLS DISTINCT 仍允许普通物料退货。deferred graph trigger 逐行验证 sales return line 属于该头、sales order line 相同且数量相等，不能靠调用方声明越过祖先关系。返回 lines 按 sales return line id 升序。

`PurchaseOrderInvoicingPort::targets` 是 proof 前无锁标识收集，只返回构造锁计划必需的订单 id、行 id、供应商 id 与采购类型；`lock_targets_after_global` 只能在 coordinator 已完成全部全局类别后按 order id/line id 升序 `FOR UPDATE`，仍只返回同一标识形状。两次结果都按 `purchase_order_id UUID bytes ASC,purchase_order_line_id UUID bytes ASC` 排序且无重复，调用方只以规范化键重建 plan；任何一项不得返回或读取数量、金额、status、row_version，也不得把这些字段塞进 `F50LockPlan`。`states_after_seal` 必须收到同事务同法人的已 seal proof，首句验证其覆盖全部订单/行后只重读已锁行，不补锁，才按同一顺序恰返回一项/输入行的数量、status 与 row_version；空输入三个读取方法都不访问数据库并返回空数组，非空输入不得重复。两个 mutator 同样先验证 proof，再只消费已锁行并在 owner 内重读当前状态；`PurchaseOrderInvoiceLineEffect` 不携带调用方 expected version。register 只加正 delta，reverse 只减正 delta，逐行强制 `0<=invoiced_quantity<=ordered_quantity` 并真实更新 `row_version+1`。DirectExpense 行在累计等于 ordered 时置 CLOSED，全部行闭合后订单置 COMPLETED；红字减少后行恢复 OPEN、COMPLETED 订单恢复 SUPPLIER_CONFIRMED。Material 订单完成仍由收货数量驱动，开票回写只维护 invoiced quantity 与 `is_type_locked=true`。发票/红字状态条件更新是唯一调用胜者，重放不二次累计；proof 缺失、覆盖不全、锁后容量不足或状态不容许均零写入失败关闭。

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

全部写请求必须带 `Idempotency-Key`，幂等作用域为法人、用户、端点、键值四元组，重复请求且 `request_hash` 相同时返回首次结果并带 `Idempotent-Replay: true`。采购退货 post 的规范摘要必须包含 path `purchase_return_id`、`expected_row_version` 以及按冻结顺序序列化的全部 `credit_note_identifiers`（含每个新红字法定标识的四字段）；同键更换任一原票 id 或 identifier 必须返回 payload mismatch，不能重用首次红字。其他字段按基线规范化。本阶段不引入任何业务侧的第二套幂等机制，第 4.6 小节的来源幂等键是采购需求这一个对象的去重依据，不替代 `Idempotency-Key`。

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
| POST /api/v1/procure/purchase-orders/{id}/actions/submit-for-approval | 提交至必经的 PROCURE_MANAGER 审批；配置或节点展开无效时 fail-closed | submit | PROCURE.PURCHASE_ORDER.SUPPLIER_QUALIFICATION_EXPIRED、PROCURE.PURCHASE_ORDER.BATCH_QUANTITY_MISMATCH、PROCURE.PURCHASE_REQUISITION.ORDERED_QUANTITY_EXCEEDED、PLATFORM.APPROVAL.NODE_HAS_NO_APPROVER、PLATFORM.SOD.SELF_APPROVAL_FORBIDDEN |
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
| POST /api/v1/procure/purchase-returns/{id}/actions/submit-for-approval | 提交至必经的 PROCURE_MANAGER 审批；配置或节点展开无效时 fail-closed | submit | PROCURE.PURCHASE_RETURN.QUANTITY_EXCEEDS_RETURNABLE、PROCURE.PURCHASE_RETURN.BATCH_OR_SERIAL_NOT_IN_RECEIPT、PLATFORM.APPROVAL.NODE_HAS_NO_APPROVER、PLATFORM.SOD.SELF_APPROVAL_FORBIDDEN |
| POST /api/v1/procure/purchase-returns/{id}/actions/post | 仅全部审批节点通过后过账并执行第 4.4 小节算法；body 恰为 `PostPurchaseReturn { expected_row_version,credit_note_identifiers }`，后者与锁后实际 billed 原票键集合精确相等；不得由 submit 绕过审批调用 | post | PROCURE.PURCHASE_RETURN.NEGATIVE_STOCK_BLOCKED、PLATFORM.APPROVAL.NODE_HAS_NO_APPROVER、PLATFORM.SOD.SELF_APPROVAL_FORBIDDEN |
| POST /api/v1/procure/purchase-returns/{id}/actions/void | 作废未过账单据 | void | PROCURE.PURCHASE_RETURN.ILLEGAL_STATUS_TRANSITION |
| POST /api/v1/procure/purchase-returns/{id}/actions/record-supplier-refusal | 仅直运退货；PROCURE_MANAGER 提交 reason、evidence_attachment_ids 与 row_version，建立指定 FINANCE_MANAGER 的审批；审批人须重新认证，批准事务调用 `CostReturnMarkPort`。路由、调用点与真实实现随阶段 11 同批启用，本阶段只冻结契约且不注册空路由 | submit | PROCURE.PURCHASE_RETURN.ILLEGAL_STATUS_TRANSITION、PROCURE.PURCHASE_RETURN.SALES_RETURN_LINK_REQUIRED、PLATFORM.AUTHZ.REAUTH_REQUIRED |
| GET /api/v1/procure/purchase-returns 与 /{id} | 列表与详情；详情分列 `physical_return_voucher_id?` 与按 id 排序的 `purchase_credit_note_voucher_ids[]`，不返回含义不明的单一 voucher_id | read | — |

`actions/post` 是附录 A.1 度量项「退货登记」在采购侧的度量端点。

#### 5.5 付款申请

| 方法与路径 | 说明 | 权限动作 | 主要错误码 |
|---|---|---|---|
| POST /api/v1/procure/payment-requests | 创建草稿 | create | PROCURE.PAYMENT_REQUEST.SUPPLIER_TERMINATED、PROCURE.PAYMENT_REQUEST.PAYEE_ACCOUNT_MISSING |
| PATCH /api/v1/procure/payment-requests/{id} | 草稿修改 | update | PLATFORM.CONCURRENCY.STALE_VERSION |
| POST /api/v1/procure/payment-requests/{id}/actions/submit-for-approval | 提交并占用 | submit | PROCURE.PAYMENT_REQUEST.AMOUNT_EXCEEDS_OPEN_BALANCE、PROCURE.PAYMENT_REQUEST.DUPLICATE_INVOICE_RESERVATION、PLATFORM.SOD.DUTY_CONFLICT |
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
| GET 与 POST /api/v1/portal/supplier-invoice-uploads | INVOICE_UPLOAD | 查询与上传发票元数据；POST 按同法人、同供应商、规范化号码防重，命中返回 `PORTAL.SUPPLIER_INVOICE_UPLOAD.DUPLICATED` |
| POST /api/v1/portal/supplier-invoice-uploads/{id}/actions/return | 内部财务权限，不属于供应商门户能力 | 请求只含 `reason`、`row_version`；把 `row_version` 原样映射为 `expected_row_version` 后调用 `SupplierInvoiceUploadWritebackPort::return_upload`，锁后状态或版本失配返回 `PORTAL.SUPPLIER_INVOICE_UPLOAD.STATE_CHANGED`，成功后发布 `portal.supplier_invoice_upload.returned.v1`；供应商主体调用返回 `PLATFORM.AUTHZ.INTERNAL_ROLE_FOR_PORTAL_FORBIDDEN` |
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

> **IPC 身份核验现行替代。** 本节后文旧字面「客户端 PID token 账户二次核对」不得实现：core 在读应用字节前执行 `ImpersonateNamedPipeClient`→`OpenThreadToken` 核验 `NT SERVICE\ep-portal`，并在所有分支 `RevertToSelf`，PID 只审计；portal 以 `SECURITY_SQOS_PRESENT|SECURITY_IDENTIFICATION` 打开管道并在发送前核验 core 进程 token，每次重连重新核验。只能按下文精确 operation 授权，不得信任管道封套中的主体或法人声明。

路径前缀 `/portal/v1`，与第 5.7 小节一一对应，另加三个会话端点。

| 方法与路径 | 说明 |
|---|---|
| POST /portal/v1/session/login | 转发到 core-server 的门户身份认证入口，成功后下发仅 HttpOnly、Secure、SameSite=Strict 的会话 cookie |
| POST /portal/v1/session/logout | 注销 |
| GET /portal/v1/session/current | 当前账号、绑定供应商与授权法人集合 |

portal-gateway 在每个请求上做四件事：本地 cookie 形状与缓存提示校验、按账号与来源地址限流、构造未受信的 `PortalPipeRequest { opaque_session_token, requested_legal_entity_id, device_id, request_id }`、在响应上叠加水印呈现参数。该封套没有 user/account/supplier/role/duty/data-scope/client 字段，`requested_legal_entity_id` 只是请求选择，不是授权事实。core-server 只从管道账户固定 `ClientKind::Portal`，重新校验 session token、`account_kind=PORTAL`、device 绑定、供应商绑定与授权法人集合后自行构造 `SecurityContext`；内部员工 token、伪法人、伪 device、自填 client 或封套多余主体字段一律拒绝。门户请求在 portal-gateway 新建 trace，不接受外部传入的 `traceparent`，公网侧关联标识放入 `X-Correlation-Id`，按基线第 9.3 节。

portal-gateway 到 core-server 的三项身份操作与五项业务能力只走固定 `\\.\pipe\ep-core`，不请求 `127.0.0.1:8080`，也不配置 core API URL。管道 server 是 `NT SERVICE\ep-core`；DACL 精确增加客户端 `NT SERVICE\ep-portal`。服务端在读取应用字节前执行 `ImpersonateNamedPipeClient`，以 `OpenThreadToken` 取得冒充线程 token 并核验服务 SID/账户为 `NT SERVICE\ep-portal`，所有分支执行 `RevertToSelf`；客户端 PID 仅写审计，不参与身份或授权判定。实现 allowlist 必须逐项列出：身份操作为 `portal.session.sign_in.v1`、`portal.session.sign_out.v1`、`portal.identity.me.v1`；五项业务能力由 `portal.order_confirm.v1`、`portal.delivery_notice.v1`、`portal.invoice_upload.begin.v1`、`portal.invoice_upload.chunk.v1`、`portal.invoice_upload.end.v1`、`portal.invoice_upload.abort.v1`、`portal.settlement_query.v1`、`portal.profile_maintain.v1` 八个 operation 承载。不得以 `portal.*` 或 `portal.invoice_upload.*` 通配实现白名单。普通请求沿用 4 字节大端长度加 JSON、单帧不超过 1 MiB；发票正文复用基线唯一 `BoundedChunkStreamV1`，总长上限 52428800，解码块上限 524288、seq 从 0 连续、每块 ACK 后才发下一块、ACK/空闲/绝对超时为 10/30/3600 秒并校验块哈希和总 SHA-256。乱序、重复、缺块、超限、长度或哈希不符立即 abort，重试使用新 UUIDv7 request_id；portal-gateway 与 core-server 都只保留单块加固定协议开销，不把明文写临时文件。core-server 的 8080 与 portal-gateway 的 8090 只作为第三方反向代理 upstream，不得把这两个端口恢复成产品进程业务 IPC。

---

### 6. 并发与事务边界

#### 6.1 事务边界总表

| 用例 | 事务内包含 | 事务外经 Outbox |
|---|---|---|
| 采购需求生成 | 需求单写入、幂等 finish、Outbox 条目、同事务通知命令、审计终结批 | 站内通知投递、检索索引 |
| 采购订单提交 | 订单头与行与批次行写入、需求累计下达数量回写、流程实例启动、幂等 finish、Outbox、同事务通知命令、审计终结批 | 站内通知投递、门户待确认投影刷新、检索索引 |
| 收货过账 | 采购单据写入、会计期间解析、库存数量账与金额账写入与取价、入账分配写入、总账凭证生成、订单行与批次行与送货通知回写、幂等 finish、Outbox、同事务通知命令、审计终结批 | 站内通知投递、门户投影刷新、检索索引 |
| 采购退货过账 | 退货单写入、库存两账写入与取价、总账凭证生成、收货行与订单行回写、幂等 finish、Outbox、同事务通知命令、审计终结批 | 供应商质量记录生成、站内通知投递、门户投影刷新 |
| 付款申请提交 | 申请头与行写入、流程实例启动、幂等 finish、Outbox、同事务通知命令、审计终结批 | 站内通知投递 |
| 门户订单确认与改期 | 订单状态与交期更新、幂等 finish、Outbox、同事务通知命令、审计终结批 | 给采购主管的站内通知投递、门户投影刷新 |
| 门户送货通知提交 | 通知头与行写入、幂等 finish、Outbox、同事务通知命令、审计终结批 | 给仓管员的站内通知投递、待收货列表投影刷新 |
| 门户发票上传 | 上传记录写入、附件元数据关联、幂等 finish、Outbox、同事务通知命令、审计终结批 | 给财务的站内通知投递、待登记队列投影刷新 |
| 门户档案变更提交 | 经 `SupplierSelfServiceCommand::submit_profile_change` 在同一事务内创建变更申请、幂等 finish、Outbox、同事务通知命令、审计终结批 | 给采购负责人的站内通知投递 |

每个用例一个事务，一个 HTTP 请求内不开启第二个写事务。事务内禁止外部 HTTP 调用、文件正文读写、通知发送与长时计算。附件正文的上传经 `platform_file` 的上传流水线在业务事务之外完成，业务事务只写附件关联表中的元数据引用。

所有写用例采用同一收口顺序：先按本章冻结的跨模块锁序完成业务事实、子账、凭证与同步投影，再执行幂等 `finish`，再写 Outbox，再写确需同事务落库的通知命令，最后调用 `AuditWriter::append_terminal` 批量落审计；不存在的类别跳过但后缀不得调换。`append_terminal` 封印 `Tx`，其后任何 SQL `query/execute`、仓储写入或跨模块端口调用都必须以内置不变量错误拒绝并令整个事务回滚，唯一允许的后继动作是 `UnitOfWork::commit`。共享跨模块契约测试 `audit_terminal_seals_tx` 以收货和采购退货为夹具：审计后分别尝试本地仓储、`InventoryPostingPort`、`PostingPort` 与 `PurchaseCreditNotePort` 写入，均应失败、审计后零新增行、事务整体回滚；正常路径由 recording transaction 断言审计批是 commit 前最后一批数据库执行。

#### 6.2 隔离级别与超时

业务事务一律 `READ COMMITTED`。本阶段不引入 `REPEATABLE READ` 事务，第 8.6 小节的六个 `ReconCheck` 由 `ep-platform-recon` 的执行器在其提供的快照上执行，快照上下文类型为 `ep_foundation::port::SnapshotCtx`。事务预算沿用基线第 10.3 节：业务事务不超过 5 秒，读写池 `statement_timeout` 10 秒，`lock_timeout` 3 秒，`idle_in_transaction_session_timeout` 15 秒。

#### 6.3 锁策略

| 场景 | 锁 | 顺序 |
|---|---|---|
| 收货过账 | 采购订单行、交货批次行、送货通知行的 `FOR UPDATE` | 按各自主键升序，先订单行、后批次行、再通知行 |
| 采购退货过账 | F-50 第 10 节统一锁序中本事务实际存在对象的 `FOR UPDATE` | 先无锁收集；严格按原款项/退款单 → 退款来源链接 → 采购退货单 → 原发票头 → 原发票行 → 收货行 → GRNI 根/效果 → 库存余额/金额 → 应收/应付正向主条目 → 预收/预付条目 → 核销根 → 核销效果行 → 冲销行与累计；无关类别跳过，每类 `id ASC`，集合漂移整事务重试 |
| 付款申请提交、付款/冲正与终态释放 | F-50 PayableReservation 类别先取 `payable-reservation:` advisory 再锁既有汇总行；AP original entry 同属同一 plan | 按 `purchase_invoice_id` 升序；mutator 只验 proof、不补锁；状态迁移唯一胜者才增减，所有真更新显式 `row_version+1` |
| 采购需求累计下达 | 需求行 `FOR UPDATE` | 按主键升序 |
| 门户订单确认与本方改期处理 | 订单头 `FOR UPDATE` | 单行 |

固定升序取锁是死锁避免的第一手段。序列化失败 40001 与死锁 40P01 由数据访问层重试 3 次，退避 50、150、450 毫秒，只对尚未产生任何外部可见副作用的事务重试，重试次数进 `ep_db_tx_retries_total`，标签为 pool 与 sqlstate，该指标由阶段 2 注册与填充。

#### 6.4 乐观锁

采购需求、采购订单及其行与批次行、收货单、采购退货单、付款申请、供应商准入、门户绑定、送货通知、发票上传均带 `row_version`，更新一律按基线第 3.7 节的条件写法。普通更新受影响行数为 0 返回 409 与 `PLATFORM.CONCURRENCY.STALE_VERSION` 并回带当前版本号与最后修改人；供应商发票上传的接受/退回 owner port 按 F-50 专用契约把状态或版本变化统一映射为 `PORTAL.SUPPLIER_INVOICE_UPLOAD.STATE_CHANGED`，不返回第二个并发码。`procure.goods_receipt_line_costings` 为仅追加表，不带 `row_version`。

#### 6.5 幂等与 Outbox

写请求的 `Idempotency-Key` 与业务写入同事务，存储在 `platform_msg.idempotency_keys`，保留 7 天。本阶段的全部领域事件与业务状态、审计事件处于同一事务，写入次序固定为业务/子账/凭证/投影、幂等 `finish`、Outbox、同事务通知命令、审计终结批；事务提交前不发起任何外部调用。消费端幂等由 `platform_msg.inbox_consumptions` 的唯一约束保证，消费副作用与该行插入同事务。

本阶段的 Outbox 事件信封一律携带 `posting_date` 与 `accounting_period_id` 两个字段：收货过账与采购退货过账事件取其单据上的实际取值，其余事件取空值。`procure.goods_receipt.posted.v1` 与 `procure.purchase_return.posted.v1` 两个事件在 `ledger.posting_trigger_event_types` 中的登记行按裁定 A-21 由阶段 9a 的种子迁移一次写入，本阶段不新增回填迁移。该登记行按裁定 A-21 与总览第 1.5 节第三条每行只填 `event_type`，原有的 `ledger_event_kind` 与 `registered_by_module` 两列已删除，本阶段不得再引用；`ep_contract_ledger::PostingTriggerRegistry::assert_registered` 与错误码 `LEDGER.POSTING_TRIGGER_EVENT_TYPE.REGISTRY_MISMATCH` 整项撤销，本阶段不做启动自检、不做 `--check` 静态断言，也不向阶段 9b 的关账受理追加任何前置校验，关账受理前提仍为两条。登记表一致性的承接方只有两条：`xtask configdoc` 从 `docs/event-catalog.md` 生成阶段 9a 的种子迁移并在 CI 中逐字比对，以及阶段 3b 的 `event-catalog-consistent` 自检项，该项取 Degrading 且不通过时停止派发未登记事件类型。本模块最终 15 个事件含 F-50 新增的 `portal.supplier_invoice_upload.returned.v1`，与 `docs/event-catalog.md` 逐字比对通过即为达标；`accepted` 由阶段 10 受理事务产生，不计入本阶段。两个 posting-trigger 事件在 PENDING 或 DISPATCHING 状态下进入关账受理前提二的统计，`posting_date` 为空的其余事件不计入该统计。

15 个事件的唯一清单固定如下，不允许以匿名 Outbox 条目补足计数：

1. 采购需求：`procure.purchase_requisition.created.v1`、`procure.purchase_requisition.closed.v1`。
2. 采购订单及门户确认：`procure.purchase_order.submitted.v1`、`procure.purchase_order.issued.v1`、`procure.purchase_order.reschedule_proposed.v1`、`procure.purchase_order.supplier_confirmed.v1`。
3. 收退货：`procure.goods_receipt.posted.v1`、`procure.receipt_rejection.registered.v1`、`procure.purchase_return.submitted.v1`、`procure.purchase_return.posted.v1`。
4. 付款申请：`procure.payment_request.submitted.v1`、`procure.payment_request.approved.v1`。
5. 门户单据：`portal.delivery_notice.submitted.v1`、`portal.supplier_invoice_upload.uploaded.v1`、`portal.supplier_invoice_upload.returned.v1`。

各事件的唯一触发点、payload、消费者与 `produces_voucher` 逐项以 `docs/event-catalog.md` 为准。门户档案变更由 mdm owner 在 `SupplierSelfServiceCommand` 事务内产生其自有事件，不冒充阶段 7 事件；供应商上传 `accepted` 由阶段 10 产生，也不计入上述 15 项。其他只需审计而无需跨模块传播的状态变更不创建匿名领域事件。

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

#### 7.1 进程配置（`C:\EP\config\config.toml` 与环境变量）

| 键名 | 类型 | 默认值 | 承载进程 | 生效方式 |
|---|---|---|---|---|
| EP__PORTAL__SESSION__MAX_AGE_SECONDS | u32 | 7200 | portal-gateway | 重启生效 |
| EP__PORTAL__SESSION__IDLE_TIMEOUT_SECONDS | u32 | 900 | portal-gateway | 重启生效 |
| EP__PORTAL__SESSION__VALIDATION_CACHE_TTL_SECONDS | u32 | 30 | portal-gateway | 重启生效 |
| EP__PORTAL__RATE_LIMIT__REQUESTS_PER_MINUTE | u32 | 120 | portal-gateway | 重启生效 |
| EP__PORTAL__RATE_LIMIT__BURST | u32 | 40 | portal-gateway | 重启生效 |
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

`EP__PORTAL__SELF_REGISTRATION__ENABLED` 冻结为默认 `false`。受限自助注册代码完整保留（注册频率限制、邀请码校验、待审核账号只能访问自身注册状态三项）；只有许可证、签名配置与安全审批三者同时满足才允许未来正式变更为启用，首版发布不以启用为验收项。

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

八个参数的出厂默认值就是 F-51 确认的当前规范值，实现方无二次选择。未来若经正式变更流程调整，只需通过配置发布通道发布，不改代码、表结构或迁移；变更不追溯重算历史单据。

---

### 8. 测试计划

#### 8.1 单元测试

位于 `ep-domain-procure` 与 `ep-domain-portal` 内，不触网、不触库、不触文件系统、不取真实时间，`Clock` 一律注入 `FixedClock`。覆盖的分支如下。

采购需求：四类来源各自的生成守卫；来源单据行已关闭的拒绝；同一 `source_idempotency_key` 的去重；累计下达数量三档状态迁移的边界（等于零、介于之间、等于需求数量）；手工关闭与来源作废触发关闭两条路径；直运来源强制 `DIRECT_EXPENSE` 且不允许改为物料类。来源形状逐项覆盖：CONTRACT/SALES_ORDER 缺 source_contract_id 被拒，PROJECT_TASK 缺 project_id 被拒；有合同派生任务固化 project_id+contract_id，无合同手工任务只固化 project_id 且成功建单，STOCK_SHORTAGE 的三项业务归集 id 均为空。另对 `CLM_TERM_PURCHASE_REQUISITION` 逐项覆盖 CONTRACT、SALES_ORDER、有合同 PROJECT_TASK 三支命中与无合同 PROJECT_TASK/STOCK_SHORTAGE/异合同不命中，PENDING/PARTIALLY_ORDERED 自动关闭、ORDERED 转人工，以及 `Completed/AlreadySatisfied/NeedsManualDecision` 三态；两个 decision code 的状态矩阵、非空同单 `decision_result_doc_id`、非空 reason 与错码拒绝均各有正反例。

采购订单：物料类与直接费用类各自的必填字段矩阵；交货批次行数量合计等于行数量的断言，含合计大于与合计小于两个反例；约定交期不早于订单日期；累计下达数量不超过需求数量；十一态状态机的全部合法迁移与至少同等数量的非法迁移拒绝；作废三条前置条件的四种组合；已收货行不可变更数量与单价；供应商四态对下单的四种约束。另逐状态覆盖第 4.2.8 节 VOID/CLOSE/终态保持的唯一映射，断言真实变更版本加一、终态保持版本不变及 owner audit 三键 before/after、固定 action/reason/object/version/time。

收货单：超收、短收、平收三分支；批次管理与序列号管理开启与关闭的四种组合；序列号条数等于数量的断言；`posting_date` 晚于服务器自然日的拒绝；入账分配数量合计等于行数量的断言，含一条分配与两条分配两种形态；post 动作只消费路径上的既有头行且成功/失败后 id 均不变；全零会计效果接受 Skipped、非零只接受 Posted、首次 IdempotentReplay 与结果/期间错配失败关闭；过账后不可修改。

采购退货：可退数量计算（收货数量减累计退货数量）的边界；物料类与直运两个场景的必填字段矩阵；批次与序列号必须来自原收货行；post 动作只消费路径上的既有 PENDING_APPROVAL 头行；一行退货关联多次收货时按各自分配逐条取价的入参构造；记录型端口断言 collect/reload 只各调一次 `lock_candidates_for_receipt_lines` 且返回不含量额/版本，seal 前三种容量查询调用次数均为零，seal 后 MATERIAL 只调带同一 proof 的 `match_states`、DROP_SHIP 只调带同一 proof 的 `billed_allocations_for_purchase_invoice_lines`；Stage 10 接线夹具另断言 `PurchaseOrderInvoicingPort::targets/lock_targets_after_global` 只返回四项标识且键集合相等，seal 后才调 `states_after_seal`，writeback effect 不含 expected version；跨两张原票时按原票分组各一张红字、请求 identifier 键集合须与实际原票集合精确相等；物理三项全零接受 Skipped 并保持 voucher 空，非零只接受 Posted；累计冲回不超过原归集金额。

付款申请：发票付款与预付款两类型的必填矩阵；占用金额加已占用不超过未核销余额的边界；九态状态机；已付金额不超过申请金额的断言；撤回、驳回、作废、关闭四条释放路径；第二张申请命中 upsert 冲突分支时 row_version 恰加一，释放重放不重复扣减，并发加占用/释放只形成合法串行结果且金额与计数永不为负。另逐状态覆盖第 4.2.8 节 VOID/WITHDRAW/CLOSE/终态保持映射，FULLY_PAID 与未闭合付款效果必须拒绝，并断言 owner audit 的固定 action、根引用、版本和同事务零部分写入。

门户：五项能力码的白名单裁剪，逐能力逐字段断言不出现禁止字段；数据范围两条断言的四种组合（供应商匹配与否、法人在授权集合与否）；送货通知累计数量不超过订单行剩余待收数量；送货通知被引用后不可作废；发票上传的价税合计等式与同号重复判定。

领域属性测试（proptest）覆盖本阶段承担的四组不变量，对应规格第 17.3 章：

1. 采购订单行的累计收货数量减累计退货数量在任意合法操作序列后不小于零，且不超过订单数量加放行的超收量。
2. 收货行的累计退货数量在任意合法操作序列后不超过该行收货数量。
3. 收货行入账分配的数量合计在任意合法操作序列后恒等于该行收货数量。
4. 同一采购发票的占用金额合计在任意申请提交与释放序列后恒等于当前未关闭申请行的金额合计，且不小于零。

#### 8.2 集成测试

位于各 crate 的 `tests/` 与 `apps/core-server/tests/`、`apps/portal-gateway/tests/`。使用真实 PostgreSQL 16，每个用例独占一个 `ep_test_<nanoid>` 数据库，用例结束即删库。不使用内存库或 mock 替代数据库。

场景清单：

1. 迁移正向执行与按 `-- rollback:` 段回退，三十三个文件逐个验证，其中 append-only 登记回填文件验证正向插入与回退删除两个方向，portal 外键追补文件验证三条长复合外键、四表延迟触发器及孤儿/错祖先预检；系统目录还须逐项查得采购订单头行批次、收货六表、采购退货六表的候选键、长复合外键、延迟函数与约束触发器，并验证回退按触发器、函数、外键、表的依赖逆序无残留。对 `goods_receipt_line_costings` 的根 FK 与长父 FK 查询 `pg_constraint`，逐条断言 `confdeltype=RESTRICT`、`condeferrable=true`、`condeferred=true`；空表预生成 UUIDv7 后单 INSERT `id=root_effect_id` 成功 COMMIT，缺根、跨法人根、跨收货行父或跨根父分别失败且零残行。断言 `db/checks/append_only_consistency.sql` 返回零行；静态扫描断言这 33 个常规事务文件均不含 `CREATE INDEX CONCURRENTLY`，且全部新空表索引由同一建表迁移用普通 `CREATE INDEX` 建成。
2. 三十一张表的行级安全策略生效：变量缺失时不可见不可写；跨法人上下文读、写、更新、聚合、排序六类操作均不返回也不写入他法人数据。
3. 采购需求四类来源的端到端生成，含合同派生事件的重复投递 3 次只产生一条需求；PROJECT_TASK 另以合同派生任务、无合同手工任务各跑一例，二者都固化 project_id，后者的 source_contract_id/contract_id 保持空。库存不足扫描必须注入真实 `SalesAwareReplenishmentPolicyQuery`，验证它与销售 A2 对同一组合返回完全相同的 `available_qty`，并覆盖阈值为空、`available_qty` 分别大于/等于/小于 `reorder_point`、建议量公式、按组合分页无遗漏无重复、`limit=1/500` 接受而 `0/501` 精确拒绝、同扫描时段重试、跨扫描时段已有未结需求以及 CLOSED 后下一时段重建。绕过服务直写同一法人+仓库+物料的第二张未结 STOCK_SHORTAGE 需求必须被生成键唯一约束拒绝。同一真实库场景另造三支合同归属需求各一条及无合同 PROJECT_TASK/STOCK_SHORTAGE/异合同反例，断言 ImpactRule 只命中前三条；自动关闭、ORDERED 两种决策码、结果 id 错单、空 reason、重放不重复审计，以及规则注册后累计恰为 4 全部通过。
4. 采购订单从草稿到下达到供应商确认的全链路：出厂单节点 `PROCURE_MANAGER` 链由非申请人的有效经理全部通过后成功；审批链缺失/零节点与节点存在但展开集合为空分别返回 `PLATFORM.APPROVAL.NODE_HAS_NO_APPROVER`，申请人落入审批集合返回 `PLATFORM.SOD.SELF_APPROVAL_FORBIDDEN`。三类拒绝均断言单据状态、`row_version` 不变，且采购业务、审批任务、库存、凭证、审计与 Outbox 零写入。另绕过服务 direct SQL 构造头金额与行汇总不等、负累计、`returned_quantity>received_quantity`、`invoiced_quantity>quantity`、`FULLY_RECEIVED` 却未收满，以及普通 FK 全部命中但行/批次跨订单拼接，均须在语句或 COMMIT 被数据库拒绝。该场景复用同一根再跑第 4.2.8 节迁移撤销，逐一覆盖 VOID、CLOSE、CLOSED/VOIDED 保持和依赖未反向拒绝；断言 owner audit 与 R0 为两个同法人同 occurred_at 的事件、receipt target 指 owner event，而错 action/root/before/after/version、复用 R0 event id 或只改根的夹具在 Stage 14 延迟图提交时失败。
5. 分批订货两种形态：需求侧分多次下达为多张订单，累计不超过需求数量；订单侧一行多个交货批次，按批次逐次收货。direct SQL 另覆盖批次数量合计不等订单行、批次累计合计不等行累计、批次状态与累计不符三组提交失败反例。
6. 收货过账的同事务性、既有 id 与状态形状验证：先由创建端点落一份 DRAFT 收货及真实行 id，再注入总账契约失败与库存契约失败，断言该头仍为 DRAFT、头行 id/摘要逐值不变，入账分配、库存流水、凭证与 Outbox 零新增；成功时断言 inventory source_doc_id/source_line_id 恰为路径头/既有行并原子写 POSTED/period/posted_at。分别覆盖非零 Posted 与零价收货 Skipped，后者仍有库存数量事实和 quantity>0/amount=0 的 GRNI 根，但 voucher 为空；首次 IdempotentReplay、非零 Skipped、零值 Posted 均回滚。再直写五个状态的合法形状及过账列/效果图非法组合；额外构造普通 FK 各自全命中但收货头、订单行、批次、notice 行跨头拼接，供应商/物料/仓库/单价快照错误，库存或 GRNI 来源/分段错误，累计数量漂移，以及超收没有有效审批证据，逐条由即时 CHECK、长复合外键或延迟图在语句或 COMMIT 拒绝。非零无 voucher、全零却有 voucher、voucher 来源或期间错误也必须整笔回滚。
7. 收货过账对采购订单行、交货批次行、送货通知行三处累计数量的回写正确性；另以 notice 头/行、PO 头/行各自 FK 都存在但相互错配的 direct SQL 证明长复合外键与四表延迟 notice 图拒绝跨祖先拼接，通知累计与状态漂移也在 COMMIT 失败。
8. 超收三分支：容差内直接过账；超出容差转审批后过账；拒收只登记记录不产生库存流水与凭证。
9. 采购退货：先覆盖与采购订单同组的出厂 `PROCURE_MANAGER` 成功路径、审批链缺失/零节点、节点展开为空及申请人自审反例；错误码与全域零写入断言完全相同，且任一反例都不得进入结存查询。由创建/审批端点得到既有 PENDING_APPROVAL 头行后再调用 path post，成功前后头行 id 不变，失败时头状态与行 reversal 字段不变。审批通过后，通过记录型端口断言过账只调用 `StockOnHandQueryPort::on_hand` 取得未扣销售需求的物理结存，且从不调用 `AvailabilityQueryPort::available`；结存不足按当前物理结存拒绝，充足才继续。未开票段按原暂估原额追加 `PURCHASE_RETURN/DECREASE`；已开票段按 distinct 原票 id 分组，每票恰调用一次链接红字，再逐父等额追加 `PURCHASE_RETURN/DECREASE`，两者同事务且 GRNI 净变化为零。以同一退货跨两张原票验证两张红字及顺序，并逐个覆盖 identifier 缺失、多余、重复、乱序、复用蓝票号码与 row_version 不一致，全部零写；纯未开票传非空 identifier 也拒绝。物理凭证来源恒为 `PURCHASE_RETURN_INVENTORY`，红字恒为物料 `PURCHASE_INVOICE_LINKED_RETURN_REVERSED` 或直接费用 `PURCHASE_INVOICE_DIRECT_EXPENSE_REVERSED`；同单混合段至多一张物理凭证。覆盖正负 `linked_return_price_difference_amount/return_carrying_difference_amount`、零金额但数量大于零的 GRNI 根与物理 Skipped、部分退货按当前移动平均价、全数退清时库存数量/金额/单价同时归零、重复根被数据库唯一键拒绝，以及红字/退货并发只能形成合法串行结果。另以空 `GrniInvoiceMatch[]`、空 `GrniCreditNoteReversal[]` 分别调用写回端口，断言精确拒绝且零写入；非空结果断言固定排序键、方向、来源行映射与 `total_amount=sum(effects.amount)`。任一端口失败均不得留下部分写入。数据库反例还要绕过服务构造普通 FK 均命中但退货行挂错收货头、costing、供应商或 sales return，MATERIAL/DROP_SHIP 效果半图、库存 movement 来源错退货、三项全零却有 physical voucher、非零缺 voucher，以及已 POSTED DROP_SHIP 无 linked 红字、红字跨退货或多余红字；均须由 Stage 7/10 两侧延迟触发器在 COMMIT 拒绝并全回滚。
   本项另以记录型 invoice/procure owner 强制调用时序：collect/reload 两次候选集合相同才 seal，proof 前若调用任一容量查询或 `PurchaseOrderInvoicingPort::states_after_seal` 立即失败；proof 前 `targets/lock_targets_after_global` 只返回订单/行/供应商/类型四项标识，proof 后 MATERIAL 返回逐收货行状态，DROP_SHIP 返回与既有 `purchase_invoice_line_id` 一一覆盖的 allocation，并调用 `states_after_seal` 读取已锁采购订单 quantity/status/row_version。两路的原票 id/行 id/row_version/可退 quantity、unit price、tax rate、net/tax/gross 均逐项核对，采购 writeback command 不带 expected version；乱序、缺项、多项、零容量成功或 proof 覆盖不足全部零写入。Stage 10 第 19090930 号双向图的 direct-SQL 负例还覆盖孤立 linked 红字、非 POSTED 退货、错法人/供应商/原票头行/期间/记账日、非 INPUT+RED_LETTER、同原票分组缺票或多票、quantity/net/tax/gross 不等、物料 GRNI 父链不等额及 DROP_SHIP 错祖先/伪造库存或 GRNI；全部在 COMMIT 拒绝。
10. 一行退货关联多次收货：三次收货各自单价不同，一次退货跨三条分配，断言逐条取价与逐条回冲。
11. 直运采购退货：不产生库存流水，与销售退货单逐笔勾稽，分次冲回累计不超过原归集金额。
12. 付款申请与 AP 并发：该场景随 `INVOICE_PAYMENT` 分支在阶段 10 执行，不以任何桩替代余额取数。覆盖同发票申请提交分别与付款、进项红字、供应商返款再核销、资金冲正并发，断言 F-50 只形成合法串行结果且提交时始终 `effective_open >= reserved_amount`；另绕过服务直写两侧制造反例，必须被 deferred terminal trigger 在 COMMIT 拒绝。
13. 付款申请状态随付款登记/冲正回写的全链路：两次部分付款、最后一次付清、部分付款后 CLOSED、付款后资金冲正重开、CLOSED 后冲正不重开、每种精确重放与并发双提交。逐次断言 line/header paid 合计、reservation delta/count、所有真更新 `row_version+1`，以及唯一 finance 状态胜者之外零采购写入；本阶段先以 port 契约测试覆盖 owner 逻辑，阶段 10 再以真实财务付款/AP 效果覆盖跨模块事务。同一场景再覆盖迁移撤销的 VOID/WITHDRAW/CLOSE/终态保持与 FULLY_PAID/未闭合付款拒绝，逐项断言独立 owner audit target、R0 引用、reservation 释放、固定三键 before/after 和失败时根/占用/审计/receipt 零部分写入。
14. 供应商四态对采购需求、采购订单、付款申请、收货、退货五类操作的约束矩阵，共二十格逐格断言。
15. 门户五项能力的受控访问：以 A 供应商的门户账号访问 B 供应商的订单、收货、发票、付款四类对象，一律返回 404 与 `PLATFORM.AUTHZ.NOT_FOUND_OR_DENIED`；以门户账号访问任一内部端点一律被拒。
16. 门户返回字段白名单：对五项能力的响应做全字段快照（insta），任何新增字段导致快照失败，防止字段泄漏被静默引入。
17. 门户订单确认与改期的完整协商回合，含本方接受与本方拒绝两条路径与订单进入收货流程后拒绝门户操作。
18. 门户送货通知从提交、被部分引用、被全部引用到不可作废的全链路。
19. 门户发票上传的重复号码拒绝、退回后重传、受理后置终态三条路径；第三条路径在本阶段只断言 `ACCEPTED` 终态下的门户查询裁剪与状态取值，其终态由集成测试直接写入 `portal.supplier_invoice_uploads` 构造，受理回写本身随 E2E-T-03 的被受理一路在阶段 10 交付，本阶段不写受理入口，也不注入任何替身。
20. 门户对账查询的取数与内部应付台账同源：该场景随三个门户对账端点与 `SupplierStatementQuery` 在阶段 10 同批执行；本阶段执行的是门户对账查询在采购订单与收货两组字段上的取数与裁剪。
21. portal-gateway 的会话、限流与管道转发：三项身份操作和承载五项业务能力的八个 operation 都经 `ep-core` 精确 allowlist，HTTP 回环不可用/8080 被阻断时仍正常；以 `NT SERVICE\ep-portal` 之外账户调用任一 portal operation、`ep-portal` 调用未列 operation、或实现白名单出现通配模式均被 DACL、token 账户核对或契约测试拒绝。50 MiB 上传覆盖逐块 ACK、背压、乱序/重复/缺块、超限、块/总哈希错误、取消和三档超时，峰值缓冲不超过单块加固定协议开销；限流触发返回 429 并记入运维中心，未登记设备与非门户客户端取值被拒。
22. 幂等：本阶段全部写端点各执行一次重复提交，断言返回首次结果并带 `Idempotent-Replay: true`；键相同而载荷不同时返回 409。

外部电子签章不在本阶段范围内，本阶段不引入任何 wiremock 打桩。

第 9 项、第 12 项与第 20 项分别依赖阶段 10 的 `ReceiptInvoiceMatchQueryPort` 与 `PurchaseCreditNotePort`、`PayableLedgerQuery`、`SupplierStatementQuery`。三处一律不接替身，也不登记任何顺延验收：第 9 项在本阶段只执行发票未登记分支，已登记分支的用例代码与断言随两个端口在阶段 10 同批交付；第 12 项整条推迟到阶段 10；第 20 项随三个门户对账端点在阶段 10 同批交付。本阶段执行第 8.2 小节二十二个场景中的二十个，第 12 项与第 20 项不在其内，两项在第 9 节退出条件第 25 条逐条列名。

#### 8.3 端到端测试

后端 E2E 用 Rust 集成测试直接打 HTTP 接口，桌面端用 Playwright 驱动 WebView 与 tauri-driver，门户 Web 用 Playwright 驱动浏览器，移动端按规格第 6.2 章矩阵中「采购与供应商协同」一行取值为简化的范围执行 XCUITest 与 Espresso。本阶段的界面代码落在 `clients/desktop/src/modules/procure/` 与 `clients/mobile/src/modules/procure/`，门户站点由 portal-gateway 承载，三处均由上述用例覆盖。

用例清单：

- E2E-P-01 采购订货：从审批生效合同派生的采购需求出发，下达采购订单，门户确认交期，覆盖规格第 8 章第 4 步。
- E2E-P-02 分批订货：需求侧分两次下达，订单侧一行分三个交货批次，逐批次收货，对应规格第 17.2 章基础分支中的分批订货。
- E2E-P-03 收货暂估：按非零采购订单不含税单价暂估入库，断言既有收货单原子进入 POSTED、输出凭证号可查、入账分配为 `ESTIMATED`、库存两账同源；再以零单价跑同路径，断言 Skipped/voucher 空但库存数量事实、零金额 GRNI、权威状态、事件和审计完整。发票侧的回冲与价差在财务阶段联测。
- E2E-P-04 发票数量少于收货数量的收货侧：一次收货登记部分数量的发票后再次收货，断言未匹配部分的暂估继续留存。本阶段断言收货侧的入账分配与订单行累计数量，财务侧判据在财务阶段。
- E2E-P-05 一张发票跨多次收货的收货侧：三次收货形成三条入账分配，供发票侧逐次回冲。
- E2E-P-06 采购退货 GRNI：以既有审批通过退货 path id 覆盖未收票、已收票、同单混合及跨两张原票；已收票按每张原票各用一个新红字 identifier 执行 `PURCHASE_CREDIT_NOTE/INCREASE → PURCHASE_RETURN/DECREASE`，红字来源为 `PURCHASE_INVOICE_LINKED_RETURN_REVERSED`、物理来源为 `PURCHASE_RETURN_INVENTORY`，最终 GRNI 与总账差额均为零且库存只减少一次，头原子 POSTED、行 reversal 取库存返回；另覆盖数量大于零而暂估/库存金额均为零时物理 Skipped/voucher 空，以及全数退清后库存数量、金额余额与移动平均单价同时为零。
- E2E-P-07 超量开票路径一的收货侧：在存在待处理超量开票余额的采购订单上登记收货，断言库存契约结果与 `finance.overbilling_settlements` 出现反向匹配，且该部分不写任何 GRNI `INCREASE`、不再按订单单价挂暂估。
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

`tests/rls_matrix` 中新增本阶段的三十一张表，覆盖读取、写入、更新、删除、聚合、排序、报表投影与错误信息泄漏八类，另新增门户维度的两类：以门户账号跨供应商访问，以门户账号跨授权法人访问。该测试目标属发布门禁项。

#### 8.6 对账与不变量校验

本阶段在 `ep-app-procure` 实现六个 `ep_platform_recon::ReconCheck`，并按裁定 A-06 全部在 `apps/job-worker/src/wiring/` 目录下经 `ReconRegistry::register` 注册，由 ep-platform-recon 的执行器按法人逐轮遍历、在其提供的快照上分批执行，差额非零即生成对账差异事项并按规格第 10.2 章拦截关账。裁定 A-06 固定的四个注册方十五个校验项中，本阶段承担六个。六个 check 的 `code()` 取值即下表编号，`blocks_period_close()` 一律为真，`category()` 一律取 `ReconCategory::Invariant`，落库文本为 `INVARIANT`：本阶段的六条判据全部是金额与数量守恒；单目标引用由数据库真实外键保证，封闭多态引用由 owner 写入事务校验，二者都不另占对账项，见第 3.1 小节。

| 编号 | 判据 |
|---|---|
| R-PROC-01 | 采购订单行的 `received_quantity` 等于该行全部已过账收货行的数量合计 |
| R-PROC-02 | 收货行的 `returned_quantity` 等于关联该行的全部已过账退货行数量合计，且不超过该行 `quantity` |
| R-PROC-03 | 采购需求的 `ordered_quantity` 等于关联采购订单行的数量合计，且不超过 `required_quantity` |
| R-PROC-04 | `payable_reservations.reserved_amount = Σ(INVOICE_PAYMENT 未终态行.requested_amount-paid_amount)`；每行与头 `0<=paid<=requested`、头 paid 等于行合计；并与 finance 权威 `effective_open` 满足 `reserved_amount<=effective_open` |
| R-PROC-05 | 收货行三方计价段守恒：经 `InventoryPricingLookupPort::priced_segments_by_source_line(PURCHASE_RECEIPT,line_id)` 返回的全部 IN 段数量合计必须等于收货行数量；其中 `ESTIMATED_PO_PRICE` 段按数量/金额与该收货行 `GOODS_RECEIPT/INCREASE` GRNI 根逐项及合计相等，`OVERBILL_INVOICE_PRICE` 段按数量/金额与 finance 的锁定 `overbilling_settlements` 逐项及合计相等；未知分支、空段、重复键、任一侧多出或缺失均生成阻断关账的差异事项 |
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

1. 三十三个迁移文件（三十一个建表文件、一个 `append_only_registry` 登记回填文件与一个 portal 外键追补文件）在空库上由常规事务 Runner 按文件版本号全序执行成功，并按其 `-- rollback:` 段逆向回退成功；33 个文件均不含 `CREATE INDEX CONCURRENTLY`，新空表索引均由对应建表迁移用普通 `CREATE INDEX` 建成且系统目录可查。`V20261018090700__procure_create_goods_receipts.sql` 的 `ck_goods_receipts_posting_shape`，以及订单、收货、退货、notice 图所需候选键、长复合外键、延迟函数和所有约束触发器均可由系统目录精确查得；GRNI 根与长父自 FK 都是 RESTRICT、DEFERRABLE INITIALLY DEFERRED，空表首根单 INSERT 可提交，缺根/跨法人/跨收货行或跨根父均不可提交。第 8.2 节第 1、4 至 9 项的 direct SQL 非法图全部被数据库拒绝。回退后 `procure` 与 `portal` 两个 schema 无残留对象，`platform_core.append_only_registry` 中无本阶段残留登记行。
2. 三十一张表全部 `ENABLE` 且 `FORCE` 行级安全，策略名与基线模板一致，运行期账号不具备 `BYPASSRLS` 与 `SUPERUSER`。`--check` 模式的 `rls-enabled-and-forced` 自检项对这三十一张表通过。
3. `tests/rls_matrix` 的十类断言在本阶段三十一张表上全部通过，含门户跨供应商与跨授权法人两类。
4. 第 5 节列出的端点中，除第 5.7 小节标注随阶段 10 交付的三个对账端点，以及 `record-supplier-refusal` 随阶段 11 `CostReturnMarkPort` 真实实现同批启用外，均在本阶段可用；不得提前注册返回成功或固定失败的空路由。逐个已启用端点的封套、分页、排序白名单、过滤算子、幂等语义与错误码由契约测试断言。
5. 第 8.2 小节的二十二个集成场景中本阶段执行二十个并全部通过，其中第 4 至 9 项包含订单头行批次、收货-notice、采购退货及其库存/GRNI/voucher 图的 direct SQL 正反例；第 4、13 项还覆盖两类迁移撤销 owner audit 与 R0 分离、receipt target、逐态 before/after/row_version、依赖守卫及失败零部分写入；第 6 项覆盖 `goods_receipts` 五个状态、零/非零 voucher 及所有祖先/效果非法组合；第 12 项与第 20 项随阶段 10 的对应端口同批执行，两项在第 25 条列名。
6. 第 8.3 小节的十六个 E2E 用例全部通过，其中 E2E-T-01 至 E2E-T-04 对应规格第 19 章阶段 3 门户条目的四项闭环用例；E2E-T-03 在本阶段只执行上传与被退回两条路径，其被受理一路随阶段 10 的受理回写同批执行，该路径在第 25 条列名。
7. 第 8.1 小节的四组领域属性测试各运行不少于 1000 个用例且无反例。
8. 第 6.7 小节的六个并发场景中本阶段执行五个并全部通过，第 4 项随付款申请的 `INVOICE_PAYMENT` 分支在阶段 10 执行；`procure.goods_receipt.posted.v1` 的重复投递 3 次业务效果、外发事件与审计记录各只产生一次。
9. 第 8.6 小节的六个 `ReconCheck` 已在 `ep-app-procure` 实现并在 `apps/job-worker/src/wiring/` 目录下经 `ReconRegistry::register` 注册，注入任一差额后对账差异事项生成且关账请求被拒绝，差额清零后关账可通过。
10. 第 8.4 小节的四项数据保护控制测试通过，其中导出审批以「无导出入口」判定并已取得安全负责人的书面确认。
11. 第 8.8 小节的覆盖率门槛全部达标，`cargo llvm-cov --fail-under-lines` 在 CI 上通过。
12. 依赖方向自检脚本通过：`ep-domain-procure` 与 `ep-domain-portal` 不出现 sqlx、reqwest、tokio 的 IO 模块、`std::fs`、`std::net`、`SystemTime::now`、`rand` 六类符号；`ep-app-procure` 与 `ep-app-portal` 之间无相互依赖；除 `apps/*/src/wiring/` 目录外无 `use ep_adapter_db_pg::` 出现。
13. 文件规模纪律通过：本阶段新增文件无一超过 800 行，函数无一超过 50 行，嵌套深度无一超过 4 层。
14. `docs/event-catalog.md` 已登记本阶段 15 个事件类型，`docs/error-codes.md` 已登记本阶段完整引用的 31 个 PROCURE/PORTAL 码（27 个 `PROCURE.*`、1 个门户能力码、3 个供应商发票上传码）且与 `ep-foundation::error::codes` 常量表一致（平台段错误码由阶段 1 登记，本阶段不重复登记），`docs/data-dictionary.md` 已登记 31 张表，三处由 CI 校验一致。
15. 附录 A.1 清单内本阶段的八个度量端点在附录 A.3 基准数据集上给出 `EXPLAIN` 证据，无顺序扫描。
16. portal-gateway 以独立的服务虚拟账户 `NT SERVICE\ep-portal` 启动，进程内无任何事务数据库连接，该判定由 `/scripts/` 下的部署校验脚本以 `pg_stat_activity` 断言一次，不做每次启动的自检；反向代理上门户站点与员工站点使用独立站点、独立证书与独立访问策略，核对结论已写入部署记录。
17. 本阶段新增指标固定为 0；旧“三个指标”没有任何具名定义，已由 F-54 撤销，不得以未命名配额驱动实现。采购与门户填充的既有 HTTP、数据库、Outbox、死信与对账指标可在 ops-agent 的 127.0.0.1:9101 上读到，标签基数符合基线第 9.2 节纪律。
18. 第 7.2 小节的八个业务参数已通过配置发布通道发布一次，改值不需要重启进程与改表结构。
19. 本模块在规格第 6.2 章能力矩阵中取值为完整或简化的能力域，其四端界面已实现并通过 Playwright 与 tauri-driver 的桌面用例、XCUITest 与 Espresso 的移动用例；取值为 VIEW_ONLY 的能力域只实现只读视图；取值为 NOT_APPLICABLE 的不实现入口。供应商门户站点以浏览器承载，其用例由 Playwright 驱动。
20. 本阶段全部路由的能力域码与动作类别常量已在 `crates/contract/procure/src/capability.rs` 与 `crates/contract/portal/src/capability.rs` 声明，`xtask configdoc` 通过。
21. `ProcureReferenceCounter` 与 `ProcureTradeHistoryProvider` 已实现并在两个 wiring 目录注册到 `MasterReferenceCounterRegistry` 与 `TradeHistoryProviderRegistry`，覆盖未终态采购需求、采购订单、收货、采购退货、付款申请与采购订单行、收货行的历史成交。
22. 已在 `crates/contract/procure/src/port/subledger_balance.rs` 定义 `GrniSubledgerBalancePort`（`Send + Sync`，带 `#[async_trait::async_trait]`），方法为 `async fn balance(&self, snapshot: &dyn SnapshotCtx, legal_entity_id: Id<LegalEntity>, accounting_period_id: Id<AccountingPeriod>, accounting_period_seq: i32) -> Result<Money, AppError>`；实现类型 `GrniSubledgerBalanceQuery` 位于 `crates/application/procure/src/projection/subledger_balance.rs`，只按 `goods_receipt_line_costings` 的 INCREASE/DECREASE 追加效果与 period seq 算截至期间余额，不跨 schema、不按 UUID 比较期间。已在 `crates/contract/procure/src/port/grni_effect_writeback.rs` 定义并在 ep-app-procure 实现 `GrniEffectWritebackPort`，其两方法、锁序、部分回冲舍入、末次尾差、进项红字重开与根/父累计上限均按第 3.2.10 节；阶段 10 在采购发票与进项红字事务内注入并调用。两个 `impl` 均与类型同 crate，注入由阶段 10 在两个 wiring 目录写入（`ep_contract_finance::SubledgerBalanceProvider` 一名全卷作废）。
23. `PurchaseReturnLinkPort::link_drop_ship_return` 已实现并在两个 wiring 目录首次接线，阶段 6 在本阶段之前未注入任何替身，直运退货勾稽端到端通过。
24. 八个单据类型码 PR、PO、GR、RJ、PRT、PAYR、DN、SIU 已登记入 `docs/data-dictionary.md` 的单据类型码一节与 `ep-platform-sequence` 的常量表，`xtask configdoc --check-doc-type-codes` 通过。
25. 阶段 7 首次交付时，两个 wiring 目录不注入任何 `Noop`，也不提前伪接阶段 10 尚未实现的端口；阶段 10 同批交付并正式接线的类型名已经冻结为 `ep_contract_invoice::ReceiptInvoiceMatchQueryPort`、`ep_contract_invoice::PurchaseCreditNotePort`、`ep_contract_finance::PayableLedgerQuery`、`ep_contract_finance::SupplierStatementQuery` 与 `ep_contract_portal::SupplierInvoiceUploadWritebackPort`。它们承接采购退货已开票分支、发票付款占用、三个门户对账端点、集成场景 12/20 和发票上传 `UPLOADED → ACCEPTED`；不存在“落码前再裁定名称”的占位事项。
   其中 `ReceiptInvoiceMatchQueryPort` 的方法 roster 冻结为一个 proof 前纯 id 候选方法与三个 proof 后容量方法；Stage 10 的 API snapshot/trybuild 必须编译本节 MATERIAL/DROP_SHIP 两个 consumer fixture，并拒绝漏 proof、proof 前容量调用和旧两方法 ABI。
   本阶段自有 `PurchaseOrderInvoicingPort` 的 roster 冻结为两个 proof 前标识方法、一个 proof 后锁定状态读取方法与两个 proof 后 mutator，共五个方法；contract snapshot/trybuild 必须拒绝旧四方法 ABI、proof 前读取状态、候选 DTO 出现量额/状态/版本，以及 writeback effect 出现 `expected_line_row_version`。该方法数变化不新增 trait，也不改变本阶段公开 trait 总数。
26. `platform_core.append_only_registry` 中存在 `procure.goods_receipt_line_costings` 一行，其 `mode` 为 `APPEND_ONLY`、`mutable_columns` 为空数组，仅追加触发器已按该行挂接，`xtask sqlcheck` 执行 `db/checks/append_only_consistency.sql` 返回零行。
27. 严重与高危缺陷全部关闭，中危缺陷已登记并给出规避方案与责任人。
28. `ep-contract-costing` 已建立且只含本阶段前置的 `CostReturnMarkPort` 契约切片，签名与 `CostReturnMarkCommand` 七字段逐字固定；阶段 7 发布装配中没有实现、Noop 或 `record-supplier-refusal` 路由。阶段 11 的退出条件必须以真实实现同批启用该路由，并覆盖 PROCURE_MANAGER、FINANCE_MANAGER、重新认证、审批、只追加标记及追加更正七项端到端断言。
29. `ContractTerminationPurchaseRequisitionImpactRule` 已以 code=`CLM_TERM_PURCHASE_REQUISITION` 注册为第四个真实规则，`ImpactRegistry` 累计注册数恰为 4；三支来源、两类自动关闭状态、ORDERED 两个具名 decision code、同单 `decision_result_doc_id` 与统一三态结果均按第 4.6.1 节通过真实 PostgreSQL 测试。wiring 中无 ImpactRule 替身且不存在采购侧合同终止消费者；未完成或错误人工决策不能推进合同闭合。

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
| 第 7.7 章 | 三十一张表的行级隔离以 `app.legal_entity_id` 为唯一判据；不使用 `BYPASSRLS`；跨法人查询按法人逐个设置变量分别查询 |
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
| 第 21.17 章 | 门户暴露面的四项遏制：独立进程与账户、会话与限流取值收窄、受限自助注册默认关闭、portal-gateway 资源单位 app-portal 的静态限额取值；按裁定 F-08 第五节对己-1 第四节的重裁，第四项在本平台只剩内存硬上限一维——磁盘 IO 一维归零、CPU 一维待实测、突发上限的折算规则整条不成立，这是本次平台变更的第二处实质安全回退，须写入交付说明，见第 11 节风险三 |

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
| 4.9.1 至 4.9.7 | 五项能力白名单、八条访问与数据约束（原写「七条」，实测 PRD 第 4.9.2 节为 8 条，裁定 F-42 更正）、订单与交期确认、送货通知、发票上传、收付款对账查询、自身档案维护 |
| 4.10 | 五条权限与职责分离规则 |
| 4.11 | 四条异常处理与错误反馈规则 |
| 4.12 | 四条验收要点 |
| 2.4.4 | 门户提交的档案变更经 `ep_contract_mdm::SupplierSelfServiceCommand` 生成待审批变更申请，本阶段提供门户侧入口与回执 |
| 5.5.2 | 通用库存侧输入与校验的五项字段与四步校验顺序，在收货与退货两处调用 |
| 5.5.3 | 采购收货入库的库存侧输入与系统处理，本阶段提供来源单据与调用编排 |
| 5.5.6 | 采购退货出库的库存侧输入与系统处理，同上 |
| 5.6.4 | 出库方向的提交时结存充足性校验在采购退货处落点 |
| 5.6.5 | 库存不足触发采购建议的建议单本体由本阶段承载；策略维度固定为法人+仓库+物料，字段固定为 `reorder_point/target_stock`，每 60 分钟按 `available_qty <= reorder_point` 判定并以 `max(target_stock-available_qty,0)` 建议 |
| 6.8.2 | 财务在付款登记环节读取的六项付款申请信息，由 `ep-contract-procure` 的查询端口提供 |
| 6.8.6 | 付款申请的已付金额与状态回写，由 `PaymentRequestWritebackPort` 提供 |
| 6.10.4 | 门户对账查询的数据来源为财务的应付台账与核销关系，本阶段只做投影与裁剪 |
| 10.5.2 | 本阶段产生的提醒事项：审批待办到达、审批结果、死信与人工任务三类的采购与门户实例 |
| 11.2 | 门户并发计入合计 20 人上限 |

---

### 11. 风险与预留

#### 11.1 技术风险

风险一：收货与退货的同事务多腿写入使事务变长。收货过账在一个事务内串联采购单据写入、总账取价与凭证生成、库存两账写入三段，行数多时可能逼近 5 秒的业务事务预算与 3 秒的普通交易提交通过线。遏制手段是把单次收货明细行上限定为 200 行（`EP__PROCURE__RECEIPT__MAX_LINES`），超出转后台任务并由站内通知回执；并在阶段 4 的容量测试上以基准数据集实测。若实测不达标，收窄行数上限而不是拆事务，理由是拆事务会破坏第 17.3 章的两账一致与子账总账勾稽在写入时点即成立这一性质。

风险二：跨模块的同事务契约调用把三个模块的失败耦合在一起。库存或总账任一实现出现长事务或锁等待，采购侧的提交一并失败。遏制手段是 `InventoryPostingPort`、`StockOnHandQueryPort`、`InventoryPricingLookupPort`、`PostingPort` 与 `AccountingPeriodResolver` 五个端口的方法签名不含任何 IO 之外的等待语义、不做外部调用、不做长时计算，并在契约测试中断言其单次调用的语句数与耗时上界。`AvailabilityQueryPort` 仅供销售可用量与补货扫描链路，采购退货不依赖它。库存与总账的真实实现已分别在阶段 8 与阶段 9a 合入，本阶段的契约测试对真实实现直接执行一遍，不停留在桩上。

风险三：门户是首版唯一的公网暴露面。遏制手段有四项：独立进程与独立系统账户、会话与限流取值收窄、受限自助注册默认关闭，以及 portal-gateway 资源单位 app-portal 的静态限额取值。第四项按裁定 F-08 第五节对己-1 第四节的重裁改写：本平台的资源单位是具名 Job Object，portal-gateway 是八个自研二进制之一，其资源单位由服务宿主层在 `ServiceMain` 早期读取 `deploy/` 下的静态限额文件后创建或打开并自我指派，不由编排层落实。规格第 13.1 章公网门户一行的四类取值逐类判：内存硬上限一列保留，落 `JOB_OBJECT_LIMIT_JOB_MEMORY`，绝对字节按附录 D.2 的 BC-1 基线组合由该行内存百分数算定；原与之同值的内存保底一列按做不到二删除，本平台没有内存压力下优先不回收的软保底，不得以最小工作集冒充，且触限行为由内核终止进程改为分配失败返回错误；磁盘 IO 份额一列按做不到一删除，本平台不提供按权重的磁盘 IO 比例分配，不得把绝对预留或绝对带宽上限写成份额；CPU 份额一列暂降为硬件规格标定与认证实测的意图声明、不落运行期取值，其运行期承载按该裁定第十二节实测清单第 2 项（并入附录庚五）待实测。据此，门户的资源侧遏制在实测出结论前只剩内存硬上限一维：磁盘 IO 一维直接归零，CPU 一维待实测，突发上限一列的折算规则按补裁甲在本平台被乘数消失、整条不成立。被攻破或遭洪泛的门户进程在 CPU 与磁盘 IO 两维上不再受任何运行期约束——原文「只在竞争时约束份额、空闲时可借用」已是较弱的表述，本平台连这一层也没有。本阶段与交付材料一律不得把 CPU、磁盘 IO 与突发上限三者中的任何一项表述为已覆盖。这是本次平台变更的第二处实质安全回退（第一处是命名管道名字空间没有创建侧准入控制，见基线第 2 节所载的新增残余风险），须与规格第 21.17 章的残余风险并列写入交付说明，不得沉默。CPU 一维的重新生效谓词取机器可观测的事实：一旦 `deploy/` 下的静态限额文件出现公网门户一行的 CPU 取值行，本节该维自动由待实测转为有运行期承载并同批改写本节，不写成任何需要人工翻牌的动作。未被覆盖的路径有两条：一是应用层限流器生效之前的 TLS 握手 CPU 消耗，二是每个来源地址均不超过 `EP__PORTAL__RATE_LIMIT__REQUESTS_PER_MINUTE` 的分散洪泛；两条在首版都不消除，只按本节如实披露，且在 CPU 一维无运行期承载期间第一条不再有任何资源侧兜底。公网入口前置的 WAF 或 API 网关按规格第 17.5 章由客户提供并运维，不属于部署适配范围，也不是附录 D 的认证维度，平台不验收其规则集与防护效果，因此一律不计入平台侧的覆盖面；部署时须核对其是否已配置并写入部署记录，未配置时按该章持续告警并记录暴露窗口。过载处置的可归因失败事件仍由 portal-gateway 的限流与超时承担；原「cgroup 侧的节流只延迟不失败、不产生该类失败事件」一句在本平台不成立，据实改写：磁盘 IO 一维已无运行期承载、CPU 一维待实测，资源单位侧首版不产生任何节流；唯一有承载的内存硬上限触限时的表现是分配失败返回错误而不是延迟，其失败形态与限流超时不同，该形态的归因口径在本平台尚未取值，本节只如实登记，不得沿用「只延迟不失败」这一已失去承载物的表述。残余风险按规格第 21.17 章保留，门户与核心之间只有进程与系统账户边界而不是机器边界这一点不因本阶段的措施改变。

> **风险三的现行资源修正。** 风险三内“CPU 待实测、静态文件出现值后自动生效”只保留为历史成因，不得实现。首版 CPU 比例与突发上限、按权重磁盘 IO 份额均固定不启用；静态限额文件不允许这些字段，内存硬上限是唯一运行期配额列。未来启用必须另立产品版本、正式裁定与 Windows 实测发布门，不能由文件自动翻牌。portal-gateway 的 Job Object 名按基线唯一算法取 `Global\EP_<deployment UUID去连字符大写>_APP_PORTAL`。

风险四：门户字段白名单一旦遗漏即构成数据外发。F-51 已批准第 4.7 小节白名单为首版唯一基线；遏制手段是第 8.2 小节第 16 项的全字段快照测试，任何新增字段都会导致快照失败并必须走正式暴露面变更评审。本项不再是发布阻塞或实现方选择项。

风险五：`payable_reservations` 是本阶段引入的第二处金额状态，与财务侧的应付 `effective_open` 存在漂移可能。遏制不是只靠周期对账：所有增占用/AP 降低/付款释放/冲正恢复走同一 F-50 key 与 proof，付款先释放本次 delta 再终检，跨 schema deferred terminal trigger 在提交点兜底，R-PROC-04 再作周期检出；全程不做异步同步。

风险六：本阶段的八个业务参数取的是冻结默认值，其中超收容差与转审批阈值（U-F-04）直接影响收货的拦截行为，若客户在实施期改值，历史已过账收货不重算。该性质须在交付说明中写明。

#### 11.2 F-51 确认的冻结决定

下列条目均已由 F-51 确认为首版规范值；“变更代价”只描述未来正式版本变更，不表示当前仍待选择。

假设 A1：采购需求是单行单据。理由是 PRD 第 4.3.2 小节的字段表只有单个物料与单个数量，没有明细行的结构。切换代价为新增一张 `procure.purchase_requisition_lines` 表与一次数据回填迁移，属中等代价，因此在整合期确认。

假设 A2：收货与采购退货的采购单据、库存两账与总账凭证在同一个数据库事务内同步写入，不经 Outbox 异步过账。理由有三条：规格第 17.3 章要求存货金额账合计等于总账存货科目余额；PRD 第 4.5.3 小节把凭证号列为收货登记的输出；PRD 第 4.5.6 小节要求库存或财务侧写入不一致时界面返回明确失败。三条同时成立只有同事务一种实现。由此产生的推论是：规格第 10.2 章关账受理前提下不存在收货与退货的异步过账路径，受理前提二统计的是这两个事件的未投递条目而不是未生成的凭证。本阶段仍在 Outbox 信封上携带 `posting_date` 与 `accounting_period_id`，两个事件在 `ledger.posting_trigger_event_types` 中的登记行按裁定 A-21 由阶段 9a 的种子迁移写入；`PostingTriggerRegistry::assert_registered` 按总览第 1.5 节第三条整项撤销，本阶段不做启动自检、不做 `--check` 静态断言，也不向关账受理追加前置校验，该统计的可枚举性由 `xtask configdoc` 在 CI 中对第 14 号种子迁移与 `docs/event-catalog.md` 的逐字比对以及阶段 3b 的 `event-catalog-consistent` 保证。总账与库存的契约端口按 A-01 接受 `&mut dyn Tx`，已由阶段 1 提供。

决定 A3：采购退货在「采购发票已登记」分支下调用 `ep_contract_invoice::PurchaseCreditNotePort::register_credit_note`，进项红字发票由 invoice 模块登记。命令与返回值只采用 F-50 第 6.5 节、阶段 10 第 4.11 节的最终 exact Rust 契约：请求含供应商、原采购发票、必填的本退货 `linked_purchase_return_id`、法定号码 `identifier`、记账日期、原票 `expected_original_row_version`、超量结清标记，以及非空 `InvoiceReversalLineInput[]`；行内必须是进项来源、带两类 effect kind、数量、税率和净/税/价税合计，不接收 `source_effect_seq` 或 GRNI 金额。采购调用方与端口在同一事务复用记忆化期间解析，返回三项期间必须与调用方先前结果相等。返回的 `PurchaseCreditNoteView` 含非空冲销 id/凭证 id、三项金额、链接退货 id、逐项 `grni_reopened_effects` 与汇总；采购侧同事务逐条追加等数量等额退货冲减，确切顺序与凭证分工按第 4.4 小节。链接实物退货的红字不得重复写库存/成本腿。该端口由阶段 10 交付，本阶段不注入任何替身，发票已登记分支连同红字登记按第 4.4 小节整条推迟到阶段 10 同批交付。直运采购退货上供应商拒绝接受退回时，唯一入口为 `record-supplier-refusal`：PROCURE_MANAGER 提交证据与原因，FINANCE_MANAGER 重新认证并审批，批准事务调用 `CostReturnMarkPort`；标记只追加，不得直接撤销或删除，事实变化时由 costing 追加引用原记录的成本冲回或更正条目，原标记永久保留为证据。`ep-contract-costing` 与 trait 形态由本阶段先建，阶段 11 交付真实实现并同批启用路由与调用点；两阶段均不得注入 Noop。U-C-09 据此关闭。

假设 A4（对应 U-C-08）：供应商的资质证照、价格资料、交期资料与风险记录唯一存储在 mdm 的供应商档案及其版本与子表上，`procure` 只存准入结论与质量记录两类；风险记录按裁定 C-10 一律经 `ep_contract_mdm::SupplierRiskRecordPort::append` 与 `::list` 读写，`procure.supplier_risk_records` 已撤销，见第 3.2.2 小节。理由是 PRD 第 2.4.1 小节与第 2.4.3 小节已把这四类定义为供应商档案的字段与子表，而 PRD 第 4.8.1 小节只是从采购视角复述。切换代价为把四张表从 mdm 迁到 procure 并改门户提交的写入目标，属中等代价。准入结论存于 procure 的理由是它只被采购侧读取且带自己的状态机。

假设 A5：门户账号的身份主体、口令、MFA、会话与设备登记归 `platform_identity`，`portal.supplier_portal_users` 只存账号与供应商与法人的授权绑定及五项能力白名单。理由是规格第 12.1 章把外部供应商用户列为身份能力，重复建目录会产生第二套认证面。

假设 A6：集合级 actions 路径（`POST /api/v1/<module>/<resource-plural>/actions/<verb>`，不带 `{id}`）是合法形态。理由是采购需求禁止手工新建但允许由销售订单行发起，该动作不属于任何已有需求。该形态回写基线第 5.1 节。

假设 A7：门户会话有效期 2 小时、空闲 15 分钟、会话校验缓存 30 秒、单附件 50 MB、列表页大小上限 50。五项均低于内部取值，理由是公网暴露面。回写基线第 11.6 节。

假设 A8：收货单状态机新增 `PENDING_APPROVAL` 一态。理由是 PRD 第 4.5.4 小节要求超收转审批而第 4.5.5 小节的状态机漏列该态。

假设 A9（对应 U-F-01）：物料类采购订单必须关联采购需求，直接费用类允许不关联需求直接创建。理由是规格第 8 章限定采购需求只有四个来源，若直接费用类也强制关联则服务类采购无入口。该取值由第 7.2 小节的两个业务参数承载，改值不需改代码。

决定 A10（对应 U-F-02）：补货策略固定挂在法人、仓库、物料组合上，字段为 `reorder_point` 与 `target_stock`，且 `target_stock >= reorder_point >= 0`；首版不另设安全库存字段。唯一物理存储为 `inventory.replenishment_policies`，`ReplenishmentPolicyReadPort` 与持久化 owner 为 inventory；阶段 6/sales 以该读端口和销售 A2 同源可用量组合实现 `SalesAwareReplenishmentPolicyQuery`；采购侧只注入消费 `ReplenishmentPolicyQuery`，扫描 `limit` 为 `1..=500`。每 60 分钟按 `available_qty <= reorder_point` 触发，建议量为 `max(target_stock-available_qty,0)`；阈值为空跳过。扫描配置默认关闭，开启后同一组合只保留一张未结自动需求。以上均是当前唯一实现值，不设开启前产品决策。

决定 A11（对应 U-F-04、U-F-05、U-F-12、U-F-13、U-F-14）：五项当前值逐字取第 7.2 小节：超收容差 0、超容差转审批、改期轮次不限、采购退货需审批、门户不展示账龄、资质整体过期阻断下单。实现方直接采用；未来正式变更代价为一次配置发布，历史单据不重算。

决定 A12（对应 U-F-10）：F-51 已批准第 4.7 小节门户返回字段白名单为首版基线；发票上传使用头+行，头无单一税率，行含税率、净额、税额与价税合计。当前开发和发布不再等待额外批准；未来扩列必须走正式暴露面变更评审并更新全字段快照。

决定 A13（对应 U-F-11）：首版只允许邀请开通且 `EP__PORTAL__SELF_REGISTRATION__ENABLED=false`。受限自助注册代码可保留；只有许可证、签名配置与安全审批三者同时满足才允许未来正式启用，待审核账号仍只能访问自身注册状态。

决定 A14（对应 U-F-03）：首版不实现采购需求的合并与拆分；一张采购订单可由订单行 `purchase_requisition_id` 关联多条同法人同供应商需求，但不产生合并后的新需求单。该值已冻结；未来若增加显式合并，需新增关系表与回填迁移，代价中等。

决定 A15（对应 U-A-07、U-A-08、U-A-11）：退货原因、风险类型、付款条件三个字典与本阶段四条审批链的出厂配置由平台配置承载，本阶段登记字典键与审批链标识。采购订单与采购退货的出厂链各固定一个 `PROCURE_MANAGER` 必经角色节点；链缺失、零节点、节点展开为空或申请人自审一律按第 4.2.4 小节 fail-closed，绝不自动进入下一态。其余两条审批链也不得把空链解释为自动通过；未来改变节点只能经签名配置发布。

决定 A16（对应 U-F-06、U-F-07、U-F-08、U-F-09）：四项均已冻结，实现方无二次选择。U-F-06 固定直接费用类的合同、销售订单、项目三个归集字段至少一项非空，由 `ck_purchase_requisitions_type_fields` 与 `ck_purchase_order_lines_type_fields` 两条 CHECK 表达；未来放宽需同步把三项全空归入未分摊差异。U-F-07 固定在首次收货登记或首次采购发票登记前允许改采购类型，其后由 `is_type_locked` 锁定；未来提前到订单下达只改一处守卫。U-F-08 固定采用第 4.2.6 小节五态与三条守卫；未来变化需改 `admission_status` CHECK 与对应守卫。U-F-09 固定按第 3.2.2 小节字段，由采购退货过账经 Outbox 消费者自动生成，拒收与手工来源经第 5.6 小节补录，风险类型字典按决定 A15 归平台配置；未来变更需同步消费者与字段迁移。

#### 11.3 为后续阶段预留的扩展点

1. `procure.purchase_order_lines` 上已预留 `invoiced_quantity` 列与 `PurchaseOrderInvoicingPort` 端口，供发票阶段回写累计已开票数量与把直接费用类订单推进到已完成。
2. `procure.goods_receipt_line_costings` 的来源与方向分别以 `source_kind`、`direction` 两个文本 CHECK 表达；新增真实 GRNI 业务来源时扩展 `source_kind` 并同批补方向、父子约束与验收，不以超量开票结清路径伪装成 GRNI 效果。
3. `procure.purchase_returns` 不保存头级开票布尔值；过账分段只取同一事务锁后 `ReceiptInvoiceMatchQueryPort` 的逐收货行结果，历史只按链接进项红字与 GRNI 效果逐行追溯。查询侧可导出非持久化摘要 `NONE|PARTIAL|ALL`。`purchase_return_lines.reversal_unit_price` 仍由库存契约回填；采购侧任何情况下都不自行取价或自行推断开票状态。
4. `portal.supplier_portal_users.capabilities` 为文本数组加 CHECK，后续版本恢复客户门户与经销商门户时，能力码集合可扩展而不改表结构；但门户能力是封闭白名单，扩展必须先在规格第 5.5 章登记。
5. `procure.payable_reservations` 的行结构可直接承载后续的付款计划占用，只需新增一个占用类型列。
6. 门户投影的字段白名单以常量表达并有全字段快照测试兜底，后续新增门户能力时该测试是唯一必须更新的地方，构成可审查的暴露面变更清单。
7. 本阶段不建任何物化视图，门户对账查询与内部台账同源直查。若阶段 4 的容量测试显示该查询击穿通过线，扩展点是在 `reporting` 侧建立只读投影，而不是在 `portal` 侧建第二套余额口径。
