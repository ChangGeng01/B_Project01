## 阶段 14：运维、备份与发布硬化

本阶段交付两件事。一是把规格第 13.3 章唯一的可用性类承诺即可恢复性，从纸面变成机器上持续运行、可被检出、可被诚实披露的机制。二是把第 17.2、17.5 章、附录 A、附录 D 与第 22 章的门禁判定变成可执行工装与可归档证据包。本阶段不新增业务模块、不新增进程、不新增 schema、不新增模块码、不新增错误分类、不新增依赖方向。凡涉及账务的一律指向规格第 5.2 章事件-分录表与第 10.2 章，本文不复述借贷与取价。

### 0. 偏离共享基线与本阶段新增决定

偏离一，落点写出直接出网，不经 integration-gateway。基线第 2 节写 integration-gateway 是首版唯一对外出网进程。规格第 13.4 章认证的落点类型含客户对象存储，同章落点侧访问控制条规定写出侧凭据只由写出组件的系统账户持有、不复用于其他进程，第 7.7 章又逐项枚举了两个写出进程的凭据持有范围。因此对象存储落点的写出必须由 archive-writer 与 backup-writer 自身发起。本阶段把该句收窄为 integration-gateway 是首版唯一面向外部业务系统的出网进程，落点写出不在其内，并提出基线第 2 节修订。影响范围：两个写出进程的 systemd 单元需放开到落点的出向网络策略，目的地址集合固定为部署记录所载落点，不接受运行期变更。

偏离二，platform_ops 台账表不带 legal_entity_id，也不建行级策略。基线第 3.8 节把 platform_ops 的机器级指标列为不带法人列的四类之一，第 4 节又要求每张业务表带 legal_entity_id。规格第 15.3 章要求台账同时覆盖两类按法人与会计期间归属的条目，即内部对账校验未完成与关账受理被拒。若给台账加 legal_entity_id 并套第 3.8 节模板，部署级条目的法人列必然为空，行级策略下 NULL 比较结果为 NULL，这些条目对任何人都不可见，台账失效。本阶段取值：台账表不带 legal_entity_id，改带 scope_legal_entity_id 与 scope_accounting_period_id 两个可空的展示归属标注列，命名刻意与 legal_entity_id 区分以免被迁移生成器套用模板；不建策略；读取侧可见性由 ABAC 在应用层按运维管理员、安全管理员与审计管理员三类角色判定。台账不承载任何业务数据，与第 3.8 节该四类的口径一致。该偏离由阶段 2 先行落实：platform_ops.degradation_windows 一张表由阶段 2 按 A-26 建立，列定义与本文第 3.1 节表 3 完全一致，并交付 ux_degradation_windows_kind_scope_closed 与 ck_degradation_windows_open_order 两条约束；本阶段只做 kind 取值扩展、追加两条 CHECK 与全部索引，不重建表、不增删列。

偏离三，平台端点的路径模块段取 platform。基线第 5.1 节的 module 段只枚举了 15 个业务模块码，错误码段已允许 PLATFORM。本阶段取值：平台自身资源路径固定为 /api/v1/platform/<resource-plural>，事件类型的模块段同样允许取 platform，与错误码的 PLATFORM 段一一对应，并提出基线第 5.1 节与第 6.1 节修订。

偏离四，ops-agent 的两个端点不使用第 5.2 节封套。/metrics 输出 Prometheus 文本格式，/healthz 与 /readyz 输出精简 JSON，二者不带 success 与 error 字段，也不要求 Authorization 与 Idempotency-Key。理由是其消费方为 Prometheus 与 systemd，封套使之不可解析。二者只监听 127.0.0.1，不承载任何业务数据。

本阶段新增决定，基线未覆盖，阶段结束时回写基线：落点可写性判定的连续失败与连续成功阈值、三类写出的周期取值、部署级备份加密的算法与对象格式、恢复模式的触发方式、台账 kind 枚举、RPO 依据枚举的排序算法、写出进程本地暂存上限。逐项取值见第 4 节与第 7 节。

被阻塞事项：PRD 附录乙的 U-L-10 与 U-L-11 未决。U-L-10 影响台账的可见角色范围、界面入口与导出格式，本阶段临时取值为可见角色三类、入口在运维中心一级导航、导出格式为 JSON 与 CSV 两种，切换代价只在前端与导出适配层，服务端模型不变。U-L-11 影响诚实披露在界面内的呈现位置与客户确认是否留痕，本阶段临时取值为独立的部署状态与已知限制页面且客户确认留痕写审计，切换代价同样只在前端。两项都不阻塞服务端交付。

---

### 1. 交付物清单

本阶段结束时下列东西存在且可运行。

1. archive-writer 可执行进程，承载三项写出：事务日志连续归档写出、附件正文向服务器之外落点的增量写出、审计证据存储向落点的写出，三项各自不超过 15 分钟周期，三项之间的先后由进程内部调度落实。含附件正文写出点水位推进器、本地 spool 暂存与补写、落点可写性持续判定。审计证据目录 /var/lib/ep/audit-evidence 的属主为 ep-worker、组为 ep、权限 0750，本进程以组 ep 的只读权限读取并写出到服务器之外落点，对该目录只有读权限，不具备写入与删除权限，证据文件与段根签名由 job-worker 产生。
2. backup-writer 可执行进程，承载四项：每日全量基础备份（流式，本机只留暂存缓冲）、附件正文的存量引导搬运与每日全量写出、备份自动校验、归档链断裂后重建恢复基线的那一次全量基础备份。另承载配置、证书、模块包、低代码规则包与基础设施定义的随日全量同批写出。另含恢复模式，承担整机失效恢复与密钥恢复材料隔离恢复两类演练的编排。
3. ops-agent 可执行进程，暴露 127.0.0.1:9101 指标端点与 127.0.0.1:9102 健康端点，以 ep_ops_ro 只读角色读取运维视图。
4. core-server 内的运维中心用例集与只读 API：降级与暴露窗口台账、两个 RPO 取值与依据、备份集与校验结论、归档通道状态、复制会话与复制槽交叉核对、容量水位、配额事件、部署记录、密钥恢复材料核验登记、恢复演练登记。
5. core-server 内的写出上报受理器：接收两个写出进程经 Unix domain socket 上报的四类内容，在同一事务内写 platform_ops 表、写审计事件、写 Outbox 条目，并按第 15.3 章开闭暴露窗口。
6. core-server 内的复制交叉核对器：以只读分析池的单列固定连接按不长于 5 分钟周期读取 pg_stat_replication 与 pg_replication_slots，与上报记录逐条比对。
7. ep-adapter-sink，落点适配层，三种认证落点类型的统一写入、读回、探针与吞吐实测。
8. 部署级备份加密实现，落在 ep-adapter-kms 之上，实例级密钥、信封加密、写出前施加、附件正文保持法人密钥域原密文不二次加密。
9. 归档通道状态机与断链处置器，含落点可写与不可写两支，含归档通道暂停终态。
10. 恢复编排与恢复点对齐算法，含附件元数据与正文逐条一致性校验的流式实现。
11. 性能与容量认证工装 ep-bench：负载生成器、必判必记项采集器、认证报告生成器。
12. 发布门禁工装 ep-release-gate：证据收集、按第 17.2 章通过标准与第 22 章十五条逐条判定、发布证据包组装。
13. 供应链安全流水线：SBOM、签名、可复现构建、离线依赖仓库、客户侧验签工具。
14. 等级保护三级控制项自评矩阵与四项永久性不符合项封闭清单，落在 docs/compliance/ 并由 CI 校验不得超出封闭清单。
15. 恢复手册、运维手册、部署记录模板与交付说明的诚实披露八条文本，落在 docs/runbooks/ 与 docs/delivery/。
16. OpsDisposalService，位于 crates/platform/obs/src/disposal.rs，实现阶段 3b 在 crates/platform/file/src/port/disposal.rs 定义的 DisposalPort，承担附件对象、密钥域、备份集与扩展表四类处置范围的执行，含密钥销毁与到达备份保留期的备份集处置，产出销毁证明。

---

### 2. crate 与进程归属

新增 crate。

| crate | 归属进程 | 职责 |
|---|---|---|
| ep-adapter-sink | archive-writer、backup-writer | 三类落点的写入、读回、列举、探针、吞吐实测；不含加密、不含业务语义 |
| ep-adapter-replication | archive-writer、backup-writer | PostgreSQL 流复制协议客户端：复制槽建立与查询、WAL 流接收、基础备份流式接收；不含落点语义 |
| ep-bench | 不随产品交付 | 负载生成器与认证采集器，位于 tools/bench/，不在 crates/ 下，工作区成员，不进入发布制品与 SBOM |
| ep-release-gate | 不随产品交付 | 门禁判定与证据包组装，位于 tools/release-gate/，不在 crates/ 下，不进入发布制品与 SBOM；自校验项 RG-TOOLS-EXCLUDED 断言 SBOM 中不含 ep-bench 与 ep-release-gate 两个包名 |

改动 crate。

| crate | 归属进程 | 改动 |
|---|---|---|
| ep-platform-obs | core-server、job-worker、ops-agent | 扩展阶段 2 已交付的 DegradationLedger 与 degradation_windows 的 kind 取值；新增运维中心台账模型：RPO 依据判定、容量与配额事件、部署记录；新增 crates/platform/obs/src/disposal.rs 的 OpsDisposalService，实现阶段 3b 在 crates/platform/file/src/port/disposal.rs 定义的 DisposalPort；新增 crates/platform/obs/src/capability.rs 中本阶段各用例的能力域码与动作类别常量；新增本阶段指标注册项 |
| ep-platform-file | core-server | 新增附件写出范围查询端口，向 archive-writer 提供对象范围与元数据提交状态；不改动上传流水线状态机 |
| ep-platform-audit | core-server | 新增审计证据存储的写出范围查询端口，供 archive-writer 取段根与签名对象；审计链与分段签名本身不改 |
| ep-platform-kms 侧 ep-adapter-kms | archive-writer、backup-writer | 新增实例级部署备份加密密钥的解封与信封操作 |
| ep-adapter-ipc | 全部 | 新增本阶段七种报文类型 |
| ep-platform-recon | job-worker | 本体、三张表、ReconCheck 与 ReconExecutor 由阶段 9a 交付；本阶段只新增恢复验收模式的调用入口与留证字段，调用形态为 ReconExecutor::run，run_kind 取 RECOVERY_ACCEPTANCE；本阶段不实现也不注册任何 ReconCheck，注册方清单见裁定 A-06 |
| apps/archive-writer、apps/backup-writer、apps/ops-agent | 同名 | 由骨架变为完整实现 |
| apps/core-server | core-server | 新增运维中心用例、上报受理器、交叉核对器的装配 |

依赖方向核对。ep-adapter-sink 与 ep-adapter-replication 只依赖 ep-foundation 与 ep-contract-*，不依赖 application，不互相依赖，共用的重试与退避逻辑下沉 ep-foundation。ep-platform-obs 不依赖任何 domain 与 application。archive-writer 与 backup-writer 两个 apps 不依赖任何 ep-app-*，其与 core-server 的全部交互只经 ep-adapter-ipc 的报文类型，报文类型定义在 ep-contract-portal 之外的独立位置，即放在 ep-foundation 的 ipc 模块下，理由是它跨越 platform 与 adapter 两侧且不属于任何业务模块。

前置依赖。本阶段在调整后的阶段顺序中排在最后，下列前置件在本阶段开工前均已存在，本阶段不重复交付，也不向任何后续阶段留空实现。一，ep-foundation 的 SecurityContext、SYSTEM_PRINCIPAL_ID、SYSTEM_DEVICE_ID、CapabilityDomain 与 ActionClass 由阶段 1 提供。二，platform_ops schema、platform_ops.degradation_windows 与 ep-platform-obs 的 DegradationLedger 由阶段 2 提供。三，crates/platform/file/src/port/disposal.rs 的 DisposalPort、DisposalRequest 与 DisposalReceipt 由阶段 3b 提供。四，ep-platform-recon 本体、三张表与 ReconExecutor 由阶段 9a 提供。五，ep-adapter-esign 与其 crates/adapter/esign/tests/contract_sandbox.rs 契约测试由阶段 6 提供，本阶段只执行并归档其对真实沙箱的通过记录。

---

### 3. 数据库变更

全部落在既有 platform_ops schema，不新增 schema。属主为 ep_mod_platform_ops，运行期读写由 ep_app_rw，ops-agent 只读由 ep_ops_ro。本节全部表的 created_by 与 updated_by 在系统上下文与种子迁移中一律取 ep-foundation 的 SYSTEM_PRINCIPAL_ID，即 00000000-0000-7000-8000-000000000001，不得自选其他字面量；两个写出进程经 IPC 上报产生的条目同样取该常量，理由是这两个进程不持有人类主体身份。

公共列约定。本阶段 platform_ops 表一律带 id uuid 主键（应用侧 UUIDv7）、security_level smallint not null default 20、data_scope_tags text[] not null default '{}'、created_at timestamptz not null default now()、created_by uuid not null。可更新表另带 row_version bigint not null default 1、updated_at timestamptz not null default now()、updated_by uuid not null。仅追加表不带 row_version 与 updated_*，带 reverses_id uuid null。全部表不带 legal_entity_id，不建行级策略，理由见第 0 节偏离二。时间列一律 timestamptz，日期列一律 date，金额与吞吐等数值按基线第 3.5 节取 numeric(18,6) 或 numeric(9,6)。文本列一律 text 加 CHECK 长度约束，取值上限按基线第 11.2 节。

活动行唯一性的统一写法。凡需要保证同一作用域下至多一条活动记录的表，一律用哨兵值而非部分索引：结束时间列取 timestamptz not null default 'infinity'，唯一约束建在作用域键加该列上。理由是基线第 3.10 节禁止部分索引，且该写法在同一语句内即可完成开与闭，不需要额外的指针表，也不触发基线第 3.6 节禁止的 DELETE。

#### 3.1 表清单

表 1 platform_ops.deployment_records，部署记录，仅追加加哨兵。
列：id、security_level、data_scope_tags、revision bigint not null、server_spec jsonb not null（CPU 核数、内存、磁盘型号与容量）、disk_capacity_floor_bytes bigint not null、cgroup_quota_frozen_ref text not null（认证报告编号）、rto_hours numeric(9,6) not null default 4.000000、rto_reestimated boolean not null default false、rto_reestimation_basis text null（CHECK 长度不超过 2000）、shard_pickup_sla_hours int null、dual_control_authorizers jsonb not null default '[]'、waf_frontend_configured boolean not null、waf_attestation_at timestamptz null、data_volume_within_baseline boolean not null、certification_report_ref text null、drill_report_ref text null、notes text null、superseded_at timestamptz not null default 'infinity'、created_at、created_by。
约束：pk_deployment_records；ux_deployment_records_superseded_at (superseded_at)；ux_deployment_records_revision (revision)；ck_deployment_records_rto_positive CHECK (rto_hours > 0)；ck_deployment_records_shard_sla CHECK (shard_pickup_sla_hours is null or shard_pickup_sla_hours > 0)。
索引：ix_deployment_records_created_at。
说明：shard_pickup_sla_hours 为空即该部署未约定分片取件时限，按规格第 13.4 章不得在交付材料中宣称 4 小时 RTO，该结论由 v_rpo_status 与门禁工装同时读取。

表 2 platform_ops.offsite_sinks，服务器之外落点，仅追加加哨兵。
列：id、security_level、data_scope_tags、sink_kind text not null CHECK in ('LOCAL_DIR','NFS_SMB_MOUNT','OBJECT_STORAGE')、root_ref text not null、media_type text not null CHECK in ('ONLINE','OFFLINE','NONE')、rotation_period_minutes int null、writability text not null CHECK in ('WRITABLE','UNWRITABLE','UNKNOWN')、writability_changed_at timestamptz not null、req_online boolean not null、req_auto_write boolean not null、req_failure_detectable boolean not null、access_control_attested boolean not null default false、access_control_attested_at timestamptz null、access_control_evidence_ref text null、readback_throughput_mibps numeric(18,6) null、write_throughput_mibps numeric(18,6) null、throughput_measured_at timestamptz null、superseded_at timestamptz not null default 'infinity'、公共列。
约束：ux_offsite_sinks_superseded_at；ck_offsite_sinks_offline_rotation CHECK (media_type <> 'OFFLINE' or rotation_period_minutes is not null)；ck_offsite_sinks_none_kind CHECK (media_type <> 'NONE' or (req_online = false and req_auto_write = false and req_failure_detectable = false))。
索引：ix_offsite_sinks_created_at。
说明：media_type 取 NONE 表示客户未配置任何服务器之外落点，此时该部署没有 RPO 承诺。

表 3 platform_ops.degradation_windows，降级与暴露窗口台账，可更新。本表由阶段 2 按 A-26 建立，列定义与下列各行完全一致，本阶段只做扩展，不重建表、不增删列。
列：id、security_level、data_scope_tags、row_version、kind text not null CHECK in（下列 18 个取值）、scope_key text not null（CHECK 长度不超过 200）、scope_legal_entity_id uuid null、scope_accounting_period_id uuid null、basis text not null（CHECK 长度不超过 2000）、detail jsonb not null default '{}'、opened_at timestamptz not null、closed_at timestamptz not null default 'infinity'、closing_condition text not null（CHECK 长度不超过 2000）、is_suppressible boolean not null、suppressed_until timestamptz null、created_at、created_by、updated_at、updated_by。
kind 取值：OFFSITE_SINK_NOT_CONFIGURED、OFFSITE_SINK_OFFLINE_MEDIA_RPO_DEGRADED、WAL_ARCHIVE_WRITEOUT_OVERDUE_OR_FAILED、ATTACHMENT_INCREMENTAL_WRITEOUT_OVERDUE_OR_FAILED、ATTACHMENT_BOOTSTRAP_WINDOW_EXCEEDED、ATTACHMENT_RPO_NOT_YET_ACHIEVED、AUDIT_EVIDENCE_WRITEOUT_OVERDUE_OR_FAILED、PORTAL_WAF_NOT_CONFIGURED、AUDIT_ANCHOR_OVERDUE、WRITER_ROLE_CONTAINMENT_MISSING、REPLICATION_CROSSCHECK_NO_RESULT、OFFSITE_COPY_PROTECTION_MISSING、ARCHIVE_SLOT_RETENTION_WARNING、ARCHIVE_CHAIN_BROKEN、RECON_RUN_UNFINISHED、PERIOD_CLOSE_ACCEPTANCE_REJECTED、BACKGROUND_TASK_WINDOW_MISSED、RESOURCE_QUOTA_EXPOSURE。其中 OFFSITE_SINK_NOT_CONFIGURED 与 WRITER_ROLE_CONTAINMENT_MISSING 两个取值由阶段 2 建表时给出，其余 16 个由本阶段扩展该列的 CHECK 取值。
约束：ux_degradation_windows_kind_scope_closed (kind, scope_key, closed_at) 与 ck_degradation_windows_open_order CHECK (closed_at > opened_at) 两条由阶段 2 建表时交付，前者保证同一 kind 与作用域至多一条活动条目，本阶段不改写这两条；本阶段追加 ck_degradation_windows_le_required CHECK (kind not in ('RECON_RUN_UNFINISHED','PERIOD_CLOSE_ACCEPTANCE_REJECTED') or (scope_legal_entity_id is not null and scope_accounting_period_id is not null)) 与 ck_degradation_windows_not_suppressible CHECK (kind not in ('OFFSITE_SINK_NOT_CONFIGURED','WRITER_ROLE_CONTAINMENT_MISSING') or (is_suppressible = false and suppressed_until is null)) 两条。
索引：全部索引由本阶段追加，即 ix_degradation_windows_kind_opened_at；ix_degradation_windows_closed_at_opened_at；ix_degradation_windows_scope_legal_entity_id_opened_at。
说明：归档通道暂停不单列 kind，按规格第 15.3 章含在 ARCHIVE_CHAIN_BROKEN 的同一个暴露窗口内，其 detail 内以 sub_state 取值 SUSPENDED 标注；这与规格“含落点持续不可写期间暂不重建复制槽的那一段”一致，窗口自断点起算，只在新的全量基础备份写出并通过自动校验时闭合。

表 4 platform_ops.writeout_runs，写出批次，仅追加。
列：id、security_level、data_scope_tags、channel text not null CHECK in ('WAL_ARCHIVE','ATTACHMENT_INCREMENTAL','ATTACHMENT_FULL','AUDIT_EVIDENCE','FULL_BACKUP','CONFIG_BUNDLE','ATTACHMENT_BOOTSTRAP')、writer_process text not null CHECK in ('archive-writer','backup-writer')、sink_id uuid not null、period_seq bigint not null、started_at timestamptz not null、finished_at timestamptz null、outcome text not null CHECK in ('OK','FAILED','ABORTED')、bytes_written bigint not null default 0、object_count int not null default 0、failure_category text null CHECK in ('SINK_UNWRITABLE','ENCRYPTION','CHECKSUM','SOURCE_READ','QUOTA','OTHER')、last_error text null、report_id uuid not null、reverses_id uuid null、created_at、created_by。
约束：ux_writeout_runs_report_id (report_id)，是 IPC 上报的幂等键；ux_writeout_runs_channel_period_seq (channel, period_seq)。
索引：ix_writeout_runs_channel_started_at；ix_writeout_runs_outcome_started_at。

表 5 platform_ops.attachment_watermarks，附件正文写出点水位，仅追加。
列：id、security_level、data_scope_tags、watermark_at timestamptz not null、pending_object_count int not null、oldest_pending_committed_at timestamptz null、bootstrap_state text not null CHECK in ('NOT_STARTED','RUNNING','DONE')、bootstrap_remaining_bytes bigint not null default 0、manifest_ref text not null、sink_id uuid not null、advanced_at timestamptz not null、report_id uuid not null、created_at、created_by。
约束：ux_attachment_watermarks_report_id。
索引：ix_attachment_watermarks_watermark_at；ix_attachment_watermarks_advanced_at。

表 6 platform_ops.backup_sets，备份集，可更新，走状态机。
列：id、security_level、data_scope_tags、row_version、kind text not null CHECK in ('DAILY_FULL','CHAIN_REBUILD_BASELINE','CONFIG_BUNDLE','ATTACHMENT_FULL')、state text not null CHECK in ('PLANNED','RUNNING','WRITTEN','VERIFIED','VERIFY_FAILED','ABORTED')、sink_id uuid not null、started_at timestamptz null、written_at timestamptz null、verified_at timestamptz null、bytes bigint null、base_lsn text null、backup_label_ref text null、manifest_ref text null、encryption_key_ref text not null、spill_peak_bytes bigint null、abort_reason text null CHECK in ('SPILL_LIMIT','SINK_UNWRITABLE','SOURCE_ERROR','SUPERSEDED')、公共列。
约束：ck_backup_sets_state_time CHECK (state <> 'VERIFIED' or verified_at is not null)。
索引：ix_backup_sets_kind_started_at；ix_backup_sets_state_started_at。

表 7 platform_ops.backup_runner_slot，备份串行槽，单行，可更新。
列：id uuid not null（固定常量）、current_backup_set_id uuid null、row_version、updated_at、updated_by、security_level、data_scope_tags、created_at、created_by。
约束：ck_backup_runner_slot_singleton CHECK (id = '00000000-0000-0000-0000-0000000000b1'::uuid)。
说明：每日全量备份与断链后重建基线备份的串行由该行的乐观锁保证，避免依赖单副本这一前提。

表 8 platform_ops.backup_verifications，备份自动校验结论，仅追加。
列：id、security_level、data_scope_tags、backup_set_id uuid not null、method text not null CHECK in ('MANIFEST_CHECKSUM','DECRYPT_READBACK','PG_VERIFYBACKUP','ATTACHMENT_CHECKSUM')、started_at、finished_at、outcome text not null CHECK in ('PASS','FAIL')、bytes_read bigint not null、mismatched_object_count int not null default 0、detail jsonb not null default '{}'、report_id uuid not null、created_at、created_by。
约束：ux_backup_verifications_report_id。索引：ix_backup_verifications_backup_set_id_started_at。

表 9 platform_ops.archive_channel，归档通道状态机，单行，可更新。
列：id uuid not null（固定常量）、state text not null CHECK in ('HEALTHY','RETENTION_WARNING','SLOT_INVALIDATED','REBUILDING','SUSPENDED')、slot_name text not null、slot_active boolean not null、confirmed_flush_lsn text null、broken_at timestamptz null、break_cause text null CHECK in ('SLOT_WAL_LIMIT','WRITER_STOPPED','WRITER_NOT_ADVANCING','SINK_UNWRITABLE')、rebuild_backup_set_id uuid null、restored_at timestamptz null、row_version、公共列。
约束：ck_archive_channel_singleton CHECK (id = '00000000-0000-0000-0000-0000000000a1'::uuid)；ck_archive_channel_broken CHECK (state not in ('SLOT_INVALIDATED','REBUILDING','SUSPENDED') or (broken_at is not null and break_cause is not null))。

表 10 platform_ops.archive_channel_transitions，通道状态迁移，仅追加。
列：id、security_level、data_scope_tags、from_state text not null、to_state text not null、cause text not null、occurred_at timestamptz not null、detail jsonb not null default '{}'、report_id uuid not null、created_at、created_by。
约束：ux_archive_channel_transitions_report_id。索引：ix_archive_channel_transitions_occurred_at。

表 11 platform_ops.replication_reports，写出进程的复制生命周期上报，仅追加。
列：id、security_level、data_scope_tags、writer_process text not null CHECK in ('archive-writer','backup-writer')、db_role text not null CHECK in ('ep_archiver','ep_backuper')、report_kind text not null CHECK in ('CONN_ESTABLISHED','CONN_CLOSED','SLOT_CREATED','SLOT_INVALIDATED','BASEBACKUP_STARTED','BASEBACKUP_FINISHED')、slot_name text null、backend_pid int null、occurred_at timestamptz not null、outcome text not null CHECK in ('OK','FAILED')、report_id uuid not null、spooled boolean not null default false、created_at、created_by。
约束：ux_replication_reports_report_id。索引：ix_replication_reports_occurred_at；ix_replication_reports_db_role_occurred_at。
说明：spooled 为真表示该条是 core-server 不可用期间在写出进程本地暂存后补写的，交叉核对按 occurred_at 而非写入时刻比对。

表 12 platform_ops.replication_crosscheck_runs，交叉核对，仅追加。
列：id、security_level、data_scope_tags、started_at、finished_at、outcome text not null CHECK in ('MATCHED','MISMATCHED','NO_RESULT')、db_sessions_seen int not null default 0、db_slots_seen int not null default 0、reports_matched int not null default 0、unmatched_sessions jsonb not null default '[]'、unmatched_reports jsonb not null default '[]'、created_at、created_by。
索引：ix_replication_crosscheck_runs_started_at。

表 13 platform_ops.wal_retention_samples，复制槽本机保留量采样，可按期清理。
列：id、security_level、data_scope_tags、sampled_at timestamptz not null、slot_name text not null、retained_bytes bigint not null、max_slot_wal_keep_bytes bigint not null、retention_ratio numeric(9,6) not null、pg_wal_bytes bigint not null、created_at、created_by。
索引：ix_wal_retention_samples_sampled_at。保留 90 天，超期按基线第 3.6 节允许的过期指标快照清理路径删除。

表 14 platform_ops.capacity_samples，磁盘容量水位采样，可按期清理。
列：id、security_level、data_scope_tags、sampled_at、component text not null CHECK in ('ATTACHMENT_CURRENT','ATTACHMENT_HISTORY','DB_DATA','ARCHIVE_LOCAL','BASEBACKUP_SPILL','SEARCH_AND_TEMP')、used_bytes bigint not null、floor_bytes bigint not null、ratio numeric(9,6) not null、created_at、created_by。
索引：ix_capacity_samples_sampled_at。保留 400 天，覆盖年度容量复核。

表 15 platform_ops.quota_events，配额与让路事件，可按期清理。
列：id、security_level、data_scope_tags、occurred_at、cgroup_slice text not null、event_kind text not null CHECK in ('THROTTLED','LOW_SHARE_BREACHED','BACKGROUND_TASK_DELAYED')、resource text not null CHECK in ('CPU','MEMORY','IO')、sample_window_seconds int not null、measured_share numeric(9,6) null、configured_share numeric(9,6) null、unmet_demand_evidence jsonb not null default '{}'、created_at、created_by。
索引：ix_quota_events_occurred_at；ix_quota_events_cgroup_slice_occurred_at。
说明：保底份额被击穿按规格第 13.1 章的两条件同时成立判定，即实际获得低于份额且同期存在未被满足的资源需求，后者的证据取 cpu.stat 的 throttled 计数上升、memory.events 的 low 事件或 io.stat 排队时延超阈值，逐项落在 unmet_demand_evidence 内。

表 16 platform_ops.key_recovery_materials，密钥恢复材料登记，可更新，不存材料本身。
列：id、security_level（固定 40）、data_scope_tags、row_version、material_kind text not null CHECK in ('TENANT_ROOT','LEGAL_ENTITY_KEY_DOMAIN','DEPLOYMENT_BACKUP_ENCRYPTION_KEY')、scope_ref text null、carrier text not null CHECK in ('BUILTIN_KMS','CUSTOMER_HSM')、shard_count smallint not null、shard_locations jsonb not null、dual_control_authorizers jsonb not null、last_verified_at timestamptz null、next_verification_due_on date not null、verification_method text not null、stored_with_protected_copy boolean not null default false、公共列。
约束：ck_key_recovery_materials_shards CHECK (shard_count >= 2)；ck_key_recovery_materials_not_colocated CHECK (stored_with_protected_copy = false)，落实规格第 13.4 章“不得与其保护的副本存放于同一落点”。
索引：ix_key_recovery_materials_next_verification_due_on。

表 17 platform_ops.key_recovery_verifications，核验结论，仅追加。
列：id、security_level、data_scope_tags、key_recovery_material_id uuid not null、performed_at、performed_by_party text not null CHECK in ('CUSTOMER_OPS','CUSTOMER_PER_CONTRACT')、outcome text not null CHECK in ('PASS','FAIL')、isolated_env_ref text not null、approval_ref text not null、report_ref text not null、created_at、created_by。
索引：ix_key_recovery_verifications_key_recovery_material_id_performed_at。

表 18 platform_ops.recovery_drills，恢复演练与真实恢复登记，可更新。
列：id、security_level、data_scope_tags、row_version、drill_kind text not null CHECK in ('WHOLE_MACHINE_RECOVERY','KEY_MATERIAL_ISOLATED_RECOVERY','PRODUCTION_RECOVERY')、attempt_no smallint not null、window_started_at、window_ended_at timestamptz null、sink_id uuid not null、sink_kind_at_drill text not null、readback_throughput_mibps numeric(18,6) null、rto_seconds bigint null、rpo_db_seconds bigint null、rpo_attachment_seconds bigint null、shard_pickup_seconds bigint null、attachment_check_total int null、attachment_check_failed int null、attachment_check_seconds bigint null、invariant_check_batches int null、invariant_check_max_batch_seconds bigint null、invariant_check_total_seconds bigint null、invariant_check_mem_peak_bytes bigint null、invariant_check_tempfile_peak_bytes bigint null、decrypt_seconds bigint null、outcome text null CHECK in ('PASS','FAIL')、report_ref text null、公共列。
约束：ck_recovery_drills_attempt CHECK (attempt_no >= 1)；ux_recovery_drills_kind_attempt (drill_kind, attempt_no) 仅对 drill_kind 非 PRODUCTION_RECOVERY 生效，实现方式为把 PRODUCTION_RECOVERY 的 attempt_no 取该行 id 的时间序号，避免部分索引。
索引：ix_recovery_drills_drill_kind_window_started_at。
说明：shard_pickup_seconds 单独留证且不计入 rto_seconds，按规格第 13.4 章与附录 A.6；attachment_check_seconds、decrypt_seconds 与 invariant_check_total_seconds 三项计入 rto_seconds。

表 19 platform_ops.alert_suppressions，告警抑制与静音，仅追加。
列：id、security_level、data_scope_tags、degradation_window_id uuid not null、action text not null CHECK in ('SUPPRESS','UNSUPPRESS')、acted_at timestamptz not null、acted_by uuid not null、until_at timestamptz null、reason text not null（CHECK 长度不超过 2000）、approval_ref text null、created_at、created_by。
索引：ix_alert_suppressions_degradation_window_id_acted_at。

#### 3.2 视图

- platform_ops.v_degradation_open：closed_at = 'infinity' 的全部条目，含是否被抑制、抑制到期时间与是否可抑制。
- platform_ops.v_rpo_status：输出两行，target 取 DATABASE 与 ATTACHMENT，各行含 effective_seconds、basis、basis_source_kind、evidence_ref；判定算法见第 4.6 节。
- platform_ops.v_backup_last_success：按 kind 给出最近一次 VERIFIED 的备份集及其时间。
- platform_ops.v_capacity_current：六项组件的最近一次采样与占容量下限比。
- platform_ops.v_ops_health：ops-agent 与门禁工装的单一入口，聚合上述四个视图的关键取值。

#### 3.3 权限

ep_ops_ro 授予上述五个视图的 SELECT，不授予任何基表。ep_app_rw 授予全部基表读写。ep_analyst_ro 不授予 platform_ops 任何对象，理由是运维台账不属于分析与报表取数范围。ep_archiver 与 ep_backuper 不授予 platform_ops 任何对象，两个写出进程一律经 IPC 上报，不直连。

#### 3.4 迁移编号与顺序

目录 db/migrations/platform_ops/，历史表 platform_ops.refinery_schema_history。执行顺序即文件名字典序。

1. V202611030900__platform_ops_deployment_records.sql
2. V202611030905__platform_ops_offsite_sinks.sql
3. V202611030910__platform_ops_extend_degradation_windows.sql，只做 ALTER：把 kind 的 CHECK 取值由阶段 2 的 2 个扩展至 18 个、追加 ck_degradation_windows_le_required 与 ck_degradation_windows_not_suppressible 两条 CHECK、追加三个索引；本表由阶段 2 建立，本文件不建表。
4. V202611030915__platform_ops_writeout_runs.sql
5. V202611030920__platform_ops_attachment_watermarks.sql
6. V202611030925__platform_ops_backup_sets.sql
7. V202611030930__platform_ops_backup_runner_slot.sql
8. V202611030935__platform_ops_backup_verifications.sql
9. V202611030940__platform_ops_archive_channel.sql
10. V202611030945__platform_ops_archive_channel_transitions.sql
11. V202611030950__platform_ops_replication_reports.sql
12. V202611030955__platform_ops_replication_crosscheck_runs.sql
13. V202611031000__platform_ops_wal_retention_samples.sql
14. V202611031005__platform_ops_capacity_samples.sql
15. V202611031010__platform_ops_quota_events.sql
16. V202611031015__platform_ops_key_recovery_materials.sql
17. V202611031020__platform_ops_key_recovery_verifications.sql
18. V202611031025__platform_ops_recovery_drills.sql
19. V202611031030__platform_ops_alert_suppressions.sql
20. V202611031035__platform_ops_views.sql
21. V202611031040__platform_ops_grants_ops_ro.sql
22. V202611031045__backfill_platform_ops_singletons.sql，插入 archive_channel 与 backup_runner_slot 两行常量。

每个文件头部带 -- rollback: 段。第 7 号与第 9 号两个单行表的回退为 DROP TABLE；第 3 号回退为把 kind 的 CHECK 取值收回阶段 2 的两个、删除本阶段追加的两条 CHECK 与三个索引，不删表；第 22 号回退为按常量 id 置 archive_channel.state 与 backup_runner_slot.current_backup_set_id 为初值，不删除行。除第 3 号外全部为新增表与新增索引，第 3 号为约束与索引的在线增补，均属基线第 3.9 节的在线变更范围，索引一律 CREATE INDEX CONCURRENTLY，迁移会话固定 lock_timeout 5s 与 statement_timeout 30min。

---

### 4. 领域模型与关键算法

#### 4.1 核心结构体与枚举

落在 ep-platform-obs 与两个新适配 crate。

- SinkDescriptor { kind: SinkKind, root: SinkRoot, credential_ref: SecretRef, media_type: MediaType }，SinkKind 取 LocalDir、NfsSmbMount、ObjectStorage，MediaType 取 Online、Offline、None。
- SinkWritability 取 Writable、Unwritable、Unknown。
- WriteoutChannel 取 WalArchive、AttachmentIncremental、AttachmentFull、AuditEvidence、FullBackup、ConfigBundle、AttachmentBootstrap。
- ArchiveChannelState 取 Healthy、RetentionWarning、SlotInvalidated、Rebuilding、Suspended。
- BreakCause 取 SlotWalLimit、WriterStopped、WriterNotAdvancing、SinkUnwritable。
- BackupSetState 取 Planned、Running、Written、Verified、VerifyFailed、Aborted。
- DegradationKind 为第 3.1 节表 3 的 18 个取值，其中 OFFSITE_SINK_NOT_CONFIGURED 与 WRITER_ROLE_CONTAINMENT_MISSING 两项由阶段 2 在 ep-platform-obs 中定义，本阶段扩展其余 16 项。
- RpoBasis 取 Default15Min、DegradedToMediaRotation、NoCommitment、BootstrapNotYetAchieved、ExposureWindowOpen、WriterNotInService、ArchiveChainBroken。
- AttachmentWatermark { at: DateTime<Utc>, pending: u32, oldest_pending_committed_at: Option<DateTime<Utc>>, manifest_ref: String }。
- RecoveryPoint { db_point: Lsn 与时间、attachment_point: 水位时刻、aligned: DateTime<Utc> }。
- EnvelopeHeader { magic, format_version: u16, alg: AeadAlg（固定 Aes256Gcm）, dbek_ref: KeyRef, nonce: [u8;12], aad: ObjectIdentity }。

#### 4.2 归档通道状态机

状态与守卫条件如下，全部迁移写入 archive_channel_transitions 并同事务写审计。

- Healthy 到 RetentionWarning：守卫为 retention_ratio 大于等于 0.60。动作为开 ARCHIVE_SLOT_RETENTION_WARNING 暴露窗口，只告警，按规格第 13.4 章不触发任何备份动作。
- RetentionWarning 到 Healthy：守卫为 retention_ratio 小于 0.55，取 0.05 迟滞带以免抖动。动作为闭窗口。
- Healthy 或 RetentionWarning 到 SlotInvalidated：守卫为四类成因任一成立，即数据库侧该槽被判失效（pg_replication_slots.wal_status 取 lost）、archive-writer 进程停止、archive-writer 长时间不推进确认位点（confirmed_flush_lsn 在两个写出周期内不前进）、落点持续不可写。动作为开 ARCHIVE_CHAIN_BROKEN 暴露窗口，记 broken_at 与 break_cause，同时把事务数据库 RPO 依据切到 ArchiveChainBroken。
- SlotInvalidated 到 Rebuilding：守卫为落点可写性判定为 Writable。动作按顺序为删除已失效的复制槽、重建新槽、由 backup-writer 执行一次新的全量基础备份，该次备份与每日全量串行不并发。
- SlotInvalidated 到 Suspended：守卫为落点可写性判定为 Unwritable 且持续超过暂停阈值。动作为不重建复制槽、不执行全量基础备份、保持实例可写、本机事务日志不再因该槽堆积；持续告警并在台账 detail 内标注 sub_state 为 SUSPENDED；ARCHIVE_CHAIN_BROKEN 窗口不闭合。
- Suspended 到 Rebuilding：守卫为落点恢复可写。该迁移由平台自动执行，不需人工发起。
- Rebuilding 到 Healthy：守卫为该次基线备份写出到落点并通过自动校验，即对应 backup_sets 行进入 Verified。动作为闭 ARCHIVE_CHAIN_BROKEN 窗口，restored_at 置值。仅重建复制槽不触发该迁移。
- Rebuilding 到 Suspended：守卫为重建过程中落点再次转为不可写。

边界条件三条。一，Suspended 是终态意义上的稳态，没有平台侧自愈路径，出口只有客户修复落点；平台不反复重建复制槽，也不反复执行无处写出的基础备份；界面与台账文案不得出现“正在恢复”一类表述。二，处于 SlotInvalidated、Rebuilding 与 Suspended 三态期间，v_rpo_status 的事务数据库行一律按 ArchiveChainBroken 展示，不得展示 15 分钟默认承诺。三，该状态机不因 archive-writer 重启而复位，状态持久化在数据库单行上，进程启动时先读该行再决定行为。

#### 4.3 落点可写性判定

判定不依赖人工发起，由写出组件按规格第 13.4 章三项最低要求中“写入失败可被平台检测”这一项持续执行。

算法。每 EP__SINK__PROBE_INTERVAL_SECONDS 执行一次探针：向落点写入一个固定前缀的小对象、读回、比对、覆盖为下一次的对象名（落点侧允许写出账户覆盖其自身探针前缀，不涉及备份对象）。真实写出的成功与失败同样计入判定序列。连续 EP__SINK__UNWRITABLE_AFTER_FAILURES 次失败判为 Unwritable，连续 EP__SINK__WRITABLE_AFTER_SUCCESSES 次成功判为 Writable。判定翻转即发事件并更新 offsite_sinks 的 writability。

暂停阈值。落点判为 Unwritable 起，若在 EP__ARCHIVE__SUSPEND_AFTER_MINUTES 内未恢复，则通道由 SlotInvalidated 转 Suspended。取值 30 分钟，是两个 15 分钟写出周期，理由是短于两个周期的不可写属正常抖动，不应立即宣布无恢复点。

边界条件。落点 media_type 为 None 时判定不执行，直接开 OFFSITE_SINK_NOT_CONFIGURED 窗口且该窗口不可抑制；media_type 为 Offline 时探针仍执行但结果不用于 RPO 判定，该部署的 RPO 依据固定为 DegradedToMediaRotation。

#### 4.4 附件正文写出点水位推进算法

这是规格第 13.4 章附件与元数据恢复点对齐条的实现依据，也是附录 A.6 附件一致性判据成立的前提。

定义。水位 W 是一个时刻，满足在 W 之前提交的全部附件元数据，其对应正文都已完成向服务器之外落点的写出并通过校验。

步骤。
1. archive-writer 经 IPC 向 core-server 请求写出范围，入参为上次水位 W_prev 与上次已处理的最大元数据提交序，出参为一个按提交序升序的对象流，每项含 attachment_object_id、metadata_committed_at、content_ref、content_sha256、content_size、key_domain_ref。
2. archive-writer 维护 pending 集合，键为 attachment_object_id，值为该对象的元数据提交时刻与写出状态。
3. 对每个未写出对象，按其法人密钥域内的原密文原样写出到落点，不二次施加部署级备份加密（该密文已按规格第 7.5 章加密），写出后按 content_sha256 读回校验。
4. 每完成一批，令 T_min 为 pending 中尚未完成对象的最小 metadata_committed_at。若 pending 为空，W 推进到本批已知的最大 metadata_committed_at；否则 W 推进到 T_min 的前一微秒。
5. W 只增不减。W 与 pending 计数、oldest_pending_committed_at 一并写入落点上的水位 manifest 对象，并经 IPC 上报入 attachment_watermarks。
6. manifest 对象自身以部署级备份加密写出，理由是它含元数据提交时刻与对象标识；其内容必须只凭落点即可读出，不依赖已失效的原服务器。

边界条件。
- 引导窗口内，bootstrap_state 为 RUNNING，不产生 W，v_rpo_status 的附件行按 BootstrapNotYetAchieved 展示；引导完成后 W 自引导起点开始推进。
- 单个对象连续写出失败达到重试上限时，该对象保留在 pending 中，W 因此停滞，ATTACHMENT_INCREMENTAL_WRITEOUT_OVERDUE_OR_FAILED 窗口打开。这是有意行为：宁可水位停滞并暴露，也不跳过对象使恢复点上出现元数据在而正文不在。
- 首版无附件物理删除，已写删除标记的对象其正文仍需写出与保留，不从 pending 中剔除。
- core-server 不可用期间，archive-writer 不能取得新的写出范围，但已在 pending 中的对象继续写出，W 继续在已知范围内推进；上报进本地 spool，恢复后补写。

#### 4.5 恢复点对齐与整机失效恢复

恢复点对齐。W_db 取落点上已写出并通过校验的 WAL 归档所能支撑的最后一致时刻；W_att 取落点上水位 manifest 内的 W。恢复点 R 等于两者较早的一个，把事务数据库以 recovery_target_time 等于 R 回退到该点。该规则保证任一恢复点上元数据存在则正文必然存在。

整机失效恢复步骤，恢复模式下按规格第 13.1 章使用扣除操作系统预留后的全部可分配量。
1. 分片取回与双人控制完成，恢复材料在现场可用。此步不计入 RTO，但其实际耗时单独留证。
2. backup-writer 以 EP__BACKUP__MODE 取 restore 启动，读落点索引，解封部署级备份加密密钥。
3. 读出 W_att 与最近一次 Verified 的全量基础备份及其后的 WAL 归档链，计算 W_db 与 R。
4. 解密并展开基础备份，回放 WAL 至 R。
5. 与第 4 步并行，流式写入附件正文：每个对象在写入过程中计算 sha256，写完即经 IPC 上报 (attachment_object_id, sha256, size)。该实现满足附录 A.6 允许流式计算而不要求恢复后另跑全量读取的口径。
6. core-server 以恢复档配置启动，逐条比对上报的校验和与 platform_file 元数据记录，输出逐条比对结论、未通过条目清单与该校验实际耗时。任一条不满足即本次恢复或演练不达标。
7. 恢复审计证据存储、配置、证书、模块包、低代码规则包与基础设施定义。
8. job-worker 以恢复验收模式经阶段 9a 交付的 ep-platform-recon 的 ReconExecutor::run 执行规格第 17.3 章全部强制不变量校验，run_kind 取 RECOVERY_ACCEPTANCE，覆盖面宽于每日校验与关账前校验；分批规模、单批时限与单查询内存及临时空间上限按附录 A.6 演练实测冻结的恢复模式取值。
9. 重建归档与备份通道，产出新的基线备份并通过自动校验。
10. 汇总 rto_seconds、rpo_db_seconds、rpo_attachment_seconds、decrypt_seconds、attachment_check_seconds、invariant_check_* 六组取值写入 recovery_drills。

边界条件。W_att 缺失即返回 PLATFORM.RECOVERY.WATERMARK_UNAVAILABLE，恢复只能到 W_db 且必须在恢复报告中显式记录附件缺失范围，本次演练判定不达标。落点上只有本机副本时该演练不成立，判定为未达标。未经分片恢复材料解密的演练按未验证处理，不得以明文副本或原运行环境在线密钥完成。

#### 4.6 两个 RPO 取值与依据的判定算法

输入为 offsite_sinks 当前行、archive_channel 当前行、三类写出的最近一次结果与周期、attachment_watermarks 的 bootstrap_state、degradation_windows 中的活动条目、deployment_records 当前行。输出为两行，target 分别为 DATABASE 与 ATTACHMENT。

依据的严劣序，由劣到优固定为：NoCommitment、WriterNotInService、ArchiveChainBroken、BootstrapNotYetAchieved、DegradedToMediaRotation、ExposureWindowOpen、Default15Min。取值算法为对每个 target 收集其全部成立的依据，取严劣序中最靠前的一个作为展示依据。

各依据的成立条件。NoCommitment 在 media_type 为 None 时对两个 target 同时成立。WriterNotInService 在 WRITER_ROLE_CONTAINMENT_MISSING 窗口活动时对两个 target 同时成立。ArchiveChainBroken 在 archive_channel.state 属三个断链态之一时只对 DATABASE 成立。BootstrapNotYetAchieved 在 bootstrap_state 非 DONE 时只对 ATTACHMENT 成立。DegradedToMediaRotation 在 media_type 为 Offline 时对两个 target 同时成立，effective_seconds 取 rotation_period_minutes 乘 60。ExposureWindowOpen 在该 target 对应的写出超期或失败窗口活动时成立，effective_seconds 取当前时刻减该 target 最近一次 OK 的 writeout_runs.finished_at。Default15Min 在其余情形成立，effective_seconds 取 900。

对外披露取值按规格第 13.3 章取两者较大值。台账必须同时展示两行且各自标注依据，不得只展示较优的一个，也不得对任一方在降级或未达成状态下展示默认承诺值。台账取值与部署记录取值不一致时按较差一方展示。

#### 4.7 部署级备份加密

对象格式为 header 加密文加认证标签。header 明文可读，含 magic、format_version、alg 固定 AES-256-GCM、dbek_ref 含版本号、nonce 12 字节、aad 为对象身份三元组（channel、period_seq 或 backup_set_id、对象相对路径）。DEK 为每对象随机 32 字节，由 DBEK 以 AES-256-GCM 包裹后放入 header 的 wrapped_dek 字段，即信封加密。DBEK 为实例级，由部署方统一持有，载体只有内置 KMS 与客户自有硬件密码机两种，不属于任一法人密钥域。

施加范围。事务日志归档、每日全量备份、审计证据存储副本、配置与证书与模块包与低代码规则包与基础设施定义副本、附件水位 manifest，一律施加。附件正文保持其法人密钥域内的原密文原样写出，不重复施加。两类合起来使落点上不存在任何明文物理副本。

必须明写的结论。该加密只阻断落点侧的外部可读，不恢复副本上的法人隔离。同时持有 DBEK 与落点读取权限者可读到除行内敏感字段外的全部法人业务数据。该结论按规格第 21.21 章写入交付说明与客户合同，界面与文档不得使用受控读取、法人隔离、等效或已满足一类措辞。

#### 4.8 复制槽保留量判定与交叉核对

保留量判定。ops-agent 与 core-server 分别按 EP__OPS__WAL_RETENTION_SAMPLE_PERIOD_SECONDS 采样 pg_replication_slots 的 safe_wal_size 与 wal_status，以及 pg_wal 目录占用，算出 retention_ratio 等于 retained_bytes 除以 max_slot_wal_keep_bytes，写入 wal_retention_samples。达到 0.60 触发第 4.2 节的 RetentionWarning 迁移。wal_status 取 lost 即判槽失效。规格明确该告警只提示保留量正在堆积，不触发任何备份动作，实现中不得挂接任何自动备份。

交叉核对。core-server 以只读分析池的一条单列固定连接、专用语句超时 EP__OPS__CROSSCHECK_STATEMENT_TIMEOUT_MS，按不长于 EP__OPS__CROSSCHECK_PERIOD_SECONDS 的固定周期读取 pg_stat_replication 与 pg_replication_slots 两张统计视图，与 replication_reports 逐条比对。比对键为 db_role 加 backend_pid 加时间区间。出现无对应上报的复制会话或复制槽、或上报中出现无对应会话的记录时，结论记 MISMATCHED，按第 15.3 章告警并按第 12.5 章记入审计。连续两个周期未产生比对结论时开 REPLICATION_CROSSCHECK_NO_RESULT 窗口，两个写出进程照常运行，连续归档与每日全量备份不停止。该核对不适用只读分析池面向交互式查询的语句超时与单查询资源上限，也不属于第 13.1 章让路顺序中被让路的后台任务。已知局限必须在文档中写明：起止落在同一采样周期内的连接可能漏检，该核对只能尽力检出。

#### 4.9 备份集状态机与暂存缓冲

迁移。Planned 到 Running 守卫为取得 backup_runner_slot 的乐观锁；Running 到 Written 守卫为流式写出完成且落点返回成功；Written 到 Verified 守卫为自动校验四项方法全部 PASS；Written 到 VerifyFailed 守卫为任一方法 FAIL，动作为该备份不计入有效备份并告警；Running 到 Aborted 守卫为本机暂存缓冲占用达到 EP__BACKUP__SPILL_MAX_BYTES 或落点转不可写，动作为中止该次备份并告警，且不得挤占连续归档本机保留子项。

暂存缓冲。按规格附录 A.3，本机不为全量基础备份预留可容纳整份的空间，backup-writer 以流式方式写出，本机只承载写出期间的暂存缓冲，缓冲占用达到子项取值时中止并告警。因此本机不承诺保留任何可直接读回的全量备份副本，整机失效恢复一律从落点副本进行。

备份自动校验。四种方法为 MANIFEST_CHECKSUM（对落点上每个对象逐个比对清单校验和）、DECRYPT_READBACK（读回并以恢复材料解密抽验固定比例的数据块，比例取 100% 于认证演练、取配置值于生产）、PG_VERIFYBACKUP（对基础备份的 backup_manifest 执行标准校验）、ATTACHMENT_CHECKSUM（对附件全量写出结果逐对象比对）。校验不建立到生产事务数据库实例的连接，不占用连接额度与复制槽。校验结论按规格第 13.4 章写入审计。

#### 4.10 处置执行与 DisposalPort 实现

端口由阶段 3b 在 crates/platform/file/src/port/disposal.rs 定义，含 DisposalRequest、DisposalReceipt 与 DisposalPort 三项。本阶段提供其唯一实现 OpsDisposalService，位于 crates/platform/obs/src/disposal.rs，在 apps/core-server/src/wiring.rs 与 apps/job-worker/src/wiring.rs 注入。阶段 2 的密钥销毁实际执行、阶段 3b 的附件与审计证据物理删除、阶段 13 的扩展对象物理删除路径一律指向该实现，各阶段不自建第二条销毁路径。

触发面。只由 ops 专用路径与 ops 专用账号触发，不在 /api/v1/platform 前缀下对外暴露，因此不进入第 5 节端点表。

执行前置，逐项校验，任一不成立即拒绝执行并写审计。一，DisposalRequest.approval_ref 对应的审批链已通过。二，DisposalRequest.second_approver_id 与申请人不同，落实双人控制，申请人不可自审。三，DisposalRequest.reauth_ref 为规格第 12.1 章要求的重新认证凭证且在有效期内。四，落点可写性判定为 Writable，否则返回 PLATFORM.OFFSITE_SINK.UNWRITABLE。

执行范围。DisposalRequest.scope 取 AttachmentObjects、KeyDomain、BackupSets、ExtTables 四者之一，object_refs 为该范围内的对象引用清单。密钥销毁走 KeyDomain，到达备份保留期的备份集销毁走 BackupSets，两者与附件正文一样必须把落点上的历史副本在同一次处置内一并覆盖，未一并覆盖的销毁证明不成立。

执行后置。同一事务内写 platform_audit.audit_events 并生成销毁证明对象，返回 DisposalReceipt，其 disposal_plan_id 回填请求取值、disposed_count 为实际处置对象数、certificate_ref 为销毁证明对象引用、executed_at 为执行完成时刻。

边界条件。处置不可逆，本阶段不提供撤销路径。处置执行不阻塞事务日志接收与附件正文写出，不改变归档通道状态机，也不闭合任何暴露窗口。

#### 4.11 本阶段注册的指标

| 指标 | 类型 | 标签 | 注册方 | 填充方 |
|---|---|---|---|---|
| ep_replication_crosscheck_age_seconds | gauge | channel，取值 archive 与 backup | 本阶段 | 本阶段 |

ep_replication_crosscheck_age_seconds 的注册与填充均归本阶段，阶段 2 只交付该核对的取数函数与只读分析池中划出的独占连接，不登记也不填充该指标。ep_degradation_windows_open 由阶段 2 注册并填充，本阶段只扩展其 kind 取值，不重复登记。ep_db_pool_connections 与 ep_db_statement_duration_seconds 由阶段 1 注册，本阶段不重复登记。docs/metrics-catalog.md 的唯一性校验由阶段 1 的 xtask 执行，本阶段的登记项须先过该校验。

---

### 5. API 契约

统一前提。全部端点前缀 /api/v1/platform，请求头按基线第 5.6 节固定集合，写请求必带 Idempotency-Key，响应按基线第 5.2 节封套。分页、排序、过滤按基线第 5.3 节。权限一律按 ABAC 判定，主体角色取运维管理员、安全管理员、审计管理员三类，对当前上下文不可见的记录按基线第 5.5 节返回 404 与 PLATFORM.AUTHZ.NOT_FOUND_OR_DENIED。本节端点不涉及规格第 12.1 章六类高风险操作，因此不要求 X-Reauth-Token，只有密钥恢复材料核验登记一项要求审批链且申请人不可自审。处置执行不在本节端点内，其双人控制与重新认证凭证要求见第 4.10 节。

能力域码与动作类别按裁定 A-20 声明。本节全部路由逐用例声明一对常量，命名为用例名的全大写下划线形式后接 _DOMAIN 与 _ACTION，类型取阶段 1 在 ep-foundation 冻结的 CapabilityDomain 与 ActionClass，本阶段不自定义能力域码，也不重新定义这两个枚举。本节路由都在 /api/v1/platform 前缀下，常量一律声明在 crates/platform/obs/src/capability.rs，能力域一律取 CapabilityDomain::PlatformAdminLowcodeOps。动作类别按只读查询取 Read、部署记录导出取 Export、其余写端点取 Write。ops-agent 的三个端点与第 4.10 节的处置执行都不在 /api/v1 命名空间内，不参与该判定，不声明常量。xtask configdoc 断言每个 /api/v1/ 路由都能解析到一对常量，缺失即构建失败。

| 方法与路径 | 请求 | 响应 data | 主要错误码 | 幂等 | 权限 |
|---|---|---|---|---|---|
| GET /degradation-windows | filter[kind]、filter[state]=open\|closed、filter[scope_legal_entity_id]、分页排序 | 台账条目数组，含 kind、scope、basis、opened_at、closed_at、closing_condition、is_suppressible、suppressed_until | 无 | 读 | 三类角色 |
| GET /degradation-windows/{id} | 无 | 单条含 detail | PLATFORM.AUTHZ.NOT_FOUND_OR_DENIED | 读 | 三类角色 |
| POST /degradation-windows/{id}/actions/suppress | { until_at, reason } | 抑制结果 | PLATFORM.DEGRADATION_WINDOW.NOT_SUPPRESSIBLE 409、PLATFORM.DEGRADATION_WINDOW.ALREADY_CLOSED 409、PLATFORM.CONCURRENCY.STALE_VERSION 409 | 键相同重放返回首次结果 | 运维管理员与安全管理员 |
| POST /degradation-windows/{id}/actions/unsuppress | { reason } | 同上 | 同上 | 同上 | 同上 |
| GET /recovery-objectives | 无 | { rpo: [ {target, effective_seconds, basis, evidence_ref} x2 ], disclosed_rpo_seconds, rto: {hours, applicable, preconditions, shard_pickup_sla_hours} } | 无 | 读 | 三类角色 |
| GET /offsite-sinks | 无 | 当前落点与其判定结论 | 无 | 读 | 三类角色 |
| POST /offsite-sinks/actions/probe | 无 | 后台任务回执 | PLATFORM.OFFSITE_SINK.NOT_CONFIGURED 409 | 是 | 运维管理员 |
| POST /offsite-sinks/actions/measure-throughput | { direction: read\|write\|both } | 后台任务回执 | PLATFORM.OFFSITE_SINK.UNWRITABLE 503 | 是 | 运维管理员 |
| POST /offsite-sinks/actions/attest-access-control | { evidence_ref, conclusion, notes } | 部署记录新版本 | PLATFORM.OFFSITE_SINK.ACCESS_CONTROL_NOT_ATTESTED 409 | 是 | 安全管理员 |
| GET /archive-channel | 无 | 当前状态、broken_at、break_cause、sub_state、rebuild_backup_set_id | 无 | 读 | 三类角色 |
| POST /archive-channel/actions/reevaluate-sink | 无 | 后台任务回执 | 无 | 是 | 运维管理员 |
| GET /backup-sets | filter[kind]、filter[state]、分页 | 备份集数组 | 无 | 读 | 三类角色 |
| GET /backup-sets/{id} | 无 | 单条含校验结论数组 | 无 | 读 | 三类角色 |
| POST /backup-sets/actions/run-full | { kind: DAILY_FULL } | 后台任务回执 | PLATFORM.BACKUP_SET.CONCURRENT_RUN 409、PLATFORM.OFFSITE_SINK.UNWRITABLE 503 | 是 | 运维管理员 |
| GET /replication-crosschecks | 分页 | 核对结论数组 | 无 | 读 | 安全管理员与审计管理员 |
| GET /capacity | 无 | 六项组件水位与容量下限对照 | 无 | 读 | 三类角色 |
| GET /quota-events | filter[event_kind]、分页 | 配额事件数组 | 无 | 读 | 运维管理员 |
| GET /key-recovery-materials | 无 | 登记项数组，不含材料本身 | 无 | 读 | 安全管理员 |
| POST /key-recovery-materials/{id}/actions/record-verification | { performed_at, performed_by_party, outcome, isolated_env_ref, approval_ref, report_ref } | 核验记录 | PLATFORM.KEY_RECOVERY_MATERIAL.VERIFICATION_OVERDUE 409（仅提示不阻断，按业务冲突返回时不阻断登记，实际取值为登记成功但同时开窗口）、PLATFORM.APPROVAL.SELF_APPROVAL_FORBIDDEN 409 | 是 | 安全管理员，需审批链 |
| GET /recovery-drills | filter[drill_kind]、分页 | 演练数组 | 无 | 读 | 三类角色 |
| POST /recovery-drills | { drill_kind, attempt_no, window_started_at, sink_id } | 演练记录 | PLATFORM.RECOVERY_DRILL.DUPLICATE_ATTEMPT 409 | 是 | 运维管理员 |
| POST /recovery-drills/{id}/actions/record-result | 六组耗时取值加 outcome 加 report_ref | 演练记录 | PLATFORM.RECOVERY.ATTACHMENT_CONTENT_MISSING 409、PLATFORM.RECOVERY.ATTACHMENT_CHECKSUM_MISMATCH 409 | 是 | 运维管理员 |
| GET /deployment-record | 无 | 部署记录当前版本 | 无 | 读 | 三类角色 |
| POST /deployment-record/actions/export | { format: json\|csv } | 后台任务回执 | 无 | 是 | 三类角色 |

ops-agent 端点，不使用封套：GET http://127.0.0.1:9101/metrics 返回 Prometheus 文本；GET http://127.0.0.1:9102/healthz 返回进程存活；GET http://127.0.0.1:9102/readyz 返回基线第 7.3 节十三个命名自检项的结论摘要与当前降级条目数，摘要按注册名标识，不用序号。

本阶段新增的全部错误码，登记入 docs/error-codes.md 与 ep-foundation 的 error::codes：
PLATFORM.DEGRADATION_WINDOW.NOT_SUPPRESSIBLE、PLATFORM.DEGRADATION_WINDOW.ALREADY_CLOSED、PLATFORM.OFFSITE_SINK.NOT_CONFIGURED、PLATFORM.OFFSITE_SINK.UNWRITABLE、PLATFORM.OFFSITE_SINK.MEDIA_TYPE_OFFLINE、PLATFORM.OFFSITE_SINK.ACCESS_CONTROL_NOT_ATTESTED、PLATFORM.ARCHIVE_CHANNEL.SLOT_INVALIDATED、PLATFORM.ARCHIVE_CHANNEL.SUSPENDED、PLATFORM.BACKUP_SET.CONCURRENT_RUN、PLATFORM.BACKUP_SET.VERIFY_FAILED、PLATFORM.BACKUP_SET.SPILL_LIMIT_EXCEEDED、PLATFORM.BACKUP_ENCRYPTION.KEY_UNAVAILABLE、PLATFORM.KEY_RECOVERY_MATERIAL.VERIFICATION_OVERDUE、PLATFORM.KEY_RECOVERY_MATERIAL.SHARD_PICKUP_SLA_MISSING、PLATFORM.RECOVERY.WATERMARK_UNAVAILABLE、PLATFORM.RECOVERY.ATTACHMENT_CONTENT_MISSING、PLATFORM.RECOVERY.ATTACHMENT_CHECKSUM_MISMATCH、PLATFORM.RECOVERY_DRILL.DUPLICATE_ATTEMPT、PLATFORM.CAPACITY.DISK_WATERMARK_EXCEEDED。分类归属：落点不可写、备份加密密钥不可用与磁盘水位属 INFRASTRUCTURE 且 retryable 为真；其余属 BUSINESS_CONFLICT 且 retryable 为假。

进程间接口报文，承载于 /run/ep/ipc/core.sock，帧格式按基线第 2 节的 4 字节大端长度前缀加 JSON 体：
1. WriteoutResultReport，对应规格第 7.7 章四类上报的写出结果。
2. VerificationConclusionReport，对应校验结论。
3. FailureEventReport，对应失败事件。
4. ReplicationLifecycleReport，对应复制连接建立与断开、复制槽建立与失效、全量基础备份起止，逐项记录角色、进程、起止时间与结果。
5. AttachmentWriteoutScopeQuery 与其应答，由 core-server 提供附件写出对象范围与元数据提交状态。
6. AttachmentChecksumVerdictReport，恢复模式下的流式校验和上报。
7. BackupSlotAcquire 与 BackupSlotRelease，串行槽申请与释放。
全部报文带 report_id（UUIDv7），core-server 侧以对应表的 ux_*_report_id 唯一约束做幂等，不复用 platform_msg.idempotency_keys，理由是后者的作用域四元组含法人与端点，与部署级上报不匹配。

---

### 6. 并发与事务边界

事务边界。上报受理器一个报文一个事务，事务内完成三件事：写对应 platform_ops 表、写 platform_audit.audit_events、写 platform_msg.outbox_events。台账开闭与状态机迁移在同一事务内完成，不拆分。隔离级别 READ COMMITTED。事务内禁止外部调用与文件正文读写，落点写出全部发生在写出进程内且在事务之外。

锁策略。archive_channel 与 backup_runner_slot 两个单行表的更新一律 SELECT ... FOR UPDATE 加乐观锁 row_version，受影响行数为 0 判版本冲突并返回 PLATFORM.CONCURRENCY.STALE_VERSION。degradation_windows 的开窗依赖阶段 2 交付的 ux_degradation_windows_kind_scope_closed 唯一约束，重复开窗触发唯一冲突后转为读取既有活动条目并返回，即开窗天然幂等。台账的开与闭一律经阶段 2 在 ep-platform-obs 交付的 DegradationLedger 的 open 与 close，本阶段扩展其 kind 取值与实现，不另建第二条写入路径。

幂等键。IPC 报文以 report_id 唯一约束幂等；HTTP 写请求以 Idempotency-Key 按基线第 5.4 节幂等；备份与写出任务以 period_seq 与 backup_set_id 幂等，重复触发返回既有结果。

与 Outbox 的关系。本阶段的十三个事件一律经 Outbox 异步投递，消费方为 job-worker 的通知投递器与派生存储写入器。事件信封的 legal_entity_id 字段对部署级事件取部署固定的系统法人标识，该标识不对应任何真实法人，只用于满足信封结构，事件不进入派生存储的法人分区。该处理写入 docs/event-catalog.md 备注。posting_date 与 accounting_period_id 两项对部署级事件为空。

失败重试与补偿。落点写出失败按指数退避在写出进程内重试，退避序列取 5 秒、15 秒、45 秒、2 分钟、5 分钟，五次内未成功即计一次周期失败并上报；周期失败即开对应暴露窗口，不进入 Outbox 死信。Outbox 投递失败按基线第 6.2 节的八次退避序列，全部失败置 DEAD。core-server 不可用期间写出进程的上报进本地 spool（/var/lib/ep/<proc>/spool/），上限 EP__ARCHIVE__SPOOL_MAX_BYTES，达到上限后丢弃最旧的非关键报文但保留全部 ReplicationLifecycleReport，理由是后者是规格第 7.7 章三项遏制手段之一的证据，不可丢；spool 不阻塞事务日志接收与附件正文写出。

必须覆盖的并发场景。一，落点转不可写与每日全量备份窗口重叠。二，复制槽失效与每日全量备份窗口重叠，验证在用复制槽数不超过 3。三，断链重建基线备份与每日全量备份的串行，验证不并发。四，同一 report_id 重复上报不少于 3 次，验证只产生一次效果。五，暴露窗口的并发开闭。六，core-server 重启期间写出进程持续写出且上报补写不丢。

---

### 7. 配置项

前缀 EP__，层级用双下划线，deny_unknown_fields。生效方式分三档：热生效指监听 /etc/ep/config.d 变更后在下一周期取用；重启生效指必须重启该进程；重判生效指改变后必须重新执行落点判定并按附录 A.6 重做一次整机失效恢复演练。

| 键 | 类型 | 默认值 | 生效 | 说明 |
|---|---|---|---|---|
| EP__SINK__KIND | 枚举 LOCAL_DIR/NFS_SMB_MOUNT/OBJECT_STORAGE | 无，必填 | 重判 | 认证的三种落点类型之外不验收 |
| EP__SINK__ROOT | 字符串 | 无，必填 | 重判 | 目录路径、挂载点或对象存储 URI |
| EP__SINK__CREDENTIAL_REF | 机密引用 | 无，必填 | 热生效 | 形如 secret://sink/writer#1 |
| EP__SINK__MEDIA_TYPE | 枚举 ONLINE/OFFLINE/NONE | 无，必填 | 重判 | 部署时判定结论，OFFLINE 即 RPO 降级 |
| EP__SINK__ROTATION_PERIOD_MINUTES | u32 | 空 | 热生效 | MEDIA_TYPE 为 OFFLINE 时必填 |
| EP__SINK__PROBE_INTERVAL_SECONDS | u32 | 60 | 热生效 | 可写性探针周期 |
| EP__SINK__UNWRITABLE_AFTER_FAILURES | u8 | 3 | 热生效 | 本阶段新增决定 |
| EP__SINK__WRITABLE_AFTER_SUCCESSES | u8 | 2 | 热生效 | 本阶段新增决定 |
| EP__SINK__READBACK_THROUGHPUT_MIN_MIBPS | u32 | 无，由认证报告冻结后填 | 重启 | 低于该值按第 13.3 章重估 RTO |
| EP__ARCHIVE__SLOT_NAME | 字符串 | ep_archive_slot | 重启 | 具名持久物理复制槽 |
| EP__ARCHIVE__MAX_SLOT_WAL_KEEP_GB | u32 | 350 | 重启 | 与附录 A.3 连续归档本机保留子项一致，启动自检核对数据库实际取值 |
| EP__ARCHIVE__RETENTION_WARN_RATIO | numeric(9,6) | 0.600000 | 热生效 | 第 13.4 章的 60% 告警阈值 |
| EP__ARCHIVE__WAL_WRITEOUT_PERIOD_SECONDS | u32 | 300 | 热生效 | 15 分钟上限的三分之一，留两倍余量 |
| EP__ARCHIVE__ATTACHMENT_INCREMENTAL_PERIOD_SECONDS | u32 | 300 | 热生效 | 同上 |
| EP__ARCHIVE__AUDIT_EVIDENCE_PERIOD_SECONDS | u32 | 300 | 热生效 | 同上，与事务日志归档一致 |
| EP__ARCHIVE__SUSPEND_AFTER_MINUTES | u32 | 30 | 热生效 | 落点不可写多久后转归档通道暂停 |
| EP__ARCHIVE__SPOOL_DIR | 路径 | /var/lib/ep/archive-writer/spool | 重启 | 上报本地暂存 |
| EP__ARCHIVE__SPOOL_MAX_BYTES | u64 | 21474836480 | 热生效 | 20 GiB |
| EP__BACKUP__MODE | 枚举 normal/restore | normal | 重启 | 恢复模式触发方式，不新增命令行参数 |
| EP__BACKUP__RESTORE_PLAN_PATH | 路径 | 空 | 重启 | restore 模式必填 |
| EP__BACKUP__FULL_SCHEDULE | cron 表达式 | 0 1 * * * | 热生效 | 每日全量备份窗口起点 |
| EP__BACKUP__ATTACHMENT_FULL_SCHEDULE | cron 表达式 | 0 3 * * * | 热生效 | 附件正文每日全量写出 |
| EP__BACKUP__SPILL_MAX_BYTES | u64 | 53687091200 | 热生效 | 50 GiB，附录 A.3 全量基础备份本机暂存子项 |
| EP__BACKUP__BOOTSTRAP_DEADLINE_HOURS | u32 | 无，必填 | 热生效 | 引导窗口时限，由实施方估算并写入部署记录 |
| EP__BACKUP__VERIFY_DECRYPT_SAMPLE_RATIO | numeric(9,6) | 0.050000 | 热生效 | 生产抽验比例，认证演练固定为 1.000000 |
| EP__BACKUP_ENCRYPTION__DBEK_REF | 机密引用 | 无，必填 | 热生效 | 部署级备份加密密钥引用，含版本 |
| EP__BACKUP_ENCRYPTION__ALGORITHM | 枚举 | AES_256_GCM | 重启 | 首版只此一值 |
| EP__OPS__METRICS_LISTEN | socket 地址 | 127.0.0.1:9101 | 重启 | |
| EP__OPS__HEALTH_LISTEN | socket 地址 | 127.0.0.1:9102 | 重启 | |
| EP__OPS__CROSSCHECK_PERIOD_SECONDS | u32 | 300 | 热生效 | 上限 300，超出即拒绝启动 |
| EP__OPS__CROSSCHECK_STATEMENT_TIMEOUT_MS | u32 | 2000 | 热生效 | 单列固定连接的专用超时 |
| EP__OPS__WAL_RETENTION_SAMPLE_PERIOD_SECONDS | u32 | 30 | 热生效 | 与附录 A.4 的 30 秒采样口径一致 |
| EP__OPS__CAPACITY_SAMPLE_PERIOD_SECONDS | u32 | 300 | 热生效 | |
| EP__OPS__DISK_WATERMARK_RATIO | numeric(9,6) | 0.800000 | 热生效 | 附录 A.3 的 80% 复核阈值 |
| EP__KEY_RECOVERY__VERIFICATION_INTERVAL_DAYS | u32 | 183 | 热生效 | 每 6 个月核验 |
| EP__KEY_RECOVERY__SHARD_PICKUP_SLA_HOURS | u32 | 无，必填 | 热生效 | 未填即不得宣称 4 小时 RTO |

启动自检的本阶段落地。基线第 7.3 节的 offsite-sink-requirements 项即服务器之外落点的三项最低要求判定，由本阶段实现，细化为八个子判定，不新增自检项，自检项一律按注册名标识、不用序号：落点在线可写、平台可自动写入、写入失败可被平台检测；介质类型判定结论存在；部署级备份加密密钥可解引用；落点访问控制核对结论已写入部署记录；密钥恢复材料的分片取件时限已约定；规格第 7.7 章两个专用角色的三项遏制手段已落实。前三项任一不满足按降级状态启动并持续告警；第八项任一不落实按规格第 7.7 章不得启用该角色，两个写出进程不投入运行，开 WRITER_ROLE_CONTAINMENT_MISSING 窗口且该窗口不可关闭。--check 模式执行基线十三个命名自检项并输出结构化报告后退出，用于部署验收与升级前置校验。

---

### 8. 测试计划

覆盖率门槛。本阶段全部 crate 属平台内核，行覆盖率不低于 85%；ep-bench 与 ep-release-gate 不进入发布制品，按其余代码 70% 计；新增与修改代码不低于 80%；工作区整体不低于 80%。工具 cargo-llvm-cov，CI 以 --fail-under-lines 强制，分档路径规则写入 codecov.toml。

#### 8.1 单元测试

- 水位推进算法：乱序提交序、单对象反复失败使水位停滞、pending 为空时的推进、引导期不产生水位、水位单调不减、对象删除标记不剔除。
- 归档通道状态机：八条迁移逐条、迟滞带不抖动、Suspended 无自愈路径、进程重启后从持久状态恢复、三个断链态下 RPO 依据一律为 ArchiveChainBroken。
- 落点可写性判定：连续失败与连续成功阈值、MEDIA_TYPE 为 NONE 与 OFFLINE 的短路分支。
- RPO 依据判定：七种依据的成立条件、严劣序取值、两个 target 各自取值、对外披露取较大值、台账与部署记录不一致时取较差、任一降级态下不得输出 900。
- 备份集状态机：六个状态与其守卫，暂存缓冲触限中止，校验四方法任一 FAIL 即 VerifyFailed。
- 信封加密：header 编解码、AAD 绑定对象身份、篡改密文与篡改 header 均校验失败、DBEK 版本切换后旧版本在轮换窗口内仍可解。
- 保留量判定：ratio 计算、0.60 触发、wal_status 取 lost 判失效。
- 台账开闭：唯一约束下的幂等开窗、不可抑制 kind 的抑制被拒、抑制记名记时。
- 恢复点对齐：R 取两者较早、W_att 缺失分支。

#### 8.2 领域属性测试

用 proptest 生成随机的元数据提交与正文写出交错序列，验证三条不变量。
1. 水位单调不减。
2. 对任一水位取值 W，在 W 之前提交的元数据集合是已完成写出对象集合的子集。
3. 对任一 (W_db, W_att)，取 R 等于较早者后，恢复点上元数据存在则正文必然存在。
第三条直接对应附录 A.6 的附件一致性判据，也是规格第 13.4 章“不得出现元数据在、正文不在”的形式化表达。

#### 8.3 集成测试

一律使用真实 PostgreSQL 16，每用例独占一个 ep_test_<nanoid> 库，用例结束即删库。落点用真实本地目录与真实 NFS 或 SMB 挂载；对象存储落点用本机 S3 兼容打桩，另提供一套契约测试跑客户对象存储沙箱。

场景清单。
1. 复制槽建立、接收 WAL、推进确认位点、正常写出到落点、周期不超过 15 分钟。
2. max_slot_wal_keep_size 触界：注入落点不可写并持续写入负载，验证保留量到 60% 告警且不触发任何备份动作，到上限时数据库回收未确认日志使槽失效、实例保持可写。这一项同时是规格第 17.2 章混沌场景中磁盘写满一类的实现证据。
3. 归档链断裂两支：落点可写支走删槽、建槽、重建基线备份、自动校验、闭窗口的完整路径，验证仅重建复制槽不闭窗口；落点不可写支进入暂停态，验证不重建槽、不执行备份、实例可写、窗口不闭合、落点恢复后自动转入重建支。
4. 三类成因逐条：archive-writer 停止、archive-writer 长时间不推进确认位点、落点长时间不可写，验证均按断链处置而非只按进程重启处理。
5. 附件增量写出与水位推进，含大文件与接近 5 GB 单文件上限的对象。
6. 审计证据写出周期不超过 15 分钟，写出对象与段根签名一致。
7. 每日全量备份流式写出、暂存缓冲峰值、四种自动校验方法。
8. 断链重建基线备份与每日全量备份的串行，验证在用复制连接不超过 2、在用复制槽不超过 3。
9. 交叉核对：制造无对应上报的复制会话与无对应会话的上报各一次，验证 MISMATCHED 与审计写入；停掉核对连接两个周期，验证 NO_RESULT 窗口打开且两个写出进程照常运行。
10. 部署级备份加密：落点上全部写出对象为密文，无恢复材料时无法读出任何业务数据，含未被字段级加密的明文业务表内容；以写出组件系统账户之外的身份读取落点被拒绝并告警。该项直接对应规格第 17.2 章数据保护控制与销毁证明测试的落点判据与第 22 章第 8 条。
11. 两个专用角色的越权测试：无法读取任何业务表、无法执行任何 DDL、无法从服务器之外建立连接、无法经界面与 API 借用。该项属发布门禁与第 7.3 章数据库认证套件必测项，并入 tests/rls_matrix 目标执行，断言经阶段 2 按 C-05 提供的 assert_replication_role_containment，本阶段不重复实现同名断言函数。
12. 三项遏制手段任一缺失时角色不得启用、两个写出进程不投入运行、窗口不可关闭。
13. 时间点恢复：把库恢复到指定 R，验证数据一致。
14. core-server 不可用期间写出继续、上报进 spool、恢复后补写不重不漏。
15. 混沌与故障注入六类：依赖服务超时、连接池与内存资源耗尽、消息积压、系统时钟漂移、磁盘写满、进程崩溃后重启恢复；预期行为为核心交易按第 15.1 章返回可重试或明确失败、不产生数据不一致、故障移除后 5 分钟内自愈；进程崩溃场景另验证重启后未完成任务自动恢复、已确认事务零丢失。
16. 台账十八类 kind 的开闭各一条，其中 RECON_RUN_UNFINISHED 与 PERIOD_CLOSE_ACCEPTANCE_REJECTED 两类由 ep-platform-recon 与 ledger 侧触发，本阶段只验证受理与展示。

#### 8.4 端到端与演练

- 附录 A.6 整机失效恢复，至少两次，两次均达标。判定项逐条：RTO 不超过 4 小时（含解密耗时、附件逐条一致性校验耗时与第 17.3 章全部强制不变量校验耗时）、RPO 不超过 15 分钟且对事务数据库与附件正文同时成立、恢复后通过第 17.3 章全部强制不变量校验、每条附件元数据都能找到对应正文且正文校验和与元数据记录一致。落点固定为在线可写类型，取另一台机器上的目录或客户对象存储二者之一，落点类型与实测持续读回吞吐记入认证报告。抽样校验不成立，必须覆盖全部附件对象。
- 附录 A.6 密钥恢复材料隔离恢复，至少两次，两次均达标。在无原运行环境密钥的隔离环境中只装载备份的分片恢复材料，完成一次解密与恢复，覆盖客户自带密钥场景，恢复数据通过第 17.3 章强制不变量校验。
- 两类演练的分片取回与双人控制耗时单独留证并注明未计入 RTO 判定。
- 两类演练各须记录第 17.3 章不变量校验的分批规模、各批耗时分布、单批最大耗时、实测总耗时、单查询内存与临时空间占用峰值，两次中取较不利的一次作为恢复模式取值的冻结依据。

#### 8.5 性能与容量认证

按附录 A.1 至 A.4 在 BC-1 基线组合上执行一次完整基线测试。数据集由 ep-datagen 按附录 A.3 产出并版本化冻结。

必判项，任一不成立不得出具认证结论。
1. 三类写出周期均不超过 15 分钟。
2. 该次每日全量备份在业务负载稳定段内完整完成。
3. 附件正文每日全量写出按 800 GB 全量计完整完成，不得抽样，不得复用增量字节数。
4. 备份自动校验对该次实际产出的全量备份与附件全量写出结果完整完成。
5. 每日内部对账覆盖 2 个法人与 36 个会计期间完整完成，且其实测总耗时折算后落在一个自然日执行窗口内。
6. 附录 A.2 的全部时延通过线成立，且在备份窗口内的样本子集上同样成立，该子集每场景不少于 40 个样本。
7. 常驻连接峰值不超过 42、并发连接峰值不超过 52。
8. 每场景样本不少于 200 次，单次运行错误率不超过 0.1%。

必记项。三项写出的周期分布与字节量对比、按稳定段折算的事务日志生成速率、附件新增字节数、备份起止时刻与传输字节数、两个备份进程实际获得的磁盘 IO 份额、备份窗口内外各自的 P95 与 P99 与最大值及其差异超过 30% 时的原因、对账起止与分批耗时分布与资源峰值、期间关账窗口的起止与结束方式与受理前提逐项判定结果与顺延入账凭证张数、复制连接与复制槽在备份窗口内外的峰值、pg_wal 实测峰值占用、磁盘五项实测占用与合计值、cgroup 配额取值。

期间关账为必测必记项而非达标项，不设通过线，但未按附录 A.4 实测即不得冻结 A.1 该项取值，也不得出具认证结论。

#### 8.6 安全与供应链测试

- SAST 取 clippy 全 lint 加 -D warnings、cargo-audit、cargo-deny、semgrep 规则集；DAST 对 core-server 与 portal-gateway 的 HTTP 面执行；模糊测试用 cargo-fuzz 覆盖信封解码、IPC 帧解码、manifest 解析三个解析面。
- 依赖、容器与密钥扫描进 CI，密钥扫描覆盖全仓库历史。
- 第三方渗透测试结论为严重与高危发现全部关闭，分级按第 17.4 章 CVSS v3.1 口径。
- 安装包、容器、模块与插件全部签名，签名算法取 ECDSA P-256，私钥保存在硬件密码机或内置 KMS；提供离线验签工具供客户侧验签。
- SBOM 取 CycloneDX 格式随每次构建产出；构建来源证明随离线包一并交付；可复现构建以固定 rust-toolchain.toml、SOURCE_DATE_EPOCH、--remap-path-prefix 与离线 vendor 目录实现，CI 中做两次独立构建比对产物哈希。

#### 8.7 发布门禁项清单

ep-release-gate 逐项判定，判定结论进入发布证据包，任一为否即不得发布。

| 门禁项 | 判据 | 判据提供方 |
|---|---|---|
| RG-CI-PROBE-ABSENT | 发布制品的 cargo tree -e features 输出中不含 ci-probe，且镜像内不含符号 api_v1_system_echo | 阶段 1 的 ci-probe feature 门控 |
| RG-TOOLS-EXCLUDED | SBOM 中不含 ep-bench 与 ep-release-gate 两个包名 | 本阶段 |
| RG-RLS-MATRIX-GREEN | tests/rls_matrix 的 32 组矩阵全部通过 | 阶段 4 |

---

### 9. 退出条件

全部为可客观判定项。

1. archive-writer、backup-writer、ops-agent 三个进程在 BC-1 基线组合上以 --check 通过基线第 7.3 节十三个命名自检项，并在生产配置下连续运行不少于 7 个自然日，期间三类写出周期无一次超过 15 分钟。
2. 第 3 节的 19 张表（其中 degradation_windows 由阶段 2 建立，本阶段只做取值、约束与索引扩展）、5 个视图与 22 个迁移文件全部落库，每个迁移带 rollback 段，refinery 历史表与二进制期望版本一致。
3. 归档通道状态机的八条迁移在集成测试中逐条通过，落点可写与不可写两支各完整走通一次，暂停态在落点恢复后自动转入重建且无需人工发起。
4. 台账十八类 kind 的开闭各至少一条实证记录；两类不可关闭告警在管理员尝试关闭时被拒并写审计；其余类的抑制与静音记名记时写入审计。
5. v_rpo_status 在七种依据下各输出一次正确取值，且在任一降级、未达成或承诺不成立状态下均未输出 900。
6. 落点上的全部写出对象为密文；无恢复材料时无法从副本读出任何业务数据，含未被字段级加密的明文业务表内容；以写出组件系统账户之外的身份读取落点被拒绝并告警；落点访问控制核对结论与实测证据已写入部署记录。
7. 两个专用角色的越权测试在 tests/rls_matrix 目标内全部通过，断言取阶段 2 提供的 assert_replication_role_containment，本阶段无同名函数的第二份实现；三项遏制手段任一缺失时两个写出进程不投入运行，且该告警不可关闭。
8. 复制交叉核对在 5 分钟周期内持续产生结论；注入的两类不匹配均被检出并写审计；连续两周期无结论时窗口打开且写出不停止。
9. 附录 A.6 整机失效恢复与密钥恢复材料隔离恢复两类演练各执行两次且两次均达标，四份演练报告齐备，其中附件元数据与正文的逐条比对结论、未通过条目清单与该校验实际耗时单独留证，恢复模式的不变量校验分批取值已冻结。
10. 附录 A.1 至 A.4 的完整基线测试执行一次，第 8.5 节八项必判项全部成立，全部必记项已记入认证报告，cgroup 配额取值与服务器规格随该报告冻结。
11. 规格第 17.2 章十七类自动化测试的本阶段相关类型全部执行：混沌与故障注入六类、备份与完整恢复、审计链与不可变存储、数据保护控制与销毁证明、安全模糊与渗透。严重与高危缺陷全部关闭，中危缺陷登记并给出规避方案与责任人。
12. 覆盖率达标：平台内核与不变量相关代码不低于 85%，新增与修改代码不低于 80%，工作区整体不低于 80%，无带 issue 编号之外的 #[ignore]。
13. 等级保护三级控制项自评矩阵完成，除第 17.5 章登记的四项永久性不符合项外全部符合，其余不符合项均已关闭并经具备资质机构预评估；CI 校验不符合项条目未超出封闭清单。
14. 供应链安全五项齐备：SBOM、构建来源证明、可复现构建两次比对一致、离线依赖仓库、客户侧验签工具。
15. ep-release-gate 对第 22 章十五条与第 17.2 章通过标准逐条产出判定结论，第 8.7 节的 RG-CI-PROBE-ABSENT、RG-TOOLS-EXCLUDED 与 RG-RLS-MATRIX-GREEN 三个门禁项全部为通过，发布证据包组装完成，含认证报告、演练报告、台账快照与暴露窗口记录、缺陷台账、渗透测试结论、等保自评结论、各业务阶段四端界面交付情况汇总矩阵、签字验收记录。
16. PRD 第 11.11 节八条诚实披露文本已进入交付说明与客户合同模板，并在产品界面可达处呈现；交付、认证与验收材料经文本检查未出现高可用、零停机、自动切换、受控读取、法人隔离、等效、已满足、优先级隔离、资源隔离、性能保证十项禁用措辞。
17. OpsDisposalService 已实现阶段 3b 定义的 DisposalPort 并在 core-server 与 job-worker 两处 wiring 注入；AttachmentObjects、KeyDomain、BackupSets、ExtTables 四类处置范围各有一次完整执行记录，销毁证明对象与审计条目齐备，落点侧历史副本在同一次处置内一并覆盖；缺审批链、缺第二审批人或缺重新认证凭证时执行被拒并写审计。
18. 电子签章的认证清单已补齐：crates/adapter/esign/tests/contract_sandbox.rs 对真实沙箱的一次通过记录已归档，或已提交规格附录 B 允许的等效验证证据。
19. ep_replication_crosscheck_age_seconds 已由本阶段注册并填充，其在 docs/metrics-catalog.md 内的条目由本阶段首次写入且唯一，阶段 2 不写该条目；本阶段未重复登记 ep_degradation_windows_open、ep_db_pool_connections 与 ep_db_statement_duration_seconds。
20. platform_ops.degradation_windows 的 kind 取值已由阶段 2 的 2 个扩展至 18 个，两条 CHECK 与三个索引已追加，阶段 2 交付的 ux_degradation_windows_kind_scope_closed 与 ck_degradation_windows_open_order 未被改写。
21. 本阶段全部 /api/v1/ 路由的能力域码与动作类别常量已按 A-20 声明在 crates/platform/obs/src/capability.rs，能力域一律取 foundation::CapabilityDomain::PlatformAdminLowcodeOps，动作类别取 foundation::ActionClass，xtask configdoc 通过。

---

### 10. 与规格和 PRD 的对应

规格条目。
- 第 7.5 章：应用级不可变四项中的每份备份自动校验、至少一份备份落在服务器之外；审计证据存储与文件使用独立路径与独立保留策略。
- 第 7.7 章：两个写出进程的连接与复制槽枚举、复制槽 max_slot_wal_keep_size、四类上报路径、交叉核对、三项角色侧遏制手段、越权测试项。
- 第 7.8 章与第 12.3 章：部署级备份加密密钥为实例级、不属任一法人密钥域、载体只有内置 KMS 与客户自有硬件密码机；密钥恢复材料的分片、双人控制与每 6 个月核验。
- 第 12.5 章：审计证据存储向落点的写出周期不超过 15 分钟并自动校验；最近一次成功锚定时间在运维中心可见且超期告警。
- 第 13.1 章：八个进程的 cgroup 配额与让路顺序的运行期表现，即配额触发限流、后台任务被让路延迟与保底份额被击穿三类事件的记录与保留；恢复模式的资源档位；文件存储正文读写按发起进程计费。
- 第 13.2 章：BC-1 部署适配、单机容器编排、OCI 容器交付。
- 第 13.3 章：RPO 与 RTO 两项目标及其全部前提、降级与不成立情形、以演练验收不以运行期统计值判定。
- 第 13.4 章：全部十九条备份要求逐条落地，含连续归档与时间点恢复、服务器之外落点、三类写出周期、复制槽保留量阈值与硬上限、归档链断裂两支处置、附件与元数据恢复点对齐、落点三项最低要求、写出组件、服务器之外副本保护、落点访问控制分层、落点回传吞吐单独度量、离线介质降级、未配置落点无承诺、附件每日全量写出、配置与证书与模块包与低代码规则包与基础设施定义单独备份、密钥恢复材料分离保管、每份备份自动校验、定期隔离恢复演练。
- 第 15.1 章与第 15.2 章：本阶段错误码的五类归属；备份与写出失败不静默忽略。
- 第 15.3 章：运维中心全部登记项、两个 RPO 取值与依据枚举、告警持续可见、两类不可关闭告警、其余类抑制与静音记名记时、台账快照与暴露窗口记录纳入交付验收。
- 第 16 章与附录 A.1 至 A.4：性能与容量认证的度量对象、统计口径、基准数据集、负载模型与全部必判必记项。
- 第 17.2 章：混沌与故障注入六类、备份与完整恢复、审计链与不可变存储、数据保护控制与销毁证明、安全模糊与渗透五类测试；发布缺陷门禁。
- 第 17.3 章：恢复后的全部强制不变量校验作为恢复验收口径的调用与留证。
- 第 17.4 章：供应链安全五项与严重高危分级口径。
- 第 17.5 章与附录 D：BC-1 单一基线组合、通过判据、认证报告、发布前置项与运行期项划分、四项永久性不符合项封闭清单。
- 附录 A.5 与 A.6：可恢复性判定方式与两类演练判定表全部判据。
- 第 19 章阶段 4 与第 22 章：退出条件与十五条验收标准的门禁判定与证据包。
- 第 21.6、21.13、21.18、21.19、21.21 章：单点故障、补丁分发与漏洞响应台账、应用级不可变不等价、让路次序不是运行期保证、物理副本可被操作系统特权者读取五项风险的登记与披露。

PRD 条目。
- 第 10.6.2 节审计链验证工具：本阶段只提供其运维中心侧的锚定超期状态呈现，工具本体属审计阶段。
- 第 10.6.3 节运维中心的降级与暴露窗口台账：三条用户可见行为要求全部落地。
- 第 11.8 节计划内停机：本阶段提供窗口开始前完成一次通过校验的备份这一前置条件的自动判定。
- 第 11.9 节降级状态的用户可见性：六行规则逐行落地。
- 第 11.10 节：门户请求因资源配额限流失败的事件记入运维中心并计入错误率口径。
- 第 11.11 节诚实披露八条：八条文本与其在界面、部署记录、交付说明三处同时可见的要求。
- 第 11.12 节验收对应关系：六条指向的判据由本阶段的门禁工装逐条判定。
- 附录乙 U-L-10 与 U-L-11：本阶段给出临时取值并标注被阻塞状态。

---

### 11. 风险与预留

风险一，落点回传吞吐决定 RTO 是否成立，而落点由客户提供并运维。控制：部署时与每次更换落点后各实测一次持续读回与持续写入吞吐，写入部署记录；低于认证报告记录值时按第 13.3 章重估 RTO 并在该落点上重做一次整机失效恢复演练，未重做按未验证处理。风险不可消除，只能度量与披露。

风险二，附件正文 800 GB 的每日全量写出与 4 小时 RTO 同时成立的余量有限。控制：认证运行必须按 800 GB 全量计不得抽样；恢复演练的附件写入与校验和计算流式合并以免二次全量读取；实测超出 4 小时时只能上调 RTO 承诺值并同步修订规格与交付材料后重新演练，不得缩减校验范围或改按抽样。

风险三，交叉核对只能尽力检出，起止落在同一采样周期内的复制连接可能漏检。控制：采样周期上限 300 秒且不可配置为更长；漏检可能性写入交付说明；真正的遏制在操作系统层访问控制与审计，不在应用层，该结论按第 21.21 章披露。

风险四，归档通道暂停态没有平台侧自愈路径，客户长期不修复落点即长期无新恢复点。控制：告警不可由管理员关闭，台账依据固定为 ArchiveChainBroken，进入该状态起即书面告知客户并写入交付说明；界面文案禁止出现正在恢复一类表述。

风险五，本机不保留可直接读回的全量备份副本，落点与恢复材料同时不可用即无恢复路径。控制：恢复材料按分片、双人控制、每 6 个月核验且不得与其保护的副本同处一个落点，由 ck_key_recovery_materials_not_colocated 在数据库层强制。

风险六，配额取值随认证报告冻结，客户不得单方调高，但客户实际负载可能长期偏离基线。控制：配额触发限流、让路延迟与保底击穿三类事件持续记录并保留；容量水位达到下限 80% 时按第 15.3 章告警并要求扩容或按处置流程物理删除，二者均未执行时把容量暴露写入部署记录并书面告知客户。

风险七，对象存储落点使写出进程具备出网能力，扩大了攻击面。控制：出向策略的目的地址集合固定为部署记录所载落点，写出侧凭据只由写出进程系统账户持有、不下发人类用户、不用于交互式登录、不复用于其他进程；该项纳入部署验收核对。

风险八，处置执行不可逆，且同一批对象分散在本机存储与服务器之外落点两侧，任一侧漏处置即销毁证明不成立。控制：OpsDisposalService 把落点侧对象纳入同一次处置范围并逐对象留证，disposed_count 与销毁证明对象引用一并写审计；双人控制与重新认证凭证在实现内强制，不设跳过开关；密钥销毁与到期备份集销毁一律走该实现，其他阶段不得自建销毁路径；落点不可写时拒绝执行而不做部分处置。

为后续版本预留的扩展点，本阶段只留接口不实现。SinkKind 枚举预留经认证的不可变后端与异地在线不可变备份库两个取值位；ArchiveChannelState 预留多副本形态下的槽迁移态；recovery_drills.drill_kind 预留单故障域失效与区域灾难两类演练；部署级备份加密的 AeadAlg 预留商用密码算法位，其切换机制随第 12.3 章延期项恢复；degradation_windows.kind 的枚举以 CHECK 约束表达而非 PostgreSQL enum，按基线第 3.2 节可在线增补取值；ep-adapter-sink 的 trait 方法只用 foundation 类型，后续更换落点后端不触及上层。
