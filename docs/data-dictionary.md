# 数据字典

本文件是全部业务表与全局码表的唯一登记处。表结构以迁移文件为准，本文件登记的是列语义、取值域与跨阶段共用的码表；两处不一致时以迁移文件为准并同批修正本文件。

## 1. 组织方式

按 schema 分节，每个 schema 一节，节内按表名字典序。每张表一张列表，列固定为：列名、类型、可空、默认、语义。

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

`created_by` 与 `updated_by` 在系统上下文下一律写入 `ep_foundation::principal::SYSTEM_PRINCIPAL_ID`，字面量为 `00000000-0000-7000-8000-000000000001`。该取值同时满足 UUIDv7 的版本位与变体位校验，且不可能与 ID 生成器产出的任何值碰撞，因此不需要另设一个「系统用户」记录来占位。

四类附加列：

- 单据类表另加 `doc_no text not null` 与 `status text not null`，`status` 带 CHECK 约束枚举该单据状态机的全部取值。
- 档案类表另加 `code text not null`、`is_active boolean not null default true`、`deactivated_at timestamptz null`。
- 会计相关表另加 `posting_date date` 或 `business_date date` 与 `accounting_period_id uuid`。
- 仅追加表不带 `row_version`、`updated_at`、`updated_by`；是否带 `reverses_id uuid null` 由该表有无业务冲销或更正语义决定，有的必须带并写明它指向哪张表的哪条记录，没有的一律不得带，不得为满足列约定而保留一个恒为 NULL 的该列。

不设 `tenant_id` 或 `customer_id` 列：客户隔离由「每个客户一个独立事务数据库实例」的部署形态承担，再加一列只会制造第二套隔离口径与两处越权测试面。

附件引用不落在业务表列上，一律经 `<主表单数>_attachments` 关联表，列为 `owner_id`、`attachment_object_id`、`purpose`、`sort_no` 与公共列。

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

本阶段登记条数为 0。理由不是遗漏：阶段 1 不引入任何单据与任何档案，一个类型码都没有被测对象。各类型码由其单据或档案所在阶段登记，任何阶段不得新增未在本节登记的码。

### 5.2 判定

代码侧的对应物是 `ep-platform-sequence` 的类型码常量表，判据是本节与该常量表逐项一致且无重复，由 CI 项 `xtask configdoc --check-doc-type-codes` 执行。

该常量表由阶段 3a 交付。在它存在之前，「逐项一致且无重复」的被测输入为空集，比对恒真；恒真的判据不作为通过判定，因此该逐项比对整条推迟到阶段 3a 生效。阶段 1 对本节只判一件事：本节存在。

## 6. platform_core schema（阶段 2 与阶段 4）

本 schema 承载密钥域、数据密钥、敏感字段清单、仅追加登记、迁移窗口与未受行级策略表登记六类平台元数据，以及按裁定 A-04 归入的集团、组织、部门、岗位与部门层级闭包五张组织架构表（阶段 2，共十三张，含表六附带的单例锁表），以及阶段 4 任务 #20 交付的九张身份主体表（账号、凭据、口令历史、设备、会话、复核挑战、登录尝试、锁定窗口与应急启用），共二十二张。凡不带 `legal_entity_id` 的表一律登记在 `unpoliced_table_registry`，由 `db/checks/13` 强制：阶段 2 登记八行，阶段 4 九张身份主体表与 `platform_authz` 的 `permission_items`、`object_scope_bindings` 两张由第 29 号回填迁移一次写入（04 计划第 12.2 节偏离一）。

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
| kek_ref | text | 否 | 无 | KEK 引用，形如 `kms://builtin/le/<uuid>` |
| kek_version | int | 否 | 1 | KEK 版本，大于 0 |
| provisioned_at | timestamptz | 是 | 无 | 供给完成时间 |
| destroy_planned_at | timestamptz | 是 | 无 | 销毁计划时间 |
| destroyed_at | timestamptz | 是 | 无 | 销毁完成时间 |
| destroy_evidence_ref | text | 是 | 无 | 销毁证明的审计引用 |

唯一约束 `ux_key_domains_legal_entity_id_domain_kind`：一个法人至多一个同类密钥域。

### 6.3 data_keys（数据密钥台账）

带 `legal_entity_id`，策略 `rls_data_keys_le`。`security_level` 默认 40。

| 列名 | 类型 | 可空 | 默认 | 语义 |
|---|---|---|---|---|
| key_domain_id | uuid | 否 | 无 | 所属密钥域，同 schema 真实外键，ON DELETE RESTRICT |
| purpose | text | 否 | 无 | `FIELD`、`BLIND_INDEX`、`ATTACHMENT`、`ARCHIVE` 四用途 |
| security_level_scope | smallint | 否 | 无 | DEK 服务的密级子域，取 10、20、30、40 |
| version | int | 否 | 无 | 同域同用途同子域内的版本号，大于 0 |
| algorithm | text | 否 | 无 | `AES_256_GCM` 或 `HMAC_SHA256` |
| wrapped_key | bytea | 否 | 无 | KEK 信封后的 DEK |
| wrap_kek_version | int | 否 | 无 | 包裹时的 KEK 版本 |
| state | text | 否 | 无 | `ACTIVE`、`RETIRING`、`RETIRED`、`DESTROYED` 四态 |
| activated_at | timestamptz | 否 | 无 | 生效时间 |
| retiring_at、retired_at、destroyed_at | timestamptz | 是 | 无 | 轮换与销毁时点 |

唯一约束 `ux_data_keys_domain_purpose_scope_version` 在 `(key_domain_id, purpose, security_level_scope, version)` 四列上：该名 50 字节未达 63 字节标识符上限，按全称保留；其列序全称形态 `ux_data_keys_key_domain_id_purpose_security_level_scope_version` 因超限不采用，此处登记备查。首版不使用部分索引，取当前有效密钥按该 ux 前缀定位后 `order by version desc limit 1`。

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
| home_legal_entity_id | uuid | 否 | 无 | 归属法人，只作审计分段与默认法人 |
| supplier_ref_id | uuid | 是 | 无 | 供应商账号引用 |
| clearance_level | smallint | 否 | 20 | 账号自身许可等级，取 10、20、30、40 |
| status | text | 否 | 无 | `UNACTIVATED`、`ACTIVE`、`LOCKED`、`SUSPENDED`、`DEACTIVATED` 五态 |
| is_mfa_required | boolean | 否 | false | 是否强制多因子 |
| activated_on | date | 是 | 无 | 启用日；停用取 `deactivated_at` |
| last_login_at | timestamptz | 是 | 无 | 最近登录时刻 |

时间序索引按偏离三取 `ix_user_accounts_status_created_at`（本表无 `legal_entity_id` 列）。

### 6.14 user_credentials（认证凭据，阶段 4）

不带 `legal_entity_id`、不建策略，登记于表十三。`security_level` 默认 40。`user_id` 外键指向 user_accounts，ON DELETE RESTRICT。

| 列名 | 类型 | 可空 | 默认 | 语义 |
|---|---|---|---|---|
| credential_kind | text | 否 | 无 | `PASSWORD`、`TOTP`、`WEBAUTHN_PLATFORM`、`WEBAUTHN_ROAMING`、`X509_CERT` 五类 |
| verifier | text | 是 | 无 | PASSWORD 存 Argon2id 的 PHC 串；X509_CERT 存证书指纹 |
| public_key、credential_handle | bytea | 是 | 无 | WebAuthn 两类凭据的公钥与凭据句柄；`credential_handle` 全库唯一 |
| secret_ref | text | 是 | 无 | TOTP 种子只存机密引用，形如 `secret://kms/totp/<user_id>#<ver>`，种子本体在 KMS |
| sign_count | bigint | 否 | 0 | WebAuthn 签名计数 |
| status | text | 否 | 无 | `ACTIVE`、`SUSPENDED`、`REVOKED`、`EXPIRED` 四态 |
| activated_at | timestamptz | 否 | now() | 生效时刻 |
| expires_at、last_used_at、revoked_at | timestamptz | 是 | 无 | 到期、最近使用与吊销时点 |

`ck_user_credentials_material` 按 kind 强制对应载体非空：口令与证书类要有 verifier，WebAuthn 类要有 public_key 与 credential_handle，TOTP 类要有 secret_ref。

### 6.15 user_password_history（口令历史，仅追加，阶段 4）

不带 `legal_entity_id`、不建策略，登记于表十三，并登记于 `append_only_registry`（mode 取 `APPEND_ONLY`，不带 row_version 与 updated_* 两对列）。`security_level` 默认 40。自有列：`user_id`（不可空）、`verifier`（不可空，历史口令的 Argon2id PHC 串）。索引 `ix_user_password_history_user_id_created_at`。

### 6.16 user_devices（设备登记，阶段 4）

不带 `legal_entity_id`、不建策略，登记于表十三。`security_level` 默认 30。

| 列名 | 类型 | 可空 | 默认 | 语义 |
|---|---|---|---|---|
| user_id | uuid | 否 | 无 | 所属用户 |
| device_id | text | 否 | 无 | 设备标识，长度 1 至 64，全库唯一 |
| client | text | 否 | 无 | 六端取值 `win`、`mac`、`ios`、`android`、`portal`、`ops` |
| public_key | bytea | 是 | 无 | 设备密钥（WebAuthn 形态） |
| attestation_ref | text | 是 | 无 | 核验证明引用 |
| restricted_legal_entity_id | uuid | 是 | 无 | 受限于单一法人的设备授权约束，认证中间件与授权集合取交集 |
| status | text | 否 | 无 | `PENDING`、`ACTIVE`、`REVOKED` 三态 |
| registered_at | timestamptz | 否 | now() | 登记时刻 |
| revoked_at、last_seen_at | timestamptz | 是 | 无 | 吊销与最近出现时点 |

### 6.17 sessions（会话，阶段 4）

不带 `legal_entity_id`、不建策略，登记于表十三。`security_level` 默认 30。令牌只存 SHA-256 摘要，明文不落库也不进日志。

| 列名 | 类型 | 可空 | 默认 | 语义 |
|---|---|---|---|---|
| user_id | uuid | 否 | 无 | 所属用户 |
| user_device_row_id | uuid | 否 | 无 | 设备行引用，外键指向 user_devices，ON DELETE RESTRICT |
| token_hash | bytea | 否 | 无 | 令牌 SHA-256 摘要，全库唯一 |
| active_legal_entity_id | uuid | 否 | 无 | 本会话活动法人 |
| client | text | 否 | 无 | 六端取值，同 user_devices |
| issued_at | timestamptz | 否 | now() | 签发时刻 |
| expires_at | timestamptz | 否 | 无 | 绝对到期；滑动续期合并事务只刷新下行两列 |
| idle_expires_at | timestamptz | 否 | 无 | 空闲到期，续期写入 now + 空闲超时 |
| last_seen_at | timestamptz | 否 | now() | 最近核验时刻 |
| revoked_at | timestamptz | 是 | 无 | 撤销时刻 |
| revoke_reason | text | 是 | 无 | 撤销理由，长度不超过 128 |
| is_breakglass | boolean | 否 | false | 是否应急账号会话，供 `ep_breakglass_active_sessions` 分计 |

索引：`ix_sessions_user_id_expires_at`、`ix_sessions_last_seen_at`、时间序 `ix_sessions_created_at`。

### 6.18 reauth_challenges（复核挑战，阶段 4）

不带 `legal_entity_id`、不建策略，登记于表十三。`security_level` 默认 30。待签内容摘要由服务端按规范化算法重算，不采信客户端传值（04 计划第 4.4 节）。

| 列名 | 类型 | 可空 | 默认 | 语义 |
|---|---|---|---|---|
| user_id、session_id | uuid | 否 | 无 | 发起人与发起会话 |
| operation_type | text | 否 | 无 | 六类高危操作：`CONTRACT_EFFECTIVE`、`PAYMENT`、`INVOICE_ISSUE`、`LEDGER_POSTING`、`PERIOD_CLOSE`、`SENSITIVE_EXPORT` |
| subject_digest | bytea | 否 | 无 | 待签内容 SHA-256 摘要 |
| subject_summary | jsonb | 否 | 无 | 摘要前的五项规范化明文（审计展示用） |
| nonce | bytea | 否 | 无 | 防重放随机数 |
| credential_kind_used | text | 是 | 无 | 验证时使用的凭据种类 |
| status | text | 否 | 无 | `ISSUED`、`VERIFIED`、`CONSUMED`、`FAILED`、`EXPIRED`、`ABANDONED` 六态 |
| token_hash | bytea | 是 | 无 | 复核令牌摘要，验证后写入，全库唯一，一次性消费 |
| issued_at、expires_at | timestamptz | 否 | 无 | 签发与到期 |
| verified_at、consumed_at | timestamptz | 是 | 无 | 验证与核销时点 |
| failure_count | int | 否 | 0 | 验证失败次数，不小于 0 |

索引：`ix_reauth_challenges_user_id_status_expires_at`、时间序 `ix_reauth_challenges_created_at`。

### 6.19 login_attempts（登录尝试，仅追加，阶段 4）

不带 `legal_entity_id`、不建策略，登记于表十三，并登记于 `append_only_registry`（mode 取 `APPEND_ONLY`，不带 row_version 与 updated_* 两对列）。`security_level` 默认 30。自有列：`user_id`（可空，账号不存在时只有哈希）、`login_name_hash`（bytea，哈希存储防攻击者注入明文）、`outcome`（八值 `SUCCESS`、`CREDENTIAL_INVALID`、`ACCOUNT_LOCKED`、`ACCOUNT_INACTIVE`、`MFA_REQUIRED`、`MFA_INVALID`、`DEVICE_UNREGISTERED`、`ADMISSION_REJECTED`）、`client`（可空）、`source_addr`（可空，长度不超过 64）、`occurred_at`。索引 `ix_login_attempts_occurred_at`（限流与清理）与 `ix_login_attempts_user_id_occurred_at`（锁定窗口判定）。

### 6.20 account_lockouts（锁定窗口，阶段 4）

不带 `legal_entity_id`、不建策略，登记于表十三。`security_level` 默认 30。自有列：`user_id`（唯一约束 `ux_account_lockouts_user_id`，一人至多一行）、`failure_count`（int 不小于 0，默认 0）、`window_started_at`（默认 now()）、`locked_until`（可空，取非空即处于锁定）、`last_failure_at`（可空）。索引 `ix_account_lockouts_locked_until` 供到期解锁清理扫描。

### 6.21 breakglass_activations（应急账号启用，单据类，阶段 4）

不带 `legal_entity_id`、不建策略，登记于表十三。`security_level` 默认 40。`doc_no` 类型码 `BGA`，全库唯一（本表无 `legal_entity_id` 列，唯一约束不带法人）；该类型码在 §5.1 的登记随编号生成本体（属 3b 同批）一并补齐。

| 列名 | 类型 | 可空 | 默认 | 语义 |
|---|---|---|---|---|
| status | text | 否 | 无 | `DRAFT`、`PENDING_APPROVAL`、`APPROVED`、`ACTIVE`、`EXPIRED`、`CLOSED`、`REJECTED` 七态 |
| user_id | uuid | 否 | 无 | 被启用的应急账号 |
| requested_by、approved_by | uuid | 否、是 | 无 | 申请人与批准人 |
| reason | text | 否 | 无 | 启用理由，长度 1 至 2000 |
| approval_ref | text | 是 | 无 | 审批引用 |
| allowed_action_set | text[] | 否 | 无 | 非空子集，取 `UNLOCK_OR_RESET_ADMIN`、`RESTORE_CONTROLLED_CONFIG_RELEASE`、`TRIGGER_BACKUP_OR_RESTORE` 三值 |
| activated_at、expires_at、closed_at | timestamptz | 是 | 无 | 启用、到期与关闭时点 |
| rotated_at | timestamptz | 是 | 无 | 关闭同事务内凭据轮换完成时点（退出条件 14） |
| rotation_result | text | 是 | 无 | 轮换结果 |

索引 `ix_breakglass_activations_status_expires_at` 供到期失效扫描，时间序取 `ix_breakglass_activations_created_at`。

## 7. platform_ops schema（阶段 2）

### 7.1 degradation_windows（降级窗口台账）

不带 `legal_entity_id`、不建策略，登记于 `unpoliced_table_registry`。列定义按裁定 A-26 以阶段 14 计划为准，本阶段建表并交付两条约束；写入端口在 `ep-platform-obs`。

| 列名 | 类型 | 可空 | 默认 | 语义 |
|---|---|---|---|---|
| kind | text | 否 | 无 | `OFFSITE_SINK_NOT_CONFIGURED`、`WRITER_NOT_IN_SERVICE`、`PORT_NOT_IMPLEMENTED` |
| subject | text | 是 | 无 | 开窗对象的完整类型名（端口名或平台能力名），长度不超过 200 |
| scope_key | text | 否 | 无 | 范围键，长度不超过 200 |
| scope_legal_entity_id、scope_accounting_period_id | uuid | 是 | 无 | 标注列，只作标注不作策略判据 |
| basis | text | 否 | 无 | 开窗依据，长度不超过 2000 |
| detail | jsonb | 否 | '{}' | 附加明细 |
| opened_at | timestamptz | 否 | 无 | 开窗时间 |
| closed_at | timestamptz | 否 | 'infinity' | 关窗时间；未关闭取无穷远，`ck_degradation_windows_open_order` 要求晚于 opened_at |
| closing_condition | text | 否 | 无 | 关窗条件描述，长度不超过 2000 |
| is_suppressible | boolean | 否 | 无 | 可否抑制；前两个 kind 取值不可抑制 |
| suppressed_until | timestamptz | 是 | 无 | 抑制截止时间 |

唯一约束 `ux_degradation_windows_kind_scope_closed` 建在 `kind`、`subject`、`scope_legal_entity_id`、`scope_accounting_period_id` 与 `closed_at` 五者上：同一对象至多一个未关闭窗口。

## 8. platform_authz schema（阶段 4）

本 schema 承载阶段 4 任务 #20 交付的十五张授权表：权限项与范围锚（2 张）、角色与授权（7 张）、审批链与高风险请求（4 张）、配置版本（1 张）与职责分离规则（1 张）。除 `permission_items` 与 `object_scope_bindings` 两张不带法人列（行集合对两个法人取值相同，登记于 `unpoliced_table_registry`，可见性由授权判定第二阶段的对象级判定承担）外，其余十三张逐张经 `platform_core.apply_le_rls` 生成行级策略，不写手工变体。

### 8.1 permission_items（权限项，阶段 4）

不带 `legal_entity_id`、不建策略，登记于表十三。自有列：`code`（唯一，形如 `sales.sales_order`，正则 `^[a-z][a-z0-9_]*(\.[a-z][a-z0-9_]*)+$` 且长度不超过 128）、`module_code`（15 个模块码或 platform）、`function_point`、`allowed_actions`（非空子集，恰取 `VIEW`、`CREATE`、`UPDATE`、`SUBMIT`、`APPROVE`、`EXPORT` 六动作）、`object_type`、`description`（可空）。约束 `ck_permission_items_forbidden_codes` 拒写以 `platform.legal_entity_isolation` 与 `platform.direct_db_access` 两前缀开头的 code，关闭或修改法人隔离机制与直连业务库两类权限项写不进本表。索引 `ix_permission_items_module_code`。

### 8.2 object_scope_bindings（对象范围锚登记，阶段 4）

不带 `legal_entity_id`、不建策略，登记于表十三，是记录级判定的落点。自有列：`object_type`（唯一）、`schema_name`、`table_name`（均不可空）、`owner_user_col`、`owning_dept_col`、`project_col`、`customer_col`（四锚列均可空）、`security_level_col`（不可空，默认 'security_level'）。没有登记的对象类型在记录级判定阶段一律拒绝，不默认放行；本阶段随建表回填 platform 自身三对象（platform.user_accounts、platform.roles、platform.high_risk_requests），业务对象登记在其所属阶段。

### 8.3 roles（角色，档案类，阶段 4）

带 `legal_entity_id`，策略 `rls_roles_le`。角色一律按法人建立，不做跨法人全局角色。自有列：`code`（法人内唯一，正则 `^[A-Z][A-Z0-9_]{0,63}$`）、`name`（1 至 200）、`duty_class`（可空，职责角色取 `SYSTEM`、`DATA`、`SECURITY`、`AUDIT`、`KEY`、`CONFIG` 六值，业务角色为空）、`is_portal_role`（默认 false）、`lifecycle_state`（`DRAFT`、`PENDING_RELEASE`、`EFFECTIVE`、`SUPERSEDED`、`RETIRED` 五态）、`retired_at`（可空）、`is_active`（默认 true）、`deactivated_at`（可空）。

### 8.4 role_permission_grants（角色权限授予，阶段 4）

带 `legal_entity_id`，策略 `rls_role_permission_grants_le`。自有列：`role_id`、`permission_item_code`（长度 1 至 128）、`action`（六动作之一）。唯一索引建在 `(legal_entity_id, role_id, permission_item_code, action)` 四列上：全称 `ux_role_permission_grants_legal_entity_id_role_id_permission_item_code_action` 共 80 字节超过 63 字节标识符上限，按列序缩写为 `ux_role_permission_grants_le_id_role_id_perm_item_code_action`（61 字节）：`legal_entity_id` 缩为 `le_id`、`permission_item_code` 缩为 `perm_item_code`，此处登记全称备查。

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

带 `legal_entity_id`，策略 `rls_approval_chains_le`。自有列：`code`（正则 `^[A-Z][A-Z0-9_]{0,63}$`，长度 1 至 64）、`name`（1 至 200）、`scenario`（长度 1 至 128，如合同生效、销项发票开具）、`version_no`（int 不小于 1，默认 1）、`lifecycle_state`（五态同 roles）、`is_active`（默认 true）、`deactivated_at`（可空）。唯一约束 `ux_approval_chains_legal_entity_id_code_version_no` 在 `(legal_entity_id, code, version_no)` 上。

### 8.13 approval_chain_nodes（审批链节点，阶段 4）

带 `legal_entity_id`，策略 `rls_approval_chain_nodes_le`。表上没有 allow_skip 一类列：越权跳过不是被校验拒绝的配置，而是根本没有承载它的字段。自有列：`approval_chain_id`、`node_no`（int 自 1 起，无空洞由静态校验承担）、`approver_kind`（`ROLE`、`POSITION`、`DEPT_MANAGER` 三类；ROLE 类经 `role_code` 引用，其余经 `approver_ref` 引用）、`approver_ref`（uuid 可空）、`role_code`（可空，长度 1 至 64）、`quorum`（int 不小于 1，默认 1）、`timeout_hours`（可空，不小于 1）。唯一索引建在 `(legal_entity_id, approval_chain_id, node_no)` 上：全称 `ux_approval_chain_nodes_legal_entity_id_approval_chain_id_node_no` 共 69 字节超限，缩写为 `ux_approval_chain_nodes_le_id_approval_chain_id_node_no`（55 字节）：`legal_entity_id` 缩为 `le_id`，此处登记全称备查。

### 8.14 high_risk_requests（高风险请求，单据类，阶段 4）

带 `legal_entity_id`，策略 `rls_high_risk_requests_le`。`doc_no` 类型码 `HRR`，法人内唯一；该类型码在 §5.1 的登记随编号生成本体（属 3b 同批）一并补齐。提交与审批四端点属 3b 同批交付，本阶段只建表与静态校验。

| 列名 | 类型 | 可空 | 默认 | 语义 |
|---|---|---|---|---|
| doc_no | text | 否 | 无 | 单据编号，长度 1 至 64，法人内唯一 |
| status | text | 否 | 无 | 十一态：`PENDING_INITIATION`、`PENDING_REAUTH`、`REAUTH_FAILED`、`LOCKED`、`REAUTH_PASSED`、`IN_APPROVAL`、`APPROVED`、`REJECTED`、`WITHDRAWN`、`ABANDONED`、`EXECUTED` |
| operation_type | text | 否 | 无 | 六类高危操作，取值集同 reauth_challenges |
| subject_object_type、subject_object_id | text、uuid | 否 | 无 | 待签对象定位 |
| subject_digest | bytea | 否 | 无 | 待签内容 SHA-256 摘要 |
| reauth_challenge_id | uuid | 是 | 无 | 关联复核挑战 |
| approval_chain_id | uuid | 否 | 无 | 所用审批链 |
| approval_instance_ref | uuid | 是 | 无 | 流程引擎实例的逻辑引用，跨平台组件不建外键 |
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
