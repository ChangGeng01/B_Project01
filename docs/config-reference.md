# 配置参考

本文件是全部配置键的唯一登记处。代码侧的配置结构体与本文件由 CI 项 `xtask configdoc` 逐键比对，缺一即失败。新增配置键的顺序是先登记后使用。

## 1. 加载顺序与总则

五层覆盖，后者覆盖前者：内置默认、主配置文件、片段目录（按文件名字典序）、环境变量、命令行参数。

- 配置结构体开启 `deny_unknown_fields`。未知键一律拒绝启动，不忽略、不警告了事。写错一个键名却照常启动，是最难排查的一类故障。
- 类型错误的错误消息含键路径，指出是哪一个键。
- 环境变量映射为双下划线分段并全大写，前缀 `EP__`。例：`db.pool.rw_max` 对应 `EP__DB__POOL__RW_MAX`。
- 「生效方式」一列：启动表示改动后需重启进程；SIGHUP 表示可热加载；取用表示在下次取用该值时生效。

## 2. 登记表

下表是全部已登记的配置键：阶段 1 引入的键为主体，阶段 2 任务 #11 新增的连接预算与迁移台账两小节、任务 #12 与 #14 新增的密钥管理与迁移窗口两小节、阶段 3a 任务 #18 新增的平台内核一节、阶段 4 任务 #22 新增的授权判定与并发准入两小节均同批登记。

### 2.1 HTTP

| 键 | 类型 | 默认值 | 生效方式 |
|---|---|---|---|
| http.bind_addr | string | 按进程固定，见第 3 节 | 启动 |
| http.max_body_bytes | u64 | 1048576 | 启动 |
| http.request_timeout_ms | u32 | 8000 | 启动 |
| http.shutdown_drain_ms | u32 | 30000 | 启动 |
| http.concurrency_limit | u16 | 20 | 启动 |
| http.concurrency_wait_ms | u32 | 10000 | 启动 |

`http.request_timeout_ms` 的 8000 与同步等待上限 8 秒同源，超时返回 `PLATFORM.SYSTEM.SYNC_TIMEOUT`。`http.concurrency_limit` 与 `http.concurrency_wait_ms` 是并发闸门的两个参数，等待超时返回 `PLATFORM.CAPACITY.CONCURRENCY_LIMIT` 并计入 `ep_quota_throttled_total`。

### 2.2 IPC

| 键 | 类型 | 默认值 | 生效方式 |
|---|---|---|---|
| ipc.socket_path | path | `/run/ep/ipc/<proc>.sock` | 启动 |
| ipc.max_frame_bytes | u32 | 1048576 | 启动 |

### 2.3 数据库连接

| 键 | 类型 | 默认值 | 生效方式 |
|---|---|---|---|
| db.host | string | 127.0.0.1 | 启动 |
| db.port | u16 | 5432 | 启动 |
| db.database | string | ep | 启动 |
| db.user | string | ep_app_rw | 启动 |
| db.password_ref | string | secret://db/app_rw#1 | 取用 |

`db.password_ref` 写的是引用而不是口令本身，见第 5 节。

### 2.4 连接池与超时

| 键 | 类型 | 默认值 | 生效方式 |
|---|---|---|---|
| db.pool.rw_max | u16 | 20 | 启动 |
| db.pool.ro_max | u16 | 10 | 启动 |
| db.pool.worker_max | u16 | 5 | 启动 |
| db.pool.integ_max | u16 | 5 | 启动 |
| db.pool.ops_max | u16 | 2 | 启动 |
| db.pool.acquire_timeout_ms | u32 | 8000 | 启动 |
| db.pool.max_lifetime_s | u32 | 1800 | 启动 |
| db.pool.idle_timeout_s | u32 | 300 | 启动 |
| db.timeout.<池>.statement_ms | u32 | 逐池取值见下 | 启动 |
| db.timeout.<池>.lock_ms | u32 | 3000 | 启动 |
| db.timeout.<池>.idle_in_tx_ms | u32 | 15000 | 启动 |
| db.ro.work_mem_kb | u32 | 65536 | 启动 |
| db.ro.temp_file_limit_kb | u32 | 2097152 | 启动 |
| db.retry.max_attempts | u8 | 3 | SIGHUP |
| db.retry.backoff_ms | u32 数组 | [50,150,450] | SIGHUP |
| db.budget.resident_max | u16 | 42 | 启动 |
| db.budget.peak_max | u16 | 52 | 启动 |
| db.migration.expected_versions_path | path | /etc/ep/migration-versions.toml | 取用 |

`<池>` 取 `rw`、`ro`、`worker`、`integ`、`ops` 五值。`statement_ms` 的逐池默认值取阶段 1 计划第 7.2 节的池表：rw 10000、ro 60000、worker 300000、integ 10000、ops 5000。`lock_ms` 与 `idle_in_tx_ms` 五池同值。`db.ro.temp_file_limit_kb` 保留取值登记但不在会话级下发：`temp_file_limit` 为 SUSET 参数，应用角色无权 SET，该限额由引导侧角色默认值承接（db/bootstrap/03_role_defaults.sql）。

`db.retry.*` 只对尚未产生任何外部可见副作用的事务生效，触发条件为 SQLSTATE 40001 与 40P01；两键按 SIGHUP 热生效，热加载时重建重试策略而不是在旧策略上打补丁。

`db.budget.*` 是连接预算的两个上限（裁定 C-04）：启动时五池规模求和校验，超限以退出码 78 拒启。`db.migration.expected_versions_path` 是迁移预期版本台账的读取位置。

### 2.5 日志、指标与追踪

| 键 | 类型 | 默认值 | 生效方式 |
|---|---|---|---|
| log.level | string | info | SIGHUP |
| log.debug_auto_off_minutes | u16 | 30 | SIGHUP |
| metrics.enabled | bool | true | 启动 |
| metrics.bind_addr | string | 按进程固定，见第 3 节 | 启动 |
| trace.sample_ratio | f32 | 0.1 | SIGHUP |
| trace.otlp_enabled | bool | false | 启动 |
| trace.otlp_endpoint | string 可空 | null | 启动 |

`log.debug_auto_off_minutes` 是 debug 级别的自动回落时长，避免有人临时开了 debug 之后忘记关掉。

### 2.6 机密与自检

| 键 | 类型 | 默认值 | 生效方式 |
|---|---|---|---|
| secrets.dir | path | /var/lib/ep/secrets | 取用 |
| secrets.provider | enum：file、kms | file | 启动 |
| selfcheck.clock_skew_max_ms | u32 | 1000 | 启动 |

### 2.7 运行时

| 键 | 类型 | 默认值 | 生效方式 |
|---|---|---|---|
| runtime.worker_threads | u16 | 0，表示按 cgroup CPU 配额推导 | 启动 |
| runtime.blocking_threads | u16 | 32 | 启动 |

### 2.8 出网

| 键 | 类型 | 默认值 | 生效方式 |
|---|---|---|---|
| egress.allowlist | string 数组 | 空 | SIGHUP |
| egress.connect_timeout_ms | u32 | 3000 | 启动 |
| egress.request_timeout_ms | u32 | 15000 | 启动 |
| egress.ca_bundle_path | path | /etc/ep/ca/esign-ca.pem | 取用 |
| egress.breaker.failure_threshold | u16 | 5 | 启动 |
| egress.breaker.open_ms | u32 | 30000 | 启动 |
| egress.breaker.half_open_probes | u8 | 1 | 启动 |

出网默认白名单为空，即默认不允许出网。`egress.ca_bundle_path` 的存在理由见 ADR-0004：基础镜像内没有系统证书库。

### 2.9 spool 与门户

| 键 | 类型 | 默认值 | 生效方式 |
|---|---|---|---|
| spool.dir | path | `/var/lib/ep/<proc>/spool` | 启动 |
| spool.max_bytes | u64 | 268435456 | 启动 |
| portal.upstream_base_url | string | http://127.0.0.1:8080 | 启动 |
| portal.rate_limit_rps | u16 | 20 | SIGHUP |

spool 容量超上限时丢弃最旧记录并记 ERROR，绝不阻塞写出。

### 2.10 密钥管理

| 键 | 类型 | 默认值 | 生效方式 |
|---|---|---|---|
| kms.backend | enum：builtin、hsm | builtin | 启动 |
| kms.builtin.master_key_path | path | /var/lib/ep/kms/master.key | 启动 |
| kms.hsm.pkcs11_module | string | 空 | 启动 |
| kms.hsm.slot | u32 | 0 | 启动 |
| kms.hsm.pin_ref | secret 引用 | secret://kms/hsm_pin#1 | 取用 |

`kms.backend` 取 `builtin` 时主密钥取自 `kms.builtin.master_key_path`：32 字节随机内容、权限必须 0400 且属主为本进程账户，否则拒启动（见 02 计划第 12 节假设一）；取 `hsm` 时不回落内置实现，`kms.hsm.*` 三键指向不存在的硬件后端即以配置错误退出码失败。`kms.hsm.pin_ref` 只写 `secret://` 引用不写 PIN 本身，见第 5 节。

### 2.11 迁移窗口

| 键 | 类型 | 默认值 | 生效方式 |
|---|---|---|---|
| migration.window_ttl_max_min | u32 | 240 | SIGHUP |

`migration.window_ttl_max_min` 是开窗端点可接受的窗口存续时长上限，默认 240 对齐 02 计划第 12 节假设四（基线迁移执行上限 30 分钟加一倍余量，取规格第 12.1 章应急账号 8 小时的一半以示更严）；该键热生效，下次开窗取用新值。

### 2.12 平台内核

| 键 | 类型 | 默认值 | 生效方式 |
|---|---|---|---|
| platform.idempotency.retention_days | u32 | 7 | 启动 |

`platform.idempotency.retention_days` 是幂等键定稿行的保留天数（03 计划表 12）：`platform_msg.idempotency_keys` 的行以 `expires_at = created_at + 保留天数` 落库，过期行由保留期清理扫描物理删除；core-server 与 job-worker 双进程生效，环境变量为 `EP__PLATFORM__IDEMPOTENCY__RETENTION_DAYS`。

### 2.13 授权判定

| 键 | 类型 | 默认值 | 生效方式 |
|---|---|---|---|
| authz.snapshot.poll_interval_ms | u32 | 2000 | 取用 |
| authz.decision.explain_enabled | bool | false | SIGHUP |
| authz.scope.max_department_depth | u8 | 8 | 取用 |
| authz.scope.in_list_threshold | u16 | 200 | 取用 |
| authz.export.sensitive_row_threshold | u32 | 1000 | 取用 |

`authz.snapshot.poll_interval_ms` 是授权快照轮询重载的间隔，重载体查 `authz_config_versions` 的 EFFECTIVE 版本号，版本变化即整体替换按法人分片的快照；checksum 不符时开降级窗口而不是继续用旧快照。`authz.scope.max_department_depth` 是记录级范围中部门闭包展开的深度上限，超限截断并计 `ep_authz_scope_truncated_total`；`authz.scope.in_list_threshold` 是部门集合超过该阈值后 IN 列表退化为 EXISTS 子查询的开关点。`authz.export.sensitive_row_threshold` 是导出含敏感字段行数的阈值。环境变量前缀为 `EP__AUTHZ__`。

### 2.14 并发准入

| 键 | 类型 | 默认值 | 生效方式 |
|---|---|---|---|
| admission.max_concurrent_users | u16 | 20 | 启动 |
| admission.queue_max_len | u16 | 40 | 启动 |
| admission.queue_wait_timeout_seconds | u8 | 10 | 启动 |
| admission.active_window_seconds | u16 | 60 | 启动 |

并发准入以活跃用户为计量单位，活跃用户定义为 `admission.active_window_seconds` 内有过请求的不同用户。并发上限 `admission.max_concurrent_users` 满后新请求入队，队列超过 `admission.queue_max_len` 或等待超过 `admission.queue_wait_timeout_seconds` 秒即拒绝，返回 `PLATFORM.CAPACITY.CONCURRENCY_LIMIT`（该码由阶段 1 登记，本处不重复登记）。环境变量前缀为 `EP__ADMISSION__`。

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
| auth.x509.trust_anchor_ref | String | secret://pki/client_ca#1 | 启动 |
| auth.breakglass.max_session_seconds | u32 | 28800 | 启动 |
| auth.breakglass.idle_rotation_days | u16 | 365 | 启动 |

身份域二十二键取 04 计划 §7 配置表的 EP__AUTH__* 全键（阶段 4 任务 #21 登记），一律启动时生效：把安全参数做成热生效开关会在运行期制造不经配置发布通道的旁路。口令与锁定诸键的默认值是 U-B-14 临时取值（12/3/90/5 与 5/900/1800）；`auth.session.ttl_seconds` 与 `auth.session.idle_timeout_seconds` 只影响新会话；`auth.webauthn.rp_id` 与 `auth.webauthn.origins` 必填无默认，缺失即启动自检失败；`auth.breakglass.max_session_seconds` 只能调小不得调大（规格第 12.1 章 8 小时上限）。敏感取值不进本段：X.509 信任锚与 TOTP 种子按基线第 7.2 节以 secret:// 引用表达。环境变量前缀为 `EP__AUTH__`。


## 3. 按进程固定的监听地址

`http.bind_addr` 与 `metrics.bind_addr` 的默认值随进程不同，全部只监听回环地址：

| 进程 | 端口 |
|---|---|
| core-server | 8080 |
| job-worker | 8081 |
| integration-gateway | 8082 |
| portal-gateway | 8090 |
| ops-agent | 指标 9101，健康与就绪 9102 |
| plugin-host | 无 HTTP 监听，仅 IPC |
| archive-writer、backup-writer | 无监听，仅 IPC 客户端与 spool |

## 4. 不进配置文件的两类

一是运行期可变的业务参数，阶段 1 一条都不引入；二是机密，配置里只写 `secret://` 引用。

## 5. 机密处理与一处临时状态

`secrets.provider` 取 `file` 时使用 `FileSecretProvider`，从 `secrets.dir` 读取权限 0600 的文件，不做信封加密。**这是阶段 1 的临时实现**，由密钥阶段替换为内置 KMS 或 HSM 解封，见 ADR-0007。在替换之前，机密在磁盘上是明文，其保护完全依赖文件权限与运行账户隔离，不得读成「已具备密钥管理」。

原第二处临时状态（请求头 `X-Legal-Entity-Id`、`X-Device-Id`、`X-Client`、`Authorization` 四个在阶段 1 只校验存在性与格式、不做任何真实校验）已由阶段 4 关闭（任务 #23）：四头纯格式校验保留为第一道（runtime `http/headers.rs`），真实校验经端口在 core-server 装配注入——认证层以 `Authorization` 令牌 SHA-256 摘要查 sessions，法人校验层对照 `user_legal_entity_grants` 授权集合校验 `X-Legal-Entity-Id`；关闭说明见 ADR-0011 追加段。系统端点与 PRE_AUTH 白名单（登录、MFA 完成前段、法人列表、门户登录）豁免这些头，豁免清单在代码中是一张固定表，新增豁免路径须改该表并触发安全审查。

两条配套的 CI 断言：`SecretString` 未实现 `Debug` 与 `Display`；配置结构体中任何名字含 `password`、`secret`、`key`、`token` 的字段，类型必须是 `SecretString` 或 `SecretRef`。

## 6. 已删除、不得再引入的两个键

下面两个键曾出现在早期草案中，已随裁定删除，任何阶段不得再引入，此处只作追溯，不属第 2 节登记表：

- `selfcheck.pending_as_failure`：随阶段 1 计划第 13 节假设二删除。Pending 项一律不阻止启动，这是固定行为不是开关；置真会让建设期的每一个进程都起不来，它没有真实的取用者。
- `selfcheck.quota_manifest_path`：随第 13 节新增决定十四删除。资源限额改为部署侧的静态 drop-in 加一次性部署校验，不再有生成的配额清单文件，任何进程的启动自检中也不出现资源限额项。

## 7. 当前状态

如实记录：配置结构体已落地在 `crates/platform/runtime/src/config/sections.rs`，`xtask configdoc` 已交付，把本文件第 2 节登记表与代码侧结构体逐键比对，通过时以退出码 0 返回，不静默放过缺键或多余键。登记表的出处仍是阶段 1 计划第 8 节与退出条件 9，新增配置键仍照先登记后使用的顺序。
