## 阶段 6：合同与销售（CLM、销售与 OMS、CPQ 价格权限、客户信用额度校验）

本阶段承载规格第 5.2 章 CLM、销售与 OMS、客户信用额度校验、CPQ 价格权限四个条目的原生能力，以及规格第 8 章黄金业务闭环第 1 步、第 2 步、第 3 步、第 8 步与第 11 步销售侧的单据主体，对应 PRD 第 3 节全节，其中第 8 步的交付确认在 PRD 第 3 节与第 5 节均无承载小节，属 PRD 附录乙 U-C-01，见第 10.2 节与第 11.3 小节。其中第 8 步的交付确认单按裁定 A-09 归本阶段建表、建用例、发事件，是该步的唯一落点；库存腿由阶段 8 提供端口，收入与成本腿由阶段 9a 提供端口，过渡科目腿由阶段 10 提供端口，该腿与交付确认的过账路径按第 11.5 小节与阶段 10 同批接线，本阶段不注入任何空实现。本阶段属规格第 19 章阶段 3 的建设内容，其时延与容量通过线在阶段 4 统一判定。

全文取值一律遵循共享技术基线。基线已定死的事项本节直接引用，不重新决定；基线未覆盖而本阶段必须取值的，在第 11.3 小节集中列出并标注为本阶段新增决定；与基线有出入的，在第 11.2 小节单列偏离项。

本阶段的工作次序按 T0 贯通线重排，阶段范围归属不变。阶段顺序固定为 1 → 2 → 3a → 4 → 3b-1 → T0 → 5 → 9a → 8 → 6 → 7 → 10 → 11 → 9b → 14，T0 是插在阶段 3b-1 与阶段 5 之间的一条最薄贯通线，不新增任何范围，判据是一条合同从建单走到管理层看到一个数。本阶段在 T0 中贡献的最小切片只有两项，一份单审批节点的合同与一张由该合同生效派生出的销售订单。落到确切标识符是 `clm.contract_types`、`clm.contracts`、`clm.contract_lines`、`clm.contract_approvals`、`sales.credit_policies`、`sales.sales_orders`、`sales.sales_order_lines` 七张表，`create_contract`、`submit_for_approval`、`make_effective` 三个 clm 用例与 `ep_contract_sales::SalesOrderDerivationPort` 的派生实现，端点 `POST /api/v1/clm/contracts`、`PUT /api/v1/clm/contracts/{id}/lines`、`POST /api/v1/clm/contracts/{id}/actions/submit-for-approval`、`POST /api/v1/clm/contracts/{id}/actions/make-effective` 与 `GET /api/v1/sales/sales-orders/{id}`，事件 `clm.contract.effective.v1`。T0 内只启用 `clm.contract_approvals` 的 `chain_kind = EFFECTIVE` 一条链，该节点即规格第 8 章第 2 步要求的管理层必经节点；`sales.credit_policies` 只建一行且 `null_limit_behavior` 取 `SKIP_CHECK`，信用三桶不进 T0 判据；合同生效的重新认证按规格第 12.1 章在 T0 内即成立，不推迟；T0 内合同行的默认税率经 `ep_contract_invoice::TaxRateOptionQuery::default_rate` 取得，`invoice.tax_rate_options` 的建表迁移与种子迁移两条及该查询的 `default_rate` 与 `list` 两个方法属阶段 10 的 T0 切片第五项，与本阶段的两项切片在 T0 期间一并交付。T0 用 `ep-datagen` 最小样本，不要求 scale 数据集、不要求分支覆盖、只要求桌面端，其判据由 T0 自身判定，不重复计入第 9 节退出条件。

T0 通过后本阶段其余部分一律在这条已贯通的骨架上加厚，分三批施工。第一批是合同侧加厚，含模板与条款库、四条审批链与折扣审批、电子签章与实体印章、版本与修订、续签、合并、提前终止、五项校验与价格权限、三类到期提醒触发源。第二批是订单侧加厚，含分批交付行的拆分与合并、订单变更与版本、订阅与租赁、销售退货与换货、在途桶与 `sales.v_credit_exposure_in_transit`、四个受治理数据集视图、四端界面。第三批是交付确认段，按第 11.5 小节与阶段 10 的 finance 端口同批施工，含 `sales.delivery_confirmations` 与 `sales.delivery_confirmation_lines` 的过账路径、信用三桶中两桶的接线与销售退货的红冲前置判定。三批之外本阶段不再有其他工作次序上的约束，M7 判定的是全分支闭环而不是首次贯通。

---

### 1. 交付物清单

本阶段结束时下列可运行物存在，且可由 `cargo test --workspace` 与 `apps/core-server --check` 验证。

1. 六个新增库 crate 编译通过并被 apps 装配：`ep-contract-clm`、`ep-domain-clm`、`ep-app-clm`、`ep-contract-sales`、`ep-domain-sales`、`ep-app-sales`，以及 `ep-contract-cpq`、`ep-domain-cpq`、`ep-app-cpq` 中与价格权限校验相关的部分。
2. 一个新增适配 crate `ep-adapter-esign` 编译通过，目录为 `crates/adapter/esign/`，并在 integration-gateway 中装配为唯一的对外出网出口；其两套契约测试文件 `crates/adapter/esign/tests/contract_sandbox.rs` 与 `crates/adapter/esign/tests/contract_stub.rs` 存在且共用同一组断言函数。
3. `db/migrations/cpq/`、`db/migrations/clm/`、`db/migrations/sales/` 三个迁移目录下的全部迁移可在空库上离线执行成功，并可按各文件头 `-- rollback:` 段落回退到本阶段之前的版本；其中含按裁定 A-09 新建的 `sales.delivery_confirmations` 与 `sales.delivery_confirmation_lines` 两张表，以及按裁定 A-18 新建的 `clm.v_contracts_dataset`、`clm.v_contract_delivery_milestones`、`sales.v_sales_orders_dataset`、`sales.v_order_delivery_batches` 四个受治理数据集视图。按裁定通则第五条，本阶段不新增任何跨 schema 迁移。
4. core-server 暴露第 5 节列出的全部 HTTP 端点，`/api/v1/clm/*`、`/api/v1/sales/*`、`/api/v1/cpq/price-authorities`，四端可调用；其中 `POST /api/v1/sales/delivery-confirmations/{id}/actions/confirm-delivery` 一条按第 11.5 小节随第三批与阶段 10 同批注册，在此之前不注册，也不返回占位结果。
5. job-worker 中运行三类消费者，名字固定为 `clm.derivation`、`clm.milestone_confirm`、`sales.delivery_writeback`，其中 `sales.delivery_writeback` 与 `clm.milestone_confirm` 消费本模块发出的 `sales.delivery.confirmed.v1` 并随第三批交付；三者的死信可在运维中心枚举。
6. integration-gateway 中运行电子签章出口，含超时、退避、熔断与证据固化，签署状态由本进程按退避轮询拉取。
7. `docs/event-catalog.md` 中登记本阶段的 18 个领域事件，清单与计数口径见第 6.3 小节的事件登记表，其中含 `sales.delivery.confirmed.v1` 与销售退货的登记、关闭、取消、驳回四个事件；本阶段第 5 节 API 契约表中出现的全部错误码已登记在 `docs/error-codes.md` 并与 `ep-foundation::error::codes` 一致，由 CI 校验。
8. `ep-testkit` 中新增 `ContractBuilder`、`SalesOrderBuilder`、`DeliveryScheduleBuilder`、`CreditFixture` 四个构造器；`ep-datagen` 在默认 scale 下生成合同与销售订单行各 10 万条并满足本阶段的全部不变量。
9. 一个可重复执行的端到端用例集 `apps/core-server/tests/e2e_stage6/`，覆盖第 8 节列出的 13 个 E2E 场景。
10. 一份 `docs/adr/2026-11-clm-esign-polling.md`，记录电子签章不设公网入站回调而采用轮询的决定与理由。
11. 四端界面：`clients/desktop/src/modules/clm/`、`clients/desktop/src/modules/sales/`、`clients/mobile/src/modules/clm/`、`clients/mobile/src/modules/sales/` 四个目录存在并可构建，按规格第 6.2 章能力矩阵的取值实现。

---

### 2. crate 与进程归属

#### 2.1 新增与改动的 crate

| crate | 层 | 新增或改动 | 主要内容 |
|---|---|---|---|
| ep-contract-clm | 契约 | 新增 | 合同命令与查询 DTO、合同事件类型、供其他模块调用的 `ContractQueryPort`、`ContractMilestonePort`、`ContractDerivationCallbackPort`、`ContractDerivationPlanQuery`、`ContractPaymentScheduleQuery`；`src/capability.rs` 中为每个用例声明 `<USECASE_SCREAMING>_DOMAIN` 与 `<USECASE_SCREAMING>_ACTION` 一对常量 |
| ep-domain-clm | 领域 | 新增 | 合同聚合、合同版本、关键条款、交付节点、收付款期次、签署编排、派生批次；合同状态机与守卫；`ContractRepository`、`SignatureGateway`、`TemplateRenderer` 三个端口 |
| ep-app-clm | 应用 | 新增 | 20 个用例、合同侧授权入口、事务边界、派生编排、审计与 Outbox 写入；`ClmProductUsageProbe` 与 `ClmReferenceCounter` 两个探针实现 |
| ep-contract-sales | 契约 | 新增 | 销售订单、交付确认与退货的命令查询 DTO、事件类型、`SalesOrderDerivationPort`、`CreditExposureQueryPort`、`SalesOrderQueryPort`、`SalesReturnCommandPort`；`src/capability.rs` 中为每个用例声明 `<USECASE_SCREAMING>_DOMAIN` 与 `<USECASE_SCREAMING>_ACTION` 一对常量 |
| ep-domain-sales | 领域 | 新增 | 销售订单聚合、订单行、分批交付行、订单变更版本、交付确认聚合、销售退货聚合、换货关联；四套状态机；信用占用在途桶的纯计算 |
| ep-app-sales | 应用 | 新增 | 20 个用例（含 `create_delivery_confirmation` 与 `confirm_delivery` 两个）、信用校验编排、交付确认三腿编排与回写消费、订单变更审批回写；`SalesProductUsageProbe`、`SalesReferenceCounter`、`SalesTradeHistoryProviderImpl` 三个探针实现 |
| ep-contract-cpq | 契约 | 改动 | 追加 `PriceAuthorityPort` 与价格权限判定 DTO；价目表查询 trait 由主数据阶段定义，本阶段只消费；在阶段 5 已建的 `src/capability.rs` 中只追加价格权限档案路由的一对常量，不重定义能力域码 |
| ep-domain-cpq | 领域 | 改动 | 追加价格权限值对象与判定规则、行金额与净单价的计算规则 |
| ep-app-cpq | 应用 | 改动 | 追加价格权限档案的维护用例与判定用例 |
| ep-adapter-esign | 适配 | 新增 | 电子签章外部出口的 HTTP 客户端、请求签名、响应验签、证据固化、熔断与退避 |
| ep-testkit | 测试 | 改动 | 追加本阶段四个构造器与信用夹具 |
| ep-datagen | 测试 | 改动 | 追加合同、订单、分批交付行、退货单的生成器 |

除按 A-23 在 `clients/desktop/src/modules/` 与 `clients/mobile/src/modules/` 下各新增 `clm` 与 `sales` 两个模块目录外，不新增任何 crate 之外的目录结构，crate 内目录严格按基线第 10.1 节。`ep-domain-clm` 与 `ep-domain-sales` 中不得出现 sqlx、reqwest、`std::fs`、`std::net`、`SystemTime::now`、`rand` 符号，由基线第 8.4 节的静态检查强制。

#### 2.2 依赖方向

- `ep-app-clm` 依赖 `ep-foundation`、`ep-platform-*`、`ep-domain-clm`、`ep-contract-clm`，以及 `ep-contract-sales`、`ep-contract-procure`、`ep-contract-finance`、`ep-contract-mdm`、`ep-contract-cpq`、`ep-contract-inventory`、`ep-contract-invoice` 七个外部模块契约。`ep-contract-project` 按 C-19 移除，项目任务不再由本模块同步派生；`ep-contract-invoice` 按 C-11 引入，合同行的默认税率经 `TaxRateOptionQuery` 取得。
- `ep-app-sales` 依赖 `ep-foundation`、`ep-platform-*`、`ep-domain-sales`、`ep-contract-sales`，以及 `ep-contract-clm`、`ep-contract-mdm`、`ep-contract-cpq`、`ep-contract-inventory`、`ep-contract-ledger`、`ep-contract-finance`、`ep-contract-invoice`、`ep-contract-procure` 八个外部模块契约，其中 `ep-contract-inventory` 与 `ep-contract-ledger` 供交付确认的库存腿与凭证腿调用，`ep-contract-procure` 供直运退货的勾稽调用。
- `ep-app-clm` 与 `ep-app-sales` 之间不存在直接依赖。合同派生销售订单一律经 `ep-contract-sales::SalesOrderDerivationPort`，其实现是 `ep-app-sales` 的用例，在 apps 的 `wiring/` 目录中注入。
- `ep-adapter-esign` 只依赖 `ep-foundation` 与 `ep-domain-clm::port::SignatureGateway`，不依赖任何 application。

#### 2.3 进程归属

| 能力 | 进程 | 说明 |
|---|---|---|
| 合同与订单的全部命令与查询 API | core-server | 含四端与合同侧受控查询 |
| 交付确认单的登记与确认过账 | core-server | 确认动作在单个事务内依次调用库存腿、过渡科目腿与凭证腿三个契约端口，三腿一次全真接线，按第 11.5 小节随第三批与阶段 10 同批交付 |
| 合同附件正文的读写 | core-server | 交易路径上的附件正文按基线第 2 节归 core-server |
| 合同生效派生编排与执行 | job-worker | 消费 `clm.contract.effective.v1`，按派生项逐项执行 |
| 交付确认回写与合同履约进度推进 | job-worker | `sales.delivery_writeback` 消费本模块发出的 `sales.delivery.confirmed.v1`，更新分批交付行、订单行与订单状态；`clm.milestone_confirm` 消费同一事件，更新 `clm.contract_milestones` 的交付节点；两个消费者各只写本模块 schema，按基线第 1.3 节不跨模块写入 |
| 合同到期提醒的定时触发 | job-worker | 使用 ep-platform-flow 的定时器与 ep-platform-notify 的站内通知，本阶段只提供触发源投影 |
| 电子签章的发起、状态轮询、结果拉取与验签 | integration-gateway | 首版唯一的对外出网出口 |
| 合同模板渲染与 PDF 归档 | job-worker | 经 `ep_foundation::port::doc::DocTemplatePort::render` 与 `PdfRenderPort::render_pdf`，不新增接口，同步等待超过 8 秒的一律转后台任务 |

不新增进程，不改动任何进程的监听端口、系统账户与 cgroup 归属。

---

### 3. 数据库变更

#### 3.1 通用约定

下列约定对本阶段全部新建表成立，逐表不再重复。

- 每张表包含基线第 4 节的九个公共列：`id uuid`、`legal_entity_id uuid`、`security_level smallint default 20`、`data_scope_tags text[] default '{}'`、`row_version bigint default 1`、`created_at timestamptz default now()`、`created_by uuid`、`updated_at timestamptz default now()`、`updated_by uuid`。标注为仅追加的表不带 `row_version`、`updated_at`、`updated_by`，改带 `reverses_id uuid null`。
- 每张表按基线第 3.8 节的统一模板启用并强制行级安全，策略名 `rls_<table>_le`，判据只有 `app.legal_entity_id`。本阶段不新增任何不带 `legal_entity_id` 的表。
- 每张表的基线索引固定为 `pk_<table>` 与 `ix_<table>_legal_entity_id_created_at`；单据类表另加 `ux_<table>_legal_entity_id_doc_no`。下表只列基线索引之外的追加索引。
- 枚举列一律 `text` 加 CHECK，取值大写 snake_case；金额 `numeric(18,2)`、单价与数量 `numeric(18,6)`、比例与税率 `numeric(9,6)`。
- 同 schema 内引用建真实外键并 `ON DELETE RESTRICT`；跨 schema 引用只留逻辑引用列，不建外键，其存在性由 application 层经对方模块契约在写入时校验。按裁定 A-06，本阶段的跨模块逻辑引用不进入 `ReconCheck` 注册清单，不建周期性对账校验项，这是首版的已知边界。
- 本阶段全部为新建空表，索引随建表在同一迁移文件内用普通 `CREATE INDEX` 建立；只有后续对存量表追加索引才用 `CREATE INDEX CONCURRENTLY`。
- 文本列长度按基线第 11.2 节：编码 64、名称 200、简述 500、备注与原因与说明 2000、条款正文 1 MB，一律 `text` 加 CHECK。

#### 3.2 cpq schema 的变更

迁移 `V202611020900__cpq_create_price_authorities.sql`。

表 `cpq.price_authorities`，档案类。

| 列 | 类型 | 可空 | 说明 |
|---|---|---|---|
| code | text | 否 | 档案编码，法人内唯一 |
| name | text | 否 | 名称 |
| subject_kind | text | 否 | CHECK in ROLE, POSITION, USER |
| subject_id | uuid | 否 | 逻辑引用 platform 侧角色、岗位或用户 |
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
| V202611021000 | clm.contract_types |
| V202611021005 | clm.contract_templates、clm.contract_template_versions |
| V202611021010 | clm.clauses、clm.clause_versions |
| V202611021020 | clm.contracts |
| V202611021025 | clm.contract_lines |
| V202611021030 | clm.contract_terms |
| V202611021035 | clm.contract_milestones |
| V202611021040 | clm.contract_obligations |
| V202611021045 | clm.contract_payment_schedules |
| V202611021050 | clm.contract_attachments |
| V202611021055 | clm.contract_annotations |
| V202611021060 | clm.contract_versions |
| V202611021065 | clm.contract_approvals |
| V202611021070 | clm.signature_requests |
| V202611021075 | clm.signature_events |
| V202611021080 | clm.seal_usages |
| V202611021085 | clm.contract_derivations、clm.contract_derivation_items |
| V202611021090 | clm.contract_validations、clm.contract_validation_items |
| V202611021095 | clm.contract_merge_links |
| V202611021100 | clm.v_contract_milestone_progress、clm.v_contract_reminder_sources |
| V202611021105 | clm.v_contracts_dataset、clm.v_contract_delivery_milestones |

表 `clm.contract_types`，档案类。列为 `code text`、`name text`、`requires_project boolean default false`、`requires_procurement_default boolean default false`、`approval_chain_terms_code text`、`approval_chain_discount_code text`、`approval_chain_payment_code text`、`approval_chain_attachment_code text`、`default_template_id uuid`、`is_active boolean`、`deactivated_at timestamptz`。约束 `ux_contract_types_legal_entity_id_code`。四个审批链编码是逻辑引用 platform_flow 的流程定义键，不建外键。

表 `clm.contract_templates`，档案类。列为 `code text`、`name text`、`contract_type_id uuid not null`（同 schema 外键）、`current_version_no int not null default 0`、`is_active boolean`、`deactivated_at timestamptz`。约束 `ux_contract_templates_legal_entity_id_code`。

表 `clm.contract_template_versions`，仅追加。列为 `contract_template_id uuid not null`、`version_no int not null`、`body_attachment_object_id uuid`、`default_terms jsonb not null default '{}'`、`clause_refs jsonb not null default '[]'`、`published_at timestamptz`、`release_package_id uuid`、`status text CHECK in DRAFT, PUBLISHED, RETIRED`。约束 `ux_contract_template_versions_template_version` 在 `(contract_template_id, version_no)`。发布经 ep-platform-release 的配置发布通道，`release_package_id` 是其逻辑引用。

表 `clm.clauses` 与 `clm.clause_versions`，结构与模板同构，`clauses` 另有 `category text`，`clause_versions` 有 `body text` 且 CHECK 长度不超过 1 MB。

表 `clm.contracts`，单据类。

| 列 | 类型 | 可空 | 说明 |
|---|---|---|---|
| doc_no | text | 否 | 由 ep-platform-sequence 生成，类型码 CT |
| status | text | 否 | CHECK in DRAFT, PENDING_APPROVAL, REJECTED, PENDING_SIGNATURE, EFFECTIVE, IN_PERFORMANCE, COMPLETED, TERMINATED, VOID |
| contract_type_id | uuid | 否 | 同 schema 外键 |
| customer_id | uuid | 否 | 逻辑引用 mdm 客户档案 |
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

表 `clm.contract_lines`。列除公共列外为 `contract_id uuid not null`（外键）、`line_no int not null`、`item_kind text CHECK in PRODUCT, MATERIAL`、`item_id uuid not null`、`uom_code text not null`、`quantity numeric(18,6) not null`、`list_unit_price numeric(18,6)`、`price_floor numeric(18,6)`、`unit_price numeric(18,6) not null`、`discount_rate numeric(9,6) not null default 0`、`net_unit_price numeric(18,6) not null`、`is_tax_included boolean not null default false`、`tax_rate numeric(9,6) not null default 0`、`line_amount numeric(18,2) not null`、`line_amount_with_tax numeric(18,2) not null`、`delivery_date date not null`、`warehouse_id uuid`、`order_type text not null CHECK in NORMAL, DROP_SHIP, CONSIGNMENT, SUBSCRIPTION, LEASE`、`cycle_unit text`、`cycle_length int`、`lease_from date`、`lease_to date`、`auto_renew boolean not null default false`、`requires_procurement boolean not null default false`、`requires_discount_approval boolean not null default false`、`price_list_id uuid`、`price_list_line_id uuid`、`source_contract_line_id uuid`。

约束与索引：`ux_contract_lines_contract_id_line_no`；`ck_contract_lines_quantity_positive`；`ck_contract_lines_discount_range`；`ck_contract_lines_warehouse_required` 约束 `order_type = 'DROP_SHIP' or warehouse_id is not null`；`ck_contract_lines_cycle_required` 约束 `order_type not in ('SUBSCRIPTION','LEASE') or (cycle_unit is not null and cycle_length is not null)`；追加索引 `ix_contract_lines_contract_id`、`ix_contract_lines_legal_entity_id_item_id`。

表 `clm.contract_terms`，一合同一行。列为 `contract_id uuid not null unique`、`body text`、`warranty_clause text`、`liability_clause text`、`dispute_resolution text`、`structured jsonb not null default '{}'`、`clause_refs jsonb not null default '[]'`。约束 `ux_contract_terms_contract_id`。

表 `clm.contract_milestones`。列为 `contract_id uuid not null`、`milestone_no int not null`、`name text not null`、`promised_date date not null`、`status text not null CHECK in PLANNED, ACTIVE, CONFIRMED, CANCELLED`、`confirmed_date date`、`delivery_confirmation_id uuid`（跨 schema 逻辑引用 `sales.delivery_confirmations`，按裁定 A-09 保持逻辑引用，不建外键）、`owner_user_id uuid`。约束 `ux_contract_milestones_contract_id_milestone_no`；追加索引 `ix_contract_milestones_legal_entity_id_promised_date_status`（到期提醒与交付指标取数）。该表不带产品、物料与订单字段，与规格第 5.5 章经营驾驶舱条目的口径一致。

表 `clm.contract_obligations`。列为 `contract_id uuid not null`、`seq_no int not null`、`name text not null`、`description text`、`due_date date`、`status text CHECK in OPEN, FULFILLED, WAIVED`。

表 `clm.contract_payment_schedules`。列为 `contract_id uuid not null`、`period_no int not null`、`condition_text text`、`basis text not null CHECK in RATIO, AMOUNT`、`ratio numeric(9,6)`、`amount numeric(18,2)`、`amount_with_tax numeric(18,2)`、`due_date date not null`、`remark text`。约束 `ux_contract_payment_schedules_contract_id_period_no`；`ck_contract_payment_schedules_basis` 约束 `(basis='RATIO' and ratio is not null and amount is null) or (basis='AMOUNT' and amount is not null and ratio is null)`；追加索引 `ix_contract_payment_schedules_legal_entity_id_due_date`。同一合同内 basis 不混用，由应用层校验并写入 `clm.contract_validations`。按裁定 C-20，本表是收付款计划行的唯一出处，`ep_contract_finance::ReceivablePlanPort` 已撤销，finance 不再派生第二套；阶段 10 的到款自动核销经本阶段提供的 `ep_contract_clm::ContractPaymentScheduleQuery::schedules(tx, ctx, contract_id)` 取数。

表 `clm.contract_attachments`，按基线第 4 节的附件关联表规范。列为 `owner_id uuid not null`（指向 contracts.id）、`attachment_object_id uuid not null`、`purpose text not null CHECK in CONTRACT_BODY, SIGNED_FILE, SEAL_SCAN, SUPPORTING`、`sort_no int not null default 0`、`contract_version_no int not null`。追加索引 `ix_contract_attachments_owner_id_purpose`。

表 `clm.contract_annotations`。列为 `contract_id uuid not null`、`attachment_object_id uuid not null`、`attachment_version_no int not null`、`page_no int`、`anchor jsonb not null default '{}'`、`body text not null`、`state text not null CHECK in OPEN, RESOLVED`、`resolved_by uuid`、`resolved_at timestamptz`。追加索引 `ix_contract_annotations_contract_id_state`。

表 `clm.contract_versions`，仅追加。列为 `contract_id uuid not null`、`version_no int not null`、`snapshot jsonb not null`、`change_reason text`、`created_at`、`created_by`、`reverses_id uuid null`。约束 `ux_contract_versions_contract_id_version_no`。快照内容为合同头、行、条款、交付节点、收付款期次与附件引用的完整取值，用于版本比较与追溯。

表 `clm.contract_approvals`，仅追加。列为 `contract_id uuid not null`、`contract_version_no int not null`、`chain_kind text not null CHECK in TERMS, DISCOUNT, PAYMENT, ATTACHMENT, CREDIT, EFFECTIVE`、`flow_instance_id uuid not null`、`outcome text CHECK in APPROVED, REJECTED, RETURNED, WITHDRAWN`、`concluded_at timestamptz`、`approver_user_id uuid`、`comment text`、`reauth_ref uuid`。追加索引 `ix_contract_approvals_contract_id_chain_kind`、`ix_contract_approvals_flow_instance_id`。

表 `clm.signature_requests`。列为 `contract_id uuid not null`、`contract_version_no int not null`、`provider_code text not null`、`external_request_id text`、`status text not null CHECK in PENDING, SUBMITTED, SIGNING, SIGNED, REJECTED, FAILED, CANCELLED`、`submitted_at timestamptz`、`concluded_at timestamptz`、`attempts int not null default 0`、`next_poll_at timestamptz`、`last_error text`、`signed_attachment_object_id uuid`、`verify_result text CHECK in NOT_VERIFIED, PASSED, FAILED`、`evidence_hash bytea`。约束 `ux_signature_requests_contract_id_version` 在 `(contract_id, contract_version_no)`；追加索引 `ix_signature_requests_status_next_poll_at`。

表 `clm.signature_events`，仅追加。列为 `signature_request_id uuid not null`、`occurred_at timestamptz not null`、`kind text not null CHECK in SUBMITTED, POLLED, SIGNED, REJECTED, FAILED, VERIFIED`、`external_status text`、`payload_digest bytea`、`evidence_attachment_object_id uuid`、`reverses_id uuid null`。追加索引 `ix_signature_events_signature_request_id_occurred_at`。外部返回的原始报文不落列，只固化摘要与证据附件，避免明文进入业务表与日志。

表 `clm.seal_usages`。列为 `contract_id uuid not null`、`contract_version_no int not null`、`seal_name text not null`、`used_at timestamptz not null`、`operator_user_id uuid not null`、`scan_attachment_object_id uuid not null`、`remark text`。追加索引 `ix_seal_usages_contract_id`。

表 `clm.contract_derivations`。列为 `contract_id uuid not null`、`contract_version_no int not null`、`trigger text not null CHECK in EFFECTIVE, AMENDMENT, RENEWAL`、`status text not null CHECK in RUNNING, DONE, FAILED`、`flow_instance_id uuid`、`started_at timestamptz not null`、`finished_at timestamptz`、`item_total int not null default 0`、`item_done int not null default 0`。约束 `ux_contract_derivations_contract_id_version_trigger` 在 `(contract_id, contract_version_no, trigger)`，这是派生幂等的第一道保证。

表 `clm.contract_derivation_items`。列为 `contract_derivation_id uuid not null`（外键）、`artifact_kind text not null CHECK in SALES_ORDER, PURCHASE_REQUISITION, PROJECT_TASK, RECEIVABLE_PLAN, MILESTONE`、`source_ref_id uuid`（合同行或期次或交付节点的 id，整单粒度的为空）、`target_module text not null`、`target_doc_id uuid`、`target_doc_no text`、`status text not null CHECK in PENDING, DISPATCHING, DONE, DEAD`、`attempts int not null default 0`、`available_at timestamptz not null default now()`、`last_error text`、`idempotency_key uuid not null`。约束 `ux_contract_derivation_items_unique` 在 `(contract_derivation_id, artifact_kind, coalesce(source_ref_id, contract_derivation_id))`，这是派生幂等的第二道保证；追加索引 `ix_contract_derivation_items_status_available_at`。

表 `clm.contract_validations`。列为 `contract_id uuid not null`、`contract_version_no int not null`、`occasion text not null CHECK in SUBMIT, DERIVE, MERGE_RESUBMIT, RENEW_SUBMIT`、`verdict text not null CHECK in PASSED, BLOCKED, REVIEW_REQUIRED`、`evaluated_at timestamptz not null`、`evaluated_by uuid not null`、`audit_event_id uuid`。追加索引 `ix_contract_validations_contract_id_occasion`。

表 `clm.contract_validation_items`。列为 `contract_validation_id uuid not null`（外键）、`check_kind text not null CHECK in PRICE_AUTHORITY, CONTRACT_INTEGRITY, STOCK_AVAILABILITY, LEAD_TIME, CREDIT_LIMIT`、`result text not null CHECK in PASSED, FAILED, FLAGGED`、`source_line_id uuid`、`snapshot jsonb not null default '{}'`、`message_code text`。快照内容为该项取数的输入与输出，例如信用项记录信用额度、三部分占用取值、本次待增加占用与判定结论，直接对应 PRD 3.14.3 的取数快照要求。

表 `clm.contract_merge_links`，按基线的多对多命名。列为 `source_contract_id uuid not null`、`target_contract_id uuid not null`、`merged_at timestamptz not null`、`merged_by uuid not null`。约束 `ux_contract_merge_links_source_target`。

视图 `clm.v_contract_milestone_progress`：按合同聚合交付节点的计划数、已确认数、逾期数与最近到期日，只读取本 schema 的表。

视图 `clm.v_contract_reminder_sources`：把合同有效期止日、交付节点约定日期、收付款期次到期日三类触发源统一为 `(legal_entity_id, contract_id, source_kind, due_date, owner_user_id)` 五列，供 ep-platform-flow 的定时器取数。三类触发源与 PRD 3.9.2 的三行一一对应。
按裁定 A-18，第 V202611021105 号迁移 `V202611021105__clm_create_dataset_views.sql` 建立本模块的两个受治理数据集视图。`clm.v_contracts_dataset` 的 dataset code 为 `clm_contracts`，grain 取 DOCUMENT，取数为 `clm.contracts`；`clm.v_contract_delivery_milestones` 的 dataset code 为 `clm_contract_delivery_milestones`，grain 取 DOCUMENT_LINE，取数为 `clm.contract_milestones`，输出列含 `contract_id`、`milestone_no`、`name`、`promised_date`、`status`、`confirmed_date`、`delivery_confirmation_id`、`owner_user_id`。两个视图都必须含 `legal_entity_id`、`security_level`、`data_scope_tags` 三列，都不做聚合、不跨 schema 连接，并在同一迁移内执行 `GRANT SELECT ON <视图> TO ep_analyst_ro`，不授予 `ep_app_rw` 之外的任何写权限。列名与类型签名必须与阶段 11 的 `reporting.dataset_fields` 登记一致，由阶段 11 的启动自检项 `reporting-dataset-signature-matched` 校验，本阶段在第 9 节退出条件中把列签名同步给阶段 11。该文件头的 `-- rollback:` 段为 `drop view` 与对应的 `revoke`。

#### 3.4 sales schema 的变更

| 迁移编号 | 建立对象 |
|---|---|
| V202611031000 | sales.credit_policies |
| V202611031005 | sales.customer_credit_controls |
| V202611031010 | sales.sales_orders |
| V202611031015 | sales.sales_order_lines |
| V202611031020 | sales.delivery_schedules |
| V202611031021 | sales.delivery_confirmations |
| V202611031022 | sales.delivery_confirmation_lines |
| V202611031025 | sales.sales_order_versions |
| V202611031030 | sales.sales_order_changes、sales.sales_order_change_lines |
| V202611031040 | sales.sales_returns、sales.sales_return_lines |
| V202611031050 | sales.return_line_delivery_links |
| V202611031055 | sales.exchange_links |
| V202611031060 | sales.order_validations、sales.order_validation_items |
| V202611031070 | sales.v_credit_exposure_in_transit |
| V202611031075 | sales.v_sales_orders_dataset、sales.v_order_delivery_batches |

表 `sales.credit_policies`，法人级策略，经配置发布通道维护。列为 `scope text not null CHECK in LEGAL_ENTITY`、`on_exceed text not null CHECK in BLOCK, REVIEW`、`null_limit_behavior text not null CHECK in TREAT_AS_ZERO, TREAT_AS_UNLIMITED, SKIP_CHECK`、`amount_basis text not null CHECK in WITH_TAX, WITHOUT_TAX`、`deduct_advance_receipts boolean not null default false`、`recheck_on_order_change boolean not null default true`、`release_package_id uuid`。约束 `ux_credit_policies_legal_entity_id_scope`，每法人一行。

表 `sales.customer_credit_controls`，每法人每客户一行，同时是信用校验的串行化点。列为 `customer_id uuid not null`、`on_exceed_override text CHECK in BLOCK, REVIEW`、`last_checked_at timestamptz`、`last_exposure jsonb not null default '{}'`。约束 `ux_customer_credit_controls_legal_entity_id_customer_id`。该行在信用校验事务内以 `SELECT ... FOR UPDATE` 取用，不存在时以 `INSERT ... ON CONFLICT DO NOTHING` 建立。

表 `sales.sales_orders`，单据类。

| 列 | 类型 | 可空 | 说明 |
|---|---|---|---|
| doc_no | text | 否 | 类型码 SO |
| status | text | 否 | CHECK in PENDING_RELEASE, RELEASED, CHANGE_APPROVAL, PARTIALLY_DELIVERED, DELIVERED, CLOSED, CANCELLED |
| customer_id | uuid | 否 | 逻辑引用 mdm |
| source_contract_id | uuid | 否 | 逻辑引用 clm，跨模块不建外键 |
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

约束与索引：`ux_sales_orders_legal_entity_id_doc_no`；追加索引 `ix_sales_orders_legal_entity_id_customer_id_status`（信用在途桶聚合）、`ix_sales_orders_source_contract_id`、`ix_sales_orders_legal_entity_id_status_promised_to`。

表 `sales.sales_order_lines`。列除公共列外为 `sales_order_id uuid not null`（外键）、`line_no int not null`、`customer_id uuid not null`（冗余自订单头，使信用聚合可走覆盖索引）、`source_contract_line_id uuid not null`、`item_kind text`、`item_id uuid not null`、`uom_code text not null`、`quantity numeric(18,6) not null`、`net_unit_price numeric(18,6) not null`、`tax_rate numeric(9,6) not null default 0`、`line_amount numeric(18,2) not null`、`line_amount_with_tax numeric(18,2) not null`、`delivery_date date not null`、`warehouse_id uuid`、`delivered_quantity numeric(18,6) not null default 0`、`returned_quantity numeric(18,6) not null default 0`、`open_amount_with_tax numeric(18,2) not null`、`status text not null CHECK in OPEN, PARTIALLY_DELIVERED, DELIVERED, CLOSED, CANCELLED`。

约束与索引：`ux_sales_order_lines_sales_order_id_line_no`；`ck_sales_order_lines_delivered_range` 约束 `delivered_quantity >= 0 and delivered_quantity <= quantity`；`ck_sales_order_lines_returned_range` 约束 `returned_quantity >= 0 and returned_quantity <= delivered_quantity`；追加索引 `ix_sales_order_lines_sales_order_id`、`ix_sales_order_lines_legal_entity_id_customer_id_status`（包含 `open_amount_with_tax` 的列顺序设计为使信用在途桶聚合不出现顺序扫描）、`ix_sales_order_lines_source_contract_line_id`。

表 `sales.delivery_schedules`。列为 `sales_order_id uuid not null`、`sales_order_line_id uuid not null`（外键）、`batch_no int not null`、`quantity numeric(18,6) not null`、`promised_date date not null`、`warehouse_id uuid`、`delivered_quantity numeric(18,6) not null default 0`、`status text not null CHECK in PENDING, DELIVERED, CLOSED, CANCELLED`、`delivery_confirmation_id uuid`（同 schema 外键，按裁定 A-09 因被引用表建于本表之后，该外键由第 V202611031021 号迁移以 `ALTER TABLE sales.delivery_schedules ADD CONSTRAINT fk_delivery_schedules_delivery_confirmations FOREIGN KEY (delivery_confirmation_id) REFERENCES sales.delivery_confirmations(id) ON DELETE RESTRICT` 补建，会话沿用 `lock_timeout = '5s'`）、`confirmed_date date`。约束 `ux_delivery_schedules_line_batch` 在 `(sales_order_line_id, batch_no)`；`ck_delivery_schedules_quantity_positive`；追加索引 `ix_delivery_schedules_legal_entity_id_promised_date_status`（交付指标的期间维度取数）、`ix_delivery_schedules_sales_order_id`。

表 `sales.delivery_confirmations`，单据类，类型码 DC，按裁定 A-09 由第 V202611031021 号迁移建立。

| 列 | 类型 | 可空 | 说明 |
|---|---|---|---|
| doc_no | text | 否 | 由 ep-platform-sequence 生成，类型码 DC |
| status | text | 否 | CHECK in DRAFT, CONFIRMED |
| customer_id | uuid | 否 | 逻辑引用 mdm |
| sales_order_id | uuid | 否 | 同 schema 外键 |
| posting_date | date | 否 | 记账日期，取值与用途见基线第 3.4 节 |
| warehouse_id | uuid | 是 | 出库仓库，直运时为空 |
| is_drop_ship | boolean | 否 | 默认 false |
| confirmed_at | timestamptz | 是 | 确认过账时点 |
| confirmed_by | uuid | 是 | 确认人 |
| voucher_id | uuid | 是 | 逻辑引用 ledger，确认时由凭证腿回填 |
| remark | text | 是 | 备注 |

约束与索引：`ux_delivery_confirmations_legal_entity_id_doc_no`；追加索引 `ix_delivery_confirmations_sales_order_id`、`ix_delivery_confirmations_legal_entity_id_posting_date`。不设作废态，冲正一律经销售退货单，理由是基线第 3.6 节禁止软删除且已过账分录只追加。本表不带 `accounting_period_id`，与第 11.2 小节的偏离项一并登记。

表 `sales.delivery_confirmation_lines`，按裁定 A-09 由第 V202611031022 号迁移建立。列除公共列外为 `delivery_confirmation_id uuid not null`（同 schema 外键）、`line_no int not null`、`sales_order_line_id uuid not null`（同 schema 外键）、`delivery_schedule_id uuid not null`（同 schema 外键）、`item_kind text`、`item_id uuid not null`、`uom_code text not null`、`quantity numeric(18,6) not null`、`net_unit_price numeric(18,6) not null`、`tax_rate numeric(9,6) not null default 0`、`line_amount numeric(18,2) not null`、`line_amount_with_tax numeric(18,2) not null`、`warehouse_id uuid`、`batch_no text not null default '-'`、`serial_nos text[] not null default '{}'`、`cogs_amount numeric(18,2)`（确认时由库存腿回填）、`stock_movement_id uuid`（逻辑引用 inventory）。约束 `ux_delivery_confirmation_lines_confirmation_id_line_no`；追加索引 `ix_delivery_confirmation_lines_sales_order_line_id`。批次列与序列号列取固定值按基线第 11.4 节。

表 `sales.sales_order_versions`，仅追加。列为 `sales_order_id uuid not null`、`version_no int not null`、`snapshot jsonb not null`、`change_id uuid`、`created_at`、`created_by`、`reverses_id uuid null`。约束 `ux_sales_order_versions_order_version`。

表 `sales.sales_order_changes`。列为 `sales_order_id uuid not null`、`from_version_no int not null`、`to_version_no int`、`status text not null CHECK in DRAFT, PENDING_APPROVAL, APPROVED, REJECTED, WITHDRAWN`、`reason text not null`、`flow_instance_id uuid`、`requires_recheck boolean not null default false`、`recheck_validation_id uuid`、`applied_at timestamptz`。追加索引 `ix_sales_order_changes_sales_order_id_status`。

表 `sales.sales_order_change_lines`。列为 `sales_order_change_id uuid not null`（外键）、`sales_order_line_id uuid`、`operation text not null CHECK in ADD, MODIFY, CLOSE`、`new_quantity numeric(18,6)`、`new_delivery_date date`、`new_warehouse_id uuid`、`new_net_unit_price numeric(18,6)`、`source_contract_line_id uuid`。

表 `sales.sales_returns`，单据类。列为 `doc_no text`、`status text CHECK in DRAFT, SUBMITTED, REGISTERED, CLOSED, CANCELLED`、`customer_id uuid not null`、`sales_order_id uuid not null`、`return_reason text not null`、`return_warehouse_id uuid`、`posting_date date not null`（记账日期，取值与用途见基线第 3.4 节）、`is_drop_ship boolean not null default false`、`registered_at timestamptz`、`flow_instance_id uuid`、`remark text`。约束 `ux_sales_returns_legal_entity_id_doc_no`；追加索引 `ix_sales_returns_legal_entity_id_customer_id_status`、`ix_sales_returns_sales_order_id`、`ix_sales_returns_legal_entity_id_posting_date`。

表 `sales.sales_return_lines`。列为 `sales_return_id uuid not null`（外键）、`line_no int not null`、`sales_order_line_id uuid not null`、`item_kind text`、`item_id uuid not null`、`uom_code text not null`、`quantity numeric(18,6) not null`、`net_unit_price numeric(18,6) not null`、`tax_rate numeric(9,6) not null`、`line_amount numeric(18,2) not null`、`line_amount_with_tax numeric(18,2) not null`、`warehouse_id uuid`、`batch_no text not null default '-'`、`serial_nos text[] not null default '{}'`。约束 `ux_sales_return_lines_return_line_no`。批次列取固定值 `'-'` 而非 NULL，按基线第 11.4 节。

表 `sales.return_line_delivery_links`，多对多，承载退货明细行与交付确认单的关联，是规格第 5.2 章退货回冲取价三分支的输入。列为 `sales_return_line_id uuid not null`（外键）、`delivery_confirmation_id uuid not null`（同 schema 外键，按裁定 A-09 由逻辑引用改为真实外键 ON DELETE RESTRICT）、`delivery_confirmation_line_id uuid not null`（同 schema 外键，同上）、`quantity numeric(18,6) not null`、`assigned_by text not null CHECK in MANUAL, AUTO_FIFO`。约束 `ux_return_line_delivery_links_pair` 在 `(sales_return_line_id, delivery_confirmation_line_id)`；追加索引 `ix_return_line_delivery_links_delivery_confirmation_id`。

表 `sales.exchange_links`。列为 `sales_return_id uuid not null`、`replacement_delivery_schedule_id uuid not null`、`linked_at timestamptz not null`、`linked_by uuid not null`。约束 `ux_exchange_links_pair`。

表 `sales.order_validations` 与 `sales.order_validation_items`，列结构与 `clm.contract_validations` 及其明细完全同构，只把 `contract_id` 换为 `sales_order_id`，`occasion` 取 `CHECK in RELEASE, CHANGE_SUBMIT, CHANGE_APPROVE`。两处同构而分表的理由见第 11.2 小节。

视图 `sales.v_credit_exposure_in_transit`：按 `(legal_entity_id, customer_id)` 汇总 `sales.sales_order_lines` 中订单状态属于 RELEASED、CHANGE_APPROVAL、PARTIALLY_DELIVERED 且行状态属于 OPEN、PARTIALLY_DELIVERED 的 `open_amount_with_tax` 合计。待放行订单不计入，与 PRD 3.14.2 末句一致。
按裁定 A-18，第 V202611031075 号迁移 `V202611031075__sales_create_dataset_views.sql` 建立本模块的两个受治理数据集视图。`sales.v_sales_orders_dataset` 的 dataset code 为 `sales_sales_orders`，grain 取 DOCUMENT，取数为 `sales.sales_orders`；`sales.v_order_delivery_batches` 的 dataset code 为 `sales_order_delivery_batches`，grain 取 DOCUMENT_LINE，取数为 `sales.delivery_schedules`，输出列含 `sales_order_id`、`sales_order_line_id`、`batch_no`、`quantity`、`delivered_quantity`、`promised_date`、`warehouse_id`、`status`、`delivery_confirmation_id`、`confirmed_date`。两个视图都必须含 `legal_entity_id`、`security_level`、`data_scope_tags` 三列，都不做聚合、不跨 schema 连接，并在同一迁移内执行 `GRANT SELECT ON <视图> TO ep_analyst_ro`，不授予 `ep_app_rw` 之外的任何写权限。列名与类型签名必须与阶段 11 的 `reporting.dataset_fields` 登记一致，由阶段 11 的启动自检项 `reporting-dataset-signature-matched` 校验。该文件头的 `-- rollback:` 段为 `drop view` 与对应的 `revoke`。

#### 3.5 数据库角色与迁移账号

本阶段不新增数据库角色。三个 schema 的属主分别为 `ep_mod_cpq`、`ep_mod_clm`、`ep_mod_sales`，迁移由 `ep_migrator` 在迁移窗口执行，运行期由 `ep_app_rw` 读写，只读分析由 `ep_analyst_ro` 访问。迁移会话固定 `SET lock_timeout = '5s'` 与 `SET statement_timeout = '30min'`。

---

### 4. 领域模型与关键算法

本阶段不自行生成总账凭证，也不自行写库存数量账与金额账。交付确认的三腿一律经对方模块的契约端口在 `confirm_delivery` 的同一事务内触发，凭证与库存账由被调方写入，本阶段只传入来源单据与计量项并回填返回值；销售退货登记只产生业务单据与领域事件。分录、取价、回冲单价与税额分支一律按规格第 5.2 章财务规则条目的事件-分录表及其规则块，由财务模块与库存模块承接，本节不复述。

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
| IN_PERFORMANCE | TERMINATED | 提前终止 | 见第 11.3 小节的临时取值 |

合同有效期止日届满不改变状态，只作为提醒触发源，由 `clm.v_contract_reminder_sources` 承载。已生效合同的修订不在原版本上改写，一律经 `actions/amend` 生成 `version_no + 1` 的新草稿版本并重走审批与生效链路。

#### 4.3 订单与分批交付行状态机

订单状态按 PRD 3.11.5，守卫条件的要点为：PENDING_RELEASE 到 RELEASED 要求信用与库存两项重跑通过，或信用审批结论为通过；RELEASED 到 CANCELLED 要求全部行 `delivered_quantity` 为零；PARTIALLY_DELIVERED 到 CLOSED 要求关闭原因非空并写入审计；CHANGE_APPROVAL 的进入与退出由 `sales.sales_order_changes` 驱动，退出时回到进入变更前的状态。DELIVERED、CLOSED、CANCELLED 为终态，开票与到款不改变订单状态。

分批交付行状态为 PENDING、DELIVERED、CLOSED、CANCELLED 四取值。逾期不是状态，由 `promised_date` 与当前服务器自然日派生，自然日取值一律用 `(now() AT TIME ZONE 'Asia/Shanghai')::date`。

#### 4.4 取价与行金额算法

输入为法人、客户、产品或物料、计量单位、单据日期、数量、录入单价、折扣率与操作者的价格权限档案。

1. 经 `ep-contract-cpq::PriceListQueryPort` 取命中的价目行。命中判定的输入与筛选条件按 PRD 2.8.3，由主数据阶段实现，本阶段只消费。
2. 多行命中时返回 `CPQ.PRICE_AUTHORITY.MULTIPLE_PRICE_LIST_HITS`，携带全部命中行，要求操作者显式选择，不由系统任意取一行。
3. 无命中时不阻断，`list_unit_price` 与 `price_floor` 留空。
4. `net_unit_price = round6(unit_price * (1 - discount_rate))`，舍入策略为四舍五入且中值远离零。
5. `line_amount = round2(quantity * net_unit_price)`；`line_amount_with_tax = round2(line_amount * (1 + tax_rate))`，`is_tax_included` 为真时改为 `line_amount = round2(quantity * net_unit_price / (1 + tax_rate))` 且 `line_amount_with_tax = round2(quantity * net_unit_price)`。
6. 中间值在内存中以全精度 Decimal 保留，只在写库前一次性 round，按基线第 3.5 节。
7. 合同金额与订单金额由行金额按 2 位小数直接累加，不再二次舍入，因此头与行天然相等。

边界条件：数量为零或负数在字段级校验阶段即被拒绝；折扣率为 1 时净单价为零，允许但一律标记待折扣审批；默认税率按裁定 C-11 经 `ep_contract_invoice::TaxRateOptionQuery::default_rate` 取得，不经 `ep-contract-mdm`；税率字典的唯一出处是 `invoice.tax_rate_options`，其建表迁移与种子迁移两条及 `TaxRateOptionQuery` 的 `default_rate` 与 `list` 两个方法属阶段 10 的 T0 切片第五项，自 T0 起即可取用，`MdmTaxRateStub` 整项撤销，阶段 5 不提供任何税率桩；税率的取值集合依赖 U-D-04，未定不阻塞本阶段实现，合同行与订单行一律按行携带 `tax_rate`。

#### 4.5 价格权限判定

对每一合同行独立判定，判定结果只打标不阻断，与 PRD 3.3.3 第一行一致。

- 取操作者在单据日期上生效的价格权限档案，按 USER、POSITION、ROLE 的顺序取第一条命中；三级均无命中时返回 `CPQ.PRICE_AUTHORITY.NOT_CONFIGURED` 并阻断提交，理由是无权限基准时无法判定，静默放行会使折扣审批链永不触发。
- `discount_rate > max_discount_rate` 时 `requires_discount_approval` 置真。
- 存在 `price_floor` 且 `net_unit_price < price_floor` 且 `allow_below_price_floor` 为假时置真。
- 价目未命中且 `allow_no_price_list_hit` 为假时置真。
- 合同行中存在任一 `requires_discount_approval` 为真时，提交审批时挂起折扣审批链；全部为假时不进入折扣审批节点，其余三条链照常执行。

#### 4.6 五项校验

校验在一个只读事务内取数，判定结论与取数快照写入 `clm.contract_validations` 与其明细，并按基线第 9.4 节写入审计事件。执行顺序固定为合同校验、价格权限、库存可用量、交期、客户信用额度。

1. 合同校验，阻断项。判定内容为头行必填齐备、客户与产品处于启用状态、每一合同行的 `delivery_date` 落在 `[valid_from, valid_to]` 区间内、三类信息齐备。三类信息齐备的判定为：条款正文非空且交付节点至少一条；收付款期次至少一条且比例合计等于 1 或金额合计等于合同金额；附件中至少存在一个 `purpose = CONTRACT_BODY` 的对象。该四项是 U-E-09 的待决内容，本阶段按 PRD 3.3.3 的拟定实现，结论落定后回写。
2. 价格权限，按第 4.5 小节，不阻断。
3. 库存可用量，对每一非直运行经 `ep_contract_inventory::AvailabilityQueryPort::available` 按法人、物料、仓库、交期日取可用量，可用量小于该行数量时该项记为 FAILED。
4. 交期，取库存可用量项的结论派生，可用量不足即交期不可满足。
5. 客户信用额度，按第 4.7 小节。

处置：合同校验 FAILED 一律阻断并定位到字段；库存可用量与交期 FAILED 在建单提交时不阻断，只记录并使派生时该订单进入待放行，理由与临时取值见第 11.3 小节；信用额度 FAILED 按 `sales.credit_policies` 的 `on_exceed` 取值阻断或转审批。

#### 4.7 客户信用额度算法

已占用金额由三部分构成，三者按同一订单不重复占用，构成与迁移时点严格按 PRD 3.14.2 与规格第 5.2 章客户信用额度校验条目。

实现口径为不设独立的占用台账，三部分各由其状态的权威模块给出，非重复由生命周期本身保证。

- 在途订单金额：由本阶段的 `sales.v_credit_exposure_in_transit` 给出，等于已放行且尚未交付部分的含税金额合计，`open_amount_with_tax = round2((quantity - delivered_quantity) * net_unit_price * (1 + tax_rate))`，该列在交付确认回写、订单变更生效、订单取消与关闭三处同事务维护。
- 已交付未开票金额与应收未收金额：经 `ep_contract_finance::ReceivableExposureQuery::exposure` 一次调用取回 `delivered_unbilled_amount` 与 `receivable_open_amount` 两项，其取数分别为该客户在应收账款未开票过渡科目上的借方余额与应收台账未核销余额合计。按裁定 C-14，`finance::CreditExposureQuery` 与 `finance::CustomerCreditExposurePort` 两个旧名作废；对外唯一入口是本模块的 `ep_contract_sales::CreditExposureQueryPort::exposure`，由本阶段把在途桶与上述两项组装为 `CreditExposureView` 的 `credit_limit`、`in_transit_amount`、`delivered_unbilled_amount`、`receivable_open_amount`、`available_amount` 五项返回。本阶段不注入任何替身，`ReceivableExposureQuery` 按裁定 C-14 不进 T0 切片，与阶段 10 该端口按第 11.5 小节同批交付同批验收，承载三桶组装的用例整体落在第三批并在该批次一次接线，三桶取数当场成立，不存在只取两桶、取 `None` 或以零值参与求和的形态。

判定步骤：

1. 在信用校验事务内对 `sales.customer_credit_controls` 的该客户行执行 `SELECT ... FOR UPDATE`，行不存在时先插入。该行是同一客户并发下单的串行化点，`lock_timeout` 为 3 秒，超时返回 `BUSINESS_CONFLICT`。
2. 取客户档案的信用额度。为空时按 `null_limit_behavior` 处置，出厂默认 `TREAT_AS_ZERO`。
3. 取三部分占用并求和，金额口径按 `amount_basis`，出厂默认 `WITH_TAX`。
4. 本次待增加占用：合同建单提交时取本合同全部合同行的含税金额合计；合同生效派生时取本次派生的订单行含税金额合计；订单变更时取变更后与变更前的在途金额之差且只在为正时判定。
5. 判定 `requested + occupied <= credit_limit`。不成立时按 `on_exceed` 阻断或转审批，出厂默认 `REVIEW`。
6. 把信用额度、三部分取值、本次待增加占用、超出金额与判定结论写入校验明细的 `snapshot` 并写入审计。
7. 更新 `customer_credit_controls.last_exposure` 与 `last_checked_at`，用于界面展示与对账，不作为判定依据。

释放的反向情形按同一映射反向执行，不设单独的释放动作：订单取消与剩余数量关闭把对应行的 `open_amount_with_tax` 归零；交付确认使该部分自在途桶移出并由财务侧进入已交付未开票桶；开票、到款与红冲的桶间迁移全部发生在财务侧。销售退货登记本身不改变本阶段的在途桶，因为退货针对的是已交付部分，其释放体现为财务侧两个桶的减少，这一点在第 8 节以专门用例验证。

边界条件：客户在两个法人下分别设额度，跨法人不合并，理由与临时取值见第 11.3 小节；预收账款不抵减占用，由 `deduct_advance_receipts` 开关承载，默认关闭；`ep-contract-finance` 端口不可用时信用校验返回 `INFRASTRUCTURE` 且可重试，不静默按零占用放行。已交付未开票与应收未收两桶随第三批与阶段 10 的 `ReceivableExposureQuery` 同批交付，交付即为真实取数；校验明细的 `snapshot` 一律记录信用额度与三桶的真实取值，不存在按未接线呈现或以 `None` 参与判定的形态。

#### 4.8 合同生效与派生算法

生效动作在一个事务内完成：校验重新认证凭证、校验生效审批结论、写合同状态为 EFFECTIVE、写 `clm.contract_versions` 快照、写审计事件、写 Outbox 条目 `clm.contract.effective.v1`。事务内不发起任何外部调用，不读写附件正文。

派生编排在 job-worker 中执行，步骤如下。

1. 消费 `clm.contract.effective.v1`，在 `platform_msg.inbox_consumptions` 上以 `(consumer = 'clm.derivation', event_id)` 唯一约束保证只处理一次。
2. 在一个事务内建立 `clm.contract_derivations` 批次行与全部 `clm.contract_derivation_items`，`item_total` 一次算定。批次行的唯一约束在 `(contract_id, contract_version_no, trigger)`，重复投递直接命中冲突并结束。
3. 派生项的生成规则：销售订单一张，含全部合同行；采购需求按 `requires_procurement` 为真或 `order_type = DROP_SHIP` 的合同行逐行一条；项目任务在 `contract_types.requires_project` 为真时按交付节点逐条一条；收款计划按收付款期次逐期一条；交付节点为本模块内对象，派生项的动作是把 `clm.contract_milestones.status` 由 PLANNED 置为 ACTIVE，纳入同一批次只为使追溯与计数口径一致。其中收款计划派生项按裁定 C-20 只写本模块的 `clm.contract_payment_schedules`，不调用任何外部端口；项目任务派生项按裁定 C-19 只登记不派发，在批次建立的同一事务内即置 `status = DONE`、`target_module` 取 project、`target_doc_id` 留空，实际项目任务由阶段 12 的 `project.contract_derivation` 消费者消费 `clm.contract.effective.v1` 后经本小节末段的 `ContractDerivationPlanQuery` 自行派生，追溯经该查询的 `unique_key` 对应，本阶段不再同步派生项目任务。
4. 每个派生项一个独立事务，调用目标模块的契约端口，`Idempotency-Key` 取该派生项的 `id`，本身是 UUIDv7。目标端口固定为：销售订单经 `ep_contract_sales::SalesOrderDerivationPort`；采购需求经 `ep_contract_procure::PurchaseRequisitionIntakePort::intake`，其 `unique_key` 取 `CONTRACT:{contract_id}:{contract_line_id}:{contract_version}`，按裁定 C-17 与第 11.5 小节该端口的派发整条推迟到阶段 7，本阶段不注入替身也不写调用点；交付节点与收款计划两类为本模块内写入，不出模块；项目任务类不调用端口，按第 3 步处理。成功写 `target_doc_id`、`target_doc_no` 与 `status = DONE`；采购需求派生项在阶段 7 接线之前 `status` 恒为 PENDING、`target_doc_id` 与 `target_doc_no` 留空且不计入 `item_done`，因此含该项的合同停在 EFFECTIVE 且 `derivation_state` 保持 RUNNING，界面按未接线呈现，阶段 7 接线后补派发并推进到 IN_PERFORMANCE，不得构造占位单号；失败 `attempts + 1` 并按基线第 6.2 节的八档退避重排 `available_at`，八次全部失败置 `DEAD` 并写入 `platform_msg.dead_letters`。
5. 销售订单派生项内先执行第 4.6 小节的后四项校验，结论写 `sales.order_validations`；四项全通过则订单状态为 RELEASED，信用或库存任一不足则为 PENDING_RELEASE 并写 `pending_release_reason`，同时按 `on_exceed` 追加信用审批节点或直接置为待放行。
6. 全部项 DONE 时把合同置为 IN_PERFORMANCE、`derivation_state = DONE`，并发 `clm.contract.derivation_completed.v1`。存在 DEAD 项时 `derivation_state = FAILED`，合同保持 EFFECTIVE 并在界面显示待人工修复，与 PRD 3.6 中已生效到已生效的自环一致。
7. 人工修复后可重放该批次，重放按第 3.3 小节的两道唯一约束去重，不产生重复单据。

边界条件：单张合同的派生项数上限由 `EP__CLM__DERIVATION__MAX_ITEMS_PER_CONTRACT` 约束，默认 2000，超出直接拒绝生效并提示拆分合同；派生过程中合同不接受修订，`actions/amend` 在 `derivation_state = RUNNING` 时返回 `CLM.CONTRACT.DERIVATION_IN_PROGRESS`。
按裁定 A-16，本阶段在 `crates/contract/clm/src/port/derivation.rs` 提供 `ContractDerivationPlanQuery::derivation_plan(tx, ctx, contract_id, contract_version_no)`，返回 `ContractDerivationPlan`，实现落在 ep-app-clm，取数与上述派生项生成规则同源，不另建第二套计划。其 `items` 的 `unique_key` 取值规则固定为 `<contract_id>:<contract_version_no>:<item_kind>:<source_contract_line_id 或 milestone_no>`，阶段 12 的派生任务以该键做唯一性去重。该键与第 4 步采购需求经 `PurchaseRequisitionIntakePort` 传入的 `unique_key` 不是同一个键，前者供阶段 12 的派生去重，后者供采购需求登记去重，两者各按其裁定取值。

#### 4.9 合同变更后的重新派生

按 PRD 3.5.4 的五种情形分派处理。

| 情形 | 处理 |
|---|---|
| 新增合同行 | 在已派生订单上追加订单行，追加动作走 `SalesOrderDerivationPort::append_lines`，追加后按第 4.6 小节重跑后四项校验 |
| 已派生未开始交付的行数量或交期变更 | 调整对应订单行与其分批交付行，走第 4.10 小节的订单变更版本 |
| 已部分交付的行变更 | 只允许调整未交付部分，`new_quantity` 不得小于 `delivered_quantity`，否则返回 `SALES.SALES_ORDER.DELIVERED_QTY_EXCEEDED` |
| 收付款信息变更 | 按裁定 C-20 直接维护本模块的 `clm.contract_payment_schedules`，只调整尚未开票且尚未到款的期次，已开票或已到款的期次不调整；`ep_contract_finance::ReceivablePlanPort` 已撤销，不再派生第二套收款计划 |
| 交付节点变更 | 调整 `status = ACTIVE` 的节点，`status = CONFIRMED` 的节点不调整 |

#### 4.10 订单变更与分批交付

订单变更：提交变更建立 `sales.sales_order_changes` 并把订单置为 CHANGE_APPROVAL，此后数量、单价、交期、仓库四类字段被锁定，其余字段仍可维护。审批通过后在一个事务内写旧版本快照、应用变更行、`version_no + 1`、重算 `open_amount_with_tax`、解除锁定、按 `requires_recheck` 重跑库存可用量、交期与信用三项。审批驳回则订单回到进入变更前的状态，变更单置 REJECTED。单价的修改不在订单上直接进行，`new_net_unit_price` 只允许由合同变更派生的变更单携带，用户直接提交时返回 `SALES.SALES_ORDER.PRICE_CHANGE_NOT_ALLOWED`。

拆分与合并：同一订单行的全部分批交付行数量合计必须等于该订单行数量，不等时返回 `SALES.DELIVERY_SCHEDULE.SPLIT_SUM_MISMATCH`。未拆分的订单行在派生时即建立一条分批交付行，其数量与约定交付日期取订单行取值，因此系统中不存在没有分批交付行的订单行，这使交付确认与交付指标的取数只有一条路径。`status` 不为 PENDING 的分批交付行不可再拆分、不可改数量与仓库，返回 `SALES.DELIVERY_SCHEDULE.NOT_SPLITTABLE`。拆分与合并不改变订单总量、总金额与信用占用总额，该性质由领域属性测试守护。

#### 4.11 交付确认与三腿过账

交付确认单是规格第 8 章黄金业务闭环第 8 步的唯一落点，按裁定 A-09 归本阶段。单据只有 DRAFT 与 CONFIRMED 两个状态，不设作废态，冲正一律经销售退货单。建表与登记动作属第二批，确认动作的三腿一次全真接线属第三批，按第 11.5 小节与阶段 10 的 `UnbilledArPort` 同批交付同批验收；该批次之外本阶段不建该调用点，`confirm_delivery` 用例与其端点不写入代码，也不注入任何替身，因此系统内不存在只落两腿的已确认交付。

登记动作 `create_delivery_confirmation` 位于 `crates/application/sales/src/usecase/create_delivery_confirmation.rs`。按 `sales_order_id` 取该订单下 `status = PENDING` 的分批交付行，逐条建立 `sales.delivery_confirmation_lines`，`sales_order_line_id` 与 `delivery_schedule_id` 均为同 schema 外键；本次数量不得超过该分批交付行的 `quantity - delivered_quantity`，超出返回 `SALES.DELIVERY_CONFIRMATION.QTY_EXCEEDS_SCHEDULED`；`net_unit_price`、`tax_rate`、`line_amount` 与 `line_amount_with_tax` 自订单行带出，不在本单重新取价；`is_drop_ship` 自订单头的 `order_type` 派生；`posting_date` 由操作者录入，取值与用途按基线第 3.4 节。

确认动作 `confirm_delivery` 位于 `crates/application/sales/src/usecase/confirm_delivery.rs`，在一个事务内完成，四步次序固定，不得调换。

1. 会计期间解析。经 `ep_contract_ledger::AccountingPeriodResolver::resolve` 在事务最前解析一次，库存腿与过渡科目腿复用其返回值，不各自再解析。
2. 库存腿，端口由阶段 8 提供。经 `ep_contract_inventory::InventoryPostingPort::post_outbound(tx, ctx, OutboundPosting { reason: MovementReason::DeliveryConfirmation, pricing: OutboundPricing::MovingAverage, source: SourceRef{ doc_type: DELIVERY_CONFIRMATION, .. }, lines })`，返回每行的 `cogs_amount` 与 `stock_movement_id` 并回填到 `sales.delivery_confirmation_lines`。`SourceDocType::DELIVERY_CONFIRMATION` 由本模块传入。`is_drop_ship` 为真时整段跳过，两列留空。
3. 过渡科目腿，端口由阶段 10 提供。经 `ep_contract_finance::UnbilledArPort::record_on_delivery(tx, ctx, DeliveryUnbilledArCommand { delivery_confirmation_id, customer_id, posting_date, accounting_period_id, direction: DEBIT, net_amount })`，写 `finance.unbilled_ar_entries`。该端口与本用例按第 11.5 小节同批接线，本阶段不注入替身。
4. 凭证腿，端口由阶段 9a 提供。经 `ep_contract_ledger::PostingPort::post(tx, ctx, PostingInput { source_kind: VoucherSourceKind::DELIVERY_CONFIRMED, branch: DROP_SHIP 或 NON_DROP_SHIP, posting_date, source_document, measures })`，`measures` 含 `revenue_amount`、`unbilled_receivable_amount`、`cogs_amount`、`inventory_release_amount` 四项，返回的凭证标识回填到 `sales.delivery_confirmations.voucher_id`。

四步之后在同一事务内把单据置 CONFIRMED，写 `confirmed_at` 与 `confirmed_by`，写审计事件，写 Outbox 条目 `sales.delivery.confirmed.v1`。任一腿失败整笔回滚，不存在只写一腿的中间态。本阶段不判定借贷方向、不取价、不确定科目，四项 `measures` 的口径与其对应分录一律按规格第 5.2 章的事件-分录表，本小节不复述。

事件 `sales.delivery.confirmed.v1` 的 `aggregate_type` 取 `sales.delivery_confirmations`，payload 字段固定为 `delivery_confirmation_id`、`doc_no`、`sales_order_id`、`customer_id`、`contract_id`、`is_drop_ship`、`voucher_id`、`lines`，其中 `lines` 每元素含 `delivery_confirmation_line_id`、`sales_order_line_id`、`delivery_schedule_id`、`item_kind`、`item_id`、`quantity`、`warehouse_id`、`batch_no`、`serial_nos`、`revenue_amount`、`cogs_amount`；`revenue_amount` 取该行的 `line_amount`，`cogs_amount` 取库存腿的回填值，直运时为空。信封的 `posting_date` 取单据的 `posting_date`，`accounting_period_id` 取 PostingPort 返回值。

回写由第 2.3 节的两个消费者承担，各只写本模块 schema。`sales.delivery_writeback` 推进分批交付行的 `delivered_quantity` 与 `status`、订单行的 `delivered_quantity` 与 `open_amount_with_tax`、订单状态，并把 `sales.delivery_schedules.delivery_confirmation_id` 与 `confirmed_date` 回填；`clm.milestone_confirm` 推进 `clm.contract_milestones` 的交付节点。在途桶按第 4.7 小节由该回写移出，不设第二条回写路径。

#### 4.12 销售退货

前置校验按 PRD 3.13.1：退货数量不超过该订单行 `delivered_quantity - returned_quantity`；每一退货明细行必须至少关联一条交付确认单行，关联方式为操作者指定或按交付先后自动带出，写入 `sales.return_line_delivery_links` 并记录 `assigned_by`；该退货部分已开票的必须先完成红字冲销，按裁定 C-16 由 `ep_contract_invoice::InvoiceReversalStatusQuery::is_fully_credit_noted(tx, ctx, sales_order_line_id, quantity)` 判定，未完成时返回 `SALES.SALES_RETURN.INVOICE_NOT_CREDIT_NOTED` 并列出待冲销的发票，原 `InvoiceStatusPort` 一名作废，该判定按裁定 C-16 不进 T0 切片，与阶段 10 的该 trait 按第 11.5 小节同批交付同批验收，本阶段不注入替身，承载该判定的退货登记分支整体落在第三批并在该批次当场成立；直运订单的退货 `is_drop_ship` 为真，不产生库存流水，按裁定 B-07 在同一登记事务内调用 `ep_contract_procure::PurchaseReturnLinkPort::link_drop_ship_return(tx, ctx, sales_return_id, lines)` 勾稽对应的采购退货，该调用整条推迟到阶段 7，本阶段不注入替身也不写调用点，阶段 7 之前系统内不存在采购订单，直运订单无从交付，该路径由既有的 `SALES.SALES_RETURN.QTY_EXCEEDS_DELIVERED` 自然阻断，不新增错误码。

登记动作在一个事务内写退货单状态为 REGISTERED、更新订单行 `returned_quantity`、写审计、写 Outbox 条目 `sales.sales_return.registered.v1`。该事件的载荷携带退货明细行与其交付确认单关联，是库存模块回冲取价与财务模块生成红字分录的输入，取价与分录一律按规格第 5.2 章销售退货事件与退货回冲的取价三分支。
三个终态动作同样各自在一个事务内写状态、审计与 Outbox 条目，按裁定 A-17：REGISTERED 迁到 CLOSED 发 `sales.sales_return.closed.v1`，payload 含 `sales_return_id`、`doc_no`、`sales_order_id`、`source_ref`、`closed_at`；任一状态迁到 CANCELLED 发 `sales.sales_return.cancelled.v1`，payload 另含 `cancel_reason`；SUBMITTED 因审批驳回退回 DRAFT 发 `sales.sales_return.rejected.v1`，payload 另含 `reject_reason` 与 `approval_ref`。三者与既有的 `sales.sales_return.registered.v1` 一并登记在第 6.3 小节的事件登记表。退货单的对外创建入口固定为 `ep_contract_sales::SalesReturnCommandPort::create_sales_return`，`CreateSalesReturn`、`SalesReturnSourceRef`、`CreateSalesReturnLine`、`SalesReturnDeliveryLink` 与 `SalesReturnView` 五个 DTO 的字段按裁定 A-17 冻结，阶段 12 的服务工单退货经该端口调用，不另起第二个入口。

换货不设独立单据，按一笔退货加一笔在原订单上追加或放行的分批交付行表达，两者之间写 `sales.exchange_links`。

#### 4.13 电子签章编排

1. 合同全部审批节点通过后置为 PENDING_SIGNATURE。签署方式为 ESIGN 时建立 `clm.signature_requests` 并写 Outbox 条目 `clm.contract.signature_requested.v1`。
2. job-worker 消费该事件，经回环 HTTP 调用 integration-gateway 的 `POST /internal/v1/esign/requests`。core-server 与 job-worker 都不直接出网。
3. integration-gateway 经 `ep-adapter-esign` 提交签署，记录 `external_request_id`，写 `clm.signature_events` 的 SUBMITTED 条目。请求超时取 `EP__CLM__ESIGN__REQUEST_TIMEOUT_MS`，失败按指数退避重试，连续失败达阈值触发熔断。
4. integration-gateway 按 `EP__CLM__ESIGN__POLL_INTERVAL_SECONDS` 轮询签署状态，每次轮询写一条 POLLED 事件，直到状态为已签署、已拒绝或超过 `EP__CLM__ESIGN__POLL_MAX_HOURS`。不设公网入站回调，理由见第 11.2 小节。
5. 已签署时拉取带签章的合同文件，执行验签，验签通过后经 `ep-platform-file` 写入附件对象并建立 `purpose = SIGNED_FILE` 的关联，`verify_result` 置 PASSED，写 SIGNED 与 VERIFIED 两条事件，发 `clm.contract.signed.v1`。验签失败置 `verify_result = FAILED` 并返回 `CLM.SIGNATURE_REQUEST.VERIFY_FAILED`，合同保持 PENDING_SIGNATURE。
6. 外部不可用时按规格第 15.1 章归类为 EXTERNAL_SYSTEM，HTTP 502，`retryable` 为真，合同保持 PENDING_SIGNATURE 并显示可重试提示；耗尽重试后进入死信与人工处理。
7. 实体印章路径不经外部系统，由用印责任人登记 `clm.seal_usages` 并上传扫描件，登记完成即满足生效守卫。

---

### 5. API 契约

全部端点遵循基线第 5 节：路径前缀 `/api/v1`，JSON 字段 snake_case，成功与失败封套固定，写请求必须带 `Idempotency-Key`，请求头固定集合含 `Authorization`、`X-Legal-Entity-Id`、`X-Device-Id`、`X-Client`，高风险操作另带 `X-Reauth-Token`。分页、排序与过滤按基线第 5.3 节，本阶段各列表端点的排序白名单在下表逐个给出。存在性泄漏一律按基线第 5.5 节返回 404 与 `PLATFORM.AUTHZ.NOT_FOUND_OR_DENIED`。
本节全部路由按裁定 A-20 逐用例声明一对常量，命名为 `<USECASE_SCREAMING>_DOMAIN` 与 `<USECASE_SCREAMING>_ACTION`，类型取阶段 1 在 ep-foundation 冻结的 `CapabilityDomain` 与 `ActionClass`，本阶段不重新定义能力域码。第 5.1 小节 CLM 端点的能力域取 `CapabilityDomain::ClmContractEsign`，声明在 `crates/contract/clm/src/capability.rs`；第 5.2 小节 SALES 端点的能力域取 `CapabilityDomain::SalesOrderFulfillment`，声明在 `crates/contract/sales/src/capability.rs`；第 5.3 小节 `/api/v1/cpq/price-authorities` 的能力域按裁定 A-20 取 `CapabilityDomain::SalesOrderFulfillment`，在阶段 5 已建的 `crates/contract/cpq/src/capability.rs` 中只追加不重定义。动作类别的取值规则为：只读查询取 `Read`，创建与修改取 `Write`，`actions/submit-for-approval`、`actions/release`、`actions/register`、`actions/confirm-delivery` 一类提交动作取 `Submit`；审批结论一律由 ep-platform-flow 的审批任务端点承载，本阶段不出现 `Approve`；本阶段无导出路由，不出现 `Export`。第 5.4 小节的 integration-gateway 内部端点在 `/internal/v1/` 下、不对四端暴露，按 A-20 不参与判定也不声明常量。`xtask configdoc` 断言每个 `/api/v1/` 路由都能解析到一对常量，缺失即构建失败。

#### 5.1 CLM 端点

| 方法与路径 | 请求要点 | 响应要点 | 主要错误码 | 幂等语义 | 权限 |
|---|---|---|---|---|---|
| POST /api/v1/clm/contracts | 合同头字段，行可空 | 合同视图，status 为 DRAFT | VALIDATION | 幂等键四元组，重放回首次结果 | clm.contract.create |
| GET /api/v1/clm/contracts | 排序白名单 created_at、doc_no、valid_to、total_amount；过滤 status、customer_id、contract_type_id、valid_to、owner_user_id | 分页列表，默认排序 created_at desc, id desc，默认筛选最近 3 个自然月 | 无 | 读请求无幂等键 | clm.contract.read |
| GET /api/v1/clm/contracts/{id} | 无 | 合同头行条款节点期次附件的完整视图 | PLATFORM.AUTHZ.NOT_FOUND_OR_DENIED | 无 | clm.contract.read |
| PATCH /api/v1/clm/contracts/{id} | 携带 row_version | 更新后的视图 | PLATFORM.CONCURRENCY.STALE_VERSION | 幂等键 | clm.contract.update |
| PUT /api/v1/clm/contracts/{id}/lines | 全量替换合同行 | 行列表与汇总金额 | VALIDATION、CPQ.PRICE_AUTHORITY.MULTIPLE_PRICE_LIST_HITS、CPQ.PRICE_AUTHORITY.NOT_CONFIGURED | 幂等键 | clm.contract.update |
| PUT /api/v1/clm/contracts/{id}/terms | 关键条款结构化字段与正文 | 条款视图 | VALIDATION | 幂等键 | clm.contract.update |
| PUT /api/v1/clm/contracts/{id}/milestones | 交付节点清单全量替换 | 节点列表 | VALIDATION | 幂等键 | clm.contract.update |
| PUT /api/v1/clm/contracts/{id}/payment-schedules | 期次列表全量替换 | 期次列表 | CLM.CONTRACT.PAYMENT_SCHEDULE_SUM_MISMATCH | 幂等键 | clm.contract.update |
| POST /api/v1/clm/contracts/{id}/attachments | 附件对象 id 与用途 | 关联列表 | VALIDATION | 幂等键 | clm.contract.update |
| POST /api/v1/clm/contracts/{id}/annotations | 批注锚点与正文 | 批注视图 | VALIDATION | 幂等键 | clm.contract.annotate |
| POST /api/v1/clm/contracts/{id}/actions/apply-template | 模板 id 与版本号 | 套用后的条款与节点 | CLM.CONTRACT_TEMPLATE.VERSION_NOT_PUBLISHED | 幂等键 | clm.contract.update |
| POST /api/v1/clm/contracts/{id}/actions/submit-for-approval | row_version | 校验明细与审批实例 id | CLM.CONTRACT.THREE_INFO_INCOMPLETE、CLM.CONTRACT.LINE_DELIVERY_DATE_OUT_OF_RANGE、CLM.CONTRACT.CUSTOMER_INACTIVE、CLM.CONTRACT.PRODUCT_INACTIVE、SALES.SALES_ORDER.CREDIT_LIMIT_EXCEEDED、CLM.CONTRACT.INVALID_STATE_TRANSITION | 幂等键；重复提交返回首次校验结论 | clm.contract.submit |
| POST /api/v1/clm/contracts/{id}/actions/void | 原因 | 状态视图 | CLM.CONTRACT.INVALID_STATE_TRANSITION | 幂等键 | clm.contract.void |
| POST /api/v1/clm/contracts/{id}/actions/retry-signature | 无 | 签署请求视图 | CLM.SIGNATURE_REQUEST.EXTERNAL_UNAVAILABLE | 幂等键 | clm.contract.sign |
| POST /api/v1/clm/contracts/{id}/actions/register-seal-usage | 印章名、用印时间、扫描件附件 id | 用印记录 | CLM.SEAL_USAGE.SCAN_REQUIRED | 幂等键 | clm.contract.seal |
| POST /api/v1/clm/contracts/{id}/actions/reject-signature | 原因 | 状态视图 | CLM.CONTRACT.INVALID_STATE_TRANSITION | 幂等键 | clm.contract.sign |
| POST /api/v1/clm/contracts/{id}/actions/make-effective | 必带 X-Reauth-Token | 状态视图与派生批次 id | CLM.CONTRACT.SIGNATURE_NOT_COMPLETED、PLATFORM.REAUTH.TOKEN_REQUIRED、PLATFORM.AUTHZ.SELF_APPROVAL_FORBIDDEN | 幂等键；重放不重复触发派生 | clm.contract.make_effective |
| POST /api/v1/clm/contracts/{id}/actions/amend | 变更原因 | 新版本草稿视图 | CLM.CONTRACT.AMEND_ON_NON_EFFECTIVE、CLM.CONTRACT.DERIVATION_IN_PROGRESS | 幂等键 | clm.contract.amend |
| POST /api/v1/clm/contracts/{id}/actions/renew | 新有效期与可调整字段 | 续签合同草稿视图 | CLM.CONTRACT.RENEW_SOURCE_NOT_ELIGIBLE | 幂等键 | clm.contract.renew |
| POST /api/v1/clm/contracts/actions/merge | 来源合同 id 列表与新合同头 | 新合同草稿视图 | CLM.CONTRACT.MERGE_SOURCE_NOT_ELIGIBLE、CLM.CONTRACT.MERGE_CUSTOMER_MISMATCH | 幂等键 | clm.contract.merge |
| POST /api/v1/clm/contracts/{id}/actions/terminate | 终止原因 | 状态视图与已派生单据处置清单 | CLM.CONTRACT.INVALID_STATE_TRANSITION | 幂等键 | clm.contract.terminate |
| GET /api/v1/clm/contracts/{id}/versions | 无 | 版本列表与差异 | 无 | 无 | clm.contract.read |
| GET /api/v1/clm/contracts/{id}/derivations | 无 | 派生批次与逐项状态 | 无 | 无 | clm.contract.read |
| GET /api/v1/clm/contracts/{id}/validations | 无 | 校验运行与逐项快照 | 无 | 无 | clm.contract.read |
| GET /api/v1/clm/contracts/{id}/performance | 无 | 履约记录投影，含交付节点进度、收付款期次进度、派生订单交付进度、关联退货换货与工单 | 无 | 无 | clm.contract.read |
| GET、POST、PATCH /api/v1/clm/contract-templates 与 /api/v1/clm/clauses | 档案维护 | 档案视图 | VALIDATION | 幂等键 | clm.template.manage |

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
| POST /api/v1/sales/delivery-confirmations | 交付确认单头与行，行按分批交付行选取 | 交付确认单视图，status 为 DRAFT | VALIDATION、SALES.DELIVERY_CONFIRMATION.QTY_EXCEEDS_SCHEDULED | 幂等键 | sales.delivery.create |
| POST /api/v1/sales/delivery-confirmations/{id}/actions/confirm-delivery | row_version；本端点按第 11.5 小节随第三批与阶段 10 同批注册 | 状态视图，含 voucher_id 与逐行 cogs_amount | SALES.DELIVERY_CONFIRMATION.INVALID_STATE_TRANSITION、SALES.DELIVERY_CONFIRMATION.QTY_EXCEEDS_SCHEDULED；三腿透传的错误按其所属模块的错误码原样返回，不在本模块重新编码 | 幂等键；重放不重复过账也不重复发事件 | sales.delivery.confirm |
| GET /api/v1/sales/delivery-confirmations | 排序白名单 posting_date、created_at、doc_no；过滤 status、customer_id、sales_order_id、posting_date | 分页列表 | 无 | 无 | sales.delivery.read |
| GET /api/v1/sales/delivery-confirmations/{id} | 无 | 交付确认单头行与三腿回填结果 | PLATFORM.AUTHZ.NOT_FOUND_OR_DENIED | 无 | sales.delivery.read |
| POST /api/v1/sales/sales-returns | 退货单头行与交付确认关联 | 退货单草稿 | SALES.SALES_RETURN.DELIVERY_LINK_REQUIRED、SALES.SALES_RETURN.QTY_EXCEEDS_DELIVERED | 幂等键 | sales.return.create |
| POST /api/v1/sales/sales-returns/{id}/actions/submit | row_version | 状态视图与审批实例 id | SALES.SALES_RETURN.INVOICE_NOT_CREDIT_NOTED | 幂等键 | sales.return.submit |
| POST /api/v1/sales/sales-returns/{id}/actions/register | 记账日期 | 状态视图 | SALES.SALES_RETURN.INVOICE_NOT_CREDIT_NOTED、SALES.SALES_RETURN.QTY_EXCEEDS_DELIVERED | 幂等键；重放不重复发事件 | sales.return.register |
| POST /api/v1/sales/sales-returns/{id}/actions/cancel 与 /actions/close | 原因 | 状态视图 | 状态机错误 | 幂等键 | sales.return.manage |
| POST /api/v1/sales/sales-returns/{id}/actions/link-exchange | 替换发货的分批交付行 id | 换货关联视图 | VALIDATION | 幂等键 | sales.return.manage |
| GET /api/v1/sales/credit-exposures | 查询参数 customer_id | 信用额度、三部分占用明细、可用额度 | PLATFORM.AUTHZ.NOT_FOUND_OR_DENIED | 无 | sales.credit.read |
| GET、PUT /api/v1/sales/credit-policies | 法人级策略 | 策略视图 | VALIDATION | 幂等键 | sales.credit.manage |

#### 5.3 CPQ 端点

| 方法与路径 | 说明 |
|---|---|
| GET、POST、PATCH /api/v1/cpq/price-authorities | 价格权限档案的维护，排序白名单 code、created_at，过滤 subject_kind、is_active |

#### 5.4 integration-gateway 内部端点

`POST /internal/v1/esign/requests` 与 `GET /internal/v1/esign/requests/{external_request_id}`，只监听 127.0.0.1:8082，不进入 `/api/v1` 命名空间，不对四端暴露，调用方只有 job-worker。

#### 5.5 版本化

本阶段全部端点为 v1 首次发布。后续新增可选请求字段、新增响应字段与新增枚举取值的接收侧不升主版本；客户端必须容忍未知的 `order_type`、`status` 与 `pending_release_reason` 取值并按未知降级展示。

---

### 6. 并发与事务边界

#### 6.1 事务清单

| 用例 | 事务内容 | 隔离级别 | 锁策略 |
|---|---|---|---|
| 合同草稿保存与修改 | 合同头行条款节点期次的写入、审计 | READ COMMITTED | 乐观锁 row_version |
| 合同提交审批 | 五项校验取数、校验记录写入、状态迁移、审批实例建立、审计、Outbox | READ COMMITTED | `customer_credit_controls` 行 FOR UPDATE |
| 合同生效 | 重新认证与审批结论校验、状态迁移、版本快照、审计、Outbox | READ COMMITTED | 合同行乐观锁 |
| 派生批次建立 | 批次行与全部派生项的写入 | READ COMMITTED | 批次唯一约束 |
| 单个派生项执行 | 调用目标模块用例、写回 target_doc_id、审计 | READ COMMITTED | 派生项行 FOR UPDATE SKIP LOCKED |
| 销售订单放行 | 后四项校验、状态迁移、审计、Outbox | READ COMMITTED | `customer_credit_controls` 行 FOR UPDATE |
| 订单变更审批通过 | 旧版本快照、变更应用、版本号递增、open_amount 重算、重跑校验、审计、Outbox | READ COMMITTED | 订单与订单行乐观锁 |
| 分批交付行拆分与合并 | 分批行全量重写、守恒校验、审计 | READ COMMITTED | 订单行 FOR UPDATE |
| 交付确认单登记 | 交付确认单头行写入、对分批交付行未交付量的校验、审计 | READ COMMITTED | 分批交付行 FOR UPDATE |
| 交付确认过账 | 会计期间解析、库存腿、过渡科目腿、凭证腿、单据置 CONFIRMED 与三处回填、审计、Outbox | READ COMMITTED | 交付确认单行与分批交付行 FOR UPDATE |
| 交付确认事件消费 | inbox 去重行、分批行与订单行数量回写、订单与合同状态推进、交付节点确认、审计 | READ COMMITTED | 订单行与分批行 FOR UPDATE |
| 销售退货登记 | 退货单状态、订单行 returned_quantity、审计、Outbox | READ COMMITTED | 订单行 FOR UPDATE |
| 合同合并 | 新合同建立、来源合同置 VOID、merge_links、审计 | READ COMMITTED | 来源合同行 FOR UPDATE |

内部对账与关账前强制校验涉及本阶段数据时，由阶段 9b 注册的校验项按基线第 8.4 节在单个 REPEATABLE READ 事务或由其导出的快照上执行，本阶段按裁定 A-06 不实现也不注册任何 `ReconCheck`。

事务预算按基线第 10.3 节：业务事务不超过 5 秒，读写池 `statement_timeout` 10 秒，`lock_timeout` 3 秒，`idle_in_transaction_session_timeout` 15 秒。事务内禁止外部 HTTP 调用、附件正文读写、发送通知与长时计算，因此签署提交、模板渲染与站内通知一律经 Outbox 转出事务之外。

#### 6.2 幂等键

- 全部写端点必须带 `Idempotency-Key`，作用域为法人、用户、端点、键值四元组，存储在 `platform_msg.idempotency_keys`，与业务写入同事务。
- 派生项的幂等键取派生项自身 id，同时由 `ux_contract_derivation_items_unique` 提供第二道保证。
- Outbox 事件的消费幂等由 `platform_msg.inbox_consumptions(consumer, event_id)` 保证，本阶段的消费者名固定为 `clm.derivation`、`clm.milestone_confirm`、`sales.delivery_writeback`。
- 电子签章提交的幂等键取 `signature_requests.id`，随请求传给外部系统，避免重复签署。

#### 6.3 与 Outbox 的关系

本阶段发出的领域事件全部与业务状态、审计事件在同一数据库事务内写入 `platform_msg.outbox_events`。事件信封字段按基线第 6.1 节完整填写，其中 `security_level` 与 `data_scope_tags` 自源记录继承，`posting_date` 在交付确认与销售退货登记两类事件上非空，分别取交付确认单与退货单的 `posting_date`，`accounting_period_id` 取 PostingPort 返回值；合同与订单类事件不产生凭证，`posting_date` 与 `accounting_period_id` 为空，这与基线第 6.1 节对可过账事件的要求不冲突，因为该两项是关账受理前提的可枚举依据，只对会产生凭证的事件有意义。

按裁定 A-21，`sales.delivery.confirmed.v1` 与 `sales.sales_return.registered.v1` 两条在 `ledger.posting_trigger_event_types` 中的登记行由阶段 9a 的种子迁移一次写入，本阶段不新增任何 `backfill_posting_trigger_event_types` 迁移，也不做启动自检、`--check` 静态断言与关账受理前置校验；登记表一致性的承接方只有两条，一是 `xtask configdoc` 从 `docs/event-catalog.md` 生成阶段 9a 的第 14 号种子迁移并在 CI 中与仓库文件逐字比对，二是阶段 3b 的 `event-catalog-consistent` 自检项且不通过时停止派发未登记事件类型；本阶段既不回填也不判读该表。登记行与上述 `posting_date` 非空两者齐备，这两类事件才按裁定 C-28 的受理前提二计入待过账积压。

本阶段的事件总数固定为 18，第 1 节与第 9 节的计数只引用本小节，不另写数字。其中九个的事件名由本计划与裁定固定，逐条如下；其余九个是合同与销售订单状态机的迁移事件，名称按基线第 6.1 节的四段式在实现前先登记入 `docs/event-catalog.md`。

| 事件 | aggregate_type | 产生位 | posting_date |
|---|---|---|---|
| clm.contract.effective.v1 | clm.contracts | 第 4.8 小节的生效事务 | 空 |
| clm.contract.derivation_completed.v1 | clm.contracts | 第 4.8 小节派生编排第 6 步 | 空 |
| clm.contract.signature_requested.v1 | clm.contracts | 第 4.13 小节第 1 步 | 空 |
| clm.contract.signed.v1 | clm.contracts | 第 4.13 小节第 5 步 | 空 |
| sales.delivery.confirmed.v1 | sales.delivery_confirmations | 第 4.11 小节的 confirm_delivery 事务 | 非空，取交付确认单的 posting_date |
| sales.sales_return.registered.v1 | sales.sales_returns | 第 4.12 小节的登记动作 | 非空，取退货单的 posting_date |
| sales.sales_return.closed.v1 | sales.sales_returns | 第 4.12 小节 REGISTERED 迁到 CLOSED | 空 |
| sales.sales_return.cancelled.v1 | sales.sales_returns | 第 4.12 小节任一状态迁到 CANCELLED | 空 |
| sales.sales_return.rejected.v1 | sales.sales_returns | 第 4.12 小节 SUBMITTED 因审批驳回退回 DRAFT | 空 |

后三个事件的 payload 字段按裁定 A-17 固定，见第 4.12 小节。

#### 6.4 失败重试与补偿

- 数据库序列化失败 40001 与死锁 40P01 由数据访问层统一重试 3 次，退避 50、150、450 毫秒，只对尚未产生外部可见副作用的事务重试。
- 派生项失败按八档退避重试，耗尽进入死信，人工修复后可重放。
- 派生批次不做整批回滚。理由是已成功派生的销售订单可能已被下游引用，回滚会造成比部分派生更严重的不一致；补偿方式是把失败项修复后重放，或由人工在合同上执行终止并按第 11.3 小节的处置清单逐项处理已派生单据。
- 电子签章失败按超时、退避、熔断三级处理，熔断打开期间新的签署请求直接进入 Outbox 等待，不占用连接。
- 交付确认事件消费失败时不吞掉异常，按至少一次语义重投；因订单已取消等业务原因无法回写的，写入 `platform_msg.dead_letters` 并在运维中心可枚举，由人工修复后重投，不静默忽略；本阶段不产生对账差异事项，理由见第 6.1 小节。

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
| EP__CLM__ESIGN__CREDENTIAL_REF | string | secret://esign/api#1 | 启动加载；机密版本变更在下次取用时热生效，不需重启 |
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

- 合同状态机：九个状态、二十条合法迁移逐条一个用例；非法迁移集合用参数化用例穷举并断言返回 `CLM.CONTRACT.INVALID_STATE_TRANSITION`。
- 订单状态机与分批交付行状态机同上。
- 退货单状态机同上。
- 取价与行金额：含税与不含税两种录入口径、折扣为零与非零、价目未命中、多行命中、税率为零与 13%、数量与单价均取六位小数时的舍入；期望值在测试中写死为字面量，不由被测代码反算。
- 价格权限判定：五种命中组合与三级取用顺序。
- 五项校验：合同校验的四个子项各一个失败用例；库存与交期的失败传导；信用三桶的六种迁移时点。
- 信用判定：额度为空的三种 `null_limit_behavior`、`amount_basis` 两种取值、`on_exceed` 两条路径、超出金额的计算。
- 分批交付行拆分：合计相等、合计不等、已交付行不可拆、合并后的批次号重排。
- 派生项生成规则：五类派生物在直运、需立项、订阅、寄售四种合同形态下的项数与来源引用。
- 收付款期次校验：比例合计等于 1、金额合计等于合同金额、两种基准混用被拒。
- 交付确认：本次数量超过分批交付行未交付量被拒、直运单跳过库存腿、非直运单的四步次序、任一腿失败整笔回滚，四条各一个用例，三腿以记录型桩断言入参与调用次序。

#### 8.2 领域属性测试

用 proptest 覆盖五组不变量，对应规格第 17.3 章可归属本阶段的判据。

1. 分批交付数量守恒：任意拆分与合并序列后，同一订单行的全部分批行数量合计恒等于订单行数量。
2. 信用三桶不重叠：对任意的下单、交付、开票、到款、退货事件序列，同一订单行的金额在任一时点只落在三部分中的一部分，三部分之和不超过该订单行的含税金额。
3. 金额舍入一致：合同头金额恒等于合同行金额按 2 位累加的结果，订单头金额恒等于订单行金额按 2 位累加的结果，任意数量与单价组合下成立。
4. 派生幂等：对任意的重复投递序列，同一 `(contract_id, contract_version_no, trigger, artifact_kind, source_ref_id)` 只产生一个目标单据。
5. 退货数量守恒：任意退货序列后，`returned_quantity` 恒不超过 `delivered_quantity`，且 `delivered_quantity` 恒不超过 `quantity`。

#### 8.3 集成测试

使用真实 PostgreSQL 16，每个用例独占一个 `ep_test_<nanoid>` 库，结束即删库。禁止用内存库或 mock 替代数据库。外部电子签章用 wiremock 打桩，同时提供一套对真实沙箱执行的契约测试，后者在阶段 4 按附录 B 判定。

场景清单：

1. 合同建单到提交审批的完整路径，含五项校验记录与审计事件的写入核对。
2. 折扣超权限时折扣审批链被挂起，未超权限时不进入折扣节点，其余三条链照常执行。
3. 管理层节点不可跳过：构造缺少管理层节点的审批链配置，验证在配置发布阶段即被拒绝。
4. 申请人不可自审：发起人尝试审批自己发起的合同被拒绝并给出冲突节点。
5. 合同生效缺少重新认证凭证被拒绝，凭证过期被拒绝，凭证绑定的待签内容摘要不匹配被拒绝。
6. 派生完整路径：五类派生项在同一批次内全部建立且与 `item_total` 一致；销售订单、收款计划与交付节点三类在本阶段真实生成对应单据与记录并双向可追溯可查；采购需求派生项 `status` 恒为 PENDING、`target_doc_id` 留空且不计入 `item_done`，其派发在阶段 7 接线后补做；项目任务派生项按裁定 C-19 只登记不派发，`status` 置 DONE 且 `target_doc_id` 留空。不含采购需求派生项的合同进入履约中，含该项的合同停在已生效。
7. 派生重复投递 3 次，派生单据只产生一次。
8. 派生失败进入死信，运维中心可枚举，人工修复后重放不产生重复单据。
9. 派生时信用不足使订单进入待放行，信用审批通过后转为已放行。
10. 派生时库存不足使订单进入待放行，库存恢复后重跑校验转为已放行。
11. 同一客户并发下单：两条并发的合同提交，验证串行化后总占用不超额，其中一条被阻断或转审批。
12. 订单变更提高金额时重跑信用与库存，降低金额时不重跑。
13. 订单变更审批通过与交付确认事件交叠，已交付数量不被覆盖。
14. 分批交付行拆分后信用占用总额不变。
15. 销售退货前置红冲校验：未红冲时阻断并列出待冲销发票，红冲后可登记。
16. 直运订单的销售退货不产生库存流水事件，事件载荷中 `is_drop_ship` 为真。
17. 电子签章超时、失败、熔断与恢复四条路径，以及验签失败时合同保持待签署。
18. 实体印章路径：用印登记后可执行生效动作，缺扫描件时被拒绝。
19. 合同合并：来源合同状态不合规被拒、客户不一致被拒、合并成功后来源置作废并保留关联。
20. 合同续签：续签版本与原合同双向可达，生效后派生新的订单、收款计划与交付节点，不重复派生原合同已有单据。
21. 法人越权测试集 `tests/rls_matrix` 的本阶段部分：对 `clm` 与 `sales` 两个 schema 的全部表覆盖读取、写入、更新、删除、聚合、排序、报表投影与错误信息泄漏八类，跨法人一律不可见且不泄露存在性。
22. 迁移的执行与回退：全部迁移在空库上执行成功，按各文件的 `-- rollback:` 段落回退后 schema 与本阶段之前一致。
23. 交付确认三腿同事务：非直运单一次确认后 `sales.delivery_confirmation_lines` 的 `cogs_amount` 与 `stock_movement_id` 已回填、`sales.delivery_confirmations.voucher_id` 已回填、`sales.delivery.confirmed.v1` 的信封带 `posting_date` 与 `accounting_period_id`，过渡科目腿在同一事务内产生 `finance.unbilled_ar_entries` 条目；本场景整条属第三批，随阶段 10 的 `UnbilledArPort` 同批执行，不含任何经替身实现的断言。
24. 交付确认的重复提交与重复消费：同一 `Idempotency-Key` 重放 3 次只产生一张交付确认单、一次三腿调用与一条事件；`sales.delivery_writeback` 重复消费同一事件后 `delivered_quantity` 不重复累加。

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
| E2E-6-09 | 订单拆分为三条分批交付行并分批交付，订单状态由已放行经部分交付到已交付 | 规格第 8 章第 3 步与第 8 步的销售侧 |
| E2E-6-10 | 销售退货完整用例：已开票部分先红冲再退货，退货后信用占用相应释放 | 规格第 8 章第 11 步、第 17.2 章财务内核测试的销售退货基础分支的销售侧 |
| E2E-6-11 | 换货用例：一笔退货加一笔在原订单上放行的分批交付行，两者保留换货关联 | 规格第 8 章第 11 步 |
| E2E-6-12 | 电子签章端到端：签署发起、结果回传、验签、签章文件归入合同附件与审计 | 规格第 10.4 章连接器验收判据、第 19 章阶段 3 |
| E2E-6-13 | 交付确认完整用例：由分批交付行建交付确认单并确认过账，同一事务内四步依次成功，单据置已确认并回填 voucher_id 与逐行 cogs_amount，`sales.delivery_writeback` 与 `clm.milestone_confirm` 消费自身事件后分批交付行、订单行 `delivered_quantity` 与合同交付节点同步推进 | 规格第 8 章第 8 步 |

E2E-6-04、E2E-6-10 的账务侧判据由财务与库存阶段承接，本阶段只验证销售侧的单据、状态、占用与事件；两阶段合并执行时按规格第 17.2 章财务内核测试的对应条目判定差额为零。按第 11.5 小节，E2E-6-04、E2E-6-09 的交付段、E2E-6-10 与 E2E-6-13 四项整条属第三批，与阶段 10 的 finance 端口同批执行，四项都不含任何经替身实现的断言，也不再登记顺延项。E2E-6-01 与第 8.3 节场景 6 中采购需求派生物的端到端断言在阶段 7 接线后补做，项目任务派生物的端到端断言由阶段 12 的 `project.contract_derivation` 消费者承接，本阶段只断言这两类派生项行已建立、采购需求项为 PENDING、项目任务项为 DONE 且两者 `target_doc_id` 均留空。E2E-6-02 与 E2E-6-04 的三桶断言按 U-E-10 以规格第 5.2 章为准，不按规格第 17.2 章末段的两部分表述。

#### 8.5 性能相关项

本阶段涉及附录 A.1 度量清单中的七项：常规交互中的销售订单表单打开并带出默认值、库存可用量查询、审批任务列表加载；普通交易提交中的合同提交、合同审批提交、审批放行提交、销售订单提交、退货登记；常用报表中的销售订单履约明细。合同生效派生按附录 A.1 为非交互观察项，只记录不设通过线。

本阶段的性能要求为：在附录 A.3 基准数据集上，上述查询的 `EXPLAIN` 输出中不得出现顺序扫描，逐条附执行计划证据。具体涉及的索引为 `ix_contracts_legal_entity_id_customer_id_status`、`ix_sales_order_lines_legal_entity_id_customer_id_status`、`ix_delivery_schedules_legal_entity_id_promised_date_status`、`ix_contract_milestones_legal_entity_id_promised_date_status`。时延通过线在阶段 4 统一判定，本阶段不冻结取值。

#### 8.6 覆盖率门槛

- 信用占用计算、分批交付数量守恒、派生幂等三处属规格第 17.3 章强制不变量相关代码，行覆盖率不低于 85%。
- 本阶段其余代码行覆盖率不低于 70%，新增与修改代码不低于 80%。
- 工作区整体行覆盖率不低于 80%。
- 工具为 cargo-llvm-cov，阈值由 `codecov.toml` 中与 crate 清单一一对应的路径规则表达，CI 上以 `--fail-under-lines` 强制。
- `#[ignore]` 必须带 issue 编号注释且存活不超过一个阶段。

---

### 9. 退出条件

下列条目全部达成才算本阶段完成，每条均可客观判定。

1. 第 1 节的十一项交付物全部存在，`cargo build --workspace --release` 与 `cargo clippy --workspace -- -D warnings` 通过；`apps/core-server/src/wiring/` 与 `apps/job-worker/src/wiring/` 两个目录下的全部文件中不出现任何 `Noop`、`Stub`、`Fake`、`Dummy` 前缀的注入行，本阶段不产生任何空实现，该口径与技术基线第 10.4 节一致，判据提供方是阶段 1 随 `xtask` 交付的 archcheck 规则 `unwired-absent`；第 11.5 小节第三批的退出条目与阶段 10 的 finance 端口同批判定，其余条目在第二批结束时判定。
2. 三个迁移目录的全部迁移在空库上按文件版本号全序执行成功，且各文件的回退说明经一次实际回退验证；按裁定通则第五条本阶段不新增任何跨 schema 迁移，`ledger.posting_trigger_event_types` 的两行登记由阶段 9a 的种子迁移写入，本阶段既不回填也不判读该表。
3. `apps/core-server --check` 与 `apps/job-worker --check` 在基线第 7.3 节十项中的九项上全部通过并输出结构化报告，本阶段不追加任何启动自检项；`offsite-sink-requirements` 一项按阶段 1 计划整条推迟到阶段 14，本阶段返回 `NOT_APPLICABLE` 并在报告中标注承担阶段，不计入本条的通过项，该处置按基线第 12 节通则第六条取换判据一档；本模块的 18 个事件与 `docs/event-catalog.md` 经 `xtask configdoc` 逐字比对通过。
4. 基线第 1.3 节的依赖方向自检脚本对本阶段新增 crate 全部通过，`ep-domain-clm` 与 `ep-domain-sales` 中无 sqlx、reqwest、文件与网络符号。
5. 第 8.1 至 8.3 节的全部单元、属性与集成测试通过，集成测试跑在真实 PostgreSQL 16 上。
6. 第 8.4 节的十三个 E2E 场景在 Windows 与 macOS 两端全部通过，在 iOS 与 Android 两端按简化取值通过，合同生效的重新认证在四端一致。
7. `tests/rls_matrix` 的本阶段部分八类越权测试全部通过，跨法人零泄漏。
8. 第 8.6 节的三档覆盖率门槛全部达标。
9. 第 8.5 节涉及的四条索引在基准数据集上的 `EXPLAIN` 证据已归档，无顺序扫描。
10. 本阶段的 18 个事件已登记在 `docs/event-catalog.md`，与第 6.3 小节事件登记表列出的九个逐字一致；本阶段第 5 节 API 契约表中出现的全部错误码已登记在 `docs/error-codes.md` 并与 `ep-foundation::error::codes` 一致，由 CI 校验通过。
11. 本阶段新增的六个指标已在 ops-agent 的 127.0.0.1:9101 上可抓取，标签基数符合基线第 9.2 节的纪律。
12. 合同生效、订单放行、退货登记三类操作的审计事件已进入按法人与自然日的哈希链，审计链验证工具在本阶段的用例数据上通过。
13. 第 11.2 小节的偏离项已提出对应的基线修订并被整合员接受，第 11.3 小节的新增决定已回写基线。
14. 派生失败到死信、人工修复、重放不产生重复单据的完整链路已在运维中心可见并演示通过。
15. 本模块的四个受治理数据集视图 `clm.v_contracts_dataset`、`clm.v_contract_delivery_milestones`、`sales.v_sales_orders_dataset`、`sales.v_order_delivery_batches` 已发布并授予 `ep_analyst_ro`，每个视图含 `legal_entity_id`、`security_level`、`data_scope_tags` 三列，`ep_app_rw` 之外无任何写权限，列签名已同步给阶段 11 且与 `reporting.dataset_fields` 的登记一致。
16. clm 与 sales 两个模块在规格第 6.2 章能力矩阵中取值为完整或简化的能力域，其四端界面已实现并通过 Playwright 与 tauri-driver 的桌面用例、XCUITest 与 Espresso 的移动用例；取值为 VIEW_ONLY 的能力域只实现只读视图；取值为 NOT_APPLICABLE 的不实现入口。
17. 本阶段全部路由的能力域码与动作类别常量已在 `crates/contract/clm/src/capability.rs`、`crates/contract/sales/src/capability.rs` 与阶段 5 已建的 `crates/contract/cpq/src/capability.rs` 声明，`xtask configdoc` 通过。
18. `ClmProductUsageProbe` 与 `SalesProductUsageProbe` 已实现并注入阶段 5 提供的 `AnyProductUsageProbe`，阶段 5 的启动自检项 `master-data-usage-probes-registered` 在 clm 与 sales 启用时通过；本模块的 `ClmReferenceCounter` 与 `SalesReferenceCounter` 已注册到 `MasterReferenceCounterRegistry`，`SalesTradeHistoryProviderImpl` 已注册到 `TradeHistoryProviderRegistry`。
19. 四个单据类型码 CT、SO、SR、DC 已登记入 `docs/data-dictionary.md` 的单据类型码一节与 `ep-platform-sequence` 的常量表，`xtask configdoc --check-doc-type-codes` 通过。
20. 规格第 21.4 章要求的专业签字已取得并留档：法务在本阶段签字，签字人资格证据随版本留档；签字缺失或不通过时本阶段不得退出，整改后重新测试并重新签字，不得以未记录的方式豁免（规格第 22 章第 12 条）。本条由裁定 F-42 新增，此前四份计划的退出条件中无任何签字项。

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
| 第 17.2 章 | 客户信用额度校验判据，该章末段只列应收未收与在途订单两部分，与第 5.2 章的三部分不一致，已按 U-E-10 登记为待决，本阶段以第 5.2 章为准；四端端到端测试；集成与契约测试中的电子签章连接器用例 |
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
| 3.6 | 合同状态机九状态与全部流转、有效期止日不改变状态、合同版本与修订规则 |
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
| 无承载节 | 交付确认功能在 PRD 第 3 节与第 5 节均无小节，属附录乙 U-C-01，本阶段依据规格第 8 章第 8 步与第 5.2 章事件-分录表实现，临时取值与切换代价见第 11.3 小节 |

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
| PRD 附录乙 U-E 组共十六条未决事项，来源节均为 PRD 第 3 节，全部落在本阶段 | 结论落定后可能需要改数据与改校验 | 逐条临时取值见第 11.3 小节，取值集中在 `sales.credit_policies`、`sales.customer_credit_controls` 与 `cpq.price_authorities` 三张可配置表，切换代价限于改配置与一次数据回填，不改表结构；U-E-10 是唯一例外，其反向裁定的代价见第 11.3 小节该行 |

#### 11.2 对基线的偏离项

1. `sales.sales_returns` 与 `sales.delivery_confirmations` 只带 `posting_date`，不带 `accounting_period_id`。基线第 4 节要求会计相关表两者兼具。偏离理由是规格第 5.2 章总账功能与期末处理块规定凭证的会计期间字段是判定期间归属的唯一依据，业务单据持有该列会产生第二处判定点，并在顺延入账时与凭证不一致；会计期间在 `confirm_delivery` 事务内由 `AccountingPeriodResolver::resolve` 解析一次并随事件信封传出，不落单据列。影响范围为本阶段仅有的两张会计相关表。建议基线第 4 节把该规则收窄为凭证与子账条目带 `accounting_period_id`，业务事件登记单据只带 `posting_date`。
2. 业务表上使用 `customer_id` 列。基线第 4 节写明不设 `customer_id` 列，其理由段落指向规格第 7.1 章的每客户一实例，可见该处的客户指承租的企业客户，即租户。本阶段的 `customer_id` 是业务客户档案的逻辑外键，不承担隔离职责。建议基线第 4 节把该条改写为不设租户隔离列，以免与业务客户字段混淆。
3. `sales.order_validations` 与 `clm.contract_validations` 两处同构表。基线第 12 节禁止引入第二套机制。本处不是两套机制而是同一模型在两个模块内的本地存储，理由是基线第 1.3 节禁止跨模块直接读写业务表，订单侧的重跑发生在 `ep-app-sales` 事务内，无法写入 clm 的表。两表的列、枚举与序列化类型由 `ep-foundation` 中的同一类型导出，CI 校验两处 DDL 的列集合一致。
4. 电子签章不设公网入站回调，改由 integration-gateway 按退避轮询拉取签署状态。规格第 10.4 章只要求回传签署结果，未规定方向。偏离理由是首版公网侧只有供应商门户一个站点，新增入站入口会扩大规格第 21.17 章的暴露面，且单机形态下入站回调在停机窗口内会丢失。代价是签署结果的可见延迟上限等于一个轮询间隔。该决定另出一份 ADR。
5. `cpq.price_authorities` 建在 cpq schema。若整合员判定 cpq schema 的建设整体归主数据阶段，则本表连同其迁移文件移交该阶段，本阶段改为只消费 `ep-contract-cpq::PriceAuthorityPort`，其余内容不变。

#### 11.3 本阶段新增决定与临时取值

下列取值中标注编号的对应 PRD 附录乙的未决事项，本阶段给出临时取值并说明切换代价；未标注编号的是基线与 PRD 均未覆盖、由本阶段新增的决定，需回写基线。

| 事项 | 临时取值或新增决定 | 是否阻塞本阶段 | 切换代价 |
|---|---|---|---|
| U-E-01 信用额度维护范围 | 按客户加法人分别设定 | 否 | 改为按客户全局时需改额度取数与三桶聚合的法人范围，涉及跨法人查询按基线第 3.8 节逐法人设置会话变量后合并 |
| U-E-02 额度为空的默认行为 | `null_limit_behavior` 默认 `TREAT_AS_ZERO`，配合 `on_exceed` 默认 `REVIEW`，使新客户首单转审批而非被阻断 | 否 | 改配置即可，无需改数 |
| U-E-03 三部分的价税口径 | 统一取含税口径，`amount_basis` 默认 `WITH_TAX` | 否 | 改为不含税时需同步改财务侧端口的取数口径，两侧必须同时切换 |
| U-E-04 预收是否抵减占用 | 不抵减，`deduct_advance_receipts` 默认 false | 否 | 改配置并要求财务侧端口追加预收余额返回项 |
| U-E-05 超额处置的配置粒度 | 法人级默认加客户级覆盖两层，出厂默认 `REVIEW` | 否 | 改为系统级时删除客户级覆盖列 |
| U-E-06 订单变更是否重跑信用 | 重跑，`recheck_on_order_change` 默认 true，只在提高金额或提前交期时触发 | 否 | 改配置即可 |
| U-E-07 转审批的审批人角色与层级 | 信用超额转审批走 `clm.contract_approvals` 中 `chain_kind = CREDIT` 的一条链，出厂审批人取销售负责人，不强制管理层节点；派生出的订单侧按第 4.8 小节第 5 步在 `on_exceed` 取 REVIEW 时追加同一链 | 否 | 改为必须管理层审批时只改该审批链定义中的节点角色，不改表结构与状态机，须与 U-A-08 同批关闭 |
| U-E-08 库存可用量与交期不通过的处置 | 建单提交时不阻断，只记录并使派生出的订单进入待放行。理由是规格第 8 章第 3 步明确规定库存可用量不足的派生单据置为待放行，若建单时直接阻断则该条路径永不可达 | 否 | 改为阻断时只需把校验项的处置由 FLAGGED 改为 BLOCKED |
| U-E-09 合同校验的具体内容 | 按 PRD 3.3.3 的四项拟定实现，见第 4.6 小节 | 否 | 追加或删除子项即可 |
| U-E-10 规格第 17.2 章与第 5.2 章的信用额度判据不一致 | 本阶段按规格第 5.2 章客户信用额度校验条目的三部分实现，即应收未收、已交付未开票与在途订单，不按第 17.2 章末段的两部分表述；理由是第 17.2 章末段自身写明按第 5.2 章客户信用额度校验条目的口径，第 5.2 章是取值出处，第 17.2 章末段属复述遗漏 | 否 | 结论落定后回写规格第 17.2 章判据，决策人为财务负责人；若反向裁定为两部分，须删去 `CreditExposureView` 的 `delivered_unbilled_amount` 一项、改第 4.7 小节的求和、改 E2E-6-02 与 E2E-6-04 的明细断言并撤销 `ReceivableExposureQuery` 的该返回项，代价触及契约、实现与用例三层 |
| U-E-11 模板版本升级的影响范围 | 已套用旧版本的草稿、在审与已生效合同一律不受影响，模板版本号在合同上固化 | 否 | 改为影响草稿时需追加一次批量重套用的批处理用例 |
| U-E-12 合同提前终止与撤回 | 实现提前终止动作，仅允许自 IN_PERFORMANCE 发起，须经审批；终止时生成已派生单据的处置清单，尚未交付的订单行置关闭、尚未开票的收款计划期次置作废、尚未确认的交付节点置取消，已发生的一律不回退。在审撤回按平台审批链的撤回能力承载，不另设动作 | 否 | 处置策略改变时只改处置清单的生成规则 |
| U-E-13 拆分粒度与变更字段清单 | 单订单行的分批交付行上限 60 条；已部分交付订单允许变更的字段为未交付部分的数量、交期与仓库，禁止变更单价与物料；关闭剩余数量须填原因并经销售负责人审批 | 否 | 改上限为配置项调整；改字段清单需同步改守卫条件 |
| U-E-14 退货单是否独立审批 | 设独立审批链 `sales.return`，退货原因为受控取值列表，出厂取值为质量问题、发错货、客户取消、其他四项 | 否 | 取消审批链时把 SUBMITTED 状态直接映射到 REGISTERED |
| U-E-15 订阅与租赁字段清单 | 周期单位取 DAY、WEEK、MONTH、QUARTER、YEAR 五值，周期长度为正整数，租期起止为日期，自动续期为布尔 | 否 | 字段增删按基线第 3.9 节的在线新增可空列执行 |
| U-E-16 关键条款结构化字段清单 | 交付节点清单、质保条款、违约责任、争议解决方式、合同义务清单五项为结构化，其余进条款正文 | 否 | 结构化字段增补走 `structured jsonb` 或新增可空列 |
| U-A-01 单据编号 | 合同类型码 CT、销售订单 SO、销售退货 SR、交付确认单 DC，四码按裁定 C-26 登记在 `docs/data-dictionary.md` 的单据类型码一节，格式按基线第 11.1 节 | 否 | 类型码改动只改序列配置 |
| U-C-01 交付确认的承载节 | PRD 第 3 节与第 5 节均无该小节，本阶段依据规格第 8 章第 8 步与第 5.2 章事件-分录表先行实现，单据主体按裁定 A-09 归 sales 模块 | 否，PRD 侧的验收基准待该条关闭后补 | 若改判由 PRD 第 5 节承载即单据主体归库存模块，切换代价为 `sales.delivery_confirmations` 与 `sales.delivery_confirmation_lines` 两表跨 schema 迁移、`sales.return_line_delivery_links` 与 `sales.delivery_schedules` 两处真实外键退回逻辑引用、`sales.delivery.confirmed.v1` 的 aggregate_type 与 payload 改名、类型码 DC 改登记模块，以及阶段 8 与阶段 9a 与阶段 10 三条腿的调用方反转，属高代价 |
| U-C-02 交付确认的操作者角色 | 只冻结能力常量 sales.delivery.create、sales.delivery.confirm 与 sales.delivery.read，不预置角色绑定 | 否 | 决策后只增配置发布包中的角色能力映射行，不改代码与表 |
| 价格权限档案的承载 | 新增 `cpq.price_authorities`，三级取用顺序为 USER、POSITION、ROLE，三级均无命中时阻断提交。PRD 2.8.1 只给了价目行的价格下限，未给操作者权限侧取值，本阶段据规格第 5.2 章 CPQ 条目的价格权限校验要求补齐 | 否 | 若主数据阶段另行承载则本表移交 |
| 电子签章的兜底入口 | 保留人工上传已签署文件的入口，同样要求验签、附件归档与审计，用于外部系统长期不可用时闭环不中断 | 否 | 无 |
| 合同派生项数上限 | 2000，超出拒绝生效并提示拆分合同 | 否 | 调配置 |

#### 11.4 为后续阶段预留的扩展点

1. `clm.contract_lines.order_type` 的五取值枚举与 `sales.sales_orders.order_type` 同源，寄售在库台账与代销结算恢复时只需在该枚举上增加分支处理，不改表结构。
2. `clm.contract_derivation_items.artifact_kind` 是开放枚举，后续新增派生物类型只需追加取值与目标模块端口，派生编排本身不改。
3. `sales.credit_policies` 预留 `deduct_advance_receipts` 与 `amount_basis` 两个开关，信用评级模型与账期分级策略恢复时可在该表上扩展策略维度。
4. `ep-contract-clm::ContractQueryPort` 已把合同头、行、条款、交付节点四类投影分开暴露，客户 360 视图、经营驾驶舱的合同维度下钻与全文检索索引可直接消费，不需要为其新增读取通道。
5. `clm.signature_requests.provider_code` 预留多签章服务商并存，附录 B 的外部替换验收在阶段 4 只需新增一个 provider 实现并跑同一套契约测试。
6. `sales.delivery_schedules.promised_date` 是交付指标期间维度的唯一取数来源，`clm.contract_milestones.promised_date` 是合同交付节点侧的唯一取数来源，两者的字段命名与索引已按规格第 5.5 章经营驾驶舱条目的下钻口径准备，报表阶段经第 3.3 与 3.4 小节的四个受治理数据集视图取用，不需要再建物化投影表。

#### 11.5 跨阶段调用点的接线次序

本阶段与后继阶段之间共五个跨阶段调用点，一律不使用空实现，也不设顺延验收台账。硬规则是跨模块同步调用的被调方必须与调用方同批交付，做不到就把该调用连同其用例整条推迟到被调方所在阶段，两者之外不存在第三种形态；任何返回零值、空集合、固定业务分支或恒定成功的实现在本阶段一律禁止，`apps/core-server/src/wiring/` 与 `apps/job-worker/src/wiring/` 两个目录下的全部文件中不得出现 `Noop`、`Stub`、`Fake`、`Dummy` 前缀的注入行，测试装配中的记录型桩不受此限。下表是本阶段全部跨阶段调用点的唯一出处。

| 跨阶段调用点 | 契约方法 | 处置 | 接线时点与缺席时的数据表征 |
|---|---|---|---|
| 交付确认的过渡科目腿 | `ep_contract_finance::UnbilledArPort::record_on_delivery` | 同批交付 | 与阶段 10 该端口同批交付同批验收，三腿在本阶段第三批一次全真接线，该批次之外本阶段不建该调用点，`confirm_delivery` 用例与其端点不写入代码，不存在只落两腿的已确认交付 |
| 信用三桶中的已交付未开票与应收未收 | `ep_contract_finance::ReceivableExposureQuery::exposure` | 同批交付 | 按裁定 C-14 不进 T0 切片，与阶段 10 该端口同批交付同批验收，三桶取数在本阶段第三批当场成立，该批次之外本阶段不建该调用点，不存在只取两桶或取 `None` 的形态 |
| 销售退货的红冲前置判定 | `ep_contract_invoice::InvoiceReversalStatusQuery::is_fully_credit_noted` | 同批交付 | 按裁定 C-16 不进 T0 切片，与阶段 10 该 trait 同批交付同批验收，`SALES.SALES_RETURN.INVOICE_NOT_CREDIT_NOTED` 判定在本阶段第三批当场成立，该批次之外本阶段不建该调用点 |
| 合同派生的采购需求派发 | `ep_contract_procure::PurchaseRequisitionIntakePort::intake` | 整条推迟 | 推迟到阶段 7，本阶段不写调用点，派生项 `status` 恒为 PENDING、`target_doc_id` 留空、不计入 `item_done` |
| 直运退货的采购侧勾稽 | `ep_contract_procure::PurchaseReturnLinkPort::link_drop_ship_return` | 整条推迟 | 推迟到阶段 7，本阶段不写调用点，直运订单在阶段 7 之前无从交付，退货由 `SALES.SALES_RETURN.QTY_EXCEEDS_DELIVERED` 自然阻断 |

本表第三行为同批交付，阶段 10 交付该 trait 时与本阶段第三批一次接线，不做替换动作；阶段 10 端口表中先注入空实现再由本阶段替换的措辞已按总览第 1.5 节第八条整段撤销，本阶段不承接任何替换动作。已退货未冲回成本的置位方按 PRD 附录乙 U-C-09 待决，阶段 11 只交付 `CostReturnMarkPort` 的实现与注册、不指名调用方，本阶段不涉及。项目任务派生按裁定 C-19 只登记不派发，不进本表，其端到端断言由阶段 12 承接，出处在第 8.4 小节。
