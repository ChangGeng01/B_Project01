> **F-57 状态：`HISTORICAL_DO_NOT_EXECUTE`。** 本文只保留历史任务正文；旧 **F57-20** 仅是需求所有权桶，不是现行 task 或执行顺序。当前权威集合为 [F-57 总体设计](../../specs/2026-08-23-f57-governed-automation-fabric-design.md)、[F-57 需求追踪矩阵](../../reviews/2026-08-23-f57-requirements-traceability.md)、[F-57 权威替代登记](../../reviews/2026-08-23-f57-authority-supersession-register.md)与 [2026-08-24 收敛实施主计划](../2026-08-24-f57-converged-program.md)；F-57 是设计/计划权威，不是产品已实现声明，本地模型实现延期。

## 阶段 8：库存与存货计价

### 0. 本阶段的定位与三条硬边界

本阶段交付 mdm 模块码之外的 inventory 模块的全部内容，即规格第 5.2 章库存与 WMS 条目、PRD 第 5 节的全部可实现项。本阶段有三条不可越界的规则，后续各小节的所有设计都是它的推论。

第一条，库存模块是库存数量账与库存金额账的唯一权威写入者（规格第 7.2 章第 663 行、PRD 第 5.8 节）。本阶段不提供任何直接改动库存数字的 HTTP 写端点，全部写入只经由本阶段对外暴露的过账端口，由来源业务事件的用例在其自身事务内调用。

第二条，本阶段不定义任何取价规则（PRD 第 5.1.2 节）。暂估取价、暂估回冲、价差拆分、超量开票反向匹配的入账单价、退货回冲的取价三分支、交付确认结转销货成本的取价，一律执行规格第 5.2 章财务规则条目的事件-分录表及其七个规则块。本阶段实现的是这些规则中依赖库存状态的那一部分算法与状态（移动加权平均单价、未被价差覆盖在库数量、结存数量、原单价读取），并把计算结果返回给调用方用于生成分录。借贷方向与科目一概不由本阶段决定。

第三条，凭证与子账共用同一会计期间归属（规格第 5.2 章子账与凭证共用同一期间归属块）。本阶段不解析会计期间，会计期间由业务事件的编排用例在同一事务内解析一次，并以入参形式同时传给库存与总账。该解析的唯一入口是阶段 9a 交付的 `AccountingPeriodResolver::resolve`，其第二步的零期间分支在该法人尚无任何期间时按记账日期所属自然月在同一业务事务内建立该期间并置 OPEN，本阶段不重复实现该分支，也不提供任何直接置位期间状态的入口。
本阶段整体排在贯通线 T0 之后，不向 T0 贡献任何最小切片，全部工作按在已贯通骨架上加厚的口径展开，详见第 2 节。三条硬边界之外，另有三条归属边界由跨阶段归属裁定固定，本阶段不得越界。其一，交付确认单主体归 sales 模块，`sales.delivery_confirmations` 与 `sales.delivery_confirmation_lines` 两张表由阶段 6 建立，本阶段不建任何单据表，只按裁定 A-09 提供交付确认的库存腿，即 `InventoryPostingPort::post_outbound`，`SourceDocType::DELIVERY_CONFIRMATION` 由 ep-app-sales 传入，直运行由 sales 侧整段跳过库存腿、本阶段不产生任何流水。其二，进项发票台账归 invoice 模块，`invoice.purchase_invoices` 与 `invoice.purchase_invoice_lines` 由阶段 10 建立，本阶段只提供价差拆分入口 `InventoryVariancePort::split_variance` 供 ep-app-invoice 调用。其三，取价一律归本阶段，总账只做分录映射与借贷平衡（裁定 C-13），收货登记与采购退货由采购模块在同一事务内先调本阶段的库存端口再调 `ep_contract_ledger::PostingPort::post`，ledger 侧不提供任何取价方法。

### 1. 交付物清单

本阶段结束时，仓库中存在以下可运行、可验证的产物。

| 序号 | 交付物 | 可验证形态 |
|---|---|---|
| D1 | ep-contract-inventory crate | 编译通过，首版终态含十个对外 trait 与全部命令、结果 DTO，无任何 IO 依赖；本阶段定义或实现其中六个，阶段 6 同批追加 `AvailabilityQueryPort` 与 `ReplenishmentPolicyQuery`，阶段 11 同批追加 `StockValueOutboundPort`，阶段 12 同批追加只读 `SerialStateQuery` 并由 ep-app-inventory 实现；本阶段另实现由阶段 5 定义在 ep-contract-mdm 的 `WarehouseDeactivationCheckPort`，四个后续 inventory trait 在本阶段均无空实现或注入行 |
| D2 | ep-domain-inventory crate | 编译通过，含库存四个聚合、补货策略值对象、计价与取价领域服务、五组不变量断言，`cargo test` 全绿，不含 sqlx 符号 |
| D3 | ep-app-inventory crate | 编译通过，含三个过账用例、一个停用校验用例、一个补货策略配置用例、A1 与 A3 至 A11 的十个查询投影、两个 `ReconCheck` 实现、`InventoryMaterialUsageProbe`、`InventoryReferenceCounter` 与 `InventorySubledgerBalanceQuery` |
| D4 | ep-adapter-db-pg 中的 inventory 仓储实现 | 十张表的仓储与查询实现，只访问 inventory schema |
| D5 | db/migrations/inventory 下既有 1 个 schema 前置迁移，加本阶段新增 13 个迁移文件，本阶段退出时共 14 个；阶段 10 再追加 1 个后建 invoice 外键追补，首版目录终态共 15 个 | `--check` 模式下迁移历史版本一致，全部表 RLS 已 ENABLE 且 FORCE；本阶段不得再次创建 `inventory` schema |
| D6 | core-server 上的 HTTP 端点：本阶段注册 A1、A3 至 A12 共 11 个，A2 在阶段 6 与销售需求提供者同批注册，首版终态共 12 个；只有 A12 为补货策略配置写入口，其余均只读 | 各自交付批次均可用 curl 打通，返回基线第 5.2 节封套；阶段 6 完成后端点集合恰为 A1 至 A12 |
| D7 | 两个 `ep_platform_recon::ReconCheck` 实现在 job-worker 的 `ReconRegistry` 注册并可运行 | 注入差异后写入 `platform_core.recon_discrepancies`，可追溯 |
| D8 | ep-testkit 中的 `InventoryPostingDriver` 与七个构造器 | 集成测试可在无采购、销售、发票模块的情况下驱动全部库存路径 |
| D9 | ep-datagen 中的库存流水生成器 | `--scale=default` 产出 50 万条库存流水、36 个会计期间的基准数据集 |
| D10 | docs 三处登记 | error-codes.md 新增 21 条、event-catalog.md 新增 1 条、data-dictionary 新增 10 张表 |
| D11 | 性能证据 | 四个附录 A.1 度量项的 EXPLAIN 输出与 P95 实测报告 |
| D12 | inventory 模块四端界面 | `clients/desktop/src/modules/inventory/` 与 `clients/mobile/src/modules/inventory/` 下的模块目录，桌面用例经 Playwright 与 tauri-driver、移动用例经 XCUITest 与 Espresso 通过 |
| D13 | 受治理数据集视图 `inventory.v_stock_value_entries` | 视图存在且含 legal_entity_id、security_level、data_scope_tags 三列，已授予 `ep_analyst_ro`，列签名与阶段 11 的 `reporting.dataset_fields` 登记一致 |

本阶段交付 inventory 模块四端界面的 A1、A3 至 A11 十个查询面（裁定 A-23），位置固定为 `clients/desktop/src/modules/inventory/` 与 `clients/mobile/src/modules/inventory/`；A2 可用量查询面由阶段 6 与真实销售未交付量提供者同批启用，首版终态为十一个查询面。具备 `inventory.replenishment_policy:write` 的管理用户还可在桌面配置页调用 A12；A12 只修改补货策略，不直接修改库存数量账或金额账。规格第 6.2 章库存台账与收发扫码一行四端取值均为完整，因此四端最终均实现完整查询视图。界面不新增任何直接修改库存数字的写入口，第一条硬边界不因补货配置下沉而放宽。

### 2. crate 与进程归属

新增四个 crate，改动两个既有 crate 与两个客户端工程，不新增任何进程。

| crate | 类型 | 装配进程 | 职责 |
|---|---|---|---|
| ep-contract-inventory | 新增 | core-server、job-worker | 对外 trait 与 DTO，只依赖 ep-foundation |
| ep-domain-inventory | 新增 | core-server、job-worker | 聚合、值对象、计价服务、取价判定、不变量断言、仓储端口 trait |
| ep-app-inventory | 新增 | core-server、job-worker | 过账用例、查询投影、补货策略配置、授权调用、审计与 Outbox 写入、两个 `ReconCheck` 实现、两个 mdm 侧探针实现与子账侧余额端口实现 `InventorySubledgerBalanceQuery` |
| ep-adapter-db-pg | 改动 | core-server、job-worker | 新增 `repo/inventory/` 目录下 7 个仓储文件与 1 个查询文件 |
| apps/core-server | 改动 | core-server | `apps/core-server/src/wiring/` 目录下注入 inventory 实现，本阶段注册 A1、A3 至 A12 共 11 个端点；A2 由阶段 6 与 `AvailabilityQueryPort`、销售未交付量查询同批注册，整个首版终态为 12 个 |
| apps/job-worker | 改动 | job-worker | `apps/job-worker/src/wiring/` 目录下向 `ReconRegistry` 注册 2 个对账检查、向 `MasterReferenceCounterRegistry` 注册 `InventoryReferenceCounter`、注入 `InventoryMaterialUsageProbe`，本阶段不注册任何事件消费者 |
| clients/desktop | 改动 | 桌面客户端 | `src/modules/inventory/` 下的库存台账、库存流水、两张报表、补货策略配置与扫码校验页面 |
| clients/mobile | 改动 | 移动客户端 | `src/modules/inventory/` 下的扫码录入与台账查询页面 |

依赖方向按基线第 1.3 节，逐条自查如下。ep-domain-inventory 只依赖 ep-foundation 与 ep-contract-inventory。ep-app-inventory 依赖 ep-foundation、ep-platform-authz、ep-platform-audit、ep-platform-outbox、ep-platform-obs、ep-platform-recon、ep-domain-inventory、ep-contract-inventory、ep-contract-mdm、ep-contract-ledger。其中 ep-contract-ledger 用于第 4.9 节第三项两个 `ReconCheck` 里存货项子账与总账勾稽一项的总账侧取数，即 `ep_contract_ledger::TotalAccountBalanceProvider`；该 trait 按阶段 9 计划第 1 节的 9a 段交付清单属 9a 段交付，9a 排在本阶段之前，故本阶段命名它即可编译，不构成对后续阶段的前置引用。本段依赖枚举是本阶段结束时的快照，按裁定 F-05 通则甲不具跨阶段封闭效力，后续阶段可在基线第 1.3 节允许项内增边而不回改本段。ep-app-inventory 不依赖任何其他模块的 application crate。ep-contract-inventory 不依赖 ep-contract-mdm，物料与仓库属性以扁平化的入参结构体传入，避免契约层横向耦合。

跨模块调用方向明确为单向：procure、sales、invoice、finance 四个模块的 application crate 依赖 ep-contract-inventory 并在装配时注入本阶段的实现，其中 finance 侧是裁定 G-01 把子账余额端口移到被调方契约后新增的第四个调用方；本阶段不反向依赖它们。mdm 的仓库停用用例依赖 ep-contract-inventory 的停用校验 trait，同样在装配时注入。
本阶段在调整后的阶段顺序 1 → 2 → 3a → 4 → 3b-1 → T0 → 5 → 9a → 8 → 6 → 7 → 10 → 11 → 9b → 14 中排在阶段 9a 之后、阶段 6 之前，整体落在贯通线 T0 之后；阶段 3b-2 不在这条链上，按阶段 3 计划第 3.0 节判定四的下游拉动点排在 T0 之后，阶段 12 与阶段 11 并行，阶段 13 与阶段 9b 并行。T0 从阶段 5、6、9a、10、11 各取一个最小切片，把一条合同从建单走到管理层看到一个数，五个切片逐项展开共十二项：一个客户、一个产品、一份单审批节点的合同、一张销售订单、一张销项发票、`invoice.tax_rate_options` 的建表与种子及 `TaxRateOptionQuery`、一个打开的会计期间、一张凭证、一笔到款登记与核销、最小应收台账、一个银行账户建档与一张收入报表，其中会计期间由 `AccountingPeriodResolver::resolve` 第二步的零期间分支在首次过账的同一事务内建立。十二项中没有库存项，因此本阶段不向 T0 交付任何切片，也不因 T0 而提前开工。本阶段整体按加厚口径施工：开工时客户、产品、合同、订单、销项发票、税率字典、到款、凭证、会计期间与收入报表已在骨架上真实跑通，本阶段做的是在这条已贯通的骨架上加库存两账与取价这一层。因此 ep-platform-recon 的对账框架、ep-contract-ledger 的过账端口与 ep-contract-mdm 的探针 trait 在本阶段开工时均已存在，本阶段不为任何端口注入空实现，也不存在只登记对账语句不执行的过渡期。本阶段在跨模块调用中一律是被调方在先的一侧，按被调方与调用方同批交付的硬规则，调用方阶段 6、阶段 7、阶段 10 各自接线并在其自身阶段完成该调用的验收，本阶段不为任何调用方预留占位实现，也不登记任何顺延项。反过来，交付确认单、采购收货单与采购发票分别由阶段 6、阶段 7、阶段 10 在其自身 schema 建立，本阶段只提供被它们调用的库存腿与价差拆分入口。本阶段实现但不拥有的三个 trait 见第 4.9 节，本阶段自有并自行定义的子账余额端口见第 5.1 节，注入位置为 `apps/core-server/src/wiring/` 与 `apps/job-worker/src/wiring/` 两个目录下的全部文件，不是单个 `wiring.rs` 文件。

进程归属：全部过账路径、查询路径与库存事件写入在 core-server 内执行；对账检查在 job-worker 内执行；portal-gateway 不接触库存数据，供应商门户不提供任何库存视图（PRD 第 4.9.1 节的能力边界内无库存项）。

### 3. 数据库变更

schema 固定为 `inventory`，属主角色 `ep_mod_inventory`，运行期读写走 `ep_app_rw`，对账走 job-worker 池的同一账号，只读分析走 `ep_analyst_ro`。十张新表，全部按基线第 4 节的公共列排列。以下表定义中未重复列出的公共列一律按基线第 4 节取值：`id uuid`、`legal_entity_id uuid`、`security_level smallint default 20`、`data_scope_tags text[] default '{}'`、`row_version bigint default 1`、`created_at`、`created_by`、`updated_at`、`updated_by`。仅追加表按基线第 4 节去掉 `row_version`、`updated_at`、`updated_by`。本阶段五张仅追加表都由正负数量/金额、移动方向和权威来源业务单据表达反向效果，没有逐行“冲销哪一条父事实”的真实父链，因而均不设 `reverses_id`；以后只有先冻结父链语义、同法人外键和累计上限才可为具体表新增该列。
公共列 `created_by` 在由 job-worker 的对账执行器或库存期初导入通道写入时取 `ep_foundation::SYSTEM_PRINCIPAL_ID`，即裁定 A-02 冻结的保留取值 `00000000-0000-7000-8000-000000000001`，不得另写全零值或其他自选值。

十张表各自建立 `UNIQUE(legal_entity_id,id)`。固定单目标引用全部建立真实 `ON DELETE RESTRICT` 外键：`warehouse_id` 指向 `mdm.warehouses(legal_entity_id,id)`，`material_id` 指向 `mdm.materials(legal_entity_id,id)`，`accounting_period_id` 指向 `ledger.accounting_periods(legal_entity_id,id)`，业务 `created_by/updated_by` 指向 `platform_authz.user_legal_entity_grants(legal_entity_id,user_id)`；movement、qty entry、value entry、variance split 的内部引用也统一使用同法人复合形状。为同时证明“同法人且属于同一 movement”，`stock_qty_entries` 与 `variance_splits` 另建候选键 `UNIQUE(legal_entity_id,movement_id,id)`；value、serial 与余额 last-pointer 对明细的引用必须使用该长键，不能退化为只按 `(legal_entity_id,id)` 命中另一 movement 的明细。`V20261016090300__inventory_create_variance_splits.sql` 在目标表建成后同文件补 `stock_value_entries(legal_entity_id,movement_id,variance_split_id) -> variance_splits(legal_entity_id,movement_id,id)`，先做空库/孤儿预检，不留下无约束窗口。带 `source_doc_type/source_module` 判别的来源单据及其来源行属于封闭多态组合，通常由库存 owner 在同一过账事务按判别值校验法人、单据、行与业务状态。唯一晚建例外是 `source_doc_type=PURCHASE_INVOICE` 的价差调用：此时父发票头行尚未插入，库存端只校验调用者必须是已装配的 invoice owner、预生成 id 的 UUIDv7/命令形状、上下文法人和来源头行 id 自洽，不执行必然查不到父行的 SELECT；阶段 10 在审计终结前重读完整父图，最终存在性、法人和头行归属由下述递延外键在提交时兜底。阶段 10 在目标行表建成后用 `V20261019090910__inventory_add_invoice_foreign_keys.sql` 补 `(legal_entity_id,source_doc_id) -> invoice.purchase_invoices(legal_entity_id,id)` 与 `(legal_entity_id,source_doc_id,source_doc_line_id) -> invoice.purchase_invoice_lines(legal_entity_id,purchase_invoice_id,id)` 两条真实外键。两条均固定为 `ON DELETE RESTRICT DEFERRABLE INITIALLY DEFERRED`：采购发票登记事务先写价差、后写带 NOT NULL 凭证/子账引用的发票父图，提交时再验完整父图；提交时父行缺失、法人不等或头行不一致均整笔回滚。追补前价差写入口不启用。

长外键只证明子行属于同一 movement，不能证明它属于该 movement 内的同一计价段。为封死“id 真实但挂到同 movement 另一段”的直写污染，第 9 号迁移在九张库存图表全部存在后建立一个公共校验函数 `inventory.assert_inventory_graph_consistent()`，并在 movement、qty、value、split、movement serial 五张仅追加事实表上建立 `AFTER INSERT DEFERRABLE INITIALLY DEFERRED` 约束触发器，在 qty/value/coverage balance 与 serial state 四张投影表上建立 `AFTER INSERT OR UPDATE DEFERRABLE INITIALLY DEFERRED` 约束触发器。函数按受影响的 movement 或投影键在提交时重读整张父子图；所有第 1 至 9 号迁移完成前库存写入口保持关闭，故不存在触发器安装前的可写窗口。精确判据见表 9 后的“库存图提交约束”，应用事务末断言与 R3 对账只作第二、第三道防线，不能替代数据库约束。

#### 3.1 表定义

表 1，`inventory.stock_movements`，库存移动事件头，仅追加。一次过账写一条。

| 列 | 类型 | 可空 | 约束与说明 |
|---|---|---|---|
| id | uuid | 否 | pk_stock_movements |
| legal_entity_id | uuid | 否 | RLS 判据 |
| security_level | smallint | 否 | 默认 20 |
| data_scope_tags | text[] | 否 | 默认 '{}' |
| business_date | date | 否 | 原始业务日期，取该业务事件的记账日期 |
| accounting_period_id | uuid | 否 | 会计期间归属，由调用方传入；与法人组成复合外键指向 `ledger.accounting_periods(legal_entity_id,id)` |
| accounting_period_seq | int | 否 | 该法人内会计期间的单调序号，随 accounting_period_id 一并传入，用于期间区间聚合 |
| deferred_from_period_id | uuid | 是 | `ResolvedPeriod` 的顺延来源；非空时与法人组成复合外键指向 `ledger.accounting_periods(legal_entity_id,id) ON DELETE RESTRICT`，供重放结果与原凭证期间逐值核对 |
| direction | text | 否 | ck_stock_movements_direction，取值 `IN`、`OUT`、`VALUE_ADJUST` |
| reason | text | 否 | ck_stock_movements_reason，取值见下文七项 |
| source_doc_type | text | 否 | ck_stock_movements_source_doc_type，取值 `PURCHASE_RECEIPT`、`PURCHASE_RETURN`、`DELIVERY_CONFIRMATION`、`SALES_RETURN`、`PURCHASE_INVOICE`、`MIGRATION_STOCK_ADJUSTMENT`、`MIGRATION_STOCK_HISTORY` |
| source_doc_id | uuid | 否 | 与 `source_doc_type/source_module` 组成封闭多态来源白名单，不建伪外键；owner 同事务校验 |
| source_doc_no | text | 否 | ck 长度 1 至 64 |
| source_module | text | 否 | ck 取值 `procure`、`sales`、`invoice`、`migration` |
| line_count | int | 否 | 计价段数，数据库 CHECK 固定 `1..=200`；应用层另校验 `lines.len() <= min(EP__INVENTORY__POSTING__MAX_LINES,200)`；同一来源业务行拆成多个价格/批次段时分别计数 |
| request_hash | bytea | 否 | `SHA-256(JCS(规范化过账命令))` 的 32 字节摘要；用于来源单据级重放校验，CHECK `octet_length(request_hash)=32`，不包含日志文本或明文附件 |
| created_at / created_by | | 否 | |

`reason` 的七项取值：`PURCHASE_RECEIPT`、`SALES_RETURN`、`DELIVERY_CONFIRMATION`、`PURCHASE_RETURN`、`PURCHASE_INVOICE_VARIANCE`、`MIGRATION_OPENING`、`MIGRATION_HISTORY`。同一张采购收货单可同时包含暂估段与超量开票反向匹配段，二者用明细金额流水的 `pricing_branch=ESTIMATED_PO_PRICE|OVERBILL_INVOICE_PRICE` 区分，头行始终只写一条 `reason=PURCHASE_RECEIPT`，不得按价格分支拆成两个 movement。采购退货是否已开票只决定 GRNI 与进项红字分段，不改变库存事实；未开票、已开票和同单混合的物料退货均只写一条 `reason=PURCHASE_RETURN` 的 movement，并受来源单据唯一键保护。`MIGRATION_OPENING` 承载库存期初，只允许在该法人尚无任何库存流水时执行；`MIGRATION_HISTORY` 只承载 Stage 14 已批准批次按业务日期、source record_seq、posting_line_key 稳定全序回放的历史 IN/OUT/VALUE_ADJUST，不供普通业务 API、Excel 或插件调用。

`source_module` 不由调用方提交，按 `source_doc_type` 封闭派生：采购收货/采购退货取 `procure`，交付确认/销售退货取 `sales`，采购发票取 `invoice`，两类迁移取 `migration`。`direction × reason × source_doc_type × source_module` 只接受九组：原六组 `IN/PURCHASE_RECEIPT/PURCHASE_RECEIPT/procure`、`IN/SALES_RETURN/SALES_RETURN/sales`、`OUT/DELIVERY_CONFIRMATION/DELIVERY_CONFIRMATION/sales`、`OUT/PURCHASE_RETURN/PURCHASE_RETURN/procure`、`VALUE_ADJUST/PURCHASE_INVOICE_VARIANCE/PURCHASE_INVOICE/invoice`、`IN/MIGRATION_OPENING/MIGRATION_STOCK_ADJUSTMENT/migration`，以及 `IN|OUT|VALUE_ADJUST/MIGRATION_HISTORY/MIGRATION_STOCK_HISTORY/migration` 三组。Stage 8 首次建表的原六组由 `V20261016090000` 锁死；Stage 14 的 `V20261023092600__platform_ops_harden_data_migration_evidence_graph.sql` 在启用 `inventory.stock_history` writer 前原子替换为上述九个完整 NULL-safe AND 分支，并同步 reason/source type 单列 CHECK。应用层在首笔 SQL 前做同一校验，数据库 CHECK 是绕过服务直写的最终闸门，不得只保留四个单列 CHECK。

索引：`pk_stock_movements`；`ix_stock_movements_legal_entity_id_created_at`（基线）；`ux_stock_movements_le_src_doc` 唯一，列为 `(legal_entity_id, source_doc_type, source_doc_id)`，这是本阶段的过账幂等根；`ix_stock_movements_le_period` 列为 `(legal_entity_id, accounting_period_seq, business_date)`；`ix_stock_movements_le_bizdate` 列为 `(legal_entity_id, business_date, id)`。

表 2，`inventory.stock_qty_entries`，库存数量流水，仅追加。基线第 3.2 节已登记该表名。

| 列 | 类型 | 可空 | 约束与说明 |
|---|---|---|---|
| id | uuid | 否 | pk |
| legal_entity_id | uuid | 否 | |
| security_level / data_scope_tags | | 否 | |
| movement_id | uuid | 否 | fk_stock_qty_entries_stock_movements，ON DELETE RESTRICT，同 schema 建真实外键 |
| line_no | int | 否 | ck 大于 0 |
| posting_line_key | text | 否 | 调用方稳定计价段键，ASCII 1 至 128 字节；同一 movement 内唯一，重放结果按此键重建 |
| source_doc_line_id | uuid | 否 | 沿用 movement 的来源判别，属于同一个封闭多态来源行组合 |
| source_doc_line_no | int | 否 | |
| warehouse_id | uuid | 否 | 与法人组成复合外键指向 `mdm.warehouses(legal_entity_id,id)` |
| material_id | uuid | 否 | 与法人组成复合外键指向 `mdm.materials(legal_entity_id,id)` |
| batch_no | text | 否 | 默认 `'-'`，ck 长度 1 至 64 且字符集为 `[A-Za-z0-9._-]`，空批次固定取 `'-'`（基线第 11.4 节） |
| quantity | numeric(18,6) | 否 | 与 direction 组成 `ck_stock_qty_entries_direction_quantity`：`(direction='IN' AND quantity>0) OR (direction='OUT' AND quantity<0)` |
| qty_balance_after | numeric(18,6) | 否 | ck 大于等于 0 |
| direction | text | 否 | 冗余自 movement，ck 取值 `IN`、`OUT` |
| business_date | date | 否 | 冗余自 movement |
| accounting_period_id | uuid | 否 | 冗余自 movement |
| accounting_period_seq | int | 否 | 冗余自 movement |
| created_at / created_by | | 否 | |

冗余四列的理由：收发存汇总与期末库存价值表按会计期间区间聚合，若每次都回连 movements 会在 50 万行规模上产生 hash join，实测无法稳定落在 10 秒通过线内。其与 movement 的逐值一致性由提交时库存图约束触发器强制；写入路径断言与 R3 对账仍保留，但不得成为可提交错误冗余值的理由。

索引与候选键：`pk`；`ux_stock_qty_entries_le_movement_id_id` 唯一，列为 `(legal_entity_id,movement_id,id)`，供长复合外键引用；`ux_stock_qty_entries_le_movement_line_no` 唯一，列为 `(legal_entity_id,movement_id,line_no)`；`ix_stock_qty_entries_legal_entity_id_created_at`（基线）；`ix_stock_qty_entries_le_dim_seq` 列为 `(legal_entity_id, warehouse_id, material_id, batch_no, accounting_period_seq)`；`ix_stock_qty_entries_le_bizdate` 列为 `(legal_entity_id, business_date, id)`；`ix_stock_qty_entries_movement` 列为 `(movement_id, line_no)`；`ux_stock_qty_entries_movement_posting_key` 列为 `(movement_id,posting_line_key)`；`ix_stock_qty_entries_legal_entity_id_material_id` 列为 `(legal_entity_id, material_id)`，供裁定 A-13 的物料引用探针做存在性判定，索引名与所在表按基线第 3.10 节的 `ix_<table>_<col…>` 规则一致，不登记任何命名例外。

表 3，`inventory.stock_value_entries`，库存金额流水，仅追加。基线第 3.2 节已登记该表名。

| 列 | 类型 | 可空 | 约束与说明 |
|---|---|---|---|
| id | uuid | 否 | pk |
| legal_entity_id | uuid | 否 | |
| security_level | smallint | 否 | 默认 20，金额与单价列另由字段级密级 30 控制 |
| data_scope_tags | text[] | 否 | |
| movement_id | uuid | 否 | fk_stock_value_entries_stock_movements |
| line_no | int | 否 | |
| posting_line_key | text | 否 | 与同段 qty_entry 或 variance_split 相同，同一 movement 内唯一 |
| qty_entry_id | uuid | 是 | 与 `legal_entity_id,movement_id` 组成 `fk_stock_value_entries_qty_entry`，指向 `stock_qty_entries(legal_entity_id,movement_id,id) ON DELETE RESTRICT`；VALUE_ADJUST 时为 NULL，其余必须非空 |
| source_doc_line_id | uuid | 否 | |
| source_doc_line_no | int | 否 | 来源业务行号；重放结果不得依赖跨 schema 回查 |
| warehouse_id | uuid | 否 | |
| material_id | uuid | 否 | |
| quantity | numeric(18,6) | 否 | 与同源数量流水同值，VALUE_ADJUST 时为 0 |
| amount | numeric(18,2) | 否 | 入库大于等于 0、出库小于等于 0、调整可正可负；方向组合 CHECK 固定此符号，单价为零时取 0，理由见第 4.3 节边界条件 |
| direction | text | 否 | 冗余自 movement，取 `IN|OUT|VALUE_ADJUST`；与 amount 的 CHECK 固定为 `(IN AND amount>=0) OR (OUT AND amount<=0) OR VALUE_ADJUST`，R3 逐行核对与 movement 一致 |
| applied_unit_price | numeric(18,6) | 否 | 本次实际取价，VALUE_ADJUST 时为 0 |
| pricing_branch | text | 否 | ck 取值见下文八项 |
| value_balance_after | numeric(18,2) | 否 | |
| qty_balance_after | numeric(18,6) | 否 | 该法人该仓库该物料全批次合计结存 |
| moving_avg_unit_price_after | numeric(18,6) | 否 | |
| variance_split_id | uuid | 是 | 与 `legal_entity_id,movement_id` 组成 `fk_stock_value_entries_variance_split`，指向 `variance_splits(legal_entity_id,movement_id,id) ON DELETE RESTRICT`；仅 VALUE_ADJUST 非空 |
| business_date / accounting_period_id / accounting_period_seq | | 否 | 冗余自 movement |
| created_at / created_by | | 否 | |

`pricing_branch` 的八项取值：`ESTIMATED_PO_PRICE`（采购收货暂估）、`OVERBILL_INVOICE_PRICE`（超量开票反向匹配）、`MOVING_AVERAGE`、`MOVING_AVERAGE_CLEARING`（任一出库使结存归零时按当前金额余额全额出清）、`ORIGINAL_DELIVERY_PRICE`（销售退货始终按原交付实际成本入库）、`VARIANCE_ON_HAND`（价差拆分的尚有库存部分）、`MIGRATION_OPENING`、`MIGRATION_HISTORY`。最后一项只允许 movement.reason=MIGRATION_HISTORY；历史 IN/OUT 逐段使用来源已规范化 amount，VALUE_ADJUST 使用来源 on_hand variance，owner writer 仍按前一段 after 值重算并拒绝数量、金额或移动平均连续性断裂。Stage 14 的 092600 同步替换该 CHECK 与 Rust 枚举后才启用 writer。物料采购退货不设原收货价或原暂估价库存分支，原金额只用于 GRNI 消费。统一表级形状 CHECK 固定为：`IN => quantity>0 AND amount>=0 AND qty_entry_id IS NOT NULL AND variance_split_id IS NULL`；`OUT => quantity<0 AND amount<=0 AND qty_entry_id IS NOT NULL AND variance_split_id IS NULL`；`VALUE_ADJUST => quantity=0 AND amount<>0 AND qty_entry_id IS NULL AND variance_split_id IS NOT NULL`。另建 `ck_stock_value_entries_after_non_negative`：三个 after 列均 `>=0`；建 `ck_stock_value_entries_zero_after_shape`：`qty_balance_after <> 0 OR (value_balance_after = 0 AND moving_avg_unit_price_after = 0)`。零价 IN/OUT 允许 amount=0；价差为零时不写 value entry，而不是写不满足 `amount<>0` 的占位行。

索引：`pk`；`ux_stock_value_entries_le_qty_entry` 在 `(legal_entity_id,qty_entry_id)` 上使用普通 `UNIQUE`，允许多行 NULL，但任一非空 qty entry 最多对应一条 value entry；`ux_stock_value_entries_le_movement_line_no` 为 `(legal_entity_id,movement_id,line_no)`；`ix_stock_value_entries_legal_entity_id_created_at`（基线）；`ix_stock_value_entries_le_seq_dim` 列为 `(legal_entity_id, accounting_period_seq, warehouse_id, material_id)`，这是期末库存价值表与收发存汇总金额侧的主查询路径；`ix_stock_value_entries_movement` 列为 `(movement_id, line_no)`；`ux_stock_value_entries_movement_posting_key` 列为 `(movement_id,posting_line_key)`；`ix_stock_value_entries_le_source_line_posting_key` 列为 `(legal_entity_id,source_doc_line_id,posting_line_key)`，供稳定段查询。

表 4，`inventory.variance_splits`，价差拆分记录，仅追加。

| 列 | 类型 | 可空 | 约束与说明 |
|---|---|---|---|
| id | uuid | 否 | pk |
| legal_entity_id / security_level / data_scope_tags | | 否 | |
| movement_id | uuid | 否 | 指向 VALUE_ADJUST 的 movement |
| source_doc_id / source_doc_no / source_doc_line_id / source_doc_line_no | | 否 | 采购发票登记单与其明细行；两条真实复合外键由阶段 10 的 `V20261019090910__inventory_add_invoice_foreign_keys.sql` 在目标建成后追补 |
| posting_line_key | text | 否 | 同一 movement 内稳定计价段键，ASCII 1 至 128 字节 |
| warehouse_id / material_id | uuid | 否 | |
| matched_quantity | numeric(18,6) | 否 | ck 大于 0 |
| total_variance_amount | numeric(18,2) | 否 | 本次匹配的发票不含税金额减本次回冲暂估金额，由调用方传入 |
| on_hand_quantity | numeric(18,6) | 否 | ck 大于等于 0 |
| issued_quantity | numeric(18,6) | 否 | ck 大于等于 0 |
| on_hand_variance_amount | numeric(18,2) | 否 | |
| issued_variance_amount | numeric(18,2) | 否 | |
| uncovered_before | numeric(18,6) | 否 | 本次匹配前的未被价差覆盖在库数量 |
| uncovered_after | numeric(18,6) | 否 | 本次匹配后的取值 |
| value_balance_amount_after | numeric(18,2) | 否 | 本行按稳定键处理后的存货金额余额快照，必须大于等于 0；供精确幂等重放，不跨表重算 |
| moving_avg_unit_price_after | numeric(18,6) | 否 | 本行处理后的移动平均单价快照，必须大于等于 0；供精确幂等重放 |
| business_date / accounting_period_id / accounting_period_seq | | 否 | |

表级 CHECK 五条：`ck_variance_splits_qty_split` 断言 `on_hand_quantity + issued_quantity = matched_quantity`；`ck_variance_splits_amount_split` 断言 `on_hand_variance_amount + issued_variance_amount = total_variance_amount`；`ck_variance_splits_coverage_non_negative` 断言 `uncovered_before >= 0 AND uncovered_after >= 0`；`ck_variance_splits_coverage_transition` 断言 `uncovered_after = uncovered_before - on_hand_quantity`；`ck_variance_splits_after_non_negative` 断言 `value_balance_amount_after >= 0 AND moving_avg_unit_price_after >= 0`。这些约束把规格第 17.2 章必测分支十一的两句判定和精确重放所信任的快照形状固化到数据库层。

索引与候选键：`pk`；`ux_variance_splits_le_movement_id_id` 唯一，列为 `(legal_entity_id,movement_id,id)`，供长复合外键引用；基线 ix；`ix_variance_splits_le_dim` 列为 `(legal_entity_id, warehouse_id, material_id, created_at)`；`ux_variance_splits_src_line` 唯一，列为 `(legal_entity_id, source_doc_line_id, warehouse_id, material_id)`；`ux_variance_splits_movement_posting_key` 唯一，列为 `(movement_id,posting_line_key)`。

表 5，`inventory.stock_qty_balances`，数量账余额，可更新。

| 列 | 类型 | 可空 | 约束与说明 |
|---|---|---|---|
| 公共列全套（含 row_version） | | | |
| warehouse_id / material_id | uuid | 否 | |
| batch_no | text | 否 | 默认 `'-'` |
| quantity | numeric(18,6) | 否 | ck_stock_qty_balances_non_negative 断言大于等于 0 |
| last_movement_id | uuid | 是 | 与 last_qty_entry_id 必须同空或同非空 |
| last_qty_entry_id | uuid | 是 | 非空时由 `(legal_entity_id,last_movement_id,last_qty_entry_id)` 指向 `stock_qty_entries(legal_entity_id,movement_id,id) ON DELETE RESTRICT`；提交触发器另强制该父行的 warehouse/material/batch 与本余额键逐值相等 |

索引：`pk`；基线 ix；`ux_stock_qty_balances_le_dim` 唯一，列为 `(legal_entity_id, warehouse_id, material_id, batch_no)`；`ix_stock_qty_balances_le_mat` 列为 `(legal_entity_id, material_id, warehouse_id)`，用于可用量查询按物料聚合。

数据库层的非负 CHECK 是规格第 17.3 章库存数量守恒的最后一道闸，与应用层的提交时校验共同构成两层防线。另建 `ck_stock_qty_balances_last_pointer_shape`，以 NULL-safe 表达式强制两个 last 指针同空或同非空；长复合外键保证同 movement 归属，提交触发器继续保证父 qty entry 与本余额的仓库、物料、批次键完全一致。

表 6，`inventory.stock_value_balances`，金额账余额，可更新。

| 列 | 类型 | 可空 | 约束与说明 |
|---|---|---|---|
| 公共列全套 | | | |
| warehouse_id / material_id | uuid | 否 | |
| quantity | numeric(18,6) | 否 | 全批次合计结存，ck 大于等于 0 |
| value_amount | numeric(18,2) | 否 | `ck_stock_value_balances_non_negative` 断言大于等于 0；负采购价差超过在库账面价值的部分转已出库价差，不制造负存货 |
| moving_avg_unit_price | numeric(18,6) | 否 | 派生值，结存为 0 时取 0 |
| last_movement_id | uuid | 是 | 非空时与法人组成复合外键指向 `stock_movements(legal_entity_id,id) ON DELETE RESTRICT` |

CHECK：`ck_stock_value_balances_non_negative` 断言 `value_amount >= 0 AND moving_avg_unit_price >= 0`；`ck_stock_value_balances_zero_price` 以 NULL-safe 表达式断言 `quantity > 0 OR (moving_avg_unit_price = 0 AND value_amount = 0)`，把规格第 5.2 与第 17.3 章的非负存货、零结存规则同时固化到数据库层；`quantity = 0` 时金额余额和单价任一非零均由数据库拒绝，不存在残值例外。

索引：`pk`；基线 ix；`ux_stock_value_balances_le_dim` 唯一，列为 `(legal_entity_id, warehouse_id, material_id)`。

表 7，`inventory.variance_coverage_balances`，未被价差覆盖在库数量，可更新。

| 列 | 类型 | 可空 | 约束与说明 |
|---|---|---|---|
| 公共列全套 | | | |
| warehouse_id / material_id | uuid | 否 | |
| uncovered_quantity | numeric(18,6) | 否 | ck_variance_coverage_balances_non_negative 断言大于等于 0 |
| last_movement_id | uuid | 是 | 非空时与法人组成复合外键指向 `stock_movements(legal_entity_id,id) ON DELETE RESTRICT` |

索引：`pk`；基线 ix；`ux_variance_coverage_balances_le_dim` 唯一，列为 `(legal_entity_id, warehouse_id, material_id)`。

表 8，`inventory.serial_states`，序列号当前状态，可更新。

| 列 | 类型 | 可空 | 约束与说明 |
|---|---|---|---|
| 公共列全套 | | | |
| serial_no | text | 否 | ck 长度 1 至 64 且字符集 `[A-Za-z0-9._-]` |
| material_id | uuid | 否 | |
| warehouse_id | uuid | 否 | `IN_STOCK` 与 `SHIPPED` 都保留最近一次所在仓库；与法人组成复合外键指向 `mdm.warehouses(legal_entity_id,id)` |
| batch_no | text | 否 | 默认 `'-'` |
| status | text | 否 | ck 取值 `IN_STOCK`、`SHIPPED` |
| last_movement_id | uuid | 否 | 与下一列共同组成长复合外键 |
| last_qty_entry_id | uuid | 否 | `(legal_entity_id,last_movement_id,last_qty_entry_id) -> stock_qty_entries(legal_entity_id,movement_id,id) ON DELETE RESTRICT`；提交触发器还要求存在同一 `(movement,qty_entry,serial_no)` 的 serial fact，且父 qty 的 material/warehouse/batch 与本状态一致，`IN_STOCK` 父方向为 IN、`SHIPPED` 父方向为 OUT |

索引：`pk`；基线 ix；`ux_serial_states_le_serial_no` 唯一，列为 `(legal_entity_id, serial_no)`，依据为 PRD 第 4.5.2 节序列号在该法人内唯一；`ix_serial_states_le_dim_status` 列为 `(legal_entity_id, warehouse_id, material_id, status)`。

表 9，`inventory.stock_movement_serials`，序列号出入库明细，仅追加。

| 列 | 类型 | 可空 | 约束与说明 |
|---|---|---|---|
| id / legal_entity_id / security_level / data_scope_tags | | 否 | |
| movement_id | uuid | 否 | fk_stock_movement_serials_stock_movements |
| qty_entry_id | uuid | 否 | 与 `legal_entity_id,movement_id` 组成 `fk_stock_movement_serials_qty_entry`，指向 `stock_qty_entries(legal_entity_id,movement_id,id) ON DELETE RESTRICT`；提交触发器强制本行 material/warehouse/direction/date/period 与该 qty 段逐值相等 |
| serial_no | text | 否 | 同上字符集与长度约束 |
| material_id / warehouse_id | uuid | 否 | |
| direction | text | 否 | ck 取值 `IN`、`OUT` |
| business_date / accounting_period_id / accounting_period_seq | | 否 | |
| created_at / created_by | | 否 | |

索引：`pk`；基线 ix；`ux_stock_movement_serials_entry_serial` 唯一，列为 `(qty_entry_id, serial_no)`；`ix_stock_movement_serials_le_serial` 列为 `(legal_entity_id, serial_no, created_at)`，用于序列号追溯。

库存图提交约束冻结如下，迁移实现不得删减为应用校验或事后对账。

1. 每条 qty/value/split/serial fact 的 `legal_entity_id`、`movement_id`、`security_level`、`data_scope_tags` 与 `created_by` 必须命中并等于同一 movement；其 direction、business_date、accounting_period_id、accounting_period_seq 等共有冗余列逐值相等。split 的 source_doc_id/source_doc_no 还必须等于 VALUE_ADJUST movement 的来源头；任一 NULL 偷渡或不等均拒绝。
2. IN/OUT movement 提交时必须同时满足 `qty_count = value_count = movement.line_count`。每条 qty 的 `line_no` 在 `1..=line_count` 且段内唯一，并有且只有一条 value；两行的 id 关联之外，`line_no`、posting_line_key、source_doc_line_id/no、warehouse_id、material_id、quantity、direction、business_date、accounting_period_id/seq 必须逐值相等。`ux_stock_value_entries_le_qty_entry` 负责“至多一条”，延迟触发器负责“至少一条”和逐值相等。
3. VALUE_ADJUST movement 提交时 `split_count = movement.line_count`。每条 split 的 posting_line_key 在 movement 内唯一；`on_hand_variance_amount <> 0` 时有且只有一条 value，等于 0 时不得有 value。存在的 value 必须以 variance_split_id 指向本 split，并与其 posting_line_key、source_doc_line_id/no、warehouse_id、material_id、business_date、accounting_period_id/seq 逐值相等，同时固定 `direction=VALUE_ADJUST`、`quantity=0`、`amount=on_hand_variance_amount`、`applied_unit_price=0`、`pricing_branch=VARIANCE_ON_HAND`、`value_balance_after=value_balance_amount_after`、`moving_avg_unit_price_after` 同值；value 的 `line_no` 必须位于 `1..=line_count` 且在 movement 内唯一。
4. 每条 movement serial 必须与其 qty parent 的 material、warehouse、direction、business_date、accounting_period_id/seq 一致。每条 serial state 的 last pair 不仅指向真实 qty，还必须命中同一个 serial fact，并与其 material、warehouse、batch 一致；`warehouse_id` 在两种状态均非空，状态方向对应关系固定为 `IN_STOCK/IN`、`SHIPPED/OUT`。
5. qty balance 的 last pair 必须命中同 warehouse/material/batch 的 qty。value balance 与 coverage balance 的非空 last_movement_id 必须在该 movement 下至少存在同 warehouse/material 的 value 或 split；全零/issued-only 价差没有 value 时以 split 证明该 movement 确实处理了本维度，不能把指针挂到同法人另一维度。
6. 触发器在 COMMIT 或显式 `SET CONSTRAINTS ALL IMMEDIATE` 时执行；任一父图缺行、行数错误、同 movement 错段、冗余值不等或投影指针错维度，整笔事务回滚。触发器查询使用 movement、posting key、qty/split id 和余额维度的既有唯一键/索引，不做无界全表扫描。

表 10，`inventory.replenishment_policies`，法人、仓库、物料维度的补货策略，可更新。它是 F-51 U-F-02 的唯一策略存储；采购模块只经本阶段契约读取，不得复制阈值。

| 列 | 类型 | 可空 | 约束与说明 |
|---|---|---|---|
| 公共列全套 | | | 含 `row_version`，每次配置变更按乐观锁更新并写审计 |
| warehouse_id | uuid | 否 | 与法人组成复合外键指向已存在的 `mdm.warehouses(legal_entity_id,id)`；`WarehouseQueryPort` 另校验启用状态 |
| material_id | uuid | 否 | 与法人组成复合外键指向 `mdm.materials(legal_entity_id,id)`；`MaterialQueryPort` 另校验启用且可库存 |
| reorder_point | numeric(18,6) | 是 | 与 `target_stock` 同为空表示停用该组合；启用时大于等于 0 |
| target_stock | numeric(18,6) | 是 | 与 `reorder_point` 同为空或同为非空；启用时大于等于 `reorder_point` |

表级约束 `ck_replenishment_policies_threshold_pair` 使用 NULL-safe 表达式固定为：两列同时为 NULL，或两列同时非 NULL 且 `target_stock >= reorder_point AND reorder_point >= 0`；禁止只填一列。索引为 `pk_replenishment_policies`、基线索引 `ix_replenishment_policies_legal_entity_id_created_at`，以及 `(legal_entity_id, warehouse_id, material_id)` 上的 `ux_replenishment_policies_le_warehouse_material`。策略行不物理删除；把两阈值同时置空即停用，保留历史审计与 `row_version`。同一组合不存在第二张策略表，也不把阈值写入余额表、物料档案或采购需求。

#### 3.2 RLS 策略

十张表全部带 `legal_entity_id`，逐表按基线第 3.8 节的模板生成，不写任何变体。

```sql
alter table inventory.<t> enable row level security;
alter table inventory.<t> force row level security;
create policy rls_<t>_le on inventory.<t>
  using (legal_entity_id = nullif(current_setting('app.legal_entity_id', true), '')::uuid)
  with check (legal_entity_id = nullif(current_setting('app.legal_entity_id', true), '')::uuid);
```

不设 BYPASSRLS 角色。对账组件按规格第 7.7 章的内部对账系统安全上下文逐法人遍历，每轮只写入单一法人的会话变量。

#### 3.3 迁移编号与顺序

目录 `db/migrations/inventory/`，迁移历史落在全局唯一的 `platform_core.schema_history`。执行顺序由单一全局 Runner 按文件版本号全序排定，不存在任何模块顺序声明文件。本阶段文件的版本号只需晚于其真实被引用的对象：第 1 号晚于阶段 2 建立 `ep_mod_inventory` 属主角色的引导脚本，第 12 号晚于阶段 2 建立 `platform_core.append_only_registry` 与 `attach_table_guards` 的迁移，第 2 至 11 号与第 13 号只引用本 schema 内先建的对象。本阶段全部文件的版本号早于阶段 7 的 procure 建表迁移与阶段 11 的 costing 文件，与阶段 8 排在阶段 7 与阶段 11 之前一致；`warehouse_id`、`material_id` 与 `accounting_period_id` 的目标都早于本阶段。movement 与其明细的来源单据组合是封闭多态，不建伪外键；`variance_splits` 的固定采购发票目标则由阶段 10 的 `V20261019090910__inventory_add_invoice_foreign_keys.sql` 追补，目标到位前相应写入口关闭。原句称本阶段版本号晚于其引用的 procure 对象的建表迁移，与本节上表的取值和阶段次序均不符，该句作废。

`inventory` schema 与属主授权已由现存前置迁移 `V20260901113000__inventory_create_schema.sql` 建立；该文件保持原版本、原校验和，不计入本阶段新增迁移。下表只列本阶段新增的 13 个文件。

| 序 | 文件名 | 内容 | 在线变更边界 |
|---|---|---|---|
| 1 | V20261016090000__inventory_create_stock_movements.sql | 表 1 加索引加 RLS | 新增表，可在线 |
| 2 | V20261016090100__inventory_create_stock_qty_entries.sql | 表 2 | 新增表，可在线 |
| 3 | V20261016090200__inventory_create_stock_value_entries.sql | 表 3 | 新增表，可在线 |
| 4 | V20261016090300__inventory_create_variance_splits.sql | 表 4 | 新增表，可在线 |
| 5 | V20261016090400__inventory_create_stock_qty_balances.sql | 表 5 | 新增表，可在线 |
| 6 | V20261016090500__inventory_create_stock_value_balances.sql | 表 6 | 新增表，可在线 |
| 7 | V20261016090600__inventory_create_variance_coverage_balances.sql | 表 7 | 新增表，可在线 |
| 8 | V20261016090700__inventory_create_serial_states.sql | 表 8 | 新增表，可在线 |
| 9 | V20261016090800__inventory_create_movement_serials.sql | 表 9；在九张库存图表齐备后建立 `assert_inventory_graph_consistent()` 与九表 DEFERRABLE 约束触发器 | 新增表与空图约束，可在线；完成前写入口关闭 |
| 10 | V20261016090900__inventory_create_replenishment_policies.sql | 表 10 加索引加 RLS | 新增表，可在线 |
| 11 | `concurrent/V20261016091000__inventory_create_report_indexes.sql` | 三条复合索引，两条报表专用与一条物料引用探针专用；文件位于 `db/migrations/inventory/concurrent/`，由独立非事务执行器依次 `CREATE INDEX CONCURRENTLY`，不得混入事务 DDL | 可在线，单次锁持有不超过 5 秒 |
| 12 | V20261016091100__inventory_backfill_append_only_registry.sql | 向 `platform_core.append_only_registry` 登记五张仅追加表 | 仅数据登记，可在线 |
| 13 | V20261016091200__inventory_create_dataset_views.sql | 建 `inventory.v_stock_value_entries` 并授予 `ep_analyst_ro` | 新增视图，可在线 |

每个新增文件头部带 `-- rollback:` 段。第 1 至 10 号的回退主体为对应的 `drop table`，这些新建空表的索引随建表文件以普通 `CREATE INDEX` 创建；第 9 号须先从八张已存在表与本文件新表删除库存图约束触发器，再删除公共校验函数和表 9，逆序回退不得留下引用已删除函数的触发器。第 11 号为 `drop index concurrently`，且是本阶段唯一位于 `concurrent/` 的非事务文件；第 12 号为删除本次登记的五行并 drop 该五张表上对应的 `assert_append_only` 触发器，第 13 号为 `drop view`。既有 schema 前置迁移不由本阶段回退；本阶段整体回退的终点是“schema 仍存在、十张业务表与本阶段视图均不存在”。迁移会话固定 `SET lock_timeout = '5s'` 与 `SET statement_timeout = '30min'`，不在迁移中调用应用代码，不在同一文件中既建表又回填数据。

#### 3.4 本阶段新增的命名决定

按基线第 0 节的纪律，以下三项基线未覆盖，本阶段决定并在阶段结束时回写基线。

一是余额类表的后缀。基线第 3.2 节定义了 `_lines`、`_entries`、`_links`、`_attachments` 四种后缀，未覆盖余额类。本阶段决定余额类表统一用 `_balances` 后缀，语义为按维度组合唯一、可更新、随流水同步维护的当前值。

二是二级明细表的命名。`stock_movement_serials` 是 `stock_qty_entries` 之下的第二级明细，不适用 `_lines`。本阶段决定二级明细表命名为主表单数加语义复数。

三是索引名超长时的缩写。PostgreSQL 标识符上限为 63 字节，`ux_stock_movements_legal_entity_id_source_doc_type_source_doc_id` 为 65 字节。本阶段决定超长时按列名取语义缩写并在数据字典中登记全称映射，缩写词表固定为 `le` 对应 legal_entity_id、`dim` 对应该表的完整维度列组、`seq` 对应 accounting_period_seq、`src` 对应 source。
#### 3.5 受治理数据集视图与仅追加登记

两项跨阶段登记随本阶段迁移一并交付，二者都不是本模块自用的结构，但由本模块作为基表所有者提供。

一是受治理数据集视图（裁定 A-18）。视图名固定为 `inventory.v_stock_value_entries`，dataset code 固定为 `inventory_stock_value_entries`，grain 取 ENTRY，由本阶段第 13 号新增迁移建立。视图必须含 `legal_entity_id`、`security_level`、`data_scope_tags` 三列，同一迁移内执行 `GRANT SELECT ON inventory.v_stock_value_entries TO ep_analyst_ro`，不授予 `ep_app_rw` 之外的任何写权限。视图取数为 `inventory.stock_value_entries`，不做聚合、不跨 schema 连接，金额与单价列的字段级密级仍为 30，投影口径与第 5 节一致。列名与类型签名必须与阶段 11 的 `reporting.dataset_fields` 登记一致，由阶段 11 的 `reporting-dataset-signature-matched` 按降级口径校验，即不一致时关闭该数据集对应的报表入口、登记降级窗口并告警，不拒绝进程启动；本阶段在退出条件中把该列签名同步给阶段 11。

二是仅追加登记（裁定 B-02）。本阶段第 12 号新增迁移向 `platform_core.append_only_registry` 登记五行，`schema_name` 一律取 `inventory`，`table_name` 依次取 `stock_movements`、`stock_qty_entries`、`stock_value_entries`、`variance_splits`、`stock_movement_serials`，`mode` 一律取 `APPEND_ONLY`，`mutable_columns` 一律取 `'{}'`。登记列以阶段 2 实建的四列为准，本阶段不写入这四列之外的任何列。文件内先按上述五行插入登记，再依次调用 `platform_core.attach_table_guards('inventory','stock_movements')`、`('inventory','stock_qty_entries')`、`('inventory','stock_value_entries')`、`('inventory','variance_splits')`、`('inventory','stock_movement_serials')`，顺序不得颠倒，挂接函数读登记表取可变列白名单，先挂接后登记取不到 `mutable_columns`。第 1 至 10 号建表迁移一律不调用 `attach_table_guards`，五张仅追加表的触发器只在本文件内挂接。该迁移的主要创建对象是 inventory 五张表上的仅追加触发器与其登记行，按裁定通则第五条放在 `db/migrations/inventory/` 目录下，其版本号晚于阶段 2 建立 `platform_core.append_only_registry` 与 `attach_table_guards` 的迁移，空库上按文件版本号全序执行时其前置对象已建立。登记与触发器的一致性由 `db/checks/append_only_consistency.sql` 断言，`xtask sqlcheck` 执行。

### 4. 领域模型与关键算法

#### 4.1 核心类型

跨模块边界值对象位于 `ep-contract-inventory/src/dto/value.rs`，领域 crate 直接复用，禁止在 domain 再声明一套同名类型；否则 contract 反向依赖 domain 会形成循环。`Money`、`UnitPrice`、`Quantity`、`SecurityLevel` 与法人/期间/仓库/物料 marker 复用 ep-foundation，库存自有行 marker 位于 `ep-contract-inventory/src/marker.rs`。

```rust
pub struct BatchNo(String);          // 不变式：非空、长度 ≤ 64、字符集受限；EMPTY 常量为 "-"
pub struct SerialNo(String);         // 同上字符集与长度
pub struct PostingLineKey(String);   // ASCII 1..=128；规范格式 `<source-line-uuid>:<segment-seq>`
pub enum SourceDocType { PurchaseReceipt, PurchaseReturn, DeliveryConfirmation,
                         SalesReturn, PurchaseInvoice, MigrationStockAdjustment,
                         MigrationStockHistory }
pub enum MovementDirection { In, Out, ValueAdjust }
pub enum SerialStatus { InStock, Shipped }
pub enum MovementReason { PurchaseReceipt, SalesReturn, DeliveryConfirmation, PurchaseReturn,
                          PurchaseInvoiceVariance, MigrationOpening, MigrationHistory }
pub enum PricingBranch { EstimatedPoPrice, OverbillInvoicePrice, MovingAverage,
                         MovingAverageClearing, OriginalDeliveryPrice,
                         VarianceOnHand, MigrationOpening, MigrationHistory }
pub struct SourceDocumentRef { pub doc_type: SourceDocType, pub doc_id: uuid::Uuid, pub doc_no: String }
pub struct SourceLineRef { pub line_id: uuid::Uuid, pub line_no: i32 }
pub struct OriginalCostAllocation { pub source_line_id: uuid::Uuid,
                                    pub quantity: Quantity, pub amount: Money }
pub enum StockMovement {}
pub enum StockQtyEntry {}
pub enum StockValueEntry {}
pub enum VarianceSplit {}
pub enum SerialState {}
```

上述字符串 newtype 只暴露验证构造器与只读访问器，不暴露 tuple 字段；`SourceDocumentRef.doc_no` 清洗后长度 1 至 64，`SourceLineRef.line_no > 0`。`Money` 本身允许有符号值，因此价差 DTO 直接使用 `Money` 并由字段不变量注明可正可负，不另引用不存在的 `SignedMoney` 类型。

领域内部键与聚合位于 `ep-domain-inventory/src/model/`，不进入公开 ABI。

```rust
pub struct StockQtyBalance { key: BatchKey, quantity: Quantity, row_version: u64 }
pub struct StockValueBalance { key: StockKey, quantity: Quantity, value_amount: Money,
                               moving_avg: UnitPrice, row_version: u64 }
pub struct VarianceCoverage { key: StockKey, uncovered: Quantity, row_version: u64 }
pub struct SerialState { serial: SerialNo, material_id: Id<Material>, warehouse_id: Id<Warehouse>,
                         batch: BatchNo, status: SerialStatus, row_version: u64 }
pub struct StockMovement { /* 仅追加聚合，含 qty_entries、value_entries、serial_rows */ }
```

其中领域私有 `StockKey { warehouse_id: Id<Warehouse>, material_id: Id<Material> }` 与 `BatchKey { warehouse_id: Id<Warehouse>, material_id: Id<Material>, batch: BatchNo }` 在 `src/model/key.rs` 定义；名称不从 contract 导出，也不与 marker `StockMovement` 混用。

#### 4.2 取价入参的表达

取价规则由规格第 5.2 章定义。采购收货的暂估/超量匹配分段由采购编排在同一事务内根据阶段 10 的反向匹配结果确定并逐段传入；库存模块只验证段数量、单价与允许的分支，不得再次查询发票或自行重做匹配。销售退货的金额由原交付追加事实按累计差额公式冻结，库存只验证并消费这些精确分配；普通出库与采购退货的最终分支仍依赖锁后库存状态。入参枚举如下，这是本阶段与采购、销售、发票三个模块之间最关键的契约。

```rust
pub enum InboundPricing {
    Explicit { branch: ExplicitInboundBranch, unit_price: UnitPrice }, // 采购收货分段、迁移期初
    ReturnAtOriginalDeliveryCost { allocations: Vec<OriginalCostAllocation> }, // 销售退货
}
pub enum ExplicitInboundBranch { EstimatedPoPrice, OverbillInvoicePrice, MigrationOpening }
pub enum OutboundPricing {
    MovingAverage,                                            // 交付确认发货
    ReturnAtMovingAverage,                                    // 全部物料采购退货；GRNI 原额由采购侧另行传给总账
}
```

对应关系逐条给出。`Explicit` 承载规格第 5.2 章采购收货事件的按采购订单不含税单价暂估、超量开票三条结清路径中路径一的已登记发票不含税单价，以及迁移期初；`branch` 必须与头部原因配对：`PurchaseReceipt` 只允许前两项，`MigrationOpening` 只允许后一项，其他组合拒绝为 `PLATFORM.REQUEST.INVALID_PAYLOAD`。MIGRATION_HISTORY 不扩展普通 `ExplicitInboundBranch`，只由 `MigrationModuleWriter` 调用 crate-private `post_migration_history(tx,ctx,record_id,batch_no,lines)`；该入口的每行显式含 direction、quantity、amount、source line、posting key、warehouse/material/batch/serial 与期间，固定写 MigrationHistory 三个枚举值并逐段复算 after 连续性，其他调用方在类型层不可构造。同比来源收货行需要两种价格时，调用方拆成两个 `InboundPostingLine`，二者可共享 `source_line`，但必须有不同且稳定的 `posting_line_key`，数量与序列号集合必须无重无漏。入库 `ReturnAtOriginalDeliveryCost` 只承载销售退货，并始终以调用方按原交付实际金额形成的 `allocations` 合计入库；它不得读取当前移动平均价，也不存在按当前结存选择分支。出库 `ReturnAtMovingAverage` 承载全部物料采购退货，不再按“已开票/未开票”选择库存计价；部分退货按锁后当前移动加权平均价，退货后数量归零时按退货前库存金额余额全额出清，当前账面价值为零时库存金额即为零。

采购发票是否已登记仍由 invoice 模块给出，但只决定采购侧消费开放 GRNI 还是先由链接进项红字重开 GRNI，不再决定库存出库价。销售退货的 `allocations` 由调用方从原交付数量、原库存金额事实与已登记退货累计量按累计差额公式形成；本模块保留 `ep_contract_inventory::InventoryPricingLookupPort::priced_segments_by_source_line`，按来源行返回全部稳定计价段，而不是把可能多段的收货行压成一个单价。销售交付行在首版必须恰有一个出库段，调用方以该段原始 `outbound_amount` 为分配上限，不从六位单价反算两位金额；采购收货行可有暂估与一个或多个超量匹配段，采购对账逐段勾稽。收货入账单价的权威出处唯一为 `inventory.stock_value_entries.applied_unit_price`，`procure.goods_receipt_line_costings` 按裁定 C-12 只保存 GRNI 追加效果；采购退货总账中的 `grni_consumed_amount` 由该效果链确定，和库存返回的 `inventory_return_amount` 不相等时由 `return_carrying_difference_amount` 承接。

#### 4.3 算法一：入库过账

入口输入是整份 `InboundPosting`，不是单行。文档 wrapper 与逐行内核的边界固定如下，逐行内核绝不创建 movement：

1. 先校验整份命令的法人、来源、原因、期间、标签、1 至上限条 lines、稳定键唯一性、同来源行分段数量与全局序列号集合；按 `posting_line_key` 升序规范化后计算 `request_hash`。任何可在无锁状态判定的错误都在首笔数据库写入前返回。
2. 调用方把来源 `(source.doc_type,source.doc_id)`、全部 availability/value/coverage/quantity 维度与序列号组成 F-50 计划，执行 `lock_all → reload → seal_after_reload`；本端口首条 SQL 前用 coordinator `assert_covers` 验证传入 proof 覆盖该完整子计划。来源 advisory、余额初始化/锁与序列号 advisory/state 锁均已由 Inventory slice 按 F-50 二十类全局顺序取得，本端口不得补锁。随后先查询唯一 movement：已有行且 `InventoryPostingCanonicalV1` 摘要相同则在任何当前 MDM 启停/批次/序列号策略校验前按第 6.3 节精确重放，摘要不同返回重复来源错误；不存在才重读已锁余额和当前 MDM 快照并继续。
3. 一次插入唯一 `stock_movements` 根，`line_count=lines.len()`；随后始终按稳定键顺序运行逐行内核。任一行失败时根、此前明细与余额更新随同一事务整体回滚。
4. 逐行内核先校验 `quantity > 0`；物料启用批次管理时 `batch_no` 必须非 `'-'`，未启用时必须等于 `'-'`；物料启用序列号管理时序列号条数必须等于 `quantity` 且 `quantity` 必须为整数。
5. 取价并冻结金额流水的 `applied_unit_price`。`Explicit { branch, unit_price }` 时 `unit = applied_unit_price = unit_price`，分支逐字取 `branch`；先按第 4.2 节校验 `reason × branch` 组合。`ReturnAtOriginalDeliveryCost` 时要求 allocations 非空、`source_line_id` 唯一、每项数量大于零且金额非负、`Σ allocations.quantity = 本行数量`；不等返回 `INVENTORY.STOCK_VALUE_BALANCE.ORIGINAL_PRICE_ALLOCATION_MISMATCH`。成立时 `amount=Σ allocations.amount`、分支固定为 `ORIGINAL_DELIVERY_PRICE`，并以 `applied_unit_price = round(amount / quantity, 6, MidpointAwayFromZero)` 形成展示/后续均价输入；不得改读当前 moving_avg、从原六位单价重算两位金额或任取首末段。
6. 计算金额。Explicit 的 `amount = round(unit × quantity, 2)`；销售退货的金额已经由第 5 步精确 allocations 合计给出，不二次舍入。两种路径均允许合法零金额。
7. 在已锁余额的内存工作副本上按稳定键顺序演算三类余额：数量账与金额账数量分别增加 `quantity`，金额余额增加 `amount`，未覆盖数量增加 `quantity`；以 Decimal 全精度计算每段 `moving_avg = round(value_amount / quantity, 6)`，把该段 after 快照固化到待写事实。共享同一余额键的后段以前段内存结果为 before，不在段间执行数据库 UPDATE。
8. 文档级 SQL 顺序固定为：一次 movement 根；随后按稳定键逐段写 `qty_entry → value_entry → stock_movement_serials`；全部子事实成功后，再按 distinct key 各执行一次 `UPDATE qty_balance`、一次 `UPDATE value_balance`、一次 `UPDATE coverage_balance`，并按每个序列号各更新一次 `serial_state`。每个余额的 `last_*` 指向影响该键的最后稳定段子事实，after 快照仍保留每段内存演算值。即时外键父行必须先存在；不得先更新 `last_*` 再补父行，也不得二次回填或让同一余额在一份命令内增加多次 row_version。全部完成后一次构造结果与单个待刷新事件并断言不变量。

边界条件：`quantity` 为零或负一律 `VALIDATION`；所有显式与移动平均 `unit_price`、每个原成本 allocation 金额必须大于等于 0，负值一律拒绝为 `PLATFORM.REQUEST.INVALID_PAYLOAD`，采购负价差只能走第 4.5 节。`unit_price` 或 allocations 合计为零时 `amount` 取 0，金额流水照写、金额账不变，不设跳过写入的分支，理由是第 4.8 节 I2 要求每条 `IN` 或 `OUT` 数量流水有且只有一条金额流水与之对应。第 4.4 节移动加权平均单价为零时的出库同此处理。

#### 4.4 算法二：出库过账

入口同样是整份 `OutboundPosting`。调用方必须先把来源、availability/value/coverage/quantity 与 serial 全集纳入同一 F-50 plan 并取得 proof；本端口先 `assert_covers`，再按 `InventoryPostingCanonicalV1` 执行精确重放，只有新来源才校验当前 MDM 并使用锁后余额。来源与余额/state 锁不得在端口内补取。新路径只创建一条 `line_count=lines.len()` 的 movement，最后按稳定键逐行运行下列内核。逐行内核不得创建 movement 或临时补锁；任一行失败整份单据回滚。

1. 校验数量、批次、序列号与第 4.3 节相同，序列号还需处于 `IN_STOCK` 且所在仓库与物料匹配。
2. `stock_qty_balances` 不存在即判定结存不足；校验 `qty_balance.quantity >= quantity`，不成立返回 `INVENTORY.STOCK_QTY_BALANCE.INSUFFICIENT_BALANCE`，错误详情携带当前结存与请求数量。本阶段一律硬阻断负结存。
3. 取价。`MovingAverage` 与 `ReturnAtMovingAverage` 都只读取已锁定 `value_balance` 的当前非负账面价值；数量未全数出清时 `unit = moving_avg`、分支为 `MOVING_AVERAGE`。`ReturnAtMovingAverage` 不读取原收货价，也不按发票登记状态切换库存计价。
4. 计算非负业务幅值，含出清归零规则。设 `qty_after = value_balance.quantity - quantity`。
   - 若 `qty_after == 0`，不论调用意图是普通出库还是采购退货，都令 `amount = value_balance.value_amount`，分支记为 `MOVING_AVERAGE_CLEARING`，并固定 `applied_unit_price = round(pre_clear_value_amount / quantity, 6, MidpointAwayFromZero)`；不得沿用可能因六位舍入而无法重构本次出清金额的旧 moving_avg。
   - 其余情形 `amount = round(unit × quantity, 2)` 或逐笔累加。
5. 在已锁余额的内存工作副本上按稳定键逐段演算：数量账与金额账数量分别减少 `quantity`，金额余额减少业务幅值 `amount`，`uncovered=max(0,uncovered-quantity)`，并重算非负移动平均价、固化逐段 after 快照。数据库 qty/value 流水数量均计划写负号，value amount 写 `-amount`，公开 `outbound_amount` 返回非负幅值。
6. 文档级 SQL 顺序与入库相同：一次 movement 根，按稳定键写完全部 `qty_entry → value_entry → stock_movement_serials` 子事实，最后按 distinct qty/value/coverage key 各 UPDATE 一次、每个 serial_state 各 UPDATE 一次；last ids 取该键最后稳定段，序列号最终置 `SHIPPED`。禁止段间更新、反序、二次回填或同键多次版本增加。全部完成后只返回一个 movement、一次登记事件并断言不变量。

#### 4.5 算法三：价差拆分

对应规格第 5.2 章价差拆分规则的全文。入口为 `InventoryVariancePort::split_variance`，调用方按裁定 A-10 收窄为 `ep-app-invoice`，采购模块不再直接调用本入口。命令以一张采购发票为头、携带 `(source_line, warehouse, material, matched_quantity, total_variance_amount)` 列表，整张发票只调用一次；无论价差是否全为零，都只产生一个受 `ux_stock_movements_le_src_doc` 保护的 VALUE_ADJUST movement，并为每条输入写一条可重放的 split。`total_variance_amount` 由调用方按本次匹配的发票不含税金额减本次回冲暂估金额算出，本模块不重算。

1. 调用方先把采购发票来源、全部 availability/value/coverage 维度组成同一 F-50 plan，执行 `lock_all → reload → seal_after_reload`。本端口首条 SQL 前 `assert_covers`；已有 movement 时按 `InventoryPostingCanonicalV1` 在当前 MDM 校验前精确重放，摘要不等则拒绝。新来源才校验每行 `matched_quantity > 0`、稳定键唯一、来源必须为 `PURCHASE_INVOICE` 与当前 MDM，并只重读已锁余额；不得自行取得来源、availability、value 或 coverage 锁。随后写唯一 movement，`line_count=lines.len()`。
2. 按 `posting_line_key` 升序逐行处理：`on_hand_quantity = min(matched_quantity, uncovered_quantity)`，`issued_quantity = matched_quantity - on_hand_quantity`。
3. 先算 `raw_on_hand_variance = round(total_variance × on_hand_quantity / matched_quantity, 2)`。比值以 Decimal 全精度计算，只在此处 round 一次。
4. 为保持存货金额非负，`on_hand_variance = max(raw_on_hand_variance, -value_balance.value_amount)`；`issued_variance = total_variance - on_hand_variance`。因此正价差按数量比例拆分；负价差最多把当前存货价值降至零，超出账面价值的负数与全部尾差都归已出库部分，不进入负存货。
5. 在已锁余额的内存工作副本上按稳定键逐行执行 `value_amount += on_hand_variance`、重算非负 moving_avg、`uncovered -= on_hand_quantity`，并固化每行 after 快照；共享同一 stock key 的后行以前行内存结果为 before，段间不更新数据库。
6. 文档级先写一次 movement，再按稳定键为每条输入写 `variance_split → optional value_entry`；所有子事实成功后，按 distinct value/coverage key 各 UPDATE 一次，last id 取该键最后一个非零 value entry（若全零则 last value entry 保持原值，last movement 可更新为本次 movement）。不得先写余额 last id或二次回填。`total_variance=0`、issued-only 与整批全零也不得省略 movement 或 split；其 `value_entry_id` 可以为空。
7. 返回唯一 movement 与输入键完全相等、按键升序的结果；`variance_split_id` 必有，`value_entry_id` 可空。调用方把 `issued_variance` 计入当期主营业务成本并生成分录。

跨发票不重复占用的证明链：`uncovered` 只在入库时增加、在出库时减少、在价差处理时按本次尚有库存数量扣减，登记发票本身不重置该数量（步骤 9 只减不增）。因此同一法人同一仓库同一物料先后由两张发票匹配时，第二张发票读到的 `uncovered` 已扣除第一张的占用，两张发票的尚有库存数量合计不超过该期间的实际在库数量。这正是规格第 17.2 章必测分支十一的两句判定，本阶段以集成测试 I-07 直接验证。

#### 4.6 出清归零与禁止零结存残值

本节是已同步回写基线第 3.5 节的现行统一规则，不再是待签署的偏离项。部分出库金额等于锁后移动加权平均单价乘出库数量并 round 到 2 位；任何出库使该法人该仓库该物料的全批次合计结存数量归零时，本次出库金额直接取退货或发货前库存金额余额全额。

理由：规格第 17.3 章存货金额账与数量账一致这一项对仍有结存时给出六位单价的确定误差上界，对零结存则要求库存金额余额与单价同时严格为零。若结存为零而金额余额留有尾差残值，强制不变量必然不成立，关账被拦截且无可达的解除路径（首版无盘点、无库存调整单据，PRD 第 5.6.4 节修复路径只有补登或冲正来源事件，无法消除纯舍入残值）。出清归零使零结存判据严格成立，且本次出库金额与按六位单价乘数量算出的差异落在该章已经冻结的舍入上界内，仍是当前账面价值计量。

影响范围：适用于交付出库和全部物料采购退货的最后一笔出库，其金额同源传给财务模块生成主营业务成本或存货贷方腿。采购退货另外取得本次消费的 GRNI 原额，两者差额按 `return_carrying_difference_amount = inventory_return_amount - grni_consumed_amount` 进入主营业务成本；这不是第二次库存写入，也不改变 GRNI 的原额。销售退货不受当前是否零结存影响，始终命中 `ORIGINAL_DELIVERY_PRICE` 并按原交付实际金额冲回，因此库存入库金额、SALES_RETURN 成本腿与原成本 capture 父链逐根同额。

无零结存残值例外。`ORIGINAL_DELIVERY_PRICE` 只用于销售退货入库且不再是 fallback，不是出库分支；原 `ORIGINAL_RECEIPT_PRICE` 与 `ORIGINAL_ESTIMATE_PRICE` 两个采购退货库存分支已撤销。未开票采购退货仍按原收货暂估金额消费 GRNI，但库存金额按当前账面价值出库，二者通过有符号主营业务成本差额腿守恒；不得为了保持两者表面相等而留下金额孤儿、制造负库存金额或豁免关账差异。

基线第 3.5 节已同步写入上述规则；本文、主规格第 5.2/17.3 章、阶段 7 的采购退货事务和阶段 9 的 `PURCHASE_RETURN_INVENTORY` 三腿映射共同构成唯一口径，无需开发前另行签字选择。

`value_amount` 在任何时刻都不得为负。发票不含税价显著低于暂估价时，第 4.5 节先把在库部分最多冲至零，剩余负价差归 `issued_variance` 并进入当期主营业务成本；合法低价发票仍可登记，库存返回值与总账端口需要的金额也始终是方向明确的非负存货腿或有符号成本价差，不再依靠事后对账解释负存货。

#### 4.7 序列号状态机

| 当前状态 | 事件 | 目标状态 | 守卫条件 | 违反时的错误码 |
|---|---|---|---|---|
| 不存在 | 入库 | IN_STOCK | 该法人内该序列号不存在 | 无 |
| SHIPPED | 入库 | IN_STOCK | 物料一致 | `INVENTORY.SERIAL_STATE.MATERIAL_MISMATCH` |
| IN_STOCK | 入库 | 拒绝 | 该序列号已在库 | `INVENTORY.SERIAL_STATE.ALREADY_IN_STOCK` |
| IN_STOCK | 出库 | SHIPPED | 物料一致且仓库一致 | `INVENTORY.SERIAL_STATE.WAREHOUSE_MISMATCH` |
| SHIPPED | 出库 | 拒绝 | 不在库 | `INVENTORY.SERIAL_STATE.NOT_IN_STOCK` |
| 不存在 | 出库 | 拒绝 | 不在库 | `INVENTORY.SERIAL_STATE.NOT_IN_STOCK` |

退货入库后允许再次发出，且允许在与原发货仓库不同的仓库入库，入库时更新 `warehouse_id` 与 `batch_no`。这是本阶段对未决事项 U-G-04 的冻结取值，理由是首版无调拨，若不允许换仓入库则退回到非原仓库的货物将永久不可发出，闭环断裂。

#### 4.8 五组不变量断言

五组断言必须在业务事实与投影写完后、幂等 `finish`/Outbox/审计终结批之前于同一事务内执行。任一不成立即中止并回滚当前请求，不得先写审计再继续执行数据库语句；失败事实由外层失败审计通道在其独立短事务中以审计作为最后一批写入，不中止进程（基线第 10.2 节）。

I1，数量守恒：本次写入后 `qty_balance.quantity` 等于该维度全部数量流水 `quantity` 的代数和，且大于等于 0。
I2，两账同源：本次每条 `IN` 或 `OUT` 方向的数量流水有且只有一条金额流水与之对应（`qty_entry_id` 相等），且两者 `quantity` 相同、`accounting_period_id` 相同、`business_date` 相同。
I3，金额余额一致：`value_balance.value_amount` 等于该维度全部金额流水 `amount` 的代数和；`value_balance.quantity` 等于该维度全部批次的 `qty_balance.quantity` 之和。
I4，单价重算：`value_balance.quantity == 0` 时 `value_amount == 0 && moving_avg == 0`；大于 0 时 `moving_avg == round(value_amount / quantity, 6)`，且满足主规格第 17.3 章的六位单价舍入误差上界。
I5，未覆盖上界：`0 ≤ uncovered_quantity ≤ value_balance.quantity`。
#### 4.9 本阶段实现的外部 trait

三项，全部由其他阶段定义、本阶段实现，三个定义方阶段（5、5、9a）均早于本阶段，实现类型名与位置一律照跨阶段归属裁定，不另取名。原第四项存货子账侧余额提供者按裁定 G-01 改为本模块自有端口，移入第 5.1 节，不再计入本节。

一是 `ep_contract_mdm::MaterialUsageProbe::has_stock_movement(&self, ctx: &SecurityContext, material_id: Id<Material>) -> Result<bool, AppError>`，trait 由阶段 5 定义（裁定 A-13）。实现类型固定为 `InventoryMaterialUsageProbe`，位于 `crates/application/inventory/src/probe/material_usage.rs`，取数为 `inventory.stock_qty_entries` 上按 `(legal_entity_id, material_id)` 的数量流水存在性判定，命中索引 `ix_stock_qty_entries_legal_entity_id_material_id`。第 3.1 节表 1 的 `inventory.stock_movements` 不带 material_id 列，物料维度落在其明细表，因此判定与索引一并落在 `inventory.stock_qty_entries` 上，索引名与所在表一致，不登记命名例外。`inventory.stock_value_entries` 中 `qty_entry_id` 为空的纯金额调整行不参与该判定。该探针的注册判定不挂在启动自检上：`master-data-usage-probes-registered` 下沉为模块启用动作的前置校验，探针未注册则拒绝启用 inventory 模块，启用后不在每次进程启动时复判，八个进程不因该项拒绝启动。阶段 5 的档案停用校验对 inventory 的覆盖随本实现注入而成立，两阶段各自验收各自的部分，本阶段不接收也不登记任何顺延项。

二是 `ep_contract_mdm::MasterReferenceCounter`，trait 与注册表 `MasterReferenceCounterRegistry` 由阶段 5 定义（裁定 A-15）。实现类型固定为 `InventoryReferenceCounter`，位于 `crates/application/inventory/src/probe/reference_counter.rs`，`module_code()` 返回 `ModuleCode::Inventory`，`count_open_documents` 在 `MasterObjectKind::Material` 下返回该物料非零结存的仓库物料批次组合数，其余 object_kind 返回 0。本阶段不承担任何 `SalesTradeHistoryProvider` 或 `PurchaseTradeHistoryProvider` 实现。

三是 `ep_platform_recon::ReconCheck`，trait、注册表 `ReconRegistry` 与执行器由阶段 9a 交付（裁定 A-06）。本阶段实现两个检查并在 `apps/job-worker/src/wiring/` 目录下经 `ReconRegistry::register` 注册，两个即裁定 A-06 给本阶段固定的校验项数，不多也不少：库存数量守恒，`category()` 取 `INVARIANT`；存货项子账与总账勾稽，`category()` 取 `SUBLEDGER_VS_LEDGER`。两个取值逐字取自裁定 A-06 中 `platform_core.recon_check_definitions.category` 的两项 CHECK 取值，`ReconCategory` 的判别式与该两项一一对应，本阶段不另取名。两者的 `blocks_period_close()` 均返回 true，`run_batch` 的快照入参为 `&dyn SnapshotCtx`，分批规模取第 7 节的 `EP__INVENTORY__RECON__BATCH_SIZE`，差异事项写入 `platform_core.recon_discrepancies`。第 3.1 节与第 4.6 节提到的 R2、R3 两组判据落在这两个实现内，不另起第三个检查。本阶段十张表上的 `source_doc_id`、`source_doc_line_id`、`warehouse_id`、`material_id` 四类跨模块引用不另建 `CROSS_MODULE_LINK` 校验项，依据是裁定 A-06 给本阶段固定的校验项只有上述两个；其中 `warehouse_id` 与 `material_id` 是单目标引用，`source_doc_id` 与 `source_doc_line_id` 是多态来源单据引用，四者的引用存在性一律按基线第 3.3 节的跨 schema 引用规则处理，业务状态与法人一致性由 ep-contract-mdm 与来源模块的契约校验承担，本阶段不在阶段计划内另立口径，也不再以总览 R14 的未覆盖面清单作为依据。

### 5. API 契约

路径前缀 `/api/v1/inventory`，承载进程 core-server。A1、A3 至 A12 共 11 个由本阶段注册；其中 A12 是补货策略唯一写入口，其余为只读。A2 的契约在本节冻结，但必须等阶段 6 的销售订单未交付量提供者与组合实现同批完成后才注册，期间路由不存在且不得注入零值提供者。共同约定：请求头按基线第 5.6 节固定集合；`X-Legal-Entity-Id` 必填并经授权法人集合校验后写入 `app.legal_entity_id`；GET 不要求 `Idempotency-Key`，A12 必带 UUIDv7 `Idempotency-Key`；响应按基线第 5.2 节封套；分页、排序、过滤按基线第 5.3 节，`filter` 的十种算子中本模块的白名单逐端点给出。

金额相关字段的统一处理：`unit_price`、`amount`、`value_amount`、`moving_avg_unit_price` 四类字段的字段级密级为 30，调用方不具备该密级时字段整体从响应中省略而不是置空，也不返回错误，符合规格第 12.2 章字段级权限与 PRD 第 5.7.1 节数量可见不等于金额可见的口径。

| 序 | 方法与路径 | 用途 | 主要参数 | 响应要点 | 权限 |
|---|---|---|---|---|---|
| A1 | GET /api/v1/inventory/stock-balances | 库存台账查询，PRD 第 5.6.1 节 | filter[warehouse_id]、filter[material_id]、filter[material_category_id]、filter[batch_no]、filter[include_zero]、sort 白名单为 warehouse_code、material_code、batch_no、quantity | 行含 warehouse_id、warehouse_code、material_id、material_code、material_name、batch_no、uom_code、quantity；具备金额密级时按仓库物料附 moving_avg_unit_price 与 value_amount | inventory.stock_balance:read |
| A2（阶段 6 同批启用） | GET /api/v1/inventory/available-quantities | 可用量查询，PRD 第 5.6.2 节 | material_id 必填、warehouse_id 可选，为空时按该法人全部已启用仓库分别列出并给出合计 | 行含 warehouse_id、material_id、quantity、reserved_quantity、available_quantity；`reserved_quantity` 是已确认或已下达且尚未交付的销售订单剩余量，`available_quantity = quantity - reserved_quantity`；meta 含 total_quantity 与 total_available_quantity | inventory.stock_balance:read |
| A3 | GET /api/v1/inventory/stock-values | 库存金额查询，PRD 第 5.7.1 节 | filter[warehouse_id]、filter[material_id]、filter[material_category_id]、accounting_period_id 可选默认当前打开期间 | 行含 quantity、moving_avg_unit_price、value_amount；meta 含按仓库小计与法人合计 | inventory.stock_value:read 且密级 30 |
| A4 | GET /api/v1/inventory/stock-movements | 库存流水查询，PRD 第 5.6.3 节 | filter[warehouse_id]、filter[material_id]、filter[batch_no]、filter[serial_no]、filter[direction]、filter[source_doc_type]、filter[business_date]=between:、filter[accounting_period_id]=in: 两条检索路径都必须可用 | 行含 business_date、accounting_period_id、accounting_period_label、warehouse、material、batch_no、direction、quantity、amount、source_doc_type、source_doc_no、created_by；默认排序 business_date asc, source_doc_no asc | inventory.stock_movement:read |
| A5 | GET /api/v1/inventory/stock-movements/{id} | 单条流水详情 | 无 | 含全部明细行与序列号清单、value_entry 的 pricing_branch 与 variance_split_id | 同上 |
| A6 | GET /api/v1/inventory/batches | 出库选批次的候选列表 | warehouse_id 必填、material_id 必填 | 只返回 quantity 大于 0 的批次 | inventory.stock_balance:read |
| A7 | GET /api/v1/inventory/serials | 序列号在库状态批量查询，供扫码即时反馈 | filter[serial_no]=in: 上限 200 个、filter[warehouse_id]、filter[status] | 行含 serial_no、status、warehouse_id、material_id、batch_no | inventory.serial:read |
| A8 | GET /api/v1/inventory/serials/{serial_no} | 序列号追溯，PRD 第 5.6.3 节 | 无 | 含当前状态与按时间升序的全部出入库记录 | 同上 |
| A9 | GET /api/v1/inventory/stock-in-out-summaries | 收发存汇总表，PRD 第 5.7.2 节 | accounting_period_id 必填、filter[warehouse_id]、filter[material_id] | 数量侧按仓库物料批次给出期初数量、本期收入数量、本期发出数量、期末结存数量；金额侧按仓库物料给出期初金额、本期收入金额、本期发出金额、本期调整金额、期末金额；meta 带口径标注两条 | inventory.stock_movement:read，金额侧另需密级 30 |
| A10 | GET /api/v1/inventory/period-end-stock-values | 期末库存价值表，PRD 第 5.7.3 节 | accounting_period_id 必填、filter[warehouse_id] | 行含期末结存数量、期末移动加权平均单价、期末库存金额；meta 含按仓库小计、法人合计与同法人同期间存货科目余额的跳转参数 | inventory.stock_value:read 且密级 30 |
| A11 | GET /api/v1/inventory/replenishment-policies | 补货策略列表与配置页取数 | filter[warehouse_id]、filter[material_id]、filter[enabled]；默认按 warehouse_id、material_id 升序，游标分页每页上限 500 | 行含 warehouse_id、material_id、reorder_point、target_stock、enabled、row_version；两阈值均空时 enabled=false | inventory.replenishment_policy:read |
| A12 | PUT /api/v1/inventory/replenishment-policies/{warehouse_id}/{material_id} | 新建、修改或停用单一补货策略 | body 为 `{reorder_point,target_stock,expected_row_version}`；两阈值必须同空或同为非空，已有行必须给锁前版本，新行必须令 expected_row_version 为空 | 返回 A11 单行视图；同空表示停用而非删除；同组合并发只允许一笔按唯一键与 row_version 成功 | inventory.replenishment_policy:write |

A9 的两条口径标注是硬性响应字段，不是界面文案：一是收发存汇总的收入数量与发出数量不包含只影响金额账的调整、金额列包含该调整（PRD 第 5.5.7 节）；二是期初与期末按会计期间字段划分而不按原始业务日期划分，存在顺延入账时一笔记账日期属于上一期间的库存流水会计入本期间（PRD 第 5.7.2 节期间口径）。两条以 `meta.disclosures` 数组返回，由界面原样展示。

A10 不自行计算勾稽差额，差额由对账组件判定（PRD 第 5.7.3 节界面要求）。响应中的 `meta.ledger_reference` 只给出跳转所需的法人与期间参数，不携带总账侧金额，避免在本模块内形成第二处存货科目余额取数口径。

A12 先经 mdm 契约验证仓库与物料同法人且均启用、物料可库存，再按 `(legal_entity_id, warehouse_id, material_id)` 取得行锁并写入；引用不存在或不可见统一返回 `PLATFORM.AUTHZ.NOT_FOUND_OR_DENIED`，阈值组合或大小关系非法返回 `PLATFORM.REQUEST.INVALID_PAYLOAD` 并把 `details.field` 定位到对应字段，已有行版本不符返回 `PLATFORM.CONCURRENCY.STALE_VERSION`。成功路径写一条结构化审计，`before/after` 都含两阈值与版本，不发领域事件；阶段 7 的扫描在同一事务内按页调用 `ReplenishmentPolicyQuery::list_for_scan`，只返回两阈值均非空的启用行，`limit` 必须在 1 至 500，否则返回 `PLATFORM.REQUEST.INVALID_PAYLOAD`。

错误码（全部登记入 `docs/error-codes.md` 与 `ep-foundation::error::codes`）。

| 错误码 | category | HTTP | 触发点 |
|---|---|---|---|
| INVENTORY.STOCK_QTY_BALANCE.INSUFFICIENT_BALANCE | BUSINESS_CONFLICT | 409 | 出库结存不足 |
| INVENTORY.STOCK_QTY_BALANCE.NEGATIVE_RESULT | BUSINESS_CONFLICT | 409 | 写入后断言 I1 不成立 |
| INVENTORY.STOCK_MOVEMENT.DUPLICATE_SOURCE_DOCUMENT | BUSINESS_CONFLICT | 409 | 同一来源单据重复过账，唯一约束冲突 |
| INVENTORY.STOCK_MOVEMENT.QUANTITY_NOT_POSITIVE | VALIDATION | 400 | 行数量非正 |
| INVENTORY.STOCK_MOVEMENT.BATCH_REQUIRED | VALIDATION | 400 | 启用批次管理但批次为空 |
| INVENTORY.STOCK_MOVEMENT.BATCH_NOT_ALLOWED | VALIDATION | 400 | 未启用批次管理但传入非 `'-'` |
| INVENTORY.STOCK_MOVEMENT.BATCH_NOT_FOUND | BUSINESS_CONFLICT | 409 | 出库批次在该仓库该物料无结存 |
| INVENTORY.STOCK_MOVEMENT.LINE_LIMIT_EXCEEDED | VALIDATION | 400 | 单次过账行数超配置上限 |
| INVENTORY.STOCK_MOVEMENT.WAREHOUSE_INACTIVE | BUSINESS_CONFLICT | 409 | 仓库已停用 |
| INVENTORY.STOCK_MOVEMENT.MATERIAL_INACTIVE | BUSINESS_CONFLICT | 409 | 物料已停用 |
| INVENTORY.STOCK_MOVEMENT.PERIOD_REF_MISMATCH | VALIDATION | 400 | 传入的期间 id 与期间序号不配对 |
| INVENTORY.STOCK_MOVEMENT.MIGRATION_NOT_EMPTY | BUSINESS_CONFLICT | 409 | 迁移期初路径下该法人已存在库存流水 |
| INVENTORY.SERIAL_STATE.COUNT_MISMATCH | VALIDATION | 400 | 序列号条数不等于行数量 |
| INVENTORY.SERIAL_STATE.DUPLICATE_IN_LINE | VALIDATION | 400 | 行内序列号重复 |
| INVENTORY.SERIAL_STATE.ALREADY_IN_STOCK | BUSINESS_CONFLICT | 409 | 入库的序列号已在库 |
| INVENTORY.SERIAL_STATE.NOT_IN_STOCK | BUSINESS_CONFLICT | 409 | 出库的序列号不在库 |
| INVENTORY.SERIAL_STATE.WAREHOUSE_MISMATCH | BUSINESS_CONFLICT | 409 | 出库序列号所在仓库不符 |
| INVENTORY.SERIAL_STATE.MATERIAL_MISMATCH | BUSINESS_CONFLICT | 409 | 序列号所属物料不符 |
| INVENTORY.STOCK_VALUE_BALANCE.ORIGINAL_PRICE_ALLOCATION_MISMATCH | VALIDATION | 400 | 逐笔取价的数量合计不等于行数量 |
| INVENTORY.VARIANCE_SPLIT.MATCHED_QUANTITY_NOT_POSITIVE | VALIDATION | 400 | 匹配数量非正 |
| INVENTORY.STOCK_BALANCE.WAREHOUSE_HAS_STOCK | BUSINESS_CONFLICT | 409 | 仓库停用前置校验不通过，详情列出仍有结存的物料清单 |

仓库所属法人与来源单据法人不一致这一情形不新增错误码，按基线第 5.5 节的存在性泄漏统一处理返回 404 与 `PLATFORM.AUTHZ.NOT_FOUND_OR_DENIED`，与 PRD 第 5.9 节权限或策略拒绝分类不冲突：PRD 要求的是不回显无权数据，404 更严格。
#### 5.1 十个 inventory 对外 trait 与一个 mdm-owned 实现的完整签名

首版终态共十个 trait 位于 ep-contract-inventory。本阶段定义或实现其中六个：`InventoryPostingPort`、`InventoryVariancePort`、`InventoryPricingLookupPort`、`StockOnHandQueryPort`、`ReplenishmentPolicyReadPort` 与 `StockValueSubledgerBalancePort`。阶段 6 在其 `ConfirmedOpenSalesDemandQuery` 已有真实实现后同批追加 `AvailabilityQueryPort`，并实现 `SalesAwareAvailabilityQuery` 注册 A2；同批追加 `ReplenishmentPolicyQuery` 的组合实现 `SalesAwareReplenishmentPolicyQuery`，把本阶段的策略行与同一套可用量计算合成阶段 7 扫描所需视图。第九个 `StockValueOutboundPort` 按裁定 F-05 由阶段 11 与其实现类型 `InventoryStockValueOutboundQuery` 同批追加；第十个只读 `SerialStateQuery` 由阶段 12 为设备/工单接线同批追加到既有 contract，并在 ep-app-inventory 实现。本阶段另实现阶段 5 定义在 `ep-contract-mdm` 的 `WarehouseDeactivationCheckPort`，但不得把该 trait 搬进 inventory 契约造成 mdm→inventory 反向依赖。本阶段对四个后续 inventory trait 都不交付空实现、不写注入行，也不注册依赖它们的路由。写路径与查询路径的事务句柄一律取阶段 1 冻结的 `ep_foundation::port::Tx`，两个余额类端口不接事务句柄、改接同批冻结的 `&dyn SnapshotCtx`，见第 6.1 节；跨模块档案/单据 marker 取 `ep_foundation::id::marker`，库存自有 marker `StockMovement/StockQtyEntry/StockValueEntry/VarianceSplit/SerialState` 取 `ep_contract_inventory::marker`。阶段 7 曾用的 `StockInboundPort`、`StockOutboundPort`、`StockAvailabilityQueryPort` 三个名字按裁定 C-18 作废。

过账命令与结果 DTO 在 `crates/contract/inventory/src/dto/posting.rs` 一次冻结如下；字段不得由各调用模块另造同名近似结构。`posting_line_key` 是调用方为一次物理计价段生成的稳定 `PostingLineKey`：规范格式固定为小写来源业务行 UUID、冒号、从 1 开始的十进制段序号；段序号按 `(pricing branch code, batch_no, correlation id UUID bytes)` 升序确定，单段恒为 `:1`。同一来源业务行因暂估/超量匹配或跨批次而拆段时共享 `source_line`，但段键不同；重试从同一锁后分配集确定同一键，不生成随机值。该键原样存入数量/金额流水或价差拆分行。`period` 的三项期间值只允许由同一事务内 `AccountingPeriodResolver::resolve` 的 `ResolvedPeriod` 映射，任何调用方不得从日期重算 `accounting_period_seq`。

```rust
pub struct InventoryPeriodRef {
    pub business_date: chrono::NaiveDate,
    pub accounting_period_id: Id<AccountingPeriod>,
    pub accounting_period_seq: i32,
    pub deferred_from_period_id: Option<Id<AccountingPeriod>>,
}
pub struct PostingSecurityLabel {
    pub security_level: SecurityLevel,
    pub data_scope_tags: Vec<String>,
}
pub struct WarehousePostingRef {
    pub warehouse_id: Id<Warehouse>,
    pub legal_entity_id: Id<LegalEntity>,
    pub is_active: bool,
}
pub struct MaterialPostingRef {
    pub material_id: Id<Material>,
    pub legal_entity_id: Id<LegalEntity>,
    pub is_active: bool,
    pub is_stock_item: bool,
    pub batch_managed: bool,
    pub serial_managed: bool,
}
pub enum InboundReason { PurchaseReceipt, SalesReturn, MigrationOpening }
pub enum OutboundReason { DeliveryConfirmation, PurchaseReturn }

pub struct InboundPostingLine {
    pub posting_line_key: PostingLineKey,
    pub source_line: SourceLineRef,
    pub warehouse: WarehousePostingRef,
    pub material: MaterialPostingRef,
    pub batch_no: BatchNo,
    pub serial_nos: Vec<SerialNo>,
    pub quantity: Quantity,
    pub pricing: InboundPricing,
}
pub struct InboundPosting {
    pub legal_entity_id: Id<LegalEntity>,
    pub source: SourceDocumentRef,
    pub reason: InboundReason,
    pub period: InventoryPeriodRef,
    pub label: PostingSecurityLabel,
    pub lines: Vec<InboundPostingLine>,
}
pub struct OutboundPostingLine {
    pub posting_line_key: PostingLineKey,
    pub source_line: SourceLineRef,
    pub warehouse: WarehousePostingRef,
    pub material: MaterialPostingRef,
    pub batch_no: BatchNo,
    pub serial_nos: Vec<SerialNo>,
    pub quantity: Quantity,
    pub pricing: OutboundPricing,
}
pub struct OutboundPosting {
    pub legal_entity_id: Id<LegalEntity>,
    pub source: SourceDocumentRef,
    pub reason: OutboundReason,
    pub period: InventoryPeriodRef,
    pub label: PostingSecurityLabel,
    pub lines: Vec<OutboundPostingLine>,
}

pub struct MovementResult {
    pub stock_movement_id: Id<StockMovement>,
    pub legal_entity_id: Id<LegalEntity>,
    pub source: SourceDocumentRef,
    pub direction: MovementDirection,
    pub reason: MovementReason,
    pub period: InventoryPeriodRef,
    pub line_count: u16,
}
pub struct InboundLineResult {
    pub posting_line_key: PostingLineKey,
    pub source_line: SourceLineRef,
    pub stock_movement_id: Id<StockMovement>,
    pub qty_entry_id: Id<StockQtyEntry>,
    pub value_entry_id: Id<StockValueEntry>,
    pub quantity: Quantity,
    pub inbound_amount: Money,
    pub applied_unit_price: UnitPrice,
    pub pricing_branch: PricingBranch,
    pub qty_balance_after: Quantity,
    pub value_balance_quantity_after: Quantity,
    pub value_balance_amount_after: Money,
    pub moving_avg_unit_price_after: UnitPrice,
}
pub struct InboundPostingResult {
    pub movement: MovementResult,
    pub lines: Vec<InboundLineResult>,
}
pub struct OutboundLineResult {
    pub posting_line_key: PostingLineKey,
    pub source_line: SourceLineRef,
    pub stock_movement_id: Id<StockMovement>,
    pub qty_entry_id: Id<StockQtyEntry>,
    pub value_entry_id: Id<StockValueEntry>,
    pub quantity: Quantity,
    pub outbound_amount: Money,
    pub applied_unit_price: UnitPrice,
    pub pricing_branch: PricingBranch,
    pub qty_balance_after: Quantity,
    pub value_balance_quantity_after: Quantity,
    pub value_balance_amount_after: Money,
    pub moving_avg_unit_price_after: UnitPrice,
}
pub struct OutboundPostingResult {
    pub movement: MovementResult,
    pub lines: Vec<OutboundLineResult>,
}

pub struct VarianceSplitLine {
    pub posting_line_key: PostingLineKey,
    pub source_line: SourceLineRef,
    pub warehouse: WarehousePostingRef,
    pub material: MaterialPostingRef,
    pub matched_quantity: Quantity,
    pub total_variance_amount: Money, // 可正可负
}
pub struct VarianceSplitCommand {
    pub legal_entity_id: Id<LegalEntity>,
    pub source: SourceDocumentRef, // 只允许 PURCHASE_INVOICE
    pub period: InventoryPeriodRef,
    pub label: PostingSecurityLabel,
    pub lines: Vec<VarianceSplitLine>,
}
pub struct VarianceSplitLineResult {
    pub posting_line_key: PostingLineKey,
    pub source_line: SourceLineRef,
    pub variance_split_id: Id<VarianceSplit>,
    pub value_entry_id: Option<Id<StockValueEntry>>,
    pub matched_quantity: Quantity,
    pub on_hand_quantity: Quantity,
    pub issued_quantity: Quantity,
    pub total_variance_amount: Money, // 三项均可正可负
    pub on_hand_variance_amount: Money,
    pub issued_variance_amount: Money,
    pub uncovered_before: Quantity,
    pub uncovered_after: Quantity,
    pub value_balance_amount_after: Money,
    pub moving_avg_unit_price_after: UnitPrice,
}
pub struct VarianceSplitResult {
    pub movement: MovementResult,
    pub lines: Vec<VarianceSplitLineResult>,
}
```

命令级不变量也是契约的一部分：头部法人必须同时等于安全上下文当前法人、全部仓库/物料快照法人；物理 IN/OUT 过账要求仓库和物料当前启用且物料可库存；`PurchaseInvoiceVariance` 是对既有收货的后续结清，只要求仓库/物料仍存在、同法人、物料类型与原收货关系一致，不要求当前 `is_active=true`，因为停用只阻止新单据选择，不能使存量发票永久无法登记。`reason × source.doc_type` 必须命中第 3.1 节封闭组合；`lines` 为 1 至 `POSTING__MAX_LINES` 条；`posting_line_key` 在命令内唯一；输出集合与输入键集合完全相等并按键升序；入库与出库金额结果均为非负业务幅值，数据库 `OUT` 流水自行写负号，价差结果使用可正可负的 `Money`。每份命令恰有一个 movement；价差每条输入恰有一个 split，即使整批或单行价差为零也不例外。`PostingSecurityLabel` 必须能由当前 `SecurityContext` 写入且原样落到 movement、两账、序列号流水与 Outbox，禁止调用方借端口降密。多段共享来源行时，各段数量之和必须等于调用方锁定的来源业务行本次登记数量，序列号集合必须两两不交；这一来源总量由调用模块负责校验，库存端口负责段内、键内和序列号全局不变量。

```rust
// crates/contract/inventory/src/port/posting.rs
#[async_trait::async_trait]
pub trait InventoryPostingPort: Send + Sync {
    async fn post_inbound(&self, tx: &mut dyn Tx, ctx: &SecurityContext,
                          f50_lock_proof: &TransactionLockProof, cmd: InboundPosting)
        -> Result<InboundPostingResult, AppError>;
    async fn post_outbound(&self, tx: &mut dyn Tx, ctx: &SecurityContext,
                           f50_lock_proof: &TransactionLockProof, cmd: OutboundPosting)
        -> Result<OutboundPostingResult, AppError>;
    async fn find_movement_by_source(&self, tx: &mut dyn Tx, ctx: &SecurityContext,
                                     legal_entity_id: Id<LegalEntity>, source: SourceDocumentRef)
        -> Result<Option<MovementResult>, AppError>;
}

// crates/contract/inventory/src/port/variance.rs
#[async_trait::async_trait]
pub trait InventoryVariancePort: Send + Sync {
    async fn split_variance(&self, tx: &mut dyn Tx, ctx: &SecurityContext,
                            f50_lock_proof: &TransactionLockProof, cmd: VarianceSplitCommand)
        -> Result<VarianceSplitResult, AppError>;
}

// crates/contract/inventory/src/port/pricing_lookup.rs
pub struct PricedSegment {
    pub posting_line_key: PostingLineKey,
    pub source_line: SourceLineRef,
    pub direction: MovementDirection, // 仅 In 或 Out
    pub quantity: Quantity,           // 非负业务幅值
    pub amount: Money,                // 非负业务幅值
    pub applied_unit_price: UnitPrice,
    pub pricing_branch: PricingBranch,
}

#[async_trait::async_trait]
pub trait InventoryPricingLookupPort: Send + Sync {
    async fn priced_segments_by_source_line(
        &self,
        tx: &mut dyn Tx,
        ctx: &SecurityContext,
        source_doc_type: SourceDocType,
        source_doc_line_id: uuid::Uuid,
    ) -> Result<Vec<PricedSegment>, AppError>;
}

// 返回集合按 posting_line_key ASC，逐项来自同法人 stock_value_entries；
// quantity/amount 把 OUT 的数据库负号转成非负业务幅值，VALUE_ADJUST 不进入本查询。
// 未过账或不存在的来源行合法返回空集合；已过账且为 IN/OUT 的来源行返回全部稳定段。
// movement 的 source_doc_type 与请求不符、同 movement 重复稳定键或命中 VALUE_ADJUST 才是不变量故障。

// crates/contract/inventory/src/port/on_hand.rs（阶段 8）
pub struct AvailabilityQuery {
    pub legal_entity_id: Id<LegalEntity>,
    pub material_id: Id<Material>,
    pub warehouse_id: Option<Id<Warehouse>>,
}
pub struct AvailabilityView {
    pub warehouse_id: Id<Warehouse>,
    pub on_hand_quantity: Quantity,
    pub reserved_quantity: Quantity,
    pub available_quantity: Quantity,
}

#[async_trait::async_trait]
pub trait StockOnHandQueryPort: Send + Sync {
    async fn on_hand(&self, tx: &mut dyn Tx, ctx: &SecurityContext,
                     legal_entity_id: Id<LegalEntity>, warehouse_id: Id<Warehouse>,
                     material_id: Id<Material>, batch_no: &BatchNo) -> Result<Quantity, AppError>;
    async fn on_hand_by_warehouse(&self, tx: &mut dyn Tx, ctx: &SecurityContext,
                                  q: AvailabilityQuery) -> Result<Vec<(Id<Warehouse>, Quantity)>, AppError>;
}

// crates/contract/inventory/src/port/serial_state.rs（阶段 12 同批追加，阶段 8 不造占位实现）
pub struct SerialStateView {
    pub serial_state_id: Id<SerialState>,
    pub serial_no: SerialNo,
    pub material_id: Id<Material>,
    pub warehouse_id: Id<Warehouse>,
    pub batch_no: BatchNo,
    pub status: SerialStatus,
}

#[async_trait::async_trait]
pub trait SerialStateQuery: Send + Sync {
    async fn resolve_by_id(
        &self,
        tx: &mut dyn Tx,
        ctx: &SecurityContext,
        serial_state_id: Id<SerialState>,
    ) -> Result<SerialStateView, AppError>;

    async fn resolve_by_serial_no(
        &self,
        tx: &mut dyn Tx,
        ctx: &SecurityContext,
        serial_no: &SerialNo,
    ) -> Result<SerialStateView, AppError>;
}

// 两方法只读同法人 inventory.serial_states；不存在、不可见或法人不符统一返回
// PLATFORM.AUTHZ.NOT_FOUND_OR_DENIED，禁止用不同错误泄漏序列号存在性。warehouse_id
// 在两种状态都为非空强类型值并原样反映权威行；SHIPPED 行保留最后所在仓库，
// 不得由查询层擅自清空或包装成 Option。不提供创建、更新或跨法人查询方法。

// crates/contract/inventory/src/port/availability.rs（阶段 6 与真实销售需求提供者同批追加）
#[async_trait::async_trait]
pub trait AvailabilityQueryPort: Send + Sync {
    async fn available(&self, tx: &mut dyn Tx, ctx: &SecurityContext, q: AvailabilityQuery)
        -> Result<Vec<AvailabilityView>, AppError>;
}

// crates/contract/inventory/src/port/replenishment.rs
pub struct ReplenishmentPolicyKey {
    pub warehouse_id: Id<Warehouse>,
    pub material_id: Id<Material>,
}
pub struct StoredReplenishmentPolicyView {
    pub key: ReplenishmentPolicyKey,
    pub reorder_point: Option<Quantity>,
    pub target_stock: Option<Quantity>,
    pub row_version: u64,
}
pub struct ReplenishmentPolicyScanView {
    pub key: ReplenishmentPolicyKey,
    pub reorder_point: Quantity,
    pub target_stock: Quantity,
    pub available_qty: Quantity,
}

#[async_trait::async_trait]
pub trait ReplenishmentPolicyReadPort: Send + Sync {
    async fn list_stored(&self, tx: &mut dyn Tx, ctx: &SecurityContext,
                         legal_entity_id: Id<LegalEntity>,
                         after: Option<ReplenishmentPolicyKey>, limit: u16)
        -> Result<Vec<StoredReplenishmentPolicyView>, AppError>;
}

// 阶段 6 与 SalesAwareAvailabilityQuery 同批追加真实组合实现，阶段 7 只消费。
#[async_trait::async_trait]
pub trait ReplenishmentPolicyQuery: Send + Sync {
    async fn list_for_scan(&self, tx: &mut dyn Tx, ctx: &SecurityContext,
                           legal_entity_id: Id<LegalEntity>,
                           after: Option<ReplenishmentPolicyKey>, limit: u16)
        -> Result<Vec<ReplenishmentPolicyScanView>, AppError>;
}

// crates/contract/mdm/src/port/warehouse_deactivation.rs（阶段 5 定义，阶段 8 实现）
#[async_trait::async_trait]
pub trait WarehouseDeactivationCheckPort: Send + Sync {
    async fn assert_no_stock(&self, tx: &mut dyn Tx, ctx: &SecurityContext,
                             warehouse_id: Id<Warehouse>) -> Result<(), AppError>;
}
// crates/contract/inventory/src/port/subledger_balance.rs      阶段 8 定义（裁定 G-01）
#[async_trait::async_trait]
pub trait StockValueSubledgerBalancePort: Send + Sync {
    async fn balance(&self, snapshot: &dyn SnapshotCtx,
                     legal_entity_id: Id<LegalEntity>,
                     accounting_period_id: Id<AccountingPeriod>,
                     accounting_period_seq: i32) -> Result<Money, AppError>;
}

// crates/contract/inventory/src/port/stock_value_outbound.rs   阶段 11 同批追加（裁定 F-05）
#[async_trait::async_trait]
pub trait StockValueOutboundPort: Send + Sync {
    async fn outbound_amount(&self, snapshot: &dyn SnapshotCtx,
                             legal_entity_id: Id<LegalEntity>,
                             accounting_period_id: Id<AccountingPeriod>) -> Result<Money, AppError>;
}
```

五条调用约定随签名一并冻结。其一，交付确认的库存腿（裁定 A-09）：ep-app-sales 在 confirm_delivery 的同一事务内以文档级 `OutboundPosting { reason: OutboundReason::DeliveryConfirmation, source: SourceDocumentRef { doc_type: DELIVERY_CONFIRMATION, .. }, period, label, lines }` 调用 `post_outbound`，每条 `OutboundPostingLine.pricing=OutboundPricing::MovingAverage`，再按 `posting_line_key` 把 `outbound_amount` 映射为销售行 `cogs_amount`，所有行的 `stock_movement_id` 必须与结果头一致；`is_drop_ship` 为真时由 sales 侧整段跳过该调用。其二，`StockOnHandQueryPort` 只提供库存权威结存；阶段 6 的 `AvailabilityQueryPort` 在同一事务内把它与 `ConfirmedOpenSalesDemandQuery` 组合，逐法人、仓库、物料计算 `reserved_quantity = CONFIRMED/RELEASED 销售订单行未交付剩余量之和`、`available_quantity = on_hand_quantity - reserved_quantity`，A2 与销售订单确认守卫共用这一个实现。取消、关闭与实际交付立即减少未交付量；确认或下达订单时先按 `(legal_entity_id, warehouse_id, material_id)` 固定顺序加锁，再锁内重算并在不足时拒绝，避免并发超卖。`on_hand` 仍是阶段 7 采购退货结存充足性前置校验的取数入口。其三，`ReplenishmentPolicyReadPort` 只返回本表持久化策略；阶段 6 的 `SalesAwareReplenishmentPolicyQuery` 对每个启用策略调用与 A2 同一个 `SalesAwareAvailabilityQuery`，按 `(warehouse_id, material_id)` 升序、每页最多 500 行返回 `available_qty`，不得在采购模块重算第二套可用量。其四，能力域码与动作类别（裁定 A-20）：A1、A3 至 A11 取 `CapabilityDomain::InventoryLedgerScan` 与 `ActionClass::Read`，A12 取 `CapabilityDomain::InventoryLedgerScan` 与 `ActionClass::Write`；阶段 6 同批启用的 A2 取 Read。常量按 `<USECASE_SCREAMING>_DOMAIN` 与 `<USECASE_SCREAMING>_ACTION` 声明在 `crates/contract/inventory/src/capability.rs`，`xtask configdoc` 断言每个已注册 `/api/v1/` 路由都能解析到一对常量，缺失即构建失败。其五，子账余额端口（裁定 G-01，原裁定 B-08 的端口落位由该裁定修订）：实现类型固定为 `InventorySubledgerBalanceQuery`，位于 `crates/application/inventory/src/projection/subledger_balance.rs`，按 `accounting_period_seq <= target_seq` 返回该法人截至目标期间的存货金额账累计；`accounting_period_id` 只作法人归属与调用证据校验，不作 UUID 顺序比较。调用方是阶段 10 的 `ReconciliationItemQuery` 组装处，注入行由阶段 10 写入 `apps/core-server/src/wiring/` 与 `apps/job-worker/src/wiring/` 两个目录，本阶段不为其预留任何占位实现。`StockValueOutboundPort` 不在本阶段冻结调用约定：其调用方是阶段 11 的 `COSTING_INVENTORY_COGS_VS_STOCK_VALUE` 检查，trait、实现类型与注入行按裁定 F-05 由阶段 11 同批交付。

### 6. 并发与事务边界

#### 6.1 事务归属

库存过账、取价、结存与补货扫描端口不自行开启事务：三个过账/取价端口、`StockOnHandQueryPort`、`ReplenishmentPolicyReadPort`、阶段 6 同批追加的可用量与补货组合查询端口及停用校验端口的方法签名一律接受调用方传入的 `&mut dyn Tx`，该类型由阶段 1 在 `ep_foundation::port::tx` 冻结（裁定 A-01）。A12 是本阶段唯一拥有写事务的 HTTP 用例，由 core-server 的 `UnitOfWork::transact` 开且只开一个事务，顺序固定为策略行、幂等 `finish`、审计终结批；A11 走一个只读事务。第 5.1 节的子账余额端口与出库金额端口两者不接受 `&mut dyn Tx`，其入参为 `&dyn SnapshotCtx`，与本节末段两个 `ReconCheck` 的快照同源，同样不开事务，跨 crate 取具体句柄的 downcast 只允许出现在 ep-adapter-db-pg 内。全部方法在所属用例的同一事务内执行，符合基线第 10.3 节一个用例一个事务、禁止在一个 HTTP 请求内开启多个写事务。

一次采购收货登记的完整事务内容为：采购模块写收货单与订单行回写、库存模块写 movement 与两账、财务模块写凭证与应付账款暂估台账及同步投影、调用方执行幂等 `finish`、刷新 Outbox、写同事务通知命令、最后写审计终结批，全部在同一事务内提交，调用次序固定为先 `InventoryPostingPort::post_inbound` 后 `ep_contract_ledger::PostingPort::post`（裁定 C-13）。规格第 5.2 章要求财务模块按同一业务事件生成唯一一张总账凭证，且规格第 10.2 章关账受理后建立快照时要求该期间的全部凭证已可见，因此凭证不得延迟到 Outbox 消费时才生成。全部凭证一律与业务事件同事务生成、Outbox 只承载派生、通知、检索与报表数据集这一口径已由裁定 C-28 定死，本阶段以入参契约把它显式化：过账端口返回的每行金额是分录的存货腿金额，调用方必须在同一事务内使用它。

所有库存写路径遵守同一终结协议：跨模块 owner port 只立即写业务事实、子账和同步投影；其领域事件只登记到事务级待刷新集合，不得当场向 Outbox 表执行 SQL。事务所有者在所有凭证与投影完成后依次执行幂等 `finish`、刷新全部 Outbox 待写项、写同事务通知命令，最后调用 `AuditWriter::append_terminal`。`append_terminal` 封印 `Tx`，其后任何 SQL `query/execute`、仓储写入或跨模块端口调用都必须以内置不变量错误拒绝并令整个事务回滚，唯一允许的后继动作是 `UnitOfWork::commit`。共享跨模块契约测试 `audit_terminal_seals_tx` 以采购收货为夹具：审计后分别尝试库存仓储、`PostingPort` 与 Outbox 刷新，均应失败、审计后零新增行、事务整体回滚；正常路径由 recording transaction 断言审计批是 commit 前最后一批数据库执行。

隔离级别 `READ COMMITTED`（基线第 8.4 节）。对账检查不自行开事务，其快照由 `ep_foundation::port::UnitOfWork::snapshot_transact` 导出的 `SnapshotCtx` 承载，经阶段 9a 的 ep-platform-recon 执行器逐批传入 `ReconCheck::run_batch`。

#### 6.2 锁策略

所有余额行一律先 `SELECT ... FOR UPDATE` 再按 `row_version` 条件 UPDATE。行锁已持有时版本条件恒成立，`row_version` 保留作为防御性检查与基线第 3.7 节的一致性要求，冲突仍映射为 `PLATFORM.CONCURRENCY.STALE_VERSION`。

库存过账不再有一套端口私有锁序；唯一顺序是 F-50 二十类中的 Inventory 六级，一次过账内先规范化、排序、去重，再由 `InventoryF50LockSlice` 依次完成：

1. 来源 `(source_doc_type,source_doc_id)` 按枚举序与 UUID bytes 升序取得 `inventory-source:` transaction advisory，覆盖尚不存在的 movement 幂等根。
2. 全部 `(warehouse_id,material_id)` 升序取得 `sales-availability:` transaction advisory，使销售确认/释放、补货扫描和任何库存变化共享同一最高级组合锁。
3. `stock_value_balances`，按 `(warehouse_id, material_id)` 升序。
4. `variance_coverage_balances`，按 `(warehouse_id, material_id)` 升序。
5. `stock_qty_balances`，按 `(warehouse_id, material_id, batch_no)` 升序。
6. 完成前五类后，对命令内全部 `(legal_entity_id,serial_no)` 升序取得 `pg_advisory_xact_lock(hashtextextended('inventory-serial:'||legal_entity_id||':'||serial_no,0))`，覆盖尚不存在的 serial_state；随后以同序 `SELECT inventory.serial_states ... FOR UPDATE` 锁后重载已有行并执行状态机。所有入库、出库、退货与扫码写路径都必须使用同一前缀和顺序，不得先读状态再补 advisory lock。

六级全在 `CrossModuleLockCoordinator::lock_all` 内完成。调用方锁后重载同一键集并 seal，`InventoryPostingPort` 与 `InventoryVariancePort` 只验证 proof 和读取已锁快照，不得重新取得其中任一 advisory/row lock。阶段 8 自有的迁移期初用例也先注册真实 Inventory slice 并走相同 coordinator；阶段施工不需要尚未交付的 Finance/Procure/Invoice 空 owner。

余额行初始化与取锁固定为两段式，禁止用 no-op `DO UPDATE` 绕过全库 row_version 触发器。先按同一全局排序对所有缺失候选执行 `INSERT ... VALUES (零余额,row_version=1) ON CONFLICT (...) DO NOTHING`；再对完整维度集合按第 1 至 3 类顺序、类内键升序执行 `SELECT ... FOR UPDATE` 并回读权威行。冲突胜方插入、败方在第二段等候并读到同一行，不存在先查后插竞态。命令按稳定键在内存工作副本上演算逐段 after 快照，全部子事实落库后才对每个 distinct balance key 执行恰一次真正的业务 `UPDATE ... SET ..., row_version=row_version+1 WHERE row_version=? RETURNING *`；初始化不算业务变更，已有行不会因取锁空增版本，同一命令的两个价格段或两个批次共享金额键时也只增加一次版本。

补货策略配置、销售可用量守卫与阶段 7 自动采购需求扫描共享另一条组合锁序：先对所有 `(legal_entity_id, warehouse_id, material_id)` 按字典序排序并逐个取得 `pg_advisory_xact_lock(hashtextextended('sales-availability:'||legal_entity_id||':'||warehouse_id||':'||material_id,0))`，再读取或锁定 `replenishment_policies`，随后才读取销售未交付量、库存余额及采购未结需求；任何路径不得先锁业务行再补取该 advisory lock。A12 已有行用 `FOR UPDATE` 加 `row_version` 谓词，新行依赖唯一键串行化，唯一冲突后重读并按版本冲突返回，不做无界重试。这样策略阈值更新与同一时点扫描只能落成某一合法串行序。

`lock_timeout` 取读写池的 3 秒（基线第 10.3 节），超时按 `INFRASTRUCTURE` 分类返回 503 并可重试。

#### 6.3 幂等

三层。第一层是平台的 `Idempotency-Key`，作用域为法人、用户、端点、键值四元组，由来源模块的写端点承担。第二层是 `ux_stock_movements_le_src_doc` 唯一约束，同一来源单据只能有一个 movement。第三层是端口内唯一版本化摘要 `InventoryPostingCanonicalV1`：JCS 对象只含法人 id，来源 `doc_type/doc_id/doc_no`，方向/原因，期间四项，规范化安全标签，以及每条行的 `posting_line_key`、来源行 id/no、仓库 id、物料 id、批次、数量、显式价格分支/单价或原交付成本 allocations；价差行另含 matched quantity 与 total variance。它明确排除 `WarehousePostingRef.is_active` 与 `MaterialPostingRef.is_active/is_stock_item/batch_managed/serial_managed` 等可变 MDM 快照布尔值。标签按规范字符串升序去重；lines 按 posting_line_key 升序且键不得重复；每行 serial_nos 按规范值升序去重，allocations 按 `(source_line_id,quantity,amount)` 升序并拒绝重复来源分配；UUID、日期与 Decimal 使用唯一规范字符串。对该 JCS 取 SHA-256 写入 `stock_movements.request_hash`。

proof 后先按来源查询 movement，再做任何当前 MDM 可用性或策略校验：命中且 V1 摘要相同，必须从 movement、两账与 variance_splits 重建并返回与首次逐字段相同的 `InboundPostingResult`、`OutboundPostingResult` 或 `VarianceSplitResult`，不再写任何行，也不得再次登记 pending event；摘要不同返回 `INVENTORY.STOCK_MOVEMENT.DUPLICATE_SOURCE_DOCUMENT`。因此首次过账后仓库/物料停用、批次或序列号管理标志改变、标签输入原序不同、行/serial/allocation 扫描顺序不同，都仍精确重放；只有权威 id、来源、期间、标签集合、数量、价格或分配关系改变才构成内容冲突。`find_movement_by_source` 只供诊断和调用方关联查询，业务重放不要求先捕获唯一冲突再二次调用。两个不同的幂等键并发提交同一来源文档也只产生一次库存事实与一个 Outbox 事件。

#### 6.4 与 Outbox 的关系

本阶段在库存过账端口首次写完 movement、数量账与金额账后，把唯一事件 `inventory.stock_movement.posted.v1` 登记到调用方事务的待刷新集合；此时不执行 Outbox SQL。信封按基线第 6.1 节，`aggregate_type` 取 `inventory.stock_movements`，`posting_date` 取 `business_date`，`accounting_period_id` 取本次过账的期间，`security_level` 与 `data_scope_tags` 从 movement 行取，缺失即拒绝登记。payload 固定为 `stock_movement_id,source_document_type,source_document_id,direction,reason,business_date,accounting_period_id,lines`，lines 按稳定键升序且使用 direction-tagged union：`IN` 行从 qty/value entry 取 `batch_no`、`quantity=+业务数量`、`amount=+inbound_amount` 与真实 `pricing_branch`；`OUT` 行取 `batch_no`、`quantity=-业务数量`、`amount=-outbound_amount` 与真实 `pricing_branch`；`VALUE_ADJUST` 行从 `variance_splits` 取 `batch_no='-'`、`quantity=0`、`amount=on_hand_variance_amount`、`pricing_branch=VARIANCE_ON_HAND`，已出库部分不属于库存金额事件，issued-only 与全零行的 amount 均为 0。三种 line 都固定含 `posting_line_key,source_document_line_id,source_document_line_no,warehouse_id,material_id,batch_no,quantity,amount,pricing_branch`，不携带随后才由来源用例生成的 `voucher_id`。调用方完成凭证、来源单据与投影并执行幂等 `finish` 后才刷新待写项，随后才允许同事务通知命令与审计终结批；任一后续写入失败全部回滚。payload 快照测试必须覆盖 IN、OUT、正/负 VALUE_ADJUST、issued-only 与全零，并断言相同来源摘要重放不会新增 pending event 或 Outbox。

首版不以该事件触发采购建议；阶段 7 已冻结的每 60 分钟 `SalesAwareReplenishmentPolicyQuery` 扫描是采购建议的唯一触发路径，避免扫描与事件双路重复建单。该事件只供 reporting 派生消费，幂等由 `platform_msg.inbox_consumptions(consumer,event_id)` 保证。只影响金额账的价差行已经由 `PostingPort` 在同一事务调用 `CostCaptureService` 捕获到 `costing.cost_entries`；B-09 的 `inventory.stock_movement.value_adjusted.v1` 与 `costing.stock_value_adjust` 异步补记通道整体撤销，不得实现。

本阶段的事件不承载分录、不承载凭证生成，因此事件投递失败进入死信不会破坏账务一致性，只会延迟报表刷新；采购建议只由上一段的周期扫描触发，不消费本事件。
本阶段不向 `ledger.posting_trigger_event_types` 登记任何行（裁定 A-21），库存事件不独立产生凭证。凭证一律由产生该库存流水的来源业务事件在同一事务内经 `ep_contract_ledger::PostingPort::post` 生成，其存货腿金额来自本阶段过账端口的返回值。因此关账受理前提二的待过账积压统计不会把本阶段的这一事件计入。

#### 6.5 失败重试与补偿

序列化失败 40001 与死锁 40P01 由数据访问层统一重试 3 次，退避 50、150、450 毫秒（基线第 8.4 节）。库存过账在事务提交前不产生任何外部可见副作用，因此可安全重试。

本阶段不提供补偿动作。库存流水只追加不可修改不可删除（PRD 第 5.5.8 节），登记错误的纠正方式只有由来源业务事件登记对应的反向事件。库存模块不提供独立冲正入口，这是硬边界，不因任何运维诉求开放。

#### 6.6 六组必测并发场景中本阶段承担的两组

基线第 8.4 节列出六组必测并发场景，本阶段直接承担第二组同一物料的并发出库与移动加权平均单价重算，并参与第三组同一采购订单的并发发票匹配与暂估回冲（本阶段侧为并发价差拆分对未覆盖数量的争用）与第五组关账受理与在途写事务的交叠（本阶段侧为在途库存过账落入待关闭期间时的期间归属一致性）。

### 7. 配置项

三项新增配置，全部在 `EP__INVENTORY__` 前缀下，结构体开启 `deny_unknown_fields`。

| 键名 | 类型 | 默认值 | 生效方式 | 说明 |
|---|---|---|---|---|
| EP__INVENTORY__POSTING__MAX_LINES | u32 | 200 | 进程启动时加载，变更需重启 | 单次过账的明细行上限，与基线第 5.1 节批量操作上限 200 对齐；core-server 与 job-worker 各自读取；该上限属 PRD 附录乙未决事项 U-A-10 的冻结取值，见第 11.2 节 |
| EP__INVENTORY__POSTING__MAX_SERIALS_PER_LINE | u32 | 1000 | 同上 | 单行序列号条数上限，防止单条明细行的序列号数组撑爆事务预算；该上限属 PRD 附录乙未决事项 U-A-10 的冻结取值，见第 11.2 节 |
| EP__INVENTORY__RECON__BATCH_SIZE | u32 | 2000 | 同上 | 对账检查的分批规模，单位为仓库与物料的组合数；该取值按规格第 10.2 章由附录 A.4 认证期实测冻结，本处默认值只是认证前的初值 |

不新增允许负结存的配置项。理由是该口径属未决事项 U-G-02，把未决业务口径落成运行期可变参数会制造一个永远无人负责取值的开关；本阶段硬编码为阻断，切换代价见第 11 节。

不新增导出行数、分页、默认筛选期间三类配置，它们由基线第 11.5 节全局固定。

启动自检不新增项，本阶段也不把任何判读业务数据行的判定挂到启动路径上。基线第 7.3 节的 `rls-enabled-and-forced` 一项断言全部带法人列的表均已 ENABLE 且 FORCE 行级安全，自动覆盖本阶段新增的十张表；该项与 `runtime-role-privileges-bounded` 只读 `pg_class` 与 `pg_roles`，判读的是结构与角色而不是业务行，二者留在阻断级。与本阶段有关的另外两项按降级口径处理：`reporting-dataset-signature-matched` 不一致时关闭该数据集对应的报表入口、登记降级窗口并告警，不拒绝启动；`master-data-usage-probes-registered` 已按第 4.9 节下沉为模块启用动作的前置校验，不在启动路径上判定。本阶段九张库存数量金额账相关表的结存非负、两账同源与勾稽差额一律由第 4.9 节的两个 `ReconCheck` 在 job-worker 内周期执行并生成差异事项；第十张补货策略表只受结构约束、RLS、授权、审计与第 8 节配置用例约束，不进入财务勾稽。任何阶段不得把业务行判定改写成启动时的闸门：这台服务器没有备节点，把业务数据判定放进启动路径等于让一条错误数据停掉八个进程。自检项一律按注册名标识，不用序号（裁定 C-25）。

### 8. 测试计划

#### 8.1 单元测试

位于 `ep-domain-inventory` 内，`#[cfg(test)]`，不触库不触网不取真实时间，时间经 `FixedClock` 注入。

计价与取价分支覆盖清单，逐条给出被测分支与断言要点。

| 编号 | 被测分支 | 断言要点 |
|---|---|---|
| U-01 | 入库 Explicit 取价与组合守卫 | 三种 branch 的非负金额等于 round(单价乘数量, 2)，单价重算为金额除数量 round 到 6 位；PurchaseReceipt 只接受暂估/超量两项、MigrationOpening 只接受期初项；三种 branch 的负 unit_price 与其余 reason×branch 均零写入拒绝 |
| U-01H | 历史库存专用入口与封闭 tuple | `post_migration_history` 对 IN、OUT、VALUE_ADJUST 各一条合法连续历史通过；固定 reason/source type/module/pricing branch 与 record id/batch no，普通 InventoryPostingPort、Excel、插件无法构造。九个 movement tuple 之外任一组合、非 MIGRATION_HISTORY pricing branch、record id 重复、日期/record_seq 逆序、after 数量/金额/均价差最小单位均整事务拒绝 |
| U-02 | 入库 ReturnAtOriginalDeliveryCost 在有正值结存时仍按原交付成本 | 当前 moving_avg 与原交付成本不同，金额仍恰等于 allocations 合计，分支为 ORIGINAL_DELIVERY_PRICE |
| U-03 | 入库 ReturnAtOriginalDeliveryCost 在零结存时按原交付成本 | 一行多 allocation 直接合计冻结 amount，不从六位单价反算，最后 `applied_unit_price=round6(amount/quantity)` |
| U-04 | 原交付成本为零 | 数量为正、allocation amount 与入库 amount 均为零，仍写数量/金额一一对应事实并重算均价 |
| U-05 | allocations 为空、来源重复、负金额或数量合计不等于行数量 | 返回 INVALID_PAYLOAD 或 ORIGINAL_PRICE_ALLOCATION_MISMATCH，零写入 |
| U-06 | 出库 MovingAverage 非出清 | 金额等于 round(单价乘数量, 2)，余额与单价按第 4.4 节更新 |
| U-07 | 出库 MovingAverage 出清归零 | 金额等于当前金额余额全额，分支记为 MOVING_AVERAGE_CLEARING，余额与单价均归零 |
| U-08 | 采购退货当前账面价值与 GRNI 原额分离 | 部分退货库存金额按当前移动平均价，全部退清按余额全额；原暂估只形成 `grni_consumed_amount`，差额精确等于 `return_carrying_difference_amount` |
| U-09 | 出库结存不足 | 返回 INSUFFICIENT_BALANCE，余额不变 |
| U-10 | 价差拆分全在库 | on_hand 等于 matched，issued 为 0，金额账加全额差额 |
| U-11 | 价差拆分全已出库 | on_hand 为 0，issued 等于 matched，金额账不变 |
| U-12 | 价差拆分部分在库与负价差穿零 | 正差额按比例拆分且尾差落 issued；负差额的 raw 在库额若会使金额余额小于零，则在库额截到 `-value_amount`、余数归 issued，处理后余额与均价均为零，两部分之和仍恒等于总差额 |
| U-13 | 价差拆分时未覆盖为 0 | on_hand 为 0，不写 value_entry，仍写 variance_splits |
| U-14 | 未覆盖数量 clamp | 出库数量大于未覆盖数量时未覆盖归 0 而非负 |
| U-15 | 空批次归集 | 未启用批次管理的物料一律以 `'-'` 归集，与显式传入 `'-'` 等价 |
| U-16 | 序列号状态机六条迁移 | 逐条断言目标状态与错误码 |
| U-17 | 舍入边界 | 中值远离零策略，覆盖 0.005、0.015、负值中值三组 |
| U-18 | 单价为 6 位小数除不尽 | 尾差留在金额余额，不产生调整分录 |
| U-19 | Posting DTO 集合与稳定键 | 规范段键排序、同一来源行多段、重复键、标签降密、跨法人快照、输入输出键集合及 variance 全零/部分非零结果形状逐项断言 |

领域属性测试（proptest），对应基线第 8.1 节要求的五组不变量中本阶段承担的三组，各生成不少于 1000 组随机操作序列。

- P-01 库存守恒：随机生成入库与出库序列，断言任意时刻结存数量等于流水代数和且非负，任意时刻数量账与金额账的 `quantity` 一致。
- P-02 移动加权平均单价重算：断言结存大于 0 时 `|金额余额 - round(结存数量 × 单价, 2)| ≤ round(结存数量 × 5e-7, 2) + 0.01`；断言移动平均路径下结存归零时金额余额为 0。
- P-03 价差拆分：随机生成匹配数量、总差额、现有金额余额与未覆盖数量，断言 `on_hand + issued = matched`、两部分差额之和恒等于总差额、处理后 `value_amount >= 0`、`0 ≤ uncovered ≤ 结存`、多次匹配的 `on_hand` 合计不超过初始未覆盖数量。

借贷平衡与核销守恒两组属性测试不由本阶段承担，归总账阶段与财务阶段。

`tests/trybuild/stage8_contracts/serial_state_view_ok.rs` 必须直接构造第 6.2 节唯一 `SerialStateView`，逐字段恰含 `serial_state_id,serial_no,material_id,warehouse_id,batch_no,status` 六项；漏字段、重复 `material_id` 或消费方传入第二个物料字段均编译失败。该代码块同时作为 doctest 编译，Stage12 consumer 正例只构造一次 `material_id`。

#### 8.2 集成测试

位于 `crates/application/inventory/tests/`，使用真实 PostgreSQL 16，每个用例独占 `ep_test_<nanoid>` 库，用例结束删库，禁止内存库或 mock。测试数据一律经 `ep-testkit` 构造器与 `InventoryPostingDriver` 生成，禁止手写 INSERT。

| 编号 | 场景 | 对应判据 |
|---|---|---|
| I-01 | 收货暂估入库后两账同源、未覆盖数量增加 | 规格第 17.2 章必测分支一前半、第 17.3 章存货金额账与数量账一致 |
| I-02 | 发票登记价差拆分后两账仍同源一致，单价按调整后金额除结存重算 | 必测分支一后半 |
| I-03 | 一张发票跨多次收货：逐次回冲、未覆盖数量不重复占用 | 必测分支十一第一至三句 |
| I-04 | 两张发票先后匹配同一法人同一仓库同一物料：第二张不含已被第一张覆盖的数量，两张合计不超过实际在库 | 必测分支十一末两句 |
| I-05 | 发票数量少于收货数量：未匹配部分的暂估留存，库存两账仍同源 | 必测分支十 |
| I-06 | 未收票采购退货按原收货暂估金额消费 GRNI，但库存按锁后当前账面价值出库；两者差额进入主营业务成本，退货后 GRNI、库存数量、库存金额与总账逐项勾稽 | 必测分支十二 |
| I-07 | 同一收货行同时含暂估余量与两条不同发票价的超量反向匹配：三个稳定段只产生一条 PURCHASE_RECEIPT movement，逐段单价/数量/序列号无重无漏，只有暂估段生成 GRNI，匹配段各有 settlement 且不产生价差记录 | 必测分支十三路径一的库存侧与文档级幂等根 |
| I-08 | 销售退货一行关联多张交付确认单：逐原交付根按锁后累计差额传入精确 quantity/amount，库存入库金额等于 allocations 合计并与成本反向 capture 逐根同额 | 必测分支十四 |
| I-09 | 出清归零：全部交付出库后金额余额与单价同时归零，随后销售退货按原交付实际成本入库并正确重算新移动平均价 | 第 4.6 节统一规则的正向验证 |
| I-10 | 直运交付确认与直运销售退货不产生任何库存流水 | 必测分支七与十五的库存侧 |
| I-11 | 负结存阻断：出库超结存时返回 INSUFFICIENT_BALANCE，余额与流水均无变化 | PRD 第 5.6.4 节提交时校验 |
| I-12 | 批次维度：未启用批次的物料按 `'-'` 单条归集；启用批次的物料分批出库互不串批 | 规格第 17.3 章按仓库物料批次逐项核对 |
| I-13 | 序列号全链路：收货入库、交付出库、销售退货再入库、再次发出，追溯链完整 | PRD 第 5.4.2 节与 U-G-04 冻结取值 |
| I-14 | 序列号异常四条：条数不等、行内重复、出库不在库、入库已在库 | PRD 第 5.9 节异常表 |
| I-15 | 收发存汇总平衡关系：期初加收入减发出等于期末，按仓库物料批次逐项成立；金额侧期初加收入减发出加调整等于期末 | PRD 第 5.7.2 节平衡关系 |
| I-16 | 期末库存价值表法人合计等于金额账合计 | PRD 第 5.7.3 节勾稽要求的库存侧 |
| I-17 | 顺延入账：同一业务事件的库存条目与传入的会计期间一致，按原始业务日期与按会计期间两条路径均可检索到该条流水，且结果标注实际落入的会计期间 | 规格第 5.2 章子账与凭证共用同一期间归属块、必测分支顺延项的库存侧 |
| I-18 | 幂等：IN/OUT、全零 variance、issued-only variance 分别首次提交后改变当前余额再以同一规范命令重放 2 次，逐字段返回首次固化的 after 快照；每组 movement、每输入 split 与 Outbox 各仅一份，重放不登记 pending event；只改数量、价格段、期间或标签时返回 DUPLICATE_SOURCE_DOCUMENT 且零新增 | PRD 第 5.9 节、`request_hash` 与精确重放协议 |
| I-19 | 迁移期初路径：该法人已有流水时拒绝 | 第 3.1 节预留分支 |
| I-20 | 结构与对账负向注入：数据库直写负数量余额或负金额余额、value entry 三个负 after、qty_after=0 但 value/avg after 非零、split 的负 coverage/after 或不满足 `uncovered_after=uncovered_before-on_hand_quantity`，均被具名 CHECK 拒绝且零提交；在合法非负行上注入数量/金额两账不一致、存货子账/总账不等，各自生成可追溯并阻断关账的差异事项 | 规格第 10.2 章发布验收与数据库非负闸门 |
| I-21 | 采购退货全数出清：库存当前账面金额 120、GRNI 原额 100 时，库存金额余额归零，物理凭证贷库存 120、借 GRNI 100、借主营业务成本 20；反向差额场景同样平衡，均不得产生零结存金额孤儿 | 第 4.6 节与 `PURCHASE_RETURN_INVENTORY` 三腿映射 |
| I-22 | 探针与引用计数器：某物料有库存流水时 `InventoryMaterialUsageProbe` 返回真、无流水时返回假；`InventoryReferenceCounter` 在该物料有非零结存时返回仓库物料批次组合数、结存全部归零后返回 0 | 裁定 A-13 与 A-15 |
| I-23 | 数据集视图与仅追加登记：`inventory.v_stock_value_entries` 含三列安全列且 `ep_analyst_ro` 可读、`ep_app_rw` 不可写；五张仅追加表在 `platform_core.append_only_registry` 的登记与触发器一致 | 裁定 A-18 与 B-02 |
| I-24 | 补货策略：同一法人仓库物料只能有一行；两阈值同空可停用、只空一个或 `target_stock < reorder_point` 的服务写入与数据库直写均拒绝；阶段 6 组合查询以同一可用量实现返回启用策略，且 `available_qty = on_hand - confirmed_or_released_unfulfilled_sales_qty` | F-51 U-F-02 与 U-G-01 的跨阶段承接 |
| I-25 | 即时外键、提交图约束、文档级 flush 与单次余额更新：对 IN/OUT 故障注入跳过 qty/value/serial 父事实、先写 last id，VALUE_ADJUST 注入跳过 split 与反序写入，均整笔回滚；在真实 PostgreSQL 事务中分别直写错误 line_count、qty/value/split/serial 的 movement 冗余值、同法人另一 movement 的真实明细 id、同一 movement 另一 posting line/来源行/仓库物料段的真实 id、qty 缺 value/一 qty 多 value、零 on-hand split 偷写 value、非零 on-hand split 缺 value或 amount/after 不等、三类 balance 与 serial_state 的 last pointer 错维度/错 serial/错方向。每个反例均须先让普通 FK 命中，再由具名 UNIQUE/CHECK 或延迟触发器在 `SET CONSTRAINTS ALL IMMEDIATE`/COMMIT 拒绝，断言 movement、事实、余额、Outbox 全部零部分提交。正例覆盖 IN、OUT、非零/零/issued-only VALUE_ADJUST 及同 movement 两计价段；同 stock key 两价格段、同 stock key 两批次分别断言逐段 after 快照按稳定键演算，但每个 distinct balance/state key 只 UPDATE 一次、row_version 只加一。另以已有/全新余额并发跑同维度命令，断言初始化只用 DO NOTHING、第二段锁读唯一行且无触发器异常 | 第 3.1 节库存图提交约束、第 4.3 至 4.5 节写序与第 6.2 节两段式初始化/文档级 flush |

法人越权测试集独立成 `tests/rls_matrix` 的 inventory 子目标，覆盖基线第 8.4 节的八类：读取、写入、更新、删除、聚合、排序、报表投影与错误信息泄漏。具体做法是以法人 A 的安全上下文对法人 B 的十张表逐表发起操作，断言读取返回空集、写入被 RLS 拒绝、聚合结果不含 B 的数据、按金额排序时 B 的记录不影响 A 的位次、报表端点 A9 与 A10 的合计不含 B、A11/A12 不泄漏或改写 B 的策略、错误消息不回显 B 的任何字段值。另覆盖内部对账系统安全上下文按法人逐轮遍历时每轮只写单一法人变量。该子目标属发布门禁项。

并发测试固定六组，每组不少于 200 次迭代。

- C-01 同一物料同一批次 20 线程并发出库，断言无负结存、无死锁、结存等于流水代数和、单价序列单调可解释。
- C-02 并发出库与并发价差拆分交叠，断言未覆盖数量不出现负值且不重复占用。
- C-03 同一来源单据的并发重复过账：相同摘要只产生一次流水且所有调用结果相同；不同摘要竞态恰一成功、其余稳定返回 DUPLICATE_SOURCE_DOCUMENT，余额与流水不混合。
- C-04 跨多物料的乱序批量过账，断言锁顺序生效、无死锁；故意以反序提交一组用例验证排序逻辑确实起作用。
- C-05 同一法人仓库物料上的 A12 策略修改、销售订单确认和阶段 7 补货扫描并发交叠，断言无死锁、同一组合只有一行策略、采购建议只能对应某一合法串行时点，旧 `row_version` 必须失败且不得静默覆盖。
- C-06 同一 SHIPPED serial 向两个不同仓并发入库时恰一成功、另一条锁后返回 ALREADY_IN_STOCK；同一 IN_STOCK serial 被两个出库命令并发消费时恰一成功、另一条返回 NOT_IN_STOCK。两组均断言数量账只有一份效果、serial_state 与唯一成功 movement 一致，且不存在行场景也由 advisory lock 串行化。

#### 8.3 端到端测试

本阶段既交付 inventory 模块的四端界面，也提供联调断言库，E2E 分两部分。本阶段四端查询界面消费 A1、A3 至 A11 共 10 个只读端点，A2 页面与路由由阶段 6 在真实销售需求提供者接线时同批启用；桌面管理界面另以具备写权限的身份调用 A12，A12 只配置补货策略，不直接修改库存数字。

本阶段自测部分：桌面端经 Playwright 与 tauri-driver 驱动 `clients/desktop/src/modules/inventory/`，覆盖本阶段 10 个查询页面在法人切换、字段级金额权限有无两种身份下的展示差异，并覆盖 A12 新建、修改、停用、旧版本冲突及无权限拒绝；阶段 6 同批补 A2 页面、组合查询与 `on_hand=100/reserved=30/available=70` 的端到端断言，同时验证同一数值进入阶段 7 补货建议。移动端按规格第 6.2 章能力矩阵库存台账与收发扫码四端取值均为完整，对 `clients/mobile/src/modules/inventory/` 执行 XCUITest 与 Espresso 各一个场景，覆盖扫码录入批次与序列号的即时校验反馈（调用 A6 与 A7）。

联调部分：规格第 8 章黄金业务闭环十四步中的第 5 步收货、第 8 步交付确认发货、第 11 步退货三步的库存侧断言，由阶段 9b 的 `testkit/scenarios/golden_loop_14_steps.rs` 执行，本阶段提供断言库 `ep-testkit::inventory_assertions`，含两账同源、守恒、勾稽三组断言函数，供该用例与后续阶段直接引用而不各写一套。

本阶段不设对外演示的里程碑。原里程碑 M5 已按总览第 5 节整项撤销，降级为本阶段的组件验收，判据写在第 9 节退出条件第 26 条。理由是本阶段按第一条硬边界不提供任何直接改动库存数字的 HTTP 写端点，库存写入侧只能经 `ep-testkit` 的 `InventoryPostingDriver` 调用第 5.1 节的过账端口驱动，这是组件级证据而不是可演示的产品路径，不应占用一个里程碑；A12 只是补货策略配置，不构成库存过账演示。M5 一并消失后，不允许用测试夹具直接写库来构造前置数据这条规则恢复为无例外。读出侧经本阶段 10 个只读端点与四端界面验证，其中 A6 与 A7 的扫码即时反馈由移动端真实调用；A2 在阶段 6 与销售需求提供者同批启用并补足首版终态第 11 个只读端点。收货、交付确认发货与价差拆分三条写入路径的真实调用方分别落在阶段 7、阶段 6 与阶段 10，各自在其自身阶段与本阶段的端口一次接实并同批验收，本阶段不登记任何顺延项，也不向总览的顺延清单写入任何条目。

#### 8.4 性能相关项

基准数据集按规格附录 A.3，由 `ep-datagen --scale=default --seed=<冻结值>` 产出：法人 2 个、物料 5000 条、库存流水 50 万条、会计期间 36 个。仓库数固定为 6 个（每法人 3 个）；这是已经批准的首版性能基准值，理由是仓库数直接决定余额行基数与报表分组数，不取值则性能结论不可复现，取值依据为规格第 2.2 章的目标客户规模。

| 度量项 | 通过线出处 | 目标 |
|---|---|---|
| 库存可用量查询（端点 A2） | 附录 A.1 常规交互清单 | P95 不超过 2 秒 |
| 入库过账（经端口，含两账与序列号） | 附录 A.1 普通交易提交清单 | P95 不超过 3 秒 |
| 出库过账 | 同上 | P95 不超过 3 秒 |
| 库存收发存汇总（端点 A9） | 附录 A.1 常用报表清单 | P95 不超过 10 秒 |

另需提交 EXPLAIN 证据：A1、A2、A4、A9、A10 五个端点在基准数据集上不得出现顺序扫描（基线第 3.10 节）。A9 与 A10 的期初与截止聚合走 `ix_stock_value_entries_le_seq_dim` 与 `ix_stock_qty_entries_le_dim_seq` 的范围扫描，需在证据中显示为 Index Only Scan 或 Index Scan。

样本数按附录 A.2 每场景不少于 200 次，只取负载稳定段，单次运行错误率超过 0.1% 该次无效。

#### 8.5 覆盖率门槛

按规格第 17.2 章与基线第 8.2 节，在 `codecov.toml` 中按路径表达。

| 路径 | 门槛 | 依据 |
|---|---|---|
| crates/domain/inventory/src/service/、src/rule/ | 行覆盖率不低于 85% | 强制不变量相关代码 |
| crates/application/inventory/src/usecase/、src/recon/ | 不低于 85% | 同上 |
| crates/domain/inventory/ 其余、crates/contract/inventory/ | 不低于 70% | 其余代码 |
| crates/application/inventory/ 其余、adapter-db-pg 的 inventory 仓储 | 不低于 70% | 其余代码 |
| 本阶段新增与修改代码整体 | 不低于 80% | 规格第 17.2 章 |

工具 cargo-llvm-cov，CI 上以 `--fail-under-lines` 强制。`#[ignore]` 必须带 issue 编号且不得跨阶段存活。

### 9. 退出条件

以下 26 条全部可客观判定，逐条达成才算本阶段完成。

1. 四个新增 crate 与两个改动 crate 在 `cargo build --workspace --all-features` 下零警告通过，`-D warnings` 生效。
2. 依赖方向自检脚本通过：`ep-domain-inventory` 不出现 sqlx、reqwest、tokio IO、std::fs、std::net、SystemTime::now、rand 符号；`ep-app-inventory` 的用例函数中不出现 reqwest 与文件写入符号；`ep-app-inventory` 不依赖任何其他模块的 application crate。
3. 文件规模纪律通过：单文件不超过 800 行，函数不超过 50 行，嵌套不超过 4 层。
4. `db/migrations/inventory/` 目录中既有 schema 前置迁移与本阶段 13 个新增迁移共 14 个文件在空库上顺序执行成功，`--check` 模式报告迁移历史版本与二进制期望版本一致；第 9 号迁移后九表库存图约束触发器全部存在且为 `DEFERRABLE INITIALLY DEFERRED`，每个本阶段新增迁移文件带 `-- rollback:` 段并经一次实际回退演练。
5. 十张表全部 `ENABLE` 且 `FORCE` 行级安全，`ep_app_rw` 不具备 BYPASSRLS 与 SUPERUSER，启动自检 `rls-enabled-and-forced` 与 `runtime-role-privileges-bounded` 两项通过。
6. 单元测试、三组领域属性测试、Stage8 contract trybuild 与 `SerialStateView` doctest 全绿。
7. 集成测试 I-01 至 I-25 全绿；I-25 的同 movement 错段、错误 line_count/冗余值和错误投影指针直 SQL 负例均在提交点失败且零部分写入。
8. `tests/rls_matrix` 的 inventory 子目标八类全绿。
9. 并发测试 C-01 至 C-06 全绿，无死锁记录，重试次数指标有值且在阈值内。
10. 覆盖率达到第 8.5 节的五档门槛。
11. 四个性能度量项达到通过线，五个端点的 EXPLAIN 证据中无顺序扫描，证据归档到 `docs/evidence/stage-8/`。
12. 两个 `ep_platform_recon::ReconCheck` 实现已在 job-worker 的 `ReconRegistry` 注册并可按法人与会计期间执行，注入三类差异后差异事项写入 `platform_core.recon_discrepancies` 且可追溯，注入清零后校验通过。
13. 第 5 节错误码表中的 21 个错误码在 `docs/error-codes.md` 与 `ep-foundation::error::codes` 两处一致，CI 的重复码校验通过。
14. 唯一事件 `inventory.stock_movement.posted.v1` 在 `docs/event-catalog.md` 登记，信封字段完整，缺少 `security_level` 或 `data_scope_tags` 时入队被拒绝的用例通过；三个已撤销 value-adjusted 名称不进入运行期注册表。
15. 本阶段新增指标固定为 0；旧句未给出任何具名指标，已由 F-54 撤销，不得以未命名配额驱动实现。库存只填充既有数据库与对账指标，ops-agent 的 9101 端点可抓取，标签基数纪律通过（不含 user_id、doc_no、trace_id）。
16. 数据字典中十张表逐列登记，含第 3.4 节三项新增命名决定与其缩写词表。
17. 第 4.6 节的出清归零规则已与基线第 3.5 节逐字同义，并由 U-07、U-08、I-09、I-21 证明部分出库、全数出清、GRNI 原额与库存当前账面价值差额四个分支；不存在待签署的偏离项。
18. 第 11.2 节列出的六项原未决事项已在本文件逐项冻结为首版终态，与 PRD 附录乙 U-G-01、U-G-02、U-G-03、U-G-04、U-G-06、U-A-10 对齐；其中 U-G-01 的 A2 与组合端口由阶段 6 同批启用，本阶段没有零值提供者或提前注册的路由；U-A-04 与 U-G-05 已由基线直接冻结，不另造不存在的外部清单文件。
19. inventory 模块在规格第 6.2 章能力矩阵中取值为完整或简化的能力域，其四端界面已实现并通过 Playwright 与 tauri-driver 的桌面用例、XCUITest 与 Espresso 的移动用例；取值为 VIEW_ONLY 的能力域只实现只读视图；取值为 NOT_APPLICABLE 的不实现入口。
20. `inventory.v_stock_value_entries` 已发布并授予 `ep_analyst_ro`，列签名已同步给阶段 11 且与 `reporting.dataset_fields` 的登记一致。
21. 本阶段全部路由的能力域码与动作类别常量已在 `crates/contract/inventory/src/capability.rs` 声明，`xtask configdoc` 通过。
22. `InventoryMaterialUsageProbe` 已实现并注入，启用 inventory 模块的动作在探针未注册时被拒绝、在注册后可完成，该判定不进启动自检，进程启动路径不因其失败而拒绝启动。
23. 本模块的 `InventoryReferenceCounter` 已实现并注册到 `MasterReferenceCounterRegistry`，本模块不承担任何 TradeHistoryProvider。
24. 已在 `crates/contract/inventory/src/port/subledger_balance.rs` 定义 `StockValueSubledgerBalancePort`，并由 `InventorySubledgerBalanceQuery`（位于 `crates/application/inventory/src/projection/subledger_balance.rs`）实现，trait 名、方法签名与实现类型名、位置按裁定 G-01 固定，按传入 `accounting_period_seq` 返回该法人截至目标期间的存货金额账累计，后续事件不得改变旧期间结果；本阶段不写任何注入行，注入由阶段 10 在 `apps/core-server/src/wiring/` 与 `apps/job-worker/src/wiring/` 两个目录下写入。
25. 五张仅追加表已按 `mode` 取 `APPEND_ONLY`、`mutable_columns` 取 `'{}'` 登记 `platform_core.append_only_registry`，`db/checks/append_only_consistency.sql` 经 `xtask sqlcheck` 执行通过。
26. 原里程碑 M5 降级而来的组件验收已完成：以 `InventoryPostingDriver` 各驱动一条收货暂估入库、一条交付确认出库与一条价差拆分，三条路径的两账同源、数量守恒与取价分支断言全绿；验收报告写明三者的真实调用方分别在阶段 7、阶段 6、阶段 10 接线，本阶段不登记任何顺延项、不作为对外演示节点；`apps/core-server/src/wiring/` 与 `apps/job-worker/src/wiring/` 两个目录下的全部文件中不出现任何以 Noop、Stub、Fake、Dummy 为前缀的实现类型注入行，该判据由阶段 1 随 xtask 交付的 archcheck 规则 unwired-absent 断言，出现即构建失败。

### 10. 与规格和 PRD 的对应

#### 10.1 规格条目

| 规格位置 | 本阶段实现的内容 |
|---|---|
| 第 5.2 章库存与 WMS 条目 | 仓库与库存台账的库存侧、收发存记录、可用量的结存输入与阶段 6 组合契约、批次与序列号标识、移动加权平均一种方法与单一成本层、出库按加权平均单价结转、两账同源同步、按仓库与物料的库存金额查询、期末库存价值表、销售退货为入库方向、采购退货为出库方向、数量账写入权归库存模块 |
| 第 5.2 章事件-分录表交付确认事件 | 按交付确认时点的移动加权平均单价从库存金额账结转的库存侧算法；直运不产生库存流水；交付确认单主体与其两张表归 sales 模块并由阶段 6 建立，本阶段不建单据表 |
| 第 5.2 章事件-分录表采购收货事件 | 按采购订单不含税单价暂估入库的库存侧写入；已被反向匹配的收货数量不走暂估 |
| 第 5.2 章事件-分录表采购发票事件 | 价差拆分中尚有库存部分调整存货金额并重算单价的库存侧写入；已出库部分金额的计算并返回 |
| 第 5.2 章事件-分录表销售退货与采购退货事件 | 两个方向的库存流水与两账更新 |
| 第 5.2 章价差拆分规则 | 未被价差覆盖在库数量的全部维护逻辑、尚有库存数量与已出库数量的判定、按比例拆分与尾差归属、跨发票跨期间不重复占用 |
| 第 5.2 章超量开票三条结清路径之路径一 | 反向匹配收货按已登记发票不含税单价同源写两账、不走暂估、不产生价差 |
| 第 5.2 章退货回冲的取价三分支 | 三个分支的判定与执行、回冲后单价一律按回冲后金额余额除结存数量重算、结存为零时单价归零、历史成本不重算 |
| 第 5.2 章子账与凭证共用同一期间归属块 | 库存两账条目携带会计期间字段与原始业务日期，顺延时随调用方一并顺延 |
| 第 6.2 章能力矩阵库存台账与收发扫码行 | 四端取值均为完整，本阶段提供扫码即时校验所需的两个端点 |
| 第 7.2 章数据所有权与不变量 | 库存两账唯一权威写入者为库存模块；库存流水只追加不覆盖 |
| 第 7.5 章文件、分析与归档 | 库存流水列入仅追加对象，进入审计 |
| 第 7.7 章法人行级隔离机制 | 十张表的 RLS 策略、内部对账系统安全上下文的逐法人遍历取数 |
| 第 7.9 章派生存储安全继承 | `inventory.stock_movement.posted.v1` 携带 security_level 与 data_scope_tags |
| 第 10.2 章主系统规则 | 库存数量守恒与存货项子账总账勾稽两项检查以 `ep_platform_recon::ReconCheck` 实现，在 job-worker 的 `ReconRegistry` 注册与执行、分批口径、未完成处置 |
| 第 12.2 章授权 | 库存金额、单价与价值表金额列的字段级权限与密级 30 |
| 第 15.1 章错误分类 | 21 个错误码的五类分类映射与四要素齐备 |
| 第 15.2 章可靠任务 | 负结存与勾稽差额进入死信与人工修复，不静默忽略 |
| 第 16 章与附录 A.1、A.2、A.3 | 四个度量项的通过线与基准数据集规模 |
| 第 17.2 章财务内核测试 | 十五类必测分支中的第一、三（库存侧）、七、十、十一、十二、十三、十四、十五共九类的库存侧断言 |
| 第 17.3 章强制不变量 | 库存数量守恒、存货金额账与数量账一致两项由本阶段实现并校验；子账与总账勾稽的存货项由本阶段提供子账侧取数 |

#### 10.2 PRD 节次

| PRD 位置 | 本阶段实现 |
|---|---|
| 第 5.1.1 节 | 本节覆盖的全部内容 |
| 第 5.1.2 节 | 六项取价一律指向规格，本阶段不复述不改写 |
| 第 5.1.3 节 | 库位、质检、预留、拣货与波次、调拨、盘点均不实现、不出现入口；无任何独立于业务事件的库存增减入口 |
| 第 5.2.1 至 5.2.2 节 | 仓库作为唯一存放地点维度、跨仓库不合并单价、仓库归属单一法人；仓库档案由阶段 5 的 `mdm.warehouses` 唯一承载，本阶段只经 `MasterDataLookup` 读取 |
| 第 5.2.3 节 | 仓库停用前置校验的库存侧，即实现阶段 5 定义的 `WarehouseDeactivationCheckPort::assert_no_stock`，实现类固定为 `InventoryWarehouseDeactivationCheck`，判定该仓库全部物料结存为零 |
| 第 5.3.1 至 5.3.3 节 | 数量账四维度、金额账三维度、批次不承载独立成本、按批次不可查金额 |
| 第 5.4.1 至 5.4.3 节 | 批次必填规则、出库批次候选、一行一批次、批次无状态；序列号非台账维度、条数校验、追溯链、扫码重复处理 |
| 第 5.5.1 节 | 出入库事件总表七行的库存侧全部实现 |
| 第 5.5.2 节 | 五个字段的通用校验与四步校验顺序 |
| 第 5.5.3 至 5.5.6 节 | 四类出入库操作的库存侧处理与输出 |
| 第 5.5.7 节 | 只影响金额账的调整记录，方向为无方向、数量为零、金额可正可负、可追溯到采购发票登记单；收发存汇总的数量列不含该记录、金额列包含 |
| 第 5.5.8 节 | 库存流水不可变性与八个携带字段 |
| 第 5.6.1 至 5.6.3 节 | 库存台账与流水查询由本阶段启用；A2 可用量契约由本阶段冻结、由阶段 6 以真实销售未交付量同批启用，终态结果列、公式与排序唯一 |
| 第 5.6.4 节 | 提交时校验、事后校验、修复路径 |
| 第 5.6.5 节 | 提供结存数量与可用量作为采购建议的判定输入 |
| 第 5.7.1 至 5.7.3 节 | 三张报表端点、勾稽要求的子账侧、跳转参数 |
| 第 5.8 节 | 法人隔离、字段级权限、唯一写入者、审计、四端取值、库存操作不属六类高风险操作 |
| 第 5.9 节 | 异常表九行逐行对应到错误码 |
| 第 5.10 节 | 四个度量项一律指向规格，本阶段不重取数值 |

### 11. 风险与预留

#### 11.1 已知技术风险

R1，出清归零规则被局部实现遗漏。该规则已经写入基线，不再存在批准风险；实现风险是普通交付路径采用全额出清、采购退货路径却仍按旧原暂估价留下金额孤儿。控制：两条路径共用第 4.4 节同一函数，`qty_after == 0` 必须优先于调用意图分支；U-07、U-08、I-09、I-21 以及恢复后强制不变量共同断言金额余额和单价均归零，任一遗漏直接失败。

R2，库存腿金额与凭证腿的同事务一致性。裁定 C-28 已定死全部凭证一律与业务事件同事务生成、Outbox 只承载派生、通知、检索与报表数据集，因此不存在总账侧异步生成凭证的分支，本条风险收窄为编排用例误用：调用方若在库存腿之后另开事务写凭证，规格第 10.2 章关账受理后建立快照的时点会读不到该凭证。缓解：第 6.1 节已把 `&mut dyn Tx` 的同事务要求写入端口签名，跨事务调用在类型上不可表达；本阶段的集成测试 I-17 以传入期间为准做一致性断言，联调时若出现跨事务写入，该断言会立即失败。

R3，热点物料的行锁串行化。20 并发下若集中在少数物料上出库，`stock_value_balances` 的单行锁会把并发退化为串行，威胁 3 秒的普通交易提交通过线。缓解：C-01 并发测试直接度量该场景的 P95；若不达标，可行的优化是把金额账余额的更新推迟到语句级并使用 `UPDATE ... RETURNING` 的单语句原子更新，减少锁持有时间，但不改变一次一行的语义。不采用分片计数器，理由是移动加权平均单价必须读到全局一致的金额余额。

R4，收发存汇总与期末库存价值表的期初聚合随期间数增长。36 个期间末尾的期初聚合需要扫过接近全量的流水。缓解：`accounting_period_seq` 的范围扫描索引已就位；若认证期实测逼近 10 秒，扩展点是按期间物化一份余额快照，见第 11.3 节。

R5，极端负采购价差的分流实现若遗漏账面价值下限，可能制造负存货或让总账价差不守恒。缓解：数据库 `value_amount >= 0` CHECK 作最后闸门，第 4.5 节把穿零部分确定性转入 issued variance；U-12、P-03、I-02 与 I-20 同时断言存货非负、两部分价差守恒及总账勾稽，不再把负存货当成可接受展示口径。

R6，序列号唯一性范围与设备档案的冲突（U-G-07、U-J-03）。本阶段按法人内唯一实现，若阶段 12 的设备档案采用同一产品下唯一，同一序列号可能在两处各存一份。缓解：本阶段的 `serial_states` 是库存侧的唯一真相，阶段 12 的设备档案应引用而非另建，该约束作为阶段 12 的输入前提；本阶段不提供任何跨模块的序列号写入端口。

#### 11.2 未决事项的冻结取值与切换代价

| 编号 | 冻结取值 | 是否阻塞本阶段 | 切换代价 |
|---|---|---|---|
| U-G-01 可用量构成 | `available = on_hand - 已确认或已下达且尚未交付的销售订单剩余量`；不建持久化预留表。阶段 8 交付 `StockOnHandQueryPort`，阶段 6 同批交付 `ConfirmedOpenSalesDemandQuery`、`AvailabilityQueryPort` 组合实现与 A2 路由；订单确认/下达、取消/关闭与交付均在同一锁序下重算 | 不阻塞，已冻结 | 首版不得切换为 `reserved_quantity=0` 或只看结存；改变公式属于产品语义变更，须另立裁定并同步销售确认守卫、A2、补货建议与并发测试 |
| U-G-02 是否允许负结存 | 一律硬阻断，不提供配置 | 不阻塞 | 若改为可配置，需在物料或仓库档案上加一个开关字段（归 mdm）、在出库路径加一个分支、去掉 `ck_stock_qty_balances_non_negative`、新增 4 个测试用例；去掉数据库 CHECK 属收紧变更的逆操作，可在线执行 |
| U-G-03 批次号与序列号的长度字符集 | 长度上限 64、字符集 `[A-Za-z0-9._-]`、批次号手工录入、唯一性范围为法人加仓库加物料 | 不阻塞 | 放宽长度属基线第 7.4 章在线变更范围，改 CHECK 即可；收紧字符集需回填校验 |
| U-G-04 序列号状态语义 | 两状态 IN_STOCK 与 SHIPPED，退货入库后可再次发出且允许换仓入库 | 不阻塞 | 若增加已退回等第三状态，需扩 CHECK 取值与状态机守卫，约 1 个文件加 6 个用例 |
| U-G-06 关账是否固化快照 | 不固化，已关闭期间按实时聚合取数 | 不阻塞 | 若改为固化，新增一张快照表与一个 job-worker 任务，A9 与 A10 的响应结构不变，扩展点见第 11.3 节 |
| U-A-10 单据明细行数与序列号条数上限 | 单次过账明细行上限取 200，见第 7 节 `EP__INVENTORY__POSTING__MAX_LINES`，与基线第 5.1 节批量操作上限一致；单行序列号条数上限取 1000，见第 7 节 `EP__INVENTORY__POSTING__MAX_SERIALS_PER_LINE`，该值在规格与基线中均无出处，是本阶段按基线第 10.3 节事务预算估算的技术侧冻结取值；单次连续扫码条数不单独设限，连续扫码为逐次调用端点 A6 与 A7，单次批量校验的条数由第 5 节 A7 的 `filter[serial_no]=in:` 上限 200 个约束 | 不阻塞 | 改两个配置项的默认值并重启即可，不改表结构、不改 API 契约、不改端口签名；上调任一取值须重跑第 8.2 节并发测试 C-01 与第 8.4 节普通交易提交的 P95 度量项，不达标时按第 11.1 节 R3 收窄取值而不拆事务；`EP__INVENTORY__POSTING__MAX_LINES` 上调超过 200 会突破基线第 5.1 节批量操作上限，须同步提出基线修订，不得只在本阶段偏离 |

U-A-04 数量单价金额的小数位与舍入、U-G-05 空批次标识两项已由基线第 3.5 节与第 11.4 节定死，本阶段直接照用，不再另行取值。

#### 11.3 为后续阶段预留的扩展点

E1，期间余额快照。A9 与 A10 的取数集中在 `ep-app-inventory/src/projection/period_aggregation.rs` 一个文件内，通过一个 `PeriodAggregationSource` trait 取数。当前实现为实时聚合，若 U-G-06 决策为固化快照或 R4 的性能不达标，只需新增一个快照实现并在装配处替换，API 契约与响应结构不变。

E2，可用量计算的唯一组合点。阶段 6 的 `AvailabilityQueryPort` 实现只组合本阶段 `StockOnHandQueryPort` 与销售侧 `ConfirmedOpenSalesDemandQuery`，A2、销售确认守卫与补货建议共用；不得在库存表另建预留余额，也不得在任一消费者自行重算第二套可用量。

E3，多计价方法。当前 `PricingBranch` 与计价服务是按移动加权平均单一方法写死的，但两账分离、流水仅追加、余额独立三项结构本身与计价方法无关。若后续引入先进先出，扩展点是在 `stock_value_balances` 之外新增成本层表并把计价服务改为 trait，`stock_qty_entries` 与 `stock_movements` 不需要改动。本阶段不预埋任何多方法的空壳代码。

E4，迁移库存通道分成两个不可互换来源。库存期初只用 `MIGRATION_STOCK_ADJUSTMENT/MIGRATION_OPENING/MIGRATION_OPENING`，按裁定 A-24 仍是该法人无任何流水时的唯一落点，总账侧由阶段 9a 期初余额批次承担。完整历史只用 Stage 14 的 `MIGRATION_STOCK_HISTORY/MIGRATION_HISTORY/MIGRATION_HISTORY`，允许 IN/OUT/VALUE_ADJUST 三 direction，来源 id 固定 data_migration_records.id、来源号固定 batch_no，并经 crate-private `post_migration_history` 按稳定全序和 after 连续性回放；不能拿期初来源伪装历史，也不能由普通过账入口调用。Stage 14 的 092600 在启用该 writer 前同批替换 movement/pricing CHECK 与 Rust enum，并交付九 tuple、三 direction、direct-SQL 绕过与 rollback catalog 证据；应收应付预收预付与资金账户期初仍归阶段 10。

E5，成本归集查询的取数接口。规格第 5.2 章成本归集与销货成本结转条目的存货类成本来源是交付确认时从库存金额账结转的销货成本，成本阶段需要按交付确认单行回查结转金额与单价。本阶段的 `ep_contract_inventory::InventoryPricingLookupPort::priced_segments_by_source_line` 端口与 `stock_value_entries(legal_entity_id,source_doc_line_id,posting_line_key)` 索引即为该接口，成本阶段直接引用。来源单据行的口径按裁定 A-09 固定为 `sales.delivery_confirmation_lines`，查询显式传 `SourceDocType::DELIVERY_CONFIRMATION`；首版每条交付行必须恰返回一个 OUT 段，多段或空集合按不变量故障，不任取第一行。采购收货对账显式传 `SourceDocType::PURCHASE_RECEIPT` 并允许返回多个 IN 段。

E6，断言库复用。`ep-testkit::inventory_assertions` 提供两账同源、数量守恒、存货勾稽三组断言函数，闭环联调阶段与恢复演练（规格附录 A.5、A.6 要求恢复后执行第 17.3 章全部强制不变量校验）直接引用同一实现，避免恢复验收另写一套判据。
