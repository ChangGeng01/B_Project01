## 阶段 3：平台内核

本阶段建设十组平台能力：Outbox 与幂等、单据编号、审计事件哈希链分段签名、文件引用与本地文件存储、站内通知与移动推送出口、持久化工作流与死信、错误分类与重试、全文检索索引与查询、模块许可与生命周期、最小配置发布通道。前七组对应规格第 5.1 章“Outbox、幂等、编号、通知、文件引用”条目、“低代码表单、流程、规则、审批、定时器、补偿和 SLA”条目中的流程引擎运行时部分、“审计与变更留痕”条目，以及规格第 15.1 章与第 15.2 章的错误分类与可靠任务要求；后三组按归属裁定 A-05、A-07、A-19 与 A-27 前移到本阶段，理由是阶段 5 的启动自检要求模块许可先于其可用、本阶段自身的定时器扫描与 Outbox 投递要按模块开关过滤、阶段 4 的三个授权类内容项落地要求配置发布的内容项端口先于阶段 4 存在。本阶段不建设身份、授权、法人隔离、密钥、元数据与运维中心，这些由其他阶段提供，见文末依赖清单。

本阶段按裁定通则第四条的顺序链拆为 3a 与 3b 两段，3a 排在阶段 4 之前，3b 排在阶段 4 之后。3a 段只含两项，且都不依赖身份、授权与法人隔离：`platform_msg.idempotency_keys` 表与其端口实现，配置发布的内容项落地端口与注册表。其余全部内容属 3b 段。全文按此标注，凡未标注 3a 的交付物、迁移与退出条件均属 3b 段。

本阶段没有任何业务模块，因此全部验收都在合成聚合上完成，这一点与规格第 17.2 章流程引擎认证套件“使用合成聚合与合成不变量验证补偿结果，不提前引用尚未建设的财务或库存模块”的要求一致。

### 3.0 本阶段的判定前提与三条贯穿设计

在展开清单之前先固定三条贯穿本阶段全部组件的设计判定，后续各节直接引用，不再重复论证。

判定一，行级安全使全部后台扫描必须按法人逐轮进行。共享技术基线第 3.8 节规定 `app.legal_entity_id` 是行级策略的唯一判据，不设 `BYPASSRLS` 角色，跨法人访问只能逐个法人设置会话变量后分别查询。本阶段的 Outbox 取件、定时器扫描、审计段锚定、通知投递、上传会话回收、死信统计六类后台扫描全部落在 job-worker 内，因此它们一律实现为“取法人清单，按法人轮转，每法人一次独立事务”。法人清单由阶段 2 的 ep-platform-tenancy 契约提供。首版法人数为 2，轮询间隔 200 毫秒按法人平摊后每法人 100 毫秒，仍在规格第 15.2 章的可靠任务要求之内。这一形态不是实现偏好，是行级安全的必然结果，任何“一次扫全库”的实现都会被行级策略静默过滤成空集，属实现缺陷。

判定二，审计段行是本阶段唯一的全局串行化点，其持锁时长直接决定系统吞吐上限。规格第 12.5 章要求“段内链序由事务数据库的单调序列分配，该序列是唯一串行化点，核心不持有链状态”，同时要求每条事件在事务内写入前序哈希与本条哈希。前序哈希必须读取该段当前最后一条事件的哈希，而读取与写入之间若不串行化，两个并发事务会读到同一前序哈希并写出两条互不衔接的链条。因此审计追加必须在 `platform_audit.audit_segments` 的段行上取排他锁。该锁在事务提交时才释放，故审计写入必须是工作单元闭包中的最后一批写入。这一条要求把共享技术基线第 10.3 节示例中的写入顺序由“保存聚合、写审计、写 Outbox”调整为“保存聚合、写 Outbox、写审计”，见第 3.12 节偏离项。

判定三，附件的元数据可用状态严格蕴含本机正文存在。共享技术基线第 10.3 节禁止在事务内做文件正文读写，规格第 7.5 章又禁止文件存储路径开放覆盖写与原地删除接口，两条合起来意味着“先落盘后写元数据”会在崩溃时留下无法清除的孤儿文件，“先写元数据后落盘”会产生元数据在而正文不在的窗口。本阶段采用三段式：先写版本行为 `PENDING` 并预分配存储路径，再落盘，再在第二个事务内置为 `AVAILABLE`。崩溃落在任一间隙都可由 job-worker 内的幂等收敛任务按“路径上文件是否存在”收敛，且 `AVAILABLE` 一经写入即蕴含正文已在本机落盘。该任务按裁定 A-06 不称为对账：它不产生对账差异事项，不实现 `ep_platform_recon::ReconCheck`，也不依赖阶段 9a 交付的 `ep-platform-recon` 框架。该性质是规格第 13.4 章附件恢复点水位口径成立的本机侧前提。

---

### 3.1 交付物清单

本阶段结束时，下列东西存在且可运行。

平台能力，八项。

1. Outbox 写入与消费：业务状态、审计事件与 Outbox 条目在同一数据库事务写入；job-worker 按法人轮转取件、投递、退避重试、转死信；消费端由 `platform_msg.inbox_consumptions` 保证幂等；投递统计与积压指标可读。
2. 幂等键（3a 段）：`platform_msg.idempotency_keys` 表与阶段 2 定义的 `ep_adapter_db::port::IdempotencyStore` 端口实现，`try_begin(tx, scope, request_hash)` 返回 `IdempotencyOutcome::FirstCall`、`Replay { status, body }` 或 `PayloadMismatch`，`finish(tx, scope, response_status, response_body)` 写回首次结果；重复请求回放首次结果并回带 `Idempotent-Replay: true`。请求头的存在性与 UUIDv7 合法性由阶段 1 的 `IdempotencyKeyHeaderGuard` 校验并返回 `PLATFORM.IDEMPOTENCY.KEY_REQUIRED`，本阶段不重复校验、不重复登记该码。
3. 单据编号与档案编码：按共享技术基线第 11.1 节的格式生成，在业务事务内取号，回滚即退号，位数溢出自动扩展。
4. 审计事件哈希链与分段签名：按法人与自然日分段的 SHA-256 哈希链、每 5 分钟或每 1000 条的 ECDSA P-256 段根签名、签名后立即写入独立的审计证据存储路径、链验证工具与验证报告。
5. 文件引用与本地文件存储：附件对象与版本模型、分片上传与断点续传、类型识别与恶意内容检查、按法人密钥域与密级子域的信封加密落盘、只写入不覆盖不删除的存储适配、附件恢复点水位的只读输入视图。
6. 站内通知：通知实体、模板、未读计数、标记已读、按法人与接收人的列表查询；站内通知在业务事务内同步写入，不依赖任何异步链路。
7. 移动推送出口：推送设备登记、推送载荷组装与脱敏、经 integration-gateway 的出网投递、送达状态记录；推送不可用时只剩站内通知，业务提醒不中断。
8. 持久化工作流引擎：流程定义版本、实例、步骤、人工任务、定时器、SLA、补偿、运行约束、版本迁移与模拟；补偿失败进入人工任务队列并告警。同时交付 `ep_platform_flow::port::RuleEvaluator` 与 `ep_platform_flow::port::WasmComputePort` 两个端口定义，其实现类型 `AstRuleEvaluator` 与 `PluginHostWasmCompute` 按裁定 B-05 由阶段 13b 交付，见第 3.4.8 节。

横切能力，四项。

9. 错误分类与错误码表：五类分类到 HTTP 状态与响应封套的统一映射，`docs/error-codes.md` 的 `PLATFORM` 段与 `ep-foundation::error::codes` 常量表由 CI 校验一致。其中 `PLATFORM.IDEMPOTENCY.KEY_REQUIRED`、`PLATFORM.IDEMPOTENCY.PAYLOAD_MISMATCH`、`PLATFORM.CAPACITY.CONCURRENCY_LIMIT`、`PLATFORM.CONCURRENCY.STALE_VERSION`、`PLATFORM.AUTHZ.NOT_FOUND_OR_DENIED`、`PLATFORM.AUTHZ.OBJECT_FORBIDDEN`、`PLATFORM.DB.MIGRATION_WINDOW_CLOSED` 七个码按裁定 C-24 由阶段 1 登记，本阶段只引用不重复登记。
10. 重试与熔断：数据访问层的序列化失败与死锁重试、Outbox 的八段退避、外部出口的超时与熔断器骨架。
11. 死信与人工修复：死信表、记名重投、双人审批丢弃、按法人与记账日期的可枚举统计（这是规格第 10.2 章关账受理前提“该法人该会计期间的异步过账队列未清空或存在未修复死信”能被判定的依据）。
12. 事件目录与登记：`docs/event-catalog.md` 的 `platform` 段，含本阶段登记的 17 个事件类型与其信封字段约束。第 17 个为最小配置发布通道产生的 `platform.config_release.released.v1`，按裁定 A-27 由本阶段 3b 段登记，阶段 13b 扩展该通道时不重复登记。

可运行的验证物，五项。

13. 流程引擎认证套件的阶段必过项，含崩溃恢复、重复投递不少于 3 次、定时器幂等与可重放、流程定义版本升级、补偿正确性五组，跑在合成聚合与合成不变量上。
14. Outbox 可靠投递测试项，含至少一次投递、重复投递去重、崩溃恢复后不丢不重三组，这是规格第 7.3 章数据库认证套件的必含项。
15. 审计链与不可变存储测试，含链验证工具在抽样法人与日期段上全通过、段根哈希与证据存储签名一致、覆盖与删除尝试被应用层拒绝并写审计。
16. 混沌与故障注入的六类场景中本阶段可独立执行的五类：依赖服务超时、连接池与内存资源耗尽、消息积压、磁盘写满、进程崩溃后重启恢复。系统时钟漂移一类的判定依赖授时源自检，与阶段的启动自检项联测。
17. 本阶段的基准数据集扩展：`ep-datagen` 增加审计事件、Outbox 条目、通知、附件对象与流程实例五类的生成器，规模取值见第 3.8.5 节。

按归属裁定前移到本阶段的能力，四项。第 19 项属 3a 段，其余三项属 3b 段。

18. 全文检索索引与查询（3b 段）：`ep-adapter-search` 实现 `ep_foundation::port::search::SearchIndexPort` 与 `SearchQueryPort`，索引按法人分区，物理路径 `/var/lib/ep/search/<legal_entity_id>/`。`SearchDocument`、`SearchQuery`、`SearchHit` 三个类型与两个 trait 由阶段 1 在 `crates/foundation/src/port/search.rs` 建空文件、本阶段补齐。写入一律经 job-worker 消费 Outbox 事件触发，不在业务事务内调用。本阶段不交付任何业务对象的检索文档投影函数，投影由各业务阶段按 `SearchDocument` 结构提供，见第 3.4.10 节。
19. 配置发布的内容项落地端口（3a 段）：`crates/platform/release/src/port/config_item.rs`，内容为 `ConfigItemApplier` trait、`ItemKind` 枚举 15 项、`ConfigPackageItem` DTO 与 `ConfigItemApplierRegistry`，其中 `Tx` 取自 `ep_foundation::port::tx`。本项无表、无用例、不依赖身份与授权，因此排在阶段 4 之前，使阶段 4 的三个授权类 applier 不再倒挂，见第 3.4.12 节。
20. 最小配置发布通道（3b 段）：`platform_meta.config_packages`、`platform_meta.config_package_items`、`platform_meta.config_release_orders` 三张表，六态发布状态机，ECDSA P-256 签名与逐项 `item_hash` 校验，发布与回退用例，`ConfigItemApplierRegistry` 的运行期装配，以及 `FlowDefinitionApplier` 与 `NotifyRuleApplier` 两个 applier。本阶段不建 `config_release_steps` 与 `config_edit_locks`，十一态生命周期、自动测试编排与编辑锁由阶段 13b 扩展。
21. 模块许可与生命周期（3b 段）：`platform_core.module_registrations`、`platform_core.license_grants`、`platform_core.feature_flags` 三张表与 `ep-platform-license` 的 `ModuleLicenseQuery`。定时器扫描与 Outbox 投递按 `ModuleLicenseQuery::module_state` 过滤，落实规格第 5.6 章模块停用后停止定时任务与对外事件。基线第 7.3 节的自检项 `license-and-modules-consistent` 由本阶段从 Pending 换成实现。模块停用再启用的端到端验收按裁定 A-05 顺延到阶段 13b。

---

### 3.2 crate 与进程归属

#### 3.2.1 新增 crate

| crate | 层 | 新增或改动 | 装配进程 |
|---|---|---|---|
| ep-platform-outbox | platform | 新增 | core-server 写入侧，job-worker 消费侧，integration-gateway 推送消费侧 |
| ep-platform-sequence | platform | 新增 | core-server |
| ep-platform-audit | platform | 新增 | core-server 追加侧，job-worker 锚定与验证侧 |
| ep-platform-file | platform | 新增 | core-server |
| ep-platform-notify | platform | 新增 | core-server 站内写入侧，job-worker 推送编排侧 |
| ep-platform-flow | platform | 新增 | core-server 命令侧，job-worker 调度与执行侧 |
| ep-platform-license | platform | 新增（3b 段） | core-server，job-worker |
| ep-platform-release | platform | 新增（3a 段端口，3b 段通道） | core-server 发布与回退侧，job-worker 传播段预留 |
| ep-adapter-file | adapter | 新增 | core-server，job-worker 只读 |
| ep-adapter-search | adapter | 新增（3b 段） | job-worker 写入侧，core-server 查询侧 |
| ep-foundation | foundation | 改动 | 全部 |
| ep-adapter-db | adapter | 改动 | 全部持库进程 |
| ep-adapter-db-pg | adapter | 改动 | 全部持库进程 |
| ep-platform-obs | platform | 改动 | 全部 |
| ep-testkit | 测试 | 改动 | 测试 |
| ep-datagen | 测试 | 改动 | 测试 |

`ep-adapter-kms` 由阶段 2 交付，本阶段只消费其接口，不改动其公开签名。若阶段 2 尚未交付签名接口，本阶段用其接口的桩实现开发，但退出条件必须在真实实现上判定。

#### 3.2.2 各 crate 的内容边界

`ep-foundation` 新增四组类型，全部为无业务语义的通用类型，符合基线第 1.3 节对 foundation 的禁止项。

- `error::codes` 常量表的 `PLATFORM` 段，`AppError` 的 `incident_no`、`occurred_at`、`retryable`、`advice` 四个字段的构造与序列化。
- `resilience` 模块：`Backoff`（固定序列与指数两种）、`CircuitBreaker`（失败计数、开启窗口、半开探针）、`RetryPolicy`。这是本基线未覆盖事项，登记为本阶段新增决定。
- `canonical` 模块：RFC 8785 JCS 规范化序列化，供审计哈希与证据文件使用。
- `Redacted<T>` 与 `SecretString` 的日志与错误消息拦截，配合第 3.9 节的日志禁止清单。

`ep-platform-outbox`：`OutboxWriter` 与 `OutboxConsumer` 端口、信封构造与校验、幂等键仓储、死信模型与状态机、退避策略。不含任何模块的事件语义。

`ep-platform-sequence`：`NumberAllocator` 端口、编号格式化与解析、类型码注册表、位数扩展算法。

`ep-platform-audit`：`AuditRecorder` 端口、段模型与链追加算法、锚定任务、链验证器、证据文件的写入与读取端口。

`ep-platform-file`：附件对象与版本聚合、上传会话状态机、扫描端口 `ContentInspector`、存储端口 `ObjectStore`、密钥引用组装。正文的字节读写全部经 `ep-adapter-file`。处置端口 `DisposalPort` 与其两个 DTO 按裁定 A-22 定义在 `crates/platform/file/src/port/disposal.rs`，本阶段只给 trait、DTO 与空实现，实现由阶段 14 交付，见第 3.4.7 节。

`ep-platform-notify`：通知聚合、模板渲染、接收人解析端口、推送载荷组装与脱敏、送达状态。

`ep-platform-flow`：流程定义模型、实例聚合、步骤与补偿模型、定时器、人工任务、守卫条件表达式的最小求值器、调度器与执行器。

`ep-adapter-file`：两个命名空间的本机文件存储。`published` 命名空间只提供 `create_new`、`open_read`、`stat` 三个方法，不提供覆盖与删除；`staging` 命名空间额外提供 `remove`，只承载未发布的分片临时数据。审计证据存储是第三个命名空间 `evidence`，与 `published` 同为只追加但使用独立根目录与独立保留策略，对应规格第 7.5 章“与本章的文件使用独立的存储路径和独立的保留策略”。

`ep-platform-license`（3b 段）：模块注册与安装态状态机、许可凭证的解析与有效期判定、功能开关求值，对外只暴露 `ModuleLicenseQuery`。不含任何模块的业务语义。

`ep-platform-release`：3a 段只含 `port::config_item` 一个模块，即 `ConfigItemApplier` trait、`ItemKind`、`ConfigPackageItem` 与 `ConfigItemApplierRegistry`，除 `ep-foundation` 外不依赖任何 crate；3b 段追加配置包与发布单聚合、六态状态机、签名与验签、发布与回退编排。

`ep-adapter-search`（3b 段）：内置检索索引的按法人分区读写，实现 `ep_foundation::port::search` 的两个 trait。只依赖 `ep-foundation`，不依赖任何 `ep-platform-*`，索引根目录与分区路径见第 3.4.10 节。

#### 3.2.3 依赖方向核对

本阶段全部新增 crate 均为 `ep-platform-*` 与 `ep-adapter-*`，依赖只指向 `ep-foundation` 与其他 `ep-platform-*`，不依赖任何 `ep-domain-*` 与 `ep-app-*`，符合基线第 1.3 节。`ep-adapter-file` 只依赖 `ep-foundation` 与 `ep-platform-file` 中的 `ObjectStore` 端口 trait，不依赖任何其他 adapter。装配全部发生在 `apps/core-server/src/wiring.rs`、`apps/job-worker/src/wiring.rs`、`apps/integration-gateway/src/wiring.rs` 三处。

platform 内部的依赖边为：`ep-platform-outbox → ep-foundation`；`ep-platform-audit → ep-foundation`；`ep-platform-file → ep-foundation`；`ep-platform-sequence → ep-foundation`；`ep-platform-license → ep-foundation`；`ep-platform-release → ep-foundation`（3a 段），3b 段追加 `ep-platform-audit`、`ep-platform-outbox`；`ep-platform-notify → ep-foundation, ep-platform-release`；`ep-platform-flow → ep-foundation, ep-platform-outbox, ep-platform-audit, ep-platform-release`。无环，因为 `ep-platform-release` 不反向依赖 `ep-platform-flow` 与 `ep-platform-notify`，两个 applier 落在实现方 crate 内。`ep-adapter-search` 只依赖 `ep-foundation`。CI 的 `cargo metadata` 自检脚本增加本阶段八个 platform crate 与两个 adapter crate 的断言。

#### 3.2.4 进程职责增量

| 进程 | 本阶段新增职责 |
|---|---|
| core-server | 全部平台端点；业务事务内的取号、审计追加、Outbox 写入、站内通知写入；附件上传下载的正文读写；流程实例的启动、取消与人工任务命令；模块许可判定；全文检索查询；配置包与发布单的创建、审批、签名、发布执行与回退 |
| job-worker | 按法人轮转的 Outbox 取件与投递、审计段锚定与证据写出、链验证任务、流程调度与步骤执行、定时器扫描、补偿执行、推送编排、死信统计、上传会话回收、附件状态的幂等收敛、检索索引写入、保留期清理 |
| integration-gateway | 移动推送的出网投递，监听 `127.0.0.1:8082` 的内部端点 `POST /internal/v1/push/dispatch`；超时、退避、熔断与失败证据固化 |
| portal-gateway | 门户侧附件上传与通知的呈现层转发，自身不建库连接，全部取数经 core-server 的 `/api/v1/portal/...` 受控能力 API |
| ops-agent | 暴露本阶段新增指标，全部经 `ep_ops_ro` 只读角色读取运维视图 |

archive-writer 与 backup-writer 在本阶段不改动，本阶段只为其提供两个只读视图作为水位输入，见第 3.3.6 节。

---

### 3.3 数据库变更

全部迁移文件路径为 `db/migrations/<schema>/`，历史表为 `<schema>.refinery_schema_history`。执行顺序由 `db/migrations/order.toml` 的既有平台顺序决定：`platform_core`、`platform_authz`、`platform_meta`、`platform_flow`、`platform_audit`、`platform_msg`、`platform_file`、`platform_ops`。本阶段触及其中六个 schema，`platform_meta` 因裁定 A-27 的最小配置发布通道纳入。

每个迁移文件承载一张表的完整定义，含建表、CHECK 约束、索引与行级安全策略。理由是把行级安全拆到单独文件会产生一个策略尚未启用的中间态窗口，与基线第 3.8 节的默认拒绝口径冲突。基线第 3.9 节“一个文件只做一件事”按“一个对象的完整建立为一件事”解读，数据回填仍单独成文件。每个文件的头部注释含 `-- rollback:` 段。

#### 3.3.1 迁移编号与顺序

假设 3a 段迁移落在 2026 年 9 月中旬、3b 段迁移落在 2026 年 11 月初，编号如下表。若实际执行月份不同，只调整时间戳，相对顺序与分段归属不变。

| 序 | 文件名 | schema | 段 |
|---|---|---|---|
| 1 | `V202609150900__platform_msg_create_idempotency_keys.sql` | platform_msg | 3a |
| 2 | `V202611020900__platform_core_create_number_sequences.sql` | platform_core | 3b |
| 3 | `V202611020901__platform_core_create_module_registrations.sql` | platform_core | 3b |
| 4 | `V202611020902__platform_core_create_license_grants.sql` | platform_core | 3b |
| 5 | `V202611020903__platform_core_create_feature_flags.sql` | platform_core | 3b |
| 6 | `V202611020905__platform_meta_create_config_packages.sql` | platform_meta | 3b |
| 7 | `V202611020906__platform_meta_create_config_package_items.sql` | platform_meta | 3b |
| 8 | `V202611020907__platform_meta_create_config_release_orders.sql` | platform_meta | 3b |
| 9 | `V202611020910__platform_flow_create_process_definitions.sql` | platform_flow | 3b |
| 10 | `V202611020911__platform_flow_create_process_instances.sql` | platform_flow | 3b |
| 11 | `V202611020912__platform_flow_create_process_steps.sql` | platform_flow | 3b |
| 12 | `V202611020913__platform_flow_create_process_tasks.sql` | platform_flow | 3b |
| 13 | `V202611020914__platform_flow_create_process_timers.sql` | platform_flow | 3b |
| 14 | `V202611020915__platform_flow_create_process_compensations.sql` | platform_flow | 3b |
| 15 | `V202611020920__platform_audit_create_audit_segments.sql` | platform_audit | 3b |
| 16 | `V202611020921__platform_audit_create_audit_events.sql` | platform_audit | 3b |
| 17 | `V202611020922__platform_audit_create_audit_anchors.sql` | platform_audit | 3b |
| 18 | `V202611020923__platform_audit_create_audit_verifications.sql` | platform_audit | 3b |
| 19 | `V202611020931__platform_msg_create_outbox_events.sql` | platform_msg | 3b |
| 20 | `V202611020932__platform_msg_create_inbox_consumptions.sql` | platform_msg | 3b |
| 21 | `V202611020933__platform_msg_create_dead_letters.sql` | platform_msg | 3b |
| 22 | `V202611020934__platform_msg_create_notification_templates.sql` | platform_msg | 3b |
| 23 | `V202611020935__platform_msg_create_notifications.sql` | platform_msg | 3b |
| 24 | `V202611020936__platform_msg_create_notification_deliveries.sql` | platform_msg | 3b |
| 25 | `V202611020937__platform_msg_create_push_registrations.sql` | platform_msg | 3b |
| 26 | `V202611020940__platform_file_create_attachment_objects.sql` | platform_file | 3b |
| 27 | `V202611020941__platform_file_create_attachment_versions.sql` | platform_file | 3b |
| 28 | `V202611020942__platform_file_create_upload_sessions.sql` | platform_file | 3b |
| 29 | `V202611020943__platform_file_create_upload_parts.sql` | platform_file | 3b |
| 30 | `V202611020944__platform_file_create_scan_results.sql` | platform_file | 3b |
| 31 | `V202611020950__platform_file_create_watermark_views.sql` | platform_file | 3b |
| 32 | `V202611020960__platform_msg_create_ops_views.sql` | platform_msg | 3b |

第 9 至 14 号在 `platform_flow` 内的顺序保证被引用方先建，第 6 至 8 号在 `platform_meta` 内同理保证 `config_packages` 早于 `config_package_items` 与 `config_release_orders`；跨 schema 不建外键，故 `platform_flow` 早于 `platform_audit` 与 `platform_msg` 不构成引用问题。第 31 与 32 两个视图文件跨 schema 取数，按裁定通则第五条放在所涉 schema 中位次靠后的那个目录：第 31 号建 `platform_file` 与 `platform_audit` 两个 schema 的视图，放在 `db/migrations/platform_file/`；第 32 号建 `platform_msg`、`platform_flow` 与 `platform_audit` 三个 schema 的视图，放在 `db/migrations/platform_msg/`。第 1 号属 3a 段，号段排在阶段 4 的 `V202610…` 之前；第 2 至 32 号属 3b 段，号段排在其后；两个号段互不重叠，也不与阶段 2 已占用的 `V20260901…` 号段冲突。

#### 3.3.2 公共列的适用口径

基线第 4 节的公共列清单按下列口径应用到平台表，逐表在下文标注。

- 纯技术表不带 `security_level` 与 `data_scope_tags`：`idempotency_keys`、`inbox_consumptions`、`upload_parts`、`number_sequences`、`audit_segments`、`audit_anchors`、`process_timers`。理由是它们不承载可被派生存储索引或按密级过滤的内容，加两列只会制造无人维护的常量列。
- 参与派生存储与密级过滤的表带这两列：`notifications`、`attachment_objects`、`attachment_versions`、`process_instances`、`process_tasks`。
- 仅追加表不带 `row_version`、`updated_at`、`updated_by`：`audit_events`、`outbox_events`、`dead_letters`、`process_steps`、`process_compensations`、`inbox_consumptions`、`scan_results`。其中 `outbox_events` 与 `dead_letters` 的状态列是投递控制列，其信封与载荷仅追加，见第 3.12 节澄清项。
- 上述仅追加表中，只有 `process_compensations` 带 `reverses_id`，指向被补偿的 `process_steps.id`。其余仅追加表不设该列，见第 3.12 节。
- 部署级表不带 `legal_entity_id` 与 `data_scope_tags`：`platform_core.module_registrations`、`platform_core.license_grants`、`platform_core.feature_flags`、`platform_meta.config_packages`、`platform_meta.config_package_items`、`platform_meta.config_release_orders`。前三张按裁定 A-05 属全局配置字典类，三列全不带；后三张按阶段 13 计划第 3.2.10 至 3.2.12 节带 `security_level`，不带另两列。六张表都不建行级安全策略。

#### 3.3.3 索引命名的长度规则

基线第 3.10 节的索引命名在多列组合上会超过 PostgreSQL 的 63 字节标识符上限。本阶段登记一条确定性缩短规则，作为本基线未覆盖事项的新增决定：先按固定缩写表替换列段（`legal_entity_id` 缩为 `le`，`recipient_user_id` 缩为 `recipient`，`created_at` 缩为 `created`，`occurred_at` 缩为 `occurred`，`accounting_period_id` 缩为 `period`，`attachment_object_id` 缩为 `object`，`process_instance_id` 缩为 `instance`）；若仍超过 63 字节，截断到 55 字节并追加下划线与原全名 SHA-256 前 7 位十六进制。规则实现在迁移生成器中，同一输入恒定产出同一名字。

#### 3.3.4 逐表定义

以下每张表的行级安全策略一律按基线第 3.8 节的模板生成，不写变体，模板不再逐表重复。凡带 `legal_entity_id` 的表都 `ENABLE` 且 `FORCE` 行级安全，策略名为 `rls_<table>_le`。本阶段 30 张新表中，表 1 至表 24 带 `legal_entity_id` 并按模板建策略；表 25 至表 30 是按裁定 A-05 与 A-27 前移的六张部署级表，不带 `legal_entity_id`、不建策略，其可见性不随 `app.legal_entity_id` 变化。

**表 1 `platform_core.number_sequences`**

| 列 | 类型 | 约束 |
|---|---|---|
| id | uuid | not null，pk_number_sequences |
| legal_entity_id | uuid | not null |
| scope_kind | text | not null，ck_number_sequences_scope_kind 取值 `DOCUMENT`、`ARCHIVE` |
| type_code | text | not null，ck_number_sequences_type_code 长度 2 至 4 且匹配 `^[A-Z]{2,4}$` |
| period_key | text | not null，ck_number_sequences_period_key 匹配 `^[0-9]{6}$`，`ARCHIVE` 固定 `000000` |
| next_value | bigint | not null default 1，ck_number_sequences_next_value 大于 0 |
| width | smallint | not null default 6，ck_number_sequences_width 取值 6 至 12 |
| row_version | bigint | not null default 1 |
| created_at / created_by / updated_at / updated_by | | 按公共列 |

索引：`pk_number_sequences`；`ux_number_sequences_le_scope_kind_type_code_period_key`（唯一）；`ix_number_sequences_le_created`。

**表 2 `platform_flow.process_definitions`**

| 列 | 类型 | 约束 |
|---|---|---|
| id | uuid | not null，pk |
| legal_entity_id | uuid | not null |
| code | text | not null，长度不超过 64 |
| version | int | not null，大于 0 |
| definition | jsonb | not null |
| definition_hash | text | not null，64 位小写十六进制 |
| state | text | not null，ck 取值 `PUBLISHED`、`SUPERSEDED`、`RETIRED` |
| release_ref | uuid | null，指向 `platform_meta.config_release_orders`，跨 schema 逻辑引用 |
| signature_ref | text | null |
| published_at | timestamptz | null |
| is_active | boolean | not null default true |
| deactivated_at | timestamptz | null |
| row_version, created_at, created_by, updated_at, updated_by | | 按公共列 |

索引：`pk_process_definitions`；`ux_process_definitions_le_code_version`；`ix_process_definitions_le_created`。

说明：`DRAFT`、`待自动测试`、`待审批` 等 PRD 第 10.4.1 节的发布前状态不落在本表，本表只承载已签名发布的定义。发布链路由本阶段 3b 段的最小配置发布通道承担，落地入口是 `FlowDefinitionApplier`，本表只接收其发布结果，见第 3.4.12 节；PRD 第 10.4.1 节的十一态生命周期由阶段 13b 扩展。

**表 3 `platform_flow.process_instances`**

| 列 | 类型 | 约束 |
|---|---|---|
| id | uuid | not null，pk |
| legal_entity_id | uuid | not null |
| security_level | smallint | not null default 20 |
| data_scope_tags | text[] | not null default '{}' |
| definition_id | uuid | not null，同 schema 外键 fk_process_instances_process_definitions，ON DELETE RESTRICT |
| definition_code | text | not null |
| definition_version | int | not null |
| business_key | text | null，长度不超过 200，跨模块逻辑引用 |
| business_object_type | text | null |
| business_object_id | uuid | null |
| state | text | not null，ck 取值见状态机 |
| variables | jsonb | not null default '{}' |
| step_count | int | not null default 0 |
| active_branch_count | int | not null default 0 |
| next_wake_at | timestamptz | null |
| started_at | timestamptz | not null |
| deadline_at | timestamptz | not null |
| ended_at | timestamptz | null |
| end_reason | text | null |
| correlation_id | uuid | not null |
| causation_id | uuid | null |
| row_version, created_at, created_by, updated_at, updated_by | | 按公共列 |

索引：`pk_process_instances`；`ix_process_instances_le_created`；`ix_process_instances_le_state_next_wake_at`（调度取件）；`ix_process_instances_le_business_object_type_object_id`（按业务对象反查）；`ix_process_instances_le_definition_code_state`。

**表 4 `platform_flow.process_steps`**（仅追加）

| 列 | 类型 | 约束 |
|---|---|---|
| id | uuid | not null，pk |
| legal_entity_id | uuid | not null |
| process_instance_id | uuid | not null，fk_process_steps_process_instances |
| step_no | int | not null，实例内单调递增 |
| node_id | text | not null |
| node_kind | text | not null，ck 取值 `SERVICE`、`APPROVAL`、`HUMAN_TASK`、`TIMER`、`GATEWAY`、`SUBPROCESS` |
| branch_id | text | not null default `'-'` |
| idempotency_key | text | not null |
| attempt | int | not null default 1 |
| outcome | text | not null，ck 取值 `COMPLETED`、`FAILED`、`SKIPPED` |
| is_compensable | boolean | not null default false |
| compensation_node_id | text | null |
| input | jsonb | not null default '{}' |
| output | jsonb | not null default '{}' |
| error_code | text | null |
| started_at | timestamptz | not null |
| ended_at | timestamptz | not null |
| created_at | timestamptz | not null default now() |
| created_by | uuid | not null |

索引：`pk_process_steps`；`ux_process_steps_le_instance_idempotency_key`；`ix_process_steps_le_instance_step_no`；`ix_process_steps_le_created`。

分支标识空值按基线第 11.4 节的空批次标识同理取固定值 `'-'`，理由相同：它是分组键，NULL 会把同一实例的顺序分支拆成两组。

**表 5 `platform_flow.process_tasks`**

| 列 | 类型 | 约束 |
|---|---|---|
| id | uuid | not null，pk |
| legal_entity_id | uuid | not null |
| security_level | smallint | not null default 20 |
| data_scope_tags | text[] | not null default '{}' |
| process_instance_id | uuid | not null，fk |
| node_id | text | not null |
| kind | text | not null，ck 取值 `APPROVAL`、`HUMAN_TASK`、`COMPENSATION_FAILURE`、`LIMIT_EXCEEDED` |
| state | text | not null，ck 取值 `PENDING`、`CLAIMED`、`COMPLETED`、`CANCELLED` |
| assignee_user_id | uuid | null |
| candidate_role_codes | text[] | not null default '{}' |
| initiator_user_id | uuid | not null |
| title | text | not null，长度不超过 200 |
| due_at | timestamptz | null |
| sla_breached_at | timestamptz | null |
| claimed_at | timestamptz | null |
| completed_at | timestamptz | null |
| decision | text | null，ck 取值 `APPROVED`、`REJECTED`、`WITHDRAWN`、`DONE` |
| decision_reason | text | null，长度不超过 2000 |
| reauth_ref | uuid | null |
| approval_ref | uuid | null |
| row_version, created_at, created_by, updated_at, updated_by | | 按公共列 |

索引：`pk_process_tasks`；`ix_process_tasks_le_created`；`ix_process_tasks_le_assignee_state_due_at`（待办列表，对应附录 A.1 的审批任务列表加载）；`ix_process_tasks_le_instance_state`；`ix_process_tasks_le_state_due_at`（SLA 扫描）。

`initiator_user_id` 是职责分离判定的输入，`assignee_user_id` 等于它时由授权层拒绝，对应规格第 12.2 章“申请人不可自审”。判定本身在阶段 4 的 ep-platform-authz，本表只提供事实。

**表 6 `platform_flow.process_timers`**

| 列 | 类型 | 约束 |
|---|---|---|
| id | uuid | not null，pk |
| legal_entity_id | uuid | not null |
| process_instance_id | uuid | not null，fk |
| node_id | text | not null |
| kind | text | not null，ck 取值 `TIMER`、`SLA`、`RETRY` |
| state | text | not null，ck 取值 `SCHEDULED`、`FIRED`、`CONSUMED`、`CANCELLED` |
| fire_at | timestamptz | not null |
| fired_at | timestamptz | null |
| consumed_at | timestamptz | null |
| idempotency_key | text | not null |
| payload | jsonb | not null default '{}' |
| row_version, created_at, created_by, updated_at, updated_by | | 按公共列 |

索引：`pk_process_timers`；`ux_process_timers_le_instance_idempotency_key`；`ix_process_timers_le_state_fire_at`（扫描）；`ix_process_timers_le_created`。

**表 7 `platform_flow.process_compensations`**（仅追加）

| 列 | 类型 | 约束 |
|---|---|---|
| id | uuid | not null，pk |
| legal_entity_id | uuid | not null |
| process_instance_id | uuid | not null，fk |
| reverses_id | uuid | not null，指向被补偿的 `process_steps.id`，同 schema 外键 |
| compensation_node_id | text | not null |
| idempotency_key | text | not null |
| attempt | int | not null default 1 |
| outcome | text | not null，ck 取值 `COMPLETED`、`FAILED` |
| error_code | text | null |
| started_at / ended_at | timestamptz | not null |
| created_at / created_by | | 按公共列 |

索引：`pk_process_compensations`；`ux_process_compensations_le_instance_idempotency_key`；`ix_process_compensations_le_instance_created`。

**表 8 `platform_audit.audit_segments`**

| 列 | 类型 | 约束 |
|---|---|---|
| id | uuid | not null，pk |
| legal_entity_id | uuid | not null |
| event_day | date | not null，Asia/Shanghai 自然日 |
| first_seq | bigint | null |
| last_seq | bigint | null |
| last_hash | bytea | null，32 字节 |
| event_count | bigint | not null default 0 |
| state | text | not null，ck 取值 `OPEN`、`CLOSED` |
| last_anchor_seq | bigint | null |
| last_anchored_at | timestamptz | null |
| row_version, created_at, created_by, updated_at, updated_by | | 按公共列 |

索引：`pk_audit_segments`；`ux_audit_segments_le_event_day`；`ix_audit_segments_le_created`；`ix_audit_segments_le_state_last_anchored_at`（锚定扫描）。

本表的行是判定二所述的唯一串行化点。段行在该法人该自然日首次审计写入时以 `INSERT ... ON CONFLICT DO NOTHING` 建立。

**表 9 `platform_audit.audit_events`**（仅追加，列集按基线第 9.4 节固定，不增不减）

| 列 | 类型 | 约束 |
|---|---|---|
| event_id | uuid | not null，pk_audit_events |
| legal_entity_id | uuid | not null |
| event_day | date | not null |
| seq | bigserial | not null |
| prev_hash | bytea | not null，32 字节，段首条为 32 字节全零 |
| hash | bytea | not null，32 字节 |
| actor_user_id | uuid | not null |
| actor_device_id | uuid | null |
| action | text | not null |
| object_type | text | not null |
| object_id | uuid | null |
| object_version | bigint | null |
| before | jsonb | null |
| after | jsonb | null |
| reason | text | null，长度不超过 2000 |
| approval_ref | uuid | null |
| reauth_ref | uuid | null |
| client | text | not null，ck 取值 `win`、`mac`、`ios`、`android`、`portal`、`ops`、`system` |
| occurred_at | timestamptz | not null |

索引：`pk_audit_events`；`ux_audit_events_le_event_day_seq`；`ix_audit_events_le_occurred`（代替基线的 `_created_at` 基线索引，本表无 `created_at`）；`ix_audit_events_le_object_type_object_id_occurred`（按对象检索）；`ix_audit_events_le_actor_user_id_occurred`（按操作者检索）；`ix_audit_events_le_action_occurred`（按事件类型检索）。全部索引名按第 3.3.3 节的缩短规则产出。

`client` 的取值集合在基线第 5.6 节的六个之外增加 `system`，用于系统上下文写入的审计事件，如锚定任务与保留期清理。登记为本阶段新增决定。

`seq` 使用一个全局 `bigserial`。`nextval` 不随事务回滚，因此段内 `seq` 会出现空洞。链验证按段内 `seq` 升序连接判定，不要求 `seq` 连续，这一点写入验证算法与验证报告口径，见第 3.4.3 节。

**表 10 `platform_audit.audit_anchors`**

| 列 | 类型 | 约束 |
|---|---|---|
| id | uuid | not null，pk |
| legal_entity_id | uuid | not null |
| audit_segment_id | uuid | not null，fk_audit_anchors_audit_segments |
| anchor_seq | bigint | not null |
| root_hash | bytea | not null，32 字节 |
| event_count | bigint | not null |
| algorithm | text | not null，ck 取值 `ECDSA_P256_SHA256`、`RSA_PSS_SHA256` |
| key_ref | text | not null |
| signature | bytea | null |
| state | text | not null，ck 取值 `PENDING_SIGN`、`SIGNED`、`EVIDENCED`、`FAILED` |
| signed_at | timestamptz | null |
| evidence_path | text | null |
| evidence_written_at | timestamptz | null |
| last_error | text | null |
| row_version, created_at, created_by, updated_at, updated_by | | 按公共列 |

索引：`pk_audit_anchors`；`ux_audit_anchors_le_segment_anchor_seq`；`ix_audit_anchors_le_created`；`ix_audit_anchors_le_state_created`（重试扫描）。

**表 11 `platform_audit.audit_verifications`**

| 列 | 类型 | 约束 |
|---|---|---|
| id | uuid | not null，pk |
| legal_entity_id | uuid | not null |
| range_from | date | not null |
| range_to | date | not null |
| single_event_id | uuid | null |
| state | text | not null，ck 取值 `QUEUED`、`RUNNING`、`PASSED`、`FAILED`、`ABORTED` |
| segments_total | int | not null default 0 |
| segments_passed | int | not null default 0 |
| first_failure_event_id | uuid | null |
| first_failure_reason | text | null |
| report | jsonb | null |
| requested_by | uuid | not null |
| started_at / finished_at | timestamptz | null |
| row_version, created_at, created_by, updated_at, updated_by | | 按公共列 |

索引：`pk_audit_verifications`；`ix_audit_verifications_le_created`；`ix_audit_verifications_le_state_created`。

**表 12 `platform_msg.idempotency_keys`**（列集按基线第 5.4 节）

| 列 | 类型 | 约束 |
|---|---|---|
| id | uuid | not null，pk |
| key | uuid | not null |
| legal_entity_id | uuid | not null |
| user_id | uuid | not null |
| endpoint | text | not null，模板路径加方法 |
| request_hash | text | not null，64 位小写十六进制 |
| state | text | not null，ck 取值 `IN_PROGRESS`、`COMPLETED` |
| response_status | smallint | null |
| response_body | jsonb | null |
| created_at | timestamptz | not null default now() |
| expires_at | timestamptz | not null |

索引：`pk_idempotency_keys`；`ux_idempotency_keys_le_user_id_endpoint_key`；`ix_idempotency_keys_expires_at`（清理）。

`state` 一列是基线第 5.4 节列表之外的新增，理由是两个并发的同键请求必须有一个明确的“首次仍在处理中”结论，否则第二个请求要么长时间阻塞要么读到空响应体。登记为本阶段新增决定，对应错误码 `PLATFORM.IDEMPOTENCY.IN_PROGRESS`。

**表 13 `platform_msg.outbox_events`**（仅追加的信封与载荷，加投递控制列）

| 列 | 类型 | 约束 |
|---|---|---|
| event_id | uuid | not null，pk_outbox_events |
| legal_entity_id | uuid | not null |
| event_type | text | not null |
| event_version | int | not null |
| occurred_at | timestamptz | not null |
| aggregate_type | text | not null |
| aggregate_id | uuid | not null |
| aggregate_version | bigint | not null |
| security_level | smallint | not null |
| data_scope_tags | text[] | not null default '{}' |
| posting_date | date | null |
| accounting_period_id | uuid | null |
| correlation_id | uuid | not null |
| causation_id | uuid | null |
| idempotency_key | uuid | null |
| actor_user_id | uuid | not null |
| actor_device_id | uuid | null |
| actor_on_behalf_of | uuid | null |
| payload | jsonb | not null |
| status | text | not null，ck 取值 `PENDING`、`DISPATCHING`、`DONE`、`DEAD` |
| attempts | int | not null default 0 |
| available_at | timestamptz | not null default now() |
| locked_by | text | null |
| locked_until | timestamptz | null |
| last_error | text | null |
| created_at | timestamptz | not null default now() |

索引：`pk_outbox_events`；`ix_outbox_events_le_created`；`ix_outbox_events_le_status_available_at_event_id`（取件）；`ix_outbox_events_le_period_status`（关账前提枚举，列为 `legal_entity_id, accounting_period_id, status`）；`ix_outbox_events_le_status_posting_date`（按记账日期枚举待消费过账条目）。

`ix_outbox_events_le_period_status` 与 `ix_outbox_events_le_status_posting_date` 两条索引存在的唯一理由是规格第 10.2 章关账受理前提中“该法人该会计期间的异步过账队列未清空”必须可枚举且不得走顺序扫描。本阶段建索引，判定逻辑由关账所在阶段实现。

**表 14 `platform_msg.inbox_consumptions`**（仅追加）

| 列 | 类型 | 约束 |
|---|---|---|
| id | uuid | not null，pk |
| legal_entity_id | uuid | not null |
| consumer | text | not null，形如 `<process>.<handler>` |
| event_id | uuid | not null |
| consumed_at | timestamptz | not null default now() |
| created_at | timestamptz | not null default now() |
| created_by | uuid | not null |

索引：`pk_inbox_consumptions`；`ux_inbox_consumptions_consumer_event_id`（唯一，基线第 6.2 节指定）；`ix_inbox_consumptions_le_created`。

**表 15 `platform_msg.dead_letters`**（仅追加的信封与载荷，加处置控制列）

列集为 `outbox_events` 的全部信封与载荷列，加 `id uuid`、`source_event_id uuid not null`、`failure_category text not null`（取值为五类错误分类）、`last_error text not null`、`first_failed_at timestamptz not null`、`attempts int not null`、`state text not null`（ck 取值 `OPEN`、`REPAIRING`、`REPAIRED`、`DISCARDED`）、`repaired_by uuid null`、`repaired_at timestamptz null`、`approval_ref uuid null`、`discard_reason text null`、`created_at`、`created_by`。

索引：`pk_dead_letters`；`ux_dead_letters_source_event_id`；`ix_dead_letters_le_created`；`ix_dead_letters_le_state_created`；`ix_dead_letters_le_state_posting_date`（关账前提枚举）；`ix_dead_letters_le_period_state`。

`state` 与 `repaired_by` 等处置列可更新，但本表不带 `row_version`：并发处置由 `UPDATE ... WHERE id = $1 AND state = $expected` 的受影响行数判定，冲突映射为 `PLATFORM.DEAD_LETTER.STATE_INVALID`。这是对基线第 3.7 节的一处按仅追加表口径的应用，见第 3.12 节。

**表 16 `platform_msg.notification_templates`**

| 列 | 类型 | 约束 |
|---|---|---|
| id | uuid | not null，pk |
| legal_entity_id | uuid | not null |
| code | text | not null，长度不超过 64 |
| version | int | not null |
| notice_type | text | not null，ck 取值为 PRD 第 10.5.2 节的十类提醒事项码 |
| title_template | text | not null，长度不超过 200 |
| body_template | text | not null，长度不超过 2000 |
| push_title_template | text | null |
| push_body_template | text | null |
| severity | text | not null，ck 取值 `INFO`、`WARN`、`CRITICAL` |
| release_ref | uuid | null |
| is_active | boolean | not null default true |
| deactivated_at | timestamptz | null |
| row_version, created_at, created_by, updated_at, updated_by | | 按公共列 |

索引：`pk_notification_templates`；`ux_notification_templates_le_code_version`；`ix_notification_templates_le_created`。

**表 17 `platform_msg.notifications`**

| 列 | 类型 | 约束 |
|---|---|---|
| id | uuid | not null，pk |
| legal_entity_id | uuid | not null |
| security_level | smallint | not null default 20 |
| data_scope_tags | text[] | not null default '{}' |
| recipient_user_id | uuid | not null |
| notice_type | text | not null |
| severity | text | not null |
| title | text | not null，长度不超过 200 |
| body | text | not null，长度不超过 2000 |
| related_object_type | text | null |
| related_object_id | uuid | null |
| related_doc_no | text | null |
| source_kind | text | not null，ck 取值 `PROCESS_TASK`、`PROCESS_RESULT`、`TIMER`、`RECON`、`PERIOD_CLOSE`、`DEAD_LETTER`、`LICENSE`、`OPS_DEGRADATION` |
| source_ref | uuid | null |
| is_read | boolean | not null default false |
| read_at | timestamptz | null |
| dedupe_key | text | not null |
| row_version, created_at, created_by, updated_at, updated_by | | 按公共列 |

索引：`pk_notifications`；`ix_notifications_le_created`；`ix_notifications_le_recipient_is_read_created`（未读列表，对应附录 A.1 的站内通知列表加载）；`ix_notifications_le_recipient_notice_type_created`；`ux_notifications_le_recipient_dedupe_key`（唯一，扇出去重）。

`dedupe_key` 是同一提醒事项对同一接收人的去重键，取值为 `<notice_type>:<source_kind>:<source_ref>:<轮次>`。有它才能使定时器重放不产生重复通知，对应规格第 9.1 章“重复触发不得产生重复业务效果、重复事件或重复审计记录”。

`body` 与 `title` 在写入前必须已完成无权字段剔除，对应 PRD 第 10.5.5 节“通知标题与正文不得包含无权字段的内容”。剔除动作在 core-server 的通知组装处执行，模板变量的可用集合由模板声明，声明外的变量拒绝渲染。

**表 18 `platform_msg.notification_deliveries`**

| 列 | 类型 | 约束 |
|---|---|---|
| id | uuid | not null，pk |
| legal_entity_id | uuid | not null |
| notification_id | uuid | not null，fk_notification_deliveries_notifications |
| channel | text | not null，ck 取值 `IN_APP`、`MOBILE_PUSH` |
| status | text | not null，ck 取值 `PENDING`、`SENT`、`DELIVERED`、`FAILED`、`SUPPRESSED` |
| attempts | int | not null default 0 |
| push_registration_id | uuid | null |
| last_error | text | null |
| sent_at / delivered_at | timestamptz | null |
| row_version, created_at, created_by, updated_at, updated_by | | 按公共列 |

索引：`pk_notification_deliveries`；`ix_notification_deliveries_le_created`；`ix_notification_deliveries_le_notification_id_channel`；`ix_notification_deliveries_le_channel_status_created`。

`IN_APP` 的行在通知写入的同一事务内以 `status = 'DELIVERED'` 直接写入，理由见第 3.4.5 节。

**表 19 `platform_msg.push_registrations`**

| 列 | 类型 | 约束 |
|---|---|---|
| id | uuid | not null，pk |
| legal_entity_id | uuid | not null |
| user_id | uuid | not null |
| device_id | uuid | not null，逻辑引用阶段 4 的设备登记 |
| platform | text | not null，ck 取值 `ios`、`android` |
| token_ciphertext | bytea | not null，按法人密钥域字段级加密 |
| token_fingerprint | text | not null，用于唯一约束的盲索引 |
| is_active | boolean | not null default true |
| deactivated_at | timestamptz | null |
| last_success_at / last_failure_at | timestamptz | null |
| consecutive_failures | int | not null default 0 |
| row_version, created_at, created_by, updated_at, updated_by | | 按公共列 |

索引：`pk_push_registrations`；`ux_push_registrations_le_user_id_device_id_platform`；`ix_push_registrations_le_created`；`ix_push_registrations_le_user_id_is_active`。

推送令牌属于规格第 7.8 章意义上的行内敏感属性，按字段级密钥加密存储，唯一性用同一法人密钥域下的盲索引 `token_fingerprint` 承担，密文不直接进入唯一约束，这一点是规格第 7.8 章的明确要求。

**表 20 `platform_file.attachment_objects`**

| 列 | 类型 | 约束 |
|---|---|---|
| id | uuid | not null，pk |
| legal_entity_id | uuid | not null |
| security_level | smallint | not null default 20 |
| data_scope_tags | text[] | not null default '{}' |
| current_version_no | int | not null default 0 |
| display_name | text | not null，长度不超过 200 |
| purpose | text | not null |
| deleted_at | timestamptz | null |
| deleted_by | uuid | null |
| row_version, created_at, created_by, updated_at, updated_by | | 按公共列 |

索引：`pk_attachment_objects`；`ix_attachment_objects_le_created`；`ix_attachment_objects_le_deleted_at_created`。

本表是基线第 3.6 节允许带删除标记的两类对象之一。删除标记不影响历史引用与历史版本，物理删除只能由处置流程经专用路径与专用账号发起；本阶段只定义 `DisposalPort` 端口并在两个 wiring 注入 `NoopDisposalPort`，实现由阶段 14 的 `OpsDisposalService` 交付，见裁定 A-22 与第 3.4.7 节。

**表 21 `platform_file.attachment_versions`**

| 列 | 类型 | 约束 |
|---|---|---|
| id | uuid | not null，pk |
| legal_entity_id | uuid | not null |
| security_level | smallint | not null default 20 |
| data_scope_tags | text[] | not null default '{}' |
| attachment_object_id | uuid | not null，fk_attachment_versions_attachment_objects |
| version_no | int | not null，大于 0 |
| state | text | not null，ck 取值 `PENDING`、`AVAILABLE`、`QUARANTINED`、`FAILED`、`SUPERSEDED` |
| storage_path | text | not null，相对 `published` 根的路径 |
| content_hash | text | not null，明文 SHA-256 的 64 位小写十六进制 |
| size_bytes | bigint | not null，大于 0 且不超过配置上限 |
| content_type | text | not null，服务端识别结果 |
| declared_content_type | text | not null，客户端声明值 |
| key_domain_ref | text | not null，法人密钥域引用 |
| dek_ref | text | not null，该对象数据密钥的引用 |
| encryption_algorithm | text | not null，ck 取值 `AES_256_GCM` |
| available_at | timestamptz | null |
| upload_session_id | uuid | null |
| row_version, created_at, created_by, updated_at, updated_by | | 按公共列 |

索引：`pk_attachment_versions`；`ux_attachment_versions_le_object_version_no`；`ix_attachment_versions_le_created`；`ix_attachment_versions_le_state_created`（收敛与回收）；`ix_attachment_versions_le_available_at`（水位输入）。

规格第 7.5 章要求“事务数据库只保存对象 ID、版本、哈希、大小、类型、密级、密钥引用和业务关联”，本表逐项对应，正文不入库。

**表 22 `platform_file.upload_sessions`**

| 列 | 类型 | 约束 |
|---|---|---|
| id | uuid | not null，pk |
| legal_entity_id | uuid | not null |
| attachment_object_id | uuid | null，续传到已有对象时非空 |
| target_version_no | int | not null |
| state | text | not null，ck 取值 `INITIATED`、`UPLOADING`、`ASSEMBLING`、`SCANNING`、`COMMITTED`、`ABORTED`、`REJECTED`、`EXPIRED` |
| declared_size_bytes | bigint | not null |
| declared_content_type | text | not null |
| declared_content_hash | text | not null |
| part_size_bytes | int | not null |
| part_count | int | not null |
| uploaded_part_count | int | not null default 0 |
| staging_key_ref | text | not null，会话级临时密钥引用 |
| expires_at | timestamptz | not null |
| owner_user_id | uuid | not null |
| owner_device_id | uuid | null |
| row_version, created_at, created_by, updated_at, updated_by | | 按公共列 |

索引：`pk_upload_sessions`；`ix_upload_sessions_le_created`；`ix_upload_sessions_le_state_expires_at`（回收）；`ix_upload_sessions_le_owner_user_id_state`（并发上限判定）。

**表 23 `platform_file.upload_parts`**（仅追加）

| 列 | 类型 | 约束 |
|---|---|---|
| id | uuid | not null，pk |
| legal_entity_id | uuid | not null |
| upload_session_id | uuid | not null，fk |
| part_no | int | not null，从 1 起 |
| size_bytes | int | not null |
| part_hash | text | not null |
| received_at | timestamptz | not null |
| created_at / created_by | | 按公共列 |

索引：`pk_upload_parts`；`ux_upload_parts_le_session_part_no`；`ix_upload_parts_le_created`。

同一分片重传时以 `part_hash` 相同判为幂等重复，直接返回成功；`part_hash` 不同判为 `PLATFORM.UPLOAD_SESSION.PART_HASH_MISMATCH`。

**表 24 `platform_file.scan_results`**（仅追加）

| 列 | 类型 | 约束 |
|---|---|---|
| id | uuid | not null，pk |
| legal_entity_id | uuid | not null |
| attachment_version_id | uuid | not null，fk |
| inspector | text | not null，ck 取值 `TYPE_SNIFF`、`STRUCTURE`、`CLAMD` |
| verdict | text | not null，ck 取值 `PASS`、`REJECT`、`SKIPPED`、`ERROR` |
| detail | text | null，长度不超过 2000 |
| engine_version | text | null |
| scanned_at | timestamptz | not null |
| duration_ms | int | not null |
| created_at / created_by | | 按公共列 |

索引：`pk_scan_results`；`ix_scan_results_le_version_created`；`ix_scan_results_le_created`。

**表 25 `platform_core.module_registrations`**（3b 段，部署级，不带 `legal_entity_id`，不建策略）

| 列 | 类型 | 约束 |
|---|---|---|
| id | uuid | not null，pk |
| module_code | text | not null，ck 取值为基线第 1.2 节的 15 个模块码，与 `ep_foundation::ModuleCode` 的序列化取值一一对应 |
| display_name | text | not null，长度不超过 64 |
| install_state | text | not null，ck 取值 `NOT_INSTALLED`、`INSTALLED_ENABLED`、`INSTALLED_DISABLED` |
| installed_at | timestamptz | null |
| state_changed_at | timestamptz | null |
| row_version, created_at, created_by, updated_at, updated_by | | 按公共列 |

索引：`pk_module_registrations`；`ux_module_registrations_module_code`；`ix_module_registrations_install_state_created_at`。

**表 26 `platform_core.license_grants`**（3b 段，部署级）

| 列 | 类型 | 约束 |
|---|---|---|
| id | uuid | not null，pk |
| license_no | text | not null，`ux_license_grants_license_no` 唯一 |
| issued_to | text | not null |
| valid_from | date | not null |
| valid_to | date | not null |
| named_user_limit | int | not null，大于 0 |
| module_codes | text[] | not null，元素取值同 `module_registrations.module_code` |
| signature | bytea | not null |
| revoked_at | timestamptz | null |
| row_version, created_at, created_by, updated_at, updated_by | | 按公共列 |

索引：`pk_license_grants`；`ux_license_grants_license_no`；`ix_license_grants_valid_to_created_at`。

**表 27 `platform_core.feature_flags`**（3b 段，部署级）

| 列 | 类型 | 约束 |
|---|---|---|
| id | uuid | not null，pk |
| feature_code | text | not null，`ux_feature_flags_feature_code` 唯一 |
| module_code | text | not null |
| is_enabled | boolean | not null |
| requires_license | boolean | not null |
| row_version, created_at, created_by, updated_at, updated_by | | 按公共列 |

索引：`pk_feature_flags`；`ux_feature_flags_feature_code`；`ix_feature_flags_module_code_created_at`。

**表 28 `platform_meta.config_packages`**（3b 段，部署级）

列与约束按阶段 13 计划第 3.2.10 节逐项照建，本阶段一次建齐列集，只收窄 `status` 的 CHECK：本阶段取值为 `DRAFT`、`PENDING_APPROVAL`、`REJECTED`、`APPROVED`、`RELEASED`、`ROLLED_BACK` 六项，对应第 3.4.12 节的六态状态机；阶段 13b 扩展该 CHECK 到 PRD 第 10.4.1 节的十一态。

**表 29 `platform_meta.config_package_items`**（3b 段，部署级）

列与约束按阶段 13 计划第 3.2.11 节逐项照建。`item_kind` 的 CHECK 一次建齐 15 项，与 3a 段冻结的 `ItemKind` 枚举逐项一致，未在 `ConfigItemApplierRegistry` 注册实现的 `item_kind` 由运行期校验拒绝发布，不靠 CHECK 拦截。

**表 30 `platform_meta.config_release_orders`**（3b 段，部署级）

列与约束按阶段 13 计划第 3.2.12 节逐项照建。`status` 的 CHECK 一次建齐，本阶段只使用 `SUBMITTED`、`APPROVED`、`REJECTED`、`EXECUTING`、`SUCCEEDED`、`FAILED`、`CANCELLED` 七项，`QUEUED` 与 `COMPENSATED` 留给阶段 13b 的停机窗口排队与 DDL 补偿。

本阶段只建上述三张配置表。`config_edit_locks` 与阶段 13 计划中承载逐项落地记录的 `config_release_steps` 按裁定 A-27 由阶段 13b 建，`config_autotest_runs` 与 `config_release_mutex` 同属阶段 13b。

#### 3.3.5 只读运维视图

`platform_msg.v_outbox_backlog`：按法人、状态、事件类型聚合的待处理条数与最老条目年龄，供 ops-agent 的 `ep_ops_ro` 角色读取。
`platform_msg.v_dead_letter_backlog`：按法人、状态、会计期间聚合的死信条数。
`platform_flow.v_flow_backlog`：按法人、状态聚合的实例数与人工任务数。
`platform_audit.v_anchor_lag`：按法人的最近一次成功锚定时间与未锚定条数，供 `ep_audit_anchor_age_seconds` 指标与规格第 15.3 章“审计段根哈希最近一次成功锚定超过约定间隔”的台账条目使用。

四个视图均按 `v_` 前缀，不用物化视图，符合基线第 3.2 节。视图继承基表的行级安全，`ep_ops_ro` 读取时同样按法人逐轮设置会话变量。

#### 3.3.6 为写出进程预留的两个视图

`platform_file.v_attachment_watermark_inputs`：输出 `legal_entity_id`、`attachment_version_id`、`storage_path`、`size_bytes`、`available_at`，按 `available_at` 升序。这是规格第 13.4 章“附件正文写出点是一个水位时刻，在该时刻之前提交的全部附件元数据其对应正文都已完成写出”的元数据侧输入，本阶段只提供输入，水位推进与写出由后续阶段的 archive-writer 实现。

`platform_audit.v_evidence_write_inputs`：输出 `legal_entity_id`、`audit_segment_id`、`anchor_id`、`evidence_path`、`evidence_written_at`，供 archive-writer 按 15 分钟周期把审计证据存储写出到服务器之外落点。

两个视图是本阶段与备份归档阶段之间唯一的接缝，不通过表结构耦合。

---

### 3.4 领域模型与关键算法

本阶段不涉及任何账务处理。凡与会计相关的取价、借贷与期间归属一律按规格第 5.2 章财务规则条目的事件-分录表执行，本阶段只在 Outbox 信封上承载 `posting_date` 与 `accounting_period_id` 两个字段并保证其可枚举，不解释其语义。

#### 3.4.1 单据编号

核心结构体：`DocumentNumber { type_code: TypeCode, legal_entity_code: LegalEntityCode, period_key: PeriodKey, serial: u64, width: u8 }`，`Display` 输出 `SO-01-202608-000123`。`legal_entity_code` 取 `ep_platform_tenancy::LegalEntityRef` 的 `entity_no`（2 位数字），经 `LegalEntityDirectory` 读取，见第 3.13 节依赖三。

取号算法，在业务事务内执行，共五步。

1. 由类型码注册表校验 `type_code` 已登记，未登记返回 `PLATFORM.SEQUENCE.TYPE_CODE_NOT_REGISTERED`。
2. `period_key` 由记账日期或业务日期按 `(date)::text` 取前六位得出；档案类固定 `000000`。
3. 执行 `INSERT INTO platform_core.number_sequences (...) VALUES (...) ON CONFLICT DO NOTHING`，保证序列行存在。
4. 执行取号语句。

```sql
update platform_core.number_sequences
   set width = case when next_value + 1 > (power(10, width)::bigint - 1)
                    then width + 1 else width end,
       next_value = next_value + 1,
       row_version = row_version + 1,
       updated_at = now(), updated_by = $u
 where legal_entity_id = $1 and scope_kind = $2
   and type_code = $3 and period_key = $4
returning next_value as serial_value, width as effective_width;
```

`UPDATE ... SET` 的全部右侧表达式读同一行的旧值，因此 `width` 的判定用旧 `next_value` 与旧 `width`，`next_value` 也读旧值，两者语义自洽。

5. 按 `effective_width` 补零格式化。

边界条件四项。其一，回滚即退号，因为整个 `UPDATE` 在业务事务内，行锁到提交才释放，故不产生空号，这是基线第 11.1 节的明确要求。其二，同一法人同一类型同一月份的取号串行，20 并发下写写等待可测但不构成瓶颈；`lock_timeout` 为 3 秒，超时映射为 `PLATFORM.SEQUENCE.ALLOCATION_TIMEOUT`，分类 `INFRASTRUCTURE`，可重试。其三，取号必须紧邻主表插入之前，不得在用例开头取号，理由是持锁时间等于取号到提交的时长。其四，档案编码允许人工指定，人工指定时不走本算法，唯一性由 `(legal_entity_id, code)` 唯一约束保证，冲突返回 `PLATFORM.SEQUENCE.CODE_ALREADY_EXISTS`；单据编号不允许人工指定，传入即返回 `PLATFORM.SEQUENCE.MANUAL_CODE_NOT_ALLOWED`。

#### 3.4.2 审计事件哈希链追加

核心结构体：`AuditEvent`（列集见表 9）、`SegmentKey { legal_entity_id, event_day }`、`ChainLink { prev_hash: [u8;32], hash: [u8;32] }`。

哈希输入的规范化。本阶段规定审计事件的哈希输入为 RFC 8785 JCS 规范化的 JSON 对象，字段为除 `hash` 外的全部列，`prev_hash` 以小写十六进制字符串承载，`bytea` 与 `uuid` 一律以字符串承载，`occurred_at` 以 RFC 3339 UTC 微秒精度字符串承载。`before` 与 `after` 中的一切数值型业务数据一律以字符串承载，不以 JSON number 承载。最后一条是硬性要求：PostgreSQL 的 `jsonb` 会对数字做规范化，`1.10` 与 `1.1` 回读后不可区分，若以 number 承载则哈希不可重算，链验证会在没有任何篡改的情况下报失败。登记为本阶段新增决定。

追加算法，在业务事务内执行，七步。

1. 计算 `event_day = (occurred_at AT TIME ZONE 'Asia/Shanghai')::date`。
2. 收集本事务内待写入的全部审计事件，按 `event_day` 分组；若跨越两个自然日，按 `event_day` 升序依次处理，避免与另一事务反向加锁形成死锁。
3. 对每个 `event_day`，执行 `INSERT INTO platform_audit.audit_segments ... ON CONFLICT (legal_entity_id, event_day) DO NOTHING`，再执行 `SELECT last_hash, last_seq, event_count FROM platform_audit.audit_segments WHERE legal_entity_id = $1 AND event_day = $2 FOR UPDATE`。这一步是判定二所述的串行化点，`lock_timeout` 为 3 秒，超时映射为 `PLATFORM.AUDIT_EVENT.SEGMENT_LOCK_TIMEOUT`。
4. 以 `last_hash` 为 `prev_hash`（段首条取 32 字节全零），按上述规范化算出 `hash = SHA-256(canonical_bytes)`。多条事件在同一段内按写入顺序链式串接，第 n 条的 `prev_hash` 是第 n−1 条的 `hash`。
5. 批量 `INSERT` 全部事件，`seq` 由 `bigserial` 分配。
6. 更新段行的 `last_hash`、`last_seq`、`event_count`、`first_seq`（首次写入时设置），`row_version` 加一。
7. 事务提交。

边界条件五项。其一，审计写入必须是工作单元闭包内的最后一批写入，段锁持有时间因此近似为一次更新加一次提交刷盘的时长；本阶段设内部观测目标为审计写入对业务事务耗时的增量 P95 不超过 15 毫秒，作为容量健康度的观察项，不作为规格通过线。其二，`seq` 空洞不判为链断裂。其三，同一事务写多条事件时链内顺序即写入顺序，验证时按 `seq` 升序即可重现。其四，段跨日不影响链，跨日的第一条事件在新段中 `prev_hash` 为全零，段与段之间不建立跨段链接，这与规格第 12.5 章“每段为一条独立链”一致。其五，客户端提交的审计事件不建本地分段链，由中心按同一序列写入对应法人与自然日的段，规格第 12.5 章明确要求，本阶段的追加接口对全部来源使用同一路径，不为客户端开第二条路径。

#### 3.4.3 段根签名、锚定与链验证

锚定触发条件：某段自上次锚定以来经过时间不少于 `anchor_interval_seconds`（默认 300），或未锚定事件条数不少于 `anchor_event_threshold`（默认 1000）。扫描周期 `anchor_scan_interval_seconds` 默认 30 秒，由 job-worker 按法人轮转执行。

锚定分三段，理由是签名要调用 KMS、写证据文件要落盘，两者都不能在持有段锁的事务内做，否则会把业务事务的段锁等待放大到 KMS 与磁盘的延迟上。

阶段 A，短事务。取段行 `FOR UPDATE`，读 `last_seq` 与 `last_hash`。取到锁即说明该段无在途审计写入，因为一切审计写入都要先取该锁。插入 `audit_anchors` 一行，`anchor_seq = last_seq`，`root_hash = last_hash`，`state = 'PENDING_SIGN'`，更新段行的 `last_anchor_seq`。提交。持锁时间为两次单行写。

阶段 B，无锁。以 `key_ref` 指向的签名私钥对 `SHA-256(JCS({segment_id, legal_entity_id, event_day, anchor_seq, root_hash, event_count}))` 做 ECDSA P-256 签名。算法取值来自规格第 12.3 章“摘要使用 SHA-256，签名使用 RSA 或 ECDSA”，本阶段取 ECDSA P-256，理由是签名长度短、验签快，且首版不含商用密码档位。`UPDATE ... SET signature = $1, signed_at = now(), state = 'SIGNED' WHERE id = $2 AND state = 'PENDING_SIGN'`，受影响行数为 0 即已被并发处理，直接跳过。

阶段 C，写证据。经 `ep-adapter-file` 的 `evidence` 命名空间以 `create_new` 写入 `<legal_entity_id>/<event_day>/<anchor_seq>-<anchor_id>.json`，内容为 JCS 规范化的锚定记录。`create_new` 遇到同名文件返回已存在，视为幂等成功。写成功后 `UPDATE ... SET state = 'EVIDENCED', evidence_path, evidence_written_at`。按裁定 C-27，审计证据目录为 `/var/lib/ep/audit-evidence`，属主 `ep-worker`，组 `ep`，权限 0750；证据文件与段根签名一律由 job-worker 产生，archive-writer 以组 `ep` 的只读权限读取并写出到服务器之外落点，本进程不承担写出，也不授予 archive-writer 任何写入与删除权限。

失败处理：阶段 B 或 C 失败时 `state` 保持不变、`last_error` 记录，由下一轮扫描重试；连续失败超过 8 次置 `FAILED` 并写死信与站内通知，指标 `ep_audit_evidence_write_failures_total` 上升。规格第 12.5 章要求“最近一次成功锚定时间在运维中心可见，超过约定间隔告警”，本阶段以 `ep_audit_anchor_age_seconds` 指标与 `platform_audit.v_anchor_lag` 视图承载，台账条目的登记由运维中心所在阶段实现，本阶段提供数据源。

链验证算法，按规格第 12.5 章“按法人、日期段或单条事件验证哈希前后连续，段根哈希与审计证据存储中的签名一致”，六步。

1. 校验请求范围不超过 `verify_max_days`（默认 92），超出返回 `PLATFORM.AUDIT_VERIFICATION.RANGE_TOO_WIDE`。
2. 对范围内每个 `(legal_entity_id, event_day)` 段，按 `seq` 升序流式读取全部事件。
3. 逐条重算 `hash` 并与存储值比较；比较 `prev_hash` 与上一条的 `hash`；段首条要求 `prev_hash` 为全零。
4. 对该段的每个 `audit_anchors` 行，取 `anchor_seq` 处事件的 `hash`，与 `root_hash` 比较；用公钥验签。
5. 读取 `evidence_path` 指向的证据文件，与库内锚定记录逐字段比对。
6. 输出报告：逐段给出通过或不通过，不通过时定位到首个失败的 `event_id` 与失败原因（哈希不匹配、链断裂、签名不一致、证据缺失、证据内容不一致五类）。

边界条件三项。其一，验证是只读操作，不取段锁，因此可能读到验证期间新增的事件，实现上以“验证开始时的 `last_seq` 快照”为上界，超出上界的事件不纳入本次验证。其二，验证在单个 `REPEATABLE READ` 事务或由其导出的快照上执行，与基线第 8.4 节对内部对账的口径一致。其三，验证作为后台任务由 job-worker 执行，因为基线第 7.1 节把命令行参数冻结为三个，不允许新增子命令，故不交付独立 CLI，验证入口只有 API 加后台任务，登记为本阶段新增决定。

#### 3.4.4 Outbox 与幂等

写入侧：`OutboxWriter::enqueue(tx, &events)`。信封字段缺任一必填项即返回 `PLATFORM.OUTBOX.ENVELOPE_INCOMPLETE`；`event_type` 未在事件目录登记即返回 `PLATFORM.OUTBOX.EVENT_TYPE_NOT_REGISTERED`。规格第 7.9 章要求“缺少来源对象 ID、版本、法人 ID、密级和数据范围标签的事件不得写入派生存储”，本阶段把这一校验前移到写入侧，`security_level` 与 `data_scope_tags` 缺失即拒绝入 Outbox，不留到消费侧再发现。

取件侧，按法人轮转，取件语句按基线第 6.2 节固定：

```sql
select ... from platform_msg.outbox_events
 where status = 'PENDING' and available_at <= now()
 order by available_at, event_id
   for update skip locked limit 100;
```

批量 100，轮询 200 毫秒，无待处理时退避到 2 秒。取件事务内即把 `status` 置 `DISPATCHING`、`locked_by` 置进程标识、`locked_until` 置 `now() + lock_lease_seconds`，提交后再投递。投递为进程内的处理器调用，不发起外部 HTTP，唯一例外是推送出口，它由 integration-gateway 独立消费，见第 3.4.6 节。

消费侧幂等：处理器的副作用与 `INSERT INTO platform_msg.inbox_consumptions (consumer, event_id) ...` 在同一事务内，唯一约束冲突即判为已消费，跳过副作用并直接置 `DONE`。这是规格第 7.3 章“Outbox 可靠投递测试项，覆盖至少一次投递、重复投递去重和崩溃恢复后不丢不重”的实现依据。

重试退避固定八段：1 秒、5 秒、30 秒、2 分钟、10 分钟、30 分钟、1 小时、2 小时。第 8 次失败后事务内完成三件事：插入 `dead_letters`、把 `outbox_events.status` 置 `DEAD`、写审计事件。转死信时的 `failure_category` 取五类分类之一，取值来自处理器返回的 `AppError.category`。

崩溃恢复：`locked_until` 过期的 `DISPATCHING` 条目由扫描器改回 `PENDING` 并把 `available_at` 置当前时刻。因为投递副作用与 `inbox_consumptions` 同事务，重放不会重复产生副作用。

幂等键算法（3a 段）：本阶段只实现阶段 2 定义的 `ep_adapter_db::port::IdempotencyStore`，请求头的存在性与 UUIDv7 合法性已由阶段 1 的 `IdempotencyKeyHeaderGuard` 判定，本阶段不重复判断。`try_begin(tx, scope: IdempotencyScope, request_hash: [u8; 32])` 在业务事务内先执行 `INSERT INTO platform_msg.idempotency_keys (..., state) VALUES (..., 'IN_PROGRESS') ON CONFLICT (legal_entity_id, user_id, endpoint, key) DO NOTHING`，`IdempotencyScope` 的四个字段与该唯一约束逐项对应，`request_hash` 以 64 位小写十六进制存入 `request_hash` 列。受影响行数为 1 返回 `IdempotencyOutcome::FirstCall`；为 0 则读取已有行：`state = 'COMPLETED'` 且 `request_hash` 相同时返回 `Replay { status, body }`，调用方据此回放并带 `Idempotent-Replay: true`；`request_hash` 不同时返回 `PayloadMismatch`，由调用方映射为 `PLATFORM.IDEMPOTENCY.PAYLOAD_MISMATCH`；`state = 'IN_PROGRESS'` 时首次调用尚在执行，此时不返回任何 `IdempotencyOutcome`，而以 `Err(AppError)` 返回 `PLATFORM.IDEMPOTENCY.IN_PROGRESS`。`finish(tx, scope, response_status: u16, response_body: &[u8])` 在同一事务内把 `state` 置 `COMPLETED` 并写入 `response_status` 与 `response_body`。事务回滚时 `IN_PROGRESS` 行一并回滚，不留残留。`request_hash` 取请求体规范化后的 SHA-256，不含请求头。保留 7 天。

#### 3.4.5 站内通知与扇出

站内通知在业务事务内同步写入，不经 Outbox。理由是规格第 5.1 章把站内通知定为“首版唯一验收不可豁免的通知渠道”，把它挂在至少一次投递的异步链路上会引入一个本可避免的丢失面与延迟面；而通知的写入是同库单表插入，放进业务事务不违反基线第 10.3 节的事务内禁止清单。登记为本阶段新增决定。

写入算法四步。其一，按 `notice_type` 取当前生效的模板版本。其二，解析接收人，得到 `Vec<UserId>`；解析方式由触发源决定：审批待办取 `process_tasks.assignee_user_id` 或候选角色展开，审批结果取 `process_instances` 的发起人，对账差异取该法人的数据责任人，许可宽限期取全体在职用户。其三，对每个接收人渲染标题与正文，渲染只允许使用模板声明的变量集合，声明外变量拒绝渲染；渲染前对该接收人做一次字段可见性裁剪，无权字段以事项类型与编号替代。其四，批量插入 `notifications` 与 `notification_deliveries`（`IN_APP` 通道直接 `DELIVERED`），`ON CONFLICT (legal_entity_id, recipient_user_id, dedupe_key) DO NOTHING` 完成去重。

扇出规模：首版命名用户 50，最大扇出为许可宽限期告警的 50 行，单事务可承受。若某类提醒的接收人超过 200，改为写一条 Outbox 事件由 job-worker 分批扇出，阈值 `notify.sync_fanout_max` 默认 200。

未读上限：单用户未读数达到 `unread_cap_per_user`（默认 2000）时，新通知仍写入，同时把该用户最旧的已超过保留期的已读通知纳入下一轮清理，并写一条 `WARN` 级运行日志。不丢新通知，这是不可豁免渠道的底线。

保留期与未读上限两个取值对应 PRD 附录乙 U-K-04，本阶段给临时取值 180 天与 2000 条，标注为待产品负责人决策；切换代价只是改配置，不涉及结构变更，故本阶段不被阻塞。

#### 3.4.6 移动推送出口

推送是站内通知之上的可选增强，不是任何提醒的保证渠道，规格第 5.1 章与 PRD 第 10.5.1 节都明确。因此推送链路的任何失败一律不产生用户可见错误，只记录 `notification_deliveries.status = 'FAILED'` 与指标。

链路四步。其一，core-server 写入通知的同一事务内，若该接收人存在活跃 `push_registrations` 且 `notify.push_enabled` 为真，写一条 `platform.notification.push_requested.v1` 到 Outbox 并插入 `notification_deliveries` 的 `MOBILE_PUSH` 行为 `PENDING`。其二，job-worker 消费该事件，组装推送载荷：默认只含事项类型与关联单据编号，不含任何业务字段，由 `notify.push_body_includes_business_fields` 控制，默认关闭，对应 PRD 附录乙 U-K-05 且默认取最保守值。其三，job-worker 调用 integration-gateway 的 `POST http://127.0.0.1:8082/internal/v1/push/dispatch`，超时 5 秒。其四，integration-gateway 执行出网投递，带超时、退避与熔断，把结果回写到 `notification_deliveries`。

连续失败达到阈值的 `push_registrations` 行置 `is_active = false`，理由是失效令牌会持续消耗出网重试预算。

推送出口的进程归属是本阶段对共享技术基线的一处实质偏离，见第 3.12 节偏离项一。

#### 3.4.7 附件上传流水线与本地文件存储

规格第 7.5 章的上传流程为“临时对象、哈希与类型校验、病毒与恶意内容检查、数据库确认、正式发布”，本阶段逐段实现。

上传会话状态机：

| 当前状态 | 目标状态 | 触发 | 守卫条件 |
|---|---|---|---|
| — | INITIATED | init-upload | 单用户并发上传数未超上限，全局并发上传数未超上限，声明大小不超过单文件上限，剩余磁盘空间足够 |
| INITIATED | UPLOADING | 首个分片写入成功 | 会话未过期 |
| UPLOADING | UPLOADING | 后续分片写入 | 分片序号在 1..part_count，同序号重传时哈希一致 |
| UPLOADING | ASSEMBLING | complete | 已收分片数等于 part_count，各分片哈希已校验 |
| ASSEMBLING | SCANNING | 组装与总哈希校验通过 | 总哈希等于声明哈希，总大小等于声明大小 |
| ASSEMBLING | REJECTED | 总哈希或大小不符 | — |
| SCANNING | COMMITTED | 全部检查器返回 PASS 或 SKIPPED | 版本行由 PENDING 置 AVAILABLE 成功 |
| SCANNING | REJECTED | 任一检查器返回 REJECT | — |
| INITIATED/UPLOADING | ABORTED | abort 或超时回收 | — |
| 任一非终态 | EXPIRED | 超过 expires_at | — |

正文落盘的三段式，对应判定三。

第一段，事务 A：写 `attachment_versions` 一行，`state = 'PENDING'`，`storage_path` 预先由 `<legal_entity_id>/<security_level>/<yyyy>/<mm>/<version_id>` 确定，`dek_ref` 与 `key_domain_ref` 由 KMS 派生。事务提交。

第二段，事务外：从 staging 流式读取分片，用会话级临时密钥解密，边解密边计算明文 SHA-256，边用该版本的数据密钥以 AES-256-GCM 加密，经 `ep-adapter-file` 的 `published` 命名空间以 `create_new`（`O_CREAT | O_EXCL`）写入目标路径，写完 `fsync`。若目标路径已存在，判为前次崩溃后的重入，跳过写入直接进入第三段。

第三段，事务 B：校验明文哈希等于声明哈希，把 `attachment_versions.state` 由 `PENDING` 置 `AVAILABLE` 并写 `available_at`，更新 `attachment_objects.current_version_no`，写审计事件与 `platform.attachment.published.v1` 到 Outbox，把 `upload_sessions.state` 置 `COMMITTED`。提交后异步删除 staging 分片。

staging 的加密取舍：分片以会话级临时密钥加密后落盘，而不是明文落盘。理由是恶意内容检查需要明文，若明文落盘则在检查与加密之间存在一段明文驻留窗口，规格第 6.5 章要求“附件在所属法人密钥域内加密存储”，明文窗口虽不在正式路径上但仍是同一台服务器上的可读副本。采用会话级密钥后，扫描路径改为流式解密到管道，明文不落盘。staging 目录权限 0700，属主 ep-core，会话终态后立即删除；staging 不进入任何写出与备份范围。

崩溃收敛：job-worker 内的幂等收敛任务按周期扫描 `state = 'PENDING'` 且 `created_at` 早于 `now() - 30 分钟` 的版本行，路径上文件存在则补做事务 B，不存在则置 `FAILED`。`version_id` 唯一且不复用，故失败的 `storage_path` 永不重用，不会与后续写入冲突。按裁定 A-06，该任务不是对账：它只按“路径上文件是否存在”做幂等收敛，不产生对账差异事项，不实现 `ep_platform_recon::ReconCheck`，也不依赖阶段 9a 交付的 `ep-platform-recon` 框架。

存储适配的不可变约束：`ep-adapter-file` 的 `published` 与 `evidence` 两个命名空间在 Rust 类型层面就不暴露删除与覆盖方法，不是运行期检查。理由是规格第 7.5 章要求“承载上述内容的文件存储路径在应用侧不开放覆盖写与原地删除接口”，运行期检查可被绕过，类型层面不可。物理删除只经 `DisposalPort`，该端口按裁定 A-22 定义在 `crates/platform/file/src/port/disposal.rs`：

```rust
pub struct DisposalRequest {
    pub disposal_plan_id: uuid::Uuid,
    pub scope: DisposalScope,          // AttachmentObjects、KeyDomain、BackupSets、ExtTables
    pub object_refs: Vec<DisposalObjectRef>,
    pub approval_ref: uuid::Uuid,
    pub second_approver_id: Id<UserAccount>,
    pub reauth_ref: uuid::Uuid,
}
pub struct DisposalReceipt {
    pub disposal_plan_id: uuid::Uuid,
    pub disposed_count: u64,
    pub certificate_ref: String,
    pub executed_at: chrono::DateTime<chrono::Utc>,
}
#[async_trait::async_trait]
pub trait DisposalPort: Send + Sync {
    async fn dispose(&self, ctx: &SecurityContext, req: DisposalRequest)
        -> Result<DisposalReceipt, AppError>;
}
```

本阶段只交付上述 trait 与两个 DTO，并在 `apps/core-server/src/wiring.rs` 与 `apps/job-worker/src/wiring.rs` 注入 `NoopDisposalPort`，该行标注 `// TODO(stage-14): replace with real impl`；实现类型为阶段 14 的 `OpsDisposalService`，位于 `crates/platform/obs/src/disposal.rs`，只由 ops 专用路径与专用账号触发。

恶意内容检查：`ContentInspector` 端口，三个内置实现。`TYPE_SNIFF` 按魔数识别真实类型并与声明类型比对，不一致且不在允许的等价集合内即 REJECT。`STRUCTURE` 检查可执行文件头、OOXML 与 ODF 中的宏与外部引用、PDF 中的 JavaScript 与自动动作、归档炸弹（压缩比与展开深度上限）。`CLAMD` 是可选实现，经本机 Unix socket 调用 clamd，未配置时返回 SKIPPED。

假设：规格第 6.5 章的措辞是“适用的病毒扫描”，本阶段据此假定病毒扫描引擎是客户环境可选组件而非平台必备交付项，平台交付的是接入点与三个内置检查器；该假设的理由是引入病毒库需要持续更新通道，与单机不出网形态冲突，且引擎的许可条款会影响私有化分发。该假设与其后果写入交付说明。若产品负责人判定必须内置引擎，本阶段只需增加一个 `ContentInspector` 实现，不影响其余结构。

检查未通过的处置：版本行置 `QUARANTINED`，不可被任何单据引用、不可下载，保留 `quarantine_retention_days`（默认 90）后由处置流程删除。对应 PRD 附录乙 U-L-03，本阶段给临时取值并标注待决，不阻塞。

#### 3.4.8 持久化工作流引擎

实例状态机：

| 状态 | 可去往 | 触发 | 守卫条件 |
|---|---|---|---|
| CREATED | RUNNING | 调度器首次取件 | 定义版本存在且为 PUBLISHED |
| RUNNING | WAITING | 步骤产出等待条件 | 已写入对应的人工任务或定时器 |
| RUNNING | RUNNING | 步骤完成且有后继 | 步骤数未超上限，并行分支数未超上限，未超最长运行期 |
| WAITING | RUNNING | 人工任务完成或定时器触发 | 触发的幂等键未被消费 |
| RUNNING | COMPLETED | 到达结束节点 | 无活跃分支 |
| RUNNING | COMPENSATING | 步骤重试耗尽且定义声明需补偿 | 存在可补偿的已完成步骤 |
| RUNNING | FAILED | 步骤重试耗尽且定义未声明补偿 | — |
| COMPENSATING | COMPLETED | 全部补偿完成 | — |
| COMPENSATING | MANUAL_INTERVENTION | 任一补偿重试耗尽 | 已写入 COMPENSATION_FAILURE 人工任务并告警 |
| RUNNING/WAITING | MANUAL_INTERVENTION | 触及运行约束上限 | 已写入 LIMIT_EXCEEDED 人工任务 |
| CREATED/RUNNING/WAITING | CANCELLED | 取消命令 | 调用方具备取消权限，实例未处于 COMPENSATING |
| MANUAL_INTERVENTION | RUNNING / CANCELLED | 人工处置 | 处置人记名并写审计 |

执行模型：一步一事务。这是崩溃恢复正确性的关键设计。规格第 17.2 章要求“在步骤执行前、执行中、提交后和补偿过程中随机终止核心进程，恢复后实例状态、业务效果、事件和补偿结果必须与预期一致”；若一次调度跨多个事务，就需要额外的租约与回滚补偿逻辑，而一步一事务把这四个终止点都归约为“事务提交或未提交”两种结果。

单步事务内的动作顺序：加载实例行并 `FOR UPDATE`，按 `definition_version` 加载定义，求值守卫条件选出下一节点，执行该节点，写 `process_steps`，更新实例状态与 `next_wake_at`，写 Outbox，写审计。

调度取件：

```sql
select id from platform_flow.process_instances
 where state in ('CREATED','RUNNING','WAITING')
   and next_wake_at is not null and next_wake_at <= now()
 order by next_wake_at, id
   for update skip locked limit 20;
```

按法人轮转执行，执行并发度 `executor_concurrency` 默认 4。

步骤幂等键：`<instance_id>:<node_id>:<execution_no>`，`execution_no` 由该实例该节点的已有步骤数决定。该键既写入 `process_steps.idempotency_key`（唯一约束），也作为 `Idempotency-Key` 传给被调用的模块用例。重复投递不少于 3 次时，第二次起在唯一约束上冲突，事务回滚并判为已执行，跳过副作用。这同时满足规格第 9.1 章“每个步骤携带幂等键；重复投递不得产生重复业务效果、重复事件或重复审计记录”。

服务步骤的可调用范围：只能调用同进程内经 `ep-contract-<module>` 注入的用例，不得发起外部 HTTP、不得读写文件正文、不得等待用户输入，与基线第 10.3 节的事务内禁止清单一致。需要外部调用的步骤一律建模为“发出 Outbox 事件加等待回调”的两步。

定时器：写入时即插入 `process_timers`，`state = 'SCHEDULED'`。扫描语句按法人轮转：

```sql
select id from platform_flow.process_timers
 where state = 'SCHEDULED' and fire_at <= now()
 order by fire_at, id for update skip locked limit 50;
```

触发事务内把 `state` 由 `SCHEDULED` 置 `FIRED`，受影响行数必须为 1 才继续，然后唤醒实例（置 `next_wake_at = now()`）。实例消费后置 `CONSUMED`。幂等与可重放三点：进程重启后未 `FIRED` 的定时器仍在表中，不漏触发；已 `FIRED` 未 `CONSUMED` 的由实例推进消费，不漏；重复扫描被状态更新的受影响行数拦住，不重。实现不把单副本当作唯一触发的前提，规格第 9.1 章明确要求这一点。

SLA：以 `kind = 'SLA'` 的定时器表达，触发时不推进实例，只写 `process_tasks.sla_breached_at` 并产生一条流程时限提醒通知，对应 PRD 第 10.5.2 节的“流程时限提醒”。

补偿：进入 `COMPENSATING` 后，按 `process_steps` 中 `outcome = 'COMPLETED'` 且 `is_compensable = true` 的记录按 `step_no` 降序逐条执行，每条一个事务，写 `process_compensations`（`reverses_id` 指向被补偿步骤）。单条补偿重试上限 `compensation_max_attempts` 默认 5，退避同步骤退避。任一条重试耗尽即实例置 `MANUAL_INTERVENTION`，写 `COMPENSATION_FAILURE` 人工任务、发站内通知、指标 `ep_flow_instances_manual_intervention` 上升，不静默结束，规格第 9.1 章明确要求。允许部分失败：已成功的补偿不回滚，人工任务中列出已完成与未完成的补偿清单。

版本与迁移：实例持 `definition_id` 与 `definition_version`，推进时按该版本加载，新版本发布不影响运行中实例。版本迁移是显式命令，支持 `dry_run = true` 的模拟，模拟输出每个待迁移实例的当前节点在新版本中是否存在、变量是否兼容；不可迁移的实例列出原因并不迁移。迁移记录写审计，可按批次回退。

运行约束：`max_instance_duration_days`、`max_steps_per_instance`、`max_parallel_branches` 三项超限即置 `MANUAL_INTERVENTION` 并写 `LIMIT_EXCEEDED` 人工任务，对应规格第 9.1 章“单实例最长运行期、最大步骤数、最大并行分支和实例保留期由配置约束，超限进入人工处理”。

守卫条件表达式：本阶段只交付最小求值器，支持字段引用（`vars.x`、`instance.state`）、比较（六种）、逻辑（与或非）、集合成员、空判定，以及一个不超过 12 个函数的白名单（长度、上取整、日期加减等）。表达式无副作用、无循环、求值步数上限 1000，超限返回 `VALIDATION`。该求值器只服务于流程守卫条件，不是 `RuleEvaluator` 的实现。完整的声明式规则引擎与受限 WASM 计算不在本阶段范围，本阶段只保证接口位点存在：`ep_platform_flow::port::RuleEvaluator` 与 `ep_platform_flow::port::WasmComputePort` 两个 trait 定义在 `ep-platform-flow`，按裁定 B-05，其实现类型分别为 `AstRuleEvaluator`（位于 `crates/platform/meta/src/rule/`，装配进 core-server）与 `PluginHostWasmCompute`（位于 `crates/adapter/wasm/`，装配进 plugin-host），两者均由阶段 13b 交付；本阶段在两个 wiring 注入 `NoopRuleEvaluator` 与 `NoopWasmComputePort`，该行标注 `// TODO(stage-13): replace with real impl`。端点 `POST /api/v1/platform/rule-evaluations/actions/evaluate` 属阶段 13b，本阶段不建第二条求值路径。

#### 3.4.9 错误分类与重试

分类到 HTTP 的映射按基线第 5.5 节固定，不增不减。按裁定 C-24，`PLATFORM.IDEMPOTENCY.KEY_REQUIRED`、`PLATFORM.IDEMPOTENCY.PAYLOAD_MISMATCH`、`PLATFORM.CAPACITY.CONCURRENCY_LIMIT`、`PLATFORM.CONCURRENCY.STALE_VERSION`、`PLATFORM.AUTHZ.NOT_FOUND_OR_DENIED`、`PLATFORM.AUTHZ.OBJECT_FORBIDDEN`、`PLATFORM.DB.MIGRATION_WINDOW_CLOSED` 七个码由阶段 1 登记在 `crates/foundation/src/error/codes.rs` 与 `docs/error-codes.md` 的 `PLATFORM` 段，本阶段只引用，登记冲突由 CI 判负。本阶段实现四件事。

其一，`AppError` 到响应封套的统一映射中间件，位于 core-server 的 HTTP 层，只做一次翻译，各 crate 的 `Error` 到 `AppError` 的映射写在各自 `error.rs`。

其二，`incident_no` 的生成：格式 `ERR-<YYYYMMDD>-<6 位法人内流水>`，由 `ep-platform-sequence` 以 `scope_kind = 'DOCUMENT'`、`type_code = 'ERR'` 分配。事故编号必须在错误路径上可分配，因此该分配走独立短事务，不参与已回滚的业务事务；分配失败时退化为 `ERR-<YYYYMMDD>-<trace_id 前 12 位>`，保证错误响应永远有关联编号，规格第 15.1 章要求“每个错误包含关联编号”。

其三，数据库重试：序列化失败 40001 与死锁 40P01 在 `ep-adapter-db` 的工作单元层重试 3 次，退避 50、150、450 毫秒，且只对尚未产生任何外部可见副作用的事务重试。“外部可见副作用”在本阶段的判定为：该事务尚未调用过任何 `ep-adapter-file` 的写入方法、尚未调用过 integration-gateway。重试次数计入阶段 2 注册并填充的 `ep_db_tx_retries_total`（counter，标签 `pool` 与 `sqlstate`），按裁定 C-21 本阶段不登记 `ep_tx_retry_total`。重试耗尽返回 `PLATFORM.RETRY.SERIALIZATION_FAILURE_EXHAUSTED`，分类 `INFRASTRUCTURE`，`retryable = true`。

其四，熔断器：`foundation::resilience::CircuitBreaker`，连续失败达 `circuit_failure_threshold`（默认 5）后开启 `circuit_open_seconds`（默认 30），半开时放 `circuit_half_open_probes`（默认 1）个探针。本阶段唯一使用者是推送出口；电子签章出口的熔断在其所在阶段复用同一组件。规格第 15.1 章规定 `EXTERNAL_SYSTEM` 分类首版仅指电子签章服务，因此推送失败不映射为 `EXTERNAL_SYSTEM`，只记录送达状态，不产生用户可见错误，也不计入错误率口径。

存在性泄漏的统一处理按基线第 5.5 节：不可见记录一律返回 404 与 `PLATFORM.AUTHZ.NOT_FOUND_OR_DENIED`。本阶段全部读端点按此实现，包括通知详情、附件详情、审计事件详情、流程实例详情。

#### 3.4.10 全文检索索引写入与查询（3b 段）

类型体按裁定 A-07 冻结，阶段 1 在 `crates/foundation/src/port/search.rs` 建空文件，本阶段补齐：

```rust
pub struct SearchDocument {
    pub legal_entity_id: Id<LegalEntity>,
    pub object_type: String,          // 形如 "mdm.customers"
    pub object_id: uuid::Uuid,
    pub title: String,
    pub body: String,
    pub tags: Vec<String>,
    pub security_level: SecurityLevel,
    pub data_scope_tags: Vec<String>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}
pub struct SearchQuery {
    pub legal_entity_id: Id<LegalEntity>,
    pub keyword: String,
    pub object_types: Vec<String>,
    pub max_security_level: SecurityLevel,
    pub page: u32,
    pub page_size: u32,
}
pub struct SearchHit { pub object_type: String, pub object_id: uuid::Uuid,
                       pub title: String, pub highlight: String, pub score: f32 }

#[async_trait::async_trait]
pub trait SearchIndexPort: Send + Sync {
    async fn upsert(&self, doc: SearchDocument) -> Result<(), AppError>;
    async fn remove(&self, legal_entity_id: Id<LegalEntity>, object_type: &str,
                    object_id: uuid::Uuid) -> Result<(), AppError>;
}
#[async_trait::async_trait]
pub trait SearchQueryPort: Send + Sync {
    async fn search(&self, q: SearchQuery) -> Result<(Vec<SearchHit>, u64), AppError>;
}
```

写入路径只有一条：job-worker 的索引消费者按法人轮转消费 Outbox 事件，在业务事务提交之后调用 `upsert` 与 `remove`。业务事务内不得出现这两个方法的调用，理由是基线第 10.3 节禁止事务内做文件正文读写，索引写入是文件写入。`xtask archcheck` 断言 `SearchIndexPort` 的调用点只出现在 job-worker 装配的消费者模块中，core-server 的用例路径上出现即构建失败。消费失败按 Outbox 的八段退避重试，第 8 次转死信，不影响业务事务。

索引按法人分区，物理路径 `/var/lib/ep/search/<legal_entity_id>/`。分区是行级安全在索引侧的等价物：查询必须带 `legal_entity_id`，跨法人查询按法人逐轮发起，不做跨分区合并查询。`SearchQuery.max_security_level` 取自 `SecurityContext.clearance_level`，不接受调用方传参；命中结果仍按数据库侧的可见性复核一次，索引不作为授权判据，与规格第 7.9 章派生存储安全继承一致。

本阶段不交付任何业务对象的检索文档投影函数。投影由各业务阶段按 `SearchDocument` 结构提供，本阶段只提供合成对象的投影用于自测。

#### 3.4.11 模块许可与生命周期（3b 段）

契约按裁定 A-05 冻结，落在 `ep-platform-license`：

```rust
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum ModuleState { NotInstalled, InstalledEnabled, InstalledDisabled }
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum LicenseStatus { Valid, ExpiringSoon, Expired, Revoked }

pub trait ModuleLicenseQuery: Send + Sync {
    fn module_state(&self, module: ModuleCode) -> ModuleState;
    fn is_feature_enabled(&self, feature_code: &str) -> bool;
    fn license_status(&self) -> LicenseStatus;
}
```

`ModuleCode` 是阶段 1 按基线第 1.2 节 15 个模块码冻结的枚举，本阶段只消费不重新定义。

安装态状态机四条边：`NOT_INSTALLED` 到 `INSTALLED_ENABLED`（安装并启用）、`INSTALLED_ENABLED` 到 `INSTALLED_DISABLED`（停用）、`INSTALLED_DISABLED` 到 `INSTALLED_ENABLED`（再启用）、`INSTALLED_DISABLED` 到 `NOT_INSTALLED`（卸载）。每次迁移写 `state_changed_at` 与一条审计事件，非法迁移返回 `BUSINESS_CONFLICT`。

许可状态判定顺序固定：`revoked_at` 非空为 `Revoked`；`valid_to` 早于当前日期为 `Expired`；距 `valid_to` 不足临期窗口为 `ExpiringSoon`；否则 `Valid`。临期窗口取 30 天，属本阶段临时取值，切换只改配置。

过滤点两处，都在 job-worker：定时器扫描取件后按实例所属定义的模块段解析 `ModuleCode`，Outbox 投递取件后按 `event_type` 的模块段解析 `ModuleCode`；`module_state` 不为 `InstalledEnabled` 时跳过该条，条目保持 `PENDING` 且不累加 `attempts`，模块再启用后自动恢复投递。这落实规格第 5.6 章模块停用后停止定时任务与对外事件，且不把停用误判为投递失败。

自检项 `license-and-modules-consistent` 由本阶段从 Pending 换成实现，判据三条：处于 `INSTALLED_ENABLED` 的模块码集合被至少一张未吊销且未过期的 `license_grants.module_codes` 覆盖；`feature_flags.requires_license` 为真的功能开关其 `module_code` 处于 `INSTALLED_ENABLED`；许可的 `named_user_limit` 不低于当前启用的命名用户数。任一不满足以退出码 78 退出。

模块停用再启用的端到端验收按裁定 A-05 顺延到阶段 13b，本阶段只做单元与集成层面的判定。

#### 3.4.12 配置发布的内容项端口与最小发布通道

3a 段只交付一个文件 `crates/platform/release/src/port/config_item.rs`，内容为下列 trait、`ItemKind` 枚举 15 项、`ConfigPackageItem` DTO 与 `ConfigItemApplierRegistry`。trait 方法签名与阶段 13 计划第 4.6 节逐字一致，其中 `Tx` 取自 `ep_foundation::port::tx`：

```rust
pub trait ConfigItemApplier: Send + Sync {
    fn item_kind(&self) -> ItemKind;
    fn validate(&self, item: &ConfigPackageItem, ctx: &SecurityContext) -> Result<(), AppError>;
    fn apply(&self, tx: &mut dyn Tx, item: &ConfigPackageItem, ctx: &SecurityContext) -> Result<(), AppError>;
    fn revert(&self, tx: &mut dyn Tx, item: &ConfigPackageItem, ctx: &SecurityContext) -> Result<(), AppError>;
    fn requires_derived_store_rebuild(&self, item: &ConfigPackageItem) -> bool;
}
```

`ItemKind` 的 15 项为 CUSTOM_OBJECT、CUSTOM_FIELD、CUSTOM_RELATION、CUSTOM_INDEX、CUSTOM_VIEW、UI_LAYOUT、FLOW_DEFINITION、AUTHZ_ROLE、AUTHZ_POLICY、AUTHZ_FIELD_GRANT、REPORT_DEFINITION、METRIC_DEFINITION、DASHBOARD_DEFINITION、PRINT_TEMPLATE、NOTIFY_RULE，与阶段 13 计划第 3.2.11 节逐项一致。`ConfigPackageItem` 的字段为 `id`、`security_level`、`config_package_id`、`item_kind`、`item_code`、`change_kind`、`applies_to_legal_entity_ids`、`before_spec`、`after_spec`、`item_hash`、`sort_no`。`ConfigItemApplierRegistry` 只提供按 `ItemKind` 注册与查找两个方法，装配在两个 wiring 中。3a 段无表、无用例、不依赖身份与授权，因此可排在阶段 4 之前，阶段 4 的 `AuthzRoleApplier`、`AuthzPolicyApplier`、`AuthzFieldGrantApplier` 由此不再倒挂。

本阶段实现的 applier 共两个：`FlowDefinitionApplier`（位于 `ep-platform-flow`，落地到 `platform_flow.process_definitions`）与 `NotifyRuleApplier`（位于 `ep-platform-notify`，落地到 `platform_msg.notification_templates` 与提醒规则）。其余 13 个按裁定 A-19 分派到阶段 4、11 与 13b，本阶段不代做。查不到实现的 `item_kind` 整包拒绝发布，错误码 `PLATFORM.CONFIG_RELEASE_ORDER.APPLIER_NOT_REGISTERED`，分类 `BUSINESS_CONFLICT`，由本阶段登记。

3b 段的六态发布状态机按裁定 A-27 以 PRD 第 10.4.1 节的十一态为唯一出处，本阶段实现其中六态：Draft 到 PendingApproval（提交审批，守卫为内容项数在 1 至 2000 之间且每项 `item_hash` 与其 `after_spec` 的规范化 SHA-256 一致）、PendingApproval 到 Approved（审批通过，守卫为审批人不等于提交人）、PendingApproval 到 Rejected（审批驳回，记名并写审计）、Approved 到 Released（签名并执行发布单）、Released 到 RolledBack（回退发布单）。差异审查由 `GET /api/v1/platform/config-packages/{id}/diff` 端点承载，不单列为状态。非法迁移返回 `BUSINESS_CONFLICT` 与 `PLATFORM.CONFIG_PACKAGE.*` 的对应码，不静默忽略。其余五态 PendingAutotest、TestFailed、TestPassed、SignedPendingRelease、Superseded 由阶段 13b 补齐，扩展只放宽 `ck_config_packages_status`，不改写任何既有行。

签名与验签：签名算法固定为 ECDSA P-256，密钥经 `ep-adapter-kms` 取用；`item_hash` 为该项 `after_spec` 的 JSON 规范化序列化（键按字典序、无空白、UTF-8）后的 SHA-256 十六进制小写，与阶段 13 计划第 4.7 节一致；导入时逐项重算 `item_hash` 并比对，任一不符整包置拒绝。

发布执行：在一个 `READ COMMITTED` 事务内按 `sort_no` 升序对每个内容项调用 `validate` 与 `apply`，同一事务内把发布单置 `SUCCEEDED`、配置包置 `RELEASED`，写审计事件与 Outbox 事件 `platform.config_release.released.v1`。回退按 `sort_no` 逆序调用 `revert`，以 `before_spec` 恢复，同样单事务。任一 applier 的 `requires_derived_store_rebuild` 为真时，本阶段只把该判定结果写入事件载荷，派生存储重建的传播段由阶段 13b 实现。

本阶段不交付自动测试编排、编辑锁、停机窗口排队与在线 DDL，这四项与十一态生命周期一并由阶段 13b 扩展。

---

### 3.5 API 契约

全部端点前缀 `/api/v1/platform`，门户侧为 `/api/v1/portal/...` 由 portal-gateway 转发。请求头集合、封套、分页、排序、过滤、幂等键、版本化一律按基线第 5 章，本节不重复，只给各端点的差异项。

权限项名称形如 `platform.<resource>.<action>`，判定由阶段 4 的 ep-platform-authz 承担；本阶段负责在每个端点上声明所需权限项并注册到权限项目录。

#### 3.5.1 通知

| 方法与路径 | 说明 | 幂等 | 权限项 | 主要错误码 |
|---|---|---|---|---|
| GET /api/v1/platform/notifications | 列表，过滤 `is_read`、`notice_type`、`severity`、`created_at` between；排序白名单 `created_at`、`severity`；默认 `created_at desc, id desc` | 不适用 | platform.notification.read | — |
| GET /api/v1/platform/notifications/unread-count | 返回 `{ "unread": n }` | 不适用 | platform.notification.read | — |
| GET /api/v1/platform/notifications/{id} | 详情 | 不适用 | platform.notification.read | PLATFORM.AUTHZ.NOT_FOUND_OR_DENIED |
| POST /api/v1/platform/notifications/{id}/actions/mark-read | 标记已读，重复调用返回同一结果 | 必填 | platform.notification.read | PLATFORM.AUTHZ.NOT_FOUND_OR_DENIED |
| POST /api/v1/platform/notifications/actions/mark-read-batch | 批量，单次上限 200 | 必填 | platform.notification.read | VALIDATION |
| POST /api/v1/platform/push-registrations | 登记推送令牌，请求体 `{device_id, platform, token}` | 必填 | platform.push_registration.write | PLATFORM.PUSH_REGISTRATION.TOKEN_INVALID |
| POST /api/v1/platform/push-registrations/{id}/actions/deactivate | 停用登记 | 必填 | platform.push_registration.write | — |

列表只返回当前用户自己的通知；`recipient_user_id` 不接受作为过滤参数，避免用它探测他人的提醒事项存在性。

#### 3.5.2 附件

| 方法与路径 | 说明 | 幂等 | 权限项 |
|---|---|---|---|
| POST /api/v1/platform/attachments/actions/init-upload | 请求 `{attachment_object_id?, display_name, purpose, security_level, declared_size_bytes, declared_content_type, declared_content_hash}`；响应 `{session_id, part_size_bytes, part_count, expires_at, uploaded_parts:[]}` | 必填 | platform.attachment.write |
| GET /api/v1/platform/attachments/uploads/{session_id} | 断点续传查询，返回已收分片清单 | 不适用 | platform.attachment.write |
| PUT /api/v1/platform/attachments/uploads/{session_id}/parts/{part_no} | 上传单个分片，`Content-Type: application/octet-stream`，请求头 `X-Part-Sha256` | 见下 | platform.attachment.write |
| POST /api/v1/platform/attachments/uploads/{session_id}/actions/complete | 触发组装、检查与发布；同步等待超过 8 秒即转后台任务并返回任务回执 | 必填 | platform.attachment.write |
| POST /api/v1/platform/attachments/uploads/{session_id}/actions/abort | 中止会话 | 必填 | platform.attachment.write |
| GET /api/v1/platform/attachments/{id} | 对象详情含版本清单摘要 | 不适用 | platform.attachment.read |
| GET /api/v1/platform/attachments/{id}/versions | 版本列表 | 不适用 | platform.attachment.read |
| GET /api/v1/platform/attachments/{id}/versions/{version_no}/content | 下载正文，支持 `Range`，`Content-Type: application/octet-stream` | 不适用 | platform.attachment.read |
| POST /api/v1/platform/attachments/{id}/actions/mark-deleted | 写删除标记 | 必填 | platform.attachment.delete_mark |

分片 PUT 的幂等语义单列：`Idempotency-Key` 仍必填以满足基线第 5.4 节，但服务端不为分片写 `platform_msg.idempotency_keys` 行，改以 `(session_id, part_no, part_hash)` 作为自然幂等键判等。理由是 5 GB 文件按 8 MiB 分片是 640 次写请求，为每次写一行幂等键会在单次上传中产生 640 行、7 天保留期内的无谓膨胀，且分片本身已具备可判等的自然键。见第 3.12 节偏离项三。

门户侧三个端点由 core-server 承载并由 portal-gateway 转发：`POST /api/v1/portal/attachments/actions/init-upload`、`PUT /api/v1/portal/attachments/uploads/{session_id}/parts/{part_no}`、`POST /api/v1/portal/attachments/uploads/{session_id}/actions/complete`，权限项为 `portal.attachment.write`，安全级别上限受门户脱敏投影约束。附录 A.1 的门户提交“发票上传”通过线只覆盖到提交回执可见，正文传输按规格第 6.5 章的大文件通道单独记录，本阶段的度量口径按此拆分。

#### 3.5.3 审计

| 方法与路径 | 说明 | 幂等 | 权限项 |
|---|---|---|---|
| GET /api/v1/platform/audit-events | 查询，法人与时间范围必填；可选 `actor_user_id`、`actor_device_id`、`action`、`object_type`、`object_id`、`correlation_id`；跨度上限 366 天；深偏移改键集分页 | 不适用 | platform.audit_event.read |
| GET /api/v1/platform/audit-events/{id} | 详情，含变更前后值与审批过程；字段级裁剪；高风险操作行另含认证方式、待签内容摘要、认证时间与设备 | 不适用 | platform.audit_event.read |
| GET /api/v1/platform/audit-segments | 段列表与锚定状态，含最近一次成功锚定时间 | 不适用 | platform.audit_segment.read |
| POST /api/v1/platform/audit-chain-verifications | 发起验证，请求 `{range_from, range_to}` 或 `{event_id}`，返回任务回执 | 必填 | platform.audit_verification.execute |
| GET /api/v1/platform/audit-chain-verifications/{id} | 验证报告 | 不适用 | platform.audit_verification.read |

验证报告的响应体固定含 `anchor_age_seconds` 与 `anchor_age_alert` 两项，对应 PRD 第 10.6.2 节“最近一次成功锚定时间超过约定间隔时，验证报告需同时呈现该状态”。报告与界面文案不得使用等效或已满足一类措辞，文案表中对应条目固定为“应用级不可变，与经认证的 WORM 后端不等价”。

#### 3.5.4 Outbox 与死信

| 方法与路径 | 说明 | 幂等 | 权限项 |
|---|---|---|---|
| GET /api/v1/platform/outbox-events | 只读积压查询，过滤 `status`、`event_type`、`accounting_period_id`、`posting_date` | 不适用 | platform.outbox_event.read |
| GET /api/v1/platform/dead-letters | 死信列表，过滤 `state`、`failure_category`、`accounting_period_id`、`posting_date` | 不适用 | platform.dead_letter.read |
| GET /api/v1/platform/dead-letters/{id} | 详情，含原信封与载荷、失败历史 | 不适用 | platform.dead_letter.read |
| POST /api/v1/platform/dead-letters/{id}/actions/start-repair | OPEN 到 REPAIRING，记名 | 必填 | platform.dead_letter.repair |
| POST /api/v1/platform/dead-letters/{id}/actions/replay | 重投，记名并写审计 | 必填 | platform.dead_letter.repair |
| POST /api/v1/platform/dead-letters/{id}/actions/discard | 丢弃，请求体必带 `approval_ref` | 必填 | platform.dead_letter.discard |

丢弃需要双人审批，基线第 6.2 节明确。审批本身由流程引擎的审批任务承载，`approval_ref` 指向已完成的 `process_tasks`；审批未完成或审批人与发起人相同时返回 `PLATFORM.DEAD_LETTER.DISCARD_APPROVAL_REQUIRED`。

#### 3.5.5 流程

| 方法与路径 | 说明 | 幂等 | 权限项 |
|---|---|---|---|
| GET /api/v1/platform/process-definitions | 已发布定义列表 | 不适用 | platform.process_definition.read |
| GET /api/v1/platform/process-instances | 实例列表，过滤 `state`、`definition_code`、`business_object_type`、`business_object_id` | 不适用 | platform.process_instance.read |
| GET /api/v1/platform/process-instances/{id} | 实例详情含变量与当前节点 | 不适用 | platform.process_instance.read |
| GET /api/v1/platform/process-instances/{id}/steps | 执行轨迹与补偿轨迹，按 `step_no` 升序 | 不适用 | platform.process_instance.read |
| POST /api/v1/platform/process-instances/{id}/actions/cancel | 取消实例 | 必填 | platform.process_instance.cancel |
| POST /api/v1/platform/process-instances/{id}/actions/resume | 从 MANUAL_INTERVENTION 恢复 | 必填 | platform.process_instance.intervene |
| POST /api/v1/platform/process-instances/actions/migrate-version-batch | 版本迁移，`dry_run` 可选，单次上限 200 | 必填 | platform.process_instance.migrate |
| GET /api/v1/platform/process-tasks | 待办列表，默认过滤当前用户为受理人或候选角色成员 | 不适用 | platform.process_task.read |
| POST /api/v1/platform/process-tasks/{id}/actions/claim | 认领 | 必填 | platform.process_task.claim |
| POST /api/v1/platform/process-tasks/{id}/actions/complete | 完成，请求体含 `decision` 与 `decision_reason`；高风险节点必带 `X-Reauth-Token` | 必填 | platform.process_task.complete |
| POST /api/v1/platform/process-tasks/{id}/actions/reassign | 改派，记名并写审计 | 必填 | platform.process_task.reassign |

`complete` 的守卫条件三项：任务处于 `PENDING` 或 `CLAIMED`；当前用户是受理人或候选角色成员，否则 `PLATFORM.PROCESS_TASK.NOT_ASSIGNED`；当前用户不是 `initiator_user_id`，否则 `PLATFORM.PROCESS_TASK.SELF_APPROVAL_FORBIDDEN`。第三项对应规格第 12.2 章“申请人不可自审”，判定输入由本阶段提供，判定实现由授权层执行。

`GET /api/v1/platform/process-tasks` 对应附录 A.1 常规交互清单中的“审批任务列表加载”，通过线 P95 不超过 2 秒。

#### 3.5.6 编号

无对外写端点，编号只能由用例内部经 `NumberAllocator` 分配。提供一个只读端点 `GET /api/v1/platform/number-sequences` 供运维核对当前流水位置，权限项 `platform.number_sequence.read`。

#### 3.5.7 全文检索（3b 段）

| 方法与路径 | 说明 | 幂等 | 权限项 |
|---|---|---|---|
| GET /api/v1/platform/search | 关键字检索，查询参数 `keyword`、`object_types`（可重复）、`page`、`page_size`，响应为 `SearchHit` 列表与总数 | 不适用 | platform.search.query |

`legal_entity_id` 取自安全上下文，`max_security_level` 取自 `SecurityContext.clearance_level`，两者都不接受传参，避免用参数越权扩大检索面。附录 A.1 的“全文检索返回首页结果”度量项落在该端点，其通过线在真实档案数据上由阶段 5 判定，本阶段只在合成文档上记录基线值。

#### 3.5.8 配置发布（3b 段）

端点路径与权限项照抄阶段 13 计划第 5 节，本阶段只实现六态所需的子集，不新增路径。

| 方法与路径 | 说明 | 幂等 | 权限项 |
|---|---|---|---|
| POST /api/v1/platform/config-packages | 就地创建配置包，`source` 取 `IN_PLACE` | 必填 | lowcode.config_package.create |
| POST /api/v1/platform/config-packages/actions/import | 导入并验签，同一 `content_hash` 重复导入返回既有包 | 必填 | lowcode.config_package.import |
| GET /api/v1/platform/config-packages/{id} | 包详情含内容项摘要 | 不适用 | lowcode.config_package.view |
| GET /api/v1/platform/config-packages/{id}/diff | 与指定包的差异，供提交审批前的人工差异审查 | 不适用 | lowcode.config_package.view |
| POST /api/v1/platform/config-packages/{id}/actions/submit-for-approval | Draft 到 PendingApproval | 必填 | lowcode.config_package.submit |
| POST /api/v1/platform/config-packages/{id}/actions/approve | PendingApproval 到 Approved，审批人不得等于提交人 | 必填 | lowcode.config_package.approve |
| POST /api/v1/platform/config-packages/{id}/actions/reject | PendingApproval 到 Rejected | 必填 | lowcode.config_package.approve |
| POST /api/v1/platform/config-packages/{id}/actions/sign | 签名，已签名重复调用返回既有签名 | 必填 | lowcode.config_package.sign |
| POST /api/v1/platform/config-release-orders | 建发布单或回退单 | 必填 | lowcode.config_release.submit |
| POST /api/v1/platform/config-release-orders/{id}/actions/execute | 执行，重复执行返回同一回执 | 必填 | lowcode.config_release.execute |
| GET /api/v1/platform/config-release-orders/{id} | 发布单详情含逐项落地结果 | 不适用 | lowcode.config_release.view |

`actions/run-autotest`、`config-edit-locks` 与停机窗口排队三组端点属阶段 13b，本阶段不实现。

#### 3.5.9 模块许可（3b 段）

本阶段不新增对外端点，`ModuleLicenseQuery` 是唯一取用入口。许可凭证与模块安装态在本阶段经迁移种子写入，运行期的许可导入与模块启停入口由阶段 13b 与阶段 14 承载，本阶段不预先占用路径。

---

### 3.6 并发与事务边界

#### 3.6.1 事务清单

| 事务 | 隔离级别 | 内含写入 | 锁 | 预算 |
|---|---|---|---|---|
| 业务命令事务（core-server 用例） | READ COMMITTED | 业务表、取号行、Outbox、站内通知、幂等键、审计事件与段行 | 序列行排他锁、审计段行排他锁、聚合行乐观锁 | 5 秒；`statement_timeout` 10 秒，`lock_timeout` 3 秒 |
| Outbox 取件事务 | READ COMMITTED | `outbox_events` 的 status/locked 列 | `FOR UPDATE SKIP LOCKED` | 1 秒 |
| Outbox 消费事务 | READ COMMITTED | 处理器副作用、`inbox_consumptions`、`outbox_events` 置 DONE | 唯一约束 | 按处理器，上限 30 秒 |
| 锚定阶段 A | READ COMMITTED | `audit_anchors` 插入、段行 last_anchor_seq | 段行排他锁 | 1 秒 |
| 锚定阶段 B/C | 无事务或短事务 | `audit_anchors` 更新 | 条件更新 | KMS 与磁盘超时各 10 秒 |
| 链验证 | REPEATABLE READ | 只读，`audit_verifications` 更新在独立短事务 | 无 | 单段上限 5 分钟 |
| 流程单步事务 | READ COMMITTED | 实例行、`process_steps`、任务或定时器、Outbox、审计 | 实例行 `FOR UPDATE` | 5 秒 |
| 定时器触发事务 | READ COMMITTED | `process_timers` 状态、实例 `next_wake_at` | `FOR UPDATE SKIP LOCKED` | 1 秒 |
| 补偿单条事务 | READ COMMITTED | `process_compensations`、实例状态、审计 | 实例行 `FOR UPDATE` | 5 秒 |
| 附件事务 A | READ COMMITTED | `attachment_versions` PENDING 行 | 无 | 1 秒 |
| 附件事务 B | READ COMMITTED | 版本置 AVAILABLE、对象版本号、Outbox、审计 | 对象行乐观锁 | 2 秒 |
| 保留期清理事务 | READ COMMITTED | 按批 DELETE，单批不超过 1000 行 | 无 | 30 秒 |
| 配置发布段二事务（3b 段） | READ COMMITTED | 按 `sort_no` 升序的 applier `apply`、发布单与配置包状态、审计、Outbox | 发布互斥以 `config_release_orders` 上 `status = 'EXECUTING'` 的存在性判定 | 30 秒 |

只读分析池的 `statement_timeout` 60 秒、`work_mem` 64 MB、`temp_file_limit` 2 GB，job-worker 池 `statement_timeout` 300 秒，ops 池 5 秒，取值全部按基线第 10.3 节，本阶段不改。

#### 3.6.2 锁顺序与死锁防范

本阶段引入三类会被多个事务同时争用的行：编号序列行、审计段行、流程实例行。统一加锁顺序为：业务聚合行，编号序列行，流程实例行，审计段行。审计段行排在最后，与判定二一致。跨自然日的多段写入按 `event_day` 升序加锁。该顺序写入 `ep-adapter-db` 的工作单元文档并由代码评审逐条核对；违反顺序的代码在集成测试的死锁注入用例中会被检出。

#### 3.6.3 与 Outbox 的关系

业务状态、审计事件与 Outbox 条目写入同一数据库事务，基线第 6.2 节与规格第 15.2 章共同要求，本阶段无例外。事务提交前不发起任何外部调用，包括 KMS 签名与文件写入。流程实例状态首版取“同事务写入”一路，规格第 9.1 章允许的“由 Outbox 保证最终一致”一路保留为配置开关 `flow.state_persistence`（取值 `same_transaction`、`outbox_eventual`），默认 `same_transaction`；两路的契约测试都要跑，因为规格第 19 章阶段 1 契约冻结条目要求“契约测试必须覆盖流程实例状态的两条路径，并在每条路径下验证业务状态、审计与 Outbox 仍在同一事务内提交”。

#### 3.6.4 必测的六组并发场景在本阶段的对应

基线第 8.4 节列出六组必测并发场景，其中四组要到业务模块阶段才能跑。本阶段承担并必须通过的是：同一单据的乐观锁冲突（以合成聚合表达）、Outbox 同一事件的重复投递不少于 3 次。另加本阶段特有的五组：同一法人同一自然日的并发审计追加不产生链分叉；同一编号序列的并发取号不产生重号与空号；同一流程实例的并发推进只有一条生效；同一上传会话的并发分片写入与重传；同一死信的并发重投只生效一次。

#### 3.6.5 失败重试与补偿

三类失败的处理路径互不混用。数据库瞬时冲突走工作单元重试，3 次退避 50、150、450 毫秒。Outbox 投递失败走八段退避后转死信，死信只能人工处置。流程步骤失败走步骤退避（5 次，1、5、30、120、600 秒）后进补偿，补偿失败进人工任务。三条路径都不会静默丢弃，符合规格第 15.2 章“财务不平、库存不守恒或金额不一致时禁止静默忽略”的同类要求。

---

### 3.7 配置项

全部配置在启动时读取并生效，改动需重启对应进程。唯一例外是机密引用的版本变更，按基线第 7.2 节在下次取用时使用新版本，不需重启。全部结构体开启 `deny_unknown_fields`。

| 键 | 类型 | 默认值 | 生效进程 |
|---|---|---|---|
| EP__PLATFORM__SEQUENCE__DEFAULT_WIDTH | u8 | 6 | core-server |
| EP__PLATFORM__SEQUENCE__MAX_WIDTH | u8 | 12 | core-server |
| EP__PLATFORM__SEQUENCE__LOCK_TIMEOUT_MS | u32 | 3000 | core-server |
| EP__PLATFORM__IDEMPOTENCY__RETENTION_DAYS | u16 | 7 | core-server, job-worker |
| EP__PLATFORM__OUTBOX__BATCH_SIZE | u16 | 100 | job-worker |
| EP__PLATFORM__OUTBOX__POLL_INTERVAL_MS | u32 | 200 | job-worker |
| EP__PLATFORM__OUTBOX__IDLE_BACKOFF_MS | u32 | 2000 | job-worker |
| EP__PLATFORM__OUTBOX__MAX_ATTEMPTS | u8 | 8 | job-worker |
| EP__PLATFORM__OUTBOX__RETRY_BACKOFF_SECONDS | Vec\<u32\> | [1,5,30,120,600,1800,3600,7200] | job-worker |
| EP__PLATFORM__OUTBOX__DISPATCH_CONCURRENCY | u8 | 4 | job-worker |
| EP__PLATFORM__OUTBOX__LOCK_LEASE_SECONDS | u32 | 60 | job-worker |
| EP__PLATFORM__OUTBOX__DONE_RETENTION_DAYS | u16 | 30 | job-worker |
| EP__PLATFORM__OUTBOX__INBOX_RETENTION_DAYS | u16 | 60 | job-worker |
| EP__PLATFORM__AUDIT__ANCHOR_INTERVAL_SECONDS | u32 | 300 | job-worker |
| EP__PLATFORM__AUDIT__ANCHOR_EVENT_THRESHOLD | u32 | 1000 | job-worker |
| EP__PLATFORM__AUDIT__ANCHOR_SCAN_INTERVAL_SECONDS | u32 | 30 | job-worker |
| EP__PLATFORM__AUDIT__ANCHOR_AGE_ALERT_SECONDS | u32 | 900 | job-worker, ops-agent |
| EP__PLATFORM__AUDIT__SEGMENT_LOCK_TIMEOUT_MS | u32 | 3000 | core-server |
| EP__PLATFORM__AUDIT__SIGNATURE_ALGORITHM | enum | ECDSA_P256_SHA256 | job-worker |
| EP__PLATFORM__AUDIT__SIGNING_KEY_REF | string | secret://audit/segment_signing#1 | job-worker |
| EP__PLATFORM__AUDIT__EVIDENCE_DIR | path | /var/lib/ep/audit-evidence | job-worker |
| EP__PLATFORM__AUDIT__VERIFY_MAX_DAYS | u16 | 92 | core-server, job-worker |
| EP__PLATFORM__AUDIT__QUERY_MAX_DAYS | u16 | 366 | core-server |
| EP__PLATFORM__FILE__ROOT_DIR | path | /var/lib/ep/files/published | core-server, job-worker |
| EP__PLATFORM__FILE__STAGING_DIR | path | /var/lib/ep/files/staging | core-server |
| EP__PLATFORM__FILE__MAX_OBJECT_BYTES | u64 | 5368709120 | core-server |
| EP__PLATFORM__FILE__PART_BYTES | u32 | 8388608 | core-server |
| EP__PLATFORM__FILE__SESSION_TTL_HOURS | u16 | 24 | core-server, job-worker |
| EP__PLATFORM__FILE__MAX_CONCURRENT_UPLOADS_PER_USER | u8 | 3 | core-server |
| EP__PLATFORM__FILE__MAX_CONCURRENT_UPLOADS_GLOBAL | u8 | 6 | core-server |
| EP__PLATFORM__FILE__UPLOAD_BANDWIDTH_BYTES_PER_SEC | u64 | 52428800 | core-server |
| EP__PLATFORM__FILE__DOWNLOAD_BANDWIDTH_BYTES_PER_SEC | u64 | 52428800 | core-server |
| EP__PLATFORM__FILE__FREE_SPACE_MIN_BYTES | u64 | 107374182400 | core-server |
| EP__PLATFORM__FILE__SCAN__MODE | enum | builtin_only | core-server |
| EP__PLATFORM__FILE__SCAN__CLAMD_SOCKET | path | /run/clamav/clamd.ctl | core-server |
| EP__PLATFORM__FILE__SCAN__TIMEOUT_SECONDS | u32 | 120 | core-server |
| EP__PLATFORM__FILE__SCAN__MAX_ARCHIVE_RATIO | u32 | 200 | core-server |
| EP__PLATFORM__FILE__SCAN__MAX_ARCHIVE_DEPTH | u8 | 4 | core-server |
| EP__PLATFORM__FILE__QUARANTINE_RETENTION_DAYS | u16 | 90 | job-worker |
| EP__PLATFORM__NOTIFY__RETENTION_DAYS | u16 | 180 | job-worker |
| EP__PLATFORM__NOTIFY__UNREAD_CAP_PER_USER | u32 | 2000 | core-server |
| EP__PLATFORM__NOTIFY__SYNC_FANOUT_MAX | u16 | 200 | core-server |
| EP__PLATFORM__NOTIFY__PUSH_ENABLED | bool | false | core-server, job-worker |
| EP__PLATFORM__NOTIFY__PUSH_ENDPOINT | url | http://127.0.0.1:8082/internal/v1/push/dispatch | job-worker |
| EP__PLATFORM__NOTIFY__PUSH_TIMEOUT_MS | u32 | 5000 | job-worker |
| EP__PLATFORM__NOTIFY__PUSH_MAX_ATTEMPTS | u8 | 3 | job-worker |
| EP__PLATFORM__NOTIFY__PUSH_BODY_INCLUDES_BUSINESS_FIELDS | bool | false | job-worker |
| EP__PLATFORM__NOTIFY__PUSH_DEACTIVATE_AFTER_FAILURES | u8 | 10 | integration-gateway |
| EP__PLATFORM__FLOW__STATE_PERSISTENCE | enum | same_transaction | core-server, job-worker |
| EP__PLATFORM__FLOW__SCHEDULER_INTERVAL_MS | u32 | 200 | job-worker |
| EP__PLATFORM__FLOW__EXECUTOR_CONCURRENCY | u8 | 4 | job-worker |
| EP__PLATFORM__FLOW__BATCH_SIZE | u16 | 20 | job-worker |
| EP__PLATFORM__FLOW__TIMER_SCAN_BATCH | u16 | 50 | job-worker |
| EP__PLATFORM__FLOW__MAX_INSTANCE_DURATION_DAYS | u16 | 365 | core-server, job-worker |
| EP__PLATFORM__FLOW__MAX_STEPS_PER_INSTANCE | u16 | 500 | job-worker |
| EP__PLATFORM__FLOW__MAX_PARALLEL_BRANCHES | u8 | 16 | job-worker |
| EP__PLATFORM__FLOW__STEP_MAX_ATTEMPTS | u8 | 5 | job-worker |
| EP__PLATFORM__FLOW__STEP_RETRY_BACKOFF_SECONDS | Vec\<u32\> | [1,5,30,120,600] | job-worker |
| EP__PLATFORM__FLOW__COMPENSATION_MAX_ATTEMPTS | u8 | 5 | job-worker |
| EP__PLATFORM__FLOW__INSTANCE_RETENTION_DAYS | u16 | 730 | job-worker |
| EP__PLATFORM__FLOW__EXPRESSION_MAX_STEPS | u16 | 1000 | core-server, job-worker |
| EP__PLATFORM__SEARCH__ROOT_DIR | path | /var/lib/ep/search | core-server, job-worker |
| EP__PLATFORM__RETRY__SERIALIZATION_MAX_ATTEMPTS | u8 | 3 | 全部持库进程 |
| EP__PLATFORM__RETRY__SERIALIZATION_BACKOFF_MS | Vec\<u32\> | [50,150,450] | 全部持库进程 |
| EP__PLATFORM__RETRY__CIRCUIT_FAILURE_THRESHOLD | u8 | 5 | job-worker, integration-gateway |
| EP__PLATFORM__RETRY__CIRCUIT_OPEN_SECONDS | u32 | 30 | job-worker, integration-gateway |
| EP__PLATFORM__RETRY__CIRCUIT_HALF_OPEN_PROBES | u8 | 1 | job-worker, integration-gateway |

运行期可变的业务参数不进配置文件，按基线第 7.1 节：提醒规则的触发对象、触发日期字段、提前量、重复策略与接收人解析方式，通知模板的标题与正文，流程定义，全部存事务数据库并经配置发布通道签名发布。发布通道按裁定 A-27 由本阶段 3b 段交付，见第 3.4.12 节；流程定义与提醒规则、通知模板的落地分别经 `FlowDefinitionApplier` 与 `NotifyRuleApplier`。

启动自检增量：按裁定 C-25，自检项一律以注册名标识，不用序号，注册表为阶段 1 的 `SelfCheckRegistry`，报告按注册顺序输出且基线十三项在前。本阶段在基线第 7.3 节的十三个命名项之外追加四项，登记为本阶段新增决定：`audit-evidence-store-writable`，审计证据存储目录可写且不具备覆盖与删除权限，剩余空间不低于阈值；`audit-signing-key-usable`，签名私钥可解引用且可完成一次自签自验；`attachment-store-ready`，附件存储根目录、staging 目录与检索索引根目录存在、权限位正确、剩余空间不低于 `FREE_SPACE_MIN_BYTES`；`event-catalog-consistent`，事件目录中登记的事件类型与代码中注册的处理器无缺漏无多余。四项任一失败以退出码 78 退出，`--check` 模式一并执行这四项。另外，基线十三项中的 `license-and-modules-consistent` 由阶段 1 注册为 Pending，本阶段 3b 段换成实现，判据见第 3.4.11 节。

---

### 3.8 测试计划

#### 3.8.1 单元测试覆盖的分支

编号：位数扩展的临界（999999 到 1000000）、类型码校验的四个长度边界、档案类与单据类的 `period_key` 取值、人工指定的拒绝路径、格式化补零、解析回读的往返一致。

审计：段首条的全零前序哈希、多条同事务事件的链式串接、跨自然日分组与升序加锁的顺序生成、JCS 规范化对键序与数值表示的稳定性（含 `1.10` 与 `1.1` 在字符串承载下可区分的断言）、`seq` 空洞不判为断裂、锚定触发条件的两个分支（时间到与条数到）、验证器的五类失败定位。

Outbox：信封必填项的逐字段缺失、`security_level` 与 `data_scope_tags` 缺失的拒绝、八段退避序列的逐次取值、第 8 次失败转死信、`DISPATCHING` 租约过期回收、幂等键三态判定的九种组合（三种 `state` 乘三种 `request_hash` 关系）。

文件：上传会话状态机的全部合法迁移与全部非法迁移、分片重传的哈希一致与不一致、总哈希不符的拒绝、三个内置检查器各自的 PASS 与 REJECT 分支、归档炸弹的比例与深度两个上限、三段式落盘在四个崩溃点上的收敛判定。

通知：模板变量白名单外的变量拒绝渲染、无权字段的替代文案、`dedupe_key` 的去重、未读上限触发时新通知仍写入、推送载荷在默认配置下不含业务字段。

流程：实例状态机的全部合法迁移与全部非法迁移、步骤幂等键的构造、守卫条件求值器的算子矩阵与步数上限、补偿逆序的顺序断言、三项运行约束各自的超限分支、版本迁移的可迁与不可迁判定。

错误：五类分类到 HTTP 与 `retryable` 的映射矩阵、`incident_no` 的正常与退化两条生成路径、不可见记录统一返回 404 的四个端点。

领域属性测试（proptest），本阶段登记四组不变量。其一，对任意事件序列，链追加后按 `seq` 升序重算哈希恒等于存储值。其二，对任意并发取号序列，产出的编号集合无重复且无空号。其三，对任意投递与崩溃序列，每个事件的副作用恰好发生一次。其四，对任意步骤与补偿序列，补偿执行顺序恒为已完成步骤的严格逆序。这四组是规格第 17.2 章“领域属性测试”在本阶段的落点；基线第 8.1 节要求的借贷平衡、库存守恒、核销守恒、移动加权平均、价差拆分五组属业务模块阶段，本阶段不承担。

#### 3.8.2 集成测试场景清单

全部用真实 PostgreSQL 16，每用例独占一库，库名 `ep_test_<nanoid>`，用例结束即删库。合成聚合建在测试库内的临时 schema `test_synth`，由测试自行创建，不进 `db/migrations`，因此不违反“任何阶段不得新增 schema”。

行级安全，独立测试目标 `tests/rls_matrix` 的本阶段部分：对本阶段 24 张带 `legal_entity_id` 的表逐表调用阶段 1 提供的八个断言函数 `assert_read`、`assert_write`、`assert_update`、`assert_delete`、`assert_aggregate`、`assert_sort`、`assert_report_projection`、`assert_error_leak`，另覆盖会话变量缺失时的默认拒绝、会话归还后变量已清空、跨法人查询按法人逐轮而非 `OR` 展开三项；对六张部署级表断言其不建策略且可见性不随 `app.legal_entity_id` 变化。按裁定 C-05，断言函数骨架由阶段 1 提供、数据库侧策略断言与复制角色入口借用由阶段 2 提供、32 组完整矩阵与发布门禁项 `RG-RLS-MATRIX-GREEN` 由阶段 4 提供，本阶段只调用，不实现同名函数，也不承担该门禁项的判定。

Outbox 可靠投递，对应规格第 7.3 章必含项：至少一次投递、重复投递去重（同一事件强制投递 5 次）、崩溃恢复后不丢不重（在取件后投递前、投递后写 `inbox_consumptions` 前、写后置 `DONE` 前三个点终止 job-worker）。

审计链，对应规格第 17.2 章“审计链与不可变存储测试”：20 并发写入 5000 条事件后链验证全通过；跨自然日边界的段切换；锚定在 KMS 不可用时的重试与恢复；证据文件被外部改写后验证报告定位到该锚定；证据路径的覆盖与删除尝试被应用层拒绝并写审计；`audit_events` 上的 `UPDATE` 与 `DELETE` 被 CI 的 SQL 静态检查与数据库权限双重拒绝。

流程引擎认证套件的阶段必过项，对应规格第 17.2 章：崩溃恢复（在步骤执行前、执行中、提交后、补偿过程中随机终止 job-worker 各不少于 20 次）；重复投递不少于 3 次时业务效果、外发事件与审计记录只产生一次；定时器幂等与可重放（进程重启、模拟升级重启、从备份恢复三种重放场景下不漏触发不重复触发）；流程定义版本升级（运行中实例继续用旧版本，新实例用新版本，显式迁移与回退各一次）；补偿正确性（逆序、幂等重试、部分失败后进人工任务队列并告警，用合成聚合与合成不变量验证）。

附件：5 GB 文件的分片上传与断点续传（中断在 30%、60%、95% 三点）；并发上限触发；总哈希不符拒绝；检查未通过进隔离；三段式落盘在四个崩溃点的收敛；同一内容两次上传产生两个独立物理副本（规格第 6.5 章明确不做去重）；按法人密钥域与密级子域产生不同密文；下载的 `Range` 请求。

通知：站内通知在业务事务回滚时一并回滚；推送出口不可用时站内通知照常送达；接收人对关联单据无权限时正文不含无权字段；接收人已停用时按 PRD 第 10.5.5 节需重新指派，本阶段实现为写入 `RECIPIENT_INACTIVE` 标记并进人工任务队列，指派方式待决。

死信：转死信、重投成功、重投再失败、丢弃需双人审批、丢弃时申请人不可自审、按法人与会计期间的可枚举统计。

混沌与故障注入，五类：依赖服务超时（KMS 与 clamd 各注入超时）、连接池与内存资源耗尽（把读写池打满并验证 job-worker 池不被挤占）、消息积压（灌入 50 万条 Outbox 条目并验证取件不退化为顺序扫描）、磁盘写满（附件写入与证据写出各触发一次并验证按 `INFRASTRUCTURE` 返回且不产生半截元数据）、进程崩溃后重启恢复（core-server 与 job-worker 各强制终止 20 次）。预期行为一律为按规格第 15.1 章返回可重试或明确失败、不产生数据不一致、故障移除后 5 分钟内自愈。

契约测试：流程实例状态的 `same_transaction` 与 `outbox_eventual` 两条路径，各自验证业务状态、审计与 Outbox 仍在同一事务内提交。

许可与配置发布（3b 段）：模块由 `INSTALLED_ENABLED` 置 `INSTALLED_DISABLED` 后其定时器不再触发、其事件不再投递且条目保持 `PENDING` 不累加 `attempts`，再启用后自动恢复；许可过期与吊销各触发一次 `license-and-modules-consistent` 自检失败；配置包六态的全部合法与非法迁移；签名被篡改与任一项 `item_hash` 不符时整包拒绝；未在 `ConfigItemApplierRegistry` 注册的 `item_kind` 整包拒绝；发布后回退按 `sort_no` 逆序以 `before_spec` 恢复，`FlowDefinitionApplier` 与 `NotifyRuleApplier` 各跑一次发布与一次回退。

全文检索（3b 段）：索引写入只在 job-worker 的消费者中发生，core-server 的用例路径上不出现 `SearchIndexPort` 调用且由 `xtask archcheck` 判负；同一文档重复投递只产生一份索引条目；删除事件后查询不再命中；两个法人的索引分区互不可见；查询按 `SecurityContext.clearance_level` 过滤后仍对命中结果做数据库侧可见性复核。

#### 3.8.3 端到端用例

本阶段无业务模块，端到端用例跑在合成模块上，共五条。

E2E-1 合成单据全链路：创建合成单据（取号、写审计、写 Outbox、写站内通知）、启动审批流程、审批人认领并完成（带重新认证令牌）、流程完成、Outbox 消费产生下游合成效果、审计链验证通过。覆盖本阶段七组能力中的六组。

E2E-2 附件全链路：init-upload、分片上传含一次中断续传、complete、检查通过、下载、新增第二个版本、旧版本物理文件仍存在、写删除标记后不可下载但历史审计仍可查。

E2E-3 补偿全链路：三步流程在第三步失败，逆序补偿前两步，第一步补偿重试耗尽，实例进 `MANUAL_INTERVENTION`，人工任务生成，站内通知送达，人工处置后实例取消，全过程审计轨迹可按实例查询完整执行轨迹与补偿轨迹。

E2E-4 定时器与提醒：配置一条提醒规则，定时器触发产生站内通知，进程重启后不重复触发，从备份恢复后不漏触发。

E2E-5 死信与人工修复：注入一个恒失败的消费处理器，事件走完八段退避进死信，站内通知送达责任人，重投仍失败，双人审批后丢弃，全过程写审计。

E2E-6 配置发布最小通道（3b 段）：创建含一个 `FLOW_DEFINITION` 项与一个 `NOTIFY_RULE` 项的配置包，经差异审查、审批、签名、执行发布，`platform_flow.process_definitions` 与 `platform_msg.notification_templates` 各新增一个已发布版本，`platform.config_release.released.v1` 写出，站内通知送达配置管理员；随后回退该发布单，两个定义按 `before_spec` 恢复，全过程审计轨迹完整。

四端 UI 层面的验证不在本阶段，本阶段只跑后端 E2E（Rust 集成测试直接打 HTTP 接口）。基线第 8.1 节要求的 Playwright、tauri-driver、XCUITest 与 Espresso 覆盖属客户端阶段。

#### 3.8.4 性能相关项

| 度量项 | 来源 | 通过线 | 判定方式 |
|---|---|---|---|
| 站内通知列表加载 | 附录 A.1 常规交互 | P95 不超过 2 秒 | 20 并发负载模型，样本不少于 200 |
| 审批任务列表加载 | 附录 A.1 常规交互 | P95 不超过 2 秒 | 同上 |
| 附件列表加载 | 附录 A.1 常规交互 | P95 不超过 2 秒 | 同上 |
| 审计查询首页 | 本阶段观察项 | 无通过线 | 记录 P95 与 P99 |
| 审计写入对业务事务的耗时增量 | 本阶段观察项 | 内部目标 P95 不超过 15 毫秒 | 有无审计两组对照 |
| Outbox 端到端投递延迟 | 本阶段观察项 | 内部目标 P95 不超过 2 秒 | 从写入到 `DONE` |
| 锚定间隔 | 规格第 12.5 章 | 不超过 5 分钟或 1000 条 | `ep_audit_anchor_age_seconds` |
| 附件上传吞吐 | 规格第 6.5 章大文件通道 | 无通过线 | 单独记录 |

`EXPLAIN` 证据：上述三个进入附录 A.1 的查询，以及 Outbox 取件、定时器扫描、死信按会计期间枚举三条语句，合计六条，必须在基准数据集上给出不含顺序扫描的执行计划，基线第 3.10 节要求。

#### 3.8.5 基准数据集扩展

`ep-datagen` 增加五类生成器。规格附录 A.3 未给出本阶段五类对象的规模取值，本阶段按下列口径推算并显式标注为假设：审计事件按“每张单据平均 6 条审计事件”从 10 万销售订单行、10 万采购订单行、150 万会计分录反推，取 400 万条，分布在 2 个法人、36 个会计期间对应的约 1100 天，约 2200 段；Outbox 条目取 200 万条，其中 `DONE` 195 万、`PENDING` 5000、`DEAD` 200；站内通知取 60 万条，其中未读 5000；附件对象沿用附录 A.3 的 10 万个约 800 GB，版本数按 1.2 倍取 12 万个版本；流程实例取 30 万个，其中运行中 2000、人工任务待办 500。假设的理由是附录 A.3 只冻结了业务对象规模，平台对象规模是其派生量，若不给取值则本阶段的索引与执行计划无从判定；该取值随本阶段结束回写附录 A.3 的生成器版本。生成器接受 `--seed` 与 `--scale`，版本化并随认证结论冻结。

#### 3.8.6 覆盖率门槛

本阶段全部代码属规格第 17.2 章意义上的平台内核代码，行覆盖率不低于 85%。新增与修改代码不低于 80%。工作区整体不低于 80%。工具为 cargo-llvm-cov，CI 以 `--fail-under-lines` 强制，分档阈值在 `codecov.toml` 中按 crate 路径表达。`#[ignore]` 必须带 issue 编号且存活不超过本阶段。

---

### 3.9 退出条件

下列 29 项全部达成才算本阶段完成，每项都可由 CI 产物或测试报告客观判定。第 29 项是 3a 段的独立闸门，必须在阶段 4 开工前达成；其余各项在 3b 段结束时判定。

1. 32 个迁移文件在空库上按 `order.toml` 顺序执行成功，每个文件的 `-- rollback:` 段可执行或已注明只能用备份回退；3a 段的第 1 号与 3b 段的第 2 至 32 号在含阶段 4 迁移的合并环境上按版本号单调应用成功。
2. 30 张新表中，24 张带 `legal_entity_id` 的表全部 `ENABLE` 且 `FORCE` 行级安全，策略按统一模板生成，`tests/rls_matrix` 的本阶段部分八类全通过；六张部署级表按裁定 A-05 与 A-27 不带 `legal_entity_id`、不建策略，且已断言其可见性不随 `app.legal_entity_id` 变化。
3. 运行期账号 `ep_app_rw` 在本阶段表上无 DDL、无策略管理权限，`--check` 的 `rls-enabled-and-forced` 与 `runtime-role-privileges-bounded` 两项通过。
4. `--check` 的十七个命名项（基线第 7.3 节十三项加本阶段四项 `audit-evidence-store-writable`、`audit-signing-key-usable`、`attachment-store-ready`、`event-catalog-consistent`）在部署环境上全部通过并输出结构化报告，其中基线项 `license-and-modules-consistent` 已由本阶段 3b 段从 Pending 换成实现。
5. Outbox 可靠投递三组测试全通过，含至少一次投递、重复投递去重、崩溃恢复不丢不重。
6. 幂等键三态判定的九种组合全通过，重复请求回带 `Idempotent-Replay: true`。
7. 编号并发取号 10 万次无重号、无空号，位数扩展临界通过。
8. 审计链：20 并发写入 5000 条后链验证全通过；跨日段切换正确；证据文件外部改写后验证报告定位到该锚定。
9. 锚定：在持续写入负载下 `ep_audit_anchor_age_seconds` 的最大值不超过 900 秒，KMS 中断 5 分钟后自动恢复锚定。
10. 审计证据路径的覆盖与删除尝试被应用层拒绝并写入审计证据，`ep-adapter-file` 的 `published` 与 `evidence` 命名空间在类型层面不暴露删除与覆盖方法（由编译期断言测试证明）。
11. 流程引擎认证套件的五组阶段必过项全部通过，通过结论写入验收证据。规格第 9.1 章的不达标预案未被触发。
12. 附件三段式落盘在四个崩溃点上全部收敛，无孤儿文件、无元数据在而正文不在。
13. 5 GB 单文件分片上传与三点断点续传成功，同一内容两次上传产生两个独立物理副本。
14. 站内通知在推送出口完全不可用的部署形态下照常送达，E2E-1 与 E2E-4 通过。
15. 死信的转入、重投、双人审批丢弃三条路径全通过，按法人与会计期间的枚举查询不走顺序扫描。
16. 五类混沌场景全部通过，故障移除后 5 分钟内自愈，进程崩溃后未完成任务自动恢复且已确认事务零丢失。
17. 六条后端 E2E 全通过。
18. 附录 A.1 的三个本阶段度量项在 20 并发负载模型下 P95 不超过 2 秒，样本各不少于 200，单次运行错误率不超过 0.1%。
19. 六条关键语句的 `EXPLAIN` 证据不含顺序扫描。
20. 覆盖率：本阶段代码行覆盖率不低于 85%，新增与修改代码不低于 80%，工作区整体不低于 80%。
21. `docs/error-codes.md` 的 `PLATFORM` 段、`docs/event-catalog.md` 的 `platform` 段、`ep-foundation::error::codes` 常量表、指标注册表四处由 CI 校验一致，无重复码、无未登记项。
22. 本阶段的四项偏离与十二项新增决定已回写共享技术基线，并经平台架构负责人签字。
23. 模块许可：三张许可表建立，`ModuleLicenseQuery` 可用，模块置 `INSTALLED_DISABLED` 后其定时器停止触发、其事件停止投递且条目不累加 `attempts`，再启用后自动恢复；停用再启用的端到端验收按裁定 A-05 顺延到阶段 13b，本阶段以集成测试判定。
24. 最小配置发布通道：三张配置表建立，六态状态机的全部合法与非法迁移有测试，ECDSA P-256 签名与逐项 `item_hash` 重算校验通过，`FlowDefinitionApplier` 与 `NotifyRuleApplier` 两个 applier 已实现并在两个 wiring 注册，未注册 `ItemKind` 的内容项整包拒绝发布。
25. 全文检索：`SearchIndexPort` 与 `SearchQueryPort` 在 `ep-adapter-search` 上实现，索引按法人分区落在 `/var/lib/ep/search/<legal_entity_id>/`，`xtask archcheck` 断言业务事务路径上不出现索引写调用，两个法人的分区互不可见。
26. 本阶段全部路由的能力域码与动作类别常量已按裁定 A-20 声明，常量落在 `crates/platform/flow/src/capability.rs`，取值取自 `ep_foundation::CapabilityDomain` 与 `ep_foundation::ActionClass`，`/api/v1/platform/` 下的平台路由能力域一律取 `CapabilityDomain::PlatformAdminLowcodeOps`，`xtask configdoc` 通过。
27. `DisposalPort` 的 trait 与两个 DTO 已定义在 `crates/platform/file/src/port/disposal.rs`，两个 wiring 已注入 `NoopDisposalPort` 并标注 `// TODO(stage-14): replace with real impl`，实现由阶段 14 的 `OpsDisposalService` 交付。
28. 附件的幂等收敛任务在四个崩溃点上收敛，且不产生任何对账差异事项、不实现 `ReconCheck`、不依赖 `ep-platform-recon`。
29. 3a 段闸门：`platform_msg.idempotency_keys` 与 `IdempotencyStore` 实现、`crates/platform/release/src/port/config_item.rs` 端口与注册表两项已完成并通过各自单元测试，且该段不引入对 `ep-platform-identity` 与 `ep-platform-authz` 的任何依赖，`cargo metadata` 自检通过。

---

### 3.10 与规格和 PRD 的对应

#### 3.10.1 规格条目

| 规格条目 | 本阶段实现内容 |
|---|---|
| 第 5.1 章 Outbox、幂等、编号、通知、文件引用 | 全部四项能力；通知只交付站内通知与移动推送两条渠道，其余九条渠道不实现 |
| 第 5.1 章 低代码流程、审批、定时器、补偿和 SLA | 持久化流程引擎运行时；表单、规则表达式完整能力与 WASM 计算不在本阶段 |
| 第 5.1 章 审计与变更留痕 | 审计事件模型、哈希链、分段签名、链验证工具、审计查询端点 |
| 第 6.5 章 大文件 | 单文件上限 5 GB、分片上传、断点续传、带宽限制、法人密钥域与密级子域加密、不做去重、发布前类型识别与恶意内容检查 |
| 第 7.2 章 已过账分录、库存流水、审批证据与审计证据不可覆盖 | 仅追加表设计、`ep-adapter-file` 的不可删除不可覆盖命名空间、CI 的 SQL 静态检查 |
| 第 7.3 章 Outbox 可靠投递测试项 | 三组测试作为数据库认证套件的输入 |
| 第 7.5 章 文件、分析与归档 | 事务库只存对象 ID、版本、哈希、大小、类型、密级、密钥引用与业务关联；上传五段流程；应用级不可变五项要求中的前四项；审计证据与附件使用独立存储路径与独立保留策略 |
| 第 7.7 章 法人行级隔离 | 本阶段 24 张表的统一策略模板；后台扫描按法人逐轮 |
| 第 7.8 章 密钥域 | 附件按法人密钥域与密级子域加密；推送令牌按字段级密钥加密、盲索引承担唯一性 |
| 第 7.9 章 派生存储安全继承 | Outbox 信封强制携带来源对象 ID、版本、法人 ID、密级与数据范围标签，缺失即拒绝入队；内置检索索引按法人分区，索引文档携带密级与数据范围标签，查询按 `SecurityContext.clearance_level` 过滤且不作为授权判据 |
| 第 5.6 章 模块生命周期 | `platform_core` 三张许可表、`ModuleLicenseQuery` 与安装态状态机；模块停用后其定时器不再触发、其事件不再投递 |
| 第 9.1 章 流程引擎语义要求 | 八条要求逐条实现：同事务持久化、步骤幂等键、定时器幂等可重放且不把单副本当前提、补偿逆序与人工任务兜底、定义版本化、运行约束、引擎状态写审计、高风险流程不依赖内存状态 |
| 第 10.1 章 内部异步处理使用 Outbox 与轻量队列 | 内置队列构建在 Outbox 表之上，不引入外部消息中间件 |
| 第 10.2 章 关账受理前提的可枚举依据 | Outbox 与死信按法人、会计期间、记账日期的索引与视图 |
| 第 12.5 章 审计 | 十条要求逐条实现，含同事务写入、按法人与自然日分段、数据库单调序列为唯一串行化点、客户端不建本地链、SHA-256 哈希链、每 5 分钟或 1000 条签名、签名后立即写证据、最近锚定时间可见、链验证工具、日志与审计分离 |
| 第 13.1 章 让路顺序机制二 | job-worker 内部的应用层调度：对账批次优先于其余后台任务，本阶段的后台任务一律排在对账批次之后 |
| 第 13.4 章 附件恢复点对齐 | 提供 `v_attachment_watermark_inputs` 作为水位输入；不采用元数据事务同步等待外写的实现 |
| 第 15.1 章 错误分类 | 五类分类、四要素（关联编号、发生时间、可否重试、处理建议）、界面不展示堆栈密钥与内部拓扑 |
| 第 15.2 章 可靠任务 | 命令幂等、同事务写业务状态与 Outbox、持久化 Saga 与补偿与人工任务、超时退避熔断死信重放 |
| 第 15.3 章 运维中心 | 提供 Outbox 与死信积压、锚定滞后两类数据源与四个只读视图；台账本体由运维阶段实现 |
| 第 17.2 章 单元测试、领域属性测试、集成与契约测试、流程引擎认证、审计链与不可变存储测试、混沌与故障注入 | 见第 3.8 节 |
| 第 17.3 章 审计链可验证 | 链验证工具在指定范围内全通过即为该项判据 |
| 第 19 章 阶段 2 退出条件中的 Outbox、幂等、编号、通知、文件引用与流程引擎必过项 | 本阶段承担；许可与配置发布两项按裁定 A-05 与 A-27 前移到本阶段 3b 段；身份、授权、证书、高风险操作重新认证、数据保护控制五项由其他阶段承担 |

#### 3.10.2 PRD 条目

| PRD 节 | 本阶段实现的功能 |
|---|---|
| 10.4.4 流程定制 | 版本化定义、运行中实例继续用发起时版本、版本迁移可模拟可回退、四项运行约束、步骤幂等键、补偿逆序与人工任务队列 |
| 10.5.1 两条渠道及其地位 | 站内通知不可豁免且不依赖任何对外出口；移动推送为可选增强，不可用时不影响应用内送达 |
| 10.5.2 提醒事项清单 | 十类提醒事项的 `notice_type` 枚举与模板；其中审批待办、审批结果、高风险操作待审批、流程时限四类由本阶段的流程引擎直接触发；合同到期、对账差异、关账被拒、死信与人工任务、许可临期、运维降级六类由本阶段提供写入接口，触发源在各自阶段 |
| 10.5.3 提醒规则的配置 | 定时器承载，触发与执行幂等且可重放，重启与从备份恢复后不漏不重 |
| 10.5.4 通知中心的用户操作 | 查看、跳转、标记已读、与审批待办的关系四项；跳转后仍按权限判定，无权时不展示单据内容 |
| 10.5.5 异常 | 无权限时正文不含无权字段；推送不可用时不重试到其他渠道；接收人停用需重新指派（指派方式待决，本阶段进人工任务队列） |
| 10.6.1 审计查询入口与条件 | 查询条件八项、结果列十一项、字段级裁剪、高风险行的四项附加列、按记账日期与会计期间检索、运行日志不在本入口 |
| 10.6.2 审计链验证工具 | 输入、处理、输出、异常、诚实要求五项 |
| 11.4 附件与大文件 | 5 GB 上限、分片与断点续传与带宽限制、上传中与检查中与可用三种状态可区分、正文传输不计入时延通过线、不做去重、新版本不覆盖旧版本、删除只写标记 |
| 11.10 错误与失败提示 | 五类分类、四要素、外部系统故障首版仅指电子签章（推送失败不属此类）、基础设施故障如实表达为服务不可用 |
| 附录乙 U-A-01、U-A-02 | 编号规则按基线第 11.1 节实现，落点为本阶段的 `ep-platform-sequence`，PRD 第 10 节的编号能力承载节由此确定 |
| 附录乙 U-A-03 | 本阶段表的文本长度按基线第 11.2 节的 CHECK 约束实现 |
| 附录乙 U-A-06 | `docs/error-codes.md` 的 `PLATFORM` 段建立错误码编制规则与文案表的第一批条目 |

---

### 3.11 风险与预留

#### 3.11.1 技术风险

风险一，审计段行是全局串行化点，其持锁时长是系统吞吐的硬约束。若某个用例把审计写入放在事务中段而后续还有慢操作，段锁会被拉长到该慢操作的时长，进而拖垮同法人同日的全部写入。控制手段有三：工作单元在类型层面把 `audit.record` 约束为闭包的末位调用；集成测试中加一条“段锁等待时长上限”的断言；`ep_audit_segment_lock_wait_seconds` 指标在超过 200 毫秒时告警。残余风险是业务模块阶段引入的长事务仍可能触发，需在各业务阶段的代码评审清单中保留该项。

风险二，`bigserial` 分配不回滚使段内 `seq` 出现空洞，若验证工具误把空洞当断裂，会在没有任何篡改的情况下报失败并触发不必要的安全事件响应。控制手段是把“按 `seq` 升序连接、不要求连续”写入验证算法与验证报告口径，并在集成测试中构造大量回滚事务后验证仍通过。

风险三，`jsonb` 的数值规范化会破坏哈希可重算性。控制手段是 `before` 与 `after` 中一切数值以字符串承载，并在单元测试中断言 `1.10` 与 `1.1` 在字符串承载下产出不同哈希。残余风险是业务模块阶段的开发者可能直接把 `Decimal` 序列化为 number，需要在 `AuditRecorder` 的入参类型上禁止 number，由类型系统拦截。

风险四，staging 明文窗口。已采用会话级临时密钥消除，但会话密钥与数据密钥同在一台服务器上，掌握操作系统权限者仍可解密。这是规格第 7.5 章与第 13.1 章已声明的残余风险的一部分，不额外承诺。

风险五，恶意内容检查的深度有限。三个内置检查器只能拦住已知结构特征，无病毒库即无法拦住已知恶意样本。该限制写入交付说明，不得表述为已提供病毒防护。

风险六，移动推送出口的进程归属偏离基线。见第 3.12 节，若基线修订未获批准，退路是关闭推送（`push_enabled = false`），此时该部署只剩站内通知，不影响任何必须送达的提醒，代价是移动端到达能力缺失，按规格第 5.7 章退回站内通知。

风险七，本阶段无业务模块，全部验收在合成聚合上完成，存在“合成场景通过而真实场景不通过”的风险。控制手段是合成聚合必须覆盖真实业务的三个特征：多表写入、跨模块契约调用、带会计期间归属的事件；并在业务模块阶段回归本阶段的流程引擎认证套件。

风险八，Outbox 表在基准数据集下达到 200 万行，取件语句的索引选择性依赖 `status` 的分布。若 `DONE` 条目清理不及时，`status = 'PENDING'` 的扫描可能退化。控制手段是保留期清理作为必跑的后台任务并进入连续两个执行窗口未完成即告警的口径，以及 `EXPLAIN` 证据作为退出条件。

风险九，保留期清理需要在 `platform_msg` 与 `platform_flow` 上执行 `DELETE`，与基线第 3.6 节的禁止清单冲突。见第 3.12 节偏离项四。清理顺序必须保证 `inbox_consumptions` 的保留期严格长于 `outbox_events` 的 `DONE` 保留期，否则先清消费记录再重放已清 Outbox 条目会重复产生副作用；本阶段取 60 天与 30 天，差值 30 天，并在清理任务中加断言：若 `INBOX_RETENTION_DAYS` 不大于 `DONE_RETENTION_DAYS`，启动自检失败。

风险十，3b 段的范围因裁定 A-05、A-07、A-19 与 A-27 扩大到许可、检索与配置发布三项，工期与评审面随之扩大。控制手段是三项一律按最小集交付：许可只交付表、状态机与 `ModuleLicenseQuery`，不交付运行期操作端点；检索只交付端口、适配与消费者，不交付任何业务对象的投影函数；配置发布只交付六态与两个 applier，不交付自动测试、编辑锁与在线 DDL。残余风险是阶段 13b 扩展时要在已发布的三张配置表上做列扩展与 CHECK 扩展，该扩展按基线第 3.9 节的在线 DDL 约束执行，并须持有阶段 2 提供的迁移窗口。

风险十一，3a 与 3b 拆段后迁移号段跨越阶段 4。若排期变动使 3b 早于阶段 4 落地，第 2 至 32 号的时间戳必须一并前移，否则会出现已应用版本号大于待应用版本号的乱序。控制手段是把号段与阶段顺序的对应关系写入第 3.3.1 节，并在 CI 中断言本阶段 3b 号段严格大于阶段 4 的最大版本号。

#### 3.11.2 为后续阶段预留的扩展点

`AuditRecorder` 端口：业务模块只调用 `record(tx, ctx, action, object, before, after, reason, approval_ref, reauth_ref)`，不接触链与段。

`NumberAllocator` 端口：业务模块只声明类型码与作用域，不接触序列表。

`OutboxWriter` 与消费者注册表：新模块只需登记事件类型与处理器，不改调度代码。

`ObjectStore` 的三个命名空间与 `DisposalPort`：阶段 14 只需以 `OpsDisposalService` 实现 `DisposalPort`，不改存储适配。

`ConfigItemApplier` 与 `ConfigItemApplierRegistry`：阶段 4、11 与 13b 只需实现自己的 applier 并在两个 wiring 注册，发布链路、差异审查、签名、审批与回退全部复用，不改本阶段任何表。

`SearchIndexPort` 与 `SearchQueryPort`：业务阶段只需按 `SearchDocument` 结构提供投影函数，不接触索引分区与写入时机。

`ModuleLicenseQuery`：各阶段只读该 trait 判定模块状态与功能开关，不直接读许可表。

`ContentInspector`：接入病毒引擎只需新增一个实现并在配置中启用。

`RuleEvaluator` 与 `WasmComputePort`：阶段 13b 的接入位点已存在，实现类型分别为 `AstRuleEvaluator` 与 `PluginHostWasmCompute`；本阶段只提供流程守卫条件的最小求值器与两个空实现，不占用这两个端口。

`ChannelDispatcher`：通知渠道的抽象接口已按两条渠道实现，后续版本恢复统一消息中心时新增渠道不改通知写入侧。

`v_attachment_watermark_inputs` 与 `v_evidence_write_inputs`：归档与备份阶段的唯一接缝。

`ix_outbox_events_le_period_status` 与 `ix_dead_letters_le_period_state`：关账阶段判定受理前提的取数入口。

`flow.state_persistence` 开关：规格第 9.1 章不达标预案切换到外部流程编排平台时，`outbox_eventual` 路径即为该预案的落点，本阶段已实现并已测试。

---

### 3.12 对共享技术基线的偏离项与本阶段新增决定

#### 3.12.1 偏离项，共四项，需同步修订基线

偏离一，移动推送出口的进程归属。基线第 2 节把 integration-gateway 定为“首版唯一的对外出网进程，只承载电子签章一类出口”，同时把“站内通知与推送投递”列为 job-worker 职责。规格第 5.1 章明确移动推送“依赖客户环境到外部推送服务的出网通道”，因此推送必然需要出网，而 job-worker 不应成为第二个出网进程。本阶段的处理是：推送的编排与载荷组装留在 job-worker，出网动作交由 integration-gateway 执行。影响范围为 integration-gateway 新增一个内部端点与一个出口适配器，其 cgroup、系统账户、数据库池上限均不变。提议基线第 2 节的 integration-gateway 职责描述修订为“承载电子签章与移动推送两类出口”，并同时明确：规格第 15.1 章的 `EXTERNAL_SYSTEM` 错误分类首版仍仅指电子签章，推送失败不进入该分类、不进入错误率口径，因为推送不是任何提醒的保证渠道。

偏离二，二进制正文通道。基线第 5.1 节规定载荷为 `application/json`。附件正文的上传与下载无法用 JSON 承载。提议基线第 5.1 节增列一条例外：附件正文的分片上传与下载使用 `application/octet-stream`，路径限于 `/api/v1/platform/attachments/uploads/{session_id}/parts/{part_no}` 与 `/api/v1/platform/attachments/{id}/versions/{version_no}/content` 两类，其余一律 JSON。影响范围仅限这两条路径。

偏离三，分片上传的幂等键落库豁免。基线第 5.4 节要求全部写请求的幂等键写入 `platform_msg.idempotency_keys`。分片 PUT 单次上传可产生 640 次写请求，为每次写一行幂等键在 7 天保留期内产生无谓膨胀，且分片具备 `(session_id, part_no, part_hash)` 这一自然幂等键。提议基线第 5.4 节增列：以自然幂等键判等的端点可豁免落库，豁免端点必须在基线中逐条登记，当前只有分片 PUT 一条。

偏离四，保留期清理的 `DELETE` 范围。基线第 3.6 节只允许在 `platform_msg` 的过期幂等键与 `platform_ops` 的过期指标快照上执行按期清理。本阶段需要清理 `outbox_events` 的 `DONE` 条目、`inbox_consumptions`、超过保留期的已读 `notifications` 与其 `notification_deliveries`、超过保留期的已结束 `process_instances` 及其 `process_steps` 与 `process_timers`、终态 `upload_sessions` 与 `upload_parts`。提议基线第 3.6 节把允许清理的清单扩展为上述七类，并同时明确永不清理的清单：`audit_events`、`audit_segments`、`audit_anchors`、`dead_letters`、`attachment_objects`、`attachment_versions`、`scan_results`、`process_compensations`。清理任务必须保证 `inbox_consumptions` 保留期严格长于 `outbox_events` 的 `DONE` 保留期，该断言进启动自检。

#### 3.12.2 澄清项，共三项，属基线内部张力，本阶段按下列口径执行

澄清一，工作单元内的写入顺序。基线第 10.3 节的示例顺序为保存聚合、写审计、写 Outbox。审计段行是全局串行化点，其锁持有到事务提交，因此审计必须最后写。本阶段执行顺序为保存聚合、写 Outbox、写审计，建议基线第 10.3 节的示例同步调整。两者对“同一事务内写入”这一实质要求没有任何影响。

澄清二，`outbox_events` 与 `dead_letters` 的仅追加口径。基线第 4 节把 Outbox 与死信列为仅追加表并要求带 `reverses_id`。两表的信封与载荷确实仅追加，但投递状态与处置状态必须可更新。本阶段口径为：两表不带 `row_version`、`updated_at`、`updated_by`，状态更新以条件更新的受影响行数判定并发；两表不设 `reverses_id`，因为该列的语义是业务性冲销，对投递条目无意义。建议基线第 4 节据此澄清。

澄清三，`audit_events` 的基线索引。基线第 3.10 节要求每张业务表带 `ix_<table>_legal_entity_id_created_at`，而 `audit_events` 的固定列集中没有 `created_at`，只有 `occurred_at`。本阶段以 `ix_audit_events_le_occurred` 替代，语义等价。建议基线第 3.10 节增列这一例外。

#### 3.12.3 本阶段新增决定，共十二项，基线未覆盖，阶段结束时回写

一，`ep-foundation` 新增 `resilience` 与 `canonical` 两个模块。二，平台端点的路径模块段固定为 `platform`，事件类型的模块段固定为 `platform`。三，索引名超过 63 字节时的确定性缩短规则。四，审计哈希输入采用 RFC 8785 JCS，且 `before` 与 `after` 中的数值一律以字符串承载。五，`audit_events.client` 增加 `system` 取值。六，`idempotency_keys` 增加 `state` 列与三态判定，其中并发在途一路不占用 `IdempotencyOutcome` 的变体，以 `Err(PLATFORM.IDEMPOTENCY.IN_PROGRESS)` 返回，与裁定 C-07 冻结的三个返回值并存。七，站内通知在业务事务内同步写入，不经 Outbox。八，审计链验证工具的入口只有 API 加后台任务，不交付 CLI 子命令。九，启动自检增加四个命名项。十，`ep-adapter-file` 划分 `published`、`staging`、`evidence` 三个命名空间，删除方法只在 `staging` 上存在。十一，按裁定 A-05 与 A-27，`platform_core` 的三张许可表与 `platform_meta` 的三张配置表由本阶段 3b 段建立，阶段 13b 只做列扩展与状态扩展，本阶段不建 `ep-platform-meta` 的任何自定义对象表。十二，按裁定 A-07，`ep_foundation::port::search` 的类型体与两个 trait 由本阶段补齐，阶段 1 只建空文件；索引写入只允许出现在 job-worker 的消费者中，由 `xtask archcheck` 断言。

#### 3.12.4 被业务未决事项影响的临时取值

本阶段不被任何未决事项阻塞。下列六项按临时取值实现，业务侧决策后只需改配置，不涉及结构变更，切换代价为一次重启。

| 未决编号 | 事项 | 本阶段临时取值 |
|---|---|---|
| U-K-04 | 站内通知保留期与单用户未读上限 | 180 天，2000 条 |
| U-K-05 | 推送正文是否携带业务字段 | 不携带 |
| U-K-06 | 审计查询跨度与验证工具单次跨度上限 | 366 天，92 天 |
| U-L-03 | 检查未通过附件的可见状态与处理路径 | 隔离，不可引用不可下载，保留 90 天后经处置流程删除 |
| U-L-04 | 分片大小、并发上传数、断点续传有效期、带宽限制 | 8 MiB，单用户 3、全局 6，24 小时，50 MiB/s |
| U-A-11 | 提醒默认提前量与重复频率 | 本阶段只提供配置结构，不预置取值，规则由配置发布通道下发 |

U-A-01 与 U-A-02 已由共享技术基线第 11.1 节取值，本阶段直接照做；U-A-03 已由基线第 11.2 节取值；U-B-05 已由基线第 11.3 节取值；U-A-05 已由基线第 11.5 节取值；U-L-01 与 U-L-02 已由基线第 11.6 节取值。

---

### 3.13 本阶段依赖的其他阶段产出

本节是 needs 数组的正文说明，逐项写明依赖内容、被阻塞的范围与临时替代方案。

依赖一，工程基线（阶段 1）：Cargo workspace 骨架、`rust-toolchain.toml`；`ep-foundation` 的 `Id`、`Money`、`Clock`、`IdGen`、`Rng`、`SecurityLevel`、`AppError`、`ErrorCode`、`DomainEvent` 信封，按裁定 A-03 冻结的 19 字段 `SecurityContext` 与两个构造函数，按裁定 A-01 冻结的 `port::tx` 三件套 `Tx`、`SnapshotCtx`、`UnitOfWork`（两个方法 `transact` 与 `snapshot_transact`）与 `id::marker` 的 22 个标记类型，按裁定 A-02 冻结的 `SYSTEM_PRINCIPAL_ID` 与 `SYSTEM_DEVICE_ID`，按裁定 A-05 冻结的 `ModuleCode`，按裁定 A-20 冻结的 `CapabilityDomain` 与 `ActionClass`，以及 `port::search` 的空模块文件；按裁定 C-24 由阶段 1 登记的七个平台错误码；按裁定 C-07 的 `IdempotencyKeyHeaderGuard`；按裁定 C-05 的 `tests/rls_matrix` CI 目标与八个断言函数；按裁定 C-23 注册的两个数据库连接池指标；`ep-adapter-db` 的连接池与 `ep-adapter-db-pg` 的会话变量注入与归还清除钩子，refinery Runner 与 `db/migrations/order.toml` 的骨架，CI 的依赖方向自检与 SQL 静态检查，`ep-testkit` 与 `ep-datagen` 骨架。缺失则本阶段无法开工，无临时替代。

依赖二，密钥与密码（阶段 2）：`ep-adapter-kms` 的信封加密（AES-256-GCM 数据密钥的派生与解封）与签名验签（ECDSA P-256），每法人数据加密密钥域与密级子域的建立，以及按裁定 B-04 由阶段 2 提供的盲索引派生函数 `derive_blind_key` 与 `BlindIndex`，本阶段的 `push_registrations.token_fingerprint` 取其派生值，不自建第二套哈希。缺失时以内存桩实现开发，但第 3.9 节退出条件的第 9、10、13 项必须在真实实现上判定。

依赖三，法人与组织（阶段 2）：`ep-platform-tenancy::LegalEntityDirectory::list_active`（供后台按法人轮转）与其返回的 `LegalEntityRef`，编号格式的法人段取该结构的 `entity_no`（2 位数字）。缺失时以固定两法人的测试夹具替代，退出条件不受影响，但编号格式的法人段无法在真实数据上验收。

依赖四，身份（阶段 4）：`ep-platform-identity` 的用户目录、会话令牌校验、设备登记、用户停用状态、`X-Reauth-Token` 的签发与校验。缺失则通知接收人解析、推送设备绑定、人工任务的重新认证三处只能用桩。

依赖五，授权（阶段 4）：`ep-platform-authz` 的权限项注册与判定入口、职责分离判定（申请人不可自审）、字段级与密级过滤。缺失则全部端点的权限声明只能登记不能生效，第 3.9 节退出条件第 2 项的“报表投影”与“错误信息泄漏”两类无法判定。

依赖六，元数据（阶段 13b）：`ep-platform-meta` 的自定义对象注册与六个 `CUSTOM_` 前缀的 applier，用于让附件、流程、审计、检索对自定义对象自动生效，规格第 7.4 章要求“自定义对象自动获得权限、流程、审计、搜索、API 和报表能力”。缺失不阻塞本阶段核心能力，但该自动生效的验收推迟到阶段 13b。

依赖七，配置发布与模块许可：两项均按裁定 A-05、A-19 与 A-27 前移到本阶段，不再构成对其他阶段的依赖。3a 段交付 `ConfigItemApplier` 端口与注册表，3b 段交付最小发布通道与模块许可本体，见第 3.4.11 节与第 3.4.12 节。本阶段只向后依赖两项扩展：PRD 第 10.4.1 节的十一态发布生命周期、自动测试编排与编辑锁由阶段 13b 扩展；模块停用再启用的端到端验收顺延到阶段 13b。

依赖八，可观测性（阶段 2）：`ep-platform-obs` 的日志字段约定与指标注册表，含按裁定 C-21 由阶段 2 注册并填充的 `ep_db_tx_retries_total`。本阶段产出四个数据源视图与二十条指标，台账条目的登记由运维中心承担。

依赖九，归档与备份（阶段 14）：archive-writer 的审计证据与附件正文向服务器之外落点的写出，backup-writer 的每日全量备份。本阶段只提供两个只读输入视图，不实现写出；按裁定 C-27，archive-writer 对审计证据目录只有组只读权限，证据文件与段根签名由本阶段的 job-worker 产生。规格第 13.3 章的 RPO 判定不在本阶段范围。

依赖十，电子签章（阶段 6）：`ep-adapter-esign` 按裁定 A-25 由阶段 6 交付，目录 `crates/adapter/esign/`，装配进 integration-gateway。本阶段交付熔断器与重试组件供其复用，`EXTERNAL_SYSTEM` 错误分类的唯一来源在阶段 6 落地，真实沙箱的通过记录在阶段 14 的认证清单中判定。

依赖十一，业务决策：U-K-04、U-K-05、U-K-06、U-L-03、U-L-04、U-A-11 六项，本阶段已给临时取值，不构成阻塞。
