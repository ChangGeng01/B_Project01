# ADR-0023 F-57 Provider Manifest 与 Resource Grant

- 状态：已接受
- 出处：F-57 可替换 provider、MCP、能力包权限、数据驻留与受控 carrier 契约
- 关系：补全 `CapabilityPackageManifestV1`、`ProviderManifestV1`、`ResourceGrantV1` 与 invocation/runtime authorization 的唯一组合规则；取代任何未定义的 `BC-2` 容器前置条件

## 背景

F-57 允许同一能力连接本地、自建、客户已有、私有云或经批准的外部提供者，也允许能力包携带 WASM、签名 Windows worker、连接器和满足宿主证据后的 Hyper-V-isolated Windows container。签名能力包只定义包可请求的权限上限，不能单独证明某一 provider、某一次调用或当前用户有权使用这些资源。

若 `ProviderManifestV1`、`ResourceGrantV1`、canonical encoding、generation 和撤销语义未冻结，不同 carrier 可能各自解释字段、网络、文件、密钥和资源权限；provider 也可能利用包的宽权限绕过调用级最小授权。XML 若通过内容嗅探或通用解析器隐式启用，还会绕过显式 codec、schema、大小和外部实体边界。

## 决定

### 一、唯一签名和 canonical encoding

`ProviderManifestV1` 与 `ResourceGrantV1` 都使用严格 JSON，解析时拒绝重复 key、未知字段、非 UTF-8、非有限数字、前导零和非 canonical 标识。签名载荷采用 RFC 8785 JCS；UUID 使用小写连字符形式，时间使用 UTC RFC 3339，digest 使用小写 `sha256:` 加 64 位十六进制，二进制使用无 padding base64url；表示集合的数组必须按各节给出的稳定 key 排序且去重。

两者都复用现有 `SignedBusinessArtifactV1<T>` 的 detached CMS、signer token、离线 chain/full-CRL、ECDSA-P256/RSA-PSS 闭集和 purpose separation，不新建 raw-signature、自由算法字符串或系统 root-store fallback。签名对象的 digest 为 `SHA-256(JCS(payload))`；签名和 digest 字段位于外层，不能进入自身 payload。

### 二、`ProviderManifestV1` exact schema

payload 只能包含下列非空字段；`xml_policy` 是唯一条件字段，不使用时必须缺失而不是 `null`：

| 字段 | 类型与约束 |
|---|---|
| `schema_version` | 常量 `1` |
| `purpose` | 常量 `EP-F57-PROVIDER-MANIFEST-V1`；与共享 trait/policy 三方逐字相等 |
| `provider_id` | UUID；跨版本稳定 |
| `provider_code` | 小写 ASCII，正则 `[a-z][a-z0-9_.-]{2,127}` |
| `provider_version` | canonical SemVer |
| `package_ref` | `{package_id, package_version, package_manifest_digest}`；内置 provider 使用签名 release package 的等价 ref，不允许空 owner |
| `carrier` | 本 ADR 第三节的闭合 carrier 变体 |
| `codecs` | 非空、排序去重的 `ProviderCodecV1` 闭集 |
| `contracts` | 非空数组；每项精确含 `operation_code`、`direction`=`INPUT|OUTPUT|RECONCILE`、`media_type`、`schema_id`、`schema_version`、`schema_digest`、`maximum_bytes` |
| `provided_capabilities` | 非空、按 canonical capability id 排序去重 |
| `operation_bindings` | 非空、按 `(capability,operation_code)` 排序唯一的 `ProviderOperationBindingV1`；把 capability/effect 唯一绑定到 contracts 与 reconcile operation |
| `permission_ceiling` | 本 ADR 第四节的 `PermissionCeilingV1` |
| `data_policy` | `{processing_location, retention_mode, maximum_retention_seconds, provider_logging, model_training}`；`processing_location` 必须是下节 `ProcessingLocationEvidenceV1`，外部训练在最高安全档固定 `FORBIDDEN` |
| `lifecycle` | `{health_operation,health_binding,reconcile_operation,drain_timeout_ms,invocation_timeout_ms,disable_mode,unknown_outcome_mode}`；health 使用专门只读 lifecycle binding，不借业务 capability；不可逆外部效果的 unknown 固定进入 reconciliation |
| `conformance_refs` | 非空、排序去重的 `{suite_id, suite_version, evidence_digest}` |
| `generation` | 正整数，必须等于待激活 signed generation |
| `issued_at` | `TrustedUtc` |
| `not_before` | `TrustedUtc`，不得晚于 `expires_at` |
| `expires_at` | `TrustedUtc`；永久 manifest 也必须由站点策略给出有限复核期 |
| `xml_policy` | 仅当 `codecs` 含 `XML_XSD_V1` 时出现，结构见第八节 |

`contracts` 按 `(operation_code, direction)` 排序且组合唯一；`provided_capabilities` 必须是 package ref 所声明能力的子集。`ProviderOperationBindingV1` exact fields 为 `{capability,operation_code,effect_kind,input_contract_digest,output_contract_digest,reconcile_operation_code,reconcile_contract_digest}`；`effect_kind` 闭集为 `READ_ONLY|IDEMPOTENT_WRITE|EXTERNAL_EFFECT`。每个 provided capability 至少一行 binding，不能出现未 provided 的 capability；同一 capability+operation 只能一行。`operation_code` 在 contracts 中必须恰有 INPUT 与 OUTPUT 两行且 digest 逐字命中；READ_ONLY/IDEMPOTENT_WRITE 的两个 reconcile 字段都必须为 JSON null，EXTERNAL_EFFECT 的两个字段都必须非空并 exact-join lifecycle.reconcile_operation 的 RECONCILE contract。一个 invocation 只能选择一行完整 binding，不能按相同 schema digest、媒体类型或 capability 猜 operation。法人、数据分类和资源上限的唯一声明位置分别是 `permission_ceiling.legal_entities`、`permission_ceiling.data_classes` 和 `permission_ceiling.resource_ceiling`；`data_policy` 或 manifest 顶层出现旧字段 `permitted_legal_entities`、`maximum_classification` 或 `resource_ceiling` 一律按 unknown field 拒绝，不做 exact-equal、min 或隐式覆盖。manifest 不能包含数据库连接串、明文 secret、任意本地路径、shell/command template、代理覆盖、动态库路径或自由代码。

### 2.1 `ProcessingLocationEvidenceV1` exact schema

`data_policy.processing_location` 不是自由字符串、地区标签或外部附件引用。它是 strict object，字段恰为：

| 字段 | 类型与约束 |
|---|---|
| `schema_version` | 常量 `1` |
| `residency_profile` | 常量 `CN_MAINLAND_ONLY_V1` |
| `country_or_region` | strict object `{country_code, region_code, location_kind}`；`country_code` 固定 `CN`，`region_code` 为客户部署清单或 provider 证据中的 canonical lower-ASCII region/site code，正则 `[a-z0-9][a-z0-9.-]{0,63}`，`location_kind` 只允许 `CUSTOMER_SITE|CUSTOMER_IAAS_REGION|EXTERNAL_PROVIDER_REGION` |
| `carrier_kind` | 必须逐字等于本 manifest `carrier` 的标签 |
| `carrier_binding_digest` | `sha256:` lowerhex；输入为本 manifest exact `carrier` object 的 JCS，用于把地点证据绑定到具体本地进程、IaaS 实例或远端 origin |
| `endpoint_evidence` | 排序去重的 `ProcessingEndpointEvidenceV1[]`；无网络 carrier 必须为空，存在网络权限或 `REMOTE_HTTPS` 时不得为空 |
| `data_classes` | 非空、排序去重，且必须与 `permission_ceiling.data_classes` exact-equal；不能用 `ALL`、`CUSTOMER_DATA` 等通配 |
| `evidence_id` | UUID |
| `evidence_digest` | `sha256:` lowerhex；绑定本次地点、运营方、网络解析/路由和实测材料 |
| `verification_authority_subject` | `spki-sha256:<64 lowerhex>`；必须属于签名 DeploymentManifest 的驻留验证者 roster |
| `verified_at` | `TrustedUtc`，不得晚于 manifest `issued_at` |
| `expires_at` | `TrustedUtc`，必须晚于 `verified_at` 且不得早于 manifest `expires_at`；地点证据失效时 manifest 同步失效，不能只刷新一个数据库字段 |

`ProcessingEndpointEvidenceV1` 字段恰为 `{origin,port,spki_sha256,country_code,region_code,operator_code,redirect_policy,proxy_policy,evidence_digest}`。`origin` 是无 path/query/fragment 的 canonical HTTPS origin，`port` 为 1..65535，`spki_sha256` 与 `evidence_digest` 使用 `sha256:` lowerhex，`country_code` 固定 `CN`，`region_code` 使用上表同一 grammar，`operator_code` 为 canonical provider/operator code，`redirect_policy` 与 `proxy_policy` 均固定 `DENY`。数组按 `(origin,port,spki_sha256)` 排序且组合唯一。

`endpoint_evidence` 必须与 `permission_ceiling.network` exact-join：每个允许接收客户或可关联客户数据的 origin/port/SPKI 恰一行，`REMOTE_HTTPS` carrier 自身 origin 也必须恰一行；缺失、额外、DNS/HTTP 重定向、系统代理、解析后落到非 `CN` 网络证据、`UNKNOWN` 地区或过期证据均使 effective permission 为空。`carrier_binding_digest` 对物理/IaaS carrier 还必须分别命中第三节规定的宿主/region/nesting 证据。调用级 grant 不得扩大地点、endpoint 或数据类别；broker 每次网络 host call 都重验 active manifest、endpoint exact match 和证据有效期。

### 2.2 `DataPolicyV1` 与 `ProviderLifecycleV1` exact schema

`data_policy` 是 strict `DataPolicyV1`，字段恰为 `{processing_location,retention_mode,maximum_retention_seconds,provider_logging,model_training}`：

- `processing_location` 必须逐字段满足 §2.1；
- `retention_mode` 闭集为 `NO_PROVIDER_RETENTION|INVOCATION_BOUNDED|CONTRACT_BOUNDED`；
- `maximum_retention_seconds` 是 0..2,592,000 的整数。`NO_PROVIDER_RETENTION` 时必须为 0 且 provider 在响应提交/失败后零化调用内容；`INVOCATION_BOUNDED` 时必须为 1..86,400 且不得晚于 invocation/effect reconciliation deadline；`CONTRACT_BOUNDED` 时必须为 1..2,592,000、命中签名站点 policy 的更小上限并产生到期处置证据。provider 自报的隐含/无限保留一律拒绝；
- `provider_logging` 闭集仅为 `FORBIDDEN|FIXED_NON_CUSTOMER_METADATA_ONLY`。后者只允许 schema 固定的 `invocation_id,provider_id,operation_code,started_at,finished_at,outcome,stable_error_code,byte_counts,evidence_digest`，禁止业务字段、请求/响应正文、对象名称、secret/key/file bytes 或客户可识别 hash；
- `model_training` 在本最高安全档只能为常量 `FORBIDDEN`，不存在 opt-in、匿名化训练或 provider 默认条款例外。

`lifecycle` 是 strict `ProviderLifecycleV1`，字段恰为 `{health_operation,health_binding,reconcile_operation,drain_timeout_ms,invocation_timeout_ms,disable_mode,unknown_outcome_mode}`：

- 两个 operation code 均使用 `[a-z][a-z0-9_.-]{2,127}` 且互不相同。`health_binding` 是独立 strict `ProviderHealthBindingV1={invocation_capability:"platform.provider.health",operation_code,effect_kind:"READ_ONLY",input_contract_digest,output_contract_digest,resource_profile:"PROVIDER_HEALTH_MINIMAL_V1"}`；`operation_code` 必须逐字等于 `health_operation`，两个 digest 分别 exact-join contracts 中该 code 唯一 INPUT/OUTPUT 行。它是 §2 `operation_bindings`/`provided_capabilities` 完整业务映射规则的唯一明确例外：不得复制进普通 binding，也不要求 provider“提供”系统 capability。Broker 只允许不可委派的 authority lifecycle principal 调用，并要求本 ADR §5 exact `ResourceGrantV1.invocation_origin.kind=LIFECYCLE`、`lifecycle_operation=HEALTH_PROBE`；这不是普通业务 grant，且固定 capability/operation/resource profile 后不能携带 object/file/secret/key/customer data scope。用户、插件、MCP、支持会话、INTERACTIVE/DURABLE origin 或伪造 lifecycle principal 全部拒绝；固定 health resource profile 最多 5 秒/64 MiB、无文件、无 secret、无客户 data class/legal entity，网络仅可到 carrier 已签 endpoint并发送空客户载荷。INPUT exact shape 为 `ProviderHealthProbeRequestV1={probe_nonce,deadline_ms}`，OUTPUT 为 `ProviderHealthProbeResultV1={provider_id,provider_manifest_digest,observed_state,observed_at,evidence_digest}`，`observed_state=HEALTHY|DEGRADED|UNAVAILABLE`；任何额外正文/identifier/hash 都拒绝且结果只能影响 provider lifecycle，不成为业务成功事实。`reconcile_operation` 必须恰有 RECONCILE contract并绑定每个可能产生外部效果的 operation。缺失、错 direction、重名、health capability/origin/grant 绕行或未登记 code 都使 manifest 无效；
- `invocation_timeout_ms` 为 100..300,000，`drain_timeout_ms` 为 1,000..300,000 且不得小于 invocation timeout；host 使用 Task 2 monotonic deadline，manifest 数字不能替代可信时钟；
- `disable_mode` 只能为常量 `DRAIN_THEN_DISABLE`：先拒绝新 invocation，再等待/终止可安全中断的调用，并把不可证明结果转入 reconcile；不存在立即遗忘、保留后台通道或无限 drain；
- `unknown_outcome_mode` 只能为常量 `RECONCILE_BEFORE_RETRY`。Unknown 必须先持久化 Effect/Obligation 和调用 exact identity，再由 `reconcile_operation` 获取独立外部证据；不得自动重试不可逆效果，也不得把超时当 Rejected/Confirmed。

JCS 中两个对象按字段名 canonicalize；闭集 token 大小写逐字匹配，数字禁止字符串/浮点/负值/null。站点策略只能进一步缩短 retention/timeout 或把 logging 从 metadata 降为 FORBIDDEN，不能扩大 manifest。Conformance 必须包含 retention 三模式边界、logging schema/正文泄漏、training 非 FORBIDDEN、operation contract/direction/linkage、timeout 上下界/相对关系、disable token 和 Unknown 自动重试的逐项负例。

### 三、carrier 闭集与宿主证据

`ProviderCarrierV1` 只有以下五个带标签变体，未知值拒绝：

1. `TRUSTED_IN_PROCESS_ADAPTER`：仅允许第一方、release-signed、随 Rust authority 维护升级的基础 adapter；不能热加载，也不能把客户扩展标成此类。
2. `WASM_COMPONENT`：绑定 component digest，默认无 preopen/network，资源只能经 broker grant。
3. `WINDOWS_JOB_WORKER`：绑定 Authenticode signer、PE digest、service SID/restricted-token profile 和 private-pipe policy。
4. `HYPERV_WINDOWS_CONTAINER`：绑定 image digest、Windows base build、HCS/Hyper-V isolation policy 和 ephemeral data-root policy。
5. `REMOTE_HTTPS`：只能经 SQL-free integration gateway；精确绑定 HTTPS origin、port、SPKI pin、mTLS policy、DNS/IP policy、redirect=`DENY`、proxy=`DENY` 和 response limit。

`carrier` 使用内部标签 `kind` 的 tagged union，五个 variant 的 strict fields 恰为下表；每个 object 都拒绝未知/缺失字段，不能把 profile 名换成自由配置：

| `kind` | 其余 exact fields 与常量 |
|---|---|
| `TRUSTED_IN_PROCESS_ADAPTER` | `component_code,crate_package,release_artifact_digest,signer_subject,abi_version`；两个 digest/subject 遵守第一节格式，`abi_version` 为正整数，component/crate 必须命中签名 release manifest，不能给动态路径 |
| `WASM_COMPONENT` | `component_digest,wit_world,wit_contract_digest,runtime_profile,preopens,direct_network`；`runtime_profile="F57_WASMTIME_BROKERED_V1"`、`preopens=[]`、`direct_network="DENY"`，WIT world 为 `[a-z][a-z0-9_.-]{2,127}`，两个 digest 为 `sha256:` lowerhex |
| `WINDOWS_JOB_WORKER` | `executable_digest,authenticode_signer_subject,service_sid,restricted_token_profile,job_object_profile,named_pipe_policy`；三个 profile 常量分别为 `F57_RESTRICTED_TOKEN_V1`、`F57_JOB_OBJECT_KILL_ON_CLOSE_V1`、`FIRST_INSTANCE_LOCAL_MUTUAL_ATTESTATION_V1`，SID 必须 canonical 且 exact-bind 当前服务身份 |
| `HYPERV_WINDOWS_CONTAINER` | `image_digest,windows_base_build,hcs_profile,hyperv_isolation_profile,ephemeral_data_root_policy,egress_mode`；三个 profile 常量分别为 `F57_HCS_V1`、`HYPERV_ISOLATION_REQUIRED_V1`、`VALIDATED_DATA_ROOT_EPHEMERAL_V1`，`egress_mode="BROKER_ONLY"` |
| `REMOTE_HTTPS` | `origin,port,spki_sha256,mtls_policy,dns_ip_policy,redirect_policy,proxy_policy,response_limit_bytes`；origin 为无 path/query/fragment 的 canonical HTTPS origin，port 1..65535，`mtls_policy="REQUIRED_CLIENT_CERT_V1"`、`dns_ip_policy="PINNED_RESOLUTION_CN_V1"`、redirect/proxy 都为 `DENY`，response limit 为正整数且不大于 resource ceiling output bytes |

`carrier_binding_digest` 的输入精确为上述 tagged object 的 RFC 8785 JCS；不能只 hash `kind`、外部文件名或 UI 摘要。每个 variant 的 digest、signer/SID/build/profile 与实际进程、component、HCS container 或 TLS peer 由 host 在激活和每次 invocation 重新核对，无法读回即失败关闭。

容器证据按 carrier 区分，不再引用未定义的 `BC-2`：

- 物理 Windows Server 必须证明 CPU virtualization/SLAT、DEP、Hyper-V、Windows Containers、HCS、匹配 host/container build、Hyper-V isolation、资源容量与 escape 负例；`nesting` 固定为 `NOT_APPLICABLE_PHYSICAL`。
- IaaS Windows Server 除上述证据外，必须额外绑定 cloud provider、tenant、region、instance/SKU、host generation 和该 SKU 的 nested-virtualization 实测证据；缺任一项返回 `HOST_CAPABILITY_UNAVAILABLE`。
- P340 32GB profile 默认不激活 container。默认关闭不免除 adapter、conformance、drain/delete 和 orphan-cleanup 交付。

### 四、`PermissionCeilingV1` exact schema

`permission_ceiling` 只含以下字段；空数组表示该类权限为零，不能表示通配：

- `capabilities: [CapabilityId]`；
- `objects: [{object_code, actions, fields}]`，actions 只允许 `READ|CREATE|UPDATE|EXECUTE`，字段显式列举；
- `data_classes: [SecurityLevelCode]`；wire 只能是整数 `10|20|30|40`，必须非空、升序、唯一且 prefix-closed，合法值仅 `[10]`、`[10,20]`、`[10,20,30]`、`[10,20,30,40]`；
- `file_classes: [{class, access}]`，access 只允许 `READ_HANDLE|CREATE_HANDLE`，不接受原始路径；
- `network: [{origin, port, spki_sha256, methods}]`，methods 只允许 `GET|POST|PUT|PATCH|DELETE`；
- `secret_purposes: [{secret_ref, purpose}]`，运行时只换取调用级 opaque handle；
- `key_purposes: [{key_ref, purpose}]`，禁止 raw key；
- `legal_entity_scope: EXPLICIT_LIST` 与非空 `legal_entities`；跨全部法人没有通配值；
- `resource_ceiling: {cpu_millis, memory_bytes, wall_time_ms, output_bytes, process_count, concurrency}`。

`objects` 按 `object_code` 排序且 object_code 唯一，每行 actions 与 fields 分别按 canonical token 排序去重，禁止用同一 object_code 多行表达并集；`network` 按 `(origin,port,spki_sha256)` 排序且组合唯一，每行 methods 按 canonical method token 排序去重；`secret_purposes` 按 `(secret_ref,purpose)` 排序且组合唯一，`key_purposes` 按 `(key_ref,purpose)` 排序且组合唯一，同一 ref 的不同 purpose 也因此具有唯一稳定次序；其余数组按 canonical value 排序去重。manifest 未声明的类别一律为零权限。

### 五、`ResourceGrantV1` exact schema

每次 invocation 由 authority 生成一个单次、短期、不可转授的 grant，payload 只能含：

| 字段 | 类型与约束 |
|---|---|
| `schema_version` | 常量 `1` |
| `purpose` | 常量 `EP-F57-RESOURCE-GRANT-V1`；与共享 trait/policy 三方逐字相等 |
| `grant_id` | UUID |
| `invocation_id` | UUID；与 `InvocationEnvelope` 相同 |
| `provider_id` / `provider_version` | exact active provider |
| `provider_manifest_digest` | exact signed manifest digest |
| `package_manifest_digest` | exact owning package/release digest |
| `generation` | exact active signed generation |
| `legal_entity_id` | 单一法人，不允许数组或全局值 |
| `invocation_origin` | 本节闭合 `InvocationOriginV1`；交互、durable work 与 authority lifecycle 三者不可互相伪装 |
| `authority_epoch` | 当前单写权威 epoch；每次 host call 重验，切换后旧 grant 立即失效 |
| `authorization_context_digest` | `sha256:` lowerhex；绑定签发时已验证的 capability/scope/SoD/approval/assignment/lifecycle policy exact decision inputs，不替代实时重验 |
| `capability` | 单一 capability |
| `operation_code` / `effect_kind` | exact active `ProviderOperationBindingV1` 的 operation/effect；不得由 provider 或 caller 改写 |
| `object_fields` | 此次允许的 exact object/action/field 集 |
| `data_classes` | 与 ceiling 同 wire 的非空、升序、唯一、prefix-closed `SecurityLevelCode[]`；只能缩小，不能用单一最大值重建或扩大集合 |
| `file_handles` | 调用级 opaque handle；含 object id/digest/class，不含原始路径 |
| `network_endpoints` | 此次允许的 exact origin/port/SPKI/method 集 |
| `secret_handles` / `key_handles` | 调用级 opaque handle 与 purpose；不含 secret/key bytes |
| `resource_limits` | 不大于 manifest/package ceiling 的 exact limits |
| `input_contract_digest` / `output_contract_digest` | exact operation binding contracts |
| `reconcile_operation_code` / `reconcile_contract_digest` | EXTERNAL_EFFECT 时逐字命中 binding/lifecycle；其余 effect kind 均为 JSON null |
| `issued_at` / `expires_at` | `TrustedUtc`；`expires_at` 不得超过 invocation deadline 或 manifest expiry |
| `invocation_nonce` | 32 个随机字节，base64url |
| `maximum_uses` | 常量 `1`；broker 子 handle 也不得扩大权限或期限 |

`InvocationOriginV1` 是以 `kind` 判别、每个 object 均拒绝未知字段的唯一闭集：

- `INTERACTIVE={kind:"INTERACTIVE",principal_id,device_id,session_id}`；三个 ID 都是 UUID，必须逐字命中当前未撤销、未过期、device-bound 的 authority session。签发与每次 host call 均重验当前 capability、法人/对象/字段 scope、MFA freshness、SoD、authority epoch 和 session/device state；断线、撤权、设备隔离或 session 结束立即使 grant 失效。
- `DURABLE={kind:"DURABLE",service_principal_id,durable_execution_id,objective_id,work_item_id,trigger_fact_id,assignment_attempt_id}`；六个 ID 都是 UUID，service principal 必须命中不可交互登录、不可转授的 automation roster，其他五个 ID exact-join 同一法人、当前 generation、未终结 Objective/WorkItem、不可变 trigger fact 和当前 assignment attempt。它从当前 automation capability、policy、SoD/approval evidence 与 assignment scope 重新授权；不得复制发起人的 session/device、延长临时 grant，或因原发起人仍/已离线而改变结果。定时执行、重试、补偿和 external-effect reconciliation 都使用该 variant，并保持 exact-once obligation/effect identity。
- `LIFECYCLE={kind:"LIFECYCLE",service_principal_id,lifecycle_run_id,lifecycle_operation}`；两个 ID 是 UUID，`lifecycle_operation` 闭集为 `HEALTH_PROBE|DRAIN|RECONCILE`。service principal 必须命中 non-delegable authority lifecycle roster，run exact-join 当前 provider/generation/lifecycle state。HEALTH_PROBE 只能选择 §2.2 的只读 health binding、固定 minimal resource profile、空 customer data/file/secret/key scope；DRAIN 不得启动业务 effect；RECONCILE 只能选择原 UNKNOWN effect 已绑定的 reconcile operation/evidence。用户、插件、MCP 或普通业务 principal 不能声明此 variant。

INTERACTIVE、DURABLE、LIFECYCLE 没有 nullable 交叉字段，也没有 caller-supplied custom origin。签发事务先持久化 strict `AuthorizationContextV1`，其 exact fields 为 `{schema_version:1,purpose:"EP-F57-AUTHORIZATION-CONTEXT-V1",invocation_origin_sha256,authority_epoch,legal_entity_id,capability,operation_binding_sha256,package_ceiling_sha256,provider_ceiling_sha256,invocation_scope_sha256,runtime_policy_generation,runtime_policy_sha256,required_source_kinds,source_refs,deny_decision:"NO_APPLICABLE_DENY",deny_evaluation_sha256,evaluated_at,valid_until}`；每个 object 拒绝未知字段。`operation_binding_sha256` 是所选七元 binding 的 JCS digest；`invocation_scope_sha256` 是 grant 的 `object_fields,data_classes,file_handles/network_endpoints/secret_handles/key_handles` resource identity（去掉 handle binding digest）与 `resource_limits` strict object 的 JCS digest。`valid_until` 不晚于 grant、session/assignment/lifecycle source、manifest 或 policy 中最早到期者。

`required_source_kinds` 是 byte-sorted unique closed enum；`source_refs` 按 `(source_kind,source_id,source_version,payload_sha256)` 排序唯一，每项 exact 为 `{source_kind,source_id,source_version,payload_sha256}`，ID 为 UUID、version 为正整数、digest 为 `sha256:` lowerhex，且 kind 必须出现在 required set 并恰好一项。闭集按 byte order 恰为 `APPROVAL_DECISION|ASSIGNMENT_ATTEMPT|AUTHORITY_POLICY|DEVICE_POSTURE|DURABLE_SERVICE_PRINCIPAL_GRANT|INTERACTIVE_SESSION|LIFECYCLE_SERVICE_PRINCIPAL_GRANT|MFA_EVIDENCE|OBJECTIVE_STATE|PRINCIPAL_CAPABILITY_SET|PROVIDER_LIFECYCLE_STATE|SOD_DECISION|TRIGGER_FACT|UNKNOWN_EFFECT|WORK_ITEM_STATE`。INTERACTIVE 的基础 required array 恰为 `["AUTHORITY_POLICY","DEVICE_POSTURE","INTERACTIVE_SESSION","PRINCIPAL_CAPABILITY_SET","SOD_DECISION"]`；当前 risk policy 要求 MFA/approval 时分别加入 `MFA_EVIDENCE|APPROVAL_DECISION` 并重新 byte-sort。DURABLE 基础 array 恰为 `["ASSIGNMENT_ATTEMPT","AUTHORITY_POLICY","DURABLE_SERVICE_PRINCIPAL_GRANT","OBJECTIVE_STATE","SOD_DECISION","TRIGGER_FACT","WORK_ITEM_STATE"]`，按当前 policy 加 `APPROVAL_DECISION` 后重新排序；不得出现 interactive source。LIFECYCLE 基础 array 恰为 `["AUTHORITY_POLICY","LIFECYCLE_SERVICE_PRINCIPAL_GRANT","PROVIDER_LIFECYCLE_STATE"]`，只有 RECONCILE 再加入末尾的 `UNKNOWN_EFFECT`；不得出现 human/session/work-item source。`deny_evaluation_sha256` 是所有适用 deny rule ID/version/input/result 的 byte-sorted strict row array digest；任何 applicable deny 使事务不创建 context/grant，因此唯一可序列化 token 是 `NO_APPLICABLE_DENY`。

`authorization_context_digest=SHA-256("EP-F57-AUTHORIZATION-CONTEXT-V1\0" || JCS(AuthorizationContextV1))`。grant 与 context 在同一 authority-fenced transaction 内 exact `(grant_id,invocation_id,digest)` 关联；worker只得到 grant/digest，broker从 append-only authority row加载 context。每次 host call 都重验 trusted time、authority epoch，逐个把 source ref exact-join为当前、有效、未撤销的同 version/digest，重新执行 deny evaluation和四路 scope交集，再重算 operation/ceiling/scope/policy/context digest；任何 current input 漂移都拒绝，不把签发时摘要当缓存的 allow。grant 按第一节 canonicalize/sign；provider host 必须把 grant 绑定到已验证的 service SID/process/image digest/worker instance 或 container identity。复制到其他 invocation、origin、provider、进程、generation、法人或 authority epoch 均失败。

上述复合字段不得由 carrier 自行设计。grant 的 `{capability,operation_code,effect_kind,input_contract_digest,output_contract_digest,reconcile_operation_code,reconcile_contract_digest}` 必须逐字等于当前 manifest 中唯一选定的 `ProviderOperationBindingV1`；grant 签发、host dispatch、worker ABI、ATTEMPT/COMPLETION 和 UNKNOWN reconciliation 都保存并重验这七项。相同 schema digest 不能使两个 operation 互换；manifest swap、operation swap、capability swap、effect downgrade、reconcile null/非空形状和 digest mismatch 均在 dispatch 前失败。nested exact schemas 为：

- `ObjectGrantV1={object_code,action,fields}`，action 只允许 `READ|CREATE|UPDATE|EXECUTE`，fields 按 canonical field code 排序去重；按 `(object_code,action)` 排序且组合唯一。
- `FileHandleV1={handle_id,object_id,object_digest,file_class,access,handle_binding_digest}`，两个 ID 为 UUID，digest 为 `sha256:` lowerhex，access 只允许 `READ_HANDLE|CREATE_HANDLE`；按 handle ID 排序唯一。
- `NetworkEndpointGrantV1={origin,port,spki_sha256,methods}`，形状与 `PermissionCeilingV1.network` exact-equal，按 `(origin,port,spki_sha256)` 排序唯一。
- `SecretHandleV1={handle_id,secret_ref,purpose,handle_binding_digest}` 与 `KeyHandleV1={handle_id,key_ref,purpose,handle_binding_digest}`；handle ID 为 UUID，ref/purpose 使用对应 ceiling 的 canonical identity，按 handle ID 排序且 ref/purpose 不重复。
- `ResourceLimitsV1={cpu_millis,memory_bytes,wall_time_ms,output_bytes,process_count,concurrency}`，六值均为正整数且逐项不大于 package/provider ceiling。

每个 `handle_binding_digest` 必须 exact-equal `SHA-256("EP-F57-HANDLE-BINDING-V1\0" || JCS({grant_id,invocation_id,provider_id,provider_version,provider_manifest_digest,generation,legal_entity_id,invocation_origin_sha256,authority_epoch,authorization_context_digest,issued_at,expires_at,maximum_uses:1,handle_id,resource_identity}))`；`invocation_origin_sha256=SHA-256(JCS(invocation_origin))`，resource identity 是该 nested object 去掉 binding digest 后的 JCS。broker 读取 handle 时逐项重算，并以 `(handle_id,invocation_nonce)` 单次 CAS 消费；错 invocation/origin/provider/generation/epoch/authorization/identity、到期、重复或 `maximum_uses != 1` 全拒绝。`object_fields/file_handles/network_endpoints/secret_handles/key_handles` 分别按上述 key canonical sort/unique；未知字段、通配、重复 identity、空 handle、raw path/secret/key 或无法与 ceiling 使用同一 identity 比较均使整个 grant 无效。

### 六、唯一权限求值公式

对每一项 capability、对象、字段、文件、网络、secret、key、数据等级和资源，唯一有效权限为：

```text
effective = package_ceiling
          ∩ provider_ceiling
          ∩ invocation_grant
          ∩ runtime_authorization_now
```

四项均按相同 canonical resource identity 求交集；`data_classes` 四路都使用上述整数 token 的 prefix-closed set，交集仍须是 prefix-closed set，空集即拒绝。任一项缺失、未知、过期、撤销、错代或无法比较，结果为空；不得把单一 `max/ceiling` 数字、名称字符串或隐式枚举序号混入比较。显式 deny、法人隔离、职责分离和内核安全底线在交集后再次否决，任何 allow 都不能覆盖 deny。

runtime authorization 在 invocation 接收时以及每次 broker host call 时重新求值，不能把启动时结果缓存到调用结束。package/provider ceiling 只能缩小不能被 grant 扩大；broker 子 handle 只能比父 grant 更窄。provider 输出还必须通过 exact output schema、字段和字节上限，响应成功不能证明业务效果成功。

### 七、generation、撤销和热替换

active identity 精确为 `(provider_id, provider_version, provider_manifest_digest, package_manifest_digest, generation)`。desired/observed 任一不一致时停止新 invocation；高风险调用失败关闭。

签名 revocation record 精确绑定上述 identity、`revocation_generation`、`revoked_at`、closed reason code 和 replacement identity。package、provider、signer/certificate、contract schema 或 generation 任一撤销都会立即拒绝新 grant；运行中的 grant 在下一次 broker call 失效。对不可安全中断的外部效果，host 停止后续调用并进入 `Unknown`/reconcile；其余 worker 在 drain deadline 后由 Job Object/HCS 终止。

升级只能 stage → conformance → approval → signed generation → drain → activate。新 invocation 使用新 identity，旧 invocation 继续钉住旧 identity直到完成、补偿、Unknown/reconcile 或受控终止；不得静默跨代。停用保留业务数据、附件、事实、审计和 portable export。

### 八、XML 只能作为显式 codec

`ProviderCodecV1` 闭集为 `JSON_SCHEMA_V1|MCP_JSON_RPC_V1|CSV_RFC4180_V1|XLSX_OOXML_V1|DOCX_OOXML_V1|PDF_V1|BINARY_EXACT_V1|XML_XSD_V1`。不得按扩展名、MIME 猜测、响应首字节或 provider 自报自动启用 XML。

选择 `XML_XSD_V1` 时 `xml_policy` 必须精确包含：

- `media_types` 非空 allowlist；
- `root_qname`、`namespace_allowlist`、`xsd_id`、`xsd_version`、`xsd_digest`；
- `field_mapping_digest`，其输出只能进入已登记 typed contract；
- `maximum_bytes`、`maximum_depth`、`maximum_elements`、`maximum_attributes_per_element`、`maximum_text_bytes`；
- 固定 `dtd=DENY`、`external_entities=DENY`、`parameter_entities=DENY`、`xinclude=DENY`、`external_schema_location=DENY`、`xslt=DENY`、`network_resolution=DENY`。

XML 必须使用有界 streaming parser，在映射前完成 XSD/namespace/root/大小验证。XMLDSIG、SOAP、WSDL 和任意 XPath/XSLT 不由 `XML_XSD_V1` 隐式支持；需要时必须新增独立版本化 codec、威胁模型和 conformance，未知 codec 失败关闭。

## 理由

把 package、provider、invocation 与实时授权分成四层并取交集，可以同时保留可插拔能力和最小权限。统一 canonical/signed manifest 让本地、自建、已有服务和外部 provider 使用同一治理模型；carrier-aware 容器证据避免对物理 P340错误要求 nested virtualization，也不放松 IaaS 风险。XML 显式化可阻止 XXE、schema substitution 和解析器资源耗尽成为隐蔽旁路。

## 后果

正面：provider 权限只有一个可验证答案；法人、分类和资源 ceiling 各只有一个 manifest authority；package 无法替调用者扩权；manifest、grant、generation、contract 和撤销可完整审计；物理/IaaS 容器门无歧义；XML 不会被意外启用。

代价：每个 provider 必须提供 strict manifest、schema digest、conformance 和生命周期实现；每次 host call 都有实时交集求值成本；新增 codec/carrier 必须新建版本化 ADR/契约而不能添加自由字符串。

## 影响范围

- capability package、provider、MCP、integration gateway 与 plugin host；
- WASM、Windows Job worker、Hyper-V Windows container 与 remote HTTPS carrier；
- signed generation、authorization、secret/key/file broker、audit 与 revocation registry；
- provider manifest/resource grant migration、contract fixtures、XML parser 和 conformance tests；
- Server Control Center 的 provider 权限预览、数据驻留、激活、排空、撤销与证据界面。

Conformance 必须至少包含 `legacy_top_level_resource_ceiling_is_rejected`、`legacy_data_policy_legal_entities_is_rejected`、`legacy_data_policy_maximum_classification_is_rejected`、`permission_ceiling_is_the_only_scope_classification_resource_authority`、`same_secret_ref_multiple_purposes_have_one_canonical_digest`、`same_key_ref_multiple_purposes_have_one_canonical_digest`、`security_level_unknown_code_is_rejected`、`security_level_non_prefix_set_is_rejected`、`grant_cannot_widen_security_level_set`、`interactive_origin_requires_live_session_and_device`、`durable_origin_rejects_copied_human_session`、`durable_origin_requires_current_work_assignment`、`lifecycle_origin_is_non_delegable_and_operation_bound`、`origin_swap_changes_every_handle_binding`、`authority_epoch_or_authorization_context_drift_revokes_grant`；secret/key 两项必须用同一 ref 的多 purpose 乱序输入证明 canonical bytes/digest 唯一，并证明重复 `(ref,purpose)` 被拒绝。全套测试还必须证明 package/provider/grant/runtime 四路按同一 identity 逐项求交，四路 `{10,20,30,40}`、`{10,20,30}`、`{10,20}`、`{10}` 的结果恰为 `{10}`，任一未知/乱序/重复/非 prefix 集均在授权前失败。
