# ServerAdmin and Customer-Controlled Deployment Carrier Implementation Plan

> **F-57 状态：`SUPERSEDED_DO_NOT_EXECUTE`。** 本文件只保留为历史设计输入，不得单独或续跑执行；F-57 已将管理面定义为 Windows Server 权威端控制中心，并与员工 Workbench 分离。获得另行开发授权后也只能从 [2026-08-24 收敛实施主计划](2026-08-24-f57-converged-program.md) 指向的 G0 bootstrap 子计划开始，不存在本文件的续跑入口。

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** 交付嵌入 core-server 的独立 ServerAdmin 静态 SPA、精确 90 格的第五客户端能力矩阵，以及客户自控物理机/境内 IaaS VM 两种等价单机承载体及其证据门禁。

**Architecture:** `ClientKind` 同批扩为八值，持久设备只新增 `server_admin`，MCP 只进入 audit 和协议上下文；能力表新增 18 个 ServerAdmin 行并用二进制冻结快照兜底。ServerAdmin 在构建期生成静态资源并嵌入 core-server 的现有员工 HTTPS `/server-admin/`，只消费受权 API；许可证与声明式签名模块包只组合 F-56 已冻结的 config-package multipart/import/autotest/submit/sign/release 及脱敏 bootstrap，不新增管理后端或审批结论路径。承载体在 Stage 14a0 已冻结的 generic deployment/evidence 基础上扩展部署事实和 validator，再由 Stage 14b 做真实校验；不改变 Windows Server 2022 原生服务、PostgreSQL 16、本地附件、KMS/HSM、备份或恢复拓扑。

**Tech Stack:** Rust 2021、Axum、PostgreSQL 16、React/TypeScript/Vite build-time toolchain、embedded static assets、existing UI component library、Windows Server 2022/vTPM、PowerShell/Rust evidence probes、Playwright browser tests、真实 PostgreSQL 与 Stage 14 recovery tests。

**Specs:** `docs/superpowers/specs/2026-08-22-f55-approved-ai-mcp-server-admin-cloud-freeze.md`；许可证、签名模块包、ServerAdmin 导入/只读审批与 AI/MCP entitlement 以后续 `docs/superpowers/specs/2026-08-22-f56-license-signed-module-package-freeze.md` 为唯一权威。

## Global Constraints

- 本计划只在用户另行授权后执行；当前只冻结文档，不运行产品开发、迁移、构建、测试或发布。
- 本文件是任务内容与任务内 step 顺序的唯一来源，但**不是跨线调度入口**。首版集成必须从 `2026-08-10-first-release-dev-plan/13c-local-ai-mcp-server-admin.md` 开始，并按其 Task 1–6 DAG 调用本文件的指定 Task/Step；禁止独立从本文件 Task 1 一路执行到 Task 6。F-56 Stage 3b/13b 必须先交付许可/模块 runtime、special config-package handler、ServerAdmin multipart import 与 bootstrap DTO；本文件只消费它们，不再实现第二套。Stage 14a0 先提供 generic deployment record 与通用证据框架，本文件再交付 carrier schema/validator/gate，真实现场认证只由 Stage 14b 执行；13c entitlement wiring 另须等待 `F-56 Stage 3b-2 → Stage 14a1-F56-adapter`，不得把 14a1 误当本 carrier 计划的基础所有者。主计划只重排任务批次，不得改变本文件每个任务内部的 failing-test→implement→verify 顺序。
- 本计划固定依赖为 `Stage 14a0 evidence foundation → 本计划/Stage 13c → Stage 14b final certification`。Stage 14a0 唯一拥有 `platform_ops.deployment_records` 基础迁移、`DeploymentRecord` 当前版本模型/仓储/选择器、服务器/备份 evidence contract/fixture、bounded strict-JCS/opaque-ref/digest/signature 校验原语和 fail-closed release-gate registry；本计划只消费并通过 `V20261024090700` 扩展，不得复制或反向接管这些基础。Stage 14a1-F56-adapter 只拥有 F-56 query/adapter 且无权产生真实 PASS；Task 6 只实现 gate/parser/fixture并生成候选 evidence，实机取证、签名报告和最终发布判定只在 Stage 14b 发生。
- Stage 14a0 必须先把阶段 14 自有的全部 28 个 `V20261023...` 迁移（末项 `V20261023092600`）写入 catalog；任一可变共享数据库必须再接续 Stage 6 的 `V20261023092700`、`V20261023092800`，先通过全局 pre-F55 恰 30 条、末项 092800 的 `PreF55DatabaseAdmissionV1`，才可执行本计划所属的 F-55 `V20261024...` 九迁移批次。Stage 14b 不得补写更低版本；Stage 14a0、14a1 或本计划完成都不构成部分发布。
- `ClientKind` 恰好八值且顺序固定：`Win, Mac, Ios, Android, Portal, Ops, ServerAdmin, Mcp`；序列化为 `win|mac|ios|android|portal|ops|server_admin|mcp`。
- `platform_audit.audit_events.client` 恰好上述八值加 `system`；`user_devices.client` 只新增 `server_admin`，MCP 复用 grant 来源 device 且不落设备 client。
- ServerAdmin 不创设角色或超级权限，不绕过 RLS、字段权限、密级、记录范围、SoD、审批、许可证或 re-auth。
- ServerAdmin 的许可证/模块包管理唯一入口是 F-56 已冻结的同路径同权限 `POST /api/v1/platform/config-packages/actions/import`：Win/Mac 继续使用 `application/json {attachment_object_id}`；ServerAdmin 只可发送 `multipart/form-data`，恰一个名为 `package`、文件名以小写 `.epcfg` 结尾的 file part，零其他 part/form field。该路径是唯一编译期 route-local body-limit 窄例外：`Content-Length` 必须存在且为 `1..=4,194,304` 的十进制值，缺失、非法、为零、超限或 `Transfer-Encoding: chunked` 全在读 body 前拒绝；流式读取仍以 4,194,304 bytes 硬截止并拒绝短读/长读，其他路由保持全局 1 MiB 上限。handler 只用 CREATE_NEW 写固定 `C:\ProgramData\EnterprisePlatform\staging\config-import\<request-id>.epcfg`；目录 owner SYSTEM、关闭继承且显式 DACL 只允许 SYSTEM/Administrators/`NT SERVICE\ep-core` 管理，其余无 ACE，并拒绝 UNC/device/reparse/ADS/hardlink。逐流计算 digest 并验完后，无论成功或失败都先关闭句柄再删除；它不是 attachment/通用文件能力，不能列表、下载或读取，`platform.document_attachment=NOT_APPLICABLE` 保持不变。
- ServerAdmin 只可组合 import、autotest、submit-for-approval、sign、release-order/execute；`approve|reject` 与标准任务完成结论始终只允许 Win/Mac。ServerAdmin 只能只读显示审批待办与结论，不能调用/代理/隐藏触发 approve/reject，也不能 direct DB/KMS/file/service。special `LICENSE_GRANT|MODULE_PACKAGE` 从 import 起 immutable，sign 只复验并保留发行方 exact signature bytes，通用 ROLLBACK 必须拒绝；续期、撤销、安装、启用、停用、升级和版本回退均通过新的 imported/signed 单项包，不提供布尔开关。
- 当许可为 `Restricted` 或不存在 current grant 时，向导可达性严格缩为 F-56 恢复例外：`LICENSE_GRANT` 的 import→autotest→submit→Win/Mac `CONFIG_RELEASE` 审批结论→sign→release-order/execute，以及 `MODULE_PACKAGE:DISABLE` 的同链；普通配置项、模块 `INSTALL|ENABLE|UPGRADE|ROLLBACK_VERSION` 与其他业务审批都以 `PLATFORM.LICENSE.RESTRICTED=BUSINESS_CONFLICT/409/false` 拒绝。ServerAdmin 在该例外中仍只提交/观察，审批结论仍只在 Win/Mac，不能出现 approve/reject 代理或隐藏动作。
- `GET /api/v1/platform/client-bootstrap?client=server_admin` 的可空 `license_module_admin` 是唯一许可证/模块只读 DTO；只有已认证 ServerAdmin 且有 `lowcode.config_package.view` 才填充，其他 client 或缺权限逐字为 null。字段闭集、masked license number、四态/可信时间/三项用量、module/entitlement code 集与 15 行模块摘要逐字按 F-56 §6；无 current 或签名失效时不得回显未受信的许可身份、日期、code 或 limit，但三项实际 usage count 仍返回。任何状态都禁止 signature/payload/source ref/path/key ref/secret。
- 能力矩阵恰好 90 行；MCP 零矩阵行。ServerAdmin 2 个 FULL、3 个 VIEW_ONLY、13 个 NOT_APPLICABLE 的逐格值固定为 F-55 §5.2。
- ServerAdmin 是 `clients/server-admin` 独立 route tree/artifact；生产只有 build-time Node，运行期零 Node、零开发服务器、零新后端进程、零新监听端口、零可写静态目录。
- `/server-admin/` 复用员工登录/MFA/session/device/CSRF/CSP/bootstrap；自填 `X-Client:mcp` 无效，`X-Client:server_admin` 仍受能力矩阵和普通 authz。
- AI reporting 的 compose/execute route 必须接受 `X-Client:server_admin`；其完整 client enum 恰为 `win|mac|ios|android|ops|server_admin`，拒绝 portal/mcp。ServerAdmin 的 `ReportingReportPrint=VIEW_ONLY` 只允许只读分析体验，不自动授予 AI：每次仍需相应 `reporting.ai_analysis.compose|execute` 权限与对象范围、AI license、当前 session/device/法人/密级/字段/记录条件。模型包管理只读权限 `platform.admin.ai_model.view` 也不能替代这两项 AI 权限。
- ServerAdmin 不提供客户、合同、订单、采购、库存、项目、主数据、财务、发票、附件或门户业务写；VIEW_ONLY 不承载审批通过/驳回。
- 共享 `HighRiskOperation` 恰好七值：原六类业务高风险加运维高风险 `DATA_MIGRATION`。ServerAdmin 只能查看迁移证据，不提供数据迁移执行/重放/绕过审批入口，也不得保留旧六值快照。
- ServerAdmin 展示 LOCAL MCP 安装状态时只读十五列整组 joint-null 或 transport-complete receipt/materialization：共同 receipt/root/time/root-SD；stdio 的 HCS 空且 profile/SID/provider/sublayer及 CONNECT_V4/V6、RECV_ACCEPT_V4/V6、RESOURCE_ASSIGNMENT_V4/V6 六个 filter key 全有；可选 Hyper-V container 的 HCS 有且十个 sandbox 值全空。不得提供补填/清空/重绑/卸载按钮；补全只由 ops receipt 在 APPROVED 时执行一次，ACTIVE 前必须完整，rollback 新 DRAFT 不复制。receipt UUID 通过 exact advisory lock+AI/MCP 跨表查重；AI receipt manifest digest 是 inner AI package manifest，MCP receipt manifest digest 是 approved connector manifest，inner MCP artifact manifest 只由 package digest/CAB 复验绑定。卸载还要求所有引用终态、零进程/句柄和不在 newest two。
- 两种 LOCAL 只允许一个非 spanning CAB，exact bytes 1..2,147,483,647；stdio extracted entries≤2,000,000,000，Hyper-V `image-layout.tar`≤2,000,000,000 且 OCI expanded≤5,368,709,120。每个 root regular-file roster 只含固定 `package.cab + exact archive entries`，安装二次读回与 plugin-host 每调用复验 package digest/closed roster/inner CMS/entry↔extracted equality。stdio 安装根必须是 manifest-version-specific 且文件对象不共享，每根只授唯一 exact SID；ServerAdmin 只显示 ref/digest/验证结论，不显示解析路径或提供 clone/share 选项。WFP provider/sublayer/filter 的 readback 必须同时证明 persistent flags、sublayer weight `0xf100`、六层 filter 的 exact SID/block/max-weight/flags，以及 owner SYSTEM、group Administrators、protected self-relative DACL、SACL absent 和 canonical 四个 allow ACE：SYSTEM/Administrators/ep-ops=`FWPM_GENERIC_ALL`、ep-plugin=`FWPM_ACTRL_READ`；UI 不提供 repair/override。Hyper-V 只在 strict OCI config/Entrypoint/ContainerUser、产品 PE/DLL Authenticode、Windows base allowlist、nested 证据和 WER/dump 零泄漏门禁全绿时显示可用；HCS image GC 结论必须来自全 connector/manifest identity 引用扫描。
- ServerAdmin 的 MCP 审计视图必须按同一事实展示 inbound、remote core/worker、local core/worker stdio/Hyper-V container 的 attempt/completion/spool/`UNKNOWN_AFTER_CRASH`；只展示 binding/manifest identity、`decoded_name_sha256` 等冻结摘要字段，绝不还原 raw tool/URI/object/header/secret/payload。core/worker 各有一个固定 1-GiB/1024×1-MiB 专用 completion spool，只显示 slot/ready/corrupt/fail-closed 结论不显示 JCS 正文/本地路径；`.reserve→.tmp→.ready`、1,048,507-byte JCS 上限、30 秒 replay 与 corrupt 取证不可由 UI 绕过/删除。cancel control 不显示为独立调用，但原 invocation completion 不得丢失。UI 不得把 `ep-worker` 的 remote/local exchange ACE 表述成通用出网或本地执行权限。
- ServerAdmin 只读显示的 MCP 事实还必须与九帧双向 exchange 一致：request/RequestBegin invocation id 相等、wire 无 wall-clock deadline，RequestEnd COMPLETE 只表示 receiver 已验证并保留 rate 且零 dispatch；caller 独立提交 deterministic ATTEMPT 后才发送无 ACK `DispatchAuthorized`，receiver 核 stream/event id 后恰 dispatch 一次。Begin/authorization 无 ACK，Chunk/End/Abort 仅对应 `CONTINUE|COMPLETE|ABORTED`，七 abort reasons 含 `RATE_LIMIT|AUDIT_UNAVAILABLE`，错态至多一个 Abort；receiver 本机单调 30 秒不可重置。ResponseEnd COMPLETE 只证明 raw terminal length/sequence/SHA-256，caller 之后才 size-first strict parse/schema/field；失败写 terminal completion且不反转 ACK、退款或重放。response 上限 8,388,608，exact SSE 固定 +23 且 decoded 上限 8,388,631。管理面不可改大这些上限，不得把超字节伪装为 schema-invalid。core/gateway/plugin-host 分别是 inbound/remote/local 唯一 rate owner，计数成功后不退还，冷启灌入 60 timestamps；UI 不得提供 reset/override。
- 首版 outbound job 必须携原始非空法人/用户/设备行/session/request identity 并在执行时重验；纯 system actor、来源缺失或失效都零 rate/attempt/dispatch。caller completion slot 后在专用 DB connection 取得 connector-keyed session shared lock并首次重读 ENABLED/ACTIVE/identity/authz/binding，RequestEnd COMPLETE 后同锁最终重读，再 ATTEMPT/authorize/dispatch/terminal/unlock；zero-DB receiver 不猜 current connector，disable/revoke/manifest switch 以同 key exclusive lock 串行。ServerAdmin 不提供把 system job 升格为 MCP、补填 actor、绕锁或重放旧来源信封的控件。
- F-55 五项权限由 `090300` 以固定 ids `...0310`–`...0314` 和 object-scope binding ids `...0504`–`...0508` 登记：`reporting.ai_analysis.compose|execute`、`platform.mcp.connector.manage|grant.issue`、`platform.admin.ai_model.view`；每项 object type 等于自身 code，不 seed 任何 role grant。ServerAdmin AI 视图只认 `platform.admin.ai_model.view` 的 `(platform_ops,ai_model_packages,...,security_level)` binding，不能借 compose/execute 或管理员客户端身份绕过。
- ServerAdmin 不提供 grant token 恢复或重放：grant token 只接受完整 50-byte ASCII `epmcp1.` + 32 CSPRNG bytes 的 43-char canonical base64url-no-pad，hash/DPoP 都覆盖完整 token；UI 不解析、持久或回显旧 token。grant issue 禁止 `Idempotency-Key` 且不入通用 response cache，拒绝必须是既有 `PLATFORM.REQUEST.INVALID_PAYLOAD` 与唯一 detail `Idempotency-Key is forbidden for this endpoint`，lost response 只能撤销/过期后重签。counter 成功后才返回的 `X-EP-MCP-Proof-Counter-Accepted` 仅透传，UI/代理/审计/日志都不得持久；入站 ATTEMPT 失败发生在 counter 后，必须显示 accepted-header fact + `MCP.AUDIT.UNAVAILABLE` + 零 dispatch。`max_calls=1` 的首个 accepted request 即使同 UPDATE 把 grant 置 CONSUMED 仍须完成本次授权/dispatch，只有后续请求失败。AI compose 的基础校验后独立 45-slot/120000-in-122000-ms gate，与 `/mcp` 的 identity 后公平 16→per-connector 4/proof 后 30000-in-32000-ms gate，都不占普通 8 秒/20-slot 业务闸门；管理 SPA 不提供同步 outbound MCP 入口，remote/local exchange 仅来自 `/mcp` route context 或 non-HTTP job/Outbox。
- ServerAdmin 对 MCP 状态只呈现权威只读事实，不提供绕过状态机的按钮：connector 只允许 `REGISTERED→PENDING_APPROVAL→DISABLED`、`DISABLED↔ENABLED`、任一非 REVOKED→REVOKED，REVOKED 终态；enable 必须在 connector-keyed exclusive lock 内重读 row version，并满足唯一 compatible ACTIVE manifest、签名/key 吊销、LOCAL 物化或 REMOTE origin/current credential probe、license/gates。grant 只允许 `ACTIVE→CONSUMED|REVOKED|EXPIRED` 且后三终态；expiry scanner、主动撤销和最后计数 UPDATE 各自只能产生对应终态。UI 不提供直接状态改写、终态恢复、无证据启用或 grant 终态互转。
- MCP credential 形状在管理面不可改写：INBOUND 必须 `None`，REMOTE 只能 `None|HTTP_AUTHORIZATION_BEARER`，stdio 只能 `None|LOCAL_SECRET_PIPE_UTF8`，Hyper-V container 首版必须 `None`。ServerAdmin 只显示非秘密 `wincred://` ref/结果，不显示 secret、stdio handle locator 或 guest bootstrap 选项。
- ServerAdmin 只显示固定路径 offline `McpManifestTrustBundleV1` 的 bundle id/digest/gate 结论；不返回 JCS/CMS/SPKI/path，不提供下载、hot reload、单文件替换或 key override。更新事实只能来自 MCP gate 关闭、gateway/plugin-host 停止、staged CAB/JCS/CMS 验证、write-through 双文件替换和重启复验。
- request `params._meta` 是 MCP request object 唯一有界 extension-map 例外，验证后忽略；success `_meta` 仍只含 serverInfo。入站 MCP error data 只有普通 `{stable_code,request_id}` 与 version-only `{stable_code,request_id,supported,requested}` 两形状。除此之外所有 F-55 strict JSON object/internally tagged enum DTO 和 OpenAPI object 均拒绝未知字段；unit enum 只接受冻结 variant/大小写，ServerAdmin 不做宽松兼容解析。
- `DeploymentCarrier` 恰好 `CustomerControlledPhysical|CustomerControlledDomesticIaasVm`；两者是客户控制的一台 Windows Server 2022，同一制品、组件、账户、pipe、ACL、资源、备份、恢复和发布门禁。
- VM 必须境内、客户自控、vTPM=true 且有 attestation；物理机 provider 固定 `CUSTOMER_CONTROLLED`、vTPM=false/ref 空；两者都必须有 carrier attestation 和备份故障域证据。
- 本节 policy/evidence/child/probe JCS 统一为无 BOM 合法 UTF-8、RFC 8785 exact bytes、每份≤1,048,576 bytes，拒绝未知字段、duplicate key 与非规范 number；时间仅 UTC 秒精度。code/jurisdiction/version/subject/key/ref、数组上限及 byte-sort/dedup 全按 F-55 §6.2 的 exact grammar，不能交给 provider 插件宽松解析。
- Carrier 唯一验证入口固定为 `validate_deployment_carrier(record, policy, evidence, facts)`；`CarrierFactProbe` 唯一方法是 `collect(stage14_run_id, deployment_id) -> CarrierFactProbeResultV1`，其 strict typed result 是当前现场事实的唯一来源。验证同时覆盖策略/证据签名与 digest/ref、Stage 14 run/deployment/policy 绑定、全部 child parser/preimage、部署记录十四列、legacy/current guard、机器/provider/region/SKU/TPM/vTPM/nested/Hyper-V、七个托管组件 false 和三维备份隔离，不能只做数据库枚举或信任 evidence 自报值。
- `CarrierPolicyV1.allowed_iaas_regions[]` 固定 `min_tpm_version="2.0",vtpm_attestation_required=true,vtpm_attestation_profile="TPM2_QUOTE_SHA256_V1"`，另含 1..8 个 byte-sort/dedup 的 `vtpm_ak_trust_anchor_spki_sha256[]` 64-lowerhex；`approved_vm_skus[]` 每项只含 `vm_sku,nested_virtualization_supported` 并按 SKU bytes 排序去重。物理机与 VM 的 `tpm_version` 首版都必须为 `2.0`；只有 flag=true 的批准 SKU 可使 Hyper-V MCP gate 变绿，false 仍支持 stdio。
- `CarrierEvidenceV1` 必带 UUIDv7 `stage14_run_id`、UTC 秒级 start/completed/verified，`started < completed`、窗口≤8h、`verified_at=completed_at`，全部 child 使用相同 run id 且 observed_at 落在闭区间。VM vTPM 只接受 exact `VtpmAttestationEvidenceV1`/`TPM2_QUOTE_SHA256_V1` 的 canonical TPM2B、challenge/PCR quote/event-log/measured-boot 校验；物理机 vTPM 字段全空/false，但物理 TPM 2.0 原始证据必须由同一次 Stage 14 server-spec 留存。两 carrier 都必须验证 exact `CustomerControlEvidenceV1`，客户 OS-admin/备份凭据/KMS-HSM control 三项 true、vendor interactive/remote-support 两项 false。
- nested=true 时必须附带相同 run 的 exact `NestedVirtualizationEvidenceV1`，并同时核 policy flag、CarrierEvidence 三字段/report digest 与当前 hypervisor/isolation probe；nested=false 时 ref/digest 为空且只关闭 Hyper-V transport。所有 carrier ref 只允许 `ep-evidence://carrier/<deployment-uuid>/<kind>/sha256/<digest>` 不透明形状；`vtpm|nested-virtualization|backup-failure-domain|customer-control` 都是 exact `.jcs` child，无 provider binary/自由后缀/插件解析分支，DB/API/审计不暴露解析路径。
- `CarrierEvidenceV1.authorizations` 恰为按 role bytes 排序的 `SECURITY|OPERATIONS` 两项；两名 subject、key ref 与批准职责必须互异。每项及 `CarrierEvidenceSignatureV1` 的签名字段都恰为 `signature_p1363_b64url: String`：RFC 4648 §5 canonical base64url-no-pad，解码恰 64-byte P-256 low-S P1363 且重编码逐 byte 相等，integer array/lowerhex/DER/padding 均拒绝。`BackupFailureDomainEvidenceV1` 必含同 run 的三个 strict `BackupDimensionProbeEvidenceV1`，每项逐维验证 domain-separated digest、不同 production/backup domain、独立 write identity、生产身份不可删除备份、实际 restore probe 与 dimension→mechanism 闭集；bundle digest/ref 和其 `{dimension,evidence_digest}` 投影必须与 CarrierEvidence 的 `backup_failure_domain_evidence_digest`/`backup_separation_evidence` 逐项闭合。完整 evidence 再由部署 KMS purpose `CARRIER_EVIDENCE_V1` 生成 strict sidecar；一人双签、同 key 双签、角色/purpose/preimage/签名编码/吊销或任一 digest/ref/root/DACL 不符均拒绝。
- 同一 VM 目录、同一虚拟磁盘、同一在线管理员可覆盖的快照不能当离站备份；provider snapshot 不能成为唯一备份。
- SaaS、多客户共享、Kubernetes、整平台容器、HA/读副本、托管 PostgreSQL/KMS/队列/函数、厂商遥测/回传和自动在线更新全部拒绝。
- Carrier 稳定错误集合恰好使用 `OPS.DEPLOYMENT.CARRIER_NOT_ALLOWED`、`OPS.DEPLOYMENT.REGION_NOT_DOMESTIC`、`OPS.DEPLOYMENT.VTPM_EVIDENCE_MISSING`、`OPS.BACKUP.FAILURE_DOMAIN_NOT_SEPARATE`，category/HTTP/retryable 逐项照抄 F-55 §8。
- 本线自有发布 gate 固定 `RG-SERVER-ADMIN-MATRIX-90-GREEN` 与 `RG-DEPLOYMENT-CARRIER-EVIDENCE-GREEN`；F-56 `RG-LICENSE-MODULE-LIFECYCLE-GREEN` 是 AI/MCP applicability 的共同独立前置，并必须与适用的 F-55 gate 绑定同一 exact `stage14_run_id/deployment_id/product_build_sha256`。ServerAdmin 只显示该签名结论，不复制 evidence parser/DTO 或提供 override。
- ServerAdmin 的 AI 管理面只按 F-55 §5.3 查看不可变模型包身份、状态和认证报告引用；不得返回安装路径、提示词、tokenizer/weights、签名正文、secret 或下载链接，不得提供 upload/install/activate/disable/revoke/action API，也不得触发 hub/依赖下载。F-55 AI/MCP 可调用性只来自 F-56 `ModuleLicenseQuery`：AI=`F55LocalAi`，双向 MCP=`F55Mcp`，`Active|ExpiringSoon|GracePeriod` 可用、`Restricted` 不可用；UI 不能以 purchased 历史、feature flag、模块码、配置或人工证据覆盖 currently licensed 结论。普通业务被通用 license gate 拒绝时稳定返回 F-56 `PLATFORM.LICENSE.RESTRICTED=BUSINESS_CONFLICT/409/false`；ServerAdmin 只显示该脱敏结论，AI/MCP 调用则继续按各自 route-specific 错误闭集呈现，前端不得把不同边界改写成一个通用响应。
- AI 认证引用仅允许 `ep-evidence://ai-resource/<release-batch>/<model-package>/<cpu-local|gpu-local>/sha256/<report-digest>`；core 必须验最大 1,048,576-byte strict `AiResourceCertificationReportV1` 与 strict `AiResourceCertificationSignatureV1`。sidecar 字段 `signature_p1363_b64url: String` 只接受 canonical base64url-no-pad、解码恰 64-byte P-256 low-S P1363 且 byte-equal round-trip，拒绝 integer array/lowerhex/DER/padding；同时验 purpose `AI_RESOURCE_CERTIFICATION_V1`、release/model/build/server/load/gate 绑定和证据根 owner/DACL。ServerAdmin 只返回 ref/digest/状态，不返回解析路径、report/sidecar 正文或签名。
- ServerAdmin 的 AI read model 只显示最终 signed report 对 mandatory `ai_runtime_release_facts` gate 的结论/digest；`AiRuntimeReleaseFactsV1` 本身只在 gate 内存重算，无 ref/独立签名/DB 行/持久 JSON。AI 包只允许单 CAB≤2,147,483,647、`model.gguf`≤2,000,000,000 和 fixed `package.cab+7 entries` root；UI 不提供大包豁免、spanning、文件下载或安装操作。AI compose 已运行任务在取消后 2,000 ms 未确认终止时，首版不杀不存在的 per-invocation 子进程，而是关闭 readiness、终止整个 `ai-inferer`/`APP_AI` Job，使全部同进程在途请求取消/清零/NOT_ACTIVE；重启并经独立包复验、fresh activate ACK 与当前认证 gate 后才重开。UI 只显示这一脱敏 health 结论，不显示请求或结果。

---

## File Map

| 单元 | 文件 | 责任 |
|---|---|---|
| Client identity | `crates/foundation/src/security/context.rs` | 八值 `ClientKind` 与序列化 |
| Matrix | `crates/platform/meta/src/client_capability.rs` | 90-cell frozen snapshot/hash and decision |
| SPA | `clients/server-admin/` | 独立路由树、受权管理视图、无业务写 |
| Embed | `apps/core-server/{build.rs,src/server_admin.rs}` | 构建期资源嵌入与 `/server-admin/` serving |
| Carrier | `crates/platform/obs/src/deployment_carrier.rs` | 两值 enum、事实验证、证据形状 |
| 数据 | `090400`–`090700` migrations | matrix/client/audit/carrier persistent shape |
| 验收 | `tests/server_admin/`、`tests/deployment_carrier/` | matrix/static SPA/carrier release gates |

### Task 1: Expand ClientKind and persistent client checks atomically

**Files:**
- Modify: `crates/foundation/src/security/context.rs`
- Modify: `xtask/src/archcheck/frozen.rs`
- Create: `db/migrations/platform_core/V20261024090500__platform_core_add_server_admin_client_kind.sql`
- Create: `db/migrations/platform_audit/V20261024090600__platform_audit_add_server_admin_and_mcp_clients.sql`
- Create: `db/checks/27_f55_client_kinds.sql`
- Test: `crates/foundation/tests/f55_client_kind.rs`
- Test: `testkit/tests/f55_client_kind_schema_pg.rs`
- Modify: `docs/migration-catalog.md`

**Interfaces:**
- Consumes: existing six-value `ClientKind`, `user_devices.client` and `audit_events.client`.
- Produces:

```rust
pub enum ClientKind {
    Win, Mac, Ios, Android, Portal, Ops, ServerAdmin, Mcp,
}

pub const HUMAN_OR_PROTOCOL_CLIENTS: [ClientKind; 8] = [
    ClientKind::Win, ClientKind::Mac, ClientKind::Ios, ClientKind::Android,
    ClientKind::Portal, ClientKind::Ops, ClientKind::ServerAdmin, ClientKind::Mcp,
];
```

- [ ] **Step 1: Write failing serialization tests.** Assert the exact eight ordered serialized strings, round trips, unknown-value rejection, `server_admin` accepted from authenticated HTTP context and external `X-Client:mcp` rejected before context creation.
- [ ] **Step 2: Write failing schema tests.** Assert `user_devices.client` accepts the existing six plus only `server_admin`, rejects `mcp`; audit accepts those seven plus `mcp|system`, and rejects all aliases/case variants.
- [ ] **Step 3: Run focused tests.** Run: `cargo test -p ep-foundation --test f55_client_kind && cargo test -p ep-testkit --test f55_client_kind_schema_pg`. Expected: FAIL because new values/checks are absent.
- [ ] **Step 4: Extend the enum and frozen count.** Add the two variants and explicit serde names; update every exhaustive match and metric-client label encoder; `system` remains audit-only and not a `ClientKind`.
- [ ] **Step 5: Implement `090500` and `090600`.** `090500` changes only persistent device/client checks to add `server_admin`; `090600` changes audit client check to the nine-value terminal set. Neither migration rewrites historical rows.
- [ ] **Step 6: Verify atomically.** Run: `cargo test -p ep-foundation --test f55_client_kind && cargo test -p ep-testkit --test f55_client_kind_schema_pg && cargo xtask archcheck`. Expected: PASS with frozen count 8 and no path that persists `mcp` to `user_devices`.
- [ ] **Step 7: Stage for the master atomic schema commit.** When invoked by 13c, do not run a ClientKind-only commit. Stage the listed files only as this task's contribution to 13c Task 3 Step 4; the one joint `feat(f55): land atomic schema and shared identities` commit completes this checkbox.

### Task 2: Add the exact 18-row ServerAdmin capability column and frozen hash

**Files:**
- Create: `db/migrations/platform_meta/V20261024090400__platform_meta_add_server_admin_capability_rows.sql`
- Create: `db/checks/28_f55_server_admin_matrix.sql`
- Create: `crates/platform/meta/src/client_capability.rs`
- Modify: `crates/platform/meta/src/lib.rs`
- Modify: `apps/core-server/src/platform/middleware.rs`
- Test: `testkit/tests/f55_server_admin_matrix_pg.rs`
- Test: `crates/platform/meta/tests/f55_matrix.rs`
- Modify: `docs/migration-catalog.md`

**Interfaces:**
- Consumes: the existing 72-row four-client capability matrix and `CapabilityDomain::ALL` in its fixed 18-item order.
- Produces `SERVER_ADMIN_CAPABILITIES: [(CapabilityDomain, CapabilityValue, &'static str); 18]`, a 90-cell canonical snapshot and its SHA-256.

```rust
pub const SERVER_ADMIN_FULL: [CapabilityDomain; 2] = [
    CapabilityDomain::PlatformAdminLowcodeOps,
    CapabilityDomain::PlatformExtensionDynamicCode,
];
pub const SERVER_ADMIN_VIEW_ONLY: [CapabilityDomain; 3] = [
    CapabilityDomain::PlatformApprovalNotify,
    CapabilityDomain::PlatformFullTextSearch,
    CapabilityDomain::ReportingReportPrint,
];
```

- [ ] **Step 1: Write the failing 18-cell truth table.** Enumerate all 18 domains in order; assert the two FULL, three VIEW_ONLY and thirteen NOT_APPLICABLE values. Assert ViewOnly alternative `desktop://same-object/write`; business N/A rows 1–6, 8–9, 11–13, 15 use `desktop://capability-domain`; row 18 uses `portal://supplier-web`.
- [ ] **Step 2: Write database shape tests.** Assert exactly 90 total rows, exactly 18 per `win|mac|ios|android|server_admin`, zero rows for `mcp`, no duplicate `(domain,client)` and database canonical bytes equal the embedded snapshot/hash.
- [ ] **Step 3: Run tests.** Run: `cargo test -p ep-platform-meta --test f55_matrix && cargo test -p ep-testkit --test f55_server_admin_matrix_pg`. Expected: FAIL with 72 rows.
- [ ] **Step 4: Implement `090400`.** Insert the 18 literal rows and no SQL column; update table client CHECK to five matrix clients only. Preserve all 72 prior cells byte-for-byte.
- [ ] **Step 5: Implement runtime decision.** `FULL` follows normal authz; `VIEW_ONLY` permits only `ActionClass::Read`; N/A rejects before object lookup and returns the existing capability-denied envelope. Database/hash mismatch keeps the embedded snapshot authoritative, rejects matrix writes and emits the existing high-priority alert.
- [ ] **Step 6: Verify exact counts and hash.** Run: `cargo test -p ep-platform-meta --test f55_matrix && cargo test -p ep-testkit --test f55_server_admin_matrix_pg`. Expected: PASS with `18×5=90`, `2/3/13`, and zero MCP cells.
- [ ] **Step 7: Stage for the master atomic schema commit.** When invoked by 13c, do not run a matrix-only commit. Stage the listed files only as this task's contribution to 13c Task 3 Step 4; the one joint `feat(f55): land atomic schema and shared identities` commit completes this checkbox.

### Task 3: Build and embed the independent ServerAdmin static SPA

**Files:**
- Reference: `docs/superpowers/plans/2026-08-22-license-module-package-implementation.md`
- Create: `clients/server-admin/package.json`
- Create: `clients/server-admin/tsconfig.json`
- Create: `clients/server-admin/vite.config.ts`
- Create: `clients/server-admin/index.html`
- Create: `clients/server-admin/src/main.tsx`
- Create: `clients/server-admin/src/routes.tsx`
- Create: `clients/server-admin/src/api.ts`
- Create: `clients/server-admin/src/guards.tsx`
- Create: `clients/server-admin/src/views/overview.tsx`
- Create: `clients/server-admin/src/views/identity.tsx`
- Create: `clients/server-admin/src/views/packages.tsx`
- Create: `clients/server-admin/src/views/extensions.tsx`
- Create: `clients/server-admin/src/views/mcp.tsx`
- Create: `clients/server-admin/src/views/ai.tsx`
- Create: `clients/server-admin/src/views/evidence.tsx`
- Create: `apps/core-server/build.rs`
- Create: `apps/core-server/src/server_admin.rs`
- Create: `apps/core-server/src/platform/ai_model_packages.rs`
- Modify: `apps/core-server/src/main.rs`
- Modify: `apps/core-server/Cargo.toml`
- Create: `docs/openapi/ai-admin.v1.yaml`
- Modify: `xtask/Cargo.toml`
- Create: `xtask/tests/f55_ai_admin_openapi.rs`
- Test: `apps/core-server/tests/f55_server_admin_assets.rs`
- Test: `apps/core-server/tests/f55_ai_model_package_view.rs`
- Test: `clients/server-admin/src/routes.test.tsx`
- Test: `clients/server-admin/src/views/ai.test.tsx`

**Interfaces:**
- Consumes: shared `clients/ui` components, employee auth/session/bootstrap, F-56 multipart config-package import and `license_module_admin` bootstrap contract, existing platform admin APIs, MCP management APIs and read-only AI model endpoints.
- Produces: one immutable asset manifest embedded in `core-server`, `/server-admin/` HTML fallback and hashed JS/CSS under `/server-admin/assets/`.

```ts
export const SERVER_ADMIN_ROUTES = [
  "/", "/identity", "/packages", "/extensions", "/mcp", "/ai", "/evidence"
] as const;
```

```rust
pub struct AiModelPackageAdminViewV1 {
    pub id: Uuid,
    pub model_code: String,
    pub model_version: String,
    pub runtime_abi_version: u16,
    pub package_digest: Sha256Digest,
    pub manifest_digest: Sha256Digest,
    pub signer_subject: String,
    pub signature_kind: AiModelSignatureKind,
    pub prompt_template_version: String,
    pub max_context_tokens: u32,
    pub max_concurrent_requests: u16,
    pub execution_profile: AiExecutionProfileV1,
    pub resource_formula_version: String,
    pub certification_report_ref: Option<String>,
    pub certification_report_digest: Option<Sha256Digest>,
    pub verified_at: Option<DateTime<Utc>>,
    pub certified_at: Option<DateTime<Utc>>,
    pub activated_at: Option<DateTime<Utc>>,
    pub disabled_at: Option<DateTime<Utc>>,
    pub revoked_at: Option<DateTime<Utc>>,
    pub status: AiModelPackageStatus,
    pub row_version: i64,
}
```

The `/packages` route is the only F-56 ServerAdmin wizard. Its failing/positive tests must prove the strict one lower-case-`.epcfg` file-part multipart boundary, required decimal `Content-Length=1..4,194,304`, missing/invalid/zero/overflow/chunked pre-body rejection, streaming short/long rejection, unique route-local 4 MiB exception with every other route still 1 MiB, staging ACL/CREATE_NEW/finally-delete boundary, imported immutable special-item summary, autotest→submit→read-only approval conclusion→sign→release sequence, and the exact permission-gated `license_module_admin` bootstrap projection. It must prove zero attachment creation, zero `approve|reject` call, zero direct state toggle, zero signature/path/secret disclosure and zero new backend route; Win/Mac JSON attachment import and approval completion remain unchanged. Bootstrap fixtures additionally prove no-current/signature-invalid responses omit every untrusted license identity/date/code/limit while retaining the three actual usage counts.

- [ ] **Step 1: Write route, asset, permission and AI-view failures.** Assert exactly the seven routes, no business-module route, no approval conclude action, base path `/server-admin/`, content-hashed assets, immutable cache for hashes, no-cache HTML, CSP/CSRF headers and no filesystem fallback. Assert permission id `...0314`, binding id `...0508`, code/object type `platform.admin.ai_model.view`, VIEW, `(platform_ops,ai_model_packages,...,security_level)` and zero default role grants. `f55_ai_model_package_view.rs` asserts the two GET routes return exactly `AiModelPackageAdminViewV1`, serialize all present digests as lowercase 64-hex, accept only optional `cursor,page_size`, reject `limit/page/sort/filter`, default page_size 50/max 100, order `created_at DESC,id DESC`, and enforce cursor `epcur1.<base64url-no-pad(JCS(AiModelPackageCursorV1))>` with strict `schema_version=1,endpoint="AI_MODEL_PACKAGES",created_at,id`, UTC microseconds/lowercase UUID, decoded≤512, canonical round-trip and keyset `(created_at,id)<(...)`, and make nonexistent/unauthorized detail indistinguishable 404. Response envelopes are exactly `data={items:[...]},meta={page_size,next_cursor}`; next cursor exists only when more rows, otherwise NULL. Fixtures assert the six-status/five-time truth table, immutable verified time, retry-round certified/activated/disabled behavior, REVOKED retained shape and certification ref/digest nullability. A present ref/digest pair must match the exact AI-resource opaque grammar and verified strict report/sidecar: max JCS bytes, release/model/profile/digest binding, CPU/GPU conditionals, exact PASSED gate registry, purpose/preimage/key-state plus canonical `signature_p1363_b64url` decoded exact low-S raw64, with array/hex/DER/padding negatives and fixed-root owner/DACL are all negative fixtures. POST/PUT/PATCH/DELETE, upload/install/activate/disable/revoke/download/action, `installed_root_ref`, `install_receipt_id`, resolved evidence path, report/sidecar body, prompt/model/tokenizer bytes, signature body, secrets and any field outside the frozen list are absent. `f55_ai_admin_openapi.rs` proves these routes live only in independent `docs/openapi/ai-admin.v1.yaml`, not in reporting or MCP OpenAPI.
- [ ] **Step 2: Run unit and HTTP tests.** Run: `npm --prefix clients/server-admin test -- --run && cargo test -p core-server --test f55_server_admin_assets && cargo test -p core-server --test f55_ai_model_package_view`. Expected: FAIL because the client/assets and view endpoints are absent.
- [ ] **Step 3: Create the independent SPA.** Use shared visual components only; create its own entry/route tree/api client. Every API request sends `X-Client: server_admin` through the existing authenticated client and never accepts server/account/role/legal-entity overrides from URL state. The packages client consumes only F-56's existing import/autotest/submit/sign/release and bootstrap shapes; its API type has no approve/reject/direct-enable/direct-disable method.
- [ ] **Step 4: Implement build-time embedding.** Vite emits to Cargo `OUT_DIR`; `build.rs` validates filenames/digests and generates a Rust static table. `server_admin.rs` serves only table entries and SPA HTML fallback; runtime cannot read or modify a static directory and does not invoke Node.
- [ ] **Step 5: Wire existing employee security and the F-56 read/import surfaces.** Reuse login/MFA/session/device/CSRF/CSP/bootstrap middleware; unauthenticated requests redirect through the existing login flow; N/A routes are absent, ViewOnly controls have no write buttons and backend denial remains authoritative. For `client=server_admin`, render `license_module_admin` only when nonnull, accept its no-current/signature-invalid redacted projection without inventing identity/date/code/limit, retain the three supplied usage counts, and treat unknown/extra fields as contract failure. Send `.epcfg` only as the exact one-file-part multipart request with the required length/non-chunked boundary; never convert it into an attachment, base64 JSON or filesystem path. Approval cards stay read-only and link the user to Win/Mac for task completion.
- [ ] **Step 6: Add and independently document read-only AI model endpoints.** Add `GET /api/v1/platform/ai-model-packages` and `GET /api/v1/platform/ai-model-packages/{id}` in `apps/core-server/src/platform/ai_model_packages.rs`, returning exactly `AiModelPackageAdminViewV1` from the immutable registry. When certification ref/digest is present, resolve only the compile-time AI-resource evidence root; verify opaque grammar, SYSTEM/no-inheritance DACL, exact report JCS digest/size/fields/current facts and sidecar purpose/preimage/P-256 key/version/subject/revocation before returning only the ref/digest. Both routes require `platform.admin.ai_model.view` and `PlatformAdminLowcodeOps + Read`; list accepts only optional `cursor,page_size`; rejects `limit,page,sort,filter`; defaults page_size 50/max 100; orders `created_at DESC,id DESC`; and validates exact `epcur1.<base64url-no-pad(JCS(AiModelPackageCursorV1))>` strict cursor with endpoint literal, UTC-microsecond created_at, lowercase UUID, decoded≤512, canonical re-encode and `(created_at,id)<(...)` keyset. Reject an incomplete ref/digest pair or malformed/failed evidence as `AI.MODEL_PACKAGE.SIGNATURE_INVALID`, never show a partial item, and unify absent/denied detail as 404. Register no write/download/action route. Return exactly `data.items` and `meta.page_size,next_cursor`, where next cursor is present only if more rows and otherwise NULL. Freeze every nested query/response/error object with `additionalProperties:false` in independent `docs/openapi/ai-admin.v1.yaml`; the xtask test compares both operations and every DTO field to Rust and rejects any third path or any placement in `ai-reporting.v1.yaml`.
- [ ] **Step 7: Verify process/port containment.** A packaged install starts the existing product services only; process and listener snapshots before/after opening ServerAdmin are identical. Search release artifacts for `node.exe`, dev-server scripts and writable static roots; all counts are zero.
- [ ] **Step 8: Run tests.** Run: `npm --prefix clients/server-admin test -- --run && npm --prefix clients/server-admin run build && cargo test -p core-server --test f55_server_admin_assets && cargo test -p core-server --test f55_ai_model_package_view && cargo test -p ep-xtask --test f55_ai_admin_openapi`. Expected: PASS and deterministic asset manifest hashes across two builds with fixed `SOURCE_DATE_EPOCH`.
- [ ] **Step 9: Commit.** Run: `git add clients/server-admin apps/core-server/build.rs apps/core-server/src/server_admin.rs apps/core-server/src/platform/ai_model_packages.rs apps/core-server/src/main.rs apps/core-server/Cargo.toml apps/core-server/tests/f55_server_admin_assets.rs apps/core-server/tests/f55_ai_model_package_view.rs docs/openapi/ai-admin.v1.yaml xtask/Cargo.toml xtask/tests/f55_ai_admin_openapi.rs && git commit -m "feat(client): embed independent ServerAdmin SPA"`.

### Task 4: Enforce the ServerAdmin feature surface end to end

**Files:**
- Create: `tests/server_admin/package.json`
- Create: `tests/server_admin/playwright.config.ts`
- Create: `tests/server_admin/matrix.spec.ts`
- Create: `tests/server_admin/admin_views.spec.ts`
- Create: `tests/server_admin/forbidden_business.spec.ts`
- Create: `tests/server_admin/process_surface.spec.ts`
- Modify: `xtask/src/ci.rs`
- Modify: `tools/release-gate/src/main.rs`

**Interfaces:**
- Consumes: embedded SPA, capability matrix, F-56 package/import/bootstrap surfaces, existing admin endpoints, MCP endpoints and AI model read endpoints.
- Produces: browser-level evidence for `RG-SERVER-ADMIN-MATRIX-90-GREEN`.

F-56 browser coverage is exact, not illustrative: test `PERPETUAL|SUBSCRIPTION` and `Active|ExpiringSoon|GracePeriod|Restricted`, current/history/purchased versus currently licensed, three over-limit warnings, and all 15 module rows. Drive only the five legal module actions through new signed single-item config packages and prove `NOT_INSTALLED→INSTALL→INSTALLED_DISABLED→ENABLE→INSTALLED_ENABLED→DISABLE→INSTALLED_DISABLED`, higher-version UPGRADE and separately approved historical `ROLLBACK_VERSION`; reject uninstall, enable-state upgrade, direct SQL/toggle and generic config rollback. Under `Restricted` or no-current fixtures, prove only the exact `LICENSE_GRANT` recovery chain and `MODULE_PACKAGE:DISABLE` chain remain reachable, every ordinary business write/approval/egress/new-automation and every other module/config action returns `PLATFORM.LICENSE.RESTRICTED`, and approval completion still occurs only on Win/Mac. During DISABLE, exercise the F-56 module-keyed shared/exclusive advisory gate, 30-second drain timeout `PLATFORM.MODULE.IN_FLIGHT_DRAIN_TIMEOUT=INFRASTRUCTURE/503/true`, queued-request recheck and preservation of UI metadata/business rows/audit/config evidence. ServerAdmin only observes/submits these packages and never concludes approval.

- [ ] **Step 1: Write the browser truth table.** Seed users with/without underlying permissions. Verify system config, identity, module/package, extension/MCP, AI evidence, audit/health/backup/recovery/migration evidence are visible only when authorized; approval queue/search/reporting are read-only. The MCP page consumes only the five `X-Client=server_admin` connector management operations, exact cursor/page_size envelopes and redacted connector/detail/manifest DTOs; it rejects unknown pagination/fields and never attempts grant-token recovery. State fixtures render only `REGISTERED→PENDING_APPROVAL→DISABLED`、`DISABLED↔ENABLED`、non-REVOKED→REVOKED and terminal grant states; enable failure caused by missing compatible ACTIVE manifest/signature/key/materialization-or-probe/license/gate stays DISABLED and exposes no bypass control. MCP evidence renders separate core/worker spool capacity/ready/corrupt/fail-closed conclusions, core+worker remote/local caller identities, fixed single-CAB/root-roster equality, per-version stdio-root verification, exact WFP provider/sublayer/six-filter/security-descriptor readback and Hyper-V OCI/WER/nested gate conclusion, but no spool record/path, package path, raw invocation name/URI or action that clears corrupt evidence. It exposes only manifest trust bundle id/digest/gate, shows `MCP.AUDIT.UNAVAILABLE` plus accepted-counter/zero-dispatch when ATTEMPT fails after counter, and represents the nine-frame RequestEnd COMPLETE→caller ATTEMPT→no-ACK DispatchAuthorized boundary without a manual authorize/replay control. The reporting-analysis fixture sends `X-Client:server_admin`, succeeds only with ReportingReportPrint+Read plus the exact compose/execute permission/object scope and AI license, and rejects portal/mcp or relying only on `platform.admin.ai_model.view`. AI evidence shows only the fixed single-CAB/root-roster and final signed report gate conclusion/digest, never the in-memory runtime facts body; a forced running compose disconnect first shows cooperative cancellation; a missed 2-second ACK shows whole `ai-inferer`/`APP_AI` Job termination, every same-process in-flight request cancelled/zeroized/NOT_ACTIVE, readiness closed through restart, and fresh package verification/activate ACK/certification gate before reopening, with no per-invocation process, old ACK or late result. A pure-system outbound job fixture shows rejected-before-rate/attempt/dispatch and offers no actor override. Disable and re-enable a signed module and assert its metadata and business rows are unchanged, with re-enable limited to the same signed version or a separately installed signed upgrade.
- [ ] **Step 2: Write forbidden-surface cases.** Direct navigation and crafted API requests for CRM/contract/order/procurement/inventory/project/MDM/finance/invoice/attachment/portal writes, every one of the seven `HighRiskOperation` values, explicit `DATA_MIGRATION` execution/replay, ViewOnly write, approval conclude, self-declared super-admin and RLS/SoD bypass all fail with the existing stable denial; migration evidence remains read-only.
- [ ] **Step 3: Run the browser tests.** Run: `npm --prefix tests/server_admin test`. Expected: FAIL until route/permission fixtures are complete.
- [ ] **Step 4: Complete UI guards without trusting them as security.** Route loader uses bootstrap matrix to hide N/A and render ViewOnly labels; every backend route still applies normal authz and capability middleware. Keep the model-package management subview read-only and gated by `platform.admin.ai_model.view`, with no upload/install/activate/disable/revoke/download controls. The F-56 packages subview accepts only exact multipart import and existing autotest/submit/sign/release actions, renders `license_module_admin` only from the permission-gated bootstrap, and has no approve/reject/direct lifecycle/attachment/download control. Separately, the read-only reporting-analysis surface may call compose/execute with `X-Client:server_admin` only when F-56 `F55LocalAi` is currently licensed (`Active|ExpiringSoon|GracePeriod`), `ReportingReportPrint + Read`, the exact route permission/object scope and all current security facts pass; purchased history、feature flag、client identity or model-view permission alone shows no compose/execute control. Browser/API negatives reject portal/mcp and a stale five-value reporting client enum.
- [ ] **Step 5: Implement the 90-cell gate.** Add/extend `ep-release-gate verify --gate <name> --evidence-dir <path>`. The gate hashes canonical database rows and embedded snapshot, checks all 18 ServerAdmin cells, client/audit/metric counts, zero MCP rows and zero extra process/listener/writable assets.
- [ ] **Step 6: Verify.** Run: `npm --prefix tests/server_admin test && cargo run -p ep-release-gate -- verify --gate RG-SERVER-ADMIN-MATRIX-90-GREEN --evidence-dir target/release-evidence`. Expected: PASS with exactly `FULL=2`, `VIEW_ONLY=3`, `NOT_APPLICABLE=13` for ServerAdmin.
- [ ] **Step 7: Commit.** Run: `git add tests/server_admin xtask/src/ci.rs tools/release-gate/src/main.rs && git commit -m "test(client): close ServerAdmin matrix gate"`.

### Task 5: Persist and validate the two deployment carriers

**Files:**
- Create: `crates/platform/obs/src/deployment_carrier.rs`
- Modify: `crates/platform/obs/src/lib.rs`
- Create: `db/migrations/platform_ops/V20261024090700__platform_ops_add_deployment_carrier.sql`
- Create: `db/checks/29_f55_deployment_carrier.sql`
- Test: `crates/platform/obs/tests/f55_deployment_carrier.rs`
- Test: `testkit/tests/f55_deployment_carrier_schema_pg.rs`
- Modify: `docs/migration-catalog.md`

**Interfaces:**
- Consumes: Stage 14a0 唯一拥有并已冻结的 `platform_ops.deployment_records`、`DeploymentRecord` 当前版本模型/仓储/选择器、服务器规格与备份 evidence contract/fixture；不消费 Stage 14a1 的 F-56 query/adapter，也不消费 Stage 14b 尚未产生的真实认证结论。
- Produces:

```rust
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum DeploymentCarrier {
    CustomerControlledPhysical,
    CustomerControlledDomesticIaasVm,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CarrierFactProbeResultV1 {
    pub schema_version: u16,
    pub stage14_run_id: Uuid,
    pub deployment_id: Uuid,
    pub carrier_kind: DeploymentCarrier,
    pub machine_sid_digest: Sha256Digest,
    pub site_code: Option<String>,
    pub provider_code: String,
    pub region_code: String,
    pub vm_sku: Option<String>,
    pub tpm_version: String,
    pub vtpm_present: bool,
    pub vtpm_attestation_digest: Option<Sha256Digest>,
    pub nested_virtualization_supported: bool,
    pub windows_hypervisor_present: bool,
    pub hyperv_isolation_probe_passed: bool,
    pub customer_control_attestation_digest: Sha256Digest,
    pub managed_components: CarrierManagedComponentsV1,
    pub backup_failure_domain_code: String,
    pub backup_failure_domain_evidence_digest: Sha256Digest,
    pub observed_at: DateTime<Utc>,
    pub probe_build_digest: Sha256Digest,
}

pub trait CarrierFactProbe {
    fn collect(
        &self,
        stage14_run_id: Uuid,
        deployment_id: Uuid,
    ) -> Result<CarrierFactProbeResultV1, AppError>;
}

pub fn validate_deployment_carrier(
    record: &DeploymentRecord,
    policy: &CarrierPolicyV1,
    evidence: &CarrierEvidenceV1,
    facts: &dyn CarrierFactProbe,
) -> Result<(), AppError>;
```

- [ ] **Step 1: Write failing fourteen-column schema truth table.** Physical positive requires provider `CUSTOMER_CONTROLLED`, site region, matching residency/region jurisdiction, vTPM false/ref null, nonempty failure-domain code/evidence and carrier attestation, plus nonempty carrier-policy/evidence refs and 32-byte digests; its signed server-spec and current probe still prove physical TPM `2.0` in the same Stage 14 run. VM positive requires approved provider/region, matching jurisdiction, TPM `2.0`, vTPM true/ref nonempty and the same failure-domain/attestation/policy/evidence fields. Assert all fourteen columns are nullable with no default, each post-upgrade INSERT must be complete, and a legacy all-null row may only receive its immutable-guard `superseded_at` update; it cannot be partially backfilled, selected current or pass a release gate. Resolve the persisted `backup_failure_domain_evidence_ref` only as kind `backup-failure-domain`; its terminal digest must equal the exact `BackupFailureDomainEvidenceV1` JCS SHA-256 and `CarrierEvidenceV1.backup_failure_domain_evidence_digest`, without adding a fifteenth deployment column.
- [ ] **Step 2: Add explicit negatives.** Reject a third carrier, foreign region, missing provider/region/control attestation, either carrier without TPM `2.0`, VM without vTPM, physical with any vTPM field, missing failure-domain evidence, any of the seven managed-component flags, provider snapshot as sole backup, policy/evidence signature or digest/ref mismatch, deployment/policy/run binding mismatch, invalid UUIDv7 run id, non-second UTC time, start≥complete, window>8h, `verified_at!=completed_at`, child run/observed-time mismatch, and any current `CarrierFactProbeResultV1` fact that differs from signed record/evidence/children. Apply the shared 1,048,576-byte/JCS/code/ref/array/duplicate-key constraints to policy, evidence, every child and probe result. VM negatives cover policy vTPM profile drift, malformed/noncanonical TPM2B/base64url, challenge/PCR/quote/event-log/measured/Secure Boot mismatch, absent/duplicate/unsorted/unapproved `vm_sku`, nested conditional mismatch, wrong run/deployment/provider/region/SKU in `NestedVirtualizationEvidenceV1`, false hypervisor/isolation probe or wrong signed probe digest. Customer-control negatives flip any required customer/vendor boolean, identity/managed-component/digest/run binding or save raw account/SID. Backup negatives cover a missing/oversize/unknown-field bundle, wrong run/deployment/carrier/failure-domain binding, any dimension set/order/digest/probe-object error, same production/backup domain, shared write identity, production-deletable backup, invalid dimension→mechanism mapping, stale restore probe, bundle digest/ref mismatch, or a bundle projection that differs from `CarrierEvidenceV1.backup_separation_evidence`. Reject wrong/missing `CarrierEvidenceSignatureV1`, purpose/preimage/key scope/version/revocation/signature encoding, every carrier ref whose scheme/deployment/kind/digest shape is not the frozen opaque `ep-evidence://carrier/...` form, a child not stored as exact `.jcs`, and every resolved evidence file whose owner/DACL/reparse/hardlink/ADS/ref/hash readback fails.
- [ ] **Step 3: Run tests.** Run: `cargo test -p ep-platform-obs --test f55_deployment_carrier && cargo test -p ep-testkit --test f55_deployment_carrier_schema_pg`. Expected: FAIL because enum/columns are absent.
- [ ] **Step 4: Implement `090700` only after the Stage 14a0 migration freeze and global pre-F55 admission.** Add exactly the fourteen F-55 §6.2 nullable/no-default columns and a whole-row CHECK whose only shapes are all-null legacy or complete physical/VM. Install a BEFORE INSERT trigger that requires the complete shape; extend the immutable guard so an existing all-null legacy row may only set `superseded_at` and can never be partially backfilled. The current-deployment selector accepts only a complete new revision, so the first post-migration deployment revision must carry all policy/evidence refs/digests and carrier facts; partial NULLs, sentinels and legacy rows are rejected by current selection and every F-55/Stage 14b gate. Before applying `090700` to any shared database, require `PreF55DatabaseAdmissionV1` to prove the Stage 14a0-owned 28 migrations through 092600 plus Stage 6-owned 092700/092800 are all present/applied as the global 30-row pre-F55 tail；absence is a hard migration failure，never a reason to backfill a lower version later。
- [ ] **Step 5: Implement the single application validator.** Strict-deserialize the policy, evidence, vTPM/customer-control/nested/backup child DTOs, `CarrierFactProbeResultV1` and `CarrierEvidenceSignatureV1` under the shared JCS bounds, then expose only `validate_deployment_carrier(record: &DeploymentRecord, policy: &CarrierPolicyV1, evidence: &CarrierEvidenceV1, facts: &dyn CarrierFactProbe)`. Call only `facts.collect(evidence.stage14_run_id,evidence.deployment_id)`; reject a returned run/deployment mismatch and never substitute evidence/ref text for current facts. Verify release-root policy chain/revocation, policy digest/opaque-ref/current-release binding, both authorization and deployment-sidecar `signature_p1363_b64url` canonical no-pad/exact-64/low-S P1363 encodings and their exact preimages/key states, exact UUIDv7 Stage 14 ≤8h window/verified equality, all child run/time/preimage/ref/digest bindings, both distinct customer authorizations, deployment-KMS sidecar, evidence digest/ref, all fourteen columns plus legacy/new/current guards, physical and VM TPM `2.0`, current machine/site/provider/region/SKU/vTPM/nested/hypervisor/isolation facts, customer-control booleans and all seven managed components false. For VM, prove exact vTPM nonce/challenge/AK TPM2B/certificate chain/offline SPKI anchor/quote/PCR/event-log bytes+digest+replay/profile and one byte-equal approved SKU; nested=false closes only Hyper-V, while nested=true verifies the exact child and probe facts. Resolve the backup bundle and prove each embedded `BackupDimensionProbeEvidenceV1`, its domain-separated digest/restore/separation facts, bundle digest/ref, CarrierEvidence projection and current probe digest agree. Policy has no inferred expiry; every release, carrier fact or policy digest change requires a new run/evidence.
- [ ] **Step 6: Verify.** Run: `cargo xtask sqlcheck && cargo test -p ep-platform-obs --test f55_deployment_carrier && cargo test -p ep-testkit --test f55_deployment_carrier_schema_pg`. Expected: PASS with exactly two enum/database carrier values.
- [ ] **Step 7: Stage for the master atomic schema commit.** When invoked by 13c, do not run a carrier-schema-only commit. Stage the listed files only as this task's contribution to 13c Task 3 Step 4; the one joint `feat(f55): land atomic schema and shared identities` commit completes this checkbox after all Stage 14a0/global-pre-F55/F-55 migration prerequisites pass.

### Task 6: Prove carrier equivalence and close Stage 14 evidence

**Files:**
- Create: `tests/deployment_carrier/Cargo.toml`
- Create: `tests/deployment_carrier/src/lib.rs`
- Create: `tests/deployment_carrier/tests/f55_carrier.rs`
- Create: `scripts/verify-deployment-carrier.ps1`
- Reference: `crates/foundation/src/error/codes.rs`（13c Task 1 已预登记）
- Reference: `crates/platform/obs/src/metrics/registry.rs`（13c Task 1 已预登记）
- Modify: `xtask/src/ci.rs`
- Modify: `tools/release-gate/src/main.rs`
- Modify: `scripts/verify-release.ps1`
- Reference: `docs/error-codes.md`
- Reference: `docs/metrics-catalog.md`
- Modify: `docs/threat-model.md`
- Create: `docs/runbooks/deployment-carrier.md`
- Test: `tests/deployment_carrier/tests/f55_carrier.rs`

**Interfaces:**
- Consumes: Stage 14a0 的 signed deployment-record/evidence contracts，selected carrier、service/package manifest，以及供 Stage 14b 采集的 backup/recovery/key-recovery/ransomware evidence 接口；本任务不得假定 Stage 14a1 或 Stage 14b 的结果已存在。
- Produces: four stable carrier errors、`ep_deployment_carrier_info{carrier}` with two label values、`RG-DEPLOYMENT-CARRIER-EVIDENCE-GREEN` 的 parser/predicate/fixture 与候选 evidence contract。最终 `PASS`、签名实机 evidence 和 release package 只由 Stage 14b 产生。

- [ ] **Step 1: Write shared JCS, policy and Stage 14 evidence parser failures.** Set `[package] name = "ep-deployment-carrier"`. Apply the common ≤1,048,576-byte no-BOM UTF-8 RFC 8785/UTC-seconds/code/ref/array bounds and reject duplicate keys, noncanonical numbers, unknown fields and unsorted/duplicate arrays for every DTO. Strict `CarrierPolicyV1` has only `schema_version,policy_code,policy_version,residency_jurisdiction_code,allowed_physical_sites,allowed_iaas_regions,required_backup_separation,managed_components`; IaaS entries add exact `min_tpm_version="2.0",vtpm_attestation_required=true,vtpm_attestation_profile="TPM2_QUOTE_SHA256_V1"`, 1..8 byte-sorted/deduplicated 64-lowerhex `vtpm_ak_trust_anchor_spki_sha256[]`, and byte-sorted strict `vm_sku,nested_virtualization_supported` items. Separation is exactly the three frozen dimensions and all seven managed booleans are false. Strict `CarrierEvidenceV1` has only `schema_version=1,stage14_run_id,stage14_started_at,stage14_completed_at,deployment_id,carrier_kind,provider_code,region_code,vm_sku,residency_jurisdiction_code,region_jurisdiction_code,tpm_version,vtpm_present,vtpm_attestation_digest,nested_virtualization_supported,nested_virtualization_evidence_ref,nested_virtualization_evidence_digest,customer_control_attestation_digest,managed_components,backup_failure_domain_code,backup_failure_domain_evidence_digest,backup_separation_evidence,verified_at,verifier_subject,carrier_policy_digest,authorizations`. Require UUIDv7 run id, start<complete≤8h, verified=complete, child run equality/observed closed-window, TPM `2.0` for both carriers and the exact physical/VM nullability.
- [ ] **Step 2: Write vTPM, customer-control and nested child parser failures.** Strict `VtpmAttestationEvidenceV1` has exactly `schema_version,stage14_run_id,deployment_id,provider_code,region_code,vm_sku,profile,challenge_nonce_b64url,ak_public_tpm2b_public_b64url,ak_certificate_chain_der_b64url,quote_tpm2b_attest_b64url,quote_tpm2b_signature_b64url,signature_scheme,pcr_bank,pcr_selection,pcr_values,event_log_b64url,event_log_sha256,secure_boot_enabled,measured_boot_verified,probe_build_digest,observed_at`; enforce profile `TPM2_QUOTE_SHA256_V1`; canonical no-pad challenge/AK/quote/signature/event-log decoding at exact F-55 bounds; 1..8 leaf-to-root DER certs with total≤65,536; per-run CSPRNG 32-byte nonzero nonce dedup; exact run/deployment/policy-bound challenge preimage; offline chain/time/constraints/key-usage/leaf-AK equality and policy SPKI anchor; signature scheme/AK match; PCR `[0,2,4,7,11]`; event-log digest and bounded TCG Event03 SHA-256 replay; quote/PCR/Secure Boot/measured-boot verification; and no trailing bytes. Strict `CustomerControlEvidenceV1` has exactly `schema_version,stage14_run_id,deployment_id,carrier_kind,provider_code,region_code,customer_control_plane_subject_digest,windows_machine_sid_digest,customer_holds_os_admin,customer_holds_backup_credentials,customer_holds_kms_or_hsm_control,vendor_interactive_login_present,vendor_remote_support_enabled,managed_components,probe_build_digest,observed_at`; require three customer booleans true, two vendor booleans false and no raw subject/SID. Strict `NestedVirtualizationEvidenceV1` adds `stage14_run_id` to its exact deployment/provider/region/SKU/hypervisor/isolation/probe/time fields; both booleans true and nested=false requires no child/ref/digest.
- [ ] **Step 3: Write backup, fact-probe, authorization, sidecar and opaque-ref failures.** Strict `BackupDimensionProbeEvidenceV1` has exactly `schema_version,stage14_run_id,deployment_id,dimension,production_domain_digest,backup_domain_digest,separation_mechanism,backup_write_identity_separate,production_identity_can_delete_backup,restore_probe_digest,probe_build_digest,observed_at`; require distinct domains, write identity true, production-delete false, actual restore binding, exact dimension→mechanism map, and `evidence_digest=SHA-256("EP-CARRIER-BACKUP-DIMENSION-V1\0" || JCS(probe))`. Strict `BackupFailureDomainEvidenceV1` has only `schema_version=1,stage14_run_id,deployment_id,carrier_kind,backup_failure_domain_code,observed_at,entries`; its three byte-sorted strict `dimension,probe_evidence,evidence_digest` entries must rehash and project exactly to CarrierEvidence. Assert the bundle `observed_at` and all three embedded probe `observed_at` values fall within the same referenced CarrierEvidence Stage 14 closed window. Strict `CarrierFactProbeResultV1` uses the exact 21 fields in the interface above, physical `site_code=region_code`/VM site null, TPM `2.0`, conditional fields and no raw identity/path/attestation/secret/signature; `CarrierFactProbe` exposes only `collect(run_id,deployment_id)`. Test two sorted distinct role authorizations and exact strict fields `role,subject,approved_at,signature_key_ref,signature_key_version,signature_p1363_b64url`, then exact `CarrierEvidenceSignatureV1` fields `schema_version,purpose,deployment_id,evidence_digest,key_ref,key_version,signer_subject,signature_p1363_b64url`, domain-separated preimages and current-or-retired-nonrevoked deployment key. Every signature string must canonical no-pad round-trip to exactly 64-byte low-S P-256 P1363; reject integer array/lowerhex/DER/padding/high-S/wrong length. Resolve only the six opaque kinds; policy/evidence have fixed adjacent sidecars, all four child kinds are exact `.jcs` with no own sidecar/provider format, and every ref/digest/root owner/DACL/path-safety readback must close.
- [ ] **Step 4: Write equivalence cases.** Feed one physical and one domestic IaaS evidence bundle from the same package digest and valid distinct Stage 14 runs; assert identical services/config schema/database/backup/recovery/RPO/RTO/key recovery/ransomware/offline patch criteria. Only carrier-specific evidence may differ.
- [ ] **Step 5: Write backup attack cases.** Same VM directory, same virtual disk/domain, shared write identity, production-deletable backup, mutable provider snapshot, snapshot-only chain, stale/nonmatching restore probe and every invalid dimension/mechanism pair fail `OPS.BACKUP.FAILURE_DOMAIN_NOT_SEPARATE`; separately credentialed offsite/immutable media with all three current probe objects passes.
- [ ] **Step 6: Run local parser tests.** Run: `cargo test -p ep-deployment-carrier --test f55_carrier`. Expected: FAIL because the carrier parser/predicate/wiring and gate fixtures are absent; the four errors and one metric already exist from 13c Task 1 and must not be reported missing or re-registered.
- [ ] **Step 7: Consume and verify the pre-registered carrier catalog subset.** 13c Task 1 已一次性登记全部 F-55 catalog；本步逐项断言其中恰有 F-55 §8 的 4 个 OPS 错误与 `ep_deployment_carrier_info{carrier}`，再把 validator/probe 接到既有常量与 descriptor，禁止重复登记、改名或改写文档 catalog。`carrier` 只取 `customer_controlled_physical|customer_controlled_domestic_iaas_vm`；provider/region never become labels.
- [ ] **Step 8: Implement the Windows fact probe and signed evidence verification.** `verify-deployment-carrier.ps1` emits only strict `CarrierFactProbeResultV1` for the requested run/deployment, with current machine/site/provider/region/SKU/TPM/vTPM/nested/Hyper-V/customer-control/managed/backup digests and signed probe digest; no raw account/SID, child body, signature, secret or path. The Rust adapter applies the common byte/parser bounds before returning typed facts. The single validator resolves and verifies policy, evidence/sidecar and all exact child JCS; calls `collect` with the evidence ids; compares every probe, fourteen-column, policy, child, run/time, ref/digest, two-person and deployment-KMS fact. It independently validates vTPM TPM2 quote/PCR/event-log, customer-control booleans, nested conditionals and all embedded backup dimension probes. Any mismatch closes the carrier gate; nested-only mismatch closes Hyper-V MCP as well.
- [ ] **Step 9: Implement the fail-closed release-gate predicate and disclosure for Stage 14b.** Require the selected carrier evidence plus full existing Stage 14b/RPO≤15min/RTO≤4h/offsite restore/key recovery/ransomware/offline patch gates and identical pass criteria. Provider SLA or snapshot cannot substitute. Document provider/tenant-root disk/memory/network/rollback/stop risk, vTPM limits, no single-node HA and protected-backup restore; APIs expose only ref/digest/conclusion. In Stage 13c, missing real evidence must keep `RG-DEPLOYMENT-CARRIER-EVIDENCE-GREEN` nonzero; fixtures prove the predicate but cannot promote it to release PASS.
- [ ] **Step 11: Verify local registries.** Run: `cargo test -p ep-deployment-carrier --test f55_carrier && cargo xtask errorcodes && cargo xtask configdoc`. Expected: PASS.
- [ ] **Step 12: Commit the Stage 13c carrier candidate.** Run: `git add tests/deployment_carrier scripts/verify-deployment-carrier.ps1 xtask/src/ci.rs tools/release-gate/src/main.rs scripts/verify-release.ps1 docs/threat-model.md docs/runbooks/deployment-carrier.md && git commit -m "test(ops): close deployment carrier evidence gate"`. The shared error/config/metric registries and their docs belong to 13c Task 1 and must remain unchanged in this commit. This commit contains parser/predicate/probe code and signed fixtures only; it does not require either real carrier run or claim the final carrier gate PASS.
**External Step 10: Stage 14b carrier acceptance after Step 12; not a Stage 13c Task 6 checkbox.** Only after the committed Stage 13c candidate, all other stages 1–13 and the remaining Stage 14b prerequisites are complete, Stage 14b runs on Windows Server 2022 physical and customer-controlled domestic IaaS VM: `powershell -NoProfile -File scripts/verify-deployment-carrier.ps1`, `cargo xtask ci`, recovery/key/ransomware drills and `powershell -NoProfile -File scripts/verify-release.ps1`. Expected: each passes only with a complete exact run whose child/probe ids and times close; absence remains nonzero. The numeric label is retained for existing master-plan cross-references, but this external continuation cannot block Step 11 or the Stage 13c candidate commit. Until Stage 14b finishes it, no Task 6 fixture, local parser PASS or ServerAdmin matrix PASS authorizes installation, enablement or release.

## Completion Evidence

- [ ] Rust/API/DB/metrics agree on eight `ClientKind` values; audit alone adds `system`; persistent devices never store `mcp`.
- [ ] Matrix contains exactly 90 cells with ServerAdmin `2 FULL + 3 VIEW_ONLY + 13 N/A`; MCP has zero cells and embedded/database hashes match.
- [ ] `/server-admin/` is independently built and statically embedded; runtime process/listener set is unchanged and no writable asset directory exists.
- [ ] ServerAdmin exposes only authorized management/evidence/read-only surfaces and cannot conclude approvals or write any forbidden business domain. F-56 import uses exactly one lower-case-`.epcfg` file part, required decimal `Content-Length=1..4,194,304`, no chunked/other part or field, pre-body and streaming short/long enforcement, the only 4 MiB route exception, private CREATE_NEW staging/finally-delete, then autotest/submit/sign/release；approve/reject remain Win/Mac-only and `platform.document_attachment` remains N/A.
- [ ] `license_module_admin` is null outside authorized ServerAdmin bootstrap and otherwise exactly the F-56 masked four-state/usage/entitlement/15-module snapshot；no-current/signature-invalid omits all untrusted license identity/date/code/limit yet retains three actual usage counts. Signed single-item package lifecycle covers all five legal module actions, shared/exclusive drain gate and retained data；under Restricted/no-current only `LICENSE_GRANT` and `MODULE_PACKAGE:DISABLE` recovery chains survive, every other ordinary business/module/config action uses `PLATFORM.LICENSE.RESTRICTED`, and ServerAdmin still cannot conclude approval. No direct toggle、uninstall、generic rollback、secret/signature/path disclosure or private entitlement source exists.
- [ ] Deployment record accepts exactly two customer-controlled carriers and rejects foreign region, missing vTPM VM, managed components and non-separated backup evidence.
- [ ] The unique carrier validator accepts only the unified bounded strict-JCS policy/evidence/child/probe family bound to one exact UUIDv7 Stage 14 run, all fourteen persisted facts and `CarrierFactProbe.collect` observations; TPM 2.0, VM vTPM quote/PCR/event-log, customer-control and conditional nested evidence close exactly. The three embedded backup probes, bundle/ref/digest/projection and current probe facts agree; SECURITY and OPERATIONS use distinct registered subjects/keys/duties and complete evidence carries the exact deployment-KMS `CarrierEvidenceSignatureV1` sidecar.
- [ ] Both carriers use the identical signed product package and platform test criteria; selected-carrier evidence gate is machine-verifiable and provider snapshots never replace offsite backup.
- [ ] Stage 14a0 → Stage 13c → Stage 14b remains one-way for this carrier line: Stage 14a0 owns the generic foundation, Stage 14a1 separately owns only the F-56 adapter, this plan owns only F-55 extensions/predicates/fixtures, and Stage 14b alone owns real carrier certification, the final gate result and release permission; no partial release path exists.
