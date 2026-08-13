# 指标目录

本文件是全部指标名的唯一登记处。指标名的唯一性由 CI 校验，同一指标只能由一个阶段注册，重复登记即构建失败。

## 1. 命名与暴露

命名形如 `ep_<subsystem>_<metric>_<unit>`。计数器以 `_total` 结尾，时长以 `_seconds` 结尾，字节数以 `_bytes` 结尾。

指标由 ops-agent 在 127.0.0.1:9101 以 Prometheus 文本格式暴露，仅内网可达，可对接客户已有的 Prometheus 与 Grafana。ops-agent 的 `/metrics` 聚合本机各进程的指标端点，抓取失败的目标按 `up=0` 标记，不静默丢弃。

## 2. 标签基数纪律

- 禁止把 `user_id`、`doc_no`、`trace_id` 作为标签。这三项的取值集合随业务量无上限增长，一旦进标签即为时序爆炸。
- `legal_entity_id` 允许，理由是首版只有 2 个法人。
- `route` 一律取模板路径而不是实例路径。

本纪律对下表十八项逐项成立，新增指标须在登记时逐标签核对。

## 3. 登记表

「注册方」是创建该指标并使其在指标端点上可见的阶段，「填充方」是写入非零样本的阶段。两者可以不同：按裁定 C-23，`ep_db_pool_connections` 与 `ep_db_statement_duration_seconds` 由阶段 1 注册、阶段 2 填充，其判据是指标名存在，而不是有非零样本。

| 指标名 | 类型 | 标签 | 注册方 | 填充方 | 含义 |
|---|---|---|---|---|---|
| `ep_build_info` | gauge | `version`、`git_commit` | 阶段 1 | 阶段 1 | 构建标识，取值恒为 1，信息全在标签上 |
| `ep_selfcheck_pending_items` | gauge | `process` | 阶段 1 | 阶段 1 | 该进程启动自检报告中 Pending 项的条数 |
| `ep_db_pool_connections` | gauge | `pool` | 阶段 1 | 阶段 2 | 各具名连接池的当前连接数 |
| `ep_db_statement_duration_seconds` | histogram | `pool`、`statement_kind` | 阶段 1 | 阶段 2 | 单条 SQL 的执行时长分布 |
| `ep_db_tx_retries_total` | counter | `pool`、`sqlstate` | 阶段 2 | 阶段 2 | 因可重试 SQLSTATE 触发的事务重试次数 |
| `ep_http_request_duration_seconds` | histogram | `route`、`method`、`status_class`、`client` | 阶段 1 | 阶段 1 | HTTP 请求时长分布，在中间件栈中填充 |
| `ep_quota_throttled_total` | counter | `route` | 阶段 1 | 阶段 1 | 被并发闸门拒绝的请求数，在闸门中填充 |
| `ep_degradation_windows_open` | gauge | 无 | 阶段 2 | 阶段 2 | 当前未关闭的降级窗口总数，台账每次开闭后刷新 |
| `ep_authz_decision_duration_seconds` | histogram | `legal_entity_id`、`outcome` | 阶段 4 | 阶段 4 | 授权判定四阶段流水线单次判定时长分布 |
| `ep_authz_denied_total` | counter | `legal_entity_id`、`reason` | 阶段 4 | 阶段 4 | 授权拒绝次数，reason 取九拒绝理由的指标标签形态 |
| `ep_authz_scope_truncated_total` | counter | `legal_entity_id` | 阶段 4 | 阶段 4 | 记录级部门闭包深度超限截断次数，伴随 WARN 日志 |
| `ep_reauth_challenges_total` | counter | `legal_entity_id`、`operation_type` | 阶段 4 | 阶段 4 | 高风险操作二次认证挑战签发次数 |
| `ep_session_admission_queue_wait_seconds` | histogram | `outcome` | 阶段 4 | 阶段 4 | 会话准入排队等待时长分布，outcome 取 admitted/rejected |
| `ep_session_admission_rejected_total` | counter | `reason` | 阶段 4 | 阶段 4 | 会话准入拒绝次数，reason 取 queue_full/wait_timeout/closed |
| `ep_authn_login_attempts_total` | counter | `outcome` | 阶段 4 | 阶段 4 | 登录尝试次数，outcome 取登录结果八值的指标标签形态 |
| `ep_authn_active_sessions` | gauge | 无 | 阶段 4 | 阶段 4 | 当前活跃会话数，认证中间件按空闲窗口内核验会话刷新 |
| `ep_breakglass_active_sessions` | gauge | 无 | 阶段 4 | 阶段 4 | 当前应急账号活跃会话数，认证中间件刷新 |
| `ep_high_risk_requests_open` | gauge | `legal_entity_id` | 阶段 4 | 阶段 3b 同批 | 未结束高风险请求数，法人维度刷新；高风险请求四端点属 3b 同批交付，填充面随其就位，本阶段判据是指标名存在（同裁定 C-23 分工） |

### 3.1 取值域与桶

- `pool` 取 `rw`、`ro`、`worker`、`integ`、`ops` 五值，与阶段 1 计划第 7.2 节的五个具名池一一对应。
- `client` 取 `win`、`mac`、`ios`、`android`、`portal`、`ops` 六值，即技术基线第 5.6 节 `X-Client` 头的六个取值，与 `crates/foundation/src/security/context.rs` 的 `ClientKind` 六个变体一一对应。
- `status_class` 取 `2xx`、`3xx`、`4xx`、`5xx`。
- `ep_http_request_duration_seconds` 的桶固定为 0.05、0.1、0.25、0.5、1、2、3、5、10、30，与技术基线第 9.2 节逐值一致。改桶等同于改指标，须走一次登记变更。
- `ep_db_statement_duration_seconds` 的桶由阶段 2 任务 #11 定死：0.0005、0.001、0.0025、0.005、0.01、0.025、0.05、0.1、0.25、0.5、1、2.5、5、10、30，十五个桶 0.5 毫秒起步、逐段约 2.5 倍递增，30 秒封顶对齐各池 statement_timeout 的最大取值。改桶等同于改指标，须走一次登记变更。
- `sqlstate` 取 `40001`、`40P01` 两值，与重试策略的可重试 SQLSTATE 表逐值一致。
- `ep_authz_decision_duration_seconds` 的桶由阶段 4 任务 #22 定死：0.0001、0.00025、0.0005、0.001、0.0025、0.005、0.01、0.025、0.05、0.25、1，十一桶 0.1 毫秒起步，对齐纯逻辑判定 P95 低于 1 毫秒的目标口径；`outcome` 取 `allowed`、`denied` 两值。改桶等同于改指标，须走一次登记变更。
- `ep_session_admission_queue_wait_seconds` 的桶同为阶段 4 定死：0.005、0.01、0.05、0.1、0.25、0.5、1、2.5、5、10，十桶封顶对齐准入等待上限 10 秒；`outcome` 取 `admitted`、`rejected` 两值。
- `ep_authz_denied_total` 的 `reason` 取九拒绝理由的小写下划线形态；`ep_reauth_challenges_total` 的 `operation_type` 取四类受限高危操作的大写枚举取值。
- `ep_authn_login_attempts_total` 的 `outcome` 取登录结果八值的小写下划线形态，取值集与 `crates/platform/identity/src/types.rs` 的 `LoginAttemptOutcome` 逐项一致：success、credential_invalid、account_locked、account_inactive、mfa_required、mfa_invalid、device_unregistered、admission_rejected。挑战过期与末因子禁入归 mfa_invalid，限流拒入归 admission_rejected，不新造取值。

## 4. 阶段 1 的登记范围

阶段 1 登记且只登记上表中注册方为阶段 1 的六项，六项一次性注册在同一处注册表内；`ep_db_tx_retries_total` 与 `ep_degradation_windows_open` 由阶段 2 登记，不在阶段 1 范围。阶段 1 不登记任何超出范围的指标名，也不登记任何与已登记项同义的别名——同义名是重复登记的主要来源，本文件不设废弃名一节，作废指标名的追溯记录留在裁定登记文件，不在本文件出现。

规格第 15.3 章要求的降级与暴露窗口台账既进数据库表也各出一个 gauge，两处不得只有其一；该 gauge 即上表 `ep_degradation_windows_open`，由阶段 2 任务 #14 登记并填充：数据库侧台账是 `platform_ops.degradation_windows`，实现侧在每次开闭窗口后刷新 gauge，取值与台账未关闭窗口数一致。

阶段 2 登记的指标是上表中注册方为阶段 2 的两项：`ep_db_tx_retries_total`（任务 #11 随数据库驱动一并登记）与 `ep_degradation_windows_open`（任务 #14 登记）。注册落点与阶段 1 相同，仍是 `crates/platform/obs/src/metrics/registry.rs` 的同一张定义表，十四项合表、不另立注册表。阶段 2 同时填充上表中填充方为阶段 2 的三项：`ep_db_pool_connections` 与 `ep_db_statement_duration_seconds` 由两个应用各自经 `ObsDbMetrics` 桥接在数据库装配处填充，`ep_db_tx_retries_total` 在事务重试路径填充，`ep_degradation_windows_open` 在降级台账开闭后填充；填充面落在 `crates/adapter/db-pg` 与两应用的 `wiring/metrics.rs`，阶段 1 注册时判据所要求的「指标名存在」之外，本阶段起四项均有非零样本路径。

阶段 4 任务 #22 登记上表中注册方为阶段 4 的六项：授权判定时长与拒绝计数、部门闭包截断计数、复核挑战签发计数、会话准入排队时长与拒绝计数，注册落点仍是同一张定义表；填充面在 `crates/platform/authz` 的判定与准入路径，经装配侧桥接进注册表。

阶段 4 任务 #23 登记身份面四项：登录尝试计数与两项会话 gauge、未结束高风险请求 gauge，注册落点仍是同一张定义表（十四项扩为十八项）。填充面：两项会话 gauge 在认证中间件的活跃会话台账（`apps/core-server/src/platform/middleware.rs` 的 `SessionTracker`）刷新；登录尝试计数在登录端点成功与失败分支及 PRE_AUTH 限流拒入点填充（`platform/identity.rs` 与同文件中间件）；`ep_high_risk_requests_open` 的填充面属 3b 同批的高风险请求端点，本阶段只注册不填充，判据同裁定 C-23。

## 5. 机器判定与当前状态

指标名唯一性校验由 `xtask` 实现，判据是本文件登记表内无重名，且登记表与代码侧注册表逐项一致。

当前状态如实记录如下。阶段 1 计划第 13 节新增决定五指定的注册落点 `crates/platform/obs/src/metrics/registry.rs` 已落地，上表十八项在该文件的定义表内逐项注册，名称、类型、标签集与本文件第 3 节登记表一致；禁止标签的登记也在同处以注册期判定拦下。唯一性校验子命令已由阶段 0 交付：`cargo xtask configdoc` 受理指标登记校验，判据是本文件登记表与代码侧注册表逐项一致且无重名，违例走文档漂移退出码；该子命令与 `docs/config-reference.md` 的校验同出一入口。阶段 2 任务 #11 与任务 #14 分别登记 `ep_db_tx_retries_total` 与 `ep_degradation_windows_open`，填充侧按第 4 节阶段 2 段落所述就位；阶段 4 任务 #22 登记六项授权与准入指标，任务 #23 登记身份面四项，十八项中除 `ep_high_risk_requests_open` 的填充面随 3b 同批就位外，其余十七项的注册与填充分工全部落实。

## 6. 维护纪律

- 一个指标只能由一个阶段注册。跨阶段复用同一指标时，追加的是标签取值而不是新指标名。
- 已登记的指标名不得改名。观测口径变化时新增指标并把旧指标标注为停止填充的版本，仪表盘的迁移在同一变更内完成。
- 新增指标的登记项必须同时给出类型、全部标签、取值域与注册填充两方，缺一不受理。
