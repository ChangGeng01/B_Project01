## 阶段 13：四端客户端与低代码

本阶段承接规格第 6 章客户端与用户体验、第 7.4 章可定制数据库、第 9 章低代码与规则与模块发布、第 3.1 章与第 3.6 章白标与分发、第 12.4 章按端的数据保护控制，以及 PRD 第 8.4 节、第 10.4 节、第 10.7 节。本阶段不产生任何会计分录，也不新增任何账务口径，涉及账务的一律指向规格第 5.2 章事件-分录表，由财务阶段承担。
本阶段的交付边界按裁定 A-23 收窄：本阶段不交付任何业务界面，交付物只有客户端壳、路由注册表、能力矩阵闸、白标构建与四端制品；各业务模块的四端界面位于 `clients/desktop/src/modules/<module>/` 与 `clients/mobile/src/modules/<module>/`，由阶段 5 至阶段 12 各自交付，四端验收矩阵由阶段 14 汇总。裁定表中称为阶段 13b 的条目即本阶段。本阶段在调整后的阶段顺序 1 → 2 → 3a → 4 → 3b → 5 → 9a → 8 → 6 → 7 → 10 → 11 → 9b → 13 → 14 中排在阶段 11 之后、阶段 14 之前，阶段 12 在阶段 10 之后与阶段 11 并行。

本计划遵守共享技术基线。凡基线已定死的取值直接引用，不重新决定。本阶段新增的决定在第 12 节集中列出，需在阶段结束时回写基线。

---

### 1. 交付物清单

本阶段结束时，下列各项存在且可运行。

#### 1.1 服务端可运行物

1. core-server 内新增的低代码建模 API、扩展登记 API、客户端引导 API 与能力闸中间件，以及在阶段 3b 最小发布通道之上扩展的配置发布 API，全部经 `/api/v1/platform/...` 暴露，可用 `curl` 完成第 5 节全部端点的往返。
2. job-worker 内在阶段 3b 发布执行器之上扩展的 DDL 段编排、在线 DDL 执行器、派生存储重新打标任务、扩展自动停用巡检，可由 Outbox 事件驱动跑通一次含 DDL 的发布与一次回退。
3. plugin-host 进程从空壳变为可用宿主：可加载签名 WASM Component、按能力清单裁剪输入、按资源限额中止执行、把结果经 `/run/ep/ipc/plugin.sock` 返回给 core-server 与 job-worker，且该进程的数据库连接数恒为 0。
4. `platform_meta` 下 19 张新表与其迁移文件、回退说明、种子数据（能力等价矩阵 18 行乘 4 端共 72 行），以及对阶段 3b 已建的 `config_packages`、`config_package_items`、`config_release_orders` 三张表的列扩展与状态扩展。
5. `docs/error-codes.md` 新增 37 条错误码、`docs/event-catalog.md` 新增 10 个事件类型、`docs/data-dictionary.md` 新增 19 张表条目，三处与代码常量表由 CI 校验一致。本阶段引用但由阶段 1 登记的 `PLATFORM.CONCURRENCY.STALE_VERSION`、`PLATFORM.AUTHZ.NOT_FOUND_OR_DENIED` 与 `PLATFORM.DB.MIGRATION_WINDOW_CLOSED` 三条不计入本阶段条数，见裁定 C-24。

#### 1.2 客户端可运行物

6. Windows 客户端安装包（msi 与便携 exe 各一）、macOS 客户端安装包（dmg，已签名并公证）、iOS 客户端（ipa，企业签名通道与商店通道各一份配置）、Android 客户端（aab 与 apk）。四份制品均由同一份 `brand.toml` 驱动产出，均可完成登录、设备登记、按能力矩阵渲染入口、离线草稿保存与恢复后提交。
7. 桌面端两个签名原生插件：打印机插件与 USB Key/智能卡插件，各自以独立子进程运行，含能力清单与本地登记表。
8. 客户端 Rust 核心 crate 集合（第 2 节列出），含本地加密缓存库、增量同步、声明式规则解释器、原生插件宿主、统一网络出口与证书链校验。
9. 四端共用的 React/TypeScript 界面壳与共用组件包，含浅色、深色、高对比度三套主题，全键盘与命令面板，WCAG AA 自动检查零严重问题，以及按能力矩阵裁剪入口的路由注册表。按裁定 A-23，本包不含任何业务模块界面，业务模块界面由阶段 5 至阶段 12 交付到 `clients/desktop/src/modules/<module>/` 与 `clients/mobile/src/modules/<module>/`。

#### 1.3 工具与流水线

10. `tools/epcfg`：配置包的打包、差异、签名、验签与离线导出导入命令行工具，可在开发与测试空间使用，也可在生产侧做只读校验。
11. `tools/epbrand`：白标构建流水线入口，输入 `brand.toml` 与源码 commit，输出四端未签名制品与其哈希清单，再调用签名步骤产出签名制品；同一输入两次构建的未签名制品哈希一致。
12. `tools/epplug`：WASM 插件 SDK 与打包签名工具，含一个示例插件（税额校验计算）与其能力清单。
13. 商店政策合规检查门禁脚本，四项检查项按规格第 3.6 章逐项判定，未通过即中断流水线。

#### 1.4 证据物

14. 附录 C.2 十二项门槛在 C.1 设备基线上的复测报告与全部原始测量数据。
15. 规格第 6.2 章能力等价矩阵 18 行、豁免清单每条替代路径的逐条核对表，含四端 E2E 执行证据。
16. 一次完整的配置发布与回退演练证据包：含差异审查记录、自动测试报告、审批与签名记录、执行耗时、锁持有时长、回退结果与审计链验证结论。

---

### 2. crate 与进程归属

#### 2.1 服务端新增或改动的 crate

| crate | 归属进程 | 本阶段职责 | 依赖方向核对 |
|---|---|---|---|
| ep-platform-meta | core-server、job-worker | 自定义对象与字段与关系与索引与视图的建模、在线 DDL 计划与影响分析、界面布局、能力等价矩阵判定、声明式规则 AST 与解释器实现 `AstRuleEvaluator`、自定义对象向权限与流程与搜索与报表的注册端口、六个 CUSTOM_ 与 UI_LAYOUT 类 `ConfigItemApplier` 实现 | 只依赖 ep-foundation 与其他 ep-platform-*，无 sqlx、无 reqwest |
| ep-platform-release | core-server、job-worker | 本 crate 由阶段 3b 按裁定 A-27 交付最小发布通道，`ConfigItemApplier` 端口与 `ConfigItemApplierRegistry` 由阶段 3a 按裁定 A-19 交付；本阶段在其上扩展内容项差异算法、自动测试编排、十一态发布状态机、DDL 段编排与回退编排 | 依赖 ep-foundation、ep-platform-meta、ep-platform-audit、ep-platform-outbox |
| ep-adapter-wasm | plugin-host、core-server、job-worker | wasmtime Component 宿主、能力清单裁剪、燃料与内存与时限限额、编译缓存、宿主导入函数四件套，实现类型 `PluginHostWasmCompute` 对应阶段 3b 定义的 `ep_platform_flow::port::WasmComputePort`，见裁定 B-05；core-server 与 job-worker 侧只编入其 IPC 客户端 | adapter 层，可依赖 foundation 与 platform/domain 的端口 trait，不依赖 application |
| ep-adapter-ipc | plugin-host、core-server、job-worker | 复用基线第 2 节已定的帧格式，新增 plugin 通道的请求与响应类型 | 同上 |
| ep-foundation | 全部 | 本阶段不新增也不改动 foundation 类型：`Tx`、`SnapshotCtx`、`UnitOfWork` 由阶段 1 按裁定 A-01 在 `port::tx` 中冻结，`SecurityContext` 与 `ClientKind` 按裁定 A-03 冻结，`ModuleCode`、`CapabilityDomain`、`ActionClass` 按裁定 A-20 冻结，`Redacted<T>` 同由阶段 1 提供，本阶段只引用 | 不依赖工作区内任何 crate |
| ep-platform-obs | ops-agent | 注册本阶段 9 个新指标 | 只登记，不改结构 |

本阶段不新增 platform crate（`ep-platform-release` 与 `ep-platform-license` 均由阶段 3b 交付，本阶段只在前者之上扩展），不新增业务模块 crate，不新增进程，不新增 schema，不新增错误分类。

#### 2.2 客户端 crate（不属八进程，位于 workspace 之外的独立 Cargo workspace）

客户端代码位于仓库 `/clients/`，为独立 Cargo workspace，与服务端 workspace 通过路径依赖复用 `ep-foundation`、`ep-contract-*` 与 `ep-platform-meta`。客户端不得依赖任何 `ep-app-*`、`ep-adapter-db*` 与 `ep-platform-outbox`。

| crate | 平台 | 职责 |
|---|---|---|
| ep-client-core | 四端 | 客户端核心装配：会话、安全上下文、统一网络出口、TLS 与证书链校验、错误封套解析、能力矩阵本地判定、引导数据缓存 |
| ep-client-cache | 四端 | 本地加密缓存库：SQLCipher 打开与密钥解封、表结构、增量同步、冲突处理、超期清除、标签随行 |
| ep-client-keystore | 四端 | 设备硬件保护密钥的封装与解封：Windows DPAPI 加 TPM、macOS Keychain 加 Secure Enclave、iOS Secure Enclave、Android Keystore |
| ep-client-rules | 四端 | 声明式规则解释器的客户端外壳，直接复用 ep-platform-meta 的 `rule` 模块，只对 `executable_on_client` 为真的规则求值 |
| ep-client-draft | 四端 | 离线草稿：本地保存、待中心校验标记、恢复连接后按业务模块正常提交端点重提交 |
| ep-client-sync | 四端 | 按记录版本与更新时间的增量拉取、法人切换、缓存失效 |
| ep-client-plughost | Windows、macOS | 桌面端原生插件宿主：验签、能力清单核对、子进程拉起、受限 IPC、崩溃与超时隔离、本地登记表 |
| ep-client-device | 四端 | 设备登记、远程注销响应、安全清除、生物识别调用、相机扫码（移动端）、MDM 合规状态读取 |
| ep-client-audit | 四端 | 客户端审计事件的本地暂存与向中心提交；客户端不建本地分段链，照抄规格第 12.5 章 |

外壳与界面：

- `/clients/desktop`：Tauri 2 桌面壳，Windows 与 macOS 共用，暴露 Tauri IPC 命令给 WebView；其 `src/modules/<module>/` 为各业务模块的桌面界面目录，按裁定 A-23 由阶段 5 至阶段 12 各自填入，本阶段只交付壳、路由注册表与目录约定。
- `/clients/mobile`：Tauri 2 移动壳，iOS 与 Android 共用，含各自的生命周期与后台任务适配；其 `src/modules/<module>/` 为各业务模块的移动界面目录，归属同上。
- `/clients/ui`：React 加 TypeScript 共用组件包，四端共用同一套组件、权限模型与流程模型，仅按端切换布局密度与入口可见性；本包不含任何业务模块界面。
- `/clients/plugins/printer`、`/clients/plugins/usbkey`：两个桌面端原生插件的独立可执行工程。

#### 2.3 进程侧连接与账号的变化

- core-server 与 job-worker 继续使用 `ep_app_rw`，连接池上限不变。
- 在线 DDL 执行需要 `ep_migrator`。落地方式为：job-worker 仅在存在一张已签名已批准且含 DDL 段的发布单时，临时建立一条 `ep_migrator` 连接，执行完毕立即关闭。该连接计入基线第 2 节“迁移与应急临时连接另计不超过 10”的额度，同时按规格第 7.7 章把该账号的启用与回收各写一条审计事件。任何其他路径不得建立 `ep_migrator` 连接。
- plugin-host 的数据库连接数为 0，本阶段以启动自检与集成测试双重断言。

---

### 3. 数据库变更

#### 3.1 总则

本阶段全部新表落在 `platform_meta` 一个 schema 内。理由是本阶段不得定义其他阶段拥有的 schema 的表，而品牌配置、客户端版本、扩展登记三组虽不属狭义元数据，但都是部署级配置对象，与自定义对象元数据同属配置发布链路的内容项，集中在一个 schema 内使模块隔离边界与迁移边界一致。

表分两类。

- 部署级配置表：按基线第 3.8 节的第四类“全局配置字典”处理，不带 `legal_entity_id`，不带 `data_scope_tags`，不建行级策略。其可见性由对象级权限判定，不承载任何业务数据。这一归类是本阶段的显式判断，理由是低代码配置在本部署内跨两个法人共用，为其编造一个法人列会制造第二套隔离口径，与基线第 4 节反对 `tenant_id` 的理由同构。
- 法人级运行台账表：带 `legal_entity_id`，按基线第 3.8 节模板建行级策略，列全。

全部表带基线第 4 节公共列，顺序按基线；仅追加表按基线不带 `row_version`、`updated_at`、`updated_by`，改带 `reverses_id`。枚举一律 `text` 加 CHECK。时间列 `timestamptz`，日期列 `date`。主键 `uuid`，应用侧 UUIDv7。
按裁定 A-27，`platform_meta.config_packages`、`platform_meta.config_package_items` 与 `platform_meta.config_release_orders` 三张表由阶段 3b 随最小发布通道建立，第 3.2.10 至 3.2.12 节所列列定义即阶段 3b 的落地口径，本阶段只做列扩展与状态扩展，不重复建表；`config_release_steps`（裁定 A-27 中称 `config_item_apply_logs`）、`config_autotest_runs`、`config_edit_locks` 与 `config_release_mutex` 四张由本阶段建立，本阶段因此新建 19 张表。阶段 3b 的发布状态机为 Draft、PendingReview、PendingApproval、Approved、Released、RolledBack 六态，本阶段扩展为第 3.2.10 节的十一态：追加裁定 A-27 明列的 PendingAutotest 与 TestPassed 两态，另追加 TestFailed、Rejected、SignedPendingRelease、Superseded 四态；阶段 3b 的 PendingReview 在本阶段由差异审查环节承载，不单列为状态，迁移把遗留的 PENDING_REVIEW 行改写为 DRAFT 后重建 CHECK。全部种子迁移与系统上下文写入的 `created_by` 一律取 `foundation::SYSTEM_PRINCIPAL_ID`，不得自选取值，见裁定 A-02。

#### 3.2 表定义

##### 3.2.1 platform_meta.custom_objects（部署级）

| 列 | 类型 | 可空 | 默认 | 说明 |
|---|---|---|---|---|
| id | uuid | 否 | 无 | 主键 |
| security_level | smallint | 否 | 20 | 对象级密级，取值 10、20、30、40 |
| code | text | 否 | 无 | 对象码，同时是 `ext` 下的物理表名 |
| name | text | 否 | 无 | 显示名 |
| physical_table_name | text | 否 | 无 | 固定为 `ext.` 加 code |
| is_document | boolean | 否 | false | 单据类为真，档案类为假 |
| doc_type_code | text | 是 | 无 | 单据类必填，2 至 4 位大写字母 |
| definition_version | bigint | 否 | 1 | 每次生效的定义版本，客户端引导按此判定缓存失效 |
| status | text | 否 | 'DRAFT' | DRAFT、PENDING_DDL、ACTIVE、DDL_FAILED、RETIRED |
| retired_at | timestamptz | 是 | 无 | 停用时间 |
| row_version | bigint | 否 | 1 | 乐观锁 |
| created_at / created_by / updated_at / updated_by | 按基线 | 否 | 按基线 | 公共列 |

约束与索引：

- `pk_custom_objects`。
- `ux_custom_objects_code`。
- `ix_custom_objects_status_created_at`。
- `ck_custom_objects_code_shape`：`code ~ '^[a-z][a-z0-9_]{2,62}s$'`，即小写 snake_case 且以 s 结尾，落实基线第 3.2 节表名复数。
- `ck_custom_objects_status`：取值集合。
- `ck_custom_objects_security_level`：取值 in (10,20,30,40)。
- `ck_custom_objects_doc_type`：`is_document = false and doc_type_code is null` 或 `is_document = true and doc_type_code ~ '^[A-Z]{2,4}$'`。
- 单据类型码的全局唯一性按裁定 C-26 由应用层校验：`doc_type_code` 必须与 `docs/data-dictionary.md` 单据类型码一节的全量表以及 `ep-platform-sequence` 的常量表逐项比对，重复即拒绝并返回 `PLATFORM.CUSTOM_OBJECT.DOC_TYPE_CODE_CONFLICT`，HTTP 409，category 为 BUSINESS_CONFLICT；`xtask configdoc --check-doc-type-codes` 同时覆盖内置码与已生效的自定义对象码两类。
- `ck_custom_objects_name_len`：`char_length(name) <= 200`，按基线第 11.2 节。
- 无行级策略。

##### 3.2.2 platform_meta.custom_fields（部署级）

| 列 | 类型 | 可空 | 说明 |
|---|---|---|---|
| id | uuid | 否 | 主键 |
| security_level | smallint | 否 | 表自身的密级，固定 20 |
| custom_object_id | uuid | 是 | 自定义对象上的字段时非空，同 schema 外键 ON DELETE RESTRICT |
| core_object_type | text | 是 | 核心对象上的自定义字段时非空，取值形如 `sales.sales_orders` |
| code | text | 否 | 字段码，同时是物理列名 |
| name | text | 否 | 显示名 |
| data_type | text | 否 | INTEGER、DECIMAL、FLOAT、BOOLEAN、STRING、TEXT、DATE、TIMESTAMP、ENUM、REFERENCE、JSON |
| is_nullable | boolean | 否 | 首版新增列一律为真，收紧非空进停机窗口 |
| max_length | int | 是 | STRING 与 TEXT 必填 |
| decimal_precision | smallint | 是 | DECIMAL 必填 |
| decimal_scale | smallint | 是 | DECIMAL 必填 |
| enum_values | text[] | 是 | ENUM 必填，取值大写 snake_case |
| reference_object_type | text | 是 | REFERENCE 必填 |
| default_expr | text | 是 | 只接受常量字面量，不接受易失函数 |
| field_security_level | smallint | 是 | 空表示按所属对象取值 |
| physical_column_name | text | 否 | 与 code 同值 |
| definition_version | bigint | 否 | 定义版本 |
| status | text | 否 | DRAFT、PENDING_DDL、ACTIVE、DDL_FAILED、RETIRED |
| row_version、created_at、created_by、updated_at、updated_by | 按基线 | 否 | 公共列 |

约束与索引：

- `pk_custom_fields`、`fk_custom_fields_custom_objects`。
- `ux_custom_fields_object_code`：唯一约束在 `(coalesce(custom_object_id::text, core_object_type), code)` 上无法写为普通唯一索引，因此改为新增一个生成列 `owner_key text not null`，由应用写入 `custom_object_id::text` 或 `core_object_type`，在 `(owner_key, code)` 上建 `ux_custom_fields_owner_key_code`。这样不使用函数索引，符合基线第 3.10 节。
- `ix_custom_fields_owner_key_created_at`。
- `ck_custom_fields_owner_exactly_one`：`custom_object_id is not null` 与 `core_object_type is not null` 恰有一个成立。
- `ck_custom_fields_type_params`：按 data_type 逐类校验参数组合非空或为空。
- `ck_custom_fields_decimal_bounds`：`decimal_precision <= 18 and decimal_scale <= 6`，与基线第 3.5 节的金额、单价、数量精度上界一致。
- `ck_custom_fields_code_shape`：`code ~ '^[a-z][a-z0-9_]{1,62}$'`，且不得命中保留列名集合（公共列九项加 `doc_no`、`status`、`code`、`is_active`、`deactivated_at`、`posting_date`、`business_date`、`accounting_period_id`、`reverses_id`），保留列名由应用层校验并返回 `PLATFORM.CUSTOM_OBJECT.RESERVED_NAME`。
- 无行级策略。

##### 3.2.3 platform_meta.custom_relations（部署级）

列：id、security_level、code、from_object_type text、to_object_type text、cardinality text（ONE_TO_ONE、ONE_TO_MANY、MANY_TO_MANY）、link_table_name text（多对多必填，形如 `ext.<a>_<b>_links`）、fk_column_name text（一对一与一对多必填）、on_delete text（固定 'RESTRICT'）、definition_version bigint、status text、公共列。

约束与索引：`pk_custom_relations`、`ux_custom_relations_code`、`ix_custom_relations_from_object_type_created_at`、`ck_custom_relations_cardinality`、`ck_custom_relations_on_delete`、`ck_custom_relations_shape`（按 cardinality 判定 link_table_name 与 fk_column_name 的必填组合）。无行级策略。

##### 3.2.4 platform_meta.custom_indexes（部署级）

列：id、security_level、code、target_object_type text、index_kind text（SINGLE、COMPOSITE、UNIQUE）、column_codes text[]、physical_index_name text、definition_version bigint、status text、公共列。

约束与索引：`pk_custom_indexes`、`ux_custom_indexes_physical_index_name`、`ix_custom_indexes_target_object_type_created_at`、`ck_custom_indexes_kind`、`ck_custom_indexes_columns_len`（`array_length(column_codes,1) between 1 and 4`，SINGLE 时必须为 1）。该 CHECK 集合就是规格第 7.4 章公共能力基线的落地，函数索引、局部索引与 JSON 路径索引在此不存在可表达的取值。无行级策略。

##### 3.2.5 platform_meta.custom_views（部署级）

列：id、security_level、code、name、base_object_type text、select_field_codes text[]、filter_spec jsonb、sort_spec jsonb、definition_version bigint、status text、公共列。

约束与索引：`pk_custom_views`、`ux_custom_views_code`、`ix_custom_views_base_object_type_created_at`。视图为逻辑视图，不建数据库视图对象，理由是数据库视图会绕过统一前置查询服务的字段级与密级过滤；查询在应用层按 filter_spec 组装并经同一权限路径执行。无行级策略。

##### 3.2.6 platform_meta.ddl_plans（部署级）

列：id、security_level、config_release_order_id uuid（同 schema 外键）、plan_no text、target_schema text（固定 'ext'）、execution_mode text（ONLINE、MAINTENANCE_WINDOW）、statement_count int、impact_index jsonb、impact_capacity jsonb、impact_performance jsonb、impact_security jsonb、impact_migration jsonb、lock_timeout_ms int（默认 5000）、statement_timeout_ms int（默认 1800000）、status text（PLANNED、EXECUTING、SUCCEEDED、FAILED、ROLLED_BACK、DEFERRED_TO_WINDOW）、started_at timestamptz、finished_at timestamptz、elapsed_ms int、max_lock_wait_ms int、failure_reason text、公共列。

约束与索引：`pk_ddl_plans`、`ux_ddl_plans_plan_no`、`fk_ddl_plans_config_release_orders`、`ix_ddl_plans_status_created_at`、`ck_ddl_plans_execution_mode`、`ck_ddl_plans_status`、`ck_ddl_plans_lock_timeout`（`lock_timeout_ms <= 5000`，落实规格第 7.4 章的 5 秒上限，配置不得调高）、`ck_ddl_plans_statement_timeout`（`statement_timeout_ms <= 1800000`）。五个影响分析列一律非空，对应 PRD 第 10.4.1 节“发布前的影响分析覆盖索引、容量、性能、安全与迁移五项”。无行级策略。

##### 3.2.7 platform_meta.ddl_plan_steps（部署级，仅追加）

列：id、security_level、ddl_plan_id uuid、seq int、sql_kind text（CREATE_TABLE、ADD_COLUMN、CREATE_INDEX_CONCURRENTLY、ENABLE_RLS、CREATE_POLICY、RELAX_CHECK、DROP_INDEX_CONCURRENTLY、DROP_COLUMN、DROP_TABLE）、sql_text text、is_online boolean、started_at、finished_at、lock_wait_ms int、elapsed_ms int、outcome text（OK、TIMEOUT、ERROR、ROLLED_BACK）、error_text text、created_at、created_by、reverses_id uuid。

约束与索引：`pk_ddl_plan_steps`、`fk_ddl_plan_steps_ddl_plans`、`ux_ddl_plan_steps_plan_seq`、`ix_ddl_plan_steps_ddl_plan_id_created_at`、`ck_ddl_plan_steps_sql_kind`、`ck_ddl_plan_steps_outcome`。仅追加表，无 row_version。无行级策略。

##### 3.2.8 platform_meta.ui_layouts（部署级）

列：id、security_level、code、layout_kind text（FORM、LIST、HOME、MENU、BOARD）、target_object_type text（MENU 与 HOME 取固定值 `-`）、role_id uuid（空表示默认布局）、client_scope text（ALL、DESKTOP、MOBILE）、spec jsonb、definition_version bigint、status text（DRAFT、ACTIVE、SUPERSEDED、RETIRED）、公共列。

约束与索引：`pk_ui_layouts`、`ux_ui_layouts_kind_target_role_scope_version`（在 `(layout_kind, target_object_type, role_key, client_scope, definition_version)` 上，其中 `role_key text not null` 为应用写入的 `role_id::text` 或 `'-'`，同 3.2.2 的处理，避免函数索引与 NULL 分组歧义，理由与基线第 11.4 节空批次标识取 `'-'` 同构）、`ix_ui_layouts_status_created_at`、`ck_ui_layouts_kind`、`ck_ui_layouts_client_scope`、`ck_ui_layouts_status`。无行级策略。

##### 3.2.9 platform_meta.client_capability_values（部署级）

规格第 6.2 章取值矩阵的机器可读副本，是运行期能力闸的唯一判据。

列：id、security_level、capability_domain text、client text（win、mac、ios、android）、value text（FULL、SIMPLIFIED、VIEW_ONLY、NOT_APPLICABLE）、exemption_ref text、alternative_path text、frozen_hash text、公共列。

约束与索引：`pk_client_capability_values`、`ux_client_capability_values_domain_client`、`ix_client_capability_values_client_created_at`、`ck_client_capability_values_client`、`ck_client_capability_values_value`、`ck_client_capability_values_exemption`（value 为 VIEW_ONLY 或 NOT_APPLICABLE 时 `alternative_path` 必须非空，落实规格第 6.2 章“标注为仅查看或不适用的取值必须在本清单中有对应条目说明替代路径”）。

种子数据由迁移文件 backfill 写入，18 个能力域乘 4 端共 72 行，取值逐格照抄规格第 6.2 章矩阵。能力域码表见第 4.4 节。无行级策略。

##### 3.2.10 platform_meta.config_packages（部署级）
本表由阶段 3b 按裁定 A-27 随最小发布通道建立，本节列定义即其落地口径；本阶段只做列扩展与状态扩展，追加 PENDING_AUTOTEST、TEST_FAILED、TEST_PASSED、REJECTED、SIGNED_PENDING_RELEASE、SUPERSEDED 六个状态取值与自动测试相关列，不重复建表。

列：id、security_level、package_no text、name text、source text（IMPORTED、IN_PLACE）、git_ref text、manifest jsonb、content_hash text、item_count int、signature bytea、signature_key_ref text、signer_subject text、signed_at timestamptz、min_platform_version text、status text、rejected_reason text、公共列。

status 取值与 PRD 第 10.4.1 节状态表逐行对应：DRAFT（草稿）、PENDING_AUTOTEST（待自动测试）、TEST_FAILED（测试失败）、TEST_PASSED（测试通过）、PENDING_APPROVAL（待审批）、REJECTED（已驳回）、APPROVED（已批准）、SIGNED_PENDING_RELEASE（已签名待发布）、RELEASED（已发布）、ROLLED_BACK（已回退）、SUPERSEDED（已被替代）。

约束与索引：`pk_config_packages`、`ux_config_packages_package_no`、`ux_config_packages_content_hash`、`ix_config_packages_status_created_at`、`ck_config_packages_status`、`ck_config_packages_source`、`ck_config_packages_item_count`（`item_count between 1 and 2000`）、`ck_config_packages_signed`（status 为 SIGNED_PENDING_RELEASE 及之后时 `signature`、`signer_subject`、`signed_at` 三列均非空）。无行级策略。

##### 3.2.11 platform_meta.config_package_items（部署级）
本表由阶段 3b 按裁定 A-27 随最小发布通道建立，本节列定义即其落地口径，`item_hash` 算法与第 4.7 节一致；本阶段只做列扩展，不重复建表。

列：id、security_level、config_package_id uuid、item_kind text、item_code text、change_kind text（ADD、MODIFY、REMOVE）、applies_to_legal_entity_ids uuid[]（空数组表示全部法人）、before_spec jsonb、after_spec jsonb、item_hash text、sort_no int、公共列。

item_kind 取值封闭为 15 项：CUSTOM_OBJECT、CUSTOM_FIELD、CUSTOM_RELATION、CUSTOM_INDEX、CUSTOM_VIEW、UI_LAYOUT、FLOW_DEFINITION、AUTHZ_ROLE、AUTHZ_POLICY、AUTHZ_FIELD_GRANT、REPORT_DEFINITION、METRIC_DEFINITION、DASHBOARD_DEFINITION、PRINT_TEMPLATE、NOTIFY_RULE。其中 FLOW_、AUTHZ_、REPORT_、METRIC_、DASHBOARD_、PRINT_、NOTIFY_ 七类的定义对象表由流程、权限、报表、通知各自阶段拥有，本表只保存其序列化快照与哈希，落地由第 4.6 节的 `ConfigItemApplier` 端口交回各阶段实现。本阶段不定义那些表。

约束与索引：`pk_config_package_items`、`fk_config_package_items_config_packages`、`ux_config_package_items_pkg_kind_code`、`ix_config_package_items_config_package_id_created_at`、`ck_config_package_items_item_kind`、`ck_config_package_items_change_kind`、`ck_config_package_items_specs`（ADD 时 `before_spec` 为空且 `after_spec` 非空，REMOVE 时相反，MODIFY 时两者均非空）。无行级策略。

##### 3.2.12 platform_meta.config_release_orders（部署级）
本表由阶段 3b 按裁定 A-27 随最小发布通道建立，本节列定义即其落地口径；本阶段只做列扩展与状态扩展，不重复建表。

列：id、security_level、order_no text、config_package_id uuid、action text（RELEASE、ROLLBACK）、rollback_to_package_id uuid、execution_mode text（ONLINE、MAINTENANCE_WINDOW）、submitted_by uuid、approved_by uuid、approval_ref text、reauth_ref text、scheduled_window_start timestamptz、status text（SUBMITTED、APPROVED、REJECTED、QUEUED、EXECUTING、SUCCEEDED、FAILED、COMPENSATED、CANCELLED）、started_at、finished_at、elapsed_ms int、failure_reason text、公共列。

约束与索引：`pk_config_release_orders`、`ux_config_release_orders_order_no`、`fk_config_release_orders_config_packages`、`ix_config_release_orders_status_created_at`、`ck_config_release_orders_action`、`ck_config_release_orders_status`、`ck_config_release_orders_self_approval`（`approved_by is null or approved_by <> submitted_by`，落实规格第 12.2 章申请人不可自审）、`ck_config_release_orders_rollback`（action 为 ROLLBACK 时 `rollback_to_package_id` 非空）。无行级策略。

##### 3.2.13 platform_meta.config_release_steps（部署级，仅追加）

列：id、security_level、config_release_order_id uuid、seq int、item_kind text、item_code text、applier text、phase text（DDL、METADATA、PROPAGATION）、outcome text（OK、FAILED、SKIPPED、COMPENSATED）、started_at、finished_at、elapsed_ms int、error_text text、created_at、created_by、reverses_id uuid。

约束与索引：`pk_config_release_steps`、`fk_config_release_steps_config_release_orders`、`ux_config_release_steps_order_seq`、`ix_config_release_steps_config_release_order_id_created_at`、`ck_config_release_steps_phase`、`ck_config_release_steps_outcome`。无行级策略。

##### 3.2.14 platform_meta.config_autotest_runs（部署级）

列：id、security_level、config_package_id uuid、suite text、outcome text（PASSED、FAILED、SKIPPED）、started_at、finished_at、elapsed_ms int、failure_count int、report jsonb、公共列。

suite 取值封闭为 8 项：SCHEMA_VALIDATION、IMPACT_ANALYSIS、RLS_MATRIX、ROLE_PREVIEW、FLOW_SEMANTICS、REPORT_PERMISSION、CAPABILITY_MATRIX、SOD_CHECK。

约束与索引：`pk_config_autotest_runs`、`fk_config_autotest_runs_config_packages`、`ux_config_autotest_runs_pkg_suite`、`ix_config_autotest_runs_config_package_id_created_at`、`ck_config_autotest_runs_suite`、`ck_config_autotest_runs_outcome`。无行级策略。

##### 3.2.15 platform_meta.config_edit_locks（部署级）

对应 PRD 附录乙 U-K-01 的临时取值。

列：id、security_level、item_kind text、item_code text、locked_by uuid、locked_at timestamptz、expires_at timestamptz、公共列。

约束与索引：`pk_config_edit_locks`、`ux_config_edit_locks_kind_code`、`ix_config_edit_locks_expires_at_created_at`、`ck_config_edit_locks_window`（`expires_at > locked_at`）。过期锁由 job-worker 的巡检任务按 `expires_at` 删除，这是基线第 3.6 节允许物理删除的两类之外的第三类，本阶段登记为对基线第 3.6 节的追加：低代码编辑锁属于短生命周期的协作锁，不承载任何业务事实，其清理与 `platform_msg` 的过期幂等键同类。无行级策略。

##### 3.2.16 platform_meta.config_release_mutex（部署级）

单行互斥表，用于串行化发布执行。基线第 3.10 节禁止部分索引，因此不能用带条件的唯一索引表达“同时只有一个执行中的发布单”。

列：id uuid（固定为全零 UUID）、security_level、holder_order_id uuid、acquired_at timestamptz、公共列。

约束与索引：`pk_config_release_mutex`、`ck_config_release_mutex_singleton`（`id = '00000000-0000-0000-0000-000000000000'::uuid`）。执行器以 `select ... for update` 取锁。无行级策略。

##### 3.2.17 platform_meta.brand_profiles（部署级）

对应 PRD 附录乙 U-K-07 的临时取值，把白标可配置项固定为下表列集。

列：id、security_level、code、product_name text、vendor_display_name text、app_identifier_win text、app_identifier_mac text、app_identifier_ios text、app_identifier_android text、logo_attachment_object_id uuid、splash_attachment_object_id uuid、login_background_attachment_object_id uuid、theme_primary_color text、theme_accent_color text、notify_template_set_code text、signing_identity_ref text、distribution_channel text（APP_STORE、ENTERPRISE_MDM）、store_policy_check_passed_at timestamptz、status text（DRAFT、ACTIVE、SUPERSEDED）、公共列。

约束与索引：`pk_brand_profiles`、`ux_brand_profiles_code`、`ix_brand_profiles_status_created_at`、`ck_brand_profiles_distribution_channel`、`ck_brand_profiles_color`（`theme_primary_color ~ '^#[0-9A-Fa-f]{6}$'`，accent 同）、`ck_brand_profiles_status`。`signing_identity_ref` 只存 `secret://` 引用，不存密钥材料，照抄基线第 7.2 节。无行级策略。

##### 3.2.18 platform_meta.client_releases（部署级）

列：id、security_level、client text（win、mac、ios、android）、version text、build_no bigint、brand_profile_id uuid、artifact_hash text、artifact_size_bytes bigint、min_supported_version text、is_forced_security_update boolean、rollout_percent smallint、rollout_legal_entity_ids uuid[]、rollout_department_ids uuid[]、released_at timestamptz、withdrawn_at timestamptz、status text（DRAFT、ROLLING_OUT、FULL、WITHDRAWN）、release_notes text、公共列。

约束与索引：`pk_client_releases`、`ux_client_releases_client_version`、`fk_client_releases_brand_profiles`、`ix_client_releases_status_created_at`、`ck_client_releases_client`、`ck_client_releases_rollout_percent`（`between 0 and 100`）、`ck_client_releases_status`、`ck_client_releases_notes_len`（`char_length(release_notes) <= 2000`）。无行级策略。

##### 3.2.19 platform_meta.extensions（部署级）

服务端 WASM 插件与桌面端原生插件共用一张登记表，落实规格第 9.3 章“签名、版本锁定、能力声明、最小权限授予和审计要求对以下形态一致”。

列：id、security_level、code、name、kind text（SERVER_WASM、DESKTOP_NATIVE）、version text、publisher_subject text、artifact_hash text、artifact_size_bytes bigint、signature bytea、signature_verified_at timestamptz、capability_manifest jsonb、resource_limits jsonb、target_platforms text[]（DESKTOP_NATIVE 时取 win、mac 的子集）、status text（REGISTERED、PENDING_APPROVAL、APPROVED、ENABLED、DISABLED、REVOKED）、disabled_reason text、consecutive_failures int、approval_ref text、公共列。

约束与索引：`pk_extensions`、`ux_extensions_code_version`、`ix_extensions_status_created_at`、`ck_extensions_kind`、`ck_extensions_status`、`ck_extensions_artifact_hash`（`artifact_hash ~ '^[0-9a-f]{64}$'`）、`ck_extensions_consecutive_failures`（`>= 0`）。无行级策略。

##### 3.2.20 platform_meta.extension_capability_grants（部署级）

列：id、security_level、extension_id uuid、capability text、object_type text、field_codes text[]、granted_by uuid、approval_ref text、granted_at timestamptz、revoked_at timestamptz、公共列。

capability 取值封闭为 4 项，是首版扩展能力集的全部：READ_OBJECT_FIELDS（读取由调用方按本授予裁剪后传入的字段）、COMPUTE_ONLY（纯计算，无输入裁剪之外的任何能力）、DEVICE_PRINTER（桌面端打印机）、DEVICE_SMARTCARD（桌面端 USB Key 与智能卡）。网络、文件、密钥、数据库四类能力在此不存在可表达的取值，落实规格第 9.3 章“插件默认没有网络、文件、密钥或业务数据权限”。

约束与索引：`pk_extension_capability_grants`、`fk_extension_capability_grants_extensions`、`ux_extension_capability_grants_ext_cap_object`、`ix_extension_capability_grants_extension_id_created_at`、`ck_extension_capability_grants_capability`、`ck_extension_capability_grants_device_kind`（capability 为 DEVICE_ 开头时对应扩展的 kind 必须为 DESKTOP_NATIVE，由应用层同事务校验并在此以 CHECK 无法表达的部分交由用例断言）。无行级策略。

##### 3.2.21 platform_meta.extension_invocations（法人级，仅追加）

列：id、legal_entity_id uuid、security_level、data_scope_tags text[]、extension_id uuid、caller_process text（core、worker）、caller_user_id uuid、caller_device_id uuid、entry_point text、input_hash text、output_hash text、fuel_consumed bigint、memory_peak_bytes bigint、duration_ms int、outcome text、error_text text、occurred_at timestamptz、created_at、created_by、reverses_id uuid。

outcome 取值：OK、TRAP、TIMEOUT、FUEL_EXHAUSTED、MEMORY_LIMIT、CAPABILITY_DENIED、HOST_ERROR、THROTTLED。

约束与索引：`pk_extension_invocations`、`fk_extension_invocations_extensions`、`ix_extension_invocations_legal_entity_id_created_at`、`ix_extension_invocations_extension_id_occurred_at`、`ck_extension_invocations_outcome`。

行级策略按基线第 3.8 节模板生成 `rls_extension_invocations_le`，并 `enable` 加 `force`。`error_text` 在写入前经 `foundation::Redacted` 处理，不得包含插件输入的明文。

##### 3.2.22 platform_meta.client_bootstrap_dispatches（法人级，仅追加）

落实规格第 7.4 章“自定义对象下发到客户端时随会话一并下发对象结构、字段密级、权限策略和声明式规则版本，下发范围可审计”。

列：id、legal_entity_id uuid、security_level、data_scope_tags text[]、user_id uuid、device_id uuid、client text、bootstrap_hash text、custom_object_codes text[]、rule_versions jsonb、ui_layout_versions jsonb、capability_snapshot_hash text、brand_profile_code text、dispatched_at timestamptz、created_at、created_by、reverses_id uuid。

约束与索引：`pk_client_bootstrap_dispatches`、`ix_client_bootstrap_dispatches_legal_entity_id_created_at`、`ix_client_bootstrap_dispatches_user_id_dispatched_at`、`ck_client_bootstrap_dispatches_client`。行级策略按模板生成 `rls_client_bootstrap_dispatches_le`。

#### 3.3 ext schema 下自定义对象物理表的生成模板

每个自定义对象生成一张 `ext.<code>` 表，由迁移生成器统一产出，不允许手写变体。列顺序固定为基线第 4 节公共列在前、单据或档案专属列居中、自定义字段在后。

```sql
create table ext.<code> (
  id uuid not null,
  legal_entity_id uuid not null,
  security_level smallint not null default <对象密级>,
  data_scope_tags text[] not null default '{}',
  row_version bigint not null default 1,
  created_at timestamptz not null default now(),
  created_by uuid not null,
  updated_at timestamptz not null default now(),
  updated_by uuid not null,
  -- 单据类追加
  doc_no text not null,
  status text not null,
  -- 档案类追加
  code text not null,
  is_active boolean not null default true,
  deactivated_at timestamptz null,
  -- 自定义字段按 custom_fields 顺序追加，一律可空
  constraint pk_<code> primary key (id),
  constraint ck_<code>_status check (status in (...))
);
alter table ext.<code> enable row level security;
alter table ext.<code> force row level security;
create policy rls_<code>_le on ext.<code>
  using (legal_entity_id = nullif(current_setting('app.legal_entity_id', true), '')::uuid)
  with check (legal_entity_id = nullif(current_setting('app.legal_entity_id', true), '')::uuid);
create index ix_<code>_legal_entity_id_created_at on ext.<code> (legal_entity_id, created_at);
create unique index ux_<code>_legal_entity_id_doc_no on ext.<code> (legal_entity_id, doc_no); -- 单据类
```

自定义索引在此之后以 `create index concurrently` 追加。多对多关系生成 `ext.<a>_<b>_links` 表，列为 `id`、`legal_entity_id`、公共列、`<a>_id`、`<b>_id`，同 schema 内建真实外键 ON DELETE RESTRICT，`ux_<a>_<b>_links_pair` 在 `(legal_entity_id, <a>_id, <b>_id)` 上。跨 schema 的引用不建外键，照抄基线第 3.3 节。

#### 3.4 迁移编号与顺序

迁移文件路径 `db/migrations/platform_meta/`，历史表 `platform_meta.refinery_schema_history`。`db/migrations/order.toml` 中 platform_meta 已在平台组内，本阶段不改动模块顺序。下列时间戳为占位取值，开工时按实际时间重取，相对顺序不得改变。

| 序 | 文件名 | 内容 |
|---|---|---|
| 1 | V202704060900__platform_meta_custom_object_model.sql | custom_objects、custom_fields、custom_relations、custom_indexes、custom_views 五表 |
| 2 | V202704060905__platform_meta_ddl_plan.sql | ddl_plans、ddl_plan_steps |
| 3 | V202704060910__platform_meta_ui_layouts.sql | ui_layouts |
| 4 | V202704060915__platform_meta_client_capability_values.sql | client_capability_values 建表 |
| 5 | V202704060920__platform_meta_backfill_capability_matrix.sql | 72 行种子数据，逐格照抄规格第 6.2 章 |
| 6 | V202704060925__platform_meta_alter_config_package.sql | 对阶段 3b 已建的 config_packages 与 config_package_items 做列扩展：追加自动测试相关列，把遗留的 PENDING_REVIEW 行改写为 DRAFT 后按第 3.2.10 节的十一态重建 `ck_config_packages_status` |
| 7 | V202704060930__platform_meta_config_release.sql | 对阶段 3b 已建的 config_release_orders 做列扩展与状态扩展（本阶段第 3.2.12 节列集相对最小通道的增量），并新建 config_release_steps、config_autotest_runs、config_edit_locks、config_release_mutex 四表 |
| 8 | V202704060935__platform_meta_backfill_release_mutex_row.sql | 互斥表单行种子，`created_by` 取 `foundation::SYSTEM_PRINCIPAL_ID` |
| 9 | V202704060940__platform_meta_brand_profiles.sql | brand_profiles |
| 10 | V202704060945__platform_meta_client_releases.sql | client_releases |
| 11 | V202704060950__platform_meta_extensions.sql | extensions、extension_capability_grants |
| 12 | V202704060955__platform_meta_extension_invocations.sql | extension_invocations 含 RLS |
| 13 | V202704061000__platform_meta_client_bootstrap_dispatches.sql | client_bootstrap_dispatches 含 RLS |
| 14 | V202704061005__platform_meta_grant_ext_schema.sql | 为 ep_migrator 授予 ext schema 的 DDL 权限，为 ep_app_rw 授予 ext schema 的表数据读写默认权限，为 ep_analyst_ro 授予只读默认权限 |

每个文件头部带 `-- rollback:` 段。第 14 个文件的回退说明注明只能用升级前备份，理由是撤回 schema 级默认权限会使已建的自定义对象表不可访问。

---

### 4. 领域模型与关键算法

#### 4.1 核心结构体与枚举

以下类型位于 `ep-platform-meta` 与 `ep-platform-release`，均只用 foundation 类型，不含任何 IO。凡已在 `ep-foundation` 冻结的类型本阶段一律引用不再定义，见裁定 A-01、A-03 与 A-20。

```rust
// ep-platform-meta::value
pub enum FieldDataType { Integer, Decimal { precision: u8, scale: u8 }, Float, Boolean,
    String { max_len: u32 }, Text { max_len: u32 }, Date, Timestamp,
    Enum { values: Vec<EnumValue> }, Reference { target: ObjectType }, Json }

pub enum IndexKind { Single, Composite, Unique }
pub enum Cardinality { OneToOne, OneToMany, ManyToMany }
pub enum CapabilityValue { Full, Simplified, ViewOnly, NotApplicable }
// ClientKind 由阶段 1 按裁定 A-03 冻结，CapabilityDomain 与 ActionClass 由阶段 1 按裁定 A-20 冻结，
// 三者均位于 ep-foundation，本阶段只 use 不再定义

// ep-platform-meta::model
pub struct CustomObject { id: Id, code: ObjectCode, security_level: SecurityLevel,
    is_document: bool, doc_type_code: Option<DocTypeCode>, fields: Vec<CustomField>,
    indexes: Vec<CustomIndex>, definition_version: u64, status: ObjectStatus }

// ep-platform-meta::rule
pub enum RuleExpr { Lit(RuleValue), FieldRef(FieldPath), LineAgg{ kind: AggKind, path: FieldPath },
    Cmp{ op: CmpOp, l: Box<RuleExpr>, r: Box<RuleExpr> },
    Logic{ op: LogicOp, args: Vec<RuleExpr> },
    Arith{ op: ArithOp, l: Box<RuleExpr>, r: Box<RuleExpr> },
    Between{ v: Box<RuleExpr>, lo: Box<RuleExpr>, hi: Box<RuleExpr> },
    InSet{ v: Box<RuleExpr>, set: Vec<RuleValue> },
    IsNull(Box<RuleExpr>), Today, PeriodOf(Box<RuleExpr>),
    WasmCall{ extension_code: String, entry_point: String, args: Vec<RuleExpr> } }

// ep-platform-release::model
pub enum PackageStatus { Draft, PendingAutotest, TestFailed, TestPassed, PendingApproval,
    Rejected, Approved, SignedPendingRelease, Released, RolledBack, Superseded }
pub enum ReleaseOrderStatus { Submitted, Approved, Rejected, Queued, Executing,
    Succeeded, Failed, Compensated, Cancelled }
pub enum ItemKind { /* 15 项，见 3.2.11；本枚举与 ConfigPackageItem 由阶段 3a 在 crates/platform/release/src/port/config_item.rs 交付，见裁定 A-19 */ }
pub enum ChangeKind { Add, Modify, Remove }
```

#### 4.2 配置包状态机

状态与流转逐条对应 PRD 第 10.4.1 节的表，守卫条件如下。

| 起点 | 终点 | 触发 | 守卫条件 |
|---|---|---|---|
| Draft | PendingAutotest | 提交自动测试 | 包内容项数在 1 至 2000 之间；包体积不超过 64 MiB；每项 `item_hash` 与其 `after_spec` 的 SHA-256 一致 |
| PendingAutotest | TestPassed | 平台 | 8 个 suite 的 outcome 全为 PASSED 或 SKIPPED，且 SKIPPED 仅允许出现在该包不含对应 item_kind 时 |
| PendingAutotest | TestFailed | 平台 | 任一 suite 为 FAILED |
| TestFailed | Draft | 配置管理员修改 | 持有该包全部内容项的编辑锁 |
| TestPassed | PendingApproval | 提交发布单 | 包已锁定不可再改；`min_platform_version` 不高于当前版本 |
| PendingApproval | Approved | 审批通过 | 审批人不等于提交人；审批人具备配置发布审批权限项；审批链无越权跳过 |
| PendingApproval | Rejected | 审批驳回 | 同上审批人条件 |
| Approved | SignedPendingRelease | 平台签名 | 签名密钥可解引用；`content_hash` 与包实际内容一致 |
| SignedPendingRelease | Released | 执行发布单 | 执行模式与 `ddl_plans.execution_mode` 一致；若为 MAINTENANCE_WINDOW 则必须落在已登记的停机窗口内；互斥锁取得成功 |
| Released | RolledBack | 回退发布单 | 回退目标为上一 Released 包；该目标在保留窗口内（最近 10 个且不早于 180 天）；回退发布单本身已完成审批 |
| Released | Superseded | 后续版本发布 | 平台自动置位，与新包置 Released 同事务 |

非法迁移一律返回 `BUSINESS_CONFLICT` 与 `PLATFORM.CONFIG_PACKAGE.*` 的对应码，不静默忽略。

#### 4.3 在线 DDL 计划生成与执行算法

输入为配置包中全部 CUSTOM_ 前缀内容项的差集，输出为一份 `ddl_plans` 与其有序 `ddl_plan_steps`。

步骤如下。

1. 归一化。把 ADD、MODIFY、REMOVE 三类内容项按目标对象聚合，得到每个对象的目标结构；与 `platform_meta` 中该对象 ACTIVE 版本比对，得到列级与索引级差异。
2. 基线校验。字段类型必须落在规格第 7.4 章的 11 种之内；索引类型必须落在 3 种之内；JSON 列不得建索引也不得设 CHECK 校验；对象级密级与字段级密级必须有值，字段级为空时按对象级取值，两者都为空即拒绝。任一项不通过返回 `VALIDATION`，计划不生成。
3. 语句映射与执行模式判定。

| 差异 | 生成语句 | 执行模式 |
|---|---|---|
| 新增对象 | create table、enable rls、force rls、create policy、三条基线索引 | ONLINE |
| 新增可空列 | alter table add column，无默认或常量非易失默认 | ONLINE |
| 新增索引 | create index concurrently | ONLINE |
| 放宽长度 | drop constraint 旧 CHECK，add constraint 新 CHECK not valid，validate constraint | ONLINE |
| 新增多对多关系 | create table link，rls 五件套，唯一索引 | ONLINE |
| 收紧长度、改列类型、收紧非空、重建主键、删除列、删除表 | 对应 DDL | MAINTENANCE_WINDOW |

计划整体的执行模式取其中最严者。含 MAINTENANCE_WINDOW 语句的计划在未登记停机窗口时返回 `PLATFORM.DDL_PLAN.REQUIRES_MAINTENANCE_WINDOW`。

4. 五项影响分析。索引项给出新增索引数、该对象索引总数与配额比对、按现有行数与平均行宽估算的索引体积；容量项给出新增列的行宽增量、`ext` 下对象总数与字段总数与配额比对、磁盘剩余量；性能项给出每条 `create index concurrently` 按现有行数与认证期实测吞吐外推的预计耗时与 30 分钟上限比对；安全项给出密级赋值核对结论、RLS 模板齐备结论、新增查询入口是否已纳入 RLS 矩阵测试的结论；迁移项给出可逆性判定与回退方式，不可逆的注明只能用升级前备份或影子表。五项写入 `ddl_plans` 的五个 jsonb 列，缺一不可。
5. 执行。DDL 段的第一步是调用 `ep_platform_release::MigrationWindowGuard::assert_open(tx)`，该守卫由阶段 2 提供，见裁定 B-03；未持有已打开的迁移窗口时不建立任何连接、不执行任何语句，返回 `PLATFORM.DB.MIGRATION_WINDOW_CLOSED`，HTTP 409，category 为 BUSINESS_CONFLICT，该错误码由阶段 1 登记，本阶段只引用。守卫通过后由 job-worker 建立一条 `ep_migrator` 连接，会话上执行 `set lock_timeout = '5s'` 与 `set statement_timeout = '30min'`。逐条语句在自动提交下执行，理由是 `create index concurrently` 不能在事务块内。每条语句执行前后各取一次 `clock_timestamp()`，把等待锁的时长与执行时长写入 `ddl_plan_steps`。
6. 失败与回退。任一语句失败时立即停止，按已成功语句的逆序执行补偿语句：`create index concurrently` 对应 `drop index concurrently`，`add column` 对应 `drop column`，`create table` 对应 `drop table`，`create policy` 对应 `drop policy`，`validate constraint` 与 `add constraint` 对应 `drop constraint` 并恢复原 CHECK。补偿完成后计划置 ROLLED_BACK；若失败原因为 lock_timeout，计划另置 DEFERRED_TO_WINDOW 并把回退原因、操作对象与耗时写入审计，照抄规格第 7.4 章运行期口径，不判定为认证失败。
7. 元数据与 DDL 的一致化。DDL 无法与元数据写入同事务，因此采用两阶段：先在一个事务内把相关 `custom_objects` 与 `custom_fields` 置 PENDING_DDL 并写审计；执行 DDL；成功后在一个事务内置 ACTIVE、递增 `definition_version`、写审计与 Outbox 事件；失败后在一个事务内置 DDL_FAILED 并写审计。第 7 节新增的启动自检项保证不存在 ACTIVE 元数据而物理表缺失、或物理表存在而未开启行级安全的组合，任一不成立进程拒绝启动。

边界条件：单次计划的语句数上限 200；同一时刻只允许一份 EXECUTING 的计划，由发布互斥锁保证；`ext` 表在 RLS 策略创建成功之前不对任何应用账号开放，`grant` 语句排在 `create policy` 之后。

#### 4.4 能力等价矩阵与移动端豁免的运行期判定

能力域码由阶段 1 在 `ep-foundation` 的 `CapabilityDomain` 枚举中冻结，见裁定 A-20，本阶段不再定义。下表第 n 行的能力域码即该枚举第 n 个变体的序列化取值，与规格第 6.2 章矩阵 18 行一一对应，取值逐格照抄，不重述。

| 序 | 能力域码 | 规格第 6.2 章矩阵行 |
|---|---|---|
| 1 | crm.customer_360 | 客户档案与客户 360 查询 |
| 2 | clm.contract_esign | 合同条款与电子签章 |
| 3 | sales.order_fulfillment | 销售订单与履约 |
| 4 | procure.supplier_collab | 采购与供应商协同 |
| 5 | inventory.ledger_scan | 库存台账与收发扫码 |
| 6 | service.workorder_equipment | 售后工单与设备台账 |
| 7 | platform.approval_notify | 审批待办与站内通知 |
| 8 | project.task_milestone | 项目任务与交付节点 |
| 9 | mdm.master_data | MDM 主数据维护与审批 |
| 10 | platform.full_text_search | 全文检索 |
| 11 | ledger.posting_close | 财务过账与期末结账 |
| 12 | finance.settlement_view | 收付款登记与对账查看 |
| 13 | invoice.apply_issue | 发票申请与开具登记 |
| 14 | reporting.report_print | 报表与像素级打印 |
| 15 | platform.document_attachment | 文档与附件协作 |
| 16 | platform.admin_lowcode_ops | 系统管理、低代码配置与运维 |
| 17 | platform.extension_dynamic_code | 扩展插件与动态扩展代码 |
| 18 | portal.supplier_web | 外部门户 Web 形态：供应商 |

判定算法。

1. 常量由各业务阶段按裁定 A-20 在 `crates/contract/<module>/src/capability.rs` 中以 `<USECASE_SCREAMING>_DOMAIN` 与 `<USECASE_SCREAMING>_ACTION` 成对声明，本阶段只做运行期判定，不代其他阶段声明。本阶段自身平台路由的两个常量按同一命名规范声明在 `ep-platform-meta` 与 `ep-platform-release` 各自的 `capability.rs` 中，能力域一律取 `CapabilityDomain::PlatformAdminLowcodeOps`，由 `xtask configdoc` 断言每个路由都能解析到一对常量。同一用例同时落入两个能力域时按取值较低的所在行判定，照抄规格第 6.2 章。
2. core-server 的能力闸中间件在授权判定之前执行，读取请求头 `X-Client`，从 `platform_meta.client_capability_values` 取该能力域该端的取值。`portal` 与 `ops` 两个取值不参与本判定，门户不纳入四端等价，运维端只访问 `ops-agent` 暴露的端点。
3. 判定结果：
   - Full：放行。
   - Simplified：放行，但批量端点的单次上限按端下调，移动端 50 条，桌面端 200 条；超出返回 `VALIDATION`。业务对象、权限模型与流程结果不变，照抄规格第 6.2 章取值含义。
   - ViewOnly 且 action_class 不为 Read：拒绝，403 与 `PLATFORM.CLIENT_CAPABILITY.WRITE_NOT_AVAILABLE_ON_CLIENT`，`error.advice` 为该操作在桌面端完成的说明，`error.details` 携带 `alternative_path`，响应体 `data` 为空但响应头带 `X-Desktop-Handoff-Token`。
   - NotApplicable：返回 404 与 `PLATFORM.AUTHZ.NOT_FOUND_OR_DENIED`，不区分不存在与不承载，照抄基线第 5.5 节的存在性泄漏统一处理。
4. 桌面接续令牌是 U-K-08 的临时取值：一次性令牌，有效期 5 分钟，绑定用户、法人、目标能力域与目标单据标识，桌面端登录同一用户后凭该令牌拉起同一单据同一草稿。移动端界面同时提供“发送到桌面端继续”的入口，入口本身不构成写入。
5. 矩阵冻结校验：`client_capability_values` 全表按 `(capability_domain, client, value, exemption_ref)` 排序后计算 SHA-256，与二进制内置的冻结快照哈希比对，不一致即启动自检失败并返回退出码 78，落实规格第 6.2 章“本清单随本规格冻结”。
6. 客户端侧同样按引导数据中的矩阵取值渲染入口，ViewOnly 的能力域不渲染提交、审批与写入入口。客户端隐藏不构成访问控制，服务端闸是唯一权威，照抄 PRD 第 10.4.3 节。

#### 4.5 声明式规则与移动端 WASM 豁免

1. 规则以 AST 形式存储与下发，无任何代码下发。AST 节点数上限 500，求值深度上限 32，超限返回 `PLATFORM.RULE.AST_LIMIT_EXCEEDED`。
2. 数值一律 `foundation::Money`、`UnitPrice`、`Quantity`、`Rate` 四类，中间值以 Decimal 全精度保留，只在产出最终判定值时按基线第 3.5 节 round，舍入策略 `MidpointAwayFromZero`。
3. 规则含 `WasmCall` 节点即 `requires_wasm` 为真。`executable_on_client` 的取值为 `!requires_wasm`，四端一致。
4. 移动端遇到 `requires_wasm` 为真的规则时不求值，单据保存为本地草稿并置 `pending_central_validation`，不产生正式业务记录也不产生正式会计分录，照抄规格第 6.2 章与第 6.3 章。恢复连接后按该业务模块的正常提交端点提交，中心执行全部规则并把“该单据曾以待中心校验草稿提交”写入审计。
5. 联网状态下客户端可调用 `POST /api/v1/platform/rule-evaluations/actions/evaluate` 获得与中心一致的预校验结果，该端点只读不写，不建立业务记录。
6. 桌面端同样不在本地执行 WASM 计算，首版 WASM 宿主只在服务端 plugin-host 中存在，照抄规格第 9.3 章“首版服务端只有这一种扩展形态”。
7. 实现类型按裁定 B-05 固定：规则求值实现类型为 `AstRuleEvaluator`，位于 `crates/platform/meta/src/rule/`，装配进 core-server，实现阶段 3b 定义的 `ep_platform_flow::port::RuleEvaluator`；WASM 计算实现类型为 `PluginHostWasmCompute`，位于 `crates/adapter/wasm/`，装配进 plugin-host，实现阶段 3b 定义的 `ep_platform_flow::port::WasmComputePort`。`POST /api/v1/platform/rule-evaluations/actions/evaluate` 只调用 `AstRuleEvaluator`，本阶段不新建第二条求值路径。

#### 4.6 配置发布执行与回退算法

发布执行分三段，段间不共享事务。

段一，DDL 段。仅当包含 CUSTOM_ 前缀内容项且存在结构差异时执行，按第 4.3 节。

段二，元数据与配置段。在一个 `READ COMMITTED` 事务内，按 `sort_no` 升序对每个内容项调用对应的 applier。`ConfigItemApplier` 端口、`ItemKind` 枚举、`ConfigPackageItem` DTO 与 `ConfigItemApplierRegistry` 由阶段 3a 在 `crates/platform/release/src/port/config_item.rs` 交付，见裁定 A-19；下列签名即该文件的内容，其中 `Tx` 取自 `ep-foundation` 的 `port::tx`，见裁定 A-01。本阶段只实现 applier，不改端口签名。

```rust
pub trait ConfigItemApplier: Send + Sync {
    fn item_kind(&self) -> ItemKind;
    fn validate(&self, item: &ConfigPackageItem, ctx: &SecurityContext) -> Result<(), AppError>;
    fn apply(&self, tx: &mut dyn Tx, item: &ConfigPackageItem, ctx: &SecurityContext) -> Result<(), AppError>;
    fn revert(&self, tx: &mut dyn Tx, item: &ConfigPackageItem, ctx: &SecurityContext) -> Result<(), AppError>;
    fn requires_derived_store_rebuild(&self, item: &ConfigPackageItem) -> bool;
}
```

本阶段在 `ep-platform-meta` 中实现 CUSTOM_OBJECT、CUSTOM_FIELD、CUSTOM_RELATION、CUSTOM_INDEX、CUSTOM_VIEW、UI_LAYOUT 六个 applier。其余九个 applier 的归属按裁定 A-19 写死：FLOW_DEFINITION 的 `FlowDefinitionApplier` 与 NOTIFY_RULE 的 `NotifyRuleApplier` 由阶段 3b 在 `ep-platform-flow` 与 `ep-platform-notify` 实现；AUTHZ_ROLE 的 `AuthzRoleApplier`、AUTHZ_POLICY 的 `AuthzPolicyApplier`、AUTHZ_FIELD_GRANT 的 `AuthzFieldGrantApplier` 由阶段 4 在 `ep-platform-authz` 实现；REPORT_DEFINITION 的 `ReportDefinitionApplier`、METRIC_DEFINITION 的 `MetricDefinitionApplier`、DASHBOARD_DEFINITION 的 `DashboardDefinitionApplier`、PRINT_TEMPLATE 的 `PrintTemplateApplier` 由阶段 11 在 `ep-app-reporting` 实现。全部实现在 `apps/core-server/src/wiring.rs` 与 `apps/job-worker/src/wiring.rs` 注册到阶段 3a 提供的 `ConfigItemApplierRegistry`。本阶段不定义端口，也不定义那些阶段的表与接口。

段二成功后在同一事务内写入：`config_release_orders` 置 SUCCEEDED、`config_packages` 置 RELEASED、上一 RELEASED 包置 SUPERSEDED、审计事件、Outbox 事件 `platform.config_release.released.v1`。

段三，传播段。由 job-worker 消费 Outbox 事件执行，包含：任一 applier 的 `requires_derived_store_rebuild` 为真时按法人逐个重建内置搜索索引分区，重建期间该分区停止对外服务，重建后重放待处理的删除与更正事件并与来源做条数一致性校验与哈希抽样对账，照抄规格第 7.9 章；客户端引导数据版本号递增，使在线客户端在下一次引导时拉到新配置；站内通知按 PRD 第 10.5.2 节送达配置管理员。

失败与补偿：

- 段一失败，段二不执行，段一按第 4.3 节逆序补偿，发布单置 FAILED。
- 段二失败，事务回滚，段一按逆序补偿，发布单置 FAILED。
- 段三失败，发布已生效，进入 Outbox 重试与死信，按基线第 6.2 节的 8 次退避；连续失败进入死信并按 PRD 第 10.5.2 节通知责任人；权限、密级或分区规则变更的传播未完成前，受影响范围的检索与报表入口不可用，照抄规格第 7.9 章与 PRD 第 10.4.5 节。

回退算法：

- 回退发布单以上一 RELEASED 包为目标，按 `sort_no` 逆序对当前包的内容项调用 `revert`，使用 `before_spec` 恢复。
- 数据定制的回退取值（U-K-02 的临时取值）：新增字段与新增对象的回退只把元数据置 RETIRED，界面与 API 不再暴露该字段与该对象，物理列与物理表与其中数据一律保留，不执行 DROP。理由是 U-K-02 未决，而规格第 7.2 章与第 7.5 章要求业务数据只追加不覆盖，回退不得删掉已录入的业务数据。物理删除只能由单独的停机窗口计划发起，经双人审批，并按裁定 A-22 经 `ep_platform_file::port::disposal::DisposalPort` 交由阶段 14 的 `OpsDisposalService` 执行，走规格第 12.4 章的处置流程与处置清单；本阶段不实现任何物理删除路径。
- 回退同样触发受影响派生存储的重新打标，照抄 PRD 第 10.4.5 节。
- 可回退版本数与时间窗（U-K-02 的临时取值）：保留最近 10 个 RELEASED 包，且发布时间不早于 180 天；超出范围的包置 SUPERSEDED 且不可作为回退目标，尝试回退到该包返回 `PLATFORM.CONFIG_RELEASE_ORDER.ROLLBACK_TARGET_EXPIRED`。

#### 4.7 配置包签名与验签算法

1. 打包：`manifest.toml` 含包码、名称、版本、`min_platform_version`、内容项清单，每项含 `item_kind`、`item_code`、`change_kind`、`sort_no`、`item_hash`。`item_hash` 为该项 `after_spec` 的 JSON 规范化序列化（键按字典序、无空白、UTF-8）后的 SHA-256 十六进制小写。
2. `content_hash` 为 `manifest.toml` 字节流的 SHA-256。
3. 签名：以 `EP__RELEASE__SIGNING_KEY_REF` 指向的私钥对 `content_hash` 做 ECDSA P-256 签名，私钥由内置 KMS 或客户 HSM 持有，两种载体接口相同，照抄规格第 12.3 章。签名操作写审计事件，含密钥引用与版本，不含密钥材料。
4. 验签：导入时逐项重算 `item_hash` 并比对，重算 `content_hash` 并比对，验证签名，核对 `signer_subject` 在 `EP__RELEASE__TRUSTED_SIGNER_SUBJECTS` 内（厂商签名主体与客户签名主体并列受信，照抄规格第 3.2 章），核对 `min_platform_version` 不高于当前版本。任一不通过置 REJECTED 并返回对应错误码。

#### 4.8 服务端 WASM 插件沙箱算法

1. 加载：读 `platform_meta.extensions` 中 status 为 ENABLED 的记录，按 `artifact_hash` 从 `platform_file` 取制品，重算哈希比对，验签，核对 `capability_manifest` 与 `extension_capability_grants` 的交集非空且清单未声明超出授予的能力；任一不通过不加载并写审计。
2. 编译：wasmtime Component Model 编译一次，按 `artifact_hash` 缓存到 `EP__PLUGIN__COMPILE_CACHE_DIR`；进程重启时从缓存恢复，缓存文件损坏即重新编译。
3. 实例化：每次调用建立独立 `Store`，设置燃料上限、内存上限、表元素上限、实例数上限；开启 epoch interruption，tick 为 100 毫秒，交易路径的执行时限 2000 毫秒，Worker 路径 30000 毫秒。
4. 宿主导入函数只有四个：`log(level, msg)` 写日志且经 `Redacted` 处理，`now()` 返回调用方传入的固定时刻，`get_input(name)` 只返回按 grants 裁剪后的输入字段，`emit_result(json)` 返回结果。没有网络、文件、随机数、环境变量、时钟与线程的导入项。随机数如需要由宿主按调用注入固定种子，保证同一输入可重放。
5. 输入裁剪：调用方在 core-server 或 job-worker 侧按该扩展的 `READ_OBJECT_FIELDS` 授予逐字段裁剪，未授予的字段不进入 IPC 报文。裁剪在统一安全上下文内执行，先按法人、记录、字段与密级过滤，再按授予裁剪。
6. 结果处置：`emit_result` 的 JSON 按调用点声明的结果模式校验，不符返回 `PLATFORM.EXTENSION.MANIFEST_MISMATCH`。
7. 失败分类与错误映射：

| 情形 | 记录 outcome | 分类 | 错误码 |
|---|---|---|---|
| 声明或调用了未授予的能力 | CAPABILITY_DENIED | PERMISSION_DENIED | PLATFORM.EXTENSION.CAPABILITY_DENIED |
| 扩展已停用 | 不产生调用 | PERMISSION_DENIED | PLATFORM.EXTENSION.DISABLED |
| 燃料耗尽、内存超限、执行超时 | FUEL_EXHAUSTED、MEMORY_LIMIT、TIMEOUT | INFRASTRUCTURE | PLATFORM.EXTENSION.RESOURCE_LIMIT_EXCEEDED |
| WASM trap | TRAP | INFRASTRUCTURE | PLATFORM.EXTENSION.RESOURCE_LIMIT_EXCEEDED |
| plugin-host 不可达或 IPC 失败 | HOST_ERROR | INFRASTRUCTURE | PLATFORM.EXTENSION.HOST_UNAVAILABLE |
| cgroup 突发上限触发限流 | THROTTLED | INFRASTRUCTURE | PLATFORM.EXTENSION.HOST_UNAVAILABLE |

8. 自动停用：同一扩展同一入口连续失败达到 `EP__PLUGIN__AUTO_DISABLE_FAILURE_THRESHOLD`（默认 3）次时，把 `extensions.status` 置 DISABLED、`disabled_reason` 置具体原因，写审计并按 PRD 第 10.5.2 节通知，照抄规格第 9.3 章“插件崩溃、超时或越权调用只影响该子进程，宿主记录事件并按策略停用该插件”。成功一次即把 `consecutive_failures` 归零。
9. 事务边界约束：插件调用一律发生在写事务之外。规则求值在开启事务之前完成，把求值结果作为命令输入带入事务，落实基线第 10.3 节“事务内禁止外部调用与长时计算”。

#### 4.9 桌面端原生插件沙箱算法

1. 本地登记表位于客户端加密缓存库，列为插件码、版本、签名主体、制品哈希、能力清单、状态。加载前逐项核对签名主体、版本、哈希与能力清单，任一不匹配不加载，照抄规格第 9.3 章。
2. 本地登记表内容与中心 `platform_meta.extensions` 按 `artifact_hash` 核对，不一致不加载并向中心上报。
3. 插件运行在独立子进程，与客户端核心通过受限 IPC 通信。IPC 承载在 Windows 命名管道与 macOS Unix domain socket 上，帧格式复用基线第 2 节的 4 字节大端长度前缀加 JSON 体。子进程不共享客户端进程内存、不接收本地缓存数据库密钥、不接收会话令牌。
4. 传入子进程的数据只有该次操作的最小必要数据，且已按字段权限脱敏；子进程的日志按同一脱敏规则限制。
5. 崩溃、超时或越权调用只影响该子进程；宿主记录事件、按策略停用该插件，并把插件加载、能力授予、设备访问与异常退出写入客户端审计，随设备健康状态上报中心。
6. 设备策略关闭原生插件加载时，本机打印机与 USB Key 能力停用并显式降级；同时按规格第 12.4 章把该端的强制控制降级为浏览器门户端口径，高密级内容改为只读预览并禁止下载；降级事件与降级范围记入客户端审计并按客户与设备登记，不改变能力矩阵的冻结取值。

#### 4.10 客户端本地缓存与同步算法

1. 本地库为 SQLCipher，密钥为设备硬件保护密钥解封出的数据密钥，绑定 TPM、Secure Enclave 或 Keystore。
2. 缓存内容只有当前用户按权限可访问的数据子集。每条缓存记录携带来源对象标识、版本、法人标识、密级与数据范围标签，与规格第 7.9 章派生存储的标签口径一致。这是本阶段的判断，理由是客户端缓存同样是来源数据的副本，需要同样的裁剪依据才能在离线态正确隐藏无权字段。
3. 增量同步按 `row_version` 与 `updated_at` 拉取。同一记录冲突时以中心版本为准，本地内容不覆盖中心记录。
4. 清除触发：退出登录、设备注销、缓存超期。超期取值为 U-L-06 的临时取值，桌面端 14 天、移动端 7 天，可由设备策略下调，下调值随策略下发并写审计。
5. 断网期只能保存本地草稿，不产生正式业务记录与正式会计分录；恢复连接后由中心重新校验并提交。
6. 移动端本地缓存范围与单文件上限可按设备存储、网络与安全策略下调，下调后的数值随设备策略下发并可审计，照抄规格第 6.2 章。

#### 4.11 白标构建与签名流水线算法

1. 输入：源码 commit、`Cargo.lock`、`rust-toolchain.toml`、`brand.toml`、目标端集合、版本号。
2. 品牌资源校验：产品名称长度不超过 200，Logo 与启动页按各端要求的尺寸与格式集合逐项校验，主题色为六位十六进制，应用标识按各端命名规则校验。任一不通过返回 `PLATFORM.BRAND_PROFILE.ASSET_INVALID`。
3. 商店政策合规检查门禁，四项逐条判定，照抄规格第 3.6 章：应用主体与账号归属一致、品牌与标识不冲突、隐私声明完整、资质材料齐备。未通过不得提交商店，返回 `PLATFORM.BRAND_PROFILE.STORE_POLICY_CHECK_FAILED`。
4. 可复现构建：固定 `SOURCE_DATE_EPOCH`、启用 `--remap-path-prefix` 去除构建路径、锁定全部依赖版本、在容器化构建环境内执行。同一输入两次构建产出的未签名制品哈希必须一致，该结论是规格第 3.2 章私有构建级支持判据的前提。
5. 签名：桌面端 Windows 用 Authenticode，macOS 用 codesign 加公证；iOS 与 Android 按分发路径选择证书。厂商托管的签名私钥保存在硬件密码机，按客户隔离密钥域，签名操作双人控制并单独审计，照抄规格第 3.1 章。应用商店分发一律使用客户自有账号与证书，厂商不在自有账号下集中发布多个客户的白标应用，照抄规格第 3.6 章。
6. 产出登记：制品哈希、SBOM、签名主体、版本写入 `platform_meta.client_releases`。
7. 灰度：`rollout_percent` 与 `rollout_legal_entity_ids`、`rollout_department_ids` 三项共同决定可见范围，可按法人、部门或用户逐步启用，照抄规格第 18 章。`is_forced_security_update` 为真时不受灰度比例约束，全量下发，客户端在更新完成前拒绝进入业务界面。

---

### 5. API 契约

全部端点遵守基线第 5 章：路径前缀 `/api/v1`，JSON 字段 snake_case，成功与失败封套按基线第 5.2 节，写请求必带 `Idempotency-Key`，请求头集合按基线第 5.6 节。平台侧路径段取 `platform`；若平台阶段已确定另一取值，本阶段无条件改用其取值，该取值不影响本阶段其他设计。自定义对象的通用数据端点路径段取 `ext`，与 schema 名一致，不新增模块码。

下表的权限要求列写权限项名，具体角色映射由权限阶段承担。全部端点在授权判定之前先过第 4.4 节的能力闸，能力域为 `platform.admin_lowcode_ops`，四端取值为完整、完整、仅查看、仅查看，因此下表全部写端点在 iOS 与 Android 上返回 403 与 `PLATFORM.CLIENT_CAPABILITY.WRITE_NOT_AVAILABLE_ON_CLIENT`。

#### 5.1 数据定制

| 方法与路径 | 请求 | 响应 data | 主要错误码 | 幂等语义 | 权限项 |
|---|---|---|---|---|---|
| GET /api/v1/platform/custom-objects | 分页、排序、过滤按基线第 5.3 节 | 对象列表 | 无 | 只读 | lowcode.custom_object.view |
| POST /api/v1/platform/custom-objects | `{code, name, security_level, is_document, doc_type_code, fields:[...]}` | 对象详情，status 为 DRAFT | PLATFORM.CUSTOM_OBJECT.RESERVED_NAME、TYPE_NOT_IN_BASELINE、SECURITY_LEVEL_REQUIRED、QUOTA_EXCEEDED | Idempotency-Key 四元组，重放返回首次结果 | lowcode.custom_object.create |
| GET /api/v1/platform/custom-objects/{id} | 无 | 对象详情含字段、关系、索引、视图 | 404 统一按基线第 5.5 节 | 只读 | lowcode.custom_object.view |
| PATCH /api/v1/platform/custom-objects/{id} | 局部字段，必带 `row_version` | 新版本 | PLATFORM.CONCURRENCY.STALE_VERSION、PLATFORM.CONFIG_EDIT_LOCK.HELD_BY_ANOTHER_USER | 乐观锁加幂等键 | lowcode.custom_object.modify |
| POST /api/v1/platform/custom-objects/{id}/actions/retire | `{reason}` | 对象详情，status 为 RETIRED | BUSINESS_CONFLICT 若存在引用 | 幂等键 | lowcode.custom_object.retire |
| POST /api/v1/platform/custom-fields | `{owner_kind, custom_object_id 或 core_object_type, code, name, data_type, ...}` | 字段详情 | 同上 | 幂等键 | lowcode.custom_field.create |
| POST /api/v1/platform/custom-indexes | `{target_object_type, code, index_kind, column_codes}` | 索引详情 | PLATFORM.CUSTOM_OBJECT.INDEX_KIND_NOT_IN_BASELINE、QUOTA_EXCEEDED | 幂等键 | lowcode.custom_index.create |
| POST /api/v1/platform/custom-objects/actions/plan-ddl | `{config_package_id}` | `{ddl_plan_id, execution_mode, statements:[...], impact:{index,capacity,performance,security,migration}}` | PLATFORM.DDL_PLAN.REQUIRES_MAINTENANCE_WINDOW | 幂等键，同一包重复调用返回同一计划 | lowcode.custom_object.plan_ddl |
| POST /api/v1/platform/config-edit-locks | `{item_kind, item_code}` | `{lock_id, expires_at}` | PLATFORM.CONFIG_EDIT_LOCK.HELD_BY_ANOTHER_USER | 幂等键，同一用户重复取锁续期 | lowcode.config.edit |
| DELETE /api/v1/platform/config-edit-locks/{id} | 无 | 空 | 404 | 幂等键 | lowcode.config.edit |

自定义对象的数据端点由平台按对象码自动注册，路径与形状固定：

| 方法与路径 | 说明 |
|---|---|
| GET /api/v1/ext/{object-code} | 列表，分页排序过滤按基线第 5.3 节，默认排序单据类按 `created_at desc, id desc`，档案类按 `code asc` |
| POST /api/v1/ext/{object-code} | 新建，单据类由 `ep-platform-sequence` 取号，编号格式按基线第 11.1 节 |
| GET /api/v1/ext/{object-code}/{id} | 详情，字段按字段级权限与密级裁剪 |
| PATCH /api/v1/ext/{object-code}/{id} | 更新，必带 `row_version` |
| POST /api/v1/ext/{object-code}/{id}/actions/{verb} | 状态机动作，verb 取值来自该对象的状态机定义 |

#### 5.2 界面定制

| 方法与路径 | 请求 | 响应 data | 主要错误码 | 幂等语义 | 权限项 |
|---|---|---|---|---|---|
| GET /api/v1/platform/ui-layouts | 按 layout_kind、target_object_type、role_id、client_scope 过滤 | 布局列表 | 无 | 只读 | lowcode.ui_layout.view |
| POST /api/v1/platform/ui-layouts | `{code, layout_kind, target_object_type, role_id, client_scope, spec}` | 布局详情 | PLATFORM.UI_LAYOUT.FIELD_HIDING_NOT_ACCESS_CONTROL、PLATFORM.UI_LAYOUT.CAPABILITY_VALUE_NOT_MODIFIABLE | 幂等键 | lowcode.ui_layout.create |
| PATCH /api/v1/platform/ui-layouts/{id} | 局部字段加 `row_version` | 新版本 | PLATFORM.CONCURRENCY.STALE_VERSION | 乐观锁加幂等键 | lowcode.ui_layout.modify |
| POST /api/v1/platform/ui-layouts/actions/preview-as-role | `{layout_id, role_id, sample_record_id}` | `{rendered_spec, returned_fields, withheld_fields}` | 无 | 只读 | lowcode.ui_layout.preview |

`preview-as-role` 的 `withheld_fields` 只返回字段码不返回值，用于核对无权字段确实不返回而非仅不显示，落实 PRD 第 10.4.3 节的验证要求。

保存布局时的校验：若 `spec` 中某字段被标为隐藏而该字段在该角色的字段权限中为可见，返回 `PLATFORM.UI_LAYOUT.FIELD_HIDING_NOT_ACCESS_CONTROL` 并指出该字段；若 `spec` 试图为某能力域提供高于矩阵取值的入口，返回 `PLATFORM.UI_LAYOUT.CAPABILITY_VALUE_NOT_MODIFIABLE`。

#### 5.3 配置发布与回退

| 方法与路径 | 请求 | 响应 data | 主要错误码 | 幂等语义 | 权限项 |
|---|---|---|---|---|---|
| POST /api/v1/platform/config-packages | `{name, source:"IN_PLACE", items:[...]}` | 包详情，status 为 DRAFT | PLATFORM.CONFIG_PACKAGE.ITEM_LIMIT_EXCEEDED | 幂等键 | lowcode.config_package.create |
| POST /api/v1/platform/config-packages/actions/import | `{attachment_object_id}` | 包详情 | SIGNATURE_INVALID、ITEM_HASH_MISMATCH、SIGNER_NOT_TRUSTED、PLATFORM_VERSION_TOO_LOW | 幂等键，同一 `content_hash` 重复导入返回既有包 | lowcode.config_package.import |
| GET /api/v1/platform/config-packages/{id} | 无 | 包详情含内容项摘要与各 suite 结论 | 404 | 只读 | lowcode.config_package.view |
| GET /api/v1/platform/config-packages/{id}/diff | `?against={package_id}` | `{added:[...], modified:[...], removed:[...]}`，每项含 before 与 after 的规范化 JSON | 404 | 只读 | lowcode.config_package.view |
| POST /api/v1/platform/config-packages/{id}/actions/run-autotest | 无 | `{run_ids:[...]}`，同步返回受理回执，结果异步 | BUSINESS_CONFLICT 若状态不为 DRAFT | 幂等键 | lowcode.config_package.autotest |
| POST /api/v1/platform/config-packages/{id}/actions/submit-for-approval | `{note}` | 包详情 | AUTOTEST_NOT_PASSED | 幂等键 | lowcode.config_package.submit |
| POST /api/v1/platform/config-packages/{id}/actions/approve | `{note}` | 包详情 | SELF_APPROVAL_FORBIDDEN | 幂等键 | lowcode.config_package.approve |
| POST /api/v1/platform/config-packages/{id}/actions/reject | `{reason}` | 包详情 | 同上 | 幂等键 | lowcode.config_package.approve |
| POST /api/v1/platform/config-packages/{id}/actions/sign | 无 | 包详情含 `signer_subject` 与 `signed_at` | INFRASTRUCTURE 若密钥不可解引用 | 幂等键，已签名重复调用返回既有签名 | lowcode.config_package.sign |
| POST /api/v1/platform/config-release-orders | `{config_package_id, action, rollback_to_package_id, execution_mode, scheduled_window_start}` | 发布单详情 | ROLLBACK_TARGET_EXPIRED、REQUIRES_MAINTENANCE_WINDOW | 幂等键 | lowcode.config_release.submit |
| POST /api/v1/platform/config-release-orders/{id}/actions/execute | 无 | `{task_receipt_id}`，转后台任务 | CONCURRENT_RELEASE_IN_PROGRESS、DERIVED_STORE_REBUILD_REQUIRED、PLATFORM.DB.MIGRATION_WINDOW_CLOSED | 幂等键，重复执行返回同一回执 | lowcode.config_release.execute |
| POST /api/v1/platform/config-release-orders/{id}/actions/cancel | `{reason}` | 发布单详情 | BUSINESS_CONFLICT 若已进入 EXECUTING | 幂等键 | lowcode.config_release.execute |
| GET /api/v1/platform/config-release-orders/{id} | 无 | 发布单详情含逐步执行记录 | 404 | 只读 | lowcode.config_release.view |

发布执行是长时操作，按基线第 11.6 节同步等待上限 8 秒，`actions/execute` 一律返回任务回执并转后台任务，完成后由站内通知送达。

#### 5.4 扩展登记与沙箱

| 方法与路径 | 请求 | 响应 data | 主要错误码 | 幂等语义 | 权限项 |
|---|---|---|---|---|---|
| POST /api/v1/platform/extensions/actions/register | `{attachment_object_id, kind, code, version}` | 扩展详情含解析出的能力清单 | EXTENSION.SIGNATURE_INVALID、EXTENSION.MANIFEST_MISMATCH | 幂等键，同一 `artifact_hash` 返回既有登记 | ext.extension.register |
| POST /api/v1/platform/extensions/{id}/actions/request-approval | `{requested_capabilities:[...]}` | 扩展详情 | BUSINESS_CONFLICT | 幂等键 | ext.extension.register |
| POST /api/v1/platform/extensions/{id}/actions/approve | `{granted_capabilities:[...], note}` | 扩展详情含授予清单 | SELF_APPROVAL_FORBIDDEN | 幂等键 | ext.extension.approve |
| POST /api/v1/platform/extensions/{id}/actions/enable | 无 | 扩展详情 | EXTENSION.SIGNATURE_INVALID | 幂等键 | ext.extension.enable |
| POST /api/v1/platform/extensions/{id}/actions/disable | `{reason}` | 扩展详情 | 无 | 幂等键 | ext.extension.enable |
| GET /api/v1/platform/extensions/{id}/invocations | 分页与时间范围过滤 | 调用流水，含 outcome、耗时、燃料与内存峰值 | 404 | 只读 | ext.extension.view |
| POST /api/v1/platform/rule-evaluations/actions/evaluate | `{rule_code, rule_version, input}` | `{passed, message_code, details}` | RULE.EXPRESSION_PARSE_FAILED、RULE.AST_LIMIT_EXCEEDED、EXTENSION.* | 幂等键，纯只读 | lowcode.rule.evaluate |

#### 5.5 白标与客户端

| 方法与路径 | 请求 | 响应 data | 主要错误码 | 幂等语义 | 权限项 |
|---|---|---|---|---|---|
| GET /api/v1/platform/brand-profiles/current | 无 | 品牌配置，含产品名、Logo 与启动页的附件对象标识、主题色 | 无 | 只读，任何已认证用户可读 | 无需权限项 |
| GET /api/v1/platform/brand-profiles | 分页 | 品牌配置列表 | 无 | 只读 | brand.profile.view |
| POST /api/v1/platform/brand-profiles | 全字段 | 品牌配置详情 | BRAND_PROFILE.ASSET_INVALID、STORE_POLICY_CHECK_FAILED | 幂等键 | brand.profile.manage |
| POST /api/v1/platform/brand-profiles/{id}/actions/activate | 无 | 品牌配置详情 | BUSINESS_CONFLICT、BRAND_PROFILE.ASSET_INVALID | 幂等键 | brand.profile.manage |
| POST /api/v1/platform/client-releases | 全字段 | 版本详情 | VALIDATION | 幂等键 | client.release.manage |
| POST /api/v1/platform/client-releases/{id}/actions/roll-out | `{rollout_percent, rollout_legal_entity_ids, rollout_department_ids}` | 版本详情 | VALIDATION | 幂等键 | client.release.manage |
| POST /api/v1/platform/client-releases/{id}/actions/withdraw | `{reason}` | 版本详情 | 无 | 幂等键 | client.release.manage |
| GET /api/v1/platform/client-releases/check | `?client=&version=&build_no=` | `{action:"NONE" 或 "OPTIONAL" 或 "FORCED", target_version, release_notes, download_url}` | CLIENT_RELEASE.FORCED_UPDATE_REQUIRED 在其他端点上返回 | 只读 | 无需权限项 |
| GET /api/v1/platform/client-capabilities | `?client=` | 该端 18 个能力域的取值与替代路径 | 无 | 只读 | 无需权限项 |
| GET /api/v1/platform/client-bootstrap | `?client=` | 见下 | CLIENT_CAPABILITY.MATRIX_HASH_MISMATCH | 只读，但写一条 `client_bootstrap_dispatches` 与一条审计事件 | 无需权限项 |

`client-bootstrap` 的响应 data 结构：

```json
{
  "bootstrap_hash": "…",
  "capability_values": [{"capability_domain":"ledger.posting_close","value":"VIEW_ONLY","alternative_path":"…"}],
  "custom_objects": [{"code":"inspection_records","definition_version":7,"security_level":20,
    "fields":[{"code":"inspector","data_type":"STRING","max_length":200,"field_security_level":20}]}],
  "ui_layouts": [{"code":"…","layout_kind":"FORM","definition_version":3,"spec":{}}],
  "rules": [{"code":"…","version":5,"requires_wasm":false,"executable_on_client":true,"ast":{}}],
  "field_grants": [{"object_type":"ext.inspection_records","field_code":"inspector","can_read":true,"can_write":false}],
  "brand": {"product_name":"…","theme_primary_color":"#1A5FB4","logo_attachment_object_id":"…"},
  "device_policy": {"cache_ttl_days":7,"max_local_attachment_bytes":268435456,"native_plugin_enabled":false}
}
```

该端点每次调用写一条 `client_bootstrap_dispatches` 与一条审计事件，落实规格第 7.4 章“下发范围可审计”。响应缓存 TTL 由 `EP__CLIENT__BOOTSTRAP_CACHE_TTL_SECONDS` 控制，默认 300 秒；`definition_version` 或 `bootstrap_hash` 变化时客户端强制重取。

#### 5.6 版本化

本阶段全部端点为 v1。自定义对象端点的形状随对象定义变化，但路径与封套不变；新增字段属于向后兼容变更，不升主版本；删除或重命名字段由配置回退承担，不通过 API 版本表达。客户端必须容忍 `capability_values` 与 `item_kind` 出现未知取值并按未知降级处理，照抄基线第 5.6 节。

---

### 6. 并发与事务边界

#### 6.1 事务划分

| 操作 | 事务边界 | 隔离级别 | 锁策略 |
|---|---|---|---|
| 建模对象、字段、索引、视图、布局的增删改 | 一个用例一个事务 | READ COMMITTED | 行级乐观锁 `row_version`；编辑锁表按内容项粒度，TTL 1800 秒 |
| 配置包创建与内容项写入 | 一个事务 | READ COMMITTED | 无额外锁 |
| 配置包导入与验签 | 验签在事务外完成，写入在一个事务内 | READ COMMITTED | `ux_config_packages_content_hash` 承担去重 |
| 自动测试运行 | 每个 suite 一个只读事务 | RLS 矩阵与角色预览两个 suite 用 REPEATABLE READ 单事务，其余 READ COMMITTED | 只读，不占写锁 |
| 包签名 | 一个事务，签名调用在事务外完成后带入 | READ COMMITTED | 无 |
| 发布单受理 | 一个事务 | READ COMMITTED | 无 |
| 发布执行段一（DDL） | 每条语句自动提交，不在事务内 | 会话级 `lock_timeout=5s`、`statement_timeout=30min` | `create index concurrently` 不取表级排他锁；`add column` 取 ACCESS EXCLUSIVE 但受 5 秒超时约束 |
| 发布执行段二（元数据与配置） | 一个事务 | READ COMMITTED | 先取 `config_release_mutex` 的 `select for update` |
| 发布执行段三（传播） | 每个法人一个事务 | READ COMMITTED | 搜索索引分区重建期间该分区停止对外服务 |
| 扩展登记与能力授予 | 一个事务 | READ COMMITTED | 乐观锁 |
| 插件调用 | 不在任何写事务内 | 不适用 | 无数据库锁 |
| 调用流水写入 | 与调用点的业务事务同事务；无业务事务时单独一个事务 | READ COMMITTED | 仅追加 |

#### 6.2 发布的串行化

`platform_meta.config_release_mutex` 是单行表，发布执行器在段一之前以 `select * from platform_meta.config_release_mutex where id = '000…0' for update` 取锁，直到段二事务提交或整体补偿完成才释放。第二个并发发布请求在 `lock_timeout` 内取不到锁即返回 409 与 `PLATFORM.CONFIG_RELEASE_ORDER.CONCURRENT_RELEASE_IN_PROGRESS`。理由是基线第 3.10 节禁止部分索引，无法用带条件的唯一索引表达“同时只有一个执行中的发布单”。

段一在自动提交下执行，因此互斥锁必须在段一之前取得且跨段持有；实现方式为在段一开始前开启一个只持有互斥行的长事务，段一的 DDL 在另一条 `ep_migrator` 连接上执行，段二在第三条连接上执行并在提交时一并释放互斥行。三条连接的生命周期由发布执行器统一管理，异常路径一律先补偿再释放。

#### 6.3 幂等键与 Outbox

- HTTP 层：全部写端点必带 `Idempotency-Key`，作用域为法人、用户、端点、键值四元组，存储在 `platform_msg.idempotency_keys`，与业务写入同事务，照抄基线第 5.4 节。部署级配置端点的法人取该请求头 `X-Legal-Entity-Id` 的取值，即同一动作在不同法人上下文下发起视为不同的幂等作用域；这是本阶段的取值，理由是部署级配置对象无法人列但请求仍带法人上下文，若不纳入四元组会使两名分属不同法人的配置管理员的重放互相干扰。
- Outbox：发布成功在段二事务内写入 `platform.config_release.released.v1`，`idempotency_key` 取发布单标识。段三消费端幂等由 `platform_msg.inbox_consumptions(consumer, event_id)` 唯一约束保证。
- 重试退避照抄基线第 6.2 节的 8 次；全部失败置 DEAD 并写死信，按 PRD 第 10.5.2 节通知责任人。
- 死信重投必须记名并写审计；丢弃需要双人审批。

#### 6.4 失败重试与补偿

- 数据库序列化失败 40001 与死锁 40P01 在数据访问层重试 3 次，退避 50、150、450 毫秒，只对尚未产生外部可见副作用的事务重试，照抄基线第 8.4 节。DDL 段不适用该重试，DDL 失败一律走补偿。
- 段一的补偿是逆序 DDL；段二的补偿是事务回滚；跨段失败的补偿是段二未提交加段一逆序。补偿部分失败时该发布单置 COMPENSATED 并进入人工任务队列并告警，不得静默结束，照抄规格第 9.1 章。
- 插件调用失败不触发业务事务重试，由调用点按第 4.8 节的错误映射向上返回；连续失败达阈值自动停用该扩展。

#### 6.5 与规格第 10.2 章关账的关系

配置发布不产生会计分录，因此不进入关账受理前提的判定。但发布执行会占用磁盘 IO 与数据库连接，按规格第 13.1 章让路顺序属第 8 级“其余后台任务”；发布执行器在检测到期间关账请求已受理且尚未产生结论时暂停新的发布单执行，已在执行中的发布单继续执行至终态。这是本阶段的取值，理由是关账前的强制对账校验对同一快照敏感，DDL 在此期间执行会使校验语句的执行计划失效。

---

### 7. 配置项

全部键名前缀 `EP__`，层级用双下划线，结构体开启 `deny_unknown_fields`，照抄基线第 7.1 节。

| 键 | 类型 | 默认值 | 生效方式 |
|---|---|---|---|
| EP__CLIENT__BOOTSTRAP_CACHE_TTL_SECONDS | u32 | 300 | 下次引导生效 |
| EP__CLIENT__MOBILE_BATCH_MAX | u16 | 50 | 启动时生效 |
| EP__CLIENT__DESKTOP_BATCH_MAX | u16 | 200 | 启动时生效 |
| EP__CLIENT__HANDOFF_TOKEN_TTL_SECONDS | u32 | 300 | 启动时生效 |
| EP__CLIENT__CACHE_TTL_DAYS__DESKTOP | u16 | 14 | 下次引导生效 |
| EP__CLIENT__CACHE_TTL_DAYS__MOBILE | u16 | 7 | 下次引导生效 |
| EP__CLIENT__MAX_LOCAL_ATTACHMENT_BYTES__DESKTOP | u64 | 5368709120 | 下次引导生效 |
| EP__CLIENT__MAX_LOCAL_ATTACHMENT_BYTES__MOBILE | u64 | 268435456 | 下次引导生效 |
| EP__CLIENT__NATIVE_PLUGIN_ENABLED_DEFAULT | bool | true | 下次引导生效 |
| EP__LOWCODE__MAX_CUSTOM_OBJECTS | u16 | 200 | 启动时生效 |
| EP__LOWCODE__MAX_FIELDS_PER_OBJECT | u16 | 100 | 启动时生效 |
| EP__LOWCODE__MAX_INDEXES_PER_OBJECT | u8 | 5 | 启动时生效 |
| EP__LOWCODE__DDL_LOCK_TIMEOUT_MS | u32 | 5000 | 启动时生效，取值不得大于 5000 |
| EP__LOWCODE__DDL_STATEMENT_TIMEOUT_MS | u32 | 1800000 | 启动时生效，取值不得大于 1800000 |
| EP__LOWCODE__DDL_MAX_STATEMENTS_PER_PLAN | u16 | 200 | 启动时生效 |
| EP__LOWCODE__EDIT_LOCK_TTL_SECONDS | u32 | 1800 | 启动时生效 |
| EP__LOWCODE__RULE_MAX_AST_NODES | u16 | 500 | 启动时生效 |
| EP__LOWCODE__RULE_MAX_EVAL_DEPTH | u8 | 32 | 启动时生效 |
| EP__RELEASE__SIGNING_KEY_REF | string | "secret://config/release_signing#1" | 取用时解析，轮换不重启 |
| EP__RELEASE__TRUSTED_SIGNER_SUBJECTS | list of string | 空 | 启动时生效，为空时导入式包一律拒绝 |
| EP__RELEASE__ROLLBACK_KEEP_PACKAGES | u8 | 10 | 启动时生效 |
| EP__RELEASE__ROLLBACK_MAX_AGE_DAYS | u16 | 180 | 启动时生效 |
| EP__RELEASE__PACKAGE_MAX_BYTES | u64 | 67108864 | 启动时生效 |
| EP__RELEASE__PACKAGE_MAX_ITEMS | u16 | 2000 | 启动时生效 |
| EP__RELEASE__PAUSE_DURING_PERIOD_CLOSE | bool | true | 启动时生效 |
| EP__PLUGIN__MAX_INSTANCES | u16 | 8 | 启动时生效 |
| EP__PLUGIN__DEFAULT_FUEL | u64 | 200000000 | 启动时生效 |
| EP__PLUGIN__DEFAULT_MEMORY_BYTES | u64 | 67108864 | 启动时生效 |
| EP__PLUGIN__EPOCH_TICK_MS | u32 | 100 | 启动时生效 |
| EP__PLUGIN__CALL_TIMEOUT_MS__TRANSACTIONAL | u32 | 2000 | 启动时生效 |
| EP__PLUGIN__CALL_TIMEOUT_MS__WORKER | u32 | 30000 | 启动时生效 |
| EP__PLUGIN__AUTO_DISABLE_FAILURE_THRESHOLD | u8 | 3 | 启动时生效 |
| EP__PLUGIN__COMPILE_CACHE_DIR | path | "/var/lib/ep/plugin-host/cache" | 启动时生效 |
| EP__PLUGIN__TRUSTED_SIGNER_SUBJECTS | list of string | 空 | 启动时生效，为空时不加载任何扩展 |
| EP__BRAND__ACTIVE_PROFILE_CODE | string | "default" | 下次引导生效 |

敏感项按基线第 7.2 节只写 `secret://` 引用，内存中以 `secrecy::SecretString` 包装。

启动自检的追加项按裁定 C-25 改为命名项，基线第 7.3 节的十三项固定项一并按注册名标识，本阶段不再以序号称呼，需回写基线：

- `custom-object-ddl-consistent`：`ext` schema 下全部表均已 `ENABLE` 且 `FORCE` 行级安全，且各自存在 `rls_<table>_le` 策略；`platform_meta.custom_objects` 中 status 为 ACTIVE 的每个对象在 `ext` 下均有对应物理表，反之不存在孤立物理表。
- `client-capability-matrix-frozen`：`platform_meta.client_capability_values` 的内容哈希与二进制内置的冻结快照一致。

裁定 C-25 为本阶段固定了上述两个项名，因此原有的另外两项校验不再作为启动自检项：status 为 ENABLED 的扩展其制品可读、哈希一致、签名可验、`capability_manifest` 未超出已授予能力，改由第 4.8 节第 1 条的加载路径逐次校验，不通过即不加载并写审计；当前生效的品牌配置引用的附件对象存在且可读，改由 `POST /api/v1/platform/brand-profiles/{id}/actions/activate` 用例在激活前校验，不通过返回 `PLATFORM.BRAND_PROFILE.ASSET_INVALID`。

`--check` 模式按 `SelfCheckRegistry` 的注册顺序一并执行全部注册项，基线十三项在前，本阶段两项在后。

---

### 8. 测试计划

#### 8.1 单元测试

覆盖分支逐项列出。

1. DDL 计划生成器：七类差异各自的语句序列与执行模式判定；混合差异时整体执行模式取最严者；语句数超 200 时拒绝。
2. 基线校验器：11 种字段类型逐个通过、超出基线的类型逐个拒绝；3 种索引类型逐个通过、函数索引与局部索引与 JSON 路径索引的表达一律无法构造；JSON 列建索引拒绝；JSON 列设 CHECK 拒绝。
3. 密级校验：对象级为空拒绝；字段级为空时继承对象级；两者均为空拒绝。
4. 保留列名：公共列九项加八个专属列共十七个名字逐个拒绝。
5. 影响分析五项：各自在空差异、单表小差异、多表大差异三种输入下的输出结构完整性。
6. 配置包状态机：11 个状态、表列出的 12 条合法迁移全部通过；任意两状态之间的非法迁移全部返回对应错误码；自审批拒绝。
7. 发布单状态机：9 个状态与其合法迁移；回退目标过期拒绝；回退目标非上一 RELEASED 包拒绝。
8. 签名与验签：正确签名通过；篡改 manifest 一个字节拒绝；篡改任一内容项一个字节拒绝；签名主体不在信任列表拒绝；`min_platform_version` 高于当前版本拒绝。
9. 差异算法：新增、修改、删除三类内容项的 diff 输出；同一内容项在两包中完全一致时不进入差异。
10. 能力矩阵判定：四个取值乘五个动作类别共二十种组合的判定结果；同一用例落入两个能力域时取较低者；未知端取值按未知降级。
11. 规则 AST 求值：每个节点类型的正常与边界输入；Decimal 精度按基线第 3.5 节，含四舍五入中值远离零的三组样例；节点数 500 与 501 的分界；深度 32 与 33 的分界；除零、空值比较、集合为空、区间上下界颠倒五类边界。
12. 插件能力裁剪：授予字段子集时输入报文只含子集；未授予任何 `READ_OBJECT_FIELDS` 时输入为空；声明超出授予的能力拒绝加载。
13. 插件资源限额判定：燃料、内存、时限三类各自的中止分类映射。
14. 品牌资源校验：产品名超长、颜色格式错误、应用标识不合规、Logo 尺寸不符四类各自拒绝。
15. 编号生成：自定义单据对象的编号格式按基线第 11.1 节，法人加类型加年月三元组独立自增，流水溢出扩位；`doc_type_code` 与 `docs/data-dictionary.md` 单据类型码一节全量表重复时按裁定 C-26 拒绝并返回 `PLATFORM.CUSTOM_OBJECT.DOC_TYPE_CODE_CONFLICT`。

#### 8.2 领域属性测试（proptest）

对应规格第 17.2 章的领域属性测试类型，本阶段贡献四组不变量。

1. 规则求值全域性：任意合法 AST 在任意合法输入上求值不 panic、不越界、不整数溢出，且求值节点数不超过 AST 节点数乘输入明细行数。
2. 元数据往返：任意合法对象定义序列化为配置包内容项再解析回来，与原定义相等。
3. 发布与回退的元数据段幂等性：任意合法配置包 apply 后 revert，`platform_meta` 中除 `definition_version`、审计与仅追加表之外的全部行与 apply 之前逐列相等。
4. DDL 计划补偿完备性：对任意合法语句序列，在任意前缀位置注入失败后执行补偿，数据库结构（`information_schema` 的表、列、索引、策略四视图）与 `platform_meta` 均回到起点。

#### 8.3 集成测试（真实 PostgreSQL 16，禁用内存库与 mock）

每个用例独占一个数据库，按 `ep_test_<nanoid>` 建库，用例结束即删库。测试数据一律经 `ep-testkit` 构造器，禁止手写 INSERT。

| 序 | 场景 | 判据 |
|---|---|---|
| 1 | 新建自定义对象并发布 | `ext` 表存在；`enable` 与 `force` 行级安全均为真；`rls_<table>_le` 策略存在；三条基线索引存在 |
| 2 | 两法人越权矩阵 | 对该自定义对象执行读取、写入、更新、删除、聚合、排序、报表投影与错误信息泄漏八类，另覆盖两个复制角色与内部对账系统安全上下文的五个入口借用，全部不返回对方法人数据 |
| 3 | 新增可空列的锁持有 | 在 100 万行 `ext` 表上执行，`ddl_plan_steps.lock_wait_ms` 加执行时长不超过 5000 毫秒 |
| 4 | 新增索引的执行时长 | 在附录 A.3 基准数据集规模的 `ext` 表上 `create index concurrently` 不超过 30 分钟；建成后目标查询的 `EXPLAIN` 无顺序扫描 |
| 5 | 锁超时的自动回退 | 人为持有冲突锁使 `add column` 超时，计划置 ROLLED_BACK 与 DEFERRED_TO_WINDOW；审计事件含回退原因、操作对象与耗时；数据库结构无残留 |
| 6 | 元数据与 DDL 的一致化 | 在 DDL 执行成功后、元数据置 ACTIVE 之前杀死 job-worker，重启后启动自检项 `custom-object-ddl-consistent` 检出孤立物理表并拒绝启动；执行修复用例后该项通过 |
| 7 | 导入包篡改 | 篡改任一内容项一个字节，导入返回 `ITEM_HASH_MISMATCH` 且不落库 |
| 8 | 自动测试未通过阻止提交 | RLS_MATRIX 注入一条越权可读策略，suite 置 FAILED，提交发布单返回 `AUTOTEST_NOT_PASSED` |
| 9 | 自审批拒绝 | 提交人调用 approve 返回 `SELF_APPROVAL_FORBIDDEN` |
| 10 | 并发发布 | 两个发布单同时执行，第二个返回 `CONCURRENT_RELEASE_IN_PROGRESS`；数据库结构与元数据无交叉污染 |
| 11 | 发布幂等 | 同一 `Idempotency-Key` 重复调用 execute，只产生一份 `config_release_steps`，第二次响应头带 `Idempotent-Replay: true` |
| 12 | 回退不删数据 | 发布新增字段、录入 1000 行数据、回退，字段元数据置 RETIRED，物理列与 1000 行数据仍可由 `ep_analyst_ro` 读出 |
| 13 | 回退触发重新打标 | 权限类内容项回退后，搜索索引对应法人分区进入重建，重建期间检索入口返回 `DERIVED_STORE_REBUILD_REQUIRED`，重建完成后条数与来源一致 |
| 14 | 派生存储越权 | 以跨法人与跨密级安全上下文对自定义对象发起检索、排序与分面计数，均不返回无权数据 |
| 15 | 插件签名不符 | 篡改制品一个字节，enable 返回 `EXTENSION.SIGNATURE_INVALID`；对已 ENABLED 的扩展篡改制品后按第 4.8 节第 1 条不加载并写审计，后续调用返回 `PLATFORM.EXTENSION.DISABLED` |
| 16 | 插件燃料耗尽 | 构造死循环插件，调用返回 `RESOURCE_LIMIT_EXCEEDED`，`extension_invocations.outcome` 为 FUEL_EXHAUSTED |
| 17 | 插件自动停用 | 同一入口连续 3 次失败，扩展置 DISABLED，第 4 次调用返回 `EXTENSION.DISABLED`，审计与站内通知各一条 |
| 18 | 插件无 IO 能力 | 构造尝试打开套接字与文件的插件，编译期即因缺少导入项失败；宿主导入表断言只有四个函数 |
| 19 | plugin-host 零连接 | 断言 `pg_stat_activity` 中不存在来自 plugin-host 的连接；断言 plugin-host 的配置中不含数据库连接串 |
| 20 | 插件在事务外调用 | 静态检查 `ep-app-*` 的用例函数中不出现 plugin IPC 客户端符号；运行期断言插件调用发生时当前连接无活动事务 |
| 21 | 能力闸 | `X-Client: ios` 调用付款登记提交返回 403 与 `WRITE_NOT_AVAILABLE_ON_CLIENT` 并带 `X-Desktop-Handoff-Token`；`X-Client: win` 放行；`X-Client: ios` 调用扩展登记返回 404 |
| 22 | 矩阵冻结 | 篡改 `client_capability_values` 一行，启动自检项 `client-capability-matrix-frozen` 失败，进程启动返回退出码 78 与 `MATRIX_HASH_MISMATCH` |
| 23 | 引导下发可审计 | 调用 `client-bootstrap` 一次，`client_bootstrap_dispatches` 增一行含对象清单与规则版本，审计事件增一条 |
| 24 | ep_migrator 连接的启用与回收 | 一次含 DDL 的发布产生恰好两条审计事件（启用与回收）；发布前后 `pg_stat_activity` 中 `ep_migrator` 连接数为 0 |
| 25 | 配额上限 | 对象数达 200、单对象字段数达 100、单对象索引数达 5 时，再新增返回 `QUOTA_EXCEEDED` |
| 26 | 模块生命周期 | 对含自定义对象的模块执行停用与再启用，停用前后该对象的记录条数与校验和一致，停用期间授权查询与审计检索仍可执行，再启用后配置、权限授予与 Outbox 未投递条目差异为零 |
| 27 | 编辑锁 | 两名配置管理员同时编辑同一对象，第二人返回 `CONFIG_EDIT_LOCK.HELD_BY_ANOTHER_USER`；锁过期后可取得 |
| 28 | 关账期间暂停发布 | 受理一次关账请求后提交发布单执行，返回受理但排队；关账产生结论后自动继续执行 |
| 29 | 迁移窗口未打开 | 未登记打开的迁移窗口时执行含 DDL 段的发布，`MigrationWindowGuard::assert_open` 拒绝，返回 409 与 `PLATFORM.DB.MIGRATION_WINDOW_CLOSED`，全程 `pg_stat_activity` 中不出现 `ep_migrator` 连接；打开窗口后同一发布单执行成功 |
| 30 | 模块停用与再启用 | 经阶段 3b 的 `ModuleLicenseQuery::module_state` 把某模块置 INSTALLED_DISABLED 后，其定时任务不再触发、对外事件停止投递；再启用后两者恢复，配置、权限授予与 Outbox 未投递条目差异为零 |

外部电子签章不在本阶段范围内，本阶段不使用 wiremock 打桩。

#### 8.4 端到端测试

桌面端用 Playwright 驱动桌面 WebView，用 tauri-driver 驱动桌面壳；移动端用 XCUITest 与 Espresso。

四端矩阵覆盖按规格第 6.2 章：取值为完整或简化的能力域在该端跑通端到端场景；取值为仅查看或不适用的能力域按豁免清单载明的替代路径验证。
按裁定 A-23，业务闭环类端到端用例随各业务阶段的四端界面交付：下表 E1 至 E6 由阶段 5 至阶段 12 在自己的第 8 节测试计划中执行，本阶段只交付其运行所需的客户端壳、路由注册表与能力闸，并对执行证据逐条汇总；E7 至 E12 属壳层、发布链路与白标制品，由本阶段自行执行，其中 E9 以本阶段的自定义对象单据为被测对象，不依赖任何业务模块界面。四端验收矩阵由阶段 14 汇总。

| 序 | 用例 | 端 | 判据 |
|---|---|---|---|
| E1 | 黄金业务闭环 14 步全程 | Windows、macOS | 全程可执行，记录、校验、审计与结果与服务端集成测试一致 |
| E2 | 库存台账与收发扫码完整闭环 | 四端 | 移动端连续扫码 100 次识别率不低于 99%，单次识别不超过 1 秒 |
| E3 | 售后工单与设备台账完整闭环 | 四端 | 同一操作在任一端发起产生的记录与审计相同 |
| E4 | 审批待办与站内通知完整闭环 | 四端 | 站内通知在四端均可查看、跳转与标记，无权时按权限拒绝处理 |
| E5 | 六个仅查看能力域的写入入口缺失 | iOS、Android | 财务过账与期末结账、收付款登记与对账查看、发票申请与开具登记、报表与像素级打印、文档与附件协作、系统管理与低代码配置六类均不出现提交、审批与写入入口；进入写入路径时给出该操作在桌面端完成的说明并可发送到桌面端继续 |
| E6 | 含自定义对象与声明式规则的移动端场景 | iOS、Android | Rust 规则解释结果、字段级权限裁剪、审计结果与恢复连接后的中心重校验四者一致；含受限 WASM 计算的规则只能保存为待中心校验草稿，草稿在中心不产生业务记录 |
| E7 | 配置发布与回退全流程 | Windows | 差异审查、自动测试、审批、签名、发布、回退六步全程可执行，全过程记名记时写入审计 |
| E8 | 打印机与 USB Key 端到端 | Windows、macOS | 各一次成功；关闭原生插件加载后能力停用并显式降级，高密级内容改为只读预览并禁止下载，降级事件与范围记入客户端审计 |
| E9 | 断网草稿与恢复提交 | 四端 | 断网期无法完成审批、过账、开票与任何状态流转；草稿保存在本地加密缓存；恢复后由中心重新校验并提交；同一记录冲突以中心版本为准 |
| E10 | 白标制品四端启动 | 四端 | 应用图标、启动页、登录页与关于页显示 `brand.toml` 中的产品名与 Logo |
| E11 | 强制安全更新 | 四端 | `is_forced_security_update` 为真时客户端在更新完成前拒绝进入业务界面 |
| E12 | 读屏软件端到端下单 | 四端 | 各完成一次；WCAG AA 自动检查零严重问题 |

#### 8.5 性能与容量

附录 C.2 十二项门槛在附录 C.1 设备基线上复测，每项以旧机型或中端机结果为准，通过线逐项照抄附录 C.2，本计划不重写数值。

本阶段另设三项，均为本阶段新增取值：

- 插件调用在交易路径上的 P95 不超过 50 毫秒。理由是它落在规格第 16 章普通交易提交 3 秒通过线之内，必须给业务逻辑留出余量。
- `client-bootstrap` 的 P95 不超过 2 秒，按规格第 16 章常规交互通过线。
- 一次含 20 个内容项且含 3 条 DDL 语句的发布在附录 A.3 基准数据集上的段一加段二总时长不超过 30 分钟，与规格第 7.4 章迁移执行上限一致。

附录 A.1 度量清单内涉及自定义对象的查询在基准数据集上不得出现顺序扫描，阶段计划提交对应查询的 `EXPLAIN` 证据。

#### 8.6 覆盖率门槛

- `ep-platform-meta`、`ep-platform-release`、`ep-adapter-wasm` 属平台内核代码，行覆盖率不低于 85%。
- 客户端 Rust 核心 crate（`ep-client-core`、`ep-client-cache`、`ep-client-keystore`、`ep-client-plughost`、`ep-client-audit`）行覆盖率不低于 85%。这是本阶段在规格之上追加的取值，理由是这五个 crate 承担认证、安全策略、本地缓存加密与审计提交，属规格第 17.2 章“平台内核代码”的同类。
- 其余客户端 crate 与 `/clients/ui` 的 TypeScript 代码行覆盖率不低于 70%。
- 新增与修改代码行覆盖率不低于 80%。
- 工作区整体行覆盖率不低于 80%。
- 工具为 cargo-llvm-cov，CI 以 `--fail-under-lines` 强制；TypeScript 用 vitest 的 c8 覆盖率并同样在 CI 强制。`#[ignore]` 必须带 issue 编号且存活不超过本阶段。

#### 8.7 与规格第 17.2 章与第 17.3 章判据的对应

| 规格判据 | 本阶段对应测试 |
|---|---|
| 第 17.2 章 四端端到端测试 | E1 至 E12，按第 6.2 章矩阵确定各端场景范围；E6 直接对应“另执行一个含自定义对象与声明式规则的移动端场景” |
| 第 17.2 章 数据库适配认证中的自定义对象测试项与在线变更逐操作实测 | 集成 3、4、5、6；在线变更范围内操作的锁持有与执行时长不超过上限；新增可空列与新增索引达到在线能力底线 |
| 第 17.2 章 派生存储越权与删除更正传播测试 | 集成 13、14 |
| 第 17.2 章 模块生命周期测试 | 集成 26 |
| 第 17.2 章 身份与访问控制测试中的六类高风险操作四端口径一致 | E5、E7、E8，配合权限阶段的重新认证用例 |
| 第 17.2 章 数据保护控制与销毁证明测试的按端可执行范围 | E8、E9 |
| 第 17.3 章 权限不能跨法人、字段或密级越权 | 集成 2、8、13、14、21、22；`tests/rls_matrix` 独立测试目标追加自定义对象与自定义查询入口两类 |
| 第 17.3 章 审计链可验证 | 集成 5、17、23、24 产生的审计事件纳入审计链验证工具的抽样范围 |

---

### 9. 退出条件

下列各条全部达成才算本阶段完成，每条均可客观判定。

1. 四端制品各一份存在且可安装，均由同一份 `brand.toml` 产出，白标构建流水线的商店政策合规检查门禁四项在流水线日志中逐项显示通过。
2. 同一输入两次执行 `tools/epbrand` 产出的未签名制品哈希逐端一致。
3. 附录 C.2 十二项门槛在附录 C.1 设备基线上复测完成，通过或已获产品负责人书面批准豁免；全部原始测量数据进入证据包。
4. 规格第 6.2 章能力等价矩阵 18 行乘 4 端共 72 格逐格核对完成：取值为完整或简化的格由该能力域所属业务阶段按裁定 A-23 在其阶段跑通端到端场景，本阶段汇总其执行证据；取值为仅查看或不适用的格在豁免清单中有对应条目并按替代路径验证通过；服务端能力闸对 72 格的判定由本阶段逐格覆盖。
5. `platform_meta.client_capability_values` 的内容哈希与二进制内置冻结快照一致，启动自检项 `client-capability-matrix-frozen` 通过；人为篡改一行后进程以退出码 78 拒绝启动。
6. 五类定制各完成一次端到端发布与一次回退：数据定制与界面定制由本阶段在 `ep-platform-meta` 实现的六个 applier 承担；FLOW_DEFINITION 与 NOTIFY_RULE 由阶段 3b 的 `FlowDefinitionApplier` 与 `NotifyRuleApplier` 承担，三个 AUTHZ_ 类由阶段 4 的三个 applier 承担，四个报表类由阶段 11 的四个 applier 承担，归属按裁定 A-19，均在本阶段扩展后的发布通道上跑通，这九个类别至少各有一次 apply 与一次 revert 的执行记录。
7. 一次含 3 条 DDL 语句的在线发布在基准数据集上完成，单条语句锁持有不超过 5 秒，计划总执行时长不超过 30 分钟，`ddl_plan_steps` 中逐条有实测的锁等待与执行时长。
8. 人为制造锁超时后计划自动回退并转停机窗口，审计事件含回退原因、操作对象与耗时，数据库结构无残留。
9. 回退演练完成：按新增字段录入的业务数据在回退后仍可读出，字段元数据为 RETIRED，界面与 API 不再暴露该字段。
10. `tests/rls_matrix` 追加自定义对象与自定义查询入口两类后全部通过，八类越权面全覆盖。
11. plugin-host 的数据库连接数在全程为 0，宿主导入函数表只有四个函数，尝试网络与文件访问的插件在编译期即失败。
12. 插件连续失败自动停用生效，停用事件写入审计并经站内通知送达。
13. 桌面端打印机与 USB Key 各完成一次端到端验证；关闭原生插件加载后能力停用并显式降级，降级事件与范围记入客户端审计并按客户与设备登记。
14. 服务端能力闸对移动端六个仅查看能力域的写入端点一律返回 403 与 `PLATFORM.CLIENT_CAPABILITY.WRITE_NOT_AVAILABLE_ON_CLIENT` 并带 `X-Desktop-Handoff-Token`，桌面接续令牌可拉起同一单据同一草稿；移动端界面上这六个能力域无提交、审批与写入入口一条按裁定 A-23 由各业务阶段随其界面验收，本阶段汇总其执行证据。
15. 含受限 WASM 计算的规则在移动端只能保存为待中心校验草稿，恢复连接后由中心重新校验并写入审计，该场景在 iOS 与 Android 各执行一次。
16. 覆盖率门槛按第 8.6 节逐项达成，CI 强制生效。
17. 本阶段新增的 37 条错误码、10 个事件类型、19 张表、9 个指标、38 个配置项在 `docs/error-codes.md`、`docs/event-catalog.md`、`docs/data-dictionary.md` 与代码常量表中登记齐备，CI 一致性校验通过；本阶段引用但由阶段 1 按裁定 C-24 登记的 `PLATFORM.CONCURRENCY.STALE_VERSION`、`PLATFORM.AUTHZ.NOT_FOUND_OR_DENIED` 与 `PLATFORM.DB.MIGRATION_WINDOW_CLOSED` 不计入本阶段条数。
18. 依赖方向自检脚本通过：客户端 crate 不依赖任何 `ep-app-*` 与 `ep-adapter-db*`；`ep-platform-meta` 与 `ep-platform-release` 不出现 sqlx、reqwest、`std::fs`、`std::net` 与 `SystemTime::now` 符号。
19. 一次完整的配置发布与回退演练证据包归档，含差异审查记录、8 个 suite 的自动测试报告、审批与签名记录、执行耗时、锁持有时长、回退结果与审计链验证结论。
20. 本阶段的偏离项与新增决定（第 12 节）已回写共享技术基线，基线更新经平台架构负责人确认。
21. 模块许可的停用与再启用验收通过：按裁定 A-05，`ep-platform-license` 本体与其三张表由阶段 3b 交付，本阶段只保留一条验收，即某模块置 INSTALLED_DISABLED 后其定时任务停止、对外事件停发，再启用后两者恢复，执行记录见集成测试 30。
22. 本阶段全部平台路由的能力域码与动作类别常量已按裁定 A-20 声明，`xtask configdoc` 通过；自定义单据对象的 `doc_type_code` 与 `docs/data-dictionary.md` 单据类型码一节的全量表无重复，`xtask configdoc --check-doc-type-codes` 通过。
23. 含 DDL 段的发布在未打开迁移窗口时被 `ep_platform_release::MigrationWindowGuard::assert_open` 拒绝并返回 409 与 `PLATFORM.DB.MIGRATION_WINDOW_CLOSED`，留有一次拒绝与一次放行的执行记录；`ep_platform_flow::port::RuleEvaluator` 与 `WasmComputePort` 的实现类型 `AstRuleEvaluator` 与 `PluginHostWasmCompute` 已在 wiring 注册，规则求值端点只经 `AstRuleEvaluator`。

---

### 10. 与规格和 PRD 的对应

#### 10.1 规格条目

| 规格章节 | 本阶段实现的条目 |
|---|---|
| 3.1 私有化与白标 | 同一核心代码通过品牌配置、签名模块、低代码包与插件产生差异；独立产品名称、Logo、应用标识、证书与商店版本；白标构建、签名、灰度与发布流水线化；厂商托管签名私钥在硬件密码机内双人控制并单独审计 |
| 3.6 白标分发路径与签名身份归属 | 应用商店分发与企业签名/MDM 私有分发两条路径；商店政策合规检查门禁四项；被拒与超期的替代形态切换记录 |
| 5.1 平台内核 | 元数据、自定义对象、字段、关系、索引和视图；低代码表单与规则；配置发布；API、插件和连接器能力注册中的插件登记部分 |
| 6.1 技术路线 | Tauri 2 加 React/TypeScript；Rust 承担认证、安全策略、本地缓存数据库、同步、领域校验与设备能力；TypeScript 只承担界面；客户端全部网络 I/O 经 Rust 核心统一出口，TLS 通道与证书链校验由 Rust 核心执行 |
| 6.2 一致性与兼容 | 四端业务能力等价的运行期判定与豁免清单全部条目；18 行取值矩阵的机器可读落地与冻结校验；外设范围；后台常驻与长时任务；动态扩展代码；自定义对象与低代码规则的移动端豁免；本地缓存与文件；深度编辑；表格；财务与结账；收付款与对账；系统管理与配置发布；文档与 PDF 协作；分析与报表创作 |
| 6.3 本地缓存与设备 | 设备独立硬件保护密钥；本地缓存与凭据加密并绑定 TPM、Secure Enclave 或 Keystore；缓存只保存权限内子集；按记录版本与变更时间增量拉取；冲突以中心为准；断网只保存本地草稿；设备登记、远程注销与安全清除；缓存超期清除 |
| 6.4 文档、表格与设备 | 原生设备通过签名插件适配打印机与 USB Key/智能卡；移动端只使用随包静态签入的相机扫码；条码的企业自定义编码规则生成与识别 |
| 7.4 可定制数据库 | 自定义对象、字段、关系、校验、索引和视图；真实表加在线 DDL；不使用 EAV；公共能力基线的类型与索引限定；在线变更边界与运行期超限自动回退；对象级与字段级密级建模时赋值；随会话下发对象结构、字段密级、权限策略与规则版本且下发范围可审计；默认配额；发布前五项影响分析；声明式包进入 Git 支持差异审查与回退 |
| 7.9 派生存储安全继承 | 权限模型、密级规则或分区规则变更时受影响派生存储在变更生效前完成重建或重新打标；重建期间该分区停止对外服务；重建后重放与条数一致性校验 |
| 9.1 低代码能力 | 自定义对象、表单布局、列表列、首页、菜单与看板；声明式规则表达式与版本化规则；复杂计算调用受限 WASM 函数；不允许任意 JavaScript、Python、Shell 或本机程序进入核心环境 |
| 9.2 配置发布 | 开发测试与生产隔离；配置进入 Git 经差异审查、自动测试、审批与签名；验证失败阻止生产发布 |
| 9.3 模块与插件 | 签名 WASM Component；SDK 以 Rust 为主；插件默认无网络、文件、密钥与业务数据权限；必须声明能力、对象、字段与资源限额；权限经审批后最小授予；不能直连核心数据库也不能读取明文机密；扩展运行时三种形态；桌面端原生插件的九项安全边界 |
| 12.4 DLP 与隐私 | 桌面端与移动端的强制控制；关闭原生插件或设备不合规时的降级为门户端口径与只读预览禁止下载；降级事件与范围记入审计 |
| 13.1 正式拓扑 | plugin-host 与核心同机，按第 9.3 章承载服务端签名 WASM 组件；插件运行时 5% 的 CPU、内存与磁盘 IO 份额与突发上限外壳；插件运行时整体触及突发上限时限流其调用 |
| 15.3 运维中心 | 本阶段 9 个指标进入运维中心；配额触发限流事件记入运维中心 |
| 17.2 自动化测试 | 四端端到端测试；数据库适配认证的自定义对象测试项与在线变更实测；派生存储越权与传播测试；模块生命周期测试 |
| 17.3 强制不变量 | 权限不能跨法人、字段或密级越权；审计链可验证 |
| 18 升级、版本与生命周期 | 客户端支持分批发布与强制安全更新 |
| 附录 C 四端客户端 PoC 量化门槛 | C.2 十二项门槛在 C.1 设备基线上的首版验收复测 |

#### 10.2 PRD 条目

| PRD 节 | 本阶段实现的功能 |
|---|---|
| 8.4.1 报表设计器 | 报表定义作为配置包内容项进入发布通道；本阶段不实现设计器本体与取数 |
| 8.4.2 企业自定义指标 | 指标表达式复用本阶段的声明式表达式 AST 与解释器 |
| 8.4.5 报表类配置对象的状态与流转 | 四类报表配置对象通过本阶段的 `ConfigItemApplier` 进入统一发布与回退闭环 |
| 10.2.2 配置对象与配置操作 | 权限配置的新版本进入本阶段的发布流程后生效；发布前执行法人越权测试集 |
| 10.2.4 权限拒绝的用户可见行为 | 能力闸的拒绝按规格第 15.1 章分类，不泄露存在性 |
| 10.3 六类高风险操作 | 移动端取值为仅查看的高风险操作不提供提交入口并显式说明在桌面端完成；桌面端 USB Key 与智能卡的重新认证由本阶段的原生插件承载取证接口 |
| 10.4.1 配置发布的通用生命周期 | 11 个状态与其流转、触发条件与触发人逐行落地；发布前五项影响分析；发布与回退全过程记名记时写入审计 |
| 10.4.2 数据定制 | 能改与不能改逐条；在线与停机边界；建模必填的对象级与字段级密级；发布前影响分析加自动测试；运行期超限自动回退并转停机窗口且记入审计；回退按声明式包差异审查后执行 |
| 10.4.3 界面定制 | 表单布局、列表列与列顺序、首页、菜单、看板；不得以隐藏字段替代字段级权限；不得改变四端取值矩阵；不得引入任意 JavaScript；按角色预览核对无权字段确实不返回；按上一已发布版本整体回退 |
| 10.4.4 流程定制 | 流程定义作为配置包内容项进入同一发布通道与版本回退；本阶段不实现流程引擎本体 |
| 10.4.5 权限定制 | 权限内容项进入同一发布通道；变更生效前完成受影响派生存储的重建或重新打标；发布前执行法人越权测试集且新增自定义对象与新增查询入口必须通过 |
| 10.4.6 报表定制 | 报表类内容项进入同一发布通道与整体回退，且不影响已导出的历史文件 |
| 10.4.7 五类定制的共同硬边界 | 不得跨模块直接读写业务表；不得取得事务业务库直连；系统管理与低代码配置在移动端只提供查看与告警处理 |
| 10.7.1 白标的可见范围 | 品牌配置项清单在 `brand_profiles` 中冻结为 U-K-07 的临时取值 |
| 10.7.2 分发路径的可见差异 | 两条分发路径与切换记录 |
| 10.7.3 四端的用户可见差异 | 仅查看能力不出现写入入口并给出桌面端说明；同一操作在任一端结果相同；移动端不接受动态扩展代码；WASM 规则只能保存为待中心校验草稿；不承诺后台常驻；关闭原生插件后的降级；本地缓存与单文件上限可下调且可审计 |
| 10.7.4 数据保护控制按端的可见差异 | 三档控制强度与两类降级路径 |
| 11.5 离线、弱网与断网行为 | 断网只保存本地草稿；冲突以中心为准；缓存加密并绑定设备密钥；退出登录、设备注销或超期时清除 |
| 11.6 浏览器、客户端与设备要求 | 四端客户端形态；六类仅查看能力域；设备登记与远程注销；外设范围；WCAG AA 与主题与快捷键与命令面板与连续扫码；不做平板版式适配 |
| 附录乙 U-K-01 至 U-K-08、U-L-06 | 本阶段给出临时取值，见第 11.3 节 |

---

### 11. 风险与预留

#### 11.1 技术风险

| 风险 | 影响 | 控制 | 触发后的处置 |
|---|---|---|---|
| Tauri 2 移动端成熟度不足，附录 C.2 门槛在本阶段复测未通过 | 四端等价验收不成立 | 阶段 1 已完成 PoC 判定并冻结 Rust 核心接口语义；本阶段的 UI 层与 Rust 核心之间只有 Tauri IPC 一层桥 | 按规格第 6.1 章切换 Flutter UI，返工范围限于 IPC 桥改 FFI 桥、移动端生命周期与后台任务适配、推送、深链、平台插件与外设适配层；Rust 核心九个 crate 不动 |
| `create index concurrently` 在基准数据集上超过 30 分钟 | 新增索引失去在线变更能力，触及规格第 7.4 章的在线能力底线 | 影响分析的性能项在计划阶段外推并给出预警；单次计划语句数上限 200 | 该操作登记入停机窗口操作清单；若新增索引整体无法达到底线，按规格第 7.4 章交付说明必须明确降级为停机窗口变更，不得以在线 DDL 能力通过认证 |
| DDL 与元数据无法同事务导致的中间态 | 出现 ACTIVE 元数据而物理表缺失，或物理表存在而未开启行级安全 | 两阶段写入加启动自检项 `custom-object-ddl-consistent`；集成测试 6 专测该场景 | 进程拒绝启动并给出具体对象清单，由修复用例把元数据置 DDL_FAILED 或补建策略 |
| 插件执行占用同机资源影响交易时延 | 规格第 16 章 3 秒通过线受损 | plugin-host 独立 cgroup 与 5% 份额与三倍突发上限封顶；交易路径调用时限 2000 毫秒；调用在事务外 | 触及突发上限即限流其调用并记入运维中心；连续限流按规格第 15.3 章经 `ep_platform_obs::DegradationLedger` 的 `open` 与 `close` 登记降级窗口，降级类别取阶段 14 冻结的十八类之一，见裁定 A-26 |
| WASM 宿主自身的漏洞成为越权入口 | 对应规格第 21.7 章风险 | 宿主导入函数只有四个且无网络、文件、密钥与数据库；能力清单与最小权限授予；输入按字段权限裁剪后才进入 IPC；plugin-host 数据库连接数为 0 | 按规格第 3.3 章在本实例内停用该扩展，停用决定、影响范围与恢复条件记入审计 |
| 白标维护矩阵膨胀 | 对应规格第 21.8 章风险 | 单一核心加配置化品牌；客户不维护长期核心代码分支；构建、签名、灰度全流水线化；可复现构建使制品哈希可核对 | 品牌配置项清单冻结在 `brand_profiles` 的列集内，新增可配置项必须先改该表并回写 U-K-07 决策 |
| 配置回退删掉已录入业务数据 | 对应 U-K-02 未决 | 本阶段取值为回退只停用元数据不执行 DROP | 该取值在 U-K-02 决策后按决策调整；若决策要求物理删除，改由单独的停机窗口计划加双人审批，并按裁定 A-22 经 `DisposalPort` 交由阶段 14 的 `OpsDisposalService` 按规格第 12.4 章处置清单承担 |
| 生产环境内的就地创作与规格第 9.2 章开发测试生产隔离的张力 | 审计与合规口径受质疑 | 生产内的 DRAFT 状态配置对运行期一律不可见，运行期只读取 ACTIVE 版本；差异审查以 Git 中的声明式包为准；就地式包在签名后内容不可再改 | 若客户或审计要求更严，收窄为只接受 IMPORTED 来源的包，把 `source` 的可选取值在配置中限定为 IMPORTED |

#### 11.2 为后续阶段预留的扩展点

- `ConfigItemApplier` 端口由阶段 3a 按裁定 A-19 交付，其 `item_kind` 取值集合可扩展。新增一类定制内容项时只需实现该 trait 并在 wiring 注册到 `ConfigItemApplierRegistry`，发布链路、差异算法、签名、审批与回退全部复用，不改本阶段任何表。
- `CapabilityValue` 枚举与 `client_capability_values` 表结构支持新增能力域行与新增端列。恢复客户门户或经销商门户配套应用时，只需新增能力域行与新的 `client` 取值，不改判定算法。
- `extension_capability_grants.capability` 的取值集合封闭在 4 项。恢复服务端隔离容器形态或新增外设适配时，新增取值并同步扩展宿主导入函数表；宿主导入函数表的断言测试是新增能力必须同步修改的强制点。
- 客户端本地缓存的记录标签已按规格第 7.9 章口径携带，为后续恢复离线数据租约、租约到期锁定与撤销序列、离线草稿字段级合并预留了判定依据。
- `ep-client-plughost` 的能力清单与子进程 IPC 已与服务端 WASM 宿主共用同一份能力枚举与帧格式，为后续把桌面端插件形态统一到 WASM Component 预留了收敛路径。
- 白标构建流水线的可复现构建能力直接支撑规格第 3.2 章私有构建级源码许可的支持判据，后续开放该许可级别时不需要新建流水线。
- `ddl_plans` 的五项影响分析列为 jsonb，后续增加影响维度不需要迁移表结构。

#### 11.3 本阶段给出的临时取值与其阻塞判定

下列事项在 PRD 附录乙中为待决，本阶段给出技术侧临时取值以免阻塞实现。本阶段不被阻塞，但业务决策变更时的切换代价如下。

| 编号 | 本阶段临时取值 | 切换代价 |
|---|---|---|
| U-K-01 | 配置包按整包发布；并发编辑按内容项粒度加编辑锁，TTL 1800 秒 | 改为按单对象发布需要拆分 `config_release_orders` 与互斥锁的粒度，影响第 6.2 节的串行化设计，工作量中等 |
| U-K-02 | 保留最近 10 个已发布包且不早于 180 天；数据定制回退只停用元数据不删数据 | 改为物理删除需要新增停机窗口计划与双人审批路径，并按裁定 A-22 经 `DisposalPort` 交由阶段 14 的 `OpsDisposalService` 接入规格第 12.4 章处置清单，工作量中等 |
| U-K-03 | 对象数 200、单对象字段数 100、单对象索引数 5 | 只改配置默认值，工作量小；但需重跑第 8.5 节的容量项 |
| U-K-05 | 移动推送正文不携带任何业务字段，只含事项类型与关联编号 | 若允许携带需新增按密级的正文裁剪，工作量小 |
| U-K-07 | 白标可配置项固定为 `brand_profiles` 的列集，含主题色、登录页背景与通知模板集 | 新增可配置项需要迁移该表并同步 `tools/epbrand`，工作量小 |
| U-K-08 | 仅查看能力的提示文案为该操作在桌面端完成的一句话说明；提供桌面接续令牌与深链跳转 | 只改文案与是否启用跳转，工作量小 |
| U-L-06 | 本地缓存有效期桌面端 14 天、移动端 7 天；草稿超期提示确认后清除 | 只改配置默认值，工作量小 |
| U-A-05 | 分页与排序与筛选期间按基线第 11.5 节，本阶段不另设 | 无 |

---

### 12. 本阶段对共享技术基线的新增决定与偏离项

按基线第 0 节，下列各项在基线中未覆盖或需要追加，本阶段显式登记，阶段结束时回写基线。

1. 客户端代码位置与 crate 命名。基线第 1.1 节只覆盖服务端 workspace。本阶段新增 `/clients/` 为独立 Cargo workspace，crate 前缀 `ep-client-`，通过路径依赖复用 `ep-foundation`、`ep-contract-*` 与 `ep-platform-meta`，禁止依赖 `ep-app-*` 与 `ep-adapter-db*`。edition 2021，禁止 nightly，与基线一致。桌面壳与移动壳分别位于 `/clients/desktop` 与 `/clients/mobile`，其 `src/modules/<module>/` 为业务模块界面目录，按裁定 A-23 由阶段 5 至阶段 12 各自交付，本阶段只建立目录约定与路由注册表。
2. 非常驻工具目录。基线第 2 节的八个进程是常驻进程清单。本阶段新增 `/tools/` 目录承载 `epcfg`、`epbrand`、`epplug` 三个一次性命令行工具，不属于进程清单，不占用系统账户与 cgroup。
3. 客户端本地加密缓存库选型。取 SQLCipher，经 rusqlite 的 bundled-sqlcipher 特性引入。理由是附录 C.2 要求本地加密数据库随机读写吞吐不低于 20 MB/s、10 万行查询 P95 不超过 1 秒，纯 Rust 的嵌入式库在加密路径上尚无同等实测证据。该选型只作用于客户端，不触及基线第 3 节的服务端数据库约定。
4. 服务端 WASM 宿主选型。取 wasmtime 与 wasmtime-wasi，主版本在 workspace 根 `[workspace.dependencies]` 中锁定为 26 系列，只启用 Component Model，不启用任何 WASI 网络与文件能力。
5. 部署级配置表的归类。本阶段涉及的 20 张部署级表按基线第 3.8 节第四段归入“全局配置字典”类，不带 `legal_entity_id` 与 `data_scope_tags`，不建行级策略，其余公共列齐备。其中 17 张由本阶段建立，`config_packages`、`config_package_items` 与 `config_release_orders` 三张由阶段 3b 按裁定 A-27 建立并沿用同一归类。理由见第 3.1 节。
6. 唯一约束中的空值替代取值。`custom_fields.owner_key` 与 `ui_layouts.role_key` 在语义为空时取 `'-'`，与基线第 11.4 节空批次标识的理由同构：该列是唯一索引的组成键，NULL 在唯一约束中的语义会使重复定义得以并存。
7. 编辑锁的物理删除。基线第 3.6 节允许物理删除的表只有两类，本阶段追加第三类：`platform_meta.config_edit_locks` 的过期行由 job-worker 按 `expires_at` 清理。理由是它是短生命周期协作锁，不承载任何业务事实。
8. 平台侧 API 路径段取 `platform`，自定义对象数据端点路径段取 `ext`。两者都不新增模块码，`ext` 与 schema 名一致。
9. 自定义对象的领域事件统一为 `platform.custom_record.created.v1`、`platform.custom_record.updated.v1`、`platform.custom_record.state_changed.v1` 三个类型，具体对象由信封的 `aggregate_type` 承载为 `ext.<object_code>`。理由是不新增模块码。
10. 幂等作用域中法人维度对部署级端点的取值。取请求头 `X-Legal-Entity-Id` 的值，理由见第 6.3 节。
11. 启动自检项按裁定 C-25 改为按注册名标识，不再以序号称呼。本阶段追加 `custom-object-ddl-consistent` 与 `client-capability-matrix-frozen` 两项，前者一并覆盖原第 4 项对 `ext` schema 的扩展；扩展制品校验与品牌附件校验不再作为启动自检项，改由第 4.8 节加载路径与品牌激活用例承担。
12. 覆盖率门槛追加客户端 Rust 核心五个 crate 不低于 85%，TypeScript 界面包不低于 70%。
13. 关账受理期间暂停新发布单执行，取值见第 6.5 节。
14. 客户端本地缓存记录携带来源对象标识、版本、法人标识、密级与数据范围标签，与规格第 7.9 章派生存储同一口径。理由见第 4.10 节。

本阶段不偏离基线第 3.5 节的金额与数量精度、第 3.8 节的行级策略模板、第 5 章的封套与分页与幂等、第 6 章的事件与 Outbox、第 10.3 节的事务边界、第 10.4 节的分层边界。
