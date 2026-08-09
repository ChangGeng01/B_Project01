## 阶段 12 售后、项目与设备

本阶段实现 PRD 第 9 节的全部五类对象（设备档案与保修、客户投诉记录、售后工单、退换修登记、项目与项目任务），并实现规格第 8 章第 12 步要求的客户 360 聚合读取入口。本阶段的硬边界是不生成任何总账凭证、不写库存数量账与库存金额账、不产生成本归集，全部账务与库存后果由本阶段关联的销售退货单在其所属模块承接。

本计划严格照做共享技术基线。基线已给出取值的一律引用不重取；基线未覆盖而本阶段必须取值的，集中登记在第 13 节；规格与 PRD 未定义而必须假设的，集中登记在第 14 节；PRD 附录乙已登记的未决事项，集中登记在第 12 节并给出临时取值与切换代价。

### 0. 范围与不做的事

本阶段做的事：

1. 设备档案与保修信息的三条建档路径、在保状态判定、设备当前状态的字典化与终止状态确认。
2. 客户投诉记录的登记、受理、关闭、取消，以及升级为工单。
3. 售后工单的三个创建入口、六状态状态机、关联对象一致性校验、处理记录追加、时限提醒的触发登记。
4. 退换修登记行的三类处理方式、与销售退货单和发货侧单据的挂接、由对方终态事件回写、三条追溯链路双向可达。
5. 项目与项目任务的字段与状态机、合同生效派生项目任务的幂等消费、由项目任务提交采购需求的双向引用。
6. 客户 360 聚合端点与区块提供者契约的扩充，本阶段自实现投诉、设备、工单三个区块。按裁定 C-09，唯一端点 GET /api/v1/crm/customers/{id}/customer-360 与唯一契约 ep_contract_crm::Customer360SectionProvider 由阶段 5 建立，本阶段只追加区块取值与区块实现，不新增路径，不保留 /overview。

本阶段明确不做的事，取值按 PRD 9.1.2 与规格第 5.2 章售后与 FSM、项目与 PPM 两个条目：现场派工与调度、服务权益与服务合同计费、售后知识库、独立服务 SLA 引擎、点检计划、维修工单、备件、设备成本、可靠性分析、预测维护、保修索赔与费用结算与延展销售、WBS 多层分解、资源与产能、工时填报、项目预算、项目风险、变更管理、挣值、工单成本归集。本阶段也不定义销售退货单、交付确认单、合同、采购需求、客户档案本身。

---

### 1. 交付物清单

本阶段结束时下列东西可运行、可验证。

| 序号 | 交付物 | 形态 | 判定方式 |
|---|---|---|---|
| D-01 | ep-contract-service、ep-domain-service、ep-app-service 三个 crate | 编译通过并被 core-server 与 job-worker 装配 | cargo build 与依赖自检脚本通过 |
| D-02 | ep-contract-project、ep-domain-project、ep-app-project 三个 crate | 同上 | 同上 |
| D-03 | 对阶段 5 已建立的 ep-contract-crm 客户 360 区块契约的扩充与 ep-app-crm 聚合用例的三个新区块 | 编译通过并被 core-server 装配 | 同上 |
| D-04 | service schema 的 13 张表与 project schema 的 5 张表 | db/migrations/service/ 与 db/migrations/project/ 下的迁移可离线执行并可回退 | refinery 迁移在空库上执行成功，且 --check 模式下启动自检项 rls-enabled-and-forced 通过 |
| D-05 | 售后侧 36 个 HTTP 端点、项目侧 16 个 HTTP 端点、客户 360 的 1 个端点 | core-server 暴露于 /api/v1/service、/api/v1/project、/api/v1/crm | 端点级集成测试全绿 |
| D-06 | 25 个领域事件的发布与 3 个消费者 | Outbox 写入与 job-worker 消费 | 重复投递不少于 3 次的幂等测试通过 |
| D-07 | 三张受控取值字典的出厂数据与配置发布通道接入 | 迁移回填 + 配置发布包 | 字典改动经签名发布后生效，且不触发 DDL |
| D-08 | 工单时限提醒的定时器登记与站内通知送达 | 经 ep-platform-flow 定时器与 ep-platform-notify | 两类提醒的端到端测试通过 |
| D-09 | tests/rls_matrix 中本阶段 18 张带法人表的越权矩阵用例 | 独立测试目标 | 八类越权面全部返回 404 或 403，无内容回显 |
| D-10 | 闭环第 12 步的用例片段与三条追溯链路双向可达用例 | 前者为 testkit/scenarios/stage12_service_step12.rs 中的步骤函数与断言，供阶段 9b 的 testkit/scenarios/golden_loop_14_steps.rs 在第 12 步引用；后者为 apps/core-server/tests/ 下的 E2E | 两者在本阶段各自跑通全绿，整条链路的串接通过由阶段 9b 的该用例判定 |
| D-11 | 边界不变量用例 | 执行本阶段全部用例前后规格第 17.3 章三项取值不变 | 由 ep-platform-recon 语句集比对，差额为零 |
| D-12 | docs/event-catalog.md、docs/error-codes.md、docs/data-dictionary.md 三处登记，其中数据字典含本阶段五个单据类型码 EQ、CPL、WO、PRJ、PT | 文档 | CI 一致性校验通过，且 xtask configdoc --check-doc-type-codes 通过 |
| D-13 | project.v_projects_dataset 受治理数据集视图，dataset code 为 project_projects，grain 为 DOCUMENT | db/migrations/project/ 下的视图迁移 | 视图已发布并授予 ep_analyst_ro，列签名与 reporting.dataset_fields 的登记一致 |
| D-14 | service 与 project 两个模块的四端界面 | clients/desktop/src/modules/service/、clients/desktop/src/modules/project/ 与 clients/mobile/src/modules/ 下的同名目录 | 四端 UI 用例全绿 |
| D-15 | 本阶段全部路由的能力域码与动作类别常量 | crates/contract/service/src/capability.rs 与 crates/contract/project/src/capability.rs | xtask configdoc 通过 |
| D-16 | ServiceReferenceCounter | crates/application/service/src/probe/master_reference.rs 与两个 wiring.rs 的注册行 | 阶段 5 的档案停用引用计数在 service 模块上有计数 |

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
| ep-app-project | crates/application/project | 应用 | ep-foundation、ep-platform-*、ep-domain-project、ep-contract-（project、clm、procure、mdm） | core-server、job-worker |

ep-contract-service 对外只暴露 ReturnRepairTraceQuery 一个 trait。按裁定 B-06 撤销 EquipmentQuery，不建 crates/contract/service/src/port/equipment.rs，设备的跨模块可见性只保留三条路径，见 5.1 节末段。ep-contract-project 对外不暴露任何命令 trait，按裁定 C-19 撤销 ProjectTaskDerivationPort，合同派生一律由本阶段消费事件后自行派生。本阶段依赖的三个跨模块 trait 及其提供方固定为：ep_contract_clm::ContractDerivationPlanQuery（裁定 A-16，阶段 6 提供）、ep_contract_sales::SalesReturnCommandPort（裁定 A-17，阶段 6 提供）、ep_contract_procure::PurchaseRequisitionIntakePort（裁定 C-17，阶段 7 提供）。三者的名字与签名以裁定为准，本阶段不另立第二套命名，也不在本计划中复述其字段之外的推测。

#### 2.2 改动的既有 crate

| crate | 改动 |
|---|---|
| ep-contract-crm | 扩充阶段 5 已建立的 crm 契约：在既有 Customer360SectionProvider 上追加 Customer360SectionKind 的 Complaints、Equipments、WorkOrders 三个取值与配套的 Customer360Item 字段，不新增 trait，不新增端点 |
| ep-app-crm | 新增 usecase/query_customer_360.rs，做区块扇出、超时降级与合并 |
| ep-adapter-db-pg | 新增 repository/service/ 与 repository/project/ 两个目录，各仓储只访问自己模块的 schema |
| ep-adapter-search | 五类对象的检索文档投影一律产出 foundation::port::search::SearchDocument，object_type 取表全名如 service.equipment_records，写入方仍为 job-worker 的索引消费者，该消费者与 ep-adapter-search 本体按裁定 A-07 由阶段 3b 交付 |
| apps/core-server/src/wiring.rs | 注册两个模块的仓储与用例，注册三个客户 360 区块提供者，并把 ServiceReferenceCounter 注册进阶段 5 提供的 MasterReferenceCounterRegistry |
| apps/job-worker/src/wiring.rs | 注册三个 Outbox 消费者 project.contract_derivation、project.requisition_intake 与 service.return_repair_writeback、一个定时器回调，并把 ServiceReferenceCounter 注册进 MasterReferenceCounterRegistry |
| ep-testkit | 新增 EquipmentRecordBuilder、WorkOrderBuilder、ComplaintBuilder、ProjectBuilder、ProjectTaskBuilder、ContractDerivationPlanFake、SalesReturnPortFake，后两者分别按裁定 A-16 与 A-17 冻结的签名实现；另新增 testkit/scenarios/stage12_service_step12.rs，内含闭环第 12 步的步骤函数与断言，由阶段 9b 的 testkit/scenarios/golden_loop_14_steps.rs 引用 |
| ep-datagen | 基准数据集追加设备 5000 台、工单 20000 张、投诉 5000 条、项目 200 个、项目任务 4000 条 |

#### 2.3 进程归属

- core-server：全部交互式命令与查询、客户 360 聚合、退换修登记行提交时对 ep-contract-sales 已交付数量查询的同步调用；销售退货单的创建命令不在 core-server 发起，见 4.6。
- job-worker：三个 Outbox 消费者，即合同生效派生项目任务的 project.contract_derivation、项目任务提交采购需求的 project.requisition_intake、退换修登记行挂接与回写的 service.return_repair_writeback；工单时限提醒定时器的回调执行；检索索引传播事件的发布方。
- 本阶段不新增进程，不使用 integration-gateway、plugin-host、portal-gateway。本阶段对象不进入供应商门户的受控能力 API。

#### 2.4 依赖方向自检

本阶段新增的依赖全部落在基线第 1.3 节允许的方向内，评审时逐条核对下列四项：ep-domain-service 与 ep-domain-project 不出现 sqlx、reqwest、tokio 的 IO 模块、std::fs、std::net、SystemTime::now、rand；ep-app-service 不依赖 ep-app-sales 与 ep-app-clm，跨模块一律经对方 contract 的 trait；ep-app-clm 与 ep-app-finance 为实现客户 360 区块而依赖 ep-contract-crm，方向为 application 依赖 contract，合规；ep-adapter-db-pg 中 service 仓储只出现 service.* 表名，project 仓储只出现 project.* 表名，由 CI 的 SQL 静态检查断言。

---

### 3. 数据库变更

#### 3.1 迁移文件与执行顺序

order.toml 中业务 schema 顺序已固定为 mdm、cpq、clm、sales、procure、inventory、costing、project、service、invoice、finance、ledger、crm、portal、reporting，因此 project 先于 service 执行。本阶段不新增 schema，不改动 order.toml。本阶段不建任何跨 schema 外键，跨模块引用只留逻辑引用列，其存在性由 application 层在写入时经对方模块契约校验；按裁定 A-06 本阶段不实现也不注册任何 ReconCheck，跨模块逻辑引用不建周期性对账校验项，属首版已知边界，第 8.5 节的不变量核对只运行其他阶段已注册的校验项。

| 顺序 | 文件 | 内容 |
|---|---|---|
| 1 | db/migrations/project/V202611020900__project_create_projects.sql | 建 project.projects，含索引与 RLS |
| 2 | db/migrations/project/V202611020905__project_create_project_tasks.sql | 建 project.project_tasks |
| 3 | db/migrations/project/V202611020910__project_create_task_requisition_links.sql | 建 project.project_task_purchase_requisition_links |
| 4 | db/migrations/project/V202611020915__project_create_attachment_links.sql | 建 project.project_attachments、project.project_task_attachments |
| 5 | db/migrations/project/V202611020920__project_create_dataset_views.sql | 建 project.v_projects_dataset 并授予 ep_analyst_ro，按裁定 A-18 |
| 6 | db/migrations/service/V202611021000__service_create_dictionaries.sql | 建 service.equipment_statuses、service.work_order_types、service.complaint_channels |
| 7 | db/migrations/service/V202611021005__service_create_equipment_records.sql | 建 service.equipment_records |
| 8 | db/migrations/service/V202611021010__service_create_customer_complaints.sql | 建 service.customer_complaints |
| 9 | db/migrations/service/V202611021015__service_create_work_orders.sql | 建 service.work_orders |
| 10 | db/migrations/service/V202611021020__service_create_work_order_lines.sql | 建 service.work_order_lines |
| 11 | db/migrations/service/V202611021025__service_create_work_order_logs.sql | 建 service.work_order_logs |
| 12 | db/migrations/service/V202611021030__service_create_reminder_policies.sql | 建 service.work_order_reminder_policies |
| 13 | db/migrations/service/V202611021035__service_create_attachment_links.sql | 建四张附件关联表 |
| 14 | db/migrations/service/V202611021040__service_backfill_seed_dictionaries.sql | 回填三张字典的出厂取值，按法人逐个写入，created_by 取 foundation::SYSTEM_PRINCIPAL_ID |

每个文件头部带 `-- rollback:` 段。前十三个文件的回退为对应的 drop table、drop view 与 drop policy，属可安全逆向；第十四个回填文件的回退为按 code 删除出厂行，若该行已被业务数据引用则拒绝回退并注明只能用升级前备份回退。全部文件按基线第 3.9 节固定 `SET lock_timeout = '5s'` 与 `SET statement_timeout = '30min'`，全部索引用 CREATE INDEX CONCURRENTLY，因此建表与建索引拆在同一文件的两个语句块而不放在同一事务内，文件内显式声明不使用隐式事务包裹。

#### 3.2 公共列与统一约束

18 张表全部带基线第 4 节的九个公共列，顺序按基线排列：id、legal_entity_id、security_level、data_scope_tags、row_version、created_at、created_by、updated_at、updated_by。其中 service.work_order_logs 为仅追加表，不带 row_version、updated_at、updated_by，改带 reverses_id uuid null。

18 张表全部带 legal_entity_id，因此全部按基线第 3.8 节模板生成 RLS：`enable row level security`、`force row level security`、一条 `rls_<table>_le` 策略，using 与 with check 均为 `legal_entity_id = nullif(current_setting('app.legal_entity_id', true), '')::uuid`。策略由迁移生成器产出，本阶段不写变体。

全部 text 列带 CHECK 长度约束，取值按基线第 11.2 节：编码 64、名称 200、简述 500、备注与原因与说明 2000、保修条款文本 1 MB。全部枚举列为 text 加 CHECK，取值为大写 snake_case。全部时间列为 timestamptz，日期列为 date。数量列为 numeric(18,6)。本阶段没有金额列。

#### 3.3 project schema 逐表定义

project.projects（单据类）

| 列 | 类型 | 可空 | 默认 | 说明 |
|---|---|---|---|---|
| 公共列九列 | — | — | — | 按基线第 4 节 |
| doc_no | text | 否 | 无 | 项目编号，格式 PRJ-<法人码>-<YYYYMM>-<6 位流水> |
| status | text | 否 | 'IN_PROGRESS' | CHECK 取值 IN_PROGRESS、COMPLETED、CLOSED |
| name | text | 否 | 无 | 项目名称，长度 ≤ 200 |
| customer_id | uuid | 是 | 无 | 逻辑引用 mdm 客户，无外键 |
| source_contract_id | uuid | 是 | 无 | 来源合同，逻辑引用 clm |
| project_group_contract_id | uuid | 是 | 无 | 合同续签链的根合同，用于定位项目，取值取自 ContractDerivationPlan 的 project_group_contract_id，见 4.7 |
| owner_user_id | uuid | 否 | 无 | 项目负责人 |
| planned_start_on | date | 是 | 无 | 计划开始日期 |
| planned_finish_on | date | 是 | 无 | 计划完成日期 |
| description | text | 是 | 无 | 说明，长度 ≤ 2000 |
| completed_at | timestamptz | 是 | 无 | 流转到 COMPLETED 的时点 |
| closed_at | timestamptz | 是 | 无 | 流转到 CLOSED 的时点 |

约束：ck_projects_status；ck_projects_name_len；ck_projects_description_len；ck_projects_plan_range 为 `planned_finish_on is null or planned_start_on is null or planned_finish_on >= planned_start_on`。
索引：pk_projects；ux_projects_legal_entity_id_doc_no；ix_projects_legal_entity_id_created_at；ux_projects_le_group_contract 建于 (legal_entity_id, project_group_contract_id)，用于保证一条合同续签链只有一个项目，手工项目该列为 NULL 因而互不冲突；ix_projects_legal_entity_id_customer_id；ix_projects_legal_entity_id_owner_user_id。

project.project_tasks（单据类）

| 列 | 类型 | 可空 | 默认 | 说明 |
|---|---|---|---|---|
| 公共列九列 | — | — | — | — |
| doc_no | text | 否 | 无 | 任务编号，格式 PT-<法人码>-<YYYYMM>-<6 位流水> |
| status | text | 否 | 'NOT_STARTED' | CHECK 取值 NOT_STARTED、IN_PROGRESS、COMPLETED、CANCELLED |
| project_id | uuid | 否 | 无 | 同 schema 真实外键，fk_project_tasks_projects，ON DELETE RESTRICT |
| name | text | 否 | 无 | 任务名称，≤ 200 |
| source | text | 否 | 无 | CHECK 取值 CONTRACT_DERIVED、MANUAL |
| source_contract_id | uuid | 是 | 无 | 来源合同 |
| source_contract_version_no | integer | 是 | 无 | 来源合同版本号 |
| derivation_unique_key | text | 是 | 无 | 取值为 ep_contract_clm::ContractDerivationItem 的 unique_key，格式按裁定 A-16 为 <contract_id>:<contract_version_no>:<item_kind>:<source_contract_line_id 或 milestone_no>，≤ 200 |
| derivation_batch_no | integer | 是 | 无 | 取值为 ContractDerivationPlan 的 derivation_batch_no，只用于追溯 |
| assignee_user_id | uuid | 是 | 无 | 任务负责人 |
| planned_start_on | date | 是 | 无 | — |
| planned_finish_on | date | 是 | 无 | — |
| actual_finish_on | date | 是 | 无 | 流转到 COMPLETED 时按中国标准时间自然日写入 |
| description | text | 是 | 无 | ≤ 2000 |
| cancel_reason | text | 是 | 无 | ≤ 2000 |

约束：ck_project_tasks_status；ck_project_tasks_source；ck_project_tasks_derived_fields 为 `source <> 'CONTRACT_DERIVED' or (source_contract_id is not null and derivation_unique_key is not null)`；ck_project_tasks_derivation_unique_key_len 为 `derivation_unique_key is null or char_length(derivation_unique_key) between 1 and 200`；ck_project_tasks_plan_range；ck_project_tasks_finish_when_completed 为 `status <> 'COMPLETED' or actual_finish_on is not null`；ck_project_tasks_cancel_reason 为 `status <> 'CANCELLED' or cancel_reason is not null`。
索引：pk_project_tasks；ux_project_tasks_legal_entity_id_doc_no；ix_project_tasks_legal_entity_id_created_at；ux_project_tasks_le_derivation_unique_key 建于 (legal_entity_id, derivation_unique_key)，是派生幂等的数据库侧兜底，手工任务该列为 NULL 因而互不冲突；ix_project_tasks_legal_entity_id_source_contract_id；ix_project_tasks_project_id_status；ix_project_tasks_legal_entity_id_assignee_user_id_planned_finish_on。索引名长度均在 63 字节内，超长时按第 13 节登记的收缩规则处理。

project.project_task_purchase_requisition_links（关联表）

| 列 | 类型 | 可空 | 默认 | 说明 |
|---|---|---|---|---|
| 公共列九列 | — | — | — | — |
| project_task_id | uuid | 否 | 无 | 同 schema 外键，ON DELETE RESTRICT |
| purchase_requisition_id | uuid | 否 | 无 | 逻辑引用 procure，无外键 |
| purchase_requisition_doc_no | text | 是 | 无 | 冗余展示用，≤ 64 |
| requested_at | timestamptz | 否 | now() | 提交时点 |

索引：pk；ux_task_requisition_links_le_requisition 建于 (legal_entity_id, purchase_requisition_id)，保证一条采购需求最多来自一个项目任务；ix_task_requisition_links_project_task_id；ix_task_requisition_links_legal_entity_id_created_at。

project.project_attachments、project.project_task_attachments：按基线第 4 节附件关联表定义，列为公共列加 owner_id、attachment_object_id、purpose text、sort_no integer。owner_id 建同 schema 外键。ux 建于 (owner_id, attachment_object_id)。

#### 3.4 service schema 逐表定义

service.equipment_statuses、service.work_order_types、service.complaint_channels（档案类字典，三张表结构相同）

| 列 | 类型 | 可空 | 默认 | 说明 |
|---|---|---|---|---|
| 公共列九列 | — | — | — | — |
| code | text | 否 | 无 | ≤ 64 |
| name | text | 否 | 无 | ≤ 200 |
| sort_no | integer | 否 | 0 | 列表排序 |
| is_active | boolean | 否 | true | 停用不影响历史引用 |
| deactivated_at | timestamptz | 是 | 无 | — |
| is_terminal | boolean | 否 | false | 只在 equipment_statuses 上存在，true 表示终止状态 |

索引：pk；ux_<table>_legal_entity_id_code；ix_<table>_legal_entity_id_created_at。这三张表是运行期可变的枚举字典，按基线第 7.1 节存事务数据库并经配置发布通道签名发布，改动取值不触发 DDL。

service.equipment_records（档案类）

| 列 | 类型 | 可空 | 默认 | 说明 |
|---|---|---|---|---|
| 公共列九列 | — | — | — | data_scope_tags 写入 customer:<客户编码> |
| code | text | 否 | 无 | 设备编号，格式 EQ-<法人码>-<YYYYMM>-<6 位流水>，生成后不可修改 |
| is_active | boolean | 否 | true | 见第 14 节假设 A-05 |
| deactivated_at | timestamptz | 是 | 无 | — |
| serial_no | text | 是 | 无 | ≤ 64，与库存序列号同一取值口径 |
| model | text | 否 | 无 | 型号，≤ 200 |
| customer_id | uuid | 否 | 无 | 逻辑引用 mdm 客户 |
| product_id | uuid | 是 | 无 | 逻辑引用 mdm 产品 |
| batch_no | text | 否 | '-' | 未启用批次时取 '-'，按基线第 11.4 节 |
| sales_order_line_id | uuid | 是 | 无 | 逻辑引用 sales |
| delivery_confirmation_id | uuid | 是 | 无 | 路径一写入，只读，逻辑引用 sales.delivery_confirmations，该表按裁定 A-09 由阶段 6 建立 |
| delivery_confirmation_line_id | uuid | 是 | 无 | 路径一写入，建档去重用，逻辑引用 sales.delivery_confirmation_lines |
| delivered_on | date | 是 | 无 | 交付日期 |
| installed_on | date | 是 | 无 | 安装日期 |
| current_status_code | text | 否 | 无 | 引用本 schema 字典的 code，同 schema 但不建外键，理由见下 |
| warranty_start_on | date | 是 | 无 | — |
| warranty_end_on | date | 是 | 无 | — |
| warranty_scope | text | 是 | 无 | ≤ 500 |
| warranty_terms | text | 是 | 无 | ≤ 1 MB |
| remark | text | 是 | 无 | ≤ 2000 |
| source | text | 否 | 无 | CHECK 取值 DELIVERY_CONFIRMATION、MANUAL、MIGRATION |
| migration_batch_no | text | 是 | 无 | ≤ 64，规格第 7.10 章迁移批次标识 |

约束：ck_equipment_records_source；ck_equipment_records_install_after_delivery 为 `installed_on is null or delivered_on is null or installed_on >= delivered_on`；ck_equipment_records_warranty_range 为 `warranty_end_on is null or warranty_start_on is null or warranty_end_on >= warranty_start_on`；ck_equipment_records_batch_no_len 为 `char_length(batch_no) between 1 and 64`；ck_equipment_records_migration_source 为 `source <> 'MIGRATION' or migration_batch_no is not null`。交付日期不得晚于登记时点自然日不落在 CHECK 上，理由是该判据依赖当前时间，不是不可变表达式，改由应用层按 Clock 端口判定。
current_status_code 不建外键，理由是字典行由配置发布通道写入并可停用，外键会把配置停用与业务表更新绑死；取值合法性由应用层在写入前对字典做存在性与启用状态校验，字典行只允许停用不允许删除，孤儿取值因此无从产生，本阶段不设周期性孤儿取值核对。
索引：pk_equipment_records；ux_equipment_records_legal_entity_id_code；ix_equipment_records_legal_entity_id_created_at；ix_equipment_records_legal_entity_id_customer_id；ix_equipment_records_legal_entity_id_serial_no；ix_equipment_records_le_delivery_conf_line 建于 (legal_entity_id, delivery_confirmation_line_id)，用于路径一建档的重复判定；ix_equipment_records_legal_entity_id_current_status_code。基线三条之外的四条索引理由是设备列表、按客户聚合的客户 360 区块、按交付确认行去重与工单创建时的设备检索四类查询进入附录 A.1 的度量范围，需给出 EXPLAIN 无顺序扫描的证据。
序列号唯一性：本阶段不建唯一索引，见第 12 节 U-J-03。

service.customer_complaints（单据类）

| 列 | 类型 | 可空 | 默认 | 说明 |
|---|---|---|---|---|
| 公共列九列 | — | — | — | — |
| doc_no | text | 否 | 无 | 投诉编号，格式 CPL-<法人码>-<YYYYMM>-<6 位流水> |
| status | text | 否 | 'REGISTERED' | CHECK 取值 REGISTERED、PROCESSING、CLOSED、CANCELLED |
| customer_id | uuid | 否 | 无 | 逻辑引用 mdm 客户 |
| contact_name | text | 是 | 无 | ≤ 200 |
| contact_info_enc | bytea | 是 | 无 | 联系方式密文，字段级信封加密，见 3.6 |
| contact_info_key_ref | text | 是 | 无 | 密钥引用与版本，≤ 200 |
| complaint_on | date | 否 | 无 | 投诉日期，不得晚于登记时点自然日，应用层判定 |
| channel_code | text | 是 | 无 | 引用 complaint_channels |
| content | text | 否 | 无 | 投诉内容，≤ 2000 |
| contract_id | uuid | 是 | 无 | 逻辑引用 clm |
| sales_order_line_id | uuid | 是 | 无 | 逻辑引用 sales |
| product_id | uuid | 是 | 无 | 逻辑引用 mdm |
| equipment_record_id | uuid | 是 | 无 | 同 schema 外键 fk_customer_complaints_equipment_records |
| accepted_by | uuid | 是 | 无 | 受理人 |
| accepted_at | timestamptz | 是 | 无 | — |
| handling_note | text | 是 | 无 | 处理说明，≤ 2000 |
| closed_at | timestamptz | 是 | 无 | — |
| cancel_reason | text | 是 | 无 | ≤ 2000 |

关联工单不在本表存列，理由见 4.3。
约束：ck_customer_complaints_status；ck_customer_complaints_accept 为 `status <> 'PROCESSING' or accepted_by is not null`；ck_customer_complaints_close 为 `status <> 'CLOSED' or handling_note is not null`；ck_customer_complaints_cancel 为 `status <> 'CANCELLED' or cancel_reason is not null`；各 text 列长度约束。
索引：pk；ux_customer_complaints_legal_entity_id_doc_no；ix_customer_complaints_legal_entity_id_created_at；ix_customer_complaints_legal_entity_id_customer_id_complaint_on；ix_customer_complaints_legal_entity_id_status。

service.work_orders（单据类）

| 列 | 类型 | 可空 | 默认 | 说明 |
|---|---|---|---|---|
| 公共列九列 | — | — | — | — |
| doc_no | text | 否 | 无 | 工单编号，格式 WO-<法人码>-<YYYYMM>-<6 位流水> |
| status | text | 否 | 'DRAFT' | CHECK 取值 DRAFT、PENDING_ACCEPTANCE、IN_PROGRESS、PENDING_CUSTOMER_CONFIRM、COMPLETED、CANCELLED |
| work_order_type_code | text | 否 | 无 | 引用 work_order_types |
| customer_id | uuid | 否 | 无 | 逻辑引用 mdm 客户 |
| contact_name | text | 是 | 无 | ≤ 200 |
| contact_info_enc | bytea | 是 | 无 | 字段级信封加密 |
| contact_info_key_ref | text | 是 | 无 | — |
| source_complaint_id | uuid | 是 | 无 | 同 schema 外键 fk_work_orders_customer_complaints |
| sales_order_id | uuid | 是 | 无 | 逻辑引用 sales |
| sales_order_line_id | uuid | 是 | 无 | 逻辑引用 sales |
| contract_id | uuid | 是 | 无 | 逻辑引用 clm |
| product_id | uuid | 是 | 无 | 逻辑引用 mdm |
| batch_no | text | 否 | '-' | — |
| serial_no | text | 是 | 无 | ≤ 64 |
| equipment_record_id | uuid | 是 | 无 | 同 schema 外键 fk_work_orders_equipment_records |
| warranty_status | text | 否 | 'NO_WARRANTY_INFO' | CHECK 取值 IN_WARRANTY、WARRANTY_NOT_STARTED、WARRANTY_EXPIRED、NO_WARRANTY_INFO，创建时快照写入，只读 |
| warranty_judged_on | date | 否 | 无 | 在保判定日期快照，只读 |
| problem_description | text | 否 | 无 | ≤ 2000 |
| expected_finish_on | date | 是 | 无 | 不得早于创建时点自然日，应用层判定 |
| assignee_user_id | uuid | 是 | 无 | 处理人 |
| terminal_equipment_confirmed_by | uuid | 是 | 无 | 选用终止状态设备的确认人 |
| terminal_equipment_confirmed_at | timestamptz | 是 | 无 | — |
| submitted_at | timestamptz | 是 | 无 | 进入 PENDING_ACCEPTANCE 的时点，时限提醒的计时起点 |
| accepted_at | timestamptz | 是 | 无 | 进入 IN_PROGRESS 的时点 |
| conclusion_note | text | 是 | 无 | 处理结论说明，≤ 2000 |
| completed_at | timestamptz | 是 | 无 | — |
| cancel_reason | text | 是 | 无 | ≤ 2000 |

约束：ck_work_orders_status；ck_work_orders_warranty_status；ck_work_orders_assignee 为 `status not in ('IN_PROGRESS','PENDING_CUSTOMER_CONFIRM','COMPLETED') or assignee_user_id is not null`；ck_work_orders_conclusion 为 `status <> 'COMPLETED' or conclusion_note is not null`；ck_work_orders_cancel 为 `status <> 'CANCELLED' or cancel_reason is not null`。
索引：pk；ux_work_orders_legal_entity_id_doc_no；ix_work_orders_legal_entity_id_created_at；ux_work_orders_le_source_complaint 建于 (legal_entity_id, source_complaint_id)，由数据库保证一条投诉最多升级一次；ix_work_orders_legal_entity_id_customer_id_created_at；ix_work_orders_legal_entity_id_equipment_record_id；ix_work_orders_legal_entity_id_sales_order_line_id；ix_work_orders_legal_entity_id_assignee_user_id_status；ix_work_orders_legal_entity_id_status_submitted_at。最后一条支撑时限提醒的扫描与工单列表的默认筛选。

service.work_order_lines（明细行表，承载 PRD 9.6 的退换修登记行）

| 列 | 类型 | 可空 | 默认 | 说明 |
|---|---|---|---|---|
| 公共列九列 | — | — | — | — |
| work_order_id | uuid | 否 | 无 | 同 schema 外键，ON DELETE RESTRICT |
| line_no | integer | 否 | 无 | 登记行号，工单内连续 |
| handling_method | text | 否 | 无 | CHECK 取值 RETURN、EXCHANGE、REPAIR |
| status | text | 否 | 'PENDING' | CHECK 取值 PENDING、LINKED、COMPLETED、VOIDED |
| product_id | uuid | 是 | 无 | — |
| batch_no | text | 否 | '-' | — |
| serial_no | text | 是 | 无 | — |
| equipment_record_id | uuid | 是 | 无 | 同 schema 外键 |
| quantity | numeric(18,6) | 否 | 无 | CHECK > 0 |
| sales_order_line_id | uuid | 是 | 无 | 退货与换货必填，逻辑引用 sales |
| reason_note | text | 是 | 无 | 登记原因说明，≤ 2000 |
| sales_return_id | uuid | 是 | 无 | 退货侧单据，逻辑引用 sales |
| sales_return_line_id | uuid | 是 | 无 | — |
| outbound_document_id | uuid | 是 | 无 | 发货侧单据，逻辑引用 sales |
| outbound_document_line_id | uuid | 是 | 无 | — |
| repair_result_note | text | 是 | 无 | ≤ 2000 |
| repair_finished_on | date | 是 | 无 | — |
| void_reason | text | 是 | 无 | ≤ 2000 |

约束：ck_work_order_lines_handling_method；ck_work_order_lines_status；ck_work_order_lines_quantity_positive 为 `quantity > 0`；ck_work_order_lines_order_line_required 为 `handling_method = 'REPAIR' or sales_order_line_id is not null`；ck_work_order_lines_repair_no_doc 为 `handling_method <> 'REPAIR' or (sales_return_id is null and outbound_document_id is null)`；ck_work_order_lines_complete_needs_doc 为 `status <> 'COMPLETED' or handling_method = 'REPAIR' or sales_return_id is not null`；ck_work_order_lines_void_reason 为 `status <> 'VOIDED' or void_reason is not null`。换货是否强制两侧配对不落 CHECK，见第 12 节 U-J-08。
索引：pk；ux_work_order_lines_work_order_id_line_no；ix_work_order_lines_legal_entity_id_created_at；ix_work_order_lines_legal_entity_id_sales_return_id；ix_work_order_lines_legal_entity_id_sales_order_line_id；ix_work_order_lines_legal_entity_id_equipment_record_id；ix_work_order_lines_legal_entity_id_status_handling_method。

service.work_order_logs（仅追加表）

| 列 | 类型 | 可空 | 默认 | 说明 |
|---|---|---|---|---|
| id、legal_entity_id、security_level、data_scope_tags、created_at、created_by | — | — | — | 仅追加表不带 row_version、updated_at、updated_by |
| reverses_id | uuid | 是 | 无 | 本条是对哪条记录的更正说明 |
| work_order_id | uuid | 否 | 无 | 同 schema 外键 |
| action_note | text | 否 | 无 | 处理动作说明，≤ 2000 |

索引：pk；ix_work_order_logs_work_order_id_created_at；ix_work_order_logs_legal_entity_id_created_at。展示顺序按 created_at、id，不设行号，理由是行号需要额外的串行化点而 created_at 与 UUIDv7 的 id 已可给出稳定全序。本表在业务 schema 上禁止 DELETE 与 UPDATE，由 CI 的 SQL 静态检查断言。

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

service.equipment_record_attachments、service.customer_complaint_attachments、service.work_order_attachments、service.work_order_log_attachments：结构同 project 侧附件关联表。

#### 3.5 设备状态变更历史不建表

PRD 9.3.4 要求状态变更记录变更前后取值、操作者、时间与原因并写入审计。本阶段不建业务侧历史表，改为在同一事务写 platform_audit.audit_events，object_type 取 service.equipment_records，before 与 after 携带 current_status_code，reason 携带原因说明；设备详情页的变更历史经 ep-platform-audit 的查询能力读取。理由是审计事件已是该事实的权威且不可覆盖的落点，另建业务表会形成同一事实的第二处记录，与基线第 9.4 节的硬边界冲突。

#### 3.6 联系方式的字段级加密

规格第 12.3 章把联系方式列为行内敏感字段。投诉与工单上的联系方式按字段级信封加密存储于 `contact_info_enc bytea`，密钥经 ep-adapter-kms 在该法人密钥域下取用，`contact_info_key_ref` 记录密钥标识与版本。该列不参与过滤、排序、聚合、唯一约束与全文检索，检索文档投影中该字段以掩码写入。日志、错误消息与指标标签中一律不出现该字段，Rust 侧用 foundation::Redacted 包装。含该字段的列表导出按规格第 12.1 章敏感数据导出执行重新认证与审批，由平台导出能力承担，本阶段只声明字段敏感标记与密级。

#### 3.7 受治理数据集视图

按裁定 A-18，本阶段发布一个受治理数据集视图 project.v_projects_dataset，dataset code 为 project_projects，grain 为 DOCUMENT，由 db/migrations/project/V202611020920__project_create_dataset_views.sql 建立。视图取数为 project.projects，必须包含 legal_entity_id、security_level、data_scope_tags 三列，另含 id、doc_no、status、name、customer_id、source_contract_id、project_group_contract_id、owner_user_id、planned_start_on、planned_finish_on、completed_at、closed_at、created_at。同一迁移内执行 GRANT SELECT ON project.v_projects_dataset TO ep_analyst_ro，不授予 ep_app_rw 之外的任何写权限。视图的列名与类型签名必须与 reporting.dataset_fields 的登记一致，由阶段 11 的启动自检项 reporting-dataset-signature-matched 校验；该目录行由阶段 11 先播种，在本视图发布之前该自检项按已登记但未发布降级放行，本阶段结束后转为强制。本阶段不为 service schema 发布任何数据集视图，售后侧对外取数仍走 5.1 至 5.3 的端点与全文检索文档。

---

### 4. 领域模型与关键算法

#### 4.1 核心类型

ep-domain-service 的 model 目录一个聚合一个文件：EquipmentRecord、CustomerComplaint、WorkOrder（含 WorkOrderLine 与 WorkOrderLog 两个内部实体）。value 目录：WarrantyWindow、WarrantyStatus、HandlingMethod、WorkOrderStatus、ComplaintStatus、LineStatus、EquipmentStatusCode、BatchNo、SerialNo。rule 目录：warranty.rs、line_quantity.rs、work_order_guard.rs。port 目录：EquipmentRepository、ComplaintRepository、WorkOrderRepository、EquipmentStatusDictionary。

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
| PENDING_ACCEPTANCE | IN_PROGRESS、CANCELLED | 转 IN_PROGRESS 时 assignee_user_id 非空且该用户具备售后工程师角色 | 售后主管、售后工程师 |
| IN_PROGRESS | PENDING_CUSTOMER_CONFIRM、COMPLETED、CANCELLED | 转 COMPLETED 需守卫 G1 与 G2；转 CANCELLED 需守卫 G3 | 处理人、售后主管 |
| PENDING_CUSTOMER_CONFIRM | IN_PROGRESS、COMPLETED、CANCELLED | 同上 | 处理人、售后主管 |
| COMPLETED | 无 | 终态只读 | — |
| CANCELLED | 无 | 终态只读 | — |

守卫 G1：conclusion_note 非空。守卫 G2：全部登记行状态属于 {COMPLETED, VOIDED}，否则返回 SERVICE.WORK_ORDER.OPEN_LINES_EXIST 并在 details 中给出未结清登记行的行号、处理方式与关联单据编号清单。守卫 G3：不存在状态属于 {PENDING, LINKED} 的登记行，即取消前必须先作废这些行。任何非法迁移返回 SERVICE.WORK_ORDER.INVALID_STATE_TRANSITION，分类 BUSINESS_CONFLICT，HTTP 409。终态记录的任何字段修改返回 SERVICE.WORK_ORDER.TERMINAL_READ_ONLY。工单不设内置审批链，需要审批时由低代码在既有流转上加审批节点，本阶段的状态集合不因此改变。

#### 4.5 关联对象一致性校验

创建与修改工单时按固定顺序执行，任一不通过即定位到字段并阻止提交。

1. 法人一致：全部关联对象的 legal_entity_id 与工单相同。跨法人引用无入口，且 RLS 使对方法人的记录不可见。
2. 可见性：对当前安全上下文不可见的对象一律按 PLATFORM.AUTHZ.NOT_FOUND_OR_DENIED 返回 404，提示中不回显任何内容，取值按基线第 5.5 节。
3. 客户一致：设备的 customer_id、订单行的客户、合同的客户必须等于工单 customer_id，否则 SERVICE.WORK_ORDER.CUSTOMER_MISMATCH。订单行与合同的客户经 ep-contract-sales 与 ep-contract-clm 的查询 trait 取得，不直读对方表。
4. 设备带出：选择设备后带出 customer_id、product_id、batch_no、sales_order_line_id 并按 4.2 判定在保状态。
5. 订单行带出：选择订单行后带出 contract_id、product_id、batch_no，带出的 contract_id 置为只读。
6. 设备终止状态：设备的 current_status_code 在字典中 is_terminal 为 true 时，若请求未带 `terminal_equipment_confirmed` 标记则返回 SERVICE.WORK_ORDER.EQUIPMENT_TERMINAL_STATUS_CONFIRM_REQUIRED；带该标记时校验调用者具备售后主管角色，写入确认人与确认时点，并在同一事务写审计。
7. 允许为空：原订单、合同、产品、批次与设备均可为空，客户与问题描述不可为空。

#### 4.6 退换修登记行

可退数量计算，位于 ep-domain-service::rule::line_quantity::returnable。
输入：delivered_qty（由 ep-contract-sales 的 SalesOrderLineDeliveryQuery 取得的该订单行已交付数量）、registered_qty（本模块该订单行上状态属于 {PENDING, LINKED, COMPLETED} 的登记行数量之和，VOIDED 不计）、request_qty。
判定：request_qty > 0 且 request_qty ≤ delivered_qty − registered_qty，否则返回 SERVICE.WORK_ORDER_LINE.QUANTITY_EXCEEDS_RETURNABLE，details 中回带已交付数量与已登记数量。全部比较用 numeric(18,6) 对应的 Quantity 类型，不做浮点比较，不做隐式舍入。
该校验是前置校验。权威校验在 sales 创建销售退货单时再执行一次，理由是已交付数量归 sales 所有且本模块不能对其加锁；两次判定不一致时以 sales 的结论为准，本模块把登记行退回 PENDING 并返回 SERVICE.WORK_ORDER_LINE.SALES_RETURN_REJECTED。

三类处理方式的挂接：
- RETURN：登记行提交后发布 service.work_order_line.registered.v1，由 job-worker 的 service.return_repair_writeback 消费者消费该事件并在其事务内调用 `ep_contract_sales::SalesReturnCommandPort::create_sales_return(tx, ctx, cmd)` 发起销售退货单创建，成功后回写 sales_return_id 与 sales_return_line_id 并把行置为 LINKED。命令入参 CreateSalesReturn 按裁定 A-17 填写：customer_id 与 sales_order_id 取工单与登记行上的取值，return_reason 取登记行的 reason_note，return_warehouse_id 留空（本阶段不定义仓库），posting_date 取消费时点的中国标准时间自然日，source_ref 取 SalesReturnSourceRef { source_module: ModuleCode::Service, source_doc_type: "WO", source_doc_id: 工单 id, source_doc_line_id: 登记行 id }，lines 每项的 sales_order_line_id、quantity、batch_no、serial_nos 取登记行取值；登记行关联的设备带有 delivery_confirmation_line_id 时按 DeliveryLinkAssignedBy::Manual 填一条 delivery_links，否则 delivery_links 传空数组由 sales 按 AutoFifo 指派。返回的 SalesReturnView 用于回写单据编号与状态。一条登记行最多关联一张销售退货单行，按 U-J-10 暂按一对一。
- EXCHANGE：同时挂接一张销售退货单行与一张发货侧单据行。本阶段只做登记意图、挂接与回写三件事，不定义两张单据本身。是否强制配对见第 12 节 U-J-08。
- REPAIR：只做登记，填写 repair_result_note 与 repair_finished_on 后由处理人直接从 PENDING 置为 COMPLETED，不关联外部单据，不改变设备当前状态，不产生备件与成本。

登记行状态机守卫：PENDING → LINKED 需 sales_return_id 或 outbound_document_id 至少一项非空；LINKED → COMPLETED 只能由对方单据终态事件驱动，接口层不暴露人工置完成的入口（REPAIR 除外）；LINKED → PENDING 由对方单据作废或驳回事件驱动，并向处理人发站内通知；任一状态 → VOIDED 需填写 void_reason 且调用者具备售后主管角色。驱动这三条迁移的事件按裁定 A-17 固定为三个：sales.sales_return.closed.v1 驱动 LINKED → COMPLETED，sales.sales_return.cancelled.v1 与 sales.sales_return.rejected.v1 驱动 LINKED → PENDING，三者均由阶段 6 发布；既有的 sales.sales_return.registered.v1 只用于确认建单成功。RETURN 与 EXCHANGE 在未挂接单据时不得置为 COMPLETED，由 ck_work_order_lines_complete_needs_doc 与领域守卫双重拦截。

追溯三链路：从工单查全部登记行及其关联单据由本模块自身查询满足；从销售退货单反查来源工单与登记行由 ep-contract-service 的 ReturnRepairTraceQuery 提供，sales 侧详情调用该 trait；从设备档案查该设备涉及的全部工单与登记行由 ix_work_orders_legal_entity_id_equipment_record_id 与 ix_work_order_lines_legal_entity_id_equipment_record_id 支撑。

#### 4.7 合同生效派生项目任务

触发：job-worker 消费 clm 发布的 clm.contract.effective.v1，消费者名按裁定 C-19 固定为 project.contract_derivation。派生项的内容不放在事件载荷里，理由是基线第 6.1 节要求 payload 只放最小必要数据与引用 ID；本阶段在消费时调用 `ep_contract_clm::ContractDerivationPlanQuery::derivation_plan(tx, ctx, contract_id, contract_version_no)` 读取该合同该版本的派生计划，派生项由合同模板决定，本阶段不解释合同条款，与 PRD 9.7.1 的“只接收派生结果”一致。该 trait 及其 DTO 按裁定 A-16 由阶段 6 提供且形状已冻结，本阶段不再把它登记为待确认事项；按裁定 C-19 撤销 ep_contract_project::ProjectTaskDerivationPort，clm 不同步派生项目任务。该方法接受事务句柄，因此在消费者事务内调用。

派生计划的字段按裁定 A-16 固定：ContractDerivationPlan 含 contract_id、contract_version_no、derivation_batch_no、project_group_contract_id、items；ContractDerivationItem 含 item_kind、unique_key、source_contract_line_id、milestone_no、name、promised_date、quantity、owner_user_id。本阶段只消费 item_kind 为 ProjectTask 的项，DeliveryMilestone、PurchaseRequisitionLine、PaymentScheduleLine 三类整项忽略并计入日志；quantity 与 milestone_no 不落库，只用于日志与排障。

算法（单事务）：
1. 幂等前置：向 platform_msg.inbox_consumptions 插入 (consumer='project.contract_derivation', event_id)，唯一冲突即整批跳过并置 DONE。
2. 定位项目：取派生计划的 project_group_contract_id，为空时退回取 contract_id，按 (legal_entity_id, 该取值) 查 project.projects，存在则复用，不存在则取号新建，状态 IN_PROGRESS，来源合同与客户由派生计划带入。续签合同因共用根合同 id 而复用同一项目，与 PRD 9.7.6 的“新派生的任务与原任务同属一个项目”一致。
3. 数量守卫：items 长度超过配置上限时整批失败，返回 PROJECT.PROJECT_TASK.DERIVATION_LIMIT_EXCEEDED 并进入死信，理由是避免一次错误配置在单机上产生不可控写入量。
4. 逐项 upsert：按 (legal_entity_id, derivation_unique_key) 查既有任务，键取 ContractDerivationItem 的 unique_key。不存在则插入，状态 NOT_STARTED，source 取 CONTRACT_DERIVED，name 取 item 的 name，planned_finish_on 取 promised_date，planned_start_on 留空，assignee_user_id 取 owner_user_id；存在且状态属于 {COMPLETED, CANCELLED} 则跳过并计入 skipped_terminal，不覆盖终态任务；存在且状态属于 {NOT_STARTED, IN_PROGRESS} 则更新 name、planned_finish_on、assignee_user_id 与 derivation_batch_no，row_version 加一。
5. 写审计与 Outbox：逐条写 project.project_task.derived.v1，整批写一条 project.project.created.v1（仅新建项目时）。
6. 事务提交。

失败处理：按基线第 6.2 节的八次退避重试，全部失败置 DEAD 并写死信，死信按 legal_entity_id 可枚举，人工修复后记名重投，取值按规格第 15.2 章。派生的项目任务不参与规格第 8 章第 3 步的价格权限、库存可用量、交期与信用额度校验，因此不存在待放行的项目任务，派生完成即为 NOT_STARTED。

重复投递判定：同一 event_id 重复投递由 inbox_consumptions 拦截；不同 event_id 但同一 unique_key 的重复派生由 ux_project_tasks_le_derivation_unique_key 与第 4 步的 upsert 分支拦截。两层合起来保证重复投递不产生重复任务、重复事件与重复审计记录，对应退出条件中派生任务按唯一键不重复一条。裁定 A-16 的 unique_key 含 contract_version_no，因此合同变更产生新版本时同一模板项得到新键并按新键新建任务，旧版本的非终态任务不再出现在新计划中，按 U-J-13 保留原状并在项目详情中标注为来源已变更；同一版本内的重复派生仍走第 4 步的更新分支。

#### 4.8 项目与任务状态机

项目任务：NOT_STARTED → IN_PROGRESS 需 assignee_user_id 非空；IN_PROGRESS → NOT_STARTED 允许；任一非终态 → COMPLETED 时写入 actual_finish_on 为流转时点中国标准时间自然日；任一非终态 → CANCELLED 需 cancel_reason。COMPLETED 与 CANCELLED 为终态只读。
项目：IN_PROGRESS → COMPLETED 与 IN_PROGRESS → CLOSED 均需守卫 P1，即全部任务状态属于 {COMPLETED, CANCELLED}，否则 PROJECT.PROJECT.OPEN_TASKS_EXIST 并给出未结清任务清单；COMPLETED → CLOSED 允许；CLOSED 为终态，不再接受任何任务变更，任务侧的写用例在其项目为 CLOSED 时一律拒绝。守卫 P1 的取值来自第 12 节 U-J-14 的临时取值。

#### 4.9 由项目任务提交采购需求

在一个事务内加载任务 FOR UPDATE，校验任务状态属于 {NOT_STARTED, IN_PROGRESS} 且其项目状态不为 CLOSED，跨模块入口只有 `ep_contract_procure::PurchaseRequisitionIntakePort::intake(tx, ctx, cmd)` 一个，按裁定 C-17 该端口由阶段 7 提供，本阶段不直接写对方表，也不使用 PurchaseRequisitionDerivationPort 一类的旧名。由于该调用是跨模块同步命令且需要建立双向引用，本阶段采用两段式：本事务内只发布 project.project_task.requisition_requested.v1，由 job-worker 的 project.requisition_intake 消费者消费后调用该端口创建采购需求，回写 purchase_requisition_id 与 doc_no。入参 PurchaseRequisitionIntake 按裁定 C-17 填写：source_module 取 ModuleCode::Project，source_doc_id 取 project_id，source_doc_line_id 取 project_task_id，material_id、quantity、required_on 取任务上的申请取值，unique_key 取 `project.project_tasks:<project_task_id>:<本次提交的 Idempotency-Key>`，由 procure 侧据此保证不重复建单。理由是基线第 10.3 节禁止在事务内做跨模块的写编排，且一个用例一个事务。占位行的 purchase_requisition_id 在回写前不可为空这一约束因此改为：占位阶段不写 link 行，改在回写阶段一次性写入，link 表的 purchase_requisition_id 保持非空。任务侧在回写前展示为提交中，取值来源为该任务上未完成的 requisition_requested 事件，由 Outbox 状态查询给出。阶段 7 交付前，本阶段在两个 wiring.rs 注入 NoopPurchaseRequisitionIntakePort 并加注释 `// TODO(stage-7): replace with real impl`。

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
2. 对已注册的提供者并发扇出，并发上限取配置，每个区块各自使用只读分析池的一个连接，语句超时按只读池取值，另在应用侧对每个区块施加 section_timeout_ms 的超时。
3. 未注册提供者的区块返回 section_status 为 NOT_AVAILABLE；超时或失败的区块返回 DEGRADED 并计一次 ep_crm_customer360_section_degraded_total，不使整个请求失败，理由是客户 360 是查询类视图，单一区块不可用不应阻断其余区块。
4. 每区块内按 occurred_on 降序、object_id 降序截断到 size 条，size 默认 20、上限 50。
5. 全部区块的字段级裁剪与密级过滤由 ep-platform-authz 在提供者内部完成，聚合层不做二次裁剪，也不做跨区块排序，避免通过排序位次间接暴露无权数据，取值按规格第 7.9 章。

本阶段实现 Complaints、Equipments、WorkOrders 三个提供者，位于 ep-app-service，其中 EquipmentsSectionProvider 是设备在客户 360 中的唯一可见路径，按裁定 B-06 不经任何跨模块 trait；Contracts 与 Receipts 由 ep-app-clm 与 ep-app-finance 实现。端点与契约在阶段 5 已启用并只挂载 mdm 自己的区块，本阶段接管后追加上述三个区块，未注册的区块按 3 的规则返回 NOT_AVAILABLE。三个自实现提供者的取数各命中一条索引：ix_customer_complaints_legal_entity_id_customer_id_complaint_on、ix_equipment_records_legal_entity_id_customer_id、ix_work_orders_legal_entity_id_customer_id_created_at。

#### 4.11 与账务和库存的边界

本阶段的任何用例都不写 ledger 与 inventory 的任何表，也不发布会计事件。事件-分录表在规格第 5.2 章财务规则条目，本阶段不复述借贷与取价。退换修产生的实物出入库与账务后果一律由销售退货单在其所属模块按规格第 5.2 章财务规则条目的销售退货事件与退款事件承接。这一边界是本阶段退出条件之一，判定方式见 8.5。

---

### 5. API 契约

全部端点遵循基线第 5 节：路径前缀 /api/v1，字段 snake_case，成功与失败封套固定，写请求必带 Idempotency-Key、Authorization、X-Legal-Entity-Id、X-Device-Id、X-Client，分页参数 page 与 page_size（默认 20、上限 200），排序 sort 白名单，过滤 filter[<field>]=<op>:<value>。本阶段无高风险操作，因此不要求 X-Reauth-Token；含联系方式的导出除外，其重新认证由平台导出能力承担。

按裁定 A-20，本阶段每个用例在 crates/contract/service/src/capability.rs 与 crates/contract/project/src/capability.rs 中声明一对常量 `<USECASE_SCREAMING>_DOMAIN` 与 `<USECASE_SCREAMING>_ACTION`，售后侧的能力域一律取 `CapabilityDomain::ServiceWorkorderEquipment`，项目侧一律取 `CapabilityDomain::ProjectTaskMilestone`，动作类别取 `ActionClass` 的 Read、Write、Submit 之一，本阶段没有 Approve 与 Export 路由。客户 360 端点的一对常量随 `CapabilityDomain::CrmCustomer360` 由阶段 5 在 crates/contract/crm/src/capability.rs 中声明，本阶段不重复声明。两个枚举由阶段 1 在 ep-foundation 冻结，`xtask configdoc` 断言每个 `/api/v1/` 路由都能解析到一对常量，缺失即构建失败。

#### 5.1 设备档案

| 方法与路径 | 说明 | 主要请求字段 | 响应 | 主要错误码 | 权限 |
|---|---|---|---|---|---|
| GET /api/v1/service/equipments | 设备列表 | filter 支持 customer_id、product_id、model、current_status_code、warranty_status、delivered_on(between)；sort 白名单 created_at、code、delivered_on | data 为设备摘要数组，warranty_status 实时计算，meta 为分页 | — | service.equipment_record.read |
| GET /api/v1/service/equipments/{id} | 设备详情 | — | 含实时在保状态、状态变更历史（取自审计）、附件清单 | PLATFORM.AUTHZ.NOT_FOUND_OR_DENIED | 同上 |
| POST /api/v1/service/equipments | 手工新增 | model、customer_id、serial_no、product_id、batch_no、delivered_on、installed_on、current_status_code、保修四项、remark | 201 与设备详情 | SERVICE.EQUIPMENT_RECORD.WARRANTY_RANGE_INVALID、INSTALL_BEFORE_DELIVERY、DELIVERY_DATE_IN_FUTURE、STATUS_UNKNOWN | service.equipment_record.create |
| POST /api/v1/service/equipments/actions/create-from-delivery-batch | 从交付确认单逐台建档，delivery_confirmation_id 指向 sales.delivery_confirmations | delivery_confirmation_id、lines 数组（line_id、count、model、保修四项），单次上限 200 | data 为生成的设备数组与跳过清单 | VALIDATION 超限、SERVICE.EQUIPMENT_RECORD.* | service.equipment_record.create |
| PATCH /api/v1/service/equipments/{id} | 修改非保修字段 | 允许 model、product_id、serial_no、installed_on、remark；row_version 必填 | 设备详情 | PLATFORM.CONCURRENCY.STALE_VERSION | service.equipment_record.update |
| POST /api/v1/service/equipments/{id}/actions/change-status | 变更当前状态 | to_status_code、reason、row_version | 设备详情 | SERVICE.EQUIPMENT_RECORD.STATUS_UNKNOWN | service.equipment_record.change-status |
| POST /api/v1/service/equipments/{id}/actions/update-warranty | 维护保修信息 | 保修四项、reason、row_version | 设备详情 | SERVICE.EQUIPMENT_RECORD.WARRANTY_EDIT_FORBIDDEN、WARRANTY_RANGE_INVALID | service.equipment_record.maintain-warranty，仅售后主管 |
| GET /api/v1/service/equipments/{id}/work-orders | 按设备查工单与登记行 | 分页 | 工单摘要与登记行摘要 | — | service.work_order.read |

序列号重复不阻断提交，重复时在 meta.warnings 中回带 SERVICE.EQUIPMENT_RECORD.SERIAL_NO_DUPLICATED 与已存在的设备编号。

按裁定 B-06，设备的跨模块可见性只保留三条路径：本节的 GET /api/v1/service/equipments 与 /{id}、全文检索索引中 object_type 为 service.equipment_records 的文档、以及本阶段自实现的 EquipmentsSectionProvider。不提供 ep-contract-service::EquipmentQuery，报表侧的设备取数一律经受治理数据集视图，低代码的设备引用经上述 HTTP 端点解析。

#### 5.2 客户投诉

| 方法与路径 | 说明 | 幂等语义 | 主要错误码 |
|---|---|---|---|
| GET /api/v1/service/customer-complaints | 列表，filter 支持 status、customer_id、complaint_on(between)、accepted_by | — | — |
| GET /api/v1/service/customer-complaints/{id} | 详情，含反查得到的关联工单编号 | — | PLATFORM.AUTHZ.NOT_FOUND_OR_DENIED |
| POST /api/v1/service/customer-complaints | 登记，状态直接为 REGISTERED | 四元组幂等，重放回带首次结果与 Idempotent-Replay: true | VALIDATION 各字段 |
| PATCH /api/v1/service/customer-complaints/{id} | 非终态下修改可编辑字段 | 需 row_version | PLATFORM.CONCURRENCY.STALE_VERSION、SERVICE.CUSTOMER_COMPLAINT.TERMINAL_READ_ONLY |
| POST /api/v1/service/customer-complaints/{id}/actions/accept | 受理并填写受理人 | 同上 | SERVICE.CUSTOMER_COMPLAINT.INVALID_STATE_TRANSITION |
| POST /api/v1/service/customer-complaints/{id}/actions/close | 关闭并填写处理说明 | 同上 | SERVICE.CUSTOMER_COMPLAINT.HANDLING_NOTE_REQUIRED |
| POST /api/v1/service/customer-complaints/{id}/actions/cancel | 取消并填写原因，限售后主管 | 同上 | PERMISSION_DENIED |
| POST /api/v1/service/customer-complaints/{id}/actions/escalate-to-work-order | 升级为工单 | 同一 Idempotency-Key 重放返回首次工单；不同键的重复升级返回 409 | SERVICE.CUSTOMER_COMPLAINT.ALREADY_ESCALATED |

#### 5.3 售后工单

| 方法与路径 | 说明 | 主要错误码 |
|---|---|---|
| GET /api/v1/service/work-orders | 列表，filter 支持 status、work_order_type_code、customer_id、assignee_user_id、created_at(between)、warranty_status、has_open_lines(eq:true/false)；默认筛选期间最近 3 个自然月 | — |
| GET /api/v1/service/work-orders/{id} | 详情，含登记行、处理记录、附件 | PLATFORM.AUTHZ.NOT_FOUND_OR_DENIED |
| POST /api/v1/service/work-orders | 创建（草稿或直接提交由 submit 标记决定）；三个入口共用本端点，入口差异只体现在预填 | SERVICE.WORK_ORDER.CUSTOMER_MISMATCH、EQUIPMENT_TERMINAL_STATUS_CONFIRM_REQUIRED |
| PATCH /api/v1/service/work-orders/{id} | 非终态下修改；warranty_status 与 warranty_judged_on 为只读，传入即 VALIDATION | SERVICE.WORK_ORDER.TERMINAL_READ_ONLY |
| POST /api/v1/service/work-orders/{id}/actions/submit | DRAFT → PENDING_ACCEPTANCE，写 submitted_at 并登记时限定时器 | SERVICE.WORK_ORDER.INVALID_STATE_TRANSITION |
| POST /api/v1/service/work-orders/{id}/actions/assign | 指派或自行受理，写 assignee_user_id 与 accepted_at | SERVICE.WORK_ORDER.ASSIGNEE_REQUIRED |
| POST /api/v1/service/work-orders/{id}/actions/request-customer-confirmation | IN_PROGRESS → PENDING_CUSTOMER_CONFIRM | 同上 |
| POST /api/v1/service/work-orders/{id}/actions/resume-processing | PENDING_CUSTOMER_CONFIRM → IN_PROGRESS | 同上 |
| POST /api/v1/service/work-orders/{id}/actions/complete | 完成，守卫 G1 与 G2 | SERVICE.WORK_ORDER.OPEN_LINES_EXIST、CONCLUSION_REQUIRED |
| POST /api/v1/service/work-orders/{id}/actions/cancel | 取消，守卫 G3，限售后主管 | SERVICE.WORK_ORDER.OPEN_LINES_EXIST |
| POST /api/v1/service/work-orders/{id}/logs | 追加一条处理记录，只追加不覆盖 | VALIDATION |
| GET /api/v1/service/work-orders/{id}/logs | 处理记录列表，按 created_at、id 升序 | — |
| POST /api/v1/service/work-orders/{id}/lines | 新增登记行 | SERVICE.WORK_ORDER_LINE.QUANTITY_EXCEEDS_RETURNABLE、SALES_ORDER_LINE_REQUIRED、SERVICE.WORK_ORDER.MAX_LINES_EXCEEDED |
| GET /api/v1/service/work-orders/{id}/lines | 登记行列表 | — |
| PATCH /api/v1/service/work-orders/{id}/lines/{line_id} | PENDING 状态下修改数量与说明 | BUSINESS_CONFLICT |
| POST /api/v1/service/work-orders/{id}/lines/{line_id}/actions/link-sales-return | 手工挂接已存在的销售退货单行 | SERVICE.WORK_ORDER_LINE.ALREADY_LINKED |
| POST /api/v1/service/work-orders/{id}/lines/{line_id}/actions/link-outbound | 挂接发货侧单据行 | 同上 |
| POST /api/v1/service/work-orders/{id}/lines/{line_id}/actions/complete-repair | 维修登记完成，写维修结果与完成日期 | BUSINESS_CONFLICT 处理方式非 REPAIR |
| POST /api/v1/service/work-orders/{id}/lines/{line_id}/actions/void | 作废登记行，限售后主管 | PERMISSION_DENIED |
| GET /api/v1/service/work-order-lines | 跨工单的登记行清单，filter 支持 handling_method、status、has_linked_document | — |

#### 5.4 项目与项目任务

| 方法与路径 | 说明 | 主要错误码 |
|---|---|---|
| GET /api/v1/project/projects | 列表，filter 支持 status、customer_id、owner_user_id、source_contract_id | — |
| GET /api/v1/project/projects/{id} | 详情，含任务统计与附件 | PLATFORM.AUTHZ.NOT_FOUND_OR_DENIED |
| POST /api/v1/project/projects | 手工新建 | VALIDATION |
| PATCH /api/v1/project/projects/{id} | 修改，需 row_version | PLATFORM.CONCURRENCY.STALE_VERSION |
| POST /api/v1/project/projects/{id}/actions/complete | 完成，守卫 P1 | PROJECT.PROJECT.OPEN_TASKS_EXIST |
| POST /api/v1/project/projects/{id}/actions/close | 关闭，守卫 P1 | 同上 |
| GET /api/v1/project/project-tasks | 跨项目任务列表，filter 支持 project_id、status、assignee_user_id、planned_finish_on(between)、source | — |
| GET /api/v1/project/projects/{id}/tasks | 项目下任务列表 | — |
| POST /api/v1/project/projects/{id}/tasks | 手工新增任务，source 固定 MANUAL | VALIDATION |
| PATCH /api/v1/project/project-tasks/{id} | 修改任务 | PROJECT.PROJECT_TASK.TERMINAL_READ_ONLY |
| POST /api/v1/project/project-tasks/{id}/actions/start | NOT_STARTED → IN_PROGRESS | PROJECT.PROJECT_TASK.ASSIGNEE_REQUIRED |
| POST /api/v1/project/project-tasks/{id}/actions/revert-to-not-started | IN_PROGRESS → NOT_STARTED | PROJECT.PROJECT_TASK.INVALID_STATE_TRANSITION |
| POST /api/v1/project/project-tasks/{id}/actions/complete | 完成并写实际完成日期 | 同上 |
| POST /api/v1/project/project-tasks/{id}/actions/cancel | 取消并填写原因 | 同上 |
| POST /api/v1/project/project-tasks/{id}/actions/submit-purchase-requisition | 提交采购需求，异步回写 | PROJECT.PROJECT_TASK.REQUISITION_ALREADY_LINKED |
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

全部写用例按基线第 10.3 节的工作单元闭包表达，入口为 `UnitOfWork::transact(ctx, |tx| …)`，一个用例一个事务，隔离级别 READ COMMITTED，业务事务不超过 5 秒，读写池 statement_timeout 10 秒、lock_timeout 3 秒。事务内一律不做外部 HTTP 调用、不读写附件正文、不发通知、不等待用户输入。跨模块的只读同步查询（如取已交付数量、校验客户一致）在事务开始前完成并把结果作为入参传入闭包，理由是这些调用虽在同进程内但仍是跨模块调用，放在事务内会拉长事务并把对方模块的锁等待引入本事务。例外是按裁定 A-16、A-17、C-17 冻结签名中带 `&mut dyn Tx` 的三个调用 ContractDerivationPlanQuery::derivation_plan、SalesReturnCommandPort::create_sales_return 与 PurchaseRequisitionIntakePort::intake，三者按签名必须在调用方事务内执行，且三者都只出现在 job-worker 的消费者事务里，不出现在 core-server 的交互式用例里，事务句柄类型为 ep_foundation::port::Tx。

一个事务内写入的内容固定为三类并集：业务状态、审计事件、Outbox 条目。三者同事务是规格第 8 章事务边界与基线第 6.2 节的硬要求。

#### 6.2 锁策略与锁序

统一锁序为：先工单行，再登记行，再处理记录。任何涉及登记行的用例都先对其工单行执行 `select … from service.work_orders where id = $1 and legal_entity_id = … for update`，再对登记行集合执行 `for update`。理由是工单完成的守卫要读全部登记行，而登记行新增会改变该集合，固定锁序把两者串行化并避免与新增登记行之间形成死锁。项目侧同理，先项目行再任务行。

乐观锁：全部可更新表带 row_version，更新语句按基线第 3.7 节写为带 row_version 条件的 UPDATE，受影响行数为 0 即 PLATFORM.CONCURRENCY.STALE_VERSION、HTTP 409，响应回带当前版本号与最后修改人。工单状态流转同时使用悲观行锁与乐观版本校验：行锁保证守卫读到的登记行集合稳定，版本校验保证客户端提交的是它看到的那一版。

序列化失败 40001 与死锁 40P01 由数据访问层统一重试 3 次，退避 50、150、450 毫秒，且只对尚未产生任何外部可见副作用的事务重试。本阶段全部写用例的外部可见副作用只有 Outbox 条目，而 Outbox 与业务写入同事务、回滚即消失，因此全部写用例可重试。

#### 6.3 幂等

- HTTP 写请求：四元组幂等键，存 platform_msg.idempotency_keys，与业务写入同事务，保留 7 天。重放返回首次结果并带 Idempotent-Replay: true；键相同而 request_hash 不同返回 409 与 PLATFORM.IDEMPOTENCY.PAYLOAD_MISMATCH。
- 事件消费：三个消费者各自使用 platform_msg.inbox_consumptions 的 (consumer, event_id) 唯一约束，消费副作用与该行插入同事务。消费者名固定为 project.contract_derivation（按裁定 C-19，消费 clm.contract.effective.v1）、project.requisition_intake（消费 project.project_task.requisition_requested.v1）与 service.return_repair_writeback（消费本模块自身的 service.work_order_line.registered.v1 与裁定 A-17 冻结的三个销售退货终态事件）。
- 业务层兜底：项目任务按 (legal_entity_id, derivation_unique_key) 唯一；投诉升级按 (legal_entity_id, source_complaint_id) 唯一；采购需求引用按 (legal_entity_id, purchase_requisition_id) 唯一。三者使幂等不只依赖消息层。

#### 6.4 与 Outbox 的关系

本阶段发布的全部事件走 platform_msg.outbox_events，信封字段按基线第 6.1 节完整填写。本阶段的事件不承载会计语义，因此 posting_date 与 accounting_period_id 两个信封字段一律置空，且本阶段不向 ledger.posting_trigger_event_types 登记任何行。按裁定 C-28 的受理前提二判定语句，posting_date 为空且不命中该注册表的事件一律不计入待消费过账条目数，本阶段的事件因此两条都不满足计入条件。这一点需在事件目录中对本阶段的 25 个事件逐条标注为非过账事件，避免关账受理判定把它们计入。

取件、批量 100、轮询 200 毫秒、退避 8 档、死信与重投一律沿用基线，不另建机制。

#### 6.5 失败重试与补偿

| 失败点 | 处理 |
|---|---|
| 派生项目任务时 clm 的派生项查询不可用 | 按 EXTERNAL 之外的 INFRASTRUCTURE 处理，事件退避重试；八次后进死信 |
| 创建销售退货单被 sales 拒绝（数量、状态、法人） | 登记行退回 PENDING，写审计，向处理人发站内通知，不重试；错误码 SERVICE.WORK_ORDER_LINE.SALES_RETURN_REJECTED |
| 创建销售退货单超时但对方可能已成功 | 由 sales 的命令端口幂等键保证不重复创建，重试安全；超过八次退避后进死信并人工修复 |
| 销售退货单终态事件迟到或乱序 | 回写用例做状态收敛：只允许 LINKED → COMPLETED 与 LINKED → PENDING，非法迁移记 WARN 并置 DONE，不进死信 |
| 采购需求创建失败 | 不写 link 行，事件退避重试；进死信后任务侧提交中标记由死信状态解释 |
| 站内通知发送失败 | 平台侧重试，不阻断本阶段任何状态流转，取值按 PRD 9.9 |

本阶段没有需要补偿的多步写编排：登记行与销售退货单之间是引用关系而非资金或库存后果，销售退货单被作废时登记行退回 PENDING 即为完整的反向路径，不需要 Saga 补偿分支。

---

### 7. 配置项

全部键按基线第 7.1 节前缀 EP__、双下划线分层，结构体开启 deny_unknown_fields。运行期可变的业务参数不进配置文件，因此工单提醒阈值、三张字典的取值、列表默认列均落在数据库并经配置发布通道发布。该通道按裁定 A-27 由阶段 3b 提供，本阶段只作为使用方接入，不自建第二套发布路径。

| 键 | 类型 | 默认值 | 生效方式 | 说明 |
|---|---|---|---|---|
| EP__SERVICE__WORK_ORDER__MAX_LINES_PER_ORDER | u16 | 200 | 启动时读取，改动需重启 core-server | 与基线批量上限 200 对齐 |
| EP__SERVICE__WORK_ORDER__REMINDER_TIMER_ENABLED | bool | true | 启动时读取 | 关闭后不登记时限定时器，用于迁移窗口 |
| EP__SERVICE__EQUIPMENT__CREATE_FROM_DELIVERY_MAX_ROWS | u16 | 200 | 启动时读取 | 单次从交付确认单建档的上限 |
| EP__SERVICE__EQUIPMENT__SERIAL_NO_DUPLICATE_CHECK | bool | true | 启动时读取 | 关闭后不做重复提示，仅用于历史导入窗口 |
| EP__PROJECT__DERIVATION__MAX_TASKS_PER_CONTRACT | u16 | 500 | 启动时读取，core-server 与 job-worker 取同值 | 单次派生的任务条数上限 |
| EP__PROJECT__DERIVATION__PLAN_QUERY_TIMEOUT_MS | u32 | 3000 | 启动时读取 | 读取合同派生项的超时 |
| EP__CRM__CUSTOMER_360__DEFAULT_SECTION_SIZE | u16 | 20 | 启动时读取 | 未传 section_size 时的默认值，取值来源见第 12 节 U-J-15 |
| EP__CRM__CUSTOMER_360__MAX_SECTION_SIZE | u16 | 50 | 启动时读取 | 请求超过即 VALIDATION |
| EP__CRM__CUSTOMER_360__SECTION_TIMEOUT_MS | u32 | 1500 | 启动时读取 | 单区块超时，超时即 DEGRADED |
| EP__CRM__CUSTOMER_360__PROVIDER_CONCURRENCY | u8 | 5 | 启动时读取 | 区块扇出并发上限，不超过只读分析池上限 10 的一半 |

本阶段不引入新的机密引用，不改动机密库结构。本阶段在启动自检中不新增检查项；按裁定 C-25 自检项一律按注册名标识，基线项 rls-enabled-and-forced 自然覆盖本阶段新增的 18 张表。

---

### 8. 测试计划

#### 8.1 单元测试

位于被测 crate 内，不触网、不触库、不触文件系统、不取真实时间，时间一律经 FixedClock 注入。

- 在保状态判定：4 个取值的正例各 1 条；边界 6 条（judge_on 等于 start、等于 end、start 等于 end、start 为空、end 为空、两者均为空）；proptest 属性 3 条（结果必落在四取值之一；start 与 end 均非空时四取值互斥且覆盖全体日期；把 judge_on 沿时间轴推进，结果序列只能按 NOT_STARTED → IN_WARRANTY → EXPIRED 单调前进）。
- 工单状态机：6×6 共 36 组迁移逐一断言，其中合法 10 条、非法 26 条；守卫 G1、G2、G3 各覆盖通过与拒绝两条。
- 投诉状态机 4×4 共 16 组；登记行状态机 4×4 共 16 组；任务状态机 4×4 共 16 组；项目状态机 3×3 共 9 组。
- 可退数量：等于边界通过、超出最小单位（1e-6）拒绝、已作废行不计入、多行累加、已交付为零时任何数量均拒绝、负数与零拒绝。
- 派生 upsert 决策函数：插入、更新、跳过终态三分支各 1 条；同一批次内重复 key 拒绝；items 超上限拒绝。
- 客户 360 合并：截断到 size、排序稳定性、区块超时降级、未注册区块返回 NOT_AVAILABLE、无权区块返回空且状态为 OK。
- 编号格式与文本长度校验：各类型码前缀正确、长度超限返回 VALIDATION 且定位到字段。

工具为 cargo test、rstest 参数化、insta 快照（错误响应体）、proptest（在保判定与可退数量两组）。

#### 8.2 集成测试

使用真实 PostgreSQL 16，每个用例独占一个 ep_test_<nanoid> 数据库，用例结束删库；数据一律经 ep-testkit 构造器与用例路径产生，禁止手写 INSERT。

场景清单：

1. 设备三条建档路径各一条：手工、从交付确认单批量、迁移接口写入并带迁移批次标识。
2. 从交付确认单重复建档：同一交付确认行第二次提交被识别为已建档并跳过，返回跳过清单。
3. 保修信息修改：售后主管可改、售后工程师被拒（403）、修改写审计并保留修改前取值、不回溯改写既有工单的在保快照。
4. 设备终止状态：未确认时 409、售后主管确认后可创建工单、确认写审计。
5. 投诉全状态路径：登记 → 受理 → 关闭；登记 → 取消；终态修改被拒。
6. 投诉升级：成功一次；同一投诉第二次升级返回 409 并回带既有工单编号。
7. 工单三个创建入口产生的对象与状态机一致（同一断言集跑三遍）。
8. 关联一致性七条校验逐条命中：法人不一致、对象不可见、客户不一致、设备带出、订单行带出、终止状态、允许为空。
9. 登记行三类处理方式：退货经 SalesReturnCommandPort::create_sales_return 生成销售退货单（端口用 ep-testkit 的 SalesReturnPortFake 与 wiremock 双实现各跑一遍，两者均按裁定 A-17 的签名实现）、换货挂两张单据、维修直接完成。
10. 回写：sales.sales_return.closed.v1 驱动登记行到 COMPLETED；sales.sales_return.cancelled.v1 与 sales.sales_return.rejected.v1 驱动退回 PENDING 并发通知；三个事件的乱序与迟到各一条状态收敛用例。
11. 工单完成守卫：存在 PENDING 行时被拒并回带清单；全部行终态后通过。工单取消守卫同理。
12. 派生：一次合同生效派生出 N 条任务与 1 个项目；同一事件重复投递 5 次仅一套任务、一套事件、一套审计记录；同一合同版本内重复派生时终态任务被跳过、非终态任务被更新；合同变更产生新版本时按新的 unique_key 建新任务而旧版本的非终态任务保持原状并标注为来源已变更；派生计划中 item_kind 非 ProjectTask 的项被整项忽略；续签合同复用同一项目。
13. 派生失败：派生项查询持续失败，八次退避后进死信，死信按法人可枚举，重投成功后任务正确。
14. 项目任务提交采购需求：经 PurchaseRequisitionIntakePort::intake 回写建立双向引用；同一 unique_key 重复提交在 procure 侧不重复建单；同一采购需求重复回写被唯一约束拦截；阶段 7 交付前该用例跑 NoopPurchaseRequisitionIntakePort 分支并断言任务停留在提交中。
15. 客户 360：三个自实现区块返回正确数据；未注册的合同与回款区块返回 NOT_AVAILABLE；人为注入超时的区块返回 DEGRADED 且其余区块正常。
16. 处理记录只追加：追加成功、UPDATE 与 DELETE 语句在 CI 静态检查中被拦截、更正说明经 reverses_id 关联。
17. 附件关联：四张附件表的挂接与解除挂接、附件正文不落业务表列。
18. 字段级加密：联系方式写入为密文、按该字段过滤与排序的请求返回 VALIDATION、日志与错误响应中不出现明文。
19. 受治理数据集视图：project.v_projects_dataset 存在且含 legal_entity_id、security_level、data_scope_tags 三列；ep_analyst_ro 可 SELECT 而任何写语句被拒；视图列名与类型签名与 reporting.dataset_fields 的登记逐列一致。

RLS 与越权：本阶段 18 张表全部纳入 tests/rls_matrix，覆盖读取、写入、更新、删除、聚合、排序、报表投影与错误信息泄漏八类；另覆盖两个复制角色与内部对账系统安全上下文的入口借用测试。该测试目标属发布门禁项。

并发：命中基线第 8.4 节六组必测场景中的第一组（同一单据的乐观锁冲突）与第六组（Outbox 同一事件重复投递不少于 3 次），并追加本阶段特有的三组：两个用户并发升级同一投诉（恰好一个成功）；一个用户完成工单同时另一个用户新增登记行（按锁序串行化，后者要么被守卫拒绝要么在完成前入库并使完成被拒，不出现完成后仍有 PENDING 行）；两个用户对同一订单行并发登记退货（本模块前置校验可能同时通过，sales 侧权威校验拒绝其一，被拒的登记行退回 PENDING 且不产生第二张退货单）。

#### 8.3 端到端测试

- E2E-01 闭环第 12 步：售后技术支持记录形成工单，关联原订单、合同、产品、批次、设备与保修并读取在保状态，工单与投诉进入客户 360 视图。本阶段只交付该步的用例片段，落点为 testkit/scenarios/stage12_service_step12.rs 中的步骤函数与断言，由阶段 9b 的 testkit/scenarios/golden_loop_14_steps.rs 在第 12 步引用；整条链路的串接执行归该用例，本阶段不另建第二条链路用例。
- E2E-02 退换修打通：一条退货登记行生成销售退货单，退货单到终态后登记行回写为已完成；一条换货登记行挂接退货侧与发货侧两张单据；工单、销售退货单与设备档案三处追溯链路双向可达。
- E2E-03 派生幂等：合同生效派生项目任务，重复投递不产生重复任务；派生失败进死信并可人工修复。
- E2E-04 四端：售后工单与设备台账能力域在 Windows、macOS、iOS、Android 四端按完整取值执行同一场景集（Playwright 驱动桌面 WebView 与 tauri-driver 驱动壳，XCUITest 与 Espresso 驱动移动端）；项目任务与交付节点能力域桌面两端完整、移动两端简化；移动端相机扫码录入序列号与批次可用。界面代码按裁定 A-23 位于 clients/desktop/src/modules/service/、clients/desktop/src/modules/project/ 与 clients/mobile/src/modules/ 下的同名目录，客户 360 视图并入阶段 5 已建立的 crm 模块目录，均由本阶段交付；阶段 13 只提供客户端壳、路由注册表与能力矩阵闸，不交付本阶段的业务界面。
- E2E-05 时限提醒：待受理停留超阈值向售后主管送达站内通知；期望完成时间临近与超出向处理人与售后主管送达；无移动推送通道的部署下站内通知照常送达。
- E2E-06 客户 360：销售角色查询该客户的历史合同、回款、投诉、设备与服务记录；无权客户返回 404。

#### 8.4 性能相关项

在 ep-datagen 的 A.3 基准数据集（另加本阶段追加的设备、工单、投诉、项目、任务数据）与附录 A.4 的 20 并发负载下：

- 售后工单创建按附录 A.1 普通交易提交度量项判定，P95 ≤ 3 秒；客户投诉登记共用该度量项。
- 按附录 A.1 末段允许新增度量项的规则，新增一个常规交互度量项“客户 360 视图加载”，通过线沿用规格第 16 章常规交互 P95 ≤ 2 秒，不改动既有通过线。
- 工单列表、投诉列表、设备列表、按设备查工单、按订单或合同查工单、登记行清单、项目任务列表七个查询逐一给出 EXPLAIN 证据，在基准数据集上不得出现顺序扫描。
- 每场景样本不少于 200 次，只取负载稳定段，单次运行错误率超过 0.1% 即该次运行无效。
- 时延与容量通过线的最终判定在阶段 4 统一执行，本阶段只需给出本地实测证据与 EXPLAIN 证据。

#### 8.5 不变量与边界测试

- 执行本阶段全部集成与 E2E 用例前后，用 ep-platform-recon 的语句集在同一 REPEATABLE READ 快照上核对规格第 17.3 章的库存数量守恒、存货金额账与数量账一致、子账与总账勾稽三项，差额为零且取值不变。
- 直接断言本阶段用例前后 ledger.vouchers、ledger.voucher_lines、inventory 的数量流水与金额流水四张表的行数与校验和不变，即本阶段确未生成任何总账凭证与库存流水。这是 PRD 9.11 第四条验收要点的可执行形式。
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

下列 22 项全部达成才算本阶段完成，每项均可客观判定。

1. 18 张表的迁移在空库上顺序执行成功，回退段可执行，refinery 历史表记录完整。
2. 18 张表全部 ENABLE 且 FORCE 行级安全，策略名与模板一致；启动自检项 rls-enabled-and-forced 通过。
3. tests/rls_matrix 中本阶段 18 张表的八类越权用例全绿，无内容回显、无排序与聚合侧信道。
4. 53 个 HTTP 端点全部具备集成测试且全绿，封套、分页、排序白名单、过滤运算符、幂等头四项由统一的契约测试断言。
5. 25 个事件在 docs/event-catalog.md 登记，命名为四段过去分词形式，信封字段完整，并逐条标注为非过账事件。
6. 全部错误码在 docs/error-codes.md 与 ep-foundation::error::codes 两处登记且一致，CI 校验通过，代码中不内联中文文案。
7. 五个新增指标在基线第 9.2 节登记并由 ops-agent 暴露。
8. 工单六状态、投诉四状态、登记行四状态、任务四状态、项目三状态的全部迁移组合有单元测试断言，非法迁移一律 409。
9. 在保状态判定的四取值与六个边界有测试，且属性测试通过。
10. 投诉最多升级一次由数据库唯一索引保证，并有并发用例证明恰好一个成功。
11. 派生幂等：同一事件重复投递 5 次只产生一套任务、事件与审计；派生任务按裁定 A-16 的 unique_key 不重复；终态任务不被覆盖；续签复用同一项目。
12. 派生失败进入死信并可记名重投，死信按法人可枚举。
13. 三条追溯链路双向可达的 E2E 用例全绿。
14. 闭环第 12 步的用例片段已交付为 testkit/scenarios/stage12_service_step12.rs 中的步骤函数与断言，其自身在本阶段单独跑通，并可被阶段 9b 的 testkit/scenarios/golden_loop_14_steps.rs 引用；整条链路的串接通过由阶段 9b 的该用例承担，不在本阶段判定。
15. 执行本阶段全部用例前后，规格第 17.3 章三项不变量取值不变，且凭证与库存流水四张表的行数与校验和不变。
16. 四端 E2E 按规格第 6.2 章矩阵取值通过：售后工单与设备台账四端完整，项目任务与交付节点桌面完整、移动简化，移动端扫码可用。
17. 覆盖率达到 8.6 节的五档门槛。
18. 本阶段新增决定（第 13 节）已回写共享技术基线，未决事项（第 12 节）的临时取值已在代码中集中于一处常量或一张字典表，切换代价可核对。
19. 本模块在规格第 6.2 章能力矩阵中取值为完整或简化的能力域，其四端界面已实现并通过 Playwright 与 tauri-driver 的桌面用例、XCUITest 与 Espresso 的移动用例；取值为 VIEW_ONLY 的能力域只实现只读视图；取值为 NOT_APPLICABLE 的不实现入口。
20. project.v_projects_dataset 已发布并授予 ep_analyst_ro，dataset code 为 project_projects，列签名已同步给阶段 11，阶段 11 的启动自检项 reporting-dataset-signature-matched 对该视图由已登记但未发布的降级放行转为强制。
21. 本阶段全部 /api/v1/ 路由的能力域码与动作类别常量已在 crates/contract/service/src/capability.rs 与 crates/contract/project/src/capability.rs 中声明，xtask configdoc 通过。
22. 本模块的 MasterReferenceCounter 实现 ServiceReferenceCounter 已实现并注册进阶段 5 提供的 MasterReferenceCounterRegistry；按裁定 A-15 的实现清单，本阶段不承担任何 TradeHistoryProvider。

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
| 9.3.5 | 保修信息维护 | update-warranty 端点，限售后主管，不回溯 |
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
| R-01 | sales 的销售退货单命令端口与终态事件交付晚于本阶段的联调窗口 | E2E-02 与退出条件第 13 项延后 | 接口按裁定 A-17 已冻结，testkit 的 SalesReturnPortFake 按该签名实现，三个终态事件用事件夹具先行验证本侧全部分支；契约就绪后只替换装配，不改用例；调整后的顺序中阶段 6 排在阶段 12 之前，不再存在接口形状未定的风险 |
| R-02 | 派生计划的 unique_key 含 contract_version_no，合同变更后同一模板项得到新键 | 跨版本重新派生退化为新建任务，PRD 9.7.6 的更新语义只在同版本内成立 | 接口形状按裁定 A-16 已冻结，本阶段照此实现并把跨版本处置对齐 U-J-13：旧版本非终态任务保留并标注来源已变更；该退化已在 4.7 重复投递判定一段写明，留待人工复核键格式 |
| R-03 | 客户 360 的区块注册顺序依赖阶段 5 已建立的契约与端点 | 区块缺位时用户看到 NOT_AVAILABLE | 归属按裁定 C-09 已定死：唯一端点与唯一契约由阶段 5 建立，本阶段只追加三个区块实现，不新增路径；未注册区块显式返回 NOT_AVAILABLE 而非报错 |
| R-04 | 设备序列号唯一性未定（U-J-03），存量数据可能出现重复 | 决策落地后需去重再加唯一索引 | 现在即记录重复提示与被提示的设备对，落在审计中，决策后可据此批量核对 |
| R-05 | 三张字典的取值集合未定（U-A-07、U-J-01、U-J-05），出厂取值可能与客户口径不符 | 统计与下钻维度不稳定 | 字典化而非 CHECK 枚举，改取值不触发 DDL；已引用的取值只允许停用不允许删除 |
| R-06 | 客户 360 在 20 并发下扇出五个区块，可能击穿常规交互 2 秒通过线 | 附录 A 判定不过 | 区块并发上限与单区块超时可配；三个自实现区块各命中一条索引；必要时把 section_size 默认下调到 10，代价是一次配置变更 |
| R-07 | 工单完成守卫的锁序若被后续用例破坏会产生死锁 | 偶发 40P01 | 锁序写入 ep-app-service::tx 的注释与一条集成测试（并发完成与并发新增登记行）固化 |
| R-08 | 联系方式字段级加密的实现约定可能与平台安全阶段的约定冲突 | 列名与密钥引用格式返工 | 加密与解密封装在 ep-app-service 的一个模块内，列名与格式集中定义；已作为新增决定登记待回写基线 |
| R-09 | 附件上限未定（U-A-15），大量附件可能拖慢工单详情 | 详情时延 | 详情端点只返回附件元数据分页，不返回正文；正文经平台附件通道单独取用 |
| R-10 | 项目任务提交采购需求采用两段式，回写前存在提交中窗口 | 用户可能重复提交 | 端点幂等键加 (legal_entity_id, purchase_requisition_id) 唯一约束；提交中状态在界面明示，重复提交返回 409 |

#### 11.2 为后续阶段预留的扩展点

- 工单成本与工时：work_orders 与 work_order_lines 均不含金额列，后续开通时按在线变更规则新增可空 numeric(18,2) 列并新增成本归集事件，不需要改既有列类型。
- 服务 SLA 引擎：时限提醒现由 flow 定时器承担，提醒策略表已按法人与工单类型分行，后续引入 SLA 引擎时该表可整体迁移为 SLA 定义的输入，业务表不动。
- 项目任务与合同交付节点的引用（U-J-11）：预留方式为在线新增一个可空 uuid 列并加一条普通索引，本阶段不预先添加该列，理由是未决字段先落库会形成长期为空的列并进入全部投影。
- 项目任务的单层父子分组（U-J-14 后半）：同样以在线新增可空 parent_task_id 列实现，本阶段不添加。
- 工单重开（U-J-07）：状态机以数据驱动的迁移表实现，开通重开只需在迁移表中增加 COMPLETED → IN_PROGRESS 一行并加一条守卫，不改表结构，但需同步改工单统计口径。
- EAM 其余部分：设备档案的 current_status_code 已字典化且带 is_terminal 语义，后续点检与维修工单可直接引用该字典与设备主键。
- 客户 360 的第六类及以后区块：新增区块只需新增一个 SectionKind 取值与一个提供者实现，聚合层不改。枚举扩展按基线第 5.6 节，客户端必须容忍未知取值并按未知降级展示。

---

### 12. 未决事项的临时取值与切换代价

本阶段被 PRD 附录乙的 16 条 U-J 事项与 U-A、U-B、U-C 三组的 8 条触及。逐条给出是否阻塞、临时取值与切换代价。未列出的事项与本阶段无关。

| 编号 | 是否阻塞 | 临时取值 | 切换代价 |
|---|---|---|---|
| U-J-01 设备状态取值集合 | 不阻塞 | 字典出厂五行：IN_STOCK 待交付、IN_SERVICE 使用中、UNDER_REPAIR 维修中（以上 is_terminal=false），SCRAPPED 已报废、RETURNED 已退回（is_terminal=true） | 改字典行，经配置发布通道发布，无 DDL；已引用取值只停用不删除 |
| U-J-02 保修起始日期默认取值 | 不阻塞 | 不设默认，为空即判为无保修信息 | 若改为默认取交付日期，只改建档用例的一处赋值与三条测试 |
| U-J-03 序列号唯一性范围 | 不阻塞 | 不建唯一约束，重复时在 meta.warnings 提示 | 决策后一次 CREATE UNIQUE INDEX CONCURRENTLY 加一次存量去重；去重量由现有提示记录估算 |
| U-J-04 工单状态能否低代码扩展 | 不阻塞 | 状态集合固定，低代码只能在既有流转上加审批与时限 | 若允许扩展，状态需从 CHECK 改为字典表，属收紧类变更需停机窗口 |
| U-J-05 工单优先级与类型、投诉渠道取值 | 不阻塞 | 不设优先级字段；work_order_types 出厂四行 INSTALL、REPAIR、CONSULT、COMPLAINT_FOLLOWUP；complaint_channels 出厂四行 PHONE、EMAIL、ONSITE、SALES_RELAY | 增优先级需在线新增一个可空列与一条索引；字典取值改动无 DDL |
| U-J-06 工单时限阈值 | 不阻塞 | 出厂策略一行：待受理停留 480 分钟、期望完成提前 1440 分钟 | 改策略表行，经配置发布通道发布 |
| U-J-07 工单重开 | 不阻塞 | 不允许重开，终态只读 | 见 11.2 |
| U-J-08 换货配对规则 | 不阻塞 | 不强制配对：允许只挂退货侧或只挂发货侧，两侧都挂时按两者均到终态才置完成 | 若改为强制配对，加一条领域守卫与一条 CHECK，需回填校验存量行 |
| U-J-09 维修完成确认方与附件 | 不阻塞 | 由处理人确认，不强制附件 | 若改为强制附件，加一条守卫与一条测试 |
| U-J-10 登记行与退货单行基数 | 不阻塞 | 一对一 | 改为一对多需把两个引用列迁到关联表，属结构变更需停机窗口 |
| U-J-11 任务与交付节点引用 | 不阻塞 | 不建立引用 | 见 11.2 |
| U-J-12 派生任务负责人默认值 | 不阻塞 | 取派生项中的 owner_user_id，字段名按裁定 A-16 冻结，为空即留空 | 若改为取合同负责人或项目负责人，只改 4.7 第 4 步的一处赋值 |
| U-J-13 变更导致任务不再需要的处置 | 不阻塞 | 保留既有任务，不自动作废；派生计划中不再出现的 unique_key 对应的非终态任务保持原状并在项目详情中标注为来源已变更，合同升版后旧版本键必然不再出现，因而该分支是常态而非例外 | 若改为自动作废，加一个派生分支与一条事件，需补审计与通知 |
| U-J-14 项目存在未终态任务时能否完成或关闭 | 不阻塞 | 阻断，守卫 P1 | 若改为允许，去掉守卫 P1 与两条测试 |
| U-J-15 客户 360 区块条数与排序 | 不阻塞 | 每区块 20 条，按业务日期降序、对象 ID 降序 | 改默认配置值，无代码改动 |
| U-J-16 设备是否纳入首批历史导入 | 不阻塞 | 纳入，source=MIGRATION 与 migration_batch_no 已就位；迁移对账项按规格第 7.10 章为条数与关系一致，不涉及金额与数量 | 若不纳入，关闭迁移接口即可 |
| U-A-01 编号规则 | 不阻塞 | 按基线第 11.1 节，类型码 EQ、CPL、WO、PRJ、PT，五个码按裁定 C-26 登记在 docs/data-dictionary.md 的单据类型码一节，并与 ep-platform-sequence 的常量表逐项一致，由 xtask configdoc --check-doc-type-codes 校验唯一 | 类型码变更需同改常量、数据字典一行与一次存量数据说明 |
| U-A-03 文本长度 | 不阻塞 | 按基线第 11.2 节 | 放宽长度属在线变更，改 CHECK 即可 |
| U-A-05 列表默认值 | 不阻塞 | 按基线第 11.5 节 | 无 |
| U-A-11 提醒提前量 | 不阻塞 | 同 U-J-06 | 同上 |
| U-A-15 附件上限 | 阻塞一项校验 | 本阶段不在业务侧设附件数量上限，由平台附件能力统一判定；决策前工单附件不设条数校验 | 决策后在平台侧加校验，本阶段不改 |
| U-B-08 项目与客户维度授予粒度 | 不阻塞 | 本阶段只负责供给 data_scope_tags（project:<项目编号>、customer:<客户编码>），判定与叠加方式归权限模块 | 若标签形态变更，改一处标签生成函数 |
| U-C-04 客户 360 视图无定义节 | 不阻塞 | 技术落点已由裁定 C-09 定死，唯一端点 GET /api/v1/crm/customers/{id}/customer-360 与唯一契约 Customer360SectionProvider 由阶段 5 建立，本阶段在同一端点上追加区块，不新增路径，CustomerPanelProvider 作废；PRD 附录乙 U-C-04 在需求侧仍为待决，视图的承载节与区块清单由产品负责人决策，本阶段不代拍 | 落点变更无代价；区块清单若由产品另定，按增量注册增减区块实现，不改契约 |
| U-C-10 设备档案是否在交付确认时自动生成与生成粒度 | 不阻塞 | 不自动生成：本阶段不为 sales.delivery.confirmed.v1 注册消费者，第 6.3 节的消费者仍为三个，建档一律由 POST /api/v1/service/equipments/actions/create-from-delivery-batch 人工发起；粒度为逐台一行，每行台数由入参 lines 数组的 count 指定，不按交付确认单明细行汇总成一行，序列号不作为建档判据，见 U-J-03；单次上限取 EP__SERVICE__EQUIPMENT__CREATE_FROM_DELIVERY_MAX_ROWS 默认 200；重复建档由应用层按 delivery_confirmation_line_id 判定并跳过，取数走普通索引 ix_equipment_records_le_delivery_conf_line，不建唯一约束 | 改判为交付确认时自动生成：为 sales.delivery.confirmed.v1 新增一个本模块的 job-worker 消费者并按 platform_msg.inbox_consumptions 补幂等键，同时把 ix_equipment_records_le_delivery_conf_line 升为唯一索引以防重复投递产生重复建档；改判为按明细行粒度：改 service.equipment_records 的行粒度语义与建档入参，即去掉 lines 数组的 count 改为每行只生成一条，并改第 8.2 节第 1 与第 2 两条用例 |

---

### 13. 本阶段新增决定（需回写共享技术基线）

下列六条是基线未覆盖而本阶段必须取值的事项，按基线第 0 节要求显式标注为本阶段新增决定，并在阶段结束时回写基线对应章节。

1. 仅追加表清单扩充（回写基线第 4 节）：新增 service.work_order_logs 为仅追加表，不带 row_version、updated_at、updated_by，带 reverses_id，业务 schema 上禁止对其执行 UPDATE 与 DELETE，由 CI 的 SQL 静态检查断言。理由是 PRD 9.5.5 要求处理记录只追加不覆盖不删除。
2. 敏感明文列的命名与类型（回写基线第 4 节）：需要字段级信封加密的列一律命名为 `<语义>_enc`，类型 bytea，另配 `<语义>_key_ref text` 记录密钥标识与版本；该类列不得进入索引、唯一约束、过滤、排序、聚合与全文检索。若平台安全阶段另定同类约定，以其为准并整体替换。
3. 索引名的 63 字节收缩规则（回写基线第 3.10 节）：索引名超过 PostgreSQL 的 63 字节标识符上限时，按 `ux_<table>_<缩写列名序列>` 收缩，缩写规则为 legal_entity_id 缩为 le、其余列去掉 _id 后缀，收缩后的全名与原列清单在数据字典中登记。
4. 模块局部受控取值字典（回写基线第 3.2 节与第 7.1 节）：取值集合未决且需支持管理员维护的枚举，一律建模块局部字典表（档案类，带 code、name、sort_no、is_active），存事务数据库并经配置发布通道签名发布，不使用 CHECK 枚举，也不引用不存在的全局字典能力。引用列不建外键，取值合法性由应用层在写入前校验，字典行只允许停用不允许删除，不设周期性孤儿取值核对。
5. 非过账事件的标注（回写基线第 6.1 节）：不承载会计语义的领域事件在事件目录中标注为非过账事件，其信封的 posting_date 与 accounting_period_id 置空，且不计入规格第 10.2 章关账受理前提中的待消费过账条目数。本阶段 25 个事件全部属于该类。
6. 新增五个指标（回写基线第 9.2 节）：ep_service_work_orders_open（gauge，标签 legal_entity_id、status）、ep_service_work_order_open_lines（gauge，标签 legal_entity_id）、ep_crm_customer360_section_duration_seconds（histogram，标签 section）、ep_crm_customer360_section_degraded_total（counter，标签 section）、ep_project_contract_derivation_tasks_total（counter，标签 outcome 取 inserted、updated、skipped_terminal）。标签基数纪律照旧，不使用 user_id、doc_no、trace_id 作标签。

本阶段不偏离基线的任何既有取值，因此不设偏离项一节。

---

### 14. 假设清单

下列七条是规格与 PRD 未定义而实现必须知道的事项，显式标注为假设并给出理由，不静默假定。

- A-01 合同派生项的读取方式：该假设已由裁定 A-16 确认并冻结，clm 提供 ContractDerivationPlanQuery，本阶段在消费合同生效事件后读取派生计划，而不是从事件载荷中取任务清单。理由是基线第 6.1 节要求 payload 只放最小必要数据与引用 ID，把一份可能上百条的任务清单塞进事件载荷会使 Outbox 行体积不可控；PRD 9.7.1 的“只接收派生结果”指本阶段不解释合同条款，与读取方式无关。
- A-02 项目与合同的对应关系：该假设已由裁定 A-16 确认，ContractDerivationPlan 携带 project_group_contract_id 即合同续签链的根合同标识，本阶段按该标识定位项目，该字段可空时退回取 contract_id，使续签派生的任务与原任务落在同一项目。理由是 PRD 9.7.6 明确要求新派生任务与原任务同属一个项目，而续签会产生新的合同标识，只按 contract_id 定位无法满足该要求。
- A-03 客户 360 的实现形态：假设采用区块提供者扇出的实时查询而非物化投影。理由是首版不使用物化视图（基线第 3.2 节），且规格第 7.9 章对派生存储有 15 分钟传播窗口与安全继承要求，实时查询可避免引入第三处需要传播与越权测试的数据副本。
- A-04 客户 360 的性能度量归属：假设按附录 A.1 末段“后续新增场景只增加度量项，不改变通过线”的规则，新增一个常规交互度量项，通过线沿用规格第 16 章常规交互 P95 2 秒。理由是附录 A.1 未单列该场景，而规格第 8 章第 12 步把它列为闭环验收内容，必须有可判定的时延口径。
- A-05 设备档案的停用路径：假设首版不提供设备档案停用入口，is_active 恒为 true，deactivated_at 恒为空，两列按基线第 4 节档案类要求保留以便后续开通。理由是 PRD 9.3 只定义了设备当前状态（含终止状态），未定义档案层面的停用，两套语义并存会产生第三态并使工单可选设备的判据出现歧义。
- A-06 项目任务提交采购需求的两段式：假设采用“本事务只发事件、由 job-worker 调用 ep_contract_procure::PurchaseRequisitionIntakePort::intake 后回写引用”的两段式，而非在同一事务内同步调用。端口名按裁定 C-17 已冻结，两段式本身仍是本阶段的假设。理由是基线第 10.3 节禁止事务内的跨模块写编排且要求一个用例一个事务，规格第 8 章事务边界要求跨领域流程使用 Outbox 与持久化工作流。
- A-07 处理记录不设行号：假设处理记录按 created_at 与 UUIDv7 的 id 给出稳定全序，不设独立行号列。理由是行号需要一个额外的串行化点，而处理记录是高频追加对象，串行化点会与工单行锁叠加形成不必要的争用。
