> **F-57 状态：`HISTORICAL_DO_NOT_EXECUTE`。** 本文只保留历史任务正文，现行工作由 F-57 **Task 21** 承接。当前权威集合为 [F-57 总体设计](../../specs/2026-08-23-f57-governed-automation-fabric-design.md)、[F-57 需求追踪矩阵](../../reviews/2026-08-23-f57-requirements-traceability.md)、[F-57 权威替代登记](../../reviews/2026-08-23-f57-authority-supersession-register.md)与 [F-57 实施计划](../2026-08-23-f57-governed-automation-fabric-implementation.md)；F-57 是设计/计划权威，不是产品已实现声明，本地模型实现延期。

## 阶段 12 售后、项目与设备

本阶段实现 PRD 第 9 节的全部五类对象（设备档案与保修、客户投诉记录、售后工单、退换修登记、项目与项目任务），并实现规格第 8 章第 12 步要求的客户 360 聚合读取入口。本阶段的硬边界是不生成任何总账凭证、不写库存数量账与库存金额账、不产生成本归集，全部账务与库存后果由本阶段关联的销售退货单在其所属模块承接。

本计划严格照做共享技术基线。基线已给出取值的一律引用不重取；本阶段补充并已回写共享技术基线的规则集中登记在第 13 节；本阶段其余已冻结实现规则集中登记在第 14 节。PRD 附录乙事项均已关闭，第 12 节登记首版冻结值与未来变更影响，全文不保留实施期选择。
本阶段与 T0 贯通线的关系。整卷阶段顺序固定为 1 → 2 → 3a → 4 → 3b-1 → T0 → 5 → 9a → 8 → 6 → 7 → 10 → 11 → 9b → 14，阶段 3b-2 不在这条链上，本阶段在阶段 10 之后与阶段 11 并行，阶段 13 在本阶段之后与阶段 9b 并行。T0 是在阶段 3b-1 结束后、阶段 5 全量开工之前插入的一条最薄贯通线，切片取自阶段 5、6、9a、10、11，判据是一条合同从建单走到管理层看到一个数。本阶段不向 T0 贡献任何切片，理由是设备、投诉、工单、退换修与项目五类对象都不在那条最薄闭环上，规格第 8 章第 12 步本身也排在第 11 步之后。本阶段整体在 T0 已贯通的骨架上加厚：开工时客户档案、产品、合同、销售订单、交付确认单、销售退货命令端口、销项发票与采购需求入口都已真实存在并被真实调用过，因此本阶段不为任何跨模块调用注入替身，不设任何顺延验收项，也不承担任何首次贯通判据。闭环第 12 步只交付用例片段，其串接由阶段 9b 在全分支闭环时执行。

### 0. 范围与不做的事

本阶段做的事：

1. 设备档案与保修信息的三条建档路径、在保状态判定、设备当前状态的字典化与终止状态确认。
2. 客户投诉记录的登记、受理、关闭、取消，以及升级为工单。
3. 售后工单的三个创建入口、六状态状态机、关联对象一致性校验、处理记录追加、时限提醒的触发登记。
4. 退换修登记行的三类处理方式、与销售退货单和发货侧单据的挂接、由对方终态事件回写、三条追溯链路双向可达。
5. 项目与项目任务的字段与状态机、合同生效派生项目任务的幂等消费、由项目任务提交采购需求的双向引用。
6. 客户 360 聚合端点与区块提供者契约的扩充，本阶段自实现投诉、设备、工单三个区块。按裁定 C-09，唯一端点 GET /api/v1/crm/customers/{id}/customer-360 与唯一契约 ep_contract_crm::Customer360SectionProvider 由阶段 5 建立，本阶段只追加区块取值与区块实现，不新增路径，不保留 /overview。
7. F-10 合同终止目录中的 `CLM_TERM_PROJECT_TASK` 规则：未开始任务自动取消，在制任务进入人工决策；作为第七个真实 `ImpactRule` 注册后完成终止处置机制的全量接线。本阶段不新增合同终止事件消费者。

本阶段明确不做的事，取值按 PRD 9.1.2 与规格第 5.2 章售后与 FSM、项目与 PPM 两个条目：现场派工与调度、服务权益与服务合同计费、售后知识库、独立服务 SLA 引擎、点检计划、维修工单、备件、设备成本、可靠性分析、预测维护、保修索赔与费用结算与延展销售、WBS 多层分解、资源与产能、工时填报、项目预算、项目风险、变更管理、挣值、工单成本归集。本阶段也不定义销售退货单、交付确认单、合同、采购需求、客户档案本身。

---

### 1. 交付物清单

本阶段结束时下列东西可运行、可验证。

| 序号 | 交付物 | 形态 | 判定方式 |
|---|---|---|---|
| D-01 | ep-contract-service、ep-domain-service、ep-app-service 三个 crate | 编译通过并被 core-server 与 job-worker 装配 | cargo build 与依赖自检脚本通过 |
| D-02 | ep-contract-project、ep-domain-project、ep-app-project 三个 crate，含 `ContractTerminationProjectTaskImpactRule` | 同上 | 同上，且 `ImpactRegistry` 终态注册数为 7 |
| D-03 | 对阶段 5 已建立的 ep-contract-crm 客户 360 区块契约的扩充与 ep-app-crm 聚合用例的三个新区块 | 编译通过并被 core-server 装配 | 同上 |
| D-04 | service schema 的 16 张表与 project schema 的 6 张表 | project/service 建表迁移与 procure/costing 两个项目外键晚绑定迁移可离线执行并可回退 | ADR-0013 冻结的 ep-migrate 自建 Runner 在空库上顺序执行成功，16 个迁移版本统一写入单一 `schema_history`，且 `--check` 模式下启动自检项 `rls-enabled-and-forced` 通过；三个迁移撤销更正事实随各自 owner 根表文件同建，不占新版本号 |
| D-05 | 售后侧 37 个 HTTP 端点、项目侧 16 个 HTTP 端点、客户 360 的 1 个端点 | core-server 暴露于 /api/v1/service、/api/v1/project、/api/v1/crm | 端点级集成测试全绿 |
| D-06 | 25 个领域事件的发布与 3 个消费者 | Outbox 写入与 job-worker 消费 | 重复投递不少于 3 次的幂等测试通过 |
| D-07 | 四张受控取值字典的出厂数据与配置发布通道接入 | 迁移回填 + 配置发布包 | 字典改动经配置发布通道签名发布后生效，且不触发 DDL |
| D-08 | 工单时限提醒的定时器登记与站内通知送达 | 经 ep-platform-flow 定时器与 ep-platform-notify | 两类提醒的端到端测试通过 |
| D-09 | tests/rls_matrix 中本阶段 22 张带法人表的越权矩阵用例 | 独立测试目标 | 八类越权面全部返回 404 或 403，无内容回显 |
| D-10 | 闭环第 12 步的用例片段与三条追溯链路双向可达用例 | 前者为 testkit/scenarios/stage12_service_step12.rs 中的步骤函数与断言，供阶段 9b 的 testkit/scenarios/golden_loop_14_steps.rs 在第 12 步引用；后者为 apps/core-server/tests/ 下的 E2E | 两者在本阶段各自跑通全绿，整条链路的串接通过由阶段 9b 的该用例判定 |
| D-11 | 边界不变量用例 | 执行本阶段全部用例前后规格第 17.3 章三项取值不变，且凭证与库存流水四张表的行数与校验和不变 | 前者由 ep-platform-recon 语句集比对，差额为零；后者用例前后各取一次数直接比对 |
| D-12 | docs/event-catalog.md、docs/error-codes.md、docs/data-dictionary.md 三处登记，其中数据字典含本阶段五个单据类型码 EQ、CPL、WO、PRJ、PT | 文档 | CI 一致性校验通过，且 xtask configdoc --check-doc-type-codes 通过 |
| D-13 | project.v_projects_dataset 受治理数据集视图，dataset code 为 project_projects，grain 为 DOCUMENT | db/migrations/project/ 下的视图迁移 | 视图已发布并授予 ep_analyst_ro，列签名与 reporting.dataset_fields 的登记一致 |
| D-14 | service 与 project 两个模块的四端界面 | clients/desktop/src/modules/service/、clients/desktop/src/modules/project/ 与 clients/mobile/src/modules/ 下的同名目录 | 四端 UI 用例全绿 |
| D-15 | 本阶段全部路由的能力域码与动作类别常量 | crates/contract/service/src/capability.rs 与 crates/contract/project/src/capability.rs | xtask configdoc 通过 |
| D-16 | ServiceReferenceCounter | crates/application/service/src/probe/master_reference.rs 与两个 wiring 目录中的注册行 | 阶段 5 的档案停用引用计数在 service 模块上有计数 |
| D-17 | F-10 项目任务影响面处置接线 | `crates/application/project/src/impact/contract_termination_project_task.rs` 与 `apps/job-worker/src/wiring/impact.rs` 的真实注册行 | 七类合同终止处置项全量闭合及正反向 E2E 通过；无第二个终止消费者、无替身规则 |

---

### 2. crate 与进程归属

#### 2.1 新增 crate

| crate | 目录 | 层 | 依赖 | 装配进程 |
|---|---|---|---|---|
| ep-contract-service | crates/contract/service | 契约 | ep-foundation | core-server、job-worker |
| ep-domain-service | crates/domain/service | 领域 | ep-foundation、ep-contract-service | core-server、job-worker |
| ep-app-service | crates/application/service | 应用 | ep-foundation、ep-platform-（authz、flow、audit、outbox、sequence、notify、file、obs）、ep-domain-service、ep-contract-（service、sales、clm、mdm、crm、inventory） | core-server、job-worker |
| ep-contract-project | crates/contract/project | 契约 | ep-foundation | core-server、job-worker |
| ep-domain-project | crates/domain/project | 领域 | ep-foundation、ep-contract-project | core-server、job-worker |
| ep-app-project | crates/application/project | 应用 | ep-foundation、ep-platform-*（含 impact）、ep-domain-project、ep-contract-（project、clm、procure、mdm） | core-server、job-worker |

ep-contract-service 对外只暴露 ReturnRepairTraceQuery 一个 trait。按裁定 B-06 撤销 EquipmentQuery，不建 crates/contract/service/src/port/equipment.rs，设备的跨模块可见性只保留三条路径，见 5.1 节末段。ep-contract-project 对外不暴露任何命令 trait，按裁定 C-19 撤销 ProjectTaskDerivationPort，合同派生一律由本阶段消费事件后自行派生。本阶段依赖的跨模块 trait 及提供方固定为：ep_contract_clm::ContractDerivationPlanQuery（阶段 6）、ep_contract_sales::SalesOrderLineDeliveryQuery、`SalesReturnCommandPort` 与 `SalesExchangeLinkCommandPort`（阶段 6）、ep_contract_procure::PurchaseRequisitionIntakePort（阶段 7）、ep_contract_mdm::MasterDataLookup 与 `ClassificationItemQuery`（阶段 5），以及本阶段为 F-51 U-J-03 在既有 ep-contract-inventory 中追加并在 ep-app-inventory 实现的 `SerialStateQuery`。`ClassificationItemQuery::assert_active(&mut dyn Tx, ctx, "RETURN_REASON", return_reason_code)` 是退换登记行写入退货原因的唯一跨 schema 校验入口。`SalesOrderLineDeliveryQuery::delivered_quantity(&mut dyn Tx, ctx, sales_order_line_id)` 返回订单、客户、合同、品项与累计交付数量，只用于工单关联一致性和登记数量的事务内前置守卫；销售退货的可退数量仍由 sales 在写入时权威复核。`SerialStateQuery` 的唯一 ABI 在阶段 8 第 5.1 节：`resolve_by_id` 接收 `Id<SerialState>`，`resolve_by_serial_no` 接收 `&SerialNo`，两者均返回含 id、serial_no、material_id、warehouse_id、batch_no、status 的 `SerialStateView`；不存在、不可见和跨法人统一 `PLATFORM.AUTHZ.NOT_FOUND_OR_DENIED`，不提供创建或修改方法，库存仍是唯一写入者。`SalesExchangeLinkCommandPort::link_exchange(&mut dyn Tx, ctx, LinkSalesExchange)` 的命令固定含 sales_return_line_id、replacement_delivery_schedule_id、idempotency_key，在同一事务写 sales.exchange_links，并由 sales 校验同法人、同原订单、同客户、同产品与两侧一对一；本阶段不得只在 service 留一份配对。

#### 2.2 改动的既有 crate

| crate | 改动 |
|---|---|
| ep-contract-crm | 扩充阶段 5 已建立的 crm 契约：在既有 Customer360SectionProvider 上追加 Customer360SectionKind 的 Complaints、Equipments、WorkOrders 三个取值与配套的 Customer360Item 字段，不新增 trait，不新增端点 |
| ep-app-crm | 新增 usecase/query_customer_360.rs，做区块扇出、超时降级与合并 |
| ep-contract-inventory | 在 `src/port/serial_state.rs` 追加只读 `SerialStateQuery` 与 `SerialStateView`，供设备、工单与扫码输入解析；不追加写命令 |
| ep-app-inventory | 在 `src/projection/serial_state_query.rs` 实现 `SerialStateQuery`，只读 `inventory.serial_states` 并受法人 RLS；由 core-server 注入 ep-app-service |
| ep-adapter-db-pg | 新增 repository/service/ 与 repository/project/ 两个目录，各仓储只访问自己模块的 schema |
| ep-adapter-search | 本阶段不改动本 crate，原记在本行的检索文档投影职责按裁定 F-05 移出：五类对象的 foundation::port::search::SearchDocument 投影函数落在 ep-app-service 与 ep-app-project，各自置于 src/projection/search_document.rs，object_type 取表全名如 service.equipment_records，由 job-worker 的索引消费者调用后经 SearchIndexPort 写入；ep-adapter-search 本体与该消费者按裁定 A-07 由阶段 3b 交付。落点依据为裁定 F-05 通则甲一与阶段 3 计划第 18 项「本阶段不交付任何业务对象的检索文档投影函数，投影由各业务阶段按 SearchDocument 结构提供」 |
| apps/core-server/src/wiring/ | 注册两个模块的仓储与用例，注册三个客户 360 区块提供者，并把 ServiceReferenceCounter 注册进阶段 5 提供的 MasterReferenceCounterRegistry |
| apps/job-worker/src/wiring/ | 注册三个 Outbox 消费者 project.contract_derivation、project.requisition_intake 与 service.return_repair_writeback、一个定时器回调；把 `ContractTerminationProjectTaskImpactRule` 注册进平台 `ImpactRegistry`，把 ServiceReferenceCounter 注册进 MasterReferenceCounterRegistry。不得为合同终止另建项目消费者 |
| ep-testkit | 新增 EquipmentRecordBuilder、WorkOrderBuilder、ComplaintBuilder、ProjectBuilder、ProjectTaskBuilder、ContractDerivationPlanFake、SalesReturnPortFake，后两者分别按裁定 A-16 与 A-17 冻结的签名实现；另新增 testkit/scenarios/stage12_service_step12.rs，内含闭环第 12 步的步骤函数与断言，由阶段 9b 的 testkit/scenarios/golden_loop_14_steps.rs 引用 |
| ep-datagen | 基准数据集追加设备 5000 台、工单 20000 张、投诉 5000 条、项目 200 个、项目任务 4000 条 |

#### 2.3 进程归属

- core-server：全部交互式命令与查询、客户 360 聚合、退换修登记行提交时对 ep-contract-sales 已交付数量查询的同步调用；销售退货单的创建命令不在 core-server 发起，见 4.6。
- job-worker：三个 Outbox 消费者，即合同生效派生项目任务的 project.contract_derivation、项目任务提交采购需求的 project.requisition_intake、退换修登记行挂接与回写的 service.return_repair_writeback；工单时限提醒定时器的回调执行；检索索引传播事件的发布方；向既有 `ImpactRegistry` 注册 `CLM_TERM_PROJECT_TASK` 的真实规则。`clm.contract.terminated.v1` 仍只有平台的 `platform.impact_assess` 一个消费者，本阶段规则由 `ImpactAssessor` 调用，不形成第四个消费者。
- 本阶段不新增进程，不使用 integration-gateway、plugin-host、portal-gateway。本阶段对象不进入供应商门户的受控能力 API。

#### 2.4 依赖方向自检

本阶段新增的依赖全部落在基线第 1.3 节允许的方向内，评审时逐条核对下列五项：ep-domain-service 与 ep-domain-project 不出现 sqlx、reqwest、tokio 的 IO 模块、std::fs、std::net、SystemTime::now、rand；ep-app-service 不依赖 ep-app-sales、ep-app-clm 或 ep-app-inventory，跨模块一律经对方 contract 的 trait；ep-app-project 只依赖 `ep-platform-impact` 的 `ImpactRule` 契约，不读写 platform_core 的影响面表；ep-app-inventory 只实现 `ep-contract-inventory` 中的只读查询，不依赖 service；ep-adapter-db-pg 中 service 仓储只出现 service.* 表名，project 仓储只出现 project.* 表名，inventory 的序列号读取只出现在 inventory 仓储文件，由 CI 的 SQL 静态检查断言。

---

### 3. 数据库变更

#### 3.1 迁移文件与执行顺序

迁移执行顺序由 ADR-0013 冻结的 ep-migrate 自建 Runner 按文件版本号全序排定，本阶段十六个常规迁移文件的范围与顺序见下表：project 目录五个、在 project.projects 建成后立即执行的 procure/costing 晚绑定外键文件各一个、service 目录九个；全部版本统一写入单一 `platform_core.schema_history`。本阶段不新增 schema，也不存在第二套顺序声明或迁移历史。单目标且目标已先建的跨 schema 引用按全局外键规则使用 `(legal_entity_id, ref_id) -> target(legal_entity_id,id) ON DELETE RESTRICT`；因此 `project_task_purchase_requisition_links.purchase_requisition_id` 在建表文件内联指向阶段 7 已建的 `procure.purchase_requisitions`。应用端仍须通过目标模块 `ep-contract-*` 端口校验目标的可引用状态与业务范围，外键只承担存在性和法人一致性的最终兜底。service/project 仓储的业务 SQL 不直接读取或改写其他 schema；跨模块查询与命令仍只落目标模块仓储。按裁定 A-06 本阶段不实现或注册 ReconCheck，第 8.5 节只运行其他阶段已注册校验项。

| 顺序 | 文件 | 内容 |
|---|---|---|
| 1 | db/migrations/project/V20261021090000__project_create_projects.sql | 建 `project.projects` 与仅追加 `project.project_migration_corrections`，含复合 FK、shape、RLS、APPEND_ONLY 登记与 guard；跨 tasks 最终效果图留到 090100 安装 |
| 2 | db/migrations/procure/V20261021090030__procure_add_project_foreign_keys.sql | 为 purchase_requisitions.project_id、purchase_order_lines.project_id 追补指向 project.projects 的同法人复合外键 |
| 3 | db/migrations/costing/V20261021090040__costing_add_project_foreign_keys.sql | 为 cost_entries.project_id、revenue_entries.project_id 追补指向 project.projects 的同法人复合外键 |
| 4 | db/migrations/project/V20261021090100__project_create_project_tasks.sql | 建 `project.project_tasks`，并在 tasks 已存在后安装 project migration correction 的 DEFERRABLE 最终效果图 |
| 5 | db/migrations/project/V20261021090200__project_create_task_requisition_links.sql | 建 project.project_task_purchase_requisition_links，内联采购需求复合外键、双向唯一键，以及任务状态与链接行基数双表同构的延迟约束触发器 |
| 6 | db/migrations/project/V20261021090300__project_create_attachment_links.sql | 建 project.project_attachments、project.project_task_attachments |
| 7 | db/migrations/project/V20261021090400__project_create_dataset_views.sql | 建 project.v_projects_dataset 并授予 ep_analyst_ro，按裁定 A-18 |
| 8 | db/migrations/service/V20261021090500__service_create_dictionaries.sql | 建 service.equipment_statuses、service.work_order_types、service.complaint_channels、service.work_order_priorities |
| 9 | db/migrations/service/V20261021090600__service_create_equipment_records.sql | 建 `service.equipment_records` 与仅追加 `service.equipment_migration_corrections`，含复合 FK、字典终态延迟图、RLS、APPEND_ONLY 登记与 guard |
| 10 | db/migrations/service/V20261021090700__service_create_customer_complaints.sql | 建 `service.customer_complaints` 与仅追加 `service.customer_complaint_migration_corrections`，含复合 FK、延迟最终效果图、RLS、APPEND_ONLY 登记与 guard |
| 11 | db/migrations/service/V20261021090800__service_create_work_orders.sql | 建 service.work_orders |
| 12 | db/migrations/service/V20261021090900__service_create_work_order_lines.sql | 建 service.work_order_lines |
| 13 | db/migrations/service/V20261021091000__service_create_work_order_logs.sql | 建 service.work_order_logs；登记 `APPEND_ONLY, mutable_columns={}` 并调用统一 `attach_table_guards` |
| 14 | db/migrations/service/V20261021091100__service_create_reminder_policies.sql | 建 service.work_order_reminder_policies |
| 15 | db/migrations/service/V20261021091200__service_create_attachment_links.sql | 建四张附件关联表 |
| 16 | db/migrations/service/V20261021091300__service_backfill_seed_dictionaries.sql | 回填四张字典的出厂取值，按法人逐个写入，created_by 取 foundation::SYSTEM_PRINCIPAL_ID |

每个文件头部带 `-- rollback:` 段。前十五个文件的回退为对应的 drop table、drop view、drop policy 或 drop constraint，属可安全逆向；第十六个回填文件的回退为按 code 删除出厂行，若该行已被业务数据引用则拒绝回退并注明只能用升级前备份回退。全部文件按基线第 3.9 节固定 `SET lock_timeout = '5s'` 与 `SET statement_timeout = '30min'`。本阶段只对新建空表创建索引，统一使用普通 `CREATE INDEX` 并由 ep-migrate 按“一个常规迁移文件一个事务”执行；不使用 `CREATE INDEX CONCURRENTLY`，也不把常规文件伪装成非事务迁移。未来若对存量大表追加在线索引，必须另放 ADR-0013 指定的 `concurrent/` 路径。

#### 3.2 公共列与统一约束

22 张表全部带法人、安全属性与创建证据。十八张可变表带基线第 4 节九个公共列，顺序为 id、legal_entity_id、security_level、data_scope_tags、row_version、created_at、created_by、updated_at、updated_by；`project.project_migration_corrections`、`service.equipment_migration_corrections`、`service.customer_complaint_migration_corrections` 与 `service.work_order_logs` 四张仅追加表只带前述 id、legal_entity_id、security_level、data_scope_tags、created_at、created_by 六列，不带 row_version、updated_at、updated_by。前三张是每个 owner 根最多一条的迁移撤销更正事实，不带 `reverses_id`；其原 APPLY 归属由 Stage 14 writer receipt 与同 id R0 审计绑定。`service.work_order_logs` 只有在 `entry_kind=CORRECTION` 时才带非空 `reverses_id` 指向同一工单内的真实父记录，普通动作行不得填写。

22 张表全部带 legal_entity_id，因此全部按基线第 3.8 节模板生成 RLS：`enable row level security`、`force row level security`、一条 `rls_<table>_le` 策略，using 与 with check 均为 `legal_entity_id = nullif(current_setting('app.legal_entity_id', true), '')::uuid`。策略由迁移生成器产出，本阶段不写变体。

22 张法人表还必须各自建立 `UNIQUE(legal_entity_id,id)` 候选键。单目标引用无论同 schema 或跨 schema 均建立真实 `ON DELETE RESTRICT` 外键：法人级目标固定使用 `(legal_entity_id,<ref>) -> target(legal_entity_id,id)`；公共 `created_by/updated_by` 及 owner、assignee、accepted、confirmed 等业务用户列固定指向 `platform_authz.user_legal_entity_grants(legal_entity_id,user_id)`；附件对象固定指向 `platform_file.attachment_objects(legal_entity_id,id)`。应用契约继续校验目标业务状态，但不得替代引用完整性。下文仅把带 `object_type/id` 的封闭多态称为逻辑引用；本阶段没有把固定单目标列列入例外。

全部 text 列带 CHECK 长度约束，取值按基线第 11.2 节：编码 64、名称 200、简述 500、备注与原因与说明 2000、保修条款文本 1 MB。全部枚举列为 text 加 CHECK，取值为大写 snake_case。全部时间列为 timestamptz，日期列为 date。数量列为 numeric(18,6)。本阶段没有金额列。

#### 3.3 project schema 逐表定义

project.projects（单据类）

| 列 | 类型 | 可空 | 默认 | 说明 |
|---|---|---|---|---|
| 公共列九列 | — | — | — | 按基线第 4 节 |
| doc_no | text | 否 | 无 | 项目编号，格式 PRJ-<法人码>-<YYYYMM>-<6 位流水> |
| status | text | 否 | 'IN_PROGRESS' | CHECK 取值 IN_PROGRESS、COMPLETED、CLOSED |
| name | text | 否 | 无 | 项目名称，长度 ≤ 200 |
| customer_id | uuid | 是 | 无 | 与法人组成复合外键指向 `mdm.customers(legal_entity_id,id)` |
| source_contract_id | uuid | 是 | 无 | 与法人组成复合外键指向 `clm.contracts(legal_entity_id,id)` |
| project_group_contract_id | uuid | 是 | 无 | 合同续签链的根合同；与法人组成复合外键指向 `clm.contracts(legal_entity_id,id)`，取值见 4.7 |
| owner_user_id | uuid | 否 | 无 | 与法人组成复合外键指向 `platform_authz.user_legal_entity_grants(legal_entity_id,user_id)` |
| planned_start_on | date | 是 | 无 | 计划开始日期 |
| planned_finish_on | date | 是 | 无 | 计划完成日期 |
| description | text | 是 | 无 | 说明，长度 ≤ 2000 |
| completed_at | timestamptz | 是 | 无 | 流转到 COMPLETED 的时点 |
| closed_at | timestamptz | 是 | 无 | 流转到 CLOSED 的时点 |

约束：ck_projects_status；ck_projects_name_len；ck_projects_description_len；ck_projects_plan_range 为 `planned_finish_on is null or planned_start_on is null or planned_finish_on >= planned_start_on`。
索引：pk_projects；ux_projects_legal_entity_id_doc_no；ix_projects_legal_entity_id_created_at；ux_projects_le_group_contract 建于 (legal_entity_id, project_group_contract_id)，用于保证一条合同续签链只有一个项目，手工项目该列为 NULL 因而互不冲突；ix_projects_legal_entity_id_customer_id；ix_projects_legal_entity_id_owner_user_id。

project.project_migration_corrections（仅追加的迁移撤销更正事实）

| 列 | 类型 | 可空 | 默认 | 说明 |
|---|---|---|---|---|
| id、legal_entity_id、security_level、data_scope_tags、created_at、created_by | — | — | — | 仅追加六列；不带 row_version、updated_at、updated_by |
| project_id | uuid | 否 | 无 | 与法人组成真实复合外键指向 `project.projects(legal_entity_id,id)` |
| correction_mode | text | 否 | 无 | 只取 `CLOSE`、`RETAIN_CLOSED` |
| status_before | text | 否 | 无 | CLOSE 时只取 IN_PROGRESS、COMPLETED；RETAIN_CLOSED 时固定 CLOSED |
| status_after | text | 否 | 无 | 固定 CLOSED |
| root_row_version_before | bigint | 否 | 无 | owner 命令锁根后读取的版本，必须大于零 |
| root_row_version_after | bigint | 否 | 无 | CLOSE 时精确为 before+1；RETAIN_CLOSED 时等于 before |
| reason | text | 否 | 无 | 固定 `DATA_MIGRATION_REVERSED`，不接受自由文本 |

约束固定为：`UNIQUE(legal_entity_id,id)`、`UNIQUE(legal_entity_id,project_id)`；`(legal_entity_id,project_id) -> project.projects(legal_entity_id,id) ON DELETE RESTRICT DEFERRABLE INITIALLY DEFERRED`；`ck_project_migration_corrections_shape` 只允许 `(CLOSE, IN_PROGRESS|COMPLETED, CLOSED, after=before+1)` 或 `(RETAIN_CLOSED, CLOSED, CLOSED, after=before)` 两种 NULL-safe 形状，并要求两个版本均正数、reason 精确等于固定值。待 tasks 表已由 090100 建成后，同一 `V20261021090100` 安装 `project.assert_project_migration_correction_consistent()`，只在 correction 表的 INSERT/UPDATE/DELETE 上附着 `CONSTRAINT TRIGGER DEFERRABLE INITIALLY DEFERRED`：提交点按 `(legal_entity_id,project_id)` 锁项目，强制根的 status/row_version/security_level/data_scope_tags 逐列等于 correction 的 after 值和安全属性，并强制本项目全部任务已处于 COMPLETED/CANCELLED。触发器只证明该次更正提交的最终效果，不附着项目根，故后续唯一合法的 contract-derivation 恢复分支不会被历史事实错误阻断。

本表随 `V20261021090000` 建立；同文件先向 `platform_core.append_only_registry` 插入 `('project','project_migration_corrections','APPEND_ONLY','{}')` 再调用 `attach_table_guards`，运行账号 UPDATE/DELETE 必须失败。090100 rollback 先删 correction constraint trigger/function 再 DROP tasks；090000 rollback 再按 detach guard→删除 registry 行→DROP correction 表→DROP projects 的顺序执行。非空表回退仍只允许开发/空库或先恢复升级前备份，不得删历史事实来迁就 down。

project.project_tasks（单据类）

| 列 | 类型 | 可空 | 默认 | 说明 |
|---|---|---|---|---|
| 公共列九列 | — | — | — | — |
| doc_no | text | 否 | 无 | 任务编号，格式 PT-<法人码>-<YYYYMM>-<6 位流水> |
| status | text | 否 | 'NOT_STARTED' | CHECK 取值 NOT_STARTED、IN_PROGRESS、COMPLETED、CANCELLED |
| project_id | uuid | 否 | 无 | 同 schema 真实外键，fk_project_tasks_projects，ON DELETE RESTRICT |
| name | text | 否 | 无 | 任务名称，≤ 200 |
| source | text | 否 | 无 | CHECK 取值 CONTRACT_DERIVED、MANUAL |
| source_contract_id | uuid | 是 | 无 | 与法人组成复合外键指向 `clm.contracts(legal_entity_id,id)` |
| source_contract_version_no | integer | 是 | 无 | 与 `source_contract_id` 及法人组成复合外键指向 `clm.contract_versions(legal_entity_id,contract_id,version_no)`；MANUAL 时两列均空 |
| derivation_unique_key | text | 是 | 无 | 取值为 ep_contract_clm::ContractDerivationItem 的 unique_key，格式按裁定 A-16 为 <contract_id>:<contract_version_no>:<item_kind>:<source_contract_line_id 或 milestone_no>，≤ 200 |
| derivation_obligation_key | text | 是 | 无 | 同一合同版本链内稳定的义务键，取自 ContractDerivationItem.obligation_key，≤ 200 |
| derivation_obligation_hash | text | 是 | 无 | CLM 对义务业务字段的 RFC 8785/SHA-256 摘要，64 位小写十六进制 |
| derivation_batch_no | integer | 是 | 无 | 取值为 ContractDerivationPlan 的 derivation_batch_no，只用于追溯 |
| derivation_stale | boolean | 否 | false | 来源义务被删除或改变时置 true；不得据此自动改任务状态 |
| derivation_stale_at | timestamptz | 是 | 无 | 首次置 stale 的时点，之后不覆盖 |
| assignee_user_id | uuid | 是 | 无 | 与法人组成复合外键指向 `platform_authz.user_legal_entity_grants(legal_entity_id,user_id)` |
| planned_start_on | date | 是 | 无 | — |
| planned_finish_on | date | 是 | 无 | — |
| actual_finish_on | date | 是 | 无 | 流转到 COMPLETED 时按中国标准时间自然日写入 |
| requisition_link_state | text | 是 | 无 | 未提交时为空；提交采购需求后取 PENDING、LINKED、FAILED |
| requisition_material_id | uuid | 是 | 无 | 首次提交采购需求时冻结；与法人组成复合外键指向 mdm.materials(legal_entity_id,id)，ON DELETE RESTRICT |
| requisition_quantity | numeric(18,6) | 是 | 无 | 首次提交采购需求时冻结，必须大于零 |
| requisition_required_on | date | 是 | 无 | 首次提交采购需求时冻结 |
| requisition_link_last_error | text | 是 | 无 | 仅 FAILED 时保存清洗后的最终错误，≤ 2000；不得保存堆栈、SQL 或敏感字段 |
| description | text | 是 | 无 | ≤ 2000 |
| cancel_reason | text | 是 | 无 | ≤ 2000 |

约束：ck_project_tasks_status；ck_project_tasks_source；`ck_project_tasks_derived_fields` 强制 CONTRACT_DERIVED 行的 source_contract_id、source_contract_version_no、derivation_unique_key、derivation_obligation_key、derivation_obligation_hash、derivation_batch_no 全部非空且 batch_no>0，MANUAL 行这六列全部为空且 derivation_stale=false；两个键长度 1..200；hash 匹配 `^[0-9a-f]{64}$`；`ck_project_tasks_stale_at` 强制 derivation_stale 与 derivation_stale_at 同空同非空；`ck_project_tasks_requisition_link_state` 强制状态只取 PENDING、LINKED、FAILED，状态为空时三个申请快照和 last_error 全空，状态非空时 requisition_material_id/quantity/required_on 全非空且 quantity>0；只有 FAILED 允许并要求非空 `requisition_link_last_error`，PENDING/LINKED 时错误为空；ck_project_tasks_plan_range；ck_project_tasks_finish_when_completed；ck_project_tasks_cancel_reason。
索引：pk_project_tasks；ux_project_tasks_legal_entity_id_doc_no；ix_project_tasks_legal_entity_id_created_at；ux_project_tasks_le_derivation_unique_key 建于 (legal_entity_id, derivation_unique_key)，是新版本补充任务幂等兜底；`ix_project_tasks_le_contract_obligation` 建于 `(legal_entity_id, source_contract_id, derivation_obligation_key, source_contract_version_no desc)`，用于版本比较；ix_project_tasks_legal_entity_id_source_contract_id；ix_project_tasks_project_id_status；ix_project_tasks_legal_entity_id_assignee_user_id_planned_finish_on。手工任务的派生键为空，普通唯一索引允许其共存。

project.project_task_purchase_requisition_links（关联表）

| 列 | 类型 | 可空 | 默认 | 说明 |
|---|---|---|---|---|
| 公共列九列 | — | — | — | — |
| project_task_id | uuid | 否 | 无 | 同 schema 外键，ON DELETE RESTRICT |
| purchase_requisition_id | uuid | 否 | 无 | 由 `PurchaseRequisitionIntakePort` 回执写入；与法人组成复合外键指向 `procure.purchase_requisitions(legal_entity_id,id)`，ON DELETE RESTRICT |
| purchase_requisition_doc_no | text | 是 | 无 | 冗余展示用，≤ 64 |
| requested_at | timestamptz | 否 | now() | 提交时点 |

索引：pk；`ux_task_requisition_links_le_task` 建于 `(legal_entity_id, project_task_id)`，保证一个项目任务首版最多关联一张采购需求；`ux_task_requisition_links_le_requisition` 建于 `(legal_entity_id, purchase_requisition_id)`，保证一张采购需求最多来自一个项目任务；ix_task_requisition_links_legal_entity_id_created_at。旧的仅对 `project_task_id` 建普通索引口径作废，由前述唯一索引覆盖。

`V20261021090200__project_create_task_requisition_links.sql` 必须在 `project.project_tasks` 与 `project.project_task_purchase_requisition_links` 两表各安装一个 `DEFERRABLE INITIALLY DEFERRED` 约束触发器，二者调用同一断言函数并覆盖 INSERT/UPDATE/DELETE。函数在提交前按 `(legal_entity_id,project_task_id)` 稳定锁定任务并用锁后新语句快照计数，强制 `requisition_link_state='LINKED'` 当且仅当恰有一条 link；`NULL|PENDING|FAILED` 当且仅当没有 link。不存在任务、超过一条 link、链接行与状态任一方向不一致都拒绝提交；延迟到提交点是为了允许消费者在同一事务内以任意顺序插 link 与置 LINKED。普通应用路径仍先锁任务，触发器只作 direct-SQL 与并发写偏差的最终兜底。

project.project_attachments、project.project_task_attachments：按基线第 4 节附件关联表定义，列为公共列加 owner_id、attachment_object_id、purpose text、sort_no integer。`(legal_entity_id,owner_id)` 建指向各自 owner 表的同法人复合外键，`(legal_entity_id,attachment_object_id)` 建指向 `platform_file.attachment_objects(legal_entity_id,id)` 的真实复合外键。ux 建于 `(legal_entity_id,owner_id,attachment_object_id)`。

#### 3.4 service schema 逐表定义

service.equipment_statuses、service.work_order_types、service.complaint_channels、service.work_order_priorities（档案类字典，四张表结构相同）

| 列 | 类型 | 可空 | 默认 | 说明 |
|---|---|---|---|---|
| 公共列九列 | — | — | — | — |
| code | text | 否 | 无 | ≤ 64 |
| name | text | 否 | 无 | ≤ 200 |
| sort_no | integer | 否 | 0 | 列表排序 |
| is_active | boolean | 否 | true | 停用不影响历史引用 |
| deactivated_at | timestamptz | 是 | 无 | — |
| is_terminal | boolean | 否 | false | 只在 equipment_statuses 上存在，true 表示终止状态 |

索引：pk；ux_<table>_legal_entity_id_code 建于 (legal_entity_id, code)；ix_<table>_legal_entity_id_created_at。业务表上的取值列按 3.1 的口径建复合外键指向该唯一键。四张表按 F-51 U-A-07 播种：`equipment_statuses` 为 IN_STOCK、IN_SERVICE、UNDER_REPAIR（`is_terminal=false`）及 SCRAPPED、RETURNED（`is_terminal=true`）；`work_order_types` 为 INSTALL、REPAIR、CONSULT、COMPLAINT_FOLLOWUP；`complaint_channels` 为 PHONE、EMAIL、ONSITE、SALES_RELAY；`work_order_priorities` 为 LOW、NORMAL、HIGH、URGENT。四张表均允许经签名配置包新增、修改显示名、调整排序与停用；编码一旦被业务记录引用即不可修改或物理删除，只能停用，历史记录继续显示。新增设备状态必须显式声明 is_terminal；其余三表不含该列。

service.equipment_records（档案类）

| 列 | 类型 | 可空 | 默认 | 说明 |
|---|---|---|---|---|
| 公共列九列 | — | — | — | data_scope_tags 写入 customer:<客户编码> |
| code | text | 否 | 无 | 设备编号，格式 EQ-<法人码>-<YYYYMM>-<6 位流水>，生成后不可修改 |
| is_active | boolean | 否 | true | 首版恒为 true，见第 14 节 F-05 |
| deactivated_at | timestamptz | 是 | 无 | — |
| inventory_serial_state_id | uuid | 是 | 无 | 与法人组成复合外键指向 `inventory.serial_states(legal_entity_id,id)`；不在 service 保存第二份序列号 |
| model | text | 否 | 无 | 型号，≤ 200 |
| customer_id | uuid | 否 | 无 | 与法人组成复合外键指向 `mdm.customers(legal_entity_id,id)` |
| product_id | uuid | 是 | 无 | 与法人组成复合外键指向 `mdm.products(legal_entity_id,id)` |
| batch_no | text | 否 | '-' | 未启用批次时取 '-'，按基线第 11.4 节 |
| sales_order_line_id | uuid | 是 | 无 | 与法人组成复合外键指向 `sales.sales_order_lines(legal_entity_id,id)` |
| delivery_confirmation_id | uuid | 是 | 无 | 路径一写入，只读；与法人组成复合外键指向 `sales.delivery_confirmations(legal_entity_id,id)` |
| delivery_confirmation_line_id | uuid | 是 | 无 | 路径一写入，建档去重用；与法人、`delivery_confirmation_id` 组成复合外键指向 `sales.delivery_confirmation_lines(legal_entity_id,delivery_confirmation_id,id)` |
| source_delivery_unit_no | integer | 是 | 无 | 路径一在交付行内从 1 起逐台编号，和交付行组成幂等键 |
| delivered_on | date | 是 | 无 | 交付日期 |
| installed_on | date | 是 | 无 | 安装日期 |
| current_status_code | text | 否 | 无 | 通过复合外键引用本 schema 字典的 `(legal_entity_id, code)`，见下 |
| warranty_start_on | date | 是 | 无 | — |
| warranty_end_on | date | 是 | 无 | — |
| warranty_scope | text | 是 | 无 | ≤ 500 |
| warranty_terms | text | 是 | 无 | ≤ 1 MB |
| remark | text | 是 | 无 | ≤ 2000 |
| source | text | 否 | 无 | CHECK 取值 DELIVERY_CONFIRMATION、MANUAL、MIGRATION |
| migration_batch_no | text | 是 | 无 | ≤ 64，规格第 7.10 章迁移批次标识 |

约束：ck_equipment_records_source；`ck_equipment_records_delivery_source` 强制 source=DELIVERY_CONFIRMATION 时 delivery_confirmation_id、delivery_confirmation_line_id、source_delivery_unit_no 全非空且 unit_no>0，其他来源的 unit_no 为空；ck_equipment_records_install_after_delivery；ck_equipment_records_warranty_range；ck_equipment_records_batch_no_len；ck_equipment_records_migration_source。交付日期不得晚于登记时点自然日不落 CHECK，改由 Clock 端口判定。
current_status_code 建复合外键 fk_equipment_records_equipment_statuses，指向 service.equipment_statuses 的 (legal_entity_id, code) 唯一键并 ON DELETE RESTRICT。原先不建外键的理由是配置停用会与业务表更新绑死，该理由不成立：规格第 5.6 章的停用只停界面入口、写入接口、定时任务与对外事件，字典行只允许停用不允许删除，全程无 DDL 也无 delete，外键不参与其中任何一步且永不触发。应用层只保留启用状态判定与可读错误码，不再承担存在性校验；孤儿取值由外键在写入瞬间挡住，周期性孤儿取值核对整条不设。work_orders 的 work_order_type_code、customer_complaints 的 channel_code 与 work_order_reminder_policies 的 work_order_type_code 三处同 schema 字典引用同此处理。
索引：pk_equipment_records；ux_equipment_records_legal_entity_id_code；ix_equipment_records_legal_entity_id_created_at；ix_equipment_records_legal_entity_id_customer_id；`ux_equipment_records_le_inventory_serial_state` 建于 `(legal_entity_id, inventory_serial_state_id)`；`ux_equipment_records_le_delivery_unit` 建于 `(legal_entity_id, delivery_confirmation_line_id, source_delivery_unit_no)`，使按交付行逐台重放不重复；ix_equipment_records_legal_entity_id_current_status_code。序列号本体的法人内全局唯一由 `inventory.ux_serial_states_le_serial_no` 强制，service 只保存其 id。手工设备没有可引用的库存序列号时该列为空并使用设备 code，首版不建立第二套设备序列号命名空间。

service.equipment_migration_corrections（仅追加的迁移撤销更正事实）

| 列 | 类型 | 可空 | 默认 | 说明 |
|---|---|---|---|---|
| id、legal_entity_id、security_level、data_scope_tags、created_at、created_by | — | — | — | 仅追加六列；不带 row_version、updated_at、updated_by |
| equipment_record_id | uuid | 否 | 无 | 与法人组成真实复合外键指向 `service.equipment_records(legal_entity_id,id)` |
| correction_mode | text | 否 | 无 | 只取 `SET_RETURNED`、`RETAIN_TERMINAL` |
| status_before_code | text | 否 | 无 | 与法人组成复合外键指向 `equipment_statuses(legal_entity_id,code)` |
| status_after_code | text | 否 | 无 | 与法人组成同一字典 FK；SET_RETURNED 时固定 RETURNED，RETAIN_TERMINAL 时等于 before |
| root_row_version_before | bigint | 否 | 无 | owner 命令锁根后读取的版本，必须大于零 |
| root_row_version_after | bigint | 否 | 无 | SET_RETURNED 时精确为 before+1；RETAIN_TERMINAL 时等于 before |
| reason | text | 否 | 无 | 固定 `DATA_MIGRATION_REVERSED` |

约束固定为：`UNIQUE(legal_entity_id,id)`、`UNIQUE(legal_entity_id,equipment_record_id)`；根 FK 与两条状态字典 FK 均 `ON DELETE RESTRICT DEFERRABLE INITIALLY DEFERRED`；CHECK 强制两个状态码长度 1..64、两个版本正数、reason 固定，并只允许 `(SET_RETURNED,status_after_code='RETURNED',after=before+1)` 或 `(RETAIN_TERMINAL,status_after_code=status_before_code,after=before)`。同文件安装 `service.assert_equipment_migration_correction_consistent()` 并只在 correction 表 INSERT/UPDATE/DELETE 上附着 `CONSTRAINT TRIGGER DEFERRABLE INITIALLY DEFERRED`：提交点锁设备根与两个字典行，SET_RETURNED 要求 before 字典 `is_terminal=false` 且 after 的 RETURNED 行 `is_terminal=true`，RETAIN_TERMINAL 要求 before/after 同一字典行且 `is_terminal=true`；两态都强制设备根当前状态、row_version、security_level、data_scope_tags 等于 correction after-image。触发器不附着设备根，不会阻断今后经合法状态入口的独立变更。

本表与根同由 `V20261021090600` 建立，先登记 `APPEND_ONLY, mutable_columns={}` 再 attach guard。rollback 固定为 detach guard→删 registry→删约束触发器/函数→DROP correction→DROP equipment_records；运行账号 UPDATE/DELETE、错法人、错终态字典或伪造根 after-image 必须由数据库拒绝。

service.customer_complaints（单据类）

| 列 | 类型 | 可空 | 默认 | 说明 |
|---|---|---|---|---|
| 公共列九列 | — | — | — | — |
| doc_no | text | 否 | 无 | 投诉编号，格式 CPL-<法人码>-<YYYYMM>-<6 位流水> |
| status | text | 否 | 'REGISTERED' | CHECK 取值 REGISTERED、PROCESSING、CLOSED、CANCELLED |
| customer_id | uuid | 否 | 无 | 与法人组成复合外键指向 `mdm.customers(legal_entity_id,id)` |
| contact_name | text | 是 | 无 | ≤ 200 |
| contact_info_enc | bytea | 是 | 无 | 联系方式密文，字段级信封加密，见 3.6 |
| contact_info_key_ref | text | 是 | 无 | 密钥引用与版本，≤ 200 |
| complaint_on | date | 否 | 无 | 投诉日期，不得晚于登记时点自然日，应用层判定 |
| channel_code | text | 是 | 无 | 引用 complaint_channels |
| content | text | 否 | 无 | 投诉内容，≤ 2000 |
| contract_id | uuid | 是 | 无 | 与法人组成复合外键指向 `clm.contracts(legal_entity_id,id)` |
| sales_order_line_id | uuid | 是 | 无 | 与法人组成复合外键指向 `sales.sales_order_lines(legal_entity_id,id)` |
| product_id | uuid | 是 | 无 | 与法人组成复合外键指向 `mdm.products(legal_entity_id,id)` |
| equipment_record_id | uuid | 是 | 无 | 同 schema 外键 fk_customer_complaints_equipment_records |
| accepted_by | uuid | 是 | 无 | 受理人；与法人组成复合外键指向 `platform_authz.user_legal_entity_grants(legal_entity_id,user_id)` |
| accepted_at | timestamptz | 是 | 无 | — |
| handling_note | text | 是 | 无 | 处理说明，≤ 2000 |
| closed_at | timestamptz | 是 | 无 | — |
| cancel_reason | text | 是 | 无 | ≤ 2000 |

关联工单不在本表存列，理由见 4.3。
约束：ck_customer_complaints_status；ck_customer_complaints_accept 为 `status <> 'PROCESSING' or accepted_by is not null`；ck_customer_complaints_close 为 `status <> 'CLOSED' or handling_note is not null`；ck_customer_complaints_cancel 为 `status <> 'CANCELLED' or cancel_reason is not null`；各 text 列长度约束。
索引：pk；ux_customer_complaints_legal_entity_id_doc_no；ix_customer_complaints_legal_entity_id_created_at；ix_customer_complaints_legal_entity_id_customer_id_complaint_on；ix_customer_complaints_legal_entity_id_status。

service.customer_complaint_migration_corrections（仅追加的迁移撤销更正事实）

| 列 | 类型 | 可空 | 默认 | 说明 |
|---|---|---|---|---|
| id、legal_entity_id、security_level、data_scope_tags、created_at、created_by | — | — | — | 仅追加六列；不带 row_version、updated_at、updated_by |
| complaint_id | uuid | 否 | 无 | 与法人组成真实复合外键指向 `service.customer_complaints(legal_entity_id,id)` |
| correction_mode | text | 否 | 无 | 只取 `CANCEL`、`RETAIN_TERMINAL` |
| status_before | text | 否 | 无 | CANCEL 时只取 REGISTERED、PROCESSING；RETAIN_TERMINAL 时只取 CLOSED、CANCELLED |
| status_after | text | 否 | 无 | CANCEL 时固定 CANCELLED；RETAIN_TERMINAL 时等于 before |
| root_row_version_before | bigint | 否 | 无 | owner 命令锁根后读取的版本，必须大于零 |
| root_row_version_after | bigint | 否 | 无 | CANCEL 时精确为 before+1；RETAIN_TERMINAL 时等于 before |
| reason | text | 否 | 无 | 固定 `DATA_MIGRATION_REVERSED` |

约束固定为：`UNIQUE(legal_entity_id,id)`、`UNIQUE(legal_entity_id,complaint_id)`；根复合 FK `ON DELETE RESTRICT DEFERRABLE INITIALLY DEFERRED`；`ck_customer_complaint_migration_corrections_shape` 只允许 `(CANCEL,REGISTERED|PROCESSING,CANCELLED,after=before+1)` 或 `(RETAIN_TERMINAL,CLOSED|CANCELLED,status_after=status_before,after=before)`，并要求版本正数与固定 reason。同文件安装 `service.assert_customer_complaint_migration_correction_consistent()`，只在 correction 表 INSERT/UPDATE/DELETE 上附着 `CONSTRAINT TRIGGER DEFERRABLE INITIALLY DEFERRED`：提交点锁投诉根，逐列核实根当前 status/row_version/security_level/data_scope_tags 等于 correction after-image。该触发器不附着投诉根，普通后续读取不需重放历史图。

本表与根同由 `V20261021090700` 建立，先登记 `APPEND_ONLY, mutable_columns={}` 再 attach guard。rollback 固定为 detach guard→删 registry→删约束触发器/函数→DROP correction→DROP customer_complaints；运行账号 UPDATE/DELETE、错法人、错终态或伪造根 after-image 必须由数据库拒绝。

service.work_orders（单据类）

| 列 | 类型 | 可空 | 默认 | 说明 |
|---|---|---|---|---|
| 公共列九列 | — | — | — | — |
| doc_no | text | 否 | 无 | 工单编号，格式 WO-<法人码>-<YYYYMM>-<6 位流水> |
| status | text | 否 | 'DRAFT' | CHECK 取值 DRAFT、PENDING_ACCEPTANCE、IN_PROGRESS、PENDING_CUSTOMER_CONFIRM、COMPLETED、CANCELLED |
| work_order_type_code | text | 否 | 无 | 引用 work_order_types |
| priority_code | text | 否 | 'NORMAL' | 引用 work_order_priorities |
| customer_id | uuid | 否 | 无 | 与法人组成复合外键指向 `mdm.customers(legal_entity_id,id)` |
| contact_name | text | 是 | 无 | ≤ 200 |
| contact_info_enc | bytea | 是 | 无 | 字段级信封加密 |
| contact_info_key_ref | text | 是 | 无 | — |
| source_complaint_id | uuid | 是 | 无 | 同 schema 外键 fk_work_orders_customer_complaints |
| sales_order_id | uuid | 是 | 无 | 与法人组成复合外键指向 `sales.sales_orders(legal_entity_id,id)` |
| sales_order_line_id | uuid | 是 | 无 | 与法人、`sales_order_id` 组成复合外键指向 `sales.sales_order_lines(legal_entity_id,sales_order_id,id)`；两列同空或同非空 |
| contract_id | uuid | 是 | 无 | 与法人组成复合外键指向 `clm.contracts(legal_entity_id,id)` |
| product_id | uuid | 是 | 无 | 与法人组成复合外键指向 `mdm.products(legal_entity_id,id)` |
| batch_no | text | 否 | '-' | — |
| inventory_serial_state_id | uuid | 是 | 无 | 与法人组成复合外键指向库存权威 `inventory.serial_states(legal_entity_id,id)`；`SerialStateQuery` 继续校验可用状态 |
| equipment_record_id | uuid | 是 | 无 | 同 schema 外键 fk_work_orders_equipment_records |
| follow_up_of_work_order_id | uuid | 是 | 无 | 仅“创建返修跟进”写入，指向一张 COMPLETED 原工单；普通新建与 CANCELLED 后新建均为空 |
| warranty_status | text | 否 | 'NO_WARRANTY_INFO' | CHECK 取值 IN_WARRANTY、WARRANTY_NOT_STARTED、WARRANTY_EXPIRED、NO_WARRANTY_INFO，创建时快照写入，只读 |
| warranty_judged_on | date | 否 | 无 | 在保判定日期快照，只读 |
| problem_description | text | 否 | 无 | ≤ 2000 |
| expected_finish_on | date | 是 | 无 | 不得早于创建时点自然日，应用层判定 |
| assignee_user_id | uuid | 是 | 无 | 处理人；与法人组成复合外键指向用户法人授权 |
| terminal_equipment_confirmed_by | uuid | 是 | 无 | 选用终止状态设备的确认人；与法人组成复合外键指向用户法人授权 |
| terminal_equipment_confirmed_at | timestamptz | 是 | 无 | — |
| submitted_at | timestamptz | 是 | 无 | 进入 PENDING_ACCEPTANCE 的时点，时限提醒的计时起点 |
| accepted_at | timestamptz | 是 | 无 | 进入 IN_PROGRESS 的时点 |
| conclusion_note | text | 是 | 无 | 处理结论说明，≤ 2000 |
| completed_at | timestamptz | 是 | 无 | — |
| cancel_reason | text | 是 | 无 | ≤ 2000 |

约束：ck_work_orders_status；ck_work_orders_warranty_status；ck_work_orders_assignee；ck_work_orders_conclusion；ck_work_orders_cancel；`ck_work_orders_not_self_follow_up` 强制 follow_up_of_work_order_id 与 id 不同。跟进来源必须为 COMPLETED 属跨行状态守卫，由创建返修跟进用例在锁定原工单后校验；数据库外键保证来源存在且同法人。
索引：pk；ux_work_orders_legal_entity_id_doc_no；ix_work_orders_legal_entity_id_created_at；ux_work_orders_le_source_complaint；ix_work_orders_legal_entity_id_customer_id_created_at；ix_work_orders_legal_entity_id_equipment_record_id；ix_work_orders_legal_entity_id_inventory_serial_state_id；ix_work_orders_legal_entity_id_follow_up_of_work_order_id；ix_work_orders_legal_entity_id_sales_order_line_id；ix_work_orders_legal_entity_id_assignee_user_id_status；ix_work_orders_legal_entity_id_status_submitted_at。priority_code、work_order_type_code 均建复合外键至各自字典。

service.work_order_lines（明细行表，承载 PRD 9.6 的退换修登记行）

| 列 | 类型 | 可空 | 默认 | 说明 |
|---|---|---|---|---|
| 公共列九列 | — | — | — | — |
| work_order_id | uuid | 否 | 无 | 同 schema 外键，ON DELETE RESTRICT |
| line_no | integer | 否 | 无 | 登记行号，工单内连续 |
| handling_method | text | 否 | 无 | CHECK 取值 RETURN、EXCHANGE、REPAIR |
| status | text | 否 | 'PENDING' | CHECK 取值 PENDING、LINKED、COMPLETED、VOIDED |
| product_id | uuid | 是 | 无 | 与法人组成复合外键指向 `mdm.products(legal_entity_id,id)` |
| batch_no | text | 否 | '-' | — |
| inventory_serial_state_id | uuid | 是 | 无 | 与法人组成复合外键指向 `inventory.serial_states(legal_entity_id,id)`，不存序列号文本 |
| equipment_record_id | uuid | 是 | 无 | 同 schema 外键 |
| quantity | numeric(18,6) | 否 | 无 | CHECK > 0 |
| sales_order_line_id | uuid | 是 | 无 | 退货与换货必填；与法人组成复合外键指向 `sales.sales_order_lines(legal_entity_id,id)` |
| return_reason_object_type | text | 否 | 生成列 | 固定生成为 `RETURN_REASON`，不接受客户端输入 |
| return_reason_code | text | 是 | 无 | RETURN/EXCHANGE 必填；与法人及生成常量组成复合外键指向 `mdm.classification_items(legal_entity_id,object_type,code)`；查询契约继续校验启用状态 |
| reason_note | text | 是 | 无 | 登记原因说明，≤ 2000 |
| return_posting_date | date | 是 | 无 | RETURN/EXCHANGE 必填；作为销售退货记账日期逐字传入 `CreateSalesReturn.posting_date` |
| return_warehouse_id | uuid | 是 | 无 | 与法人组成复合外键指向 `mdm.warehouses(legal_entity_id,id)`；RETURN/EXCHANGE 按销售行形态填写并逐字传入 `CreateSalesReturn.return_warehouse_id` |
| sales_return_id | uuid | 是 | 无 | 退货侧单据；与法人组成复合外键指向 `sales.sales_returns(legal_entity_id,id)` |
| sales_return_line_id | uuid | 是 | 无 | 与法人、`sales_return_id` 组成复合外键指向 `sales.sales_return_lines(legal_entity_id,sales_return_id,id)` |
| replacement_delivery_schedule_id | uuid | 是 | 无 | EXCHANGE 的替换发货行；与法人组成复合外键指向 `sales.delivery_schedules(legal_entity_id,id)`；`SalesExchangeLinkCommandPort` 继续校验业务范围 |
| sales_return_terminal_at | timestamptz | 是 | 无 | sales_return_line 所属退货单 CLOSED 时写入 |
| replacement_terminal_at | timestamptz | 是 | 无 | 对应 delivery_schedule 经 sales.delivery.confirmed.v1 确认完成时写入 |
| repair_result_note | text | 是 | 无 | ≤ 2000 |
| repair_finished_on | date | 是 | 无 | — |
| void_reason | text | 是 | 无 | ≤ 2000 |

约束：ck_work_order_lines_handling_method；ck_work_order_lines_status；ck_work_order_lines_quantity_positive；ck_work_order_lines_order_line_required；`ck_work_order_lines_return_input` 强制 RETURN/EXCHANGE 的 return_reason_code、return_posting_date 非空，REPAIR 的 return_reason_code、return_posting_date、return_warehouse_id 全空；`ck_work_order_lines_repair_no_external` 强制 REPAIR 的 sales_return_id、sales_return_line_id、replacement_delivery_schedule_id 与两侧终态时间全为空；`ck_work_order_lines_exchange_pair` 强制 EXCHANGE 在 LINKED/COMPLETED 时 sales_return_id、sales_return_line_id、replacement_delivery_schedule_id 三列全部非空；`ck_work_order_lines_sales_return_pair` 强制 sales_return_id 与 sales_return_line_id 同空同非空；`ck_work_order_lines_return_no_replacement` 强制 RETURN 的 replacement_delivery_schedule_id/replacement_terminal_at 为空；`ck_work_order_lines_complete_terminal` 强制 COMPLETED 时 REPAIR 已有 repair_result_note/repair_finished_on，RETURN 已有 sales_return_terminal_at，EXCHANGE 两个 terminal_at 均非空；ck_work_order_lines_void_reason。配对列的非空与归属由 CHECK 加复合外键强制，客户与产品相同由领域守卫和 sales 查询契约强制。
索引：pk；ux_work_order_lines_work_order_id_line_no；ix_work_order_lines_legal_entity_id_created_at；ix_work_order_lines_legal_entity_id_sales_return_id；ix_work_order_lines_legal_entity_id_replacement_delivery_schedule_id；ix_work_order_lines_legal_entity_id_sales_order_line_id；ix_work_order_lines_legal_entity_id_equipment_record_id；ix_work_order_lines_legal_entity_id_inventory_serial_state_id；ix_work_order_lines_legal_entity_id_status_handling_method。

service.work_order_logs（仅追加表）

| 列 | 类型 | 可空 | 默认 | 说明 |
|---|---|---|---|---|
| id、legal_entity_id、security_level、data_scope_tags、created_at、created_by | — | — | — | 仅追加表不带 row_version、updated_at、updated_by |
| work_order_id | uuid | 否 | 无 | 同 schema 外键 |
| entry_kind | text | 否 | 'ACTION' | CHECK 取值 ACTION、CORRECTION |
| reverses_id | uuid | 是 | 无 | CORRECTION 必填，指向同法人、同工单内被本条更正的直接父记录；ACTION 必空 |
| action_note | text | 否 | 无 | 处理动作说明，≤ 2000 |

约束与索引：`UNIQUE(legal_entity_id,id)` 与 `UNIQUE(legal_entity_id,work_order_id,id)` 提供候选键；`(legal_entity_id,work_order_id)` 真实复合外键指向 `service.work_orders(legal_entity_id,id)`；`(legal_entity_id,work_order_id,reverses_id)` 真实复合自外键指向本表 `(legal_entity_id,work_order_id,id) ON DELETE RESTRICT`；NULL-safe `ck_work_order_logs_correction_shape` 强制 ACTION/空父与 CORRECTION/非空父两种互斥形状。延迟约束触发器在提交前拒绝自指、环与跨链挂接；本表没有数量或金额效果，故不存在累计上限。索引：pk；ix_work_order_logs_work_order_id_created_at；ix_work_order_logs_legal_entity_id_created_at。展示顺序按 created_at、id，不设行号，理由是行号需要额外的串行化点而 created_at 与 UUIDv7 的 id 已可给出稳定全序。

同一建表迁移必须把 `service.work_order_logs` 登记到 `platform_core.append_only_registry`，`mode='APPEND_ONLY'`、`mutable_columns='{}'`，随后调用阶段 2 的统一 `platform_core.attach_table_guards('service','work_order_logs')`；运行账号的任意 UPDATE/DELETE 因此由数据库拒绝，应用 SQL 静态检查只作第二道门禁。rollback 顺序固定为先调用 `detach_table_guards`、删除本表的 registry 行，再删除表及其其余对象，不能留下悬空登记或先 DROP 导致回退失败。

service.work_order_reminder_policies（档案类配置）

| 列 | 类型 | 可空 | 默认 | 说明 |
|---|---|---|---|---|
| 公共列九列 | — | — | — | — |
| code | text | 否 | 无 | ≤ 64 |
| name | text | 否 | 无 | ≤ 200 |
| work_order_type_code | text | 是 | 无 | 为空表示适用全部类型 |
| pending_acceptance_threshold_minutes | integer | 否 | 无 | CHECK 在 5 与 20160 之间 |
| due_ahead_minutes | integer | 否 | 无 | CHECK 在 0 与 20160 之间 |
| is_active | boolean | 否 | true | — |
| deactivated_at | timestamptz | 是 | 无 | — |

索引：pk；ux_work_order_reminder_policies_legal_entity_id_code；ix_..._legal_entity_id_created_at；ux_work_order_reminder_policies_le_type 建于 (legal_entity_id, work_order_type_code)，保证一个类型最多一条生效策略。本表经配置发布通道发布。

service.equipment_attachments、service.customer_complaint_attachments、service.work_order_attachments、service.work_order_line_attachments：结构同 project 侧附件关联表；四个关系名与总数据字典及 Stage 14 静态迁移投影逐字一致，禁止再使用旧名 `equipment_record_attachments` 或 `work_order_log_attachments`。

#### 3.5 普通设备状态变更历史不另建表

PRD 9.3.4 要求普通状态变更记录变更前后取值、操作者、时间与原因并写入审计。本阶段不建通用设备状态历史表，普通 change-status 仍在同一事务写 `platform_audit.audit_events`，object_type 取 `service.equipment_records`，before 与 after 携带 current_status_code，reason 携带原因说明；详情页历史经 ep-platform-audit 查询。`equipment_migration_corrections` 不是第二套普通状态历史：它只承载 Stage 14 已批准的历史导入撤销，一台设备最多一行、固定两种 shape、固定原因、APPEND_ONLY，并须与该次 R0/writer receipt 同事务绑定；普通接口不能写入或查询它作为状态历史。二者语义与权限不重叠。

#### 3.6 联系方式的字段级加密

规格第 12.3 章把联系方式列为行内敏感字段。投诉与工单上的联系方式按字段级信封加密存储于 `contact_info_enc bytea`，密钥经 `ep_foundation::port::kms::KmsBackend` 在该法人密钥域下取用（该端口 trait 定义在 ep-foundation，其实现落 ep-adapter-kms，本阶段只作为调用方，不命名任何实现类型），`contact_info_key_ref` 记录密钥标识与版本。该列不参与过滤、排序、聚合、唯一约束与全文检索，检索文档投影中该字段以掩码写入。日志、错误消息与指标标签中一律不出现该字段，Rust 侧用 foundation::Redacted 包装。含该字段的列表导出按规格第 12.1 章敏感数据导出执行重新认证与审批，由平台导出能力承担，本阶段只声明字段敏感标记与密级。

#### 3.7 受治理数据集视图

按裁定 A-18，本阶段发布一个受治理数据集视图 project.v_projects_dataset，dataset code 为 project_projects，grain 为 DOCUMENT，由 db/migrations/project/V20261021090400__project_create_dataset_views.sql 建立。视图取数为 project.projects，必须包含 legal_entity_id、security_level、data_scope_tags 三列，另含 id、doc_no、status、name、customer_id、source_contract_id、project_group_contract_id、owner_user_id、planned_start_on、planned_finish_on、completed_at、closed_at、created_at。同一迁移内执行 GRANT SELECT ON project.v_projects_dataset TO ep_analyst_ro，不授予 ep_app_rw 之外的任何写权限。视图的列名与类型签名必须与 reporting.dataset_fields 的登记一致，由阶段 11 的启动自检项 reporting-dataset-signature-matched 判定。该自检项为降级级，任何取值下都不阻断任何进程启动：本视图尚未发布或签名不符时，关闭以 project_projects 为来源的报表入口，经阶段 2 已交付的 DegradationLedger 开一个降级窗口并持续告警，视图发布且签名一致后关窗。本阶段不设由降级放行转为强制的时点。本阶段不为 service schema 发布任何数据集视图，售后侧对外取数仍走 5.1 至 5.3 的端点与全文检索文档。

---

### 4. 领域模型与关键算法

#### 4.1 核心类型

ep-domain-service 的 model 目录一个聚合一个文件：EquipmentRecord、CustomerComplaint、WorkOrder（含 WorkOrderLine 与 WorkOrderLog 两个内部实体）。value 目录：WarrantyWindow、WarrantyStatus、HandlingMethod、WorkOrderStatus、ComplaintStatus、LineStatus、EquipmentStatusCode、WorkOrderPriorityCode、BatchNo、InventorySerialStateRef；扫码入参仍可用 SerialNo 值对象校验字符集，但持久化实体只保存 InventorySerialStateRef。rule 目录：warranty.rs、line_quantity.rs、work_order_guard.rs。port 目录：EquipmentRepository、ComplaintRepository、WorkOrderRepository、EquipmentStatusDictionary、WorkOrderPriorityDictionary。

ep-domain-project：model 下 Project 与 ProjectTask；value 下 ProjectStatus、TaskStatus、TaskSource、DerivationUniqueKey（包装裁定 A-16 的 unique_key 字符串并校验其四段格式）；rule 下 derivation.rs、project_guard.rs；port 下 ProjectRepository、ProjectTaskRepository。

WorkOrder 聚合边界包含其登记行与处理记录，理由是工单完成与取消的守卫必须在同一聚合内读取全部登记行状态；跨聚合的只有设备与投诉两个引用。

#### 4.2 在保状态判定

纯函数，位于 ep-domain-service::rule::warranty::judge。

输入：judge_on（NaiveDate，由 Clock 端口取中国标准时间自然日）、start（Option<NaiveDate>）、end（Option<NaiveDate>）。
输出：WarrantyStatus 四取值之一。
步骤：start 或 end 任一为 None 直接返回 NO_WARRANTY_INFO；judge_on < start 返回 WARRANTY_NOT_STARTED；judge_on > end 返回 WARRANTY_EXPIRED；其余返回 IN_WARRANTY。
边界条件：start 等于 end 时该日判为 IN_WARRANTY；judge_on 等于 start 或等于 end 均判为 IN_WARRANTY，即闭区间；start > end 由数据库 CHECK 阻断，函数不做该分支并在调试断言中拒绝。
读取时点：设备详情与列表按当前自然日实时计算，不落库；工单创建时调用一次并把 warranty_status 与 warranty_judged_on 写入工单，此后不随设备保修信息变更改写，工单上这两个字段只读。保修信息修改的用例显式断言不回溯更新任何已存在工单。

#### 4.3 投诉升级为工单

一条投诉最多升级一次由 ux_work_orders_le_source_complaint 唯一索引保证，不使用先查后写。用例流程：在一个事务内加载投诉行 FOR UPDATE，校验其状态不为 CANCELLED，取号生成工单编号，把客户、联系人、联系方式密文、关联合同、关联订单行、关联产品、关联设备复制到工单草稿，按 4.2 判定在保状态，插入工单；唯一约束冲突映射为 SERVICE.CUSTOMER_COMPLAINT.ALREADY_ESCALATED，并在响应 details 中回带既有工单编号。投诉侧不存关联工单列，界面上的关联工单由按 source_complaint_id 的反查得到，该反查在同一唯一索引上完成，不产生额外扫描。

#### 4.4 工单状态机

状态与允许流转按 PRD 9.5.4，本阶段不扩展，取值集合固定。

| 当前状态 | 允许目标 | 守卫条件 | 触发者角色 |
|---|---|---|---|
| DRAFT | PENDING_ACCEPTANCE、CANCELLED | 客户与问题描述非空；关联对象一致性校验全通过 | 创建者 |
| PENDING_ACCEPTANCE | IN_PROGRESS、CANCELLED | 转 IN_PROGRESS 时 assignee_user_id 非空且该用户具备 `TECHNICIAN` | `PROJECT_MANAGER` 或被指派的 `TECHNICIAN` |
| IN_PROGRESS | PENDING_CUSTOMER_CONFIRM、COMPLETED、CANCELLED | 转 COMPLETED 需守卫 G1 与 G2；转 CANCELLED 需守卫 G3 | 被指派的 `TECHNICIAN` 或 `PROJECT_MANAGER` |
| PENDING_CUSTOMER_CONFIRM | IN_PROGRESS、COMPLETED、CANCELLED | 同上 | 被指派的 `TECHNICIAN` 或 `PROJECT_MANAGER` |
| COMPLETED | 无 | 终态只读 | — |
| CANCELLED | 无 | 终态只读 | — |

守卫 G1：conclusion_note 非空。守卫 G2：全部登记行状态属于 {COMPLETED, VOIDED}，否则返回 SERVICE.WORK_ORDER.OPEN_LINES_EXIST 并在 details 中给出未结清登记行的行号、处理方式与关联单据编号清单。守卫 G3：不存在状态属于 {PENDING, LINKED} 的登记行，即取消前必须先作废这些行。任何非法迁移返回 SERVICE.WORK_ORDER.INVALID_STATE_TRANSITION，分类 BUSINESS_CONFLICT，HTTP 409。终态记录的任何字段修改返回 SERVICE.WORK_ORDER.TERMINAL_READ_ONLY。低代码只能在上述既有迁移上增加审批、提醒与时限，不得新增状态或迁移。

终态不原地重开。COMPLETED 工单只可调用“创建返修跟进”：事务内锁定原工单并再次确认其状态仍为 COMPLETED，取新 WO 编号，复制客户、联系人、来源合同/订单/产品、批次、设备、库存序列号引用与在保快照到一张新的 DRAFT 工单，写 `follow_up_of_work_order_id=原工单 id`，问题描述由调用者填写且审计记录来源；不复制登记行、处理记录、结论或附件。跟进次数不设硬上限，可沿引用链追溯。CANCELLED 工单不提供该动作，只能走普通新建并保持 follow_up_of_work_order_id 为空。原工单及全部处理记录永久保留。

#### 4.5 关联对象一致性校验

创建与修改工单时按固定顺序执行，任一不通过即定位到字段并阻止提交。

1. 法人一致：全部关联对象的 legal_entity_id 与工单相同。跨法人引用无入口，且 RLS 使对方法人的记录不可见。
2. 可见性：对当前安全上下文不可见的对象一律按 PLATFORM.AUTHZ.NOT_FOUND_OR_DENIED 返回 404，提示中不回显任何内容，取值按基线第 5.5 节。
3. 客户一致：设备的 customer_id、订单行的客户、合同的客户必须等于工单 customer_id，否则 SERVICE.WORK_ORDER.CUSTOMER_MISMATCH。订单行与合同的客户经 ep-contract-sales 与 ep-contract-clm 的查询 trait 取得，不直读对方表。
4. 设备带出：选择设备后带出 customer_id、product_id、batch_no、sales_order_line_id、inventory_serial_state_id 并按 4.2 判定在保状态；请求另带序列号时先经 SerialStateQuery 解析，结果必须与设备引用相同，否则 SERVICE.WORK_ORDER.SERIAL_STATE_MISMATCH。
5. 订单行带出：选择订单行后带出 contract_id、product_id、batch_no，带出的 contract_id 置为只读。
6. 设备终止状态：设备的 current_status_code 在字典中 is_terminal 为 true 时，若请求未带 `terminal_equipment_confirmed` 标记则返回 SERVICE.WORK_ORDER.EQUIPMENT_TERMINAL_STATUS_CONFIRM_REQUIRED；带该标记时校验调用者具备 `PROJECT_MANAGER`，写入确认人与确认时点，并在同一事务写审计。
7. 允许为空：原订单、合同、产品、批次与设备均可为空，客户与问题描述不可为空。

#### 4.6 退换修登记行

可退数量计算，位于 ep-domain-service::rule::line_quantity::returnable。
输入：delivered_qty（由 ep-contract-sales 的 SalesOrderLineDeliveryQuery 取得的该订单行已交付数量）、registered_qty（本模块该订单行上状态属于 {PENDING, LINKED, COMPLETED} 的登记行数量之和，VOIDED 不计）、request_qty。
判定：request_qty > 0 且 request_qty ≤ delivered_qty − registered_qty，否则返回 SERVICE.WORK_ORDER_LINE.QUANTITY_EXCEEDS_RETURNABLE，details 中回带已交付数量与已登记数量。全部比较用 numeric(18,6) 对应的 Quantity 类型，不做浮点比较，不做隐式舍入。
该校验是前置校验。权威校验在 sales 创建销售退货单时再执行一次，理由是已交付数量归 sales 所有且本模块不能对其加锁；两次判定不一致时以 sales 的结论为准，本模块把登记行退回 PENDING 并返回 SERVICE.WORK_ORDER_LINE.SALES_RETURN_REJECTED。

三类处理方式的挂接：
- RETURN：登记行提交后发布 `service.work_order_line.registered.v1`，由 job-worker 的 `service.return_repair_writeback` 消费者在事务内调用 `ep_contract_sales::SalesReturnCommandPort::create_sales_return(tx, ctx, cmd)`。命令严格按 A-17 现行 supersession 组装：`customer_id` 取工单客户，`sales_order_id` 取 `SalesOrderLineDeliveryQuery` 的权威返回，`return_reason=return_reason_code`、`return_warehouse_id=work_order_line.return_warehouse_id`、`posting_date=work_order_line.return_posting_date`、`remark=reason_note`，`source_ref=Some({source_module:SERVICE,source_doc_type:"WORK_ORDER_RETURN_REQUEST",source_doc_id:work_order_line_id,source_doc_line_id:event_id})`，`allocation_mode=DeliveryAllocationMode::AutoFifo`；这里 event_id 取当前 Outbox 信封 id，同一事件重投不变，取消/驳回后重新提交产生新 event_id，因而能建立新退货尝试且不复用旧销售退货行。命令中的 `delivery_links` 必须为空，由 sales 锁定交付确认行后按 `confirmed_at,id` FIFO 生成并持久化 `assigned_by=AutoFifo`。序列号不从 service 列读取，`inventory_serial_state_id` 非空时经 `SerialStateQuery` 解析权威 serial_no 后以单元素 `serial_nos` 传给 sales。返回 `SalesReturnView.lines` 必须恰有一行，且其 `sales_order_line_id/quantity` 与请求逐值相等；消费者只使用该返回行的 `sales_return_line_id` 回写，不按行号猜测、不二次查询“最新行”。随后把返回头 id 与行 id 分别写入 `sales_return_id/sales_return_line_id` 并置 LINKED。一条登记行在每次有效尝试中固定一对一关联一张销售退货单行，按事件稳定的 source_ref 条件唯一约束使崩溃重试不重复建单；`ReturnRepairTraceQuery` 以 source_doc_id 反查登记行及所属工单。
- EXCHANGE：登记意图只有在退货侧和替换发货侧候选都准备好后才能转 LINKED。自动路径复用上一条完全相同的 `AutoFifo` 销售退货命令，registered 事件必须携带 `replacement_delivery_schedule_id`；消费者从 `SalesReturnView.lines[0].sales_return_line_id` 取得新建行 id 后，在同一事务调用 `SalesExchangeLinkCommandPort::link_exchange` 写 `sales.exchange_links`，`LinkSalesExchange.idempotency_key` 固定取该 work_order_line_id。成功后才一次回写 service 的 `sales_return_id`、返回的 `sales_return_line_id` 与 replacement_delivery_schedule_id，任一调用失败整笔事务回滚。手工路径只提供一个 `link-exchange` 动作，同时接收销售退货行 id 与替换 delivery_schedule id，把 HTTP `Idempotency-Key` 原样传入同一端口，先在 sales 建权威关联再写 service，不提供分别挂一侧的动作。service 先校验工单 customer_id 与登记行 product_id，sales 端口再权威校验退货行与替换发货行属于同一原订单、同一客户、同一产品且两侧均未另行配对；任一失败映射为 SERVICE.WORK_ORDER_LINE.EXCHANGE_SCOPE_MISMATCH 或 ALREADY_LINKED，且两模块均不写。只退不换必须用 RETURN；只补发必须调用 sales 的独立补发动作，不得建立 EXCHANGE 登记行。
- REPAIR：只做登记，填写 repair_result_note 与 repair_finished_on 后由处理人直接从 PENDING 置为 COMPLETED，不关联外部单据，不改变设备当前状态，不产生备件与成本。

登记行状态机守卫：RETURN 的 PENDING → LINKED 需销售退货头行引用齐全；EXCHANGE 的 PENDING → LINKED 需销售退货头行与 replacement_delivery_schedule_id 三列一次齐全并通过客户/产品一致性校验。LINKED → COMPLETED 只能由对方终态事件驱动，接口层不暴露人工置完成入口（REPAIR 除外）：`sales.sales_return.closed.v1` 对 RETURN 写 sales_return_terminal_at 并完成；对 EXCHANGE 只写 sales_return_terminal_at，`sales.delivery.confirmed.v1` 的 lines 命中 replacement_delivery_schedule_id 时只写 replacement_terminal_at，消费者在每次写入后仅当两个时点均非空才完成。任一侧先到均可，重复事件幂等。`sales.sales_return.cancelled.v1` 只可能由 DRAFT/SUBMITTED 发出，或 `rejected.v1` 由 SUBMITTED 驳回时发出；二者命中 RETURN 时回 PENDING 并清退货引用，命中 EXCHANGE 时回 PENDING 并清空 service 的两侧三列与终态时间，审计保留清空前引用，sales.exchange_links 原关联永久保留且不得重绑。REGISTERED 销售退货不可取消，只能进入 CLOSED；service 不实现也不等待“REGISTERED 后 cancelled”分支。重新成对挂接必须使用新建的销售退货行和新建的替换分批交付行，不能复用已取消 pair 的任一侧。任一状态 → VOIDED 需 void_reason 且调用者具备 `PROJECT_MANAGER`。`sales.sales_return.registered.v1` 只确认登记完成，不是 service 终态。数据库 CHECK 与领域守卫共同保证 EXCHANGE 永不以单侧关联或单侧终态完成。

追溯三链路：从工单查全部登记行及其关联单据由本模块自身查询满足；从销售退货单反查来源工单与登记行由 ep-contract-service 的 ReturnRepairTraceQuery 提供，sales 侧详情调用该 trait；从设备档案查该设备涉及的全部工单与登记行由 ix_work_orders_legal_entity_id_equipment_record_id 与 ix_work_order_lines_legal_entity_id_equipment_record_id 支撑。

#### 4.7 合同生效派生项目任务

触发：job-worker 消费 clm 发布的 clm.contract.effective.v1，消费者名按裁定 C-19 固定为 project.contract_derivation。派生项的内容不放在事件载荷里，理由是基线第 6.1 节要求 payload 只放最小必要数据与引用 ID；本阶段在消费时调用 `ep_contract_clm::ContractDerivationPlanQuery::derivation_plan(tx, ctx, contract_id, contract_version_no)` 读取该合同该版本的派生计划，派生项由合同模板决定，本阶段不解释合同条款，与 PRD 9.7.1 的“只接收派生结果”一致。该 trait 及其 DTO 按裁定 A-16 由阶段 6 提供且形状已冻结；按裁定 C-19 撤销 ep_contract_project::ProjectTaskDerivationPort，clm 不同步派生项目任务。该方法接受事务句柄，因此在消费者事务内调用。

派生计划字段按阶段 6 的 F-51 回写固定：ContractDerivationPlan 含 contract_id、contract_version_no、derivation_batch_no、project_group_contract_id、items；ContractDerivationItem 含 item_kind、unique_key、obligation_key、obligation_hash、source_contract_line_id、milestone_no、name、promised_date、quantity、owner_user_id。本阶段只消费 item_kind=ProjectTask 的项；三个键/摘要直接使用 CLM 返回值，不自行拼接或重算。

算法（单事务）：
1. 幂等前置：向 platform_msg.inbox_consumptions 插入 (consumer='project.contract_derivation', event_id)，唯一冲突即整批跳过并置 DONE。
2. 定位项目：取派生计划的 project_group_contract_id，为空时退回取 contract_id，按 (legal_entity_id, 该取值) 查 project.projects，存在则复用，不存在则取号新建，状态 IN_PROGRESS，来源合同与客户由派生计划带入。续签合同因共用根合同 id 而复用同一项目，与 PRD 9.7.6 的“新派生的任务与原任务同属一个项目”一致。
3. 数量守卫：items 长度超过配置上限时整批失败，返回 PROJECT.PROJECT_TASK.DERIVATION_LIMIT_EXCEEDED 并进入死信，理由是避免一次错误配置在单机上产生不可控写入量。
4. 载入对照集：锁定该项目全部 `source='CONTRACT_DERIVED'` 任务，按 derivation_obligation_key 取当前合同版本之前版本号最大的任务作为上一义务实例；终态任务仍进入对照但永不修改。把新计划按 obligation_key 建唯一映射，重复键或空 hash 视为 CLM 契约错误并整批失败。
5. 分类并应用，顺序固定为 REMOVED、CHANGED、UNCHANGED、NEW：
   - REMOVED：上一义务存在、新计划不存在。上一任务若为 NOT_STARTED/IN_PROGRESS，则只置 `derivation_stale=true` 与首次 stale_at，不改状态、不清负责人，并创建第 7 步处置事项；若为 COMPLETED/CANCELLED 则永久保留且不写 stale。
   - CHANGED：同 obligation_key 但 hash 改变。旧非终态任务按 REMOVED 置 stale 并建处置事项；旧终态任务原样永久保留。随后按新版本 unique_key 新建一张 NOT_STARTED 补充任务，写入新 obligation_key/hash、版本与负责人，不覆盖旧任务。
   - UNCHANGED：同 obligation_key 且 hash 相同，不新建、不更新旧任务；无论旧任务是否终态均沿用，避免合同升版复制重复义务。
   - NEW：上一版本不存在该 obligation_key，按新版本 unique_key 新建 NOT_STARTED 补充任务。
6. 若 NEW/CHANGED 需要新任务而项目当前为 COMPLETED 或 CLOSED，消费者先以系统原因 `CONTRACT_OBLIGATION_SUPPLEMENTED` 把项目恢复为 IN_PROGRESS 并写审计；该迁移只允许 project.contract_derivation 消费者执行，用户端点不可调用。这样续签仍复用同一 project_group_contract_id 项目，同时满足补充任务必须创建。
7. 每个首次置 stale 的任务调用 ep-platform-flow 已交付的人工任务命令创建分配给 `PROJECT_MANAGER` 角色队列的处置事项，幂等键固定为 `PROJECT_DERIVATION_STALE:<project_task_id>:<contract_version_no>`，载荷含项目、旧任务、合同、新旧版本、obligation_key 与 REMOVED/CHANGED 原因。处置事项只提醒并提供跳转，不自动完成或取消任务；任务负责人必须在项目端点显式完成或取消旧任务。
8. 新建任务逐条写 project.project_task.derived.v1，项目新建或系统恢复分别写既有项目事件/审计；事务提交。

失败处理：按基线第 6.2 节的八次退避重试，全部失败置 DEAD 并写死信，死信按 legal_entity_id 可枚举，人工修复后记名重投，取值按规格第 15.2 章。派生的项目任务不参与规格第 8 章第 3 步的价格权限、库存可用量、交期与信用额度校验，因此不存在待放行的项目任务，派生完成即为 NOT_STARTED。

重复投递判定：同一 event_id 由 inbox_consumptions 拦截；不同 event_id 但同一新版本 unique_key 由 ux_project_tasks_le_derivation_unique_key 拦截；stale 处置事项另以第 7 步幂等键去重。三层共同保证重复投递不产生重复补充任务、处置事项、事件或审计。终态任务永久保留；不再需要的未终态任务只置 derivation_stale 并等待负责人处置，任何分支都不得自动删除或取消。

#### 4.8 项目与任务状态机

项目任务：NOT_STARTED → IN_PROGRESS 需 assignee_user_id 非空；IN_PROGRESS → NOT_STARTED 允许；任一非终态 → COMPLETED 时写入 actual_finish_on 为流转时点中国标准时间自然日；任一非终态 → CANCELLED 需 cancel_reason。COMPLETED 与 CANCELLED 为终态只读。
项目：IN_PROGRESS → COMPLETED 与 IN_PROGRESS → CLOSED 均需守卫 P1，即全部任务状态属于 {COMPLETED, CANCELLED}，否则 PROJECT.PROJECT.OPEN_TASKS_EXIST 并给出未结清任务清单；COMPLETED → CLOSED 允许。普通用户路径把 CLOSED 视为终态并拒绝任务变更；唯一系统例外是第 4.7 节收到同续签链 NEW/CHANGED 合同义务时由 project.contract_derivation 恢复为 IN_PROGRESS，创建补充任务并写审计，其他消费者和 API 均不可触发。

#### 4.8.1 F-10 `CLM_TERM_PROJECT_TASK` 影响面规则

实现类型固定为 `ContractTerminationProjectTaskImpactRule`，位于 `crates/application/project/src/impact/contract_termination_project_task.rs`，实现阶段 3 的 `ep_platform_impact::ImpactRule`。`code()` 固定返回 `CLM_TERM_PROJECT_TASK`，`upstream_event_type()` 固定返回 `clm.contract.terminated.v1`，`target_module` 固定为 `ModuleCode::Project`。它是阶段 12 追加的第七个真实注册项；目录在所有阶段恒为七条，阶段 12 结束时 `ImpactRegistry` 真实注册数才达到 7。本阶段不得创建消费 `clm.contract.terminated.v1` 的第二个消费者，也不得以 Noop、空规则或直接 DONE 的占位实现凑注册数。

`assess` 在调用方事务内只经 project 仓储查询同法人且 `source_contract_id=contract_id`、状态属于 `NOT_STARTED|IN_PROGRESS` 的任务，按 `project_task_id UUID bytes` 升序返回一项一个目标；已 COMPLETED/CANCELLED、其他合同、其他法人及 `source_contract_id` 为空的任务均不命中。目标引用固定携带 `target_doc_id=project_task_id`、`target_doc_no=task_no`、`target_doc_line_no=null`。NOT_STARTED 产生 `AUTO_CANCEL`，IN_PROGRESS 产生 `MANUAL_DECISION`；后者由平台按 `target_module=PROJECT` 的固定管理角色映射分配给 `PROJECT_MANAGER`，规则不得自行传入任意角色。

`dispose` 必须在当前 `&mut dyn Tx` 中按 `project_task_id` 取得任务行 `FOR UPDATE`，再复核法人、`source_contract_id` 与当前状态；使用平台统一结果型，三条分支唯一如下。

1. 当前仍为 NOT_STARTED：写 `status=CANCELLED`、`cancel_reason="合同终止 <合同编号>"`、`actual_finish_on=null`，递增 row_version，并在同一事务写审计；返回 `ImpactDisposeOutcome::Completed { reason: "PROJECT_TASK_AUTO_CANCELLED" }`。同一处置项的 `idempotency_key` 固定为 item id，重放不得二次改写或二次审计。
2. 当前已为 COMPLETED 或 CANCELLED：不改任务、不补第二条审计，返回 `ImpactDisposeOutcome::AlreadySatisfied { reason: "PROJECT_TASK_ALREADY_TERMINAL" }`；平台据此把项计为 DONE。该分支承接 assess 后由负责人先行完成/取消的合法竞态，不把已经满足的目标误报为失败。
3. assess 时为 NOT_STARTED、加锁重检时已变为 IN_PROGRESS，或人工项被推进时任务仍为 IN_PROGRESS：不改任务，返回 `ImpactDisposeOutcome::NeedsManualDecision { reason: "PROJECT_TASK_IN_PROGRESS_REQUIRES_DECISION" }`。这不是失败，不增加 attempts、不退避、不进死信。平台在同一事务把 item 的 `disposition_kind` 改为 MANUAL_DECISION、保持 PENDING，按 PROJECT → PROJECT_MANAGER 固定映射创建或幂等复用一个 `HUMAN_TASK` 并回填 `process_task_id`；规则自身不创建流程任务。

人工决策项不允许一句“继续处理”就闭合。负责人只能先经现有项目任务完成或取消动作把目标推进到 COMPLETED/CANCELLED；取消仍要求非空 cancel_reason。随后提交的命令形状固定为 `ManualImpactDecision { decision_code, decision_reason, decision_result_doc_id }`，本规则的封闭允许集只有两码：`PROJECT_TASK_COMPLETED` 与 `PROJECT_TASK_CANCELLED`。两码均要求清洗后 `decision_reason` 非空，且 `decision_result_doc_id` 非空并严格等于本处置项的 `target_doc_id`；不解析理由文本猜分支，也不只把决策藏在 process task outcome 中。平台只校验该 code 属于目录允许集、理由非空和结果 id 必填形状；本规则在同一事务对目标任务 `FOR UPDATE`，复核同法人、同来源合同及结果 id 同任务后，`PROJECT_TASK_COMPLETED` 只在当前状态为 COMPLETED 时返回 `AlreadySatisfied { reason: "PROJECT_TASK_MANUAL_COMPLETED" }`，`PROJECT_TASK_CANCELLED` 只在当前状态为 CANCELLED 时返回 `AlreadySatisfied { reason: "PROJECT_TASK_MANUAL_CANCELLED" }`。错码、空理由、空/异任务结果 id、状态与码不匹配都拒绝且保持 PENDING；目标仍为 IN_PROGRESS 时继续返回 `NeedsManualDecision`。决策通过时平台把 code、reason、result doc id 逐字持久到 `impact_disposition_items`后才置 DONE。任何自动或人工分支都不直接改合同状态；只有平台在全部七类项 DONE、无 DEAD 且 `item_done=item_total` 后推进合同到 TERMINATED 并发 `clm.contract.termination_completed.v1` 恰一次。

本规则与第 4.7 节的合同升版处置是两个互斥场景：合同变更或续签造成义务 NEW/CHANGED/REMOVED 时仍按 `derivation_stale` 与补充任务规则，不自动取消旧任务；只有 F-10 合同终止批次调用本小节规则，NOT_STARTED 才自动取消。不得把 U-J-13 的变更规则套到终止场景，也不得让终止规则处理普通升版。

#### 4.9 由项目任务提交采购需求

本用例固定为两段式，且任务自身持久化状态，不再从 Outbox 临时推断“提交中”。提交动作请求体固定为 `{material_id,quantity,required_on}`。第一段在一个事务内加载任务 `FOR UPDATE`，校验任务状态属于 `{NOT_STARTED, IN_PROGRESS}`、所属项目不为 CLOSED，并按以下封闭状态机执行：未提交时冻结三项申请快照，写 `requisition_link_state=PENDING`、清空 `requisition_link_last_error`，同事务发布 `project.project_task.requisition_requested.v1`；FAILED 允许重新提交，但三项请求必须与冻结快照逐值相同，否则返回 `PLATFORM.IDEMPOTENCY.PAYLOAD_MISMATCH`，相同则置回 PENDING 并发新事件；PENDING 的不同 HTTP 幂等键重复提交返回 `PROJECT.PROJECT_TASK.REQUISITION_PENDING` 且不增发事件；LINKED 或已存在 link 行返回 `PROJECT.PROJECT_TASK.REQUISITION_ALREADY_LINKED`。HTTP 四元组幂等仍返回首次响应，但它只去重动作请求，绝不进入采购来源键。事件从冻结快照取 material_id/quantity/required_on，`unique_key` 永远固定为 `PROJECT_TASK:{project_task_id}`，与阶段 7 的来源键逐字一致；一个任务无论重试或换 HTTP 幂等键都不能形成第二张采购需求。

第二段由 job-worker 的 `project.requisition_intake` 消费者在一个数据库事务内完成。消费者先插入 Inbox 去重行并锁定任务，随后调用唯一跨模块入口 `ep_contract_procure::PurchaseRequisitionIntakePort::intake(tx, ctx, cmd)`；命令逐字段取 Stage7 第 4.6.2 小节唯一 ABI：`source_module=ModuleCode::Project`、`source_doc_id=project_id`、`source_doc_line_id=project_task_id`、`source_doc_no=Some(task_no)`、`source_contract_id=project_tasks.source_contract_id`（可空）、`suggested_purchase_type=Material`、`material_id=Some(event.material_id)`、`warehouse_id=None`、`expense_item_code=None`、`quantity/required_on` 取事件快照、`suggested_supplier_id=None`、`is_drop_ship=false`、`unique_key=PROJECT_TASK:{project_task_id}`。对手工任务不得伪造合同；procure 侧据 source_doc_id 固化必填 project_id，仅当可空 source_contract_id 存在时固化 contract_id。`intake` 返回后只使用回执的 `purchase_requisition_id/doc_no`，在同一事务插入 `project_task_purchase_requisition_links`、把任务置 LINKED 并保持 last_error 为空；采购需求受理、双向 link、任务 LINKED 与 Inbox 行必须同事务提交，任一步失败全部回滚，不允许采购需求已建而 link 未建。link 表的 purchase_requisition_id 始终非空，且 `(legal_entity_id,project_task_id)` 与 `(legal_entity_id,purchase_requisition_id)` 两个唯一键共同兜底。

八档退避全部失败后的第九次失败路径也冻结：`project.requisition_intake` 的 job-worker dispatch 分支在写 `outbox_events.status=DEAD` 与 `platform_msg.dead_letters` 的同一事务，经 project 仓储锁定 payload 中的任务，只在当前仍为 PENDING 时写 `requisition_link_state=FAILED` 和清洗后的 `requisition_link_last_error`；LINKED 不回退，异法人或异任务按不变量故障告警。死信记名 replay 或任务上的重试动作复用原 `PROJECT_TASK:{project_task_id}`：重投成功可由 FAILED 直接原子收敛到 LINKED，重投再次失败仍保持 FAILED；重新提交建立新事件时先按第一段原子回到 PENDING。阶段 7 排在本阶段之前，本阶段开工时 `PurchaseRequisitionIntakePort` 的真实实现已装配，两个 wiring 目录下均不得出现任何替身。理由是基线第 10.3 节禁止在交互请求事务内跨模块写编排；跨模块写只发生在消费者的单一短事务内。

#### 4.10 客户 360 聚合

契约（ep-contract-crm，按裁定 C-09 由阶段 5 建立，本阶段只追加三个 SectionKind 取值与三个提供者实现，不新增 trait）：

```
pub enum Customer360SectionKind { Contracts, Receipts, Complaints, Equipments, WorkOrders }
pub struct Customer360Query { pub customer_id: Uuid, pub section: Customer360SectionKind, pub size: u16 }
pub struct Customer360Item {
    pub object_type: String, pub object_id: Uuid, pub doc_no: String,
    pub title: String, pub occurred_on: NaiveDate, pub status_code: String,
    pub amount: Option<Money>, pub security_level: i16,
}
#[async_trait] pub trait Customer360SectionProvider: Send + Sync {
    fn kind(&self) -> Customer360SectionKind;
    async fn fetch(&self, ctx: &SecurityContext, q: &Customer360Query)
        -> Result<Vec<Customer360Item>, AppError>;
}
```

聚合算法（ep-app-crm::usecase::query_customer_360）：
1. 校验客户在当前安全上下文下可见，不可见返回 404 与 PLATFORM.AUTHZ.NOT_FOUND_OR_DENIED。
2. 对已注册的提供者并发扇出，并发上限取配置。service/clm 自有查询提供者各自使用只读分析池的一个连接，语句超时按只读池取值；Receipts 提供者因阶段 10 的 finance ABI 精确要求 `&mut dyn Tx`，由提供者单独开启一个 `READ ONLY REPEATABLE READ` 法人事务并把同一句柄传入 finance query，绝不把具体连接或跨 schema SQL交给 ep-app-crm。另在应用侧对每个区块施加 section_timeout_ms 的超时。
3. 未注册提供者的区块返回 section_status 为 NOT_AVAILABLE；超时或失败的区块返回 DEGRADED 并计一次 ep_crm_customer360_section_degraded_total，不使整个请求失败，理由是客户 360 是查询类视图，单一区块不可用不应阻断其余区块。
4. 每区块内按 occurred_on 降序、object_id 降序截断到 size 条，size 默认 20、上限 50。
5. 全部区块的字段级裁剪与密级过滤由 ep-platform-authz 在提供者内部完成，聚合层不做二次裁剪，也不做跨区块排序，避免通过排序位次间接暴露无权数据，取值按规格第 7.9 章。

本阶段实现 Complaints、Equipments、WorkOrders 三个提供者，位于 ep-app-service，其中 EquipmentsSectionProvider 是设备在客户 360 中的唯一可见路径，按裁定 B-06 不经任何跨模块 trait；Contracts 与 Receipts 由 ep-app-clm 与 ep-app-finance 实现。`ReceiptsSectionProvider` 复用阶段 10 的真实 `ReceivableLedgerQuery`：先以 `ReceivableLedgerQueryInput { legal_entity_id, customer_id: Some(q.customer_id), contract_id: None, sales_order_id: None, entry_ids: None, period: None, after, limit: 200 }` 分页取得该客户应收主条目，再对可见条目调用 `settlement_effects`，只保留有效净方向为 APPLY 且 `source_doc_type=RECEIPT` 的资金根，按 `source_doc_id` 去重并映射 `source_doc_no/business_date/amount`，最后按 `business_date DESC,source_doc_id UUID bytes DESC` 截断到 q.size；空台账合法返回空区块。它不得从 `finance.receipts` 或任何 finance view 直查，不得注入 Noop，ep-app-crm 只看到 `Customer360SectionProvider`。端点与契约在阶段 5 已启用并只挂载 mdm 自己的区块，本阶段接管后追加上述三个区块，未注册的区块按 3 的规则返回 NOT_AVAILABLE。三个自实现提供者的取数各命中一条索引：ix_customer_complaints_legal_entity_id_customer_id_complaint_on、ix_equipment_records_legal_entity_id_customer_id、ix_work_orders_legal_entity_id_customer_id_created_at。

#### 4.11 与账务和库存的边界

本阶段的任何用例都不写 ledger 与 inventory 的任何表，也不发布会计事件。事件-分录表在规格第 5.2 章财务规则条目，本阶段不复述借贷与取价。退换修产生的实物出入库与账务后果一律由销售退货单在其所属模块按规格第 5.2 章财务规则条目的销售退货事件与退款事件承接。这一边界是本阶段退出条件之一，判定方式见 8.5。

#### 4.12 历史导入撤销的 owner 更正命令

本节只为 Stage 14 已冻结的 `MigrationModuleWriter::apply_reversal` 提供三个 crate-private owner 命令，不增加 HTTP、公开 `ep-contract-*` trait、普通角色权限、通用 reversal 表或新的业务事件。三个入口固定为 `reverse_migrated_project`、`reverse_migrated_customer_complaint`、`reverse_migrated_equipment`，只能由各自 `ep-app-project`/`ep-app-service` 的 migration writer 在 Stage 14 调用方 UnitOfWork 内调用；输入中的 root id、服务端 UUIDv7 correction id 与固定原因均由已锁定的 migration record 派生，客户端、Excel、MCP、插件和普通 Posting/Repository 入口均不能选择或调用。

`reverse_migrated_project` 的顺序固定为锁项目→按 task id UUID bytes 锁全部任务→校验 Stage 14 已把 LINKED 采购需求依赖排在本对象之前。NOT_STARTED/IN_PROGRESS 任务逐条复用既有 CANCEL 入口，固定 `cancel_reason='DATA_MIGRATION_REVERSED'`；完成任务集合终态守卫后，IN_PROGRESS/COMPLETED 项目复用既有 CLOSED 入口并插入 mode=CLOSE 的 correction，原本已 CLOSED 则不改根而插入 mode=RETAIN_CLOSED。返回的 owner effect 精确为 `target_object_type='project.project_migration_corrections'`、`target_id=correction.id`；不得删除任务、链接、附件或项目。

`reverse_migrated_customer_complaint` 锁投诉后，REGISTERED/PROCESSING 复用现有 CANCEL 入口并插入 mode=CANCEL 的 correction；CLOSED/CANCELLED 不改原处理说明、关闭/取消证据或工单关系，只插入 mode=RETAIN_TERMINAL。若存在由本投诉升级且尚未完成 Stage 14 反向计划的工单，本对象不得进入可执行 plan。返回 effect 精确为 `service.customer_complaint_migration_corrections` 新行。

`reverse_migrated_equipment` 锁设备与当前/RETURNED 两条状态字典行，先复核没有未结工单且 Stage 14 已满足库存序列占用反向依赖；当前状态字典 `is_terminal=false` 时复用现有 change-status 入口置 RETURNED 并插入 mode=SET_RETURNED，`is_terminal=true` 时保持原状态并插入 mode=RETAIN_TERMINAL。不得删除设备、附件、序列引用或普通状态审计；返回 effect 精确为 `service.equipment_migration_corrections` 新行。

三命令都必须在同一事务把 root/task after-image、具名 correction fact、既有状态事件/审计（仅实际发生状态迁移的分支）、Stage 14 event_id=receipt id 的 R0、writer receipt 与 migration record REVERSED 指针一次提交。终态 retain 分支没有伪造状态事件，证据由具名 correction + R0 构成。任一步失败整事务回滚；每个 correction 表的 root 唯一键使不同 HTTP 幂等键或不同 run 重放也不能产生第二条 owner effect。数据库延迟图只验证本次提交最终形状，后续业务合法变化不回写、更改或删除该不可变事实。

---

### 5. API 契约

全部端点遵循基线第 5 节：路径前缀 /api/v1，字段 snake_case，成功与失败封套固定，写请求必带 Idempotency-Key、Authorization、X-Legal-Entity-Id、X-Device-Id、X-Client，分页参数 page 与 page_size（默认 20、上限 200），排序 sort 白名单，过滤 filter[<field>]=<op>:<value>。本阶段无高风险操作，因此不要求 X-Reauth-Token；含联系方式的导出除外，其重新认证由平台导出能力承担。

按裁定 A-20，本阶段每个用例在 crates/contract/service/src/capability.rs 与 crates/contract/project/src/capability.rs 中声明一对常量 `<USECASE_SCREAMING>_DOMAIN` 与 `<USECASE_SCREAMING>_ACTION`，售后侧的能力域一律取 `CapabilityDomain::ServiceWorkorderEquipment`，项目侧一律取 `CapabilityDomain::ProjectTaskMilestone`，动作类别取 `ActionClass` 的 Read、Write、Submit 之一，本阶段没有 Approve 与 Export 路由。客户 360 端点的一对常量随 `CapabilityDomain::CrmCustomer360` 由阶段 5 在 crates/contract/crm/src/capability.rs 中声明，本阶段不重复声明。两个枚举由阶段 1 在 ep-foundation 冻结，`xtask configdoc` 断言每个 `/api/v1/` 路由都能解析到一对常量，缺失即构建失败。

出厂角色绑定统一使用 RoleCode：`TECHNICIAN` 可登记安装、调试、维修、处理日志与交付证据，并只读取阶段 6 提供的不含价格合同摘要；不得借售后端点读取合同总额、单价、成本或毛利，也不得最终确认交付、确认收入或执行财务审批。`PROJECT_MANAGER` 承担终止状态确认、保修维护、工单取消、登记行作废和派生 stale 处置队列；最终交付确认仍只能在 sales 侧由 `SALES_MANAGER` 或 `PROJECT_MANAGER` 执行。本阶段文档中的“处理人”只表示记录关系，不生成新 RoleCode。

#### 5.1 设备档案

| 方法与路径 | 说明 | 主要请求字段 | 响应 | 主要错误码 | 权限 |
|---|---|---|---|---|---|
| GET /api/v1/service/equipments | 设备列表 | filter 支持 customer_id、product_id、model、current_status_code、warranty_status、delivered_on(between)、serial_no；serial_no 先经 SerialStateQuery 解析 id 再过滤 service 引用；sort 白名单 created_at、code、delivered_on | data 为设备摘要数组，序列号展示经 inventory 查询投影，warranty_status 实时计算，meta 为分页 | — | service.equipment_record.read |
| GET /api/v1/service/equipments/{id} | 设备详情 | — | 含实时在保状态、状态变更历史（取自审计）、附件清单 | PLATFORM.AUTHZ.NOT_FOUND_OR_DENIED | 同上 |
| POST /api/v1/service/equipments | 手工新增 | model、customer_id、inventory_serial_state_id 或扫码 serial_no 二选一且均可不填、product_id、batch_no、delivered_on、installed_on、current_status_code、保修四项、remark；serial_no 只作解析入参不落 service | 201 与设备详情 | SERVICE.EQUIPMENT_RECORD.WARRANTY_RANGE_INVALID、SERVICE.EQUIPMENT_RECORD.INSTALL_BEFORE_DELIVERY、SERVICE.EQUIPMENT_RECORD.DELIVERY_DATE_IN_FUTURE、SERVICE.EQUIPMENT_RECORD.STATUS_UNKNOWN、SERVICE.EQUIPMENT_RECORD.SERIAL_STATE_NOT_FOUND、SERVICE.EQUIPMENT_RECORD.SERIAL_STATE_ALREADY_LINKED | service.equipment_record.create |
| POST /api/v1/service/equipments/actions/create-from-delivery-batch | 从交付确认单逐台建档 | delivery_confirmation_id、lines 数组（line_id、count、model、inventory_serial_state_ids、保修四项），serial-managed 行的 id 数必须等于 count，服务端按数组顺序生成 unit_no=1..count，单次上限 200 | data 为生成设备数组与按 `(line_id, unit_no)` 幂等跳过清单 | SERVICE.EQUIPMENT_RECORD.BATCH_LIMIT_EXCEEDED、PLATFORM.REQUEST.INVALID_PAYLOAD | service.equipment_record.create |
| PATCH /api/v1/service/equipments/{id} | 修改非保修字段 | 允许 model、product_id、inventory_serial_state_id、installed_on、remark；绑定序列号变更需 reason，row_version 必填 | 设备详情 | PLATFORM.CONCURRENCY.STALE_VERSION、SERVICE.EQUIPMENT_RECORD.SERIAL_STATE_ALREADY_LINKED | service.equipment_record.update |
| POST /api/v1/service/equipments/{id}/actions/change-status | 变更当前状态 | to_status_code、reason、row_version | 设备详情 | SERVICE.EQUIPMENT_RECORD.STATUS_UNKNOWN | service.equipment_record.change-status |
| POST /api/v1/service/equipments/{id}/actions/update-warranty | 维护保修信息 | 保修四项、reason、row_version | 设备详情 | SERVICE.EQUIPMENT_RECORD.WARRANTY_EDIT_FORBIDDEN、SERVICE.EQUIPMENT_RECORD.WARRANTY_RANGE_INVALID | service.equipment_record.maintain-warranty，仅 `PROJECT_MANAGER` |
| GET /api/v1/service/equipments/{id}/work-orders | 按设备查工单与登记行 | 分页 | 工单摘要与登记行摘要 | — | service.work_order.read |

service 永不接受“新建序列号”写入。传 serial_no 时只调用 SerialStateQuery 解析已有 `inventory.serial_states`；不存在返回 `SERVICE.EQUIPMENT_RECORD.SERIAL_STATE_NOT_FOUND`，已被设备引用返回 `SERVICE.EQUIPMENT_RECORD.SERIAL_STATE_ALREADY_LINKED`。手工设备没有库存序列号可留空并只用设备 code；任何列表、详情、导出或搜索中的序列号文本均按 inventory_serial_state_id 实时读取，不从 service 回退第二份文本。

按裁定 B-06，设备的跨模块可见性只保留三条路径：本节的 GET /api/v1/service/equipments 与 /{id}、全文检索索引中 object_type 为 service.equipment_records 的文档、以及本阶段自实现的 EquipmentsSectionProvider。不提供 ep-contract-service::EquipmentQuery，报表侧的设备取数一律经受治理数据集视图，低代码的设备引用经上述 HTTP 端点解析。

#### 5.2 客户投诉

| 方法与路径 | 说明 | 幂等语义 | 主要错误码 |
|---|---|---|---|
| GET /api/v1/service/customer-complaints | 列表，filter 支持 status、customer_id、complaint_on(between)、accepted_by | — | — |
| GET /api/v1/service/customer-complaints/{id} | 详情，含反查得到的关联工单编号 | — | PLATFORM.AUTHZ.NOT_FOUND_OR_DENIED |
| POST /api/v1/service/customer-complaints | 登记，状态直接为 REGISTERED | 四元组幂等，重放回带首次结果与 Idempotent-Replay: true | PLATFORM.REQUEST.INVALID_PAYLOAD |
| PATCH /api/v1/service/customer-complaints/{id} | 非终态下修改可编辑字段 | 需 row_version | PLATFORM.CONCURRENCY.STALE_VERSION、SERVICE.CUSTOMER_COMPLAINT.TERMINAL_READ_ONLY |
| POST /api/v1/service/customer-complaints/{id}/actions/accept | 受理并填写受理人 | 同上 | SERVICE.CUSTOMER_COMPLAINT.INVALID_STATE_TRANSITION |
| POST /api/v1/service/customer-complaints/{id}/actions/close | 关闭并填写处理说明 | 同上 | SERVICE.CUSTOMER_COMPLAINT.HANDLING_NOTE_REQUIRED |
| POST /api/v1/service/customer-complaints/{id}/actions/cancel | 取消并填写原因，仅 `PROJECT_MANAGER` | 同上 | PLATFORM.AUTHZ.OBJECT_FORBIDDEN |
| POST /api/v1/service/customer-complaints/{id}/actions/escalate-to-work-order | 升级为工单 | 同一 Idempotency-Key 重放返回首次工单；不同键的重复升级返回 409 | SERVICE.CUSTOMER_COMPLAINT.ALREADY_ESCALATED |

#### 5.3 售后工单

| 方法与路径 | 说明 | 主要错误码 |
|---|---|---|
| GET /api/v1/service/work-orders | 列表，filter 支持 status、work_order_type_code、priority_code、customer_id、assignee_user_id、follow_up_of_work_order_id、created_at(between)、warranty_status、has_open_lines(eq:true/false)；默认筛选期间最近 3 个自然月 | — |
| GET /api/v1/service/work-orders/{id} | 详情，含登记行、处理记录、附件 | PLATFORM.AUTHZ.NOT_FOUND_OR_DENIED |
| POST /api/v1/service/work-orders | 创建（草稿或直接提交由 submit 标记决定）；三个入口共用本端点，入口差异只体现在预填 | SERVICE.WORK_ORDER.CUSTOMER_MISMATCH、SERVICE.WORK_ORDER.EQUIPMENT_TERMINAL_STATUS_CONFIRM_REQUIRED |
| PATCH /api/v1/service/work-orders/{id} | 非终态下修改；warranty_status 与 warranty_judged_on 为只读，传入即 `PLATFORM.REQUEST.INVALID_PAYLOAD` | SERVICE.WORK_ORDER.TERMINAL_READ_ONLY |
| POST /api/v1/service/work-orders/{id}/actions/submit | DRAFT → PENDING_ACCEPTANCE，写 submitted_at 并登记时限定时器 | SERVICE.WORK_ORDER.INVALID_STATE_TRANSITION |
| POST /api/v1/service/work-orders/{id}/actions/assign | 指派或自行受理，写 assignee_user_id 与 accepted_at | SERVICE.WORK_ORDER.ASSIGNEE_REQUIRED |
| POST /api/v1/service/work-orders/{id}/actions/request-customer-confirmation | IN_PROGRESS → PENDING_CUSTOMER_CONFIRM | 同上 |
| POST /api/v1/service/work-orders/{id}/actions/resume-processing | PENDING_CUSTOMER_CONFIRM → IN_PROGRESS | 同上 |
| POST /api/v1/service/work-orders/{id}/actions/complete | 完成，守卫 G1 与 G2 | SERVICE.WORK_ORDER.OPEN_LINES_EXIST、SERVICE.WORK_ORDER.CONCLUSION_REQUIRED |
| POST /api/v1/service/work-orders/{id}/actions/cancel | 取消，守卫 G3，仅 `PROJECT_MANAGER` | SERVICE.WORK_ORDER.OPEN_LINES_EXIST |
| POST /api/v1/service/work-orders/{id}/actions/create-repair-follow-up | 仅 COMPLETED；创建新的 DRAFT 并写 follow_up_of_work_order_id，次数无硬上限；CANCELLED 调用拒绝 | SERVICE.WORK_ORDER.INVALID_STATE_TRANSITION |
| POST /api/v1/service/work-orders/{id}/logs | 追加一条处理记录，只追加不覆盖 | PLATFORM.REQUEST.INVALID_PAYLOAD |
| GET /api/v1/service/work-orders/{id}/logs | 处理记录列表，按 created_at、id 升序 | — |
| POST /api/v1/service/work-orders/{id}/lines | 新增登记行；RETURN/EXCHANGE 请求体必带 return_posting_date，并按销售行形态给出可空 return_warehouse_id | SERVICE.WORK_ORDER_LINE.QUANTITY_EXCEEDS_RETURNABLE、SERVICE.WORK_ORDER_LINE.SALES_ORDER_LINE_REQUIRED、SERVICE.WORK_ORDER.MAX_LINES_EXCEEDED |
| GET /api/v1/service/work-orders/{id}/lines | 登记行列表 | — |
| PATCH /api/v1/service/work-orders/{id}/lines/{line_id} | PENDING 状态下修改数量与说明 | SERVICE.WORK_ORDER_LINE.INVALID_STATE_TRANSITION |
| POST /api/v1/service/work-orders/{id}/lines/{line_id}/actions/link-sales-return | 仅 RETURN，手工挂接已存在的销售退货单行 | SERVICE.WORK_ORDER_LINE.ALREADY_LINKED |
| POST /api/v1/service/work-orders/{id}/lines/{line_id}/actions/link-exchange | 仅 EXCHANGE；同一请求必须给 sales_return_line_id 与 replacement_delivery_schedule_id，经 SalesExchangeLinkCommandPort 成对写入 | SERVICE.WORK_ORDER_LINE.ALREADY_LINKED、SERVICE.WORK_ORDER_LINE.EXCHANGE_PAIR_REQUIRED、SERVICE.WORK_ORDER_LINE.EXCHANGE_SCOPE_MISMATCH |
| POST /api/v1/service/work-orders/{id}/lines/{line_id}/actions/complete-repair | 维修登记完成，写维修结果与完成日期 | SERVICE.WORK_ORDER_LINE.PROCESSING_METHOD_MISMATCH |
| POST /api/v1/service/work-orders/{id}/lines/{line_id}/actions/void | 作废登记行，仅 `PROJECT_MANAGER` | PLATFORM.AUTHZ.OBJECT_FORBIDDEN |
| GET /api/v1/service/work-order-lines | 跨工单的登记行清单，filter 支持 handling_method、status、has_linked_document | — |

#### 5.4 项目与项目任务

| 方法与路径 | 说明 | 主要错误码 |
|---|---|---|
| GET /api/v1/project/projects | 列表，filter 支持 status、customer_id、owner_user_id、source_contract_id | — |
| GET /api/v1/project/projects/{id} | 详情，含任务统计与附件 | PLATFORM.AUTHZ.NOT_FOUND_OR_DENIED |
| POST /api/v1/project/projects | 手工新建 | PLATFORM.REQUEST.INVALID_PAYLOAD |
| PATCH /api/v1/project/projects/{id} | 修改，需 row_version | PLATFORM.CONCURRENCY.STALE_VERSION |
| POST /api/v1/project/projects/{id}/actions/complete | 完成，守卫 P1 | PROJECT.PROJECT.OPEN_TASKS_EXIST |
| POST /api/v1/project/projects/{id}/actions/close | 关闭，守卫 P1 | 同上 |
| GET /api/v1/project/project-tasks | 跨项目任务列表，filter 支持 project_id、status、assignee_user_id、planned_finish_on(between)、source、requisition_link_state；响应显式返回该三态或 null，FAILED 时返回清洗后的 last_error | — |
| GET /api/v1/project/projects/{id}/tasks | 项目下任务列表；响应显式返回 requisition_link_state 与可空 last_error | — |
| POST /api/v1/project/projects/{id}/tasks | 手工新增任务，source 固定 MANUAL | PLATFORM.REQUEST.INVALID_PAYLOAD |
| PATCH /api/v1/project/project-tasks/{id} | 修改任务 | PROJECT.PROJECT_TASK.TERMINAL_READ_ONLY |
| POST /api/v1/project/project-tasks/{id}/actions/start | NOT_STARTED → IN_PROGRESS | PROJECT.PROJECT_TASK.ASSIGNEE_REQUIRED |
| POST /api/v1/project/project-tasks/{id}/actions/revert-to-not-started | IN_PROGRESS → NOT_STARTED | PROJECT.PROJECT_TASK.INVALID_STATE_TRANSITION |
| POST /api/v1/project/project-tasks/{id}/actions/complete | 完成并写实际完成日期 | 同上 |
| POST /api/v1/project/project-tasks/{id}/actions/cancel | 取消并填写原因 | 同上 |
| POST /api/v1/project/project-tasks/{id}/actions/submit-purchase-requisition | 请求 `{material_id,quantity,required_on}`；首次冻结快照并置 PENDING，FAILED 仅允许同快照重试，LINKED 拒绝 | PLATFORM.IDEMPOTENCY.PAYLOAD_MISMATCH、PROJECT.PROJECT_TASK.REQUISITION_PENDING、PROJECT.PROJECT_TASK.REQUISITION_ALREADY_LINKED |
| GET /api/v1/project/project-tasks/{id}/purchase-requisitions | 该任务提交的采购需求引用清单 | — |

#### 5.5 客户 360

GET /api/v1/crm/customers/{id}/customer-360，该路径按裁定 C-09 在阶段 5 已启用，本阶段不新增路径

查询参数：sections（可选，逗号分隔的区块名，默认全部五个）、section_size（默认 20，上限 50）。
响应 data 结构：

```json
{
  "customer_id": "…",
  "sections": [
    { "kind": "work_orders", "section_status": "OK",
      "items": [ { "object_type": "service.work_orders", "object_id": "…",
                   "doc_no": "WO-01-202611-000123", "title": "…",
                   "occurred_on": "2026-11-02", "status_code": "IN_PROGRESS",
                   "amount": null, "security_level": 20 } ] },
    { "kind": "contracts", "section_status": "NOT_AVAILABLE", "items": [] }
  ]
}
```

section_status 取值为 OK、DEGRADED、NOT_AVAILABLE 三种。权限为 crm.customer_360.read 加上各区块所属对象的读权限，任一区块无权时该区块返回空数组且 section_status 为 OK，不区分无权与无数据，避免通过区块状态泄漏存在性。

---

### 6. 并发与事务边界

#### 6.1 事务边界

全部写用例按基线第 10.3 节的工作单元闭包表达，入口为 `UnitOfWork::transact(ctx, |tx| …)`，一个用例一个事务，隔离级别 READ COMMITTED，业务事务不超过 5 秒，读写池 statement_timeout 10 秒、lock_timeout 3 秒。事务内一律不做外部 HTTP 调用、不读写附件正文、不发通知、不等待用户输入。所有冻结签名含 `&mut dyn Tx` 的跨模块契约都在调用方同一工作单元内执行，事务句柄统一为 `ep_foundation::port::Tx`；本阶段的完整集合为 `ContractDerivationPlanQuery`、`SalesOrderLineDeliveryQuery`、`SalesReturnCommandPort`、`SalesExchangeLinkCommandPort`、`PurchaseRequisitionIntakePort`、`MasterDataLookup`、`ClassificationItemQuery` 与 `SerialStateQuery`。其中只读方法只做索引命中的目标行读取或短锁校验，禁止扫描与外部 IO；命令方法按各端口冻结的锁序执行。交互界面可在事务外做一次只读预览，但写用例必须在事务内重新调用相应契约并仅以事务内结果判定，预览结果不得直接进入持久化守卫。`ContractDerivationPlanQuery`、`SalesReturnCommandPort` 与 `PurchaseRequisitionIntakePort` 只出现在 job-worker 消费者事务；`SalesExchangeLinkCommandPort` 同时用于 job-worker 自动配对和 core-server 的单动作手工配对；`SalesOrderLineDeliveryQuery`、`MasterDataLookup`、`ClassificationItemQuery` 与 `SerialStateQuery` 可出现在 core-server 的交互式写用例，全部仍受 5 秒事务上限约束。

一个事务内写入的内容固定为三类并集：业务状态、审计事件、Outbox 条目。三者同事务是规格第 8 章事务边界与基线第 6.2 节的硬要求。

#### 6.2 锁策略与锁序

统一锁序为：先工单行，再登记行，再处理记录。任何涉及登记行的用例都先对其工单行执行 `select … from service.work_orders where id = $1 and legal_entity_id = … for update`，再对登记行集合执行 `for update`。理由是工单完成的守卫要读全部登记行，而登记行新增会改变该集合，固定锁序把两者串行化并避免与新增登记行之间形成死锁。项目侧同理，先项目行再任务行。

乐观锁：全部可更新表带 row_version，更新语句按基线第 3.7 节写为带 row_version 条件的 UPDATE，受影响行数为 0 即 PLATFORM.CONCURRENCY.STALE_VERSION、HTTP 409，响应回带当前版本号与最后修改人。工单状态流转同时使用悲观行锁与乐观版本校验：行锁保证守卫读到的登记行集合稳定，版本校验保证客户端提交的是它看到的那一版。

序列化失败 40001 与死锁 40P01 由数据访问层统一重试 3 次，退避 50、150、450 毫秒，且只对尚未产生任何外部可见副作用的事务重试。本阶段全部写用例的外部可见副作用只有 Outbox 条目，而 Outbox 与业务写入同事务、回滚即消失，因此全部写用例可重试。

#### 6.3 幂等

- HTTP 写请求：四元组幂等键，存 platform_msg.idempotency_keys，与业务写入同事务，保留 7 天。重放返回首次结果并带 Idempotent-Replay: true；键相同而 request_hash 不同返回 409 与 PLATFORM.IDEMPOTENCY.PAYLOAD_MISMATCH。
- 事件消费：三个消费者各自使用 platform_msg.inbox_consumptions 的 (consumer, event_id) 唯一约束，消费副作用与该行插入同事务。消费者名固定为 project.contract_derivation（消费 clm.contract.effective.v1）、project.requisition_intake（消费 project.project_task.requisition_requested.v1）与 service.return_repair_writeback（消费 service.work_order_line.registered.v1、三个销售退货事件及 sales.delivery.confirmed.v1）。
- 业务层兜底：项目任务按 (legal_entity_id, derivation_unique_key) 唯一；投诉升级按 (legal_entity_id, source_complaint_id) 唯一；采购需求引用按 (legal_entity_id, purchase_requisition_id) 唯一。三者使幂等不只依赖消息层。

#### 6.4 与 Outbox 的关系

本阶段发布的全部事件走 platform_msg.outbox_events，信封字段按基线第 6.1 节完整填写。本阶段的事件不承载会计语义，因此 posting_date 与 accounting_period_id 两个信封字段一律置空，且本阶段不向 ledger.posting_trigger_event_types 登记任何行。按裁定 C-28 的受理前提二判定语句，posting_date 为空且不命中该注册表的事件一律不计入待消费过账条目数，本阶段的事件因此两条都不满足计入条件。这一点需在事件目录中对本阶段的 25 个事件逐条标注为非过账事件，避免关账受理判定把它们计入。

25 个事件的名称、聚合与最小 payload 冻结如下；本表与 `docs/event-catalog.md` 必须集合相等，不存在未命名名额，也不允许实施时用同义名称替换。

| 事件类型 | aggregate_type | 触发时点 | payload 最小字段 |
|---|---|---|---|
| `project.project.created.v1` | `project.projects` | 手工或合同派生首次建立项目 | project_id、doc_no、customer_id、source_contract_id、source |
| `project.project.completed.v1` | `project.projects` | 全部任务终态且项目进入 COMPLETED | project_id、doc_no、completed_at |
| `project.project.closed.v1` | `project.projects` | 全部任务终态且项目进入 CLOSED | project_id、doc_no、closed_at |
| `project.project_task.derived.v1` | `project.project_tasks` | NEW/CHANGED 派生项建立新任务 | project_id、project_task_id、contract_id、contract_version_no、obligation_key、derivation_unique_key |
| `project.project_task.requisition_requested.v1` | `project.project_tasks` | 任务原子进入 requisition_link_state=PENDING | project_id、project_task_id、source_contract_id（可空）、material_id、quantity、required_on、unique_key（固定 `PROJECT_TASK:{project_task_id}`） |
| `service.equipment_record.created.v1` | `service.equipment_records` | 手工或交付批次创建设备档案 | equipment_record_id、code、customer_id、product_id、delivery_confirmation_id |
| `service.equipment_record.status_changed.v1` | `service.equipment_records` | 设备当前状态受控变更 | equipment_record_id、from_status_code、to_status_code、reason |
| `service.equipment_record.warranty_updated.v1` | `service.equipment_records` | 项目经理更新保修信息 | equipment_record_id、warranty_start_on、warranty_end_on、reason |
| `service.customer_complaint.registered.v1` | `service.customer_complaints` | 投诉登记为 REGISTERED | complaint_id、doc_no、customer_id、complaint_on |
| `service.customer_complaint.accepted.v1` | `service.customer_complaints` | 投诉受理并写受理人 | complaint_id、doc_no、accepted_by、accepted_at |
| `service.customer_complaint.closed.v1` | `service.customer_complaints` | 填写处理说明并关闭 | complaint_id、doc_no、handling_note、closed_at |
| `service.customer_complaint.cancelled.v1` | `service.customer_complaints` | 项目经理填写原因并取消 | complaint_id、doc_no、cancel_reason、cancelled_at |
| `service.customer_complaint.escalated.v1` | `service.customer_complaints` | 投诉唯一升级工单成功 | complaint_id、work_order_id、work_order_doc_no |
| `service.work_order.created.v1` | `service.work_orders` | 任一入口创建工单或返修跟进单 | work_order_id、doc_no、customer_id、source_complaint_id、follow_up_of_work_order_id |
| `service.work_order.submitted.v1` | `service.work_orders` | DRAFT 进入 PENDING_ACCEPTANCE | work_order_id、doc_no、submitted_at、due_at |
| `service.work_order.assigned.v1` | `service.work_orders` | 写入受理人并进入 IN_PROGRESS | work_order_id、doc_no、assignee_user_id、accepted_at |
| `service.work_order.customer_confirmation_requested.v1` | `service.work_orders` | IN_PROGRESS 进入 PENDING_CUSTOMER_CONFIRM | work_order_id、doc_no、requested_at |
| `service.work_order.processing_resumed.v1` | `service.work_orders` | PENDING_CUSTOMER_CONFIRM 回到 IN_PROGRESS | work_order_id、doc_no、resumed_at |
| `service.work_order.completed.v1` | `service.work_orders` | G1/G2 守卫通过并进入 COMPLETED | work_order_id、doc_no、conclusion_note、completed_at |
| `service.work_order.cancelled.v1` | `service.work_orders` | G3 守卫通过并进入 CANCELLED | work_order_id、doc_no、cancel_reason、cancelled_at |
| `service.work_order.follow_up_created.v1` | `service.work_orders` | COMPLETED 原工单建立新的 DRAFT 跟进单 | source_work_order_id、follow_up_work_order_id、follow_up_doc_no |
| `service.work_order_line.registered.v1` | `service.work_order_lines` | RETURN/EXCHANGE 提交或 REPAIR 行建立 | work_order_id、work_order_line_id、handling_method、quantity、sales_order_line_id、return_posting_date（REPAIR 为空）、return_warehouse_id（可空）、replacement_delivery_schedule_id |
| `service.work_order_line.linked.v1` | `service.work_order_lines` | 退换行权威销售关联建立并进入 LINKED | work_order_id、work_order_line_id、sales_return_id、sales_return_line_id、replacement_delivery_schedule_id |
| `service.work_order_line.completed.v1` | `service.work_order_lines` | REPAIR 人工完成或退换终态守卫满足 | work_order_id、work_order_line_id、handling_method、completed_at |
| `service.work_order_line.voided.v1` | `service.work_order_lines` | 项目经理填写原因并把非终态行置 VOIDED | work_order_id、work_order_line_id、void_reason、voided_at |

上述五个 project 事件加二十个 service 事件恰为 25。每行 `produces_voucher=false`、`posting_date=null`、`accounting_period_id=null`；代码常量集合、阶段表与事件目录由 `xtask eventcatalog` 做三方集合相等检查，数量相等但名称不等同样失败。

取件、批量 100、轮询 200 毫秒、退避 8 档、死信与重投一律沿用基线，不另建机制。

#### 6.5 失败重试与补偿

| 失败点 | 处理 |
|---|---|
| 派生项目任务时 clm 的派生项查询不可用 | 按 EXTERNAL 之外的 INFRASTRUCTURE 处理，事件退避重试；八次后进死信 |
| 创建销售退货单被 sales 拒绝（数量、状态、法人） | 登记行退回 PENDING，写审计，向处理人发站内通知，不重试；错误码 SERVICE.WORK_ORDER_LINE.SALES_RETURN_REJECTED |
| 创建销售退货单超时但对方可能已成功 | 由 sales 的命令端口幂等键保证不重复创建，重试安全；超过八次退避后进死信并人工修复 |
| 销售退货单终态事件迟到或乱序 | 回写用例做状态收敛：只允许 LINKED → COMPLETED 与 LINKED → PENDING，非法迁移记 WARN 并置 DONE，不进死信 |
| 采购需求创建失败 | `intake + link + LINKED` 同事务回滚，任务保持 PENDING 并按八档退避；第九次失败与 Outbox DEAD/死信同事务把任务置 FAILED 并保存清洗后的 last_error，绝不再从死信临时推断业务状态 |
| 站内通知发送失败 | 平台侧重试，不阻断本阶段任何状态流转，取值按 PRD 9.9 |

本阶段没有需要补偿的多步写编排：登记行与销售退货单之间是引用关系而非资金或库存后果，销售退货单被作废时登记行退回 PENDING 即为完整的反向路径，不需要 Saga 补偿分支。

---

### 7. 配置项

全部键按基线第 7.1 节前缀 EP__、双下划线分层，结构体开启 deny_unknown_fields。运行期可变的业务参数不进配置文件，因此工单提醒阈值、四张字典的取值、列表默认列均落在数据库并经配置发布通道发布。该通道按裁定 A-27 由阶段 3b 提供，本阶段只作为使用方接入，不自建第二套发布路径。

| 键 | 类型 | 默认值 | 生效方式 | 说明 |
|---|---|---|---|---|
| EP__SERVICE__WORK_ORDER__MAX_LINES_PER_ORDER | u16 | 200 | 启动时读取，改动需重启 core-server | 与基线批量上限 200 对齐 |
| EP__SERVICE__WORK_ORDER__REMINDER_TIMER_ENABLED | bool | true | 启动时读取 | 关闭后不登记时限定时器，用于迁移窗口 |
| EP__SERVICE__EQUIPMENT__CREATE_FROM_DELIVERY_MAX_ROWS | u16 | 200 | 启动时读取 | 单次从交付确认单建档的上限 |
| EP__PROJECT__DERIVATION__MAX_TASKS_PER_CONTRACT | u16 | 500 | 启动时读取，core-server 与 job-worker 取同值 | 单次派生的任务条数上限 |
| EP__PROJECT__DERIVATION__PLAN_QUERY_TIMEOUT_MS | u32 | 3000 | 启动时读取 | 读取合同派生项的超时 |
| EP__CRM__CUSTOMER_360__DEFAULT_SECTION_SIZE | u16 | 20 | 启动时读取 | 未传 section_size 时的默认值，取值来源见第 12 节 U-J-15 |
| EP__CRM__CUSTOMER_360__MAX_SECTION_SIZE | u16 | 50 | 启动时读取 | 请求超过即 `PLATFORM.REQUEST.INVALID_PAYLOAD` |
| EP__CRM__CUSTOMER_360__SECTION_TIMEOUT_MS | u32 | 1500 | 启动时读取 | 单区块超时，超时即 DEGRADED |
| EP__CRM__CUSTOMER_360__PROVIDER_CONCURRENCY | u8 | 5 | 启动时读取 | 区块扇出并发上限，不超过只读分析池上限 10 的一半 |

本阶段不引入新的机密引用，不改动机密库结构。本阶段在启动自检中不新增检查项；按裁定 C-25 自检项一律按注册名标识，基线项 rls-enabled-and-forced 只读系统目录，自然覆盖本阶段新增的 22 张表。与本阶段有关的唯一自检项 reporting-dataset-signature-matched 归阶段 11，按 3.7 的口径为降级级，其判读结果只决定报表入口的开闭，不决定任何进程能否启动。

---

### 8. 测试计划

#### 8.1 单元测试

位于被测 crate 内，不触网、不触库、不触文件系统、不取真实时间，时间一律经 FixedClock 注入。

- 在保状态判定：4 个取值的正例各 1 条；边界 6 条（judge_on 等于 start、等于 end、start 等于 end、start 为空、end 为空、两者均为空）；proptest 属性 3 条（结果必落在四取值之一；start 与 end 均非空时四取值互斥且覆盖全体日期；把 judge_on 沿时间轴推进，结果序列只能按 NOT_STARTED → IN_WARRANTY → EXPIRED 单调前进）。
- 工单状态机：6×6 共 36 组迁移逐一断言，其中合法 10 条、非法 26 条；守卫 G1、G2、G3 各覆盖通过与拒绝两条；COMPLETED/CANCELLED 到任意状态均拒绝，返修跟进只新建工单而不改变原状态。
- 投诉状态机 4×4 共 16 组；登记行状态机 4×4 共 16 组；任务状态机 4×4 共 16 组；项目状态机 3×3 共 9 组。
- 可退数量：等于边界通过、超出最小单位（1e-6）拒绝、已作废行不计入、多行累加、已交付为零时任何数量均拒绝、负数与零拒绝。
- 派生版本比较函数：NEW、UNCHANGED、CHANGED、REMOVED 四类，分别覆盖旧任务非终态与终态；断言终态不修改，CHANGED 新建补充任务，REMOVED/CHANGED 的旧非终态只置 stale 并产生处置事项；同批 obligation_key 重复、hash 非法与 items 超上限均拒绝。
- `CLM_TERM_PROJECT_TASK`：assess 对 NOT_STARTED/IN_PROGRESS 分别产出 AUTO_CANCEL/MANUAL_DECISION，终态、异法人和异合同均不命中；dispose 的 NOT_STARTED、终态、并发变为 IN_PROGRESS 三支分别精确返回 `Completed`、`AlreadySatisfied`、`NeedsManualDecision` 及三个冻结 reason。人工分支逐项断言 `PROJECT_TASK_COMPLETED`/`PROJECT_TASK_CANCELLED` 两码、非空理由、结果 id 必填且同目标任务、COMPLETED/CANCELLED 状态匹配；错码、空理由、异任务 id 与状态错配均不得闭合。
- 客户 360 合并：截断到 size、排序稳定性、区块超时降级、未注册区块返回 NOT_AVAILABLE、无权区块返回空且状态为 OK。
- 编号格式与文本长度校验：各类型码前缀正确、长度超限返回 `PLATFORM.REQUEST.INVALID_PAYLOAD` 且定位到字段。

工具为 cargo test、rstest 参数化、insta 快照（错误响应体）、proptest（在保判定与可退数量两组）。

#### 8.2 集成测试

使用真实 PostgreSQL 16，每个用例独占一个 ep_test_<nanoid> 数据库，用例结束删库；数据一律经 ep-testkit 构造器与用例路径产生，禁止手写 INSERT。

场景清单：

1. 设备三条建档路径各一条：手工、从交付确认单批量、迁移接口写入并带迁移批次标识。
2. 从交付确认单重复建档：同一交付确认行第二次提交被识别为已建档并跳过，返回跳过清单。
3. 保修信息修改：`PROJECT_MANAGER` 可改、`TECHNICIAN` 被拒（403）、修改写审计并保留修改前取值、不回溯改写既有工单的在保快照。
4. 设备终止状态：未确认时 409、`PROJECT_MANAGER` 确认后可创建工单、确认写审计。
5. 投诉全状态路径：登记 → 受理 → 关闭；登记 → 取消；终态修改被拒。
6. 投诉升级：成功一次；同一投诉第二次升级返回 409 并回带既有工单编号。
7. 工单三个创建入口产生的对象与状态机一致（同一断言集跑三遍）。
8. 关联一致性七条校验逐条命中：法人不一致、对象不可见、客户不一致、设备带出、订单行带出、终止状态、允许为空。
9. 登记行三类处理方式：RETURN/EXCHANGE 均把 return_posting_date、可空 return_warehouse_id、remark 与 `{SERVICE,WORK_ORDER_RETURN_REQUEST,work_order_line_id,event_id}` source_ref 逐值传给真实 SalesReturnCommandPort，显式使用 AutoFifo 且命令 delivery_links 为空；sales 按 confirmed_at/id FIFO 返回恰一行及已解析 links，service 只用返回的 sales_return_line_id。同一事件重复投递命中 source_ref 唯一键不重复建退货，取消/驳回后的新事件可建立新的退货尝试。EXCHANGE 再经真实 SalesExchangeLinkCommandPort 在 sales.exchange_links 建权威配对并在 service 同时写两侧引用，分别只给一侧均被拒且两模块零写入，客户或产品不同被拒；REPAIR 直接完成。只退用 RETURN，只补发走 sales 独立动作，不出现伪 EXCHANGE。
10. 回写：RETURN 由 sales.sales_return.closed.v1 完成；EXCHANGE 分别覆盖退货先终态、替换发货先终态及重复乱序，两侧都终态后且仅此时完成；cancelled/rejected 清空 EXCHANGE 两侧并退回 PENDING。所有事件重复 3 次只写一次时间、审计和通知。
11. 工单完成守卫：存在 PENDING 行时被拒并回带清单；全部行终态后通过。工单取消守卫同理。
12. 派生：首版合同建 N 条任务与 1 个项目；下一版本构造 NEW/UNCHANGED/CHANGED/REMOVED 各一项，断言新增和改变各建新版本唯一键的补充任务、未变不复制、删除和改变的旧非终态置 derivation_stale 且各产生一条 PROJECT_MANAGER 处置事项、旧终态永久原样保留；负责人显式完成或取消 stale 任务。重复事件 5 次不重复任务/事项/事件/审计；续签复用同一项目，项目原为 CLOSED 时仅消费者恢复 IN_PROGRESS 并记录审计。
13. 派生失败：派生项查询持续失败，八次退避后进死信，死信按法人可枚举，重投成功后任务正确。
14. 项目任务提交采购需求：首次提交把 material_id/quantity/required_on 快照、PENDING 与事件同事务，`PurchaseRequisitionIntakePort::intake + 双向 link + LINKED` 与 Inbox 同一消费者事务；合同派生任务和无合同手工任务各一例，后者的 source_contract_id/采购 contract_id 均为空但 project_id 必填。同一任务换三个 HTTP 幂等键时采购来源键始终为 `PROJECT_TASK:{project_task_id}`，procure 只建一单；FAILED 后相同快照可重试，任一快照字段改变精确返回 PAYLOAD_MISMATCH 且保持 FAILED。`(legal_entity_id,project_task_id)` 与 `(legal_entity_id,purchase_requisition_id)` 两个唯一键分别拦第二张需求和重复回写。夹具驱动第九次失败时断言 Outbox DEAD、dead_letters 与任务 FAILED/last_error 同事务；重投成功由 FAILED 收敛到 LINKED，不使用任何替身实现。
15. 客户 360：三个自实现区块返回正确数据；未注册的合同与回款区块返回 NOT_AVAILABLE；人为注入超时的区块返回 DEGRADED 且其余区块正常。
16. 处理记录只追加：ACTION 与 CORRECTION 两种合法形状追加成功；真实 PostgreSQL 下使用运行账号直接执行 UPDATE 与 DELETE 均被统一 guard 拒绝，原行 checksum/行数不变，同时运行共享 `append_only_consistency.sql`，断言 registry 恰有一条 `APPEND_ONLY, mutable_columns={}` 记录且 guard 目录逐项一致；应用 SQL 的 UPDATE/DELETE 另由 CI 静态检查拦截。更正说明经同法人同工单复合自外键关联；跨工单父记录、自指、两行成环及 ACTION/非空父、CORRECTION/空父五类反例均由 PostgreSQL 约束拒绝。迁移回退测试另断言先 detach guard、删 registry 行再 DROP 表，回退后两处目录均无悬空项。
17. 附件关联：四张附件表的挂接与解除挂接、附件正文不落业务表列。
18. 字段级加密：联系方式写入为密文、按该字段过滤与排序的请求返回 `PLATFORM.AUTHZ.SORT_FIELD_FORBIDDEN`、日志与错误响应中不出现明文。
19. 受治理数据集视图：project.v_projects_dataset 存在且含 legal_entity_id、security_level、data_scope_tags 三列；ep_analyst_ro 可 SELECT 而任何写语句被拒；视图列名与类型签名与 reporting.dataset_fields 的登记逐列一致。
20. 序列号权威：同一法人的 inventory.serial_states 重复 serial_no 被库存唯一索引拒绝；service 三张业务表只存在 inventory_serial_state_id 而不存在 serial_no 列；同一 serial state 绑定第二台设备被唯一索引拒绝；扫码只解析已有库存记录，不存在时不创建第二命名空间。
21. 工单返修跟进：COMPLETED 可连续创建三层新 DRAFT 且每层 follow_up_of_work_order_id 正确、原工单与日志校验和不变；CANCELLED 调用拒绝，普通新建来源为空。
22. `CLM_TERM_PROJECT_TASK` 真实规则：同一合同各造 NOT_STARTED、IN_PROGRESS、COMPLETED、CANCELLED、异合同与异法人任务，断言只为前两项建处置项；自动项取消任务并写固定原因与一次审计。人工项分别先经现有动作把目标置 COMPLETED/CANCELLED，再提交匹配的 `PROJECT_TASK_COMPLETED`/`PROJECT_TASK_CANCELLED`、非空 reason 与等于目标任务的 result id，才分别以两个稳定 reason 闭合。两码互换、空理由、空/异任务 result id 均拒绝；重复 dispose 三次不重复改写、审计或 HUMAN_TASK。
23. assess/dispose 竞态：先以 NOT_STARTED 产出 AUTO_CANCEL，再在 dispose 加锁前把任务推进到 IN_PROGRESS，断言结果为 `NeedsManualDecision`、attempts 不增加、无 dead letter，平台同事务把项转 MANUAL_DECISION、只建一条分配到 PROJECT_MANAGER 的 HUMAN_TASK；目标仍在制时重复推进保持 PENDING。并发变为 COMPLETED/CANCELLED 后，只有与锁后状态匹配的决策码与同任务 result id 能闭合，决策三字段持久到处置项而非只存 process task outcome。
24. 项目任务—采购需求链接数据库闭环：使用真实 PostgreSQL 分别以“先插 link 后置 LINKED”和“先置 LINKED 后插 link”两种事务内写序提交成功；`LINKED` 无 link、`NULL/PENDING/FAILED` 各自有 link、删除 LINKED 的 link、把有 link 的任务改回非 LINKED、跨法人链接及同任务第二条 link 均在 COMMIT 被数据库拒绝。另查询 `pg_constraint/pg_trigger` 断言任务表与 link 表的两个约束触发器均为 `DEFERRABLE INITIALLY DEFERRED`，函数覆盖 INSERT/UPDATE/DELETE，且失败事务不留下半套状态。
25. 三类迁移撤销 owner fact：真实 PostgreSQL 下，project/complaint/equipment 各跑一次“先改根后插 correction”和“先插 correction 后改根”的合法事务并成功 COMMIT；终态 retain 分支不更新根但各追加恰一 correction。逐项负测错法人根、重复 root、错误 mode/status/version/reason、安全属性不一致、项目仍有非终态 task、设备 SET_RETURNED 的 before 字典为终态、设备 RETAIN_TERMINAL 的字典为非终态、只改根不插 correction 后伪造 REVERSE receipt，以及 correction 表 UPDATE/DELETE，均由普通 FK、CHECK、Stage 12 DEFERRABLE 图、APPEND_ONLY guard 或 Stage 14 receipt/R0 图拒绝。`pg_constraint/pg_trigger` 断言三条根 FK 与三组 constraint trigger 均为 DEFERRABLE INITIALLY DEFERRED；共享 `append_only_consistency.sql` 断言三张 correction 与 work_order_logs 四行登记/物理 guard 完全一致。三个既有迁移 rollback 在空库按正文顺序成功且不留 registry/trigger/function 悬空项。

RLS 与越权：本阶段 22 张表全部纳入 tests/rls_matrix，覆盖读取、写入、更新、删除、聚合、排序、报表投影与错误信息泄漏八类；另覆盖两个复制角色与内部对账系统安全上下文的入口借用测试，其断言函数 assert_replication_role_containment 与 assert_recon_context_borrow 由阶段 2 提供，本阶段只调用，不实现同名函数。该测试目标属发布门禁项。

并发：命中基线第 8.4 节六组必测场景中的第一组（同一单据的乐观锁冲突）与第六组（Outbox 同一事件重复投递不少于 3 次），并追加本阶段特有的三组：两个用户并发升级同一投诉（恰好一个成功）；一个用户完成工单同时另一个用户新增登记行（按锁序串行化，后者要么被守卫拒绝要么在完成前入库并使完成被拒，不出现完成后仍有 PENDING 行）；两个用户对同一订单行并发登记退货（本模块前置校验可能同时通过，sales 侧权威校验拒绝其一，被拒的登记行退回 PENDING 且不产生第二张退货单）。

#### 8.3 端到端测试

- E2E-02 退换修打通：RETURN 在退货终态后完成；EXCHANGE 同时挂接 sales_return_line 与 replacement_delivery_schedule，客户产品一致，两侧按任意顺序到终态后才完成；工单、sales.exchange_links、销售退货、替换发货与设备档案追溯双向可达。
- E2E-03 派生幂等与续签处置：合同升版的新增/改变义务生成补充任务，删除/改变的旧非终态置 stale 并产生 PROJECT_MANAGER 处置事项，终态保留；重复投递无重复，失败进死信并可人工修复。
- E2E-04 四端：售后工单与设备台账能力域在 Windows、macOS、iOS、Android 四端按完整取值执行同一场景集；项目任务与交付节点桌面完整、移动简化；移动端相机扫码把 serial_no 经 SerialStateQuery 解析成 inventory_serial_state_id，service 不保存文本。界面代码按裁定 A-23 位于既定模块目录，阶段 13 只提供客户端壳、路由注册表与能力矩阵闸。
- E2E-05 时限提醒：待受理停留超阈值向 `PROJECT_MANAGER` 角色队列送达站内通知；期望完成时间临近与超出向处理人与 `PROJECT_MANAGER` 送达；无移动推送通道的部署下站内通知照常送达。
- E2E-06 客户 360：销售角色查询该客户的历史合同、回款、投诉、设备与服务记录；无权客户返回 404。
- E2E-07 F-10 全量闭环：一份七类处置项俱全的合同进入 TERMINATING，`ImpactRegistry` 已有 7 个真实规则，项目任务中 NOT_STARTED 被自动取消、IN_PROGRESS 先由 PROJECT_MANAGER 经既有动作完成或取消，再提交匹配的 `PROJECT_TASK_COMPLETED`/`PROJECT_TASK_CANCELLED`、非空理由与同任务 result id；平台保存三字段后闭合该项。全部七类项闭合后合同到达 TERMINATED，`clm.contract.termination_completed.v1` 恰好一条。终止端点以同一幂等键重放 3 次仍只有一个批次。
- E2E-08 F-10 反向闭环：故意令 `CLM_TERM_PROJECT_TASK` 人工项保持 PENDING，断言合同不到 TERMINATED、批次保持 RUNNING、SLA 定时器到点写超时并产生一条流程时限提醒；空理由、错码、空/异任务 result id 与状态错配均被拒绝且三决策字段不落库。再让该项连续失败至 DEAD，断言批次 FAILED、合同仍 TERMINATING；记名 replay 后以合法命令闭合，completion 事件仍恰一次。至少保留“PENDING 不闭合、人工命令形状/语义错误不闭合、DEAD 不闭合”三类真正的否定断言。

#### 8.4 性能相关项

在 ep-datagen 的 A.3 基准数据集（另加本阶段追加的设备、工单、投诉、项目、任务数据）与附录 A.4 的 20 并发负载下：

- 售后工单创建按附录 A.1 普通交易提交度量项判定，P95 ≤ 3 秒；客户投诉登记共用该度量项。
- 按附录 A.1 末段允许新增度量项的规则，新增一个常规交互度量项“客户 360 视图加载”，通过线沿用规格第 16 章常规交互 P95 ≤ 2 秒，不改动既有通过线。
- 工单列表、投诉列表、设备列表、按设备查工单、按订单或合同查工单、登记行清单、项目任务列表七个查询逐一给出 EXPLAIN 证据，在基准数据集上不得出现顺序扫描。
- 每场景样本不少于 200 次，只取负载稳定段，单次运行错误率超过 0.1% 即该次运行无效。
- 时延与容量通过线的最终判定在阶段 4 统一执行，本阶段只需给出本地实测证据与 EXPLAIN 证据。

#### 8.5 不变量与边界测试

- 执行本阶段全部集成与 E2E 用例前后，用 ep-platform-recon 的语句集在同一 REPEATABLE READ 快照上核对规格第 17.3 章的库存数量守恒、存货金额账与数量账一致、子账与总账勾稽三项，差额为零且取值不变。
- 直接断言本阶段全部集成与 E2E 用例前后，ledger.vouchers、ledger.voucher_lines 与 inventory 的数量流水、金额流水四张表的行数与校验和不变，即本阶段确未生成任何总账凭证与库存流水。这是 PRD 9.11 第四条验收要点的可执行形式。
- 审计断言：PRD 9.10 列出的八类必须留痕动作各产生且仅产生一条审计事件，事件与业务变更同事务，哈希链在该日期段上验证通过。

#### 8.6 覆盖率门槛

| 范围 | 门槛 | 依据 |
|---|---|---|
| ep-domain-service、ep-domain-project | 行覆盖率 ≥ 85% | 本阶段自设更高门槛，理由是状态机与守卫是 PRD 9.11 边界成立的判定点 |
| ep-app-service、ep-app-project、ep-app-crm 的 360 用例 | ≥ 80% | 规格第 17.2 章其余代码 70% 之上，按基线新增代码 80% 取值 |
| ep-adapter-db-pg 的两个仓储目录 | ≥ 70% | 规格第 17.2 章其余代码 |
| 本阶段新增与修改代码 | ≥ 80% | 规格第 17.2 章 |
| 工作区整体 | 不低于 80% | 基线第 8.2 节 |

工具为 cargo-llvm-cov，CI 以 --fail-under-lines 强制，分档由 codecov.toml 的路径规则表达。本阶段不允许 #[ignore]，确需跳过的必须带 issue 编号且存活不超过本阶段。

---

### 9. 退出条件

下列 25 项全部达成才算本阶段完成，每项均可客观判定。

1. 22 张表及两个项目外键晚绑定文件构成的 16 个迁移版本由 ADR-0013 冻结的 ep-migrate 自建 Runner 在空库上按版本全序执行成功，回退段可执行，16 个版本均且仅写入单一 `schema_history`，不创建或读取任何第二套迁移历史表；三张 correction 表复用 090000/090600/090700，project correction 图在 tasks 建成的 090100 安装，不占第 17 个版本；090030/090040 必须位于 project.projects 建成后、任何项目维度写入口启用前。
2. 22 张表全部 ENABLE 且 FORCE 行级安全，策略名与模板一致；启动自检项 rls-enabled-and-forced 通过。
3. tests/rls_matrix 中本阶段 22 张表的八类越权用例全绿，无内容回显、无排序与聚合侧信道。
4. 54 个 HTTP 端点全部具备集成测试且全绿，封套、分页、排序白名单、过滤运算符、幂等头四项由统一的契约测试断言。
5. 25 个事件在 docs/event-catalog.md 登记，命名为四段过去分词形式，信封字段完整，并逐条标注为非过账事件。
6. 全部错误码在 docs/error-codes.md 与 ep-foundation::error::codes 两处登记且一致，CI 校验通过，代码中不内联中文文案。
7. 五个新增指标在基线第 9.2 节登记并由 ops-agent 暴露。
8. 工单六状态、投诉四状态、登记行四状态、任务四状态、项目三状态的全部迁移组合有单元测试断言；项目 CLOSED → IN_PROGRESS 仅 project.contract_derivation 的补充义务分支可用，其余非法迁移一律 409。
9. 在保状态判定的四取值与六个边界有测试，且属性测试通过。
10. 投诉最多升级一次由数据库唯一索引保证，并有并发用例证明恰好一个成功。
11. 派生幂等与版本处置：同一事件重复投递 5 次不重复；NEW/CHANGED 以新版本 unique_key 建补充任务，UNCHANGED 不复制，REMOVED/CHANGED 的旧非终态只置 derivation_stale 并各建一条 PROJECT_MANAGER 处置事项，终态永久不改；旧任务只由负责人显式完成或取消；续签复用同一项目。
12. 派生失败进入死信并可记名重投，死信按法人可枚举。
13. 三条追溯链路双向可达；RETURN/EXCHANGE 自动路径均以 AutoFifo 调用 A-17 现行 exact DTO，逐值断言 return_posting_date、可空 return_warehouse_id、remark、按 event_id 稳定的 source_ref、空输入 links 及返回的 sales_return_line_id/已解析 links；同一事件重复三次只建一单，取消/驳回后新事件使用新 source_ref 并建新行。EXCHANGE 在 sales.exchange_links 与 service 登记行同时存在成对引用、客户产品一致且两侧都终态后才完成，单侧挂接、只退伪装换货、只补发伪装换货均被测试拒绝。DRAFT/SUBMITTED 销售退货可取消，REGISTERED 取消被拒且 service 不等待该不存在的分支。
14. 闭环第 12 步的用例片段已交付为 testkit/scenarios/stage12_service_step12.rs 中的步骤函数与断言，其自身在本阶段单独跑通，并可被阶段 9b 的 testkit/scenarios/golden_loop_14_steps.rs 引用；整条链路的串接通过由阶段 9b 的该用例承担，不在本阶段判定。
15. 执行本阶段全部用例前后，规格第 17.3 章三项不变量取值不变，且凭证与库存流水四张表的行数与校验和不变。
16. 四端 E2E 按规格第 6.2 章矩阵取值通过：售后工单与设备台账四端完整，项目任务与交付节点桌面完整、移动简化；移动扫码只解析 inventory.serial_states 并保存 id，service schema 无 serial_no 权威列；COMPLETED 返修跟进新建工单且 CANCELLED 不可跟进。
17. 覆盖率达到 8.6 节的五档门槛。
18. 本阶段新增决定（第 13 节）已回写共享技术基线，第 12 节已关闭事项的冻结值已固化在约束、单一规则函数或受控字典中，不存在首版反向开关。
19. 本模块在规格第 6.2 章能力矩阵中取值为完整或简化的能力域，其四端界面已实现并通过 Playwright 与 tauri-driver 的桌面用例、XCUITest 与 Espresso 的移动用例；取值为 VIEW_ONLY 的能力域只实现只读视图；取值为 NOT_APPLICABLE 的不实现入口。
20. project.v_projects_dataset 已发布并授予 ep_analyst_ro，dataset code 为 project_projects，列签名已同步给阶段 11，且阶段 11 的启动自检项 reporting-dataset-signature-matched 对该视图不再开降级窗口；该自检项在任何取值下都不阻断启动。
21. 本阶段全部 /api/v1/ 路由的能力域码与动作类别常量已在 crates/contract/service/src/capability.rs 与 crates/contract/project/src/capability.rs 中声明，xtask configdoc 通过。
22. 本模块的 MasterReferenceCounter 实现 ServiceReferenceCounter 已实现并注册进阶段 5 提供的 MasterReferenceCounterRegistry；按裁定 A-15 的实现清单，本阶段不承担任何 TradeHistoryProvider。档案停用引用计数按注册表实时枚举判定，本阶段注册后即时生效，不设顺延登记项，阶段 5 的相应验收不再顺延到本阶段结束。
23. `ContractTerminationProjectTaskImpactRule` 已作为 code=`CLM_TERM_PROJECT_TASK` 的第七个真实规则注册，目录条数与注册数均为 7，wiring 中无 ImpactRule 替身且不存在项目侧合同终止消费者。`PROJECT_TASK_COMPLETED`/`PROJECT_TASK_CANCELLED` 两码、非空理由、同任务必填 `decision_result_doc_id`、锁后 COMPLETED/CANCELLED 状态语义与决策三字段持久化均经真实 PostgreSQL 正反例验证。第 8.2 节场景 22、23 与 E2E-07、E2E-08 全绿；全量闭合后合同才到 TERMINATED 且 `clm.contract.termination_completed.v1` 恰一次，PENDING、人工命令形状/语义错误、DEAD 三类反向分支均能阻止闭合。
24. `service.work_order_logs` 的 APPEND_ONLY 机制通过共享 `append_only_consistency.sql`：registry 模式与空 mutable_columns、统一 guard 目录和物理触发器一致；运行账号 UPDATE/DELETE 真实被拒，ACTION/CORRECTION INSERT 可提交。`V20261021091000` rollback 在临时库按 detach guard→删 registry→DROP 表顺序执行成功且无悬空目录项。
25. `project.project_migration_corrections`、`service.customer_complaint_migration_corrections`、`service.equipment_migration_corrections` 三个具名 owner fact 与第 4.12 节三命令全部实现；合法改变/终态 retain、任意事务内写序、全部 direct-SQL 反例、RLS、APPEND_ONLY、三组 DEFERRABLE 图、Stage 14 correction target/R0/receipt 联合图及三个既有迁移 rollback 测试全绿。任一迁移 REVERSE 仍能只改根、借通用 JSON/reversal 表、复用旧 correction、缺具名 fact 或删除历史时不得退出。

---

### 10. 与规格和 PRD 的对应

#### 10.1 规格条目

| 规格章节 | 条目 | 本阶段实现的部分 |
|---|---|---|
| 第 5.2 章 售后与 FSM | 工单及其与原订单、合同、产品、批次、设备和保修的关联 | 4.5 的七条一致性校验与 work_orders 的七个关联列 |
| 第 5.2 章 售后与 FSM | 客户投诉记录 | service.customer_complaints 与 5.2 的八个端点 |
| 第 5.2 章 售后与 FSM | 退换修登记并与订单的退货换货打通 | service.work_order_lines 与 4.6 |
| 第 5.2 章 售后与 FSM | 设备台账：编号或序列号、型号、所属客户、关联产品与批次、交付与安装日期、当前状态、保修起止与范围与条款文本、在保状态判定 | service.equipment_records 的全部列与 4.2 |
| 第 5.2 章 售后与 FSM | 流程时限提醒由第 5.1 章低代码流程的时限能力承担 | 5.3 的 submit 端点登记定时器，本阶段不建 SLA 引擎 |
| 第 5.2 章 项目与 PPM | 项目实体与项目任务 | project.projects、project.project_tasks |
| 第 5.2 章 项目与 PPM | 由项目触发的采购需求 | 4.9 与 project_task_purchase_requisition_links |
| 第 5.2 章 项目与 PPM | 按项目控制访问的权限维度 | data_scope_tags 写入 project:<项目编号>，判定由 ep-platform-authz 承担 |
| 第 5.2 章 CRM | 客户 360 视图，可查询历史合同、回款、投诉、设备和服务记录 | 4.10 与 5.5，本阶段实现三个区块并定义五区块契约 |
| 第 5.6 章 | 禁止跨模块直接读写业务表，跨模块只使用公开契约与版本化事件 | 全部跨模块交互经 contract trait 与 Outbox 事件，仓储按 schema 分文件 |
| 第 6.2 章 | 售后工单与设备台账四端完整；项目任务与交付节点桌面完整、移动简化；移动端相机扫码 | E2E-04 |
| 第 7.2 章 | 售后模块拥有工单、投诉、设备台账与保修信息；已过账分录与库存流水只追加 | 本阶段不写 ledger 与 inventory，处理记录只追加 |
| 第 7.9 章 | 派生存储写入必须携带 security_level 与 data_scope_tags | 全部事件信封填写两项，检索文档投影按两项裁剪 |
| 第 7.10 章 | 历史导入经模块迁移接口、带迁移批次标识 | equipment_records 的 source=MIGRATION 与 migration_batch_no |
| 第 8 章 第 12 步 | 售后工单闭环步骤 | E2E-01 |
| 第 12.2 章 | 法人、部门、岗位、项目、客户、记录和字段级权限；策略默认拒绝 | 端点权限码与 data_scope_tags 供给 |
| 第 12.5 章 | 业务变更与审计事件同事务 | 全部写用例，PRD 9.10 的八类动作 |
| 第 15.1 章 | 五类错误分类与四要素 | 本阶段错误码表 |
| 第 15.2 章 | 可靠任务、幂等、死信与人工修复 | 6.3 与 6.5 |
| 第 16 章、附录 A.1 | 售后工单创建的普通交易提交度量项 | 8.4 |
| 第 17.2 章 | 单元、领域属性、集成与契约、四端 E2E、身份与访问控制测试 | 第 8 节 |
| 第 17.3 章 | 三项强制不变量在本阶段操作前后不变 | 8.5 |
| 第 19 章 阶段 3 | 项目任务与交付节点、售后工单与设备台账两个建设条目 | 本阶段全部交付物 |

本阶段不承担的规格条目：交付节点的定义与确认动作在 CLM，交付指标口径在报表与经营看板，销售退货单与换货的发货侧在销售与 OMS，采购需求本身在采购与 SRM，成本归集在财务。

#### 10.2 PRD 条目

| PRD 节 | 内容 | 本阶段的落点 |
|---|---|---|
| 9.1.1 | 五类对象 | 全部实现 |
| 9.1.2 | 首版不含的能力 | 第 0 节明列，代码中无对应入口 |
| 9.1.3 | 与账务、库存的边界 | 4.11 与 8.5 |
| 9.1.4 | 与其他节的接缝 | 第 2.1 节的 contract 依赖与 needs 数组 |
| 9.1.5 | 四端与非功能口径 | E2E-04 与 8.4 |
| 9.2 | 七类操作者角色 | 端点权限码，无高风险操作 |
| 9.3.1 | 三条建档路径 | 5.1 的三个入口 |
| 9.3.2 | 设备输入字段 | equipment_records 逐列对应 |
| 9.3.3 | 在保状态判定与两个读取时点 | 4.2 |
| 9.3.4 | 当前状态的语义约束与终止状态确认 | 字典的 is_terminal 与 4.5 第 6 条 |
| 9.3.5 | 保修信息维护 | update-warranty 端点，仅 `PROJECT_MANAGER`，不回溯 |
| 9.3.6 | 三个读取方 | 按裁定 B-06 只保留三条路径：GET /api/v1/service/equipments 与 /{id}、全文检索索引中 object_type 为 service.equipment_records 的文档、本阶段自实现的 EquipmentsSectionProvider；不设 ep-contract-service::EquipmentQuery |
| 9.4.1–9.4.3 | 投诉入口、字段、状态机 | 5.2 与状态机表 |
| 9.4.4 | 投诉与工单的关系 | 4.3 |
| 9.5.1–9.5.4 | 工单入口、字段、校验、状态机 | 4.4、4.5、5.3 |
| 9.5.5 | 处理记录追加型 | work_order_logs 与 5.3 |
| 9.5.6 | 时限与提醒 | 定时器登记与 E2E-05 |
| 9.5.7 | 工单不产生的四类后果 | 4.11 与 8.5 |
| 9.6.1–9.6.6 | 退换修三类方式、状态机与回写、追溯 | 4.6 与 E2E-02 |
| 9.7.1–9.7.6 | 项目任务来源、字段、状态机、采购需求、重新派生 | 4.7、4.8、4.9 |
| 9.7.7 | 项目维度边界 | 只作访问维度与成本归集维度，不向经营驾驶舱供数 |
| 9.8 | 七个查询 | 5.1 至 5.4 的列表端点与索引 |
| 9.9 | 十二类失败场景 | 错误码表逐条对应 |
| 9.10 | 审计与留痕 | 8.5 的审计断言 |
| 9.11 | 六条验收要点 | 退出条件第 13 至 17 项 |

---

### 11. 风险与预留

#### 11.1 技术风险

| 编号 | 风险 | 影响 | 缓解 |
|---|---|---|---|
| R-01 | sales 的销售退货单命令端口与终态事件在联调时行为与冻结签名不符 | E2E-02 与退出条件第 13 项延后 | 阶段 6 排在本阶段之前，本阶段开工即接真实实现，不设替身也不设顺延；契约逐字段锁定 A-17 F-54 supersession 的 remark、allocation_mode、source_ref 与返回 lines/sales_return_line_id，AutoFifo 和 REGISTERED 不可取消有跨模块契约测试，三个终态事件用事件夹具驱动本侧合法分支 |
| R-02 | CLM 的 obligation_key 或 obligation_hash 生成不稳定 | 未改变义务被误判为 CHANGED，产生多余补充任务与处置事项 | 两值只由阶段 6 按 RFC 8785/SHA-256 生成，本阶段不重算；契约测试用字段顺序变化、同值 JSON 与真实字段变化三组向量固化，重复 obligation_key 整批 fail-closed |
| R-03 | 客户 360 的区块注册顺序依赖阶段 5 已建立的契约与端点 | 区块缺位时用户看到 NOT_AVAILABLE | 归属按裁定 C-09 已定死：唯一端点与唯一契约由阶段 5 建立，本阶段只追加三个区块实现，不新增路径；未注册区块显式返回 NOT_AVAILABLE 而非报错 |
| R-04 | service 误写第二份序列号文本或绕过库存解析 | 序列号权威分叉，法人内唯一失效 | service 三张表只存 inventory_serial_state_id，DDL 快照测试禁止 serial_no 列；库存唯一索引加设备引用唯一索引双重约束，所有扫码输入只经 SerialStateQuery |
| R-05 | 受控字典发布错误地改码或删除已引用行 | 历史显示与统计维度断裂 | 四张 service 字典经签名配置发布，只允许新增、改显示名/排序、停用；被引用 code 不可修改，任何行不得物理删除，同 schema 复合外键兜底；MDM 退货原因由 MDM 自身相同治理规则保证 |
| R-06 | 客户 360 在 20 并发下扇出五个区块，可能击穿常规交互 2 秒通过线 | 附录 A 判定不过 | 区块并发上限与单区块超时可配；三个自实现区块各命中一条索引，单次取数在毫秒级；必要时把 section_size 默认下调到 10，代价是一次配置变更 |
| R-07 | 工单完成守卫的锁序若被后续用例破坏会产生死锁 | 偶发 40P01 | 锁序写入 ep-app-service::tx 的注释与一条集成测试（并发完成与并发新增登记行）固化 |
| R-08 | 联系方式字段级加密的密钥版本或密文格式实现漂移 | 无法轮换或跨版本解密 | 严格复用平台信封加密格式与 KmsBackend，service 只定义语义列，不自建密文格式；密钥版本写 key_ref，兼容性进契约测试 |
| R-09 | 平台统一附件上限内仍有大量附件元数据 | 工单详情时延 | 详情端点只返回附件元数据分页，不返回正文；正文经平台附件通道单独取用，首版不设第二个业务侧条数上限 |
| R-10 | 项目任务提交采购需求采用两段式，PENDING 窗口或最终失败可能诱发重复提交 | 重复采购需求或界面长期误显示“提交中” | task 持久化 PENDING/LINKED/FAILED 与清洗后的 last_error；来源键固定 `PROJECT_TASK:{project_task_id}`，并以 `(legal_entity_id,project_task_id)`、`(legal_entity_id,purchase_requisition_id)` 双唯一约束兜底；PENDING 返回专用 409，最终死信原子置 FAILED |

#### 11.2 为后续阶段预留的扩展点

- 工单成本与工时：work_orders 与 work_order_lines 均不含金额列，后续开通时按在线变更规则新增可空 numeric(18,2) 列并新增成本归集事件，不需要改既有列类型。
- 服务 SLA 引擎：时限提醒现由 flow 定时器承担，提醒策略表已按法人与工单类型分行，后续引入 SLA 引擎时该表可整体迁移为 SLA 定义的输入，业务表不动。
- 项目任务与合同交付节点的直接引用首版不包含（U-J-11 冻结值）：任务通过 source_contract_id、obligation_key 与派生计划追溯，不预建长期为空的节点 id 列；未来新增时走在线可空列与普通索引变更。
- 项目任务的单层父子分组（U-J-14 后半）：同样以在线新增可空 parent_task_id 列实现，本阶段不添加。
- 工单终态永不原地重开：返修始终经 follow_up_of_work_order_id 新建工单。未来流程扩展也只能围绕跟进工单增加提醒或审批，不得添加 COMPLETED/CANCELLED 回到非终态的迁移。
- EAM 其余部分：设备档案的 current_status_code 已字典化且带 is_terminal 语义，后续点检与维修工单可直接引用该字典与设备主键。
- 客户 360 的第六类及以后区块：新增区块只需新增一个 SectionKind 取值与一个提供者实现，聚合层不改。枚举扩展按基线第 5.6 节，客户端必须容忍未知取值并按未知降级展示。

---

### 12. 已关闭事项的首版冻结值

本阶段被 PRD 附录乙的 16 条 U-J 事项与 U-A、U-B、U-C 三组的 8 条触及。全部已关闭；表内值是唯一首版口径，“未来变更影响”只供以后版本评审，不授权实现方选择。

| 编号 | 首版状态 | 冻结取值 | 未来变更影响 |
|---|---|---|---|
| U-J-01 设备状态取值集合 | 已关闭 | 字典出厂五行：IN_STOCK 待交付、IN_SERVICE 使用中、UNDER_REPAIR 维修中（以上 is_terminal=false），SCRAPPED 已报废、RETURNED 已退回（is_terminal=true） | 新增/改显示/排序/停用走签名配置；已引用编码不可改删 |
| U-J-02 保修起始日期默认取值 | 已关闭 | 不设默认，为空即判为无保修信息 | 未来若默认取交付日期，改建档赋值与测试 |
| U-J-03 序列号唯一性范围 | 已关闭 | 法人内全局唯一，`inventory.serial_states` 是唯一权威；设备、工单、登记行只引用 inventory_serial_state_id，不保存 serial_no；手工设备无库存序列号时留空并用设备 code | 改变权威边界须数据迁移与跨模块架构决策，不能靠配置切换 |
| U-J-04 工单状态能否低代码扩展 | 已关闭 | 固定 DRAFT/PENDING_ACCEPTANCE/IN_PROGRESS/PENDING_CUSTOMER_CONFIRM/COMPLETED/CANCELLED；低代码只能在既有迁移上加审批、提醒和时限，不能新增状态或迁移 | 改状态集合需新版本 schema 与统计口径迁移 |
| U-J-05 工单优先级与类型、投诉渠道取值 | 已关闭 | priority_code 必填，优先级 LOW/NORMAL/HIGH/URGENT；工单类型 INSTALL/REPAIR/CONSULT/COMPLAINT_FOLLOWUP；渠道 PHONE/EMAIL/ONSITE/SALES_RELAY | 新增编码可走签名配置发布；已引用编码不可改删 |
| U-J-06 工单时限阈值 | 已关闭 | 出厂策略一行：待受理停留 480 分钟、期望完成提前 1440 分钟 | 改策略表行，经配置发布通道发布 |
| U-J-07 工单重开 | 已关闭 | 原地重开次数固定 0；COMPLETED 只可创建写 follow_up_of_work_order_id 的新返修跟进且次数无硬上限；CANCELLED 只能普通独立新建 | 终态与历史保留规则不可由低代码修改 |
| U-J-08 换货配对规则 | 已关闭 | EXCHANGE 必须同时关联销售退货行与 replacement delivery schedule，客户产品一致；两侧都终态才完成；只退用 RETURN，只补发走独立动作 | 放宽会破坏销售与售后追溯，不设首版开关 |
| U-J-09 维修完成确认方与附件 | 已关闭 | 由被指派的 TECHNICIAN 或 PROJECT_MANAGER 确认，不强制附件 | 未来若强制附件，加守卫与测试 |
| U-J-10 登记行与退货单行基数 | 已关闭 | 一对一 | 改为一对多需把引用迁到关联表，属结构变更 |
| U-J-11 任务与交付节点引用 | 已关闭 | 首版不建直接节点 id；以合同、obligation_key 和派生计划追溯 | 未来新增可空列与索引 |
| U-J-12 派生任务负责人默认值 | 已关闭 | 取派生项 owner_user_id，为空即留空 | 未来改变默认来源只改 4.7 的赋值规则 |
| U-J-13 变更导致任务不再需要的处置 | 已关闭 | 本值只适用于合同变更或续签：终态永久保留；删除/改变义务的旧非终态只置 derivation_stale 并建 PROJECT_MANAGER 处置事项，不自动删/取消；新增/改变义务用新版本 unique_key 建补充任务，旧任务由负责人显式完成或取消。合同终止场景不适用本值，唯一走第 4.8.1 节 `CLM_TERM_PROJECT_TASK`：NOT_STARTED 自动取消、IN_PROGRESS 人工决策 | 变更/续签自动处置或终止规则改变都会改变责任与审计语义，须分别版本化，不得混用 |
| U-J-14 项目存在未终态任务时能否完成或关闭 | 已关闭 | 阻断，守卫 P1 | 放宽需改变项目完整性规则与测试 |
| U-J-15 客户 360 区块条数与排序 | 已关闭 | 每区块 20 条，按业务日期降序、对象 ID 降序 | 改默认配置值，无代码改动 |
| U-J-16 设备是否纳入首批历史导入 | 已关闭 | 纳入，source=MIGRATION 与 migration_batch_no 已就位；迁移对账为条数与关系一致 | 未来范围变更只关闭迁移入口 |
| U-A-01 编号规则 | 已关闭 | 类型码 EQ、CPL、WO、PRJ、PT，登记在数据字典并与 sequence 常量一致 | 类型码变更需同步常量、字典与存量说明 |
| U-A-03 文本长度 | 已关闭 | 按基线第 11.2 节 | 放宽长度属在线 CHECK 变更 |
| U-A-05 列表默认值 | 已关闭 | 按基线第 11.5 节 | 无 |
| U-A-11 提醒提前量 | 已关闭 | 同 U-J-06 | 同上 |
| U-A-15 附件上限 | 已关闭 | 本阶段不设第二个业务侧条数上限，统一由平台附件能力判定；详情始终分页且不内联正文 | 平台上限变更不改 service schema |
| U-B-08 项目与客户维度授予粒度 | 已关闭 | 本阶段只供给 data_scope_tags（project:<项目编号>、customer:<客户编码>），判定与叠加归权限模块 | 标签形态变更只改生成函数 |
| U-C-04 客户 360 视图承载 | 已关闭 | 唯一端点 GET /api/v1/crm/customers/{id}/customer-360 与 Customer360SectionProvider 由阶段 5 建立，本阶段追加投诉、设备、工单区块；CustomerPanelProvider 作废，不新增路径 | 未来增减区块只扩 SectionKind/Provider，不改端点 |
| U-C-10 设备档案生成与粒度 | 已关闭 | 不自动生成；只由 create-from-delivery-batch 人工发起，逐台一行。服务端为每个交付行生成 unit_no=1..count，`(legal_entity_id, delivery_confirmation_line_id, source_delivery_unit_no)` 唯一；有库存序列号的台次引用对应 inventory_serial_state_id；单次上限 200 | 自动生成属于未来新消费者，不改变当前首版行为 |

---

### 13. 本阶段新增并已回写共享技术基线的规则

下列六条均为首版唯一实现口径，已回写共享技术基线对应章节，不再等待阶段结束后选择或替换。

1. 仅追加表清单扩充（回写基线第 4 节）：新增 `service.work_order_logs`、`project.project_migration_corrections`、`service.customer_complaint_migration_corrections`、`service.equipment_migration_corrections` 四张仅追加表，均不带 row_version、updated_at、updated_by。只有 work_order_logs 的 `entry_kind=CORRECTION` 行带非空 reverses_id，并以同法人同工单复合自外键及无环触发器指向真实父记录；三张 migration correction 不带 reverses_id，各以同法人 root FK、root 唯一键、形状 CHECK 与 DEFERRABLE 最终效果图证明 owner effect，并由 Stage 14 receipt/R0 绑定原 APPLY。各自创建迁移先登记 `APPEND_ONLY, mutable_columns={}` 再附着统一 guard，运行账号 UPDATE/DELETE 必须由数据库拒绝；CI SQL 静态检查与 `append_only_consistency.sql` 是第二道门禁。理由分别是 PRD 9.5.5 的处理记录不覆盖，以及迁移撤销不得覆盖/删除历史且不能依赖通用 reversal 表。
2. 敏感明文列的命名与类型（已回写基线第 4 节）：需要字段级信封加密的列一律命名为 `<语义>_enc`，类型 bytea，另配 `<语义>_key_ref text` 记录密钥标识与版本；该类列不得进入索引、唯一约束、过滤、排序、聚合与全文检索。全项目只采用这一格式，不接受阶段内替代命名或类型。
3. 索引名的 63 字节收缩规则（回写基线第 3.10 节）：索引名超过 PostgreSQL 的 63 字节标识符上限时，按 `ux_<table>_<缩写列名序列>` 收缩，缩写规则为 legal_entity_id 缩为 le、其余列去掉 _id 后缀，收缩后的全名与原列清单在数据字典中登记。
4. 模块局部受控取值字典（已回写基线第 3.2 节与第 7.1 节）：本阶段固定建立 `service.equipment_statuses`、`service.work_order_types`、`service.complaint_channels`、`service.work_order_priorities` 四张模块局部字典表（档案类，带 code、name、sort_no、is_active；设备状态另带 is_terminal），存事务数据库并经配置发布通道签名发布，不使用 CHECK 枚举，也不引用不存在的全局字典能力。字典表在 `(legal_entity_id, code)` 上建唯一键，引用列建复合外键指向该唯一键并 `ON DELETE RESTRICT`；字典行只允许停用不允许删除，已引用 code 不可改名。停用不执行任何 DDL 与 delete；应用层只判定启用状态并给出可读错误码，不设周期性孤儿取值核对。首版种子唯一取值为：设备状态 `IN_STOCK/IN_SERVICE/UNDER_REPAIR/SCRAPPED/RETURNED`（末两项终态）；工单类型 `INSTALL/REPAIR/CONSULT/COMPLAINT_FOLLOWUP`；投诉渠道 `PHONE/EMAIL/ONSITE/SALES_RELAY`；工单优先级 `LOW/NORMAL/HIGH/URGENT`。
5. 非过账事件的标注（回写基线第 6.1 节）：不承载会计语义的领域事件在事件目录中标注为非过账事件，其信封的 posting_date 与 accounting_period_id 置空，且不计入规格第 10.2 章关账受理前提中的待消费过账条目数。本阶段 25 个事件全部属于该类。
6. 新增五个指标（已回写基线第 9.2 节）：ep_service_work_orders_open（gauge，标签 legal_entity_id、status）、ep_service_work_order_open_lines（gauge，标签 legal_entity_id）、ep_crm_customer360_section_duration_seconds（histogram，标签 section）、ep_crm_customer360_section_degraded_total（counter，标签 section）、ep_project_contract_derivation_tasks_total（counter，标签 outcome 取 `new`、`changed`、`unchanged`、`stale`、`terminal_retained`）。标签基数纪律照旧，不使用 user_id、doc_no、trace_id 作标签。

本阶段不偏离基线的任何既有取值，因此不设偏离项一节。

---

### 14. 已冻结实现规则

下列七条均为首版唯一实现规则，开发、测试和验收直接据此执行，不存在可由实现方另选的分支。

- F-01 合同派生项的读取方式：按裁定 A-16，clm 提供 `ContractDerivationPlanQuery`，本阶段在消费合同生效事件后读取派生计划，不从事件载荷取任务清单。基线第 6.1 节要求 payload 只放最小必要数据与引用 ID；PRD 9.7.1 的“只接收派生结果”指本阶段不解释合同条款，与读取方式无关。
- F-02 项目与合同的对应关系：按裁定 A-16，`ContractDerivationPlan.project_group_contract_id` 是合同续签链的根合同标识；本阶段优先按该标识定位项目，该字段为空时取 `contract_id`，使续签派生任务与原任务落在同一项目。
- F-03 客户 360 的实现形态：固定采用区块提供者扇出的实时查询，不建物化投影或结果缓存。该实现遵守首版不使用物化视图的规则，并避免产生需要另行传播安全策略的数据副本。
- F-04 客户 360 的性能度量归属：固定新增一个常规交互度量项，通过线为 P95 不超过 2 秒；不单设较宽阈值。
- F-05 设备档案的停用路径：首版不包含设备档案停用入口，`is_active` 恒为 true，`deactivated_at` 恒为空；两列只为后续兼容保留。终止语义只由 `current_status_code` 的 `SCRAPPED/RETURNED` 表达。
- F-06 项目任务提交采购需求：固定采用两段式。交互事务把 task 原子置 PENDING 并发事件；job-worker 以可空 source_contract_id 与固定来源键 `PROJECT_TASK:{project_task_id}` 调用 `PurchaseRequisitionIntakePort::intake`，采购受理、双向 link、task LINKED、Inbox 同事务；最终死信与 task FAILED/last_error 同事务。一个任务只允许一个 link，不在交互事务内同步跨模块写。
- F-07 处理记录排序：不设独立行号列，固定按 `created_at ASC, id ASC` 给出稳定全序，其中 id 为 UUIDv7；游标由 `(created_at, id)` 组成。
