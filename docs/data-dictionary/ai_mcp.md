# F-55 本地 AI、MCP 与部署 carrier 数据字典

> **F-57 状态：`REGISTRY_PENDING_REBASELINE`。** 本地模型表属于延期输入；MCP/provider 与 carrier 只能保留未冲突细节，须按 F-57 动态工具 manifest、短期 handle、HDD 和客户自控边界再基线。
>
> **激活/owner tasks：Tasks 14、15。** 本分册目前不是 F-57 实现权威，完成相应任务的再基线与显式激活前不得据此实施；Task 15 只交付 null provider boundary，本地 AI 实现仍为 `DEFERRED`。

历史状态（F-57 下无效）：**曾标为开发前冻结、尚未执行迁移**。本分册是 F-55 四张新增表（**F-66 注：F-55 时点确为四张；第五张 `mcp_transport_registry_versions` 系 F-57 再基线新增，见 :9 更正与 §2.1**）、部署 carrier 追加列及所需候选键的历史逐列登记；AI/MCP 本体与 carrier 以 `docs/superpowers/specs/2026-08-22-f55-approved-ai-mcp-server-admin-cloud-freeze.md` 为历史来源，许可证、签名模块包、AI/MCP entitlement 与共同许可门禁的重叠面曾由更晚的 `docs/superpowers/specs/2026-08-22-f56-license-signed-module-package-freeze.md` 原子替换。它不表示迁移、代码、模型包、门禁或认证已经完成。

**五张**新增表固定为（本行原写「四张」，实际本册定义五张——第五张 `platform_meta.mcp_transport_registry_versions` 见 §2.1，F-65 更正）：

1. `platform_ops.ai_model_packages`（部署级、无 RLS）；
2. `platform_meta.mcp_connectors`（法人级、FORCE RLS）；
3. `platform_meta.mcp_manifest_versions`（法人级、FORCE RLS）；
4. `platform_authz.mcp_human_grants`（法人级、FORCE RLS）；
5. `platform_meta.mcp_transport_registry_versions`（见 §2.1；F-65 补入清单——本清单原列四项与首句「五张」不符）。

F-55 不新增 AI 草案表、AI 结果表、MCP 调用日志表、MCP session 表、secret 表或 Outbox 事件表。AI 计划只存在于五分钟签名 token；MCP 调用审计复用 `platform_audit.audit_events`；只有 F-55 MCP connector 的持久 credential 存 Windows Credential Manager，平台通用机密继续使用 ADR-0007 的 `secret://` KMS。

## 1. `platform_ops.ai_model_packages`

部署级可更新表，不带 `legal_entity_id`、不建 RLS；登记到 `platform_core.unpoliced_table_registry`，固定 `admission_basis=SAME_FOR_ALL_ENTITIES`、`isolation_entry=platform.ops.ai_model_package.read`、`matrix_case_id=rls-unpoliced-ai-model-packages`。只有 core-server 的模型选择器与 ops 只读视图可读，`ai-inferer` 无数据库连接。

| 列名 | 类型 | 可空 | 默认 | 语义 |
|---|---|---:|---|---|
| id | uuid | 否 | 应用侧 UUIDv7 | 主键 |
| security_level | smallint | 否 | 40 | 固定机密；CHECK `=40` |
| data_scope_tags | text[] | 否 | `'{}'` | 部署事实不按法人切分，固定空数组 |
| row_version | bigint | 否 | 1 | 状态乐观锁 |
| created_at | timestamptz | 否 | `now()` | 登记时间 |
| created_by | uuid | 否 | 无 | 真实单列 FK 到 `platform_core.user_accounts(id)` |
| updated_at | timestamptz | 否 | `now()` | 最后状态变化时间 |
| updated_by | uuid | 否 | 无 | 真实单列 FK 到 `platform_core.user_accounts(id)` |
| model_code | text | 否 | 无 | 1..64，字符集 `[a-z0-9._-]` |
| model_version | text | 否 | 无 | 1..64，字符集 `[A-Za-z0-9._+-]` |
| runtime_abi_version | int | 否 | 无 | 首版固定 1 |
| package_digest | bytea | 否 | 无 | 整包 SHA-256，固定 32 字节 |
| manifest_digest | bytea | 否 | 无 | JCS manifest SHA-256，固定 32 字节 |
| signer_subject | text | 否 | 无 | 已验证签名主体，1..512 |
| signature_kind | text | 否 | 无 | `PROD_AUTHENTICODE\|DEV_ECDSA_P256` |
| installed_root_ref | text | 否 | 无 | ACL 保护只读安装根引用，不是可提交路径 |
| install_receipt_id | uuid | 否 | 无 | `ops.signed_artifact.install_receipt.v1` 幂等收据 id |
| installed_at | timestamptz | 否 | 无 | 安装器完成原子发布的时点 |
| prompt_template_version | text | 否 | 无 | 活动固定提示模板版本，1..64 |
| max_context_tokens | int | 否 | 无 | 必须大于 2048，给固定 2048 输出 token 留空间 |
| max_concurrent_requests | int | 否 | 15 | 固定 15，CHECK `=15` |
| execution_profile | text | 否 | 无 | `CPU_LOCAL\|GPU_LOCAL` |
| resource_formula_version | text | 否 | `AI_RAM_V1_0_095_HOST` | 首版只允许该字面量 |
| certification_report_ref | text | 是 | 无 | exact `ep-evidence://ai-resource/<release-batch>/<model-package>/<cpu-local\|gpu-local>/sha256/<digest>` opaque ref |
| certification_report_digest | bytea | 是 | 无 | `AiResourceCertificationReportV1` exact JCS SHA-256；32 字节，与 ref 同空同非空 |
| verified_at | timestamptz | 是 | 无 | 包签名、hash、Runtime ABI 双重验证完成时点 |
| certified_at | timestamptz | 是 | 无 | 资源认证完成时点 |
| activated_at | timestamptz | 是 | 无 | 成为唯一活动包时点 |
| disabled_at | timestamptz | 是 | 无 | 停用时点 |
| revoked_at | timestamptz | 是 | 无 | 撤销时点 |
| status | text | 否 | `REGISTERED` | `REGISTERED\|VERIFIED\|CERTIFIED\|ACTIVE\|DISABLED\|REVOKED` |
| active_slot | smallint | 是 | 生成列 | `status='ACTIVE'` 时为 1，否则 NULL |

唯一约束固定为 `UNIQUE(model_code,model_version)`、`UNIQUE(package_digest)`、`UNIQUE(install_receipt_id)`、`UNIQUE(active_slot)`。`installed_root_ref` 只能是 `ep-install://ai-model/sha256/<package_digest lowerhex>`。状态边只有 `REGISTERED→VERIFIED→CERTIFIED→ACTIVE`、`ACTIVE→DISABLED`、`DISABLED→CERTIFIED`，任一非 REVOKED 状态可到 `REVOKED`；REVOKED 终态。认证收据事务先插 REGISTERED，再走合法边到 VERIFIED；CERTIFIED/ACTIVE 必须同时具有 `certification_report_ref/certification_report_digest`，ACTIVE 还必须具有 `verified_at/certified_at/activated_at`，PROD 环境拒绝 DEV 签名。报告 ref grammar、编译期根、owner/DACL/reparse 防护、最大 1 MiB strict report、`AI_RESOURCE_CERTIFICATION_V1` ECDSA P-256 sidecar、当前 release batch/产品 build/ai runtime/模型包/ABI/profile/server spec/load/gate 逐项绑定与每次激活复验，全部按 F-55 §3.7；`certified_at` 必须等于 report `finished_at`，不得只因 ref/digest 非空判绿。身份、摘要、签名、安装根/收据、提示版本、Runtime ABI、模型限制和 execution profile 全部不可更新；升级登记新行。

Runtime ABI v1 还须从签名 manifest 验证 `CANDLE/0.11.0 + HF_TOKENIZERS_RUST/0.23.1 + QWEN2 + GGUF_V3 + MOSTLY_Q4_0 + GREEDY_SCHEMA_DFA_V1`，不把这些值复制成第二套数据库列；完整闭集见 F-55 §3.2。首版只接受禁止 spanning 的单 CAB：exact CAB/package ≤2147483647 bytes，`model.gguf≤2000000000`，七项解包总计≤2130706432。安装根 regular-file roster 恰为固定 `package.cab` 加 exact 七项提取文件；`package.cab` digest 等于本行 package digest，ai-inferer 在激活前独立复核 CAB Authenticode/roster/inner CMS 与每个 archive entry↔extracted bytes，不只信收据。

Stage 14 的 `AiRuntimeReleaseFactsV1` 只是由当前签名 PE/Cargo.lock/vendor/SBOM/features 当场重算的 strict JCS gate input，不是第二张表或第二套签名证据；其 exact digest 必须进入最终已签 `AiResourceCertificationReportV1.gate_results[ai_runtime_release_facts]`，字段/大小/排序/profile/version 规则按 F-55 §3.7。

## 2. `platform_meta.mcp_connectors`

法人级可更新表，策略名固定 `rls_mcp_connectors_le`，执行 `ENABLE ROW LEVEL SECURITY` 与 `FORCE ROW LEVEL SECURITY`。

| 列名 | 类型 | 可空 | 默认 | 语义 |
|---|---|---:|---|---|
| id | uuid | 否 | 应用侧 UUIDv7 | 主键 |
| legal_entity_id | uuid | 否 | 无 | 法人；真实 FK 到 `platform_core.legal_entities(id)` |
| security_level | smallint | 否 | 30 | 连接器登记密级 |
| data_scope_tags | text[] | 否 | `'{}'` | 连接器可见范围标签 |
| row_version | bigint | 否 | 1 | 乐观锁 |
| created_at、updated_at | timestamptz | 否 | `now()` | 公共时间 |
| created_by、updated_by | uuid | 否 | 无 | 以 `(legal_entity_id,user_id)` 指向法人授权 |
| code | text | 否 | 无 | 法人内唯一，`^[a-z][a-z0-9._-]{0,63}$` |
| name | text | 否 | 无 | 1..200 |
| direction | text | 否 | 无 | `INBOUND\|OUTBOUND` |
| transport | text | 否 | 无 | `INBOUND_HTTPS\|REMOTE_STREAMABLE_HTTP\|LOCAL_SIGNED_STDIO\|LOCAL_WINDOWS_HYPERV_CONTAINER` |
| status | text | 否 | `REGISTERED` | `REGISTERED\|PENDING_APPROVAL\|ENABLED\|DISABLED\|REVOKED` |

shape CHECK 固定为 INBOUND 只能配 `INBOUND_HTTPS`，OUTBOUND 只能配其余三值。`UNIQUE(legal_entity_id,id)`、`UNIQUE(legal_entity_id,code)`、`UNIQUE(legal_entity_id,id,direction,transport)` 均建立为候选键。状态边固定 `REGISTERED→PENDING_APPROVAL→DISABLED`、`DISABLED→ENABLED`、`ENABLED→DISABLED`，任一非 REVOKED 状态可到 `REVOKED`；REVOKED 终态。ENABLE 必须在同一事务证明恰有一份同 connector 的 compatible ACTIVE manifest、credential probe 成功，且 F-56 同一 current signed grant 在本行 `legal_entity_id` scope 内含 `F55Mcp`、状态为 `Active|ExpiringSoon|GracePeriod`；并要求共同 `RG-LICENSE-MODULE-LIFECYCLE-GREEN` 与 `RG-MCP-CONFORMANCE-GREEN|RG-MCP-CONTAINMENT-GREEN` 对同一 run/deployment/build 均真实通过。历史 purchased、配置开关、模块码或人工证据不能替代 currently licensed；DISABLE/REVOKE 在 Restricted 中仍允许并保留全部版本与审计。

### 2.1 `platform_meta.mcp_transport_registry_versions`

部署级、append-only 的 MCP HTTPS transport registry 投影；它不带 `legal_entity_id`、不替代下节可并存的法人/connector manifest 版本。运行角色只可读，只有 core-server 的已验证 generation 激活事务可插入新版本并 supersede 旧 current slot。

| 列名 | 类型 | 可空 | 默认 | 语义 |
|---|---|---:|---|---|
| id | uuid | 否 | 应用侧 UUIDv7 | immutable version 主键 |
| deployment_id | uuid | 否 | 无 | 与 signed deployment/current generation 逐字相等 |
| generation | bigint | 否 | 无 | 正数；与 registry payload 和 current generation 相等 |
| registry_jcs | bytea | 否 | 无 | `McpTransportRegistryV1` 的 exact RFC 8785 JCS bytes；无 BOM/尾随换行，最多 16,384 bytes |
| registry_payload_sha256 | bytea | 否 | 无 | `registry_jcs` raw bytes SHA-256，32 bytes；digest 不嵌入自身 payload |
| source_generation_item_id | uuid | 否 | 无 | `MCP_TRANSPORT_REGISTRY` generation item 的 immutable ID |
| source_generation_payload_sha256 | bytea | 否 | 无 | 当前 signed generation payload digest，32 bytes |
| state | text | 否 | `ACTIVE` | `ACTIVE\|SUPERSEDED`；只有新 ACTIVE 插入可把原 ACTIVE 原子 supersede |
| active_slot | smallint | 是 | 生成列 | ACTIVE 时 1，否则 NULL |
| activated_at | timestamptz | 否 | 无 | verified current generation 激活时点 |
| superseded_at | timestamptz | 是 | 无 | SUPERSEDED 必填，ACTIVE 必空 |

`UNIQUE(deployment_id,generation)`、`UNIQUE(deployment_id,source_generation_item_id)`、`UNIQUE(deployment_id,active_slot)` 固定 current singleton。strict payload exact 为 `{schema_version:1,purpose:"EP-F57-MCP-TRANSPORT-REGISTRY-V1",protocol_version:"2026-07-28",listener_owner:"core-server",path:"/mcp",method:"POST",allowed_jsonrpc_methods:["resources/list","resources/read","resources/templates/list","server/discover","tools/call","tools/list"],generation}`；method array 按 UTF-8 bytes 排序唯一，generation 与行值/签名 generation exact-equal。repository 在插入前重算 canonical bytes/digest并验证 `GenerationItemV1.item_payload_sha256`；unknown/duplicate/missing field、自含 digest、错 generation、旧 generation 复活、第二 current 或把任一 connector manifest digest 当 transport digest 全部拒绝。`mcp_manifest_versions` 仍按 `(legal_entity_id,connector_id)` 各自拥有 ACTIVE slot，多 connector 并存是合法正例。

## 3. `platform_meta.mcp_manifest_versions`

法人级版本表，策略名固定 `rls_mcp_manifest_versions_le`，ENABLE、FORCE RLS。manifest 内容不可变，只有审批/活动状态及其证据可沿状态图前进。

| 列名 | 类型 | 可空 | 默认 | 语义 |
|---|---|---:|---|---|
| id | uuid | 否 | 应用侧 UUIDv7 | 主键 |
| legal_entity_id | uuid | 否 | 无 | 法人 |
| security_level | smallint | 否 | 30 | 继承/不得低于 connector |
| data_scope_tags | text[] | 否 | `'{}'` | 继承 connector 的规范集合 |
| row_version | bigint | 否 | 1 | 状态乐观锁 |
| created_at、updated_at | timestamptz | 否 | `now()` | 公共时间 |
| created_by、updated_by | uuid | 否 | 无 | 法人授权长 FK |
| connector_id | uuid | 否 | 无 | 父 connector |
| version_no | int | 否 | 无 | 从 1 递增 |
| protocol_version | text | 否 | `2026-07-28` | CHECK 只允许该版本 |
| manifest_json | jsonb | 否 | 无 | RFC 8785 JCS 前的强类型 manifest 内容 |
| manifest_digest | bytea | 否 | 无 | JCS SHA-256，固定 32 字节 |
| signature | bytea | 否 | 无 | canonical low-S IEEE-P1363 ECDSA P-256 签名，固定 64 字节 |
| signature_key_ref | text | 否 | 无 | `MCP_MANIFEST_V1` 发布 key ref，1..512 |
| signature_key_version | text | 否 | 无 | 验签 key version，1..128 |
| signer_subject | text | 否 | 无 | 已验证 signer，1..512 |
| remote_scheme | text | 是 | 无 | remote 固定 `https`，其他 transport 为空 |
| remote_host | text | 是 | 无 | 规范 ASCII host；无 userinfo/通配符 |
| remote_port | int | 是 | 无 | 1..65535 |
| remote_path | text | 是 | 无 | 规范绝对路径，不含 fragment |
| credential_ref | text | 是 | 无 | 可选 canonical `WindowsCredentialRef`；逐字复用配置参考第 5 节的 grammar、512-byte 引用界与 Win32 `TargetName` 映射，不另造 parser；不存 secret。CredentialBlob 固定 1..2560 bytes（`CRED_MAX_CREDENTIAL_BLOB_SIZE=5*512`），仅由 SCM 加载 profile 后的 `ep-integ\|ep-plugin` 服务 current token 经 F-55 维护协议写入；普通管理员 vault 不等价 |
| artifact_legal_entity_id | uuid | 是 | 无 | local transport 必填且等于本行法人 |
| artifact_attachment_version_id | uuid | 是 | 无 | local 签名包附件版本 |
| artifact_hash | bytea | 是 | 无 | local artifact SHA-256，固定 32 字节 |
| artifact_size_bytes | bigint | 是 | 无 | local artifact 正字节数 |
| artifact_eligible | boolean | 是 | 无 | local 必须 true，其他 transport 为空 |
| install_receipt_id | uuid | 是 | 无 | local 安装收据；remote/inbound 为空 |
| installed_root_ref | text | 是 | 无 | stdio exact `ep-install://mcp-stdio/<manifest-version-uuid>/sha256/<digest>`；Hyper-V exact `ep-install://mcp-wcow/sha256/<digest>` |
| installed_at | timestamptz | 是 | 无 | local 原子物化完成时点 |
| hcs_image_identity | text | 是 | 无 | Hyper-V container exact `hcswcow://sha256/<container digest>`；其余为空 |
| installed_root_sd_sha256 | bytea | 是 | 无 | local root self-relative security descriptor SHA-256；32 字节 |
| sandbox_profile_name | text | 是 | 无 | stdio deterministic AppContainer profile；其余为空 |
| sandbox_sid | text | 是 | 无 | stdio exact AppContainer SID；其余为空 |
| wfp_provider_guid | uuid | 是 | 无 | stdio 固定 WFP provider GUID；其余为空 |
| wfp_sublayer_guid | uuid | 是 | 无 | stdio 固定 WFP sublayer GUID；其余为空 |
| wfp_connect_v4_filter_key | uuid | 是 | 无 | stdio `FWPM_LAYER_ALE_AUTH_CONNECT_V4` deterministic persistent block key；其余为空 |
| wfp_connect_v6_filter_key | uuid | 是 | 无 | stdio `FWPM_LAYER_ALE_AUTH_CONNECT_V6` deterministic persistent block key；其余为空 |
| wfp_recv_accept_v4_filter_key | uuid | 是 | 无 | stdio `FWPM_LAYER_ALE_AUTH_RECV_ACCEPT_V4` deterministic persistent block key；其余为空 |
| wfp_recv_accept_v6_filter_key | uuid | 是 | 无 | stdio `FWPM_LAYER_ALE_AUTH_RECV_ACCEPT_V6` deterministic persistent block key；其余为空 |
| wfp_resource_assignment_v4_filter_key | uuid | 是 | 无 | stdio `FWPM_LAYER_ALE_RESOURCE_ASSIGNMENT_V4` deterministic persistent block key；其余为空 |
| wfp_resource_assignment_v6_filter_key | uuid | 是 | 无 | stdio `FWPM_LAYER_ALE_RESOURCE_ASSIGNMENT_V6` deterministic persistent block key；其余为空 |
| status | text | 否 | `DRAFT` | `DRAFT\|PENDING_APPROVAL\|APPROVED\|ACTIVE\|SUPERSEDED\|REJECTED\|REVOKED` |
| approval_ref | uuid | 是 | 无 | 审批实例引用 |
| approved_by、rejected_by | uuid | 是 | 无 | 法人授权用户证据 |
| approved_at、rejected_at | timestamptz | 是 | 无 | 审批结论时点 |
| rejected_reason | text | 是 | 无 | REJECTED 时 1..2000 |
| activated_at、superseded_at | timestamptz | 是 | 无 | 活动版本时点 |
| active_slot | smallint | 是 | 生成列 | ACTIVE 时 1，否则 NULL |

候选键与 FK：

- `UNIQUE(legal_entity_id,id)`、`UNIQUE(legal_entity_id,connector_id,id)`、`UNIQUE(legal_entity_id,connector_id,version_no)`、`UNIQUE(legal_entity_id,connector_id,active_slot)`；
- `manifest_digest` 不唯一，允许受控回退复制历史 canonical 内容到新 version；`install_receipt_id` 非空时本表唯一，并由下述 handler 锁与 AI 表形成跨表全局唯一；
- `(legal_entity_id,connector_id)` 真实复合 FK 到 connectors；
- local artifact 以 `(artifact_legal_entity_id,artifact_attachment_version_id)` 真实复合 FK 到 `platform_file.attachment_versions(legal_entity_id,id) ON DELETE RESTRICT`；
- approved/rejected 用户以 `(legal_entity_id,user_id)` 指向 `platform_authz.user_legal_entity_grants`；两结论互斥，申请人与批准人不得相同。

transport shape：

- INBOUND：remote、credential、artifact 与安装收据四组列全部为空；
- REMOTE：remote 四列必填，artifact/安装收据为空；manifest 内 `remote.tls_spki_sha256` 必须非空且通过 trigger 与 JCS 内容核对；credential 为空或合法 `wincred://`；
- LOCAL 两种从 DRAFT 起 remote 四列为空、artifact 五列完整且 eligible=true。REMOTE credential 可空或合法 `wincred://`，stdio credential 可空或 `LOCAL_SECRET_PIPE_UTF8`，Hyper-V container 首版 credential 必须为空。DRAFT/PENDING_APPROVAL 的整组 materialization 列全空；APPROVED 可全空等待安装，且只有 APPROVED 行允许一次 guarded CAS 原子补全。两种 local 都写 `install_receipt_id/installed_root_ref/installed_at/installed_root_sd_sha256`；stdio 的 hcs identity 为空且十个 sandbox/WFP 字段全有，Hyper-V container 的 hcs identity 必填并逐字等于 `hcswcow://sha256/<manifest container_image_digest lowerhex>`、十个 sandbox/WFP 字段全空。stdio root 的 manifest-version UUID 必须等于本行 id，同 CAB 跨版本/connector 复用仍物理复制到独立 root，禁止共享文件对象。非 local 的 materialization 列全空。

状态边只有 `DRAFT→PENDING_APPROVAL`、`PENDING_APPROVAL→APPROVED|REJECTED`、`APPROVED→ACTIVE`、`ACTIVE→SUPERSEDED`，任一非终态可到 REVOKED；REJECTED/SUPERSEDED/REVOKED 终态，旧 SUPERSEDED 永不重开，回退复制到更高 DRAFT 并以当前 key 重签。APPROVED 及以后必须有批准快照，REJECTED 只带拒绝结论；LOCAL 在 APPROVED→ACTIVE 前必须已有匹配安装收据。安装 CAS 必须带旧 row_version，从整组全空到 transport-complete 一次完成，不得同时改变 manifest/artifact/signature/审批字段，不得清空或二次改写；REVOKED 可保留撤销前已有完整组或全空但不得互换。回退新 DRAFT 不复制旧 receipt/root/HCS/sandbox facts。ACTIVE 时 connector 不能 REVOKED，manifest 的 direction/transport/code 必须与父 connector 一致。manifest、digest、signature/key/signer、protocol、origin/credential/artifact 及 version_no 全不可更新；安装组只有上述一次 CAS 例外，内容变化只能新增更高版本。

`ops.signed_artifact.install_receipt.v1` 是 AI/MCP 两表唯一生产写入口。事务先执行 `pg_advisory_xact_lock(hashtextextended(lower(receipt_id::text),4995704681966667073))`，再同时查询 `platform_ops.ai_model_packages.install_receipt_id` 与本表；两边均无才写，恰有同 artifact/同字段一条则返回原对象，跨 kind、字段不同或异常双命中 fail closed。生产角色不可旁路写 receipt/materialization 列；hash 碰撞只增加串行，不影响正确性。两种 local 也只接受禁止 spanning 的单 CAB，exact CAB≤2147483647；stdio 解包 files 总计≤2000000000，Hyper-V 唯一 `image-layout.tar≤2000000000`（OCI expanded layers 仍≤5368709120）。每个 root 保留固定 `package.cab + exact extracted entries` 封闭 roster，plugin-host 调用前独立复验 package digest、inner CMS 与 entry↔extracted bytes。

gateway/plugin-host 的 manifest 公钥不进数据库。唯一离线验证根为 F-55 §4.2 的 `McpManifestTrustBundleV1`：strict JCS 字段 `schema_version=1,bundle_id,release_batch_id,generated_at,entries[]`，每项只含 purpose/key ref/version/subject/P-256 DER SPKI bytes+digest/`ACTIVE|RETIRED|REVOKED`；1..256 项按 key ref/version 排序去重，bundle 上限 1 MiB。固定本机目录、SYSTEM owner/DACL、相邻 detached CMS、离线停服原子替换与 gate-closed 吊销顺序均按 F-55，数据库/API 不保存 SPKI 或私钥，也不提供热更新入口。

## 4. `platform_authz.mcp_human_grants`

法人级短期授权表，策略名固定 `rls_mcp_human_grants_le`，ENABLE、FORCE RLS。token 明文只在 issue 成功响应出现一次；数据库只保存 SHA-256。

| 列名 | 类型 | 可空 | 默认 | 语义 |
|---|---|---:|---|---|
| id | uuid | 否 | 应用侧 UUIDv7 | grant id |
| legal_entity_id | uuid | 否 | 无 | 来源会话活动法人 |
| security_level | smallint | 否 | 无 | 取签发时用户 clearance，10/20/30/40 |
| data_scope_tags | text[] | 否 | `'{}'` | 签发时规范范围标签快照，只用于摘要/审计；调用仍逐次重建 |
| row_version | bigint | 否 | 1 | 撤销与次数条件更新乐观锁 |
| created_at、updated_at | timestamptz | 否 | `now()` | 公共时间 |
| created_by、updated_by | uuid | 否 | 无 | 必须等于 user_id 或具备安全管理员权限 |
| connector_id | uuid | 否 | 无 | INBOUND connector |
| manifest_version_id | uuid | 否 | 无 | 当时 ACTIVE manifest |
| user_id | uuid | 否 | 无 | 签发人/被授权人，首版相同 |
| source_session_id | uuid | 否 | 无 | 来源活动会话 |
| source_device_id | uuid | 否 | 无 | 来源 `user_devices.id`，不是可自填 device 文本 |
| token_hash | bytea | 否 | 无 | `SHA-256(完整 50-byte ASCII token)`，固定 32 字节且全库唯一 |
| scope_digest | bytea | 否 | 无 | F-55 `EP-MCP-GRANT-SCOPE-V1` exact preimage 摘要，32 字节 |
| allowed_tool_names | text[] | 否 | `'{}'` | 排序、去重、逐项属于 manifest tools |
| allowed_resource_uri_templates | text[] | 否 | `'{}'` | 排序、去重、逐项属于 manifest resources |
| max_calls | int | 否 | 100 | 1..100 |
| used_calls | int | 否 | 0 | 0..max_calls；proof 通过且被受理即原子加一，下游失败不退还 |
| last_proof_counter | bigint | 否 | 0 | 无前导零 u64 逻辑范围；数据库非负且始终等于 used_calls |
| issued_at | timestamptz | 否 | `now()` | 签发时点 |
| expires_at | timestamptz | 否 | 无 | `issued_at < expires_at <= issued_at + 600 seconds` |
| state | text | 否 | `ACTIVE` | `ACTIVE\|CONSUMED\|REVOKED\|EXPIRED` |
| revoked_at | timestamptz | 是 | 无 | 仅 REVOKED 必填 |

两组允许项不能同时为空。token 的唯一 wire grammar 是 `epmcp1.` + 恰好 32 个 CSPRNG bytes 的 43-char RFC 4648 §5 base64url-no-pad，全串恰好 50 ASCII bytes；解析必须满足 `\Aepmcp1\.[A-Za-z0-9_-]{43}\z`、解码恰好 32 bytes，并且无 padding 重编码后逐 byte 等于输入。hash 与 DPoP 均使用完整 50-byte ASCII token，不是解码后的随机 bytes。`UNIQUE(legal_entity_id,id)` 与 `UNIQUE(token_hash)` 固定。真实父链固定为：

- `(legal_entity_id,user_id)` → `user_legal_entity_grants(legal_entity_id,user_id)`；
- `(legal_entity_id,connector_id,manifest_version_id)` → `mcp_manifest_versions(legal_entity_id,connector_id,id)`；
- `(legal_entity_id,user_id,source_device_id,source_session_id)` → `sessions(active_legal_entity_id,user_id,user_device_row_id,id)`；后者候选键由本迁移同批建立；
- `(user_id,source_device_id)` → `user_devices(user_id,id)`；候选键同批建立。

签发时父 session/device/account/法人授权、connector 与 manifest 必须均活动，且 connector 必须 INBOUND/INBOUND_HTTPS。每次 proof 通过后的受理用单条条件 UPDATE 要求 `last_proof_counter+1=counter`、ACTIVE/未过期/未超次数，同时令 `last_proof_counter=counter,used_calls=used_calls+1,state=CASE WHEN used_calls+1=max_calls THEN 'CONSUMED' ELSE 'ACTIVE' END`；达到 max_calls 的该次受理仍可继续下游调用，之后 grant 终态。`ACTIVE→CONSUMED|REVOKED|EXPIRED` 是唯一路径，三者终态；过期扫描只把仍 ACTIVE 的过期行改 EXPIRED。登出、账号/设备停用、法人授权失效、manifest 失活或主动撤销立即令 grant 在逐次校验中无效，并由同事务/异步收敛为相应终态；调用不能仅依赖快照列放行。

`scope_digest=SHA-256(ASCII("EP-MCP-GRANT-SCOPE-V1\0") || JCS(scope))`；scope strict 字段恰为 `schema_version=1,legal_entity_id,user_id,source_session_id,source_device_row_id,connector_id,manifest_version_id,manifest_digest,security_level,data_scope_tags,allowed_tool_names,allowed_resource_uri_templates`。UUID 为小写连字符文本、digest 为 lowerhex，数组按 UTF-8 bytes 排序去重。数据库列名 `source_device_id` 存的是 `user_devices.id` 行 UUID，对应 preimage 的 `source_device_row_id`；HTTP/DPoP 的 `Device-Id` 则是 join 得到的外部 text，不得互换。grant issue 禁止 `Idempotency-Key` 且不写通用 response cache；携带该头使用既有 `PLATFORM.REQUEST.INVALID_PAYLOAD` 并给字段级原因，不新增同义错误码。明文 token 只返回一次，丢响应后撤销/过期再签发。

## 5. `platform_ops.deployment_records` F-55 追加列

以下十四列由 `V20261024090700__platform_ops_add_deployment_carrier.sql` 追加到每个受控版本行；既有 `IMMUTABLE_COLUMNS` 登记仍只有 `superseded_at` 可变，因此 carrier 事实通过新增完整 revision 变更，不原地修改。为兼容已有行，十四列物理上全部 nullable、全部无默认；数据库 shape 只允许十四列全空 legacy 或完整形状，不允许部分为空。

| 列名 | 类型 | 可空 | 默认 | 语义 |
|---|---|---:|---|---|
| carrier_kind | text | 是 | 无 | 完整行：`CUSTOMER_CONTROLLED_PHYSICAL\|CUSTOMER_CONTROLLED_DOMESTIC_IAAS_VM` |
| provider_code | text | 是 | 无 | 完整行：物理机固定 `CUSTOMER_CONTROLLED`；VM 为批准 provider code |
| region_code | text | 是 | 无 | 完整行：物理机为 site code；VM 为 provider region code |
| residency_jurisdiction_code | text | 是 | 无 | 完整行：合同数据驻留法域码 |
| region_jurisdiction_code | text | 是 | 无 | 完整行：实际 site/region 法域码，必须等于驻留法域 |
| vtpm_present | boolean | 是 | 无 | 完整行：VM=true；物理机=false |
| vtpm_attestation_ref | text | 是 | 无 | VM 必填；物理机必须为空 |
| backup_failure_domain_code | text | 是 | 无 | 完整行：离站副本故障域规范码 |
| backup_failure_domain_evidence_ref | text | 是 | 无 | 完整行：三维隔离证据引用 |
| carrier_attestation_ref | text | 是 | 无 | 完整行：客户控制权、管理员责任与禁用托管组件的签字证据 |
| carrier_policy_ref | text | 是 | 无 | 完整行：签名 `CarrierPolicyV1` 只读引用 |
| carrier_policy_digest | bytea | 是 | 无 | 完整行：policy exact JCS SHA-256，32 字节 |
| carrier_evidence_ref | text | 是 | 无 | 完整行：签名 `CarrierEvidenceV1` 只读引用 |
| carrier_evidence_digest | bytea | 是 | 无 | 完整行：evidence exact JCS SHA-256，32 字节 |

完整行的 `provider_code,region_code,backup_failure_domain_code` 长度 1..64 ASCII bytes，必须匹配 `[A-Z0-9][A-Z0-9._-]{0,63}`；`residency_jurisdiction_code,region_jurisdiction_code` 长度 2..16 ASCII bytes，必须匹配 `[A-Z0-9][A-Z0-9-]{1,15}`。十四列中恰有五个 DB ref：`vtpm_attestation_ref,backup_failure_domain_evidence_ref,carrier_attestation_ref,carrier_policy_ref,carrier_evidence_ref`，分别映射 kind `vtpm|backup-failure-domain|customer-control|policy|evidence`；它们都是 1..512 ASCII bytes，只允许 `ep-evidence://carrier/<lowercase-deployment-uuid>/<kind>/sha256/<64-lowerhex>`。统一 ref parser 另支持第六个 kind `nested-virtualization`，但它只出现在 `CarrierEvidenceV1.nested_virtualization_evidence_ref` child DTO，绝不是 deployment_records 第十五列。不得用通用 1..1024 ref 或允许 colon 的通用 code CHECK 代替这些逐列规则。两个 DB digest 固定 32 字节。拒绝任意路径/URL/UNC/device/ADS/reparse，编译期根、owner/DACL 与 digest readback 逐字按 F-55 §6.2。物理机 shape：`provider_code=CUSTOMER_CONTROLLED`、`vtpm_present=false/vtpm_attestation_ref=NULL`；VM shape：provider 不得为该固定值、vTPM=true/ref 非空。两种 carrier 都要求 TPM version 2.0 证据、jurisdiction 相等、policy/evidence/failure-domain/customer-control 引用和摘要完整。`ck_deployment_records_f55_carrier_shape NOT VALID` 允许且只允许“十四列全 NULL”或上述完整形状；NOT VALID 只避免扫描历史，新增/更新行仍受 CHECK。受控 revision writer 与 `BEFORE INSERT` trigger 拒绝新全空行；current selector 与所有 F-55/Stage 14 gate 只接受完整形状。历史全空行只可写 `superseded_at`，不得原地补列或再次成为 current。

签名 `CarrierPolicyV1` 与 `CarrierEvidenceV1` 的 exact JCS 字段、签名 purpose、managed component 七项 false、两种 carrier 的 TPM 2.0、两名 `SECURITY|OPERATIONS` 不同主体/不同 key 授权签名，以及备份 `SITE_OR_REGION|ACCOUNT_OR_CREDENTIAL_DOMAIN|MEDIA_OR_IMMUTABILITY_DOMAIN` 三维证据见 F-55 §6.2。全部 policy/evidence/probe JCS 最大 1048576 bytes、时间 UTC 秒精度；CarrierEvidence 必须带同一 `stage14_run_id,stage14_started_at,stage14_completed_at`，窗口最长 8 小时。IaaS policy item 还必须有 `vtpm_attestation_profile="TPM2_QUOTE_SHA256_V1"` 与按字节排序的 `approved_vm_skus[{vm_sku,nested_virtualization_supported}]`；evidence 带 exact `vm_sku,nested_virtualization_supported,nested_virtualization_evidence_ref/digest`，true 时验证带同 run id 的 `NestedVirtualizationEvidenceV1`，false 时两证据字段为空且 Hyper-V local transport 不启用。`VtpmAttestationEvidenceV1`、`CustomerControlEvidenceV1`、`CarrierFactProbeResultV1` 的 exact fields/canonical base64/TPM quote/PCR/digest 前像按 F-55 §6.2，不允许 provider-specific parser 分支。

`CarrierEvidenceV1` 还必须含非空 `backup_failure_domain_evidence_digest`。`backup_failure_domain_evidence_ref` 解析出的末段 digest 必须逐字等于它，并验证 strict `BackupFailureDomainEvidenceV1{schema_version=1,stage14_run_id,deployment_id,carrier_kind,backup_failure_domain_code,observed_at,entries[]}`；bundle 最大 1 MiB，entries 恰为三维各一项且按 dimension bytes 排序，每项恰有 `dimension,probe_evidence,evidence_digest`。`probe_evidence` 是 F-55 exact `BackupDimensionProbeEvidenceV1`，digest 以 `EP-CARRIER-BACKUP-DIMENSION-V1\0` domain-separated preimage 重算；三项摘要投影与 CarrierEvidence 的 `backup_separation_evidence` 逐项相等。DB 仍只需现有一列 ref，不新增第十五列。完整 CarrierEvidence 的部署 KMS sidecar 固定为 `CarrierEvidenceSignatureV1{schema_version=1,purpose="CARRIER_EVIDENCE_V1",deployment_id,evidence_digest,key_ref,key_version,signer_subject,signature_p1363_b64url}`，字段是 canonical base64url-no-pad 且解码恰 64-byte low-S P1363，preimage/key-state 逐字按 F-55。policy 使用 `.p7s`，evidence 使用 `.sig.jcs`；vtpm/nested/backup/customer-control child 不另造签名，只由其 exact JCS digest 被完整签名 evidence 绑定。

应用与 Stage 14 唯一调用 `validate_deployment_carrier(record,&policy,&evidence,&dyn CarrierFactProbe)`，逐项核 policy/evidence/两名授权/部署 KMS 签名、ref/digest、deployment/policy/bundle 绑定、DB 十四列、legacy/new/current guard、当前 machine/provider/region/SKU/TPM/nested/hypervisor/isolation probe 与三维隔离。schema 不含有效期；policy 由当前签名包/key 吊销状态决定，evidence 必须在当前 Stage 14 运行窗口生成。任一不一致 fail closed。数据库 CHECK 不从 ref 自由文本猜测策略或证据结论。

## 6. F-55 权限项、范围锚与审计形状

`V20261024090300__platform_authz_create_mcp_human_grants.sql` 同批幂等 seed 以下五条 `permission_items`，id 固定为 `...0310` 至 `...0314`：

| id 尾号 | code / object_type | module | function_point | allowed_actions |
|---:|---|---|---|---|
| 0310 | `reporting.ai_analysis.compose` | reporting | 本地分析 AI 方案生成 | `[VIEW]` |
| 0311 | `reporting.ai_analysis.execute` | reporting | 本地分析 AI 查询执行 | `[VIEW]` |
| 0312 | `platform.mcp.connector.manage` | platform | MCP 连接器管理 | `[VIEW,UPDATE]` |
| 0313 | `platform.mcp.grant.issue` | platform | MCP 人类授权签发 | `[CREATE]` |
| 0314 | `platform.admin.ai_model.view` | platform | AI 模型包查看 | `[VIEW]` |

每行 object_type 必须逐字等于 code。对应 `object_scope_bindings` id `...0504` 至 `...0508`：两个 AI operation 分别指 `reporting.datasets(min_security_level)`；connector manage 与 grant issue 分别以自己的 object_type 指 `platform_meta.mcp_connectors(security_level)`，grant issue 判定 object id 取请求 connector_id；AI model view 指 `platform_ops.ai_model_packages(security_level)`；四个 owner/dept/project/customer 锚除已说明者均 NULL。迁移不写任何 `role_permission_grants`，由签名 authz 配置显式授予。冲突行若字段不完全相等必须 RAISE，不能静默覆盖。

AI 审计也不新建表。编译期 `crates/platform/audit/src/object_registry.rs` 登记 object-level/id-required 的 `reporting.ai.query_turn`，actions 恰为 `AI_QUERY_PLAN_COMPOSED|AI_QUERY_PLAN_EXECUTION_ATTEMPTED`。可签发计划在 token 前写 COMPOSED；合法 token/current facts 在查询前写 EXECUTION_ATTEMPTED，失败则不签 token 或零 SQL。两者 object id 是同一 turn UUID，before=NULL，strict masked after 与空值规则逐字按 F-55 §3.8；问题/计划仅 hash，结果、SQL、literal、token、prompt/模型输出正文不入审计。

MCP 调用审计不新建表，使用 `platform_audit.audit_events`：identity 可解析后的 dispatch 前拒绝恰一条 `MCP_CALL_COMPLETION`；已 dispatch 恰有先独立提交的 `MCP_CALL_ATTEMPT` 和终态 completion。namespace 固定 `3f9b8e44-78a5-5ff0-8fc9-6ad25a8a5c55`，event id 为 F-55 §4.7 UUIDv5 例外；`object_type=platform.mcp.invocation`、object id 为 UUIDv7 invocation，并在同一个 compile-time object registry 登记 exact 两 action。strict `after` 的 phase-aware 空值矩阵逐字以 F-55 §4.7 为准：name/URI 未规范解码时 name hash、binding/schema 为空；已解码但 binding 不存在或不可见时只留 decoded hash，binding/schema 仍为空且 input fields 为 `[]`；只有解析成功的 named binding 才留 binding digest，tools/call 同时留 request schema，resources/read 的 request schema 仍为空；四个 discover/list 方法无 binding/schema。无 ATTEMPT completion 只能 REJECTED；connector terminal bytes 未取得时 response hash/bytes 为 NULL/0，取得后即使 schema-invalid 也记 exact hash/实际 bytes，output fields 只在验证通过时非空。stable code、outcome 与 response schema 的其余组合亦按该真值表。raw tool name、expanded URI、对象 id、header/payload/extension/secret 永不进入审计；不存在与存在但不可见必须同形。identity 尚不可解析的 transport 拒绝只写一条脱敏 `MCP_TRANSPORT_REJECTION` 部署安全日志。

identity 后先在 caller 专用 1 GiB/1024-slot 目录预留一个 exact 1 MiB completion slot；再取 permits/rate/counter。九项 `McpExchangeChunkStreamFrameV1` 的 RequestEnd COMPLETE 只表示 receiver 已验证且保留 rate，caller 成功提交 ATTEMPT 后才发送 `DispatchAuthorized`，receiver 在此之前零外呼/零本地启动；wire 无 wall-clock deadline 字段，30 秒只用 receiver 单调时钟。completion 的 `.reserve→.tmp→.ready`、1048507-byte JCS 界、`.tmp/.corrupt` 恢复与 UNKNOWN 规则按 F-55 §4.7；任何 slot/审计确认失败使用 `MCP.AUDIT.UNAVAILABLE` 且禁止自动重试，reconciler 永不猜测或重放外部效果。

## 7. 能力矩阵、ClientKind 与审计持久化变化

> **F-57 current override：**本节原 F-55 `server_admin`/90 格口径已由 `RULING-UX-01` 取代，不得实施。

- `platform_meta.client_capabilities` 保持四个 Workbench client × 18 域的 72 格；Control Center 属权威 `ops` 管理面且不另加第五码矩阵，`Mcp` 也不入矩阵。
- Rust `ClientKind` exact-set 为 `win|mac|ios|android|portal|ops|mcp` 七值；Control Center 固定 `ops`，F-55 `server_admin` 不存在。
- `platform_core.user_devices.client` 保持 `win|mac|ios|android|portal|ops` 六值；MCP 复用 grant 来源设备，不写 `mcp`。
- `platform_audit.audit_events.client` exact-set 为 `win|mac|ios|android|portal|ops|mcp|system` 八值；metrics 的 `client` 标签取前七个 ClientKind 值。
- foundation enum、入口 middleware、设备 CHECK、audit CHECK/adapter、metrics 与 archcheck 必须在 Task 14 同批 exact-set 变更；发现 `server_admin`、允许外部自填 `mcp` 或任一层数量/字面量漂移都失败。

## 8. 迁移、索引与退出判据

逐表迁移版本/路径只读 `docs/migration-catalog.md` 的 F-55 九行。**五张**表及 carrier 追加列必须满足（第五张 `mcp_transport_registry_versions` 的 FK/RLS/digest 判据同适用，F-65）：
> **F-65 补**：第五张表不在 F-55 九行内，其迁移落点按 F-57 master §4.1 的两条 `platform_meta` 预留（`20261025091600`、`20261025091800`）承接——**此为按迁移名与 legacy seed `:279` 替换关系的推断，非逐字出处，G0 生成时须核**；「只读 F-55 九行」对第五张取不到，须并读上述两条。

1. 所有具名 FK、RLS、FORCE RLS、候选键、状态边与不可变列由数据库元数据测试覆盖；
2. 所有 digest/token hash 固定 32 字节；所有数组规范排序且无重复；
3. manifest JCS digest、签名与 transport shape 在写入、启用和每次调用三处复验；
4. grant 次数并发测试证明不超过 max_calls，失效父链零调用；
5. ai_model_packages 登记进 unpoliced registry，理由与本文件逐字一致；
6. fresh database 全量迁移、历史 checksum 零变化，逻辑回退只禁用路由并保留全部行。
