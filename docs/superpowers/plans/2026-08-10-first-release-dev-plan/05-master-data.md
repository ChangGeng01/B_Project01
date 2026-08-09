## 阶段 5 主数据

本阶段交付客户、供应商、物料、产品四类档案，与之绑定的价目表、历史成交资料带出、档案编码规则的使用侧、启用停用、批量导入导出，以及客户 360 视图的数据基础。本阶段不产生任何会计凭证、不写库存数量账与库存金额账、不触发任何账务事件，规格第 5.2 章财务规则条目的事件-分录表在本阶段没有适用条目，本阶段只保证被这些规则用作分组键的物料、产品、仓库以外的档案标识稳定且冻结规则可执行。

本阶段严格照抄共享技术基线的命名、类型、封套、分页、幂等、错误分类与依赖方向。基线未覆盖而本阶段必须取值的事项集中在第 12 节与第 13 节，正文中逐处标注。

### 0 边界与本阶段不做的事

本阶段承载的范围，来自 PRD 第 2 节全节，以及 PRD 第 1.4 节闭环第 1 步中属于带出的部分。

本阶段明确不承载的内容，逐条列出，避免与其他阶段重叠。

- 会计科目表由 ledger 模块维护，设备台账由 service 模块维护，两者不进入主数据治理，本阶段不建表也不建接口，依据是规格第 7.2 章与 PRD 第 2.1.1 节。
- 仓库档案属 inventory 模块，本阶段不建仓库表，依据是基线第 1.2 节模块表 inventory 行。
- 地点主数据在 PRD 未定义维护操作，且首版唯一的实际用途是仓库归属，本阶段不建地点表，登记为 U-C-05 与 U-C-06 的待决影响面。
- 组织、法人、部门、岗位、用户账号属 platform 侧，本阶段只读引用：集团与组织架构取 platform_core.enterprise_groups、platform_core.organizations、platform_core.departments、platform_core.positions 与 platform_core.department_closures 五张表，法人清单经 ep-platform-tenancy 的 LegalEntityDirectory::list_active 读取，部门子树经 DepartmentClosureQuery::descendant_ids 读取，五张表与两个契约由阶段 2 按裁定 A-04 交付。
- 价格权限校验、折扣与折扣审批属销售阶段，本阶段只交付价目表本体与取价命中，并对外暴露取价端口。
- 供应商门户的入口、会话与脱敏投影属门户阶段，本阶段只对外暴露供应商自维护变更申请的受控命令端口 ep_contract_mdm::SupplierSelfServiceCommand，名字按裁定 B-10 固定，不另起第二套措辞。
- 客户 360 视图中来自合同、订单、回款、投诉、设备与服务记录的区块，属各自模块阶段，本阶段只交付区块注册契约 ep_contract_crm::Customer360SectionProvider 与视图端点 GET /api/v1/crm/customers/{id}/customer-360 的骨架；端点与契约按裁定 C-09 与阶段 12 共用，阶段 12 只扩充区块不新增路径。
- 历史数据迁移导入走规格第 7.10 章的迁移接口，与本阶段的日常批量导入是两条互不混用的通道；本阶段只交付日常批量导入。按裁定 A-24 不设独立数据迁移阶段，期初与历史数据导入通道分别落在阶段 9a 的总账期初余额批次、阶段 10 的应收应付预收预付与资金账户期初、阶段 8 的库存期初，四类档案自身不设期初通道。

### 1 交付物清单

本阶段结束时下列内容可运行、可测试、可演示。

1. 六个新增业务 crate 编译通过并接入依赖方向自检脚本：ep-contract-mdm、ep-domain-mdm、ep-app-mdm、ep-contract-cpq、ep-domain-cpq、ep-app-cpq，另加 ep-contract-crm 与 ep-app-crm 两个仅承载客户 360 视图骨架的 crate，以及按裁定 A-08 归本阶段的 ep-adapter-doc。
2. mdm 与 cpq 两个 schema 的全部迁移可离线执行、可重复执行、可按文件头的回退说明逆向，refinery 历史表分别落在 mdm.refinery_schema_history 与 cpq.refinery_schema_history。
3. core-server 暴露本阶段的全部同步端点，四类档案的建档、提交、撤回、作废、变更申请、审批结论应用、停用、启用、查询、版本查询、引用校验，价目表的建档与取价，历史成交资料查询，导入批次与导出任务的发起与回执，客户 360 概览。
4. job-worker 承载本阶段的五类后台任务：审批结论消费、导入文件解析与草稿落库、导出文件渲染、价目表失效日扫描、供应商资质到期日扫描；另承载档案变更事件到内置搜索索引的写入，写入经 ep_foundation::port::search::SearchIndexPort，投影结构取 ep_foundation::port::search::SearchDocument，端口与适配实现按裁定 A-07 分别由阶段 1 与阶段 3b 提供，本阶段只提供投影函数。
5. 一套可运行的档案生命周期：草稿到待审批到已生效启用到已生效停用到再启用，全程经变更申请单承载并留版本快照，全程写审计。
6. 一套可运行的取价带出：给定法人、客户、产品、计量单位与单据日期，返回零命中、单命中、多命中三种结果，多命中时显式要求人工选择。
7. 一套可运行的批量导入：模板下载、上传、逐行校验、错误行清单下载、通过行落草稿、批量提交审批。
8. 基准数据集生成器新增主数据分片，按规格附录 A.3 产出每法人客户、供应商、物料各 5000 条，产品 5000 条，价目表 20 张合计 5 万行明细。
9. 三份文档更新并通过 CI 一致性校验：docs/data-dictionary/mdm.md 与 docs/data-dictionary/cpq.md、docs/event-catalog.md 新增条目、docs/error-codes.md 新增条目。
10. 法人越权测试目标 tests/rls_matrix 中新增本阶段 30 张表的八类用例，全部通过。
11. 规格附录 A.1 中归属本阶段的四个度量项的 EXPLAIN 证据与 P95 实测记录：客户列表按条件过滤并翻页、客户详情打开、附件列表加载、全文检索返回首页结果，另加销售订单表单打开并带出默认值这一项中的取价子段实测。
12. crates/foundation/src/port/doc.rs 按裁定 A-08 补齐 SheetSpec、ColumnSpec、CellValue、PdfSource、PrintLayout 五个类型与 SpreadsheetPort、DocTemplatePort、PdfRenderPort 三个 trait，并交付 ep-adapter-doc 的实现，覆盖导入模板生成、错误行清单渲染、XLSX 读写三项用途。
13. mdm.v_customers_dataset、mdm.v_products_dataset、mdm.v_materials_dataset 三个受治理数据集视图已发布并授予 ep_analyst_ro，dataset code 依次为 mdm_customers、mdm_products、mdm_materials，grain 均为 DOCUMENT，按裁定 A-18 交付。
14. 本模块的四端界面：clients/desktop/src/modules/mdm、clients/desktop/src/modules/cpq、clients/desktop/src/modules/crm 与 clients/mobile/src/modules/mdm、clients/mobile/src/modules/cpq、clients/mobile/src/modules/crm 六个目录，按裁定 A-23 交付。
15. 能力域码与动作类别常量：crates/contract/mdm/src/capability.rs、crates/contract/cpq/src/capability.rs 与 crates/contract/crm/src/capability.rs 中为每个用例声明一对 <USECASE_SCREAMING>_DOMAIN 与 <USECASE_SCREAMING>_ACTION 常量，取值来自阶段 1 冻结的 foundation::CapabilityDomain 与 foundation::ActionClass，xtask configdoc 通过。
16. ep-contract-mdm 的两个注册表 MasterReferenceCounterRegistry 与 TradeHistoryProviderRegistry 及其聚合逻辑可运行，注册项由阶段 6、7、8、10、12 按裁定 A-15 反向注入。

### 2 crate 与进程归属

#### 2.1 新增 crate

| crate | 路径 | 职责 | 依赖 |
|---|---|---|---|
| ep-contract-mdm | crates/contract/mdm | 四类档案与两类辅助资料的命令、查询、事件类型与 DTO；对外的四个端口 trait；供其他模块调用的引用校验与版本读取 trait | ep-foundation |
| ep-domain-mdm | crates/domain/mdm | 四类档案聚合、变更申请聚合、值对象、状态机、冻结规则、唯一性规则、编码规则、版本差异计算 | ep-foundation、ep-contract-mdm |
| ep-app-mdm | crates/application/mdm | 全部用例、事务边界、授权调用、审计与 Outbox 写入、导入导出编排、历史成交资料聚合 | ep-foundation、ep-platform-*、ep-domain-mdm、ep-contract-* |
| ep-contract-cpq | crates/contract/cpq | 价目表命令与查询、取价端口 PriceResolver、取价结果 DTO | ep-foundation |
| ep-domain-cpq | crates/domain/cpq | 价目表聚合、明细行值对象、状态机、生效期与适用范围规则、取价命中规则 | ep-foundation、ep-contract-cpq |
| ep-app-cpq | crates/application/cpq | 价目表用例、取价用例、失效扫描用例 | ep-foundation、ep-platform-*、ep-domain-cpq、ep-contract-mdm、ep-contract-cpq |
| ep-contract-crm | crates/contract/crm | 客户 360 区块注册契约 Customer360SectionProvider 与区块 DTO，按裁定 C-09 由阶段 12 扩充，不新增第二套契约 | ep-foundation |
| ep-app-crm | crates/application/crm | 客户 360 查询用例，组装 mdm 档案区块与已注册的其他区块 | ep-foundation、ep-platform-*、ep-contract-crm、ep-contract-mdm |
| ep-adapter-doc | crates/adapter/doc | SpreadsheetPort、DocTemplatePort、PdfRenderPort 三个端口的实现，覆盖导入模板生成、错误行清单渲染与 XLSX 读写；按裁定 A-08 本阶段是该 crate 的唯一提供方 | ep-foundation |

本阶段不新增 crates/domain/crm，理由是首版客户 360 视图没有自有聚合与自有不变量，只有组装逻辑，建立空领域 crate 会制造无内容的分层。

#### 2.2 改动的既有 crate

| crate | 改动 |
|---|---|
| ep-adapter-db-pg | 新增 mdm 与 cpq 两个仓储实现目录，按 schema 分文件，一个仓储只访问自己模块的 schema |
| ep-foundation | 在阶段 1 建立的空文件 crates/foundation/src/port/doc.rs 中补齐 SheetSpec、ColumnSpec、CellValue、PdfSource、PrintLayout 五个类型与 SpreadsheetPort、DocTemplatePort、PdfRenderPort 三个 trait，签名按裁定 A-08 冻结，阶段 6、10、11、13 只在其上增量取值，不新增 trait |
| ep-adapter-search | 按 ep_foundation::port::search::SearchDocument 结构定义四类档案与价目表的投影函数，写入方为 job-worker 的索引消费者；索引正文只含编码、名称、简称、规格型号、类别与备注，不含开票要素与银行信息 |
| ep-testkit | 新增 CustomerBuilder、SupplierBuilder、MaterialBuilder、ProductBuilder、PriceListBuilder、ChangeRequestBuilder 六个构造器与探针桩 |
| ep-datagen | 新增主数据分片，按规格附录 A.3 取值产出 |
| apps/core-server | 在 wiring.rs 中装配本阶段的仓储、端口实现与路由 |
| apps/job-worker | 在 wiring.rs 中注册本阶段的五类后台任务与两类事件消费者 |

#### 2.3 进程归属

| 进程 | 本阶段承载的内容 |
|---|---|
| core-server | 本阶段全部 HTTP 端点、全部同步用例、全部业务事务、档案附件正文的读写转发 |
| job-worker | 审批结论消费、导入解析、导出渲染、价目表失效扫描、资质到期扫描、搜索索引写入、本阶段事件的死信重投 |
| portal-gateway | 无本阶段代码，门户阶段经 core-server 的 /api/v1/portal 受控能力 API 调用本阶段暴露的供应商自维护命令 |
| integration-gateway、plugin-host、ops-agent、archive-writer、backup-writer | 无本阶段代码 |

### 3 数据库变更

#### 3.1 总览与迁移顺序

新增两个 schema 下的 30 张表，其中 mdm 27 张、cpq 3 张。schema 与角色已在基线第 3.1 节登记，本阶段不新增 schema，不新增角色。db/migrations/order.toml 中 mdm 与 cpq 的相对顺序已由基线固定为 mdm 在 cpq 之前，本阶段不改动该文件的模块顺序。

迁移文件按下表顺序，路径 db/migrations/mdm/ 与 db/migrations/cpq/，命名按基线第 3.9 节。每个文件只做一件事，每个文件头部带 -- rollback: 段。

| 序号 | 文件名 | 内容 |
|---|---|---|
| 1 | V202609010900__mdm_create_uoms.sql | 建 mdm.uoms |
| 2 | V202609010905__mdm_create_classification_items.sql | 建 mdm.classification_items |
| 3 | V202609010910__mdm_create_customers.sql | 建 mdm.customers |
| 4 | V202609010915__mdm_create_customer_contacts.sql | 建 mdm.customer_contacts |
| 5 | V202609010920__mdm_create_customer_addresses.sql | 建 mdm.customer_addresses |
| 6 | V202609010925__mdm_create_customer_invoice_profiles.sql | 建 mdm.customer_invoice_profiles |
| 7 | V202609010930__mdm_create_suppliers.sql | 建 mdm.suppliers |
| 8 | V202609010935__mdm_create_supplier_contacts.sql | 建 mdm.supplier_contacts |
| 9 | V202609010940__mdm_create_supplier_payment_profiles.sql | 建 mdm.supplier_payment_profiles |
| 10 | V202609010945__mdm_create_supplier_qualifications.sql | 建 mdm.supplier_qualifications |
| 11 | V202609010950__mdm_create_supplier_price_records.sql | 建 mdm.supplier_price_records |
| 12 | V202609010955__mdm_create_supplier_leadtime_records.sql | 建 mdm.supplier_leadtime_records |
| 13 | V202609011000__mdm_create_supplier_risk_records.sql | 建 mdm.supplier_risk_records |
| 14 | V202609011005__mdm_create_materials.sql | 建 mdm.materials |
| 15 | V202609011010__mdm_create_products.sql | 建 mdm.products |
| 16 | V202609011015__mdm_create_product_material_links.sql | 建 mdm.product_material_links |
| 17 | V202609011020__mdm_create_change_requests.sql | 建 mdm.change_requests |
| 18 | V202609011025__mdm_create_record_versions.sql | 建 mdm.record_versions |
| 19 | V202609011030__mdm_create_import_batches.sql | 建 mdm.import_batches |
| 20 | V202609011035__mdm_create_import_batch_rows.sql | 建 mdm.import_batch_rows |
| 21 | V202609011040__mdm_create_export_jobs.sql | 建 mdm.export_jobs |
| 22 | V202609011045__mdm_create_attachment_link_tables.sql | 建六张附件关联表 |
| 23 | V202609011050__mdm_enable_rls.sql | 对 mdm 的 27 张表按基线第 3.8 节模板启用并强制行级安全 |
| 24 | V202609011055__mdm_create_lookup_indexes.sql | 建本节声明的全部非基线索引，一律 CREATE INDEX CONCURRENTLY |
| 25 | V202609011057__mdm_backfill_sensitive_field_registry.sql | 按裁定 A-28 向 platform_core.sensitive_field_registry 插入四行，逐行给全裁定 C-06 冻结的十一列取值，公共列另按基线第 4 节；四行 schema_name 均取 mdm，table_name 取 customer_invoice_profiles 与 supplier_payment_profiles，column_name 取 bank_name 与 bank_account_no 且为逻辑列名不带 _enc 后缀，category 均取 ACCOUNT，security_level 均取 30，is_field_encrypted 取 bank_account_no 两行为 true 与 bank_name 两行为 false，normalization 均取 TRIM_NFKC，release_ref 均取 MIGRATION 加本迁移版本号；bank_name 两行的 blind_index 取 NONE、blind_index_column 留空、mask_style 取 NONE，bank_account_no 两行的 blind_index 取 EXACT、blind_index_column 取 bank_account_no_bidx、mask_style 取 KEEP_LAST_4；created_by 取 foundation::SYSTEM_PRINCIPAL_ID |
| 26 | V202609011058__mdm_create_dataset_views.sql | 按裁定 A-18 建 mdm.v_customers_dataset、mdm.v_products_dataset、mdm.v_materials_dataset 三个视图，每个视图带 legal_entity_id、security_level、data_scope_tags 三列，并在同一迁移中 GRANT SELECT TO ep_analyst_ro |
| 27 | V202609011100__cpq_create_price_lists.sql | 建 cpq.price_lists |
| 28 | V202609011105__cpq_create_price_list_lines.sql | 建 cpq.price_list_lines |
| 29 | V202609011110__cpq_create_price_list_customer_links.sql | 建 cpq.price_list_customer_links |
| 30 | V202609011115__cpq_enable_rls.sql | 对 cpq 的 3 张表启用并强制行级安全 |
| 31 | V202609011120__cpq_create_lookup_indexes.sql | 建 cpq 的取价索引 |

文件名中的时间戳按实际撰写时间取值，同一 schema 内的执行顺序由文件名字典序决定，上表给出的是相对顺序，不是绝对时间承诺。全部迁移属基线第 3.9 节的在线变更范围，无一项需要停机窗口。索引创建一律并发执行，迁移会话固定 SET lock_timeout = '5s' 与 SET statement_timeout = '30min'。

本阶段的迁移只有第 25 号一处回填数据，即按裁定 A-28 向 platform_core.sensitive_field_registry 登记银行字段四行，该表是平台侧的字段元数据登记入口，登记内容属字段治理而非业务数据。该迁移跨 schema 写入，按裁定通则第五条放在 db/migrations/order.toml 中位次靠后的一方即 db/migrations/mdm/ 目录下，空库上按 order.toml 全量执行时 platform_core.sensitive_field_registry 已经建立。四类档案与价目表的编号规则行仍不由 mdm 的迁移写入 platform_core.number_sequences，理由是那会构成跨模块直接写业务表；改由 ep-app-mdm 在模块生命周期的启用动作中经 ep-platform-sequence 的注册端口幂等登记。本阶段不依赖 platform_meta，字段元数据的登记入口只有阶段 2 的 platform_core.sensitive_field_registry 与阶段 4 的 platform_authz.field_permissions 两处，后者的字段级授权行由阶段 3b 的配置发布通道在本阶段之后写入，本阶段交付时按默认拒绝处理。

#### 3.2 公共列与共通约定

每张表都带基线第 4 节的公共列，顺序按基线：id、legal_entity_id、security_level、data_scope_tags、row_version、created_at、created_by、updated_at、updated_by。仅追加表去掉 row_version、updated_at、updated_by 三列。下文各表只列专有列。

主键类型 uuid，取值为应用侧生成的 UUIDv7，数据库侧不设默认值。同 schema 内的引用建真实外键，ON DELETE RESTRICT。跨 schema 的引用只留逻辑列，不建外键，存在性由 application 层经对方模块契约校验。本阶段涉及的跨模块逻辑引用共四类：cpq.price_list_lines.product_id 与 uom_id 指向 mdm、cpq.price_list_customer_links.customer_id 指向 mdm、全部 owner_user_id 指向 platform_core 的用户、全部 attachment_object_id 指向 platform_file.attachment_objects。

档案类表的状态表达按下列映射，PRD 第 2.2.1 节的五个状态映射到 status 与 is_active 两列，不引入第三套写法。

| PRD 状态 | status | is_active | version_no |
|---|---|---|---|
| 草稿 | DRAFT | true | 0 |
| 待审批 | PENDING_APPROVAL | true | 0 |
| 已生效启用 | EFFECTIVE | true | 大于 0 |
| 已生效停用 | EFFECTIVE | false | 大于 0 |
| 已作废 | VOID | true | 0 |

由此每张档案类表带三条固定 CHECK 约束。

- ck_<table>_status_active：status = 'EFFECTIVE' or (is_active and deactivated_at is null)
- ck_<table>_active_deactivated：is_active = (deactivated_at is null)
- ck_<table>_version_effective：(version_no > 0) = (status = 'EFFECTIVE')

本阶段统一使用一种索引写法表达唯一性中的条件唯一，称为空槽唯一索引：新增一个可空列作为槽位，条件成立时写入分组键，条件不成立时置 NULL，再对该槽位建普通唯一索引。PostgreSQL 的唯一索引默认按 NULLS DISTINCT 处理，因此 NULL 行互不冲突。采用这一写法的理由是基线第 3.10 节禁止首版使用部分索引与函数索引，而条件唯一在本阶段出现六处，需要一种统一且不违反基线的表达。槽位列的取值由 CHECK 约束用 CASE 表达式强制，CASE 出现在 CHECK 中不构成函数索引。

#### 3.3 mdm schema 逐表定义

mdm.uoms 计量单位，档案类，不走审批链。

| 列 | 类型 | 可空 | 约束 |
|---|---|---|---|
| code | text | 否 | ck 长度 1 至 64 |
| name | text | 否 | ck 长度 1 至 200 |
| is_active | boolean | 否 | 默认 true |
| deactivated_at | timestamptz | 是 | ck 与 is_active 互斥 |

索引：pk_uoms、ix_uoms_legal_entity_id_created_at、ux_uoms_legal_entity_id_code。

mdm.classification_items 受控取值字典，档案类，承载客户类型、供应商分类、物料类别、产品类别、证照类型、风险类别、结算方式七类取值。税率一类按裁定 C-11 移出本表，唯一出处为阶段 10 的 invoice.tax_rate_options。

| 列 | 类型 | 可空 | 约束 |
|---|---|---|---|
| object_type | text | 否 | ck in ('CUSTOMER_TYPE','SUPPLIER_CATEGORY','MATERIAL_CATEGORY','PRODUCT_CATEGORY','QUALIFICATION_TYPE','RISK_CATEGORY','SETTLEMENT_METHOD') |
| code | text | 否 | ck 长度 1 至 64 |
| name | text | 否 | ck 长度 1 至 200 |
| parent_id | uuid | 是 | fk_classification_items_classification_items |
| sort_no | int | 否 | 默认 0 |
| is_active | boolean | 否 | 默认 true |
| deactivated_at | timestamptz | 是 | |

索引：pk、ix_classification_items_legal_entity_id_created_at、ux_classification_items_legal_entity_id_object_type_code。

mdm.customers 客户档案，档案类。

| 列 | 类型 | 可空 | 约束 |
|---|---|---|---|
| code | text | 否 | ck 长度 1 至 64 |
| status | text | 否 | ck in ('DRAFT','PENDING_APPROVAL','EFFECTIVE','VOID') |
| is_active | boolean | 否 | 默认 true |
| deactivated_at | timestamptz | 是 | |
| version_no | bigint | 否 | 默认 0 |
| name | text | 否 | ck 长度 1 至 200 |
| short_name | text | 是 | ck 长度不超过 200 |
| unified_social_credit_code | text | 是 | ck 长度等于 18 |
| alternate_identifier | text | 是 | ck 长度不超过 64 |
| customer_type | text | 否 | 逻辑引用 classification_items.code |
| owner_user_id | uuid | 否 | 逻辑引用平台用户 |
| settlement_method | text | 是 | 逻辑引用 classification_items.code |
| payment_term_days | int | 是 | ck 大于等于 0 |
| credit_limit | numeric(18,2) | 是 | ck 大于等于 0 |
| remark | text | 是 | ck 长度不超过 2000 |

另有 ck_customers_identifier_present：unified_social_credit_code is not null or alternate_identifier is not null。索引：pk、ix_customers_legal_entity_id_created_at、ux_customers_legal_entity_id_code、ux_customers_legal_entity_id_unified_social_credit_code、ix_customers_legal_entity_id_name、ix_customers_legal_entity_id_name_pattern（带 text_pattern_ops 操作符类，用于列表的前缀 like）、ix_customers_legal_entity_id_status_is_active、ix_customers_legal_entity_id_owner_user_id。

mdm.customer_contacts 客户联系人。专有列：customer_id uuid 非空外键、person_name text 非空长度不超过 200、title text 可空、phone text 可空长度不超过 32、email text 可空长度不超过 320、is_default boolean 非空默认 false、default_slot uuid 可空、sort_no int 非空默认 0、is_active boolean 非空默认 true、deactivated_at timestamptz 可空。约束 ck_customer_contacts_default_slot：default_slot is not distinct from (case when is_default and is_active then customer_id else null end)。约束 ck_customer_contacts_reachable：phone is not null or email is not null。索引：pk、ix_customer_contacts_legal_entity_id_created_at、ix_customer_contacts_legal_entity_id_customer_id、ux_customer_contacts_legal_entity_id_default_slot、fk_customer_contacts_customers。

mdm.customer_addresses 客户收货地址。专有列：customer_id、address_line text 非空长度不超过 500、receiver_name text 可空、receiver_phone text 可空长度不超过 32、is_default、default_slot、sort_no、is_active、deactivated_at。约束与索引与 customer_contacts 同构。

mdm.customer_invoice_profiles 客户开票要素，与客户一对一。专有列：customer_id、invoice_title text 可空、taxpayer_no text 可空长度不超过 64、registered_address text 可空长度不超过 500、registered_phone text 可空长度不超过 32、bank_name text 可空、bank_account_no_enc bytea 可空、bank_account_no_key_ref text 可空、bank_account_no_tail text 可空、bank_account_no_bidx bytea 可空；本表不设同名明文列 bank_account_no。索引：pk、ix_..._legal_entity_id_created_at、ux_customer_invoice_profiles_legal_entity_id_customer_id、ix_customer_invoice_profiles_legal_entity_id_bank_account_no_bidx。bank_name 与 bank_account_no 两列的字段级密级取 30，按裁定 A-28 登记在 platform_core.sensitive_field_registry，登记行的 column_name 取逻辑列名不带 _enc 后缀，不改本表行级 security_level 默认值 20。bank_account_no 按裁定 A-28 取 is_field_encrypted 为真，依据是规格第 7.8 章把行内敏感字段的字段级密钥定为强制项并把账户类属性列入最低覆盖面，该项不属待决；其物理形态固定为 bank_account_no_enc 承载密文、bank_account_no_key_ref 记录密钥标识与版本、bank_account_no_tail 承载掩码保留的后四位。bank_name 取 is_field_encrypted 为假并保持明文物理列，这是 PRD 附录乙 U-A-12 未决期间的临时取值。db/checks/11 按 is_field_encrypted 分支断言，对 bank_account_no 断言物理表上存在 bank_account_no_enc 且类型为 bytea 且不存在同名明文列，对 bank_name 只断言 mdm.customer_invoice_profiles.bank_name 三元组在 information_schema.columns 中命中实际列。等值定位一律经盲索引列 bank_account_no_bidx，取值为 derive_blind_key(legal_entity_id, 'mdm.customer_invoice_profiles.bank_account_no', plaintext) 的前 16 字节，与 foundation::BlindIndex 的 [u8; 16] 一致，derive_blind_key 与 BlindIndex 由阶段 2 按裁定 B-04 提供。该列上只建普通 btree ix_customer_invoice_profiles_legal_entity_id_bank_account_no_bidx，不建唯一约束，也不走阶段 2 计划第 4.4 节所称的完整 32 字节例外路径，依据是 PRD 第 2.3.1 节的开票要素与第 2.4.1 节的开票与收款信息都不要求银行账号在法人内不重复，裁定 B-04 只为 finance.cash_accounts 指名唯一约束 ux_cash_accounts_legal_entity_id_bank_account_no_bidx，对 mdm 两列只约定列名；阶段 2 计划第 4.5 节假设三把 mdm 两列一并写入例外路径，与上述两条依据不符，按权威顺序以 PRD 与裁定表为准。客户与供应商的银行账号重复不构成错误，本阶段不在该列上做唯一性校验，第 4.4 节唯一性表因此不新增行。规格第 7.8 章禁止字段级密文直接用于唯一约束，本阶段不自建第二套哈希，待决范围见第 13 节 U-A-12。

mdm.suppliers 供应商档案，档案类。列与 customers 同构，差异为：unified_social_credit_code 非空且长度等于 18，无 alternate_identifier 与 credit_limit，新增 supplier_category text 非空、portal_enabled boolean 非空默认 false、qualification_status text 非空默认 'VALID' 且 ck in ('VALID','EXPIRING','EXPIRED')。索引除与 customers 同构的七条外，新增 ix_suppliers_legal_entity_id_qualification_status。

mdm.supplier_contacts 与 mdm.supplier_payment_profiles，结构分别与 customer_contacts 与 customer_invoice_profiles 同构，主体列为 supplier_id，supplier_payment_profiles 无注册地址与注册电话两列；银行字段的密级、登记行、物理列形态与盲索引列 bank_account_no_bidx 与 customer_invoice_profiles 同构，同样按裁定 A-28 取 bank_account_no 的 is_field_encrypted 为真，并以 bank_account_no_enc bytea 与 bank_account_no_key_ref text 与 bank_account_no_tail text 三列承载、不保留同名明文列，bank_name 取假并保持明文列，盲索引的域串取 mdm.supplier_payment_profiles.bank_account_no，该列同样取前 16 字节、只建普通 btree ix_supplier_payment_profiles_legal_entity_id_bank_account_no_bidx、不建唯一约束，理由与客户开票要素一节相同。

mdm.supplier_qualifications 资质证照。专有列：supplier_id、qualification_type text 非空、qualification_no text 非空长度不超过 64、issuing_authority text 可空长度不超过 200、valid_from_date date 非空、valid_to_date date 可空、sort_no、is_active、deactivated_at。约束 ck_supplier_qualifications_period：valid_to_date is null or valid_to_date >= valid_from_date。索引：pk、ix_..._legal_entity_id_created_at、ux_supplier_qualifications_legal_entity_id_supplier_id_qualification_type_qualification_no、ix_supplier_qualifications_legal_entity_id_valid_to_date、fk_supplier_qualifications_suppliers。

mdm.supplier_price_records 供应商价格资料。专有列：supplier_id、material_id、uom_id、unit_price numeric(18,6) 非空且大于等于 0、is_tax_included boolean 非空、tax_rate numeric(9,6) 可空且大于等于 0 小于 1、valid_from_date date 非空、valid_to_date date 可空、source text 非空 ck in ('INTERNAL','SUPPLIER_PORTAL')、is_active、deactivated_at。索引：pk、ix_..._legal_entity_id_created_at、ix_supplier_price_records_legal_entity_id_supplier_id_material_id_valid_from_date、fk_supplier_price_records_suppliers、fk_supplier_price_records_materials、fk_supplier_price_records_uoms。

mdm.supplier_leadtime_records 交期资料。专有列：supplier_id、material_id、lead_time_days int 非空且大于等于 0、valid_from_date、valid_to_date、source、is_active、deactivated_at。索引与价格资料同构。

mdm.supplier_risk_records 质量与风险记录。专有列：supplier_id、occurred_on date 非空、risk_category text 非空、description text 非空长度不超过 2000、source text 非空、is_active、deactivated_at。索引：pk、ix_..._legal_entity_id_created_at、ix_supplier_risk_records_legal_entity_id_supplier_id_occurred_on。按裁定 C-10 本表是供应商风险记录的唯一权威表，procure.supplier_risk_records 撤销，采购阶段经本阶段提供的 ep_contract_mdm::SupplierRiskRecordPort 读写；质量记录归 procure 的 procure.supplier_quality_records，本阶段不建该表。

mdm.materials 物料档案，档案类。专有列除档案类共通五列外：name、specification text 可空长度不超过 500、base_uom_id uuid 非空外键、material_category text 非空、is_batch_managed boolean 非空、is_serial_managed boolean 非空、default_purchase_tax_rate numeric(9,6) 可空 ck 大于等于 0 小于 1、owner_user_id、remark。索引：pk、ix_materials_legal_entity_id_created_at、ux_materials_legal_entity_id_code、ix_materials_legal_entity_id_name、ix_materials_legal_entity_id_name_pattern、ix_materials_legal_entity_id_status_is_active、fk_materials_uoms。

mdm.products 产品档案，档案类。专有列：name、product_category text 非空、sales_uom_id uuid 非空外键、default_output_tax_rate numeric(9,6) 可空、is_sellable boolean 非空默认 true、owner_user_id、remark。索引：pk、ix_products_legal_entity_id_created_at、ux_products_legal_entity_id_code、ix_products_legal_entity_id_name、ix_products_legal_entity_id_name_pattern、ix_products_legal_entity_id_status_is_active_is_sellable、fk_products_uoms。

mdm.product_material_links 产品与物料关联。专有列：product_id uuid 非空外键、material_id uuid 非空外键、active_product_id uuid 可空、is_active、deactivated_at。约束 ck_product_material_links_active_slot：active_product_id is not distinct from (case when is_active then product_id else null end)。索引：pk、ix_..._legal_entity_id_created_at、ux_product_material_links_legal_entity_id_active_product_id、ux_product_material_links_legal_entity_id_product_id_material_id、两条外键。首版关联基数为至多一条，由 ux_product_material_links_legal_entity_id_active_product_id 强制；U-C 组决策放开为多对多时，只需删除该唯一索引，属基线第 3.9 节的在线变更范围。

mdm.change_requests 变更申请单，单据类，是本阶段唯一的审批入口。

| 列 | 类型 | 可空 | 约束 |
|---|---|---|---|
| doc_no | text | 否 | ux 带法人 |
| status | text | 否 | ck in ('DRAFT','PENDING_APPROVAL','APPROVED','REJECTED','WITHDRAWN','VOID') |
| object_type | text | 否 | ck in ('CUSTOMER','SUPPLIER','MATERIAL','PRODUCT','PRICE_LIST') |
| object_id | uuid | 否 | 逻辑引用对应档案表 |
| change_kind | text | 否 | ck in ('CREATION','FIELD_CHANGE','DEACTIVATION','REACTIVATION') |
| base_version_no | bigint | 否 | 发起时的档案版本，CREATION 时为 0 |
| base_row_version | bigint | 否 | 发起时的档案行版本，用于乐观锁比对 |
| proposed | jsonb | 否 | 提议生效后的完整快照 |
| diff | jsonb | 否 | 逐字段变更前后对照 |
| reason | text | 是 | 长度不超过 2000 |
| source | text | 否 | ck in ('INTERNAL','SUPPLIER_PORTAL','IMPORT') |
| name_duplicate_confirmed | boolean | 否 | 默认 false |
| name_duplicate_note | text | 是 | 长度不超过 2000 |
| approval_instance_id | uuid | 是 | 逻辑引用 platform_flow 实例 |
| open_slot | uuid | 是 | 空槽唯一索引槽位 |
| submitted_at | timestamptz | 是 | |
| decided_at | timestamptz | 是 | |
| decided_by | uuid | 是 | |

约束 ck_change_requests_open_slot：open_slot is not distinct from (case when status in ('DRAFT','PENDING_APPROVAL') then object_id else null end)。索引：pk、ix_change_requests_legal_entity_id_created_at、ux_change_requests_legal_entity_id_doc_no、ux_change_requests_legal_entity_id_open_slot、ix_change_requests_legal_entity_id_object_type_object_id、ix_change_requests_legal_entity_id_status。ux_change_requests_legal_entity_id_open_slot 是 PRD 第 2.2.1 节同一档案同时只允许存在一份未结束的变更申请这一规则的唯一强制点，不靠先查后插。

mdm.record_versions 档案版本快照，仅追加表，不带 row_version、updated_at、updated_by，也不带 reverses_id，理由是版本快照不存在冲销语义，加一个恒为 NULL 的列只会制造误解；本表不在基线第 4 节仅追加表一条的枚举之内，该取舍按第 12.1 节 D3 登记为对基线第 4 节的偏离，不列入第 12.2 节的新增决定。专有列：object_type、object_id、version_no bigint 非空、change_request_id uuid 非空、change_kind text 非空、snapshot jsonb 非空、effective_at timestamptz 非空。索引：pk、ix_record_versions_legal_entity_id_created_at、ux_record_versions_legal_entity_id_object_id_version_no。

mdm.import_batches 导入批次，单据类。专有列：doc_no、status ck in ('UPLOADED','PARSING','PARSED','DRAFTING','DRAFTED','SUBMITTED','FAILED')、object_type、source_attachment_object_id uuid 非空、template_version text 非空、total_rows int 可空、passed_rows int 可空、failed_rows int 可空、drafted_rows int 可空、error_report_attachment_object_id uuid 可空、started_at、finished_at、last_error text 可空。索引：pk、ix_..._legal_entity_id_created_at、ux_import_batches_legal_entity_id_doc_no、ix_import_batches_legal_entity_id_status。

mdm.import_batch_rows 导入行结果，仅追加表，同样不带 row_version、updated_at、updated_by，也不带 reverses_id，理由与 mdm.record_versions 相同，一并按第 12.1 节 D3 登记。专有列：import_batch_id uuid 非空外键、row_no int 非空、raw jsonb 非空、outcome text 非空 ck in ('PASSED','FAILED')、error_field text 可空、error_code text 可空、error_message text 可空、created_object_id uuid 可空。索引：pk、ix_..._legal_entity_id_created_at、ux_import_batch_rows_legal_entity_id_import_batch_id_row_no、fk_import_batch_rows_import_batches。

mdm.export_jobs 导出任务，单据类。专有列：doc_no、status ck in ('QUEUED','RUNNING','SUCCEEDED','FAILED')、object_type、filter jsonb 非空、column_keys text[] 非空、includes_sensitive_fields boolean 非空、reauth_ref uuid 可空、approval_ref uuid 可空、row_count int 可空、result_attachment_object_id uuid 可空、started_at、finished_at、last_error text 可空。索引：pk、ix_..._legal_entity_id_created_at、ux_export_jobs_legal_entity_id_doc_no。

六张附件关联表：mdm.customer_attachments、mdm.supplier_attachments、mdm.supplier_qualification_attachments、mdm.supplier_risk_record_attachments、mdm.material_attachments、mdm.product_attachments。列按基线第 4 节固定形态：owner_id uuid 非空、attachment_object_id uuid 非空、purpose text 非空、sort_no int 非空默认 0，另加 is_active boolean 非空默认 true 与 deactivated_at timestamptz 可空。追加这两列的理由是基线第 3.6 节禁止在业务 schema 上执行 DELETE，而解除附件关联是必需操作，该取舍按第 12.1 节 D2 登记为对基线第 4 节的偏离，不列入第 12.2 节的新增决定。索引：pk、ix_..._legal_entity_id_created_at、ix_<table>_legal_entity_id_owner_id、ux_<table>_legal_entity_id_owner_id_attachment_object_id。

#### 3.4 cpq schema 逐表定义

cpq.price_lists 价目表，档案类另带扩展状态机。

| 列 | 类型 | 可空 | 约束 |
|---|---|---|---|
| code | text | 否 | ux 带法人 |
| name | text | 否 | ck 长度 1 至 200 |
| status | text | 否 | ck in ('DRAFT','PENDING_APPROVAL','EFFECTIVE','EXPIRED','VOID') |
| is_active | boolean | 否 | 默认 true，EFFECTIVE 下取 false 表示已停用 |
| deactivated_at | timestamptz | 是 | |
| version_no | bigint | 否 | 默认 0 |
| scope_kind | text | 否 | ck in ('ALL_CUSTOMERS','SPECIFIED_CUSTOMERS','SPECIFIED_CUSTOMER_TYPES') |
| scope_customer_types | text[] | 否 | 默认 '{}' |
| effective_from_date | date | 否 | |
| effective_to_date | date | 是 | ck 不早于 effective_from_date |
| owner_user_id | uuid | 否 | |
| remark | text | 是 | 长度不超过 2000 |

ck_price_lists_version_effective 调整为：(version_no > 0) = (status in ('EFFECTIVE','EXPIRED'))，理由是已失效是从已生效自动流转而来，版本号必须保留。索引：pk、ix_price_lists_legal_entity_id_created_at、ux_price_lists_legal_entity_id_code、ix_price_lists_legal_entity_id_status_effective_to_date。最后一条支撑每日失效扫描。

cpq.price_list_lines 价目表明细行。专有列：price_list_id uuid 非空外键、product_id uuid 非空跨模块逻辑引用、uom_id uuid 非空跨模块逻辑引用、is_tax_included boolean 非空、unit_price numeric(18,6) 非空且大于等于 0、floor_price numeric(18,6) 可空且大于等于 0、active_slot uuid 可空、sort_no int 非空默认 0、is_active boolean 非空默认 true、deactivated_at timestamptz 可空。约束 ck_price_list_lines_floor：floor_price is null or floor_price <= unit_price。约束 ck_price_list_lines_active_slot：active_slot is not distinct from (case when is_active then price_list_id else null end)。索引：pk、ix_price_list_lines_legal_entity_id_created_at、ux_price_list_lines_legal_entity_id_active_slot_product_id_uom_id、ix_price_list_lines_legal_entity_id_product_id_uom_id、fk_price_list_lines_price_lists。倒数第二条是取价的主索引。

cpq.price_list_customer_links 指定客户范围。专有列：price_list_id uuid 非空外键、customer_id uuid 非空跨模块逻辑引用、active_slot uuid 可空、is_active、deactivated_at。索引：pk、ix_..._legal_entity_id_created_at、ux_price_list_customer_links_legal_entity_id_active_slot_customer_id、ix_price_list_customer_links_legal_entity_id_price_list_id_customer_id。

#### 3.5 RLS 策略

30 张表全部带 legal_entity_id，全部按基线第 3.8 节的统一模板生成策略，策略名为 rls_<table>_le，策略由迁移生成器统一产出，不允许手写变体。本阶段不新增不带 legal_entity_id 的表。本阶段不使用 BYPASSRLS，跨法人查询按授权法人集合逐个法人设置会话变量后分别查询再在应用侧合并。

### 4 领域模型与关键算法

#### 4.1 核心类型

ep-domain-mdm 中的聚合根共六个：Customer、Supplier、Material、Product、ChangeRequest，另加不走审批链的 Uom 与 ClassificationItem 两个简单档案。ep-domain-cpq 中的聚合根为 PriceList，明细行与客户范围行是其内部实体，不独立成聚合。

关键值对象：MasterCode（编码，法人加对象类型内唯一，生效后冻结）、UnifiedSocialCreditCode（带校验位的 18 位标识）、VersionNo（从 0 起的单调递增版本号）、FieldDiff（单字段的前后对照）、RecordSnapshot（档案的完整快照，含头与子表，附件只存对象 ID）、PriceHit（单条取价命中，含价目表 ID、编码、单价、价格下限、含税标记、生效期）。

关键枚举与取值，一律 text 加 CHECK，取值大写 snake_case。

| 枚举 | 取值 |
|---|---|
| MasterStatus | DRAFT、PENDING_APPROVAL、EFFECTIVE、VOID |
| PriceListStatus | DRAFT、PENDING_APPROVAL、EFFECTIVE、EXPIRED、VOID |
| ChangeKind | CREATION、FIELD_CHANGE、DEACTIVATION、REACTIVATION |
| ChangeRequestStatus | DRAFT、PENDING_APPROVAL、APPROVED、REJECTED、WITHDRAWN、VOID |
| MasterObjectType | CUSTOMER、SUPPLIER、MATERIAL、PRODUCT、PRICE_LIST |
| ScopeKind | ALL_CUSTOMERS、SPECIFIED_CUSTOMERS、SPECIFIED_CUSTOMER_TYPES |
| QualificationStatus | VALID、EXPIRING、EXPIRED |
| RecordSource | INTERNAL、SUPPLIER_PORTAL、IMPORT |

#### 4.2 档案状态机

状态、流转、守卫条件，四类档案共用同一张表，价目表在此基础上多一条自动失效流转。

| 起点 | 终点 | 触发 | 守卫条件 |
|---|---|---|---|
| 无 | DRAFT | 创建档案草稿 | 具备该对象类型的创建权限；编码若人工指定则法人内唯一；引用字段存在且可用 |
| DRAFT | DRAFT | 编辑草稿 | 行版本一致；未存在未结束的变更申请 |
| DRAFT | PENDING_APPROVAL | 提交审批，生成 CREATION 变更申请 | 全部必填齐备；唯一性校验通过；同名校验已确认或无同名；ux_change_requests 空槽可占用 |
| PENDING_APPROVAL | DRAFT | 审批退回或申请人撤回 | 撤回时申请无人处理；退回由审批人触发 |
| PENDING_APPROVAL | EFFECTIVE 且 is_active 为 true | 审批通过并应用 | 变更申请状态为 APPROVED；档案 row_version 与 base_row_version 一致 |
| DRAFT 或 PENDING_APPROVAL | VOID | 申请人作废 | 版本号为 0，即从未生效 |
| EFFECTIVE | EFFECTIVE 且版本递增 | FIELD_CHANGE 变更申请通过并应用 | 冻结字段未被修改；引用字段可用；行版本一致 |
| EFFECTIVE 且 is_active 为 true | EFFECTIVE 且 is_active 为 false | DEACTIVATION 变更申请通过并应用 | 无 |
| EFFECTIVE 且 is_active 为 false | EFFECTIVE 且 is_active 为 true | REACTIVATION 变更申请通过并应用 | 无 |
| EFFECTIVE | 无 | 删除 | 不提供，PRD 第 2.2.1 节明确已生效档案不提供作废路径，删除口径待决 |
| price_lists 的 EFFECTIVE | EXPIRED | 每日失效扫描 | effective_to_date 非空且早于服务器自然日 |
| price_lists 的 EXPIRED | PENDING_APPROVAL | 提交变更申请以延长生效期 | 与 FIELD_CHANGE 同 |

停用与启用同样经变更申请与审批链，因此 DEACTIVATION 与 REACTIVATION 两类申请的 proposed 快照与基线快照除 is_active 外完全相同，diff 只有一行。停用不校验该档案上是否还有未完成单据，也不自动关闭这些单据，只在提交界面展示引用计数作为提示，依据 PRD 第 2.2.5 节。

#### 4.3 冻结字段判定

冻结字段清单，来自 PRD 第 2.3.1 至 2.6.1 小节的字段表。

| 对象 | 生效后永久冻结 | 条件冻结 | 判定条件 |
|---|---|---|---|
| 客户 | code、unified_social_credit_code、legal_entity_id | 无 | |
| 供应商 | code、unified_social_credit_code、legal_entity_id | 无 | |
| 物料 | code、legal_entity_id | base_uom_id、is_batch_managed、is_serial_managed | 该物料存在库存流水 |
| 产品 | code、legal_entity_id | sales_uom_id | 该产品被已生效合同行或销售订单行引用 |
| 价目表 | code、legal_entity_id | 无 | |

条件冻结的判定不由 mdm 自行查询别的模块的表，而是经 ep-contract-mdm 中定义的两个消费方端口完成，实现由后续阶段的模块在 apps/core-server/src/wiring.rs 与 apps/job-worker/src/wiring.rs 中注入。注入形态按裁定通则第三条：本阶段先注入以 Noop 前缀命名的空实现并在该行加注释 // TODO(stage-8): replace with real impl 与 // TODO(stage-6): replace with real impl，由实现阶段替换该行。

- MaterialUsageProbe::has_stock_movement(&self, ctx: &SecurityContext, material_id: Id<Material>) -> Result<bool, AppError>，实现类型 InventoryMaterialUsageProbe 由阶段 8 交付，位于 crates/application/inventory/src/probe/material_usage.rs，取数为 inventory.stock_qty_entries 上按 material_id 的数量流水存在性判定，命中索引 ix_stock_qty_entries_legal_entity_id_material_id，索引列为 legal_entity_id 与 material_id，按裁定 A-13 执行；inventory.stock_value_entries 中 qty_entry_id 为空的纯金额调整行不参与该判定。
- ProductUsageProbe::is_referenced_by_effective_sales(&self, ctx: &SecurityContext, product_id: Id<Product>) -> Result<bool, AppError>，两个实现类型 ClmProductUsageProbe 与 SalesProductUsageProbe 由阶段 6 交付，组合类型 AnyProductUsageProbe(Vec<Arc<dyn ProductUsageProbe>>) 由本阶段在 ep-app-mdm 中提供，任一返回 true 即为 true。

探针缺位时的行为固定为：判定为不存在使用，即放行修改，同时写一条 WARN 日志与一条审计备注。理由是阶段 5 交付时点库存与销售模块尚未交付，不可能存在流水与引用，若默认拒绝则整个实施期无法修改这三个字段。为防止后续模块交付后忘记注入探针，本阶段在 core-server 的启动自检中追加一项命名自检项 master-data-usage-probes-registered：经 ep_platform_license::ModuleLicenseQuery::module_state 读取模块状态，若 Inventory 为 InstalledEnabled 而 MaterialUsageProbe 未注册，或 Sales 与 Clm 任一为 InstalledEnabled 而 ProductUsageProbe 未注册，则以退出码 78 拒绝启动。该项按裁定 C-25 以注册名标识而不用序号，注册在基线第 7.3 节十三个命名项之后；ModuleLicenseQuery 由阶段 3b 按裁定 A-05 提供，本阶段只读不实现。物料三项条件冻结的完整性验收顺延到阶段 8，产品一项顺延到阶段 6。

#### 4.4 唯一性、重名与引用校验

四项校验的执行时机与强制点。

| 校验 | 时机 | 强制点 | 失败行为 |
|---|---|---|---|
| 编码唯一，法人内跨全部状态 | 提交审批与应用生效两处 | 数据库唯一索引 ux_<table>_legal_entity_id_code | 唯一冲突映射为 MDM.<RESOURCE>.CODE_DUPLICATED，HTTP 409 |
| 统一社会信用代码唯一 | 同上 | ux_<table>_legal_entity_id_unified_social_credit_code | MDM.<RESOURCE>.USCC_DUPLICATED |
| 名称重复，仅比对已生效启用记录 | 提交审批时 | 应用层查询，走 ix_<table>_legal_entity_id_name | 未确认时返回 MDM.<RESOURCE>.NAME_DUPLICATE_UNCONFIRMED，details 列出至多 20 条同名档案编码；确认后写入 change_requests.name_duplicate_note 并写审计 |
| 引用存在且可用 | 提交审批与应用生效两处 | 同 schema 内引用由外键兜底，跨 schema 引用由 application 层经对方契约校验 | MDM.<RESOURCE>.REFERENCE_UNAVAILABLE，details 定位到字段 |

唯一性一律以数据库唯一索引为最终强制点，应用层的预查询只用于给出友好提示，不作为正确性依据，理由是 20 并发下先查后插存在真实竞态。

#### 4.5 统一社会信用代码校验

按 GB 32100 的 18 位结构：第 1 位登记管理部门代码，第 2 位机构类别代码，第 3 至 8 位登记管理机关行政区划码，第 9 至 17 位主体标识码，第 18 位校验码。字符集为 31 个码字，即 0 至 9 与 A 至 Z 去掉 I、O、S、V、Z。校验步骤如下。

1. 长度必须等于 18，全部字符必须落在码字集合内，否则返回 VALIDATION。
2. 取前 17 位的码字序号，与加权因子序列逐位相乘求和。加权因子序列为 1、3、9、27、19、26、16、17、20、29、25、13、8、24、10、30、28。
3. 校验值等于 31 减去和对 31 取余，若结果等于 31 则取 0。
4. 校验值对应的码字必须等于第 18 位，否则返回 VALIDATION 与错误码 MDM.<RESOURCE>.USCC_CHECKSUM_INVALID。

加权因子序列与码字集合在实现时必须与 GB 32100 原文逐位核对，并以不少于 30 组真实格式的正样本与 10 组篡改样本作为单元测试向量固化。客户档案在客户类型落入豁免集合时跳过本校验并要求 alternate_identifier 非空，豁免集合的临时取值见第 13 节。供应商不设豁免。

#### 4.6 编码生成与人工指定

档案编码格式固定为 <类型码>-<法人码>-<6 位流水>，例如 CUST-01-000123。与基线第 11.1 节的单据编号格式相比少一个年月段，理由是档案是长期存在的实体，年月段会让编码暗示建档时间并在跨年时产生视觉不连续，而档案编码生效后冻结、不可回收复用，年月段不提供任何额外区分度。该格式为本阶段新增决定并回写基线第 11.1 节。

类型码登记如下，均为 4 位大写字母，占用 ep-platform-sequence 的类型码空间。按裁定 C-26 全部类型码统一登记在 docs/data-dictionary.md 的单据类型码一节，由 xtask configdoc --check-doc-type-codes 校验该表与 ep-platform-sequence 常量表逐项一致且全局无重复；本阶段占用下表八个码，不新增未在该节登记的码。

| 对象 | 类型码 | 编号形态 |
|---|---|---|
| 客户档案 | CUST | 档案编码 |
| 供应商档案 | SUPP | 档案编码 |
| 物料档案 | MATL | 档案编码 |
| 产品档案 | PROD | 档案编码 |
| 价目表 | PRLS | 档案编码 |
| 变更申请单 | MDCR | 单据编号，含年月段 |
| 导入批次 | MDIB | 单据编号，含年月段 |
| 导出任务 | MDEX | 单据编号，含年月段 |

档案编码允许人工指定，人工指定时不占用流水序列，唯一性由数据库唯一索引强制。自动生成时在业务事务内经 SequencePort 取号，回滚即退号，不产生空号。单据编号不允许人工指定。流水位数不足时按基线第 11.1 节自动扩展为 7 位。

#### 4.7 版本快照与差异计算

应用生效时的步骤，是一个纯函数加一次写入。

1. 从 change_requests.proposed 取得提议快照，与当前档案的实际快照做逐字段比对，得到 FieldDiff 列表。字段路径用点分表达，子表行用 contacts[3].phone 这类下标路径，下标按 sort_no 与 id 的复合排序确定，保证同一份数据两次计算得到同一路径。
2. 比对结果与提交时算出的 change_requests.diff 逐条核对，不一致即判定为在审期间基线已被改动，返回 BUSINESS_CONFLICT 与 MDM.CHANGE_REQUEST.BASE_DRIFTED，不静默覆盖。该核对与 base_row_version 的乐观锁比对互为补充：乐观锁保证行未变，diff 核对保证子表未变。
3. 逐条应用 FieldDiff，version_no 加 1，写入 mdm.record_versions 一行，snapshot 取应用后的完整快照。
4. 版本号连续递增，不允许跳号；ux_record_versions_legal_entity_id_object_id_version_no 是该连续性的强制点。

变更生效不回溯已产生的单据，本阶段的保证方式是：单据侧在引用档案时通过 MasterDataLookup::resolve_reference 取回 (id, code, name, version_no) 四元组并自行留存 version_no，需要还原引用时点取值时调用 MasterDataLookup::load_version。mdm 不负责改写任何单据。

#### 4.8 价目表取价命中算法

输入为法人、客户 ID、客户类型、产品 ID、计量单位 ID、单据日期，输出为命中行列表与一个是否需要人工选择的标志。

1. 按 ix_price_list_lines_legal_entity_id_product_id_uom_id 取出该法人该产品该计量单位的全部启用明细行，通常不超过个位数。
2. 对每条明细行按主键取其价目表头，过滤条件为 status 等于 EFFECTIVE、is_active 为真、effective_from_date 不晚于单据日期、effective_to_date 为空或不早于单据日期。
3. 适用范围判定：ALL_CUSTOMERS 直接命中；SPECIFIED_CUSTOMER_TYPES 时判定客户类型是否落在 scope_customer_types 中；SPECIFIED_CUSTOMERS 时按 ix_price_list_customer_links_legal_entity_id_price_list_id_customer_id 做存在性判定。
4. 命中数为 0 时返回空列表与 no_hit 标志，界面显式提示未命中价目表，单价留空，不阻断建单。
5. 命中数为 1 时返回该行的单价、价格下限与含税标记作为默认值。
6. 命中数大于 1 时返回全部命中行并置 requires_manual_selection 为真，按价目表编码升序排列。该排序只保证输出稳定，不表示优先级。PRD 第 2.8.3 节明确多行命中的优先级规则未定义，在决策落定前系统不得任意取一行。

批量取价把第 1 步的输入表达为一个 VALUES 列表并与明细行表连接，一次往返完成至多 200 行的取价，避免 200 次往返击穿规格附录 A.1 中销售订单表单打开并带出默认值这一项的通过线。该查询的 EXPLAIN 必须显示明细行表走索引扫描、价目表头走主键扫描、客户范围表走索引扫描，不得出现顺序扫描。

历史成交资料不参与上述默认值计算，只作为并列的参考列表展示，操作者显式选用后才回填，回填后由销售阶段按价格权限规则重新判定，依据 PRD 第 2.8.3 节与第 2.9.3 节。

#### 4.9 价目表失效扫描与资质到期扫描

两个每日任务，由 ep-platform-flow 的定时器触发，在 job-worker 执行，触发与执行必须幂等且可重放。

价目表失效扫描：每日 00:05 中国标准时间触发，按 ix_price_lists_legal_entity_id_status_effective_to_date 取出 status 为 EFFECTIVE 且 effective_to_date 早于服务器自然日的价目表，逐条置为 EXPIRED 并发 cpq.price_list.expired.v1。服务器自然日按基线第 3.4 节用 (now() AT TIME ZONE 'Asia/Shanghai')::date 取值。幂等由目标状态判定保证，重复执行不产生第二条事件。

资质到期扫描：每日 00:10 触发，按 ix_supplier_qualifications_legal_entity_id_valid_to_date 取出即将到期与已过期的启用资质，把供应商的 qualification_status 置为 EXPIRING 或 EXPIRED，并对新进入 EXPIRING 的供应商向其采购负责人发一条站内通知。提前天数由配置项承载，取值待决，见第 13 节。证照过期不自动停用供应商档案，依据 PRD 第 2.4.2 节。资质过期是否阻断新建采购订单属采购阶段，本阶段只把 qualification_status 经 ep-contract-mdm 暴露出去。

#### 4.10 导入批次算法

1. 操作者下载模板，模板由 ep-adapter-doc 经 DocTemplatePort::render 与 SpreadsheetPort::write_xlsx 按对象类型生成，列表达为 ColumnSpec，列与 PRD 第 2.3 至 2.8 小节字段表一一对应，必填列由 ColumnSpec::required 标注，模板携带一个版本串写入首行隐藏单元格。
2. 上传文件经 platform-file 的上传流水线落为附件对象，随后创建 import_batches 一行，状态 UPLOADED。
3. job-worker 取件，状态置 PARSING，经 SpreadsheetPort::read_xlsx 解析全部行，期望列以 ColumnSpec 数组传入。模板版本与当前版本不一致即整批失败并返回 MDM.IMPORT_BATCH.TEMPLATE_MISMATCH。行数超过配置上限即整批失败并返回 MDM.IMPORT_BATCH.ROW_LIMIT_EXCEEDED。
4. 逐行执行与界面单条录入完全相同的校验：必填、格式、唯一性、引用存在性、法人归属、权限。批量导入不豁免权限校验、审计记录与唯一性校验，依据 PRD 第 2.11.1 节第 2 条。行内唯一性还要与本批次内已通过的行互相比对，避免同一批次内自相重复。
5. 全部行校验完毕后状态置 PARSED，写入 import_batch_rows，通过行与错误行各写一行。错误行一律不入库为档案，只留在 import_batch_rows。
6. 状态置 DRAFTING，把通过行按每 500 行一个事务落为档案草稿，写回 created_object_id。中途失败时已落库的草稿保留，批次状态置 FAILED 并记录已落库行数，操作者可对该批次发起续跑，续跑按 created_object_id 为空的行继续，因此续跑幂等。
7. 状态置 DRAFTED。操作者调用批量提交审批端点，通过行的草稿按同一状态机与同一审批链批量生成 CREATION 变更申请，导入不构成审批豁免路径，依据 PRD 第 2.11.1 节第 4 条。
8. 错误行清单由 ep-adapter-doc 经 SpreadsheetPort::write_xlsx 渲染为可下载文件，逐行标注行号、字段与失败原因。
9. 导入批次与执行结果按规格第 12.5 章写入审计，发 mdm.import_batch.completed.v1。

#### 4.11 导出算法

导出一律走异步任务。创建 export_jobs 一行，job-worker 按 filter 与 column_keys 取数，取数前由 ep-platform-authz 裁剪列：无权字段不进入 column_keys，也不出现在导出文件中。若裁剪后的列集合与敏感字段清单有交集，则 includes_sensitive_fields 置真，该任务必须携带有效的 X-Reauth-Token 与审批引用才能创建，按规格第 12.1 章敏感数据导出的高风险操作口径执行。行数上限沿用基线第 11.5 节的 50000 行，超出即拒绝并提示收窄筛选条件。结果文件经 platform-file 落为附件对象，回执由站内通知送达。

导出文件与错误清单一律经第 4.15 节冻结的文档端口渲染：表格形态经 SpreadsheetPort::write_xlsx，需要 PDF 形态时经 PdfRenderPort::render_pdf 并传入 PrintLayout，模板套用经 DocTemplatePort::render，三者由本阶段按裁定 A-08 交付，阶段 6、10、11、13 只在其上增量取值，不新增 trait。敏感字段清单的判定取 platform_core.sensitive_field_registry 中 schema_name 为 mdm 的登记行，按裁定 A-28 执行。

#### 4.12 历史成交资料聚合

历史成交资料不是本阶段的表，是一次跨模块聚合查询，取数键与来源按 PRD 第 2.9 节。

- 销售侧取数键为客户加产品，来源为合同行、销售订单行及其关联的交付确认与销项发票登记结果。
- 采购侧取数键为供应商加物料，来源为采购订单行、采购发票行与本阶段的 mdm.supplier_price_records。

实现方式为消费方定义端口：ep-contract-mdm 定义 SalesTradeHistoryProvider 与 PurchaseTradeHistoryProvider 两个 trait 与 TradeHistoryProviderRegistry 注册表，两个 trait 的方法固定为 module_code() 与 recent(ctx, customer_id 或 supplier_id, item_id, limit)，返回统一的 TradeHistoryItem（单据类型、单据编号、单据日期、数量、计量单位、单价、含税标记、当前状态、来源模块）。实现清单按裁定 A-15 固定为四个：阶段 6 的 SalesTradeHistoryProviderImpl、阶段 10 的 InvoiceSalesTradeHistoryProvider、阶段 7 的 ProcureTradeHistoryProvider、阶段 10 的 InvoicePurchaseTradeHistoryProvider，各自在本阶段之后于两个 wiring.rs 中注册到该注册表，本阶段不代做任何一份。ep-app-mdm 的用例把注册表中已注册的实现依次调用并按单据日期倒序合并，条数与时间窗口由配置项限制。采购侧的 mdm.supplier_price_records 是本模块自有表，由 ep-app-mdm 直接读取，不经注册表；因此阶段 5 交付时注册表为空而端点可用，返回内容只有供应商价格资料一项。历史成交资料的完整性验收顺延到阶段 10。

结果一律只读，权限按规格第 7.7 章的法人行级隔离与第 12.2 章的记录级与字段级权限裁剪，跨法人不可见。

#### 4.13 引用计数与可引用性判定

ep-contract-mdm 定义 MasterReferenceCounter trait 与 MasterReferenceCounterRegistry 注册表，trait 的方法固定为 module_code() 与 count_open_documents(ctx, object_kind: MasterObjectKind, object_id)，MasterObjectKind 取 Customer、Supplier、Material、Product、PriceList 五值，返回某档案在某模块下的未完成单据数量。实现清单按裁定 A-15 固定为七个：阶段 6 的 ClmReferenceCounter 与 SalesReferenceCounter、阶段 7 的 ProcureReferenceCounter、阶段 8 的 InventoryReferenceCounter、阶段 10 的 InvoiceReferenceCounter 与 FinanceReferenceCounter、阶段 12 的 ServiceReferenceCounter，各自在本阶段之后注册，本阶段不代做任何一份。停用提交界面把注册表中已注册实现的计数求和展示，计数覆盖的模块清单由注册表实时枚举，未注册的模块不计入并显式列为未覆盖，避免用户误以为为零即无引用。停用引用计数的完整性验收顺延到阶段 12 结束。

可引用性判定由 MasterDataLookup::assert_referenceable 提供，判定规则为 status 等于 EFFECTIVE 且 is_active 为真，产品另加 is_sellable 为真才可进入合同行、销售订单行与价目表明细行。不满足时返回 BUSINESS_CONFLICT 与 MDM.<RESOURCE>.NOT_REFERENCEABLE，details 中给出停用时间。这是 PRD 第 2.10 节停用后对新建单据一律拒绝的唯一强制点。该判定与 MasterDataLookup::resolve_reference、MasterDataLookup::load_version 一并列入第 4.15 节的对外契约端口清单，跨模块调用一律经该清单中的 trait，其他模块不得直接读 mdm 与 cpq 的表，依据基线第 1.3 节。

#### 4.14 客户 360 视图的数据基础

本节之后新增第 4.15 节，集中列出本阶段对外冻结的契约端口，避免同一事物在多处出现两套名字。

ep-contract-crm 定义 Customer360SectionProvider trait，含 section_key、section_title、load(ctx, customer_id, limit) 三项，名字按裁定 C-09 固定，CustomerPanelProvider 作废。ep-app-crm 的 query_customer_360 用例组装两部分：一是从 ep-contract-mdm 取得的档案区块，含编码、名称、客户类型、责任人、联系人、开票要素可见部分、信用额度、状态与版本号；二是已注册的其他区块。未注册的区块不出现在响应中，响应的 meta 中列出当前可用区块清单，供客户端判断是哪些模块尚未启用。唯一端点为 GET /api/v1/crm/customers/{id}/customer-360，本阶段交付时该路径已启用并只挂载 mdm 自己的区块，阶段 12 接管后追加其余区块，不新增路径，不保留 /overview。

#### 4.15 本阶段对外冻结的契约端口清单

下列 trait 由本阶段在 ep-contract-mdm、ep-contract-cpq 与 ep-contract-crm 中定义并冻结，跨模块调用一律经这些 trait。事务句柄类型为 ep_foundation::port::Tx，快照上下文为 ep_foundation::port::SnapshotCtx，两者由阶段 1 按裁定 A-01 冻结，本阶段只引用不重定义；跨模块方法签名一律写 &mut dyn Tx。

| trait | crate | 方法 | 实现方 |
|---|---|---|---|
| MasterDataLookup | ep-contract-mdm | resolve_reference、load_version、assert_referenceable | 本阶段 |
| MasterReferenceCounter | ep-contract-mdm | module_code、count_open_documents | 阶段 6、7、8、10、12 按裁定 A-15 |
| SalesTradeHistoryProvider | ep-contract-mdm | module_code、recent | 阶段 6 与阶段 10 按裁定 A-15 |
| PurchaseTradeHistoryProvider | ep-contract-mdm | module_code、recent | 阶段 7 与阶段 10 按裁定 A-15 |
| MaterialUsageProbe | ep-contract-mdm | has_stock_movement | 阶段 8 的 InventoryMaterialUsageProbe |
| ProductUsageProbe | ep-contract-mdm | is_referenced_by_effective_sales | 阶段 6 的 ClmProductUsageProbe 与 SalesProductUsageProbe |
| SupplierRiskRecordPort | ep-contract-mdm | append、list | 本阶段 |
| SupplierSelfServiceCommand | ep-contract-mdm | submit_profile_change、upload_qualification | 本阶段 |
| PriceResolver | ep-contract-cpq | resolve_batch | 本阶段 |
| Customer360SectionProvider | ep-contract-crm | section_key、section_title、load | 本阶段与阶段 12 |

两个由本阶段实现的跨模块端口签名按裁定 C-10 与 B-10 逐字固定，其他阶段不得改写。

- SupplierRiskRecordPort::append(tx: &mut dyn Tx, ctx: &SecurityContext, supplier_id: Id<Supplier>, record: SupplierRiskRecord) -> Result<(), AppError> 与 SupplierRiskRecordPort::list(tx: &mut dyn Tx, ctx: &SecurityContext, supplier_id: Id<Supplier>) -> Result<Vec<SupplierRiskRecord>, AppError>，写入落在 mdm.supplier_risk_records。
- SupplierSelfServiceCommand::submit_profile_change(&self, tx: &mut dyn Tx, ctx: &SecurityContext, supplier_id: Id<Supplier>, patch: SupplierProfilePatch) -> Result<SupplierChangeRequestView, AppError> 与 SupplierSelfServiceCommand::upload_qualification(&self, tx: &mut dyn Tx, ctx: &SecurityContext, supplier_id: Id<Supplier>, doc: QualificationUpload) -> Result<(), AppError>，两者一律生成待审批的变更申请，不直接写档案。

税率不在本清单内。本阶段不对外提供税率查询端口，默认税率的唯一取用入口是阶段 10 的 ep_contract_invoice::TaxRateOptionQuery；阶段 10 交付前由本阶段的字典桩 MdmTaxRateStub 承担临时取值，阶段 10 交付时执行 V…__invoice_backfill_migrate_tax_rates_from_mdm.sql 并删除该桩，按裁定 C-11 执行。

### 5 API 契约

全部端点遵循基线第 5 节：路径前缀 /api/v1，字段 snake_case，封套固定，分页排序过滤参数固定，全部写请求必带 Idempotency-Key，鉴权头固定集合。下表中的权限一列写的是所需的对象级权限动作，记录级与字段级由 ep-platform-authz 在用例内判定。

每条路由另按裁定 A-20 在 crates/contract/<module>/src/capability.rs 中声明一对 <USECASE_SCREAMING>_DOMAIN 与 <USECASE_SCREAMING>_ACTION 常量，取值来自阶段 1 冻结的 foundation::CapabilityDomain 与 foundation::ActionClass。本阶段路由涉及的能力域为 MdmMasterData、CrmCustomer360、PlatformFullTextSearch 与 PlatformDocumentAttachment，价目表与取价路由取 SalesOrderFulfillment；动作类别按 Read、Write、Submit、Approve、Export 五类逐路由取值。xtask configdoc 断言每个 /api/v1/ 路由都能解析到一对常量，缺失即构建失败；ci-probe feature 门控的探针路由与 /internal/v1/ 下不对四端暴露的内部端点不参与判定，不声明常量。本阶段只引用 foundation::CapabilityDomain，不重新定义能力域码。

#### 5.1 四类档案的通用端点

下表以 customers 为例，suppliers、materials、products 三类路径与语义完全同构，只把资源段与错误码的资源段替换。

| 方法与路径 | 请求 | 响应 | 主要错误码 | 幂等 | 权限 |
|---|---|---|---|---|---|
| GET /api/v1/mdm/customers | 分页排序过滤参数，默认排序 code asc | data 为客户摘要数组，meta 为分页信息 | PLATFORM.AUTHZ.OBJECT_FORBIDDEN | 无 | mdm.customer.read |
| GET /api/v1/mdm/customers/{id} | 无 | data 为客户完整视图，含子表与附件关联 | PLATFORM.AUTHZ.NOT_FOUND_OR_DENIED | 无 | mdm.customer.read |
| GET /api/v1/mdm/customers/{id}/versions | 分页参数 | data 为版本摘要数组 | 同上 | 无 | mdm.customer.read |
| GET /api/v1/mdm/customers/{id}/versions/{version_no} | 无 | data 为该版本快照 | MDM.CUSTOMER.VERSION_NOT_FOUND | 无 | mdm.customer.read |
| POST /api/v1/mdm/customers | 客户草稿完整结构，code 可省略表示自动生成 | data 为新建草稿 | MDM.CUSTOMER.CODE_DUPLICATED、MDM.CUSTOMER.USCC_CHECKSUM_INVALID、MDM.CUSTOMER.REFERENCE_UNAVAILABLE | 必填 | mdm.customer.create |
| PATCH /api/v1/mdm/customers/{id} | 部分字段与 row_version | data 为更新后草稿 | PLATFORM.CONCURRENCY.STALE_VERSION、MDM.CUSTOMER.INVALID_TRANSITION | 必填 | mdm.customer.update |
| POST /api/v1/mdm/customers/{id}/actions/submit-for-approval | row_version、name_duplicate_confirmed、name_duplicate_note、reason | data 为生成的变更申请 | MDM.CUSTOMER.NAME_DUPLICATE_UNCONFIRMED、MDM.CHANGE_REQUEST.ALREADY_OPEN | 必填 | mdm.customer.submit |
| POST /api/v1/mdm/customers/{id}/actions/void | row_version、reason | data 为作废后的档案 | MDM.CUSTOMER.INVALID_TRANSITION | 必填 | mdm.customer.void |

#### 5.2 变更申请端点

| 方法与路径 | 说明 | 主要错误码 |
|---|---|---|
| POST /api/v1/mdm/change-requests | 对已生效档案发起 FIELD_CHANGE、DEACTIVATION 或 REACTIVATION 申请，请求含 object_type、object_id、change_kind、proposed、reason | MDM.CHANGE_REQUEST.ALREADY_OPEN、MDM.MASTER_RECORD.FROZEN_FIELD_MODIFIED、MDM.MATERIAL.STOCK_MOVEMENT_EXISTS、MDM.PRODUCT.SALES_REFERENCE_EXISTS |
| GET /api/v1/mdm/change-requests | 列表，支持按 object_type、status、object_id 过滤 | |
| GET /api/v1/mdm/change-requests/{id} | 详情，含逐字段变更前后对照 | PLATFORM.AUTHZ.NOT_FOUND_OR_DENIED |
| PATCH /api/v1/mdm/change-requests/{id} | 编辑 DRAFT 状态的申请 | PLATFORM.CONCURRENCY.STALE_VERSION |
| POST /api/v1/mdm/change-requests/{id}/actions/submit-for-approval | 提交到审批链 | MDM.CHANGE_REQUEST.SELF_APPROVAL_FORBIDDEN、MDM.CHANGE_REQUEST.BASE_DRIFTED |
| POST /api/v1/mdm/change-requests/{id}/actions/withdraw | 申请人撤回 | MDM.CHANGE_REQUEST.INVALID_TRANSITION |
| POST /api/v1/mdm/change-requests/{id}/actions/reapply | 审批已通过但应用失败进入死信后的记名重放入口 | MDM.CHANGE_REQUEST.INVALID_TRANSITION |

审批的通过与退回动作不在本阶段的端点上，由平台统一审批端点承载。平台审批完成后写出平台侧的审批完成事件，ep-app-mdm 在 job-worker 中订阅并应用结论。本阶段不提供任何绕过审批链直接置生效的端点。申请人不可自审由 ep-platform-authz 的职责分离判定，本阶段只负责在提交时把申请人写入流程实例的发起人字段。

#### 5.3 供应商侧四类记录端点

| 方法与路径 | 说明 |
|---|---|
| POST、PATCH、GET /api/v1/mdm/suppliers/{id}/qualifications | 资质证照的登记、修改与查询，修改走变更申请 |
| POST、PATCH、GET /api/v1/mdm/suppliers/{id}/price-records | 供应商价格资料 |
| POST、PATCH、GET /api/v1/mdm/suppliers/{id}/leadtime-records | 交期资料 |
| POST、PATCH、GET /api/v1/mdm/suppliers/{id}/risk-records | 质量与风险记录 |

资质证照的变更纳入供应商档案的变更申请快照，其余三类记录不进入档案版本，可独立维护并单独写审计，理由是价格、交期与风险记录的变更频度远高于档案本体，纳入档案版本会使版本号迅速膨胀且审批链不适用。该分工为本阶段新增决定。

#### 5.4 价目表端点

| 方法与路径 | 说明 | 主要错误码 |
|---|---|---|
| GET、POST、PATCH /api/v1/cpq/price-lists 与 /{id} | 价目表头的查询、建档与草稿编辑 | CPQ.PRICE_LIST.PERIOD_INVALID、CPQ.PRICE_LIST.SCOPE_CUSTOMER_REQUIRED |
| GET、POST、PATCH /api/v1/cpq/price-lists/{id}/lines | 明细行维护 | CPQ.PRICE_LIST_LINE.DUPLICATED、CPQ.PRICE_LIST_LINE.FLOOR_PRICE_ABOVE_UNIT_PRICE、CPQ.PRICE_LIST_LINE.PRODUCT_NOT_SELLABLE |
| POST /api/v1/cpq/price-lists/{id}/actions/submit-for-approval | 提交审批 | CPQ.PRICE_LIST.INVALID_TRANSITION |
| POST /api/v1/cpq/price-lists/{id}/actions/void | 作废未生效价目表 | CPQ.PRICE_LIST.INVALID_TRANSITION |
| POST /api/v1/cpq/price-quotes/actions/resolve-batch | 批量取价，单次至多 200 行，请求为 (customer_id, product_id, uom_id, quote_date) 数组 | CPQ.PRICE_QUOTE.LINE_LIMIT_EXCEEDED |

取价端点无副作用但仍按基线第 5.4 节必填并落库幂等键，重复请求直接回放。该开销可接受：一次表单打开只产生一条幂等键，7 天保留期内的总量在 20 并发规模下不构成负担。取价响应每行的结构为 { product_id, uom_id, hits: [PriceHit], requires_manual_selection: bool }。

#### 5.5 引用校验、历史成交与导入导出端点

| 方法与路径 | 说明 |
|---|---|
| POST /api/v1/mdm/master-records/actions/assert-referenceable-batch | 单据侧批量校验引用可用性，返回逐条结论与不可用原因 |
| GET /api/v1/mdm/trade-histories | 历史成交资料查询，参数 side 取 SALES 或 PURCHASE，另带 customer_id 与 product_id 或 supplier_id 与 material_id |
| GET /api/v1/mdm/import-templates/{object_type} | 下载导入模板 |
| POST /api/v1/mdm/import-batches | 创建导入批次 |
| GET /api/v1/mdm/import-batches/{id} | 批次进度与结果 |
| GET /api/v1/mdm/import-batches/{id}/error-rows | 错误行清单，支持分页与下载 |
| POST /api/v1/mdm/import-batches/{id}/actions/submit-drafts-for-approval | 通过行草稿批量提交审批，单次至多 200 条按基线第 5.1 节批量上限拆批 |
| POST /api/v1/mdm/exports | 创建导出任务，含敏感字段时必带 X-Reauth-Token 与审批引用 |
| GET /api/v1/mdm/exports/{id} | 导出任务回执 |
| GET /api/v1/crm/customers/{id}/customer-360 | 客户 360 概览，按裁定 C-09 是唯一端点，与阶段 12 共用，不保留 /overview |

#### 5.6 存在性泄漏与错误封套

对当前安全上下文不可见的记录，读、写一律返回 404 与 PLATFORM.AUTHZ.NOT_FOUND_OR_DENIED，不区分不存在与无权。只有当前用户对该对象类型完全无权时返回 403 与 PLATFORM.AUTHZ.OBJECT_FORBIDDEN。全部错误响应携带 incident_no、occurred_at、retryable、advice 四项，message 与 advice 为简体中文，不出现堆栈、SQL、表名与主机名。

### 6 并发与事务边界

#### 6.1 事务划分

| 用例 | 事务内容 | 隔离级别 |
|---|---|---|
| 创建档案草稿 | 取号、插入档案头与子表、写审计 | READ COMMITTED |
| 编辑草稿 | 按 row_version 更新档案头与子表、写审计 | READ COMMITTED |
| 提交审批 | 加载档案 for update、校验、插入 change_requests、更新档案 status、发起流程实例、写审计 | READ COMMITTED |
| 撤回或作废 | 更新 change_requests、更新档案 status、写审计 | READ COMMITTED |
| 应用审批结论 | 插入 inbox_consumptions、加载档案 for update、比对 base_row_version 与 diff、应用变更、写 record_versions、更新 change_requests、写审计、写 Outbox | READ COMMITTED |
| 价目表建档与提交 | 与档案同构，明细行整体替换 | READ COMMITTED |
| 取价 | 单条只读语句，不开写事务 | READ COMMITTED |
| 导入解析 | 每 500 行一个事务，批次头状态更新单独事务 | READ COMMITTED |
| 导出渲染 | 取数为只读事务，文件写入在事务外，结果登记为独立写事务 | READ COMMITTED |
| 失效扫描与到期扫描 | 每 100 条一个事务 | READ COMMITTED |

事务预算沿用基线第 10.3 节：业务事务不超过 5 秒，读写池 statement_timeout 10 秒，lock_timeout 3 秒。job-worker 池 statement_timeout 300 秒，导入解析的单批事务在该预算内。事务内禁止外部 HTTP 调用、文件正文读写、发送通知与长时计算，本阶段的四处涉及文件正文的地方（导入源文件读取、错误清单渲染、导出文件渲染、模板生成）一律在事务外完成，只把附件对象 ID 写入事务。

事务的开启与提交一律经 ep_foundation::port::UnitOfWork 的 transact 与 snapshot_transact 两个方法，只读快照事务的唯一入口是 snapshot_transact；ep-app-mdm、ep-app-cpq 与 ep-app-crm 对 UnitOfWork 取泛型参数 U: UnitOfWork 而不是 trait 对象，理由是该 trait 含泛型方法不满足对象安全。跨模块端口的事务句柄参数一律写 &mut dyn Tx，取具体连接的 downcast 只允许出现在 crates/adapter/db-pg 内，按裁定 A-01 执行。

#### 6.2 锁策略

档案更新一律先 select ... for update 加载聚合行，再更新头与子表，避免子表更新与头更新之间的交错。更新语句一律带 row_version 条件，受影响行数为 0 即判定版本冲突，映射为 PLATFORM.CONCURRENCY.STALE_VERSION 与 HTTP 409，响应回带当前版本号与最后修改人。

同一档案的变更申请单例不用先查后插，而由 ux_change_requests_legal_entity_id_open_slot 强制，唯一冲突映射为 MDM.CHANGE_REQUEST.ALREADY_OPEN，响应中给出在途申请的编号与当前审批节点。

导入的批量草稿落库不加聚合锁，唯一性冲突由唯一索引拦截并落为该行的错误结果，不回滚整批。

#### 6.3 幂等键与 Outbox

全部写端点必带 Idempotency-Key，头的存在性与 UUIDv7 合法性由阶段 1 的 IdempotencyKeyHeaderGuard 校验，重放判定经阶段 2 定义的 ep_adapter_db::port::IdempotencyStore 的 try_begin 与 finish 两个方法，落库表为阶段 3a 的 platform_msg.idempotency_keys；幂等作用域为法人、用户、端点、键值四元组，幂等键写入与业务写入同事务，本阶段不自建第三处判等，按裁定 C-07 执行。

审批完成事件的消费幂等由 platform_msg.inbox_consumptions 的唯一约束保证，消费副作用与该行插入同事务。本阶段的消费者标识固定为 mdm.change_request_applier 与 mdm.search_indexer 两个，后者按裁定 A-07 消费本阶段的档案变更事件并经 ep_foundation::port::search::SearchIndexPort 写索引，索引写入不在业务事务内进行。

本阶段的领域事件与业务状态、审计事件写入同一事务，禁止在事务提交前发起任何外部调用。事件投递语义为至少一次，重试退避按基线第 6.2 节的 8 次序列，全部失败后进入死信。本阶段的事件不带 posting_date 与 accounting_period_id，理由是主数据不产生过账条目；且按裁定 A-21 本阶段不向 ledger.posting_trigger_event_types 登记任何行，因此本阶段的事件不落入关账受理前提二的统计口径，该口径由阶段 9a 按裁定 C-28 定义。该点必须在事件目录中显式标注，避免关账实现误把主数据事件计入。

#### 6.4 失败重试与补偿

序列化失败 40001 与死锁 40P01 由数据访问层重试 3 次，退避 50、150、450 毫秒，只对尚未产生外部可见副作用的事务重试。唯一冲突与版本冲突不重试。

审批结论应用失败的补偿路径：Outbox 重试耗尽后进入死信，死信条目携带 change_request_id，人工在界面上经 POST /api/v1/mdm/change-requests/{id}/actions/reapply 记名重放，重放走同一幂等路径，重复重放不产生第二个版本。丢弃死信需要双人审批，按基线第 6.2 节。

导入批次中途失败的补偿路径：已落库草稿保留，批次置 FAILED，操作者续跑，续跑按 created_object_id 为空的行继续，因此补偿是幂等的续做而非回滚。选择续做而非整批回滚的理由是 5000 行规模下整批回滚会把已通过校验的工作全部作废，且 PRD 第 2.11.1 节只要求错误行不入库，未要求通过行整体回滚。

### 7 配置项

全部新增配置键前缀 EP__，层级用双下划线，结构体开启 deny_unknown_fields。运行期可变的业务参数不进配置文件，本节列出的都是不随法人变化的技术参数。

| 键 | 类型 | 默认值 | 生效方式 | 说明 |
|---|---|---|---|---|
| EP__MDM__IMPORT__MAX_ROWS | u32 | 5000 | 重启生效 | 单次导入最大行数，取值对齐规格附录 A.3 每类主数据 5000 条 |
| EP__MDM__IMPORT__BATCH_SIZE | u32 | 500 | 重启生效 | 草稿落库的单事务行数 |
| EP__MDM__IMPORT__TEMPLATE_VERSION | string | 与二进制版本绑定 | 重启生效 | 模板版本串，不一致即整批拒绝 |
| EP__MDM__CODE__ALLOW_MANUAL | bool | true | 重启生效 | 档案编码是否允许人工指定 |
| EP__MDM__USCC__CHECKSUM_ENABLED | bool | true | 重启生效 | 是否执行统一社会信用代码校验位校验 |
| EP__MDM__USCC__EXEMPT_CUSTOMER_TYPES | string 数组 | ["INDIVIDUAL","OVERSEAS"] | 重启生效 | 免校验的客户类型码，临时取值 |
| EP__MDM__NAME_DUPLICATE__PROBE_LIMIT | u32 | 20 | 重启生效 | 同名提示中列出的档案条数上限 |
| EP__MDM__QUALIFICATION__EXPIRY_LEAD_DAYS | u32 | 30 | 重启生效 | 资质到期提醒提前天数，临时取值，U-A-11 决策后改为低代码定时器配置 |
| EP__MDM__QUALIFICATION__SCAN_ENABLED | bool | true | 重启生效 | 资质到期扫描开关 |
| EP__MDM__TRADE_HISTORY__MAX_ROWS | u32 | 20 | 重启生效 | 历史成交资料展示条数上限，临时取值 |
| EP__MDM__TRADE_HISTORY__WINDOW_MONTHS | u32 | 12 | 重启生效 | 历史成交资料时间窗口，临时取值 |
| EP__MDM__TRADE_HISTORY__INCLUDE_VOIDED | bool | false | 重启生效 | 是否包含已作废、已红冲与已退货单据，临时取值 |
| EP__MDM__FREEZE__REQUIRE_PROBE_WHEN_MODULE_ENABLED | bool | true | 重启生效 | 经 ModuleLicenseQuery::module_state 判定相关模块已启用而探针未注册时是否拒绝启动 |
| EP__CPQ__PRICE_RESOLVE__MAX_LINES | u32 | 200 | 重启生效 | 批量取价单次行数上限，与基线第 5.1 节批量上限一致 |
| EP__CPQ__PRICE_LIST__EXPIRY_SCAN_ENABLED | bool | true | 重启生效 | 价目表失效扫描开关 |

两个扫描任务的触发时刻不做成配置项，而是作为流程定义中的定时器条目由阶段 3b 交付的 ep-platform-release 配置发布通道下发，本阶段不自建第二套发布路径，依据基线第 7.1 节运行期可变的业务参数不进配置文件与裁定 A-27。

新增指标登记如下，均在 ops-agent 的 127.0.0.1:9101 暴露，标签遵守基线第 9.2 节的基数纪律。

| 指标 | 类型 | 标签 |
|---|---|---|
| ep_mdm_change_requests_open | gauge | legal_entity_id、object_type |
| ep_mdm_import_rows_total | counter | object_type、outcome |
| ep_mdm_qualification_expired_total | gauge | legal_entity_id |
| ep_cpq_price_resolve_duration_seconds | histogram | 无 |
| ep_cpq_price_resolve_hit_count | histogram | 无 |

日志的 operation 字段取 <module>.<usecase>，例如 mdm.submit_customer_for_approval、cpq.resolve_price_batch。开票要素与银行账号一律经 foundation::Redacted 包装，不进入日志、错误消息与指标标签。

### 8 测试计划

#### 8.1 单元测试

位于 ep-domain-mdm 与 ep-domain-cpq 内，不触网、不触库、不触文件系统、不取真实时间，时间经 FixedClock 注入。覆盖分支如下。

- 档案状态机：第 4.2 节表中的每一条合法流转各一例，另加每个状态下的全部非法流转各一例，共 11 条合法流转与 34 条非法流转。
- 价目表状态机：含自动失效流转与从已失效提交变更两条特有路径。
- 冻结字段判定：五类对象的永久冻结字段各一例，物料三项与产品一项的条件冻结在探针返回真与返回假两种情形下各一例，探针缺位时的放行一例。
- 统一社会信用代码校验：30 组正样本、10 组单字符篡改样本、长度不足、含非法码字、校验值为 31 的边界样本各一例。
- 编码生成：自动生成、人工指定、流水溢出到 7 位、法人码两位补零四例。
- 版本差异计算：头字段变更、子表新增行、子表停用行、子表重排序、附件关联变更、无差异提交六例，另加同一份数据两次计算路径稳定性一例。
- 取价命中：零命中、单命中、多命中、生效期左右边界日、已停用价目表、已失效价目表、明细行已停用、产品不可销售、适用范围三种取值各一例，共 14 例。
- 空槽唯一索引的槽位计算：默认联系人置位与取消、明细行停用与启用、变更申请开启与结束六例。
- 文本长度与数值范围校验：基线第 11.2 节七类长度上限各一例边界，税率的 0 与接近 1 两个边界。

#### 8.2 领域属性测试

用 proptest 表达，是规格第 17.2 章要求的独立测试类型。本阶段虽不承载第 17.3 章的五组账务不变量，仍须表达四条与其前提相关的属性。

1. 版本号单调且连续：对任意合法操作序列，record_versions 中同一 object_id 的 version_no 集合等于 1 至 N 的连续整数集合，且档案上的 version_no 等于 N。
2. 冻结字段不可变：对任意合法操作序列，物料的 base_uom_id、is_batch_managed、is_serial_managed 在探针返回真之后不再变化。该属性是规格第 17.3 章库存数量守恒按仓库、物料、批次逐项核对得以成立的前提。
3. 停用的最小影响：DEACTIVATION 应用前后，除 is_active、deactivated_at、version_no、row_version、updated_at、updated_by 六列外，档案的全部字段与全部子表行逐字段相等。该属性对应 PRD 第 2.10 节停用不触发任何单据状态变化、不产生凭证、不影响任何强制不变量。
4. 取价命中集合的等价性：批量取价的结果与逐条取价的结果逐行相等，且命中集合等于对全部明细行逐条施加命中判定后得到的集合。

#### 8.3 集成测试

使用真实 PostgreSQL 16，每个用例独占一个数据库，用例结束即删库。禁止用内存库或 mock 替代数据库。场景清单如下。

1. RLS 矩阵：tests/rls_matrix 中新增 30 张表的读取、写入、更新、删除、聚合、排序、报表投影与错误信息泄漏八类用例，八个断言函数 assert_read、assert_write、assert_update、assert_delete、assert_aggregate、assert_sort、assert_report_projection、assert_error_leak 由阶段 1 在 testkit/src/rls_matrix.rs 提供，本阶段只补数据与用例，不重复实现同名函数，按裁定 C-05 执行。删除一类验证业务 schema 上的 DELETE 被拒绝。错误信息泄漏一类验证跨法人访问返回 404 而非 403，且响应中不含目标记录的任何字段值。
2. 会话变量缺失时的默认拒绝：连接未设置 app.legal_entity_id 时对 30 张表的读写全部返回零行或被拒绝。
3. 唯一性并发：20 个并发事务同时以同一编码建档，恰好 1 个成功，其余 19 个返回 MDM.CUSTOMER.CODE_DUPLICATED，无一例返回内部错误。
4. 变更申请单例并发：20 个并发请求同时对同一档案发起变更申请，恰好 1 个成功，其余返回 MDM.CHANGE_REQUEST.ALREADY_OPEN。
5. 乐观锁冲突：两个会话读取同一档案后先后提交，后者返回 PLATFORM.CONCURRENCY.STALE_VERSION 并回带当前版本号与最后修改人。该项对应基线第 8.4 节六组必测并发场景中的同一单据乐观锁冲突一组。
6. 基线漂移检测：在审批期间由第三方修改档案子表，应用结论时返回 MDM.CHANGE_REQUEST.BASE_DRIFTED。
7. Outbox 重复投递：同一审批完成事件重复投递不少于 3 次，档案版本只增加 1，事件只外发一次，审计只写一条。该项对应基线第 8.4 节的 Outbox 重复投递一组。
8. 导入全流程：5000 行文件，其中 250 行含各类错误，验证错误行不入库、通过行落草稿、行号与字段定位准确、续跑幂等、批次统计与实际条数一致。
9. 导入超限：5001 行文件整批拒绝。
10. 导出裁剪：以无银行字段权限的用户发起导出，结果文件中不出现该两列；以有权限用户发起且未带重新认证凭证时被拒绝。
11. 取价 EXPLAIN：在基准数据集上对批量取价语句执行 EXPLAIN，断言不出现 Seq Scan，输出作为证据归档。
12. 列表查询 EXPLAIN：客户列表的默认排序、按状态过滤、按名称前缀过滤三条语句各断言不出现 Seq Scan。
13. 深偏移拒绝：page 乘 page_size 超过 10000 时服务端拒绝并要求改用键集分页。
14. 跨模块引用校验：以未注册探针与已注册探针两种装配分别运行冻结字段修改，验证第 4.3 节的两种行为。
15. 启动自检：经 ModuleLicenseQuery::module_state 判定 Inventory 为 InstalledEnabled 而未注册 MaterialUsageProbe 时，命名自检项 master-data-usage-probes-registered 失败，进程以退出码 78 退出，失败项名与原因写入 stderr。
16. 迁移可逆：对全部 31 个迁移文件按其 rollback 段执行逆向，再重新执行，数据库结构逐对象比对一致。
17. 附件关联：解除关联后再重新关联同一附件对象成功，验证空槽写法在附件表上的行为。
18. 搜索索引传播：档案生效后 15 分钟内索引可查，停用后索引中的可引用标记同步更新，跨法人检索不返回无权数据，对应规格第 7.9 章的派生存储越权与传播测试。
19. 受治理数据集视图：三个视图在 ep_analyst_ro 角色下可读、在其他只读角色下不可读，返回列含 legal_entity_id、security_level、data_scope_tags 三列，列名与类型签名与阶段 11 的 reporting.dataset_fields 登记一致，按裁定 A-18 执行。
20. 敏感字段登记与盲索引：第 25 号迁移执行后 platform_core.sensitive_field_registry 中存在 mdm.customer_invoice_profiles 与 mdm.supplier_payment_profiles 的 bank_name 与 bank_account_no 共四行，四行的 category 均为 ACCOUNT、security_level 均为 30，bank_account_no 两行的 is_field_encrypted 为真且 blind_index 为 EXACT 且 blind_index_column 为 bank_account_no_bidx，bank_name 两行的 is_field_encrypted 为假；两张表上存在 bank_account_no_enc bytea 与 bank_account_no_key_ref text 与 bank_account_no_tail text 三列且不存在同名明文列 bank_account_no，db/checks/11 返回零行；以同一明文两次计算 derive_blind_key 得到相同结果，跨法人得到不同结果，按裁定 A-28 与 B-04 执行。

外部依赖只有电子签章一类，本阶段不涉及，因此本阶段不引入 wiremock 打桩。

#### 8.4 端到端测试

后端 E2E 用 Rust 集成测试直接打 HTTP 接口，桌面端用 Playwright 驱动 WebView 与 tauri-driver，移动端用 XCUITest 与 Espresso 只跑规格第 6.2 章矩阵中 MDM 主数据维护与审批一行取值为简化的场景。

| 编号 | 场景 | 端 |
|---|---|---|
| E5-01 | 客户建档、提交、审批、生效、出现在引用校验结果中 | 桌面端、移动端简化 |
| E5-02 | 已生效客户发起变更、审批、版本递增、历史版本可查、按旧版本号可还原引用时点取值 | 桌面端 |
| E5-03 | 供应商建档含资质附件、准入审批、资质到期扫描置 EXPIRING 与 EXPIRED、站内通知送达采购负责人 | 桌面端、移动端简化 |
| E5-04 | 物料建档、产生流水后修改冻结字段被拒并提示改用新建物料 | 桌面端 |
| E5-05 | 产品建档、关联唯一物料、销售侧带出物料字段并允许改选 | 桌面端 |
| E5-06 | 价目表建档、审批生效、零命中、单命中、多命中三分支的界面表现 | 桌面端 |
| E5-07 | 批量导入 1000 行含 50 行错误、错误清单下载、通过行批量提交审批 | 桌面端，移动端按规格第 6.2 章转桌面端执行 |
| E5-08 | 导出含敏感字段触发重新认证与审批、回执由站内通知送达 | 桌面端 |
| E5-09 | 停用客户后新建引用被拒、存量引用不受影响、停用提示展示引用计数与覆盖模块清单 | 桌面端 |
| E5-10 | 法人 A 的档案在法人 B 上下文不可见、不可写、检索不返回、错误信息不泄露存在性 | 桌面端 |
| E5-11 | 门户提交的供应商自维护变更生成待审批申请，不直接写入档案 | 后端 E2E 加门户桩 |
| E5-12 | 申请人尝试审批自己提交的申请被拒并提示不可自审 | 桌面端 |

本阶段的四端界面按裁定 A-23 由本阶段交付，代码位于 clients/desktop/src/modules/mdm、cpq、crm 与 clients/mobile/src/modules/mdm、cpq、crm 六个目录。规格第 6.2 章能力矩阵中取值为完整或简化的能力域实现完整入口，取值为 VIEW_ONLY 的能力域只实现只读视图，取值为 NOT_APPLICABLE 的不实现入口。E5-04 的冻结字段被拒分支依赖阶段 8 的 InventoryMaterialUsageProbe，E5-05 的产品条件冻结分支依赖阶段 6 的 ClmProductUsageProbe 与 SalesProductUsageProbe，E5-09 的引用计数覆盖模块清单依赖阶段 12 的最后一份 MasterReferenceCounter，三处在本阶段以空实现装配执行，完整断言分别顺延到阶段 8、阶段 6 与阶段 12。

#### 8.5 性能相关项

本阶段承担规格附录 A.1 中的四个度量项与一个子段，统计口径按附录 A.2，样本不少于 200 次，只取负载稳定段，单次运行错误率超过 0.1% 即该次运行无效。

| 度量项 | 通过线 | 本阶段的取数路径 |
|---|---|---|
| 客户列表按条件过滤并翻页 | P95 2 秒 | GET /api/v1/mdm/customers，走 ix_customers 系列索引 |
| 客户详情打开 | P95 2 秒 | GET /api/v1/mdm/customers/{id}，一次主键与四次子表索引扫描 |
| 附件列表加载 | P95 2 秒 | 档案附件关联表按 owner_id 索引扫描加 platform_file 的对象元数据 |
| 全文检索返回首页结果 | P95 2 秒 | 内置搜索索引，本阶段负责档案文档的写入与字段裁剪 |
| 销售订单表单打开并带出默认值中的取价子段 | 该项整体属销售阶段，本阶段单独记录取价子段耗时作为观察项 | POST /api/v1/cpq/price-quotes/actions/resolve-batch，200 行一次往返 |

基准数据集按规格附录 A.3：法人 2 个、客户与供应商与物料各 5000 条、产品 5000 条。ep-datagen 接受 --seed 与 --scale，生成器随本阶段结论一并版本化。

#### 8.6 覆盖率门槛

| 范围 | 门槛 | 依据 |
|---|---|---|
| ep-domain-mdm、ep-domain-cpq | 行覆盖率不低于 85% | 冻结字段判定与取价命中直接决定规格第 17.3 章库存与存货一致性所依赖的分组键稳定性，按强制不变量相关代码取值 |
| ep-app-mdm、ep-app-cpq、ep-app-crm、ep-contract-* | 行覆盖率不低于 70% | 规格第 17.2 章其余代码档 |
| 本阶段新增与修改代码 | 不低于 80% | 规格第 17.2 章 |
| 工作区整体 | 不低于 80% | 基线第 8.2 节 |

工具为 cargo-llvm-cov，CI 上以 --fail-under-lines 强制，路径规则写入 codecov.toml。本阶段不允许存在无 issue 编号的 #[ignore]。

### 9 退出条件

下列各条可客观判定，全部达成才算本阶段完成。

1. 30 张表与 31 个迁移文件在空库上一次执行成功，逆向执行成功，再次正向执行成功，结构逐对象比对一致。
2. 全部 30 张表已 ENABLE 且 FORCE 行级安全，命名自检项 rls-enabled-and-forced 在本阶段表上通过。
3. tests/rls_matrix 中本阶段的八类用例全部通过，零跳过。
4. 第 8.1 至 8.4 节列出的全部用例通过，无长期跳过项。
5. 覆盖率达到第 8.6 节的四档门槛，CI 门禁通过。
6. 第 8.5 节的四个度量项在基准数据集与 20 并发负载下 P95 达标，取价子段的观察值已记录，五条 EXPLAIN 证据已归档且不含 Seq Scan。
7. 依赖方向自检脚本通过：ep-domain-mdm 与 ep-domain-cpq 中不出现 sqlx、reqwest、tokio 的 IO 模块、std::fs、std::net、SystemTime::now、rand 任一符号；ep-app-* 的用例函数中不出现 reqwest 与文件写入符号；除 wiring.rs 外任何地方不出现 use ep_adapter_db_pg。
8. 文件规模纪律通过：本阶段新增文件无一超过 800 行，函数无一超过 50 行，嵌套无一超过 4 层。
9. docs/event-catalog.md 中本阶段的 24 个事件全部登记且与代码常量一致；docs/error-codes.md 中本阶段的错误码全部登记且与 ep-foundation 的 error::codes 一致，CI 一致性校验通过，无重复码；PLATFORM 段的七个平台错误码按裁定 C-24 由阶段 1 登记，本阶段只引用不重复登记。
10. docs/data-dictionary/mdm.md 与 docs/data-dictionary/cpq.md 与实际表结构逐列一致，由 CI 从数据库元数据比对生成校验。
11. 五个新增指标在 ops-agent 端点上可抓取，标签基数符合纪律。
12. ep-datagen 可产出符合规格附录 A.3 主数据规模的数据集，生成器版本已冻结。
13. 一次完整的手工演示可跑通：建立两个法人各自的四类档案与一张价目表，导入 1000 行物料，停用一个客户，导出一份含敏感字段的客户清单，并在法人 B 的上下文下验证法人 A 的全部数据不可见。
14. 本阶段的三处偏离与七项新增决定已回写共享技术基线对应章节，并经评审确认。
15. 本模块在规格第 6.2 章能力矩阵中取值为完整或简化的能力域，其四端界面已实现并通过 Playwright 与 tauri-driver 的桌面用例、XCUITest 与 Espresso 的移动用例；取值为 VIEW_ONLY 的能力域只实现只读视图；取值为 NOT_APPLICABLE 的不实现入口。
16. 本模块数据集视图已发布并授予 ep_analyst_ro，列签名已同步给阶段 11。
17. 本阶段全部路由的能力域码与动作类别常量已声明，常量分别位于 crates/contract/mdm/src/capability.rs、crates/contract/cpq/src/capability.rs 与 crates/contract/crm/src/capability.rs，xtask configdoc 通过；按裁定 A-20 阶段 6 只对 crates/contract/cpq/src/capability.rs 追加常量，不重定义本阶段已声明的常量。
18. crates/foundation/src/port/doc.rs 的五个类型与三个 trait 已按裁定 A-08 冻结，ep-adapter-doc 的实现覆盖导入模板生成、错误行清单渲染与 XLSX 读写三项，后续阶段无需新增 trait。
19. mdm.classification_items 中不存在 TAX_RATE_PRESET 取值，字典桩 MdmTaxRateStub 已交付并在代码注释与本计划中标注其撤销时点为阶段 10 交付 invoice.tax_rate_options 之日。
20. platform_core.sensitive_field_registry 中存在 mdm.customer_invoice_profiles 与 mdm.supplier_payment_profiles 的 bank_name 与 bank_account_no 共四行，bank_account_no 两行的 is_field_encrypted 为真、bank_name 两行为假；两张表上不存在同名明文列 bank_account_no，bank_account_no_enc 为 bytea 且 bank_account_no_key_ref 与 bank_account_no_tail 两列齐备，db/checks/11 返回零行；本阶段不引用 platform_meta 的任何对象。
21. 顺延项已登记且各有承接阶段：物料三项条件冻结的完整性验收顺延到阶段 8，产品一项顺延到阶段 6，停用引用计数完整性顺延到阶段 12 结束，历史成交资料完整性顺延到阶段 10，四处均已在本阶段以空实现装配并留 TODO(stage-<n>) 注释。

### 10 与规格和 PRD 的对应

#### 10.1 规格条目

| 规格章节 | 本阶段实现的内容 |
|---|---|
| 5.1 平台内核 | 作为使用方接入编号、Outbox、幂等、通知、审计、文件引用六项能力；不实现这些能力本身 |
| 5.2 MDM 条目 | 客户、供应商、物料、产品四类主数据的编码、责任人、版本与变更审批全量实现；单位以最小档案实现；地点不实现；首版不含质量评分、数据血缘、记录合并去重、跨法人分发四项按不实现处理 |
| 5.2 CRM 条目 | 客户档案本体实现；客户 360 视图交付 Customer360SectionProvider 区块注册契约与 GET /api/v1/crm/customers/{id}/customer-360 端点骨架，历史合同、回款、投诉、设备与服务记录五类区块由阶段 12 按裁定 C-09 在同一端点上扩充 |
| 5.2 CPQ 条目 | 产品价目实现；价格权限校验与折扣审批不实现，只暴露价格下限供其取用 |
| 5.2 采购与 SRM 条目 | 供应商档案与资质、价格、交期、质量、风险五类记录实现；准入即档案生效审批；询比价、招投标、VMI 与绩效考核模型不实现 |
| 5.5 全文检索条目 | 四类档案与价目表的关键字检索文档写入与字段裁剪实现 |
| 5.5 供应商门户条目 | 只实现供应商自维护变更申请的服务端命令端口 ep_contract_mdm::SupplierSelfServiceCommand，门户入口不实现 |
| 6.2 能力矩阵 | MDM 主数据维护与审批一行的桌面端完整与移动端简化取值按矩阵执行；表格能力一行的导入导出按只有导入与导出、移动端转桌面端执行；本模块四端界面按裁定 A-23 由本阶段交付，阶段 13 只交付客户端壳与能力矩阵闸 |
| 7.1 事务数据 | 法人 ID 携带、UTC 存储、人民币单一币种在本阶段全部表上执行 |
| 7.2 数据所有权 | mdm 为四类主数据的唯一权威写入者；本阶段不写总账、不写库存、不写发票台账 |
| 7.4 可定制数据库 | 本阶段全部迁移落在公共能力基线内的类型与索引，全部属在线变更范围，无停机窗口操作 |
| 7.5 文件与归档 | 档案附件按版本保存、不覆盖旧版本、不提供覆盖与原地删除接口 |
| 7.7 法人行级隔离 | 30 张表按统一模板建策略；跨法人查询逐法人设置会话变量后合并；不使用 BYPASSRLS |
| 7.9 派生存储安全继承 | 档案事件携带 security_level 与 data_scope_tags；搜索索引正文不含开票要素与银行信息；删除与更正在 15 分钟内传播 |
| 7.10 历史数据导入 | 明确日常批量导入与迁移通道互不混用，本阶段只交付前者；按裁定 A-24 不设独立数据迁移阶段，期初与历史数据导入通道分别归阶段 9a、阶段 10 与阶段 8 |
| 8 黄金业务闭环第 1 步 | 建单时自动带出客户、产品、价目与历史成交资料四项中的全部四项的服务端取数 |
| 12.2 授权 | 审批链不可越权跳过与申请人不可自审在变更申请上执行；字段级权限与密级在开票要素与银行信息上执行 |
| 12.5 审计 | 档案的新建、变更、停用、启用、作废、审批、导入批次、导出任务、敏感导出的重新认证一律与业务变更同事务写审计 |
| 15.1 错误分类 | 本阶段全部错误落在五类分类内，封套四要素齐备，权限拒绝与不存在不可区分 |
| 16 与附录 A.1 | 承担四个度量项与一个观察子段 |
| 17.2 自动化测试 | 单元、领域属性、集成与契约、四端端到端五类测试在本阶段范围内执行 |
| 17.3 强制不变量 | 本阶段不直接承载账务不变量，但通过物料三项冻结字段保证库存数量守恒与存货金额账一致的分组键稳定，并通过法人隔离测试集保证权限不越权一项 |

#### 10.2 PRD 条目

| PRD 节 | 本阶段实现的功能 |
|---|---|
| 2.1.1 治理范围 | 四类档案的边界，会计科目表与设备台账的排除 |
| 2.1.2 法人隔离 | 所属法人由服务端从安全上下文写入，不采信客户端声明；两个法人下各自建档、互不可见 |
| 2.1.3 角色 | 申请人、责任人、审批人、系统管理员四类角色的权限动作定义 |
| 2.2.1 档案状态机 | 五个状态映射到 status 与 is_active 两列，五条流转规则，已生效不提供作废路径 |
| 2.2.2 变更审批与版本 | 变更申请单承载新版本、档案继续按当前生效版本对外可用、单据引用不中断、同一档案单例约束、逐字段前后对照、冻结字段拒绝、变更不回溯 |
| 2.2.3 编码与编号规则 | 四类档案各一套独立序列、法人内唯一、生效后冻结、不可回收复用 |
| 2.2.4 唯一性与重复校验 | 四项校验全部实现，名称重复的确认动作写入审批意见 |
| 2.2.5 停用与启用 | 停用只改可引用性、不校验未完成单据、展示引用计数、启用走同一审批 |
| 2.2.6 附件、检索与列表 | 附件按版本保存、检索按法人与字段级权限裁剪、列表默认值按基线第 11.5 节 |
| 2.2.7 并发编辑 | 乐观并发、版本冲突按业务冲突拒绝、展示冲突对象与当前版本、不静默覆盖 |
| 2.3 客户档案 | 全部 13 类字段与其必填、可变更、校验规则 |
| 2.4 供应商档案 | 全部 12 类字段、资质到期扫描与标注、四类供应商侧记录、门户提交生成待审批申请 |
| 2.5 物料档案 | 全部 11 类字段与三项条件冻结规则 |
| 2.6 产品档案 | 全部 11 类字段与销售计量单位的条件冻结规则 |
| 2.7 物料与产品的关系 | 编码空间独立、口径分工、只有物料无产品与只有产品无物料两类允许、建单带出物料 |
| 2.8 价目表 | 全部字段、六状态机、取价命中三分支 |
| 2.9 历史成交资料 | 销售侧与采购侧的取数键、展示内容、只读、权限裁剪、显式选用才回填 |
| 2.10 引用关系与停用影响 | 五类档案停用后对新建与存量的行为，统一规则不触发单据状态变化 |
| 2.11.1 与 2.11.3 | 日常批量导入与导出全量实现 |
| 2.11.2 | 明确不在本阶段，走规格第 7.10 章通道；通道归属按裁定 A-24 分列于阶段 9a、阶段 10 与阶段 8 |
| 2.12 异常与失败提示 | 表中 14 个场景的系统处理与提示要点逐条落到错误码与 details |
| 2.13 首版不含 | 七条不含项在本阶段不实现，不留半成品入口 |

### 11 风险与预留

#### 11.1 技术风险

R1 供应商档案的模块归属分歧。基线第 1.2 节把供应商档案与资质列在 procure 行，PRD 第 2.4 节把档案本体、资质、价格、交期、质量、风险全部写在主数据节，这是 PRD 附录乙 U-C-08 登记的双归属问题。本阶段收口到 mdm。若整合期改判归 procure，代价为把 8 张表跨 schema 迁移并把 ep-contract-mdm 的三个供应商侧 trait 反向，属一次停机窗口内的表迁移，估算 1 至 2 人日加一次数据搬迁。缓解方式是把供应商侧的对外接口全部收敛到 ep-contract-mdm 的四个 trait 上，采购阶段不直接读 mdm 的表。

R2 探针缺位期的冻结字段放行。库存与销售模块交付前，物料三项与产品一项冻结字段实际不受保护。缓解方式是命名自检项 master-data-usage-probes-registered 强制，另在验收演练前用一次全量对账脚本核对：对每个物料，若库存流水存在则其 base_uom_id 必须与首次流水记录的单位一致。该脚本在阶段 8 交付 InventoryMaterialUsageProbe 时一并加入内部对账组件；物料三项的完整性验收顺延到阶段 8，产品一项顺延到阶段 6。

R3 取价多行命中无优先级。PRD 第 2.8.3 节明确该规则未定义。本阶段返回全部命中行并强制人工选择。若决策改为系统自动取一行，CPQ 的返回值语义从命中集合变为单一默认值，销售阶段的取数代码需同步改，估算半人日，且 price_lists 需新增一个优先级列，属在线变更范围内的新增可空列。本阶段按纪律不预建该列。

R4 名称前缀匹配的索引与排序规则。数据库排序规则非 C 时普通 B-tree 索引不支持 like 前缀匹配走索引，因此本阶段在四张档案表上另建带 text_pattern_ops 操作符类的复合索引。该索引不属函数索引与部分索引，不违反基线第 3.10 节，但需在认证期确认 PostgreSQL 16 上的实际行为，并在 EXPLAIN 证据中体现。

R5 变更申请快照的体积。proposed 与 diff 两列为 jsonb，包含档案头与全部子表。供应商含 20 张资质证照时单条快照可达数十 KB。缓解方式是快照中附件只存对象 ID 不存元数据，且 change_requests 只保留在途与近期已决申请的完整快照，历史版本由 record_versions 承载。若实测单条超过 256 KB 则改为只存变更涉及的字段子集，代价是审批界面需要额外一次取数。

R6 导出任务表可能被平台收编。本阶段的 mdm.export_jobs 是自有表。若后续阶段引入平台级统一异步任务台账，本表按迁移收编，代价为一次数据搬迁与端点路径变更。本阶段把该表的列设计为对象类型无关，降低收编成本。

R7 搜索索引重建时长。两个法人合计四万条档案，索引重建按规格第 7.9 章可按法人整体重建，重建期间该分区停止对外服务。本阶段需在集成测试中实测一次全量重建耗时并记录，若超过 15 分钟则需与运维中心的暴露窗口口径对齐。

R8 枚举字典化与固定枚举的切换。四类分类取值由 mdm.classification_items 承载。若 U-A-07 决策为固定枚举且不允许管理员增删，则需把四列改为 CHECK 约束并下线该表，属改列约束的收紧操作，需要停机窗口。估算半人日加一次窗口。

#### 11.2 为后续阶段预留的扩展点

预留一律以接口与索引形态存在，不预建未登记的列，遵守基线第 12 节的落地纪律。

- 跨模块引用的五个消费方端口 MaterialUsageProbe、ProductUsageProbe、MasterReferenceCounter、SalesTradeHistoryProvider 与 PurchaseTradeHistoryProvider 已定义，两个注册表 MasterReferenceCounterRegistry 与 TradeHistoryProviderRegistry 已在 apps/core-server/src/wiring.rs 与 apps/job-worker/src/wiring.rs 中留出注册位，实现方与实现类型名按裁定 A-13、A-14 与 A-15 固定，后续阶段只需实现并注入。
- 客户 360 的 Customer360SectionProvider 已定义并留出注册位，合同、订单、回款、投诉、设备与服务记录六类区块由各自阶段在同一端点上注册。
- 供应商自维护命令端口 SupplierSelfServiceCommand 已定义，门户阶段只需实现门户侧端点并调用。
- 产品与物料关联基数放开为多对多，只需删除一条唯一索引，属在线变更。
- 价目表适用范围扩展新的取值，只需扩展 CHECK 约束与 ScopeKind 枚举，客户端按基线第 5.6 节容忍未知取值并降级展示。
- 档案编码规则的年月段若后续要求加入，属编号规则配置的取值变更，不改表结构。
- 本阶段的全部事件版本号为 v1，破坏性变更时按基线第 6.1 节新增 v2 并并行一段时间。

### 12 对共享技术基线的偏离与新增决定

#### 12.1 偏离项

| 编号 | 偏离内容 | 理由 | 影响范围 | 基线修订建议 |
|---|---|---|---|---|
| D1 | 供应商档案与资质由 mdm 承载，而非基线第 1.2 节模块表 procure 行所写 | PRD 第 2.4 节把档案本体与五类记录全部定义在主数据节，且本阶段的范围由阶段划分明确包含供应商档案；分置两处会产生两份供应商档案 | procure 阶段改为只引用不建表 | 基线第 1.2 节 procure 行改为采购需求、采购订单与分批订货、收货、采购退货、付款申请，删去供应商档案与资质，并在 mdm 行补入供应商资质与价格交期质量风险记录 |
| D2 | 附件关联表在基线第 4 节固定的四列之外追加 is_active 与 deactivated_at | 基线第 3.6 节禁止在业务 schema 上执行 DELETE，而解除附件关联是必需操作 | 全部模块的附件关联表 | 基线第 4 节附件引用一条补入这两列 |
| D3 | mdm.record_versions 与 mdm.import_batch_rows 按仅追加表处理，两表都不带 reverses_id | 基线第 4 节仅追加表一条的枚举不含这两张表，而版本快照与导入行结果都没有冲销语义，加一个恒为 NULL 的列只会制造误解 | 只影响本阶段这两张表 | 基线第 4 节仅追加表一条的表枚举中补入 mdm.record_versions 与 mdm.import_batch_rows，并写明这两张表无冲销语义、不带 reverses_id |

#### 12.2 本阶段新增决定

下列事项基线未覆盖，本阶段取值并在阶段结束时回写基线。

| 编号 | 事项 | 取值 |
|---|---|---|
| N1 | 档案编码格式 | <类型码>-<法人码>-<6 位流水>，不含年月段，回写基线第 11.1 节 |
| N2 | 本阶段的八个类型码 | CUST、SUPP、MATL、PROD、PRLS、MDCR、MDIB、MDEX，按裁定 C-26 登记在 docs/data-dictionary.md 的单据类型码一节，由 xtask configdoc --check-doc-type-codes 校验全局唯一，并回写基线第 11.1 节的类型码登记指引 |
| N3 | 档案五状态到 status 与 is_active 两列的映射 | 见第 3.2 节，回写基线第 4 节档案类表补充规则 |
| N4 | 空槽唯一索引的写法 | 可空槽位列加普通唯一索引加 CASE 表达式的 CHECK，作为首版不使用部分索引前提下表达条件唯一的统一写法，回写基线第 3.10 节 |
| N5 | 带操作符类的复合索引 | text_pattern_ops 类索引不属基线第 3.10 节禁止的函数索引与部分索引，允许用于前缀匹配，回写基线第 3.10 节 |
| N6 | 模块级启动自检 | 允许模块在基线第 7.3 节的十三个命名项之后追加自身的命名自检项，失败同样以退出码 78 退出；本阶段的追加项名为 master-data-usage-probes-registered，按裁定 C-25 以注册名标识而不用序号，回写基线第 7.3 节 |
| N7 | 主数据事件不进入关账枚举 | 不带 posting_date 与 accounting_period_id 的领域事件不计入关账受理前提中待消费过账条目数的枚举范围，回写基线第 6.1 节 |

### 13 被阻塞的业务决策与临时取值

本阶段引用的待决事项、是否阻塞、临时取值与切换代价。全部临时取值都在配置项或字典表中承载，切换不需要改领域逻辑。

| 编号 | 事项 | 是否阻塞本阶段 | 临时取值 | 切换代价 |
|---|---|---|---|---|
| U-A-01、U-A-02 | 编号规则与其承载节 | 不阻塞 | 按第 12.2 节 N1 与 N2 | 改编号规则配置数据 |
| U-A-03 | 文本长度上限 | 不阻塞 | 按基线第 11.2 节 | 放宽长度属在线变更，收紧需停机窗口 |
| U-A-05 | 列表默认值 | 不阻塞 | 按基线第 11.5 节，档案默认排序 code asc | 改配置 |
| U-A-06 | 错误文案粒度 | 不阻塞 | 每个错误码一条中文文案，集中在 docs/error-codes.md | 改文案表 |
| U-A-07 | 受控取值枚举清单与管理员增删 | 不阻塞 | 由 mdm.classification_items 表承载七类取值，出厂预置最小集合，允许管理员增删；税率一类按裁定 C-11 不在本表 | 若决策为固定枚举，需改 CHECK 约束并下线该表，需停机窗口 |
| U-A-08 | 默认审批链 | 阻塞演示不阻塞开发 | 出厂预置一条单节点审批链，审批人取该对象类型的主数据审批人角色 | 改审批链配置 |
| U-A-09 | 导入模板列、最大行数、失败语义 | 不阻塞 | 最大 5000 行，通过行落库，错误行不落库，错误清单逐行标注 | 改配置项与校验分支 |
| U-A-11 | 提醒提前量 | 不阻塞 | 资质到期提前 30 天 | 改配置项，决策后改为低代码定时器配置 |
| U-A-12 | 开户银行是否同列敏感清单、三场景脱敏形态、导出是否触发重新认证 | 不阻塞 | 待决范围只有三问，裁定表与本阶段均不代拍：开户银行是否同列敏感字段清单、列表与详情与导出三场景的脱敏形态、导出是否触发重新认证。银行账号纳入行内敏感字段并做字段级加密由规格第 7.8 章强制，不在待决范围内，第 25 号迁移中 bank_account_no 两行的 is_field_encrypted 取真，物理列为 bank_account_no_enc 与 bank_account_no_key_ref 与 bank_account_no_tail 三列且不保留同名明文列。三问的临时取值依次为：开户银行纳入并登记两行，其 is_field_encrypted 取假、物理列保持 bank_name text 明文；bank_account_no 两行的 mask_style 取 KEEP_LAST_4 且后四位取自 bank_account_no_tail，bank_name 两行取 NONE，三场景同形态并一律经阶段 4 的 FieldProjector 渲染；导出是否触发重新认证不由本表列承载，统一指向阶段 4 的重新认证判定函数，该函数对这四列判真。四行的 security_level 与 mask_style 同为未决期间的临时取值 | 第一问改判为不纳入时删除或改写 bank_name 两行，属数据行变更，不改代码也不改表；改判为开户银行也做字段级加密时，须在一次变更内同时把 bank_name 两行的 is_field_encrypted 改为真、把物理列改为 bank_name_enc bytea 并补 bank_name_key_ref text、删去同名明文列，缺一 db/checks/11 必然判负。第二问改判只改这四行的 mask_style，不改代码。第三问改判限于阶段 4 判定函数的入参配置，本阶段不在表列上另给第二套答案。决策人为安全负责人与产品负责人，截止点按总览 R12 的 M3 之前关闭 U-A 组 |
| U-B-05 | 权限求值顺序 | 不阻塞 | 按基线第 11.3 节，显式拒绝优先 | 无 |
| U-B-06 | 字段权限是否有脱敏中间态 | 不阻塞 | 假定存在可见但脱敏中间态，掩码保留后 4 位 | 改字段元数据 |
| U-B-07 | 记录级权限授予方式 | 不阻塞 | 假定按责任人授予，因此四类档案均建 owner_user_id 索引 | 若改为按创建人或显式共享，索引仍可用，只改判定策略 |
| U-C-04 | 客户 360 视图无定义节 | 部分阻塞 | 本阶段只交付 Customer360SectionProvider 区块注册契约与 /customer-360 端点骨架，区块内容由各模块阶段在同一端点上注册 | 区块内容为增量注册，不改契约 |
| U-C-05 | 计量单位与地点的承载节 | 阻塞物料与产品建档 | 计量单位由本阶段以最小档案交付，不走审批链；地点不交付 | 若改归其他模块，需迁移 mdm.uoms 一张表并改两条外键为跨模块逻辑引用 |
| U-C-06 | 仓库档案归属 | 不阻塞 | 归 inventory，本阶段不建表 | 无 |
| U-C-08 | 供应商档案双归属 | 不阻塞 | 归 mdm，见第 12.1 节 D1 | 见 R1 |
| U-D-04 | 税率可选值集合 | 不阻塞 | 列上只校验大于等于 0 小于 1；可选值集合按裁定 C-11 归阶段 10 的 invoice.tax_rate_options，阶段 10 交付前由本阶段的字典桩 MdmTaxRateStub 承担临时取值 | 阶段 10 交付时执行税率迁移并删除 MdmTaxRateStub，取用入口改为 ep_contract_invoice::TaxRateOptionQuery |
| U-E-01 | 信用额度按客户还是按客户加法人 | 不阻塞 | 落在 mdm.customers 上，因主数据按法人隔离且首版不含跨法人分发，等价于按客户加法人 | 若改为按客户全局，与首版不含跨法人主数据分发直接冲突，需先修订规格 |
| U-E-02 | 信用额度为空的默认行为 | 不阻塞 | 列可空，NULL 表示未维护，0 表示零额度，两者语义不同；判定归销售阶段 | 无 |
| U-F-08 | 供应商状态集合与语义 | 不阻塞 | 沿用四类档案的统一状态机，不另设暂停与终止 | 若新增状态，需扩 CHECK 约束 |
| U-F-09 | 质量风险记录字段与是否自动生成 | 不阻塞 | 只支持手工登记，字段按 PRD 第 2.4.3 节四项 | 自动生成时由 procure 经 ep-contract-mdm 的命令写入，不改表结构 |
| U-F-14 | 资质过期是否阻断新建采购订单 | 不阻塞 | 本阶段只暴露 qualification_status，不做阻断 | 判定归采购阶段 |
| U-G-05 | 空批次标识 | 不阻塞 | 按基线第 11.4 节取固定值单个连字符，本阶段只在物料的批次管理标记上体现 | 无 |
| PRD 2.3.1 无统一社会信用代码的客户类型 | 客户建档 | 不阻塞 | 客户类型落入 INDIVIDUAL 与 OVERSEAS 时免校验并要求 alternate_identifier 非空 | 改配置项 EP__MDM__USCC__EXEMPT_CUSTOMER_TYPES |
| PRD 2.2.1 | 已生效档案的删除口径 | 不阻塞 | 不提供删除路径，只提供停用 | 若决策提供删除，需新增状态与一套引用检查 |
| PRD 2.2.5 | 是否在有未结清应收或非零结存时阻断停用 | 不阻塞 | 不阻断，只展示引用计数 | 若改为阻断，由 MasterReferenceCounter 的返回值加一条守卫条件，半人日 |
| PRD 2.8.3 | 多行命中优先级 | 不阻塞 | 返回全部命中行并强制人工选择 | 见 R3 |
| PRD 2.9.3 | 历史成交的条数、窗口、排序与是否含作废单据 | 不阻塞 | 20 条、12 个月、按单据日期倒序、不含作废红冲与退货 | 改三个配置项 |
