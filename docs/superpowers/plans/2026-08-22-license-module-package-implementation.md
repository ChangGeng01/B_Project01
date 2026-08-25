# F-56 许可证与签名模块包实施计划

> **F-57 状态：`SUPERSEDED_DO_NOT_EXECUTE`。** 本文件只保留为历史设计输入，不得单独或续跑执行；F-56 的许可证包络与模块数据保留原则已并入 F-57。任何实现必须从 `2026-08-23-f57-governed-automation-fabric-implementation.md` Task 1 开始。

状态：**开发执行清单已冻结；本文件不表示已经实现。**
设计依据：[F-56 许可证与签名模块包终态冻结](../specs/2026-08-22-f56-license-signed-module-package-freeze.md)

## 1. 边界与完成定义

本计划只实现 F-56：不新增服务、端口、数据库或迁移编号，不引入动态本机代码/SQL 模块包，不改变 WASM、MCP 容器或 AI 模型包通道。完成定义是 Stage 3b/13b/13c/14 的现有施工点按同一契约闭合，并取得 `RG-LICENSE-MODULE-LIFECYCLE-GREEN`；未取得真实证据前只能称实现候选，不能称可发布。

## 2. 固定文件与依赖顺序

唯一顺序：

1. Stage 3b 更新 planned 090100/090200/090500/093300，令 Stage 3 `ItemKind::ALL` 与建表 CHECK 恰为 18 项；
2. Stage 13b 的 `V20261022090500` 再把 `ItemKind::ALL` 与 CHECK 从 18 原子扩为 20 项；
3. `ep-platform-license` 实现 strict parser、CMS verifier、可信时间、四态、两个 applier 与查询接口；
4. `core-server`/`job-worker` wiring 注册两个 applier，并接运行期受限运行和模块过滤；
5. `epcfg` 与 ServerAdmin 只复用既有配置包 API；
6. Stage 13c 的 AI/MCP gate 改读 F-56 entitlement；
7. Stage 14a0 的通用证据基础可在本线之前独立完成；本线 Stage 3b-2 类型/运行时到位后，Stage 14a1 才实现只读 F-56 projection/common-gate adapter，随后 Stage 13c 消费；terminal20、配置包全链、PreF55 与真实 PASS 继续等待 Stage 13b，并只由 Stage 14b 最终判定。

任何后一步不得用假行、固定 true、旧 F55 payload 或临时环境变量越过前一步。

## 3. Task 1：冻结 schema 与 migration assertions

未来实现时修改既定文件：

- `db/migrations/platform_core/V20261013090100__platform_core_create_module_registrations.sql`
- `db/migrations/platform_core/V20261013090200__platform_core_create_license_grants.sql`
- `db/migrations/platform_meta/V20261013090500__platform_meta_create_config_package_items.sql`
- `db/migrations/platform_meta/V20261022090500__platform_meta_alter_config_package.sql`
- `db/migrations/platform_meta/V20261022090600__platform_meta_config_release.sql`
- `db/migrations/platform_core/V20261013093300__platform_core_backfill_stage03_unpoliced_table_registry.sql`

先写失败测试，逐列断言 F-56 第 3.2、4.3 节，Stage 3 exact-set=18、Stage 13 exact-set=20，特殊 item 单项形状、current slot 唯一、self-FK、grant 的 `governance_legal_entity_id`/FK/同 deployment 永久一致，以及 090500 建立 nullable `accepted_trust_bundle_sha256`、093300 才建立 `UNIQUE(config_package_id,id)`、六条 grant/revocation/module source FK、接受摘要一次写入/发布态/普通项恒空/跨表相等约束。090100 fresh migration 必须按 F-56 exact UUID/display catalog seed 恰 15 个 NOT_INSTALLED module row，禁止稀疏表；093300 deferred graph 还须强制 RELEASED history 中 `package_id` 与 `(module,package_code,semver)` 各自一一映射同一 exact inner。Stage 13 的 090600 必须逐项证明固定管理 API 的 30 个 permission item、12 个 object-scope binding 与 Stage 13 §3.2.10 ID/code/action/object/真实表映射完全相等，且 `role_permission_grants` 自动 seed 为零；bootstrap 使用的 10 个 permission-action pair 只能从这份完整目录解析。然后才写 SQL。fresh database 全量迁移必须通过；不得生成新 SQL 文件或改任何版本号。

## 4. Task 2：实现 strict artifact 与离线验签

在 `ep-platform-license` 内建立唯一 DTO/parser：

- `SignedBusinessArtifactV1<T>`；
- `LicenseGrantPayloadV1`、`LicenseRevocationPayloadV1`；
- `ModulePackageManifestV1`、`ModulePackageItemV1`；
- `ProductModulesManifestV1`、15 行 `ProductModuleContractV1`、`ModuleContractDescriptorV1/ModuleAbiEntryV1` 与固定 `C:\EP\product-modules.v1.jcs` safe-handle loader；
- strict object `{major:u16,minor:u16,patch:u16}` 的 `SemVerV1`；
- `LicenseStatus`、`LicenseRestrictionReason`、`LicenseEvaluationV1`、`EntitlementCodeV1`；`ModuleLicenseQuery::license_evaluation` 一次返回同快照的 status/reason/trusted time，禁止调用方拼接撕裂结果。

测试先建立并跑红 F-56 golden PKI/CMS fixture set，再覆盖 JCS byte limit、unknown/duplicate、各数组精确 cardinality/顺序/重复、日期与 scope 条件、canonical base64url、detached CMS 的 one-SignerInfo/SKI sid/content/digest/exact signed attrs/no unsigned attrs/leaf+necessary-intermediates certificate set、ECDSA/RSA-PSS exact params、链/撤销/SPKI、wrong deployment、candidate issued_at 对导入前可信时间的未来偏移和 DEV root。fixture 必须逐项锁死 F-56 首版 leaf/CA/CRL extension exact-set，并拒绝五类 name/policy 扩展、未知/额外 extension、错 critical 位、额外 KU/EKU、IDP/delta/freshest 与 entry extension；不把 crate API 变成规范，最终 crate/version 只由 Cargo.lock/SBOM 固定。签名人的唯一授权输入是已独立验签且绑定同 deployment/build 的 `DeploymentManifestV1.license_trusted_signer_subjects`，恰 1..64 个 canonical SPKI token 并按 UTF-8 bytes 排序去重；它是可识别 identity roster，CAB 新 roster 必须包含数据库全部 RELEASED special inner/source-outer 历史引用 token，删除仍被引用 token 必须在替换前失败。新 artifact 仍要求 roster 命中且链 ACTIVE，保留已撤销 token 只让 CRL 正确归类为 REVOKED、绝不恢复授权。本地 `release.trusted_signer_subjects=[]` 只表示无覆盖，非空只能作与 signed roster 逐项相等的漂移断言。缺/空/乱序/重复 roster、本地非空不等、用本地项增删 signer、历史引用不是 roster 子集或 roster 与 deployment/build 不同都须在 readiness 和发布 gate 前失败。`license-roots.p7b` 只接受 F-56 exact empty-content/no-signer DER CMS bundle、1..64 CA cert、1..256 base CRL、自签 anchor/非自签 intermediate 分类与唯一 path；不得另读 Windows 任意根、联网补链或热替换。每个 RELEASED special source item 的 `accepted_trust_bundle_sha256` 永久保存首次接受摘要，grant 行摘要与其 source 相等，合法轮换不得回填旧行；CAB 必须以同一离线发布批次原子提供新 signed deployment roster 与对应 trust bundle，更新部署清单后、gate 重开前 exact-set 分别复验全部 RELEASED grant/revocation/module item 的 inner/source outer、交叉 current projections，并生成同时绑定旧接受摘要、新验证摘要、signed deployment manifest digest、exact roster 与 inner/outer 分结论的签名证据。current 失败关闭相应运行门；历史 inner 或 outer 明确 CRL REVOKED 只隔离并排除 purchased/rollback/正向证明，其他历史 source/digest/signature/chain 异常保持变更与发布门关闭。普通 KMS outer、special publisher outer 与 inner 验签分别断言。

在仓库建立 F-56 精确列出的 15 个 `contracts/modules/<wire>.contract.v1.jcs` 和 digest 命名的 schema JCS。实现 `cargo xtask module-contracts verify`：重算 JSON Schema/descriptor digest、版本、依赖 DAG，并把每个 `ep-contract-*` compiled `MODULE_ABI_REGISTRY` 与 descriptor entry 做双向 exact-set；由 descriptor 生成 crate 常量和 `product-modules.v1.jcs`，禁止另一个手写常量或依赖 registry。descriptor/schema/compiled registry 任一缺、多、重复、same-version-different-digest 或外部 `$ref` 都构建失败。

## 5. Task 3：实现许可证 applier 与四态

`LicenseGrantApplier` 只接受 imported 单项 ADD。锁属于 whole command transaction，不属于 applier 的迟到补锁：import/autotest/submit/approve/special sign/create-release-order/execute，以及 autotest accept、worker claim/lease/heartbeat/aggregate 的每个短事务，都在 `BEGIN/SET LOCAL` 后以第一条业务 SQL 无条件取得 `pg_advisory_xact_lock(hashtextextended('platform-license-current',0))`，然后才准调用幂等存储、读/claim package 或 applier；typed REJECT 固定不取 license lock且不得先查询再切分支。全部 `LICENSE_GRANT|MODULE_PACKAGE` special config package 在首次 RELEASE 后永久保持 RELEASED，普通配置的自动 `RELEASED→SUPERSEDED` 和通用 rollback 必须跳过它们；多个 special RELEASED 是正确历史，current/history 只看 license/module 投影与 source FK。GRANT/REVOCATION applier 只能在已持有该 exclusive transaction lock 的事务内重读 current/history 与重算全部守卫，不能只锁一条可能不存在的 current 行：

- GRANT：验证首发或直接后继、`governance_legal_entity_id` 首张冻结/后继不变、重建投影、原子关闭旧 current 并插入新 current，初始化 `last_trusted_at=max(pre_import_trusted_now,candidate.issued_at)`；
- REVOKE：锁 current，验证 grant/license/deployment，写 revocation 投影但不删除/替换 grant；
- CRL 恢复：仅当唯一 current 投影/source/digest/inner/source-outer bytes 与历史接受证据仍自洽、失败类别唯一为 current grant/revocation inner 和/或 source outer signer 被 CRL 吊销时，接受 inner+outer 均 ACTIVE、同 deployment/governance 且逐字 supersedes current id 的新 grant；提交前仍 Restricted，任意其他漂移不得借路；
- `revert` 始终返回 `PLATFORM.CONFIG_RELEASE_ORDER.NON_ROLLBACKABLE_ITEM`。

可信时间只按 F-56 单一 `TrustedClockV1` 公式计算：进程启动 anchor 与每次 transaction 候选都纳入 bootstrap committed_at、current/history issued_at、revocation issued_at、全部 `last_trusted_at`、已验 hash-chain 的 checkpoint `trusted_now`、wall 与 monotonic；query 只算。readiness、所有会推进 special package 的关口和目标 cadence 每 240 秒的 job 以同一 exclusive license lock 推进；current 存在时独立 CAS `last_trusted_at`，零 current 不做伪 CAS。UTC slot 唯一为 `floor(unix_seconds(trusted_now)/240)*240` 再发出 canonical RFC3339 whole-second；按 `(action,purpose,deployment_id,slot_utc)` 查询 append-only checkpoint：零行插入入口 snapshot，一行保留 exact bytes 只核对，多行失败关闭；同 slot 后续较新 `trusted_now` 不 UPDATE checkpoint，也不要求与首次值相等。入口 snapshot 必须在锁内重读 current 后、任何 special mutation 前一次捕获，terminal AuditWriter 不得在首发/换槽后重算。零行分支才创建新 UUIDv7，并写完整 envelope：治理法人/SYSTEM grant/null device、`action='LICENSE_TRUSTED_TIME_CHECKPOINT'`、`object_type='platform.license_trusted_time'`、`object_id=deployment_id`、null version/before/reason/approval/reauth、exact snapshot after、system client、`occurred_at=captured_trusted_now`；链列只由 AuditWriter 派生，一行复用不创建事件。耐久键固定 `license-trusted-time:v1:<lowercase-deployment-id>:<slot_utc>`；`trusted_date` 取 UTC calendar date。表驱动测试覆盖 permanent/subscription、`valid_to-60 days` 当日、到期后 1/30/31 日、进程内倒拨不降、零 current audit-only checkpoint、同 slot复用、重复篡改、完整 envelope 漂移、checkpoint/续期竞态、uptime 区间相邻成功值差 `<=300s` 正例与 `>300s` 失败，以及错误前跳被持久化后只能以 Stage 14 可信整库+审计恢复而不能 direct reset；300 秒只作有完整 uptime/audit 证据时的发布观测上限，不宣传为恶意宿主下的硬保证。并发测试覆盖零 current 双首发、同前驱双续期和续期/撤销竞态，必须恰一条合法候选提交，输家稳定返回 special-shape 错误。受限运行只实现 F-56 `crates/platform/license/src/admission.rs` 的 exact 10 值 `LicenseAdmissionEffectV1`、request、`LicenseAdmissionGate::admit` 与三值 binding，不得另设布尔函数。零 current、验签失效、过期或撤销时仍只放行 `LICENSE_GRANT` 全链和 `MODULE_PACKAGE:DISABLE` 全链；有效许可的 LIST scope 未含目标法人时，该法人同样只读/可导出且四类副作用返回 `PLATFORM.LICENSE.RESTRICTED`，但不篡改部署级全局状态。其他常规业务写入/审批/出站/新自动化统一返回该码，证明首次安装和换证不会自锁。

首装另实现既有 `ep-migrate apply` 的三个捆绑参数，绝不增加第六个子命令。三条路径只接受 F-56 的 `C:\ProgramData\EnterprisePlatform\evidence\stage14\initial-governance\<lowercase-deployment-id>\` fixed-root 三文件与 exact DACL。按 strict bootstrap 双 CMS、签名部署清单管理员证书 roster、双离线信任包与 initial `.epcfg` 验证，在 fresh/服务未 readiness 的唯一 PostgreSQL 事务复用默认链 provisioner、身份 Argon2id policy 与审计写者，建立治理法人、signed key-domain id 的 `PROVISIONING` 行、由 deployment/domain 唯一计算的 provider-independent `kek_ref`/version=1、两个不同 MFA 管理员/设备/X509 credential，以及治理法人下 SYSTEM/两 operator 恰三条法人授权。signed device UUID 同时作为 `user_devices.id` 与外部 device_id canonical text，禁止生成第二 id。两个 bootstrap 角色的授权不是 code-only：逐项采用 F-56 第 3.1 节冻结的八对 `F56_CONFIG_OPERATOR` 与两对 `SECURITY_ADMIN` `(permission_item_code,action)` exact-set，插入前验证 permission 行存在且 allowed_actions 含对应 action；缺、多、错 action 全事务回滚。`ApprovalChainProvisioner::provision_defaults` 必须产生 Stage 4 catalog 的 37 条默认链/单节点，不得只建 CONFIG_RELEASE。密码只经 no-echo `ReadConsoleW`。

先写 projection/audit contract tests，再实现该事务：所有 initial-governance 语义摘要复用 F-56 `projection_digest`；authorization registry、内嵌 database-bootstrap projection 及其 legal-entity/key-domain/two-operator/three-entity-grant/two-role/ten-permission-pair/two-user-role-grant/37-chain exact-set 均用同一 DTO。预分配 audit event id、一次捕获 committed_at，构造 exact unsigned receipt 并得 direct-byte digest，再以 `platform.bootstrap.initial_governance.v1` typed audit ABI 的完整 envelope 绑定：signed 治理法人、同法人 SYSTEM grant/null device、固定 action/object/id/version、null before/reason/approval/reauth、exact after、system client 与 committed_at；链列只由 AuditWriter 派生。测试必须证明 JSON number/string、null/缺键、数组排序、37→1、device row/external id分叉、permission pair 缺多、摘要相等但内嵌前像漂移、receipt/audit id/time 不同、任一 envelope 列漂移，以及 password、password PHC/verifier/salt 或其 digest进入任一证据都失败；公开 X509 verifier/SKI 仍按 F-56 exact 字段保留。该事务绝不调用 KMS。提交后只写 unsigned exact receipt，不给 ep-migrate 部署 KMS 能力或 sidecar；core-server 在 public readiness 前以同 signed id 复用阶段 2 唯一 `KeyDomainProvisioner` resume：仅 KEK 按固定 label 幂等 ensure/readback/冲突隔离，DEK 只以 DB wrapped row 为持久真相，补齐并 readback 4 purpose × 4 scope 的 exact 16-row 矩阵后，才在同一数据库事务置 ACTIVE并追加 exact `action='platform.key_domain.activated.v1'` audit。崩溃只允许 same-digest/PE/audit 补 receipt，任何第二次 bootstrap 数据 mutation、force 或不同输入拒绝。Stage 14 typed child evidence 对 receipt exact digest、审计 hash chain、同一 DB projection前像、三条法人授权、37条默认链、initial archive、同域 ACTIVE/activation audit 与首张 RELEASED grant做 exact binding；缺证据不冒充 PASS。

core-server 路由元组、core/worker 非 HTTP registry 必须按 F-56 exact-set 登记 `Fixed|ConfigRelease|McpInbound`，`xtask` 与 Blocking 静态自检比较实际 route/job/event/approval-owner/outbound-operation 集合，缺/多/重复全部失败；自检不得读取 current grant。配置发布八类动态入口必须在 strict target 锁内解析 recovery，`/mcp` 必须从已验签 binding 的 ActionClass 解析效果；任何其他入口不能动态升格 recovery。`InFlightConvergence` 只承接已经发生副作用后的内部终结/取消/无新副作用补偿，首次或重试外发、PENDING/DISPATCHING 待派发、新 claim 均仍按 IntegrationOutbound/AutomationStart 判定。测试必须逐一移除和伪造 binding，并把 registry digest/正反结果交 Stage 14 common gate。

## 6. Task 4：实现模块包 applier 与运行过滤

`ModulePackageApplier` 只接受 imported 单项 ADD，按 F-56 五条合法动作通过 `ModuleOperationGate` 取锁，重验 inner/source outer signature、许可、平台范围、签名产品 manifest 的 DAG 依赖闭包和 contract digest。构建只从 15 个已验证 descriptor/compiled ABI registry 生成 strict `target/release-package/product-modules.v1.jcs`，把它纳入签名 `MANIFEST.sha256`，安装到 `C:\EP\product-modules.v1.jcs`；runtime safe-handle readback 和 Stage14 projection 缺一失败。安装态/动作是部署全局事实，special package 不接受法人范围；业务调用才把当前有效许可按请求法人裁切。INSTALL/新 UPGRADE inner 必须 ACTIVE；ENABLE/DISABLE exact current inner 与 ROLLBACK_VERSION exact historical origin inner 可 ACTIVE|RETIRED-nonrevoked，本次 action outer 始终 ACTIVE。INSTALL/ENABLE/UPGRADE/ROLLBACK_VERSION 都要求当前有效许可含“目标 module + 依赖闭包”，INSTALL 不要求依赖已启用而 ENABLE 必须全部启用。RELEASED history 对 package_id 与 module+code+semver 的 exact inner 一一映射在锁内/commit trigger 双重验证。DISABLE 在受限运行中仍可走恢复链且通常要求 artifact exact 等于当前已安装包；若 current inner 和/或 current source outer signer 后来被 CRL 吊销，则只允许 ACTIVE outer 签名一个 `DISABLE + 原样旧 inner` item，旧两层/接受/source/projection 自洽、失败只为至少一层 REVOKED且另一层 ACTIVE|RETIRED。它只把旧 inner 用作停用目标，绝不能用于 ENABLE/INSTALL/UPGRADE/ROLLBACK。该窄事务除 recovery item 的 accepted event 外，还按 F-56 同一 terminal batch 写唯一 `MODULE_SIGNER_REVOKED_DISABLED` 审计：预分配两个互异 UUIDv7，先写 recovery event、再写 accepted event 且使后者为 batch 最后一条；两者共享治理法人、execute SecurityContext、approval_ref，reason/reauth 均空。recovery event 的完整 envelope 及 accepted event 的完整 envelope 都逐字采用 Stage 3；`audit.before` 保存更新前完整 typed `EP-F56-CURRENT-MODULE-PROJECTION-V1` DTO，`audit.after` 才保存 previous source/recovery 两对 id、before/after projection digest、disabled_at 与 domain-separated reason digest。collector 从 before preimage 重算前摘要，并以 checked row_version+1、disabled state/time 与 recovery reason 的唯一变换重建 after，绝不能只相信两个摘要。event object id/version/time 与派生 after 逐项相等，same-byte 回放不增行，Stage 14 只由该 action/hash chain 和冻结链顺序派生 recovery peer，不能按相同 inner、package id 或最近时间猜选。当前 row 继续引用旧 source，只有 package/inner 投影与旧 source 相等；`last_transition_reason` 必须等于 recovery item reason。停用后才可用全新 ACTIVE inner/outer 的更高版本 UPGRADE 替换。按 F-56 固定各动作时间投影，历史时间不得被清空；`revert` 同样拒绝。

`ModuleLicenseQuery::module_state` 只给 raw 管理投影；`module_is_currently_licensed` 在同一 snapshot 递归要求目标+传递依赖全 raw enabled、各自 current source/accepted digest/inner+outer/product contract trust有效，以及 current grant 覆盖闭包与法人 scope。已知负态返回 `Ok(false)`，IO/parse/source/digest/catalog 歧义返回 `Err` 并由 caller 失败关闭；`feature_is_enabled` 必须经过 owner module effective gate。AI/MCP 平台本身不虚构 ModuleCode，具体业务对象仍由 owner module registry/gate保护。

会产生 `BusinessWrite|BusinessApproval|IntegrationOutbound|AutomationStart` 的 core 普通事务，在读幂等或业务 payload 前先以第一条业务 SQL 取得全局 license shared advisory xact lock，再按 module wire order 取得 owner/dependency shared locks并调用 effective query，所有锁持有到提交/回滚；四类事务可彼此并发，但会被许可替换/撤销的 exclusive 等待排空。worker scheduler/outbox 的 claim 短事务同样先取 license shared；真实外部派发另用专用 session 先持 license shared、再持 module shared，直到副作用/取消终结后 finally unlock，且不跨网络持数据库事务。普通纯读以及 `ReadReportAuditBackupExport|IdentitySecurityDisposition|ComplianceDisposition|InFlightConvergence` 四类允许处置可不取 license lock；`LicenseGrantRecovery|ModuleDisableRecovery` 只决定 Restricted 准入，绝不把 ConfigRelease special 命令从本计划第 5 节的 whole-transaction exclusive 降成 `NONE`。所有多锁按 module wire 排序、总 deadline 30 秒：INSTALL/UPGRADE/ROLLBACK 取目标 exclusive；ENABLE 取目标 exclusive+dependencies shared；DISABLE 固定取全部15个 exclusive但只改目标，从而排空所有反向依赖并在 product DAG 损坏时仍安全。超时整事务回滚并返回 `PLATFORM.MODULE.IN_FLIGHT_DRAIN_TIMEOUT`；崩溃断连自动释锁。依赖 disabled 后 raw dependent 不变但 effective=false，依赖恢复后自动恢复。查询、报表、审计、备份、导出与合规/身份处置负例必须证明未被误拦。

## 7. Task 5：接入配置发布与 ServerAdmin

更新 `ConfigItemApplierRegistry`，在 core/worker 两个 wiring 各注册且只能注册一次。`epcfg` 支持打包和只读验证两种 special item；实现 F-56 exact ZIP32/STORE 三 entry、canonical TOML、`after_spec` exact JCS/item hash 与 detached publisher CMS，拒绝所有容器/重发歧义。普通 item hash 同步补齐 REMOVE 取 before_spec、ADD/MODIFY 取 after_spec，绝不对 null 求摘要。item_code 与 inner identity/action 严格相等；import 解析落行，后续用唯一 canonical writer 从数据库重建 exact manifest/hash，special package 导入后立即不可变，`actions/sign` 只复验并保留发行方 outer signature，不用部署 KMS 覆盖。

同一 import 路由增加可由已认证 Win/Mac/ServerAdmin 使用的 strict multipart 单 `.epcfg` 形态；Win/Mac 原有 attachment-object JSON 保持兼容。按 F-56 固定 boundary/filename/MIME/header/CRLF grammar，framing 恰为 `136+2*boundary_len+filename_len` 且最多 404 bytes，archive 最多 4,193,900 bytes。该路由获得唯一编译期 4,194,304-byte body 窄例外，`Content-Length` 必须在范围内且等于 framing+archive，缺失/非法/为零/超限/任何 Transfer-Encoding 在读 body 前拒绝，流式 body/file 双截止，其他路由继续 1 MiB。所有 transport/archive/canonical syntax 与上限失败按 F-56 稳定使用既有 `PLATFORM.REQUEST.INVALID_PAYLOAD`/400，零配置包落库且不新增 413 码；CMS、signer trust、item hash、typed special 语义各用 F-56 闭集码，模块兼容码不吞签名错误。Restricted 下只允许该 import route strict parse后、exclusive transaction 内从持久候选 exact bytes确认唯一 LICENSE_GRANT 时映射 recovery；普通包/MODULE非DISABLE与通用附件上传仍拒绝。按 F-56 固定 staging 根、DACL、CREATE_NEW、路径拒绝和 finally 删除；另实现 core public readiness 前的 fixed-root 遗留枚举，以及每次同 request-id CREATE_NEW 前的受控 stale recovery：只删除 canonical UUID 文件名、regular/single-link/no-reparse/no-ADS、owner/DACL 正确的非权威遗留，异常对象隔离、告警并阻 readiness；覆盖逐 byte 崩溃、DB commit 前后恢复与幂等重试。ServerAdmin 只组合 import/autotest/submit/sign/release，审批决定仍只在 Win/Mac，ServerAdmin 待办保持只读。`client-bootstrap?client=server_admin` 的 `license_module_admin` 只在具备 `lowcode.config_package.view` 时返回严格脱敏快照；无 current 或签名失效不能回显未受信的许可身份/日期/code/limit，三项实际 usage count 仍返回；15 个 module row 另返回 `NOT_INSTALLED|TRUSTED|SIGNER_REVOKED|INVALID` 的 `package_trust_status`，避免把安装态误当有效运行态。

每个 F-56 special command 派生 `governance_context_id`（首张从候选 grant 取，其后从首次 RELEASED grant history 取），请求头若存在只可相等、UI 不可覆盖，且当前 session/operator 必须具备该法人下对应授权。`DRAFT|PENDING_AUTOTEST|TEST_FAILED|TEST_PASSED` 的 `approval_legal_entity_id` 固定 NULL；submit 同一事务才首次写入派生 id，`PENDING_APPROVAL` 及以后由 deferred graph 强制等于冻结治理法人。治理法人 active 且不得停用，申请/批准/执行按同法人职责分离。禁止新增 license/module CRUD、手工 enable 布尔值、secret/signature 下载或 direct service/DB/KMS 操作。UI E2E 覆盖早期状态 NULL、submit 原子赋值、权限分离、自审拒绝、错误法人、审批摘要漂移、重复执行幂等和 nonrollbackable 提示。

## 8. Task 6：替换 F-55 entitlement 来源

删除 F-55 私有 `F55LicenseGrantPayloadV1` parser 和旧四态映射；保留 `F55EntitlementEvidenceQuery` 名称，但实现只投影 F-56 current/history grant。AI 用 `F55_LOCAL_AI`，入站/出站 MCP 共用 `F55_MCP`。`ACTIVE|EXPIRING_SOON|GRACE_PERIOD` 为 currently licensed；`RESTRICTED` 为 false。

测试必须证明配置 true、module code、feature flag、人工 JSON 或命令行不能伪造购买态；无效许可不得被 collector 静默跳过。

## 9. Task 7：登记、证据与门禁

同步更新：

- `docs/error-codes.md`：新增六个码 `PLATFORM.LICENSE.RESTRICTED`、`PLATFORM.CONFIG_PACKAGE.SPECIAL_ITEM_SHAPE_INVALID`、`PLATFORM.CONFIG_RELEASE_ORDER.NON_ROLLBACKABLE_ITEM`、`PLATFORM.MODULE.LICENSE_REQUIRED`、`PLATFORM.MODULE.PACKAGE_INVALID_OR_INCOMPATIBLE`、`PLATFORM.MODULE.IN_FLIGHT_DRAIN_TIMEOUT`；本节/全局总数固定为 73/495；
- `docs/metrics-catalog.md`：登记 `ep_license_status_info`、`ep_license_usage_over_limit`、`ep_module_install_state_info`；
- `docs/migration-catalog.md`：只扩既有四行描述，总数不变；
- Stage 14：登记 `RG-LICENSE-MODULE-LIFECYCLE-GREEN` 并使 AI/MCP applicability 依赖它。

证据必须绑定同一 `stage14_run_id/deployment_id/product_build_sha256`，包含 initial governance bootstrap unsigned receipt 的 exact hash、双 CMS input、ep-migrate PE、审计链/数据库 projection、同域 ACTIVE/activation audit、首张 RELEASED grant/source、可信时间、许可状态、模块投影、contract descriptor/product manifest、ItemKind exact-set、自动测试报告与真实 PostgreSQL schema digest，并由既有 Stage 14 evidence signer 对 typed aggregate/child 签名；不存在 ep-migrate sidecar。zero-current evidence 只能在真实零 grant 时表现为 `grants=[]/current=null/RESTRICTED+NO_CURRENT_GRANT`；无真实证据时 gate 非零。

## 10. 最终验证清单

- [ ] F-56 两类 strict envelope、CMS/PKI exact shape、`.epcfg` exact container/canonical manifest/after-spec item hash、三条 verifier 与 trust root 正反测试通过；
- [ ] permanent/subscription、60 天、30 天、撤销、倒拨时间全部边界通过；
- [ ] 无 current/签名失效/过期/撤销不自锁：只开放两条恢复链，其他受限写稳定返回 `PLATFORM.LICENSE.RESTRICTED`；
- [ ] current grant 并发、续期、撤销、inner/outer CRL direct-successor 恢复、治理法人冻结/审批绑定、scope、三项用量语义通过；
- [ ] admission 的 10 effect/3 binding、HTTP 与非 HTTP exact-set、ConfigRelease/MCP 动态解析、LIST scope 与 InFlightConvergence 负例通过，registry digest 进入共同 gate；
- [ ] 15 个 contract descriptor/schema/compiled ABI registry、产品模块 JCS/DAG/签名 roster/install readback/Stage14 projection 一致，五条模块边、历史 package identity 一一映射、effective 递归依赖、current inner/outer CRL 下仅 ACTIVE-outer DISABLE 恢复与后续全新 ACTIVE UPGRADE、唯一 `MODULE_SIGNER_REVOKED_DISABLED` 审计 peer/双 projection/reason hash 闭合、revoked 历史禁用、全部非法边、全 15 锁排空/30 秒总超时/崩溃释锁、停用保留数据和再启用通过；
- [ ] special item 只能 imported/single/add/release，通用 rollback 稳定拒绝；
- [ ] ItemKind/SQL/数据字典/迁移目录 exact-set 一致，总迁移数未变；
- [ ] AI/MCP entitlement 只有 F-56 一条来源；
- [ ] ServerAdmin 没有新后端、超级角色或审批结论旁路；multipart staging 与脱敏 bootstrap 通过；
- [ ] fresh production 的 `ep-migrate apply` 三参数治理自举、双签/MFA/SoD/key-domain/audit/receipt、same-digest 崩溃补写与二次拒绝通过，且仍恰五个子命令；
- [ ] `RG-LICENSE-MODULE-LIFECYCLE-GREEN` 在目标 Windows/PostgreSQL 证据上通过；
- [ ] 设计未决为 0；产品完成/发布状态仍按实际证据如实报告。
