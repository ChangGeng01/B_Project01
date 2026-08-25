> **F-57 状态：`HISTORICAL_DO_NOT_EXECUTE`。** 本文只保留历史任务正文，现行工作由 F-57 **Task 19** 承接。当前权威集合为 [F-57 总体设计](../../specs/2026-08-23-f57-governed-automation-fabric-design.md)、[F-57 需求追踪矩阵](../../reviews/2026-08-23-f57-requirements-traceability.md)、[F-57 权威替代登记](../../reviews/2026-08-23-f57-authority-supersession-register.md)与 [F-57 实施计划](../2026-08-23-f57-governed-automation-fabric-implementation.md)；F-57 是设计/计划权威，不是产品已实现声明，本地模型实现延期。

## 阶段 6：合同与销售（CLM、销售与 OMS、CPQ 价格权限、客户信用额度校验）

> **F-50 范围修订。** 信用敞口与所有应收消费者只读 `effective_open`；销售退货/合同终止先登记来源业务动作，需要改票时再调用发票冲销；历史成交返回两个资格并在选价前重验。正文中相反的 `open_amount`、先红后退或旧 provider 句仅作历史基线。

本阶段承载规格第 5.2 章 CLM、销售与 OMS、客户信用额度校验、CPQ 价格权限四个条目的原生能力，以及规格第 8 章黄金业务闭环第 1 步、第 2 步、第 3 步、第 8 步与第 11 步销售侧的单据主体，对应 PRD 第 3 节全节，其中第 8 步的交付确认在 PRD 第 3 节与第 5 节均无承载小节，属 PRD 附录乙 U-C-01，见第 10.2 节与第 11.3 小节。其中第 8 步的交付确认单按裁定 A-09 归本阶段建表、建用例、发事件，是该步的唯一落点；库存腿由阶段 8 提供端口，收入与成本腿由阶段 9a 提供端口，过渡科目腿由阶段 10 提供端口，该腿与交付确认的过账路径按第 11.5 小节与阶段 10 同批接线，本阶段不注入任何空实现。本阶段属规格第 19 章阶段 3 的建设内容，其时延与容量通过线在阶段 4 统一判定。

全文取值一律遵循共享技术基线。基线已定死的事项本节直接引用，不重新决定；基线未覆盖而本阶段必须取值的，在第 11.3 小节集中列出并标注为本阶段新增决定；与基线有出入的，在第 11.2 小节单列偏离项。

本阶段的工作次序按 T0 贯通线重排，阶段范围归属不变。阶段顺序固定为 1 → 2 → 3a → 4 → 3b-1 → T0 → 5 → 9a → 8 → 6 → 7 → 10 → 11 → 9b → 14，T0 是插在阶段 3b-1 与阶段 5 之间的一条最薄贯通线，不新增任何范围，判据是一条合同从建单走到管理层看到一个数。本阶段在 T0 中贡献的最小切片只有两项，一份单审批节点的合同与一张由该合同生效派生出的销售订单。落到确切标识符是 `clm.contract_types`、`clm.contracts`、`clm.contract_lines`、`clm.contract_approvals`、`sales.credit_policies`、`sales.sales_orders`、`sales.sales_order_lines` 七张表，`create_contract`、`submit_for_approval`、`make_effective` 三个 clm 用例与 `ep_contract_sales::SalesOrderDerivationPort` 的派生实现，端点 `POST /api/v1/clm/contracts`、`PUT /api/v1/clm/contracts/{id}/lines`、`POST /api/v1/clm/contracts/{id}/actions/submit-for-approval`、`POST /api/v1/clm/contracts/{id}/actions/make-effective` 与 `GET /api/v1/sales/sales-orders/{id}`，事件 `clm.contract.effective.v1`。T0 内只启用 `clm.contract_approvals` 的 `chain_kind = EFFECTIVE` 一条链，该节点固定为 `approver_kind=ROLE, role_code=MANAGEMENT_APPROVER`，即规格第 8 章第 2 步要求的管理层必经节点；`sales.credit_policies` 只建一行且 `null_limit_behavior` 取 `SKIP_CHECK`，信用三桶不进 T0 判据；合同生效的重新认证按规格第 12.1 章在 T0 内即成立，不推迟；T0 内合同行的默认税率经 `ep_contract_invoice::TaxRateOptionQuery::default_rate` 取得，`invoice.tax_rate_options` 的建表迁移与种子迁移两条及该查询的 `default_rate` 与 `list` 两个方法属阶段 10 的 T0 切片第五项，与本阶段的两项切片在 T0 期间一并交付。T0 用 `ep-datagen` 最小样本，不要求 scale 数据集、不要求分支覆盖、只要求桌面端，其判据由 T0 自身判定，不重复计入第 9 节退出条件。

T0 通过后本阶段其余部分一律在这条已贯通的骨架上加厚，分三批施工。第一批是合同侧加厚，含模板与条款库、四条审批链与折扣审批、电子签章与实体印章、版本与修订、续签、合并、提前终止、五项校验与价格权限、三类到期提醒触发源。第二批是订单侧加厚，含分批交付行的拆分与合并、订单变更与版本、订阅与租赁、销售退货与换货、在途桶与 `sales.v_credit_exposure_in_transit`、四个受治理数据集视图、四端界面；同时按 F-51 U-G-01 交付真实 `ConfirmedOpenSalesDemandQuery`、销售感知的 `AvailabilityQueryPort` 组合实现并注册库存 A2 路由，再按 U-F-02 交付复用同一可用量实现的 `SalesAwareReplenishmentPolicyQuery`，这些契约、实现与装配必须同批启用，不存在只返回结存、保留量为零或采购侧另算可用量的过渡实现。第三批是交付与反向过账段，按第 11.5 小节与阶段 10 的 finance 端口同批施工，含 `sales.delivery_confirmations` 与 `sales.delivery_confirmation_lines` 的过账路径、信用三桶中两桶的接线、销售退货的红冲前置判定，以及非直运 INVENTORY 退货的同步入库、凭证、未开票应收与销售回写闭环。三批之外本阶段不再有其他工作次序上的约束，M7 判定的是全分支闭环而不是首次贯通。

---

### 1. 交付物清单

本阶段结束时下列可运行物存在，且可由 `cargo test --workspace` 与 `apps/core-server --check` 验证。

1. 六个新增库 crate 编译通过并被 apps 装配：`ep-contract-clm`、`ep-domain-clm`、`ep-app-clm`、`ep-contract-sales`、`ep-domain-sales`、`ep-app-sales`，以及 `ep-contract-cpq`、`ep-domain-cpq`、`ep-app-cpq` 中与价格权限校验相关的部分。
2. 一个新增适配 crate `ep-adapter-esign` 编译通过，目录为 `crates/adapter/esign/`，并在 integration-gateway 中装配为唯一的对外出网出口；其两套契约测试文件 `crates/adapter/esign/tests/contract_sandbox.rs` 与 `crates/adapter/esign/tests/contract_stub.rs` 存在且共用同一组断言函数。
3. `db/migrations/cpq/`、`db/migrations/clm/`、`db/migrations/sales/` 三个迁移目录下的全部迁移可在空库上离线执行成功，并可按各文件头 `-- rollback:` 段落回退到本阶段之前的版本；其中含按裁定 A-09 新建的 `sales.delivery_confirmations` 与 `sales.delivery_confirmation_lines` 两张表，按裁定 A-18 新建的 `clm.v_contracts_dataset`、`clm.v_contract_delivery_milestones`、`sales.v_sales_orders_dataset`、`sales.v_order_delivery_batches` 四个受治理数据集视图，以及在销售目标建成后执行的一个 `clm` 跨 schema 外键追补迁移。
4. core-server 暴露第 5 节列出的全部 HTTP 端点，`/api/v1/clm/*`、`/api/v1/sales/*`、`/api/v1/cpq/price-authorities`，四端可调用；并把阶段 8 只冻结契约、尚未注册的 `GET /api/v1/inventory/available-quantities`（A2）与本阶段的真实销售需求提供者、组合实现同批注册。其中 `POST /api/v1/sales/delivery-confirmations/{id}/actions/confirm-delivery` 一条按第 11.5 小节随第三批与阶段 10 同批注册，在此之前不注册，也不返回占位结果。
5. job-worker 中运行两类本阶段消费者，名字固定为 `clm.derivation`、`clm.milestone_confirm`；前者执行合同派生，后者消费 `sales.delivery.confirmed.v1` 推进合同交付节点，两者的死信可在运维中心枚举。另向阶段 3 的 `ImpactRegistry` 注册 `CLM_TERM_SALES_ORDER_LINE`、`CLM_TERM_MILESTONE`、`CLM_TERM_DELIVERY_CONFIRMATION` 三个真实规则；合同终止事件仍只由平台 `platform.impact_assess` 消费，不新增本阶段终止消费者。分批交付行、订单行和订单头的交付回写按 U-G-01 必须在 `confirm_delivery` 原事务内完成，不再设 `sales.delivery_writeback` 异步消费者或第二条业务回写路径；非直运 INVENTORY 销售退货的入库、库存金额回冲、凭证、未开票应收与销售侧回写也必须在 `register_sales_return` 原事务内完成，`sales.sales_return.registered.v1` 只是完成后的派生通知，禁止任何消费者据此补写库存或财务事实。
6. integration-gateway 中运行电子签章出口，含超时、退避、熔断、服务商响应验签与清洗回执；job-worker 只经 DACL 命名管道提交签署、触发状态查询并接收反向分块文件，实际对外请求仍只由 gateway 执行。gateway 不持数据库、文件库或 KMS 能力且不消费 Outbox；签章状态、附件对象、合同关联与审计证据全部由 worker 写入平台权威存储。
7. `docs/event-catalog.md` 中登记第 6.3 小节冻结的本阶段全量领域事件，其中含 F-10 两个合同终止事件、`sales.delivery.confirmed.v1` 与销售退货的登记、关闭、取消、驳回四个事件；本阶段第 5 节 API 契约表中出现的全部错误码已登记在 `docs/error-codes.md` 并与 `ep-foundation::error::codes` 一致，由 CI 校验。
8. `ep-testkit` 中新增 `ContractBuilder`、`SalesOrderBuilder`、`DeliveryScheduleBuilder`、`CreditFixture` 四个构造器；`ep-datagen` 在默认 scale 下生成合同与销售订单行各 10 万条并满足本阶段的全部不变量。
9. 一个可重复执行的端到端用例集 `apps/core-server/tests/e2e_stage6/`，覆盖第 8 节列出的 15 个 E2E 场景，其中两项专测 F-10 阶段 6 的正向与反向终止路径。
10. 四端界面：`clients/desktop/src/modules/clm/`、`clients/desktop/src/modules/sales/`、`clients/mobile/src/modules/clm/`、`clients/mobile/src/modules/sales/` 四个目录存在并可构建，按规格第 6.2 章能力矩阵的取值实现。

---

### 2. crate 与进程归属

#### 2.1 新增与改动的 crate

| crate | 层 | 新增或改动 | 主要内容 |
|---|---|---|---|
| ep-contract-clm | 契约 | 新增 | 合同命令与查询 DTO、合同事件类型、供其他模块调用的 `ContractQueryPort`、`ContractMilestonePort`、`ContractDerivationCallbackPort`、`ContractDerivationPlanQuery`、`ContractPaymentScheduleQuery`；`src/capability.rs` 中为每个用例声明 `<USECASE_SCREAMING>_DOMAIN` 与 `<USECASE_SCREAMING>_ACTION` 一对常量 |
| ep-domain-clm | 领域 | 新增 | 合同聚合、合同版本、关键条款、交付节点、收付款期次、签署编排、派生批次；合同状态机与守卫；`ContractRepository`、`SignatureGateway`、`TemplateRenderer` 三个端口 |
| ep-app-clm | 应用 | 新增 | 20 个用例、合同侧授权入口、事务边界、派生编排、审计与 Outbox 写入；F-10 的 `ContractTerminationMilestoneImpactRule` 与 `ContractTerminationCompletionPort`（实现 `ImpactSourceCompletionPort`）；合同变更与履约投影统一消费 `ReceiptPlanBillingQuery::billing_by_period`；`ClmProductUsageProbe` 与 `ClmReferenceCounter` 两个探针实现 |
| ep-contract-sales | 契约 | 新增 | 销售订单、交付确认与退货的命令查询 DTO、事件类型、`SalesOrderDerivationPort`、`CreditExposureQueryPort`、`SalesOrderQueryPort`、`SalesOrderLineDeliveryQuery`、`ConfirmedOpenSalesDemandQuery`、`SalesReturnCommandPort`、`SalesExchangeLinkCommandPort`；`src/capability.rs` 中为每个用例声明 `<USECASE_SCREAMING>_DOMAIN` 与 `<USECASE_SCREAMING>_ACTION` 一对常量 |
| ep-contract-inventory | 契约 | 改动 | 在阶段 8 已交付的 `ReplenishmentPolicyReadPort`、策略 DTO 与 `AvailabilityQueryPort` DTO 上，同批追加 `AvailabilityQueryPort` 与 `ReplenishmentPolicyQuery` 两个 trait；后者只向阶段 7 暴露带同源 `available_qty` 的启用策略扫描视图 |
| ep-domain-sales | 领域 | 新增 | 销售订单聚合、订单行、分批交付行、订单变更版本、交付确认聚合、销售退货聚合、换货关联；四套状态机；信用占用在途桶的纯计算 |
| ep-app-sales | 应用 | 新增 | 20 个用例（含 `create_delivery_confirmation`、`confirm_delivery` 与 `register_sales_return`）、信用校验编排、交付确认三腿编排与同步订单回写、销售退货的库存/凭证/未开票应收三腿编排及同步回写、订单变更审批回写；F-10 的 `ContractTerminationSalesOrderLineImpactRule` 与 `ContractTerminationDeliveryConfirmationImpactRule`；`ConfirmedOpenSalesDemandQueryImpl`、`SalesAwareAvailabilityQuery` 与 `SalesAwareReplenishmentPolicyQuery` 三个销售感知实现；`SalesProductUsageProbe`、`SalesReferenceCounter`、`SalesTradeHistoryProviderImpl` 三个探针实现 |
| apps/job-worker | 装配 | 改动 | 在 `src/wiring/impact.rs` 向既有 `ImpactRegistry` 注册上述三个真实规则，并以唯一键 `(ModuleCode::Clm,"clm.contract.terminated.v1")` 注册真实 `ContractTerminationCompletionPort`；不得注册本阶段合同终止消费者，不得注入规则或 completion port 替身。另作为电子签章 Outbox 的唯一消费者和数据库写入方，经 `ep-integ` 双工管道收发清洗回执与 `esign_file.*` 分块文件，并在完整校验后建立附件和合同关联 |
| ep-contract-cpq | 契约 | 改动 | 追加 `PriceAuthorityPort` 与价格权限判定 DTO；价目表查询 trait 由主数据阶段定义，本阶段只消费；在阶段 5 已建的 `src/capability.rs` 中只追加价格权限档案路由的一对常量，不重定义能力域码 |
| ep-domain-cpq | 领域 | 改动 | 追加价格权限值对象与判定规则、行金额与净单价的计算规则 |
| ep-app-cpq | 应用 | 改动 | 追加价格权限档案的维护用例与判定用例 |
| ep-adapter-esign | 适配 | 新增 | 电子签章外部出口的 HTTP 客户端、请求签名、响应验签、稳定码清洗、熔断与退避，以及不落盘的已签文件有界分块流；不得依赖数据库、文件库或 KMS 客户端 |
| ep-testkit | 测试 | 改动 | 追加本阶段四个构造器与信用夹具 |
| ep-datagen | 测试 | 改动 | 追加合同、订单、分批交付行、退货单的生成器 |

除按 A-23 在 `clients/desktop/src/modules/` 与 `clients/mobile/src/modules/` 下各新增 `clm` 与 `sales` 两个模块目录外，不新增任何 crate 之外的目录结构，crate 内目录严格按基线第 10.1 节。`ep-domain-clm` 与 `ep-domain-sales` 中不得出现 sqlx、reqwest、`std::fs`、`std::net`、`SystemTime::now`、`rand` 符号，由基线第 8.4 节的静态检查强制。

#### 2.2 依赖方向

- `ep-app-clm` 依赖 `ep-foundation`、`ep-platform-*`（含 `ep-platform-impact`）、`ep-domain-clm`、`ep-contract-clm`，以及 `ep-contract-sales`、`ep-contract-procure`、`ep-contract-finance`、`ep-contract-mdm`、`ep-contract-cpq`、`ep-contract-inventory`、`ep-contract-invoice` 七个外部模块契约。`ep-contract-project` 按 C-19 移除，项目任务不再由本模块同步派生；`ep-contract-invoice` 同时提供合同行默认税率所需 `TaxRateOptionQuery` 与期次净开票金额唯一读取面 `ReceiptPlanBillingQuery`，clm 不直接读取 invoice schema。
- `ep-app-sales` 依赖 `ep-foundation`、`ep-platform-*`（含 `ep-platform-impact`）、`ep-domain-sales`、`ep-contract-sales`，以及 `ep-contract-clm`、`ep-contract-mdm`、`ep-contract-cpq`、`ep-contract-inventory`、`ep-contract-ledger`、`ep-contract-finance`、`ep-contract-invoice`、`ep-contract-procure`、`ep-contract-costing` 九个外部模块契约。其中 `ep-contract-inventory` 的 `StockOnHandQueryPort` 供 `SalesAwareAvailabilityQuery` 组合结存与销售需求，`ReplenishmentPolicyReadPort` 供 `SalesAwareReplenishmentPolicyQuery` 组合启用策略与同一可用量；`AvailabilityQueryPort`、`ReplenishmentPolicyQuery` 均由本阶段同批追加并由这两个类型真实实现。`InventoryPostingPort` 供交付确认出库和非直运 INVENTORY 销售退货入库，`InventoryPricingLookupPort` 只供交付确认详情按稳定分段组装；销售退货改由 `DeliveryCaptureReturnBasisQuery` 取得原交付 current live basis，不调用库存计价查询。ledger 契约供两类业务的凭证腿调用，`ep-contract-procure` 供直运退货的勾稽调用。`ep-app-sales` 只依赖 inventory/costing 契约，不依赖其 application crate；阶段 8 与阶段 11 也不反向依赖 `ep-app-sales`，阶段 7 只消费 `ReplenishmentPolicyQuery`，不得依赖 sales 或 inventory 的 application crate。
- `ep-app-clm` 与 `ep-app-sales` 之间不存在直接依赖。合同派生销售订单一律经 `ep-contract-sales::SalesOrderDerivationPort`，其实现是 `ep-app-sales` 的用例，在 apps 的 `wiring/` 目录中注入。
- `ep-adapter-esign` 只依赖 `ep-foundation` 与 `ep-domain-clm::port::SignatureGateway`，不依赖任何 application、数据库适配器、平台文件库或 KMS；它的全部业务结果只返回给 IPC 服务层，不直接固化状态或证据。

#### 2.3 进程归属

| 能力 | 进程 | 说明 |
|---|---|---|
| 合同与订单的全部命令与查询 API | core-server | 含四端与合同侧受控查询 |
| 库存可用量最终组合与 A2 路由 | core-server | 本阶段把阶段 8 的 `StockOnHandQueryPort` 与真实 `ConfirmedOpenSalesDemandQuery` 注入 `SalesAwareAvailabilityQuery`，同批实现 `AvailabilityQueryPort` 并注册 A2；未齐备时路由不存在，不注入空 provider |
| 销售感知补货策略组合 | job-worker | 本阶段把阶段 8 的 `ReplenishmentPolicyReadPort` 与上述同一个 `SalesAwareAvailabilityQuery` 注入 `SalesAwareReplenishmentPolicyQuery`，并以 `ReplenishmentPolicyQuery` 注册到装配根；阶段 7 的自动采购需求扫描只消费该 trait，不读取 sales 表、不自行重算可用量 |
| 交付确认单的登记与确认过账 | core-server | 确认动作在单个事务内依次调用库存腿、过渡科目腿与凭证腿三个契约端口，三腿一次全真接线，按第 11.5 小节随第三批与阶段 10 同批交付 |
| 合同附件正文的读写 | core-server | 交易路径上的附件正文按基线第 2 节归 core-server |
| 合同生效派生编排与执行 | job-worker | 消费 `clm.contract.effective.v1`，按派生项逐项执行 |
| 合同履约进度推进 | job-worker | `clm.milestone_confirm` 消费 `sales.delivery.confirmed.v1`，更新 `clm.contract_milestones` 的交付节点；销售侧分批交付行、订单行与订单状态已在确认原事务内更新，job-worker 不再二次回写 sales schema |
| 合同履约页净已开金额 | core-server | 调用阶段 10 真实 `ReceiptPlanBillingQuery::billing_by_period`，按 `invoice.invoice_receipt_plan_links` 当前正向减反向分摊显示；clm 不存副本、不注册开票回写消费者 |
| 合同到期提醒的定时触发 | job-worker | 使用 ep-platform-flow 的定时器与 ep-platform-notify 的站内通知，本阶段只提供触发源投影 |
| F-10 合同终止规则执行 | job-worker | 平台唯一消费者 `platform.impact_assess` 通过 `ImpactRegistry` 调用本阶段三个真实规则；全部项闭合时经唯一 `ContractTerminationCompletionPort` 同事务推进合同并发完成事件。本阶段规则注册数恰为 3，其余四类只有目录占位项，不存在第二个终止消费者或任何替身 |
| 电子签章的发起、状态轮询、结果拉取与验签 | job-worker 经 `\\.\pipe\ep-integ` 编排并落库，integration-gateway 只对外执行 | 电子签章是首版唯一合同外部集成，也是唯一 `EXTERNAL_SYSTEM` 错误分类来源；worker→gateway 只允许 `esign.request.submit.v1`、`esign.status.get.v1`，SIGNED 后 gateway→worker 在同一双工连接只允许 `esign_file.begin.v1`、`esign_file.chunk.v1`、`esign_file.end.v1`、`esign_file.abort.v1`。产品进程间不存在内部 HTTP；gateway 是唯一出网进程，并另可承载平台可选移动推送与同机客户 ICAP 出口，但数据库连接、Outbox 消费、文件对象/元数据、KMS 凭据及 clm 写入能力全部为零 |
| 合同模板渲染与 PDF 归档 | job-worker | 经 `ep_foundation::port::doc::DocTemplatePort::render` 与 `PdfRenderPort::render_pdf`，不新增接口，同步等待超过 8 秒的一律转后台任务 |

不新增进程，不改动任何进程的监听端口、系统账户与具名 Job Object 资源单位归属。

---

### 3. 数据库变更

#### 3.1 通用约定

下列约定对本阶段全部新建表成立，逐表不再重复。

- 每张表包含基线第 4 节的九个公共列：`id uuid`、`legal_entity_id uuid`、`security_level smallint default 20`、`data_scope_tags text[] default '{}'`、`row_version bigint default 1`、`created_at timestamptz default now()`、`created_by uuid`、`updated_at timestamptz default now()`、`updated_by uuid`。标注为仅追加的表不带 `row_version`、`updated_at`、`updated_by`。本阶段的版本快照、审批结论与签章事件均无“冲销某一父行”的业务语义，因此都不设 `reverses_id`；以后只有先冻结明确父链、反向效果和真实自/他表外键，才可为具体表增加该列。
- 每张表按基线第 3.8 节的统一模板启用并强制行级安全，策略名 `rls_<table>_le`，判据只有 `app.legal_entity_id`。本阶段不新增任何不带 `legal_entity_id` 的表。
- 每张表的基线索引固定为 `pk_<table>`、支撑跨 schema 外键的 `ux_<table>_legal_entity_id_id` 与 `ix_<table>_legal_entity_id_created_at`；单据类表另加 `ux_<table>_legal_entity_id_doc_no`。下表只列基线索引之外的追加索引。
- 枚举列一律 `text` 加 CHECK，取值大写 snake_case；金额 `numeric(18,2)`、单价与数量 `numeric(18,6)`、比例与税率 `numeric(9,6)`。
- 同 schema 与单一目标跨 schema 引用均以 `(legal_entity_id,<ref_id>)` 指向目标 `(legal_entity_id,id)` 建真实外键并 `ON DELETE RESTRICT`；业务用户列统一指向 `platform_authz.user_legal_entity_grants(legal_entity_id,user_id)`，附件列统一指向 `platform_file.attachment_objects(legal_entity_id,id)`。目标已存在时约束随建表迁移内联；唯一后建目标 `clm.contract_milestones.delivery_confirmation_id` 由迁移目录冻结的 `V20261017093700__clm_add_cross_schema_foreign_keys.sql` 在 `sales.delivery_confirmations` 建成后追补。`reauth_ref` 以单列真实外键指向 `platform_core.reauth_challenges(id)`，并在锁内校验证据主体与当前法人。应用层契约仍在事务内校验业务状态。只有 `subject_kind/subject_id`、`item_kind/item_id`、派生目标等显式登记的封闭多态引用，以及 `approval_ref`、`release_package_id` 保留基线第 3.3 节白名单形状；不得以不注册 `ReconCheck` 为由把单目标引用降为无约束逻辑标识。
- 本阶段全部为新建空表，索引随建表在同一迁移文件内用普通 `CREATE INDEX` 建立；只有后续对存量表追加索引才用 `CREATE INDEX CONCURRENTLY`。
- 文本列长度按基线第 11.2 节：编码 64、名称 200、简述 500、备注与原因与说明 2000、条款正文 1 MB，一律 `text` 加 CHECK。

#### 3.2 cpq schema 的变更

迁移 `V20261017090000__cpq_create_price_authorities.sql`。

表 `cpq.price_authorities`，档案类。

| 列 | 类型 | 可空 | 说明 |
|---|---|---|---|
| code | text | 否 | 档案编码，法人内唯一 |
| name | text | 否 | 名称 |
| subject_kind | text | 否 | CHECK in ROLE, POSITION, USER |
| subject_id | uuid | 否 | 与 `subject_kind` 组成 `ROLE|POSITION|USER` 封闭多态主体；写事务按 kind 校验同法人目标与有效授权，属于显式多态白名单，不建伪外键 |
| max_discount_rate | numeric(9,6) | 否 | 允许的最大折扣率，取值 0 至 1 |
| allow_below_price_floor | boolean | 否 | 是否允许净单价低于价目行的价格下限，默认 false |
| allow_no_price_list_hit | boolean | 否 | 价目未命中时是否视为权限内，默认 false |
| effective_from | date | 否 | 生效起日 |
| effective_to | date | 是 | 生效止日，空表示长期有效 |
| is_active | boolean | 否 | 默认 true |
| deactivated_at | timestamptz | 是 | 停用时间 |

约束与索引：`ux_price_authorities_legal_entity_id_code`；`ck_price_authorities_discount_range` 约束 `max_discount_rate >= 0 and max_discount_rate <= 1`；`ck_price_authorities_effective_range` 约束 `effective_to is null or effective_to >= effective_from`；追加索引 `ix_price_authorities_subject_kind_subject_id`。

#### 3.3 clm schema 的变更

迁移顺序如下，单一全局 Runner 按文件版本号全序执行，本阶段三个目录的文件版本号按 cpq、clm、sales 的被引用先后递增，与基线第 3.9 节一致。

| 迁移编号 | 建立对象 |
|---|---|
| V20261017090100 | clm.contract_types |
| V20261017090200 | clm.contract_templates、clm.contract_template_versions |
| V20261017090300 | clm.clauses、clm.clause_versions |
| V20261017090400 | clm.contracts |
| V20261017090500 | clm.contract_lines |
| V20261017090600 | clm.contract_terms |
| V20261017090700 | clm.contract_milestones |
| V20261017090800 | clm.contract_obligations |
| V20261017090900 | clm.contract_payment_schedules |
| V20261017091000 | clm.contract_attachments |
| V20261017091100 | clm.contract_annotations |
| V20261017091200 | clm.contract_versions |
| V20261017091300 | clm.contract_approvals |
| V20261017091400 | clm.signature_requests；随后为 clm.contract_attachments 的签章来源列补同法人外键 |
| V20261017091500 | clm.signature_events |
| V20261017091600 | clm.seal_usages |
| V20261017091700 | clm.contract_derivations、clm.contract_derivation_items |
| V20261017091800 | clm.contract_validations、clm.contract_validation_items |
| V20261017091900 | clm.contract_merge_links |
| V20261017092000 | clm.v_contract_milestone_progress、clm.v_contract_reminder_sources |
| V20261017092100 | clm.v_contracts_dataset、clm.v_contract_delivery_milestones |
| V20261017093700 | `clm_add_cross_schema_foreign_keys`：在 `sales.delivery_confirmations` 已存在后，为 `clm.contract_milestones.delivery_confirmation_id` 补 `(legal_entity_id,delivery_confirmation_id)` 复合外键；只允许 `ALTER TABLE ADD CONSTRAINT ... ON DELETE RESTRICT`，回退只删该约束 |
| V20261023092700 | `clm_harden_contract_economic_graph`：合同头行金额、单订单形状、版本快照与 ACTIVE 付款期次的延迟经济图 |

表 `clm.contract_types`，档案类。列为 `code text`、`name text`、`requires_project boolean default false`、`requires_procurement_default boolean default false`、`approval_terms_definition_id uuid`、`approval_discount_definition_id uuid`、`approval_payment_definition_id uuid`、`approval_attachment_definition_id uuid`、`default_template_id uuid`、`is_active boolean`、`deactivated_at timestamptz`。约束 `ux_contract_types_legal_entity_id_code`。四个审批定义列均以复合真实外键指向 `platform_flow.process_definitions(legal_entity_id,id) ON DELETE RESTRICT`，不再使用无法唯一定位版本的 code-only 逻辑键。

表 `clm.contract_templates`，档案类。列为 `code text`、`name text`、`contract_type_id uuid not null`（同 schema 外键）、`current_version_no int not null default 0`、`is_active boolean`、`deactivated_at timestamptz`。约束 `ux_contract_templates_legal_entity_id_code`。

表 `clm.contract_template_versions`，仅追加。列为 `contract_template_id uuid not null`、`version_no int not null`、`body_attachment_object_id uuid`、`default_terms jsonb not null default '{}'`、`clause_refs jsonb not null default '[]'`、`published_at timestamptz`、`release_package_id uuid`、`status text CHECK in DRAFT, PUBLISHED, RETIRED`。约束 `ux_contract_template_versions_template_version` 在 `(contract_template_id, version_no)`。发布经 ep-platform-release 的配置发布通道，`release_package_id` 属精确命名的平台发布证明白名单，由发布事务校验，不建伪外键。

表 `clm.clauses` 与 `clm.clause_versions`，结构与模板同构，`clauses` 另有 `category text`，`clause_versions` 有 `body text` 且 CHECK 长度不超过 1 MB。

表 `clm.contracts`，单据类。

| 列 | 类型 | 可空 | 说明 |
|---|---|---|---|
| doc_no | text | 否 | 由 ep-platform-sequence 生成，类型码 CT |
| status | text | 否 | CHECK in DRAFT, PENDING_APPROVAL, REJECTED, PENDING_SIGNATURE, EFFECTIVE, IN_PERFORMANCE, TERMINATING, COMPLETED, TERMINATED, VOID；TERMINATING 为非终态“终止处置中” |
| contract_type_id | uuid | 否 | 同 schema 外键 |
| customer_id | uuid | 否 | 复合外键指向 mdm.customers |
| name | text | 否 | 合同名称 |
| owner_user_id | uuid | 否 | 销售负责人 |
| valid_from | date | 否 | 有效期起日 |
| valid_to | date | 否 | 有效期止日 |
| signing_method | text | 否 | CHECK in ESIGN, PHYSICAL_SEAL |
| total_amount | numeric(18,2) | 否 | 合同不含税金额，由行汇总 |
| total_amount_with_tax | numeric(18,2) | 否 | 合同含税金额，由行汇总 |
| version_no | int | 否 | 合同版本号，默认 1 |
| template_id | uuid | 是 | 所用模板 |
| template_version_no | int | 是 | 所用模板版本 |
| renewed_from_contract_id | uuid | 是 | 续签的原合同 |
| merged_into_contract_id | uuid | 是 | 合并去向 |
| effective_at | timestamptz | 是 | 生效时点 |
| derivation_state | text | 否 | CHECK in NOT_STARTED, RUNNING, DONE, FAILED，默认 NOT_STARTED |
| terminated_reason | text | 是 | 提前终止原因 |
| remark | text | 是 | 备注 |

约束与索引：`ux_contracts_legal_entity_id_doc_no`；`ck_contracts_valid_range` 约束 `valid_to >= valid_from`；追加索引 `ix_contracts_legal_entity_id_customer_id_status`、`ix_contracts_legal_entity_id_status_valid_to`（合同到期提醒取数）、`ix_contracts_renewed_from_contract_id`。

表 `clm.contract_lines`。列除公共列外为 `contract_id uuid not null`（外键）、`line_no int not null`、`item_kind text CHECK in PRODUCT, MATERIAL`、`item_id uuid not null`（封闭多态引用）、`costing_mode text not null CHECK in INVENTORY, DIRECT_EXPENSE`（取自 MDM 并随合同版本冻结）、`inventory_material_id uuid`（复合外键指向 mdm.materials，随合同版本冻结）、`uom_code text not null`、`quantity numeric(18,6) not null`、`list_unit_price numeric(18,6)`、`price_floor numeric(18,6)`、`unit_price numeric(18,6) not null`、`discount_rate numeric(9,6) not null default 0`、`net_unit_price numeric(18,6) not null`、`is_tax_included boolean not null default false`、`tax_rate numeric(9,6) not null default 0`、`line_amount numeric(18,2) not null`、`line_amount_with_tax numeric(18,2) not null`、`delivery_date date not null`、`warehouse_id uuid`（复合外键指向 mdm.warehouses）、`order_type text not null CHECK in NORMAL, DROP_SHIP, CONSIGNMENT, SUBSCRIPTION, LEASE`、`cycle_unit text`、`cycle_length int`、`lease_from date`、`lease_to date`、`auto_renew boolean not null default false`、`requires_procurement boolean not null default false`、`requires_discount_approval boolean not null default false`、`price_list_id uuid`（复合外键指向 cpq.price_lists）、`price_list_line_id uuid`（复合外键指向 cpq.price_list_lines）、`source_contract_line_id uuid`（同 schema 外键）。两项库存快照必须经阶段 5 冻结的 `MasterDataLookup::resolve_sales_item_profile` 一次取得：MATERIAL 行固定为 INVENTORY 且 `inventory_material_id=item_id`；PRODUCT 行取 `mdm.products.costing_mode`，INVENTORY 产品带出唯一启用关联物料，DIRECT_EXPENSE 产品的 `inventory_material_id` 为空。

约束与索引：`ux_contract_lines_contract_id_line_no`；另建候选键 `UNIQUE(legal_entity_id,contract_id,id)` 供销售来源长外键使用；`ck_contract_lines_quantity_positive`；`ck_contract_lines_discount_range`；`ck_contract_lines_item_costing` 强制 MATERIAL 只能取 INVENTORY 且物料快照等于 item_id；`ck_contract_lines_inventory_material` 强制 INVENTORY 时物料快照非空、DIRECT_EXPENSE 时为空；`ck_contract_lines_warehouse_required` 约束 `costing_mode = 'DIRECT_EXPENSE' or order_type = 'DROP_SHIP' or warehouse_id is not null`；`ck_contract_lines_cycle_required` 约束 `order_type not in ('SUBSCRIPTION','LEASE') or (cycle_unit is not null and cycle_length is not null)`；追加索引 `ix_contract_lines_contract_id`、`ix_contract_lines_legal_entity_id_item_id`。

表 `clm.contract_terms`，一合同一行。列为 `contract_id uuid not null unique`、`body text`、`warranty_clause text`、`liability_clause text`、`dispute_resolution text`、`structured jsonb not null default '{}'`、`clause_refs jsonb not null default '[]'`。约束 `ux_contract_terms_contract_id`。

表 `clm.contract_milestones`。列为 `contract_id uuid not null`、`milestone_no int not null`、`name text not null`、`promised_date date not null`、`status text not null CHECK in PLANNED, ACTIVE, CONFIRMED, CANCELLED`、`confirmed_date date`、`delivery_confirmation_id uuid`（复合外键指向 `sales.delivery_confirmations`，由后序 `clm_add_cross_schema_foreign_keys` 追补）、`owner_user_id uuid`（复合外键指向用户法人授权）。约束 `ux_contract_milestones_contract_id_milestone_no`；追加索引 `ix_contract_milestones_legal_entity_id_promised_date_status`（到期提醒与交付指标取数）。该表不带产品、物料与订单字段，与规格第 5.5 章经营驾驶舱条目的口径一致。

表 `clm.contract_obligations`。列为 `contract_id uuid not null`、`seq_no int not null`、`name text not null`、`description text`、`due_date date`、`status text CHECK in OPEN, FULFILLED, WAIVED`。

表 `clm.contract_payment_schedules`。列为 `contract_id uuid not null`、`period_no int not null`、`condition_text text`、`basis text not null CHECK in RATIO, AMOUNT`、`ratio numeric(9,6)`、`amount numeric(18,2)`、`amount_with_tax numeric(18,2)`、`due_date date not null`、`status text not null default 'ACTIVE' CHECK in ('ACTIVE','VOIDED')`、`void_reason text`、`remark text`。约束 `ux_contract_payment_schedules_contract_id_period_no`；NULL-safe `ck_contract_payment_schedules_basis` 约束 RATIO 时 `ratio` 非空且 `amount/amount_with_tax` 同为空，AMOUNT 时 `ratio` 为空且 `amount/amount_with_tax` 同为非空；`ck_contract_payment_schedules_void_reason` 约束 VOIDED 时 reason 非空、ACTIVE 时 reason 为空；追加索引 `ix_contract_payment_schedules_legal_entity_id_due_date`。这里的 VOIDED 是期次自身状态，不是合同作废，也不增加 SETTLED：已结清仍由 finance 权威判定。提交审批及以后，同一合同的 ACTIVE 行 basis 不得混用，且 RATIO 合计必须精确等于 `1.000000`，AMOUNT 的不含税/含税合计必须分别精确等于合同头两项总额；这些条件由下述延迟经济图而非仅由应用校验。按裁定 C-20，本表是收付款计划行的唯一出处，`ep_contract_finance::ReceivablePlanPort` 已撤销，finance 不再派生第二套；`ep_contract_clm::ContractPaymentScheduleQuery::schedules(tx, ctx, contract_id)` 的 DTO 固定新增 `status`，阶段 10 的到款自动核销只取 ACTIVE 行。期次净已开金额的唯一权威是 `invoice.invoice_receipt_plan_links`，clm 不保存金额副本、不接收开票回写事件，也不经 Outbox 保持副本最终一致。

`ep_contract_invoice::ReceiptPlanBillingQuery` 的唯一方法升级并冻结为 `billing_by_period(tx: &mut dyn Tx, ctx: &SecurityContext, contract_id: Id<Contract>) -> Result<BTreeMap<i32, Money>, AppError>`。实现只读 invoice owner 数据，按各期正向发票分摊减去 VOID/RED_LETTER 的反向分摊返回当前净额；没有链接的期次由调用方视为零，返回值不得为负。合同履约页、合同变更的“尚未开票”守卫与 `CLM_TERM_PAYMENT_SCHEDULE` 规则只消费这一方法：净额大于零即已开票占用，等于零才可按各自规则调整或作废。

表 `clm.contract_attachments`，按基线第 4 节的附件关联表规范。列为 `owner_id uuid not null`（指向 contracts.id）、`attachment_object_id uuid not null`、`purpose text not null CHECK in CONTRACT_BODY, SIGNED_FILE, SEAL_SCAN, SUPPORTING`、`sort_no int not null default 0`、`contract_version_no int not null`、`source_signature_request_id uuid null`、`source_file_ordinal int null`。`ck_contract_attachments_signature_source` 强制两个来源列同时为空，或 `purpose='SIGNED_FILE'` 且两列同时非空、ordinal 非负；普通唯一约束 `ux_contract_attachments_signature_file` 固定在 `(legal_entity_id,source_signature_request_id,source_file_ordinal)`，PostgreSQL 默认 `NULLS DISTINCT` 允许多个无签章来源附件，同时使每个非空回传文件只能关联一次，不建立基线禁止的部分索引。追加索引 `ix_contract_attachments_owner_id_purpose`。本表先于签章请求表建立，因此第 V20261017091400 号迁移在创建 `clm.signature_requests` 后同文件补 `fk_contract_attachments_signature_request (legal_entity_id,source_signature_request_id) → clm.signature_requests(legal_entity_id,id)`；回退先删该 FK 再删签章请求表，不保留无约束窗口到阶段退出。

表 `clm.contract_annotations`。列为 `contract_id uuid not null`、`attachment_object_id uuid not null`、`attachment_version_no int not null`、`page_no int`、`anchor jsonb not null default '{}'`、`body text not null`、`state text not null CHECK in OPEN, RESOLVED`、`resolved_by uuid`、`resolved_at timestamptz`。追加索引 `ix_contract_annotations_contract_id_state`。

表 `clm.contract_versions`，仅追加。列为 `contract_id uuid not null`、`version_no int not null`、`snapshot jsonb not null`、`change_reason text`、`created_at`、`created_by`。约束 `ux_contract_versions_contract_id_version_no` 精确建于 `(legal_entity_id,contract_id,version_no)`，既保证法人内版本唯一，也作为销售订单与阶段 12 来源合同版本复合外键的候选键。快照必须按固定 schema 保存 `header`、按 `line_no` 排序的 `lines`、`terms`、`milestones`、`payment_schedules` 与 `attachments`；每个 lines 元素逐字包含合同行 id、item/costing/material/uom/quantity/net_unit_price/is_tax_included/tax_rate/两项行额、交期/仓库以及 order_type/cycle/lease/auto_renew。快照内容用于版本比较与下游来源图复核；版本之间按 `version_no` 追溯，不伪造冲销父链。

`db/migrations/clm/V20261023092700__clm_harden_contract_economic_graph.sql` 安装 `clm.assert_contract_economic_graph_consistent()`，以 `DEFERRABLE INITIALLY DEFERRED` 约束触发器覆盖 `contracts`、`contract_lines`、`contract_payment_schedules` 与 `contract_versions` 的 INSERT/UPDATE/DELETE。提交点逐行复算本节第 4.4 节税内/税外两项金额并强制合同头等于行和；非 DRAFT 合同至少一行。因为一次派生固定只生成一张销售订单，所有有效合同行的 `order_type/cycle_unit/cycle_length/lease_from/lease_to/auto_renew` 必须逐值相同，周期与租赁字段仍受各自行 CHECK。PENDING_APPROVAL、REJECTED、PENDING_SIGNATURE、EFFECTIVE、IN_PERFORMANCE、TERMINATING、COMPLETED、TERMINATED、VOID 各态均要求 ACTIVE 付款期次存在、basis 单一且合计按上一段闭合；DRAFT 可暂时不闭合，VOIDED 期次不计合计。

同一迁移再安装 `BEFORE INSERT` 的 `clm.assert_contract_version_snapshot_matches_current()` 与 contract_versions 的仅追加守卫。插入版本时必须锁合同头、行、期次及其余快照来源，并逐字段证明新 snapshot 等于插入时权威表；UPDATE/DELETE 一律拒绝。上述非 DRAFT 提交点还要求 `(contract_id,contracts.version_no)` 快照恰有一行且该当前版本 snapshot 仍等于提交点权威头/行/期次；历史版本只保持不可变，不与当前行重比。版本变更事务因此固定先更新权威行与 `contracts.version_no`、再插入匹配的新当前快照，提交前两者必须闭合。普通 FK 全命中但错头合计、错税内公式、混合订单类型、付款 basis 混用/合计不闭合或伪造当前版本快照都被拒；回退先移除四表触发器与两项守卫再删函数，不改业务数据。

表 `clm.contract_approvals`，仅追加。列为 `contract_id uuid not null`、`contract_version_no int not null`、`chain_kind text not null CHECK in TERMS, DISCOUNT, PAYMENT, ATTACHMENT, CREDIT, EFFECTIVE, TERMINATION`、`flow_instance_id uuid not null`（复合外键指向 `platform_flow.process_instances(legal_entity_id,id)`）、`outcome text CHECK in APPROVED, REJECTED, RETURNED, WITHDRAWN`、`concluded_at timestamptz`、`approver_user_id uuid`（复合外键指向用户法人授权）、`comment text`、`reauth_ref uuid`（单列外键指向 `platform_core.reauth_challenges(id)`）。追加索引 `ix_contract_approvals_contract_id_chain_kind`、`ix_contract_approvals_flow_instance_id`。`TERMINATION` 出厂链为管理者审批且申请人不可自审，审批结论是进入 TERMINATING 的必要条件。

出厂链中 `EFFECTIVE` 的管理者必经节点固定为单节点 `MANAGEMENT_APPROVER`；`CREDIT` 信用超额链固定为单节点 `FINANCE_MANAGER`。两链均禁止申请人自审，节点展开为空时拒绝提交。客户后续可经签名配置发布把 EFFECTIVE 节点替换为 ROLE、POSITION 或 DEPT_MANAGER，但首版 schema 不含金额条件。

表 `clm.signature_requests`。列为 `contract_id uuid not null`、`contract_version_no int not null`、`provider_code text not null`、`external_request_id text`、`status text not null default 'PENDING' CHECK in PENDING, SUBMITTED, SIGNING, SIGNED, REJECTED, FAILED, CANCELLED`、`submitted_at timestamptz`、`concluded_at timestamptz`、`attempts int not null default 0`、`next_poll_at timestamptz`、`last_error text`、`signed_file_count int not null default 0 CHECK (signed_file_count >= 0)`、`verify_result text not null default 'NOT_VERIFIED' CHECK in NOT_VERIFIED, PASSED, FAILED`、`evidence_hash bytea`。`ck_signature_requests_signed_files` 强制 SIGNED 时 `signed_file_count>0 and verify_result='PASSED'`，非 SIGNED 时 `signed_file_count=0 and verify_result<>'PASSED'`；签章文件不在本表保存单一附件 id，逐文件关联只在 `clm.contract_attachments(source_signature_request_id, source_file_ordinal)` 建立。约束 `ux_signature_requests_contract_id_version` 在 `(contract_id, contract_version_no)`，并以 `ux_signature_requests_legal_entity_id_id` 支撑上一段同法人外键；追加索引 `ix_signature_requests_status_next_poll_at`。

表 `clm.signature_events`，仅追加。列为 `signature_request_id uuid not null`、`occurred_at timestamptz not null`、`kind text not null CHECK in SUBMITTED, POLLED, SIGNED, REJECTED, FAILED, VERIFIED`、`external_status text`、`payload_digest bytea`、`evidence_attachment_object_id uuid`（复合外键指向 `platform_file.attachment_objects(legal_entity_id,id)`）。追加索引 `ix_signature_events_signature_request_id_occurred_at`。外部返回的原始报文不落列；`payload_digest` 只对管道清洗回执计算摘要，`evidence_attachment_object_id` 只引用平台已完整接收并校验的签章文件或人工证据，不承接服务商原始响应。该表是事件事实而非冲销效果链，不设 `reverses_id`。

表 `clm.seal_usages`。列为 `contract_id uuid not null`、`contract_version_no int not null`、`seal_name text not null`、`used_at timestamptz not null`、`operator_user_id uuid not null`、`scan_attachment_object_id uuid not null`、`remark text`。追加索引 `ix_seal_usages_contract_id`。

表 `clm.contract_derivations`。列为 `contract_id uuid not null`、`contract_version_no int not null`、`trigger text not null CHECK in EFFECTIVE, AMENDMENT, RENEWAL`、`status text not null CHECK in RUNNING, DONE, FAILED`、`flow_instance_id uuid`、`started_at timestamptz not null`、`finished_at timestamptz`、`item_total int not null default 0`、`item_done int not null default 0`。约束 `ux_contract_derivations_contract_id_version_trigger` 在 `(contract_id, contract_version_no, trigger)`，这是派生幂等的第一道保证。

表 `clm.contract_derivation_items`。列为 `contract_derivation_id uuid not null`（外键）、`artifact_kind text not null CHECK in SALES_ORDER, PURCHASE_REQUISITION, PROJECT_TASK, RECEIVABLE_PLAN, MILESTONE`、`source_ref_id uuid`（合同行或期次或交付节点的 id，整单粒度的为空）、`target_module text not null`、`target_doc_id uuid`、`target_doc_no text`、`status text not null CHECK in PENDING, DISPATCHING, DONE, DEAD`、`attempts int not null default 0`、`available_at timestamptz not null default now()`、`last_error text`、`idempotency_key uuid not null`。约束 `ux_contract_derivation_items_unique` 在 `(contract_derivation_id, artifact_kind, coalesce(source_ref_id, contract_derivation_id))`，这是派生幂等的第二道保证；追加索引 `ix_contract_derivation_items_status_available_at`。

表 `clm.contract_validations`。列为 `contract_id uuid not null`、`contract_version_no int not null`、`occasion text not null CHECK in SUBMIT, DERIVE, MERGE_RESUBMIT, RENEW_SUBMIT`、`verdict text not null CHECK in PASSED, BLOCKED, REVIEW_REQUIRED`、`evaluated_at timestamptz not null`、`evaluated_by uuid not null`、`audit_event_id uuid`。追加索引 `ix_contract_validations_contract_id_occasion`。

表 `clm.contract_validation_items`。列为 `contract_validation_id uuid not null`（外键）、`check_kind text not null CHECK in PRICE_AUTHORITY, CONTRACT_INTEGRITY, STOCK_AVAILABILITY, LEAD_TIME, CREDIT_LIMIT`、`result text not null CHECK in PASSED, FAILED, FLAGGED`、`source_line_id uuid`、`snapshot jsonb not null default '{}'`、`message_code text`。快照内容为该项取数的输入与输出，例如信用项记录信用额度、三部分占用取值、本次待增加占用与判定结论，直接对应 PRD 3.14.3 的取数快照要求。

表 `clm.contract_merge_links`，按基线的多对多命名。列为 `source_contract_id uuid not null`、`target_contract_id uuid not null`、`merged_at timestamptz not null`、`merged_by uuid not null`。约束 `ux_contract_merge_links_source_target`。

视图 `clm.v_contract_milestone_progress`：按合同聚合交付节点的计划数、已确认数、逾期数与最近到期日，只读取本 schema 的表。

视图 `clm.v_contract_reminder_sources`：把合同有效期止日、交付节点约定日期、收付款期次到期日三类触发源统一为 `(legal_entity_id, contract_id, source_kind, due_date, owner_user_id)` 五列，供 ep-platform-flow 的定时器取数。三类触发源与 PRD 3.9.2 的三行一一对应，但固定排除合同状态为 TERMINATING 或 TERMINATED 的全部行，并排除 `contract_payment_schedules.status='VOIDED'` 的期次，避免终止处置后继续生成到期提醒。
按裁定 A-18，第 V20261017092100 号迁移 `V20261017092100__clm_create_dataset_views.sql` 建立本模块的两个受治理数据集视图。`clm.v_contracts_dataset` 的 dataset code 为 `clm_contracts`，grain 取 DOCUMENT，取数为 `clm.contracts`；`clm.v_contract_delivery_milestones` 的 dataset code 为 `clm_contract_delivery_milestones`，grain 取 DOCUMENT_LINE，取数为 `clm.contract_milestones`，输出列含 `contract_id`、`milestone_no`、`name`、`promised_date`、`status`、`confirmed_date`、`delivery_confirmation_id`、`owner_user_id`。两个视图都必须含 `legal_entity_id`、`security_level`、`data_scope_tags` 三列，都不做聚合、不跨 schema 连接，并在同一迁移内执行 `GRANT SELECT ON <视图> TO ep_analyst_ro`，不授予 `ep_app_rw` 之外的任何写权限。列名与类型签名必须与阶段 11 的 `reporting.dataset_fields` 登记一致，由阶段 11 的启动自检项 `reporting-dataset-signature-matched` 校验，本阶段在第 9 节退出条件中把列签名同步给阶段 11。该文件头的 `-- rollback:` 段为 `drop view` 与对应的 `revoke`。

#### 3.4 sales schema 的变更

| 迁移编号 | 建立对象 |
|---|---|
| V20261017092200 | sales.credit_policies |
| V20261017092300 | sales.customer_credit_controls |
| V20261017092400 | sales.sales_orders |
| V20261017092500 | sales.sales_order_lines |
| V20261017092600 | sales.delivery_schedules |
| V20261017092700 | sales.delivery_confirmations |
| V20261017092800 | sales.delivery_confirmation_lines |
| V20261017092900 | sales.sales_order_versions |
| V20261017093000 | sales.sales_order_changes、sales.sales_order_change_lines |
| V20261017093100 | sales.sales_returns、sales.sales_return_lines |
| V20261017093200 | sales.return_line_delivery_links、sales.return_line_capture_allocations 与退货延迟图 |
| V20261017093300 | sales.exchange_links |
| V20261017093400 | sales.order_validations、sales.order_validation_items |
| V20261017093500 | sales.v_credit_exposure_in_transit |
| V20261017093600 | sales.v_sales_orders_dataset、sales.v_order_delivery_batches |
| V20261017093630 | 为 sales.return_line_capture_allocations 登记 append-only registry 并安装仅追加守卫 |
| V20261023092800 | 订单来源版本、头行金额、分批、交付累计区间、税内价尾差与 open amount 的延迟经济图 |

表 `sales.credit_policies`，法人级策略，经配置发布通道维护。列为 `scope text not null CHECK in LEGAL_ENTITY`、`on_exceed text not null CHECK in BLOCK, REVIEW`、`null_limit_behavior text not null CHECK in TREAT_AS_ZERO, TREAT_AS_UNLIMITED, SKIP_CHECK`、`amount_basis text not null default 'WITH_TAX' CHECK (amount_basis = 'WITH_TAX')`、`deduct_advance_receipts boolean not null default false`、`recheck_on_order_change boolean not null default true`、`release_package_id uuid`。约束 `ux_credit_policies_legal_entity_id_scope`，每法人一行。`amount_basis` 是为未来版本化迁移保留的显式证据列，首版只接受 `WITH_TAX`，配置发布包、API 和数据库均不得写入 `WITHOUT_TAX`。

表 `sales.customer_credit_controls`，每法人每客户一行，同时是信用校验的串行化点。列为 `customer_id uuid not null`、`on_exceed_override text CHECK in BLOCK, REVIEW`、`last_checked_at timestamptz`、`last_exposure jsonb not null default '{}'`。约束 `ux_customer_credit_controls_legal_entity_id_customer_id`。该行在信用校验事务内以 `SELECT ... FOR UPDATE` 取用，不存在时以 `INSERT ... ON CONFLICT DO NOTHING` 建立。

表 `sales.sales_orders`，单据类。

| 列 | 类型 | 可空 | 说明 |
|---|---|---|---|
| doc_no | text | 否 | 类型码 SO |
| status | text | 否 | CHECK in PENDING_RELEASE, RELEASED, CHANGE_APPROVAL, PARTIALLY_DELIVERED, DELIVERED, CLOSED, CANCELLED |
| customer_id | uuid | 否 | 复合外键指向 `mdm.customers(legal_entity_id,id)` |
| source_contract_id | uuid | 否 | 复合外键指向 `clm.contracts(legal_entity_id,id)` |
| source_contract_version_no | int | 否 | 来源合同版本 |
| order_type | text | 否 | CHECK in NORMAL, DROP_SHIP, CONSIGNMENT, SUBSCRIPTION, LEASE |
| owner_user_id | uuid | 否 | 销售负责人 |
| total_amount | numeric(18,2) | 否 | 不含税合计 |
| total_amount_with_tax | numeric(18,2) | 否 | 含税合计 |
| promised_from | date | 否 | 约定交期区间起 |
| promised_to | date | 否 | 约定交期区间止 |
| ship_to_address | text | 是 | 收货地址 |
| cycle_unit | text | 是 | 订阅或租赁周期单位 |
| cycle_length | int | 是 | 周期长度 |
| lease_from | date | 是 | 租期起 |
| lease_to | date | 是 | 租期止 |
| auto_renew | boolean | 否 | 默认 false |
| version_no | int | 否 | 订单版本号，默认 1 |
| pending_release_reason | text | 是 | CHECK in CREDIT, STOCK, CREDIT_AND_STOCK |
| closed_reason | text | 是 | 关闭或取消原因 |
| remark | text | 是 | 备注 |

约束与索引：`ux_sales_orders_legal_entity_id_doc_no`；`fk_sales_orders_source_contract_version(legal_entity_id,source_contract_id,source_contract_version_no) -> clm.contract_versions(legal_entity_id,contract_id,version_no) ON DELETE RESTRICT DEFERRABLE INITIALLY DEFERRED`，不能只给 source_contract_id 建单列外键；追加索引 `ix_sales_orders_legal_entity_id_customer_id_status`（信用在途桶聚合）、`ix_sales_orders_source_contract_id`、`ix_sales_orders_legal_entity_id_status_promised_to`。

表 `sales.sales_order_lines`。列除公共列外为 `sales_order_id uuid not null`（外键）、`line_no int not null`、`customer_id uuid not null`（冗余自订单头，使信用聚合可走覆盖索引）、`source_contract_id uuid not null`、`source_contract_version_no int not null`、`source_contract_line_id uuid not null`、`item_kind text`、`item_id uuid not null`、`costing_mode text not null CHECK in INVENTORY, DIRECT_EXPENSE`（自合同行原样冻结）、`inventory_material_id uuid`（自合同行原样冻结，复合外键指向 `mdm.materials`）、`is_drop_ship boolean not null`（自订单头类型派生并冻结）、`inventory_demand_state text not null default 'INACTIVE' CHECK in INACTIVE, CONFIRMED, RELEASED`、`open_inventory_demand_slot boolean GENERATED ALWAYS AS (CASE WHEN inventory_demand_state in ('CONFIRMED','RELEASED') and status in ('OPEN','PARTIALLY_DELIVERED') and costing_mode='INVENTORY' and is_drop_ship=false THEN true ELSE NULL END) STORED`、`uom_code text not null`、`quantity numeric(18,6) not null`、`net_unit_price numeric(18,6) not null`、`is_tax_included boolean not null`、`tax_rate numeric(9,6) not null default 0`、`line_amount numeric(18,2) not null`、`line_amount_with_tax numeric(18,2) not null`、`delivery_date date not null`、`warehouse_id uuid`（复合外键指向 `mdm.warehouses`）、`delivered_quantity numeric(18,6) not null default 0`、`returned_quantity numeric(18,6) not null default 0`、`open_amount_with_tax numeric(18,2) not null`、`status text not null CHECK in OPEN, PARTIALLY_DELIVERED, DELIVERED, CLOSED, CANCELLED`。三项 source 列由订单头与来源版本快照派生且客户端不可写。`inventory_demand_state` 是订单行是否进入 U-G-01 未交付需求集合的业务状态，不是库存预留表；DIRECT_EXPENSE、直运和终态行固定为 INACTIVE。

约束与索引：`ux_sales_order_lines_sales_order_id_line_no`；额外建立 `UNIQUE(legal_entity_id,sales_order_id,id)` 作为阶段 12 头行一致性复合外键的候选键；`fk_sales_order_lines_source_contract_line(legal_entity_id,source_contract_id,source_contract_line_id) -> clm.contract_lines(legal_entity_id,contract_id,id) ON DELETE RESTRICT DEFERRABLE INITIALLY DEFERRED`，并由下述图证明该行仍属于头所指 version snapshot；`ck_sales_order_lines_delivered_range` 约束 `delivered_quantity >= 0 and delivered_quantity <= quantity`；`ck_sales_order_lines_returned_range` 约束 `returned_quantity >= 0 and returned_quantity <= delivered_quantity`；`ck_sales_order_lines_inventory_material` 与合同行同义；`ck_sales_order_lines_demand_eligible` 强制只有 `costing_mode=INVENTORY`、`is_drop_ship=false` 且仓库与物料快照均非空的非终态行可取 CONFIRMED 或 RELEASED；追加索引 `ix_sales_order_lines_sales_order_id`、`ix_sales_order_lines_legal_entity_id_customer_id_status`（包含 `open_amount_with_tax` 的列顺序设计为使信用在途桶聚合不出现顺序扫描）、`ix_sales_order_lines_source_contract_line_id`，以及普通索引 `ix_sales_order_lines_open_inventory_demand`，键为 `(legal_entity_id,open_inventory_demand_slot,warehouse_id,inventory_material_id)`、包含 `(quantity,delivered_quantity)`。查询固定带 `open_inventory_demand_slot=true`；槽位由数据库生成且客户端不可写，因此无需部分索引或函数索引。本表是新建空表，该索引随建表用普通 `CREATE INDEX`，不使用 CONCURRENTLY。

表 `sales.delivery_schedules`。列为 `sales_order_id uuid not null`、`sales_order_line_id uuid not null`、`batch_no int not null`、`quantity numeric(18,6) not null`、`promised_date date not null`、`warehouse_id uuid`、`delivered_quantity numeric(18,6) not null default 0`、`status text not null CHECK in PENDING, DELIVERED, CLOSED, CANCELLED`。`V20261017092600__sales_create_delivery_schedules.sql` 同批建立候选键 `UNIQUE(legal_entity_id,sales_order_id,sales_order_line_id,id)`，并把 `(legal_entity_id,sales_order_id,sales_order_line_id)` 以 `ON DELETE RESTRICT` 真实复合外键指向 `sales.sales_order_lines(legal_entity_id,sales_order_id,id)`；不得保留两个各自能命中、却允许订单头与订单行来自不同订单的独立外键。约束 `ux_delivery_schedules_line_batch` 在 `(sales_order_line_id, batch_no)`；`ck_delivery_schedules_quantity_positive`；`ck_delivery_schedules_delivered_range` 强制 `0 <= delivered_quantity <= quantity`，且非终止业务分支中只有 `delivered_quantity=quantity` 才可取 DELIVERED，未交完保持 PENDING；追加索引 `ix_delivery_schedules_legal_entity_id_promised_date_status`（交付指标的期间维度取数）、`ix_delivery_schedules_sales_order_id`。同一分批行允许由多张交付确认单部分交付，权威关联只取 `sales.delivery_confirmation_lines.delivery_schedule_id`；头上不保存会被第二次交付覆盖的单值 `delivery_confirmation_id/confirmed_date`。

表 `sales.delivery_confirmations`，单据类，类型码 DC，按裁定 A-09 由第 V20261017092700 号迁移建立。

| 列 | 类型 | 可空 | 说明 |
|---|---|---|---|
| doc_no | text | 否 | 由 ep-platform-sequence 生成，类型码 DC |
| status | text | 否 | CHECK in DRAFT, CONFIRMED |
| customer_id | uuid | 否 | 复合外键指向 `mdm.customers(legal_entity_id,id)` |
| sales_order_id | uuid | 否 | 同 schema 外键 |
| posting_date | date | 否 | 记账日期，取值与用途见基线第 3.4 节 |
| warehouse_id | uuid | 是 | 出库仓库，直运时为空 |
| is_drop_ship | boolean | 否 | 默认 false |
| confirmed_at | timestamptz | 是 | 确认过账时点 |
| confirmed_by | uuid | 是 | 确认人 |
| voucher_id | uuid | 是 | 复合外键指向 `ledger.vouchers(legal_entity_id,id)`，确认时由凭证腿同事务写入 |
| remark | text | 是 | 备注 |

约束与索引：`ux_delivery_confirmations_legal_entity_id_doc_no`、候选键 `UNIQUE(legal_entity_id,sales_order_id,id)`；`ck_delivery_confirmations_confirmation_shape` 强制 DRAFT 时 `confirmed_at/confirmed_by/voucher_id` 三者全空，CONFIRMED 时 `confirmed_at/confirmed_by` 全非空而 `voucher_id` 按下述零效果规则可空；追加索引 `ix_delivery_confirmations_sales_order_id`、`ix_delivery_confirmations_legal_entity_id_posting_date`。`V20261017092800__sales_create_delivery_confirmation_lines.sql` 在三张目标表齐备后建立 `sales.assert_delivery_confirmation_graph_consistent()`，并在 `delivery_confirmations`、`delivery_confirmation_lines` 上各装一个 `DEFERRABLE INITIALLY DEFERRED` 约束触发器；后述第 92800 号硬化迁移以同名函数终态替换并把触发源扩到订单与分批表。提交时函数按稳定 id 锁读完整关系图并强制：头 `customer_id` 等于订单客户，`is_drop_ship=(sales_orders.order_type='DROP_SHIP')`；非直运头的 `warehouse_id` 非空并等于每条明细、订单行与分批行仓库，直运头及其明细仓库为空；每条明细的产品、计价模式、物料、单位、净价、`is_tax_included` 与税率逐值等于所属订单行，且数量不超过同一分批行锁后剩余量。DRAFT 行的 `allocation_quantity_before` 为空；CONFIRMED 行必须非空，同一订单行的全部已确认区间从 0 连续、无重叠无空洞。设区间起点 B、本次量 q，按第 4.2 节定义累计函数 `cum_net(x)` 与 `cum_gross(x)`，行额必须分别等于 `cum_net(B+q)-cum_net(B)` 与 `cum_gross(B+q)-cum_gross(B)`，从而税内价不会再乘税且最后一段自动吸收尾差。DRAFT 时全部明细 `cogs_amount/stock_movement_id` 均为空；CONFIRMED 且头非直运的每条 INVENTORY 行两项全非空，直运或 DIRECT_EXPENSE 行两项全空。CONFIRMED 头的 `voucher_id IS NULL` 当且仅当全部行 `line_amount=0 AND line_amount_with_tax=0 AND coalesce(cogs_amount,0)=0`，任一会计效果非零时凭证必填，全部效果为零时不得伪造零金额凭证；非空凭证还必须是同法人、`source_kind=DELIVERY_CONFIRMATION`、`source_document_type='DELIVERY_CONFIRMATION'`、`source_document_id=delivery_confirmations.id` 的本次普通凭证。任一半回填、漏回填、跨订单拼接、错快照、错累计区间/尾差或凭证零值/来源形状不符均拒绝提交。回退先删两个触发器与函数，再删本表。不设作废态，冲正一律经销售退货单，理由是基线第 3.6 节禁止软删除且已过账分录只追加。本表不带 `accounting_period_id`，与第 11.2 小节的偏离项一并登记。

表 `sales.delivery_confirmation_lines`，按裁定 A-09 由第 V20261017092800 号迁移建立。列除公共列外为 `delivery_confirmation_id uuid not null`、`sales_order_id uuid not null`（冗余祖先键，客户端不可写）、`line_no int not null`、`sales_order_line_id uuid not null`、`delivery_schedule_id uuid not null`、`item_kind text`、`item_id uuid not null`、`costing_mode text not null CHECK in INVENTORY, DIRECT_EXPENSE`、`inventory_material_id uuid`（自订单行冻结，复合外键指向 `mdm.materials`）、`uom_code text not null`、`quantity numeric(18,6) not null`、`net_unit_price numeric(18,6) not null`、`is_tax_included boolean not null`、`tax_rate numeric(9,6) not null default 0`、`allocation_quantity_before numeric(18,6)`、`line_amount numeric(18,2) not null`、`line_amount_with_tax numeric(18,2) not null`、`warehouse_id uuid`（复合外键指向 `mdm.warehouses`）、`batch_no text not null default '-'`、`serial_nos text[] not null default '{}'`、`cogs_amount numeric(18,2)`（仅 INVENTORY 行由库存腿回填）、`stock_movement_id uuid`（仅 INVENTORY 行复合外键指向 `inventory.stock_movements`）。`allocation_quantity_before` 只能在确认事务由服务端按锁后累计量写入，客户端与 DRAFT 登记不可写。候选键为 `UNIQUE(legal_entity_id,delivery_confirmation_id,id)` 与 `UNIQUE(legal_entity_id,sales_order_id,sales_order_line_id,delivery_confirmation_id,id)`；三条 `ON DELETE RESTRICT` 长复合外键分别为 `(legal_entity_id,sales_order_id,delivery_confirmation_id) -> delivery_confirmations(legal_entity_id,sales_order_id,id)`、`(legal_entity_id,sales_order_id,sales_order_line_id) -> sales_order_lines(legal_entity_id,sales_order_id,id)`、`(legal_entity_id,sales_order_id,sales_order_line_id,delivery_schedule_id) -> delivery_schedules(legal_entity_id,sales_order_id,sales_order_line_id,id)`。另有 `ux_delivery_confirmation_lines_confirmation_id_line_no`、`ck_delivery_confirmation_lines_allocation_nonnegative`、`ck_delivery_confirmation_lines_inventory_material` 与 `ck_delivery_confirmation_lines_direct_no_stock`（DIRECT_EXPENSE 时物料快照、cogs_amount、stock_movement_id 均为空），并追加索引 `ix_delivery_confirmation_lines_sales_order_line_id`。这些候选键同时供销售退货与阶段 12 的设备来源头行一致性外键使用；批次列与序列号列取固定值按基线第 11.4 节。

表 `sales.sales_order_versions`，仅追加。列为 `sales_order_id uuid not null`、`version_no int not null`、`snapshot jsonb not null`、`change_id uuid`、`created_at`、`created_by`。约束 `ux_sales_order_versions_order_version`；版本按 `version_no` 形成序列，不设无业务语义的 `reverses_id`。

表 `sales.sales_order_changes`。列为 `sales_order_id uuid not null`、`from_version_no int not null`、`to_version_no int`、`status text not null CHECK in DRAFT, PENDING_APPROVAL, APPROVED, REJECTED, WITHDRAWN`、`reason text not null`、`flow_instance_id uuid`、`requires_recheck boolean not null default false`、`recheck_validation_id uuid`、`applied_at timestamptz`。追加索引 `ix_sales_order_changes_sales_order_id_status`。

表 `sales.sales_order_change_lines`。列为 `sales_order_change_id uuid not null`（外键）、`sales_order_line_id uuid`、`operation text not null CHECK in ADD, MODIFY, CLOSE`、`new_quantity numeric(18,6)`、`new_delivery_date date`、`new_warehouse_id uuid`、`new_net_unit_price numeric(18,6)`、`source_contract_line_id uuid`。

表 `sales.sales_returns`，单据类。列为 `doc_no text`、`status text CHECK in DRAFT, SUBMITTED, REGISTERED, CLOSED, CANCELLED`、`customer_id uuid not null`、`sales_order_id uuid not null`、`return_reason text not null`、`return_warehouse_id uuid`、`posting_date date not null`（记账日期，取值与用途见基线第 3.4 节）、`is_drop_ship boolean not null default false`、可空整体来源 `source_module text/source_doc_type text/source_doc_id uuid/source_doc_line_id uuid`、`registered_at timestamptz`、`voucher_id uuid`（非零会计效果时由凭证腿同步回填，`(legal_entity_id,voucher_id)` 真实复合外键指向 `ledger.vouchers(legal_entity_id,id)`）、`flow_instance_id uuid`、`remark text`。约束 `ux_sales_returns_legal_entity_id_doc_no`、候选键 `UNIQUE(legal_entity_id,sales_order_id,id)`、`ck_sales_returns_registration_shape` 与 `ck_sales_returns_source_ref_shape`；前者固定为未登记三态 `registered_at/voucher_id` 全空、REGISTERED/CLOSED 的 `registered_at` 非空且 `voucher_id` 交给下述延迟图约束判定，来源形状强制四列全空或全非空，`source_doc_type` 最长 64。`ux_sales_returns_le_source_ref` 是不带 `WHERE` 的普通 `UNIQUE(legal_entity_id,source_module,source_doc_type,source_doc_id,source_doc_line_id)`；PostgreSQL 默认 `NULLS DISTINCT` 允许多张无来源退货，而任一完整来源只能出现一次，不得用部分唯一索引改写该语义。另有索引 `ix_sales_returns_legal_entity_id_customer_id_status`、`ix_sales_returns_sales_order_id`、`ix_sales_returns_legal_entity_id_posting_date`。同一来源重放锁定并返回既有完整视图，命令摘要不同返回 `PLATFORM.IDEMPOTENCY.PAYLOAD_MISMATCH`。CANCELLED 只可来自 DRAFT/SUBMITTED，因而两字段恒空；已 REGISTERED 的退货已存在追加型库存与会计事实，只能进入 CLOSED，首版没有取消或冲正入口。只有完整会计效果全零的 REGISTERED/CLOSED 退货可保持 voucher 空；非零退货必须在登记事务一次写入凭证，不允许先登记状态再补凭证。

表 `sales.sales_return_lines`。列为 `sales_return_id uuid not null`、`sales_order_id uuid not null`（冗余祖先键，客户端不可写）、`line_no int not null`、`sales_order_line_id uuid not null`、`item_kind text`、`item_id uuid not null`、`costing_mode text not null CHECK in (INVENTORY, DIRECT_EXPENSE)`（自原销售订单行冻结，退货时不得重取）、`inventory_material_id uuid`（自原销售订单行冻结）、`uom_code text not null`、`quantity numeric(18,6) not null`、`net_unit_price numeric(18,6) not null`、`is_tax_included boolean not null`、`tax_rate numeric(9,6) not null`、`line_amount numeric(18,2) not null`、`line_amount_with_tax numeric(18,2) not null`、`warehouse_id uuid`、`batch_no text not null default '-'`、`serial_nos text[] not null default '{}'`、`inventory_return_amount numeric(18,2)`（非直运 INVENTORY 行登记时由库存腿同步回填）、`stock_movement_id uuid`（`(legal_entity_id,stock_movement_id)` 真实复合外键指向 `inventory.stock_movements(legal_entity_id,id)`，同一退货单各库存行取本次 movement id）。候选键为 `UNIQUE(legal_entity_id,sales_return_id,id)` 与 `UNIQUE(legal_entity_id,sales_return_id,sales_order_id,sales_order_line_id,id)`；两条 `ON DELETE RESTRICT` 长复合外键分别为 `(legal_entity_id,sales_order_id,sales_return_id) -> sales_returns(legal_entity_id,sales_order_id,id)`、`(legal_entity_id,sales_order_id,sales_order_line_id) -> sales_order_lines(legal_entity_id,sales_order_id,id)`。另有 `ux_sales_return_lines_return_line_no` 与 `ck_sales_return_lines_direct_no_inventory`；候选键同时供阶段 12 工单登记行保证退货头行同属一单。INVENTORY 行物料快照必填，DIRECT_EXPENSE 行物料快照为空且只走收入冲回；`is_tax_included` 必须逐值等于原订单行，使退货完整保留来源定价快照。批次列取固定值 `'-'` 而非 NULL，按基线第 11.4 节；跨头状态与过账结果形状统一由下述延迟图触发器兜底，不得只靠 `register_sales_return` 应用校验。

表 `sales.return_line_delivery_links`，多对多，承载退货明细行与交付确认单的关联及按原交付实际金额冻结的累计区间。列为 `sales_return_id uuid not null`、`sales_order_id uuid not null`、`sales_order_line_id uuid not null`（三项均为客户端不可写的冗余祖先键）、`sales_return_line_id uuid not null`、`delivery_confirmation_id uuid not null`、`delivery_confirmation_line_id uuid not null`、`quantity numeric(18,6) not null CHECK (quantity>0)`、`assigned_by text not null CHECK in MANUAL, AUTO_FIFO`，以及登记前均为空、登记事务一次写入的 `allocation_quantity_before numeric(18,6) null`、`revenue_amount numeric(18,2) null`、`gross_amount numeric(18,2) null`、`cost_amount numeric(18,2) null`。后三项是本链接按原交付确认行实际净收入、含税收入与 INVENTORY COGS 分得的金额，合法值可为 0，不按当前售价、订单当前价或当前移动平均价重算。

`V20261017093200__sales_create_return_line_delivery_links.sql` 建立两条 `ON DELETE RESTRICT` 长复合外键：`(legal_entity_id,sales_return_id,sales_order_id,sales_order_line_id,sales_return_line_id) -> sales_return_lines(legal_entity_id,sales_return_id,sales_order_id,sales_order_line_id,id)`，以及 `(legal_entity_id,sales_order_id,sales_order_line_id,delivery_confirmation_id,delivery_confirmation_line_id) -> delivery_confirmation_lines(legal_entity_id,sales_order_id,sales_order_line_id,delivery_confirmation_id,id)`。候选键增加 `UNIQUE(legal_entity_id,sales_return_id,sales_return_line_id,delivery_confirmation_line_id,id)`；约束 `ux_return_line_delivery_links_pair` 在 `(sales_return_line_id, delivery_confirmation_line_id)`；`ck_return_line_delivery_links_allocation_shape` 强制四项分配列同空同非空，非空时 `allocation_quantity_before>=0` 且三金额均 `>=0`、`gross_amount>=revenue_amount`；追加索引 `ix_return_line_delivery_links_delivery_confirmation_id`。

同一文件再建立仅追加表 `sales.return_line_capture_allocations`。除仅追加公共列外，业务列为 `sales_return_id uuid not null`、`sales_return_line_id uuid not null`、`delivery_confirmation_line_id uuid not null`、`return_line_delivery_link_id uuid not null`（四项为不可写祖先键），`side text not null CHECK (side IN ('REVENUE','COST'))`，`cost_role text null CHECK (cost_role IN ('MAIN_OPERATING_COST','DIRECT_EXPENSE_COST'))`，`revenue_root_entry_id/revenue_live_entry_id uuid null`、`cost_root_entry_id/cost_live_entry_id uuid null`，生成列 `root_entry_id uuid GENERATED ALWAYS AS (coalesce(revenue_root_entry_id,cost_root_entry_id)) STORED`、`live_entry_id uuid GENERATED ALWAYS AS (coalesce(revenue_live_entry_id,cost_live_entry_id)) STORED`，以及 `amount numeric(18,2) not null CHECK (amount>0)`。NULL-safe `ck_return_line_capture_allocations_side_shape` 强制 REVENUE 时只允许 revenue 根/live 对非空且 `cost_role IS NULL`，COST 时只允许 cost 根/live 对非空且 `cost_role IS NOT NULL`；不得用一个无真实外键的裸多态 id 代替两组实体列。候选键 `UNIQUE(legal_entity_id,id)`；长复合 FK `(legal_entity_id,sales_return_id,sales_return_line_id,delivery_confirmation_line_id,return_line_delivery_link_id) -> return_line_delivery_links(legal_entity_id,sales_return_id,sales_return_line_id,delivery_confirmation_line_id,id) ON DELETE RESTRICT`；唯一键 `ux_return_line_capture_allocations_link_side_live` 在 `(legal_entity_id,return_line_delivery_link_id,side,live_entry_id)`，同一 link/side/live fragment 只能消费一次。`V20261017093630__sales_backfill_append_only_registry.sql` 将本表以 `mode=APPEND_ONLY,mutable_columns='{}'` 登记进 `platform_core.append_only_registry` 并按登记安装拒绝 UPDATE/DELETE 的守卫；回退先删守卫再删登记行，不删除业务表。

同一第 93200 号迁移建立 `sales.assert_sales_return_graph_consistent()`，并在 `sales_returns`、`sales_return_lines`、`return_line_delivery_links`、`return_line_capture_allocations` 上各装一个 `DEFERRABLE INITIALLY DEFERRED` 约束触发器，函数在迁移末尾才启用。提交时它按 `delivery_confirmation_line_id` 稳定顺序锁读完整图并强制：退货头客户等于订单客户，`is_drop_ship=(sales_orders.order_type='DROP_SHIP')`，非直运 `return_warehouse_id` 非空；行的产品、计价模式、物料、单位、净价、`is_tax_included` 与税率逐值等于同一订单行，行仓库等于退货头仓库；链接父交付必须为 CONFIRMED，且上述长键证明退货行与交付行属于同一订单、同一订单行。

REGISTERED/CLOSED 至少一条退货行，每行链接数量合计精确等于退货数量；每条原交付行的已登记链接区间 `[allocation_quantity_before,allocation_quantity_before+quantity)` 必须从 0 起连续、无重叠无空洞且末端等于该行已登记退货累计，不得超过原交付数量。对每个 link 和原交付行金额 `M`、原交付数量 `Q`、区间起点 `B`、本次数量 `q`，三额分别强制为 `round(M*(B+q)/Q,2)-round(M*B/Q,2)`，其中 `M` 依次取原行 `line_amount`、`line_amount_with_tax`、`coalesce(cogs_amount,0)`，舍入为 PostgreSQL numeric MidpointAwayFromZero；因此最后覆盖全量的区间自动吸收尾差，而历史区间无需重算。每条退货行 `line_amount=SUM(link.revenue_amount)`、`line_amount_with_tax=SUM(link.gross_amount)`、`coalesce(inventory_return_amount,0)=SUM(link.cost_amount)`。DRAFT/SUBMITTED/CANCELLED 的 link 四项分配列全空且无 capture allocation；REGISTERED/CLOSED 四项全非空，link 的 REVENUE/COST fragment 金额分别精确汇总为 `revenue_amount/cost_amount`，相应金额为零时该 side 必须零 fragment。REGISTERED 后 link 的数量、区间、三金额和全部 capture allocation 永久不可改删。

DRAFT/SUBMITTED/CANCELLED 的所有行 `inventory_return_amount/stock_movement_id` 全空；REGISTERED/CLOSED 的非直运 INVENTORY 行两项全非空且金额非负，DIRECT_EXPENSE 或直运行两项全空。库存结果非空时，所有行指向同一条同法人 `inventory.stock_movements`，且该 movement 的 `direction/reason/source_doc_type/source_module/source_doc_id` 恰为 `IN/SALES_RETURN/SALES_RETURN/sales/sales_returns.id`；其数量与金额明细的 `source_doc_line_id`、物料、仓库、批次、序列、数量和金额必须一一对应本退货行及 links 的原交付实际成本，不允许只命中同一 movement 的另一段或按当前移动平均价替换。REGISTERED/CLOSED 的 `voucher_id IS NULL` 当且仅当全部行 `line_amount=0 AND line_amount_with_tax=0 AND coalesce(inventory_return_amount,0)=0`；任一会计效果非零时凭证必填，全部效果为零时凭证必须为空；非空凭证必须是同法人、`source_kind=SALES_RETURN`、`source_document_type='SALES_RETURN'`、`source_document_id=sales_returns.id` 的本次普通凭证。回退先删四个触发器与函数，再删 capture allocation 与 link 表。该数据库闸门与应用锁序共同承担并发上限，普通外键全部能命中但跨订单拼接、错退货头、错交付行、错累计区间/金额、错库存段、半回填或错误 voucher 的图均在提交时整笔失败。

Stage 11 成本与收入表建成后，`db/migrations/sales/V20261020090130__sales_add_costing_capture_foreign_keys.sql` 才启用 REGISTERED 写入口。该追补先为两组现存 id 做同法人、根/live 归属与孤儿预检，再建立 `(legal_entity_id,revenue_root_entry_id,revenue_live_entry_id) -> costing.revenue_entries(legal_entity_id,root_entry_id,id)` 与成本侧同形的两条 `ON DELETE RESTRICT DEFERRABLE INITIALLY DEFERRED` 长复合外键；随后 `CREATE OR REPLACE sales.assert_sales_return_graph_consistent()`，既有 sales 四表触发器自动调用新函数，只需在 `costing.cost_entries/revenue_entries` 两表各新增一个补充约束触发器以覆盖反向写入。提交点逐 fragment 证明 live entry 与 root 同链、当前符号为正且有足够开放额，root 的 `source_document_type='DELIVERY_CONFIRMATION'`、头/行恰等于 link 原交付，REVENUE 属于收入侧；COST 的 `cost_role` 必须逐值等于 live entry 锁后权威角色，且只允许 `MAIN_OPERATING_COST|DIRECT_EXPENSE_COST`。图函数对每个 link/side 锁读同一原交付的全部候选 leaf，以“提交后开放额 + 本退货在该 leaf 的反向额”还原锁前 `A_i`，再按第 4.12 节的整数分 largest-remainder 公式重算；allocation 的 leaf 集合、每片金额及 role 必须逐值相等。同一事务生成的 SALES_RETURN 反向 capture 必须以该 live entry 为直接父、来源头行为当前退货、绝对金额逐 fragment 相等，并按 `cost_role` 落到匹配的静态成本计量腿；错 side、错 role、错 measure、错维度、FIFO/任意分摊或同额但挂到另一 live leaf 均在提交时拒绝。受控更正后的一个交付根可有多个 current live fragment，故不得退化成 link 上单一 capture id。追补回退先删除 costing 两表的补充触发器，恢复第 93200 号旧函数体（sales 原四表触发器始终保留），再删两条长复合 FK；回退后 REGISTERED 写入口必须随之关闭。

表 `sales.exchange_links`。列为 `sales_return_line_id uuid not null`（同 schema 外键指向 `sales.sales_return_lines`，`ON DELETE RESTRICT`）、`replacement_delivery_schedule_id uuid not null`（同 schema 外键指向 `sales.delivery_schedules`，`ON DELETE RESTRICT`）、`linked_at timestamptz not null`、`linked_by uuid not null`。一条退货行与一条替换分批交付行构成一对一配对：`ux_exchange_links_return_line` 建于 `(legal_entity_id, sales_return_line_id)`，`ux_exchange_links_replacement_schedule` 建于 `(legal_entity_id, replacement_delivery_schedule_id)`；不保留只约束 pair、却允许任一侧重复配对的 `ux_exchange_links_pair`。本表不设状态和删除动作；任一侧业务终态由各自单据维护，取消后的历史配对仍保留并由调用方按当前状态判定，不以删除关联掩盖历史。

表 `sales.order_validations` 与 `sales.order_validation_items`，列结构与 `clm.contract_validations` 及其明细完全同构，只把 `contract_id` 换为 `sales_order_id`，`occasion` 取 `CHECK in RELEASE, CHANGE_SUBMIT, CHANGE_APPROVE`。两处同构而分表的理由见第 11.2 小节。

视图 `sales.v_credit_exposure_in_transit`：按 `(legal_entity_id, customer_id)` 汇总 `sales.sales_order_lines` 中订单状态属于 RELEASED、CHANGE_APPROVAL、PARTIALLY_DELIVERED 且行状态属于 OPEN、PARTIALLY_DELIVERED 的 `open_amount_with_tax` 合计。待放行订单不计入，与 PRD 3.14.2 末句一致。

`db/migrations/sales/V20261023092800__sales_harden_order_delivery_economic_graph.sql` 建立 `sales.assert_sales_order_economic_graph_consistent()`，并以 `DEFERRABLE INITIALLY DEFERRED` 约束触发器覆盖 `sales_orders`、`sales_order_lines`、`delivery_schedules`、`delivery_confirmations` 与 `delivery_confirmation_lines` 的 INSERT/UPDATE/DELETE；它同时 `CREATE OR REPLACE` 上述交付图函数，使两函数共用稳定锁序与同一累计金额函数。提交点先以订单头的三列来源复合 FK 锁定 `clm.contract_versions`：每条订单行的 source contract/version 必须等于头，`source_contract_line_id` 必须在该版本快照的 lines 数组中恰出现一次，且 item/costing/material/uom/quantity/net_unit_price/is_tax_included/tax_rate/两项行额/交期/仓库逐字段等于该快照；订单 customer、order_type/cycle/lease/auto_renew 与合计也必须等于来源版本，头两项金额再等于订单行和。订单至少一行，每行至少一条 delivery schedule，且 `SUM(schedule.quantity)=line.quantity`。

同一图把所有确认事实反向纳入：每个 schedule 与订单行的 `delivered_quantity` 分别等于指向它的 CONFIRMED delivery line 数量和；非终态订单行若已交付量为 0/部分/全量，状态分别为 OPEN/PARTIALLY_DELIVERED/DELIVERED，CLOSED/CANCELLED 可保留历史交付量但 `open_amount_with_tax` 固定为 0。其他行的 `open_amount_with_tax` 唯一等于 `line_amount_with_tax - SUM(CONFIRMED delivery_line.line_amount_with_tax)`，不得再按剩余数量乘单价重算；全量交付因此精确归零。每个已确认交付区间及金额按上一段 B/q 累计差分公式闭合，`delivery_schedules` 或确认事实任一侧反向更新都会触发同一检查。普通 FK 全命中但错合同版本、混合订单类型、伪造头合计、分批量不守恒、累计交付不等、区间空洞/重叠、税内价重复乘税、分段尾差错误或伪造 open amount 均在 COMMIT 被拒。回退先删五表触发器，再恢复第 92800 号初始交付函数体并删经济图函数，不改业务数据。
按裁定 A-18，第 V20261017093600 号迁移 `V20261017093600__sales_create_dataset_views.sql` 建立本模块的两个受治理数据集视图。`sales.v_sales_orders_dataset` 的 dataset code 为 `sales_sales_orders`，grain 取 DOCUMENT，取数为 `sales.sales_orders`；`sales.v_order_delivery_batches` 的 dataset code 为 `sales_order_delivery_batches`，grain 取 DOCUMENT_LINE，取数为 `sales.delivery_schedules`，输出列含 `sales_order_id`、`sales_order_line_id`、`batch_no`、`quantity`、`delivered_quantity`、`promised_date`、`warehouse_id`、`status`。逐次交付追溯另从 `sales.delivery_confirmation_lines` 读取，不在一对多事实之上伪造单值确认引用。两个视图都必须含 `legal_entity_id`、`security_level`、`data_scope_tags` 三列，都不做聚合、不跨 schema 连接，并在同一迁移内执行 `GRANT SELECT ON <视图> TO ep_analyst_ro`，不授予 `ep_app_rw` 之外的任何写权限。列名与类型签名必须与阶段 11 的 `reporting.dataset_fields` 登记一致，由阶段 11 的启动自检项 `reporting-dataset-signature-matched` 校验。该文件头的 `-- rollback:` 段为 `drop view` 与对应的 `revoke`。

#### 3.5 数据库角色与迁移账号

本阶段不新增数据库角色。三个 schema 的属主分别为 `ep_mod_cpq`、`ep_mod_clm`、`ep_mod_sales`，迁移由 `ep_migrator` 在迁移窗口执行，运行期由 `ep_app_rw` 读写，只读分析由 `ep_analyst_ro` 访问。迁移会话固定 `SET lock_timeout = '5s'` 与 `SET statement_timeout = '30min'`。

---

### 4. 领域模型与关键算法

本阶段不直接写总账、库存数量账或库存金额账，而是只经所属模块的公开契约在调用方原事务内同步完成。交付确认在 `confirm_delivery` 内完成库存出库、凭证与未开票应收三腿；销售退货在 `register_sales_return` 内完成库存入库（仅非直运 INVENTORY 行）、凭证与未开票应收三腿。凭证与库存账由被调方写入，本阶段传入来源单据、取价意图和计量项并同步回填结果；两类事件都只在全部业务后果成功后写 Outbox，绝不作为补写库存或财务事实的触发器。分录、取价、回冲单价与税额分支一律按规格第 5.2 章财务规则条目的事件-分录表及其规则块，由财务模块与库存模块承接，本节不另造规则。

#### 4.1 核心结构体与枚举

`ep-domain-clm` 的聚合根是 `Contract`，聚合内含 `ContractLine`、`ContractTerms`、`Milestone`、`PaymentSchedule`、`AttachmentRef` 五类子实体，以及 `ContractVersionSnapshot` 值对象。签署编排是独立聚合 `SignatureRequest`，派生批次是独立聚合 `DerivationBatch`，两者与 `Contract` 之间只有标识引用，理由是三者的生命周期与事务边界不同，放在同一聚合会把一个用例撑成多个写事务。

`ep-domain-sales` 的聚合根是 `SalesOrder`，聚合内含 `SalesOrderLine` 与 `DeliverySchedule`。`SalesReturn` 是独立聚合，含 `SalesReturnLine` 与 `DeliveryLink`。`OrderChange` 是独立聚合。

关键值对象：`Money`、`UnitPrice`、`Quantity`、`Rate` 直接取自 `ep-foundation`；本阶段新增 `DiscountRate`（0 至 1 的 Rate 收窄）、`CreditExposure`（三桶取值与合计）、`OrderTypeMark`（五取值枚举与其字段约束）、`DerivationKey`（合同、版本、派生物类型、来源行四元组）。

枚举一律与第 3 节的 CHECK 取值逐字一致，由 `ep-contract-*` 中的 `serde` 派生类型作为唯一定义处，数据库 CHECK 由迁移生成器从该类型导出，避免两处漂移。

#### 4.2 合同状态机

状态与流转严格按 PRD 3.6，守卫条件如下。

| 起点 | 终点 | 触发 | 守卫条件 |
|---|---|---|---|
| DRAFT | PENDING_APPROVAL | 提交审批 | 五项校验的阻断项全部通过；三类信息齐备；至少一条合同行；收付款期次合计校验通过；乐观锁版本匹配 |
| DRAFT | VOID | 作废 | 无派生记录 |
| PENDING_APPROVAL | PENDING_SIGNATURE | 全部审批链通过 | 四条链中已触发的链全部结论为通过，且管理层节点已通过；未触发折扣的合同不要求折扣链 |
| PENDING_APPROVAL | REJECTED | 任一节点驳回 | 无 |
| PENDING_APPROVAL | DRAFT | 退回修改 | 保留既有审批记录，版本号不变 |
| REJECTED | DRAFT | 重新编辑 | 由发起人执行 |
| REJECTED | VOID | 放弃 | 无派生记录 |
| PENDING_SIGNATURE | EFFECTIVE | 生效动作 | 签署方式为 ESIGN 时 signature_requests.status 为 SIGNED 且 verify_result 为 PASSED；签署方式为 PHYSICAL_SEAL 时存在 seal_usages 记录且附有扫描件；重新认证凭证有效且绑定本次待签内容摘要；生效审批实例结论为通过且审批人不等于发起人 |
| PENDING_SIGNATURE | REJECTED | 签署被拒或用印被否决 | 无 |
| EFFECTIVE | IN_PERFORMANCE | 派生完成 | contract_derivations.status 为 DONE 且 item_done 等于 item_total |
| EFFECTIVE | EFFECTIVE | 派生失败 | 状态不变，derivation_state 置 FAILED，写死信并在界面显示待人工修复 |
| IN_PERFORMANCE | COMPLETED | 履约完成 | 全部交付节点为 CONFIRMED 或 CANCELLED，且全部收付款期次经财务契约判定为已结清 |
| IN_PERFORMANCE | TERMINATING | 提前终止审批通过 | `chain_kind=TERMINATION` 结论通过、审批人不等于发起人、终止原因非空、乐观锁版本匹配 |
| EFFECTIVE | TERMINATING | 派生失败后终止审批通过 | 上一行全部守卫，且 `derivation_state=FAILED`；RUNNING 时拒绝 |
| TERMINATING | TERMINATED | 系统闭合 | 该合同 `impact_assessments.status='DONE'` 且 `item_done=item_total` |
| TERMINATING | TERMINATING | 处置失败自环 | 存在 DEAD 项时批次置 FAILED、合同保持 TERMINATING，界面显示待人工修复；修复重放后继续推进 |

合同有效期止日届满不改变状态，只作为提醒触发源，由 `clm.v_contract_reminder_sources` 承载。已生效合同的修订不在原版本上改写，一律经 `actions/amend` 生成 `version_no + 1` 的新草稿版本并重走审批与生效链路。

#### 4.2.1 F-10 阶段 6 三条影响面规则

本阶段实现且真实注册三条 `ep_platform_impact::ImpactRule`：`ContractTerminationSalesOrderLineImpactRule` 位于 `crates/application/sales/src/impact/contract_termination_sales_order_line.rs`，`ContractTerminationMilestoneImpactRule` 位于 `crates/application/clm/src/impact/contract_termination_milestone.rs`，`ContractTerminationDeliveryConfirmationImpactRule` 位于 `crates/application/sales/src/impact/contract_termination_delivery_confirmation.rs`。三者的 `upstream_event_type()` 都固定返回 `clm.contract.terminated.v1`，`code()` 分别返回 `CLM_TERM_SALES_ORDER_LINE`、`CLM_TERM_MILESTONE`、`CLM_TERM_DELIVERY_CONFIRMATION`，取数与写入只经本模块仓储。`assess` 和 `dispose` 都复用调用方的 `&mut dyn ep_foundation::port::Tx`、`SecurityContext` 与法人 RLS；处置幂等键固定为处置项 id，规则不自建消费者、待办或重试队列。

三条规则的返回型唯一为 `ImpactDisposeOutcome::{Completed { reason }, AlreadySatisfied { reason }, NeedsManualDecision { reason }}`，`reason` 为下文列出的非空稳定码，不接受调用方传入科目、方向、金额或任意角色。`NeedsManualDecision` 不是失败：平台在同一事务把项改为 `MANUAL_DECISION`、保持 `PENDING`，按 `target_module`的固定管理者映射建立或复用 `HUMAN_TASK` 并回填 `process_task_id`；它不增加 attempts、不退避、不进死信。

1. `CLM_TERM_SALES_ORDER_LINE`。`assess` 按订单行 id 升序取该合同派生且 `status in ('OPEN','PARTIALLY_DELIVERED')` 的订单行；`delivered_quantity=0` 产出 `AUTO_CANCEL`，否则产出 `AUTO_CLOSE`。`dispose` 先对 INVENTORY 非直运行取第 4.3.1 节同一库存组合 advisory transaction lock，再按分批交付行 id、订单行 id、订单头 id 的固定次序 `FOR UPDATE` 并重读当前值。当前行已为 DELIVERED/CLOSED/CANCELLED 时零写入返回 `AlreadySatisfied { reason: "SALES_ORDER_LINE_ALREADY_TERMINAL" }`；仍为 OPEN/PARTIALLY_DELIVERED 时以锁后 `delivered_quantity` 为唯一分支依据，零交付置 CANCELLED 并返回 `Completed { reason: "SALES_ORDER_LINE_AUTO_CANCELLED" }`，已有交付置 CLOSED、写 `close_reason="合同终止 <合同编号>"` 并返回 `Completed { reason: "SALES_ORDER_LINE_AUTO_CLOSED" }`。两个分支都在同一事务取消该行下所有 PENDING 分批交付行、把 `inventory_demand_state` 置 INACTIVE、写一次审计并按唯一可用量实现重算。本次后订单所有行均终态时，锁后全订单交付量合计为零则头置 CANCELLED 并只发 `sales.sales_order.cancelled.v1` 一次，否则头置 CLOSED 并只发 `sales.sales_order.closed.v1` 一次。锁后从零交付变为有交付时直接执行当前正确的 AUTO_CLOSE 分支，不使用 assess 时的旧数量。
2. `CLM_TERM_MILESTONE`。`assess` 按节点 id 升序取该合同下 `status in ('PLANNED','ACTIVE')` 的交付节点，一律产出 `AUTO_CANCEL`。`dispose` 对目标节点 `FOR UPDATE` 后重检法人、合同归属与状态；仍为 PLANNED/ACTIVE 时置 CANCELLED、写一次审计并返回 `Completed { reason: "CONTRACT_MILESTONE_AUTO_CANCELLED" }`，已为 CONFIRMED/CANCELLED 时零写入返回 `AlreadySatisfied { reason: "CONTRACT_MILESTONE_ALREADY_TERMINAL" }`。目标不存在、异法人或异合同不得伪装成已满足，而是返回受控错误进入平台重试/死信链。
3. `CLM_TERM_DELIVERY_CONFIRMATION`。`assess` 按交付确认单 id 升序取该合同名下 `status='CONFIRMED'` 的单据，但若其每一行的确认数量已被状态为 REGISTERED/CLOSED 的销售退货行通过 `sales.return_line_delivery_links.quantity` 全额覆盖，则不命中；其余一律产出 `MANUAL_DECISION`。平台接收的命令形状固定为 `ManualImpactDecision { decision_code, decision_reason, decision_result_doc_id }`，本规则只允许 `RETURN_REGISTERED` 与 `NO_RETURN` 两个 code，两者都要求清洗后 `decision_reason` 非空，禁止解析理由文本猜分支。`RETURN_REGISTERED` 要求 `decision_result_doc_id` 非空；规则在同一事务锁定交付确认单、该 id 对应的销售退货单及其关联行，验证同法人、退货单 `status in ('REGISTERED','CLOSED')` 且存在指向本交付确认单的 `return_line_delivery_links`，通过后返回 `Completed { reason: "DELIVERY_CONFIRMATION_RETURN_REGISTERED" }`。`NO_RETURN` 必须 `decision_result_doc_id=null`，重检目标仍是本合同的 CONFIRMED 单据后返回 `Completed { reason: "DELIVERY_CONFIRMATION_NO_RETURN" }`。错 code、空/异单结果 id、空理由、关联不成立或退货状态不符一律拒绝并保持 PENDING，不改任何财务、库存或交付确认事实。若提交人工决策前该确认单已被合法退货全额覆盖，锁后复核返回 `AlreadySatisfied { reason: "DELIVERY_CONFIRMATION_ALREADY_RETURNED" }`。

三条 `assess` 均过滤异法人、异合同和不合格终态；返回空时平台按目录把该 code 的单个占位项置 DONE 并写固定理由“无适用对象”，有目标时则按一对象一项展开。阶段 6 结束时 `ImpactRegistry` 真实注册数恰为 3；目录仍恰为 7，另四个未注册 code 的占位项必须保持 PENDING、`target_doc_id/target_doc_no` 为空、不计入 `item_done`，因而合同保持 TERMINATING。这些占位项是目录事实而非规则替身；不得提前置 DONE、伪造单号或注入 Noop。

来源闭合端口唯一实现为 `ContractTerminationCompletionPort`，实现阶段 3 的 `ep_platform_impact::ImpactSourceCompletionPort`，注册键固定为 `(ModuleCode::Clm,"clm.contract.terminated.v1")`。`complete(tx,ctx,source,assessment_id,item_total)` 必须复用调用方事务：按合同 id `FOR UPDATE`，复核同法人、source version、合同仍为 TERMINATING、该 assessment 确属该合同且平台已确认全部项 DONE/无 DEAD；随后置 TERMINATED、递增 row_version，先只发布一次 `clm.contract.termination_completed.v1` 到 Outbox，再由事务所有者把审计放入最终终结批。重复调用同一 assessment 时返回已满足且不得重复事件；异合同、异法人、旧版本或非 TERMINATING 均受控失败，不能把批次单独置 DONE。平台本体、三个规则或 completion port 任一缺失、重复或为替身时，`impact-registry-consistent` 在启动或模块启用提交前失败关闭，模块启用回滚并返回 `PLATFORM.IMPACT_COMPLETION_PORT.NOT_REGISTERED`；不得退化为平台直写 clm、异步补终态或 Noop 成功。

#### 4.3 订单与分批交付行状态机

订单状态按 PRD 3.11.5，守卫条件的要点为：PENDING_RELEASE 到 RELEASED 要求信用与库存两项重跑通过，或信用审批结论为通过；库存通过只代表可把 INVENTORY 非直运行的 `inventory_demand_state` 从 INACTIVE 置为 CONFIRMED，订单头仍可因信用审批停在 PENDING_RELEASE；真正放行时把这些行置为 RELEASED。RELEASED 到 CANCELLED 要求全部行 `delivered_quantity` 为零；PARTIALLY_DELIVERED 到 CLOSED 要求关闭原因非空并写入审计；CHANGE_APPROVAL 的进入与退出由 `sales.sales_order_changes` 驱动，退出时回到进入变更前的状态。DELIVERED、CLOSED、CANCELLED 为终态，终态行的库存需求状态必须为 INACTIVE，开票与到款不改变订单状态。

分批交付行状态为 PENDING、DELIVERED、CLOSED、CANCELLED 四取值。逾期不是状态，由 `promised_date` 与当前服务器自然日派生，自然日取值一律用 `(now() AT TIME ZONE 'Asia/Shanghai')::date`。

#### 4.3.1 U-G-01 销售未交付需求与库存可用量

`ep-contract-sales` 在 `src/port/open_sales_demand.rs` 定义并由 `ep-app-sales` 的 `ConfirmedOpenSalesDemandQueryImpl` 真实实现唯一销售需求查询；签名与 DTO 固定如下，不另建 reserved 表、物化投影或第二个同义 trait。

```rust
pub struct ConfirmedOpenSalesDemandFilter {
    pub legal_entity_id: Id<LegalEntity>,
    pub material_id: Id<Material>,
    pub warehouse_id: Option<Id<Warehouse>>,
}
pub struct ConfirmedOpenSalesDemandView {
    pub legal_entity_id: Id<LegalEntity>,
    pub warehouse_id: Id<Warehouse>,
    pub material_id: Id<Material>,
    pub open_quantity: Quantity,
}

#[async_trait::async_trait]
pub trait ConfirmedOpenSalesDemandQuery: Send + Sync {
    async fn summarize(
        &self,
        tx: &mut dyn ep_foundation::port::Tx,
        ctx: &SecurityContext,
        filter: ConfirmedOpenSalesDemandFilter,
    ) -> Result<Vec<ConfirmedOpenSalesDemandView>, AppError>;
}
```

查询只扫 `sales.sales_order_lines`，固定以 `open_inventory_demand_slot=true` 命中普通索引，并按 `(legal_entity_id, warehouse_id, inventory_material_id)` 汇总 `sum(quantity - delivered_quantity)`；槽位生成表达式逐项等价于 `inventory_demand_state in ('CONFIRMED','RELEASED')`、`status in ('OPEN','PARTIALLY_DELIVERED')`、`costing_mode='INVENTORY'`、`is_drop_ship=false`，业务 CHECK 另保证仓库与物料快照非空，结果不得为负。`CONFIRMED` 表示库存校验已在锁内通过但订单头仍可能等待信用审批，`RELEASED` 表示订单已下达；PENDING_RELEASE 本身既不自动纳入也不自动排除，唯一判据是该行的 `inventory_demand_state`。查询命中第 3.4 节的 `ix_sales_order_lines_open_inventory_demand`，不得跨 schema 直读库存表。

`SalesAwareAvailabilityQuery` 由本阶段在 `crates/application/sales/src/query/availability.rs` 实现阶段 6 同批追加的 `ep_contract_inventory::AvailabilityQueryPort`：在调用方同一个 `&mut dyn Tx` 上先经阶段 8 的 `StockOnHandQueryPort::on_hand_by_warehouse` 取结存，再经上述 `ConfirmedOpenSalesDemandQuery::summarize` 取销售需求，按仓库左连接后固定计算 `reserved_quantity = open_quantity`、`available_quantity = on_hand_quantity - reserved_quantity`。销售建单、订单确认守卫、阶段 8 的 A2 路由与下述补货组合全部注入这个实现；任何只返回 on_hand、把 reserved 固定为零或另写一套 SQL 的实现都禁止。传入 `warehouse_id=Some(id)` 时必须恰好返回该仓一行，即使库存余额与销售需求都不存在也返回三项数量均为零，不能用空集合表达零结存。A2 的 `quantity` 即 on_hand，`reserved_quantity` 允许大于 quantity，因此 `available_quantity` 可为负并如实展示；负值不等于允许负结存，实际出库仍受阶段 8 的结存硬阻断。

并发守卫的单事务算法固定如下。

1. 把本次可能增减需求的旧组合与新组合取并集，按 `(legal_entity_id UUID bytes, warehouse_id UUID bytes, material_id UUID bytes)` 升序排列。`SalesDemandRepository::lock_availability_keys` 由 `ep-adapter-db-pg` 实现，在当前事务逐键执行 `pg_advisory_xact_lock(hashtextextended('sales-availability:' || legal_entity_id || ':' || warehouse_id || ':' || material_id, 0))`；哈希碰撞只会额外串行化，不会削弱正确性。锁超时沿用 3 秒并返回 `PLATFORM.DB.LOCK_TIMEOUT`。
2. 锁齐后调用同一个 `SalesAwareAvailabilityQuery` 重算当前可用量。订单首次确认的 `requested` 是待转 CONFIRMED 行按组合汇总的全部未交付量；从 INACTIVE 直接下达时同义；已 CONFIRMED 再转 RELEASED 的增量为零；订单变更只比较新需求减旧有效需求的正增量，仓库或物料移动同时在旧组合释放、在新组合申请。
3. 任一组合 `available_quantity < requested` 时返回 `SALES.SALES_ORDER.STOCK_NOT_AVAILABLE`，details 固定含法人、仓库、物料、available_quantity、requested，显式确认、下达与变更审批动作零写入。合同提交与合同生效派生仍按 U-E-08 的既有处置只记录失败并把订单留在 PENDING_RELEASE、相关行留在 INACTIVE，不把该业务结果伪装成成功确认。
4. 足够时在同一事务把确认行置 CONFIRMED、下达行置 RELEASED；不改变库存的取消、关闭仍用本小节直接组合锁。实际交付按第 4.11 小节把同一 `sales-availability:` key 放入 F-50 InventoryAvailability 类别，并连同库存来源/余额/state 一次取齐，不得先经本仓储取得组合锁再补其他类别；proof 后才增加 `delivered_quantity` 与写库存，随后用同一组合实现重算并把结果写入审计 after。实际交付减少 on_hand 与 open_quantity 的数量相同，必须在 `confirm_delivery` 原事务完成，因此可用量保持守恒；禁止用异步 `sales.delivery_writeback` 延后释放需求。

首版不建库存预留表，不维护 `reserved_quantity` 列；`inventory_demand_state` 只是订单行业务状态，权威数量始终由现行订单行动态聚合。所有确认、下达、取消、关闭、交付与改变数量/仓库/物料的批准变更都必须走本小节锁与重算函数，绕过即架构检查失败。

#### 4.3.2 U-F-02 销售感知补货策略组合

`SalesAwareReplenishmentPolicyQuery` 固定放在 `crates/application/sales/src/query/replenishment_policy.rs`，实现阶段 6 同批追加的 `ep_contract_inventory::ReplenishmentPolicyQuery`。它只组合阶段 8 的 `ReplenishmentPolicyReadPort` 与上一小节同一个 `SalesAwareAvailabilityQuery`；不访问 inventory 表或 procure 表，不复制阈值，也不实现第二套可用量 SQL。`list_for_scan` 的 `limit` 只允许 1 至 500，越界返回 `PLATFORM.REQUEST.INVALID_PAYLOAD`；结果按 `(warehouse_id UUID bytes, material_id UUID bytes)` 严格升序且最多 500 行，只含两阈值均非空的启用策略。`available_qty` 逐行取同事务内 `SalesAwareAvailabilityQuery::available(warehouse_id=Some(key.warehouse_id), material_id=key.material_id)` 返回的唯一行之 `available_quantity`，所以公式仍唯一为 `on_hand - CONFIRMED/RELEASED 未交付订单剩余量`。

分页与并发算法冻结如下，实施方不得另选。

1. 以请求的 `after` 为内部游标，经 `ReplenishmentPolicyReadPort::list_stored` 按每批最多 500 行枚举候选；停用行可在内部跳过，但必须把内部游标推进到该原始批次末键，不能因一批全是停用行而提前宣告扫描结束。
2. 每一原始批次先只读出候选键，不持有业务行锁；把其中启用候选按 `(legal_entity_id UUID bytes, warehouse_id UUID bytes, material_id UUID bytes)` 排序去重，逐键取得与第 4.3.1 节完全相同的 `sales-availability:` advisory transaction lock。取得全部组合锁后，用同一批次起始游标再次调用 `list_stored`，只采用第二次读到且阈值仍非空的当前版本；第一次枚举到、加锁前已停用或变更的行不使用旧值。A12 也遵循先 advisory lock、后策略行锁的同一锁序，因此第二次读取后不会与同组合配置更新交叉。
3. 在锁内逐策略调用同一 `SalesAwareAvailabilityQuery`，组装 `ReplenishmentPolicyScanView`；达到请求 `limit` 即以最后返回的策略键作为调用方下一页 `after`，不足则继续枚举后续原始批次，直至取得足量启用行或底层返回空页。新增键恰好落在已越过的游标之前时由下一轮 60 分钟扫描承接，不在本轮回退游标。
4. 两个进程分别按同一装配函数构造组合：`apps/core-server/src/wiring/` 用阶段 8 的真实 `StockOnHandQueryPort` 与本阶段真实 `ConfirmedOpenSalesDemandQueryImpl` 构造一个进程内共享的 `Arc<SalesAwareAvailabilityQuery>`，同时注入 A2 与订单守卫；`apps/job-worker/src/wiring/` 用相同两项真实依赖构造该组合，再与阶段 8 的真实 `ReplenishmentPolicyReadPort` 一并注入 `SalesAwareReplenishmentPolicyQuery`，后者以 `Arc<dyn ReplenishmentPolicyQuery>` 注入阶段 7 自动采购需求扫描。两进程使用同一实现类型与公式，不声称跨进程共享内存实例。任一真实依赖缺席时对应 A2/订单守卫或扫描用例与定时任务均不注册，不允许 Noop、空页、零值 provider 或回退到仅 on-hand。

阶段 7 是该组合查询的唯一业务消费者：它只依据返回的 `reorder_point`、`target_stock` 与 `available_qty` 判定 `available_qty <= reorder_point` 并计算 `max(target_stock - available_qty, 0)`，不得直接查询 `inventory.replenishment_policies`、sales 订单行或库存余额。A11/A12 的配置读写仍归阶段 8；本阶段不新增补货策略 HTTP 端点或物理表。

#### 4.4 取价与行金额算法

输入为法人、客户、产品或物料、计量单位、单据日期、数量、录入单价、折扣率与操作者的价格权限档案。

1. 经 `ep-contract-cpq::PriceListQueryPort` 取命中的价目行。命中判定的输入与筛选条件按 PRD 2.8.3，由主数据阶段实现，本阶段只消费。
2. 多行命中时返回 `CPQ.PRICE_AUTHORITY.MULTIPLE_PRICE_LIST_HITS`，携带全部命中行，要求操作者显式选择，不由系统任意取一行。
3. 无命中时不阻断，`list_unit_price` 与 `price_floor` 留空。
4. `net_unit_price = round6(unit_price * (1 - discount_rate))`，舍入策略为四舍五入且中值远离零。
5. 定义唯一累计函数。税外价时 `cum_net(x)=round2(x*net_unit_price)`、`cum_gross(x)=round2(cum_net(x)*(1+tax_rate))`；税内价时 `cum_net(x)=round2(x*net_unit_price/(1+tax_rate))`、`cum_gross(x)=round2(x*net_unit_price)`。整行固定为 `line_amount=cum_net(quantity)`、`line_amount_with_tax=cum_gross(quantity)`；部分交付或其他分段只能取 `cum(B+q)-cum(B)`，不得对每段独立重算 `round2(q*p)`。`tax_rate=-1` 非法且由税率闭集拒绝，故税内除法分母不为零。
6. 中间值在内存中以全精度 Decimal 保留，只在写库前一次性 round，按基线第 3.5 节。
7. 合同金额与订单金额由行金额按 2 位小数直接累加，不再二次舍入，因此头与行天然相等。

边界条件：数量为零或负数在字段级校验阶段即被拒绝；折扣率为 1 时净单价为零，允许但一律标记待折扣审批；默认税率按裁定 C-11 经 `ep_contract_invoice::TaxRateOptionQuery::default_rate(&mut dyn Tx,ctx,legal_entity_id,item_id)` 取得，不经 `ep-contract-mdm`；税率字典的唯一出处是 `invoice.tax_rate_options`，其建表迁移与种子迁移两条及 `TaxRateOptionQuery` 的 `default_rate` 与 `list` 两个方法属阶段 10 的 T0 切片第五项，自 T0 起即可取用，`MdmTaxRateStub` 整项撤销，阶段 5 不提供任何税率桩；合同行、订单行、交付确认行与退货行一律按行携带冻结的 `is_tax_included` 与 `tax_rate`，下游不得凭净价猜测录入口径。

#### 4.5 价格权限判定

对每一合同行独立判定，判定结果只打标不阻断，与 PRD 3.3.3 第一行一致。

- 取操作者在单据日期上生效的价格权限档案，按 USER、POSITION、ROLE 的顺序取第一条命中；三级均无命中时返回 `CPQ.PRICE_AUTHORITY.NOT_CONFIGURED` 并阻断提交，理由是无权限基准时无法判定，静默放行会使折扣审批链永不触发。
- `discount_rate > max_discount_rate` 时 `requires_discount_approval` 置真。
- 存在 `price_floor` 且 `net_unit_price < price_floor` 且 `allow_below_price_floor` 为假时置真。
- 价目未命中且 `allow_no_price_list_hit` 为假时置真。
- 合同行中存在任一 `requires_discount_approval` 为真时，提交审批时挂起折扣审批链；全部为假时不进入折扣审批节点，其余三条链照常执行。

#### 4.6 五项校验

校验在一个只读事务内取数，判定结论与取数快照写入 `clm.contract_validations` 与其明细，并按基线第 9.4 节写入审计事件。执行顺序固定为合同校验、价格权限、库存可用量、交期、客户信用额度。

1. 合同校验，阻断项。判定内容为头行必填齐备、客户与产品处于启用状态、每一合同行的 `delivery_date` 落在 `[valid_from, valid_to]` 区间内、三类信息齐备。三类信息齐备的判定固定为：条款正文非空且交付节点至少一条；收付款期次至少一条且比例合计等于 1 或金额合计等于合同金额；附件中至少存在一个 `purpose = CONTRACT_BODY` 的对象。该 U-E-09 首版值已批准，不再留给实现方选择。
2. 价格权限，按第 4.5 小节，不阻断。
3. 库存可用量，只对 `costing_mode=INVENTORY` 且非直运行经第 4.3.1 小节唯一的 `ep_contract_inventory::AvailabilityQueryPort::available` 按法人、冻结物料、仓库取当前可用量；同一组合的多行先合计请求量，再与可用量比较，不能逐行各自通过后造成合计超卖。可用量小于请求量时该项记为 FAILED。DIRECT_EXPENSE 与直运行固定记为 NOT_APPLICABLE，不得为了通过校验而虚构物料或仓库。
4. 交期，取库存可用量项的结论派生，可用量不足即交期不可满足。
5. 客户信用额度，按第 4.7 小节。

处置：合同校验 FAILED 一律阻断并定位到字段；库存可用量与交期 FAILED 在建单提交时不阻断，只记录并使派生时该订单进入待放行，理由与冻结取值见第 11.3 小节；信用额度 FAILED 按 `sales.credit_policies` 的 `on_exceed` 取值阻断或转审批。

#### 4.7 客户信用额度算法

已占用金额由三部分构成，三者按同一订单不重复占用，构成与迁移时点严格按 PRD 3.14.2 与规格第 5.2 章客户信用额度校验条目。

实现口径为不设独立的占用台账，三部分各由其状态的权威模块给出，非重复由生命周期本身保证。

- 在途订单金额：由本阶段的 `sales.v_credit_exposure_in_transit` 给出，等于已放行且尚未交付部分的含税金额合计。非终态行的唯一公式是 `open_amount_with_tax = line_amount_with_tax - SUM(CONFIRMED delivery_confirmation_lines.line_amount_with_tax)`；CLOSED/CANCELLED 固定为 0。它不按剩余量乘价重算，因此税内价不会重复乘税，分段交付尾差由累计区间精确守恒，全量交付必为 0。该列在交付确认回写、订单变更生效、订单取消与关闭三处同事务维护，并由销售经济图在订单/分批/确认任一侧变化时复核。
- 已交付未开票金额与应收未收金额：经 `ep_contract_finance::ReceivableExposureQuery::exposure(&mut dyn Tx,ctx,customer_id)` 一次调用取回 `delivered_unbilled_gross_amount` 与 `receivable_open_amount` 两项，其取数分别为该客户在应收账款未开票过渡科目上的含税借方余额与应收台账未核销价税合计。sales 对外 `CreditExposureView.delivered_unbilled_amount` 逐值取前者，字段改名只发生在 sales 的展示 DTO，不能让 finance contract 同时保留两个名字。按裁定 C-14，`finance::CreditExposureQuery` 与 `finance::CustomerCreditExposurePort` 两个旧名作废；对外唯一入口是本模块的 `ep_contract_sales::CreditExposureQueryPort::exposure`，由本阶段把在途桶与上述两项组装为 `CreditExposureView` 的 `credit_limit`、`in_transit_amount`、`delivered_unbilled_amount`、`receivable_open_amount`、`available_amount` 五项返回。本阶段不注入任何替身，`ReceivableExposureQuery` 按裁定 C-14 不进 T0 切片，与阶段 10 该端口按第 11.5 小节同批交付同批验收，承载三桶组装的用例整体落在第三批并在该批次一次接线，三桶取数当场成立，不存在只取两桶、取 `None` 或以零值参与求和的形态。

判定步骤：

1. 在信用校验事务内对 `sales.customer_credit_controls` 的该客户行执行 `SELECT ... FOR UPDATE`，行不存在时先插入。该行是同一客户并发下单的串行化点，`lock_timeout` 为 3 秒，超时返回 `PLATFORM.DB.LOCK_TIMEOUT`。
2. 取客户档案的信用额度。为空时按 `null_limit_behavior` 处置，出厂默认 `TREAT_AS_ZERO`。
3. 取三部分占用并求和，三桶及本次请求金额全部固定取价税合计；`amount_basis` 必须为 `WITH_TAX`，不读取或实现不含税分支。
4. 本次待增加占用：合同建单提交时取本合同全部合同行的含税金额合计；合同生效派生时取本次派生的订单行含税金额合计；订单变更时取变更后与变更前的在途金额之差且只在为正时判定。
5. 判定 `requested + occupied <= credit_limit`。不成立时按 `on_exceed` 阻断或转审批，出厂默认 `REVIEW`。
6. 把信用额度、三部分取值、本次待增加占用、超出金额与判定结论写入校验明细的 `snapshot` 并写入审计。
7. 更新 `customer_credit_controls.last_exposure` 与 `last_checked_at`，用于界面展示与对账，不作为判定依据。

释放的反向情形按同一映射反向执行，不设单独的释放动作：订单取消与剩余数量关闭把对应行的 `open_amount_with_tax` 归零；交付确认使该部分自在途桶移出并由财务侧进入已交付未开票桶；开票、到款与红冲的桶间迁移全部发生在财务侧。销售退货登记本身不改变本阶段的在途桶，因为退货针对的是已交付部分，其释放体现为财务侧两个桶的减少，这一点在第 8 节以专门用例验证。

边界条件：客户在两个法人下分别设额度，跨法人不合并，理由与冻结取值见第 11.3 小节；预收账款不抵减占用，由 `deduct_advance_receipts` 开关承载，默认关闭；`ep-contract-finance` 端口不可用时信用校验返回 `INFRASTRUCTURE` 且可重试，不静默按零占用放行。已交付未开票与应收未收两桶随第三批与阶段 10 的 `ReceivableExposureQuery` 同批交付，交付即为真实取数；校验明细的 `snapshot` 一律记录信用额度与三桶的真实取值，不存在按未接线呈现或以 `None` 参与判定的形态。

#### 4.8 合同生效与派生算法

生效动作在一个事务内完成：校验重新认证凭证、校验生效审批结论、写合同状态为 EFFECTIVE、写 `clm.contract_versions` 快照、执行幂等 `finish`、写 Outbox 条目 `clm.contract.effective.v1`，最后批量写审计事件。事务内不发起任何外部调用，不读写附件正文；审计终结批之后不得再调用数据库或跨模块端口。

派生编排在 job-worker 中执行，步骤如下。

1. 消费 `clm.contract.effective.v1`，在 `platform_msg.inbox_consumptions` 上以 `(consumer = 'clm.derivation', event_id)` 唯一约束保证只处理一次。
2. 在一个事务内建立 `clm.contract_derivations` 批次行与全部 `clm.contract_derivation_items`，`item_total` 一次算定。批次行的唯一约束在 `(contract_id, contract_version_no, trigger)`，重复投递直接命中冲突并结束。
3. 派生项的生成规则：销售订单一张，含全部合同行；采购需求按 `requires_procurement` 为真或 `order_type = DROP_SHIP` 的合同行逐行一条；项目任务在 `contract_types.requires_project` 为真时按交付节点逐条一条；收款计划按收付款期次逐期一条；交付节点为本模块内对象，派生项的动作是把 `clm.contract_milestones.status` 由 PLANNED 置为 ACTIVE，纳入同一批次只为使追溯与计数口径一致。其中收款计划派生项按裁定 C-20 只写本模块的 `clm.contract_payment_schedules`，不调用任何外部端口；项目任务派生项按裁定 C-19 只登记不派发，在批次建立的同一事务内即置 `status = DONE`、`target_module` 取 project、`target_doc_id` 留空，实际项目任务由阶段 12 的 `project.contract_derivation` 消费者消费 `clm.contract.effective.v1` 后经本小节末段的 `ContractDerivationPlanQuery` 自行派生，追溯经该查询的 `unique_key` 对应，本阶段不再同步派生项目任务。
4. 每个派生项一个独立事务，调用目标模块的契约端口，`Idempotency-Key` 取该派生项的 `id`，本身是 UUIDv7。目标端口固定为：销售订单经 `ep_contract_sales::SalesOrderDerivationPort`；采购需求经 `ep_contract_procure::PurchaseRequisitionIntakePort::intake`，其 `unique_key` 取 `CONTRACT:{contract_id}:{contract_line_id}:{contract_version}`，按裁定 C-17 与第 11.5 小节该端口的派发整条推迟到阶段 7，本阶段不注入替身也不写调用点；交付节点与收款计划两类为本模块内写入，不出模块；项目任务类不调用端口，按第 3 步处理。成功写 `target_doc_id`、`target_doc_no` 与 `status = DONE`；采购需求派生项在阶段 7 接线之前 `status` 恒为 PENDING、`target_doc_id` 与 `target_doc_no` 留空且不计入 `item_done`，因此含该项的合同停在 EFFECTIVE 且 `derivation_state` 保持 RUNNING，界面按未接线呈现，阶段 7 接线后补派发并推进到 IN_PERFORMANCE，不得构造占位单号；失败 `attempts + 1` 并按基线第 6.2 节的八档退避重排 `available_at`，八次全部失败置 `DEAD` 并写入 `platform_msg.dead_letters`。
5. 销售订单派生项内先执行第 4.6 小节的后四项校验，结论写 `sales.order_validations`；四项全通过则订单状态为 RELEASED，信用或库存任一不足则为 PENDING_RELEASE 并写 `pending_release_reason`，同时按 `on_exceed` 追加信用审批节点或直接置为待放行。
6. 全部项 DONE 时把合同置为 IN_PERFORMANCE、`derivation_state = DONE`，并发 `clm.contract.derivation_completed.v1`。存在 DEAD 项时 `derivation_state = FAILED`，合同保持 EFFECTIVE 并在界面显示待人工修复，与 PRD 3.6 中已生效到已生效的自环一致。
7. 人工修复后可重放该批次，重放按第 3.3 小节的两道唯一约束去重，不产生重复单据。

边界条件：单张合同的派生项数上限由 `EP__CLM__DERIVATION__MAX_ITEMS_PER_CONTRACT` 约束，默认 2000，超出直接拒绝生效并提示拆分合同；派生过程中合同不接受修订，`actions/amend` 在 `derivation_state = RUNNING` 时返回 `CLM.CONTRACT.DERIVATION_IN_PROGRESS`。
按裁定 A-16 并以 F-51 U-J-13 补足版本比较语义，本阶段在 `crates/contract/clm/src/port/derivation.rs` 提供 `ContractDerivationPlanQuery::derivation_plan(tx, ctx, contract_id, contract_version_no)`，返回 `ContractDerivationPlan`，实现落在 ep-app-clm，取数与上述派生项生成规则同源，不另建第二套计划。`ContractDerivationItem` 固定包含 `item_kind`、`unique_key`、`obligation_key`、`obligation_hash`、`source_contract_line_id`、`milestone_no`、`name`、`promised_date`、`quantity`、`owner_user_id`：`unique_key` 取 `<contract_id>:<contract_version_no>:<item_kind>:<source_contract_line_id 或 milestone_no>`，用于新版本补充任务的幂等创建；`obligation_key` 取 `<item_kind>:<source_contract_line_id 或 milestone_no>`，在同一合同版本链内稳定，用于对照同一义务；`obligation_hash` 取上述业务字段（不含 `unique_key` 与合同版本号）按 RFC 8785 规范化 JSON 后的 SHA-256，用于判定义务是否改变。三个字段均由 CLM 产生，阶段 12 不自行重算。该键与第 4 步采购需求经 `PurchaseRequisitionIntakePort` 传入的 `unique_key` 不是同一个键，前者供阶段 12 的派生去重，后者供采购需求登记去重，两者各按其裁定取值。

#### 4.9 合同变更后的重新派生

按 PRD 3.5.4 的五种情形分派处理。

| 情形 | 处理 |
|---|---|
| 新增合同行 | 在已派生订单上追加订单行，追加动作走 `SalesOrderDerivationPort::append_lines`，追加后按第 4.6 小节重跑后四项校验 |
| 已派生未开始交付的行数量或交期变更 | 调整对应订单行与其分批交付行，走第 4.10 小节的订单变更版本 |
| 已部分交付的行变更 | 只允许调整未交付部分，`new_quantity` 不得小于 `delivered_quantity`，否则返回 `SALES.SALES_ORDER.DELIVERED_QTY_EXCEEDED` |
| 收付款信息变更 | 按裁定 C-20 直接维护本模块的 `clm.contract_payment_schedules`；同一事务先经 `ReceiptPlanBillingQuery::billing_by_period` 判定该期净已开金额为零，并经 finance 权威查询判定尚未到款，才允许调整。任一金额大于零都不调整；`ep_contract_finance::ReceivablePlanPort` 已撤销，不再派生第二套收款计划或金额副本 |
| 交付节点变更 | 调整 `status = ACTIVE` 的节点，`status = CONFIRMED` 的节点不调整 |

#### 4.10 订单变更与分批交付

订单变更：提交变更建立 `sales.sales_order_changes` 并把订单置为 CHANGE_APPROVAL，此后数量、单价、交期、仓库四类字段被锁定，其余字段仍可维护。审批通过后在一个事务内先按第 4.3.1 小节锁住旧、新全部库存组合，以旧的有效需求为基线计算各组合正增量并重跑唯一可用量查询；任一组合 `available < requested` 时整笔拒绝且旧版本继续生效。全部通过后才写旧版本快照、应用变更行、`version_no + 1`、重算 `open_amount_with_tax` 与 `inventory_demand_state`、解除锁定，并重跑交期与信用其余两项；减量与移出旧仓库立即释放对应需求。审批驳回则订单回到进入变更前的状态，变更单置 REJECTED。单价的修改不在订单上直接进行，`new_net_unit_price` 只允许由合同变更派生的变更单携带，用户直接提交时返回 `SALES.SALES_ORDER.PRICE_CHANGE_NOT_ALLOWED`。

拆分与合并：同一订单行的全部分批交付行数量合计必须等于该订单行数量，不等时返回 `SALES.DELIVERY_SCHEDULE.SPLIT_SUM_MISMATCH`。未拆分的订单行在派生时即建立一条分批交付行，其数量与约定交付日期取订单行取值，因此系统中不存在没有分批交付行的订单行，这使交付确认与交付指标的取数只有一条路径。`status` 不为 PENDING 的分批交付行不可再拆分、不可改数量与仓库，返回 `SALES.DELIVERY_SCHEDULE.NOT_SPLITTABLE`。拆分与合并不改变订单总量、总金额与信用占用总额，该性质由领域属性测试守护。

#### 4.11 交付确认与三腿过账

交付确认单是规格第 8 章黄金业务闭环第 8 步的唯一落点，按裁定 A-09 归本阶段。单据只有 DRAFT 与 CONFIRMED 两个状态，不设作废态，冲正一律经销售退货单。建表与登记动作属第二批，确认动作的三腿一次全真接线属第三批，按第 11.5 小节与阶段 10 的 `UnbilledArPort` 同批交付同批验收；该批次之外本阶段不建该调用点，`confirm_delivery` 用例与其端点不写入代码，也不注入任何替身，因此系统内不存在只落两腿的已确认交付。

登记动作 `create_delivery_confirmation` 位于 `crates/application/sales/src/usecase/create_delivery_confirmation.rs`，只允许 `WAREHOUSE_USER` 登记出库与发运事实。按 `sales_order_id` 取该订单下 `status = PENDING` 的分批交付行，逐条建立 `sales.delivery_confirmation_lines`，`sales_order_line_id` 与 `delivery_schedule_id` 均为同 schema 外键；本次数量不得超过该分批交付行的 `quantity - delivered_quantity`，超出返回 `SALES.DELIVERY_CONFIRMATION.QTY_EXCEEDS_SCHEDULED`。`costing_mode`、`inventory_material_id`、`net_unit_price`、`is_tax_included` 与 `tax_rate` 自订单行带出；DRAFT 时 `allocation_quantity_before` 为空，展示金额只是按当前锁后已确认累计量计算的预览，不是持久化事实。确认事务锁住订单行、全部分批与既有确认行后，把锁后已确认累计量写入 `allocation_quantity_before`，再按第 4.4 节 `cum(B+q)-cum(B)` 一次冻结两项行额；不得把订单整行金额直接复制给部分交付，也不得对本次数量独立舍入。`is_drop_ship` 自订单头的 `order_type` 派生；`posting_date` 由操作者录入，取值与用途按基线第 3.4 节。

确认动作 `confirm_delivery` 位于 `crates/application/sales/src/usecase/confirm_delivery.rs`，只允许 `SALES_MANAGER` 或 `PROJECT_MANAGER` 执行最终交付确认；`TECHNICIAN` 可登记安装、调试、维修和交付证据，但不得执行本动作。确认在一个事务内完成。普通销售确认/下达仍按第 4.3.1 小节直接取得 `sales-availability:` 锁；本动作同时改变库存，必须改走 F-50 coordinator，禁止先用销售私有方法取得 availability 锁后再补来源/余额锁。四步次序固定，不得调换。

1. 会计期间解析、统一锁计划与 proof。记账日早于服务端事务日时先验证 `ledger.backdate`、绑定本确认命令摘要的重新认证与 FINANCE_MANAGER 审批并构造唯一 `BackdateAuthorization`，否则为 None；该证明只由服务端产生，客户端不能提交引用。经 `ep_contract_ledger::AccountingPeriodResolver::resolve` 在事务最前解析一次，库存腿与过渡科目腿复用其返回值，不各自再解析。只做权限/强类型解析、UUID 预生成与无锁读取，收集本交付确认来源 `InventorySourceDocument(DeliveryConfirmation,id)`、非直运 INVENTORY 行的 availability/value/coverage/quantity/serial 全集；调用 `lock_all` 后再按稳定 id 锁交付确认单、分批交付行和订单行，重读同一关系图并重建 plan。只有 `seal_after_reload` 逐值相等才取得 `TransactionLockProof`；集合漂移以 40001 整事务重试。除期间零行建立与 coordinator 的零余额锁脚手架外，proof 前不得计算出库金额、更新销售需求或写任何业务事实。
2. 库存腿，端口由阶段 8 提供。只把 `costing_mode=INVENTORY` 且非直运的行传给 `ep_contract_inventory::InventoryPostingPort::post_outbound(tx, ctx, &f50_lock_proof, OutboundPosting { reason: OutboundReason::DeliveryConfirmation, source: SourceDocumentRef{ doc_type: DELIVERY_CONFIRMATION, .. }, period, label, lines })`；每条行的 `posting_line_key` 固定取 `<delivery_confirmation_line_id>:1`，由 `PostingLineKey` 验证构造器建立，`source_line` 取同一行 id/line_no，`pricing=OutboundPricing::MovingAverage`，仓库与物料取同事务 MDM 锁后快照。返回集合按 `posting_line_key` 一一映射，把 `outbound_amount` 回填为 `cogs_amount`，逐行 `stock_movement_id` 必须等于结果头 id。直运行和 DIRECT_EXPENSE 行均不调用库存端口，两列留空；混合单据只为 INVENTORY 子集产生库存流水。
3. 凭证腿，端口由阶段 9a 提供。以锁后行与库存结果构造交付 measures：`revenue_amount=Σline_amount`、库存金额键 `cogs_amount=Σ非直运 INVENTORY 行 cogs_amount` 仅在存在该类行时提交（允许合计为零）；`gross_amount=Σline_amount_with_tax` 只作普通销售守卫与过渡子账控制总额，不进入 PostingInput，且要求 revenue 与 gross 同为零或同为正。`PostingInput` 七字段逐项固定为 `source_kind=DELIVERY_CONFIRMED`、锁后 `posting_date`、步骤 1 的 `backdate_authorization`、`source_document={object_type:"DELIVERY_CONFIRMATION",id:本确认id,doc_no:锁后单号}`、`source_event_id=None`、上述 `measures` 与本段 `attributions`；不存在 `source_sequence_no`。owner 的 confirmed 事件尚未进入统一后缀，故 source_event 必须为空。每条非零 `line_amount` 生成一条 revenue attribution，`source_document_line_id=delivery_confirmation_line_id`、`measure_key=RevenueAmount`、`amount=line_amount`、`capture_kind=RevenueDeliveryOrder`、`reverses_capture_entry_id=None`；每条非零 `cogs_amount` 生成一条 cost attribution，行 id 相同、`measure_key=CogsAmount`、`amount=cogs_amount`、`capture_kind=CostInventoryCogs`、父为空。两类 dimensions 都只取锁后冻结快照：contract/order/order_line/customer、可空 project、PRODUCT 时 product、可空 material；成本行另带 warehouse，收入行 warehouse 为空。attributions 按 `(measure_key,source_document_line_id)` 排序，逐键金额和必须分别等于 measure 绝对值；零 measure 不生成 attribution，非成本/收入控制额不得混入。调用 `PostingPort::post` 后，若任一收入、销货成本或库存释放会计效果非零，只接受 `PostingOutcome::Posted`，回带两项期间必须分别等于 `resolved.accounting_period_id()` 与 `resolved.deferred_from_period_id()`，并把 `voucher_id` 回填交付确认头；若全部会计效果为零，只接受同期间的 `PostingOutcome::Skipped`，头 `voucher_id` 保持空。首次执行命中 `IdempotentReplay` 代表孤立凭证图，非零效果得到 `Skipped`、零效果得到 `Posted` 或期间不等均按内部不变量整事务失败。DIRECT_EXPENSE 不生成库存或虚构销货成本腿，但其非零收入行仍必须有 revenue attribution。
4. 过渡科目腿，端口由阶段 10 提供。仅当 `net_amount/gross_amount` 同为正数时，经 `UnbilledArPort::record_on_delivery(tx,ctx,DeliveryUnbilledArCommand { delivery_confirmation_id,customer_id,posting_date,accounting_period_id:resolved.accounting_period_id(),accounting_period_seq:resolved.accounting_period_seq(),deferred_from_period_id:resolved.deferred_from_period_id(),voucher_id:posted.voucher_id,direction:UnbilledArDirection::Debit,net_amount,gross_amount })` 一次插入 NOT NULL/APPEND_ONLY 行；此分支必为 `Posted` 且 voucher 非空。两额同为零时不调用该端口、不伪造零额过渡子账；一空一非零已在第 3 步拒绝。信用投影只用 gross，会计勾稽只用 net。顺序固定为 period resolve→inventory→PostingPort→适用时 UnbilledArPort，不允许先插入空 voucher 再 UPDATE；任一后步失败仍因同一事务整体回滚。

四步之后仍在同一事务内按确认数量增加 `sales.delivery_schedules.delivered_quantity` 与 `sales.sales_order_lines.delivered_quantity`，同步维护两层 status、订单头状态，并把 `open_amount_with_tax` 写为订单行含税总额减全部已确认交付行含税额；分批行未交完保持 PENDING，恰好交完才 DELIVERED，每次确认的权威关联已经在本次 `delivery_confirmation_lines` 中，不回写会覆盖历史的一对一确认列。订单行剩余量为零时把其 `inventory_demand_state` 置 INACTIVE，否则保留 RELEASED。随后用同一个 `SalesAwareAvailabilityQuery` 对已锁组合重算，把结果保存在待写审计的 after 快照中，再把交付确认单置 CONFIRMED 并写 `confirmed_at` 与 `confirmed_by`；最后依次执行幂等 `finish`、写 Outbox 条目 `sales.delivery.confirmed.v1`、写同事务通知命令（如有）与审计终结批。延迟图在 COMMIT 再从确认事实反算两层累计、区间和 open amount；任一腿或回写失败整笔回滚，不存在只写一腿、库存已出而销售需求未释放的中间态。本阶段不判定借贷方向、不取价、不确定科目，四项 `measures` 的口径与其对应分录一律按规格第 5.2 章的事件-分录表，本小节不复述。

事件 `sales.delivery.confirmed.v1` 的 `aggregate_type` 取 `sales.delivery_confirmations`，payload 字段固定为 `delivery_confirmation_id`、`doc_no`、`sales_order_id`、`customer_id`、`contract_id`、`is_drop_ship`、可空 `voucher_id`、`lines`，其中 `lines` 每元素含 `delivery_confirmation_line_id`、`sales_order_line_id`、`delivery_schedule_id`、`item_kind`、`item_id`、`costing_mode`、`inventory_material_id`、`quantity`、`allocation_quantity_before`、`net_unit_price`、`is_tax_included`、`tax_rate`、`warehouse_id`、`batch_no`、`serial_nos`、`revenue_amount`、`gross_amount`、`cogs_amount`；revenue/gross 分别取该确认区间的 `line_amount/line_amount_with_tax`，`cogs_amount` 仅 INVENTORY 非直运行取库存腿回填值，其余为空。`voucher_id` 为空严格表示本次全部会计效果为零，不表示待补写。信封的 `posting_date` 取单据的 `posting_date`，`accounting_period_id` 取 `Posted|Skipped` 均回带的实际期间。

销售侧回写只由上述确认原事务承担，不存在 `sales.delivery_writeback` 消费者。`clm.milestone_confirm` 仅消费事件推进 `clm.contract_milestones` 的交付节点，不改 sales schema。在途信用桶与 U-G-01 未交付数量都由确认事务中的同一数量变化立即移出，不设第二条销售回写路径。

#### 4.12 销售退货

前置校验按 PRD 3.13.1：退货数量不超过该订单行 `delivered_quantity - returned_quantity`；每一退货明细行必须至少关联一条交付确认单行，且同一退货行的 `Σ sales.return_line_delivery_links.quantity` 必须精确等于本次退货数量，链接行必须属于同一法人、同一销售订单行及已确认交付，关联方式为操作者指定或按交付先后自动带出并记录 `assigned_by`。锁定原交付行后，还必须逐行满足“既有 REGISTERED/CLOSED 退货已占用链接数量 + 本次链接数量 ≤ 原交付确认行数量”；DRAFT/SUBMITTED/CANCELLED 不占用，两个并发登记由同一原交付行锁串行化。任一不成立返回 `SALES.SALES_RETURN.DELIVERY_LINK_REQUIRED` 或 `SALES.SALES_RETURN.QTY_EXCEEDS_DELIVERED`，零写入。该退货部分已开票的必须先完成红字冲销，按裁定 C-16 由 `ep_contract_invoice::InvoiceReversalStatusQuery::is_fully_credit_noted(tx, ctx, sales_order_line_id, quantity)` 判定；只有返回 `CreditNoteStatus.is_fully_credit_noted=true` 才继续，false 时以同一返回值的 `pending_invoices` 生成 `SALES.SALES_RETURN.INVOICE_NOT_CREDIT_NOTED` 明细，未开票数量不要求红冲，无票时返回 true 与空清单。原 `InvoiceStatusPort` 一名作废；该判定不进 T0，与阶段 10 的该 trait 按第 11.5 小节同批交付同批验收，本阶段不注入替身，承载该判定的退货登记分支整体落在第三批并在该批次当场成立。直运订单的退货 `is_drop_ship=true`，整段跳过库存端口且凭证不得出现 `inventory_return_amount`；按裁定 B-07 在同一登记事务内调用 `ep_contract_procure::PurchaseReturnLinkPort::link_drop_ship_return(tx, ctx, sales_return_id, lines)` 勾稽对应的采购退货，该调用整条推迟到阶段 7，本阶段不注入替身也不写调用点，阶段 7 之前系统内不存在采购订单，直运订单无从交付，该路径由既有的 `SALES.SALES_RETURN.QTY_EXCEEDS_DELIVERED` 自然阻断，不新增错误码。

退货状态机只有五类合法动作、六条起终点边：DRAFT→SUBMITTED（提交审批）、SUBMITTED→DRAFT（审批驳回并发 rejected 事件）、DRAFT→CANCELLED 与 SUBMITTED→CANCELLED（共用登记前取消动作并发 cancelled 事件）、SUBMITTED→REGISTERED（审批通过后执行下述同步登记）、REGISTERED→CLOSED（业务闭合并发 closed 事件）。REGISTERED、CLOSED、CANCELLED 均不得再进入 CANCELLED 或回到前态，命中统一返回 `SALES.SALES_RETURN.INVALID_STATE_TRANSITION` 且零写入。首版不提供已登记销售退货的冲正端点或事件；未来若需要撤销，必须新增独立的反向业务单据、库存反向流水、会计冲正凭证和版本化事件，绝不得改写或取消既有 REGISTERED 事实。

`register_sales_return` 位于 `crates/application/sales/src/usecase/register_sales_return.rs`。库存调用前后的销售侧映射与登记响应 DTO 固定如下：前两个类型是该用例的应用层私有结构，只负责把销售退货行一一映射到阶段 8 已冻结的 `InboundPosting`/`InboundPostingResult`，不得增加调用方可传入的金额或取价分支；后两个类型位于 `crates/contract/sales/src/dto/sales_return.rs`，是 HTTP 登记端点与幂等存档共用的唯一响应形状。

```rust
pub struct SalesReturnInboundLineInput {
    pub sales_return_line_id: Id<SalesReturnLine>,
    pub posting_line_key: PostingLineKey, // 规范值 `<sales_return_line_id>:1`，重试不变
    pub line_no: i32,
    pub warehouse_id: Id<Warehouse>,
    pub material_id: Id<Material>,
    pub batch_no: BatchNo,
    pub serial_nos: Vec<SerialNo>,
    pub quantity: Quantity,
    pub allocations: Vec<OriginalCostAllocation>,
}
pub struct SalesReturnInboundLineOutput {
    pub sales_return_line_id: Id<SalesReturnLine>,
    pub stock_movement_id: Id<StockMovement>,
    pub inventory_return_amount: Money,
    pub pricing_branch: PricingBranch,
}
pub struct RegisteredSalesReturnLineView {
    pub sales_return_line_id: Id<SalesReturnLine>,
    pub inventory_return_amount: Option<Money>,
    pub stock_movement_id: Option<Id<StockMovement>>,
}
pub struct RegisteredSalesReturnView {
    pub sales_return_id: Id<SalesReturn>,
    pub doc_no: String,
    pub status: SalesReturnStatus, // 恒为 REGISTERED
    pub registered_at: chrono::DateTime<chrono::Utc>,
    pub voucher_id: Option<uuid::Uuid>, // 仅全部会计效果为零时为 None
    pub lines: Vec<RegisteredSalesReturnLineView>, // 按 sales_return_line_id 升序
}
```

HTTP 登记端点与幂等结果都只返回 `RegisteredSalesReturnView`；头 `voucher_id=None` 当且仅当本次全部收入/税/库存回收入/成本冲回会计效果为零。非直运 INVENTORY 行的两个 `Option` 同时为 Some（金额允许为零），DIRECT_EXPENSE 与直运行同时为 None，不接受一空一非空。创建端口与登记端点是两种不同响应；创建端口的唯一精确契约如下，取代 A-17 早期的三字段 `SalesReturnView`：

```rust
pub enum DeliveryAllocationMode { Manual, AutoFifo }
pub struct SalesReturnSourceRef {
    pub source_module: ModuleCode,
    pub source_doc_type: String,
    pub source_doc_id: uuid::Uuid,
    pub source_doc_line_id: uuid::Uuid,
}
pub struct SalesReturnDeliveryLink {
    pub delivery_confirmation_line_id: Id<DeliveryConfirmationLine>,
    pub quantity: Quantity,
    pub assigned_by: DeliveryLinkAssignedBy,
}
pub struct CreateSalesReturnLine {
    pub sales_order_line_id: Id<SalesOrderLine>,
    pub quantity: Quantity,
    pub batch_no: String,
    pub serial_nos: Vec<String>,
    pub delivery_links: Vec<SalesReturnDeliveryLink>,
}
pub struct CreateSalesReturn {
    pub customer_id: Id<Customer>,
    pub sales_order_id: Id<SalesOrder>,
    pub return_reason: String,
    pub return_warehouse_id: Option<Id<Warehouse>>,
    pub posting_date: chrono::NaiveDate,
    pub source_ref: Option<SalesReturnSourceRef>,
    pub remark: Option<String>,
    pub allocation_mode: DeliveryAllocationMode,
    pub lines: Vec<CreateSalesReturnLine>,
}
pub struct SalesReturnLineView {
    pub sales_return_line_id: Id<SalesReturnLine>,
    pub sales_order_line_id: Id<SalesOrderLine>,
    pub quantity: Quantity,
    pub delivery_links: Vec<SalesReturnDeliveryLink>,
}
pub struct SalesReturnView {
    pub sales_return_id: Id<SalesReturn>,
    pub doc_no: String,
    pub status: SalesReturnStatus,
    pub lines: Vec<SalesReturnLineView>,
}
#[async_trait::async_trait]
pub trait SalesReturnCommandPort: Send + Sync {
    async fn create_sales_return(
        &self, tx: &mut dyn Tx, ctx: &SecurityContext, cmd: CreateSalesReturn,
    ) -> Result<SalesReturnView, AppError>;
}
```

`Manual` 要求每个命令行 links 非空、合计等于退货数量且 `assigned_by=Manual`；`AutoFifo` 要求调用方 links 全空，sales 锁定同法人同订单行的可退已确认交付后按 `confirmed_at ASC, delivery_confirmation_line_id UUID bytes ASC` 分配并持久化 `assigned_by=AutoFifo`。两种模式都重验累计可退量。返回 lines 按 `sales_return_line_id` 升序、links 按交付确认行 id 升序；调用方必须使用返回的真实行 id。`remark` 清洗后最长 2000，`source_ref` 四字段按头表整体形状与条件唯一键持久化。

退货金额与原 capture 分配没有自由分支。锁定每条原交付行后，以其原始确认数量 `Q`、实际净收入、含税收入与实际 INVENTORY COGS 为三个总额 `M`；对本 link 在全部已登记退货中的连续区间 `[B,B+q)`，金额一律取 `round(M*(B+q)/Q,2)-round(M*B/Q,2)`。因此中间批次只取累计差额，覆盖到 `Q` 的末段自动吸收两位金额尾差，禁止从六位单价反算、按当前售价或当前移动平均价重估。每条非直运 INVENTORY 退货行传给阶段 8 的元素固定为 `OriginalCostAllocation { source_line_id: delivery_confirmation_line_id, quantity: link.quantity, amount: link.cost_amount }`，按交付行 id 排序，数量和金额分别精确汇总为退货行数量与 `inventory_return_amount`；合法零 COGS 的元素允许 `amount=0`。取价恒为 `InboundPricing::ReturnAtOriginalDeliveryCost { allocations }`，与当前库存数量、金额余额或移动平均单价无关，不存在备用取价分支。

同一锁内以所有原交付行调用 `ep_contract_costing::DeliveryCaptureReturnBasisQuery::lock_available`。收入分配按 `RevenueLiveFragment`，成本分配按 `CostLiveFragment { role: ReturnCostRole, .. }`；每个 link/side 都必须把 owner 返回的全部 current live leaves 纳入同一整数分 largest-remainder，禁止 FIFO、任取第一片或逐片 round。令 `T` 为 link 该 side 的两位金额整数分、`A_i` 为第 i 个 leaf 的 `available_amount` 整数分、`S=ΣA_i`；先拒绝 `T>S`，再取 `base_i=floor(T*A_i/S)` 与精确余数 `fraction_i=(T*A_i) mod S`，把 `T-Σbase_i` 个 1 分依次补给排序键 `(fraction_i DESC, role_ordinal ASC, root_entry_id UUID bytes ASC, live_entry_id UUID bytes ASC)` 最前的 leaf。收入 side 的 `role_ordinal` 固定为 0；成本为 `MainOperatingCost=0,DirectExpenseCost=1`。金额为零不落 allocation 行；其余逐片必须 `0<amount<=available_amount` 且合计恰为 T。受控更正后同一 root 的多个 live leaf 必须全部参与，该公式在重试、并发串行与不同数据库执行计划下结果相同。随后把 root/live、side、cost role、金额和原 link 写入 `return_line_capture_allocations`。`ReturnCostRole::MainOperatingCost` 唯一映射静态 `MeasureKey::InventoryReturnCogsAmount`，`ReturnCostRole::DirectExpenseCost` 唯一映射 `MeasureKey::InventoryReturnDirectExpenseAmount`；sales 不导入 `ep-contract-ledger::AccountRole`，也不根据 account id 或配置猜 measure。收入/成本可分配 live 余额不足、owner 返回错交付/错侧/错角色/错维度或集合漂移均按不变量整笔回滚。

登记事务的执行次序冻结为以下五步，禁止调换，所有调用复用同一个 `&mut dyn Tx` 与 `SecurityContext`。

1. 事务最前经 `AccountingPeriodResolver::resolve` 解析一次完整 `ResolvedPeriod`。随后仅以无锁查询收集本销售退货来源 `InventorySourceDocument(SalesReturn,id)` 与全部非直运 INVENTORY 行的 availability/value/coverage/quantity/serial 键，执行 `lock_all`；全局类别完成后才按稳定 id 锁退货头、退货行、订单行、交付关联和原交付行，并以全部原交付行调用一次 `DeliveryCaptureReturnBasisQuery::lock_available`，按 owner 固定 root/live 顺序取得锁后叶片。重跑业务守卫与同一候选收集，owner 结果的行、side、root/live、role、开放额和维度也必须与锁后复读逐值相同；全部规范化集合相等才 `seal_after_reload` 得 proof，漂移走 40001。不得先调用第 4.3.1 节的销售私有 availability 锁；DIRECT_EXPENSE 行和直运行不进入库存 plan。除期间零行建立与 coordinator 零余额锁脚手架外，proof 前不计算累计差额或写分配、不更新状态或数量。
2. proof 后按上文累计差额公式一次写定 links 的区间与三额，并在 current live fragments 间确定性形成、写入 capture allocations。若存在非直运 INVENTORY 行，组装一次文档级 `InboundPosting`：`reason=InboundReason::SalesReturn`，来源头固定为 `SourceDocumentRef { doc_type: SourceDocType::SALES_RETURN, doc_id, doc_no }`，业务日期与三项期间值取第一步的 `ResolvedPeriod`，`label` 原样取销售退货头；每个输入行以同事务锁后 MDM 快照构造 `WarehousePostingRef/MaterialPostingRef`，`source_line` 取退货行 id/line_no，`posting_line_key` 固定取规范值 `<sales_return_line_id>:1`，取价固定为 `InboundPricing::ReturnAtOriginalDeliveryCost { allocations: Vec<OriginalCostAllocation { source_line_id,quantity,amount }> }`。调用 `InventoryPostingPort::post_inbound(tx,ctx,&f50_lock_proof,cmd)` 后，输出 `posting_line_key` 集合必须与输入集合完全相等、无重无漏，把 `InboundLineResult.inbound_amount` 映射为 `inventory_return_amount`；逐行金额必须等于持久化 links 的 `cost_amount` 合计、可以为零，且逐行 id 都等于结果头唯一 `stock_movement_id`，`pricing_branch` 恒为 `ORIGINAL_DELIVERY_PRICE`。否则按 `PLATFORM.SYSTEM.INTERNAL_ERROR` 失败关闭。不存在该类行时不调用库存端口，库存效果集合为空。
3. 以严格七字段调用 `PostingPort::post`：`source_kind=VoucherSourceKind::SALES_RETURN`、锁后 `posting_date`、第一步 `backdate_authorization`、`source_document={object_type:"SALES_RETURN",id:本退货id,doc_no:锁后单号}`、`source_event_id=None`、`measures` 与本段 `attributions`；不存在 `source_sequence_no`。`revenue_amount=Σ links.revenue_amount` 为收入键；成本控制总额 `inventory_return_amount=Σ links.cost_amount` 不进入 `measures`，只用于守恒，按 allocations 的 `cost_role` 分组形成可选的 `inventory_return_cogs_amount` 与 `inventory_return_direct_expense_amount` 两个静态键，且两键合计必须严格等于控制总额。每条非零 REVENUE allocation 生成 `RevenueSalesReturn` attribution，`measure_key=RevenueAmount`；每条非零 COST allocation 按 `MainOperatingCost -> (InventoryReturnCogsAmount,CostInventoryCogs)`、`DirectExpenseCost -> (InventoryReturnDirectExpenseAmount,CostDirectExpense)` 生成 attribution；`source_document_line_id` 均为当前退货行，`amount=allocation.amount`、`reverses_capture_entry_id=allocation.live_entry_id`、dimensions 逐值复制 owner fragment。attributions 按 `(measure_key,source_document_line_id,live_entry_id)` 排序，各键金额合计与对应 measure 精确相等，零组不生成 attribution。另计算 `gross_amount=Σ links.gross_amount` 只作守卫和未开票应收控制额，不进入 PostingInput；普通退货要求 revenue 与 gross 同为零或同为正。收入、税、库存回收入或成本冲回任一会计效果非零时只接受 `PostingOutcome::Posted`；全部会计效果为零时只接受 `PostingOutcome::Skipped`。两种结果回带的两项期间都必须分别等于 `resolved.accounting_period_id()`/`resolved.deferred_from_period_id()`；本事务尚未登记完成时出现 `IdempotentReplay` 代表孤立凭证图，非零效果得到 `Skipped`、零效果得到 `Posted` 或期间不等均按 `PLATFORM.SYSTEM.INTERNAL_ERROR` 失败并整体回滚。金额、方向和科目只接受 `docs/data-dictionary/ledger.md` 的 SALES_RETURN 映射，不由 sales 重算或自选。`LEDGER.POSTING.MEASURE_INVALID` 原样返回。
4. `revenue_amount/gross_amount` 同为正数时，结果必为 `Posted`，调用 `UnbilledArPort::record_on_sales_return(tx,ctx,SalesReturnUnbilledArCommand { sales_return_id,customer_id,posting_date,accounting_period_id:resolved.accounting_period_id(),accounting_period_seq:resolved.accounting_period_seq(),deferred_from_period_id:resolved.deferred_from_period_id(),voucher_id:posted.voucher_id,direction:UnbilledArDirection::Credit,net_amount:revenue_amount,gross_amount })`；两额同为零时不调用该端口、不写零额过渡子账，即使库存回收入非零并产生凭证也一样。一空一非零在第 3 步拒绝，不允许先插入空 voucher 再更新。
5. 只在前三条同步业务腿全部成功后，按输出集合回填非直运 INVENTORY 行的 `inventory_return_amount`、`stock_movement_id`，更新订单行 `returned_quantity`，把退货头置 REGISTERED 并一次写入 `registered_at` 与可空 `voucher_id`；`Posted` 写 Some，合法全零 `Skipped` 写 None。links 的累计区间/三额与仅追加 capture allocations 必须已在本事务写定，提交时由双向图逐片核对 owner role、对应静态 measure 腿及新反向 capture。随后执行幂等 `finish`，把 `sales.sales_return.registered.v1` 写入 Outbox，写同事务通知命令（如有），最后写审计终结批。审计 after 至少含可空 `voucher_id`、按退货行 id 排序的库存输出、`net_amount`、`gross_amount`、两项成本 measure 合计与三项期间值。DIRECT_EXPENSE 与直运行的两个库存回填保持空。任一步失败，退货分配、库存两账、凭证/capture、未开票应收、数量回写、审计和 Outbox 全部回滚。

`sales.sales_return.registered.v1` 是上述事务完成后的派生事件，不是过账命令。payload 固定为 `sales_return_id`、`doc_no`、`sales_order_id`、`customer_id`、可空 `source_ref`、`is_drop_ship`、可空 `voucher_id`、`lines`；`voucher_id=None` 严格表示全部会计效果为零而非待补写。每个 line 固定含 `sales_return_line_id`、`sales_order_line_id`、`item_kind`、`item_id`、`costing_mode`、`inventory_material_id`、`quantity`、`warehouse_id`、`batch_no`、`serial_nos`、`revenue_amount`、`inventory_return_amount`、`stock_movement_id`、`delivery_links`，每个 delivery link 含 `delivery_confirmation_line_id`、`quantity`、`assigned_by`。非直运 INVENTORY 行的两项库存结果非空，DIRECT_EXPENSE 与直运行为空。任何消费者都不得调用库存端口、PostingPort、UnbilledArPort 或回写 sales schema；重复消费只影响各消费者自己的派生投影。

幂等重放在进入第一步前先锁定 `(legal_entity_id,user_id,endpoint,idempotency_key)`：已成功的同键请求直接返回既有 `RegisteredSalesReturnView`，不再次解析期间或调用三条业务腿；同一退货单不同键并发由退货头锁与 REGISTERED 状态守卫串行化，后到者返回既有结果或 `SALES.SALES_RETURN.INVALID_STATE_TRANSITION`，不得重复增加 `returned_quantity`、重复入库、重复凭证、重复未开票应收、重复审计或重复事件。库存、ledger 或 finance 返回的受控错误原样透传；共享端口输出集合不满足冻结契约时只返回固定的 `PLATFORM.SYSTEM.INTERNAL_ERROR` 与关联编号，不把内部 id/金额泄露给无字段权限用户。

三个终态动作同样各自在一个事务内按“状态与投影、幂等 `finish`、Outbox、同事务通知命令、审计终结批”的顺序收口：REGISTERED 迁到 CLOSED 发 `sales.sales_return.closed.v1`，payload 含 `sales_return_id`、`doc_no`、`sales_order_id`、可空 `source_ref`、`closed_at`；仅 DRAFT/SUBMITTED 可迁到 CANCELLED 并发 `sales.sales_return.cancelled.v1`，payload 另含 `cancel_reason`；SUBMITTED 因审批驳回退回 DRAFT 发 `sales.sales_return.rejected.v1`，payload 另含 `reject_reason` 与 `approval_ref`。三者与既有的 `sales.sales_return.registered.v1` 一并登记在第 6.3 小节的事件登记表。退货单的对外创建入口固定为 `ep_contract_sales::SalesReturnCommandPort::create_sales_return`，`CreateSalesReturn`、`SalesReturnSourceRef`、`CreateSalesReturnLine`、`SalesReturnDeliveryLink` 与 `SalesReturnView` 五个 DTO 的字段按裁定 A-17 冻结，阶段 12 的服务工单退货经该端口调用，不另起第二个入口。

换货不设独立单据，按一笔退货加一笔在原订单上追加或放行的分批交付行表达，两者之间写 `sales.exchange_links`。权威写入口只有 `ep_contract_sales::SalesExchangeLinkCommandPort`：

```rust
#[async_trait::async_trait]
pub trait SalesExchangeLinkCommandPort: Send + Sync {
    async fn link_exchange(
        &self,
        tx: &mut dyn ep_foundation::port::Tx,
        ctx: &SecurityContext,
        cmd: LinkSalesExchange,
    ) -> Result<SalesExchangeLinkView, AppError>;
}
```

`LinkSalesExchange` 固定只有 `sales_return_line_id`、`replacement_delivery_schedule_id`、`idempotency_key` 三项；`SalesExchangeLinkView` 返回 link_id、两侧 id、linked_at、linked_by。阶段 12 的售后 EXCHANGE 自动与手工路径都调用该端口，不得直接写 `sales.exchange_links`，也不得只在 service schema 保存一份关联。

阶段 12 的工单登记还使用本阶段提供并由 `ep-app-sales` 实现的只读契约 `SalesOrderLineDeliveryQuery`。唯一方法固定为 `delivered_quantity(&mut dyn ep_foundation::port::Tx, ctx: &SecurityContext, sales_order_line_id: Id<SalesOrderLine>) -> Result<SalesOrderLineDeliveryView, AppError>`；返回视图固定含 `sales_order_line_id`、`sales_order_id`、`customer_id`、`contract_id`、`item_kind`、`item_id`、`delivered_quantity`。查询受法人 RLS 与对象权限约束，不可见时返回 `PLATFORM.AUTHZ.NOT_FOUND_OR_DENIED`，不提供写方法。它只给售后登记的事务内前置守卫和客户/产品带出使用；销售退货登记仍以本模块 `delivered_quantity - returned_quantity` 的权威校验为最终结论，消费方不得把该快照当成退货占用权威。

实现固定执行以下单事务算法：按 `sales_returns → sales_return_lines → sales_orders → sales_order_lines → delivery_schedules` 的顺序锁定涉及记录；退货头不得为 CANCELLED，替换分批交付行不得为 CANCELLED；两侧必须属于当前法人和同一原销售订单，订单客户相同，退货行与替换订单行的 `item_kind` 都必须为 PRODUCT 且 `item_id` 完全相同。任一不符返回 `SALES.EXCHANGE_LINK.SCOPE_MISMATCH`，零写入。随后检查两个一对一唯一键：同一 pair 重放返回既有视图；任一侧已与另一记录配对返回 `SALES.EXCHANGE_LINK.ALREADY_LINKED`。首次调用插入一行并写审计，审计 after 含两侧 id 与 idempotency_key。首版不提供解除、换边或删除动作；任一侧取消时保留历史关联，由售后登记行依 F-51 U-J-08 回到待配对并只能新建合法业务单据，不篡改原关联。

#### 4.13 电子签章编排

1. 合同全部审批节点通过后置为 PENDING_SIGNATURE。签署方式为 ESIGN 时建立 `clm.signature_requests` 并写 Outbox 条目 `clm.contract.signature_requested.v1`。
2. job-worker 消费该事件，以 `NT SERVICE\ep-worker` 身份连接固定管道 `\\.\pipe\ep-integ` 并调用 `esign.request.submit.v1`；`signature_requests.id` 同时是对外幂等键。submit 回执逐字固定为 `{request_id, external_request_id, outcome: ACCEPTED|FAILED, provider_code?, retryable}`，只允许清洗稳定码，不含服务商原始响应。core-server 与 job-worker 都不直接出网。
3. integration-gateway 经 `ep-adapter-esign` 提交签署并返回上述管道回执；job-worker 只在 `outcome=ACCEPTED` 时于自己的事务记录 `external_request_id` 并写 SUBMITTED 事件，FAILED 则按 `retryable` 进入重试或死信。请求超时取 `EP__CLM__ESIGN__REQUEST_TIMEOUT_MS`，连续失败达阈值触发熔断。gateway 无数据库连接、不消费 Outbox，也不直接写 clm、附件或审计。
4. job-worker 按 `EP__CLM__ESIGN__POLL_INTERVAL_SECONDS` 调度下一次查询，经同一管道调用 `esign.status.get.v1`。status 回执逐字固定为 `{external_request_id, status: PENDING|SIGNED|REJECTED|EXPIRED|FAILED, provider_code?, retryable, signed_files:[{file_ordinal, sanitized_name, mime_type, total_len, content_sha256}]}`；非 SIGNED 时 `signed_files` 必须为空，SIGNED 时必须至少一项且 ordinal 恰为从 0 开始的无重复连续序列，所有名称已清洗、长度与哈希形状有效。integration-gateway 每次收到 operation 才向外部服务拉取一次状态并返回，job-worker 写 POLLED 事件；外部 EXPIRED 唯一映射为内部 `signature_requests.status=FAILED` 并把 `clm.signature_events.external_status` 记为 EXPIRED，REJECTED 与 FAILED 同名映射，直到终态或超过 `EP__CLM__ESIGN__POLL_MAX_HOURS`。不设公网入站回调，理由见第 11.2 小节。
5. `status=SIGNED` 时，gateway 在服务商响应验签通过后，按 `signed_files.file_ordinal` 逐文件在同一双工连接反向执行 `esign_file.begin.v1`、`esign_file.chunk.v1`、`esign_file.end.v1`；取消或任何协议错误执行 `esign_file.abort.v1`。begin 为 `{request_id(UUIDv7), external_request_id, file_ordinal, total_files, sanitized_name, mime_type, total_len<=5368709120, content_sha256}`，其余 DTO、seq 从 0 连续、解码块 1..524288 字节、逐块与总 SHA-256、每块 ACK 且最多一块在途、10 秒 ACK/30 秒空闲/3600 秒绝对超时逐字复用 `BoundedChunkStreamV1`。gateway 只保留单块有界内存，不落盘；重试必须使用新 request_id。服务商把状态报为 SIGNED 只证明外部流程终结，不代表回传文件已安全或可发布。
6. job-worker 收到有效 SIGNED manifest 后先把内部请求置 SIGNING，再把整批文件写入阶段 3 附件流水线的临时加密对象，完成连续序号、累计长度、逐块与逐文件完整哈希、ordinal 连续性和 `total_files` 校验；随后逐文件严格执行长度/哈希复核、TYPE_SNIFF、STRUCTURE、部署模式要求的病毒扫描、电子签章验签、数据库确认与发布，不得绕过或调整顺序。`NONE` 模式按已冻结降级语义跳过病毒防护但仍执行其余步骤；`CUSTOMER_ICAP` 必须取得 PASS。只有整批文件全部进入 PUBLISHED，worker 才在一个 clm 事务逐文件写 `purpose=SIGNED_FILE`、`source_signature_request_id` 与 `source_file_ordinal`，更新 `signed_file_count`、`status=SIGNED`、`verify_result=PASSED`，写 SIGNED/VERIFIED 事件并发 `clm.contract.signed.v1`。可重试的传输或 ICAP ERROR 保持 SIGNING 并按退避换新 request_id 重试；TYPE_SNIFF/STRUCTURE/病毒命中/签章验签等非重试拒绝，或重试耗尽，置请求 FAILED，验签失败另置 `verify_result=FAILED` 并返回 `CLM.SIGNATURE_REQUEST.VERIFY_FAILED`。任一步失败均清理尚未确认的临时件，已建对象保持 QUARANTINED、不建立部分合同关联、不把合同转为 SIGNED。
6. 外部不可用时按规格第 15.1 章归类为 EXTERNAL_SYSTEM，HTTP 502，`retryable` 为真，合同保持 PENDING_SIGNATURE 并显示可重试提示；耗尽重试后进入死信与人工处理。
7. 实体印章路径不经外部系统，由用印责任人登记 `clm.seal_usages` 并上传扫描件，登记完成即满足生效守卫。

---

### 5. API 契约

全部端点遵循基线第 5 节：路径前缀 `/api/v1`，JSON 字段 snake_case，成功与失败封套固定，写请求必须带 `Idempotency-Key`，请求头固定集合含 `Authorization`、`X-Legal-Entity-Id`、`X-Device-Id`、`X-Client`，高风险操作另带 `X-Reauth-Token`。分页、排序与过滤按基线第 5.3 节，本阶段各列表端点的排序白名单在下表逐个给出。存在性泄漏一律按基线第 5.5 节返回 404 与 `PLATFORM.AUTHZ.NOT_FOUND_OR_DENIED`。
本节全部路由按裁定 A-20 逐用例声明一对常量，命名为 `<USECASE_SCREAMING>_DOMAIN` 与 `<USECASE_SCREAMING>_ACTION`，类型取阶段 1 在 ep-foundation 冻结的 `CapabilityDomain` 与 `ActionClass`，本阶段不重新定义能力域码。第 5.1 小节 CLM 端点的能力域取 `CapabilityDomain::ClmContractEsign`，声明在 `crates/contract/clm/src/capability.rs`；第 5.2 小节 SALES 端点的能力域取 `CapabilityDomain::SalesOrderFulfillment`，声明在 `crates/contract/sales/src/capability.rs`；第 5.3 小节 `/api/v1/cpq/price-authorities` 的能力域按裁定 A-20 取 `CapabilityDomain::SalesOrderFulfillment`，在阶段 5 已建的 `crates/contract/cpq/src/capability.rs` 中只追加不重定义。动作类别的取值规则为：只读查询取 `Read`，创建与修改取 `Write`，`actions/submit-for-approval`、`actions/release`、`actions/register`、`actions/confirm-delivery` 一类提交动作取 `Submit`；审批结论一律由 ep-platform-flow 的审批任务端点承载，本阶段不出现 `Approve`；本阶段无导出路由，不出现 `Export`。第 5.4 小节是进程间 IPC operation，不是 HTTP 路由、不对四端暴露，按 A-20 不声明能力常量。`xtask configdoc` 断言每个 `/api/v1/` 路由都能解析到一对常量，缺失即构建失败。

角色包的出厂绑定按 F-51 固定：`sales.delivery.create` 只绑定 `WAREHOUSE_USER`；`sales.delivery.confirm` 只绑定 `SALES_MANAGER`、`PROJECT_MANAGER`。`TECHNICIAN` 只获得交付证据登记与 `clm.contract.read_technical_summary`：后者复用合同详情查询但经字段投影永久排除合同总额、行单价、成本与毛利，仅返回客户、合同编号、产品/服务、交付节点、安装/调试要求和非价格条款摘要。`TECHNICIAN` 不绑定最终交付确认、收入确认或任何财务审批能力。

#### 5.1 CLM 端点

| 方法与路径 | 请求要点 | 响应要点 | 主要错误码 | 幂等语义 | 权限 |
|---|---|---|---|---|---|
| POST /api/v1/clm/contracts | 合同头字段，行可空 | 合同视图，status 为 DRAFT | PLATFORM.REQUEST.INVALID_PAYLOAD | 幂等键四元组，重放回首次结果 | clm.contract.create |
| GET /api/v1/clm/contracts | 排序白名单 created_at、doc_no、valid_to、total_amount；过滤 status、customer_id、contract_type_id、valid_to、owner_user_id | 分页列表，默认排序 created_at desc, id desc，默认筛选最近 3 个自然月 | 无 | 读请求无幂等键 | clm.contract.read |
| GET /api/v1/clm/contracts/{id} | 无 | 合同头行条款节点期次附件的完整视图 | PLATFORM.AUTHZ.NOT_FOUND_OR_DENIED | 无 | clm.contract.read |
| PATCH /api/v1/clm/contracts/{id} | 携带 row_version | 更新后的视图 | PLATFORM.CONCURRENCY.STALE_VERSION | 幂等键 | clm.contract.update |
| PUT /api/v1/clm/contracts/{id}/lines | 全量替换合同行 | 行列表与汇总金额 | PLATFORM.REQUEST.INVALID_PAYLOAD、CPQ.PRICE_AUTHORITY.MULTIPLE_PRICE_LIST_HITS、CPQ.PRICE_AUTHORITY.NOT_CONFIGURED | 幂等键 | clm.contract.update |
| PUT /api/v1/clm/contracts/{id}/terms | 关键条款结构化字段与正文 | 条款视图 | PLATFORM.REQUEST.INVALID_PAYLOAD | 幂等键 | clm.contract.update |
| PUT /api/v1/clm/contracts/{id}/milestones | 交付节点清单全量替换 | 节点列表 | PLATFORM.REQUEST.INVALID_PAYLOAD | 幂等键 | clm.contract.update |
| PUT /api/v1/clm/contracts/{id}/payment-schedules | 期次列表全量替换 | 期次列表 | CLM.CONTRACT.PAYMENT_SCHEDULE_SUM_MISMATCH | 幂等键 | clm.contract.update |
| POST /api/v1/clm/contracts/{id}/attachments | 附件对象 id 与用途 | 关联列表 | PLATFORM.REQUEST.INVALID_PAYLOAD | 幂等键 | clm.contract.update |
| POST /api/v1/clm/contracts/{id}/annotations | 批注锚点与正文 | 批注视图 | PLATFORM.REQUEST.INVALID_PAYLOAD | 幂等键 | clm.contract.annotate |
| POST /api/v1/clm/contracts/{id}/actions/apply-template | 模板 id 与版本号 | 套用后的条款与节点 | CLM.CONTRACT_TEMPLATE.VERSION_NOT_PUBLISHED | 幂等键 | clm.contract.update |
| POST /api/v1/clm/contracts/{id}/actions/submit-for-approval | row_version | 校验明细与审批实例 id | CLM.CONTRACT.THREE_INFO_INCOMPLETE、CLM.CONTRACT.LINE_DELIVERY_DATE_OUT_OF_RANGE、CLM.CONTRACT.CUSTOMER_INACTIVE、CLM.CONTRACT.PRODUCT_INACTIVE、SALES.SALES_ORDER.CREDIT_LIMIT_EXCEEDED、CLM.CONTRACT.INVALID_STATE_TRANSITION | 幂等键；重复提交返回首次校验结论 | clm.contract.submit |
| POST /api/v1/clm/contracts/{id}/actions/void | 原因 | 状态视图 | CLM.CONTRACT.INVALID_STATE_TRANSITION | 幂等键 | clm.contract.void |
| POST /api/v1/clm/contracts/{id}/actions/retry-signature | 无 | 签署请求视图 | CLM.SIGNATURE_REQUEST.EXTERNAL_UNAVAILABLE | 幂等键 | clm.contract.sign |
| POST /api/v1/clm/contracts/{id}/actions/register-seal-usage | 印章名、用印时间、扫描件附件 id | 用印记录 | CLM.SEAL_USAGE.SCAN_REQUIRED | 幂等键 | clm.contract.seal |
| POST /api/v1/clm/contracts/{id}/actions/reject-signature | 原因 | 状态视图 | CLM.CONTRACT.INVALID_STATE_TRANSITION | 幂等键 | clm.contract.sign |
| POST /api/v1/clm/contracts/{id}/actions/make-effective | 必带 X-Reauth-Token | 状态视图与派生批次 id | CLM.CONTRACT.SIGNATURE_NOT_COMPLETED、PLATFORM.AUTHZ.REAUTH_REQUIRED、PLATFORM.SOD.SELF_APPROVAL_FORBIDDEN | 幂等键；重放不重复触发派生 | clm.contract.make_effective |
| POST /api/v1/clm/contracts/{id}/actions/amend | 变更原因 | 新版本草稿视图 | CLM.CONTRACT.AMEND_ON_NON_EFFECTIVE、CLM.CONTRACT.DERIVATION_IN_PROGRESS | 幂等键 | clm.contract.amend |
| POST /api/v1/clm/contracts/{id}/actions/renew | 新有效期与可调整字段 | 续签合同草稿视图 | CLM.CONTRACT.RENEW_SOURCE_NOT_ELIGIBLE | 幂等键 | clm.contract.renew |
| POST /api/v1/clm/contracts/actions/merge | 来源合同 id 列表与新合同头 | 新合同草稿视图 | CLM.CONTRACT.MERGE_SOURCE_NOT_ELIGIBLE、CLM.CONTRACT.MERGE_CUSTOMER_MISMATCH | 幂等键 | clm.contract.merge |
| POST /api/v1/clm/contracts/{id}/actions/terminate | 终止原因、row_version；必须完成 TERMINATION 审批 | TERMINATING 状态视图、同一 impact_assessment_id 与七类处置项摘要；幂等重放不建第二批 | CLM.CONTRACT.INVALID_STATE_TRANSITION | 幂等键 | clm.contract.terminate |
| GET /api/v1/clm/contracts/{id}/impact-assessment | 无 | 经 `ep_platform_impact::ImpactAssessmentQuery::by_source` 返回当前批次、实项与目录占位项，人工项含决策码允许集、结果 id 必填/必空形状、待办与当前决策三字段；无批次返回空视图 | PLATFORM.AUTHZ.NOT_FOUND_OR_DENIED | 无 | clm.contract.read |
| GET /api/v1/clm/contracts/{id}/versions | 无 | 版本列表与差异 | 无 | 无 | clm.contract.read |
| GET /api/v1/clm/contracts/{id}/derivations | 无 | 派生批次与逐项状态 | 无 | 无 | clm.contract.read |
| GET /api/v1/clm/contracts/{id}/validations | 无 | 校验运行与逐项快照 | 无 | 无 | clm.contract.read |
| GET /api/v1/clm/contracts/{id}/performance | 无 | 履约记录投影，含交付节点进度、收付款期次进度、派生订单交付进度、关联退货换货与工单；各期 `billed_amount` 只取 `ReceiptPlanBillingQuery::billing_by_period`，不读 clm 金额副本 | 无 | 无 | clm.contract.read |
| GET、POST、PATCH /api/v1/clm/contract-templates 与 /api/v1/clm/clauses | 档案维护 | 档案视图 | PLATFORM.REQUEST.INVALID_PAYLOAD | 幂等键 | clm.template.manage |

审批任务的通过、驳回与退回不在本阶段设端点，一律由 ep-platform-flow 的审批任务端点承载，CLM 只注册四条审批链定义与结论回调处理器。这一处理避免出现第二套审批入口。

#### 5.2 SALES 端点

| 方法与路径 | 请求要点 | 响应要点 | 主要错误码 | 幂等语义 | 权限 |
|---|---|---|---|---|---|
| GET /api/v1/sales/sales-orders | 排序白名单 created_at、doc_no、promised_to、total_amount_with_tax；过滤 status、customer_id、source_contract_id、order_type | 分页列表 | 无 | 无 | sales.order.read |
| GET /api/v1/sales/sales-orders/{id} 与 /{id}/lines | 无 | 订单头行与分批交付行 | 无 | 无 | sales.order.read |
| POST /api/v1/sales/sales-orders/{id}/actions/release | 无 | 校验明细与订单状态 | SALES.SALES_ORDER.CREDIT_LIMIT_EXCEEDED、SALES.SALES_ORDER.STOCK_NOT_AVAILABLE、SALES.SALES_ORDER.INVALID_STATE_TRANSITION | 幂等键 | sales.order.release |
| POST /api/v1/sales/sales-orders/{id}/actions/cancel | 原因 | 状态视图 | SALES.SALES_ORDER.INVALID_STATE_TRANSITION | 幂等键 | sales.order.cancel |
| POST /api/v1/sales/sales-orders/{id}/actions/close-remaining | 原因 | 状态视图与关闭数量 | SALES.SALES_ORDER.INVALID_STATE_TRANSITION | 幂等键 | sales.order.close |
| POST /api/v1/sales/sales-orders/{id}/actions/submit-change | 变更行与原因 | 变更单视图与审批实例 id | SALES.SALES_ORDER.CHANGE_IN_PROGRESS、SALES.SALES_ORDER.DELIVERED_QTY_EXCEEDED、SALES.SALES_ORDER.PRICE_CHANGE_NOT_ALLOWED | 幂等键 | sales.order.change |
| GET /api/v1/sales/sales-orders/{id}/versions | 无 | 版本快照列表 | 无 | 无 | sales.order.read |
| POST /api/v1/sales/sales-order-lines/{id}/delivery-schedules/actions/split | 分批数量、约定日期、仓库 | 分批交付行列表 | SALES.DELIVERY_SCHEDULE.SPLIT_SUM_MISMATCH、SALES.DELIVERY_SCHEDULE.NOT_SPLITTABLE | 幂等键 | sales.order.schedule |
| POST /api/v1/sales/sales-order-lines/{id}/delivery-schedules/actions/merge | 待合并的分批行 id 列表 | 分批交付行列表 | SALES.DELIVERY_SCHEDULE.NOT_SPLITTABLE | 幂等键 | sales.order.schedule |
| GET /api/v1/sales/delivery-schedules | 排序白名单 promised_date、created_at；过滤 status、customer_id、sales_order_id、promised_date | 分页列表，供交付经办与交付指标取数 | 无 | 无 | sales.order.read |
| POST /api/v1/sales/delivery-confirmations | 交付确认单头与行，行按分批交付行选取 | 交付确认单视图，status 为 DRAFT | PLATFORM.REQUEST.INVALID_PAYLOAD、SALES.DELIVERY_CONFIRMATION.QTY_EXCEEDS_SCHEDULED | 幂等键 | sales.delivery.create |
| POST /api/v1/sales/delivery-confirmations/{id}/actions/confirm-delivery | row_version；本端点按第 11.5 小节随第三批与阶段 10 同批注册 | 状态视图，含 voucher_id 与逐行 cogs_amount | SALES.DELIVERY_CONFIRMATION.INVALID_STATE_TRANSITION、SALES.DELIVERY_CONFIRMATION.QTY_EXCEEDS_SCHEDULED；三腿透传的错误按其所属模块的错误码原样返回，不在本模块重新编码 | 幂等键；重放不重复过账也不重复发事件 | sales.delivery.confirm |
| GET /api/v1/sales/delivery-confirmations | 排序白名单 posting_date、created_at、doc_no；过滤 status、customer_id、sales_order_id、posting_date | 分页列表 | 无 | 无 | sales.delivery.read |
| GET /api/v1/sales/delivery-confirmations/{id} | 无 | 交付确认单头行与三腿回填结果 | PLATFORM.AUTHZ.NOT_FOUND_OR_DENIED | 无 | sales.delivery.read |
| POST /api/v1/sales/sales-returns | 退货单头行与交付确认关联 | 退货单草稿 | SALES.SALES_RETURN.DELIVERY_LINK_REQUIRED、SALES.SALES_RETURN.QTY_EXCEEDS_DELIVERED | 幂等键 | sales.return.create |
| POST /api/v1/sales/sales-returns/{id}/actions/submit | row_version | 状态视图与审批实例 id | SALES.SALES_RETURN.INVOICE_NOT_CREDIT_NOTED | 幂等键 | sales.return.submit |
| POST /api/v1/sales/sales-returns/{id}/actions/register | 记账日期、row_version | `RegisteredSalesReturnView`，含 voucher_id 与逐行 inventory_return_amount、stock_movement_id | SALES.SALES_RETURN.INVOICE_NOT_CREDIT_NOTED、SALES.SALES_RETURN.QTY_EXCEEDS_DELIVERED、SALES.SALES_RETURN.DELIVERY_LINK_REQUIRED；库存、ledger、finance 三腿的错误按所属模块原样返回 | 幂等键；重放不重复入库、凭证、未开票应收、数量回写、审计或事件 | sales.return.register |
| POST /api/v1/sales/sales-returns/{id}/actions/cancel 与 /actions/close | 原因；cancel 仅 DRAFT/SUBMITTED，close 仅 REGISTERED | 状态视图 | SALES.SALES_RETURN.INVALID_STATE_TRANSITION | 幂等键；已 REGISTERED 的 cancel 恒拒绝且不冲销任何效果 | sales.return.manage |
| POST /api/v1/sales/sales-returns/{id}/actions/link-exchange | `sales_return_line_id` 与 `replacement_delivery_schedule_id`；路径 id 必须是该退货行所属退货单 | 换货关联视图 | SALES.EXCHANGE_LINK.SCOPE_MISMATCH、SALES.EXCHANGE_LINK.ALREADY_LINKED | `Idempotency-Key` 原样进入 LinkSalesExchange；同 pair 重放返回原视图 | sales.return.manage |
| GET /api/v1/sales/credit-exposures | 查询参数 customer_id | 信用额度、三部分占用明细、可用额度 | PLATFORM.AUTHZ.NOT_FOUND_OR_DENIED | 无 | sales.credit.read |
| GET、PUT /api/v1/sales/credit-policies | 法人级策略 | 策略视图 | PLATFORM.REQUEST.INVALID_PAYLOAD | 幂等键 | sales.credit.manage |

#### 5.3 CPQ 端点

| 方法与路径 | 说明 |
|---|---|
| GET、POST、PATCH /api/v1/cpq/price-authorities | 价格权限档案的维护，排序白名单 code、created_at，过滤 subject_kind、is_active |

#### 5.4 integration-gateway 内部 IPC 契约

> **身份核验现行替代。** 下段旧字面「客户端 PID token 复核」不得实现：server 在读取任何应用字节前执行 `ImpersonateNamedPipeClient`→`OpenThreadToken` 核验服务 SID/账户，并在所有分支 `RevertToSelf`，PID 只作审计关联；client 以 Identification SQOS 打开并在发送前核验 server 进程 token，每次重连重新核验。仅 bootstrap 首实例取 `first_pipe_instance(true)`，后续/补位实例取 `false`。

电子签章使用固定管道 `\\.\pipe\ep-integ`：worker→gateway 只有 `esign.request.submit.v1`、`esign.status.get.v1`，gateway→worker 在同一双工连接只有 `esign_file.begin.v1`、`esign_file.chunk.v1`、`esign_file.end.v1`、`esign_file.abort.v1`。服务端账户、DACL、服务端冒充后以线程 token 核验客户端服务 SID/账户、客户端发送前核验 server 进程 token、4 字节大端长度加 JSON 普通帧、1 MiB 上限及 `BoundedChunkStreamV1` 逐项复用 `docs/config-reference.md` 第 2.2 节；PID 只记审计，本阶段不定义第二种 framing。发起账户只有 `NT SERVICE\ep-worker`，gateway 的入站 operation allowlist 必须拒绝 core、ops、其他服务账户与任意本地进程调用两个 e-sign 请求 operation；worker 也必须校验反向帧来自已认证的同一 gateway 连接且 operation 属于四项 `esign_file.*`。gateway 不开 8082 或其他内部 HTTP 监听，不登记 endpoint 配置，也不存在 `/internal/v1/esign/*` 兼容路径。

integration-gateway 的数据库能力固定为零：部署包与配置模型均不含 `ep_app_rw` 或其他数据库凭据、数据库连接/连接池、文件库或 KMS 凭据；该进程不消费 Outbox，不读写附件对象/元数据、clm 表或其他业务表。签章普通回执和反向文件流只是管道消息，唯一持久化方是 job-worker。

电子签章持久凭据因此不使用 `secret://`。唯一引用固定为 `wincred://esign/api`，类型为 `WindowsCredentialRef`；其 parser、formatter 与 Win32 `TargetName` 逐字映射只复用 `docs/config-reference.md` 第 5 节，不允许签章 adapter 自行截取 path 或加前缀。实际 blob 只存在由 SCM 正常启动并加载服务账户 profile 的 `NT SERVICE\ep-integ` current token/logon session 的 `CRED_TYPE_GENERIC` Credential Manager，1..2560 bytes，后一个上限逐字取 Windows SDK `CRED_MAX_CREDENTIAL_BLOB_SIZE = 5*512`。禁止管理员模拟该 token或手工 `LoadUserProfile`。初始化、轮换和删除一律复用 `docs/config-reference.md` 第 5 节的 `ep-secretctl` 本地维护协议：SCM control code 200 只使 gateway 在 60 秒单次窗口创建 `\\.\pipe\ep-integ-secretctl`，严格 DACL/本地完整提升管理员 token/签名 PE digest/双人 CMS grant 全部通过后，gateway 才以自身 token 执行 CredWrite/CredDelete。secret 只经一次 binary frame 进入 gateway，不经 HTTP、ServerAdmin、普通 `ep-integ` 业务管道、argv、env 或文件；写后由 gateway 做 provider probe，失败时 CREATE 删除新值、ROTATE 回写旧值、DELETE 恢复旧值，任一恢复失败都使签章能力保持关闭并写高严重度 Event Log。第一次 Win32 mutation 前还必须落配置参考冻结的非秘密 write-through intent；APPLYING 后异常退出由下次 SCM 启动重建 CLOSED_FAILED，禁止把残留 credential 当成功并须同 target/purpose 新双人 grant 纠正。维护前停止新签章 egress 并排空在途；正常服务重启后必须仍能读取同一 target。

#### 5.5 阶段 8 A2 的同批注册

本阶段不另定义库存路径或响应 DTO，只负责最终接线并启用阶段 8 已冻结的 `GET /api/v1/inventory/available-quantities`。请求固定为 `material_id` 必填、`warehouse_id` 可选；响应逐仓库返回 `warehouse_id`、`material_id`、`quantity`、`reserved_quantity`、`available_quantity`，meta 返回 `total_quantity` 与 `total_available_quantity`。权限与能力常量沿用阶段 8 的 `inventory.stock_balance:read`、`CapabilityDomain::InventoryLedgerScan`、`ActionClass::Read`；handler 只调用本阶段真实装配的 `AvailabilityQueryPort`，不在 handler 内重写聚合。任一依赖未装配时启动路由表不含 A2，而不是运行时返回零值或 501。

#### 5.6 版本化

本阶段全部端点为 v1 首次发布。后续新增可选请求字段、新增响应字段与新增枚举取值的接收侧不升主版本；客户端必须容忍未知的 `order_type`、`status` 与 `pending_release_reason` 取值并按未知降级展示。

---

### 6. 并发与事务边界

#### 6.1 事务清单

| 用例 | 事务内容 | 隔离级别 | 锁策略 |
|---|---|---|---|
| 合同草稿保存与修改 | 合同头行条款节点期次的写入、幂等 finish、审计终结批 | READ COMMITTED | 乐观锁 row_version |
| 合同提交审批 | 五项校验取数、校验记录写入、状态迁移、审批实例建立、幂等 finish、Outbox、审计终结批 | READ COMMITTED | `customer_credit_controls` 行 FOR UPDATE |
| 合同生效 | 重新认证与审批结论校验、状态迁移、版本快照、幂等 finish、Outbox、审计终结批 | READ COMMITTED | 合同行乐观锁 |
| 派生批次建立 | 批次行与全部派生项的写入 | READ COMMITTED | 批次唯一约束 |
| 单个派生项执行 | 调用目标模块用例、写回 target_doc_id、审计终结批 | READ COMMITTED | 派生项行 FOR UPDATE SKIP LOCKED |
| 销售订单库存确认与放行 | 后四项校验、库存需求 INACTIVE→CONFIRMED/RELEASED、订单状态迁移、幂等 finish、Outbox、审计终结批 | READ COMMITTED | `customer_credit_controls` 行 FOR UPDATE；库存组合按固定顺序取事务级 advisory lock，锁内重算唯一可用量 |
| 销售订单取消或剩余量关闭 | 订单与行终态、库存需求置 INACTIVE、信用与可用量重算、幂等 finish、Outbox、审计终结批 | READ COMMITTED | 订单与订单行 FOR UPDATE；受影响库存组合按固定顺序取事务级 advisory lock |
| 订单变更审批通过 | 锁内比较新旧需求增量、旧版本快照、变更应用、版本号递增、open_amount 与需求状态重算、幂等 finish、Outbox、审计终结批 | READ COMMITTED | 订单与订单行 FOR UPDATE；旧新库存组合并集按固定顺序取事务级 advisory lock |
| 分批交付行拆分与合并 | 分批行全量重写、守恒校验、幂等 finish、审计终结批 | READ COMMITTED | 订单行 FOR UPDATE |
| 交付确认单登记 | 交付确认单头行写入、对分批交付行未交付量的校验、幂等 finish、审计终结批 | READ COMMITTED | 分批交付行 FOR UPDATE |
| 交付确认过账 | 会计期间解析、库存腿、过渡科目腿、凭证腿、分批行/订单行/订单头同步回写、需求即时减少、单据置 CONFIRMED、幂等 finish、Outbox、同事务通知命令、审计终结批 | READ COMMITTED | 交付确认单、分批交付行与订单行 FOR UPDATE；受影响库存组合按固定顺序取事务级 advisory lock |
| 合同交付节点事件消费 | inbox 去重行、合同交付节点确认、审计终结批 | READ COMMITTED | 合同节点行 FOR UPDATE；不回写 sales schema |
| 销售退货登记 | 会计期间解析；原交付累计差额与 current live capture fragments 锁定/分配；非直运 INVENTORY 行按原交付实际成本同步入库；按 cost role 分腿的销售退货凭证、未开票应收冲回；退货行库存结果、退货头与订单行 returned_quantity 同步回写；幂等 finish、Outbox、同事务通知命令、审计终结批 | READ COMMITTED | 退货头、退货行、订单行、交付关联、原交付行与 costing root/live 按固定顺序 FOR UPDATE；库存组合按法人、仓库、物料固定顺序取事务级 advisory lock |
| `CLM_TERM_SALES_ORDER_LINE` 处置 | 取消/关闭订单行及 PENDING 分批行、释放需求、重算订单头、必要的订单终态事件 Outbox、审计终结批 | READ COMMITTED | 先按组合取可用量 advisory lock，再按分批行、订单行、订单头 id 固定顺序 FOR UPDATE |
| `CLM_TERM_MILESTONE` 处置 | 锁后重检、节点取消、审计终结批 | READ COMMITTED | 目标交付节点 FOR UPDATE |
| `CLM_TERM_DELIVERY_CONFIRMATION` 人工决策 | 校验决策码与结果 id 形状，锁后核对交付确认、退货单状态与退货行关联；不自动动账 | READ COMMITTED | 交付确认单、退货单、退货行与关联行按 id 升序 FOR UPDATE |
| 合同合并 | 新合同建立、来源合同置 VOID、merge_links、幂等 finish、审计终结批 | READ COMMITTED | 来源合同行 FOR UPDATE |

内部对账与关账前强制校验涉及本阶段数据时，由阶段 9b 注册的校验项按基线第 8.4 节在单个 REPEATABLE READ 事务或由其导出的快照上执行，本阶段按裁定 A-06 不实现也不注册任何 `ReconCheck`。

事务预算按基线第 10.3 节：业务事务不超过 5 秒，读写池 `statement_timeout` 10 秒，`lock_timeout` 3 秒，`idle_in_transaction_session_timeout` 15 秒。事务内禁止外部 HTTP 调用、附件正文读写、发送通知与长时计算，因此签署提交、模板渲染与站内通知一律经 Outbox 转出事务之外。

所有写用例采用同一收口顺序：先按各算法已冻结的引用顺序完成业务事实、子账、凭证和同步投影，再执行幂等 `finish`，再写 Outbox，再写确需同事务落库的通知命令，最后调用 `AuditWriter::append_terminal` 批量落审计；不存在的类别跳过，但后缀不得调换。`append_terminal` 封印 `Tx`，其后任何 SQL `query/execute`、仓储写入或跨模块端口调用都必须以内置不变量错误拒绝并令整个事务回滚，唯一允许的后继动作是 `UnitOfWork::commit`。共享跨模块契约测试 `audit_terminal_seals_tx` 以交付确认和销售退货两条多端口路径为夹具：审计后分别尝试本地仓储、`PostingPort` 与 `UnbilledArPort` 写入，均应失败、审计后零新增行、事务整体回滚；正常路径由 recording transaction 断言审计批是 commit 前最后一批数据库执行。

#### 6.2 幂等键

- 全部写端点必须带 `Idempotency-Key`，作用域为法人、用户、端点、键值四元组，存储在 `platform_msg.idempotency_keys`，与业务写入同事务。
- 销售退货登记的幂等结果必须持久化 `sales_return_id`、可空 `voucher_id` 与按退货行 id 排序的 `inventory_return_amount/stock_movement_id`；成功重放直接返回该结果，不重新调用库存、ledger 或 finance。`voucher_id=None` 只能来自首次全零 `Skipped` 的已登记终态，不能被解释为待补写。退货头状态锁是不同幂等键并发的第二道防线。
- 派生项的幂等键取派生项自身 id，同时由 `ux_contract_derivation_items_unique` 提供第二道保证。
- Outbox 事件的消费幂等由 `platform_msg.inbox_consumptions(consumer, event_id)` 保证，本阶段的消费者名固定为 `clm.derivation`、`clm.milestone_confirm`；销售订单交付回写不经事件消费。
- 电子签章提交的幂等键取 `signature_requests.id`，随请求传给外部系统，避免重复签署。

#### 6.3 与 Outbox 的关系

本阶段发出的领域事件全部与业务状态和审计事件处于同一数据库事务，但写入次序固定为业务/子账/凭证/投影、幂等 `finish`、`platform_msg.outbox_events`、同事务通知命令、审计终结批。事件信封字段按基线第 6.1 节完整填写，其中 `security_level` 与 `data_scope_tags` 自源记录继承，`posting_date` 在交付确认与销售退货登记两类事件上非空，分别取交付确认单与退货单的 `posting_date`，`accounting_period_id` 取 PostingPort 返回值；合同与订单类事件不产生凭证，`posting_date` 与 `accounting_period_id` 为空，这与基线第 6.1 节对可过账事件的要求不冲突，因为该两项是关账受理前提的可枚举依据，只对会产生凭证的事件有意义。

`sales.sales_return.registered.v1` 只能在第 4.12 小节的同步库存、凭证、未开票应收与销售回写全部完成后写出；它携带已完成结果供 service、reporting、search、notify 派生使用，不承担库存或财务命令。事件目录与 wiring 静态检查必须同时断言该事件不存在 inventory、ledger、finance 或 sales 写回消费者。

按裁定 A-21，`sales.delivery.confirmed.v1` 与 `sales.sales_return.registered.v1` 两条在 `ledger.posting_trigger_event_types` 中的登记行由阶段 9a 的种子迁移一次写入，本阶段不新增任何 `backfill_posting_trigger_event_types` 迁移，也不做启动自检、`--check` 静态断言与关账受理前置校验；登记表一致性的承接方只有两条，一是 `xtask configdoc` 从 `docs/event-catalog.md` 生成阶段 9a 的第 14 号种子迁移并在 CI 中与仓库文件逐字比对，二是阶段 3b 的 `event-catalog-consistent` 自检项且不通过时停止派发未登记事件类型；本阶段既不回填也不判读该表。登记行与上述 `posting_date` 非空两者齐备，这两类事件才按裁定 C-28 的受理前提二计入待过账积压。

本阶段的事件总数固定为 **20**：原有九个具名事件、原先未命名的九个合同/订单迁移事件，以及 F-10 新增的两个终止事件。二十个现已全部逐项命名，代码常量与目录只接受下表集合，不再保留“实现前再命名”或实现方自选事件粒度的槽位。第 1 节与第 9 节只引用本小节，不另写数字。

| 事件 | aggregate_type | 产生位 | posting_date |
|---|---|---|---|
| clm.contract.submitted.v1 | clm.contracts | DRAFT 提交进入 PENDING_APPROVAL 的事务 | 空 |
| clm.contract.rejected.v1 | clm.contracts | 审批或签署拒绝使合同进入 REJECTED 的事务 | 空 |
| clm.contract.effective.v1 | clm.contracts | 第 4.8 小节的生效事务 | 空 |
| clm.contract.derivation_completed.v1 | clm.contracts | 第 4.8 小节派生编排第 6 步 | 空 |
| clm.contract.signature_requested.v1 | clm.contracts | 第 4.13 小节第 1 步 | 空 |
| clm.contract.signed.v1 | clm.contracts | 第 4.13 小节第 5 步 | 空 |
| clm.contract.completed.v1 | clm.contracts | 履约与结清守卫通过并进入 COMPLETED 的事务 | 空 |
| clm.contract.voided.v1 | clm.contracts | DRAFT 或 REJECTED 且无派生记录时作废的事务 | 空 |
| clm.contract.terminated.v1 | clm.contracts | F-10 终止审批通过、合同进入 TERMINATING 的事务 | 空 |
| clm.contract.termination_completed.v1 | clm.contracts | 影响面批次闭合、合同进入 TERMINATED 的事务 | 空 |
| sales.sales_order.created.v1 | sales.sales_orders | 合同派生或人工建单事务 | 空 |
| sales.sales_order.released.v1 | sales.sales_orders | 信用与库存守卫通过并进入 RELEASED 的事务 | 空 |
| sales.sales_order.changed.v1 | sales.sales_orders | 订单变更审批应用、版本与需求重算事务 | 空 |
| sales.sales_order.closed.v1 | sales.sales_orders | 剩余量按关闭原因关闭的事务 | 空 |
| sales.sales_order.cancelled.v1 | sales.sales_orders | 零交付订单取消并释放需求的事务 | 空 |
| sales.delivery.confirmed.v1 | sales.delivery_confirmations | 第 4.11 小节的 confirm_delivery 事务 | 非空，取交付确认单的 posting_date |
| sales.sales_return.registered.v1 | sales.sales_returns | 第 4.12 小节的登记动作 | 非空，取退货单的 posting_date |
| sales.sales_return.closed.v1 | sales.sales_returns | 第 4.12 小节 REGISTERED 迁到 CLOSED | 空 |
| sales.sales_return.cancelled.v1 | sales.sales_returns | 第 4.12 小节 DRAFT/SUBMITTED 在登记前迁到 CANCELLED | 空 |
| sales.sales_return.rejected.v1 | sales.sales_returns | 第 4.12 小节 SUBMITTED 因审批驳回退回 DRAFT | 空 |

销售退货后三个终态事件的 payload 字段按裁定 A-17 固定，见第 4.12 小节。F-10 两个终止事件的 payload、唯一消费者与 `produces_voucher=false` 逐字取 `docs/event-catalog.md`：`terminated` 只由 `platform.impact_assess` 消费，`termination_completed` 只在 `impact_assessments.status='DONE' and item_done=item_total` 后产生；两者都不进入 `ledger.posting_trigger_event_types`。

#### 6.4 失败重试与补偿

- 数据库序列化失败 40001 与死锁 40P01 由数据访问层统一重试 3 次，退避 50、150、450 毫秒，只对尚未产生外部可见副作用的事务重试。
- 派生项失败按八档退避重试，耗尽进入死信，人工修复后可重放。
- 派生批次不做整批回滚。理由是已成功派生的销售订单可能已被下游引用，回滚会造成比部分派生更严重的不一致；补偿方式是把失败项修复后重放，或由人工在合同上执行终止并按第 11.3 小节的处置清单逐项处理已派生单据。
- 电子签章失败按超时、退避、熔断三级处理，熔断打开期间新的签署请求继续留在 worker 的 Outbox 消费重试链，不占用外部连接；命名管道忙按共享 IPC 规则重试，DACL/账户/operation 不匹配一律失败关闭且不降级到回环 HTTP。反向文件流乱序、重复、缺块、长度或哈希不符立即 abort 并清理 worker 临时件，重试使用新 request_id；gateway 不写死信或任何数据库记录。
- `clm.milestone_confirm` 消费失败时不吞掉异常，按至少一次语义重投；因合同节点版本冲突等原因无法推进时写入 `platform_msg.dead_letters` 并在运维中心可枚举，由人工修复后重投，不静默忽略。销售侧订单与需求已在确认原事务内完成，不受该消费者失败影响；本阶段不产生对账差异事项，理由见第 6.1 小节。

#### 6.5 必测并发场景

本阶段承担基线第 8.4 节六组并发场景中的两组，另自行追加三组。

1. 同一合同或同一订单的乐观锁冲突（基线第一组）。
2. 同一客户的并发下单与信用额度占用（基线第四组）。
3. 同一合同的重复生效提交，验证只产生一个派生批次。
4. 同一派生批次的重复投递不少于 3 次，验证派生单据只产生一次。
5. 订单变更审批通过与交付确认事件回写的交叠，验证已交付数量不被变更覆盖。

---

### 7. 配置项

全部键在 `EP__` 前缀下，层级用双下划线，反序列化开启 `deny_unknown_fields`。运行期可变的业务参数不进配置文件，信用超额策略、提醒提前量与审批链定义一律存事务数据库并经配置发布通道签名发布，按基线第 7.1 节。

| 键名 | 类型 | 默认值 | 生效方式 |
|---|---|---|---|
| EP__CLM__ESIGN__BASE_URL | string | 无默认 | 启动加载；缺失时签章能力以降级状态启动并在运维中心登记暴露窗口 |
| EP__CLM__ESIGN__CREDENTIAL_REF | WindowsCredentialRef | `wincred://esign/api` | 启动加载；实际值只由 `ep-integ` 在本地维护窗口写入/轮换 |
| EP__CLM__ESIGN__REQUEST_TIMEOUT_MS | u64 | 10000 | 重启生效 |
| EP__CLM__ESIGN__POLL_INTERVAL_SECONDS | u64 | 60 | 重启生效 |
| EP__CLM__ESIGN__POLL_MAX_HOURS | u64 | 168 | 重启生效 |
| EP__CLM__ESIGN__CIRCUIT_BREAKER__FAILURE_THRESHOLD | u32 | 5 | 重启生效 |
| EP__CLM__ESIGN__CIRCUIT_BREAKER__OPEN_SECONDS | u64 | 120 | 重启生效 |
| EP__CLM__DERIVATION__ITEM_TIMEOUT_MS | u64 | 5000 | 重启生效 |
| EP__CLM__DERIVATION__MAX_ITEMS_PER_CONTRACT | u32 | 2000 | 重启生效 |
| EP__CLM__TEMPLATE__RENDER_TIMEOUT_MS | u64 | 8000 | 重启生效 |
| EP__CLM__CONTRACT__MAX_LINES | u32 | 500 | 重启生效 |
| EP__SALES__CREDIT__EXPOSURE_QUERY_TIMEOUT_MS | u64 | 2000 | 重启生效 |
| EP__SALES__ORDER__MAX_LINES | u32 | 500 | 重启生效 |
| EP__SALES__DELIVERY_SCHEDULE__MAX_PER_LINE | u32 | 60 | 重启生效 |
| EP__SALES__RETURN__MAX_LINES | u32 | 200 | 重启生效 |

启动自检的追加项：本阶段不追加任何启动自检项。原拟的第一项即 `clm` 与 `sales` 两个 schema 的迁移历史版本比对，已由基线第 7.3 节的 `migration-version-matched` 覆盖全部 schema，不再重复注册。原拟的第二项按裁定 A-21 判读 `ledger.posting_trigger_event_types` 的数据行，属判读业务数据的自检，一律不作启动闸门，该项整项撤销：本阶段不做启动自检、不做 `--check` 静态断言、也不挂关账受理前置校验，登记表一致性按第 6.3 小节由 `xtask configdoc` 在 CI 中的逐字比对与阶段 3b 的 `event-catalog-consistent` 两条承接。正常启动路径不再因该项拒绝服务。integration-gateway 在 `EP__CLM__ESIGN__BASE_URL` 缺失时不退出，以降级状态启动。

---

### 8. 测试计划

#### 8.1 单元测试

覆盖下列分支，全部位于被测 crate 内，不触库、不触网、不取真实时间。

- 合同状态机：十个状态、第 4.2 节表中的十六条允许行逐条一个用例（其中 EFFECTIVE 与 TERMINATING 各有一条状态保持自环）；其余起终点组合用参数化用例穷举并断言返回 `CLM.CONTRACT.INVALID_STATE_TRANSITION`。
- 订单状态机与分批交付行状态机同上。
- 退货单状态机：五类动作、六条合法起终点边逐条通过；REGISTERED→CANCELLED、CLOSED→CANCELLED、CANCELLED→任意状态及所有其他非法组合穷举返回 `SALES.SALES_RETURN.INVALID_STATE_TRANSITION`，且已登记单据的 voucher、库存效果和未开票应收保持不变。
- 取价与行金额：含税与不含税两种录入口径、折扣为零与非零、价目未命中、多行命中、税率为零与 13%、数量与单价均取六位小数时的舍入；再把同一数量拆成三段，逐段断言 `cum(B+q)-cum(B)` 且段和精确等于整行，覆盖税内价不得重复乘税与 1 分尾差由末段吸收。期望值在测试中写死为字面量，不由被测代码反算。
- 价格权限判定：五种命中组合与三级取用顺序。
- 五项校验：合同校验的四个子项各一个失败用例；库存与交期的失败传导；信用三桶的六种迁移时点。
- U-G-01：INACTIVE/CONFIRMED/RELEASED 三态转换、同组合多行先合计、可用量等于结存减动态未交付量、变更只比较正增量、确认不足零写入、交付同时减少结存与需求后可用量守恒；产品行使用冻结的 `inventory_material_id`，不得把 product_id 当 material_id。
- U-F-02：补货扫描的 limit=0、1、500、501 四个边界；跨越全停用原始页后仍能取到下一启用行；停用策略不返回；每行 `available_qty` 与同参数 `SalesAwareAvailabilityQuery` 逐值相等；加锁前策略版本改变时只采用锁后二次读取值；输出严格按仓库、物料排序且游标续页无重复无遗漏。
- 信用判定：额度为空的三种 `null_limit_behavior`、`amount_basis=WITH_TAX` 的固定守卫及拒绝其他值、`on_exceed` 两条路径、超出金额的计算。
- 分批交付行拆分：合计相等、合计不等、已交付行不可拆、合并后的批次号重排。
- 派生项生成规则：五类派生物在直运、需立项、订阅、寄售四种合同形态下的项数与来源引用。
- 收付款期次校验：比例合计等于 1、金额合计等于合同金额、两种基准混用被拒。
- 交付确认：本次数量超过分批交付行未交付量被拒、直运单跳过库存腿、非直运单的四步次序、任一腿失败整笔回滚，四条各一个用例，三腿以记录型桩断言入参与调用次序；另加零售价且零库存成本的合法全零交付，断言 `Skipped` 回带期间、CONFIRMED/voucher 空、无 unbilled 行但数量/状态/Outbox/审计完整，以及非零效果返回 `Skipped`、零效果返回 `Posted`、首次返回 `IdempotentReplay` 三个失败关闭分支。
- 销售退货同步过账：逐字断言 `period resolve → DeliveryCaptureReturnBasisQuery/原交付累计分配 → post_inbound → PostingPort/反向 capture → 适用时 UnbilledArPort → writeback/幂等 finish/Outbox/通知/audit-last` 次序；单一原交付、多交付不同实际成本、受控更正后多个 live fragments、`MainOperatingCost`/`DirectExpenseCost` 并存、累计区间中段与全量末段尾差、零收入/零 COGS、混合 INVENTORY/DIRECT_EXPENSE、全 DIRECT_EXPENSE、直运十组。对 available 比例构造会产生 1 至 n-1 个余分且 fraction 相同/不同的夹具，逐分断言 largest-remainder 的 `(fraction DESC,role ASC,root UUID,live UUID)` tie-break，反例把同额改成 FIFO 或调换一分钱并断言 COMMIT 拒绝。记录型端口必须核对 `InboundReason::SalesReturn`、`posting_line_key=<退货行 id>:1`、逐行 `InboundPricing::ReturnAtOriginalDeliveryCost { allocations: Vec<OriginalCostAllocation { source_line_id,quantity,amount }> }`、结果键一一对应；严格七字段 `PostingInput` 不含 `source_sequence_no`，`inventory_return_amount` 只作控制总额且恰等于 `inventory_return_cogs_amount+inventory_return_direct_expense_amount`，每个 attribution 的 measure/capture kind/parent/dimensions 与 owner fragment role 逐值一致。并在 owner query、库存、ledger/capture、finance 和最终回写五个失败点分别断言零提交；全零收入与全零库存回收入接受 `Skipped`、不调用 UnbilledArPort、REGISTERED/voucher 空且事件审计齐全。首次 `IdempotentReplay`、非零效果 `Skipped`、零效果 `Posted` 与期间结果不一致均按不变量故障回滚。
- F-10 三规则：三个 code、上游事件与目标模块逐字断言；订单行的零/非零交付分支、分批行取消、订单头汇总终态；节点的可处置/已终态分支；交付确认的未退/已全额退货筛选。三态 outcome 与全部稳定 reason 逐一写死期望值，不用被测代码反算。`RETURN_REGISTERED`/`NO_RETURN` 的 code、非空理由、结果 id 空值矩阵、退货状态与关联归属均各有正反例。
- 收款计划净开票消费：给定 `billing_by_period` 返回缺席、零、正数三种期次，合同变更守卫与终止规则都只允许前两种，履约投影逐期原样显示 `Money`；测试记录型桩只验证调用与映射，不实现第二套聚合。
- 换货关联：同客户同原订单同产品成功、客户不同、产品不同、替换分批行属另一订单、任一侧已配对、同 pair 与同幂等键重放七组；断言两个唯一键与固定锁序。

#### 8.2 领域属性测试

用 proptest 覆盖六组不变量，对应规格第 17.3 章可归属本阶段的判据。

1. 分批交付数量守恒：任意拆分与合并序列后，同一订单行的全部分批行数量合计恒等于订单行数量。
2. 信用三桶不重叠：对任意的下单、交付、开票、到款、退货事件序列，同一订单行的金额在任一时点只落在三部分中的一部分，三部分之和不超过该订单行的含税金额。
3. 金额舍入一致：合同头金额恒等于合同行金额按 2 位累加的结果，订单头金额恒等于订单行金额按 2 位累加的结果；对任意合法数量分割与税内/税外单价，全部连续区间的 `cum(B+q)-cum(B)` 之和逐分等于整行净额与含税额，最后区间精确吸收尾差。
4. 派生幂等：对任意的重复投递序列，同一 `(contract_id, contract_version_no, trigger, artifact_kind, source_ref_id)` 只产生一个目标单据。
5. 退货数量守恒：任意退货序列后，`returned_quantity` 恒不超过 `delivered_quantity`，且 `delivered_quantity` 恒不超过 `quantity`。
6. 可售数量守恒：任意确认、下达、增减量、移仓、取消、关闭与交付序列后，每个 `(法人, 仓库, 物料)` 的动态未交付需求恒等于 CONFIRMED/RELEASED 行的 `Σ(quantity-delivered_quantity)`，`available_quantity` 恒等于 on_hand 减该值；系统不建 `reserved_quantity` 列。任意成功确认或下达在其锁内快照上均满足 `available_before >= requested_increment`。

#### 8.3 集成测试

使用真实 PostgreSQL 16，每个用例独占一个 `ep_test_<nanoid>` 库，结束即删库。禁止用内存库或 mock 替代数据库。外部电子签章用 wiremock 打桩，同时提供一套对真实沙箱执行的契约测试，后者在阶段 4 按附录 B 判定。

场景清单：

1. 合同建单到提交审批的完整路径，含五项校验记录与审计事件的写入核对。
2. 折扣超权限时折扣审批链被挂起，未超权限时不进入折扣节点，其余三条链照常执行。
3. 管理层与信用节点不可跳过：构造缺少 `MANAGEMENT_APPROVER` 的合同生效链、缺少 `FINANCE_MANAGER` 的信用超额链或申请人与节点展开用户相交，验证在配置发布或提交阶段被 fail-closed 拒绝。
4. 申请人不可自审：发起人尝试审批自己发起的合同被拒绝并给出冲突节点。
5. 合同生效缺少重新认证凭证被拒绝，凭证过期被拒绝，凭证绑定的待签内容摘要不匹配被拒绝。
6. 派生完整路径：五类派生项在同一批次内全部建立且与 `item_total` 一致；销售订单、收款计划与交付节点三类在本阶段真实生成对应单据与记录并双向可追溯可查；采购需求派生项 `status` 恒为 PENDING、`target_doc_id` 留空且不计入 `item_done`，其派发在阶段 7 接线后补做；项目任务派生项按裁定 C-19 只登记不派发，`status` 置 DONE 且 `target_doc_id` 留空。不含采购需求派生项的合同进入履约中，含该项的合同停在已生效。
7. 派生重复投递 3 次，派生单据只产生一次。
8. 派生失败进入死信，运维中心可枚举，人工修复后重放不产生重复单据。
9. 派生时信用不足使订单进入待放行，只允许非申请人的 `FINANCE_MANAGER` 审批，通过后转为已放行；`SALES_MANAGER` 无此审批能力。
10. 派生时库存不足使订单与各行分别保持 PENDING_RELEASE、INACTIVE；库存恢复后在组合锁内重跑，同一事务先把行置 CONFIRMED 再把订单放行并置 RELEASED。
11. 同一客户并发下单：两条并发的合同提交，验证串行化后总占用不超额，其中一条被阻断或转审批。
12. 订单变更提高金额时重跑信用；提高数量或移入新仓库时按新旧组合正增量重跑库存，减量或移出旧仓库在同事务释放需求但仍执行锁内重算。
13. 订单变更审批通过与交付确认事务交叠，固定组合锁序下无死锁，已交付数量不被覆盖，最终未交付需求与现行订单行一致。
14. 分批交付行拆分后信用占用总额不变；分别用税内价与税外价把一行拆成三次确认，逐次核对连续 allocation 区间、净额/含税额累计差分与 open amount，第三次后两项分段金额和等于原行且 open amount 精确为零。
15. 销售退货前置红冲校验：未红冲时阻断并列出待冲销发票，红冲后进入第 4.12 小节同步库存/凭证/未开票应收链路；链路未全部成功不得进入 REGISTERED。
16. 直运订单的销售退货不调用库存端口、不产生库存数量/金额流水或 `inventory_return_amount` 凭证计量项，登记事件载荷中 `is_drop_ship=true` 且逐行 `inventory_return_amount/stock_movement_id` 为空；重复消费该事件仍不得出现库存写入。
17. 电子签章超时、失败、熔断与恢复四条路径，以及验签失败时合同保持待签署；逐字验证 submit/status 两种清洗回执、四项反向文件 operation、512 KiB 块界、seq/长度/块哈希/总哈希、单块在途和 10/30/3600 秒超时，任一失败不产生部分附件关联且清理临时件。另验证 `NT SERVICE\ep-worker` 可调用两个 e-sign 请求 operation，core、ops 与任意其他账户被 DACL 或 operation allowlist 拒绝，反向帧只被同一已认证 gateway 连接接受，8082 和 `/internal/v1/esign/*` 均无监听；gateway 配置与进程句柄中不存在数据库、文件库、KMS 或 Outbox 能力。WinCred 子矩阵在 Windows Server 2022 实机断言 SCM 加载 `ep-integ` profile、服务 current token 写入/读取、正常重启后同一 target 仍可读；普通管理员直接 CredWrite、模拟服务 token 与手工 `LoadUserProfile` 不能冒充。另断言 2560 bytes 成功、2561 bytes 预拒绝、双签/时窗/客户端 PE/DACL 任一不符失败、ROTATE probe 失败恢复旧值、回滚失败保持能力关闭，且 secret marker 不进 HTTP、ServerAdmin、普通业务管道、argv、env、文件、日志或 receipt。
18. 实体印章路径：用印登记后可执行生效动作，缺扫描件时被拒绝。
19. 合同合并：来源合同状态不合规被拒、客户不一致被拒、合并成功后来源置作废并保留关联。
20. 合同续签：续签版本与原合同双向可达，生效后派生新的订单、收款计划与交付节点，不重复派生原合同已有单据。
21. 法人越权测试集 `tests/rls_matrix` 的本阶段部分：对 `clm` 与 `sales` 两个 schema 的全部表覆盖读取、写入、更新、删除、聚合、排序、报表投影与错误信息泄漏八类，跨法人一律不可见且不泄露存在性。
22. 迁移的执行与回退：全部迁移在空库上执行成功，按各文件的 `-- rollback:` 段落回退后 schema 与本阶段之前一致。
23. 交付确认三腿同事务与 U-D-01 分支：混合单据中 INVENTORY 行回填 `cogs_amount` 与 `stock_movement_id`，DIRECT_EXPENSE 服务行两列保持空且不产生库存/销货成本腿；两类行均确认收入与未开票应收，凭证和事件回填完整。实际直接费用尚未捕获时毛利显示“暂估/成本缺失”。本场景整条属第三批，随阶段 10 的 `UnbilledArPort` 同批执行，不含替身断言。
24. 交付确认的重复提交：同一 `Idempotency-Key` 重放 3 次只产生一张交付确认单、一次三腿调用、一次订单同步回写与一条事件，`delivered_quantity` 和需求释放均不重复累加；重复消费事件只幂等推进合同节点，不接触 sales schema。
25. 操作者与字段边界：`WAREHOUSE_USER` 可登记交付事实但不可最终确认，`SALES_MANAGER` 与 `PROJECT_MANAGER` 可最终确认；`TECHNICIAN` 只能登记交付证据并读取不含金额、成本、毛利的技术摘要，尝试最终确认或读取价格字段均被拒且无字段泄漏。
26. `SalesExchangeLinkCommandPort` 契约：经 HTTP 与 trait 两个入口各执行同 pair 重放、任一侧重复配对、跨法人、客户不同、产品不同、非原订单与任一侧取消；只有首个合法 pair 产生一行 `sales.exchange_links` 与一条审计记录，所有失败均零写入。
27. U-G-01 并发超卖：同一 `(法人, 仓库, 物料)` 以 20 个事务并发确认或下达、总请求量大于结存；成功事务的请求量之和不超过各自锁内可用量，其余返回 `SALES.SALES_ORDER.STOCK_NOT_AVAILABLE`，无负结存、无重复需求、无死锁。
28. A2 与确认守卫同源：在零需求、CONFIRMED、RELEASED、部分交付、取消、关闭六个快照逐一比较 HTTP A2、`AvailabilityQueryPort` 与确认守卫的三项数量，结果逐值相等；装配缺 `ConfirmedOpenSalesDemandQuery` 时 A2 路由不得注册，不能返回 reserved=0。
29. 销售物料快照：MATERIAL、INVENTORY 产品、DIRECT_EXPENSE 产品三类合同行经 `resolve_sales_item_profile` 带出后派生到订单、交付和退货行；唯一关联物料随后停用或版本变化不回写旧单据，INVENTORY 产品缺唯一物料时建单 fail-closed。
30. U-F-02 真实组合：建立 1,203 条策略，夹入停用行、零结存、负可用量与跨法人数据，以 500 行游标分页扫完；结果只含本法人启用策略、排序稳定、无重无漏，每行 `available_qty` 与 A2/`AvailabilityQueryPort` 的同组合值一致。并发执行 A12 阈值修改与阶段 7 扫描时按统一 advisory 锁序串行化，扫描只看到修改前或修改后的完整二元阈值；缺少策略、结存或销售需求任一真实 provider 时 job-worker 不注册自动采购扫描。
31. F-10 `assess` 集合：同一合同构造 3 条可处置订单行、2 个可取消节点、1 张未退货交付确认，再各加已终态、异合同、异法人与已全额退货反例；断言三条规则分别只产出 3、2、1 个实项，其余四个目录 code 各只有一个 PENDING 空目标占位项。`ImpactRegistry` 注册数恰为 3、目录条数恰为 7，没有替身规则或第二个终止消费者。
32. F-10 自动处置与竞态：零交付订单行被取消，非零交付行被关闭，两者的 PENDING 分批行均被取消且不再进入逾期清单；订单头按锁后总交付量进入 CANCELLED/CLOSED，需求即时释放且终态事件恰一条。节点仍可处置时返回 Completed，在锁前先被确认/取消时返回 AlreadySatisfied；订单行在 assess 后发生合法交付竞态时以锁后数量选正确自动分支。每项 dispose 重放 3 次不二次改写、审计或发事件。
33. F-10 人工决策：由平台以 SALES → `SALES_MANAGER` 的固定映射建一条 HUMAN_TASK。`RETURN_REGISTERED` 对同法人、指向本确认单且状态 REGISTERED/CLOSED 的退货单成功；`NO_RETURN` 仅在结果 id 为空时成功。错 code、空理由、两类结果 id 形状颠倒、异法人、异确认单、退货单状态不符及只在理由文本写“已退”均零业务写入并保持 PENDING。决策前已全额退货则返回 AlreadySatisfied；平台按稳定决策码、理由与 `decision_result_doc_id` 持久化，不只存流程 outcome。
34. 收款计划净已开金额同源：在 `invoice.invoice_receipt_plan_links` 建两期正向分摊，并依次加入部分红字、全额红字与 VOID 反向分摊；真实 `ReceiptPlanBillingQuery::billing_by_period`、合同履约页、合同变更守卫与 `CLM_TERM_PAYMENT_SCHEDULE` 对每期净额逐值一致。净额大于零的期次不自动作废，归零后可作废；缺失真实查询实现时三个调用点均不注册，且 clm schema、Outbox 与消费者中都不存在开票金额副本。
35. 非直运 INVENTORY 销售退货同步闭环：不论当前库存余额是否为零，都固定命中 `ORIGINAL_DELIVERY_COST`。让一条退货行关联两条不同原净收入/gross/COGS 的交付确认行，并把其中一个原 capture 做受控更正形成跨两个 `ReturnCostRole` 的多个 current live fragments；逐项核对 link 的连续累计区间、三额差分公式、全量最后区间吸收尾差、收入/成本两 side 按 available 整数分 largest-remainder 的 base/余分/tie-break、capture allocation 的 root/live/side/role/amount、`OriginalCostAllocation { source_line_id,quantity,amount }`、库存数量/金额账、收入与两项成本 measure、未开票应收、退货行回填、订单行 returned_quantity、审计与单条 Outbox 同事务。另以两个并发退货争用同一原交付行剩余数量和同一 live fragment 开放额，断言锁后串行重算且只有累计数量与金额均合法的事务成功。分别在 owner query、库存、PostingPort/capture、UnbilledArPort 与最终状态回写前注入失败，五次都断言 links 分配列、allocation 表及所有权威表零增量且退货仍未登记。
36. 销售退货幂等与事件派生边界：非零场景同一 `Idempotency-Key` 并发及顺序各重放 3 次，只产生一个 stock_movement、一张 voucher、一条 unbilled_ar_entry、一次 returned_quantity 增量、一条审计和一条 `sales.sales_return.registered.v1`；全零场景同样重放，只产生一次库存数量事实/数量回写、零 voucher、零 unbilled、一次审计和一条事件，响应与事件的 voucher 均为空。不同键竞态被退货头锁与状态守卫串行化。把该事件对 service、reporting、search、notify 各重复投递 3 次，inventory/ledger/finance/sales 权威表行数与金额均不变化，wiring 中不存在该事件的库存或财务写消费者。
37. 已登记退货不可取消：分别从 DRAFT、SUBMITTED 取消成功且 `registered_at/voucher_id` 恒空；构造已完成库存、凭证与未开票应收的 REGISTERED 退货后调用 cancel，返回 `SALES.SALES_RETURN.INVALID_STATE_TRANSITION`，状态和所有已过账效果逐值不变且不发 cancelled 事件；随后 close 成功且只发 closed 事件。直接尝试写入“CANCELLED 且 voucher 非空”由 `ck_sales_returns_registration_shape` 拒绝。
38. 交付祖先图 direct-SQL：先建立两个同法人订单、各自订单行、分批行与 DRAFT 交付头，使每个普通单列外键目标都真实存在，再分别尝试把确认行的头、订单行、分批行交叉拼接，以及篡改头客户、直运标志、仓库或行产品/价格快照；长复合外键或 `assert_delivery_confirmation_graph_consistent` 必须在语句或提交时拒绝且零部分行。合法非零 CONFIRMED 图缺 voucher、合法全零图携带 voucher、DRAFT 图预填库存结果三组也必须拒绝；全零且 voucher 空、非零且 voucher 来源同本确认单两组通过。
39. 退货祖先与效果图 direct-SQL：在两个订单及两张已确认交付中构造“退货头、退货行、交付行各自普通 FK 均命中但祖先交叉”的图，另构造链接合计少于/大于退货量、REGISTERED 空行、累计退货超过原交付、区间重叠/空洞/非零起点、三额差分或尾差错误、行三额不等 links 合计、非终态预填分配或 allocation、终态半回填、movement 属另一退货或同 movement 另一来源段、voucher 来源另一退货；长复合外键或 `assert_sales_return_graph_consistent` 必须拒绝并整事务回滚。Stage11 追补后再构造 revenue/cost side 列错形、root/live 跨链或跨法人、root 源自另一交付行、负向/耗尽 live、fragment 合计不等 link、缺/多/错父 SALES_RETURN capture，以及登记后 UPDATE/DELETE link 或 allocation，均在语句或 COMMIT 拒绝。合法全零终态零 fragment/voucher 空、合法多 live fragment 非零终态 voucher 非空两组通过，两个并发事务争用同一交付行与 live 开放额时锁后至多一个完整图提交。
40. 来源唯一键与迁移 DDL 正反例：在同一法人连续插入三张四项来源全空的退货均成功；插入两张完全相同的非空 `(source_module,source_doc_type,source_doc_id,source_doc_line_id)` 时第二张由普通 `ux_sales_returns_le_source_ref` 拒绝。查询 `pg_index` 断言该唯一键不存在谓词；空库迁移与回退还要验证 `V20261017092800`、`V20261017093200`、`V20261017093630`、`V20261023092700` 与 `V20261023092800` 的长复合外键、候选键、全部延迟触发器/函数、capture allocation 仅追加守卫与 registry 行按声明出现与消失。Stage11 再执行/回退 `V20261020090130`，由 `pg_constraint` 断言 revenue/cost root-live FK 均同法人、RESTRICT、DEFERRABLE INITIALLY DEFERRED，costing 两表补充触发器与旧 sales 四表触发器无重无漏。
41. 合同经济图 direct-SQL：让所有普通 FK 命中，分别伪造合同头净额/含税额、税内价重复乘税、非 DRAFT 空行、同合同混合 order_type/cycle/lease、ACTIVE 付款期次 basis 混用、ratio 不等 1、amount/amount_with_tax 不等合同头，以及 contract_versions snapshot 的头/行/期次任一字段漂移；均由 `assert_contract_economic_graph_consistent` 在 COMMIT 拒绝。合法 DRAFT 暂不闭合可提交，合法提交审批及以后图逐态通过。
42. 订单来源与履约经济图 direct-SQL：建立两个合同版本、两张订单和完整普通 FK 目标，分别构造订单头指版本 A 而行取版本 B、source line 不在快照、混合订单类型、伪造头行合计、分批数量少/多于订单行、schedule/line delivered_quantity 与确认事实不等、CONFIRMED allocation 非零起点/重叠/空洞、税内价重复乘税、三段尾差错误、open amount 伪造及 CLOSED/CANCELLED 非零 open；长 FK 或 `assert_sales_order_economic_graph_consistent` 必须在 COMMIT 拒绝且零部分写入。并发确认同一订单行时锁后只允许连续区间完整提交，合法税内/税外三段全量交付均精确归零。

#### 8.4 端到端测试

后端 E2E 用 Rust 集成测试直接打 HTTP 接口；四端 UI 用 Playwright 驱动桌面 WebView 与 tauri-driver 驱动桌面壳，移动端用 XCUITest 与 Espresso 只跑规格第 6.2 章矩阵中取值为完整或简化的场景。本阶段涉及的两行能力域为合同条款与电子签章、销售订单与履约，两行在 Windows 与 macOS 为完整，在 iOS 与 Android 为简化，合同生效的重新认证要求在四端一致。

| 编号 | 场景 | 判据来源 |
|---|---|---|
| E2E-6-01 | 一条合同从建单、审批、签署、生效到派生批次建立，销售订单、收款计划与交付节点三类单据全部可见并双向可追溯；采购需求派生项可见且 `status` 为 PENDING、`target_doc_id` 为空，项目任务派生项可见且 `status` 为 DONE、`target_doc_id` 为空，界面按未接线呈现 | 规格第 8 章第 1 至 3 步 |
| E2E-6-02 | 信用超额阻断路径：提示信用额度、已占用金额、可用信用额度、本次需占用金额、超出金额与三部分构成明细 | 规格第 5.2 章客户信用额度校验条目、PRD 3.14.4；规格第 17.2 章末段的判据只列应收未收与在途订单两部分，与第 5.2 章不一致，已按 U-E-10 登记在第 11.3 小节，本用例按第 5.2 章的三部分判定 |
| E2E-6-03 | 信用超额转审批路径：审批通过后合同继续原审批链，待放行派生单据转为已放行 | 同上 |
| E2E-6-04 | 信用额度的下单占用与释放：下单、交付、开票、到款、退货五个时点的三桶迁移逐点核对 | 规格第 19 章阶段 3 的客户信用额度校验门槛 |
| E2E-6-05 | 一次由原合同派生续签版本并重新审批生效派生新单据的完整用例 | 规格第 19 章阶段 3 的 CLM 门槛 |
| E2E-6-06 | 一次合同合并用例 | 同上 |
| E2E-6-07 | 三类到期提醒各触发一次：合同有效期、交付节点日期、收付款计划到期日 | 同上 |
| E2E-6-08 | 一次订阅或租赁类型订单用例，周期与租期字段随分批交付与变更版本正确流转 | 规格第 19 章阶段 3 的销售与 OMS 门槛、PRD 3.12 |
| E2E-6-09 | 订单拆分为三条分批交付行并分批交付：确认/下达前后 A2 分别显示动态需求，交付事务同时减少结存与未交付需求，订单状态由已放行经部分交付到已交付且可用量守恒 | 规格第 8 章第 3 步与第 8 步的销售侧、F-51 U-G-01 |
| E2E-6-10 | 销售退货完整用例：已开票部分先红冲再退货；非直运 INVENTORY 行在登记事务按原交付实际 net/gross/COGS 的连续累计区间与 current live capture fragments 同步回收入、适用的未开票应收、存货与主营业务成本，并回填可空 voucher_id/inventory_return_amount/stock_movement_id；当前结存非零与零两种场景都必须使用 `ReturnAtOriginalDeliveryCost`，多原交付、多 live fragment 与末段尾差逐笔可追溯。另覆盖全部会计效果为零时合法 Skipped、零 capture allocation/零凭证/零 unbilled 而业务与库存数量事实完整；退货后信用占用相应释放，任一腿失败无部分事实 | 规格第 8 章第 11 步、第 17.2 章财务内核测试的销售退货基础分支与必测分支十四 |
| E2E-6-11 | 换货用例：一笔退货行加一笔在原订单上放行的替换分批交付行，经 `SalesExchangeLinkCommandPort` 建立一对一关联；客户与产品相同，同 pair 重放不新增，任一侧试图再次配对被拒 | 规格第 8 章第 11 步、F-51 U-J-08 |
| E2E-6-12 | 电子签章端到端：签署发起、结果回传、验签、签章文件归入合同附件与审计 | 规格第 10.4 章连接器验收判据、第 19 章阶段 3 |
| E2E-6-13 | 交付确认完整用例：由分批交付行建交付确认单并确认过账，同一事务内锁库存组合、四步依次成功、同步推进分批交付行与订单行 `delivered_quantity` 及订单头并即时释放销售需求，非零效果回填 voucher_id；全零售价/成本分支合法 Skipped、voucher 与 unbilled 为空但确认、库存数量事实、事件和逐行结果完整；`clm.milestone_confirm` 消费事件后只推进合同交付节点 | 规格第 8 章第 8 步、F-51 U-G-01 |
| E2E-6-14 | 一份 IN_PERFORMANCE 合同终止审批通过后进入 TERMINATING；三条已注册规则产出实项，订单行/分批行/节点自动闭合，交付确认人工项以 `RETURN_REGISTERED` 或 `NO_RETURN` 闭合；另四类为 PENDING 空目标占位项，因此批次仍 RUNNING、合同仍 TERMINATING，展示“未接线”而非伪终态 | F-10 A-7/A-11、PRD 3.5.5 |
| E2E-6-15 | 对交付确认人工项依次提交空理由、错 code、异单/异法人/非 REGISTERED/CLOSED 退货单、`RETURN_REGISTERED` 空结果 id 与 `NO_RETURN` 非空结果 id，全部拒绝且保持 PENDING；已全额退货单据不被 assess 产出。重放不重复待办，不改财务、库存或交付确认事实 | F-10 A-1/A-7 反向判据 |

E2E-6-04、E2E-6-10 的会计规则与库存取价由阶段 8、9a、10 的真实端口提供，但事务编排和完整回滚由本阶段 `ep-app-sales` 负责；合并执行时同时核对销售单据、库存两账、凭证、未开票应收、信用占用与事件，并按规格第 17.2 章对应条目判定差额为零，不再把账务侧推给异步事件或后续人工验收。按第 11.5 小节，E2E-6-04、E2E-6-09 的交付段、E2E-6-10 与 E2E-6-13 四项整条属第三批，与阶段 10 的 finance 端口同批执行，四项都不含任何经替身实现的断言，也不再登记顺延项。E2E-6-14 与 E2E-6-15 在本阶段当场使用阶段 3 的真实影响面平台与本阶段三条真实规则，不等后续阶段、不注入四条替身；合同保持 TERMINATING 正是对未注册占位项的必要断言。E2E-6-01 与第 8.3 节场景 6 中采购需求派生物的端到端断言在阶段 7 接线后补做，项目任务派生物的端到端断言由阶段 12 的 `project.contract_derivation` 消费者承接，本阶段只断言这两类派生项行已建立、采购需求项为 PENDING、项目任务项为 DONE 且两者 `target_doc_id` 均留空。E2E-6-02 与 E2E-6-04 的三桶断言按 U-E-10 以规格第 5.2 章为准，不按规格第 17.2 章末段的两部分表述。

#### 8.5 性能相关项

本阶段涉及附录 A.1 度量清单中的七项：常规交互中的销售订单表单打开并带出默认值、库存可用量查询、审批任务列表加载；普通交易提交中的合同提交、合同审批提交、审批放行提交、销售订单提交、退货登记；常用报表中的销售订单履约明细。合同生效派生按附录 A.1 为非交互观察项，只记录不设通过线。

本阶段的性能要求为：在附录 A.3 基准数据集上，上述查询的 `EXPLAIN` 输出中不得出现顺序扫描，逐条附执行计划证据。具体涉及的索引为 `ix_contracts_legal_entity_id_customer_id_status`、`ix_sales_order_lines_legal_entity_id_customer_id_status`、`ix_sales_order_lines_open_inventory_demand`、`ix_delivery_schedules_legal_entity_id_promised_date_status`、`ix_contract_milestones_legal_entity_id_promised_date_status`。A2 的计划证据必须同时显示阶段 8 结存侧索引扫描与本阶段销售需求生成槽普通索引扫描，不能只证明一侧。时延通过线在阶段 4 统一判定，本阶段不冻结取值。

#### 8.6 覆盖率门槛

- 信用占用计算、分批交付数量守恒、派生幂等三处属规格第 17.3 章强制不变量相关代码，行覆盖率不低于 85%。
- 本阶段其余代码行覆盖率不低于 70%，新增与修改代码不低于 80%。
- 工作区整体行覆盖率不低于 80%。
- 工具为 cargo-llvm-cov，阈值由 `codecov.toml` 中与 crate 清单一一对应的路径规则表达，CI 上以 `--fail-under-lines` 强制。
- `#[ignore]` 必须带 issue 编号注释且存活不超过一个阶段。

---

### 9. 退出条件

下列条目全部达成才算本阶段完成，每条均可客观判定。

1. 第 1 节的十项交付物全部存在，`cargo build --workspace --release` 与 `cargo clippy --workspace -- -D warnings` 通过；`apps/core-server/src/wiring/` 与 `apps/job-worker/src/wiring/` 两个目录下的全部文件中不出现任何 `Noop`、`Stub`、`Fake`、`Dummy` 前缀的注入行，本阶段不产生任何空实现，该口径与技术基线第 10.4 节一致，判据提供方是阶段 1 随 `xtask` 交付的 archcheck 规则 `unwired-absent`；第 11.5 小节第三批的退出条目与阶段 10 的 finance 端口同批判定，其余条目在第二批结束时判定。
2. 三个迁移目录的全部阶段 6 迁移在空库上按文件版本号全序执行成功，且各文件的回退说明经一次实际回退验证；`V20261017093630__sales_backfill_append_only_registry.sql` 的 allocation 登记与仅追加守卫正反执行无残留，`V20261017093700__clm_add_cross_schema_foreign_keys.sql` 已在后建销售目标存在后补齐 clm 外键，`V20261023092700__clm_harden_contract_economic_graph.sql` 与 `V20261023092800__sales_harden_order_delivery_economic_graph.sql` 的候选键、版本 FK、五表反向触发源、延迟图与恢复型回退均由系统目录及场景 41/42 证明。Stage11 目标建立后，`V20261020090130__sales_add_costing_capture_foreign_keys.sql` 的两条 root/live 长 FK、costing 两表补充触发器和恢复型回退亦在影子库通过；其成功前销售退货 REGISTERED 写入口不启用。其余单目标跨 schema 外键均随各自建表文件内联；`ledger.posting_trigger_event_types` 的两行登记由阶段 9a 的种子迁移写入，本阶段既不回填也不判读该表。
3. `apps/core-server --check` 与 `apps/job-worker --check` 在基线第 7.3 节十项中的九项上全部通过并输出结构化报告，本阶段不追加任何启动自检项；`offsite-sink-requirements` 一项按阶段 1 计划整条推迟到阶段 14，本阶段返回 `NOT_APPLICABLE` 并在报告中标注承担阶段，不计入本条的通过项，该处置按基线第 12 节通则第六条取换判据一档；本模块事件集合与第 6.3 小节及 `docs/event-catalog.md` 经 `xtask configdoc` 做集合相等比对通过。
4. 基线第 1.3 节的依赖方向自检脚本对本阶段新增 crate 全部通过，`ep-domain-clm` 与 `ep-domain-sales` 中无 sqlx、reqwest、文件与网络符号。
5. 第 8.1 至 8.3 节的全部单元、属性与集成测试通过，集成测试跑在真实 PostgreSQL 16 上。
6. 第 8.4 节的十五个 E2E 场景在 Windows 与 macOS 两端全部通过，在 iOS 与 Android 两端按简化取值通过，合同生效的重新认证在四端一致。
7. `tests/rls_matrix` 的本阶段部分八类越权测试全部通过，跨法人零泄漏。
8. 第 8.6 节的三档覆盖率门槛全部达标。
9. 第 8.5 节涉及的五条本阶段索引及 A2 结存侧索引在基准数据集上的 `EXPLAIN` 证据已归档，无顺序扫描。
10. 本阶段第 6.3 小节逐项列名的事件已全部登记在 `docs/event-catalog.md`，登记集合与代码常量集合逐字一致、无多余与缺漏；F-10 两个终止事件均为非过账事件且 `clm.contract.terminated.v1` 的消费者恰为一个。本阶段第 5 节 API 契约表中出现的全部错误码已登记在 `docs/error-codes.md` 并与 `ep-foundation::error::codes` 一致，由 CI 校验通过。
11. 本阶段新增指标固定为 0；合同与销售只填充既有 HTTP、数据库、Outbox 与死信指标。旧“六个指标”没有任何具名定义，已由 F-54 撤销，不得以未命名配额驱动实现；`docs/metrics-catalog.md` 集合一致性校验通过。
12. 合同生效、订单放行、退货登记三类操作的审计事件已进入按法人与自然日的哈希链，审计链验证工具在本阶段的用例数据上通过。
13. 第 11.2 小节的偏离项已提出对应的基线修订并被整合员接受，第 11.3 小节的新增决定已回写基线。
14. 派生失败到死信、人工修复、重放不产生重复单据的完整链路已在运维中心可见并演示通过。
15. 本模块的四个受治理数据集视图 `clm.v_contracts_dataset`、`clm.v_contract_delivery_milestones`、`sales.v_sales_orders_dataset`、`sales.v_order_delivery_batches` 已发布并授予 `ep_analyst_ro`，每个视图含 `legal_entity_id`、`security_level`、`data_scope_tags` 三列，`ep_app_rw` 之外无任何写权限，列签名已同步给阶段 11 且与 `reporting.dataset_fields` 的登记一致。
16. clm 与 sales 两个模块在规格第 6.2 章能力矩阵中取值为完整或简化的能力域，其四端界面已实现并通过 Playwright 与 tauri-driver 的桌面用例、XCUITest 与 Espresso 的移动用例；取值为 VIEW_ONLY 的能力域只实现只读视图；取值为 NOT_APPLICABLE 的不实现入口。
17. 本阶段全部路由的能力域码与动作类别常量已在 `crates/contract/clm/src/capability.rs`、`crates/contract/sales/src/capability.rs` 与阶段 5 已建的 `crates/contract/cpq/src/capability.rs` 声明，`xtask configdoc` 通过。
18. `ClmProductUsageProbe` 与 `SalesProductUsageProbe` 已实现并注入阶段 5 提供的 `AnyProductUsageProbe`，阶段 5 的启动自检项 `master-data-usage-probes-registered` 在 clm 与 sales 启用时通过；本模块的 `ClmReferenceCounter` 与 `SalesReferenceCounter` 已注册到 `MasterReferenceCounterRegistry`，`SalesTradeHistoryProviderImpl` 已注册到 `TradeHistoryProviderRegistry`。
19. 四个单据类型码 CT、SO、SR、DC 已登记入 `docs/data-dictionary.md` 的单据类型码一节与 `ep-platform-sequence` 的常量表，`xtask configdoc --check-doc-type-codes` 通过。
20. 规格第 21.4 章要求的专业签字已取得并留档：法务在本阶段签字，签字人资格证据随版本留档；签字缺失或不通过时本阶段不得退出，整改后重新测试并重新签字，不得以未记录的方式豁免（规格第 22 章第 12 条）。本条由裁定 F-42 新增，此前四份计划的退出条件中无任何签字项。
21. `SalesExchangeLinkCommandPort::link_exchange(&mut dyn Tx, …)` 已在 ep-contract-sales 定义并由 ep-app-sales 真实实现；`sales.exchange_links` 以退货行和替换 delivery schedule 两个独立唯一键强制一对一，客户、原订单与产品一致性守卫、幂等重放、取消后历史保留及阶段 12 的跨模块契约测试全部通过。
22. `ConfirmedOpenSalesDemandQueryImpl`、`SalesAwareAvailabilityQuery` 与阶段 8 的 `StockOnHandQueryPort` 已在同一批真实装配，`AvailabilityQueryPort` 与 A2 路由同批启用；确认、下达、变更、取消、关闭、交付六类写路径全部调用同一固定组合锁与重算函数，集成场景 27 至 29 及 E2E-6-09、E2E-6-13 通过。仓库中不存在 reserved 持久化表/列、零值 provider、`sales.delivery_writeback` 消费者或第二套可用量 SQL。
23. `ep-contract-inventory::ReplenishmentPolicyQuery` 与 `SalesAwareReplenishmentPolicyQuery` 已同批交付，core-server 与 job-worker 均由同一装配函数构造 `SalesAwareAvailabilityQuery`；job-worker 组合阶段 8 的真实 `ReplenishmentPolicyReadPort` 后只向阶段 7 暴露这一 trait。1 至 500 的分页边界、停用行跳页、零结存、负可用量、锁后二次读取、跨法人隔离及 A12 并发测试全部通过，集成场景 30 与阶段 8 的 I-24 使用相同期望值；仓库中不存在采购侧第二套阈值表或可用量 SQL。
24. `ContractTerminationSalesOrderLineImpactRule`、`ContractTerminationMilestoneImpactRule`、`ContractTerminationDeliveryConfirmationImpactRule` 已以三个固定 code 真实注册，阶段退出时 `ImpactRegistry` 注册数恰为 3、编译期目录恰为 7；`ContractTerminationCompletionPort` 已以 `(CLM,clm.contract.terminated.v1)` 唯一真实注册。集成场景 31 至 33 与 E2E-6-14/15 全绿：三条 assess/dispose、锁后竞态重检、统一三态 outcome、两个交付确认决策码与 `decision_result_doc_id` 对象语义均有真实 PostgreSQL 正反例；另四类占位项保持 PENDING 且合同保持 TERMINATING。缺失/重复/替身 completion port 时启动与模块启用均失败关闭；wiring 无任何影响面替身，`clm.contract.terminated.v1` 仍只有 `platform.impact_assess` 一个消费者。全部七类闭合的终态测试在阶段 12 承接，并验证完成端口与批次同事务、完成事件恰一次。
25. `ReceiptPlanBillingQuery` 只有 `billing_by_period(&mut dyn Tx,ctx,contract_id) -> Result<BTreeMap<i32,Money>,AppError>` 一种读取形状；真实实现以 `invoice.invoice_receipt_plan_links` 正向分摊减 VOID/RED 反向分摊返回逐期净额。集成场景 34 通过，合同变更、履约页与收款计划终止规则同源，净额大于零判占用；clm 表、Outbox、事件消费者和应用投影中均不存在第二份已开票金额或第二套聚合 SQL。
26. 电子签章的六个 operation、两种普通回执、`BoundedChunkStreamV1` 反向文件流和第 8.3 节场景 17 全部通过；SIGNED manifest 的每个 ordinal 恰有一个 `clm.contract_attachments` 关联且没有部分批次。每个文件均有“临时加密对象→长度/hash/type/structure→按部署模式病毒扫描→签章验签→数据库确认/发布”的阶段 3 流水线证据，未通过者保持 QUARANTINED 且合同不转 SIGNED。integration-gateway 的配置结构、Windows 服务凭据、进程句柄和依赖图均无数据库/连接池、文件库、KMS、Outbox 消费或业务表写入能力；全部签章结果只由 job-worker 持久化。另在 Windows Server 2022 以 SCM 加载 profile 的服务虚拟账户实测 `ep-secretctl` 维护状态机、current-token CredWrite/Read、正常重启同 target 可读、双人 grant、probe/rollback、Event Log 与 zeroize；普通管理员 CredWrite、模拟服务 token 与手工 `LoadUserProfile` 均不能冒充。secret 取 2560 bytes 成功、2561 bytes 在 Win32 调用前失败，维护管道在 CLOSED 状态不存在，HTTP/ServerAdmin/argv/env/file 均无 secret 入口。
27. `register_sales_return` 已按第 4.12 小节以一个真实数据库事务完成期间解析、原交付累计差额/current live fragment 分配、非直运 INVENTORY 行的 `ReturnAtOriginalDeliveryCost` 入库、严格七字段 `PostingPort::post`、`UnbilledArPort::record_on_sales_return`、销售侧同步回写、审计和 Outbox；集成场景 35 至 37 与 E2E-6-10 全绿。当前结存非零/为零、多原交付、部分更正后多 live leaf、`MainOperatingCost`/`DirectExpenseCost` 两角色、累计中段与末段尾差、零原 COGS、DIRECT_EXPENSE、直运、五个失败注入点和并发幂等均有真实 PostgreSQL 证据；`inventory_return_amount` 未进入 PostingInput 且恒等于两个静态成本 measure 之和，trybuild 证明 sales 不依赖 ledger 的 AccountRole。`sales.sales_return.registered.v1` 的全部消费者重复投递后库存、ledger、finance、sales 权威表零变化，wiring 与事件目录中不存在该事件驱动的库存/财务补写通道。DRAFT/SUBMITTED 可取消，REGISTERED 只能关闭且首版无冲正入口；对已登记退货调用 cancel 时所有已过账效果逐值不变、无 cancelled 事件。
28. 第 92600、92800、93200 号迁移已按第 3.4 节建立交付与退货祖先长复合外键、候选键、普通来源唯一键及两个延迟图约束函数；集成场景 38 至 40 全绿。普通外键全部存在但跨订单拼接、错误快照、错误库存/凭证来源、半效果图和链接累计超限均不可提交；全零与非零 voucher 形状逐值通过。回退时约束触发器和函数先于表删除，空库全序和逐文件回退都不留下悬空依赖。

---

### 10. 与规格和 PRD 的对应

#### 10.1 规格条目

| 规格章节与条目 | 本阶段实现的部分 |
|---|---|
| 第 5.2 章 CLM | 多类型合同、模板、条款、修订、电子签章、实体印章、履约、义务、收付款与档案；合同审批后自动生成订单、采购需求、项目任务、收款计划与交付节点；合同合并；合同续签的派生、关联、追溯与重新审批生效；合同到期提醒的三类触发源投影 |
| 第 5.2 章 销售与 OMS | 订单创建与变更、每次变更保留版本与审批记录；订单拆分与分批交付；退货、换货、直运、寄售的下单口径；合同建单提交与合同生效派生两个时点的五项校验；ATP 的简化判定调用侧 |
| 第 5.2 章 客户信用额度校验 | 信用额度字段的读取、下单时的信用占用与可用额度校验、三部分占用构成与不重复占用、超额时阻断或转审批两条路径 |
| 第 5.2 章 CPQ | 下单时的价格权限校验、折扣及其审批随合同审批链执行 |
| 第 5.5 章 订阅与租赁 | 订单头的订阅与租赁类型标记及周期与租期字段，复用分批交付、变更版本与审批链路 |
| 第 5.5 章 电子签章与印章连接器 | 合同审批通过后发起签署、回传签署结果与带签章的合同文件、印章使用留痕并归入合同附件与审计 |
| 第 5.6 章 模块规则 | 模块自有数据与迁移、禁止跨模块直接读写业务表、跨模块只用公开契约与版本化事件 |
| 第 8 章第 1 步 | 销售建单、自动带出客户产品价目与历史成交资料、五项校验、超额阻断或转审批 |
| 第 8 章第 2 步 | 四条审批链、管理层必经节点、不可越权跳过、审批意见版本附件全程留痕、电子签章与印章、合同生效的重新认证 |
| 第 8 章第 3 步 | 合同生效派生五类单据、Outbox 与持久化工作流驱动并保证幂等、双向可追溯、派生时校验重跑与待放行、派生失败进入死信与人工修复、合同变更后的重新派生 |
| 第 8 章第 8 步 | 交付确认单的登记与确认过账，同一事务内依次调用库存腿、过渡科目腿与凭证腿三个契约端口，事件信封带记账日期与会计期间，交付回写推进分批交付行、订单行与合同交付节点 |
| 第 8 章第 11 步销售侧 | 销售退货单的登记与前置红冲校验、直运退货的库存侧无流水、换货按退货与发货两笔事件组合表达 |
| 第 9.1 章流程引擎语义要求 | 派生编排的流程实例状态持久化、步骤幂等键、至少一次投递、补偿逆序与人工任务兜底、流程定义版本化 |
| 第 12.1 章 | 合同生效作为六类高风险操作之一的重新认证，认证方式、待签内容摘要、时间与设备写入审计证据 |
| 第 12.2 章 | 申请人不可自审、审批链不可越权跳过、默认拒绝、权限求值顺序按基线第 11.3 节 |
| 第 12.5 章 | 合同与订单的谁在何时对哪条记录做了什么、审批、重新认证一律写审计 |
| 第 15.1 章 | 本阶段全部错误按五类分类映射，每条错误含关联编号、发生时间、可否重试与处理建议 |
| 第 15.2 章 | 派生失败与交付确认回写失败进入死信与人工修复，不静默忽略 |
| 第 17.2 章 | 客户信用额度校验判据：F-51 已冻结为应收未收、已交付未开票与在途订单三桶且全部按价税合计，第 17.2 章末段的两桶旧复述作废；四端端到端测试；集成与契约测试中的电子签章连接器用例 |
| 第 17.3 章 | 合同、订单、发票、收付款可对账中的合同订单侧；权限不能跨法人越权 |
| 第 19 章阶段 3 | CLM 的续签、合并与三类到期提醒三个独立门槛；销售与 OMS 的订阅或租赁订单门槛；客户信用额度校验的四项门槛 |
| 附录 A.1 | 合同提交、合同审批提交、审批放行提交、销售订单提交、退货登记五个提交类度量项，销售订单表单打开与销售订单履约明细两个查询类度量项，合同生效派生的观察项 |

#### 10.2 PRD 节

| PRD 节 | 本阶段实现的功能 |
|---|---|
| 3.2 | 七类角色的操作边界与四端取值 |
| 3.3.1 至 3.3.4 | 合同建单的前置条件、头行字段、五项校验、草稿与提交的处理差异 |
| 3.4.1 至 3.4.3 | 四条审批链的不可变要求、审批动作三取值、电子签章与实体印章两条路径、合同生效的高风险控制 |
| 3.5.1 至 3.5.4 | 五类派生对象、派生机制与可追溯、派生时的校验重跑与待放行、合同变更后的五种重新派生情形 |
| 3.6 | 合同状态机十状态与全部流转、有效期止日不改变状态、合同版本与修订规则 |
| 3.7.1 至 3.7.3 | 关键条款的结构化字段与正文、收付款期次列表与合计校验、附件的版本化与四类用途 |
| 3.8 | 合同模板与条款库的版本化、模板版本号随合同留痕、经配置发布流程发布 |
| 3.9.1 至 3.9.3 | 履约记录投影、三类到期提醒触发源、续签的四条要求 |
| 3.10 | 合同合并的三条规则 |
| 3.11.1 至 3.11.5 | 订单只有派生一个来源、订单头行字段、订单变更的四条规则、拆分与分批交付的五条规则、订单状态机与分批交付行状态 |
| 3.12 | 五种订单类型的下单口径与首版边界 |
| 3.13.1 至 3.13.2 | 销售退货单的字段、四条前置校验、状态五取值；换货的组合表达与关联标记 |
| 3.14.1 至 3.14.5 | 信用额度字段与三项派生值展示、三部分占用构成与不重复占用、两个校验时点与判定规则、超额两条路径、首版边界 |
| 3.15 | 六类异常场景的错误分类与处理路径 |
| 3.16 | 本节涉及的度量项、四端取值、并发与数据规模前提、验收依据 |
| 2.8.3 | 建单时的取价与带出行为的调用侧，多行命中要求显式选择 |
| 2.9.1、2.9.3 | 销售侧历史成交资料的参考展示与显式选用后回填，回填后重新判定价格权限 |
| 10.3.1、10.3.2 | 合同生效在六类高风险操作中的触发点、重新认证的五步交互 |
| 10.5.2 | 合同到期提醒事项的产生与送达接入 |
| 无承载节 | 交付确认功能在 PRD 第 3 节与第 5 节均无小节，属附录乙 U-C-01，本阶段依据规格第 8 章第 8 步与第 5.2 章事件-分录表实现，冻结取值与切换代价见第 11.3 小节 |

---

### 11. 风险与预留

#### 11.1 已知技术风险

| 风险 | 影响 | 控制 |
|---|---|---|
| 信用三部分中的两部分由财务模块提供，财务阶段尚未交付时本阶段无法端到端验证 | E2E-6-04 属第三批 | 不注入任何替身，两桶取数按第 11.5 小节与阶段 10 的 `ReceivableExposureQuery` 同批交付同批验收，承载该取数的用例整体落在第三批，该批次之外本阶段不建该调用点；一套契约测试固化该 trait 的语义，同批接线时以同一套测试验证真实实现 |
| 派生项数与单张合同规模无上限时会产生长时批处理 | 派生观察项时长不可控，job-worker 池被占满 | 以 `EP__CLM__DERIVATION__MAX_ITEMS_PER_CONTRACT` 一条上限约束，派生项在批次内串行执行，不设并发配置键 |
| 电子签章外部系统不可用时合同长期停在待签署 | 闭环第 2 步阻塞 | 轮询上限 168 小时后置 FAILED 并进入死信；同时保留实体印章路径与人工上传已签文件的兜底入口，兜底入口同样要求验签与审计 |
| 信用校验对同一客户加行锁，极端情况下同客户下单串行 | 20 并发下同客户密集下单时的排队 | 锁粒度为法人加客户一行，`lock_timeout` 3 秒，超时返回业务冲突而非无限等待；把该场景纳入必测并发场景第 2 组并记录排队时长 |
| 合同快照 jsonb 随行数增长，版本表体积膨胀 | 备份与归档体量上升 | 快照只存合同头行条款节点期次与附件引用，不存附件正文与条款正文全文，条款正文以摘要与附件对象引用替代 |
| PRD 附录乙 U-E 组十六条已关闭决定全部落在本阶段 | 实现偏离冻结值会使信用、价格与审批口径不一致 | 逐条冻结取值见第 11.3 小节；U-E-03、U-E-07 与 U-E-10 已分别固化为三桶价税合计、`FINANCE_MANAGER` 单节点和三桶权威判据，不保留首版反向分支 |

#### 11.2 对基线的偏离项

1. `sales.sales_returns` 与 `sales.delivery_confirmations` 只带 `posting_date`，不带 `accounting_period_id`。该口径已被共享数据字典接受并冻结：凭证与子账条目保存会计期间，来源业务登记单据只保存业务日期；会计期间在 `confirm_delivery` 事务内由 `AccountingPeriodResolver::resolve` 唯一解析并随事件信封传给过账端，不在业务单据保存第二份。这里保留为历史偏离记录，现行开发不再存在反向分支。
2. 业务表上的 `customer_id` 已在共享数据字典澄清为 `mdm.customers` 业务客户档案的同法人逻辑外键，不是租户隔离列；禁止的是 `tenant_id`、`deployment_customer_id` 及同义列。隔离仍唯一依赖 `legal_entity_id`、强制 RLS 与密钥域，因此本项不再构成现行基线偏离。
3. `sales.order_validations` 与 `clm.contract_validations` 两处同构表。基线第 12 节禁止引入第二套机制。本处不是两套机制而是同一模型在两个模块内的本地存储，理由是基线第 1.3 节禁止跨模块直接读写业务表，订单侧的重跑发生在 `ep-app-sales` 事务内，无法写入 clm 的表。两表的列、枚举与序列化类型由 `ep-foundation` 中的同一类型导出，CI 校验两处 DDL 的列集合一致。
4. 电子签章不设公网入站回调，改由 job-worker 按退避调度并经固定管道逐次请求 integration-gateway 拉取签署状态。规格第 10.4 章只要求回传签署结果，未规定方向。偏离理由是首版公网侧只有供应商门户一个站点，新增入站入口会扩大规格第 21.17 章的暴露面，且单机形态下入站回调在停机窗口内会丢失。代价是签署结果的可见延迟上限等于一个轮询间隔。本条与第 4.13 小节即首版现行唯一决策和实现依据，不依赖未创建的另行 ADR。
5. `cpq.price_authorities` 固定由本阶段在 cpq schema 建表并交付迁移、维护用例与 `PriceAuthorityPort`；阶段 5 只拥有价目表与取价端口，不承载价格权限档案。该归属为首版唯一口径，不在实施期移交。

#### 11.3 本阶段新增决定与冻结取值

下列取值中标注编号的对应 PRD 附录乙已关闭事项；表内值均为首版冻结值，不再保留实施期选择。未标注编号的是基线与 PRD 均未覆盖、由本阶段新增且已经回写基线的决定。

| 事项 | 冻结取值或新增决定 | 是否阻塞本阶段 | 切换代价 |
|---|---|---|---|
| U-E-01 信用额度维护范围 | 按客户加法人分别设定 | 否 | 改为按客户全局时需改额度取数与三桶聚合的法人范围，涉及跨法人查询按基线第 3.8 节逐法人设置会话变量后合并 |
| U-E-02 额度为空的默认行为 | `null_limit_behavior` 默认 `TREAT_AS_ZERO`，配合 `on_exceed` 默认 `REVIEW`，使新客户首单转审批而非被阻断 | 否 | 改配置即可，无需改数 |
| U-E-03 三部分的价税口径 | 三桶与本次请求金额统一固定取价税合计；`amount_basis` 首版数据库 CHECK 只允许 `WITH_TAX`，不存在不含税配置或代码分支 | 否 | 改为不含税须新版本同时迁移 schema、财务查询 DTO、历史快照与全部信用测试 |
| U-E-04 预收是否抵减占用 | 不抵减，`deduct_advance_receipts` 默认 false | 否 | 改配置并要求财务侧端口追加预收余额返回项 |
| U-E-05 超额处置的配置粒度 | 法人级默认加客户级覆盖两层，出厂默认 `REVIEW` | 否 | 改为系统级时删除客户级覆盖列 |
| U-E-06 订单变更是否重跑信用 | 重跑，`recheck_on_order_change` 默认 true，只在提高金额或提前交期时触发 | 否 | 改配置即可 |
| U-E-07 转审批的审批人角色与层级 | 信用超额转审批走 `clm.contract_approvals` 中 `chain_kind = CREDIT` 的单节点链，出厂 RoleCode 固定 `FINANCE_MANAGER`，申请人不可自审；派生订单在 `on_exceed=REVIEW` 时追加同一链 | 否 | 未来替换审批链只经签名配置发布，不改表结构与状态机 |
| U-E-08 库存可用量与交期不通过的处置 | 建单提交时不阻断，只记录并使派生出的订单进入待放行。理由是规格第 8 章第 3 步明确规定库存可用量不足的派生单据置为待放行，若建单时直接阻断则该条路径永不可达 | 否 | 改为阻断时只需把校验项的处置由 FLAGGED 改为 BLOCKED |
| U-E-09 合同校验的具体内容 | 固定执行 PRD 3.3.3 的四项，见第 4.6 小节 | 否 | 未来追加或删除子项须版本化校验规则与测试 |
| U-E-10 规格第 17.2 章与第 5.2 章的信用额度判据不一致 | 已批准以第 5.2 章三桶为准：应收未收、已交付未开票与在途订单，三桶均取含税金额；第 17.2 章末段的两桶复述作废 | 否 | 不存在首版反向分支 |
| U-E-11 模板版本升级的影响范围 | 已套用旧版本的草稿、在审与已生效合同一律不受影响，模板版本号在合同上固化 | 否 | 改为影响草稿时需追加一次批量重套用的批处理用例 |
| U-E-12 合同提前终止与撤回 | 按 F-10 唯一机制执行：IN_PERFORMANCE，或 `derivation_state=FAILED` 的 EFFECTIVE，经 TERMINATION 审批进入非终态 TERMINATING 并发 `clm.contract.terminated.v1`；`platform.impact_assess` 建七类处置项，全部闭合后系统进入 TERMINATED 并发 `clm.contract.termination_completed.v1`。未注册规则仍建 PENDING 占位项，不得以 Noop 或直接 DONE 穿透。新开销项申请、新登记交付确认、订单变更、新建采购需求与合同修订在 TERMINATING/TERMINATED 均阻断；销售退货、发票作废/红冲、客户退款与采购退货四类反向补偿动作继续允许。收款期次置 VOIDED 前调用 `ReceiptPlanBillingQuery::billing_by_period`，仅净已开金额等于零者自动作废，大于零者由发票影响规则承接；交付节点与订单处置由对应 ImpactRule 完成。在审撤回仍按平台审批链承载，不另设动作 | 否 | 处置规则目录、状态机、事件、下游阻断和七个阶段规则必须整体版本化，不允许只改清单生成 |
| U-E-13 拆分粒度与变更字段清单 | 单订单行的分批交付行上限 60 条；已部分交付订单允许变更的字段为未交付部分的数量、交期与仓库，禁止变更单价与物料；关闭剩余数量须填原因并经销售负责人审批 | 否 | 改上限为配置项调整；改字段清单需同步改守卫条件 |
| U-E-14 退货单是否独立审批 | 设独立审批链 `sales.return`，默认节点 `SALES_MANAGER`；退货原因引用 MDM 的 RETURN_REASON，出厂编码固定 QUALITY_ISSUE、WRONG_ITEM、DELIVERY_DAMAGE、CANCELLED、OTHER | 否 | 未来调整链或显示名走签名配置发布；不得删除已引用编码 |
| U-E-15 订阅与租赁字段清单 | 周期单位取 DAY、WEEK、MONTH、QUARTER、YEAR 五值，周期长度为正整数，租期起止为日期，自动续期为布尔 | 否 | 字段增删按基线第 3.9 节的在线新增可空列执行 |
| U-E-16 关键条款结构化字段清单 | 交付节点清单、质保条款、违约责任、争议解决方式、合同义务清单五项为结构化，其余进条款正文 | 否 | 结构化字段增补走 `structured jsonb` 或新增可空列 |
| U-A-01 单据编号 | 合同类型码 CT、销售订单 SO、销售退货 SR、交付确认单 DC，四码按裁定 C-26 登记在 `docs/data-dictionary.md` 的单据类型码一节，格式按基线第 11.1 节 | 否 | 类型码改动只改序列配置 |
| U-B-03 管理者必经节点 | 合同 `EFFECTIVE` 出厂链固定含一个 `approver_kind=ROLE, role_code=MANAGEMENT_APPROVER` 必经节点；首版不按金额分档、不读部门上级链。申请人不可自审、角色无人时 fail-closed；替换为 ROLE、POSITION 或 DEPT_MANAGER 只能经签名配置发布 | 否 | 未来替换节点只改审批配置；金额条件需版本化扩展 schema |
| U-B-04 技术角色边界 | `TECHNICIAN` 只可登记安装、调试、维修与交付证据并读取无价格合同摘要；合同总额、单价、成本、毛利字段全部隐藏，不得最终交付确认、收入确认或财务审批。最终交付确认只允许 `SALES_MANAGER` 或 `PROJECT_MANAGER` | 否 | 出厂权限包逐能力绑定，不设首版反向开关 |
| U-B-18 敏感导出 | 本阶段任何导出均调用平台统一分类器：含敏感字段、对象密级 ≥30、行数达到敏感行数阈值任一命中即敏感，审计导出始终敏感；该阈值默认且最高 1000，只允许调低收紧；敏感导出重新认证并审批，非敏感也写审计；仅 XLSX、CSV、PDF，最多 50,000 行 | 否 | 分类阈值与阶段 11 U-I-11 共用 `EP__AUTHZ__EXPORT__SENSITIVE_ROW_THRESHOLD`，本模块不另设且不得高于 1000 |
| U-C-01 交付确认的承载节 | 已由裁定 A-09 关闭：单据主体固定归 sales，本阶段依据规格第 8 章第 8 步与第 5.2 章事件-分录表实现 | 否 | 首版无反向分支 |
| U-C-02、U-C-03 交付确认的操作者角色 | `WAREHOUSE_USER` 只负责出库/发运事实登记并绑定 sales.delivery.create；最终确认 sales.delivery.confirm 只绑定 `SALES_MANAGER`、`PROJECT_MANAGER`，`TECHNICIAN` 不得确认 | 否 | 角色能力映射作为出厂签名配置包的一部分交付 |
| U-G-01 库存可用量 | `available = on_hand - CONFIRMED/RELEASED 未交付订单剩余量`，按法人、仓库、冻结物料计算；动态聚合，不建预留表。确认/下达在固定组合锁内比较正增量，`available < requested` 即拒绝；取消、关闭和实际交付在同事务立即减少需求。阶段 8 只交付结存输入与 DTO，本阶段交付真实销售需求、最终组合和 A2 注册 | 否 | 改变口径须同时版本化订单需求状态、销售查询、库存 A2、并发锁与全部守恒测试，不能只改展示公式 |
| 价格权限档案的承载 | 本阶段在 cpq schema 建 `cpq.price_authorities` 并交付维护用例与迁移；三级取用顺序为 USER、POSITION、ROLE，三级均无命中时阻断提交。阶段 5 只提供价目表与取价端口 | 否 | 首版无移交分支；后续若改变模块边界须走架构变更而非实施期选择 |
| 电子签章的兜底入口 | 保留人工上传已签署文件的入口，同样要求验签、附件归档与审计，用于外部系统长期不可用时闭环不中断 | 否 | 无 |
| 合同派生项数上限 | 2000，超出拒绝生效并提示拆分合同 | 否 | 调配置 |

#### 11.4 为后续阶段预留的扩展点

1. `clm.contract_lines.order_type` 的五取值枚举与 `sales.sales_orders.order_type` 同源，寄售在库台账与代销结算恢复时只需在该枚举上增加分支处理，不改表结构。
2. `clm.contract_derivation_items.artifact_kind` 是开放枚举，后续新增派生物类型只需追加取值与目标模块端口，派生编排本身不改。
3. `sales.credit_policies` 只把 `deduct_advance_receipts` 作为现行策略开关；`amount_basis` 是被数据库固定为 `WITH_TAX` 的证据列，不是首版开关。信用评级模型与账期分级策略恢复时可在该表上扩展策略维度，但若改变价税口径必须按 U-E-03 做版本化迁移。
4. `ep-contract-clm::ContractQueryPort` 已把合同头、行、条款、交付节点四类投影分开暴露，客户 360 视图、经营驾驶舱的合同维度下钻与全文检索索引可直接消费，不需要为其新增读取通道。
5. `clm.signature_requests.provider_code` 预留多签章服务商并存，附录 B 的外部替换验收在阶段 4 只需新增一个 provider 实现并跑同一套契约测试。
6. `sales.delivery_schedules.promised_date` 是交付指标期间维度的唯一取数来源，`clm.contract_milestones.promised_date` 是合同交付节点侧的唯一取数来源，两者的字段命名与索引已按规格第 5.5 章经营驾驶舱条目的下钻口径准备，报表阶段经第 3.3 与 3.4 小节的四个受治理数据集视图取用，不需要再建物化投影表。

#### 11.5 跨阶段调用点的接线次序

本阶段与前后阶段之间共十一个跨阶段调用点，一律不使用空实现，也不设顺延验收台账。硬规则是跨模块同步调用的被调方必须与调用方同批交付，做不到就把该调用连同其用例整条推迟到被调方所在阶段，两者之外不存在第三种形态；任何返回零值、空集合、固定业务分支或恒定成功的实现在本阶段一律禁止，`apps/core-server/src/wiring/` 与 `apps/job-worker/src/wiring/` 两个目录下的全部文件中不得出现 `Noop`、`Stub`、`Fake`、`Dummy` 前缀的注入行，测试装配中的记录型桩不受此限。下表是本阶段全部跨阶段调用点的唯一出处。

| 跨阶段调用点 | 契约方法 | 处置 | 接线时点与缺席时的数据表征 |
|---|---|---|---|
| 库存可用量的销售需求组合 | 阶段 8 `ep_contract_inventory::StockOnHandQueryPort::on_hand_by_warehouse` + 本阶段 `ep_contract_sales::ConfirmedOpenSalesDemandQuery::summarize`，由本阶段 `AvailabilityQueryPort::available` 组合 | 同批交付 | 阶段 8 只交付结存输入与 DTO、不注册 A2；本阶段第二批交付真实销售需求、`SalesAwareAvailabilityQuery`、组合锁接线与 A2 路由并同批启用。任一部分缺席时 A2 路由不存在，订单确认/下达动作也不注册，禁止 reserved=0、空集合或只读结存的降级实现 |
| 库存不足补货扫描组合 | 阶段 8 `ep_contract_inventory::ReplenishmentPolicyReadPort::list_stored` + 本阶段同源 `SalesAwareAvailabilityQuery::available`，由本阶段 `ReplenishmentPolicyQuery::list_for_scan` 组合 | 同批交付 | 本阶段第二批交付 `SalesAwareReplenishmentPolicyQuery` 及 job-worker 真实装配；阶段 7 只消费该 trait 并按返回值建自动采购需求，不读取策略表、销售表或库存余额。任一 provider 缺席时扫描用例与定时任务均不注册，禁止空页、零值或只取 on-hand 的降级实现；分页上限固定 500 |
| F-10 影响面规则注册 | 阶段 3 `ep_platform_impact::{ImpactRule, ImpactRegistry, ImpactDisposeOutcome}` + 本阶段三个规则实现 | 同批交付 | 本阶段开工时阶段 3 平台本体、两表、唯一消费者与人工决策命令已真实存在；本阶段与三规则同批注入，注册数由 0 变 3。平台缺席时本阶段 F-10 整条不能退出，禁止新消费者、Noop 或直接 DONE；后续四类只以 PENDING 目录占位项表征 |
| F-10 来源闭合 | 阶段 3 `ep_platform_impact::ImpactSourceCompletionPort` + 本阶段 `ContractTerminationCompletionPort` | 同批交付 | 与终止端点和三规则同批注册；键固定为 `(CLM,clm.contract.terminated.v1)` 且恰一个。缺失、重复或替身使启动/模块启用失败关闭，平台不得直接写 clm；全部项目 DONE 后在 ImpactAssessor 调用方同一事务推进 TERMINATED 并发完成事件 |
| 收款计划净已开金额 | 阶段 10 `ep_contract_invoice::ReceiptPlanBillingQuery::billing_by_period(&mut dyn Tx,ctx,contract_id)` | 同批交付 | 阶段 10 交付读取 `invoice.invoice_receipt_plan_links` 正向分摊减 VOID/RED 反向分摊的真实实现时，与本阶段合同变更守卫、履约页投影及 `CLM_TERM_PAYMENT_SCHEDULE` 规则同批接线；净额大于零视为占用。缺席时这三条调用路径不注册，不允许空 map、clm 金额列、Outbox 回写或第二套聚合 SQL |
| 交付确认的过渡科目腿 | `ep_contract_finance::UnbilledArPort::record_on_delivery` | 同批交付 | 与阶段 10 该端口同批交付同批验收，三腿在本阶段第三批一次全真接线，该批次之外本阶段不建该调用点，`confirm_delivery` 用例与其端点不写入代码，不存在只落两腿的已确认交付 |
| 信用三桶中的已交付未开票与应收未收 | `ep_contract_finance::ReceivableExposureQuery::exposure` | 同批交付 | 按裁定 C-14 不进 T0 切片，与阶段 10 该端口同批交付同批验收，三桶取数在本阶段第三批当场成立，该批次之外本阶段不建该调用点，不存在只取两桶或取 `None` 的形态 |
| 销售退货的原交付实际金额、current live capture 与同步入库 | 阶段 11 `ep_contract_costing::DeliveryCaptureReturnBasisQuery::lock_available` + 阶段 8 `ep_contract_inventory::InventoryPostingPort::post_inbound` | 同批交付 | Stage 11 owner 以 `RevenueLiveFragment` 与 `CostLiveFragment { role: ReturnCostRole }` 返回全部当前开放叶片；本阶段据原交付累计差额生成 `{quantity,amount}` allocations，阶段 8 恒用 `InboundPricing::ReturnAtOriginalDeliveryCost`。`MainOperatingCost/DirectExpenseCost` 在 sales 内穷举映射两个冻结 MeasureKey，不导入 ledger 角色。任一真实端口缺席时登记用例和路由不注册，禁止任取一片、当前移动平均、单价反算、零金额补位或 Outbox 库存消费者降级 |
| 销售退货的红冲前置判定 | `ep_contract_invoice::InvoiceReversalStatusQuery::is_fully_credit_noted` | 同批交付 | 按裁定 C-16 不进 T0 切片，与阶段 10 该 trait 同批交付同批验收，`SALES.SALES_RETURN.INVOICE_NOT_CREDIT_NOTED` 判定在本阶段第三批当场成立，该批次之外本阶段不建该调用点 |
| 合同派生的采购需求派发 | `ep_contract_procure::PurchaseRequisitionIntakePort::intake` | 整条推迟 | 推迟到阶段 7，本阶段不写调用点，派生项 `status` 恒为 PENDING、`target_doc_id` 留空、不计入 `item_done` |
| 直运退货的采购侧勾稽 | `ep_contract_procure::PurchaseReturnLinkPort::link_drop_ship_return` | 整条推迟 | 推迟到阶段 7，本阶段不写调用点，直运订单在阶段 7 之前无从交付，退货由 `SALES.SALES_RETURN.QTY_EXCEEDS_DELIVERED` 自然阻断 |

销售退货的 owner live basis、库存入库与红冲前置判定行都属于第三批的硬依赖：阶段 11 的 `DeliveryCaptureReturnBasisQuery`、阶段 8 的库存写端口与阶段 10 的发票查询 trait 必须同批真实接线，不做替换动作；任一缺席时整条登记用例和路由不注册。阶段 10 端口表中先注入空实现再由本阶段替换的措辞已按总览第 1.5 节第八条整段撤销，本阶段不承接任何替换动作。已退货未冲回成本的置位方已由 F-51 U-C-09 固定为采购退货上的供应商拒绝动作，阶段 11 交付 `CostReturnMarkPort` 实现，本阶段不调用。项目任务派生按裁定 C-19 只登记不派发，不进本表，其端到端断言由阶段 12 承接，出处在第 8.4 小节。
