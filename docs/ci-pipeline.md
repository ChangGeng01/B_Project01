# F-57 分层验证与发布证据流水线

> 状态：`CURRENT_SUBJECT` / `APPROVED_TARGET` / `IMPLEMENTATION_NOT_STARTED`
> 日期：2026-08-24（Australia/Melbourne）
> 权威入口：`docs/superpowers/plans/2026-08-24-f57-converged-program.md`
> 目的：以开发反馈速度、真实平台证据和最高安全发布门三者分层，避免普通改动重复运行四端、72 小时和洁净恢复，也避免低档绿灯冒充生产认证。

## 1. 唯一判定实现

调度平台只准备受控执行器并调用 Rust 判定器。唯一命令族为：

```text
cargo xtask f57 graph generate --check
cargo xtask f57 verify --level l0|l1|l2|l3 ...
cargo xtask f57 gate g0|g1|g2|g3|g4|g5|g6 ...
cargo xtask f57 evidence verify ...
```

GitHub Actions、Forgejo/Woodpecker 或其他自建平台不得复制测试选择、Requirement 状态、签名、证据有效期和最终判绿逻辑。更换调度平台只允许替换薄适配器。

`windows-f57-release-precommit` 薄适配器只能调用一个精确入口：

```text
cargo xtask f57 verify --level l1 --profile windows-f57-release-precommit
```

该入口由 `ep-xtask` 内版本化、冻结的 `WindowsF57ReleasePrecommitPlanV1` 解析并执行本文件 §7.1 的完整内部命令转录；YAML、PowerShell 与调度平台不得复制、删减或重排该转录。这样“外部只有一个 Rust 入口”和“内部必须跑完明确命令集合”是同一契约的两层，不是两套入口。

当前仓库尚未交付该命令族；此时请求 F-57 分层证据必须返回 `NOT_DELIVERED`/退出码 70，不能回退使用旧 Linux、25-task 或 11-stage 聚合冒充现行结果。

## 2. 四层证据

| Level | 使用时机 | 必须证明/合并 | 唯一可声明结果 |
|---|---|---|---|
| `L0_DEVELOPER` | 每次本地小改动 | format、lint、CapabilityGraph 生成无 diff、archcheck、受影响 feature 单元/属性测试 | 受影响的纯开发检查通过 |
| `L1_PULL_REQUEST` | 合并前 | L0，加受影响依赖闭包、到期 Fresh PostgreSQL 16、Rust/TypeScript/OpenAPI/UI/权限投影、关键静态安全负例 | 该变更满足合并门 |
| `L2_INTEGRATION_CANDIDATE` | G4、G5 与最终 G6 候选 | 同一 authority 候选树、CTC-01 E2E/故障注入、选定 Workbench 协议；G5 还证明四平台同协议集成，最终 G6 在生产签名制品上执行 first-due G0…G5 的精确 149-ID 集合（JCS 向量 SHA-256 `5ec5a8663b3763bcbf85d84438d6236e9a9680fc0749ad0ca3efa04f124a5a7a`） | `DEV_SLICE_GREEN`、`INTEGRATION_GREEN` 的集成证明，或 final-L2 子集通过 |
| `L3_RELEASE_CERTIFICATION` | 正式发布 | 精确载入 final-L2 的 149-ID 集合和六个同运行辅助 carrier 结果；graph-bound handler 把 carrier 证据评估为正式 Requirement 结果，L3 只执行 first-due G6 的精确 36-ID 集合（JCS 向量 SHA-256 `e7a2fae4fa1a3384c47592ffd0310b3b8717cb595bc5c4397f61e0be2cbc85df`）；两集互斥且并集精确覆盖 185/185，carrier auxiliary IDs 不进入任一 `test_results` 集合 | `RELEASE_CERTIFIED` |

L0/L1 不能声明 PostgreSQL、Windows、客户端、硬件或恢复已通过。G5 L2 可以证明四平台使用同一协议的集成一致性，但不能声明四端生产签名、安装、升级、撤销、P340、备份、最高安全或生产已认证。只有 L3 可以签发 release certificate；release certificate 仍不等于客户生产准入。

证书后的客户部署启用不属于第五个证据等级，也不改变 L3 的含义。独立 `ProductionActivationAuthorityV1` 在已验证 `RELEASE_CERTIFIED` 上完成双人风险接受、认证保障措施与新鲜现场读回，只可持久化到 `LIVE_READBACK_BOUND` 并返回私有 activation-ready proof。随后 upper `ProductionGenerationAdmissionAuthorityV1` 必须通过唯一 `commit_activation_and_genesis_admission_cas`，在同一 PostgreSQL 事务中追加 `ACTIVATED`、创建同一 OBSERVED tuple 的 `GENESIS_FULL_CERTIFICATION` admission、推进 current head 并递增 `business_api_generation`；全部成功或全部回滚后，才可声明 `PRODUCTION_ACTIVATED_SINGLE_DISK_DEGRADED`。不存在可单独 durable 的 terminal activation 路径。

## 3. 需求到期选择

G0 生成的 `docs/generated/f57/requirement-delivery.tsv` 是 185 行到期选择视图。每行恰有：

```text
requirement_id
capability_id
owner_task
activation_task
test_id
test_target_path
test_symbol
evidence_id
evidence_schema
platform_lane
first_due_profile
slice_probe_profiles_json
release_due_profile
```

旧 `F57-01…F57-25` 只表示所有权和证据归集，不表示执行顺序。`first_due_profile` 使用主计划 §4 的基础映射，再应用 57 行逐 Requirement 覆盖表；全部 185 行 `release_due_profile=G6_RELEASE`。11 个延期边界在 L3 到期的是“接口和禁用负例”，不是延期实现。`slice_probe_profiles_json` 只选择较早档位的窄切片测试，不得满足、部分通过或提升父 Requirement。

同一 TestID 被多个 Requirement 引用时，在一个候选树上只运行一次；结果按 Requirement binding 聚合，不按旧 task 重复运行。缺少、空跑、错平台、错 profile、错 tree、错 graph、过期或未签的结果都不满足 due row。

## 4. 候选身份

每份结果至少绑定：

```rust
pub struct CandidateIdentityV1 {
    pub repository_tree_sha256: Sha256Digest,
    pub git_commit: String,
    pub cargo_lock_sha256: Sha256Digest,
    pub capability_graph_sha256: Sha256Digest,
    pub generator_version: String,
    pub migration_manifest_sha256: Sha256Digest,
    pub toolchain_manifest_sha256: Sha256Digest,
    pub artifact_signer_registry_sha256: Sha256Digest,
}
```

`Sha256Digest` is the master plan's single strict digest type: private 32-byte storage with an exact 64-character lowercase hexadecimal JSON string wire. Uppercase, prefix, whitespace, wrong length and JSON number arrays are invalid.

`docs/evidence/f57-foundation.v1.schema.json` 是零 import 的唯一 schema DAG 根，并独占共享 primitive/identifier、principal/delivery、candidate/evidence ref、跨阶段 client nominal 与四字段 detached-CMS envelope field set。上面的 `CandidateIdentityV1` 恰有八个字段，字段增删、改名或在其他 schema 重定义都无效。每个 signed-root schema 必须直接以一个精确相对 `$ref` import foundation，即使它已可经 helper 间接到达；它只组合一次 foundation envelope、只把 `payload` 收窄到本地 strict payload，并以 draft-2020-12 `unevaluatedProperties=false` 封闭。任何 foundation nominal 的复制、第二 owner、仅传递依赖、缺失直接边、foundation 反向边、绝对/网络 `$ref` 或 schema cycle 都在 G0 schema-DAG 与最终 offline-closure golden 中失败。

代际只有一组 exact wire。`docs/evidence/f57-generation.v1.schema.json` 与 `crates/platform/release/src/generation.rs` 是 manifest/reverse-plan/participant-ACK 以及 participant apply/rollback readback 的唯一 schema/Rust nominal owner；schema 直接 import foundation，分别且只为 signed manifest、signed reverse plan 组合一次 envelope，另有 strict plain internal readback 与 ACK roots。manifest/reverse-plan/ACK 字段数固定为 `13/9/14`；plain ACK 的第十三字段是 exact `participant_apply_readback_ref`，第十四字段才是服务器可信时间 `acknowledged_at_unix_ms`，没有 CMS 字段或 detached-envelope wrapper。apply readback 只接受 tagged `DESIRED_ITEM`，其 canonical `applied_items` exact-set 等于 participant 的 `required_item_ids`；每个 package item 必须携带非空 `generation_transition_ref`，`readiness_refs` 必须闭合对应 operation result 与最终 installed-state readback。rollback readback 对真实前代使用 `DESIRED_ITEM`，对首次安装项使用 `DEACTIVATED_RETAIN_DATA`：它不得伪造 predecessor item，必须指向原 forward item/transition 与 `ABSENT` installed-state readback，后者含非空 absence/retained-data proof。rollback target 只能是精确前代的 `PRIOR_OBSERVED_GENERATION`，或仅 generation 1 可用且不携带虚构前代 ref/digest 的 `NO_OBSERVED_GENERATION`；FAILED/UNKNOWN readback 不产生 ACK 或 rollback commit。

`docs/schemas/f57-generation-approval-registry.v1.schema.json` 与 `crates/platform/release/src/generation_approval.rs` 单独拥有 signed approval registry：payload 恰为七字段，rows 恰为 manifest、reverse plan、migration plan 三条五字段 canonical row，无 wildcard。产品固定部署信任先验证 registry；已验证 storage manifest 的 `policy_ids` 又必须以 `generation-approval-registry-sha256:<64-lowerhex>` 固定其完整 envelope，并从 DATA_HDD 的派生路径 `generations/trust/generation-approval-registry.v1.json` exact-load，旧版本、邻接文件、自签根或 ambient Windows trust 均不能替换。G1-01 的 generation-domain adapter 只能向部署唯一 `AuthorityStorageManifestRotationCoordinatorV1` 提交 typed change；自建 CNG/PIV 或已批准企业 signer 必须产生同一四角色 SPKI/DN/CMS contract，以不可导出、不可跨角色的 registry/manifest/reverse-plan/migration keys 完成 registry + storage-pin 的全局串行成对安装/轮换。半安装、并行 package/generation writer 或恢复时选择“最新文件”都使启动关闭。只有该 registry 驱动的 domain verifier 能构造私有 `VerifiedGenerationManifestV1` 并绑定 exact registry ref；generic proof、89-row evidence registry proof 或 raw payload 不能授权 generation consumer。

唯一 generation digest 是完整 canonical signed-envelope bytes 的 SHA-256，并与其 `ArtifactRefV1.sha256` 相等；payload hash、graph hash、generation number 或重序列化 digest 都无效。generation 1 的 previous OBSERVED digest 必须为 null，后续必须逐代精确指向上一个 durable OBSERVED envelope。items、required participants 与 item subsets 从完整 graph 确定性派生且 canonical unique；G1-01 的 frozen creation attempt 必须由 prior OBSERVED + compiled rollback policy 派生每个 signed reverse plan，固定 ID/time/path，逐项 create-new/fsync/reload 后才签 manifest。`reverse_plan_ref` 只允许同 item/source 的 `RESTORE_ARTIFACT`（distinct nonnull target）、`DEACTIVATE_RETAIN_DATA`（null target）或策略显式允许且 target=source 的 `NO_OP`，数据保留恒为 `RETAIN_ALL_GENERATION_DATA`。manifest/reverse plan 使用 exhaustive issued-at-only five-minute current-verification issuance rows，崩溃只能采用同 attempt 原字节。G0 只交付 wire/parser/verifier/golden，不生成生产 registry/key/plan/generation/readback/ACK；G1-01 仅在 trust pair、storage manifest 与 DATA_HDD 验证后生成 reverse plans、签名并 create-new 存储 generation，再构造 declaration；G1-05 独占激活、typed readback 与 ACK 构造。ACK 只能由服务器在 authenticated Service-SID IPC + fresh stored/reloaded readback 后按 declaration row/item subset 重算，participant/client/plugin 提交 ACK、混用 attempt/digest/readback 或缺/多/重复 ACK 均失败。

运行拓扑只允许两阶段，不再存在 `RuntimeTopologyManifestV1`。`docs/evidence/f57-runtime-topology.v1.schema.json` 是 `RuntimeTopologyDeclarationV1|RuntimeTopologyCertificationV1` 的唯一 schema owner，直接且仅一次 import foundation；两者分别使用 exact declaration/certification media，是存入 `EvidenceObjectStoreV1` 的 strict plain JCS，不是 signed envelope，也不增加 signer-registry row。G0 只交付 wire/schema、确定性 builder 与 live verifier，G0 gate 不得生成部署声明；G1-01 只有在签名 storage manifest 与 DATA_HDD 验证通过并产生/验证同一 signed generation 后，才可首次构造并存储 declaration。G1-05 只通过私有 `VerifiedRuntimeTopologyDeclarationV1` 在 exact ref/JCS/fresh live-readback 相等后激活并写 exact ACK set。G6 的最终候选不能沿用早于生产安装的代际：`WINDOWS_AUTHORITY_BUILD` 与 `WINDOWS_SERVICE_INSTALL` terminal PASS 后，`FinalInstalledGenerationAuthorityV1::begin_or_adopt` 必须从当前 OBSERVED predecessor 创建恰好下一代，把 graph-exact runtime deployment closure/readback、最终五个 Authority 服务、固定 G0 evidence-signer broker、exact 六行备份/恢复组件及其实际 artifact/端点/能力写入新 generation 与 declaration，并由现有 G1 coordinator 推进到同一 attempt 的 durable `OBSERVED_COMMITTED`。候选冻结只能绑定该 final-installed manifest/declaration、完整 ACK exact-set 与 `GenerationObservedReleaseSelectionRecordV1`。G6 随后必须等同一候选的 P340 terminal PASS 后才构造 certification，发布证书再绑定它；生产启用还要重新 exact-match 证书、certification、final-installed OBSERVED selection、runtime deployment、组件与现场 topology/readback。唯一发布顺序是 `verified storage -> initial generation/declaration/OBSERVED -> production build -> production install/readback -> final-installed next generation/declaration/OBSERVED_COMMITTED -> observed selection -> candidate freeze -> terminal P340 -> topology certification -> release certificate`；其后才可能独立执行 `production activation`。预安装/desired-only generation、旧单体类型、提前认证、声明中混入 candidate/P340/capacity 字段、signed-wrapper 替代、ref/profile/host/capacity 漂移或 declaration-only 生产启用均失败关闭。

Windows 产品运行单元不能用固定进程数表达。`WindowsRuntimeDeploymentClosureV1` 必须从同一 compiled CapabilityGraph 为每个 participant 确定性生成恰好一行 `ACTIVE` 或 `DEFERRED_DISABLED`；`WindowsRuntimeDeploymentReadbackSetV1` 再为每个 ACTIVE 行给出一个 carrier-specific 正读回、为每个 deferred 行给出一个五项计数全零的 `DEFERRED_ABSENT`。五种载体家族严格分型：`WINDOWS_SERVICE` 对应 `WINDOWS_SERVICE_PE/WINDOWS_SERVICE_RUNNING`，`JOB_OBJECT_WORKER` 对应 `JOB_OBJECT_WORKER_PE/JOB_OBJECT_WORKER_RUNNING`，`IN_PROCESS` 对应 `IN_PROCESS_HOST/IN_PROCESS_READY`，WASM 对应 wire `WASM_SANDBOX` 与 `WASM_COMPONENT/WASM_READY`，Hyper-V 对应 wire `HYPER_V_CONTAINER` 与 `HYPER_V_CONTAINER/HYPER_V_CONTAINER_READY`。host/supervisor 依赖必须存在且无环，artifact/config/policy/session/readiness 全字段 exact-match。精确集合关系分层验证：全体 graph participant IDs ↔ closure participant IDs ↔ readback participant IDs 一一对应；仅 ACTIVE participant IDs ↔ positive-readback participant IDs ↔ topology declaration participant IDs ↔ generation required-participant IDs ↔ ACK participant IDs 一一对应。`database_consumers` 是 active service identities 的独立 graph-exact 投影，active participant 可有零或多行；generation `items` 与每个 participant 的 `required_item_ids` 是另一独立 canonical exact 投影，允许多对多，但每个 item 至少被引用一次且不得引用集合外 ID。`DEFERRED_DISABLED` 只有 absence readback，且必须从 artifact、declaration participant、database consumer、generation required participant、participant-item edge 与 ACK 集中排除。固定九进程、目录扫描、installer side list、ACTIVE 未安装、deferred 实际存在、orphan/out-of-set item 或跨 carrier 替代均禁止。首版 local AI 固定为 `DEFERRED_DISABLED{reason=LOCAL_AI_IMPLEMENTATION_DEFERRED}`，只允许显式 `NullAiProviderV1` 行为；不得出现 `ai-inferer` 进程、端点、模型包或资源预留。

每个 signed `CapabilityPackagePayloadV1` 必须是 exact 十三字段：除 graph-owned closed `component_class` 与 tagged `hotplug_contract` 外，必须以 `implementation_manifest_ref` 指向 exact 八字段 `CapabilityPackageImplementationManifestV1`。实现清单按 tagged `DECLARATIVE_BUNDLE|WASM_MODULE|WINDOWS_NATIVE_BINARY|HYPER_V_CONTAINER_IMAGE|DATABASE_MIGRATION_BUNDLE|FOUNDATION_ARTIFACT_SET` 封闭，并把 package ID/version/class、全部实现 artifact、SBOM 与 `implementation_set_sha256` 一次性闭合；任何入口点、WIT/schema、Authenticode readback、OCI signature、migration 或 foundation artifact 不在清单内都不可执行。ADR-0023 `PermissionCeilingV1` 只能从其唯一 provider-permission owner 导入。Compiler 以 graph slot 的 `component_class+scope_mode` 强制最低 grade/executor/rollback 表，package 不得自分类、降档或把 deployment-global class 缩成法人局部范围。

唯一 package schema 同时校验 signed package、signed exact 30-field maintenance plan、strict plain nine-field desired-state `CapabilityPackageGenerationItemV1`、sixteen-field typed dual-control decision、reservation/authorization scope、execution-trust snapshot、execution authorization、per-attempt generation transition、operation request/result 与 installed-state readback。30-field plan 只绑定 `recovery_checkpoint_policy_ref`，绝不携带尚未产生的实际 checkpoint；nine-field desired item 只表达 reusable desired state 与 tagged scope，不携带 plan、decision、checkpoint 或一次执行的 authority。每次 participant/item apply 必须另建 `CapabilityPackageGenerationTransitionV1`，把 generation/attempt/scope/source/target、execution action和reverse plan固定下来；mutation 还必须绑定 common execution-trust snapshot，maintenance 再额外绑定 reservation/plan/execution-authorization/checkpoint/hold refs，`VERIFY_UNCHANGED` 则保持这些执行引用全 null。transition 再由 apply readback 和十四字段 ACK 认证。Package/plan/registry 三个 signed root 各有 generated stable CMS type descriptor，package/maintenance signer role 分离且不进入 89-row evidence registry；plain operational roots绝不套 signed envelope。

CI exact-check universal `VERIFY_UNCHANGED`（source/target item、package/version/state/scope byte-equal，所有 maintenance ref 与 operation ID 为 null，只需 fresh installed-state readback），以及 `INSTALL ABSENT→INSTALLED_DISABLED`、`ENABLE INSTALLED_DISABLED→ENABLED`、`DISABLE ENABLED→INSTALLED_DISABLED`、`UPGRADE same-state→same-state higher` 和显式 `ROLLBACK same-state→same-state lower`；系统不存在 uninstall/delete action。普通十一类可执行五种 mutation；`RUST_KERNEL|POSTGRESQL_DATABASE_MIGRATION|CRYPTOGRAPHY_FOUNDATION|STORAGE_FOUNDATION` 在生产仅允许 `UPGRADE|ROLLBACK`，初装只能来自签名 release/recovery bootstrap，且必须用 concrete `DEPLOYMENT` scope 和完整 impacted-legal-entity exact-set。source/target probes、checked window cap、retained-data、disabled closed-admission、enabled-after-production-admission 与 ABSENT nullability/absence proof 都逐字段负测。

每个 mutation 执行以 `(activation_attempt_id,participant_id,item_id)` 为唯一 SQL PK；多 participant 引用同一 item 时各自拥有独立 transition、execution-trust snapshot、operation request/result、installed-state readback 与 ACK，不能借用另一 participant 的证据。所有 grade 在第一外部 intent 的同一 CAS 前冻结 common `CapabilityPackageExecutionTrustSnapshotV1`；`VERIFY_UNCHANGED` 只做当次 current verification 而不冻结外部 intent。每个外部调用必须先 create-new 持久化 `CapabilityPackageOperationRequestV1`，其 binding 精确包含 generation/attempt/deployment/participant/item/scope/epoch/action、globally unique operation ID、transition、trust snapshot、可空 execution authorization，以及 forward 或 rollback 所需的 implementation/checkpoint/kernel-pointer/reverse binding；executor 只实现 `begin_or_adopt|query_exact`，response loss 只能查询/采用同一 `(operation_id,binding_sha256)`，任何字段漂移为冲突。Forward failure/UNKNOWN 只能返回上层；coordinator先 durable `ROLLBACK_STARTED{rollback_execution_attempt_id}` 并派发 private rollback request，随后 control broker 或 recovery tool 才能按 graph-derived strategy补偿。Package-local `APPLIED_VERIFIED` 绝不等于 generation OBSERVED 或 production admitted。

Maintenance 使用严格两阶段授权：upper authority 先由 compiled graph、current tenancy snapshot、current OBSERVED 与 fresh source readback产生 reservation和历史 structural plan；随后持久化 `HOLD_INTENT -> ADMISSION_CLOSED -> DRAIN_COMPLETE -> BARRIER_COMMITTED`，在同一 `write_barrier_id` 下由 Task 11 冻结覆盖全部启用 DATA_HDD authority class/root 的 `AuthorityRecoveryCutManifestV1`，并创建 exact-binding 同一 reservation、checkpoint policy、barrier 与 full cut 的 actual BACKUP checkpoint，再以 fresh decisions、current trust snapshot、source readback、hold/checkpoint/full cut 创建 execution authorization；只有 `CHECKPOINT_BOUND` 后才可 transition并提交第一 privileged forward intent。plan authoring journal覆盖 freeze/decision/provider/spool/object/bind crash cut，但属于运行恢复日志，不进入 release offline bundle。过期/撤权发生在 intent 后只能 query/adopt、measure 或 rollback，不能重授权或新发 forward；UNKNOWN/timeout保持 admission closed，全部数据、附件、审计、恢复状态和旧版本 pin 保留。

Storage manifest 是所有 trust-domain 轮换的唯一全局序列化权威：generation/package adapter 都只能向部署唯一 `AuthorityStorageManifestRotationCoordinatorV1` 提交 tagged domain change，由同一 maintenance lock、monotonic manifest CAS 与 fsynced hash-chain journal原子更新 registry digest/SPKI pin 和 fixed path；package 不得拥有第二 manifest writer。每个 desired item携带已 materialize 的 portable package-registry ref，离线和恢复都禁止目录扫描、猜测当前版本或按“最新文件”选取。

`migration_manifest_sha256` 精确等于 canonical `MigrationClosureIdentityV1` 的 JCS SHA-256；该结构只含 `schema_version=1`、baseline registry digest、固定 69-file baseline apply-manifest digest、当前 47-row F57 reservation-manifest digest 与 310-row legacy-disposition seed digest。319 是 catalog 中 `PLANNED` 的总行数（9 个 baseline-absent + 310 个 legacy），绝不能代替 legacy seed 的 310 行身份。baseline apply manifest 永不包含 F57 row；各 task 只原子更新自己 reservation 的 `CREATED` 状态。`artifact_signer_registry_sha256` 绑定 G0 生成、企业根引导验证且无 wildcard 的 exact 89-row 签名主体登记；每个候选和 gate receipt 还指向 bundle 内同一签名登记字节。聚合时任何字段不一致即 `CANDIDATE_IDENTITY_MISMATCH`。禁止把不同提交、不同 CapabilityGraph、不同客户端协议、不同 baseline/F57 迁移闭包、签名登记或工具链的绿灯拼成一个候选。

阶段 receipt 只证明其绑定候选，不是可永久复用的批准。G0…G3 使用 typed `PRE_INTEGRATION{candidate_identity_sha256}`，G4/G5 使用 typed `SIGNED_CANDIDATE{candidate_manifest_ref,candidate_identity_sha256}`；禁止 nullable manifest 字段。Standalone G4/G5 必须 typed-load `SignedIntegrationCandidateV1`，最终 G6 内新生成的 G4/G5 receipts 必须 typed-load 当前 `ReleaseCandidateV1`，不得指回旧 G5 manifest。同一 aggregate 内全部 receipt 精确匹配一个 `CandidateRunIdentityV1={candidate_identity_sha256,gate_run_id}`，但每个 gate 保留自己的合法 binding variant；G0…G3 不会被伪装成 signed binding。唯一执行键始终是 `(candidate_identity_sha256,gate_run_id,TestID)`，最终候选 manifest ref 只是额外来源约束，不能改变执行键。

每个候选运行使用一个显式 `--run-journal`。它是签名、哈希链、追加并 fsync 的宕机恢复权威，并由 journal-wide OS lease 单写。276 个封闭 TestID 精确分解为 185 个 Requirement、78 个 slice probe、3 个 client conformance auxiliary、4 个 client validation auxiliary 和 6 个 release-carrier auxiliary，并共用唯一执行键；真实执行前必须先持久化 `TEST_STARTED{execution_attempt_id,start_context_refs}`，正常完成写一个 `TEST_COMPLETED`，宕机恢复只能经 `TEST_UNKNOWN -> TEST_RECONCILED` 绑定同一次物理结果。不得第二次开始、第二结果、重签旧结果或择优重跑。CandidateBound Fresh-PG 使用单独 operation 协议；候选、client aggregates、offline schema manifest、L2/L3、receipt、certificate 只在全部输入 terminal 后走 deterministic finalization/adopt 协议。L2/client-build/carrier 已产生的 terminal `TestResultRefV1` 由 gate 精确合并，gate 只启动仍为 `ABSENT` 的 TestID。

持久化位置由类型唯一决定：276 项结果进入 `TestResultStoreV1`，14 类普通聚合进入 `EvidenceEnvelopeStoreV1`，三类候选进入 `CandidateManifestStoreV1`，backup checkpoint/full recovery cut、架构全局 slot/attempt、离线 schema closure 各有独立 store；package activation attempt 与 maintenance reservation 是各自 exact SQL CAS projection，production activation/admission/hold 是 upper authority 的独立 CAS projection。其余嵌套 package/log/readback/timeseries/manifest 才进入 `objects/sha256/<digest>`。外部签名输入只在验证后进入 `inputs/<digest>.json`。所有 object/file store 都是 create-new、exact-byte adopt/conflict fail；调用方文件名、目录扫描、覆盖或 hash 搜索均禁止。候选、offline manifest、L2、L3、receipt、certificate 绑定严格延伸的签名 checkpoint；aggregate 有效期不得晚于任一已消费 time-bearing typed evidence 输入、offline manifest 或 journal run 的最早有效期。不可变候选与静态 content-addressed ref 没有自身有效期，只做 exact 校验。只有 schema 明确允许、被测制品/硬件未变且重新验证仍通过的外部证据，才可按候选规则引用。

客户端技术选型使用一个全局 architecture slot/attempt 和永久归档。`DECISION_BOUND` 前必须先落盘决策、32-byte/64-lowerhex nonce 对应的 RFC-3161 token、DecisionSigner/TSA 两条链、每个非根证书的 CRL+OCSP 及 trust-closure；BOUND 后才可确定性生成 manifest。归档内部统一使用 `architecture-archive-relative://root/...`，固定 chains/revocation/trust-closure 路径，不依赖原始 Windows 目录或网络。每次 `validate-selected` 把已提交完整归档 create-new 复制到 `inputs/client-stack-decision-archive/<manifest-sha256>/`，payload 同时绑定 copied manifest ref 与其 decision ref；offline walker 从 validation→manifest→decision/trust/proofs/entries 精确遍历且禁止扫描/额外文件。历史 replay 只按归档可信时间授权永久 decision，subordinate 仍做 schema/digest/CMS 数学与当时窗口校验但不要求其 90 天期限或证书今天仍有效；当前四端包始终重新验证、重新签发 90 天 evidence。

G4/G5 使用 `SignedIntegrationCandidateV1`，G6 使用 signed `ReleaseCandidateV1`。最终候选的 `windows-authority` 直接引用已签 `WindowsAuthorityArtifactSetV1`；该 set 又必须 exact-bind graph-derived `WindowsRuntimeDeploymentClosureV1`、六行 `WindowsServerComponentSetV1`、固定 G0 evidence-signer broker readback 与全部 carrier-specific artifacts。四个 client lane 引用验证 runner 当次产生的 mode/platform `SignedArtifactRefV1`；候选聚合器不重签任何包或权威制品。最终候选还必须携带 `generation_manifest_ref`、`generation_approval_registry_ref`、canonical `generation_participant_ack_refs` exact-set 与 `generation_observed_selection_ref`：它们离线证明 declaration 指向的 whole-envelope digest、独立 approval trust provenance、同一 durable OBSERVED activation attempt 的完整服务器派生 ACK 集，以及候选冻结期间被租约固定且只可绑定一次的当前 OBSERVED selection。该 selection 必须 byte-equal `FinalInstalledGenerationAuthorityV1` 为刚安装 graph-exact deployment 返回的 attempt/manifest/declaration/ACK refs；G4/G5 IntegrationCandidate 不冒充该最终生产闭包。只有 `verify --level l2|l3 --candidate` 的值是显式 `--candidate-manifest` 文件的 exact-byte SHA-256，而不是 `CandidateIdentityV1` 的 hash；`client-gate|client-build|candidate build|candidate freeze` 的 `--candidate` 保持 `<git-rev>`，已签 carrier 输入使用独立 `--candidate-manifest`。Identity hash 作为证据中的独立字段重算并校验。L2/L3 还必须显式接收 manifest、`--bundle-root`、run-journal 和输出路径，所有路径均须解析在该 root 下；禁止靠目录扫描、默认文件名或从 digest 反推 manifest。L2 与 L3 输出分别是 `SignedBusinessArtifactV1<L2CandidateEvidencePayloadV1>` 和 `SignedBusinessArtifactV1<L3CandidateEvidencePayloadV1>`，任何 gate 只接受通过 typed CMS 校验的完整 envelope；L3 还必须通过必填绝对路径 `--l2-evidence` typed-load 最终 L2，并用 `final_l2_evidence_ref` 精确绑定同一 envelope。候选 manifest、observed selection、代际/ACK/runtime-deployment/组件闭包、离线 schema manifest、L2/L3 evidence、checkpoint 和 aggregate receipt 必须 exact-match 同一运行。

## 5. 结果语义与退出码

| 结果 | 退出码 | 可满足 due row |
|---|---:|---|
| `PASS` | 0 | 是 |
| `FAIL` | 1 | 否 |
| 参数或命令错误 | 2 | 否 |
| `NOT_COVERED` / 当前环境无法判定 | 3 | 否 |
| `NOT_DELIVERED` / 工具或能力未交付 | 70 | 否 |

不存在 `SKIP_AS_PASS`、`ALLOW_FAILURE` 或空测试绿灯。超时、runner 失联、证据上传失败和外部 carrier 状态未知均为非零。

## 6. L0 Developer

标准命令：

```bash
cargo xtask f57 verify --level l0 --changed-from HEAD^
```

选择器读取 Git change set、Cargo dependency graph、feature owner、迁移 catalog 和 CapabilityGraph projection dependency。至少执行：

1. `cargo fmt --all -- --check`；
2. 工作区适用 `clippy -D warnings`；
3. `cargo xtask archcheck`；
4. `cargo xtask f57 graph generate --check`；
5. 受影响 feature/platform 的单元和属性测试；
6. 新错误、事件、指标、配置和 schema 引用的登记一致性。

若 change set 无法解析、变更跨越未登记边界或测试选择为空，返回 3 或 1，不能假定无影响。

## 7. L1 Pull Request

标准命令：

```bash
cargo xtask f57 verify --level l1 --changed-from HEAD^
```

L1 在 L0 之上执行：

- 所有受影响 feature 的公开 contract test 及依赖者；
- 到期的 PostgreSQL 16 clean database、upgrade、rollback/checkpoint 和 RLS 负例；
- CapabilityGraph 到 exact 30 个非自引用 projection family（恰四个 multi-member；覆盖 Rust、TypeScript、OpenAPI、UI、权限、MCP/Excel、package/provider、Test manifest、P340 policy 与 semantic contracts）的相同 digest；
- generation manifest/reverse-plan/ACK 的 `13/9/14` 字段、apply/rollback readback strict roots、tagged `DESIRED_ITEM|DEACTIVATED_RETAIN_DATA`、generation 1 `NO_OBSERVED_GENERATION`、plain ACK 的 exact apply-readback ref 与 trusted acknowledgement time、approval-registry payload 七字段/固定三 row、四角色自建/既有 signer 等价与 key nonexportability/ACL、全局 storage-manifest rotation pair-install crash recovery、issued-at-only issuance descriptors、frozen reverse-plan/manifest creation attempt、storage-policy digest pin、whole-envelope digest、previous-OBSERVED 连续性、reverse-action/retention 约束、graph-derived participant/item exact-set、独立 approval trust domain 与 generation→declaration 顺序的 schema/byte/negative goldens；
- signed capability-package payload 十三字段、implementation manifest 八字段与全部 tagged artifact closure、strict pure desired item 九字段、plan 三十字段但仅 checkpoint policy、decision 十六字段、scope/transition/trust/authorization/operation/readback media，三等级与 graph component-class/scope 最低档映射、普通/四类 global action table、per-participant PK/operation tuple、两阶段 hold→barrier→full recovery cut→execution authorization、prepare/drain/switch/probe/rollback 全 crash-cut、UNKNOWN/timeout 不推进、数据保留、旧版本 pin、global storage-manifest rotation 与 production-admission gate 的 schema/state/CAS/negative goldens；
- 客户端生命周期 fixture corpus 的 exact 四个公开 DER 测试根、16 个固定原生包、strict corpus manifest 与 CapabilityGraph 两个 typed source vector 全字段一致；四个平台分别在全新隔离信任库中只装入对应固定根并验证 native metadata/签名链，且 secret scan 证明仓库无 fixture 私钥、密码或 provisioning secret；
- SQL 注入、跨法人、越字段、越 query-use、绕过 `AuthorizedPgTx`、网关持 DB/KMS、动态 SQL和第二 writer 等静态/集成负例；
- 离线依赖锁、SBOM/许可证/secret scan 的到期子集；
- Windows-target 代码的 MSVC build/test，Apple/Android 文件受影响时对应受控 runner 的 build/contract test。

PR 调度适配器固定为 PowerShell 或对应平台的受控脚本，只调用上述命令并回传退出码。它不得根据文件名自行删减 due selection。

### 7.1 强制 Windows Server 2022 / MSVC 预提交门

`.github/workflows/ci.yml` 中的 `windows-f57-release-precommit` 是 Task 14 与最终发布路径的强制 job，必须运行在已批准的 Windows Server 2022 x64 self-hosted runner 和锁定的 MSVC 工具链上。Linux/macOS 编译、交叉编译、`cfg(windows)` 排除、手工豁免或允许失败都不能满足该门。YAML 只可执行以下唯一外部入口并原样回传退出码：

```powershell
cargo xtask f57 verify --level l1 --profile windows-f57-release-precommit
```

`ep-xtask` 内的 `WindowsF57ReleasePrecommitPlanV1` 必须确定性解析并执行以下完整、可审计的内部子进程转录；下列命令是 Rust 计划的实现契约，不得复制到 YAML 形成第二套选择逻辑：

```powershell
cargo xtask f57 graph generate --check
cargo run -p authority-kernel-abi-gen --locked -- --check
cargo test -p ep-platform-powershell-trust -p powershell-trust-tool --all-targets --locked
cargo test -p ep-platform-release -p ep-platform-runtime -p ep-platform-package -p ep-platform-backup -p ep-platform-generation-activation -p ep-platform-tenancy -p ep-platform-ups-contract -p ep-adapter-ups-windows -p ep-authority-kernel -p ep-adapter-file -p ep-adapter-db-pg -p core-server -p recovery-tool -p ep-xtask -p ep-testkit -p ep-release-gate --all-targets --locked
cargo build -p core-server -p ep-authority-kernel -p ep-adapter-ups-windows -p recovery-tool --release --locked
cargo test -p core-server --test windows_service_process_dispatch --test authority_kernel_loader_composition --test authority_kernel_abi_binding --locked -- --nocapture
cargo test -p ep-authority-kernel --test abi_compatibility --test abi_export_and_layout --test windows_service_dynamic_readback --test package_maintenance_composition --test final_installed_generation_composition --test production_activation_composition --test production_admission_gate_composition --test power_shutdown_continuation_composition --locked -- --nocapture
cargo test -p ep-platform-backup -p ep-adapter-backup -p evidence-signing-broker -p backup-writer -p backup-checkpoint-signer -p data-volume-unlock-broker -p backup-target -p recovery-tool -p pg-passphrase-helper --all-targets --locked -- --nocapture
cargo test -p ep-xtask --test f57_release_carrier --test f57_windows_runtime_deployment --locked -- --nocapture
cargo test -p ep-testkit --test f57_final_candidate --test f57_final_installed_generation --test f57_package_maintenance_production --test f57_production_activation --test f57_production_generation_admission --test f57_production_admission_execution_lease --test f57_production_admission_bypass --test f57_production_admission_races --test f57_windows_runtime_deployment --test f57_windows_recovery_security --test f57_postgres16_recovery --test f57_postgres16_windows_install --test f57_backup_storage_safeguard --test f57_backup_topology_signing_trust --test f57_backup_checkpoint_transition --test f57_release_gate_unit --test f57_release_dependency_dag --test f57_ups_adapter_contract --test f57_ups_command_reconciliation --locked -- --nocapture
```

ABI 检查只能调用 master 定义的唯一命令 `cargo run -p authority-kernel-abi-gen --locked -- --check`；它必须对 `apps/core-server/src/kernel/abi.rs`、`include/ep_authority_kernel_api_v1.h` 与 `crates/platform/authority-kernel/ep-authority-kernel.def` 做零 diff 验证。随后在锁定 MSVC x64 下重读 PE export table：只能有一个 named、non-forwarded export `ep_authority_kernel_get_api_v1`，不能有第二个 named export 或 unnamed ordinal-only export；ABI version/size/offset 必须为 `1/48/[0,4,8,16,24,32,40]`，C round-trip、section protection、held-file identity 与 generated import allowlist 都必须通过。手改任一生成文件、generator 漂移、额外/forwarded export 或只编译未检查 PE 都失败。

该 job 必须在真实 Windows 进程中验证精确五个 Authority launcher-role 服务向量：ordinary Authority、dormant continuation、control broker、raw signer facade 与 journal signer facade；同时覆盖 `CreateProcessW`、`StartServiceCtrlDispatcherW`、Service SID/token readback、Authority 角色端点 nonce challenge、无 activation 的 continuation 零副作用退出，以及少/多/未知服务向量的负例。这里的五个服务不是整机清单：EnterprisePlatform-owned 固定清单精确为九个 SCM 服务，即这五个 Authority 服务、`EPF57EvidenceSignerBroker` 与 backup writer/checkpoint signer/data-volume-unlock broker 三个 component service；完整生产主机另必须含固定 `ep-postgres16` 与 runtime-deployment closure 中不别名的 `ACTIVE/WINDOWS_SERVICE` 行，所以基数是 `10 + active_additional_windows_service_count`。系统/第三方服务不计入该等式，未知 `EP*|EnterprisePlatform`-owned 行必须失败。

G0 `EPF57EvidenceSignerBroker` 必须以独立 service SID、固定 ImagePath/argv、AUTO_START、依赖、`SeChangeNotifyPrivilege`、最终句柄映像与 authenticated readiness 逐字段读回。其 `F57EvidenceSignerV1` pipe 由 broker SID 创建并持有第一实例；SYSTEM/broker 有 server-instance 权，`EPF57EvidenceSignerClients` 仅有 concrete `0x00120183` client data rights。客户端必须按该 mask 打开，不能使用 `GENERIC_WRITE`，且 AS/facade/client group 创建第一、第二或 replacement pipe instance 的负例必须失败。active broker session 的客户端组、group DACL、raw/journal key ownership、DATA_HDD state root 与零 mutable-SSD fallback 都必须 exact-match；两 facade 仍无 key ACE。

备份/恢复组件集精确为六行：`BACKUP_WRITER_SERVICE|BACKUP_CHECKPOINT_SIGNER_SERVICE|DATA_VOLUME_UNLOCK_BROKER_SERVICE|RECOVERY_TOOL|PG_PASSPHRASE_HELPER|BACKUP_TARGET_AGENT`。前五行 on-host，前三行是不同安全身份的 AUTO_START SCM service，recovery tool 是唯一 Scheduled Task，passphrase helper 是唯一按需 executable，target agent 只能 off-host。三个 service runtime contract 各以 exact `component_id` 为挑战域；复用 Authority `service_role`、跨组件响应、共享 writer/signer/unlock 身份或把 target 装到 P340 都失败。

`RECOVERY_TOOL` 门必须从静态 policy 到 live token 完整闭合：专用 `EPF57Recovery` local account、`DEDICATED_LOCAL_S4U`、`S4U`、`LEAST_PRIVILEGE`，account rights 恰为 `[SeBackupPrivilege,SeBatchLogonRight,SeChangeNotifyPrivilege,SeManageVolumePrivilege,SeRestorePrivilege]`，Task `<RequiredPrivileges>` 恰为去掉 `SeBatchLogonRight` 后的四项；direct group 只有 Users，admin/operator prohibited-group intersection 为空，account flags 恰为 `0x00010240`，Task Scheduler 无 stored password 且 installer plaintext residue 为零。真实无副作用 self-test 必须证明 user SID、MEDIUM/DEFAULT、非 AppContainer/非 restricted token、四项 privilege exact-set，以及 WS2022 S4U fixture 的 canonical group/attribute set，要求 `SERVICE_ASSERTED_IDENTITY=S-1-18-2` 且不能用 `S-1-18-1` 冒充凭据认证。任务必须仍是 `\EnterprisePlatform\F57\RecoveryToolV1`、零 trigger、单一固定 Exec、`--scheduled-task-server`、固定 folder/task/executable DACL 与固定 pipe；AS 的 pipe ACE 只能是 `0x00120183`，不能创建第二实例。请求只允许 `START|QUERY|ADOPT` 与 master 的 exact 六 operation，不能携带 argv/path/task/service/SQL/shell。

DATA_HDD 解锁必须在独立 `EPF57DataVolumeUnlockBroker` 中完成，不能借用 recovery task。门禁要求 LocalSystem primary token + `RESTRICTED` service SID、`SeChangeNotifyPrivilege|SeManageVolumePrivilege`、outbound network false、AS-only typed client、RT 只能 query SCM 且不能 start/stop，pipe client仍为 `0x00120183` 且无第二实例。BitLocker DATA_HDD protector exact-set 是 `{PUBLIC_KEY,RECOVERY_PASSWORD}`，fixed-data auto-unlock 为 false。九行 pre-HDD locator set、每行 final-handle/DACL/media/digest/size、零 unresolved/reparse/ADS/hard-link escape必须读回；registry/authority/public-object limits 为 `1048576/1048576/16777216`，每个 trust bundle/revocation 最大 `4194304`、checkpoint 最大 `65536`。WMI 只能连本机 `ROOT\CIMV2\Security\MicrosoftVolumeEncryption`，proxy 为 packet privacy + impersonate、无 delegation/调用方凭据；namespace 只 merge broker SID 的 `WBEM_ENABLE|WBEM_METHOD_EXECUTE=0x00000003`，保留 OS/provider ACE 并拒绝 `0x000000FC` 其他权，唯一 method 为 explicit-thumbprint、empty-PIN 的 `UnlockWithCertificateThumbprint`。真实 WS2022 restricted-token fixture 必须返回 0 并 exact-match protector type 7/certificate/provider/volume。ordinary reboot 重开既有 Microsoft Platform Crypto Provider key；clean SSD 不得声称由 DER/SPKI/TPM handle 重建旧私钥，而必须在 admission closed 下用 off-host 48-digit recovery password 的双人 ceremony 解锁、生成新的 TPM-backed nonexportable key/CA certificate、加验新 PUBLIC_KEY protector、提升 signed authority epoch 与 TPM NV head、正常重启验证 broker unlock，最后移除旧 protector；八步 hash chain、零 recovery-secret leak 与未提前开 admission 都是硬门。

`package_maintenance_composition` 覆盖 reservation、two-phase hold/barrier/checkpoint/full cut、execution authorization、per-participant operation/query-adopt/rollback；`final_installed_generation_composition` 覆盖 final-installed OBSERVED 和 14-field ACK/readback closure。production admission 测试不能只验证一个 Boolean route gate：generated router 与 bypass registry 必须从同一 compiled source 产出并 exact-join master 的十行 method/route/selector/authorization 集，无 prefix/wildcard/default；`WindowsAuthorityArtifactSetV1.production_admission_bypass_registry_ref` 必须签名绑定 exact bytes/media，service-install evidence 必须 exact-repeat该 ref并证明 installed bytes，final candidate 只能经其 signed Authority artifact set 到达同一 ref，startup 也只 typed-load该 candidate-bound installed ref。每个业务 command/query 必须在 admission/hold writer 共用的 deployment-scoped serializable lock/CAS 内先 exact-check服务器派生 current authority epoch、OBSERVED/admission/API generation 与全部 target-scope hold，要求 epoch exact-equal typed admission、OBSERVED generation 和 verified security context，并 create-new 或 exact-adopt exact twenty-field `ProductionAdmissionExecutionLeaseV1{ACCEPTED}` 后才返回 affine permit；epoch 进入 full binding digest，客户端不可提供。terminal 只能是相同 epoch 的 `COMPLETED_IN_PLACE|HANDED_OFF_EXACT_ONCE`；hold 同锁写 `ADMISSION_CLOSED`，只有跨全部 epoch 的 intersecting ACCEPTED count 在同事务为零才可写 `DRAIN_COMPLETE|BARRIER_COMMITTED`。唯一约束保持 `(deployment_id,request_id)`，跨 epoch 重用同一 request ID 不得产生第二次 effect。必须覆盖 permit-before-hold、hold-before-permit、response loss、process crash、terminal-CAS loss、multi-scope intersection、request-ID changed-binding、cross-epoch replay、old-epoch orphan 与 terminal epoch drift，证明 barrier 后无业务写。bypass permit 只能到 control/health handler，永远不能构造 `AuthorizedPgTx`。

Runtime SSD 门按两个不相交集合执行：Set A 是完整 signed `RuntimeSsdReproducibleRuntimeInventoryV1`（catalog-verified Windows、immutable product bytes、bounded reconstructible OS cache与 TPM-bound reenrollable key metadata）；Set B 恰为四个 mutable class、十九 media contract、二十 path rows，分别是 POWER capsule、package-recovery capsule、kernel pointer/head、reconstructible signed native-code slot/cache。全卷 entry/ADS/hard-link/VSS/locator scan 必须映射到 A 或 B，rejected/unclassified/inaccessible/partial 与 customer/business digest/canary count 全为零。post-reboot persistent-file policy 恰为八行：page/swap/hibernation/kernel-or-full-dump/mini-dump/WER local dump 六行 DISABLED，VSS diff area 与 product malware quarantine 两行只能在 verified DATA_HDD。telemetry policy 恰为七行：product Event Log/ETL、Task Scheduler Operational、Defender Operational/history 为 SSD bounded no-customer schema，Windows Firewall text log DISABLED，HTTP.sys error log 与 Authority access audit 在 DATA_HDD；零 unregistered channel/session、零 canary/digest hit。Set A 不是 scanner exclusion，Set B 不能出现第五类。

`f57_windows_runtime_deployment` 还必须覆盖五种 carrier/artifact/readback 矩阵、host/supervisor 无环、graph exact-set/bijection、deferred absence 和首版 local-AI `NullAiProviderV1`；固定九进程 fixture 不得成为 graph 判定来源。该 job 只证明代码可在 Windows Server 2022/MSVC 上构建并组合，不冒充 Task-15 的 clean-HEAD 生产签名、MSI 安装、restricted-token/WMI/BitLocker 现场资格或动态 carrier/P340 证据；真实现场门中的任一项未交付只能 `NOT_DELIVERED`，不能由 mock PASS。

## 8. L2 Integration Candidate

标准命令：

```powershell
cargo xtask f57 verify --level l2 --candidate <candidate-manifest-sha256> --candidate-manifest <path> --bundle-root <path> --run-journal <path> --out <path>
```

L2 只在受控 Windows Server 2022 集成执行器上生成 authority verdict，并绑定：

- 原生 Windows 服务候选或 G4 允许的开发宿主；
- PostgreSQL 16 Fresh database；
- HDD 测试数据根与附件 quarantine；
- 最小 Control Center 与 Windows Workbench；
- CTC-01 exact 主链；
- 重复点击、CAS 并发、执行中撤权、SoD、进程重启、外部成功但响应丢失、Unknown 对账、HDD 黄线、备份负载重叠、数据库/附件一致恢复；
- package hotplug 的 graph-grade/scope/action enforcement、13-field package + 8-field implementation closure、pure desired item + per-attempt transition、per-participant trust/operation/readback 隔离、prepare/drain/switch/probe/rollback 故障注入、two-phase admission hold/barrier/full recovery cut/execution authorization、response-loss query/adopt、UNKNOWN 不推进、global storage-manifest rotation 串行化、production admission mismatch 全路由关闭、旧版本 pin 与数据/审计/恢复保留；

L2 的目的/目标/业务状态是封闭关系：

| L2 purpose | 目标 | 必须证明的 Objective 状态 |
|---|---|---|
| `DEVELOPMENT_SLICE` | `DEV_SLICE_GREEN` | `CONTRACT_FULFILMENT`、`SALES_ORDER_FULFILMENT`、`RECEIVABLE_COLLECTION` 为 `CLOSED`；`PROCUREMENT_FULFILMENT=WAITING`，缺失义务精确为 PurchaseInvoice、AP、SupplierPayment，证明没有假关闭 |
| `INTEGRATION` | `INTEGRATION_GREEN` | 四个 Objective 均为 `CLOSED`，采购必须经过独立 closure review，并证明选定栈的四平台同协议集成一致性 |
| `FINAL_RELEASE` | `RELEASE_CERTIFIED` | 在最终生产签名制品上重验四个 Objective 均为 `CLOSED`；其 `test_results` 必须恰为 first-due G0…G5 的 canonical 149-ID 向量及固定 digest，L3 必须恰为 first-due G6 的 canonical 36-ID 向量及固定 digest；六个辅助 carrier TestID 只进入 `carrier_refs`，不进入 185 行 `test_results` |

其他 purpose/目标/状态组合全部失败关闭。

## 9. L3 Release Certification

G6 的首条命令固定为 release client validation，并同时建立权威 run header；只有 header 尚不存在时才接受完整七件套：

```powershell
cargo xtask f57 client-gate validate-selected --selection-receipt docs/decisions/f57-client-stack-decision.v1.json --candidate HEAD --release --storage-manifest <absolute-signed-storage-manifest-path> --deployment-manifest <absolute-deployment-manifest-path> --deployment-manifest-signature <absolute-deployment-manifest-signature-path> --deployment-trust-bundle <absolute-deployment-trust-bundle-path> --storage-trust-root <absolute-storage-trust-root-path> --storage-revocation <absolute-storage-revocation-path> --storage-checkpoint <absolute-storage-checkpoint-path> --bundle-root <g6-data-hdd-evidence-root> --run-journal <g6-data-hdd-evidence-root>\gate-run.jcs.jsonl --out <g6-data-hdd-evidence-root>\client-stack-validation.v1.json
```

这七个路径缺一、重复、相对路径或文件角色错配都在写入前失败。若在 header 前崩溃，使用同一完整命令恢复；header 一旦存在，所有重入和后续命令都禁止这七个参数，只从 header 固定的 in-bundle 归档解析并复验实时卷身份。

标准命令：

```powershell
cargo xtask f57 verify --level l3 --candidate <candidate-manifest-sha256> --candidate-manifest <g6-data-hdd-evidence-root>\candidate.v1.json --l2-evidence <g6-data-hdd-evidence-root>\l2-evidence.v1.json --bundle-root <g6-data-hdd-evidence-root> --run-journal <g6-data-hdd-evidence-root>\gate-run.jcs.jsonl --out <g6-data-hdd-evidence-root>\l3-evidence.v1.json
```

L3 不再发明一套含义重叠的十项 carrier 名称；它只精确连接以下两个正交集合。

候选制品 exact-set：

| 最终 `ReleaseArtifactRefV1` 逻辑 lane | 必需闭包 |
|---|---|
| `windows-authority` | 直接 typed-load 同一次 build carrier 已签的 `WindowsAuthorityArtifactSetV1`：Windows Server 2022 生产 MSI/Authority manifest、graph-exact `WindowsRuntimeDeploymentClosureV1`、五个 Authority launcher-role 服务、固定 G0 evidence-signer broker、六行 `WindowsServerComponentSetV1`、全部 graph-active carrier-specific artifacts 与 deferred-disabled absence contract、签名/工具链/运行拓扑闭包；总二进制/进程数不得硬编码，禁止二次 wrapper |
| `windows-client` | validation 产生且 artifact-set 只引用的 Windows 生产签名包，及八类生命周期证据：签名、安装/启动、升级、撤销、能力、资源、DLP、可访问性 |
| `macos-client` | 同一 Employee API 的 macOS 生产包及同样八类生命周期证据 |
| `ios-client` | 同一 Employee API 的 iOS 生产包及同样八类生命周期证据 |
| `android-client` | 同一 Employee API 的 Android 生产包及同样八类生命周期证据，含受管设备/Root 负例 |

物理认证 recipe exact-set：

| `ReleaseCarrierRecipeIdV1` | 必需证据 |
|---|---|
| `WINDOWS_AUTHORITY_BUILD` | 从 clean `HEAD` 生成 graph-exact `WindowsRuntimeDeploymentClosureV1` 并构建/签名每个 ACTIVE carrier 所需 PE/package/module/image/host binding；同时产生 Authority MSI/manifest、固定 G0 evidence-signer broker 包、exact 六行备份/恢复组件集与 off-host-only target 包。五个 Authority SCM 的 installed executable 都固定为 `ep-core-server.exe`，raw `ImagePath/BINARY_PATH_NAME` 则必须是 Windows-quoted executable 加各角色 exact argv 并通过 parser round-trip；Rust kernel 是 content-addressed version slot 中的 `ep-authority-kernel.dll`，且 unique generator `--check` 与 one-export PE golden 必须通过。RECOVERY_TOOL 行精确声明 fixed S4U Scheduled Task；DATA_VOLUME_UNLOCK_BROKER_SERVICE 行精确声明 PUBLIC_KEY broker；23-field artifact set 还必须分别绑定 strict `UpsAdapterManifestV1` 与 `Postgres16WindowsInstallContractV1`。DEFERRED_DISABLED 行没有可安装 artifact，制品/进程总数由 graph 决定 |
| `WINDOWS_SERVICE_INSTALL` | 按 closure 安装/激活每个 ACTIVE `WINDOWS_SERVICE\|JOB_OBJECT_WORKER\|IN_PROCESS\|WASM_SANDBOX\|HYPER_V_CONTAINER` 行并生成一一对应正读回，对每个 DEFERRED_DISABLED 行生成全零 absence readback；另安装并启动五个 Authority launcher-role 服务、固定 G0 evidence-signer broker、backup-writer/checkpoint-signer/data-volume-unlock-broker 三个 component service，按 fixed S4U action/account/token/SDDL/IPC/allowlist 注册但不后台运行 `\EnterprisePlatform\F57\RecoveryToolV1`，安装但不后台启动 passphrase-helper，证明 P340 未安装 target agent；同时取得九个 product SCM 行、完整 `10 + active_additional_windows_service_count` host inventory、quoted ImagePath/argv、SCM/Service SID、Scheduled Task、`0x00120183` pipe client DACL/second-instance denial、组件能力、持有映像、Authority `service_role` challenge、三服务独立 `component_id` challenge、PUBLIC_KEY/locator/WMI restricted-token readback、Job Object、CNG/TPM、BitLocker、W32Time。现有 `install-services.ps1` 还须按 artifact-set 内的 contract 安装 exact `ep-postgres16` demand-start/no-recovery 虚拟账户服务，并在 DATA_HDD 解锁前证明零 PostgreSQL 进程、启动后产出 signed 22-field install evidence 所认证的 strict `Postgres16WindowsInstallReadbackV1`；`downgrade_allowed=false`、`.control`/SBOM 扩展 exact-set、九路径投影/读回双射、关键 GUC/HBA/ident canonical vectors/effective byte equality、严格 loopback、TLS/SCRAM、checksums/durability、归档与无 tablespace/reparse/ambient override 全部通过，旧 build/lock、顺序/大小写/缺多/重复、路径碰撞或 effective 漂移全部失败 |
| `POSTGRES16_PITR` | PostgreSQL 16 流式备份/PITR、同一 write barrier 下覆盖 signed storage manifest 全部已启用 DATA_HDD authority class/root 的 `AuthorityRecoveryCutManifestV1`、checkpoint/cut digest binding 和两块不同离线介质 readback；cut 必须 exact-load active-config current-head `BackupTopologyV1`。APPEND_ONLY 与 A/B 两个既有 subordinate leaf 分别携带相同的当前 `StorageSafeguardReadbackV1` ref，三者与 cut 的 topology ref、PITR attempt/checkpoint exact-join；safeguard 的 target/support refs typed-load正确 tag/signature/quorum/projection，A/B transition sequence/predecessor/hash 闭合，checkpoint ref 等于 PITR binding，expiry 公式与 current head 有效。外层 16 字段及 18/17/30 registry 不变。只恢复 PostgreSQL/附件子集、复用旧 safeguard、target/quota/permission/partial/history 未知、容量不等式失败、A/B 含恢复材料或未物理断开均不能 PASS |
| `BACKUP_RESTORE_CERTIFICATION` | 同一物理尝试内严格 1/2/3 三阶段洁净恢复；三次分别从同一个 checkpoint/full recovery cut exact-restore 并逐行验证全部 authority class/root，覆盖备份投毒、最近备份/单块介质不可用、分域密钥、轮换、对账与 admission reopen gate；缺/多/重复/cross-barrier/cross-cut row 都失败 |
| `P340_RELEASE72_HOUR` | P340/i5-10500/32GiB、≥240GB SSD runtime、≥1TB CMR HDD data、15+3+2+1 会话、11+5+2+2 动作、重报表/自动化/备份/审计重叠、4321 个一分钟样本、25 个完整指标、十个 nested + 七个 supporting 签名 readback、72 小时 |
| `POWER_SHUTDOWN` | 最高安全档只接受候选绑定 `SIGNED_VENDOR_ADAPTER`；Windows standard carrier 的 UNKNOWN 与零控制能力只能形成不足证据。必须验证 manifest/binary/service SID/config generation/transport/credential security、实际 UPS 自检/通信与已签 P340 供电路径、900 秒阈值、typed outlet-cycle command/ACK 的同 ID 同 digest byte-identical query/adopt、不同 digest conflict、UNKNOWN 禁止重发、唯一 owner-token 关机调用、1074→13→6006→UPS off/on→12→6005、Authority recovery proof、双卷 clean、BIOS `POWER_ON` 自动来电启动、永久服务休眠/清 activation 及前后 distinct boot ID |

每个 recipe 先由 Rust 签一个含 exact typed inputs/outputs 的 staging plan，fsync `TEST_STARTED` 后才复制 plan/input 并调用固定 AllSigned 脚本。G6 的第一条命令先验证并归档 Stage-14 部署信任链与 `SignedF57AuthorityStorageManifestV1`，把 manifest ref、DATA_HDD volume identity、data-root digest 和派生 release-evidence-root digest 固化进 journal header；所有后续命令与恢复重入逐字匹配该 `VERIFIED_DATA_HDD_ROOT`。P340 冻结后以 Residency/geometry/同一 volume tuple 把它升级为 `CERTIFIED_DATA_HDD`，证书必须反向绑定同一 tuple，不能让前置步骤消费未来证书。Service 只接收从同一 terminal `WindowsAuthorityArtifactSetV1` 展开的 plan-bound signed artifact set、authority manifest 和 MSI 三项固定 path/media 输入，并要求 raw 的 set/MSI/runtime-deployment-closure/runtime-deployment-readback/component refs 精确回指该来源；P340 只接收 policy/input manifest；Power 只接收同一 terminal P340 链的 UPS identity/policy。前五个脚本只 create-new 写 raw；Rust 验证完整 cardinality 后签 completion/result。只有 `BACKUP_RESTORE_CERTIFICATION` 使用 1/2/3 三次有序物理恢复：三次都 exact-load 同一 `AuthorityRecoveryCutManifestV1` 与 checkpoint，逐行恢复/验证完整 authority-class exact-set；run 1/2 只写对应 raw，run 3 后才可 completion，部分三阶段 raw 只能 UNKNOWN，绝不重跑已开始的物理阶段。

备份/恢复部署闭集恰为 `BACKUP_WRITER_SERVICE`、`BACKUP_CHECKPOINT_SIGNER_SERVICE`、`DATA_VOLUME_UNLOCK_BROKER_SERVICE`、`RECOVERY_TOOL`、`PG_PASSPHRASE_HELPER`、`BACKUP_TARGET_AGENT`。前三项是 P340 上三种分离安全身份的 Windows 服务，其中 unlock broker 为 LocalSystem + restricted service SID；`RECOVERY_TOOL` 是非 SCM、按需 S4U Scheduled Task，`PG_PASSPHRASE_HELPER` 是按需工具；target agent 必须在不同主机且为独立包，禁止与 P340 共置、共享管理员或凭据域。`WINDOWS_AUTHORITY_BUILD` 必须从 clean frozen `HEAD` 构建并签名 Authority、G0 broker、这六个组件及每个 graph-ACTIVE carrier 需要的全部制品；总二进制/进程数从固定 inventory 加 graph projection/closure 派生，不能把历史九进程当 graph 判定。`WINDOWS_SERVICE_INSTALL` 必须 exact-load 同一 artifact set，安装五个 on-host 组件、验证 off-host target，并把与 runtime participant 重叠的 artifact/service/capability/live-process 身份作 equality join，不能用两次安装满足一行。RECOVERY_TOOL readback 必须逐字段证明固定 task identity/action/account/S4U runtime token/privileges/SDDL/IPC 和封闭 operation allowlist，无可变 argv/path/task/service/SQL/shell 输入；unlock broker readback必须闭合 fixed locator/trust/authority/NV/GPO/certificate/WMI/volume/protector 与 no-network 事实。三个 component service 各自使用 `WindowsBackupServiceRuntimeReadbackV1`，挑战响应必须绑定其 exact `component_id`、boot/PID/start-key/held-image/token-SID/nonce/session；不得借用 `WindowsAuthorityServiceRuntimeReadbackV1.challenged_role`、伪装成 Authority service role 或在组件间复用响应。writer 不能签 checkpoint，signer 不能读备份明文，unlock broker 不能读数据库/备份/签名/任意卷，任何缺行、混包、下载 helper、挑战域混用或能力合并都失败关闭。

`crates/platform/backup/src/safeguard.rs` 与 `docs/evidence/f57-backup-storage-safeguard.v1.schema.json` 是 `BackupTopologyV1|StorageSafeguardReadbackV1|StorageSafeguardSupportEvidenceV1` 的唯一 owner。CI 必须检查 active-config current-head、revision/predecessor/anti-fork、按 enum 六角色、writer/target SPKI、连续 target、按序 A/B、live failure/admin/credential/custody/location domain、每盘 exact 两个分域 human custodians、保留/physical-capacity/quota/reserve 不等式、partial optionality、至少两代与七日。每次 install/PITR/activation/retry 的 readback 使用新 nonce/session/object，expiry=`observed+max_age<=300s`，按 binding exact checkpoint；target support 单签、A/B transition/observation 双签，所有 ref kind/media/projection/chain exact；六角色权限拒绝矩阵和 A/B 断开/健康/no-recovery-material 通过。`AppendOnlySinkV1` 的 just-written token 是私有按值消费的一次性 capability；响应丢失只 adopt 已存 signed receipt，第二次读、历史枚举、target direct/unbound operation 和 mutation API 必须在编译或存储负例中失败。该闭合不新增 recipe、candidate signer 或 18/17/30 subordinate row。

两条 pre-freeze carrier 都 terminal PASS 后，必须运行 `cargo xtask f57 generation activate-installed-release --candidate HEAD --bundle-root <g6-data-hdd-evidence-root> --run-journal <g6-data-hdd-evidence-root>\gate-run.jcs.jsonl`。客户端只提交同运行 precursor checkpoint 与 build/install result refs；安装在 `EPAuthorityServer` 内的 `FinalInstalledGenerationAuthorityV1` 从当前 OBSERVED predecessor 创建或采用恰好下一代，typed-load graph-exact deployment closure/readback set、五个 Authority launcher-role 服务、固定 G0 broker、六组件/五 on-host readback、off-host target proof 及最终 artifact/端点/能力。它要求全体 graph/closure/readback participant IDs 一一对应，并要求 ACTIVE/positive/declaration/generation-required/ACK participant IDs 一一对应；`database_consumers` 与 generation item/`required_item_ids` relation 分别按 active graph 独立 exact-project，允许零或多 consumer 与多对多 item relation，但禁止 orphan/out-of-set item。DEFERRED_DISABLED 行只有 absence proof，不能进入任一 active relation。每个 participant 都必须 create-new 存储/reload apply readback；每个 package item row 都精确指向其 per-attempt transition、operation/results 与 installed-state evidence，十四字段 ACK 再绑定该 apply-readback ref。直到 complete ACK exact-set 使同一 attempt durable `OBSERVED_COMMITTED` 才返回。候选 freeze 随后只能通过租约固定的 `GenerationObservedReleaseSelectionRecordV1` 采用该 attempt并遍历完整 readback/transition closure；预安装、旧制品、desired-only、ACK 缺 `participant_apply_readback_ref`、混 attempt/readback、consumer/item relation 漂移、deferred participant 混入 active set、早期 generation 或响应丢失后另建 attempt 都不能进入候选。

`POWER_SHUTDOWN` 使用受控跨重启 continuation：五个服务永久 `AUTO_START`，分别为 ordinary `EPAuthorityServer`、`EPF57PowerShutdownContinuation`、`EPAuthorityControl`、无密钥 raw signer facade 与无密钥 journal signer facade；无 activation child key 值 `ActiveRecordPath` 时 continuation 只做零副作用自检后 `STOPPED`。raw/journal facade 只把角色封闭的 frozen operation 转发给 G0 `F57EvidenceSignerV1`；该 G0 broker/session 是两个证据 key container 的唯一持有者，facade 无 key ACE、不得直接或跨角色用钥。POWER 对五个 SCM 对象、安装映像、bundle/journal/staging/capsule/state、专用 activation child registry key、四条 pipe（control/raw/journal/recovery-proof）与两只 G0 broker key 使用 exact 18-row object/action SDDL：SCM、安装映像、bundle/run 根、已安装 activation child key、四条永久 pipe 与两只 key 只允许 `VERIFY_EXISTING_IMMUTABLE`，仅本次 staging/capsule/state 允许 `CREATE_WITH_DESCRIPTOR`；任何漂移都不得修复，也不得把 registry value 当 ACL 对象。首次命令 prepare-only；固定 SSD capsule 先落 typed `UpsOutletCycleCommandV1` 的 dispatch intent，再落 API-commit marker，然后才允许唯一一次 Windows API 调用；精确 User32/1074 后以该 command ID/digest 调用 `UpsOutletControlPortV1::begin_or_adopt|query_exact`，User32 原始记录与 byte-identical typed `UpsOutletCycleCommandAckV1` 必须组成预关机已 fsync 的不可分 composite acknowledgement。相同 ID 不同 digest、设备无法查询导致的 UNKNOWN、changed boot、manifest/config/service/transport/credential drift 均禁止重发或重建 ACK。控制 broker 的 `SERVICE_ACCEPT_PRESHUTDOWN`/600000ms 策略只完成这个既有 attempt，绝不产生第二次 API 或新 UPS ID。下一次启动只读验证固定 ACK 后才可创建 postboot controller；ACK 缺失即使两项外部事实事后都可见也禁止重建。成功要求七个 plain PASS 对象加 intent/API-marker/composite-ACK 三个非 PASS 控制、四段状态 prefix、Authority recovery proof、前后 boot/lifecycle、BIOS `POWER_ON` 和 outlet off/on 闭环。失败严格分为四个无 controller 的前置码与七个带 controller 的后置码，保留精确一/二/三个控制对象并终结 UNKNOWN；DATA_HDD 锁定时只能先删除 activation value、证明 continuation 保持注册/`AUTO_START` 且休眠并保留 capsule，待 HDD/人工供电或 UPS 修复后同 attempt 重入才能写终态，绝不伪造自动恢复或 PASS。

UPS 内层时间契约与 POWER 外层窗口必须分别断言：manifest 固定 `5/15/86400/30` 秒；每个 status 的 runtime-binding digest exact-join signed identity，initial/previous/trigger 拒绝跨 boot/PID/start-key，sequence 只在该 binding 递增；vendor self-test 必须由 closed raw provider attestation 证明 nonfuture 且 <=24h PASS。USB GUID/device instance 与 network numeric-IP/nonzero-port structured endpoint 必须 canonical，网络 readback exact 一行。adapter 在任何 provider 调用前耐久化一次私有 start marker，ACK 必须与 command 使用同一 boot/source，并满足 checked `start <= acknowledgement_observed <= min(start+30000,command deadline)`。响应丢失只可在该内层期限内 query/adopt 字节相同 ACK；30 秒未知即 `COMMAND_STATE_UNKNOWN` 且零重发。两项 UTC 时间只供报告，篡改 UTC 不得改变 verdict；`600000ms` POWER 窗只完成 User32/composite/preshutdown 对账，不能放宽、重置或复活内层期限。

Package foundation restore 是另一条受控跨重启路径；`docs/evidence/f57-package-recovery-control.v1.schema.json` 与 recovery-tool 是 recovery-domain-signed kernel pointer/capsule 的唯一 owner，它们不属于 package trust signed roots。恢复前，Scheduled Task recovery tool 必须在 restore set 之外冻结 `PackageRecoveryContinuationCapsuleV1`，并以 `CAPSULE_FROZEN -> RESTORE_INTENT_COMMITTED -> RESTORE_PROVIDER_COMMITTED -> RESTORE_VERIFIED -> SQL_STATE_RESEALED -> CAPSULE_RETIRED` 的 append-only chain推进；capsule绑定 rollback/request/result、plan/execution authorization、checkpoint/full recovery cut、pre-restore row hashes 与 content-addressed terminal reseal payload。启动时先验证 capsule、TPM/off-host monotonic head和外部 restore/readback，再对 restored SQL 执行唯一 expected-restored→terminal CAS；不得信任已回滚 SQL 决定 continuation、重放 forward 或用 hash 猜缺失数据。

所有五个 Authority 服务的 `installed_executable_path` 永久且逐字等于 `C:\Program Files\EnterprisePlatform\Authority\ep-core-server.exe`；SCM `ImagePath/BINARY_PATH_NAME` 的 raw value 则分别等于 Windows-quoted 该路径加 master 五行 `ServiceInstall.Arguments`，并必须 round-trip 为 exact role argv。maintenance 不调用 `ChangeServiceConfig`、不覆盖该 launcher。Rust-kernel implementation 只能暂存为 `versions\<implementation-set-sha256>\ep-authority-kernel.dll`，由 recovery tool 校验 final handle/SSD/AuthentiCode/SBOM/digest 后 query/adopt recovery-domain-signed `AuthorityKernelSlotPointerV1`；pointer绑定 monotonic slot、目标 generation/transition、predecessor 和 recovery-journal head，launcher每次启动先验 CMS/head/bytes 再载入 DLL，前 slot保留到 generation OBSERVED 且 production admitted。

`RUNTIME_SSD` 持久检查不是“磁盘只有四个目录”。全卷覆盖先分为不相交 Set A/Set B：Set A 是 signed `RuntimeSsdReproducibleRuntimeInventoryV1` 中可由 Windows catalog、candidate artifact、bounded OS cache 或 TPM-bound reenrollment metadata 重建且不含客户字段的字节；Set B 是 exact 四个 mutable exception class，即 bounded POWER capsule、bounded package recovery-continuation capsule、recovery-domain-signed kernel slot pointer/journal head、content-addressed signed native-code slots/cache，并闭合十九 media contract/二十 path row、大小/实例/保留/重建 authority。完整 locator-resolution、全卷 allocated-stream classification、ADS/hard-link/VSS 与 canary/business-digest negatives 必须证明每个持久 entry 唯一落入 A 或 B；零 rejected/unclassified/inaccessible/partial。Windows post-reboot persistent policy 另 exact 八行：pagefile、swapfile、hibernation、kernel/full dump、minidump、WER LocalDump 六行关闭；VSS diff area 与 product quarantine 只在 verified DATA_HDD。OS telemetry policy exact 七行：四行 bounded no-customer SSD、firewall text log disabled、HTTP.sys error 与 Authority access audit 两行 DATA_HDD。SSD-loss drill 必须从 authenticated DATA_HDD + off-host heads重建 control/pointer并从签名清单重置代码；terminal capsule 在 HDD/off-host terminal proof durable 后安全退休。Set B 的第五类、Set A byte/catalog mismatch、任何客户/业务 authority 字节或隐藏 Windows persistence 均使 residency 与 production admission 失败。

普通开机与 clean-SSD 恢复是两套门。普通开机必须由专用 unlock broker 通过 fixed locator/bootstrap closure、Microsoft Platform Crypto Provider 的既有 TPM-backed nonexportable key、explicit certificate thumbprint 和 local packet-private WMI 完成 PUBLIC_KEY unlock；recovery task、continuation、AS 都不得取得裸私钥或 48-digit recovery password。clean-SSD/TPM-loss 必须保留 admission closed，按双人 off-host recovery-password ceremony 走八步 `SsdDataHddRecoveryAndReenrollmentReadbackV1`：解锁旧 protector、建立新 TPM key/CA certificate、添加/验证新 PUBLIC_KEY protector、提升 unlock-authority epoch 与 TPM NV anti-rollback、正常重启验证 broker unlock、最后移除旧 protector。任何由 public metadata “重建原 key”、提前开 admission、auto-unlock=true、步骤/前驱 hash/operation ID 漂移或 secret leak 都失败。

证据权威性不是第七个 recipe：L3/证书验证器自身必须复验 185 个 TestID 的 bijection、全部 manifest/digest、89-row evidence signer registry、独立 generation/migration approval trust binding、graph participant → runtime-deployment closure/readback 双射、ACTIVE participant → positive readback/declaration participant/generation-required participant/ACK participant 双射、database-consumer 独立 exact 投影、generation item 与 participant-item edge 独立 exact 投影，再沿 OBSERVED selection → certification 闭环，以及固定 G0 broker、六行组件集/五行 on-host 安装读回/off-host target proof、签名链、撤销、可信时间、候选身份、journal 前缀和各 sole-store 路径。五个候选制品与六个 recipe 结果都必须同一 candidate/run/profile、在有效期内；任一集合缺失、额外、重复、别名或跨运行均失败关闭。G0…G5 prerequisite 使用 plain `GateReceiptRefV1` typed-load 已签 receipt，不再给 ref 二次签名。

长时硬件与恢复证据由受控 carrier 生成，生产 Authority 只参与本机测量，不承担全局 CI 聚合。所有 carrier evidence 必须为同一 candidate/profile，且在各自有效期内。

## 10. 证书后生产启用

`RELEASE_CERTIFIED` 只说明一组不可变制品及其证据符合发布规范，不自动打开业务流量。客户先在 Authority 不可变审计/事实库中记录一份未过期的 `SingleDiskDegradedProductionAcceptanceV1`，其部署、候选与证书必须完全一致，`certified_concurrent_users=20`，两名批准人必须是不同且当前有权的客户主体，并明确接受以下 exact 五项风险：

1. `SINGLE_DATA_HDD_NO_RAID`；
2. `NO_HIGH_AVAILABILITY`；
3. `RANSOMWARE_RECOVERY_DEPENDS_ON_EXTERNAL_APPEND_ONLY_AND_OFFLINE_ROTATION`；
4. `MANUAL_RECOVERY_MAY_BE_REQUIRED`；
5. `LOCAL_AI_DEFERRED`。

接受记录还必须完整绑定已认证的外部只追加目标、两块离线轮换介质、恢复保管链、UPS、BitLocker、TPM 与洁净恢复证据。只有在该事实已经记录后，才运行：

```powershell
cargo xtask f57 production activate --receipt <g6-data-hdd-evidence-root>\g6\release-certificate.v1.json --bundle-root <g6-data-hdd-evidence-root> --acceptance-id <approved-single-disk-acceptance-id>
```

`ProductionActivationAuthorityV1` 必须先离线重验 release certificate，再从显式 bundle 重建候选、final-installed OBSERVED selection、拓扑认证、P340/certified-HDD、graph-exact runtime deployment、五个 Authority launcher-role 服务、固定 G0 broker、六组件/五 on-host 安装、PostgreSQL install contract/readback、备份/恢复与现场保障闭包，并采集四类新鲜 sealed live readback：topology、runtime deployment、Authority/G0-broker/backup-component installed readback、strict `StorageSafeguardReadbackV1`。safeguard 必须在 exact purpose/media 下 strict-load，同一 activation/retry/attempt/candidate/certificate/selection/当前 boot，expiry 从同一 `BackupTopologyV1` 推导，且每次 retry 都使用新 challenge、新 mTLS session binding 和新对象；仍未过期的旧 ref 也拒绝。runtime deployment 读回必须重新证明全体 graph/closure/readback participant IDs 一一对应、ACTIVE/positive/declaration/generation-required/ACK participant IDs 一一对应、database-consumer 与 generation item/subset 两个独立投影 exact-match，以及所有 DEFERRED_DISABLED 行仍不存在于 active relations；该类漂移使用 typed `RuntimeDeploymentDrift`。非法证书、接受或 safeguard 输入在 attempt 创建前失败；合法 request 后的 collector 故障或任一类 live drift 则以 typed failure code 追加 `FAILED_HELD`，保留同一 `activation_id`、全部历史并继续隔离业务路由。修复后再次执行完全相同命令，authority 必须重新验证同一仍有效的证书和双人接受，以 exact prior failure record hash 做 CAS 追加严格递增 ordinal 的 `RETRY_COMMITTED`，然后完整重采上述四类 live readback；旧 readback不可复用。允许的独立 verifier 路径止于 `REQUEST_COMMITTED|RETRY_COMMITTED -> LIVE_READBACK_BOUND`；任何 pre-activation state 可进入 `FAILED_HELD`，held 只能 `FAILED_HELD -> RETRY_COMMITTED`。唯一 upper 联合事务才允许 `LIVE_READBACK_BOUND -> ACTIVATED + GENESIS_FULL_CERTIFICATION`。成功响应丢失重入返回同一联合结果；证书/接受过期或改变、runtime participant/carrier/config/absence/consumer/item-relation 漂移、其他现场漂移、跳过/复用 readback、一人批准、风险或保障缺失、target 共置、第二 `activation_id`、stale writer 或 desired-only generation 均保持隔离且非零。

联合事务精确绑定同一 final-installed durable OBSERVED 的 epoch/generation/manifest/digest/attempt、完整 ACK exact-set、transition exact-set、认证 resource envelope 和 impacted-scope exact-set；它锁定 activation-ready row、OBSERVED 和 admission head，在一个 commit 中写 activation terminal、genesis admission/head 与新 `business_api_generation`。崩溃点与响应丢失测试必须证明不可观察到“只有 ACTIVATED”或“只有 genesis”的状态。所有业务 command/query 都在同一次 admission read 中 exact-check current durable OBSERVED tuple 等于 current admitted tuple，且目标 scope 无 live `ProductionAdmissionHoldV1`；route 不得只检查 activation row、package-local `APPLIED_VERIFIED`、desired generation 或可变开关。只有联合结果成功后才显示 `PRODUCTION_ACTIVATED_SINGLE_DISK_DEGRADED`，不得声称 RAID 或 HA。

生产路由旁路登记必须 exact 为以下十行，且每行 response class 都是 `CONTROL_METADATA_NO_CUSTOMER_FIELDS`、只 bypass production admission/hold，绝不 bypass parsing/authentication/CSRF/authorization/MFA/SoD/epoch/rate-limit/audit，也不能构造 business `AuthorizedPgTx`：

| Method + canonical route | Selector | Authorization |
|---|---|---|
| `GET /internal/v1/health/live` | `HEALTH_LIVE` | `f57.platform.health.read` |
| `GET /internal/v1/health/ready` | `HEALTH_READY` | `f57.platform.health.read` |
| `POST /control/v1/commands` | `PRODUCTION_ACTIVATION_COMMAND` | `f57.production.activate` |
| `POST /control/v1/commands` | `PACKAGE_MAINTENANCE_COMMAND` | `f57.package.maintenance.execute` |
| `POST /control/v1/commands` | `RECOVERY_OPERATION_COMMAND` | `f57.recovery.operate` |
| `POST /control/v1/queries` | `PRODUCTION_ADMISSION_STATUS_QUERY` | `f57.production.admission.read` |
| `POST /control/v1/queries` | `PACKAGE_MAINTENANCE_QUERY` | `f57.package.maintenance.read` |
| `POST /control/v1/queries` | `RECOVERY_OPERATION_QUERY` | `f57.recovery.operation.read` |
| `POST /control/v1/queries` | `AUTHORITY_RECOVERY_PROOF_QUERY` | `f57.recovery.proof.read` |
| `GET /control/v1/commands/{request_id}` | `ORIGINAL_COMMAND_BYPASS_INHERIT` | exact stored original command classification/capability |

这十行只能由 Authority build 使用与 router/capability registry 相同的 compiled source 生成一份 canonical JCS artifact。`WindowsAuthorityArtifactSetV1.production_admission_bypass_registry_ref` 必须在 signed build result 中绑定其 exact bytes 与 `application/vnd.ep.f57-production-admission-bypass-registry-v1+json`；`ReleaseWindowsServiceInstallEvidenceV1.production_admission_bypass_registry_ref` 必须 exact-repeat 同一 ref，并以安装读回证明 installed bytes；final candidate 只能经已签名 `WindowsAuthorityArtifactSetV1` 到达该 ref，startup 也只能 typed-load 该 candidate-bound installed ref。ambient file、数据库可编辑 route、latest-file scan、未进入 candidate closure 的 build-only registry、跨 run/attempt/build 字节或 ref 全部在 bypass route ready 前失败关闭。

每个非 bypass 业务请求必须使用 stable authenticated `request_id` 与 full binding digest，在 admission head/hold writer 共用的 deployment-scoped serializable lock/CAS 下，一次完成服务器派生 authority epoch、OBSERVED/admitted/API-generation/scope/hold 检查和 exact twenty-field `ProductionAdmissionExecutionLeaseV1{state=ACCEPTED}` create-new/exact-adopt，才得到 affine permit；epoch 必须同时等于 current authority、typed admission、OBSERVED generation 与 verified security context并进入 binding digest，客户端不可提交，不存在 check→handler 间隙或进程内计数器。command/query commit 只能把同一 epoch 的 lease 终结为 `COMPLETED_IN_PLACE`，exact-once Outbox/workflow handoff只能终结为 `HANDED_OFF_EXACT_ONCE`。hold writer 在同一锁中先落 `ADMISSION_CLOSED`，并且只有同一事务跨全部 epoch 的 intersecting ACCEPTED count 为零才可落 `DRAIN_COMPLETE|BARRIER_COMMITTED`。唯一约束保持 `(deployment_id,request_id)`；request ID 换 payload/epoch、cross-epoch replay、old-epoch orphan ACCEPTED、terminal epoch/result/ref 漂移、permit-before-hold/hold-before-permit、响应丢失、进程崩溃、terminal-CAS loss 与 multi-scope race 都必须 fail closed 或 exact-adopt同一结果，不能产生第二次 effect；任何测试若能观察 barrier 后新业务写即失败。

后续 package delta 先按 scope落持久化 hold；global class 必须关闭全部业务写入，entity-local maintenance 至少关闭对应 scope。新 generation 一旦成为 global OBSERVED 而 admission 尚未前移，所有路由因 generation/admission mismatch 自动关闭，直至单个 deployment-wide admission CAS 对完整 impacted-scope/transition set 原子成功；不存在逐法人部分准入。普通 class 的 `PACKAGE_DELTA` 只有在 fresh ACK/transition/readback证明 permission/data/network/host/executor/resource footprint 完全位于既有认证 envelope 内、无新 participant/service/database consumer/capacity 或 safeguard weakening 时才可 reopen。四类 global package、任何资源增加或 runtime topology扩张都必须重新完成 clean build/install、P340/capacity/recovery certification、release certificate 和客户 production activation。`ROLLBACK_REOPEN` 只在 fresh typed predecessor readback 与 prior admission byte-exact 时合法；UNKNOWN、missing proof、stale writer 或 crash 始终保持 hold，multi-scope delta 必须 all-or-nothing。

## 11. 签名层级

- L0：无发布签名，只记录本地结果；不能进入 release aggregate。
- L1：受控 CI 测试身份签名 lane receipt；不能签产品制品。
- L2：候选证据身份签名，Windows 安装器可使用明确标记的内部测试签名；不能分发生产。
- L3：Windows Authenticode、Apple、iOS、Android 和证据聚合分别由隔离签名角色完成；聚合只携带公开链、撤销、时间戳和 digest，不携带私钥。

生产签名键、客户 generation signer 和 CI evidence signer 是独立故障域。生产 P340、普通 CI runner、compiler worker、plugin/provider 和远程支持均不得取得可导出的签名私钥。`EPF57PowerRawSigner` 与 `EPF57GateJournalSigner` 只是无密钥服务 facade；它们只能按固定 role/discriminator 把已冻结操作转发到 G0 `F57EvidenceSignerV1`。G0 broker/session 是 raw/journal CNG key container 的唯一 owner，facade token 与 SDDL 均无 key ACE。

## 12. 离线与供应链

全部 release build/test/scan/aggregate 在无公网环境使用锁文件、固定工具链和离线依赖仓库。普通 runner 不临时下载 Cargo、npm、Xcode、Gradle、scanner 或签名工具。最终候选冻结前生成并签名 `F57OfflineSchemaManifestV1`：从 canonical `RELEASE_CERTIFICATE_V1` schema root 与最小 bootstrap 自动求全部 reachable typed JSON 与 `$ref` 传递闭包，按 exact digest 复制到 bundle。schema closure 必须含 foundation、generation manifest/reverse-plan/14-field ACK、participant apply/rollback readbacks、generation approval、runtime topology/deployment、server components、capability-package operational media、package registry、backup checkpoint/full recovery cut、backup topology/storage safeguard/support evidence、PostgreSQL 16 Windows package-lock/install-contract/install-readback、UPS manifest/status/typed command/ACK、production-generation-admission、production-admission-bypass registry、production-admission execution lease 和 client-common sole owners。

对象遍历从每个 signed-generation package item 依次跟随 `desired item -> portable package registry + desired package -> implementation manifest -> every implementation artifact/SBOM/schema/WIT/signature/migration/foundation ref -> concrete scope tenancy snapshot`。reverse 分支必须按 tag 完整遍历：`RESTORE_ARTIFACT -> prior desired item`；`DEACTIVATE_RETAIN_DATA -> original forward item + per-attempt transition + ABSENT installed-state readback + nonnull retained-data proof`；`NO_OP -> source desired item`。generation 1 的 `NO_OBSERVED_GENERATION` 只能使用第二分支或策略允许的 NO_OP，不得制造前代。从每个 selected ACK 必须跟随 `participant_apply_readback_ref -> canonical item readbacks -> nonnull per-attempt generation transition -> execution-trust snapshot + typed operation request/result + final installed-state readback`；maintenance transition 还必须到达 reservation、signed historical 30-field plan、current execution authorization、两份 current decision、source/target implementation closure、admission hold、actual BACKUP checkpoint 和完整 `AuthorityRecoveryCutManifestV1`。若验证 production delta，则还要到达 exact predecessor admission、ACK/transition exact-sets 与 resource envelope。离线 verifier 重做 participant双射、consumer/item-edge 独立投影、scope/action/state/probe/window/CMS/decision/registry-pin/reverse/rollback/checkpoint/cut/admission binding及 deferred排除；缺/多/orphan/wrong-media/cross-scope/cross-cut ref、implementation manifest 外字节或 package-local `APPLIED_VERIFIED` 冒充 OBSERVED/admitted 都失败。

`PackageMaintenancePlanFinalizationStoreV1` 的 plan-finalization journal 只用于在线 crash recovery：offline bundle 不复制、不遍历、不 replay 它，也不把其不存在当证书缺口。离线闭包只携带不可变 artifacts/readbacks；禁止固定文件数、目录猜测、“latest”选择、开发 allowlist 或联网解析。签名区若需时间戳，只连接客户批准的受控 RFC 3161 TSA；TSA、链、CRL、策略、message imprint、nonce 或可信时间未知时失败关闭。

SBOM、依赖许可证、漏洞状态和构建来源绑定 candidate identity。漏洞映射使用 `AFFECTED|NOT_AFFECTED|UNKNOWN`；`UNKNOWN` 不显示绿色。

## 13. 证据保存与重验

证据目录不是源码仓库。源代码只保存 schema、policy 和非秘密公开 fixture；运行证据进入 DACL 保护的外部 evidence root，并按 manifest 被 hash/sign。

每个 receipt 至少保存：gate/level、candidate identity、deployment/hardware/storage profile、selected TestID exact-set、逐 TestID outcome、Requirement binding、runner/carrier identity、起止时间、工具链、日志/制品 digest、签名和有效期。

最终离线重验命令：

```powershell
cargo xtask f57 evidence verify --receipt <g6-data-hdd-evidence-root>\g6\release-certificate.v1.json --bundle-root <g6-data-hdd-evidence-root> --expect-type RELEASE_CERTIFICATE_V1 --offline
```

验证器先用最小 bootstrap 到达候选内的 signer registry 与 offline schema manifest，再按 manifest 对最终证书可达的每个 signed JSON 重做 schema、签名、ref、size/digest 和闭包验证；它必须 exact-load `WindowsRuntimeDeploymentClosureV1|WindowsRuntimeDeploymentReadbackSetV1` 并重验 graph 全集/ACTIVE 子集/DEFERRED_DISABLED 排除关系，不能用当前机器扫描结果或固定进程表替代。任何 schema 缺失/额外/版本漂移、runtime deployment 集合漂移或网络解析均失败。只有同时满足 185/185 release-due 行、零未知/过期/缺失 evidence、零启用延期能力、零 candidate mismatch 才返回 0。返回 0 的状态仍只是 `RELEASE_CERTIFIED`；生产启用必须再执行 §10，且 activation record 不反向改变证书字节或 L3 结果。

## 14. 当前实施状态

本文是已批准的目标契约，不是已通过的流水线证据。当前仓库只能声称：

```text
CI_DESIGN_APPROVED
PRODUCT_DECISIONS_OPEN=0
REQUIREMENT_SET=185
TEST_ID_REGISTRY=276
SIGNER_REGISTRY_ROWS=89
DEVELOPMENT_READINESS=READY_NOT_AUTHORIZED
DEVELOPMENT_AUTHORIZATION_REQUIRED
DEVELOPMENT_AUTHORIZED=false
IMPLEMENTATION_NOT_STARTED
NO_F57_GATE_RECEIPT
NO_RELEASE_CERTIFICATE
NO_PRODUCTION_ACTIVATION
NO_PRODUCTION_GENERATION_ADMISSION
```

首次授权开发时只执行 G0 的 L0/L1 架构和生成门。L2、L3 不能因命令已预登记而提前显示 PASS。
