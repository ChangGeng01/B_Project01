# F-57 客户端、生命周期与安全运营执行契约

> 日期：2026-08-23（Australia/Melbourne）
> 收敛修订：2026-08-24（Australia/Melbourne）
> 状态：`CURRENT_SUBJECT` / `APPROVED` / `IMPLEMENTATION_NOT_STARTED`
> 适用范围：员工四端 Workbench、客户/供应商门户、员工 HTTPS API、客户端分发、终端数据保护、数据驻留、数据生命周期、远程支持、安全事件、完整导出和生产运营边界
> 权威关系：本文细化 [F-57 总体设计](2026-08-23-f57-governed-automation-fabric-design.md)，不得改变其唯一权威、动态权限、HDD、客户持钥和失败关闭不变量

## 0. 用途和唯一解释规则

本文关闭总体设计中已经确定方向、但仍可能让实现人员自行选择协议、状态或失败结果的主题。本文中的 exact-set、状态机、字段和失败语义是第一阶段的实现输入；部署现场只填写证书主体、端点、设备、介质、责任人和实测证据，不得改变契约。

本文对应稳定需求：`GOV-007`、`GOV-010`、`CLI-004`、`CLI-008`、`CLI-009`、`INT-001`、`INT-002`、`INT-004`、`POR-003`、`SEC-004`、`SEC-006`、`SEC-007`、`SEC-015`、`SEC-016`、`SEC-017`、`NFR-001`、`NFR-005`、`NFR-009`、`NFR-014`、`NFR-016`、`NFR-017`、`NFR-018`、`DEF-010` 和 `DEF-011`。

本文为追踪而保留的 `Task 1…25` 只表示历史 `F57-01…F57-25` 所有权桶；不再表示旧计划顺序。实际文件、迁移、依赖和门禁只由 2026-08-24 收敛主计划及 G0、G1/G2、G3/G4、G5/G6 子计划决定。

## 1. 员工 C/S 在线协议

### 1.1 唯一网络入口

Workbench 只通过签名 deployment manifest 中的 `employee_api_origin` 访问员工 API。第一阶段协议是 HTTPS 上的版本化 JSON/二进制分片传输，不允许客户端连接 PostgreSQL、Windows named pipe、服务器文件目录、Control Center 私有路由或任意内部服务。

`employee_api_origin` 必须是无 path/query/fragment 的 HTTPS origin。证书、SPKI pin、受信 CA、允许网络和代理策略属于签名部署配置；重定向、系统代理自动发现、localhost、loopback、IP 直连、降级 HTTP 和跨 origin token 转发一律拒绝。

第一阶段员工入口的 **method + path exact-set** 固定如下；`*`、可选尾斜杠、别名、通用对象路由和未列出的 method 全部不是兼容入口：

| Method | Exact path | 用途与失败关闭边界 |
|---|---|---|
| `POST` | `/employee/v1/session/start` | 登录、MFA、设备绑定和初始 generation report；不接受客户端声明 actor、role、法人或 policy |
| `POST` | `/employee/v1/session/handshake` | 代际握手；严格执行 §1.2.1，不能用缓存 directive 放宽 |
| `POST` | `/employee/v1/session/renew` | 在当前身份、设备、撤销和 generation 全部重验后续期 |
| `POST` | `/employee/v1/session/end` | 注销并撤销当前 session/refresh family；幂等重复不复活 |
| `POST` | `/employee/v1/commands` | 强类型命令提交；服务器重验身份、设备、授权、SoD、版本、generation 和幂等键 |
| `GET` | `/employee/v1/commands/{request_id}` | 只读取得同 principal/device/legal-entity 作用域内的既有命令回执；不得重执行命令 |
| `POST` | `/employee/v1/queries` | 类型化查询、裁剪投影和不透明游标分页；不返回隐藏对象数量或聚合侧信道 |
| `GET` | `/employee/v1/tasks/stream` | 有界 SSE 任务/失效提示；只作刷新提示，不携带权威业务正文，断流按 watermark 补齐 |
| `GET` | `/employee/v1/ui-schema/{generation}` | 取得该代签名 UI schema、能力矩阵和客户端版本边界；签名/摘要/兼容不符即失败关闭 |
| `POST` | `/employee/v1/files/upload-sessions` | 为一个目标对象创建有界、短期、作用域固定的上传会话 |
| `GET` | `/employee/v1/files/upload-sessions/{upload_id}` | 查询/恢复同一上传会话的权威分片水位；错主体或过期统一拒绝 |
| `PUT` | `/employee/v1/files/upload-sessions/{upload_id}/chunks/{chunk_no}` | 写入声明编号和 digest 的单一分片；正文不进 JSON、日志或 URL |
| `POST` | `/employee/v1/files/upload-sessions/{upload_id}/complete` | 完成后只进入隔离/扫描链，不直接形成可发布附件 |
| `GET` | `/employee/v1/files/{object_id}/versions/{version_id}` | 只下载明确的已发布不可变版本；禁止 bare-latest、路径或存储 locator |
| `POST` | `/employee/v1/devices/{device_id}/attestations` | 提交受管设备证明；设备 ID 必须与认证上下文一致 |
| `POST` | `/employee/v1/devices/{device_id}/wipe-receipts` | 提交签名擦除回执；它只证明擦除流程，不自行恢复设备信任 |

OpenAPI、四端生成客户端、gateway 路由表和 contract test 必须逐项比较这 16 个 method/path pair。G4–G6 的业务 owner 只能在既有 `/commands`、`/queries` 信封中追加具名 discriminator；除非先修改本契约并同步所有权/版本登记，否则不得增加路径。

### 1.2 请求、结果和错误信封

每个在线命令固定携带：`request_id`、`command_type`、`idempotency_key`、`expected_generation`、`expected_subject_version`、`generation_report`、`client_version`、`device_key_id`、`device_signature` 和类型化 `payload`。actor、当前法人、会话、授权依据、风险等级与批准策略全部由服务端认证上下文取得。

查询固定携带：`query_type`、`generation`、`generation_report`、类型化过滤条件、允许的 sort key、`page_size` 和不透明 cursor。`page_size` 上限由签名策略固定；cursor 绑定 principal、device、legal entity、query type、generation、授权摘要和过期时间，错绑定或过期不得回退成第一页静默执行。

结果固定携带：`correlation_id`、`authoritative_generation`、`generation_directive`、`subject_version`、`outcome`、`audit_ref` 和类型化值。错误遵守 `NFR-010`，不得泄露对象存在性、堆栈、SQL、路径、密钥或内部拓扑。

#### 1.2.1 `ClientGenerationReportV1` 与握手

`ClientGenerationReportV1` 是 strict object，字段恰为 `mode,desired_generation,observed_generation,observed_ui_schema_sha256,observed_capability_matrix_sha256,client_package_sha256`。`mode` 闭集为 `BOOTSTRAP|ACTIVE`：

- `BOOTSTRAP` 只允许首次安装或安全擦除后使用；`desired_generation=observed_generation=0`，三个 digest 都为 `null`，只能调用登录、握手、更新和诊断，不能查询客户数据或提交业务命令。
- `ACTIVE` 要求 desired/observed 都为正整数，三个 digest 都是 64 位 lowerhex；`desired_generation` 是客户端最后一次验证的服务器 `ClientGenerationDirectiveV1.desired_generation`，`observed_generation` 是端侧实际原子激活并正在渲染/校验的签名 generation，二者不得由 UI 配置或缓存文本伪造。

`ClientGenerationDirectiveV1` 是服务器 strict result object，字段恰为 `authoritative_generation,desired_generation,minimum_compatible_generation,compatibility,ui_schema_sha256,capability_matrix_sha256,minimum_client_version,recommended_client_version`；`compatibility` 闭集为 `COMPATIBLE|REFRESH_REQUIRED|UPDATE_REQUIRED|REVOKED`。`POST /employee/v1/session/handshake` 在认证设备上下文中接收 report 并返回 directive；每次 session start/renew、command 和 query 仍必须携带同一 report，服务器不得只信握手缓存。

命令的 `expected_generation` 必须等于 `generation_report.observed_generation`；查询的 `generation` 也必须相等。服务器把 report 与当前 signed generation、客户端 package digest、UI schema/capability digest 和兼容窗口逐项比较。高风险命令在 `BOOTSTRAP`、desired/observed 不等、observed 不等 authoritative、digest 不等、低于 minimum、package revoked 或 compatibility 非 `COMPATIBLE` 时返回 `PLATFORM.CLIENT.GENERATION_INCOMPATIBLE` 并零业务写；低风险只读只有在签名兼容矩阵明确允许时可返回裁剪结果和 `REFRESH_REQUIRED` directive。客户端原子激活成功后下一次 report 才可提升 observed；只写 desired、伪造 observed 或部分切换 schema/capability 均失败关闭。

### 1.3 会话和兼容窗口

- 内部员工会话总有效期固定 8 小时、空闲 30 分钟失效，同时活跃会话上限 3；高风险动作仍须重新认证，不能用长会话替代。
- access/session secret 只保存在操作系统安全存储；设备私钥不可导出。服务器撤销用户、设备、证书、会话或客户端版本后，所有端最迟在下一请求失败关闭。
- 服务器只支持当前 API major。minor 采用向后兼容 closed schema；未知字段默认拒绝，只有明确标为 `forward_ignorable` 的显示元数据可以忽略。
- 签名 generation 固定 `minimum_client_version`、`recommended_client_version` 和 `revoked_client_digests[]`。低于最低版本只能进入更新/诊断界面，不能提交业务命令。
- `CLI-009` 的四端 contract suite 必须用同一 machine-readable IDL、同一正负 fixture 和同一错误码验证 Windows、macOS、iOS、Android；端侧不得各自维护协议副本。
- 四端 suite 还必须逐端覆盖 bootstrap、desired 超前、observed 落后、schema/capability/package digest 错配、原子提升 observed、撤销包和高风险 fail-closed；只回显 `expected_generation` 不算满足 `F57-C02`。

## 2. 离线、终端缓存和数据防外泄

### 2.1 离线持久化唯一允许集合

终端不保存通用业务数据库。可持久化 exact-set 为：

1. 签名 UI schema 和能力矩阵；
2. 有界 `ClientIntentV1` 队列；
3. 用户显式选择的临时附件草稿；
4. 可选 `MinimalOfflineProjectionV1`，仅在签名设备策略逐对象、逐字段允许时启用。

`MinimalOfflineProjectionV1` 必须绑定 principal、device、legal entity、record IDs、field allowlist、generation、server version、issued-at、absolute expiry、最大行数和最大字节数。最高安全档默认不启用业务投影离线读；客户若启用，TTL 不得超过 24 小时，单设备总量不得超过签名策略的较小值，过期、撤权、注销、设备不合规、key 不可用或 generation 撤销立即擦除。它永远不是权威状态。

OS 云备份、桌面同步目录、通用浏览器 cache、系统相册、剪贴板历史、Recent/Jump List、搜索索引、崩溃上传和未受管分享目标必须排除业务缓存。复制缓存文件、回滚旧快照、改系统时间或把密文移到另一设备不能恢复访问。

### 2.2 端口能力矩阵

| 控制 | 受管 Windows/macOS 原生端 | 合规 iOS/Android 原生端 | 浏览器门户 |
|---|---|---|---|
| 服务端字段裁剪/脱敏 | 强制 | 强制 | 强制 |
| 动态水印 | 强制 | 强制 | 强制 |
| 导出审批与审计 | 强制 | 强制 | 强制 |
| 打印 | 默认阻断；获批打印走审计 spool | 不提供普通打印入口 | 尽力限制，界面必须明示不作绝对保证 |
| 剪贴板、share/open-in | 受管设备策略强制；高密级恒拒绝 | 受管容器策略强制；高密级恒拒绝 | 尽力限制，无法防止浏览器/系统绕行时改为只读预览 |
| 截图/录屏 | 使用 OS/MDM 能力阻断或检测；不能虚假承诺阻止外部相机 | 使用 OS/MDM 能力阻断或检测；不能虚假承诺阻止外部相机 | 不作阻断承诺，水印和审计持续生效 |
| 已下载文件失效 | 仅在受管 viewer/原生插件内强制 | 仅在受管容器内强制 | 不作承诺；高密级禁止下载 |
| Root/Jailbreak/调试器 | 不合规时禁止高密级、离线和下载 | 不合规时禁止高密级、离线和下载 | 不适用；按浏览器端能力边界 |

`SEC-015` 的含义是“在受支持、受管、合规的端点上强制；在无法可靠控制的端口上降级为只读/禁止下载并诚实披露”，不是宣称软件能阻止另一台相机或被攻陷的操作系统。

### 2.3 设备状态和擦除证据

设备状态闭集为 `PENDING|COMPLIANT|RESTRICTED|REVOKED`，允许边固定为 `PENDING→COMPLIANT|RESTRICTED|REVOKED`、`COMPLIANT→RESTRICTED|REVOKED`、`RESTRICTED→COMPLIANT|REVOKED`；REVOKED 为终态。首次登记必须保持 PENDING，直到当前签名设备策略的完整 attestation 通过才可 COMPLIANT；校验失败或证据不完整进入 RESTRICTED，不能借默认值放行。

Root/Jailbreak、调试注入、系统完整性失败、可恢复的 MDM 失联超窗或安全版本过低进入 RESTRICTED；只有重新取得当前策略版本的完整 attestation、轮换 device epoch/session credential、服务器重新验证且没有任何 revocation reason 后，才能 `RESTRICTED→COMPLIANT`。密钥被盗、设备丢失、证书明确撤销或管理员永久撤销进入 REVOKED；不得恢复原 device_id，重新启用必须登记新 device_id 和新密钥。擦除命令必须形成 server receipt、端侧 receipt 或“未能到达”的暴露窗口，不能把已发送命令冒充已擦除。

T-F57-CLI-004 与 T-F57-SEC-015 必须覆盖全部允许边、未列边拒绝、受限设备高密级/离线/下载失败关闭、re-attestation 恢复门禁、REVOKED 不可恢复以及三种擦除证据结果。

## 3. 四端签名、分发、更新和撤销

### 3.1 `ClientDistributionProfileV1`

每个客户、平台和 audience 的分发资料必须是签名 generation 的一部分，字段 exact-set 为：

```text
schema_version, customer_id, platform, audience,
carrier, application_id, package_digest, package_version,
signing_identity_ref, signing_chain_digest, notarization_or_store_receipt,
minimum_os_version, minimum_client_version, update_origin,
rollout_policy, rollback_policy, revoked_digests[], issued_at, expires_at
```

`carrier` 闭集：

- Windows：`CUSTOMER_OFFLINE_REPOSITORY`、`CUSTOMER_MDM`；
- macOS：`CUSTOMER_MDM`、`CUSTOMER_DEVELOPER_ID_NOTARIZED`；
- iOS：`CUSTOMER_MDM_ENTERPRISE`、`CUSTOMER_APP_STORE`；
- Android：`CUSTOMER_MDM`、`CUSTOMER_MANAGED_PLAY`、`CUSTOMER_SIGNED_APK_REPOSITORY`；
- 外部参与者替代形态：`CONTRACT_APPROVED_WEB_PWA`。

面向客户外部人员的公共商店应用必须使用客户自己的开发者主体、账号和证书；厂商可以代构建、代提交，但不能成为永久账号所有者。内部员工优先使用客户 MDM/企业分发或客户离线仓库。证书、账号或商店审核失败不能降级为未签名安装。

### 3.2 PWA 回退和移动代码边界

公共商店审核连续两轮失败或超过合同约定 14 个自然日时，可以在客户批准后切换 `CONTRACT_APPROVED_WEB_PWA`。切换记录必须列出推送、离线、相机/扫码、设备合规和 DLP 差异；不满足高密级要求的能力保持关闭，而不是假装等价。

iOS/Android 不加载能力包、WASM、原生插件或动态下载的可执行扩展代码。允许下发的只有签名 UI schema、规则数据、模板和静态资源；所有设备能力随已签名应用版本发布。桌面端扩展也只能使用经批准的原生壳能力，不能绕过服务器能力包治理。

### 3.3 更新和失窃证书

- 客户决定更新窗口；无强制厂商在线更新。
- 每个平台先验证包签名、证书链、包 digest、SBOM、generation 和最小版本，再分阶段 rollout。
- 更新失败自动保留上一个已签名可启动版本；数据库/配置迁移不因客户端回滚而回滚权威事实。
- 被盗或撤销签名身份立即进入 incident 流程；相应 digest 加入 `revoked_digests[]`，服务端拒绝其新会话，高风险在线会话立即撤销。
- Tauri 任一必选目标未通过上述签名、安装、更新、撤销和真实启动门，主线在 G5 客户端技术门停止；唯一回退是按现行计划完成全 Flutter + Rust 分支，不能混搭后继续。

## 4. Provider、外部处理位置和 XML 边界

Provider 权限和载体契约以 [ADR-0023](../../adr/ADR-0023-f57-provider-manifest-resource-grant.md) 为准。有效权限永远是：

```text
package ceiling ∩ provider ceiling ∩ invocation grant ∩ current runtime authorization
```

任一层缺失、未知、过期、撤销或 generation 不一致即拒绝。Provider manifest 必须声明处理位置、保留、日志、训练/再利用、网络、文件、密钥、资源、对账和排空语义；数据驻留不允许由连接器默认值决定。

通用 XML/SOAP/XSD 不属于第一阶段核心 provider exact-set（`DEF-011`）。需要 XML 的行业或厂商连接器必须作为签名 provider codec 逐项认证，固定 XSD/version/content-type，使用流式解析和大小/深度/节点预算，禁用 DTD、外部实体、XInclude 和任意 stylesheet/network fetch，并仍通过 `ImportProposal` 或类型化命令；不得新增通用 XML 写旁路。

## 5. 中国大陆驻留与首版本地化

### 5.1 数据驻留 (`NFR-017`)

首版生产客户数据和一切可关联客户的衍生数据只允许在中国大陆境内处理和持久化。范围 exact-set：权威数据库/WAL、附件、索引、日志、审计、导出、临时文件、备份、离线轮换、恢复材料元数据、监控明细、支持诊断包、provider 输入/输出和客户可关联遥测。

签名 DeploymentManifest 与 BackupEvidence 使用各自 strict schema 记录 `jurisdiction=CN`、国家/区域、物理或云 carrier、处理/备份位置、证明摘要、`verified_at` 和 `expires_at`。ProviderManifest 不得复制该字段别名，必须仅使用 ADR-0023 `data_policy.processing_location: ProcessingLocationEvidenceV1`，其中 `residency_profile=CN_MAINLAND_ONLY_V1` 且 `country_or_region.country_code=CN`，并 exact-bind carrier、endpoint、数据类别、证据摘要、验证者和有效期。地点未知、证明过期、跨境 endpoint、跨境日志/遥测、境外备份或支持导出一律失败关闭。公网 DNS、证书服务或包下载不因此获得客户内容；它们只能处理不含客户/部署可关联值的公开材料。

当前首版只接受境内客户自控 P340 物理机；`IAAS_WINDOWS_SERVER_HDD_STRICT` 仅是未来独立 profile 扩展缝，当前必须返回 `PROFILE_NOT_IMPLEMENTED` / `STORAGE_MEDIA_UNVERIFIED` 并禁止进入候选、发布或生产 terminal。未来新 graph/profile version 若获另行授权，IaaS 才须证明客户控制、境内区域、Windows Server 单机且无托管数据库/KMS/队列偷换，并披露 provider/tenant root 可复制内存/磁盘的残余风险；云卷不能证明底层 HDD 介质与缓存边界时仍不得承载 `HDD_STRICT` 正式生产。

### 5.2 语言、币种和业务时间 (`NFR-018`)

- UI、用户错误和客户模板默认且首版唯一产品语言为 `zh-CN`；内部稳定代码、ID 和 API 字段使用英文标识。
- 经营本位币固定 `CNY`，金额存 `numeric(18,2)`/`Money`；第一阶段业务表不提供可编辑币种、汇率或汇兑损益字段。
- 权威时间戳以 UTC 保存；业务自然日和面向用户的默认显示时区固定 `Asia/Shanghai`。持续时间、租约和超时只用 monotonic clock。
- 多语言、多币种、外汇、进出口、报关和信用证不是可通过低代码、插件或隐藏字段启用的首版能力；尝试发布相关核心字段/规则必须被 deferred-capability gate 拒绝。

## 6. 数据保留、法律保留、隐私处置和销毁

### 6.1 对象和优先级

Task 23 必须实现：

```text
RetentionPolicyV1
LegalHoldV1
DispositionCaseV1
DispositionEvidenceV1
ErasureTombstoneV1
```

优先级固定为：有效法律保留 > 不可覆盖的财务/审计/安全保留底线 > 合同/法规保留 > 客户普通保留 > 到期处置。低优先级规则不能缩短高优先级期限；保留期未知时不自动销毁并产生不可抑制异常。

### 6.2 状态机

`LegalHoldV1` 状态闭集为 `DRAFT|APPROVED|ACTIVE|RELEASE_REQUESTED|RELEASED|CANCELLED`。允许边只有 `DRAFT→APPROVED|CANCELLED`、`APPROVED→ACTIVE`、`ACTIVE→RELEASE_REQUESTED`、`RELEASE_REQUESTED→RELEASED|ACTIVE`；RELEASE_REQUESTED→ACTIVE 只表示释放被独立审批拒绝/撤回，hold 始终继续生效并保留决定证据。`RELEASED`、`CANCELLED` 是不可恢复终态，所有未列边拒绝；只有 DRAFT 可取消，APPROVED 之后不能用 CANCELLED 绕过激活/释放链。激活和释放均须不同提交人/审批人、重新认证、范围预览和审计；hold 在 ACTIVE 与 RELEASE_REQUESTED 均有效，不能直接删除或在释放待审时允许处置。

`DispositionCaseV1` 状态闭集为 `PLANNED|IMPACTED|APPROVED|EXECUTING|VERIFIED|CLOSED|FAILED_CONTAINED`。正常边只为 `PLANNED→IMPACTED→APPROVED→EXECUTING→VERIFIED→CLOSED`；PLANNED/IMPACTED/APPROVED/EXECUTING/VERIFIED 任一步失败均可进入 FAILED_CONTAINED。`ResumeDispositionCase` 是 FAILED_CONTAINED 的唯一出边，只能回 EXECUTING，且必须保持同一 case ID、同一已批准 scope/method、批准仍有效，先完成失败影响对账、补齐证据并以幂等 continuation 恢复；不得直接到 VERIFIED/CLOSED，也不得把已部分销毁的数据当作未执行。批准过期、scope/method 变化或无法证明幂等 continuation 时，旧 case 永久保持 FAILED_CONTAINED，并创建带 `supersedes_case_id` 的新 PLANNED case。处置方法闭集：`DELETE_UNREFERENCED`、`PSEUDONYMIZE`、`CRYPTO_ERASE`、`RETAIN_TOMBSTONE`、`DENY_IMMUTABLE`。

财务事实、审批事实、审计链和安全证据不得物理改写；依法需要降低可识别性时使用不可逆假名化/密钥处置并保留最小 tombstone、依据和验证结果。业务表无运行期 `DELETE` grant；可信 lifecycle worker 只消费批准 case 的作用域 handle。

### 6.3 附件、备份与恢复传播

法律保留须固定并传播到对象、附件版本、索引、导出、备份 pin 和相关审计。释放只允许新备份停止携带已合法处置内容；旧备份按自身有效保留期到期，不能为隐私请求破坏可恢复性。任何恢复后必须重放签名 disposition ledger，使已处置数据在恢复环境中保持不可见/被再次处置，不能因恢复“复活”。

## 7. 远程支持生命周期

系统没有永久远程通道。`SupportSessionV1` 状态闭集为 `REQUESTED|APPROVED|READY|ACTIVE|CLOSED|REVOKED|EXPIRED|FAILED_CONTAINED`。允许边固定为：`REQUESTED→APPROVED|REVOKED|EXPIRED|FAILED_CONTAINED`、`APPROVED→READY|REVOKED|EXPIRED|FAILED_CONTAINED`、`READY→ACTIVE|REVOKED|EXPIRED|FAILED_CONTAINED`、`ACTIVE→CLOSED|REVOKED|EXPIRED|FAILED_CONTAINED`；四个终态无出边。撤销、过期或失败可从任一相应前置状态发生，只有 ACTIVE 完成凭据撤销和证据封存后才可 CLOSED；不得用 CLOSED 掩盖失败清理。

请求必须声明人员、客户、用途、系统、对象/字段范围、网络 origin、允许动作、是否可见客户正文、开始/结束时间和工单。默认最长 1 小时，签名策略绝对上限 4 小时；延长须新批准。高密级正文默认不可见，优先使用客户检查、脱敏并导出的诊断包。

激活要求客户控制的 VPN/跳板或一次性反向会话、MFA、重新认证、独立审批和最小短期凭据。支持人员不能取得数据库、KMS、备份销毁或 authority 提升能力；任何例外必须走对应高风险命令，不能写入支持会话。到期、撤销、网络变化或设备不合规立即断开、撤销凭据并生成关闭证据。录屏/命令/文件转移按客户策略审计，秘密和客户正文不得进入普通日志。

`APPROVED→READY` 不是审批命令的隐含副作用，也不能由激活命令跨越。唯一执行者是受控 support provisioner 消费内部强类型命令 `PrepareSupportTransportV1={support_session_id,expected_row_version,approved_scope_digest,credential_policy_digest,network_policy_digest}`；这些字段来自服务端已验证会话、批准快照与签名配置代，外部 HTTP 不得提交。Provisioner 创建一次性凭据和客户控制的 VPN/跳板或反向会话后，必须回读凭据 epoch/到期、网络 origin/目的、最小动作范围与不可到达数据库/KMS/备份控制面的负证据。全部一致时，同一事务追加 `SupportTransportPreparedV1={support_session_id,transport_kind,credential_ref_digest,credential_epoch,network_readback_digest,scope_digest,expires_at,row_version}` 并以 CAS 进入 READY；fact 只存引用/digest，不存秘密。任一创建、回读、持久化或补偿清理失败都撤销已建凭据，进入 FAILED_CONTAINED，并追加 `SupportTransportPreparationFailedV1={support_session_id,failure_code,cleanup_outcome,evidence_ref,row_version}`。`ActivateSupportSession` 只接受 READY，重新验证上述 fact 未过期、网络/凭据 readback 未漂移后进入 ACTIVE；重试使用同一命令 identity，不能重复签发凭据。该内部命令/fact 不新增公开 API discriminator，但必须进入审计、outbox、状态机与崩溃恢复测试。

## 8. 安全事件与漏洞运营 (`SEC-017`)

`SecurityIncidentState` 闭集为 `DETECTED|TRIAGED|CONTAINED|ERADICATING|RECOVERING|RECONCILING|CLOSED|CLOSED_FALSE_POSITIVE`。允许边仅为 `DETECTED→TRIAGED`、`TRIAGED→CONTAINED|CLOSED_FALSE_POSITIVE`、`CONTAINED→ERADICATING`、`ERADICATING→RECOVERING`、`RECOVERING→RECONCILING`、`RECONCILING→CLOSED`。误报只能从 TRIAGED 进入 CLOSED_FALSE_POSITIVE 并保留依据；两个 CLOSED 状态终结后若发现新事实，创建引用旧事件的新 incident，不复活旧记录。严重性采用 `CRITICAL/HIGH/MEDIUM/LOW` 闭集，阈值和 SLA 进入签名 policy；不得由插件自行降低。

每个事件必须保存受影响 deployment/generation/package/SBOM/证书/密钥/账号/数据范围、可信时间、证据位置、containment、轮换、恢复点、对账和客户通知记录。若 authority 可能被攻陷，最后本机日志不能作为唯一证据；先隔离网络和写权，使用服务器外 checkpoint/备份/EDR 证据，轮换会话、证书、密钥和 provider secret，再从 known-clean 点恢复并做业务对账。未完成 reconciliation 不得重新放行高风险写入。

漏洞登记必须把组件/SBOM version range 映射到 deployment generation，产生受影响/不受影响/未知三态。未知不得显示绿色；缓解、补丁、撤回包和重新认证都经签名发布代。

## 9. 完整可移植导出 (`GOV-007`)

完整导出包 `F57PortableExportV1` 固定包含：

- canonical manifest（JCS JSON）、每域 schema/version 和行数；
- UTF-8 CSV 或 canonical JSONL 业务数据，稳定 ID 和引用不丢失；
- 原始附件字节、版本、MIME、长度、digest、密级和关联；
- 配置代、客户 schema、权限/流程/UI/报表/模板、能力包/provider manifest；
- 审计 checkpoint、处置 tombstone、保留/法律保留清单；
- 每文件 digest、整体 Merkle root、签名、客户控制的导出加密 recipient；
- 独立、离线可运行的 verifier 和导入说明。

导出、验证和空环境导入不得依赖有效许可证、厂商在线服务或厂商持有的密钥。Task 23 必须证明在全新空环境中校验、导入、重建引用和附件，并与源环境按域数量、金额、不可变事实、审计 checkpoint 和 Merkle root 对账。非法/未知字段、缺文件、digest 错、引用断裂或不支持 schema 版本一律拒绝部分导入。

## 10. 生产运营补充边界

### 10.1 备份抗耗尽

服务器外只追加 target 必须有按 deployment/writer 的对象数、字节数、速率和并发 quota，以及普通 writer 不可占用的 reserve。target 满、quota 异常突增、partial upload 超时或 generation 爆发进入不可抑制风险状态；writer 不能删除旧对象自救。partial-object 回收由独立低权 maintenance identity 按签名清单执行，不能触碰已完成/已 pin 的对象。

离线介质状态图固定为：`BLANK → ENROLLED → ACTIVE_APPEND → VERIFIED_DISCONNECTED`；到下一轮时 `VERIFIED_DISCONNECTED → ROTATION_DUE → ACTIVE_APPEND`，前提是容量、健康和保留策略仍通过；停止追加时 `ACTIVE_APPEND → SEALED_VERIFIED → RETIRED_PENDING_DISPOSAL → DESTROYED`。`SEALED_VERIFIED` 之后不得回到可写状态。旧 `media_id` 在 `DESTROYED` 后终态不复用；物理介质经双人批准、可验证 crypto-erase/销毁与重新验收后如需再用，必须以新 `media_id` 从 `BLANK` 开始。任何时点至少保留跨越检测窗口的多个已验证 generation，不能用“最新一次成功”覆盖全部 known-clean 历史。

### 10.2 UPS provider

首发支持两级 carrier：`WINDOWS_STANDARD_POWER_STATUS` 提供在线/电池/剩余估算和关机事件的最小证据；`SIGNED_VENDOR_ADAPTER` 额外提供型号、序列号、固件、电池日期、W/VA、self-test。**生产认证只认 `SIGNED_VENDOR_ADAPTER`**（F-63 按 RULING-UPS-01 与 threat-model §296 收严：最高安全档与 `POWER_SHUTDOWN` 只能使用候选绑定的签名适配器，首版唯一生产基线即最高安全档，故 `WINDOWS_STANDARD_POWER_STATUS` 无生产认证路径、仅供开发与非生产环境；本句原为「在线状态、剩余运行时间、通信健康、自检**或等价可验证信号**」的析取式，与严侧三处相反）。生产认证并须能执行 Task 7 固定安全关机顺序；缺字段不得伪造默认值，改为 `CAPABILITY_INSUFFICIENT` 并阻止上线。

### 10.3 20 人聚合负载

20 人容量 envelope 是 Workbench 内部员工、客户门户和供应商门户的合计活跃业务用户；Control Center 管理会话不计入这 20 人，但必须有独立保留资源且与 20 人同时存在。认证至少包含：15 个 Workbench、3 个客户门户、2 个供应商门户，同时 1 个 Control Center 管理会话；再叠加登录/重连 burst、门户附件上传、自动化、增量备份和重报表单并发。第二组保留 P340 档案的 11/5/2/2 业务动作构成。两组都通过才可签发 20 人证书。

### 10.4 Windows 后继 LTSC 边界

Windows Server 2022 是首发认证基线。`NFR-016` 第一阶段交付的是 OS adapter seam、2022 安装/恢复证据、后继 LTSC 探针和签名迁移 playbook；Windows Server 2025/后继 LTSC 的真实硬件认证是 `DEF-010`，不伪装成首发已通过。

Microsoft 官方生命周期页列出的 PT 日期为：Windows Server 2022 主流支持截至 2026-10-13，扩展支持截至 2031-10-14。主流支持结束后签发的生产证书必须附补丁来源、支持策略、客户风险接受和已排期的后继 LTSC 认证；缺一项不得签发新生产证书。扩展支持结束前必须完成迁移；若 P340 驱动/TPM/存储不能认证，则迁移到受支持服务器硬件，而不是降低安全门。

### 10.5 恢复认证策略

`RecoveryCertificationPolicyV1` 的判定固定为：真实部署首次上线前至少完成一次全量洁净服务器恢复与业务对账，只能显示 `INITIAL_RESTORE_VERIFIED`；第二次同 profile 连续成功后进入 `CANDIDATE_MEASURED`；滚动 90 天内第三次连续全量成功、窗口内无失败或未结事故后才可 `CERTIFIED`，证书最长有效 90 天。状态闭集为 `UNVERIFIED|INITIAL_RESTORE_VERIFIED|CANDIDATE_MEASURED|CERTIFIED|EXPIRED|INVALIDATED`，允许边仅为 `UNVERIFIED→INITIAL_RESTORE_VERIFIED`、`INITIAL_RESTORE_VERIFIED→CANDIDATE_MEASURED|INVALIDATED`、`CANDIDATE_MEASURED→CERTIFIED|INVALIDATED`、`CERTIFIED→EXPIRED|INVALIDATED`。首次成功前的失败保留 UNVERIFIED 和失败证据；任何后续演练失败，或硬件、存储拓扑、PostgreSQL build/extension、密钥/保管人、保留策略、数据量级、release/config generation、恢复流程发生变化，立即 INVALIDATED；到 `valid_until` 未重新认证则 EXPIRED。EXPIRED/INVALIDATED 为当前 certification record 的终态，重新认证必须创建新 `certification_id`、引用 predecessor 并从 UNVERIFIED 重新取得三次连续成功，不得复活或沿用旧样本。UI、API 和导出不得把前 3 种冒充承诺。

## 11. 延期能力 exact registry (`GOV-010`)

G0 必须生成 `DeferredCapabilityRegistryV1`，每行字段固定为：`capability_id`、`name`、`disposition`、`canonical_requirement_id`、`allowed_interface`、`forbidden_routes/modules/menus/claims[]`、`activation_adr`、`activation_evidence`。现行边界 RequirementID 是以下 **11 行闭集**，不得使用“至少覆盖”、通配符或未登记别名扩张：

| capability_id | name | disposition | canonical_requirement_id | allowed_interface | 未激活时禁止面 |
|---|---|---|---|---|---|
| DEF-001 | 本地大模型实现 | DEFERRED_WITH_INTERFACE | AI-005 | AI provider、授权、审计、资源和模型版本契约 | 模型包、模型进程、本地推理 route/menu/claim |
| DEF-002 | 完整 MRP/MES/高级排产 | DEFERRED_WITH_INTERFACE | PROC-001 | `ProcurementDemandProvider` 与外部生产连接器 | MRP/MES/APS 模块、排产 route/menu/claim |
| DEF-003 | 大型 WMS/自动化立库 | DEFERRED_WITH_INTERFACE | INV-001 | 库存命令、事件和仓储连接器 | 高级 WMS/自动立库模块、route/menu/claim |
| DEF-004 | 法定总账/税务/工资 | DEFERRED_WITH_INTERFACE | FIN-012 | 经营事实、导出、对账和专业系统连接器 | 法定凭证账簿、申报、工资、法定年结 route/menu/claim |
| DEF-005 | 完整 PPM/EVM | DEFERRED_WITH_INTERFACE | PRJ-004 | 项目、里程碑、成本、收款节点和风险扩展契约 | 完整 WBS/资源/预算变更/EVM route/menu/claim |
| DEF-006 | 主主、双活、多写 | OUT_OF_SCOPE | NFR-008 | 单写 authority discovery、warm-standby fencing | 多写/双活启动、拓扑、route/menu/claim |
| DEF-007 | PostgreSQL 外权威数据库 | DEFERRED_WITH_INTERFACE | DBP-001 | `AuthoritativeDatabaseProvider` seam；当前只认证 PostgreSQL 16 | 非 PG authority 启动、迁移和 current claim |
| DEF-008 | 任意原生 DLL 热注入 | OUT_OF_SCOPE | PKG-003 | 签名内核维护升级及 WASM/受控 worker/container adapter | DLL 注入、未签名 native load、热替换 route/menu/claim |
| DEF-009 | 寄售/订阅/租赁完整销售闭环 | DEFERRED_WITH_INTERFACE | SAL-008 | 三类类型化 sales provider seam 与禁用证据 | 三类业务模块、route/menu/claim 和营销声明 |
| DEF-010 | Windows Server 2025/后继 LTSC 实机认证 | DEFERRED_WITH_INTERFACE | NFR-016 | OS adapter seam、探针和签名迁移 playbook | 后继 LTSC 已认证 claim、生产证书和默认 carrier |
| DEF-011 | 通用 XML/SOAP/XSD 交换面 | DEFERRED_WITH_INTERFACE | INT-002 | 签名 provider codec seam | 通用 XML route、任意对象 hydration、XML-to-SQL 和 current claim |

`capability_id` 必须与本表 11 个值逐字一致；`canonical_requirement_id` 必须逐字匹配本表并存在于 185 行需求种子。`activation_adr` 和 `activation_evidence` 在未激活时都固定为 `REQUIRED`，不得为空或写成“不适用”。[业务执行契约 §16.1](2026-08-23-f57-business-execution-contract.md#161-当前不得激活或宣称可用) 的 12 个产品文字 token 是 operational alias 闭集，每个通过 canonical RequirementID 归入本表或现行稳定业务需求；它们不是第 12 个及以后的边界 RequirementID。

延期项在未激活前必须同时满足：不可安装、不可见、不可调用、不可通过低代码/插件绕行、不可在产品材料宣称已支持；只允许登记的接口/禁用证据存在。恢复延期项必须先新增 ADR、稳定需求、owner task、测试和签名证据，不能只改一个开关。

## 12. 发布验收矩阵

### 12.1 渐进客户端认证

客户端产品契约始终是统一 Employee API、CapabilityGraph 投影和 UI schema，业务语义不得按平台分叉。交付档位固定如下：

- `CTC-01 / DEV_SLICE_GREEN`：先认证最小 Windows Workbench；macOS/iOS/Android 可并行做安全存储、文件、相机、扫码、通知、签名、安装和性能 probe，但未认证平台不得发布生产包；
- `INTEGRATION_GREEN`：按 capability 的 `first_due_profile` 补齐需要的平台和 provider，不得用 Windows 结果替代其他端；
- `RELEASE_CERTIFIED`：一旦产品对外宣称 Windows/macOS/iOS/Android 通用，四端安装、企业签名、升级、最低版本、撤销、DLP、性能、可访问性和同一业务结果必须全部取证。

每个平台的“全部取证”不是一份可自由解释的测试报告，而是主计划唯一 `ClientPlatformGateEvidenceV1` 聚合内嵌的八份、八类各一份独立签名 `ClientPlatformLifecycleEvidenceV1`：`PACKAGE_SIGNATURE`、`INSTALL_START`、`UPGRADE`、`REVOCATION`、`CAPABILITY`、`RESOURCE`、`DLP`、`ACCESSIBILITY`。八份证据必须逐项重复同一栈、平台、可信 runner、绑定、协议、包字节、签名身份和时间上下文并全部 `PASS`；缺失、额外、重复、跨包/跨平台/跨运行、类别与 details 不匹配或用通用附件代替类型化证据均失败关闭。内嵌使平台聚合成为唯一持久化输出，不再产生 32 个需要猜路径的旁路文件；发布验收只消费该权威聚合，不从日志目录猜测证据。

客户端栈选择也不是多跑几次再挑最好结果。首次架构认证只允许主计划定义的一份 `ArchitectureDecisionAttemptV1` 和一条签名、严格前缀扩展的 attempt journal；四个平台在同一 attempt 下各执行一次。`UNKNOWN` 只表示本次尝试不能得出结论，不能当作失败后重跑或替换平台结果；任一终态 `FAIL` 选择既定 Flutter + Rust fallback。选定分支将已经验证的决策信封逐字节复制到唯一提交路径 `docs/decisions/f57-client-stack-decision.v1.json`，后续 G5/G6 只接受该提交路径和精确摘要，禁止重新基准测试、重新签名或另选一份“更优”决策。

Tauri 2 未通过技术门时只触发既定 Flutter + Rust fallback 的计划修订，不改变服务器 capability contract。完整离线 intent/conflict engine 不属于 CTC-01；在线 Windows 切片只保留版本化 seam，离线能力仍须在其 `first_due_profile` 到期时完整实现和认证。低档客户端通过不得提升为生产分发声明。

| 主题 | Owner Task | 必须证据 |
|---|---:|---|
| 员工 API/四端协议 | 18 | 同一 IDL 的四端 contract、会话/撤权/游标/错误/分片负例 |
| 客户签名分发 | 17 | 四平台真实签名、安装、更新、撤销、证书失窃和回退 evidence |
| 终端 DLP/离线 | 18 | 受管/不合规/Root/Jailbreak、截图/剪贴板/分享、cache copy/rollback/wipe receipt |
| Provider/处理位置 | 14 | ADR-0023 exact schema、四层交集、驻留/资源/排空/Unknown |
| 数据生命周期 | 23 | hold precedence、双人释放、附件/备份传播、restore 后 tombstone 重放 |
| 远程支持 | 16 | 无永久通道、短期授权、到期/撤销、字段/网络 scope 和完整审计 |
| 安全事件 | 24 | 隔离、外部证据、轮换、known-clean 恢复、业务对账和再放行 |
| 完整导出 | 23 | 无许可证空环境校验/导入/对账，客户持钥 |
| 驻留/本地化 | 24/25 | CN region evidence、跨境负例、zh-CN/CNY/Asia-Shanghai/UTC 存储 |
| 后继 LTSC | 24 | 首发 seam/playbook boundary PASS；真实认证保持 `DEFERRED_WITH_INTERFACE` |
| 恢复证书 | 24/25 | 首次洁净恢复、滚动 90 天三次连续成功、失效条件和 UI/API 诚实性 |

任一行只有静态文档、mock 或自述而没有对应平台/数据库/设备/恢复证据时，状态仍是 `NOT_IMPLEMENTED` 或 `UNVERIFIED`。
