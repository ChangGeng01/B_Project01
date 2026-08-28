# 数据字典

> **F-57 状态：`REGISTRY_PENDING_REBASELINE`。** 现有表、字段和码值只在 F-57 未取代的窄范围内可复用；实施计划 Task 1 完成 F-57 数据所有者、动态权限、generation、能力包、自动化、HDD 证据和经营账边界登记前，本文件不得单独作为执行闭集。

## F-57 立即生效的字段取代

| 旧字段/约束 | 状态 | F-57 替代 |
|---|---|---|
| `deployment_records.rto_hours default 4`、`recovery_drills.rto_seconds <= 14400` | `SUPERSEDED` | `measured_rto_seconds` 与 `certified_rto_seconds` 必须绑定 `hardware_profile_id + data_volume_bytes + software_generation + drill_id`；没有实测不填承诺值 |
| `virus_scan_mode=NONE` 的生产形状 | `SUPERSEDED` | 生产固定 `REQUIRED_PROVIDER`；provider 不可用/超时/未知时附件保持隔离，发布门失败 |
| 单一 `offsite_sinks` 记录即可满足勒索恢复 | `SUPERSEDED` | 同时存在服务器外追加式自动增量层、完全离线加密轮换层和独立恢复材料；暖备、RAID、同机副本均不计 |
| `server_spec` 的旧 10 用户/64GB/统一磁盘假设 | `SUPERSEDED` | `P340_LOW_RESOURCE_V1`：约 20 活跃用户、32GB、重报表并发 1、本地模型关闭；值只由实机容量证书晋级 |
| SSD/`C:\ProgramData` 中的客户状态、秘密、spool、附件、索引或证据 | `SUPERSEDED` | 全部通过稳定 volume ID 派生到 HDD `{data_root}`；SSD 只承载 OS、程序和可重建静态物 |

旧段落为保留历史快照；出现上述旧字段或 CHECK 时，Task 1 必须按此表修改迁移 reservation 与目标模型，不能同时保留两套可用形状。

本文件是已冻结或已存在迁移的业务表与全局码表的规范登记处。尚未进入迁移的对象，以对应阶段计划中的冻结表模型为开发前契约，首次建立迁移时必须同批进入本文件或对应的 `docs/data-dictionary/<schema>.md`。表结构以已合入的迁移文件为准；迁移与字典不一致时不允许阶段退出，必须在同一变更中修正。

## 1. 组织方式

按 schema 分节，每个 schema 一节，节内按表名字典序。每张表一张列表，列固定为：列名、类型、可空、默认、语义。

跨阶段承重对象或开发前已冻结 schema 另有分册：`docs/data-dictionary/platform_flow.md`、`platform_audit.md`、`mdm.md`、`cpq.md`、`clm_sales.md`、`invoice.md`、`finance.md`、`ledger.md`、`portal.md`、`procure.md` 与 `ai_mcp.md`。总册与分册共同构成同一份数据字典；同一字段若两处重复，必须同批保持逐字同义，不允许用“分册较新”解释冲突。F-55 的四张新增表、部署 carrier 追加列及关联候选键以 `ai_mcp.md` 逐列登记。

公共列在第 2 节统一给出，各表不重复列出，只列该表自有的列。表定义处必须写明该表属单据类、档案类、会计相关类还是仅追加类，因为这四类各有附加列约定。

阶段 1 不建任何业务表，因此本文件在阶段 1 结束时没有任何 schema 分节。阶段 2 按 02 计划第 3.5 节交付 `platform_core` 与 `platform_ops` 两个分节，合计十四张表，见第 6、7 节。

## 2. 公共列

每张业务表必须有下列列，顺序也按此排列。

| 列名 | 类型 | 可空 | 默认 | 语义 |
|---|---|---|---|---|
| id | uuid | 否 | 无，应用侧 UUIDv7 | 主键 |
| legal_entity_id | uuid | 否 | 无 | 法人，RLS 唯一判据，任何跨法人引用禁止 |
| security_level | smallint | 否 | 20 | 密级，取值见第 3 节；字段级密级未赋值时按所属对象取值 |
| data_scope_tags | text[] | 否 | '{}' | 数据范围标签，元素形态见第 4 节 |
| row_version | bigint | 否 | 1 | 乐观锁版本 |
| created_at | timestamptz | 否 | now() | 创建时间，UTC |
| created_by | uuid | 否 | 无 | 创建人用户 ID |
| updated_at | timestamptz | 否 | now() | 最后更新时间，UTC |
| updated_by | uuid | 否 | 无 | 最后更新人用户 ID |

所有带 `legal_entity_id` 的业务表都必须把 `(legal_entity_id,created_by)` 与 `(legal_entity_id,updated_by)` 以真实复合外键指向 `platform_authz.user_legal_entity_grants(legal_entity_id,user_id) ON DELETE RESTRICT`；仅追加表的 `created_by` 同样适用。系统上下文一律写入 `ep_foundation::principal::SYSTEM_PRINCIPAL_ID`，字面量为 `00000000-0000-7000-8000-000000000001`，且阶段 4 必须为每个法人保留一条该主体的有效授权行。`platform_core.user_accounts` 中的固定 `account_kind=SYSTEM`、无凭据账号是全局身份主体与上述逐法人授权的上游行，不是带法人业务用户列的直接外键目标。`SYSTEM_SESSION_ID=00000000-0000-7000-8000-000000000002` 只用作 `SecurityContext::system` 的非空会话哨兵，`platform_core.sessions` 不建对应行，不得用于 reauth、续期或人类会话查询。

四类附加列：

- 单据类表另加 `doc_no text not null` 与 `status text not null`，`status` 带 CHECK 约束枚举该单据状态机的全部取值。
- 档案类表另加 `code text not null`、`is_active boolean not null default true`、`deactivated_at timestamptz null`。
- 凭证、子账条目及会计期间归属自身就是权威事实的表，同时带 `posting_date date` 或 `business_date date` 与 `accounting_period_id uuid`。仅登记来源业务动作、由同事务 `AccountingPeriodResolver` 解析期间并交给凭证/子账的业务单据只带业务日期，不保存第二份 `accounting_period_id`；各阶段逐表明确所属一类，不允许两处独立解析期间。
- 仅追加表不带 `row_version`、`updated_at`、`updated_by`；是否带 `reverses_id uuid null` 由该表有无业务冲销或更正语义决定，有的必须带并写明它指向哪张表的哪条记录，没有的一律不得带，不得为满足列约定而保留一个恒为 NULL 的该列。

不设 `tenant_id`、`deployment_customer_id` 或任何同义租户隔离列：一个部署客户的同一事务数据库可承载多个法人，隔离唯一由 `legal_entity_id`、强制 RLS 与独立密钥域承担。业务域中的固定单目标 `customer_id` 是指向 `mdm.customers` 的客户档案真实外键，可由合同、订单、工单等属主表按业务需要使用；双方带法人列时必须建立 `(legal_entity_id,customer_id) -> mdm.customers(legal_entity_id,id) ON DELETE RESTRICT`，不承担租户或法人隔离职责。

所有最终会写入 PostgreSQL `text`、`text[]`、`json` 或 `jsonb` 的字符串，在 JSON/TOML/转义序列完成解码后统一禁止 Unicode U+0000；校验必须发生在任何持久化、审计、Outbox、文件落地或其他业务副作用之前，不能依赖 PostgreSQL 驱动报错。普通入口沿其既有 INVALID_PAYLOAD/字段校验码失败；F-56 special 的 manifest/item 内业务字段统一映射 `PLATFORM.CONFIG_PACKAGE.SPECIAL_ITEM_SHAPE_INVALID` 并零落库，容器层 JSON/TOML 语法损坏仍沿既有外层 container 错误。契约测试至少逐一覆盖直接 NUL、JSON `\u0000`、TOML basic-string `\u0000`、数组/嵌套对象中的 NUL，以及相邻 U+0001/U+FFFD 正例，并断言失败时 package/item/audit/Outbox/staging 业务事实均为零。

附件引用不直接落在业务主表列上，一律经 `<主表单数>_attachments` 关联表；`owner_id`、`attachment_object_id`、`purpose`、`sort_no` 与公共列是必备最小列。模块可为可追溯性追加受约束的来源列，例如签章请求 id 与文件 ordinal，但不得在主表另存同一附件 id 或绕过平台附件发布状态。

## 3. 密级取值

`security_level` 为 smallint，四级取值如下。代码侧的对应物是 `crates/foundation/src/security/level.rs` 的 `SecurityLevel`，序列化为数字而不是变体名，与本表同源。

| 取值 | 名称 | 变体 |
|---|---|---|
| 10 | 公开 | `Public` |
| 20 | 内部 | `Internal` |
| 30 | 保密 | `Confidential` |
| 40 | 机密 | `Secret` |

未知取值一律反序列化失败，不静默降级为最低级。

## 4. 数据范围标签的元素形态

`data_scope_tags` 的每个元素形如 `<kind>:<value>`，kind 取 `[a-z0-9_-]`，value 取 `[A-Za-z0-9_-]`，总长上限 128，二者均不可为空。例：`dept:sales`、`project:P-2026-0007`。

该形态由 `crates/foundation/src/security/context.rs` 的 `DataScopeTag` 唯一实现，公共列与事件信封的同名字段共用它，两处不得各自编解码。

## 5. 单据类型码

本节是单据类型码与档案类型码的全局唯一登记表，单据类与档案类共用同一张表。类型码全局唯一，新增类型码必须先在本节登记再实现。

### 5.1 登记表

| 类型码 | 名称 | 类别 | 所属模块 | 登记阶段 |
|---|---|---|---|---|
| BGA | 应急账号启用单 | 单据 | platform | 阶段 4 |
| CDRV | 资金单据冲正单 | 单据 | finance | 阶段 10 |
| CPL | 客户投诉单 | 单据 | service | 阶段 12（F-51 开发前登记） |
| CORR | 总账更正凭证 | 单据 | ledger | 阶段 9（F-50 开发前登记） |
| CT | 合同 | 单据 | clm | 阶段 6（F-51 开发前登记） |
| CUST | 客户档案 | 档案 | mdm | 阶段 5（F-51 开发前登记） |
| DC | 交付确认单 | 单据 | sales | 阶段 6（F-51 开发前登记） |
| DN | 供应商送货通知 | 单据 | portal | 阶段 7 |
| EQ | 设备档案 | 档案 | service | 阶段 12（F-51 开发前登记） |
| GR | 采购收货单 | 单据 | procure | 阶段 7 |
| GV | 总账凭证 | 单据 | ledger | 阶段 9 |
| HRR | 高风险操作申请单 | 单据 | platform | 阶段 4 |
| INVA | 开票申请单 | 单据 | invoice | 阶段 10 |
| IRVS | 发票冲销登记单 | 单据 | invoice | 阶段 10 |
| MATL | 物料档案 | 档案 | mdm | 阶段 5（F-51 开发前登记） |
| MDCR | 主数据变更申请单 | 单据 | mdm | 阶段 5（F-51 开发前登记） |
| MDEX | 主数据导出任务 | 单据 | mdm | 阶段 5（F-51 开发前登记） |
| MDIB | 主数据导入批次 | 单据 | mdm | 阶段 5（F-51 开发前登记） |
| OBB | 总账期初余额导入批次 | 单据 | ledger | 阶段 9 |
| OBST | 超量开票结清单 | 单据 | finance | 阶段 10 |
| PAYM | 付款登记单 | 单据 | finance | 阶段 10 |
| PAYR | 付款申请单 | 单据 | procure | 阶段 7 |
| PCR | 会计期间关账请求 | 单据 | ledger | 阶段 9 |
| PINV | 进项发票 | 单据 | invoice | 阶段 10 |
| PO | 采购订单 | 单据 | procure | 阶段 7 |
| PR | 采购需求单 | 单据 | procure | 阶段 7 |
| PRJ | 项目 | 单据 | project | 阶段 12（F-51 开发前登记） |
| PRLS | 价目表 | 档案 | cpq | 阶段 5（F-51 开发前登记） |
| PROD | 产品档案 | 档案 | mdm | 阶段 5（F-51 开发前登记） |
| PRT | 采购退货单 | 单据 | procure | 阶段 7 |
| PT | 项目任务 | 单据 | project | 阶段 12（F-51 开发前登记） |
| RCPT | 到款登记单 | 单据 | finance | 阶段 10 |
| RFND | 退款与返款单 | 单据 | finance | 阶段 10 |
| RJ | 收货拒收单 | 单据 | procure | 阶段 7 |
| RT | 报表渲染任务 | 单据 | reporting | 阶段 11（F-51 开发前登记） |
| SINV | 销项发票 | 单据 | invoice | 阶段 10 |
| SIU | 供应商发票上传单 | 单据 | portal | 阶段 7 |
| SO | 销售订单 | 单据 | sales | 阶段 6（F-51 开发前登记） |
| SR | 销售退货单 | 单据 | sales | 阶段 6（F-51 开发前登记） |
| SUPP | 供应商档案 | 档案 | mdm | 阶段 5（F-51 开发前登记） |
| WHSE | 仓库档案 | 档案 | mdm | 阶段 5（F-51 开发前登记） |
| WO | 售后工单 | 单据 | service | 阶段 12（F-51 开发前登记） |
| YEC | 年末结转单 | 单据 | ledger | 阶段 9 |

现行全集固定为 **43 个**：阶段 4 两个、阶段 5 九个（含 F-51 新增的 `WHSE`）、阶段 6 四个、阶段 7 八个、阶段 9 五个（含 F-50 新增的 `CORR`）、阶段 10 九个、阶段 11 一个、阶段 12 五个。阶段 1 自身登记条数为 0；阶段 3a 建立 `ep-platform-sequence` 类型码常量表时必须一次装入本表 43 个值，不允许只装已开工阶段的子集。后续新增类型码仍须先改本表并同批更新常量表，任何阶段不得实现未在本节登记的码。

### 5.2 判定

代码侧的对应物是 `ep-platform-sequence` 的类型码常量表，判据是本节与该常量表逐项一致且无重复，由 CI 项 `xtask configdoc --check-doc-type-codes` 执行。

该常量表由 F-57 实现批次交付（原写「阶段 3a」，属已取代的十四阶段；F-67 对齐现行谓词）。在它存在之前，「逐项一致且无重复」的被测输入为空集，比对恒真；恒真的判据不作为通过判定，因此该逐项比对**在该节登记表出现第一行时自动生效**（`xtask/src/configdoc.rs:442` 逐字「该表出现第一行即自动生效，不以阶段号为触发谓词」；原写「推迟到阶段 3a 生效」与实现谓词相反，F-67 对齐）。阶段 1 对本节只判一件事：本节存在。

## 6. platform_core schema（阶段 2、3 与阶段 4）

本 schema 承载密钥域、数据密钥、敏感字段清单、仅追加登记、迁移窗口与未受行级策略表登记六类平台元数据，以及按裁定 A-04 归入的集团、组织、部门、岗位与部门层级闭包五张组织架构表（阶段 2，共十三张，含表六附带的单例锁表），阶段 3 的 `module_registrations`、`license_grants`、`feature_flags` 三张部署级表与影响面评估两张法人表，以及阶段 4 任务 #20 交付的九张身份主体表，共二十七张。凡不带 `legal_entity_id` 的表一律登记在 `unpoliced_table_registry`，由 `db/checks/13` 强制：阶段 2 登记八行；阶段 3 的上述三张部署级表随 `V20261013093300` 登记；阶段 4 九张身份主体表与 `platform_authz.permission_items/object_scope_bindings` 由第 29 号回填迁移一次写入。阶段 3 两张影响面表均带法人并 ENABLE、FORCE RLS，不进该登记表。

表定义类别说明：本 schema 各表均非单据类也非会计相关类；`legal_entities`、`enterprise_groups`、`organizations`、`departments`、`positions` 为档案类（自带 `code`、`is_active`、`deactivated_at` 或按表形态略去停用两列），其余为登记/台账/机制类，不适用四类附加列约定，各自的登记口径在下表逐列写明。

### 6.1 legal_entities（法人注册表，档案类）

不带 `legal_entity_id`、不建行级安全策略（隔离机制自身的元数据，登记于表十三）。公共列除 `legal_entity_id` 外八列居首。

| 列名 | 类型 | 可空 | 默认 | 语义 |
|---|---|---|---|---|
| group_id | uuid | 是 | 无 | 所属集团，同 schema 真实外键指向 enterprise_groups，ON DELETE RESTRICT |
| code | text | 否 | 无 | 法人码，长度 1 至 64，全库唯一 |
| entity_no | text | 否 | 无 | 两位数字法人码，正则 `^[0-9]{2}$`，全库唯一，供单据编号法人段取用 |
| name | text | 否 | 无 | 法人名称，长度 1 至 200 |
| short_name | text | 是 | 无 | 简称，长度不超过 64 |
| display_timezone | text | 否 | 'Asia/Shanghai' | 展示时区，CHECK 只允许该取值 |
| currency_code | text | 否 | 'CNY' | 本位币，CHECK 只允许该取值 |
| is_active | boolean | 否 | true | 档案启用态 |
| deactivated_at | timestamptz | 是 | 无 | 停用时间 |

### 6.2 key_domains（密钥域）

带 `legal_entity_id`，策略 `rls_key_domains_le`。`security_level` 默认 40（机密）。

| 列名 | 类型 | 可空 | 默认 | 语义 |
|---|---|---|---|---|
| domain_kind | text | 否 | 无 | 首版只放行 `LEGAL_ENTITY`；`GROUP_SHARED` 预留不放行 |
| state | text | 否 | 无 | `PROVISIONING`、`ACTIVE`、`DESTROY_PLANNED`、`DESTROYED` 四态 |
| kek_ref | text | 否 | 无 | provider-independent logical locator；exact 为 `kms://ep/v1/deploy/<lowercase-deployment-uuid>/domain/<lowercase-key-domain-uuid>/kek/1`，两个 UUID 都是 canonical lowercase，domain UUID 必须等于本行 `id`；不编码 builtin/HSM、slot、文件或 provider 路径 |
| kek_version | int | 否 | 1 | KEK 逻辑版本，持久域 1..2,147,483,647；首版 exact 1，locator 尾段与本列逐字相等；Rust 可用 u32 但 adapter/DTO 入库前拒绝大于 i32::MAX，禁止 cast/wrap |
| provisioned_at | timestamptz | 是 | 无 | 供给完成时间 |
| destroy_planned_at | timestamptz | 是 | 无 | 销毁计划时间 |
| destroyed_at | timestamptz | 是 | 无 | 销毁完成时间 |
| destroy_evidence_ref | text | 是 | 无 | 销毁证明的审计引用 |

`key_domains.legal_entity_id -> legal_entities(id) ON DELETE RESTRICT` 是真实 FK；除主键外必须有 `UNIQUE(legal_entity_id,id)`，供所有子表建立同法人复合 FK。唯一约束 `ux_key_domains_legal_entity_id_domain_kind` 继续保证一个法人至多一个同类密钥域。

`ck_key_domains_kek_locator` 只在本行内强制 exact grammar、locator 中 domain UUID 逐字等于本行 `id`、尾段版本逐字等于 `kek_version`；SQL CHECK 不能把 locator 中 deployment UUID 与外部签名部署清单比较。该 deployment 绑定只能由 `KeyDomainProvisioner` 在标准供给、bootstrap resume 与 Stage 14 证据验证时通过受信 manifest 重验，任何文档或测试不得伪称普通 CHECK 已证明。locator 在 `PROVISIONING` 即写入，只表示预定不可变对象身份，不证明 provider 中 KEK 已存在。`ck_key_domains_state_shape` 固定：PROVISIONING 的 `provisioned_at/destroy_planned_at/destroyed_at/destroy_evidence_ref` 全空；ACTIVE 恰有 `provisioned_at` 且后三项全空；DESTROY_PLANNED 恰有 `provisioned_at,destroy_planned_at` 且销毁两项为空；DESTROYED 四项全非空，时间单调 `provisioned_at<=destroy_planned_at<=destroyed_at`。所有状态的 `kek_ref/kek_version` 都非空，PROVISIONING 不允许用 null locator 代替待供给。只有目标法人完全没有 `key_domains` 行时返回 `PLATFORM.KEY_DOMAIN.NOT_PROVISIONED`；一旦 PROVISIONING 行已持久化，后续 KEK/DEK/KMS/readback/16 矩阵任一失败统一返回 `PLATFORM.KEY_DOMAIN.KEY_UNAVAILABLE`，不得回退 NOT_PROVISIONED。

### 6.3 data_keys（数据密钥台账）

带 `legal_entity_id`，策略 `rls_data_keys_le`。`security_level` 默认 40。

| 列名 | 类型 | 可空 | 默认 | 语义 |
|---|---|---|---|---|
| key_domain_id | uuid | 否 | 无 | 所属密钥域；只允许复合真实 FK `(legal_entity_id,key_domain_id) -> key_domains(legal_entity_id,id) ON DELETE RESTRICT`，禁止仅按 id 跨法人引用 |
| purpose | text | 否 | 无 | `FIELD`、`BLIND_INDEX`、`ATTACHMENT`、`ARCHIVE` 四用途 |
| security_level_scope | smallint | 否 | 无 | DEK 服务的密级子域，取 10、20、30、40 |
| version | int | 否 | 无 | 同域同用途同子域内的 data-key 版本号，范围 1..65535；Rust/DTO/adapter 统一非零 u16，EPC1 header 固定 2-byte u16 |
| algorithm | text | 否 | 无 | `AES_256_GCM` 或 `HMAC_SHA256` |
| operational_wrapped_key | bytea | 否 | 无 | 同一 DEK 面向日常 TPM/HSM/KMS operational recipient 的 ADR-0020 strict JCS `OperationalDataKeyEnvelopeV1` exact bytes；必须正长度且不得作为 provider-private blob |
| operational_wrap_key_version | int | 否 | 无 | operational 包裹密钥版本，范围 1..2,147,483,647；Rust u32 超出 i32::MAX 必须在 adapter/DTO 入库前拒绝 |
| operational_recipient_ref | text | 否 | 无 | canonical operational recipient 引用；非空且不得等于 recovery recipient |
| recovery_wrapped_key | bytea | 否 | 无 | 同一 DEK 面向离线 recovery recipient 的独立认证信封；必须正长度 |
| recovery_wrap_key_version | int | 否 | 无 | recovery 包裹密钥版本，范围 1..2,147,483,647；Rust u32 超出 i32::MAX 必须在 adapter/DTO 入库前拒绝 |
| recovery_recipient_ref | text | 否 | 无 | canonical 离线恢复 recipient 引用；非空且不得等于 operational recipient |
| wrap_context_generation | int | 否 | 无 | 两份信封共同绑定的签名 wrap-context generation，范围 1..2,147,483,647 |
| wrap_envelope_version | int | 否 | 无 | 双 recipient 信封协议版本，范围 1..2,147,483,647 |
| state | text | 否 | 无 | `ACTIVE`、`RETIRING`、`RETIRED`、`DESTROYED` 四态 |
| activated_at | timestamptz | 否 | 无 | 生效时间 |
| retiring_at、retired_at、destroyed_at | timestamptz | 是 | 无 | 轮换与销毁时点 |

`ck_data_keys_state_shape` 精确强制：ACTIVE 时 `retiring_at/retired_at/destroyed_at` 全空；RETIRING 时仅 `retiring_at` 非空；RETIRED 时 `retiring_at/retired_at` 非空而 `destroyed_at` 空；DESTROYED 时三者全非空。所有存在的时间必须满足 `activated_at<=retiring_at<=retired_at<=destroyed_at` 的相应前缀。另以行内 CHECK 强制 `version between 1 and 65535`；两个 `*_wrap_key_version`、`wrap_context_generation`、`wrap_envelope_version` 都在 `1..=2147483647`；两份 `*_wrapped_key` 都满足 `octet_length(...)>0`；两个 recipient ref 非空、canonical 且逐字不同。purpose 继续与 algorithm 联合约束，state 继续与时间/信封完整形状联合约束，不能只分别检查枚举。WrappedDataKey、Readback、DataKeyHandle、DataKeyRef 与 generate 参数统一用非零 u16；canonical ref 的版本是无前导零十进制。某 tuple 当前 data-key version 已为 65535 时轮换固定返回 `PLATFORM.KEY_DOMAIN.TRANSITION_INVALID`，不得加一溢出、回绕到 0 或另造负数。KEK、wrap KEK 与两个 generation/version 的 Rust 值可为 u32，但持久边界统一 1..=i32::MAX，超界在 SQL cast 前拒绝。不得接受半套时间、空信封、相同 recipient 或单信封行。

`recovery_wrapped_key` 的 bytes 进一步必须是 [ADR-0020](adr/ADR-0020-dual-recipient-data-key-recovery.md) §§6–8 的 strict JCS `DataKeyRecoveryEnvelopeV1`，不是不透明 provider blob。payload 的 deployment/legal-entity/domain/data-key/version/purpose/scope、三个 wrap/generation 版本和 `recovery_recipient_ref` 必须与本行逐字相等；recipient-set digest、2-of-3 share、PIV certificate state、predecessor/current rotation chain和 envelope digest由 repository/parser 在写入、readback、激活与恢复前重验。数据库长度/CHECK 只是第一道形状门，不能替代 cryptographic/AAD/share verification。

`operational_wrapped_key` 的 bytes 同样必须是 [ADR-0020](adr/ADR-0020-dual-recipient-data-key-recovery.md) §6 的无 BOM、无尾随换行、最多 65,536 bytes 的 strict RFC 8785 JCS `OperationalDataKeyEnvelopeV1`，root exact fields、profile、provider-manifest/key-identity digest、12-byte nonce、32-byte ciphertext、16-byte tag、AAD 前像与 predecessor chain 全部按该 ADR；payload 的 deployment/legal-entity/domain/data-key/version/purpose/scope、`operational_wrap_key_version`、`operational_recipient_ref`、`wrap_context_generation` 和 `wrap_envelope_version` 必须与本行逐字相等。repository/parser 在写入、readback、激活、轮换和恢复重包裹前重验 canonical bytes、AEAD、provider identity、nonce 唯一性与版本连续性；数据库的正长度 CHECK 不能把它降级成不透明 blob。

`ck_data_keys_purpose_algorithm` 固定映射：`FIELD|ATTACHMENT|ARCHIVE` 只能用 `AES_256_GCM`，`BLIND_INDEX` 只能用 `HMAC_SHA256`；不能只分别检查两个枚举闭集。唯一约束 `ux_data_keys_domain_purpose_scope_version` 在 `(key_domain_id, purpose, security_level_scope, version)` 四列上：该名 50 字节未达 63 字节标识符上限，按全称保留；其列序全称形态 `ux_data_keys_key_domain_id_purpose_security_level_scope_version` 因超限不采用，此处登记备查。首版不使用部分索引，取当前有效密钥按该 ux 前缀定位后 `order by version desc limit 1`。

DEFERRABLE INITIALLY DEFERRED constraint trigger 同时附着 `key_domains/data_keys`，COMMIT 时强制每个 `(key_domain_id,purpose,security_level_scope)` 至多一条 ACTIVE；任一 domain 处于 ACTIVE 时，ACTIVE data key 必须恰为四个 purpose × `10|20|30|40` 四个 scope 的笛卡尔积 16 行，零缺失、零额外、每 tuple 恰一条。PROVISIONING 可暂时没有 ACTIVE data key，但不能转 ACTIVE 后再补矩阵。

外部 KMS/HSM 供给不塞入 PostgreSQL 事务。独立端口 `KmsKeyMaterialProvisioner` 以本行 logical locator 保证 KEK，逐 tuple 只生成一次 detached DEK，然后分别为 operational 与 recovery recipient 生成并认证两份信封；任一正确路径都能恢复同一 DEK，不要求两域同时在线。正常 readiness 只经 operational 路径 readback，离线 recovery 只经 [ADR-0020](adr/ADR-0020-dual-recipient-data-key-recovery.md) 的 `PIV_SHAMIR_2_OF_3_V1` 仪式（固定 3 份 share、任意 2 份重构）并在洁净主机重包裹给新 operational recipient，运行服务不能调用 recovery。两信封各自绑定 deployment、legal entity、purpose、data-key id/version、wrap-context generation、recipient 与 envelope version。DEK 的唯一耐久形态是本表双信封与绑定元数据，不得以明文、文件、环境变量或另一 provider 私有 locator 持久化。

首装对四 purpose × 四 scope 共 16 个 tuple 的双信封全部生成并完成 operational readback 后，才在一个数据库事务插入 16 条 ACTIVE data key、把 domain 从 PROVISIONING 推进 ACTIVE，并以 audit terminal batch 写唯一 `action='platform.key_domain.activated.v1'`；完整 envelope 还固定 `event_id` 为本事务 terminal batch 前预分配的新 UUIDv7，`legal_entity_id=key_domains.legal_entity_id`，`actor_user_id=SYSTEM_PRINCIPAL_ID` 且命中该法人的 ACTIVE SYSTEM 法人授权，`actor_device_id=null`，`object_type='platform.key_domains'`，`object_id=key_domain_id`，`object_version=ACTIVE row_version`，`before=null`，`after=上述 payload`，`reason/approval_ref/reauth_ref=null`，`client='system'`，`occurred_at=activated_at`；`event_day/seq/prev_hash/hash` 只由 AuditWriter 既有分段链算法派生。payload 是 exact 闭集 `{schema_version:1,deployment_id,key_domain_id,legal_entity_id,activation_source,bootstrap_id,kek_ref,kek_version,kek_provider_fingerprint_sha256,data_keys,activated_at}`；`activation_source` 仅 `STANDARD|INITIAL_GOVERNANCE`，前者要求 `bootstrap_id=null`，后者要求 signed bootstrap UUID。`data_keys` 恰 16 项，每项 exact 为 `{data_key_id,purpose,security_level_scope,version,algorithm,operational_wrap_key_version,operational_recipient_ref,operational_wrapped_key_sha256,recovery_wrap_key_version,recovery_recipient_ref,recovery_wrapped_key_sha256,wrap_context_generation,wrap_envelope_version}`，按 purpose wire `FIELD,BLIND_INDEX,ATTACHMENT,ARCHIVE` 再按 scope `10,20,30,40` 排序；三个 SHA-256 wire 都是 64 lowerhex，`activated_at` 为 UTC whole-second。同一事务的 16 行、KEK locator/version/fingerprint、双信封绑定、PROVISIONING→ACTIVE 与审计 payload 必须逐项相等，审计是最后一批写入；缺项、额外项、错序、错 source/bootstrap 形状均整笔回滚并返回 `PLATFORM.KEY_DOMAIN.KEY_UNAVAILABLE`。A-04 轮换每次只选一个 purpose，但同批生成、双包裹、operational readback 并切换该 purpose 的四个 scope；不得只轮换一个 scope、跨 purpose 拼批或让同一 tuple 同时有两条 ACTIVE。

### 6.4 sensitive_field_registry（敏感字段登记表）

不带 `legal_entity_id`、不建策略，登记于表十三。业务列集按裁定 C-06 冻结为十一列，`approved_by`、`approved_at` 两列已撤销，批准留痕由 `release_ref` 承载。

| 列名 | 类型 | 可空 | 默认 | 语义 |
|---|---|---|---|---|
| schema_name、table_name、column_name | text | 否 | 无 | 受保护列三元组，三列联合唯一；`column_name` 是逻辑列名不含 `_enc` 后缀 |
| category | text | 否 | 无 | `IDENTITY`、`CONTACT`、`ACCOUNT`、`TAX_ID`、`PAYMENT_TOKEN`、`LEGAL`、`HEALTH` |
| security_level | smallint | 否 | 无 | 该列密级，取 10、20、30、40；未赋值按所属对象取值的规则由 `effective_level()` 承载 |
| is_field_encrypted | boolean | 否 | false | 物理列是否为信封密文；取真时物理列集为 `<列名>_enc bytea` 与 `<列名>_key_ref text`，需要掩码尾数再加 `<列名>_tail`，需要查重再加 `<列名>_bidx`，不保留同名明文列，由 `db/checks/11` 断言 |
| blind_index | text | 否 | 'NONE' | 首版只放行 `NONE` 与 `EXACT`，`PREFIX` 预留不放行 |
| blind_index_column | text | 是 | 无 | 盲索引列名；`NONE` 时为空 |
| mask_style | text | 否 | 'NONE' | 掩码样式，取值语义由阶段 4 解释 |
| normalization | text | 否 | 'TRIM_NFKC' | 取 `NONE`、`TRIM_NFKC`、`TRIM_NFKC_LOWER`、`DIGITS_ONLY` |
| release_ref | text | 否 | 无 | 批准留痕与登记来源：经迁移取 `MIGRATION:<版本号>`，经端点取 `ENDPOINT:<审批记录号>` |

`blind_index = 'EXACT'` 的登记行对应的物理 `<列名>_bidx` 一律存 `BlindIndex([u8; 32])`，并必须带固定命名的 `CHECK (<列名>_bidx IS NULL OR octet_length(<列名>_bidx) = 32)`；`db/checks/11` 同时核验列存在、类型为 `bytea` 和长度 CHECK。宽度不是配置项，也不允许按字段例外。是否另建唯一约束只由该业务字段的唯一性规则决定。

`BlindIndexService::compute` 的 selector 必须是含密级的 canonical `<schema>.<table>.<logical-column>@<10|20|30|40>`，首版四个 ACCOUNT/PAYMENT token 精确为 `platform_msg.push_registrations.token@30`、`mdm.customer_invoice_profiles.bank_account_no@30`、`mdm.supplier_payment_profiles.bank_account_no@30`、`finance.cash_accounts.bank_account_no@30`；裸 FQN、错 scope、物理 `_enc/_bidx` 列名或别名一律拒绝，不能由调用者默认补 `@30`。

阶段 3 高保密审批命令快照登记一行：`schema_name=platform_flow`、`table_name=approval_command_snapshots`、逻辑 `column_name=command`、`category=LEGAL`、`security_level=30`、`is_field_encrypted=true`、`blind_index=NONE`、`blind_index_column=NULL`、`mask_style=FULL`、`normalization=NONE`、`release_ref=MIGRATION:20261013093700`。物理形态固定为 `command_enc bytea + command_key_ref text`，不得存在 `command` 明文列或 `command_bidx`；`command_digest` 只校验密文与 AAD 完整性，不承担查询、查重或明文等值比较。审批命令执行结果由同表 `result_object_type text/result_object_id uuid/result_doc_no text` 定位：仅 CONSUMED 在业务对象同一事务写 type/id 与可空 doc_no，其他三态全空且终态不可改；完整列集与状态 CHECK 见 [platform_flow 数据字典](data-dictionary/platform_flow.md)。

### 6.5 append_only_registry（仅追加登记表）

不带 `legal_entity_id`、不建策略，登记于表十三。

| 列名 | 类型 | 可空 | 默认 | 语义 |
|---|---|---|---|---|
| schema_name、table_name | text | 否 | 无 | 受管表二元组，联合唯一 |
| mode | text | 否 | 无 | `APPEND_ONLY` 或 `IMMUTABLE_COLUMNS` |
| mutable_columns | text[] | 否 | '{}' | 仅 `IMMUTABLE_COLUMNS` 使用；`APPEND_ONLY` 时必须为空（CHECK 强制） |

### 6.6 migration_windows（迁移窗口台账）

不带 `legal_entity_id`、不建策略，登记于表十三。同一时刻至多一个 `OPEN` 窗口，由对附带单例锁表 `migration_window_lock(id smallint primary key check (id = 1))` 的 `SELECT ... FOR UPDATE` 串行化；锁表不带公共列，登记于表十三并豁免 `db/checks/01`。

| 列名 | 类型 | 可空 | 默认 | 语义 |
|---|---|---|---|---|
| state | text | 否 | 无 | `OPEN`、`CLOSED` |
| approval_ref | text | 否 | 无 | 双人审批引用，缺失即不可开窗 |
| reason | text | 否 | 无 | 开窗理由，长度不超过 2000 |
| opened_by、opened_at | uuid、timestamptz | 否 | 无 | 开窗人与开窗时间 |
| expires_at | timestamptz | 否 | 无 | 到期时间，CHECK 要求晚于 opened_at |
| closed_by、closed_at | uuid、timestamptz | 是 | 无 | 关窗人与关窗时间 |
| close_kind | text | 是 | 无 | `MANUAL`、`EXPIRED`、`FAILED` |
| applied_versions | text[] | 否 | '{}' | 本窗口实际应用的迁移版本 |

### 6.7 enterprise_groups（集团表，档案类）

不带 `legal_entity_id`、不建策略，登记于表十三。自有列：`code`（长度 1 至 64，全库唯一）、`name`（长度 1 至 200）、`is_active`（默认 true）、`deactivated_at`（可空）。`legal_entities.group_id` 指向本表。

### 6.8 organizations（组织表，档案类）

带 `legal_entity_id`，策略 `rls_organizations_le`。自有列：`code`（法人内唯一）、`name`（长度 1 至 200）、`org_kind`（`CORPORATION`、`BRANCH`、`DIVISION`）、`parent_organization_id`（可空，自引用外键 ON DELETE RESTRICT）、`is_active`（默认 true，停用态直接删列 `deactivated_at` 不适用，组织停用按档案口径以 `is_active` 表达）。

### 6.9 departments（部门表，档案类）

带 `legal_entity_id`，策略 `rls_departments_le`。自有列：`organization_id`（不可空，外键指向 organizations）、`code`（法人内唯一）、`name`（长度 1 至 200）、`parent_department_id`（可空，自引用外键）、`level_no`（smallint，大于 0，与闭包表同事务维护）、`is_active`、`deactivated_at`。阶段 4 的 `department_id` 外键目标即本表。

### 6.10 positions（岗位表，档案类）

带 `legal_entity_id`，策略 `rls_positions_le`。自有列：`department_id`（不可空，外键指向 departments）、`code`（法人内唯一）、`name`（长度 1 至 200）、`rank_no`（smallint，大于 0）、`is_active`、`deactivated_at`。阶段 4 的 `position_id` 外键目标即本表。

### 6.11 department_closures（部门层级闭包表）

带 `legal_entity_id`，策略 `rls_department_closures_le`。自有列：`ancestor_department_id`、`descendant_department_id`（均不可空，外键指向 departments，ON DELETE RESTRICT）、`depth`（smallint，不小于 0；自环行取 0）。唯一约束 `ux_department_closures_pair` 在两个部门列上，按裁定 A-04 冻结不得改写。索引 `ix_department_closures_le_id_descendant_id` 按基线第 3.8 节缩写规则命名，全称为 `ix_department_closures_legal_entity_id_descendant_department_id`，此处登记全称备查。

### 6.12 unpoliced_table_registry（未受行级策略表登记）

不带 `legal_entity_id`、不建策略，登记本表自身。自有列：`schema_name`、`table_name`（联合唯一）、`admission_basis`（取 `SAME_FOR_ALL_ENTITIES` 与 `ISOLATION_OR_DEPLOYMENT_METADATA` 两值）、`isolation_entry`（法人可见性所落的应用层入口，长度 1 至 200）、`matrix_case_id`（`tests/rls_matrix` 用例标识）。阶段 2 登记八行，见 02 计划表十三。

### 6.13 user_accounts（员工账号目录，档案类，阶段 4）

不带 `legal_entity_id`、不建策略，登记于表十三。`home_legal_entity_id` 只用于审计事件分段与默认法人，不参与访问判定；法人可见性由 `platform_authz.user_legal_entity_grants` 内联承担（04 计划第 12.2 节偏离一）。`security_level` 默认 30。

| 列名 | 类型 | 可空 | 默认 | 语义 |
|---|---|---|---|---|
| account_kind | text | 否 | 无 | `EMPLOYEE`、`PORTAL`、`BREAKGLASS`、`SYSTEM` 四类 |
| login_name | text | 否 | 无 | 登录名，长度 1 至 64，全库唯一 |
| employee_no | text | 是 | 无 | 工号，长度 1 至 64，全库唯一索引对多个 NULL 不生效 |
| display_name | text | 否 | 无 | 显示名，长度 1 至 200 |
| home_legal_entity_id | uuid | 否 | 无 | 归属法人，只作审计分段与默认法人；真实单列外键指向 `platform_core.legal_entities(id) ON DELETE RESTRICT` |
| clearance_level | smallint | 否 | 20 | 账号自身许可等级，取 10、20、30、40 |
| status | text | 否 | 无 | `UNACTIVATED`、`ACTIVE`、`LOCKED`、`SUSPENDED`、`DEACTIVATED` 五态 |
| is_mfa_required | boolean | 否 | false | 是否强制多因子 |
| activated_on | date | 是 | 无 | 启用日；停用取 `deactivated_at` |
| last_login_at | timestamptz | 是 | 无 | 最近登录时刻 |

时间序索引按偏离三取 `ix_user_accounts_status_created_at`（本表无 `legal_entity_id` 列）。本表的 `created_by/updated_by` 是无法人身份元数据证据，以真实单列自外键指向 `user_accounts(id) ON DELETE RESTRICT`；种入首个 `SYSTEM_PRINCIPAL_ID` 时由追补迁移使用受控引导顺序，最终约束验证后不留孤儿。最终目标形状不含历史 `supplier_ref_id`；该列由 `V20261012115000__platform_core_drop_user_accounts_supplier_ref_id.sql` 删除，供应商门户身份的唯一业务绑定是 `portal.supplier_portal_users`，不得在账号表兼容回填或保存第二份映射。

### 6.14 user_credentials（认证凭据，阶段 4）

不带 `legal_entity_id`、不建策略，登记于表十三。`security_level` 默认 40。`user_id` 以真实单列外键指向 `user_accounts(id) ON DELETE RESTRICT`；公共 `created_by/updated_by` 亦为真实单列账号外键。

| 列名 | 类型 | 可空 | 默认 | 语义 |
|---|---|---|---|---|
| credential_kind | text | 否 | 无 | `PASSWORD`、`TOTP`、`WEBAUTHN_PLATFORM`、`WEBAUTHN_ROAMING`、`X509_CERT` 五类 |
| verifier | text | 是 | 无 | PASSWORD 存 Argon2id 的 PHC 串；X509_CERT 存证书指纹 |
| public_key、credential_handle | bytea | 是 | 无 | WebAuthn 两类凭据的公钥与凭据句柄；`credential_handle` 全库唯一 |
| secret_enc | bytea | 是 | 无 | 仅 TOTP：种子的 EPC1 密文；不存明文、文件、环境变量或外部 object ref |
| secret_key_ref | text | 是 | 无 | 仅 TOTP：EPC1 canonical data-key id/version 冗余投影，exact `data-key://<lowercase-data-key-uuid>#<u16非零无前导零版本>`，只用于完整性核对、不参与选钥 |
| last_used_counter | bigint | 是 | 无 | 仅 TOTP：enrollment/首次验证前为空，首次成功后非负且只允许严格单调增加，用于拒绝同一 time-step 重放 |
| sign_count | bigint | 否 | 0 | WebAuthn 签名计数 |
| status | text | 否 | 无 | `ACTIVE`、`SUSPENDED`、`REVOKED`、`EXPIRED` 四态 |
| activated_at | timestamptz | 否 | now() | 生效时刻 |
| expires_at、last_used_at、revoked_at | timestamptz | 是 | 无 | 到期、最近使用与吊销时点 |

`ck_user_credentials_material` 是 exact one-of：PASSWORD 只允许 verifier 非空；X509_CERT 只允许 verifier 与 credential_handle 非空；两类 WebAuthn 只允许 public_key 与 credential_handle 非空；TOTP 只允许 `secret_enc/secret_key_ref` 同时非空，`last_used_counter` 可空或非负；非 TOTP 的这三列必须全空。TOTP credential id 必须在生成种子前预分配，purpose 固定 `KeyPurpose::Field(L40)`，唯一 pseudo-column AAD 为 `Aad::for_field(legal_entity_id,"platform_core.user_credentials.totp_secret",credential_id,SecurityLevel::L40)`；写入与验证直接用 `KmsBackend::wrap/unwrap` 处理 EPC1，禁止 `secret://`、外部 SecretProvider、把 `data_keys` 的任一 DEK 信封或明文 DEK 带入 identity crate。验证成功与 `last_used_counter` 的 `NULL→counter` 或 `old<counter` 条件更新在同一事务，影响零行即按重放拒绝，绝不回退或复用 counter。

### 6.15 user_password_history（口令历史，仅追加，阶段 4）

不带 `legal_entity_id`、不建策略，登记于表十三，并登记于 `append_only_registry`（mode 取 `APPEND_ONLY`，不带 row_version 与 updated_* 两对列）。`security_level` 默认 40。自有列：`user_id`（不可空，真实单列外键指向 `user_accounts(id) ON DELETE RESTRICT`）、`verifier`（不可空，历史口令的 Argon2id PHC 串）。`created_by` 同样指向账号主键；索引 `ix_user_password_history_user_id_created_at`。

### 6.16 user_devices（设备登记，阶段 4）

不带 `legal_entity_id`、不建策略，登记于表十三。`security_level` 默认 30。

| 列名 | 类型 | 可空 | 默认 | 语义 |
|---|---|---|---|---|
| user_id | uuid | 否 | 无 | 所属用户；真实单列外键指向 `user_accounts(id) ON DELETE RESTRICT` |
| device_id | text | 否 | 无 | 设备标识，长度 1 至 64，全库唯一 |
| client | text | 否 | 无 | 六端取值 `win`、`mac`、`ios`、`android`、`portal`、`ops` |
| public_key | bytea | 是 | 无 | 设备密钥（WebAuthn 形态） |
| attestation_ref | text | 是 | 无 | 核验证明引用 |
| restricted_legal_entity_id | uuid | 是 | 无 | 受限法人；非空时真实单列外键指向 `legal_entities(id) ON DELETE RESTRICT`，认证中间件再与用户授权集合取交集 |
| status | text | 否 | 无 | F-57 current exact `PENDING`、`COMPLIANT`、`RESTRICTED`、`REVOKED`；旧 `ACTIVE` 只在 93010 迁移输入出现并一次性映射为 COMPLIANT，之后拒绝 |
| device_epoch | bigint | 否 | 1 | 正整数；attestation 恢复、限制或撤销所需 credential rotation 按状态命令单调增加，会话必须逐请求匹配 |
| attestation_policy_id | text | 是 | 无 | 最近一次受信设备策略；PENDING 可空，COMPLIANT 必填且命中 current signed policy |
| attestation_digest | bytea | 是 | 无 | 最近 exact attestation evidence 的 32-byte SHA-256；COMPLIANT 必填 |
| attested_at | timestamptz | 是 | 无 | 最近受信 attestation 时间；COMPLIANT 必填 |
| restriction_reason | text | 是 | 无 | RESTRICTED 必填的闭合原因；其他状态必须为空，永久原因只能进 REVOKED |
| state_changed_at | timestamptz | 否 | now() | 最近合法状态边的可信时间；与 append-only transition 行逐字对应 |
| registered_at | timestamptz | 否 | now() | 登记时刻 |
| revoked_at、last_seen_at | timestamptz | 是 | 无 | 吊销与最近出现时点 |

公共 `created_by/updated_by` 为真实单列账号外键。状态边只有 `PENDING→COMPLIANT|RESTRICTED|REVOKED`、`COMPLIANT→RESTRICTED|REVOKED`、`RESTRICTED→COMPLIANT|REVOKED`，REVOKED 终态；每边写 append-only attestation transition。设备行本身不证明某法人授权；引用设备的会话或业务证据必须在持锁事务核对 `user_id`、`device_epoch`、COMPLIANT 门和法人授权。PortalDevice 使用 portal schema 的独立状态机，不能写入本表。

### 6.17 sessions（会话，阶段 4）

不带 `legal_entity_id`、不建策略，登记于表十三。`security_level` 默认 30。令牌只存 SHA-256 摘要，明文不落库也不进日志。

| 列名 | 类型 | 可空 | 默认 | 语义 |
|---|---|---|---|---|
| user_id | uuid | 否 | 无 | 所属用户；真实单列外键指向 `user_accounts(id) ON DELETE RESTRICT` |
| user_device_row_id | uuid | 否 | 无 | 真实单列外键指向 `user_devices(id) ON DELETE RESTRICT` |
| token_hash | bytea | 否 | 无 | 令牌 SHA-256 摘要，全库唯一 |
| active_legal_entity_id | uuid | 否 | 无 | 本会话活动法人；真实单列外键指向 `legal_entities(id) ON DELETE RESTRICT` |
| client | text | 否 | 无 | 六端取值，同 user_devices |
| issued_at | timestamptz | 否 | now() | 签发时刻 |
| expires_at | timestamptz | 否 | 无 | 绝对到期；滑动续期合并事务只刷新下行两列 |
| idle_expires_at | timestamptz | 否 | 无 | 空闲到期，续期写入 now + 空闲超时 |
| last_seen_at | timestamptz | 否 | now() | 最近核验时刻 |
| revoked_at | timestamptz | 是 | 无 | 撤销时刻 |
| revoke_reason | text | 是 | 无 | 撤销理由，长度不超过 128 |
| is_breakglass | boolean | 否 | false | 是否应急账号会话，供 `ep_breakglass_active_sessions` 分计 |

索引：`ix_sessions_user_id_expires_at`、`ix_sessions_last_seen_at`、时间序 `ix_sessions_created_at`。公共 `created_by/updated_by` 为真实单列账号外键；创建或续期事务必须证明设备行的 `user_id` 等于会话用户且该用户对活动法人存在有效授权，单列 FK 不替代归属校验。

### 6.18 reauth_challenges（登录 MFA 与高风险复核挑战，阶段 4）

不带 `legal_entity_id`、不建策略，登记于表十三。历史表名保留，但 `challenge_kind` 明确区分登录 MFA 与高风险复核；`security_level` 默认 30。摘要由服务端按规范化算法重算，不采信客户端传值（04 计划第 4.4 节）。

| 列名 | 类型 | 可空 | 默认 | 语义 |
|---|---|---|---|---|
| challenge_kind | text | 否 | 无 | `SIGN_IN_MFA`、`HIGH_RISK_REAUTH` |
| user_id | uuid | 否 | 无 | 挑战主体；真实单列外键指向 `user_accounts(id) ON DELETE RESTRICT` |
| session_id | uuid | 是 | 无 | 高风险复核必填并真实单列外键指向 `sessions(id) ON DELETE RESTRICT`；登录 MFA 必须为空 |
| user_device_row_id | uuid | 是 | 无 | 登录 MFA 必填并真实单列外键指向 `user_devices(id) ON DELETE RESTRICT`；高风险复核必须为空 |
| default_legal_entity_id | uuid | 是 | 无 | 登录 MFA 必填并真实单列外键指向 `legal_entities(id) ON DELETE RESTRICT`；高风险复核必须为空 |
| operation_type | text | 是 | 无 | 高风险复核必填；七类取值恰为 `CONTRACT_EFFECTIVE`、`PAYMENT`、`INVOICE_ISSUE`、`LEDGER_POSTING`、`PERIOD_CLOSE`、`SENSITIVE_EXPORT`、`DATA_MIGRATION`；其中 `DATA_MIGRATION` 的 subject 与批准事实只按阶段 14 专用契约形成；登录 MFA 必须为空 |
| subject_digest | bytea | 否 | 无 | 待签内容 SHA-256 摘要 |
| subject_summary | jsonb | 否 | 无 | 与 `subject_digest` 同源、由服务端重算且敏感字段已掩码的规范化摘要（审计展示用）：`SIGN_IN_MFA` 按登录 MFA 契约；前六类业务高风险按操作类型、法人 ID、单据编号、关键金额或会计期间、生效影响五项；`DATA_MIGRATION` 恰按 `operation_type`、`legal_entity_id`、`batch_id`、`known_difference_id`、`reauth_purpose`、`content_version`、`content_hash` 七键 JCS 形状，不得套用五项业务摘要 |
| nonce | bytea | 否 | 无 | 防重放随机数 |
| credential_kind_used | text | 是 | 无 | 验证时使用的凭据种类 |
| status | text | 否 | 无 | `ISSUED`、`VERIFIED`、`CONSUMED`、`FAILED`、`EXPIRED`、`ABANDONED` 六态 |
| token_hash | bytea | 否 | 无 | 32 字节随机不透明挑战/复核令牌的 SHA-256，原文不入库，全库唯一，一次性消费 |
| issued_at、expires_at | timestamptz | 否 | 无 | 签发与到期 |
| verified_at、consumed_at | timestamptz | 是 | 无 | 验证与核销时点 |
| failure_count | int | 否 | 0 | 验证失败次数，不小于 0 |

NULL-safe CHECK 强制两类字段组合：SIGN_IN_MFA 从 ISSUED 直接到 CONSUMED并创建会话，HIGH_RISK_REAUTH 按 ISSUED→VERIFIED→CONSUMED。公共 `created_by/updated_by` 为真实单列账号外键；消费事务必须校验 session/device 的 `user_id` 与挑战主体相同，并校验目标法人授权。索引：`ux_reauth_challenges_token_hash`、`ix_reauth_challenges_user_id_status_expires_at`、时间序 `ix_reauth_challenges_created_at`。

### 6.19 login_attempts（登录尝试，仅追加，阶段 4）

不带 `legal_entity_id`、不建策略，登记于表十三，并登记于 `append_only_registry`（mode 取 `APPEND_ONLY`，不带 row_version 与 updated_* 两对列）。`security_level` 默认 30。自有列：`user_id`（可空；非空时真实单列外键指向 `user_accounts(id) ON DELETE RESTRICT`，账号不存在时只有哈希）、`login_name_hash`（bytea，哈希存储防攻击者注入明文）、`outcome`（八值 `SUCCESS`、`CREDENTIAL_INVALID`、`ACCOUNT_LOCKED`、`ACCOUNT_INACTIVE`、`MFA_REQUIRED`、`MFA_INVALID`、`DEVICE_UNREGISTERED`、`RATE_LIMITED`，不存在旧值 `ADMISSION_REJECTED`）、`client`（可空）、`source_addr`（可空，长度不超过 64）、`occurred_at`。`created_by` 为真实单列账号外键；索引 `ix_login_attempts_occurred_at`（限流与清理）与 `ix_login_attempts_user_id_occurred_at`（锁定窗口判定）。

### 6.20 account_lockouts（锁定窗口，阶段 4）

不带 `legal_entity_id`、不建策略，登记于表十三。`security_level` 默认 30。自有列：`user_id`（真实单列外键指向 `user_accounts(id) ON DELETE RESTRICT`；唯一约束 `ux_account_lockouts_user_id`，一人至多一行）、`failure_count`（int 不小于 0，默认 0）、`window_started_at`（默认 now()）、`locked_until`（可空，取非空即处于锁定）、`last_failure_at`（可空）。公共 `created_by/updated_by` 为真实单列账号外键；索引 `ix_account_lockouts_locked_until` 供到期解锁清理扫描。

### 6.21 breakglass_activations（应急账号启用，单据类，阶段 4）

不带 `legal_entity_id`、不建策略，登记于表十三。`security_level` 默认 40。`doc_no` 类型码 `BGA`，全库唯一（本表无 `legal_entity_id` 列，唯一约束不带法人）；该类型码在 §5.1 的登记随编号生成本体（属 3b 同批）一并补齐。

| 列名 | 类型 | 可空 | 默认 | 语义 |
|---|---|---|---|---|
| status | text | 否 | 无 | `DRAFT`、`PENDING_APPROVAL`、`APPROVED`、`ACTIVE`、`EXPIRED`、`CLOSED`、`REJECTED` 七态 |
| user_id | uuid | 否 | 无 | 被启用的应急账号；真实单列外键指向 `user_accounts(id) ON DELETE RESTRICT` |
| requested_by、approved_by | uuid | 否、是 | 无 | 申请人与批准人；各自为真实单列账号外键，批准人非空时不得等于申请人 |
| reason | text | 否 | 无 | 启用理由，长度 1 至 2000 |
| approval_ref | text | 是 | 无 | 审批引用 |
| allowed_action_set | text[] | 否 | 无 | 非空子集，取 `UNLOCK_OR_RESET_ADMIN`、`RESTORE_CONTROLLED_CONFIG_RELEASE`、`TRIGGER_BACKUP_OR_RESTORE` 三值 |
| activated_at、expires_at、closed_at | timestamptz | 是 | 无 | 启用、到期与关闭时点 |
| rotated_at | timestamptz | 是 | 无 | 关闭同事务内凭据轮换完成时点（退出条件 14） |
| rotation_result | text | 是 | 无 | 轮换结果 |

公共 `created_by/updated_by` 为真实单列账号外键。索引 `ix_breakglass_activations_status_expires_at` 供到期失效扫描，时间序取 `ix_breakglass_activations_created_at`。

### 6.22 impact_assessments（影响面评估批次，阶段 3）

带 `legal_entity_id`、`security_level`、`data_scope_tags` 与可更新公共列，ENABLE、FORCE RLS，策略 `rls_impact_assessments_le`。

| 列名 | 类型 | 可空 | 默认 | 语义 |
|---|---|---|---|---|
| source_module | text | 否 | 无 | 来源模块，取 foundation `ModuleCode` 序列化值 |
| source_doc_id | uuid | 否 | 无 | 来源业务对象 id |
| source_doc_version | bigint | 否 | 无 | 来源版本，大于 0 |
| source_event_id | uuid | 否 | 无 | 触发事件 id；同法人唯一 |
| source_event_type | text | 否 | 无 | 首版只允许 `clm.contract.terminated.v1` |
| reason | text | 否 | 无 | 清洗后的来源动作理由，长度 1 至 2000 |
| status | text | 否 | 无 | `RUNNING\|DONE\|FAILED` |
| item_total | int | 否 | 0 | 目录占位与真实目标项目总数，非负 |
| item_done | int | 否 | 0 | DONE 数，非负且不大于总数 |
| item_dead | int | 否 | 0 | DEAD 数，非负且不大于总数 |
| started_at | timestamptz | 否 | 无 | 批次建立时间 |
| finished_at | timestamptz | 是 | 无 | 仅 DONE 非空 |
| last_error_code | text | 是 | 无 | 已登记稳定错误码，不存底层异常正文 |

CHECK：RUNNING 的 `finished_at` 为空；DONE 必须 `finished_at` 非空、`item_done=item_total`、`item_dead=0`；FAILED 必须 `finished_at` 为空、`item_dead>0`。唯一约束：`(legal_entity_id,source_module,source_doc_id,source_doc_version,source_event_type)` 与 `(legal_entity_id,source_event_id)`；索引 `ix_impact_assessments_le_status_started_id`、`ix_impact_assessments_le_source_doc`。

### 6.23 impact_disposition_items（影响面逐目标处置项，阶段 3）

带 `legal_entity_id`、`security_level`、`data_scope_tags` 与可更新公共列，ENABLE、FORCE RLS，策略 `rls_impact_disposition_items_le`。

| 列名 | 类型 | 可空 | 默认 | 语义 |
|---|---|---|---|---|
| impact_assessment_id | uuid | 否 | 无 | 同 schema 复合外键 `(legal_entity_id,id)` 指向批次，ON DELETE RESTRICT |
| impact_rule_code | text | 否 | 无 | `docs/impact-catalog.md` 七码之一 |
| target_module | text | 否 | 无 | 目标模块 `ModuleCode` |
| target_doc_id | uuid | 是 | 无 | 未接线占位及 `NO_APPLICABLE_TARGET` 目录终态为空，真实目标项目必填 |
| target_doc_no | text | 是 | 无 | 目标为空的目录项为空；仅展示，不作授权依据 |
| target_doc_line_no | int | 是 | 无 | 非空时大于 0 |
| disposition_kind | text | 否 | 无 | `AUTO_CLOSE\|AUTO_CANCEL\|MANUAL_DECISION\|INFORM_ONLY` |
| state | text | 否 | 无 | `PENDING\|DISPATCHING\|DONE\|DEAD` |
| attempts | smallint | 否 | 0 | 0..9；首投失败为 1，第八次重试仍失败为 9 |
| available_at | timestamptz | 否 | now() | 下一次可领取时间 |
| locked_by、locked_until | text、timestamptz | 是 | 无 | 同空同非空；只允许 DISPATCHING 持有 |
| last_error_code、last_error | text | 是 | 无 | 稳定码与清洗后摘要 |
| process_task_id | uuid | 是 | 无 | 与法人组成真实复合外键 `(legal_entity_id,process_task_id) -> platform_flow.process_tasks(legal_entity_id,id) ON DELETE RESTRICT` |
| decision_code | text | 是 | 无 | 人工 DONE 时必填且必须属于本规则允许集 |
| decision_reason | text | 是 | 无 | 人工 DONE 时清洗后非空；不解析文本决定分支 |
| decision_result_doc_id | uuid | 是 | 无 | 按目录逐 decision code 必填或必空 |
| decided_by、decided_at | uuid、timestamptz | 是 | 无 | 人工 DONE 时必填 |
| outcome_reason | text | 是 | 无 | DONE 时非空稳定原因码 |

表级 CHECK：目标三字段全空的目录行只允许 `PENDING + outcome_reason NULL` 或 `DONE + outcome_reason='NO_APPLICABLE_TARGET'` 两种形状；两者 `attempts=0`，租约、错误、流程、decision 与决定人/时间字段全空。其他 DONE、DEAD、DISPATCHING 行必须 target id 非空。非 MANUAL_DECISION 的五个决策/决定字段全为空；人工 PENDING 时三个决策字段全为空；人工 DONE 时 code、reason、decided_by、decided_at、process_task_id 必填，result id 的对象形状由目录加业务规则同事务校验。DONE 必有 outcome_reason，DEAD 必有 last_error_code。唯一约束在 `(legal_entity_id,impact_assessment_id,impact_rule_code,coalesce(target_doc_id,impact_assessment_id),coalesce(target_doc_line_no,0))`；领取与批次索引为 `ix_impact_items_le_state_available_id`、`ix_impact_items_le_assessment_state_id`、`ix_impact_items_le_process_task_id`。

### 6.24 module_registrations（F-56 模块注册与当前签名包投影，阶段 3）

CRL-revoked signer 的 DISABLE 窄恢复路径除本次 recovery item 唯一 `platform.config_special.accepted.v1` 外，在同一 audit terminal batch 还必须恰写一条 append-only `action='MODULE_SIGNER_REVOKED_DISABLED'`。batch 前预分配两个互异的新 UUIDv7 event id，唯一链顺序为 recovery event 在前、accepted event 在后且为 batch 最后一条；same-byte 回放不分配、不追加、不重排。两事件共享冻结治理法人、同一次 execute 的受信 `SecurityContext.actor_user_id/actor_device_id/client` 与 `config_packages.approval_ref`，两者 `reason/reauth_ref` 均为 null。recovery event 完整 envelope 的其余列固定 `event_id=<第一枚 UUIDv7>`、`object_type='platform.module_registrations'`、`object_id=current row id`、`object_version=after.row_version`、`before=<下述完整 DTO>`、`after=<下段 recovery DTO>`、`occurred_at=disabled_at`；accepted event 使用第二枚 UUIDv7 且其余 envelope 逐字采用同名冻结。`event_day/seq/prev_hash/hash` 只由 AuditWriter 按该顺序派生。audit `before` 是更新前完整 strict DTO `{schema_version:1,purpose:"EP-F56-CURRENT-MODULE-PROJECTION-V1",id,module_code,display_name,row_version,install_state,package_id,package_code,package_version,package_payload_sha256,package_signature_cms_sha256,package_signer_subject,package_signed_at,module_contract_version,module_contract_sha256,min_platform_version,max_platform_version_exclusive,released_on,source_config_package_id,source_config_item_id,installed_at,state_changed_at,enabled_at,disabled_at,last_transition_reason}`，其 `schema_version` 是 JSON number `1`，SemVer=strict object/null，digest=64 lowerhex，time=UTC whole-second。

audit `after` 恰为 `{schema_version:1,purpose:"EP-MODULE-SIGNER-REVOKED-DISABLED-V1",module_code,previous_source_config_package_id,previous_source_config_item_id,recovery_config_package_id,recovery_config_item_id,before_projection_sha256,after_projection_sha256,disabled_at,reason_sha256}`，`schema_version` 也是 JSON number `1`。四 id 分别取锁内 before current source 与本次 RELEASED DISABLE item；`reason_sha256=SHA-256(ASCII("EP-MODULE-DISABLE-REASON-V1")||0x00||UTF-8(recovery reason))`。两摘要均使用 `SHA-256(ASCII("EP-F56-CURRENT-MODULE-PROJECTION-V1")||0x00||JCS(dto))`；before digest 从 audit.before 重算，after DTO 只能由 before 做 checked `row_version+1`、disabled 状态/时间、recovery reason 的唯一变换派生，必须保留 previous source 两列与旧 inner/package 投影，且 `state_changed_at=disabled_at=audit.occurred_at`、`last_transition_reason=recovery item reason`。Stage 14 只能从这条 action、hash chain、两 digest 与 accepted event 派生 peer；缺失、重复、溢出或任一 id/version/time/reason/digest 不等均不得 PASS。

部署级表，不带 `legal_entity_id`、`security_level`、`data_scope_tags`，不建 RLS；由 `V20261013093300` 登记进 `unpoliced_table_registry(admission_basis='SAME_FOR_ALL_ENTITIES')`。完整列集为下表加可更新公共列 `id,row_version,created_at,created_by,updated_at,updated_by`：

| 列名 | 类型 | 可空 | 默认 | 语义 |
|---|---|---|---|---|
| module_code | text | 否 | 无 | foundation 的 15 个 `ModuleCode` 之一，全表唯一 |
| display_name | text | 否 | 无 | 1..64 UTF-8 bytes |
| install_state | text | 否 | 无 | `NOT_INSTALLED\|INSTALLED_DISABLED\|INSTALLED_ENABLED` |
| installed_at | timestamptz | 是 | 无 | 首次 INSTALL 时点 |
| state_changed_at | timestamptz | 是 | 无 | 最近动作提交时点 |
| package_id | uuid | 是 | 无 | 内层 manifest `package_id` |
| package_code | text | 是 | 无 | 正则 `^[a-z][a-z0-9._-]{0,63}$` |
| package_version_major | int | 是 | 无 | 0..65535 |
| package_version_minor | int | 是 | 无 | 0..65535 |
| package_version_patch | int | 是 | 无 | 0..65535 |
| package_payload_sha256 | bytea | 是 | 无 | 32 raw bytes，`SHA-256(JCS(manifest))`；对应 JSON wire 只收 64 lowerhex |
| package_signature | bytea | 是 | 无 | detached CMS exact bytes，1..1,048,576 bytes |
| package_signer_subject | text | 是 | 无 | exact `spki-sha256:<64 lowerhex>`；显示 DN 只派生展示，不作身份依据 |
| package_signed_at | timestamptz | 是 | 无 | manifest `issued_at`，UTC 秒精度 |
| module_contract_version | int | 是 | 无 | 已安装态为 1..2,147,483,647；Rust 保留 u32，descriptor/product manifest/module package/parser 入库前 checked conversion，拒绝更大值且不得 cast/wrap |
| module_contract_sha256 | bytea | 是 | 无 | 32 raw bytes，等于唯一签名 `product-modules.v1.jcs` 中该模块的编译期契约摘要 |
| min_platform_version | text | 是 | 无 | canonical 三段 SemVer |
| max_platform_version_exclusive | text | 是 | 无 | 可空 canonical 三段 SemVer；非空时严格大于 min |
| released_on | date | 是 | 无 | 包发布日期；安装/升级时受当前许可维护权守卫 |
| source_config_package_id | uuid | 是 | 无 | F-56 特殊配置包 id |
| source_config_item_id | uuid | 是 | 无 | F-56 特殊内容项 id；非空时全表唯一 |
| enabled_at | timestamptz | 是 | 无 | 最近一次 ENABLE 时点 |
| disabled_at | timestamptz | 是 | 无 | INSTALL 或最近一次 DISABLE 时点 |
| last_transition_reason | text | 是 | 无 | 签名动作项的 reason，1..1000 UTF-8 bytes |

`ck_module_registrations_projection_shape` 强制 NOT_INSTALLED 的 package/source/安装与动作投影全空；两个 INSTALLED 状态的 package identity、三段 package 版本、digest/CMS/signer/signed_at、contract、`min_platform_version`、released_on、source、installed/state_changed/reason 均非空，只有 `max_platform_version_exclusive` 可为 NULL。该 NULL 唯一表示无上界；非空时才要求严格大于 min 且当前产品版本小于 max。时间投影固定为：INSTALL 同一提交时点写 `installed_at=state_changed_at=disabled_at` 且 `enabled_at=null`；ENABLE 只更新 `state_changed_at=enabled_at` 并保留 installed/disabled；DISABLE 只更新 `state_changed_at=disabled_at` 并保留 installed/enabled；UPGRADE/ROLLBACK_VERSION 只替换 package/source/reason 并更新 state_changed，不抹除三个历史时点。不存在制品正文、附件、路径、URL、脚本、SQL、WASM、容器或 hook 列。五条合法动作均为部署级命令且 DTO 不含 `legal_entity_id`：INSTALL/ENABLE 及采用另一版本的 UPGRADE/ROLLBACK_VERSION 必须从同一 current 有效 grant 证明目标 module 与依赖闭包全部 module codes 已授权；INSTALL 只落 disabled，依赖不要求已安装或启用，ENABLE 才额外要求每个依赖已为 `INSTALLED_ENABLED`。DISABLE 不要求 current 许可或维护期，因而在 Restricted 中仍允许，但 special item 必须携带当前安装旧 `SignedBusinessArtifact` exact bytes，identity/digest/signature/signer/source 必须与当前投影逐字段相等；不能把“总是允许”解释成任意旧包可停用。

fresh 090100 在建表事务原子种入恰 15 行，均由 `SYSTEM_PRINCIPAL_ID` 创建/更新、`row_version=1`、`install_state=NOT_INSTALLED` 且全部 package/source/install/action-time 列为空；不采用“缺行等于未安装”。catalog 身份闭集如下，行不得删除，id/code/display name 不可运行时编辑，也不得加第 16 行：

| id | module_code | display_name |
|---|---|---|
| `00000000-0000-7000-8000-000000000601` | `mdm` | 主数据管理 |
| `00000000-0000-7000-8000-000000000602` | `crm` | 客户关系管理 |
| `00000000-0000-7000-8000-000000000603` | `cpq` | 配置、定价与报价 |
| `00000000-0000-7000-8000-000000000604` | `clm` | 合同生命周期管理 |
| `00000000-0000-7000-8000-000000000605` | `sales` | 销售与订单 |
| `00000000-0000-7000-8000-000000000606` | `procure` | 采购管理 |
| `00000000-0000-7000-8000-000000000607` | `inventory` | 库存管理 |
| `00000000-0000-7000-8000-000000000608` | `costing` | 成本管理 |
| `00000000-0000-7000-8000-000000000609` | `project` | 项目管理 |
| `00000000-0000-7000-8000-000000000610` | `service` | 售后服务 |
| `00000000-0000-7000-8000-000000000611` | `finance` | 收付款与往来 |
| `00000000-0000-7000-8000-000000000612` | `ledger` | 经营分录与期间 |
| `00000000-0000-7000-8000-000000000613` | `invoice` | 发票管理 |
| `00000000-0000-7000-8000-000000000614` | `portal` | 客户与供应商门户 |
| `00000000-0000-7000-8000-000000000615` | `reporting` | 报表与分析 |

若当前 package 的 inner signer 和/或 current RELEASED source special outer signer 被当前 bundle CRL 明确标为 REVOKED，该模块 effective runtime 立即关闭，但部署级 `LicenseStatus` 不变；旧 package 不作当前正向证明，任一层 signer 已 REVOKED 的历史包都不再是 ENABLE/ROLLBACK_VERSION/rollback candidate。唯一停用逃生口是不重签 inner：新 `action=DISABLE` special item 仍携带旧 SignedBusinessArtifact exact bytes，只由 ACTIVE signer 对新 canonical outer manifest 签名；仅当旧 inner、旧 source outer、不可变 source item/接受摘要及其 payload/digest/signature/current projection 自洽，失败类别只能是旧 inner 和/或 source outer CRL REVOKED，且未撤销层为 ACTIVE 或 RETIRED-nonrevoked 时放行。bad digest/signature/source/chain、不能唯一分类或任何其他 action 全拒。停用后若许可有效，只有 inner+outer signer 都为 ACTIVE、semver 严格更高且全部普通 UPGRADE 守卫成立的新包可替换 revoked projection；旧包只留作版本/审计历史。法人 scope 只在逐法人业务请求的 `ModuleLicenseQuery` 中判。运行角色无 DELETE，identity 只能由新的已签名 MODULE_PACKAGE 动作替换。

contract digest 的唯一前像是仓库恰 15 个 `contracts/modules/<wire>.contract.v1.jcs` strict descriptor，每个至多 262,144 bytes、UTF-8 无 BOM、RFC 8785 JCS exact bytes。DTO 恰为 `ModuleContractDescriptorV1 { schema_version:1,purpose:"EP-MODULE-CONTRACT-V1",module_code,module_contract_version,module_dependencies,abi_entries }`；version wire 只收 1..2,147,483,647 的 JSON integer，Rust u32 的更大值在 parser/adapter 入库前拒绝。dependencies 按 wire 排序去重、只指 15 值且不含自身，全图 DAG；ABI 为 1..4096 项，按 `(kind,code)` 排序且组合唯一，kind 只取 `COMMAND|QUERY|EVENT|JOB|PERMISSION`，code 匹配 `[a-z][a-z0-9_.-]{0,127}`。每项 schema 唯一位于 `contracts/modules/<wire>/schemas/<schema_sha256-lowerhex>.schema.v1.jcs`，最大 65,536 bytes、strict JCS JSON Schema 2020-12，只允许本文件 `#` fragment ref；文件名、entry 值与重算摘要三等。`module_contract_sha256=SHA-256(descriptor exact bytes)`；任一 byte/dependency/ABI/schema digest 变化必须严格升 version，同 version 不得换 digest。每个 `ep-contract-<module>` 的版本/摘要/ABI registry 由 descriptor 生成，`cargo xtask module-contracts verify` 与 compiled public registry 双向 exact-set；禁止手写摘要或第二 dependency registry。

模块契约与依赖的唯一产品目录是待签 `target/release-package/product-modules.v1.jcs`、安装后 `C:\EP\product-modules.v1.jcs` 的同一 exact file，最大 262,144 bytes、UTF-8 无 BOM、strict RFC 8785 JCS。DTO 恰为 product_version 与按 wire 排序的 15 行 modules；每行只有 module_code、1..2,147,483,647 的 contract version、32-byte digest 的 64-lowerhex JSON wire、排序去重 dependency 闭集，且全图 DAG。文件只由上述 15 个已验证 descriptor 的 version/digest/dependencies 和 canonical product version 生成并 strict 回读，属于 `MANIFEST.sha256` closed roster 并受产品 Authenticode CAB 覆盖；安装器 safe-handle 原子复制/readback，core/worker 从 `C:\EP` fixed root 打开并拒绝 reparse/ADS/hardlink/path drift。Stage 14 既有产品投影保存 exact file digest、product_version、15 行 contract/dependency digest 与 DAG 结论；不为此新增 F-56 表、列或迁移。

全部 RELEASED MODULE_PACKAGE history 保持两套一一映射：同一 `package_id` 只能对应同一 exact inner artifact；同一 `(module_code,package_code,package_version)` 只能对应同一 `package_id` 与同一 exact inner。ENABLE/DISABLE/ROLLBACK_VERSION 可重复带回该 exact inner；不同 payload/digest/signature/signer bytes 冒用任一 identity，在 release 锁内和 093300 deferred COMMIT 都整项拒绝，不允许运行时任选历史。

动作的 signer 状态是运行守卫而非新列：首次 INSTALL 与新 UPGRADE 要求 inner+本次 special outer 均 ACTIVE；ENABLE/DISABLE 复用既有 RELEASED inner exact artifact，inner 可 RETIRED 但不可 REVOKED，本次 outer 必须 ACTIVE；ROLLBACK_VERSION 只可精确引用既有 RELEASED historical artifact，inner 可 RETIRED 但不可 REVOKED，新 outer 仍 ACTIVE。current inner 和/或 current source outer REVOKED 的唯一例外是上一段 DISABLE 逃生口。

并发原语唯一为 `ModuleOperationGate`，key=`hashtextextended('platform-module:' || ModuleCode wire,0)`。业务事务在读取业务行前取得 owner module transaction-level shared lock；worker 在读取 payload/claim/dispatch 前以专用连接取得 session-level shared lock，finally 释放；两者随后调用 effective gate 递归复验依赖。INSTALL/UPGRADE/ROLLBACK_VERSION 取目标 exclusive；ENABLE 按全局 wire 顺序取目标 exclusive 与传递依赖 shared，锁内重验 raw/effective；DISABLE 按 15 个 ModuleCode wire 顺序取得全部 15 把 exclusive，在总计 30 秒 deadline 内锁齐但只修改目标。任一超时以 `PLATFORM.MODULE.IN_FLIGHT_DRAIN_TIMEOUT` 整笔回滚，状态、路由、幂等、Outbox、审计零部分变化。

`V20261013093300` 在配置表存在后添加 `fk_module_registrations_source_package(source_config_package_id)->platform_meta.config_packages(id) ON DELETE RESTRICT` 与 `fk_module_registrations_source_item(source_config_package_id,source_config_item_id)->platform_meta.config_package_items(config_package_id,id) ON DELETE RESTRICT`，并建立 `ux_module_registrations_source_config_item_id`。它依赖同迁移添加的父候选键 `UNIQUE(config_package_id,id)`，因此错包 item 即使 id 存在也不能写入。

### 6.25 license_grants（F-56 签名许可与撤销投影，阶段 3）

`license_trusted_signer_subjects` 是 identity roster 而非撤销表。CAB 新 signed deployment manifest 的 roster 必须包含数据库全部 RELEASED special inner+outer 历史引用 token 的 exact superset，删除任一引用 token 必须使轮换失败。保留 revoked token 只为历史可识别，CRL 分类仍优先产生 REVOKED，不重新授权；新 artifact 必须同时 token 在 roster 且整链 ACTIVE。已引用 token 真正移除只能可信整库回退或新 deployment，不得原地删历史。

F-56 CMS 链首版是窄 DER/RFC profile，不锁 crate/API，实现版本由 `Cargo.lock` 与 SBOM 固定。所有 license chain certificate 拒绝 `nameConstraints|certificatePolicies|policyMappings|policyConstraints|inhibitAnyPolicy`。leaf 必需 SKI/AKI/KU/EKU，可选 BC 只能 absent 或 `CA=false`；KU 只含 `digitalSignature`，EKU 只含 `codeSigning`。CA（intermediate/anchor）extension 闭集恰为 SKI、AKI、critical `BC(CA=true,pathLen honored)` 与 critical `KU(keyCertSign+cRLSign)`。所有未列 certificate extension 无论 critical 与否都拒绝。完整 base CRL 的 extension 只允许且必须有 AKI+CRLNumber；IDP、delta、freshest、任何其他 CRL extension 与任何 entry extension 全拒绝。不存在“policy 成立”或库默认 extension 的宽松分支。

initial-governance 只使用一套 typed evidence：`projection_digest(domain,dto)=SHA-256(ASCII(domain)||0x00||JCS(dto))`，typed root 的 `schema_version` 是 JSON number `1`、`purpose=domain`。`platform.bootstrap.initial_governance.v1` typed audit ABI 的完整 envelope 固定 `event_id=<事务内预分配 UUIDv7>`、`legal_entity_id=<signed governance_legal_entity_id>`、`actor_user_id=SYSTEM_PRINCIPAL_ID` 且命中本事务已建的同法人 ACTIVE SYSTEM grant、`actor_device_id=null`、`action='INITIAL_GOVERNANCE_BOOTSTRAPPED'`、`object_type='platform.initial_governance'`、`object_id=bootstrap_id`、`object_version=1`、`before=null`、`after=<下述 exact root>`、`reason/approval_ref/reauth_ref=null`、`client='system'`、`occurred_at=committed_at`；`event_day/seq/prev_hash/hash` 仅由 AuditWriter 派生。receipt 的 `audit_event_id/committed_at` 与之相等。audit `after` exact 为 `{schema_version:1,purpose:"EP-INITIAL-GOVERNANCE-AUDIT-V1",bootstrap_id,deployment_id,bootstrap_body_sha256,bootstrap_authorization_registry_sha256,initial_license_archive_sha256,deployment_manifest_sha256,database_bootstrap_projection,database_bootstrap_projection_sha256,receipt_body_sha256,schema_manifest_sha256,ep_migrate_pe_sha256,committed_at,status:"COMMITTED"}`。

`database_bootstrap_projection` domain 恰为 `EP-INITIAL-GOVERNANCE-DATABASE-PROJECTION-V1`，root exact `{schema_version,purpose,legal_entity,key_domain,operators,legal_entity_grants,roles,role_permission_pairs,user_role_grants,approval_chains}`；`approval_chains` 是 F-56 冻结的 37 项复数集合，不接受 singular 别名。child 形状/null/排序/exact-set 逐字采用 F-56 同名 database projection ABI，不另造第二套。`database_bootstrap_projection_sha256=projection_digest("EP-INITIAL-GOVERNANCE-DATABASE-PROJECTION-V1",database_bootstrap_projection)`，authorization registry 也只按 F-56 canonical DTO 求 domain-separated digest；数组全按 ABI wire bytes canonical 排序。`receipt_body_sha256=SHA-256(receipt exact JCS bytes)` 不自包含；预分配 event id 与冻结 committed_at 使 receipt 在 audit INSERT 前可唯一重建，无 digest 循环。after/projection 必须绑定 receipt ids/digests/mapping，严禁密码、PHC/verifier、credential secret、证书正文或任何秘密。

签名 `DeploymentManifestV1.license_trusted_signer_subjects` 是 F-56 inner/outer signer 授权的唯一事实，恰含 1..64 个 exact `spki-sha256:<64 lowerhex>`，按 UTF-8 bytes 严格升序去重。本地 `release.trusted_signer_subjects` 只是可选 exact-equal 断言：`[]` 表示不覆盖且直接使用 signed roster；非空必须长度、顺序、token 逐字等于 roster，否则 readiness 与 ops check 失败关闭。本地值不得增删/替换 signer；parser 拒绝 0 项 signed roster、65 项、乱序、重复、非 lowerhex/错长 token 与任何 nonempty local mismatch。CAB signer 轮换必须更新并重签 deployment manifest；本地断言若非空则同批更新为新 roster exact copy，单改本地值永不构成授权。

部署级表，不带 `legal_entity_id`、`security_level`、`data_scope_tags`，不建 RLS；由 093300 登记进 `unpoliced_table_registry`。本表一行是一份已接受 grant，`id` 逐字等于 signed payload `grant_id`；续期插新行并保留旧行，撤销只在命中的 current 行写一次撤销组。完整列集为下表加可更新公共列 `id,row_version,created_at,created_by,updated_at,updated_by`：

| 列名 | 类型 | 可空 | 默认 | 语义 |
|---|---|---|---|---|
| license_no | text | 否 | 无 | 1..128 UTF-8 bytes；可跨续期重复，不建唯一键 |
| deployment_id | uuid | 否 | 无 | 必须等于签名部署清单和 Stage 14 current deployment id |
| governance_legal_entity_id | uuid | 否 | 无 | 首张 RELEASED grant 冻结的部署治理法人；真实 FK 指向 `platform_core.legal_entities(id) ON DELETE RESTRICT`，全部后继逐字相同且 LIST scope 必含 |
| issued_to | text | 否 | 无 | 1..256 UTF-8 bytes |
| license_kind | text | 否 | 无 | `PERPETUAL\|SUBSCRIPTION` |
| issued_at | timestamptz | 否 | 无 | 签发时刻，UTC 秒精度 |
| valid_from | date | 否 | 无 | 生效日 |
| valid_to | date | 是 | 无 | SUBSCRIPTION 必填且不早于 valid_from；PERPETUAL 必空 |
| maintenance_valid_to | date | 是 | 无 | SUBSCRIPTION 等于 valid_to；PERPETUAL 为空或不早于 valid_from |
| legal_entity_scope | text | 否 | 无 | `ALL\|LIST` |
| legal_entity_ids | uuid[] | 否 | `'{}'` | ALL 恰为空；LIST 为按 wire bytes 排序去重的 1..1024 个 UUID，并必须包含 `governance_legal_entity_id` |
| legal_entity_limit | int | 否 | 无 | 1..1,000,000 |
| named_user_limit | int | 否 | 无 | 1..1,000,000 |
| registered_device_limit | int | 否 | 无 | 1..1,000,000 |
| module_codes | text[] | 否 | 无 | 排序、去重、非空；元素取 15 个 ModuleCode |
| entitlement_codes | text[] | 否 | `'{}'` | 排序、去重、可为空；元素只取 `F55_LOCAL_AI\|F55_MCP` |
| payload_sha256 | bytea | 否 | 无 | 32 raw bytes；JSON wire 只收 64 lowerhex，从本行 grant 列重建 JCS 后必须相等 |
| signature | bytea | 否 | 无 | grant detached CMS exact bytes，1..1,048,576 bytes |
| signer_subject | text | 否 | 无 | exact `spki-sha256:<64 lowerhex>`，逐字命中签名 `DeploymentManifestV1.license_trusted_signer_subjects` 唯一 roster；显示 DN 不参与比较 |
| trust_bundle_sha256 | bytea | 否 | 无 | 32 bytes；grant 首次 RELEASE 时使用的离线 release trust bundle 摘要，必须等于 `grant_source_config_item_id` 所指 RELEASED item 的 `accepted_trust_bundle_sha256`；不可变且不要求永久等于以后轮换出的当前 bundle 摘要 |
| supersedes_grant_id | uuid | 是 | 无 | 自 FK `ON DELETE RESTRICT`；首 grant 为空，续期指 current 直接前驱，不得成环 |
| superseded_at | timestamptz | 是 | 无 | 移出 current slot 的可信时点 |
| current_slot | smallint | 是 | 无 | 只允许 `0\|null`，普通唯一键保证至多一张 current |
| last_trusted_at | timestamptz | 否 | 无 | UTC 秒精度；新 grant 初值固定为 `max(pre_import_trusted_now,candidate.issued_at)`，此后只允许单调推进 |
| revoked_at | timestamptz | 是 | 无 | 接受撤销的本地可信时点 |
| revocation_id | uuid | 是 | 无 | signed revocation id；非空时唯一 |
| revocation_issued_at | timestamptz | 是 | 无 | signed revocation `issued_at`，UTC 秒精度 |
| revocation_reason_code | text | 是 | 无 | `CONTRACT_ENDED\|REISSUED\|COMPROMISED\|CUSTOMER_REQUEST` |
| revocation_payload_sha256 | bytea | 是 | 无 | 撤销组非空时 32 raw bytes；JSON wire 只收 64 lowerhex，按列重建 JCS 后必须相等 |
| revocation_signature | bytea | 是 | 无 | 撤销组非空时 detached CMS exact bytes，1..1,048,576 bytes |
| revocation_signer_subject | text | 是 | 无 | exact `spki-sha256:<64 lowerhex>`；显示 DN 仅展示 |
| grant_source_config_package_id | uuid | 否 | 无 | grant 特殊配置包 id |
| grant_source_config_item_id | uuid | 否 | 无 | grant 特殊内容项 id，全表唯一 |
| revocation_source_config_package_id | uuid | 是 | 无 | 与 revocation item 同空同非空 |
| revocation_source_config_item_id | uuid | 是 | 无 | 非空时全表唯一 |

`ck_license_grants_kind_dates`、`ck_license_grants_scope`、三项 limit CHECK、摘要/签名长度 CHECK、`ck_license_grants_current_shape` 与 `ck_license_grants_revocation_shape` 逐项强制上表；`ux_license_grants_current_slot(current_slot)`、`ux_license_grants_revocation_id(revocation_id)`、`ux_license_grants_grant_source_item(grant_source_config_item_id)`、`ux_license_grants_revocation_source_item(revocation_source_config_item_id)` 为普通唯一键，利用 PostgreSQL 多 NULL 语义保留历史。`license_no` 只有查询索引，不唯一。零 current 是合法未供给/恢复状态，唯一键只保证至多一张；首张有效 grant 接受事务提交后恰一张 current。首张 submit 前治理法人必须存在且 active；首张 RELEASE 同事务冻结该 deployment 的值，后继 grant 必须逐字相等，治理期间停用命令失败、删除由 FK 阻断。special 在 DRAFT 至 TEST_PASSED 的 `approval_legal_entity_id` 必须为空；命令只在内存从首张候选或首次 RELEASED history 派生 governance context 并核当前 operator 对该法人授权，请求头若存在只能相等。submit 同事务才首次写 approval 法人，PENDING_APPROVAL 及以后必须等于冻结值；提前写入、授权缺失、请求头覆盖或 history/source/signature 不唯一均失败关闭。

GRANT、REVOCATION 与 MODULE_PACKAGE 的 `apply` 只能在 F-56 special 全局锁序已经建立的事务内调用；applier 可幂等地再次请求同一 transaction-level `platform-license-current` advisory lock，但 whole transaction 的第一条业务 SQL 才是权威边界。取得该锁后才重读全部 current/history 与已接受撤销，禁止先查候选、锁 package/order/item 或只锁可能不存在的 current 行。锁内按下段 `TrustedClockV1` 唯一公式求不含候选的 `pre_import_trusted_now`，接受新 grant 时写 `last_trusted_at=max(pre_import_trusted_now,candidate.issued_at)`；接受 revocation 时把命中 current 推进到 `max(existing_last_trusted_at,pre_import_trusted_now,candidate.issued_at)`。续期随后把旧 current 移槽并写 superseded_at 后插新 current；并发首发、同一前驱并发续期和续期/撤销竞态均在锁内重算且恰一合法提交，输家返回 `PLATFORM.CONFIG_PACKAGE.SPECIAL_ITEM_SHAPE_INVALID`，不得泄漏 SQLSTATE。撤销保留 current 行。运行角色无 DELETE；grant identity/signature/source 不可改，只允许受控 current 替换、可信时间单调推进和一次已验签撤销。

093300 添加四条 license source 约束：`grant_source_config_package_id -> platform_meta.config_packages(id)`、`(grant_source_config_package_id,grant_source_config_item_id) -> platform_meta.config_package_items(config_package_id,id)`、`revocation_source_config_package_id -> platform_meta.config_packages(id)`、`(revocation_source_config_package_id,revocation_source_config_item_id) -> platform_meta.config_package_items(config_package_id,id)`，全部 `ON DELETE RESTRICT`；grant 两列非空，revocation 两列同空同非空。与 §6.24 明列的 module package FK 和同包复合 item FK 合计六条。deferred graph 在 COMMIT 强制治理法人唯一冻结/后继相等/LIST 包含/持续 active、special 提交前 approval 法人为空且提交后等于派生值，以及全部 RELEASED MODULE_PACKAGE history 的两套 exact-inner identity 一一映射。数组、payload digest、CMS/链/撤销、deployment、后继及 trusted time 由 applier 锁内重建复验。表中不存 license_status；唯一 `ModuleLicenseQuery` 五方法中 `license_evaluation()` 从同一快照返回 status/reason/trusted_now，调用方不得拆查。

每个 core/worker 进程恰有一个 `TrustedClockV1`。数据库连接后、public readiness 前必须先验证相关 audit hash chain，再读持久证据与一次 `system_utc_at_start`；唯一启动集合为 `process_anchor_utc=max(initial-governance bootstrap committed_at,current/history grant.issued_at,已接受 revocation.issued_at,全部 license_grants.last_trusted_at,全部已验证 hash-chain 且结构有效的 trusted-time checkpoint.trusted_now,system_utc_at_start)`。尚无某类证据时只从 max 集合移除该项，不得视为 epoch；未验审计链的 bootstrap/checkpoint 不得纳入。随后捕获 OS monotonic `Instant`，进程内候选恒为 `process_anchor_utc+monotonic_elapsed`。每次 query/apply 在事务开始后只读一次 wall clock，唯一公式为 `trusted_now=max(上述持久证据,system_utc_now,process_anchor_utc+monotonic_elapsed)`，日期取 UTC calendar date；普通 query 只计算、不写行。readiness 与 special 推进关口固定取 `LICENSE_CURRENT_EXCLUSIVE` 并锁内 CAS current；typed reject NONE 且不推进。job-worker target cadence 不得超过 240 秒，checkpoint 固定取 exclusive，current 只在严格增加时 CAS；append-only audit 永不 UPDATE。缺 checkpoint 或 wall/monotonic trajectory 偏差超过 300 秒发不可抑制安全告警；崩溃跨重启最多存在小于 300 秒未持久观察窗口，必须如实披露，不宣称 NTP/TPM 级防篡改。已持久化错误前跳只能随 Stage 14 可信备份整体恢复。

checkpoint audit action 固定 `LICENSE_TRUSTED_TIME_CHECKPOINT`，after strict-JCS 闭集 `{schema_version:1,purpose:"EP-LICENSE-TRUSTED-TIME-CHECKPOINT-V1",deployment_id,slot_utc,trusted_now,current_grant_id}`；`schema_version` 沿具名 typed-audit ABI 是 JSON number `1`。`slot_utc` 唯一按 `floor(unix_seconds(trusted_now)/240)*240` 计算并输出 canonical RFC 3339 UTC whole-second，分钟可为 `00/04/08/...`，跨小时仍按 Unix epoch，禁止五分钟取整。`ensure_checkpoint` 入口在同一 license exclusive lock 内、任何业务 mutation 前一次性捕获 `trusted_now`、唯一 current grant id/null 与命中 current 的 revocation id/null，据此冻结 slot/payload snapshot；terminal AuditWriter 只使用该 snapshot，禁止在 grant/revocation 投影 mutation 后重读 current 或重算 trusted_now。

slot 查询零行时才预分配新 UUIDv7 event id 并追加；一行复用不得分配或写新事件。新 checkpoint 的完整 envelope 固定 `legal_entity_id=<冻结治理法人>`、`actor_user_id=SYSTEM_PRINCIPAL_ID` 且命中同法人 ACTIVE SYSTEM grant、`actor_device_id=null`、`action='LICENSE_TRUSTED_TIME_CHECKPOINT'`、`object_type='platform.license_trusted_time'`、`object_id=deployment_id`、`object_version=null`、`before=null`、`after=<入口冻结 exact payload>`、`reason/approval_ref/reauth_ref=null`、`client='system'`、`occurred_at=<入口捕获 trusted_now>`；`event_day/seq/prev_hash/hash` 仅由 AuditWriter 派生。deployment object id 不随 current grant 换槽，数据库当前时间或默认 object 均不等价。

exclusive 内按 `(action,after->>'purpose',after->>'deployment_id',after->>'slot_utc')` 查询：0 行才由 AuditWriter 用冻结 snapshot 追加；1 行保留既有 exact bytes，只核 shape/current id/hash chain；>1 或不等失败关闭。同 slot 后续动作复用既有 checkpoint，不要求其 trusted_now 等于本次值；current.last_trusted_at 独立 CAS。耐久键恰为 `license-trusted-time:v1:<lowercase-deployment-id>:<slot_utc>`；audit legal_entity/actor/client 固定治理法人、其 SYSTEM grant、system。job target cadence 不得超过 240 秒，special 也 ensure 当前 slot，有 uptime 证据时持久 gap 小于或等于 300 秒才 PASS，大于 300 秒失败。零 current 首发与必要 checkpoint 同事务，不能因 current 为空自锁。

checkpoint 治理来源严格优先为 current grant、validated initial-governance audit/receipt、本次首张 GRANT candidate。仅当部署显式 non-production、bootstrap absent、zero-current 且 legal_entities 零行时，public readiness 可不写 checkpoint并固定 `RESTRICTED/NO_CURRENT_GRANT`，worker dormant，且永不得生成 Stage 14 evidence；production bootstrap/readiness 无豁免。此后首张 GRANT whole-exclusive 事务从 candidate 派生治理法人，必须先证明法人已存在、operator 授权有效，并同事务创建首 checkpoint。

签名身份与 CRL 无第二种解释：所有 `Sha256Digest` JSON wire 只收 64 lowerhex、数据库只存 32 raw bytes；所有 F-56 `*_signer_subject` 与 special outer `signer_subject` 只收 `spki-sha256:<64 lowerhex>`，输入为 leaf exact DER SPKI，并且只能逐字命中签名 `DeploymentManifestV1.license_trusted_signer_subjects` 唯一 roster；display DN 只作显示，本地配置不构成第二信任源。outer 的 RFC 3339 signed_at、inner 的 RFC 3339 issued_at 必须分别与 CMS ASN.1 signingTime 语义上为同一 UTC whole-second instant；1950..2049 只用 DER UTCTime，其余只用 GeneralizedTime，Z-only、含秒、无小数/offset，不比较文本 bytes。SignerInfo、链中每张证书与每份 CRL 的 signature AlgorithmIdentifier 只允许 ECDSA P-256/SHA-256 parameters absent，或 RSA-PSS/SHA-256（RSA modulus≥3072、MGF1-SHA256、saltLength=32、trailerField=1）；SHA-1、RSA PKCS#1 v1.5、NULL/默认/隐式参数均拒绝。

状态从唯一完整链导出，non-anchor=leaf+全部 intermediate。每张 non-anchor 必须在 signed_time 有效；anchor 必须在 signed_time 有效且通过自签/CA/KeyUsage/critical-extension 检查，trusted_now 后 anchor 自身过期不触发 RETIRED，但从 bundle 移除/替换或形成多链立即 UNTRUSTED。`REVOKED>ACTIVE>RETIRED>UNTRUSTED` 仅在全链 CRL prerequisite 成功后适用：结构/unique path/signed_time 通过后，先为每个实际 issuer 唯一选出 global-highest、覆盖 trusted_now、issuer Name/AKI/SKI/签名/CRLNumber/nextUpdate 全合法的完整 base CRL；任一 issuer 缺失、尚未生效、过期、同最高号冲突、无覆盖、delta/indirect/removeFromCRL/unknown critical 或非法，整链立即 UNTRUSTED，不扫描其他 issuer serial、不进入 CRL recovery、不退旧 CRL。只有全集成功才扫描全部 serial：任一命中为 REVOKED；零命中且全部当前有效为 ACTIVE；零命中、全部 signed_time 有效、当前至少一张过期且无 not-yet-valid，并有首次 ACTIVE 接受/source/digest/signature 自洽证据才 RETIRED且只复验既有 RELEASED current/history。新 import/release inner+outer 只接 ACTIVE，既有合法 current/history 可 ACTIVE 或 RETIRED-nonrevoked；不访问 CDP/OCSP/网络。

强制负例：两 issuer 链中一条 serial 被合法 CRL 命中、另一 issuer 的 global-highest CRL 缺失/非法时，结果必须先为 UNTRUSTED 而非 REVOKED；修复第二 issuer prerequisite 后才允许全链 serial scan 得出 REVOKED。

grant 行内 `trust_bundle_sha256` 与 source item 的接受摘要只记录首次 RELEASE 证据；revocation/module action 不另增摘要列，只从各自不可删除 RELEASED source item 读取。计划轮换绝不回填这些值，也不得要求历史摘要持续等于当前 bundle。当前 `license-roots.p7b` 的实际摘要必须等于签名部署清单中的期望摘要；轮换只允许 CAB 维护操作同步更新两者、关闭许可/模块变更门，并以新 bundle 枚举重验 exact set：全部 RELEASED `LICENSE_GRANT` items（Grant 与 Revoke）及全部 RELEASED `MODULE_PACKAGE` items。每项复验 persisted inner/outer artifact，并把 current grant/current revocation/current module projection 与 source/type/digest/identity 交叉核对，签名证据保存旧接受摘要、新验证摘要、对象 id、outer 结论、inner 结论与总结果。

current grant 或命中 current 的 revocation 的 inner 和/或 source special outer 失败使全局 `RESTRICTED/SIGNATURE_INVALID`；当前安装模块的 inner 和/或 current source outer 失败只关闭该模块 effective runtime，绝不改变只由 current grant/revocation 导出的部署级 `LicenseStatus`。历史 inner 或 outer signer 被新 CRL 明确命中时标记 `HISTORICAL_SIGNER_REVOKED`，保留/隔离并排除 purchased、rollback candidate 和正向证明，但不倒推另一份有效 current 为 Restricted；其他历史断链、source/digest/signature 漂移或结构损坏只关闭许可/模块变更门与共同 release gate，不改写独立有效 current，直至可信恢复。若唯一 current grant 或命中它的 revocation 的 inner 和/或 source outer 唯一失败为 CRL REVOKED，只有旧 row/source/payload/digest/signature bytes、outer bytes 与接受证据自洽时，inner+outer 都为 ACTIVE、同 deployment/治理法人且直接后继的 GRANT 才可在固定 advisory-lock 事务恢复。没有相应 CAB 清单更新的磁盘或部署清单漂移仍立即使 current 失败关闭，不得回退到旧 bundle、历史接受摘要或 Windows 任意根。

fresh production 零 current 的唯一首装入口是既有 `ep-migrate apply --initial-governance-bootstrap=<bootstrap.jcs> --initial-license-package=<license.epcfg> --receipt-out=<dir>`，不是新子命令、端点、服务、表或迁移。验证 signed deployment id 为 canonical lowercase UUID 后，三参数只接受固定根 `C:\ProgramData\EnterprisePlatform\evidence\stage14\initial-governance\<lowercase-deployment-id>\` 的 `bootstrap.jcs`、`license.epcfg` 与该目录，receipt 固定名 `initial-governance.receipt.v1.jcs`。目录 owner SYSTEM、DACL PROTECTED；显式 inheritable allow ACE 恰为 SYSTEM/BUILTIN\Administrators/`NT SERVICE\ep-ops` FullControl 与 `NT SERVICE\ep-core` 的 `FILE_GENERIC_READ|FILE_TRAVERSE|READ_CONTROL|SYNCHRONIZE`，其余包括 Users/Authenticated Users/Everyone/ep-worker/ep-ai/ep-integ/ep-plugin 无 ACE，ep-core 无 write/delete/WRITE_DAC/WRITE_OWNER。fixed-root safe handle 拒绝 UNC/device/reparse/ADS/hardlink/8.3/case/path drift；receipt 只准 CREATE_NEW、flush、关闭后 readback，禁止覆盖与 sidecar。

`bootstrap.jcs` 最大 1,048,576 bytes、strict JCS，root exact `{body,authorizations}`。body exact 为 `schema_version=1,purpose="EP-INITIAL-GOVERNANCE-BOOTSTRAP-V1",bootstrap_id,deployment_id,deployment_manifest_sha256,initial_license_archive_sha256,issued_at,expires_at,legal_entity,operators`；窗口大于 0 且不超过 24 小时。`legal_entity` exact 为 `{id,key_domain_id,code,entity_no,name,short_name}`，两 id 均 canonical UUID，id 等于候选 grant 治理法人，signed `key_domain_id` 全库未占用；operators 与 authorizations 各恰两项、按 `CONFIG_OPERATOR|SECURITY_APPROVER` 排序，分别绑定两个互异 user/login/device/SPKI 与 win/mac client，两份 detached CMS 都签 `JCS(body)` 且来自签名部署清单中两个不同客户安全管理员证书。固定 `license.epcfg` 必须完整通过 F-56 container、ACTIVE inner/outer、首张 grant、deployment/governance/scope 与 archive digest 验证，只作资格验证，不代应用内 import/审批/RELEASE。

fresh 数据库前置 exact 为：`legal_entities` 零行；`user_accounts` 恰一行 SYSTEM_PRINCIPAL_ID/SYSTEM；`user_credentials`、`user_password_history`、`user_devices`、`sessions`、`reauth_challenges`、`login_attempts`、`account_lockouts`、`breakglass_activations` 零行；法人 authz、key-domain/data-key、license、config package/item/order 业务行零行；SYSTEM/deployment/migration seed 与签名清单完全相等。唯一 PostgreSQL 事务创建 active 治理法人、signed key_domain_id 的 `LEGAL_ENTITY/PROVISIONING` 域并写 exact logical `kek_ref="kms://ep/v1/deploy/<lowercase-deployment-id>/domain/<lowercase-key-domain-id>/kek/1",kek_version=1,provisioned_at=null`，另建两名 ACTIVE+MFA EMPLOYEE、各一 PENDING Win/Mac device及 console password/X509 credential。随后必须先创建恰三条 ACTIVE `platform_authz.user_legal_entity_grants`：同一治理法人分别授给 SYSTEM_PRINCIPAL_ID、CONFIG_OPERATOR user、SECURITY_APPROVER user，三行 id 为新 UUIDv7，`granted_by=SYSTEM_PRINCIPAL_ID`、`granted_from=<bootstrap committed_at 的 UTC date>`、`granted_to=null`；除此之外该表仍零行。只有这三行对复合 FK 可见后，才创建两条用户角色绑定、`F56_CONFIG_OPERATOR` 与 `SECURITY_ADMIN` exact permissions 及 default CONFIG_RELEASE chain；两个角色绑定的法人/user/granted_by 必须逐字命中对应 grant。最后写 `platform.bootstrap.initial_governance.v1`，payload 绑定三条 grant id 及 exact `(legal_entity_id,user_id,granted_by,granted_from,granted_to)` mapping；密码只从关闭 echo 的本机 ReadConsoleW 各确认两次，拒绝 stdin/argv/env/file。事务不调用 KMS、不建外部 material、不插 data_keys、不写 ACTIVE；失败只回滚数据库事实。两台设备随后必须各自通过 current signed device policy 的真实 attestation 并写唯一 `PENDING→COMPLIANT` transition，Stage 14 才允许两名 bootstrap 用户完成登录证明；bootstrap 双 CMS 不能替代端点 attestation。

事务提交后 core-server 在 public readiness 前按 signed bootstrap_id/key_domain_id 调既有 `KeyDomainProvisioner` resume；它经 `KmsKeyMaterialProvisioner` 保证 logical KEK，并为四 purpose×四 scope exact 16 tuple 逐把生成同一 DEK 的 operational/recovery 双 recipient 信封，再只经 operational 路径 readback。全部通过后唯一数据库事务插 16 条 ACTIVE data_keys、把同一域 `PROVISIONING→ACTIVE` 并写 §6.3 唯一 `platform.key_domain.activated.v1`，其中 `activation_source=INITIAL_GOVERNANCE,bootstrap_id=<signed bootstrap_id>`；失败保留 PROVISIONING、按既有规则补偿/隔离 orphan、关闭 readiness，重启只继续同一域并返回 `PLATFORM.KEY_DOMAIN.KEY_UNAVAILABLE`。非秘密 receipt body exact 为 `schema_version=1,purpose="EP-INITIAL-GOVERNANCE-RECEIPT-V1",bootstrap_id,deployment_id,bootstrap_body_sha256,initial_license_archive_sha256,governance_legal_entity_id,key_domain_id,key_domain_state="PROVISIONING",operator_user_ids,device_ids,role_codes,legal_entity_grant_ids,committed_at,audit_event_id,schema_manifest_sha256,ep_migrate_pe_sha256,status="COMMITTED"`；四个数组均按 bytes 排序，`legal_entity_grant_ids` 恰为上述三行 id。`receipt_body_sha256` 不自包含，只写审计 payload；可信性来自双 CMS、命中产品清单的 Authenticode ep-migrate PE digest、审计 hash chain 与 exact cross-check，ep-migrate 无 KMS/sidecar 能力。同 input digest 且数据库已有同审计终结、仅漏 receipt 时可只读补文件；其余重跑永久拒绝。Stage 14 必须再核 receipt/audit exact bytes、三条 grant 的 exact mapping、DB projection、初始 archive、最终首张 RELEASED grant、同域 ACTIVE、16-tuple 双信封 graph 与唯一 activation audit。

Control Center（受信 `ClientKind::Ops`）每次读取与每日 monitor 都在同一 repeatable-read snapshot 实时按唯一 SQL 谓词计数：启用法人=`platform_core.legal_entities.is_active=true`；命名用户=`platform_core.user_accounts.account_kind<>'SYSTEM' AND status IN ('ACTIVE','LOCKED','SUSPENDED')`；已登记设备=`platform_core.user_devices.status IN ('PENDING','COMPLIANT','RESTRICTED')`，REVOKED 不计且进入 RESTRICTED 不得逃避额度。三项按部署去重主键；每日任务只刷新 metrics 并在越限/恢复边缘发既有告警，不新增 usage 表、日终持久化快照、月度签名申报、联网遥测或发行方上报。授权导出只是当时点报表并走普通导出审计，不得冒充日终/月报；超限不阻断创建。Restricted 的写例外闭集为 LICENSE_GRANT import→autotest→submit→仅 Win/Mac `CONFIG_RELEASE` approve-or-reject→sign→RELEASE order/execute 全链，以及 action=DISABLE 的 MODULE_PACKAGE 同一全链；Control Center 的 `ops` origin 对审批待办与结论只读，绝不调用 approve/reject。其他常规业务写返回 `PLATFORM.LICENSE.RESTRICTED`。即使部署级许可仍为 ACTIVE/EXPIRING_SOON/GRACE_PERIOD，`LicenseAdmissionGate` 也必须对每个已有目标法人的普通业务写以受信上下文的 `Some(legal_entity_id)` 重验 LIST scope；不命中时返回同一码且零写入，但不改变全局 `LicenseStatus`、不产生新的 restriction reason，查询/报表/导出继续可用。只有真正无目标法人的部署级操作可传 None，已有目标法人传 None 必须失败关闭。该恢复闭集逐项适用于 `NOT_YET_VALID|EXPIRED_BEYOND_GRACE|REVOKED|SIGNATURE_INVALID|NO_CURRENT_GRANT` 五个 `LicenseRestrictionReason`，避免许可自锁。

### 6.26 feature_flags（部署级功能开关，阶段 3）

不带法人列和 RLS，登记于 `unpoliced_table_registry`。自有列：`feature_code text`（全表唯一）、`module_code text`（15 个 ModuleCode）、`is_enabled boolean`、`requires_license boolean`；加可更新公共列 `id,row_version,created_at,created_by,updated_at,updated_by`。`module_state()` 仅返回 raw `install_state` 管理投影；`module_is_currently_licensed(module,legal_entity_id)` 才是 effective runtime admission：在同一 repeatable-read snapshot 从签名 product DAG 递归计算目标与传递依赖闭包，只有闭包每行 raw INSTALLED_ENABLED、current projection/唯一 RELEASED source item/accepted digest/inner exact artifact/source special outer exact bytes 自洽、inner 与 outer 在当前 bundle 下 ACTIVE 或 RETIRED-nonrevoked、contract/version/dependencies 命中 product catalog，且同一 current grant 有效、module_codes 覆盖闭包、目标法人命中 scope 时 `Ok(true)`。完整合法负态（未装/停用/依赖停用/未授权/范围外/许可无效/明确 signer CRL revoked）返回 `Ok(false)`；结构、IO、strict parse、零/多 source、摘要/signature/source/catalog/DAG/projection 歧义返回 `Err`，调用者失败关闭。`feature_is_enabled(feature_code,legal_entity_id)` 先读取唯一 feature row，再无条件经过 owner module 同一 effective gate；仅 row enabled 且 owner effective 才 true，`requires_license` 不能成为绕过 owner/依赖/签名目录/current grant/scope 的分支。

## 7. platform_ops schema（阶段 2 与阶段 14）

> **F-57 当前再基线。** 本章原 Stage-14 §§7.3–7.17 的 offsite/writeout/backup/archive/recovery 表仅作为历史语义输入保留，不是可执行的当前 DDL。`docs/f57-legacy-migration-disposition.seed.tsv` 已把所有 `PLATFORM_OPS_RECOVERY_V1` 旧迁移统一替换到 Task 24 的 `V20261025093500__platform_ops_create_authority_epochs.sql`；实现与验收必须使用下列 F-57 exact set，禁止同时创建旧表、兼容视图或第二套状态真值。

### 7.0 F-57 当前 authority/backup/recovery exact set

Task 24 的聚合迁移恰创建以下 20 张当前表：

| table | mutability / owner | exact role |
|---|---|---|
| `authority_epochs` | current CAS + append-only evidence / command authority | deployment single-writer epoch and lease fence |
| `recovery_cuts` | immutable / backup coordinator | generation、base backup、LSN floor/target and attachment-set cut |
| `recovery_cut_attachments` | append-only / backup coordinator | exact pinned immutable attachment ciphertext refs/digests |
| `backup_targets` | signed current versions / backup control | target identity、failure domain、media/retention/role policy |
| `backup_topologies` | signed current versions / backup control | target/media/principal/capacity/custody graph |
| `backup_sets` | current CAS / backup control | set lifecycle、generation、cipher graph and recovery refs |
| `backup_set_objects` | append-only / backup control | object/chunk identity、EPB1 record digest/order/length |
| `backup_set_receipts` | append-only / backup control | conditional-create target receipt per exact record |
| `backup_checkpoints` | append-only / checkpoint signer | signed complete set、WAL span、cut and receipt graph |
| `backup_key_envelopes` | immutable version chain / recovery owner | ADR-0024 envelope digest/current predecessor graph; no plaintext key |
| `backup_runner_leases` | current CAS / backup runner | authority epoch、holder and trusted/monotonic lease |
| `offline_media` | current CAS / offline-media owner | current physical-media identity and lifecycle state |
| `offline_media_transitions` | append-only / offline-media owner | typed before/after version and required evidence |
| `backup_quota_reservations` | current CAS / backup control | bounded object/byte/rate/concurrency/emergency reserve admission |
| `recovery_materials` | append-only / recovery owner | independent recipient/token/custody material refs/digests |
| `recovery_material_rotations` | append-only / recovery owner | predecessor-linked rotation/revocation evidence |
| `recovery_drills` | append-only / recovery owner | clean-host drill input/profile/outcome/evidence |
| `recovery_certifications` | current CAS / recovery certification owner | exact certification state/profile/validity/predecessor |
| `recovery_certification_samples` | append-only / recovery certification owner | ordered success/failure/profile-equality evidence |
| `wal_retention_samples` | append-only / archive evidence owner | WAL floor/span/gap/capacity evidence bound to checkpoints |

All 20 are deployment-level control/evidence tables without `legal_entity_id` or business RLS. Each is separately registered by F57-25 in `platform_core.unpoliced_table_registry`; only exact least-privilege core/backup/recovery service roles receive grants. Every current row has positive `row_version` and CAS, every history/evidence row is append-only, foreign keys bind exact deployment/set/object/cut/envelope/checkpoint identities, and unique-current constraints reject two active target/topology/envelope/lease/certification slots. `recovery_cuts.verified_base_backup_id` must reference a `backup_sets` row whose verified checkpoint exists；`target_lsn >= max(base_backup_end_lsn,min_recovery_point_lsn)`；attachment count/Merkle root is recomputed from `recovery_cut_attachments`. A checkpoint is insertable only when every `backup_set_object` has one digest-equal target receipt, the global EPB1 ordinal range is exact and the WAL span has no gap. Pins/retention prevent ciphertext GC until all dependent cut/checkpoint leases are independently released.

`offline_media.state` is exactly `BLANK|ENROLLED|ACTIVE_APPEND|VERIFIED_DISCONNECTED|ROTATION_DUE|SEALED_VERIFIED|RETIRED_PENDING_DISPOSAL|DESTROYED`; allowed edges are exactly `BLANK→ENROLLED→ACTIVE_APPEND→VERIFIED_DISCONNECTED`、`VERIFIED_DISCONNECTED→ROTATION_DUE→ACTIVE_APPEND` and `ACTIVE_APPEND→SEALED_VERIFIED→RETIRED_PENDING_DISPOSAL→DESTROYED`. `DESTROYED` is terminal and physical reuse requires a new `media_id`；`SEALED_VERIFIED` never returns to writable. The transition table stores exact before/after row versions plus health/capacity/retention/rotation/destruction evidence, and a deferred graph trigger proves the current row is the deterministic fold of all transitions.

`recovery_certifications.state` is exactly `UNVERIFIED|INITIAL_RESTORE_VERIFIED|CANDIDATE_MEASURED|CERTIFIED|EXPIRED|INVALIDATED`. Success 1/2/3 for the same exact profile moves through the first four states；the third must occur within the rolling prior 90 days and creates `valid_until<=certified_at+90 days`. Failure while UNVERIFIED appends a failed sample without a state change；failure or any registered profile-dimension change after the first success moves to INVALIDATED；only CERTIFIED reaches EXPIRED at `valid_until`. EXPIRED/INVALIDATED are terminal and recertification creates a new ID with `predecessor_certification_id` pointing to the terminal row. Deferred graph triggers, profile digests, unique sample sequence and CAS make restart/concurrent writers deterministic.

The still-visible legacy prose below is explicitly non-normative for F-57 table identity/cardinality. Where it explains a security invariant not contradicted above, Task 24 may use it as test input；where names/state/roles differ, this §7.0 and the F-57 Task 24 plan win. Task 1/25 doc checks fail if any old Stage-14 migration/table is treated as current.

本 schema 终态固定为 24 张表和 5 个视图。`degradation_windows` 由阶段 2 建立，首次建表的 kind CHECK 与当时可用的 Rust `DegradationKind`/contract 恰含 `OFFSITE_SINK_NOT_CONFIGURED`、`WRITER_NOT_IN_SERVICE`、`PORT_NOT_IMPLEMENTED` 三项。Stage 14a 在同一变更集把 Rust 枚举/contract 扩为下表终态 21 项；数据库侧由 `V20261023092500__platform_ops_harden_backup_evidence_graph.sql` 把 kind CHECK 从 3 项替换为同序 21 项，并把不可抑制 CHECK 扩为终态 5 项。阶段 14 其余建立 22 表，F-55 再新增 `ai_model_packages` 一表。`deployment_records` 至 `alert_suppressions` 17 张以及 `ai_model_packages` 共 18 张部署级表不带 `legal_entity_id`、不建 RLS，按阶段 14/F-55 登记进 `platform_core.unpoliced_table_registry`；`data_migration_batches` 至 `data_migration_writer_receipts` 共 6 张业务数据表带 `legal_entity_id`，全部 ENABLE、FORCE RLS，复合外键同带法人。部署级表不是第 2 节所称业务表，其公共列只取 `id`、`security_level`、`data_scope_tags`、`created_at`、`created_by`，可更新表再取 `row_version`、`updated_at`、`updated_by`。F-55 表与 carrier 追加列逐列见 `docs/data-dictionary/ai_mcp.md`。阶段 14 的 092500/092600 硬化迁移向 `append_only_registry` 新增十条 APPEND_ONLY 与两条 IMMUTABLE_COLUMNS 登记：APPEND_ONLY 为 writeout_runs、attachment_watermarks、backup_verifications、archive_channel_transitions、replication_reports、key_recovery_verifications、alert_suppressions、data_migration_reconciliations、data_migration_approval_evidences、data_migration_writer_receipts，mutable_columns 均为空；deployment_records 与 offsite_sinks 为 IMMUTABLE_COLUMNS，mutable_columns 精确为 `{superseded_at}`。

### 7.1 degradation_windows（降级窗口台账）

不带 `legal_entity_id`、不建策略，登记于 `unpoliced_table_registry`。列定义按裁定 A-26 以阶段 14 计划为准，本阶段建表并交付两条约束；写入端口在 `ep-platform-obs`。

| 列名 | 类型 | 可空 | 默认 | 语义 |
|---|---|---|---|---|
| kind | text | 否 | 无 | 终态 21 值：`OFFSITE_SINK_NOT_CONFIGURED`、`WRITER_NOT_IN_SERVICE`、`PORT_NOT_IMPLEMENTED`、`OFFSITE_SINK_OFFLINE_MEDIA_RPO_DEGRADED`、`WAL_ARCHIVE_WRITEOUT_OVERDUE_OR_FAILED`、`ATTACHMENT_INCREMENTAL_WRITEOUT_OVERDUE_OR_FAILED`、`ATTACHMENT_BOOTSTRAP_WINDOW_EXCEEDED`、`ATTACHMENT_RPO_NOT_YET_ACHIEVED`、`AUDIT_EVIDENCE_WRITEOUT_OVERDUE_OR_FAILED`、`PORTAL_WAF_NOT_CONFIGURED`、`AUDIT_ANCHOR_OVERDUE`、`OFFSITE_COPY_PROTECTION_MISSING`、`ARCHIVE_SLOT_RETENTION_WARNING`、`ARCHIVE_CHAIN_BROKEN`、`RECON_RUN_UNFINISHED`、`PERIOD_CLOSE_ACCEPTANCE_REJECTED`、`AUTHZ_SNAPSHOT_CHECKSUM_MISMATCH`、`CUSTOM_OBJECT_DDL_INCONSISTENT`、`REPLICATION_CROSSCHECK_NO_RESULT`、`VIRUS_SCANNER_NOT_AVAILABLE`、`LEGAL_ENTITY_KEY_DOMAIN_UNAVAILABLE`；阶段 2 首次 CHECK 只含前三项，`V20261023092500` 才放行全部 21 项 |
| subject | text | 是 | 无 | 开窗对象的完整类型名（端口名或平台能力名），长度不超过 200 |
| scope_key | text | 否 | 无 | 范围键，长度不超过 200 |
| scope_legal_entity_id、scope_accounting_period_id | uuid | 是 | 无 | 标注列，只作标注不作策略判据 |
| basis | text | 否 | 无 | 开窗依据，长度不超过 2000 |
| detail | jsonb | 否 | '{}' | 附加明细 |
| opened_at | timestamptz | 否 | 无 | 开窗时间 |
| closed_at | timestamptz | 否 | 'infinity' | 关窗时间；未关闭取无穷远，`ck_degradation_windows_open_order` 要求晚于 opened_at |
| closing_condition | text | 否 | 无 | 关窗条件描述，长度不超过 2000 |
| is_suppressible | boolean | 否 | 无 | 可否抑制；终态 `OFFSITE_SINK_NOT_CONFIGURED`、`OFFSITE_COPY_PROTECTION_MISSING`、`WRITER_NOT_IN_SERVICE`、`VIRUS_SCANNER_NOT_AVAILABLE`、`LEGAL_ENTITY_KEY_DOMAIN_UNAVAILABLE` 五项不可抑制 |
| suppressed_until | timestamptz | 是 | 无 | 抑制截止时间 |

唯一约束 `ux_degradation_windows_kind_scope_closed` 固定使用 PostgreSQL 16 的 `UNIQUE NULLS NOT DISTINCT (kind, subject, scope_legal_entity_id, scope_accounting_period_id, closed_at)`：同一对象至多一个未关闭窗口；`NULLS NOT DISTINCT` 不得省略，否则部署级窗口的可空作用域会绕过唯一性。

阶段 14 的 `V20261023090300` 先加 `ck_degradation_windows_le_required` 与第一版四项 `ck_degradation_windows_not_suppressible`，`V20261023090350` 加 `ix_degradation_windows_kind_opened_at`、`ix_degradation_windows_closed_at_opened_at`、`ix_degradation_windows_scope_legal_entity_id_opened_at`；`V20261023092500` 再把 kind CHECK 从 3 项替换为终态 21 项，并把不可抑制 CHECK 替换为上述终态 5 项。`RECON_RUN_UNFINISHED` 与 `PERIOD_CLOSE_ACCEPTANCE_REJECTED` 必须同时带法人、会计期间标注；`OFFSITE_SINK_NOT_CONFIGURED`、`OFFSITE_COPY_PROTECTION_MISSING`、`WRITER_NOT_IN_SERVICE`、`VIRUS_SCANNER_NOT_AVAILABLE` 与 `LEGAL_ENTITY_KEY_DOMAIN_UNAVAILABLE` 不可抑制。

### 7.2 deployment_records（部署记录，受控版本行）

自有列：`revision bigint`、`server_spec jsonb`、`disk_capacity_floor_bytes bigint`、`resource_quota_frozen_ref text`、`rto_hours numeric(9,6) default 4.000000`、`rto_reestimated boolean default false`、`rto_reestimation_basis text null`、`shard_pickup_sla_hours int null`、`dual_control_authorizers jsonb default '[]'`、`waf_frontend_configured boolean`、`waf_attestation_at timestamptz null`、`virus_scan_mode text`（`NONE|CUSTOMER_ICAP`，无默认）、`virus_scan_icap_url text null`、`data_volume_within_baseline boolean`、`certification_report_ref text null`、`drill_report_ref text null`、`notes text null`，以及 F-55 的 `carrier_kind`、`provider_code`、`region_code`、`residency_jurisdiction_code`、`region_jurisdiction_code`、`vtpm_present`、`vtpm_attestation_ref`、`backup_failure_domain_code`、`backup_failure_domain_evidence_ref`、`carrier_attestation_ref`、`carrier_policy_ref`、`carrier_policy_digest`、`carrier_evidence_ref`、`carrier_evidence_digest`，最后为 `superseded_at timestamptz default 'infinity'`；F-55 **十四列**（含 policy/evidence 四列）的类型、可空性与条件 CHECK 见 `ai_mcp.md` §5。F-55 的 `CarrierEvidenceV1` 与当前 fact probe 对物理机、VM 两种 carrier 均要求 `tpm_version` 逐字等于 `2.0`；物理机 `server_spec` 是 strict object，其中 `tpm` 子对象字段恰为 `version,present,ready,ek_public_sha256,stage14_run_id,observed_at,probe_build_digest`，且唯一允许的形状为 `{version:"2.0",present:true,ready:true,ek_public_sha256:<64 lowerhex>,stage14_run_id:<UUIDv7>,observed_at:<UTC 秒精度 YYYY-MM-DDTHH:MM:SSZ>,probe_build_digest:<64 lowerhex>}`，不得缺字段、增字段或以自由 JSON 代替；`stage14_run_id`、`observed_at` 与 probe digest 必须逐项绑定同次 Stage 14 carrier evidence/probe。`revision`、`created_at` 与 `superseded_at` 分别唯一；RTO 必须大于 0，分片取件 SLA 为空或大于 0。`ck_deployment_records_virus_scan` 要求 NONE 时 URL 为空、CUSTOMER_ICAP 时 URL 非空，应用层再拒绝非 `icap` scheme、主机名、重定向与非回环 IP。仅 `superseded_at` 可由 infinity 单向更新为同事务新活动行的 created_at，新 revision 必须为旧值+1；延迟图保证提交时恰一活动行，禁止无后继闭合与回开。

### 7.3 offsite_sinks（服务器之外落点，受控版本行）

自有列：`sink_kind`（`LOCAL_DIR|NFS_SMB_MOUNT|OBJECT_STORAGE`）、`root_ref text`、`media_type`（`ONLINE|OFFLINE|NONE`）、`rotation_period_minutes int null`、`writability`（`WRITABLE|UNWRITABLE|UNKNOWN`）、`writability_changed_at`、`req_online`、`req_auto_write`、`req_failure_detectable`、`access_control_attested boolean default false`、`access_control_attested_at null`、`access_control_evidence_ref null`、`writer_identity_ref text null`、`restore_identity_ref text null`、`disposal_identity_ref text null`、`append_only_attested boolean default false`、`append_only_attested_at timestamptz null`、`append_only_evidence_ref text null`、`append_only_probe_at timestamptz null`、`append_only_probe_result text default 'UNKNOWN'`（`PASS|FAIL|UNKNOWN`）、`readback_throughput_mibps numeric(18,6) null`、`write_throughput_mibps numeric(18,6) null`、`throughput_measured_at null`、`superseded_at default 'infinity'`。三个 identity_ref 是经清洗的账户/角色标识而非 secret ref，media_type 非 NONE 时必须存在且两两不同；append_only_attested=true 时证据、时点与 PASS 结论必须齐全。OFFLINE 必须给轮换周期；NONE 时三项要求均为 false；`created_at` 与 `superseded_at` 分别唯一。仅 superseded_at 可单向闭合到同事务新活动行的 created_at，延迟图保证提交时恰一活动行。全部对象使用批次唯一不可复用 key 与 CREATE_NEW/`If-None-Match: *`；writer 仅列举、创建新对象与必要校验读，restore 平时封存且只读，disposal 为第三身份并只在双人审批及重新认证后临时解封。对象存储 IAM、Windows/SMB DACL 或经认证 NFSv4 ACL 的覆盖、删除、重命名、改权/策略负向探针任一未被拒即记 FAIL 并打开不可抑制 `OFFSITE_COPY_PROTECTION_MISSING`；普通 POSIX/NFS 可写目录若不能分离创建与删除权限，不满足保护门。该字段组只证明最小权限防删，不代表 WORM；客户存储管理员仍可绕过。

### 7.4 writeout_runs（写出批次，仅追加）

自有列：`channel`（`WAL_ARCHIVE|ATTACHMENT_INCREMENTAL|ATTACHMENT_FULL|AUDIT_EVIDENCE|FULL_BACKUP|CONFIG_BUNDLE|ATTACHMENT_BOOTSTRAP`）、`writer_process`（`archive-writer|backup-writer`）、`sink_id`、`period_seq bigint`、`started_at`、`finished_at`、`outcome`（`OK|FAILED|ABORTED`）、`bytes_written bigint default 0`、`object_count int default 0`、`failure_category`（可空，`SINK_UNWRITABLE|ENCRYPTION|CHECKSUM|SOURCE_READ|QUOTA|OTHER`）、`last_error null`、`report_id uuid`。`sink_id` 真实 FK 到 offsite_sinks；`report_id` 唯一，`(channel,period_seq)` 唯一；092500 强制 run 只在结束后一次插入、finished>=started、计数非负、OK 与失败字段同空、失败/中止与失败字段同非空。本表 APPEND_ONLY、无业务冲销语义、不带 `reverses_id`。

### 7.5 attachment_watermarks（附件写出水位，仅追加）

自有列：`watermark_at`、`pending_object_count int`、`oldest_pending_committed_at null`、`bootstrap_state`（`NOT_STARTED|RUNNING|DONE`）、`bootstrap_remaining_bytes bigint default 0`、`manifest_ref text`、`sink_id uuid`、`advanced_at`、`report_id uuid`。`sink_id` 真实 FK 到 offsite_sinks；`report_id` 唯一；本表 APPEND_ONLY，按 `watermark_at` 与 `advanced_at` 建索引。

### 7.6 backup_sets（备份集，可更新）

自有列：`kind`（`DAILY_FULL|CHAIN_REBUILD_BASELINE|CONFIG_BUNDLE|ATTACHMENT_FULL`）、`state`（`PLANNED|RUNNING|WRITTEN|VERIFIED|VERIFY_FAILED|ABORTED|DISPOSED`）、`sink_id`、`writeout_run_id null`、`started_at null`、`written_at null`、`verification_concluded_at null`、`verified_at null`、`aborted_at null`、`disposed_at null`、`disposed_from_state`（可空，`VERIFIED|VERIFY_FAILED|ABORTED`）、`disposal_certificate_ref null`、`bytes null`、`base_lsn null`、`backup_label_ref null`、`manifest_ref null`、`encryption_key_ref text`、`spill_peak_bytes null`、`abort_reason`（可空，`SPILL_LIMIT|SINK_UNWRITABLE|SOURCE_ERROR|SUPERSEDED`）。sink/writeout 为真实 FK；七态逐态形状、`PLANNED→RUNNING→WRITTEN→VERIFIED|VERIFY_FAILED`、`RUNNING→ABORTED`、三结果态到 DISPOSED 的单向边，以及按 kind 的精确 writeout channel/校验方法集合，由 Stage 14 §3.1.1 DEFERRABLE 图强制。PLANNED 只可在启动前调整 kind/sink/encryption，RUNNING/WRITTEN 禁止同态改写业务列，结果态除单向处置外不可变，DISPOSED 全不可变。PLANNED/RUNNING/WRITTEN/ABORTED 零校验行；VERIFIED/VERIFY_FAILED 一次具备完整必需集合，concluded_at 等于最大 finished_at，VERIFIED 的 verified_at 同值。按种类/开始时间与状态/开始时间建索引。

### 7.7 backup_runner_slot（备份串行槽，可更新单例）

主键固定 `00000000-0000-0000-0000-0000000000b1`；自有列仅 `current_backup_set_id uuid null`，为指向 backup_sets 的 DEFERRABLE FK，非空当且仅当所指备份 RUNNING。每日全量与断链重建基线均以该行乐观锁串行化；单例随 090700 建表插入，早于建表的 090000 是 no-op。

### 7.8 backup_verifications（备份校验结论，仅追加）

自有列：`backup_set_id`、`method`（`MANIFEST_CHECKSUM|DECRYPT_READBACK|PG_VERIFYBACKUP|ATTACHMENT_CHECKSUM`）、`started_at`、`finished_at`、`outcome`（`PASS|FAIL`）、`bytes_read bigint`、`mismatched_object_count int default 0`、`detail jsonb default '{}'`、`report_id uuid`。backup_set_id 为 DEFERRABLE FK；`report_id` 与 `(backup_set_id,method)` 各自唯一，计数非负、结束不早于开始、started_at 不早于 backup.written_at、PASS 时 mismatch=0；本表 APPEND_ONLY，按备份集与开始时间建索引。

### 7.9 archive_channel（归档通道，可更新单例）

主键固定 `00000000-0000-0000-0000-0000000000a1`。自有列：`state`（`HEALTHY|RETENTION_WARNING|SLOT_INVALIDATED|REBUILDING|SUSPENDED`）、`slot_name`、`slot_active`、`confirmed_flush_lsn null`、`broken_at null`、`break_cause`（可空，`SLOT_WAL_LIMIT|WRITER_STOPPED|WRITER_NOT_ADVANCING|SINK_UNWRITABLE`）、`rebuild_backup_set_id null`、`restored_at null`、`last_transition_id null`、`replication_check_last_outcome`（可空，`MATCHED|MISMATCHED|NO_RESULT`）、`replication_check_last_at null`、`replication_check_no_result_streak smallint default 0`、`replication_check_last_error_code null`。rebuild backup 为 DEFERRABLE FK；`(id,last_transition_id)` 以真实复合 FK 指向 transitions 的 `(archive_channel_id,id)` 候选键。初始 seed 固定 row_version=1、HEALTHY、slot_name=`ep_archive_slot`、slot_active=true、streak=0，其余业务可空列全空且无版本证据；其后每版必须有连续 `STATE_CHANGE|OBSERVATION` 证据并由 last_transition_id 指向最后一行。typed after-image 必须可从固定初始行重建全部历史版本，恢复基线类型/状态与逐态形状按 Stage 14 §3.1.1。单例随 090900 建表插入。

### 7.10 archive_channel_transitions（归档通道版本证据，仅追加）

自有列：`archive_channel_id`（固定单例 id）、`transition_kind`（`STATE_CHANGE|OBSERVATION`）、`from_row_version bigint`、`to_row_version bigint`、`from_state`、`to_state`，以及当前单例业务列的 typed after-image：`to_slot_name`、`to_slot_active`、`to_confirmed_flush_lsn null`、`to_broken_at null`、`to_break_cause null`、`to_rebuild_backup_set_id null`、`to_restored_at null`、`to_replication_check_last_outcome null`、`to_replication_check_last_at null`、`to_replication_check_no_result_streak`、`to_replication_check_last_error_code null`；其后为 `cause`、`occurred_at`、`detail jsonb default '{}'`、`report_id uuid`。`to_row_version=from_row_version+1`，streak 非负；report_id 与 `(archive_channel_id,to_row_version)` 唯一，另有 `(archive_channel_id,id)` 候选键；archive channel 和 rebuild backup 都是真实 DEFERRABLE FK。本表 APPEND_ONLY，与当前单例组成 DEFERRABLE 逐版本证据图。STATE_CHANGE 只允许 Stage 14 §4.2 展开的九个 from/to 对；OBSERVATION 同态且生命周期列不可改，只允许 LSN 非递减与复制核对四字段更新。核对结论空值/MATCHED|MISMATCHED/NO_RESULT 三种形状、at 严格递增、NO_RESULT streak 首次为 1 后逐次加一及有结论归零均由同一图强制，STATE_CHANGE 无新结论时不得偷清 streak。detail 不是重放输入；按发生时间建索引。

### 7.11 replication_reports（复制生命周期上报，仅追加）

自有列：`writer_process`（`archive-writer|backup-writer`）、`db_role`（`ep_archiver|ep_backuper`）、`report_kind`（`CONN_ESTABLISHED|CONN_CLOSED|SLOT_CREATED|SLOT_INVALIDATED|BASEBACKUP_STARTED|BASEBACKUP_FINISHED`）、`slot_name null`、`backend_pid null`、`occurred_at`、`outcome`（`OK|FAILED`）、`report_id uuid`、`spooled boolean default false`。`report_id` 唯一；本表 APPEND_ONLY，补写行仍按 `occurred_at` 判时序。

### 7.12 wal_retention_samples（WAL 保留量采样，可清理）

自有列：`sampled_at`、`slot_name`、`retained_bytes bigint`、`max_slot_wal_keep_bytes bigint`、`retention_ratio numeric(9,6)`、`pg_wal_bytes bigint`。按采样时间建索引，保留 90 天，超期只走基线允许的指标快照清理路径。

### 7.13 capacity_samples（容量水位采样，可清理）

自有列：`sampled_at`、`component`（`ATTACHMENT_CURRENT|ATTACHMENT_HISTORY|DB_DATA|ARCHIVE_LOCAL|BASEBACKUP_SPILL|SEARCH_AND_TEMP`）、`used_bytes bigint`、`floor_bytes bigint`、`ratio numeric(9,6)`。按采样时间建索引，保留 400 天。

### 7.14 key_recovery_materials（密钥恢复材料登记，可更新）

`security_level` 固定 40，只登记元数据，不存材料。自有列：`material_kind`（`TENANT_ROOT|LEGAL_ENTITY_KEY_DOMAIN|DEPLOYMENT_BACKUP_ENCRYPTION_KEY`）、`scope_ref null`、`carrier`（`BUILTIN_KMS|CUSTOMER_HSM`）、`shard_count smallint`、`shard_locations jsonb`、`dual_control_authorizers jsonb`、`last_verified_at null`、`next_verification_due_on date`、`verification_method text`、`stored_with_protected_copy boolean default false`。分片至少 2，且不得与所保护副本同落点。

### 7.15 key_recovery_verifications（密钥恢复核验，仅追加）

自有列：`key_recovery_material_id`、`performed_at`、`performed_by_party`（`CUSTOMER_OPS|CUSTOMER_PER_CONTRACT`）、`outcome`（`PASS|FAIL`）、`isolated_env_ref`、`approval_ref`、`report_ref`。材料为真实 FK；本表 APPEND_ONLY，按材料与执行时间建索引。

### 7.16 recovery_drills（恢复演练与真实恢复，可更新）

自有列：`drill_kind`（`WHOLE_MACHINE_RECOVERY|KEY_MATERIAL_ISOLATED_RECOVERY|PRODUCTION_RECOVERY`）、`backup_selection`（`LATEST_VERIFIED|RETENTION_TAIL`）、`state`（`RUNNING|PASSED|FAILED`）、`attempt_no int`、`window_started_at`、`window_ended_at null`、`sink_id`、`backup_set_id`、`backup_verified_at_at_start`、`retention_days_at_start smallint`、`sink_kind_at_drill`、`readback_throughput_mibps null`、`rto_seconds null`、`rpo_db_seconds null`、`rpo_attachment_seconds null`、`shard_pickup_seconds null`、`attachment_check_total null`、`attachment_check_failed null`、`attachment_check_seconds null`、`invariant_check_batches null`、`invariant_check_max_batch_seconds null`、`invariant_check_total_seconds null`、`invariant_check_mem_peak_bytes null`、`invariant_check_tempfile_peak_bytes null`、`decrypt_seconds null`、`outcome`（可空，`PASS|FAIL`）、`failure_stage`（可空，`READBACK|KEY_SHARD_PICKUP|DECRYPT|ATTACHMENT_CHECK|INVARIANT_CHECK|RPO_EVALUATION|RTO_EVALUATION|OTHER`）、`failure_code null`、`report_ref null`。sink/backup 为 DEFERRABLE FK；所指备份开始时必须 VERIFIED 且 kind 只可 DAILY_FULL 或 CHAIN_REBUILD_BASELINE；`attempt_no>=1`、retention days>=1，`(drill_kind,backup_selection,attempt_no)` 唯一。RUNNING 到 PASSED/FAILED 的逐态形状、开始时 VERIFIED 快照、事后处置兼容与 retention-tail 年龄/RPO 规则按 Stage 14 §3.1.1；WHOLE/PRODUCTION 的 PASSED 具备完整数据恢复指标且 rto<=14400、LATEST_VERIFIED 两项 RPO 成对非空且各<=900，FAILED 必须具备失败阶段/稳定码且附件三列、不变量五列、RPO 两列分别全空或全非空，其他单值只保存实际完成项；KEY_MATERIAL_ISOLATED 始终只允许 shard-pickup/decrypt 指标，PASSED 两项必填，FAILED 允许按执行进度为空但 decrypt 非空时 shard 必非空，其余指标全空；终态不可变。

### 7.17 alert_suppressions（告警抑制动作，仅追加）

自有列：`degradation_window_id`、`action`（`SUPPRESS|UNSUPPRESS`）、`acted_at`、`acted_by`、`until_at null`、`reason text`、`approval_ref null`。window 为真实 FK；reason 不超过 2000 字；本表 APPEND_ONLY，按窗口与动作时间建索引。不可抑制窗口的写入由服务与 `degradation_windows` CHECK 双重拒绝。

### 7.18 data_migration_batches（历史迁移批次，可更新、法人 RLS）

自有列：`batch_no`、`source_kind`（`XLSX_CSV|ODBC|FILE_MANIFEST|HTTPS_API`）、`source_system_ref`、`source_schema_fingerprint bytea`、`source_readonly_test_ref null`、`template_code`、`template_version`、`template_sha256 bytea`、`status`（`DRAFT|APPROVED|TRIAL_RUNNING|TRIAL_FAILED|TRIAL_PASSED|SOURCE_FROZEN|APPLYING|DELTA_CATCHUP|RECONCILING|READY_FOR_CUTOVER|CUTOVER_COMPLETED|REVERSAL_PENDING|REVERSED|CANCELLED`）、`task_available_at default now()`、`task_locked_by null`、`task_locked_until null`、`task_attempts int default 0`、`ledger_scope jsonb default '[]'`、`warehouse_scope jsonb default '[]'`、`required_reconciliation_keys jsonb`、`source_module_codes text[]`、`window_starts_at`、`window_ends_at`、`data_owner_id`、`customer_finance_owner_id`、`content_version bigint default 1`、`approval_content_hash bytea`、`current_run_no int default 0`、`trial_pass_count smallint default 0`、`trial_nonconvergent_count smallint default 0`、`source_frozen_at null`、`source_readonly_evidence_ref null`、`delta_watermark null`、`source_manifest_sha256 bytea null`、`source_record_count bigint null`、`trial_report_ref null`、`reconciliation_digest bytea null`、`final_reconciliation_report_ref null`、`cutover_content_hash bytea null`、`cutover_at null`、`reversal_batch_ref null`、`reversal_reason null`、`reversal_content_hash bytea null`、`cancelled_from_status`（可空，`DRAFT|APPROVED|TRIAL_FAILED|TRIAL_PASSED`）、`cancelled_at null`、`cancel_reason null`。旧三项任意 approval/decision ref 不存在。两名 owner 均以同法人复合 FK 指向 user grant；七个 bytea 摘要逐个固定 32 字节；source schema 指纹由已验签模板派生，trial/freeze/apply 每次重算必须相等；模块码与 required keys 为规范排序非空集合；租约成对、计数非负、窗口有效；`(legal_entity_id,batch_no)` 与 `(legal_entity_id,id)` 为候选键。source_readonly_test_ref 在 DRAFT 可空，发起批准时写入并与 source_schema_fingerprint 一起纳入 approval_content_hash，APPROVED 及其后必非空；DRAFT 中任一审批内容输入改变必须同时 content_version+1 并重算 hash，APPROVED 后输入不可变；CANCELLED 三项证据同空同非空、原因长度 1 至 2000，保存合法前态并清租约，非 CANCELLED 三项全空；approval/cutover/reversal 三个 hash 的精确输入、逐态形状与批准集合按 Stage 14 §3.1.2。

### 7.19 data_migration_records（迁移逐记录台账，可更新、法人 RLS）

不保存来源原文、附件正文、访问凭据或可逆定位值。自有列：`batch_id`、`run_no int`、`chunk_no int`、`record_seq bigint`、`module_code`、`object_type`、`source_locator_sha256 bytea`、`source_record_sha256 bytea`、`mapped_security_level null`、`mapped_key_domain_id null`、`mapped_retention_policy_code null`、`target_object_type null`、`target_id null`、`target_record_sha256 bytea null`、`apply_receipt_id uuid null`、`reversal_receipt_id uuid null`、`status`（`QUEUED|VALIDATED|APPLIED|FAILED|REVERSED`）、`error_code null`、`sanitized_error null`、`applied_at null`。run/chunk/seq 均为正，module/object 必须为 25 个封闭组合；三类摘要固定 32 字节；映射三列全空或全非空，security level 只取 10/20/30/40，retention code 长度 1 至 64，092600 为 key_domains 补 `(legal_entity_id,id)` 候选键并以同法人 DEFERRABLE 长 FK 锁住 mapped key domain。有 `(legal_entity_id,id)` 与 `(legal_entity_id,batch_id,id)` 候选键、同法人 batch FK、普通唯一键 `ux_data_migration_records_target_reservation(legal_entity_id,target_object_type,target_id)`，以及指向表 23 同批同记录的两条 nullable DEFERRABLE 长 FK。五态形状与单向边按 Stage 14 §3.1.2：QUEUED 无映射/预留；VALIDATED 同时具备三项映射、catalog 固定 target relation 与服务端新 UUIDv7 target id，且静态延迟分支证明该同法人根尚不存在；从 VALIDATED 失败完整保留映射与预留，APPLIED 才填实际目标 hash/receipt/time。预留形成后不可改且 APPLY 必须使用该 id；APPLIED/REVERSED 不能只凭裸目标三元组成立，owner 静态投影还须证明 security、key domain 与 retention 逐项等于映射。

### 7.20 data_migration_reconciliations（迁移对账，仅追加、法人 RLS）

自有列：`batch_id`、`run_no`、`check_kind`（`COUNT|AMOUNT|RELATIONSHIP|ATTACHMENT|HASH|DEBIT_CREDIT_BALANCE|INVENTORY_CONSERVATION|OPENING_CONTINUITY|SECURITY_ASSIGNMENT`）、`scope_key`、`source_value jsonb`、`target_value jsonb`、`difference_value jsonb`、`outcome`（`PASS|FAIL|APPROVED_DIFFERENCE`）、`known_difference_id null`、`report_ref`、`checked_at`。同法人 batch FK；known difference 非空时以 `(legal_entity_id,batch_id,known_difference_id)` 长 FK 锁同批次；`APPROVED_DIFFERENCE` 与差异引用同真同假；借贷平衡、库存守恒、期初连续性、安全赋值四类只允许 PASS。`(legal_entity_id,batch_id,run_no,check_kind,scope_key)` 唯一；本表 APPEND_ONLY。

### 7.21 data_migration_known_differences（已知差异审批，可更新、法人 RLS）

自有列：`batch_id`、`module_code`、`category`（`CLOSED_PERIOD_SOURCE_IMBALANCE_OR_INCOMPLETE|CLOSED_HISTORY_SETTLED_OR_CLOSED|NONCRITICAL_MISSING_HISTORY_DETAIL|NAMED_MIGRATION_BALANCING_ENTRY`）、`ledger_or_warehouse_scope`、`source_document_scope`、`amount numeric(18,2) null`、`quantity numeric(24,6) null`、`cause`、`cannot_zero_reason`、`proposal_ref`、`data_owner_id`、`module_owner_id`、`finance_owner_id`、`content_version bigint default 1`、`approval_content_hash bytea`、`decision`（`PROPOSED|APPROVED|REJECTED|REVOKED`，默认 PROPOSED）、`decided_at null`。旧 decision/approval refs 不存在；module_code 只取 11 个迁移模块码且必须属于 batch.source_module_codes，并纳入内容 hash；三名 owner 均以同法人复合 FK 指向 user grant；hash 固定 32 字节。有 `(legal_entity_id,id)`、`(legal_entity_id,batch_id,id)` 候选键和 batch 长 FK。PROPOSED 中任一审批内容输入改变必须同时 content_version+1 并重算 hash；只允许 `PROPOSED→APPROVED|REJECTED`、`APPROVED→REVOKED`，两终态不可变；三方批准、拒绝与一一撤销证据只认表 22，逐态形状按 Stage 14 §3.1.2。

### 7.22 data_migration_approval_evidences（批准证据，仅追加、法人 RLS）

自有列：`batch_id`、`known_difference_id null`、`subject_difference_id`（生成列，NULL 取全零 UUID）、`phase`（`BATCH_APPROVAL|KNOWN_DIFFERENCE_DECISION|CUTOVER_APPROVAL|REVERSAL_APPROVAL`）、`decision`（`APPROVED|REJECTED|REVOKED`）、`reauth_purpose`（生成列；已知差异撤销取 `KNOWN_DIFFERENCE_REVOCATION`，其余取 phase）、`approver_kind`（`DATA_OWNER|MODULE_OWNER|FINANCE_OWNER|SECOND_APPROVER`）、`module_code null`、`subject_module_code`（生成列，NULL 取 `-`）、`approver_role_id`、`approver_role_code`、`approver_role_grant_id`、`approver_grant_effective_from date`、`approver_grant_effective_to_at_decision date null`、`content_version`、`content_hash bytea`、`process_instance_id`、`process_task_id`、`reauth_challenge_id uuid`、`definition_id`、`definition_code`、`definition_version`、`definition_hash`、`submitted_by/at`、`decided_by/at`、`reverses_evidence_id null`。hash 固定 32 字节、definition hash 为 64 位小写十六进制、申请人不得自审；module 与 known-difference 两组 nullable 形状、REVOKED 反向形状按 Stage 14 §3.1.2。batch/difference/submitted/decided user 有逐条同法人真实 FK，`reauth_challenge_id -> platform_core.reauth_challenges(id) ON DELETE RESTRICT`；092600 为 platform_flow 三父表补候选键后，以 instance+definition 五列、instance+task 三列、definition 五列三条 DEFERRABLE 长 FK 锁死流程归属与发布定义；另为 roles 建 `(legal_entity_id,id,code)`、为 user_role_grants 建 `(legal_entity_id,user_id,role_id,id,effective_from)` 候选键，并由表 22 以角色三列及决定者+角色+grant+effective_from 五列两条 DEFERRABLE 长 FK 锁死角色身份与授权。effective_to 保存插证时快照，决定日必须在有效期内；后续正常到期不反向作废不可变历史证据。DATA_OWNER/FINANCE_OWNER/SECOND_APPROVER 分别固定 `OPS_DATA_OWNER`/`FINANCE_MANAGER`/`SECURITY_ADMIN`，11 个 MODULE_OWNER 模块使用 Stage 14 §3.1.2 的静态全映射，流程 candidate_role_codes 必须为对应单元素数组。task.reauth_ref 必须等于 reauth_challenge_id；挑战必须为已消费的 `HIGH_RISK_REAUTH+DATA_MIGRATION`、user_id=submitted_by 且 consumed_at 不晚于流程实例开始，并以 `SHA-256(RFC8785({operation_type:'DATA_MIGRATION',legal_entity_id,batch_id,known_difference_id,reauth_purpose,content_version,content_hash}))` 精确绑定主体；对象恰含七键，UUID 取 RFC 9562 小写连字符字符串，content_version 取 JSON 整数，content_hash 取 64 位小写 hex，批次 phase 的 known_difference_id 取 JSON null，结果为 32 原始字节。已知差异初次决定与撤销使用不同 reauth_purpose，禁止跨动作或旧版本重放。另有 `UNIQUE NULLS NOT DISTINCT (legal_entity_id,batch_id,phase,known_difference_id,content_version,approver_kind,module_code,decision)`，以及包含两个生成哨兵与 id 的 reversal parent 候选键；REVOKED 用相同长列加 reverses_evidence_id 建真实自 FK，nullable 不会令 FK 静默跳过。流程 variables 只含七个低敏路由键，函数从 task.reauth_ref 派生挑战、从当前业务行重算 hash并派生角色授权快照；本场景不建 approval command snapshot，也不扩展 ModuleCode。本表 APPEND_ONLY，直接 INSERT 不授 ep_app_rw，只由标准流程 callback 调用 `platform_ops.record_data_migration_approval_evidence(legal_entity_id,process_instance_id,process_task_id)` 写入。

### 7.23 data_migration_writer_receipts（模块写入者回执，仅追加、法人 RLS）

自有列：`batch_id`、`record_id`、`run_no`、`module_code`、`object_type`、`effect_kind`（`APPLY|REVERSE`）、`target_object_type`、`target_id`、`target_record_sha256 bytea`、`writer_contract_version`、`effect_sha256 bytea`、`idempotency_key`、`owner_effect_at`、`reverses_receipt_id null`。摘要固定 32 字节、contract version 正数；有 `(legal_entity_id,id)` 与 `(legal_entity_id,batch_id,record_id,id)` 候选键、同法人 batch/record 长 FK、REVERSE 到本记录 APPLY 的长自 FK，全部 DEFERRABLE；`(legal_entity_id,record_id,effect_kind)` 唯一。APPLY target type/id 必须等于 VALIDATED 预留，hash/time 与 APPLIED 记录逐列一致。REVERSE 只允许下表逐对象固定的三种封闭 effect：交易取消/冲销/更正新 fact；具名不可变 version/change fact 加原根 after-image；以及仅采购订单、付款申请、资金账户三支可用的具名独立 owner audit change fact 加原根 after-image。第三种 target 固定为 `platform_audit.audit_events.event_id`，owner event 与 R0 必须 event_id 不同、同法人同 occurred_at，三个 action 与 before/after 状态版本图逐字固定；普通状态审计不满足。全部 REVERSE 指回同记录 APPLY，并另有 `platform_audit.audit_events.event_id=REVERSE receipt id` 的 R0，action 固定 DATA_MIGRATION_REVERSED、对象指原 APPLY 根、after 恰含 Stage 14 §4.12.1 六键。idempotency_key 精确为 `dm:v1:` 加 `{legal_entity_id,batch_id,module_code,object_type,source_locator_sha256,effect_kind}` 的 RFC 8785/SHA-256 小写 hex，不含 run_no；effect_sha256 的十三键规范对象、UUID/hash/null 编码按 Stage 14 §3.1 表 23，结果为 32 原始字节。本表 APPEND_ONLY，直接 INSERT 不授 ep_app_rw。executor 调用受控 SECURITY DEFINER 精确 ABI `record_data_migration_writer_receipt(legal_entity_id,receipt_id,record_id,effect_kind,target_object_type,target_id,writer_contract_version,idempotency_key,reverses_receipt_id)`；函数先重算 key，再按下列 25 行生成固定 relation/join/order 分支，从实际同法人目标重算 target/effect 摘要，不信调用方摘要、不运行动态 SQL、不查询 information_schema。APPLY owner_effect_at 取数据库时钟；REVERSE 精确取 R0.occurred_at。与表 19 指针构成双向恰一图，不要求异构目标表新增统一 provenance 列。

静态 projection 算法固定为 Stage 14 §4.12.1 的 `row_v1/set_v1`：每个 relation 的行保留 id、法人、安全属性、创建证据与全部业务列，只删除 row_version/updated_at/updated_by；每个 child 集按 id UUID bytes 升序，空集为 `[]`；顶层五键、RFC 8785 与 SHA-256 不得变体。下表“provenance”中的 A0 指 VALIDATED 空根预留，R0 指同号不可变反向审计；根 after-image 通常必须另有表内具名 version/change fact，唯一例外是下表三条明确写出固定 action 的独立 owner audit target，且同样必须静态投影并核实根最终 after-image。

| object kind | owner/writer | APPLY root relation | APPLY children / provenance | REVERSE owner effect target |
|---|---|---|---|---|
| `mdm.customer_bundle` | mdm / ep-app-mdm | `mdm.customers` | contacts、addresses、invoice_profiles 按 customer_id；customer_attachments 按 owner_id；A0 | `mdm.record_versions`，DEACTIVATION change request + customer after-image + R0 |
| `mdm.supplier_bundle` | mdm / ep-app-mdm | `mdm.suppliers` | contacts、payment_profiles、qualifications、price/leadtime/risk records 按 supplier_id；三类 attachments 按 root/qualification/risk owner ids；A0 | `mdm.record_versions`，DEACTIVATION change request + supplier after-image + R0 |
| `mdm.material_bundle` | mdm / ep-app-mdm | `mdm.materials` | material_attachments.owner_id；A0 | `mdm.record_versions`，DEACTIVATION change request + material after-image + R0 |
| `mdm.product_bundle` | mdm / ep-app-mdm | `mdm.products` | product_material_links.product_id、product_attachments.owner_id；A0 | `mdm.record_versions`，DEACTIVATION change request + product after-image + R0 |
| `mdm.warehouse` | mdm / ep-app-mdm | `mdm.warehouses` | 无 children；A0 | `mdm.record_versions`，仓库守卫后的 DEACTIVATION + root after-image + R0 |
| `cpq.price_list_bundle` | cpq / ep-app-cpq | `cpq.price_lists` | lines/customer_links.price_list_id；A0 | `mdm.record_versions`，PRICE_LIST DEACTIVATION + price_list after-image + R0 |
| `clm.contract_bundle` | clm / ep-app-clm | `clm.contracts` | lines、terms、milestones、obligations、payment_schedules、attachments、annotations、versions 按 contract_id，merge_links 按 source/target；A0 | `clm.contract_versions`，VOID/termination/终态 reversal version + root after-image + R0 |
| `sales.sales_order_bundle` | sales / ep-app-sales | `sales.sales_orders` | lines、schedules、confirmations/lines、versions、changes/change_lines；A0 + contract/version truth | `sales.sales_order_changes`，既有 CANCEL/CLOSE 且已交付先生成 sales returns + R0 |
| `sales.sales_return_bundle` | sales / ep-app-sales | `sales.sales_returns` | lines、delivery_links、capture_allocations、exchange_links；A0 + order/delivery/live capture | `sales.delivery_confirmations`，REGISTERED/CLOSED 补偿交付确认；未登记走既有 CANCEL；R0 |
| `procure.purchase_order_bundle` | procure / ep-app-procure | `procure.purchase_orders` | lines、line_batches、payment_plans、attachments；A0 | `platform_audit.audit_events.event_id` owner event target，action=`PROCURE_PURCHASE_ORDER_MIGRATION_REVERSED`；before/after exact schema_version+row_version+status，row_version=规范十进制 JSON string，Stage 7 逐态 VOID/CLOSE/终态保持；owner audit + purchase_order after-image + 独立同 occurred_at R0 |
| `procure.goods_receipt_bundle` | procure / ep-app-procure | `procure.goods_receipts` | lines、serials、costings、attachments、rejections/rejection attachments；A0 + PO ancestry | `procure.purchase_returns`，既有完整退货通道 + R0 |
| `procure.purchase_return_bundle` | procure / ep-app-procure | `procure.purchase_returns` | lines、serials、attachments、对应 supplier_quality_records；A0 + receipt/cost/credit-note ancestry | `procure.goods_receipts`，补偿收货并按需反向 invoice reversal + R0 |
| `procure.payment_request_bundle` | procure / ep-app-procure | `procure.payment_requests` | lines、attachments；payable_reservations 排除；A0 | `platform_audit.audit_events.event_id` owner event target，action=`PROCURE_PAYMENT_REQUEST_MIGRATION_REVERSED`；before/after exact schema_version+row_version+status，row_version=规范十进制 JSON string，Stage 7 逐态 VOID/WITHDRAW/CLOSE/终态保持并释放 reservation；owner audit + payment_request after-image + 独立同 occurred_at R0 |
| `inventory.stock_opening` | inventory / ep-app-inventory | `inventory.stock_movements` | qty/value/variance/serial movement facts；A0 + `IN/MIGRATION_OPENING/MIGRATION_STOCK_ADJUSTMENT/migration`，source id=record | 新反向 `inventory.stock_movements`，逐段数量/金额/序列镜像 + R0 |
| `inventory.stock_history` | inventory / ep-app-inventory | `inventory.stock_movements` | 同 stock_opening；A0 + direction 三值之一、`MIGRATION_HISTORY/MIGRATION_STOCK_HISTORY/migration`，source id=record | 新反向 `inventory.stock_movements`，pricing_branch=MIGRATION_HISTORY + R0 |
| `ledger.opening_balance` | ledger / ep-app-ledger | `ledger.opening_balance_batches` | batch_lines；A0 + source=MIGRATION_BATCH、migration_batch_no=batch_no | 新 `ledger.vouchers`，按 opening lines 逐科目完整镜像 + R0 |
| `ledger.historical_voucher` | ledger / ep-app-ledger | `ledger.vouchers` | voucher_lines；A0 + HISTORICAL_MIGRATION/DATA_MIGRATION_RECORD、source id=record/no=batch | 新 HISTORICAL_MIGRATION 镜像 voucher，头行 reverses_id 指原图 + R0 |
| `finance.open_items_opening` | finance / ep-app-finance | 四选一：`receivable_entries`、`payable_entries`、`advance_receipt_entries`、`advance_payment_entries` | 无 children；A0 + MIGRATION_OPENING，advance source id=record | 所选台账的新 migration-opening reversal/effect，父 id 指原 entry + R0 |
| `finance.cash_account_opening` | finance / ep-app-finance | `finance.cash_accounts` | 无 children；A0 | `platform_audit.audit_events.event_id` owner event target，action=`FINANCE_CASH_ACCOUNT_MIGRATION_REVERSED`；before/after exact schema_version+row_version+is_active+deactivated_at，row_version=规范十进制 JSON string，无未结资金事实，active 停用/已 inactive 保持；owner audit + cash_account after-image + 独立同 occurred_at R0 |
| `invoice.sales_invoice_bundle` | invoice / ep-app-invoice | `invoice.sales_invoices` | lines、receipt_plan_links、number_registry、application 及两类 links、sales_invoice_attachments；A0 | `invoice.invoice_reversals` OUTPUT，含 lines/number/attachments + R0 |
| `invoice.purchase_invoice_bundle` | invoice / ep-app-invoice | `invoice.purchase_invoices` | lines、number_registry、具名 purchase_invoice_attachments；A0 | `invoice.invoice_reversals` INPUT，含 lines/number/attachments + R0 |
| `project.project_bundle` | project / ep-app-project | `project.projects` | tasks、project/task attachments、task-requisition links；A0 | 新 `project.project_migration_corrections` target；project_id=APPLY root，CLOSE/RETAIN_CLOSED；correction + project/task after-images + R0 |
| `service.customer_complaint_bundle` | service / ep-app-service | `service.customer_complaints` | customer_complaint_attachments；A0 | 新 `service.customer_complaint_migration_corrections` target；complaint_id=APPLY root，CANCEL/RETAIN_TERMINAL；correction + complaint after-image + R0 |
| `service.equipment_bundle` | service / ep-app-service | `service.equipment_records` | equipment_attachments；A0 + source=MIGRATION、migration_batch_no=batch_no | 新 `service.equipment_migration_corrections` target；equipment_record_id=APPLY root，SET_RETURNED/RETAIN_TERMINAL；correction + equipment after-image + R0 |
| `service.work_order_bundle` | service / ep-app-service | `service.work_orders` | lines、logs、work_order_attachments、work_order_line_attachments；A0 | 新 `service.work_order_logs` CORRECTION + work_order CANCEL/terminal after-image + R0 |

092500 安装 `platform_ops.assert_backup_evidence_graph_consistent()`，附着 writeout_runs、backup_sets、backup_runner_slot、backup_verifications、archive_channel、archive_channel_transitions、recovery_drills 七表的 INSERT/UPDATE/DELETE `CONSTRAINT TRIGGER DEFERRABLE INITIALLY DEFERRED`。092600 安装 `platform_ops.assert_data_migration_evidence_graph_consistent()`，附着表 18 至表 23 的同类延迟约束触发器，并创建精确 ABI `record_data_migration_approval_evidence(p_legal_entity_id uuid,p_process_instance_id uuid,p_process_task_id uuid) RETURNS uuid` 与 `record_data_migration_writer_receipt(p_legal_entity_id uuid,p_receipt_id uuid,p_record_id uuid,p_effect_kind text,p_target_object_type text,p_target_id uuid,p_writer_contract_version int,p_idempotency_key text,p_reverses_receipt_id uuid) RETURNS uuid` 两个 SECURITY DEFINER 受控写函数。批准函数不接受 reauth 入参，只从 task.reauth_ref 派生、锁定并验证 CONSUMED 的 DATA_MIGRATION challenge 及其精确 subject digest。两函数 owner 为 ep_mod_platform_ops，固定 search_path，只向 ep_app_rw 授 EXECUTE；两图均按最终事务快照锁父行并检查双侧，普通 FK 命中但错父属、错状态段、错角色授权、APPLY 未命中预留或 REVERSE 缺 catalog owner effect/R0 仍拒绝。092600 还按 inventory §13 与 Stage 9 冻结值增加 MIGRATION_HISTORY 与 HISTORICAL_MIGRATION owner source，并 seed 下述 data-migration permission/binding；三组变更均须在其他状态图启用前完成，rollback 要求对应事实为空后才可撤销。

### 7.24 platform_ops 视图

固定五个：`v_degradation_open`（未关闭窗口）、`v_rpo_status`（DATABASE 与 ATTACHMENT 两行 RPO 结论）、`v_backup_last_success`（各备份种类最近当前 VERIFIED；DISPOSED 不再入选）、`v_capacity_current`（六组件最新容量）、`v_ops_health`（前四者的健康聚合）。`ep_ops_ro` 只获这五个视图的 SELECT，不获基表权限；六张迁移表仍在 core-server 内经 RLS 与 ABAC 访问。

## 8. platform_authz schema（阶段 4）

本 schema 终态承载十六张授权表：阶段 4 交付的十五张，加 F-55 的 `mcp_human_grants` 一张。除 `permission_items` 与 `object_scope_bindings` 两张不带法人列（行集合对两个法人取值相同，登记于 `unpoliced_table_registry`，可见性由授权判定第二阶段的对象级判定承担）外，其余十四张逐张经 `platform_core.apply_le_rls` 生成行级策略，不写手工变体；F-55 表逐列见 `docs/data-dictionary/ai_mcp.md`。

### 8.1 permission_items（权限项，阶段 4）

Stage 13 `V20261022090600` 必须幂等 seed 恰 30 个固定管理权限项：id 为 `00000000-0000-7000-8000-000000000320` 至 `...0349`，code、`allowed_actions`、`object_type` 及 id 的逐行 exact mapping 以 Stage 13 计划 §3.4 的 30 行表为唯一目录；每行 `module_code='platform',function_point=code,description=null`，action 按全局顺序 canonical 保存。迁移逐行 `ON CONFLICT DO NOTHING` 后重读并断言 id/code/module/function/actions/object/description 全字段等于冻结值，缺/多/漂移均失败；不 seed 任何 `role_permission_grants`。Stage 13 的每条固定 API route 必须登记该表冻结的 `(permission code,Action)`，不得只查 code 或默认选 allowed_actions 的任一值。

不带 `legal_entity_id`、不建策略，登记于表十三。自有列：`code`（唯一，形如 `sales.sales_order`，正则 `^[a-z][a-z0-9_]*(\.[a-z][a-z0-9_]*)+$` 且长度不超过 128）、`module_code`（15 个模块码或 platform）、`function_point`、`allowed_actions`（非空子集，恰取 `VIEW`、`CREATE`、`UPDATE`、`SUBMIT`、`APPROVE`、`EXPORT` 六动作）、`object_type`、`description`（可空）。约束 `ck_permission_items_forbidden_codes` 拒写以 `platform.legal_entity_isolation` 与 `platform.direct_db_access` 两前缀开头的 code，关闭或修改法人隔离机制与直连业务库两类权限项写不进本表。索引 `ix_permission_items_module_code`。

Stage 14 的 092600 固定增加一行：`id=00000000-0000-7000-8000-000000000315`、`code=platform.data_migration`、`module_code=platform`、`function_point=历史数据迁移批次管理`、`allowed_actions=[VIEW,CREATE,UPDATE,SUBMIT]`、`object_type=platform.data_migration`。全部历史迁移 route 只用这一项，授权 object id 一律取 batch id；四个职责名称不是四个额外 permission。迁移 `ON CONFLICT DO NOTHING` 后逐字段断言，任一已存字段不同即失败；不自动 seed role_permission_grants。

### 8.2 object_scope_bindings（对象范围锚登记，阶段 4）

同一 090600 幂等 seed 恰 12 个固定 binding：id 为 `00000000-0000-7000-8000-000000000520` 至 `...0531`，按 object type lexicographic 顺序逐行映射 `platform.brand_profiles|client_releases|config_edit_locks|config_package_items|config_packages|config_release_orders|custom_fields|custom_indexes|custom_objects|ddl_plans|extensions|ui_layouts` 到 Stage 13 §3.4 同名表。每行 `schema_name='platform_meta'`、四个 scope anchor 恰为 SQL NULL、`security_level_col='security_level'`；`ON CONFLICT DO NOTHING` 后逐字段断言 id/object/schema/table/四锚/security 。动态 ext 对象只在 PENDING_DDL→ACTIVE 发布事务以确定性 id 同时生成 `ext.object.<object-code>` permission 与 `ext.<object-code>` binding，owner anchor=`created_by`、其余锚 null、security=`security_level`；退役保留目录/历史授权但撤路由，并由授权 guard 拒绝新 grant，不得删行或重用 code/id。

不带 `legal_entity_id`、不建策略，登记于表十三，是记录级判定的落点。自有列：`object_type`（唯一）、`schema_name`、`table_name`（均不可空）、`owner_user_col`、`owning_dept_col`、`project_col`、`customer_col`（四锚列均可空）、`security_level_col`（不可空，默认 'security_level'）。没有登记的对象类型在记录级判定阶段一律拒绝，不默认放行；本阶段随建表回填 platform 自身三对象（platform.user_accounts、platform.roles、platform.high_risk_requests），业务对象登记在其所属阶段。`permission_items.object_type` 不对本表建逐行外键：权限项与 binding 可由不同配置项同批发布，正确边界是配置包提交及模块启用前的集合闭包校验；校验必须证明每个会用于记录级判定的 object type 恰有一行 binding，且表与所有非空锚列真实存在，失败统一返回 `PLATFORM.AUTHZ.SCOPE_BINDING_MISSING` 并整事务回滚。

Stage 14 的 092600 固定增加一行：`id=00000000-0000-7000-8000-000000000509`、`object_type=platform.data_migration`、`schema_name=platform_ops`、`table_name=data_migration_batches`、`owner_user_col/owning_dept_col/project_col/customer_col=NULL`、`security_level_col=security_level`。批次子表先以同法人长 FK解析回 batch 后再判定，不拿 record/difference/receipt id 作为 scope anchor。该行同样先 DO NOTHING 再逐字段断言，冲突不覆盖。

### 8.3 roles（角色，档案类，阶段 4）

带 `legal_entity_id`，策略 `rls_roles_le`。角色一律按法人建立，不做跨法人全局角色。自有列：`code`（法人内唯一，正则 `^[A-Z][A-Z0-9_]{0,63}$`）、`name`（1 至 200）、`duty_class`（可空，职责角色取 `SYSTEM`、`DATA`、`SECURITY`、`AUDIT`、`KEY`、`CONFIG` 六值，业务角色为空）、`is_portal_role`（默认 false）、`lifecycle_state`（`DRAFT`、`PENDING_RELEASE`、`EFFECTIVE`、`SUPERSEDED`、`RETIRED` 五态）、`retired_at`（可空）、`is_active`（默认 true）、`deactivated_at`（可空）。

### 8.4 role_permission_grants（角色权限授予，阶段 4）

带 `legal_entity_id`，策略 `rls_role_permission_grants_le`。自有列：`role_id`、`permission_item_code`（长度 1 至 128）、`action`（六动作之一）。`(legal_entity_id,role_id)` 真实复合外键指向 `roles(legal_entity_id,id) ON DELETE RESTRICT`；`permission_item_code` 以具名真实外键 `fk_role_permission_grants_permission_item_code` 指向全局目录 `permission_items(code) ON DELETE RESTRICT`，孤儿历史行必须在 `V20261012115500__platform_authz_add_missing_foreign_keys.sql` 加约束前被前置检查定位并令迁移失败，不得静默删除。唯一索引建在 `(legal_entity_id, role_id, permission_item_code, action)` 四列上：全称 `ux_role_permission_grants_legal_entity_id_role_id_permission_item_code_action` 共 80 字节超过 63 字节标识符上限，按列序缩写为 `ux_role_permission_grants_le_id_role_id_perm_item_code_action`（61 字节）：`legal_entity_id` 缩为 `le_id`、`permission_item_code` 缩为 `perm_item_code`，此处登记全称备查。

### 8.5 access_policies（访问策略，阶段 4）

带 `legal_entity_id`，策略 `rls_access_policies_le`。自有列：`role_id`（可空，空表示适用全部角色）、`object_type`、`effect`（`ALLOW`、`DENY`）、`priority`（int 默认 100）、`condition`（jsonb，受限声明式结构：只允许对 department、position、project、customer、security_level、data_scope_tag 六属性做 in、not_in、lte、gte、has_tag 五种断言的合取，serde 强类型反序列化，不做字符串求值）、`lifecycle_state`（五态同 roles）。索引 `ix_access_policies_legal_entity_id_object_type_effect`。

### 8.6 field_permissions（字段权限，阶段 4）

带 `legal_entity_id`，策略 `rls_field_permissions_le`。自有列：`role_id`、`object_type`、`field_name`（长度均 1 至 128）、`visibility`（`HIDDEN`、`MASKED`、`READ`、`WRITE` 四值）、`mask_style`（可空，`FULL`、`KEEP_LAST_4`、`KEEP_DOMAIN` 三值）。唯一索引建在 `(legal_entity_id, role_id, object_type, field_name)` 四列上：全称 `ux_field_permissions_legal_entity_id_role_id_object_type_field_name` 共 65 字节超限，缩写为 `ux_field_permissions_le_id_role_id_obj_type_field_name`（54 字节）：`legal_entity_id` 缩为 `le_id`、`object_type` 缩为 `obj_type`，此处登记全称备查。

### 8.7 user_legal_entity_grants（用户法人授权，阶段 4）

带 `legal_entity_id`，策略 `rls_user_legal_entity_grants_le`。本表是认证中间件校 `X-Legal-Entity-Id` 与法人可见性内联的权威源。自有列：`user_id`、`granted_from`（date）、`granted_to`（date 可空）、`granted_by`。唯一约束 `ux_user_legal_entity_grants_legal_entity_id_user_id` 在 `(legal_entity_id, user_id)` 上。

### 8.8 user_role_grants（用户角色授予，阶段 4）

带 `legal_entity_id`，策略 `rls_user_role_grants_le`。自有列：`user_id`、`role_id`、`effective_from`（date）、`effective_to`（date 可空）、`granted_by`。唯一索引建在 `(legal_entity_id, user_id, role_id, effective_from)` 四列上：全称 `ux_user_role_grants_legal_entity_id_user_id_role_id_effective_from` 共 66 字节超限，缩写为 `ux_user_role_grants_le_id_user_id_role_id_eff_from`（50 字节）：`legal_entity_id` 缩为 `le_id`、`effective_from` 缩为 `eff_from`，此处登记全称备查。

### 8.9 user_org_assignments（用户组织任职，阶段 4）

带 `legal_entity_id`，策略 `rls_user_org_assignments_le`。自有列：`user_id`、`department_id`、`position_id`（两列分别以 `(legal_entity_id, department_id)`、`(legal_entity_id, position_id)` 复合外键指向 platform_core 两表，ON DELETE RESTRICT）、`effective_from`、`effective_to`（date 可空）。

### 8.10 user_scope_grants（用户范围授予，阶段 4）

带 `legal_entity_id`，策略 `rls_user_scope_grants_le`。自有列：`user_id`、`scope_kind`（`PROJECT`、`CUSTOMER`、`RECORD` 三值；RECORD 时 `object_type` 必填）、`object_type`（可空）、`scope_ref_id`、`can_reshare`（默认 false，CHECK 恒假：首版不允许转授）、`granted_by`、`effective_from`、`effective_to`。唯一索引建在 `(legal_entity_id, user_id, scope_kind, scope_ref_id)` 四列上：全称 `ux_user_scope_grants_legal_entity_id_user_id_scope_kind_scope_ref_id` 共 69 字节超限，缩写为 `ux_user_scope_grants_le_id_user_id_scope_kind_scope_ref_id`（59 字节）：`legal_entity_id` 缩为 `le_id`，此处登记全称备查。

### 8.11 sod_rules（职责分离规则，阶段 4）

带 `legal_entity_id`，策略 `rls_sod_rules_le`。自有列：`rule_code`（法人内唯一，长度 1 至 128）、`rule_kind`（`DUTY_EXCLUSION`、`ROLE_EXCLUSION`、`SELF_APPROVAL`、`CHAIN_SKIP` 四类）、`left_ref`、`right_ref`（可空，规则两端引用）、`enforcement`（默认 `BLOCK`）、`message_code`（长度 1 至 128）。默认规则由第 28 号回填迁移写入。

### 8.12 approval_chains（审批链，档案类，阶段 4）

带 `legal_entity_id`，策略 `rls_approval_chains_le`。自有列：`code`（正则 `^[A-Z][A-Z0-9_]{0,63}$`，长度 1 至 64）、`name`（1 至 200）、`scenario`（非自由文本，CHECK 精确取阶段 4 第 4.1 节 `ApprovalScenarioCode` 三十七值，含扩展启用 `EXTENSION_ENABLE`）、`version_no`（int 不小于 1，默认 1）、`lifecycle_state`（五态同 roles）、`is_active`（默认 true）、`deactivated_at`（可空）、`active_scenario_slot`（生成列：仅 `is_active=true and lifecycle_state='EFFECTIVE'` 时等于 scenario，否则为空）。保留 `ux_approval_chains_legal_entity_id_code_version_no`；权威版本唯一约束 `ux_approval_chains_legal_entity_id_scenario_version_no` 在 `(legal_entity_id,scenario,version_no)` 上，权威活动槽唯一约束 `ux_approval_chains_legal_entity_id_active_scenario_slot` 在 `(legal_entity_id,active_scenario_slot)` 上，因 NULL 可重复而保证每法人每场景至多一个有效活动版本。

### 8.13 approval_chain_nodes（审批链节点，阶段 4）

带 `legal_entity_id`，策略 `rls_approval_chain_nodes_le`。表上没有 allow_skip 一类列：越权跳过不是被校验拒绝的配置，而是根本没有承载它的字段。自有列：`approval_chain_id`、`node_no`（int 自 1 起，无空洞由静态校验承担）、`approver_kind`（`ROLE`、`POSITION`、`DEPT_MANAGER` 三类；ROLE 类经 `role_code` 引用，其余经 `approver_ref` 引用）、`approver_ref`（uuid 可空）、`role_code`（可空，长度 1 至 64）、`quorum`（int 不小于 1，默认 1）、`timeout_hours`（可空，不小于 1）。唯一索引建在 `(legal_entity_id, approval_chain_id, node_no)` 上：全称 `ux_approval_chain_nodes_legal_entity_id_approval_chain_id_node_no` 共 69 字节超限，缩写为 `ux_approval_chain_nodes_le_id_approval_chain_id_node_no`（55 字节）：`legal_entity_id` 缩为 `le_id`，此处登记全称备查。

### 8.14 high_risk_requests（高风险请求，单据类，阶段 4）

带 `legal_entity_id`，策略 `rls_high_risk_requests_le`。`doc_no` 类型码 `HRR`，法人内唯一；该类型码在 §5.1 的登记随编号生成本体（属 3b 同批）一并补齐。提交与审批四端点属 3b 同批交付，本阶段只建表与静态校验。

| 列名 | 类型 | 可空 | 默认 | 语义 |
|---|---|---|---|---|
| doc_no | text | 否 | 无 | 单据编号，长度 1 至 64，法人内唯一 |
| status | text | 否 | 无 | 十一态：`PENDING_INITIATION`、`PENDING_REAUTH`、`REAUTH_FAILED`、`LOCKED`、`REAUTH_PASSED`、`IN_APPROVAL`、`APPROVED`、`REJECTED`、`WITHDRAWN`、`ABANDONED`、`EXECUTED` |
| operation_type | text | 否 | 无 | 六类业务高危操作，取值恰为 `CONTRACT_EFFECTIVE`、`PAYMENT`、`INVOICE_ISSUE`、`LEDGER_POSTING`、`PERIOD_CLOSE`、`SENSITIVE_EXPORT`；明确排除 `DATA_MIGRATION`，后者只使用阶段 14 专用批准事实，不进入本表 |
| subject_object_type、subject_object_id | text、uuid | 否 | 无 | 待签对象定位 |
| subject_digest | bytea | 否 | 无 | 待签内容 SHA-256 摘要 |
| reauth_challenge_id | uuid | 是 | 无 | 关联复核挑战 |
| approval_chain_id | uuid | 否 | 无 | 所用审批链 |
| approval_ref | uuid | 是 | 无 | 精确审批实例引用；属于基线具名 `approval_ref` 白名单，不建立伪物理外键，由流程契约在同一事务校验法人、场景与终态 |
| initiator_user_id、initiator_device_id | uuid、text | 否 | 无 | 发起人与发起设备 |
| submitted_at | timestamptz | 否 | 无 | 提交时刻 |
| decided_at、executed_at | timestamptz | 是 | 无 | 决策与执行时点 |
| execution_ref | uuid | 是 | 无 | 执行回调写入的业务引用 |
| reject_reason | text | 是 | 无 | 拒绝理由 |

索引：`ix_high_risk_requests_legal_entity_id_status_operation_type` 供未结束请求扫描（`ep_high_risk_requests_open` 的法人维度刷新面），时间序取基线索引。

### 8.15 authz_config_versions（授权配置版本，阶段 4）

带 `legal_entity_id`，策略 `rls_authz_config_versions_le`。自有列：`version_no`（bigint 不小于 1，法人内唯一）、`state`（`DRAFT`、`STAGED`、`EFFECTIVE`、`ROLLED_BACK` 四态）、`release_bundle_ref`（可空）、`checksum`（bytea，配置包校验和，启动自检 authz-snapshot-loadable 逐法人比对）、`published_by`、`published_at`（可空）。第 27 号回填迁移为两个法人各写一行 `EFFECTIVE`，是本阶段运行期唯一的生效版本来源。

## 9. 跨模块引用的实体标记类型

下列 22 项是被三个以上阶段的契约层同时引用的实体，集中声明在 `crates/foundation/src/id/marker.rs`，供 `Id<T>` 在契约层表达跨模块引用。清单冻结 22 项，改名与增删由 `xtask archcheck` 的 `foundation-frozen-items` 规则按名逐项断言。

标记类型无字段、无方法、无 trait 实现，只承载类型身份；它们不是表，本节登记它们是因为每一项都对应后续阶段的一张主表，逐项对齐可以防止同一实体在不同模块里落成两张表。

`LegalEntity`、`UserAccount`、`Session`、`Department`、`Position`、`Project`、`Customer`、`Supplier`、`Material`、`Product`、`Warehouse`、`Contract`、`ContractLine`、`SalesOrder`、`SalesOrderLine`、`DeliveryConfirmation`、`DeliveryConfirmationLine`、`PurchaseOrder`、`GoodsReceiptLine`、`PurchaseInvoice`、`PurchaseInvoiceLine`、`AccountingPeriod`。

## 10. 维护纪律

- 先登记后实现：新增表、新增列、新增类型码都是先改本文件再写迁移。
- 已登记的类型码不得改名，也不得回收后另作他用。
- 列语义变化按破坏性变更处理，与改 API 字段同级。
- 本文件的 schema 分节随各阶段的迁移一并提交，不允许迁移已合入而字典未更新。
- 每个产生或修改迁移的阶段，退出条件自动包含「迁移、阶段计划与数据字典逐表逐列一致」。即使该阶段计划没有重复写出这一条，也不构成豁免。

## 11. F-50 开发冻结字典（阶段 7、9、10）

本节登记 F-50 的最终物理语义，优先于阶段 7、9、10 旧计划中的单行发票、单税率、`origin` 与 `reverses_id` 方向推断模型。这里是开发前字典，不表示迁移已经执行。

### 11.1 封闭枚举

| 枚举 | 取值 |
|---|---|
| SettlementEffectKind | `APPLY`、`RELEASE` |
| SettlementFundingOrigin | `DIRECT_CASH`、`ADVANCE_AUTO` |
| ArApEntryKind | `ORIGINAL`、`REVERSAL` |
| InvoiceDirection | `OUTPUT`、`INPUT` |
| InvoiceReversalKind | `VOID`、`RED_LETTER` |
| QuantityEffectKind | `REDUCE`、`NONE` |
| PricingEffectKind | `ORIGINAL_UNIT_PRICE`、`ADJUSTED` |
| InvoiceNumberScheme | `UNIFIED_20`、`LEGACY_CODE_NUMBER` |
| InvoiceMedium | `ELECTRONIC`、`PAPER` |
| InvoiceNumberOwnerType | `SALES_BLUE`、`PURCHASE_BLUE`、`OUTPUT_RED`、`INPUT_RED` |
| VoucherSourceKind 新增值 | F-50：`CORRECTION`；Stage 14 092600：`HISTORICAL_MIGRATION`；终态 19 项，后三个受控特殊来源均不进普通 `JOURNAL_MAP` |

### 11.2 finance 核销与退款来源

四张关系表 `finance.receivable_settlement_links`、`payable_settlement_links`、`advance_receipt_settlement_links`、`advance_payment_settlement_links` 共同拥有 `effect_kind text not null`、`source_doc_type text not null`、`source_doc_id uuid not null`、`root_apply_id uuid not null`、`reverses_id uuid null`、`settled_amount numeric(18,2) not null`；前两张另有 `funding_origin text not null`。根、派生、父子累计、同法人同条目同根与无环约束按 F-50 第 3.1 节建立复合自引用外键、NULL-safe CHECK 和延迟约束触发器。

`finance.refund_source_payment_links` 的业务列冻结为 `refund_id`、`source_doc_type`（`RECEIPT|PAYMENT`）、`source_doc_id`、`linked_amount numeric(18,2)`、`advance_consumed_amount numeric(18,2)`、`settlement_released_amount numeric(18,2)`；后三者非负，逐行 `linked_amount = advance_consumed_amount + settlement_released_amount`。退款产生的预收预付效果行与应收应付 RELEASE 行增加 `refund_source_payment_link_id`，由数据库外键保证同法人并由延迟约束保证对应同一退款与原款项。

### 11.3 invoice 头行与号码

`finance.unbilled_ar_entries` 现行业务金额列为 `net_amount numeric(18,2) not null` 与 `gross_amount numeric(18,2) not null`；数据库唯一金额 CHECK 为 `net_amount>=0 AND gross_amount>0 AND gross_amount>=net_amount`，不得另建 `net_amount>0` CHECK。普通业务入口在用例层另行强制净额大于零，F-50 纯税冲销允许 net=0/gross>0，两列不得相互推导。`voucher_id` 仍 NOT NULL，整行 APPEND_ONLY；交付/退货命令必须携完整 `accounting_period_id,accounting_period_seq,deferred_from_period_id,voucher_id,net_amount,gross_amount`，使用 period→inventory→posting→unbilled 顺序一次插入，不先空后更新。`finance.v_unbilled_ar_net` 同时输出 net_balance 供总账勾稽与 gross_balance 供信用三桶，不新增视图或迁移文件。

`invoice.sales_invoice_lines` 是新增表，业务列为 `sales_invoice_id`、`line_no`、`sales_order_id`、`sales_order_line_id`、`item_kind`、`item_id`、`uom_code`、`quantity numeric(18,6)`、`net_unit_price numeric(18,6)`、`tax_rate numeric(9,6)`、`net_amount numeric(18,2)`、`tax_amount numeric(18,2)`、`gross_amount numeric(18,2)`；同一发票 `line_no` 唯一。

`invoice.purchase_invoice_lines` 在旧字段基础上增加 `tax_rate numeric(9,6) not null` 与 `gross_amount numeric(18,2) not null`；`invoice.purchase_invoices` 删除头 `tax_rate`、`is_credit_note`、`reversed_by_id`。`invoice.sales_invoices` 删除头 `tax_rate` 及号码副本。两张蓝票头只保留服务端汇总的 `net_amount/tax_amount/gross_amount` 和 `invoice_number_registry_id`。

`invoice.invoice_reversals` 使用 `direction`、`reversal_kind`、`source_sales_invoice_id`、`source_purchase_invoice_id`、**`linked_purchase_return_id uuid NULL`**、服务端汇总三金额及可空 `invoice_number_registry_id`，两种原票 id 按方向恰一非空；删除单 `source_invoice_id` 与头 `red_tax_rate`。`invoice.invoice_reversal_lines` 的业务列为 `invoice_reversal_id`、两组 source invoice/line id、`quantity_effect_kind`、`pricing_effect_kind`、`quantity`、`tax_rate`、`net_amount`、`tax_amount`、`gross_amount`，**另含 `source_effect_seq`**；关键列 `NOT NULL`，**但两组 source invoice／line id 按方向条件可空，不得声明为全部 `NOT NULL`**（本节原写「关键列全部 NOT NULL」且漏列 `linked_purchase_return_id` 与 `source_effect_seq`，与分卷 `docs/data-dictionary/invoice.md:46`／`:52` 及 F-50 设计 `:348`／`:352` 相反，F-60 更正）；来源组用 NULL-safe XOR、默认 `MATCH SIMPLE` 复合外键与延迟头行一致性触发器约束。

`invoice.invoice_number_registry` 的业务列为 `invoice_medium`、`number_scheme`、`invoice_code`、`invoice_no`、`identifier_key`、`owner_type`、`owner_id`。除旧制下允许 `invoice_code` 取值、统一 20 位制式必须为空外，所有键字段 NOT NULL；`identifier_key` 是 `GENERATED ALWAYS AS (...) STORED NOT NULL`。唯一键为 `(legal_entity_id, identifier_key)` 与 `(legal_entity_id, owner_type, owner_id)`，另有 `(legal_entity_id, id)` 候选唯一键供业务头复合外键引用。

### 11.4 portal 供应商发票上传

`portal.supplier_invoice_uploads` 删除单一 `tax_rate`，增加 `invoice_medium`、`number_scheme`，号码字段按同一制式校验，头三金额为行汇总；数据库生成 `identifier_key` 与 `active_identifier_slot`，后者在 `UPLOADED|ACCEPTED` 时取前者、`RETURNED` 时取 NULL，并以普通 `UNIQUE(legal_entity_id,supplier_id,active_identifier_slot)` 表达活动号码唯一，不使用部分索引。状态形状 CHECK 强制退回原因与正式进项发票引用恰按终态出现。新增 `portal.supplier_invoice_upload_lines`：`supplier_invoice_upload_id`、`line_no`、`purchase_order_id`、`purchase_order_line_id`、`goods_receipt_id`、`goods_receipt_line_id`、`cost_kind`、`item_id`、`quantity`、`net_unit_price`、`tax_rate`、`net_amount`、`tax_amount`、`gross_amount`。表带基线公共列、法人 RLS、同头行号唯一及同法人复合外键。上传在 ACCEPTED 前不占中央号码。

### 11.5 ledger 更正凭证

新增 `ledger.correction_vouchers` 与 `ledger.correction_voucher_lines` 两张仅追加、带法人 RLS 的表。头表至少保存 `doc_no`（类型码 `CORR`）、`source_voucher_id`、`reason`、`posting_date`、`accounting_period_id`、`generated_voucher_id`、发起人与过账时点；行表保存原凭证行、固定借贷方向、科目、金额与说明。只允许引用已过账原凭证，累计更正不得超过相应原行金额，借贷必须平衡，不能携带资金账户、业务单据状态或自由科目输入；凭证只能经 `PostingPort::post_correction` 一次构造并原子写入。

### 11.6 sales 销售退货原交付金额与 capture 分配

`sales.return_line_delivery_links` 在既有退货/订单/交付祖先键和 `quantity` 之外，增加登记前同空、登记事务一次写入的 `allocation_quantity_before numeric(18,6)`、`revenue_amount numeric(18,2)`、`gross_amount numeric(18,2)`、`cost_amount numeric(18,2)`。非空时区间起点非负、三金额非负且 gross 不小于 revenue；REGISTERED/CLOSED 后数量、区间和三金额不可改删。对同一原交付行，已登记链接的 `[before,before+quantity)` 从 0 连续、无重叠无空洞且不超过交付数量；每个金额用 `round(M*(before+quantity)/original_quantity,2)-round(M*before/original_quantity,2)`，M 依次取原行实际 net/gross/`coalesce(cogs,0)`，从而全量末段吸收舍入尾差。退货行 net/gross/`coalesce(inventory_return_amount,0)` 分别等于 links 三额合计，库存入库金额不得改用当前移动平均成本。

新增仅追加表 `sales.return_line_capture_allocations`，业务祖先列为 `sales_return_id/sales_return_line_id/delivery_confirmation_line_id/return_line_delivery_link_id`，以长复合 FK 指向唯一 link；`side` 只取 `REVENUE|COST`，`cost_role` 在 REVENUE 时为空、COST 时必为 `MAIN_OPERATING_COST|DIRECT_EXPENSE_COST`。为避免一个裸多态 id 绕过真实 FK，物理列采用互斥两组 `revenue_root_entry_id/revenue_live_entry_id` 与 `cost_root_entry_id/cost_live_entry_id`，另生成只读 `root_entry_id/live_entry_id=coalesce(...)`；`amount numeric(18,2)>0`。唯一键为 `(legal_entity_id,return_line_delivery_link_id,side,live_entry_id)`。每个 link/side 对 owner 返回的全部 live leaves，以锁后 available 整数分执行 largest-remainder：先 floor 比例份额，余分按 `(fraction DESC,role ordinal ASC,root UUID bytes,live UUID bytes)` 补一分，收入 role ordinal 固定 0、成本 MAIN=0/DIRECT=1；禁止 FIFO 或任取第一片。`V20261017093630__sales_backfill_append_only_registry.sql` 登记 `APPEND_ONLY` 且 mutable_columns 为空；运行账号不得 UPDATE/DELETE。

Stage 11 的 `V20261020090130__sales_add_costing_capture_foreign_keys.sql` 为两组 root/live 列分别补到 `costing.revenue_entries/cost_entries(legal_entity_id,root_entry_id,id)` 的 `ON DELETE RESTRICT DEFERRABLE INITIALLY DEFERRED` 长 FK。补充延迟图证明 root 的原始来源恰为 link 的交付头行，live 与 root 同链、为当前正向开放 fragment 且金额足够；REVENUE 属交付收入，COST 的 `cost_role` 必须等于 live 权威角色。每个 allocation 还必须与同一事务生成、来源为当前 SALES_RETURN 头行、`reverses_id=live_entry_id`、绝对金额相等且 measure/capture kind 匹配 role 的反向 capture 一一对应。图从“提交后开放额 + 本退货反向额”还原锁前 available 并逐片重算 largest-remainder；link 的相应金额为零时没有该 side allocation，非零时 fragments 合计恰等于 link 金额。成本 role 分组只生成 `inventory_return_cogs_amount` 与 `inventory_return_direct_expense_amount`，两者合计等于不进入 PostingInput 的控制总额 `inventory_return_amount`。`DeliveryCaptureReturnBasisQuery` 是 sales 取得 current live fragments 的唯一入口，sales schema/应用不得直读 costing 表。

## 12. F-51 仓库主数据冻结

`mdm.warehouses` 是仓库档案的唯一权威表，由阶段 5 的第 33 号迁移创建、第 34 号迁移启用并强制 RLS。表带档案类公共列，专有列为：

| 列 | 类型 | 可空 | 规则 |
|---|---|---:|---|
| code | text | 否 | 法人内唯一，类型码 `WHSE`，首次生效后冻结 |
| status | text | 否 | `DRAFT\|PENDING_APPROVAL\|EFFECTIVE\|VOID` |
| is_active | boolean | 否 | 默认 true |
| deactivated_at | timestamptz | 是 | 与 is_active 一致 |
| version_no | bigint | 否 | 默认 0，已生效时大于 0 |
| name | text | 否 | 1–200 字 |
| is_default_receiving | boolean | 否 | 默认 false，每法人最多一个已启用默认收货仓 |
| is_default_shipping | boolean | 否 | 默认 false，每法人最多一个已启用默认发货仓 |
| default_receiving_slot | uuid | 是 | 仅在 `status='EFFECTIVE' AND is_active AND is_default_receiving` 时等于 `legal_entity_id`，否则为 NULL；NULL-safe CHECK 加普通唯一索引维护每法人一个默认收货仓 |
| default_shipping_slot | uuid | 是 | 仅在 `status='EFFECTIVE' AND is_active AND is_default_shipping` 时等于 `legal_entity_id`，否则为 NULL；NULL-safe CHECK 加普通唯一索引维护每法人一个默认发货仓 |
| owner_user_id | uuid | 否 | 与法人组成真实复合外键 `(legal_entity_id,owner_user_id) -> platform_authz.user_legal_entity_grants(legal_entity_id,user_id) ON DELETE RESTRICT`；身份契约另校验授权状态 |
| remark | text | 是 | 最长 2000 字 |

仓库建档、变更、停用与再启用复用 `mdm.change_requests`。停用必须在同一事务内通过 `WarehouseDeactivationCheckPort` 和已启用来源模块的 `MasterReferenceCounter`；库存结存非零、存在未完成来源单据或必需检查器缺失都不允许停用。`inventory` schema 不得再建第二张仓库档案表。

## 13. inventory schema（阶段 8 与 F-51 U-F-02 开发前冻结）

固定十张表：`stock_movements`、`stock_qty_entries`、`stock_value_entries`、`variance_splits`、`stock_qty_balances`、`stock_value_balances`、`variance_coverage_balances`、`serial_states`、`stock_movement_serials`、`replenishment_policies`。全部带 `legal_entity_id`、`UNIQUE(legal_entity_id,id)` 并启用且强制 RLS。`stock_movements`、`stock_qty_entries`、`stock_value_entries`、`variance_splits`、`stock_movement_serials` 是仅追加表，只带公共创建列；其余五张是可更新余额、状态或策略表，带完整公共列与 `row_version`。五张仅追加表没有逐行反向父链，均不带恒空 `reverses_id`。

固定单目标引用全部建立真实 `ON DELETE RESTRICT` 外键：`warehouse_id` 指向 `mdm.warehouses(legal_entity_id,id)`，`material_id` 指向 `mdm.materials(legal_entity_id,id)`，`accounting_period_id` 与 `deferred_from_period_id` 指向 `ledger.accounting_periods(legal_entity_id,id)`，内部 movement、quantity entry、value entry 与 variance split 引用同样使用带法人的复合形状。子表同时保存 movement 与段 id 时必须使用含 movement 的更长候选键和外键，数据库不能允许 M1 的子行挂到 M2 的 quantity/variance 段。公共 `created_by/updated_by` 指向 `platform_authz.user_legal_entity_grants(legal_entity_id,user_id)`；系统写入也使用每法人已有授权的 `SYSTEM_PRINCIPAL_ID`。只有带 `source_doc_type/source_module` 判别的 movement 来源单据/行组合属于封闭多态白名单；仓库、物料、期间和固定采购发票来源不得降级成逻辑引用。

“同 movement”仍不足以证明“同计价段”。`V20261016090800__inventory_create_movement_serials.sql` 在九张库存图表齐备后建立 `inventory.assert_inventory_graph_consistent()`，并给 movement/qty/value/split/serial facts 与 qty/value/coverage balances、serial states 建立 `DEFERRABLE INITIALLY DEFERRED` 约束触发器。提交时统一强制：所有子事实与 movement 的法人、安全标签、actor、direction/date/period 冗余逐值相等；IN/OUT 的 `qty_count=value_count=line_count` 且每 qty 恰一条逐字段相等的 value；VALUE_ADJUST 的 `split_count=line_count`，非零 on-hand split 恰一条 amount/after 相等的 value、零 on-hand split 无 value；serial 与 qty 的维度/方向/期间一致；四张投影的 last pointer 命中同一维度，serial state 还必须命中同 serial fact。普通 FK 能命中但同 movement 错段、错误 line_count 或冗余值不等时仍在提交点整笔拒绝；应用断言与对账不替代该约束。

### 13.1 stock_movements（库存移动头，仅追加）

| 业务列 | 类型 | 可空 | 约束与语义 |
|---|---|---:|---|
| business_date | date | 否 | 原始业务记账日期 |
| accounting_period_id | uuid | 否 | 同法人复合外键指向 `ledger.accounting_periods` |
| accounting_period_seq | int | 否 | 法人内会计期间单调序号 |
| deferred_from_period_id | uuid | 是 | 非空时同法人复合外键指向 `ledger.accounting_periods`，记录关期顺延来源 |
| direction | text | 否 | `IN\|OUT\|VALUE_ADJUST` |
| reason | text | 否 | `PURCHASE_RECEIPT\|SALES_RETURN\|DELIVERY_CONFIRMATION\|PURCHASE_RETURN\|PURCHASE_INVOICE_VARIANCE\|MIGRATION_OPENING\|MIGRATION_HISTORY` |
| source_doc_type | text | 否 | `PURCHASE_RECEIPT\|PURCHASE_RETURN\|DELIVERY_CONFIRMATION\|SALES_RETURN\|PURCHASE_INVOICE\|MIGRATION_STOCK_ADJUSTMENT\|MIGRATION_STOCK_HISTORY` |
| source_doc_id | uuid | 否 | 与 type/module 组成封闭多态来源 |
| source_doc_no | text | 否 | 长度 1..64 |
| source_module | text | 否 | 由 type 派生，`procure\|sales\|invoice\|migration` |
| line_count | int | 否 | 数据库 CHECK 固定 `line_count BETWEEN 1 AND 200`；计价段而非来源业务行数量 |
| request_hash | bytea | 否 | `SHA-256(JCS(command))`，CHECK 固定 32 字节 |

NULL-safe 组合 CHECK 只允许九组 `direction/reason/source_doc_type/source_module`：原六组 `IN/PURCHASE_RECEIPT/PURCHASE_RECEIPT/procure`、`IN/SALES_RETURN/SALES_RETURN/sales`、`OUT/DELIVERY_CONFIRMATION/DELIVERY_CONFIRMATION/sales`、`OUT/PURCHASE_RETURN/PURCHASE_RETURN/procure`、`VALUE_ADJUST/PURCHASE_INVOICE_VARIANCE/PURCHASE_INVOICE/invoice`、`IN/MIGRATION_OPENING/MIGRATION_STOCK_ADJUSTMENT/migration`，以及 direction 分别为 `IN`、`OUT`、`VALUE_ADJUST` 的三组 `MIGRATION_HISTORY/MIGRATION_STOCK_HISTORY/migration`。后者只由 Stage 14 092600 启用的 crate-private writer 写，source_doc_id=data_migration_records.id、source_doc_no=batch_no。唯一键 `ux_stock_movements_le_src_doc` 为 `(legal_entity_id,source_doc_type,source_doc_id)`；查询索引为 `(legal_entity_id,accounting_period_seq,business_date)` 与 `(legal_entity_id,business_date,id)`。

### 13.2 stock_qty_entries（数量流水，仅追加）

| 业务列 | 类型 | 可空 | 约束与语义 |
|---|---|---:|---|
| movement_id | uuid | 否 | 同法人复合外键指向 `stock_movements` |
| line_no | int | 否 | 大于 0 |
| posting_line_key | text | 否 | ASCII 1..128 字节，规范值 `<source-line-uuid>:<segment-seq>` |
| source_doc_line_id | uuid | 否 | 沿用 movement 判别的封闭多态来源行 |
| source_doc_line_no | int | 否 | 来源展示行号 |
| warehouse_id、material_id | uuid | 否 | 分别同法人复合外键指向 MDM 仓库、物料 |
| batch_no | text | 否 | 默认 `'-'`，长度 1..64，字符集 `[A-Za-z0-9._-]` |
| quantity | numeric(18,6) | 否 | 非零；IN 为正，OUT 为负 |
| qty_balance_after | numeric(18,6) | 否 | 大于等于 0 |
| direction | text | 否 | `IN\|OUT`，与 quantity 符号组成完整 CHECK |
| business_date | date | 否 | 冗余自 movement |
| accounting_period_id | uuid | 否 | 同法人复合外键指向会计期间，值冗余自 movement |
| accounting_period_seq | int | 否 | 冗余自 movement |

另建 `UNIQUE(legal_entity_id,movement_id,id)` 作为所有下游段引用的父属候选键，另建 `UNIQUE(legal_entity_id,movement_id,line_no)`；`(movement_id,posting_line_key)` 唯一。line_no 在 `1..=movement.line_count`，direction/date/period/标签与 movement 的一致性由提交触发器强制。查询索引固定为 `(legal_entity_id,warehouse_id,material_id,batch_no,accounting_period_seq)`、`(legal_entity_id,business_date,id)`、`(movement_id,line_no)` 与 `(legal_entity_id,material_id)`。

### 13.3 stock_value_entries（金额流水，仅追加）

| 业务列 | 类型 | 可空 | 约束与语义 |
|---|---|---:|---|
| movement_id | uuid | 否 | 同法人复合外键指向 `stock_movements` |
| line_no | int | 否 | 大于 0 |
| posting_line_key | text | 否 | ASCII 1..128 字节；同一 movement 内唯一 |
| qty_entry_id | uuid | 是 | 非 VALUE_ADJUST 必填；`(legal_entity_id,movement_id,qty_entry_id)` 指向 `stock_qty_entries(legal_entity_id,movement_id,id)` |
| source_doc_line_id | uuid | 否 | 来源业务行 id |
| source_doc_line_no | int | 否 | 来源业务行号，重放不跨 schema 回查 |
| warehouse_id、material_id | uuid | 否 | 分别同法人复合外键指向 MDM 仓库、物料 |
| quantity | numeric(18,6) | 否 | IN 正、OUT 负、VALUE_ADJUST 为 0 |
| amount | numeric(18,2) | 否 | IN 非负、OUT 非正、VALUE_ADJUST 非零 |
| direction | text | 否 | `IN\|OUT\|VALUE_ADJUST`，冗余自 movement |
| applied_unit_price | numeric(18,6) | 否 | 大于等于 0；VALUE_ADJUST 为 0 |
| pricing_branch | text | 否 | `ESTIMATED_PO_PRICE\|OVERBILL_INVOICE_PRICE\|MOVING_AVERAGE\|MOVING_AVERAGE_CLEARING\|ORIGINAL_DELIVERY_PRICE\|VARIANCE_ON_HAND\|MIGRATION_OPENING\|MIGRATION_HISTORY` |
| value_balance_after | numeric(18,2) | 否 | 大于等于 0 |
| qty_balance_after | numeric(18,6) | 否 | 大于等于 0 |
| moving_avg_unit_price_after | numeric(18,6) | 否 | 大于等于 0 |
| variance_split_id | uuid | 是 | 仅 VALUE_ADJUST 必填；由 `V20261016090300__inventory_create_variance_splits.sql` 追补 `(legal_entity_id,movement_id,variance_split_id) -> variance_splits(legal_entity_id,movement_id,id)` |
| business_date | date | 否 | 冗余自 movement |
| accounting_period_id | uuid | 否 | 同法人复合外键指向会计期间，值冗余自 movement |
| accounting_period_seq | int | 否 | 冗余自 movement |

完整形状 CHECK 固定为：`IN => quantity>0 AND amount>=0 AND qty_entry_id IS NOT NULL AND variance_split_id IS NULL`；`OUT => quantity<0 AND amount<=0 AND qty_entry_id IS NOT NULL AND variance_split_id IS NULL`；`VALUE_ADJUST => quantity=0 AND amount<>0 AND applied_unit_price=0 AND qty_entry_id IS NULL AND variance_split_id IS NOT NULL`。`ck_stock_value_entries_after_non_negative` 断言 `value_balance_after >= 0 AND qty_balance_after >= 0 AND moving_avg_unit_price_after >= 0`；`ck_stock_value_entries_zero_after_shape` 断言 `qty_balance_after <> 0 OR (value_balance_after = 0 AND moving_avg_unit_price_after = 0)`。零价 IN/OUT 允许 amount 为 0；零价差不写占位行。普通 `UNIQUE(legal_entity_id,qty_entry_id)` 允许多行 NULL、但限制任一 qty 至多一条 value，`UNIQUE(legal_entity_id,movement_id,line_no)` 锁定段号；提交触发器保证 IN/OUT 每 qty 至少一条且共享 line/key/source/dimension/quantity/direction/date/period，VALUE_ADJUST value 与 split 的 key/source/dimension/date/period、on-hand amount 和 after 快照逐值一致。`(movement_id,posting_line_key)` 唯一；计价段稳定查询索引 `ix_stock_value_entries_le_source_line_posting_key` 固定为 `(legal_entity_id,source_doc_line_id,posting_line_key)`，另有 `(legal_entity_id,accounting_period_seq,warehouse_id,material_id)` 与 `(movement_id,line_no)`。

### 13.4 variance_splits（价差拆分，仅追加）

| 业务列 | 类型 | 可空 | 约束与语义 |
|---|---|---:|---|
| movement_id | uuid | 否 | 同法人复合外键指向 VALUE_ADJUST movement |
| source_doc_id | uuid | 否 | 采购发票头 id |
| source_doc_no | text | 否 | 采购发票展示号，长度 1..64 |
| source_doc_line_id | uuid | 否 | 采购发票行 id |
| source_doc_line_no | int | 否 | 采购发票行号 |
| posting_line_key | text | 否 | ASCII 1..128 字节；同一 movement 内唯一 |
| warehouse_id、material_id | uuid | 否 | 分别同法人复合外键指向 MDM 仓库、物料 |
| matched_quantity | numeric(18,6) | 否 | 大于 0 |
| total_variance_amount | numeric(18,2) | 否 | 发票不含税金额减回冲暂估金额 |
| on_hand_quantity、issued_quantity | numeric(18,6) | 否 | 均大于等于 0，合计等于 matched_quantity |
| on_hand_variance_amount、issued_variance_amount | numeric(18,2) | 否 | 合计等于 total_variance_amount |
| uncovered_before、uncovered_after | numeric(18,6) | 否 | 均大于等于 0 |
| value_balance_amount_after | numeric(18,2) | 否 | 处理后存货金额余额快照，大于等于 0 |
| moving_avg_unit_price_after | numeric(18,6) | 否 | 处理后移动平均单价快照，大于等于 0 |
| business_date | date | 否 | 冗余自 movement |
| accounting_period_id | uuid | 否 | 同法人复合外键指向会计期间 |
| accounting_period_seq | int | 否 | 冗余自 movement |

五条数据库 CHECK 固定为：`on_hand_quantity + issued_quantity = matched_quantity`、`on_hand_variance_amount + issued_variance_amount = total_variance_amount`、`uncovered_before >= 0 AND uncovered_after >= 0`、`uncovered_after = uncovered_before - on_hand_quantity`、`value_balance_amount_after >= 0 AND moving_avg_unit_price_after >= 0`。另建 `UNIQUE(legal_entity_id,movement_id,id)` 作为 value entry 引用的父属候选键；`(legal_entity_id,source_doc_line_id,warehouse_id,material_id)` 与 `(movement_id,posting_line_key)` 各自唯一，查询索引为 `(legal_entity_id,warehouse_id,material_id,created_at)`。提交触发器还强制 split 与 VALUE_ADJUST movement 的 source head、标签、日期和期间一致，并实施“非零 on-hand 恰一 value、零 on-hand 无 value”的双向基数与逐字段匹配。采购发票是固定单目标而非多态例外；Stage10 的 `V20261019090910__inventory_add_invoice_foreign_keys.sql` 追补 `(legal_entity_id,source_doc_id) -> invoice.purchase_invoices(legal_entity_id,id)` 与 `(legal_entity_id,source_doc_id,source_doc_line_id) -> invoice.purchase_invoice_lines(legal_entity_id,purchase_invoice_id,id)`，两条均为 `ON DELETE RESTRICT DEFERRABLE INITIALLY DEFERRED`。同一事务可先写 split、后写父发票图，但提交时父头、父行、法人和头行归属必须完整，失败时整笔回滚；追补前该写入口关闭。

### 13.5 stock_qty_balances（数量余额，可更新）

业务列为 `warehouse_id uuid not null`、`material_id uuid not null`、`batch_no text not null default '-'`、`quantity numeric(18,6) not null`、`last_movement_id uuid null`、`last_qty_entry_id uuid null`。仓库与物料使用同法人真实复合外键；两个 last 指针必须同空同非空，非空时以 `(legal_entity_id,last_movement_id,last_qty_entry_id)` 指向 `stock_qty_entries(legal_entity_id,movement_id,id)`，不另留可跨 movement 的独立 qty FK，并由提交触发器继续强制父 qty 的 warehouse/material/batch 等于本余额键；`quantity >= 0`。`(legal_entity_id,warehouse_id,material_id,batch_no)` 唯一，另有 `(legal_entity_id,material_id,warehouse_id)` 查询索引。

### 13.6 stock_value_balances（金额余额，可更新）

业务列为 `warehouse_id uuid not null`、`material_id uuid not null`、`quantity numeric(18,6) not null`、`value_amount numeric(18,2) not null`、`moving_avg_unit_price numeric(18,6) not null`、`last_movement_id uuid null`。仓库、物料与 movement 均使用同法人真实复合外键；last_movement 非空时，提交触发器要求该 movement 下存在同 warehouse/material 的 value 或 split，零/issued-only 价差允许由 split 证明维度。`ck_stock_value_balances_non_negative` 固定为 `quantity >= 0 AND value_amount >= 0 AND moving_avg_unit_price >= 0`；`ck_stock_value_balances_zero_price` 固定为 `quantity > 0 OR (quantity = 0 AND value_amount = 0 AND moving_avg_unit_price = 0)`，不允许零结存残值。`(legal_entity_id,warehouse_id,material_id)` 唯一。

### 13.7 variance_coverage_balances（价差覆盖余额，可更新）

业务列为 `warehouse_id uuid not null`、`material_id uuid not null`、`uncovered_quantity numeric(18,6) not null`、`last_movement_id uuid null`。三项 id 均使用同法人真实复合外键；last_movement 非空时，提交触发器要求该 movement 下存在同 warehouse/material 的 qty/value/split 事实，不能挂到同 movement 的另一维度；CHECK 固定 `uncovered_quantity >= 0`；`(legal_entity_id,warehouse_id,material_id)` 唯一。

### 13.8 serial_states（序列号状态，可更新）

业务列为 `serial_no text not null`、`material_id uuid not null`、`warehouse_id uuid not null`、`batch_no text not null default '-'`、`status text not null`、`last_movement_id uuid not null`、`last_qty_entry_id uuid not null`。serial/batch 长度为 1..64 且字符集 `[A-Za-z0-9._-]`，status 只取 `IN_STOCK|SHIPPED`；两种状态都保留最近所在仓库，物料与仓库为同法人真实复合外键。last 指针以 `(legal_entity_id,last_movement_id,last_qty_entry_id)` 指向 `stock_qty_entries(legal_entity_id,movement_id,id)`；提交触发器还要求命中同 `(movement,qty,serial_no)` 的 serial fact，父 qty 的 material/warehouse/batch 与本状态一致，并强制 `IN_STOCK/IN`、`SHIPPED/OUT`。`(legal_entity_id,serial_no)` 唯一，查询索引为 `(legal_entity_id,warehouse_id,material_id,status)`。

### 13.9 stock_movement_serials（序列号出入库流水，仅追加）

业务列为 `movement_id uuid not null`、`qty_entry_id uuid not null`、`serial_no text not null`、`material_id uuid not null`、`warehouse_id uuid not null`、`direction text not null`、`business_date date not null`、`accounting_period_id uuid not null`、`accounting_period_seq int not null`。qty entry 以 `(legal_entity_id,movement_id,qty_entry_id)` 指向 `stock_qty_entries(legal_entity_id,movement_id,id)`，物料、仓库与会计期间为同法人真实复合外键；direction 只取 `IN|OUT`，serial 形状同 §13.8。提交触发器把 material/warehouse/direction/date/period/标签逐值锁到该 qty 与 movement，不能改挂同 movement 另一段。`(qty_entry_id,serial_no)` 唯一，追溯索引为 `(legal_entity_id,serial_no,created_at)`。

### 13.10 replenishment_policies（补货策略，可更新）

业务列固定为 `warehouse_id uuid not null`、`material_id uuid not null`、`reorder_point numeric(18,6) null`、`target_stock numeric(18,6) null`。仓库与物料为同法人真实复合外键；`(legal_entity_id,warehouse_id,material_id)` 唯一。NULL-safe CHECK 只允许两阈值同时为空，或同时非空且 `target_stock >= reorder_point >= 0`。行不删除，两阈值同空表示停用；阈值不得复制到物料、仓库、库存余额或采购需求表。

A11 是该对象的受控列表入口，权限为 `inventory.replenishment_policy:read`；A12 `PUT /api/v1/inventory/replenishment-policies/{warehouse_id}/{material_id}` 是唯一写入口，权限为 `inventory.replenishment_policy:write`，以 `(legal_entity_id, warehouse_id, material_id)` 定位业务对象并用 `expected_row_version` 做乐观锁。新行的期望版本必须为空，已有行必须等于锁前版本；成功时审计 `object_type` 固定取 `inventory.replenishment_policies`、`object_id` 取该策略行 UUID，`before/after` 完整记录两阈值与版本。它是配置对象，不生成单据号，不登记单据类型码，不发领域事件，也不直接修改库存数量或金额。

读取所有权固定为阶段 8 的 `ReplenishmentPolicyReadPort`；阶段 6 的 `SalesAwareReplenishmentPolicyQuery` 仅把启用策略与同一个 `SalesAwareAvailabilityQuery` 组合成 `ReplenishmentPolicyQuery`，分页上限 500；阶段 7 自动采购需求扫描只消费后者。采购不得直接读取本表，也不得另存或重算阈值、结存与销售未交付量。

## 14. costing schema（阶段 11 开发前冻结）

状态：开发前契约，尚未执行迁移；逐列来源为阶段 11 计划 §3.1，首次迁移落地时由数据库元数据生成逐列表格并在同一变更中补入本节。

- `costing.cost_entries`：仅追加成本事实，冲销血缘列固定为 `root_entry_id uuid not null`、`effect_seq bigint generated always as identity`、`reverses_id uuid null`、`lineage_slot uuid generated always as (coalesce(reverses_id,'00000000-0000-0000-0000-000000000000'::uuid)) stored`；来源只允许 `INVENTORY_COGS|DIRECT_EXPENSE|POSTING_VARIANCE`。除 `CostReturnMarkPort` 可把 `is_returned_not_reversed/return_mark_reason/return_mark_approval_ref` 从未标注一次性置为完整标注外，禁止更新和删除；后续冲回或更正只新增条目。
- `costing.revenue_entries`：与成本表使用相同四项血缘列；来源只允许 `DELIVERY_ORDER|DELIVERY_MILESTONE|SALES_RETURN`；不设仓库、差异原因或退货未冲回成本标注列，首版受控更正不允许收入侧。
- 治理视图：`v_cost_entries_dataset`、`v_revenue_entries_dataset`、`v_margin_dataset`。三者都含 `legal_entity_id/security_level/data_scope_tags`，无 `SECURITY DEFINER`，只授只读权限。

两个事实表都以 `accounting_period_id` 作为历史期间归集依据。每表提供 `UNIQUE(legal_entity_id,id)` 与 `UNIQUE(legal_entity_id,root_entry_id,id)` 候选键；根指针以 `(legal_entity_id,root_entry_id)` 指向同表 `(legal_entity_id,id) ON DELETE RESTRICT DEFERRABLE INITIALLY DEFERRED`，直接父以 `(legal_entity_id,root_entry_id,reverses_id)` 指向同表 `(legal_entity_id,root_entry_id,id) ON DELETE RESTRICT DEFERRABLE INITIALLY DEFERRED`。根的 UUIDv7 `id` 必须在 INSERT 前生成并在同一行写 `root_entry_id=id`，所以空表首根的自引用只在 COMMIT 校验，不先写 NULL、不后补 UPDATE。NULL-safe 形状强制根 `reverses_id IS NULL AND root_entry_id=id`、派生行 `reverses_id IS NOT NULL AND root_entry_id<>id`。唯一键改为 `(legal_entity_id,voucher_line_id,source_document_line_id,lineage_slot)`：根用 nil slot 防普通重复捕获；反向行用父 id，允许同一销售退货来源行和凭证行分别冲多个不同 capture，同时拒绝同一父重复。

cost/revenue 各有独立 `DEFERRABLE INITIALLY DEFERRED` 血缘约束触发器。提交时按根稳定锁读并用锁后新语句快照强制父子同法人、同表、同根，`parent.effect_seq<child.effect_seq`，符号相反、无环，且任一父的直接反向子效果绝对额合计不超过父绝对额；两个并发事务合计超额时只能一个成功。根/父形状逐腿服从 `CaptureParentRequirement`：NewRootOnly 父空，ReverseCurrentLiveOnly 必须 exact live 父，conditional 按有符号 capture amount 分支。受控更正仅限 cost，反向条目指向源 cost capture，目标成本条目再指向该反向条目，不能让同号目标直接挂源父；收入 source/check/trait 均无 correction 入口。`root_entry_id/effect_seq/reverses_id/lineage_slot` 全部不可更新；成本表唯一允许的三列单向标注不改变血缘。标注组残值按同 `root_entry_id` 的根与全部深度后代求和。

会计期间、凭证、凭证行、科目、合同、销售订单、销售订单行、客户、产品、物料与仓库等固定单目标列全部建立同法人真实复合外键并 `ON DELETE RESTRICT`；凭证行使用含 `voucher_id` 的更长归属键。`project_id` 由 Stage12 的 `V20261021090040__costing_add_project_foreign_keys.sql` 在目标建立后追补。只有 `source_document_type/source_document_id/source_document_line_id` 封闭多态来源、精确审批引用与 `release_package_id` 继续属于具名无外键白名单；公开契约仍负责目标状态与业务范围校验，不替代引用完整性。

销售退货按 `DeliveryCaptureReturnBasisQuery::lock_available` 读取原交付当前可冲回 fragment；收入返回 `RevenueLiveFragment`，成本返回带 `ReturnCostRole::{MainOperatingCost,DirectExpenseCost}` 的 `CostLiveFragment`，每个 fragment 固定含 root/live id、严格正的开放额与维度，按 delivery line、side、root、live 稳定锁序。受控更正形成多个 live fragment 时必须全量返回。进项红字按 `PurchaseInvoiceCaptureReversalBasisQuery::lock_available` 与输入三元组 `(original_purchase_invoice_id,original_purchase_invoice_line_id,original_capture_kind)` 锁读 `DirectExpense` 或 `PostingVariance(EstimatePriceDiffIssued)` 的 current cost leaves；它不复用销售 DTO，而返回 `PurchaseInvoiceCostLiveFragment {root_entry_id,live_entry_id,available_amount:PositiveMoney,effect_sign:DebitCost|CreditCost,role,dimensions}`。原 root 可正可负，只返回与 root 同 sign 的 current leaf，开放额取绝对值；Stage 10 按 sign 反向、按绝对额 largest-remainder，measure 保持有符号而 attribution amount 恒正。Stage 6/10 均不得直读 costing 表。`V20261020090130__sales_add_costing_capture_foreign_keys.sql` 以两组 nullable 实体列为 `sales.return_line_capture_allocations` 补 revenue/cost root-live 长复合 FK，并把双向延迟图装到 sales allocation 与两张 costing 表；不得用一个无真实 FK 的多态 `live_entry_id` 或只保存原 root 单 id。

## 15. reporting schema（阶段 11 开发前冻结）

状态：开发前契约，尚未执行迁移；逐列来源为阶段 11 计划 §3.2，首次迁移落地时由数据库元数据生成逐列表格并在同一变更中补入本节。

固定九张表：`datasets`、`dataset_fields`、`report_objects`、`report_object_versions`、`report_object_publications`、`report_object_dependencies`、`aging_bucket_profiles`、`aging_bucket_lines`、`render_tasks`。

- `datasets` 与 `dataset_fields` 是全部署同值目录，不带 `legal_entity_id`，必须登记进 `platform_core.unpoliced_table_registry`；其余七张表全部带法人 RLS。
- `report_objects.governance_scope` 只允许 `PERSONAL|ENTERPRISE`。PERSONAL 必须有 `scope_owner_user_id` 且只走 `DRAFT|ACTIVE|RETIRED`；ENTERPRISE owner 必须为空且只走 `DRAFT|PENDING_APPROVAL|PUBLISHED|DEACTIVATED`。
- PERSONAL 版本的提交、审批、发布包和停用证据列必须全空，且每对象至多一个 ACTIVE 版本；ENTERPRISE 当前发布版本只由 `report_object_publications` 指针表达，回退只切整对象版本指针，不删除版本、不做字段级回退。
- 账龄分档为 profile + lines 唯一模型，默认七档取阶段 11 §3.4；finance 不保留第二张分档表。
- `render_tasks` 类型码 `RT`，格式只允许 `XLSX|CSV|PDF`，预检与最终行数均不得超过 50,000；敏感任务在排队前必须同时具有重新认证和审批证据。

## 16. service schema（阶段 12 开发前冻结）

状态：开发前契约，尚未执行迁移；逐列来源为阶段 12 计划 §3.2–§3.4。

固定十六张表：四张局部字典 `equipment_statuses`、`work_order_types`、`complaint_channels`、`work_order_priorities`；业务表 `equipment_records`、`equipment_migration_corrections`、`customer_complaints`、`customer_complaint_migration_corrections`、`work_orders`、`work_order_lines`、`work_order_logs`、`work_order_reminder_policies`；附件关联表 `equipment_attachments`、`customer_complaint_attachments`、`work_order_attachments`、`work_order_line_attachments`。全部带法人 RLS。

局部字典引用使用同 schema 复合外键并 `ON DELETE RESTRICT`，字典行只停用不删除。库存序列状态、客户、合同、产品、销售订单及其行、交付确认及其行、销售退货及其行、替换交付计划、仓库、退货原因分类项、业务用户与附件对象等固定单目标列全部使用同法人真实复合外键并 `ON DELETE RESTRICT`；带父头归属的行目标使用正文冻结的更长候选键。`SerialStateQuery`、`SalesOrderLineDeliveryQuery`、`SalesReturnCommandPort` 与 `SalesExchangeLinkCommandPort` 仍在同一调用方事务校验可引用状态与业务一致性，不替代外键。service 不保存第二份 `serial_no`；只有正文明确的 `object_type/object_id` 封闭多态组合属于无外键白名单。`work_order_logs` 的普通 ACTION 行不带父链，CORRECTION 行必须以同法人、同工单复合自外键指向真实父日志；`equipment_migration_corrections` 与 `customer_complaint_migration_corrections` 各以同法人 root FK 与 root 唯一键绑定一台设备/一张投诉，均不带 reverses_id。三表全部登记 `platform_core.append_only_registry(mode='APPEND_ONLY',mutable_columns='{}')` 并附着统一数据库 guard，运行账号 UPDATE/DELETE 必须失败，静态 SQL 检查不替代此约束。

`service.equipment_migration_corrections` 自有列精确为 `equipment_record_id`、`correction_mode`（`SET_RETURNED|RETAIN_TERMINAL`）、`status_before_code`、`status_after_code`、`root_row_version_before/after bigint`、`reason='DATA_MIGRATION_REVERSED'`；两个状态码均用同法人复合 FK 指向 `equipment_statuses`。SET_RETURNED 要求 before 字典非终态、after=RETURNED 且 row_version 加一；RETAIN_TERMINAL 要求 before/after 同码且字典终态、row_version 不变。`service.customer_complaint_migration_corrections` 自有列精确为 `complaint_id`、`correction_mode`（`CANCEL|RETAIN_TERMINAL`）、`status_before/after`、`root_row_version_before/after bigint`、同一固定 reason；CANCEL 只允许 REGISTERED/PROCESSING→CANCELLED 且版本加一，RETAIN 只允许 CLOSED/CANCELLED 原态且版本不变。两表各在 correction 侧安装 `DEFERRABLE INITIALLY DEFERRED` 最终效果 trigger，提交时锁根并核实 status、row_version、security_level、data_scope_tags；设备图另锁字典并核实 is_terminal。普通设备状态历史仍只写 `platform_audit.audit_events`，migration correction 不是通用状态历史。

设备、投诉、工单类型码分别为 `EQ`、`CPL`、`WO`。工单六状态、登记行四状态和终态只读规则以阶段 12 §4.4、§4.6 为唯一状态机。

## 17. project schema（阶段 12 开发前冻结）

状态：开发前契约，尚未执行迁移；逐列来源为阶段 12 计划 §3.2。

固定六张表：`projects`、`project_migration_corrections`、`project_tasks`、`project_task_purchase_requisition_links`、`project_attachments`、`project_task_attachments`，全部带法人 RLS。类型码 `PRJ` 与 `PT` 已在 §5.1 登记。

客户、来源合同、合同版本、采购需求、采购物料、业务用户与附件对象等固定单目标列全部建立同法人真实复合外键并 `ON DELETE RESTRICT`；合同版本以 `(legal_entity_id,contract_id,version_no)`、采购需求链接以 `(legal_entity_id,purchase_requisition_id)` 指向目标候选键。各提供方 contract trait 仍在同一事务校验可引用状态与业务范围，不替代外键。合同续签链以 `project_group_contract_id` 复用同一项目；任务的 `unique_key/obligation_key/obligation_hash` 直接使用 CLM 返回值，不由 project 重算。采购需求链接只有在异步回写取得真实 `purchase_requisition_id` 后一次写入，不建立可空占位链接，并以 `(legal_entity_id,project_task_id)` 与 `(legal_entity_id,purchase_requisition_id)` 两个唯一约束落实首版一对一。

`project_tasks.requisition_link_state` 与链接行基数属于一项跨表不变量：`LINKED` 当且仅当同法人恰有一条 `project_task_purchase_requisition_links`；`NULL|PENDING|FAILED` 当且仅当没有链接行。两表各装同一 `DEFERRABLE INITIALLY DEFERRED` 约束触发器并覆盖 INSERT/UPDATE/DELETE，提交点稳定锁任务后复算，因此同事务可任意排序地写两侧，但不能提交半套、删除半套或错误状态。`CONTRACT_DERIVED` 任务的 `derivation_batch_no` 与其余五项派生来源字段同为必填且大于零；MANUAL 行六项全空。

`project.project_migration_corrections` 是仅追加的迁移撤销 owner fact，自有列精确为 `project_id`、`correction_mode`（`CLOSE|RETAIN_CLOSED`）、`status_before/after`、`root_row_version_before/after bigint`、`reason='DATA_MIGRATION_REVERSED'`；`UNIQUE(legal_entity_id,project_id)`，根 FK `ON DELETE RESTRICT DEFERRABLE INITIALLY DEFERRED`。CLOSE 只允许 IN_PROGRESS/COMPLETED→CLOSED 且版本加一，RETAIN_CLOSED 只允许 CLOSED 原态且版本不变。090100 在 tasks 建成后给 correction 侧安装 DEFERRABLE 最终效果 trigger，提交点锁项目并核实根 after-image、安全属性及全部 tasks 已 COMPLETED/CANCELLED；本表登记 APPEND_ONLY 且不带 reverses_id。Stage 14 的 `reverse_migrated_project` 必须复用 task CANCEL/project CLOSED 并以该新 correction id 为 REVERSE target，不得只改根或删除历史。

## 18. platform_meta 配置发布审批与无损删除语义

`platform_meta.config_packages` 是部署级表，但审批发生在受信法人上下文。除阶段 13 §3.2.10 的包体、签名、自动测试与公共列外，阶段 3b 建表迁移一次建立：`content_version bigint not null default 1`；`approval_legal_entity_id uuid`、`approval_scenario text`、`submitted_by uuid`、`submitted_at timestamptz`、`approval_ref uuid`、`approval_chain_id uuid`、`approval_chain_version_no int`、`approval_definition_digest bytea`、`approval_content_version bigint`、`approval_content_hash text`、`approved_by uuid`、`approved_at timestamptz`、`rejected_by uuid`、`rejected_at timestamptz`、`rejected_reason text`。`signer_subject` wire 固定 `spki-sha256:<64 lowerhex>`。`ck_config_packages_approval_shape` 强制 DRAFT/PENDING_AUTOTEST/TEST_FAILED/TEST_PASSED 的全部审批列为空，PENDING_APPROVAL 及以后才完整保存法人、申请人、流程、链与内容快照；PENDING_APPROVAL 无结论，REJECTED 只带拒绝，APPROVED 及以后只带批准且不可自审。special 的请求头法人不是来源，只能与命令派生 governance context 相等。法人、审批链与三组用户证据分别建真实 FK/复合 FK；`approval_ref` 保留平台证明白名单。

### 18.1 F-56 特殊许可/模块内容项与终态 ItemKind

`LICENSE_GRANT|MODULE_PACKAGE` special 包的 `RELEASED` 是永久终态，首次 RELEASE 后不得进入 `SUPERSEDED|ROLLED_BACK`，也不参与普通 lineage 自动替代。新 grant/revoke/module action 各自新建另一份仍为 RELEASED 的单项包，多份 special RELEASED 同时存在是正确历史形状；current/history/superseded grant 与 current module 只由 `license_grants/module_registrations` 投影及 source FK 表达，不借 config package status 表达。093300 的 deferred graph 在 COMMIT 拒绝 special SUPERSEDED/ROLLED_BACK、RELEASED 接受摘要为空、非 RELEASED 摘要非空或任何清摘要，并扫描全部 special RELEASED history 而不是只扫 current。

`platform_meta.config_package_items` 的 Stage 3 Rust `ItemKind::ALL` 与数据库 CHECK 必须同序恰为 18 项：原前 16 项后接 `LICENSE_GRANT`、`MODULE_PACKAGE`。`V20261022090500__platform_meta_alter_config_package.sql` 在尾部追加 `MCP_CONNECTOR`、`MCP_MANIFEST_VERSION`，并在同一交付批更新 Rust `ItemKind::ALL` 与 `ck_config_package_items_item_kind` 到终态 20；不得出现一侧 18、一侧 20 的可发布版本。Stage 3 的 090500 随建表创建 `accepted_trust_bundle_sha256 bytea null` 与“null 或恰 32 bytes”CHECK，但不创建父候选键；093300 才补 `UNIQUE(config_package_id,id)`，供 §6.24/6.25 六条同包 source FK 引用。

全平台 `item_hash` 兼容算法固定为：ADD/MODIFY=`SHA-256(JCS(after_spec))`，REMOVE=`SHA-256(JCS(before_spec))`，保存 64 位 lowerhex；按 change kind 被选中的 spec 必须非 null，禁止对 null 求摘要。kind/code/change/sort/scope 与 spec 形状由已签 manifest、表 CHECK 与每阶段重算共同绑定。F-56 special 固定 ADD，其 `item.jcs` 只保存 `after_spec` 的 RFC 8785 JCS exact bytes，MODULE_PACKAGE 的 action/reason 已在该 after_spec 内。

含 `LICENSE_GRANT|MODULE_PACKAGE` 的包必须 `source=IMPORTED,item_count=1`；唯一 item 必须 `change_kind=ADD,before_spec=null,after_spec!=null,applies_to_legal_entity_ids=[]`。两种 after_spec 分别为 strict signed license artifact 与 strict module item+signed artifact。每个 payload JCS/CMS 上限 1,048,576 bytes，signature 为 canonical base64url-no-pad。DRAFT 到 TEST_PASSED 期间 approval 法人列必须为空；每个推进命令从首张候选 signed governance id 或唯一首次 RELEASED grant history 派生 `governance_context_id`，并要求当前 session/operator 对该法人具有对应权限，请求头若提供只能相等。submit 同事务才首次写 approval 法人，之后不可改变。archive/ZIP/TOML/item JSON/base64 的语法、上限、CRC 或 entry 错误统一 `PLATFORM.REQUEST.INVALID_PAYLOAD`/400/零落库；typed item hash 不等用 `PLATFORM.CONFIG_PACKAGE.ITEM_HASH_MISMATCH`；CMS 密码学失败用 `PLATFORM.CONFIG_PACKAGE.SIGNATURE_INVALID`；链/CRL/EKU/root/subject 失败用 `PLATFORM.CONFIG_PACKAGE.SIGNER_NOT_TRUSTED`；strict DTO 已成功后的 special 业务 shape/metadata/governance 偏离才用 `PLATFORM.CONFIG_PACKAGE.SPECIAL_ITEM_SHAPE_INVALID`。`PLATFORM.MODULE.PACKAGE_INVALID_OR_INCOMPATIBLE` 只用于签名/信任通过后的版本、contract、maintenance、history identity 或兼容性失败。

`ConfigItemApplier::validate` 无 `Tx`，只允许 pure、deterministic 的 syntax/shape 校验；不得读取数据库、KMS、文件、current license/module 或其他可变外部事实，其成功不是发布授权。F-56 两个 `apply` 只能从锁内数据库持久化的 exact package/item bytes 重跑 signature/trust/current/source/dependency/governance 与业务守卫，只有 locked apply 全部通过才可提交；事务外 safe-parse 或 earlier validate 不能替代。

`pre_idempotency_lock` 使用同一 advisory key `hashtextextended('platform-license-current',0)` 的三值闭集。会产生 `BusinessWrite|BusinessApproval|IntegrationOutbound|AutomationStart` 的普通 handler/job 在 `BEGIN/SET LOCAL` 后以第一条业务 SQL 取 transaction-level shared lock，随后才可 `try_begin`、query/claim、row/module lock 并重验 admission；普通 shared 事务彼此并发。F-56 special 推进、current grant/revocation 替换与 trusted-time checkpoint 取 transaction-level exclusive lock；exclusive 等既有 shared 副作用事务排空，后续请求排队并在锁内重验。纯读及冻结 effect 为 `ReadReportAuditBackupExport|IdentitySecurityDisposition|ComplianceDisposition|InFlightConvergence` 的允许路径可按 binding 取 NONE；`LicenseGrantRecovery|ModuleDisableRecovery` 只决定 Restricted 准入，只能由 strict `ConfigRelease` 目标在已取 exclusive 的事务内派生，绝不赋予 NONE。Outbox/worker claim 短事务取 shared 并重验；真正外发前以专用连接取得同 key 的 session-level shared，再取 module session shared，重验后持到外部副作用/取消终结并 finally 释放，禁止跨外部调用持数据库事务。

所有可能推进 F-56 special 的事务唯一锁序为：`BEGIN`/mandatory session-context `SET LOCAL` 后，第一条业务 SQL 取 `pg_advisory_xact_lock(hashtextextended('platform-license-current',0))`；随后才可 `try_begin` 或 query/claim。总序固定为 license exclusive →（仅 ordinary execute）`platform_meta.config_release_mutex FOR UPDATE` → `(config_package_id,release_order_id,item.sort_no,item.id)` canonical rows → ModuleCode wire 顺序的 module locks；ordinary execute 的连接 1 持有 license/mutex 直至 COMMIT，special execute 不取 mutex 且跳过 DDL 段一。最后才锁内重读 current/history/source/dependency并写 projection/package/order/Outbox/audit。import 可在事务前 safe-parse 但结果非权威；autotest/submit/approve/sign/create-release-order/execute 与 autotest accept/claim/lease/final aggregate 的每个短事务都无条件取 exclusive，普通配置包走这些共享入口也不例外；九个 suite 的纯只读查询事务可 NONE。GRANT/REVOCATION/MODULE_PACKAGE applier 可幂等重取同一 transaction-level exclusive。reject 是进入事务前已由 typed branch 固定的唯一 ConfigRelease 写结论例外，取 NONE 并只闭合同一 immutable content hash；不得无锁查包后在 approve/reject 间改判。

特殊包仍完整经过九套 autotest、`CONFIG_RELEASE` 非自审审批、外层签名和 RELEASE 执行，但不能创建 `action=ROLLBACK`；创建命令先按上述 whole-transaction lock order 取得 license lock 与 package/order/items 后，命中任一特殊 kind 即以 `PLATFORM.CONFIG_RELEASE_ORDER.NON_ROLLBACKABLE_ITEM`（409、不可重试）零写入拒绝。`LicenseGrantApplier` 与 `ModulePackageApplier` 由 Stage 3b `ep-platform-license` 实现，其 `revert` 不在合法编排图内且误调同样失败；发布后只能用新的已签名 grant/revoke 或模块动作单项包继承，历史包、item、许可与模块来源投影均不可删除。special RELEASE 事务在 exact archive 投影、outer/inner、item/content hash 与当前 bundle 全部复验后，才把唯一 item 的接受摘要一次 `null→32 bytes`，同事务执行 applier 投影并把 package/order 置 RELEASED/SUCCEEDED；失败全回滚，非空不可改/不可清，幂等重放只接受 same-byte。093300 的 DEFERRABLE F-56 graph 挂 `config_packages/config_package_items/module_registrations/license_grants/legal_entities` 五表，在 COMMIT 强制普通 item 恒 null、special 未 RELEASED 为 null/RELEASED 恰32、grant 摘要等于 source item、治理法人/approval/active 图一致，以及两套 RELEASED module history identity 一一映射；revocation/module action 不复制摘要列。

首次 RELEASE 同事务以审计终结批写唯一 `action='platform.config_special.accepted.v1'`；完整 envelope 还固定 `event_id` 为本次 RELEASE terminal batch 前预分配的新 UUIDv7，`legal_entity_id` 为冻结治理法人，`actor_user_id/actor_device_id/client` 逐字取 execute 的受信 `SecurityContext`，`object_type='platform.config_package_items'`，`object_id=config_item_id`，`object_version` 为 terminal item row_version，`before=null`，`after=上述 payload`，`reason=null`，`approval_ref=config_packages.approval_ref`，`reauth_ref=null`，`occurred_at=accepted_trusted_now`；`event_day/seq/prev_hash/hash` 只由 AuditWriter 既有分段链算法派生；same-byte 回放不重复写。payload exact strict-JCS 为 `{schema_version:1,purpose:"EP-CONFIG-SPECIAL-ACCEPTED-V1",config_package_id,config_item_id,artifact_kind,artifact_id,artifact_action,accepted_trusted_now,accepted_trust_bundle_sha256,inner_signer_subject,inner_chain_sha256,inner_trust_state,outer_signer_subject,outer_chain_sha256,outer_trust_state,payload_sha256,item_hash,content_hash,source_projection_sha256}`。该具名 typed-audit DTO 的 `schema_version` 是 JSON number `1`，不是 string；无具名 typed-audit ABI 时业务 decimal/integer 才使用 canonical 十进制 JSON string，不能反向覆盖本闭集。artifact kind/id/action、inner state `ACTIVE|RETIRED_NONREVOKED|REVOKED_AS_DISABLE_TARGET` 与 outer ACTIVE 的动作限制以 F-56 为准。chain digest 唯一为 `SHA-256(ASCII("EP-CMS-CHAIN-V1")||0x00||leaf→anchor 每张 exact DER 的 u32be(length)||DER)`。source digest 唯一为 `SHA-256(ASCII("EP-CONFIG-SPECIAL-SOURCE-PROJECTION-V1")||0x00||JCS(dto))`，`dto` exact 为 `{schema_version:1,purpose:"EP-CONFIG-SPECIAL-SOURCE-PROJECTION-V1",config_package_id,package_no,source:"IMPORTED",status:"RELEASED",content_hash,outer_signature_sha256,outer_signer_subject,outer_signed_at,config_item_id,item_kind,item_code,change_kind:"ADD",sort_no:1,applies_to_legal_entity_ids:[],before_spec_sha256:null,after_spec_sha256,item_hash,accepted_trust_bundle_sha256}`；outer signature 与 item.jcs 分别重算两个摘要。缺键、unknown key、摘要/链/source 投影不等均使 RELEASE 整笔回滚。

special `.epcfg` 是 file cap=4,193,900 bytes 的单卷 ZIP32/STORE archive，exact entries 为 `manifest.toml,item.jcs,outer-signature.p7s`，固定 ZIP overhead=330 bytes；禁止 ZIP64/encryption/data descriptor/extra/comment/directory/重复或大小写碰撞/path escape/link-or-reparse/尾随或嵌套 archive，并固定 DOS 时间、CRC 与 local/central size/offset。entry 上限为 item.jcs=2,882,850、canonical manifest=262,144、outer detached CMS=1,048,576 bytes，四数相加恰为 file cap。manifest 以 exact key order/格式绑定 item kind/code/change/sort/scope/before-null/hash，import 后必须从保存列经同一 canonical writer 重建；outer CMS detached content 恰为 manifest exact bytes且新 signer ACTIVE，special outer/inner 虽共同只信 license-roots.p7b仍独立验证，普通 KMS outer 是第三条路径。

普通配置包的 `actions/sign` 使用独立只读 `KmsSigningKeyIdentityResolver` 补齐 signer identity，但不改变 `KmsBackend` 六方法。配置 secret ref 每次只解析一次为 immutable/versioned `KeyRef`；流程固定为 resolve `SigningKeyIdentityV1 { key_ref,spki_sha256 }` before→同 ref `KmsBackend::sign(content_hash exact bytes)`→同 ref `verify=true`→resolve after 且 exact equal。`SigningKeyIdentityV1::signer_subject()` 唯一输出 `spki-sha256:<64 lowerhex>`；摘要来自该 KeyRef 公钥 exact DER SPKI，resolver 不返回私钥。全部成功才在锁定包的同一事务写 canonical `signature_key_ref/signer_subject/signature/signed_at` 并推进；ref/identity 漂移、不可解引用、假 token、错 ref 或 verify=false 均零推进。Builtin/HSM adapter 同时实现 resolver，轮换只影响后续签名、不回填历史。

F-56 imported special 在 import 时把已通过 publisher outer verifier 的发行方 `signature` exact bytes、`signer_subject`、`signed_at` 原样写入 `config_packages`，其中 signer token 必为 `spki-sha256:<64 lowerhex>`、时间只取 manifest/CMS 一致的 signed_at，`signature_key_ref=null` 且包立即不可修改；其 `actions/sign` 只重算 hash、用 canonical writer 重建 manifest exact bytes、复验并逐字保留这组三值后推进状态，部署 KMS sign 与 identity resolver 调用必须均为零。artifact 内层 CMS 不走普通外层 KMS verifier，只认固定 `C:\ProgramData\EnterprisePlatform\trust\license-roots.p7b`，时间只取 payload/manifest issued_at；不得读取 Windows 任意根、联网补链或临时根。普通 outer、special outer 与 inner 三项验证互不替代。

Control Center（受信 `ClientKind::Ops`）只组合既有 API，不新增路由或手工布尔开关。`.epcfg` multipart 仍使用 `POST /api/v1/platform/config-packages/actions/import`，该同一路由唯一获得编译期 route-local 4,194,304-byte body limit。未加引号的 boundary 是 1..70 ASCII bytes，冻结 HTTP-token 安全子集为 `[A-Za-z0-9'._+-]`；无 preamble/epilogue且 CRLF-only；恰一个 `name="package"` file part，Content-Disposition 与 exact MIME `application/vnd.enterprise-platform.epcfg+zip` 两个 headers 以冻结顺序出现，filename 为匹配 `[A-Za-z0-9][A-Za-z0-9._-]{0,121}\.epcfg` 的 7..128 ASCII bytes，零其他 header/part/form field，结尾为规范 closing boundary。framing 恰 `136+2*boundary_len+filename_len<=404`，archive/file cap=4,193,900。

`Content-Length` 必填、为规范十进制 `1..=4,194,304` 且等于 framing+archive；任何 Transfer-Encoding、长度/boundary/framing/header/MIME/filename/part 偏离都在读 body/建 staging 前拒绝，逐流同时以 body/file 两个 cap 拒绝短读/长读。全部失败只返回既有 `PLATFORM.REQUEST.INVALID_PAYLOAD`，HTTP 400、不可重试、零配置包落库，不新增 413 或同义码。其他路由与全局 1 MiB 上限不变。Restricted 时该向导可发起 LICENSE_GRANT 全链和 MODULE_PACKAGE/DISABLE 全链，但 `CONFIG_RELEASE` 批准/驳回只允许 Win/Mac 既有待办；Control Center 的 `ops` origin 仅只读显示待办与结论，绝不调用 approve/reject，且通用 `CompleteProcessTask` 必须读取 task subject 的特殊 item 后执行同一 client guard，不能成为旁路。其余写统一 `PLATFORM.LICENSE.RESTRICTED`。

`GET /api/v1/platform/client-bootstrap` 的 `license_module_admin` 字段在所有 client 响应中都必须存在：仅来自权威 Control Center、已认证为受信 `ops` 且具备 `lowcode.config_package.view` 时非空；F-55 `ops` wire token 不再接受。其他情况逐字为 JSON null。非空对象所有键始终存在，exact 字段为 `license_no_masked,license_kind,license_status,restriction_reason,valid_from,valid_to,maintenance_valid_to,last_trusted_at,usage,module_codes,entitlement_codes,modules`；前八项不可用的 Option 必须逐字为 JSON null，绝不省略。status/reason/trusted time 取同一次 `license_evaluation()`。`usage` exact object 恰有 `legal_entities,named_users,registered_devices` 三键，每项 `limit,current,over_limit` 三键始终存在，形状为 `{limit:u32|null,current:u64,over_limit:bool|null}`；`limit=null` 时 `over_limit=null`，否则严格等于 `current>limit`。modules 恰 15 行且每行 `module_code,display_name,install_state,package_trust_status,package_code,package_version,state_changed_at` 七键始终存在，后三项不可用时为 JSON null；`package_version` 非空时复用 strict `SemVerV1` object。`package_trust_status` 闭集为 `NOT_INSTALLED|TRUSTED|SIGNER_REVOKED|INVALID`，从 current bundle 对 source item/投影重算，install_state 不等同 effective runtime。可信 `license_no` 至少四个 Unicode scalar 时 `license_no_masked="****"+最后四个 scalar`，不足四个时固定 `"****"`，不得泄漏原长度。零 current 固定 `RESTRICTED/NO_CURRENT_GRANT`，身份/日期/trusted time 为 null、code 集为空，usage 仍报三个实际 current 但 limit/over_limit 为 null；SIGNATURE_INVALID 不得显示未受信 current 的身份、日期、code 或 limit；其他 Restricted 原因只显示已完整验签的脱敏字段。两个 code 集与 modules 按 wire bytes 排序；unknown key、任一 missing key、把 null 与省略互换都属于 OpenAPI/序列化契约失败。禁止返回 signature/payload/source/path/key/secret 或原始 license_no。

### 18.2 扩展启用审批与授权图

`platform_meta.extensions` 与 `platform_meta.extension_capability_grants` 都是部署级表，但每次启用审批固定一个 `approval_legal_entity_id`。扩展表保存不可变制品身份、规范化 `capability_manifest/manifest_hash`、规范化 `requested_grants/requested_grants_hash`，以及 `EXTENSION_ENABLE` 的申请人、流程、链 id/版本/digest、artifact/manifest/requested-grants 三项审批摘要和批准/拒绝结论。审批链与三组用户证据采用与 config package 相同的复合真实外键；`approval_ref` 仍是平台证明白名单。REGISTERED 无审批证据；PENDING_APPROVAL 只有完整申请快照；REJECTED 只有拒绝结论且终态；APPROVED/ENABLED/DISABLED/REVOKED 只有批准结论且申请人不可自审；DISABLED 必有原因，REVOKED 终态。

grant 行的 `scope_key` 非空：READ_OBJECT_FIELDS 等于规范化 object_type，其他三类等于 `-`；`active_slot` 在未撤销时固定 1，撤销后为空。普通唯一键 `(extension_id,capability,scope_key,active_slot)` 只封住单一有效 grant，同时允许保留多条撤销历史并重新授予。复合父键同时绑定 extension kind、审批法人和 `approval_ref`，`granted_by` 以 `(approval_legal_entity_id,granted_by)` 指向法人授权。`assert_extension_enable_graph_consistent()` 作为 DEFERRABLE INITIALLY DEFERRED 约束触发器覆盖父表与 grant 表：ENABLED 必须已有闭合批准证据，有效 grants 的规范数组必须精确等于请求快照且逐项被 manifest 包含；撤销有效 grant 的事务必须先把父扩展置 DISABLED；REJECTED/REVOKED 无有效 grant。停用、撤销和升级均保留既有制品、授权历史与业务数据。

普通 `config_package_items.change_kind=REMOVE` 只退休对应元数据并从 API、UI、查询注册表与客户端引导隐藏；普通配置发布的 `ddl_plan_steps.sql_kind` 闭集不含 `DROP_COLUMN`、`DROP_TABLE`。物理表、列和数据保留，发布、回退与中途失败前后受影响表 row_count 与业务列 checksum 必须不变。物理处置只能由阶段 14 独立 `DisposalPort` 双人审批流程执行，普通配置包与其 applier 不得调用；F-56 两类特殊项不允许 REMOVE 或通用回退，只允许新的签名动作继承。
