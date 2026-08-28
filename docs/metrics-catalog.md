# 指标目录

> **F-57 状态：`REGISTRY_PENDING_REBASELINE`。** 现有指标保持名称稳定，但 F-57 P340/HDD、双层备份、动态权限、generation、长链自动化、包/provider 和四端证据尚须再基线；不得据此宣称生产监控已完整。

本文件是全部指标名的唯一登记处。指标名的唯一性由 CI 校验，同一指标只能由一个阶段注册，重复登记即构建失败。

## 1. 命名与暴露

命名形如 `ep_<subsystem>_<metric>_<unit>`。计数器以 `_total` 结尾，时长以 `_seconds` 结尾，字节数以 `_bytes` 结尾。

指标由 ops-agent 在 127.0.0.1:9101 以 Prometheus 文本格式暴露，仅内网可达，可对接客户已有的 Prometheus 与 Grafana。ops-agent 的 `/metrics` 聚合本机各进程的指标端点，抓取失败的目标按 `up=0` 标记，不静默丢弃。

## 2. 标签基数纪律

- 禁止把 `user_id`、`doc_no`、`trace_id` 作为标签。这三项的取值集合随业务量无上限增长，一旦进标签即为时序爆炸。
- 既有业务指标的 `legal_entity_id` 允许，理由是首版只有 2 个法人；F-55 新增的 15 项 AI/MCP/carrier 指标和 F-56 新增的 3 项许可/模块指标一律不带该标签。
- `route` 一律取模板路径而不是实例路径。

本纪律对下表 70 项逐项成立，新增指标须在登记时逐标签核对。

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
| `ep_degradation_windows_open` | gauge | 无 | 阶段 2 | 阶段 2 | 当前未关闭的降级窗口总数，台账每次开闭后刷新 |
| `ep_authz_decision_duration_seconds` | histogram | `legal_entity_id`、`outcome` | 阶段 4 | 阶段 4 | 授权判定四阶段流水线单次判定时长分布 |
| `ep_authz_denied_total` | counter | `legal_entity_id`、`reason` | 阶段 4 | 阶段 4 | 授权拒绝次数，reason 取九拒绝理由的指标标签形态 |
| `ep_authz_scope_truncated_total` | counter | `legal_entity_id` | 阶段 4 | 阶段 4 | 记录级部门闭包深度超限截断次数，伴随 WARN 日志 |
| `ep_reauth_challenges_total` | counter | `legal_entity_id`、`operation_type` | 阶段 4 | 阶段 4 | 高风险操作二次认证挑战签发次数 |
| `ep_authn_active_users` | gauge | 无 | 阶段 4 | 阶段 4 | 最近 60 秒内有请求的不同用户数；内部与门户合计，超过 20 人不作为拒绝条件 |
| `ep_sla_active_user_limit_exceeded_total` | counter | 无 | 阶段 4 | 阶段 4 | 活跃用户数由不超过 20 跃迁为超过 20 的次数；用于告警和标记 SLA 不适用区间 |
| `ep_authn_login_attempts_total` | counter | `outcome` | 阶段 4 | 阶段 4 | 登录尝试次数，outcome 取登录结果八值的指标标签形态 |
| `ep_authn_active_sessions` | gauge | 无 | 阶段 4 | 阶段 4 | 当前活跃会话数，认证中间件按空闲窗口内核验会话刷新 |
| `ep_breakglass_active_sessions` | gauge | 无 | 阶段 4 | 阶段 4 | 当前应急账号活跃会话数，认证中间件刷新 |
| `ep_high_risk_requests_open` | gauge | `legal_entity_id` | 阶段 4 | 阶段 3b 同批 | 未结束高风险请求数，法人维度刷新；高风险请求四端点属 3b 同批交付，填充面随其就位，本阶段判据是指标名存在（同裁定 C-23 分工） |
| `ep_outbox_pending_events` | gauge | `event_type` | 阶段 3 | 阶段 3 | 当前待投递 Outbox 条目数 |
| `ep_outbox_dispatch_attempts_total` | counter | `event_type`、`outcome` | 阶段 3 | 阶段 3 | Outbox 派发尝试次数 |
| `ep_dead_letters_open` | gauge | `consumer` | 阶段 3 | 阶段 3 | 当前未解决死信数 |
| `ep_audit_anchor_age_seconds` | gauge | `legal_entity_id` | 阶段 3 | 阶段 3 | 各法人最近一次成功审计锚定距今秒数 |
| `ep_audit_evidence_write_failures_total` | counter | `stage` | 阶段 3 | 阶段 3 | 审计证据签名或写出失败次数；stage 取 sign、write |
| `ep_flow_instances_manual_intervention` | gauge | `reason` | 阶段 3 | 阶段 3 | 当前处于人工干预状态的流程实例数 |
| `ep_audit_segment_lock_wait_seconds` | histogram | `legal_entity_id` | 阶段 3 | 阶段 3 | 审计段锁等待时长 |
| `ep_license_status_info` | gauge | `license_kind`、`status` | 阶段 3（F-56） | 阶段 3（F-56） | 当前许可只暴露一条值为 1 的样本；kind 与四态均来自已验签 current grant |
| `ep_license_usage_over_limit` | gauge | `dimension` | 阶段 3（F-56） | 阶段 3（F-56） | 三项许可计量当前是否超限，值只取 0/1；超限不驱动业务拒绝 |
| `ep_module_install_state_info` | gauge | `module`、`state` | 阶段 3（F-56） | 阶段 3（F-56） | 15 个内置模块各只暴露当前三态中一条值为 1 的样本 |
| `ep_mdm_change_requests_open` | gauge | `legal_entity_id`、`object_type` | 阶段 5 | 阶段 5 | 当前未结束主数据变更申请数 |
| `ep_mdm_import_rows_total` | counter | `object_type`、`outcome` | 阶段 5 | 阶段 5 | 主数据导入逐行结果数 |
| `ep_mdm_qualification_expired_total` | gauge | `legal_entity_id` | 阶段 5 | 阶段 5 | 当前已过期资质数；沿用阶段 5 冻结名，语义为当前量 |
| `ep_cpq_price_resolve_duration_seconds` | histogram | 无 | 阶段 5 | 阶段 5 | 批量取价耗时 |
| `ep_cpq_price_resolve_hit_count` | histogram | 无 | 阶段 5 | 阶段 5 | 单次批量取价的命中行数分布，不是累计 counter |
| `ep_recon_run_duration_seconds` | histogram | `run_kind`、`outcome` | 阶段 9 | 阶段 9 | 统一对账执行器一次运行耗时 |
| `ep_recon_unfinished_total` | gauge | `run_kind`、`reason` | 阶段 9 | 阶段 9 | 当前未完成对账项数；沿用基线冻结名，语义为当前量 |
| `ep_period_close_rejected_total` | counter | `reason` | 阶段 9 | 阶段 9 | 关账受理或结论被拒次数 |
| `ep_ledger_posting_duration_seconds` | histogram | `source_kind` | 阶段 9 | 阶段 9 | 总账过账耗时 |
| `ep_ledger_deferred_vouchers_total` | counter | `legal_entity_id` | 阶段 9 | 阶段 9 | 因期间顺延产生的凭证数 |
| `ep_ledger_open_periods` | gauge | `legal_entity_id` | 阶段 9 | 阶段 9 | 当前开放会计期间数 |
| `ep_ledger_period_close_window_seconds` | histogram | `legal_entity_id`、`conclusion` | 阶段 9 | 阶段 9 | 关账请求从受理到结论的窗口时长 |
| `ep_finance_settlement_conflicts_total` | counter | `conflict_kind` | 阶段 10 | 阶段 10 | 核销并发或余额漂移冲突次数 |
| `ep_finance_reconciliation_difference_amount` | gauge | `item`、`legal_entity_id` | 阶段 10 | 阶段 10 | 财务对账项当前差额；金额以主币最小单位表达 |
| `ep_invoice_import_rows_total` | counter | `outcome` | 阶段 10 | 阶段 10 | 发票导入逐行结果数 |
| `ep_analytics_query_duration_seconds` | histogram | `dataset`、`query_kind`、`legal_entity_id` | 阶段 11 | 阶段 11 | 受治理分析查询耗时 |
| `ep_analytics_query_terminated_total` | counter | `reason` | 阶段 11 | 阶段 11 | 因资源或边界终止的分析查询数 |
| `ep_report_render_duration_seconds` | histogram | `task_kind`、`output_format` | 阶段 11 | 阶段 11 | 报表制品渲染耗时 |
| `ep_report_render_queue_depth` | gauge | 无 | 阶段 11 | 阶段 11 | 当前待渲染任务数 |
| `ep_costing_entries_written_total` | counter | `side`、`source_type` | 阶段 11 | 阶段 11 | 成本或收入捕获追加条目数 |
| `ep_service_work_orders_open` | gauge | `legal_entity_id`、`status` | 阶段 12 | 阶段 12 | 当前未终态工单数 |
| `ep_service_work_order_open_lines` | gauge | `legal_entity_id` | 阶段 12 | 阶段 12 | 当前未完成工单行数 |
| `ep_crm_customer360_section_duration_seconds` | histogram | `section` | 阶段 12 | 阶段 12 | 客户 360 区块查询耗时 |
| `ep_crm_customer360_section_degraded_total` | counter | `section` | 阶段 12 | 阶段 12 | 客户 360 区块降级次数 |
| `ep_project_contract_derivation_tasks_total` | counter | `outcome` | 阶段 12 | 阶段 12 | 合同派生项目任务结果数 |
| `ep_ai_inference_requests_total` | counter | `outcome` | 阶段 13c（F-55） | 阶段 13c（F-55） | 本地模型推理请求结果数；只统计进入 AI 边界的请求 |
| `ep_ai_inference_duration_seconds` | histogram | `outcome` | 阶段 13c（F-55） | 阶段 13c（F-55） | 本地模型推理从受理到返回或受控失败的时长 |
| `ep_ai_inference_queue_depth` | gauge | 无 | 阶段 13c（F-55） | 阶段 13c（F-55） | AI 当前排队项数，固定不超过 30 |
| `ep_ai_working_set_bytes` | gauge | 无 | 阶段 13c（F-55） | 阶段 13c（F-55） | AI 进程当前 working set 字节数 |
| `ep_ai_job_memory_limit_bytes` | gauge | 无 | 阶段 13c（F-55） | 阶段 13c（F-55） | 按 F-55 算定并施加的 Job Object 内存硬上限 |
| `ep_ai_gpu_vram_bytes` | gauge | 无 | 阶段 13c（F-55） | 阶段 13c（F-55） | GPU profile 当前显存占用；CPU profile 固定为 0 |
| `ep_ai_plan_validations_total` | counter | `outcome`、`reason` | 阶段 13c（F-55） | 阶段 13c（F-55） | 模型计划确定性校验结果数 |
| `ep_mcp_calls_total` | counter | `direction`、`transport`、`method`、`outcome` | 阶段 13c（F-55） | 阶段 13c（F-55） | MCP 六方法调用结果数 |
| `ep_mcp_call_duration_seconds` | histogram | `direction`、`transport`、`method`、`outcome` | 阶段 13c（F-55） | 阶段 13c（F-55） | MCP 单请求端到端时长 |
| `ep_mcp_payload_bytes_total` | counter | `direction`、`flow` | 阶段 13c（F-55） | 阶段 13c（F-55） | 经收容边界接收或发送的 MCP payload 累计字节数 |
| `ep_mcp_active_grants` | gauge | 无 | 阶段 13c（F-55） | 阶段 13c（F-55） | 当前仍有效的入站人类 grant 数 |
| `ep_mcp_denials_total` | counter | `reason` | 阶段 13c（F-55） | 阶段 13c（F-55） | MCP 在协议、manifest、grant 或逐次权限重检处的拒绝数 |
| `ep_mcp_local_children` | gauge | `transport` | 阶段 13c（F-55） | 阶段 13c（F-55） | 当前本地 MCP 受控子进程数 |
| `ep_mcp_local_forced_terminations_total` | counter | `reason` | 阶段 13c（F-55） | 阶段 13c（F-55） | 本地 MCP 子进程被收容器强制终止的次数 |
| `ep_deployment_carrier_info` | gauge | `carrier` | 阶段 13c（F-55） | 阶段 14 | 当前部署 carrier 信息；只为实际 carrier 暴露值为 1 的单一样本 |
| `ep_archive_write_lag_seconds` | gauge | 无 | 阶段 14 | 阶段 14 | 连续 WAL 写出最近成功点距今秒数 |
| `ep_attachment_write_lag_seconds` | gauge | 无 | 阶段 14 | 阶段 14 | 附件增量写出最近成功点距今秒数 |
| `ep_backup_last_success_timestamp_seconds` | gauge | `backup_kind` | 阶段 14 | 阶段 14 | 最近一次有效备份成功的 Unix 时间戳 |

### 3.1 取值域与桶

- `pool` 只取 `rw`、`ro`、`worker`、`ops` 四值，与 ADR-0018 曾冻结的 `PoolKind { Rw, Ro, Worker, Ops }` 逐项对应（**F-67 注：ADR-0019 已明文取代 ADR-0018 的固定四池，四值在此仅作导入种子；现行权威为 ADR-0019 的按签名代 exact 登记**）。
- `client` 取 `win`、`mac`、`ios`、`android`、`portal`、`ops`、`mcp` 七值，与 F-57 `ClientKind` 七个变体逐项对应；Control Center 固定为受信 `ops`，不存在 `server_admin`；`mcp` 只由 `/mcp` grant middleware 写入，`system` 只进入审计而不进入该指标标签。
- `status_class` 取 `2xx`、`3xx`、`4xx`、`5xx`。
- `ep_http_request_duration_seconds` 的桶固定为 0.05、0.1、0.25、0.5、1、2、3、5、10、30，与技术基线第 9.2 节逐值一致。改桶等同于改指标，须走一次登记变更。
- `ep_db_statement_duration_seconds` 的桶由阶段 2 任务 #11 定死：0.0005、0.001、0.0025、0.005、0.01、0.025、0.05、0.1、0.25、0.5、1、2.5、5、10、30，十五个桶 0.5 毫秒起步、逐段约 2.5 倍递增，30 秒封顶对齐各池 statement_timeout 的最大取值。改桶等同于改指标，须走一次登记变更。
- `sqlstate` 取 `40001`、`40P01` 两值，与重试策略的可重试 SQLSTATE 表逐值一致。
- `ep_authz_decision_duration_seconds` 的桶由阶段 4 任务 #22 定死：0.0001、0.00025、0.0005、0.001、0.0025、0.005、0.01、0.025、0.05、0.25、1，十一桶 0.1 毫秒起步，对齐纯逻辑判定 P95 低于 1 毫秒的目标口径；`outcome` 取 `allowed`、`denied` 两值。改桶等同于改指标，须走一次登记变更。
- `ep_authn_active_users` 固定使用最近 60 秒窗口，管理端每 5 秒读取；`ep_sla_active_user_limit_exceeded_total` 只在从 20 或以下跃迁到 21 或以上时增加一次，持续超限不重复累加。二者均不得驱动登录或写入拒绝。
- `ep_authz_denied_total` 的 `reason` 取九拒绝理由的小写下划线形态；`ep_reauth_challenges_total` 的 `operation_type` 必须恰取 `CONTRACT_EFFECTIVE|PAYMENT|INVOICE_ISSUE|LEDGER_POSTING|PERIOD_CLOSE|SENSITIVE_EXPORT|DATA_MIGRATION` 七个 `HighRiskOperation` 的 `SCREAMING_SNAKE_CASE` 值。指标 fixture 必须断言值集数量为 7 且与共享枚举逐项相等，不得缩成移动端受限操作子集。
- `ep_authn_login_attempts_total` 的 `outcome` 取登录结果八值的小写下划线形态，取值集与 `crates/platform/identity/src/types.rs` 的 `LoginAttemptOutcome` 逐项一致：success、credential_invalid、account_locked、account_inactive、mfa_required、mfa_invalid、device_unregistered、rate_limited。挑战过期与末因子禁入归 mfa_invalid；`rate_limited` 只表示认证前端点的登录名/来源地址速率限制，不表示活跃用户规模超限。
- `event_type` 与 `consumer` 只能取 `docs/event-catalog.md` 的登记值；`outcome` 由各行含义冻结为有限枚举，不得填原始错误文本。`reason`、`conflict_kind`、`run_kind`、`source_kind`、`item`、`dataset`、`query_kind`、`task_kind`、`output_format`、`side`、`source_type`、`section`、`status` 与 `backup_kind` 必须来自代码封闭枚举或唯一目录，不得使用用户输入。
- 新增 histogram 的桶固定如下：审计段锁 0.001、0.005、0.01、0.025、0.05、0.1、0.2、0.5、1、3；取价、总账过账、客户 360 区块沿用 HTTP 桶；对账、关账窗口、分析查询与报表渲染用 0.1、0.25、0.5、1、2、3、5、8、10、30、60、120、300、900、3600。`ep_cpq_price_resolve_hit_count` 的桶为 0、1、5、10、20、50、100、200。
- F-55 AI/MCP 标签闭集固定如下，不得把名称、ID、版本、provider、region、endpoint、路径或错误正文塞入标签：
  - AI inference `outcome` 只取 `ok|invalid_plan|timeout|busy|model_error|contained`；AI validation `outcome` 只取 `accepted|rejected`，`reason` 只取 `none|schema_version|dataset|field|list_cardinality|forbidden_construct|operator_arity|literal_type|result_code|limit|extra_field|payload_size`。accepted 必须配 `none`，rejected 不得配 `none`。
  - MCP `direction` 只取 `inbound|outbound`；`transport` 只取 `inbound_https|remote_streamable_http|local_signed_stdio|local_windows_hyperv_container`；`method` 只取 `server/discover|tools/list|tools/call|resources/list|resources/templates/list|resources/read`；call `outcome` 只取 `ok|cancelled|denied|invalid_request|timeout|unavailable|schema_invalid|contained`；`flow` 只取 `request|response`。stdio `notifications/cancelled` 与 IPC `DispatchAuthorized` 都是 transport control，不计作 method 样本。
  - MCP denial `reason` 只取 `invalid_request|payload_size|protocol_version|protocol_header|method|manifest|capability|grant|device_proof|tool_not_visible|resource_not_visible|high_risk|idempotency|credential|response_schema|local_containment|audit_unavailable|rate_limit|timeout`；本地强制终止 `reason` 只取 `deadline|caller_disconnect|job_limit|child_attempt|protocol_violation|containment_violation`；`ep_mcp_local_children.transport` 只取两种 local transport。
  - carrier 只取 `customer_controlled_physical|customer_controlled_domestic_iaas_vm`。
- F-56 标签闭集：`license_kind` 只取 `perpetual|subscription`，`status` 只取 `active|expiring_soon|grace_period|restricted`，`dimension` 只取 `legal_entity|named_user|registered_device`，`module` 只取 15 个 `ModuleCode` wire lowercase，模块 `state` 只取 `not_installed|installed_enabled|installed_disabled`。禁止 license number、package code/version、signer、deployment id 或原因正文进入标签。
- `ep_ai_inference_duration_seconds` 的桶固定为 0.05、0.1、0.25、0.5、1、2、3、5、10、30、60、120；`ep_mcp_call_duration_seconds` 的桶固定为 0.005、0.01、0.025、0.05、0.1、0.25、0.5、1、2、3、5、10、30。改桶等同于改指标。

## 4. 阶段 1 的登记范围

阶段 1 登记且只登记上表中注册方为阶段 1 的五项，五项一次性注册在同一处注册表内；`ep_db_tx_retries_total` 与 `ep_degradation_windows_open` 由阶段 2 登记，不在阶段 1 范围。阶段 1 不登记任何超出范围的指标名，也不登记任何与已登记项同义的别名——同义名是重复登记的主要来源，本文件不设废弃名一节，作废指标名的追溯记录留在裁定登记文件，不在本文件出现。

规格第 15.3 章要求的降级与暴露窗口台账既进数据库表也各出一个 gauge，两处不得只有其一；该 gauge 即上表 `ep_degradation_windows_open`，由阶段 2 任务 #14 登记并填充：数据库侧台账是 `platform_ops.degradation_windows`，实现侧在每次开闭窗口后刷新 gauge，取值与台账未关闭窗口数一致。

阶段 2 登记的指标是上表中注册方为阶段 2 的两项：`ep_db_tx_retries_total`（任务 #11 随数据库驱动一并登记）与 `ep_degradation_windows_open`（任务 #14 登记）。注册落点与阶段 1 相同，仍是 `crates/platform/obs/src/metrics/registry.rs` 的同一张定义表；截至阶段 2 共七项，不另立注册表。阶段 2 同时填充上表中填充方为阶段 2 的四项：`ep_db_pool_connections` 与 `ep_db_statement_duration_seconds` 由两个应用各自经 `ObsDbMetrics` 桥接在数据库装配处填充，`ep_db_tx_retries_total` 在事务重试路径填充，`ep_degradation_windows_open` 在降级台账开闭后填充；填充面落在 `crates/adapter/db-pg` 与两应用的 `wiring/metrics.rs`，阶段 1 注册时判据所要求的「指标名存在」之外，本阶段起四项均有非零样本路径。

阶段 4 任务 #22 登记上表中注册方为阶段 4 的六项：授权判定时长与拒绝计数、部门闭包截断计数、复核挑战签发计数、60 秒活跃用户 gauge 与活跃用户上限越界计数，注册落点仍是同一张定义表；填充面在 `crates/platform/authz` 的判定路径与认证中间件的活跃用户跟踪器，经装配侧桥接进注册表。

阶段 4 任务 #23 登记身份面四项：登录尝试计数与两项会话 gauge、未结束高风险请求 gauge，注册落点仍是同一张定义表（十三项扩为十七项）。填充面：两项会话 gauge 在认证中间件的活跃会话台账（`apps/core-server/src/platform/middleware.rs` 的 `SessionTracker`）刷新；登录尝试计数在登录端点成功与失败分支及 PRE_AUTH 限流拒入点填充（`platform/identity.rs` 与同文件中间件）；`ep_high_risk_requests_open` 的填充面属 3b 同批的高风险请求端点，本阶段只注册不填充，判据同裁定 C-23。

## 5. 机器判定与当前状态

指标名唯一性校验由 `xtask` 实现，判据是本文件登记表内无重名，且登记表与代码侧注册表逐项一致。

当前目标登记集固定为上表 70 项。F-54 对其既有 52 项执行的引用差集基线继续有效；F-55 追加 15 项，F-56 追加 3 项，均由同一校验纳入，不建立第二张注册表。剔除 crate、数据库角色/schema、测试库前缀、明确作废的 `ep_quota_throttled_total`、`ep_replication_crosscheck_age_seconds`、`ep_db_replication_crosscheck_age_seconds`，并把旧别名 `ep_db_retries_total`、`ep_tx_retry_total` 统一回 `ep_db_tx_retries_total` 后，指标 `referenced-minus-registered` 必须为 0。代码侧注册表、填充点与 `cargo xtask configdoc` 必须在首批实施中逐项与本表对齐；在该校验真实返回 0 之前能力状态为 `UNVERIFIED`，不阻塞按本目录开发，但不得声称登记漂移门禁已经验证通过。

**F-67 注：本段的阶段分派按旧十四阶段口径写成，其中「阶段 13c（F-55）」十五项的承接方已被 ADR-0019 撤销强制性（无 `ai-inferer` 进程即无 `ep_ai_working_set_bytes` 的被测对象）；本表在 G0 后由 generated metrics catalog 取代为机器真值，此处分派只作导入种子读。** 阶段注册责任固定为：阶段 1 五项，阶段 2 两项，阶段 3 十项（含 F-56 三项），阶段 4 十项，阶段 5 五项，阶段 9 七项，阶段 10 三项，阶段 11 五项，阶段 12 五项，阶段 13c（F-55）十五项，阶段 14 三项，合计 70。阶段 6、7、8、13（不含 13c）不新增指标，只填充通用 HTTP、数据库、Outbox 或对账指标；不得保留未命名配额。阶段 14 的三项写出/备份指标是基线早已具名的指标，不产生任何 Outbox 事件；F-55 的 AI/MCP/carrier 与 F-56 许可/模块指标同样不新增 Outbox 事件。

## 6. 维护纪律

- 一个指标只能由一个阶段注册。跨阶段复用同一指标时，追加的是标签取值而不是新指标名。
- 已登记的指标名不得改名。观测口径变化时新增指标并把旧指标标注为停止填充的版本，仪表盘的迁移在同一变更内完成。
- 新增指标的登记项必须同时给出类型、全部标签、取值域与注册填充两方，缺一不受理。
