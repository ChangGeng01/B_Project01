## 阶段 13：四端客户端与低代码

本阶段承接规格第 6 章客户端与用户体验、第 7.4 章可定制数据库、第 9.1 章低代码能力与第 9.2 章配置发布的首版保留部分、第 9.3 章模块与插件中的桌面端签名原生插件形态、第 3.1 章与第 3.6 章白标与分发、第 12.4 章按端的数据保护控制，以及 PRD 第 8.4 节、第 10.4 节、第 10.7 节。按架构审计 N-01 与 ARCH-03，服务端 WASM 插件宿主、配置包签名与验签、配置包自动测试编排三项在首版整体删除，删除后失去落点的规格条文即第 9.1 章“复杂计算调用受限 WASM 函数”一句、第 9.2 章的自动测试与签名两句、第 9.3 章的服务端 WASM 形态段，按规格第 2.2 章的既定做法登记进第 5.7 章延期目录并需产品负责人批准，本阶段不承接这三项。本阶段不产生任何会计分录，也不新增任何账务口径，涉及账务的一律指向规格第 5.2 章事件-分录表，由财务阶段承担。
本阶段的交付边界按裁定 A-23 收窄：本阶段不交付任何业务界面，交付物只有客户端壳、路由注册表、能力矩阵闸、白标构建与四端制品；各业务模块的四端界面位于 `clients/desktop/src/modules/<module>/` 与 `clients/mobile/src/modules/<module>/`，由阶段 5 至阶段 12 各自交付，四端验收矩阵由阶段 14 汇总。裁定表中称为阶段 13b 的条目即本阶段。
本阶段按贯通线 T0 拆成先后两批，阶段范围归属不变，只改本阶段内部的工作次序。第一批是桌面壳最小切片，与 T0 同批交付，排在阶段 4 与阶段 3b 之后、阶段 5 全量开工之前，内容与判据见第 1.5 节。第二批是本阶段其余全部交付物，仍排在阶段 11 之后、阶段 14 之前，一律在 T0 已贯通的骨架上加厚，不再承担任何首次贯通性质的判据。调整后的阶段顺序为 1 → 2 → 3a → 4 → 3b → T0 → 5 → 9a → 8 → 6 → 7 → 10 → 11 → 9b → 13 → 14，阶段 12 在阶段 10 之后与阶段 11 并行。按架构审计 ARCH-03 第二档第 6 条，本阶段第二批不在关键路径上：它不被任何后继阶段的功能依赖，只被阶段 14 的四端验收矩阵汇总。

本计划遵守共享技术基线。凡基线已定死的取值直接引用，不重新决定。本阶段新增的决定在第 12 节集中列出，需在阶段结束时回写基线。

---

### 1. 交付物清单

本阶段结束时，下列各项存在且可运行。

#### 1.1 服务端可运行物

1. core-server 内新增的低代码建模 API、扩展登记 API、客户端引导 API、白标与客户端版本 API 与能力闸中间件，全部经 `/api/v1/platform/...` 暴露，可用 `curl` 完成第 5 节全部端点的往返。配置包与发布单的 API 归阶段 3b 的最小发布通道，本阶段不新增也不扩展。
2. job-worker 内在阶段 3b 发布执行器之上追加的 DDL 段编排、在线 DDL 执行器与派生存储重新打标任务，可由 Outbox 事件驱动跑通一次含 DDL 的发布与一次回退。
3. `platform_meta` 下 15 张新表与其迁移文件、回退说明、种子数据（能力等价矩阵 18 行乘 4 端共 72 行）。阶段 3b 已建的 `config_packages`、`config_package_items` 与 `config_release_orders` 三张表本阶段不做列扩展也不做状态扩展。
4. `docs/error-codes.md` 新增 27 条错误码、`docs/event-catalog.md` 新增 9 个事件类型、`docs/data-dictionary.md` 新增 15 张表条目，三处与代码常量表由 CI 校验一致。本阶段引用但由阶段 1 登记的 `PLATFORM.CONCURRENCY.STALE_VERSION`、`PLATFORM.AUTHZ.NOT_FOUND_OR_DENIED` 与 `PLATFORM.DB.MIGRATION_WINDOW_CLOSED` 三条不计入本阶段条数，见裁定 C-24。

#### 1.2 客户端可运行物

5. Windows 客户端安装包（msi 与便携 exe 各一）、macOS 客户端安装包（dmg，已签名并公证）、iOS 客户端（ipa，企业签名通道与商店通道各一份配置）、Android 客户端（aab 与 apk）。四份制品均由同一份 `brand.toml` 驱动产出，均可完成登录、设备登记、按能力矩阵渲染入口、离线草稿保存与恢复后提交。
6. 桌面端两个签名原生插件：打印机插件与 USB Key/智能卡插件，各自以独立子进程运行，含能力清单与本地登记表。
7. 客户端 Rust 核心 crate 集合（第 2 节列出），含本地加密缓存库、增量同步、原生插件宿主、统一网络出口与证书链校验。客户端不再内置声明式规则解释器，理由与去向见第 4.5 节。
8. 四端共用的 React/TypeScript 界面壳与共用组件包，含浅色、深色、高对比度三套主题，全键盘与命令面板，WCAG AA 自动检查零严重问题，以及按能力矩阵裁剪入口的路由注册表。按裁定 A-23，本包不含任何业务模块界面，业务模块界面由阶段 5 至阶段 12 交付到 `clients/desktop/src/modules/<module>/` 与 `clients/mobile/src/modules/<module>/`。

#### 1.3 工具与流水线

9. `tools/epbrand`：白标构建流水线入口，输入 `brand.toml` 与源码 commit，输出四端未签名制品与其哈希清单，再调用签名步骤产出签名制品；同一输入两次构建的未签名制品哈希一致。
10. 商店政策合规检查门禁脚本，四项检查项按规格第 3.6 章逐项判定，未通过即中断流水线。

#### 1.4 证据物

11. 附录 C.2 十二项门槛在 C.1 设备基线上的复测报告与全部原始测量数据。
12. 规格第 6.2 章能力等价矩阵 18 行、豁免清单每条替代路径的逐条核对表，含四端 E2E 执行证据；按裁定 A-23，业务闭环类用例的执行证据由阶段 5 至阶段 12 提交，本阶段只汇总，不自行执行。
13. 一次完整的配置发布与回退演练证据包：含差异审查记录、审批记录、执行耗时、锁持有时长、回退结果与审计链验证结论。发布通道归阶段 3b，本阶段只在其上执行数据定制与界面定制两类内容项的演练。

#### 1.5 与贯通线 T0 的关系

本阶段向 T0 贡献一个桌面壳最小切片，内容固定为下列五项，不含其余任何交付物：`/clients/desktop` 的 Tauri 2 桌面壳与其 Tauri IPC 命令表；`/clients/ui` 的路由注册表与浅色一套主题；`ep-client-core` 的会话、安全上下文、统一网络出口与证书链校验；core-server 的能力闸中间件，其判据取二进制内置的能力矩阵冻结快照，不读 `platform_meta.client_capability_values`；`GET /api/v1/platform/client-bootstrap` 的最小形态，只返回能力取值与品牌默认值。该切片的判据只有一条：T0 的那一条合同能在 Windows 桌面端建单，并在同一端看到 T0 的那张收入报表。

T0 不要求本阶段的下列各项，它们一律排在阶段 11 之后：移动端两端与其制品、白标 `brand.toml` 驱动的四端制品与商店合规门禁、本地加密缓存与离线草稿与增量同步、桌面端两个原生插件、能力矩阵表与其冻结比对、引导下发台账、深色与高对比度两套主题、低代码建模与配置发布的界面。T0 不要求四端、不要求 scale 数据集、不要求分支覆盖。

本阶段第二批的全部交付物在 T0 已贯通的骨架上加厚。原先由本阶段承担的首次贯通性质表述一律删除，M7 保留为全分支闭环判据，本阶段不重复承担。

---

### 2. crate 与进程归属

#### 2.1 服务端新增或改动的 crate

| crate | 归属进程 | 本阶段职责 | 依赖方向核对 |
|---|---|---|---|
| ep-platform-meta | core-server、job-worker | 自定义对象与字段与关系与索引与视图的建模、在线 DDL 计划与影响分析、界面布局、能力等价矩阵判定、自定义对象向权限与流程与搜索与报表的注册端口、六个 CUSTOM_ 与 UI_LAYOUT 类 `ConfigItemApplier` 实现 | 只依赖 ep-foundation 与其他 ep-platform-*，无 sqlx、无 reqwest |
| ep-platform-release | core-server、job-worker | 本 crate 由阶段 3b 按裁定 A-27 交付最小发布通道，`ConfigItemApplier` 端口与 `ConfigItemApplierRegistry` 由阶段 3a 按裁定 A-19 交付；本阶段只在其上追加内容项差异算法与 DDL 段编排两项，不改状态机、不加自动测试编排、不加签名 | 依赖 ep-foundation、ep-platform-meta、ep-platform-audit、ep-platform-outbox |
| ep-foundation | 全部 | 本阶段不新增也不改动 foundation 类型：`Tx`、`SnapshotCtx`、`UnitOfWork` 由阶段 1 按裁定 A-01 在 `port::tx` 中冻结，`SecurityContext` 与 `ClientKind` 按裁定 A-03 冻结，`ModuleCode`、`CapabilityDomain`、`ActionClass` 按裁定 A-20 冻结，`Redacted<T>` 同由阶段 1 提供，本阶段只引用 | 不依赖工作区内任何 crate |
| ep-platform-obs | ops-agent | 注册本阶段 5 个新指标 | 只登记，不改结构 |

本阶段不新增 platform crate（`ep-platform-release` 与 `ep-platform-license` 均由阶段 3b 交付，本阶段只在前者之上追加两项），不新增业务模块 crate，不新增进程，不新增 schema，不新增错误分类。按架构审计 N-01，`ep-adapter-wasm` 整只删除，`ep-adapter-ipc` 不再新增 plugin 通道因而本阶段不改动该 crate，`AstRuleEvaluator` 与 `ep_platform_flow::port::RuleEvaluator` 的实现一并删除，去向见第 4.5 节。

#### 2.2 客户端 crate（不属八进程，位于 workspace 之外的独立 Cargo workspace）

客户端代码位于仓库 `/clients/`，为独立 Cargo workspace，与服务端 workspace 通过路径依赖复用 `ep-foundation`、`ep-contract-*` 与 `ep-platform-meta`。客户端不得依赖任何 `ep-app-*`、`ep-adapter-db*` 与 `ep-platform-outbox`。

| crate | 平台 | 职责 |
|---|---|---|
| ep-client-core | 四端 | 客户端核心装配：会话、安全上下文、统一网络出口、TLS 与证书链校验、错误封套解析、能力矩阵本地判定、引导数据缓存 |
| ep-client-cache | 四端 | 本地加密缓存库：SQLCipher 打开与密钥解封、表结构、增量同步、冲突处理、超期清除、标签随行 |
| ep-client-keystore | 四端 | 设备硬件保护密钥的封装与解封：Windows DPAPI 加 TPM、macOS Keychain 加 Secure Enclave、iOS Secure Enclave、Android Keystore |
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
- 在线 DDL 执行需要 `ep_migrator`。落地方式为：job-worker 仅在存在一张已批准且含 DDL 段的发布单时，临时建立一条 `ep_migrator` 连接，执行完毕立即关闭。该连接计入基线第 2 节“迁移与应急临时连接另计不超过 10”的额度，同时按规格第 7.7 章把该账号的启用与回收各写一条审计事件。任何其他路径不得建立 `ep_migrator` 连接。
- 本阶段不新增任何常驻进程与常驻连接。按架构审计 N-01，plugin-host 进程连同系统账户 ep-plugin、cgroup 分片 app-plugin.slice 与 `/run/ep/ipc/plugin.sock` 一并删除：规格第 4.3 章的 apps 清单由八进程改为七进程，第 7.7 章连接枚举删去 plugin-host 一行，该行常驻连接本为 0，因此 42 与 52 两个总数不变，不需重算。

---

### 3. 数据库变更

#### 3.1 总则

本阶段全部新表落在 `platform_meta` 一个 schema 内。理由是本阶段不得定义其他阶段拥有的 schema 的表，而品牌配置、客户端版本、扩展登记三组虽不属狭义元数据，但都是部署级配置对象，与自定义对象元数据同属配置发布链路的内容项，集中在一个 schema 内使模块隔离边界与迁移边界一致。

表分两类。

- 部署级配置表：按基线第 3.8 节的第四类“全局配置字典”处理，不带 `legal_entity_id`，不带 `data_scope_tags`，不建行级策略。其可见性由对象级权限判定，不承载任何业务数据。这一归类是本阶段的显式判断，理由是低代码配置在本部署内跨两个法人共用，为其编造一个法人列会制造第二套隔离口径，与基线第 4 节反对 `tenant_id` 的理由同构。
- 法人级运行台账表：带 `legal_entity_id`，按基线第 3.8 节模板建行级策略，列全。

全部表带基线第 4 节公共列，顺序按基线；仅追加表按基线不带 `row_version`、`updated_at`、`updated_by`，改带 `reverses_id`。枚举一律 `text` 加 CHECK。时间列 `timestamptz`，日期列 `date`。主键 `uuid`，应用侧 UUIDv7。
按裁定 A-27，`platform_meta.config_packages`、`platform_meta.config_package_items` 与 `platform_meta.config_release_orders` 三张表由阶段 3b 随最小发布通道建立，第 3.2.10 至 3.2.12 节所列列定义即阶段 3b 的落地口径，本阶段不做列扩展也不做状态扩展，不重复建表；本阶段因此新建 15 张表。发布状态机固定为阶段 3b 已实现的 DRAFT、PENDING_APPROVAL、REJECTED、APPROVED、RELEASED、ROLLED_BACK 六态，差异审查由 `GET /api/v1/platform/config-packages/{id}/diff` 端点承载，不单列为状态。PRD 第 10.4.1 节十一态中其余五态的处置按架构审计 ARCH-03 第一档第 3 条：PENDING_AUTOTEST、TEST_FAILED、TEST_PASSED 随自动测试编排删除，SIGNED_PENDING_RELEASE 随包签名删除，四项一并登记进规格第 5.7 章延期目录；SUPERSEDED 折叠为 `config_packages.superseded_by_id` 一列上的事实，不单列状态，理由是它只被回退目标判定读取，没有独立的流转守卫。全部种子迁移与系统上下文写入的 `created_by` 一律取 `foundation::SYSTEM_PRINCIPAL_ID`，不得自选取值，见裁定 A-02。

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
本表由阶段 3b 按裁定 A-27 随最小发布通道建立，本节列定义即其落地口径；本阶段不做列扩展也不做状态扩展，不重复建表。

列：id、security_level、package_no text、name text、source text（固定 IN_PLACE）、git_ref text、manifest jsonb、content_hash text、item_count int、min_platform_version text、status text、superseded_by_id uuid、rejected_reason text、公共列。原 signature、signature_key_ref、signer_subject 与 signed_at 四列按架构审计 ARCH-03 第一档第 3 条随包签名一并删除，理由见第 4.7 节第 3 条。

status 取值封闭为六项：DRAFT（草稿）、PENDING_APPROVAL（待审批）、REJECTED（已驳回）、APPROVED（已批准）、RELEASED（已发布）、ROLLED_BACK（已回退）。被替代的事实写 `superseded_by_id`，不占状态位。

约束与索引：`pk_config_packages`、`ux_config_packages_package_no`、`ux_config_packages_content_hash`、`ix_config_packages_status_created_at`、`ck_config_packages_status`、`ck_config_packages_source`（`source = 'IN_PLACE'`）、`ck_config_packages_item_count`（`item_count between 1 and 2000`）、`fk_config_packages_superseded_by`（同表自引用，ON DELETE RESTRICT）。无行级策略。

##### 3.2.11 platform_meta.config_package_items（部署级）
本表由阶段 3b 按裁定 A-27 随最小发布通道建立，本节列定义即其落地口径，`item_hash` 算法与第 4.7 节一致；本阶段只做列扩展，不重复建表。

列：id、security_level、config_package_id uuid、item_kind text、item_code text、change_kind text（ADD、MODIFY、REMOVE）、applies_to_legal_entity_ids uuid[]（空数组表示全部法人）、before_spec jsonb、after_spec jsonb、item_hash text、sort_no int、公共列。

item_kind 取值封闭为 15 项：CUSTOM_OBJECT、CUSTOM_FIELD、CUSTOM_RELATION、CUSTOM_INDEX、CUSTOM_VIEW、UI_LAYOUT、FLOW_DEFINITION、AUTHZ_ROLE、AUTHZ_POLICY、AUTHZ_FIELD_GRANT、REPORT_DEFINITION、METRIC_DEFINITION、DASHBOARD_DEFINITION、PRINT_TEMPLATE、NOTIFY_RULE。其中 FLOW_、AUTHZ_、REPORT_、METRIC_、DASHBOARD_、PRINT_、NOTIFY_ 七类的定义对象表由流程、权限、报表、通知各自阶段拥有，本表只保存其序列化快照与哈希，落地由第 4.6 节的 `ConfigItemApplier` 端口交回各阶段实现。本阶段不定义那些表；PRINT_TEMPLATE 内容项的打印排版按裁定 A-08 只产出 `ep_foundation::port::doc::PrintLayout` 取值，本阶段不自建渲染路径，也不新增 trait。

约束与索引：`pk_config_package_items`、`fk_config_package_items_config_packages`、`ux_config_package_items_pkg_kind_code`、`ix_config_package_items_config_package_id_created_at`、`ck_config_package_items_item_kind`、`ck_config_package_items_change_kind`、`ck_config_package_items_specs`（ADD 时 `before_spec` 为空且 `after_spec` 非空，REMOVE 时相反，MODIFY 时两者均非空）。无行级策略。

##### 3.2.12 platform_meta.config_release_orders（部署级）
本表由阶段 3b 按裁定 A-27 随最小发布通道建立，本节列定义即其落地口径；本阶段只做列扩展与状态扩展，不重复建表。

列：id、security_level、order_no text、config_package_id uuid、action text（RELEASE、ROLLBACK）、rollback_to_package_id uuid、execution_mode text（ONLINE、MAINTENANCE_WINDOW）、submitted_by uuid、approved_by uuid、approval_ref text、reauth_ref text、scheduled_window_start timestamptz、status text（SUBMITTED、APPROVED、REJECTED、QUEUED、EXECUTING、SUCCEEDED、FAILED、COMPENSATED、CANCELLED）、started_at、finished_at、elapsed_ms int、failure_reason text、公共列。

约束与索引：`pk_config_release_orders`、`ux_config_release_orders_order_no`、`fk_config_release_orders_config_packages`、`ix_config_release_orders_status_created_at`、`ck_config_release_orders_action`、`ck_config_release_orders_status`、`ck_config_release_orders_self_approval`（`approved_by is null or approved_by <> submitted_by`，落实规格第 12.2 章申请人不可自审）、`ck_config_release_orders_rollback`（action 为 ROLLBACK 时 `rollback_to_package_id` 非空）。无行级策略。

##### 3.2.13 至 3.2.16 四张表的删除

按架构审计 ARCH-03 第一档第 3 条与第二档第 4 条，原 `config_release_steps`、`config_autotest_runs`、`config_edit_locks` 与 `config_release_mutex` 四张表整体删除，不留占位，本节只登记其去向，节号留号不补。发布的逐步执行记录折叠进 `platform_audit` 的审计事件，按发布单标识与内容项标识检索，不另建表；自动测试编排连同 8 个 suite 一并删除，其中 RLS_MATRIX 与 SOD_CHECK 两项的判据改由第 8.3 节的集成用例与 `tests/rls_matrix` 在发布前承担，其余六项本就与首版无消费方；并发编辑由内容项行上的 `row_version` 乐观锁承担，第二名编辑者提交时返回 `PLATFORM.CONCURRENCY.STALE_VERSION`，不再有编辑锁表与其过期巡检任务，基线第 3.6 节因此不需要追加第三类可物理删除的表；发布执行的串行化改由第 6.2 节的两项判据承担，不再有单行互斥表与跨段长事务。

##### 3.2.17 platform_meta.brand_profiles（部署级）

对应 PRD 附录乙 U-K-07 的临时取值，把白标可配置项固定为下表列集。

列：id、security_level、code、product_name text、vendor_display_name text、app_identifier_win text、app_identifier_mac text、app_identifier_ios text、app_identifier_android text、logo_attachment_object_id uuid、splash_attachment_object_id uuid、login_background_attachment_object_id uuid、theme_primary_color text、theme_accent_color text、notify_template_set_code text、signing_identity_ref text、distribution_channel text（APP_STORE、ENTERPRISE_MDM）、store_policy_check_passed_at timestamptz、status text（DRAFT、ACTIVE、SUPERSEDED）、公共列。

约束与索引：`pk_brand_profiles`、`ux_brand_profiles_code`、`ix_brand_profiles_status_created_at`、`ck_brand_profiles_distribution_channel`、`ck_brand_profiles_color`（`theme_primary_color ~ '^#[0-9A-Fa-f]{6}$'`，accent 同）、`ck_brand_profiles_status`。`signing_identity_ref` 只存 `secret://` 引用，不存密钥材料，照抄基线第 7.2 节。无行级策略。

##### 3.2.18 platform_meta.client_releases（部署级）

列：id、security_level、client text（win、mac、ios、android）、version text、build_no bigint、brand_profile_id uuid、artifact_hash text、artifact_size_bytes bigint、min_supported_version text、is_forced_security_update boolean、rollout_percent smallint、rollout_legal_entity_ids uuid[]、rollout_department_ids uuid[]、released_at timestamptz、withdrawn_at timestamptz、status text（DRAFT、ROLLING_OUT、FULL、WITHDRAWN）、release_notes text、公共列。

约束与索引：`pk_client_releases`、`ux_client_releases_client_version`、`fk_client_releases_brand_profiles`、`ix_client_releases_status_created_at`、`ck_client_releases_client`、`ck_client_releases_rollout_percent`（`between 0 and 100`）、`ck_client_releases_status`、`ck_client_releases_notes_len`（`char_length(release_notes) <= 2000`）。无行级策略。

##### 3.2.19 platform_meta.extensions（部署级）

桌面端签名原生插件的登记表，落实规格第 9.3 章“签名、版本锁定、能力声明、最小权限授予和审计要求对以下形态一致”。按架构审计 N-01，服务端 WASM 形态删除后本表只承载 DESKTOP_NATIVE 一种形态，表本身保留，理由是打印机与 USB Key 两个插件的签名主体、版本、哈希与能力清单仍需登记。

列：id、security_level、code、name、kind text（固定 DESKTOP_NATIVE）、version text、publisher_subject text、artifact_hash text、artifact_size_bytes bigint、signature bytea、signature_verified_at timestamptz、capability_manifest jsonb、resource_limits jsonb、target_platforms text[]（取 win、mac 的子集）、status text（REGISTERED、PENDING_APPROVAL、APPROVED、ENABLED、DISABLED、REVOKED）、disabled_reason text、consecutive_failures int、approval_ref text、公共列。

约束与索引：`pk_extensions`、`ux_extensions_code_version`、`ix_extensions_status_created_at`、`ck_extensions_kind`、`ck_extensions_status`、`ck_extensions_artifact_hash`（`artifact_hash ~ '^[0-9a-f]{64}$'`）、`ck_extensions_consecutive_failures`（`>= 0`）。无行级策略。

##### 3.2.20 platform_meta.extension_capability_grants（部署级）

列：id、security_level、extension_id uuid、capability text、object_type text、field_codes text[]、granted_by uuid、approval_ref text、granted_at timestamptz、revoked_at timestamptz、公共列。

capability 取值封闭为 3 项，是首版扩展能力集的全部：READ_OBJECT_FIELDS（读取由调用方按本授予裁剪后传入的字段，承载第 4.9 节第 4 条的最小必要数据裁剪，逐项核对后保留）、DEVICE_PRINTER（桌面端打印机）、DEVICE_SMARTCARD（桌面端 USB Key 与智能卡）。原 COMPUTE_ONLY 一项随服务端 WASM 形态一并删除，首版无其他消费方。网络、文件、密钥、数据库四类能力在此不存在可表达的取值，落实规格第 9.3 章“插件默认没有网络、文件、密钥或业务数据权限”。

约束与索引：`pk_extension_capability_grants`、`fk_extension_capability_grants_extensions`、`ux_extension_capability_grants_ext_cap_object`、`ix_extension_capability_grants_extension_id_created_at`、`ck_extension_capability_grants_capability`、`ck_extension_capability_grants_device_kind`（capability 为 DEVICE_ 开头时对应扩展的 kind 必须为 DESKTOP_NATIVE，由应用层同事务校验并在此以 CHECK 无法表达的部分交由用例断言）。无行级策略。

##### 3.2.21 platform_meta.extension_invocations（法人级，仅追加）

按架构审计 N-01，本表由服务端插件调用流水收窄为桌面端原生插件的加载与异常记录，由客户端随设备健康状态上报后写入。

列：id、legal_entity_id uuid、security_level、data_scope_tags text[]、extension_id uuid、caller_user_id uuid、caller_device_id uuid、entry_point text、duration_ms int、outcome text、error_text text、occurred_at timestamptz、created_at、created_by、reverses_id uuid。原 caller_process、input_hash、output_hash、fuel_consumed 与 memory_peak_bytes 五列随服务端 WASM 形态一并删除。

outcome 取值：OK、TIMEOUT、CRASHED、CAPABILITY_DENIED、SIGNATURE_MISMATCH、LOAD_REFUSED。

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

迁移文件路径 `db/migrations/platform_meta/`，迁移历史落在全局唯一的 `platform_core.schema_history`。执行顺序由单一全局 Runner 按文件版本号全序排定，不存在任何模块顺序声明文件。下列时间戳为占位取值，开工时按实际时间重取，相对顺序不得改变。

| 序 | 文件名 | 内容 |
|---|---|---|
| 1 | V202704060900__platform_meta_custom_object_model.sql | custom_objects、custom_fields、custom_relations、custom_indexes、custom_views 五表 |
| 2 | V202704060905__platform_meta_ddl_plan.sql | ddl_plans、ddl_plan_steps |
| 3 | V202704060910__platform_meta_ui_layouts.sql | ui_layouts |
| 4 | V202704060915__platform_meta_client_capability_values.sql | client_capability_values 建表 |
| 5 | V202704060920__platform_meta_backfill_capability_matrix.sql | 72 行种子数据，逐格照抄规格第 6.2 章 |
| 6 | V202704060940__platform_meta_brand_profiles.sql | brand_profiles |
| 7 | V202704060945__platform_meta_client_releases.sql | client_releases |
| 8 | V202704060950__platform_meta_extensions.sql | extensions、extension_capability_grants |
| 9 | V202704060955__platform_meta_extension_invocations.sql | extension_invocations 含 RLS |
| 10 | V202704061000__platform_meta_client_bootstrap_dispatches.sql | client_bootstrap_dispatches 含 RLS |

每个文件头部带 `-- rollback:` 段。本阶段不再追加任何 `ext` schema 级授权迁移：`ext` schema 本身、其属主角色以及对 `ep_app_rw` 与 `ep_analyst_ro` 的 USAGE 与表默认权限由阶段 2 按裁定 C-01 随二十四个 schema 一次建立，`ep_migrator` 在阶段 2 已被授予全部 `ep_mod_*` 角色成员资格，因而在 `ext` 下具备 DDL 权限；自定义对象表的数据读写授权由第 4.3 节的 DDL 计划在 `create policy` 之后逐表发出，不经默认权限，理由见第 4.3 节边界条件。

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

// 声明式规则的表达式类型与求值器按架构审计 ARCH-03 第二档第 5 条合并到阶段 3b 的流程守卫求值器，
// 本阶段不再定义 RuleExpr，也不再实现 AstRuleEvaluator，见第 4.5 节

// ep-platform-release::model
pub enum PackageStatus { Draft, PendingApproval, Rejected, Approved, Released, RolledBack }
pub enum ReleaseOrderStatus { Submitted, Approved, Rejected, Queued, Executing,
    Succeeded, Failed, Compensated, Cancelled }
pub enum ItemKind { /* 15 项，见 3.2.11；本枚举与 ConfigPackageItem 由阶段 3a 在 crates/platform/release/src/port/config_item.rs 交付，见裁定 A-19 */ }
pub enum ChangeKind { Add, Modify, Remove }
```

#### 4.2 配置包状态机

状态与流转按阶段 3b 最小发布通道的六态落地，本阶段不扩展，守卫条件如下。PRD 第 10.4.1 节十一态中其余五态的删除与折叠见第 3.1 节。

| 起点 | 终点 | 触发 | 守卫条件 |
|---|---|---|---|
| Draft | PendingApproval | 提交发布单 | 包内容项数在 1 至 2000 之间；包体积不超过 64 MiB；每项 `item_hash` 与其 `after_spec` 的 SHA-256 一致；包已锁定不可再改；`min_platform_version` 不高于当前版本 |
| PendingApproval | Approved | 审批通过 | 审批人不等于提交人；审批人具备配置发布审批权限项；审批链无越权跳过 |
| PendingApproval | Rejected | 审批驳回 | 同上审批人条件 |
| Rejected | Draft | 配置管理员修改 | 该包全部内容项行的 `row_version` 未被他人推进 |
| Approved | Released | 执行发布单 | 执行模式与 `ddl_plans.execution_mode` 一致；若为 MAINTENANCE_WINDOW 则必须落在已登记的停机窗口内；不存在其他 status 为 EXECUTING 的发布单 |
| Released | RolledBack | 回退发布单 | 回退目标为上一 Released 包；该目标在保留窗口内（最近 10 个且不早于 180 天）；回退发布单本身已完成审批 |

上一已发布包在新包置 Released 的同一事务内写 `superseded_by_id`，不进入状态流转。非法迁移一律返回 `BUSINESS_CONFLICT` 与 `PLATFORM.CONFIG_PACKAGE.*` 的对应码，不静默忽略。

#### 4.3 在线 DDL 计划生成与执行算法

输入为配置包中全部 CUSTOM_ 前缀内容项的差集，输出为一份 `ddl_plans` 与其有序 `ddl_plan_steps`。

步骤如下。

1. 归一化。把 ADD、MODIFY、REMOVE 三类内容项按目标对象聚合，得到每个对象的目标结构；与 `platform_meta` 中该对象 ACTIVE 版本比对，得到列级与索引级差异。
2. 基线校验。字段类型必须落在规格第 7.4 章的 11 种之内；索引类型必须落在 3 种之内；JSON 列不得建索引也不得设 CHECK 校验；对象级密级与字段级密级必须有值，字段级为空时按对象级取值，两者都为空即拒绝。任一项不通过返回 `VALIDATION`，计划不生成。
3. 语句映射与执行模式判定。

| 差异 | 生成语句 | 执行模式 |
|---|---|---|
| 新增对象 | create table、enable rls、force rls、create policy、逐表 grant、三条基线索引 | ONLINE |
| 新增可空列 | alter table add column，无默认或常量非易失默认 | ONLINE |
| 新增索引 | create index concurrently | ONLINE |
| 放宽长度 | drop constraint 旧 CHECK，add constraint 新 CHECK not valid，validate constraint | ONLINE |
| 新增多对多关系 | create table link，rls 五件套，唯一索引 | ONLINE |
| 收紧长度、改列类型、收紧非空、重建主键、删除列、删除表 | 对应 DDL | MAINTENANCE_WINDOW |

计划整体的执行模式取其中最严者。含 MAINTENANCE_WINDOW 语句的计划在未登记停机窗口时返回 `PLATFORM.DDL_PLAN.REQUIRES_MAINTENANCE_WINDOW`。

4. 五项影响分析。索引项给出新增索引数、该对象索引总数与配额比对、按现有行数与平均行宽估算的索引体积；容量项给出新增列的行宽增量、`ext` 下对象总数与字段总数与配额比对、磁盘剩余量；性能项给出每条 `create index concurrently` 按现有行数与认证期实测吞吐外推的预计耗时与 30 分钟上限比对；安全项给出密级赋值核对结论、RLS 模板齐备结论、新增查询入口是否已纳入 RLS 矩阵测试的结论；迁移项给出可逆性判定与回退方式，不可逆的注明只能用升级前备份或影子表。五项写入 `ddl_plans` 的五个 jsonb 列，缺一不可。
5. 执行。DDL 段的第一步由 job-worker 的 DDL 执行器在把控制交给 `ep-platform-release` 的编排之前，调用经装配注入的 `ep_adapter_db::port::MigrationWindowGuard` 实例的 `assert_open(tx)`，该端口与其唯一实现 `PgMigrationWindowGuard` 均由阶段 2 交付并已在两个 wiring 注入，`ep-platform-release` 不引用该 trait，见裁定 B-03；未持有已打开的迁移窗口时不建立任何连接、不执行任何语句，返回 `PLATFORM.DB.MIGRATION_WINDOW_CLOSED`，HTTP 409，category 为 BUSINESS_CONFLICT，该错误码由阶段 1 登记，本阶段只引用。守卫通过后由 job-worker 建立一条 `ep_migrator` 连接，会话上执行 `set lock_timeout = '5s'` 与 `set statement_timeout = '30min'`。逐条语句在自动提交下执行，理由是 `create index concurrently` 不能在事务块内。每条语句执行前后各取一次 `clock_timestamp()`，把等待锁的时长与执行时长写入 `ddl_plan_steps`。
6. 失败与回退。任一语句失败时立即停止，按已成功语句的逆序执行补偿语句：`create index concurrently` 对应 `drop index concurrently`，`add column` 对应 `drop column`，`create table` 对应 `drop table`，`create policy` 对应 `drop policy`，`validate constraint` 与 `add constraint` 对应 `drop constraint` 并恢复原 CHECK。补偿完成后计划置 ROLLED_BACK；若失败原因为 lock_timeout，计划另置 DEFERRED_TO_WINDOW 并把回退原因、操作对象与耗时写入审计，照抄规格第 7.4 章运行期口径，不判定为认证失败。
7. 元数据与 DDL 的一致化。DDL 无法与元数据写入同事务，因此采用两阶段：先在一个事务内把相关 `custom_objects` 与 `custom_fields` 置 PENDING_DDL 并写审计；执行 DDL；成功后在一个事务内置 ACTIVE、递增 `definition_version`、写审计与 Outbox 事件；失败后在一个事务内置 DDL_FAILED 并写审计。第 7 节按裁定 C-25 追加的自检项 `custom-object-ddl-consistent` 检出 ACTIVE 元数据而物理表缺失、或物理表存在而未开启行级安全的组合，按架构审计 ARCH-02 该项为 Degrading 级：检出即把相关 `custom_objects` 置 DDL_FAILED 并隔离其全部入口，登记降级窗口并持续告警，进程照常启动，不以一次业务数据判读阻断七个进程。

边界条件：单次计划的语句数上限 200；同一时刻只允许一份 EXECUTING 的计划，由第 6.2 节的两项判据保证，即发布单行锁加唯一性断言与迁移窗口本身的互斥；`ext` 表在 RLS 策略创建成功之前不对任何应用账号开放，`grant` 语句排在 `create policy` 之后。

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

1. 常量由各业务阶段按裁定 A-20 在 `crates/contract/<module>/src/capability.rs` 中以 `<USECASE_SCREAMING>_DOMAIN` 与 `<USECASE_SCREAMING>_ACTION` 成对声明，本阶段只做运行期判定，不代其他阶段声明。本阶段自身的路由分 `/api/v1/platform/` 与 `/api/v1/ext/` 两段，两段同属裁定 A-20 的第二类落点即平台路由，不构成第三类，按 A-20 为每个用例成对声明常量，能力域一律取 `CapabilityDomain::PlatformAdminLowcodeOps`；落点按承载该路由处理器的 crate 归位，第 5.1、5.2、5.4、5.5 各段与 `/api/v1/ext/` 的五个路由形状落 `crates/platform/meta/src/capability.rs`，第 5.3 节配置包与发布单两段的常量随该段一并归阶段 3b，落 `crates/platform/release/src/capability.rs`，本阶段不声明。由 `xtask configdoc` 断言每个 `/api/v1/` 路由都能解析到一对常量，缺失即构建失败。同一用例同时落入两个能力域时按取值较低的所在行判定，照抄规格第 6.2 章。
2. core-server 的能力闸中间件在授权判定之前执行，读取请求头 `X-Client`，从 `platform_meta.client_capability_values` 取该能力域该端的取值。`portal` 与 `ops` 两个取值不参与本判定，门户不纳入四端等价，运维端只访问 `ops-agent` 暴露的端点。
3. 判定结果：
   - Full：放行。
   - Simplified：放行，但批量端点的单次上限按端下调，移动端 50 条，桌面端 200 条；超出返回 `VALIDATION`。业务对象、权限模型与流程结果不变，照抄规格第 6.2 章取值含义。
   - ViewOnly 且 action_class 不为 Read：拒绝，403 与 `PLATFORM.CLIENT_CAPABILITY.WRITE_NOT_AVAILABLE_ON_CLIENT`，`error.advice` 为该操作在桌面端完成的说明，`error.details` 携带 `alternative_path`，响应体 `data` 为空但响应头带 `X-Desktop-Handoff-Token`。
   - NotApplicable：返回 404 与 `PLATFORM.AUTHZ.NOT_FOUND_OR_DENIED`，不区分不存在与不承载，照抄基线第 5.5 节的存在性泄漏统一处理。
4. 桌面接续令牌是 U-K-08 的临时取值：一次性令牌，有效期 5 分钟，绑定用户、法人、目标能力域与目标单据标识，桌面端登录同一用户后凭该令牌拉起同一单据同一草稿。移动端界面同时提供“发送到桌面端继续”的入口，入口本身不构成写入。
5. 矩阵冻结比对：`client_capability_values` 全表按 `(capability_domain, client, value, exemption_ref)` 排序后计算 SHA-256，与二进制内置的冻结快照哈希比对。按架构审计 ARCH-02，二进制内置的冻结快照本身就是权威：比对不一致时以内置快照为运行期判据继续运行，同时拒绝一切对该表的写入，登记降级窗口并持续告警，不再以退出码 78 阻断启动。规格第 6.2 章“本清单随本规格冻结”由内置快照承载，而不是由启动时的一次数据库判读承载。
6. 客户端侧同样按引导数据中的矩阵取值渲染入口，ViewOnly 的能力域不渲染提交、审批与写入入口。客户端隐藏不构成访问控制，服务端闸是唯一权威，照抄 PRD 第 10.4.3 节。

#### 4.5 声明式规则与移动端离线草稿

1. 规则以 AST 形式存储与下发，无任何代码下发。按架构审计 ARCH-03 第二档第 5 条，全系统只保留一个表达式求值器：阶段 3b 的流程守卫最小求值器提升为唯一求值器，同时服务流程守卫、本阶段的声明式规则与 PRD 第 8.4.2 节的自定义指标表达式。本阶段不再定义第二套 AST，也不再实现 `AstRuleEvaluator`；AST 节点数上限、求值深度上限与其错误码随该求值器一并归阶段 3b。这是合并不是新增，净减两条计算路径。
2. 数值一律 `foundation::Money`、`UnitPrice`、`Quantity`、`Rate` 四类，中间值以 Decimal 全精度保留，只在产出最终判定值时按基线第 3.5 节 round，舍入策略 `MidpointAwayFromZero`。
3. 规则一律在服务端求值。`executable_on_client` 字段保留位点，取值依据改为“该规则是否只引用随引导下发的字段”，首版不存在引用仅服务端可得数据的规则时恒为真；客户端不再内置解释器，见第 1.2 节第 7 项。
4. 断网时四端一律不求值，单据保存为本地草稿并置 `pending_central_validation`，不产生正式业务记录也不产生正式会计分录，依据是第 4.10 节第 5 条的离线断网场景，与已删除的 WASM 形态无关。恢复连接后按该业务模块的正常提交端点提交，中心执行全部规则并把“该单据曾以待中心校验草稿提交”写入审计。
5. 按架构审计 N-01，服务端 WASM 一线全删，不留占位：`RuleExpr::WasmCall` 变体、`requires_wasm` 字段、`ep_platform_flow::port::WasmComputePort` 与其实现类型 `PluginHostWasmCompute`、阶段 3b 注入的 `NoopWasmComputePort` 及其 TODO 注释、`tools/epplug` 与示例插件一并删除。裁定 B-05 中关于 `WasmComputePort` 的部分随之作废，关于 `RuleEvaluator` 的部分按本节第 1 条改为唯一求值器归阶段 3b，`NoopRuleEvaluator` 一并删除。这两处按 Noop 空实现新硬规则的第二条出路处理，即整条推迟而不是留空实现：超出该求值器表达能力的计算按厂商侧领域代码交付，与阶段 10 的 `TaxLine::validate` 同一路径；恢复条件写入规格第 5.7 章，出现客户实际提出、求值器确不能表达且客户愿意付费的计算时与扩展市场同批评估，重建时不预先绑定 wasmtime。
6. 本阶段不再提供独立预校验端点，`POST /api/v1/platform/rule-evaluations/actions/evaluate` 随 `AstRuleEvaluator` 一并删除，联网态的预校验由各业务模块提交端点的校验分支承担。

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

本阶段在 `ep-platform-meta` 中实现 CUSTOM_OBJECT、CUSTOM_FIELD、CUSTOM_RELATION、CUSTOM_INDEX、CUSTOM_VIEW、UI_LAYOUT 六个 applier。其余九个 applier 的归属按裁定 A-19 写死：FLOW_DEFINITION 的 `FlowDefinitionApplier` 与 NOTIFY_RULE 的 `NotifyRuleApplier` 由阶段 3b 在 `ep-platform-flow` 与 `ep-platform-notify` 实现；AUTHZ_ROLE 的 `AuthzRoleApplier`、AUTHZ_POLICY 的 `AuthzPolicyApplier`、AUTHZ_FIELD_GRANT 的 `AuthzFieldGrantApplier` 由阶段 4 在 `ep-platform-authz` 实现；REPORT_DEFINITION 的 `ReportDefinitionApplier`、METRIC_DEFINITION 的 `MetricDefinitionApplier`、DASHBOARD_DEFINITION 的 `DashboardDefinitionApplier`、PRINT_TEMPLATE 的 `PrintTemplateApplier` 由阶段 11 在 `ep-app-reporting` 实现。全部实现在 `apps/core-server/src/wiring.rs` 与 `apps/job-worker/src/wiring.rs` 注册到阶段 3a 提供的 `ConfigItemApplierRegistry`。本阶段不定义端口，也不定义那些阶段的表与接口。九个 applier 按架构审计 ARCH-03 第二档第 4 条全部保留，理由是阶段 4 的权限配置生效路径、A-28 的字段级授权写入与 PRD 第 10.4.1 节的变更控制都只有这一条落点。

段二成功后在同一事务内写入：`config_release_orders` 置 SUCCEEDED、`config_packages` 置 RELEASED、上一 RELEASED 包写入 `superseded_by_id`、审计事件与逐步执行记录、Outbox 事件 `platform.config_release.released.v1`。该事件随最小发布通道归阶段 3b 登记，不计入本阶段的事件类型条数。

段三，传播段。由 job-worker 消费 Outbox 事件执行，包含：任一 applier 的 `requires_derived_store_rebuild` 为真时按法人逐个重建内置搜索索引分区，重建经阶段 3b 按裁定 A-07 交付的 `ep-adapter-search` 执行，本阶段只按 `ep_foundation::port::search::SearchDocument` 产出投影并经 `SearchIndexPort` 写入，不自建第二条写入路径，重建期间该分区停止对外服务，重建后重放待处理的删除与更正事件并与来源做条数一致性校验与哈希抽样对账，照抄规格第 7.9 章；客户端引导数据版本号递增，使在线客户端在下一次引导时拉到新配置；站内通知按 PRD 第 10.5.2 节送达配置管理员。

失败与补偿：

- 段一失败，段二不执行，段一按第 4.3 节逆序补偿，发布单置 FAILED。
- 段二失败，事务回滚，段一按逆序补偿，发布单置 FAILED。
- 段三失败，发布已生效，进入 Outbox 重试与死信，按基线第 6.2 节的 8 次退避；连续失败进入死信并按 PRD 第 10.5.2 节通知责任人；权限、密级或分区规则变更的传播未完成前，受影响范围的检索与报表入口不可用，照抄规格第 7.9 章与 PRD 第 10.4.5 节。

回退算法：

- 回退发布单以上一 RELEASED 包为目标，按 `sort_no` 逆序对当前包的内容项调用 `revert`，使用 `before_spec` 恢复。
- 数据定制的回退取值（U-K-02 的临时取值）：新增字段与新增对象的回退只把元数据置 RETIRED，界面与 API 不再暴露该字段与该对象，物理列与物理表与其中数据一律保留，不执行 DROP。理由是 U-K-02 未决，而规格第 7.2 章与第 7.5 章要求业务数据只追加不覆盖，回退不得删掉已录入的业务数据。物理删除只能由单独的停机窗口计划发起，经双人审批，并按裁定 A-22 经 `ep_platform_file::port::disposal::DisposalPort` 交由阶段 14 的 `OpsDisposalService` 执行，走规格第 12.4 章的处置流程与处置清单；本阶段不实现任何物理删除路径。
- 回退同样触发受影响派生存储的重新打标，照抄 PRD 第 10.4.5 节。
- 可回退版本数与时间窗（U-K-02 的临时取值）：保留最近 10 个 RELEASED 包，且发布时间不早于 180 天；超出范围的包不可作为回退目标，尝试回退到该包返回 `PLATFORM.CONFIG_RELEASE_ORDER.ROLLBACK_TARGET_EXPIRED`。

#### 4.7 配置包内容哈希与差异算法

1. 打包：`manifest.toml` 含包码、名称、版本、`min_platform_version`、内容项清单，每项含 `item_kind`、`item_code`、`change_kind`、`sort_no`、`item_hash`。`item_hash` 为该项 `after_spec` 的 JSON 规范化序列化（键按字典序、无空白、UTF-8）后的 SHA-256 十六进制小写。
2. `content_hash` 为 `manifest.toml` 字节流的 SHA-256，用途只有就地式包的去重与差异审查，不用于来源认证。
3. 按架构审计 ARCH-03 第一档第 3 条，配置包的 ECDSA 签名与验签、`tools/epcfg` 与导入式包三项整体删除。删除理由是首版一个部署内只有就地式包一种来源，签名的唯一用途是认证跨部署传递的外来包，而首版不存在这类包，签名与信任主体清单因此没有消费方。规格第 9.2 章“经过差异审查、自动测试、审批和签名”中的自动测试与签名两句随之失去落点，按规格第 2.2 章登记进第 5.7 章延期目录并需产品负责人批准；差异审查由 `GET /api/v1/platform/config-packages/{id}/diff` 承担，审批由第 4.2 节的六态状态机与 `ck_config_release_orders_self_approval` 承担，全过程记名记时写入审计，这三项不降级。
4. 差异算法：两包内容项按 `(item_kind, item_code)` 对齐，逐项比对规范化后的 `after_spec`，输出新增、修改与删除三类，每项给出 before 与 after 的规范化 JSON；同一内容项在两包中完全一致时不进入差异。

#### 4.8 服务端 WASM 插件沙箱算法（本节整体删除）

按架构审计 N-01 与 ARCH-03 第一档第 1 条，本节整节删除，节号留号不补，不留任何形式的占位。删除清单逐项如下：plugin-host 进程与其系统账户、cgroup 分片与 IPC 通道；crate `ep-adapter-wasm` 与实现类型 `PluginHostWasmCompute`；workspace 根 `[workspace.dependencies]` 中 wasmtime 与 wasmtime-wasi 两行与编译缓存目录约定；`tools/epplug` 与其示例插件即税额校验计算，该功能已由阶段 10 的 `TaxLine::validate` 承担并被声明为唯一的税额校验入口，属消费者位置已被占；`EP__PLUGIN__` 前缀的七个配置项；`PLATFORM.EXTENSION.RESOURCE_LIMIT_EXCEEDED` 与 `PLATFORM.EXTENSION.HOST_UNAVAILABLE` 两条错误码；插件调用相关的四个指标即调用次数、燃料消耗、内存峰值与限流次数。

明令禁止的三种折中：保留空壳进程占位、保留 crate 骨架、保留 feature 默认关闭的 wasmtime 依赖。三者的维护负担照旧，版本升级、安全公告响应、供应链门禁与 SBOM 均照跑，而收益为零。

桌面端原生插件的加载、能力核对、隔离与停用另在第 4.9 节独立规定，不受本节删除影响；`PLATFORM.EXTENSION.CAPABILITY_DENIED`、`PLATFORM.EXTENSION.DISABLED`、`PLATFORM.EXTENSION.SIGNATURE_INVALID` 与 `PLATFORM.EXTENSION.MANIFEST_MISMATCH` 四条错误码由第 4.9 节继续使用。

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

全部端点遵守基线第 5 章：路径前缀 `/api/v1`，JSON 字段 snake_case，成功与失败封套按基线第 5.2 节，写请求必带 `Idempotency-Key`，请求头集合按基线第 5.6 节。平台侧路径段取 `platform`，该取值已由阶段 2 的九个平台路由按裁定 A-20 落定，本阶段沿用不再另议。自定义对象的通用数据端点路径段取 `ext`，与 schema 名一致，不新增模块码。

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

本节各端点归阶段 3b 的最小发布通道，本节的路径、请求与错误码即其落地口径，本阶段不新增也不扩展任何端点，只作为第 4.6 节发布执行的调用面引用。原 `actions/import`、`actions/run-autotest` 与 `actions/sign` 三个端点按第 4.7 节第 3 条随导入式包、自动测试与签名一并删除。

| 方法与路径 | 请求 | 响应 data | 主要错误码 | 幂等语义 | 权限项 |
|---|---|---|---|---|---|
| POST /api/v1/platform/config-packages | `{name, source:"IN_PLACE", items:[...]}` | 包详情，status 为 DRAFT | PLATFORM.CONFIG_PACKAGE.ITEM_LIMIT_EXCEEDED | 幂等键 | lowcode.config_package.create |
| GET /api/v1/platform/config-packages/{id} | 无 | 包详情含内容项摘要 | 404 | 只读 | lowcode.config_package.view |
| GET /api/v1/platform/config-packages/{id}/diff | `?against={package_id}` | `{added:[...], modified:[...], removed:[...]}`，每项含 before 与 after 的规范化 JSON | 404 | 只读 | lowcode.config_package.view |
| POST /api/v1/platform/config-packages/{id}/actions/submit-for-approval | `{note}` | 包详情 | BUSINESS_CONFLICT 若状态不为 DRAFT | 幂等键 | lowcode.config_package.submit |
| POST /api/v1/platform/config-packages/{id}/actions/approve | `{note}` | 包详情 | SELF_APPROVAL_FORBIDDEN | 幂等键 | lowcode.config_package.approve |
| POST /api/v1/platform/config-packages/{id}/actions/reject | `{reason}` | 包详情 | 同上 | 幂等键 | lowcode.config_package.approve |
| POST /api/v1/platform/config-release-orders | `{config_package_id, action, rollback_to_package_id, execution_mode, scheduled_window_start}` | 发布单详情 | ROLLBACK_TARGET_EXPIRED、REQUIRES_MAINTENANCE_WINDOW | 幂等键 | lowcode.config_release.submit |
| POST /api/v1/platform/config-release-orders/{id}/actions/execute | 无 | `{task_receipt_id}`，转后台任务 | CONCURRENT_RELEASE_IN_PROGRESS、DERIVED_STORE_REBUILD_REQUIRED、PLATFORM.DB.MIGRATION_WINDOW_CLOSED | 幂等键，重复执行返回同一回执 | lowcode.config_release.execute |
| POST /api/v1/platform/config-release-orders/{id}/actions/cancel | `{reason}` | 发布单详情 | BUSINESS_CONFLICT 若已进入 EXECUTING | 幂等键 | lowcode.config_release.execute |
| GET /api/v1/platform/config-release-orders/{id} | 无 | 发布单详情含由审计事件检索出的逐步执行记录 | 404 | 只读 | lowcode.config_release.view |

发布执行是长时操作，按基线第 11.6 节同步等待上限 8 秒，`actions/execute` 一律返回任务回执并转后台任务，完成后由站内通知送达。

#### 5.4 扩展登记

| 方法与路径 | 请求 | 响应 data | 主要错误码 | 幂等语义 | 权限项 |
|---|---|---|---|---|---|
| POST /api/v1/platform/extensions/actions/register | `{attachment_object_id, code, version}` | 扩展详情含解析出的能力清单，kind 恒为 DESKTOP_NATIVE | EXTENSION.SIGNATURE_INVALID、EXTENSION.MANIFEST_MISMATCH | 幂等键，同一 `artifact_hash` 返回既有登记 | ext.extension.register |
| POST /api/v1/platform/extensions/{id}/actions/request-approval | `{requested_capabilities:[...]}` | 扩展详情 | BUSINESS_CONFLICT | 幂等键 | ext.extension.register |
| POST /api/v1/platform/extensions/{id}/actions/approve | `{granted_capabilities:[...], note}` | 扩展详情含授予清单 | SELF_APPROVAL_FORBIDDEN | 幂等键 | ext.extension.approve |
| POST /api/v1/platform/extensions/{id}/actions/enable | 无 | 扩展详情 | EXTENSION.SIGNATURE_INVALID | 幂等键 | ext.extension.enable |
| POST /api/v1/platform/extensions/{id}/actions/disable | `{reason}` | 扩展详情 | 无 | 幂等键 | ext.extension.enable |
| GET /api/v1/platform/extensions/{id}/invocations | 分页与时间范围过滤 | 桌面端插件的加载与异常记录，含 outcome 与耗时 | 404 | 只读 | ext.extension.view |

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
  "rules": [{"code":"…","version":5,"executable_on_client":true,"ast":{}}],
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
| 建模对象、字段、索引、视图、布局的增删改 | 一个用例一个事务 | READ COMMITTED | 只有行级乐观锁 `row_version`，不设编辑锁 |
| 配置包创建与内容项写入 | 一个事务 | READ COMMITTED | 无额外锁 |
| 发布单受理 | 一个事务 | READ COMMITTED | 无 |
| 发布执行段一（DDL） | 每条语句自动提交，不在事务内 | 会话级 `lock_timeout=5s`、`statement_timeout=30min` | `create index concurrently` 不取表级排他锁；`add column` 取 ACCESS EXCLUSIVE 但受 5 秒超时约束 |
| 发布执行段二（元数据与配置） | 一个事务 | READ COMMITTED | 先对该发布单行取 `select for update`，同一事务内断言不存在其他 EXECUTING 发布单 |
| 发布执行段三（传播） | 每个法人一个事务 | READ COMMITTED | 搜索索引分区重建期间该分区停止对外服务 |
| 扩展登记与能力授予 | 一个事务 | READ COMMITTED | 乐观锁 |
| 桌面端插件加载与异常记录写入 | 客户端上报后单独一个事务 | READ COMMITTED | 仅追加 |

#### 6.2 发布的串行化

串行化由两项判据承担，不再有单行互斥表。第一项是发布执行器在受理与段二两处各对该发布单行取 `select ... for update`，并在同一事务内断言不存在其他 status 为 EXECUTING 的发布单，不成立即返回 409 与 `PLATFORM.CONFIG_RELEASE_ORDER.CONCURRENT_RELEASE_IN_PROGRESS`。第二项是含 DDL 段的发布必须持有已打开的迁移窗口，而迁移窗口由阶段 2 的 `MigrationWindowGuard` 保证同时只有一个打开，因此两份含 DDL 的发布不可能并行进入段一。

按架构审计 ARCH-03 第一档第 3 条，原为串行化建立的 `config_release_mutex` 单行表、跨段持有互斥行的长事务与三条连接的编排一并删除。删除后连接由三条降为两条：段一的 `ep_migrator` 连接与段二的 `ep_app_rw` 连接，段一失败时先按第 4.3 节逆序补偿再由段二落库失败结论。理由是一台服务器、20 人并发、发布由一名配置管理员发起，用一张表加一个跨段长事务表达互斥，其收益低于它在异常路径上制造的连接与锁泄漏面。

#### 6.3 幂等键与 Outbox

- HTTP 层：全部写端点必带 `Idempotency-Key`，作用域为法人、用户、端点、键值四元组，存储在 `platform_msg.idempotency_keys`，与业务写入同事务，照抄基线第 5.4 节。部署级配置端点的法人取该请求头 `X-Legal-Entity-Id` 的取值，即同一动作在不同法人上下文下发起视为不同的幂等作用域；这是本阶段的取值，理由是部署级配置对象无法人列但请求仍带法人上下文，若不纳入四元组会使两名分属不同法人的配置管理员的重放互相干扰。
- Outbox：发布成功在段二事务内写入 `platform.config_release.released.v1`，`idempotency_key` 取发布单标识。段三消费端幂等由 `platform_msg.inbox_consumptions(consumer, event_id)` 唯一约束保证。
- 重试退避照抄基线第 6.2 节的 8 次；全部失败置 DEAD 并写死信，按 PRD 第 10.5.2 节通知责任人。
- 死信重投必须记名并写审计；丢弃需要双人审批。

#### 6.4 失败重试与补偿

- 数据库序列化失败 40001 与死锁 40P01 在数据访问层重试 3 次，退避 50、150、450 毫秒，只对尚未产生外部可见副作用的事务重试，照抄基线第 8.4 节。DDL 段不适用该重试，DDL 失败一律走补偿。
- 段一的补偿是逆序 DDL；段二的补偿是事务回滚；跨段失败的补偿是段二未提交加段一逆序。补偿部分失败时该发布单置 COMPENSATED 并进入人工任务队列并告警，不得静默结束，照抄规格第 9.1 章。
- 桌面端插件调用失败不触发服务端事务重试，由客户端按第 4.9 节第 5 条隔离该子进程；同一入口连续失败达阈值自动停用该插件，停用事件随设备健康状态上报中心。

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
| EP__RELEASE__ROLLBACK_KEEP_PACKAGES | u8 | 10 | 启动时生效 |
| EP__RELEASE__ROLLBACK_MAX_AGE_DAYS | u16 | 180 | 启动时生效 |
| EP__RELEASE__PACKAGE_MAX_BYTES | u64 | 67108864 | 启动时生效 |
| EP__RELEASE__PACKAGE_MAX_ITEMS | u16 | 2000 | 启动时生效 |
| EP__RELEASE__PAUSE_DURING_PERIOD_CLOSE | bool | true | 启动时生效 |
| EP__EXT__TRUSTED_SIGNER_SUBJECTS | list of string | 空 | 启动时生效，为空时不加载任何桌面端原生插件 |
| EP__EXT__AUTO_DISABLE_FAILURE_THRESHOLD | u8 | 3 | 启动时生效 |
| EP__BRAND__ACTIVE_PROFILE_CODE | string | "default" | 下次引导生效 |

敏感项按基线第 7.2 节只写 `secret://` 引用，内存中以 `secrecy::SecretString` 包装。

启动自检的追加项按裁定 C-25 改为命名项，基线第 7.3 节的十三项固定项一并按注册名标识，本阶段不再以序号称呼，需回写基线。按架构审计 ARCH-02，自检项分 Blocking 与 Degrading 两级，本阶段追加的两项一律为 Degrading 级：两项判读的都是数据库里的配置行与业务行，而这台服务器没有备节点，用一次数据判读阻断七个进程的启动是把可用性押在一次判读上。

- `client-capability-matrix-frozen`（Degrading）：`platform_meta.client_capability_values` 的内容哈希与二进制内置的冻结快照比对。不一致时以内置快照为运行期判据继续运行，同时拒绝一切对该表的写入，经 `ep_platform_obs::DegradationLedger` 的 `open` 与 `close` 登记降级窗口并持续告警，不阻断启动。
- `custom-object-ddl-consistent`（Degrading）：`ext` schema 下全部表均已 `ENABLE` 且 `FORCE` 行级安全，且各自存在 `rls_<table>_le` 策略；`platform_meta.custom_objects` 中 status 为 ACTIVE 的每个对象在 `ext` 下均有对应物理表，反之不存在孤立物理表。不一致时把相关 `custom_objects` 置 DDL_FAILED 并隔离其全部入口，登记降级窗口并持续告警，不阻断启动；孤立物理表不隔离任何入口，只告警。

裁定 C-25 为本阶段固定了上述两个项名，因此原有的另外两项校验不再作为启动自检项：status 为 ENABLED 的扩展其制品可读、哈希一致、签名可验、`capability_manifest` 未超出已授予能力，改由第 4.9 节的加载路径逐次校验，不通过即不加载并写审计；当前生效的品牌配置引用的附件对象存在且可读，改由 `POST /api/v1/platform/brand-profiles/{id}/actions/activate` 用例在激活前校验，不通过返回 `PLATFORM.BRAND_PROFILE.ASSET_INVALID`。

`--check` 模式按 `SelfCheckRegistry` 的注册顺序一并执行全部注册项，基线十三项在前，本阶段两项在后；`--check` 对 FAILED 与 DEGRADED 一律非零退出，因此部署与升级前置仍是严格闸门，进程启动不是。

---

### 8. 测试计划

#### 8.1 单元测试

覆盖分支逐项列出。

1. DDL 计划生成器：第 4.3 节第 3 步映射表六行差异各自的语句序列与执行模式判定；混合差异时整体执行模式取最严者；语句数超 200 时拒绝。
2. 基线校验器：11 种字段类型逐个通过、超出基线的类型逐个拒绝；3 种索引类型逐个通过、函数索引与局部索引与 JSON 路径索引的表达一律无法构造；JSON 列建索引拒绝；JSON 列设 CHECK 拒绝。
3. 密级校验：对象级为空拒绝；字段级为空时继承对象级；两者均为空拒绝。
4. 保留列名：公共列九项加第 3.2.2 节列出的九个专属列共十八个名字逐个拒绝。
5. 影响分析五项：各自在空差异、单表小差异、多表大差异三种输入下的输出结构完整性。
6. 配置包状态机：6 个状态、第 4.2 节表列出的 6 条合法迁移全部通过；任意两状态之间的非法迁移全部返回对应错误码；自审批拒绝。
7. 发布单状态机：9 个状态与其合法迁移；回退目标过期拒绝；回退目标非上一 RELEASED 包拒绝。
8. 内容哈希：键序与空白不同的两份等价 JSON 得到同一 `item_hash`；任一字节改动使 `item_hash` 改变；`min_platform_version` 高于当前版本拒绝提交。
9. 差异算法：新增、修改、删除三类内容项的 diff 输出；同一内容项在两包中完全一致时不进入差异。
10. 能力矩阵判定：四个取值乘五个动作类别共二十种组合的判定结果；同一用例落入两个能力域时取较低者；未知端取值按未知降级。
11. 能力矩阵冻结比对：表内容与内置快照一致时不开降级窗口；篡改一行后以内置快照判定、开降级窗口、对该表的写入被拒绝、进程不退出三项同时成立。
12. 桌面端插件能力裁剪：授予字段子集时传入子进程的报文只含子集；未授予任何 `READ_OBJECT_FIELDS` 时报文为空；声明超出授予的能力拒绝加载。
13. 桌面端插件停用判定：连续失败计数、阈值命中后置 DISABLED、成功一次归零三类。
14. 品牌资源校验：产品名超长、颜色格式错误、应用标识不合规、Logo 尺寸不符四类各自拒绝。
15. 编号生成：自定义单据对象的编号格式按基线第 11.1 节，法人加类型加年月三元组独立自增，流水溢出扩位；`doc_type_code` 与 `docs/data-dictionary.md` 单据类型码一节全量表重复时按裁定 C-26 拒绝并返回 `PLATFORM.CUSTOM_OBJECT.DOC_TYPE_CODE_CONFLICT`。

#### 8.2 领域属性测试（proptest）

对应规格第 17.2 章的领域属性测试类型，本阶段贡献四组不变量。

1. 内容项哈希与差异的自反性：任意合法内容项集合与自身做差异得到空集；对其中任意一项做任意非空改动后，差异必含且仅含该项。
2. 元数据往返：任意合法对象定义序列化为配置包内容项再解析回来，与原定义相等。
3. 发布与回退的元数据段幂等性：任意合法配置包 apply 后 revert，`platform_meta` 中除 `definition_version`、审计与仅追加表之外的全部行与 apply 之前逐列相等。
4. DDL 计划补偿完备性：对任意合法语句序列，在任意前缀位置注入失败后执行补偿，数据库结构（`information_schema` 的表、列、索引、策略四视图）与 `platform_meta` 均回到起点。

#### 8.3 集成测试（真实 PostgreSQL 16，禁用内存库与 mock）

每个用例独占一个数据库，按 `ep_test_<nanoid>` 建库，用例结束即删库。测试数据一律经 `ep-testkit` 构造器，禁止手写 INSERT。本表删除的行沿用原编号留号不补，以免打断第 8.7 节与第 9 节对集成用例序号的既有引用。

| 序 | 场景 | 判据 |
|---|---|---|
| 1 | 新建自定义对象并发布 | `ext` 表存在；`enable` 与 `force` 行级安全均为真；`rls_<table>_le` 策略存在；三条基线索引存在 |
| 2 | 两法人越权矩阵 | 对该自定义对象执行读取、写入、更新、删除、聚合、排序、报表投影与错误信息泄漏八类，另覆盖两个复制角色与内部对账系统安全上下文的五个入口借用，全部不返回对方法人数据 |
| 3 | 新增可空列的锁持有 | 在 100 万行 `ext` 表上执行，`ddl_plan_steps.lock_wait_ms` 加执行时长不超过 5000 毫秒 |
| 4 | 新增索引的执行时长 | 在附录 A.3 基准数据集规模的 `ext` 表上 `create index concurrently` 不超过 30 分钟；建成后目标查询的 `EXPLAIN` 无顺序扫描 |
| 5 | 锁超时的自动回退 | 人为持有冲突锁使 `add column` 超时，计划置 ROLLED_BACK 与 DEFERRED_TO_WINDOW；审计事件含回退原因、操作对象与耗时；数据库结构无残留 |
| 6 | 元数据与 DDL 的一致化 | 在 DDL 执行成功后、元数据置 ACTIVE 之前杀死 job-worker，重启后启动自检项 `custom-object-ddl-consistent` 检出不一致，进程照常启动，相关 `custom_objects` 置 DDL_FAILED 且其全部入口被隔离，降级窗口已开；执行修复用例后该项通过且降级窗口关闭 |
| 9 | 自审批拒绝 | 提交人调用 approve 返回 `SELF_APPROVAL_FORBIDDEN` |
| 10 | 并发发布 | 两个发布单同时执行，第二个返回 `CONCURRENT_RELEASE_IN_PROGRESS`；数据库结构与元数据无交叉污染 |
| 11 | 发布幂等 | 同一 `Idempotency-Key` 重复调用 execute，只产生一份逐步执行的审计事件序列，第二次响应头带 `Idempotent-Replay: true` |
| 12 | 回退不删数据 | 发布新增字段、录入 1000 行数据、回退，字段元数据置 RETIRED，物理列与 1000 行数据仍可由 `ep_analyst_ro` 读出 |
| 13 | 回退触发重新打标 | 权限类内容项回退后，搜索索引对应法人分区进入重建，重建期间检索入口返回 `DERIVED_STORE_REBUILD_REQUIRED`，重建完成后条数与来源一致 |
| 14 | 派生存储越权 | 以跨法人与跨密级安全上下文对自定义对象发起检索、排序与分面计数，均不返回无权数据 |
| 15 | 桌面端插件签名不符 | 篡改制品一个字节，enable 返回 `EXTENSION.SIGNATURE_INVALID`；对已 ENABLED 的插件篡改制品后按第 4.9 节第 1 条与第 2 条不加载并向中心上报，`extension_invocations` 增一行 outcome 为 SIGNATURE_MISMATCH |
| 17 | 桌面端插件自动停用 | 同一入口连续 3 次失败，插件置 DISABLED，第 4 次加载被拒绝，客户端审计与站内通知各一条 |
| 19 | 全仓无服务端 WASM 残留 | `cargo metadata` 中不出现 wasmtime 与 wasmtime-wasi；全仓检索 `plugin-host`、`ep-adapter-wasm`、`WasmComputePort`、`PluginHostWasmCompute`、`WasmCall`、`requires_wasm` 六个符号零命中；两个 wiring 中不出现任何 Noop 前缀符号 |
| 21 | 能力闸 | `X-Client: ios` 调用付款登记提交返回 403 与 `WRITE_NOT_AVAILABLE_ON_CLIENT` 并带 `X-Desktop-Handoff-Token`；`X-Client: win` 放行；`X-Client: ios` 调用扩展登记返回 404 |
| 22 | 矩阵冻结降级 | 篡改 `client_capability_values` 一行，进程照常启动，运行期判定以内置快照为准，对该表的写入被拒绝，降级窗口已开且告警已发；`--check` 模式在同一状态下以非零退出 |
| 23 | 引导下发可审计 | 调用 `client-bootstrap` 一次，`client_bootstrap_dispatches` 增一行含对象清单与规则版本，审计事件增一条 |
| 24 | ep_migrator 连接的启用与回收 | 一次含 DDL 的发布产生恰好两条审计事件（启用与回收）；发布前后 `pg_stat_activity` 中 `ep_migrator` 连接数为 0 |
| 25 | 配额上限 | 对象数达 200、单对象字段数达 100、单对象索引数达 5 时，再新增返回 `QUOTA_EXCEEDED` |
| 26 | 模块生命周期 | 经阶段 3b 的 `ModuleLicenseQuery::module_state` 把含自定义对象的模块置 INSTALLED_DISABLED，其定时任务不再触发、对外事件停止投递，停用前后该对象的记录条数与校验和一致，停用期间授权查询与审计检索仍可执行；再启用后定时任务与对外事件投递恢复，配置、权限授予与 Outbox 未投递条目差异为零 |
| 27 | 并发编辑 | 两名配置管理员同时编辑同一内容项，第二人提交返回 `PLATFORM.CONCURRENCY.STALE_VERSION`，重取后可提交 |
| 28 | 关账期间暂停发布 | 受理一次关账请求后提交发布单执行，返回受理但排队；关账产生结论后自动继续执行 |
| 29 | 迁移窗口未打开 | 未登记打开的迁移窗口时执行含 DDL 段的发布，`MigrationWindowGuard::assert_open` 拒绝，返回 409 与 `PLATFORM.DB.MIGRATION_WINDOW_CLOSED`，全程 `pg_stat_activity` 中不出现 `ep_migrator` 连接；打开窗口后同一发布单执行成功 |

外部电子签章不在本阶段范围内，本阶段不使用 wiremock 打桩。

#### 8.4 端到端测试

桌面端用 Playwright 驱动桌面 WebView，用 tauri-driver 驱动桌面壳；移动端用 XCUITest 与 Espresso。

四端矩阵覆盖按规格第 6.2 章：取值为完整或简化的能力域在该端跑通端到端场景；取值为仅查看或不适用的能力域按豁免清单载明的替代路径验证。
按裁定 A-23，业务闭环类端到端用例随各业务阶段的四端界面交付：下表 E1 由阶段 9b 在其 `testkit/scenarios/golden_loop_14_steps.rs` 之上执行，E2 至 E6 由其所属业务阶段在自己的第 8 节测试计划中执行，其中 E5 跨六个仅查看能力域，由各能力域所属阶段分别执行，本阶段只交付其运行所需的客户端壳、路由注册表与能力闸，并对执行证据逐条汇总；E7 至 E12 属壳层、发布链路与白标制品，由本阶段自行执行，其中 E9 以本阶段的自定义对象单据为被测对象，不依赖任何业务模块界面。四端验收矩阵由阶段 14 汇总。

| 序 | 用例 | 端 | 判据 |
|---|---|---|---|
| E1 | 黄金业务闭环 14 步全程 | Windows、macOS | 整条链路的贯通验收由阶段 9b 的 `testkit/scenarios/golden_loop_14_steps.rs` 承担，本阶段只汇总其在 Windows 与 macOS 两端的走查证据；记录、校验、审计与结果与服务端集成测试一致 |
| E2 | 库存台账与收发扫码完整闭环 | 四端 | 移动端连续扫码 100 次识别率不低于 99%，单次识别不超过 1 秒 |
| E3 | 售后工单与设备台账完整闭环 | 四端 | 同一操作在任一端发起产生的记录与审计相同 |
| E4 | 审批待办与站内通知完整闭环 | 四端 | 站内通知在四端均可查看、跳转与标记，无权时按权限拒绝处理 |
| E5 | 六个仅查看能力域的写入入口缺失 | iOS、Android | 财务过账与期末结账、收付款登记与对账查看、发票申请与开具登记、报表与像素级打印、文档与附件协作、系统管理与低代码配置六类均不出现提交、审批与写入入口；进入写入路径时给出该操作在桌面端完成的说明并可发送到桌面端继续 |
| E6 | 含自定义对象与声明式规则的移动端场景 | iOS、Android | 字段级权限裁剪、审计结果与恢复连接后的中心重校验三者一致；断网时该单据只能保存为待中心校验草稿，草稿在中心不产生业务记录 |
| E7 | 配置发布与回退全流程 | Windows | 差异审查、审批、发布、回退四步全程可执行，全过程记名记时写入审计 |
| E8 | 打印机与 USB Key 端到端 | Windows、macOS | 各一次成功；关闭原生插件加载后能力停用并显式降级，高密级内容改为只读预览并禁止下载，降级事件与范围记入客户端审计 |
| E9 | 断网草稿与恢复提交 | 四端 | 断网期无法完成审批、过账、开票与任何状态流转；草稿保存在本地加密缓存；恢复后由中心重新校验并提交；同一记录冲突以中心版本为准 |
| E10 | 白标制品四端启动 | 四端 | 应用图标、启动页、登录页与关于页显示 `brand.toml` 中的产品名与 Logo |
| E11 | 强制安全更新 | 四端 | `is_forced_security_update` 为真时客户端在更新完成前拒绝进入业务界面 |
| E12 | 读屏软件端到端下单 | 四端 | 各完成一次；WCAG AA 自动检查零严重问题 |

#### 8.5 性能与容量

附录 C.2 十二项门槛在附录 C.1 设备基线上复测，每项以旧机型或中端机结果为准，通过线逐项照抄附录 C.2，本计划不重写数值。

本阶段另设三项，均为本阶段新增取值：

- `client-bootstrap` 的 P95 不超过 2 秒，按规格第 16 章常规交互通过线。
- 一次含 20 个内容项且含 3 条 DDL 语句的发布在附录 A.3 基准数据集上的段一加段二总时长不超过 30 分钟，与规格第 7.4 章迁移执行上限一致。

附录 A.1 度量清单内涉及自定义对象的查询在基准数据集上不得出现顺序扫描，阶段计划提交对应查询的 `EXPLAIN` 证据。

#### 8.6 覆盖率门槛

- `ep-platform-meta` 与 `ep-platform-release` 属平台内核代码，行覆盖率不低于 85%。
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
5. `platform_meta.client_capability_values` 的内容哈希与二进制内置冻结快照一致，启动自检项 `client-capability-matrix-frozen` 通过；人为篡改一行后进程照常启动，运行期判定以内置快照为准，对该表的写入被拒绝，降级窗口已开，`--check` 模式在同一状态下非零退出。
6. 五类定制各完成一次端到端发布与一次回退：数据定制与界面定制由本阶段在 `ep-platform-meta` 实现的六个 applier 承担；FLOW_DEFINITION 与 NOTIFY_RULE 由阶段 3b 的 `FlowDefinitionApplier` 与 `NotifyRuleApplier` 承担，三个 AUTHZ_ 类由阶段 4 的三个 applier 承担，四个报表类由阶段 11 的四个 applier 承担，归属按裁定 A-19，均在阶段 3b 的六态发布通道上跑通，这九个类别至少各有一次 apply 与一次 revert 的执行记录。
7. 一次含 3 条 DDL 语句的在线发布在基准数据集上完成，单条语句锁持有不超过 5 秒，计划总执行时长不超过 30 分钟，`ddl_plan_steps` 中逐条有实测的锁等待与执行时长。
8. 人为制造锁超时后计划自动回退并转停机窗口，审计事件含回退原因、操作对象与耗时，数据库结构无残留。
9. 回退演练完成：按新增字段录入的业务数据在回退后仍可读出，字段元数据为 RETIRED，界面与 API 不再暴露该字段。
10. `tests/rls_matrix` 追加自定义对象与自定义查询入口两类后全部通过，八类越权面全覆盖。
11. 桌面端原生插件的子进程不共享客户端进程内存、不接收本地缓存数据库密钥、不接收会话令牌三条各有一次断言；传入子进程的报文只含按 `READ_OBJECT_FIELDS` 授予裁剪后的字段。
12. 桌面端原生插件连续失败自动停用生效，停用事件写入客户端审计并经站内通知送达，同时随设备健康状态上报中心。
13. 桌面端打印机与 USB Key 各完成一次端到端验证；关闭原生插件加载后能力停用并显式降级，降级事件与范围记入客户端审计并按客户与设备登记。
14. 服务端能力闸对移动端六个仅查看能力域的写入端点一律返回 403 与 `PLATFORM.CLIENT_CAPABILITY.WRITE_NOT_AVAILABLE_ON_CLIENT` 并带 `X-Desktop-Handoff-Token`，桌面接续令牌可拉起同一单据同一草稿；移动端界面上这六个能力域无提交、审批与写入入口一条按裁定 A-23 由各业务阶段随其界面验收，本阶段汇总其执行证据。
15. 断网时移动端只能把单据保存为待中心校验草稿，恢复连接后由中心重新校验并写入审计，该场景在 iOS 与 Android 各执行一次。
16. 覆盖率门槛按第 8.6 节逐项达成，CI 强制生效。
17. 本阶段新增的 27 条错误码、9 个事件类型、15 张表、5 个指标、23 个配置项在 `docs/error-codes.md`、`docs/event-catalog.md`、`docs/data-dictionary.md` 与代码常量表中登记齐备，CI 一致性校验通过；本阶段引用但由阶段 1 按裁定 C-24 登记的 `PLATFORM.CONCURRENCY.STALE_VERSION`、`PLATFORM.AUTHZ.NOT_FOUND_OR_DENIED` 与 `PLATFORM.DB.MIGRATION_WINDOW_CLOSED` 不计入本阶段条数。删减明细为：错误码删 10 条即配置包签名与验签四条、自动测试一条、编辑锁一条、插件资源与宿主不可达两条、规则解析与节点上限两条；事件删 1 个即 `platform.config_release.released.v1` 随最小发布通道归阶段 3b；表删 4 张见第 3.2.13 节；指标删 4 个见第 4.8 节；配置项删 12 个见第 7 节。
18. 依赖方向自检脚本通过：客户端 crate 不依赖任何 `ep-app-*` 与 `ep-adapter-db*`；`ep-platform-meta` 与 `ep-platform-release` 不出现 sqlx、reqwest、`std::fs`、`std::net` 与 `SystemTime::now` 符号。
19. 一次完整的配置发布与回退演练证据包归档，含差异审查记录、审批记录、执行耗时、锁持有时长、回退结果与审计链验证结论；证据包不含自动测试报告与签名记录，两项已按第 4.7 节第 3 条删除并登记规格第 5.7 章。
20. 本阶段的偏离项与新增决定（第 12 节）已回写共享技术基线，基线更新经平台架构负责人确认。
21. 模块许可的停用与再启用验收通过：按裁定 A-05，`ep-platform-license` 本体与其三张表由阶段 3b 交付，本阶段只保留一条验收，即某模块置 INSTALLED_DISABLED 后其定时任务停止、对外事件停发，再启用后两者恢复，执行记录见集成测试 26。
22. 本阶段全部 `/api/v1/` 路由，即 `/api/v1/platform/` 与 `/api/v1/ext/` 两段，其能力域码与动作类别常量已按裁定 A-20 声明并落 `crates/platform/meta/src/capability.rs`，能力域一律取 `CapabilityDomain::PlatformAdminLowcodeOps`，`xtask configdoc` 通过；第 5.3 节配置包与发布单两段的常量归阶段 3b，本阶段不声明；自定义单据对象的 `doc_type_code` 与 `docs/data-dictionary.md` 单据类型码一节的全量表无重复，`xtask configdoc --check-doc-type-codes` 通过。
23. 含 DDL 段的发布在未打开迁移窗口时被经装配注入的 `ep_adapter_db::port::MigrationWindowGuard` 实例的 `assert_open` 拒绝并返回 409 与 `PLATFORM.DB.MIGRATION_WINDOW_CLOSED`，留有一次拒绝与一次放行的执行记录；两个 wiring 中不出现 `NoopWasmComputePort`、`NoopRuleEvaluator` 与任何 Noop 前缀符号，`ep-adapter-wasm`、`WasmComputePort`、`PluginHostWasmCompute` 与 `AstRuleEvaluator` 在全仓无残留符号，`cargo metadata` 中无 wasmtime 与 wasmtime-wasi。
24. 本阶段向 T0 贡献的桌面壳最小切片按第 1.5 节的五项逐项交付，并在 T0 演练中完成一次判据走查：T0 的那一条合同在 Windows 桌面端建单，并在同一端看到 T0 的那张收入报表；该切片之外的本阶段交付物不参与 T0，也不因 T0 提前交付。

---

### 10. 与规格和 PRD 的对应

#### 10.1 规格条目

| 规格章节 | 本阶段实现的条目 |
|---|---|
| 3.1 私有化与白标 | 同一核心代码通过品牌配置、签名模块、低代码包与插件产生差异；独立产品名称、Logo、应用标识、证书与商店版本；白标构建、签名、灰度与发布流水线化；厂商托管签名私钥在硬件密码机内双人控制并单独审计 |
| 3.6 白标分发路径与签名身份归属 | 应用商店分发与企业签名/MDM 私有分发两条路径；商店政策合规检查门禁四项；被拒与超期的替代形态切换记录 |
| 5.1 平台内核 | 元数据、自定义对象、字段、关系、索引和视图；低代码表单与规则；API、插件和连接器能力注册中的桌面端原生插件登记部分。配置发布归阶段 3b，服务端 WASM 插件按第 4.8 节删除并登记规格第 5.7 章 |
| 6.1 技术路线 | Tauri 2 加 React/TypeScript；Rust 承担认证、安全策略、本地缓存数据库、同步、领域校验与设备能力；TypeScript 只承担界面；客户端全部网络 I/O 经 Rust 核心统一出口，TLS 通道与证书链校验由 Rust 核心执行 |
| 6.2 一致性与兼容 | 四端业务能力等价的运行期判定与豁免清单全部条目；18 行取值矩阵的机器可读落地与冻结校验；外设范围；后台常驻与长时任务；动态扩展代码；自定义对象与低代码规则的移动端豁免；本地缓存与文件；深度编辑；表格；财务与结账；收付款与对账；系统管理与配置发布；文档与 PDF 协作；分析与报表创作 |
| 6.3 本地缓存与设备 | 设备独立硬件保护密钥；本地缓存与凭据加密并绑定 TPM、Secure Enclave 或 Keystore；缓存只保存权限内子集；按记录版本与变更时间增量拉取；冲突以中心为准；断网只保存本地草稿；设备登记、远程注销与安全清除；缓存超期清除 |
| 6.4 文档、表格与设备 | 原生设备通过签名插件适配打印机与 USB Key/智能卡；移动端只使用随包静态签入的相机扫码；条码的企业自定义编码规则生成与识别 |
| 7.4 可定制数据库 | 自定义对象、字段、关系、校验、索引和视图；真实表加在线 DDL；不使用 EAV；公共能力基线的类型与索引限定；在线变更边界与运行期超限自动回退；对象级与字段级密级建模时赋值；随会话下发对象结构、字段密级、权限策略与规则版本且下发范围可审计；默认配额；发布前五项影响分析；声明式包进入 Git 支持差异审查与回退 |
| 7.9 派生存储安全继承 | 权限模型、密级规则或分区规则变更时受影响派生存储在变更生效前完成重建或重新打标；重建期间该分区停止对外服务；重建后重放与条数一致性校验 |
| 9.1 低代码能力 | 自定义对象、表单布局、列表列、首页、菜单与看板；声明式规则表达式与版本化规则，求值器为阶段 3b 的唯一表达式求值器；不允许任意 JavaScript、Python、Shell 或本机程序进入核心环境。“复杂计算调用受限 WASM 函数”一句按第 4.5 节第 5 条删除并登记规格第 5.7 章，超出该求值器表达能力的计算按厂商侧领域代码交付 |
| 9.2 配置发布 | 开发测试与生产隔离；配置进入 Git 经差异审查与审批；验证失败阻止生产发布。自动测试与签名两句按第 4.7 节第 3 条删除并登记规格第 5.7 章 |
| 9.3 模块与插件 | 桌面端签名原生插件的九项安全边界；插件必须声明能力、对象、字段与资源限额；权限经审批后最小授予；不能直连核心数据库也不能读取明文机密。服务端 WASM Component 形态按第 4.8 节删除，扩展运行时首版只剩桌面端签名原生插件与移动端静态签入两种形态，规格第 3.3 章“首版只提供企业私有扩展”改写为“首版企业私有扩展只有桌面端签名原生插件一种形态”，并同步交付说明与客户合同措辞 |
| 12.4 DLP 与隐私 | 桌面端与移动端的强制控制；关闭原生插件或设备不合规时的降级为门户端口径与只读预览禁止下载；降级事件与范围记入审计 |
| 13.1 正式拓扑 | 本阶段不新增进程。plugin-host 与其 5% 的 CPU、内存与磁盘 IO 份额按第 4.8 节删除，规格第 13.1 章配额表删去插件运行时一行，其余行权重一律不改，删出的份额进入盈余不重新分配，让路顺序第 8 级改为“其余后台任务”，以免再触发一次配额标定与认证重测 |
| 15.3 运维中心 | 本阶段 5 个指标进入运维中心；配额触发限流事件记入运维中心 |
| 17.2 自动化测试 | 四端端到端测试；数据库适配认证的自定义对象测试项与在线变更实测；派生存储越权与传播测试；模块生命周期测试 |
| 17.3 强制不变量 | 权限不能跨法人、字段或密级越权；审计链可验证 |
| 18 升级、版本与生命周期 | 客户端支持分批发布与强制安全更新 |
| 附录 C 四端客户端 PoC 量化门槛 | C.2 十二项门槛在 C.1 设备基线上的首版验收复测 |

#### 10.2 PRD 条目

| PRD 节 | 本阶段实现的功能 |
|---|---|
| 8.4.1 报表设计器 | 报表定义作为配置包内容项进入发布通道；本阶段不实现设计器本体与取数 |
| 8.4.2 企业自定义指标 | 指标表达式复用阶段 3b 的唯一表达式求值器，本阶段不另建 AST 与解释器 |
| 8.4.5 报表类配置对象的状态与流转 | 四类报表配置对象通过本阶段的 `ConfigItemApplier` 进入统一发布与回退闭环 |
| 10.2.2 配置对象与配置操作 | 权限配置的新版本进入本阶段的发布流程后生效；发布前执行法人越权测试集 |
| 10.2.4 权限拒绝的用户可见行为 | 能力闸的拒绝按规格第 15.1 章分类，不泄露存在性 |
| 10.3 六类高风险操作 | 移动端取值为仅查看的高风险操作不提供提交入口并显式说明在桌面端完成；桌面端 USB Key 与智能卡的重新认证由本阶段的原生插件承载取证接口 |
| 10.4.1 配置发布的通用生命周期 | 6 个状态与其流转、触发条件与触发人逐行落地；发布前五项影响分析；发布与回退全过程记名记时写入审计。其余五态按第 3.1 节删除或折叠并登记规格第 5.7 章 |
| 10.4.2 数据定制 | 能改与不能改逐条；在线与停机边界；建模必填的对象级与字段级密级；发布前影响分析加自动测试；运行期超限自动回退并转停机窗口且记入审计；回退按声明式包差异审查后执行 |
| 10.4.3 界面定制 | 表单布局、列表列与列顺序、首页、菜单、看板；不得以隐藏字段替代字段级权限；不得改变四端取值矩阵；不得引入任意 JavaScript；按角色预览核对无权字段确实不返回；按上一已发布版本整体回退 |
| 10.4.4 流程定制 | 流程定义作为配置包内容项进入同一发布通道与版本回退；本阶段不实现流程引擎本体 |
| 10.4.5 权限定制 | 权限内容项进入同一发布通道；变更生效前完成受影响派生存储的重建或重新打标；发布前执行法人越权测试集且新增自定义对象与新增查询入口必须通过 |
| 10.4.6 报表定制 | 报表类内容项进入同一发布通道与整体回退，且不影响已导出的历史文件 |
| 10.4.7 五类定制的共同硬边界 | 不得跨模块直接读写业务表；不得取得事务业务库直连；系统管理与低代码配置在移动端只提供查看与告警处理 |
| 10.7.1 白标的可见范围 | 品牌配置项清单在 `brand_profiles` 中冻结为 U-K-07 的临时取值 |
| 10.7.2 分发路径的可见差异 | 两条分发路径与切换记录 |
| 10.7.3 四端的用户可见差异 | 仅查看能力不出现写入入口并给出桌面端说明；同一操作在任一端结果相同；移动端不接受动态扩展代码；断网时只能保存待中心校验草稿；不承诺后台常驻；关闭原生插件后的降级；本地缓存与单文件上限可下调且可审计 |
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
| 桌面端原生插件的子进程成为越权入口 | 对应规格第 21.7 章风险 | 子进程不共享客户端进程内存、不接收本地缓存密钥与会话令牌；传入报文按字段权限裁剪；签名主体、版本与哈希三项核对不通过即不加载 | 按规格第 3.3 章在本实例内停用该插件，停用决定、影响范围与恢复条件记入审计 |
| 白标维护矩阵膨胀 | 对应规格第 21.8 章风险 | 单一核心加配置化品牌；客户不维护长期核心代码分支；构建、签名、灰度全流水线化；可复现构建使制品哈希可核对 | 品牌配置项清单冻结在 `brand_profiles` 的列集内，新增可配置项必须先改该表并回写 U-K-07 决策 |
| 配置回退删掉已录入业务数据 | 对应 U-K-02 未决 | 本阶段取值为回退只停用元数据不执行 DROP | 该取值在 U-K-02 决策后按决策调整；若决策要求物理删除，改由单独的停机窗口计划加双人审批，并按裁定 A-22 经 `DisposalPort` 交由阶段 14 的 `OpsDisposalService` 按规格第 12.4 章处置清单承担 |
| 生产环境内的就地创作与规格第 9.2 章开发测试生产隔离的张力 | 审计与合规口径受质疑 | 生产内的 DRAFT 状态配置对运行期一律不可见，运行期只读取 ACTIVE 版本；差异审查以 Git 中的声明式包为准；包在提交审批后内容不可再改 | 若客户或审计要求更严，收窄为只在停机窗口内执行发布，并把提交人与审批人限定为两名不同的具名账号；不恢复导入式包与包签名，恢复需先按规格第 5.7 章重新作出范围决策 |

#### 11.2 为后续阶段预留的扩展点

- `ConfigItemApplier` 端口由阶段 3a 按裁定 A-19 交付，其 `item_kind` 取值集合可扩展。新增一类定制内容项时只需实现该 trait 并在 wiring 注册到 `ConfigItemApplierRegistry`，发布链路、差异算法、审批与回退全部复用，不改本阶段任何表。
- `CapabilityValue` 枚举与 `client_capability_values` 表结构支持新增能力域行与新增端列。恢复客户门户或经销商门户配套应用时，只需新增能力域行与新的 `client` 取值，不改判定算法。
- `extension_capability_grants.capability` 的取值集合封闭在 3 项。新增外设适配时新增取值并同步扩展桌面端插件的能力核对表；该核对表的断言测试是新增能力必须同步修改的强制点。恢复服务端扩展形态属范围决策，按规格第 5.7 章重新论证，不由本节的扩展点承接。
- 客户端本地缓存的记录标签已按规格第 7.9 章口径携带，为后续恢复离线数据租约、租约到期锁定与撤销序列、离线草稿字段级合并预留了判定依据。
- `ep-client-plughost` 的子进程帧格式直接引用基线第 2 节，能力清单只与本阶段的三项能力对齐，不再与已删除的服务端 WASM 宿主共用枚举；后续若恢复服务端扩展形态，宿主形态重新论证，不预先绑定 WASM Component。
- 白标构建流水线的可复现构建能力直接支撑规格第 3.2 章私有构建级源码许可的支持判据，后续开放该许可级别时不需要新建流水线。
- `ddl_plans` 的五项影响分析列为 jsonb，后续增加影响维度不需要迁移表结构。

#### 11.3 本阶段给出的临时取值与其阻塞判定

下列事项在 PRD 附录乙中为待决，本阶段给出技术侧临时取值以免阻塞实现。本阶段不被阻塞，但业务决策变更时的切换代价如下。

| 编号 | 本阶段临时取值 | 切换代价 |
|---|---|---|
| U-K-01 | 配置包按整包发布；并发编辑由内容项行上的 `row_version` 乐观锁承担，不设编辑锁 | 改为按单对象发布需要拆分 `config_release_orders` 的粒度并改写第 6.2 节的两项串行化判据，工作量中等 |
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
2. 非常驻工具目录。基线第 2 节的进程清单是常驻进程清单。本阶段新增 `/tools/` 目录只承载 `epbrand` 一个一次性命令行工具，不属于进程清单，不占用系统账户与 cgroup。原 `epcfg` 与 `epplug` 两个工具分别按第 4.7 节第 3 条与第 4.8 节删除。
3. 客户端本地加密缓存库选型。取 SQLCipher，经 rusqlite 的 bundled-sqlcipher 特性引入。理由是附录 C.2 要求本地加密数据库随机读写吞吐不低于 20 MB/s、10 万行查询 P95 不超过 1 秒，纯 Rust 的嵌入式库在加密路径上尚无同等实测证据。该选型只作用于客户端，不触及基线第 3 节的服务端数据库约定。
4. 服务端扩展形态的收窄。基线第 2 节的进程清单由八进程改为七进程，删去 plugin-host；workspace 根 `[workspace.dependencies]` 中不出现 wasmtime 与 wasmtime-wasi；`ep-adapter-wasm` 不存在；首版企业私有扩展只有桌面端签名原生插件一种形态。恢复条件按规格第 5.7 章登记，与扩展市场同批评估，重建时重新论证宿主形态，不预先绑定 wasmtime。
5. 部署级配置表的归类。本阶段涉及的 16 张部署级表按基线第 3.8 节第四段归入“全局配置字典”类，不带 `legal_entity_id` 与 `data_scope_tags`，不建行级策略，其余公共列齐备。其中 13 张由本阶段建立，`config_packages`、`config_package_items` 与 `config_release_orders` 三张由阶段 3b 按裁定 A-27 建立并沿用同一归类。理由见第 3.1 节。
6. 唯一约束中的空值替代取值。`custom_fields.owner_key` 与 `ui_layouts.role_key` 在语义为空时取 `'-'`，与基线第 11.4 节空批次标识的理由同构：该列是唯一索引的组成键，NULL 在唯一约束中的语义会使重复定义得以并存。
7. 配置包状态机的取值与基线第 3.6 节的复原。本阶段不再追加可物理删除的第三类表，`platform_meta.config_edit_locks` 已删除，基线第 3.6 节允许物理删除的表仍只有两类。配置包状态固定为第 4.2 节的六态，PRD 第 10.4.1 节其余五态的删除与折叠一并回写基线。
8. 平台侧 API 路径段取 `platform`，自定义对象数据端点路径段取 `ext`。两者都不新增模块码，`ext` 与 schema 名一致。
9. 自定义对象的领域事件统一为 `platform.custom_record.created.v1`、`platform.custom_record.updated.v1`、`platform.custom_record.state_changed.v1` 三个类型，具体对象由信封的 `aggregate_type` 承载为 `ext.<object_code>`。理由是不新增模块码。
10. 幂等作用域中法人维度对部署级端点的取值。取请求头 `X-Legal-Entity-Id` 的值，理由见第 6.3 节。
11. 启动自检项按裁定 C-25 改为按注册名标识，不再以序号称呼。本阶段按该裁定的注册顺序追加 `client-capability-matrix-frozen` 与 `custom-object-ddl-consistent` 两项，两项按架构审计 ARCH-02 一律登记为 Degrading 级，判读结果不阻断进程启动，只按第 7 节写明的运行期后果处置并登记降级窗口，`--check` 对 DEGRADED 仍非零退出；后者一并覆盖基线项 `rls-enabled-and-forced` 对 `ext` schema 的扩展；扩展制品校验与品牌附件校验不再作为启动自检项，改由第 4.9 节加载路径与品牌激活用例承担。自检项的 severity 取值域为 Blocking 与 Degrading 两值，需回写基线第 7.3 节。
12. 覆盖率门槛追加客户端 Rust 核心五个 crate 不低于 85%，TypeScript 界面包不低于 70%。
13. 关账受理期间暂停新发布单执行，取值见第 6.5 节。
14. 客户端本地缓存记录携带来源对象标识、版本、法人标识、密级与数据范围标签，与规格第 7.9 章派生存储同一口径。理由见第 4.10 节。

本阶段不偏离基线第 3.5 节的金额与数量精度、第 3.8 节的行级策略模板、第 5 章的封套与分页与幂等、第 6 章的事件与 Outbox、第 10.3 节的事务边界、第 10.4 节的分层边界。
