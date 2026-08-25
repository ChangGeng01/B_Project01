> **F-57 状态：`HISTORICAL_DO_NOT_EXECUTE`。** 固定 RoleCode/岗位不是当前授权边界；仅复用 RLS、字段、SoD 和认证细节。

## 阶段 4：身份、认证与权限

> **F-50 增量。** 已过账凭证的纠错授权分成发票冲销、资金单据冲正、总账更正凭证三类业务入口；高风险、重新认证、审批与职责分离沿用既有门禁，但不得提供自由分录或绕过来源单据的通用入口。号码重复信息仍按可见性裁剪。

本阶段交付平台内核的身份与访问控制层，覆盖规格第 12.1 章身份与认证、第 12.2 章授权、第 7.7 章安全上下文建立与法人越权测试集、PRD 第 10.1 节至第 10.3 节。本阶段不实现流程引擎本体、不实现配置发布通道本体、不实现审计哈希链本体、不实现通知投递本体。按裁定通则第四条，阶段顺序固定为 1 → 2 → 3a → 4 → 3b-1 → T0 → 5 → 9a → 8 → 6 → 7 → 10 → 11 → 9b → 14，阶段 3b-2 不在这条链上，其各项按阶段 3 计划第 3.0 节判定四的下游拉动点排在 T0 之后；这四项本体均由阶段 3b 交付，落在本阶段之后，其中 T0 所需的部分属 3b-1 批。空实现通则已废止：本阶段不在 apps/core-server/src/wiring/ 与 apps/job-worker/src/wiring/ 两个目录下的任何文件中注入以 Noop、Stub、Fake、Dummy 四类前缀命名的替身，两个目录中不出现任何替身类型，装配期缺实现即拒绝启动；该断言由阶段 1 随 xtask 交付的 archcheck 规则 unwired-absent 承担，出现即构建失败。依赖这四项本体的功能点按同批交付处置，与阶段 3b 一并实现、一并验收，本阶段不为它们保留任何顺延登记。同批清单只有五项：一是高风险请求进入 IN_APPROVAL 之后的全部迁移与 submit、approve、reject、withdraw 四个端点；二是 authz-config-versions 的 stage-for-release 与 activate 两个端点及其配置包导出与差异视图；三是本阶段全部写事务中的审计事件写入，理由是 platform_audit.audit_events 本身由阶段 3b 建立；四是应急账号启用的站内通知投递；五是第 8.2 节第 16 项对模块许可状态的读取。清单之外的部分在本阶段内自足，不依赖阶段 3b 的任何交付物。ConfigItemApplier 端口按 A-19 由阶段 3a 交付，本阶段在该端口上实现三个 AUTHZ 类 applier，见第 4.8 节。全部接缝的归属按裁定表逐条写死，本阶段不再登记 needs。

本计划遵守共享技术基线。凡基线已定死的取值直接引用不再重述；凡基线未覆盖而本阶段必须取值的，一律在第 12 节“本阶段新增决定与已批准偏离项”中显式登记，并给出回写基线的位置。PRD 附录乙原待决项均采用 F-51/F-52 冻结值，不再保留实现选择。
本阶段与贯通线 T0 的关系。T0 是一条不新增任何范围的最薄贯通线，插在阶段 3b-1 与阶段 5 之间，即阶段 4 与 3b-1 两段都结束之后、阶段 5 全量开工之前，从阶段 5、6、9a、10、11 各取最小切片，判据是一条合同从建单走到管理层看到一个数。本阶段整体排在 T0 之前，因此不从 T0 取切片，而是为 T0 提供它唯一需要的身份底座。本阶段内部的工作次序据此分两批，阶段范围归属不变，表与端点的阶段归属也不变，退出条件仍按第 9 节在本阶段结束时一次判定。

第一批是 T0 底座，15 张表加四条链路，在 T0 开跑之前完成。platform_core 五张：user_accounts、user_credentials、user_devices、sessions、reauth_challenges。platform_authz 十张：permission_items、object_scope_bindings、roles、role_permission_grants、user_legal_entity_grants、user_role_grants、approval_chains、approval_chain_nodes、high_risk_requests、authz_config_versions。四条链路是口令登录与设备登记、安全上下文建立与 app.legal_entity_id 写入、授权判定第一阶段与第二阶段、ContractEffective 与 InvoiceIssue 两类高风险操作的重新认证与供 T0 使用的单节点审批链定义及其静态校验。InvoiceIssue 一类与 ContractEffective 同批落在第一批，理由是授权清单第十条把 invoice.sales_invoices 的一张销项发票定死为 T0 内的切片，而阶段 10 的开票端点必带 X-Reauth-Token 且销项发票的 reauth_ref 与 approval_ref 两列非空，该类重新认证与其审批链不进第一批，T0 在开票一步即停。T0 用到的身份数据只有一个法人、一个操作员账号、一个设备、一个业务角色与四条单节点审批链，四条的 scenario 依次为合同生效、发票申请单、销项发票开具与资金账户建档，条数与阶段 10 第 0 节的 T0 切片清单对齐，由 ep-datagen 的 T0 最小样本生成，不用默认 scale 数据集，不要求分支覆盖，不要求四端，只要求桌面端。

第二批是加厚，在第一批的底座上补齐，与第一批同在 T0 开跑之前完成，共 9 张表与其余功能。本阶段整体是 T0 的前置，两批都排在 T0 之前，第二批不跨到 T0 之后。platform_core 四张：user_password_history、login_attempts、account_lockouts、breakglass_activations。platform_authz 五张：access_policies、field_permissions、user_org_assignments、user_scope_grants、sod_rules。功能侧包括多因子与 WebAuthn 与 X509 三类认证方式、其余四类业务高风险操作、职责分离四类规则、记录级与字段级与密级判定、受控应急本地账号、账号生命周期四操作、门户端点，以及第 8.3 节的 32 组完整矩阵与第 8.5 节的全部性能项。共享 `HighRiskOperation` 同时预留并实现第七值 `DataMigration` 的挑战签发、核销与移动端拒绝；该值不进入本阶段通用 `high_risk_requests` 状态机，其版本绑定审批与执行只由阶段 14 的专用流程和证据图承载。

---

### 1. 交付物清单

本阶段结束时，下列东西可运行、可用命令验证。

1. 两个新平台 crate：ep-platform-identity 与 ep-platform-authz，随 core-server 与 job-worker 编译进二进制并在 wiring 中装配完成。
2. 一条可跑通的登录链路：桌面端提交登录名与口令，经二次因子校验后取得不透明会话令牌，随后携带 Authorization、X-Legal-Entity-Id、X-Device-Id、X-Client 四个头访问任一受保护端点，服务端建立安全上下文并把 app.legal_entity_id 写入数据库会话变量。
3. 一条可跑通的授权链路：同一请求内完成法人、对象、记录、字段与密级五阶段判定，命中拒绝时返回基线第 5.5 节规定的封套与错误码，且对不可见记录返回 404 而非 403。
4. 一个字段级受控只读视图端点，对已注册对象类型返回按角色字段权限与密级裁剪后的投影，无权字段不出现在响应体的键集合中。该端点是规格附录 A.1 常规交互清单中“字段级受控只读视图加载”这一度量项的被测对象。
5. 七类高风险操作的重新认证底座：六类业务高风险操作继续使用通用请求单据与审批网关；第七类运维高风险 `DATA_MIGRATION` 只复用挑战签发、待签内容摘要服务端重算、核销与 X-Reauth-Token 单次消费，随后进入阶段 14 的专用版本绑定审批与证据图，不建立第二张通用高风险请求单。审批链定义的静态合法性校验与运行期审批授权判定保持失败关闭。
6. 职责分离运行期与配置期双重执行：五类管理员职责互斥、申请人不可自审、审批链不可越权跳过，三者在配置保存时拒绝、在运行期再次判定。
7. 受控应急本地账号的申请、审批、限时启用、允许操作集合裁剪、到期自动失效与使用后凭据轮换。
8. 按 A-19 实现三个 AUTHZ 类 ConfigItemApplier：AuthzRoleApplier、AuthzPolicyApplier、AuthzFieldGrantApplier，三者位于 ep-platform-authz，实现阶段 3a 在 `crates/platform/release/src/port/config_item.rs` 交付的端口，注册到 ConfigItemApplierRegistry。配置包的导出、差异视图与经发布通道审批签名后的生效切换属开篇同批清单第二项，随阶段 3b 一并交付，本阶段不提供 bundle provider，也不提供任何绕过发布通道的生效开关；本阶段运行期唯一的生效版本由第 3.5 节 27 号种子迁移直接写入 authz_config_versions 并取 EFFECTIVE。
9. tests/rls_matrix 的第三段。按 C-05，CI 目标名与 assert_read、assert_write、assert_update、assert_delete、assert_aggregate、assert_sort、assert_report_projection、assert_error_leak 八个断言函数骨架由阶段 1 在 `testkit/src/rls_matrix.rs` 提供，assert_replication_role_containment 与 assert_recon_context_borrow 两个函数由阶段 2 追加，本阶段交付 matrix_32.rs 的 32 组完整矩阵与发布门禁项 RG-RLS-MATRIX-GREEN 的判定，不重复实现上述十个同名函数。该目标可单独执行并输出结构化报告。
10. 24 张表及 5 个回填/登记文件组成的 29 个既有迁移、6 个不回改历史 SQL 的追补迁移及其回退说明，可在空库上离线执行到最新版本并通过启动自检 rls-enabled-and-forced 与 runtime-role-privileges-bounded 两项。
11. 三份文档增量：docs/error-codes.md 新增本阶段错误码，其中 PLATFORM.IDEMPOTENCY.KEY_REQUIRED、PLATFORM.IDEMPOTENCY.PAYLOAD_MISMATCH、PLATFORM.CONCURRENCY.STALE_VERSION、PLATFORM.AUTHZ.NOT_FOUND_OR_DENIED、PLATFORM.AUTHZ.OBJECT_FORBIDDEN、PLATFORM.CAPACITY.CONCURRENCY_LIMIT、PLATFORM.DB.MIGRATION_WINDOW_CLOSED 七个按 C-24 由阶段 1 登记，本阶段只引用不重复登记；docs/event-catalog.md 新增本阶段五个事件；docs/data-dictionary 新增本阶段 24 张表条目。
12. ep-testkit 新增身份与授权夹具，ep-datagen 新增两档数据：T0 最小样本一次生成 1 个法人、1 个操作员账号、1 个设备、1 个业务角色与 4 条单节点审批链，四条的 scenario 依次为合同生效、发票申请单、销项发票开具与资金账户建档，供 T0 使用；默认 scale 数据集生成 2 个法人、50 名命名用户、角色与授权集合，供第二批的 32 组矩阵与第 8.5 节性能项使用。

---

### 2. crate 与进程归属

#### 2.1 新增 crate

| crate | 路径 | 层 | 归属进程 |
|---|---|---|---|
| ep-platform-identity | crates/platform/identity | platform | core-server、job-worker |
| ep-platform-authz | crates/platform/authz | platform | core-server、job-worker、integration-gateway |

ep-platform-identity 承载本地账号目录、凭据、多因子、设备登记、会话与令牌、重新认证挑战、账号锁定、受控应急本地账号。ep-platform-authz 承载 RBAC 与 ABAC 判定、记录级范围编译、字段级与密级过滤、职责分离、审批授权判定、审批链定义、高风险操作请求。

依赖方向：ep-platform-identity 依赖 ep-foundation 与 ep-platform-authz；ep-platform-authz 依赖 ep-foundation、ep-platform-tenancy 与 ep-platform-release，最后一项只用其 A-19 的 ConfigItemApplier 端口，该端口由阶段 3a 交付且不带表与用例，不构成环。两者均不依赖任何 domain、application 与 adapter，符合基线第 1.3 节；按裁定 F-04，对 KMS 能力的使用只经 `ep_foundation::port::kms::KmsBackend`，载体实例由 apps 在 apps/core-server/src/wiring/ 与 apps/job-worker/src/wiring/ 两个目录下注入，本阶段不新增任何指向 ep-adapter-kms 的依赖边。identity 依赖 authz 而不是反向，理由是安全上下文的装配需要读取授权集合，而授权判定不需要知道凭据与会话如何产生；这条方向一旦反过来就会成环。

#### 2.2 改动的既有 crate

| crate | 改动内容 |
|---|---|
| ep-adapter-db-pg | 新增 identity/ 与 authz/ 两个仓储实现目录，一个仓储只访问自己 schema；新增 RLS 策略模板生成器对本阶段 13 张带法人列表的调用 |
| ep-adapter-kms | 本阶段零改动，列出只为交代 KMS 能力的取用位：按裁定 F-04，TOTP 种子作为密级 40 的字段业务明文，经 `ep_foundation::port::kms::KmsBackend` 的 `wrap/unwrap` 直接形成/读取 EPC1；X.509 登录 trust bundle 则经阶段 2 独立 `SecretProvider/SecretUnsealer` 读取，不冒充 KmsBackend 业务信封。载体实例由 apps/core-server/src/wiring/ 与 apps/job-worker/src/wiring/ 两个目录下的装配注入，ep-platform-identity 与 ep-platform-authz 均不依赖本 crate |
| apps/core-server | 新增两层中间件（认证层、安全上下文与法人校验层）、新增本阶段全部路由、`apps/core-server/src/wiring/` 目录下的全部文件中装配 identity 与 authz 的具体实现 |
| apps/job-worker | 新增两个后台任务：过期会话与过期挑战清理、应急账号到期失效与轮换。授权快照重载不进 job-worker，见第 2.3 节 |
| apps/portal-gateway | 不新增数据库连接，只新增把门户 Cookie 换为核心服务会话令牌的转发逻辑，其呈现层由门户阶段承担 |
| ep-testkit | 新增 UserFixture、RoleFixture、GrantFixture、ReauthFixture、HighRiskRequestBuilder |
| ep-datagen | 新增用户、角色、授权、设备四类数据的生成 |

#### 2.3 进程内的运行形态

授权判定不落在请求路径上的数据库查询里。ep-platform-authz 在 core-server 内维护一个不可变的 AuthzSnapshot，用 arc_swap::ArcSwap 持有，每次配置版本切换构造新快照整体替换，不做原地修改，符合不可变数据的编码纪律。快照按法人分片，2 个法人各一份。快照重载只有一条路径：core-server 按 EP__AUTHZ__SNAPSHOT__POLL_INTERVAL_MS 轮询 authz_config_versions 的 EFFECTIVE 版本号，发现变更即整体换上新快照。事件驱动重载与其经 job-worker 消费再由进程间接口通知的整条链删除，两条路径取其一的选择项一并删除，理由是单机单副本下持有快照的进程只有 core-server 一个，用一次跨进程投递去刷新自己进程内的缓存只增加失败面，并连带引出死信与重投一整套处置。platform.authz_policy.published.v1 事件仍在配置版本生效事务内发出，但它的唯一消费方是规格第 7.9 章的派生存储重建，不再承担快照重载。

用户维度的授权集合（角色授予、组织归属、范围授予）不进快照，理由是它随人事变动逐日变化且基数小；它在每次会话建立时读取一次并冻结在 SecurityContext 中，会话有效期内不重读。调岗改权与离职停用通过撤销该用户全部会话使其立即生效，这是 PRD 第 10.2.3 节“停用即时生效”与“权限按生效日期切换”的实现方式。

---

### 3. 数据库变更

#### 3.1 schema 归属

本阶段不新增 schema。身份主体域的表建在 platform_core，授权域的表建在 platform_authz。这一划分是本阶段新增决定，理由见第 12 节。

#### 3.2 platform_core 的身份主体表（9 张，不带 legal_entity_id）

这 9 张表不带 legal_entity_id 列、不建行级策略，按第 12.2 节偏离一改写后的正向登记制逐表登记准入判据与隔离承接点，登记内容见该节，不再作为第五类例外申报。它们的法人可见性由 platform_authz.user_legal_entity_grants 这张受策略约束的表承担：任何列出用户的查询一律与该表内联，因此在法人 A 的上下文下只能看到被授权给 A 的用户。

表 3-1 platform_core.user_accounts（档案类）

| 列 | 类型 | 约束 | 说明 |
|---|---|---|---|
| id | uuid | pk_user_accounts | 应用侧 UUIDv7 |
| account_kind | text | not null，ck_user_accounts_account_kind in ('EMPLOYEE','PORTAL','BREAKGLASS','SYSTEM') | 账号种类 |
| login_name | text | not null，ux_user_accounts_login_name，ck 长度 <= 64 | 全局唯一登录名 |
| employee_no | text | null，ux_user_accounts_employee_no，ck 长度 <= 64 | 工号，门户与系统账号为空 |
| display_name | text | not null，ck 长度 <= 200 | 姓名 |
| home_legal_entity_id | uuid | not null | 归属法人，只用于审计事件分段与默认法人，不参与访问判定 |
| clearance_level | smallint | not null default 20，ck in (10,20,30,40) | 用户密级许可 |
| status | text | not null，ck in ('UNACTIVATED','ACTIVE','LOCKED','SUSPENDED','DEACTIVATED') | 账号状态机 |
| is_mfa_required | boolean | not null default false | 管理员与高风险角色置真 |
| activated_on | date | null | 入职开通生效日 |
| deactivated_at | timestamptz | null | 停用时刻 |
| last_login_at | timestamptz | null | 最近登录 |
| security_level | smallint | not null default 30 | 账号对象自身密级 |
| data_scope_tags | text[] | not null default '{}' | |
| row_version | bigint | not null default 1 | |
| created_at / created_by / updated_at / updated_by | timestamptz / uuid | not null | 公共列 |

索引：pk_user_accounts、ux_user_accounts_login_name、ux_user_accounts_employee_no、ix_user_accounts_status_created_at。本表无 legal_entity_id，因此基线第 3.10 节的 ix_<table>_legal_entity_id_created_at 基线索引改为 ix_user_accounts_status_created_at，这是偏离的连带项。账号只承担全局认证主体，不拥有供应商业务关系；门户用户与供应商的唯一绑定落在 `portal.supplier_portal_users`。历史建表迁移中曾出现的 `supplier_ref_id` 由第 3.5 节追补迁移删除，不得建立第二份映射或兼容性回填。

表 3-2 platform_core.user_credentials

| 列 | 类型 | 约束 |
|---|---|---|
| id | uuid | pk_user_credentials |
| user_id | uuid | not null，fk_user_credentials_user_accounts on delete restrict |
| credential_kind | text | not null，ck in ('PASSWORD','TOTP','WEBAUTHN_PLATFORM','WEBAUTHN_ROAMING','X509_CERT') |
| verifier | text | null，PASSWORD 存 Argon2id 的 PHC 串；X509_CERT 只存 `cert-sha256:<64 lowerhex>`，摘要输入是 leaf exact DER certificate bytes，禁止 DN、序列号或平台显示指纹 |
| public_key | bytea | null，WebAuthn 的 COSE 公钥 |
| credential_handle | bytea | null，WebAuthn credential id 或 X509 leaf SubjectKeyIdentifier exact raw bytes；X509 取 1..64 bytes 且与证书 extension、CMS sid 逐字一致 |
| secret_enc | bytea | null，TOTP 种子的 EPC1 密文；绝不存明文或外部 object ref |
| secret_key_ref | text | null，TOTP EPC1 的 canonical data-key id/version 冗余投影，只用于完整性核对，不参与选钥 |
| last_used_counter | bigint | null，仅 TOTP 使用；首次成功前为空，之后严格单调增加以拒绝同一 time-step 重放 |
| sign_count | bigint | not null default 0，WebAuthn 计数器 |
| status | text | not null，ck in ('ACTIVE','SUSPENDED','REVOKED','EXPIRED') |
| activated_at / expires_at / last_used_at / revoked_at | timestamptz | 后三个可空 |
| security_level | smallint | not null default 40 |
| data_scope_tags、row_version、created_*、updated_* | | 公共列 |

索引：pk_user_credentials、ix_user_credentials_user_id_credential_kind、ux_user_credentials_credential_handle。约束 `ck_user_credentials_material` 是 exact one-of：PASSWORD 只允许 verifier 非空；X509_CERT 只允许 verifier 与 credential_handle 同时非空；两类 WEBAUTHN 只允许 public_key 与 credential_handle 非空；TOTP 只允许 `secret_enc` 与 `secret_key_ref` 同时非空；其余材料列在各分支都必须为空。X509 verifier 由 strict parser 强制上述 wire，不能由自由字符串构造。TOTP credential id 必须在生成 seed 前预分配；唯一 AAD 为 `Aad::for_field(legal_entity_id,"platform_core.user_credentials.totp_secret",credential_id,SecurityLevel::L40)`，purpose 固定 `KeyPurpose::Field(L40)`，写入与验证都直接调用 `KmsBackend::wrap/unwrap`，不得把 `data_keys.wrapped_key` 或明文 DEK带到 identity crate。

表 3-3 platform_core.user_password_history（仅追加）：id、user_id、verifier text not null、created_at、created_by。索引 ix_user_password_history_user_id_created_at。不带 row_version、updated_at、updated_by，也不带 reverses_id。

表 3-4 platform_core.user_devices：id、user_id、device_id text not null、client text not null（取值 win/mac/ios/android/portal/ops）、public_key bytea null、attestation_ref text null、restricted_legal_entity_id uuid null、status text not null（PENDING、ACTIVE、REVOKED）、registered_at、revoked_at、last_seen_at、公共列。索引 ux_user_devices_device_id、ix_user_devices_user_id_status。restricted_legal_entity_id 非空表示该设备只能用于该法人，安全上下文建立时取用户授权集合与该限定的交集，对应规格第 7.7 章“该用户与设备的授权法人集合”。

表 3-5 platform_core.sessions：id、user_id、user_device_row_id uuid not null（引用 user_devices.id）、token_hash bytea not null（SHA-256 of 令牌原文，令牌原文不入库）、active_legal_entity_id uuid not null、client text not null、issued_at、expires_at、idle_expires_at、last_seen_at、revoked_at、revoke_reason text null、is_breakglass boolean not null default false、公共列。索引 ux_sessions_token_hash、ix_sessions_user_id_expires_at、ix_sessions_last_seen_at。

表 3-6 platform_core.reauth_challenges：该历史表名保留，但同时承载登录 MFA 与高风险重新认证两类一次性挑战。列为 id、challenge_kind text not null（`SIGN_IN_MFA`、`HIGH_RISK_REAUTH`）、user_id uuid not null、session_id uuid null、user_device_row_id uuid null、default_legal_entity_id uuid null、operation_type text null（七值闭集：`CONTRACT_EFFECTIVE`、`PAYMENT`、`INVOICE_ISSUE`、`LEDGER_POSTING`、`PERIOD_CLOSE`、`SENSITIVE_EXPORT`、`DATA_MIGRATION`）、subject_digest bytea not null、subject_summary jsonb not null（规范化摘要，敏感字段已掩码）、nonce bytea not null、credential_kind_used text null、status text not null（ISSUED、VERIFIED、CONSUMED、FAILED、EXPIRED、ABANDONED）、token_hash bytea not null、issued_at、expires_at、verified_at、consumed_at、failure_count int not null default 0、公共列。`SIGN_IN_MFA` 强制 session_id 与 operation_type 为空、user_device_row_id 与 default_legal_entity_id 非空，subject_digest 绑定 user、device、client 与 default legal entity，成功验证从 ISSUED 直接到 CONSUMED并创建会话；`HIGH_RISK_REAUTH` 强制 session_id 与 operation_type 非空、default_legal_entity_id 为空，按 ISSUED→VERIFIED→CONSUMED。`token_hash` 是客户端收到的 32 字节随机不透明挑战令牌的 SHA-256，原文不入库。索引 pk、ux_reauth_challenges_token_hash、ix_reauth_challenges_user_id_status_expires_at；上述条件由 NULL-safe CHECK 强制。`DATA_MIGRATION` 的 subject 形状、挑战消费时点与专用审批证据闭图只取阶段 14 第 3.1.2 节，不得退化为本阶段通用摘要。

表 3-7 platform_core.login_attempts（仅追加）：id、user_id uuid null、login_name_hash bytea not null、outcome text not null（SUCCESS、CREDENTIAL_INVALID、ACCOUNT_LOCKED、ACCOUNT_INACTIVE、MFA_REQUIRED、MFA_INVALID、DEVICE_UNREGISTERED、RATE_LIMITED）、client text、source_addr text、occurred_at timestamptz not null、created_by。索引 ix_login_attempts_occurred_at、ix_login_attempts_user_id_occurred_at。本表不带 row_version、updated_at、updated_by，也不带 reverses_id，理由是登录尝试没有冲销或更正语义。登录名以哈希存储，理由是失败尝试中的登录名可能是攻击者构造的任意串，明文入库会把一张运行数据表变成半个外部输入落点。`RATE_LIMITED` 只表示认证前端点的登录名/来源地址速率限制，不得用于活跃用户超过 20 人。

表 3-8 platform_core.account_lockouts：user_id uuid pk（一人一行）、failure_count int not null default 0、window_started_at timestamptz、locked_until timestamptz null、last_failure_at timestamptz、row_version、created_*、updated_*。索引 pk_account_lockouts、ix_account_lockouts_locked_until。

表 3-9 platform_core.breakglass_activations（单据类）：id、doc_no text not null（类型码 BGA）、status text not null（DRAFT、PENDING_APPROVAL、APPROVED、ACTIVE、EXPIRED、CLOSED、REJECTED）、user_id、requested_by、approved_by uuid null、reason text not null（长度 <= 2000）、approval_ref text null、allowed_action_set text[] not null、activated_at、expires_at、closed_at、rotated_at、rotation_result text null、公共列。索引 pk、ux_breakglass_activations_doc_no、ix_breakglass_activations_status_expires_at。allowed_action_set 的取值域固定为规格第 12.1 章列出的三类：UNLOCK_OR_RESET_ADMIN、RESTORE_CONTROLLED_CONFIG_RELEASE、TRIGGER_BACKUP_OR_RESTORE，由 CHECK 约束限定，不接受其他取值。doc_no 的唯一约束不带法人（本表无法人列），是偏离的连带项。

#### 3.3 platform_authz 的授权表（15 张）

其中 object_scope_bindings 与 permission_items 不带法人列、不建策略，按第 12.2 节偏离一的正向登记制逐表登记准入判据与隔离承接点；本阶段不再援引全局配置字典这一类名，该类名连同四类封闭枚举一并作废，理由是它没有定义、容量无限，是各阶段自我归类的唯一入口，删枚举而留类名等于问题原样保留。其余 13 张全部带 legal_entity_id 并按基线第 3.8 节的统一模板建策略，策略名 rls_<table>_le。敏感字段登记表按 C-06 不在本清单内，理由与引用方式见表 3-15 之后的一段说明。

本 schema 内凡带 `legal_entity_id` 的 `user_id`、`granted_by`、`requested_by`、`approved_by`、`published_by`、`initiator_user_id` 等业务用户列，统一以 `(legal_entity_id,<user-column>)` 复合外键指向 `platform_authz.user_legal_entity_grants(legal_entity_id,user_id) ON DELETE RESTRICT`，不得直接指向无法人列的 `platform_core.user_accounts`。系统自动写入也不豁免：每个法人必须存在 `SYSTEM_PRINCIPAL_ID` 的有效授权行。`reauth_challenge_id` 等全局身份证据列保留对无法人目标的单列真实外键，并由持锁写用例验证证据主体具有当前法人授权；这只是外键形状例外，不是逻辑引用例外。

表 3-10 platform_authz.permission_items（不带法人列，按第 12.2 节登记）：code text pk（形如 sales.sales_order）、module_code text not null（15 个模块码或 platform）、function_point text not null、allowed_actions text[] not null（子集取自 VIEW、CREATE、UPDATE、SUBMIT、APPROVE、EXPORT 六个动作）、object_type text not null、description text、created_*、updated_*、row_version。索引 pk_permission_items、ix_permission_items_module_code。六个动作照抄 PRD 第 10.2.2 节“至少含查看、新建、修改、提交、审批、导出”，本阶段取值为恰好这六个，不多不少。第 11 号建表迁移同批预置 F-51 U-H-17 的全局权限项：`code='ledger.backdate'`、`module_code='ledger'`、`function_point='posting_backdate'`、`allowed_actions=['UPDATE']`、`object_type='ledger.voucher'`；应用常量名固定为 `LEDGER_BACKDATE`，不增加第七种动作。另有约束 ck_permission_items_forbidden_codes，拒绝写入以 platform.legal_entity_isolation 与 platform.direct_db_access 两个前缀开头的 code，即关闭或修改法人隔离机制与事务业务库直连两类权限项写不进这张表；该约束替代原先的同名启动自检项，见第 7 节。

表 3-11 platform_authz.object_scope_bindings（不带法人列，按第 12.2 节登记）：object_type text pk、schema_name text not null、table_name text not null、owner_user_col text null、owning_dept_col text null、project_col text null、customer_col text null、security_level_col text not null default 'security_level'、created_*、updated_*、row_version。这张表是记录级判定的落点：各业务模块在其阶段的 wiring 中登记自己对象的范围锚列，本阶段只登记 platform 自身的三个对象类型（platform.user_accounts、platform.roles、platform.high_risk_requests）并提供登记接口，业务对象的登记在其所属阶段完成。没有登记的对象类型在记录级判定阶段一律拒绝，不默认放行。

`permission_items.object_type` **不**对 `object_scope_bindings.object_type` 建物理外键，这是经裁定的生命周期例外而非遗漏：权限项目录可能在下游模块表与范围锚列尚未安装时先随签名模块包登记，例如本阶段预置的 `ledger.backdate`；若建该外键，要么引用尚不存在的业务表，要么被迫写一条不可运行的伪范围绑定。替代的强制闸门只有一套：任何 `authz_config_versions` 从 DRAFT 切到 EFFECTIVE、任何模块从已安装切到启用前，均对该版本引用到的每个 permission item 做闭包校验，要求其 object_type 在 `object_scope_bindings` 恰有一行，且 binding 指向的 schema/table/锚列真实存在；缺失返回 `PLATFORM.AUTHZ.SCOPE_BINDING_MISSING` 并整事务回滚。运行期阶段三继续默认拒绝，不能因配置历史或模块停用而放行。对应正例为 ledger 模块迁移完成并登记 binding 后可启用，负例为缺 binding、伪表、伪列三种均不能发布配置或启用模块。

表 3-12 platform_authz.roles（档案类）：id、legal_entity_id、code、name、duty_class text null（SYSTEM、DATA、SECURITY、AUDIT、KEY、CONFIG，业务角色为空）、is_portal_role boolean not null default false、lifecycle_state text not null（DRAFT、PENDING_RELEASE、EFFECTIVE、SUPERSEDED、RETIRED）、retired_at timestamptz null、is_active、deactivated_at、公共列。索引 pk、ux_roles_legal_entity_id_code、ix_roles_legal_entity_id_created_at。角色一律按法人建立，不做跨法人的全局角色，理由是全局角色会立刻在这张表上制造一处需要绕过行级策略的读路径，而基线第 3.8 节不允许任何绕过。2 个法人下的复制成本可接受。

表 3-13 platform_authz.role_permission_grants：id、legal_entity_id、role_id、permission_item_code text not null、action text not null（六动作之一）、公共列。真实外键固定为 `(legal_entity_id,role_id) -> roles(legal_entity_id,id) ON DELETE RESTRICT` 与 `permission_item_code -> permission_items(code) ON DELETE RESTRICT`；后者是全局权限项的单列引用，不得以应用校验或快照加载代替。索引 pk、ux_role_permission_grants_legal_entity_id_role_id_permission_item_code_action、ix_role_permission_grants_legal_entity_id_created_at。

表 3-14 platform_authz.access_policies：id、legal_entity_id、role_id uuid null（空表示适用全部角色）、object_type text not null、effect text not null（ALLOW、DENY）、priority int not null default 100、condition jsonb not null、lifecycle_state、公共列。索引 pk、ix_access_policies_legal_entity_id_object_type_effect、ix_access_policies_legal_entity_id_created_at。condition 是受限的声明式结构，不是表达式语言：只允许对 department、position、project、customer、security_level、data_scope_tag 六个属性做 in、not_in、lte、gte、has_tag 五种断言的合取，由 serde 强类型反序列化，不做字符串求值。理由是把策略做成表达式语言等于在权限层引入一个解释器，其求值行为将成为越权测试无法穷举的面。

表 3-15 platform_authz.field_permissions：id、legal_entity_id、role_id、object_type、field_name text not null、visibility text not null（HIDDEN、MASKED、READ、WRITE）、mask_style text null（FULL、KEEP_LAST_4、KEEP_DOMAIN）、公共列。索引 pk、ux_field_permissions_legal_entity_id_role_id_object_type_field_name。

敏感字段登记表不在本阶段建立。按 C-06，全系统唯一的登记表是 platform_core.sensitive_field_registry，由阶段 2 交付，其业务列集与唯一约束 ux_sensitive_field_registry_schema_table_column 已由裁定 C-06 冻结为十一列，本阶段不复述该列集，也不声明该表另有附加列。本阶段第 4.2 节阶段四的字段密级与默认掩码风格一律从该表读取，登记行由各模块阶段以 backfill 迁移写入，本阶段不建表也不写入任何行。该表不设 approved_by 与 approved_at 两列，规格第 12.2 章“经产品负责人批准的敏感字段清单”的批准留痕由该表的 release_ref 列承载，经迁移登记时取 `MIGRATION:<迁移版本号>`，经端点登记时取 `ENDPOINT:<审批记录号>`；某字段的导出是否触发重新认证不由表列承载，按第 12.3 节 U-B-18 的判定函数计算。

表 3-16 platform_authz.user_legal_entity_grants：id、legal_entity_id、user_id、granted_from date not null、granted_to date null、granted_by uuid not null、公共列。索引 pk、ux_user_legal_entity_grants_legal_entity_id_user_id、ix_user_legal_entity_grants_legal_entity_id_created_at。这是全系统唯一决定“某用户能不能进某法人”的表，它自身受策略约束，因此法人 A 的管理员无法看到也无法写入法人 B 的授权行。

表 3-17 platform_authz.user_role_grants：id、legal_entity_id、user_id、role_id、effective_from date not null、effective_to date null、granted_by、公共列。索引 pk、ux_user_role_grants_legal_entity_id_user_id_role_id_effective_from、ix_user_role_grants_legal_entity_id_user_id。

表 3-18 platform_authz.user_org_assignments：id、legal_entity_id、user_id、department_id uuid not null、position_id uuid not null、effective_from date not null、effective_to date null、公共列。索引 pk、ix_user_org_assignments_legal_entity_id_user_id、ix_user_org_assignments_legal_entity_id_department_id。department_id 与 position_id 的外键目标按 A-04 写死为 platform_core.departments(id) 与 platform_core.positions(id)，两张表由阶段 2 交付，本阶段迁移在其之后执行，外键在 V20261012104500 中建立。

表 3-19 platform_authz.user_scope_grants：id、legal_entity_id、user_id、scope_kind text not null（PROJECT、CUSTOMER、RECORD）、object_type text null（RECORD 时必填）、scope_ref_id uuid not null、can_reshare boolean not null default false、granted_by、effective_from date not null、effective_to date null、公共列。索引 pk、ix_user_scope_grants_legal_entity_id_user_id_scope_kind、ux_user_scope_grants_legal_entity_id_user_id_scope_kind_scope_ref_id。`can_reshare` 固定为 false 且带 CHECK 约束限定为 false；F-51 U-B-07 已冻结共享不可再转授，首版不存在放开该约束的另一实现分支。

表 3-20 platform_authz.sod_rules：id、legal_entity_id、rule_code text not null、rule_kind text not null（DUTY_EXCLUSION、ROLE_EXCLUSION、SELF_APPROVAL、CHAIN_SKIP）、left_ref text null、right_ref text null、enforcement text not null default 'BLOCK'、message_code text not null、公共列。索引 pk、ux_sod_rules_legal_entity_id_rule_code。message_code 指向 docs/error-codes.md 中的错误码，用于满足 PRD 第 10.2.2 节“异常提示需指出被拒绝的具体规则名称”。

表 3-21 platform_authz.approval_chains（档案类）：id、legal_entity_id、code、name、scenario text not null、version_no int not null default 1、lifecycle_state、is_active、deactivated_at、`active_scenario_slot text generated always as (case when is_active and lifecycle_state='EFFECTIVE' then scenario else null end) stored`、公共列。`scenario` 必须是第 4.1 节 `ApprovalScenarioCode` 的三十七个取值之一，不接受业务模块在运行期登记新字符串。索引与约束为 pk、ux_approval_chains_legal_entity_id_code_version_no、`ux_approval_chains_legal_entity_id_scenario_version_no`（`legal_entity_id,scenario,version_no`）及 `ux_approval_chains_legal_entity_id_active_scenario_slot`（`legal_entity_id,active_scenario_slot`）；最后一项利用 NULL 可重复而非 NULL 不可重复的唯一约束，数据库层保证每个法人、每个场景至多一个 `EFFECTIVE + is_active=true` 版本。

表 3-22 platform_authz.approval_chain_nodes：id、legal_entity_id、approval_chain_id、node_no int not null、approver_kind text not null（ROLE、POSITION、DEPT_MANAGER）、approver_ref uuid null、role_code text null、quorum int not null default 1、timeout_hours int null、公共列。索引 pk、ux_approval_chain_nodes_legal_entity_id_approval_chain_id_node_no。表上没有 allow_skip 一类列，理由见第 4.5 节：越权跳过不是被校验拒绝的配置，而是根本没有承载它的字段。

表 3-23 platform_authz.high_risk_requests（单据类）：id、legal_entity_id、doc_no（类型码 HRR）、status text not null（十一态见第 4.4 节）、operation_type text not null（只允许六类业务高风险值，不含 `DATA_MIGRATION`）、subject_object_type text not null、subject_object_id uuid not null、subject_digest bytea not null、reauth_challenge_id uuid null、approval_chain_id uuid not null、approval_ref uuid null（审批实例引用，属于基线第 3.3 节具名白名单）、initiator_user_id、initiator_device_id text、submitted_at、decided_at、executed_at、execution_ref uuid null、reject_reason text null、公共列。`reauth_challenge_id` 对 `platform_core.reauth_challenges(id)` 建单列真实外键，`approval_chain_id` 对 `platform_authz.approval_chains(legal_entity_id,id)`、`initiator_user_id` 对 `platform_authz.user_legal_entity_grants(legal_entity_id,user_id)` 建复合真实外键，均 `ON DELETE RESTRICT`；`subject_object_type/id` 与 `execution_ref` 属已登记的封闭多态引用，按基线白名单执行。索引 pk、ux_high_risk_requests_legal_entity_id_doc_no、ix_high_risk_requests_legal_entity_id_created_at、ix_high_risk_requests_legal_entity_id_status_operation_type。`DATA_MIGRATION` 只出现在共享枚举与 reauth_challenges，其批准事实落阶段 14 `platform_ops.data_migration_approval_evidences`，不得插入本表。

表 3-24 platform_authz.authz_config_versions：id、legal_entity_id、version_no bigint not null、state text not null（DRAFT、STAGED、EFFECTIVE、ROLLED_BACK）、release_bundle_ref uuid null、checksum bytea not null、published_by uuid null、published_at timestamptz null、公共列。索引 pk、ux_authz_config_versions_legal_entity_id_version_no、ix_authz_config_versions_legal_entity_id_state。

#### 3.4 行级策略

13 张带法人列的表逐张按基线第 3.8 节模板生成，不写手工变体。

```sql
alter table platform_authz.user_legal_entity_grants enable row level security;
alter table platform_authz.user_legal_entity_grants force row level security;
create policy rls_user_legal_entity_grants_le on platform_authz.user_legal_entity_grants
  using (legal_entity_id = nullif(current_setting('app.legal_entity_id', true), '')::uuid)
  with check (legal_entity_id = nullif(current_setting('app.legal_entity_id', true), '')::uuid);
```

其余 12 张同构，只换表名。platform_core 的 9 张身份主体表与 platform_authz 的 permission_items、object_scope_bindings 两张不建策略；这 11 张按基线第 3.8 节的正向登记制在 platform_core.unpoliced_table_registry 中逐表登记一行，admission_basis 前 9 张取 ISOLATION_OR_DEPLOYMENT_METADATA、后 2 张取 SAME_FOR_ALL_ENTITIES，登记行由第 3.5 节第 29 号回填迁移一次写入。

#### 3.5 迁移编号与顺序

db/migrations/platform_core/ 追加：

1. V20261012090000__platform_core_identity_user_accounts.sql
2. V20261012090500__platform_core_identity_user_credentials.sql
3. V20261012091000__platform_core_identity_user_password_history.sql
4. V20261012091500__platform_core_identity_user_devices.sql
5. V20261012092000__platform_core_identity_sessions.sql
6. V20261012092500__platform_core_identity_reauth_challenges.sql
7. V20261012093000__platform_core_identity_login_attempts.sql
8. V20261012093500__platform_core_identity_account_lockouts.sql
9. V20261012094000__platform_core_identity_breakglass_activations.sql
10. V20261012094500__platform_core_backfill_system_principal_account.sql

db/migrations/platform_authz/ 追加：

11. V20261012100000__platform_authz_permission_items.sql
12. V20261012100500__platform_authz_object_scope_bindings.sql
13. V20261012101000__platform_authz_roles.sql
14. V20261012101500__platform_authz_role_permission_grants.sql
15. V20261012102000__platform_authz_access_policies.sql
16. V20261012102500__platform_authz_field_permissions.sql
17. V20261012103500__platform_authz_user_legal_entity_grants.sql
18. V20261012104000__platform_authz_user_role_grants.sql
19. V20261012104500__platform_authz_user_org_assignments.sql
20. V20261012105000__platform_authz_user_scope_grants.sql
21. V20261012105500__platform_authz_sod_rules.sql
22. V20261012110000__platform_authz_approval_chains.sql
23. V20261012110500__platform_authz_approval_chain_nodes.sql
24. V20261012111000__platform_authz_high_risk_requests.sql
25. V20261012111500__platform_authz_authz_config_versions.sql
26. V20261012112000__platform_authz_backfill_permission_item_seed.sql
27. V20261012112500__platform_authz_backfill_admin_duty_roles.sql（同一文件内一并写入两个法人各一行 state 取 EFFECTIVE 的 authz_config_versions，checksum 按该文件写入的配置行现算；这是本阶段运行期唯一的生效版本来源，启动自检 authz-snapshot-loadable 据此可构造快照）
28. V20261012113000__platform_authz_backfill_default_sod_rules.sql

db/migrations/platform_core/ 再追加一个回填文件，其主要创建对象是 platform_core.unpoliced_table_registry 的登记行，版本号晚于本阶段全部建表迁移，故列在最后：

29. V20261012113500__platform_core_backfill_unpoliced_table_registry.sql（按基线第 3.8 节的正向登记制，向阶段 2 交付的 platform_core.unpoliced_table_registry 写入本阶段 11 张不带法人列的表各一行，schema、table、准入判据、隔离承接入口与 rls_matrix 用例标识五列按阶段 2 冻结的列集填写；准入判据一列，platform_core 的 9 张身份主体表取隔离机制自身的元数据一档，platform_authz 的 permission_items 与 object_scope_bindings 两张取行在本部署内对全部法人取值相同一档，取值名以阶段 2 冻结的枚举为准，逐表的隔离承接入口按第 12.2 节偏离一写明）

上述 29 个已存在迁移的文件名与校验和不可改。为使现行模型可从该历史形状安全升级，`docs/migration-catalog.md` 另冻结以下 6 个追补迁移；实施必须使用目录中的精确版本与文件名，不得回改旧 SQL：

30. `V20261012114000__platform_core_alter_reauth_challenges_dual_kind.sql`：把历史高风险专用表升级为表 3-6 的双用途形状，新增 `challenge_kind`、`user_device_row_id`、`default_legal_entity_id`，收紧 `token_hash not null`，把 `operation_type` CHECK 一次冻结为上述七值，并以 NULL-safe CHECK 固定两类挑战的互斥必填条件；升级前先完成可验证回填。第七值只让统一 reauth 底座可表达 `DATA_MIGRATION`，不把它加入 `high_risk_requests` 的六值 CHECK。
31. `V20261012114500__platform_core_add_identity_foreign_keys.sql`：补齐身份表内部真实外键与候选键，包括 password history、devices、sessions、reauth、login attempts、lockouts、breakglass 的用户引用，以及 session/reauth 的会话与设备归属约束；全部 `ON DELETE RESTRICT`。设备与会话等全局身份证据仍为单列真实外键，写用例另在锁内验证其用户与当前法人授权一致。
32. `V20261012115000__platform_core_drop_user_accounts_supplier_ref_id.sql`：删除历史 `user_accounts.supplier_ref_id`，不搬迁、不生成第二份供应商绑定；上线前置校验要求所有门户绑定已由权威 `portal.supplier_portal_users` 独立存在。
33. `V20261012115500__platform_authz_add_missing_foreign_keys.sql`：为 roles 与 approval_chains 补 `(legal_entity_id,id)` 候选键；补齐角色授权、访问策略、字段权限、用户角色授权、审批链节点、高风险请求与用户组织分配等缺失外键；其中必须具名建立 `fk_role_permission_grants_permission_item_code: role_permission_grants(permission_item_code) -> permission_items(code) ON DELETE RESTRICT`，并在加约束前以 anti-join 预检孤儿，非零即中止并列出 grant id/code，不静默删除。所有业务 `user_id` 指向 `user_legal_entity_grants(legal_entity_id,user_id)`。同一迁移把历史 `approval_instance_ref` 重命名为 `approval_ref`，并为每个现有法人幂等补齐 `SYSTEM_PRINCIPAL_ID` 的法人授权；新法人创建事务也必须同步建立该授权。`permission_items.object_type` 不在此文件建 FK，其经批准的生命周期理由与配置发布/模块启用替代闸门见表 3-11 后的段落。
34. `V20261012120000__platform_authz_alter_approval_scenario_constraints.sql`：按第 4.1 节的旧值映射表把全部现存 `scenario` 规范化为 `ApprovalScenarioCode`；发现无法映射的值即中止迁移并列出法人、链与原值，不允许静默删除或归入 OTHER。随后增加三十七值 CHECK、`(legal_entity_id,scenario,version_no)` 唯一约束、生成列 `active_scenario_slot` 与 `(legal_entity_id,active_scenario_slot)` 唯一约束；若历史数据在同一法人同一场景有多个活动版本，迁移必须失败并要求先经受控修复，禁止自行挑一个保留。
35. `V20261012120500__platform_authz_backfill_default_approval_chains.sql`：按第 4.5 节的 `ApprovalDefaultCatalog` 为每个现存法人逐场景幂等补齐缺失的默认链及节点；已有唯一活动链的场景保持不变，缺链场景以确定性 code 与 id 写入 version 1 并激活，重复运行零新增。到款链同样配置为 `FINANCE_MANAGER`，但 `EP__FINANCE__RECEIPT__REQUIRES_APPROVAL=false` 时业务入口不启动它。回退只删除能以确定性 seed 标识证明仍未被客户修改的行，不触碰客户自定义链。

迁移执行顺序由单一全局 Runner 按文件版本号全序排定，不存在任何模块顺序声明文件，本阶段只需保证自己每个文件的版本号晚于其全部被引用对象；二十四个目录按 C-01 由阶段 1 建为空目录，platform_core 与 platform_authz 均在其中。29 个既有文件先按历史版本执行，6 个追补文件再按迁移目录的全局版本执行；追补文件只用 `ALTER`、约束验证与必要的幂等数据修复，不重建既有表。`platform_authz` 的业务用户引用统一指向 `user_legal_entity_grants(legal_entity_id,user_id)`，部门与岗位引用继续指向阶段 2 的复合候选键；全局身份表引用采用本节声明的单列真实外键。所有目标在相应追补迁移执行前已存在，空库与历史升级路径都不存在引用后建对象。

每个迁移文件头部按基线第 3.9 节写 -- rollback: 段。10 号、26 至 28 号与 29 号是数据回填文件，slug 以 backfill_ 开头，其中 10 号与 26 至 28 号的回退说明为按 code 删除种子行，29 号的回退说明为按 schema 与 table 两列删除本阶段登记的 11 行。24 张建表迁移全部属于新增表，落在在线变更范围内，不需要停机窗口。RETIRED_VERSION_SLOT 号段作废，敏感字段登记表按 C-06 由阶段 2 在 platform_core 建立，本阶段不占用该号段。

第 10 号迁移写入 `00000000-0000-7000-8000-000000000001` 的系统主体账号行，account_kind 取 SYSTEM，login_name 取 system，status 取 ACTIVE，无凭据。该取值即 ep-foundation 的 SYSTEM_PRINCIPAL_ID 常量，按 A-02 由阶段 1 冻结。本阶段凡在种子迁移与系统上下文写 created_by 的一律引用该常量，设备标识引用 SYSTEM_DEVICE_ID，取值为 SYSTEM，不得自选其他值。

---

### 4. 领域模型与关键算法

本阶段不涉及任何账务处理。凡高风险操作执行后产生的分录一律按规格第 5.2 章事件-分录表由对应业务模块生成，本阶段只承担其前置的重新认证与审批放行，不参与借贷与取价。

#### 4.1 核心类型

> **系统上下文完整映射，取代本节后文的简写。** `SecurityContext::system(le, request, trace, purpose)` 逐字段固定为 `user_id=SYSTEM_PRINCIPAL_ID`、`account_kind=System`、`session_id=SYSTEM_SESSION_ID`、`legal_entity_id=le`、`device_id=SYSTEM_DEVICE_ID`、`client=Ops`、`clearance_level=10`、`roles=[]`、`duty_classes=[]`、`department_scope=Explicit([])`、`position_ids=[]`、`project_scope=[]`、`customer_scope=[]`、`record_shares=[]`、`data_scope_tags=[]`、`snapshot_version=0`、`is_breakglass=false`、`request_id=request`、`trace_id=trace`、`system_purpose=Some(purpose)`。`SYSTEM_SESSION_ID=00000000-0000-7000-8000-000000000002` 只是非空上下文哨兵，sessions 不建行且不可 reauth/续期；`SYSTEM_PRINCIPAL_ID` 必须有本阶段种子的 SYSTEM account 行。人类 AuthorizationService、会话与重认证流水线遇 System 一律拒绝；审计将 System account 映射为 `client='system'`，`system` 不进入 ClientKind 八值。任一默认值、哨兵会话查库/重认证、System 走人类授权或跨法人调用均是必测负例。

ep-foundation 侧（字段集合由阶段 1 按 A-03 冻结为下列 20 个字段，字段顺序即下列顺序，不得增删改名，本阶段只负责填充）：

```rust
pub struct SecurityContext {
    pub user_id: Id<UserAccount>,
    pub account_kind: AccountKind,
    pub session_id: Id<Session>,
    pub legal_entity_id: Id<LegalEntity>,
    pub device_id: DeviceId,
    pub client: ClientKind,
    pub clearance_level: SecurityLevel,
    pub roles: Arc<[RoleCode]>,
    pub duty_classes: Arc<[DutyClass]>,
    pub department_scope: DepartmentScope,
    pub position_ids: Arc<[Id<Position>]>,
    pub project_scope: Arc<[Id<Project>]>,
    pub customer_scope: Arc<[Id<Customer>]>,
    pub record_shares: Arc<[RecordShare]>,
    pub data_scope_tags: Arc<[DataScopeTag]>,
    pub snapshot_version: u64,
    pub is_breakglass: bool,
    pub request_id: RequestId,
    pub trace_id: TraceId,
    pub system_purpose: Option<SystemPurpose>,
}
```

SecurityContext 的构造入口只有 `SecurityContext::human(..)` 与 `SecurityContext::system(legal_entity_id, request_id, trace_id, purpose)` 两个：前者固定 `system_purpose=None`，后者按 A-02 用 SYSTEM_PRINCIPAL_ID 与 SYSTEM_DEVICE_ID 填 user_id 与 device_id、account_kind 取 System，并固定 `system_purpose=Some(purpose)`。SecurityContext 一经构造不再修改，任何“提权”都必须重新走一次会话建立，不提供任何 with_ 前缀的变换方法。配套枚举同在 ep-foundation 冻结：AccountKind 取 Human、System、Portal 三值，platform_core.user_accounts.account_kind 的四个取值按 EMPLOYEE 与 BREAKGLASS 映射为 Human、PORTAL 映射为 Portal、SYSTEM 映射为 System，映射函数落在 identity 仓储内；ClientKind 取 Win、Mac、Ios、Android、Portal、Ops、ServerAdmin、Mcp 八值。`user_devices.client` 只持久化前七值，Mcp 复用 grant 来源设备；普通 HTTP 可声明前七值，Mcp 只能由 `/mcp` grant middleware 固定，外部自填无效。DepartmentScope 取 All、Subtree、Explicit 三个变体，第 4.2 节阶段三的部门范围编译结果落在该枚举上；SystemPurpose 取 General、Reconciliation 两值，后者除定义处外只允许在 `crates/platform/recon/src/executor.rs` 出现并由 archcheck 静态拒绝其他构造点。request_id 与 trace_id 两个字段是基线第 3.8 节要求写入 app.request_id 与 app.trace_id 两条会话变量的取数来源，安全上下文之外不得另设第二处取数。

ep-foundation 侧还须冻结上表出现的七个字段类型，它们与 SecurityContext 同处 `crates/foundation/src/security/context.rs`，按 A-03 由阶段 1 一并实现，本阶段只填充不定义；A-03 的字段表以本节的结构体成文，因此这七个类型的形状与取值域也在本节给全，阶段 1 照此冻结。

```rust
pub struct DeviceId(Arc<str>);
pub struct RoleCode(Arc<str>);
pub struct DataScopeTag(Arc<str>);
pub struct RequestId(Arc<str>);
pub struct TraceId(Arc<str>);

pub enum DutyClass { System, Data, Security, Audit, Key, Config }

pub struct RecordShare {
    pub object_type: Arc<str>,
    pub object_id: uuid::Uuid,
    pub grant: RecordShareGrant,
}

pub enum RecordShareGrant { Read, Write }
```

`RecordShare` 的现行结构以上述三字段为准：`grant` 只能是 `Read|Write`，`can_reshare` 首版恒为 false 且不进结构。U-B-07 已由 F-51 关闭，不得按两字段旧形态落码。

五个字符串 newtype 的构造入口一律为 `parse(&str) -> Result<Self, AppError>`，不提供绕过校验的构造，也不实现 `From<String>`。DeviceId 长度 1 至 64，字符集固定为大小写字母、数字、下划线与连字符 `[A-Za-z0-9_-]`，与 platform_core.user_devices.device_id 列和基线第 5.6 节 X-Device-Id 头同域，点号不合法；基线第 1.4 节的 SYSTEM_DEVICE_ID 能通过该校验，`SecurityContext::system` 由此构造该字段。RoleCode 长度 1 至 64，字符集固定为大写字母、数字与下划线 `[A-Z0-9_]`，与 `platform_authz.roles.code` 同域，该列的写入一律经 RoleCode 解析后落库，数据库侧不另设第二套字符集校验。DutyClass 六个变体的序列化取值与 platform_authz.roles.duty_class 的六个字符串逐字一致，该列为空的业务角色不产生任何变体，因此 duty_classes 允许为空数组，不设表示无职责的第七个变体；互斥关系不进枚举定义，它是第 4.5 节种子 SoD 规则行的内容。RecordShare 只表达某条记录被显式共享给当前主体，object_type 与 platform_authz.object_scope_bindings.object_type 同域，第 4.2 节阶段三的 shared_record_ids 由它按 object_type 过滤后取 object_id 汇成；`grant` 承载 `Read|Write` 授予方式，记录级动作仍受阶段二权限项约束，字段粒度由阶段四承担；结构不带 `can_reshare`，首版转授恒为 `false`。U-B-07 的 ScopeCompiler 分支必须同时校验对象与 grant，不得恢复两字段形态。RecordScope 与 RecordPredicate 不进 ep-foundation，留在 ep-platform-authz，理由是两者含判定语义，前移即违反基线第 1.3 节的依赖方向。DataScopeTag 的形态为 `<kind>:<value>`，kind 取小写字母、数字、下划线与连字符，value 取大小写字母、数字、下划线与连字符，总长上限 128，其 Display 与 serde 输出即基线第 4 节公共列 data_scope_tags 的元素形态与基线第 6.1 节事件信封 data_scope_tags 的元素形态，两处不得各自编解码。RequestId 长度 8 至 64，字符集为大小写字母、数字、下划线与连字符，与基线第 5.6 节 X-Request-Id 头同域，服务端自生成时取 UUIDv7 的无连字符三十二位小写十六进制。TraceId 固定为三十二位小写十六进制，与 W3C trace-context 的 trace-id 同形，也与结构化日志的 trace_id 字段同域。

ep-platform-authz 侧：

```rust
pub enum Action { View, Create, Update, Submit, Approve, Export }

pub enum Decision { Allow, Deny(DenyReason) }

pub enum DenyReason {
    LegalEntityNotGranted,
    ObjectForbidden,
    RecordNotVisible,
    FieldForbidden { field: FieldName },
    ClassificationTooHigh { required: SecurityLevel },
    SeparationOfDutyViolation { rule_code: String },
    ReauthRequired { operation: HighRiskOperation },
    ApprovalRequired { chain_code: String },
    ScopeBindingMissing { object_type: String },
}

pub enum RecordScope {
    All,
    Predicate(RecordPredicate),
    None,
}

pub struct RecordPredicate {
    pub owner_self: bool,
    pub departments: Arc<[Id<Department>]>,
    pub projects: Arc<[Id<Project>]>,
    pub customers: Arc<[Id<Customer>]>,
    pub shared_record_ids: Arc<[Uuid]>,
    pub max_security_level: SecurityLevel,
}

pub enum FieldVisibility { Hidden, Masked(MaskStyle), Read, Write }

pub enum HighRiskOperation {
    ContractEffective, Payment, InvoiceIssue,
    LedgerPosting, PeriodClose, SensitiveExport,
    DataMigration,
}
```

`HighRiskOperation` 的 serde/数据库值依次为 `CONTRACT_EFFECTIVE`、`PAYMENT`、`INVOICE_ISSUE`、`LEDGER_POSTING`、`PERIOD_CLOSE`、`SENSITIVE_EXPORT`、`DATA_MIGRATION`。前六项是业务高风险类并进入本阶段通用高风险请求单；`DataMigration` 是运维高风险类，只由阶段 14 专用流程消费统一 reauth challenge。审批链场景与上述七类重新认证操作不是同一个枚举：一个审批场景可以不要求重新认证，也可能由同一个高风险操作触发。`ep-platform-authz::approval::ApprovalScenarioCode` 是审批场景的唯一闭集，serde 与数据库逐字使用下列三十七个 `SCREAMING_SNAKE_CASE` 值，阶段 3、5、6、7、9、10、11、13、14 只引用，不得各自再建字符串常量或“业务模块自注册”入口：

```rust
pub enum ApprovalScenarioCode {
    MdmCustomerChange,
    MdmSupplierChange,
    MdmMaterialChange,
    MdmProductChange,
    MdmWarehouseChange,
    ContractEffective,
    ContractCreditOverride,
    ContractDiscountOverride,
    ContractTermination,
    SalesReturn,
    ProcurePurchaseOrder,
    ProcurePurchaseReturn,
    ProcurePaymentRequest,
    ProcureOverReceipt,
    InvoiceApplication,
    InvoiceSalesIssue,
    InvoiceReversal,
    InvoiceImportBatchIssue,
    InvoicePurchaseCreditNote,
    FinanceReceipt,
    FinancePayment,
    FinanceRefund,
    FinanceCashAccount,
    FinanceCashDocumentReversal,
    FinanceOverbillingWriteOff,
    FinanceOverbillingWriteOffReversal,
    FinanceOpeningBalanceImport,
    LedgerAccountReferenceImport,
    LedgerBackdate,
    LedgerPeriodClose,
    LedgerYearEndClose,
    LedgerCorrectionVoucher,
    ConfigRelease,
    ExtensionEnable,
    ReportEnterprisePublish,
    ReportSensitiveExport,
    CostReturnMark,
}
```

精确持久化值依次为 `MDM_CUSTOMER_CHANGE`、`MDM_SUPPLIER_CHANGE`、`MDM_MATERIAL_CHANGE`、`MDM_PRODUCT_CHANGE`、`MDM_WAREHOUSE_CHANGE`、`CONTRACT_EFFECTIVE`、`CONTRACT_CREDIT_OVERRIDE`、`CONTRACT_DISCOUNT_OVERRIDE`、`CONTRACT_TERMINATION`、`SALES_RETURN`、`PROCURE_PURCHASE_ORDER`、`PROCURE_PURCHASE_RETURN`、`PROCURE_PAYMENT_REQUEST`、`PROCURE_OVER_RECEIPT`、`INVOICE_APPLICATION`、`INVOICE_SALES_ISSUE`、`INVOICE_REVERSAL`、`INVOICE_IMPORT_BATCH_ISSUE`、`INVOICE_PURCHASE_CREDIT_NOTE`、`FINANCE_RECEIPT`、`FINANCE_PAYMENT`、`FINANCE_REFUND`、`FINANCE_CASH_ACCOUNT`、`FINANCE_CASH_DOCUMENT_REVERSAL`、`FINANCE_OVERBILLING_WRITE_OFF`、`FINANCE_OVERBILLING_WRITE_OFF_REVERSAL`、`FINANCE_OPENING_BALANCE_IMPORT`、`LEDGER_ACCOUNT_REFERENCE_IMPORT`、`LEDGER_BACKDATE`、`LEDGER_PERIOD_CLOSE`、`LEDGER_YEAR_END_CLOSE`、`LEDGER_CORRECTION_VOUCHER`、`CONFIG_RELEASE`、`EXTENSION_ENABLE`、`REPORT_ENTERPRISE_PUBLISH`、`REPORT_SENSITIVE_EXPORT`、`COST_RETURN_MARK`。以后新增场景必须走一次兼容迁移、枚举升级、默认链决策与跨模块契约测试，不允许把未知字符串当作自定义场景放行。

ep-platform-identity 侧的关键枚举：AccountStatus、CredentialKind、SessionState、ReauthState、BreakglassState。

#### 4.2 授权判定流水线

判定顺序照抄基线第 11.3 节：先法人、再对象级、再记录级、再字段级与密级，显式拒绝优先。PRD 第 10.2.1 节的七个维度按下表映射进这四个阶段，不新增第五个阶段，也不改变顺序。

| PRD 维度 | 落在哪一阶段 | 判据 |
|---|---|---|
| 法人 | 阶段一 | 会话上下文中的 legal_entity_id 与数据库行级策略，唯一判据 |
| 岗位 | 阶段二 | 岗位绑定的角色所持有的 permission_item + action |
| 部门 | 阶段三 | RecordPredicate.departments，取用户部门及其下级闭包 |
| 项目 | 阶段三 | RecordPredicate.projects |
| 客户 | 阶段三 | RecordPredicate.customers |
| 记录 | 阶段三 | owner_self、流程当前处理人、shared_record_ids |
| 字段 | 阶段四 | field_permissions 叠加对象密级与字段密级 |

判定过程：

1. 阶段一。middleware 已把 app.legal_entity_id 写入会话变量，且 user_legal_entity_grants 上的行对该法人可见。此处不再做应用侧比较，法人隔离完全由行级策略承担，符合规格第 7.7 章“行级策略以该变量为唯一判据”。若客户端声明的法人不在授权集合内，grant 行不可见，返回 PLATFORM.AUTHZ.LEGAL_ENTITY_NOT_GRANTED，HTTP 403。
2. 阶段二。在 AuthzSnapshot 中按 (roles, object_type, action) 查表。先收集全部命中的 access_policies，effect 为 DENY 的任一条命中即返回 Deny(ObjectForbidden)；没有 DENY 且存在 role_permission_grants 命中即通过；两者皆无即 Deny(ObjectForbidden)。显式拒绝优先在此实现。
3. 阶段三。按 object_type 从 object_scope_bindings 取范围锚列；未登记直接 Deny(ScopeBindingMissing)，不默认放行。部门闭包不在本阶段自行展开，按 A-04 经 `ep_platform_tenancy::DepartmentClosureQuery::descendant_ids(tx, legal_entity_id, department_id, max_depth)` 取得，事务句柄类型为 `ep_foundation::port::Tx` 的 `&mut dyn Tx`，max_depth 取 EP__AUTHZ__SCOPE__MAX_DEPARTMENT_DEPTH。把该闭包结果与项目集合、客户集合、显式共享记录集合编译成 RecordPredicate，再由 ep-adapter-db-pg 的 ScopePredicateRenderer 渲染成 SQL 片段附加到仓储查询的 WHERE 后。单条读取时先按主键取行再用同一谓词在内存中比对。
4. 阶段四。取该对象的 security_level 与各字段的字段密级，用户 clearance_level 低于对象密级时整体 Deny(ClassificationTooHigh)；低于字段密级或字段权限为 HIDDEN 时该字段不进入响应键集合；为 MASKED 时按 mask_style 替换值；为 READ 时只读；为 WRITE 时允许写。

边界条件逐条：用户角色集合为空时阶段二直接拒绝，不回退到任何默认角色；字段密级未赋值时按所属对象密级取值，照抄基线第 4 节公共列说明；部门闭包深度上限取 8，超过深度的部门在编译期被截断并写 WARN 日志与 ep_authz_scope_truncated_total 指标，不静默；departments 集合超过 200 个时谓词由 IN 列表退化为对部门闭包临时表的 EXISTS 子查询，阈值由常量表达，理由是超长 IN 列表会让查询计划从索引扫描退化为顺序扫描，与基线第 3.10 节“不得出现顺序扫描”的要求冲突；调岗生效日期的比较用 (now() AT TIME ZONE 'Asia/Shanghai')::date，不用 current_date，照抄基线第 3.4 节。

复杂度：阶段二为哈希查表，阶段三为集合构造，两者均不触库，目标是单次判定 P95 低于 1 毫秒，由 ep_authz_decision_duration_seconds 度量。

#### 4.3 认证与会话

登录算法：

1. 记录活跃用户。活跃用户定义为最近 60 秒内有过请求的不同 `user_id`，内部与门户合计计数；超过 20 人不拒绝登录、不排队、不拒绝写入，只记录指标、发出告警并把该时段标记为性能 SLA 不适用。管理端每 5 秒刷新一次当前值。本规则只观测命名用户规模，不替代 HTTP 层独立的瞬时请求并发保护。
2. 按 login_name 取 user_accounts 行。未找到时仍执行一次固定成本的 Argon2id 校验（对内置的伪 PHC 串），使未知用户与错口令的响应时间同分布，随后返回同一个错误码 PLATFORM.AUTHN.CREDENTIAL_INVALID。
3. 取 account_lockouts 行并加行锁。locked_until 大于当前时刻即返回 PLATFORM.AUTHN.ACCOUNT_LOCKED。
4. 校验第一因子。PASSWORD 走 Argon2id 验证并检查口令有效期；X509_CERT 走挑战签名验签。
5. 判定是否需要第二因子。is_mfa_required 为真，或该用户持有任一 duty_class 非空的有效角色授予，或该用户被授予任一含高风险操作权限项的角色时，强制要求第二因子，对应规格第 12.1 章“管理员与高风险角色强制 MFA”。六类业务高风险权限沿用既有所属对象；`DATA_MIGRATION` 是否有权发起按阶段 14 的专用权限项、角色映射与业务对象重检，不从 enum 值推导授权。
6. 校验设备。X-Device-Id 必须在 user_devices 中且 status 为 ACTIVE，否则返回 PLATFORM.AUTHN.DEVICE_NOT_REGISTERED，照抄基线第 5.6 节“未登记设备拒绝访问业务数据”。
7. 会话数上限。该用户 expires_at 未到且 revoked_at 为空的会话超过 3 个时，把 issued_at 最早的一条置为 revoked，revoke_reason 取 SESSION_LIMIT_EXCEEDED，并写一条审计事件，照抄基线第 11.6 节。
8. 生成 32 字节随机令牌，base64url 编码后长度为 43，只把 SHA-256 摘要写入 sessions.token_hash，明文令牌只在响应体中出现一次。
9. 写入 sessions、写入 login_attempts、重置 account_lockouts 计数，最后批量写入审计事件，四项在同一事务内提交；审计终结批之后不得再执行任何数据库语句。

X509 登录/MFA 不留“证书指纹”和“挑战签名”两个自由实现。`EP__AUTH__X509__TRUST_ANCHOR_REF` 为 canonical SecretRef；按当前 recipient 解出的 exact bytes 必须是最大 1,048,576 bytes 的 DER empty-content/no-signer CMS CA+完整 base-CRL bundle，并使用签名部署清单冻结的 whole-chain、最高覆盖 CRL、AlgorithmIdentifier 与无 OS/network fallback 规则。其 SHA-256 必须逐字等于同一已验签部署清单的 `x509_login_trust_bundle_sha256`，否则 ep-migrate bootstrap 与 core readiness 都失败。X509 credential 的 leaf 必须 DigitalSignature+ClientAuth 且不得 CodeSigning，形成唯一有效链；presented leaf exact DER hash 必须等于 `verifier` 的 `cert-sha256:` 值，SKI raw bytes 必须等于 `credential_handle` 与 CMS version-3 SignerInfo sid，禁止仅按 DN、serial 或 SPKI 接受换证。

`SIGN_IN_MFA` 的 X509 待签体是最大 4096-byte strict RFC 8785 `X509SignInChallengeV1`，exact 字段为 `{schema_version:1,purpose:"EP-X509-SIGN-IN-MFA-V1",deployment_id,challenge_id,user_id,user_device_row_id,default_legal_entity_id,client,nonce_b64url,issued_at,expires_at}`；nonce 恰 32 random bytes、canonical base64url-no-pad，所有 id/time/client 逐字来自已落库 challenge 与 signed deployment。sign-in 响应在需要 X509 时同时返回不透明 `mfa_challenge` 与该 exact JCS 对象。complete-mfa 的 credential exact 为 `{kind:"X509_CERT",credential_handle_b64url,signature_cms_b64url}`；signature 是对 `JCS(X509SignInChallengeV1)` 的 DER detached CMS，形状复用部署管理员 CMS 的 one-SignerInfo/SKI/SHA-256/三 signedAttrs/no unsignedAttrs/整链闭集，`signingTime` 是落在 `[issued_at,expires_at]` 且不晚于 trusted-now+5min 的 UTC whole-second instant。服务端只能从 token hash 锁定 challenge 后重建待签体，禁止接受客户端自报 body；CMS、DB credential、用户、设备、法人、client、时限任一不等均以 `MFA_INVALID` 失败并按既有计数落库。作为第一因子用过的同一 credential id 不可再充当第二因子。

F-56 fresh bootstrap 的两名管理员固定以 PASSWORD 完成 sign-in 第一因子、以上各自不同 X509 credential 完成第二因子。bootstrap CMS 已证明两把私钥在建号时可用；仓库仍须为两人各有一条完整 sign-in→complete-mfa 正向 golden fixture，并覆盖错 cert hash、错 SKI、错 challenge body、同 credential 重用、登录 trust bundle digest 漂移、双链和 CRL 命中的负例。该夹具是实现/Stage 14 门禁，不新增首装端点，也不要求 core 在开放认证恢复入口前等待一次人工登录。

失败路径的事务处理是本阶段最容易写错的一处：认证失败必须持久化失败计数，而基线第 10.3 节禁止一个请求内开多个写事务。做法是登录用例的事务闭包永远返回 Ok(LoginOutcome)，其中 LoginOutcome 有 Succeeded 与 Rejected 两个变体，失败计数与失败审计在同一次提交中落库，事务提交后再由用例把 Rejected 映射成 AppError 返回给 HTTP 层。禁止用回滚表达失败。

会话校验：每个受保护请求取 Authorization 头，SHA-256 后查 sessions，校验 expires_at 与 idle_expires_at，通过后把 idle_expires_at 滑动到当前时刻加 30 分钟。滑动续期的写入合并到该请求已有的事务中；只读请求的滑动续期改为按 60 秒粒度批量写，避免每个查询请求都产生一次写事务。

#### 4.4 高风险操作的重新认证与审批

状态机照抄 PRD 第 10.3.3 节，十一个状态：待发起 PENDING_INITIATION、待重新认证 PENDING_REAUTH、认证失败 REAUTH_FAILED、已锁定 LOCKED、已认证待提交 REAUTH_PASSED、审批中 IN_APPROVAL、已批准 APPROVED、已驳回 REJECTED、已撤回 WITHDRAWN、已放弃 ABANDONED、已执行 EXECUTED。本阶段自足的部分止于 REAUTH_PASSED：PENDING_INITIATION、PENDING_REAUTH、REAUTH_FAILED、LOCKED、REAUTH_PASSED、ABANDONED 六态与其间的迁移在本阶段实现并验收。IN_APPROVAL、APPROVED、REJECTED、WITHDRAWN、EXECUTED 五态的迁移都要先建立审批实例，属开篇同批清单第一项，与阶段 3b 的流程引擎本体一并实现、一并验收；本阶段不为它们注入任何替身，也不以假实现跑通后声称可演示，下表把这五态的行一并冻结为对流程引擎的接口约束。

| 起态 | 止态 | 触发 | 守卫条件 |
|---|---|---|---|
| PENDING_INITIATION | PENDING_REAUTH | 用户提交且动作命中六类业务高风险 | 发起人对该对象持有 SUBMIT 权限；已存在生效的审批链定义 |
| PENDING_REAUTH | REAUTH_PASSED | 挑战核销通过 | 认证主体等于发起人；subject_digest 与服务端重算值相等；挑战未过期未消费 |
| PENDING_REAUTH | REAUTH_FAILED | 认证不通过 | failure_count 加一 |
| PENDING_REAUTH | ABANDONED | 用户取消或挑战超时 | 单据内容保留为草稿 |
| REAUTH_FAILED | PENDING_REAUTH | 重试 | failure_count 未达阈值 |
| REAUTH_FAILED | LOCKED | 达阈值 | 同时锁定该用户账号 |
| REAUTH_PASSED | IN_APPROVAL | 提交，建立审批实例 | 审批链静态校验通过；发起人不在任一节点的可审批集合中；X-Reauth-Token 校验通过并被消费 |
| IN_APPROVAL | APPROVED | 全部节点通过 | 节点按 node_no 递增依次完成，不允许跳号 |
| IN_APPROVAL | REJECTED | 任一节点驳回 | |
| IN_APPROVAL | WITHDRAWN | 发起人撤回 | 首个节点尚未作出结论 |
| APPROVED | EXECUTED | 平台执行业务动作 | 由业务模块回调 confirm_execution，写入 execution_ref |
| REJECTED / WITHDRAWN / ABANDONED | PENDING_INITIATION | 修改后重新发起 | 产生新的 high_risk_requests 行，旧行保留 |

待签内容摘要的规范化算法：取操作类型、法人 ID、单据编号、关键金额或会计期间、生效影响的一句话说明五项，按固定键顺序序列化为紧凑 JSON，数值一律用 Decimal 的定长字符串表示，再取 SHA-256。服务端在核销时用命令载荷重新计算摘要并与挑战中存的 subject_digest 逐字节比对，不采信客户端传来的摘要值。这一步是规格第 12.1 章“待签内容摘要写入审计证据”能够成立的前提：如果摘要由客户端提供，审计证据证明的就只是客户端说了什么。

受保护动作缺少 `X-Reauth-Token`、令牌格式非法、主体不匹配或未达到 VERIFIED 状态时，统一返回 `PLATFORM.AUTHZ.REAUTH_REQUIRED`；只有令牌曾经有效但已经消费或已失效时返回 `PLATFORM.REAUTH.TOKEN_ALREADY_CONSUMED`。X-Reauth-Token 的消费是一次条件更新：`UPDATE platform_core.reauth_challenges SET status='CONSUMED',consumed_at=now(),row_version=row_version+1,updated_at=now(),updated_by=$actor WHERE id=$1 AND status='VERIFIED' AND expires_at>now() RETURNING id,row_version`；无返回行即按上述分类拒绝。登录 MFA 的 `ISSUED → CONSUMED` 复用同一仓储 CAS 形状，只把起始状态改为 ISSUED。两条都显式增版以满足全库 `assert_row_version_bump()`，无需客户端 expected version；该更新与业务写入同事务，因此并发消费恰一成功，后续业务失败回滚后挑战也恢复到原状态并可由同一幂等请求再次消费。

#### 4.5 职责分离与审批链静态校验

`ApprovalDefaultCatalog` 是 `ApprovalScenarioCode -> 非空顺序节点模板` 的编译期全映射，match 缺一项即编译失败。五个 MDM 场景分别取 `MDM_CUSTOMER_APPROVER`、`MDM_SUPPLIER_APPROVER`、`MDM_MATERIAL_APPROVER`、`MDM_PRODUCT_APPROVER`、`MDM_WAREHOUSE_APPROVER`；`CONTRACT_EFFECTIVE`、`CONTRACT_DISCOUNT_OVERRIDE`、`CONTRACT_TERMINATION`、`INVOICE_APPLICATION`、`REPORT_ENTERPRISE_PUBLISH`、`REPORT_SENSITIVE_EXPORT` 取 `MANAGEMENT_APPROVER`；`CONTRACT_CREDIT_OVERRIDE`、`INVOICE_SALES_ISSUE`、`INVOICE_REVERSAL`、`INVOICE_IMPORT_BATCH_ISSUE`、`INVOICE_PURCHASE_CREDIT_NOTE`、`FINANCE_RECEIPT`、`FINANCE_PAYMENT`、`FINANCE_REFUND`、`FINANCE_CASH_ACCOUNT`、`FINANCE_CASH_DOCUMENT_REVERSAL`、`FINANCE_OVERBILLING_WRITE_OFF`、`FINANCE_OVERBILLING_WRITE_OFF_REVERSAL`、`FINANCE_OPENING_BALANCE_IMPORT`、`LEDGER_ACCOUNT_REFERENCE_IMPORT`、`LEDGER_BACKDATE`、`LEDGER_PERIOD_CLOSE`、`LEDGER_YEAR_END_CLOSE`、`LEDGER_CORRECTION_VOUCHER`、`COST_RETURN_MARK` 取 `FINANCE_MANAGER`；`SALES_RETURN` 取 `SALES_MANAGER`；`PROCURE_PURCHASE_ORDER`、`PROCURE_PURCHASE_RETURN`、`PROCURE_PAYMENT_REQUEST`、`PROCURE_OVER_RECEIPT` 取 `PROCURE_MANAGER`；`CONFIG_RELEASE` 与 `EXTENSION_ENABLE` 取 `SECURITY_ADMIN`。除五个 MDM 专用角色、销售、采购、配置/扩展与报表这些表中明确列出的例外，全部财务、发票和总账场景的默认审批角色都是 `FINANCE_MANAGER`。每个默认模板首版只有一个 `approver_kind=ROLE`、`quorum=1`、`timeout_hours=24` 的节点；到款默认开关为 false 只表示入口不启动审批，不表示默认链可以缺失。

默认链 provision 有且只有 `ApprovalChainProvisioner::provision_defaults(&mut dyn Tx, legal_entity_id)` 一个入口。`V20261012120500__platform_authz_backfill_default_approval_chains.sql` 对全部存量法人调用同一份确定性 catalog 语义；新法人引导事务在插入法人、系统主体法人授权之后、提交之前调用该入口。链 code、链 id 与节点 id 均由 `(legal_entity_id,scenario,seed_version)` 确定性派生，按 `(legal_entity_id,scenario,version_no)` 与 `(legal_entity_id,approval_chain_id,node_no)` 冲突即读取核对，完全相同视为幂等，不同则失败关闭并要求受控修复。已有唯一活动客户链时不覆盖；没有活动链时才补默认活动链。标准角色包只定义 RoleCode 而不自动绑定自然人，因此 provision 可以建立引用角色的结构有效链；若该法人尚无对应有效用户，下面的运行期展开仍以 `PLATFORM.APPROVAL.NODE_HAS_NO_APPROVER` 拒绝业务提交，不得把空角色当作自动通过。

F-56 fresh production 不是第二个法人/身份引导器：既有 `ep-migrate apply` 的 initial-governance 分支必须在同一 PostgreSQL 事务调用上述唯一 `provision_defaults`，并按 F-56 signed bootstrap 创建两名用户及其密码/X509 凭据、设备、治理法人授权、`F56_CONFIG_OPERATOR` 与 `SECURITY_ADMIN` 角色/权限/绑定；不得复制 catalog 或绕过本阶段约束。两份 X509 verifier/handle 逐字取对应 bootstrap CMS leaf 的 exact DER SHA-256 与 SKI，并且 leaf 必须分别命中已验签 deployment manifest 的不同 customer-security-admin roster entry、同时通过上节登录 trust bundle。默认 `CONFIG_RELEASE` 链只指向 `SECURITY_ADMIN`；CONFIG/SECURITY duty 互斥、自审与空审批人规则全部照常执行。fresh exact 零行前置、PROVISIONING key domain、receipt 与 core readiness resume 只取 F-56，不由本阶段另定义。该分支正例必须证明首次许可包的 CONFIG_OPERATOR 可用 password+X509 登录并提交、另一 SECURITY_ADMIN 可独立完成同一内容 hash 的审批，申请人自审仍被拒绝。

客户发布新版本时必须在一个事务内原子切换：先 `select id from platform_core.legal_entities where id=$1 for update` 取得该法人的串行化锁，再按 `scenario=$2` 锁全部链行；计算 `max(version_no)+1`，以 `is_active=false` 写新链与全部节点并完成结构校验和当下人员展开预检；随后先把旧活动行置 `is_active=false,deactivated_at=now()`，再把新行置 `lifecycle_state='EFFECTIVE',is_active=true`。任一步失败整笔回滚，外部观察者只会看到旧版或新版；不得先停旧版再另开事务启新版。生成列唯一约束是并发和漏锁时的最后防线，命中冲突必须返回配置冲突，不能最后写入者覆盖。

`ApprovalChainResolver::resolve_active_chain(&mut dyn Tx, legal_entity_id, scenario, initiator_user_id)` 是所有模块共用的唯一解析入口，精确读取顺序如下；流程引擎只接收其返回的不可变快照，不得重新查询或自己解释链：

```sql
select id, version_no
from platform_authz.approval_chains
where legal_entity_id = $1
  and scenario = $2
  and lifecycle_state = 'EFFECTIVE'
  and is_active = true
order by version_no, id
for share;

select id, node_no, approver_kind, approver_ref, role_code, quorum, timeout_hours
from platform_authz.approval_chain_nodes
where legal_entity_id = $1 and approval_chain_id = $2
order by node_no, id
for share;
```

第一条结果为零行返回 `PLATFORM.APPROVAL.CHAIN_NOT_FOUND`；大于一行即使理论上已被唯一约束阻断也必须返回 `PLATFORM.APPROVAL.ACTIVE_CHAIN_AMBIGUOUS` 并产生安全告警，不得任选一行。恰好一行后，第二条必须得到 1 至 10 个节点，`node_no` 必须严格为 `1..=n`；逐节点按同一事务快照展开当前有效用户并去重，每个集合非空、`1 <= quorum <= expanded_user_count`、发起人不属于任何集合。零节点、空展开或 quorum 无法满足统一返回 `PLATFORM.APPROVAL.NODE_HAS_NO_APPROVER`；发起人命中返回 `PLATFORM.SOD.SELF_APPROVAL_FORBIDDEN`。成功结果冻结 `{legal_entity_id,scenario,chain_id,chain_version_no,nodes,initiator_user_id,resolved_at,definition_digest}`，业务提交事务用这份快照启动流程并持久化引用。链在流程启动后换版不改写在途实例。

四类规则，全部在配置保存时执行一次、在运行期提交时再执行一次，两次用同一份纯函数实现。

1. DUTY_EXCLUSION。同一用户在同一法人内不得同时持有 SYSTEM、DATA、SECURITY、AUDIT、KEY 五类中的两类，照抄规格第 12.2 章。CONFIG 类原属 PRD 附录乙 U-B-17，现已由 F-51 确认本阶段冻结值：CONFIG 与 SECURITY 互斥、与其余四类可兼；实现不得二次选择。
2. ROLE_EXCLUSION。客户自定义的角色互斥对。
3. SELF_APPROVAL。审批链的任一节点的可审批主体集合与发起人集合的交集必须为空。校验方式是把节点的 approver_kind 展开成用户集合：ROLE 展开为该法人内持有该角色的有效用户，POSITION 展开为该岗位的在岗用户，DEPT_MANAGER 展开为发起人所在部门链上的负责人。交集非空即拒绝保存，错误码 PLATFORM.SOD.SELF_APPROVAL_FORBIDDEN，消息中指出冲突节点号，对应 PRD 第 10.3.4 节“指出冲突节点”。
4. CHAIN_SKIP。节点 node_no 必须从 1 起连续无空洞，quorum 必须为正且不超过该节点展开后的用户数，不存在任何表达“允许跳过”的字段。运行期推进时校验前序节点全部完成，且不提供任何管理端强制完成节点的接口。规格第 12.2 章“审批链不可越权跳过”在本设计中不是一条被校验的规则，而是一个不存在的能力。

边界条件：客户链激活时节点展开后用户集合为空即拒绝激活，理由是空集合的节点在运行期等价于隐藏跳过；唯一例外是上述系统默认 provision 可以先建立引用尚未绑定自然人的标准角色定义，但它不会获得运行期放行豁免。用户离职停用或默认角色尚未绑定导致节点展开为空时，提交保持业务零写入并返回 `PLATFORM.APPROVAL.NODE_HAS_NO_APPROVER`，不自动跳过、不自动升级、不自动通过或驳回。无链、并列活动链和空节点也分别按上述稳定码 fail-closed，不得合并成“无需审批”。

#### 4.6 受控应急本地账号

状态机：DRAFT、PENDING_APPROVAL、APPROVED、ACTIVE、EXPIRED、CLOSED、REJECTED。启用四要素按规格第 12.1 章逐项落地：限时由 expires_at 与 job-worker 的到期任务保证，单次不超过 8 小时；强认证由强制第二因子且 credential_kind 不得为 PASSWORD 单因子保证；实时告警由启用瞬间写入阶段 2 已交付的 platform_ops 台账并经其告警通道发出保证，站内通知投递属开篇同批清单第四项，随阶段 3b 一并接入，本阶段不为它注入替身；独立人员复核由 approved_by 与 requested_by 必须不同人且 approved_by 需持有 SECURITY 或 AUDIT 职责保证。

允许操作集合的执行点在授权判定阶段二之前插入一道前置闸门：会话的 is_breakglass 为真时，只有 allowed_action_set 中三类操作对应的 permission_item 可通过，其余一律 Deny(ObjectForbidden) 并触发告警。业务写入、审计策略修改、密钥签发轮换销毁、职责分离绕过、常规业务审批五类操作在该闸门处被拒绝，对应规格第 12.1 章的禁止集合。

到期后由 job-worker 执行：撤销该账号全部会话、把凭据置为 REVOKED、生成新凭据并写入机密库、把轮换结果写入 breakglass_activations.rotation_result 与审计。闲置轮换按每 12 个月一次，由同一任务按 last rotated_at 判定。

#### 4.7 字段投影

FieldProjector 输入为对象类型、对象的原始行（serde_json::Value）与 SecurityContext，输出为新的 Value，不修改输入。掩码规则：FULL 输出固定字符串六个星号；KEEP_LAST_4 保留末四位其余替换为星号，长度不足 8 位时退化为 FULL；KEEP_DOMAIN 用于电子邮箱，保留 at 之后的部分。字段在 platform_core.sensitive_field_registry 中登记且 is_field_encrypted 为真时物理列是密文，上述三条不施加于密文：KEEP_LAST_4 的后四位直接取自同表的 `<column_name>_tail` 列，FULL 与 HIDDEN 既不读密文也不解密；只有字段权限为 READ 或 WRITE 且用户 clearance_level 不低于该字段密级时，才在投影前经 SensitiveFieldDecryptor 解密后输出，字段投影路径上只有这一处解密位点，不经字段投影而需要明文的解密由需要它的那个阶段在其计划内自行指名位点，同样调用 SensitiveFieldDecryptor，全库不得出现第二套解封路径。按 A-28，命中该分支的字段以 platform_core.sensitive_field_registry 中 is_field_encrypted 为真的登记行为准，本阶段不另列第二份清单；登记行与其物理列由引入该列的模块阶段在同一迁移内交付，本阶段只按登记行渲染，不建表也不写登记行；登记行的 mask_style 取 KEEP_LAST_4 时该表必须同时存在 `<column_name>_tail` 列，没有该列的只能取 FULL。字段在 field_permissions 中无授权行时按默认拒绝处理，不进入响应键集合，与阶段二的默认拒绝一致；各模块字段的授权行按 A-19 的 AUTHZ_FIELD_GRANT applier 经配置发布通道在其所属阶段之后写入。掩码后的值不参与排序与聚合，任何列表端点如果按 MASKED 或 HIDDEN 字段排序，一律返回 VALIDATION 与 PLATFORM.AUTHZ.SORT_FIELD_FORBIDDEN，这是 PRD 第 10.2.4 节“不得通过排序位次间接暴露”的实现点。分面计数同理：计数的分组键若含无权字段，该分面整体不返回。
#### 4.8 权限配置对象的配置包 applier

按 A-19/F-56，ConfigItemApplier trait、ConfigPackageItem 与 ConfigItemApplierRegistry 由阶段 3a 在 `crates/platform/release/src/port/config_item.rs` 交付，其中的事务句柄类型取自 ep-foundation；Stage 3b 的 Rust/DB `ItemKind` 同序快照为原 16 项加 `LICENSE_GRANT|MODULE_PACKAGE` 共 18 项，Stage 13b 再原子追加两项 MCP 成终态 20。本阶段只实现其中 `AUTHZ_ROLE|AUTHZ_POLICY|AUTHZ_FIELD_GRANT` 三项，实现类型全部落在 ep-platform-authz；不得在本阶段另改枚举顺序或数据库 CHECK。

| item_kind | 实现类型 | 覆盖的配置表 |
|---|---|---|
| AUTHZ_ROLE | AuthzRoleApplier | platform_authz.roles、platform_authz.role_permission_grants |
| AUTHZ_POLICY | AuthzPolicyApplier | platform_authz.access_policies、platform_authz.sod_rules、platform_authz.approval_chains、platform_authz.approval_chain_nodes |
| AUTHZ_FIELD_GRANT | AuthzFieldGrantApplier | platform_authz.field_permissions |

三个 applier 按端口签名接受调用方传入的 `&mut dyn Tx`，在同一事务内完成配置写入与 authz_config_versions 的版本推进，不自行开事务、不做外部调用、不发通知、不在事务内触发快照重载；快照重载由 core-server 轮询版本号完成，见第 2.3 节。三者在配置保存期与运行期共用第 4.5 节的同一份静态校验纯函数，不另写一套。敏感字段登记不属于这三个 item_kind：按 C-06 该登记表落在 platform_core 且由阶段 2 建立，登记行由各模块阶段以 backfill 迁移写入。本阶段只把三个 applier 注册进阶段 3a 交付的 ConfigItemApplierRegistry 并对其写入与版本推进做单元测试，不接线任何替身；注册表随配置发布通道本体在阶段 3b 的运行期装配属开篇同批清单第二项。

---

### 5. API 契约

全部端点前缀为 /api/v1/platform，门户侧为 /api/v1/portal。请求头、封套、分页、排序、过滤、幂等键一律按基线第 5 章，本节只写差异与逐端点的语义。
本节全部路由按 A-20 在路由注册处一次性给出一个 `(CapabilityDomain, ActionClass)` 元组，两个枚举由阶段 1 冻结，本阶段不自定义能力域码也不重新定义枚举。原先逐路由声明 `<USECASE_SCREAMING>_DOMAIN` 与 `<USECASE_SCREAMING>_ACTION` 两个常量的写法与其承载文件 `crates/platform/authz/src/capability.rs` 一并删除，理由是同一事实写两个常量再由第三处引用，每加一个用例就多两处可漂移的登记，而元组与路由注册写在同一行，漏填即编译不过。`xtask configdoc` 的断言点与失败语义不变：每个 `/api/v1/` 路由都能解析到一对能力域与动作类别，缺失即构建失败，`ci-probe` feature 门控的探针路由与 `/internal/v1/` 下不对四端暴露的内部端点不参与判定。本阶段全部路由落在 `/api/v1/platform/` 与 `/api/v1/portal/` 两段，其中 `/api/v1/platform/` 下路由的能力域按 A-20 一律取 `CapabilityDomain::PlatformAdminLowcodeOps`，第 5.8 节 `/api/v1/portal/` 下的三个路由取 `CapabilityDomain::PortalSupplierWeb`。第 4.1 节的 Action 六值与 ActionClass 五值是两个不同的东西：前者是权限项的动作粒度，参与授权判定阶段二；后者是客户端能力矩阵的动作类别，由阶段 13 的能力矩阵闸使用，两者按 View 对 Read、Create 与 Update 对 Write、Submit 对 Submit、Approve 对 Approve、Export 对 Export 映射。

#### 5.1 认证前端点的头豁免

认证头豁免采用编译期常量 `PRE_AUTH_ENDPOINTS` 的固定四项逐头矩阵，不按路径前缀或运行配置放宽：内部 sign-in、complete-mfa 与门户 sign-in 豁免 `Authorization`、`X-Legal-Entity-Id`、`Idempotency-Key`；已认证的 legal-entities 列表只豁免 `X-Legal-Entity-Id`。其他头照旧必填。complete-mfa 以 `reauth_challenges.token_hash` 的一次性条件更新防重放，不把新会话令牌写进通用幂等响应缓存；首次响应在网络中断后不可重放，客户端必须重新登录并取得新挑战，这是不落盘会话令牌的安全取舍。三个认证前写端点采用登录名或挑战主体加来源地址的双维度速率限制，超限返回 HTTP 429 与 `PLATFORM.AUTHN.RATE_LIMITED`；legal-entities 仍受已认证会话的普通请求闸门。四项之外不得新增匿名或免法人端点，矩阵已同步进基线第 5.4、5.6 节。

#### 5.2 会话与身份

| 方法与路径 | 请求 | 响应 data | 主要错误码 | 幂等 | 权限 |
|---|---|---|---|---|---|
| POST /api/v1/platform/sessions/actions/sign-in | login_name、credential（kind 与 material）、client、device_id | session_token、expires_at、idle_expires_at、user_id、default_legal_entity_id；需 MFA 时 session_token 三项为 null，另返回 32-byte base64url `mfa_challenge`，X509 路径再返回 exact `x509_challenge` | PLATFORM.AUTHN.CREDENTIAL_INVALID、PLATFORM.AUTHN.ACCOUNT_LOCKED、PLATFORM.AUTHN.ACCOUNT_INACTIVE、PLATFORM.AUTHN.MFA_REQUIRED、PLATFORM.AUTHN.DEVICE_NOT_REGISTERED | 豁免幂等键，按第 5.1 节 | 匿名 |
| POST /api/v1/platform/sessions/actions/complete-mfa | mfa_challenge（32 字节随机原文的 base64url）、credential；X509 credential exact 为 `{kind:"X509_CERT",credential_handle_b64url,signature_cms_b64url}` | 成功同上且不再返回挑战 | PLATFORM.AUTHN.MFA_INVALID、PLATFORM.AUTHN.MFA_CHALLENGE_EXPIRED、PLATFORM.AUTHN.MFA_CHALLENGE_CONSUMED | 豁免幂等键；挑战令牌只能条件消费一次，网络丢响应后须重新登录 | 匿名加挑战绑定 |
| POST /api/v1/platform/sessions/actions/sign-out | 无体 | null | — | 幂等键必填，重复返回同一结果 | 本人会话 |
| GET /api/v1/platform/identity/me | — | 用户资料、当前法人、角色码、职责类、密级 | — | — | 已认证 |
| GET /api/v1/platform/identity/me/legal-entities | — | 授权法人清单，逐法人探测得到 | — | — | 已认证，豁免 X-Legal-Entity-Id |
| GET /api/v1/platform/sessions | filter[user_id]、filter[client] | 会话列表，令牌不返回 | — | — | SECURITY 或 SYSTEM 职责 |
| POST /api/v1/platform/sessions/{id}/actions/revoke | reason | null | PLATFORM.AUTHZ.NOT_FOUND_OR_DENIED | 幂等键必填 | SECURITY 职责或本人 |

GET /api/v1/platform/identity/me/legal-entities 的实现按基线第 3.8 节：取该部署已安装的法人清单，逐个设置 app.legal_entity_id 后探测 user_legal_entity_grants 是否可见，可见即授权。2 个法人下为 2 次探测。不使用 OR 展开法人列表，也不使用任何绕过策略的角色。

#### 5.3 凭据、多因子与设备

| 方法与路径 | 语义 | 权限与幂等 |
|---|---|---|
| POST /api/v1/platform/mfa-enrollments/actions/begin | 发起 TOTP 或 WebAuthn 注册，返回注册挑战 | 本人；幂等键必填 |
| POST /api/v1/platform/mfa-enrollments/actions/complete | 完成注册，写入 user_credentials | 本人；幂等键必填 |
| DELETE /api/v1/platform/mfa-enrollments/{id} | 注销一个因子 | 本人加重新认证；剩余因子数为零且 is_mfa_required 为真时返回 PLATFORM.AUTHN.MFA_LAST_FACTOR_FORBIDDEN |
| POST /api/v1/platform/devices | 设备登记，桌面端提交设备证书或 WebAuthn 证明，移动端提交硬件绑定凭据的证明 | 本人；幂等键必填 |
| POST /api/v1/platform/devices/{id}/actions/revoke | 远程注销，级联撤销该设备全部会话 | 本人或 SECURITY 职责 |
| POST /api/v1/platform/user-accounts/{id}/actions/reset-password | 口令重置 | SYSTEM 职责加重新认证；任何许可状态下可用 |

TOTP 唯一参数为 RFC 6238 compatibility profile：160-bit CSPRNG seed、HMAC-SHA-1、6 digits、30-second step，验证窗口由 `EP__AUTH__TOTP__SKEW_STEPS=1` 形成 current±1 三个 counter；同一 credential/counter 首次成功后记录 `last_used_counter` 并拒绝重放。begin 只把 seed 以一次性 enrollment challenge 内的 `otpauth` 表示返回当前已认证客户端，不写正式 credential；complete 必须在 5 分钟内验证一个 code，成功事务才写上述 EPC1 与 counter，失败/过期零 credential 写入。日志、审计、错误与 recovery code 均不得含 seed、URI、code 或其摘要。

#### 5.4 账号生命周期

对应 PRD 第 10.2.3 节四行。

| 方法与路径 | 输入 | 处理 | 错误码 |
|---|---|---|---|
| POST /api/v1/platform/user-accounts/actions/import-batch | 用户清单，单次上限 200 行 | 第一遍完成模板、格式、必填、枚举和批内重复等静态校验，任一失败则零写入；全部通过后，第二遍逐行以独立事务执行动态业务校验与幂等写入，动态失败只回滚该行 | PLATFORM.USER_ACCOUNT.BATCH_PARTIAL_FAILED，details 逐行给出 |
| POST /api/v1/platform/user-accounts/{id}/actions/activate | 生效日期、初始角色 | 校验口令复杂度策略与 MFA 要求 | PLATFORM.USER_ACCOUNT.MFA_ENROLLMENT_REQUIRED |
| POST /api/v1/platform/user-accounts/{id}/actions/transfer | 新部门、新岗位、新角色、生效日期 | 校验职责分离；校验该用户名下未结束的审批待办 | PLATFORM.SOD.DUTY_CONFLICT、PLATFORM.USER_ACCOUNT.PENDING_APPROVAL_TASKS |
| POST /api/v1/platform/user-accounts/{id}/actions/deactivate | 停用日期 | 即时生效，撤销全部会话与设备凭据，发出 platform.user_account.deactivated.v1 | — |

批量建号按 F-51 U-A-09 固定为两遍语义：第一遍静态校验失败时整个文件零写入；只有第一遍全部通过，第二遍才允许逐行独立提交，第二遍的并发变化或动态冲突只使对应行失败。响应必须同时给出文件级错误与逐行结果，禁止把第一遍静态错误降格为第二遍的部分失败。

#### 5.5 授权配置

roles、role-permission-grants、access-policies、field-permissions、sod-rules、approval-chains、user-role-grants、user-org-assignments、user-scope-grants、user-legal-entity-grants 十组资源，各自提供 GET 列表、GET 单条、POST 新建、PATCH 修改、POST {id}/actions/retire。全部要求 SECURITY 职责，user-role-grants 与 user-org-assignments 的新建要求 SYSTEM 发起加 SECURITY 确认两步，对应 PRD 第 10.2.3 节调岗行的“系统管理员发起，安全管理员确认”。敏感字段登记按 C-06 不再是本阶段的配置资源：唯一登记表是 platform_core.sensitive_field_registry，由阶段 2 建立，登记行由各模块阶段以 backfill 迁移写入，只读端点 GET /api/v1/platform/sensitive-fields 同按 C-06 由阶段 2 交付，契约以阶段 2 计划为准，本阶段不注册该路由、不另写契约、不提供任何写入端点，配置界面查阅时直接调用阶段 2 的该端点。

保存期校验按 PRD 第 10.2.2 节四条逐条实现并各有错误码：PLATFORM.SOD.DUTY_CONFLICT、PLATFORM.AUTHZ.ISOLATION_CONTROL_FORBIDDEN、PLATFORM.AUTHZ.DIRECT_DB_ACCESS_FORBIDDEN、PLATFORM.SOD.SELF_APPROVAL_FORBIDDEN。第二条与第三条的实现方式是 permission_items 中根本不存在“关闭或修改法人隔离机制”与“事务业务库直连”这两类权限项，任何引用未注册权限项的授予在保存时按 VALIDATION 拒绝，错误码 PLATFORM.AUTHZ.PERMISSION_ITEM_UNKNOWN。

| 方法与路径 | 语义 |
|---|---|
| POST /api/v1/platform/authz-config-versions/actions/validate | 对当前草稿集合执行全部静态校验，返回违规清单 |
| POST /api/v1/platform/authz-config-versions/actions/stage-for-release | 生成配置包并交阶段 3b 按 A-27 交付的配置发布通道，返回 release_bundle_ref；配置包条目按 A-19 的 AUTHZ_ROLE、AUTHZ_POLICY、AUTHZ_FIELD_GRANT 三个 item_kind 组织。本行属开篇同批清单第二项，随阶段 3b 一并交付，本阶段不注册该路由 |
| GET /api/v1/platform/authz-config-versions/{id}/diff | 与当前生效版本的差异 |
| POST /api/v1/platform/authz-config-versions/{id}/actions/activate | 由阶段 3b 的配置发布通道在签名通过后回调，切换生效版本并发出 platform.authz_policy.published.v1。本行属开篇同批清单第二项，随阶段 3b 一并交付，本阶段不注册该路由，也不提供任何不经该通道的生效路径；本阶段运行期的唯一生效版本由第 3.5 节 27 号种子迁移写入 |

#### 5.6 判定与字段视图

| 方法与路径 | 语义 | 权限 |
|---|---|---|
| POST /api/v1/platform/authz-decisions/actions/evaluate | 对给定 (user_id, object_type, object_id, action) 返回判定结论与命中的规则链 | SECURITY 或 AUDIT 职责；仅当 EP__AUTHZ__DECISION__EXPLAIN_ENABLED 为真时返回规则链 |
| GET /api/v1/platform/field-views/{object_type}/{object_id} | 返回该对象按字段权限与密级裁剪后的投影 | 该对象的 View 权限 |

字段视图端点是规格附录 A.1 度量项“字段级受控只读视图加载”的被测端点，需在基准数据集上给出 EXPLAIN 证据且无顺序扫描。

#### 5.7 高风险操作

| 方法与路径 | 语义 | 幂等与权限 |
|---|---|---|
| POST /api/v1/platform/reauth-challenges | 按操作类型与业务命令载荷发起挑战，服务端计算 subject_digest 并返回摘要展示结构 | 幂等键必填；该对象的 Submit 权限 |
| POST /api/v1/platform/reauth-challenges/{id}/actions/verify | 提交认证材料，通过后在响应头返回 X-Reauth-Token | 幂等键必填；挑战主体本人 |
| POST /api/v1/platform/high-risk-requests | 建立请求单，状态 PENDING_INITIATION | 幂等键必填 |
| POST /api/v1/platform/high-risk-requests/{id}/actions/submit | 消费 X-Reauth-Token，重算摘要比对，建立审批实例。属开篇同批清单第一项，随阶段 3b 一并交付，本阶段不注册该路由 | 幂等键必填；X-Reauth-Token 必填 |
| POST /api/v1/platform/high-risk-requests/{id}/actions/approve | 节点审批通过。属开篇同批清单第一项，随阶段 3b 一并交付 | 幂等键必填；该节点的 Approve 权限；发起人调用直接拒绝 |
| POST /api/v1/platform/high-risk-requests/{id}/actions/reject | 节点驳回，reason 必填。属开篇同批清单第一项，随阶段 3b 一并交付 | 同上 |
| POST /api/v1/platform/high-risk-requests/{id}/actions/withdraw | 发起人撤回。属开篇同批清单第一项，随阶段 3b 一并交付 | 首节点未结论 |
| GET /api/v1/platform/high-risk-requests | 列表，默认筛选最近 3 个自然月 | 记录级范围裁剪 |

移动端限制的执行点从 submit 端点前移到 POST /api/v1/platform/reauth-challenges：X-Client 为 ios 或 android 且 operation_type 属于 Payment、InvoiceIssue、LedgerPosting、PeriodClose、DataMigration 五类时，返回 PLATFORM.HIGH_RISK_REQUEST.CLIENT_NOT_ALLOWED 并在 advice 中说明该操作在桌面端完成，对应规格第 6.2 章矩阵、PRD 第 10.3.1 节移动端列与阶段 14 数据迁移专用规则。ContractEffective 与 SensitiveExport 在移动端可发起。前移的理由有两条：这五类操作在移动端连挑战都不该签发，拒绝点越早越好；下游 submit/专用流程端点晚于本阶段交付，把判定只留在那里会使端别约束在统一挑战入口失守。阶段 14 的五类迁移批准动作还必须在各自业务端点再次拒绝 ios/android，不能只依赖本行。

#### 5.8 应急账号与门户

| 方法与路径 | 语义 |
|---|---|
| POST /api/v1/platform/breakglass-activations | 提交启用申请，reason 必填 |
| POST /api/v1/platform/breakglass-activations/{id}/actions/approve | 独立复核人批准，写入 approval_ref，账号进入 ACTIVE |
| POST /api/v1/platform/breakglass-activations/{id}/actions/close | 主动结束，触发凭据轮换 |
| POST /api/v1/portal/sessions/actions/sign-in | 门户账号登录，account_kind 必须为 PORTAL |
| POST /api/v1/portal/sessions/actions/sign-out | |
| GET /api/v1/portal/identity/me | 返回门户账号的最小身份投影，不含内部角色字段 |

门户账号在授权阶段二有一道额外闸门：其可用角色必须 is_portal_role 为真，任何内部角色的授予在保存时拒绝，错误码 PLATFORM.AUTHZ.INTERNAL_ROLE_FOR_PORTAL_FORBIDDEN，对应 PRD 第 4.9.2 节与第 4.10 节。

上述三个门户身份端点由 portal-gateway 以 `NT SERVICE\ep-portal` 经固定 `\\.\pipe\ep-core` 转发，operation 逐项固定为 `portal.session.sign_in.v1`、`portal.session.sign_out.v1`、`portal.identity.me.v1`。core-server 的管道 DACL、客户端服务 SID 与逐项 allowlist 只允许该账户调用这三项，其他账户、该账户调用其他身份 operation、通配或同义 operation 均拒绝并审计；portal-gateway 不直连 core-server:8080，不持数据库凭据，也不配置 core API URL。管道请求只含不透明 session token、requested_legal_entity_id、device_id、request_id 四项未受信输入，不能携带主体、角色、供应商、职责、范围或 client；core 只从经核验管道账户固定 `ClientKind::Portal`，重新校验 token、`account_kind=PORTAL`、设备、供应商绑定和授权法人后自行构造 `SecurityContext`。阶段 7 的五项业务能力另有八个具名 operation，不得把本段三项算入“五项能力”或用 `portal.*` 代替精确白名单。

#### 5.9 存在性泄漏的统一处理

照抄基线第 5.5 节：对当前安全上下文不可见的记录，读写删一律返回 404 与 PLATFORM.AUTHZ.NOT_FOUND_OR_DENIED；只有对该对象类型完全无权时返回 403 与 PLATFORM.AUTHZ.OBJECT_FORBIDDEN。本阶段追加一条实现纪律：仓储层的“按 ID 取行”返回 Option，用例把 None 与记录级判定失败映射到同一个错误分支，两条路径共用同一段构造代码，禁止分别构造，避免两条路径的响应体在字段顺序或耗时上出现差异。

---

### 6. 并发与事务边界

#### 6.1 事务清单

| 用例 | 事务内写入 | 隔离级别 | 锁策略 |
|---|---|---|---|
| 登录 | sessions 插入、login_attempts 插入、account_lockouts 更新、被挤下线会话的撤销、审计事件 | READ COMMITTED | account_lockouts 行 FOR UPDATE，lock_timeout 3 秒 |
| 登录失败 | login_attempts 插入、account_lockouts 更新、审计事件 | READ COMMITTED | 同上；以 Ok(Rejected) 提交，不回滚 |
| 完成多因子 | user_credentials 的 sign_count 与 last_used_at 更新、sessions 插入、审计 | READ COMMITTED | user_credentials 行 FOR UPDATE 防重放 |
| 登出与会话撤销 | sessions 更新、审计 | READ COMMITTED | 乐观锁 row_version |
| 设备登记与注销 | user_devices 写、级联 sessions 撤销、审计 | READ COMMITTED | 乐观锁 |
| 账号生命周期四操作 | user_accounts 更新、user_role_grants 与 user_org_assignments 写、幂等 finish、Outbox、审计终结批 | READ COMMITTED | 乐观锁；transfer 需先读该用户未结束审批待办 |
| 授权配置保存 | 对应配置表写、authz_config_versions 更新、审计 | READ COMMITTED | 乐观锁；同一 version 内的并发编辑靠 row_version 冲突暴露 |
| 配置版本生效 | authz_config_versions 更新、幂等 finish、Outbox、审计终结批 | READ COMMITTED | 对该法人的当前生效版本行 FOR UPDATE，保证同法人同时只有一个 EFFECTIVE |
| 发起重新认证挑战 | reauth_challenges 插入、审计 | READ COMMITTED | 无 |
| 核销挑战 | reauth_challenges 条件更新、审计 | READ COMMITTED | 条件更新即锁 |
| 提交高风险请求 | high_risk_requests 更新、reauth_challenges 消费、审批实例建立（经流程引擎在同一事务内）、幂等 finish、Outbox、审计终结批 | READ COMMITTED | high_risk_requests 乐观锁；挑战条件更新 |
| 审批节点通过或驳回 | 审批实例推进、high_risk_requests 更新、幂等 finish、Outbox、审计终结批 | READ COMMITTED | 乐观锁；节点推进由流程引擎的行锁保证 |
| 应急账号启用与关闭 | breakglass_activations 更新、user_credentials 轮换、sessions 撤销、幂等 finish、Outbox、同事务通知命令、审计终结批 | READ COMMITTED | 乐观锁 |
| 越权测试与内部对账的只读遍历 | 无写入 | REPEATABLE READ 单事务 | 只读 |

全部写事务遵守基线第 10.3 节：一个用例一个事务，事务内不做外部调用、不读写文件正文、不直接投递外部通知、不等待用户输入；表中的“同事务通知命令”仅指供提交后异步投递的持久化命令。审批实例的建立必须与 high_risk_requests 的状态迁移同事务，否则会出现“请求单已提交但审批实例不存在”的悬挂态；这一点是对流程引擎的接口要求：审批实例的建立必须接受调用方传入的事务句柄，不得自行开事务。流程引擎本体由阶段 3b 交付，落在本阶段之后，因此本表中提交高风险请求与审批节点通过或驳回两行属开篇同批清单第一项，其事务边界在本阶段只作为对流程引擎的接口约束冻结，不在本阶段实现，也不接线任何替身。审计事件同理：platform_audit.audit_events 由阶段 3b 建立，本表各行中的审计事件一栏属同批清单第三项，本阶段不注入任何替身审计端口，装配期缺实现即拒绝启动，不存在审计端口返回成功而实际没有落库这一形态。

所有写用例采用同一收口顺序：先按该用例既定引用顺序完成业务事实、子账/凭证与同步投影（没有的类别跳过），再执行幂等 `finish`，再写 Outbox，再写确需同事务落库的通知命令，最后调用 `AuditWriter::append_terminal` 批量落审计。`append_terminal` 必须封印传入的 `Tx`；其后任何 SQL `query/execute`、仓储写入或跨模块端口调用都以内部不变量错误拒绝并令整个事务回滚，唯一允许的后继动作是 `UnitOfWork::commit`。共享跨模块契约测试 `audit_terminal_seals_tx` 必须覆盖本阶段至少一个本地仓储和一个流程/通知端口：审计后故意发起数据库写与端口写均失败、审计后零新增行、事务整体回滚，并由 recording transaction 断言正常路径的审计批是 commit 前最后一批数据库执行。

#### 6.2 幂等

全部写端点按基线第 5.4 节使用 Idempotency-Key，作用域为法人、用户、端点、键值四元组，幂等键写入与业务写入同事务。认证前只有内部 sign-in、complete-mfa 与门户 sign-in 三项豁免并改用速率限制；complete-mfa 另由一次性挑战的条件消费防重放，绝不把会话令牌原文存入通用幂等缓存。F-55 另冻结一个不属于认证前矩阵的已认证例外：`POST /api/v1/platform/mcp-human-grants/actions/issue` 因一次性明文 token 禁止 `Idempotency-Key` 且不写通用 response cache，携带该头用 `PLATFORM.REQUEST.INVALID_PAYLOAD` 拒绝；它仍必须通过 Authorization、法人、CSRF、设备与权限检查，不能类推到其他写 API。

重新认证令牌本身是第二重幂等：同一个 X-Reauth-Token 只能消费一次，因此即使幂等键被客户端误用为新值，重复提交仍然在挑战消费处失败并返回 PLATFORM.REAUTH.TOKEN_ALREADY_CONSUMED。

#### 6.3 与 Outbox 的关系

本阶段发出五个事件，全部在业务事务内写入 platform_msg.outbox_events，信封字段按基线第 6.1 节。平台事件的 posting_date 与 accounting_period_id 取 null，因为它们不是账务事件。关账受理前提二的判定语句按 C-28 由阶段 9a 定死，本阶段逐字采用：该法人该期间内，platform_msg.outbox_events 中 status 属于 PENDING 或 DISPATCHING、posting_date 落在该期间起止之间、且 event_type 命中 ledger.posting_trigger_event_types 的条目数为零，且 platform_msg.dead_letters 中 state 属于 OPEN 或 REPAIRING、同样命中该注册表的条数为零。posting_date 为空的平台事件一律不计入，理由是它们不产生凭证；本阶段五个事件不在阶段 9a 按 A-21 一次写入的 13 行种子之内，本阶段也不追加任何回填迁移，因此不会误拦关账。

本阶段不消费任何事件。原先由 job-worker 消费 platform.authz_policy.published.v1 再经进程间接口通知 core-server 重建快照这条链已按第 2.3 节整条删除，快照重载改为 core-server 自身轮询 authz_config_versions，因此本阶段不使用 platform_msg.inbox_consumptions，也不产生与之相关的死信与重投路径。该事件的唯一消费方是规格第 7.9 章的派生存储重建，由派生存储所属阶段承接。

#### 6.4 失败重试与补偿

序列化失败 40001 与死锁 40P01 由数据访问层重试 3 次，退避 50、150、450 毫秒，照抄基线第 8.4 节。登录用例可安全重试，因为它在重试前未产生任何外部可见副作用。挑战核销与高风险请求提交不可重试，因为它们的条件更新一旦成功即产生外部可见状态，重试由客户端按幂等键发起。

补偿路径只有一处：高风险请求进入 APPROVED 后，业务模块执行失败时不回滚审批结论，而是由业务模块写入执行失败并把请求单留在 APPROVED，由人工重试或重新发起。理由是审批是一个已经发生的事实，用补偿把它抹掉会让审计证据与实际发生的审批过程不一致。

#### 6.5 并发准入

最近 60 秒内有请求的不同用户数由 core-server 统计，内部与门户共用一个口径。超过 20 人仍照常受理登录与写入，不设用户准入信号量和等待队列；系统记录 `ep_authn_active_users`、发出规模超限告警，并把超限区间写入运维中心的 SLA 不适用记录。管理端查询每 5 秒刷新。单用户最多 3 个有效会话，第 4 个会话建立时在同一事务撤销最早会话。HTTP 层按瞬时请求数设置的资源保护属于阶段 1 的独立闸门，不得复用本节的“活跃用户数”作为拒绝条件。

---

### 7. 配置项

全部键在 `EP__AUTH`、`EP__AUTHZ` 两个前缀下，结构体开启 `deny_unknown_fields`。除注明外一律启动时生效，理由是把安全参数做成热生效开关会在运行期制造一个不经配置发布通道的旁路。活跃用户 20 人、60 秒窗口与管理端 5 秒刷新均为首版冻结常量，不设可形成另一套准入口径的 `EP__ADMISSION` 配置。

| 键 | 类型 | 默认值 | 生效方式 | 说明 |
|---|---|---|---|---|
| EP__AUTH__PASSWORD__MIN_LENGTH | u8 | 12 | 启动 | U-B-14 冻结取值 |
| EP__AUTH__PASSWORD__MIN_CHAR_CLASSES | u8 | 3 | 启动 | 大写、小写、数字、符号中取三类 |
| EP__AUTH__PASSWORD__MAX_AGE_DAYS | u16 | 90 | 启动 | 0 表示不过期 |
| EP__AUTH__PASSWORD__HISTORY_SIZE | u8 | 5 | 启动 | 与 user_password_history 配合 |
| EP__AUTH__PASSWORD__ARGON2__MEMORY_KIB | u32 | 65536 | 启动 | 单次校验约 64 MB |
| EP__AUTH__PASSWORD__ARGON2__ITERATIONS | u32 | 3 | 启动 | |
| EP__AUTH__PASSWORD__ARGON2__PARALLELISM | u32 | 1 | 启动 | 单机配额下不并行 |
| EP__AUTH__LOCKOUT__MAX_FAILURES | u8 | 5 | 启动 | U-B-14 冻结取值 |
| EP__AUTH__LOCKOUT__WINDOW_SECONDS | u32 | 900 | 启动 | |
| EP__AUTH__LOCKOUT__DURATION_SECONDS | u32 | 1800 | 启动 | |
| EP__AUTH__SESSION__TTL_SECONDS | u32 | 28800 | 启动，只影响新会话 | 基线第 11.6 节 8 小时 |
| EP__AUTH__SESSION__IDLE_TIMEOUT_SECONDS | u32 | 1800 | 启动，只影响新会话 | 基线第 11.6 节 30 分钟 |
| EP__AUTH__SESSION__MAX_PER_USER | u8 | 3 | 启动 | 基线第 11.6 节 |
| EP__AUTH__SESSION__SLIDING_WRITE_GRANULARITY_SECONDS | u32 | 60 | 启动 | 滑动续期的写合并粒度 |
| EP__AUTH__REAUTH__TTL_SECONDS | u32 | 300 | 启动 | 基线第 5.6 节 5 分钟 |
| EP__AUTH__REAUTH__MAX_FAILURES | u8 | 3 | 启动 | 达阈值锁定账号 |
| EP__AUTH__TOTP__SKEW_STEPS | u8 | 1 | 启动 | 前后各一个 30 秒窗 |
| EP__AUTH__WEBAUTHN__RP_ID | String | 无默认，必填 | 启动 | 缺失即启动自检失败 |
| EP__AUTH__WEBAUTHN__ORIGINS | Vec\<String\> | 无默认，必填 | 启动 | |
| EP__AUTH__X509__TRUST_ANCHOR_REF | String | 无默认 | 启动 | 形如 secret://pki/client_ca#1 |
| EP__AUTH__BREAKGLASS__MAX_SESSION_SECONDS | u32 | 28800 | 启动 | 规格第 12.1 章 8 小时上限，配置只能调小 |
| EP__AUTH__BREAKGLASS__IDLE_ROTATION_DAYS | u16 | 365 | 启动 | 规格第 12.1 章 12 个月 |
| EP__AUTHZ__SNAPSHOT__POLL_INTERVAL_MS | u32 | 2000 | 启动 | core-server 轮询 authz_config_versions 的间隔，快照重载的唯一路径 |
| EP__AUTHZ__DECISION__EXPLAIN_ENABLED | bool | false | 启动 | 为真时判定端点返回命中规则链 |
| EP__AUTHZ__SCOPE__MAX_DEPARTMENT_DEPTH | u8 | 8 | 启动 | U-B-09 冻结取值 |
| EP__AUTHZ__SCOPE__IN_LIST_THRESHOLD | u16 | 200 | 启动 | 超过阈值改用 EXISTS 子查询 |
| EP__AUTHZ__EXPORT__SENSITIVE_ROW_THRESHOLD | u32 | 1000 | 启动 | U-B-18 冻结取值 |

敏感取值一律不进配置文件：X.509 登录 trust bundle 以严格 `SecretRef` 经 `SecretProvider/SecretUnsealer` 读取；TOTP 没有部署级“主密钥”或 secret:// seed，逐 credential seed 只存上文法人 FIELD/L40 EPC1；会话令牌是不透明 CSPRNG 随机值，不需要可配置密钥。所有短时业务秘密在内存中用 secrecy/zeroizing 容器包装。

启动自检新增一个命名项，按 C-25 以注册名标识而不用序号，注册顺序排在基线第 7.3 节的十项命名项之后（原写的十三项为已作废的旧口径，见 00c 裁定 C-25 与阶段 1 计划第 7.3 节回写）：

- authz-snapshot-loadable：每个法人存在至少一个 EFFECTIVE 的 authz_config_versions，且据其配置行可构造出完整的 AuthzSnapshot。判据只到可构造为止，构造不出即以退出码 78 退出，理由是无快照则任何授权判定都做不了，全拒等价于停机、放行等于灾难，没有第三条路，这一项必须留在阻断级。

checksum 与配置行重算值是否一致不进阻断判据。不一致时进程不退出，改为回退到上一版 EFFECTIVE 快照、经阶段 2 已交付的 DegradationLedger 开一个 kind 取 AUTHZ_SNAPSHOT_CHECKSUM_MISMATCH 的降级窗口并持续告警；该窗口的 scope_legal_entity_id 取校验和不符的那个法人，subject 列不填，本窗口不指向任何端口。阶段 2 首次建表与当时 Rust `DegradationKind`/contract 只含三项；F-55 终态裁定要求 Stage 14a 把 Rust 枚举/contract 扩为 21 项，并由 `V20261023092500__platform_ops_harden_backup_evidence_graph.sql` 把数据库 kind CHECK 从三项扩为同序 21 项。`AUTHZ_SNAPSHOT_CHECKSUM_MISMATCH` 属终态 21 项，但不在阶段 2 初始三项内；本阶段只实现调用与状态后果，不自建第二套枚举、不新增取值，也不得改写 `V20260901104500__platform_ops_create_degradation_windows.sql`。本阶段测试以可观察 mock/contract 断言 checksum 失配时发出正确的 `open` 调用；真实 PostgreSQL 插入、读取、自动关窗与 CHECK 接纳验收必须等到 `V20261023092500` 已应用后由 Stage 14 台账集成测试执行。运行期后果写明：授权配置的新版本不生效，判定按上一版快照执行，直到人工修复后关窗。

原先登记的 duty-class-exclusivity 与 forbidden-permission-items-absent 两项删除。两者判读的都是数据库里的业务行，而这台服务器只有一台、没有备节点，把数据一致性校验做成启动硬失败等于把一处配置错误放大成全企业停摆，此时唯一可行的恢复动作恰是这些校验存在的理由所要禁止的手工改库。两项各有更早的落点：duty-class-exclusivity 下沉到角色授予与用户绑定两条写入路径，由第 4.5 节的同一份纯函数在保存期与运行期各判一次，规格第 12.2 章五类管理员职责分离的承载不变；forbidden-permission-items-absent 下沉为表 3-10 的 ck_permission_items_forbidden_codes 约束，禁止的东西写不进去，不必等下一次重启才发现。

该项失败以退出码 78 退出，--check 模式一并执行；--check 模式下降级窗口按非零退出处理，即闸门留在部署与升级前置，不留在进程启动。

---

### 8. 测试计划

#### 8.1 单元测试

在 ep-platform-authz 与 ep-platform-identity 内，不触库不触网不取真实时间。

判定流水线分支：法人未授权、对象无权、对象显式拒绝与允许并存时拒绝优先、记录范围为 All、为空、按部门命中、按项目命中、按客户命中、按显式共享命中、范围绑定未登记、密级高于用户许可、字段 HIDDEN、字段 MASKED 的三种掩码风格、字段 READ 时写入被拒、部门闭包深度超限截断、部门集合超阈值退化为 EXISTS。

状态机分支：AccountStatus 的全部合法迁移与全部非法迁移各一条；SessionState 的过期、空闲过期、被挤下线、主动撤销；ReauthState 十一条迁移中属于挑战的六条；HighRiskRequestState 的十一态与 PRD 第 10.3.3 节列出的全部迁移逐条，另加五条非法迁移（跳过审批直接执行、驳回后直接执行、撤回在首节点结论后、非发起人撤回、非本人核销挑战）。

职责分离与链校验：五类管理员两两组合共 10 对的互斥判定、CONFIG 与 SECURITY 互斥、自审检测在 ROLE 与 POSITION 与 DEPT_MANAGER 三种展开下各一条、节点号空洞、quorum 越界、节点展开为空；`ApprovalScenarioCode` 三十七值逐一命中 `ApprovalDefaultCatalog` 且没有兜底分支，默认角色与本节映射逐项相等。

摘要与掩码：待签摘要的规范化对键顺序、Decimal 表示、空值三类输入稳定；掩码的边界（长度不足 8 位、空串、非 ASCII）；登记为 is_field_encrypted 的字段其 KEEP_LAST_4 取自 `<column_name>_tail` 列且不触发解密，字段权限为 READ 时经解密位点输出、为 MASKED 时不调用解密位点，两条各一例。

领域属性测试（proptest）：任取角色集合与策略集合，若其中存在任一 DENY 命中，则判定结果必为 Deny；任取用户与对象，判定结果对同一输入恒定；任取字段权限集合，投影后的键集合是原键集合的子集且不含任何 HIDDEN 字段。这三条是本阶段对基线第 8.1 节领域属性测试的补充，不占用其五组财务不变量的名额。

#### 8.2 集成测试

使用真实 PostgreSQL 16，每用例独占一库，库名 ep_test_<nanoid>，用例结束删库。

1. 行级策略生效：以法人 A 的会话变量插入 user_legal_entity_grants，切到法人 B 后该行不可见、不可更新、不可删除。
2. 会话变量缺失时默认拒绝：不设置变量时 13 张带法人表的读写全部返回零行或被 WITH CHECK 拒绝。
3. 连接归还清理：取用连接、设置四个变量、归还、再次取用，验证四个变量为空串且预备语句缓存未被清空。
4. 登录成功与失败的事务效果：失败时 login_attempts 与 account_lockouts 均已提交。
5. 锁定与解锁：连续 5 次失败后第 6 次返回 ACCOUNT_LOCKED；locked_until 到期后自动可登录。
6. 会话上限：同一用户建立第 4 个会话时最早的一个被撤销并写审计。
7. 活跃用户规模：25 个不同用户均可成功登录并继续写入；第 21 人起只增加规模超限指标、告警与 SLA 不适用记录，管理端读数在 5 秒内刷新，任何请求都不得因这一计数返回 429 或 503。另以同一用户建立第 4 个会话，断言最早会话在同一事务被撤销。
8. 设备未登记与设备被注销后的访问拒绝。
9. 逐法人探测的授权法人清单：用户被授权 A 未被授权 B 时清单只含 A。
10. 重新认证挑战：摘要不符、过期、重复消费、非本人核销四条拒绝路径各一条；另以真实事务验证 HIGH_RISK_REAUTH 的 VERIFIED→CONSUMED 首次成功返回新 row_version=旧值+1，并发两次消费恰一成功，业务事务在挑战消费后故意失败时整笔回滚、挑战恢复 VERIFIED 且同一幂等请求可再次成功。SIGN_IN_MFA 的 ISSUED→CONSUMED 复用同组断言：首消增版、并发恰一成功、创建会话失败时挑战消费同步回滚。两类所有 UPDATE 均通过 `assert_row_version_bump()`，不得只改状态时间列。
11. 高风险请求：六类业务操作各跑一次从建单到重新认证通过；另以 `DATA_MIGRATION` 跑一次独立挑战的签发、核销、单次消费与旧 token 重放拒绝，不建立 `high_risk_requests` 行。移动端 X-Client 在 POST /api/v1/platform/reauth-challenges 上发起 Payment、InvoiceIssue、LedgerPosting、PeriodClose、DataMigration 五类受限操作均被拒。从六类业务请求的提交到执行一段，以及发起人尝试审批被拒与跳过节点被拒两条，属开篇同批清单第一项，随阶段 3b 一并执行；数据迁移的专用批准闭图归阶段 14 验收。
12. 审批链配置校验：自审配置保存被拒且错误消息含冲突节点号；节点空洞被拒；职责冲突的角色授予被拒；向 role_permission_grants 插入不存在的 permission_item_code 由 `fk_role_permission_grants_permission_item_code` 拒绝，删除仍被 grant 引用的 permission item 由 ON DELETE RESTRICT 拒绝。配置版本发布与模块启用各跑 object scope 闭包校验：合法 binding 通过，缺 binding、指向不存在表、指向不存在锚列三类均以 `PLATFORM.AUTHZ.SCOPE_BINDING_MISSING` 整事务回滚。
13. 配置版本生效：切换版本后判定结果随之改变，且切换前后不存在既非旧版又非新版的中间结论（用并发读验证快照整体替换）。
14. 应急账号：启用需独立复核人；允许的三类操作成功；业务写入、审计策略修改、密钥操作、常规业务审批四类被拒并告警；8 小时到期自动失效；关闭后凭据轮换结果写入台账与审计。
15. 账号停用：停用后会话立即不可用、设备凭据失效、发出 platform.user_account.deactivated.v1。
16. 许可受限运行状态下账号停用、口令重置、凭据轮换、权限回收四项仍可执行，对应规格第 3.4 章；许可状态与唯一受限原因一律经阶段 3b 按 F-56 交付的 `ep_platform_license::ModuleLicenseQuery::license_evaluation` 从同一快照读取，本阶段不自建第二套许可判定。
17. 幂等：同一 Idempotency-Key 重放返回首次结果并带 Idempotent-Replay 头；载荷不同返回 409。
18. Outbox：五个事件的信封字段齐全，security_level 与 data_scope_tags 非空，posting_date 为 null。
19. 快照重载与降级：写入一版新的 EFFECTIVE authz_config_versions 后，core-server 在一个轮询间隔内换上新快照；把该版本的 checksum 改坏后重启，进程不退出、判定沿用上一版快照。本阶段以可观察 mock/contract 断言 `DegradationLedger::open(AUTHZ_SNAPSHOT_CHECKSUM_MISMATCH, ...)` 恰调用一次且作用域为该法人，不声称初始三项数据库已持久化该 kind；Stage 14 在应用 `V20261023092500` 后另以真实 PostgreSQL 断言该窗口可插入、读取并在 checksum 修复后自动关窗。
20. 空库与法人引导：空库执行 35 个迁移后创建两个法人，断言存量回填与新法人事务引导对三十七个场景各有且只有一条活动链、每条至少一个节点；重复执行 backfill 与 `provision_defaults` 均零新增。默认角色尚无自然人时提交返回 `PLATFORM.APPROVAL.NODE_HAS_NO_APPROVER` 且业务、流程、通知、Outbox、审计均零写入，绑定一名非申请人后同一提交可启动流程。
21. 共用解析与原子切换：分别制造无链、数据库约束启用前的两条活动历史脏数据、零节点、角色展开为空、申请人自审及正常链，断言依次返回 `CHAIN_NOT_FOUND`、`ACTIVE_CHAIN_AMBIGUOUS`、`NODE_HAS_NO_APPROVER`、`NODE_HAS_NO_APPROVER`、`SELF_APPROVAL_FORBIDDEN` 与完整快照；并发一百次读与十次换版只能观察旧 digest 或新 digest，绝不观察无活动链、双活动链或混合节点。数据库唯一约束、服务锁顺序与错误码三层均验证。
22. F-56 initial governance 身份可用性：用两个不同客户管理员 leaf 生成 signed bootstrap，断言落库的 X509 `cert-sha256:` verifier、SKI handle、deployment roster entry 与两份 trust bundle 逐字闭合；两名用户各自用 PASSWORD 第一因子和 X509 detached-CMS 第二因子完成 sign-in/complete-mfa。错 DER hash、错 SKI、同一 credential 复用为两因子、错 challenge body、过期 signingTime、登录 bundle digest 漂移、双链、intermediate/leaf CRL 命中均稳定失败且不产生会话；CONFIG operator 提交后自审失败，另一 SECURITY admin 才可批准。

#### 8.3 法人越权测试基线（tests/rls_matrix）

独立测试目标，属发布门禁项，覆盖规格第 7.7 章要求的八类。按 C-05 的三段分工，本阶段承担第三段：CI 目标名 tests/rls_matrix 与 assert_read、assert_write、assert_update、assert_delete、assert_aggregate、assert_sort、assert_report_projection、assert_error_leak 八个断言函数由阶段 1 在 `testkit/src/rls_matrix.rs` 提供，assert_replication_role_containment 与 assert_recon_context_borrow 由阶段 2 追加，本阶段新增 matrix_32.rs 承载 32 组完整矩阵，并交付发布门禁项 RG-RLS-MATRIX-GREEN 的判定，本阶段不实现上述十个同名函数中的任何一个。八类为：

读取、写入、更新、删除、聚合、排序、报表投影、错误信息泄漏。每类在两个法人与两个密级上交叉执行，共 8 类乘 2 法人乘 2 密级共 32 组，每组既有“应可见”正例也有“应不可见”反例。判定标准为不出现越权读取、越权写入与跨法人聚合泄漏。

聚合泄漏的具体判据：跨法人的 count、sum、分面计数在越权上下文下返回 0 或不返回该分面，不得返回真实值；排序泄漏的判据：按无权字段排序的请求返回 VALIDATION，不返回按该字段排好序的结果；错误信息泄漏的判据：对不可见记录的读写删三类请求，响应体与响应时间在“记录不存在”与“记录存在但无权”两种真实情况下不可区分，时间差的 P95 不超过 5 毫秒。

另有五个入口借用测试，对应基线第 8.4 节。按 C-05，两个复制角色两项经阶段 2 追加的 assert_replication_role_containment 判定，内部对账上下文一项经阶段 2 追加的 assert_recon_context_borrow 判定，两个只读角色两项经阶段 1 提供的 assert_read 判定；本阶段不实现其中任何一个函数，只负责把它们编入矩阵与门禁判定：

1. ep_archiver 角色借用测试：验证该角色不具备任何业务表的 SELECT 权限，只有 REPLICATION 属性，且只能本机连接。
2. ep_backuper 角色借用测试：同上。
3. 内部对账系统安全上下文借用测试：验证其只写单一法人的会话变量、不建立跨法人会话、不具备 BYPASSRLS，且该上下文不存在运行期 SQL 入口，即 ReconExecutor 只按 ReconRegistry 中已注册的 ReconCheck 实现分发，不接受语句文本入参。原写法中语句集是封闭的白名单一句作废：校验语句本来就是各 ep-app-* crate 内的编译期常量，完整性由阶段 1 的制品签名链承担，再在数据库表里存一份摘要与签名引用是对同一事实的第二套账，而它声称防住的运行期 SQL 拼接在这个形态下并不存在。
4. ep_analyst_ro 借用测试：验证只读分析池受行级策略约束，不能读到当前法人之外的行。
5. ep_ops_ro 借用测试：验证其只能读运维视图，触碰任一业务表返回权限错误。

这五项的被测对象由其他阶段交付：两个复制角色、ep_analyst_ro 与 ep_ops_ro 由阶段 2 交付，内部对账系统的安全上下文由阶段 9a 的 ep-platform-recon 执行器按 A-06 交付。阶段 9a 排在本阶段之后，第 3 项整条归阶段 9a：本阶段不建假执行器、不跑该项、不做顺延登记，判据与断言函数 assert_recon_context_borrow 已由阶段 2 冻结，阶段 9a 交付执行器后直接编入其退出条件。本阶段的矩阵与门禁只统计其余四项，四项在本阶段即为强制。

#### 8.4 端到端测试

后端 E2E 用 Rust 集成测试直打 HTTP；四端 UI 用 Playwright 驱动桌面 WebView 与门户 Web、tauri-driver 驱动桌面壳、XCUITest 与 Espresso 驱动移动端。

对应规格第 17.2 章“身份与访问控制测试”条目逐句：

1. 应急本地账号：启用经指定审批人批准、单次有效期到期自动失效、使用后凭据轮换，全过程在审计证据中可查；允许的恢复操作成功，业务写入、审计策略修改、密钥管理与销毁、职责分离绕过四类被拒并告警。
2. 员工本地账号目录：口令复杂度与有效期策略生效；管理员与高风险角色强制 MFA；离职停用即时生效。
3. 高风险操作：六类业务高风险逐类验证重新认证、审批链不可越权跳过、申请人不可自审，且认证方式、待签内容摘要、时间与设备四项写入审计证据；`DATA_MIGRATION` 在本阶段验证统一 reauth 底座，在阶段 14 验证专用版本绑定审批证据。四端口径一致，按规格第 6.2 章矩阵，桌面端全量、移动端按“财务过账与期末结账”“收付款登记与对账查看”“发票申请与开具登记”三行及数据迁移专用规则取仅查看，验证移动端不提供提交入口且给出转桌面端说明。
4. 默认审批链开发就绪：从完全空库引导一个法人，逐一枚举三十七个 `ApprovalScenarioCode`，UI 或 API 均不能提交未知场景；每个场景先以空角色验证稳定拒绝，再绑定默认角色的非申请人完成一条顺序审批，最后让申请人兼任审批角色验证自审拒绝。`CONFIG_RELEASE` 与 `EXTENSION_ENABLE` 必须实际解析到 `SECURITY_ADMIN`，所有财务、发票和总账场景必须实际解析到 `FINANCE_MANAGER`，报表场景必须实际解析到 `MANAGEMENT_APPROVER`。

对应规格第 17.3 章强制不变量，本阶段承担其中一条：“权限不能跨法人、字段或密级越权”，由 tests/rls_matrix 全绿判定。其余不变量不属本阶段。

对应 PRD 第 10.2.4 节：列表可见而明细无权时点击进入按权限拒绝处理，不返回部分内容；无权字段不进入返回结果且不显示占位值。

#### 8.5 性能相关项

在 ep-datagen 默认 scale 的基准数据集上，按规格附录 A.2 口径取样不少于 200 次，在同时施加连续归档写出、附件正文增量写出、审计证据写出与一次每日全量备份的条件下测量：

- 字段级受控只读视图加载：规格附录 A.1 常规交互项，P95 不超过 2 秒，附 EXPLAIN 证据且无顺序扫描。
- 审批任务列表加载：常规交互项，同上通过线。本阶段提供高风险请求侧的待办数据，通用待办列表由流程引擎阶段承担，两处的取数路径在本阶段验证不产生顺序扫描。
- 授权判定自身：ep_authz_decision_duration_seconds 的 P95 不超过 1 毫秒，P99 不超过 5 毫秒，作为观察项冻结。
- 登录：不在规格附录 A.1 清单内，本阶段设内部观察项，P95 不超过 1.5 秒，其中 Argon2id 单次校验的目标为 120 毫秒以内，超出时下调 memory 参数并重测。理由是 20 并发下登录风暴会与 app-core.slice 的 CPU 配额直接冲突。

#### 8.6 覆盖率门槛

本阶段全部代码属于平台内核，按基线第 8.2 节适用 85% 行覆盖下限；新增与修改代码不低于 80%；工作区整体不低于 80%。工具为 cargo-llvm-cov，codecov.toml 中为 crates/platform/identity 与 crates/platform/authz 两条路径规则各设 85。不允许长期 #[ignore]。

---

### 9. 退出条件

逐条可客观判定，全部达成才算完成。

1. 35 个迁移文件在空库上按文件版本号全序离线执行成功：29 个既有文件校验和不变，6 个追补文件全部执行；24 张表与 13 条行级策略建立完成，现行双用途重新认证形状、全部具名真实外键（明确包含 `fk_role_permission_grants_permission_item_code`）、SYSTEM_PRINCIPAL 的逐法人授权、审批场景闭集与唯一活动版本约束、默认审批链及供应商绑定单一权威均经 SQL 断言验证；孤儿 permission_item_code 升级负例必须在加约束前失败并输出定位，合法历史数据升级后 FK 生效。本阶段 11 张不带法人列的表在 platform_core.unpoliced_table_registry 中各有一行登记且 db/checks 的第十三项返回零行，platform_core.schema_history 记录完整。
2. core-server 与 job-worker 以 --check 模式退出码为 0，基线第 7.3 节的十项中除 `audit-chain-verifiable`、`file-store-writable` 与 `offsite-sink-requirements` 三项外全部通过，本阶段的 authz-snapshot-loadable 一项亦通过；该三项的承担阶段均晚于本阶段（前两项归阶段 3b，末项归阶段 14），按基线第 12 节通则第六条以换判据处置，在本阶段返回 `NOT_APPLICABLE` 并在报告中标注承担阶段，既不计入通过也不计入违反，不构成本条的阻断项；且 platform_ops.degradation_windows 中没有未关闭的 AUTHZ_SNAPSHOT_CHECKSUM_MISMATCH 窗口。
3. 一条端到端脚本可在干净环境上完成：建号、开通、注册第二因子、登记设备、登录、选择法人、访问一个受保护端点、发起一次合同生效类高风险操作、完成重新认证并取得 X-Reauth-Token，全程无人工干预。脚本止于重新认证通过。提交、两级审批与留下审计证据三段属开篇同批清单，本阶段不以任何替身或假实现跑通，也不在本阶段声称可演示，三段与阶段 3b 一并实现并在阶段 3b 结束时一次判定。
4. 本阶段交付的 matrix_32.rs 的 32 组交叉用例全部通过，入口借用测试在本阶段只有 4 项且全部通过，内部对账上下文一项按第 8.3 节整条归阶段 9a、不计入本阶段判定也不登记顺延，输出的结构化报告中越权读取、越权写入、跨法人聚合泄漏三项计数为零，发布门禁项 RG-RLS-MATRIX-GREEN 判定为绿；该门禁的判据按基线第 3.8 节为 platform_core.unpoliced_table_registry 的行数与 tests/rls_matrix 中承接入口用例数相等且全绿，本阶段交付的 matrix_32.rs 是其中一段。
5. 规格第 17.2 章“身份与访问控制测试”条目的三段判据逐句有对应用例且全部通过。
6. 七类 `HighRiskOperation` 各自的重新认证允许路径与拒绝路径均有用例且通过，拒绝路径至少覆盖：摘要不符、令牌重复消费、非本人核销、挑战过期、移动端发起 Payment、InvoiceIssue、LedgerPosting、PeriodClose、DataMigration 五类受限操作。前六类业务操作的未重新认证提交、发起人自审与跳过节点三条落在通用提交与审批段，属同批清单第一项，随阶段 3b 判定；`DATA_MIGRATION` 的对应三条由阶段 14 专用端点、流程与证据图判定。
7. 五类管理员职责互斥在配置期与运行期各有一条拒绝用例通过；不存在任何一个角色或用户可以同时命中两个互斥职责类，由第 4.5 节的同一份纯函数在角色授予与用户绑定两条写入路径上拒绝，另有一条集成用例断言种子角色包在这两条路径上不产生任何冲突，不再由启动自检在真实种子数据上验证。
8. 权限项注册表中不存在“关闭或修改法人隔离机制”与“事务业务库直连”两类权限项，由 permission_items 上的 ck_permission_items_forbidden_codes 保证其写不进去，并有一条集成用例断言该约束拒绝这两类编码的写入。
9. 字段级受控只读视图端点在基准数据集上 P95 不超过 2 秒，EXPLAIN 输出无 Seq Scan，证据入库到测试证据目录。
10. 授权判定 P95 不超过 1 毫秒，指标可在 127.0.0.1:9101 抓到。
11. crates/platform/identity 与 crates/platform/authz 的行覆盖率均不低于 85%，工作区整体不低于 80%。
12. 依赖方向自检脚本通过：两个新 crate 不出现对 domain、application、adapter 的依赖，ep-platform-authz 不依赖 ep-platform-identity。
13. docs/error-codes.md、docs/event-catalog.md 与数据字典的本阶段增量已提交，CI 的错误码一致性校验与事件登记校验通过。
14. 本阶段的 3 处偏离项与 10 处新增决定已由 F-51/F-52 及现行总览完成书面批准并回写；不存在等待签字的实现分支。
15. clippy 以 -D warnings 通过，非测试代码中不出现 unwrap、expect、panic!、数组越界索引与整数溢出运算；单文件不超过 800 行、函数不超过 50 行、嵌套不超过 4 层。
16. 按 A-19 应交付的三个 applier 已在 ep-platform-authz 实现：AuthzRoleApplier、AuthzPolicyApplier、AuthzFieldGrantApplier，三者实现阶段 3a 提供的 ConfigItemApplier 端口并注册到 ConfigItemApplierRegistry，单元测试覆盖三者的写入与版本推进在同一事务内完成。配置包经发布通道审批签名后生效属开篇同批清单第二项，随阶段 3b 一并判定，本阶段不登记顺延项。
17. 本阶段全部路由在注册处各带一个 `(CapabilityDomain, ActionClass)` 元组，`crates/platform/authz/src/capability.rs` 这个文件不存在，`xtask configdoc` 通过。
18. 本阶段不交付任何业务界面。A-23 的四端界面按规格第 6.2 章能力矩阵由阶段 5 至阶段 12 各自交付，客户端壳、路由注册表与能力矩阵闸由阶段 13 交付，本阶段只交付服务端端点与其契约。
19. 按 A-15 的实现清单，MasterReferenceCounter、SalesTradeHistoryProvider 与 PurchaseTradeHistoryProvider 三个 trait 的实现方不含本阶段，本阶段不实现也不注册，注册表由阶段 5 提供。
20. 敏感字段登记表在本阶段的迁移与建表语句中不存在，本阶段对 platform_core.sensitive_field_registry 只有读取路径，由一条集成用例断言本阶段代码不含对该表的 INSERT、UPDATE 与 DELETE。
21. 第一批 T0 底座可独立验证：开篇所列 15 张表已建，四条链路各有一条用例通过，即口令登录与设备登记、安全上下文建立与 app.legal_entity_id 写入、授权判定第一阶段与第二阶段、ContractEffective 与 InvoiceIssue 两类高风险操作的重新认证与单节点审批链定义的静态校验；ep-datagen 的 T0 最小样本可一次生成 1 个法人、1 个操作员账号、1 个设备、1 个业务角色与 4 条单节点审批链，四条的 scenario 依次为合同生效、发票申请单、销项发票开具与资金账户建档，且该样本不依赖默认 scale 数据集。
22. apps/core-server/src/wiring/ 与 apps/job-worker/src/wiring/ 两个目录下的全部文件中不出现以 Noop、Stub、Fake、Dummy 四类前缀命名的类型，由阶段 1 随 xtask 交付的 archcheck 规则 unwired-absent 断言且在本阶段构建中通过，本阶段不产生任何顺延登记项；开篇同批清单五项在本阶段的退出条件中均不被宣称达成。
23. 三个门户身份端点逐一映射到 `portal.session.sign_in.v1`、`portal.session.sign_out.v1`、`portal.identity.me.v1`，以 `ep-portal` 账户经 `ep-core` 管道的正例全部通过；非 `ep-portal` 账户、未列 operation、通配 allowlist、直连 core-server:8080、自填 core URL、伪法人、内部员工 token、伪 device、自填 client 或主体/角色字段的负例全部被拒绝。

---

### 10. 与规格和 PRD 的对应

#### 10.1 规格条目

| 规格章节 | 本阶段实现的条目 |
|---|---|
| 5.1 平台内核 | RBAC、ABAC、职责分离和审批授权：按法人、部门、岗位、项目、客户、记录和字段判定访问，审批链不可越权跳过，申请人不可自审；首版不含临时授权与策略模拟（本阶段不实现，见第 11 节预留） |
| 6.2 一致性与兼容 | 硬件认证条目：桌面端本机 USB Key 与智能卡、移动端硬件绑定且私钥不可导出的凭据；六类业务高风险与 `DATA_MIGRATION` 运维高风险的重新认证和审计证据要求四端一致，数据迁移移动端只读 |
| 6.3 本地缓存与设备 | 设备须先完成登记才能访问业务数据；远程注销；退出登录与设备注销时清除本地缓存与凭据的服务端触发点 |
| 7.7 法人行级隔离机制 | 统一安全上下文的建立；服务端按用户与设备的授权法人集合校验调用方声明的法人；连接归还前清除安全上下文；法人越权测试集八类；两个复制角色与内部对账上下文的入口借用测试 |
| 7.9 派生存储安全继承 | 权限模型、密级规则变更时发出事件驱动派生存储重建或重新打标（本阶段发事件，重建由派生存储阶段执行） |
| 12.1 身份与认证 | 内置员工本地账号目录、批量建号、入职开通、调岗改权、离职停用；口令复杂度、有效期、失败锁定与 MFA 策略；管理员与高风险角色强制 MFA；七种认证方式；七类 `HighRiskOperation` 的统一重新认证底座，其中数据迁移用阶段 14 专用审批证据；认证方式、待签内容摘要、时间与设备写入审计证据；受控应急本地账号的四要素、允许操作集合、8 小时上限、用后轮换与 12 个月闲置轮换；用户、服务、设备与插件的独立工作负载身份中的用户与设备两类 |
| 12.2 授权 | RBAC 加 ABAC；法人、部门、岗位、项目、客户、记录与字段级权限；职责分离与审批授权；审批人不得与发起人为同一人；策略默认拒绝；不设全能超级管理员；五类管理员职责分离 |
| 12.5 审计 | 本阶段全部安全相关事实写审计事件并与业务变更同事务，含审批、授权变更、重新认证、敏感导出触发、应急账号启用与轮换 |
| 15.1 错误分类 | 权限或策略拒绝提供可理解原因但不泄露无权数据；每条错误含关联编号、发生时间、可否重试与处理建议 |
| 16 性能与容量 | 20 名活跃用户内适用常规交互 P95 2 秒；超过 20 人仍可用但该时段 SLA 不适用；字段级受控只读视图加载纳入同一基线 |
| 17.2 自动化测试 | 身份与访问控制测试整条；单元测试与领域属性测试的覆盖率门槛；集成与契约测试中的平台契约部分 |
| 17.3 强制不变量 | 权限不能跨法人、字段或密级越权 |
| 3.4 订阅许可生命周期 | 账号停用、口令重置、凭据轮换与权限回收在任何许可状态下保持可用，不计为业务写入 |
| 附录 A.1 | 常规交互项“字段级受控只读视图加载”；提交类中六类业务高风险操作与数据迁移运维高风险的重新认证服务端校验往返计入时延 |

#### 10.2 PRD 条目

| PRD 节 | 本阶段实现的功能 |
|---|---|
| 10.1 适用角色与职责分离 | 八类角色的定义与五类管理员互斥；不设全能超级管理员；首版身份来源只有内置目录 |
| 10.2.1 七个判定维度 | 七个维度全部实现，映射进基线第 11.3 节的四阶段顺序；密级作为属性参与过滤 |
| 10.2.2 配置对象与配置操作 | 角色、访问策略、字段权限与密级三类配置对象；权限项粒度为模块功能点加六个动作；四条保存期校验；异常提示指出被拒绝的规则名称；配置进入配置发布流程后生效 |
| 10.2.3 用户账号的生命周期操作 | 批量建号、入职开通、调岗改权、离职停用四行逐行实现，含各自的校验与结果 |
| 10.2.4 权限拒绝的用户可见行为 | 三条全部实现，含排序位次与分面计数不泄露、列表可见而明细无权时不返回部分内容 |
| 10.3.1 清单与触发点 | 六类业务高风险操作的清单、发起角色、审批归属与移动端取值；阶段 14 另追加 `DATA_MIGRATION` 为第七类运维高风险并使用专用审批证据 |
| 10.3.2 重新认证的交互流程 | 五步全部实现，含待签内容摘要的五项内容 |
| 10.3.3 状态机 | 十一个状态与全部流转逐条实现 |
| 10.3.4 异常与失败提示 | 六条全部实现并各有错误码 |
| 4.9.2 门户访问与数据约束 | 门户账号与员工目录分属不同身份来源；门户账号不得被授予内部角色 |
| 4.10 权限与职责分离 | 付款申请提交人不可作为该申请的任一审批节点 |
| 11.2 并发与规模上限 | 20 名活跃用户规模观测、超限不拒绝与单用户三会话上限 |
| 11.7 访问入口约束 | 门户会话与内部会话的分离；门户账号的法人范围限定 |
| 附录乙 U-B-05 至 U-B-18 | 逐条给出冻结取值，见第 12.3 节 |

---

### 11. 风险与预留

#### 11.1 技术风险

1. 四端认证方式的一致性风险。WebAuthn 在 Tauri 桌面壳内的可用性依赖各平台 WebView 的实现，智能卡与 USB Key 在桌面端需经原生插件访问 PKCS#11，而规格第 9.3 章允许客户关闭原生插件加载。缓解：认证方式做成可插拔的 CredentialVerifier，任一方式不可用时按 PRD 第 10.3.4 节提示改用其他已登记方式，且服务端在用户只剩一个可用因子时拒绝注销该因子；四端 PoC 的首测执行按己-3 的裁定由阶段 13 承接，阶段 1 不再产出任何覆盖 USB Key 的证据，原先“阶段 1 的四端 PoC 若未覆盖”这一前提在本阶段恒为真、不构成条件，现删去该前提改为无条件补测：本阶段在第一周内补一次 USB Key 的真机验证，未通过则把 USB Key 降级为可选方式并登记为交付差异。USB Key 属桌面端外设，按规格附录 C.3 第四条，桌面端外设门槛未通过时优先经规格第 9.3 章的桌面端签名原生插件补齐，不触发客户端 UI 技术栈切换，因此本项验证的结论不进附录 C.3 第二条的切栈触发项。
2. Argon2id 与单机 CPU 配额的冲突。65536 KiB 加 3 轮在登录风暴下会短时占满 app-core.slice 的 CPU 份额，进而拖慢在途业务事务。缓解只有一条：按第 8.5 节实测下调 memory 参数直到单次校验落在 120 毫秒以内。登录不另设以活跃用户数为依据的信号量；瞬时请求资源保护沿用阶段 1 的 HTTP 闸门，且不得因活跃用户超过 20 人拒绝正常业务。
3. 记录级谓词下推导致查询计划退化。部门闭包与项目集合进入 WHERE 后，规划器可能放弃 ix_<table>_legal_entity_id_created_at 转为顺序扫描。缓解：设 IN 列表阈值并在超阈值时改用 EXISTS 子查询；本阶段对 platform 自身三个对象类型给出 EXPLAIN 证据，业务对象的证据由其所属阶段在接入时给出，判据写入 object_scope_bindings 的登记流程。
4. 授权快照与配置发布之间的一致性窗口。快照重载是轮询的，从配置版本生效到 core-server 换上新快照之间存在一个不超过 EP__AUTHZ__SNAPSHOT__POLL_INTERVAL_MS 的窗口。缓解：判定结果携带 snapshot_version，审计事件记录该版本号；窗口上限由该配置项界定并作为观察项进指标。原先的事件驱动加进程间接口同步通知加轮询兜底这三件套删除，只留轮询一条，理由见第 2.3 节：三条路径合起来把一个上界确定的小窗口换成了一条会失败、会积死信、要重投的链。该窗口对派生存储的影响按规格第 7.9 章由派生存储阶段承担，不在本阶段闭合。
5. 标准角色包误绑定自然人的风险。F-51 U-B-01/U-B-02 已冻结五个可复制派生的标准业务角色包和八个规范岗位标签；出厂包只定义 RoleCode 与权限，不自动绑定任何自然人。实施向导必须由实施人员把客户岗位显式映射到 RoleCode，禁止按中文岗位名称自动猜测。
6. 身份主体表不带法人列是已批准的现行口径，不存在评审分支。其安全风险是实现者若忘记把“列出用户”的查询与 `platform_authz.user_legal_entity_grants` 内联，会绕过该设计的法人可见性承接点；控制手段是第 12.2 节逐表准入登记、`db/checks/13_unpoliced_registry.sql` 与 `tests/rls_matrix` 的每个 API 出口越权用例，任一遗漏直接阻断构建与发布。
7. 门户账号跨法人范围按 U-B-11 的既有冻结值使用同一套法人授权集合机制；同一身份可被授予多个法人，但每次请求仍只能携带一个法人上下文，任何列表与详情都不得跨法人合并返回。
8. 阶段顺序按裁定通则第四条固定为 1 → 2 → 3a → 4 → 3b-1 → T0 → 5 → 9a → 8 → 6 → 7 → 10 → 11 → 9b → 14 之后，本阶段落在流程引擎、配置发布通道、审计哈希链与通知投递四项本体之前。原先的缓解是注入四个 Noop 前缀空实现并把验收顺延，该做法已废止：行为固定为返回成功且不产生任何副作用的空实现，会让审批放行与审计留痕在本阶段看起来成立，而这两件事恰恰是本阶段安全性最高的演示，把它押在四个静默返回成功的空壳上是最坏的一种形态；一旦某一行替换遗漏或被回退，系统会安静地产出不完整的安全事实，而唯一的守卫是一句注释。现处置是同批交付，开篇已逐项列出五项清单，本阶段不注入任何替身、不登记任何顺延项、不在退出条件中宣称这五项可演示，它们与阶段 3b 一并实现并在阶段 3b 结束时一次判定。残余风险是阶段 3b 的排期直接决定这五项的完成时点，控制手段是 3b-1 批紧接本阶段、两段之间不插入其他阶段，且两段合起来仍排在贯通线 T0 之前；同批清单前四项所依赖的审批流程实例、最小发布通道 Draft 到 Released 的一条直路、审计哈希链与段行、同事务站内通知四者都在 3b-1 批之内，第五项的模块许可状态读取按阶段 3 计划落在 3b-2 批，按其下游拉动点就位，不构成 T0 的前置。
9. 内部对账系统安全上下文借用测试的被测对象按 A-06 由阶段 9a 交付，而 9a 排在本阶段之后。处置是整条移交而非顺延：本阶段不建假执行器、不跑该项、不留登记，判据与断言函数已由阶段 2 冻结，阶段 9a 交付执行器后直接编入其退出条件。这样本阶段少一处假实现，也少一条要跨六个阶段记着的顺延项。

#### 11.2 为后续阶段预留的扩展点

1. 企业身份联合。CredentialVerifier 与 PrincipalResolver 两个 trait 是 AD、LDAP、OIDC 接入的位点；user_accounts 预留 account_kind 枚举的扩展空间。首版不实现、不验收。
2. 临时授权与委托代理。user_role_grants 与 user_scope_grants 已带 effective_from 与 effective_to 两列，恢复该能力时只需放开写入路径与增加一个授予来源列，不需要改判定流水线。
3. 策略模拟与影响分析。POST /api/v1/platform/authz-decisions/actions/evaluate 已具备对任意主体求值的能力，模拟只需在其上加一层“以候选配置版本求值”的入参，判定内核不变。
4. 仓库维度（U-B-10）。若安全负责人决定新增仓库维度，落点是 object_scope_bindings 增加一列 warehouse_col 与 user_scope_grants 的 scope_kind 增加一个取值，但规格第 12.2 章的七个维度必须先修订，PRD 层不得自行增加维度。
5. 破窗授权流程。受控应急本地账号的 allowed_action_set 是一个 text[] 加 CHECK，通用破窗恢复时可放宽该 CHECK，但规格第 12.1 章明确首版不含通用破窗，因此本阶段不预留 API。
6. 字段级加密的覆盖面。解密位点本身不是预留项：按 A-28 首版已有实际加密字段，口径与字段投影路径上的唯一解密位点见第 4.7 节，SensitiveFieldDecryptor 的实现基于阶段 2 在 `ep_foundation::port::kms` 定义的 `KmsBackend`，载体实现留在 ep-adapter-kms，实例由 apps/core-server/src/wiring/ 与 apps/job-worker/src/wiring/ 两个目录下的装配注入，本阶段不自建第二套解封路径。预留的是覆盖面，U-A-12 决策把开户银行或其他字段改为字段级加密时，只增加登记行与物理列，本阶段的投影器与判定流水线不改；盲索引与受控投影按规格第 7.8 章由其所属阶段建设。
7. 移动端“把任务发送到桌面端继续”（U-K-08）已由 F-51 冻结。`high_risk_requests` 的 `CLIENT_NOT_ALLOWED` 错误响应中，`advice` 固定提示“该操作请在桌面端完成”；阶段 13 同批交付五分钟一次性桌面接续令牌与深链入口，不再等待产品文案或机制选择。

---

### 12. 本阶段新增决定与已批准偏离项

按基线第 0 节与第 12 节的要求，本节把本阶段自行决定的事项与已批准偏离集中列出并同步回写基线；本节不是待签清单，不保留实现方选择。

#### 12.1 新增决定（基线未覆盖，本阶段取值）

1. 身份主体表归 platform_core，授权表归 platform_authz；敏感字段登记表按 C-06 唯一落在 platform_core.sensitive_field_registry，由阶段 2 建立，本阶段只引用不建表。回写基线第 3.1 节。
2. 平台内核端点的模块段取 platform，路径为 /api/v1/platform/...。回写基线第 5.1 节。该段与已有的 /api/v1/portal/... 同类。
3. 平台事件的模块段取 platform，事件名如 platform.authz_policy.published.v1。回写基线第 6.1 节。
4. 平台事件的 posting_date 与 accounting_period_id 取 null；关账受理前提二的判定语句按 C-28 由阶段 9a 定死，本阶段第 6.3 节逐字采用，posting_date 为空的平台事件一律不计入。回写基线第 6.1 节。
5. 本阶段的两张仅追加表（user_password_history、login_attempts）不在基线第 4 节列举的六类之内，两表均无冲销或更正语义，因此按仅追加处理且不带 reverses_id，取舍与理由已逐表写在第 3.2 节表 3-3 与表 3-7 的定义处。回写基线第 4 节，把该节仅追加表一条改为：仅追加表一律不带 row_version、updated_at、updated_by；是否带 reverses_id uuid null 由该表有无业务冲销或更正语义决定，有的必须带并在表定义处写明它指向哪张表的哪条记录，没有的不得为满足列约定而保留恒为 NULL 的该列；取舍与理由由所属阶段在其表定义处逐表写明，该节不再列举表名。
6. 启动自检新增 authz-snapshot-loadable 一个命名项，按 C-25 以注册名标识，不用序号；判据只到可构造出完整 AuthzSnapshot 为止，checksum 不一致按降级窗口处理不阻断启动，窗口 kind 取 AUTHZ_SNAPSHOT_CHECKSUM_MISMATCH。该取值不由本阶段定义：阶段 2 只交付初始三项，Stage 14a 扩 Rust 枚举/contract，`V20261023092500` 扩数据库 CHECK 至终态 21 项；本阶段只实现调用和 mock/contract 验收，真实 PostgreSQL 持久化验收由 Stage 14 在 092500 后承接，禁止反向修改历史迁移。原拟的 duty-class-exclusivity 与 forbidden-permission-items-absent 两项不登记，分别下沉到角色授予与用户绑定的写入路径、以及 permission_items 的 ck_permission_items_forbidden_codes 约束。回写基线第 7.3 节，并在该节写明启动自检的正当判据是这个二进制能否在这台机器上正确运行，数据是否一致属运行期不变量，不进阻断级。
7. 新增指标十个：ep_authn_login_attempts_total、ep_authn_active_sessions、ep_authn_active_users、ep_sla_active_user_limit_exceeded_total、ep_authz_decision_duration_seconds、ep_authz_denied_total、ep_authz_scope_truncated_total、ep_reauth_challenges_total、ep_high_risk_requests_open、ep_breakglass_active_sessions。标签只用 legal_entity_id、operation_type、outcome、reason 四类，不用 user_id 与 doc_no。回写基线第 9.2 节。
8. 登录不设按活跃用户数拒绝或排队的并发信号量；最近 60 秒不同用户超过 20 人只触发指标、告警与 SLA 不适用记录。瞬时 HTTP 请求资源保护沿用阶段 1 的独立闸门，不得读取活跃用户计数。回写基线第 11.6 节。
9. 权限动作枚举固定为 VIEW、CREATE、UPDATE、SUBMIT、APPROVE、EXPORT 六个，不多不少。回写基线第 11 节。
10. SecurityContext 二十个字段中的 DeviceId、RoleCode、DutyClass、RecordShare、DataScopeTag、RequestId、TraceId 七个类型，其形状与取值域由第 4.1 节给全，按 A-03 由阶段 1 与该结构体同处冻结在 ep-foundation，本阶段只填充不定义。回写基线第 1.4 节安全上下文一段与裁定 A-03 的提供方一句，两处的交付范围为该结构体、四个配套枚举（含 SystemPurpose）与这七个类型。

#### 12.2 已批准偏离项（现行口径）

偏离一。基线第 3.8 节原写的不带 legal_entity_id 的表只有四类这句封闭枚举，与其中无定义、容量无限的全局配置字典这一类名，一并作废，改为正向登记制：凡带 legal_entity_id 的表一律按模板建策略；不带该列的表必须同时给出准入判据与隔离承接点两项并逐表登记。本阶段据此登记 11 张表，登记行落在阶段 2 交付的 platform_core.unpoliced_table_registry，由第 3.5 节第 29 号回填迁移一次写入，未登记的表按 db/checks 第十三项判为违规而建不出来，本阶段不再以第五类例外的形态申报。准入判据统一取该表的行集合与法人无关这一条，逐组核对如下：platform_core 的 9 张身份主体表是隔离机制自身的元数据，只承载身份主体、凭据引用、会话与设备，用户是可被授权多个法人的主体，给其行贴单一法人标签在语义上不成立，且会使登录路径在建立法人上下文之前无法读取账号与凭据，从而被迫引入一条绕过行级策略的读路径，而绕过是基线与规格第 7.7 章都不允许的；platform_authz 的 permission_items 与 object_scope_bindings 两张的行在本部署内对两个法人取值相同。隔离承接点逐组写明：9 张身份主体表的法人可见性落在任何列出用户的查询一律与受策略约束的 platform_authz.user_legal_entity_grants 内联这一条上；permission_items 与 object_scope_bindings 两张不返回任何与法人相关的行，其可见性由授权判定第二阶段的对象级判定承担。核验方式：tests/rls_matrix 对这 11 张表的每一个 API 出口单独设越权用例，判据与其余表相同，并逐表断言其可见性不随 app.legal_entity_id 变化，取值一变即失格、必须补法人列。影响范围：基线第 3.8 节、第 3.10 节基线索引约定、第 4 节公共列。

偏离二。基线第 5.6 节的头必填规则由固定四项逐头矩阵补充：内部 sign-in、complete-mfa 与门户 sign-in 豁免 Authorization、X-Legal-Entity-Id、Idempotency-Key；legal-entities 列表已认证，只豁免 X-Legal-Entity-Id。理由是这些头在相应时点尚不可能存在，且 complete-mfa 若使用通用幂等缓存会把返回的新会话令牌原文写入 `response_body`。补偿控制：矩阵为不可配置的编译期常量；三个认证前写端点按登录名或挑战主体加来源地址双维度限流并写入 login_attempts；complete-mfa 另以数据库中一次性挑战令牌的 ISSUED→CONSUMED 条件更新防重放，丢失成功响应时重新开始登录。影响范围已同步进基线第 5.4 节、第 5.6 节。

偏离三。基线第 3.10 节规定每张业务表的基线索引固定为三条，其中含 ix_<table>_legal_entity_id_created_at。9 张不带法人列的表改为 ix_<table>_<主查询列>_created_at。这是偏离一的连带项，不单独申请。影响范围：基线第 3.10 节。

#### 12.3 PRD 附录乙原待决事项的现行冻结值

| 编号 | 冻结取值 | 是否阻塞 | 切换代价 |
|---|---|---|---|
| U-A-09 | 两遍导入：静态校验任一失败则零写入；全部通过后逐行独立事务执行动态校验与幂等写入 | 否 | 唯一取值，见 F-51 |
| U-B-01、U-B-02 | 交付 F-51 列明的五个标准业务角色包；实施向导使用 `SALES`、`PROCUREMENT`、`WAREHOUSE`、`FINANCE`、`SERVICE`、`PROJECT`、`MANAGEMENT`、`DATA_OPS` 八个规范岗位标签显式映射 RoleCode | 否 | 唯一取值，见 F-51 |
| U-B-05 | 显式拒绝优先，求值顺序按基线第 11.3 节 | 否 | 基线已定死，此处只是确认 |
| U-B-06 | 字段权限四值 HIDDEN、MASKED、READ、WRITE；掩码风格三种 FULL、KEEP_LAST_4、KEEP_DOMAIN | 否 | 增删取值需改一个枚举、一条 CHECK 与投影器的一个分支 |
| U-B-07 | 默认可见来源仅为记录责任人、当前流程处理人和显式共享；创建人不因创建永久可见；共享不可再转授，`can_reshare` 由 CHECK 固定为 false | 否 | 唯一取值，见 F-51 |
| U-B-08 | 项目与客户维度与部门维度取并集，任一命中即通过记录级判定 | 否 | 改为交集需改 RecordPredicate 的合成函数与其渲染器，两处 |
| U-B-09 | 组织层级最大深度 8；调岗后按新权限判定历史记录可见性（PRD 第 10.2.3 节已明确后半句） | 否 | 深度是配置项 |
| U-B-10 | 不新增仓库维度 | 否 | 需先修订规格第 12.2 章 |
| U-B-11 | 门户账号沿用同一套法人授权集合机制，可被授权 0 到多个法人 | 否 | 两种取值都能承载 |
| U-B-13 | 重新认证覆盖单次提交；批量操作按整批一次，摘要覆盖批内全部单据编号与合计金额 | 否 | 改为逐笔只需在批量端点循环发起挑战 |
| U-B-14 | 失败 5 次锁 30 分钟，窗口 15 分钟；口令 12 位、四类取三类、90 天有效期、历史 5 代 | 否 | 全部为配置项 |
| U-B-15 | 审批链最大节点层数 10；节点超时只重复提醒，不自动升级也不自动驳回 | 否 | 自动升级需在链节点上增加一列与流程引擎的一个定时器动作 |
| U-B-16 | 账号停用时其名下未结束的待办不自动转交，返回 PLATFORM.APPROVAL.NODE_HAS_NO_APPROVER 并转人工 | 否 | 自动转交需增加一张转交映射表 |
| U-B-17 | CONFIG 职责与 SECURITY 互斥，与其余四类可兼 | 否 | 改互斥关系只需改种子 SoD 规则行 |
| U-B-18 | 敏感导出的判定为：结果集含敏感字段清单内任一字段，或对象密级不低于 30，或单次导出行数不低于 1000，三者任一成立即为敏感导出；审计记录导出计入 | 否 | 阈值为配置项，判定条件的增删改一个纯函数 |
| U-L-01 | 最近 60 秒活跃用户超过 20 人仍不拒绝登录和写入；记录、告警并标记 SLA 不适用，管理端每 5 秒刷新；单用户第 4 个会话撤销最早会话 | 否 | 唯一取值，见 F-51 |
| U-A-12 | `bank_name` 与 `bank_account_no` 均列为密级 30 并字段级加密；列表中的账号固定 `KEEP_LAST_4`，开户行仅财务权限可见；详情完整值须专门能力、重新认证与审计；包含任一完整银行字段的导出须重新认证和审批 | 否 | 唯一取值，见 F-51 |

以上 17 条均不阻塞本阶段实施。原先登记的唯一阻塞项已解除：SecurityContext 的 20 个字段、四个配套枚举与第 4.1 节给全的七个字段类型按 A-03、SYSTEM_PRINCIPAL_ID 与 SYSTEM_DEVICE_ID 按 A-02、CapabilityDomain 与 ActionClass 按 A-20，均由阶段 1 在 ep-foundation 冻结并排在本阶段之前，本阶段只负责填充与引用，本计划不再存在需要标注为阻塞的前置项。
