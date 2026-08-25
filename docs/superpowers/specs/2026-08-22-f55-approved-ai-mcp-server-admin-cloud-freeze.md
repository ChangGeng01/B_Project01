# F-55 已批准范围冻结：本地 AI、双向 MCP、ServerAdmin 与云承载

> **F-57 现行状态（2026-08-23）：`PARTIALLY_SUPERSEDED`。** 保留本文未冲突的 Windows 隔离、MCP 安全意图、签名证据和客户自控 IaaS 规则；本地模型交付已延期，AI 改为 provider/工具/授权契约，MCP 可经类型化 manifest 扩展，ServerAdmin 改为 Windows Server 权威节点内的服务器控制中心，固定九进程不再是产品契约。现行规则见 [F-57](2026-08-23-f57-governed-automation-fabric-design.md)。

> 裁定日期：2026-08-22
>
> 状态：**仅文档、可直接开发、未实现**
>
> 权威级别：F-55 是本文 AI、MCP、ServerAdmin 与 carrier 范围的现行裁定；许可证、签名模块包、AI/MCP entitlement 与共同许可发布门禁已由后续 F-56 覆盖，相关词句必须按 `2026-08-22-f56-license-signed-module-package-freeze.md` 解释。除该后续覆盖面外，与 F-08、F-09、F-11、F-12、F-51、F-53、历史研究稿、延期目录或阶段计划旧句冲突时，以本文为准。
>
> 实施纪律：本文冻结的是唯一实现口径，不表示代码、迁移、模型包、Windows 实机证据、性能认证或发布已经完成。现有 69 个已执行迁移的文件名、内容与 checksum 一律不改；新增迁移须先登记 `docs/migration-catalog.md` 后再创建。

## 1. 裁定结论与覆盖关系

### 1.1 四组范围正式恢复

本次正式批准并恢复四组能力，均进入首版开发范围：

1. 本地数据分析 AI；
2. 入站与出站双向 MCP；
3. 独立的 ServerAdmin 静态 SPA；
4. 同一单机原生拓扑在客户自控境内物理机或客户自控境内 IaaS VM 上承载。

这四组能力已经具备唯一架构、接口、数据、权限、测试与发布口径，开发者不得再次提出“是否做、做哪一支、结果是否回模型、是否新增独立管理端、是否允许 IaaS”之类的选择题。

### 1.2 被本文覆盖的旧句

| 旧来源 | 旧状态或矛盾 | F-55 现行裁定 |
|---|---|---|
| F-09-2 / F-09-4 | 已选本地推理，但模型交付形态、具体形态、颗粒度与断言尚未冻结 | 按本文第 3 节完整恢复；模型只生成单数据集 `QueryPlan`，签名模型包与独立 `ai-inferer` 形态唯一 |
| F-09-3 | 已选独立服务器端 UI，但只登记了 72→90 的影响面 | 按本文第 5 节交付 `ServerAdmin` 静态 SPA 与完整第五列 |
| F-11-2 | 只准先交无模型第一步，并暂缓决定“结果是否回模型” | 暂缓撤销；结果与任何结果派生文本永不回模型 |
| F-12-1 | 已选“模型不产值，只产计划” | 保留并收紧为单数据集计划、确定性确认与逐次重检 |
| F-12-2 | 已选第九个常驻进程，但要求等 crate 实物再同批改枚举，并曾预估配额表另加第十行 | 条件已转为可施工要求：`apps/ai-inferer`、进程枚举、服务登记、资源单位、门禁必须在同一实现批次一次完成；配额行改按第 3.7 节复用无承载物的旧搜索行，不再新增第十行 |
| F-08 / F-51 “不使用 Hyper-V” | 原裁定针对整个平台/服务端部署与用虚拟机恢复 Linux/cgroup 语义，绝对措辞也会误伤受控第三方插件 guest | 整个平台仍是 Windows 原生单机、不得放进 Hyper-V 客户机；唯一窄例外是第 4.5 节可选 `LOCAL_WINDOWS_HYPERV_CONTAINER` 为单次 MCP 插件调用建立短命 Hyper-V-isolated utility VM，不承载产品服务、数据库或客户主数据卷，不恢复 Linux/cgroup/多机拓扑 |
| 历史 AI 研究稿 | 标题声称 9 条断言，正文实际只有 8 个具名断言；资源实测被写成开发阻断 | 本文第 3.8 节给出恰好 9 个唯一名称；未实测只阻发布，不阻开发 |
| 总体规格第 5.7、11、21.10 章与历史延期句 | 本地 AI 与 MCP 首版延期 | 仅对本文恢复的本地分析 AI 与双向 MCP 失效；其余延期项继续有效 |
| F-53 云句 | 云服务器只笼统写成“部署机器或备份落点”，且不构成在线依赖 | 细化为第 6 节两个等价承载体；允许客户自控境内 IaaS VM 作为在线生产部署机器，但不变成 SaaS 或云托管架构 |

本文不会把历史文档从审计链中删除。旧句只解释当时决策过程，不再是开发阻断，也不得用来恢复另一套实现。

许可证边界另有一条不可省略的后续覆盖：F-55 不定义私有许可 payload、许可状态或 entitlement 存储。AI 只认 F-56 `EntitlementCodeV1::F55LocalAi`，MCP 入站与出站共同只认 `EntitlementCodeV1::F55Mcp`；用于发布 applicability 的 `currently_licensed` 仅在 F-56 current grant 为 `Active|ExpiringSoon|GracePeriod` 且相应法人在签名 scope 内时成立，`Restricted` 一律不成立。`purchased` 只能从 F-56 current/history 中在现行 bundle 下仍为 `TRUSTED`、未标记 `HISTORICAL_SIGNER_REVOKED` 的 grant 重算，不放行业务。AI/MCP 是平台 capability，本身不映射或伪造第十六个 `ModuleCode`；平台入口先判 entitlement，只有请求触及具体业务对象时才继续通过该对象真实 owner module 的 effective-runtime gate，两者互不替代。owner module current 失信只关闭该模块对象路径，不反向改写 deployment `LicenseStatus` 或全局关闭无关 AI/MCP。全局 `Restricted` 的运行后果按 F-56 `LicenseAdmissionEffectV1` 裁切，不能把 applicability 布尔值误作“所有入口关闭”：AI 纯读取/草稿与 `ReadReportAuditBackupExport` 继续可用，有副作用的 MCP 出站、动态 `Write|Approve`、普通写入/审批和新自动化才由通用 admission gate 返回 `PLATFORM.LICENSE.RESTRICTED`。该平台错误发生在领域 handler 前，不扩张 AI/MCP 的 route-specific domain error enum，但经过通用 gate 的端点必须允许此外层拒绝。任何配置布尔值、十五个 `ModuleCode`、旧四态或人工证据都不能替代许可来源。

### 1.3 仍不恢复的范围

下列能力继续延期或不做，不能借 AI、MCP、低代码、插件或云承载变相恢复：

- OCR、向量检索、知识图谱、RAG、外部门户 AI 客服与工业协议本体；
- 公有大模型 API、外部 AI 推理、把业务数据或查询结果发送给模型；
- MCP Sampling、Roots、Tasks、Logging、prompts、legacy HTTP+SSE、动态客户端注册 DCR；
- 通用 SQL、shell、任意文件系统访问、任意 HTTP 代理或通用网络转发；
- SaaS、多租户厂商托管、Kubernetes、HA 集群、云托管数据库、云 KMS；
- Excel 加载项。XLSX/CSV 导入导出仍按既有首版范围交付，不因本文删减，也不由 AI 或 MCP 替代。

## 2. 终态拓扑与信任边界

终态仍是一台 Windows Server 2022 上的原生单机部署。新增一个产品常驻进程，不新增数据库、消息中间件、反向代理端口或第二台必需服务器。

```text
四端 / ServerAdmin / 入站 MCP 客户端
                 │ 同一员工 HTTPS；入站 MCP 仅 POST /mcp
                 ▼
            core-server
              │      │
       \\.\pipe\ep-ai   既有只读分析池 ep_analyst_ro
              │      │
         ai-inferer   PostgreSQL 16
         0 DB / 0 网络 / 0 文件写

core-server / job-worker
      │                              │
\\.\pipe\ep-integ              \\.\pipe\ep-plugin
      │                              │
integration-gateway             plugin-host
远端无状态 Streamable HTTP       本地签名 stdio / 可选 Hyper-V 隔离容器
0 DB/KMS/file/outbox             0 DB；子进程无直连网络与任意文件权限
```

边界固定如下：

- 只有 `core-server` 取得业务数据库连接并构造人类 `SecurityContext`；AI 与 MCP 都不得自报主体、角色、法人、密级或范围。
- `ai-inferer` 是新增的第九个产品常驻进程与独立 Job Object/资源单位。它只监听 DACL 命名管道 `\\.\pipe\ep-ai`。
- MCP 不新增常驻产品进程：入站由 `core-server` 现有员工 HTTPS 承载；远端出站复用 `integration-gateway`；本地出站复用 `plugin-host`。
- `ServerAdmin` 是编译后嵌入 `core-server` 制品的静态 SPA，不运行 Node.js、开发服务器或独立后端服务，不新增监听端口。
- 物理机与 IaaS VM 运行完全相同的 Windows 服务、PostgreSQL、命名管道、ACL、备份、恢复、认证与发布门禁。

## 3. 本地数据分析 AI

### 3.1 唯一产品能力

AI 只把自然语言问题转换成一份**单数据集** `QueryPlan`。模型不取数、不看结果、不计算业务值、不写数据库、不生成 SQL、不生成说明性结论。查询执行、权限裁剪、结果渲染与任何文字说明全部由确定性代码完成。

唯一交互顺序是：

1. `compose`：按当前调用人的权限构造字段目录，将字段目录、问题原文与固定提示模板交给本地模型；
2. 验证：确定性校验模型输出，注入记录范围谓词，规范化并由 `core-server` 签名；
3. 确认：返回人可读的“查什么、筛什么、怎么汇总、最多多少行”说明，由人明确确认；
4. `execute`：重新构造当前 `SecurityContext`，重新检查模块许可、能力域、对象/字段权限、密级、记录范围、RLS 与只读池限制，再执行已签名计划；
5. 呈现：结果只返回调用端，永不进入 `ai-inferer`、提示词、模型缓存、日志、审计正文或幂等响应体。

模型产生的计划不能写回金额、账户、税额、合同字段或任何业务事实。AI 入口不提供保存为规则、流程、报表定义或业务单据的快捷路径；如人希望长期保存，仍须走既有报表设计、审批与发布通道。

### 3.2 crate、进程与模型包

新增或扩展的实现落点固定为：

| 落点 | 唯一职责 |
|---|---|
| `crates/contract/ai` / `ep-contract-ai` | 下述 IPC DTO、枚举与 `AiInferencePort`；只依赖 `ep-foundation` |
| `crates/adapter/local-ai` / `ep-adapter-local-ai` | 模型包验签、只读加载、受约束解码与本地推理运行时；不得含 DB、HTTP 客户端或文件写 API |
| `crates/adapter/ipc` / `ep-adapter-ipc` | `\\.\pipe\ep-ai` 客户端与服务端 framing、DACL、对端 token 核验 |
| `crates/application/reporting` / `ep-app-reporting` | 目录投影、compose/confirm/execute 编排、计划校验与到既有 `QueryPlan` 的映射 |
| `apps/ai-inferer` | Windows 服务入口、模型生命周期、队列、健康与指标；没有任何业务用例 |
| `tests/ai_containment` | 第 3.8 节九条收容断言 |

生产模型包固定为单个签名离线 `.cab`：包含 `manifest.jcs.json`、权重、tokenizer、固定提示模板、许可证与 SBOM；每个文件的 SHA-256 进入 manifest，manifest 使用 detached CMS 签名，CAB 自身走生产 Authenticode。首版 package/CAB hard cap 固定为 2147483647 bytes，任何更大模型必须另立 Runtime ABI 与包装裁定，禁止利用 multi-cab spanning 绕过；该上限既适配 Windows Cabinet 的单文件边界，也符合低成本单机模型目标。开发包可按既有规则使用标记为 DEV 的 ECDSA P-256 签名，DEV 包不得进入生产。模型包只能包含数据文件，不得携带 DLL、脚本、安装钩子或自定义原生代码。

安装由既有生产 Authenticode 签名的离线补丁/模块安装器在客户批准的维护窗口完成，不新增常驻安装服务、在线下载器或管理端上传 API。安装器以 `ops-agent` 的受控本机安装入口在同卷临时目录验 CAB、内层 manifest/signature、逐文件 hash、路径和 DACL；将输入的 exact CAB bytes 以固定名 `package.cab` 保存，同时提取并 flush CAB 的 exact 七项，逐项二次读回证明提取 bytes 等于 archive entry，最后以原子目录重命名发布；失败清理临时目录且不产生收据。模型根唯一为 `C:\ProgramData\EnterprisePlatform\ai\models\<lowerhex-package-digest>\`，根内 regular-file roster 恰为 `package.cab` 加 CAB 的七项提取文件，共八项且无子目录/第九项；`SHA-256(package.cab)=package_digest`。断 ACL 继承，仅 `SYSTEM`、`Administrators` 和 `NT SERVICE\ep-ops` 可管理，`NT SERVICE\ep-ai` 只读；`NT SERVICE\ep-core` 只有读取 owner/DACL 的 `READ_CONTROL`，没有列目录、读取文件正文或执行权限。禁止 reparse point、hardlink、8.3/大小写碰撞、ADS 与从根外打开句柄。`ai-inferer` 每次候选激活先独立复验 `package.cab` Authenticode/package digest/exact archive roster/内层签名，再把 CAB entry 的 length/hash 与七项提取文件逐项相等比较，全部通过后才 mmap 提取的 `model.gguf`；不能只信收据、提取文件或其中一层签名。它对所有业务目录无权限；服务账户 `NT SERVICE\ep-ai` 没有网络 token、数据库凭据、KMS 权限、文件写权限或交互登录权。候选包“已经安装”不等于“已经活动”，唯一活动包的无数据库控制面见第 3.3 节。

安装完成后的数据库登记也只有一条路径。`ops-agent` 作为 `\\.\pipe\ep-core` 已认证客户端调用新增 operation `ops.signed_artifact.install_receipt.v1`；普通管理员、安装器进程和 `ai-inferer/plugin-host` 均无该 ACE。请求的 exact DTO 为：

```rust
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SignedArtifactInstallReceiptV1 {
    pub request_id: RequestId,
    pub schema_version: u16,                 // 必须为 1
    pub receipt_id: Uuid,
    pub artifact_kind: SignedArtifactKindV1,
    pub package_digest: Sha256Digest,
    pub manifest_digest: Sha256Digest,
    pub installed_root_ref: String,
    pub installed_at: DateTime<Utc>,
    pub subject: SignedArtifactInstallSubjectV1,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum SignedArtifactKindV1 {
    AiModelPackage,
    McpSignedStdioPackage,
    #[serde(rename = "MCP_WINDOWS_HYPERV_CONTAINER")]
    McpWindowsHyperVContainer,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "SCREAMING_SNAKE_CASE", deny_unknown_fields)]
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
```

`installed_root_ref` 不是任意路径：三个合法形状分别为 `ep-install://ai-model/sha256/<64-lowerhex>`、`ep-install://mcp-stdio/<lowercase-manifest-version-uuid>/sha256/<64-lowerhex>`、`ep-install://mcp-wcow/sha256/<64-lowerhex>`，末段必须等于 `package_digest`，stdio 的 UUID 必须等于 subject 的 `manifest_version_id`；各消费进程只以编译期根目录解析器把它映射到固定路径。AI subject 只允许 AI kind 且 MCP subject 只允许两种 MCP kind。MCP stdio 的 `container_image_digest/hcs_image_identity` 为空，十个 sandbox profile/SID/WFP 字段全部非空；Hyper-V container 则两个 container 字段非空、十个 sandbox 字段全部为空。两者都必须带实际 root security descriptor 的 SHA-256，且 image digest、manifest version 和已批准 manifest 逐项相等。

`SignedArtifactInstallReceiptV1.manifest_digest` 的 type-dependent 语义唯一固定：`AiModelPackage` 时是 CAB 内 `AiModelPackageManifestV1` exact JCS 的 SHA-256；两种 MCP kind 时是数据库已批准 `McpManifestV1` canonical JCS 的 SHA-256，必须等于该 `manifest_version_id` 的 `platform_meta.mcp_manifest_versions.manifest_digest`。MCP CAB 内 `McpLocalArtifactManifestV1` 不复用该字段；它由 `package_digest` 对 exact CAB bytes 间接唯一绑定，安装器与 plugin-host 均必须从 CAB 原文重算并验其 CMS/文件 roster，但不在收据/数据库另造第二个同名 digest。因而 lost-ACK 比较以 `package_digest + connector manifest_digest + subject` 完整确定身份，绝不把 inner artifact digest 与 connector manifest digest 混用。

`core-server` 的收据幂等是跨 AI/MCP 两张目标表的全局约束，不是各表各自唯一就算完成：事务第一步固定执行 `pg_advisory_xact_lock(hashtextextended(lower(receipt_id::text),4995704681966667073))`，随后同时查询 `platform_ops.ai_model_packages.install_receipt_id` 与 `platform_meta.mcp_manifest_versions.install_receipt_id`。两表均无该 id 才能登记；恰有一条且除 transport correlation `request_id` 外的全部持久语义（schema version、artifact kind、package/manifest digest、root ref、installed_at 和完整 subject）都能由目标行重建且逐字相等时，返回原 `registered_object_id`；命中另一 artifact kind、出现两条或任一持久语义不同均 fail closed。首次与 replay Ack 都回显本次请求的 `request_id`，固定只含 `request_id,receipt_id,registered_object_id`，不返回不能稳定重建的 acceptance timestamp。生产写角色只允许经该 handler 写这两列，迁移/运维直写被数据库权限和静态检查拒绝；advisory-lock hash 碰撞只会额外串行，不影响正确性。handler 还核 `ep-install://` 解析、root owner/DACL descriptor 及其 digest、收据字段与当前已批准 manifest/附件/配置摘要；它不读取安装文件正文，也不把 ops 收据当成最终内容验签。

AI 先插入带 `install_receipt_id/installed_at` 的 REGISTERED 行，再基于 `ops-agent` 使用同一锁定 verifier 产生且已在认证管道上复核的收据在同一事务走合法边到 VERIFIED，并把该首次服务端事务时间写入不可重写的 `verified_at`；以后 Stage 14 才能写 CERTIFIED。真正激活时 `ai-inferer` 仍须独立复验包签名、manifest、文件 hash、Runtime ABI 与 execution profile；认证报告只由有数据库和证据目录权限的 core/Stage 14 gate 核验。DB ACTIVE 与成功 ACK 同时成立前 compose 始终关闭；复验失败则该活动尝试转 DISABLED，不对外开放。MCP 只允许在 APPROVED 行上以一次 guarded CAS 写入安装收据，随后由 `plugin-host` 调用前独立复验。任何不一致返回签名/manifest 错误且不登记；lost ACK 后同一 receipt 返回相同 `registered_object_id`，持久语义不同则拒绝。离线安装器、ops-agent 与 core 的三方日志只留 receipt/package/manifest/security-descriptor digest 和结果码，不留提示词、secret、路径正文或文件正文。

#### Runtime ABI v1（唯一实现口径）

首版推理引擎不留给实现者选型。`runtime_abi_version=1` 固定使用 Rust 原生 `candle-core=0.11.0`、`candle-transformers=0.11.0` 与直接依赖 `tokenizers=0.23.1`；三者必须精确锁入 workspace/Cargo.lock 与 SBOM，不用宽松 semver 漂移。直接 `tokenizers` 依赖固定 `default-features=false`，但接受 Candle 0.11.0 在 non-wasm 构建中固定引入的 `tokenizers/onig` 与 `onig_sys` 原生依赖；这两项必须进入离线 vendor、Cargo.lock、SBOM、许可证、签名和静态安全扫描，不得被误记为“feature 全关”。仍禁止 `tokenizers/http`、Hugging Face Hub 下载器、Python、ONNX Runtime、llama.cpp/ggml FFI、自定义 native op、脚本解释器或运行期动态库下载。CPU_LOCAL 是所有客户均可用的基线构建；GPU_LOCAL 只允许同版 Candle 的 `cuda` feature 形成另一份签名且经 Stage 14 认证的制品，不存在运行期下载 backend 或切换任意 engine 的配置键。

Runtime ABI v1 的模型数据闭集为：GGUF v3 单文件 `model.gguf`、GGUF architecture 精确为 `qwen2`、GGUF `general.file_type` 精确为 `MOSTLY_Q4_0`，以及独立的 Rust Tokenizers `tokenizer.json`。`manifest.jcs.json` 是 `AiModelPackageManifestV1` 的完整 strict schema，不存在“其余字段由实现者补齐”；其 exact JSON 形状为：

```json
{
  "schema_version": 1,
  "model_code": "local-analytics",
  "model_version": "1.0.0",
  "runtime_abi_version": 1,
  "engine": "CANDLE",
  "engine_version": "0.11.0",
  "tokenizer_engine": "HF_TOKENIZERS_RUST",
  "tokenizer_engine_version": "0.23.1",
  "architecture": "QWEN2",
  "weights_format": "GGUF_V3",
  "quantization": "MOSTLY_Q4_0",
  "weights_file": "model.gguf",
  "tokenizer_file": "tokenizer.json",
  "decoder": "GREEDY_SCHEMA_DFA_V1",
  "prompt_encoding": "QWEN2_CHAT_JCS_V1",
  "prompt_template_file": "prompt-template.utf8",
  "prompt_template_version": "1.0.0",
  "prompt_template_digest": "<64 lowercase hex>",
  "max_new_tokens": 2048,
  "max_context_tokens": 32768,
  "max_concurrent_requests": 15,
  "execution_profile": "CPU_LOCAL",
  "resource_formula_version": "AI_RAM_V1_0_095_HOST",
  "eos_token_ids": [151645],
  "files": [
    {"path":"LICENSE.txt","media_type":"text/plain; charset=utf-8","size_bytes":123,"sha256":"<64 lowercase hex>"},
    {"path":"model.gguf","media_type":"application/vnd.gguf","size_bytes":123,"sha256":"<64 lowercase hex>"},
    {"path":"prompt-template.utf8","media_type":"text/plain; charset=utf-8","size_bytes":123,"sha256":"<64 lowercase hex>"},
    {"path":"sbom.cdx.json","media_type":"application/vnd.cyclonedx+json","size_bytes":123,"sha256":"<64 lowercase hex>"},
    {"path":"tokenizer.json","media_type":"application/json","size_bytes":123,"sha256":"<64 lowercase hex>"}
  ]
}
```

manifest 使用 RFC 8785 JCS，object 未知字段失败，files 必须按 path byte 排序且恰为上列五项。CAB 的 exact entry 集合恰为七项：`manifest.jcs.json`、`manifest.p7s` 与 files 中五个数据文件；禁止 multi-cab/spanning、目录、重复/大小写碰撞路径、ADS、reparse point 或第八项。manifest 与 detached signature 不进入 files；`prompt_template_digest` 必须等于 files 中 `prompt-template.utf8` 的 hash。CAB exact bytes 必须为 1..2147483647；解包七项总未压缩上限 2130706432 bytes；`model.gguf` 1..2000000000、`tokenizer.json` 1..67108864、`prompt-template.utf8` 1..65536、`LICENSE.txt` 1..1048576、`sbom.cdx.json` 1..16777216 bytes，五项之和还须不超过总上限。生产 CAB 自身先通过 Authenticode，再以 Windows CMS SignedData 验 `manifest.p7s`：CMS 必须是 detached form，验证时由调用方提供的 detached content 恰为 `manifest.jcs.json` 的 exact JCS bytes，signed attribute `messageDigest=SHA-256(content)`；signer certificate 必须具 Code Signing EKU、链到部署批准的产品/客户 release root 且未过期/吊销。签名算法只允许 ECDSA-P256-SHA256 或 RSA-PSS-SHA256（RSA modulus 至少 3072）。DEV 包只走既有 DEV ECDSA P-256 信任根并带环境标记，生产拒绝。

`eos_token_ids` 首版固定为 `[151645]`，tokenizer 还必须把 token 151644/151645 分别逐字解码为 `<|im_start|>`/`<|im_end|>`；不允许按模型包另选特殊符。预检只从 GGUF qwen2 metadata 取维度，不存在第六个 `config.json`：必需键为 `general.architecture=qwen2`、`qwen2.attention.head_count`、`qwen2.attention.head_count_kv`、`qwen2.embedding_length`、`qwen2.context_length`、`qwen2.block_count`、`qwen2.attention.layer_norm_rms_epsilon`，`qwen2.rope.freq_base` 缺席时唯一默认 10000；这些值均须为 Candle 0.11.0 loader 可表示的正数，embedding length 能被 head count 整除，KV head count 不大于且能整除 head count，GGUF context 不小于 manifest `max_context_tokens`。由 `block_count` 生成完整 tensor-name roster：根节点只允许 `token_embd.weight,output_norm.weight` 与可选 `output.weight`；每层 `i=0..block_count-1` 恰有 `blk.i.attn_q.weight,attn_k.weight,attn_v.weight,attn_q.bias,attn_k.bias,attn_v.bias,attn_output.weight,ffn_gate.weight,ffn_down.weight,ffn_up.weight,attn_norm.weight,ffn_norm.weight`。所有 rank-2 matrix tensor 必须为 `Q4_0`，所有 rank-1 norm/bias tensor 必须为 `F32`；`output.weight` 缺席时唯一语义为与 `token_embd.weight` tied，存在时必须是独立合法 rank-2 `Q4_0` tensor。缺少必需 tensor、出现未知 tensor、rank/dimension/dtype 不符或 `general.file_type` 不为 `MOSTLY_Q4_0` 均整包拒绝。首版不接受 safetensors、ONNX、其他 GGUF architecture、其他 file type、其他 dtype 组合或 package 内自定义代码；以后增加任何一种都必须提升 runtime ABI 并另立高于 F-55 的裁定。

`QWEN2_CHAT_JCS_V1` 的模型输入 bytes 唯一为（`||` 表示无隐含分隔符的 byte concat）：

```text
UTF8("<|im_start|>system\n")
|| exact prompt-template.utf8 bytes
|| UTF8("<|im_end|>\n<|im_start|>user\n")
|| JCS({"catalog_projection": <AiCatalogProjectionV1>, "question": <string>})
|| UTF8("<|im_end|>\n<|im_start|>assistant\n")
```

prompt 文件必须是无 BOM、LF-only 的合法 UTF-8，不含 NUL、`<|im_start|>` 或 `<|im_end|>`，不做 trim、Unicode normalization 或换行补齐。tokenizer 调用固定 `encode(add_special_tokens=false)`，不 padding、不 truncation、不自动 BOS；decode 固定 `skip_special_tokens=false`，不做 cleanup。输入 token 数加 2048 若超过 manifest/GGUF 两者较小的 context 上限，整次 compose 以 `AI.INPUT.CONTEXT_LIMIT_EXCEEDED` 失败，不截断目录、问题或 prompt。

解码固定为 `GREEDY_SCHEMA_DFA_V1`：temperature=0、无 top-k/top-p/seed/sampling 参数、最多 2048 个新 token。每一步先把 logits 中所有会使已解码 UTF-8 字节不再是 `AiQueryPlanV1` 唯一 JSON Schema 合法前缀的 token 置为不可选，再在剩余 token 中取唯一最高 logit；同值按 token id 较小者决定。根值只能是一个 object，字段名/顺序固定为 `schema_version,dataset_code,projections,group_by,aggregates,filters_all,order_by,limit`，不得出现 markdown fence、前后说明、未知字段、NaN/Infinity 或第二个 JSON 值。完整 object 后只接受活动包声明的 EOS；到达 token/context/deadline 上限仍未闭合即按 `AI.QUERY_PLAN.INVALID` 失败。DFA 只保证语法和 DTO 形状，字段可见性、类型、算子基数和全部业务闭集仍由第 3.3/3.4 节确定性 validator 再判一次。

具体生产模型的 `model_code/model_version/package_digest` 仍由签名离线包和 Stage 14 认证报告确定；只要满足上述 Runtime ABI v1，它是部署制品事实，不是开发者可另选推理栈的设计分支。依赖安全升级须走签名发布、完整 AI 收容/资源回归、Cargo.lock 与 SBOM 更新；不得在既有发布中静默漂移版本。

### 3.3 `ep-ai` 命名管道 exact ABI

管道名固定为 `\\.\pipe\ep-ai`，server 为 `NT SERVICE\ep-ai`。沿用基线的 4 字节大端长度前缀加 JSON framing；普通帧最大 1 MiB，身份握手/首帧/单次绝对时限分别为 5/10/120 秒。取消只由 deadline 或断开当前调用表达，不新增 cancel operation。

管道实例与账户额度同样是编译期闭集：总实例 51，`ep-core` 的 compose 数据面 45 个（15 个运行中 + 30 个排队）、`ep-core` 的模型控制面 2 个、`ep-ops` 2 个，余下 2 个实例只用于持续 accept/补位；四组额度互不借用。达到 compose 额度或内部运行/排队上限均在读取完整模型输入前返回 `AI.INFERENCE.CONCURRENCY_LIMIT`。断连立即取消尚未开始的排队项；已开始的推理必须协作取消并立即清零该请求 decoder/KV state。首版没有 per-invocation 子进程，也不尝试强杀 Rust 线程：runtime 若在取消后 2000ms 内未确认终止，服务监督器必须关闭 compose readiness 并终止整个 `ai-inferer`/`APP_AI` Job，使同进程其余在途请求一并取消、清零且以 `AI.MODEL_PACKAGE.NOT_ACTIVE` 失败；随后重启服务，并按数据库唯一 ACTIVE 行重新完成独立包复验、fresh activate ACK 与认证 gate 复核后才重新开放。不得继续计算到 120 秒 deadline，不得保留/交付任何受影响请求的结果，也不得在重启前复用旧 ACK。该额度不设配置分支，必须与第 9.1 节两个固定值同时校验。

operation 白名单恰好五项：

| 调用账户 | operation | 用途 |
|---|---|---|
| `NT SERVICE\ep-core` | `ai.query_plan.compose.v1` | 从三项模型输入生成计划 |
| `NT SERVICE\ep-core` | `ai.model.activate.v1` | 指定并复验数据库唯一 ACTIVE 的签名模型包；仅控制面保留实例可用 |
| `NT SERVICE\ep-core` | `ai.model.deactivate.v1` | 清空活动模型与全部 KV/prefix cache；仅控制面保留实例可用 |
| `NT SERVICE\ep-ops` | `health.get.v1` | 既有健康快照 |
| `NT SERVICE\ep-ops` | `metrics.snapshot.v1` | 既有有界指标快照 |

任何其他账户、operation、通配符或同义别名都在读取业务 payload 前拒绝并审计。`ep-worker`、`ep-plugin`、`ep-integ`、`ep-portal`、写出账户与迁移账户都没有 ACE。

exact Rust transport contract：

```rust
pub const AI_PIPE_NAME: &str = r"\\.\pipe\ep-ai";
pub const AI_QUERY_PLAN_COMPOSE_V1: &str = "ai.query_plan.compose.v1";
pub const AI_MODEL_ACTIVATE_V1: &str = "ai.model.activate.v1";
pub const AI_MODEL_DEACTIVATE_V1: &str = "ai.model.deactivate.v1";

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AiModelActivationDescriptorV1 {
    pub request_id: RequestId,
    pub schema_version: u16,                 // 必须为 1
    pub model_package_id: Uuid,
    pub model_code: String,
    pub model_version: String,
    pub package_digest: Sha256Digest,
    pub manifest_digest: Sha256Digest,
    pub installed_root_ref: String,
    pub execution_profile: AiExecutionProfileV1,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum AiExecutionProfileV1 { CpuLocal, GpuLocal }

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AiModelActivationAckV1 {
    pub request_id: RequestId,
    pub model_package_id: Uuid,
    pub package_digest: Sha256Digest,
    pub manifest_digest: Sha256Digest,
    pub runtime_abi_version: u16,            // 必须为 1
    pub execution_profile: AiExecutionProfileV1,
    pub independently_verified_at: DateTime<Utc>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AiModelDeactivationRequestV1 {
    pub request_id: RequestId,
    pub schema_version: u16,                 // 必须为 1
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AiModelDeactivationAckV1 {
    pub request_id: RequestId,
    pub previous_model_package_id: Option<Uuid>,
    pub deactivated_at: DateTime<Utc>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AiCatalogProjectionV1 {
    pub schema_version: u16,                 // 必须为 1
    pub datasets: Vec<AiDatasetDescriptorV1>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AiDatasetDescriptorV1 {
    pub dataset_code: String,
    pub display_name: String,
    pub grain: String,
    pub fields: Vec<AiFieldDescriptorV1>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AiFieldDescriptorV1 {
    pub field_code: String,
    pub display_name: String,
    pub data_type: AiDataTypeV1,
    pub allowed_aggregations: BTreeSet<AiAggregationV1>,
    pub is_filterable: bool,
    pub is_sortable: bool,
    pub is_groupable: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum AiDataTypeV1 { Text, Integer, Decimal, Boolean, Date, DateTime, Uuid }
#[derive(Clone, Debug, Serialize, Deserialize, Ord, PartialOrd, Eq, PartialEq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum AiAggregationV1 { Sum, Count, Min, Max, Avg }
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum AiFilterOperatorV1 {
    Eq, Ne, Lt, Lte, Gt, Gte, In, Between, IsNull, IsNotNull,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum AiOrderDirectionV1 { Asc, Desc }

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(tag = "kind", content = "value", rename_all = "SCREAMING_SNAKE_CASE", deny_unknown_fields)]
pub enum AiScalarV1 {
    Text(String),
    Integer(i64),
    Decimal(String),                       // 规范十进制定点文本
    Boolean(bool),
    Date(String),                          // YYYY-MM-DD
    DateTime(String),                      // RFC 3339，必须带 offset
    Uuid(Uuid),
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AiAggregateV1 {
    pub field_code: String,
    pub function: AiAggregationV1,
    pub result_code: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AiPredicateV1 {
    pub field_code: String,
    pub operator: AiFilterOperatorV1,
    pub values: Vec<AiScalarV1>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AiOrderV1 {
    pub result_code: String,
    pub direction: AiOrderDirectionV1,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AiQueryPlanV1 {
    pub schema_version: u16,                // 必须为 1
    pub dataset_code: String,
    pub projections: Vec<String>,
    pub group_by: Vec<String>,
    pub aggregates: Vec<AiAggregateV1>,
    pub filters_all: Vec<AiPredicateV1>,     // 只有平铺 AND；无布尔树字段
    pub order_by: Vec<AiOrderV1>,
    pub limit: u16,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct FixedPromptV1 {
    pub template_version: String,
    pub template_digest: Sha256Digest,
    pub template_utf8: SensitiveString,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AiModelInputV1 {
    pub catalog_projection: AiCatalogProjectionV1,
    pub question: SensitiveString,
    pub fixed_prompt: FixedPromptV1,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AiCachePartitionV1 {
    pub legal_entity_id: Id<LegalEntity>,
    pub user_id: Id<UserAccount>,
    pub security_context_digest: Sha256Digest,
    pub catalog_projection_digest: Sha256Digest,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AiComposePipeRequestV1 {
    pub request_id: RequestId,
    pub trace_id: TraceId,
    pub turn_id: Uuid,
    pub cache_partition: AiCachePartitionV1,
    pub input: AiModelInputV1,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AiComposePipeReplyV1 {
    pub request_id: RequestId,
    pub turn_id: Uuid,
    pub inference_batch_id: Uuid,
    pub model_code: String,
    pub model_version: String,
    pub model_package_digest: Sha256Digest,
    pub prompt_template_version: String,
    pub query_plan: AiQueryPlanV1,
}

#[async_trait]
pub trait AiInferencePort: Send + Sync {
    async fn compose(
        &self,
        request: AiComposePipeRequestV1,
    ) -> Result<AiComposePipeReplyV1, AppError>;
}
```

活动模型状态机只有一个实现口径：`ai-inferer` 每次启动都处于 `NO_ACTIVE_MODEL`，不扫描目录猜测活动包，也不读取数据库、认证报告或本地活动指针。`core-server` 在提交唯一 ACTIVE 数据库行前验证 certification ref/digest、报告签名、当前 Stage 14 gate 与模型行逐项一致，提交后才发送 `ai.model.activate.v1`；descriptor 不携带报告路径或报告正文。`ai-inferer` 只复验 descriptor、root ACL、包签名/hash、Runtime ABI 与 execution profile，成功后以原子指针切换只读权重并返回逐字段一致的 ACK。收到 ACK 后 core 再读取当前 ACTIVE 行并重验认证 gate，只在“当前数据库 ACTIVE 行 = 当前进程内 ACK 且认证 gate 仍有效”时开放 compose；切换期间、IPC 断线、ACK 丢失、进程重启或任一字段不等均 fail-closed 为 `AI.MODEL_PACKAGE.NOT_ACTIVE`。禁用、撤销或数据库无 ACTIVE 行时，core 先使业务路由 fail-closed，再调用 `ai.model.deactivate.v1`；该调用清空活动指针和全部 cache。core/ai 任一重启后均按数据库事实重发 activate/deactivate 并取得新 ACK，不复用旧内存结论。激活失败不自动改选目录中的其他包；运维只能修复该签名包、显式停用，或按既有审批状态图选择另一 CERTIFIED 包。

同一活动包在进程内只保留一份共享只读权重映射；15 个运行中请求各自拥有独立、不可共享的 decoder 状态与 KV cache，完成/取消/超时即销毁。不得复制 15 份完整权重来换取并发，不得在请求之间复用可变模型状态或 prefix/KV block；同批只允许共享只读权重。

只有 `AiModelInputV1` 的三个字段进入 tokenizer。request/trace/turn、安全上下文摘要和缓存分区只用于传输、分桶与审计关联，不能拼进提示词。reply 不存在结果集、SQL、自然语言结论或写命令字段；反序列化遇额外字段一律失败。

`AiCatalogProjectionV1.datasets` 按 `dataset_code`、fields 按 `field_code` 排序且均不重复；最大 64 个数据集、每数据集最大 256 个字段、规范 JSON 最大 512 KiB。`FixedPromptV1` 必须逐字匹配活动签名模型包中的 version/digest/text，最大 64 KiB，调用者不能覆盖。

`AiQueryPlanV1` 只允许一个 `dataset_code`，并承载投影、分组、聚合、平铺合取过滤、排序与 `limit`。禁止 join、子查询、UNION、窗口函数、任意函数名、SQL 片段、计算列、跨数据集字段与顶层 OR/NOT。字段及算子只能来自本轮目录投影；projections、group_by、aggregates、filters、order_by 上限分别为 64/16/16/32/3 且各列表内去重；至少有一项 projection 或 aggregate。`IsNull|IsNotNull` 的 values 必须为空，`Between` 恰好 2 个，`In` 为 1..=100 个，其余 operator 恰好 1 个；literal 类型必须与字段 data_type 一致。`result_code` 只能是规范字段码或 `agg_<1..16>`，不可携带 SQL 标识符。`limit` 为 1..=1000。通过校验后才转换为既有 reporting `QueryPlan`。

### 3.4 外部 HTTP API

#### Compose

`POST /api/v1/reporting/ai-query-plans/actions/compose`

```json
{
  "question": "按客户统计本月已确认销售额，倒序显示前 20 名",
  "locale": "zh-CN"
}
```

约束：`question` 为 1..2000 个 Unicode 标量值，`locale` 首版只允许字面量 `zh-CN`；请求不得带数据集码、字段码、权限、SQL、模型参数、温度或系统提示词。能力常量固定为 `ReportingReportPrint + Read`，权限码为 `reporting.ai_analysis.compose`。Compose 与 Execute 共同构成已认证只读 action POST 幂等豁免闭集：两者都绕过写请求幂等中间件、绝不读写 `platform_msg.idempotency_keys`，且携带 `Idempotency-Key` 均以 `AI.QUERY_PLAN.IDEMPOTENCY_NOT_ALLOWED` 拒绝。这避免把五分钟 plan token 或分析结果缓存七天；不能类推新增其他豁免端点。

Compose 是全局同步 HTTP 8 秒 timeout/20-slot 交易闸门的唯一 AI 具名例外。该 route 在完成普通 body/header/session/法人基础校验后进入独立公平 semaphore，容量固定 45（15 个运行位 + 30 个排队位），不占普通交易 20-slot；无位立即返回 `AI.INFERENCE.CONCURRENCY_LIMIT`，不得继续排入无界队列。route 专用 Tower timeout 固定 122000ms，内部 `ai.compose_timeout_ms=120000` 从进入 AI semaphore 起覆盖排队、IPC 与推理，余 2000ms 只供取消、清零与稳定错误封套；内部或外层 deadline 命中统一使用既有 `PLATFORM.SYSTEM.SYNC_TIMEOUT`，但只能在该专用 120/122 秒边界触发，不得被普通 8 秒 layer 提前截断。断连/122 秒到期必须取消同一 pipe invocation 并清除请求 decoder/KV state。

成功响应：

```json
{
  "turn_id": "0198...",
  "plan_token": "<opaque-signed-token>",
  "expires_at": "2026-08-22T12:05:00Z",
  "human_confirmation": {
    "dataset_label": "销售订单",
    "selected_fields": ["客户", "已确认销售额"],
    "filters": ["确认日期位于本月"],
    "grouping": ["客户"],
    "aggregation": ["已确认销售额求和"],
    "ordering": ["已确认销售额降序"],
    "limit": 20
  },
  "model": {"code": "local-analytics", "version": "1.0.0"}
}
```

`human_confirmation` 由确定性 renderer 从验证后的计划产生，模型不能提供或覆盖。compose 中只有在模型输出已经通过 JSON/结构/算子/limit 验证、却引用了本轮按当前权限裁剪目录中不存在的 dataset 或 field code 时返回 `AI.QUERY_PLAN.NOT_VISIBLE_OR_DENIED`；真实不存在与存在但当前用户不可见必须同为该 404、同封套且通过时间差门禁，零 token。其他结构、类型、算子和上限错误仍用 `AI.QUERY_PLAN.INVALID`。`AI.QUERY_PLAN.NOT_VISIBLE_OR_DENIED` 首版只在 compose 可达：execute 不查任何草案表；token 签名/claims/摘要不合法用 `AI.QUERY_PLAN.TOKEN_INVALID_OR_EXPIRED`，合法 token 所绑定的数据集、字段、授权或目录当前事实变化用 `AI.QUERY_PLAN.SECURITY_CONTEXT_CHANGED`，不得在 execute 改用 404。`plan_token` 由 `core-server` 签名；服务端不建 AI 草案表。token 不是 JWT/HMAC，唯一 framing 为 `epai1.<base64url-no-pad(key_version_utf8)>.<base64url-no-pad(JCS(claims))>.<base64url-no-pad(signature_p1363_64)>`。claims unknown-field 拒绝，exact 字段为 `schema_version=1,turn_id,legal_entity_id,user_id,source_session_id,source_device_id,query_plan,security_context_digest,catalog_projection_digest,model_package_id,model_package_digest,prompt_template_version,prompt_template_digest,issued_at,expires_at`；时间为 RFC3339 UTC 秒精度，`expires_at=issued_at+300s`。签名输入为 `SHA-256(ASCII("EP-AI-PLAN-TOKEN-V1\0") || ASCII(first_three_segments))`，经 `KmsBackend` 使用法人密钥域独立 purpose `AI_PLAN_TOKEN_V1` 的 ECDSA P-256 key，输出 canonical low-S IEEE-P1363 64 bytes；私钥不出 KMS/HSM。只用当前 ACTIVE key 签发；轮换后旧 key 仅 verify，保留到其最后一枚合法 token 的 expires_at 再加 60 秒即销毁/退役，revoked key 立即全拒绝。execute 还须要求 token key version 与法人域匹配、issued_at 不晚于当前时间 60 秒且其余 claims 与当前事实逐项一致。

#### Execute

`POST /api/v1/reporting/ai-query-plans/actions/execute`

```json
{
  "plan_token": "<opaque-signed-token>",
  "confirmed": true
}
```

能力常量固定为 `ReportingReportPrint + Read`，权限码为 `reporting.ai_analysis.execute`。`confirmed` 不为字面量 `true` 即拒绝。该只读 POST 按 Compose 段的同一闭集绕过幂等写中间件并禁止 `Idempotency-Key`，避免结果进入 `platform_msg.idempotency_keys.response_body`。执行使用既有 `ep_analyst_ro` 只读池、RLS 会话变量、单查询超时与行/字节上限；成功响应 exact DTO 为：

```json
{
  "turn_id": "0198...",
  "dataset_code": "sales.confirmed_orders",
  "columns": [
    {"result_code": "customer_name", "display_name": "客户", "data_type": "TEXT", "aggregation": null},
    {"result_code": "agg_1", "display_name": "已确认销售额", "data_type": "DECIMAL", "aggregation": "SUM"}
  ],
  "rows": [["示例客户", "1234.50"]],
  "row_count": 1,
  "truncated": false,
  "executed_at": "2026-08-22T12:01:00Z"
}
```

`columns` 顺序就是每行顺序，每个 `rows[]` 长度必须等于列数；`result_code` 唯一。INTEGER 用 JSON integer，BOOLEAN 用 JSON boolean，DECIMAL 用不含指数且无多余前导零的规范十进制定点 string，DATE/DATE_TIME/UUID/TEXT 用规范 string，SQL NULL 用 JSON null；不得返回 float、NaN/Infinity、SQL 类型名、内部列名、SQL、自然语言结论或模型生成说明。执行器以 `limit + 1` 读取：不超过 limit 时 `truncated=false`，读到额外一行时只丢弃该额外行并置 `truncated=true`。8 MiB 指 compact UTF-8 JSON 成功响应的 exact bytes（含 envelope、columns 与 rows）；序列化采用有界 sink，一旦下一完整 JSON token 会使总数超过 8388608 bytes，立即中止并丢弃全部结果，返回既有 `REPORTING.ANALYTIC_QUERY.RESULT_TOO_LARGE`，不得返回部分行或用 `truncated=true` 掩盖字节超限。行数硬上限仍为 1000；`truncated` 只表达 `limit+1` 的行截断，不表达字节截断。

执行前逐次完成以下重检，不能复用 compose 时的允许结论：活动会话与设备、法人授权、模块许可证、两项 AI 权限、能力矩阵、角色和职责、密级、字段权限、记录谓词、数据集与字段当前登记、模型/提示版本仍有效、token 未过期、计划签名与摘要一致。随后把当前记录谓词作为最外层 `AND` 注入，再由 RLS 兜底法人隔离。任一项变化都拒绝，零查询、零结果、零模型调用。

### 3.5 目录、缓存与结果隔离

- 模型目录只含当前调用人按数据集权限、字段权限与 `clearance_level` 可见的 schema 元数据。记录级 `data_scope` 不改变 schema 元数据成员关系，这是本裁定的明确取值，不是遗漏；它只在执行时形成结构化行谓词。
- 目录不得含样例值、distinct 值、最小/最大值、行数、统计量、对象名称实例、附件正文或查询结果。
- 首版不跨请求复用 KV/prefix cache：每个请求从同一份共享只读权重建立自己的新 decoder/KV state，完成、拒绝、断连、取消或超时立即清零销毁；法人、用户或相同问题也不命中旧前缀，不实现批间 cache key。
- 只允许同一时刻进入同一 forward step、且 `legal_entity_id + user_id + security_context_digest + catalog_projection_digest + model_package_digest + prompt_template_version` 六项全相等的请求作临时动态 batching；batch 结束即释放各请求独立 state，不把任一请求的 token/KV 留给下一批。
- 结果及其摘要不能进入 AI 请求、AI 缓存、模型日志、tracing 字段、崩溃转储正文、审计 before/after 正文或幂等响应缓存。Windows Error Reporting 对 `ai-inferer` 禁止 full dump，只允许不含用户 payload 的最小故障元数据。

### 3.6 模型包表与迁移形状

新增部署级表 `platform_ops.ai_model_packages`，无 `legal_entity_id`、无 RLS，并登记到 `platform_core.unpoliced_table_registry`，理由固定为 `SAME_FOR_ALL_ENTITIES`。列集：

`id`、公共部署级列、`security_level smallint`（固定 40）、`model_code text`、`model_version text`、`runtime_abi_version int`、`package_digest bytea`、`manifest_digest bytea`、`signer_subject text`、`signature_kind text`（`PROD_AUTHENTICODE|DEV_ECDSA_P256`）、`installed_root_ref text`、`install_receipt_id uuid`、`installed_at timestamptz`、`prompt_template_version text`、`max_context_tokens int`、`max_concurrent_requests int`、`execution_profile text`（`CPU_LOCAL|GPU_LOCAL`）、`resource_formula_version text`、`certification_report_ref text null`、`certification_report_digest bytea null`、`verified_at timestamptz null`、`certified_at timestamptz null`、`activated_at timestamptz null`、`disabled_at timestamptz null`、`revoked_at timestamptz null`、`status text`（`REGISTERED|VERIFIED|CERTIFIED|ACTIVE|DISABLED|REVOKED`）、`active_slot smallint generated always as (case when status='ACTIVE' then 1 else null end) stored`。

约束固定为：`UNIQUE(model_code,model_version)`、`UNIQUE(package_digest)`、`UNIQUE(install_receipt_id)`、`UNIQUE(active_slot)`；security level 恰为 40；package/manifest/certification report digest 非空时必须 32 字节；上下文和并发大于零；PROD 只接受生产签名；状态边只有 `REGISTERED→VERIFIED→CERTIFIED→ACTIVE`、`ACTIVE→DISABLED`、`DISABLED→CERTIFIED`，任一非 REVOKED 状态可到 `REVOKED`，REVOKED 不可恢复。时间/状态 truth table 固定为：REGISTERED 五时间全 NULL；VERIFIED 只有 verified_at 非空；首次 CERTIFIED 有 verified/certified，activation/disable 两者均 NULL；ACTIVE 有 verified/certified/activated，disabled/revoked NULL；DISABLED 有 verified/certified/activated/disabled，revoked NULL，且 `certified_at<=activated_at<disabled_at`；从 DISABLED 回 CERTIFIED 时保留上次 activated/disabled 两值作为一对，把 certified_at 更新为本次资源认证 report.finished_at 且必须 `disabled_at<certified_at`；再次 ACTIVE 把 activated_at 更新为本次成功 activate-ACK/gate 事务时间并清 disabled_at=NULL。REVOKED 只在转入时写 revoked_at，保留前置状态已有的其他时间/空值形状，且 revoked_at 严格晚于所有已有时间。verified_at 一经写入永不重写；certified/activated/disabled 是当前/最近一轮状态事实，过往各轮以审计事件保留，不新增时间表。certification ref/digest 同空同非空；CERTIFIED/ACTIVE/DISABLED 必须非空，REGISTERED/VERIFIED 必须为空，REVOKED 保留前置形状。身份、摘要、签名、路径、安装收据、模型限制与执行 profile 全部不可变，升级登记新行。该表只有 core-server 的模型选择器与 ops 只读视图可读，`ai-inferer` 不连接数据库。

### 3.7 资源值、算定式与 Stage 14 冻结

AI 资源认证报告不是任意文件。`AiResourceCertificationReportV1` 是最大 1048576 bytes 的 RFC 8785 JCS strict object，字段恰为 `schema_version=1,report_id,release_batch_id,product_build_digest,ai_runtime_artifact_digest,model_package_id,model_code,model_version,runtime_abi_version,package_digest,manifest_digest,execution_profile,resource_formula_version,server_spec_digest,certified_host_ram_bytes,calculated_hard_limit_bytes,max_context_tokens,max_concurrent_requests,load_profile_digest,host_commit_peak_bytes,vram_peak_bytes,gpu_device_model,gpu_driver_version,page_or_swap_observed,gate_results[],started_at,finished_at,verifier_subject`。digest 均为 64 位 lowerhex；`runtime_abi_version=1`、`max_concurrent_requests=15`；CPU profile 的三个 GPU 字段全空，GPU profile 三者全有；`page_or_swap_observed=false`。`gate_results` 每项 strict 字段恰为 `gate_code,outcome="PASSED",evidence_digest`，按 gate code bytes 排序去重并与本次 release 的 AI resource gate registry 逐元素相等，至少包含 `ai_runtime_release_facts`、15 路最大上下文负载、host commit、分页/交换、既有交易/门户/备份/归档/RPO/RTO，以及 GPU profile 的 VRAM/driver/device；缺项、额外项或非 PASSED 均不是有效报告。report 必须逐项绑定当前 release batch、签名产品 build/ai-inferer artifact、模型行/包/manifest/ABI/profile/formula、服务器规格和实测 load profile，不能把另一硬件、另一包或另一发布批次的报告换入。

`ai_runtime_release_facts` 不是第二套持久证据或签名 sidecar。Stage 14 gate 当场从已签 `ai-inferer.exe`、当前 `Cargo.lock`、离线 vendor tree、CycloneDX SBOM 与解析后的 dependency features 重算一个 `AiRuntimeReleaseFactsV1`；它是最大 262144 bytes 的 RFC 8785 JCS strict object，字段恰为 `schema_version=1,release_batch_id,runtime_abi_version,execution_profile,artifact_digest,candle_core_version,candle_transformers_version,tokenizers_version,product_features[],resolved_dependency_features[],cargo_lock_digest,offline_vendor_digest,sbom_digest,authenticode_subject,authenticode_signature_digest`。`release_batch_id` 是 UUID，不是自由字符串；digest 均为 64 位 lowerhex；两数组按 UTF-8 bytes 排序去重，每项 1..128 bytes且各最多 256 项；subject 1..512 bytes。ABI 固定 1，版本固定 `0.11.0|0.11.0|0.23.1`；CPU 的 product features 恰为 `cpu`，GPU 恰为 `cuda`，resolved feature 集必须逐元素等于发布代码内按 profile 冻结的闭集。gate 直接消费当场生成的 exact bytes、把其 SHA-256 写入最终已签报告的 `gate_code=ai_runtime_release_facts` 项，并在报告签名前再次逐项匹配 report 的 release/profile/artifact facts；该 DTO 没有 ref、独立签名、有效期或数据库行，工具不得把它称为 release evidence、不得要求另一把 key，也不得把临时 JSON 留在认证证据目录。后续认证/激活以最终签名 `AiResourceCertificationReportV1` 为权威，并从当前随产品保留的签名制品重新计算该 digest作一致性检查。

报告签名 sidecar `AiResourceCertificationSignatureV1` 同样是 strict JCS，字段恰为 `schema_version=1,purpose="AI_RESOURCE_CERTIFICATION_V1",release_batch_id,model_package_id,report_digest,key_ref,key_version,signer_subject,signature_p1363_b64url`。最后一字段是 RFC 4648 §5 base64url-no-pad canonical string，解码恰好 64 bytes、重编码必须逐 byte 等于输入，禁止 JSON integer array、lowerhex、DER 或 padding。签名输入唯一为 `SHA-256(ASCII("EP-AI-RESOURCE-CERTIFICATION-V1\0") || release_batch_id[16] || model_package_id[16] || report_digest[32])`，使用客户部署 KMS/HSM 中 purpose `AI_RESOURCE_CERTIFICATION_V1` 的 ECDSA P-256 key，signature 是 canonical low-S IEEE-P1363 64 bytes；core 每次认证状态推进、ACTIVE 提交前和收到 activate ACK 后都核 purpose、当前/retired-nonrevoked key、subject、P-256 signature、report/ref/digest 与当前全部事实。retired key 只验证其原发布批次历史证据，revoked key 立即令 gate 失效；DEV key/report 不能进入生产。

`certification_report_ref` 的唯一形状为 `ep-evidence://ai-resource/<lowercase-release-batch-uuid>/<lowercase-model-package-uuid>/<cpu-local|gpu-local>/sha256/<64-lowerhex-report-digest>`；编译期解析根固定 `C:\ProgramData\EnterprisePlatform\evidence\ai-resource\<release-batch-id>\<model-package-id>\<profile>\`，报告文件 `<digest>.jcs`、sidecar `<digest>.sig.jcs`。owner 固定 SYSTEM、关闭继承；SYSTEM/Administrators/`NT SERVICE\ep-ops` 可管理，`NT SERVICE\ep-core` 仅 read/`READ_CONTROL`，ai-inferer 与其他账户无 ACE。解析/open 拒绝 URL、UNC、device path、`..`、ADS、reparse、hardlink escape、8.3/大小写碰撞，逐次核 root、owner/DACL、文件名、JCS digest 与 sidecar；DB/API/审计只保存 opaque ref、digest 与结论，不返回本地路径、sidecar 或报告正文。`certification_report_digest` 精确保存 report JCS SHA-256，`certified_at` 精确等于 report `finished_at`；上述验证全部成功前不得写 CERTIFIED/ACTIVE。

`ai-inferer` 占用原配额表中没有独立承载物的“内置搜索索引”那一行，不从 PostgreSQL 或既有产品进程扣份额。该旧行不再承诺未来独立搜索资源单位；内置搜索继续按实际调用进程计费。AI 行的三个相对值固定继承为 CPU 权重 10、内存 10%、磁盘 IO 权重 8。故产品常驻进程和资源单位各增加一个，但配额表仍为九行；F-12-2 旧“增加第十行”的预估由本条明确覆盖。

总体规格第 13.1 章的让路次序中，`ai-inferer` 推理与 AI execute 产生的临时分析查询固定归第 7 级；execute 若命中附录 A.1 已列名的常用报表，只表示该既有报表本身仍按第 3 级验收，不把 AI compose/inference 提升为第 3 级。内置搜索的在线增量更新仍在第 4 级，但资源用量按实际执行它的既有进程归因，不再拥有独立资源单位。该分类不产生 CPU/IO 运行期优先保证，只冻结应用调度与认证观测口径。

内存绝对值只用既有同一算定式：

```text
AI_HOST_MEMORY_HARD_LIMIT_BYTES = floor(0.95 × CERTIFIED_HOST_RAM_BYTES × 0.10)
                                = floor(0.095 × CERTIFIED_HOST_RAM_BYTES)
```

不得手填另一个绝对值。Job Object 的内存硬上限必须等于该式结果。模型包的认证工作集通过线为：固定 15 路并发、包声明最大上下文、既有附录 A.4 综合负载同时运行时，`ai-inferer` 的 commit peak 不超过硬上限的 80%，没有分页/交换，且既有交易、备份、归档、门户与 RPO/RTO 通过线全部仍通过。GPU profile 还必须记录显存峰值与驱动/设备型号，但 GPU 显存不计入主机内存公式。

Stage 14 必须把服务器规格、公式版本、算定绝对值、模型包摘要、15 路负载、host peak、VRAM peak、各既有通过线与报告 hash 一并冻结到认证报告，并把报告引用写回活动模型包。测量未执行、报告缺字段或任一通过线失败时，`RG-AI-RESOURCE-CERTIFIED` 发布门禁非零；开发、单元测试、使用签名微型 fixture 的 CI 和迁移编写不受阻。失败后的唯一处置是更换更小的签名模型包或提高认证硬件规格并重跑 Stage 14，不得临时调高 AI 百分比或挤占 PostgreSQL 配额。

### 3.8 九条收容断言

`testkit/src/ai_containment.rs` 必须唯一导出：

```rust
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

AI 审计使用现有 `platform_audit.audit_events`，不新建 AI 日志表。编译期审计对象目录 `crates/platform/audit/src/object_registry.rs` 必须登记一个 object-level 且 `object_id` 必填的对象 `reporting.ai.query_turn`，其 action 闭集恰为 `AI_QUERY_PLAN_COMPOSED|AI_QUERY_PLAN_EXECUTION_ATTEMPTED`；同一文件也登记第 4.7 节 MCP 对象/action。目录一致性测试必须断言这两个 F-55 对象逐项存在、无别名、无额外 F-55 action，数据库迁移不另建可漂移的审计对象登记表。

compose 在身份、授权、模型输出、计划 validator 与 dataset 级当前权限全部通过后生成唯一 UUIDv7 `turn_id`；向客户端签发 token/返回成功前，恰先提交一条 `AI_QUERY_PLAN_COMPOSED`，`object_type=reporting.ai.query_turn,object_id=turn_id`。execute 只有在 plan token 的签名、turn id 与全部当前事实重检通过后，才以 token 的同一 turn id 在查询前恰提交一条 `AI_QUERY_PLAN_EXECUTION_ATTEMPTED`；这条 commit 失败则零 SQL、零结果。无可信身份、无合法计划或 token 尚未验真的拒绝不伪造 turn 对象，只走既有脱敏 HTTP/security denial；查询后的成功、超限或数据库错误不追加第二条 AI 行，业务结果只进入普通响应/指标，不进入审计正文。由此审计行精确表示“已生成可签发计划”和“已获准开始一次查询”两个安全边界，而不伪称终态。

两 action 的 `before` 固定 NULL，`after` 都是 `schema_version=1` strict masked object，公共字段恰为 `phase,turn_id,request_id,trace_id,question_sha256,query_plan_sha256,catalog_projection_digest,security_context_digest,model_package_id,model_package_digest,prompt_template_version,prompt_template_digest`。`phase` 与 action 分别只能为 `COMPOSED` 或 `EXECUTION_ATTEMPTED`；`turn_id` 必须等于 object id。digest 均为 64 位 lowerhex：question digest 是原请求问题的 exact UTF-8 SHA-256，query-plan digest 是已验证 `AiQueryPlanV1` 的 RFC 8785 JCS SHA-256，其余逐字等于本轮 token/当前事实。COMPOSED 的十二项全部非空；EXECUTION_ATTEMPTED 的 `question_sha256` 固定 NULL（token 不承载原问题），其余非空。不得保存问题明文、filter literal、计划 JSON、字段/数据集显示名、SQL、结果、结果摘要、plan token、模型输出或 prompt 正文。

| 断言 | 必判事实 |
|---|---|
| `assert_ai_catalog_projection` | 投影与按权限/密级算出的期望 schema 集合逐元素相等；无权限字段不出现；目录中零业务值 |
| `assert_ai_plan_containment` | 单数据集、字段/聚合/过滤/分组/排序全部在本轮目录白名单，禁 join/SQL/任意表达式，limit 合法 |
| `assert_ai_filter_conjunction` | 当前记录谓词只能作为最外层 AND；OR、NOT 或模型声称自带权限谓词均拒绝 |
| `assert_ai_rejection_indistinguishable` | 不存在字段与存在但无权字段的 AI 响应体一致，P95 时间差不超过 5 ms |
| `assert_ai_inference_containment` | crate 静态扫描无 DB/HTTP/文件写；进程连接数为 0；模型输入恰好三项；多余字段拒绝 |
| `assert_ai_cache_partitioning` | 法人、用户、安全摘要、目录摘要、模型或提示版本任一变化即不命中且不混批 |
| `assert_ai_egress_containment` | 结果及派生字符串不回模型、不进日志、审计正文、dump 或幂等响应体 |
| `assert_ai_audit_completeness` | 每个可签发 compose 计划恰一条 COMPOSED、每个可开始 execute 查询恰一条 EXECUTION_ATTEMPTED；对象/action/strict after 与空值规则逐项相等；审计失败时不签发 token 或不执行查询 |
| `assert_ai_model_resource_containment` | 模型包签名/hash/只读 ACL、Job Object 限额与算定式一致；未认证不能激活；越限只失败 AI，不影响 core/integ/PostgreSQL |

历史研究稿列出 8 个名字却声明 9 条的矛盾由最后一条正式关闭。九条另立 `tests/ai_containment`，不并入或改名既有 RLS 断言。发布门禁固定为 `RG-AI-CONTAINMENT-GREEN` 与 `RG-AI-RESOURCE-CERTIFIED`。

## 4. 双向 MCP

### 4.1 协议与方法闭集

协议版本唯一固定为官方最终版 `2026-07-28`，JSON-RPC 固定为 `2.0`。只接受带 string/number `id` 的单一 JSON-RPC request，不接受 notification、response 或 batch。应用方法恰好六个：

```text
server/discover
tools/list
tools/call
resources/list
resources/templates/list
resources/read
```

`initialize`、`notifications/initialized` 与 `ping` 已从该最终协议版本移除；`notifications/cancelled` 只按第 4.5 节作为 stdio 传输控制，不是应用方法。除此之外均返回 HTTP 404、JSON-RPC `-32601` 与稳定错误 `MCP.METHOD.NOT_ALLOWED`。明确禁止 Sampling、Roots、Tasks、Logging、prompts、elicitation、completion、subscriptions、list-changed notifications、MRTR `input_required/inputResponses/requestState`、动态客户端注册 DCR、通用 SQL、shell、任意文件系统、任意 HTTP 与未知扩展方法。`GET /mcp` 与 `DELETE /mcp` 固定 405；只有 `POST /mcp`，不保留 legacy 独立 GET/SSE、恢复游标、事件重放或跨请求 stream。

每个 request 的 `params._meta` 都必须含以下三项：`io.modelcontextprotocol/protocolVersion` 精确为 `2026-07-28`；`io.modelcontextprotocol/clientCapabilities` 精确为空 object；`io.modelcontextprotocol/clientInfo` 在本产品收紧为必填且只含 1..64 个 Unicode 标量的 `name` 与 1..64 个 Unicode 标量的 `version`。`_meta` 是全部 request object 中唯一的有界扩展例外：三个保留成员之外可有 0..32 个扩展成员，key 是 1..128 ASCII bytes、必须匹配 `[A-Za-z0-9][A-Za-z0-9._/-]{0,127}` 且含至少一个 `/`；不得重复/大小写改写保留键，不得等于或以 `/` 尾段命中 `progressToken|logLevel|trace|baggage|inputResponses|requestState`。每个扩展 value 只能是合法 JSON，compact UTF-8 最大 16384 bytes、深度最大 8、每 object/array 最多 64 项、每 string 最多 4096 个 Unicode 标量，数字只允许 i64/u64 可表示整数；整个 `_meta` compact UTF-8 最大 65536 bytes 且仍计入 1 MiB 请求界。clientInfo 与扩展都是自报调试信息，永不参与身份、权限、路由或策略；扩展 value 验证后立即忽略，不进入日志、审计或响应。所有 success result 的 `_meta` 不使用该例外，恰含下文唯一 `serverInfo` object 且拒绝其他成员。

Streamable HTTP 每个 POST 必须同时带 `Content-Type: application/json`、`Accept: application/json, text/event-stream`、`MCP-Protocol-Version: 2026-07-28` 与大小写不敏感的 header name `Mcp-Method`；后两个 header value 必须与 body 逐字一致且大小写敏感。`Mcp-Name` 只在 `tools/call` 时必填并等于 `params.name`，在 `resources/read` 时必填并等于 `params.uri`，其余四方法必须缺席。header-safe value 指逐字节全部落在 ASCII `0x21..0x7e`、不含空白且不匹配 sentinel 的值，可原样发送；其他值必须把原 UTF-8 bytes 用 RFC 4648 standard Base64（`+/` alphabet、`=` padding 必须规范且不可省略）编码成精确 `=?base64?{Base64Value}?=`。解码前整 header value 最长 2743 bytes，解码后最长 2048 bytes；拒绝 base64url、缺/多 padding、非最短或 round-trip 后不逐字相等的编码、嵌套/多重 sentinel 与任何尾随字节。接收方只解码一次再与 body 比较。缺失、额外、非法 sentinel 或不一致均在认证、grant、binding 和业务执行前返回 HTTP 400、JSON-RPC `-32020` 与 `MCP.PROTOCOL.HEADER_MISMATCH`。协议版本不支持返回 HTTP 400、JSON-RPC `-32022` 与 `MCP.PROTOCOL.VERSION_UNSUPPORTED`；此分支只在 requested header/body 版本两者相等、恰为 1..64 bytes ASCII `0x21..0x7e` 且不等于支持版本时可达，其 error data 按下文唯一特例带 `supported=["2026-07-28"],requested=<原值>`；过长、非可见 ASCII、缺失或 header/body 不等均走 HEADER_MISMATCH，不回显输入。manifest tool schema 首版禁止 `x-mcp-header`，收到 ASCII 大小写不敏感前缀 `mcp-param-`（展示名 `Mcp-Param-*`）的任意 header 不用于路由/授权且按未知 transport header 忽略。`Authorization`、`Mcp-Name`、全部 ASCII 大小写变体的 `Mcp-Param-*` 与设备证明 header 在反向代理、core、gateway、tracing、access log、错误与 crash metadata 中一律按 secret redaction；实现契约必须把 normalized lower-case prefix `mcp-param-` 作为匹配输入，不能依赖大小写敏感 regex。Base64 只为传输编码，不被当成脱敏，canary 测试必须覆盖 mixed/lower/upper case 名称并证明 URI/对象标识零落盘。

入站 response 另有唯一产品 transport header `X-EP-MCP-Proof-Counter-Accepted`。只有下文原子 counter UPDATE 成功后，随后的即时 JSON、SSE 初始 HTTP response 与所有业务/授权/外部错误才必须带其已接受 u64 十进制值；UPDATE 前的 protocol/header/token/proof/rate/inflight 拒绝绝不带该 header。`MCP_INVOCATION_ATTEMPT` 只发生在 UPDATE 成功之后；若 ATTEMPT 持久化失败，该拒绝属于 UPDATE 后结果，必须带已接受的 header，且绝不调度任何 connector 副作用。客户端只在收到该 header 且值等于本次 counter 时推进本地 counter；无 header 保持原值，网络结果不确定则不得猜测，必须撤销 grant 并重签。反向代理必须透传但不得记录该 header，CORS 不新增通配暴露；canary 同样证明其不落日志、tracing、错误正文或审计。

入站成功一律即时返回 `Content-Type: application/json` 的单一 JSON-RPC response。出站客户端必须支持即时 JSON 和同一 POST 的 request-scoped SSE；本产品 SSE 收紧为只接受一条 terminal event，decoded body 必须逐字为 `event: message\ndata: <terminal-json>\n\n`，换行只用 LF，terminal JSON 本身不得含裸换行，不接受 comment、`id`、`retry`、progress notification 或第二条 event。terminal JSON-RPC message 最大 8388608 bytes，固定 SSE framing 另占 23 bytes，故 decoded SSE body 硬上限为 8388631 bytes；HTTP chunked-transfer/TLS framing 不计入该值。任何 server-to-client request、`input_required`、日志/订阅通知、第二个 terminal response、跨请求 id、terminal 超过 8 MiB、SSE body 超过 8388631 bytes 或超过 30 秒均中止；超字节界返回 `MCP.PAYLOAD.TOO_LARGE`，大小合法但 JSON/schema/字段非法才返回 `MCP.RESPONSE.SCHEMA_INVALID`。HTTP 客户端取消只关闭当前 response stream；不发送 HTTP cancel request。服务端不签发、不接受 `Mcp-Session-Id` 或 `Last-Event-ID`，远端要求 session/legacy stream 时判为不兼容。

请求 body 最大 1 MiB、响应 body 最大 8 MiB、普通调用绝对超时 30 秒。协议只接受精确版本，不做“最近兼容”或静默降级。`POST /mcp` 是全局同步 HTTP 8 秒 timeout/20-slot 交易闸门的唯一 MCP 具名例外：基础 transport/body/token identity 与 device proof 语法校验后，core 先按第 4.7 节生成 invocation/event id 并成功预留 completion spool slot，再进入独立的 connector-keyed 公平全局 semaphore（固定总容量 16）并取得该 connector 固定 4-slot；两层均不占普通交易 20-slot、不可借位、不可配置放大，无位即 `MCP.RATE_LIMITED` 且不消费 proof counter，但因 identity 已可信，仍使用已预留 slot 写本次 REJECTED completion。取得两层 permit 后才依次保留 core-owned rate timestamp、原子消费 device proof/grant counter；slot 失败时尚未取得 permit，且 rate timestamp、proof/grant counter、ATTEMPT 与 dispatch 全为零。route 专用 Tower timeout 固定 32000ms，MCP 30 秒 deadline 从受理 proof 起覆盖 binding/dispatch/response，余 2000ms 只供取消、审计 completion/spool 与 JSON-RPC error envelope；不得被 8 秒 `PLATFORM.SYSTEM.SYNC_TIMEOUT` 截断。普通 8 秒业务 route 不得同步等待 30 秒 outbound MCP；core 的 remote/local exchange 只能源自本 `/mcp` 专用 route 上下文或非 HTTP 内部 job，其他 owner 集成由 job-worker/既有 Outbox 异步触发。

所有成功 result 必须带 `resultType:"complete"` 及 `_meta.io.modelcontextprotocol/serverInfo={name,version}`；本平台入站的 name 固定 `ep-core-mcp`，version 取当前 Authenticode 签名产品构建版本。`server/discover` 必须返回 `supportedVersions:["2026-07-28"]`，capabilities 只按活动 manifest 非空集合出现 `tools:{"listChanged":false}` 和/或 `resources:{"subscribe":false,"listChanged":false}`，不返回 `instructions`、extensions 或其他 capability，并固定 `ttlMs:0,cacheScope:"private"`。`tools/list`、`resources/list`、`resources/templates/list`、`resources/read` 同样固定 `ttlMs:0,cacheScope:"private"`；出站只接受 `cacheScope:"private"` 与 `ttlMs` 0..600000，但产品无论其值均不缓存远端业务响应。

六方法 wire profile 是以下唯一形状；params 的未知业务字段失败，`_meta` 仅按上一段的保留规则处理：

| method | exact params | exact complete result 的业务字段 |
|---|---|---|
| `server/discover` | `{_meta}` | `supportedVersions,capabilities,ttlMs,cacheScope,_meta` |
| `tools/list` | `{_meta}` | `tools,ttlMs,cacheScope,_meta` |
| `tools/call` | `{_meta,name,arguments}`，arguments 必须 object | `content,structuredContent,isError,_meta` |
| `resources/list` | `{_meta}` | `resources,ttlMs,cacheScope,_meta` |
| `resources/templates/list` | `{_meta}` | `resourceTemplates,ttlMs,cacheScope,_meta` |
| `resources/read` | `{_meta,uri}` | `contents,ttlMs,cacheScope,_meta` |

本闭集不分页：五种 list/discover/read 请求均禁止 cursor，结果不得带 nextCursor；单次清单超过 256 项或 8 MiB 时 manifest 在发布阶段失败，而不是运行时截断。`tools` 按 name 排序，每项精确为 `{name,title,inputSchema,outputSchema}`：title 固定等于 name，两个 schema 从 binding 的 code/version 在受信 schema registry 解析并以规范 JSON 返回，不带 description、icons、annotations 或 execution。无变量的 literal resource 进入 `resources`，每项精确为 `{uri,name,title,mimeType:"application/json"}` 且 name/title 等于 uri；含变量的 binding 只进入 `resourceTemplates`，每项精确为 `{uriTemplate,name,title,mimeType:"application/json"}` 且 name/title 等于 uriTemplate；两表都按 URI bytes 排序。`resources/read.contents` 恰一项 `{uri,mimeType:"application/json",text}`，text 是允许字段投影后的 RFC 8785 JCS。成功 tool call 的 `content` 恰一项 `{type:"text",text:JCS(structuredContent)}`、`structuredContent` 是允许字段投影后的 JSON object、`isError=false`；owner command 的可披露业务失败使用 `content` 一项清洗后的稳定错误、无 structuredContent、`isError=true`。协议/认证/授权/收容失败一律用 JSON-RPC error，不伪装 tool result。

本平台向入站调用者生成的 JSON-RPC error envelope 固定为 `{"jsonrpc":"2.0","id":<原id；非法 JSON 或无法确定请求 id 时必须为 null>,"error":{"code":<numeric>,"message":<固定通用消息>,"data":<下述两形状之一>}}`。普通 data 恰为 `{"stable_code":"<稳定码>","request_id":"<服务端RequestId>"}`；唯一特例是 JSON-RPC `-32022` + `MCP.PROTOCOL.VERSION_UNSUPPORTED`，data 恰为 `{"stable_code":"MCP.PROTOCOL.VERSION_UNSUPPORTED","request_id":"<服务端RequestId>","supported":["2026-07-28"],"requested":"<1..64 visible-ASCII bytes>"}`。两种都 strict unknown-field rejection，字段顺序如上；`id` 成员永不省略，不得附 details、name/uri、payload、secret 或 policy facts。该形状不被错误地要求第三方远端原样生成；远端 error 的受理与清洗规则见第 4.4 节。transport 先后与映射固定如下：

| 条件 | HTTP | JSON-RPC code | stable code |
|---|---:|---:|---|
| 非法 JSON | 400 | -32700 | `MCP.REQUEST.INVALID` |
| batch、notification/response、非法 id/params/额外业务字段 | 400 | -32600 | `MCP.REQUEST.INVALID` |
| HTTP body 超过 1 MiB 或结果超过 8 MiB | 413 | -32600 | `MCP.PAYLOAD.TOO_LARGE` |
| protocol 不支持 | 400 | -32022 | `MCP.PROTOCOL.VERSION_UNSUPPORTED` |
| 必需 header 缺失、额外或与 body 不符 | 400 | -32020 | `MCP.PROTOCOL.HEADER_MISMATCH` |
| 非六方法或禁用 capability | 404 | -32601 | `MCP.METHOD.NOT_ALLOWED` |
| bearer/grant/session/account/device/法人/manifest 无效 | 403 | -32000 | `MCP.GRANT.INVALID_OR_EXPIRED` |
| device proof 无效、重放或乱序 | 403 | -32000 | `MCP.DEVICE_PROOF.INVALID` |
| tool/resource 不存在或不可见 | 404 | -32602 | 对应 `MCP.TOOL.NOT_VISIBLE_OR_DENIED` 或 `MCP.RESOURCE.NOT_VISIBLE_OR_DENIED` |
| connector 速率或在途上限 | 429 | -32000 | `MCP.RATE_LIMITED` |
| completion slot、审计写入或确认不可用 | 503 | -32000 | `MCP.AUDIT.UNAVAILABLE`；结果可能未知，调用方不得自动重试 |
| 30 秒绝对时限 | 504 | -32000 | `MCP.CALL.TIMEOUT` |

其余 manifest、credential、幂等、外部不可用、response schema、local containment 与高风险禁区按第 8 节专码返回 JSON-RPC `-32000` 和该码登记的 HTTP status；任何拒绝都发生在不该执行的 binding/外部副作用之前。

### 4.2 exact contract 与 manifest

新增 `crates/contract/mcp`（`ep-contract-mcp`）与 `crates/platform/mcp`（`ep-platform-mcp`）。前者只放 transport DTO/port，后者唯一负责 manifest 校验、grant、绑定解析、逐次授权与审计。业务模块只能提供既有 command/query binding，不能依赖 transport 或直接发 MCP 请求。

manifest 唯一 ABI：

```rust
pub const MCP_PROTOCOL_VERSION: &str = "2026-07-28";

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum McpDirection { Inbound, Outbound }
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum McpTransport {
    InboundHttps,
    RemoteStreamableHttp,
    LocalSignedStdio,
    #[serde(rename = "LOCAL_WINDOWS_HYPERV_CONTAINER")]
    LocalWindowsHyperVContainer,
}
#[derive(Clone, Debug, Serialize, Deserialize, Ord, PartialOrd, Eq, PartialEq)]
pub enum McpAllowedMethod {
    #[serde(rename = "server/discover")] ServerDiscover,
    #[serde(rename = "tools/list")] ToolsList,
    #[serde(rename = "tools/call")] ToolsCall,
    #[serde(rename = "resources/list")] ResourcesList,
    #[serde(rename = "resources/templates/list")] ResourceTemplatesList,
    #[serde(rename = "resources/read")] ResourcesRead,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum McpBindingMode { Query, ExistingCommand }

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum McpCredentialUsageV1 {
    HttpAuthorizationBearer,
    LocalSecretPipeUtf8,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct McpCredentialBindingV1 {
    pub reference: WindowsCredentialRef,
    pub usage: McpCredentialUsageV1,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct McpConnectorManifestV1 {
    pub schema_version: u16,                 // 必须为 1
    pub protocol_version: String,            // 必须为 2026-07-28
    pub connector_code: String,
    pub direction: McpDirection,
    pub transport: McpTransport,
    pub allowed_methods: BTreeSet<McpAllowedMethod>,
    pub remote: Option<McpRemoteOriginV1>,
    pub local: Option<McpLocalLaunchV1>,
    pub credential: Option<McpCredentialBindingV1>,
    pub tools: Vec<McpToolBindingV1>,
    pub resources: Vec<McpResourceBindingV1>,
    pub limits: McpLimitsV1,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct McpRemoteOriginV1 {
    pub scheme: HttpsOnly,
    pub host_ascii: String,
    pub port: u16,
    pub path: String,
    pub tls_spki_sha256: NonEmptyVec<Sha256Digest>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct McpLocalLaunchV1 {
    pub package_digest: Sha256Digest,
    pub entrypoint_relative: Option<String>,
    pub argv_literals: Vec<String>,
    pub container_image_digest: Option<Sha256Digest>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct McpLimitsV1 {
    pub max_request_bytes: u32,              // 必须为 1_048_576
    pub max_response_bytes: u32,             // 必须为 8_388_608
    pub call_timeout_ms: u32,                // 必须为 30_000
    pub max_inflight_per_connector: u16,     // 必须为 4
    pub max_calls_per_minute: u16,           // 必须为 60
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct McpToolBindingV1 {
    pub external_name: String,
    pub mode: McpBindingMode,
    pub internal_operation_code: String,
    pub request_schema_code: String,
    pub request_schema_version: u16,
    pub response_schema_code: String,
    pub response_schema_version: u16,
    pub capability_domain: CapabilityDomain,
    pub action_class: ActionClass,
    pub permission_code: PermissionCode,
    pub object_type: String,
    pub allowed_input_fields: BTreeSet<String>,
    pub allowed_output_fields: BTreeSet<String>,
    pub idempotency_required: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct McpResourceBindingV1 {
    pub uri_template: String,
    pub internal_query_code: String,
    pub response_schema_code: String,
    pub response_schema_version: u16,
    pub capability_domain: CapabilityDomain,
    pub permission_code: PermissionCode,
    pub object_type: String,
    pub allowed_output_fields: BTreeSet<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct McpManifestSignatureEvidenceV1 {
    pub connector_id: Uuid,
    pub version_no: u32,
    pub signature_key_ref: String,
    pub signature_key_version: String,
    pub signer_subject: String,
    pub signature_p1363_b64url: String, // RFC 4648 §5 no-pad canonical；解码恰 64-byte low-S P1363，禁止整数数组/DER/hex/padding
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct McpExchangeRequestV1 {
    pub invocation_id: Uuid,
    pub request_id: RequestId,
    pub trace_id: TraceId,
    pub manifest_canonical: Vec<u8>,
    pub manifest_digest: Sha256Digest, // 已批准 connector McpManifestV1 canonical digest，非 inner artifact manifest digest
    pub manifest_signature: McpManifestSignatureEvidenceV1,
    pub method: McpAllowedMethod,
    pub payload: serde_json::Value,
    pub idempotency_key: Option<String>,
    pub materialization: Option<McpLocalMaterializationV1>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct McpLocalMaterializationV1 {
    pub install_receipt_id: Uuid,
    pub installed_root_ref: String,
    pub package_digest: Sha256Digest,
    pub manifest_digest: Sha256Digest, // 必须等于 request 顶层的 connector manifest digest
    pub container_image_digest: Option<Sha256Digest>,
    pub hcs_image_identity: Option<String>,
    pub installed_root_sd_sha256: Sha256Digest,
    pub sandbox_profile_name: Option<String>,
    pub sandbox_sid: Option<String>,
    pub wfp_provider_guid: Option<Uuid>,
    pub wfp_sublayer_guid: Option<Uuid>,
    pub wfp_connect_v4_filter_key: Option<Uuid>,
    pub wfp_connect_v6_filter_key: Option<Uuid>,
    pub wfp_recv_accept_v4_filter_key: Option<Uuid>,
    pub wfp_recv_accept_v6_filter_key: Option<Uuid>,
    pub wfp_resource_assignment_v4_filter_key: Option<Uuid>,
    pub wfp_resource_assignment_v6_filter_key: Option<Uuid>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct McpExchangeReplyV1 {
    pub request_id: RequestId,
    pub payload: serde_json::Value,
    pub payload_digest: Sha256Digest,
}

#[async_trait]
pub trait McpTransportPort: Send + Sync {
    async fn exchange(
        &self,
        request: McpExchangeRequestV1,
    ) -> Result<McpExchangeReplyV1, AppError>;
}
```

`materialization` 对 INBOUND/REMOTE 必须为空，对两种 LOCAL 必须非空并逐项等于当前 manifest version 的已安装收据；stdio 的两个 container 字段为空且十个 sandbox profile/SID/WFP 字段全有，Hyper-V container 恰好相反。两者都带非空 `installed_root_sd_sha256`。`installed_root_ref` 只能使用第 3.2 节 exact `ep-install://` 形状；`plugin-host` 通过编译期固定根解析，不接收或拼接任意绝对路径。`McpExchangeRequestV1` 是进程内 contract；不得把它作为一个普通 IPC JSON 帧序列化。

`mcp.remote.exchange.v1` 与 `mcp.local.exchange.v1` 在普通 1 MiB IPC 帧之上统一使用独立 `McpExchangeChunkStreamV1`，从而承载 1 MiB 请求加 manifest 封套及最多 8 MiB 响应，不提高全局普通帧上限。它与阶段 3 承载附件/病毒扫描的 `BoundedChunkStreamV1` 是两个不同的 strict DTO：后者 10/30/3600 秒和大文件语义不得套到 MCP。每个 JSON 帧仍是 4-byte big-endian length prefix，整帧不超过 1048576 bytes；decoded chunk 最大 524288 bytes，bytes 用 base64url-no-pad 表示。frame 是请求、授权、响应两个顺序状态机共用的 strict tagged union，恰有九个 variant：

```rust
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(tag = "frame_kind", rename_all = "SCREAMING_SNAKE_CASE", deny_unknown_fields)]
pub enum McpExchangeChunkStreamFrameV1 {
    RequestBegin {
        stream_id: Uuid, invocation_id: Uuid, request_id: RequestId, operation: String,
        trace_id: TraceId, method: McpAllowedMethod, declared_manifest_bytes: u32,
        declared_payload_bytes: u32,
        manifest_digest: Sha256Digest,
        manifest_signature: McpManifestSignatureEvidenceV1,
        materialization: Option<McpLocalMaterializationV1>,
        idempotency_key: Option<String>,
    },
    RequestChunk {
        stream_id: Uuid, sequence_no: u32, section: McpChunkSectionV1,
        decoded_len: u32, bytes_b64url: String,
    },
    RequestEnd {
        stream_id: Uuid, next_sequence_no: u32,
        manifest_digest: Sha256Digest, payload_digest: Sha256Digest,
    },
    DispatchAuthorized {
        request_stream_id: Uuid, attempt_event_id: Uuid,
    },
    ResponseBegin {
        stream_id: Uuid, request_stream_id: Uuid, request_id: RequestId,
        declared_payload_bytes: u32, payload_digest: Sha256Digest,
    },
    ResponseChunk {
        stream_id: Uuid, sequence_no: u32,
        decoded_len: u32, bytes_b64url: String,
    },
    ResponseEnd {
        stream_id: Uuid, request_stream_id: Uuid,
        next_sequence_no: u32, payload_digest: Sha256Digest,
    },
    Abort { stream_id: Uuid, reason: McpStreamAbortReasonV1 },
    Ack { stream_id: Uuid, next_sequence_no: u32, state: McpStreamAckStateV1 },
}
```

`McpChunkSectionV1` 只有 `MANIFEST|REQUEST_PAYLOAD`；abort reason 只有 `CLIENT_DISCONNECT|DEADLINE|SIZE_LIMIT|HASH_MISMATCH|PROTOCOL_ERROR|RATE_LIMIT|AUDIT_UNAVAILABLE`；ack state 只有 `CONTINUE|COMPLETE|ABORTED`。`RequestBegin` 是请求流唯一首帧，`ResponseBegin` 是响应流唯一首帧；两种 chunk 的 `sequence_no` 都从 0 独立连续递增。每个 chunk 被接收后必须取得同 `stream_id`、`next_sequence_no=sequence_no+1` 且 `state=CONTINUE` 的 Ack 才能发送下一块；`ResponseEnd` 只有在 raw terminal bytes 的声明/实际长度、序号与 SHA-256 摘要通过后才取得同 stream id、同 `next_sequence_no` 且 `state=COMPLETE` 的 Ack。该 COMPLETE **不表示业务 JSON/schema/allowed-fields 合法**；唯一有受信 schema registry 的 caller 在收齐并 hash 通过后才做 size-first strict parse/schema/field 校验，失败写本次正常 terminal completion outcome 并返回 `MCP.RESPONSE.SCHEMA_INVALID`，不倒退 Ack 或重放 dispatch。收到 `Abort` 的一方恰回一个同 stream id、当前 next sequence 且 `state=ABORTED` 的 Ack 后关闭连接。Begin 与 `DispatchAuthorized` 不产生 Ack，CONTINUE/COMPLETE/ABORTED 不得用于上述指定位置之外；错误 Ack 或 End 校验失败时，如本方尚未发送 Abort，则至多发送一个 `Abort{reason=PROTOCOL_ERROR|SIZE_LIMIT|HASH_MISMATCH}`，随后双方清零缓冲并关闭，不再发 Ack 或第二个 Abort。End 的 `next_sequence_no` 精确等于该方向已经接受的 chunk 数，空 payload 时为 0，避免“最后索引”歧义。

Request manifest 必须全部在 request payload 前。receiver 在 `RequestEnd` 后按顺序完成长度、两个 digest、manifest 签名/trust-bundle、materialization 与 schema 校验，再取得自身唯一 owner 的 global→connector inflight permit 并成功保留本次 rate timestamp；只有这些全部成功，`RequestEnd` 才取得 `Ack{state=COMPLETE}`。integration-gateway/plugin-host 坚持 0 DB，不做“当前 connector 状态”猜测；ENABLED、唯一 ACTIVE manifest 及其 digest、当前人类 identity/authz/binding 的唯一判定者是有数据库的 caller（core-server/job-worker）。rate/inflight 拒绝由 receiver 发送 `Abort{reason=RATE_LIMIT}`，caller 回 `ABORTED` Ack 后关闭并以 `MCP.RATE_LIMITED` 写仅 completion 审计；此路没有 ATTEMPT、counter 或 dispatch。`RequestEnd` 的 COMPLETE 只表示“请求已验证且出站 rate 已保留”，**绝不授权外呼或启动本地进程**。caller 收到 COMPLETE 后先在同一专用 DB connection/共享锁内再读当前 identity/authz/binding 与活动 manifest digest，全部不变后才独立提交确定性 `MCP_CALL_ATTEMPT`；commit 成功后恰发送一帧 `DispatchAuthorized{request_stream_id,attempt_event_id}`，其中 event id 必须逐字等于 `UUIDv5(MCP_AUDIT_NAMESPACE, lowerhex(RequestBegin.invocation_id bytes)||":ATTEMPT")`。receiver 只在同连接、同已完成 request stream、尚未 dispatch 且该确定性 id 相等时恰 dispatch 一次；重复、提前、错 id 或错 stream 均为 `PROTOCOL_ERROR` 且零 dispatch。ATTEMPT commit 失败时 caller 发送 `Abort{reason=AUDIT_UNAVAILABLE}`；receiver 回 ABORTED、释放 permit 且零 dispatch，已成功保留的 outbound rate timestamp按本节规则不退。caller/连接在 COMPLETE 后、dispatch authorization 前断开时 receiver 也必须零 dispatch并释放 permit；若 caller 已提交 ATTEMPT而终态不可判，则只由第 4.7 节 reconciler 记录 `UNKNOWN_AFTER_CRASH`，绝不重放调用。

response 使用新的 `stream_id`，其 `request_stream_id` 必须等于已获有效 `DispatchAuthorized` 的请求流 id，且只能在该 dispatch 得到终态后开始；response chunk 只携带远端 HTTP 或本地 child 产生的 exact 单一 terminal JSON-RPC response bytes，不再序列化 `McpExchangeReplyV1` wrapper，也不重复 manifest/materialization/signature。`ResponseBegin/End.payload_digest` 都是该 exact response bytes 的 SHA-256；core 在 `ResponseEnd` 校验长度/hash 后才 strict parse、schema/字段裁剪并构造进程内 `McpExchangeReplyV1`。这样合法 8388608-byte response 不因 wrapper 字段或 JCS 转义膨胀而被误拒。每条连接同一时刻至多一个请求/授权/响应状态机；两个方向不得共用 sequence 或 stream id。

manifest 上限 262144、request 1048576、exact terminal response 8388608 bytes；30 秒绝对时限只由 receiver 在收到 `RequestBegin` 时以本机单调时钟计算，并覆盖请求校验、rate reservation、等待 `DispatchAuthorized`、dispatch 与响应传完，不能被 caller wall clock 覆盖，也不能因 COMPLETE、授权帧或 `ResponseBegin` 重置。wire 不携带 wall-clock deadline 字段。任一 declared/actual size、顺序、base64、hash、request-stream 关联、重复 stream、本地单调 deadline 或尾随帧不符，按上一段的 Ack/Abort 状态机清零缓冲并关闭连接。request payload 是 `McpExchangeRequestV1` 中 payload 的 RFC 8785 JCS bytes，response 是上段 exact JSON-RPC bytes；除上文 request `params._meta` 的唯一有界 extension map 例外外，所有 object、internally tagged enum 与 frame 均拒绝未知字段。该流只属于现有两个 exchange operation，不新增可授权业务 operation，也不得落盘或跨连接恢复。

manifest 采用 RFC 8785 JCS。所有集合按各自规范键排序、去重，未知字段失败；canonical manifest 最大 262144 bytes，tools 与 resources 各最多 256 项。`allowed_methods` 必须与上述六方法集合逐元素相等。`tools` 与 `resources` 可分别为空，但二者不能同时为空；某类为空时对应 capability 省略且 list 返回空集合，不能删应用方法形成第二套兼容面。远端 SPKI pins 由 `NonEmptyVec` 保证非空。INBOUND 的 credential 必须为空；REMOTE 为空或恰为 `HTTP_AUTHORIZATION_BEARER`；`LOCAL_SIGNED_STDIO` 为空或恰为 `LOCAL_SECRET_PIPE_UTF8`；`LOCAL_WINDOWS_HYPERV_CONTAINER` 首版必须为空，因为宿主 anonymous-pipe handle 不可跨 Hyper-V utility VM 边界复用。其他组合拒绝。`LOCAL_SIGNED_STDIO` 的 `entrypoint_relative=Some(<canonical in-root .exe>)`、`container_image_digest=None`，argv 可为规范 literal 数组；`LOCAL_WINDOWS_HYPERV_CONTAINER` 的 `entrypoint_relative=None`、`container_image_digest=Some(...)` 且 `argv_literals=[]`，不得用空字符串冒充 absent。每个 tool/resource 的 `allowed_output_fields` 必须非空，tool 的 `allowed_input_fields` 仅在其请求 schema 确实无字段时可为空。Query binding 的 `idempotency_required=false`；ExistingCommand 必须为 true，并且 `internal_operation_code` 必须逐字命中编译期 command registry。资源只返回受治理的 `application/json`，不得映射本地路径、附件正文、数据库表名或 SQL；所有 manifest input schema 禁止 `x-mcp-header`。

`max_calls_per_minute=60` 的算法固定为无 burst 的进程内 60 秒 sliding window，以单调时钟记录受理 timestamp；六个方法全部计数。语法/认证/proof/rate/inflight 拒绝不计数；INBOUND 在 counter UPDATE 失败时原子移除预留 timestamp，而 counter UPDATE 一旦成功即计数，后续当前事实、audit attempt、binding、owner/外部调用或响应失败均不退；OUTBOUND 无 grant，rate reservation 一旦成功即计数，随后的 audit attempt 失败也不退，但仍零 dispatch。INBOUND 唯一 owner 是 core、REMOTE 唯一 owner 是 integration-gateway（统一合并 core 与 worker 调用）、LOCAL 两种唯一 owner 是 plugin-host；不得由各 caller 各算一份。INBOUND 顺序唯一为 identity/proof 语法→ids→caller completion slot reserve→global permit→connector permit→core rate timestamp reserve→counter UPDATE→当前事实/binding/authz→ATTEMPT→dispatch；OUTBOUND 顺序唯一为 caller identity→ids→caller completion slot reserve→connector 共享状态锁+当前 ENABLED/ACTIVE/identity/authz/binding 重读→receiver global/connector permit→receiver rate timestamp reserve→RequestEnd COMPLETE→caller 锁内最终重读→caller ATTEMPT commit→`DispatchAuthorized`→receiver dispatch→terminal completion→释放共享锁。共享锁是 caller 使用专用 DB connection 取得的 PostgreSQL session advisory shared lock，key 恰为 `hashtextextended(lower(connector_id::text),5568499270196608077)`；connector enable/disable/revoke 及 ACTIVE manifest 切换在同 key 上取 transaction advisory exclusive lock后才重读 `row_version`并变更。因而 disable/revoke/切换必须等待已 dispatch 的最多 30 秒调用终态，而其提交后绝无使用旧状态的新 dispatch；caller 崩溃时 DB connection 关闭自动释放锁，hash 碰撞只会多串行不会放宽。取锁/重读失败时已有 completion slot，写 REJECTED completion，零 receiver permit/rate/ATTEMPT/dispatch。三个 owner 每次冷启动或 limiter 状态丢失时都先填入 60 个当前单调时点的 synthetic timestamp，固定冷却 60 秒后才逐步恢复，因此重启不能制造额外 burst；wall-clock 回拨不影响窗口。per-connector inflight 固定 4，无等待，释放只在终态/取消完成；rate 与 inflight 任一拒绝都返回 `MCP.RATE_LIMITED`。

manifest detached signature 唯一算法为 ECDSA P-256/SHA-256，生产签名经 `KmsBackend` 使用受信发布密钥的独立 purpose `MCP_MANIFEST_V1`，私钥不出内置 KMS/HSM。签名输入精确为 `SHA-256("EP-MCP-MANIFEST-V1\0" || connector_id[16-byte UUID] || version_no[u32 big-endian] || SHA-256(JCS(manifest)))`，签名编码为 canonical low-S IEEE-P1363 `r||s` 64 bytes；数据库同时不可变保存 `signature_key_ref,signature_key_version,signer_subject`。验签每次都核对 purpose、key version、subject、信任链与吊销状态；retired 但未 revoked 的旧验证公钥只可验证历史行。轮换不修改历史行，后续升级/回退产生的新版本必须用当前活动 key 重新签名；本机接受签名的吊销更新后，引用 revoked key 的 manifest 立即不可启用/调用。外层 config package 签名不能替代此验证，二者任一失败均整包拒绝。

`integration-gateway` 与 `plugin-host` 不取得 KMS，其唯一验证根是离线发布的 `McpManifestTrustBundleV1`。bundle 是最大 1048576 bytes 的 RFC 8785 strict JCS，字段恰为 `schema_version=1,bundle_id,release_batch_id,generated_at,entries[]`；UUID 均小写规范文本、时间为 UTC 秒精度。entry strict 字段恰为 `purpose="MCP_MANIFEST_V1",key_ref,key_version,signer_subject,spki_der_b64url,spki_der_sha256,status`，status 只取 `ACTIVE|RETIRED|REVOKED`；SPKI 必须是 DER-encoded P-256 public key，base64url-no-pad 解码 1..4096 bytes 且 digest 为其 SHA-256 lowerhex。entries 为 1..256 项，按 `(key_ref UTF-8 bytes,key_version UTF-8 bytes)` 排序去重，同一 tuple 只能一项。

bundle exact JCS 存为 `mcp-manifest-trust-bundle.jcs.json`，相邻 detached CMS 为 `mcp-manifest-trust-bundle.p7s`；CMS detached content 恰为该 JCS bytes，签名/链/吊销/算法复用生产配置包 release root，且两文件还必须来自同一 Authenticode 签名离线发布 CAB。固定安装目录为 `C:\ProgramData\EnterprisePlatform\trust\mcp-manifest\`，owner SYSTEM、断继承；SYSTEM/Administrators/ep-ops 可管理，`NT SERVICE\ep-integ` 与 `NT SERVICE\ep-plugin` 仅 read/`READ_CONTROL`，其余账户无 ACE。两个进程启动时逐次核文件名、owner/DACL、JCS digest、CMS、release batch 与 SPKI；只接受 bundle 中 ACTIVE 或 RETIRED 且非 revoked 的 exact key，并在 health/audit correlation 中只记录 `bundle_id+bundle_digest`。更新/回退只能在 MCP 全局 gate 已关闭且两个进程停止的离线维护窗口内，以同目录 staging 验完整 CAB/JCS/CMS 后 write-through 原子替换两文件并重启；运行中检测文件 identity/digest 改变立即关闭 gate，绝不热重载。吊销的“立即”以客户本机接受受信的 signed revocation/bundle 发布为事实边界：流程先关 gate，再安装含 REVOKED 项的新 bundle、重启并复验，完成前不得重新开放 MCP；在线下载、远端吊销查询、只换一份文件或运行中替换均禁止。`McpManifestSignatureEvidenceV1` 随每次 exchange 传入并逐项匹配进程启动时固定的 bundle；bundle 不含私钥。

`McpRemoteOriginV1` 只允许 HTTPS、固定 host/port/path 与至少一个 SPKI pin；禁止 userinfo、fragment、通配域、重定向、系统代理和 DNS search suffix。gateway 每次解析后拒绝 loopback、link-local、multicast、私网地址和与 manifest 不一致的地址，防止 DNS rebinding/SSRF。

`McpLocalLaunchV1.entrypoint_relative` 仅对 `LOCAL_SIGNED_STDIO` 必须为签名包根内的单个规范化相对 `.exe` 路径，禁止 `..`、UNC、盘符、ADS、shell、环境变量展开与命令替换。stdio 的 `argv_literals` 为 0..32 项，每项 1..2048 UTF-16 code units，拒绝 NUL、未配对 surrogate、U+0001..U+001F 与 U+007F；总 command line 连终止 NUL 不超过 32767 UTF-16 units。`CreateProcessW` 的 `lpApplicationName` 固定为经 root resolver 得到的 exact 绝对 entrypoint，`lpCommandLine` 由 `[absolute_entrypoint] + argv_literals + [可选系统生成的 --ep-credential-handle=<uppercase-hex>]` 唯一编码：每项都加双引号；引号前连续 N 个反斜杠编码为 `2N+1` 个反斜杠再加引号，结束引号前连续 N 个反斜杠编码为 `2N` 个；其他 code unit 原样，项间一个 U+0020。编码后必须用 `CommandLineToArgvW` round-trip 得到逐项完全相等的 vector 才可启动；不经过 `cmd.exe`、PowerShell、环境展开或自定义 shell。manifest 参数禁止系统保留前缀 `--ep-credential-handle=`。`LOCAL_WINDOWS_HYPERV_CONTAINER` 必须令 entrypoint 为 `None` 且 `argv_literals=[]`，只按已签名 OCI config/HCS identity 启动。

tool `external_name` 为 1..128 bytes 的 ASCII `[A-Za-z0-9][A-Za-z0-9._-]{0,127}`，大小写敏感且全 manifest 唯一。resource template 最大 2048 UTF-8 bytes，固定使用 RFC 6570 Level 1 simple expansion，变量最多 16 个且只取 `[a-z][a-z0-9_]{0,31}`；scheme 是无变量的小写 literal，只允许 `https|mcp|ep|urn`，全部禁止 userinfo、fragment、反斜杠、控制符与 `file|data|javascript`。形状按 scheme 唯一分支：`https` 必须是 `https://<literal-ascii-host>[:nondefault-port]/<absolute-path>`；`mcp|ep` 必须是 `mcp://<literal-lowercase-authority>/<absolute-path>` 或 `ep://...`，authority 取 `[a-z0-9][a-z0-9.-]{0,127}` 且禁止端口；三种 hierarchical form 允许唯一 query，变量只能占据完整 path segment 或完整 query value。`urn` 必须是无 authority、无 `/ ? # @` 的 opaque `urn:<nid>:<nss-components>`；nid 取 `[a-z0-9][a-z0-9-]{0,31}`，nss 是一项以上由冒号分隔的 literal `[A-Za-z0-9._~-]+` 或完整 `{variable}`，变量不能嵌入 literal。

hierarchical 规范化固定为 scheme/host/authority 小写、移除 https 默认 443 与 path dot-segment、percent-encoding 十六进制大写且 unreserved 字符解码、query key 唯一并按 key/value 排序；URN 只小写 scheme/nid，NSS literal 大小写保留且不做 path/host/query 规则。变量展开后为 1..256 UTF-8 bytes，禁止 `.|..|NUL|/|\\`，重复变量值必须相等，展开后再按对应 scheme canonicalizer 校验且结果必须仍落同一 literal authority。固定正例为 `https://api.example.cn/customers/{id}`、`mcp://erp/orders/{order_id}`、`ep://reporting/datasets/{dataset}`、`urn:ep:asset:{asset_id}`；`urn://ep/...`、`urn:ep:item?x=1`、`mcp:erp:item`、`https://{host}/x`、mcp/ep 带端口、嵌入变量或 Level 2+ expression 均为负例。manifest 发布、grant 签发、list 投影、read 匹配和审计全部复用同一 `McpResourceTemplateV1` 编译器；任何非规范 URI 先拒绝，不做“先匹配后归一化”。

connector 与 manifest 不提供任意 CRUD。新增配置发布 `ItemKind::MCP_CONNECTOR` 与 `ItemKind::MCP_MANIFEST_VERSION`，只允许签名配置包经既有自动测试、审批、发布和回退流程登记新 connector/版本。`SUPERSEDED` 永远是终态，回退不得重开旧行：系统复制所选历史 APPROVED/SUPERSEDED 行已验真的 canonical manifest、artifact/origin identity 与 digest，令 `version_no=max+1`，用当前 `MCP_MANIFEST_V1` key 重新签名并生成全新的 DRAFT 行，再完整经过自动测试、审批、APPROVED→ACTIVE；新行激活与当前 ACTIVE→SUPERSEDED 同事务完成，历史行不删除、不改状态。`manifest_digest` 不设唯一约束，允许该受控复制。ServerAdmin/API 只有以下管理面：

| 方法与路径 | 语义 |
|---|---|
| `GET /api/v1/platform/mcp-connectors` | 按当前管理范围分页查看，不返回 secret |
| `GET /api/v1/platform/mcp-connectors/{id}` | connector、活动 manifest 摘要、健康与最近审计摘要 |
| `GET /api/v1/platform/mcp-connectors/{id}/manifest-versions` | 查看不可变版本与审批/签名状态 |
| `POST /api/v1/platform/mcp-connectors/{id}/actions/enable` | `row_version`；需 `platform.mcp.connector.manage`、Idempotency-Key、活动已批准 manifest 与 credential probe |
| `POST /api/v1/platform/mcp-connectors/{id}/actions/disable` | `row_version,reason`；同权限与幂等，数据/版本保留 |

F-55 MCP connector 的持久 secret 不经上述 API，只存 Windows Credential Manager；这句话不覆盖平台通用 `secret://` KMS 机密。Microsoft `CredWriteW` 把新 credential 关联到 current token 的 logon session，普通管理员直接调用只会写入管理员自己的 credential set，不能替 `NT SERVICE\ep-integ` 或 `NT SERVICE\ep-plugin` 初始化 vault。唯一可实现入口固定为产品签名、进入 SBOM 的 `ep-secretctl wincred` 加目标服务本地维护管道；最终 `CredWriteW(CRED_TYPE_GENERIC,CRED_PERSIST_LOCAL_MACHINE)` 或 `CredDeleteW` 必须由目标服务在自身 current token 下执行。CredentialBlob 产品上限统一为 1..2560 bytes，逐字取 Windows SDK 的 `CRED_MAX_CREDENTIAL_BLOB_SIZE = 5*512`；实现者可核对 Microsoft Learn 的 [CREDENTIALW](https://learn.microsoft.com/en-us/windows/win32/api/wincred/ns-wincred-credentialw) 与 [CredWriteW](https://learn.microsoft.com/en-us/windows/win32/api/wincred/nf-wincred-credwritew)。2561 bytes 必须在调用 Win32 API 之前拒绝。

`ep-secretctl` 的顶层子命令闭集为 ADR-0007 冻结的八项，`wincred` 是其中唯一 WinCred 入口；工具不直接写 vault，也不经 HTTP、ServerAdmin、数据库通用写 API、argv、环境变量或文件传递 secret。SCM 必须正常启动目标服务并加载服务虚拟账户 profile；禁止管理员进程模拟服务 token，也不提供手工 `LoadUserProfile` 旁路。目标服务收到 SCM 自定义 control code `200` 后先撤下该 target 的新 dispatch、最多 30 秒排空在途，再进入一次性维护窗口：`ep-integ` 只在窗口内创建 `\\.\pipe\ep-integ-secretctl`，`ep-plugin` 创建 `\\.\pipe\ep-plugin-secretctl`。两管道都固定 `PIPE_REJECT_REMOTE_CLIENTS`、首实例、handle 不继承，DACL 只授目标服务 SID、SYSTEM 与 BUILTIN\Administrators。服务在读取 payload 前冒充客户端，要求本机交互式、完整提升的 Administrators token，拒绝 network/service/batch/anonymous token；再以 `GetNamedPipeClientProcessId` 打开并持有客户端进程句柄，核对映像为安装根内的 `ep-secretctl.exe`、Authenticode 有效且 PE digest 命中本发布清单。PID 本身不构成身份。

维护状态机固定 `CLOSED→QUIESCING→OPEN→APPLYING→PROBING→COMMITTED→CLOSED`，失败走 `APPLYING|PROBING→ROLLING_BACK→CLOSED_FAILED→CLOSED`。OPEN 最长 60 秒、只接受一个连接和一个 grant nonce；只有 APPLYING 前的超时、断连或 SCM stop 才能销毁管道、清零后直接回 CLOSED，APPLYING 开始后的断连、超时或正常 stop 必须先走 ROLLING_BACK。`CLOSED_FAILED` 已无管道且 connector 仍 DISABLED；失败 receipt 与 Event Log 均耐久写成后，唯一允许的出边才是 `CLOSED_FAILED→CLOSED`，不重试、不启用，全部未列边均为非法并须负测。强杀、崩溃、断电等不能执行 rollback 的恢复逐字复用配置参考第 5 节的固定 16384-byte 非秘密 write-through intent：首次 Win32 mutation 前必须先落 `APPLY_INTENT`；下次 SCM 启动在任何 credential read/egress/pipe 前由残留 intent 重建 CLOSED_FAILED，耐久写 `RECOVERY_REQUIRED` receipt/Event Log/phase 后才回 CLOSED，并且只有新的双人 grant 对同 target/purpose 纠正成功才解除 recovery，绝不猜测成功或自动使用残留 credential。operation 闭集只有 `wincred.provision.apply.v1`。第一帧为最多 65536 bytes strict JSON `{request_id,grant_jcs,grant_cms_signatures,action,target_ref,purpose,secret_len}`；它只承载非秘密授权元数据，65536 是 metadata frame 上限，不放宽后续 CredentialBlob 的 2560-byte 硬上限。action 只取 `CREATE|ROTATE|DELETE`。共享 `WinCredProvisionPurposeV1` 仍以配置参考第 5 节的三项为唯一全平台 enum；本 F-55 MCP 范围只允许 `ep-integ+MCP_REMOTE_BEARER` 或 `ep-plugin+MCP_STDIO_SECRET`，必须在读 secret frame 前拒绝 `ESIGN_API_CREDENTIAL` 与所有 recipient/purpose 交叉组合，但不得从共享 enum 删除 Stage 6 使用的第三项。`WinCredProvisionGrantV1` exact JCS 逐字绑定 schema_version=1、deployment_id、recipient_service、action、target_ref、purpose、request_id、一次性 nonce、reason、not_before、expires_at 和两个不同批准人；有效期不超过 5 分钟，由签名部署清单中客户安全管理员证书闭集的两个不同 subject 各作一份 detached CMS 签名。服务核对 target 正被本进程当前 compatible ACTIVE manifest 引用、connector 已 DISABLED 且无在途调用；重放、同人双签、过期、范围不符、未知字段或额外 operation 均失败关闭。

CREATE/ROTATE 后紧跟恰好一个 binary frame `u32be(secret_len)||secret`，长度必须 1..2560 且与元数据相等；DELETE 固定 secret_len=0 且没有 secret frame。REMOTE 仍只允许 gateway 注入标准 `Authorization: Bearer <secret>`，secret 还须为 ASCII `0x21..0x7e` 且不得含空白、DEL、CR/LF 或其他控制字节；enable/probe 与每次使用前都验证，违反时在构造 header 和出网前返回 `MCP.CREDENTIAL.REF_INVALID`。不得选自定义 header、query、cookie、client certificate 或 body field。LOCAL_SIGNED_STDIO 的 secret 为 1..2560 UTF-8 bytes，plugin-host 仍以 4-byte big-endian 长度加 secret 后 EOF 写入专用匿名 pipe，通过显式 inherited-handle allowlist 只给目标子进程读端，并由系统追加非秘密 argv `--ep-credential-handle=<uppercase-hex-handle>`；manifest argv 禁止该保留前缀。LOCAL_WINDOWS_HYPERV_CONTAINER 首版 `credential=None`，没有 guest secret bootstrap、host-handle 桥接或环境变量回退。

目标服务以自身 token 先 CredRead 旧值，再执行写入和同 purpose probe。CREATE probe 失败即删除新条目；ROTATE probe 失败即从 zeroizing 内存回写旧 blob；DELETE 的 absence probe 失败则同样恢复旧 blob。任一恢复失败都令 connector 保持 DISABLED 并写高严重度 Event Log。成功、拒绝、回滚两端都以同一 request_id 写 Windows Event Log 并返回不超过 16384 bytes 的非秘密 receipt，只含 grant digest、target ref、purpose、action、两个批准人 subject、old_present、probe/result 稳定码和时间，不含 secret 或 secret hash。`ep-secretctl` 只通过关闭 echo 的 `ReadConsoleW` 读取并二次确认 secret，不接受重定向 stdin；console、pipe、CredRead、旧值、新值和 probe buffer 在每条路径显式 zeroize。轮换成功后 connector 仍为 DISABLED，须经既有 enable gate 另行启用。

### 4.3 入站 MCP

入站 MCP 只复用 `core-server` 现有员工 HTTPS listener 与 TLS/middleware，不新增端口。外部 MCP 客户端不能自注册，也不能使用服务账号；必须由已登录员工在现有会话中签发短期人类授权 grant。

grant API：

| 方法与路径 | 请求 | 权限与结果 |
|---|---|---|
| `POST /api/v1/platform/mcp-human-grants/actions/issue` | `legal_entity_id, connector_id, manifest_version_id, tool_names[], resource_uri_templates[], ttl_seconds, max_calls` | 需 `platform.mcp.grant.issue`；禁止 `Idempotency-Key`；返回一次性明文 token、grant_id、expires_at |
| `GET /api/v1/platform/mcp-human-grants?state=ACTIVE` | 游标分页 | 普通员工只能看自己；系统/安全管理员按现有范围查看 |
| `POST /api/v1/platform/mcp-human-grants/{id}/actions/revoke` | `row_version` | 本人或安全管理员；`Idempotency-Key` 必填 |

上述八个管理端点连同唯一协议端点 `POST /mcp` 的 exact wire 契约只登记在 `docs/openapi/mcp-management.v1.yaml`，首版路径数恰为 9、operation 数恰为 9；该文件不得注册 connector/manifest/grant 的任意 CRUD、secret/credential、artifact download/upload 或其他 action。五个 connector 端点的 `X-Client` 只允许 `server_admin`；三个 human-grant 端点只允许 `win|mac|ios|android|server_admin`，拒绝 `portal|ops|mcp`。`POST /mcp` 不接受外部自填 `X-Client:mcp` 来建立身份，`ClientKind::Mcp` 只由 grant middleware 内部赋值。connector GET/详情/版本列表要求 `platform.mcp.connector.manage + VIEW`，enable/disable 要求同 permission 的 UPDATE；grant issue 要求 `platform.mcp.grant.issue + CREATE`。grant owner 可在当前有效会话/设备下查看及撤销自己的 grant；非 owner 只有具备 `platform.mcp.connector.manage + VIEW`（查看）或 UPDATE（撤销），并通过目标 connector 当前对象范围时才可操作，不用中文角色名或“系统管理员”旁路权限目录。不可见 connector/version/grant 与不存在统一 404。

三个管理列表都只接受下表参数与固定倒序键集分页，`page_size` 默认 50、范围 1..100，禁止 `limit,page,sort,filter` 及未知 query；成功响应 `data` 恰为 `{items:[...]}`，`meta` 恰为 `{page_size,next_cursor}`，items 永不为 NULL，只有确定还有后页时才把本页末项编码为 next cursor，否则为 NULL：

| 端点 | 唯一 query | 固定排序/后续谓词 | cursor strict payload |
|---|---|---|---|
| connector list | `cursor?,page_size?` | `created_at DESC,id DESC`；`(created_at,id)<(...)` | `{schema_version:1,endpoint:"MCP_CONNECTORS",created_at,id}` |
| manifest versions | `cursor?,page_size?` | `version_no DESC,id DESC`；`(version_no,id)<(...)` 且 connector id 固定为 path id | `{schema_version:1,endpoint:"MCP_MANIFEST_VERSIONS",connector_id,version_no,id}` |
| ACTIVE grants | `state=ACTIVE,cursor?,page_size?`，state 必填且只允许该字面量 | `issued_at DESC,id DESC`；`(issued_at,id)<(...)` | `{schema_version:1,endpoint:"MCP_HUMAN_GRANTS_ACTIVE",issued_at,id,state:"ACTIVE"}` |

cursor grammar 统一为 `epcur1.<base64url-no-pad(RFC8785-JCS(payload))>`；解码后 1..512 bytes，body 1..683 字符、全 token 最长 690，重做 JCS/base64url-no-pad 必须与输入逐 byte 相等。时间使用 UTC 微秒精度 `YYYY-MM-DDTHH:MM:SS.ffffffZ`，UUID 为小写连字符文本，version_no 为正 i32；endpoint、path connector、state 或形状不符均为 `PLATFORM.REQUEST.INVALID_PAYLOAD`。cursor 只表示位置且不授权，服务端每页逐项重做当前法人、密级、记录范围和字段权限。

管理 view/command DTO 恰为以下闭集；所有 object 拒绝未知字段，所有摘要为 64 位 lowerhex，`row_version` 为 0..9223372036854775807 的 JSON integer，所有时间均为 UTC RFC3339：

- `McpConnectorListItemV1={id,code,name,direction,transport,status,security_level,data_scope_tags,row_version,created_at,updated_at}`。枚举逐字采用本节数据库闭集；code/name/tag 的长度与规范化逐字采用数据字典。
- `McpConnectorDetailV1` 是 list item 的全部字段再加 `active_manifest,health,recent_audit`。`active_manifest` 为 NULL 或 strict `{id,version_no,protocol_version:"2026-07-28",manifest_digest,signer_subject,status:"ACTIVE",approved_at,activated_at}`；非空时两时间非空且逐字属于该 connector 当前唯一 ACTIVE 行。`health` 恰为 `{state,checked_at,stable_code}`：`HEALTHY` 时 checked_at 非空/code NULL，`DEGRADED|UNAVAILABLE` 时二者非空，`NOT_APPLICABLE` 时二者均 NULL；stable_code 只允许本目录现有 `MCP.*`。`recent_audit` 恰为 `{last_invocation_at,last_outcome,last_incident_no}`，三者可全空表示当前范围无记录；outcome 仅 `SUCCEEDED|REJECTED|FAILED|TIMEOUT|CANCELLED|UNKNOWN_AFTER_CRASH`，incident 可在成功时为空。摘要绝不含 raw name/URI、对象 id、payload/header/secret、response body 或文件路径。
- `McpManifestVersionAdminViewV1={id,connector_id,version_no,protocol_version,manifest_digest,signer_subject,transport,status,approval_ref,approved_at,rejected_at,rejected_reason,activated_at,superseded_at,materialized,row_version,created_at}`。`materialized` 对 local 表示安装组完整，对 inbound/remote 固定 true（表示“不需本地物化且激活前置已满足”）；DRAFT/PENDING local 为 false，APPROVED 以后按实际安装组。DRAFT 的 approval/result/activation 时间全 NULL；PENDING_APPROVAL 的 approval_ref 非空而结论/activation 时间全 NULL；APPROVED/ACTIVE/SUPERSEDED 的 approval_ref/approved_at 非空、拒绝字段为空，ACTIVE 另 activated_at 非空，SUPERSEDED 另 activated_at/superseded_at 非空；REJECTED 的 approval_ref/rejected_at/rejected_reason 非空且 approved/activation 字段为空；REVOKED 只保留到达撤销前合法路径已经形成的字段，不能伪造新结论。不得返回 manifest JSON/signature/key ref、origin、credential ref、artifact/root/receipt/HCS/WFP/SID 或下载链接。
- enable request 恰为 `{row_version}`，disable request 恰为 `{row_version,reason}`，reason 为 1..500 Unicode scalar；两者要求 UUIDv7 `Idempotency-Key`。成功 data 恰为 `McpConnectorMutationResultV1={id,status,row_version}`，enable status 只能 ENABLED、disable 只能 DISABLED；幂等重放按基线返回首次同形结果，非同键并发按 `PLATFORM.CONCURRENCY.STALE_VERSION`。
- issue request 恰为 `{legal_entity_id,connector_id,manifest_version_id,tool_names,resource_uri_templates,ttl_seconds?,max_calls?}`；法人必须逐字等于 header/当前会话，connector 必须是 INBOUND+ENABLED，manifest 必须是该 connector 当前 ACTIVE。两数组各 0..256、逐 UTF-8 bytes 排序去重并逐项通过活动 manifest 的同一 name/template 编译器，二者不得同时为空。请求项不在活动 manifest 时返回 HTTP 403 `MCP.MANIFEST.CAPABILITY_DENIED`；请求项虽在 manifest、但按当前人类身份与范围不可见或无权时返回 HTTP 404 `MCP.TOOL.NOT_VISIBLE_OR_DENIED` 或 `MCP.RESOURCE.NOT_VISIBLE_OR_DENIED`，两类拒绝都零 grant 且不泄漏被拒对象。成功 data 恰为 `{grant_id,bearer_token_once,expires_at,max_calls}`；`bearer_token_once` 只出现这一次且服从下一段 50-byte grammar。request/response/通用缓存/日志/审计不得出现 `token_hash`。
- active grant list item 恰为 `McpHumanGrantActiveViewV1={id,user_id,connector_id,manifest_version_id,allowed_tool_names,allowed_resource_uri_templates,max_calls,used_calls,issued_at,expires_at,state:"ACTIVE",row_version}`，数组规范同 scope；不返回 token/hash、scope digest、session/device row、security tags 或 secret。普通人只能看到 user_id 等于自己的项。
- revoke request 恰为 `{row_version}` 且要求 UUIDv7 `Idempotency-Key`；成功 data 恰为 `{id,state:"REVOKED",revoked_at,row_version}`，revoked_at 非空。owner/admin 竞态同样使用 stale-version 409；撤销保留 grant 行与审计。

`ttl_seconds` 与 `max_calls` 是请求中唯二可省略字段：省略时分别取启动配置 `mcp.grant_ttl_seconds`、`mcp.max_calls_per_grant` 的当前值；两配置默认/硬上限分别为 600/100，只允许向下收紧。显式提交时必须分别落在 `1..=当前配置值`，0、负值、溢出或超过当前值统一返回 HTTP 400 `PLATFORM.REQUEST.INVALID_PAYLOAD` 的字段级错误，不静默钳制；因此任何签发 grant 的 TTL/次数都不可能越过当前运维上限。token 的唯一 wire grammar 是 50 个 ASCII bytes：固定前缀 `epmcp1.` 后紧跟对恰好 32 个 CSPRNG 随机 bytes 做 RFC 4648 §5 base64url-no-pad 得到的 43 字符，表层正则为 `\Aepmcp1\.[A-Za-z0-9_-]{43}\z`。解析时必须解码为恰好 32 bytes 并且无 padding 重编码后与输入逐 byte 相等；任何 `=`、空白、Unicode、非规范尾位或大小写改写都拒绝。`token_hash=SHA-256(完整 50-byte ASCII token)`，DPoP preimage 中的 `exact bearer token bytes` 也是这同一完整 byte 串；不允许只 hash 随机部分或解码后 bytes。token 只展示一次，数据库只保存该 SHA-256；grant 绑定 `legal_entity_id + user_id + source_session_id + source_device_id + manifest_version_id + scope_digest`。grant issue 是已认证写 API 中唯一的“一次性 secret 响应”例外：不得进入 `platform_msg.idempotency_keys.response_body`，携带 `Idempotency-Key` 以既有 `PLATFORM.REQUEST.INVALID_PAYLOAD` 拒绝，响应 `details` 仅给字段级原因 `Idempotency-Key is forbidden for this endpoint`，不新增同义稳定码；成功响应丢失时不能重放 token，客户端只能撤销可见的旧 grant（或等待最多当前配置 TTL）后重新签发。该取值与 complete-MFA 相同，优先保证 secret 不落通用响应缓存。签发仍在一个事务内原子插入唯一 token hash，重复点击最多产生多个短期 grant，不得返回旧 token；UI 在重签前提示并撤销同用户/connector 的旧 ACTIVE grant。签发还要求来源 `user_devices.public_key` 非空且是 DER SubjectPublicKeyInfo 的 ECDSA P-256/prime256v1 公钥，设备私钥留在 OS non-exportable keystore。登出、账号/设备停用、法人授权失效、manifest 失活或主动撤销立即使 grant 失效，不等待 TTL。

`scope_digest` 的唯一 preimage 是 `SHA-256(ASCII("EP-MCP-GRANT-SCOPE-V1\0") || JCS(scope))`。`scope` strict object 恰有 `schema_version=1,legal_entity_id,user_id,source_session_id,source_device_row_id,connector_id,manifest_version_id,manifest_digest,security_level,data_scope_tags,allowed_tool_names,allowed_resource_uri_templates`；UUID 全用小写连字符文本、digest 用 64 位 lowerhex、security_level 用 JSON integer，三个数组按 UTF-8 bytes 排序去重。`source_device_row_id` 是 `user_devices.id` UUID 外键；HTTP/DPoP 使用 join 得到的 `user_devices.device_id` text，两者不得混用。该摘要只证明签发范围快照，不替代每次调用对当前权限、记录范围、设备、会话与 manifest 的重检。

MCP 客户端调用 `POST /mcp` 时除 bearer 外必须带 `X-EP-MCP-Grant-Id`、`X-EP-MCP-Device-Id`、`X-EP-MCP-Proof-Counter`、`X-EP-MCP-Proof-Timestamp`、`X-EP-MCP-Proof`。Grant-Id 是小写连字符 UUID，Device-Id 必须逐字等于来源 `user_devices.device_id`，counter 是无前导零 u64 且首请求为 1、以后严格 `last_proof_counter+1`，timestamp 是 Unix seconds 且服务器验收时落在 ±60 秒，proof 是无 padding base64url 的 canonical low-S IEEE-P1363 ECDSA P-256 `r||s` 64 bytes。签名输入是以下 UTF-8/ASCII 行以 LF 连接且末尾无 LF 后的 SHA-256：

```text
EP-MCP-DPOP-V1
<grant_id>
<lowerhex SHA-256(exact bearer token bytes)>
<device_id>
<proof_counter>
<proof_timestamp>
2026-07-28
<method>
<decoded Mcp-Name，缺席时为空行>
<lowerhex SHA-256(exact HTTP body bytes)>
```

core 的固定顺序是：先完成 protocol/header/body/token identity、设备公钥/proof 语法与签名校验；一旦 identity 可可靠绑定法人/用户/来源设备，立即生成 invocation/event ids，并在取得任何 permit 前成功物理预留第 4.7 节 caller completion slot；随后才取得第 4.1 节 global→connector 两层 permit、保留 core-owned rate timestamp，最后执行 counter UPDATE。slot 失败时零 permit/rate/counter/ATTEMPT/dispatch，以 `MCP.AUDIT.UNAVAILABLE` 返回并只写无正文 Windows 安全日志；permit/rate 拒绝已有 slot，必须先写 REJECTED completion 再返回。上述全部步骤均在 counter UPDATE 前。随后以单条条件 UPDATE 要求 `last_proof_counter+1=counter`、grant ACTIVE/未过期/未超次数，并原子写 `last_proof_counter=counter,used_calls=used_calls+1,state=CASE WHEN used_calls+1=max_calls THEN 'CONSUMED' ELSE 'ACTIVE' END`；UPDATE 失败原子移除刚保留的 inbound rate timestamp，写 completion 并释放 permit。grant 的 ACTIVE/未过期/remaining 判断只在该条件 UPDATE 中做一次；UPDATE 成功后的当前事实重读不得因为本次刚把 grant 置为 CONSUMED 而拒绝本次。若重读 grant，只接受“仍 ACTIVE”或“由同一 UPDATE 产生的 `state=CONSUMED,used_calls=max_calls,last_proof_counter=accepted_counter`”两种形状；任何其他改变仍拒绝。因此 `max_calls=1,counter=1` 是必须能继续到 ATTEMPT/dispatch 的正例，其后任何第二次请求才被拒绝。通过 proof 且被该 UPDATE 受理的尝试即消耗一次并在所有后续 HTTP response 带 `X-EP-MCP-Proof-Counter-Accepted`；即使后续当前 session/device/account/法人授权、manifest、binding/字段权限、audit attempt、owner handler、外部 server 或响应 schema 失败也不退还。audit attempt 提交失败发生在 UPDATE 后，因此响应带 accepted header但零 dispatch。这是为了让计数与外部副作用不伪装成同一数据库事务。同一 grant 因而单飞，乱序、重复、并发第二请求或重放都以 `MCP.DEVICE_PROOF.INVALID` 拒绝且不执行 binding。客户端按 response header 而不是按 HTTP/JSON-RPC 成败推进本地 counter；网络结果不确定时该 grant 不允许猜测重试，必须撤销并签发新 grant。core 从 grant 与当前数据库事实重新构造 `SecurityContext`，固定 `client=ClientKind::Mcp`；请求体没有可提交的主体、角色、密级、范围或法人字段。`ClientKind::Mcp` 复用 grant 的来源设备行主键，但 proof/header 签的是 join 得到的外部 `user_devices.device_id` 规范文本，不是 UUID 主键；不新增或改写 `user_devices.client`，这是与 ClientKind/设备一一映射旧句的唯一具名例外。

入站每次调用都重新检查 session/device/account/法人授权、活动 manifest、能力域、对象/字段/记录权限、密级、职责分离、审批与 command registry。grant 的 ACTIVE/未过期/remaining 仅由上段原子 UPDATE 判定；后续如需重读只按上段的本次受理形状校验，不得让最后一次调用被自己写入的 CONSUMED 状态否掉。`tools/list`、`resources/list` 与 `resources/templates/list` 已按当次权限裁剪；调用者猜到被裁掉的 tool/resource 时分别返回不可区分的 `MCP.TOOL.NOT_VISIBLE_OR_DENIED` / `MCP.RESOURCE.NOT_VISIBLE_OR_DENIED`，不泄漏其是否存在。

### 4.4 出站远端 MCP

远端出站唯一传输是 `2026-07-28` 的**无状态 Streamable HTTP**，且只能由 `integration-gateway` 发出。`core-server`/`job-worker` 通过 `\\.\pipe\ep-integ` 新增唯一 operation `mcp.remote.exchange.v1`；`ep-ops` 仍只有 health/metrics。调用帧携带已批准 manifest 的 canonical bytes、签名、digest、一个允许的方法与有界 payload；gateway 不查数据库。

`integration-gateway` 每次独立完成 manifest 签名与 digest、origin 匹配、DNS/IP/代理/重定向拒绝、TLS 1.3/SPKI pin、Content-Type/SSE framing、terminal JSON-RPC transport shell、request id 与字节上限校验。它没有 schema registry，不验证业务 result schema/allowed fields；唯一业务 response schema/field validator 在有受信 registry 的 core-server/job-worker 发起进程，且固定 size-first：超过 8 MiB 先返回 `MCP.PAYLOAD.TOO_LARGE`，不超过才 strict parse 并可能返回 `MCP.RESPONSE.SCHEMA_INVALID`。gateway 保持 `0 DB / 0 KMS / 0 business-file / 0 Outbox`，不缓存业务响应，不持久化 MCP session，不提供通用 HTTP port。持久凭据只以 `WindowsCredentialRef` 存在 manifest；其唯一 canonical grammar、512-byte 引用界和 Win32 `TargetName` 逐字映射采用 `docs/config-reference.md` 第 5 节，不得定义第二个 parser。实际 1..2560-byte secret 只存 Windows Credential Manager，由 `NT SERVICE\ep-integ` 自身 current token 读取且只经上文本地维护状态机写入，不能出现在数据库、环境变量、命令行、配置包、日志或错误响应。

第三方 remote terminal error 只接受 top-level 恰为 `jsonrpc="2.0",id,error`，id 与请求逐字相等；`error` 恰含 integer `code`、1..1024 Unicode scalar 的 string `message` 与可选 `data`，整份仍受 8 MiB 上限。gateway 只判 transport shape 并原样在有界内存中交回；发起进程从不信任或透传第三方 `message/data/stable_code/request_id`，只把 `code` 与 exact error bytes SHA-256 留在脱敏 completion hash，然后统一生成本平台 `MCP.REMOTE.UNAVAILABLE` envelope。error 顶层/字段/类型/id 非法则为 `MCP.RESPONSE.SCHEMA_INVALID`，超字节界仍只为 `MCP.PAYLOAD.TOO_LARGE`。local child/guest 的合法 JSON-RPC error 同样不透传正文，统一映射 `MCP.LOCAL.CONTAINMENT_FAILED`；非法 error 映射 schema-invalid。raw error bytes 在映射后立即 zeroize，不入普通日志、审计正文、spool、tracing 或用户响应。

### 4.5 出站本地 MCP

本地 MCP 由 `plugin-host` 承载，`\\.\pipe\ep-plugin` 新增唯一 operation `mcp.local.exchange.v1`。允许两种 manifest 已签名形态：

1. `LOCAL_SIGNED_STDIO`：默认、低成本；从已验签包内以 `CreateProcessW` 启动固定 executable，MCP 只走继承的 stdin/stdout；
2. `LOCAL_WINDOWS_HYPERV_CONTAINER`：可选；只允许 `plugin-host` 经 Windows Host Compute Service API 启动 Windows Server Hyper-V-isolated、签名且 digest 固定的 Windows container image，不安装或调用 Docker/containerd socket；明确禁止 process isolation、Linux container、Kubernetes 或常驻 daemon API。宿主必须启用并通过 Hyper-V isolation probe；BC-2 还必须由 IaaS provider/VM SKU 证据证明 nested virtualization。任一条件不满足只使该 transport gate 不绿，默认 `LOCAL_SIGNED_STDIO` 仍可独立启用。

两者都是 `plugin-host` 的受控调用对象，不是新增产品服务。stdio 子进程置独立 Job Object；Hyper-V container 置独立 HCS compute system。两者都禁 child process、交互桌面、网络、任意 host 文件系统、注册表写和 named pipe 枚举，只读访问自身签名包/镜像，临时内存与匿名管道随调用销毁。`plugin-host`、stdio child 与 Hyper-V guest process 均禁止 WER 上传和 full/user dump；启动前必须 readback 系统与进程级 LocalDumps/WER policy，证明这些 executable 没有可捕获 payload/secret 的 dump 目的地，异常或不可判定即该 local transport 不启用。允许的故障证据只含 invocation id、build/package digest、exit/exception code 与时间，不含内存、handle、argv、stdin/stdout 或业务 bytes。container 无 network endpoint、无 host bind mount、base layers 只读；唯一可写层是每实例 HCS ephemeral scratch layer，路径固定 `C:\ProgramData\EnterprisePlatform\mcp\scratch\<invocation-id>\`，owner/DACL 只含 SYSTEM/Administrators/ep-ops 管理与 ep-plugin 本次调用所需访问，禁止继承/reparse/ADS，硬上限 536870912 bytes。HCS create/readback 必须证明 Hyper-V isolation、base layers、零 mount/endpoint、scratch path/limit 和 active process=1；调用终止后先销毁 compute system，再 reparse-safe 删除该 scratch 目录并证明零残留。persistent credential 仅适用于 REMOTE 与 LOCAL_SIGNED_STDIO；Hyper-V container 首版没有 secret 注入通道。

本地附件不能直接成为可执行根。两种 LOCAL 的 `artifact_hash`/`artifact_size_bytes` 必须等于附件版本事实，`package_digest=SHA-256(exact CAB bytes)`；由第 3.2 节同一离线安装通道二次读附件、验签并物化。两类都只允许单 CAB，exact bytes 为 1..2147483647，禁止 multi-cab/spanning；平台通用 5 GiB 附件上限在这里被更严格的 CAB hard cap 覆盖。LOCAL_SIGNED_STDIO CAB 的 exact entry 集合是 `mcp-artifact.jcs.json`、`mcp-artifact.p7s` 加 manifest `files[]`，而 Hyper-V-container CAB 恰为前三项中的两个 manifest 文件加唯一 `image-layout.tar`。`McpLocalArtifactManifestV1` strict 字段恰为 `schema_version=1,artifact_code,artifact_version,artifact_kind,target_os="windows",target_arch="x86_64",entrypoint_relative,container_image_digest,files`。`artifact_kind` wire 闭集恰为 `MCP_SIGNED_STDIO_PACKAGE|MCP_WINDOWS_HYPERV_CONTAINER`，分别且只能对应 connector transport `LOCAL_SIGNED_STDIO|LOCAL_WINDOWS_HYPERV_CONTAINER`，并必须逐字等于外层 `SignedArtifactKindV1`/安装收据的 kind；不得另用 Rust variant 名、简称或别名。stdio 必须有 entrypoint、container digest 为空，files 1..256 项且总未压缩不超过 2000000000 bytes；container entrypoint 为空、digest 非空，files 恰一项 `image-layout.tar` 且该 tar 为 1..2000000000 bytes。container 的 OCI 展开层总字节仍不得超过 5368709120，不另造 64 GiB artifact store。

`artifact_code` 必须匹配 `[a-z][a-z0-9._-]{0,63}`，`artifact_version` 必须匹配 `[A-Za-z0-9][A-Za-z0-9._+-]{0,63}`；两者只是被签名的包身份/运维显示字段，不参与 connector 路由、权限、版本选择或兼容判定，也不要求等于 connector code/version，因此同一已验签 CAB 可按本节复制规则由不同 connector/version 独立物化。`files[]` 每项恰为 `path,media_type,size_bytes,sha256` 并按 path UTF-8 bytes 排序。container 的唯一项必须逐字为 `{"path":"image-layout.tar","media_type":"application/vnd.oci.image.layout.v1.tar",...}`。stdio path 必须是 NFC、`/` 分隔、无空段/`.`/`..` 的规范相对路径，最后扩展名必须为小写，且 path-extension→media-type 闭集恰为：`.exe|.dll → application/vnd.microsoft.portable-executable`、`.json → application/json`、`.txt → text/plain; charset=utf-8`、`.bin|.dat → application/octet-stream`；未知扩展、大小写变体、同路径不同 media type、脚本扩展与 extensionless 文件全部拒绝，`.txt/.json` 还必须是合法 UTF-8。CAB entry 与 roster 逐项相等，entrypoint 必须是其中唯一 `.exe`，全部 PE/DLL 均须独立 Authenticode 验签且禁止脚本、安装钩子、符号链接/reparse point、ADS、device path 与包外依赖。内层 JCS/CMS 的 detached 签名、算法、证书链、吊销与 DEV/PROD 分界复用第 3.2 节；外层附件/配置签名不能替代内层验签。

stdio 原子根唯一为 `C:\ProgramData\EnterprisePlatform\mcp\packages\<lowercase-manifest-version-uuid>\<lowerhex-package-digest>\`；即使同一 CAB 被另一 connector、较高 manifest 版本或受控回退复用，也必须从已验签来源完整复制到新的 version root，禁止 hardlink、reflink/block clone 或共享可执行文件对象。WCOW 包根唯一为 `C:\ProgramData\EnterprisePlatform\mcp\wcow\<lowerhex-package-digest>\`。每个根都以固定名 `package.cab` 保存 exact input CAB，并提取 CAB 的 exact entry 集；root regular-file roster 恰为 `package.cab + archive entries`，无子目录外壳或额外文件，`SHA-256(package.cab)=package_digest`。安装器逐项二次读回证明 extracted bytes 与 archive entry 的 length/hash 相等；plugin-host 每次调用前独立复核 CAB digest/exact roster、inner manifest CMS、archive-entry↔extracted equality，再验 PE/OCI/HCS 事实，不能只信收据或提取文件。`ep-install://mcp-stdio/<manifest-version-uuid>/sha256/<digest>` 与 `ep-install://mcp-wcow/sha256/<digest>` 只能映射到对应根。两类根 owner 固定 `NT AUTHORITY\SYSTEM`，关闭 ACL 继承；显式 DACL 只有 `SYSTEM`、`BUILTIN\Administrators`、`NT SERVICE\ep-ops` 的完全控制，`NT SERVICE\ep-core` 的 `READ_CONTROL`，以及 `NT SERVICE\ep-plugin` 的只读/列举/执行，不授写入、删除、改权或改 owner；每个 stdio version root 另只对该 manifest 唯一预建的 exact AppContainer SID 授只读/执行，其他版本 SID 无 ACE。禁止 `Users`、`Authenticated Users`、`Everyone`、父目录继承、reparse point、ADS 与跨根 hardlink。收据中的 `installed_root_sd_sha256` 是发布后以 self-relative binary security descriptor 规范 bytes 算出的 SHA-256，core 登记时与 plugin-host 每次启动前都须 readback 相等；安装/GC 旧版本不会改变其他 version root 的 SD。

container 的 `image-layout.tar` 必须是 OCI Image Layout 1.0.0：根只含 `oci-layout,index.json,blobs/sha256/*`，`index.json.mediaType` 与其唯一 `windows/amd64` descriptor 的 media type 分别恰为 `application/vnd.oci.image.index.v1+json`、`application/vnd.oci.image.manifest.v1+json`，目标 manifest 自身 `mediaType` 也恰为后者；禁止 Docker media type、foreign URL、额外 platform、tag 漂移与 nondistributable 外链，其 manifest digest 必须等于 `container_image_digest`。该 OCI manifest 必须恰有一个 `application/vnd.oci.image.config.v1+json` config descriptor 与至少一个 layer descriptor；layer media type 闭集恰为 `application/vnd.oci.image.layer.v1.tar`、`application/vnd.oci.image.layer.v1.tar+gzip`、`application/vnd.oci.image.layer.nondistributable.v1.tar`、`application/vnd.oci.image.layer.nondistributable.v1.tar+gzip`，四类 blob 都必须在 layout 内，禁止 zstd、其他压缩/OCI 1.1 别名与所有未知 media type。全部 descriptor digest/size 逐项校验，uncompressed tar 与 gzip 解压后逐 entry 采用同一安全路径/链接规则，展开总量计入上述 5 GiB 界。config 必须是 strict OCI image config，`architecture="amd64",os="windows"`，`config.Entrypoint` 恰一项规范 Windows 容器内绝对 `.exe` 路径，`config.Cmd/Env/Volumes/ExposedPorts/Healthcheck/Labels/StopSignal/OnBuild/Shell` 全部 absent 或空，`WorkingDir="C:\\EP\\Plugin"`，`User="ContainerUser"`，unknown config 字段拒绝。HCS 只按该已签 config 创建恰一个受限 guest process，不接受 manifest/调用方覆盖 command、argv、env、user 或 working directory。安装器从最后一个产品 app layer 的 merged root 精确解析 Entrypoint 与其产品私有 DLL 闭包，逐文件做 production Authenticode chain/revocation/publisher 校验；Windows base layers 必须逐 digest 命中随发布包签名的 Microsoft Windows Server base-layer allowlist，不把 base OS 全量文件当产品文件重签。脚本、未签 PE/DLL、额外 app executable、外部 URL与未知 layer media type 全部拒绝。安装器先完成上述 config/layer/Authenticode 闭包，再通过 Windows HCS/WCOW layer import API 导入到产品专属只读 image store，返回不可变 identity `hcswcow://sha256/<64-lowerhex-container-image-digest>`；禁止 Docker/containerd daemon 或 socket。发布成功后写第 3.2 节收据；`plugin-host` 启动前重新核包内签名/文件摘要、config digest/Entrypoint、base allowlist 和 HCS identity/digest。

卸载要求引用闭包为空而不是只排除 ACTIVE/APPROVED：所有 `install_receipt_id` 或 `installed_root_ref` 指向该根的 manifest version 必须都处于 `REJECTED|SUPERSEDED|REVOKED`，并且没有进程、Job、HCS instance、文件句柄或 credential handle，且该根不在该 connector 最近两个已安装可回退版本内。stdio version root 不共享，因此只删除其自身六条 WFP filter、AppContainer profile/SID ACE 与 root。WCOW root 可单独按本闭包删除，但共享的 `hcs_image_identity/container_image_digest` 只有在扫描全部 connector/manifest 后确认所有引用版本都满足上述终态、最近两版保留与零 HCS instance/handle 条件时才可从 image store 删除；另一个不同 CAB/root 仍引用同 image identity 时绝不删 image。满足后只能由 ops-agent 的同一离线维护通道先标记 GC、重启后复核上述条件，再按“WFP filters→AppContainer profile/package ACE→无全局 image 引用时删 HCS image→root”顺序删除；最后必须证明零 orphan filter/profile/SID ACE/image/handle 才完成。数据库版本和收据永不删除。升级、回退均安装到新 version/digest 根；回退的新 DRAFT 不复制旧 receipt/root/sandbox identity，重新审批、物化并取得新收据，绝不覆盖旧根或把 SUPERSEDED 行重新激活。

LOCAL_SIGNED_STDIO 的“无网络/无任意文件系统”不把高权操作交给 `NT SERVICE\ep-plugin`。manifest APPROVED 后、写安装收据前，由 `NT SERVICE\ep-ops` 在受控 materialization 窗口预建 deterministic AppContainer profile；profile name 固定为 `EP.MCP.` 加 `lowerhex(SHA-256(legal_entity_id[16] || connector_id[16] || manifest_version_id[16] || package_digest[32]))` 的前 40 位，无 capability。WFP provider GUID 固定 `8f60bc4e-53c8-4f55-9dbc-30c32d89df09`，sublayer GUID 固定 `e535fa90-67b8-4bb5-a459-77a6a27f6afd`，filter namespace 固定 `71a80b6f-715b-5c31-98ce-c35f4e209922`。必须恰安装六条 persistent block filter，suffix、key 与 layer 一一为：`CONNECT_V4 → UUIDv5(namespace,profile_name||":CONNECT_V4") → FWPM_LAYER_ALE_AUTH_CONNECT_V4`、`CONNECT_V6 → ... → FWPM_LAYER_ALE_AUTH_CONNECT_V6`、`RECV_ACCEPT_V4 → ... → FWPM_LAYER_ALE_AUTH_RECV_ACCEPT_V4`、`RECV_ACCEPT_V6 → ... → FWPM_LAYER_ALE_AUTH_RECV_ACCEPT_V6`、`RESOURCE_ASSIGNMENT_V4 → ... → FWPM_LAYER_ALE_RESOURCE_ASSIGNMENT_V4`、`RESOURCE_ASSIGNMENT_V6 → ... → FWPM_LAYER_ALE_RESOURCE_ASSIGNMENT_V6`。六条 filter 都必须是同一固定 provider/sublayer、恰一个 `FWPM_CONDITION_ALE_PACKAGE_ID == exact AppContainer SID` 条件、`action.type=FWP_ACTION_BLOCK`、`weight.type=FWP_UINT64` 且值 `0xffffffffffffffff`、flags 恰为 `FWPM_FILTER_FLAG_PERSISTENT|FWPM_FILTER_FLAG_CLEAR_ACTION_RIGHT`；不得以单个抽象 inbound/outbound key 代替 IPv4/IPv6 与 connect/receive/bind 六个 layer。ops-agent 把 profile name、SID、provider/sublayer GUID、六个 filter key、root SD digest 一并绑定到安装收据；系统分配的 numeric filter id 只作运行期诊断，不持久化、不签名、不参与身份判定。已有 profile/filter 只有逐 key 执行 `FwpmFilterGetByKey0`，且 readback 的 provider/sublayer/layer、exact SID condition、block action、weight 与 flags 全部逐字相等时才可幂等复用，否则 fail closed。provider/sublayer ACL 只允许 ops-agent 管理，plugin-host 只可逐 key 查询和使用，不得创建、修改或删除 profile、DACL、provider、sublayer或 filter；Stage 14 必须覆盖 IPv4/IPv6 connect、listen/bind、receive canary、BFE stop/start 与系统重启后的六 key readback。

上述 WFP 对象全部由 ops-agent 通过 `FwpmEngineOpen0` 的非 dynamic session 创建；provider 的 flags 恰为 `FWPM_PROVIDER_FLAG_PERSISTENT`，sublayer 的 flags 恰为 `FWPM_SUBLAYER_FLAG_PERSISTENT`、weight 恰为 `0xf100`，六条 filter 必须在 provider 与 sublayer 两者 readback 成功后才创建。provider/sublayer/filter 的 security descriptor 采用同一个 exact 模板：self-relative descriptor，owner=`NT AUTHORITY\SYSTEM`、group=`BUILTIN\Administrators`，control flags 恰含 `SE_SELF_RELATIVE|SE_DACL_PRESENT|SE_DACL_PROTECTED` 且不含 `SE_DACL_AUTO_INHERITED`，SACL absent；DACL 无 inherited/deny ACE，显式 allow ACE 的 canonical 顺序恰为 SYSTEM=`FWPM_GENERIC_ALL`、BUILTIN\Administrators=`FWPM_GENERIC_ALL`、`NT SERVICE\ep-ops`=`FWPM_GENERIC_ALL`、`NT SERVICE\ep-plugin`=`FWPM_ACTRL_READ`，四项之外无 ACE。此处“管理权”唯一解释为 `FWPM_GENERIC_ALL`，不得换成通用文件 ACL、`GENERIC_ALL`、自选 ACTRL 并集或额外 `WRITE_DAC|WRITE_OWNER`；plugin-host 不因读权取得枚举全库、订阅、分类、增加、链接、修改或删除权。每次安装、启动、BFE 重启与 OS 重启后同时 readback engine session 结果、provider flags、sublayer flags/weight、四 ACE exact security descriptor 和六条 filter；任一对象缺失或漂移即 fail closed，不能仅凭 filter key 存在判绿。

每次调用时 `plugin-host` 必须先把收据中的 profile/SID/WFP filter keys/root-SD facts 与系统 readback 逐项比对，再按唯一 `MCP_STDIO_SANDBOX_PROFILE_V1` 构造 restricted lowbox token：`PROC_THREAD_ATTRIBUTE_SECURITY_CAPABILITIES` 只含 exact package SID 且 `CapabilityCount=0`，同时设置 `PROC_THREAD_ATTRIBUTE_ALL_APPLICATION_PACKAGES_POLICY=PROCESS_CREATION_ALL_APPLICATION_PACKAGES_OPT_OUT` 形成 LPAC；去管理员 SID、禁全部特权并固定低完整性。创建后在允许 child 处理业务字节前 readback 并证明 `TokenIsAppContainer=true`、package SID 相等、zero capabilities、All Application Packages opt-out/LPAC、restricted token、零 privilege 与低 IL 全部成立。`PROC_THREAD_ATTRIBUTE_HANDLE_LIST` 恰含 stdin 读端、stdout 写端、以 `CreateFileW(L"NUL",GENERIC_WRITE,FILE_SHARE_READ|FILE_SHARE_WRITE,OPEN_EXISTING)` 新建的 stderr 写端，以及 credential 存在时的 secret-pipe 读端；四者之外零 inherited handle。stderr 不得继承父进程句柄、不得合并 stdout、不得落文件或日志，NUL handle 仅对子进程可继承并在终态关闭；其他产品、业务、用户目录均无该 SID ACE。随后叠加 Job Object 的 `KILL_ON_JOB_CLOSE`、active-process-limit=1、内存/CPU/时限和禁止 breakaway，以及 DEP/CFG、禁动态代码与 Win32k system-call mitigation。任一 readback、restricted token、显式 handle list、DACL、WFP、Job 或 mitigation 不一致即零启动并返回 `MCP.LOCAL.CONTAINMENT_FAILED`。调用结束/取消/崩溃必须清除子进程、Job、handle 与临时对象；profile/filter 是签名版本级持久收容事实，只由上段 ops-agent GC 删除。Hyper-V container 必须满足上一段 HCS readback 与 scratch cleanup；实测不能证明任一值时该 transport 不可启用。Stage 14 对正常、超时、强杀、plugin-host/child/guest 崩溃与重启路径分别断言零 orphan child/Job/HCS compute system/handle/scratch，并在安装/卸载闭环断言零 orphan profile/filter/SID ACE/HCS image；另放置只授 `ALL APPLICATION PACKAGES`、不授 exact SID 的文件与注册表 canary，stdio child 必须读不到。崩溃 canary 在 stdin/secret pipe 注入标记后强制异常，必须证明 WER 网络上传关闭、系统/用户 dump 目录中零该进程 dump，Windows Event Log 与允许的最小故障证据中也没有标记或业务 bytes。

stdio wire 固定为 UTF-8、每行一个完整 JSON-RPC object、LF 结尾，禁止 CR、BOM、空行、超 8388608 bytes 行或 stdout 上的非 JSON 文字。普通完成以同 id terminal response 表达。仅当请求已经发到本地子进程且调用方断开或 deadline 到达时，`plugin-host` 才发送一行 exact transport control：`{"jsonrpc":"2.0","method":"notifications/cancelled","params":{"requestId":<原id>,"reason":"client_disconnect"}}` 或 reason=`deadline`；该 object 没有 `id`，不计入应用六方法、grant、调用次数或业务审计。发送后最多等 2 秒让子进程退出；未退出即关闭 stdin、终止 Job/HCS container 并记 forced-termination 指标。远端 HTTP 取消仍只关闭当前 response stream，绝不发送该 notification。

### 4.6 写操作与绝对禁区

MCP 写只能映射已有 command handler，不新建 repository、SQL 或旁路端口。每个 `ExistingCommand` binding 必须复用原 command 的 exact request schema、`Idempotency-Key`、乐观锁、审计、审批、SoD、事务与 Outbox 语义；MCP 层不能宣告成功后异步补做权限检查。

共享 `HighRiskOperation` 闭集恰为七值：合同生效、付款、开票、财务过账、期末结账、敏感数据导出六类业务高风险，加运维高风险 `DATA_MIGRATION`。七类在 MCP 中**无论权限多高都不得登记、列出或调用**。合同终止不进入该枚举，但同样是 MCP 绝对禁区；MCP 也不得承载任何审批通过/驳回动作，避免把职责分离变成机器自审。manifest 发布时扫描 action/command registry 闭包，运行时再次检查；命中任一七值、合同终止或审批结论动作均返回 `MCP.TOOL.HIGH_RISK_FORBIDDEN`。发布与运行测试必须断言枚举计数恰为七，防止仍加载旧六值快照。

普通写命令若按原业务规则需要审批，只能创建或继续其既有审批流程；MCP 得到 `approval_ref`/当前状态，不得伪造“已批准”。每次真正执行仍由 owner command 在当前事务重新检查审批快照和 SoD。

### 4.7 MCP 数据表

新增三张法人级表，全部 `ENABLE`、`FORCE RLS`，所有固定目标引用使用 `(legal_entity_id,id)` 真实复合外键与 `ON DELETE RESTRICT`。

**`platform_meta.mcp_connectors`**：`id, legal_entity_id, security_level, data_scope_tags, code, name, direction, transport, status, row_version` 与公共列。direction 为 `INBOUND|OUTBOUND`；transport 为四种 exact 枚举；shape 固定为 INBOUND 只能配 `INBOUND_HTTPS`，OUTBOUND 只能配其余三种。状态为 `REGISTERED|PENDING_APPROVAL|ENABLED|DISABLED|REVOKED`，合法边恰为 `REGISTERED→PENDING_APPROVAL→DISABLED`、`DISABLED→ENABLED`、`ENABLED→DISABLED`，任一非 REVOKED 状态可到 `REVOKED`，REVOKED 终态；不存在 REGISTERED/PENDING 直接 ENABLED、ENABLED 自环或 REVOKED 恢复。DISABLED→ENABLED 必须在同一事务、同 connector advisory exclusive lock 内重读 row_version，并证明恰有一份 compatible ACTIVE manifest、该 manifest 签名/吊销/本地物化或远端 origin/credential probe 全部有效、许可证与对应 MCP gate 均绿；任一事实缺失保持 DISABLED。DISABLED/REVOKED 保留全部版本、grant 与审计，REVOKED 同时令全部 grant 失效。`UNIQUE(legal_entity_id,code)`。

**`platform_meta.mcp_manifest_versions`**：`id, legal_entity_id, connector_id, version_no, protocol_version, manifest_json, manifest_digest, signature, signature_key_ref, signature_key_version, signer_subject, remote_scheme, remote_host, remote_port, remote_path, credential_ref, artifact_legal_entity_id, artifact_attachment_version_id, artifact_hash, artifact_size_bytes, artifact_eligible, install_receipt_id, installed_root_ref, installed_at, hcs_image_identity, installed_root_sd_sha256, sandbox_profile_name, sandbox_sid, wfp_provider_guid, wfp_sublayer_guid, wfp_connect_v4_filter_key, wfp_connect_v6_filter_key, wfp_recv_accept_v4_filter_key, wfp_recv_accept_v6_filter_key, wfp_resource_assignment_v4_filter_key, wfp_resource_assignment_v6_filter_key, status, approval_ref, approved_by, approved_at, rejected_by, rejected_at, rejected_reason, activated_at, superseded_at, active_slot` 与公共列。status 为 `DRAFT|PENDING_APPROVAL|APPROVED|ACTIVE|SUPERSEDED|REJECTED|REVOKED`，状态边只有 `DRAFT→PENDING_APPROVAL`、`PENDING_APPROVAL→APPROVED|REJECTED`、`APPROVED→ACTIVE`、`ACTIVE→SUPERSEDED`，任一非终态可到 REVOKED；REJECTED/SUPERSEDED/REVOKED 均终态，修改内容只能登记更高 `version_no`。`UNIQUE(legal_entity_id,connector_id,version_no)`；`active_slot smallint generated always as (case when status='ACTIVE' then 1 else null end) stored`，以 `UNIQUE(legal_entity_id,connector_id,active_slot)` 保证一份活动版本。local 两种 transport 从 DRAFT 起必须有附件版本/哈希/大小/eligible=true 并以真实复合 FK 固定到 `platform_file.attachment_versions`，但 DRAFT/PENDING_APPROVAL 的 `install_receipt_id/installed_root_ref/installed_at/hcs_image_identity/installed_root_sd_sha256/sandbox_*` 全空；APPROVED 可暂时保持整组 materialization 列全空等待安装，只有 APPROVED 行允许一次带 `row_version` 的 guarded CAS 原子补全：两种 local 都写 receipt/root/time/root-SD；Hyper-V container 同次写非空 hcs identity 且十个 sandbox 字段全空，stdio 保持 hcs identity 为空且十个 sandbox 字段全有。该 CAS 不得改 manifest/artifact/signature/审批字段，不得清空或二次改写；APPROVED→ACTIVE 前安装字段必须完整，ACTIVE/SUPERSEDED 保留不可变。REVOKED 可保留撤销前已有的完整收据或全空，二者不得互换。回退创建更高 DRAFT 时不复制旧 receipt/root/hcs/sandbox facts，须重新审批和物化。remote 必须有 HTTPS origin、SPKI pins 与可选 `wincred://` ref；inbound 两组字段都为空。manifest/digest/signature/signature-key/identity 全部不可变，安装收据仅有上述一次 CAS 例外，升级新增版本。

**`platform_authz.mcp_human_grants`**：`id, legal_entity_id, security_level, data_scope_tags, connector_id, manifest_version_id, user_id, source_session_id, source_device_id, token_hash, scope_digest, allowed_tool_names, allowed_resource_uri_templates, max_calls, used_calls, last_proof_counter, issued_at, expires_at, state, revoked_at, row_version` 与公共列。state 为 `ACTIVE|CONSUMED|REVOKED|EXPIRED`，合法边恰为 `ACTIVE→CONSUMED|REVOKED|EXPIRED`，后三者均终态、不得互转或恢复；expiry scanner 只允许把到期且仍 ACTIVE 的行 CAS 为 EXPIRED，主动撤销只允许 ACTIVE→REVOKED，最后一次计数受理只允许在同一条件 UPDATE 中 ACTIVE→CONSUMED。`used_calls between 0 and max_calls`，`last_proof_counter=used_calls` 且初值均为 0；每次受理 UPDATE 同时递增两者，并在新值等于 max_calls 时同语句改 CONSUMED，下游失败不回退计数；token hash 唯一。用户、会话、设备、connector、manifest 全用真实外键/长复合 FK 锁定同法人祖先。

MCP 调用不另建调用日志表。`MCP_AUDIT_NAMESPACE` 固定为 UUID `3f9b8e44-78a5-5ff0-8fc9-6ad25a8a5c55`；`invocation_id` 是在 transport 语法通过且 identity 已解析为法人/用户/来源设备后由有数据库的调用发起进程生成的 UUIDv7：入站和 core 发起的出站唯一 owner 是 core-server，Outbox/job 发起的出站唯一 owner 是 job-worker；integration-gateway、plugin-host 与 local child/guest 永不生成审计 identity、写 DB 或写 spool，只把 terminal transport facts 返回发起进程。此后在任何 binding/外部副作用前拒绝的请求恰向既有 `platform_audit.audit_events` 写一条 `MCP_CALL_COMPLETION`；已经获准 dispatch 的请求恰写两条：`MCP_CALL_ATTEMPT` 必须先独立提交，再调用 owner handler/远端/本地进程，终态后写 `MCP_CALL_COMPLETION`。`object_type` 固定 `platform.mcp.invocation`，`object_id=invocation_id`；`action` 只允许上述两值。编译期唯一登记落点是 `crates/platform/audit/src/object_registry.rs`：该对象必须登记为 object-level、id required、action set 恰为 `MCP_CALL_ATTEMPT|MCP_CALL_COMPLETION`；与第 3.8 节 AI 两 action 一起由 registry consistency test 逐项比较，禁止按字符串临时放行或另建数据库 registry。两个 event id 分别为 `UUIDv5(MCP_AUDIT_NAMESPACE, lowerhex(invocation_id bytes) || ":ATTEMPT")` 与同式 `":COMPLETION"`，是 `audit_events.event_id` 使用 UUIDv5 而非默认 UUIDv7 的唯一具名例外。`after` 中保存 `invocation_id`，故 reconciler 以 object id/phase 配对，不需要逆推 UUIDv5。

两类 `after` 都是 `schema_version=1` 的 strict masked object。公共字段恰为 `phase,invocation_id,direction,transport,connector_id,manifest_version_id,method,binding_digest,decoded_name_sha256,request_schema_code,request_schema_version,request_payload_sha256,input_field_codes,request_bytes,request_id,trace_id`；ATTEMPT 到此结束。COMPLETION 另恰有 `response_schema_code,response_schema_version,response_payload_sha256,output_field_codes,outcome,stable_code,duration_ms,response_bytes`。`before` 固定为空，两个字段 code 数组各为 0..1024 项且按 UTF-8 bytes 排序去重。

公共 request/binding 空值真值表唯一如下，防止 pre-binding rejection 无法构造 strict completion，也防止以空值差异泄漏“对象不存在”还是“存在但无权”：

| 阶段/方法 | `decoded_name_sha256` | `binding_digest` | `request_schema_code/version` | `input_field_codes` |
|---|---|---|---|---|
| 六方法在 identity 后但 name/URI 尚未成功规范解码 | NULL | NULL | NULL | `[]` |
| `server/discover|tools/list|resources/list|resources/templates/list` 通过 transport schema | NULL | NULL | NULL | `[]` |
| `tools/call|resources/read` 已规范解码、binding 尚未解析/不可见 | exact decoded name/URI SHA-256 lowerhex | NULL | NULL | `[]` |
| `tools/call` binding 已解析 | 同上 | canonical binding JCS SHA-256 lowerhex | binding 的 request schema code/正整数 version | 实际允许输入字段 code 集；无字段为 `[]` |
| `resources/read` binding 已解析 | 同上 | canonical binding JCS SHA-256 lowerhex | NULL | `[]` |

identity 后的 completion 其 `invocation_id,direction,transport,connector_id,manifest_version_id,method,request_payload_sha256,request_bytes,request_id,trace_id` 始终非空；其中 request hash/bytes 是已受理 exact HTTP/IPC request payload，不是业务对象。ATTEMPT 只允许上述四个无具名 binding 方法已经通过 transport schema，或 `tools/call|resources/read` 已落在“binding 已解析”行；因此 ATTEMPT 永远不存在半解析形状。对同一规范 decoded name/URI，“不存在”和“存在但不可见”必须使用完全相同的 pre-binding completion shape、stable code/HTTP 外形与计时抗枚举门禁，不能借 binding/schema 字段泄漏存在性。

COMPLETION 的 `outcome` 只允许 `SUCCEEDED|REJECTED|FAILED|TIMEOUT|CANCELLED|UNKNOWN_AFTER_CRASH`，其 phase 真值表固定为：无 ATTEMPT 只能 `REJECTED`；有 ATTEMPT 才能是其余五值。受信且 schema/字段合法的成功为 `SUCCEEDED`；有 ATTEMPT 的业务/transport/terminal-schema 非成功为 `FAILED`；30 秒绝对时限为 `TIMEOUT`；调用方断开且终止已确认、无结果可返回为 `CANCELLED`；崩溃恢复无法判定外部副作用时为 `UNKNOWN_AFTER_CRASH`。`stable_code` 只在 `SUCCEEDED|CANCELLED` 为 NULL；`REJECTED|FAILED|TIMEOUT|UNKNOWN_AFTER_CRASH` 必须是本次实际对外/恢复使用的现有平台稳定码，UNKNOWN 固定 `MCP.AUDIT.UNAVAILABLE`。`response_schema_code/version` 只对已解析 `tools/call|resources/read` binding 取该 binding 的 response schema code/正整数 version，四个无具名 binding 方法及 pre-binding completion 为 NULL；收到任一 exact owner/remote/local terminal bytes（即使其后 strict JSON/schema 校验失败）时 `response_payload_sha256` 为该 bytes 的 SHA-256 lowerhex、`response_bytes` 为实际 1..8388608，未取得 terminal bytes 时分别为 NULL/0；只有 terminal 通过 schema 与 allowed-output-field 验证时 `output_field_codes` 才为实际允许输出字段 code 集，否则固定 `[]`。平台自身 rejection/error envelope 不冒充 connector terminal bytes。审计不得保存 raw tool name、expanded resource URI、对象 id、secret、header value、extension value 或业务 payload 明文；这使第 4.1 节 `Mcp-Name`/URI canary 的零落盘规则没有审计例外。

attempt 审计提交失败则零 dispatch。F-55 不复用 writer 的 `spool.dir/spool.max_bytes`，而由 core-server 与 job-worker 各自使用同一实现的专用 `McpAuditCompletionSpoolV1`；固定目录分别为 `C:\ProgramData\EnterprisePlatform\spool\mcp-audit-completion\core\` 与 `...\worker\`，每目录硬上限 1073741824 bytes、1024 个固定 1048576-byte slot，不设运行配置。目录 owner SYSTEM、关闭继承；SYSTEM/Administrators 具完全控制，对应 `NT SERVICE\ep-core` 或 `NT SERVICE\ep-worker` 仅具本目录 create/read/write/delete/synchronize，另一个服务与所有其他主体无 ACE；拒绝 reparse、ADS、hardlink、8.3/大小写碰撞和越根路径。启动时 owner/DACL/容量与磁盘 free-space readback 任一不成立，MCP dispatch gate 不绿。

identity 解析并生成 invocation/event id 后、任何进一步授权/binding 拒绝或 ATTEMPT 前，发起进程先以 completion event UUID 为文件名 `CREATE_NEW` 一个 `.reserve`，真实写零并 flush 恰 1048576 bytes，使单条 pre-dispatch rejection 与 dispatched terminal 两路都已在任何响应/外部副作用前物理预留证据空间；无 slot、目录/磁盘超限或 flush 失败则不创建 invocation audit、零 rate/counter、零 dispatch。唯一 `McpAuditCompletionSpoolRecordV1` strict 字段恰为 `schema_version=1,event_id,invocation_id,legal_entity_id,actor_user_id,actor_device_row_id,attempt_present,occurred_at,after,record_digest`；`attempt_present=false` 只用于未成功提交 ATTEMPT 的拒绝，true 只用于已提交 ATTEMPT 后的 terminal；`record_digest=SHA-256(JCS(其余字段))`。整个 slot 文件恒为 1048576 bytes，格式恰为 33-byte ASCII `EP-MCP-AUDIT-COMPLETION-SPOOL-V1\n`、4-byte big-endian JCS length、JCS bytes、32-byte digest（逐字等于 record_digest 原始 bytes）、其余全零 padding，因此 `JCS length` 硬上限精确为 1048507。字段码数组各最多 1024 项且整个 completion 在 dispatch 前即可证明能放入该界。每个 completion 都先把同一已分配 `.reserve` write-through 改名 `.tmp`，在该文件内覆盖完整 slot 并 flush，再 write-through 原子改名 `.ready`；不得另建第二个未预留文件。只有 `.ready` 已持久化后才尝试幂等追加 DB completion，commit 后删除 ready 并 flush 目录。任何 write/rename 失败都立即关闭该进程全部新 MCP dispatch、写不含正文的 Windows Event Log，并持续保留内存终态重试同一 slot，不得删除、覆盖、向客户端声称审计成功或猜测业务成功。

启动与每 30 秒 replay 按 `(occurred_at,event_id)` 处理 `.ready`：逐项校验 grammar/digest/identity；`attempt_present=true` 必须存在逐字匹配 ATTEMPT，false 必须不存在 ATTEMPT；随后以确定性 event id 幂等追加 completion，数据库 commit 后才删除。`.tmp` 若已是完整合法 slot，按同一规则 write-through 改名 ready 后 replay；若截断/损坏，则原子改名 `.corrupt` 保留取证，有匹配 ATTEMPT 且超过 30 秒时另以同 event id 补 outcome=`UNKNOWN_AFTER_CRASH`，无 ATTEMPT 时证明零 dispatch 并写脱敏安全日志，不得静默删除。`.reserve` 若无 ATTEMPT 证明尚未 dispatch，可删除并写脱敏恢复日志；有 ATTEMPT 且已超 30 秒、无 ready/completion 时由 reconciler 补 outcome=`UNKNOWN_AFTER_CRASH` 后删除 reserve。ready 同 id 内容不同、孤立、损坏或 identity/attempt_present 不符均改名 corrupt、fail closed 并保持原件；`.corrupt` 不自动重放或删除，需 ops 受控导出取证后清理，期间占用 slot。任何 crash recovery 都绝不猜测外部副作用成功或重试调用。入站 transport/JSON/header/token 失败若尚不能可靠解析出 grant 对应法人、用户和来源设备，不得伪造 `audit_events` 的非空 FK；它恰写一条既有结构化部署安全日志事件 `MCP_TRANSPORT_REJECTION`，字段只含 `schema_version=1,server_request_id,stable_code,http_status,occurred_at`，所有 header/payload/name/URI/token/device 值均不写。该安全日志仍按基线 JSONL、DACL、轮转和外部写出规则保护；一旦 grant identity 已解析，后续拒绝全部走上述法人审计 completion，不能双写或漏写。owner command 原有领域审计继续保留，不与 MCP transport 审计互相替代；remote/local inbound/outbound 均复用同一状态机，transport cancel control 不新增审计行，但其原 invocation 必得到 `CANCELLED` 或 `TIMEOUT` completion。

上一段“先写 `.tmp`”的唯一实现含义是：先把同一已分配 `.reserve` write-through 改名 `.tmp`，在该文件内覆盖并 flush，再 write-through 原子改名 `.ready`；不得另建第二个未预留文件或释放其已占磁盘空间。

首版 outbound MCP 不允许无来源人类身份的自主定时器或纯系统 job。Outbox/job 只有在其不可变信封携带原始 `legal_entity_id + initiating_user_id + initiating_device_row_id + source_session_id + source_request_id`，且发起时已获准 MCP、执行时这些用户/设备/法人事实仍有效时才可调用；job-worker 以该非空用户/设备写 audit FK，`client=mcp`，不得伪造服务用户。无该来源、来源已失效或仅有 `client=system` 的任务必须零 rate reservation、零 attempt、零 dispatch；未来若要系统 actor，须先另立版本修改 audit 持久化 shape。

## 5. ServerAdmin 独立静态 SPA

### 5.1 形态与身份

新增 `clients/server-admin`。生产构建产物嵌入 `core-server` 并由现有员工 HTTPS 在 `/server-admin/` 提供；运行期没有 Node.js、独立 Web server、热更新器、额外端口或可写静态目录。SPA 复用 `clients/ui` 组件、既有登录/MFA/session/device/CSRF/CSP 与 `GET /api/v1/platform/client-bootstrap`，但路由树和制品独立于 Win/Mac/iOS/Android。

`ep-foundation::ClientKind` 唯一扩展为：

```rust
pub enum ClientKind {
    Win, Mac, Ios, Android, Portal, Ops, ServerAdmin, Mcp,
}
```

序列化值固定为 `win|mac|ios|android|portal|ops|server_admin|mcp`。普通员工 API 的 `X-Client` 新增 `server_admin`；`mcp` 只能由 `/mcp` grant middleware 固定，外部自填 `X-Client: mcp` 无效。`platform_audit.audit_events.client` 的 CHECK 终态为上述八值加 `system` 共九值。现有指标 `client` label 同步扩为八个人类/协议 client 值，`system` 仍不进入该 label。

ServerAdmin 不是超级管理员，不创设 `ServerAdminRole`，不绕过 RLS/字段权限/SoD/审批，也不允许直接 DB、KMS、文件、shell 或服务账户操作。系统、数据、安全、审计、密钥管理员继续两两职责分离。AI 是否显示和可调用只由 `reporting.ai_analysis.compose/execute`、当前权限及许可证决定，不由“管理员端”身份自动取得。

### 5.2 18×5 能力矩阵第五列

`server_admin` 列按下表冻结。“建议值”自 F-55 起就是首版规范值，不允许开发者再选：

| 序 | 能力域 | ServerAdmin |
|---:|---|---|
| 1 | `crm.customer_360` | `NOT_APPLICABLE` |
| 2 | `clm.contract_esign` | `NOT_APPLICABLE` |
| 3 | `sales.order_fulfillment` | `NOT_APPLICABLE` |
| 4 | `procure.supplier_collab` | `NOT_APPLICABLE` |
| 5 | `inventory.ledger_scan` | `NOT_APPLICABLE` |
| 6 | `service.workorder_equipment` | `NOT_APPLICABLE` |
| 7 | `platform.approval_notify` | `VIEW_ONLY` |
| 8 | `project.task_milestone` | `NOT_APPLICABLE` |
| 9 | `mdm.master_data` | `NOT_APPLICABLE` |
| 10 | `platform.full_text_search` | `VIEW_ONLY` |
| 11 | `ledger.posting_close` | `NOT_APPLICABLE` |
| 12 | `finance.settlement_view` | `NOT_APPLICABLE` |
| 13 | `invoice.apply_issue` | `NOT_APPLICABLE` |
| 14 | `reporting.report_print` | `VIEW_ONLY` |
| 15 | `platform.document_attachment` | `NOT_APPLICABLE` |
| 16 | `platform.admin_lowcode_ops` | `FULL` |
| 17 | `platform.extension_dynamic_code` | `FULL` |
| 18 | `portal.supplier_web` | `NOT_APPLICABLE` |

物理表仍是一行一个 `(capability_domain,client)`，因此迁移新增 18 行并把总数从 72 变成 90，不是真的新增 SQL 列。二进制冻结快照同批扩成 90 格并重算 hash。`VIEW_ONLY` 的 `alternative_path` 固定为 `desktop://same-object/write`；第 1–6、8–9、11–13、15 行的 N/A 固定 `desktop://capability-domain`，第 18 行固定 `portal://supplier-web`。`Mcp` 不进入能力等价矩阵，不产生第六列；它按 manifest binding 与逐次权限检查判定。

### 5.3 ServerAdmin 可见功能

ServerAdmin 只提供：系统配置与签名发布、用户/角色/设备/证书的受权管理、模块许可证和签名包安装/启停/升级、扩展与 MCP manifest 审批状态查看及启停、AI 模型包状态与认证报告查看、审计/健康/备份/恢复/迁移证据查看、只读审批待办、只读检索与报表。所有 approve/reject 结论仍只由 Win/Mac 既有审批待办作出，ServerAdmin 不提供审批决定入口。禁用模块时元数据与业务数据继续安全保留；恢复只能重新启用同一签名版本或安装新签名版本。

AI 模型管理面只有两条只读 API，均要求 `platform.admin.ai_model.view` 与 `PlatformAdminLowcodeOps + Read`：`GET /api/v1/platform/ai-model-packages` 只接受可选 `cursor` 与 `page_size`（默认 50、最大 100），禁止 `limit/page/sort/filter`，按 `created_at DESC,id DESC` 键集分页；`GET /api/v1/platform/ai-model-packages/{id}` 返回单项，不存在与无权统一 404。cursor 唯一 grammar 是 `epcur1.<base64url-no-pad(JCS(AiModelPackageCursorV1))>`，payload strict 字段恰为 `schema_version=1,endpoint="AI_MODEL_PACKAGES",created_at,id`；created_at 是 UTC 微秒精度 `YYYY-MM-DDTHH:MM:SS.ffffffZ`，id 是小写 UUID。解码后最大 512 bytes，重做 JCS/base64url 必须与输入逐 byte 相等；它仅是位置不是授权，服务端仍逐次做完整权限与证据检查。查询条件严格为 `(created_at,id) < (cursor.created_at,cursor.id)` 的倒序后续；响应封套 `data` 恰为 `{items:[AiModelPackageAdminViewV1...]}`，`meta` 恰为 `{page_size,next_cursor}`。只有确实还有下一页时 `next_cursor` 为本页末项编码；空页或末页固定 NULL，items 不为 NULL。

`AiModelPackageAdminViewV1` 恰含 `id,model_code,model_version,runtime_abi_version,package_digest,manifest_digest,signer_subject,signature_kind,prompt_template_version,max_context_tokens,max_concurrent_requests,execution_profile,resource_formula_version,certification_report_ref,certification_report_digest,verified_at,certified_at,activated_at,disabled_at,revoked_at,status,row_version`；status 只取 `REGISTERED|VERIFIED|CERTIFIED|ACTIVE|DISABLED|REVOKED`，signature kind 只取 `PROD_AUTHENTICODE|DEV_ECDSA_P256`，execution profile 只取 `CPU_LOCAL|GPU_LOCAL`，row_version 为 PostgreSQL bigint/Rust i64 可表示的非负 JSON integer（0..9223372036854775807），不使用 u64 分支。两个 certification 字段必须同时 NULL 或同时非 NULL；`verified_at,certified_at,activated_at,disabled_at,revoked_at` 五个时间都是 UTC RFC3339 且可 NULL，其他字段非 NULL，摘要均用小写 64 位十六进制。不得返回 `installed_root_ref`、`install_receipt_id`、提示词正文、tokenizer/权重字节、文件路径、签名正文、secret 或下载链接。证据报告缺字段/验签失败时整项以 `AI.MODEL_PACKAGE.SIGNATURE_INVALID` 拒绝，不显示半可信值；首版不注册模型包 upload/install/activate/disable/revoke/download/action API，这些变更只走签名离线发布与既有受控状态机。

F-55 的五个权限项与范围锚不是示例，必须由第 7 节 `V20261024090300` 按下表幂等 seed；字段逐字匹配 `permission_items`，不得用近义码替代：

| code | module_code | function_point | allowed_actions | object_type |
|---|---|---|---|---|
| `reporting.ai_analysis.compose` | `reporting` | `本地分析 AI 方案生成` | `[VIEW]` | `reporting.ai_analysis.compose` |
| `reporting.ai_analysis.execute` | `reporting` | `本地分析 AI 查询执行` | `[VIEW]` | `reporting.ai_analysis.execute` |
| `platform.mcp.connector.manage` | `platform` | `MCP 连接器管理` | `[VIEW,UPDATE]` | `platform.mcp.connector.manage` |
| `platform.mcp.grant.issue` | `platform` | `MCP 人类授权签发` | `[CREATE]` | `platform.mcp.grant.issue` |
| `platform.admin.ai_model.view` | `platform` | `AI 模型包查看` | `[VIEW]` | `platform.admin.ai_model.view` |

五个 permission id 固定为 `00000000-0000-7000-8000-000000000310` 至 `...0314`，按表顺序一一对应；五个 `object_scope_bindings` id 固定为 `...0504` 至 `...0508`。binding exact 为：两个 AI object type 均指 `(reporting,datasets,NULL,NULL,NULL,NULL,min_security_level)`；connector manage 指 `(platform_meta,mcp_connectors,NULL,NULL,NULL,NULL,security_level)`；grant issue 也指同一 connectors 表，授权判定的 object id 明确取请求 `connector_id`，不能取尚未创建的 grant id；AI model view 指 `(platform_ops,ai_model_packages,NULL,NULL,NULL,NULL,security_level)`。object_type 必须保持五项互异，否则现有 `(object_type,action)` 授权快照会把 compose/execute 或管理功能错误合并。若同 id/code/object_type 已有行但任何字段不一致，迁移失败而不是覆盖；`ON CONFLICT DO NOTHING` 后必须逐字段断言。迁移不自动 seed `role_permission_grants`，全部角色授权只由签名 authz 配置显式授予；默认角色因而不会自动得到 AI 或 MCP 权限。

AI compose 的授权顺序也固定：进入模型前先完成法人、许可证、能力矩阵及 role+permission 粗门；模型提出 dataset 后，以该 dataset id 经上述 AI operation binding 完成对象范围、密级、字段和记录谓词判定，再签 plan token。execute 以 token 中 dataset id 和当前事实重做全部判定。模型包行固定 security level 40 只用于 ServerAdmin 查看模型包，绝不能被误用为普通分析用户的数据密级。

它不承载客户、合同、订单、采购、库存、项目、主数据、财务、发票、附件或门户业务写入。ViewOnly 只允许查看，审批通过/驳回仍在 Win/Mac 等既有业务端完成。

## 6. 客户自控物理机与境内 IaaS VM

### 6.1 两个等价 carrier

`DeploymentCarrier` 只有：

```rust
pub enum DeploymentCarrier {
    CustomerControlledPhysical,
    CustomerControlledDomesticIaasVm,
}
```

两者均是客户控制的单台 Windows Server 2022：同机 PostgreSQL 16、同机本地附件存储、同机内置 KMS 或客户 HSM、同一组产品服务、命名管道和资源单位。IaaS 只替换硬件承载体，不替换任何产品组件。

明确不允许：厂商 SaaS、多客户共享应用/数据库、Kubernetes、容器化整个平台、HA/读副本集群、云托管 PostgreSQL、云 KMS、云托管消息队列、云函数、厂商遥测/回传与自动在线更新。IaaS provider 的磁盘、快照与网络只是客户基础设施；平台仍按本地软件交付，厂商不取得租户 root、密钥或数据访问权。

### 6.2 同门禁与部署事实

物理机与 VM 使用完全相同的附录 A、Stage 14、RPO/RTO、离站备份、恢复、密钥恢复、病毒扫描、资源、勒索恢复与发布门禁。不得因为“云盘有快照”“provider 有 SLA”跳过任一平台测试，也不得把 provider 快照当作唯一备份。

允许的 provider/region/SKU 不是源代码里的自由字符串。本节所有 policy/evidence/probe JCS 都是无 BOM、合法 UTF-8 的 RFC 8785 exact bytes，每份最大 1048576 bytes，unknown field/duplicate key/非规范 number 均拒绝；时间恰为 UTC 秒精度 `YYYY-MM-DDTHH:MM:SSZ`。`policy_code/provider_code/region_code/site_code/vm_sku/backup_failure_domain_code` 均为 1..64 ASCII bytes 且匹配 `[A-Z0-9][A-Z0-9._-]{0,63}`，jurisdiction 代码为 2..16 ASCII bytes 且匹配 `[A-Z0-9][A-Z0-9-]{1,15}`，policy version 为 1..64 ASCII bytes 且匹配 `[0-9A-Za-z][0-9A-Za-z.+-]{0,63}`，subject/key ref 为 1..256 Unicode 标量，opaque evidence ref 为 1..512 ASCII bytes。除下文明确更小/固定的数组外，数组为 1..256 项并按文中键的 UTF-8 bytes 排序去重。

每次部署先由生产发布包携带一份 Authenticode 覆盖且 detached CMS 签名的 `CarrierPolicyV1` JCS；strict 字段恰为 `schema_version=1,policy_code,policy_version,residency_jurisdiction_code,allowed_physical_sites[],allowed_iaas_regions[],required_backup_separation[],managed_components`。physical site 项恰为 `site_code,jurisdiction_code`，按 `(site_code,jurisdiction_code)` 排序；IaaS 项恰为 `provider_code,region_code,jurisdiction_code,min_tpm_version="2.0",vtpm_attestation_required=true,vtpm_attestation_profile="TPM2_QUOTE_SHA256_V1",vtpm_ak_trust_anchor_spki_sha256[],approved_vm_skus[]`，按 `(provider_code,region_code)` 排序。trust-anchor 数组恰 1..8 个 64-lowerhex SHA-256，按 bytes 排序去重，每项是该 provider/region 离线批准 vTPM AK 根证书的 DER SPKI digest；证书本文由 attestation child 携带并不从网络下载。每个 SKU 项 strict 字段恰为 `vm_sku,nested_virtualization_supported`，按 `vm_sku` bytes 排序去重且至少一项；只有 flag=true 的批准 SKU 才可启用 F-55 Hyper-V local transport，flag=false 的低成本 SKU 仍可运行其余平台能力与默认 stdio transport。required separation 排序后恰为 `SITE_OR_REGION,ACCOUNT_OR_CREDENTIAL_DOMAIN,MEDIA_OR_IMMUTABILITY_DOMAIN`；managed_components 恰为 `database=false,kms=false,message_queue=false,function=false,application_runtime=false,telemetry=false,online_update=false`，任一 true 整份策略拒绝。签名/链/吊销规则复用生产配置包 release root；策略 ref 指向部署证据目录中的只读文件，digest 是 exact JCS SHA-256，在线下载与远端策略 URL 禁止。

Stage 14 生成 `CarrierEvidenceV1` strict JCS：`schema_version=1,stage14_run_id,stage14_started_at,stage14_completed_at,deployment_id,carrier_kind,provider_code,region_code,vm_sku,residency_jurisdiction_code,region_jurisdiction_code,tpm_version,vtpm_present,vtpm_attestation_digest,nested_virtualization_supported,nested_virtualization_evidence_ref,nested_virtualization_evidence_digest,customer_control_attestation_digest,managed_components,backup_failure_domain_code,backup_failure_domain_evidence_digest,backup_separation_evidence[{dimension,evidence_digest}],verified_at,verifier_subject,carrier_policy_digest,authorizations[]`。`stage14_run_id` 是 UUIDv7，started < completed 且窗口最长 8 小时，`verified_at=stage14_completed_at`；所有 child 的 run id 相等且 observed_at 闭区间落在该窗口内。物理机与 VM 的 `tpm_version` 首版都固定为字面量 `2.0`；物理机的 `vm_sku/vtpm_present/vtpm_attestation_digest/nested_virtualization_supported/nested ref/nested digest` 取 `NULL/false/NULL/false/NULL/NULL`，物理 TPM 2.0 原始证据按 server spec 同一 Stage 14 run 保留。VM 必须 vTPM=true、attestation digest 非空，`vm_sku` 逐字命中 policy 对应 provider/region 的唯一批准项。VM 的 nested flag 必须与当前 provider/SKU probe 相等；为 true 时 ref/digest 均非空且验证下述 exact report，为 false 时二者均为空并禁止启用 Hyper-V local transport。managed_components 必须逐项与 policy 的七个 false 相同，三维 backup evidence 各恰一项，`backup_failure_domain_evidence_digest` 必须非空。

`VtpmAttestationEvidenceV1` 是 VM 唯一 vTPM child，exact JCS 字段恰为 `schema_version=1,stage14_run_id,deployment_id,provider_code,region_code,vm_sku,profile="TPM2_QUOTE_SHA256_V1",challenge_nonce_b64url,ak_public_tpm2b_public_b64url,ak_certificate_chain_der_b64url,quote_tpm2b_attest_b64url,quote_tpm2b_signature_b64url,signature_scheme,pcr_bank="SHA256",pcr_selection,pcr_values,event_log_b64url,event_log_sha256,secure_boot_enabled,measured_boot_verified,probe_build_digest,observed_at`。所有 base64url 均为 RFC 4648 §5 no-pad canonical round-trip：challenge 解码恰好 32 bytes 且不得全零，AK/quote/signature/event-log 解码分别为 1..4096/1..65536/1..4096/1..524288 bytes；证书链恰 1..8 个 DER X.509 字符串，每个 1..16384 bytes、总解码不超 65536 bytes，顺序为 leaf→root。Stage 14 gate coordinator 在采集前以 OS CSPRNG 生成 nonce，在同 `deployment_id+carrier_policy_digest+stage14_run_id` 的本次运行内以 SHA-256 hash set 去重，任何重复立即中止整次 run。TPM quote `extraData` 必须逐 byte 等于 `SHA-256(ASCII("EP-CARRIER-VTPM-CHALLENGE-V1\0") || stage14_run_id[16] || deployment_id[16] || carrier_policy_digest[32] || challenge_nonce[32])`，故旧 run/nonce/quote 不能换入新 run。

`signature_scheme` 只取 `ECDSA_P256_SHA256|RSASSA_2048_SHA256`并与 AK public area 类型相等。validator 必须 strict parse 每张证书、验整条签名/有效时间/BasicConstraints/KeyUsage，要求 leaf 允许 digitalSignature 且其 SPKI 公钥逐值等于 TPM2B_PUBLIC 中 AK 公钥，并要求 root DER SPKI SHA-256 逐字命中当前 policy 该 provider/region 的 `vtpm_ak_trust_anchor_spki_sha256[]`；不读 OS 任意 trust store、AIA/CRL URL 或网络补链。`pcr_selection` 恰为升序 `[0,2,4,7,11]`，`pcr_values` 恰五项 strict `{index,value_sha256}` 且同序/同集合；全部 digest 均为 64 lowerhex。event log bytes 必须为 TCG PC Client crypto-agile event log（Spec ID Event03 后只允许 SHA-256 bank events），`event_log_sha256=SHA-256(decoded event_log_b64url)`；validator 独立验 quote signature、重算 selected-PCR digest、重放该有界 event log，并核对 Secure Boot 与五个 PCR，两个 boot boolean 必须 true。任一 DER/TPM2B/event-log 尾随 bytes、非 canonical 长度、证书链/nonce/PCR/event-log/签名不符都拒绝。`vtpm_attestation_digest=SHA-256(exact JCS bytes)`，并必须同时等于 deployment record 的 `vtpm_attestation_ref` 末段和 `CarrierEvidenceV1` 字段。

`CustomerControlEvidenceV1` 是物理机与 VM 都必须的客户控制权 child，exact JCS 字段恰为 `schema_version=1,stage14_run_id,deployment_id,carrier_kind,provider_code,region_code,customer_control_plane_subject_digest,windows_machine_sid_digest,customer_holds_os_admin,customer_holds_backup_credentials,customer_holds_kms_or_hsm_control,vendor_interactive_login_present,vendor_remote_support_enabled,managed_components,probe_build_digest,observed_at`。两个 identity digest 是签名 probe 从当前 provider account/site controller 规范 subject 和 Windows machine SID 计算的 SHA-256 lowerhex，不保存原标识；三个 customer boolean 必须 true，两个 vendor boolean 必须 false，managed-components 恰为 policy 的七个 false。`customer_control_attestation_digest=SHA-256(exact JCS bytes)`，并必须同时等于 deployment record 的 `carrier_attestation_ref` 末段和 `CarrierEvidenceV1` 字段。

`NestedVirtualizationEvidenceV1` exact JCS 字段恰为 `schema_version=1,stage14_run_id,deployment_id,provider_code,region_code,vm_sku,windows_hypervisor_present,hyperv_isolation_probe_passed,probe_build_digest,observed_at`；两个布尔值必须都为 true，digest 是本次 Stage 14 签名 probe executable 的 SHA-256，`observed_at` 落在同一运行窗口。其 opaque ref kind 固定 `nested-virtualization`，末段 digest 等于 exact JCS bytes SHA-256，并随 CarrierEvidence 一起受双人授权与部署 KMS 完整签名覆盖。Hyper-V transport gate 必须同时复核 policy SKU flag、CarrierEvidence 三字段、该 report digest，以及 `CarrierFactProbe` 当前 provider/region/SKU/nested/hypervisor facts；四处任一变化即只关闭该 transport。

`BackupDimensionProbeEvidenceV1` 是嵌在 bundle 内的 strict object，字段恰为 `schema_version=1,stage14_run_id,deployment_id,dimension,production_domain_digest,backup_domain_digest,separation_mechanism,backup_write_identity_separate,production_identity_can_delete_backup,restore_probe_digest,probe_build_digest,observed_at`。四个 digest 都是 64 lowerhex，两个 domain digest 必须不同，write-identity 必须 true，production-can-delete 必须 false，restore digest 必须绑定本次实际恢复 probe。mechanism 与 dimension 唯一对应：`SITE_OR_REGION→DISTINCT_SITE|DISTINCT_REGION`、`ACCOUNT_OR_CREDENTIAL_DOMAIN→DISTINCT_ACCOUNT|DISTINCT_CREDENTIAL_DOMAIN`、`MEDIA_OR_IMMUTABILITY_DOMAIN→OFFLINE_MEDIA|IMMUTABLE_OBJECT_LOCK`；其他组合拒绝。每项摘要 `evidence_digest=SHA-256(ASCII("EP-CARRIER-BACKUP-DIMENSION-V1\0") || JCS(BackupDimensionProbeEvidenceV1))`。

`BackupFailureDomainEvidenceV1` 是最大 1048576 bytes 的 RFC 8785 strict JCS，字段恰为 `schema_version=1,stage14_run_id,deployment_id,carrier_kind,backup_failure_domain_code,observed_at,entries[]`；其 `observed_at` 与三个嵌入 probe evidence 的 observed_at 全部落在同一 CarrierEvidence Stage 14 窗口。`entries` 恰三项，按 `dimension` bytes 排序且集合逐元素等于 `SITE_OR_REGION|ACCOUNT_OR_CREDENTIAL_DOMAIN|MEDIA_OR_IMMUTABILITY_DOMAIN`；每项字段恰为 `dimension,probe_evidence,evidence_digest`，`probe_evidence` 是上段 exact object 且内部 dimension 必须逐字相等，digest 按上段 domain-separated preimage 重算，三项不得重复或为空。`backup_failure_domain_evidence_ref` 的 kind 固定为 `backup-failure-domain`，末段必须等于该 bundle 的 exact JCS SHA-256；该值还必须逐字等于 `CarrierEvidenceV1.backup_failure_domain_evidence_digest`，而 bundle 三项的 `{dimension,evidence_digest}` 投影必须逐项等于 `CarrierEvidenceV1.backup_separation_evidence`。因此一列 ref 唯一绑定一个包含三份原始当前 probe facts 的 bundle，不把三个互不关联的 digest 塞进同一 ref，也不增加第十五个 deployment_records 列。

`authorizations` 恰两项并按 role bytes 排序，每项 strict 字段恰为 `role,subject,approved_at,signature_key_ref,signature_key_version,signature_p1363_b64url`；role 集合逐元素等于 `SECURITY|OPERATIONS`，两项 subject、key ref 和批准职责必须互异。先对不含 `authorizations` 的其余 CarrierEvidenceV1 字段做 JCS 并得 `authorization_body_digest=SHA-256(JCS(body))`；每名授权人用客户授权登记中与该 role 匹配的 ECDSA P-256 key 对 `SHA-256("EP-CARRIER-EVIDENCE-AUTH-V1\0" || authorization_body_digest[32] || role UTF-8)` 签 canonical low-S IEEE-P1363 64 bytes。附上两项后得到完整 evidence JCS 与 `evidence_digest=SHA-256(exact JCS)`。相邻 sidecar `CarrierEvidenceSignatureV1` 也是 strict JCS，字段恰为 `schema_version=1,purpose="CARRIER_EVIDENCE_V1",deployment_id,evidence_digest,key_ref,key_version,signer_subject,signature_p1363_b64url`；所有 `signature_p1363_b64url` 都是 RFC 4648 §5 no-pad canonical string，解码恰好 64 bytes 且重编码逐 byte 相等，禁止 integer array/lowerhex/DER/padding。部署 KMS/HSM 以 purpose `CARRIER_EVIDENCE_V1` 对 `SHA-256(ASCII("EP-CARRIER-EVIDENCE-V1\0") || deployment_id[16] || evidence_digest[32])` 答 canonical low-S IEEE-P1363 64-byte ECDSA P-256 签名。gate 只接受 current 或 retired-nonrevoked 且绑定原发布批次的 key，revoked 立即失败。两名授权签名、部署 KMS sidecar、主体/职责登记、key 状态、digest 与 JCS 均由 gate 验证；一人双签、同 key 双签、role/purpose/preimage 不匹配、DER/high-S/非 64-byte 签名均拒绝。

`CarrierFactProbeResultV1` 是 signed Stage 14 probe executable 给 Rust adapter 的唯一输出，exact JCS 字段恰为 `schema_version=1,stage14_run_id,deployment_id,carrier_kind,machine_sid_digest,site_code,provider_code,region_code,vm_sku,tpm_version,vtpm_present,vtpm_attestation_digest,nested_virtualization_supported,windows_hypervisor_present,hyperv_isolation_probe_passed,customer_control_attestation_digest,managed_components,backup_failure_domain_code,backup_failure_domain_evidence_digest,observed_at,probe_build_digest`。physical 的 `site_code=region_code`、`vm_sku/vtpm digest=NULL`；VM 的 site_code=NULL 且 vTPM digest 非空。`machine_sid_digest`、其余 digest 为 64 lowerhex，tpm_version 恰为 `2.0`，布尔与空值条件必须逐项等于已验 child/current OS facts；该 result 不含路径、原始 account/SID、attestation bytes、secret 或签名。`CarrierFactProbe` 唯一方法固定为 `collect(stage14_run_id: Uuid, deployment_id: Uuid) -> Result<CarrierFactProbeResultV1, AppError>`，adapter 必须先以上文字节/形状上限 strict parse，再暴露 typed facts。

唯一验证入口固定为 `validate_deployment_carrier(record: &DeploymentRecord, policy: &CarrierPolicyV1, evidence: &CarrierEvidenceV1, facts: &dyn CarrierFactProbe) -> Result<(), AppError>`。validator 以 evidence 的 run/deployment id 调用上述唯一 probe method，不得由 ref 字符串或 evidence 自报值代替 current probe；逐项核 policy Authenticode/CMS 链与吊销、evidence 两名授权签名和部署 KMS 签名、全部 child exact parser/preimage/ref/digest、`stage14_run_id/deployment_id/carrier_policy_digest` 绑定、deployment_records 十四列、legacy/new/current guard、当前 machine/provider/region/SKU/TPM/nested facts、managed components 七 false 和三维备份隔离。`CarrierPolicyV1` 不含有效期字段，也不从自由文本推断有效期：policy 必须来自当前签名发布包且 key 未吊销。CarrierEvidence 以 exact run id/start/end 取代模糊“当前窗口”；每次发布认证、carrier facts 或 policy digest 变化都重新生成，旧 evidence 不沿用。API 只返回 ref/digest/结论，不返回 attestation 或授权签名正文。

所有 carrier 证据 ref 都是 opaque reference，不是路径或 URL。唯一形状为 `ep-evidence://carrier/<lowercase-deployment-uuid>/<kind>/sha256/<64-lowerhex>`，`kind` 只允许 `policy|evidence|vtpm|nested-virtualization|backup-failure-domain|customer-control`；末段必须等于对应 exact bytes digest，deployment id 必须等于本行。编译期解析根固定 `C:\ProgramData\EnterprisePlatform\evidence\carrier\<deployment-id>\<kind>\`。`policy` 文件恰为 `<digest>.jcs` 加 detached CMS `<digest>.p7s`；`evidence` 恰为 `<digest>.jcs` 加上段 `<digest>.sig.jcs`；`vtpm|nested-virtualization|backup-failure-domain|customer-control` 四类 child 全部恰为 `<digest>.jcs`，均按本节统一 1048576-byte 上限与各自 exact schema 解析，不存在 provider-specific 后缀、自由 binary 或插件解析器分支。四类 child evidence **不另要求相邻签名**，只以各自 exact JCS bytes digest 被完整、双人授权且部署 KMS 签名的 CarrierEvidenceV1 绑定；缺 child、digest/ref 末段不符或 CarrierEvidence 签名失效均整体验证失败，不能用未绑定 sidecar 替代。owner 固定 SYSTEM、断继承，SYSTEM/Administrators/ep-ops 可管理，ep-core 只读与 `READ_CONTROL`，其他账户无 ACE；打开时拒绝 UNC、device path、`..`、ADS、reparse point、hardlink escape 与大小写/8.3 碰撞，并逐次核 owner/DACL/ref/digest。DB/API/审计只保存和返回 opaque ref/digest/结论，永不返回解析后的本地绝对路径。

`platform_ops.deployment_records` 追加：

- `carrier_kind text`：`CUSTOMER_CONTROLLED_PHYSICAL|CUSTOMER_CONTROLLED_DOMESTIC_IAAS_VM`；
- `provider_code text`：物理机固定 `CUSTOMER_CONTROLLED`，VM 为合同批准的 provider code；
- `region_code text`：物理机为客户机房 site code，VM 为 provider region code；
- `residency_jurisdiction_code text` 与 `region_jurisdiction_code text`；两者必须相等；
- `vtpm_present boolean`、`vtpm_attestation_ref text null`：VM 必须 true 且证据非空；物理机必须 false 且该 ref 为空，物理 TPM 信息继续进 `server_spec`；
- `backup_failure_domain_code text` 与 `backup_failure_domain_evidence_ref text`：均必填；
- `carrier_attestation_ref text`：客户控制权、provider/region、管理员责任与禁用托管组件的签字证据。
- `carrier_policy_ref text` 与 `carrier_policy_digest bytea`：上述签名 policy 的只读引用与 32-byte digest；
- `carrier_evidence_ref text` 与 `carrier_evidence_digest bytea`：上述签名 evidence 的只读引用与 32-byte digest。

条件 CHECK 必须保证 VM 的 provider/region/vTPM/境内证据完整；物理机的 provider/vTPM 形状正确。应用和 Stage 14 门禁再校验 `backup_failure_domain_code` 与生产 carrier 至少在 site/zone 或 region、账户/凭据域及介质三维中满足已批准的隔离规则，不能用同一 VM 的另一个目录、同一虚拟磁盘或可被同一在线管理员直接覆盖的快照冒充离站副本。

为使已有数据库可迁移，上述十四列物理上先 nullable 且无默认，整行 CHECK 只允许“十四列全空的 legacy row”或“十四列全部满足完整 shape”；不得用逐列 NOT NULL 直接加到已有表。迁移安装 `BEFORE INSERT` trigger 强制新行必须完整，现有全空 legacy row 只允许按既有 immutable guard 写 `superseded_at`，不能原地补列、成为 current/active 或通过任何 F-55/Stage 14 gate。升级后客户必须新增一条完整 revision 并把旧行 supersede；current selector 与发布门禁只接受完整 revision。这样历史证据保留、fresh database 可直接写完整事实，且迁移不会因旧行无 carrier 事实而物理失败。

### 6.3 云残余风险

客户与合同必须明确披露：IaaS provider 或客户租户 root 可在平台控制面之外复制磁盘/内存、回滚快照、改网络或关机；vTPM 只能提高启动与密钥绑定证据强度，不能消除 provider/tenant 管理员风险。单机 VM 没有 HA，宿主或 region 故障期间服务中断，按备份恢复而非自动切换恢复。勒索软件防线仍是最小权限、服务账户隔离、离站副本不可由业务写入身份覆盖/删除、定期恢复演练与离线密钥材料，不是“上云即安全”。

## 7. 迁移建议与实施顺序

F-55 建议预留以下 9 个全局唯一 14 位版本。它们在本文中是唯一文件名；实现前必须按相同路径、slug、owner/stage 加入 `docs/migration-catalog.md`，不能临时换号。现有迁移绝不改写。

| 顺序 | 建议迁移 | 内容 |
|---:|---|---|
| 1 | `db/migrations/platform_ops/V20261024090000__platform_ops_create_ai_model_packages.sql` | 建 AI 模型包表、安装收据、security level、状态/唯一/不可变 guard |
| 2 | `db/migrations/platform_meta/V20261024090100__platform_meta_create_mcp_connectors.sql` | 建 connector 表、RLS、FK 与状态 guard |
| 3 | `db/migrations/platform_meta/V20261024090200__platform_meta_create_mcp_manifest_versions.sql` | 建 immutable manifest 版本、签名 key、附件/安装收据、active slot 与 shape trigger |
| 4 | `db/migrations/platform_authz/V20261024090300__platform_authz_create_mcp_human_grants.sql` | 建短期 grant、last proof counter、RLS、祖先 FK、受理即计数状态 trigger，并幂等 seed F-55 五权限/五 binding |
| 5 | `db/migrations/platform_meta/V20261024090400__platform_meta_add_server_admin_capability_rows.sql` | client CHECK 加 server_admin、回填 18 行、90 格 hash |
| 6 | `db/migrations/platform_core/V20261024090500__platform_core_add_server_admin_client_kind.sql` | `user_devices.client` 等持久化 client CHECK 只加 server_admin；Mcp 使用来源 device 例外 |
| 7 | `db/migrations/platform_audit/V20261024090600__platform_audit_add_server_admin_and_mcp_clients.sql` | audit client CHECK 加 server_admin/mcp，终态九值 |
| 8 | `db/migrations/platform_ops/V20261024090700__platform_ops_add_deployment_carrier.sql` | deployment carrier 十四个 nullable 列、legacy/full CHECK、新行/current guard 与签名 policy/evidence digest |
| 9 | `db/migrations/platform_core/V20261024090800__platform_core_backfill_f55_unpoliced_table_registry.sql` | 登记部署级 `platform_ops.ai_model_packages` 的 consumer/理由 |

执行顺序不能改变。Stage 13 尚未执行的 `V20261022090500__platform_meta_alter_config_package.sql` 必须先把 `config_package_items.item_kind` CHECK 从十六值改为包含 `MCP_CONNECTOR|MCP_MANIFEST_VERSION` 的十八值，F-55 不另造重复 ALTER；该迁移的 SQL 元数据测试必须断言十八值逐项相等。第 5–7 项与 `ClientKind` Rust 枚举、archcheck 期望、指标 label、SPA bootstrap、审计序列化必须同一发布批次；第 1 与第 9 项必须同一发布批次；第 2–4 项完成后才允许启用任何 MCP manifest。逻辑回退只禁用 AI/MCP/ServerAdmin 路由并保留表与数据；生产环境不得通过 DROP 表/列伪装回退。

## 8. 稳定错误码

实现时下列代码必须原样登记到 `docs/error-codes.md`；不得用通用 4xx/5xx 替代。

| 错误码 | category / HTTP / retryable | 精确触发 |
|---|---|---|
| `AI.MODEL_PACKAGE.SIGNATURE_INVALID` | BUSINESS_CONFLICT / 409 / false | 模型包签名、manifest 或任一文件 hash 不符 |
| `AI.MODEL_PACKAGE.NOT_ACTIVE` | INFRASTRUCTURE / 503 / true | 无唯一 ACTIVE 且已认证的模型包 |
| `AI.QUERY_PLAN.INVALID` | VALIDATION / 400 / false | 模型计划违反单数据集/字段/算子/limit 闭集 |
| `AI.INPUT.CONTEXT_LIMIT_EXCEEDED` | VALIDATION / 400 / false | prompt+目录+问题+最大新 token 超过签名模型/GGUF 较小 context 上限 |
| `AI.QUERY_PLAN.NOT_VISIBLE_OR_DENIED` | PERMISSION_DENIED / 404 / false | 仅 compose：结构合法的模型计划引用本轮裁剪目录中不存在或不可见的 dataset/field code，统一防枚举；execute 不使用此码 |
| `AI.QUERY_PLAN.CONFIRMATION_REQUIRED` | BUSINESS_CONFLICT / 409 / false | execute 未明确 `confirmed=true` |
| `AI.QUERY_PLAN.TOKEN_INVALID_OR_EXPIRED` | BUSINESS_CONFLICT / 409 / false | token 签名、版本、摘要或时限无效 |
| `AI.QUERY_PLAN.SECURITY_CONTEXT_CHANGED` | BUSINESS_CONFLICT / 409 / false | compose 后权限/密级/范围/目录/模型事实变化 |
| `AI.QUERY_PLAN.IDEMPOTENCY_NOT_ALLOWED` | VALIDATION / 400 / false | AI 只读 compose 或 execute 携带 Idempotency-Key |
| `AI.INFERENCE.CONCURRENCY_LIMIT` | INFRASTRUCTURE / 429 / true | ai-inferer 队列/并发封闭上限命中 |
| `AI.RESOURCE.BASELINE_NOT_CERTIFIED` | INFRASTRUCTURE / 503 / true | 生产启用时 Stage 14 资源证据缺失或不再匹配 |
| `MCP.PROTOCOL.VERSION_UNSUPPORTED` | VALIDATION / 400 / false | 版本不是 2026-07-28 |
| `MCP.REQUEST.INVALID` | VALIDATION / 400 / false | 非法 JSON-RPC、batch/notification/response、非法 id/params 或额外业务字段 |
| `MCP.PAYLOAD.TOO_LARGE` | VALIDATION / 413 / false | HTTP/IPC request、manifest、response 或 chunk 超过固定上限 |
| `MCP.PROTOCOL.HEADER_MISMATCH` | VALIDATION / 400 / false | 必需 transport header 缺失、额外、非法 sentinel 或与 body 不符 |
| `MCP.METHOD.NOT_ALLOWED` | PERMISSION_DENIED / 404 / false | 请求不在六方法闭集或命中禁用 capability |
| `MCP.MANIFEST.INVALID_OR_UNSIGNED` | BUSINESS_CONFLICT / 409 / false | manifest shape、JCS digest、签名、附件或 origin 无效 |
| `MCP.MANIFEST.CAPABILITY_DENIED` | PERMISSION_DENIED / 403 / false | 方法/工具/资源/字段不在活动 manifest |
| `MCP.GRANT.INVALID_OR_EXPIRED` | PERMISSION_DENIED / 403 / false | grant hash、TTL、次数、session/device/法人/manifest 任一无效 |
| `MCP.DEVICE_PROOF.INVALID` | PERMISSION_DENIED / 403 / false | DPoP 签名、设备、公钥、counter、timestamp 无效或重放/乱序 |
| `MCP.TOOL.NOT_VISIBLE_OR_DENIED` | PERMISSION_DENIED / 404 / false | tool 不存在或当前用户不可见，统一防枚举 |
| `MCP.RESOURCE.NOT_VISIBLE_OR_DENIED` | PERMISSION_DENIED / 404 / false | resource/template 不存在或当前用户不可见，统一防枚举 |
| `MCP.TOOL.HIGH_RISK_FORBIDDEN` | PERMISSION_DENIED / 403 / false | 七类 `HighRiskOperation`（含 `DATA_MIGRATION`）、合同终止或审批结论动作命中绝对禁区 |
| `MCP.IDEMPOTENCY.REQUIRED` | VALIDATION / 400 / false | ExistingCommand 未带 Idempotency-Key |
| `MCP.CREDENTIAL.REF_INVALID` | INFRASTRUCTURE / 503 / true | wincred ref 不存在、当前服务 credential set 不可读、用途/字符约束不符、CredentialBlob 不在 1..2560 bytes 或 secret 读取失败；2561 bytes 必须在调用 Win32 API 前命中本码 |
| `MCP.REMOTE.UNAVAILABLE` | EXTERNAL_SYSTEM / 502 / true | 允许的远端 origin 超时、TLS 或协议失败 |
| `MCP.RESPONSE.SCHEMA_INVALID` | EXTERNAL_SYSTEM / 502 / true | 远端/本地 terminal response 不超过 8 MiB，但 JSON、JSON-RPC schema 或允许字段不合法；超字节界只用 `MCP.PAYLOAD.TOO_LARGE` |
| `MCP.LOCAL.CONTAINMENT_FAILED` | INFRASTRUCTURE / 503 / true | 子进程签名、Job Object、网络/文件/container 收容未成立 |
| `MCP.AUDIT.UNAVAILABLE` | INFRASTRUCTURE / 503 / false | completion slot、审计 ATTEMPT/COMPLETION 写入、flush/replay 或结果确认不可用；可能已有外部副作用，禁止自动重试 |
| `MCP.RATE_LIMITED` | INFRASTRUCTURE / 429 / true | connector 的每分钟或在途固定上限命中 |
| `MCP.CALL.TIMEOUT` | EXTERNAL_SYSTEM / 504 / true | MCP 调用超过 30 秒绝对时限 |
| `OPS.DEPLOYMENT.CARRIER_NOT_ALLOWED` | BUSINESS_CONFLICT / 409 / false | 除下列 region/vTPM/backup 专码外，policy/evidence 缺失、strict shape/签名/链/吊销/ref/digest/双人授权/deployment 或 policy 绑定/十四列/current guard/当前 fact probe 任一无效，carrier 不在两值闭集，或出现托管组件 |
| `OPS.DEPLOYMENT.REGION_NOT_DOMESTIC` | BUSINESS_CONFLICT / 409 / false | region jurisdiction 与客户数据驻留法域不一致 |
| `OPS.DEPLOYMENT.VTPM_EVIDENCE_MISSING` | BUSINESS_CONFLICT / 409 / false | VM 缺 vTPM 或 attestation |
| `OPS.BACKUP.FAILURE_DOMAIN_NOT_SEPARATE` | BUSINESS_CONFLICT / 409 / false | 备份落点与生产故障/凭据域不隔离 |

其余账号、授权、SoD、审批、并发、配置发布、附件与 query 错误继续复用既有码。

## 9. 配置与指标

### 9.1 配置

以下是唯一配置键与硬界：

| 键 | 默认/边界 |
|---|---|
| `EP__AI__ENABLED` | false；只有 F-56 current grant 对目标法人含 `F55LocalAi`、状态为 `Active|ExpiringSoon|GracePeriod`，`RG-LICENSE-MODULE-LIFECYCLE-GREEN`、ACTIVE 模型和两项 AI 门禁均成立才可 true |
| `EP__AI__PLAN_TTL_SECONDS` | 固定 300，不可调高 |
| `EP__AI__MAX_CONCURRENT_REQUESTS` | 固定 15；必须与资源认证负载一致 |
| `EP__AI__QUEUE_CAPACITY` | 固定 30 |
| `EP__AI__COMPOSE_TIMEOUT_MS` | 固定 120000 |
| `EP__AI__RESULT_ROW_LIMIT` | 固定 1000 |
| `EP__AI__RESULT_BYTES_LIMIT` | 固定 8388608 |
| `EP__MCP__INBOUND_ENABLED` | false；只有 F-56 current grant 对目标法人含 `F55Mcp`、状态为 `Active|ExpiringSoon|GracePeriod`，`RG-LICENSE-MODULE-LIFECYCLE-GREEN`、compatible ACTIVE inbound manifest、`RG-MCP-CONFORMANCE-GREEN` 与 `RG-MCP-CONTAINMENT-GREEN` 同时成立才可 true |
| `EP__MCP__OUTBOUND_ENABLED` | false；只有与入站相同的一项 F-56 `F55Mcp` 当前授权及 `RG-LICENSE-MODULE-LIFECYCLE-GREEN` 成立，且 compatible ACTIVE outbound manifest、`RG-MCP-CONFORMANCE-GREEN` 与 `RG-MCP-CONTAINMENT-GREEN` 同时成立才可 true；不存在方向许可证 |
| `EP__MCP__GRANT_TTL_SECONDS` | 默认/最大均 600 |
| `EP__MCP__MAX_CALLS_PER_GRANT` | 默认/最大均 100 |
| `EP__MCP__REQUEST_BYTES_LIMIT` | 固定 1048576 |
| `EP__MCP__RESPONSE_BYTES_LIMIT` | 固定 8388608 |
| `EP__MCP__CALL_TIMEOUT_MS` | 固定 30000 |
| `EP__MCP__REMOTE_CONNECT_TIMEOUT_MS` | 固定 5000 |
| `EP__MCP__LOCAL_START_TIMEOUT_MS` | 固定 10000 |

模型代码/版本、远端 origin、本地 entrypoint、credential ref、权限/字段范围和资源限额不能由环境变量覆盖，只来自已批准签名记录。AI 内存硬上限只从第 3.7 节算定，不设环境变量。只有 F-55 MCP connector 的持久 secret 存 Windows Credential Manager；平台通用机密继续使用 `secret://` KMS，任何配置导出只包含 ref。

### 9.2 指标

新增指标名与 label 闭集：

- `ep_ai_inference_requests_total{outcome}`，outcome=`ok|invalid_plan|timeout|busy|model_error|contained`；
- `ep_ai_inference_duration_seconds{outcome}`；
- `ep_ai_inference_queue_depth`、`ep_ai_working_set_bytes`、`ep_ai_job_memory_limit_bytes`、`ep_ai_gpu_vram_bytes`；
- `ep_ai_plan_validations_total{outcome,reason}`，reason 为编译期封闭枚举；
- `ep_mcp_calls_total{direction,transport,method,outcome}`；
- `ep_mcp_call_duration_seconds{direction,transport,method,outcome}`；
- `ep_mcp_payload_bytes_total{direction,flow}`，flow=`request|response`；
- `ep_mcp_active_grants`；
- `ep_mcp_denials_total{reason}`；
- `ep_mcp_local_children{transport}`、`ep_mcp_local_forced_terminations_total{reason}`；
- `ep_deployment_carrier_info{carrier}`，carrier 只有两个值。

不得以 legal_entity_id、user_id、grant_id、connector_code、tool/resource name、模型版本、provider、region、endpoint、错误正文或文件路径作 label。具体身份只进入受控审计。

## 10. 测试、威胁与发布门禁

### 10.1 必测矩阵

**许可共同前置：** 先完整通过 F-56 第 8 节矩阵并形成 `RG-LICENSE-MODULE-LIFECYCLE-GREEN` 的 Stage 14 签名证据，至少覆盖 permanent/subscription、未来生效、60 天临期、宽限第 1/30/31 天、撤销、时钟倒拨、current 直接续期与并发唯一、内外签名/current-slot/scope、模块五条合法动作及停用保留数据。AI/MCP 测试不得自建许可 fixture DTO；只能消费 F-56 `LicenseGrantPayloadV1`、`EntitlementCodeV1`、`LicenseStatus` 和 current/history 投影。共同 gate 缺失或非 PASS 时，下面 AI/MCP gate 即使各自测试通过也不能放行启用或发布。

**AI：** 九条收容断言全部正反例；模型包篡改/换 signer/DEV 入生产；首包加载；pipe 抢占、错 SID、错 operation、慢帧/超限；单数据集与 join/OR/NOT/SQL 负例；compose 后停用用户/撤权/改密级/改目录/换模型；结果不回模型与零日志/dump；15 并发、队列、Job Object 越限；审计失败零查询。

**MCP：** 2026-07-28 conformance；六方法逐项正例和全部禁用 capability 负例；GET/SSE/DCR/batch 拒绝；grant TTL/次数/logout/device/法人/manifest 失效；tools/list/resources/list/resources/templates/list 裁剪与猜名不可区分；普通 query、普通 command 幂等重放；七类 `HighRiskOperation`（含 `DATA_MIGRATION`）、合同终止、审批结论动作在发布和运行两层拒绝；远端 SSRF/DNS rebinding/redirect/proxy/SPKI/超时/超限；gateway 0 DB/KMS/file/outbox 静态检查；本地签名 stdio 与 Hyper-V container 正例、错签名/child/network/path traversal/host mount/secret 环境变量负例；response 8 MiB 的 `McpExchangeChunkStreamV1` 分块/乱序/hash/abort 正反例；响应 schema/字段/字节闭包。

**ServerAdmin：** 18×5 恰好 90 格、内置 hash 与表一致；ServerAdmin 两个 FULL、三个 VIEW_ONLY、其余 N/A 逐格断言；Mcp 不进矩阵；自填 client、ViewOnly 写、N/A 枚举、超级管理员假设、绕过 RLS/SoD/审批均拒绝；SPA 无独立进程/端口/可写资源；有/无 AI 权限的显示与调用分别正确。

**Carrier：** 同一安装包分别在 Windows Server 2022 物理机和境内 IaaS VM 跑完整 Stage 14；VM provider/region/vTPM/驻留/控制权证据；同 VM 目录/磁盘/快照冒充离站备份必败；不同故障域恢复正例；无托管 DB/KMS/队列；勒索恢复、密钥恢复、RPO/RTO 与离线补丁门禁两种 carrier 完全同判据。

### 10.2 威胁模型结论

| 威胁 | 强制控制 | 残余风险 |
|---|---|---|
| prompt injection / 模型越权字段 | 模型只见裁剪目录；计划闭集验证；执行逐次重检与 RLS | 模型可持续产无效计划，表现为可用性下降，不变成数据泄漏 |
| 查询结果经模型、日志、cache 泄漏 | 结果永不回模型；内存分桶；日志/审计/dump 禁正文 | 有权用户仍能在自己屏幕看到结果，端侧截图风险沿既有 DLP 披露 |
| 恶意或被替换模型 | 离线签名、逐文件 hash、数据包无代码、只读 ACL、启动复验 | 签名发行者本身被攻破仍是供应链风险，靠证书吊销和离线换包处置 |
| AI 资源耗尽拖垮交易 | 独立进程/Job Object/0 DB；固定 15 并发；Stage 14 联合负载 | AI 自身可能 busy/timeout，按错误明确失败 |
| MCP confused deputy / grant 被盗 | 短期人类 grant、token hash、绑定 session/device/le/manifest、逐次重检 | 10 分钟内同设备凭据同时被盗仍可在授予闭包内调用 |
| MCP tool/response 注入 | exact schema、field allowlist、已有 command/query registry、无 prompt/sampling | 获批准的外部 server 仍可返回业务上错误但合 schema 的值，调用方须按既有领域校验处理 |
| SSRF、任意网络/文件/shell | gateway 唯一 egress、固定 origin+SPKI、local 无网络/fs、无 shell | 客户批准的远端服务自身可记录被允许发送的字段，须在合同披露 |
| 写重放或绕审批 | ExistingCommand + Idempotency-Key + owner handler + SoD/审批逐次重检；高风险/终止绝对禁用 | 普通允许命令仍可能被合法用户误用，沿既有审计/撤销规则处置 |
| ServerAdmin 被当成超级管理员 | 无新角色、矩阵限制、现有 RLS/SoD/审批、静态 SPA | 拥有多个合法管理员账户的串谋风险沿既有双人控制披露 |
| IaaS 管理面、快照或宿主攻击 | 客户自控 tenant、境内 region、vTPM、内置 KMS/HSM、离站故障域 | provider/tenant root 仍在平台控制之外；单机无 HA |

### 10.3 发布门禁

F-55 新增的六项能力 gate 名称保持不变；F-56 另新增一项共同许可前置。七项全部进入 Stage 14 终态发布证据：

| gate | 通过条件 |
|---|---|
| `RG-LICENSE-MODULE-LIFECYCLE-GREEN` | F-56 第 8 节自动测试、签名部署清单与首装治理 child evidence、真实 PostgreSQL、配置包 inner/special-outer 签名/审批/发布全链、current/history 投影、四态/可信时间、模块五条合法动作与停用保留数据均有同 run/build/deployment 的 Stage 14 签名证据；trust rotation 直接枚举全部 `RELEASED` special `config_package_items`，逐项验证不可变 32-byte `accepted_trust_bundle_sha256`（普通/未发布为空）、grant 行与 source item 摘要相等，并分别交叉 current grant/current revocation 与 15 行 current module projection。历史 CRL `REVOKED` 按 F-56 隔离；许可 current 失败才改变 deployment LicenseStatus，module current 失败只关闭该 module effective runtime。模块 signer 吊销只允许 ACTIVE outer 的 exact DISABLE 收容，停用后只允许全新 ACTIVE inner+outer、更高 semver 的 UPGRADE；ServerAdmin `package_trust_status` exact 四值与现场重算相等；`LicenseAdmissionGate` 的 HTTP/MCP 与 core/worker registry exact-set、guard 顺序/shared error 与全部绕过负例也必须入证 |
| `RG-AI-CONTAINMENT-GREEN` | 九条 `tests/ai_containment` 全绿且名字/数量精确匹配 |
| `RG-AI-RESOURCE-CERTIFIED` | 第 3.7 节联合负载、算定值、模型/硬件摘要与既有通过线全部有签名报告 |
| `RG-MCP-CONFORMANCE-GREEN` | pin 版本六方法正例与全部禁用方法负例全绿 |
| `RG-MCP-CONTAINMENT-GREEN` | grant、manifest、gateway、plugin-host、高风险禁区、凭据与 egress 收容全绿 |
| `RG-SERVER-ADMIN-MATRIX-90-GREEN` | 90 格逐格、hash、ClientKind/audit/metrics、无新进程端口全绿 |
| `RG-DEPLOYMENT-CARRIER-EVIDENCE-GREEN` | 所选 carrier 的 provider/region/vTPM/故障域与完整 Stage 14 证据齐全 |

七项不是七个自由格式结果文件，而是 Stage 14 全局 signed gate ABI 中的七个逻辑 slot。全局 `Stage14GateCodeV1` wire exact-set 恰为 `RG-CI-PROBE-ABSENT|RG-TOOLS-EXCLUDED|RG-PLAINTEXT-SECRETS-ABSENT|RG-RLS-MATRIX-GREEN|RG-UNWIRED-ABSENT|RG-NO-UNDECIDABLE|RG-OFFSITE-COPY-PROTECTED|RG-EXTERNAL-CLAIMS-SIGNED|RG-LICENSE-MODULE-LIFECYCLE-GREEN|RG-AI-CONTAINMENT-GREEN|RG-AI-RESOURCE-CERTIFIED|RG-MCP-CONFORMANCE-GREEN|RG-MCP-CONTAINMENT-GREEN|RG-SERVER-ADMIN-MATRIX-90-GREEN|RG-DEPLOYMENT-CARRIER-EVIDENCE-GREEN`，未知值、别名或大小写变体拒绝；`Stage14GateOutcomeV1` wire 唯一为 `PASS`。所有 UUID、digest 与时间规范逐字采用 Stage 14 的 lowercase UUID、64-lowerhex SHA-256 与 UTC 秒精度。

```rust
pub struct Stage14GateEvidenceEntryV1 {
    pub evidence_code: EvidenceCodeV1,
    pub evidence_ref: OpaqueEvidenceRef,
    pub evidence_sha256: Sha256Digest,
}

pub struct Stage14GateEvidenceIndexV1 {
    pub schema_version: u16, // exact 1
    pub gate_code: Stage14GateCodeV1,
    pub stage14_run_id: Uuid,
    pub deployment_id: Uuid,
    pub product_build_sha256: Sha256Digest,
    pub entries: Vec<Stage14GateEvidenceEntryV1>, // 1..=256，排序且唯一
    pub observed_at: DateTime<Utc>,
}

pub struct Stage14GateResultV1 {
    pub schema_version: u16, // exact 1
    pub gate_code: Stage14GateCodeV1,
    pub outcome: Stage14GateOutcomeV1, // exact PASS
    pub stage14_run_id: Uuid,
    pub deployment_id: Uuid,
    pub product_build_sha256: Sha256Digest,
    pub evidence_index_ref: OpaqueEvidenceRef,
    pub evidence_index_sha256: Sha256Digest,
    pub observed_at: DateTime<Utc>,
}
```

每项 gate 的 final 根唯一为 `target/release-evidence/gates/<lowercase-stage14-run-id>/<lowercase-gate-code>/`，文件 exact-set 恰为 `evidence-index.v1.jcs`、`evidence-index.v1.jcs.sig.jcs`、`gate-result.v1.jcs`、`gate-result.v1.jcs.sig.jcs`。index ref 唯一为 `ep-evidence://stage14/<lowercase-stage14-run-id>/gates/<lowercase-gate-code>/evidence-index/sha256/<64-lowerhex>`，其末段、result 的 `evidence_index_sha256` 与 exact index bytes digest 三者相等。两个 sidecar 均为 Stage 14 的 `Stage14EvidenceSignatureV1`，purpose 只取 `STAGE14_GATE_EVIDENCE_INDEX_V1|STAGE14_GATE_RESULT_V1`；该签名类型的全局 purpose 闭集恰为这两项加 `F55_ENTITLEMENT_SNAPSHOT_V1|F55_EFFECTIVE_CONFIG_SNAPSHOT_V1|F55_PRODUCT_MANIFEST_PROJECTION_V1|F55_APPLICABILITY_V1|F55_DISABLED_MODE_V1|DEPLOYMENT_MANIFEST_EVIDENCE_V1` 八项。最后一项只签 Stage 14 `DeploymentManifestEvidenceV1`；`InitialGovernanceEvidenceV1` 不另设 purpose，只由 lifecycle child digest 传递绑定。任何一项都不得与独立 `SecretEvidenceSignatureV1` 的 purpose/preimage 混用。

`EvidenceCodeV1` wire 为 1..128 ASCII bytes 且只匹配 `[a-z][a-z0-9_]{0,127}`，本 ABI 的 `OpaqueEvidenceRef` 为 1..2048 ASCII bytes。index entries 按 `(evidence_code UTF-8 bytes,evidence_ref canonical bytes)` 排序且组合唯一；code 必须命中该 gate 的编译期 typed evidence exact roster，ref 只能交给该 roster 指定的有界 strict-JCS parser 与 `ep-evidence://stage14/<same-run>/.../sha256/<digest>` resolver。resolver 必须复验 typed bytes、ref/digest、相同 `stage14_run_id/deployment_id/product_build_sha256` 和同一 closed run window；raw/absolute/relative path、unknown kind、self-reference、escape、reparse、ADS、hardlink、跨 run/build/deployment 或窗口外证据全部拒绝。

F-55/F-56 所称“签名部署清单”只有 Stage 14 第 8.7 节的 `DeploymentManifestV1`，不得另造 schema。其 exact field 闭集是 `schema_version=1,purpose="EP-DEPLOYMENT-MANIFEST-V1",manifest_id,deployment_id,deployment_record_revision,product_version,product_build_sha256,employee_api_origin,license_trust_bundle_sha256,license_trusted_signer_subjects,deployment_trust_bundle_sha256,x509_login_trust_anchor_ref,x509_login_trust_bundle_sha256,manifest_signer_subject,customer_security_admin_certificates,artifacts,issued_at`。`license_trusted_signer_subjects:Vec<String>` 恰 1..64 项，每项 exact `spki-sha256:<64-lowerhex>`，按 UTF-8 bytes 严格升序且唯一，是 F-56 inner/special-outer signer 的唯一授权 roster；每个 signer 必须唯一命中，不能以能向 bundle 成链替代 roster membership。effective `release.trusted_signer_subjects=[]` 只表示无覆盖，非空必须 canonical 且与 signed roster 逐项/顺序 exact-equal，否则 readiness/ops/gate 失败。管理员 roster 为 2..16 个 exact `{certificate_sha256,signer_subject,subject_key_identifier_b64url}`，按 signer_subject bytes 排序，三列各自唯一，并分别绑定同一 leaf exact DER digest、`spki-sha256:` token 与 1..64-byte SKI；成员资格还要求该 exact leaf 同时在 deployment 与登录 bundle 的唯一 whole-chain/current highest-CRL 求值中为 ACTIVE，同 SPKI 换 DER、DN/serial/display name 或跨 entry 拼三列均不算成员。artifacts item exact 为 `{artifact_code,sha256}`，按 enum 顺序恰八项 `PRODUCT_MANIFEST|PRODUCT_MANIFEST_SIGNATURE|PRODUCT_SBOM|CORE_SERVER|EP_MIGRATE|PRODUCT_MODULES_MANIFEST|LICENSE_TRUST_BUNDLE|DEPLOYMENT_TRUST_BUNDLE`。`employee_api_origin` 是 DNS host 的 canonical HTTPS origin；`product_build_sha256` 是 exact `MANIFEST.sha256` bytes digest；两个 trust digest 与对应 artifact 相等。CAB 轮换必须同一离线发布批次原子提供新 signed roster 与对应 license bundle，禁止只换一侧、跨 batch/deployment/build 拼接或靠 local config 增删 signer。

该 signed roster 是历史 signer 的可识别身份集合，不是 ACTIVE-only 授权快照。任何新 manifest/CAB 安装前必须从全部永久 RELEASED special history 重建 inner+source-outer referenced-token exact-set，并证明它是新 roster 的子集；删除任一历史引用 token 即使 current 已换代也失败。保留旧 token 不绕过 CRL：REVOKED 优先，新 artifact 仍须 roster membership 且两层 ACTIVE。原 deployment 只有可信整库回退能恢复缺失 roster/history，真正清除旧身份必须新建 deployment；trust chunks、released-special registry、roster digest 与 DB exact-set共同给出这一 containment 证据。

产品保护的 `target/release-package/trust/deployment-roots.p7b` 必须进入 product `MANIFEST.sha256`/Authenticode CAB，安装为 `C:\ProgramData\EnterprisePlatform\trust\deployment-roots.p7b`；customer-specific `C:\ProgramData\EnterprisePlatform\deployment\deployment.manifest.v1.jcs|deployment.manifest.v1.p7s` 是独立 detached-CMS 制品，禁止进入产品 MANIFEST 形成 digest 环。manifest JCS≤262144、CMS≤1048576；CMS exact one SKI SignerInfo、SHA-256、signedAttrs 只含 contentType/messageDigest/signingTime、无 unsigned，signingTime 与 issued_at 语义 UTC 秒相等且 DER time 规范；leaf 为 DigitalSignature+CodeSigning。根 bundle 是≤1048576-byte empty-content/zero-signer CMS、1..64 CA、1..256 完整 base CRL；唯一链、整条 non-anchor、最高 covering CRL 和 ECDSA-P256/RSA-PSS-SHA256 exact 参数闭集逐字取 Stage 14，禁止 OS/network fallback。manifest 目录/root 文件 owner SYSTEM、DACL PROTECTED；SYSTEM/Administrators/ep-ops FullControl，ep-core/ep-worker 仅 `FILE_GENERIC_READ|FILE_TRAVERSE|READ_CONTROL|SYNCHRONIZE`，其余无 ACE。

Stage 14 `DeploymentManifestEvidenceV1` exact fields 为 `schema_version=1,stage14_run_id,deployment_id,deployment_record_revision,product_build_sha256,manifest_id,deployment_manifest_sha256,deployment_manifest_signature_sha256,license_trust_bundle_sha256,license_trusted_signer_subject_registry_sha256,deployment_trust_bundle_sha256,x509_login_trust_bundle_sha256,x509_login_ep_migrate_readback_sha256,x509_login_ep_core_readback_sha256,customer_security_admin_certificate_registry_sha256,installed_manifest_sd_sha256,installed_trust_bundle_sd_sha256,verification_transcript_sha256,observed_at`；signer registry digest 唯一按 domain/purpose `EP-DEPLOYMENT-LICENSE-TRUSTED-SIGNER-SUBJECT-REGISTRY-V1` 对 exact DTO `{schema_version:1,purpose,subjects}` 调统一 `projection_digest`，必须与 manifest subjects、initial-governance child 与 trust report 四方相等。固定根 `target/release-evidence/deployment/<run>/` 四文件、ref `ep-evidence://stage14/<run>/deployment/deployment-manifest-evidence/sha256/<digest>` 与 purpose `DEPLOYMENT_MANIFEST_EVIDENCE_V1` 逐字取 Stage 14。effective `EP__AUTH__X509__TRUST_ANCHOR_REF` 必须等于 manifest `x509_login_trust_anchor_ref`；ep-migrate/ep-core 两个 fixed recipient 解析的同类 CA+完整 base-CRL bundle exact bytes/digest 都等于 manifest 值。

其中 `verification_transcript_sha256` 唯一按 `projection_digest` 计算 domain/purpose=`EP-DEPLOYMENT-MANIFEST-VERIFICATION-TRANSCRIPT-V1` 的 Stage 14 exact DTO，字段为 evidence 除自身 digest 外的全部绑定字段加 `checks`；checks exact 九项顺序为 `MANIFEST_JCS_CANONICAL|MANIFEST_CMS_EXACT|DEPLOYMENT_CHAIN_ACTIVE|DEPLOYMENT_CRL_GLOBAL_HIGHEST_COVERING|LICENSE_ROSTER_HISTORY_CONTAINED|X509_BUNDLE_RECIPIENTS_EQUAL|CUSTOMER_ADMIN_ROSTER_ACTIVE|INSTALLED_DACL_EXACT|ARTIFACT_DIGESTS_EXACT`。它不是 stdout/direct hash，不带人工 outcome；任一检查失败不产 evidence，缺/多/乱序或 unknown check 均失败。

fresh-production child 也只消费 Stage 14 的 `InitialGovernanceEvidenceV1`：source 根固定 `C:\ProgramData\EnterprisePlatform\evidence\stage14\initial-governance\<lowercase-deployment-id>\`，三文件 exact `bootstrap.jcs|license.epcfg|initial-governance.receipt.v1.jcs`，路径 deployment id 与 manifest/receipt/DB 相等，SYSTEM owner/PROTECTED DACL 只给 SYSTEM/Administrators/ep-ops FullControl、ep-core 上述只读 mask，其余无 ACE。receipt 是 F-56 CREATE_NEW/flush/readback 的 unsigned exact JCS，禁止任何相邻 sidecar。child exact fields 中必须包含 `license_trusted_signer_subject_registry_sha256`，并绑定 deployment-manifest evidence、bootstrap body/two CMS、initial archive、receipt/审计 hash chain、数据库 bootstrap projection、首张 RELEASED grant/governance/source、同 signed key domain 最终 ACTIVE/activation audit、schema manifest 与 ep-migrate PE；该 registry digest 必须与 manifest/evidence/trust report相等，且首张 grant accepted inner/source outer signer 都唯一命中 signed roster。两个 operator 的 X509 verifier=`cert-sha256:<leaf exact DER digest>`、credential_handle=leaf SKI raw bytes，且各自以 password+X509 完成 sign-in/MFA。其输出唯一为 `target/release-evidence/initial-governance/<run>/initial-governance-evidence.v1.jcs`，ref `ep-evidence://stage14/<run>/initial-governance/initial-governance-evidence/sha256/<digest>`，无独立 sidecar，由 lifecycle manifest child digest 传递绑定。roster 缺/空/乱序/重复、local assertion 非空不等、signer 不在 roster、digest 不等或 CAB 只换一侧均是非零负例。

F-55 parser 不得为 initial-governance digest 另选前像。`InitialGovernanceEvidenceV1` 除上一段字段外还必须带 `customer_security_admin_certificate_registry_sha256` 与 `initial_governance_audit_chain_hash_sha256`；六类 semantic digest 逐字复用 Stage 14 的 `projection_digest`、strict/null/number/sort 规则与 exact DTO/domain：`EP-CUSTOMER-SECURITY-ADMIN-CERTIFICATE-REGISTRY-V1` root `{schema_version,purpose,entries}`、三字段 certificate entry；`EP-INITIAL-GOVERNANCE-AUTHORIZATION-REGISTRY-V1` root/两项七字段 authorization entry；`EP-INITIAL-GOVERNANCE-DATABASE-PROJECTION-V1` 对 F-56 audit.after 内嵌 exact `{schema_version,purpose,legal_entity,key_domain,operators,legal_entity_grants,roles,role_permission_pairs,user_role_grants,approval_chains}`，其中 `approval_chains` 必须是复数且按 `ApprovalScenarioCode::ALL` exact 37 项；`EP-INITIAL-GOVERNANCE-KEY-DOMAIN-PROJECTION-V1` 对 Stage 14 冻结的完整 ACTIVE current row；`EP-INITIAL-GOVERNANCE-DATA-KEY-MATRIX-V1` 对 `{schema_version,purpose,key_domain_id,legal_entity_id,activation_event_id,entries}` 与固定 16 行；以及每名 operator 的 `EP-INITIAL-GOVERNANCE-AUTHENTICATION-AUDIT-PROJECTION-V1` 对既有 Stage 4 表的 exact `{schema_version,purpose,bootstrap_role,user_id,device_id,login_attempts,mfa_challenge,session}`。最后一类不用未冻结的 audit action：login_attempts 恰为 MFA_REQUIRED/SUCCESS 两行，challenge 恰为同 user/device/entity 的 CONSUMED SIGN_IN_MFA/X509，session token 仅在 collector 内存重散列命中唯一未撤销 session row，原文零落盘；challenge 与 session 的既存 `token_hash` 都必须按 64-lowerhex 输出，三表全部 `timestamptz` 非空值固定为 UTC RFC3339 恰六位小数、nullable key 永远存在且 DB NULL 只能为 JSON null；三 child 的 exact 字段集/摘要规则逐字取 Stage 14。DB projection 禁止 PHC/password verifier；initial audit after direct digest、append-only chain digest 与 domain projection digest分别重算且不得互换。unknown/missing/duplicate、36/38 条链、singular alias、null/number/sort/timestamp wire 漂移、认证三 child 缺重跨人/事务不闭合或 matrix 非 16 行都必须非零并阻止 common PASS。

其中 data-key matrix 是首次 activation 的 immutable snapshot：root exact 必须为 `{schema_version,purpose,key_domain_id,legal_entity_id,activation_event_id,entries}`，entry 只含 `{data_key_id,purpose,security_level_scope,version,algorithm,wrapped_key_sha256,wrap_kek_version,activated_at}`，恰 16 个 version-1 行从唯一 activation audit 重建；后续合法 rotation 的 current state/row_version/retiring/retired/destroyed 时间不入摘要，但同 id 不可变字段必须仍相等。key-domain projection 的 `created_at/updated_at` 固定为 UTC RFC3339 恰六位小数而非 whole-second，其余时间按 Stage 14 规则。`first_released_grant_projection_sha256` 另用 `EP-F56-FIRST-RELEASED-GRANT-PROJECTION-V1` exact root `{schema_version,purpose,grant_id,payload,payload_sha256,inner_signature_cms_sha256,inner_signer_subject,accepted_trust_bundle_sha256,source_config_package_id,source_config_item_id}`；payload 是完整 F-56 grant 且 supersedes=null，并与 DB/source/accepted audit/trust entry逐值闭合，禁止用 current projection 或 grant id 摘要代替。

每名 operator 的 `authentication_transcript_sha256` 唯一使用 domain/purpose `EP-INITIAL-GOVERNANCE-AUTHENTICATION-TRANSCRIPT-V1` 与 exact DTO `{schema_version,purpose,bootstrap_role,user_id,device_id,password_x509_sign_in_exit_code,complete_mfa_exit_code,authentication_audit_projection_sha256}` 调 `projection_digest`；两个 exit code 必须为 JSON number 0，末字段命中三表 projection。它不含 secret、token、CMS、source address 或自由 stdout，DTO/digest/operator evidence 三者必须相等。

共同 gate 的编译期 typed roster 恰为且只为 `evidence_code=license_module_lifecycle_matrix|license_admission_registry_exact_set|license_admission_negative_matrix|license_trust_rotation_exact_set` 四项；四项 exact bytes 由已验签 gate index 逐项绑定，lifecycle case/trust chunks 再由各自 manifest 传递绑定，不增加 report-specific signature purpose。source 根唯一为 `target/release-evidence/license-module/<lowercase-stage14-run-id>/`；顶层文件恰为 `license-module-lifecycle.v1.jcs|license-admission-registry.v1.jcs|license-admission-negatives.v1.jcs|license-trust-rotation.v1.jcs`，case 只在 `lifecycle/<lowercase-snake-case-wire>.v1.jcs`，chunk 只在 `trust-rotation-chunks/<ten-digit-zero-padded-1-based-u32-chunk-no>.v1.jcs`。四项顶层 ref 依次为 `ep-evidence://stage14/<same-run>/license-module/license-module-lifecycle/sha256/<digest>`、`ep-evidence://stage14/<same-run>/license-module/license-admission-registry/sha256/<digest>`、`ep-evidence://stage14/<same-run>/license-module/license-admission-negatives/sha256/<digest>`、`ep-evidence://stage14/<same-run>/license-module/license-trust-rotation/sha256/<digest>`；case ref 只取 `ep-evidence://stage14/<same-run>/license-module/license-module-lifecycle-case/<case-wire>/sha256/<digest>`，chunk ref 只取 `ep-evidence://stage14/<same-run>/license-module/license-trust-rotation-chunk/<ten-digit-zero-padded-chunk-no>/sha256/<digest>`。每个 case/chunk/admission JCS≤1048576 bytes；trust 顶层由受信 DB exact-set 流式生成并以 checked `u64` 累计，受证据卷容量约束但不设历史条目数或 256-chunk 业务上限。唯一 parser/resolver 是 Stage 14 `tools/release-gate/src/license_module.rs`。实现者不得留下未登记的“F-56 其他证据”，也不得用自由 JSON、数据库布尔值或合并 report 替代。下列 ABI 是该 Stage 14 实现段必须逐字段匹配的权威字段闭集：

```rust
pub enum F56LicenseModuleLifecycleCaseV1 {
    LicenseTimeAndStatus,
    RenewalAndCurrentSignerRecovery,
    SpecialPackageEnvelopeAndSignature,
    UsageAndScope,
    ModuleTransitionsAndDrain,
    ModuleDisableRetentionAndReenable,
    SpecialPackageAndServerAdminImport,
    RestrictedEffectMatrix,
    F55EntitlementProjection,
    PostgresqlTerminalShape,
}

pub struct F56LicenseModuleLifecycleCaseEvidenceV1 {
    pub schema_version: u16,
    pub stage14_run_id: Uuid,
    pub deployment_id: Uuid,
    pub product_build_sha256: Sha256Digest,
    pub case: F56LicenseModuleLifecycleCaseV1,
    pub test_binary_roster_sha256: Sha256Digest,
    pub test_binary_count: u16,
    pub command_roster_sha256: Sha256Digest,
    pub command_count: u16,
    pub fixture_registry_sha256: Sha256Digest,
    pub assertion_registry_sha256: Sha256Digest,
    pub assertion_count: u32,
    pub passed_count: u32,
    pub failed_count: u32,
    pub aggregate_exit_code: i32,
    pub environment_facts_sha256: Sha256Digest,
    pub execution_transcript_sha256: Sha256Digest,
    pub observed_at: DateTime<Utc>,
}

pub struct F56LicenseModuleLifecycleCaseRefV1 {
    pub case: F56LicenseModuleLifecycleCaseV1,
    pub case_ref: OpaqueEvidenceRef,
    pub case_sha256: Sha256Digest,
}

pub struct F56LicenseModuleLifecycleEvidenceV1 {
    pub schema_version: u16,
    pub stage14_run_id: Uuid,
    pub deployment_id: Uuid,
    pub product_build_sha256: Sha256Digest,
    pub deployment_manifest_evidence_ref: OpaqueEvidenceRef,
    pub deployment_manifest_evidence_sha256: Sha256Digest,
    pub initial_governance_evidence_ref: OpaqueEvidenceRef,
    pub initial_governance_evidence_sha256: Sha256Digest,
    pub entries: Vec<F56LicenseModuleLifecycleCaseRefV1>,
    pub observed_at: DateTime<Utc>,
}

pub struct F56LicenseAdmissionRegistryEvidenceV1 {
    pub schema_version: u16,
    pub stage14_run_id: Uuid,
    pub deployment_id: Uuid,
    pub product_build_sha256: Sha256Digest,
    pub core_binding_registry_sha256: Sha256Digest,
    pub core_actual_operation_registry_sha256: Sha256Digest,
    pub worker_binding_registry_sha256: Sha256Digest,
    pub worker_actual_operation_registry_sha256: Sha256Digest,
    pub core_binding_count: u32,
    pub worker_binding_count: u32,
    pub xtask_exact_set_report_sha256: Sha256Digest,
    pub blocking_selfcheck_report_sha256: Sha256Digest,
    pub observed_at: DateTime<Utc>,
}

pub enum F56LicenseAdmissionNegativeCaseV1 {
    MissingBinding,
    ExtraBinding,
    DuplicateBinding,
    WrongConfigReleaseResolver,
    WrongMcpInboundResolver,
    LegalEntityScopeBypass,
    FirstOrRetryAsInFlightConvergence,
    SharedGuardOrderOrErrorRewrite,
}

pub enum F56LicenseAdmissionObservedErrorV1 {
    PlatformLicenseRestricted,
}

pub struct F56LicenseAdmissionNegativeResultV1 {
    pub case: F56LicenseAdmissionNegativeCaseV1,
    pub xtask_exit_code: Option<NonZeroI32>,
    pub blocking_selfcheck_exit_code: Option<NonZeroI32>,
    pub runtime_exit_code: Option<NonZeroI32>,
    pub observed_error_code: Option<F56LicenseAdmissionObservedErrorV1>,
    pub probe_report_sha256: Sha256Digest,
}

pub struct F56LicenseAdmissionNegativeEvidenceV1 {
    pub schema_version: u16,
    pub stage14_run_id: Uuid,
    pub deployment_id: Uuid,
    pub product_build_sha256: Sha256Digest,
    pub entries: Vec<F56LicenseAdmissionNegativeResultV1>,
    pub observed_at: DateTime<Utc>,
}

pub enum F56TrustRotationArtifactKindV1 { LicenseGrant, LicenseRevocation, ModulePackage }
pub enum F56TrustRotationItemResultV1 {
    Trusted,
    HistoricalSignerRevoked,
    CurrentModuleSignerRevokedContained,
    ModuleSignerRevokedDisableAuthorization,
}
pub enum F56TrustRotationSignerStateV1 { Active, Retired, Revoked }
pub enum F56CurrentProjectionKindV1 { CurrentGrant, CurrentRevocation, CurrentModule }

pub struct F56TrustRotationItemEvidenceV1 {
    pub schema_version: u16,
    pub purpose: String, // exact "EP-F56-TRUST-ROTATION-ITEM-V1"
    pub config_package_id: Uuid,
    pub config_item_id: Uuid,
    pub origin_config_item_id: Uuid,
    pub artifact_kind: F56TrustRotationArtifactKindV1,
    pub artifact_id: Uuid,
    pub accepted_at: DateTime<Utc>,
    pub acceptance_audit_event_id: Uuid,
    pub acceptance_audit_payload_sha256: Sha256Digest,
    pub acceptance_audit_chain_hash_sha256: Sha256Digest,
    pub accepted_trust_bundle_sha256: Sha256Digest,
    pub accepted_trust_bundle_evidence_ref: OpaqueEvidenceRef,
    pub validation_trust_bundle_sha256: Sha256Digest,
    pub source_projection_sha256: Sha256Digest,
    pub payload_sha256: Sha256Digest,
    pub inner_signature_cms_sha256: Sha256Digest,
    pub accepted_inner_signer_subject: String,
    pub accepted_inner_signer_state: F56TrustRotationSignerStateV1,
    pub accepted_inner_chain_sha256: Sha256Digest,
    pub validation_inner_chain_sha256: Sha256Digest,
    pub outer_manifest_sha256: Sha256Digest,
    pub outer_signature_cms_sha256: Sha256Digest,
    pub source_outer_signer_subject: String,
    pub source_outer_signer_state: F56TrustRotationSignerStateV1,
    pub accepted_outer_chain_sha256: Sha256Digest,
    pub validation_outer_chain_sha256: Sha256Digest,
    pub module_code: Option<ModuleCode>,
    pub module_action: Option<ModulePackageActionV1>,
    pub current_projection_kind: Option<F56CurrentProjectionKindV1>,
    pub current_projection_sha256: Option<Sha256Digest>,
    pub current_module_install_state: Option<ModuleState>,
    pub recovery_peer_config_item_id: Option<Uuid>,
    pub module_signer_revoked_disabled_audit_event_id: Option<Uuid>,
    pub module_signer_revoked_disabled_audit_payload_sha256: Option<Sha256Digest>,
    pub module_signer_revoked_disabled_audit_chain_hash_sha256: Option<Sha256Digest>,
    pub revoked_layer_crl_registry_sha256: Option<Sha256Digest>,
    pub result: F56TrustRotationItemResultV1,
}

pub struct F56TrustRotationChunkV1 {
    pub schema_version: u16,
    pub stage14_run_id: Uuid,
    pub deployment_id: Uuid,
    pub product_build_sha256: Sha256Digest,
    pub chunk_no: u32,
    pub total_chunks: u32,
    pub entries: Vec<F56TrustRotationItemEvidenceV1>,
    pub observed_at: DateTime<Utc>,
}

pub struct F56TrustRotationChunkRefV1 {
    pub chunk_no: u32,
    pub entry_count: u16,
    pub chunk_ref: OpaqueEvidenceRef,
    pub chunk_sha256: Sha256Digest,
}

pub struct F56LicenseTrustRotationEvidenceV1 {
    pub schema_version: u16,
    pub stage14_run_id: Uuid,
    pub deployment_id: Uuid,
    pub product_build_sha256: Sha256Digest,
    pub validation_trust_bundle_sha256: Sha256Digest,
    pub license_trusted_signer_subject_registry_sha256: Sha256Digest,
    pub trusted_now: DateTime<Utc>,
    pub complete_base_crl_registry_sha256: Sha256Digest,
    pub highest_covering_crl_registry_sha256: Sha256Digest,
    pub released_special_item_registry_sha256: Sha256Digest,
    pub ordinary_or_unreleased_null_registry_sha256: Sha256Digest,
    pub ordinary_or_unreleased_item_count: u64,
    pub current_license_projection_sha256: Sha256Digest,
    pub current_grant_projection_sha256: Option<Sha256Digest>,
    pub current_revocation_projection_sha256: Option<Sha256Digest>,
    pub current_license_status: LicenseStatus,
    pub current_license_restriction_reason: Option<LicenseRestrictionReason>,
    pub current_module_projection_registry_sha256: Sha256Digest,
    pub module_runtime_containment_report_sha256: Sha256Digest,
    pub trust_rotation_negative_matrix_sha256: Sha256Digest,
    pub total_entry_count: u64,
    pub chunks: Vec<F56TrustRotationChunkRefV1>,
    pub observed_at: DateTime<Utc>,
}
```

本镜像与 Stage 14 的 trust hash ABI 必须逐字同构，禁止在 F-55 parser 另选前像。统一派生摘要为 `SHA-256(ASCII(domain)||0x00||RFC8785_JCS(exact DTO))`；每个由该 projection-digest 原语计算的根（不含已有 raw report/chunk 文件 ABI）都含 `schema_version=1,purpose=domain`，其中 schema_version wire 只接受 JSON number `1`、拒绝 string `"1"`，unknown/duplicate/missing 拒绝。UUID canonical lowercase、时间 RFC3339 UTC whole-second、digest 64 lowerhex、enum 取冻结 wire；Option key 永远存在且无值为 JSON null，空 Vec 为 `[]`，数组按下列键升序去重。payload/item JCS、canonical manifest TOML、CMS DER、`.p7b`、`.epcfg`、case/chunk/report JCS、fixture/transcript 等 raw bytes 只取直接 SHA-256、不加 domain；raw case/chunk/report struct 不额外增加 purpose，但其承载的下列具名 semantic `*_sha256` 仍按 exact DTO/domain 重算。同一 JCS 的 ref/file direct-byte digest 与 semantic domain digest 是两个不同用途，禁止互换。chain 唯一使用 `SHA-256(ASCII("EP-CMS-CHAIN-V1")||0x00||leaf→intermediate→anchor 每张 DER 的 u32-big-endian 长度+exact bytes)`。

exact projection/registry/report 闭集如下，任何同名 digest 不按该 DTO/domain 重算都使 F-55 共同前置失败：

- F-56 三个具名 typed audit `CONFIG_SPECIAL_ACCEPTED|LICENSE_TRUSTED_TIME_CHECKPOINT|MODULE_SIGNER_REVOKED_DISABLED` 都按各自 strict DTO 解析，`schema_version` wire 恰为 JSON number `1`，不适用 Stage 3 只给无具名 ABI 的 numeric-string fallback；string `"1"` 分别必须在 acceptance negative、`license_time_and_status` lifecycle negative 与 recovery negative 中真实非零拒绝。
- `license_trusted_signer_subject_registry_sha256` 逐字复用 Stage 14 domain/purpose `EP-DEPLOYMENT-LICENSE-TRUSTED-SIGNER-SUBJECT-REGISTRY-V1` 与 exact DTO `{schema_version:1,purpose,subjects}`；subjects 与已验签 `DeploymentManifestV1.license_trusted_signer_subjects` exact-equal，manifest evidence、initial-governance child、trust report 与每个 inner/source-outer membership 四方闭合。全部 RELEASED history 的 inner+source-outer referenced-token exact-set 必须由 chunks/registry/DB 重算并为 roster 子集；任何 local 非空不等、删除历史引用 token、跨 deployment/build/batch、只轮换 bundle/roster 一侧或 signer 不在 roster都先于 chain state 失败。保留 token 仅供历史识别，CRL REVOKED 优先且不授权新 artifact。

- `source_projection_sha256`：domain/purpose=`EP-CONFIG-SPECIAL-SOURCE-PROJECTION-V1`，DTO exact `{schema_version,purpose,config_package_id,package_no,source,status,content_hash,outer_signature_sha256,outer_signer_subject,outer_signed_at,config_item_id,item_kind,item_code,change_kind,sort_no,applies_to_legal_entity_ids,before_spec_sha256,after_spec_sha256,item_hash,accepted_trust_bundle_sha256}`；special 固定 `IMPORTED/RELEASED/ADD/1/[]/null`，`content_hash=outer_manifest_sha256`、`outer_signature_sha256=outer_signature_cms_sha256`、`after_spec_sha256=item_hash`。
- `released_special_item_registry_sha256`：`EP-F56-RELEASED-SPECIAL-ITEM-REGISTRY-V1` root `{schema_version,purpose,entries}`，entry `{artifact_kind,artifact_id,config_package_id,config_item_id,accepted_trust_bundle_sha256}`，按 artifact/package/item 排序且与 chunks exact-set/count 相等。`LICENSE_GRANT|MODULE_PACKAGE` special source package 首次进入 RELEASED 后永久保持 RELEASED，不走 generic `RELEASED→SUPERSEDED`；多份 RELEASED 是合法 history，current/history 只由 license/module projection决定。发现 `SUPERSEDED` special 必须在 registry 前失败。`ordinary_or_unreleased_null_registry_sha256`：`EP-F56-NULL-ACCEPTANCE-REGISTRY-V1` 同 root，entry `{classification,config_package_id,config_item_id,item_kind,package_status,accepted_trust_bundle_sha256}`；classification=`ORDINARY|UNRELEASED_SPECIAL`、末字段必须 null，按 package/item 排序；UNRELEASED_SPECIAL 只含从未 RELEASE 的合法状态，不含 SUPERSEDED，root 恰为全部 ordinary 加合法 unreleased special 且 count相等。
- `current_grant_projection_sha256`：`EP-F56-CURRENT-GRANT-PROJECTION-V1` DTO `{schema_version,purpose,row_version,payload,payload_sha256,inner_signature_cms_sha256,inner_signer_subject,accepted_trust_bundle_sha256,source_config_package_id,source_config_item_id,current_slot,superseded_at,last_trusted_at}`，payload 是全字段 F-56 `LicenseGrantPayloadV1`，current 固定 `0/null`。`current_revocation_projection_sha256`：`EP-F56-CURRENT-REVOCATION-PROJECTION-V1` DTO `{schema_version,purpose,grant_id,grant_row_version,payload,payload_sha256,inner_signature_cms_sha256,inner_signer_subject,accepted_trust_bundle_sha256,source_config_package_id,source_config_item_id,revoked_at}`，payload 是全字段 `LicenseRevocationPayloadV1`。`current_license_projection_sha256`：`EP-F56-CURRENT-LICENSE-PROJECTION-V1` DTO `{schema_version,purpose,current_grant_projection_sha256,current_revocation_projection_sha256,trusted_now,license_status,restriction_reason}`；两个 Option 命中顶层，零 current 恰为 `null/null/RESTRICTED/NO_CURRENT_GRANT`。
- 每个 module row：`EP-F56-CURRENT-MODULE-PROJECTION-V1` DTO `{schema_version,purpose,id,module_code,display_name,row_version,install_state,package_id,package_code,package_version,package_payload_sha256,package_signature_cms_sha256,package_signer_subject,package_signed_at,module_contract_version,module_contract_sha256,min_platform_version,max_platform_version_exclusive,released_on,source_config_package_id,source_config_item_id,installed_at,state_changed_at,enabled_at,disabled_at,last_transition_reason}`；三个 version wire 都是 `Option<SemVerV1>` strict object而非 string，已安装行的 package/min 必填；`module_contract_version` 的 Rust 类型虽为 u32，但签名 source、证据 DTO 与 PostgreSQL integer 的共同有效域唯一为 `1..=2147483647`，DB 不改 bigint，0/2147483648/溢出转换均拒绝。NOT_INSTALLED 所有 package/source/version/time/reason Option 为 null，并与 source inner逐值核对。registry 用 `EP-F56-CURRENT-MODULE-PROJECTION-REGISTRY-V1` root `{schema_version,purpose,entries}`、entry `{module_code,current_projection_sha256}`，恰 15 行按 module wire；chunk 只能命中对应单行 digest。
- complete CRL 用 `EP-F56-COMPLETE-BASE-CRL-REGISTRY-V1` root `{schema_version,purpose,entries}`，entry `{issuer_subject_der_sha256,issuer_spki_sha256,issuer_subject_key_identifier_b64url,crl_number_decimal,this_update,next_update,crl_der_sha256,signature_algorithm}`，算法 exact 只取 `ECDSA_P256_SHA256|RSA_PSS_SHA256`，枚举全部结构/签名合法 base CRL，按 issuer-name digest/SKI/numeric CRLNumber/time/digest。选择固定 global-highest-then-cover：对 inner/outer 唯一链的每个实际 issuer，先取其全局最高 numeric CRLNumber 且同号 DER 唯一，再要求最高号覆盖 trusted_now；所有 issuer 的这一前置必须先完整成功，任一缺失、过期、尚未生效或冲突即整项 UNTRUSTED，禁止回退低号、扫描任何 serial 或进入窄恢复。此前置全绿后才生成 highest covering `EP-F56-HIGHEST-COVERING-CRL-REGISTRY-V1`，entry `{artifact_kind,config_package_id,config_item_id,layer,chain_position,certificate_der_sha256,certificate_serial_hex,issuer_subject_der_sha256,issuer_subject_key_identifier_b64url,selected_crl_der_sha256,crl_number_decimal,this_update,next_update,serial_revoked}`，`chain_position:u16` 从 0=leaf 递增，serial 为最短 unsigned lowercase hex、无 leading `00`，每 item/INNER|OUTER/non-anchor 恰一行并按 artifact/package/item/layer/position；entry 的 revoked-layer digest 是同 root/domain 中仅保留该 item `serial_revoked=true` 行的子集，零撤销必须 null。
- license certificate/CRL extension parser 逐字复用 F-56 首版 profile：任何证书拒绝 nameConstraints/certificatePolicies/policyMappings/policyConstraints/inhibitAnyPolicy；leaf 只允许 required noncritical SKI/AKI、critical digitalSignature-only KU、noncritical codeSigning-only EKU 与 absent 或 critical CA=false/pathLen-absent BC；CA 只允许 required noncritical SKI/AKI、critical CA=true/pathLen-enforced BC 与 critical keyCertSign+cRLSign-only KU，零 EKU/未列 extension。CRL 只允许 required noncritical AKI+CRLNumber、required nextUpdate，零 IDP/delta/freshest/indirect/entry extension。合法 whole-chain/base-CRL golden bytes 必须在 verifier 开发前提供，逐 extension/critical/KU/EKU/AKI/pathLen 负例真实运行；不在文档选 crate，但实际实现依赖须由 Cargo.lock 与产品 SBOM 精确固定，升级即重跑共同 gate。
- containment 用 `EP-F56-MODULE-RUNTIME-CONTAINMENT-V1` root，恰 15 行 `{module_code,install_state,package_trust_status,source_config_item_id,trust_rotation_result,raw_enabled,effective_runtime_allowed,expected_effective_runtime_allowed,read_export_probe,write_probe,approval_probe,automation_claim_probe,outbound_probe}`；source/result 都是显式 Option，NOT_INSTALLED 恰为 null/null，两个 installed state 都非空并命中同 source trust entry。probe exact `{outcome,exit_code,observed_error_code,transcript_sha256}`，outcome=`ALLOWED|BLOCKED|NOT_APPLICABLE`，error 只为 null 或三个稳定平台/module error；ALLOWED 与 N/A 固定 `0/null`，BLOCKED 固定 nonzero/non-null，actual=expected，revoked-contained disabled 必须读/导出允许且四副作用阻断。
- negative matrix 用 `EP-F56-TRUST-ROTATION-NEGATIVE-MATRIX-V1` root、entry `{case,fixture_sha256,collector_exit_code,failure_stage,transcript_sha256}`，exit nonzero，stage=`SOURCE|ACCEPTANCE|CURRENT_PROJECTION|CHAIN|CRL|RECOVERY|CONTAINMENT|NULL_ACCEPTANCE`。case 顺序 exact 30 项：`MISSING_ORIGIN|DUPLICATE_ORIGIN|INNER_OUTER_MERGED|SIGNER_NOT_IN_DEPLOYMENT_ROSTER|CURRENT_REVOKED_WITHOUT_RECOVERY|CURRENT_UNTRUSTED|NEW_CANDIDATE_RETIRED|NEW_CANDIDATE_REVOKED|MISSING_BASE_CRL|EXPIRED_CRL|DUPLICATE_HIGHEST_CRL|DELTA_CRL|INDIRECT_CRL|REMOVE_FROM_CRL|MISSING_REVOKED_LAYER_EVIDENCE|HISTORICAL_NON_CRL_DRIFT|HISTORICAL_CLASSIFICATION_MISMATCH|RECOVERY_SUPERSEDES_MISMATCH|RECOVERY_GOVERNANCE_MISMATCH|RECOVERY_PEER_MISSING|RECOVERY_PEER_NONUNIQUE|RECOVERY_AUDIT_PROJECTION_DRIFT|SPECIAL_RELEASED_SUPERSEDED|MODULE_STILL_ENABLED|ORDINARY_ACCEPTED_DIGEST_NON_NULL|UNRELEASED_ACCEPTED_DIGEST_NON_NULL|ACCEPTED_TRUST_BUNDLE_MISSING|ACCEPTED_TRUST_BUNDLE_DIGEST_MISMATCH|ACCEPTANCE_AUDIT_MISSING_OR_MISMATCH|RETIRED_WITHOUT_FIRST_ACTIVE_EVIDENCE`。`SIGNER_NOT_IN_DEPLOYMENT_ROSTER` 必须从 otherwise-valid 新 manifest 删除一个仍被 RELEASED history inner/source outer 引用且可由 bundle 唯一成链的 token，在状态分类/安装前 `SOURCE` 非零失败。`RECOVERY_PEER_MISSING` 删除唯一 `MODULE_SIGNER_REVOKED_DISABLED` action，`RECOVERY_PEER_NONUNIQUE` 追加第二条同 module/recovery tuple 的 action，`RECOVERY_AUDIT_PROJECTION_DRIFT` 逐一变异 payload/hash chain/audit-before/after projection/object version/时间/四个 source-recovery id/reason digest；三者均须在配对前非零失败。`SPECIAL_RELEASED_SUPERSEDED` 把一个已首次 RELEASE 的 `LICENSE_GRANT|MODULE_PACKAGE` source package 篡改为 `SUPERSEDED`，必须在 exact-set 分类前以 `SOURCE` 非零失败。`MISSING_BASE_CRL|EXPIRED_CRL|DUPLICATE_HIGHEST_CRL` 三个 golden fixture 固定为一层已有 revoked serial hit、另一层分别缺 CRL/最高号过期/最高号同号 DER 冲突；三者都必须在 serial scan/recovery 前得到整项 UNTRUSTED、revoked-layer digest=null 与非零退出，禁止误判历史撤销包含态。
- `grant_trust_rotation_entry_sha256/revocation_trust_rotation_entry_sha256` 使用 `EP-F56-TRUST-ROTATION-ITEM-V1`，前像是带 `schema_version=1,purpose=domain` 的 `F56TrustRotationItemEvidenceV1` 全字段 exact object；Option 显式 null，chunk object、entry digest 与 F-55 summary 必须相等，禁止只 hash signer/source 子集或 chunk byte slice。

每个 entry 新增的接受字段必须命中同 source item 的唯一 append-only `platform.config_special.accepted.v1`。这是 F-56 具名 typed audit ABI，不适用 Stage 3 对无具名 ABI 的 numeric-string fallback；closed payload exact `{schema_version:1,purpose:"EP-CONFIG-SPECIAL-ACCEPTED-V1",config_package_id,config_item_id,artifact_kind,artifact_id,artifact_action,accepted_trusted_now,accepted_trust_bundle_sha256,inner_signer_subject,inner_chain_sha256,inner_trust_state,outer_signer_subject,outer_chain_sha256,outer_trust_state,payload_sha256,item_hash,content_hash,source_projection_sha256}`，schema_version 只接受 JSON number `1`，string `"1"` 必须命中 `ACCEPTANCE_AUDIT_MISSING_OR_MISMATCH` 非零负例。`accepted_at=accepted_trusted_now`，outer state=ACTIVE，inner state 只为 F-56 合法 `ACTIVE|RETIRED_NONREVOKED|REVOKED_AS_DISABLE_TARGET`；payload digest 是该 exact JCS 的 direct SHA-256，audit chain hash 按既有 `AuditWriter` 重算。`accepted_*_chain_sha256` 必须命中 immutable audit 的首次接受链，`validation_*_chain_sha256` 必须按本次 bundle/trusted_now 重建且 current signer state 只绑定后者，四者均按 `EP-CMS-CHAIN-V1` 重算。accepted bundle exact bytes 只从 `C:\ProgramData\EnterprisePlatform\evidence\license-trust-bundles\<digest>.p7b` 的不可变 CREATE_NEW/flush/readback、安全 DACL文件读取，再 exact-copy 至 `target/release-evidence/license-module/<run>/accepted-trust-bundles/<digest>.p7b`；ref 唯一为 `ep-evidence://stage14/<run>/license-module/license-accepted-trust-bundle/sha256/<digest>`。filename/ref、entry、运行根、copy 与接受审计五者必须相等，缺失或多余 bundle 都失败。

模块 signer 撤销后的窄停用还必须从同一 terminal batch 中唯一 `action='MODULE_SIGNER_REVOKED_DISABLED'` 的 append-only `AuditWriter` 事件重建 recovery peer，绝不按同 package、同 inner 或最近时间猜选。这同样是 F-56 typed audit ABI，不适用 generic numeric-string fallback。event `before` 是锁内更新前完整 `EP-F56-CURRENT-MODULE-PROJECTION-V1` typed DTO，exact keys 为 `{schema_version:1,purpose:"EP-F56-CURRENT-MODULE-PROJECTION-V1",id,module_code,display_name,row_version,install_state,package_id,package_code,package_version,package_payload_sha256,package_signature_cms_sha256,package_signer_subject,package_signed_at,module_contract_version,module_contract_sha256,min_platform_version,max_platform_version_exclusive,released_on,source_config_package_id,source_config_item_id,installed_at,state_changed_at,enabled_at,disabled_at,last_transition_reason}`；schema/version/SemVer 分量均为 JSON number，row version=`1..=9223372036854775807`、contract version=`1..=2147483647`，string `"1"`、前像缺失/多 key/越界都命中 `RECOVERY_AUDIT_PROJECTION_DRIFT`。

event `after` strict recovery payload exact 为 `{schema_version:1,purpose:"EP-MODULE-SIGNER-REVOKED-DISABLED-V1",module_code,previous_source_config_package_id,previous_source_config_item_id,recovery_config_package_id,recovery_config_item_id,before_projection_sha256,after_projection_sha256,disabled_at,reason_sha256}`，schema_version 仍为 JSON number `1`。before digest 从 audit before exact bytes 以 `EP-F56-CURRENT-MODULE-PROJECTION-V1` domain 重算；after DTO 只由 before 确定变换：row_version checked `+1`、`install_state=INSTALLED_DISABLED`、`state_changed_at=disabled_at=event.after.disabled_at`、`last_transition_reason=recovery item reason`，其余 key（含 previous source 与旧 inner/package）逐字保留，再重算 after digest。`reason_sha256=SHA-256(ASCII("EP-MODULE-DISABLE-REASON-V1")||0x00||UTF-8(recovery item reason))`；envelope 固定 object_type=`platform.module_registrations`、object_id=before.id、object_version=after.row_version、occurred_at=disabled_at。停用仍是 current 时 DB row 等于派生 after；后来已有合法动作时沿后续审计/投影链验证，不要求现态倒退。收容 pair 两项携带相同非空三个 audit Option，payload digest 是 event after JCS direct SHA、chain digest 从审计链重算；两个 peer 与 package/item/accepted event 只从 typed before/after 派生，其他 entry 四个 recovery/audit Option 全 null。缺/重 action、before 前像、hash chain、object version 或任一 id/digest/time/reason 漂移都禁止共同 PASS。

F-55 parser 对 special `.epcfg` 也逐字沿用同一 exactness：总长≤4,193,900、ZIP32 single-volume/STORE，local 与 central 都按 `manifest.toml,item.jcs,outer-signature.p7s` 固定顺序，header flags/version/time/date/attributes/offset/CRC/size、EOCD 与 330-byte overhead exact，零 ZIP64/descriptor/extra/comment/trailing；special `item_hash=SHA-256(item.jcs exact after-spec JCS bytes)`。inner/outer `.p7s` 都是完整 DER `ContentInfo`/`[0] EXPLICIT SignedData.version=3`，SKI SignerInfo v3，signedAttrs wire `[0] IMPLICIT` 而 signature preimage 是 canonical universal SET OF `0x31`；certificate set 无 root/CRL/多余证书。`license-roots.p7b` 是完整 DER degenerate `ContentInfo/SignedData.version=1`，empty content、空 digestAlgorithms/SignerInfos DER SET 与规范排序 CA/base-CRL bag。宽松 ZIP/CMS 解析或 raw SignedData 二义性不得进入共同 PASS。

`schema_version` 均 exact 1；`Sha256Digest` wire 恰为 64 lowerhex；`accepted_inner_signer_subject/source_outer_signer_subject` 分别来自 origin inner 与本 source outer，wire 都恰为 `spki-sha256:<64-lowerhex>`，两层 state 必须分别输出，禁止合并为一个 signer state。lifecycle 的 `deployment_manifest_evidence_ref/sha256` 与 `initial_governance_evidence_ref/sha256` 是必填 typed child，分别命中上一段固定 ref/exact bytes；deployment child 必须同时通过独立 CMS 与 `DEPLOYMENT_MANIFEST_EVIDENCE_V1` sidecar，initial-governance child 无 sidecar并由 lifecycle digest 传递绑定。两者及 lifecycle/case 均绑定同 run/deployment/build/closed window；缺 child、出现 receipt sidecar、把 receipt 当 child 或把 child 升为共同 gate 第五个顶层 code 都失败。lifecycle entries 按 enum 顺序 exact 十项，逐一对应 F-56 §8 第 1 至 10 项：许可时间/四态、续期/current-signer 恢复、special envelope/两层签名、用量/scope、模块状态边/依赖/排空、停用保留/再启用、special 限制及 ServerAdmin import/只读审批、Restricted effect、F-55 entitlement projection、PostgreSQL 终态。第一项 assertion registry 必须覆盖 `TrustedClockV1` 的 readiness 前持久/system startup anchor、OS monotonic 同进程不降，以及每个 special 推进点与 job-worker 目标间隔≤240 秒在同一 advisory lock 内按 deployment+240-second slot 耐久键处理 append-only checkpoint：`slot_utc=floor(unix_seconds/240)*240` 后转 canonical RFC3339 UTC whole-second，零行只追加首行，已有一行只验真复用，永不 UPDATE 或追加第二行；current 的 `last_trusted_at` 另以 CAS 单调推进。`ensure_checkpoint` 必须在业务 mutation 前锁内单次捕获 trusted_now/current id，terminal batch 禁止重算。还须覆盖 checkpoint/续期竞态、首行 `trusted_now/current_grant_id` 不随后续同 slot 动作改变、`last_trusted_at` 可高于首行、跨 slot 单调、回拨崩溃未持久窗口严格小于 300 秒、uptime 相邻成功 checkpoint 的 trusted-now 差值≤300 秒（>300 秒失败，留 60 秒调度预算）、slot 映射/单值/hash-chain/trajectory 告警及错误前跳只可从 Stage 14 可信备份恢复；daily-only 或单 wall-clock 测试不成立，该发布观测门限不宣称 NTP/TPM。每个 case 的 binary/command count 为 1..32，assertion count>0、passed=assertion、failed=0、aggregate exit=0，所有 registry/environment/transcript digest 来自本次真实 run；case ref/digest/文件必须相等。第 11 项只由 admission 两 report 与 trust report共同覆盖，因此四项顶层 code 是完整 roster。

零法人、零 current、零 bootstrap 的 development/test profile 可在固定 `RESTRICTED/NO_CURRENT_GRANT`、job-worker dormant 且不创建 checkpoint 的形状下进入受限 readiness；它不能推进业务/自动化/special，也永远不得成为 production Stage 14 evidence。production readiness 仍必须同时具有已验证 bootstrap/initial-governance 与当桶 append-only checkpoint；共同 gate collector 必须因缺 initial-governance child/checkpoint 对前述开发形状返回非零，禁止产出任何 production PASS/result。治理来源的唯一优先序是 current grant → 已验证 bootstrap → 仅首张 LICENSE_GRANT 首次发行事务内的 candidate；其他命令或事务不能使用 candidate fallback。

strict multipart config import 的 Windows、macOS 与 ServerAdmin 三入口必须复用同一 handler/parser。`RESTRICTED/NO_CURRENT_GRANT` 只允许在完整内外签名、来源、治理与 action 已 strict 确认后，把 exact special item 映射为首张 `LICENSE_GRANT` 的首次发行恢复；普通 attachment upload 不开放。Stage 14 recovery/disabled matrix 至少证明 Windows/macOS 零-current multipart 首发和 ServerAdmin 同 handler 正例，并证明普通包、MODULE_PACKAGE 非 `DISABLE`、通用 attachment upload、伪装 kind 与已有 current 后重用首发例外都稳定 `PLATFORM.LICENSE.RESTRICTED`、零 package/item/audit/file/attachment 写入。

registry count 各为 1..65535 且 declared/actual digest 相等；negative entries 按 enum 顺序 exact 八项，前五项具有非零 xtask+Blocking exit，后三项具有非零 runtime exit 与 exact `PLATFORM.LICENSE.RESTRICTED`。trust manifest/chunk 直接枚举全部 RELEASED grant/revocation/module special item，不能从 current projection 倒推；首次 RELEASE 后这些 special 永久保持 RELEASED，多份 RELEASED 是完整历史，current/history 只从 projection 交叉得出，任一 SUPERSEDED special 直接失败且不得吞入未发布 null registry。`total_entry_count` 与累计计数用 checked `u64`；`total_chunks:u32=ceil(total_entry_count/512)`，零 entry 时为 0，否则 `chunk_no:u32` 从 1 连续、十位 zero-pad 文件名/ref、每块 `entry_count:u16` 为 1..512，禁止 0/缺号/重复/超过 u32/格式漂移，不设 256-chunk 或其他历史条目业务上限，且 checked `sum(entry_count)=total_entry_count`。entries 跨块按 `(artifact_kind,config_package_id,config_item_id)` 排序唯一并与数据库 exact-set 相等。普通/未发布接受摘要必须为空，grant 行接受摘要与 source item 相等。每个 entry 的 `origin_config_item_id` 必须指向其 accepted inner 首次合法引入的唯一 RELEASED item；grant/revocation/新 INSTALL 或 UPGRADE 等于自身 source item，复用 inner 的 ENABLE/DISABLE/ROLLBACK_VERSION 指向唯一 exact INSTALL/UPGRADE origin。current grant/revocation entry digest 必须分别等于 manifest 对应 Option 且 module state 为空；current module entry digest 必须命中由 15 个 `(module_code,current_projection_sha256)` 排序对重算的 module registry；非 current 的 kind/digest/module-state 全空。`recovery_peer_config_item_id` 与三个 `module_signer_revoked_disabled_audit_*` Option 只允许两个模块收容 result 成组非空；peer 必须从唯一 `MODULE_SIGNER_REVOKED_DISABLED` action、审计 hash chain 与 before/after projection 派生，其他 entry 四项全 null。缺失/错误 origin、两层 subject/state 合并、正常 existing current 任一层为 REVOKED/UNTRUSTED、RETIRED 缺首次 ACTIVE exact 证明、新 candidate/action 任一层非 ACTIVE、撤销层 CRL registry 缺失、未撤销层不是 ACTIVE|RETIRED、special 被 SUPERSEDED、收容 action 缺失/重复/漂移都必须有具名负例并返回非零。

trust verifier 对 inner/outer 各自唯一链的 leaf 加全部 non-anchor intermediate 求一个层状态，不得只看 leaf。任何层状态分类前必须先为两层所有实际 issuer 完成上述 global-highest-then-cover registry；任一 issuer 缺失、最高号过期/尚未生效/同号冲突即整项 UNTRUSTED，且不得扫描任何 serial、产出 revoked-layer digest 或进入窄恢复。只有此前置全绿后，任一 non-anchor serial 命中才使该层 REVOKED；零命中且全部 non-anchor 当前有效才 ACTIVE；零命中、全部 signed-time 有效、origin/首次接受 exact 证据证明当时整层 ACTIVE、当前至少一张 non-anchor 已过期且其余无 not-yet-valid 才 RETIRED。anchor 必须 signed-time 有效；trusted-now 过期本身不退休，anchor 被移除/替换、零链/多链、约束失败或 CRL 缺失/过期/同号冲突/delta/indirect/removeFromCRL 都为 UNTRUSTED，且 UNTRUSTED 不进入 state wire、直接失败。inner/outer CMS signingTime 必须分别与 payload issued_at/manifest signed_at 语义 UTC whole-second 相等并采用规范 DER 时间；SignerInfo、证书与 CRL AlgorithmIdentifier 只接受 Stage 14 冻结的 ECDSA-P256 或 RSA-PSS-SHA256 exact 参数。正常 existing current 与非恢复 history 的 accepted inner/source outer 可各为 ACTIVE|RETIRED，RETIRED 必须有首次 ACTIVE 证据；本次新 grant/revocation candidate、INSTALL/UPGRADE 的两层必须均 ACTIVE；`TRUSTED` 才作正向证明。`HISTORICAL_SIGNER_REVOKED` 只允许非 current：accepted inner/source outer 各枚举 ACTIVE|RETIRED|REVOKED、至少一层 REVOKED、无 UNTRUSTED，`revoked_layer_crl_registry_sha256` exact 覆盖一至两层全部撤销证据，accepted/source/payload/digest/signature/首次接受 bundle bytes与审计全自洽；这是正确的 accepted-containment 包含态，不计 purchased/rollback/正向证明，但本身不是 FAIL。存在独立 `TRUSTED` current，且其 inner/outer 各为 ACTIVE，或为非撤销 RETIRED 并有同 origin/同 chain 的首次 ACTIVE exact 证据，其他共同 predicate 也全部通过时，common gate 可以 PASS。只有分类与实际 CRL containment 不等、接受证据缺失或其他历史 source/digest/signature/chain 非 CRL 漂移才令 gate 失败。current grant/revocation CRL 恢复必须由旧 history 隔离 entry 加新 current entry证明：旧两层取上述恢复集合且其余 bytes 自洽，新 grant inner+outer 均 ACTIVE、同 deployment/governance 并 direct-supersede 旧 current；其他失败不得借路。零 current 是合法 bootstrap 形状，trust manifest 必须同时为 `current_grant_projection_sha256=None/current_revocation_projection_sha256=None/RESTRICTED/NO_CURRENT_GRANT`，多 current 仍失败；current module 失信只关闭自身 effective runtime。模块 CRL 停用必须形成互指的一对：旧 current source=`CURRENT_MODULE_SIGNER_REVOKED_CONTAINED` 且已 `INSTALLED_DISABLED`，其两层各为 ACTIVE|RETIRED|REVOKED、至少一层 REVOKED、无 UNTRUSTED；新 exact DISABLE item=`MODULE_SIGNER_REVOKED_DISABLE_AUTHORIZATION`，source outer ACTIVE，原样携带与旧 entry 同 origin/subject/state/bytes 的 inner。两者均不作正向证明。停用后只接受全新 ACTIVE inner+outer、严格更高版本的 TRUSTED UPGRADE。special outer+inner 共用 `license-roots.p7b`，普通 outer 仍用部署 KMS。ServerAdmin 15 行 `package_trust_status` 必须逐项等于 `NOT_INSTALLED|TRUSTED|SIGNER_REVOKED|INVALID` 重算值。任一 report/chunk 跨 run/build/deployment/window、缺 code/ref/digest、或诊断失败被包装成 PASS 均拒绝。

F-55 entitlement snapshot 的 current ref 也使用唯一 closed hash ABI。每个 `F55LicenseGrantSummaryV1` 的 exact 字段按序为 `{schema_version,purpose,grant_id,source_row_version,license_no_sha256,license_kind,issued_at,valid_from,valid_to,legal_entity_scope,legal_entity_ids,supersedes_grant_id,superseded_at,is_current,last_trusted_at,payload_sha256,grant_signature_cms_sha256,grant_origin_config_item_id,grant_accepted_inner_signer_subject,grant_accepted_inner_signer_state,grant_source_outer_signer_subject,grant_source_outer_signer_state,revoked_at,revocation_id,revocation_issued_at,revocation_payload_sha256,revocation_signature_cms_sha256,revocation_origin_config_item_id,revocation_accepted_inner_signer_subject,revocation_accepted_inner_signer_state,revocation_source_outer_signer_subject,revocation_source_outer_signer_state,grant_source_config_package_id,grant_source_config_item_id,grant_source_accepted_trust_bundle_sha256,grant_trust_rotation_result,grant_trust_rotation_entry_sha256,revocation_source_config_package_id,revocation_source_config_item_id,revocation_source_accepted_trust_bundle_sha256,revocation_trust_rotation_result,revocation_trust_rotation_entry_sha256,trusted_now,status,restriction_reason,entitlement_codes}`，其中 `schema_version=1`、`purpose="EP-F55-LICENSE-GRANT-SUMMARY-V1"`。`F55CurrentGrantSummaryRefV1` exact 只有 `{grant_id,grant_summary_sha256}`；digest 必须以 `EP-F55-LICENSE-GRANT-SUMMARY-V1` domain 对相应 summary 全字段调用统一 `projection_digest`，Option key 无值仍为 null，`legal_entity_ids` 按 UUID bytes、`entitlement_codes` 按 wire bytes 排序去重。snapshot 中 summary、重算 digest 与 current ref 必须三者相等，禁止 hash 字段子集、grant id 或大文件 byte slice。

F-55 applicability 所消费的 Stage 14 产品 projection 也不得沿用不含模块目录的旧字段集。collector 必须从已签 `MANIFEST.sha256` closed roster 内唯一 `target/release-package/product-modules.v1.jcs` 和安装后固定 `C:\EP\product-modules.v1.jcs` 两处 safe-handle readback；两份 exact digest 必须相等，且无数据库、环境变量、ServerAdmin、MODULE_PACKAGE 或第二目录替代。projection 的模块字段 exact 为 `product_modules_manifest_sha256,installed_product_modules_manifest_sha256,product_version,product_modules,product_module_dag_conclusion`；`product_modules` 恰 15 行 `{module_code,module_contract_version,module_contract_sha256,module_dependencies_sha256}`，按 module wire 排序。每行 `module_contract_version` 必须在 `1..=2147483647` 且与 Rust u32、两份 manifest、current module projection、证据和 PostgreSQL integer 逐值相等；DB 不改 bigint，0/2147483648/越界转换是具名非零负例。每个 dependency digest 唯一使用 domain/purpose=`EP-F55-MODULE-DEPENDENCIES-V1` 对 exact DTO `{schema_version:1,purpose:"EP-F55-MODULE-DEPENDENCIES-V1",module_code,dependencies}` 调用统一 `projection_digest`；`dependencies` 按 ModuleCode wire 排序去重且可为空，禁止直接散列裸数组。DAG conclusion wire 只能为 `ACYCLIC`。缺文件、未被签名 roster 覆盖、digest 漂移、非 15 行、未知依赖、自环或成环都使产品 projection 与发布失败。

collector 只可在同父目录 `<lowercase-gate-code>.staging.<lowercase-uuidv7>` 用 `CREATE_NEW` 写四文件，close/readback/strict parse/digest/signature 全部通过后且 final 不存在时原子 rename；失败或中断必须非零且不得产出或接受 result，可控失败立即关闭并清理 staging，崩溃遗留 staging 在下次运行先隔离/清理且不可解析或打包。Stage 14 的九项永久 gate 和本节任何 `RequiredPass` 都必须具有同 run/deployment/build 的 exact index/result/two-sidecar 闭包；`DisabledEvidence|NotInBuild` 禁止对应 final result，也禁止孤儿 final index/sidecar。不存在 `FAIL|N/A|DISABLED|UNKNOWN` outcome，且不得以七个散落 digest 或文件数量替代逐份验签。

`RG-LICENSE-MODULE-LIFECYCLE-GREEN` 永远是 AI 与 MCP applicability 的共同前置，且基础产品发布时自身也必须真实 PASS；不得以 AI/MCP 未购买、配置关闭、禁用态报告或 capability gate PASS 把它降为 N/A。共同 gate 通过后，AI/MCP 的 purchased/currently-licensed 才能从同一份 F-56 current/history signed grant 投影重算；purchased 只含 TRUSTED、currently licensed 只接受 current grant 的 `Active|ExpiringSoon|GracePeriod`。零 current 的 Stage 14 entitlement snapshot 必须显式为 `current_grant=None/RESTRICTED/NO_CURRENT_GRANT`，空库还须 `grants=[]` 与四个布尔值全 false；不得序列化空 current object。这些布尔值决定发布 disposition，不改写运行时 effect 语义：全局 Restricted 只关闭写/审批/有副作用出站/新自动化，AI 纯读取/草稿及 `ReadReportAuditBackupExport` 继续可用；entitlement/source 失信在 capability payload 前失败关闭。AI/MCP 平台自身没有 module code；只有触及具体业务对象时才由该对象真实 owner-module effective gate 在 payload/inference/egress 前关闭对应路径，无关 module 失败不得全局关闭 AI/MCP。

AI/MCP 模块未购买或被停用时，其业务 HTTP 路由必须表现为不存在，数据和登记安全保留。九个产品服务及 `ep-ai/ep-integ/ep-plugin` 的 DACL health/control listener 可按统一安装基线存在，但 compose/exchange operation 必须在读取业务 payload 前 fail closed；禁用态不得出现 MCP HTTP route、业务网络 listener、出站连接、本地子进程或模型推理。发布基础产品必须用上述精确口径证明禁用态，不得把“进程/健康管道存在”误报为隐藏业务能力。若该客户购买并启用模块，对应 F-55 gate 必须全绿。

## 11. 开发退出条件与无未决声明

F-55 进入“已实现”之前必须同时满足：

1. 第 7 节 9 条迁移先入 catalog，再由 fresh database 完整执行；历史迁移 checksum 零变化；
2. `ClientKind` 八值、audit 九值、ServerAdmin 90 格和 `Mcp` 非矩阵例外在代码、数据库、指标、文档与测试一致；
3. `apps/ai-inferer`、`ep-ai` 五 operation、Windows 服务 SID/DACL、资源单位和所有计数同批落地，无半完成枚举；
4. AI compose→确认→execute 的 exact API 可用，结果字节在所有 ai-inferer ingress 中为零；
5. 九条 AI assertions、六项 F-55 release gate 与共同前置 `RG-LICENSE-MODULE-LIFECYCLE-GREEN` 具备可执行承载；许可证据只消费 F-56 current/history signed grant 投影，旧私有 payload 与旧四态均不可达；
6. MCP 六方法闭集、三种出站 transport、短期入站 grant、exact manifest、Credential Manager，以及“七类 `HighRiskOperation` + 合同终止 + 审批结论动作”三组绝对禁区全部可正反测试；WinCred 子矩阵必须在 Windows Server 2022 实机证明 SCM 加载 `ep-integ|ep-plugin` 服务账户 profile、两服务各自在 current token 下 CredWrite/Read、服务正常重启后同一 target 仍可读，且普通管理员直接 CredWrite、模拟服务 token 或手工 `LoadUserProfile` 不能冒充目标服务 vault；另证明八项 `ep-secretctl` 顶层命令闭集、2560 bytes 成功而 2561 bytes 在 Win32 调用前失败、REMOTE ASCII/no-whitespace、stdio UTF-8/4-byte framing、双人 CMS grant、60 秒单次管道、客户端身份、CREATE/ROTATE/DELETE probe/rollback、Event Log 和全路径 zeroize，并在 Win32 mutation 前、后及 probe 后三个 fault point 强杀服务，逐一证明 intent 恢复只能走 `CLOSED_FAILED→CLOSED`、能力仍禁用且需同 target/purpose 新 grant 纠正；HTTP、ServerAdmin、argv、env、secret 文件与 CLOSED 状态均不存在 secret 写入口；
7. gateway 仍为 0 DB/KMS/file/outbox，plugin-host 子进程收容成立，ServerAdmin 不新增进程/端口；
8. 物理机与 IaaS 两个 carrier 使用同一拓扑、门禁和恢复判据；所选 carrier 证据完整；
9. `docs/error-codes.md`、`docs/metrics-catalog.md`、数据字典、威胁模型、进程/IPC 白名单、阶段计划、迁移目录与发布证据清单完成对应回写；
10. OCR、向量、RAG、工业协议仍不可达；Excel 导入导出正例不回退，Excel 加载项无任何制品、路由或隐藏开关。

**无未决声明：** 本文范围内没有产品、架构、安全、API、ABI、表、状态、协议、客户端矩阵、云 carrier、错误码、指标、配置、测试或发布门禁待选择；许可、签名模块包、entitlement 和共同许可 gate 已由 F-56 终态关闭。Stage 14 尚未取得的 AI 资源数值和具体客户 carrier 证据属于按本文确定公式与判据产生的部署认证结果，不是开发问题或设计待决；它们只阻止相应能力发布/启用，不阻止立即开发。未来要恢复 OCR、向量/RAG、工业协议、Excel 加载项、MCP 禁用 capability、外部 AI、SaaS、HA、Kubernetes、托管 DB/KMS 或新增高风险 MCP 写入，必须另立高于 F-56 的后续裁定。
