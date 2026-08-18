## 阶段 4：身份、认证与权限

本阶段交付平台内核的身份与访问控制层，覆盖规格第 12.1 章身份与认证、第 12.2 章授权、第 7.7 章安全上下文建立与法人越权测试集、PRD 第 10.1 节至第 10.3 节。本阶段不实现流程引擎本体、不实现配置发布通道本体、不实现审计哈希链本体、不实现通知投递本体。按裁定通则第四条，阶段顺序固定为 1 → 2 → 3a → 4 → 3b-1 → T0 → 5 → 9a → 8 → 6 → 7 → 10 → 11 → 9b → 14，阶段 3b-2 不在这条链上，其各项按阶段 3 计划第 3.0 节判定四的下游拉动点排在 T0 之后；这四项本体均由阶段 3b 交付，落在本阶段之后，其中 T0 所需的部分属 3b-1 批。空实现通则已废止：本阶段不在 apps/core-server/src/wiring/ 与 apps/job-worker/src/wiring/ 两个目录下的任何文件中注入以 Noop、Stub、Fake、Dummy 四类前缀命名的替身，两个目录中不出现任何替身类型，装配期缺实现即拒绝启动；该断言由阶段 1 随 xtask 交付的 archcheck 规则 unwired-absent 承担，出现即构建失败。依赖这四项本体的功能点按同批交付处置，与阶段 3b 一并实现、一并验收，本阶段不为它们保留任何顺延登记。同批清单只有五项：一是高风险请求进入 IN_APPROVAL 之后的全部迁移与 submit、approve、reject、withdraw 四个端点；二是 authz-config-versions 的 stage-for-release 与 activate 两个端点及其配置包导出与差异视图；三是本阶段全部写事务中的审计事件写入，理由是 platform_audit.audit_events 本身由阶段 3b 建立；四是应急账号启用的站内通知投递；五是第 8.2 节第 16 项对模块许可状态的读取。清单之外的部分在本阶段内自足，不依赖阶段 3b 的任何交付物。ConfigItemApplier 端口按 A-19 由阶段 3a 交付，本阶段在该端口上实现三个 AUTHZ 类 applier，见第 4.8 节。全部接缝的归属按裁定表逐条写死，本阶段不再登记 needs。

本计划遵守共享技术基线。凡基线已定死的取值直接引用不再重述；凡基线未覆盖而本阶段必须取值的，一律在第 12 节“本阶段新增决定与偏离项”中显式登记，并给出回写基线的位置。凡属 PRD 附录乙待决的，给出临时取值、是否阻塞与切换代价。
本阶段与贯通线 T0 的关系。T0 是一条不新增任何范围的最薄贯通线，插在阶段 3b-1 与阶段 5 之间，即阶段 4 与 3b-1 两段都结束之后、阶段 5 全量开工之前，从阶段 5、6、9a、10、11 各取最小切片，判据是一条合同从建单走到管理层看到一个数。本阶段整体排在 T0 之前，因此不从 T0 取切片，而是为 T0 提供它唯一需要的身份底座。本阶段内部的工作次序据此分两批，阶段范围归属不变，表与端点的阶段归属也不变，退出条件仍按第 9 节在本阶段结束时一次判定。

第一批是 T0 底座，15 张表加四条链路，在 T0 开跑之前完成。platform_core 五张：user_accounts、user_credentials、user_devices、sessions、reauth_challenges。platform_authz 十张：permission_items、object_scope_bindings、roles、role_permission_grants、user_legal_entity_grants、user_role_grants、approval_chains、approval_chain_nodes、high_risk_requests、authz_config_versions。四条链路是口令登录与设备登记、安全上下文建立与 app.legal_entity_id 写入、授权判定第一阶段与第二阶段、ContractEffective 与 InvoiceIssue 两类高风险操作的重新认证与供 T0 使用的单节点审批链定义及其静态校验。InvoiceIssue 一类与 ContractEffective 同批落在第一批，理由是授权清单第十条把 invoice.sales_invoices 的一张销项发票定死为 T0 内的切片，而阶段 10 的开票端点必带 X-Reauth-Token 且销项发票的 reauth_ref 与 approval_ref 两列非空，该类重新认证与其审批链不进第一批，T0 在开票一步即停。T0 用到的身份数据只有一个法人、一个操作员账号、一个设备、一个业务角色与四条单节点审批链，四条的 scenario 依次为合同生效、发票申请单、销项发票开具与资金账户建档，条数与阶段 10 第 0 节的 T0 切片清单对齐，由 ep-datagen 的 T0 最小样本生成，不用默认 scale 数据集，不要求分支覆盖，不要求四端，只要求桌面端。

第二批是加厚，在第一批的底座上补齐，与第一批同在 T0 开跑之前完成，共 9 张表与其余功能。本阶段整体是 T0 的前置，两批都排在 T0 之前，第二批不跨到 T0 之后。platform_core 四张：user_password_history、login_attempts、account_lockouts、breakglass_activations。platform_authz 五张：access_policies、field_permissions、user_org_assignments、user_scope_grants、sod_rules。功能侧包括多因子与 WebAuthn 与 X509 三类认证方式、其余四类高风险操作、职责分离四类规则、记录级与字段级与密级判定、受控应急本地账号、账号生命周期四操作、门户端点，以及第 8.3 节的 32 组完整矩阵与第 8.5 节的全部性能项。

---

### 1. 交付物清单

本阶段结束时，下列东西可运行、可用命令验证。

1. 两个新平台 crate：ep-platform-identity 与 ep-platform-authz，随 core-server 与 job-worker 编译进二进制并在 wiring 中装配完成。
2. 一条可跑通的登录链路：桌面端提交登录名与口令，经二次因子校验后取得不透明会话令牌，随后携带 Authorization、X-Legal-Entity-Id、X-Device-Id、X-Client 四个头访问任一受保护端点，服务端建立安全上下文并把 app.legal_entity_id 写入数据库会话变量。
3. 一条可跑通的授权链路：同一请求内完成法人、对象、记录、字段与密级五阶段判定，命中拒绝时返回基线第 5.5 节规定的封套与错误码，且对不可见记录返回 404 而非 403。
4. 一个字段级受控只读视图端点，对已注册对象类型返回按角色字段权限与密级裁剪后的投影，无权字段不出现在响应体的键集合中。该端点是规格附录 A.1 常规交互清单中“字段级受控只读视图加载”这一度量项的被测对象。
5. 六类高风险操作的重新认证与审批网关：重新认证挑战的签发与核销、待签内容摘要的服务端重算与比对、X-Reauth-Token 的单次消费、高风险操作请求单据及其状态机、审批链定义的静态合法性校验与运行期审批授权判定。
6. 职责分离运行期与配置期双重执行：五类管理员职责互斥、申请人不可自审、审批链不可越权跳过，三者在配置保存时拒绝、在运行期再次判定。
7. 受控应急本地账号的申请、审批、限时启用、允许操作集合裁剪、到期自动失效与使用后凭据轮换。
8. 按 A-19 实现三个 AUTHZ 类 ConfigItemApplier：AuthzRoleApplier、AuthzPolicyApplier、AuthzFieldGrantApplier，三者位于 ep-platform-authz，实现阶段 3a 在 `crates/platform/release/src/port/config_item.rs` 交付的端口，注册到 ConfigItemApplierRegistry。配置包的导出、差异视图与经发布通道审批签名后的生效切换属开篇同批清单第二项，随阶段 3b 一并交付，本阶段不提供 bundle provider，也不提供任何绕过发布通道的生效开关；本阶段运行期唯一的生效版本由第 3.5 节 27 号种子迁移直接写入 authz_config_versions 并取 EFFECTIVE。
9. tests/rls_matrix 的第三段。按 C-05，CI 目标名与 assert_read、assert_write、assert_update、assert_delete、assert_aggregate、assert_sort、assert_report_projection、assert_error_leak 八个断言函数骨架由阶段 1 在 `testkit/src/rls_matrix.rs` 提供，assert_replication_role_containment 与 assert_recon_context_borrow 两个函数由阶段 2 追加，本阶段交付 matrix_32.rs 的 32 组完整矩阵与发布门禁项 RG-RLS-MATRIX-GREEN 的判定，不重复实现上述十个同名函数。该目标可单独执行并输出结构化报告。
10. 24 张表的迁移文件与其回退说明，可在空库上离线执行到最新版本并通过启动自检 rls-enabled-and-forced 与 runtime-role-privileges-bounded 两项。
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
| ep-adapter-kms | 本阶段零改动，列出只为交代 KMS 能力的取用位：按裁定 F-04，TOTP 种子与 X.509 信任锚引用的封装经 `ep_foundation::port::kms::KmsBackend` 的 `wrap` 与 `unwrap` 执行，载体实例由 apps/core-server/src/wiring/ 与 apps/job-worker/src/wiring/ 两个目录下的装配注入，ep-platform-identity 与 ep-platform-authz 均不依赖本 crate |
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
| supplier_ref_id | uuid | null | 门户账号所属供应商的逻辑引用，跨模块不建外键 |
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

索引：pk_user_accounts、ux_user_accounts_login_name、ux_user_accounts_employee_no、ix_user_accounts_status_created_at。本表无 legal_entity_id，因此基线第 3.10 节的 ix_<table>_legal_entity_id_created_at 基线索引改为 ix_user_accounts_status_created_at，这是偏离的连带项。

表 3-2 platform_core.user_credentials

| 列 | 类型 | 约束 |
|---|---|---|
| id | uuid | pk_user_credentials |
| user_id | uuid | not null，fk_user_credentials_user_accounts on delete restrict |
| credential_kind | text | not null，ck in ('PASSWORD','TOTP','WEBAUTHN_PLATFORM','WEBAUTHN_ROAMING','X509_CERT') |
| verifier | text | null，PASSWORD 存 Argon2id 的 PHC 串，X509_CERT 存证书指纹 |
| public_key | bytea | null，WebAuthn 的 COSE 公钥 |
| credential_handle | bytea | null，WebAuthn credential id 或智能卡 subject key identifier |
| secret_ref | text | null，TOTP 种子的机密引用，形如 secret://kms/totp/<user_id>#<ver> |
| sign_count | bigint | not null default 0，WebAuthn 计数器 |
| status | text | not null，ck in ('ACTIVE','SUSPENDED','REVOKED','EXPIRED') |
| activated_at / expires_at / last_used_at / revoked_at | timestamptz | 后三个可空 |
| security_level | smallint | not null default 40 |
| data_scope_tags、row_version、created_*、updated_* | | 公共列 |

索引：pk_user_credentials、ix_user_credentials_user_id_credential_kind、ux_user_credentials_credential_handle。约束 ck_user_credentials_material：PASSWORD 与 X509_CERT 要求 verifier 非空，两类 WEBAUTHN 要求 public_key 与 credential_handle 非空，TOTP 要求 secret_ref 非空。

表 3-3 platform_core.user_password_history（仅追加）：id、user_id、verifier text not null、created_at、created_by。索引 ix_user_password_history_user_id_created_at。不带 row_version、updated_at、updated_by，也不带 reverses_id。

表 3-4 platform_core.user_devices：id、user_id、device_id text not null、client text not null（取值 win/mac/ios/android/portal/ops）、public_key bytea null、attestation_ref text null、restricted_legal_entity_id uuid null、status text not null（PENDING、ACTIVE、REVOKED）、registered_at、revoked_at、last_seen_at、公共列。索引 ux_user_devices_device_id、ix_user_devices_user_id_status。restricted_legal_entity_id 非空表示该设备只能用于该法人，安全上下文建立时取用户授权集合与该限定的交集，对应规格第 7.7 章“该用户与设备的授权法人集合”。

表 3-5 platform_core.sessions：id、user_id、user_device_row_id uuid not null（引用 user_devices.id）、token_hash bytea not null（SHA-256 of 令牌原文，令牌原文不入库）、active_legal_entity_id uuid not null、client text not null、issued_at、expires_at、idle_expires_at、last_seen_at、revoked_at、revoke_reason text null、is_breakglass boolean not null default false、公共列。索引 ux_sessions_token_hash、ix_sessions_user_id_expires_at、ix_sessions_last_seen_at。

表 3-6 platform_core.reauth_challenges：id、user_id、session_id、operation_type text not null（六类枚举）、subject_digest bytea not null（待签内容摘要的 SHA-256）、subject_summary jsonb not null（规范化后的摘要结构，敏感字段已掩码）、nonce bytea not null、credential_kind_used text null、status text not null（ISSUED、VERIFIED、CONSUMED、FAILED、EXPIRED、ABANDONED）、token_hash bytea null、issued_at、expires_at、verified_at、consumed_at、failure_count int not null default 0、公共列。索引 pk、ux_reauth_challenges_token_hash、ix_reauth_challenges_user_id_status_expires_at。

表 3-7 platform_core.login_attempts（仅追加）：id、user_id uuid null、login_name_hash bytea not null、outcome text not null（SUCCESS、CREDENTIAL_INVALID、ACCOUNT_LOCKED、ACCOUNT_INACTIVE、MFA_REQUIRED、MFA_INVALID、DEVICE_UNREGISTERED、ADMISSION_REJECTED）、client text、source_addr text、occurred_at timestamptz not null、created_by。索引 ix_login_attempts_occurred_at、ix_login_attempts_user_id_occurred_at。本表不带 row_version、updated_at、updated_by，也不带 reverses_id，理由是登录尝试没有冲销或更正语义。登录名以哈希存储，理由是失败尝试中的登录名可能是攻击者构造的任意串，明文入库会把一张运行数据表变成半个外部输入落点。

表 3-8 platform_core.account_lockouts：user_id uuid pk（一人一行）、failure_count int not null default 0、window_started_at timestamptz、locked_until timestamptz null、last_failure_at timestamptz、row_version、created_*、updated_*。索引 pk_account_lockouts、ix_account_lockouts_locked_until。

表 3-9 platform_core.breakglass_activations（单据类）：id、doc_no text not null（类型码 BGA）、status text not null（DRAFT、PENDING_APPROVAL、APPROVED、ACTIVE、EXPIRED、CLOSED、REJECTED）、user_id、requested_by、approved_by uuid null、reason text not null（长度 <= 2000）、approval_ref text null、allowed_action_set text[] not null、activated_at、expires_at、closed_at、rotated_at、rotation_result text null、公共列。索引 pk、ux_breakglass_activations_doc_no、ix_breakglass_activations_status_expires_at。allowed_action_set 的取值域固定为规格第 12.1 章列出的三类：UNLOCK_OR_RESET_ADMIN、RESTORE_CONTROLLED_CONFIG_RELEASE、TRIGGER_BACKUP_OR_RESTORE，由 CHECK 约束限定，不接受其他取值。doc_no 的唯一约束不带法人（本表无法人列），是偏离的连带项。

#### 3.3 platform_authz 的授权表（15 张）

其中 object_scope_bindings 与 permission_items 不带法人列、不建策略，按第 12.2 节偏离一的正向登记制逐表登记准入判据与隔离承接点；本阶段不再援引全局配置字典这一类名，该类名连同四类封闭枚举一并作废，理由是它没有定义、容量无限，是各阶段自我归类的唯一入口，删枚举而留类名等于问题原样保留。其余 13 张全部带 legal_entity_id 并按基线第 3.8 节的统一模板建策略，策略名 rls_<table>_le。敏感字段登记表按 C-06 不在本清单内，理由与引用方式见表 3-15 之后的一段说明。

表 3-10 platform_authz.permission_items（不带法人列，按第 12.2 节登记）：code text pk（形如 sales.sales_order）、module_code text not null（15 个模块码或 platform）、function_point text not null、allowed_actions text[] not null（子集取自 VIEW、CREATE、UPDATE、SUBMIT、APPROVE、EXPORT 六个动作）、object_type text not null、description text、created_*、updated_*、row_version。索引 pk_permission_items、ix_permission_items_module_code。六个动作照抄 PRD 第 10.2.2 节“至少含查看、新建、修改、提交、审批、导出”，本阶段取值为恰好这六个，不多不少。另有约束 ck_permission_items_forbidden_codes，拒绝写入以 platform.legal_entity_isolation 与 platform.direct_db_access 两个前缀开头的 code，即关闭或修改法人隔离机制与事务业务库直连两类权限项写不进这张表；该约束替代原先的同名启动自检项，见第 7 节。

表 3-11 platform_authz.object_scope_bindings（不带法人列，按第 12.2 节登记）：object_type text pk、schema_name text not null、table_name text not null、owner_user_col text null、owning_dept_col text null、project_col text null、customer_col text null、security_level_col text not null default 'security_level'、created_*、updated_*、row_version。这张表是记录级判定的落点：各业务模块在其阶段的 wiring 中登记自己对象的范围锚列，本阶段只登记 platform 自身的三个对象类型（platform.user_accounts、platform.roles、platform.high_risk_requests）并提供登记接口，业务对象的登记在其所属阶段完成。没有登记的对象类型在记录级判定阶段一律拒绝，不默认放行。

表 3-12 platform_authz.roles（档案类）：id、legal_entity_id、code、name、duty_class text null（SYSTEM、DATA、SECURITY、AUDIT、KEY、CONFIG，业务角色为空）、is_portal_role boolean not null default false、lifecycle_state text not null（DRAFT、PENDING_RELEASE、EFFECTIVE、SUPERSEDED、RETIRED）、retired_at timestamptz null、is_active、deactivated_at、公共列。索引 pk、ux_roles_legal_entity_id_code、ix_roles_legal_entity_id_created_at。角色一律按法人建立，不做跨法人的全局角色，理由是全局角色会立刻在这张表上制造一处需要绕过行级策略的读路径，而基线第 3.8 节不允许任何绕过。2 个法人下的复制成本可接受。

表 3-13 platform_authz.role_permission_grants：id、legal_entity_id、role_id、permission_item_code text not null、action text not null（六动作之一）、公共列。索引 pk、ux_role_permission_grants_legal_entity_id_role_id_permission_item_code_action、ix_role_permission_grants_legal_entity_id_created_at。

表 3-14 platform_authz.access_policies：id、legal_entity_id、role_id uuid null（空表示适用全部角色）、object_type text not null、effect text not null（ALLOW、DENY）、priority int not null default 100、condition jsonb not null、lifecycle_state、公共列。索引 pk、ix_access_policies_legal_entity_id_object_type_effect、ix_access_policies_legal_entity_id_created_at。condition 是受限的声明式结构，不是表达式语言：只允许对 department、position、project、customer、security_level、data_scope_tag 六个属性做 in、not_in、lte、gte、has_tag 五种断言的合取，由 serde 强类型反序列化，不做字符串求值。理由是把策略做成表达式语言等于在权限层引入一个解释器，其求值行为将成为越权测试无法穷举的面。

表 3-15 platform_authz.field_permissions：id、legal_entity_id、role_id、object_type、field_name text not null、visibility text not null（HIDDEN、MASKED、READ、WRITE）、mask_style text null（FULL、KEEP_LAST_4、KEEP_DOMAIN）、公共列。索引 pk、ux_field_permissions_legal_entity_id_role_id_object_type_field_name。

敏感字段登记表不在本阶段建立。按 C-06，全系统唯一的登记表是 platform_core.sensitive_field_registry，由阶段 2 交付，其业务列集与唯一约束 ux_sensitive_field_registry_schema_table_column 已由裁定 C-06 冻结为十一列，本阶段不复述该列集，也不声明该表另有附加列。本阶段第 4.2 节阶段四的字段密级与默认掩码风格一律从该表读取，登记行由各模块阶段以 backfill 迁移写入，本阶段不建表也不写入任何行。该表不设 approved_by 与 approved_at 两列，规格第 12.2 章“经产品负责人批准的敏感字段清单”的批准留痕由该表的 release_ref 列承载，经迁移登记时取 `MIGRATION:<迁移版本号>`，经端点登记时取 `ENDPOINT:<审批记录号>`；某字段的导出是否触发重新认证不由表列承载，按第 12.3 节 U-B-18 的判定函数计算。

表 3-16 platform_authz.user_legal_entity_grants：id、legal_entity_id、user_id、granted_from date not null、granted_to date null、granted_by uuid not null、公共列。索引 pk、ux_user_legal_entity_grants_legal_entity_id_user_id、ix_user_legal_entity_grants_legal_entity_id_created_at。这是全系统唯一决定“某用户能不能进某法人”的表，它自身受策略约束，因此法人 A 的管理员无法看到也无法写入法人 B 的授权行。

表 3-17 platform_authz.user_role_grants：id、legal_entity_id、user_id、role_id、effective_from date not null、effective_to date null、granted_by、公共列。索引 pk、ux_user_role_grants_legal_entity_id_user_id_role_id_effective_from、ix_user_role_grants_legal_entity_id_user_id。

表 3-18 platform_authz.user_org_assignments：id、legal_entity_id、user_id、department_id uuid not null、position_id uuid not null、effective_from date not null、effective_to date null、公共列。索引 pk、ix_user_org_assignments_legal_entity_id_user_id、ix_user_org_assignments_legal_entity_id_department_id。department_id 与 position_id 的外键目标按 A-04 写死为 platform_core.departments(id) 与 platform_core.positions(id)，两张表由阶段 2 交付，本阶段迁移在其之后执行，外键在 V202610121045 中建立。

表 3-19 platform_authz.user_scope_grants：id、legal_entity_id、user_id、scope_kind text not null（PROJECT、CUSTOMER、RECORD）、object_type text null（RECORD 时必填）、scope_ref_id uuid not null、can_reshare boolean not null default false、granted_by、effective_from date not null、effective_to date null、公共列。索引 pk、ix_user_scope_grants_legal_entity_id_user_id_scope_kind、ux_user_scope_grants_legal_entity_id_user_id_scope_kind_scope_ref_id。can_reshare 固定为 false 且带 CHECK 约束限定为 false，理由是 PRD 附录乙 U-B-07 中“共享可否再转授”尚未决策，首版按不可转授实现，一旦决策放开只需放宽该 CHECK。

表 3-20 platform_authz.sod_rules：id、legal_entity_id、rule_code text not null、rule_kind text not null（DUTY_EXCLUSION、ROLE_EXCLUSION、SELF_APPROVAL、CHAIN_SKIP）、left_ref text null、right_ref text null、enforcement text not null default 'BLOCK'、message_code text not null、公共列。索引 pk、ux_sod_rules_legal_entity_id_rule_code。message_code 指向 docs/error-codes.md 中的错误码，用于满足 PRD 第 10.2.2 节“异常提示需指出被拒绝的具体规则名称”。

表 3-21 platform_authz.approval_chains（档案类）：id、legal_entity_id、code、name、scenario text not null、version_no int not null default 1、lifecycle_state、is_active、deactivated_at、公共列。索引 pk、ux_approval_chains_legal_entity_id_code_version_no。scenario 的取值域包含六类高风险操作码，也包含业务模块登记的场景码。

表 3-22 platform_authz.approval_chain_nodes：id、legal_entity_id、approval_chain_id、node_no int not null、approver_kind text not null（ROLE、POSITION、DEPT_MANAGER）、approver_ref uuid null、role_code text null、quorum int not null default 1、timeout_hours int null、公共列。索引 pk、ux_approval_chain_nodes_legal_entity_id_approval_chain_id_node_no。表上没有 allow_skip 一类列，理由见第 4.5 节：越权跳过不是被校验拒绝的配置，而是根本没有承载它的字段。

表 3-23 platform_authz.high_risk_requests（单据类）：id、legal_entity_id、doc_no（类型码 HRR）、status text not null（十一态见第 4.4 节）、operation_type text not null（六类）、subject_object_type text not null、subject_object_id uuid not null、subject_digest bytea not null、reauth_challenge_id uuid null、approval_chain_id uuid not null、approval_instance_ref uuid null（流程引擎实例的逻辑引用，跨平台组件不建外键）、initiator_user_id、initiator_device_id text、submitted_at、decided_at、executed_at、execution_ref uuid null、reject_reason text null、公共列。索引 pk、ux_high_risk_requests_legal_entity_id_doc_no、ix_high_risk_requests_legal_entity_id_created_at、ix_high_risk_requests_legal_entity_id_status_operation_type。

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

1. V202610120900__platform_core_identity_user_accounts.sql
2. V202610120905__platform_core_identity_user_credentials.sql
3. V202610120910__platform_core_identity_user_password_history.sql
4. V202610120915__platform_core_identity_user_devices.sql
5. V202610120920__platform_core_identity_sessions.sql
6. V202610120925__platform_core_identity_reauth_challenges.sql
7. V202610120930__platform_core_identity_login_attempts.sql
8. V202610120935__platform_core_identity_account_lockouts.sql
9. V202610120940__platform_core_identity_breakglass_activations.sql
10. V202610120945__platform_core_backfill_system_principal_account.sql

db/migrations/platform_authz/ 追加：

11. V202610121000__platform_authz_permission_items.sql
12. V202610121005__platform_authz_object_scope_bindings.sql
13. V202610121010__platform_authz_roles.sql
14. V202610121015__platform_authz_role_permission_grants.sql
15. V202610121020__platform_authz_access_policies.sql
16. V202610121025__platform_authz_field_permissions.sql
17. V202610121035__platform_authz_user_legal_entity_grants.sql
18. V202610121040__platform_authz_user_role_grants.sql
19. V202610121045__platform_authz_user_org_assignments.sql
20. V202610121050__platform_authz_user_scope_grants.sql
21. V202610121055__platform_authz_sod_rules.sql
22. V202610121100__platform_authz_approval_chains.sql
23. V202610121105__platform_authz_approval_chain_nodes.sql
24. V202610121110__platform_authz_high_risk_requests.sql
25. V202610121115__platform_authz_authz_config_versions.sql
26. V202610121120__platform_authz_backfill_permission_item_seed.sql
27. V202610121125__platform_authz_backfill_admin_duty_roles.sql（同一文件内一并写入两个法人各一行 state 取 EFFECTIVE 的 authz_config_versions，checksum 按该文件写入的配置行现算；这是本阶段运行期唯一的生效版本来源，启动自检 authz-snapshot-loadable 据此可构造快照）
28. V202610121130__platform_authz_backfill_default_sod_rules.sql

db/migrations/platform_core/ 再追加一个回填文件，其主要创建对象是 platform_core.unpoliced_table_registry 的登记行，版本号晚于本阶段全部建表迁移，故列在最后：

29. V202610121135__platform_core_backfill_unpoliced_table_registry.sql（按基线第 3.8 节的正向登记制，向阶段 2 交付的 platform_core.unpoliced_table_registry 写入本阶段 11 张不带法人列的表各一行，schema、table、准入判据、隔离承接入口与 rls_matrix 用例标识五列按阶段 2 冻结的列集填写；准入判据一列，platform_core 的 9 张身份主体表取隔离机制自身的元数据一档，platform_authz 的 permission_items 与 object_scope_bindings 两张取行在本部署内对全部法人取值相同一档，取值名以阶段 2 冻结的枚举为准，逐表的隔离承接入口按第 12.2 节偏离一写明）

迁移执行顺序由单一全局 Runner 按文件版本号全序排定，不存在任何模块顺序声明文件，本阶段只需保证自己每个文件的版本号晚于其全部被引用对象；二十四个目录按 C-01 由阶段 1 建为空目录，platform_core 与 platform_authz 均在其中。本阶段有两处跨 schema 引用，其主要创建对象都在 platform_authz，按裁定通则第五条一律放在 db/migrations/platform_authz/ 目录下：platform_authz 的表引用 platform_core.user_accounts；user_org_assignments 的外键指向阶段 2 交付的 platform_core.departments 与 platform_core.positions，该外键落在 V202610121045。空库上按文件版本号全序执行时，两处引用的前置对象均已由阶段 2 版本号更早的迁移建立。

每个迁移文件头部按基线第 3.9 节写 -- rollback: 段。10 号、26 至 28 号与 29 号是数据回填文件，slug 以 backfill_ 开头，其中 10 号与 26 至 28 号的回退说明为按 code 删除种子行，29 号的回退说明为按 schema 与 table 两列删除本阶段登记的 11 行。24 张建表迁移全部属于新增表，落在在线变更范围内，不需要停机窗口。V202610121030 号段作废，敏感字段登记表按 C-06 由阶段 2 在 platform_core 建立，本阶段不占用该号段。

第 10 号迁移写入 `00000000-0000-7000-8000-000000000001` 的系统主体账号行，account_kind 取 SYSTEM，login_name 取 system，status 取 ACTIVE，无凭据。该取值即 ep-foundation 的 SYSTEM_PRINCIPAL_ID 常量，按 A-02 由阶段 1 冻结。本阶段凡在种子迁移与系统上下文写 created_by 的一律引用该常量，设备标识引用 SYSTEM_DEVICE_ID，取值为 SYSTEM，不得自选其他值。

---

### 4. 领域模型与关键算法

本阶段不涉及任何账务处理。凡高风险操作执行后产生的分录一律按规格第 5.2 章事件-分录表由对应业务模块生成，本阶段只承担其前置的重新认证与审批放行，不参与借贷与取价。

#### 4.1 核心类型

ep-foundation 侧（字段集合由阶段 1 按 A-03 冻结为下列 19 个字段，字段顺序即下列顺序，不得增删改名，本阶段只负责填充）：

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
}
```

SecurityContext 的构造入口只有 `SecurityContext::human(..)` 与 `SecurityContext::system(legal_entity_id, request_id, trace_id)` 两个，后者按 A-02 用 SYSTEM_PRINCIPAL_ID 与 SYSTEM_DEVICE_ID 填 user_id 与 device_id，account_kind 取 System。SecurityContext 一经构造不再修改，任何“提权”都必须重新走一次会话建立，不提供任何 with_ 前缀的变换方法。配套枚举同在 ep-foundation 冻结：AccountKind 取 Human、System、Portal 三值，platform_core.user_accounts.account_kind 的四个取值按 EMPLOYEE 与 BREAKGLASS 映射为 Human、PORTAL 映射为 Portal、SYSTEM 映射为 System，映射函数落在 identity 仓储内；ClientKind 取 Win、Mac、Ios、Android、Portal、Ops 六值，与 user_devices.client 的六个取值以及基线第 5.6 节 X-Client 头一一对应；DepartmentScope 取 All、Subtree、Explicit 三个变体，第 4.2 节阶段三的部门范围编译结果落在该枚举上。request_id 与 trace_id 两个字段是基线第 3.8 节要求写入 app.request_id 与 app.trace_id 两条会话变量的取数来源，安全上下文之外不得另设第二处取数。

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
}
```

五个字符串 newtype 的构造入口一律为 `parse(&str) -> Result<Self, AppError>`，不提供绕过校验的构造，也不实现 `From<String>`。DeviceId 长度 1 至 64，字符集为大小写字母、数字、下划线、连字符与点，与 platform_core.user_devices.device_id 列和基线第 5.6 节 X-Device-Id 头同域，基线第 1.4 节的 SYSTEM_DEVICE_ID 能通过该校验，`SecurityContext::system` 由此构造该字段。RoleCode 长度 1 至 64，字符集为小写字母、数字、下划线与点，与 platform_authz.roles.code 同域，该列的写入一律经 RoleCode 解析后落库，数据库侧不另设第二套字符集校验。DutyClass 六个变体的序列化取值与 platform_authz.roles.duty_class 的六个字符串逐字一致，该列为空的业务角色不产生任何变体，因此 duty_classes 允许为空数组，不设表示无职责的第七个变体；互斥关系不进枚举定义，它是第 4.5 节种子 SoD 规则行的内容。RecordShare 只表达某条记录被显式共享给当前主体，object_type 与 platform_authz.object_scope_bindings.object_type 同域，第 4.2 节阶段三的 shared_record_ids 由它按 object_type 过滤后取 object_id 汇成；记录级的动作粒度由阶段二的权限项动作承担、字段粒度由阶段四承担，因此本结构不带授予方式与可否转授两类字段，U-B-07 改判只增加 ScopeCompiler 的谓词分支，不改本结构。RecordScope 与 RecordPredicate 不进 ep-foundation，留在 ep-platform-authz，理由是两者含判定语义，前移即违反基线第 1.3 节的依赖方向。DataScopeTag 的形态为 `<kind>:<value>`，kind 取小写字母、数字、下划线与连字符，value 取大小写字母、数字、下划线与连字符，总长上限 128，其 Display 与 serde 输出即基线第 4 节公共列 data_scope_tags 的元素形态与基线第 6.1 节事件信封 data_scope_tags 的元素形态，两处不得各自编解码。RequestId 长度 8 至 64，字符集为大小写字母、数字、下划线与连字符，与基线第 5.6 节 X-Request-Id 头同域，服务端自生成时取 UUIDv7 的无连字符三十二位小写十六进制。TraceId 固定为三十二位小写十六进制，与 W3C trace-context 的 trace-id 同形，也与结构化日志的 trace_id 字段同域。

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
}
```

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

1. 会话准入。先取并发信号量，当前活跃用户数达到上限时进入等待队列，等待超过 10 秒返回 503 与 PLATFORM.CAPACITY.CONCURRENCY_LIMIT，照抄基线第 11.6 节。活跃用户定义为最近 60 秒内有过请求的不同 user_id，内部与门户合计计数。
2. 按 login_name 取 user_accounts 行。未找到时仍执行一次固定成本的 Argon2id 校验（对内置的伪 PHC 串），使未知用户与错口令的响应时间同分布，随后返回同一个错误码 PLATFORM.AUTHN.CREDENTIAL_INVALID。
3. 取 account_lockouts 行并加行锁。locked_until 大于当前时刻即返回 PLATFORM.AUTHN.ACCOUNT_LOCKED。
4. 校验第一因子。PASSWORD 走 Argon2id 验证并检查口令有效期；X509_CERT 走挑战签名验签。
5. 判定是否需要第二因子。is_mfa_required 为真，或该用户持有任一 duty_class 非空的有效角色授予，或该用户被授予任一含六类高风险操作权限项的角色时，强制要求第二因子，对应规格第 12.1 章“管理员与高风险角色强制 MFA”。
6. 校验设备。X-Device-Id 必须在 user_devices 中且 status 为 ACTIVE，否则返回 PLATFORM.AUTHN.DEVICE_NOT_REGISTERED，照抄基线第 5.6 节“未登记设备拒绝访问业务数据”。
7. 会话数上限。该用户 expires_at 未到且 revoked_at 为空的会话超过 3 个时，把 issued_at 最早的一条置为 revoked，revoke_reason 取 SESSION_LIMIT_EXCEEDED，并写一条审计事件，照抄基线第 11.6 节。
8. 生成 32 字节随机令牌，base64url 编码后长度为 43，只把 SHA-256 摘要写入 sessions.token_hash，明文令牌只在响应体中出现一次。
9. 写入 sessions、写入 login_attempts、写入审计事件、重置 account_lockouts 计数，四项在同一事务内提交。

失败路径的事务处理是本阶段最容易写错的一处：认证失败必须持久化失败计数，而基线第 10.3 节禁止一个请求内开多个写事务。做法是登录用例的事务闭包永远返回 Ok(LoginOutcome)，其中 LoginOutcome 有 Succeeded 与 Rejected 两个变体，失败计数与失败审计在同一次提交中落库，事务提交后再由用例把 Rejected 映射成 AppError 返回给 HTTP 层。禁止用回滚表达失败。

会话校验：每个受保护请求取 Authorization 头，SHA-256 后查 sessions，校验 expires_at 与 idle_expires_at，通过后把 idle_expires_at 滑动到当前时刻加 30 分钟。滑动续期的写入合并到该请求已有的事务中；只读请求的滑动续期改为按 60 秒粒度批量写，避免每个查询请求都产生一次写事务。

#### 4.4 高风险操作的重新认证与审批

状态机照抄 PRD 第 10.3.3 节，十一个状态：待发起 PENDING_INITIATION、待重新认证 PENDING_REAUTH、认证失败 REAUTH_FAILED、已锁定 LOCKED、已认证待提交 REAUTH_PASSED、审批中 IN_APPROVAL、已批准 APPROVED、已驳回 REJECTED、已撤回 WITHDRAWN、已放弃 ABANDONED、已执行 EXECUTED。本阶段自足的部分止于 REAUTH_PASSED：PENDING_INITIATION、PENDING_REAUTH、REAUTH_FAILED、LOCKED、REAUTH_PASSED、ABANDONED 六态与其间的迁移在本阶段实现并验收。IN_APPROVAL、APPROVED、REJECTED、WITHDRAWN、EXECUTED 五态的迁移都要先建立审批实例，属开篇同批清单第一项，与阶段 3b 的流程引擎本体一并实现、一并验收；本阶段不为它们注入任何替身，也不以假实现跑通后声称可演示，下表把这五态的行一并冻结为对流程引擎的接口约束。

| 起态 | 止态 | 触发 | 守卫条件 |
|---|---|---|---|
| PENDING_INITIATION | PENDING_REAUTH | 用户提交且动作命中六类 | 发起人对该对象持有 SUBMIT 权限；已存在生效的审批链定义 |
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

X-Reauth-Token 的消费是一次条件更新：`update platform_core.reauth_challenges set status='CONSUMED', consumed_at=now() where id=$1 and status='VERIFIED' and expires_at > now()`，受影响行数为 0 即拒绝。该更新与业务写入同事务，因此重复提交只能成功一次。

#### 4.5 职责分离与审批链静态校验

四类规则，全部在配置保存时执行一次、在运行期提交时再执行一次，两次用同一份纯函数实现。

1. DUTY_EXCLUSION。同一用户在同一法人内不得同时持有 SYSTEM、DATA、SECURITY、AUDIT、KEY 五类中的两类，照抄规格第 12.2 章。CONFIG 类的归属属于 PRD 附录乙 U-B-17 待决，本阶段临时取值为 CONFIG 与 SECURITY 互斥、与其余四类可兼。
2. ROLE_EXCLUSION。客户自定义的角色互斥对。
3. SELF_APPROVAL。审批链的任一节点的可审批主体集合与发起人集合的交集必须为空。校验方式是把节点的 approver_kind 展开成用户集合：ROLE 展开为该法人内持有该角色的有效用户，POSITION 展开为该岗位的在岗用户，DEPT_MANAGER 展开为发起人所在部门链上的负责人。交集非空即拒绝保存，错误码 PLATFORM.SOD.SELF_APPROVAL_FORBIDDEN，消息中指出冲突节点号，对应 PRD 第 10.3.4 节“指出冲突节点”。
4. CHAIN_SKIP。节点 node_no 必须从 1 起连续无空洞，quorum 必须为正且不超过该节点展开后的用户数，不存在任何表达“允许跳过”的字段。运行期推进时校验前序节点全部完成，且不提供任何管理端强制完成节点的接口。规格第 12.2 章“审批链不可越权跳过”在本设计中不是一条被校验的规则，而是一个不存在的能力。

边界条件：节点展开后用户集合为空时拒绝保存，理由是空集合的节点在运行期等价于自动通过，等于一个隐藏的跳过；用户离职停用导致某节点展开为空时，运行期返回 PLATFORM.APPROVAL.NODE_HAS_NO_APPROVER 并转人工，不自动跳过，对应 PRD 附录乙 U-B-16。

#### 4.6 受控应急本地账号

状态机：DRAFT、PENDING_APPROVAL、APPROVED、ACTIVE、EXPIRED、CLOSED、REJECTED。启用四要素按规格第 12.1 章逐项落地：限时由 expires_at 与 job-worker 的到期任务保证，单次不超过 8 小时；强认证由强制第二因子且 credential_kind 不得为 PASSWORD 单因子保证；实时告警由启用瞬间写入阶段 2 已交付的 platform_ops 台账并经其告警通道发出保证，站内通知投递属开篇同批清单第四项，随阶段 3b 一并接入，本阶段不为它注入替身；独立人员复核由 approved_by 与 requested_by 必须不同人且 approved_by 需持有 SECURITY 或 AUDIT 职责保证。

允许操作集合的执行点在授权判定阶段二之前插入一道前置闸门：会话的 is_breakglass 为真时，只有 allowed_action_set 中三类操作对应的 permission_item 可通过，其余一律 Deny(ObjectForbidden) 并触发告警。业务写入、审计策略修改、密钥签发轮换销毁、职责分离绕过、常规业务审批五类操作在该闸门处被拒绝，对应规格第 12.1 章的禁止集合。

到期后由 job-worker 执行：撤销该账号全部会话、把凭据置为 REVOKED、生成新凭据并写入机密库、把轮换结果写入 breakglass_activations.rotation_result 与审计。闲置轮换按每 12 个月一次，由同一任务按 last rotated_at 判定。

#### 4.7 字段投影

FieldProjector 输入为对象类型、对象的原始行（serde_json::Value）与 SecurityContext，输出为新的 Value，不修改输入。掩码规则：FULL 输出固定字符串六个星号；KEEP_LAST_4 保留末四位其余替换为星号，长度不足 8 位时退化为 FULL；KEEP_DOMAIN 用于电子邮箱，保留 at 之后的部分。字段在 platform_core.sensitive_field_registry 中登记且 is_field_encrypted 为真时物理列是密文，上述三条不施加于密文：KEEP_LAST_4 的后四位直接取自同表的 `<column_name>_tail` 列，FULL 与 HIDDEN 既不读密文也不解密；只有字段权限为 READ 或 WRITE 且用户 clearance_level 不低于该字段密级时，才在投影前经 SensitiveFieldDecryptor 解密后输出，字段投影路径上只有这一处解密位点，不经字段投影而需要明文的解密由需要它的那个阶段在其计划内自行指名位点，同样调用 SensitiveFieldDecryptor，全库不得出现第二套解封路径。按 A-28，命中该分支的字段以 platform_core.sensitive_field_registry 中 is_field_encrypted 为真的登记行为准，本阶段不另列第二份清单；登记行与其物理列由引入该列的模块阶段在同一迁移内交付，本阶段只按登记行渲染，不建表也不写登记行；登记行的 mask_style 取 KEEP_LAST_4 时该表必须同时存在 `<column_name>_tail` 列，没有该列的只能取 FULL。字段在 field_permissions 中无授权行时按默认拒绝处理，不进入响应键集合，与阶段二的默认拒绝一致；各模块字段的授权行按 A-19 的 AUTHZ_FIELD_GRANT applier 经配置发布通道在其所属阶段之后写入。掩码后的值不参与排序与聚合，任何列表端点如果按 MASKED 或 HIDDEN 字段排序，一律返回 VALIDATION 与 PLATFORM.AUTHZ.SORT_FIELD_FORBIDDEN，这是 PRD 第 10.2.4 节“不得通过排序位次间接暴露”的实现点。分面计数同理：计数的分组键若含无权字段，该分面整体不返回。
#### 4.8 权限配置对象的配置包 applier

按 A-19，ConfigItemApplier trait、含 16 项的 ItemKind 枚举（第 16 项 `RULE` 由裁定 F-21 新立，代码侧由阶段 13b 同批加入）、ConfigPackageItem 与 ConfigItemApplierRegistry 由阶段 3a 在 `crates/platform/release/src/port/config_item.rs` 交付，其中的事务句柄类型取自 ep-foundation。本阶段实现其中三个 item_kind，实现类型全部落在 ep-platform-authz。

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

sign-in、legal-entities 两类端点在认证完成前调用，无法提供 Authorization、X-Legal-Entity-Id 与 Idempotency-Key。本阶段设一个固定三项的白名单常量 PRE_AUTH_ENDPOINTS，对其豁免这三个头，其余头照旧必填。豁免的补偿是按登录名与来源地址的双维度速率限制，超限返回 429 与 PLATFORM.AUTHN.RATE_LIMITED。该豁免登记为偏离项，见第 12.2 节。

#### 5.2 会话与身份

| 方法与路径 | 请求 | 响应 data | 主要错误码 | 幂等 | 权限 |
|---|---|---|---|---|---|
| POST /api/v1/platform/sessions/actions/sign-in | login_name、credential（kind 与 material）、client、device_id | session_token、expires_at、idle_expires_at、user_id、default_legal_entity_id、mfa_challenge 可选 | PLATFORM.AUTHN.CREDENTIAL_INVALID、PLATFORM.AUTHN.ACCOUNT_LOCKED、PLATFORM.AUTHN.ACCOUNT_INACTIVE、PLATFORM.AUTHN.MFA_REQUIRED、PLATFORM.AUTHN.DEVICE_NOT_REGISTERED、PLATFORM.CAPACITY.CONCURRENCY_LIMIT | 豁免幂等键，按第 5.1 节 | 匿名 |
| POST /api/v1/platform/sessions/actions/complete-mfa | mfa_challenge_id、credential | 同上 | PLATFORM.AUTHN.MFA_INVALID、PLATFORM.AUTHN.MFA_CHALLENGE_EXPIRED | 幂等键必填 | 匿名加挑战绑定 |
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

#### 5.4 账号生命周期

对应 PRD 第 10.2.3 节四行。

| 方法与路径 | 输入 | 处理 | 错误码 |
|---|---|---|---|
| POST /api/v1/platform/user-accounts/actions/import-batch | 用户清单，单次上限 200 行 | 逐行校验必填与唯一性，失败行整行退回并给出行号与原因，成功行照常建立 | PLATFORM.USER_ACCOUNT.BATCH_PARTIAL_FAILED，details 逐行给出 |
| POST /api/v1/platform/user-accounts/{id}/actions/activate | 生效日期、初始角色 | 校验口令复杂度策略与 MFA 要求 | PLATFORM.USER_ACCOUNT.MFA_ENROLLMENT_REQUIRED |
| POST /api/v1/platform/user-accounts/{id}/actions/transfer | 新部门、新岗位、新角色、生效日期 | 校验职责分离；校验该用户名下未结束的审批待办 | PLATFORM.SOD.DUTY_CONFLICT、PLATFORM.USER_ACCOUNT.PENDING_APPROVAL_TASKS |
| POST /api/v1/platform/user-accounts/{id}/actions/deactivate | 停用日期 | 即时生效，撤销全部会话与设备凭据，发出 platform.user_account.deactivated.v1 | — |

批量建号的部分失败语义取“逐行落库，失败行退回”，对应 PRD 附录乙 U-A-09；该取值为本阶段临时取值，若产品负责人改为整体回滚，改动范围限于该一个用例的事务边界。

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

移动端限制的执行点从 submit 端点前移到 POST /api/v1/platform/reauth-challenges：X-Client 为 ios 或 android 且 operation_type 属于 Payment、InvoiceIssue、LedgerPosting、PeriodClose 四类时，返回 PLATFORM.HIGH_RISK_REQUEST.CLIENT_NOT_ALLOWED 并在 advice 中说明该操作在桌面端完成，对应规格第 6.2 章矩阵与 PRD 第 10.3.1 节移动端列。ContractEffective 与 SensitiveExport 在移动端可发起。前移的理由有两条：这四类操作在移动端连挑战都不该签发，拒绝点越早越好；submit 端点随阶段 3b 同批交付，把判定留在那里会使规格第 6.2 章这条约束在本阶段无处验收。

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
| 账号生命周期四操作 | user_accounts 更新、user_role_grants 与 user_org_assignments 写、审计、Outbox | READ COMMITTED | 乐观锁；transfer 需先读该用户未结束审批待办 |
| 授权配置保存 | 对应配置表写、authz_config_versions 更新、审计 | READ COMMITTED | 乐观锁；同一 version 内的并发编辑靠 row_version 冲突暴露 |
| 配置版本生效 | authz_config_versions 更新、审计、Outbox | READ COMMITTED | 对该法人的当前生效版本行 FOR UPDATE，保证同法人同时只有一个 EFFECTIVE |
| 发起重新认证挑战 | reauth_challenges 插入、审计 | READ COMMITTED | 无 |
| 核销挑战 | reauth_challenges 条件更新、审计 | READ COMMITTED | 条件更新即锁 |
| 提交高风险请求 | high_risk_requests 更新、reauth_challenges 消费、审批实例建立（经流程引擎在同一事务内）、审计、Outbox | READ COMMITTED | high_risk_requests 乐观锁；挑战条件更新 |
| 审批节点通过或驳回 | 审批实例推进、high_risk_requests 更新、审计、Outbox | READ COMMITTED | 乐观锁；节点推进由流程引擎的行锁保证 |
| 应急账号启用与关闭 | breakglass_activations 更新、user_credentials 轮换、sessions 撤销、审计、Outbox | READ COMMITTED | 乐观锁 |
| 越权测试与内部对账的只读遍历 | 无写入 | REPEATABLE READ 单事务 | 只读 |

全部写事务遵守基线第 10.3 节：一个用例一个事务，事务内不做外部调用、不读写文件正文、不发通知、不等待用户输入。审批实例的建立必须与 high_risk_requests 的状态迁移同事务，否则会出现“请求单已提交但审批实例不存在”的悬挂态；这一点是对流程引擎的接口要求：审批实例的建立必须接受调用方传入的事务句柄，不得自行开事务。流程引擎本体由阶段 3b 交付，落在本阶段之后，因此本表中提交高风险请求与审批节点通过或驳回两行属开篇同批清单第一项，其事务边界在本阶段只作为对流程引擎的接口约束冻结，不在本阶段实现，也不接线任何替身。审计事件同理：platform_audit.audit_events 由阶段 3b 建立，本表各行中的审计事件一栏属同批清单第三项，本阶段不注入任何替身审计端口，装配期缺实现即拒绝启动，不存在审计端口返回成功而实际没有落库这一形态。

#### 6.2 幂等

全部写端点按基线第 5.4 节使用 Idempotency-Key，作用域为法人、用户、端点、键值四元组，幂等键写入与业务写入同事务。三个认证前端点豁免，改用速率限制。

重新认证令牌本身是第二重幂等：同一个 X-Reauth-Token 只能消费一次，因此即使幂等键被客户端误用为新值，重复提交仍然在挑战消费处失败并返回 PLATFORM.REAUTH.TOKEN_ALREADY_CONSUMED。

#### 6.3 与 Outbox 的关系

本阶段发出五个事件，全部在业务事务内写入 platform_msg.outbox_events，信封字段按基线第 6.1 节。平台事件的 posting_date 与 accounting_period_id 取 null，因为它们不是账务事件。关账受理前提二的判定语句按 C-28 由阶段 9a 定死，本阶段逐字采用：该法人该期间内，platform_msg.outbox_events 中 status 属于 PENDING 或 DISPATCHING、posting_date 落在该期间起止之间、且 event_type 命中 ledger.posting_trigger_event_types 的条目数为零，且 platform_msg.dead_letters 中 state 属于 OPEN 或 REPAIRING、同样命中该注册表的条数为零。posting_date 为空的平台事件一律不计入，理由是它们不产生凭证；本阶段五个事件不在阶段 9a 按 A-21 一次写入的 13 行种子之内，本阶段也不追加任何回填迁移，因此不会误拦关账。

本阶段不消费任何事件。原先由 job-worker 消费 platform.authz_policy.published.v1 再经进程间接口通知 core-server 重建快照这条链已按第 2.3 节整条删除，快照重载改为 core-server 自身轮询 authz_config_versions，因此本阶段不使用 platform_msg.inbox_consumptions，也不产生与之相关的死信与重投路径。该事件的唯一消费方是规格第 7.9 章的派生存储重建，由派生存储所属阶段承接。

#### 6.4 失败重试与补偿

序列化失败 40001 与死锁 40P01 由数据访问层重试 3 次，退避 50、150、450 毫秒，照抄基线第 8.4 节。登录用例可安全重试，因为它在重试前未产生任何外部可见副作用。挑战核销与高风险请求提交不可重试，因为它们的条件更新一旦成功即产生外部可见状态，重试由客户端按幂等键发起。

补偿路径只有一处：高风险请求进入 APPROVED 后，业务模块执行失败时不回滚审批结论，而是由业务模块写入执行失败并把请求单留在 APPROVED，由人工重试或重新发起。理由是审批是一个已经发生的事实，用补偿把它抹掉会让审计证据与实际发生的审批过程不一致。

#### 6.5 并发准入

并发上限 20 由 core-server 进程内的信号量承担，门户流量经受控能力 API 同样计入，因此单副本下计数是完整的。等待队列有界，队列长度上限取 40，超出直接返回 503，避免排队本身成为内存增长点。指标 ep_session_admission_queue_wait_seconds 与 ep_session_admission_rejected_total 暴露到运维中心。

---

### 7. 配置项

全部键在 EP__AUTH、EP__AUTHZ、EP__ADMISSION 三个前缀下，结构体开启 deny_unknown_fields。除注明外一律启动时生效，理由是把安全参数做成热生效开关会在运行期制造一个不经配置发布通道的旁路。

| 键 | 类型 | 默认值 | 生效方式 | 说明 |
|---|---|---|---|---|
| EP__AUTH__PASSWORD__MIN_LENGTH | u8 | 12 | 启动 | U-B-14 临时取值 |
| EP__AUTH__PASSWORD__MIN_CHAR_CLASSES | u8 | 3 | 启动 | 大写、小写、数字、符号中取三类 |
| EP__AUTH__PASSWORD__MAX_AGE_DAYS | u16 | 90 | 启动 | 0 表示不过期 |
| EP__AUTH__PASSWORD__HISTORY_SIZE | u8 | 5 | 启动 | 与 user_password_history 配合 |
| EP__AUTH__PASSWORD__ARGON2__MEMORY_KIB | u32 | 65536 | 启动 | 单次校验约 64 MB |
| EP__AUTH__PASSWORD__ARGON2__ITERATIONS | u32 | 3 | 启动 | |
| EP__AUTH__PASSWORD__ARGON2__PARALLELISM | u32 | 1 | 启动 | 单机配额下不并行 |
| EP__AUTH__LOCKOUT__MAX_FAILURES | u8 | 5 | 启动 | U-B-14 临时取值 |
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
| EP__AUTHZ__SCOPE__MAX_DEPARTMENT_DEPTH | u8 | 8 | 启动 | U-B-09 临时取值 |
| EP__AUTHZ__SCOPE__IN_LIST_THRESHOLD | u16 | 200 | 启动 | 超过阈值改用 EXISTS 子查询 |
| EP__AUTHZ__EXPORT__SENSITIVE_ROW_THRESHOLD | u32 | 1000 | 启动 | U-B-18 临时取值 |
| EP__ADMISSION__MAX_CONCURRENT_USERS | u16 | 20 | 启动 | 基线第 11.6 节 |
| EP__ADMISSION__QUEUE_WAIT_TIMEOUT_SECONDS | u8 | 10 | 启动 | 基线第 11.6 节 |
| EP__ADMISSION__QUEUE_MAX_LEN | u16 | 40 | 启动 | 本阶段新增 |
| EP__ADMISSION__ACTIVE_WINDOW_SECONDS | u16 | 60 | 启动 | 活跃用户判定窗，U-L-01 临时取值 |

敏感取值一律不进配置文件：X.509 信任锚、TOTP 主密钥、会话令牌无需密钥（不透明随机），全部按基线第 7.2 节以 secret:// 引用表达，内存中用 secrecy::SecretString 包装。

启动自检新增一个命名项，按 C-25 以注册名标识而不用序号，注册顺序排在基线第 7.3 节的十项命名项之后（原写的十三项为已作废的旧口径，见 00c 裁定 C-25 与阶段 1 计划第 7.3 节回写）：

- authz-snapshot-loadable：每个法人存在至少一个 EFFECTIVE 的 authz_config_versions，且据其配置行可构造出完整的 AuthzSnapshot。判据只到可构造为止，构造不出即以退出码 78 退出，理由是无快照则任何授权判定都做不了，全拒等价于停机、放行等于灾难，没有第三条路，这一项必须留在阻断级。

checksum 与配置行重算值是否一致不进阻断判据。不一致时进程不退出，改为回退到上一版 EFFECTIVE 快照、经阶段 2 已交付的 DegradationLedger 开一个 kind 取 AUTHZ_SNAPSHOT_CHECKSUM_MISMATCH 的降级窗口并持续告警；该窗口的 scope_legal_entity_id 取校验和不符的那个法人，subject 列不填，本窗口不指向任何端口。DegradationKind 的唯一定义方是阶段 2，终态取值清单的唯一出处是阶段 14，AUTHZ_SNAPSHOT_CHECKSUM_MISMATCH 落在阶段 14 的终态十八项之内，本阶段只使用该取值，不自建第二套枚举，也不新增取值。运行期后果写明：授权配置的新版本不生效，判定按上一版快照执行，直到人工修复后关窗。

原先登记的 duty-class-exclusivity 与 forbidden-permission-items-absent 两项删除。两者判读的都是数据库里的业务行，而这台服务器只有一台、没有备节点，把数据一致性校验做成启动硬失败等于把一处配置错误放大成全企业停摆，此时唯一可行的恢复动作恰是这些校验存在的理由所要禁止的手工改库。两项各有更早的落点：duty-class-exclusivity 下沉到角色授予与用户绑定两条写入路径，由第 4.5 节的同一份纯函数在保存期与运行期各判一次，规格第 12.2 章五类管理员职责分离的承载不变；forbidden-permission-items-absent 下沉为表 3-10 的 ck_permission_items_forbidden_codes 约束，禁止的东西写不进去，不必等下一次重启才发现。

该项失败以退出码 78 退出，--check 模式一并执行；--check 模式下降级窗口按非零退出处理，即闸门留在部署与升级前置，不留在进程启动。

---

### 8. 测试计划

#### 8.1 单元测试

在 ep-platform-authz 与 ep-platform-identity 内，不触库不触网不取真实时间。

判定流水线分支：法人未授权、对象无权、对象显式拒绝与允许并存时拒绝优先、记录范围为 All、为空、按部门命中、按项目命中、按客户命中、按显式共享命中、范围绑定未登记、密级高于用户许可、字段 HIDDEN、字段 MASKED 的三种掩码风格、字段 READ 时写入被拒、部门闭包深度超限截断、部门集合超阈值退化为 EXISTS。

状态机分支：AccountStatus 的全部合法迁移与全部非法迁移各一条；SessionState 的过期、空闲过期、被挤下线、主动撤销；ReauthState 十一条迁移中属于挑战的六条；HighRiskRequestState 的十一态与 PRD 第 10.3.3 节列出的全部迁移逐条，另加五条非法迁移（跳过审批直接执行、驳回后直接执行、撤回在首节点结论后、非发起人撤回、非本人核销挑战）。

职责分离与链校验：五类管理员两两组合共 10 对的互斥判定、CONFIG 与 SECURITY 互斥、自审检测在 ROLE 与 POSITION 与 DEPT_MANAGER 三种展开下各一条、节点号空洞、quorum 越界、节点展开为空。

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
7. 并发准入：并发发起 25 个登录，20 个成功，其余在 10 秒内被拒并返回 429 或 503 对应码。
8. 设备未登记与设备被注销后的访问拒绝。
9. 逐法人探测的授权法人清单：用户被授权 A 未被授权 B 时清单只含 A。
10. 重新认证挑战：摘要不符、过期、重复消费、非本人核销四条拒绝路径各一条。
11. 高风险请求：六类操作各跑一次从建单到重新认证通过；移动端 X-Client 在 POST /api/v1/platform/reauth-challenges 上发起四类受限操作被拒。从提交到执行一段，以及发起人尝试审批被拒与跳过节点被拒两条，属开篇同批清单第一项，随阶段 3b 一并执行。
12. 审批链配置校验：自审配置保存被拒且错误消息含冲突节点号；节点空洞被拒；职责冲突的角色授予被拒。
13. 配置版本生效：切换版本后判定结果随之改变，且切换前后不存在既非旧版又非新版的中间结论（用并发读验证快照整体替换）。
14. 应急账号：启用需独立复核人；允许的三类操作成功；业务写入、审计策略修改、密钥操作、常规业务审批四类被拒并告警；8 小时到期自动失效；关闭后凭据轮换结果写入台账与审计。
15. 账号停用：停用后会话立即不可用、设备凭据失效、发出 platform.user_account.deactivated.v1。
16. 许可受限运行状态下账号停用、口令重置、凭据轮换、权限回收四项仍可执行，对应规格第 3.4 章；许可状态一律经阶段 3b 按 A-05 交付的 `ep_platform_license::ModuleLicenseQuery::license_status` 读取，本阶段不自建第二套许可判定。
17. 幂等：同一 Idempotency-Key 重放返回首次结果并带 Idempotent-Replay 头；载荷不同返回 409。
18. Outbox：五个事件的信封字段齐全，security_level 与 data_scope_tags 非空，posting_date 为 null。
19. 快照重载与降级：写入一版新的 EFFECTIVE authz_config_versions 后，core-server 在一个轮询间隔内换上新快照；把该版本的 checksum 改坏后重启，进程不退出、判定沿用上一版快照、DegradationLedger 中出现一个未关闭的 AUTHZ_SNAPSHOT_CHECKSUM_MISMATCH 窗口。

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
3. 六类高风险操作：逐类验证重新认证、审批链不可越权跳过、申请人不可自审，且认证方式、待签内容摘要、时间与设备四项写入审计证据。四端口径一致，按规格第 6.2 章矩阵，桌面端全量、移动端按“财务过账与期末结账”“收付款登记与对账查看”“发票申请与开具登记”三行取仅查看，验证移动端不提供提交入口且给出转桌面端说明。

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

1. 29 个迁移文件在空库上按文件版本号全序离线执行成功，24 张表与 13 条行级策略全部建立，本阶段 11 张不带法人列的表在 platform_core.unpoliced_table_registry 中各有一行登记且 db/checks 的第十三项返回零行，platform_core.schema_history 记录完整。
2. core-server 与 job-worker 以 --check 模式退出码为 0，基线第 7.3 节的十项中除 `audit-chain-verifiable`、`file-store-writable` 与 `offsite-sink-requirements` 三项外全部通过，本阶段的 authz-snapshot-loadable 一项亦通过；该三项的承担阶段均晚于本阶段（前两项归阶段 3b，末项归阶段 14），按基线第 12 节通则第六条以换判据处置，在本阶段返回 `NOT_APPLICABLE` 并在报告中标注承担阶段，既不计入通过也不计入违反，不构成本条的阻断项；且 platform_ops.degradation_windows 中没有未关闭的 AUTHZ_SNAPSHOT_CHECKSUM_MISMATCH 窗口。
3. 一条端到端脚本可在干净环境上完成：建号、开通、注册第二因子、登记设备、登录、选择法人、访问一个受保护端点、发起一次合同生效类高风险操作、完成重新认证并取得 X-Reauth-Token，全程无人工干预。脚本止于重新认证通过。提交、两级审批与留下审计证据三段属开篇同批清单，本阶段不以任何替身或假实现跑通，也不在本阶段声称可演示，三段与阶段 3b 一并实现并在阶段 3b 结束时一次判定。
4. 本阶段交付的 matrix_32.rs 的 32 组交叉用例全部通过，入口借用测试在本阶段只有 4 项且全部通过，内部对账上下文一项按第 8.3 节整条归阶段 9a、不计入本阶段判定也不登记顺延，输出的结构化报告中越权读取、越权写入、跨法人聚合泄漏三项计数为零，发布门禁项 RG-RLS-MATRIX-GREEN 判定为绿；该门禁的判据按基线第 3.8 节为 platform_core.unpoliced_table_registry 的行数与 tests/rls_matrix 中承接入口用例数相等且全绿，本阶段交付的 matrix_32.rs 是其中一段。
5. 规格第 17.2 章“身份与访问控制测试”条目的三段判据逐句有对应用例且全部通过。
6. 六类高风险操作各自的重新认证允许路径与拒绝路径均有用例且通过，拒绝路径至少覆盖：摘要不符、令牌重复消费、非本人核销、挑战过期、移动端发起四类受限操作。未重新认证提交、发起人自审与跳过节点三条落在提交与审批段，属同批清单第一项，随阶段 3b 判定。
7. 五类管理员职责互斥在配置期与运行期各有一条拒绝用例通过；不存在任何一个角色或用户可以同时命中两个互斥职责类，由第 4.5 节的同一份纯函数在角色授予与用户绑定两条写入路径上拒绝，另有一条集成用例断言种子角色包在这两条路径上不产生任何冲突，不再由启动自检在真实种子数据上验证。
8. 权限项注册表中不存在“关闭或修改法人隔离机制”与“事务业务库直连”两类权限项，由 permission_items 上的 ck_permission_items_forbidden_codes 保证其写不进去，并有一条集成用例断言该约束拒绝这两类编码的写入。
9. 字段级受控只读视图端点在基准数据集上 P95 不超过 2 秒，EXPLAIN 输出无 Seq Scan，证据入库到测试证据目录。
10. 授权判定 P95 不超过 1 毫秒，指标可在 127.0.0.1:9101 抓到。
11. crates/platform/identity 与 crates/platform/authz 的行覆盖率均不低于 85%，工作区整体不低于 80%。
12. 依赖方向自检脚本通过：两个新 crate 不出现对 domain、application、adapter 的依赖，ep-platform-authz 不依赖 ep-platform-identity。
13. docs/error-codes.md、docs/event-catalog.md 与数据字典的本阶段增量已提交，CI 的错误码一致性校验与事件登记校验通过。
14. 本阶段的 3 处偏离项与 10 处新增决定已写入基线修订提案并经平台架构负责人签字，未签字项在计划中标注为阻塞。
15. clippy 以 -D warnings 通过，非测试代码中不出现 unwrap、expect、panic!、数组越界索引与整数溢出运算；单文件不超过 800 行、函数不超过 50 行、嵌套不超过 4 层。
16. 按 A-19 应交付的三个 applier 已在 ep-platform-authz 实现：AuthzRoleApplier、AuthzPolicyApplier、AuthzFieldGrantApplier，三者实现阶段 3a 提供的 ConfigItemApplier 端口并注册到 ConfigItemApplierRegistry，单元测试覆盖三者的写入与版本推进在同一事务内完成。配置包经发布通道审批签名后生效属开篇同批清单第二项，随阶段 3b 一并判定，本阶段不登记顺延项。
17. 本阶段全部路由在注册处各带一个 `(CapabilityDomain, ActionClass)` 元组，`crates/platform/authz/src/capability.rs` 这个文件不存在，`xtask configdoc` 通过。
18. 本阶段不交付任何业务界面。A-23 的四端界面按规格第 6.2 章能力矩阵由阶段 5 至阶段 12 各自交付，客户端壳、路由注册表与能力矩阵闸由阶段 13 交付，本阶段只交付服务端端点与其契约。
19. 按 A-15 的实现清单，MasterReferenceCounter、SalesTradeHistoryProvider 与 PurchaseTradeHistoryProvider 三个 trait 的实现方不含本阶段，本阶段不实现也不注册，注册表由阶段 5 提供。
20. 敏感字段登记表在本阶段的迁移与建表语句中不存在，本阶段对 platform_core.sensitive_field_registry 只有读取路径，由一条集成用例断言本阶段代码不含对该表的 INSERT、UPDATE 与 DELETE。
21. 第一批 T0 底座可独立验证：开篇所列 15 张表已建，四条链路各有一条用例通过，即口令登录与设备登记、安全上下文建立与 app.legal_entity_id 写入、授权判定第一阶段与第二阶段、ContractEffective 与 InvoiceIssue 两类高风险操作的重新认证与单节点审批链定义的静态校验；ep-datagen 的 T0 最小样本可一次生成 1 个法人、1 个操作员账号、1 个设备、1 个业务角色与 4 条单节点审批链，四条的 scenario 依次为合同生效、发票申请单、销项发票开具与资金账户建档，且该样本不依赖默认 scale 数据集。
22. apps/core-server/src/wiring/ 与 apps/job-worker/src/wiring/ 两个目录下的全部文件中不出现以 Noop、Stub、Fake、Dummy 四类前缀命名的类型，由阶段 1 随 xtask 交付的 archcheck 规则 unwired-absent 断言且在本阶段构建中通过，本阶段不产生任何顺延登记项；开篇同批清单五项在本阶段的退出条件中均不被宣称达成。

---

### 10. 与规格和 PRD 的对应

#### 10.1 规格条目

| 规格章节 | 本阶段实现的条目 |
|---|---|
| 5.1 平台内核 | RBAC、ABAC、职责分离和审批授权：按法人、部门、岗位、项目、客户、记录和字段判定访问，审批链不可越权跳过，申请人不可自审；首版不含临时授权与策略模拟（本阶段不实现，见第 11 节预留） |
| 6.2 一致性与兼容 | 硬件认证条目：桌面端本机 USB Key 与智能卡、移动端硬件绑定且私钥不可导出的凭据；六类高风险操作的重新认证与审计证据要求四端一致 |
| 6.3 本地缓存与设备 | 设备须先完成登记才能访问业务数据；远程注销；退出登录与设备注销时清除本地缓存与凭据的服务端触发点 |
| 7.7 法人行级隔离机制 | 统一安全上下文的建立；服务端按用户与设备的授权法人集合校验调用方声明的法人；连接归还前清除安全上下文；法人越权测试集八类；两个复制角色与内部对账上下文的入口借用测试 |
| 7.9 派生存储安全继承 | 权限模型、密级规则变更时发出事件驱动派生存储重建或重新打标（本阶段发事件，重建由派生存储阶段执行） |
| 12.1 身份与认证 | 内置员工本地账号目录、批量建号、入职开通、调岗改权、离职停用；口令复杂度、有效期、失败锁定与 MFA 策略；管理员与高风险角色强制 MFA；七种认证方式；六类高风险操作的重新认证；认证方式、待签内容摘要、时间与设备写入审计证据；受控应急本地账号的四要素、允许操作集合、8 小时上限、用后轮换与 12 个月闲置轮换；用户、服务、设备与插件的独立工作负载身份中的用户与设备两类 |
| 12.2 授权 | RBAC 加 ABAC；法人、部门、岗位、项目、客户、记录与字段级权限；职责分离与审批授权；审批人不得与发起人为同一人；策略默认拒绝；不设全能超级管理员；五类管理员职责分离 |
| 12.5 审计 | 本阶段全部安全相关事实写审计事件并与业务变更同事务，含审批、授权变更、重新认证、敏感导出触发、应急账号启用与轮换 |
| 15.1 错误分类 | 权限或策略拒绝提供可理解原因但不泄露无权数据；每条错误含关联编号、发生时间、可否重试与处理建议 |
| 16 性能与容量 | 20 名合计并发的会话准入；常规交互 P95 2 秒适用于字段级受控只读视图加载 |
| 17.2 自动化测试 | 身份与访问控制测试整条；单元测试与领域属性测试的覆盖率门槛；集成与契约测试中的平台契约部分 |
| 17.3 强制不变量 | 权限不能跨法人、字段或密级越权 |
| 3.4 订阅许可生命周期 | 账号停用、口令重置、凭据轮换与权限回收在任何许可状态下保持可用，不计为业务写入 |
| 附录 A.1 | 常规交互项“字段级受控只读视图加载”；提交类中六类高风险操作的重新认证服务端校验往返计入时延 |

#### 10.2 PRD 条目

| PRD 节 | 本阶段实现的功能 |
|---|---|
| 10.1 适用角色与职责分离 | 八类角色的定义与五类管理员互斥；不设全能超级管理员；首版身份来源只有内置目录 |
| 10.2.1 七个判定维度 | 七个维度全部实现，映射进基线第 11.3 节的四阶段顺序；密级作为属性参与过滤 |
| 10.2.2 配置对象与配置操作 | 角色、访问策略、字段权限与密级三类配置对象；权限项粒度为模块功能点加六个动作；四条保存期校验；异常提示指出被拒绝的规则名称；配置进入配置发布流程后生效 |
| 10.2.3 用户账号的生命周期操作 | 批量建号、入职开通、调岗改权、离职停用四行逐行实现，含各自的校验与结果 |
| 10.2.4 权限拒绝的用户可见行为 | 三条全部实现，含排序位次与分面计数不泄露、列表可见而明细无权时不返回部分内容 |
| 10.3.1 清单与触发点 | 六类高风险操作的枚举、发起角色、审批归属与移动端取值 |
| 10.3.2 重新认证的交互流程 | 五步全部实现，含待签内容摘要的五项内容 |
| 10.3.3 状态机 | 十一个状态与全部流转逐条实现 |
| 10.3.4 异常与失败提示 | 六条全部实现并各有错误码 |
| 4.9.2 门户访问与数据约束 | 门户账号与员工目录分属不同身份来源；门户账号不得被授予内部角色 |
| 4.10 权限与职责分离 | 付款申请提交人不可作为该申请的任一审批节点 |
| 11.2 并发与规模上限 | 20 人合计并发的准入实现 |
| 11.7 访问入口约束 | 门户会话与内部会话的分离；门户账号的法人范围限定 |
| 附录乙 U-B-05 至 U-B-18 | 逐条给出临时取值，见第 12.3 节 |

---

### 11. 风险与预留

#### 11.1 技术风险

1. 四端认证方式的一致性风险。WebAuthn 在 Tauri 桌面壳内的可用性依赖各平台 WebView 的实现，智能卡与 USB Key 在桌面端需经原生插件访问 PKCS#11，而规格第 9.3 章允许客户关闭原生插件加载。缓解：认证方式做成可插拔的 CredentialVerifier，任一方式不可用时按 PRD 第 10.3.4 节提示改用其他已登记方式，且服务端在用户只剩一个可用因子时拒绝注销该因子；四端 PoC 的首测执行按己-3 的裁定由阶段 13 承接，阶段 1 不再产出任何覆盖 USB Key 的证据，原先“阶段 1 的四端 PoC 若未覆盖”这一前提在本阶段恒为真、不构成条件，现删去该前提改为无条件补测：本阶段在第一周内补一次 USB Key 的真机验证，未通过则把 USB Key 降级为可选方式并登记为交付差异。USB Key 属桌面端外设，按规格附录 C.3 第四条，桌面端外设门槛未通过时优先经规格第 9.3 章的桌面端签名原生插件补齐，不触发客户端 UI 技术栈切换，因此本项验证的结论不进附录 C.3 第二条的切栈触发项。
2. Argon2id 与单机 CPU 配额的冲突。65536 KiB 加 3 轮在 20 并发登录风暴下会短时占满 app-core.slice 的 CPU 份额，进而拖慢在途业务事务。缓解只有一条：按第 8.5 节实测下调 memory 参数直到单次校验落在 120 毫秒以内。原拟的登录用例第二道并发信号量删除，登录与业务共用第 6.5 节已有的准入信号量，理由是上限 20 人的部署再分设一个只管登录的仲裁器，多出的是一套独立的排队与超时语义和一处新的死锁面，换不到任何隔离效果，因为两者本来就跑在同一个 slice 上。
3. 记录级谓词下推导致查询计划退化。部门闭包与项目集合进入 WHERE 后，规划器可能放弃 ix_<table>_legal_entity_id_created_at 转为顺序扫描。缓解：设 IN 列表阈值并在超阈值时改用 EXISTS 子查询；本阶段对 platform 自身三个对象类型给出 EXPLAIN 证据，业务对象的证据由其所属阶段在接入时给出，判据写入 object_scope_bindings 的登记流程。
4. 授权快照与配置发布之间的一致性窗口。快照重载是轮询的，从配置版本生效到 core-server 换上新快照之间存在一个不超过 EP__AUTHZ__SNAPSHOT__POLL_INTERVAL_MS 的窗口。缓解：判定结果携带 snapshot_version，审计事件记录该版本号；窗口上限由该配置项界定并作为观察项进指标。原先的事件驱动加进程间接口同步通知加轮询兜底这三件套删除，只留轮询一条，理由见第 2.3 节：三条路径合起来把一个上界确定的小窗口换成了一条会失败、会积死信、要重投的链。该窗口对派生存储的影响按规格第 7.9 章由派生存储阶段承担，不在本阶段闭合。
5. PRD 附录乙 U-B-01 与 U-B-02 未决导致种子角色包返工。本阶段的种子角色包只包含五类管理员与一个最小业务角色，业务角色包留空。风险是实施期无可用基准；缓解是把角色包做成配置发布包而不是迁移，决策落地后由配置包补齐，不需要改代码与改表。
6. 身份主体表不带法人列这一偏离，若在评审中被否决，改造代价为：9 张表加列、加策略、登录路径改为先查一张不带法人的登录目录表。该改造集中在 identity crate 与其迁移，估计影响 6 个文件，不外溢到 authz 与业务模块。这是本阶段最大的一处返工风险，因此第 12.2 节把偏离的理由与补偿控制写全，供评审一次性裁定。
7. 门户账号的法人范围问题（U-B-11）未决。同一供应商同时与两个法人交易时门户账号能否跨法人查看尚无结论。本阶段实现为门户账号与员工账号共用同一套法人授权集合机制，因此两种取值都能承载，不构成阻塞。
8. 阶段顺序按裁定通则第四条固定为 1 → 2 → 3a → 4 → 3b-1 → T0 → 5 → 9a → 8 → 6 → 7 → 10 → 11 → 9b → 14 之后，本阶段落在流程引擎、配置发布通道、审计哈希链与通知投递四项本体之前。原先的缓解是注入四个 Noop 前缀空实现并把验收顺延，该做法已废止：行为固定为返回成功且不产生任何副作用的空实现，会让审批放行与审计留痕在本阶段看起来成立，而这两件事恰恰是本阶段安全性最高的演示，把它押在四个静默返回成功的空壳上是最坏的一种形态；一旦某一行替换遗漏或被回退，系统会安静地产出不完整的安全事实，而唯一的守卫是一句注释。现处置是同批交付，开篇已逐项列出五项清单，本阶段不注入任何替身、不登记任何顺延项、不在退出条件中宣称这五项可演示，它们与阶段 3b 一并实现并在阶段 3b 结束时一次判定。残余风险是阶段 3b 的排期直接决定这五项的完成时点，控制手段是 3b-1 批紧接本阶段、两段之间不插入其他阶段，且两段合起来仍排在贯通线 T0 之前；同批清单前四项所依赖的审批流程实例、最小发布通道 Draft 到 Released 的一条直路、审计哈希链与段行、同事务站内通知四者都在 3b-1 批之内，第五项的模块许可状态读取按阶段 3 计划落在 3b-2 批，按其下游拉动点就位，不构成 T0 的前置。
9. 内部对账系统安全上下文借用测试的被测对象按 A-06 由阶段 9a 交付，而 9a 排在本阶段之后。处置是整条移交而非顺延：本阶段不建假执行器、不跑该项、不留登记，判据与断言函数已由阶段 2 冻结，阶段 9a 交付执行器后直接编入其退出条件。这样本阶段少一处假实现，也少一条要跨六个阶段记着的顺延项。

#### 11.2 为后续阶段预留的扩展点

1. 企业身份联合。CredentialVerifier 与 PrincipalResolver 两个 trait 是 AD、LDAP、OIDC 接入的位点；user_accounts 预留 account_kind 枚举的扩展空间。首版不实现、不验收。
2. 临时授权与委托代理。user_role_grants 与 user_scope_grants 已带 effective_from 与 effective_to 两列，恢复该能力时只需放开写入路径与增加一个授予来源列，不需要改判定流水线。
3. 策略模拟与影响分析。POST /api/v1/platform/authz-decisions/actions/evaluate 已具备对任意主体求值的能力，模拟只需在其上加一层“以候选配置版本求值”的入参，判定内核不变。
4. 仓库维度（U-B-10）。若安全负责人决定新增仓库维度，落点是 object_scope_bindings 增加一列 warehouse_col 与 user_scope_grants 的 scope_kind 增加一个取值，但规格第 12.2 章的七个维度必须先修订，PRD 层不得自行增加维度。
5. 破窗授权流程。受控应急本地账号的 allowed_action_set 是一个 text[] 加 CHECK，通用破窗恢复时可放宽该 CHECK，但规格第 12.1 章明确首版不含通用破窗，因此本阶段不预留 API。
6. 字段级加密的覆盖面。解密位点本身不是预留项：按 A-28 首版已有实际加密字段，口径与字段投影路径上的唯一解密位点见第 4.7 节，SensitiveFieldDecryptor 的实现基于阶段 2 在 `ep_foundation::port::kms` 定义的 `KmsBackend`，载体实现留在 ep-adapter-kms，实例由 apps/core-server/src/wiring/ 与 apps/job-worker/src/wiring/ 两个目录下的装配注入，本阶段不自建第二套解封路径。预留的是覆盖面，U-A-12 决策把开户银行或其他字段改为字段级加密时，只增加登记行与物理列，本阶段的投影器与判定流水线不改；盲索引与受控投影按规格第 7.8 章由其所属阶段建设。
7. 移动端“把任务发送到桌面端继续”（U-K-08）。high_risk_requests 的 CLIENT_NOT_ALLOWED 错误响应中预留 advice 字段承载跳转说明，产品决策后只改文案不改逻辑。

---

### 12. 本阶段新增决定与偏离项

按基线第 0 节与第 12 节的要求，本节把本阶段自行决定的事项与偏离基线的事项集中列出，全部需要回写基线。

#### 12.1 新增决定（基线未覆盖，本阶段取值）

1. 身份主体表归 platform_core，授权表归 platform_authz；敏感字段登记表按 C-06 唯一落在 platform_core.sensitive_field_registry，由阶段 2 建立，本阶段只引用不建表。回写基线第 3.1 节。
2. 平台内核端点的模块段取 platform，路径为 /api/v1/platform/...。回写基线第 5.1 节。该段与已有的 /api/v1/portal/... 同类。
3. 平台事件的模块段取 platform，事件名如 platform.authz_policy.published.v1。回写基线第 6.1 节。
4. 平台事件的 posting_date 与 accounting_period_id 取 null；关账受理前提二的判定语句按 C-28 由阶段 9a 定死，本阶段第 6.3 节逐字采用，posting_date 为空的平台事件一律不计入。回写基线第 6.1 节。
5. 本阶段的两张仅追加表（user_password_history、login_attempts）不在基线第 4 节列举的六类之内，两表均无冲销或更正语义，因此按仅追加处理且不带 reverses_id，取舍与理由已逐表写在第 3.2 节表 3-3 与表 3-7 的定义处。回写基线第 4 节，把该节仅追加表一条改为：仅追加表一律不带 row_version、updated_at、updated_by；是否带 reverses_id uuid null 由该表有无业务冲销或更正语义决定，有的必须带并在表定义处写明它指向哪张表的哪条记录，没有的不得为满足列约定而保留恒为 NULL 的该列；取舍与理由由所属阶段在其表定义处逐表写明，该节不再列举表名。
6. 启动自检新增 authz-snapshot-loadable 一个命名项，按 C-25 以注册名标识，不用序号；判据只到可构造出完整 AuthzSnapshot 为止，checksum 不一致按降级窗口处理不阻断启动，窗口 kind 取 AUTHZ_SNAPSHOT_CHECKSUM_MISMATCH，该取值不由本阶段定义：DegradationKind 的唯一定义方是阶段 2，终态取值清单的唯一出处是阶段 14，本阶段只使用不扩枚举。原拟的 duty-class-exclusivity 与 forbidden-permission-items-absent 两项不登记，分别下沉到角色授予与用户绑定的写入路径、以及 permission_items 的 ck_permission_items_forbidden_codes 约束。回写基线第 7.3 节，并在该节写明启动自检的正当判据是这个二进制能否在这台机器上正确运行，数据是否一致属运行期不变量，不进阻断级。
7. 新增指标十个：ep_authn_login_attempts_total、ep_authn_active_sessions、ep_authz_decision_duration_seconds、ep_authz_denied_total、ep_authz_scope_truncated_total、ep_reauth_challenges_total、ep_high_risk_requests_open、ep_breakglass_active_sessions、ep_session_admission_queue_wait_seconds、ep_session_admission_rejected_total。标签只用 legal_entity_id、operation_type、outcome、reason 四类，不用 user_id 与 doc_no。回写基线第 9.2 节。
8. 登录不设第二道并发信号量，登录与业务共用第 6.5 节的准入信号量，原拟的登录并发上限 4 撤销。回写基线第 11.6 节，该节只保留 20 人合计并发这一处仲裁，不新增第二处。
9. 权限动作枚举固定为 VIEW、CREATE、UPDATE、SUBMIT、APPROVE、EXPORT 六个，不多不少。回写基线第 11 节。
10. SecurityContext 十九个字段中的 DeviceId、RoleCode、DutyClass、RecordShare、DataScopeTag、RequestId、TraceId 七个类型，其形状与取值域由第 4.1 节给全，按 A-03 由阶段 1 与该结构体同处冻结在 ep-foundation，本阶段只填充不定义。回写基线第 1.4 节安全上下文一段与裁定 A-03 的提供方一句，两处的交付范围由该结构体与三个配套枚举改为该结构体、三个配套枚举与这七个类型。

#### 12.2 偏离项（与基线冲突，需批准）

偏离一。基线第 3.8 节原写的不带 legal_entity_id 的表只有四类这句封闭枚举，与其中无定义、容量无限的全局配置字典这一类名，一并作废，改为正向登记制：凡带 legal_entity_id 的表一律按模板建策略；不带该列的表必须同时给出准入判据与隔离承接点两项并逐表登记。本阶段据此登记 11 张表，登记行落在阶段 2 交付的 platform_core.unpoliced_table_registry，由第 3.5 节第 29 号回填迁移一次写入，未登记的表按 db/checks 第十三项判为违规而建不出来，本阶段不再以第五类例外的形态申报。准入判据统一取该表的行集合与法人无关这一条，逐组核对如下：platform_core 的 9 张身份主体表是隔离机制自身的元数据，只承载身份主体、凭据引用、会话与设备，用户是可被授权多个法人的主体，给其行贴单一法人标签在语义上不成立，且会使登录路径在建立法人上下文之前无法读取账号与凭据，从而被迫引入一条绕过行级策略的读路径，而绕过是基线与规格第 7.7 章都不允许的；platform_authz 的 permission_items 与 object_scope_bindings 两张的行在本部署内对两个法人取值相同。隔离承接点逐组写明：9 张身份主体表的法人可见性落在任何列出用户的查询一律与受策略约束的 platform_authz.user_legal_entity_grants 内联这一条上；permission_items 与 object_scope_bindings 两张不返回任何与法人相关的行，其可见性由授权判定第二阶段的对象级判定承担。核验方式：tests/rls_matrix 对这 11 张表的每一个 API 出口单独设越权用例，判据与其余表相同，并逐表断言其可见性不随 app.legal_entity_id 变化，取值一变即失格、必须补法人列。影响范围：基线第 3.8 节、第 3.10 节基线索引约定、第 4 节公共列。

偏离二。基线第 5.6 节规定 Authorization 与 X-Legal-Entity-Id 必填，第 5.4 节规定全部写请求 Idempotency-Key 必填。本阶段对三个认证前端点（sign-in、complete-mfa 之前的 sign-in、legal-entities 列表）豁免这三个头中的对应项。理由：这三个头在认证完成前不可能存在。补偿控制：白名单为固定三项的编译期常量，不可配置；豁免端点改用按登录名与来源地址的双维度速率限制并写入 login_attempts。影响范围：基线第 5.4 节、第 5.6 节。

偏离三。基线第 3.10 节规定每张业务表的基线索引固定为三条，其中含 ix_<table>_legal_entity_id_created_at。9 张不带法人列的表改为 ix_<table>_<主查询列>_created_at。这是偏离一的连带项，不单独申请。影响范围：基线第 3.10 节。

#### 12.3 PRD 附录乙待决事项的临时取值

| 编号 | 临时取值 | 是否阻塞 | 切换代价 |
|---|---|---|---|
| U-A-09 | 批量建号逐行落库，失败行退回并给出行号与原因 | 否 | 改为整体回滚只需改一个用例的事务边界 |
| U-B-01、U-B-02 | 只交付五类管理员角色与一个最小业务角色的种子包，业务角色包留空 | 否 | 角色包是配置发布包，补齐不改代码 |
| U-B-05 | 显式拒绝优先，求值顺序按基线第 11.3 节 | 否 | 基线已定死，此处只是确认 |
| U-B-06 | 字段权限四值 HIDDEN、MASKED、READ、WRITE；掩码风格三种 FULL、KEEP_LAST_4、KEEP_DOMAIN | 否 | 增删取值需改一个枚举、一条 CHECK 与投影器的一个分支 |
| U-B-07 | 记录级授予按责任人、创建人、流程当前处理人与显式共享四种；共享不可再转授，can_reshare 由 CHECK 固定为 false | 否 | 放开转授需放宽该 CHECK 并增加一条授权链深度校验 |
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
| U-L-01 | 并发定义为最近 60 秒内有请求的不同用户数；达上限排队，等待 10 秒超时返回 503 | 否 | 改为不限制只记录需去掉信号量并保留计数器 |
| U-A-12 | 该项待决，裁定表不代拍，待决范围只有三问：开户银行是否同列敏感字段清单、列表与详情与导出三场景的脱敏形态、导出是否触发重新认证；银行账号的纳入与其字段级加密按规格第 7.8 章强制落地，不在待决范围内。技术侧临时取值按 A-28：`mdm.customer_invoice_profiles` 与 `mdm.supplier_payment_profiles` 的 `bank_name` 与 `bank_account_no` 共四行登记为 ACCOUNT 类且密级 30，`bank_account_no` 两行 is_field_encrypted 取真、mask_style 取 KEEP_LAST_4 且后四位取自 `bank_account_no_tail`，`bank_name` 两行取假、mask_style 取 NONE；导出是否触发重新认证按 U-B-18 的判定函数计算，该函数对这四列判真 | 否 | 登记行是数据行，取值切换按 A-28 的切换路径在一次变更内完成，本阶段的字段投影器与 U-B-18 判定函数不改 |

以上 17 条均不阻塞本阶段实施。原先登记的唯一阻塞项已解除：SecurityContext 的 19 个字段、三个配套枚举与第 4.1 节给全的七个字段类型按 A-03、SYSTEM_PRINCIPAL_ID 与 SYSTEM_DEVICE_ID 按 A-02、CapabilityDomain 与 ActionClass 按 A-20，均由阶段 1 在 ep-foundation 冻结并排在本阶段之前，本阶段只负责填充与引用，本计划不再存在需要标注为阻塞的前置项。
