# F-57 开发就绪与全场景静态演算

> 日期：2026-08-24（Australia/Melbourne）；审计更新：2026-08-26
> 文档状态：`CURRENT` / `DESIGN_READY`
> 开发状态字段：`development_state=READY_NOT_AUTHORIZED`；`blocking_reason=DEVELOPMENT_AUTHORIZATION_REQUIRED`；`implementation_state=NOT_IMPLEMENTED`
> 生产状态字段：`production_state=PRODUCTION_NOT_READY`
> 首个可执行范围：用户另行明确授权开发后，仅 G0；当前没有任何开发 `GO`

## 1. 结论先行

F-57 的产品选择、架构边界、业务闭环、迁移顺序、故障语义、硬件诚实状态和分层证据已经收敛为一套可直接执行的设计与五文件计划集。现行闭集仍是 185 个 RequirementID，产品未决为 0；2026-08-23 的旧 25 项计划不再参与执行。

“可直接开发”只表示开发人员不需要临场决定架构、数据库所有者、接口字段、失败结果或先后顺序。它不表示代码、迁移、Windows 服务、四端、备份、恢复或 P340 容量已经实现或通过。开发仍需用户下一次明确授权，且必须从 G0 开始。

本文用三个独立字段描述开发：`development_state=READY_NOT_AUTHORIZED`、`blocking_reason=DEVELOPMENT_AUTHORIZATION_REQUIRED`、`implementation_state=NOT_IMPLEMENTED`。`DESIGN_READY=true` 只描述文档完整度；它们均不构成开发授权或另一可执行阶段。

当前 P340 不能承载真实客户数据。只有同一最终候选完成 L3、取得 `RELEASE_CERTIFIED`，并另行满足站点 UPS、服务器外只追加目标、两块离线轮换 HDD、两套分域 2-of-3 恢复材料、洁净恢复硬件、两名不同客户批准人对五项单盘风险的有效接受和新鲜现场读回，才可由独立生产启用 authority 在唯一 terminal CAS 中同时提交 `ACTIVATED` 与 `GENESIS_FULL_CERTIFICATION` 的 `ProductionGenerationAdmissionV1`。证书和 activation row 都不是单独启用权；只有 router exact-match 当前 OBSERVED digest 的 admission 才开业务命令/查询路由。

## 2. 权威来源与阅读导航

逐文件冲突的唯一 precedence 由 F-57 总体设计 §1.1 持有；本评审不重述、扩展或另造排序。以下仅为阅读导航：[总体设计](../specs/2026-08-23-f57-governed-automation-fabric-design.md)、[业务执行契约](../specs/2026-08-23-f57-business-execution-contract.md)、[客户端/生命周期/安全运营契约](../specs/2026-08-23-f57-client-lifecycle-security-contract.md)、[需求追踪](2026-08-23-f57-requirements-traceability.md)、[权威与取代登记](2026-08-23-f57-authority-supersession-register.md)、[ADR 索引](../../adr/README.md)、[Windows/P340 档案](../specs/2026-08-23-f57-windows-p340-production-profile.md)、[收敛主计划](../plans/2026-08-24-f57-converged-program.md)及其 G0–G6 子计划。

旧文档中的 `Task 1…25` 只表示 `F57-01…F57-25` 需求所有权桶，不能推导执行顺序。旧计划固定为 `HISTORICAL_DETAIL_INPUT`，不得作为执行入口。

## 3. 静态就绪门检查

| 检查项 | 现行答案 | 结论 |
|---|---|---|
| 产品范围 | 174 个现行需求 + 11 个延期边界，共 185 行 | 闭合 |
| 需求可执行面 | 57 行交付覆盖登记解析出 `2/19/0/0/2/126/36` 到期向量；22 个 canonical facade target、185 个 exact symbol、185 个 concrete handler，禁止 umbrella/skip/空实现 | 闭合 |
| 产品选择 | 本地模型延期；单一最高安全档；可自建或连接 provider；客户端技术有固定回退 | 0 项待选 |
| 执行入口 | 一份主计划 + 四份依赖有序子计划 | 唯一 |
| 物理业务边界 | 17 个 exact `FeatureOwnerIdV1` + 35 个 exact `PlatformMechanismIdV1`（原 30，F-61 扩表后同批更新，F-62）；七个 CTC owner 只是首片子集；sales/purchase invoice、receivable/payable cash、operating ledger 各自单 writer | 闭合 |
| 机器语义 | 单一 strict-wire `CapabilityGraphV1`；五份旧 API seed 一次性无损 typed import，之后转不可变历史快照；30 个非自引用投影 family（恰四个 multi-member，含 P340 policy 与 semantic contracts）、完整 generator identity、六条 exact client conformance source vector、全 stable-vector canonical sort 表 | 闭合 |
| Wire / schema 基座 | `f57-foundation.v1.schema.json` 是零 import、无反向边的唯一 DAG 根；独占共享 nominal 与 detached-CMS envelope field set；`CandidateIdentityV1` 恰为 `repository_tree_sha256,git_commit,cargo_lock_sha256,capability_graph_sha256,generator_version,migration_manifest_sha256,toolchain_manifest_sha256,artifact_signer_registry_sha256` 八字段；每个 signed root 必须直接 import foundation、只组合一次 envelope 并以 `unevaluatedProperties=false` 封闭，禁止复制、仅传递依赖、绝对/网络 ref 与 cycle | 闭合 |
| 签名代际与 ACK | 唯一 `f57-generation.v1.schema.json` + `generation.rs` owner 持有 manifest/reverse-plan/ACK 三个 strict root，字段数 exact 13/9/14，并唯一拥有 strict `GenerationParticipantApplyReadbackV1\|GenerationParticipantRollbackReadbackV1` 与 tagged item readback；plain ACK 第十三字段是 `participant_apply_readback_ref`、第十四字段才是服务器可信 `acknowledged_at_unix_ms`，无 CMS/无 envelope。Apply 只接受 `DESIRED_ITEM`；rollback 有真前代时接受 `DESIRED_ITEM`，新安装的无前代 item 必须使用 `DEACTIVATED_RETAIN_DATA` 并指向 `ABSENT` installed-state readback；`NO_OBSERVED_GENERATION` 仅 generation 1 回滚合法。独立 `f57-generation-approval-registry.v1.schema.json` + `generation_approval.rs` owner 持有七字段 signed payload 与 manifest/reverse-plan/migration-plan 三条 exact row；G1-01 的唯一 trust coordinator 支持自建 CNG/PIV 或连接已批准企业 signer，但输出同一四角色不可导出 SPKI/DN/CMS contract，并以 journal 成对安装/轮换 registry + storage pin；产品固定部署信任先验证 registry，verified storage manifest 再以唯一 `generation-approval-registry-sha256:<64-lowerhex>` policy pin 固定 DATA_HDD 派生路径中的 exact envelope；只有该 registry 的 approval verifier 能构造绑定 registry ref 的私有 `VerifiedGenerationManifestV1`，generic proof 不具授权力；唯一代际 digest 是完整 canonical signed-envelope bytes 的 SHA-256；generation 1/null previous，后续逐代 exact prior OBSERVED；frozen creation attempt 从 prior OBSERVED + compiled rollback policy 生成每 item signed reverse plan，固定 ID/time/path、使用 issued-at-only issuance rows、create-new crash-adopt 后才签 manifest；G1-05 仅从 authenticated Service-SID IPC + create-new 存储后重读的 fresh readback 服务器派生 exact ACK set；客户端/插件/participant 不能提交 plan/manifest/readback/ACK，89-row evidence registry 不授权 generation | 闭合但未执行 |
| 运行拓扑与最终安装代际 | 唯一 `f57-runtime-topology.v1.schema.json` 同时拥有 strict plain `RuntimeTopologyDeclarationV1\|RuntimeTopologyCertificationV1`，两者不签名且不增加 signer row；G0 只交付 contract/builder/verifier、不得产部署声明；G1-01/G1-05 建立初始 generation/declaration/OBSERVED。G6 必须先完成同运行 `WINDOWS_AUTHORITY_BUILD` 与 `WINDOWS_SERVICE_INSTALL`，再由 `FinalInstalledGenerationAuthorityV1::begin_or_adopt` 从当前 OBSERVED predecessor 构造恰好下一代。它 typed-load graph-exact `WindowsRuntimeDeploymentClosureV1\|WindowsRuntimeDeploymentReadbackSetV1`、最终五个 Authority launcher-role 服务、固定 G0 evidence-signer broker、六组件/五 on-host readback、off-host target proof 与实际 artifact/端点/能力，要求全体 graph/closure/readback participant IDs 一一对应、ACTIVE/positive/declaration/generation-required/ACK participant IDs 一一对应；`database_consumers` 和 generation item/subset relation 分别是 active graph 的独立 exact projection。DEFERRED_DISABLED 只有 absence proof且不进入任何 active relation。推进到同一 attempt 的 `OBSERVED_COMMITTED` 后，候选才绑定 manifest/declaration/ACK exact-set 与租约固定的 `GenerationObservedReleaseSelectionRecordV1`。P340 terminal PASS 后才认证 topology；发布证书绑定认证，独立生产启用还必须重新 exact-match 现场读回。预安装/desired-only generation、consumer/item relation 漂移、deferred 混入 active set、旧 `RuntimeTopologyManifestV1` 和 declaration-only 启用均禁止 | 闭合但未执行 |
| 权威写入 | Windows authority → `CommandPipeline` → `AuthorizedPgTx` → PostgreSQL 16 | 唯一 |
| 数据库安全 | 动态 `PrincipalRefV1`、字段返回权、独立 query-use、事务内已验证上下文、强制 RLS | 闭合 |
| 原子提交 | 当前状态 + feature fact + audit + Outbox + receipt 同事务 | 闭合 |
| 长链自动化 | Objective/Obligation/Effect/Evidence/Cycle、checkpoint、Unknown 对账、重开 | 闭合 |
| 热插拔 | Signed package payload 是 exact 十三字段，新增唯一 `implementation_manifest_ref`；其指向 exact 八字段 plain `CapabilityPackageImplementationManifestV1`，以 tagged artifact 闭合实际声明式/WASM/Windows native/Hyper-V/DB migration/foundation 字节、SBOM、签名与入口。Graph slot 唯一持有 static `scope_mode` 和 closed component class，上层用当前 tenancy snapshot 派生 concrete `LEGAL_ENTITY\|DEPLOYMENT` scope；compiler 强制不可降级 grade/executor/rollback/operation 表。九字段 generation item 只是 pure desired state，不携带 plan/decision/checkpoint/window/attempt；每个 `(activation_attempt_id,deployment_id,scope,authority_epoch,generation_number,participant_id,item_id)` 独立建 `CapabilityPackageGenerationTransitionV1`，并经 installed-state readback → participant apply readback → 14-field ACK 可达。Maintenance 严格两阶段：先 reservation + 历史 30-field plan + 当前双人权限建 hold，关闭 admission、drain 并落盘 `write_barrier_id`；Task 11 再从该 barrier 冻结 full `AuthorityRecoveryCutManifestV1`/checkpoint，然后方可用新鲜 source readback、decisions、execution-trust snapshot、hold/cut/checkpoint 创建 execution authorization 和首个 privileged intent。所有 trust-domain 变更共用唯一 deployment-global `AuthorityStorageManifestRotationCoordinatorV1`；无第二 package/generation manifest writer。普通类走 `EPAuthorityControl` 的 typed PACKAGE 协议；`RECOVERY_TOOL` 是固定 S4U action 的按需 Scheduled Task。五个 Authority 服务只共享一个 fixed `installed_executable_path=ep-core-server.exe`；raw SCM `ImagePath/BINARY_PATH_NAME` 必须是 Windows-quoted executable 加各角色 exact argv，并与 `ServiceInstall.Arguments` parser round-trip 相等。唯一 ABI generator 的 `--check` 必须使 Rust binding、C header、DEF 零 diff；DLL 只有一个 named non-forwarded export `ep_authority_kernel_get_api_v1`，Rust kernel 只切换版本化 `ep-authority-kernel.dll` 与签名 monotonic slot pointer，不覆盖 launcher、不调 `ChangeServiceConfig`。Forward 失败不自行 rollback；上层先持久化 `ROLLBACK_STARTED` 和 rollback ID。Package-local `APPLIED_VERIFIED` 不是 global OBSERVED/生产 admitted，只有全 participant ACK 后的 OBSERVED 与合法 `ProductionGenerationAdmissionV1` 才可开路由 | 闭合但未执行 |
| 客户定制 | 关系模型编译器、能力包、provider/WASM/worker/条件容器；无任意 SQL/DLL | 闭合 |
| C/S 双平面 | 服务器 Control Center 与四端 Workbench 分离；服务器权威、客户端服从；Tauri/Flutter 两分支只允许一个签名选择并统一产出 `client-branch:selected` | 闭合 |
| 数据盘 | 全部权威客户数据及可关联衍生持久数据在 HDD。RUNTIME_SSD 全卷持久内容先分为不相交 Set A/Set B：Set A 是 signed `RuntimeSsdReproducibleRuntimeInventoryV1` 所闭合的 catalog-verified Windows、immutable product bytes、bounded reconstructible OS cache 与 TPM-bound reenrollable key metadata；Set B 恰为 POWER capsule、package recovery capsule、kernel pointer/head、signed native-code slot/cache 四个 mutable class（十九 media contract、二十 path row）。Control 对象必须 BitLocker + TPM/恢复域认证、大小/保留有界并镜像 off-host；code slot 必须 content-addressed 且可丢弃。完整 locator/allocated-stream/ADS/hard-link/VSS scan 必须把每个 entry 唯一归入 A 或 B；八行 Windows persistent policy 固定六种 page/hibernation/dump/WER 关闭，VSS diff-area 与 quarantine 在 DATA_HDD；七行 telemetry policy 固定四行 bounded no-customer SSD、一行 firewall text log disabled、HTTP.sys error 与 Authority audit 两行 DATA_HDD。丢 SSD 后从 authenticated DATA_HDD/off-host heads 重建 control、从 signed manifest 重新 stage 代码；Set B 第五类、Set A byte/catalog 漂移、隐藏 Windows persistence 或任何客户字节一律失败 | 闭合 |
| 迁移 | pre-F57 388 行闭分区：69 个可执行 baseline + 319 个必须缺席；G0 不新增 F57 SQL，只受控修订 3 个未发布草案；F57 使用 exact 9-column/47-row 单文件 reservation；现行 Fresh-PG 检查权威是 exact 27-row `f57-fresh-pg-check-registry.v1.tsv`（SHA `76fed80f…01a`），以 profile + activation through-version 双条件选择，旧 23-task seed 仅为历史输入；最终 Fresh-PG 精确为 `69+47=116` | 闭合 |
| 证据 | L0/L1/L2/L3；185 行统一使用 signed `RequirementEvidenceBindingV1`；最终 L2 固定为 first-due G0…G5 的 149-ID 向量（SHA `5ec5a866…5a7a`），L3 固定为 first-due G6 的 36-ID 向量（SHA `e7a2fae4…85df`），两者互斥并集 185，六个 carrier auxiliary IDs 仅进 `carrier_refs`；276 个 TestID 精确为 `185 Requirement + 78 probe + 3 client conformance auxiliary + 4 client validation auxiliary + 6 release-carrier auxiliary`；唯一执行键为 `(candidate_identity_sha256,gate_run_id,TestID)`；显式 run journal 采用单写者 OS lease、先 `STARTED` 后副作用、签名哈希链、崩溃对账和严格延伸 checkpoint；276-ID TestResultStore、14-row EvidenceEnvelopeStore、3-row CandidateManifestStore、JournalCheckpointStore、全局 architecture slot/attempt store、OfflineSchemaManifestStore 与 content-addressed ObjectStore 分工唯一；89-row evidence signer registry 无 wildcard 且与 generation approval trust 域分离；候选/聚合使用 create-new finalization/adopt，carrier 使用 typed staging plan/input/completion；最终离线证书从显式 bundle root 可追到独立 bootstrap Schema 所引导且包含 generation、generation-approval-registry、generation-observed-selection、runtime-topology、Windows-runtime-deployment、Windows-server-component-set owner 的完整传递 schema 闭包，并重验 graph 全集/ACTIVE 子集/DEFERRED_DISABLED 排除公式、候选、final L2/L3、六级 plain typed receipt ref、185 个结果和 journal prefix；aggregate 不能延长最短输入有效期 | 闭合 |
| 离线 package 闭包 | Package 路径是 desired item → registry/package → implementation manifest → 全部实现字节/SBOM/schema/WIT/签名/migration/foundation ref + concrete tenancy scope；ACK 路径是 `participant_apply_readback_ref` → canonical item readbacks → nonnull per-attempt transition → execution-trust/typed operation results/final installed-state；maintenance 再追到 reservation、历史 plan、当前 execution authorization/decisions、hold、实际 checkpoint 和 full `AuthorityRecoveryCutManifestV1`，production delta 再追到 exact admission predecessor/ACK/transition/resource envelope。最终 bundle 不复制、不回放 plan-finalization journal；该 journal 只作在线崩溃恢复测试，离线只闭合 immutable artifact/readback，严禁 scan/latest/allowlist 补链 | 闭合 |
| 客户端架构历史 | 单一全局 slot/attempt；决策在 BOUND 前固化 RFC-3161、DecisionSigner/TSA chain、按角色/索引的 CRL+OCSP 与 trust-closure，BOUND 后生成可重建 manifest；提交归档和每次 current-bundle copy 使用 relocatable archive URI，离线从 validation→manifest→decision/trust/entries 闭包遍历；历史 subordinate 由永久 decision 公证，当前四端 evidence 仍重做 | 闭合但未执行 |
| 客户端生命周期 fixture | CapabilityGraph/compiled graph 固化 exact 四个公开非生产 DER 测试根与 16 个原生包来源；policy 每平台绑定根 DER digest/SPKI，fixture 绑定叶 SPKI/根 digest/角色/结果；G0-02 创建并评审 20 个固定公开字节和 strict corpus manifest，私钥/密码/provisioning secret 禁止入库；四 lane 仅在重置的隔离信任库中验链 | 闭合但未执行 |
| Graph-exact Windows runtime deployment | `WindowsRuntimeDeploymentClosureV1` 为每个 graph participant 生成恰好一个 `ACTIVE\|DEFERRED_DISABLED` delivery row，`WindowsRuntimeDeploymentReadbackSetV1` 为每行生成恰好一个 positive/absence row。载体严格分为 `WINDOWS_SERVICE`、`JOB_OBJECT_WORKER`、`IN_PROCESS`、WASM（wire `WASM_SANDBOX`）和 Hyper-V（wire `HYPER_V_CONTAINER`），并固定各自 artifact/readback 矩阵。全体 graph/closure/readback participant IDs 一一对应；ACTIVE/positive/declaration/generation-required/ACK participant IDs 一一对应；database consumers 独立 exact-project 到 active service identities；generation items 与每个 required participant 的 item subset 独立 exact-project，允许多对多但禁止 orphan/out-of-set item。DEFERRED_DISABLED 无 artifact 且从所有 active relation 排除。首版 local AI 固定 `DEFERRED_DISABLED{reason=LOCAL_AI_IMPLEMENTATION_DEFERRED}` + `NullAiProviderV1`，无进程/端点/模型包/资源预留；固定九进程和 installer side list 均禁止 | 闭合但未执行 |
| Windows Server 2022 服务与安全描述符 | 五个 permanent Authority launcher-role 服务是 ordinary `EPAuthorityServer`、dormant continuation、control broker、raw signer facade、journal signer facade；但 EnterprisePlatform-owned 固定清单精确为九个 SCM 服务：上述五个 + 独立 G0 `EPF57EvidenceSignerBroker` + backup writer/checkpoint signer/data-volume-unlock broker 三个 component service。完整 host 还含 `ep-postgres16` 与 graph 中不别名的 ACTIVE/WINDOWS_SERVICE 行，所以 cardinality 是 `10 + active_additional_windows_service_count`；未知 product-owned row 失败，系统/第三方服务不计入。G0 broker 是 key owner，两个 facade 无 key ACE；broker pipe 只给 client group concrete `0x00120183`，客户端不能 `GENERIC_WRITE` 或创建 first/second/replacement instance。`EPAuthorityControl` 同一 pipe 以 discriminator 封闭 POWER/PACKAGE，保留 exact 18-object SDDL。五个 Authority installed executable 固定为 `ep-core-server.exe`，raw SCM command line 必须是 quoted executable + exact role argv；唯一 ABI generator `--check`、one-export PE/size/offset/section/held-file goldens全部通过。SCM/安装映像/bundle+run/已安装 activation child key/四 pipe/两 key 仅 `VERIFY_EXISTING_IMMUTABLE`，只有本次 staging/capsule/state 才 `CREATE_WITH_DESCRIPTOR` | 闭合但未执行 |
| PostgreSQL 16 Windows 安装闭包 | Task 11 的 `postgres16_windows.rs`/单一 schema 唯一拥有 19-field package lock、13-field install contract、4-field Event Log fixture set、19-field Event Log scan coverage 与 17-field install readback 五个 strict plain root；23-field artifact set 认证 contract/scan contract/fixture，22-field service-install evidence 认证 readback/coverage，不增加 signer、installer/service-configuration PowerShell 或 backup component。V1 只允许 clean/same-lock exact adopt；`installed_files`↔SBOM/final-handle、extension set、四方 system identifier、九路径 SDDL→live DACL、typed `RUNNING` 与 SSD/HDD 分界均 exact。关键 GUC 精确为 `max_connections=64\|reserved_connections=4\|superuser_reserved_connections=3`，安全余量 2；每个 consumer 属于 `NORMAL\|RESERVED\|SUPERUSER` 并满足五条分类预算与 role privilege readback，应用不能吞保留位。HBA 只证明 loopback `hostssl`+SCRAM；client `channel_binding=require` 与协商由 authenticated probe 单独证明。`fsync_writethrough` 只是兼容性 pin；同文件双方法 qualification 绑定卷/driver/cache，最终耐久性只由 Task 15 再 exact-join P340 UPS/write-cache/flush/power-cut 证据。日志固定 collector→stderr→HDD；Event Log typed coverage 闭合两个 provider registration、同 boot bookmark/record/time、零 clear/drop/gap、fixture ref/digest/complete execution 和零 token 命中，缺失、截断、错配均拒绝。禁 `initdb --waldir`、tablespace、reparse、trust、external CIDR 与 ambient override | 闭合但未执行 |
| 备份/恢复部署闭集 | exact 六行 `BACKUP_WRITER_SERVICE\|BACKUP_CHECKPOINT_SIGNER_SERVICE\|DATA_VOLUME_UNLOCK_BROKER_SERVICE\|RECOVERY_TOOL\|PG_PASSPHRASE_HELPER\|BACKUP_TARGET_AGENT`；前五项 on-host，前三项是不同安全身份的 AUTO_START Windows service，`RECOVERY_TOOL` 是唯一 S4U Scheduled Task，helper 是唯一 on-demand executable，target agent off-host-only 且单独打包。Recovery task 使用 `EPF57Recovery`/`DEDICATED_LOCAL_S4U`/`LEAST_PRIVILEGE`、exact rights/RequiredPrivileges/account flags/direct+prohibited groups、零 Task-stored password/installer plaintext residue和真实 runtime-token self-test；action、folder/task/executable DACL、`0x00120183` pipe client mask、六 operation allowlist全固定。unlock broker 是 LocalSystem + restricted service SID、no network、typed AS-only client；DATA_HDD protector exact-set `{PUBLIC_KEY,RECOVERY_PASSWORD}`，ordinary boot 用 existing Microsoft Platform Crypto Provider key，clean SSD 只能双人 recovery-password ceremony 后新建 TPM-backed key/certificate/protector、提升 authority epoch/NV、正常重启验收再删旧 protector。build 签名 Authority、G0 broker、六组件及全部 graph-ACTIVE carrier；install carrier 安装五个 on-host 组件并证明远端 target。writer/signer/unlock 权限与 `component_id` challenge 相互隔离。唯一 `AuthorityRecoveryCutManifestV1` 覆盖全部 enabled DATA_HDD authority classes/roots；三次 clean-hardware recovery exact-verify 同一 cut 每一行 | 闭合但未执行 |
| 备份拓扑与防勒索现场证据 | `topology_signing_trust.rs`/`safeguard.rs` 与三份 schema 唯一拥有 signed `BackupTopologySigningTrustManifestV1`、signed current pointer、signed topology、strict plain safeguard 与 closed multi-CMS support root。部署 bootstrap 固定独立 trust-manifest authority；active-config 分别选择 current trust pointer 与 topology，pointer typed-load 唯一 manifest，manifest 固定 topology signer DN/SPKI/offline chain/revocation/checkpoint；私有 `BackupTopologyAuthorityV1` 只能由该 verified-current trust 构造，不能复用应用/备份恢复域、ADR-0020 roster、候选/self/support/ambient trust。topology exact-repeat trust refs并 join current singleton-target storage manifest；六角色、writer/target SPKI、off-host target、exact A/B、live domains 和每盘两个分域 human custodian 均 exact。clean install 仅允许空 retained/head/A-B 的 `INITIALIZING + INITIAL_POPULATION`；sequence 1 后进入 `BOOTSTRAPPING`，先把 head 验证到 A/B，再由 checked head 补足 minimum，闭合后才为 `HEALTHY/None`。current roots 只从 fresh `HEALTHY` 轮换，经单一 `TRANSITIONING` old-head+1 bridge，再以不得创建 checkpoint 的 `BOOTSTRAPPING + CURRENT_ROOTS_ROTATION` 完成 A/B closure；健康前禁止第二轮换。所有 retained refs typed-load 为唯一连续 `BackupCheckpointV1` 链，current head exact-bind current trust/topology/storage tuple；四个非健康状态不得 PITR/发布/恢复认证/启用。install/checkpoint/PITR/activation/retry 每次新 challenge/session/object，expiry=`observed+max_age<=300s`；target refs 单签、介质 transition/四 observation 双签且类型/链/投影闭合，八条唯一合法介质边、physical total/free/quota/reserve、partial optionality、六角色权限负探针、一次性 just-written capability、A/B 无 recovery material/断开/健康/保管均为 typed gate。cut、PITR 三 leaf、target install 与 activation exact-join同一 current tuple；16-field PITR、18/17/30 subordinate、89 candidate signer 不变 | 闭合但未执行 |
| Windows 强制预提交门 | `.github/workflows/ci.yml` 的 `windows-f57-release-precommit` 必须在批准的 Windows Server 2022 x64 self-hosted runner + 锁定 MSVC 上运行 powershell-trust、release/runtime/backup/generation-activation/core/xtask/testkit/recovery-tool、`ep-platform-ups-contract`、`ep-adapter-ups-windows` 与 `ep-authority-kernel` 全目标测试，执行 `f57_ups_adapter_contract\|f57_ups_command_reconciliation`、ABI generator `--check`、release build 与真实进程/composition tests。除 runtime-deployment/generation/package/final-installed/activation 外，还必须验证 fixed nine-SCM/product inventory + `10+active` host formula、G0 broker、six-component/five-on-host/three-service、quoted ImagePath+argv、one-export ABI、S4U account+runtime token、`0x00120183`/no second pipe instance、PUBLIC_KEY/九 locator/bootstrap/WMI/restricted-token WS2022 unlock、clean-SSD reenrollment、ExecutionLease/ten-row bypass/same-lock races、Runtime SSD Set A + four-rule Set B/eight persistent/seven telemetry，以及 UPS binary/config/provider-operation exact joins。Linux/macOS、`cfg(windows)` 排除、mock、allow-failure 或 desired-policy echo不可替代；代码组合也不冒充 Task-15 动态现场证据 | **该 job 当前不存在**（实测 `.github/workflows/ci.yml` 只有 `pipeline` 一个 job，`grep -c windows-f57-release-precommit` = 0）；本行是设计要求而非现状，落地前不得据此宣称已有 Windows 预提交门 |
| P340 硬件认证 | 固定 P340/i5-10500/32GiB/≥240GB runtime SSD/≥1TB 单 CMR HDD；4321 个一分钟样本、25/25 指标、十个 nested + 七个 supporting readback；内存/SSD 洁净恢复、Boot/BitLocker、签名 UPS outlet-group→P340 power-path、写缓存、SMART、温度、水位均为 typed 证据。BitLocker DATA_HDD 必须 exact `{PUBLIC_KEY,RECOVERY_PASSWORD}`、auto-unlock false；ordinary reboot 的 restricted-LocalSystem broker unlock与 clean-SSD 双人八步 reenrollment必须真实通过。POWER 还固定 BIOS `POWER_ON`、五个 Authority permanent `AUTO_START` role、无 activation continuation dormancy、exact 18-object/action-row SDDL、1074/clean-stop/outlet-off-on/clean-start 与 Authority recovery proof。SSD residency probe 必须按 Set A/Set B、八行 persistent policy、七行 telemetry policy确认最终句柄/卷/大小/保留/镜像/重建与零客户字节 | 闭合但未执行 |
| UPS 可插拔适配与幂等断电 | `crates/platform/ups-contract`/单一 schema 唯一拥有 exact 16/20/21/28-field manifest/status/typed command/ACK 和分离 status/control ports，`crates/adapter/ups-windows` 是唯一 Windows 实现；release 对 nominal UPS contract、authority-kernel 对 contract + adapter 均为直接依赖并由 locked dependency golden 固定。实现仅在既有 `EPAuthorityControl`，不增加服务、子进程、vendor DLL 或 signer；manifest `implementation_binary_ref` 必须是候选 authority-kernel binary，reopened digest 必须等于运行时 held binary。`configuration_projection` 是唯一部署选择，正 generation 与其 JCS digest 必须在 identity/status/command/ACK 四处一致，禁止 ambient override。standard 使用空 profile/null status profile、logical config identity 与 UNKNOWN self-test，仅监测；最高档必须 vendor adapter。USB 使用 canonical GUID/instance + service-SID ACL；网络使用 numeric-IP/nonzero-port structured exact-one destination；credential 最小授权。每个 status exact-join signed identity runtime binding，initial/previous/trigger 同 boot/PID/start-key；provider self-test attestation <=24h PASS。冻结时间恰为 5 秒轮询/15 秒有效/86400 秒 self-test/30 秒 command ACK；provider 调用前耐久 start marker，ACK observation/query 受同 boot/source `min(start+30s,command deadline)` 约束。供应商必须返回 1..128 字节 canonical ASCII `provider_operation_id`，符合 `[A-Za-z0-9][A-Za-z0-9._:-]{0,127}`，并先耐久绑定 `(ups_adapter_identity,command_id,command_sha256)`；空值、别名、改变或跨命令值均进入 UNKNOWN，绝不重发。UTC 仅报告，POWER 600 秒仅外层复合对账。同 identity+ID+digest adopt byte-identical ACK，异 digest conflict，未知不重发，boot change 前缺 composite ACK 永久失败；`f57_ups_adapter_contract\|f57_ups_command_reconciliation` 与两 UPS package 的 Windows 全目标测试是强制证据 | 闭合但未执行 |
| 生产诚实状态 | `RELEASE_CERTIFIED` 与生产启用严格分离。单 HDD 只允许 `SINGLE_DISK_DEGRADED_PRODUCTION`；启用前必须有 exact 20-user、双人、未过期且绑定同部署/候选/证书的接受记录，完整接受 exact 五风险，并重验外部只追加/两块离线介质/UPS/BitLocker/TPM/恢复与四类新鲜现场 readback。唯一启用 terminal CAS 必须同事务创建 `GENESIS_FULL_CERTIFICATION` 的 `ProductionGenerationAdmissionV1`；路由门只信任当前 OBSERVED digest 完全匹配的 admission，不信任 activation row 本身。后续 package delta 必须对 impacted scope 全集原子闭路由，并在 exact ACK/transition/predecessor/resource envelope 重验后以 `PACKAGE_DELTA` 单次重开；rollback 只能用 fresh predecessor readback 的 `ROLLBACK_REOPEN`。任何 OBSERVED/admitted mismatch、不完整 scope 或 UNKNOWN 都保持 hold。Collector/live-drift 故障进入 `FAILED_HELD`，修复后只能让同一 `activation_id` 以 prior failure hash CAS 到递增 ordinal 的 `RETRY_COMMITTED`并重采四类 readback；永不声称 RAID/HA | 闭合但未执行 |

这里的“闭合”表示规范给出了唯一预期、实现位置、失败结果与未来证据，不表示未来测试已运行。

## 4. 架构演算

### 4.1 写入与权限

```text
Workbench / Control Center / Portal / Excel / MCP / Provider
                         │
                         ▼
              各自封闭的入口信封
                         │
          服务器重建身份、法人、设备与策略
                         │
                         ▼
                 CommandPipeline
                         │
          DB 验证一次性事务上下文 ticket
                         │
                         ▼
                 AuthorizedPgTx
                         │
     状态 + 事实 + 审计 + Outbox + 回执同事务
                         │
                         ▼
                    PostgreSQL 16
```

客户端不能提交可信 actor、法人、权限、MFA、SoD 或 authority epoch。业务 repository 无法构造 `AuthorizedPgTx`；原始 GUC、普通 pool connection 和 session 级安全上下文都不能成为旁路。迁移、恢复、context issuer 和 operations 使用分离身份。

生产准入没有“先检查、后执行”的竞态窗口。每个 business command/query 在 admission head 与 hold writer 共用的 deployment-scoped serializable lock/CAS 内，使用 stable authenticated request ID + full binding digest，一次 exact-load服务器派生 current authority epoch、OBSERVED/current admission/API generation，要求 epoch 同时等于 typed admission、OBSERVED generation 与 verified security context，检查全部 target scope 与 live hold，并 create-new 或 exact-adopt exact twenty-field `ProductionAdmissionExecutionLeaseV1{ACCEPTED}` 后才返回 affine permit；epoch 纳入 binding digest且客户端不可提交。事务内完成只允许在同一 epoch 终结为 `COMPLETED_IN_PLACE`，exact-once Outbox/workflow handoff只允许终结为 `HANDED_OFF_EXACT_ONCE`；orphan ACCEPTED 没有正向 no-effect/handoff evidence 就持续阻止 drain。唯一约束保持 `(deployment_id,request_id)`，跨 epoch 重入不能产生第二次 effect；hold writer 在同一锁中先提交 `ADMISSION_CLOSED`，且同事务跨全部 epoch 的 intersecting ACCEPTED count 为零后才能提交 `DRAIN_COMPLETE|BARRIER_COMMITTED`，所以 permit-before-hold 与 hold-before-permit 都有唯一线性化结果。

唯一 admission bypass registry 是 exact 十行：两个 internal health GET、同一 control-command POST 下的 production-activation/package-maintenance/recovery 三 selector、同一 control-query POST 下的 admission-status/package-maintenance/recovery-operation/recovery-proof 四 selector，以及按 persisted original-command classification 继承的单一 receipt GET。它没有 `/control/*` prefix、wildcard、default 或 handler-local Boolean；每行只绕过 production admission/hold，仍重验 parsing、identity、CSRF、capability、MFA/SoD、epoch、operation ID、rate limit、audit，并只返回 `CONTROL_METADATA_NO_CUSTOMER_FIELDS`。其 private bypass permit 不能构造 `AuthorizedPgTx`，任何 employee/portal/MCP/file/event/unknown route仍走业务 gate。

### 4.2 CTC-01 首条闭环

G2 服务器侧主链固定为：

```text
客户 → 合同版本/回款节点/附件 → STANDARD 销售订单
→ 销售触发采购 → 采购订单 → 收货 → 客户交付
→ 销售发票 → 应收 → 收款核销
```

G2/G4 的唯一诚实结果是：

```text
CONTRACT_FULFILMENT       = CLOSED
SALES_ORDER_FULFILMENT    = CLOSED
RECEIVABLE_COLLECTION     = CLOSED
PROCUREMENT_FULFILMENT    = WAITING
```

采购目标唯一阻塞义务是 `PURCHASE_AP_CLOSED`，并保存：

```rust
ProcurementSettlementGapV1 {
    purchase_invoice_recorded: false,
    payable_recognized: false,
    supplier_payment_settled: false,
}
```

只有 G5 由 `purchase-invoicing`、`payable-cash` 与相关库存/采购 owner 真实提交采购发票、应付确认和供应商付款，使三项都为 true 后，采购目标才可进入不同复核人的 closure review 并关闭。`sales-invoicing`、`receivable-cash` 与 `operating-ledger` 分别保持销售开票、收款核销和经营分录/试算/永久期间锁定的唯一 writer；共享 schema 不扩大 writer 权。任一冲销重新打开新 cycle；旧 cycle 和事实不可覆盖。

### 4.3 可插拔与一致性

所有可变项通过签名 generation 或签名 package/provider manifest 进入。G1-01 先用产品固定部署信任与 verified storage-manifest digest pin 验证 DATA_HDD 上的 exact 三行 approval registry，再验证每个 item 对应的 signed reverse plan，最后通过该独立 generation/migration approval trust 域签署完整 generation envelope，并按同一 whole-envelope digest 构造 topology declaration。新 generation 先验证、预下载、迁移、模拟；G1-05 只根据 authenticated Service-SID IPC 和 create-new 存储/重读的 `GenerationParticipantApplyReadbackV1` 服务器派生 exact 14-field participant ACK。Apply readback 的 canonical `applied_items` 与 `required_item_ids` exact-match；每个 package item 必须携带 nonnull transition ref，readiness refs 必须闭合 typed operation result 与 installed-state evidence。全部同 attempt ACK 前 OBSERVED 指针不移动。回滚同样先持久化/重读 `GenerationParticipantRollbackReadbackV1`；有真前代时绑定 exact predecessor manifest，generation 1 才可绑定 `NO_OBSERVED_GENERATION`，新安装 item 回退为 `DEACTIVATED_RETAIN_DATA` + `ABSENT` readback，不伪造 predecessor。运行命令、流程和 effect 持有版本 pin；`UNKNOWN` effect、审计、备份、法律保留和回滚窗口都会阻止旧制品回收。

`ABSENT` installed-state readback 的 package/version/implementation/subgraph/permission/retention/pin 字段必须全部 null，`absence_readback_ref` 必须 nonnull，runtime identities 必须空且 `admission_open=false`；`INSTALLED_DISABLED|ENABLED` 则相反，所有 artifact 字段 nonnull、absence null、package/implementation exact-match。因此“新安装退回 ABSENT 但保留数据”不会变成假前代或卸载删除语义。

G6 的生产制品和实际 carrier 部署是后产生事实，所以候选冻结前有一条不可跳过的第二闭环：clean-HEAD graph projection/closure + build → 每个 ACTIVE carrier 的 exact-set 安装/正读回 + 每个 DEFERRED_DISABLED 的 absence 读回 → 五个 Authority launcher-role 服务 + 固定 G0 broker + 六组件中的五个 on-host readback + off-host target proof → `FinalInstalledGenerationAuthorityV1::begin_or_adopt` → 全 graph participant/closure/readback 双射，ACTIVE participant/positive readback/declaration participant/generation-required participant/ACK participant 双射，database-consumer 独立 exact 投影，generation item 与 participant-item edge 独立 exact 投影 → durable `OBSERVED_COMMITTED` → 租约固定 observed selection → candidate freeze。generated router 与 exact 十行 production-admission-bypass registry 必须同源；`WindowsAuthorityArtifactSetV1.production_admission_bypass_registry_ref` 在 signed build result 中绑定 exact bytes/media，`ReleaseWindowsServiceInstallEvidenceV1.production_admission_bypass_registry_ref` exact-repeat 同一 ref 并证明 installed bytes，candidate 只经已签名 artifact set 到达该 ref，startup 只 typed-load candidate-bound installed ref。该 authority 只接受同运行 build/install result refs，不接受调用方给出的 manifest、process list、generation number、item、participant、topology ref 或 loose bypass registry；每个崩溃切点只采用同一 transition prefix，不能扫描 latest、另开 attempt、把 deferred row 混入 active participant/consumer/item-edge/ACK 集或沿用安装前代际。

停用模块只阻止新入口和新执行，历史数据、附件、审计与读取证据继续保留。WASM、签名 Job Object worker 和条件式 Hyper-V 容器都默认无网络、文件、密钥或数据库权限；当前 P340 未证明容器能力时确定性返回 `HOST_CAPABILITY_UNAVAILABLE`，不能暗中降级到不隔离执行。

可直接实现的字段闭集是：13-field package payload = `{schema_version,purpose,package_id,package_version,component_class,capability_subgraph_ref,implementation_manifest_ref,permission_ceiling,required_host_capabilities,migration_plan_ref,compatibility,data_retention_contract,hotplug_contract}`；8-field implementation manifest = `{schema_version,purpose,package_id,package_version,component_class,artifacts,sbom_ref,implementation_set_sha256}`；9-field pure desired item = `{schema_version,purpose,item_id,package_id,desired_package_ref,desired_package_version,desired_lifecycle_state,package_trust_registry_ref,scope}`。任何多/缺字段、另一 implementation owner、从路径扫描补字节或将 per-attempt authority 写回 item 都是 schema/架构失败。

Implementation 的 class/tag 矩阵也是闭集：`CONFIGURATION|UI|REPORT|RULE|WORKFLOW|MCP_CONFIGURATION -> DECLARATIVE_BUNDLE`，`WASM_EXTENSION -> WASM_MODULE`，`JOB_OBJECT_WORKER|CONNECTOR|AI_OCR_PROVIDER|RUST_KERNEL -> WINDOWS_NATIVE_BINARY`，`HYPER_V_CONTAINER -> HYPER_V_CONTAINER_IMAGE`，`POSTGRESQL_DATABASE_MIGRATION -> DATABASE_MIGRATION_BUNDLE`，`CRYPTOGRAPHY_FOUNDATION|STORAGE_FOUNDATION -> FOUNDATION_ARTIFACT_SET`。`artifacts` 非空、canonical 且 duplicate-free，`implementation_set_sha256=SHA256(JCS(artifacts))`；executor 不得 stage 该向量以外的任何字节。

模块“热插拔”不是统一零停机承诺。Atomic 把已接收请求 pin 在单一 generation 并只切一次；Drain 先持久化关闭 admission 并逐项完成或 exact-once handoff；Maintenance 由 30 字段 signed plan 保留历史结构授权，但九字段 generation item 永远只是 pure desired state，不携带任何 plan。真正的每次执行权在独立 `CapabilityPackageGenerationTransitionV1` 上：atomic/drain 在第一外部 intent 同一 CAS 冻结无 plan 的 execution-trust snapshot；maintenance 先持久化 `HOLD_INTENT -> ADMISSION_CLOSED -> DRAIN_COMPLETE -> BARRIER_COMMITTED`，再由 Task 11 用该 barrier 冻结 full recovery cut/checkpoint 并进入 `CHECKPOINT_BOUND`，最后才从新鲜双人 decisions、source readback、trust snapshot、hold/checkpoint/cut 生成 execution authorization 和 `FORWARD_INTENT_COMMITTED`。Plan 中只有 checkpoint policy，没有预先伪造的 actual checkpoint。

每个多对多 package item 以 `(activation_attempt_id,deployment_id,scope,authority_epoch,generation_number,participant_id,item_id)` 为独立执行与测量 tuple；不能借用另一 participant 的 trust snapshot、operation result 或 ACK。执行存储采用 intent-before-call + `begin_or_adopt|query_exact` 的闭合 CAS；失败只报告上层，待 coordinator 先持久化 `ROLLBACK_STARTED{rollback_execution_attempt_id}` 后才可用私有 rollback request 恢复 prior item 或 exact checkpoint。过期/撤权发生在外部 intent 后，只能 reconcile/measure/rollback，不得重复 forward。Package-local `APPLIED_VERIFIED` 不等于 global OBSERVED，OBSERVED 也不等于 production admitted；数据/evidence/predecessor pin 直到上层 readback/ACK 与 `ProductionGenerationAdmissionV1` 完成。

所有 package/generation trust 轮换由同一 deployment-global `AuthorityStorageManifestRotationCoordinatorV1` 串行化一个 storage-manifest revision/CAS/journal，不存在独立 package manifest writer。普通类只能经 `EPAuthorityControl` 的 typed PACKAGE 协议；recovery class 只能调度唯一 immutable `\EnterprisePlatform\F57\RecoveryToolV1` S4U Scheduled Task。五个 Authority service 的 executable path 永久固定为 `ep-core-server.exe`，raw SCM `ImagePath/BINARY_PATH_NAME` 必须逐行等于 Windows-quoted executable + exact role argv；通过唯一 generator `--check` 和 one-export launcher↔kernel DLL ABI 验证后才可加载版本化 `ep-authority-kernel.dll`。仅 signed monotonic slot pointer 可切换，绝不覆盖 launcher 或改 SCM。

### 4.4 Windows 安装、启动与解锁资格

`RECOVERY_TOOL` 的静态 principal policy 与安装时 SID 读回严格分层。源码永远只写 `DEDICATED_LOCAL_S4U/EPF57Recovery/S4U/LEAST_PRIVILEGE`，不冻结客户机器 SID；installer 创建或 exact-adopt account 后解析 numeric SID，再替换所有 DACL placeholder。Account rights exact 为 `[SeBackupPrivilege,SeBatchLogonRight,SeChangeNotifyPrivilege,SeManageVolumePrivilege,SeRestorePrivilege]`，Task RequiredPrivileges exact 为 `[SeBackupPrivilege,SeChangeNotifyPrivilege,SeManageVolumePrivilege,SeRestorePrivilege]`；direct group 只有 Users，prohibited admin/operator set 交集必须空，user flags exact `0x00010240`。Task Scheduler 不存 S4U password，installer 不留 plaintext residue；SAM verifier 是正常 account state。live no-side-effect self-test 必须在 WS2022 冻结完整 SID/attribute 与 privilege/attribute vectors，要求 MEDIUM/DEFAULT、非 AppContainer/非 restricted token、动态 logon SID、`SERVICE_ASSERTED_IDENTITY=S-1-18-2`，并拒绝 `S-1-18-1` 冒充 credentialed identity。固定 action 是 `recovery-tool.exe --scheduled-task-server`，零 triggers、单 Exec、固定 task/folder/executable DACL；runtime-created pipe 由 RT SID owning，AS 只有 `0x00120183`，不能创建 second instance。

DATA_HDD pre-HDD bootstrap 是 build/MSI 固定的九行 locator set，不得 scan/latest：registry、unlock authority、public object root，加上同一 fixed trust directory 下 registry trust bundle/revocation/checkpoint 与 unlock-certificate CA bundle/revocation/checkpoint。Registry/authority/public-object caps exact `1048576/1048576/16777216`；每个 trust bundle/revocation cap `4194304`，checkpoint cap `65536`。post-reboot resolution readback必须 exact九行、final handle在 RUNTIME_SSD、descriptor/media/digest/size匹配、零 reparse/ADS/hard-link/unregistered locator。这些不可变 bytes 属于 Set A，不是 Set B 第五类。

Certificate policy 在 64-bit `HKLM\SOFTWARE\Policies\Microsoft\FVE` exact 读取 `FDVAllowUserCert=1`、`FDVEnforceUserCert=0`、`CertificateOID=1.3.6.1.4.1.311.67.1.1` 与 deployment-bound `IdentificationField`；provider 必须是 Microsoft Platform Crypto Provider，private key TPM-backed/nonexportable，leaf非自签，EKU exact同一 OID，KeyUsage exact `[DATA_ENCIPHERMENT,KEY_ENCIPHERMENT]`，离线 pinned chain/revocation/checkpoint通过且不使用 ambient root/network completion。WMI 仅本机 `ROOT\CIMV2\Security\MicrosoftVolumeEncryption`，packet privacy、impersonate、无 delegation/供应凭据；installer只 merge broker service SID 的 `0x00000003=WBEM_ENABLE|WBEM_METHOD_EXECUTE` required ACE并保留 OS/provider ACE，broker不得获得 `0x000000FC`。typed binary只能从 certified volume identity派生实例并调用 explicit nonzero thumbprint、empty PIN 的 `UnlockWithCertificateThumbprint`。真实 restricted-LocalSystem WS2022 gate必须返回 0、exact protector type 7/PUBLIC_KEY/certificate/volume且 fixed-data auto-unlock=false；desired GPO/WMI/descriptor不能代替这一结果。

ordinary reboot 只重开既有 PCP persisted key；clean SSD 会丢失 provider/container binding，不能由 DER/SPKI/TPM handle/public metadata复原旧私钥。唯一恢复是 admission closed 下的 off-host 48-digit recovery-password 双人 ceremony，依序完成 recovery unlock、新 TPM-backed nonexportable key + CA certificate、新 PUBLIC_KEY protector add/verify、strictly-higher unlock-authority epoch、TPM policy-protected NV head advance、normal-reboot broker unlock、旧 protector removal。`SsdDataHddRecoveryAndReenrollmentReadbackV1` 必须是一 operation ID、八步 `1..8`、递增 trusted time与 previous-step hash chain，所有 secret/leak counter为零且 `admission_opened_before_closure=false`。

## 5. 单故障场景演算

### 5.0 2026-08-26 已裁决场景（仅设计静态演算）

下表记录本轮已裁决的业务与硬件语义。它们的设计状态受 `development_state=READY_NOT_AUTHORIZED` 约束；所有相关代码、迁移、测试、硬件安装与实机/恢复证据仍为 `implementation_state=NOT_IMPLEMENTED`，不得据此宣称任何场景已通过。

| 场景 | 已裁决语义 | 当前证据边界 |
|---|---|---|
| 数量分层 | 报价、订单、履约/库存、收付与会计数量分别保有其业务含义；任何跨层拆分、合并、换算或冲销都必须保留来源、上限与追加式勾稽，不能以单一可覆写数量替代。 | 设计裁决；未实现、未运行测试。 |
| 直采 `award` | 直采的 `award` 是可审计的采购授予决定；它只绑定被选来源、数量/价格/条件与权限依据，不把外部生产者变成内部库存或第二权威写入者。 | 设计裁决；未实现、未运行测试。 |
| `PrincipalRefV1` | 授权、审计和查询使用当前、可验证的 `PrincipalRefV1`，而非固定岗位或展示名称；撤权、委托到期、法人/范围变化必须在写入及读取边界重新判定。 | 设计裁决；未实现、未运行测试。 |
| 维保无 `SKIPPED` | 维保、备份、恢复与安全门禁的不可用、超时、未知或失败均不得以 `SKIPPED` 转为通过或可认证结果；保持 hold/未认证并留下可审计原因。 | 设计裁决；未实现、无现场维保证据。 |
| 换盘灾恢 | 数据盘更换、丢失或损坏后只能使用洁净恢复硬件、分域恢复材料、已签名的有效恢复链与完整对账恢复；不得把旧盘、单份介质或历史摘要当作恢复成功。 | 设计裁决；未实现、未做实机换盘演练。 |
| root hold | root/全局维护 hold 必须先关闭 admission 并与 drain、barrier、full recovery cut/checkpoint 形成可验证顺序；它不能被局部路由、旧 permit 或调用方参数绕过。 | 设计裁决；未实现、未运行并发/恢复测试。 |
| UPS link-loss | UPS 链路丢失、状态过期或命令结果未知时按失联处理：停止风险动作并保持未认证；不得以 standard carrier、缓存状态或重发未知命令冒充可控电源证据。 | 设计裁决；未安装或验证实机 UPS。 |
| IaaS 独立 profile | `IAAS_WINDOWS_SERVER_HDD_STRICT` 是未来独立于物理 P340 的 authority profile；启用前必须由新 graph/profile version 分别证明客户租户控制、中国境内驻留、vTPM/Secure Boot、HDD/缓存/快照/临时盘/运营副本及故障域，物理载体证据不可复用。当前首版不实现、不接受该 profile。 | 未来接口裁决；当前固定 `NOT_IMPLEMENTED` / `PRODUCTION_NOT_READY`，未配置或认证 IaaS 载体。 |
| 日志保留 | 审计、应用、数据库与支持诊断日志的保留、位置、访问与缩短保留均受独立策略及证据约束；日志存在或文档描述不等于已满足留存。 | 设计裁决；未部署日志管道或保留证据。 |
| 域隔离 | 应用运行、签名、备份、恢复、生产启用及客户批准域必须相互隔离；任一 token、密钥、持有人、目标或权限复用都不能替代分域证明。 | 设计裁决；未部署、未实机验收。 |

### 5.1 权威事务与并发

| 场景 | 唯一结果 | 计划证据 |
|---|---|---|
| 重复点击、网络重放、回执丢失 | 相同 key+payload 返回原回执；不同 payload 拒绝；不重复扣减或生成义务 | G1/G4 |
| 两人同时修改同一对象 | CAS 只允许一个版本提交，另一方得到明确冲突 | G1/G2/G4 |
| 提交前任一点崩溃 | 五类写入全部为零 | G1 |
| 提交后响应丢失 | 重试读取同一 receipt，不重做业务 | G1/G4 |
| 执行中撤权/委托到期 | 提交前当前权限再验；后续 material effect 停止 | G1/G4 |
| maker 与 checker 为同一主体 | SoD 拒绝；换成不同当前授权主体才允许 | G1/G4 |
| 跨法人读取、排序、聚合或导出 | RLS 与 query-use 双重拒绝，且不能枚举数量侧信道 | G1/G4 |
| 伪造 GUC、ticket、nonce、backend 或 principal kind | 无法得到 `AuthorizedPgTx`；污染连接被销毁 | G1 |
| route admission check 通过后、handler 注册前 hold 抢先关闭 | 不存在该 gap；同一 deployment lock/CAS 只允许先 durable ACCEPTED lease 或先 durable `ADMISSION_CLOSED`，后者拒绝 permit | G1/G6 |
| permit 已发出后 hold 开始 drain | hold 先关闭新 admission，再等待所有 intersecting `ProductionAdmissionExecutionLeaseV1` 终结为 `COMPLETED_IN_PLACE\|HANDED_OFF_EXACT_ONCE`；UNKNOWN/orphan 保持 closed | G1/G6 |
| 同一 request ID 重入、换 payload/epoch、响应丢失或 terminal CAS 丢失 | `(deployment_id,request_id)` 唯一且 exact binding 含服务器派生 epoch，只采用同一 lease/result；跨 epoch replay、changed binding 或 terminal epoch drift 冲突，旧 epoch orphan ACCEPTED 继续阻止 drain，不能产生第二次执行或把 ACCEPTED 猜成 terminal | G1/G6 |
| 未登记 control/health 路由、`/control/*` prefix 或 bypass permit 进入 business repository | exact 十行 registry/selector/capability join 拒绝；bypass permit 无 `AuthorizedPgTx` 构造能力 | G1/G6 |

### 5.2 generation、包和自动化

| 场景 | 唯一结果 | 计划证据 |
|---|---|---|
| payload hash、generation number 或 graph hash 被冒充为 generation identity | 验证失败；只接受完整 canonical signed-envelope digest 且等于 manifest ref SHA-256 | G0/G1 |
| approval registry 被旧版本、邻接文件、自签根或 ambient trust 替换 | 产品固定部署信任、storage-policy digest pin、固定 DATA_HDD 派生路径或 revision/expiry 校验失败；generation 不签名、不激活 | G0/G1 |
| 四个 signer role 缺失/别名、key 可导出/ACL 跨角色，或 registry 与 storage pin 只安装一半 | provisioning/boot 失败且零 DB connection；只可从同一 fsynced rotation journal 恢复同一 pair | G1 |
| reverse plan 缺失、签名域错误、item/source 不匹配或动作/target/retention 组合非法 | manifest 签名前失败；当前 desired/OBSERVED 均不改变 | G0/G1 |
| reverse plan/manifest 崩溃恢复重生 ID、时间、动作或签名 | frozen creation-attempt exact-adopt 失败；不创建 declaration，prior OBSERVED 保持 | G1 |
| participant/client/plugin 提交 ACK 或跨 attempt 混 ACK | 输入不可表示/严格拒绝；OBSERVED 不移动 | G1 |
| plain ACK 携带 CMS/envelope，或 acknowledgement time 早于同 attempt durable start/晚于 OBSERVED commit | schema 或时序校验失败；OBSERVED 不移动，最终候选也不能冻结 | G0/G1/G6 |
| ACK 缺 `participant_apply_readback_ref`、apply/readback set 不 exact、package item transition 为 null，或 readiness 未闭合 operation result/installed state | G1-05 不构造 ACK；同 attempt OBSERVED 不移动 | G1/G5/G6 |
| 新安装 item 回滚伪造 predecessor，或 generation 1 以外使用 `NO_OBSERVED_GENERATION` | rollback readback 拒绝；新 item 只能 `DEACTIVATED_RETAIN_DATA` 并 exact 读回 `ABSENT`，其他代际必须绑定真 prior OBSERVED | G1/G5/G6 |
| generation 跳代、分叉或指向 desired-only predecessor | create-next 失败；当前 durable OBSERVED 保持不变 | G1 |
| graph/closure/readback participant IDs 不一一对应，ACTIVE/positive/declaration/generation-required/ACK participant IDs 不一一对应，database-consumer 投影不 exact，或 generation item/subset relation 缺失、多余、orphan、越界 | build/install/final-installed generation 失败；不冻结候选 | G6 |
| carrier/artifact/readback 跨型、host/supervisor 缺失成环，或固定九进程/installer side list 替代 graph projection | `WindowsRuntimeDeploymentClosureV1` 验证失败；不接受“进程在跑”作为替代 | G5/G6 |
| local AI deferred row 出现进程、端点、模型包、资源预留，或未使用 `NullAiProviderV1` | absence/产品语义测试失败；首版仍为明确不可用而不是隐藏启用 | G5/G6 |
| offline bundle 缺 runtime-deployment schema/closure/readback，或只用当前机器进程扫描替代冻结证据 | 离线证书验证失败；不能返回 `RELEASE_CERTIFIED` | G6 |
| 生产 build/install 后仍冻结安装前 generation，或 final-installed 代际不等于 predecessor+1 | `FinalInstalledGenerationAuthorityV1`/candidate freeze 失败；不会生成或采用候选 | G6 |
| final-installed attempt 在 manifest/declaration/dispatch/ACK/OBSERVED 任一切点崩溃 | 从 transition store 精确采用同一 attempt/prefix；不得扫描 latest、再开 attempt、重放已知 applied item 或重采冻结 ACK | G6 |
| raw/journal signer facade 请求直接 key access、错 role/discriminator 或绕过 G0 evidence broker，或 broker client 以 `GENERIC_WRITE`/第二 pipe instance 进入 | 服务 token/exact 18-object/action-row SDDL/operation allowlist 拒绝；facade 无 key ACE，G0 `EPF57EvidenceSignerBroker`/`F57EvidenceSignerV1` 仍是唯一 key owner/server instance，client 仅 `0x00120183` | G0/G6 |
| 部分 participant 激活 | observed 不移动，危险写入不混代 | G1/G5 |
| 激活失败或进程崩溃 | 保持旧 observed 或完整回滚，不删新数据 | G1/G5 |
| 长流程跨 generation | 新命令使用新代；旧流程继续钉住原版本 | G1/G5 |
| 模块停用 | 新入口关闭；历史数据与证据保留 | G5 |
| signed package 不是 exact 十三字段、缺/多 `implementation_manifest_ref`、八字段 implementation manifest 不 exact、tagged artifact/class 跨型、实现字节/SBOM/签名/迁移引用不闭合、复制/放宽 `PermissionCeilingV1`，或选择低于 class 最低 grade/executor/rollback 策略 | schema/generated-descriptor/domain verifier/compiler 拒绝；任何未在 implementation manifest 中的字节不可 stage，当前 generation/OBSERVED 不变 | G5/G6 |
| package registry 缺 exact media、digest/SPKI双pin、固定DN/chain/revocation，pair rotation中断，或desired-state item未携portable ref | boot/generation/offline verifier失败关闭；禁止邻接/latest/ambient替代 | G5/G6 |
| generation/package trust 各自修改 storage manifest、并发轮换产生双 winner、stale base 或半安装 pair | 唯一 `AuthorityStorageManifestRotationCoordinatorV1` 的 global lock/monotonic CAS/hash-chain 拒绝或 clean rebase；崩溃只采用同一 immutable bytes，boot 保持关闭 | G1/G5/G6 |
| hotplug 在 tuple/drain/switch/probe/rollback/restore intent或response任一切点崩溃 | 只按持久化operation ID query/adopt同一 forward/rollback attempt；不重发未知副作用、不重采tuple | G5/G6 |
| drain 中存在 UNKNOWN/未交接 work 或 timeout | 保持 DRAINING/不切换；timeout 不是 kill 或推进许可 | G5 |
| desired-state item 携带 plan/decision/checkpoint/window/attempt，或运行时没有独立 per-attempt transition | schema/上层 verifier 拒绝；不能用过期权限污染 generation，也不能用不可达操作假写 ACK | G5/G6 |
| graph slot/scope 不是 Authority 从 compiled graph + current tenancy snapshot 派生，或 per-participant tuple 中任一 attempt/deployment/scope/epoch/generation/participant/item 字节漂移 | 私有 binding 不可构造；每 participant 独立失败，不借用另一 participant 的 trust/result/ACK | G5/G6 |
| maintenance plan 预绑 actual checkpoint，或未先 `ADMISSION_CLOSED -> DRAIN_COMPLETE -> BARRIER_COMMITTED` 就冻结 cut/执行 privileged intent | 两阶段 verifier/CAS 拒绝；路由保持 hold，只允许同 barrier 的 full recovery cut/checkpoint 后构造新鲜 execution authorization | G5/G6 |
| 30-field plan 漂移/跨 item，typed `[INITIATOR,APPROVER]` decision、customer signer、窗口、probes、reverse plan、source readback、trust snapshot、hold/full-cut/checkpoint 任一不符 | 私有 maintenance execution authorization 构造失败；第一副作用前停止，raw ref/latest checkpoint/调用者 ID 不可表示 | G5/G6 |
| plan finalization 在 freeze/authorization/provider/spool/object/bind 崩溃，或同一 plan 被第二 activation 重放 | DATA_HDD finalization journal/package-store CAS 只采用原 ID/bytes/tuple；冲突终止，过期 live authorization 在 forward intent 前只能保留同 pure item/历史 plan 并用 fresh decisions 增加 execution ordinal。Finalization journal 不进离线 bundle | G5/G6 |
| `EPAuthorityControl` POWER/PACKAGE 协议串权，Scheduled Task action/account/S4U rights/account flags/runtime-token/SDDL/allowlist 漂移，Task Scheduler 存 password，或请求携 argv/path/SQL/shell | exact 18-object SDDL、typed protocol 和 `RECOVERY_TOOL` static+live verifier 在副作用前拒绝；desired XML/SDDL echo 不算 live proof | G6 |
| package 修改任一 SCM executable/raw `ImagePath`、raw command line 不是 quoted executable + exact role argv、调用 `ChangeServiceConfig`、覆盖 fixed launcher，或 launcher/kernel DLL ABI、slot-pointer/head 不匹配 | MSI/SCM parser round-trip、recovery-domain verifier/boot 失败关闭；保留旧 pointer/slot，不开 production admission | G6 |
| ABI generator `--check` 有 diff，DLL 出现第二/forwarded/ordinal-only export，或 ABI version/size/offset/section/held-file/import allowlist 漂移 | Windows Server 2022/MSVC precommit 与 build/install readback失败；不生成 Authority artifact set | G6 |
| package 在forward失败后自行rollback，或rollback request未匹配durable `ROLLBACK_STARTED` ID | rollback端点拒绝；不执行reverse；由coordinator串行重新派发exact subset | G1/G6 |
| switch 后 probe 失败 | package 只返回 FAILED/UNKNOWN；coordinator 先持久化 `ROLLBACK_STARTED` 与 rollback ID，再派发 exact reverse subset；control broker恢复 prior desired-state item，或 recovery tool按绑定checkpoint恢复并测量 predecessor；旧版本/数据 pin 保持到上层提交 | G5/G6 |
| package 全 participant 已 `APPLIED_VERIFIED`/OBSERVED，但无 exact `ProductionGenerationAdmissionV1`，或 admission digest/scope/ACK/transition/predecessor/resource envelope 漂移 | 唯一生产路由门保持关闭；只有 `GENESIS_FULL_CERTIFICATION\|PACKAGE_DELTA\|ROLLBACK_REOPEN` 的合法 CAS 可开对应 impacted scope | G6 |
| 两包声明同一事实 owner | 编译/安装/激活失败，当前代不变 | G0/G5 |
| 外部动作成功但响应丢失 | `DISPATCHED→UNKNOWN`，Objective→`RECONCILING`，零盲重试 | G1/G4/G5 |
| 对账证据与原判断相反 | `CONFLICTED` / `INCIDENT`，不得篡改历史 | G1/G4 |
| worker 重启或重复领取 | 从 checkpoint exact-once 继续 | G1/G4 |
| 两个验证进程同时领取同一 TestID | journal OS 独占 lease 只允许一个进程写 `STARTED`；另一进程在任何副作用前以 `F57_JOURNAL_BUSY` 失败 | G0/G4/G6 |
| 已签 evidence 落盘后、journal bind 前崩溃 | 重启 typed-verify 并采用原字节，只补一次 bound event；不重签、不覆盖 | G0/G4/G6 |

### 5.3 客户端、离线和门户

| 场景 | 唯一结果 | 计划证据 |
|---|---|---|
| 离线尝试付款、合同生效、最终审批、库存或权限修改 | 只保存待提交意图；服务器重连后重新鉴权 | G5 |
| 两设备离线修改普通字段 | 可证明不冲突才合并，否则人工冲突项 | G5 |
| 离线修改金额、数量、状态、合同条款或权限 | 永不自动合并 | G5 |
| 旧客户端/旧 generation 提交 | 兼容则重验；不兼容或安全语义过期则拒绝升级 | G3/G5 |
| 门户越 party/法人/白名单 | gateway 拒绝；无数据库连接和数量侧信道 | G5 |
| Tauri 2 任一平台未过硬门 | 主线停止，唯一回退为完整 Flutter + Rust 分支 | G5 |

### 5.4 Provider、MCP、Office 和文件

| 场景 | 唯一结果 | 计划证据 |
|---|---|---|
| MCP/AI 提示要求越权或读密钥 | capability、对象、字段、网络、文件、密钥、审批逐项拒绝 | G5 |
| Provider 未认证、撤销、漂移或过期 | 保持关闭；不回退到未批准 carrier | G4/G5 |
| 外部数据库被连接 | 只作为 provider；不能成为第二权威 writer | G5/G6 |
| Excel 公式、宏、隐藏列、外链或错行 | 作为 proposal 隔离解析、逐行验证；Excel/VBA 无 PG 凭据 | G4/G5 |
| 恶意附件、压缩炸弹、扫描超时/过期/未知 | 始终留在 HDD quarantine | G2/G4/G5 |
| 扫描后字节、路径或卷被替换 | final handle/volume/digest 复核失败，重新隔离 | G2/G6 |

### 5.5 HDD、时间、电源、勒索和恢复

| 场景 | 唯一结果 | 计划证据 |
|---|---|---|
| junction/mount/ADS/hardlink 把客户字节导向 SSD | 按最终句柄与设备身份阻断，首次 DB connect 之前失败 | G1/G6 |
| RUNTIME_SSD entry 既不属于 signed reproducible Set A，也不属于 exact four-rule/twenty-path Set B，Set A bytes/catalog 不符，八行 persistent policy 或七行 telemetry policy 漂移，或任一 entry 含客户/业务 authority 字节 | whole-volume/locator/ADS/hard-link/VSS/canary scan 失败，Residency/P340/admission关闭；清除后必须重跑 SSD-loss 重建和全策略 readback | G1/G6 |
| SSD 丢失后从本地 latest/单一 pointer/未认证 cache 重建，或 terminal capsule 无 off-host/HDD 终态证据就退役 | 头部 quorum/恢复域校验失败；只能从 authenticated DATA_HDD + off-host 最高一致 head 重建 control，从 signed manifest 重建代码 | G6 |
| HDD 黄线/红线 | 先停报表和低优先级任务；保留交互保存、审计和 WAL；无法安全写入则停危险写 | G1/G4/G6 |
| governor 故障 | 重任务失败关闭；绝不节流 PostgreSQL/WAL | G1 |
| W32Time 停止、回拨或快进 | duration 用单调钟；高风险、签名和发布失败关闭 | G1/G6 |
| UPS 失联或低电量 | 固定顺序停长任务、checkpoint、停 PG、关 Windows | G6 |
| UPS 使用 standard carrier 冒充可控 outlet；manifest implementation binary 不等于候选 kernel/held binary；configuration projection/generation/digest、service/endpoint/credential 漂移；`provider_operation_id` 为空、非 canonical ASCII、改变或跨 command；同 command ID 改 digest；状态未知后重发；或 boot change 后补造 ACK | `CAPABILITY_INSUFFICIENT\|COMMAND_ID_CONFLICT\|COMMAND_STATE_UNKNOWN\|DISPATCH_ACKNOWLEDGEMENT_ABSENT_AFTER_BOOT_CHANGE`；POWER 非 PASS，不重发、不重建；直接 UPS package/test gate 失败 | G6 |
| 五个 Authority launcher-role 服务缺失/多出/角色错位，固定 G0 broker或三 component service 缺失，product-owned SCM 不等于九行，完整 host 不等于 `10+active_additional_windows_service_count`，continuation 未休眠或动态身份漂移 | Windows install/POWER carrier 失败；不冻结候选、不签发布证书 | G6 |
| exact 18-object/action-row SDDL 漂移、facade 获得 key ACE、activation 被当作 registry value ACL 或永久对象被尝试修复 | 在副作用/关机前失败关闭；只读验证 permanent descriptor，不自动修复 | G6 |
| writer/checkpoint-signer/unlock-broker 合并权限、复用 Authority-role/另一组件 runtime challenge、恢复 helper 未签名、target agent 被装在 P340，或六行组件/五 on-host/三 service 闭集不完整 | component_id-bound build/install/PITR/恢复链失败；不进入 final-installed generation | G6 |
| PostgreSQL package `installed_files` 与 SBOM/engine final-handle 枚举不双射、有 missing/extra/alias/reparse/ADS/hard-link，existing lock/build 不是 clean 或 exact same-lock adopt，或四个 system identifier 不同 | package/install verifier 拒绝；异 build 只返回 `MAINTENANCE_UPGRADE_REQUIRED`，22-field install evidence 不可签发且不改变服务/数据 | G6 |
| Windows install 中 PostgreSQL contract/readback 缺失；未解析 SDDL template 被写入 live ACL、SID substitution/live DACL 不一致；service/account/start/recovery/ACL 漂移；DATA_HDD 解锁前已有进程；runtime 非 typed `RUNNING`；或 PGDATA/WAL/temp/log/TLS/config 任一落错卷/被 ambient override | 22-field install evidence 不可签发；不进入 final-installed generation，PostgreSQL 与业务路由保持关闭 | G6 |
| PostgreSQL `max_connections` 不是 64 或 pools+三类 reserve 超限；`logging_collector=on`、`log_destination=stderr`、DATA_HDD directory、`postgresql-%Y-%m-%d_%H%M%S.log`、24h/100MB、truncate-off、`EnterprisePlatform.PostgreSQL16`/early `PostgreSQL` sources、server-eventlog-off/customer-token-zero 任一漂移；`wal_sync_method` 不是 `fsync_writethrough`；同盘 `pg_test_fsync` 不支持、吞吐非正、出现 I/O error 或其卷/驱动/write-cache/tool binding 失效 | 配置/耐久性/日志 gate 失败；禁止启库、PITR、发布与生产启用，必须对变化后的硬件栈重跑 qualification | G6 |
| 勒索者删除服务器备份 | writer 无枚举/删除/覆盖/缩短保留权限；外部 checkpoint 暴露破坏 | G6 |
| trust pointer 或 topology 不是 active-config current；trust manifest/pointer generation/predecessor 不连续；由候选/自身/storage/support/应用或备份恢复域/ADR-0020 roster/ambient root 自证 signer；独立 trust/revocation/checkpoint 过期；或 current authority-storage manifest/deployment/epoch/generation/singleton target join 不成立 | topology authority 拒绝；boot/install/checkpoint/PITR/activation 全部失败关闭，不能以较新文件名或时间戳替代 current refs | G6 |
| clean install 的 `INITIALIZING + INITIAL_POPULATION` 读回非空，或把空链当作 PITR/发布/启用依据；sequence 1 未进入 `BOOTSTRAPPING`，未先验证到 A/B 就续链，达到 minimum 后仍不健康；roots 轮换不是 fresh-HEALTHY→单 bridge→只复制的 bootstrap，或健康前再次轮换；retained refs 未 typed-load、断序/分叉/head 漂移；A/B 不是连续链子集、并集不含 latest/minimum generations，或 support evidence 不闭合 | 基础设施安装只可保持 `INITIALIZING`；合法自举/轮换严格按 immutable transition 推进；任一非健康状态拒绝 PITR/发布/恢复认证/activation，业务路由保持关闭；只有新鲜 `HEALTHY` explicit current head 才可继续 | G6 |
| 离线介质 transition 不属于八条 exact 边、sealed 后复写、destroyed media 复用同 `media_id`，或 sequence/predecessor/state/head hash 任一跳变/分叉 | `NON_SUPPRESSIBLE_RISK`；介质链失效且不得计入 A/B 健康轮换，必须以新 `media_id` 从 sequence 1 `BLANK` 重新登记 | G6 |
| topology 六角色/target/A/B/域隔离/保留或容量公式不合格，permission probe 未拒绝，partial/history/reserve 未知，just-written token 可二次读取，或 activation/checkpoint retry 复用旧 safeguard | `NON_SUPPRESSIBLE_RISK` 或 typed binding failure；install/checkpoint/PITR/activation 对应阶段失败，业务路由保持关闭 | G6 |
| 最新备份被投毒 | 选择更早已签名 clean cut，恢复后全面对账 | G6 |
| 一块离线盘损坏/被盗 | 另一 distinct 介质仍物理断开且可恢复；密文不因盗盘泄露 | G6 |
| ordinary reboot 的 persisted PCP key/certificate binding、locator/bootstrap registry/NV/GPO/chain或 WMI required ACE 漂移 | dedicated unlock broker fail closed；DATA_HDD保持锁定，Authority业务面不启动；不得改用 auto-unlock/recovery task/ambient administrator | G6 |
| AS/RT 创建 unlock pipe instance、broker WMI 远程/降级认证/委托、namespace ACE 超过 `0x00000003`、thumbprint=0/PIN非空/alternate volume-method | `0x00120183`/no-second-instance、local packet-privacy + impersonate、forbidden `0xFC` 与 fixed `UnlockWithCertificateThumbprint` negatives拒绝 | G6 |
| TPM/主板/OS SSD 损坏后声称用 DER/SPKI/TPM handle 重建旧私钥，或 clean-SSD 未完成双人 recovery-password 八步 reenrollment就开 admission | 失败关闭；只能用 off-host 48-digit recovery-password ceremony 解锁，创建新 TPM-backed nonexportable key/CA cert/PUBLIC_KEY protector、提升 epoch/NV、正常重启验证后删旧 protector；全程零 secret leak | G6 |
| recovery cut 只含 PostgreSQL/附件，或缺/多/重复 authority class/root，不同 row 来自不同 barrier/cut，或三次洁净恢复未每行 exact-verify | `AuthorityRecoveryCutManifestV1`/checkpoint/恢复闭包失败；即使 PostgreSQL 能启动，业务写入和 production admission 仍保持关闭 | G4/G6 |

### 5.6 业务长周期

| 场景 | 唯一结果 | 计划证据 |
|---|---|---|
| STANDARD 销售 | 订单、采购/库存、交付、销售开票、收款和证据可勾稽 | G2/G4/G5 |
| DROP_SHIP | 不伪造自有库存移动，仍闭合采购、交付、发票和资金 | G5 |
| 六来源采购 | 来源 exact-one，合并/拆分数量守恒，外部生产只是 provider | G5 |
| 部分收货、交付、收付、退货或红冲 | 不超过上游；更正只追加；历史切片不改写 | G5 |
| 经营期间锁定后迟到事实 | 期间永不重开；顺延下一开放期间并保留原日期和依据 | G5 |
| 投诉/售后/周期维保 | 权益、配件、工时、根因、回访证据满足才关闭；复发开新 cycle | G5 |
| 项目里程碑、成本和收款 | 各事实唯一 owner，报告只读聚合并可下钻证据 | G5 |
| 客户/供应商门户协作 | 只提交白名单命令，不复制第二份业务事实 | G5 |
| 自定义模型含循环、危险类型、冲突 owner 或保护区字段 | 编译失败，当前 generation 不变 | G5 |
| 寄售、订阅、租赁、本地模型被请求 | 只有禁用且版本化 seam，不宣称完整能力 | G5/G6 |

### 5.7 发布证书与生产启用

| 场景 | 唯一结果 | 计划证据 |
|---|---|---|
| L3 已签发 `RELEASE_CERTIFIED`，但没有客户接受记录 | 控制面/恢复面可用，业务命令/查询持续隔离；不是生产启用 | G6 |
| 接受记录只有一名批准人、批准人相同/已撤权、过期或未覆盖 exact 五项风险 | 输入验证在 activation attempt 创建前失败；业务路由保持关闭 | G6 |
| 接受记录与部署、候选、证书或 20-user profile 不一致 | activation begin/adopt 在 attempt 创建前拒绝，不允许“差不多相同”的批准 | G6 |
| 合法 request 后 collector 不可用；topology/runtime-deployment/installed-component/storage-safeguard 任一 live readback 漂移；或 safeguard 不是独立信任的 current topology + current storage manifest + exact checkpoint head 的 fresh `HEALTHY` 读回且其 typed retained/support/A-B 闭包不完整 | 追加 typed `FAILED_HELD` 并保留同一 `activation_id`/完整历史；runtime deployment 使用 `RuntimeDeploymentDrift`，业务路由保持关闭且不永久死锁 | G6 |
| 故障修复后用相同命令恢复 | 重新验证同一仍有效证书和双人接受，以 exact prior failure record hash CAS 追加递增 ordinal 的 `RETRY_COMMITTED`，完整重采四类 live readback | G6 |
| retry 创建第二 activation ID、跳过/复用旧 readback、failure hash/ordinal 不连续或提前开放路由 | CAS/状态机拒绝；原 attempt 保持 `FAILED_HELD` 或当前非终态，仍隔离 | G6 |
| `ACTIVATED` 提交后响应丢失 | exact request 重入返回同一 activation 和 `business_api_generation`；不创建第二次启用 | G6 |
| 另一个证书或并发 stale writer 尝试再次启用同 deployment/epoch | 部署锁、CAS 与 partial unique 约束拒绝；已有终态不被覆盖 | G6 |
| activation 已 `ACTIVATED` 但未同步生成 `GENESIS_FULL_CERTIFICATION` admission，或 router 只检查 activation row | 终态 CAS 不完整/路由门失败关闭；业务命令/查询仍隔离 | G6 |
| generated router 与 exact 十行 bypass registry 不一一对应，或 `WindowsAuthorityArtifactSetV1.production_admission_bypass_registry_ref` → `ReleaseWindowsServiceInstallEvidenceV1.production_admission_bypass_registry_ref`/installed bytes → signed-artifact-set-only final candidate → startup typed-load 的 ref/bytes/media 链缺失或漂移 | build/install/final candidate/startup typed join失败；不能用当前 router 扫描、ambient/latest file、prefix 或 handler Boolean补链 | G6 |
| business gate 只读 admission 后才异步登记 in-flight，lease 缺失/漂移服务器派生 authority epoch，或 hold/drain 使用另一把锁、仅统计当前 epoch、使用进程内计数 | `ProductionAdmissionExecutionLeaseV1` same-lock/epoch linearization gate失败；不给 handler permit，旧 epoch orphan不能被忽略，barrier 后业务写视为 P0 failure | G1/G6 |
| package delta 只关闭部分 impacted scope，或新 OBSERVED 无 exact ACK/transition/predecessor/resource envelope admission | 部署级 atomic admission gate 拒绝；所有 impacted scope 保持 hold，不允许半新半旧对外服务 | G6 |
| rollback 尝试复用旧 predecessor readback 或直接恢复旧 admission | `ROLLBACK_REOPEN` 拒绝；必须对恢复后的 exact predecessor 重采 fresh readback/ACK 才可重开 | G6 |

## 6. 组合故障演算

单场景绿灯不足。计划必须对下列组合做 pairwise，并对金额、库存、权限、不可逆外部效果和恢复做有针对性的三因素组合：

- 断电 × 满盘 × 事务阶段；
- 撤权 × generation 切换 × 离线重连；
- provider 超时 × 重复回调 × 权限过期；
- 包停用 × 长流程 × schema 升级；
- 恶意附件 × 路径替换 × scanner 定义过期；
- manifest 回滚 × OS SSD 损坏 × 缺少一个密钥保管人；
- 勒索 × 最新备份投毒 × 一块离线介质不可用；
- 20 人重叠负载 × 重报表 × 备份 × HDD 高水位；
- 期间锁定 × 迟到事实 × 部分退款/红冲；
- authority 分区 × 旧主复活 × Outbox 重放。
- journal owner 崩溃 × 已落盘未 bind envelope × 并发恢复进程；
- 72 小时物理测试 × L2 有效期 × 最短输入 expiry；
- clean-HEAD build × fixed nine product-owned SCM + `ep-postgres16` + graph-active service inventory × six-component/five-on-host install × final-installed OBSERVED selection；
- graph delivery 全集 × ACTIVE-only topology/generation 子集 × DEFERRED_DISABLED absence；
- hotplug grade × UNKNOWN drain item × prepare/switch/probe/rollback crash cut；
- pure desired-state item/per-attempt transition × 13-field package/8-field implementation manifest/tagged artifact × graph slot/concrete tenancy scope/component class/executor/rollback strategy × 生命周期 action matrix × per-participant tuple × typed 双人授权/window × registry 双 pin/portable locator × hold/drain/barrier/full-cut/checkpoint/execution-authorization × finalization/execution operation-ID crash cuts × coordinator rollback request × 旧版本 pin/数据保留；
- participant apply/rollback readback × 新安装 item `DEACTIVATED_RETAIN_DATA`/`ABSENT` × generation 1 `NO_OBSERVED_GENERATION` × 14-field ACK/OBSERVED CAS；
- generation/package 并发 trust rotation × global storage-manifest CAS × 中途断电/恢复；
- quoted launcher+role argv × one-export generator/ABI/slot-pointer rollback × immutable S4U Scheduled Task/runtime token × POWER/PACKAGE protocol isolation；
- full HDD recovery cut × 恢复跨 barrier/cut × 三次 clean-hardware exact row-set 校验；
- PostgreSQL 五 strict root/安装认证 × installed-file/SBOM 双射 × unresolved-template→live-DACL SID 替换 × 四方 system identifier/RUNNING × `64/4/3` GUC 与 NORMAL/RESERVED/SUPERUSER 分类预算 × HBA/client channel-binding probe 分离 × Event Log provider/bookmark/gap/fixture 完整覆盖 × 同文件双 fsync qualification 与 Task-15 UPS/power-cut join；
- independent trust-manifest authority/current trust pointer/topology signer × current storage-manifest singleton target × fresh checkpoint-preparation readback × `INITIALIZING→BOOTSTRAPPING→HEALTHY` initial population × `HEALTHY→TRANSITIONING→BOOTSTRAPPING→HEALTHY` roots rotation × typed retained chain/A-B subset/latest/minimum/support evidence；
- 离线介质八条 exact lifecycle edge × sequence/predecessor/head hash × sealed/destroyed terminality × 新 media ID 重新登记；
- UPS candidate-held implementation binary × configuration projection digest/generation × canonical provider operation ID durable binding × direct contract/adapter package tests；
- RUNTIME_SSD Set A/Set B × eight-row persistent/seven-row telemetry × locator/ADS/hard-link/VSS 全卷 scan × SSD loss/off-host-head quorum/终态 capsule 退役；
- ordinary PUBLIC_KEY reboot unlock × nine-locator/bootstrap/WMI restricted-token readback × clean-SSD 双人八步 reenrollment；
- 关机跨重启 × exact 18-object/action-row SDDL 漂移 × 无密钥 facade/G0 broker `0x00120183`/no-second-instance 角色边界；
- `RELEASE_CERTIFIED` × 双人单盘风险接受 × 新鲜现场读回漂移；
- production activation `FAILED_HELD` × prior-failure-hash CAS retry × 四类 live readback 全量重采；
- production activation 响应丢失 × 并发 stale writer × deployment/epoch 唯一终态；
- OBSERVED/package delta/rollback × impacted-scope atomic hold × `GENESIS_FULL_CERTIFICATION|PACKAGE_DELTA|ROLLBACK_REOPEN` admission × router digest gate；
- generated router/exact ten-row bypass registry × artifact-set/install/candidate ref chain × `ProductionAdmissionExecutionLeaseV1` same-lock permit/hold/drain races；

守恒、幂等、单调版本、事实不可变、权限不扩大、单 writer 和证据可追溯使用属性测试；每个事务、外部 effect、备份和恢复阶段都有故障注入点。

## 7. 一年可行性复核

现有 [一年可行性演算](../../analysis/2026-08-24-f57-one-year-feasibility.ipynb)与其 SQL 来源仍作为容量和商业假设材料；其中旧 `Task 25` 字样只解释为现行 G6/L3，不是旧计划恢复。

静态结论保持：

- 对一家约 20 名活跃用户、附件量受控的合同型企业，1TB HDD 在一年净容量上有较大概率够用，但这不证明随机 I/O、满盘、断电、温度或恢复时间；必须由 G6 的真实 72 小时混合负载和恢复演练决定。
- 20 人不是登录硬上限；认证负载是 15 Workbench + 3 客户门户 + 2 供应商门户，同时保留 1 个 Control Center，会叠加自动化、增量备份、审计 checkpoint 和 1 个重报表。
- 32GB RAM 和单 HDD 要求有界连接、重任务单并发、动态降级和扩盘水位。容量证书接近阈值时必须先加 HDD/内存或降低客户负载，不能改文档放宽。
- 现有 256GB SSD 承载 Windows、固定 launcher/版本化可重建程序和缓存，但持久判断必须使用 Set A + Set B：Set A exact-inventory 可重建 Windows/product/cache/reenrollment metadata；Set B 只有 POWER capsule、package recovery capsule、kernel pointer/head、signed code slot/cache 四个 mutable class。另有 exact 八行 Windows persistent policy与七行 telemetry policy防止 pagefile/dump/WER/VSS/quarantine/EventLog/ETL/Defender/firewall/HTTP.sys 偷渡第五类；闭集外任何 entry 或客户/业务 authority 字节都失败。认证要求 raw capacity 至少 240,000,000,000 bytes、空闲至少 40GiB 并保留 20GiB 更新/回滚预算，同时必须通过全卷 residency、off-host mirror、content-addressed restage 与 SSD-loss 重建。1TB 数据盘必须实测 CMR、厂商工作负载额定至少 55TB/年且保修覆盖证据有效期；不满足就先换盘，不能靠阈值解释通过。
- 单人 + Codex 一年内更合理的工程目标是 G0、权威主干、CTC-01 和受控设计伙伴试点；不能预先承诺完整四端、供应链、P340、勒索恢复和 L3 必然在固定日期完成。
- “300 家 × 32 万元年费 + 50 次 × 8 万元启用费 = 1 亿元”是成熟规模算式，不是第一年预测。第一年应证明 1–3 家设计伙伴、3–10 个付费试点，以及部署、升级、恢复和行业包可复制。

## 8. 开发启动与停止条件

开发启动必须同时满足：

1. 用户另行明确授权开始开发；
2. 保存当前未提交工作并在隔离工作区执行；
3. 只启动 G0，不并行偷跑 G1–G6；
4. G0 每个任务遵循测试先行、窄提交和完成前验证；
5. 后续阶段只消费同一 candidate/tree/graph/generator/baseline/apply/F57-migration/toolchain/gate-run identity 的已验证 aggregate receipt；旧 standalone receipt 只保留为历史。

出现以下任一情况，立即停止对应主题并先修订规范/计划：

- 两个事实写 owner、第二权威 writer、客户端/插件/网关直连数据库；
- 客户数据或可关联衍生持久化到 authority SSD；
- `UNKNOWN` 外部结果无对账却被重试或关闭；
- 迁移版本/路径冲突、Fresh PG 不通过、生成投影漂移；
- generation whole-envelope identity/previous OBSERVED 链漂移、四角色 generation signer 未隔离/可导出、approval registry 与 storage pin 未经同一 journal 成对安装或未经产品固定信任验证、reverse plan 不是从 frozen prior-OBSERVED/policy attempt 派生或 item/source/action/retention 不一致、plan/manifest 重签而非 exact-adopt、manifest/reverse-plan/ACK 不是 exact 13/9/14、ACK 缺 apply-readback ref 或不是 exact server-derived same-attempt set、apply/rollback readback tag/集合/前代绑定漂移、generation 混代、pin 可被提前回收或停用删除历史数据；
- graph/closure/readback participant IDs 不一一对应，ACTIVE/positive/declaration/generation-required/ACK participant IDs 不一一对应，database-consumer projection 或 generation item/subset relation 不 exact，出现 orphan/out-of-set item，DEFERRED_DISABLED 携带 artifact 或混入任一 active relation，或恢复固定九进程/installer side list；
- package payload 不是 exact 13 字段、缺 `implementation_manifest_ref|hotplug_contract`、八字段 implementation manifest/tagged artifact 不能闭合全部实现字节/SBOM/签名/迁移引用、component class 降低 hotplug grade、desired item 不是 pure 九字段或缺 per-attempt transition、UNKNOWN/drain timeout 推进、per-participant tuple/scope 漂移、package/plan/trust-registry wire 或 signer-role 漂移、generation/package trust 绕过 global rotation coordinator，maintenance 未遵守 hold→drain→barrier→full-cut/checkpoint→fresh execution-authorization 两阶段并在第一副作用前绑定全部 source/target/probe/reverse/trust/readback/ref exact tuple、同 plan 二次 forward，或 crash recovery 丢失原 tuple/旧版本 pin/数据保留；
- 最终候选未绑定同运行 build/install 后的恰好下一代 `OBSERVED_COMMITTED`、未通过租约固定的 observed selection，或从 latest/目录扫描/调用方 manifest 代替 `FinalInstalledGenerationAuthorityV1` 的 transition store；
- Windows production manifest/MSI/readback 不是 exact five Authority launcher roles + fixed G0 broker + three component services 的九个 product-owned SCM、完整 host 不等于 `10+active_additional_windows_service_count`，或备份/恢复组件不是 exact 六行/五 on-host/三 service + one off-host；`RECOVERY_TOOL` 不是 fixed `EPF57Recovery` S4U action/account rights/RequiredPrivileges/flags/groups/runtime-token/SDDL/`0x00120183` IPC/six-operation allowlist 的 immutable Scheduled Task，接受 argv/path/SQL/shell；unlock broker 不是 restricted LocalSystem/no-network/PUBLIC_KEY boundary；component challenge 未绑定 exact `component_id` 或复用了 Authority `service_role`；continuation 无 activation 时未 dormancy，或 Windows Server 2022/MSVC 强制 job 被跳过；
- PostgreSQL 五 strict root（19-field package lock、13-field install contract、4-field Event Log fixture、19-field coverage、17-field readback）没有 sole owner/schema 或 exact 23/22-field signed-parent 认证；`installed_files`↔SBOM/final-handle、SDDL→live DACL、四方 identifier、typed `RUNNING` 漂移；`64/4/3` GUC、两槽 safety、NORMAL/RESERVED/SUPERUSER 分类预算/role attribute 不闭合，应用可吃保留位；HBA 冒充 channel-binding 证明或 client probe 缺失；Event Log provider/bookmark/record/time/clear/drop/gap/fixture/digest/coverage 证据缺失、截断、错配或 token 命中；把 `fsync_writethrough` 文本或单方法 qualification 冒充当前 driver/cache/UPS/flush/power-cut 耐久性；安装员/环境/CLI 能改变服务/路径/有效配置；或 PG 在 DATA_HDD gate 前启动；
- `BackupTopologySigningTrustManifestV1|BackupTopologySigningTrustCurrentPointerV1|BackupTopologyV1|StorageSafeguardReadbackV1|StorageSafeguardSupportEvidenceV1|BackupProtectionTransitionV1` 缺独立 trust-manifest authority、active-config dual-current refs、private verified-current topology authority、current singleton-target storage join、六角色/target/A-B/域隔离/retention/capacity/permission/一次性读取/fresh retry 约束；把空链 `INITIALIZING` 当健康，sequence 1 未进入 `BOOTSTRAPPING` 或未闭合 A/B/minimum 就健康，roots rotation 缺单一 bridge/不可变 anchor/只复制阶段或允许嵌套轮换，checkpoint 未经 cut 后 fresh preparation，retained chain/head/support closure 不成立，PITR/activation 不是 fresh `HEALTHY` explicit head，或介质链不属于八条 exact edge；
- UPS 不是 exact 16/20/21/28-field typed manifest/status/command/ACK，release/authority-kernel 缺 UPS contract/adapter 直接依赖或强制 tests；standard carrier 被用于最高档控制；implementation binary/config projection/provider operation binding 漂移；vendor adapter 越过 service-SID/endpoint/credential 边界；或 command unknown 被重发；
- raw/journal signer facade 持有/获授 CNG key、未只转发给 G0 `EPF57EvidenceSignerBroker/F57EvidenceSignerV1`，broker/client/AS 可创建 second pipe instance或 client rights不是 concrete `0x00120183`，POWER/PACKAGE 未在同一 fixed control pipe 按 discriminator 隔离或不是 exact 18-object SDDL；任一 Authority SCM raw command line 不是 quoted fixed launcher + exact role argv，package 改 SCM/覆盖 launcher，唯一 ABI generator `--check` 不 clean、DLL 不是 one named non-forwarded export、ABI/slot pointer/head 漂移，或 permanent descriptor 被自动修复；
- 任一平台/硬件/备份/恢复/签名 lane 缺失、空跑、过期或属于不同候选；
- 候选证据命令缺少显式 run-journal/bundle-root、绕过 closed carrier dispatcher、journal lease/hash/checkpoint 非严格延伸，或 aggregate 延长输入有效期；
- signer registry 不是 exact 89 行、carrier 输入未由 signed staging plan 固定、offline schema manifest 不是包含 implementation/participant-readback/transition/operation/full-cut/production-admission media 的最终证书完整传递闭包、离线复制/回放 plan-finalization journal，或任何 ref 被二次签名/目录猜测替代；
- `AuthorityRecoveryCutManifestV1` 不等于 storage manifest 全部 enabled HDD authority class/root exact-set、混用 barrier/cut、三次洁净恢复未每行 exact-verify，试图用同盘副本、RAID、暖备、云快照或当前单 HDD 替代勒索恢复；RUNTIME_SSD 不能 exact partition 为 signed Set A + four-rule/twenty-path Set B、八行 persistent或七行 telemetry policy漂移；DATA_HDD ordinary PUBLIC_KEY unlock 未闭合九 locator/bootstrap/WMI/restricted-token真机 readback，或 clean-SSD绕过双人八步 key/certificate/protector reenrollment；
- generated router 与 exact 十行 admission-bypass registry 不一一对应，`WindowsAuthorityArtifactSetV1`→service-install evidence→final candidate 的 registry ref/bytes/media 链漂移，业务请求未在 admission/hold 共用 deployment lock中先以服务器派生且 exact-matched authority epoch create/adopt exact twenty-field `ProductionAdmissionExecutionLeaseV1{ACCEPTED}`，唯一约束错误地包含 epoch、hold/drain使用另一把锁/仅统计当前 epoch/进程计数，或 barrier 后可观察到新业务写；
- 把 `RELEASE_CERTIFIED`、`ACTIVATED` row、package-local `APPLIED_VERIFIED` 或 OBSERVED 单独当作生产启用；缺少两名不同当前授权客户批准人、exact 五风险/20-user/保障闭包；在四类新鲜现场 readback 与同 terminal CAS 的 `GENESIS_FULL_CERTIFICATION` admission 前开路由；package delta/rollback 无 exact `PACKAGE_DELTA|ROLLBACK_REOPEN` admission；或用第二 activation ID/旧 readback/未绑定 prior failure hash 的 retry 绕过 `FAILED_HELD -> RETRY_COMMITTED`；
- 试图用静态文档、模拟器或测试签名冒充生产证据。

## 9. 最终状态

```text
PRODUCT_DECISIONS_OPEN=0
REQUIREMENT_SET=185
TEST_ID_REGISTRY=276
SIGNER_REGISTRY_ROWS=89
DESIGN_READY=true
IMPLEMENTATION_PLANS_READY=true
development_state=READY_NOT_AUTHORIZED
blocking_reason=DEVELOPMENT_AUTHORIZATION_REQUIRED
implementation_state=NOT_IMPLEMENTED
G0_BOOTSTRAP_GREEN=false
DEV_SLICE_GREEN=false
INTEGRATION_GREEN=false
RELEASE_CERTIFIED=false
PRODUCTION_ACTIVATED_SINGLE_DISK_DEGRADED=false
production_state=PRODUCTION_NOT_READY
```

因此当前准确表述是：**F-57 设计已收敛；`development_state=READY_NOT_AUTHORIZED`、`blocking_reason=DEVELOPMENT_AUTHORIZATION_REQUIRED`、`implementation_state=NOT_IMPLEMENTED`，且 `production_state=PRODUCTION_NOT_READY`。下一次若用户明确授权开发，只执行 G0。未来即使得到 `RELEASE_CERTIFIED`，仍须独立完成双人单盘风险接受、现场重验和唯一 activation CAS，才能声明生产启用。**
