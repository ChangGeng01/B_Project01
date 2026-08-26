# 配置参考

> **F-57 总体开发状态（2026-08-23）：`READY_NOT_AUTHORIZED`；本文件生成式配置登记仍为 `REGISTRY_PENDING_REBASELINE` / `NOT_IMPLEMENTED`。** 下文 328/337 旧闭集、固定九进程、AI 例外、`NONE` 病毒扫描和任何可能把客户数据写到 `C:\ProgramData`/系统 SSD 的路径，不再是完整现行配置。只有用户另行明确授权后，才可依据 [F-57](superpowers/specs/2026-08-23-f57-governed-automation-fabric-design.md) 和本节部署清单规则从 G0 开始实现并再基线；当前不得执行开发。

## F-57 不可覆盖的部署清单

以下字段属于安装时签名部署清单，不是环境变量、命令行或普通运行期配置。它们必须在 Windows 服务安装前确定，并在启动、升级和发布认证时复验：

| 字段 | 现行要求 |
|---|---|
| `deployment_id` | 客户部署唯一标识，必须与许可、配置代、备份和审计一致 |
| `software_volume_id` | 256GB SSD 的稳定设备/卷标识；只承载 signed Set A 的 Windows、程序、静态依赖、可重建模型/metadata及 exact 四类 mutable Set B（POWER capsule、package-recovery capsule、kernel pointer/head、signed native-code slot/cache），任何第五类或客户/业务 authority 字节失败 |
| `software_root` | SSD 上只读安装目录；不得作为 temp、spool、日志、导出或客户文件目录 |
| `data_volume_id` | 1TB HDD 或后续 RAID1 数据卷的稳定设备/卷标识，不能只用盘符判断 |
| `data_root` | HDD 上的加密权威数据根；PostgreSQL data/WAL、附件、索引、日志、temp、spool、导出和插件工作区均由此派生 |
| `backup_target_ids` | `F57AuthorityStorageManifestPayloadV1` 中服务器外备份目标的稳定标识；最高档必须 exact 为 `[current BackupTopologyV1.continuous_target.target_id]` 单元素向量，同机卷、普通可写 SMB、VM snapshot 或离线 A/B 介质不能冒充该连续目标 |
| `postgres16_windows_install_contract_ref` | 指向 Task 11 唯一 strict `Postgres16WindowsInstallContractV1`；由当前 `WindowsAuthorityArtifactSetV1` 认证，禁止安装员、环境变量或命令行改写 service/path/config/TLS 投影 |
| `backup_topology_signing_trust_current_ref` | 指向当前签名 `BackupTopologySigningTrustCurrentPointerV1`；pointer typed-load 唯一 `BackupTopologySigningTrustManifestV1`，由部署 bootstrap 固定的独立 trust-manifest authority 验证并按 generation/predecessor 单调推进。该 manifest 独占固定 topology signer DN/SPKI、离线证书链、撤销快照和 transparency checkpoint；禁止复用应用恢复域、备份恢复域、2-of-3 recipient/share roster、候选 signer 或 ambient Windows root |
| `backup_topology_ref` | 指向当前签名 `BackupTopologyV1` 唯一 head；仅由以上 active-config current trust pointer/manifest 构造的私有 `BackupTopologyAuthorityV1` 验证固定 signer。genesis 为 revision 1/null predecessor，后续必须 prior+1 且 exact 引用前一完整 envelope；topology exact-repeat trust current/manifest refs，storage manifest 的 `backup_target_ids` 在最高档恰为 singleton `[continuous_target.target_id]`。固定六角色、服务器外连续目标、恰好 A/B 两块离线 HDD、保留/容量及凭据/故障/管理/保管域，不得由 `sink.*`、目录扫描、时间戳或旧签名对象临场替代 |
| `ups_adapter_manifest_ref` | 指向当前 strict `UpsAdapterManifestV1`；`implementation_binary_ref` 固定为候选 authority-kernel。manifest 冻结 carrier、canonical GUID/device instance 或可按部署签名定制的 numeric-IP-octets/nonzero-port structured endpoint、最小能力和 credential policy；正数 `configuration_projection.configuration_generation` 与 `SHA256(JCS(configuration_projection))` 唯一绑定 selected profile/outlet/P340 power path。冻结 `5/15/86400/30` 秒时限，禁止固定站点 IP、DNS、文本 IP 别名、ambient 发现或配置覆盖 |
| `hardware_profile_id` | 当前物理基线唯一 `THINKSTATION_P340_I5_10500_32GB_256GB_SSD_1TB_HDD`，与 runtime topology/P340 wire byte-equal。未来新 graph/profile version 若启用 IaaS，必须使用其独立 recipe 签发的 provider/VM profile ID，不能伪装为 P340。变更磁盘、内存、CPU/provider SKU、数据规模或启用本地模型后必须生成新 profile 并再认证 |
| `authority_epoch` | 单写权威代；未来暖备提升必须在 fencing 后产生新值 |
| `infrastructure_certification_profile_ref` | 当前首版只接受物理 `SINGLE_DISK_DEGRADED_PRODUCTION` 认证证据；`IAAS_WINDOWS_SERVER_HDD_STRICT` 仅保留为未来独立 profile 标识，当前选择必须以 `PROFILE_NOT_IMPLEMENTED` / `STORAGE_MEDIA_UNVERIFIED` 失败关闭。未来启用须先发布新 graph/profile version，且不得填入或复用 P340 hardware/UPS ref。 |
| `postgres_log_retention_policy_ref` | 指向已签、不可由 `postgresql.conf`、环境或 CLI 覆盖的 PostgreSQL 日志保留政策；普通目标 30 日/20 GiB，当前日志不删、至少 7 日，legal hold 优先，只有 `EPAuthorityControl` typed cleanup 可执行 |
| `employee_api_origin` | `ep-data-migrate` 的唯一公开迁移 API origin；必须为无 path、query、fragment 的 HTTPS origin，校验证书链、SAN 和清单 host；拒绝 loopback/localhost、直连 core-server、命名管道、重定向、系统代理以及 CLI/模板覆盖 |

`data_root` 的规范子目录至少包括 `postgres/data`、`postgres/wal`、`postgres/temp/process`、`postgres/temp/restore`、`files`、`audit`、`indexes`、`logs`、`temp`、`spool`、`exports`、`plugin-work`、`quarantine`、`dumps` 和 `backup-staging`。`postgres/data` 是 PGDATA 并包含 live `pg_wal` 与数据库临时关系；`postgres/wal` 只作 WAL archive staging；`postgres/temp/process` 只作 PostgreSQL 进程 TEMP/TMP，`postgres/temp/restore` 只作恢复 scratch。程序不得自行回退到 `%TEMP%`、`%ProgramData%`、用户 profile 或当前工作目录，也不得使用 `initdb --waldir`、用户 tablespace 或 reparse descendant 绕开 DATA_HDD。

Windows pagefile、WER dump、服务 dump、恶意文件 quarantine 和包含业务载荷的诊断输出必须禁用持久化或明确路由到加密 HDD。Windows Event Log 只允许记录稳定事件码和随机、不可关联客户或对象的 `incident_id`；客户值、对象 ID、客户正文哈希、可反查 digest、附件、提示或秘密一律禁止。

密钥材料不得保存为 SSD/HDD 明文文件。生产凭据密文进入 HDD 上的产品加密秘密库；只有非导出的 wrapping handle 可以位于客户批准的 TPM、HSM 或 KMS。不得把客户凭据正文持久化到 SSD 上的 WinCred、Windows 服务 profile 或普通文件；业务密文和密钥元数据进入 HDD 权威数据库。

生产附件扫描固定为 `REQUIRED_PROVIDER`：扫描服务不可用、超时或未知时保持隔离。旧 `NONE` 只能用于开发/测试且无真实客户数据的环境，不能形成生产配置或认证证据。

`THINKSTATION_P340_I5_10500_32GB_256GB_SSD_1TB_HDD` 固定关闭本地模型、重报表并发为 1，并对 OCR、批量导入导出、低优先级自动化和维护实施有界队列。第 21 名活跃用户不被硬拒绝，但容量证书外的低优先级任务可以节流。

本文件是全部配置键的唯一登记处。代码侧的配置结构体与本文件由 CI 项 `xtask configdoc` 逐键比对，缺一即失败。新增配置键的顺序是先登记后使用。F-56 的许可证和声明式模块包不新增运行期配置键：生产验签信任包路径固定为 `C:\ProgramData\EnterprisePlatform\trust\license-roots.p7b`，其摘要、ACL、轮换和失败关闭规则属于签名产品/部署协议，不能由配置、环境变量或命令行覆盖。

## 1. 加载顺序与总则

五层覆盖，后者覆盖前者：内置默认、主配置文件、片段目录（按文件名字典序）、环境变量、命令行参数。该顺序只适用于 development/test profile。F-57 production 拒绝每一个 `EP__*` 环境变量和每一个会 shadow 配置值的 CLI 参数；非变更型 `--help`、`--version` 以及显式 self-check/validation 命令不是配置覆盖。生产有效行为只能来自已验签 deployment manifest 与已验签 active configuration generation，不存在第二份可变本地配置来源，也不得从主配置、片段、环境变量或普通 CLI 回退补值。

- 配置结构体开启 `deny_unknown_fields`。未知键一律拒绝启动，不忽略、不警告了事。写错一个键名却照常启动，是最难排查的一类故障。
- 类型错误的错误消息含键路径，指出是哪一个键。
- 仅 development/test profile 的环境变量映射为双下划线分段并全大写，前缀 `EP__`。例：`db.pool.rw_max` 对应 `EP__DB__POOL__RW_MAX`；production 出现该变量即拒绝。
- 「生效方式」一列只有两种现行语义：启动表示改动后需重启对应 Windows 服务；取用表示在下次取用该值时生效。本版不提供 SIGHUP 或其他热加载入口。

## 2. 登记表

下表是首版全部 **328 个配置登记族**：每个表格登记行计 1 个族，其中 `db.timeout.<池>.*` 的 3 个占位族分别展开 `rw|ro|worker|ops` 四值，因此生成供代码逐键比对的具体点分键共 **337 个**（`328 - 3 + 3×4`）。阶段 1–14 的现行配置表已在 F-54 做环境变量引用与点分键全量对账，F-55 追加 16 个本地 AI/MCP 登记族。迁移期望清单属于签名 PE 的编译期事实，不登记运行期配置键；明确作废的别名与只表示前缀的通配写法也不计入上述口径。

### 2.1 HTTP

| 键 | 类型 | 默认值 | 生效方式 |
|---|---|---|---|
| http.bind_addr | string | 按进程固定，见第 3 节 | 启动 |
| http.max_body_bytes | u64 | 1048576 | 启动 |
| http.request_timeout_ms | u32 | 8000 | 启动 |
| http.shutdown_drain_ms | u32 | 30000 | 启动 |
| http.concurrency_limit | u16 | 20 | 启动 |
| http.concurrency_wait_ms | u32 | 10000 | 启动 |

`http.request_timeout_ms` 的 8000 与普通同步业务等待上限 8 秒同源，超时返回 `PLATFORM.SYSTEM.SYNC_TIMEOUT`。只有两个编译期具名 route profile 不读该键：AI compose 使用独立 45-slot 数据面、Tower 122000 ms/内部推理 120000 ms；`POST /mcp` 使用独立公平全局 16-slot 再按 connector 4-slot、Tower 32000 ms/协议绝对 30000 ms。两者都不占、不借普通 `http.concurrency_limit=20`，其他 route 不得声明第三种例外。`http.concurrency_limit` 与 `http.concurrency_wait_ms` 是普通并发闸门的两个参数，等待超时返回 `PLATFORM.CAPACITY.CONCURRENCY_LIMIT`；拒绝次数由通用 HTTP 请求指标按状态码与路由统计，不另设 quota 指标。

### 2.2 IPC

| 键 | 类型 | 默认值 | 生效方式 |
|---|---|---|---|
| ipc.socket_path | path | 按服务端进程固定，见下 | 启动 |
| ipc.max_frame_bytes | u32 | 1048576 | 启动 |

服务端进程与 `ipc.socket_path` 是一一对应的封闭映射：core-server=`\\.\pipe\ep-core`、integration-gateway=`\\.\pipe\ep-integ`、plugin-host=`\\.\pipe\ep-plugin`、ai-inferer=`\\.\pipe\ep-ai`；其他进程不得创建产品业务管道。值与映射不符即拒绝启动，不提供 HTTP endpoint、端口、别名或第二条 IPC 配置。四条管道均固定 `reject_remote_clients=true`；每个 server generation 仅 bootstrap 首实例取 `first_pipe_instance(true)`，同进程后续/补位实例一律取 `first_pipe_instance(false)`。服务端在读取应用字节前冒充客户端，以线程 token 的服务 SID/账户执行逐项 operation allowlist，PID 只作审计关联；客户端发送前校验服务端进程 token。账户与 operation 任一不符立即拒绝并审计。实现白名单必须逐字符串列出，不得使用通配规则。

- `ep-core` server=`NT SERVICE\ep-core`；客户端 ACE 只含 `ep-portal|ep-archive|ep-backup|ep-ops`。`ep-portal` 只可调用 `portal.session.sign_in.v1`、`portal.session.sign_out.v1`、`portal.identity.me.v1`、`portal.order_confirm.v1`、`portal.delivery_notice.v1`、`portal.invoice_upload.begin.v1`、`portal.invoice_upload.chunk.v1`、`portal.invoice_upload.end.v1`、`portal.invoice_upload.abort.v1`、`portal.settlement_query.v1`、`portal.profile_maintain.v1`；前三项为门户身份操作，后八个 operation 承载五项门户业务能力。`ep-archive` 只可调用 `ops.attachment_writeout_scope.query.v1`、`ops.writeout_result.report.v1`、`ops.failure_event.report.v1`、`ops.replication_lifecycle.report.v1`。`ep-backup` 只可调用 `ops.writeout_result.report.v1`、`ops.verification_conclusion.report.v1`、`ops.failure_event.report.v1`、`ops.replication_lifecycle.report.v1`、`ops.attachment_checksum_verdict.report.v1`、`ops.backup_slot.acquire.v1`、`ops.backup_slot.release.v1`。`ep-ops` 只可调用 `health.get.v1`、`metrics.snapshot.v1` 与 `ops.signed_artifact.install_receipt.v1`；最后一项只接受 F-55 strict 安装收据，不是任意路径/文件登记 API。
- `ep-integ` server=`NT SERVICE\ep-integ`；客户端 ACE 只含 `ep-worker|ep-core|ep-ops`。`ep-worker` 只可调用 `push.dispatch.v1`、`esign.request.submit.v1`、`esign.status.get.v1`、`mcp.remote.exchange.v1`，并只在同一已关联签章双工连接接收 gateway 反向发送的 `esign_file.begin.v1`、`esign_file.chunk.v1`、`esign_file.end.v1`、`esign_file.abort.v1`；`ep-core` 只可调用 `virus_scan.begin.v1`、`virus_scan.chunk.v1`、`virus_scan.end.v1`、`virus_scan.abort.v1`、`mcp.remote.exchange.v1`；`ep-ops` 只可调用 `health.get.v1`、`metrics.snapshot.v1`。
- `ep-plugin` server=`NT SERVICE\ep-plugin`；客户端 ACE 只含 `ep-core|ep-worker|ep-ops`。`ep-core` 与 `ep-worker` 只可调用 `wasm.execute.v1`、`mcp.local.exchange.v1`；`ep-ops` 只可调用 `health.get.v1`、`metrics.snapshot.v1`。取消以 deadline 或断开当前调用表达，不设第二个 cancel operation。
- `ep-ai` server=`NT SERVICE\ep-ai`；客户端 ACE 只含 `ep-core|ep-ops`。`ep-core` 只可调用 `ai.query_plan.compose.v1`、`ai.model.activate.v1`、`ai.model.deactivate.v1`；`ep-ops` 只可调用 `health.get.v1`、`metrics.snapshot.v1`。其他账户没有 ACE。

四条管道的抗占满常量不进配置，首版固定如下。`ep-core` 总实例上限 32、账户活跃连接上限为 portal=20/archive=4/backup=4/ops=2；`ep-integ` 总实例上限 16、worker=8/core=4/ops=2；`ep-plugin` 总实例上限 12、core=4/worker=4/ops=2；`ep-ai` 总实例上限 51，compose 数据面 core=45、模型控制面 core=2、ops=2，余 2 只用于 accept/补位；compose 额度等于运行 15 加排队 30，控制面不得借给数据面。身份核验后发现账户/用途额度已达上限时，服务端在读取完整应用 payload 前返回固定 `PLATFORM.IPC.CONCURRENCY_LIMIT`，AI compose 管道返回 `AI.INFERENCE.CONCURRENCY_LIMIT`，并写不含正文的安全审计。

首个实例与身份核验也固定为协议而非配置。每个 server generation 启动时先创建一个带 `first_pipe_instance(true)` 的首实例以抢名并 fail-closed；成功后只有持有该首实例的同一服务进程可创建同名补位实例，补位一律 `first_pipe_instance(false)`。首实例句柄贯穿整个 listener 生命周期，断开后在同一句柄重新进入接受；首句柄异常丢失时整个服务退出并由 SCM 重启，不得以 false 实例续命。客户端以 `SECURITY_SQOS_PRESENT|SECURITY_IDENTIFICATION` 打开管道，禁止 server 借用调用方权限。server 在读取任何应用字节前依次执行 `ImpersonateNamedPipeClient`、`OpenThreadToken` 核验允许的服务 SID/账户、无条件 `RevertToSelf`；PID 只用于审计关联，不作为授权事实。client 在发送前以 server PID 与进程 token 核验预期服务账户，发送失败后的每次重连都必须重新核验，不得复用旧结论。

普通调用的连接身份握手上限 5 秒、首个长度前缀上限 10 秒、连接空闲上限 30 秒、单次调用绝对上限 120 秒；半帧、慢帧、超长帧或断连立即清零缓冲并关闭。`BoundedChunkStreamV1` 仍用逐块 ACK 10 秒、空闲 30 秒、会话绝对 3600 秒，且 3600 秒不能由普通调用的 120 秒截断。

普通 IPC 帧固定为 4 字节大端长度前缀加 JSON，整帧不超过 1 MiB。`mcp.remote.exchange.v1` 与 `mcp.local.exchange.v1` 唯一使用 F-55 `McpExchangeChunkStreamV1`：decoded chunk 最大 512 KiB、逐块 ACK、manifest/request/response 分段、连续序号与端到端摘要，request/response 总上限分别 1 MiB/8 MiB、绝对时限 30 秒；它不提高普通帧上限，也不新增 operation。该 DTO 不复用大文件 `BoundedChunkStreamV1` 的 3600 秒会话语义。产品进程间不得恢复内部 HTTP 路径、回环 TCP 或新增同义 operation；唯一回环 TCP 例外是 integration-gateway 作为客户端连接客户同机 ICAP。

病毒扫描分块协议由 core 调用上述四个 `virus_scan.*` operation：begin DTO 为 `{request_id(UUIDv7), attachment_object_id, total_len(0..5368709120), content_sha256}`，哈希是 32 字节 SHA-256 的 64 位小写十六进制；chunk DTO 为 `{request_id, seq, data_b64, chunk_sha256}`，`seq` 从 0 起连续，块哈希同形，Base64 解码后每块 1 至 524288 字节（仅总长为 0 的空文件没有数据块），每块必须收到同 `request_id` 与 `ack_seq` 的 ACK 后才可继续且最多一块在途；end DTO 为 `{request_id, next_seq, total_len, content_sha256}`。gateway 必须校验连续序号、累计长度与滚动 SHA-256 后才返回 `PASS|REJECT|ERROR`；乱序、重复、缺块、累计长度、块哈希或最终哈希不符立即 abort。块 ACK 超时 10 秒、空闲超时 30 秒、会话绝对上限 3600 秒；取消或超时必须发送 abort、关闭对应 ICAP 会话并清零缓冲，重试使用新 request_id。gateway 只保留单块加协议开销的有界内存且不落盘。只有 gateway 到客户自管扫描器的一跳可使用 `127.0.0.1|[::1]` ICAP；产品进程之间不得使用回环 TCP。

电子签章普通回执固定为两种：submit 回执 `{request_id, external_request_id, outcome: ACCEPTED|FAILED, provider_code?, retryable}`；status 回执 `{external_request_id, status: PENDING|SIGNED|REJECTED|EXPIRED|FAILED, provider_code?, retryable, signed_files:[{file_ordinal, sanitized_name, mime_type, total_len, content_sha256}]}`。两者只含清洗后的稳定码，不含服务商原始响应。状态为 SIGNED 后，gateway 对 `signed_files` 按 `file_ordinal` 逐个在同一双工连接反向执行 `esign_file.*`：begin DTO 固定为 `{request_id(UUIDv7), external_request_id, file_ordinal, total_files, sanitized_name, mime_type, total_len(0..5368709120), content_sha256}`；chunk、end、abort、逐块 ACK、512 KiB 上限、序号、哈希和 10/30/3600 秒超时完全复用 `BoundedChunkStreamV1`。worker 把整批文件接入阶段 3 的完整附件流水线，固定顺序为临时加密对象→长度/哈希/TYPE_SNIFF/STRUCTURE→按 `NONE|CUSTOMER_ICAP` 部署模式执行病毒扫描→电子签章验签→数据库确认/发布；外部状态 SIGNED 不等于文件安全。只有整批全部 PUBLISHED 才建立签章关联，任一步失败均清理未确认临时件、已建对象保持 QUARANTINED，合同不得转 SIGNED。gateway 只保留单块有界内存，不落盘、不写附件元数据。

integration-gateway 的运行期数据库能力固定为零：没有 `ep_app_rw` 或其他数据库凭据，没有数据库配置或连接池，不读写文件库元数据，不持 KMS 凭据，不消费 Outbox。推送、签章与病毒扫描的全部业务结果只作为管道回执返回；分别由 worker 或 core 在其权威事务内落库。

### 2.3 数据库连接

| 键 | 类型 | 默认值 | 生效方式 |
|---|---|---|---|
| db.host | string | 127.0.0.1 | 启动 |
| db.port | u16 | 5432 | 启动 |
| db.database | string | ep | 启动 |
| db.user | string | ep_app_rw | 启动 |
| db.password_ref | string | secret://db/app_rw#1 | 取用 |

`db.password_ref` 写的是引用而不是口令本身，见第 5 节。

生产中的 PostgreSQL 16 安装参数不是本节的可覆盖配置键。唯一 owner 是 `crates/platform/backup/src/postgres16_windows.rs`，唯一 schema 是 `docs/evidence/f57-postgres16-windows-install.v1.schema.json`；它拥有五个 strict plain root：19-field package lock、13-field install contract、4-field Event Log fixture set、19-field Event Log scan coverage 与 17-field install readback。contract 的 `server_component_set_ref` 必须回指同一 artifact set 的六组件集合；其六字段 Event Log scan contract typed-load fixture ref/digest，service-install evidence 则认证 readback 及 coverage ref。package-lock 的 `installed_files` 必须完整列出 engine root 下每个普通文件的 canonical relative path、重开长度和 SHA-256，并与离线包、SBOM 及 engine final-handle 独立重枚举形成双射；扩展名必须与锁定包 `.control`、SBOM 和 available/installed/enabled 集合 exact-match。V1 固定 `downgrade_allowed=false`，只允许 clean install 或 package-lock、包/文件集摘要、版本/control/catalog 全部 byte-equal 的 same-lock adopt/repair；发现任何不同的旧版或新版都必须先返回 `MAINTENANCE_UPGRADE_REQUIRED`，不得改服务或数据。

九个路径角色按 enum 顺序唯一出现：engine 在 SSD，PGDATA/config 与 live WAL、archive staging、`postgres/temp/process`、`postgres/temp/restore`、日志和 TLS 在 DATA_HDD，读回与投影一一对应且只允许已声明的 PGDATA/config、PGDATA/live-WAL 重叠。签名策略保存未解析 `canonical_sddl_template`/摘要；只有 PostgreSQL service account、固定 `EPF57Recovery` account 与 backup-writer service identity 创建或采用后才解析 SID、应用 ACL，并把九个 live `canonical_dacl_sddl`/摘要与模板计算结果逐字节比对。`ep-postgres16` 固定为 `NT SERVICE\ep-postgres16`、`UNRESTRICTED` service SID、`DEMAND_START`、`NO_AUTOMATIC_RESTART`、`WIN32_OWN_PROCESS`，只在当前 boot 的 DATA_HDD 解锁、storage manifest、vault、配置和 TLS 全部通过后显式启动；在此之前 process-start count 必须为零，ready 只接受 typed `RUNNING`。engine readback 的 `cluster_system_identifier|pg_control_system_identifier|sql_system_identifier` 与外层 `postgres_system_identifier` 必须都是相同的非空规范无符号十进制字符串。

关键 GUC、HBA 与 ident 使用冻结的排序/序号 canonical vectors，effective vectors 必须逐字节相等；监听集合严格为排序去重后的 `127.0.0.1|::1`。连接上限固定 `max_connections=64|reserved_connections=4|superuser_reserved_connections=3`，两槽不可分配安全余量固定为 2；每个数据库消费者声明 `NORMAL|RESERVED|SUPERUSER`，并分别校验 `N+2<=57`、`R<=4`、`N+R+2<=61`、`S<=3`、`N+R+S+2<=64`。应用只能是 NORMAL 且既非 superuser 也无 `pg_use_reserved_connections`；migration 才可是 RESERVED 且非 superuser、持有该预留角色；recovery 才可是 SUPERUSER。HBA 只证明 loopback `hostssl` 与 `scram-sha-256`，`channel_binding=require` 必须由逐 consumer authenticated client probe 证明已配置且协商成功，不能从 HBA 推断。有效配置还须 exact-readback checksums、`fsync=on`、`full_page_writes=on`、`synchronous_commit=on`、`wal_sync_method=fsync_writethrough`、`wal_level=replica`、`archive_mode=on`、固定签名归档执行器和 `temp_tablespaces=''`，并拒绝 `trust`、外部 CIDR、ambient include 与 `postgresql.auto.conf` 覆盖。`fsync_writethrough` 只是 PostgreSQL 16/Windows 兼容性 pin；生产启用前，installed-file-verified 的 `pg_test_fsync.exe` 必须对同一 DATA_HDD 测试文件分别取得 `fsync` 与 `fsync_writethrough` 支持、正吞吐、零 I/O error，并绑定工具、卷、驱动栈和 write-cache policy；Task 15 还必须把这些身份与同候选 P340 UPS/write-cache 及受控 HDD flush/power-cut 证据 exact-join，任一变化即失效。日志固定 collector→stderr→HDD；Windows Event Log 的零客户 token 只能由 typed 完整扫描证明：`Application` channel、两个固定 provider registration、同 boot 起止 bookmark/record ID/time、零 clear/drop/unexplained gap、fixture ref/digest 与完整执行计数全部匹配且 `coverage_complete=true`，缺失、截断或错配即拒绝。

### 2.4 连接池与超时

| 键 | 类型 | 默认值 | 生效方式 |
|---|---|---|---|
| db.pool.rw_max | u16 | 20 | 启动 |
| db.pool.ro_max | u16 | 10 | 启动 |
| db.pool.worker_max | u16 | 5 | 启动 |
| db.pool.ops_max | u16 | 2 | 启动 |
| db.pool.acquire_timeout_ms | u32 | 8000 | 启动 |
| db.pool.max_lifetime_s | u32 | 1800 | 启动 |
| db.pool.idle_timeout_s | u32 | 300 | 启动 |
| db.timeout.<池>.statement_ms | u32 | 逐池取值见下 | 启动 |
| db.timeout.<池>.lock_ms | u32 | 3000 | 启动 |
| db.timeout.<池>.idle_in_tx_ms | u32 | 15000 | 启动 |
| db.ro.work_mem_kb | u32 | 65536 | 启动 |
| db.ro.temp_file_limit_kb | u32 | 2097152 | 启动 |
| db.retry.max_attempts | u8 | 3 | 启动 |
| db.retry.backoff_ms | u32 数组 | [50,150,450] | 启动 |
| db.budget.resident_max | u16 | 37（历史测量种子） | 启动 |
| db.budget.temporary_max | u16 | 10（历史测量种子） | 启动 |
| db.budget.peak_max | u16 | 52（历史测量种子） | 启动 |

`<池>` 取 `rw`、`ro`、`worker`、`ops` 四值。`statement_ms` 的逐池默认值取阶段 1 计划第 7.2 节的池表：rw 10000、ro 60000、worker 300000、ops 5000。`lock_ms` 与 `idle_in_tx_ms` 四池同值。integration-gateway 没有池种类、配置或连接；旧 `integ` 值不得继续解析。`db.ro.temp_file_limit_kb` 保留取值登记但不在会话级下发：`temp_file_limit` 为 SUSET 参数，应用角色无权 SET，该限额由引导侧角色默认值承接（db/bootstrap/03_role_defaults.sql）。

`db.retry.*` 只对尚未产生任何外部可见副作用的事务生效，触发条件为 SQLSTATE 40001 与 40P01；两键在进程启动时构造重试策略，修改后须重启对应 Windows 服务。

`db.pool.*` 的 `20/10/5/2` 与 `db.budget.*` 的 `37/10/52`（另含 5 个安全余量）只记录 ADR-0018 旧四池拓扑的历史默认/测量种子，不是 F-57 的不可变产品真值，也不能直接形成生产放行值。依 [ADR-0019](adr/ADR-0019-f57-runtime-topology-and-measured-connection-budget.md)，Task 1 必须先登记签名 deployment/config generation 的连接消费者 exact set，拒绝未知或重复消费者，再按真实硬件、拓扑和并发负载重测常驻、临时/迁移/恢复与不可分配安全储备并签发容量证书；硬件、拓扑或代改变即重测。启动与迁移开窗分别按该代已认证预算校验，超限以退出码 78 拒绝。迁移预期版本清单与摘要由 `migration_manifest` 在构建期嵌入签名 PE；运行期只有数据库中的实际历史可读，不提供路径、环境变量或命令行参数覆盖期望值。

`ep-data-migrate` 的目标入口也不是运行期配置键。生产签名部署清单 schema v1 必须含唯一 `employee_api_origin`，值只能是无路径、查询与片段的 HTTPS origin；工具经第三方反向代理调用公开迁移 API，并校验证书链、SAN 主机名与清单 host。回环/localhost、直连 core-server:8080、命名管道、重定向、系统代理及命令行/迁移模板覆盖均拒绝。迁移块最多 1000 行且规范化 JSON 请求体最多 524288 字节，含 HTTP 封套的完整请求仍不得超过 `http.max_body_bytes=1048576`；本段不增加配置键或路由级 body 例外。

### 2.5 日志、指标与追踪

| 键 | 类型 | 默认值 | 生效方式 |
|---|---|---|---|
| log.level | string | info | 启动 |
| log.debug_auto_off_minutes | u16 | 30 | 启动 |
| metrics.enabled | bool | true | 启动 |
| metrics.bind_addr | string | 按进程固定，见第 3 节 | 启动 |
| trace.sample_ratio | f32 | 0.1 | 启动 |
| trace.otlp_enabled | bool | false | 启动 |
| trace.otlp_endpoint | string 可空 | null | 启动 |

`log.debug_auto_off_minutes` 是 debug 级别的自动回落时长，避免有人临时开了 debug 之后忘记关掉。

PostgreSQL 文本日志不使用本表 `log.*` 作为保留权威，也不新增可被环境/命令行放宽的键。`postgres_log_retention_policy_ref` 的签名政策固定：`max_age_seconds=2592000`、`max_total_bytes=21474836480`、`minimum_retained_age_seconds=604800`、`delete_current_log=false`；计数包含 SERVER_LOG 下所有 closed/rotated PostgreSQL 日志并用 final-handle 去重，当前打开文件、7 日内文件及 legal-hold object 永不进入删除集。唯一 owner 是既有 `EPAuthorityControl` 中的 typed `POSTGRES_LOG_RETENTION_CLEANUP`；请求必须带 policy ref、trusted time、严格按 `(closed_at,path,digest)` 排序的预览清单、legal-hold readback、空间 readback 和双人批准，执行后逐项重开验证已删/保留集及前后 digest 并入审计。`NT SERVICE\ep-postgres16`、backup writer 和普通 Authority 的 ACL 必须拒绝删除历史、改 ACL、解除 legal hold 或改政策。legal hold 导致 30 日/20 GiB 无法同时满足时，保留被保护文件并进入 fail-closed 容量处置，不得从 current/7-day/held 集删除。

DATA_HDD 的有效批量暂停门为 `max(existing yellow_free, 50 GiB)`，有效全局 hold 门为 `max(existing red_free, 40 GiB)`。当前 P340 已有 `emergency_reserve=max(20 GiB,capacity×5%)`、`yellow_free=max(2×reserve,30-day P95 growth)` 和 `platform.file.free_space_min_bytes=107374182400`，因此实际必须取更严格值，不得用新 50/40 GiB 底线将约 1TB P340 放宽到低于现行约 100/50 GiB 门。

### 2.6 机密与自检

| 键 | 类型 | 默认值 | 生效方式 |
|---|---|---|---|
| secrets.dir | path，部署清单派生 | `{data_root}\secrets` | 启动；必须位于已验证 HDD volume |
| secrets.provider | enum：kms | kms | 启动 |
| selfcheck.clock_skew_max_ms | u32 | 1000 | 启动 |

### 2.7 运行时

| 键 | 类型 | 默认值 | 生效方式 |
|---|---|---|---|
| runtime.worker_threads | u16 | 0，表示按整机可用逻辑核数推导 | 启动 |
| runtime.blocking_threads | u16 | 32 | 启动 |

### 2.8 出网

| 键 | 类型 | 默认值 | 生效方式 |
|---|---|---|---|
| egress.allowlist | string 数组 | 空 | 启动 |
| egress.connect_timeout_ms | u32 | 3000 | 启动 |
| egress.request_timeout_ms | u32 | 15000 | 启动 |
| egress.ca_bundle_path | path | `{data_root}\config\ca\provider-ca.pem` | 取用；客户 provider 信任材料属于客户配置数据 |
| egress.breaker.failure_threshold | u16 | 5 | 启动 |
| egress.breaker.open_ms | u32 | 30000 | 启动 |
| egress.breaker.half_open_probes | u8 | 1 | 启动 |

出网默认白名单为空，即默认不允许出网。`egress.ca_bundle_path` 为 Windows 原生 PE 进程使用的显式信任根；TLS 客户端使用 rustls，不依赖机器证书库，也不存在现行 OCI/scratch 基础镜像语义。

UPS 不复用本节通用 `egress.allowlist` 作为运行时放宽入口。`WINDOWS_STANDARD_POWER_STATUS` 固定无控制能力且只作监测，其 manifest 设备 profile 列表为空、status profile 为 null、identity 只代表 carrier/configuration；最高安全档必须使用签名 `SIGNED_VENDOR_ADAPTER` 并精确命中一个非空设备 profile。manifest 的 `implementation_binary_ref` 必须 byte-equal 候选 `authority_kernel_binary_ref`，重开的实现摘要还必须与运行中 held binary 相等。`configuration_projection.configuration_generation` 为正，`adapter_configuration_sha256=SHA256(JCS(configuration_projection))`；manifest、identity、状态、命令和 ACK 必须重复相同 generation/digest，普通文件或稍后覆盖不能改变选择。

USB carrier 固定零网络且设备 ACL 只授 SYSTEM 与 `EPAuthorityControl` service SID；网络 carrier 的唯一允许项来自 `UpsAdapterManifestV1.transport_policy` 的 structured endpoint，字段为 numeric-IP octets、nonzero port、protocol 与 pinned peer identity。它可随客户部署签名定制，不固定某个 IP；DNS、文本 IP 别名、proxy、redirect 永久为 false。适配器 credential 只能使用 service-SID 限定的不可导出 CNG client key 或 DPAPI-NG sealed secret locator；不新增 endpoint、credential、command、argv 或 vendor-payload 配置键。`5/15/86400/30` 秒依次是 poll interval、status 最大年龄、provider self-test 最大年龄和 command ACK 最大等待，均为 manifest/schema 冻结值；POWER 的 `600s` 只用于复合事件与崩溃对账，不能覆盖 30 秒限制。provider 调度返回的 `provider_operation_id` 必须是匹配 `[A-Za-z0-9][A-Za-z0-9._:-]{0,127}` 的 1..128 字节 canonical ASCII，并在 ACK 前耐久绑定 `(ups_adapter_identity,command_id,command_sha256)`；schedule/query/log 三方必须 byte-equal，空值、别名、变化或跨命令复用均进入 `COMMAND_STATE_UNKNOWN` 且禁止重发。

UPS runtime-loss 行为同样不进配置表。最新 status 在 15 秒到期时立即创建 deployment-wide `ProductionAdmissionHoldV1`，拒绝新 request/long-running job；从该首次过期 monotonic tick 起只有一个不可重置的 60 秒恢复窗。只有同 `ups_adapter_identity`、configuration generation/digest 和 runtime binding 的两个连续、sequence 递增、各自 fresh 的通信/self-test/output/runtime PASS 才能由 admission CAS 撤销 hold。否则无论 AC 字段为何，都进入本地 checkpoint→停 PostgreSQL→Windows safe shutdown。无可控 outlet 或无 typed ACK 只表示不能声称 provider 接受，不得阻止本地安全关机；下次启动必须人工处置并重新采集设备/电源/DATA_HDD/PostgreSQL 证据后才能重开。

### 2.9 spool

| 键 | 类型 | 默认值 | 生效方式 |
|---|---|---|---|
| spool.dir | path | `{data_root}\spool\<proc>` | 启动 |
| spool.max_bytes | u64 | 21474836480 | 启动 |

本通用二键只由 archive-writer 与 backup-writer 使用，部署模板分别展开为 `{data_root}\spool\archive-writer` 与 `{data_root}\spool\backup-writer`，两者上限均为 21474836480；不存在第二套 `archive.spool_*` 或 `backup.spool_*` 键。`archive.wal_spool_max_gb` 是 WAL 正文暂存上限，与本节 IPC 报文 spool 不同，不得互相覆盖。

F-55 MCP completion 使用的不是本通用 spool，也不新增配置键。core-server/job-worker 的固定目录、各 1 GiB/1024 个 1 MiB 预留 slot、严格 record、DACL、原子重放与 fail-closed 规则只以 F-55 §4.7 的 `McpAuditCompletionSpoolV1` 编译期常量实现；`spool.dir`、`spool.max_bytes` 不得指向或放大该目录。

报文 spool 不允许静默丢关键证据。`WriteoutResultReport`、`VerificationConclusionReport`、`FailureEventReport`、`ReplicationLifecycleReport`、`AttachmentChecksumVerdictReport` 五类均为 critical，只追加并在 core 确认入库后截断；`AttachmentWriteoutScopeQuery` 与 `BackupSlotAcquire/Release` 是需即时应答的控制请求，不落 spool，core 不可用时不得启动对应新周期。只有从本地写出 manifest 与落点对象清单可确定性重建的 `HEARTBEAT|PROGRESS_SNAPSHOT` 本地进度记录是 reconstructible，允许按 `(record_kind, object_id)` 只保留最新一条。

软停止水位固定为 `spool.max_bytes - 67108864`，末 64 MiB 只供在途周期的 critical 关闭/失败/复制生命周期报文。到达软水位后继续接收 WAL 并完成当前写出，但不得启动新全量备份或附件写出周期；写 Windows Event Log。若到达硬上限也不得删除或覆盖 critical 报文；恢复连接后先重放并让 core 以 `WRITER_NOT_IN_SERVICE`、subject=`<writer>:report-spool-exhausted` 打开不可抑制窗口，重放完成且低于软水位才关闭。普通运行过程中更新文件采用排他创建/追加、flush、原子 manifest 切换，恢复按 `(occurred_at, report_id)` 幂等重放。

### 2.10 密钥管理

| 键 | 类型 | 默认值 | 生效方式 |
|---|---|---|---|
| kms.backend | enum：builtin、hsm | builtin | 启动 |
| kms.builtin.master_key_path | deprecated path | 空；F-57 生产出现非空即拒绝 | 启动 |
| kms.hsm.pkcs11_module | string | 空 | 启动 |
| kms.hsm.slot | u32 | 0 | 启动 |
| kms.hsm.pin_ref | bootstrap 引用 | `bootstrap://windows-dpapi/hsm-pin#1` | 取用 |

`kms.backend` 取 `builtin` 时也不得生成普通 `master.key` 文件；客户批准的 TPM 2.0 non-exportable wrapping handle 包装 HDD vault 中的用途/法人 data key。`kms.builtin.master_key_path` 只为拒绝旧配置而保留，生产必须为空。取 `hsm` 时不回落内置实现，`kms.hsm.*` 指向不存在的硬件后端即以 78 失败。`kms.hsm.pin_ref` 是建立 KMS 之前的短期 bootstrap 引用，PIN 不落 SSD、HDD、argv、环境变量或日志。

盲索引宽度不属于部署配置：`KmsBackend::derive_blind_key` 固定返回 `BlindIndex([u8; 32])`，数据库固定存 32 字节。配置结构不得登记盲索引字节数或按字段覆盖宽度；唯一性由各业务字段的索引约束单独表达。

### 2.11 迁移窗口

| 键 | 类型 | 默认值 | 生效方式 |
|---|---|---|---|
| migration.window_ttl_max_min | u32 | 240 | 启动 |

`migration.window_ttl_max_min` 是开窗端点可接受的窗口存续时长上限，默认 240 对齐 02 计划第 12 节假设四（基线迁移执行上限 30 分钟加一倍余量，取规格第 12.1 章应急账号 8 小时的一半以示更严）；该键在服务启动时载入，修改后须重启。

### 2.12 平台内核

| 键 | 类型 | 默认值 | 生效方式 |
|---|---|---|---|
| platform.idempotency.retention_days | u32 | 7 | 启动 |
| platform.file.virus_scan.mode | enum：REQUIRED_PROVIDER | REQUIRED_PROVIDER；生产固定 | 启动 |
| platform.file.virus_scan.icap_url | legacy url，可空 | 空；F-57 provider manifest 接管 | 启动 |

`platform.idempotency.retention_days` 是幂等键定稿行的保留天数（03 计划表 12）：`platform_msg.idempotency_keys` 的行以 `expires_at = created_at + 保留天数` 落库，过期行由保留期清理扫描物理删除；core-server 与 job-worker 双进程生效，development/test 环境变量为 `EP__PLATFORM__IDEMPOTENCY__RETENTION_DAYS`。

生产扫描模式固定为 `REQUIRED_PROVIDER`；经批准的 Defender/AMSI/ICAP provider 由签名 provider manifest 选择，不由普通配置扩大网络或权限。provider 不存在、不可用、超时或结论未知时附件保持隔离。旧 `NONE` 只属于历史非生产配置，F-57 release gate 遇到即拒绝；旧 `icap_url` 键保持为空并在 Task 1 从运行期闭集移除。

### 2.13 授权判定

| 键 | 类型 | 默认值 | 生效方式 |
|---|---|---|---|
| authz.snapshot.poll_interval_ms | u32 | 2000 | 取用 |
| authz.decision.explain_enabled | bool | false | 启动 |
| authz.scope.max_department_depth | u8 | 8 | 取用 |
| authz.scope.in_list_threshold | u16 | 200 | 取用 |
| authz.export.sensitive_row_threshold | u32 | 1000 | 取用 |

`authz.snapshot.poll_interval_ms` 是授权快照轮询重载的间隔，重载体查 `authz_config_versions` 的 EFFECTIVE 版本号，版本变化即整体替换按法人分片的快照；checksum 不符时开降级窗口而不是继续用旧快照。`authz.scope.max_department_depth` 是记录级范围中部门闭包展开的深度上限，超限截断并计 `ep_authz_scope_truncated_total`；`authz.scope.in_list_threshold` 是部门集合超过该阈值后 IN 列表退化为 EXISTS 子查询的开关点。`authz.export.sensitive_row_threshold` 是导出含敏感字段行数的阈值。development/test 环境变量前缀为 `EP__AUTHZ__`。

`authz.export.sensitive_row_threshold` 的允许范围固定为 `1..=1000`，默认 1000；只允许调低以收紧敏感导出分类，不允许高于 F-51 U-B-18/U-I-11 的 1000 行硬阈值。格式错误、0 或大于 1000 时 core-server、job-worker 与 `--check` 均拒绝启动。

### 2.14 活跃用户规模观测

F-51 U-L-01 的首版值为编译期常量：统计最近 60 秒内有请求的不同用户；超过 20 人不拒绝登录、不排队、不拒绝写入，只记录、告警并标记性能 SLA 不适用；管理端每 5 秒刷新；单用户最多 3 个有效会话。这里不设 `admission.*` 配置，避免部署方把规模观测改造成另一套业务准入规则。`http.concurrency_limit` 与 `http.concurrency_wait_ms` 仍是按瞬时 HTTP 请求数保护进程资源的独立闸门，不得以活跃用户数驱动。

### 2.15 身份域

| 键 | 类型 | 默认值 | 生效方式 |
|---|---|---|---|
| auth.password.min_length | u8 | 12 | 启动 |
| auth.password.min_char_classes | u8 | 3 | 启动 |
| auth.password.max_age_days | u16 | 90 | 启动 |
| auth.password.history_size | u8 | 5 | 启动 |
| auth.password.argon2.memory_kib | u32 | 65536 | 启动 |
| auth.password.argon2.iterations | u32 | 3 | 启动 |
| auth.password.argon2.parallelism | u32 | 1 | 启动 |
| auth.lockout.max_failures | u8 | 5 | 启动 |
| auth.lockout.window_seconds | u32 | 900 | 启动 |
| auth.lockout.duration_seconds | u32 | 1800 | 启动 |
| auth.session.ttl_seconds | u32 | 28800 | 启动 |
| auth.session.idle_timeout_seconds | u32 | 1800 | 启动 |
| auth.session.max_per_user | u8 | 3 | 启动 |
| auth.session.sliding_write_granularity_seconds | u32 | 60 | 启动 |
| auth.reauth.ttl_seconds | u32 | 300 | 启动 |
| auth.reauth.max_failures | u8 | 3 | 启动 |
| auth.totp.skew_steps | u8 | 1 | 启动 |
| auth.webauthn.rp_id | String | 无默认 | 启动 |
| auth.webauthn.origins | String序列 | 无默认 | 启动 |
| auth.x509.trust_anchor_ref | SecretRef | secret://pki/client_ca#1 | 启动；F-56 fresh production 还要求 resolved bundle digest 命中 signed deployment manifest |
| auth.breakglass.max_session_seconds | u32 | 28800 | 启动 |
| auth.breakglass.idle_rotation_days | u16 | 365 | 启动 |

`auth.x509.trust_anchor_ref` 解出的不是自由 PEM/单证书。exact bytes 最大 1,048,576，格式为 DER empty-content/no-signer CMS，内含 1..64 CA certificates 与 1..256 完整 base CRL；整链、最高覆盖 CRL、证书/CRL算法参数、critical-extension 与禁止 OS trust/network fallback 的规则取身份阶段/F-56 唯一口径。core-server 以 `ep-core` recipient 解出的 SHA-256 必须等于已验签 `deployment.manifest.v1.jcs.x509_login_trust_bundle_sha256`；fresh bootstrap 时 ep-migrate 以 `ep-migrate` recipient 解出的 exact bytes/digest 也必须相等。两 recipient 内容漂移、manifest digest 不等或 bundle 不可解析均退出 78；不得回落 Windows 根、命令行证书或临时 root。

身份域二十二键取 04 计划 §7 配置表的全键（阶段 4 任务 #21 登记），一律启动时生效：把安全参数做成热生效开关会在运行期制造不经配置发布通道的旁路。口令与锁定诸键的默认值已由 F-51 将 U-B-14 冻结为首版现行值（12/3/90/5 与 5/900/1800），不再是临时取值；`auth.session.ttl_seconds` 与 `auth.session.idle_timeout_seconds` 只影响新会话；`auth.webauthn.rp_id` 与 `auth.webauthn.origins` 必填无默认，缺失即启动自检失败；`auth.breakglass.max_session_seconds` 只能调小不得调大（规格第 12.1 章 8 小时上限）。敏感取值不进本段：X.509 登录 trust bundle 用 `SecretRef`；TOTP seed 不用 secret://，而是逐 credential 以法人 FIELD/L40 EPC1 存在 `user_credentials.secret_enc`。仅 development/test 使用 `EP__AUTH__*` 映射；production 值来自签名 active configuration generation。

### 2.16 F-50 发票与历史成交

| 键 | 类型 | 默认值 | 生效方式 |
|---|---|---|---|
| invoice.tax.amount_tolerance | decimal(18,2) | 0.02 | 启动 |
| mdm.trade_history.include_ineffective | bool | false | 启动 |

`invoice.tax.amount_tolerance` 的 development/test 环境变量为 `EP__INVOICE__TAX__AMOUNT_TOLERANCE`，允许区间固定为 `0.00..=0.02`；缺失取默认值，格式错误、负数或大于 `0.02` 一律拒绝启动。它只用于普通发票行税额与 `round_half_up(net_amount × tax_rate, 2)` 的绝对差校验，不适用于价税合计等式，也不放宽 F-50 第 6.4 节纯税额更正的剩余税额上限。

`mdm.trade_history.include_ineffective` 的 development/test 环境变量为 `EP__MDM__TRADE_HISTORY__INCLUDE_INEFFECTIVE`。置 true 只允许历史成交终态记录出现在查询结果中，不改变 `is_selectable_as_price_source`；服务端在真正选用价格前仍须重读来源状态。

### 2.17 主数据与价目表（阶段 5）

| 键 | 类型 | 默认值 | 生效方式 |
|---|---|---|---|
| mdm.import.max_rows | u32 | 5000 | 启动 |
| mdm.import.template_version | string | 与二进制版本绑定 | 启动 |
| mdm.code.allow_manual | bool | true | 启动 |
| mdm.uscc.checksum_enabled | bool | true | 启动 |
| mdm.uscc.exempt_customer_types | string 数组 | `["INDIVIDUAL","OVERSEAS"]` | 启动 |
| mdm.name_duplicate.probe_limit | u32 | 20 | 启动 |
| mdm.qualification.expiry_lead_days | u32 | 30 | 启动 |
| mdm.qualification.scan_enabled | bool | true | 启动 |
| mdm.trade_history.max_rows | u32 | 20 | 启动 |
| mdm.trade_history.window_months | u32 | 12 | 启动 |
| mdm.freeze.require_probe_when_module_enabled | bool | true | 启动 |
| cpq.price_resolve.max_lines | u32 | 200 | 启动 |
| cpq.price_list.expiry_scan_enabled | bool | true | 启动 |

`mdm.trade_history.include_ineffective` 已登记在第 2.16 节，阶段 5 不重复登记。`mdm.import.max_rows` 固定对应 F-51 U-A-09 的主数据 5000 行上限。`mdm.freeze.require_probe_when_module_enabled` 首版只接受 true；配置为 false 时 core-server、job-worker 与 `--check` 均拒绝启动，不能把模块启用前的必需探针检查降级为可选行为。

### 2.18 合同与销售（阶段 6）

| 键 | 类型 | 默认值 | 生效方式 |
|---|---|---|---|
| clm.esign.base_url | string | 无默认 | 启动 |
| clm.esign.credential_ref | SecretRef | `secret://provider/esign/api#1` | 取用；正文密文位于 HDD vault |
| clm.esign.request_timeout_ms | u64 | 10000 | 启动 |
| clm.esign.poll_interval_seconds | u64 | 60 | 启动 |
| clm.esign.poll_max_hours | u64 | 168 | 启动 |
| clm.esign.circuit_breaker.failure_threshold | u32 | 5 | 启动 |
| clm.esign.circuit_breaker.open_seconds | u64 | 120 | 启动 |
| clm.derivation.item_timeout_ms | u64 | 5000 | 启动 |
| clm.derivation.max_items_per_contract | u32 | 2000 | 启动 |
| clm.template.render_timeout_ms | u64 | 8000 | 启动 |
| clm.contract.max_lines | u32 | 500 | 启动 |
| sales.credit.exposure_query_timeout_ms | u64 | 2000 | 启动 |
| sales.order.max_lines | u32 | 500 | 启动 |
| sales.delivery_schedule.max_per_line | u32 | 60 | 启动 |
| sales.return.max_lines | u32 | 200 | 启动 |

`clm.esign.base_url` 只表示 integration-gateway 到外部签章 provider 的 HTTPS URL，不是产品进程间地址；缺失时只关闭电子签章载体并登记降级窗口，合同仍可走实体印章或人工上传已签文件的受控路径。该键非空时还必须命中获批 provider manifest/egress grant，否则签章能力保持关闭。job-worker 到 gateway 始终使用受控本地 IPC。`clm.esign.credential_ref` 只引用 HDD 产品秘密库中的 envelope ciphertext；服务取得调用级短期内存 handle，配置、普通 IPC、日志和数据库业务表均不得出现凭据明文。初始化与轮换走服务器控制中心的双人高风险命令和 secret broker，不经普通业务 API。

### 2.19 成本、指标与报表（阶段 11）

| 键 | 类型 | 默认值 | 生效方式 |
|---|---|---|---|
| reporting.analytic.max_result_rows | u32 | 2000 | 启动 |
| reporting.analytic.max_drill_page_size | u16 | 200 | 启动 |
| reporting.analytic.max_period_span | u16 | 36 | 启动 |
| reporting.analytic.sync_budget_ms | u32 | 8000 | 启动 |
| reporting.advanced_sql.enabled | bool | true | 启动 |
| reporting.advanced_sql.max_query_bytes | u32 | 16384 | 启动 |
| reporting.advanced_sql.max_join_count | u8 | 8 | 启动 |
| reporting.advanced_sql.max_subquery_depth | u8 | 4 | 启动 |
| reporting.render.max_concurrency | u8 | 2 | 启动 |
| reporting.render.timeout_seconds | u32 | 300 | 启动 |
| reporting.render.max_export_rows | u32 | 50000 | 启动 |
| reporting.render.artifact_ttl_days | u16 | 7 | 启动 |
| costing.capture.reject_unbound_cost_leg | bool | true | 启动 |

`reporting.render.max_export_rows` 的允许范围为 1 至 50,000，只能把部署上限调低，不能突破 F-51 U-B-18/U-I-11 的硬上限。`costing.capture.reject_unbound_cost_leg` 首版只接受 true；false 为非法配置，core-server、job-worker 与 `--check` 均拒绝启动，不存在以 WARN 代替捕获失败的生产或排障旁路。报表结果缓存、物化与 `data_as_of` 粒度都不是配置项：F-51 U-I-06 固定为实时查询且精确到秒。

### 2.20 售后、项目与客户 360（阶段 12）

| 键 | 类型 | 默认值 | 生效方式 |
|---|---|---|---|
| service.work_order.max_lines_per_order | u16 | 200 | 启动 |
| service.work_order.reminder_timer_enabled | bool | true | 启动 |
| service.equipment.create_from_delivery_max_rows | u16 | 200 | 启动 |
| project.derivation.max_tasks_per_contract | u16 | 500 | 启动 |
| project.derivation.plan_query_timeout_ms | u32 | 3000 | 启动 |
| crm.customer_360.default_section_size | u16 | 20 | 启动 |
| crm.customer_360.max_section_size | u16 | 50 | 启动 |
| crm.customer_360.section_timeout_ms | u32 | 1500 | 启动 |
| crm.customer_360.provider_concurrency | u8 | 5 | 启动 |

`crm.customer_360.default_section_size` 不得大于 `crm.customer_360.max_section_size`；后者不得超过 50。`crm.customer_360.provider_concurrency` 的允许范围为 1 至 5，不得占用超过只读池一半的连接预算。工单提醒阈值、四张 service 字典、报表定义与审批链属于事务数据库中的签名配置对象，不得另加同义部署键。

### 2.21 运维、备份与恢复（阶段 14）

| 键 | 类型 | 默认值 | 生效方式 |
|---|---|---|---|
| sink.kind | enum：LOCAL_DIR、NFS_SMB_MOUNT、OBJECT_STORAGE | 无默认，必填 | 取用 |
| sink.root | string | 无默认，必填 | 取用 |
| sink.credential_ref | secret 引用 | 无默认，必填 | 取用 |
| sink.restore_credential_ref | secret 引用 | 无默认，必填 | 取用 |
| sink.disposal_credential_ref | secret 引用 | 无默认，必填 | 取用 |
| sink.media_type | enum：ONLINE、OFFLINE、NONE | 无默认，必填 | 取用 |
| sink.rotation_period_minutes | u32 可空 | null；OFFLINE 时必填 | 取用 |
| sink.probe_interval_seconds | u32 | 60 | 取用 |
| sink.unwritable_after_failures | u8 | 3 | 取用 |
| sink.writable_after_successes | u8 | 2 | 取用 |
| sink.readback_throughput_min_mibps | u32 | 无默认，由认证报告冻结 | 启动 |
| archive.slot_name | string | ep_archive_slot | 启动 |
| archive.wal_spool_max_gb | u32 | 350 | 启动 |
| archive.retention_warn_ratio | decimal(9,6) | 0.600000 | 取用 |
| archive.wal_writeout_period_seconds | u32 | 300 | 取用 |
| archive.attachment_incremental_period_seconds | u32 | 300 | 取用 |
| archive.audit_evidence_period_seconds | u32 | 300 | 取用 |
| archive.suspend_after_minutes | u32 | 30 | 取用 |
| backup.mode | enum：normal、restore | normal | 启动 |
| backup.restore_plan_path | path 可空 | null；restore 时必填 | 启动 |
| backup.full_schedule | cron | `0 1 * * *` | 取用 |
| backup.attachment_full_schedule | cron | `0 3 * * *` | 取用 |
| backup.spill_max_bytes | u64 | 53687091200 | 取用 |
| backup.bootstrap_deadline_hours | u32 | 无默认，必填 | 取用 |
| backup.verify_decrypt_sample_ratio | decimal(9,6) | 0.050000 | 取用 |
| backup_encryption.dbek_ref | secret 引用 | 无默认，必填 | 取用 |
| backup_encryption.algorithm | enum：AES_256_GCM | AES_256_GCM | 启动 |
| ops.metrics_listen | socket | 127.0.0.1:9101 | 启动 |
| ops.health_listen | socket | 127.0.0.1:9102 | 启动 |
| ops.wal_retention_sample_period_seconds | u32 | 30 | 取用 |
| ops.capacity_sample_period_seconds | u32 | 300 | 取用 |
| ops.disk_watermark_ratio | decimal(9,6) | 0.800000 | 取用 |
| key_recovery.verification_interval_days | u32 | 183 | 取用 |
| key_recovery.shard_pickup_sla_hours | u32 | 无默认，必填 | 取用 |

development/test 环境变量由表中点分键机械转换为 `EP__` 加双下划线分段，例如 `archive.wal_spool_max_gb` 对应 `EP__ARCHIVE__WAL_SPOOL_MAX_GB`；production 出现任何此类变量即拒绝。`archive.max_slot_wal_keep_gb` 的旧名已撤销，不登记别名；数据库侧 `max_slot_wal_keep_size` 必须与 `archive.wal_spool_max_gb` 同值，由自检读回核对。

本表仍只有“启动”和“取用”两种加载语义。“变更后重判”是附加发布守卫，不是第三种热加载协议：新落点类型、地址或介质结论在重新完成落点判定和整机恢复演练前不得成为有效认证值。阶段 14 不实现 SIGHUP 或目录监听。

F-57 production 不从上述 `sink.*` 行推断合格的防勒索拓扑；这些行只作为 G0 导入与 development/test 历史输入。生产必须同时接受 active-config 明确选择的 `backup_topology_signing_trust_current_ref` 与 `backup_topology_ref`。前者 typed-load 独立签名的 `BackupTopologySigningTrustCurrentPointerV1` 和唯一 `BackupTopologySigningTrustManifestV1`；部署 bootstrap 固定 trust-manifest authority，manifest 再固定 topology signer `CN=EP F57 Backup Topology Authority,O=Enterprise Platform` 的 SPKI/DN、离线链、撤销和 checkpoint。唯一 `BackupTopologyAuthorityV1` 只能由该 verified-current trust 值构造；拓扑、storage/support evidence、候选发布 signer、应用/备份恢复域、ADR-0020 2-of-3 recipient/share roster 或 ambient Windows root 均不能自证。pointer/manifest generation/predecessor 与 topology revision/predecessor 必须各自连续；有效时间和 CMS signing window 必须通过，topology 还要 exact-repeat current trust refs，并与 current `authority_storage_manifest_ref` 的 deployment/epoch/generation exact-join。最高档 storage manifest 的 `backup_target_ids` 必须恰为 `[continuous_target.target_id]`；目录扫描、时间戳、fork、过期旧 head 或配置回滚均失败。

拓扑必须为 `E={PRODUCTION,CONTINUOUS,ROTATION_A,ROTATION_B}` 的每个实体提供 `failure|administration|credential|custody|location` 五个非空 domain ID。对任意域 `d` 和任意两个不同实体 `x,y`，验证器都必须直接校验 `domain[d,x] != domain[d,y]`；五组各有 6 个 pair，共 30 个不等式，不能以 `shared_* = false` 替代。同 tenant/root 或管理组、同 SPKI/secret/recovery credential、同宿主/机房/电源故障边界、同 custody roster 或同位置是必测负例。

每次 install、checkpoint preparation、PITR、activation/retry 都必须新采 strict `StorageSafeguardReadbackV1`；target 与介质 subordinate refs 必须按 kind typed-load topology-pinned 单签/双签 `StorageSafeguardSupportEvidenceV1`，不能相信 Boolean 或任意 `ArtifactRefV1`。全新安装的 expected/latest/head、连续 retained refs 与 A/B verified refs 均为空，只能是带不可变 `INITIAL_POPULATION` transition 的 `INITIALIZING`；它可通过基础设施安装，但不能授权 PITR、发布、恢复认证或生产。cut 后、draft 前的 fresh preparation 由空链推导 sequence 1/prior null；首个签名 head 使下一次 fresh readback 进入 `BOOTSTRAPPING`，必须先把当前 head 复制并验证到 A/B。只在 distinct continuous 与 A/B-union 代数仍低于 minimum、A/B 已非空且并集含当前 head、其余健康条件全部成立时，`BOOTSTRAPPING + INITIAL_POPULATION` 才可按已验证 head checked+1 创建 sequence 2 或后续代；达到最小代数且新 head 的 A/B 验证闭合后，下一次 fresh readback 必须成为 `HEALTHY` 且 transition 为空。

正常 trust/topology/storage current roots 只能从 fresh `HEALTHY` 串行轮换：在 active-config CAS 前先建立 deployment-wide `ProductionAdmissionHoldV1`，禁止新请求/长任务，排空全部 accepted lease 并提交 `write_barrier_id`；一个 CAS 再固定旧 head/旧 roots 与新 roots，进入 `TRANSITIONING`。只允许创建一份以旧 head checked+1 为序号、绑定新 roots 及同一 hold/barrier 的 bridge checkpoint。随后状态必须是 `BOOTSTRAPPING + CURRENT_ROOTS_ROTATION`，禁止再创建 checkpoint，只能完成该 bridge 的 A/B 离线复制与验证，闭合后回到 `HEALTHY`。hold 必须跨越全部 `TRANSITIONING|BOOTSTRAPPING`；只有 fresh `HEALTHY`、transition 为空、head exact-bind 新 tuple 且同一 deployment-wide admission CAS 重新核验 epoch/OBSERVED generation/零旧 lease 才可重开。第二次轮换在恢复 `HEALTHY` 前禁止。所有 retained refs 都须 typed-load 为唯一连续 `BackupCheckpointV1` 链，current head exact-bind current trust/topology/storage tuple；`INITIALIZING|BOOTSTRAPPING|TRANSITIONING|NON_SUPPRESSIBLE_RISK` 一律禁止普通 PITR、发布、恢复认证与 production activation。

`DATA_HDD_DISASTER_REPLACEMENT` 是与 normal rotation 分离的闭合恢复协议，不要求已死旧盘的 fresh `HEALTHY`。它不从本机目录/最新文件推断输入，只接受服务器外 current configuration/trust 和最后已认证 checkpoint/cut；双人恢复授权后必须依次完成 global hold、旧 authority fencing、checked `authority_epoch`/storage generation 提升、新 physical/volume identity 与 BitLocker、新 storage manifest、洁净恢复/PITR/全数据核对、连续备份与 A/B 空链 bootstrap 到 fresh `HEALTHY`，再运行当前 P340 容量认证。这是 hold 下的灾难 PITR 特例，不给普通 `NON_SUPPRESSIBLE_RISK` 状态任何 PITR 权限；唯一接管点是最后的 admission CAS，之前一直关闭。未来 IaaS graph/profile version 必须另行冻结其独立重认证链，当前不得调用或拼接 IaaS recipe。

有效保留秒数必须 checked-equal `max(site_legal_retention_seconds,7776000,2*measured_detection_lag_p99_seconds+clean_restore_validation_window_seconds,2*offline_rotation_interval_seconds)`，离线最大代际年龄不超过 `604800` 秒且至少保留两个已验证代际。连续目标必须同时满足 required-total 公式以及真实 total/free/quota/reserve 不等式，离线盘满足 recoverable+validation+growth；所有算术 checked，partial count/bytes/oldest optionality 一致且 expired 为零。生产启用时按序 `ROTATION_A|ROTATION_B` 只能为 `VERIFIED_DISCONNECTED|SEALED_VERIFIED`，所有 live domain/location exact，具有零 attachment、撤销设备授权、安全弹出、物理断开、健康、exact 两个分域 human custodian，且不含 recovery material。上述规则均为签名部署/证据协议，不能通过配置键降低；任何缺口产生 `NON_SUPPRESSIBLE_RISK`。

历史数据迁移不新增 `data_migration.*` 配置键；每块同时受「最多 1000 行」与「规范化 JSON 请求体不超过 524288 字节」约束，两者先到为准，包含 HTTP 封套的整个请求不超过 1 MiB；租约 60 秒、心跳 20 秒、一次性会话 10 分钟与模板 schema v1 均为首版协议常量。`PATCH_STATUS_REPORT_INTERVAL_DAYS` 是合同模板参数而非软件配置，允许 1 至 7 个自然日、默认 7，不得出现在本表、环境变量或配置结构体中。

### 2.22 数据基础补充（阶段 2）

| 键 | 类型 | 默认值 | 生效方式 |
|---|---|---|---|
| kms.dek_cache.max_entries | u32 | 512 | 启动 |
| kms.dek_cache.ttl_s | u32 | 300 | 启动 |

两键只控制进程内已解封 DEK 缓存，修改后须重启持有缓存的 Windows 服务；不得把缓存刷新解释成密钥轮换。数据库连接、历史四池种子预算与逐池超时已由第 2.3–2.4 节登记；阶段 2 的 development/test fixture 必须使用这些点分键机械生成的环境变量，production 只读签名 active configuration generation。不存在 `db.dsn`、`db.ro.work_mem`、`db.ro.temp_file_limit` 或把池名与字段名拼在同一段的兼容别名。

### 2.23 平台内核完整登记（阶段 3）

| 键 | 类型 | 默认值 | 生效方式 |
|---|---|---|---|
| platform.sequence.default_width | u8 | 6 | 启动 |
| platform.sequence.max_width | u8 | 12 | 启动 |
| platform.sequence.lock_timeout_ms | u32 | 3000 | 启动 |
| platform.outbox.batch_size | u16 | 100 | 启动 |
| platform.outbox.poll_interval_ms | u32 | 200 | 启动 |
| platform.outbox.idle_backoff_ms | u32 | 2000 | 启动 |
| platform.outbox.max_attempts | u8 | 8 | 启动 |
| platform.outbox.retry_backoff_seconds | u32 数组 | `[1,5,30,120,600,1800,3600,7200]` | 启动 |
| platform.outbox.dispatch_concurrency | u8 | 4 | 启动 |
| platform.outbox.lock_lease_seconds | u32 | 60 | 启动 |
| platform.outbox.done_retention_days | u16 | 30 | 启动 |
| platform.outbox.inbox_retention_days | u16 | 60 | 启动 |
| platform.audit.anchor_interval_seconds | u32 | 300 | 启动 |
| platform.audit.anchor_event_threshold | u32 | 1000 | 启动 |
| platform.audit.anchor_scan_interval_seconds | u32 | 30 | 启动 |
| platform.audit.anchor_age_alert_seconds | u32 | 900 | 启动 |
| platform.audit.segment_lock_timeout_ms | u32 | 3000 | 启动 |
| platform.audit.signature_algorithm | enum：ECDSA_P256_SHA256 | ECDSA_P256_SHA256 | 启动 |
| platform.audit.signing_key_ref | secret 引用 | `secret://audit/segment_signing#1` | 取用 |
| platform.audit.evidence_dir | path | `{data_root}\audit\evidence` | 启动 |
| platform.audit.verify_max_days | u16 | 92 | 启动 |
| platform.audit.query_max_days | u16 | 366 | 启动 |
| platform.file.root_dir | path | `{data_root}\files\published` | 启动 |
| platform.file.staging_dir | path | `{data_root}\files\staging` | 启动 |
| platform.file.max_object_bytes | u64 | 5368709120 | 启动 |
| platform.file.part_bytes | u32 | 8388608 | 启动 |
| platform.file.session_ttl_hours | u16 | 24 | 启动 |
| platform.file.max_concurrent_uploads_per_user | u8 | 3 | 启动 |
| platform.file.max_concurrent_uploads_global | u8 | 6 | 启动 |
| platform.file.upload_bandwidth_bytes_per_sec | u64 | 52428800 | 启动 |
| platform.file.download_bandwidth_bytes_per_sec | u64 | 52428800 | 启动 |
| platform.file.free_space_min_bytes | u64 | 107374182400 | 启动 |
| platform.file.scan.timeout_seconds | u32 | 120 | 启动 |
| platform.file.scan.max_archive_ratio | u32 | 200 | 启动 |
| platform.file.scan.max_archive_depth | u8 | 4 | 启动 |
| platform.file.quarantine_retention_days | u16 | 90 | 启动 |
| platform.notify.retention_days | u16 | 180 | 启动 |
| platform.notify.unread_cap_per_user | u32 | 2000 | 启动 |
| platform.notify.sync_fanout_max | u16 | 200 | 启动 |
| platform.notify.push_enabled | bool | false | 启动 |
| platform.notify.push_timeout_ms | u32 | 5000 | 启动 |
| platform.notify.push_max_attempts | u8 | 3 | 启动 |
| platform.notify.push_body_includes_business_fields | bool | false | 启动 |
| platform.notify.push_deactivate_after_failures | u8 | 10 | 启动 |
| platform.flow.state_persistence | enum：same_transaction、outbox_eventual | same_transaction | 启动 |
| platform.flow.scheduler_interval_ms | u32 | 200 | 启动 |
| platform.flow.executor_concurrency | u8 | 4 | 启动 |
| platform.flow.batch_size | u16 | 20 | 启动 |
| platform.flow.timer_scan_batch | u16 | 50 | 启动 |
| platform.flow.max_instance_duration_days | u16 | 365 | 启动 |
| platform.flow.max_steps_per_instance | u16 | 500 | 启动 |
| platform.flow.max_parallel_branches | u8 | 16 | 启动 |
| platform.flow.step_max_attempts | u8 | 5 | 启动 |
| platform.flow.step_retry_backoff_seconds | u32 数组 | `[1,5,30,120,600]` | 启动 |
| platform.flow.compensation_max_attempts | u8 | 5 | 启动 |
| platform.flow.instance_retention_days | u16 | 730 | 启动 |
| platform.flow.expression_max_steps | u16 | 1000 | 启动 |
| platform.search.root_dir | path | `{data_root}\indexes\search` | 启动 |
| platform.retry.serialization_max_attempts | u8 | 3 | 启动 |
| platform.retry.serialization_backoff_ms | u32 数组 | `[50,150,450]` | 启动 |
| platform.retry.circuit_failure_threshold | u8 | 5 | 启动 |
| platform.retry.circuit_open_seconds | u32 | 30 | 启动 |
| platform.retry.circuit_half_open_probes | u8 | 1 | 启动 |
| impact.manual_item.sla_days | u16 | 5 | 启动 |

`platform.notify.push_enabled` 只控制是否生成可选移动推送；内部投递固定调用 `\\.\pipe\ep-integ` 的 `push.dispatch.v1`，没有可配置 endpoint。`impact.manual_item.sla_days` 的允许范围固定为 1–30；`clm.contract_termination_disposition` 的人工节点从该值建立 SLA 定时器，流程模板的 `max_instance_duration_days` 固定复用 365 天，不另设第二个键。定时器超期只提醒，不自动决策。平台内核其余键全部与阶段 3 的配置表逐项一一对应；不得以未登记别名或未声明默认值启动。

### 2.24 采购与供应商门户（阶段 7）

| 键 | 类型 | 默认值 | 生效方式 |
|---|---|---|---|
| portal.session.max_age_seconds | u32 | 7200 | 启动 |
| portal.session.idle_timeout_seconds | u32 | 900 | 启动 |
| portal.session.validation_cache_ttl_seconds | u32 | 30 | 启动 |
| portal.rate_limit.requests_per_minute | u32 | 120 | 启动 |
| portal.rate_limit.burst | u32 | 40 | 启动 |
| portal.core_api.timeout_ms | u32 | 8000 | 启动 |
| portal.upload.max_attachment_bytes | u64 | 52428800 | 启动 |
| portal.self_registration.enabled | bool | false | 启动 |
| portal.watermark.enabled | bool | true | 启动 |
| procure.receipt.max_lines | u16 | 200 | 启动 |
| procure.return.max_lines | u16 | 200 | 启动 |
| procure.requisition.stock_shortage_scan_enabled | bool | false | 启动 |
| procure.requisition.stock_shortage_scan_interval_minutes | u32 | 60 | 启动 |

旧键 `portal.upstream_base_url` 已随固定 `ep-core` 管道删除；旧 `portal.rate_limit_rps` 由本节两个精确限流键取代。三者均不得兼容读取。自助注册首版默认关闭；未来启用必须通过正式签名配置变更与安全审批。

### 2.25 库存（阶段 8）

| 键 | 类型 | 默认值 | 生效方式 |
|---|---|---|---|
| inventory.posting.max_lines | u32 | 200 | 启动 |
| inventory.posting.max_serials_per_line | u32 | 1000 | 启动 |
| inventory.recon.batch_size | u32 | 2000 | 启动 |

### 2.26 总账与关账（阶段 9）

| 键 | 类型 | 默认值 | 生效方式 |
|---|---|---|---|
| ledger.posting.max_lines_per_voucher | u16 | 500 | 启动 |
| ledger.close.inflight_wait_poll_interval_ms | u32 | 500 | 取用 |
| ledger.close.inflight_wait_warn_seconds | u32 | 300 | 取用 |
| ledger.close.batch_size | u32 | 20000 | 取用 |
| ledger.close.batch_timeout_seconds | u32 | 120 | 取用 |
| ledger.close.batch_work_mem | size string | `256MB` | 取用 |
| ledger.close.batch_temp_file_limit | size string | `4GB` | 取用 |
| ledger.close.recovery_mode_batch_size | u32，1000–20000 | 5000 | 取用 |
| ledger.close.recovery_mode_batch_timeout_seconds | u32，60–900 | 300 | 取用 |
| ledger.close.recovery_mode_batch_work_mem | size string，64MB–512MB | `128MB` | 取用 |
| ledger.close.recovery_mode_batch_temp_file_limit | size string，512MB–8GB | `2GB` | 取用 |

“取用”在这里指新关账 run 建立时一次性读取并快照；运行中的 run 不随配置变化。任一资源值越界、单位非法或数据库不接受 `SET LOCAL` 时以退出码 78 拒绝启动恢复任务，绝不回落为无限。

### 2.27 财务与发票（阶段 10）

| 键 | 类型 | 默认值 | 生效方式 |
|---|---|---|---|
| invoice.ratio.tolerance | decimal(9,6) | 0.000001 | 启动 |
| invoice.issue.require_image_attachment | bool | false | 启动 |
| invoice.import.max_rows | u32 | 2000 | 启动 |
| invoice.import.on_row_failure | enum：CONTINUE、ABORT | CONTINUE | 启动 |
| invoice.void.max_days_after_issue | u32 可空 | null（不限） | 启动 |
| finance.settlement.cross_party_allowed | bool | false | 启动 |
| finance.settlement.max_lines | u32 | 200 | 启动 |
| finance.receipt.requires_approval | bool | false | 启动 |
| finance.cash_account.requires_approval | bool | true | 启动 |
| finance.bank_account.mask_tail_digits | u8 | 4 | 启动 |
| finance.recon.max_periods_per_query | u8 | 12 | 启动 |

`invoice.void.max_days_after_issue=null` 表示首版不另设时间窗，但作废状态迁移、权限、重新认证与冲销不变量仍全部生效；填正整数 N 后仅收紧为开具后 N 个服务器自然日内允许登记。0 与负数非法。该键改变前置阈值，不允许新增、删除或关闭状态迁移。

### 2.28 客户端、低代码、插件与发布（阶段 13）

| 键 | 类型 | 默认值 | 生效方式 |
|---|---|---|---|
| client.bootstrap_cache_ttl_seconds | u32 | 300 | 取用 |
| client.mobile_batch_max | u16 | 50 | 启动 |
| client.desktop_batch_max | u16 | 200 | 启动 |
| client.handoff_token_ttl_seconds | u32 | 300 | 启动 |
| client.cache_ttl_days.desktop | u16 | 14 | 取用 |
| client.cache_ttl_days.mobile | u16 | 7 | 取用 |
| client.max_local_attachment_bytes.desktop | u64 | 5368709120 | 取用 |
| client.max_local_attachment_bytes.mobile | u64 | 268435456 | 取用 |
| client.native_plugin_enabled_default | bool | true | 取用 |
| lowcode.max_custom_objects | u16 | 200 | 启动 |
| lowcode.max_fields_per_object | u16 | 100 | 启动 |
| lowcode.max_indexes_per_object | u8 | 5 | 启动 |
| lowcode.ddl_lock_timeout_ms | u32，最大 5000 | 5000 | 启动 |
| lowcode.ddl_statement_timeout_ms | u32，最大 1800000 | 1800000 | 启动 |
| lowcode.ddl_max_statements_per_plan | u16 | 200 | 启动 |
| lowcode.edit_lock_ttl_seconds | u32 | 1800 | 启动 |
| lowcode.rule_max_ast_nodes | u16 | 500 | 启动 |
| lowcode.rule_max_eval_depth | u8 | 32 | 启动 |
| release.rollback_keep_packages | u8 | 10 | 启动 |
| release.rollback_max_age_days | u16 | 180 | 启动 |
| release.package_max_bytes | u64 | 67108864 | 启动 |
| release.package_max_items | u16 | 2000 | 启动 |
| release.signing_key_ref | secret 引用 | `secret://config/release_signing#1` | 取用 |
| release.trusted_signer_subjects | string 数组 | `[]` | 启动 |
| release.pause_during_period_close | bool | true | 启动 |
| plugin.max_instances | u16 | 8 | 启动 |
| plugin.default_fuel | u64 | 200000000 | 启动 |
| plugin.default_memory_bytes | u64 | 67108864 | 启动 |
| plugin.epoch_tick_ms | u32 | 100 | 启动 |
| plugin.call_timeout_ms.transactional | u32 | 2000 | 启动 |
| plugin.call_timeout_ms.worker | u32 | 30000 | 启动 |
| plugin.compile_cache_dir | path | `{data_root}\plugin-work\compile-cache` | 启动 |
| plugin.auto_disable_failure_threshold | u8 | 3 | 启动 |
| plugin.trusted_signer_subjects | string 数组 | `[]` | 启动 |
| brand.active_profile_code | string | default | 取用 |

客户端“取用”键在下一次引导包生成时读取；已签发客户端引导包不被原地改写。签名密钥引用在每次签名时解引用，其余插件与发布边界在进程启动时冻结。

`release.trusted_signer_subjects` 不是许可证或模块发行签名人的授权根。唯一授权事实是已验签 `DeploymentManifestV1.license_trusted_signer_subjects`：恰 1..64 个 `spki-sha256:<64-lowerhex>`，按 UTF-8 bytes 严格递增且唯一；它是可识别 signer identity roster，后续签名清单必须继续包含全部 `RELEASED` special inner/source-outer 历史引用 token，撤销只由 CRL 分类而不能靠删除 token 冒充。本键默认 `[]` 表示不覆盖并直接使用该 signed roster；若配置为非空，则自身也必须满足相同 canonical 形状并与 signed roster 逐项、顺序完全相等，否则 core/worker readiness 与运维发布 gate 失败。该键只能用于部署漂移检测，不能增加、删除或替换签名人。

### 2.29 本地 AI 与 MCP（F-55）

| 键 | 类型 | 默认值 | 生效方式 |
|---|---|---|---|
| ai.enabled | bool | false | 启动 |
| ai.plan_ttl_seconds | u32，固定 | 300 | 启动 |
| ai.max_concurrent_requests | u16，固定 | 15 | 启动 |
| ai.queue_capacity | u16，固定 | 30 | 启动 |
| ai.compose_timeout_ms | u32，固定 | 120000 | 启动 |
| ai.result_row_limit | u32，固定 | 1000 | 启动 |
| ai.result_bytes_limit | u32，固定 | 8388608 | 启动 |
| mcp.inbound_enabled | bool | false | 启动 |
| mcp.outbound_enabled | bool | false | 启动 |
| mcp.grant_ttl_seconds | u32，默认即最大 | 600 | 启动 |
| mcp.max_calls_per_grant | u16，默认即最大 | 100 | 启动 |
| mcp.request_bytes_limit | u32，固定 | 1048576 | 启动 |
| mcp.response_bytes_limit | u32，固定 | 8388608 | 启动 |
| mcp.call_timeout_ms | u32，固定 | 30000 | 启动 |
| mcp.remote_connect_timeout_ms | u32，固定 | 5000 | 启动 |
| mcp.local_start_timeout_ms | u32，固定 | 10000 | 启动 |

development/test 环境变量按总则映射，例如 `ai.enabled` 为 `EP__AI__ENABLED`；production 只从签名 active configuration generation 取得同名值。三个 `enabled` 键是布尔开关；它们只表达管理员意图，不能生成购买态或覆盖签名许可。`mcp.grant_ttl_seconds` 与 `mcp.max_calls_per_grant` 可在 `1..=600`、`1..=100` 内向下收紧，省略时分别取 600、100；其余标注“固定”的键出现不同值即拒绝启动，不做静默钳制。`ai.enabled=true` 还必须由 F-56 同一 current signed grant 在目标法人 scope 内给出 `F55LocalAi`，且状态为 `Active|ExpiringSoon|GracePeriod`，并同时具备唯一 ACTIVE 且已认证模型包、成功 activation ACK、共同 `RG-LICENSE-MODULE-LIFECYCLE-GREEN`、`RG-AI-CONTAINMENT-GREEN` 与 `RG-AI-RESOURCE-CERTIFIED`。MCP 任一 enabled=true 必须由同一 F-56 current signed grant 在目标法人 scope 内给出共同的 `F55Mcp`（不分方向），状态同为前三态，并同时具备对应方向 compatible ACTIVE manifest、共同许可 gate、`RG-MCP-CONFORMANCE-GREEN` 与 `RG-MCP-CONTAINMENT-GREEN`。任一条件消失时业务路由立即撤下/表现为不存在，health/control pipe 可存在但不得读取业务 payload、出网、启子进程或推理；历史 `purchased` 不能替代 currently licensed。模型代码/版本、远端 origin、本地 entrypoint、credential ref、权限/字段范围、资源限额和 AI 内存硬上限不设配置键，只来自已批准签名记录或 F-55 算定式。只有 F-55 MCP connector 的持久凭据存 Windows Credential Manager；该句不覆盖平台通用 `secret://` KMS 机密。配置导出一律只含 ref。


## 3. 按进程固定的监听地址

`http.bind_addr` 与 `metrics.bind_addr` 的默认值随进程不同；存在 HTTP 监听的进程只监听回环地址。`integration-gateway` 不开 HTTP 监听，健康与指标同样经受控命名管道读取：

| 进程 | 端口 |
|---|---|
| core-server | 8080 |
| job-worker | 8081 |
| integration-gateway | 无 HTTP 监听；`health.get.v1`、`metrics.snapshot.v1` 经 `\\.\pipe\ep-integ` |
| portal-gateway | 8090 |
| ops-agent | 指标 9101，健康与就绪 9102 |
| plugin-host | 无 HTTP 监听，仅 IPC |
| ai-inferer | 无 HTTP 监听；`health.get.v1`、`metrics.snapshot.v1`、`ai.query_plan.compose.v1`、`ai.model.activate.v1`、`ai.model.deactivate.v1` 经 `\\.\pipe\ep-ai` |
| archive-writer、backup-writer | 无监听，仅 IPC 客户端与 spool |

## 4. 不进配置文件的两类

一是运行期可变的业务参数；二是机密。平台通用机密只写 `secret://`，零 KMS 的 integration-gateway/plugin-host 持有的集成凭据只写 `wincred://`，建立 KMS 之前必须取得的 HSM PIN 只写 `bootstrap://`；三类引用均不得被替换成明文。

## 5. 机密处理终态与历史临时状态

### 5.1 F-57 现行秘密边界

生产只接受 `secrets.provider=kms`。所有客户凭据正文以 envelope ciphertext 保存在 `{data_root}\secrets` HDD vault；用途/法人 data key 由客户批准的 TPM/HSM/KMS non-exportable wrapping handle 包装。服务只能取得调用级、作用域化、可撤销的短期内存 handle。客户 secret、secret hash、通用 master key 正文和连接器长期凭据不得落入 SSD、WinCred、服务 profile、普通文件、argv、环境变量、日志或普通 IPC。初始化、轮换、撤销和恢复均为双人高风险命令并产生不可变证据；provider 关闭或 handle 过期时失败关闭。

### 5.2 `HISTORICAL_NON_NORMATIVE_APPENDIX`：F-55 WinCred 维护协议

下列 WinCred、DPAPI 文件树、八命令 `ep-secretctl` 和 `C:\ProgramData` intent 设计只作旧方案追溯，已被 §5.1 取代，不得在 F-57 生产实现中启用。其有价值的双人批准、短期维护窗、内存清零、故障恢复和非秘密 receipt 思路应迁入 HDD vault/secret broker，而不是恢复 WinCred 持久凭据。

生产只接受 `secrets.provider=kms`。`FileSecretProvider` 是 Stage 1 历史切片；`file` 在生产配置中未知并以 78 拒绝启动，常驻产品二进制不得链接 legacy-file feature。旧 `secret://`/DPAPI/WinCred 细节仅属于本历史附录。

Windows Credential Manager 只承载由零 KMS 进程消费的 MCP connector 与电子签章持久凭据。Microsoft 的 `CredWriteW` 把新凭据关联到 **current token 的 logon session**，因此普通管理员进程不能替 `NT SERVICE\ep-integ` 或 `NT SERVICE\ep-plugin` 写入正确的 credential set；实现必须由目标服务在自己的 current token 下调用 `CredWriteW(CRED_TYPE_GENERIC,CRED_PERSIST_LOCAL_MACHINE)`。`CredentialBlobSize` 的产品上限固定为 1..2560 bytes，直接取 Windows SDK 的 `CRED_MAX_CREDENTIAL_BLOB_SIZE = 5*512`；实现者可核对 Microsoft Learn 的 [CREDENTIALW](https://learn.microsoft.com/en-us/windows/win32/api/wincred/ns-wincred-credentialw) 与 [CredWriteW](https://learn.microsoft.com/en-us/windows/win32/api/wincred/nf-wincred-credwritew)。2561 bytes 必须在调用 Win32 API 前拒绝。

`WindowsCredentialRef` 的唯一 canonical grammar 为 `wincred://<segment>/<segment>[/...]`：恰有 2..8 个 segment，每段匹配 `[a-z0-9][a-z0-9._-]{0,63}`，完整引用不超过 512 个 UTF-8 bytes。解析器拒绝空段、`.`、`..`、大写、反斜线、冒号、百分号编码、query、fragment、额外 scheme、控制字符和尾随 `/`，不得先宽松解析再改写。传给 Win32 `CREDENTIALW.TargetName` 的值逐 UTF-16 code unit 等于该完整 canonical 引用（包括 `wincred://`），不得只取 path、增加前缀或使用别名；配置、F-55 manifest、grant、probe、receipt 和 `CredReadW|CredWriteW|CredDeleteW` 必须复用同一个强类型 parser/formatter。

WinCred 初始化、轮换与删除的唯一入口是随产品签名、进入 SBOM 且属于八命令闭集的 `ep-secretctl wincred`。它不直接调用 `CredWriteW`，也不经 HTTP、ServerAdmin、数据库通用写接口、argv、环境变量或文件传递 secret。SCM 必须正常启动目标 Windows 服务并加载该服务虚拟账户的 profile；不得由管理员进程模拟服务 token，也不得另加手工 `LoadUserProfile` 旁路。目标服务收到 SCM 自定义 control code `200` 后进入一次性维护状态并创建专用本地管道：`ep-integ` 为 `\\.\pipe\ep-integ-secretctl`，`ep-plugin` 为 `\\.\pipe\ep-plugin-secretctl`。管道固定 `PIPE_REJECT_REMOTE_CLIENTS`、首实例、无继承，DACL 只授目标服务 SID、SYSTEM 与 BUILTIN\Administrators；服务在读取 payload 前冒充客户端，要求本机交互式、完整提升的 Administrators token，拒绝 network/service/batch/anonymous token，并以 `GetNamedPipeClientProcessId` 打开持有客户端进程句柄，核对映像路径为已安装 `ep-secretctl.exe`、Authenticode 有效且 PE digest 命中本发布清单。PID 单独不构成身份。

维护状态机固定为 `CLOSED → QUIESCING → OPEN → APPLYING → PROBING → COMMITTED → CLOSED`；失败走 `APPLYING|PROBING → ROLLING_BACK → CLOSED_FAILED → CLOSED`。QUIESCING 后不接新 e-sign/MCP egress 或新 stdio child，已有调用最多按各自 30 秒绝对上限排空；OPEN 最长 60 秒、只接受一个连接和一个 grant nonce。只有尚未进入 APPLYING 的超时、断连或 SCM stop 才能在销毁管道、清零缓冲后直接回 CLOSED；APPLYING 开始后的断连、超时或正常 stop 必须先走 ROLLING_BACK。`CLOSED_FAILED` 时管道已经销毁、内存已经清零、目标能力仍为 DISABLED；只有失败 receipt 与 Event Log 均耐久写成后才允许唯一出边 `CLOSED_FAILED→CLOSED`，不自动重试、不自动启用。除 OPEN 状态外管道名不得存在；实现和测试拒绝全部未列非法边。

为覆盖进程崩溃、强杀、断电或 SCM 无法等到 rollback 的边界，服务必须在第一次 `CredWriteW|CredDeleteW` 前，以 write-through 原子替换并 flush 固定非秘密 intent：`C:\ProgramData\EnterprisePlatform\state\<recipient-service>\wincred-maintenance-intent.jcs`。该文件最大 16384 bytes、owner SYSTEM、关闭继承，DACL 只含目标服务 SID、SYSTEM、Administrators；strict JCS 字段恰为 `schema_version=1,request_id,recipient_service,action,target_ref,purpose,grant_digest,old_present,phase,started_at,updated_at`，phase 只取 `APPLY_INTENT|COMMITTED|ROLLED_BACK|RECOVERY_REQUIRED`，不得含 secret、secret hash、旧值、新值或 console/pipe bytes。正常 COMMITTED/ROLLED_BACK 只在 receipt 与 Event Log 耐久后写终态 phase。SCM 下次启动时若看到 `APPLY_INTENT`，必须在读取业务 credential、开放 egress 或创建维护管道前重建 `CLOSED_FAILED`，保持能力 DISABLED，耐久写 `RECOVERY_REQUIRED` failure receipt/Event Log 与 phase 后才走唯一边到 CLOSED；此后只接受一份新的双人 grant 对同一 target/purpose 作纠正维护，成功 probe 并写终态 phase 后才解除 recovery 标志，enable 仍须另走既有 gate。不得根据残留 target 猜测原操作成功、自动使用它或把异常重启当作普通 CLOSED。

管道 operation 闭集只有 `wincred.provision.apply.v1`。第一帧为最多 65536 bytes 的 strict JSON 元数据 `{request_id,grant_jcs,grant_cms_signatures,action,target_ref,purpose,secret_len}`；这是不含 secret 的授权 metadata frame 上限，与随后 CredentialBlob 的 2560-byte 硬上限不同，绝不放宽后者。`action` 只取 `CREATE|ROTATE|DELETE`。共享 `WinCredProvisionPurposeV1` 的全平台闭集只取 `MCP_REMOTE_BEARER|MCP_STDIO_SECRET|ESIGN_API_CREDENTIAL`，且 recipient/purpose 矩阵固定为 `ep-integ→MCP_REMOTE_BEARER|ESIGN_API_CREDENTIAL`、`ep-plugin→MCP_STDIO_SECRET`；跨行组合在读 secret frame 前拒绝。F-55 MCP 只使用前两项，Stage 6 电子签章只使用第三项，不得各自定义同名窄 enum 覆盖共享协议。grant 是 `WinCredProvisionGrantV1`，逐字绑定 schema_version=1、deployment_id、recipient_service、action、target_ref、purpose、request_id、一次性 nonce、reason、not_before、expires_at 和两个不同批准人；有效期不超过 5 分钟，由签名部署清单内的客户安全管理员证书闭集作两份 detached CMS 签名。服务核对 target 正被本进程的签名 config/ACTIVE manifest 引用、能力已禁用且无在途调用，grant 重放、两签同人、过期、范围不符或未知字段均失败关闭。

`CREATE|ROTATE` 后紧跟恰好一帧 `u32be(secret_len) || secret`，secret_len 必须 1..2560 且与元数据相等；`DELETE` 固定 secret_len=0 且没有 secret 帧。MCP REMOTE secret 另须为 ASCII `0x21..0x7e` 且无空白、DEL、CR/LF 或控制字节；MCP stdio 为 1..2560 UTF-8 bytes；电子签章按 provider 契约在 1..2560 内验证。服务以自身 current token 执行 CredRead/CredWrite/CredDelete。CREATE/ROTATE 写后必须执行同 purpose probe；CREATE probe 失败即删除新条目，ROTATE 在 zeroizing 内存保留旧 blob 并回写，回写失败则能力保持禁用并产生高严重度审计。成功后服务返回不超过 16384 bytes 的非秘密 receipt，只含 request/grant digest、target ref、purpose、action、old_present、probe/result 稳定码和时间，不含 secret 或其摘要。

目标服务与 `ep-secretctl` 均把同一 request_id 的开始、拒绝、提交或回滚结论写 Windows Event Log；日志含两个批准人 subject、grant digest、target ref 与稳定码，不含 secret/secret hash。所有 console、pipe、CredRead、旧值、写入值与 probe buffer 在每条路径显式 zeroize；`ep-secretctl` 只通过关闭 echo 的 `ReadConsoleW` 读取并二次确认 secret，不接受重定向 stdin。运维证据只保存非秘密 receipt。普通管理员直接运行 CredWrite、目标服务常驻开放维护管道、以环境变量/文件/ServerAdmin 代送 secret，以及把 2561 bytes 交给 Win32 API，均为发布负例。Windows Server 2022 实机还必须证明：SCM 加载服务账户 profile 后由目标服务写入并读取，服务正常重启后仍能以同一 target 读取同一值；另在 APPLYING 的 Win32 调用前、调用后与 probe 后三个 fault point 强杀服务，均须由 intent 恢复为 fail-closed 且不能直接出边到普通 CLOSED。管理员 vault 写入、管理员模拟 token 或手工 `LoadUserProfile` 均不能冒充上述证据。

原第二处临时状态（请求头 `X-Legal-Entity-Id`、`X-Device-Id`、`X-Client`、`Authorization` 四个在阶段 1 只校验存在性与格式、不做任何真实校验）已由阶段 4 关闭（任务 #23）：四头纯格式校验保留为第一道（runtime `http/headers.rs`），真实校验经端口在 core-server 装配注入——认证层以 `Authorization` 令牌 SHA-256 摘要查 sessions，法人校验层对照 `user_legal_entity_grants` 授权集合校验 `X-Legal-Entity-Id`；关闭说明见 ADR-0011 追加段。系统端点与 PRE_AUTH 白名单（登录、MFA 完成前段、法人列表、门户登录）豁免这些头，豁免清单在代码中是一张固定表，新增豁免路径须改该表并触发安全审查。

配套 CI 断言：`SecretBytes` 与 `SecretString` 均未实现 `Clone`、`Debug`、`Display`、`Serialize`；配置结构体中的机密引用只能使用 `SecretRef`、`WindowsCredentialRef` 或 `BootstrapRef` 三种强类型，不能退化为普通 `String`，实际 secret 不得出现在配置结构体。

## 6. 已删除、不得再引入的键

下列键曾出现在早期草案中，已随裁定删除，任何阶段不得再引入，此处只作追溯，不属第 2 节登记表：

- `admission.active_window_seconds`、`admission.max_concurrent_users`、`admission.queue_max_len`、`admission.queue_wait_timeout_seconds`、`db.migration.expected_versions_path`：本批（F-50…F-57）自本节及第 2 节一并删除。**其中 `admission.max_concurrent_users` 与 `admission.queue_max_len` 代码中仍在使用**，故 `xtask configdoc` 现报「代码里有、文档中没有登记」；**该两键须在实现批次中随准入控制口径一并裁定去留，未裁定前不得视为已作废**（F-59 登记）。**F-62 补：其余三键（`admission.active_window_seconds`、`admission.queue_wait_timeout_seconds`、`db.migration.expected_versions_path`）同样仍声明在代码配置结构体中**（`sections.rs:412`／`:411`／`:187`），`configdoc` 对五键各报一条不符；「已随裁定删除」指文档登记面，代码侧的移除随同批实现裁定。
- `selfcheck.pending_as_failure`：随阶段 1 计划第 13 节假设二删除。Pending 项一律不阻止启动，这是固定行为不是开关；置真会让建设期的每一个进程都起不来，它没有真实的取用者。
- `selfcheck.quota_manifest_path`：随第 13 节新增决定十四删除。资源限额改为部署侧的静态 drop-in 加一次性部署校验，不再有生成的配额清单文件，任何进程的启动自检中也不出现资源限额项。
- `platform.notify.push_endpoint`、`portal.core_api.base_url`、`portal.upstream_base_url`：产品进程业务 IPC 已统一为固定 DACL 命名管道，管道名与 operation 是协议常量，不设 endpoint 配置，不得兼容读取或转换成 localhost HTTP；推送固定经 `\\.\pipe\ep-integ` 的 `push.dispatch.v1`。
- `portal.rate_limit_rps`：由 `portal.rate_limit.requests_per_minute` 与 `portal.rate_limit.burst` 取代，不做双读兼容。
- `db.pool.integ_max`：integration-gateway 已归零数据库权限、连接与配置，不兼容解析 `integ` 池或该旧键。历史四池 `37`、临时 `10`、硬峰值 `52` 与安全余量 `5` 只作 ADR-0019 实测再基线的种子，不是现行固定产品预算。
- `archive.max_slot_wal_keep_gb`：由 `archive.wal_spool_max_gb` 取代；同值由部署自检与 PostgreSQL 参数读回保证。
- `ops.crosscheck_period_seconds`、`ops.crosscheck_statement_timeout_ms`、`ops.crosscheck.timeout_seconds`：专用复制交叉核对子系统已删除，交叉核对复用 `ops.wal_retention_sample_period_seconds`，不得恢复第二套周期或超时键。

## 7. 当前状态

F-57 总体仍为 `READY_NOT_AUTHORIZED`，本文件生成式登记仍为 `REGISTRY_PENDING_REBASELINE` / `NOT_IMPLEMENTED`。旧 **328 个配置族 / 337 个具体键** 只记录 F-56 快照，不是 F-57 终态计数；只有用户另行明确授权后，现行 G0 bootstrap 子计划的 Task 1 才可保留仍有效键、删除/拒绝历史旁路、登记 deployment/storage/capacity/generation/package/provider/sync/Windows 证据键，并让 `cargo xtask configdoc` 同时验证文档↔代码、稳定 volume ID、`{data_root}` 路由和生产禁止值。`IAAS_WINDOWS_SERVER_HDD_STRICT` 仅是未来保留标识：当前没有 recipe、schema、handler、正向测试或生产 terminal，选择它固定 `PROFILE_NOT_IMPLEMENTED` / `STORAGE_MEDIA_UNVERIFIED`；只有用户另行授权的新 graph/profile version 才可改变该边界。在 Windows agent 真实返回 0 前状态为 `UNVERIFIED`；当前不得执行开发、后续业务任务或宣称配置闭合。
