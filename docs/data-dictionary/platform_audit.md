# platform_audit 数据字典

> **F-57 状态：`REGISTRY_PENDING_REBASELINE`。** 既有追加写/法人规则可复用；generation、dynamic authz、package/provider、objective/effect/evidence、authority epoch 和服务器外 checkpoint 字段须先再基线。
>
> **激活/owner task：Task 11。** 本分册目前不是 F-57 实现权威；Task 11 完成原子 business fact/audit/Outbox 再基线并显式激活前不得据此实施。

历史状态（F-57 下无效）：本分册曾与 `docs/data-dictionary.md`、阶段 3 计划共同构成开发前冻结契约。旧模型四张表均带 `legal_entity_id` 并 `ENABLE`、`FORCE` RLS；带 id 的目标建立 `UNIQUE(legal_entity_id,id)`，固定单目标引用建立真实外键并全部 `ON DELETE RESTRICT`。业务用户引用均指向 `platform_authz.user_legal_entity_grants(legal_entity_id,user_id)`；设备与重新认证挑战是全局身份证据，分别以单列真实外键指向 `platform_core.user_devices(id)` 与 `platform_core.reauth_challenges(id)`，写事务另校验用户及法人归属。只有 `approval_ref` 和带 `object_type/object_id` 的封闭多态审计对象属于无外键白名单。

## audit_segments

可更新的逐法人、逐上海自然日链段与成功锚定水位。

| 列名 | 类型 | 可空 | 默认 | 语义 |
|---|---|---|---|---|
| event_day | date | 否 | 无 | Asia/Shanghai 自然日；与法人组成唯一键 |
| first_seq | bigint | 是 | 无 | 首事件序号 |
| last_seq | bigint | 是 | 无 | 最新事件序号 |
| last_hash | bytea | 是 | 无 | 最新事件 SHA-256，非空时恰 32 字节 |
| event_count | bigint | 否 | 0 | 非负事件数 |
| state | text | 否 | 无 | `OPEN`、`CLOSED` |
| last_anchor_seq | bigint | 是 | 无 | 最近一次 **EVIDENCED** 锚点序号；尝试创建或仅签名不得推进 |
| last_anchored_at | timestamptz | 是 | 无 | 最近一次证据落盘并同事务推进水位的时间 |

公共列完整存在；建立 `UNIQUE(legal_entity_id,id)`、`ux_audit_segments_le_event_day`、`ix_audit_segments_le_created` 与 `ix_audit_segments_le_state_last_anchored_at`。首次写入以 `INSERT ... ON CONFLICT DO NOTHING` 建段；阶段 C 只有在锚点 CAS 推进为 EVIDENCED 成功后，才在同一事务更新本表的两项成功水位。

## audit_events

仅追加审计事实；不带 `row_version/updated_at/updated_by`，也不带无冲销语义的 `reverses_id`。

| 列名 | 类型 | 可空 | 默认 | 语义 |
|---|---|---|---|---|
| event_id | uuid | 否 | 应用 UUIDv7 | 主键；MCP transport 两动作按下文使用确定性 UUIDv5 例外；另建 `UNIQUE(legal_entity_id,event_id)` 供同法人引用 |
| legal_entity_id | uuid | 否 | 无 | RLS 与分段法人 |
| event_day | date | 否 | 无 | Asia/Shanghai 自然日 |
| seq | bigserial | 否 | sequence | 全局序号；事务回滚可留空洞，验证不要求连续 |
| prev_hash | bytea | 否 | 无 | 32 字节；段首为全零 |
| hash | bytea | 否 | 无 | 32 字节 SHA-256 |
| actor_user_id | uuid | 否 | 无 | 与法人组成真实复合外键指向用户法人授权 |
| actor_device_id | uuid | 是 | 无 | 真实单列外键指向全局设备登记行 |
| action | text | 否 | 无 | 审计动作 |
| object_type | text | 否 | 无 | 与 object_id 组成已登记封闭多态对象 |
| object_id | uuid | 是 | 无 | 多态对象 id |
| object_version | bigint | 是 | 无 | 对象版本证据 |
| before、after | jsonb | 是 | 无 | 已按字段权限掩码的前后快照 |
| reason | text | 是 | 无 | 最长 2000 |
| approval_ref | uuid | 是 | 无 | 平台审批证明白名单 |
| reauth_ref | uuid | 是 | 无 | 真实单列外键指向重新认证挑战 |
| client | text | 否 | 无 | F-57 exact-set：`win`、`mac`、`ios`、`android`、`portal`、`ops`、`mcp`、`system`；F-55 历史值 `server_admin` 不进入 current schema |
| occurred_at | timestamptz | 否 | 无 | 业务发生时点 |

建立 `ux_audit_events_le_event_day_seq`、`ix_audit_events_le_occurred`、`ix_audit_events_le_object_type_object_id_occurred`、`ix_audit_events_le_actor_user_id_occurred` 与 `ix_audit_events_le_action_occurred`。`object_type` 始终必须属于审计对象闭集；`object_id` 只允许该对象类型登记为对象级/部署级动作时为空，其他对象类型必填，形状由同一对象目录与写入守卫校验。

Stage 14 历史迁移撤销在同一编译期对象目录登记四类 action，不新建表或 Outbox event。通用 R0 action 为 `DATA_MIGRATION_REVERSED`，只允许 Stage 14 §4.12.1 的 25 个原 APPLY 根 object type，object id 必填，after 恰含 `{schema_version,data_migration_record_id,batch_id,apply_receipt_id,owner_effect_object_type,owner_effect_id}` 六键。另有三条 owner change action：`PROCURE_PURCHASE_ORDER_MIGRATION_REVERSED` 只配 `procure.purchase_orders`，`PROCURE_PAYMENT_REQUEST_MIGRATION_REVERSED` 只配 `procure.payment_requests`，`FINANCE_CASH_ACCOUNT_MIGRATION_REVERSED` 只配 `finance.cash_accounts`；三者 object id、object_version、before、after、reason、occurred_at 均必填。前两类 before/after 各恰含 `{schema_version:1,row_version,status}`，资金账户各恰含 `{schema_version:1,row_version,is_active,deactivated_at}`；三者 schema_version 均为 JSON number 1，row_version 均为不带前导零的正十进制 JSON string，不接受 JSON number。after 的版本必须逐字等于 object_version/根版本的规范十进制文本；真实变更时 before 等于根当前版本减一，终态保持时 before/after 与根版本相等；reason 固定 DATA_MIGRATION_REVERSED。三条 owner event 必须与各自 R0 event id 不同而同法人、同 occurred_at，R0.after.owner_effect_object_type/id 固定指 owner event。registry consistency test 对 action↔object、exact JSON key set/type、object id/version required、owner/R0 分离逐项等值比较；任意业务状态审计、别名或额外 migration owner action 均不放行。

F-55 的审计对象唯一登记落点为编译期 `crates/platform/audit/src/object_registry.rs`，不新建数据库目录：

- `reporting.ai.query_turn` 固定 object-level、id required，actions 恰为 `AI_QUERY_PLAN_COMPOSED|AI_QUERY_PLAN_EXECUTION_ATTEMPTED`。前者在安全计划可签发而 token 尚未返回时提交，后者在合法 token/current facts 已复核而 SQL 尚未执行时提交；object id 均为同一 UUIDv7 turn id、event id 使用默认 UUIDv7、`before=NULL`，strict after/空值规则逐字按 F-55 §3.8。审计失败则不签发 token 或零 SQL，结果与正文不入本表。
- `platform.mcp.invocation` 固定 object-level、id required，actions 恰为 `MCP_CALL_ATTEMPT|MCP_CALL_COMPLETION`，object id 为 UUIDv7 invocation id。两动作的 event id 分别按 namespace `3f9b8e44-78a5-5ff0-8fc9-6ad25a8a5c55` 与 `lowerhex(invocation_id bytes) + ":ATTEMPT|:COMPLETION"` 生成 UUIDv5，是默认 UUIDv7 的唯一例外。`before=NULL`；`after` 使用 F-55 §4.7 strict masked schema并含 `invocation_id`。pre-binding rejection 的 `binding_digest/request_schema_code/request_schema_version=NULL,input_field_codes=[]`；name/URI 只有成功规范解码后才留 `decoded_name_sha256`，且不存在与不可见同形。只有解析成功的 tools/call binding 才填 request schema，resources/read 即使已解析 binding 仍令 request schema 为空；四个 discover/list 方法无 binding/schema。未取得 terminal bytes 时 response hash/bytes 为 NULL/0，取得后即使 schema-invalid 也只留 hash/实际 bytes；output fields 仅验证通过后非空。其余 outcome/stable-code/response-schema 空值组合逐字按 F-55 §4.7，不留 raw tool name、expanded URI、对象 id、header/payload/extension/secret。

registry consistency test 必须把以上两对象、四 action、object-level/id-required 逐项等值比较，禁止字符串临时放行、别名或额外 F-55 action。MCP caller 在可信 identity 后先生成 ids 并预留专用 1 MiB completion slot，再取得 permit/rate/counter；outbound RequestEnd COMPLETE 后仍须先 commit ATTEMPT，再发 `DispatchAuthorized`，receiver 此前零副作用。core 发起与 job-worker 发起的 outbound 都必须携带原始、非空、仍可解析的 human `legal_entity_id/user_id/device/session/request` identity；系统伪用户、空 actor 或丢失来源的 job 不得调用 MCP。无法解析出合法法人/actor/device 的早期 MCP transport 拒绝不得伪造本表非空 FK，只写脱敏部署安全日志 `MCP_TRANSPORT_REJECTION`。completion slot/flush/DB/replay 无法确认时使用 `MCP.AUDIT.UNAVAILABLE`、禁止自动重试；reconciler 最多补 `UNKNOWN_AFTER_CRASH`，绝不重放外部调用。

## audit_anchors

段根签名与外部证据写入台账；可更新、可重试，不是仅追加表。

| 列名 | 类型 | 可空 | 默认 | 语义 |
|---|---|---|---|---|
| audit_segment_id | uuid | 否 | 无 | 与法人组成真实复合外键指向 `audit_segments(legal_entity_id,id)` |
| anchor_seq | bigint | 否 | 无 | 被锚定段序号 |
| root_hash | bytea | 否 | 无 | 恰 32 字节 |
| event_count | bigint | 否 | 无 | 非负 |
| algorithm | text | 否 | 无 | `ECDSA_P256_SHA256`、`RSA_PSS_SHA256` |
| key_ref | text | 否 | 无 | 签名密钥引用 |
| signature | bytea | 是 | 无 | 签名结果 |
| state | text | 否 | 无 | `PENDING_SIGN`、`SIGNED`、`EVIDENCED`、`FAILED` |
| signed_at | timestamptz | 是 | 无 | 签名时点 |
| evidence_path | text | 是 | 无 | create-new 证据路径 |
| evidence_written_at | timestamptz | 是 | 无 | 证据落盘时间 |
| attempts | int | 否 | 0 | `CHECK attempts BETWEEN 0 AND 9`；外部签名或证据写失败以 CAS 加一 |
| available_at | timestamptz | 否 | now() | 仅 `available_at <= now()` 可被重试扫描 |
| last_error | text | 是 | 无 | 最近失败摘要 |

公共列完整存在；建立 `UNIQUE(legal_entity_id,id)`、`ux_audit_anchors_le_segment_anchor_seq`、`ix_audit_anchors_le_created` 与重试索引 `ix_audit_anchors_le_state_available_at(legal_entity_id,state,available_at,id)`。状态形状为：PENDING_SIGN 的签名、签名时间与两项证据全空；SIGNED 的签名/时间全有而证据全空；EVIDENCED 四项全有；FAILED 可保留完整签名对但证据必须同空，任何状态下签名与签名时间同空同非空。阶段 A 只插入 PENDING_SIGN，不推进 segment；阶段 C 在同一事务把 anchor 推进 EVIDENCED 并更新 segment 成功水位。第 1..8 次失败按固定退避保持原可恢复态，第 9 次进入 FAILED；记名 replay 原地重置同一 anchor id，不生成后继或跳过步骤。

## audit_verifications

| 列名 | 类型 | 可空 | 默认 | 语义 |
|---|---|---|---|---|
| range_from、range_to | date | 否 | 无 | 验证范围，from 不晚于 to |
| single_event_id | uuid | 是 | 无 | 与法人组成真实复合外键指向 `audit_events(legal_entity_id,event_id)` |
| state | text | 否 | 无 | `QUEUED`、`RUNNING`、`PASSED`、`FAILED`、`ABORTED` |
| segments_total、segments_passed | int | 否 | 0 | 非负且 passed 不超过 total |
| first_failure_event_id | uuid | 是 | 无 | 与法人组成真实复合外键指向 `audit_events(legal_entity_id,event_id)` |
| first_failure_reason | text | 是 | 无 | 首失败原因 |
| report | jsonb | 是 | 无 | 验证报告 |
| requested_by | uuid | 否 | 无 | 与法人组成真实复合外键指向用户法人授权 |
| started_at、finished_at | timestamptz | 是 | 无 | 运行起止时点 |

公共列完整存在并建立 `UNIQUE(legal_entity_id,id)`、`ix_audit_verifications_le_created` 与 `ix_audit_verifications_le_state_created`。PASSED 必须无首失败字段；FAILED 必须有首失败原因，事件 id 在无法定位单事件时可空；运行时点与状态形状由阶段 3 状态机 CHECK 固定。
