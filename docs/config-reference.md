# 配置参考

本文件是全部配置键的唯一登记处。代码侧的配置结构体与本文件由 CI 项 `xtask configdoc` 逐键比对，缺一即失败。新增配置键的顺序是先登记后使用。

## 1. 加载顺序与总则

五层覆盖，后者覆盖前者：内置默认、主配置文件、片段目录（按文件名字典序）、环境变量、命令行参数。

- 配置结构体开启 `deny_unknown_fields`。未知键一律拒绝启动，不忽略、不警告了事。写错一个键名却照常启动，是最难排查的一类故障。
- 类型错误的错误消息含键路径，指出是哪一个键。
- 环境变量映射为双下划线分段并全大写，前缀 `EP__`。例：`db.pool.rw_max` 对应 `EP__DB__POOL__RW_MAX`。
- 「生效方式」一列：启动表示改动后需重启进程；SIGHUP 表示可热加载；取用表示在下次取用该值时生效。

## 2. 登记表

下表是阶段 1 引入的全部配置键。

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
| db.pool.acquire_timeout_ms | u32 | 3000 | 启动 |
| db.pool.max_lifetime_s | u32 | 1800 | 启动 |
| db.pool.idle_timeout_s | u32 | 300 | 启动 |
| db.timeout.<池>.statement_ms | u32 | 逐池取值见下 | 启动 |
| db.timeout.<池>.lock_ms | u32 | 3000 | 启动 |
| db.timeout.<池>.idle_in_tx_ms | u32 | 15000 | 启动 |
| db.ro.work_mem_kb | u32 | 65536 | 启动 |
| db.ro.temp_file_limit_kb | u32 | 2097152 | 启动 |
| db.retry.max_attempts | u8 | 3 | 启动 |
| db.retry.backoff_ms | u32 数组 | [50,150,450] | 启动 |

`<池>` 取 `rw`、`ro`、`worker`、`integ`、`ops` 五值。`statement_ms` 的逐池默认值取阶段 1 计划第 7.2 节的池表：rw 10000、ro 60000、worker 300000、integ 10000、ops 5000。`lock_ms` 与 `idle_in_tx_ms` 五池同值。

`db.retry.*` 只对尚未产生任何外部可见副作用的事务生效，触发条件为 SQLSTATE 40001 与 40P01。

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

## 5. 机密处理与两处临时状态

`secrets.provider` 取 `file` 时使用 `FileSecretProvider`，从 `secrets.dir` 读取权限 0600 的文件，不做信封加密。**这是阶段 1 的临时实现**，由密钥阶段替换为内置 KMS 或 HSM 解封，见 ADR-0007。在替换之前，机密在磁盘上是明文，其保护完全依赖文件权限与运行账户隔离，不得读成「已具备密钥管理」。

同期第二处临时状态：请求头 `X-Legal-Entity-Id`、`X-Device-Id`、`X-Client`、`Authorization` 四个在阶段 1 只校验存在性与格式（UUID 格式、枚举取值、Bearer 前缀与 43 位 base64url），**不做任何真实校验**。阶段 1 不定义 `AuthnPort` 与 `LegalEntityScopePort`，也不注入任何空实现，真实校验与其端口由阶段 4 在交付第一条判定时同批引入。此状态不得读成「已具备鉴权」。系统端点豁免这四个头，豁免清单在代码中是一张固定表，新增豁免路径须改该表并触发安全审查。

两条配套的 CI 断言：`SecretString` 未实现 `Debug` 与 `Display`；配置结构体中任何名字含 `password`、`secret`、`key`、`token` 的字段，类型必须是 `SecretString` 或 `SecretRef`。

## 6. 已删除、不得再引入的两个键

下面两个键曾出现在早期草案中，已随裁定删除，任何阶段不得再引入，此处只作追溯，不属第 2 节登记表：

- `selfcheck.pending_as_failure`：随阶段 1 计划第 13 节假设二删除。Pending 项一律不阻止启动，这是固定行为不是开关；置真会让建设期的每一个进程都起不来，它没有真实的取用者。
- `selfcheck.quota_manifest_path`：随第 13 节新增决定十四删除。资源限额改为部署侧的静态 drop-in 加一次性部署校验，不再有生成的配额清单文件，任何进程的启动自检中也不出现资源限额项。

## 7. 当前状态

如实记录：截至本文件写成时，`crates/platform/runtime/` 下只有 `src/lib.rs` 一个骨架文件，配置结构体尚未落地，八个进程的 `main` 也尚未读取任何配置；`xtask configdoc` 当前以退出码 70 明确报「本阶段未交付」，不静默返回 0。因此第 2 节登记表与代码的逐键一致目前无被测对象。登记表的出处是阶段 1 计划第 8 节与退出条件 9，代码侧补齐时以本表为准。
