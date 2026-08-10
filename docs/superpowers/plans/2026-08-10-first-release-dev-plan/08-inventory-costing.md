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
| D1 | ep-contract-inventory crate | 编译通过，含七个对外 trait 与全部命令、结果 DTO，无任何 IO 依赖；其中第七个 `StockValueOutboundPort` 按裁定 F-05 由阶段 11 与其实现类型同批追加到本 crate，本阶段结束时的实交付数为六个 |
| D2 | ep-domain-inventory crate | 编译通过，含四个聚合、计价与取价领域服务、五组不变量断言，`cargo test` 全绿，不含 sqlx 符号 |
| D3 | ep-app-inventory crate | 编译通过，含三个过账用例、一个停用校验用例、七个查询投影、两个 `ReconCheck` 实现、`InventoryMaterialUsageProbe`、`InventoryReferenceCounter` 与 `InventorySubledgerBalanceQuery` |
| D4 | ep-adapter-db-pg 中的 inventory 仓储实现 | 九张表的仓储与查询实现，只访问 inventory schema |
| D5 | db/migrations/inventory 下 13 个迁移文件 | `--check` 模式下迁移历史版本一致，全部表 RLS 已 ENABLE 且 FORCE |
| D6 | core-server 上的 10 个只读 HTTP 端点 | 可用 curl 打通，返回基线第 5.2 节封套 |
| D7 | 两个 `ep_platform_recon::ReconCheck` 实现在 job-worker 的 `ReconRegistry` 注册并可运行 | 注入差异后写入 `platform_core.recon_discrepancies`，可追溯 |
| D8 | ep-testkit 中的 `InventoryPostingDriver` 与七个构造器 | 集成测试可在无采购、销售、发票模块的情况下驱动全部库存路径 |
| D9 | ep-datagen 中的库存流水生成器 | `--scale=default` 产出 50 万条库存流水、36 个会计期间的基准数据集 |
| D10 | docs 三处登记 | error-codes.md 新增 21 条、event-catalog.md 新增 2 条、data-dictionary 新增 9 张表 |
| D11 | 性能证据 | 四个附录 A.1 度量项的 EXPLAIN 输出与 P95 实测报告 |
| D12 | inventory 模块四端界面 | `clients/desktop/src/modules/inventory/` 与 `clients/mobile/src/modules/inventory/` 下的模块目录，桌面用例经 Playwright 与 tauri-driver、移动用例经 XCUITest 与 Espresso 通过 |
| D13 | 受治理数据集视图 `inventory.v_stock_value_entries` | 视图存在且含 legal_entity_id、security_level、data_scope_tags 三列，已授予 `ep_analyst_ro`，列签名与阶段 11 的 `reporting.dataset_fields` 登记一致 |

本阶段交付 inventory 模块的四端界面（裁定 A-23），位置固定为 `clients/desktop/src/modules/inventory/` 与 `clients/mobile/src/modules/inventory/`。规格第 6.2 章能力矩阵第 597 行库存台账与收发扫码一行四端取值均为完整，因此四端均实现完整视图。界面只消费第 5 节的十个只读端点与字段级权限投影，不新增任何写端点，第一条硬边界不因界面下沉而放宽。

### 2. crate 与进程归属

新增四个 crate，改动两个既有 crate 与两个客户端工程，不新增任何进程。

| crate | 类型 | 装配进程 | 职责 |
|---|---|---|---|
| ep-contract-inventory | 新增 | core-server、job-worker | 对外 trait 与 DTO，只依赖 ep-foundation |
| ep-domain-inventory | 新增 | core-server、job-worker | 聚合、值对象、计价服务、取价判定、不变量断言、仓储端口 trait |
| ep-app-inventory | 新增 | core-server、job-worker | 过账用例、查询投影、授权调用、审计与 Outbox 写入、两个 `ReconCheck` 实现、两个 mdm 侧探针实现与子账侧余额端口实现 `InventorySubledgerBalanceQuery` |
| ep-adapter-db-pg | 改动 | core-server、job-worker | 新增 `repo/inventory/` 目录下 6 个仓储文件与 1 个查询文件 |
| apps/core-server | 改动 | core-server | `wiring.rs` 注入 inventory 实现，路由注册 10 个端点 |
| apps/job-worker | 改动 | job-worker | `wiring.rs` 向 `ReconRegistry` 注册 2 个对账检查、向 `MasterReferenceCounterRegistry` 注册 `InventoryReferenceCounter`、注入 `InventoryMaterialUsageProbe`，本阶段不注册任何事件消费者 |
| clients/desktop | 改动 | 桌面客户端 | `src/modules/inventory/` 下的库存台账、库存流水、两张报表与扫码校验页面 |
| clients/mobile | 改动 | 移动客户端 | `src/modules/inventory/` 下的扫码录入与台账查询页面 |

依赖方向按基线第 1.3 节，逐条自查如下。ep-domain-inventory 只依赖 ep-foundation 与 ep-contract-inventory。ep-app-inventory 依赖 ep-foundation、ep-platform-authz、ep-platform-audit、ep-platform-outbox、ep-platform-obs、ep-platform-recon、ep-domain-inventory、ep-contract-inventory、ep-contract-mdm、ep-contract-ledger。其中 ep-contract-ledger 用于第 4.9 节第三项两个 `ReconCheck` 里存货项子账与总账勾稽一项的总账侧取数，即 `ep_contract_ledger::TotalAccountBalanceProvider`；该 trait 按阶段 9 计划第 1 节的 9a 段交付清单属 9a 段交付，9a 排在本阶段之前，故本阶段命名它即可编译，不构成对后续阶段的前置引用。本段依赖枚举是本阶段结束时的快照，按裁定 F-05 通则甲不具跨阶段封闭效力，后续阶段可在基线第 1.3 节允许项内增边而不回改本段。ep-app-inventory 不依赖任何其他模块的 application crate。ep-contract-inventory 不依赖 ep-contract-mdm，物料与仓库属性以扁平化的入参结构体传入，避免契约层横向耦合。

跨模块调用方向明确为单向：procure、sales、invoice、finance 四个模块的 application crate 依赖 ep-contract-inventory 并在装配时注入本阶段的实现，其中 finance 侧是裁定 G-01 把子账余额端口移到被调方契约后新增的第四个调用方；本阶段不反向依赖它们。mdm 的仓库停用用例依赖 ep-contract-inventory 的停用校验 trait，同样在装配时注入。
本阶段在调整后的阶段顺序 1 → 2 → 3a → 4 → 3b-1 → T0 → 5 → 9a → 8 → 6 → 7 → 10 → 11 → 9b → 14 中排在阶段 9a 之后、阶段 6 之前，整体落在贯通线 T0 之后；阶段 3b-2 不在这条链上，按阶段 3 计划第 3.0 节判定四的下游拉动点排在 T0 之后，阶段 12 与阶段 11 并行，阶段 13 与阶段 9b 并行。T0 从阶段 5、6、9a、10、11 各取一个最小切片，把一条合同从建单走到管理层看到一个数，五个切片逐项展开共十二项：一个客户、一个产品、一份单审批节点的合同、一张销售订单、一张销项发票、`invoice.tax_rate_options` 的建表与种子及 `TaxRateOptionQuery`、一个打开的会计期间、一张凭证、一笔到款登记与核销、最小应收台账、一个银行账户建档与一张收入报表，其中会计期间由 `AccountingPeriodResolver::resolve` 第二步的零期间分支在首次过账的同一事务内建立。十二项中没有库存项，因此本阶段不向 T0 交付任何切片，也不因 T0 而提前开工。本阶段整体按加厚口径施工：开工时客户、产品、合同、订单、销项发票、税率字典、到款、凭证、会计期间与收入报表已在骨架上真实跑通，本阶段做的是在这条已贯通的骨架上加库存两账与取价这一层。因此 ep-platform-recon 的对账框架、ep-contract-ledger 的过账端口与 ep-contract-mdm 的探针 trait 在本阶段开工时均已存在，本阶段不为任何端口注入空实现，也不存在只登记对账语句不执行的过渡期。本阶段在跨模块调用中一律是被调方在先的一侧，按被调方与调用方同批交付的硬规则，调用方阶段 6、阶段 7、阶段 10 各自接线并在其自身阶段完成该调用的验收，本阶段不为任何调用方预留占位实现，也不登记任何顺延项。反过来，交付确认单、采购收货单与采购发票分别由阶段 6、阶段 7、阶段 10 在其自身 schema 建立，本阶段只提供被它们调用的库存腿与价差拆分入口。本阶段实现但不拥有的三个 trait 见第 4.9 节，本阶段自有并自行定义的子账余额端口见第 5.1 节，注入位置为 `apps/core-server/src/wiring/` 与 `apps/job-worker/src/wiring/` 两个目录下的全部文件，不是单个 `wiring.rs` 文件。

进程归属：全部过账路径与查询路径在 core-server 内执行；对账检查与库存事件消费在 job-worker 内执行；portal-gateway 不接触库存数据，供应商门户不提供任何库存视图（PRD 第 4.9.1 节的能力边界内无库存项）。

### 3. 数据库变更

schema 固定为 `inventory`，属主角色 `ep_mod_inventory`，运行期读写走 `ep_app_rw`，对账走 job-worker 池的同一账号，只读分析走 `ep_analyst_ro`。九张新表，全部按基线第 4 节的公共列排列。以下表定义中未重复列出的公共列一律按基线第 4 节取值：`id uuid`、`legal_entity_id uuid`、`security_level smallint default 20`、`data_scope_tags text[] default '{}'`、`row_version bigint default 1`、`created_at`、`created_by`、`updated_at`、`updated_by`。仅追加表按基线第 4 节去掉 `row_version`、`updated_at`、`updated_by`，改带 `reverses_id uuid null`。
公共列 `created_by` 在由 job-worker 的对账执行器或库存期初导入通道写入时取 `ep_foundation::SYSTEM_PRINCIPAL_ID`，即裁定 A-02 冻结的保留取值 `00000000-0000-7000-8000-000000000001`，不得另写全零值或其他自选值。

#### 3.1 表定义

表 1，`inventory.stock_movements`，库存移动事件头，仅追加。一次过账写一条。

| 列 | 类型 | 可空 | 约束与说明 |
|---|---|---|---|
| id | uuid | 否 | pk_stock_movements |
| legal_entity_id | uuid | 否 | RLS 判据 |
| security_level | smallint | 否 | 默认 20 |
| data_scope_tags | text[] | 否 | 默认 '{}' |
| business_date | date | 否 | 原始业务日期，取该业务事件的记账日期 |
| accounting_period_id | uuid | 否 | 会计期间归属，由调用方传入 |
| accounting_period_seq | int | 否 | 该法人内会计期间的单调序号，随 accounting_period_id 一并传入，用于期间区间聚合 |
| direction | text | 否 | ck_stock_movements_direction，取值 `IN`、`OUT`、`VALUE_ADJUST` |
| reason | text | 否 | ck_stock_movements_reason，取值见下文八项 |
| source_doc_type | text | 否 | ck_stock_movements_source_doc_type，取值 `PURCHASE_RECEIPT`、`PURCHASE_RETURN`、`DELIVERY_CONFIRMATION`、`SALES_RETURN`、`PURCHASE_INVOICE`、`MIGRATION_STOCK_ADJUSTMENT` |
| source_doc_id | uuid | 否 | 跨模块逻辑引用，不建外键 |
| source_doc_no | text | 否 | ck 长度 1 至 64 |
| source_module | text | 否 | ck 取值 `procure`、`sales`、`invoice`、`migration` |
| line_count | int | 否 | ck 大于 0 且不超过配置上限，冗余用于查询与限额自检 |
| reverses_id | uuid | 是 | 首版恒为 NULL，按基线第 4 节仅追加表的列约定保留，库存流水的更正一律由来源业务事件登记反向事件承担，见第 6.5 节 |
| created_at / created_by | | 否 | |

`reason` 的八项取值：`PURCHASE_RECEIPT`、`PURCHASE_RECEIPT_OVERBILL_MATCHED`、`SALES_RETURN`、`DELIVERY_CONFIRMATION`、`PURCHASE_RETURN_INVOICED`、`PURCHASE_RETURN_UNINVOICED`、`PURCHASE_INVOICE_VARIANCE`、`MIGRATION_OPENING`。最后一项承载库存期初，按裁定 A-24 本阶段是库存期初导入的唯一落点，首版不设独立的数据迁移阶段；本阶段实现其入库路径但只允许在该法人尚无任何库存流水时执行，期初写入不生成凭证，其总账侧由阶段 9a 的期初余额批次承担。

索引：`pk_stock_movements`；`ix_stock_movements_legal_entity_id_created_at`（基线）；`ux_stock_movements_le_src_doc` 唯一，列为 `(legal_entity_id, source_doc_type, source_doc_id)`，这是本阶段的过账幂等根；`ix_stock_movements_le_period` 列为 `(legal_entity_id, accounting_period_seq, business_date)`；`ix_stock_movements_le_bizdate` 列为 `(legal_entity_id, business_date, id)`。

表 2，`inventory.stock_qty_entries`，库存数量流水，仅追加。基线第 3.2 节已登记该表名。

| 列 | 类型 | 可空 | 约束与说明 |
|---|---|---|---|
| id | uuid | 否 | pk |
| legal_entity_id | uuid | 否 | |
| security_level / data_scope_tags | | 否 | |
| movement_id | uuid | 否 | fk_stock_qty_entries_stock_movements，ON DELETE RESTRICT，同 schema 建真实外键 |
| line_no | int | 否 | ck 大于 0 |
| source_doc_line_id | uuid | 否 | 跨模块逻辑引用 |
| source_doc_line_no | int | 否 | |
| warehouse_id | uuid | 否 | 跨 schema 逻辑引用 mdm，不建外键 |
| material_id | uuid | 否 | 跨 schema 逻辑引用 mdm，不建外键 |
| batch_no | text | 否 | 默认 `'-'`，ck 长度 1 至 64 且字符集为 `[A-Za-z0-9._-]`，空批次固定取 `'-'`（基线第 11.4 节） |
| quantity | numeric(18,6) | 否 | ck 不等于 0，入库为正、出库为负 |
| qty_balance_after | numeric(18,6) | 否 | ck 大于等于 0 |
| direction | text | 否 | 冗余自 movement，ck 取值 `IN`、`OUT` |
| business_date | date | 否 | 冗余自 movement |
| accounting_period_id | uuid | 否 | 冗余自 movement |
| accounting_period_seq | int | 否 | 冗余自 movement |
| reverses_id | uuid | 是 | 首版恒为 NULL |
| created_at / created_by | | 否 | |

冗余四列的理由：收发存汇总与期末库存价值表按会计期间区间聚合，若每次都回连 movements 会在 50 万行规模上产生 hash join，实测无法稳定落在 10 秒通过线内。冗余列与 movements 的一致性由写入路径保证，并由对账检查 R3 逐条核对，不依赖人工纪律。

索引：`pk`；`ix_stock_qty_entries_legal_entity_id_created_at`（基线）；`ix_stock_qty_entries_le_dim_seq` 列为 `(legal_entity_id, warehouse_id, material_id, batch_no, accounting_period_seq)`；`ix_stock_qty_entries_le_bizdate` 列为 `(legal_entity_id, business_date, id)`；`ix_stock_qty_entries_movement` 列为 `(movement_id, line_no)`；`ix_stock_qty_entries_legal_entity_id_material_id` 列为 `(legal_entity_id, material_id)`，供裁定 A-13 的物料引用探针做存在性判定，索引名与所在表按基线第 3.10 节的 `ix_<table>_<col…>` 规则一致，不登记任何命名例外。

表 3，`inventory.stock_value_entries`，库存金额流水，仅追加。基线第 3.2 节已登记该表名。

| 列 | 类型 | 可空 | 约束与说明 |
|---|---|---|---|
| id | uuid | 否 | pk |
| legal_entity_id | uuid | 否 | |
| security_level | smallint | 否 | 默认 20，金额与单价列另由字段级密级 30 控制 |
| data_scope_tags | text[] | 否 | |
| movement_id | uuid | 否 | fk_stock_value_entries_stock_movements |
| line_no | int | 否 | |
| qty_entry_id | uuid | 是 | fk_stock_value_entries_stock_qty_entries，VALUE_ADJUST 时为 NULL，其余必须非空 |
| source_doc_line_id | uuid | 否 | |
| warehouse_id | uuid | 否 | |
| material_id | uuid | 否 | |
| quantity | numeric(18,6) | 否 | 与同源数量流水同值，VALUE_ADJUST 时为 0 |
| amount | numeric(18,2) | 否 | 入库为正、出库为负、调整可正可负；不设非零 CHECK，单价为零时取 0，理由见第 4.3 节边界条件 |
| applied_unit_price | numeric(18,6) | 否 | 本次实际取价，VALUE_ADJUST 时为 0 |
| pricing_branch | text | 否 | ck 取值见下文九项 |
| value_balance_after | numeric(18,2) | 否 | |
| qty_balance_after | numeric(18,6) | 否 | 该法人该仓库该物料全批次合计结存 |
| moving_avg_unit_price_after | numeric(18,6) | 否 | |
| variance_split_id | uuid | 是 | VALUE_ADJUST 时指向 variance_splits |
| business_date / accounting_period_id / accounting_period_seq | | 否 | 冗余自 movement |
| reverses_id | uuid | 是 | 首版恒为 NULL |
| created_at / created_by | | 否 | |

`pricing_branch` 的九项取值：`ESTIMATED_PO_PRICE`（采购收货暂估）、`OVERBILL_INVOICE_PRICE`（超量开票反向匹配）、`MOVING_AVERAGE`、`MOVING_AVERAGE_CLEARING`（移动加权平均分支下的出清归零）、`ORIGINAL_DELIVERY_PRICE`（销售退货零结存分支）、`ORIGINAL_RECEIPT_PRICE`（发票已登记的采购退货零结存分支）、`ORIGINAL_ESTIMATE_PRICE`（发票未登记的采购退货原额冲回）、`VARIANCE_ON_HAND`（价差拆分的尚有库存部分）、`MIGRATION_OPENING`。写迁移时以此为准，与第 4.1 节 `PricingBranch` 枚举的九项一一对应。

索引：`pk`；`ix_stock_value_entries_legal_entity_id_created_at`（基线）；`ix_stock_value_entries_le_seq_dim` 列为 `(legal_entity_id, accounting_period_seq, warehouse_id, material_id)`，这是期末库存价值表与收发存汇总金额侧的主查询路径；`ix_stock_value_entries_movement` 列为 `(movement_id, line_no)`。

表 4，`inventory.variance_splits`，价差拆分记录，仅追加。

| 列 | 类型 | 可空 | 约束与说明 |
|---|---|---|---|
| id | uuid | 否 | pk |
| legal_entity_id / security_level / data_scope_tags | | 否 | |
| movement_id | uuid | 否 | 指向 VALUE_ADJUST 的 movement |
| source_doc_id / source_doc_no / source_doc_line_id | | 否 | 采购发票登记单与其明细行 |
| warehouse_id / material_id | uuid | 否 | |
| matched_quantity | numeric(18,6) | 否 | ck 大于 0 |
| total_variance_amount | numeric(18,2) | 否 | 本次匹配的发票不含税金额减本次回冲暂估金额，由调用方传入 |
| on_hand_quantity | numeric(18,6) | 否 | ck 大于等于 0 |
| issued_quantity | numeric(18,6) | 否 | ck 大于等于 0 |
| on_hand_variance_amount | numeric(18,2) | 否 | |
| issued_variance_amount | numeric(18,2) | 否 | |
| uncovered_before | numeric(18,6) | 否 | 本次匹配前的未被价差覆盖在库数量 |
| uncovered_after | numeric(18,6) | 否 | 本次匹配后的取值 |
| business_date / accounting_period_id / accounting_period_seq | | 否 | |
| reverses_id | uuid | 是 | 首版恒为 NULL |

表级 CHECK 两条：`ck_variance_splits_qty_split` 断言 `on_hand_quantity + issued_quantity = matched_quantity`；`ck_variance_splits_amount_split` 断言 `on_hand_variance_amount + issued_variance_amount = total_variance_amount`。这两条把规格第 17.2 章必测分支十一的两句判定固化到数据库层。

索引：`pk`；基线 ix；`ix_variance_splits_le_dim` 列为 `(legal_entity_id, warehouse_id, material_id, created_at)`；`ux_variance_splits_src_line` 唯一，列为 `(legal_entity_id, source_doc_line_id, warehouse_id, material_id)`。

表 5，`inventory.stock_qty_balances`，数量账余额，可更新。

| 列 | 类型 | 可空 | 约束与说明 |
|---|---|---|---|
| 公共列全套（含 row_version） | | | |
| warehouse_id / material_id | uuid | 否 | |
| batch_no | text | 否 | 默认 `'-'` |
| quantity | numeric(18,6) | 否 | ck_stock_qty_balances_non_negative 断言大于等于 0 |
| last_movement_id | uuid | 是 | |
| last_qty_entry_id | uuid | 是 | |

索引：`pk`；基线 ix；`ux_stock_qty_balances_le_dim` 唯一，列为 `(legal_entity_id, warehouse_id, material_id, batch_no)`；`ix_stock_qty_balances_le_mat` 列为 `(legal_entity_id, material_id, warehouse_id)`，用于可用量查询按物料聚合。

数据库层的非负 CHECK 是规格第 17.3 章库存数量守恒的最后一道闸，与应用层的提交时校验共同构成两层防线。

表 6，`inventory.stock_value_balances`，金额账余额，可更新。

| 列 | 类型 | 可空 | 约束与说明 |
|---|---|---|---|
| 公共列全套 | | | |
| warehouse_id / material_id | uuid | 否 | |
| quantity | numeric(18,6) | 否 | 全批次合计结存，ck 大于等于 0 |
| value_amount | numeric(18,2) | 否 | 允许为负，理由见第 4.6 节 |
| moving_avg_unit_price | numeric(18,6) | 否 | 派生值，结存为 0 时取 0 |
| last_movement_id | uuid | 是 | |

CHECK：`ck_stock_value_balances_zero_price` 断言 `quantity > 0 OR moving_avg_unit_price = 0`，把规格第 5.2 章退货回冲取价三分支末句的结存数量为零时单价归零固化到数据库层。不设 `quantity = 0 → value_amount = 0` 的 CHECK，理由见第 4.6 节的残值口径。

索引：`pk`；基线 ix；`ux_stock_value_balances_le_dim` 唯一，列为 `(legal_entity_id, warehouse_id, material_id)`。

表 7，`inventory.variance_coverage_balances`，未被价差覆盖在库数量，可更新。

| 列 | 类型 | 可空 | 约束与说明 |
|---|---|---|---|
| 公共列全套 | | | |
| warehouse_id / material_id | uuid | 否 | |
| uncovered_quantity | numeric(18,6) | 否 | ck_variance_coverage_balances_non_negative 断言大于等于 0 |
| last_movement_id | uuid | 是 | |

索引：`pk`；基线 ix；`ux_variance_coverage_balances_le_dim` 唯一，列为 `(legal_entity_id, warehouse_id, material_id)`。

表 8，`inventory.serial_states`，序列号当前状态，可更新。

| 列 | 类型 | 可空 | 约束与说明 |
|---|---|---|---|
| 公共列全套 | | | |
| serial_no | text | 否 | ck 长度 1 至 64 且字符集 `[A-Za-z0-9._-]` |
| material_id | uuid | 否 | |
| warehouse_id | uuid | 是 | 状态为 SHIPPED 时保留最后所在仓库 |
| batch_no | text | 否 | 默认 `'-'` |
| status | text | 否 | ck 取值 `IN_STOCK`、`SHIPPED` |
| last_movement_id | uuid | 否 | |
| last_qty_entry_id | uuid | 否 | |

索引：`pk`；基线 ix；`ux_serial_states_le_serial_no` 唯一，列为 `(legal_entity_id, serial_no)`，依据为 PRD 第 4.5.2 节序列号在该法人内唯一；`ix_serial_states_le_dim_status` 列为 `(legal_entity_id, warehouse_id, material_id, status)`。

表 9，`inventory.stock_movement_serials`，序列号出入库明细，仅追加。

| 列 | 类型 | 可空 | 约束与说明 |
|---|---|---|---|
| id / legal_entity_id / security_level / data_scope_tags | | 否 | |
| movement_id | uuid | 否 | fk_stock_movement_serials_stock_movements |
| qty_entry_id | uuid | 否 | fk_stock_movement_serials_stock_qty_entries |
| serial_no | text | 否 | 同上字符集与长度约束 |
| material_id / warehouse_id | uuid | 否 | |
| direction | text | 否 | ck 取值 `IN`、`OUT` |
| business_date / accounting_period_id / accounting_period_seq | | 否 | |
| reverses_id | uuid | 是 | 首版恒为 NULL |
| created_at / created_by | | 否 | |

索引：`pk`；基线 ix；`ux_stock_movement_serials_entry_serial` 唯一，列为 `(qty_entry_id, serial_no)`；`ix_stock_movement_serials_le_serial` 列为 `(legal_entity_id, serial_no, created_at)`，用于序列号追溯。

#### 3.2 RLS 策略

九张表全部带 `legal_entity_id`，逐表按基线第 3.8 节的模板生成，不写任何变体。

```sql
alter table inventory.<t> enable row level security;
alter table inventory.<t> force row level security;
create policy rls_<t>_le on inventory.<t>
  using (legal_entity_id = nullif(current_setting('app.legal_entity_id', true), '')::uuid)
  with check (legal_entity_id = nullif(current_setting('app.legal_entity_id', true), '')::uuid);
```

不设 BYPASSRLS 角色。对账组件按规格第 7.7 章的内部对账系统安全上下文逐法人遍历，每轮只写入单一法人的会话变量。

#### 3.3 迁移编号与顺序

目录 `db/migrations/inventory/`，迁移历史落在全局唯一的 `platform_core.schema_history`。执行顺序由单一全局 Runner 按文件版本号全序排定，不存在任何模块顺序声明文件。本阶段文件的版本号只需晚于其真实被引用的对象：第 1 号晚于阶段 2 建立 `ep_mod_inventory` 属主角色的引导脚本，第 12 号晚于阶段 2 建立 `platform_core.append_only_registry` 与 `attach_table_guards` 的迁移，第 2 至 11 号与第 13 号只引用本 schema 内先建的对象。本阶段全部文件的版本号早于阶段 7 的 procure 建表迁移与阶段 11 的 costing 文件，与阶段 8 排在阶段 7 与阶段 11 之前一致；`warehouse_id` 与 `material_id` 指向的 mdm 建表迁移版本号早于本阶段全部文件，`source_doc_id` 与 `source_doc_line_id` 是多态来源单据引用，本阶段不向 procure、sales、invoice 与 ledger 的对象建立任何跨 schema 外键，因此不存在引用后建对象的情形。原句称本阶段版本号晚于其引用的 procure 对象的建表迁移，与本节上表的取值和阶段次序均不符，该句作废。

| 序 | 文件名 | 内容 | 在线变更边界 |
|---|---|---|---|
| 1 | V202610120900__inventory_create_schema.sql | 建 schema 与 `ep_mod_inventory` 属主授权 | 停机窗口内执行，本阶段唯一一次 |
| 2 | V202610120901__inventory_create_stock_movements.sql | 表 1 加索引加 RLS | 新增表，可在线 |
| 3 | V202610120902__inventory_create_stock_qty_entries.sql | 表 2 | 新增表，可在线 |
| 4 | V202610120903__inventory_create_stock_value_entries.sql | 表 3 | 新增表，可在线 |
| 5 | V202610120904__inventory_create_variance_splits.sql | 表 4 | 新增表，可在线 |
| 6 | V202610120905__inventory_create_stock_qty_balances.sql | 表 5 | 新增表，可在线 |
| 7 | V202610120906__inventory_create_stock_value_balances.sql | 表 6 | 新增表，可在线 |
| 8 | V202610120907__inventory_create_variance_coverage_balances.sql | 表 7 | 新增表，可在线 |
| 9 | V202610120908__inventory_create_serial_states.sql | 表 8 | 新增表，可在线 |
| 10 | V202610120909__inventory_create_movement_serials.sql | 表 9 | 新增表，可在线 |
| 11 | V202610120910__inventory_create_report_indexes.sql | 三条复合索引，两条报表专用与一条物料引用探针专用，全部 `CREATE INDEX CONCURRENTLY` | 可在线，单次锁持有不超过 5 秒 |
| 12 | V202610120911__inventory_backfill_append_only_registry.sql | 向 `platform_core.append_only_registry` 登记五张仅追加表 | 仅数据登记，可在线 |
| 13 | V202610120912__inventory_create_dataset_views.sql | 建 `inventory.v_stock_value_entries` 并授予 `ep_analyst_ro` | 新增视图，可在线 |

每个文件头部带 `-- rollback:` 段。第 2 至 10 号的回退语句为对应的 `drop table`，第 11 号为 `drop index concurrently`，第 12 号为删除本次登记的五行并 drop 该五张表上对应的 `assert_append_only` 触发器，第 13 号为 `drop view`。第 1 号注明只能用升级前备份回退。迁移会话固定 `SET lock_timeout = '5s'` 与 `SET statement_timeout = '30min'`，不在迁移中调用应用代码，不在同一文件中既建表又回填数据。

#### 3.4 本阶段新增的命名决定

按基线第 0 节的纪律，以下三项基线未覆盖，本阶段决定并在阶段结束时回写基线。

一是余额类表的后缀。基线第 3.2 节定义了 `_lines`、`_entries`、`_links`、`_attachments` 四种后缀，未覆盖余额类。本阶段决定余额类表统一用 `_balances` 后缀，语义为按维度组合唯一、可更新、随流水同步维护的当前值。

二是二级明细表的命名。`stock_movement_serials` 是 `stock_qty_entries` 之下的第二级明细，不适用 `_lines`。本阶段决定二级明细表命名为主表单数加语义复数。

三是索引名超长时的缩写。PostgreSQL 标识符上限为 63 字节，`ux_stock_movements_legal_entity_id_source_doc_type_source_doc_id` 为 65 字节。本阶段决定超长时按列名取语义缩写并在数据字典中登记全称映射，缩写词表固定为 `le` 对应 legal_entity_id、`dim` 对应该表的完整维度列组、`seq` 对应 accounting_period_seq、`src` 对应 source。
#### 3.5 受治理数据集视图与仅追加登记

两项跨阶段登记随本阶段迁移一并交付，二者都不是本模块自用的结构，但由本模块作为基表所有者提供。

一是受治理数据集视图（裁定 A-18）。视图名固定为 `inventory.v_stock_value_entries`，dataset code 固定为 `inventory_stock_value_entries`，grain 取 ENTRY，由第 13 号迁移建立。视图必须含 `legal_entity_id`、`security_level`、`data_scope_tags` 三列，同一迁移内执行 `GRANT SELECT ON inventory.v_stock_value_entries TO ep_analyst_ro`，不授予 `ep_app_rw` 之外的任何写权限。视图取数为 `inventory.stock_value_entries`，不做聚合、不跨 schema 连接，金额与单价列的字段级密级仍为 30，投影口径与第 5 节一致。列名与类型签名必须与阶段 11 的 `reporting.dataset_fields` 登记一致，由阶段 11 的 `reporting-dataset-signature-matched` 按降级口径校验，即不一致时关闭该数据集对应的报表入口、登记降级窗口并告警，不拒绝进程启动；本阶段在退出条件中把该列签名同步给阶段 11。

二是仅追加登记（裁定 B-02）。第 12 号迁移向 `platform_core.append_only_registry` 登记五行，`schema_name` 一律取 `inventory`，`table_name` 依次取 `stock_movements`、`stock_qty_entries`、`stock_value_entries`、`variance_splits`、`stock_movement_serials`，`mode` 一律取 `APPEND_ONLY`，`mutable_columns` 一律取 `'{}'`。登记列以阶段 2 实建的四列为准，本阶段不写入这四列之外的任何列。文件内先按上述五行插入登记，再依次调用 `platform_core.attach_table_guards('inventory','stock_movements')`、`('inventory','stock_qty_entries')`、`('inventory','stock_value_entries')`、`('inventory','variance_splits')`、`('inventory','stock_movement_serials')`，顺序不得颠倒，挂接函数读登记表取可变列白名单，先挂接后登记取不到 `mutable_columns`。第 2 至 10 号建表迁移一律不调用 `attach_table_guards`，五张仅追加表的触发器只在本文件内挂接。该迁移的主要创建对象是 inventory 五张表上的仅追加触发器与其登记行，按裁定通则第五条放在 `db/migrations/inventory/` 目录下，其版本号晚于阶段 2 建立 `platform_core.append_only_registry` 与 `attach_table_guards` 的迁移，空库上按文件版本号全序执行时其前置对象已建立。登记与触发器的一致性由 `db/checks/append_only_consistency.sql` 断言，`xtask sqlcheck` 执行。

### 4. 领域模型与关键算法

#### 4.1 核心类型

值对象（`ep-domain-inventory/src/value/`）。

```rust
pub struct BatchNo(String);          // 不变式：非空、长度 ≤ 64、字符集受限；EMPTY 常量为 "-"
pub struct SerialNo(String);         // 同上字符集与长度
pub enum MovementDirection { In, Out, ValueAdjust }
pub enum MovementReason { PurchaseReceipt, PurchaseReceiptOverbillMatched, SalesReturn,
                          DeliveryConfirmation, PurchaseReturnInvoiced,
                          PurchaseReturnUninvoiced, PurchaseInvoiceVariance, MigrationOpening }
pub enum PricingBranch { EstimatedPoPrice, OverbillInvoicePrice, MovingAverage,
                         MovingAverageClearing, OriginalDeliveryPrice, OriginalReceiptPrice,
                         OriginalEstimatePrice, VarianceOnHand, MigrationOpening }
pub struct StockKey { warehouse_id: Id, material_id: Id }            // 金额账与未覆盖数量的维度
pub struct BatchKey { warehouse_id: Id, material_id: Id, batch: BatchNo } // 数量账的维度
pub struct SourceRef { doc_type: SourceDocType, doc_id: Id, doc_no: String,
                       line_id: Id, line_no: i32 }
pub struct OriginalCostAllocation { source_line_id: Id, quantity: Quantity, unit_price: UnitPrice }
```

聚合（`src/model/`）。

```rust
pub struct StockQtyBalance { key: BatchKey, quantity: Quantity, row_version: u64 }
pub struct StockValueBalance { key: StockKey, quantity: Quantity, value_amount: Money,
                               moving_avg: UnitPrice, row_version: u64 }
pub struct VarianceCoverage { key: StockKey, uncovered: Quantity, row_version: u64 }
pub struct SerialState { serial: SerialNo, material_id: Id, warehouse_id: Option<Id>,
                         batch: BatchNo, status: SerialStatus, row_version: u64 }
pub struct StockMovement { /* 仅追加聚合，含 qty_entries、value_entries、serial_rows */ }
```

#### 4.2 取价入参的表达

取价规则由规格第 5.2 章定义，但其中的分支判定依赖库存当前状态，因此调用方只能声明取价意图，最终分支由本模块判定。入参枚举如下，这是本阶段与采购、销售、发票三个模块之间最关键的契约。

```rust
pub enum InboundPricing {
    Explicit { unit_price: UnitPrice },                       // 采购收货暂估、超量匹配、迁移期初
    ReturnAtMovingAverage { fallback: Vec<OriginalCostAllocation> }, // 销售退货
}
pub enum OutboundPricing {
    MovingAverage,                                            // 交付确认发货
    ReturnAtMovingAverage { fallback: Vec<OriginalCostAllocation> }, // 采购发票已登记的采购退货
    OriginalEstimate { allocations: Vec<OriginalCostAllocation> },   // 采购发票未登记的采购退货
}
```

对应关系逐条给出。`Explicit` 承载规格第 5.2 章采购收货事件的按采购订单不含税单价暂估，以及超量开票三条结清路径中路径一的按已登记发票不含税单价。`ReturnAtMovingAverage` 承载退货回冲的取价三分支中的前两个分支，即优先按退货发生时该仓库该物料的移动加权平均单价，结存为零或单价为零时改按 fallback 逐笔取原单价。`OriginalEstimate` 承载第三个分支，即采购发票尚未登记的采购退货一律按该次收货的原暂估单价原额冲回，不适用移动加权平均单价，因此没有条件判定。

采购发票是否已登记这一判定由 invoice 模块给出（PRD 第 4.6.2 节明确该字段由系统判定、用户不可干预），取数入口为阶段 10 交付的 `ep_contract_invoice::ReceiptInvoiceMatchQueryPort::match_state`，本模块不判定，只按调用方选择的枚举分支执行。`fallback` 与 `allocations` 中的原单价由调用方从其自身单据上固化的原结转单价或原入账单价读出；本模块提供 `ep_contract_inventory::InventoryPricingLookupPort::original_unit_price_by_source_line` 查询端口，使调用方可以从库存金额流水回查某条来源单据行当时的 `applied_unit_price`。收货入账单价的权威出处唯一为 `inventory.stock_value_entries.applied_unit_price`，`procure.goods_receipt_line_costings` 按裁定 C-12 只保留数量与金额的分配关系、不再保留单价列，两处各存一份单价的形态由该裁定消除。

#### 4.3 算法一：入库过账

输入为一条 `InboundLine`。步骤如下。

1. 校验 `quantity > 0`；物料启用批次管理时 `batch_no` 必须非 `'-'`，未启用时必须等于 `'-'`；物料启用序列号管理时序列号条数必须等于 `quantity` 且 `quantity` 必须为整数，行内序列号不得重复。
2. 按第 6.2 节的锁顺序取 `stock_value_balances`、`variance_coverage_balances`、`stock_qty_balances` 三行，不存在时先以零值 upsert 再 `SELECT ... FOR UPDATE`。
3. 取价。`Explicit { unit_price }` 时 `unit = unit_price`，分支为 `ESTIMATED_PO_PRICE` 或 `OVERBILL_INVOICE_PRICE` 或 `MIGRATION_OPENING`，由 `reason` 决定。`ReturnAtMovingAverage` 时：若 `value_balance.quantity > 0` 且 `moving_avg > 0`，则 `unit = moving_avg`，分支为 `MOVING_AVERAGE`；否则校验 `Σ fallback.quantity = 本行数量`，不等则返回 `INVENTORY.STOCK_VALUE_BALANCE.ORIGINAL_PRICE_ALLOCATION_MISMATCH`，成立则分支为 `ORIGINAL_DELIVERY_PRICE`。
4. 计算金额。单一单价时 `amount = round(unit × quantity, 2)`。逐笔取价时 `amount = Σ round(unit_i × quantity_i, 2)`，即逐笔 round 再累加，理由是每一笔对应一张原交付确认单，其回冲金额必须能与该单原结转金额逐笔对应（规格第 17.2 章必测分支十四要求回冲后存货金额账、主营业务成本与原结转金额可对应）。
5. 更新余额。`qty_balance.quantity += quantity`；`value_balance.quantity += quantity`；`value_balance.value_amount += amount`。
6. 重算单价。`moving_avg = if value_balance.quantity == 0 { 0 } else { round(value_amount / quantity, 6) }`。中间值以 Decimal 全精度参与，只在此处 round 一次。
7. 更新未覆盖数量。`uncovered += quantity`。
8. 序列号处置，见第 4.7 节。
9. 写 movement、qty_entry、value_entry、serial 行，回填 `qty_balance_after`、`value_balance_after`、`moving_avg_unit_price_after`。
10. 断言五组不变量，见第 4.8 节。

边界条件：`quantity` 为零或负一律 `VALIDATION`；`unit_price` 为负时接受（供应商价格调整可能产生负单价的迁移场景）；`unit_price` 为零时 `amount` 取 0，金额流水照写、金额账不变，不设跳过写入的分支，理由是第 4.8 节 I2 要求每条 `IN` 或 `OUT` 数量流水有且只有一条金额流水与之对应，跳过写入会使该断言在单价为零的迁移期初必然不成立。第 4.4 节移动加权平均单价为零时的出库同此处理。

#### 4.4 算法二：出库过账

1. 校验同上，序列号还需校验其当前处于 `IN_STOCK` 且所在仓库与物料匹配。
2. 按同一锁顺序取三行余额，`stock_qty_balances` 不存在即判定结存不足。
3. 结存充足性校验：`qty_balance.quantity >= quantity`，不成立返回 `INVENTORY.STOCK_QTY_BALANCE.INSUFFICIENT_BALANCE`，错误详情携带该仓库该物料该批次的当前结存数量与本次请求数量（PRD 第 5.6.4 节）。本阶段一律硬阻断，不提供允许负结存的配置，见第 11 节对 U-G-02 的临时取值。
4. 取价。`MovingAverage` 时 `unit = moving_avg`，分支 `MOVING_AVERAGE`。`ReturnAtMovingAverage` 时按第 4.3 节步骤 3 的同一判定，fallback 分支为 `ORIGINAL_RECEIPT_PRICE`。`OriginalEstimate` 时无条件走 fallback，分支 `ORIGINAL_ESTIMATE_PRICE`。
5. 计算金额，含出清归零规则。设 `qty_after = value_balance.quantity - quantity`。
   - 若分支为 `MOVING_AVERAGE` 且 `qty_after == 0`，则 `amount = value_balance.value_amount`，分支改记为 `MOVING_AVERAGE_CLEARING`。
   - 其余情形 `amount = round(unit × quantity, 2)` 或逐笔累加。
6. 更新余额：`qty_balance.quantity -= quantity`；`value_balance.quantity -= quantity`；`value_balance.value_amount -= amount`。
7. 重算单价，同第 4.3 节步骤 6。
8. 更新未覆盖数量：`uncovered = max(0, uncovered - quantity)`。语义是离开仓库的货物不再可能被后续价差调整其存货价值，因此必须先消耗未被覆盖的部分。该 clamp 保证 `0 ≤ uncovered ≤ Σ批次结存` 在任何写入序列下恒成立。
9. 写流水，序列号置 `SHIPPED`。
10. 断言不变量。

#### 4.5 算法三：价差拆分

对应规格第 5.2 章价差拆分规则的全文。入口为 `InventoryVariancePort::split_variance`，调用方按裁定 A-10 收窄为 `ep-app-invoice`，采购模块不再直接调用本入口。输入为 `(warehouse_id, material_id, matched_quantity, total_variance_amount)` 的列表，`total_variance_amount` 由调用方按本次匹配的发票不含税金额减本次回冲暂估金额算出，本模块不重算。

1. 校验 `matched_quantity > 0`。
2. 锁 `stock_value_balances` 与 `variance_coverage_balances`。
3. `on_hand_quantity = min(matched_quantity, uncovered_quantity)`。
4. `issued_quantity = matched_quantity - on_hand_quantity`。
5. `on_hand_variance = round(total_variance × on_hand_quantity / matched_quantity, 2)`，比值以 Decimal 全精度计算，只在此处 round 一次。
6. `issued_variance = total_variance - on_hand_variance`。尾差全部落在已出库部分，依据是基线第 3.5 节第三条尾差归属规则；理由在基线中已写明，即该部分不再经过存货科目，尾差留在此处不会破坏存货金额账与数量账的一致性。
7. `value_balance.value_amount += on_hand_variance`，`quantity` 不变。
8. 重算 `moving_avg`。当 `value_balance.quantity == 0` 时，由第 3 步可知 `uncovered = 0` 故 `on_hand_quantity = 0` 故 `on_hand_variance = 0`，金额账不变，单价保持 0，与 `ck_stock_value_balances_zero_price` 不冲突。
9. `uncovered_quantity -= on_hand_quantity`。
10. 写记录。`total_variance == 0` 时直接返回零结果，不写 movement。`on_hand_variance == 0` 但 `issued_variance != 0` 时写 movement 与 variance_splits，不写 value_entry。两者均非零时三者都写。
11. 返回 `issued_variance` 给调用方，由调用方计入当期主营业务成本并生成分录。

跨发票不重复占用的证明链：`uncovered` 只在入库时增加、在出库时减少、在价差处理时按本次尚有库存数量扣减，登记发票本身不重置该数量（步骤 9 只减不增）。因此同一法人同一仓库同一物料先后由两张发票匹配时，第二张发票读到的 `uncovered` 已扣除第一张的占用，两张发票的尚有库存数量合计不超过该期间的实际在库数量。这正是规格第 17.2 章必测分支十一的两句判定，本阶段以集成测试 I-07 直接验证。

#### 4.6 出清归零与零结存残值的口径

这是本阶段唯一一处偏离基线的设计，按基线第 12 节的纪律单列。

偏离项：基线第 3.5 节第一条规定出库金额一律等于移动加权平均单价乘出库数量并 round 到 2 位，第二条允许除不尽产生的尾差留在金额余额中、结存数量为零时单价归零。本阶段在移动加权平均分支下追加一条出清归零规则：当一次出库使该法人该仓库该物料的全批次合计结存数量归零时，本次出库金额取当前库存金额余额全额。

理由：规格第 17.3 章存货金额账与数量账一致这一项要求库存金额账合计等于按仓库与物料的结存数量乘加权平均单价之和。若结存为零而金额余额留有尾差残值，该等式左侧非零、右侧为零，强制不变量在结存归零点上必然不成立，关账被拦截且无可达的解除路径（首版无盘点、无库存调整单据，PRD 第 5.6.4 节修复路径只有补登或冲正来源事件，无法消除纯舍入残值）。出清归零使该等式在归零点严格成立，且本次出库金额与按单价乘数量算出的金额之差不超过 `数量 × 5×10^-7`，仍在按当时移动加权平均单价结转的语义之内。

影响范围：仅影响 `MOVING_AVERAGE` 分支的最后一笔出库，其金额同源传给财务模块生成主营业务成本或存货贷方腿，不产生任何额外分录。规格第 17.2 章必测分支十四的零结存销售退货取价不受影响，反而因为归零后单价为零而稳定命中零结存分支。

不适用范围与残值口径：按原单价固化取价的三个分支（`ORIGINAL_DELIVERY_PRICE`、`ORIGINAL_RECEIPT_PRICE`、`ORIGINAL_ESTIMATE_PRICE`）不适用出清归零。理由是规格第 5.2 章明确规定采购发票未登记的采购退货按该次收货的原暂估单价与数量原额冲回，且第 17.2 章必测分支十二明确判定该冲回金额等于原暂估金额且不随期间内加权平均单价变动，与出清归零不可兼得；两者冲突时按规格优先。这三个分支若使结存归零而金额余额非零，该残值保留在金额账并在下一次入库时被自然吸收，同时由对账检查 R2 按零结存残值单列为可追溯观察项，不生成勾稽差异事项、不拦截关账，但必须能逐条追溯到产生它的原单价分支流水；追溯不到来源的一律按差异处理并拦截关账。

同步提出的基线修订：基线第 3.5 节尾差归属第二条追加一句，即移动加权平均分支下结存归零时金额余额一并归零，按原单价固化取价的分支不适用该句。

`value_amount` 允许为负：规格只对结存数量规定不得为负，未对金额余额规定。当发票不含税单价显著低于暂估单价且在库数量接近耗尽时，价差的尚有库存部分可能把金额余额压至负值。硬拒绝会使合法的低价发票登记无法入账，反而制造账外差额，因此本阶段允许负值，由对账检查 R2 生成差异事项交数据责任人处理（规格第 15.2 章）。这是本阶段的显式假设，理由已如上。

#### 4.7 序列号状态机

| 当前状态 | 事件 | 目标状态 | 守卫条件 | 违反时的错误码 |
|---|---|---|---|---|
| 不存在 | 入库 | IN_STOCK | 该法人内该序列号不存在 | 无 |
| SHIPPED | 入库 | IN_STOCK | 物料一致 | `INVENTORY.SERIAL_STATE.MATERIAL_MISMATCH` |
| IN_STOCK | 入库 | 拒绝 | 该序列号已在库 | `INVENTORY.SERIAL_STATE.ALREADY_IN_STOCK` |
| IN_STOCK | 出库 | SHIPPED | 物料一致且仓库一致 | `INVENTORY.SERIAL_STATE.WAREHOUSE_MISMATCH` |
| SHIPPED | 出库 | 拒绝 | 不在库 | `INVENTORY.SERIAL_STATE.NOT_IN_STOCK` |
| 不存在 | 出库 | 拒绝 | 不在库 | `INVENTORY.SERIAL_STATE.NOT_IN_STOCK` |

退货入库后允许再次发出，且允许在与原发货仓库不同的仓库入库，入库时更新 `warehouse_id` 与 `batch_no`。这是本阶段对未决事项 U-G-04 的临时取值，理由是首版无调拨，若不允许换仓入库则退回到非原仓库的货物将永久不可发出，闭环断裂。

#### 4.8 五组不变量断言

写入路径末尾在同一事务内断言，任一不成立即先写审计事件再中止当前请求，不中止进程（基线第 10.2 节）。

I1，数量守恒：本次写入后 `qty_balance.quantity` 等于该维度全部数量流水 `quantity` 的代数和，且大于等于 0。
I2，两账同源：本次每条 `IN` 或 `OUT` 方向的数量流水有且只有一条金额流水与之对应（`qty_entry_id` 相等），且两者 `quantity` 相同、`accounting_period_id` 相同、`business_date` 相同。
I3，金额余额一致：`value_balance.value_amount` 等于该维度全部金额流水 `amount` 的代数和；`value_balance.quantity` 等于该维度全部批次的 `qty_balance.quantity` 之和。
I4，单价重算：`value_balance.quantity == 0` 时 `moving_avg == 0`；大于 0 时 `moving_avg == round(value_amount / quantity, 6)`。
I5，未覆盖上界：`0 ≤ uncovered_quantity ≤ value_balance.quantity`。
#### 4.9 本阶段实现的外部 trait

三项，全部由其他阶段定义、本阶段实现，三个定义方阶段（5、5、9a）均早于本阶段，实现类型名与位置一律照跨阶段归属裁定，不另取名。原第四项存货子账侧余额提供者按裁定 G-01 改为本模块自有端口，移入第 5.1 节，不再计入本节。

一是 `ep_contract_mdm::MaterialUsageProbe::has_stock_movement(&self, ctx: &SecurityContext, material_id: Id<Material>) -> Result<bool, AppError>`，trait 由阶段 5 定义（裁定 A-13）。实现类型固定为 `InventoryMaterialUsageProbe`，位于 `crates/application/inventory/src/probe/material_usage.rs`，取数为 `inventory.stock_qty_entries` 上按 `(legal_entity_id, material_id)` 的数量流水存在性判定，命中索引 `ix_stock_qty_entries_legal_entity_id_material_id`。第 3.1 节表 1 的 `inventory.stock_movements` 不带 material_id 列，物料维度落在其明细表，因此判定与索引一并落在 `inventory.stock_qty_entries` 上，索引名与所在表一致，不登记命名例外。`inventory.stock_value_entries` 中 `qty_entry_id` 为空的纯金额调整行不参与该判定。该探针的注册判定不挂在启动自检上：`master-data-usage-probes-registered` 下沉为模块启用动作的前置校验，探针未注册则拒绝启用 inventory 模块，启用后不在每次进程启动时复判，八个进程不因该项拒绝启动。阶段 5 的档案停用校验对 inventory 的覆盖随本实现注入而成立，两阶段各自验收各自的部分，本阶段不接收也不登记任何顺延项。

二是 `ep_contract_mdm::MasterReferenceCounter`，trait 与注册表 `MasterReferenceCounterRegistry` 由阶段 5 定义（裁定 A-15）。实现类型固定为 `InventoryReferenceCounter`，位于 `crates/application/inventory/src/probe/reference_counter.rs`，`module_code()` 返回 `ModuleCode::Inventory`，`count_open_documents` 在 `MasterObjectKind::Material` 下返回该物料非零结存的仓库物料批次组合数，其余 object_kind 返回 0。本阶段不承担任何 `SalesTradeHistoryProvider` 或 `PurchaseTradeHistoryProvider` 实现。

三是 `ep_platform_recon::ReconCheck`，trait、注册表 `ReconRegistry` 与执行器由阶段 9a 交付（裁定 A-06）。本阶段实现两个检查并在 `apps/job-worker/src/wiring.rs` 经 `ReconRegistry::register` 注册，两个即裁定 A-06 给本阶段固定的校验项数，不多也不少：库存数量守恒，`category()` 取 `INVARIANT`；存货项子账与总账勾稽，`category()` 取 `SUBLEDGER_VS_LEDGER`。两个取值逐字取自裁定 A-06 中 `platform_core.recon_check_definitions.category` 的三项 CHECK 取值，`ReconCategory` 的判别式与该三项一一对应，本阶段不另取名。两者的 `blocks_period_close()` 均返回 true，`run_batch` 的快照入参为 `&dyn SnapshotCtx`，分批规模取第 7 节的 `EP__INVENTORY__RECON__BATCH_SIZE`，差异事项写入 `platform_core.recon_discrepancies`。第 3.1 节与第 4.6 节提到的 R2、R3 两组判据落在这两个实现内，不另起第三个检查。本阶段九张表上的 `source_doc_id`、`source_doc_line_id`、`warehouse_id`、`material_id` 四类跨模块引用不另建 `CROSS_MODULE_LINK` 校验项，依据是裁定 A-06 给本阶段固定的校验项只有上述两个；其中 `warehouse_id` 与 `material_id` 是单目标引用，`source_doc_id` 与 `source_doc_line_id` 是多态来源单据引用，四者的引用存在性一律按基线第 3.3 节的跨 schema 引用规则处理，业务状态与法人一致性由 ep-contract-mdm 与来源模块的契约校验承担，本阶段不在阶段计划内另立口径，也不再以总览 R14 的未覆盖面清单作为依据。

### 5. API 契约

全部为只读端点，路径前缀 `/api/v1/inventory`，承载进程 core-server。共同约定：请求头按基线第 5.6 节固定集合；`X-Legal-Entity-Id` 必填并经授权法人集合校验后写入 `app.legal_entity_id`；GET 不要求 `Idempotency-Key`；响应按基线第 5.2 节封套；分页、排序、过滤按基线第 5.3 节，`filter` 的十种算子中本模块的白名单逐端点给出。

金额相关字段的统一处理：`unit_price`、`amount`、`value_amount`、`moving_avg_unit_price` 四类字段的字段级密级为 30，调用方不具备该密级时字段整体从响应中省略而不是置空，也不返回错误，符合规格第 12.2 章字段级权限与 PRD 第 5.7.1 节数量可见不等于金额可见的口径。

| 序 | 方法与路径 | 用途 | 主要参数 | 响应要点 | 权限 |
|---|---|---|---|---|---|
| A1 | GET /api/v1/inventory/stock-balances | 库存台账查询，PRD 第 5.6.1 节 | filter[warehouse_id]、filter[material_id]、filter[material_category_id]、filter[batch_no]、filter[include_zero]、sort 白名单为 warehouse_code、material_code、batch_no、quantity | 行含 warehouse_id、warehouse_code、material_id、material_code、material_name、batch_no、uom_code、quantity；具备金额密级时按仓库物料附 moving_avg_unit_price 与 value_amount | inventory.stock_balance:read |
| A2 | GET /api/v1/inventory/available-quantities | 可用量查询，PRD 第 5.6.2 节 | material_id 必填、warehouse_id 可选，为空时按该法人全部已启用仓库分别列出并给出合计 | 行含 warehouse_id、material_id、quantity、reserved_quantity、available_quantity；meta 含 total_quantity 与 total_available_quantity | inventory.stock_balance:read |
| A3 | GET /api/v1/inventory/stock-values | 库存金额查询，PRD 第 5.7.1 节 | filter[warehouse_id]、filter[material_id]、filter[material_category_id]、accounting_period_id 可选默认当前打开期间 | 行含 quantity、moving_avg_unit_price、value_amount；meta 含按仓库小计与法人合计 | inventory.stock_value:read 且密级 30 |
| A4 | GET /api/v1/inventory/stock-movements | 库存流水查询，PRD 第 5.6.3 节 | filter[warehouse_id]、filter[material_id]、filter[batch_no]、filter[serial_no]、filter[direction]、filter[source_doc_type]、filter[business_date]=between:、filter[accounting_period_id]=in: 两条检索路径都必须可用 | 行含 business_date、accounting_period_id、accounting_period_label、warehouse、material、batch_no、direction、quantity、amount、source_doc_type、source_doc_no、created_by；默认排序 business_date asc, source_doc_no asc | inventory.stock_movement:read |
| A5 | GET /api/v1/inventory/stock-movements/{id} | 单条流水详情 | 无 | 含全部明细行与序列号清单、value_entry 的 pricing_branch 与 variance_split_id | 同上 |
| A6 | GET /api/v1/inventory/batches | 出库选批次的候选列表 | warehouse_id 必填、material_id 必填 | 只返回 quantity 大于 0 的批次 | inventory.stock_balance:read |
| A7 | GET /api/v1/inventory/serials | 序列号在库状态批量查询，供扫码即时反馈 | filter[serial_no]=in: 上限 200 个、filter[warehouse_id]、filter[status] | 行含 serial_no、status、warehouse_id、material_id、batch_no | inventory.serial:read |
| A8 | GET /api/v1/inventory/serials/{serial_no} | 序列号追溯，PRD 第 5.6.3 节 | 无 | 含当前状态与按时间升序的全部出入库记录 | 同上 |
| A9 | GET /api/v1/inventory/stock-in-out-summaries | 收发存汇总表，PRD 第 5.7.2 节 | accounting_period_id 必填、filter[warehouse_id]、filter[material_id] | 数量侧按仓库物料批次给出期初数量、本期收入数量、本期发出数量、期末结存数量；金额侧按仓库物料给出期初金额、本期收入金额、本期发出金额、本期调整金额、期末金额；meta 带口径标注两条 | inventory.stock_movement:read，金额侧另需密级 30 |
| A10 | GET /api/v1/inventory/period-end-stock-values | 期末库存价值表，PRD 第 5.7.3 节 | accounting_period_id 必填、filter[warehouse_id] | 行含期末结存数量、期末移动加权平均单价、期末库存金额；meta 含按仓库小计、法人合计与同法人同期间存货科目余额的跳转参数 | inventory.stock_value:read 且密级 30 |

A9 的两条口径标注是硬性响应字段，不是界面文案：一是收发存汇总的收入数量与发出数量不包含只影响金额账的调整、金额列包含该调整（PRD 第 5.5.7 节）；二是期初与期末按会计期间字段划分而不按原始业务日期划分，存在顺延入账时一笔记账日期属于上一期间的库存流水会计入本期间（PRD 第 5.7.2 节期间口径）。两条以 `meta.disclosures` 数组返回，由界面原样展示。

A10 不自行计算勾稽差额，差额由对账组件判定（PRD 第 5.7.3 节界面要求）。响应中的 `meta.ledger_reference` 只给出跳转所需的法人与期间参数，不携带总账侧金额，避免在本模块内形成第二处存货科目余额取数口径。

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
#### 5.1 七个对外 trait 的完整签名

本阶段对外暴露的 trait 共七个，全部位于 ep-contract-inventory。前六个由本阶段定义并交付，第七个 `StockValueOutboundPort` 按裁定 F-05 由阶段 11 与其实现类型 `InventoryStockValueOutboundQuery` 同批追加到本 crate，本阶段不交付它，也不为它预留任何占位实现。写路径与查询路径的事务句柄一律取阶段 1 冻结的 `ep_foundation::port::Tx`，两个余额类端口不接事务句柄、改接同批冻结的 `&dyn SnapshotCtx`，见第 6.1 节；标记类型一律取 `ep_foundation::id::marker`，契约层不引入任何其他模块的类型。阶段 7 曾用的 `StockInboundPort`、`StockOutboundPort`、`StockAvailabilityQueryPort` 三个名字按裁定 C-18 作废。

```rust
// crates/contract/inventory/src/port/posting.rs
#[async_trait::async_trait]
pub trait InventoryPostingPort: Send + Sync {
    async fn post_inbound(&self, tx: &mut dyn Tx, ctx: &SecurityContext, cmd: InboundPosting)
        -> Result<InboundPostingResult, AppError>;
    async fn post_outbound(&self, tx: &mut dyn Tx, ctx: &SecurityContext, cmd: OutboundPosting)
        -> Result<OutboundPostingResult, AppError>;
    async fn find_movement_by_source(&self, tx: &mut dyn Tx, ctx: &SecurityContext, source: SourceRef)
        -> Result<Option<MovementResult>, AppError>;
}

// crates/contract/inventory/src/port/variance.rs
#[async_trait::async_trait]
pub trait InventoryVariancePort: Send + Sync {
    async fn split_variance(&self, tx: &mut dyn Tx, ctx: &SecurityContext, cmd: VarianceSplitCommand)
        -> Result<VarianceSplitResult, AppError>;
}

// crates/contract/inventory/src/port/pricing_lookup.rs
#[async_trait::async_trait]
pub trait InventoryPricingLookupPort: Send + Sync {
    async fn original_unit_price_by_source_line(&self, tx: &mut dyn Tx, ctx: &SecurityContext,
                                                source_doc_line_id: uuid::Uuid)
        -> Result<UnitPrice, AppError>;
}

// crates/contract/inventory/src/port/availability.rs
pub struct AvailabilityQuery {
    pub legal_entity_id: Id<LegalEntity>,
    pub material_id: Id<Material>,
    pub warehouse_id: Option<Id<Warehouse>>,
    pub required_on: chrono::NaiveDate,
}
pub struct AvailabilityView {
    pub warehouse_id: Id<Warehouse>,
    pub on_hand_quantity: Quantity,
    pub reserved_quantity: Quantity,
    pub available_quantity: Quantity,
}

#[async_trait::async_trait]
pub trait AvailabilityQueryPort: Send + Sync {
    async fn available(&self, tx: &mut dyn Tx, ctx: &SecurityContext, q: AvailabilityQuery)
        -> Result<Vec<AvailabilityView>, AppError>;
    async fn on_hand(&self, tx: &mut dyn Tx, ctx: &SecurityContext,
                     legal_entity_id: Id<LegalEntity>, warehouse_id: Id<Warehouse>,
                     material_id: Id<Material>, batch_no: &str) -> Result<Quantity, AppError>;
}

// crates/contract/inventory/src/port/deactivation.rs
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
                     accounting_period_id: Id<AccountingPeriod>) -> Result<Money, AppError>;
}

// crates/contract/inventory/src/port/stock_value_outbound.rs   阶段 11 同批追加（裁定 F-05）
#[async_trait::async_trait]
pub trait StockValueOutboundPort: Send + Sync {
    async fn outbound_amount(&self, snapshot: &dyn SnapshotCtx,
                             legal_entity_id: Id<LegalEntity>,
                             accounting_period_id: Id<AccountingPeriod>) -> Result<Money, AppError>;
}
```

四条调用约定随签名一并冻结。其一，交付确认的库存腿（裁定 A-09）：ep-app-sales 在 confirm_delivery 的同一事务内以 `OutboundPosting { reason: MovementReason::DeliveryConfirmation, pricing: OutboundPricing::MovingAverage, source: SourceRef { doc_type: DELIVERY_CONFIRMATION, .. }, lines }` 调用 `post_outbound`，本阶段按行返回 `cogs_amount` 与 `stock_movement_id`；`is_drop_ship` 为真时由 sales 侧整段跳过该调用。其二，`available` 与第 5 节端点 A2 共用同一投影函数，`reserved_quantity` 按第 11.2 节 U-G-01 的临时取值恒为零；`on_hand` 是阶段 7 采购退货结存充足性前置校验的取数入口。其三，能力域码与动作类别（裁定 A-20）：第 5 节十个端点的能力域码一律取 `CapabilityDomain::InventoryLedgerScan`，动作类别一律取 `ActionClass::Read`，常量按 `<USECASE_SCREAMING>_DOMAIN` 与 `<USECASE_SCREAMING>_ACTION` 声明在 `crates/contract/inventory/src/capability.rs`，`xtask configdoc` 断言每个 `/api/v1/` 路由都能解析到一对常量，缺失即构建失败。其四，子账余额端口（裁定 G-01，原裁定 B-08 的端口落位由该裁定修订）：实现类型固定为 `InventorySubledgerBalanceQuery`，位于 `crates/application/inventory/src/projection/subledger_balance.rs`，返回该法人该会计期间的存货金额账合计；调用方是阶段 10 的 `ReconciliationItemQuery` 组装处，注入行由阶段 10 写入 `apps/core-server/src/wiring/` 与 `apps/job-worker/src/wiring/` 两个目录，本阶段不为其预留任何占位实现。第七个端口 `StockValueOutboundPort` 不在本阶段冻结调用约定：其调用方是阶段 11 的 `COSTING_INVENTORY_COGS_VS_STOCK_VALUE` 检查，trait、实现类型与注入行按裁定 F-05 由阶段 11 同批交付。

### 6. 并发与事务边界

#### 6.1 事务归属

本阶段不开启任何自己的事务。三个过账端口、可用量查询端口、原单价回查端口与停用校验端口的方法签名一律接受调用方传入的 `&mut dyn Tx`，该类型由阶段 1 在 `ep_foundation::port::tx` 冻结（裁定 A-01）；第 5.1 节的子账余额端口与出库金额端口两者不接受 `&mut dyn Tx`，其入参为 `&dyn SnapshotCtx`，与本节末段两个 `ReconCheck` 的快照同源，同样不开事务，跨 crate 取具体句柄的 downcast 只允许出现在 ep-adapter-db-pg 内。全部方法在调用方用例的同一事务内执行，符合基线第 10.3 节一个用例一个事务、禁止在一个 HTTP 请求内开启多个写事务。

一次采购收货登记的完整事务内容为：采购模块写收货单与订单行回写、库存模块写 movement 与两账、财务模块写凭证与应付账款暂估台账、审计事件写入、Outbox 条目写入，全部在同一事务内提交，调用次序固定为先 `InventoryPostingPort::post_inbound` 后 `ep_contract_ledger::PostingPort::post`（裁定 C-13）。规格第 5.2 章要求财务模块按同一业务事件生成唯一一张总账凭证，且规格第 10.2 章关账受理后建立快照时要求该期间的全部凭证已可见，因此凭证不得延迟到 Outbox 消费时才生成。全部凭证一律与业务事件同事务生成、Outbox 只承载派生、通知、检索与报表数据集这一口径已由裁定 C-28 定死，本阶段以入参契约把它显式化：过账端口返回的每行金额是分录的存货腿金额，调用方必须在同一事务内使用它。

隔离级别 `READ COMMITTED`（基线第 8.4 节）。对账检查不自行开事务，其快照由 `ep_foundation::port::UnitOfWork::snapshot_transact` 导出的 `SnapshotCtx` 承载，经阶段 9a 的 ep-platform-recon 执行器逐批传入 `ReconCheck::run_batch`。

#### 6.2 锁策略

所有余额行一律先 `SELECT ... FOR UPDATE` 再按 `row_version` 条件 UPDATE。行锁已持有时版本条件恒成立，`row_version` 保留作为防御性检查与基线第 3.7 节的一致性要求，冲突仍映射为 `PLATFORM.CONCURRENCY.STALE_VERSION`。

全局锁顺序固定为三级，一次过账内的多行先按该顺序整体排序去重，再依次加锁，杜绝交叉死锁：

1. `stock_value_balances`，按 `(warehouse_id, material_id)` 升序。
2. `variance_coverage_balances`，按 `(warehouse_id, material_id)` 升序。
3. `stock_qty_balances`，按 `(warehouse_id, material_id, batch_no)` 升序。

余额行不存在时的 upsert 采用 `insert ... on conflict (legal_entity_id, warehouse_id, material_id[, batch_no]) do update set row_version = <table>.row_version returning *` 的写法，一条语句同时完成插入、取行锁与回读，避免先查后插的竞态。

`lock_timeout` 取读写池的 3 秒（基线第 10.3 节），超时按 `INFRASTRUCTURE` 分类返回 503 并可重试。

#### 6.3 幂等

三层。第一层是平台的 `Idempotency-Key`，作用域为法人、用户、端点、键值四元组，由来源模块的写端点承担。第二层是 `ux_stock_movements_le_src_doc` 唯一约束，同一来源单据只能过账一次，重复过账直接返回 `INVENTORY.STOCK_MOVEMENT.DUPLICATE_SOURCE_DOCUMENT`。第三层是过账端口的可查询语义：调用方在捕获重复错误后可调用 `find_movement_by_source` 取回既有结果，使重放路径返回与首次相同的金额（PRD 第 5.9 节同一来源单据行重复提交按幂等键识别只产生一次库存流水、返回既有结果）。

#### 6.4 与 Outbox 的关系

本阶段在同一事务内写入两类事件。`inventory.stock_movement.posted.v1` 与 `inventory.stock_movement.value_adjusted.v1`，信封按基线第 6.1 节，`aggregate_type` 取 `inventory.stock_movements`，`posting_date` 取 `business_date`，`accounting_period_id` 取本次过账的期间，`security_level` 与 `data_scope_tags` 从 movement 行取，缺失即拒绝入队（规格第 7.9 章派生存储写入的必备标签）。两个事件名的 aggregate 段一律取 `aggregate_type` 表名的单数形式；裁定 B-09 与总览第 4.2 节沿用的 `inventory.stock_value_adjusted.v1` 只有三段，违反基线第 6.1 节的四段式，基线高于裁定表，该旧名作废，任何阶段不得再引用，也不得为此在 `xtask eventcatalog` 中开命名白名单例外。

首版消费者两个，均不由本阶段交付。`inventory.stock_movement.posted.v1` 由阶段 7 的采购建议消费者消费（规格第 5.2 章采购与 SRM 条目的四个来源之一，PRD 第 5.6.5 节由库存侧提供判定输入）。`inventory.stock_movement.value_adjusted.v1` 的消费者固定为 `costing.stock_value_adjust`，位于 `crates/application/costing/src/consumer/stock_value_adjust.rs`，由阶段 11 交付并在 job-worker 注册，副作用为向 `costing.cost_entries` 补记只影响金额账的调整对应的成本条目（裁定 B-09）。两者的消费幂等均由 `platform_msg.inbox_consumptions(consumer, event_id)` 保证。

本阶段的事件不承载分录、不承载凭证生成，因此事件投递失败进入死信不会破坏账务一致性，只会延迟采购建议与报表刷新。这一点是有意设计：把可以异步的东西异步化，把必须同事务的东西留在同事务。
本阶段不向 `ledger.posting_trigger_event_types` 登记任何行（裁定 A-21），库存事件不独立产生凭证。凭证一律由产生该库存流水的来源业务事件在同一事务内经 `ep_contract_ledger::PostingPort::post` 生成，其存货腿金额来自本阶段过账端口的返回值。因此关账受理前提二的待过账积压统计不会把本阶段的两个事件计入。

#### 6.5 失败重试与补偿

序列化失败 40001 与死锁 40P01 由数据访问层统一重试 3 次，退避 50、150、450 毫秒（基线第 8.4 节）。库存过账在事务提交前不产生任何外部可见副作用，因此可安全重试。

本阶段不提供补偿动作。库存流水只追加不可修改不可删除（PRD 第 5.5.8 节），登记错误的纠正方式只有由来源业务事件登记对应的反向事件。库存模块不提供独立冲正入口，这是硬边界，不因任何运维诉求开放。

#### 6.6 六组必测并发场景中本阶段承担的两组

基线第 8.4 节列出六组必测并发场景，本阶段直接承担第二组同一物料的并发出库与移动加权平均单价重算，并参与第三组同一采购订单的并发发票匹配与暂估回冲（本阶段侧为并发价差拆分对未覆盖数量的争用）与第五组关账受理与在途写事务的交叠（本阶段侧为在途库存过账落入待关闭期间时的期间归属一致性）。

### 7. 配置项

三项新增配置，全部在 `EP__INVENTORY__` 前缀下，结构体开启 `deny_unknown_fields`。

| 键名 | 类型 | 默认值 | 生效方式 | 说明 |
|---|---|---|---|---|
| EP__INVENTORY__POSTING__MAX_LINES | u32 | 200 | 进程启动时加载，变更需重启 | 单次过账的明细行上限，与基线第 5.1 节批量操作上限 200 对齐；core-server 与 job-worker 各自读取；该上限属 PRD 附录乙未决事项 U-A-10 的临时取值，见第 11.2 节 |
| EP__INVENTORY__POSTING__MAX_SERIALS_PER_LINE | u32 | 1000 | 同上 | 单行序列号条数上限，防止单条明细行的序列号数组撑爆事务预算；该上限属 PRD 附录乙未决事项 U-A-10 的临时取值，见第 11.2 节 |
| EP__INVENTORY__RECON__BATCH_SIZE | u32 | 2000 | 同上 | 对账检查的分批规模，单位为仓库与物料的组合数；该取值按规格第 10.2 章由附录 A.4 认证期实测冻结，本处默认值只是认证前的初值 |

不新增允许负结存的配置项。理由是该口径属未决事项 U-G-02，把未决业务口径落成运行期可变参数会制造一个永远无人负责取值的开关；本阶段硬编码为阻断，切换代价见第 11 节。

不新增导出行数、分页、默认筛选期间三类配置，它们由基线第 11.5 节全局固定。

启动自检不新增项，本阶段也不把任何判读业务数据行的判定挂到启动路径上。基线第 7.3 节的 `rls-enabled-and-forced` 一项断言全部带法人列的表均已 ENABLE 且 FORCE 行级安全，自动覆盖本阶段新增的九张表；该项与 `runtime-role-privileges-bounded` 只读 `pg_class` 与 `pg_roles`，判读的是结构与角色而不是业务行，二者留在阻断级。与本阶段有关的另外两项按降级口径处理：`reporting-dataset-signature-matched` 不一致时关闭该数据集对应的报表入口、登记降级窗口并告警，不拒绝启动；`master-data-usage-probes-registered` 已按第 4.9 节下沉为模块启用动作的前置校验，不在启动路径上判定。本阶段九张表的结存非负、两账同源与勾稽差额一律由第 4.9 节的两个 `ReconCheck` 在 job-worker 内周期执行并生成差异事项，任何阶段不得把它们改写成启动时的闸门：这台服务器没有备节点，把业务数据判定放进启动路径等于让一条错误数据停掉八个进程。自检项一律按注册名标识，不用序号（裁定 C-25）。

### 8. 测试计划

#### 8.1 单元测试

位于 `ep-domain-inventory` 内，`#[cfg(test)]`，不触库不触网不取真实时间，时间经 `FixedClock` 注入。

计价与取价分支覆盖清单，逐条给出被测分支与断言要点。

| 编号 | 被测分支 | 断言要点 |
|---|---|---|
| U-01 | 入库 Explicit 取价 | 金额等于 round(单价乘数量, 2)，单价重算为金额除数量 round 到 6 位 |
| U-02 | 入库 ReturnAtMovingAverage 命中移动平均分支 | 结存大于 0 且单价大于 0 时按当前单价，分支记为 MOVING_AVERAGE |
| U-03 | 入库 ReturnAtMovingAverage 命中零结存分支 | 结存为 0 时逐笔按原结转单价，分支记为 ORIGINAL_DELIVERY_PRICE，多笔分配逐笔 round 再累加 |
| U-04 | 入库 ReturnAtMovingAverage 命中零单价分支 | 结存不为 0 但单价为 0 时同样走 fallback |
| U-05 | fallback 数量合计不等于行数量 | 返回 ORIGINAL_PRICE_ALLOCATION_MISMATCH |
| U-06 | 出库 MovingAverage 非出清 | 金额等于 round(单价乘数量, 2)，余额与单价按第 4.4 节更新 |
| U-07 | 出库 MovingAverage 出清归零 | 金额等于当前金额余额全额，分支记为 MOVING_AVERAGE_CLEARING，余额与单价均归零 |
| U-08 | 出库 OriginalEstimate 原额冲回 | 金额等于原暂估单价乘数量，不受当前单价影响；出清后残值保留 |
| U-09 | 出库结存不足 | 返回 INSUFFICIENT_BALANCE，余额不变 |
| U-10 | 价差拆分全在库 | on_hand 等于 matched，issued 为 0，金额账加全额差额 |
| U-11 | 价差拆分全已出库 | on_hand 为 0，issued 等于 matched，金额账不变 |
| U-12 | 价差拆分部分在库 | 按比例拆分，两部分之和恒等于总差额，尾差落在 issued |
| U-13 | 价差拆分时未覆盖为 0 | on_hand 为 0，不写 value_entry，仍写 variance_splits |
| U-14 | 未覆盖数量 clamp | 出库数量大于未覆盖数量时未覆盖归 0 而非负 |
| U-15 | 空批次归集 | 未启用批次管理的物料一律以 `'-'` 归集，与显式传入 `'-'` 等价 |
| U-16 | 序列号状态机六条迁移 | 逐条断言目标状态与错误码 |
| U-17 | 舍入边界 | 中值远离零策略，覆盖 0.005、0.015、负值中值三组 |
| U-18 | 单价为 6 位小数除不尽 | 尾差留在金额余额，不产生调整分录 |

领域属性测试（proptest），对应基线第 8.1 节要求的五组不变量中本阶段承担的三组，各生成不少于 1000 组随机操作序列。

- P-01 库存守恒：随机生成入库与出库序列，断言任意时刻结存数量等于流水代数和且非负，任意时刻数量账与金额账的 `quantity` 一致。
- P-02 移动加权平均单价重算：断言结存大于 0 时 `|金额余额 - round(结存数量 × 单价, 2)| ≤ round(结存数量 × 5e-7, 2) + 0.01`；断言移动平均路径下结存归零时金额余额为 0。
- P-03 价差拆分：随机生成匹配数量、总差额与未覆盖数量，断言 `on_hand + issued = matched`、两部分差额之和恒等于总差额、`0 ≤ uncovered ≤ 结存`、多次匹配的 `on_hand` 合计不超过初始未覆盖数量。

借贷平衡与核销守恒两组属性测试不由本阶段承担，归总账阶段与财务阶段。

#### 8.2 集成测试

位于 `crates/application/inventory/tests/`，使用真实 PostgreSQL 16，每个用例独占 `ep_test_<nanoid>` 库，用例结束删库，禁止内存库或 mock。测试数据一律经 `ep-testkit` 构造器与 `InventoryPostingDriver` 生成，禁止手写 INSERT。

| 编号 | 场景 | 对应判据 |
|---|---|---|
| I-01 | 收货暂估入库后两账同源、未覆盖数量增加 | 规格第 17.2 章必测分支一前半、第 17.3 章存货金额账与数量账一致 |
| I-02 | 发票登记价差拆分后两账仍同源一致，单价按调整后金额除结存重算 | 必测分支一后半 |
| I-03 | 一张发票跨多次收货：逐次回冲、未覆盖数量不重复占用 | 必测分支十一第一至三句 |
| I-04 | 两张发票先后匹配同一法人同一仓库同一物料：第二张不含已被第一张覆盖的数量，两张合计不超过实际在库 | 必测分支十一末两句 |
| I-05 | 发票数量少于收货数量：未匹配部分的暂估留存，库存两账仍同源 | 必测分支十 |
| I-06 | 未收票采购退货按原暂估单价原额冲回，冲回金额不随期间内单价变动 | 必测分支十二 |
| I-07 | 超量开票反向匹配收货：按发票不含税单价与匹配数量同源写两账，不产生价差记录 | 必测分支十三路径一的库存侧 |
| I-08 | 零结存销售退货：一行关联多张交付确认单，按各自原结转单价逐笔取价，回冲后单价重算，不出现零成本入库 | 必测分支十四 |
| I-09 | 出清归零：全部交付出库后金额余额与单价同时归零，随后的销售退货稳定命中零结存分支 | 第 4.6 节偏离项的正向验证 |
| I-10 | 直运交付确认与直运销售退货不产生任何库存流水 | 必测分支七与十五的库存侧 |
| I-11 | 负结存阻断：出库超结存时返回 INSUFFICIENT_BALANCE，余额与流水均无变化 | PRD 第 5.6.4 节提交时校验 |
| I-12 | 批次维度：未启用批次的物料按 `'-'` 单条归集；启用批次的物料分批出库互不串批 | 规格第 17.3 章按仓库物料批次逐项核对 |
| I-13 | 序列号全链路：收货入库、交付出库、销售退货再入库、再次发出，追溯链完整 | PRD 第 5.4.2 节与 U-G-04 临时取值 |
| I-14 | 序列号异常四条：条数不等、行内重复、出库不在库、入库已在库 | PRD 第 5.9 节异常表 |
| I-15 | 收发存汇总平衡关系：期初加收入减发出等于期末，按仓库物料批次逐项成立；金额侧期初加收入减发出加调整等于期末 | PRD 第 5.7.2 节平衡关系 |
| I-16 | 期末库存价值表法人合计等于金额账合计 | PRD 第 5.7.3 节勾稽要求的库存侧 |
| I-17 | 顺延入账：同一业务事件的库存条目与传入的会计期间一致，按原始业务日期与按会计期间两条路径均可检索到该条流水，且结果标注实际落入的会计期间 | 规格第 5.2 章子账与凭证共用同一期间归属块、必测分支顺延项的库存侧 |
| I-18 | 幂等：同一来源单据重复过账 3 次只产生一次流水，第 2、3 次返回既有结果 | PRD 第 5.9 节 |
| I-19 | 迁移期初路径：该法人已有流水时拒绝 | 第 3.1 节预留分支 |
| I-20 | 对账检查注入：注入负结存、注入两账不一致、注入金额余额为负，各自生成可追溯的对账差异事项 | 规格第 10.2 章发布验收的注入用例 |
| I-21 | 零结存残值观察项：由 ORIGINAL_ESTIMATE_PRICE 分支产生残值时不生成差异事项、不拦截关账，但可逐条追溯到来源流水 | 第 4.6 节残值口径 |
| I-22 | 探针与引用计数器：某物料有库存流水时 `InventoryMaterialUsageProbe` 返回真、无流水时返回假；`InventoryReferenceCounter` 在该物料有非零结存时返回仓库物料批次组合数、结存全部归零后返回 0 | 裁定 A-13 与 A-15 |
| I-23 | 数据集视图与仅追加登记：`inventory.v_stock_value_entries` 含三列安全列且 `ep_analyst_ro` 可读、`ep_app_rw` 不可写；五张仅追加表在 `platform_core.append_only_registry` 的登记与触发器一致 | 裁定 A-18 与 B-02 |

法人越权测试集独立成 `tests/rls_matrix` 的 inventory 子目标，覆盖基线第 8.4 节的八类：读取、写入、更新、删除、聚合、排序、报表投影与错误信息泄漏。具体做法是以法人 A 的安全上下文对法人 B 的九张表逐表发起操作，断言读取返回空集、写入被 RLS 拒绝、聚合结果不含 B 的数据、按金额排序时 B 的记录不影响 A 的位次、报表端点 A9 与 A10 的合计不含 B、错误消息不回显 B 的任何字段值。另覆盖内部对账系统安全上下文按法人逐轮遍历时每轮只写单一法人变量。该子目标属发布门禁项。

并发测试固定四组，每组不少于 200 次迭代。

- C-01 同一物料同一批次 20 线程并发出库，断言无负结存、无死锁、结存等于流水代数和、单价序列单调可解释。
- C-02 并发出库与并发价差拆分交叠，断言未覆盖数量不出现负值且不重复占用。
- C-03 同一来源单据的并发重复过账，断言只产生一次流水。
- C-04 跨多物料的乱序批量过账，断言锁顺序生效、无死锁；故意以反序提交一组用例验证排序逻辑确实起作用。

#### 8.3 端到端测试

本阶段既交付 inventory 模块的四端界面，也提供联调断言库，E2E 分两部分。本模块无写入界面，四端界面只消费第 5 节的十个只读端点。

本阶段自测部分：桌面端经 Playwright 与 tauri-driver 驱动 `clients/desktop/src/modules/inventory/`，覆盖 A1 至 A10 的十个查询页面在法人切换、字段级金额权限有无两种身份下的展示差异；移动端按规格第 6.2 章能力矩阵第 597 行库存台账与收发扫码四端取值均为完整，对 `clients/mobile/src/modules/inventory/` 执行 XCUITest 与 Espresso 各一个场景，覆盖扫码录入批次与序列号的即时校验反馈（调用 A6 与 A7）。

联调部分：规格第 8 章黄金业务闭环十四步中的第 5 步收货、第 8 步交付确认发货、第 11 步退货三步的库存侧断言，由阶段 9b 的 `testkit/scenarios/golden_loop_14_steps.rs` 执行，本阶段提供断言库 `ep-testkit::inventory_assertions`，含两账同源、守恒、勾稽三组断言函数，供该用例与后续阶段直接引用而不各写一套。

本阶段不设对外演示的里程碑。原里程碑 M5 已按总览第 5 节整项撤销，降级为本阶段的组件验收，判据写在第 9 节退出条件第 26 条。理由是本阶段按第一条硬边界不提供任何库存写端点，写入侧只能经 `ep-testkit` 的 `InventoryPostingDriver` 调用第 5.1 节的过账端口驱动，这是组件级证据而不是可演示的产品路径，不应占用一个里程碑；M5 一并消失后，不允许用测试夹具直接写库来构造前置数据这条规则恢复为无例外。读出侧一律经第 5 节十个只读端点与本阶段四端界面验证，其中端点 A6 与 A7 的扫码即时反馈由移动端真实调用。收货、交付确认发货与价差拆分三条写入路径的真实调用方分别落在阶段 7、阶段 6 与阶段 10，各自在其自身阶段与本阶段的端口一次接实并同批验收，本阶段不登记任何顺延项，也不向总览的顺延清单写入任何条目。

#### 8.4 性能相关项

基准数据集按规格附录 A.3，由 `ep-datagen --scale=default --seed=<冻结值>` 产出：法人 2 个、物料 5000 条、库存流水 50 万条、会计期间 36 个。仓库数取 6 个（每法人 3 个），这是本阶段的假设，规格附录 A.3 未给出仓库数量，理由是仓库数直接决定余额行基数与报表分组数，不取值则性能结论不可复现；取值依据是规格第 2.2 章的目标客户规模。

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
4. 13 个迁移在空库上顺序执行成功，`--check` 模式报告迁移历史版本与二进制期望版本一致；每个迁移文件带 `-- rollback:` 段并经一次实际回退演练。
5. 九张表全部 `ENABLE` 且 `FORCE` 行级安全，`ep_app_rw` 不具备 BYPASSRLS 与 SUPERUSER，启动自检 `rls-enabled-and-forced` 与 `runtime-role-privileges-bounded` 两项通过。
6. 单元测试与三组领域属性测试全绿。
7. 集成测试 I-01 至 I-23 全绿。
8. `tests/rls_matrix` 的 inventory 子目标八类全绿。
9. 并发测试 C-01 至 C-04 全绿，无死锁记录，重试次数指标有值且在阈值内。
10. 覆盖率达到第 8.5 节的五档门槛。
11. 四个性能度量项达到通过线，五个端点的 EXPLAIN 证据中无顺序扫描，证据归档到 `docs/evidence/stage-8/`。
12. 两个 `ep_platform_recon::ReconCheck` 实现已在 job-worker 的 `ReconRegistry` 注册并可按法人与会计期间执行，注入三类差异后差异事项写入 `platform_core.recon_discrepancies` 且可追溯，注入清零后校验通过。
13. 第 5 节错误码表中的 21 个错误码在 `docs/error-codes.md` 与 `ep-foundation::error::codes` 两处一致，CI 的重复码校验通过。
14. 2 个事件在 `docs/event-catalog.md` 登记，信封字段完整，缺少 `security_level` 或 `data_scope_tags` 时入队被拒绝的用例通过。
15. 本阶段新增的指标在 ops-agent 的 9101 端点可抓取，标签基数纪律通过（不含 user_id、doc_no、trace_id）。
16. 数据字典中九张表逐列登记，含第 3.4 节三项新增命名决定与其缩写词表。
17. 第 4.6 节的偏离项已在阶段交付物中单列一节，并提交基线第 3.5 节的修订建议，由平台架构负责人签署。
18. 第 11 节列出的六项未决事项的临时取值已逐项写入 `docs/pending-decisions-stage-8.md`，含切换代价估算，并与 PRD 附录乙的 U-G-01 至 U-G-07 与 U-A-10 编号对齐。
19. inventory 模块在规格第 6.2 章能力矩阵中取值为完整或简化的能力域，其四端界面已实现并通过 Playwright 与 tauri-driver 的桌面用例、XCUITest 与 Espresso 的移动用例；取值为 VIEW_ONLY 的能力域只实现只读视图；取值为 NOT_APPLICABLE 的不实现入口。
20. `inventory.v_stock_value_entries` 已发布并授予 `ep_analyst_ro`，列签名已同步给阶段 11 且与 `reporting.dataset_fields` 的登记一致。
21. 本阶段全部路由的能力域码与动作类别常量已在 `crates/contract/inventory/src/capability.rs` 声明，`xtask configdoc` 通过。
22. `InventoryMaterialUsageProbe` 已实现并注入，启用 inventory 模块的动作在探针未注册时被拒绝、在注册后可完成，该判定不进启动自检，进程启动路径不因其失败而拒绝启动。
23. 本模块的 `InventoryReferenceCounter` 已实现并注册到 `MasterReferenceCounterRegistry`，本模块不承担任何 TradeHistoryProvider。
24. 已在 `crates/contract/inventory/src/port/subledger_balance.rs` 定义 `StockValueSubledgerBalancePort`，并由 `InventorySubledgerBalanceQuery`（位于 `crates/application/inventory/src/projection/subledger_balance.rs`）实现，trait 名、方法签名与实现类型名、位置按裁定 G-01 固定，返回该法人该会计期间的存货金额账合计；本阶段不写任何注入行，注入由阶段 10 在 `apps/core-server/src/wiring/` 与 `apps/job-worker/src/wiring/` 两个目录下写入。
25. 五张仅追加表已按 `mode` 取 `APPEND_ONLY`、`mutable_columns` 取 `'{}'` 登记 `platform_core.append_only_registry`，`db/checks/append_only_consistency.sql` 经 `xtask sqlcheck` 执行通过。
26. 原里程碑 M5 降级而来的组件验收已完成：以 `InventoryPostingDriver` 各驱动一条收货暂估入库、一条交付确认出库与一条价差拆分，三条路径的两账同源、数量守恒与取价分支断言全绿；验收报告写明三者的真实调用方分别在阶段 7、阶段 6、阶段 10 接线，本阶段不登记任何顺延项、不作为对外演示节点；`apps/core-server/src/wiring/` 与 `apps/job-worker/src/wiring/` 两个目录下的全部文件中不出现任何以 Noop、Stub、Fake、Dummy 为前缀的实现类型注入行，该判据由阶段 1 随 xtask 交付的 archcheck 规则 unwired-absent 断言，出现即构建失败。

### 10. 与规格和 PRD 的对应

#### 10.1 规格条目

| 规格位置 | 本阶段实现的内容 |
|---|---|
| 第 5.2 章库存与 WMS 条目 | 仓库与库存台账的库存侧、收发存记录、可用量查询、批次与序列号标识、移动加权平均一种方法与单一成本层、出库按加权平均单价结转、两账同源同步、按仓库与物料的库存金额查询、期末库存价值表、销售退货为入库方向、采购退货为出库方向、数量账写入权归库存模块 |
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
| 第 7.7 章法人行级隔离机制 | 九张表的 RLS 策略、内部对账系统安全上下文的逐法人遍历取数 |
| 第 7.9 章派生存储安全继承 | 两个事件携带 security_level 与 data_scope_tags |
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
| 第 5.2.1 至 5.2.2 节 | 仓库作为唯一存放地点维度、跨仓库不合并单价、仓库归属单一法人；仓库档案属性由 mdm 承载，本阶段只读 |
| 第 5.2.3 节 | 仓库停用前置校验的库存侧，即 `WarehouseDeactivationCheckPort::assert_no_stock`，判定该仓库全部物料结存为零 |
| 第 5.3.1 至 5.3.3 节 | 数量账四维度、金额账三维度、批次不承载独立成本、按批次不可查金额 |
| 第 5.4.1 至 5.4.3 节 | 批次必填规则、出库批次候选、一行一批次、批次无状态；序列号非台账维度、条数校验、追溯链、扫码重复处理 |
| 第 5.5.1 节 | 出入库事件总表七行的库存侧全部实现 |
| 第 5.5.2 节 | 五个字段的通用校验与四步校验顺序 |
| 第 5.5.3 至 5.5.6 节 | 四类出入库操作的库存侧处理与输出 |
| 第 5.5.7 节 | 只影响金额账的调整记录，方向为无方向、数量为零、金额可正可负、可追溯到采购发票登记单；收发存汇总的数量列不含该记录、金额列包含 |
| 第 5.5.8 节 | 库存流水不可变性与八个携带字段 |
| 第 5.6.1 至 5.6.3 节 | 三个查询端点及其结果列、口径与排序 |
| 第 5.6.4 节 | 提交时校验、事后校验、修复路径 |
| 第 5.6.5 节 | 提供结存数量与可用量作为采购建议的判定输入 |
| 第 5.7.1 至 5.7.3 节 | 三张报表端点、勾稽要求的子账侧、跳转参数 |
| 第 5.8 节 | 法人隔离、字段级权限、唯一写入者、审计、四端取值、库存操作不属六类高风险操作 |
| 第 5.9 节 | 异常表九行逐行对应到错误码 |
| 第 5.10 节 | 四个度量项一律指向规格，本阶段不重取数值 |

### 11. 风险与预留

#### 11.1 已知技术风险

R1，出清归零偏离未获批准。第 4.6 节的偏离若不被平台架构负责人接受，则规格第 17.3 章存货金额账与数量账一致这一项在结存归零点上无法严格成立，关账将被无解除路径地拦截。缓解：偏离项作为退出条件第 17 条前置，在编码开始前完成签署；备选方案是把该项不变量的判定式改为带舍入上界的容差判定，但那需要修改规格第 17.3 章的判据，代价更大。

R2，库存腿金额与凭证腿的同事务一致性。裁定 C-28 已定死全部凭证一律与业务事件同事务生成、Outbox 只承载派生、通知、检索与报表数据集，因此不存在总账侧异步生成凭证的分支，本条风险收窄为编排用例误用：调用方若在库存腿之后另开事务写凭证，规格第 10.2 章关账受理后建立快照的时点会读不到该凭证。缓解：第 6.1 节已把 `&mut dyn Tx` 的同事务要求写入端口签名，跨事务调用在类型上不可表达；本阶段的集成测试 I-17 以传入期间为准做一致性断言，联调时若出现跨事务写入，该断言会立即失败。

R3，热点物料的行锁串行化。20 并发下若集中在少数物料上出库，`stock_value_balances` 的单行锁会把并发退化为串行，威胁 3 秒的普通交易提交通过线。缓解：C-01 并发测试直接度量该场景的 P95；若不达标，可行的优化是把金额账余额的更新推迟到语句级并使用 `UPDATE ... RETURNING` 的单语句原子更新，减少锁持有时间，但不改变一次一行的语义。不采用分片计数器，理由是移动加权平均单价必须读到全局一致的金额余额。

R4，收发存汇总与期末库存价值表的期初聚合随期间数增长。36 个期间末尾的期初聚合需要扫过接近全量的流水。缓解：`accounting_period_seq` 的范围扫描索引已就位；若认证期实测逼近 10 秒，扩展点是按期间物化一份余额快照，见第 11.3 节。

R5，`value_amount` 允许为负带来的下游影响。负存货金额会传导到总账存货科目余额与经营指标。缓解：对账检查 R2 生成差异事项，但不阻断写入；需在联调期确认财务阶段与报表阶段对负存货的展示口径。

R6，序列号唯一性范围与设备档案的冲突（U-G-07、U-J-03）。本阶段按法人内唯一实现，若阶段 12 的设备档案采用同一产品下唯一，同一序列号可能在两处各存一份。缓解：本阶段的 `serial_states` 是库存侧的唯一真相，阶段 12 的设备档案应引用而非另建，该约束作为阶段 12 的输入前提；本阶段不提供任何跨模块的序列号写入端口。

#### 11.2 未决事项的临时取值与切换代价

| 编号 | 临时取值 | 是否阻塞本阶段 | 切换代价 |
|---|---|---|---|
| U-G-01 可用量构成 | 可用量等于结存数量，`AvailabilityQueryPort::available` 与端点 A2 共用同一投影函数，`reserved_quantity` 恒为 0 | 不阻塞 | 若改为扣减已确认未发货订单数量，只需在该投影中接入 `ep-contract-sales` 的在途订单数量查询并改写 `available_quantity` 的算式，不改表结构、不改 trait 签名，估约 1 个查询文件加 3 个测试用例 |
| U-G-02 是否允许负结存 | 一律硬阻断，不提供配置 | 不阻塞 | 若改为可配置，需在物料或仓库档案上加一个开关字段（归 mdm）、在出库路径加一个分支、去掉 `ck_stock_qty_balances_non_negative`、新增 4 个测试用例；去掉数据库 CHECK 属收紧变更的逆操作，可在线执行 |
| U-G-03 批次号与序列号的长度字符集 | 长度上限 64、字符集 `[A-Za-z0-9._-]`、批次号手工录入、唯一性范围为法人加仓库加物料 | 不阻塞 | 放宽长度属基线第 7.4 章在线变更范围，改 CHECK 即可；收紧字符集需回填校验 |
| U-G-04 序列号状态语义 | 两状态 IN_STOCK 与 SHIPPED，退货入库后可再次发出且允许换仓入库 | 不阻塞 | 若增加已退回等第三状态，需扩 CHECK 取值与状态机守卫，约 1 个文件加 6 个用例 |
| U-G-06 关账是否固化快照 | 不固化，已关闭期间按实时聚合取数 | 不阻塞 | 若改为固化，新增一张快照表与一个 job-worker 任务，A9 与 A10 的响应结构不变，扩展点见第 11.3 节 |
| U-A-10 单据明细行数与序列号条数上限 | 单次过账明细行上限取 200，见第 7 节 `EP__INVENTORY__POSTING__MAX_LINES`，与基线第 5.1 节批量操作上限一致；单行序列号条数上限取 1000，见第 7 节 `EP__INVENTORY__POSTING__MAX_SERIALS_PER_LINE`，该值在规格与基线中均无出处，是本阶段按基线第 10.3 节事务预算估算的技术侧临时取值；单次连续扫码条数不单独设限，连续扫码为逐次调用端点 A6 与 A7，单次批量校验的条数由第 5 节 A7 的 `filter[serial_no]=in:` 上限 200 个约束 | 不阻塞 | 改两个配置项的默认值并重启即可，不改表结构、不改 API 契约、不改端口签名；上调任一取值须重跑第 8.2 节并发测试 C-01 与第 8.4 节普通交易提交的 P95 度量项，不达标时按第 11.1 节 R3 收窄取值而不拆事务；`EP__INVENTORY__POSTING__MAX_LINES` 上调超过 200 会突破基线第 5.1 节批量操作上限，须同步提出基线修订，不得只在本阶段偏离 |

U-A-04 数量单价金额的小数位与舍入、U-G-05 空批次标识两项已由基线第 3.5 节与第 11.4 节定死，本阶段直接照用，不再另行取值。

#### 11.3 为后续阶段预留的扩展点

E1，期间余额快照。A9 与 A10 的取数集中在 `ep-app-inventory/src/projection/period_aggregation.rs` 一个文件内，通过一个 `PeriodAggregationSource` trait 取数。当前实现为实时聚合，若 U-G-06 决策为固化快照或 R4 的性能不达标，只需新增一个快照实现并在装配处替换，API 契约与响应结构不变。

E2，可用量的预留扣减。见 U-G-01 的切换代价，扩展点为 `AvailabilityCalculator` trait 的单一实现点。

E3，多计价方法。当前 `PricingBranch` 与计价服务是按移动加权平均单一方法写死的，但两账分离、流水仅追加、余额独立三项结构本身与计价方法无关。若后续引入先进先出，扩展点是在 `stock_value_balances` 之外新增成本层表并把计价服务改为 trait，`stock_qty_entries` 与 `stock_movements` 不需要改动。本阶段不预埋任何多方法的空壳代码。

E4，期初导入通道。`source_doc_type` 的 `MIGRATION_STOCK_ADJUSTMENT` 与 `reason` 的 `MIGRATION_OPENING` 已预留并实现入库路径，按裁定 A-24 本阶段是库存期初导入的唯一落点，首版不设独立的数据迁移阶段，后续无需新增枚举取值（新增取值需要改 CHECK，属停机窗口内的收紧变更）。规格第 7.10 章要求的迁移库存调整单据以该来源类型承载，其总账侧由阶段 9a 的期初余额批次承担，应收应付预收预付与资金账户期初归阶段 10。

E5，成本归集查询的取数接口。规格第 5.2 章成本归集与销货成本结转条目的存货类成本来源是交付确认时从库存金额账结转的销货成本，成本阶段需要按交付确认单行回查结转金额与单价。本阶段的 `ep_contract_inventory::InventoryPricingLookupPort::original_unit_price_by_source_line` 端口与 `stock_value_entries` 上的 `source_doc_line_id` 索引即为该接口，成本阶段直接引用，不需要本阶段再改动。来源单据行的口径按裁定 A-09 固定为 `sales.delivery_confirmation_lines`，`SourceDocType::DELIVERY_CONFIRMATION` 由 ep-app-sales 在调用库存腿时传入，本阶段不自行判定来源类型。

E6，断言库复用。`ep-testkit::inventory_assertions` 提供两账同源、数量守恒、存货勾稽三组断言函数，闭环联调阶段与恢复演练（规格附录 A.5、A.6 要求恢复后执行第 17.3 章全部强制不变量校验）直接引用同一实现，避免恢复验收另写一套判据。
