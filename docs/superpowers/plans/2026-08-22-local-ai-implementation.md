# Contained Local Analytics AI Implementation Plan

> **F-57 现行状态（2026-08-23）：`DEFERRED`。** 当前阶段只实现 AI provider、模型/提示/工具版本、动态授权、审计和隔离契约，不交付本地模型或固定 `ai-inferer` 容量。本文保留为未来研究和可复用安全材料，未经新的用户批准不得执行。

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** 交付一个完全本地、无数据库/网络/文件写权限的 `ai-inferer`，只把自然语言转换成经确定性校验、确认和逐次授权执行的单数据集查询计划。

**Architecture:** `ep-contract-ai` 冻结模型 IPC DTO 与 `AiInferencePort`；`ep-adapter-local-ai` 只负责签名模型包加载和受约束推理；`apps/ai-inferer` 通过 `\\.\pipe\ep-ai` 提供唯一业务 operation。`ep-app-reporting` 在 core-server 内构造裁剪目录、校验计划、签发五分钟 token，并经既有 `ep_analyst_ro` 池执行；结果从不回到模型进程。

**Tech Stack:** Rust 2021、`candle-core=0.11.0`、`candle-transformers=0.11.0`、`tokenizers=0.23.1`/onig、QWEN2/GGUF v3/MOSTLY_Q4_0、`GREEDY_SCHEMA_DFA_V1`、Serde strict JSON、Axum、PostgreSQL 16、Windows named pipes/DACL/Service SID/Job Objects、detached CMS/Authenticode、CAB 离线包、CycloneDX SBOM、OpenAPI 3.1、proptest、真实 PostgreSQL 与 Windows Server 2022 测试。

**Specs:** `docs/superpowers/specs/2026-08-22-f55-approved-ai-mcp-server-admin-cloud-freeze.md`；AI entitlement、许可证状态与共同发布门禁以后续 `docs/superpowers/specs/2026-08-22-f56-license-signed-module-package-freeze.md` 为唯一权威。

## Global Constraints

- 本计划只在用户另行授权后执行；当前只冻结文档，不运行代码、迁移、构建、测试或发布。
- 本文件是任务内容与任务内 step 顺序的唯一来源，但**不是跨线调度入口**。首版集成必须从 `2026-08-10-first-release-dev-plan/13c-local-ai-mcp-server-admin.md` 开始，并按其 Task 1–6 DAG 调用本文件的指定 Task/Step；禁止独立从本文件 Task 1 一路执行到 Task 7，因为 F-56 Stage 3b/13b 的 `ModuleLicenseQuery`/signed-grant 前置、模型表与 MCP 表的 receipt 互测、F-55 权限种子和八值 `ClientKind` 都有跨线前置。主计划只重排任务批次，不得改变本文件每个任务内部的 failing-test→implement→verify 顺序。
- `ai-inferer` 是第九个且唯一新增产品常驻进程；独立服务账户固定 `NT SERVICE\ep-ai`，独立 pipe 固定 `\\.\pipe\ep-ai`，独立 Job Object suffix 固定 `APP_AI`。
- pipe framing 固定 4-byte big-endian length + JSON；普通帧 1 MiB，handshake/first-frame/call deadline 为 5/10/120 秒；operation 恰好 `ai.query_plan.compose.v1|ai.model.activate.v1|ai.model.deactivate.v1|health.get.v1|metrics.snapshot.v1`。
- pipe 实例总数固定 51：`ep-core` compose 数据面 45（15 running + 30 queued）、`ep-core` 模型控制面 2、`ep-ops=2`、2 个只作持续 accept/补位；四组额度互不借用，达到任一账户/内部额度时必须在读取完整模型输入前拒绝，额度不是配置项。断连立即取消未开始的排队项；已运行项协作取消并清零该请求 decoder/KV。首版没有 per-invocation 子进程且不强杀 Rust 线程；若 2,000 ms 内未确认终止，监督器关闭 compose readiness 并终止整个 `ai-inferer`/`APP_AI` Job，使同进程全部在途请求一并取消、清零且以 `AI.MODEL_PACKAGE.NOT_ACTIVE` 失败。服务重启后必须按数据库唯一 ACTIVE 行重新做独立包复验、fresh activate ACK 与认证 gate 复核才可开放；绝不继续算到 120 秒、交付受影响结果或复用旧 ACK。
- compose HTTP route 是普通同步 HTTP 8 秒/20-slot 交易闸门的唯一 AI 具名例外：完成普通 body/header/session/法人基础校验后进入独立公平 45-slot semaphore（15 running + 30 queued），不占普通 20-slot；无位立即返回 `AI.INFERENCE.CONCURRENCY_LIMIT`，不建目录、不入 pipe。route Tower timeout 固定 122000 ms；内部 120000 ms 从进入 AI semaphore 起覆盖排队、IPC 与推理，余 2000 ms 只供取消、KV/decoder 清零与稳定封套。任一 deadline 映射既有 `PLATFORM.SYSTEM.SYNC_TIMEOUT`，且普通 8 秒 layer 不得提前截断。
- `ai-inferer` 为 0 DB credentials、0 network token、0 KMS、0 business-file access、0 file writes、0 interactive logon；只有模型目录只读权。
- 模型输入恰好 `catalog_projection + question + fixed_prompt`；request/trace/turn/cache/security digest 不进 tokenizer；结果、结果摘要、SQL 和自然语言结论不在 reply 类型中。
- 计划只允许一个 dataset；projection/group/aggregate/filter/order 上限 `64/16/16/32/3`，`limit=1..=1000`；禁止 join、subquery、UNION、window、任意函数、SQL 片段、计算列和 OR/NOT。
- Compose question 为 `1..=2000` Unicode scalar；plan token TTL 固定 300 秒；compose 与 execute 共同构成且只构成已认证只读 action POST 幂等豁免闭集：两者都绕过写请求幂等中间件、绝不读写 `platform_msg.idempotency_keys`，携带 `Idempotency-Key` 均以 `AI.QUERY_PLAN.IDEMPOTENCY_NOT_ALLOWED` 拒绝，不能类推新增第三个豁免端点；结果上限固定 1000 行和 8,388,608 bytes。
- Compose `locale` 首版只接受大小写敏感字面量 `zh-CN`；不存在语言包登记、回退语言、区域协商或实现期选择分支。
- AI reporting 两个 action POST 的 `X-Client` 闭集恰为 `win|mac|ios|android|ops|server_admin`；`portal|mcp`、未知值、别名和大小写变体全部在业务逻辑前拒绝。`server_admin` 不是自动授权：仍必须逐次通过 `ReportingReportPrint + Read`、对应 `reporting.ai_analysis.compose|execute` 权限与对象范围、F-56 AI entitlement 和其余普通身份/设备/session/密级/字段/记录门禁。
- AI entitlement 只调用 F-56 `ModuleLicenseQuery::entitlement_is_currently_licensed(EntitlementCodeV1::F55LocalAi, legal_entity_id)`。该查询逐次从唯一 current signed grant 重建 payload、验证内层 CMS/固定 `license-roots.p7b`/部署绑定/法人 scope 与可信时间；只有 `LicenseStatus::Active|ExpiringSoon|GracePeriod` 返回 true，`Restricted`、无 current、签名/slot 异常均在 catalog/model/pipe 前失败关闭。`purchased` 只表示 current/history 曾有验签通过的 `F55LocalAi`，不能放行 compose/execute；禁止本实施线定义任何 F-55 私有许可 payload、旧四态映射，或由 `EP__AI__ENABLED`、模块码、feature flag、环境变量、人工 JSON 推断授权。普通业务被通用 license gate 拒绝时使用 F-56 `PLATFORM.LICENSE.RESTRICTED=BUSINESS_CONFLICT/409/false`；compose/execute 仍只返回本计划和 `ai-reporting.v1.yaml` 的逐端点 exact 错误闭集，不得为方便把该 PLATFORM 码加进 AI route union。
- F-56 在 `Restricted`/无 current 时保留的恢复例外仅属于 `LICENSE_GRANT` 与 `MODULE_PACKAGE:DISABLE` 的签名配置包全链；它不开放 compose/execute、模型激活或任何 AI payload/pipe。AI 只消费恢复后重新验出的 current entitlement 和共同 gate，不实现或代理 import、审批、sign、release。
- plan token 只由 core-server 经法人密钥域的 `KmsBackend` purpose `AI_PLAN_TOKEN_V1` 签发：framing 精确为 `epai1.<base64url-no-pad(key_version_utf8)>.<base64url-no-pad(JCS(claims))>.<base64url-no-pad(signature_p1363_64)>`，签名是 ECDSA P-256 canonical low-S raw `r||s` 64 bytes。只有 ACTIVE key 可签发；旧 key 仅在未 revoked 且不晚于其最后合法 token 的 `expires_at+60s` 时验证，revoked key 立即拒绝。
- 模型包只允许单个不跨卷的签名 `.cab` 数据包，exact bytes 为 1..2,147,483,647，禁止 multi-CAB/spanning；生产仅 `PROD_AUTHENTICODE`，开发 fixture 可 `DEV_ECDSA_P256` 且生产拒绝；ACTIVE 行全部署唯一。七个 archive entry 总未压缩≤2,130,706,432，`model.gguf`≤2,000,000,000；低成本 CPU baseline 必须选用能装入该单包边界的模型，更大模型必须另立 Runtime ABI/包装裁定。
- AI 安装根 regular-file roster 恰为固定 `package.cab` 加 CAB 的 exact 七个提取 entry，共八项、零子目录/第九项；`SHA-256(package.cab)=package_digest`。安装器二次读回证明每项 extracted bytes 与 archive entry length/hash 相等，`ai-inferer` 每次激活都独立复验 CAB Authenticode/digest/exact roster/inner CMS/entry↔extracted equality后才 mmap `model.gguf`，不能只信收据或提取文件。
- Runtime ABI v1 只接受 QWEN2 architecture、单文件 `model.gguf`、GGUF v3、`general.file_type=MOSTLY_Q4_0` 与独立 `tokenizer.json`；已知 rank-2 matrix tensor 必须 Q4_0、rank-1 norm/bias 必须 F32，tensor 缺失、多余或 dtype 错均整包拒绝。
- `candle-core`、`candle-transformers` 精确锁 `=0.11.0`；直接 `tokenizers` 精确锁 `=0.23.1` 且 `default-features=false`。non-wasm 解析闭包固定接受 Candle 带入的 `tokenizers/onig` 与 `onig/onig_sys` 原生依赖，并把它们纳入 vendor/Cargo.lock/SBOM/签名/静态扫描；仍禁止 `tokenizers/http`、`hf-hub` 和运行期下载。
- CPU_LOCAL 是基线构建；GPU_LOCAL 只允许同一 0.11.0 Candle 启用 `cuda` 后形成的独立 Authenticode 签名、独立 SBOM、独立 Stage 14 认证制品。两种构建不得运行期切换 feature 或加载另一套引擎。
- 解码固定 `GREEDY_SCHEMA_DFA_V1`、temperature=0、`max_new_tokens=2048`、对 `AiQueryPlanV1` schema 的 UTF-8 字节前缀 DFA；同 logit 取较小 token id；根 object 的字段顺序固定为 `schema_version,dataset_code,projections,group_by,aggregates,filters_all,order_by,limit`。完整单个 object 后只有 token id `151645` 合法且其 tokenizer 解码必须为 `<|im_end|>`。sampling、seed、beam、top-k/top-p、第二个 JSON 值、hub 下载、Python、ONNX Runtime、llama.cpp/ggml FFI、自定义 op、脚本解释器、运行期动态库下载和模型包内代码全部禁止。
- Candle/tokenizers 版本、feature 集、Cargo.lock、离线 vendor 摘要与 SBOM 必须同一发布批次冻结；任何漂移使模型包 ABI 与资源认证失效。
- 模型登记同时唯一约束 `(model_code,model_version)`、`package_digest` 与 `install_receipt_id`。安装完成只通过 `ep-ops → ep-core` 的 exact `ops.signed_artifact.install_receipt.v1` 与 `SignedArtifactInstallReceiptV1` 送达；不得使用目录轮询或隐含回调替代通知。
- `SignedArtifactInstallReceiptV1.manifest_digest` 的 type-dependent 语义不可混用：AI kind 恰为 CAB 内 `AiModelPackageManifestV1` exact JCS 的 SHA-256；MCP kind 恰为数据库已批准 connector `McpManifestV1` canonical JCS digest。inner `McpLocalArtifactManifestV1` 只由 exact CAB `package_digest` 间接绑定并由安装器/plugin-host 从 CAB 原文复验，不另写进 receipt/DB 同名字段。
- `receipt_id` 在 AI model 与 MCP manifest 两张消费表之间也必须全局唯一：共享 handler 的事务第一步精确执行 `pg_advisory_xact_lock(hashtextextended(lower(receipt_id::text),4995704681966667073))`，随后在同一事务交叉查两表；lost-ACK replay 从目标行重建并比较除 transport correlation `request_id` 外的全部持久语义，即 `schema_version,artifact_kind,package_digest,manifest_digest,installed_root_ref,installed_at` 与完整 `subject`，只有全等的同对象才返回原 `registered_object_id`。首次与 replay 的 `SignedArtifactInstallReceiptAckV1` 都回显本次请求的 `request_id`，字段恰为 `request_id,receipt_id,registered_object_id`，不含 acceptance timestamp；跨对象/跨表、两表同时命中或任一持久语义分歧一律拒绝，任何迁移/运维/repository/SQL writer 旁路均禁止。
- AI 模型五时间/状态表固定：REGISTERED 五项全 NULL；VERIFIED 只有不可重写的 `verified_at`，其值是首次 receipt handler 服务端事务时点；首次 CERTIFIED 另写当前 report `finished_at` 到 `certified_at`，activation/disable/revoked 仍 NULL；ACTIVE 具有 verified/certified/activated 且 disabled/revoked NULL；DISABLED 具有 verified/certified/activated/disabled 且 `certified_at<=activated_at<disabled_at`。DISABLED→CERTIFIED 保留上轮 activated/disabled 成对值、把 certified 更新为本轮 report `finished_at` 且要求 `disabled_at<certified_at`；再次 ACTIVE 把 activated 更新为本次成功 ACK/gate 事务时点并清 disabled=NULL。REVOKED 只写一次且严格晚于所有已有时间，保留前态其他时间/空值；certification ref/digest 同空同非空，REGISTERED/VERIFIED 必空、CERTIFIED/ACTIVE/DISABLED 必有、REVOKED 保留前态形状。
- 数据库 ACTIVE 不是单独的可调用判据：core 必须经 `ai.model.activate.v1` 让 `ai-inferer` 独立复验签名/manifest/files/ACL 并取得 digest/profile ACK；DB ACTIVE 与最新 ACK 全字段一致才开放 compose。disable 使用 `ai.model.deactivate.v1`；任一进程重启、pipe 断开或事实不一致都关闭 compose 并重新 reconcile。
- 模型 weights/tensor mapping 只读并在进程内共享；每个推理请求拥有独立、不可复用的 mutable decoder/KV state，完成、拒绝、取消、断连、超时或失败立即清零释放且永不落盘。只允许处于同一 forward step 且法人、用户、安全摘要、目录摘要、模型摘要、提示版本六项全等的请求作临时动态 batching；batch 结束不保留或交叉共享任何 token/KV/prefix state。
- AI 配置固定为 `EP__AI__ENABLED=false`、`EP__AI__PLAN_TTL_SECONDS=300`、`EP__AI__MAX_CONCURRENT_REQUESTS=15`、`EP__AI__QUEUE_CAPACITY=30`、`EP__AI__COMPOSE_TIMEOUT_MS=120000`、`EP__AI__RESULT_ROW_LIMIT=1000`、`EP__AI__RESULT_BYTES_LIMIT=8388608`；不得由环境变量覆盖模型/提示/资源公式。
- AI 稳定错误注册表恰好使用 `AI.MODEL_PACKAGE.SIGNATURE_INVALID`、`AI.MODEL_PACKAGE.NOT_ACTIVE`、`AI.QUERY_PLAN.INVALID`、`AI.INPUT.CONTEXT_LIMIT_EXCEEDED`、`AI.QUERY_PLAN.NOT_VISIBLE_OR_DENIED`、`AI.QUERY_PLAN.CONFIRMATION_REQUIRED`、`AI.QUERY_PLAN.TOKEN_INVALID_OR_EXPIRED`、`AI.QUERY_PLAN.SECURITY_CONTEXT_CHANGED`、`AI.QUERY_PLAN.IDEMPOTENCY_NOT_ALLOWED`、`AI.INFERENCE.CONCURRENCY_LIMIT`、`AI.RESOURCE.BASELINE_NOT_CERTIFIED`，category/HTTP/retryable 逐项照抄 F-55 §8；端点可达集合不得取该注册表并集。`AI.QUERY_PLAN.NOT_VISIBLE_OR_DENIED` 只允许 compose 在结构已合法但引用当前裁剪目录中不存在/不可见 dataset/field code 时返回统一 404；execute 永不返回它，token 签名/claims/digest/期限无效只用 `AI.QUERY_PLAN.TOKEN_INVALID_OR_EXPIRED`，合法 token 绑定的目录/授权当前事实变化只用 `AI.QUERY_PLAN.SECURITY_CONTEXT_CHANGED`。
- Job Object hard limit 必须等于 `floor(0.095 × CERTIFIED_HOST_RAM_BYTES)`；认证负载固定 15 并发；未认证只阻发布/启用，不阻开发和 fixture CI。
- AI 资源认证只接受最大 1,048,576-byte RFC 8785 JCS strict `AiResourceCertificationReportV1` 及 strict `AiResourceCertificationSignatureV1` sidecar；签名 purpose 是 `AI_RESOURCE_CERTIFICATION_V1`，签名输入固定绑定 release batch/model/report digest。wire 字段固定为 `signature_p1363_b64url: String`，只接受 RFC 4648 §5 canonical base64url-no-pad，解码恰 64-byte ECDSA P-256 canonical low-S P1363 且重编码逐 byte 相等；JSON integer array、lowerhex、DER、padding 均拒绝。ref 只能是 `ep-evidence://ai-resource/<release-batch>/<model-package>/<cpu-local|gpu-local>/sha256/<report-digest>`，只经编译期证据根解析；DB/API/审计不返回路径、sidecar 或报告正文。
- `ai_runtime_release_facts` 只由 Stage 14 当场从签名 `ai-inferer.exe`、Cargo.lock、offline vendor、CycloneDX SBOM 和 resolved features 重算 strict `AiRuntimeReleaseFactsV1`；最大 262,144 RFC 8785 JCS bytes，exact 字段、数组上限/排序和 profile/version 闭集逐字照 F-55 §3.7。它不持久、无 ref、无独立签名/有效期/数据库行；其 exact bytes SHA-256 只进入最终签名认证报告的 mandatory `gate_code=ai_runtime_release_facts`，后续从保留制品重算一致性，禁止创建第二套 release evidence。
- AI 审计仅使用 `platform_audit.audit_events` 和编译期 `crates/platform/audit/src/object_registry.rs`：object `reporting.ai.query_turn` 是 object-level/id-required，action 恰为 `AI_QUERY_PLAN_COMPOSED|AI_QUERY_PLAN_EXECUTION_ATTEMPTED`。两 action 的 before 固定 NULL，after 只允许 F-55 §3.8 的十二个 strict masked 字段；phase 分别 `COMPOSED|EXECUTION_ATTEMPTED`，execute 的 `question_sha256` 固定 NULL，其余空值/摘要规则逐项一致，绝不保存问题/计划/SQL/结果/prompt/token/model output 正文。
- 九条 containment assertion 名称、数量和顺序逐字使用 F-55 §3.8；不得并入或改名既有 RLS assertions。
- 共享 `HighRiskOperation` 注册表恰好七值：原六类业务高风险加运维高风险 `DATA_MIGRATION`；本 AI 实施线不得回写旧六值快照、创建同义枚举或把数据迁移暴露为 AI plan/execute 能力。
- 共享 MCP LOCAL receipt 规则不得被 AI 的同事务 REGISTERED→VERIFIED 语义误套：MCP manifest 的十五列 receipt/root/time/HCS/root-SD + 十个 sandbox profile/SID/provider/sublayer/六-filter-key materialization group 只在 APPROVED 时允许一次整组 all-null→transport-complete、ACTIVE 前完整、之后不可变，rollback 新 DRAFT 不复制。stdio 要求 HCS 空且十个 sandbox 值全有；可选 Hyper-V container 要求 HCS 有且十值全空。卸载必须等所有引用版本 `REJECTED|SUPERSEDED|REVOKED`、零进程/句柄且不在 newest two；AI 模型仍使用本计划自己的 exact receipt→REGISTERED→VERIFIED 状态机。
- 共享 MCP 本地 transport 恰为 signed stdio 与可选 `LOCAL_WINDOWS_HYPERV_CONTAINER`。stdio 每 manifest version 使用独立 `<version-uuid>/<package-digest>` 根，禁 hardlink/reflink/block clone/共享文件；使用 ops-agent 在 IPv4/IPv6 connect/receive/bind 六个 ALE layer 预建的 exact-SID AppContainer/LPAC 与六个冻结 UUIDv5 WFP filter keys，再叠加 Job 和精确 stdin/stdout/NUL-stderr/可选 secret handle list。Hyper-V 只允许 strict OCI config 的单 Entrypoint/`C:\EP\Plugin`/`ContainerUser`、签名 app PE/DLL 与 base allowlist、Hyper-V isolation 和每调用 536870912-byte scratch，禁 process isolation/tmpfs。plugin-host/child/guest 必须禁 WER 上传/full/user dump 并通过 crash-marker 零泄漏；plugin-host 只能 readback/use。
- 共享 MCP exchange 只用独立九帧 `McpExchangeChunkStreamV1`：request/RequestBegin 都带同一 invocation id，wire 无 wall-clock deadline，新增无 ACK 的 `DispatchAuthorized`；RequestEnd COMPLETE 只表示 receiver 已验证并保留 rate、仍零 dispatch，caller 独立提交 ATTEMPT 后才授权一次 dispatch。Begin/authorization 无 ACK，Chunk/End/Abort 分别只允许 `CONTINUE|COMPLETE|ABORTED`，七个 abort reason 包含 `RATE_LIMIT|AUDIT_UNAVAILABLE`，错态至多一个 Abort 后关闭。ResponseEnd COMPLETE 只验证 raw terminal length/sequence/SHA-256，不代表业务 schema；caller 此后才 size-first strict parse/schema/field，失败写 terminal completion但不反转 ACK、退款或重放。receiver 用本机单调时钟执行不可重置的 30 秒绝对界。response 是最大 8,388,608-byte 的 raw terminal JSON-RPC bytes，SSE exact 单 event 固定 +23 且 decoded 上限 8,388,631，超限先返回 payload-too-large。core/gateway/plugin-host 分别是 inbound/remote/local 的唯一 60-second/60-call rate owner，成功计数后不退还并在冷启时灌入 60 timestamps。Hyper-V credential 必须 `None`；stdio 才可用匿名 secret pipe。AI 实施不得改写这些共享 ABI/限额。
- 共享 MCP 审计对 inbound、remote core/worker 与 local core/worker stdio/Hyper-V container 统一为 caller-owned attempt/completion/spool/crash-reconcile；core-server/job-worker 分别是 DB/spool owner，gateway/plugin-host/child/guest 不写二者。outbound caller 在 completion slot 后用专用 DB connection 取得 exact connector session advisory shared lock并重读 ENABLED/ACTIVE/identity/authz/binding，RequestEnd COMPLETE 后同锁最终重读，再 ATTEMPT/authorize/dispatch/terminal/unlock；receiver 保持 0 DB。disable/revoke/manifest switch 使用同 key transaction exclusive lock。两个专用 spool 各固定 1-GiB/1024×1-MiB，identity 后先预留 `.reserve`，terminal 同 slot 走 `.tmp`→`.ready`→DB，JCS 上限 1,048,507，30 秒 replay 并将损坏件改 `.corrupt` 保留，绝不重派。cancel control 不新增事件但原 invocation 必 completion。审计只存冻结摘要，身份未解析只写 exact 五字段安全日志。outbound job 必须携并重验原始非空法人/用户/设备/session/request identity，纯 system 零 rate/attempt/dispatch。`max_calls=1` 的受理 UPDATE 可将 grant 置 CONSUMED，但同一 accepted call 仍可 dispatch，只有后续请求失败。worker 可精确调用 remote 与 local exchange，但 AI 管道不得复用这些 operation 或增加任何 `ep-ai` ACE。
- 共享 MCP 状态约束不得因 AI 迁移/receipt 复用而放宽：connector 只允许 `REGISTERED→PENDING_APPROVAL→DISABLED`、`DISABLED↔ENABLED`、任一非 REVOKED→REVOKED，REVOKED 终态；enable 在同 connector exclusive lock 中重读 row version并要求唯一 compatible ACTIVE manifest、签名/key 状态、LOCAL 物化或 REMOTE origin/current credential probe、license/gates 全部有效。grant 只允许 `ACTIVE→CONSUMED|REVOKED|EXPIRED` 且后三终态，scanner/revoke/final-counter UPDATE 各自产生唯一对应边；AI 的 `090000` migration/repository 不得写、恢复或旁路这些 MCP 状态。
- request `params._meta` 是 F-55 strict-object 规则的唯一有界 extension-map 例外，验证后立即忽略；success `_meta` 仍只含 serverInfo。入站 MCP error data 只有普通 `{stable_code,request_id}` 与 version-only `{stable_code,request_id,supported,requested}` 两种 strict 形状。除此之外所有 F-55 strict JSON object 和 internally tagged enum DTO 均显式 `#[serde(deny_unknown_fields)]`；unit enum 只接受冻结 variant 与大小写，未知字段/tag/variant/value 在业务逻辑前失败。
- 本线自有发布 gate 固定为 `RG-AI-CONTAINMENT-GREEN` 与 `RG-AI-RESOURCE-CERTIFIED`；任何 purchased+enabled AI 的 applicability 还共同依赖 F-56 `RG-LICENSE-MODULE-LIFECYCLE-GREEN`，本线只消费该签名结论，不复制其证据 DTO、parser 或 registry。

---

## File Map

| 单元 | 文件 | 责任 |
|---|---|---|
| ABI | `crates/contract/ai/src/{dto,port}.rs` | strict DTO、枚举、`AiInferencePort` |
| 模型适配 | `crates/adapter/local-ai/src/package/{manifest,verify}.rs`、`src/runtime/{candle_qwen2,gguf}.rs`、`src/decoder/schema_dfa.rs` | CAB 验签、QWEN2/GGUF v3/MOSTLY_Q4_0 只读加载、封闭解码 |
| IPC | `crates/adapter/ipc/src/ai.rs` | `ep-ai` framing、DACL、peer token、deadline |
| 进程 | `apps/ai-inferer/src/{main,service,wiring}.rs` | 服务入口、队列、模型生命周期、health/metrics |
| 编排 | `crates/application/reporting/src/ai/{catalog,validation,token,compose,execute}.rs` | 目录裁剪、校验、确认、token、逐次重检和查询 |
| HTTP | `apps/core-server/src/reporting/ai_query_plans.rs` | 两个固定 POST 路由与 envelope |
| 数据 | 两个 F-55 migration | 模型包 registry 与 unpoliced registry |
| 验收 | `tests/ai_containment/`、`testkit/src/ai_containment.rs` | 九条断言和发布证据 |

### Task 1: Add the strict AI and shared install-receipt contracts

**Files:**
- Modify: `Cargo.toml`
- Create: `crates/contract/ai/Cargo.toml`
- Create: `crates/contract/ai/src/lib.rs`
- Create: `crates/contract/ai/src/dto.rs`
- Create: `crates/contract/ai/src/port.rs`
- Create: `crates/adapter/ipc/src/signed_artifact_install.rs`
- Test: `crates/contract/ai/tests/abi.rs`
- Test: `crates/adapter/ipc/tests/f55_signed_artifact_install_abi.rs`
- Modify: `xtask/src/archcheck/deps.rs`

**Interfaces:**
- Consumes: `ep_foundation::{AppError, Id, RequestId, SecurityContext, Sha256Digest, TraceId}` and `uuid::Uuid`.
- Produces: every F-55 §3.3 `Ai*V1` DTO, `FixedPromptV1`, `AiModelInputV1`, `AiComposePipeRequestV1`, `AiComposePipeReplyV1`, plus the exact F-55 §3.2 shared `SignedArtifactInstallReceiptV1`/kind/subject/ACK types and operation literal reproduced in Task 3, and the exact port:

```rust
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum AiModelSignatureKind { ProdAuthenticode, DevEcdsaP256 }

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum AiExecutionProfileV1 { CpuLocal, GpuLocal }

pub const AI_RUNTIME_ABI_VERSION: u16 = 1;
pub const AI_MAX_NEW_TOKENS: u32 = 2048;
pub const AI_EOS_TOKEN_IDS: [u32; 1] = [151645];
pub const AI_MODEL_ACTIVATE_V1: &str = "ai.model.activate.v1";
pub const AI_MODEL_DEACTIVATE_V1: &str = "ai.model.deactivate.v1";

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AiModelActivationDescriptorV1 {
    pub request_id: RequestId,
    pub schema_version: u16,
    pub model_package_id: Uuid,
    pub model_code: String,
    pub model_version: String,
    pub package_digest: Sha256Digest,
    pub manifest_digest: Sha256Digest,
    pub installed_root_ref: String,
    pub execution_profile: AiExecutionProfileV1,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AiModelActivationAckV1 {
    pub request_id: RequestId,
    pub model_package_id: Uuid,
    pub package_digest: Sha256Digest,
    pub manifest_digest: Sha256Digest,
    pub runtime_abi_version: u16,
    pub execution_profile: AiExecutionProfileV1,
    pub independently_verified_at: DateTime<Utc>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AiModelDeactivationRequestV1 {
    pub request_id: RequestId,
    pub schema_version: u16,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AiModelDeactivationAckV1 {
    pub request_id: RequestId,
    pub previous_model_package_id: Option<Uuid>,
    pub deactivated_at: DateTime<Utc>,
}

#[async_trait]
pub trait AiInferencePort: Send + Sync {
    async fn compose(
        &self,
        request: AiComposePipeRequestV1,
    ) -> Result<AiComposePipeReplyV1, AppError>;
}
```

- [ ] **Step 1: Write the failing ABI tests.** Assert `AI_PIPE_NAME == r"\\.\pipe\ep-ai"`, compose/activate/deactivate operation literals, SCREAMING_SNAKE_CASE enums, every F-55 activation descriptor/ACK, deactivation request/ACK and compose request/reply field, plus rejection of unknown JSON fields. In `f55_signed_artifact_install_abi.rs`, assert operation `ops.signed_artifact.install_receipt.v1`, exact request fields and the three-field `request_id,receipt_id,registered_object_id` ACK with no timestamp, exact wire kinds `AI_MODEL_PACKAGE|MCP_SIGNED_STDIO_PACKAGE|MCP_WINDOWS_HYPERV_CONTAINER`, two tagged subject variants and all their fields, schema 1, strict unknown-field rejection and the three exact `ep-install://` reference shapes. Assert receipt `manifest_digest` means inner `AiModelPackageManifestV1` exact JCS for AI but approved connector `McpManifestV1` canonical JCS for either MCP kind; an inner `McpLocalArtifactManifestV1` digest in that field is rejected, and lost-ACK equality excludes transport `request_id` but includes schema/kind/package/type-dependent-manifest/root/installed-at/full subject, and replay ACK echoes the current request id. Assert `AI_RUNTIME_ABI_VERSION == 1`, `AI_MAX_NEW_TOKENS == 2048`, `AI_EOS_TOKEN_IDS == [151645]`, and compile-time dependency closure `ep-contract-ai -> ep-foundation` only.
- [ ] **Step 2: Run the contract tests.** Run: `cargo test -p ep-contract-ai --test abi && cargo test -p ep-adapter-ipc --test f55_signed_artifact_install_abi`. Expected: FAIL because the package and symbols do not exist.
- [ ] **Step 3: Create the contract packages.** Copy the F-55 §3.3 AI public structs/enums and the F-55 §3.2 shared signed-artifact request/kind/subject/ACK structs verbatim, add `#[serde(deny_unknown_fields)]` to every object-shaped DTO, keep `Decimal(String)` canonical-text and `DateTime(String)` offset requirements as validators rather than lossy deserialization. Add only the three Runtime ABI constants above; package-manifest parsing belongs to the local adapter in Task 3. `ep-contract-ai` must not depend on Candle or tokenizers, and the shared receipt DTO must not depend on an AI or MCP application crate.
- [ ] **Step 4: Add explicit ABI fixtures.** Use one accepted fixture containing one dataset/field and one rejected fixture with `{"sql":"select 1"}` appended to a plan; the latter must fail deserialization before application code runs. Add exact activation/deactivation fixtures; reject schema version other than 1, request-id mismatch, package/manifest digest or execution-profile mismatch, wrong runtime ABI, stale verification time, unexpected prior package id and every extra/missing field. Explicitly reject certification-report ref/digest in the activation descriptor or ACK: those facts are core/Stage 14 gates, not ai-inferer ABI fields.
- [ ] **Step 5: Verify contract and architecture.** Run: `cargo test -p ep-contract-ai --test abi && cargo test -p ep-adapter-ipc --test f55_signed_artifact_install_abi && cargo xtask archcheck`. Expected: PASS and no IO/database/HTTP dependency in either strict DTO surface.
- [ ] **Step 6: Commit.** Run: `git add Cargo.toml crates/contract/ai crates/adapter/ipc/src/signed_artifact_install.rs crates/adapter/ipc/tests/f55_signed_artifact_install_abi.rs xtask/src/archcheck/deps.rs && git commit -m "feat(ai): freeze inference and install receipt contracts"`.

### Task 2: Create the immutable model package registry

**Files:**
- Create: `db/migrations/platform_ops/V20261024090000__platform_ops_create_ai_model_packages.sql`
- Create: `db/migrations/platform_core/V20261024090800__platform_core_backfill_f55_unpoliced_table_registry.sql`
- Create: `db/checks/25_f55_ai_model_packages.sql`
- Create: `crates/application/reporting/src/ai/model_package_registry.rs`
- Test: `testkit/tests/f55_ai_schema_pg.rs`
- Test: `crates/application/reporting/tests/f55_model_package_lifecycle_pg.rs`
- Modify: `docs/migration-catalog.md`

**Interfaces:**
- Consumes: deployment-level common columns and `platform_core.unpoliced_table_registry`.
- Produces: `platform_ops.ai_model_packages`, global `UNIQUE(active_slot)`, exact six-state lifecycle, registry reason `SAME_FOR_ALL_ENTITIES`, and the only application writer:

```rust
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AiResourceCertificationReportV1 {
    pub schema_version: u16,
    pub report_id: Uuid,
    pub release_batch_id: Uuid,
    pub product_build_digest: Sha256Digest,
    pub ai_runtime_artifact_digest: Sha256Digest,
    pub model_package_id: Uuid,
    pub model_code: String,
    pub model_version: String,
    pub runtime_abi_version: u16,
    pub package_digest: Sha256Digest,
    pub manifest_digest: Sha256Digest,
    pub execution_profile: AiExecutionProfileV1,
    pub resource_formula_version: String,
    pub server_spec_digest: Sha256Digest,
    pub certified_host_ram_bytes: u64,
    pub calculated_hard_limit_bytes: u64,
    pub max_context_tokens: u32,
    pub max_concurrent_requests: u16,
    pub load_profile_digest: Sha256Digest,
    pub host_commit_peak_bytes: u64,
    pub vram_peak_bytes: Option<u64>,
    pub gpu_device_model: Option<String>,
    pub gpu_driver_version: Option<String>,
    pub page_or_swap_observed: bool,
    pub gate_results: Vec<AiResourceGateResultV1>,
    pub started_at: DateTime<Utc>,
    pub finished_at: DateTime<Utc>,
    pub verifier_subject: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AiResourceGateResultV1 {
    pub gate_code: String,
    pub outcome: String, // validator accepts only literal PASSED
    pub evidence_digest: Sha256Digest,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AiResourceCertificationSignatureV1 {
    pub schema_version: u16,
    pub purpose: String, // validator accepts only AI_RESOURCE_CERTIFICATION_V1
    pub release_batch_id: Uuid,
    pub model_package_id: Uuid,
    pub report_digest: Sha256Digest,
    pub key_ref: String,
    pub key_version: String,
    pub signer_subject: String,
    pub signature_p1363_b64url: String,
}

pub struct AiModelPackageView {
    pub id: Uuid,
    pub model_code: String,
    pub model_version: String,
    pub package_digest: Sha256Digest,
    pub manifest_digest: Sha256Digest,
    pub install_receipt_id: Uuid,
    pub installed_at: DateTime<Utc>,
    pub runtime_abi_version: u16,
    pub prompt_template_version: String,
    pub execution_profile: AiExecutionProfileV1,
    pub certification_report_ref: Option<String>,
    pub certification_report_digest: Option<Sha256Digest>,
    pub status: AiModelPackageStatus,
    pub row_version: i64,
}

pub trait AiModelPackageRegistry: Send + Sync {
    fn register_verified_from_receipt(
        &self,
        tx: &mut dyn Tx,
        receipt: SignedArtifactInstallReceiptV1,
    )
        -> Result<AiModelPackageView, AppError>;
    fn record_certification(
        &self,
        tx: &mut dyn Tx,
        report_ref: &str,
        report: AiResourceCertificationReportV1,
        signature: AiResourceCertificationSignatureV1,
    )
        -> Result<AiModelPackageView, AppError>;
    fn activate(&self, tx: &mut dyn Tx, id: Uuid, row_version: i64)
        -> Result<AiModelPackageView, AppError>;
    fn disable(&self, tx: &mut dyn Tx, id: Uuid, row_version: i64)
        -> Result<AiModelPackageView, AppError>;
    fn revoke(&self, tx: &mut dyn Tx, id: Uuid, row_version: i64)
        -> Result<AiModelPackageView, AppError>;
    fn select_active(&self, snapshot: &SnapshotCtx)
        -> Result<AiModelPackageView, AppError>;
}
```

- [ ] **Step 1: Write failing direct-SQL and lifecycle tests.** Cover every F-55 §3.6 column, fixed `security_level=40`, `install_receipt_id/installed_at`, nullable certification ref/digest, 32-byte digest checks, positive context/concurrency, unique model tuple/package digest/install receipt/active slot, production signature rule, evidence shape, every legal/illegal edge, identity immutability and REVOKED terminality. Freeze the exact five-time truth table: REGISTERED all NULL; VERIFIED only immutable first-server-transaction `verified_at`; first CERTIFIED adds report `finished_at`; ACTIVE has verified/certified/activated; DISABLED has all but revoked with `certified<=activated<disabled`; DISABLED→CERTIFIED preserves the prior activated/disabled pair, updates certified and requires disabled<certified; re-ACTIVE updates activated and clears disabled; REVOKED writes a once-only latest timestamp while preserving the prior shape. Certification ref/digest must be jointly NULL for REGISTERED/VERIFIED, jointly nonnull for CERTIFIED/ACTIVE/DISABLED, and retain the predecessor shape in REVOKED. Application tests prove only authenticated `ops.signed_artifact.install_receipt.v1` with AI kind/subject can insert REGISTERED and advance it to VERIFIED in the same transaction; same-object lost-ACK replay compares every persistent receipt semantic except current `request_id`, echoes the current request id in the three-field ACK and returns the same object, while divergent reuse fails. Concurrent fixtures submit one receipt UUID to AI and MCP targets: transaction first executes `pg_advisory_xact_lock(hashtextextended(lower(receipt_id::text),4995704681966667073))`, then cross-queries both tables; exactly one wins, equal same-object replay alone succeeds, hash collisions merely serialize, and direct migration/ops/repository/SQL bypass is rejected by ownership/privilege/static tests. Certification fixtures strict-parse the exact report/gate/signature structs above, cap report JCS at 1,048,576 bytes, require byte-sorted unique exact gate registry with every outcome `PASSED`, CPU/GPU conditional fields, `page_or_swap_observed=false`, ABI 1/concurrency 15 and every release/model/server/load identity match. Verify sidecar purpose/preimage/key state and `signature_p1363_b64url` canonical no-pad round-trip to exactly 64-byte P-256 low-S P1363; reject integer-array/lowerhex/DER/padded/wrong-length signatures. Verify the opaque `ep-evidence://ai-resource/<release-batch>/<package>/<cpu-local|gpu-local>/sha256/<digest>` ref/root/DACL/JCS/sidecar. Missing/extra/mismatched/stale/revoked evidence cannot move VERIFIED→CERTIFIED; valid `certified_at` equals report `finished_at`. Activation requires both AI gates; failed ai-inferer verification moves the attempted ACTIVE row to DISABLED; DISABLED returns through CERTIFIED before another attempt; disable/revoke retains row/files/evidence.
- [ ] **Step 2: Run on a fresh PostgreSQL 16 database.** Run: `cargo test -p ep-testkit --test f55_ai_schema_pg -- --nocapture`. Expected: FAIL because the table is absent.
- [ ] **Step 3: Implement `090000`.** Create the deployment-level table without `legal_entity_id` or RLS, with `security_level smallint` fixed 40, immutable `install_receipt_id uuid` and `installed_at timestamptz`, nullable certification ref/digest, `UNIQUE(model_code,model_version)`, `UNIQUE(package_digest)`, `UNIQUE(install_receipt_id)` and generated unique ACTIVE slot. Add the exact lifecycle/immutability guard for `REGISTERED|VERIFIED|CERTIFIED|ACTIVE|DISABLED|REVOKED` and the five-time/ref truth table from Step 1: immutable verified; current/latest certified/activated/disabled round; paired retained activated/disabled across DISABLED→CERTIFIED; re-ACTIVE clears disabled; terminal once-only revoked strictly last; REVOKED retains its predecessor's ref/time shape. Grant SELECT only to the core model selector and authorized ops read role; revoke application writes and grant no DB privilege to `NT SERVICE\ep-ai`.
- [ ] **Step 4: Implement `090800`.** Insert exactly one registry row for `platform_ops.ai_model_packages` with consumer `core-server model selector` and reason `SAME_FOR_ALL_ENTITIES`; make reruns checksum-stable and reject a conflicting pre-existing registration.
- [ ] **Step 5: Implement the owner repository.** `register_verified_from_receipt` accepts only the peer-authenticated shared operation; enforces schema/kind/subject and `ep-install://ai-model/sha256/<package-digest>`; resolves the compile-time root; reads only owner/DACL through ep-core `READ_CONTROL` and never lists/reads/executes package content; and rechecks receipt/config/digest facts produced by the locked ops verifier. In one transaction, the first statement is exactly `pg_advisory_xact_lock(hashtextextended(lower(receipt_id::text),4995704681966667073))`; then query both AI and MCP receipt columns, reject cross-table/object/divergent/two-row reuse, insert REGISTERED with receipt/time and immediately apply the legal edge to VERIFIED with `verified_at` equal to this first server transaction time. Replay reconstructs and compares every persistent semantic except `request_id`, then returns the original id in an ACK that echoes the current request id; divergent semantics fail. No migration, ops, second application writer or raw SQL path gets privileges. `record_certification` resolves only the frozen opaque ref through the compile-time AI evidence root, rechecks SYSTEM owner/no-inheritance DACL (`ep-ops` manage, `ep-core` read/READ_CONTROL, no ai-inferer access), exact report JCS digest and sidecar. It decodes canonical `signature_p1363_b64url` to exactly 64 low-S P1363 bytes and verifies preimage `SHA-256("EP-AI-RESOURCE-CERTIFICATION-V1\0" || release_batch_id[16] || model_package_id[16] || report_digest[32])` with deployment KMS purpose `AI_RESOURCE_CERTIFICATION_V1`, accepts only current or retired-nonrevoked historical key for that release batch, and binds all report/build/runtime/model/server/load/gate facts before writing ref/digest and `certified_at=finished_at`. ACTIVE and post-activate-ACK checks repeat ref/digest/signature/key/current-gate verification; readiness remains closed until matching independent ACK, and verifier failure commits ACTIVE→DISABLED. Every mutation uses row version, audit-terminal ordering and the DB guard.
- [ ] **Step 6: Verify schema and history safety.** Run: `cargo xtask sqlcheck && cargo test -p ep-testkit --test f55_ai_schema_pg -- --nocapture && cargo test -p ep-app-reporting --test f55_model_package_lifecycle_pg`. Expected: PASS; both new catalog paths match and every pre-F-55 migration checksum remains unchanged.
- [ ] **Step 7: Stage for the master atomic schema commit.** When invoked by 13c, do not run an AI-only commit. Stage the listed files only as this task's contribution to 13c Task 3 Step 4; the one joint `feat(f55): land atomic schema and shared identities` commit completes this checkbox after all nine migrations and both receipt tables pass together.

### Task 3: Verify, install, and load signed model packages

**Files:**
- Modify: `Cargo.toml`
- Modify: `Cargo.lock`
- Create: `crates/adapter/local-ai/Cargo.toml`
- Create: `crates/adapter/local-ai/src/lib.rs`
- Create: `crates/adapter/local-ai/src/package/mod.rs`
- Create: `crates/adapter/local-ai/src/package/manifest.rs`
- Create: `crates/adapter/local-ai/src/package/verify.rs`
- Create: `crates/adapter/local-ai/src/runtime/mod.rs`
- Create: `crates/adapter/local-ai/src/runtime/candle_qwen2.rs`
- Create: `crates/adapter/local-ai/src/runtime/gguf.rs`
- Create: `crates/adapter/local-ai/src/decoder/mod.rs`
- Create: `crates/adapter/local-ai/src/decoder/schema_dfa.rs`
- Modify: `crates/adapter/ipc/src/signed_artifact_install.rs`
- Modify: `apps/ops-agent/src/targets.rs`
- Create: `apps/ops-agent/src/ai_model_package.rs`
- Create: `apps/core-server/src/platform/signed_artifact_install_receipts.rs`
- Create: `apps/core-server/src/wiring/ai_models.rs`
- Test: `crates/adapter/local-ai/tests/package_verification.rs`
- Test: `crates/adapter/local-ai/tests/runtime_abi.rs`
- Test: `crates/adapter/local-ai/tests/schema_dfa.rs`
- Test: `apps/ops-agent/tests/f55_ai_package_install.rs`
- Test: `apps/core-server/tests/f55_ai_model_install_receipt.rs`
- Test: `crates/adapter/ipc/tests/f55_signed_artifact_install_receipt_pipe_windows.rs`
- Test: `xtask/tests/f55_ai_runtime_dependency.rs`
- Modify: `xtask/src/archcheck/deps.rs`
- Modify: `xtask/src/archcheck/source.rs`

**Interfaces:**
- Consumes: CAB bytes, detached CMS signature, production Authenticode trust store, DEV ECDSA fixture trust store and ACL-protected install root.
- Produces the only permitted dependency/feature declarations:

```toml
# workspace Cargo.toml
candle-core = { version = "=0.11.0" }
candle-transformers = { version = "=0.11.0" }
tokenizers = { version = "=0.23.1", default-features = false }

# crates/adapter/local-ai/Cargo.toml
[features]
default = ["cpu"]
cpu = []
cuda = ["candle-core/cuda", "candle-transformers/cuda"]
```

- Produces:

```rust
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AiModelPackageManifestV1 {
    pub schema_version: u16,
    pub model_code: String,
    pub model_version: String,
    pub runtime_abi_version: u16,
    pub engine: String,
    pub engine_version: String,
    pub tokenizer_engine: String,
    pub tokenizer_engine_version: String,
    pub architecture: String,
    pub weights_format: String,
    pub quantization: String,
    pub weights_file: String,
    pub tokenizer_file: String,
    pub decoder: String,
    pub prompt_encoding: String,
    pub prompt_template_file: String,
    pub prompt_template_version: String,
    pub prompt_template_digest: Sha256Digest,
    pub max_new_tokens: u32,
    pub max_context_tokens: u32,
    pub max_concurrent_requests: u16,
    pub execution_profile: AiExecutionProfileV1,
    pub resource_formula_version: String,
    pub eos_token_ids: Vec<u32>,
    pub files: Vec<AiModelPackageFileV1>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AiModelPackageFileV1 {
    pub path: String,
    pub media_type: String,
    pub size_bytes: u64,
    pub sha256: Sha256Digest,
}

pub const OPS_SIGNED_ARTIFACT_INSTALL_RECEIPT_V1: &str =
    "ops.signed_artifact.install_receipt.v1";

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SignedArtifactInstallReceiptV1 {
    pub request_id: RequestId,
    pub schema_version: u16,
    pub receipt_id: Uuid,
    pub artifact_kind: SignedArtifactKindV1,
    pub package_digest: Sha256Digest,
    pub manifest_digest: Sha256Digest,
    pub installed_root_ref: String,
    pub installed_at: DateTime<Utc>,
    pub subject: SignedArtifactInstallSubjectV1,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum SignedArtifactKindV1 {
    #[serde(rename = "AI_MODEL_PACKAGE")]
    AiModelPackage,
    #[serde(rename = "MCP_SIGNED_STDIO_PACKAGE")]
    McpSignedStdioPackage,
    #[serde(rename = "MCP_WINDOWS_HYPERV_CONTAINER")]
    McpWindowsHyperVContainer,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields, tag = "kind", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum SignedArtifactInstallSubjectV1 {
    AiModel {
        model_code: String,
        model_version: String,
        runtime_abi_version: u16,
        signer_subject: String,
        signature_kind: String,
        prompt_template_version: String,
        max_context_tokens: u32,
        max_concurrent_requests: u16,
        execution_profile: AiExecutionProfileV1,
        resource_formula_version: String,
    },
    McpLocal {
        legal_entity_id: Uuid,
        connector_id: Uuid,
        manifest_version_id: Uuid,
        artifact_attachment_version_id: Uuid,
        container_image_digest: Option<Sha256Digest>,
        hcs_image_identity: Option<String>,
        installed_root_sd_sha256: Sha256Digest,
        sandbox_profile_name: Option<String>,
        sandbox_sid: Option<String>,
        wfp_provider_guid: Option<Uuid>,
        wfp_sublayer_guid: Option<Uuid>,
        wfp_connect_v4_filter_key: Option<Uuid>,
        wfp_connect_v6_filter_key: Option<Uuid>,
        wfp_recv_accept_v4_filter_key: Option<Uuid>,
        wfp_recv_accept_v6_filter_key: Option<Uuid>,
        wfp_resource_assignment_v4_filter_key: Option<Uuid>,
        wfp_resource_assignment_v6_filter_key: Option<Uuid>,
    },
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SignedArtifactInstallReceiptAckV1 {
    pub request_id: RequestId,
    pub receipt_id: Uuid,
    pub registered_object_id: Uuid,
}

pub trait LocalModelRuntime: Send + Sync {
    fn compose(&self, input: &AiModelInputV1) -> Result<AiQueryPlanV1, AppError>;
}
```

- [ ] **Step 1: Write package rejection tests.** Start from one non-spanned signed CAB of 1..2,147,483,647 exact bytes whose exact seven entries are `manifest.jcs.json`, `manifest.p7s`, `LICENSE.txt`, `model.gguf`, `prompt-template.utf8`, `sbom.cdx.json`, `tokenizer.json`; the five-file manifest roster is byte-sorted, `model.gguf` is 1..2,000,000,000 bytes, the seven unpacked entries total≤2,130,706,432, and every other per-file bound equals F-55 §3.2. Mutate multi-CAB/spanning, each Runtime ABI manifest literal, eighth/missing/duplicate/case-colliding entry, CAB/manifest/file hash, detached-CMS form/messageDigest/EKU/chain/expiry/revocation/algorithm, production/DEV mode, prompt BOM/CRLF/NUL/special token, GGUF v3 metadata, context relation, tokenizer 151644/151645 mapping, ACL/root and package identity. Installation fixture requires the root roster to be exactly fixed `package.cab` plus the seven extracted entries and mutates any missing/extra/subdirectory file, package digest, hardlink and archive-entry↔extracted length/hash mismatch. Derive the exact Qwen2 roster only from required GGUF metadata: root `token_embd.weight,output_norm.weight` plus optional `output.weight`, and twelve exact tensors for every `blk.0..block_count-1`; reject missing/extra tensors, bad dimensions/head divisibility, rank-2 matrix not Q4_0, rank-1 norm/bias not F32 or illegal output tying. Add DLL/script/install-hook/path-traversal/reparse/ADS/safetensors/ONNX/second-weights negatives. Every package rejection maps to `AI.MODEL_PACKAGE.SIGNATURE_INVALID`; only absent ACTIVE selection maps to `AI.MODEL_PACKAGE.NOT_ACTIVE`.
- [ ] **Step 2: Write failing runtime and decoder tests.** `runtime_abi.rs` loads the minimal QWEN2/GGUF-v3/MOSTLY_Q4_0 fixture under the fixed CPU/onig feature closure and rejects any engine, version, feature, tensor roster or file-format deviation. `schema_dfa.rs` proves every emitted token keeps a valid UTF-8 byte prefix of the one fixed `AiQueryPlanV1` schema; exact field order is required; markdown, unknown fields, NaN/Infinity, leading/trailing prose and a second JSON value are impossible; equal logits choose the lower token id; completion enables only token `151645`; that token decodes exactly `<|im_end|>`; early EOS, 2048 generated-token exhaustion or deadline before closure returns `AI.QUERY_PLAN.INVALID`. A separate fixture proves input tokens plus 2048 above the smaller manifest/GGUF context bound returns `AI.INPUT.CONTEXT_LIMIT_EXCEEDED` before generation and never truncates. Compile-fail request fixtures prove there is no temperature, sampling, seed, beam, top-k or top-p input.
- [ ] **Step 3: Run the focused failures.** Run: `cargo test -p ep-adapter-local-ai --test package_verification --features cpu && cargo test -p ep-adapter-local-ai --test runtime_abi --features cpu && cargo test -p ep-adapter-local-ai --test schema_dfa --features cpu && cargo test -p ops-agent --test f55_ai_package_install`. Expected: FAIL because the adapter, verifier and installer do not exist.
- [ ] **Step 4: Freeze the Rust dependency surface.** Add the exact workspace declarations shown above, direct workspace dependencies in `ep-adapter-local-ai`, the compile-time `cpu|cuda` mapping, and updated `Cargo.lock`. CPU_LOCAL is `--features cpu`; GPU_LOCAL is `--no-default-features --features cuda`. The resolved non-wasm feature graph in both profiles must include `tokenizers/onig` and `onig/onig_sys`; only GPU adds Candle CUDA. Do not add an engine trait registry, dynamic backend loader, runtime engine/profile environment key or fallback implementation.
- [ ] **Step 5: Implement canonical package verification.** Require one CAB of 1..2,147,483,647 exact bytes and reject spanning. Verify outer CAB Authenticode, exact seven-entry set, seven-entry total≤2,130,706,432, `model.gguf`≤2,000,000,000 and every other F-55 per-file bound; strict-deserialize/JCS-check `manifest.jcs.json`; verify detached `manifest.p7s` over those exact JCS bytes and every file SHA-256 before parsing payloads. Parse only required qwen2 GGUF v3 metadata, accept absent rope base only with 10000, require `general.file_type=MOSTLY_Q4_0`, exact root/per-block tensor names/ranks/dtypes/dimensions and manifest context not above GGUF. Load only independent `tokenizer.json`; require exact 151644/151645 decoding. Resolve only `ep-install://ai-model/sha256/<64-lowerhex>` through the compile-time model root and require the regular-file roster to be exactly `package.cab` plus those seven archive entries, no subdirectories/additions; reject duplicate/colliding paths, hardlink/reparse/ADS/root escape, executable/script/native-library content and every file outside the closed roster.
- [ ] **Step 6: Implement the single Candle runtime and exact prompt encoding.** In `candle_qwen2.rs`, construct only Candle 0.11.0 quantized QWEN2 from the verified read-only GGUF mapping and Rust Tokenizers 0.23.1; expose no model/backend selector. Produce input bytes exactly as system start + exact prompt bytes + user start + JCS object with keys `catalog_projection,question` + assistant start; use `encode(add_special_tokens=false)` and decode without special-token skipping/cleanup, padding, truncation or BOS. Reject input-token count + 2048 above the smaller manifest/GGUF context bound as `AI.INPUT.CONTEXT_LIMIT_EXCEEDED` before allocation/generation and without truncation. In `gguf.rs`, compare all metadata/tensors to the verified manifest before allocation. CPU compiles without `cuda`; GPU exists only under `#[cfg(feature = "cuda")]` and reports `GPU_LOCAL`, with no runtime switch.
- [ ] **Step 7: Implement `GREEDY_SCHEMA_DFA_V1`.** Materialize the fixed schema/order as a byte-prefix automaton. At every step, mask every token whose decoded UTF-8 bytes have no DFA transition, select the highest remaining logit and break ties by lower token id. After the first complete root object, mask everything except `151645`; after that EOS stop and strict-deserialize once to `AiQueryPlanV1`. Reject no-valid-token, premature EOS, invalid UTF-8, more than 2048 new tokens, context overflow or deadline without returning partial JSON.
- [ ] **Step 8: Implement atomic installation and the shared exact receipt IPC.** `ops-agent` re-reads the one non-spanned offline CAB into a same-volume temporary directory, verifies the 2,147,483,647-byte package and per-entry/aggregate limits, Authenticode/inner CMS/hash/path, preserves the exact input bytes as fixed `package.cab`, extracts and flushes all seven exact entries, then rereads the closed eight-file roster and proves `SHA-256(package.cab)=package_digest` plus archive-entry↔extracted length/hash equality. It breaks ACL inheritance and grants manage only to SYSTEM/Administrators/`NT SERVICE\ep-ops`, read only to `NT SERVICE\ep-ai`, and only owner/DACL `READ_CONTROL` to `NT SERVICE\ep-core`; core gets no list/read-body/execute ACE. Atomically rename to `C:\ProgramData\EnterprisePlatform\ai\models\<lowerhex-package-digest>\`; failure cleans only the validated temporary root and emits no receipt. After rename send exact `SignedArtifactInstallReceiptV1` through `ops.signed_artifact.install_receipt.v1` over authenticated `ep-ops → ep-core`; only ops has that ACE. Enforce schema 1, AI kind/subject, `ep-install://ai-model/sha256/<digest>`, ref/digest equality and `manifest_digest=SHA-256(exact AiModelPackageManifestV1 JCS)` from the CAB. Core reads only the owner/DACL descriptor, rechecks receipt/config/digest facts, acquires the shared receipt UUID transaction advisory lock and cross-checks both AI/MCP tables; in that transaction insert REGISTERED, advance REGISTERED→VERIFIED and set `verified_at` to that first server transaction time. Return only current `request_id,receipt_id,registered_object_id`; lost-ACK replay excludes current request id but compares schema/kind/package/manifest/root/installed-at/full-subject semantics, echoes the replay request id and returns the same registered id. Divergent/cross-table/type-dependent-manifest reuse fails. Directory scanning/polling, implicit callbacks, arbitrary paths and model activation are forbidden.
- [ ] **Step 9: Implement independent inference re-verification.** `ai-inferer` maps no package at bare startup. Only an exact `AiModelActivationDescriptorV1` may trigger compile-time-root resolution and independent verification of root owner/DACL, exact eight-file roster, `package.cab` Authenticode/package digest/archive roster/inner CMS, every archive-entry↔extracted length/hash equality and execution profile before mmap of the extracted `model.gguf` into shared read-only weights; it has no certification-report field or evidence-directory access and never trusts only receipt/extracted files. Atomically switch only after all checks and return exact `AiModelActivationAckV1` with the same request/package/digests/profile, ABI 1 and verification time. Exact deactivation request closes new compose admission, drains/cancels under the deadline, clears active identity/weights/all KV/prefix state and returns the prior package id/time ACK. A bad activate root never changes the currently loaded identity.
- [ ] **Step 10: Add dependency, source and supply-chain negatives.** `xtask/tests/f55_ai_runtime_dependency.rs` parses Cargo metadata, `Cargo.lock`, `cargo tree -e features` fixtures and CycloneDX fixtures. It requires exactly Candle core/transformers 0.11.0 plus direct tokenizers 0.23.1, requires `tokenizers/onig` and `onig/onig_sys` in both non-wasm profiles, and requires Candle `cuda` only in GPU. It rejects `tokenizers/http`, `hf-hub`, other HTTP clients, Python/PyO3, ONNX/ORT, llama.cpp/ggml or another inference-engine FFI, custom native ops, script engines and a second inference backend. The onig/onig_sys native build is an explicit whitelist entry whose source/digest/license must appear in vendor, Cargo.lock, SBOM, signature evidence and static scan. `archcheck` also rejects DB/KMS/network/file-write APIs in the adapter/inferer; only `package/verify.rs` may open verified model files read-only.
- [ ] **Step 11: Verify the CPU baseline.** Run: `cargo test -p ep-adapter-local-ai --features cpu && cargo test -p ops-agent --test f55_ai_package_install && cargo test -p ep-adapter-ipc --test f55_signed_artifact_install_receipt_pipe_windows --target x86_64-pc-windows-msvc && cargo test -p core-server --test f55_ai_model_install_receipt && cargo test -p ep-xtask --test f55_ai_runtime_dependency && cargo xtask archcheck`. Then run: `cargo tree -p ep-adapter-local-ai -e features --features cpu`. Expected: PASS; the tree contains the exact frozen runtime versions, required onig/onig_sys closure and no forbidden dependency/feature.
- [ ] **Step 12: Compile the separately certified GPU variant.** On the Stage 14 Windows CUDA agent run: `cargo test -p ep-adapter-local-ai --test runtime_abi --no-default-features --features cuda --target x86_64-pc-windows-msvc --no-run` and `cargo tree -p ep-adapter-local-ai -e features --no-default-features --features cuda`. Expected: PASS with the same exact versions and only the `cuda` delta; this artifact is separately Authenticode-signed, receives its own SBOM and resource certification, and is never substituted for the CPU artifact.
- [ ] **Step 13: Commit.** Run: `git add Cargo.toml Cargo.lock crates/adapter/local-ai crates/adapter/ipc/src/signed_artifact_install.rs crates/adapter/ipc/tests/f55_signed_artifact_install_receipt_pipe_windows.rs apps/ops-agent/src/ai_model_package.rs apps/ops-agent/src/targets.rs apps/ops-agent/tests/f55_ai_package_install.rs apps/core-server/src/platform/signed_artifact_install_receipts.rs apps/core-server/src/wiring/ai_models.rs apps/core-server/tests/f55_ai_model_install_receipt.rs xtask/src/archcheck/deps.rs xtask/src/archcheck/source.rs xtask/tests/f55_ai_runtime_dependency.rs && git commit -m "feat(ai): freeze and install the Candle runtime"`.

### Task 4: Add the isolated `ep-ai` service process and IPC boundary

**Files:**
- Modify: `crates/adapter/ipc/src/lib.rs`
- Create: `crates/adapter/ipc/src/ai.rs`
- Create: `apps/ai-inferer/Cargo.toml`
- Create: `apps/ai-inferer/src/main.rs`
- Create: `apps/ai-inferer/src/service.rs`
- Create: `apps/ai-inferer/src/wiring.rs`
- Create: `apps/core-server/src/wiring/ai_activation.rs`
- Modify: `crates/platform/runtime/src/process.rs`
- Modify: `crates/platform/runtime/src/selfcheck/registry.rs`
- Modify: `crates/platform/runtime/src/config/sections.rs`
- Modify: `deploy/register-services.ps1`
- Modify: `deploy/resource-limits.toml`
- Modify: `scripts/verify-resource-limits.ps1`
- Modify: `scripts/verify-connection-budget.ps1`
- Test: `crates/adapter/ipc/tests/f55_ai_pipe_windows.rs`
- Test: `apps/ai-inferer/tests/f55_service.rs`
- Test: `apps/core-server/tests/f55_ai_activation_reconcile.rs`
- Test: `xtask/tests/f55_service_roster.rs`
- Test: `crates/platform/runtime/tests/f55_ai_selfcheck_scope.rs`

**Interfaces:**
- Consumes: `AiInferencePort`, existing length-prefixed IPC codec and Windows service host.
- Produces: `AiPipeClient`, `AiPipeServer`, `AiModelActivationCoordinator`, fail-closed compose-readiness latch, ninth `ProductProcess::AiInferer`, `APP_AI`, bounded queue 30 and max concurrency 15.

```toml
# apps/ai-inferer/Cargo.toml
[features]
default = ["cpu"]
cpu = ["ep-adapter-local-ai/cpu"]
cuda = ["ep-adapter-local-ai/cuda"]
```

```rust
#[cfg(any(all(feature = "cpu", feature = "cuda"), not(any(feature = "cpu", feature = "cuda"))))]
compile_error!("ai-inferer requires exactly one of cpu or cuda");

#[cfg(all(feature = "cpu", not(feature = "cuda")))]
pub const AI_EXECUTION_PROFILE: AiExecutionProfileV1 = AiExecutionProfileV1::CpuLocal;
#[cfg(all(feature = "cuda", not(feature = "cpu")))]
pub const AI_EXECUTION_PROFILE: AiExecutionProfileV1 = AiExecutionProfileV1::GpuLocal;

pub const AI_ALLOWED_OPERATIONS: [(&str, &str); 5] = [
    ("NT SERVICE\\ep-core", "ai.query_plan.compose.v1"),
    ("NT SERVICE\\ep-core", "ai.model.activate.v1"),
    ("NT SERVICE\\ep-core", "ai.model.deactivate.v1"),
    ("NT SERVICE\\ep-ops", "health.get.v1"),
    ("NT SERVICE\\ep-ops", "metrics.snapshot.v1"),
];
```

- [ ] **Step 1: Write failing Windows IPC and reconcile tests.** Test correct core compose/activate/deactivate, ops health/metrics, every other SID, worker/plugin/integ/portal accounts, pipe pre-creation race, wrong peer token, wildcard operation, unknown operation, slow handshake, slow first frame, 1 MiB overflow and 120-second absolute timeout. Assert exactly 51 instances with non-borrowing quotas: core compose data plane 45, core model control plane 2, ops 2 and accept-only 2; assert capacity rejection before a complete model input is read. Disconnect cancels a queued item immediately; for a running item it propagates cooperative cancellation, zeroizes that invocation's decoder/KV and gets a termination ACK within 2,000 ms. Prove the build has no per-invocation child process and never attempts to kill a Rust thread. A fixture that ignores cancellation must close compose readiness and terminate the entire `ai-inferer`/`APP_AI` Job; every other in-flight request in that process is cancelled, zeroized and fails `AI.MODEL_PACKAGE.NOT_ACTIVE`, with no continued compute to the 120-second deadline, late result or surviving state. Supervised restart remains NOT_ACTIVE until the unique DB ACTIVE package is independently reverified and a fresh activate ACK plus current certification gate pass; the old ACK cannot reopen readiness. `f55_ai_activation_reconcile.rs` covers DB-active/no-ACK, ACK-active/no-DB, request/package/manifest/ABI/profile/time mismatch, core certification gate failure before activation, independent package verifier failure, disconnect and either-process restart. It also proves a second package cannot activate while another row is ACTIVE and that an explicit disable plus matching deactivation ACK precedes a new-package activation. Verification/ACK/deadline failure drives the attempted ACTIVE row to DISABLED；disconnect/restart closes readiness, and no case opens compose until a fresh exact ACK matches the current unique DB row. Add a connection-budget fixture that fails while the Stage 2 eight-process roster remains unchanged, then requires the exact nine-process roster with `ai-inferer` present and all five SQL/replication counts zero; the aggregate remains resident 37, migration/emergency temporary 10, safety headroom 5 and hard peak 52. `f55_service_roster.rs` must first fail on the Stage 1 eight-service registration snapshot, then compare `ProductProcess`, packaged product executables and `deploy/register-services.ps1` as exact nine-member sets and assert the AI tuple `service=ep-ai, executable=ai-inferer.exe, account=NT SERVICE\ep-ai` occurs once.
- [ ] **Step 2: Run on the Windows test agent.** Run: `cargo test -p ep-adapter-ipc --test f55_ai_pipe_windows --target x86_64-pc-windows-msvc`. Expected: FAIL because `ep-ai` is absent.
- [ ] **Step 3: Implement the pipe boundary.** Build the DACL before accepting, verify peer service token before reading business payload, apply 5/10/120-second deadlines, reject oversize frames before allocation and dispatch only the five exact operations. Pipe disconnect or caller cancellation removes an unstarted queue item immediately; a running invocation receives cooperative cancellation and must acknowledge termination plus decoder/KV zeroization within 2,000 ms. On missed ACK, close readiness and terminate the whole `ai-inferer`/`APP_AI` Job rather than a nonexistent per-invocation process or Rust thread; all same-process in-flight calls fail NOT_ACTIVE and zeroize. Restart and recover capacity only after independent package verification, a fresh activate ACK and current certification gate pass; never deliver any affected result or reuse the prior ACK.
- [ ] **Step 4: Add the ninth service.** Register `ProductProcess::AiInferer`, service name `ep-ai`, account `NT SERVICE\ep-ai`, no network token, no DB/KMS environment, no interactive logon and Windows Error Reporting configured for minimal metadata only with full dumps disabled. The one authoritative native-service installer path is `deploy/register-services.ps1`; extend its closed product roster from eight to nine with exactly one `ep-ai → ai-inferer.exe → NT SERVICE\ep-ai` entry, using the same automatic-start, recovery, quoted-binary-path, DACL and uninstall/upgrade discipline as the existing product-service entries. The script may have operational helper rows for PostgreSQL/reverse proxy, but the product subset must equal `ProductProcess` exactly and must not infer services by scanning a directory. Extend the no-SQL selfcheck scope from five to the exact six-member set `portal-gateway|integration-gateway|plugin-host|archive-writer|backup-writer|ai-inferer`: every SQL-session check returns `NotApplicable` for AI without opening a connection or reading credentials, while every applicable non-SQL configuration, secret, package, directory and resource check still runs; `f55_ai_selfcheck_scope.rs` rejects an absent/extra member and any blanket “all checks N/A” shortcut. In the same change, update `scripts/verify-connection-budget.ps1` from the Stage 2 eight-process snapshot to the exact nine-process roster by adding `ai-inferer` with `Rw=0, Ro=0, Worker=0, Ops=0, replication=0, total resident SQL=0`; keep the frozen aggregate at resident 37, migration/emergency temporary 10, safety headroom 5 and hard peak 52, and fail on a missing, extra or nonzero AI row. Forward only the compile-time `cuda` feature shown above and report the matching constant profile; reject a loaded package/certification for the other profile. Keep health/metrics non-business and never expose another listener or runtime engine/profile switch.
- [ ] **Step 5: Implement fail-closed activation coordination.** `AiModelActivationCoordinator::activate` requires no different ACTIVE row and first verifies the selected CERTIFIED row's signed Stage 14 report ref/digest in core. It then completes CERTIFIED→ACTIVE, keeps readiness closed, builds the exact descriptor with only a fresh request id plus current package/root/manifest/profile identities, calls `ai.model.activate.v1`, and compares request id, package id/digests, ABI/profile and verification time to a fresh DB read before opening readiness. Certification evidence never enters the descriptor/ACK or ai-inferer. Switching packages is never an implicit replacement: first call `disable`, close readiness, commit the old ACTIVE→DISABLED and verify exact deactivation ACK; only then may the selected CERTIFIED row activate. Independent verification failure, negative ACK, mismatch or activation deadline commits that attempted ACTIVE row to DISABLED, keeps compose closed and alerts; it never auto-selects another package. Startup/reconnect clears in-memory ACK facts and reconciles from DB; DB absent sends deactivate, DB ACTIVE sends a fresh activate, while pipe loss alone leaves the row/history intact but readiness closed until a fresh exact ACK. No HTTP route can forge or bypass the latch.
- [ ] **Step 6: Add the resource unit.** Repurpose the former no-carrier “built-in search index” quota row as the AI row with CPU intent 10, memory 10% and IO intent 8; built-in search remains charged to its actual caller process. Add `APP_AI` as the ninth Job Object and no tenth quota row. Its static memory value is computed by `floor(0.095 × certified_host_ram_bytes)`; reject CPU-rate, burst, IO-weight and free-form absolute overrides. The verifier reads back `JOB_OBJECT_LIMIT_JOB_MEMORY` and DACL.
- [ ] **Step 7: Implement bounded inference with per-request KV isolation.** A semaphore admits 15 calls, a queue accepts 30 waiting calls, overflow returns `AI.INFERENCE.CONCURRENCY_LIMIT`, deadline returns the existing contained timeout mapping, and request-local allocation failure fails that request. The verified weight/tensor mapping is one immutable shared `Arc`; every admitted request allocates fresh mutable decoder/KV state owned by `(request_id,turn_id)` and zeroizes/drops it on success, rejection, cancellation, disconnect, timeout or error. Queued cancellation is immediate; running cancellation checks at each cooperative boundary, clears the request state and acknowledges within 2,000 ms. If it does not, the supervisor closes readiness and terminates the entire `ai-inferer`/`APP_AI` Job; all other in-flight calls are cancelled/zeroized and fail NOT_ACTIVE. Restart reconstructs service capacity but readiness stays closed until fresh package verification, activation ACK and certification gate; it does not wait for 120 seconds, publish any affected late result or reuse old ACK state. A scheduler may combine only requests at the same forward step whose `legal_entity_id,user_id,security_context_digest,catalog_projection_digest,model_package_digest,prompt_template_version` all match; batching shares no mutable state, batch completion retains no token/KV/prefix block, and no state persists or serves a later request.
- [ ] **Step 8: Verify.** Run: `cargo test -p ep-adapter-ipc --test f55_ai_pipe_windows --target x86_64-pc-windows-msvc && cargo test -p ai-inferer --test f55_service --target x86_64-pc-windows-msvc && cargo test -p core-server --test f55_ai_activation_reconcile --target x86_64-pc-windows-msvc && cargo test -p ep-platform-runtime --test f55_ai_selfcheck_scope && cargo test -p ep-xtask --test f55_service_roster && powershell -NoProfile -File scripts/verify-resource-limits.ps1 && powershell -NoProfile -File scripts/verify-connection-budget.ps1`. Expected: PASS with five exact operations, fail-closed DB/ACK reconciliation, exact equality among the nine-member process/package/service rosters, exactly six no-SQL processes with only SQL checks N/A, nine product processes, nine resource units, and a nine-row connection roster in which `ai-inferer` has zero connections while the aggregate remains `37/10/5/52`.
- [ ] **Step 9: Commit.** Run: `git add crates/adapter/ipc apps/ai-inferer apps/core-server/src/wiring/ai_activation.rs apps/core-server/tests/f55_ai_activation_reconcile.rs crates/platform/runtime/src/process.rs crates/platform/runtime/src/selfcheck/registry.rs crates/platform/runtime/src/config/sections.rs crates/platform/runtime/tests/f55_ai_selfcheck_scope.rs deploy/register-services.ps1 deploy/resource-limits.toml scripts/verify-resource-limits.ps1 scripts/verify-connection-budget.ps1 xtask/tests/f55_service_roster.rs && git commit -m "feat(ai): isolate and activate the local inference service"`.

### Task 5: Validate plans, render confirmation, and sign five-minute tokens

**Files:**
- Create: `crates/application/reporting/src/ai/mod.rs`
- Create: `crates/application/reporting/src/ai/catalog.rs`
- Create: `crates/application/reporting/src/ai/validation.rs`
- Create: `crates/application/reporting/src/ai/confirmation.rs`
- Create: `crates/application/reporting/src/ai/token.rs`
- Modify: `crates/application/reporting/src/lib.rs`
- Test: `crates/application/reporting/tests/f55_ai_query_plan.rs`
- Test: `crates/application/reporting/tests/f55_ai_plan_properties.rs`

**Interfaces:**
- Consumes: current `SecurityContext`, reporting dataset registry, field grants/classification, model package/prompt digests and `AiInferencePort`.
- Produces:

```rust
pub struct ValidatedAiQueryPlanV1 {
    pub plan: AiQueryPlanV1,
    pub plan_digest: Sha256Digest,
}

pub struct AiPlanClaimsV1 {
    pub schema_version: u16,
    pub turn_id: Uuid,
    pub legal_entity_id: Id<LegalEntity>,
    pub user_id: Id<UserAccount>,
    pub source_session_id: Uuid,
    pub source_device_id: Uuid,
    pub query_plan: AiQueryPlanV1,
    pub security_context_digest: Sha256Digest,
    pub catalog_projection_digest: Sha256Digest,
    pub model_package_id: Uuid,
    pub model_package_digest: Sha256Digest,
    pub prompt_template_version: String,
    pub prompt_template_digest: Sha256Digest,
    pub issued_at: DateTime<Utc>,
    pub expires_at: DateTime<Utc>,
}

pub const AI_PLAN_TOKEN_PREFIX: &str = "epai1";
pub const AI_PLAN_TOKEN_SIGNING_PURPOSE: &str = "AI_PLAN_TOKEN_V1";

pub fn validate_ai_query_plan(
    catalog: &AiCatalogProjectionV1,
    candidate: AiQueryPlanV1,
) -> Result<ValidatedAiQueryPlanV1, AppError>;

pub trait AiPlanTokenCodec: Send + Sync {
    fn issue(&self, claims: &AiPlanClaimsV1) -> Result<SensitiveString, AppError>;
    fn verify(&self, token: &SensitiveString, now: DateTime<Utc>)
        -> Result<AiPlanClaimsV1, AppError>;
}
```

- [ ] **Step 1: Write the validator truth table.** Add one positive case and explicit negative cases for duplicate/unsorted catalog, >64 datasets, >256 fields, >512 KiB catalog, wrong prompt version/digest/text, prompt >64 KiB, multi-dataset reference, unknown field, invalid aggregation, join/SQL-like field, duplicate entries, OR/NOT, wrong literal type/count, invalid result code, empty select, and each list/limit bound.
- [ ] **Step 2: Run focused tests.** Run: `cargo test -p ep-app-reporting --test f55_ai_query_plan && cargo test -p ep-app-reporting --test f55_ai_plan_properties`. Expected: FAIL because validators and token codec are absent.
- [ ] **Step 3: Implement canonical catalog projection.** Include only datasets and fields visible by dataset permission, field permission and clearance; sort by code; include no business values, samples, statistics, row counts, distinct/min/max values, object instance names or attachment content; leave record `data_scope` for execute-time predicate injection.
- [ ] **Step 4: Implement validation and mapping.** Normalize only after strict validation; inject no model-provided scope; convert the validated single dataset into the existing reporting `QueryPlan`; keep current row predicate as a separate outer-AND input.
- [ ] **Step 5: Implement deterministic confirmation and the exact token codec.** Renderer uses only validated labels/operations. Strict claims contain exactly the fields in the interface above, use RFC3339 UTC second precision and `expires_at=issued_at+300s`, and reject unknown fields. Encode only `epai1.<base64url-no-pad(key_version_utf8)>.<base64url-no-pad(JCS(claims))>.<base64url-no-pad(signature_p1363_64)>`; sign `SHA-256(ASCII("EP-AI-PLAN-TOKEN-V1\0") || ASCII(first_three_segments))` through the legal-entity `KmsBackend` key with purpose `AI_PLAN_TOKEN_V1`, P-256 canonical low-S P1363 raw64 output, and create no draft table. Only ACTIVE keys sign; retired non-revoked keys verify through their last legal token expiry plus 60 seconds, while revoked keys fail immediately.
- [ ] **Step 6: Verify property and key-lifecycle tests.** Generate arbitrary JSON, field sets and security deltas; accepted plans must satisfy all closed bounds, and any changed framing/claim/signature byte, cross-entity key version, issued-at more than 60 seconds in the future, expired token, high-S/DER/wrong-length signature, retired-key window overrun or revoked key must fail with `AI.QUERY_PLAN.TOKEN_INVALID_OR_EXPIRED`.
- [ ] **Step 7: Run tests.** Run: `cargo test -p ep-app-reporting --test f55_ai_query_plan && cargo test -p ep-app-reporting --test f55_ai_plan_properties`. Expected: PASS.
- [ ] **Step 8: Commit.** Run: `git add crates/application/reporting/src crates/application/reporting/tests && git commit -m "feat(reporting): validate and sign AI query plans"`.

### Task 6: Implement compose, confirm, execute, and OpenAPI

**Files:**
- Create: `crates/application/reporting/src/ai/compose.rs`
- Create: `crates/application/reporting/src/ai/execute.rs`
- Create: `apps/core-server/src/reporting/mod.rs`
- Create: `apps/core-server/src/reporting/ai_query_plans.rs`
- Modify: `apps/core-server/src/main.rs`
- Modify: `apps/core-server/src/wiring/mod.rs`
- Modify: `crates/platform/audit/src/object_registry.rs`
- Create: `docs/openapi/ai-reporting.v1.yaml`
- Modify: `xtask/Cargo.toml`
- Create: `xtask/tests/f55_ai_openapi.rs`
- Test: `apps/core-server/tests/f55_ai_http.rs`
- Test: `crates/application/reporting/tests/f55_ai_security_recheck_pg.rs`
- Test: `crates/platform/audit/tests/f55_object_registry.rs`

**Interfaces:**
- Consumes: `AiInferencePort`, fail-closed activation readiness latch, catalog/validator/token from Task 5, current authz facts, audit writer and existing `ep_analyst_ro` query executor.
- Produces:

```rust
pub struct ComposeAiQueryPlanRequest {
    pub question: String,
    pub locale: String,
}

pub const AI_COMPOSE_LOCALE: &str = "zh-CN";

pub struct ComposeAiQueryPlanResponse {
    pub turn_id: Uuid,
    pub plan_token: SensitiveString,
    pub expires_at: DateTime<Utc>,
    pub human_confirmation: AiHumanConfirmation,
    pub model: AiModelIdentity,
}

pub struct AiHumanConfirmation {
    pub dataset_label: String,
    pub selected_fields: Vec<String>,
    pub filters: Vec<String>,
    pub grouping: Vec<String>,
    pub aggregation: Vec<String>,
    pub ordering: Vec<String>,
    pub limit: u16,
}

pub struct AiModelIdentity {
    pub code: String,
    pub version: String,
}

pub struct ExecuteAiQueryPlanRequest {
    pub plan_token: SensitiveString,
    pub confirmed: bool,
}

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum AiResultDataTypeV1 { Text, Integer, Decimal, Boolean, Date, DateTime, Uuid }

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum AiResultAggregationV1 { Sum, Count, Min, Max, Avg }

#[derive(Clone, Debug, Serialize)]
#[serde(deny_unknown_fields)]
pub struct AiResultColumnV1 {
    pub result_code: String,
    pub display_name: String,
    pub data_type: AiResultDataTypeV1,
    pub aggregation: Option<AiResultAggregationV1>,
}

#[derive(Clone, Debug, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ExecuteAiQueryPlanResponseV1 {
    pub turn_id: Uuid,
    pub dataset_code: String,
    pub columns: Vec<AiResultColumnV1>,
    pub rows: Vec<Vec<serde_json::Value>>,
    pub row_count: u32,
    pub truncated: bool,
    pub executed_at: DateTime<Utc>,
}

pub async fn compose_ai_query_plan(
    ctx: &SecurityContext,
    input: ComposeAiQueryPlanRequest,
) -> Result<ComposeAiQueryPlanResponse, AppError>;

pub async fn execute_ai_query_plan(
    ctx: &SecurityContext,
    input: ExecuteAiQueryPlanRequest,
) -> Result<ExecuteAiQueryPlanResponseV1, AppError>;
```

F-56 license cases are mandatory parts of Step 1: for both routes, table-drive `Active|ExpiringSoon|GracePeriod|Restricted`、missing current、tampered CMS、wrong deployment、wrong legal-entity scope and current-slot ambiguity. Only the first three statuses with exact `F55LocalAi` may reach catalog/model work; every other case returns the existing authorization/not-ready envelope before pipe/query and preserves all license/model rows. A historically verified entitlement makes `purchased=true` only and cannot make a Restricted current grant callable; a true feature flag, module code or hand-written evidence without the signed entitlement must still fail.

- [ ] **Step 1: Write failing HTTP, idempotency, capacity, size, audit and permission-scope cases.** Cover exact two paths and `X-Client` enum: accept `win|mac|ios|android|ops|server_admin`, reject `portal|mcp`/unknown/alias/case variants. A `server_admin` positive must still have ReportingReportPrint+Read, the route's exact AI permission/object scope and license; remove each fact and prove client identity alone grants nothing. Cover literal `zh-CN` success, every other locale/case variant rejection, empty/2001-scalar question, caller-supplied dataset/field/model/prompt/decoder parameter rejection, missing AI permissions, required boolean `confirmed` absent/wrong-type, `confirmed=false` as a valid request shape returning HTTP 409 `AI.QUERY_PLAN.CONFIRMATION_REQUIRED` rather than schema-level 400, expired/tampered token and anti-enumeration response. Compose returns `AI.QUERY_PLAN.NOT_VISIBLE_OR_DENIED` only after plan JSON/shape/type/operator/limit validation succeeds but a proposed dataset/field code is absent from the current permission-pruned catalog; actual absence and invisibility share the same 404 envelope/timing and issue zero token. Execute rejects that code from its route closure: malformed/tampered/expired token maps only to `AI.QUERY_PLAN.TOKEN_INVALID_OR_EXPIRED`, while a valid token whose bound dataset/field/catalog/authorization facts changed maps only to `AI.QUERY_PLAN.SECURITY_CONTEXT_CHANGED`, never 404. Prove compose and execute are the exact two-member authenticated-read-only POST exemption: both bypass the write-idempotency middleware, both reject any `Idempotency-Key` as `AI.QUERY_PLAN.IDEMPOTENCY_NOT_ALLOWED`, and neither reads nor writes `platform_msg.idempotency_keys`; a third action POST remains under the ordinary rule. Every compose/execute request, response and nested object fixture rejects one added unknown field before use. Freeze per-route errors: both include `PLATFORM.REQUEST.INVALID_PAYLOAD` and `PLATFORM.REQUEST.HEADER_MISSING`; execute additionally includes `REPORTING.ANALYTIC_QUERY.STATEMENT_TIMEOUT|RESOURCE_LIMIT_EXCEEDED|RESULT_TOO_LARGE` and `PLATFORM.SYSTEM.SYNC_TIMEOUT`, besides their exact applicable F-55 AI errors, and a union-only route superset fails. Execute fixtures distinguish `limit+1` row truncation from bytes: an exact compact UTF-8 JSON success of 8,388,608 bytes passes; when the next complete JSON token would exceed it, the bounded sink discards the entire result and returns existing `REPORTING.ANALYTIC_QUERY.RESULT_TOO_LARGE` with no rows/partial envelope and never `truncated=true`. Assert compose completes bounded body/header/session/legal-entity checks, then owns an independent fair 45-slot route gate and never acquires/releases the ordinary 8-second/20-slot business limiter; 15 requests run, 30 wait and request 46 is rejected without catalog/pipe work. Freeze a 122000-ms route Tower timeout; internal 120000 ms starts on semaphore entry and includes queue+IPC+inference, while the final 2000 ms permits only cancel/zeroize/error-envelope work. Both deadlines map `PLATFORM.SYSTEM.SYNC_TIMEOUT`; disconnect/timeout cancels the same pipe invocation and queued work ends immediately. A running call first cooperates and zeroizes; if it misses the 2,000-ms ACK, tests prove whole-service `ai-inferer`/`APP_AI` termination, all same-process in-flight calls fail NOT_ACTIVE and zeroize, and readiness stays closed through restart until fresh verification/activate ACK/gate, with no per-invocation process, old-ACK reuse or late result. Assert fixed permission ids `...0310|...0311`, distinct code/object types, VIEW actions, bindings `...0504|...0505` to `(reporting,datasets,...,min_security_level)` and zero seeded role grants. Prove coarse role+permission denial occurs before inference; after a model proposes a dataset, object scope/classification/field/record denial occurs before token issuance. Audit fixtures require compile-time object `reporting.ai.query_turn`, exact two actions and strict twelve-field after; COMPOSED has all fields nonnull, EXECUTION_ATTEMPTED has only `question_sha256=NULL`, before is NULL, and any plaintext/extra field or database/string registry fails. Add DB ACTIVE without ACK, ACK without DB ACTIVE, stale ACK after activate/deactivate, pipe disconnect and restart; all return model-not-ready with zero inference/query.
- [ ] **Step 2: Run the HTTP suite.** Run: `cargo test -p core-server --test f55_ai_http`. Expected: FAIL with routes missing.

For Steps 3–4, every occurrence of “license” means exactly one fresh call to the F-56 method and entitlement above, after rebuilding the current legal entity and before catalog/model/query work. Compose and execute each re-read independently; the plan token never snapshots or substitutes for the execute-time F-56 check. This implementation must import the F-56 contract, not add a local repository, parser, cache, status enum or payload.

- [ ] **Step 3: Implement compose in the frozen idempotency, authorization, capacity and audit order.** Put compose in the closed two-route authenticated-read-only POST exemption so the write-idempotency middleware never reads or writes `platform_msg.idempotency_keys`; reject any `Idempotency-Key` as `AI.QUERY_PLAN.IDEMPOTENCY_NOT_ALLOWED`. Exempt compose from the ordinary 8-second/20-slot layer. After ordinary bounded body/header/session/legal-entity validation, acquire the dedicated fair fixed 45-slot semaphore, hold it through response finalization, and enforce the 15-running/30-queued downstream split; overflow maps to `AI.INFERENCE.CONCURRENCY_LIMIT` and capacity is not configurable. Start the internal 120000-ms deadline at semaphore entry so it includes queue, IPC and inference; wrap it in the 122000-ms route Tower timeout whose remaining 2000 ms is only cooperative cancel/zeroize/envelope time. Both timeout paths cancel the same pipe invocation and map `PLATFORM.SYSTEM.SYNC_TIMEOUT`; neither waits out 120 seconds after disconnect. A running call that misses cooperative termination at 2,000 ms closes readiness and terminates the whole `ai-inferer`/`APP_AI` Job; every other in-flight call fails NOT_ACTIVE and zeroizes. Restart keeps readiness closed until independent package verification, a fresh activate ACK and current certification gate pass, and never reuses old ACK or returns any affected result. Reject `locale != "zh-CN"` before catalog/model work; rebuild current context and first require legal entity, license, `ReportingReportPrint + Read`, role and coarse `reporting.ai_analysis.compose` permission. Require readiness ACK=ACTIVE, build the already field/clearance-trimmed catalog/prompt, call inference once and strictly validate its dataset. Then use that dataset id with the compose object-scope binding to evaluate scope, classification, fields and current record predicate before audit/token; never use the model-package row's security level for analysis data. Generate UUIDv7 `turn_id`, register `reporting.ai.query_turn` only in `crates/platform/audit/src/object_registry.rs`, and before token/response commit exactly one `AI_QUERY_PLAN_COMPOSED` with before NULL and strict after fields `schema_version=1,phase="COMPOSED",turn_id,request_id,trace_id,question_sha256,query_plan_sha256,catalog_projection_digest,security_context_digest,model_package_id,model_package_digest,prompt_template_version,prompt_template_digest`; all are nonnull and `turn_id=object_id`. Any post-model denial is anti-enumerating and produces no token/query; audit failure returns before token response. Never store question/filter/plan/SQL/result/token/prompt/model-output plaintext.
- [ ] **Step 4: Implement execute.** Put execute in the same exact two-route idempotency exemption, perform zero idempotency-table reads/writes, reject any `Idempotency-Key` as `AI.QUERY_PLAN.IDEMPOTENCY_NOT_ALLOWED`, require an ordinary boolean `confirmed`; missing/wrong type is invalid payload, false reaches application logic and returns 409 `AI.QUERY_PLAN.CONFIRMATION_REQUIRED`, and only true continues; rebuild context and recheck session/device/account/legal entity/license/both permissions/capability/role/duty plus the execute object-scope binding against the token dataset id, clearance/field/record/dataset/model/prompt/token facts. Require token model identity/digest, fresh unique ACTIVE row and current activation ACK to match. Before SQL, commit exactly one `AI_QUERY_PLAN_EXECUTION_ATTEMPTED` on the token's same `turn_id`/object with before NULL and the same strict after field set, `phase="EXECUTION_ATTEMPTED"`, `question_sha256=NULL`, every other field nonnull and digest/current facts equal; failure means zero query/result. Inject the current record predicate as outermost AND and execute once through `ep_analyst_ro`, reading `limit+1` with a hard 1000-row cap. Emit exactly `ExecuteAiQueryPlanResponseV1`: unique result codes determine row order/width; INTEGER is JSON integer, BOOLEAN boolean, DECIMAL canonical fixed-point string, DATE/DATE_TIME/UUID/TEXT canonical string and SQL NULL null; reject float, NaN/Infinity, SQL/internal names and row-width/type mismatch. Drop only the `limit+1` extra row and set `truncated=true`; otherwise false with exact row_count. Serialize compact UTF-8 through a bounded sink that counts the full success envelope/columns/rows; before emitting any token that would make the response exceed 8,388,608 bytes, abort serialization, discard all collected result data and return `REPORTING.ANALYTIC_QUERY.RESULT_TOO_LARGE`. Byte overflow never returns partial rows or changes `truncated`. A changed fact produces zero query/model call and no second AI audit row; post-query success/overflow/error adds no further AI audit row.
- [ ] **Step 5: Prove result and idempotency isolation.** Recording ports assert query rows, serialized bytes and derived display text never enter `AiInferencePort`, model cache, audit before/after, tracing attributes, dump payload or idempotency storage; compose and execute perform zero `platform_msg.idempotency_keys` reads/writes in success, rejection, timeout and lost-response paths.
- [ ] **Step 6: Document exact API.** Every object at every nesting level uses `additionalProperties: false`; both routes declare `X-Client` enum exactly `win|mac|ios|android|ops|server_admin` and reject portal/mcp, without implying authorization. Compose `locale` uses a one-value enum containing only `zh-CN`, and compose/execute responses and each route's exact Rust error closure are enumerated. Both routes include `PLATFORM.REQUEST.INVALID_PAYLOAD|PLATFORM.REQUEST.HEADER_MISSING`; execute additionally exposes `REPORTING.ANALYTIC_QUERY.STATEMENT_TIMEOUT|RESOURCE_LIMIT_EXCEEDED|RESULT_TOO_LARGE` and `PLATFORM.SYSTEM.SYNC_TIMEOUT`, with only the applicable F-55 AI codes on each endpoint. Compose alone declares the anti-enumeration 404 `AI.QUERY_PLAN.NOT_VISIBLE_OR_DENIED`; execute explicitly excludes it and distinguishes `AI.QUERY_PLAN.TOKEN_INVALID_OR_EXPIRED` from current-fact `AI.QUERY_PLAN.SECURITY_CONTEXT_CHANGED`. Execute declares `confirmed` as a required ordinary boolean, never `const:true`; false reaches HTTP 409 `AI.QUERY_PLAN.CONFIRMATION_REQUIRED`, while missing/wrong-type is invalid payload. Both routes declare `Idempotency-Key` prohibited with `AI.QUERY_PLAN.IDEMPOTENCY_NOT_ALLOWED`; execute declares the exact dynamic-column result envelope, row-only meaning of `truncated` and all-or-nothing compact-JSON 8,388,608-byte bound. `xtask/tests/f55_ai_openapi.rs` parses the YAML, asserts only the two AI action paths, and compares every header/request/response/error field and per-status error set to the Rust route contract and rejects any union-only superset or stale five-value client enum.
- [ ] **Step 7: Verify.** Run: `cargo test -p core-server --test f55_ai_http && cargo test -p ep-app-reporting --test f55_ai_security_recheck_pg && cargo test -p ep-xtask --test f55_ai_openapi`. Expected: PASS.
- [ ] **Step 8: Commit.** Run: `git add crates/application/reporting/src/ai apps/core-server/src apps/core-server/tests/f55_ai_http.rs crates/application/reporting/tests/f55_ai_security_recheck_pg.rs docs/openapi/ai-reporting.v1.yaml xtask/Cargo.toml xtask/tests/f55_ai_openapi.rs && git commit -m "feat(reporting): expose confirmed local AI analysis"`.

### Task 7: Close the nine assertions, observability, and release gates

**Files:**
- Reference: `docs/superpowers/plans/2026-08-22-license-module-package-implementation.md`
- Create: `testkit/src/ai_containment.rs`
- Modify: `testkit/src/lib.rs`
- Create: `tests/ai_containment/Cargo.toml`
- Create: `tests/ai_containment/src/lib.rs`
- Create: `tests/ai_containment/tests/f55_assertions.rs`
- Reference: `crates/platform/runtime/src/config/sections.rs`（13c Task 1 已预登记）
- Reference: `crates/platform/obs/src/metrics/registry.rs`（13c Task 1 已预登记）
- Reference: `crates/foundation/src/error/codes.rs`（13c Task 1 已预登记）
- Modify: `xtask/src/ci.rs`
- Modify: `xtask/src/sbom.rs`
- Modify: `tools/release-gate/src/main.rs`
- Create: `scripts/verify-ai-runtime-build.ps1`
- Reference: `docs/error-codes.md`
- Reference: `docs/metrics-catalog.md`
- Reference: `docs/config-reference.md`
- Test: `tests/ai_containment/tests/f55_assertions.rs`

**Interfaces:**
- Consumes: complete AI implementation、F-56 signed-grant entitlement contract/common gate plus the Stage 14a bounded-JCS、opaque-ref、signature and fail-closed release-gate contracts；Stage 13c uses only signed fixtures and must not require a real Stage 14b certification report.
- Produces: the exact assertion registry、runtime-facts probe、resource-report verifier and candidate gate fixtures consumed by Stage 14b, plus the exact runtime assertion constant:

```rust
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AiRuntimeReleaseFactsV1 {
    pub schema_version: u16,
    pub release_batch_id: Uuid,
    pub runtime_abi_version: u16,
    pub execution_profile: AiExecutionProfileV1,
    pub artifact_digest: Sha256Digest,
    pub candle_core_version: String,
    pub candle_transformers_version: String,
    pub tokenizers_version: String,
    pub product_features: Vec<String>,
    pub resolved_dependency_features: Vec<String>,
    pub cargo_lock_digest: Sha256Digest,
    pub offline_vendor_digest: Sha256Digest,
    pub sbom_digest: Sha256Digest,
    pub authenticode_subject: String,
    pub authenticode_signature_digest: Sha256Digest,
}

pub const AI_CONTAINMENT_ASSERTION_NAMES: [&str; 9] = [
    "assert_ai_catalog_projection",
    "assert_ai_plan_containment",
    "assert_ai_filter_conjunction",
    "assert_ai_rejection_indistinguishable",
    "assert_ai_inference_containment",
    "assert_ai_cache_partitioning",
    "assert_ai_egress_containment",
    "assert_ai_audit_completeness",
    "assert_ai_model_resource_containment",
];
```

- [ ] **Step 1: Write one positive and one negative fixture per assertion.** Set `[package] name = "ep-ai-containment"` in its Cargo manifest. The rejection-indistinguishable fixture compares bodies byte-for-byte and P95 timing within 5 ms; egress fixture seeds unique canary result strings and scans every prohibited sink; resource fixture verifies formula, signature, ACL, Job Object and non-interference. Cache-partitioning fixtures prove weights are shared read-only but two identical requests still receive distinct mutable KV allocations, no request batch shares KV, and every termination path drops it. Runtime fixtures cover exact ABI/engine/tokenizer/model/tensor-roster/decoder/EOS values, CPU `cpu + tokenizers/onig + onig/onig_sys`, GPU `cuda +` the same onig closure, a changed Cargo.lock, a mismatched SBOM, an unsigned profile artifact and a forbidden transitive dependency. Strict `AiRuntimeReleaseFactsV1` fixtures assert ≤262,144 JCS bytes, exact 15 fields including `schema_version=1` and UUID release batch, lowerhex digests, 1..128-byte array members with ≤256 byte-sorted/deduplicated entries, subject≤512, fixed versions/profile features and exact resolved feature registry. Any ref/signature/expiry/DB row/persisted temporary JSON or renaming to evidence fails. Activation fixtures require DB/ACK equality and fail closed on every mismatch/restart.
- [ ] **Step 2: Run assertions before registration.** Run: `cargo test -p ep-ai-containment --test f55_assertions -- --nocapture`. Expected: FAIL because the assertion registry/gates are absent.
- [ ] **Step 3: Consume and verify the pre-registered AI catalog subset.** 13c Task 1 已在任何子计划开始前一次性登记 F-55 全部 catalog；本步逐项断言其中恰有 F-55 §8 的 11 个 AI 错误、§9.1 的 7 个 AI 配置族和 §9.2 的 7 个 AI 指标/标签闭集，然后把业务实现接到这些既有常量与字段，禁止重复注册、改名或再次改文档目录。Registry tests reject identity-bearing labels and any extra config override for model, prompt or memory percentage；`cargo xtask errorcodes/configdoc` 此时必须能在 MCP/carrier 尚未实现但已预登记的情况下保持全局 PASS，预登记本身不得使那些能力可达。
- [ ] **Step 4: Implement `RG-AI-CONTAINMENT-GREEN`.** Add `ep-release-gate verify --gate <name> --evidence-dir <path>` to the tool CLI. This gate passes only when the array length/order/names match exactly and all eighteen fixtures pass; a skipped fixture, renamed assertion or missing evidence is failure.
- [ ] **Step 5: Implement `RG-AI-RESOURCE-CERTIFIED`.** Strict-parse RFC 8785 `AiResourceCertificationReportV1` at no more than 1,048,576 bytes and its exact sidecar. Require all digest strings lowerhex, ABI 1, concurrency 15, `page_or_swap_observed=false`, calculated hard limit equal to `floor(0.095 × certified_host_ram_bytes)`, host commit peak ≤80% of that limit, CPU GPU fields all null or GPU all nonnull, and byte-sorted/deduplicated `gate_results` exactly equal to the current release AI-resource gate registry with only `PASSED`, including mandatory `gate_code=ai_runtime_release_facts`. At gate execution, build strict `AiRuntimeReleaseFactsV1` only in memory from the signed executable, current Cargo.lock, offline vendor tree, CycloneDX SBOM and resolved feature closure; enforce its 262,144-byte/exact-field/array/version/profile bounds, hash its exact JCS bytes and require that digest to equal the report gate result while report release/profile/artifact fields match. This facts DTO has no ref, independent signature, expiry, DB row or persisted JSON; subsequent CERTIFIED/ACTIVE/post-ACK checks recompute from retained signed product facts and treat only the final signed certification report as evidence. Verify the opaque report ref grammar/resolver/root owner+DACL/file digest and signature preimage/purpose/key ref/version/subject/current-or-retired-nonrevoked release-batch scope; require `signature_p1363_b64url` canonical no-pad round-trip to exactly 64-byte P-256 low-S P1363 and reject array/hex/DER/padding. Bind product build, ai-inferer artifact, package/manifest/model/profile/formula/server/load facts and set `certified_at=finished_at`. CPU certification is mandatory for every release; a shipped/selected GPU artifact additionally needs its own signature, SBOM and full certification and may differ from CPU only by profile, product/CUDA features, artifact/signature/SBOM digests and measured GPU fields. Revoked or recomputation-drifted report facts immediately close readiness.
- [ ] **Step 6: Implement the offline runtime facts probe.** `scripts/verify-ai-runtime-build.ps1` accepts mandatory `-Profile CPU_LOCAL|GPU_LOCAL`, `-ReleaseBatchId`, `-ArtifactPath` and `-SbomPath`; verifies Authenticode offline, hashes current `Cargo.lock` and the frozen vendor directory, checks exact `cargo tree -e features`, confirms CycloneDX versions/features and scans the product artifact/dependency graph for every forbidden engine/downloader/runtime. It builds the strict `AiRuntimeReleaseFactsV1` in memory, canonicalizes/hashes it, returns nonzero on missing signature/input or mismatch, and prints only the release/profile/facts digest summary; it never downloads, signs, writes a facts JSON, creates a ref or leaves a temporary file. The Rust Stage 14 gate independently performs the same computation before signing the final resource report.
- [ ] **Step 7: Test disabled and restricted modes.** With `EP__AI__ENABLED=false`, no compose/execute business route or model process call is reachable, while model/license/module/config-package registry/history remains intact; core closes readiness and sends best-effort deactivate without deleting evidence. With true, a current F-56 signed grant scoped to the legal entity with `F55LocalAi` and status `Active|ExpiringSoon|GracePeriod` + unique ACTIVE certified package + matching independent activation ACK + both AI gates + common `RG-LICENSE-MODULE-LIFECYCLE-GREEN` are jointly required. `Restricted`、missing/tampered grant or historically purchased-only state remains fail-closed before model work and keeps all data.
- [ ] **Step 8: Verify the complete Stage 13c AI candidate.** Run: `cargo test -p ep-ai-containment --test f55_assertions -- --nocapture && cargo xtask errorcodes && cargo xtask configdoc && cargo xtask sbom && cargo run -p ep-release-gate -- verify --gate RG-AI-CONTAINMENT-GREEN --evidence-dir target/release-evidence`. Expected: PASS. Then run `cargo run -p ep-release-gate -- verify --gate RG-AI-RESOURCE-CERTIFIED --evidence-dir target/empty-release-evidence` and `cargo run -p ep-release-gate -- verify --gate RG-LICENSE-MODULE-LIFECYCLE-GREEN --evidence-dir target/empty-release-evidence`; both must be nonzero, proving local fixtures cannot fabricate final resource or license/module certification. This checkbox contains no real-machine probe, signed report or release PASS and therefore completes before the candidate commit.
- [ ] **Step 9: Commit.** Run: `git add testkit tests/ai_containment xtask/src/ci.rs xtask/src/sbom.rs tools/release-gate/src/main.rs scripts/verify-ai-runtime-build.ps1 && git commit -m "test(ai): close containment and resource gates"`. The shared error/config/metric registries and their docs belong to 13c Task 1 and must remain unchanged in this commit.

Stage 14b must obtain and verify the F-56 signed `RG-LICENSE-MODULE-LIFECYCLE-GREEN` result for the same `stage14_run_id/deployment_id/product_build_sha256` before treating either AI gate as applicable and passing; the AI report cannot stand in for the common license/module evidence, and the common evidence cannot stand in for the AI resource report.

**Stage 14b external acceptance (not a Task 7 checkbox and not a prerequisite of Step 9):** Stage 14b first fixes the current release-batch UUID and its evidence orchestrator passes that already validated UUID as typed PowerShell variable `$Stage14bReleaseBatchId`; the value has no default、manual-entry or fallback path. On the Windows Server 2022 agent, before signing the final CPU report, run `powershell -NoProfile -File scripts/verify-ai-runtime-build.ps1 -Profile CPU_LOCAL -ReleaseBatchId $Stage14bReleaseBatchId -ArtifactPath target/release/ai-inferer.exe -SbomPath target/sbom/ep-workspace.cdx.json` and assert no facts JSON/ref/sidecar was created. The Stage 14b gate independently recomputes the same in-memory facts, places their digest in the report's mandatory `ai_runtime_release_facts` result, signs the exact report/sidecar for the same batch, and only then runs `cargo run -p ep-release-gate -- verify --gate RG-AI-RESOURCE-CERTIFIED --evidence-dir target/release-evidence`; require PASS. If and only if the signed release manifest ships GPU_LOCAL, repeat that pre-signing probe with `-Profile GPU_LOCAL -ReleaseBatchId $Stage14bReleaseBatchId -ArtifactPath target/ai-gpu/release/ai-inferer.exe -SbomPath target/sbom/ep-workspace.cdx.json`, generate its separate signed report/sidecar, and require its gate PASS. Stage 14b owns these real probes, signatures and final conclusions; their absence during Stage 13c is the required fail-closed state, not an incomplete candidate commit.

## Completion Evidence

- [ ] Contract extra fields, SQL-like content and multi-dataset plans fail before execution.
- [ ] Exactly one ACTIVE certified model package exists; `ai-inferer` independently verifies it and has no database/network/write capability.
- [ ] Compose and execute perform separate current-security evaluations; every post-compose security change yields zero query and zero model call.
- [ ] Compose and execute independently call only F-56 `ModuleLicenseQuery` for `F55LocalAi`; `Active|ExpiringSoon|GracePeriod` with matching signed scope can proceed, while `Restricted`/missing/tampered/current-slot ambiguity or purchased-only history stops before inference/query and preserves data. The ordinary-business `PLATFORM.LICENSE.RESTRICTED` code does not widen either AI route's exact error closure, and the F-56 recovery exception cannot reach any AI route/model/pipe. No F-55-local license payload, old status enum, feature/module/config/environment shortcut exists.
- [ ] Human confirmation is deterministic; token lifetime is exactly 300 seconds; compose and execute are the only authenticated-read-only POST idempotency exemptions, both reject `Idempotency-Key`, and neither reads or writes idempotency response storage.
- [ ] Execute uses `truncated` only for `limit+1` row truncation; a compact success response over 8,388,608 bytes discards the whole result and returns `REPORTING.ANALYTIC_QUERY.RESULT_TOO_LARGE` with no partial envelope.
- [ ] Result bytes and result-derived text are absent from every AI ingress, cache, log, audit body and dump.
- [ ] Certification accepts only the exact 1-MiB strict report plus signed sidecar and opaque AI-resource ref; release/model/build/server/load/gate facts match, revoked or drifted evidence closes readiness, and no path/report/signature body is exposed.
- [ ] Nine named assertions pass, both AI gates have machine-verifiable evidence, and purchased+enabled applicability also requires the independently signed F-56 `RG-LICENSE-MODULE-LIFECYCLE-GREEN`; absence of either resource or common license/module certification blocks release/enable only.
