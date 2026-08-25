# platform_flow 数据字典

> **F-57 状态：`REGISTRY_PENDING_REBASELINE`。** 旧流程实例只作执行状态机底座；F-57 objective、obligation、effect、evidence、closure、cycle、incident、lease 和 checkpoint 尚须追加后才是完整模型。
>
> **激活/owner task：Task 12。** 本分册目前不是 F-57 实现权威；Task 12 完成耐久 objective-to-evidence kernel 再基线并显式激活前不得据此实施。

历史状态（F-57 下无效）：本分册曾与 `docs/data-dictionary.md`、阶段 3 计划共同构成开发前冻结契约。旧模型中所有法人级表均以 `legal_entity_id` 为唯一 RLS 判据并 `ENABLE`、`FORCE`；同法人单目标引用使用真实复合外键。

## process_instances

流程实例为可更新运行台账。`variables jsonb not null default '{}'` 只允许流程定义以 JSON Schema 逐键声明的非敏感路由元数据，字段密级必须小于 30。允许示例为业务对象判别与 id、关联/因果 id、owner module、scenario、action、`approval_command_snapshot_id`；禁止业务命令 DTO、HTTP 请求体、付款/账户/税号/身份明文、附件正文、密文副本、可逆片段及未经密钥保护的命令摘要。审批命令的唯一持久载体是下表，流程实例只保存其 id。

目标候选键 `UNIQUE(legal_entity_id,id)` 供所有流程子对象的同法人复合外键引用。

## process_steps 与 process_compensations

`process_steps` 另建 `UNIQUE(legal_entity_id,id)` 与 `UNIQUE(legal_entity_id,process_instance_id,id)` 两个候选键。`process_compensations.reverses_id` 不是公共占位列，而是每条补偿事实必填的真实父链；以 `(legal_entity_id,process_instance_id,reverses_id)` 复合外键指向 `process_steps(legal_entity_id,process_instance_id,id) ON DELETE RESTRICT`，从数据库层禁止跨法人或跨流程实例补偿。其余流程仅追加表没有逐行反向父链，不带 `reverses_id`。

## approval_command_snapshots

高保密审批命令快照；一条流程实例恰有一条。不是仅追加表，因为状态允许一次合法推进；除状态控制列外其余列全部不可变。

| 列名 | 类型 | 可空 | 默认 | 语义 |
|---|---|---|---|---|
| process_instance_id | uuid | 否 | 无 | 与法人组成真实复合外键指向 `platform_flow.process_instances(legal_entity_id,id) ON DELETE RESTRICT`；`UNIQUE(legal_entity_id,process_instance_id)` |
| owner_module | text | 否 | 无 | `ModuleCode` 序列化值，命令解密与执行属主 |
| scenario | text | 否 | 无 | 稳定审批场景码，长度 1..64 |
| action | text | 否 | 无 | 稳定命令动作码，长度 1..64 |
| schema_version | int | 否 | 无 | 大于 0，由 owner 模块逐场景冻结 |
| command_enc | bytea | 否 | 无 | AES-256-GCM 信封密文，命令 DTO 唯一持久载体；非空 |
| command_key_ref | text | 否 | 无 | 当前法人 FIELD/密级 30 数据密钥引用 |
| command_digest | bytea | 否 | 无 | 32 字节 `SHA-256(command_enc || canonical_aad)`，不作查询或明文等值索引 |
| request_hash | bytea | 否 | 无 | 32 字节，仅覆盖非敏感规范路由封套与幂等键，不覆盖命令明文 |
| state | text | 否 | `PENDING` | `PENDING`、`CONSUMED`、`REJECTED`、`EXPIRED` |
| consumed_at | timestamptz | 是 | 无 | CONSUMED 唯一非空终态时间 |
| expired_at | timestamptz | 是 | 无 | EXPIRED 唯一非空终态时间 |
| result_object_type | text | 是 | 无 | CONSUMED 必填的 owner 稳定对象类型码，长度 1..64 |
| result_object_id | uuid | 是 | 无 | CONSUMED 必填；与类型组成封闭多态执行结果定位 |
| result_doc_no | text | 是 | 无 | CONSUMED 按对象可空；非空时长度 1..64，仅冗余展示 |

公共列全部存在；`security_level` 固定为 30。`created_by`、`updated_by` 与法人组成复合外键指向 `platform_authz.user_legal_entity_grants(legal_entity_id,user_id)`。另有 `UNIQUE(legal_entity_id,id)`、`ix_approval_command_snapshots_le_state_created`。

状态只允许 `PENDING -> CONSUMED|REJECTED|EXPIRED`，三个终态无出边。PENDING/REJECTED 的两个时间与三项结果定位均空；EXPIRED 仅 `expired_at` 非空且结果全空；CONSUMED 仅 `consumed_at` 非空，type/id 必填而 doc_no 可空。更新触发器只允许 PENDING→CONSUMED 的同一 UPDATE 一次写入三项结果定位，转 REJECTED/EXPIRED 不得写，终态后不可变；业务对象创建与该 UPDATE 同一事务。密文、key ref、两个摘要、实例、owner/scenario/action/schema_version 与创建证据逐列不可变。结果 type/id 是 owner 管理的封闭多态组合，不建伪外键；以 approval_ref 定位流程实例后经本表唯一键取得结果定位。

敏感字段登记使用逻辑列 `command`：`LEGAL / 30 / encrypted=true / blind_index=NONE / mask=FULL / normalization=NONE / MIGRATION:20261013093700`。物理表不得出现 `command` 或 `command_bidx`。建表迁移为 `V20261013093600__platform_flow_create_approval_command_snapshots.sql`，登记迁移为 `V20261013093700__platform_flow_backfill_sensitive_field_registry.sql`。
