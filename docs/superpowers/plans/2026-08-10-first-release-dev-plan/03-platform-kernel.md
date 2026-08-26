> **F-57 状态：`HISTORICAL_DO_NOT_EXECUTE`。** 旧平台内核任务已被 F-57 generation/package/durable automation 计划取代。

## 阶段 3：平台内核

> **F-50 增量。** 单据类型码登记增加 `CORR`（总账更正凭证），配置文档新增发票税额容差与历史成交终态展示两键；事件目录新增 F-50 财务、发票、门户和更正凭证事件。精确登记已先落在 `docs/data-dictionary.md`、`docs/config-reference.md` 与 `docs/event-catalog.md`，实现只按 F-50 实施计划。

本阶段建设十组平台能力：Outbox 与幂等、单据编号、审计事件哈希链分段签名、文件引用与本地文件存储、站内通知与移动推送出口、持久化工作流与死信、错误分类与重试、全文检索索引与查询、模块许可与生命周期、最小配置发布通道。前七组对应规格第 5.1 章“Outbox、幂等、编号、通知、文件引用”条目、“低代码表单、流程、规则、审批、定时器、补偿和 SLA”条目中的流程引擎运行时部分、“审计与变更留痕”条目，以及规格第 15.1 章与第 15.2 章的错误分类与可靠任务要求；后三组按归属裁定 A-05、A-07、A-19 与 A-27 前移到本阶段，理由是本阶段自身的定时器扫描与 Outbox 投递要按模块开关过滤、阶段 5 的模块启用动作要取 `ModuleLicenseQuery` 判定、阶段 4 的三个授权类内容项落地要求配置发布的内容项端口先于阶段 4 存在。本阶段不建设身份、授权、法人隔离、密钥、元数据与运维中心，这些由其他阶段提供，见文末依赖清单。

本阶段按裁定通则第四条的顺序链拆为 3a 与 3b 两段，3a 排在阶段 4 之前，3b 排在阶段 4 之后。3a 段只含两项，且都不依赖身份、授权与法人隔离：`platform_msg.idempotency_keys` 表与其端口实现，配置发布的内容项落地端口与注册表。其余全部内容属 3b 段。全文按此标注，凡未标注 3a 的交付物、迁移与退出条件均属 3b 段。

本阶段没有任何业务模块，因此全部验收都在合成聚合上完成，这一点与规格第 17.2 章流程引擎认证套件“使用合成聚合与合成不变量验证补偿结果，不提前引用尚未建设的财务或库存模块”的要求一致。

### 3.0 本阶段的判定前提与四条贯穿设计

在展开清单之前先固定四条贯穿本阶段全部组件的设计判定，后续各节直接引用，不再重复论证。

判定一，行级安全使全部后台扫描必须按法人逐轮进行。共享技术基线第 3.8 节规定 `app.legal_entity_id` 是行级策略的唯一判据，不设 `BYPASSRLS` 角色，跨法人访问只能逐个法人设置会话变量后分别查询。本阶段的 Outbox 取件、定时器扫描、审计段锚定、通知投递、上传会话回收、死信统计六类后台扫描全部落在 job-worker 内，因此它们一律实现为“取法人清单，按法人轮转，每法人一次独立事务”。法人清单由阶段 2 的 ep-platform-tenancy 契约提供。首版法人数为 2，轮询间隔 200 毫秒按法人平摊后每法人 100 毫秒，仍在规格第 15.2 章的可靠任务要求之内。这一形态不是实现偏好，是行级安全的必然结果，任何“一次扫全库”的实现都会被行级策略静默过滤成空集，属实现缺陷。

判定二，审计段行是本阶段唯一的全局串行化点，其持锁时长直接决定系统吞吐上限。规格第 12.5 章要求“段内链序由事务数据库的单调序列分配，该序列是唯一串行化点，核心不持有链状态”，同时要求每条事件在事务内写入前序哈希与本条哈希。前序哈希必须读取该段当前最后一条事件的哈希，而读取与写入之间若不串行化，两个并发事务会读到同一前序哈希并写出两条互不衔接的链条。因此审计追加必须在 `platform_audit.audit_segments` 的段行上取排他锁。该锁在事务提交时才释放，故审计写入必须是工作单元闭包内的最后一次数据库写入：审计写入之后不得再发起任何数据库写入，包括 Outbox 入队、按第 3.4.5 节在同一事务内写入的站内通知与 `notification_deliveries`、幂等键的 `finish` 回写与任何投影回填。此处取最后一次写入而不是靠后于 Outbox，否则同事务内的通知与回填仍可能排在审计之后，硬边界形同虚设。这一条要求把共享技术基线第 10.3 节示例中的写入顺序由“保存聚合、写审计、写 Outbox”调整为“保存聚合、写 Outbox、写审计”，具体修订文字见第 3.12.2 节澄清一。

判定三，附件的元数据可用状态严格蕴含本机正文存在。共享技术基线第 10.3 节禁止在事务内做文件正文读写，规格第 7.5 章又禁止文件存储路径开放覆盖写与原地删除接口，两条合起来意味着“先落盘后写元数据”会在崩溃时留下无法清除的孤儿文件，“先写元数据后落盘”会产生元数据在而正文不在的窗口。本阶段采用三段式：先写版本行为 `PENDING` 并预分配存储路径，再落盘，再在第二个事务内置为 `AVAILABLE`。崩溃落在任一间隙都可由 job-worker 内的幂等收敛任务按“路径上文件是否存在”收敛，且 `AVAILABLE` 一经写入即蕴含正文已在本机落盘。该任务按裁定 A-06 不称为对账：它不产生对账差异事项，不实现 `ep_platform_recon::ReconCheck`，也不依赖阶段 9a 交付的 `ep-platform-recon` 框架。该性质是规格第 13.4 章附件恢复点水位口径成立的本机侧前提。
判定四，本阶段按 T0 贯通线拆两批交付，只改工作次序，不改范围归属。整卷在阶段 4 结束后、阶段 5 全量开工之前插入一条不新增任何范围的最薄贯通线 T0，判据是一条合同从建单走到管理层看到一个数。本阶段排在 T0 之前，是 T0 的前置，向 T0 贡献六个最小切片：一次单据编号分配、一次审计事件追加与其段行链接、一次 Outbox 写入与一次消费、一条同事务写入的站内通知、一个单审批节点的流程定义及其实例与人工任务的完成、一次经最小配置发布通道把该流程定义发布到 `platform_flow.process_definitions`。除这六项外本阶段不向 T0 贡献任何东西，T0 不要求本阶段的附件、检索、推送、定时器、补偿、许可、死信与混沌各项，也不要求四端界面。

据此把 3b 段拆为两批。3b-1 批是 T0 的前置闸门，含上述六个切片所依赖的全部实现，即 Outbox 与幂等、单据编号、审计哈希链与段行、站内通知的同事务写入、流程引擎的单节点审批路径（定义、实例、人工任务、完成、取消）、最小配置发布通道 Draft 到 Released 的一条直路与 `FlowDefinitionApplier`、错误分类到响应封套的映射，以及第 3.7 节四个自检项中的三个阻断级项。3b-2 批在 T0 已贯通的骨架上加厚，含附件上传流水线与本地文件存储、移动推送出口、定时器与 SLA、补偿与人工修复、流程定义版本迁移与模拟、死信与双人审批丢弃、全文检索、模块许可与生命周期、配置发布的回退路径与 `NotifyRuleApplier`、保留期清理、混沌与故障注入五类、基准数据集扩展与全部性能度量项。

3b-2 批各项都不是 T0 与阶段 5 开工的前置，其相对次序由下游拉动点决定，逐项固定为：模块许可与生命周期在阶段 5 的模块启用动作落地之前就位；全文检索在阶段 5 判定附录 A.1 检索度量项之前就位；附件上传流水线在阶段 6 的合同附件与签章证据之前就位；影响面平台本体、七条编译期目录、两张台账表与唯一消费者在阶段 6 开工之前就位；移动推送出口、死信统计与保留期清理在阶段 11 之前就位；混沌与故障注入五类、基准数据集扩展与性能度量项在阶段 14 的认证清单之前就位。第 3.9 节的退出条件不因此拆分而放宽，两批合并判定，拆批只改次序不改判据；流程引擎认证套件五组仍在 3b-2 批内全量执行，M7 在整卷中的地位是全分支闭环而不是首次贯通。

---

### 3.1 交付物清单

本阶段结束时，下列东西存在且可运行。

平台能力，八项。

1. Outbox 写入与消费：业务状态、审计事件与 Outbox 条目在同一数据库事务写入；job-worker 按法人轮转取件、投递、退避重试、转死信；消费端由 `platform_msg.inbox_consumptions` 保证幂等；投递统计与积压指标可读。
2. 幂等键（3a 段）：`platform_msg.idempotency_keys` 表与阶段 2 定义的 `ep_foundation::port::db::IdempotencyStore` 端口实现，`try_begin(tx, scope, request_hash)` 返回 `IdempotencyOutcome::FirstCall`、`Replay { status, body }` 或 `PayloadMismatch`，`finish(tx, scope, response_status, response_body)` 写回首次结果；重复请求回放首次结果并回带 `Idempotent-Replay: true`。请求头的存在性与 UUIDv7 合法性由阶段 1 的 `IdempotencyKeyHeaderGuard` 校验并返回 `PLATFORM.IDEMPOTENCY.KEY_REQUIRED`，本阶段不重复校验、不重复登记该码。
3. 单据编号与档案编码：按共享技术基线第 11.1 节的格式生成，在业务事务内取号，回滚即退号，位数溢出自动扩展。
4. 审计事件哈希链与分段签名：按法人与自然日分段的 SHA-256 哈希链、每 5 分钟或每 1000 条的 ECDSA P-256 段根签名、签名后立即写入独立的审计证据存储路径、链验证工具与验证报告。段根签名经 `ep_foundation::port::kms::KmsBackend` 的 `sign` 执行，私钥不出载体，`ep-platform-audit` 不依赖 `ep-adapter-kms`。
5. 文件引用与本地文件存储：附件对象与版本模型、分片上传与断点续传、类型识别与结构型恶意内容检查、按部署模式明确跳过或调用同机客户 ICAP 病毒扫描器、按法人密钥域与密级子域的固定 1 MiB 分块信封加密落盘（新版本只在开始时经 `ep_foundation::port::kms::KmsPinnedDataKeyBackend::resolve_data_key(..., CurrentForWrite)` 固定一把数据密钥，逐块 `wrap_with_data_key`，历史读取以 `ExactRef` 解析同一把；数据密钥和 AES 实现不出 adapter）、只写入不覆盖不删除的存储适配、附件恢复点水位的只读输入视图、`DisposalPort` 端口定义与处置受理路由（本阶段至阶段 13 之间一律直接拒绝并开一条降级窗口）。
6. 站内通知：通知实体、模板、未读计数、标记已读、按法人与接收人的列表查询；站内通知在业务事务内同步写入，不依赖任何异步链路。
7. 移动推送出口：推送设备登记、推送载荷组装与脱敏、经 integration-gateway 的出网投递、送达状态记录；推送不可用时只剩站内通知，业务提醒不中断。
8. 持久化工作流引擎：流程定义版本、实例、步骤、人工任务、定时器、SLA、补偿、运行约束、版本迁移与模拟；补偿失败进入人工任务队列并告警。同时交付 `ep_platform_flow::port::RuleEvaluator` 与 `ep_platform_flow::port::WasmComputePort` 两个端口定义，其实现类型 `AstRuleEvaluator` 与 `PluginHostWasmCompute` 按裁定 B-05 由阶段 13b 交付；两者是裁定通则第三条列明的三项例外中的两项，本阶段至阶段 13b 之间不注入任何实现，能力缺位按降级窗口承载，见第 3.4.8 节。

横切能力，四项。

9. 错误分类与错误码表：五类分类到 HTTP 状态与响应封套的统一映射，`docs/error-codes.md` 的 `PLATFORM` 段与 `ep-foundation::error::codes` 常量表由 CI 校验一致。其中 `PLATFORM.IDEMPOTENCY.KEY_REQUIRED`、`PLATFORM.IDEMPOTENCY.PAYLOAD_MISMATCH`、`PLATFORM.CAPACITY.CONCURRENCY_LIMIT`、`PLATFORM.CONCURRENCY.STALE_VERSION`、`PLATFORM.AUTHZ.NOT_FOUND_OR_DENIED`、`PLATFORM.AUTHZ.OBJECT_FORBIDDEN`、`PLATFORM.DB.MIGRATION_WINDOW_CLOSED` 七个码按裁定 C-24 由阶段 1 登记，本阶段只引用不重复登记。
10. 重试与熔断：数据访问层的序列化失败与死锁重试、Outbox 的八段退避、外部出口的超时与熔断器骨架。
11. 死信与人工修复：死信表、记名重投、双人审批丢弃、按法人与记账日期的可枚举统计（这是规格第 10.2 章关账受理前提“该法人该会计期间的异步过账队列未清空或存在未修复死信”能被判定的依据）。
12. 事件目录与登记：`docs/event-catalog.md` 的 `platform` 段中，本阶段登记且只登记三个具名事件：`platform.attachment.published.v1`、`platform.notification.push_requested.v1`、`platform.config_release.released.v1`，并冻结各自信封字段。旧“17 个”没有十四个可逐字对账的名称，已由 F-54 撤销，不得以未命名配额驱动实现；阶段 13b 扩展发布通道时不重复登记配置发布事件。

可运行的验证物，五项。

13. 流程引擎认证套件的阶段必过项，含崩溃恢复、重复投递不少于 3 次、定时器幂等与可重放、流程定义版本升级、补偿正确性五组，跑在合成聚合与合成不变量上。
14. Outbox 可靠投递测试项，含至少一次投递、重复投递去重、崩溃恢复后不丢不重三组，这是规格第 7.3 章数据库认证套件的必含项。
15. 审计链与不可变存储测试，含链验证工具在抽样法人与日期段上全通过、段根哈希与证据存储签名一致、覆盖与删除尝试被应用层拒绝并写审计。
16. 混沌与故障注入的六类场景中本阶段可独立执行的五类：依赖服务超时、连接池与内存资源耗尽、消息积压、磁盘写满、进程崩溃后重启恢复。系统时钟漂移一类的判定依赖授时源自检，与阶段的启动自检项联测。
17. 本阶段的基准数据集扩展：`ep-datagen` 增加审计事件、Outbox 条目、通知、附件对象与流程实例五类的生成器，规模取值见第 3.8.5 节。

按归属裁定前移到本阶段的能力，五项。第 19 项属 3a 段，其余四项属 3b 段。

18. 全文检索索引与查询（3b 段）：`ep-adapter-search` 实现 `ep_foundation::port::search::SearchIndexPort` 与 `SearchQueryPort`，索引按法人分区，物理路径 `C:\EP\search\<legal_entity_id>\`。`SearchDocument`、`SearchQuery`、`SearchHit` 三个类型与两个 trait 由阶段 1 在 `crates/foundation/src/port/search.rs` 建空文件、本阶段补齐。写入一律经 job-worker 消费 Outbox 事件触发，不在业务事务内调用。本阶段不交付任何业务对象的检索文档投影函数，投影由各业务阶段按 `SearchDocument` 结构提供，见第 3.4.10 节。
19. 配置发布的内容项落地端口（3a 段）：`crates/platform/release/src/port/config_item.rs`，内容为 `ConfigItemApplier` trait、`ItemKind` 类型、`ConfigPackageItem` DTO 与 `ConfigItemApplierRegistry`，其中 `Tx` 取自 `ep_foundation::port::tx`。本项无表、无用例、不依赖身份与授权，因此排在阶段 4 之前，使阶段 4 的三个授权类 applier 不再倒挂；3b 按 F-56 把 Rust `ItemKind::ALL` 与首次数据库 CHECK 同步冻结为阶段快照 18 项（前 16 项加 `LICENSE_GRANT|MODULE_PACKAGE`），阶段 13b 的既定 `V20261022090500` 再追加两个 MCP 项，使 Rust 与 CHECK 同批到终态 20，见第 3.4.12 节。
20. 最小配置发布通道（3b 段）：`platform_meta.config_packages`、`platform_meta.config_package_items`、`platform_meta.config_release_orders` 三张表，六态发布状态机，ECDSA P-256 外层签名与逐项 `item_hash` 校验，发布与普通回退用例，`ConfigItemApplierRegistry` 的运行期装配，以及 `FlowDefinitionApplier`、`NotifyRuleApplier`、`LicenseGrantApplier`、`ModulePackageApplier` 四个 applier。后两者只接受 F-56 的双重签名特殊单项包，禁止通用回退。本阶段不建 `config_release_steps` 与 `config_edit_locks`，十一态生命周期、自动测试编排、编辑锁和数据库 `item_kind` 终态 CHECK 由阶段 13b 扩展。
21. 模块许可与生命周期（3b-2 批）：`platform_core.module_registrations`、`platform_core.license_grants`、`platform_core.feature_flags` 三张表与 `ep-platform-license` 的唯一 `ModuleLicenseQuery`。同时交付 F-56 永久/订阅许可、内层 detached CMS 验证、可信时间、`ACTIVE|EXPIRING_SOON|GRACE_PERIOD|RESTRICTED` 四态、`F55_LOCAL_AI|F55_MCP` entitlement、声明式内置模块包及五条合法动作。定时器扫描与 Outbox 投递必须以作业/事件已绑定的目标法人调用 `ModuleLicenseQuery::module_is_currently_licensed`，不得用只反映原始管理投影的 `module_state` 放行业务运行；许可业务状态受限不阻止进程启动，也不进入 Blocking 自检，只有不读取许可业务状态的静态 `license-admission-registry-consistent` 装配一致性检查为 Blocking。模块与许可端到端全链验收按 F-56 顺延到阶段 13b，并由 `RG-LICENSE-MODULE-LIFECYCLE-GREEN` 收口，见第 3.4.11 节。
22. 合同终止影响面平台（3b-2 批，阶段 6 硬前置）：建立 `platform_core.impact_assessments`、`platform_core.impact_disposition_items`，交付 `ep-platform-impact` 的 `ImpactRule`、`ImpactRegistry`、`ImpactAssessor`、`ImpactAssessmentQuery` 与统一三态结果，编译期目录与 `docs/impact-catalog.md` 恰为七条；job-worker 只注册一个 `platform.impact_assess` 消费者。阶段 3 的真实规则注册数为 0，业务规则依阶段 6/7/10/12 增至 3/4/6/7；未接线类别只建 PENDING 占位项，不注入替身，不自动闭合。

---

### 3.2 crate 与进程归属

#### 3.2.1 新增 crate

| crate | 层 | 新增或改动 | 装配进程 |
|---|---|---|---|
| ep-platform-outbox | platform | 新增 | core-server 写入侧，job-worker 唯一消费侧；integration-gateway 不链接、不消费 |
| ep-platform-sequence | platform | 新增 | core-server |
| ep-platform-audit | platform | 新增 | core-server 追加侧，job-worker 锚定与验证侧 |
| ep-platform-file | platform | 新增 | core-server |
| ep-platform-notify | platform | 新增 | core-server 站内写入侧，job-worker 推送编排侧 |
| ep-platform-flow | platform | 新增 | core-server 命令侧，job-worker 调度与执行侧 |
| ep-platform-license | platform | 新增（3b 段） | core-server，job-worker |
| ep-platform-release | platform | 新增（3a 段端口，3b 段通道） | core-server 发布与回退侧，job-worker 传播段预留 |
| ep-platform-impact | platform | 新增（3b-2 批） | core-server 查询、人工决策与 replay；job-worker 唯一消费者及处置执行 |
| ep-adapter-file | adapter | 新增 | core-server，job-worker 只读 |
| ep-adapter-search | adapter | 新增（3b 段） | job-worker 写入侧，core-server 查询侧 |
| ep-foundation | foundation | 改动 | 全部 |
| ep-adapter-db-pg | adapter | 改动 | 全部持库进程 |
| ep-platform-obs | platform | 改动 | 全部 |
| ep-testkit | 测试 | 改动 | 测试 |
| ep-datagen | 测试 | 改动 | 测试 |

`ep-adapter-kms` 由阶段 2 交付，本阶段不依赖该 crate，只依赖阶段 2 在 `ep_foundation::port::kms` 内补齐的 `KmsBackend` 端口，两个载体实现的实例在 `apps/core-server/src/wiring/` 与 `apps/job-worker/src/wiring/` 两个目录内注入。按裁定 F-04，阶段顺序 1 → 2 → 3a 已定，端口与两个载体实现在本阶段开工前均已存在，本条不留任何桩路径，退出条件一律在真实载体实现上判定。

#### 3.2.2 各 crate 的内容边界

`ep-foundation` 新增四组类型，全部为无业务语义的通用类型，符合基线第 1.3 节对 foundation 的禁止项。必要性按基线第 12 节通则第六条降为评审判据，不由 `xtask archcheck` 判定；下列四组在提交说明中按 文件：行号 逐项举证使用位，缺举证的提交评审时按不通过处理。

- `error::codes` 常量表的 `PLATFORM` 段，`AppError` 的 `incident_no`、`occurred_at`、`retryable`、`advice` 四个字段的构造与序列化。
- `resilience` 模块：`Backoff`（固定序列与指数两种）、`CircuitBreaker`（失败计数、开启窗口、半开探针）、`RetryPolicy`。这是本基线未覆盖事项，登记为本阶段新增决定。
- `canonical` 模块：RFC 8785 JCS 规范化序列化，供审计哈希与证据文件使用。
- `Redacted<T>` 与 `SecretString` 的日志与错误消息拦截，配合第 3.9 节的日志禁止清单。

`ep-platform-outbox`：`OutboxWriter` 与 `OutboxConsumer` 端口、信封构造与校验、幂等键仓储、死信模型与状态机、退避策略。不含任何模块的事件语义。

`ep-platform-sequence`：`NumberAllocator` 端口、编号格式化与解析、类型码注册表、位数扩展算法。

`ep-platform-audit`：`AuditRecorder` 端口、段模型与链追加算法、锚定任务、链验证器、证据文件的写入与读取端口。

`ep-platform-file`：附件对象与版本聚合、上传会话状态机、扫描端口 `ContentInspector`、存储端口 `ObjectStore`、密钥引用组装。附件正文只调用 `ep_foundation::port::kms::KmsPinnedDataKeyBackend` 的 `resolve_data_key/wrap_with_data_key/unwrap_with_data_key` 三方法，不读取 `data_keys.wrapped_key`、不取得明文 DEK，也不在 platform/file adapter 外自行执行 AES；正文的字节读写全部经 `ep-adapter-file`，该 adapter 不接触密钥材料，也不依赖 `ep-adapter-kms`。处置端口 `DisposalPort` 与其两个 DTO 按裁定 A-22 定义在 `crates/platform/file/src/port/disposal.rs`，本阶段只给 trait 与 DTO，不给任何实现、两个 wiring 目录内不出现注入行，实现与注入行由阶段 14 同批落地；`DisposalPort` 是裁定通则第三条列明的三项例外之一，按例外档处理而不是整条推迟，本阶段注册处置受理路由，本阶段至阶段 13 之间的物理删除请求直接拒绝并同时开一条降级窗口，见第 3.4.7 节。

`ep-platform-notify`：通知聚合、模板渲染、接收人解析端口、推送载荷组装与脱敏、送达状态。

`ep-platform-flow`：流程定义模型、实例聚合、步骤与补偿模型、定时器、人工任务、守卫条件表达式的最小求值器、调度器与执行器。

`ep-platform-impact`：只含中立的规则目录、评估批次、处置项、注册表、执行器、查询与人工决策契约，不含合同、销售、采购、发票或项目类型。`ImpactRule` 实现落在被影响模块自己的 `ep-app-*`；本 crate 不依赖任何业务 crate。七条目录常量逐项取 `docs/impact-catalog.md`，而真实规则注册采用阶段 6/7/10/12 的渐进值，不允许在阶段 3 注入空规则。

`ep-adapter-file`：两个命名空间的本机文件存储。`published` 命名空间只提供 `create_new`、`open_read`、`stat` 三个方法，不提供覆盖与删除；`staging` 命名空间额外提供 `remove`，只承载未发布的分片临时数据。审计证据存储是第三个命名空间 `evidence`，与 `published` 同为只追加但使用独立根目录与独立保留策略，对应规格第 7.5 章“与本章的文件使用独立的存储路径和独立的保留策略”。

`ep-platform-license`（3b 段）：F-56 永久/订阅 grant 与撤销解析、内层 CMS/部署绑定复验、可信时间和四态判定、模块/entitlement/feature 求值、声明式模块包五动作状态机，以及 `LicenseGrantApplier`、`ModulePackageApplier`；运行时对外只暴露 `ModuleLicenseQuery`，配置发布侧只实现 `ConfigItemApplier`，不含任何模块业务语义或可执行模块正文。

`ep-platform-release`：3a 段只含 `port::config_item` 一个模块，即 `ConfigItemApplier` trait、`ItemKind`、`ConfigPackageItem` 与 `ConfigItemApplierRegistry`，除 `ep-foundation` 外不依赖任何 crate；3b 段追加配置包与发布单聚合、六态状态机、签名与验签、发布与回退编排。本 crate 的工作区内依赖在 3b 段止于 `ep-foundation`、`ep-platform-audit` 与 `ep-platform-outbox` 三项，阶段 13b 不再新增，见第 3.2.3 节。

`ep-adapter-search`（3b 段）：内置检索索引的按法人分区读写，实现 `ep_foundation::port::search` 的两个 trait。只依赖 `ep-foundation`，不依赖任何 `ep-platform-*`，索引根目录与分区路径见第 3.4.10 节。

#### 3.2.3 依赖方向核对

本阶段全部新增 crate 均为 `ep-platform-*` 与 `ep-adapter-*`，依赖只指向 `ep-foundation` 与其他 `ep-platform-*`，不依赖任何 `ep-domain-*` 与 `ep-app-*`，符合基线第 1.3 节。`ep-adapter-file` 只依赖 `ep-foundation` 与 `ep-platform-file` 中的 `ObjectStore` 端口 trait，不依赖任何其他 adapter。装配全部发生在 `apps/core-server/src/wiring/`、`apps/job-worker/src/wiring/`、`apps/integration-gateway/src/wiring/` 三个目录下的全部文件中。消费 KMS 能力的 `ep-platform-audit`、`ep-platform-file`、`ep-platform-notify` 与 `ep-platform-release` 四个 crate 一律只依赖 `ep_foundation::port::kms`，不依赖 `ep-adapter-kms`，载体实例在 `apps/core-server/src/wiring/` 与 `apps/job-worker/src/wiring/` 两个目录内注入；本段「依赖只指向 `ep-foundation` 与其他 `ep-platform-*`」因此成立，按裁定 F-04。

platform 内部的依赖边为：`ep-platform-outbox → ep-foundation`；`ep-platform-audit → ep-foundation`；`ep-platform-file → ep-foundation`；`ep-platform-sequence → ep-foundation`；`ep-platform-license → ep-foundation, ep-platform-release`；`ep-platform-impact → ep-foundation, ep-platform-flow, ep-platform-audit, ep-platform-outbox`；`ep-platform-release → ep-foundation`（3a 段），3b 段追加 `ep-platform-audit`、`ep-platform-outbox`；`ep-platform-notify → ep-foundation, ep-platform-release`；`ep-platform-flow → ep-foundation, ep-platform-outbox, ep-platform-audit, ep-platform-release`。无环，因为 `ep-platform-release` 不反向依赖任何 `ConfigItemApplier` 属主 crate，且 `ep-platform-flow` 不反向依赖 `ep-platform-impact`；3b 的 `ep-platform-flow`、`ep-platform-notify` 与 `ep-platform-license`、阶段 4 的 `ep-platform-authz`、阶段 11 的 `ep-app-reporting`、阶段 13b 的 `ep-platform-meta`、阶段 13c 的 `ep-platform-mcp` 一律在外，终态 20 个 `ItemKind` applier 全部落在实现方 crate 内。该方向对全卷生效，任何阶段不得为 `ep-platform-release` 新增指向属主 crate 的依赖边；跨 crate 的执行编排一律落在 `apps/*`，不落 `ep-platform-release`。`ep-adapter-search` 只依赖 `ep-foundation`。本阶段新增的九个 platform crate 与两个 adapter crate 一并纳入 `xtask archcheck` 的层位判定与 `platform-acyclic`、`platform-no-adapter` 两条规则，不另立按 crate 逐项比对期望依赖清单的自检脚本；本节的依赖枚举是本阶段结束时的快照，后续阶段可在基线第 1.3 节允许项内增边，见基线第 1.3 节末段。

#### 3.2.4 进程职责增量

| 进程 | 本阶段新增职责 |
|---|---|
| core-server | 全部平台端点；业务事务内的取号、审计追加、Outbox 写入、站内通知写入；附件上传下载的正文读写；流程实例的启动、取消与人工任务命令；影响面批次查询、人工处置与记名 replay；模块许可判定；全文检索查询；配置包与发布单的创建、审批、签名、发布执行与回退 |
| job-worker | 按法人轮转的 Outbox 取件与投递、唯一 `platform.impact_assess` 消费与影响项处置、审计段锚定与证据写出、链验证任务、流程调度与步骤执行、定时器扫描、补偿执行、推送编排、死信统计、上传会话回收、附件状态的幂等收敛、检索索引写入、保留期清理 |
| integration-gateway | 电子签章、移动推送两类外部出口，以及客户同机 ICAP 非外网扫描调用；产品侧入口只监听 DACL 保护的 `\\.\pipe\ep-integ`，不监听 TCP；只返回清洗回执或有界签章文件流，不链接 `ep-platform-outbox`、不持数据库/KMS/业务文件目录凭据、不落业务状态 |
| portal-gateway | 门户侧附件上传与通知的呈现层转发，自身不建库连接，全部取数经 core-server 的 `/api/v1/portal/...` 受控能力 API |
| ops-agent | 暴露本阶段新增指标，全部经 `ep_ops_ro` 只读角色读取运维视图 |

archive-writer 与 backup-writer 在本阶段不改动，本阶段只为其提供两个只读视图作为水位输入，见第 3.3.6 节。

---

### 3.3 数据库变更

全部迁移文件路径为 `db/migrations/<schema>/`，迁移历史落在全局唯一的 `platform_core.schema_history`。执行顺序由单一全局 Runner 按文件版本号全序排定，不存在任何模块顺序声明文件，本阶段只需保证每个文件的版本号晚于其全部被引用对象。本阶段触及八个平台 schema 中的六个，`platform_meta` 因裁定 A-27 的最小配置发布通道纳入。

每个迁移文件承载一张表的完整定义，含建表、CHECK 约束、索引与行级安全策略。理由是把行级安全拆到单独文件会产生一个策略尚未启用的中间态窗口，与基线第 3.8 节的默认拒绝口径冲突。基线第 3.9 节“一个文件只做一件事”按“一个对象的完整建立为一件事”解读，数据回填仍单独成文件。每个文件的头部注释含 `-- rollback:` 段。

#### 3.3.1 迁移编号与顺序

假设 3a 段迁移落在 2026 年 9 月中旬、3b 段迁移落在 2026 年 11 月初，编号如下表。若实际执行月份不同，只调整时间戳，相对顺序与分段归属不变。

| 序 | 文件名 | schema | 段 |
|---|---|---|---|
| 1 | `V20260915090000__platform_msg_create_idempotency_keys.sql` | platform_msg | 3a |
| 2 | `V20261013090000__platform_core_create_number_sequences.sql` | platform_core | 3b |
| 3 | `V20261013090100__platform_core_create_module_registrations.sql` | platform_core | 3b |
| 4 | `V20261013090200__platform_core_create_license_grants.sql` | platform_core | 3b |
| 5 | `V20261013090300__platform_core_create_feature_flags.sql` | platform_core | 3b |
| 6 | `V20261013090400__platform_meta_create_config_packages.sql` | platform_meta | 3b |
| 7 | `V20261013090500__platform_meta_create_config_package_items.sql` | platform_meta | 3b |
| 8 | `V20261013090600__platform_meta_create_config_release_orders.sql` | platform_meta | 3b |
| 9 | `V20261013090700__platform_flow_create_process_definitions.sql` | platform_flow | 3b |
| 10 | `V20261013090800__platform_flow_create_process_instances.sql` | platform_flow | 3b |
| 11 | `V20261013090900__platform_flow_create_process_steps.sql` | platform_flow | 3b |
| 12 | `V20261013091000__platform_flow_create_process_tasks.sql` | platform_flow | 3b |
| 13 | `V20261013091100__platform_flow_create_process_timers.sql` | platform_flow | 3b |
| 14 | `V20261013091200__platform_flow_create_process_compensations.sql` | platform_flow | 3b |
| 15 | `V20261013091300__platform_audit_create_audit_segments.sql` | platform_audit | 3b |
| 16 | `V20261013091400__platform_audit_create_audit_events.sql` | platform_audit | 3b |
| 17 | `V20261013091500__platform_audit_create_audit_anchors.sql` | platform_audit | 3b |
| 18 | `V20261013091600__platform_audit_create_audit_verifications.sql` | platform_audit | 3b |
| 19 | `V20261013091700__platform_msg_create_outbox_events.sql` | platform_msg | 3b |
| 20 | `V20261013091800__platform_msg_create_inbox_consumptions.sql` | platform_msg | 3b |
| 21 | `V20261013091900__platform_msg_create_dead_letters.sql` | platform_msg | 3b |
| 22 | `V20261013092000__platform_msg_create_notification_templates.sql` | platform_msg | 3b |
| 23 | `V20261013092100__platform_msg_create_notifications.sql` | platform_msg | 3b |
| 24 | `V20261013092200__platform_msg_create_notification_deliveries.sql` | platform_msg | 3b |
| 25 | `V20261013092300__platform_msg_create_push_registrations.sql` | platform_msg | 3b |
| 26 | `V20261013092700__platform_file_create_attachment_objects.sql` | platform_file | 3b |
| 27 | `V20261013092800__platform_file_create_attachment_versions.sql` | platform_file | 3b |
| 28 | `V20261013092900__platform_file_create_upload_sessions.sql` | platform_file | 3b |
| 29 | `V20261013093000__platform_file_create_upload_parts.sql` | platform_file | 3b |
| 30 | `V20261013093100__platform_file_create_scan_results.sql` | platform_file | 3b |
| 31 | `V20261013093200__platform_file_create_watermark_views.sql` | platform_file | 3b |
| 32 | `V20261013092400__platform_msg_create_ops_views.sql` | platform_msg | 3b |
| 33 | `V20261013092500__platform_msg_backfill_append_only_registry.sql` | platform_msg | 3b |
| 34 | `V20261013092600__platform_msg_backfill_sensitive_field_registry.sql` | platform_msg | 3b |
| 35 | `V20261013093300__platform_core_backfill_stage03_unpoliced_table_registry.sql` | platform_core | 3b |
| 36 | `V20261013093400__platform_core_create_impact_assessments.sql` | platform_core | 3b-2，须早于阶段 6 |
| 37 | `V20261013093500__platform_core_create_impact_disposition_items.sql` | platform_core | 3b-2，须晚于第 36 项且早于阶段 6 |
| 38 | `V20261013093600__platform_flow_create_approval_command_snapshots.sql` | platform_flow | 3b-2，高保密审批命令密文载体 |
| 39 | `V20261013093700__platform_flow_backfill_sensitive_field_registry.sql` | platform_flow | 3b-2，登记逻辑字段 `command` |

第 9 至 14 号与第 38、39 号在 `platform_flow` 内的顺序保证被引用方先建，第 6 至 8 号在 `platform_meta` 内同理保证 `config_packages` 早于 `config_package_items` 与 `config_release_orders`。本阶段的单目标引用全部按基线第 3.3 节建真实外键：同法人目标使用 `(legal_entity_id,<ref_id>)` 复合形状，全局身份目标使用单列主键形状；凡带法人且有 `id` 的目标表均在首次建表迁移内建立 `UNIQUE(legal_entity_id,id)` 候选键。业务用户列（含公共 `created_by/updated_by`、actor、recipient、owner、initiator、assignee、repaired/deleted by）统一指向 `platform_authz.user_legal_entity_grants(legal_entity_id,user_id)`；设备与重新认证等全局身份证据使用单列真实外键，并由持锁事务校验用户与法人归属。只有业务对象/影响结果等带判别列的封闭多态组合，以及字段名精确为 `approval_ref` 或 `release_package_id` 的平台证明保留无外键形状。第 31 与 32 两个视图文件跨 schema 取数，按裁定通则第五条放在其主要创建对象所属 schema 的目录：第 31 号的视图建在 `platform_file`，取数涉及 `platform_audit`，放在 `db/migrations/platform_file/`；第 32 号的视图建在 `platform_msg`，取数涉及 `platform_flow` 与 `platform_audit`，放在 `db/migrations/platform_msg/`。两者的版本号都晚于其取数所涉全部对象的建表迁移。第 33、34、35 与 39 号是本阶段四个数据回填文件：第 33 号按裁定 B-02 向 `platform_core.append_only_registry` 登记三张仅追加表并挂接触发器，所涉 schema 为 `platform_core`、`platform_audit` 与 `platform_msg`；第 34 号按裁定 A-28 向 `platform_core.sensitive_field_registry` 登记 `platform_msg.push_registrations` 的 `token` 一行，所涉 schema 为 `platform_core` 与 `platform_msg`。这两个文件的主要创建对象都在 `platform_msg`，按裁定通则第五条放在 `db/migrations/platform_msg/`，版本号都晚于所涉 `platform_core` 与 `platform_audit` 对象的建表迁移；第 33 号的登记行取值与挂接次序见第 3.3.7 节，第 34 号的逐列取值见表 19 之后一段。第 35 号按基线第 3.8 节的正向登记制，向阶段 2 交付的 `platform_core.unpoliced_table_registry` 写入本阶段六张不带法人列的表各一行，即 `platform_core` 的 `module_registrations`、`license_grants`、`feature_flags` 与 `platform_meta` 的 `config_packages`、`config_package_items`、`config_release_orders`；`schema_name`、`table_name`、`admission_basis`、`isolation_entry` 与 `matrix_case_id` 五列按阶段 2 冻结的列集填写，`admission_basis` 六行一律取 `SAME_FOR_ALL_ENTITIES`，`isolation_entry` 逐表写明该表法人可见性所落的应用层入口，`matrix_case_id` 取第 3.8.2 节对六张部署级表所设可见性断言用例的标识。同一 `V20261013093300` 还在配置内容表已存在后新增 `UNIQUE(config_package_id,id)` 父候选键，并补齐 F-56 六条来源约束：模块、许可 grant、许可 revocation 各一条 package FK 与一条 `(source_config_package_id,source_config_item_id) -> platform_meta.config_package_items(config_package_id,id)` 同包复合 FK，全部 `ON DELETE RESTRICT`；同时安装 DEFERRABLE INITIALLY DEFERRED 的 `assert_f56_accepted_trust_projection_consistent()`，在提交点强制普通 item 接受摘要恒空、未 RELEASED special 为空、RELEASED special 恰 32 bytes、非空摘要不可改/不可清，并使 grant 行的既有 `trust_bundle_sha256` 等于其 grant source item 摘要。模块/grant/revocation source item 的三个唯一键及 revocation 两列同空同非空形状已分别由 090100/090200 建立，093300 不重复创建。该文件不增加新表或迁移编号。第 36、37 项分别建立影响面批次与处置项，一文件一表；两表都带法人且启用、强制 RLS，不进入 `unpoliced_table_registry`。第 38 号建立审批命令密文快照表；第 39 号在该表存在后向敏感字段登记表写一行，逐列值见表 33，回退只按三元组删除该登记行，不删除或解密业务快照。全部真实 14 位版本由 `docs/migration-catalog.md` 按依赖顺序冻结，本阶段计划不得自行猜测、复用或重编号；同 slug 在全部文档中只采用目录所列版本。第 1 号属 3a 段；其余均属 3b 段，第 36、37 项作为阶段 6 的硬前置必须在阶段 6 首个迁移之前完成。

F-56 终态把上述 093300 deferred graph 的附着面扩为 `config_packages/config_package_items/module_registrations/license_grants/legal_entities` 五表；原段列出的接受摘要检查只是其子集。提交点还必须强制：同 deployment 的首张 RELEASED grant 冻结唯一 `governance_legal_entity_id`，全部后继逐字相等，LIST scope 必含该法人，目标法人存在且持续 active；special 在 `DRAFT|PENDING_AUTOTEST|TEST_FAILED|TEST_PASSED` 时 `approval_legal_entity_id` 必须为空，进入 PENDING_APPROVAL 的 submit 事务才首次写入 derived governance id，且此后各态必须等于冻结值。首张 grant 的 derived id 取候选 signed payload，后续 grant/revocation/module action 取首次 RELEASED grant history；请求头若存在只能逐字相等，绝不能覆盖。该图同时扫描全部 RELEASED MODULE_PACKAGE history，强制 `package_id` 只映射一份 exact inner artifact，且 `(module_code,package_code,semver)` 只映射同一 `package_id` 与同一 exact inner；重复带回同一 inner 合法，冒用任一 identity 在提交时拒绝。

#### 3.3.2 公共列的适用口径

基线第 4 节的公共列清单按下列口径应用到平台表，逐表在下文标注。

- 纯技术表不带 `security_level` 与 `data_scope_tags`：`idempotency_keys`、`inbox_consumptions`、`upload_parts`、`number_sequences`、`audit_segments`、`audit_anchors`、`process_timers`。理由是它们不承载可被派生存储索引或按密级过滤的内容，加两列只会制造无人维护的常量列。
- 参与派生存储与密级过滤的表带这两列：`notifications`、`attachment_objects`、`attachment_versions`、`process_instances`、`process_tasks`、`approval_command_snapshots`、`impact_assessments`、`impact_disposition_items`。
- 仅追加表不带 `row_version`、`updated_at`、`updated_by`：`audit_events`、`outbox_events`、`dead_letters`、`process_steps`、`process_compensations`、`inbox_consumptions`、`scan_results`、`upload_parts`。其中 `outbox_events` 与 `dead_letters` 的状态列是投递控制列，其信封与载荷仅追加，见第 3.12.2 节澄清二。
- 上述仅追加表中，只有 `process_compensations` 带 `reverses_id`，指向被补偿的 `process_steps.id`。其余七张不设该列，理由与已同步的基线第 4 节规则见第 3.12.2 节澄清二。
- 部署级表不带 `legal_entity_id` 与 `data_scope_tags`：`platform_core.module_registrations`、`platform_core.license_grants`、`platform_core.feature_flags`、`platform_meta.config_packages`、`platform_meta.config_package_items`、`platform_meta.config_release_orders`。前三张按裁定 A-05 三列全不带；后三张按阶段 13 计划第 3.2.10 至 3.2.12 节带 `security_level`，不带另两列。六张表都不建行级安全策略，并按基线第 3.8 节的正向登记制在 `platform_core.unpoliced_table_registry` 中逐表登记一行，六行的 `admission_basis` 一律取 `SAME_FOR_ALL_ENTITIES`，即行在本部署内对全部法人取值相同；基线第 3.8 节原列的“不带 `legal_entity_id` 的表只有四类”这条封闭枚举与其中的全局配置字典一类已由阶段 2 撤销并由该登记表取代，本阶段不再据其归类。六行登记由第 35 号迁移写入，见第 3.3.1 节。

#### 3.3.3 索引命名的长度规则

基线第 3.10 节的索引命名在多列组合上会超过 PostgreSQL 的 63 字节标识符上限。本阶段登记一条确定性缩短规则，作为本基线未覆盖事项的新增决定：先按固定缩写表替换列段（`legal_entity_id` 缩为 `le`，`recipient_user_id` 缩为 `recipient`，`created_at` 缩为 `created`，`occurred_at` 缩为 `occurred`，`accounting_period_id` 缩为 `period`，`attachment_object_id` 缩为 `object`，`process_instance_id` 缩为 `instance`）；若仍超过 63 字节，截断到 55 字节并追加下划线与原全名 SHA-256 前 7 位十六进制。规则实现在迁移生成器中，同一输入恒定产出同一名字。

#### 3.3.4 逐表定义

以下每张表的行级安全策略一律按基线第 3.8 节的模板生成，不写变体，模板不再逐表重复。凡带 `legal_entity_id` 的表都 `ENABLE` 且 `FORCE` 行级安全，策略名为 `rls_<table>_le`。本阶段 33 张新表中，表 1 至表 24、表 31 至表 33 共 27 张带 `legal_entity_id` 并按模板建策略；表 25 至表 30 是按裁定 A-05 与 A-27 前移的六张部署级表，不带 `legal_entity_id`、不建策略，其可见性不随 `app.legal_entity_id` 变化。

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
| release_package_id | uuid | null，平台发布证明，属于基线第 3.3 节具名白名单；写入时锁定并校验 `platform_meta.config_release_orders` 的已签名发布结果 |
| signature_ref | text | null |
| published_at | timestamptz | null |
| is_active | boolean | not null default true |
| deactivated_at | timestamptz | null |
| row_version, created_at, created_by, updated_at, updated_by | | 按公共列 |

索引：`pk_process_definitions`；`ux_process_definitions_le_id` 在 `(legal_entity_id,id)` 上，供流程实例复合外键引用；`ux_process_definitions_le_code_version`；`ix_process_definitions_le_created`。

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
| business_key | text | null，长度不超过 200；只作 `business_object_type/business_object_id` 封闭多态对象的稳定路由键，不单独构成引用 |
| business_object_type | text | null |
| business_object_id | uuid | null |
| state | text | not null，ck 取值见状态机 |
| variables | jsonb | not null default '{}'；只允许流程定义逐键白名单声明的非敏感路由元数据，不得放业务命令、请求体或其可逆片段 |
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

索引：`pk_process_instances`；`ux_process_instances_le_id` 在 `(legal_entity_id,id)` 上，供全部同法人子对象复合外键引用；`ix_process_instances_le_created`；`ix_process_instances_le_state_next_wake_at`（调度取件）；`ix_process_instances_le_business_object_type_object_id`（按业务对象反查）；`ix_process_instances_le_definition_code_state`。

`variables` 只承载流程定义以 JSON Schema 逐键允许的非敏感路由元数据，例如业务对象判别与 id、关联/因果 id、owner module、scenario、action 及 `approval_command_snapshot_id`；字段密级必须小于 30。业务命令 DTO、HTTP 请求体、付款/账户/税号/身份明文、附件正文、密文副本、可逆片段及其未经密钥保护的摘要一律禁止进入 `variables`、task payload、Outbox payload 或审计 `before/after`。需要审批后执行的完整命令只写表 33 的 `command_enc`，其他位置仅持 snapshot id 与非敏感路由键；流程定义发布校验与实例启动写入校验共用同一变量白名单，越界以稳定验证错误拒绝并零写入。

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

索引：`pk_process_steps`；`ux_process_steps_le_id` 在 `(legal_entity_id,id)` 上；`ux_process_steps_le_instance_id` 在 `(legal_entity_id,process_instance_id,id)` 上，供补偿父链同时校验实例归属；`ux_process_steps_le_instance_idempotency_key`；`ix_process_steps_le_instance_step_no`；`ix_process_steps_le_created`。

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

索引：`pk_process_tasks`；`ux_process_tasks_le_id` 在 `(legal_entity_id,id)` 上，供同法人复合外键引用；`ix_process_tasks_le_created`；`ix_process_tasks_le_assignee_state_due_at`（待办列表，对应附录 A.1 的审批任务列表加载）；`ix_process_tasks_le_instance_state`；`ix_process_tasks_le_state_due_at`（SLA 扫描）。

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
| reverses_id | uuid | not null，与法人和 `process_instance_id` 组成真实复合外键 `(legal_entity_id,process_instance_id,reverses_id) -> platform_flow.process_steps(legal_entity_id,process_instance_id,id) ON DELETE RESTRICT` |
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

索引与候选键：`pk_audit_segments`；`ux_audit_segments_le_id`，列为 `(legal_entity_id,id)`，供锚点同法人复合外键引用；`ux_audit_segments_le_event_day`；`ix_audit_segments_le_created`；`ix_audit_segments_le_state_last_anchored_at`（锚定扫描）。公共 `created_by/updated_by` 均以 `(legal_entity_id,<user_id>)` 真实复合外键指向 `platform_authz.user_legal_entity_grants(legal_entity_id,user_id) ON DELETE RESTRICT`。

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
| actor_user_id | uuid | not null，与法人组成真实复合外键指向 `platform_authz.user_legal_entity_grants(legal_entity_id,user_id) ON DELETE RESTRICT` |
| actor_device_id | uuid | null，非空时以真实单列外键指向 `platform_core.user_devices(id) ON DELETE RESTRICT`，写事务另校验设备主体与当前法人授权 |
| action | text | not null |
| object_type | text | not null |
| object_id | uuid | null |
| object_version | bigint | null |
| before | jsonb | null |
| after | jsonb | null |
| reason | text | null，长度不超过 2000 |
| approval_ref | uuid | null |
| reauth_ref | uuid | null，非空时以真实单列外键指向 `platform_core.reauth_challenges(id) ON DELETE RESTRICT`，写事务另校验挑战主体与当前法人授权 |
| client | text | not null，ck 取值 `win`、`mac`、`ios`、`android`、`portal`、`ops`、`system` |
| occurred_at | timestamptz | not null |

索引与候选键：`pk_audit_events`；`ux_audit_events_le_event_id`，列为 `(legal_entity_id,event_id)`，供验证结果同法人复合外键引用；`ux_audit_events_le_event_day_seq`；`ix_audit_events_le_occurred`（代替基线的 `_created_at` 基线索引，本表无 `created_at`）；`ix_audit_events_le_object_type_object_id_occurred`（按对象检索）；`ix_audit_events_le_actor_user_id_occurred`（按操作者检索）；`ix_audit_events_le_action_occurred`（按事件类型检索）。全部索引名按第 3.3.3 节的缩短规则产出。`approval_ref` 是具名平台证明白名单，`object_type/object_id` 是封闭多态对象；除此之外不得把固定目标引用降级为应用校验。

`client` 的取值集合在基线第 5.6 节的六个之外增加 `system`，用于系统上下文写入的审计事件，如锚定任务与保留期清理。登记为本阶段新增决定。

`seq` 使用一个全局 `bigserial`。`nextval` 不随事务回滚，因此段内 `seq` 会出现空洞。链验证按段内 `seq` 升序连接判定，不要求 `seq` 连续，这一点写入验证算法与验证报告口径，见第 3.4.3 节。

**表 10 `platform_audit.audit_anchors`**

| 列 | 类型 | 约束 |
|---|---|---|
| id | uuid | not null，pk |
| legal_entity_id | uuid | not null |
| audit_segment_id | uuid | not null，与法人组成 `fk_audit_anchors_audit_segments`，指向 `audit_segments(legal_entity_id,id) ON DELETE RESTRICT` |
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
| attempts | int | not null default 0，ck 取值 0..9；每次外部签名或证据写失败后以 CAS 加一 |
| available_at | timestamptz | not null default now()；仅到期行可被重试扫描 |
| last_error | text | null |
| row_version, created_at, created_by, updated_at, updated_by | | 按公共列 |

状态形状 CHECK 固定为：`PENDING_SIGN` 时 `signature/signed_at/evidence_path/evidence_written_at` 全空；`SIGNED` 时 `signature/signed_at` 非空且证据两列为空；`EVIDENCED` 时签名与证据四列全非空；`FAILED` 允许保留已经成功的签名两列，但签名与 `signed_at` 必须同空或同非空，证据两列必须同空。任何状态均满足 `attempts BETWEEN 0 AND 9`。索引与候选键：`pk_audit_anchors`；`ux_audit_anchors_le_id`，列为 `(legal_entity_id,id)`；`ux_audit_anchors_le_segment_anchor_seq`；`ix_audit_anchors_le_created`；`ix_audit_anchors_le_state_available_at`，列为 `(legal_entity_id,state,available_at,id)`，供重启后的到期重试扫描。公共 `created_by/updated_by` 指向用户法人授权的同法人真实复合外键。

**表 11 `platform_audit.audit_verifications`**

| 列 | 类型 | 约束 |
|---|---|---|
| id | uuid | not null，pk |
| legal_entity_id | uuid | not null |
| range_from | date | not null |
| range_to | date | not null |
| single_event_id | uuid | null，非空时与法人组成真实复合外键指向 `audit_events(legal_entity_id,event_id) ON DELETE RESTRICT` |
| state | text | not null，ck 取值 `QUEUED`、`RUNNING`、`PASSED`、`FAILED`、`ABORTED` |
| segments_total | int | not null default 0 |
| segments_passed | int | not null default 0 |
| first_failure_event_id | uuid | null，非空时与法人组成真实复合外键指向 `audit_events(legal_entity_id,event_id) ON DELETE RESTRICT` |
| first_failure_reason | text | null |
| report | jsonb | null |
| requested_by | uuid | not null，与法人组成真实复合外键指向 `platform_authz.user_legal_entity_grants(legal_entity_id,user_id) ON DELETE RESTRICT` |
| started_at / finished_at | timestamptz | null |
| row_version, created_at, created_by, updated_at, updated_by | | 按公共列 |

索引与候选键：`pk_audit_verifications`；`ux_audit_verifications_le_id`，列为 `(legal_entity_id,id)`；`ix_audit_verifications_le_created`；`ix_audit_verifications_le_state_created`。公共 `created_by/updated_by` 同样指向用户法人授权的同法人真实复合外键。

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
| notice_type | text | not null，ck 取值为 PRD 第 10.5.2 节十类中的九类提醒事项码（撤下「许可临期与宽限期告警」，见第 3.4.5 节） |
| title_template | text | not null，长度不超过 200 |
| body_template | text | not null，长度不超过 2000 |
| push_title_template | text | null |
| push_body_template | text | null |
| severity | text | not null，ck 取值 `INFO`、`WARN`、`CRITICAL` |
| release_package_id | uuid | null，平台发布证明，属于基线第 3.3 节具名白名单 |
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
| user_id | uuid | not null；与法人组成复合外键指向 `platform_authz.user_legal_entity_grants(legal_entity_id,user_id)` |
| device_id | uuid | not null；单列真实外键指向全局身份表 `platform_core.user_devices(id)`，写事务另校验该设备属于 `user_id` 且授权当前法人 |
| platform | text | not null，ck 取值 `ios`、`android` |
| token_enc | bytea | not null，按法人密钥域字段级加密的令牌密文 |
| token_key_ref | text | not null，记录密钥标识与版本，与 token_enc 同生共死 |
| token_bidx | bytea | not null，令牌盲索引，唯一调用为 `derive_blind_key(legal_entity_id,"platform_msg.push_registrations.token@30",plaintext)` 的完整 32-byte 返回值 |
| is_active | boolean | not null default true |
| deactivated_at | timestamptz | null |
| last_success_at / last_failure_at | timestamptz | null |
| consecutive_failures | int | not null default 0 |
| row_version, created_at, created_by, updated_at, updated_by | | 按公共列 |

索引：`pk_push_registrations`；`ux_push_registrations_le_user_id_device_id_platform`；`ix_push_registrations_le_created`；`ix_push_registrations_le_user_id_is_active`。

推送令牌属于规格第 7.8 章意义上的行内敏感属性，按字段级密钥加密存储，密文与密钥引用分列承载；同一令牌的查重用同一法人密钥域下的盲索引 `token_bidx` 承担，密文不直接进入任何索引与唯一约束，这一点是规格第 7.8 章的明确要求。`token_bidx` 上不建唯一约束，本表的唯一性只由 `ux_push_registrations_le_user_id_device_id_platform` 承担。盲索引 selector 固定为 `platform_msg.push_registrations.token@30`；写入与查重必须从同一登记/effective-level resolver 得到 scope 30，裸 `platform_msg.push_registrations.token`、其他 scope、别名或 caller 自报 selector 全拒。物理列命名按裁定 A-28 的全库唯一一套取 `<语义>_enc`、`<语义>_key_ref` 与 `<语义>_bidx`，本阶段不另起 `_cipher` 或 `_ciphertext` 一套。按同一裁定，凡受字段级密钥保护的列必须在 `platform_core.sensitive_field_registry` 有登记行，否则阶段 2 第 4.6 节的销毁证明会把本列算进仍可读范围、阶段 2 的 `db/checks/11` 也不检查本列、`blind_index_column` 亦无出处，因此第 34 号迁移向该表插一行，按裁定 C-06 冻结的十一列逐列取值为：`schema_name` 取 `platform_msg`，`table_name` 取 `push_registrations`，`column_name` 取逻辑列名 `token` 且不带 `_enc` 后缀，`category` 取 `PAYMENT_TOKEN`，`security_level` 取 30，`is_field_encrypted` 取 `true`，`blind_index` 取 `EXACT`，`blind_index_column` 取 `token_bidx`，`mask_style` 取 `FULL`，`normalization` 取 `NONE`，`release_ref` 取 `MIGRATION:V20261013092600`；`created_by` 按裁定 A-02 取 `ep_foundation::SYSTEM_PRINCIPAL_ID`，该文件的 `-- rollback:` 段按 `schema_name` 与 `table_name` 删除该行。两处取值另给理由：`mask_style` 不取 `KEEP_LAST_4`，因为本表没有 `token_tail` 列，而后四位只能取自 `<column_name>_tail`；`normalization` 取 `NONE`，因为推送令牌是大小写敏感的不透明串，取 `TRIM_NFKC` 会改写 `derive_blind_key` 的入参，使同一令牌算出两个盲索引。

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

本表是基线第 3.6 节允许带删除标记的两类对象之一。删除标记不影响历史引用与历史版本，物理删除只能由处置流程经专用路径与专用账号发起；本阶段只定义 `DisposalPort` 端口并注册处置受理路由，不注入任何实现，本阶段至阶段 13 之间的物理删除请求直接拒绝并开一条降级窗口，实现与注入行由阶段 14 的 `OpsDisposalService` 交付并在注入后关窗，见裁定 A-22 与第 3.4.7 节。

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
| artifact_eligible | boolean | generated always as (`state in ('AVAILABLE','SUPERSEDED')`) stored；只用于后续制品引用的真实复合外键，调用方不得写入 |
| storage_path | text | not null，相对 `published` 根的路径 |
| content_hash | text | not null，明文 SHA-256 的 64 位小写十六进制 |
| size_bytes | bigint | not null，大于 0 且不超过配置上限 |
| content_type | text | not null，服务端识别结果 |
| declared_content_type | text | not null，客户端声明值 |
| key_domain_ref | text | not null，法人密钥域引用 |
| dek_ref | text | not null，开始时固定的 `DataKeyHandleV1::canonical_ref()`；唯一 wire=`data-key://<lowercase-data-key-uuid>#<u16非零十进制版本>`，版本无前导零且范围 1..65535，必须与 EPA1 每块 EPC1 的 2-byte u16 data-key version 相等 |
| encryption_algorithm | text | not null，ck 取值 `AES_256_GCM` |
| available_at | timestamptz | null |
| upload_session_id | uuid | null |
| row_version, created_at, created_by, updated_at, updated_by | | 按公共列 |

索引与候选键：`pk_attachment_versions`；`ux_attachment_versions_le_id`，列为 `(legal_entity_id,id)`，供仅需固定版本身份的后续复合外键引用；`ux_attachment_versions_artifact_identity`，列为 `(legal_entity_id,id,content_hash,size_bytes,artifact_eligible)`，供客户端制品、扩展制品与其后签名模块同时证明“同法人、同一不可漂移版本、同哈希、同大小且已可发布”；`ux_attachment_versions_le_object_version_no`；`ix_attachment_versions_le_created`；`ix_attachment_versions_le_state_created`（收敛与回收）；`ix_attachment_versions_le_available_at`（水位输入）。`AVAILABLE -> SUPERSEDED` 不改变 `artifact_eligible=true`，因而不破坏既有制品引用；`PENDING`、`QUARANTINED` 与 `FAILED` 均为 false，不能成为制品父行。版本状态迁移不得从 true 档退回 false 档。

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
| inspector | text | not null，ck 取值 `TYPE_SNIFF`、`STRUCTURE`、`VIRUS_ICAP` |
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
| package_id | uuid | null；已安装态非空，等于内层 manifest `package_id` |
| package_code | text | null；已安装态非空，正则 `^[a-z][a-z0-9._-]{0,63}$` |
| package_version_major / package_version_minor / package_version_patch | int | null；已安装态三列均非空且各在 0..65535 |
| package_payload_sha256 | bytea | null；已安装态 32 bytes，等于 `SHA-256(JCS(manifest))` |
| package_signature | bytea | null；已安装态保存 detached CMS exact bytes，长度 1..1,048,576 |
| package_signer_subject | text | null；已安装态非空，exact `spki-sha256:<64 lowerhex>`，逐字命中签名 `DeploymentManifestV1.license_trusted_signer_subjects`；显示 DN 只由证书派生展示 |
| package_signed_at | timestamptz | null；已安装态非空、UTC 秒精度，取 manifest `issued_at` |
| module_contract_version | int | null；已安装态非空且 `between 1 and 2147483647`；Rust 保留 u32，但 descriptor/product manifest/package/parser 在入库前 checked conversion，拒绝大于 i32::MAX |
| module_contract_sha256 | bytea | null；已安装态 32 bytes，必须等于签名产品 manifest 的编译期契约摘要 |
| min_platform_version | text | null；已安装态为 canonical `MAJOR.MINOR.PATCH` |
| max_platform_version_exclusive | text | null；可空 canonical `MAJOR.MINOR.PATCH`，非空时严格大于 min |
| released_on | date | null；已安装态非空，受永久许可维护期或订阅有效期守卫 |
| source_config_package_id | uuid | null；已安装态非空，来源配置包；FK 由 093300 后补 |
| source_config_item_id | uuid | null；已安装态非空且全表唯一，与上列组成同包复合 FK，由 093300 后补 |
| enabled_at | timestamptz | null；最近一次 ENABLE 时点 |
| disabled_at | timestamptz | null；INSTALL 或最近一次 DISABLE 时点 |
| last_transition_reason | text | null；已安装态 1..1000 UTF-8 bytes，取签名动作项 reason |
| row_version, created_at, created_by, updated_at, updated_by | | 按公共列 |

`ck_module_registrations_projection_shape` 强制 `NOT_INSTALLED` 时 package/source/安装与转换时间投影全空；两个 INSTALLED 状态下 F-56 的 package identity、签名、契约、`min_platform_version`、released_on、source 与 reason 投影全非空，只有 `max_platform_version_exclusive` 可为 NULL。该 NULL 唯一表示无上界；非空时才要求严格大于 min 且当前产品版本小于 max。时间投影唯一规则为：INSTALL 同一提交时点写 `installed_at=state_changed_at=disabled_at` 且 `enabled_at=null`；ENABLE 只把 `state_changed_at=enabled_at` 更新为本次提交时点并保留 installed/disabled；DISABLE 只把 `state_changed_at=disabled_at` 更新为本次提交时点并保留 installed/enabled；UPGRADE/ROLLBACK_VERSION 只替换 package/source/reason 并更新 `state_changed_at`，不得抹除 installed/enabled/disabled。表中不存在制品正文、附件、路径、URL、DLL/EXE/脚本/SQL/WASM/容器或 hook 列。索引：`pk_module_registrations`；`ux_module_registrations_module_code`；`ux_module_registrations_source_config_item_id`（NULL 不冲突）；`ix_module_registrations_install_state_created_at`。

090100 建表后在同一迁移写入且只写入下列 15 行；全部 `install_state=NOT_INSTALLED`，package/source/安装与动作时间投影全为 null，`row_version=1`，`created_by=updated_by=SYSTEM_PRINCIPAL_ID`。`id/module_code/display_name` 是产品目录身份，后续升级不得改写、删除或添加第 16 行：

| id | module_code | display_name |
|---|---|---|
| `00000000-0000-7000-8000-000000000601` | `mdm` | 主数据管理 |
| `00000000-0000-7000-8000-000000000602` | `crm` | 客户关系管理 |
| `00000000-0000-7000-8000-000000000603` | `cpq` | 配置、定价与报价 |
| `00000000-0000-7000-8000-000000000604` | `clm` | 合同生命周期管理 |
| `00000000-0000-7000-8000-000000000605` | `sales` | 销售与订单 |
| `00000000-0000-7000-8000-000000000606` | `procure` | 采购管理 |
| `00000000-0000-7000-8000-000000000607` | `inventory` | 库存管理 |
| `00000000-0000-7000-8000-000000000608` | `costing` | 成本管理 |
| `00000000-0000-7000-8000-000000000609` | `project` | 项目管理 |
| `00000000-0000-7000-8000-000000000610` | `service` | 售后服务 |
| `00000000-0000-7000-8000-000000000611` | `finance` | 收付款与往来 |
| `00000000-0000-7000-8000-000000000612` | `ledger` | 总账与结账 |
| `00000000-0000-7000-8000-000000000613` | `invoice` | 发票管理 |
| `00000000-0000-7000-8000-000000000614` | `portal` | 供应商门户 |
| `00000000-0000-7000-8000-000000000615` | `reporting` | 报表与分析 |

**表 26 `platform_core.license_grants`**（3b 段，部署级）

| 列 | 类型 | 约束 |
|---|---|---|
| id | uuid | not null，pk；逐字等于 payload `grant_id` |
| license_no | text | not null，1..128 UTF-8 bytes；续期可复用同一编号，故不唯一 |
| deployment_id | uuid | not null，逐字等于签名部署清单与 Stage 14 current deployment id |
| governance_legal_entity_id | uuid | not null；首张 RELEASED grant 冻结的部署治理法人，真实 FK 指向 `platform_core.legal_entities(id) ON DELETE RESTRICT`；后继 grant 必须逐字相同且该法人必须持续 active |
| issued_to | text | not null，1..256 UTF-8 bytes |
| license_kind | text | not null，ck 取 `PERPETUAL`、`SUBSCRIPTION` |
| issued_at | timestamptz | not null，UTC 秒精度 |
| valid_from | date | not null |
| valid_to | date | null；SUBSCRIPTION 非空且不早于 valid_from，PERPETUAL 必为空 |
| maintenance_valid_to | date | null；SUBSCRIPTION 等于 valid_to，PERPETUAL 为空或不早于 valid_from |
| legal_entity_scope | text | not null，ck 取 `ALL`、`LIST` |
| legal_entity_ids | uuid[] | not null；ALL 恰为空，LIST 为按 wire bytes 排序去重的 1..1024 个 UUID，且必须包含 `governance_legal_entity_id` |
| legal_entity_limit | int | not null，1..1,000,000 |
| named_user_limit | int | not null，1..1,000,000 |
| registered_device_limit | int | not null，1..1,000,000 |
| module_codes | text[] | not null，按 wire bytes 排序去重且非空，元素取 15 个 `ModuleCode` |
| entitlement_codes | text[] | not null，按 wire bytes 排序去重、可空数组，元素只取 `F55_LOCAL_AI`、`F55_MCP` |
| payload_sha256 | bytea | not null，32 bytes，等于从本行重建 grant payload 后的 `SHA-256(JCS(payload))` |
| signature | bytea | not null，detached CMS exact bytes，长度 1..1,048,576 |
| signer_subject | text | not null，exact `spki-sha256:<64 lowerhex>`，逐字命中签名 `DeploymentManifestV1.license_trusted_signer_subjects` 唯一 roster；显示 DN 不参与身份比较 |
| trust_bundle_sha256 | bytea | not null，32 bytes；该 grant 首次 RELEASE 时实际用于内层 CMS 验链的离线 release trust bundle 摘要，必须逐字等于 `grant_source_config_item_id` 所指 RELEASED item 的 `accepted_trust_bundle_sha256`；是不可变接受证据，不要求永久等于以后轮换出的当前 bundle 摘要 |
| supersedes_grant_id | uuid | null，自 FK `ON DELETE RESTRICT`；首张为空，续期必须指向当时 current 直接前驱且不得成环 |
| superseded_at | timestamptz | null；移出 current slot 时非空且不早于新 grant issued_at 的接受时点 |
| current_slot | smallint | null；只允许 0/null，普通唯一键保证至多一张 current |
| last_trusted_at | timestamptz | not null，UTC 秒精度；新 grant 初值固定为 `max(pre_import_trusted_now,candidate.issued_at)`，此后只能单调前进 |
| revoked_at | timestamptz | null；已接受撤销的本地可信时点 |
| revocation_id | uuid | null；撤销组非空时唯一 |
| revocation_issued_at | timestamptz | null，UTC 秒精度 |
| revocation_reason_code | text | null，只取 `CONTRACT_ENDED`、`REISSUED`、`COMPROMISED`、`CUSTOMER_REQUEST` |
| revocation_payload_sha256 | bytea | null；撤销组非空时 32 bytes，按列重建撤销 payload 后复验 |
| revocation_signature | bytea | null；撤销组非空时为 detached CMS exact bytes，长度 1..1,048,576 |
| revocation_signer_subject | text | null；撤销组非空时 exact `spki-sha256:<64 lowerhex>` 并逐字命中签名 `DeploymentManifestV1.license_trusted_signer_subjects` |
| grant_source_config_package_id | uuid | not null；grant 来源配置包，FK 由 093300 后补 |
| grant_source_config_item_id | uuid | not null 且唯一；与上列组成同包复合 FK，由 093300 后补 |
| revocation_source_config_package_id | uuid | null；与 revocation item 同空同非空，FK 由 093300 后补 |
| revocation_source_config_item_id | uuid | null；非空时唯一且与上列组成同包复合 FK，由 093300 后补 |
| row_version, created_at, created_by, updated_at, updated_by | | 按公共列 |

`ck_license_grants_kind_dates`、`ck_license_grants_scope`、三项 limit CHECK、摘要/签名长度 CHECK、`ck_license_grants_current_shape` 与 `ck_license_grants_revocation_shape` 逐项落实上表；数组 canonical 排序/去重、JCS 重建、CMS/链/撤销、deployment 绑定、可信时间和续期直接后继由 `LicenseGrantApplier` 在同一持锁事务复验，不能由数据库布尔值或任意缓存替代。GRANT、REVOCATION 与 MODULE_PACKAGE 的 `apply` 只允许在下文 F-56 special 全局锁序已经建立的事务内调用；applier 可幂等地再次请求同一 transaction-level `platform-license-current` advisory lock，但不得把“第一条业务 SQL”误解为 applier 内较晚的第一句。取得全局锁后才重读全部 current/history 与已接受撤销，不得先查候选、只锁一条可能不存在的 current 行，或依靠唯一键解释业务竞态。锁内按下段 `TrustedClockV1` 唯一公式求不含候选的 `pre_import_trusted_now`；接受新 grant 时写 `last_trusted_at=max(pre_import_trusted_now,candidate.issued_at)`，接受 REVOCATION 时把命中 current 单调推进到 `max(existing_last_trusted_at,pre_import_trusted_now,candidate.issued_at)`。受控续期再把旧 current 的 `current_slot` 置空并写 `superseded_at`、插入新 current；两个并发首发、同一前驱的并发续期及续期/撤销竞态都在锁内重算，只有一条候选合法提交，输家返回 `PLATFORM.CONFIG_PACKAGE.SPECIAL_ITEM_SHAPE_INVALID` 且不得泄漏 SQLSTATE 或任选旧快照。

每个 core/worker 进程只有一个 `TrustedClockV1`。数据库连接后、public readiness 前必须先验证相关 audit hash chain，再读取持久证据与一次 `system_utc_at_start`；唯一启动锚点为 `process_anchor_utc=max(initial-governance bootstrap committed_at,current/history grant.issued_at,已接受 revocation.issued_at,全部 license_grants.last_trusted_at,全部已验证 hash-chain 且结构有效的 trusted-time checkpoint.trusted_now,system_utc_at_start)`。尚无某类证据时只从 max 集合移除该项，不得把空集解释为 epoch；未通过审计链验证的 bootstrap/checkpoint 严禁纳入。随后捕获 OS monotonic `Instant`；进程内候选恒为 `process_anchor_utc+monotonic_elapsed`，不因 wall-clock 回拨下降。每次 query/apply 在事务开始后只读一次 wall clock，唯一公式为 `trusted_now=max(上述持久证据,system_utc_now,process_anchor_utc+monotonic_elapsed)`，`trusted_date` 取该 UTC 日历日。普通 query 只计算、不写行；但启动 readiness 前，以及 import/autotest/submit/approve/sign/create-release-order/execute 每个会推进 F-56 special package 状态的关口，都必须在 `pre_idempotency_lock=LICENSE_CURRENT_EXCLUSIVE` 已取得的同一 advisory xact lock 内重读 current，并以 CAS 把 `last_trusted_at` 推进到本次 `trusted_now`，再完成原动作。reject 固定 `NONE` 且不执行本项 trusted-time CAS。job-worker target cadence 不得超过 240 秒，每次 checkpoint 都固定取 exclusive；current 行只在本次值严格增加时 CAS，append-only audit 的同 slot 去重与不可更新语义以下段为准。连续 checkpoint 缺失或 wall clock 相对 monotonic trajectory 偏差超过 300 秒发不可抑制安全告警；进程在两个 checkpoint 间崩溃并以已回拨 wall clock 重启时，未持久观察窗口小于 300 秒，必须如实披露而不得宣称 NTP/TPM 级防篡改。已受控持久化的错误前跳不得自动回拨或 direct SQL 重置，只能按 Stage 14 可信备份整体恢复数据库与审计链后重算。

checkpoint 审计 action 固定 `LICENSE_TRUSTED_TIME_CHECKPOINT`，after 是 strict-JCS 闭集 `{schema_version:1,purpose:"EP-LICENSE-TRUSTED-TIME-CHECKPOINT-V1",deployment_id,slot_utc,trusted_now,current_grant_id}`，`schema_version` 是 JSON number `1`。`slot_utc` 唯一算法为把 `trusted_now` 的 Unix seconds 取 `floor(seconds/240)*240`，再输出 canonical RFC 3339 UTC whole-second；分钟可为 `00/04/08/...`，跨小时仍必须按 Unix epoch 计算，禁止五分钟取整。`ensure_checkpoint` 入口必须在同一 license exclusive lock 内、任何业务 mutation 之前，只读一次本次 `trusted_now`、唯一 current grant id/null 与命中 current 的 revocation id/null，据此冻结 slot/payload snapshot；后续 grant/revocation 投影写入不得改这份 snapshot，terminal `AuditWriter` 只能使用它，禁止重读 current 或重算 trusted_now。

零行创建分支才预分配新 UUIDv7 `event_id`；一行复用分支不得分配或插入事件。新 checkpoint 的完整 envelope 固定 `legal_entity_id=<冻结治理法人>`、`actor_user_id=SYSTEM_PRINCIPAL_ID` 且命中该法人的 ACTIVE SYSTEM grant、`actor_device_id=null`、`action='LICENSE_TRUSTED_TIME_CHECKPOINT'`、`object_type='platform.license_trusted_time'`、`object_id=deployment_id`、`object_version=null`、`before=null`、`after=<上述入口 snapshot exact payload>`、`reason=null`、`approval_ref=null`、`reauth_ref=null`、`client='system'`、`occurred_at=<入口捕获的 trusted_now>`；`event_day/seq/prev_hash/hash` 只由 `AuditWriter` 的既有分段链算法派生。`object_id` 始终是已验签 deployment UUID，不随 current grant 换槽；数据库当前时间、默认 object 或 mutation 后重算值均拒绝。

exclusive lock 内只能按 `(action,after->>'purpose',after->>'deployment_id',after->>'slot_utc')` 查询本 deployment/slot：零行才由 `AuditWriter` 以冻结 snapshot 追加本次 payload；一行必须保留既有 exact bytes、绝不 UPDATE，只核 schema/purpose/deployment/slot、创建时 current id 形状与 hash chain；多行或不等失败关闭。同 slot 后续事务引用既有 checkpoint，不要求其 trusted_now 等于本次较新值；current 存在时其 `last_trusted_at` 仍可独立 CAS 至本次 trusted_now。语义键恰为 `license-trusted-time:v1:<lowercase-deployment-id>:<slot_utc>`，audit 的 legal_entity 固定治理法人、actor 为该法人 SYSTEM grant、client=system。job target cadence 不得超过 240 秒，special 推进也必须 ensure 当前 slot；因此有 uptime 证据时相邻持久 checkpoint gap 小于或等于 300 秒才 PASS，大于 300 秒失败。零 current 下首发与必要 checkpoint 同事务，不会以“没有 current 可更新”为由自锁。

checkpoint 的治理法人来源按严格优先级取：唯一 current grant → 已验证 initial-governance audit/receipt → 本次首张 GRANT candidate；低优先来源不得覆盖高优先证据。唯一例外是显式 non-production、bootstrap absent、zero-current 且 `legal_entities` 零行同时成立时，public readiness 可不写 checkpoint，固定呈现 `RESTRICTED/NO_CURRENT_GRANT`，checkpoint worker 保持 dormant，且该部署永久不能形成 Stage 14 evidence。此后首张 GRANT 的 whole exclusive 事务只能从 candidate 派生治理法人，要求该法人已存在且 operator 授权成立，并在同一事务创建首条 checkpoint；任一不符零推进。production 强制 bootstrap，readiness checkpoint 无此豁免。

`governance_legal_entity_id` 不是安装范围或可配置浏览法人。首张 GRANT 在 submit 前必须指向已存在且 active 的法人，首张 RELEASE 同事务冻结该 deployment 的值；后继 GRANT 必须相等，LIST scope 必含该值。DRAFT 至 TEST_PASSED 只在每个推进命令内从首张候选或首次 RELEASED history 派生 `governance_context_id`，不得提前写 `approval_legal_entity_id`；命令必须证明当前受信 session/operator 对该法人具有本动作权限，请求头若提供法人只允许与派生值相等。submit 同事务才首次把 derived id 写入 approval 列，PENDING_APPROVAL 及以后不得改变。ServerAdmin 选择、环境变量或配置都不能覆盖；首次 history/source/signature 不唯一、授权缺失或损坏时所有 special 推进失败关闭。法人停用命令必须先证明不存在以其为冻结治理法人的 deployment，删除则由真实 FK `ON DELETE RESTRICT` 阻断。

fresh production 的零 current 首装唯一入口复用既有 `ep-migrate apply`，参数闭集为 `--initial-governance-bootstrap=<bootstrap.jcs> --initial-license-package=<license.epcfg> --receipt-out=<directory>`；这不是新子命令，不新增端点、服务、监听、表或迁移。三个参数在生产 fresh install 必填，只允许全部迁移完成后、九个常驻服务开放 public readiness 前由 Authenticode 验证通过且 PE digest 命中当前签名产品清单的 `ep-migrate` 和既有迁移账户执行；开发/测试空库可省略但不能取得 Stage 14 发布证据。三个 path 不是任意路径：在验证 signed deployment id 为 canonical lowercase UUID 后，根固定为 `C:\ProgramData\EnterprisePlatform\evidence\stage14\initial-governance\<lowercase-deployment-id>\`；前两者必须分别逐字归一到该根的 `bootstrap.jcs` 与 `license.epcfg`，`receipt-out` 必须逐字归一到该根，输出文件名固定 `initial-governance.receipt.v1.jcs`。目录 owner 为 SYSTEM、DACL PROTECTED；显式 inheritable allow ACE 恰为 SYSTEM、BUILTIN\Administrators、`NT SERVICE\ep-ops` 各 FullControl，`NT SERVICE\ep-core` 只取 `FILE_GENERIC_READ|FILE_TRAVERSE|READ_CONTROL|SYNCHRONIZE`，其余包括 Users、Authenticated Users、Everyone、ep-worker、ep-ai、ep-integ、ep-plugin 均无 ACE；ep-core 明确不得 write/delete/WRITE_DAC/WRITE_OWNER。三条路径都用 fixed-root safe handle，拒绝 UNC/device/reparse/ADS/hardlink、8.3 alias、case drift 或任何路径漂移。输入必须已在固定名就位；receipt 只能 `CREATE_NEW`、`FlushFileBuffers`、关闭再 safe-handle readback，不能覆盖或写 sidecar。

`bootstrap.jcs` 最大 1,048,576 bytes、strict RFC 8785 JCS，root exact 为 `{body,authorizations}`。body exact 为 `schema_version=1,purpose="EP-INITIAL-GOVERNANCE-BOOTSTRAP-V1",bootstrap_id,deployment_id,deployment_manifest_sha256,initial_license_archive_sha256,issued_at,expires_at,legal_entity,operators`；有效窗大于 0 且至多 24 小时。`legal_entity={id,key_domain_id,code,entity_no,name,short_name}`，两个 id 都是 canonical UUID，`id` 必须等于候选 grant 的治理法人，`key_domain_id` 必须全库未占用并作为下述 PROVISIONING 行的逐字主键；`operators` 恰含按角色排序的 `CONFIG_OPERATOR|SECURITY_APPROVER` 两项，每项 exact 为 `{bootstrap_role,user_id,login_name,employee_no,display_name,client,device_id,signer_subject}`，两个 user/login/device/SPKI 全互异，client 只取 win/mac。authorizations 同样恰两项并各以客户安全管理员证书 detached CMS 签 `JCS(body)`；signer 与 operator 对应、彼此不同、命中签名部署清单的客户安全管理员证书闭集，并复用该清单双 CMS 的算法、链、CRL、属性和时间规则。bootstrap/body/license/deployment digest 或顺序任一不符退出 78、零写入。

任何写入前，工具须 safe-handle 读取并完整验证 `license.epcfg` 的 F-56 container、ACTIVE inner/outer、首张 GRANT（`supersedes_grant_id=null`）、deployment/governance/scope 绑定，并要求 archive SHA-256 等于 body；它只做首装资格验证，不能代替后续应用内 import/审批/RELEASE。fresh 数据库前置 exact 为：`legal_entities` 零行；`user_accounts` 恰一行且 id/account_kind 都是既有 `SYSTEM_PRINCIPAL_ID/SYSTEM` seed；`user_credentials`、`user_password_history`、`user_devices`、`sessions`、`reauth_challenges`、`login_attempts`、`account_lockouts`、`breakglass_activations` 零行；法人授权、角色绑定、审批链等法人 authz 业务行零行；`key_domains/data_keys` 法人密钥域业务行零行；license grant 与 config package/item/order 零行；其余 SYSTEM、deployment manifest/projection 与 migration history seed 逐项等于当前签名 schema/product 清单。任一非空、缺项或漂移退出 78，不存在 `--force`、覆盖、删除或任选存量分支。治理法人 `entity_no` 必须是两位数字，其余字段沿用 `legal_entities` 既有限制，时区/币种固定使用现有 `Asia/Shanghai/CNY` 默认。

唯一 bootstrap PostgreSQL 事务只创建该 active 法人、以 signed `key_domain_id` 为主键且 `domain_kind=LEGAL_ENTITY,state=PROVISIONING` 的密钥域；该行立即写 provider-independent `kek_ref="kms://ep/v1/deploy/<lowercase-deployment-id>/domain/<lowercase-key-domain-id>/kek/1"`、`kek_version=1,provisioned_at=null`，locator 只预定对象身份而不证明 KMS 已建。事务另建两名不同且 ACTIVE/强制 MFA 的 EMPLOYEE、各自限于治理法人的 ACTIVE Win/Mac device、console password 与 CMS leaf X509 credential；随后必须先建恰三条 ACTIVE `user_legal_entity_grants`，映射为同一治理法人下的 SYSTEM_PRINCIPAL_ID、CONFIG_OPERATOR user 与 SECURITY_APPROVER user，id 各为新 UUIDv7，`granted_by=SYSTEM_PRINCIPAL_ID,granted_from=<本事务 committed_at 的 UTC date>,granted_to=null`，除此之外该表零行。只有三行对复合 FK 可见后才建两条用户角色绑定并调用既有 deterministic catalog 的 `ApprovalChainProvisioner::provision_defaults`；绑定的法人/user/granted_by 逐字命中对应 grant。角色固定为 `F56_CONFIG_OPERATOR`（duty CONFIG，权限 exact `lowcode.config_package.view|import|autotest|submit|sign` 与 `lowcode.config_release.view|submit|execute`）和 `SECURITY_ADMIN`（duty SECURITY，权限 exact `lowcode.config_package.view|approve`），默认 CONFIG_RELEASE 链只指后者；两人不得互授、自审或合并。两个密码只经关闭 echo 的本机 `ReadConsoleW` 分别二次确认，拒绝 stdin redirect、argv/env/file，按身份模块唯一 Argon2id policy 写 PHC；任何输出、日志、审计或 receipt 不含密码/hash。该事务绝不调用 KMS、创建外部 key material、插入 data_keys 或把 key domain 标 ACTIVE；数据库写、链、人员、设备、凭据、授权或审计失败才由 PostgreSQL 整笔回滚。

bootstrap 事务提交后，core-server 必须在任何 public readiness 前以 signed `bootstrap_id/key_domain_id` 调用阶段 2 既有 `KeyDomainProvisioner` resume；该 orchestrator 只经独立 `KmsKeyMaterialProvisioner::{ensure_kek,generate_detached_data_key,readback_wrapped_data_key}` 操作上述 logical locator，依次为 `FIELD|BLIND_INDEX|ATTACHMENT|ARCHIVE` × `10|20|30|40` exact 16 个 tuple 生成并回读 wrapped DEK。DEK 只以数据库 `wrapped_key` 耐久化，任何明文/file/env/provider-private locator 都不得落盘。16 项全部 readback 后，唯一数据库事务才插入 exact 16 条 ACTIVE data_keys、把同一 PROVISIONING domain 置 ACTIVE/写 provisioned_at，并以 audit terminal batch 写唯一 `action='platform.key_domain.activated.v1'`；完整 envelope 还固定 `event_id` 为本事务 terminal batch 前预分配的新 UUIDv7，`legal_entity_id=key_domains.legal_entity_id`，`actor_user_id=SYSTEM_PRINCIPAL_ID` 且命中该法人的 ACTIVE SYSTEM 法人授权，`actor_device_id=null`，`object_type='platform.key_domains'`，`object_id=key_domain_id`，`object_version=ACTIVE row_version`，`before=null`，`after=上述 payload`，`reason/approval_ref/reauth_ref=null`，`client='system'`，`occurred_at=activated_at`；`event_day/seq/prev_hash/hash` 只由 AuditWriter 既有分段链算法派生。payload exact 为 `{schema_version:1,deployment_id,key_domain_id,legal_entity_id,activation_source:"INITIAL_GOVERNANCE",bootstrap_id,kek_ref,kek_version,kek_provider_fingerprint_sha256,data_keys,activated_at}`；data_keys 恰 16 项 `{data_key_id,purpose,security_level_scope,version,algorithm,wrap_kek_version,wrapped_key_sha256}`，按上述 purpose wire 再 scope 10/20/30/40 排序。标准非 bootstrap 供给复用同一事件但 `activation_source="STANDARD",bootstrap_id=null`；两者都要求 16 rows、状态推进与审计同事务逐项相等。外部 KMS、KEK/DEK readback 或矩阵失败不得伪称随前一 PostgreSQL 事务回滚：域保持 PROVISIONING，任何已生成但未绑定 material 按既有补偿/隔离规则收容，readiness 关闭，并统一返回 `PLATFORM.KEY_DOMAIN.KEY_UNAVAILABLE`；只有零 key_domains row 才返回 `PLATFORM.KEY_DOMAIN.NOT_PROVISIONED`。重启只对同 key_domain_id 继续 resume，绝不新建第二域或任选 orphan。九个常驻服务只有看到该域 ACTIVE、16-tuple graph、唯一 activation audit 与 bootstrap evidence 全部一致后才可开放依赖该治理域的 readiness。

事务末尾追加 `platform.bootstrap.initial_governance.v1` 审计，其 payload 绑定三条 legal-entity-grant ids 与 exact mapping；提交后按上段固定 path 原子写唯一非秘密 `initial-governance.receipt.v1.jcs`。不得生成部署 KMS sidecar，也不得给 `ep-migrate` KMS 签名能力。receipt exact body 为 `schema_version=1,purpose="EP-INITIAL-GOVERNANCE-RECEIPT-V1",bootstrap_id,deployment_id,bootstrap_body_sha256,initial_license_archive_sha256,governance_legal_entity_id,key_domain_id,key_domain_state="PROVISIONING",operator_user_ids,device_ids,role_codes,legal_entity_grant_ids,committed_at,audit_event_id,schema_manifest_sha256,ep_migrate_pe_sha256,status="COMMITTED"`，四个数组按 bytes 排序，grant id 恰三项；`receipt_body_sha256=SHA-256(receipt exact JCS bytes)` 不得自包含，只写入对应审计 payload。receipt 的可信性只来自双 CMS bootstrap input、命中签名产品清单的 Authenticode `ep-migrate` PE digest、数据库审计 hash chain 与 exact cross-check。若数据库已有逐字相同审计终结但崩溃漏 receipt，唯一重跑只读核对同 input digest/PE digest/审计 payload 后以 CREATE_NEW 补同 body receipt；已有 receipt 或任一字段不同永久拒绝，绝不二次写业务行。Stage 14 只在 receipt exact bytes 与审计 payload hash、三条 grant exact mapping、数据库 bootstrap projection、初始 license archive digest、最终首张 RELEASED grant、同 key domain 最终 ACTIVE 及唯一 activation audit 全部一致时放行启用/发布。

`trust_bundle_sha256` 只证明 grant source item 首次 RELEASE 时所用 bundle；历史 grant 行与所有 special item 的接受摘要不得因轮换回填，也不得在日常求值中要求它等于当前 bundle 摘要。revocation 与每个 MODULE_PACKAGE action 不另存第二份摘要，只从各自不可删除的 RELEASED source item 读取 `accepted_trust_bundle_sha256`。当前 `license-roots.p7b` 的完整性唯一绑定到签名部署清单所声明的期望摘要。

签名 `DeploymentManifestV1` 必须新增 `license_trusted_signer_subjects`，它是 F-56 inner/outer signer 授权的唯一事实：恰含 1..64 个 exact `spki-sha256:<64 lowerhex>`，按 UTF-8 bytes 严格升序且去重，manifest 签名与产品清单验证失败即不可用。本地 `release.trusted_signer_subjects` 只是可选 exact-equal 断言，不是授权源：`[]` 表示不覆盖并直接使用 signed roster；非空时必须长度、顺序与每个 token 逐字等于 signed roster，否则 public readiness 与 ops check 同时失败关闭。本地值绝不得增、删或替换 signer，parser 不做 set 宽松比较。CAB signer 轮换必须更新并重签 deployment manifest；若本地断言非空，同一维护批次必须更新为新 roster 的 exact copy，单改本地配置不能完成轮换。

计划轮换只能在 CAB 批准的维护操作中同步更新 bundle 与签名部署清单，并在许可/模块变更门关闭时枚举 exact set：父 package 已为 RELEASED 的全部 `LICENSE_GRANT` item（Grant 与 Revoke）和全部 `MODULE_PACKAGE` item；不得只扫 current 投影或 `license_grants` 行。每项都以新 bundle 分别重验 exact persisted special outer 与 inner、链与 CRL，并把唯一 current grant、命中 current 的 revocation、每个当前安装模块投影与其不可删除 source item 的 type/source/digest/identity 交叉核对；签名证据逐项保存旧接受摘要、新验证摘要、对象 id、outer 结论、inner 结论与总结果，轮换本身不更新任何 `accepted_trust_bundle_sha256` 或 grant `trust_bundle_sha256`。

重验结果按闭集分流：current grant 或命中 current 的 revocation 失败使全局保持 `RESTRICTED/SIGNATURE_INVALID`；当前安装模块失败只关闭该模块的业务写、审批、自动化和外发 effective runtime，绝不改变部署级 `LicenseStatus` 或生成全局许可 reason。仅当历史 signer 被新 CRL 明确命中时记 `HISTORICAL_SIGNER_REVOKED`，原 item/投影/审计保留但隔离，永不再计入 `purchased`、rollback candidate 或任何正向许可/模块证明；若另有 ACTIVE signer 的完整有效 current，该历史结论本身既不令全局 Restricted，也不阻断门重开。历史对象若为断链、source/digest/signature 漂移、结构损坏或不能精确分类为上述 CRL 情形，独立有效 current 不被倒推改写，但许可/模块变更门与 `RG-LICENSE-MODULE-LIFECYCLE-GREEN` 保持关闭，直至用可信备份/证据恢复。未伴随 CAB 清单更新的磁盘或部署清单摘要漂移仍立即使 current 失败关闭，不得回退到旧 bundle、Windows 任意根或历史接受摘要。

若唯一 current grant 或命中它的 revocation 的 inner signer 和/或其 RELEASED source special outer signer 唯一失败原因是新 CRL 明确 REVOKED，部署在替换提交前保持 `RESTRICTED/SIGNATURE_INVALID`，但允许 inner 与 outer 都由 ACTIVE signer 签发、同 deployment、同 `governance_legal_entity_id` 且 `supersedes_grant_id` 逐字指向该 current 的新 GRANT 走许可恢复链。该窄路径还要求旧 current 的 row/source/payload/digest/signature bytes、special outer bytes 与历史接受证据完全自洽；候选必须通过当前 bundle、日期、scope、用量与直接后继全部规则并在固定 advisory-lock 事务移槽。任一非 CRL 漂移、断链、多 current 或不能唯一分类的失败不得借此换证。运行角色无 DELETE，identity、签名、来源列与接受时 bundle 摘要不可原地改写；只允许受控事务单调推进 `last_trusted_at`、完成 current 替换或写入一次已验签撤销组。索引：`pk_license_grants`；`ux_license_grants_current_slot`；`ux_license_grants_revocation_id`（NULL 不冲突）；`ux_license_grants_grant_source_item`；`ux_license_grants_revocation_source_item`（NULL 不冲突）；`ix_license_grants_license_no_created_at`；`ix_license_grants_valid_to_created_at`。

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

列与约束按阶段 13 计划第 3.2.10 节逐项照建，本阶段的 `V20261013090400__platform_meta_create_config_packages.sql` 一次建齐 `content_version` 与 CONFIG_RELEASE 审批证据列：`approval_legal_entity_id`、`approval_scenario`、`submitted_by/submitted_at`、`approval_ref`、`approval_chain_id/approval_chain_version_no/approval_definition_digest`、`approval_content_version/approval_content_hash`、`approved_by/approved_at`、`rejected_by/rejected_at/rejected_reason`，以及阶段 13 所列真实法人/用户外键、自审/场景/状态形状 CHECK；阶段 13 的 090500 迁移不得重复增加或改型。本阶段只收窄 `status` 的 CHECK：取值为 `DRAFT`、`PENDING_APPROVAL`、`REJECTED`、`APPROVED`、`RELEASED`、`ROLLED_BACK` 六项，对应第 3.4.12 节的六态状态机；阶段 13b 扩展到十一态。审批形状必须保证提交前全部审批列为空；终态 special 的 DRAFT/PENDING_AUTOTEST/TEST_FAILED/TEST_PASSED 尤其不得提前填 `approval_legal_entity_id`，submit 才原子写全套证据。包体或内容项每次合法修改原子递增 `content_version` 并重算 `content_hash`；提交审批后不可修改。

**表 29 `platform_meta.config_package_items`**（3b 段，部署级）

列与约束按阶段 13 计划第 3.2.11 节逐项照建，并由 Stage 3 的 090500 首次建表迁移直接含 `accepted_trust_bundle_sha256 bytea null` 与“null 或 32 bytes”CHECK；不得由 Stage 13 的同尾号迁移重复加列或设置默认 bundle。Stage 3 的首次 CHECK 与 Rust `ItemKind::ALL` 恰为同序 18 项：前 16 项（至 `RULE`）加 `LICENSE_GRANT`、`MODULE_PACKAGE`；F-56 特殊 applier 因而可在本阶段真实落库，不存在 Rust/数据库瞬时分叉。阶段 13b 的既定 `V20261022090500__platform_meta_alter_config_package.sql` 再追加 `MCP_CONNECTOR`、`MCP_MANIFEST_VERSION`，并同批把 Rust 与 CHECK 扩为终态 20；不得修改 Stage 3 迁移或另建低版本 ALTER。`UNIQUE(config_package_id,id)` 只由 093300 在六条来源 FK 之前后补；同迁移以挂在 `config_packages/config_package_items/module_registrations/license_grants` 上的 DEFERRABLE INITIALLY DEFERRED `assert_f56_accepted_trust_projection_consistent()` 在 COMMIT 检查：普通 item 摘要恒 null；special 非 RELEASED 为 null、RELEASED 恰 32 bytes；非空不可改/不可清；grant 行现有 `trust_bundle_sha256` 与 grant source item 摘要逐字相等，revocation/module action 只从其不可删 source item 取接受摘要。未注册实现的 kind 仍由运行期整包拒绝，不靠 CHECK 跳过。

**表 30 `platform_meta.config_release_orders`**（3b 段，部署级）

列与约束按阶段 13 计划第 3.2.12 节逐项照建。`status` 的 CHECK 一次建齐，本阶段只使用 `SUBMITTED`、`APPROVED`、`REJECTED`、`EXECUTING`、`SUCCEEDED`、`FAILED`、`CANCELLED` 七项，`QUEUED` 与 `COMPENSATED` 留给阶段 13b 的停机窗口排队与 DDL 补偿。

本阶段只建上述三张配置表。`config_edit_locks` 与阶段 13 计划中承载逐项落地记录的 `config_release_steps` 按裁定 A-27 由阶段 13b 建，`config_autotest_runs` 与 `config_release_mutex` 同属阶段 13b。

**表 31 `platform_core.impact_assessments`**（3b-2 批，影响面评估批次）

| 列 | 类型 | 约束 |
|---|---|---|
| id | uuid | not null，pk |
| legal_entity_id | uuid | not null |
| security_level | smallint | not null default 20 |
| data_scope_tags | text[] | not null default '{}' |
| source_module | text | not null，取 `ModuleCode` 序列化值 |
| source_doc_id | uuid | not null |
| source_doc_version | bigint | not null，必须大于 0 |
| source_event_id | uuid | not null |
| source_event_type | text | not null，首版只允许 `clm.contract.terminated.v1` |
| reason | text | not null，清洗后长度 1 至 2000 |
| status | text | not null，CHECK 取 `RUNNING\|DONE\|FAILED` |
| item_total | int | not null default 0，非负 |
| item_done | int | not null default 0，非负且不大于 item_total |
| item_dead | int | not null default 0，非负且不大于 item_total |
| started_at | timestamptz | not null |
| finished_at | timestamptz | null |
| last_error_code | text | null，只存已登记稳定码 |
| row_version, created_at, created_by, updated_at, updated_by | | 按公共列 |

状态 CHECK 同时约束：RUNNING 时 `finished_at` 为空；DONE 时 `finished_at` 非空、`item_done=item_total` 且 `item_dead=0`；FAILED 时 `finished_at` 为空且 `item_dead>0`。索引与约束：`pk_impact_assessments`；`ux_impact_assessments_le_id` 在 `(legal_entity_id,id)` 上，供处置项复合外键引用；`ux_impact_assessments_le_source_version_event_type` 在 `(legal_entity_id,source_module,source_doc_id,source_doc_version,source_event_type)` 上唯一；`ux_impact_assessments_le_source_event_id` 在 `(legal_entity_id,source_event_id)` 上唯一；`ix_impact_assessments_le_status_started_id`；`ix_impact_assessments_le_source_doc`。本表 ENABLE、FORCE RLS，策略名 `rls_impact_assessments_le`。

**表 32 `platform_core.impact_disposition_items`**（3b-2 批，逐目标处置项）

| 列 | 类型 | 约束 |
|---|---|---|
| id | uuid | not null，pk；也是处置幂等键 |
| legal_entity_id | uuid | not null |
| security_level | smallint | not null default 20 |
| data_scope_tags | text[] | not null default '{}' |
| impact_assessment_id | uuid | not null，同 schema 复合外键 `(legal_entity_id,impact_assessment_id)`，ON DELETE RESTRICT |
| impact_rule_code | text | not null，必须属于 `docs/impact-catalog.md` 七码 |
| target_module | text | not null，取 `ModuleCode` 序列化值 |
| target_doc_id | uuid | null；未接线目录占位项及 `NO_APPLICABLE_TARGET` 终态目录项为空，真实目标项非空 |
| target_doc_no | text | null；目标为空的目录项为空 |
| target_doc_line_no | int | null，非空时大于 0 |
| disposition_kind | text | not null，CHECK 取 `AUTO_CLOSE\|AUTO_CANCEL\|MANUAL_DECISION\|INFORM_ONLY` |
| state | text | not null，CHECK 取 `PENDING\|DISPATCHING\|DONE\|DEAD` |
| attempts | smallint | not null default 0，CHECK 取 0 至 9；首投失败为 1，第八次重试仍失败为 9 |
| available_at | timestamptz | not null default now() |
| locked_by | text | null |
| locked_until | timestamptz | null |
| last_error_code | text | null，只存已登记稳定码 |
| last_error | text | null，只存清洗后摘要，不含正文与秘密 |
| process_task_id | uuid | null，与法人组成真实复合外键 `(legal_entity_id,process_task_id) -> platform_flow.process_tasks(legal_entity_id,id)`，`ON DELETE RESTRICT` |
| decision_code | text | null |
| decision_reason | text | null，清洗后长度不超过 2000 |
| decision_result_doc_id | uuid | null |
| decided_by | uuid | null |
| decided_at | timestamptz | null |
| outcome_reason | text | null，DONE 时为非空稳定原因码 |
| row_version, created_at, created_by, updated_at, updated_by | | 按公共列 |

表级 CHECK 一次冻结：`locked_by` 与 `locked_until` 同空同非空，只有 DISPATCHING 可持租约。目标三字段全空的目录项只允许两种完整形状：其一为 `state=PENDING` 且 `outcome_reason` 为空；其二为 `state=DONE` 且 `outcome_reason='NO_APPLICABLE_TARGET'`。两种形状都必须 `attempts=0`，租约、两项错误、`process_task_id`、三项 decision 与 `decided_by/decided_at` 全空；除此之外的 DONE、DEAD、DISPATCHING 项必须 `target_doc_id` 非空，目标号/行号仍按业务形状可空。非 `MANUAL_DECISION` 的 `decision_code/decision_reason/decision_result_doc_id/decided_by/decided_at` 全为空；人工项 PENDING 时前三个决策字段全为空；人工项 DONE 时 `decision_code`、非空 `decision_reason`、`decided_by`、`decided_at` 与 `process_task_id` 必填，`decision_result_doc_id` 的逐码必填/必空形状由目录与业务规则同事务校验；DONE 必须有 `outcome_reason`，DEAD 必须有 `last_error_code`，其他状态不得伪造完成原因。`decided_by` 非空时以 `(legal_entity_id,decided_by)` 复合外键指向用户法人授权。数据库不解析 `decision_reason`；`target_doc_id` 与 `decision_result_doc_id` 连同各自判别信息属于 F-54 明确登记的封闭多态 target/result 组合，按基线白名单不建外键。

索引与约束：`pk_impact_disposition_items`；`ux_impact_items_le_assessment_rule_target` 在 `(legal_entity_id,impact_assessment_id,impact_rule_code,coalesce(target_doc_id,impact_assessment_id),coalesce(target_doc_line_no,0))` 上唯一；`ix_impact_items_le_state_available_id` 用于领取；`ix_impact_items_le_assessment_state_id`；`ix_impact_items_le_process_task_id`。本表 ENABLE、FORCE RLS，策略名 `rls_impact_disposition_items_le`。

**表 33 `platform_flow.approval_command_snapshots`**（3b-2 批，高保密审批命令快照）

每个需要在审批通过后执行的命令恰有一张快照；命令明文只在调用线程的有界内存中完成规范化与信封加密，数据库从未出现明文 `command` 或 jsonb 命令副本。

| 列 | 类型 | 约束 |
|---|---|---|
| id | uuid | not null，pk |
| legal_entity_id | uuid | not null |
| security_level | smallint | not null default 30，CHECK 固定为 30 |
| data_scope_tags | text[] | not null default '{}' |
| process_instance_id | uuid | not null；`(legal_entity_id,process_instance_id)` 真实复合外键指向 `platform_flow.process_instances(legal_entity_id,id) ON DELETE RESTRICT` |
| owner_module | text | not null，取 `ModuleCode` 序列化值 |
| scenario | text | not null，稳定审批场景码，长度 1..64 |
| action | text | not null，稳定命令动作码，长度 1..64 |
| schema_version | int | not null，CHECK 大于 0；由 owner 模块逐场景冻结 |
| command_enc | bytea | not null，非空；AES-256-GCM 信封密文，是命令 DTO 的唯一持久载体 |
| command_key_ref | text | not null，指向当前法人密钥域内用途为 FIELD、密级 30 的数据密钥引用 |
| command_digest | bytea | not null，固定 32 字节；取 `SHA-256(command_enc \|\| canonical_aad)`，只校验密文完整性，不是明文等值索引 |
| request_hash | bytea | not null，固定 32 字节；只哈希法人、owner_module、scenario、action、schema_version、process_instance_id 与幂等键等非敏感规范路由封套，不哈希命令明文 |
| state | text | not null default 'PENDING'，CHECK 取 `PENDING\|CONSUMED\|REJECTED\|EXPIRED` |
| consumed_at | timestamptz | null |
| expired_at | timestamptz | null |
| result_object_type | text | null；CONSUMED 必填，owner 模块冻结的稳定对象类型码，长度 1..64 |
| result_object_id | uuid | null；CONSUMED 必填，与类型组成封闭多态执行结果定位 |
| result_doc_no | text | null；CONSUMED 按结果对象可空，非空时长度 1..64，仅冗余展示 |
| row_version, created_at, created_by, updated_at, updated_by | | 按公共列；created_by/updated_by 与法人组成复合外键指向用户法人授权 |

约束与索引：`pk_approval_command_snapshots`；`ux_approval_command_snapshots_le_instance` 在 `(legal_entity_id,process_instance_id)` 上，落实一实例一快照；`ux_approval_command_snapshots_le_id` 供后续同法人引用；`ix_approval_command_snapshots_le_state_created`；`ck_approval_command_snapshots_digest_lengths` 强制两个摘要各 32 字节；`ck_approval_command_snapshots_state_shape` 强制 PENDING/REJECTED 的两个时间与三项结果定位全空，EXPIRED 仅 expired_at 非空且三项结果全空，CONSUMED 仅 consumed_at 非空、`result_object_type/result_object_id` 必填而 `result_doc_no` 可空。状态触发器只允许 `PENDING -> CONSUMED|REJECTED|EXPIRED`，三个终态不可再迁移；只有 PENDING→CONSUMED 的同一条 UPDATE 可在写 `state/consumed_at` 时一次写入三项结果定位，转 REJECTED/EXPIRED 不得写，终态后逐列不可变。除此只可改变 `row_version`、`updated_at`、`updated_by`；`process_instance_id`、owner/scenario/action、schema_version、`command_enc`、`command_key_ref`、两个摘要及创建证据全部逐列不可变。CONSUMED 前 owner 必须在同一事务重新校验流程已批准、snapshot 与当前法人/实例/动作一致、schema_version 受支持且 request_hash 匹配，然后才解密一次、执行命令并取得真实结果定位；业务对象写入与快照的 CONSUMED+结果三字段更新必须处于同一事务。任一校验、解密或业务写入失败均零业务写入、快照仍为 PENDING，且不得把明文写入日志。`result_object_type/result_object_id` 是由 owner 写入的封闭多态结果组合，按基线白名单不建伪外键；查询由 process instance/approval_ref 定位本行后按类型路由至 owner。表按统一模板 ENABLE、FORCE RLS，策略名 `rls_approval_command_snapshots_le`。

第 39 号迁移向 `platform_core.sensitive_field_registry` 登记且只登记一行：`schema_name='platform_flow'`、`table_name='approval_command_snapshots'`、逻辑 `column_name='command'`、`category='LEGAL'`（复用既有不可改 CHECK 中用于审批法律证据的类别）、`security_level=30`、`is_field_encrypted=true`、`blind_index='NONE'`、`blind_index_column=NULL`、`mask_style='FULL'`、`normalization='NONE'`、`release_ref='MIGRATION:20261013093700'`。物理列只有 `command_enc bytea` 与 `command_key_ref text`，不存在 `command` 明文列或 `command_bidx`；`db/checks/11` 必须据此通过。

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

#### 3.3.7 仅追加表的登记与触发器挂接

按裁定 B-02，本阶段有三张仅追加表须在 `platform_core.append_only_registry` 中显式登记并挂接触发器。登记表、`platform_core.assert_append_only`、`platform_core.assert_immutable_columns` 与 `platform_core.attach_table_guards` 四者均由阶段 2 交付，本阶段只写登记行并调用挂接函数，不重复定义机制。登记行的列集为 `schema_name`、`table_name`、`mode`、`mutable_columns`，逐行取值如下，不得增行删行，也不得改取值。

| schema_name | table_name | mode | mutable_columns |
|---|---|---|---|
| platform_audit | audit_events | APPEND_ONLY | `'{}'` |
| platform_msg | outbox_events | IMMUTABLE_COLUMNS | `status`、`attempts`、`available_at`、`locked_by`、`locked_until`、`last_error` |
| platform_msg | dead_letters | IMMUTABLE_COLUMNS | `state`、`repaired_by`、`repaired_at`、`approval_ref`、`discard_reason` |

三行登记与三次挂接落在第 33 号一个迁移文件内，文件内先按上表插入三行登记，再依次调用 `platform_core.attach_table_guards('platform_audit','audit_events')`、`platform_core.attach_table_guards('platform_msg','outbox_events')`、`platform_core.attach_table_guards('platform_msg','dead_letters')`，顺序不得颠倒：挂接函数读登记表取可变列白名单，先挂接后登记取不到 `mutable_columns`。该文件的 `-- rollback:` 段为删除该三行并 drop 对应触发器。

三处取值各有理由。`audit_events` 无任何更新路径，取 `APPEND_ONLY` 与空白名单。`outbox_events` 的六列白名单与表 13 的投递控制列逐项相同，信封与载荷不在其内，与第 3.12.2 节澄清二的仅追加口径一致。`dead_letters` 的五列必须取全，第 3.5.4 节的三个处置端点分别写入这五列中的不同子集，白名单少一列即在上线后拒绝该列的写入，修复完成与丢弃两条路径直接失败。`platform_audit.audit_segments` 有 `state`、`last_anchor_seq` 与 `last_anchored_at` 的正常更新，按裁定 B-02 不进登记清单，登记为仅追加会拒绝第 3.4.3 节阶段 A 的锚定写入。

登记与物理表上实际挂接的触发器是否逐项一致，由 `db/checks/append_only_consistency.sql` 断言，`xtask sqlcheck` 执行，返回零行为通过。

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

哈希输入的规范化。本阶段规定审计事件的哈希输入为 RFC 8785 JCS 规范化的 JSON 对象，字段为除 `hash` 外的全部列，`prev_hash` 以小写十六进制字符串承载，`bytea` 与 `uuid` 一律以字符串承载，`occurred_at` 以 RFC 3339 UTC 微秒精度字符串承载。没有具名 typed-audit ABI 时，`before/after` 中业务 decimal/integer 才统一用 canonical 十进制 JSON string，避免 PostgreSQL `jsonb` 把 `1.10` 与 `1.1` 归一化后破坏前像；一旦 event/action 已冻结 unknown/missing-key 均失败的 strict DTO，则必须逐字段沿该 DTO 的 wire type，禁止全局 string 规则覆盖。尤其 `platform.key_domain.activated.v1`、`platform.config_special.accepted.v1`、`INITIAL_GOVERNANCE_BOOTSTRAPPED`、`LICENSE_TRUSTED_TIME_CHECKPOINT`、`MODULE_SIGNER_REVOKED_DISABLED` 五个 typed audit 的 `schema_version` 都是 JSON number `1`，不是字符串 `"1"`；其余数值字段同样只按各自 ABI。

追加算法，在业务事务内执行，七步。

1. 计算 `event_day = (occurred_at AT TIME ZONE 'Asia/Shanghai')::date`。
2. 收集本事务内待写入的全部审计事件，按 `event_day` 分组；若跨越两个自然日，按 `event_day` 升序依次处理，避免与另一事务反向加锁形成死锁。
3. 对每个 `event_day`，执行 `INSERT INTO platform_audit.audit_segments ... ON CONFLICT (legal_entity_id, event_day) DO NOTHING`，再执行 `SELECT last_hash, last_seq, event_count FROM platform_audit.audit_segments WHERE legal_entity_id = $1 AND event_day = $2 FOR UPDATE`。这一步是判定二所述的串行化点，`lock_timeout` 为 3 秒，超时映射为 `PLATFORM.AUDIT_EVENT.SEGMENT_LOCK_TIMEOUT`。
4. 以 `last_hash` 为 `prev_hash`（段首条取 32 字节全零），按上述规范化算出 `hash = SHA-256(canonical_bytes)`。多条事件在同一段内按写入顺序链式串接，第 n 条的 `prev_hash` 是第 n−1 条的 `hash`。
5. 批量 `INSERT` 全部事件，`seq` 由 `bigserial` 分配。
6. 更新段行的 `last_hash`、`last_seq`、`event_count`、`first_seq`（首次写入时设置），并在同一条语句中固定写 `row_version = row_version + 1, updated_at = now(), updated_by = actor_user_id`；系统发起时 `actor_user_id` 取 `ep_foundation::SYSTEM_PRINCIPAL_ID`。不得只改业务列或只增版本，否则阶段 2 的 `assert_row_version_bump()` 必须拒绝该语句。
7. 事务提交。

边界条件五项。其一，审计写入必须是工作单元闭包内的最后一批写入，段锁持有时间因此近似为一次更新加一次提交刷盘的时长；本阶段设内部观测目标为审计写入对业务事务耗时的增量 P95 不超过 15 毫秒，作为容量健康度的观察项，不作为规格通过线。其二，`seq` 空洞不判为链断裂。其三，同一事务写多条事件时链内顺序即写入顺序，验证时按 `seq` 升序即可重现。其四，段跨日不影响链，跨日的第一条事件在新段中 `prev_hash` 为全零，段与段之间不建立跨段链接，这与规格第 12.5 章“每段为一条独立链”一致。其五，客户端提交的审计事件不建本地分段链，由中心按同一序列写入对应法人与自然日的段，规格第 12.5 章明确要求，本阶段的追加接口对全部来源使用同一路径，不为客户端开第二条路径。

#### 3.4.3 段根签名、锚定与链验证

锚定触发条件先要求 `last_seq > coalesce(last_anchor_seq,0)`，即至少存在一条尚未成功锚定的新事件；在此前提下，再判某段自上次**成功**锚定以来经过时间不少于 `anchor_interval_seconds`（默认 300），或 `last_seq - coalesce(last_anchor_seq,0)` 不少于 `anchor_event_threshold`（默认 1000）。无新事件时即使时间阈值到达也不得重复创建同一根。扫描周期 `anchor_scan_interval_seconds` 默认 30 秒，由 job-worker 按法人轮转执行。每轮先按 `(available_at,id)` 恢复 `available_at <= now()` 的 `PENDING_SIGN|SIGNED` 旧行，再考虑创建新根；一个段存在任一非 `EVIDENCED` 锚点时不得创建后继锚点。这样进程在阶段 B/C 任一点崩溃后只需依数据库状态恢复，不能依赖内存计数或上一进程的队列。

锚定分三段，理由是签名要经 `ep_foundation::port::kms::KmsBackend` 的 `sign` 调用 KMS 载体、写证据文件要落盘，两者都不能在持有段锁的事务内做，否则会把业务事务的段锁等待放大到 KMS 载体与磁盘的延迟上。载体实现在 `ep-adapter-kms`，`ep-platform-audit` 不依赖该 crate，实例由 `apps/core-server/src/wiring/` 与 `apps/job-worker/src/wiring/` 两个目录注入。

阶段 A，短事务。在开事务前经当前 KMS 配置解析一条可用审计签名键引用 `selected_key_ref`；取段行 `FOR UPDATE`，同锁读取 `last_seq,last_hash,event_count`。取到锁即说明该段无在途审计写入，因为一切审计写入都要先取该锁。先确认该段仍有新事件且不存在 `state <> 'EVIDENCED'` 的锚点，再以 `INSERT ... ON CONFLICT (legal_entity_id,audit_segment_id,anchor_seq) DO NOTHING RETURNING id` 插入完整行：预生成 `id`，`legal_entity_id` 与 `audit_segment_id` 取被锁段，`anchor_seq = segment.last_seq`、`root_hash = segment.last_hash`、`event_count = segment.event_count`、`algorithm = 'ECDSA_P256_SHA256'`、`key_ref = selected_key_ref`、`state = 'PENDING_SIGN'`、`attempts = 0`、`available_at = now()`，公共创建/更新人均取 `ep_foundation::SYSTEM_PRINCIPAL_ID`；冲突时读取既有行并按其状态恢复，不新增第二行。`event_count/algorithm/key_ref` 从插入起不可变，证明的是这一根建立时的快照与当时选定的键。此阶段**不更新**段行的 `last_anchor_seq/last_anchored_at`，因为一次尝试不是成功水位。提交后才进入阶段 B。

阶段 B，无锁。只读取 anchor 行内已经冻结的 `algorithm/key_ref`，不得在重试、进程重启或密钥轮换后重新选键；以该 `key_ref` 指向的签名私钥对 `SHA-256(JCS({segment_id, legal_entity_id, event_day, anchor_seq, root_hash, event_count}))` 做该行 `algorithm` 指定的 ECDSA P-256 签名。算法取值来自规格第 12.3 章“摘要使用 SHA-256，签名使用 RSA 或 ECDSA”，本阶段新建锚点固定取 ECDSA P-256，理由是签名长度短、验签快，且首版不含商用密码档位；历史 RSA 行仍按自身 algorithm 验签。条件更新固定为 `UPDATE ... SET signature = $1, signed_at = now(), state = 'SIGNED', row_version = row_version + 1, updated_at = now(), updated_by = $system_principal_id WHERE id = $2 AND state = 'PENDING_SIGN' AND row_version = $expected_row_version RETURNING id, row_version`，其中 `$system_principal_id = ep_foundation::SYSTEM_PRINCIPAL_ID`。返回零行表示并发 worker 已推进或当前状态不再可签，不得覆盖；返回一行才可进入阶段 C。

阶段 C，写证据。经 `ep-adapter-file` 的 `evidence` 命名空间以 `create_new` 写入 `<legal_entity_id>/<event_day>/<anchor_seq>-<anchor_id>.json`，内容为 JCS 规范化的锚定记录。`create_new` 遇到同名文件返回已存在，视为幂等成功。随后开短事务并按固定顺序锁对应 `audit_segments` 行、重读 anchor：先以 `state='SIGNED' AND row_version=$expected_row_version` 条件把 anchor 更新为 `EVIDENCED`，写 `evidence_path/evidence_written_at` 及 `row_version = row_version + 1, updated_at = now(), updated_by = $system_principal_id`；再把同一 segment 的 `last_anchor_seq = greatest(coalesce(last_anchor_seq,0),$anchor_seq)`、`last_anchored_at = now()`，并同批写 segment 的 `row_version = row_version + 1, updated_at = now(), updated_by = $system_principal_id`，最后一起提交。anchor CAS 返回零行时整笔回滚，不得推进 segment 水位；重新读取后若已 `EVIDENCED` 则幂等成功，否则按当前状态重排，陈旧 worker 不得覆盖 `FAILED` 或其他状态。按裁定 C-27，审计证据目录为 `C:\EP\audit-evidence`，权限位换 NTFS ACL：该目录断继承并显式设 DACL，不保留 `BUILTIN\Users` 一类的继承 ACE，不设共用本地组，进程之间的授权逐账户列 ACE——job-worker 的服务虚拟账户 `NT SERVICE\ep-worker` 授读写，archive-writer 的服务虚拟账户 `NT SERVICE\ep-archive` 只授读取，并对该账户显式 Deny `DELETE` 与 `FILE_WRITE_DATA`。C-27 的结论一字不变，载体换 NTFS ACL 之后表达力增强，这一处是净改善且要写出来：原 0750 加组 `ep` 只能靠组权限位凑出「只读」，而 Deny ACE 是对该账户逐权限的否定，比靠组权限位凑更贴合 C-27 原文「不授予 archive-writer 任何写入与删除权限」。证据文件与段根签名一律由 job-worker 产生，archive-writer 只读取并写出到服务器之外落点，本进程不承担写出。

失败处理：阶段 B 或 C 失败时，以当前 `state + row_version` 做 CAS，令 `attempts = attempts + 1`、写 `last_error`，并同批写 `row_version = row_version + 1, updated_at = now(), updated_by = ep_foundation::SYSTEM_PRINCIPAL_ID`。新 attempts 为 1..8 时状态保持不变，`available_at` 分别取 `now() + [1m,2m,5m,10m,30m,1h,2h,4h]`；新 attempts 为 9 时只允许从 `PENDING_SIGN|SIGNED` 置 `FAILED`，并在同一事务写死信与站内通知。CAS 零行表示状态已被其他 worker 推进，当前 worker 立即停止，不补写、不覆盖。通用死信的记名 `replay` 是唯一修复入口：锁定 anchor 与 dead letter，若 FAILED 行 `signature IS NULL` 则重置为 `PENDING_SIGN`，否则重置为 `SIGNED`；同时置 `attempts=0, available_at=now(), last_error=NULL` 并增版、写系统更新证据，原 anchor id 与唯一键均不改变，dead letter 记修复人/时间。下一轮从原状态继续，禁止跳过签名或证据步骤。指标 `ep_audit_evidence_write_failures_total` 上升。规格第 12.5 章要求“最近一次成功锚定时间在运维中心可见，超过约定间隔告警”，本阶段以 `ep_audit_anchor_age_seconds` 指标与 `platform_audit.v_anchor_lag` 视图承载，台账条目的登记由运维中心所在阶段实现，本阶段提供数据源。

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

批量 100，轮询 200 毫秒，无待处理时退避到 2 秒。取件事务内即把 `status` 置 `DISPATCHING`、`locked_by` 置进程标识、`locked_until` 置 `now() + lock_lease_seconds`，提交后再投递。job-worker 是唯一 Outbox 消费进程；投递一般为进程内处理器调用，推送事件也先由 job-worker 消费并组装脱敏载荷，再经 `\\.\pipe\ep-integ` 请求 integration-gateway 对外发送，gateway 本身不读数据库、不消费 Outbox，见第 3.4.6 节。

消费侧幂等：处理器的副作用与 `INSERT INTO platform_msg.inbox_consumptions (consumer, event_id) ...` 在同一事务内，唯一约束冲突即判为已消费，跳过副作用并直接置 `DONE`。这是规格第 7.3 章“Outbox 可靠投递测试项，覆盖至少一次投递、重复投递去重和崩溃恢复后不丢不重”的实现依据。

重试退避固定八段：1 秒、5 秒、30 秒、2 分钟、10 分钟、30 分钟、1 小时、2 小时。首投失败后依次使用八档安排八次重试；第八次重试仍失败、失败计数成为 9 时，在同一事务完成三件事：以 `INSERT ... ON CONFLICT (source_event_id) DO NOTHING` 写 `dead_letters`、把 `outbox_events.status` 置 `DEAD`、写审计事件。转死信时的 `failure_category` 取五类分类之一，取值来自处理器返回的 `AppError.category`。

崩溃恢复：`locked_until` 过期的 `DISPATCHING` 条目由扫描器改回 `PENDING` 并把 `available_at` 置当前时刻。因为投递副作用与 `inbox_consumptions` 同事务，重放不会重复产生副作用。

Stage 3 command middleware 提供唯一 `pre_idempotency_lock` 三值 preamble，并统一使用 advisory key `hashtextextended('platform-license-current',0)`。会产生 `BusinessWrite|BusinessApproval|IntegrationOutbound|AutomationStart` 的普通 handler/job 固定选 `LICENSE_CURRENT_SHARED`：`BEGIN`、mandatory session-context `SET LOCAL` 后第一条业务 SQL 执行 `SELECT pg_advisory_xact_lock_shared(...)`，之后才可 `IdempotencyStore::try_begin`、查询/claim、取得 row/module lock，并在锁内重跑 `LicenseAdmissionGate`；普通共享锁之间可并发。F-56 special 推进、current grant/revocation 替换与 TrustedClock checkpoint 固定选 `LICENSE_CURRENT_EXCLUSIVE`，同一位置执行 `SELECT pg_advisory_xact_lock(...)`；exclusive 等待所有既有 shared 副作用事务排空，获得后推进 current，后续新请求排队并在锁内重验许可。纯读以及 effect 为 `ReadReportAuditBackupExport|IdentitySecurityDisposition|ComplianceDisposition|InFlightConvergence` 的冻结允许路径可按各自 binding 选 `NONE`；`LicenseGrantRecovery|ModuleDisableRecovery` 只决定 Restricted 准入，必须由 ConfigRelease shared 入口在 exclusive transaction 内 strict 派生，绝不赋予 `NONE`。reject 是唯一 ConfigRelease 写结论例外，typed command 分支必须在进入事务前固定 `NONE`，只锁自身 package/flow rows 闭合同一 immutable content hash，禁止无锁查包后在 approve/reject 间改判。

import/autotest/submit/approve/sign/create-release-order/execute 七类共享配置入口及 autotest accept、worker batch claim、lease/heartbeat、最终 aggregate 的每个短事务无条件选择 `LICENSE_CURRENT_EXCLUSIVE`；不能先查包决定锁，普通配置包经过这些共享入口也接受 exclusive。九套 suite 的纯只读查询事务不 claim、不续租、不汇总，可选 `NONE`。Outbox/worker 的 claim 短事务选择 `LICENSE_CURRENT_SHARED` 并锁内重验 admission，Restricted 时保持 PENDING/DISPATCHING 原状；真正外发绝不能在跨外部调用的数据库事务中持锁，而是在 dispatch 前用专用连接取得同 key 的 session-level shared advisory lock，再按 wire 顺序取得目标 module session-level shared lock，重验许可/模块后持到外部副作用或取消终结，并在 `finally` 反序释放。preamble 只能在现有 `UnitOfWork::transact` closure 内以 `&mut dyn Tx` 执行，禁止另开数据库事务；recording transaction 与架构检查必须证明所选 shared/exclusive lock 成功前 handler/repository/idempotency store 零数据库调用，反序在第一句违规 SQL 前失败。

幂等键算法（3a 段）：本阶段只实现阶段 2 定义的 `ep_foundation::port::db::IdempotencyStore`，请求头的存在性与 UUIDv7 合法性已由阶段 1 的 `IdempotencyKeyHeaderGuard` 判定，本阶段不重复判断。选择 `LICENSE_CURRENT_SHARED|LICENSE_CURRENT_EXCLUSIVE` 的命令中，`try_begin(tx, scope: IdempotencyScope, request_hash: [u8; 32])` 只能在对应 preamble 成功后执行；选择 `NONE` 的 typed 命令直接按其冻结分支执行。`try_begin` 首句为 `INSERT INTO platform_msg.idempotency_keys (..., state) VALUES (..., 'IN_PROGRESS') ON CONFLICT (legal_entity_id, user_id, endpoint, key) DO NOTHING`，`IdempotencyScope` 的四个字段与该唯一约束逐项对应，`request_hash` 以 64 位小写十六进制存入 `request_hash` 列。受影响行数为 1 返回 `IdempotencyOutcome::FirstCall`；为 0 则读取已有行：`state = 'COMPLETED'` 且 `request_hash` 相同时返回 `Replay { status, body }`，调用方据此回放并带 `Idempotent-Replay: true`；`request_hash` 不同时返回 `PayloadMismatch`，由调用方映射为 `PLATFORM.IDEMPOTENCY.PAYLOAD_MISMATCH`；`state = 'IN_PROGRESS'` 时首次调用尚在执行，此时不返回任何 `IdempotencyOutcome`，而以 `Err(AppError)` 返回 `PLATFORM.IDEMPOTENCY.IN_PROGRESS`。`finish(tx, scope, response_status: u16, response_body: &[u8])` 在同一事务内把 `state` 置 `COMPLETED` 并写入 `response_status` 与 `response_body`。事务回滚时 `IN_PROGRESS` 行一并回滚，不留残留。`request_hash` 取请求体规范化后的 SHA-256，不含请求头。保留 7 天。

#### 3.4.5 站内通知与扇出

站内通知在业务事务内同步写入，不经 Outbox。理由是规格第 5.1 章把站内通知定为“首版唯一验收不可豁免的通知渠道”，把它挂在至少一次投递的异步链路上会引入一个本可避免的丢失面与延迟面；而通知的写入是同库单表插入，放进业务事务不违反基线第 10.3 节的事务内禁止清单。登记为本阶段新增决定。

写入算法四步。其一，按 `notice_type` 取当前生效的模板版本。其二，解析接收人，得到 `Vec<UserId>`；解析方式由触发源决定：审批待办取 `process_tasks.assignee_user_id` 或候选角色展开，审批结果取 `process_instances` 的发起人，对账差异取该法人的数据责任人，**影响面处置取目标模块的管理者角色展开**（按裁定 F-44 决定二：销售退货类取 `SALES_MANAGER`、发票作废或红冲类取 `FINANCE_MANAGER`、采购需求类取 `PROCURE_MANAGER`、项目任务类取 `PROJECT_MANAGER`；四类一律只展开候选角色、`assignee_user_id` 留空，不在出厂数据里写死任何自然人）。**`PROJECT_MANAGER` 是本裁定新增的第六个角色码**——F-10 的 C-3 只冻结了五个，其中无项目侧角色，不补则「在制任务收尾还是取消」这一类无人可派。其三，对每个接收人渲染标题与正文，渲染只允许使用模板声明的变量集合，声明外变量拒绝渲染；渲染前对该接收人做一次字段可见性裁剪，无权字段以事项类型与编号替代。其四，批量插入 `notifications` 与 `notification_deliveries`（`IN_APP` 通道直接 `DELIVERED`），`ON CONFLICT (legal_entity_id, recipient_user_id, dedupe_key) DO NOTHING` 完成去重。

扇出规模标定改挂已冻结的规模基线：首版命名用户上限 50，任一类提醒的接收人集合都是在职用户的子集，故最大扇出不超过 50 行，单事务可承受。**此处原以「许可临期与宽限期告警」为支点，该类已撤下**——它是全卷唯一「接收人为全体在职用户」的分支，撤下后没有任何一类提醒的接收人是未定上界的集合，标定反而更稳。若某类提醒的接收人超过 200，改为写一条 Outbox 事件由 job-worker 分批扇出，阈值 `notify.sync_fanout_max` 默认 200。

未读上限：单用户未读数达到 `unread_cap_per_user`（默认 2000）时，新通知仍写入，同时把该用户最旧的已超过保留期的已读通知纳入下一轮清理，并写一条 `WARN` 级运行日志。不丢新通知，这是不可豁免渠道的底线。

保留期与未读上限两个取值对应 PRD 附录乙 U-K-04，现已由 F-51 确认冻结为 180 天与 2000 条；实现不得二次选择。未来正式变更只需改配置，不涉及结构变更。

#### 3.4.6 移动推送出口

推送是站内通知之上的可选增强，不是任何提醒的保证渠道，规格第 5.1 章与 PRD 第 10.5.1 节都明确。因此推送链路的任何失败一律不产生用户可见错误，只记录 `notification_deliveries.status = 'FAILED'` 与指标。

链路四步。其一，core-server 写入通知的同一事务内，若该接收人存在活跃 `push_registrations` 且 `notify.push_enabled` 为真，写一条 `platform.notification.push_requested.v1` 到 Outbox 并插入 `notification_deliveries` 的 `MOBILE_PUSH` 行为 `PENDING`。其二，job-worker 消费该事件，组装推送载荷：默认只含事项类型与关联单据编号，不含任何业务字段，由 `notify.push_body_includes_business_fields` 控制，默认关闭，对应 PRD 附录乙 U-K-05 且默认取最保守值。其三，job-worker 以 `NT SERVICE\ep-worker` 连接 `\\.\pipe\ep-integ`，发送 `push.dispatch.v1` 普通帧，超时 5 秒；不配置 endpoint、不发起回环 HTTP。其四，integration-gateway 执行出网投递，带超时、退避与熔断，只返回清洗后的稳定结果；job-worker 在自己的权威事务更新 `notification_deliveries`，gateway 没有数据库凭据或落库路径。

连续失败达到阈值的 `push_registrations` 行置 `is_active = false`，理由是失效令牌会持续消耗出网重试预算。
令牌明文的唯一出现位置是第二步的载荷组装：job-worker 按 `token_key_ref` strict parse `token_enc` 的 EPC1，并以 `KeyPurpose::Field(SecurityLevel::Confidential)` 对应的私有 `Aad::for_field(legal_entity_id,"platform_msg.push_registrations.token",registration_id,SecurityLevel::Confidential)` 调 `ep_foundation::port::kms::KmsBackend::unwrap`（实例由 job-worker 的 wiring 目录注入，`ep-platform-notify` 不依赖 `ep-adapter-kms`）。`unwrap` 的返回值就是推送令牌业务明文；选钥、解封 DEK、AES-GCM 与 EPC1 identity/ref 核对全部留在 adapter 内，调用方不得先取得字段密钥再自行解密。明文只在 zeroizing 进程内存存活到本次投递结束，不落盘、不写运行日志、不进错误消息（由 `Redacted<T>` 与 `SecretString` 拦截），也不进入任何审计事件、Outbox 信封与推送载荷字段。该路径不经阶段 4 的 `FieldProjector`，不做字段权限与密级判定，理由是它不向任何主体返回该列：第 3.5.1 节两个推送登记端点只写不读，本阶段没有任何端点返回 `token_enc`、`token_key_ref` 与 `token_bidx` 三列中的任何一列。阶段 4 第 4.7 节所述的唯一解密位点是就字段投影而言的，本条不经投影，也不新增第二套解封实现，解封入口仍只有 `KmsBackend`。

推送出口的进程归属已同步写入共享技术基线，见第 3.12.1 节修订一；它是现行唯一口径，不再是待批准偏离。

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
| SCANNING | COMMITTED | TYPE_SNIFF、STRUCTURE 均为 PASS；`virus_scan_mode=NONE` 时 VIRUS_ICAP 为 SKIPPED，取 CUSTOMER_ICAP 时 VIRUS_ICAP 必须为 PASS | 版本行由 PENDING 置 AVAILABLE 成功 |
| SCANNING | REJECTED | 任一内建检查为 REJECT，或 VIRUS_ICAP 为 REJECT/ERROR | 版本行置 QUARANTINED；CUSTOMER_ICAP 不可用时不得回退 NONE |
| INITIATED/UPLOADING | ABORTED | abort 或超时回收 | — |
| 任一非终态 | EXPIRED | 超过 expires_at | — |

正文落盘的三段式，对应判定三。

第一段，固定数据密钥后写事务 A。进入正式落盘前只调用一次 `KmsPinnedDataKeyBackend::resolve_data_key(deployment_id,legal_entity_id,key_domain_id,KeyPurpose::Attachment(security_level),DataKeySelectorV1::CurrentForWrite)`，取得私有且不可伪造的 `DataKeyHandleV1`；它只暴露无秘密的 `canonical_ref()`，wire 唯一为 `data-key://<lowercase-data-key-uuid>#<u16非零十进制版本>`，版本无前导零。随后事务 A 写 `attachment_versions` 一行，`state='PENDING'`，`storage_path` 预先由 `<legal_entity_id>/<security_level>/<yyyy>/<mm>/<version_id>` 确定，`dek_ref=handle.canonical_ref()`，`key_domain_ref` 固定同一法人域。事务内重读该 ref 所指 `data_keys`，要求法人、domain、purpose=ATTACHMENT、scope、id/version 与 handle 全等且此刻仍为 ACTIVE；若恰逢轮换已不再 ACTIVE，整笔回滚并从 resolve 重来。事务提交后该 PENDING 引用即进入 RETIRING→RETIRED 的残留引用守卫，轮换不得使本版本后续块漂移到新 key。外部 KMS 调用不放入 PostgreSQL 事务。

第二段，事务外：从 staging 流式读取分片，用会话级临时密钥解密，边解密边计算整份明文 SHA-256。发布加密的明文块大小固定为 `1,048,576` bytes（1 MiB），只有最后一块可为 `1..1,048,576`；`chunk_no` 从 0 连续。每块构造阶段 2 唯一 `Aad::for_attachment_chunk(legal_entity_id,security_level,attachment_object_id,attachment_version_id,total_plaintext_len,chunk_no)`，其认证 bytes 逐字为 `ASCII("EP-ATTACHMENT-CHUNK-AAD-V1\0") || legal_entity_id[16] || u16be(security_level) || attachment_object_id[16] || attachment_version_id[16] || u64be(total_plaintext_len) || u32be(chunk_no)`，并以第一段同一个 handle 调 `wrap_with_data_key`。调用方只取得单块 `CipherEnvelope` 并编码 EPC1；选钥、DEK 解封/缓存、nonce 与 AES-256-GCM 全在 KMS adapter 内，`ep-platform-file` 与 `ep-adapter-file` 均不得读取 `wrapped_key`、取得明文 DEK 或自行 AES。崩溃续传不得再选 current，而是从 PENDING 行 strict parse `dek_ref` 后以 `DataKeySelectorV1::ExactRef(ref)` 恢复同一 handle；ref 的 deployment/legal-entity/domain/purpose/scope 归属不等、REVOKED/CORRUPT 均拒绝，仍可读的 RETIRING/RETIRED 可继续。

正式文件容器唯一为 `EPA1`，所有整数大端。固定 24-byte header 为：offset 0 `EPA1[4]`；4 `format_version=1[u8]`；5 `flags=0[u8]`；6 `reserved=0[u16]`；8 `plaintext_chunk_size=1048576[u32]`；12 `total_plaintext_len[u64]`，范围 `1..=5,368,709,120`；20 `chunk_count[u32]`，必须恰为 `ceil(total_plaintext_len/1048576)`。随后恰有 `chunk_count` 条连续 record，每条为 `chunk_no[u32] || plaintext_len[u32] || envelope_len[u32] || EPC1[envelope_len]`；chunk_no 必须从 0 无缺口，非末块 plaintext_len 恰 1,048,576，末块取余且整除时仍为 1,048,576，`envelope_len` 必须恰等于 `plaintext_len+51`，EPC1 的 data-key id/version 必须逐块等于 `attachment_versions.dek_ref`，末条后禁止尾随 bytes。因非末 record 固定为 1,048,639 bytes，第 i 块 record 起点可直接计算为 `24 + i*1,048,639`；不得另建旁路索引或把密文长度写回数据库。

EPA1 经 `ep-adapter-file` 的 `published` 命名空间以 `create_new` 写入目标路径，写完 `FlushFileBuffers`。`create_new` 的语义不变——目标已存在即失败——原括注的 POSIX 标志名 `O_CREAT | O_EXCL` 在本平台不是被测对象，随之删去，本平台由 `CREATE_NEW` 创建处置承接。有一层没有等价物，如实写下：Linux 侧「再对父目录 `fsync` 一次以把目录项落盘」这一层在本平台不存在，本节因此不承诺目录项在崩溃后必定可见。该差别不新增机制去补，其后果由下文的崩溃收敛任务承接——收敛任务按「路径上文件是否存在」判定，目录项未落盘即等同于文件不存在，走置 `FAILED` 一支，不会产生半截可见的已发布版本。若目标路径已存在，判为前次崩溃后的重入，但不得直接跳过验证：必须以 `ExactRef(dek_ref)` strict parse 全部 EPA1/EPC1、逐块解密并重算明文长度/hash，通过后才进入第三段。

下载与 HTTP Range 只映射明文坐标。授权、法人/密级/版本可见性与删除标记检查必须先完成，之后才打开正文。无 Range 时顺序解密全部块并返回 200；Range 只接受单个且无空白的 `bytes=<start>-<end>`、`bytes=<start>-` 或 `bytes=-<suffix-length>`，十进制只允许 `0` 或无前导零正数，逗号多段、空项、溢出、end<start、start≥total 或 suffix=0 一律返回 416 和 `Content-Range: bytes */<total>`。end 超过末字节时截到 `total-1`，suffix 大于 total 时取全长；合法单段返回 206、`Accept-Ranges: bytes`、精确 `Content-Range` 与明文 `Content-Length`。读取时只调用一次 `resolve_data_key(...,ExactRef(dek_ref))`，按上述常量公式 seek 到首块，只对相交块以同一 handle、同一 `Aad::for_attachment_chunk` 调 `unwrap_with_data_key` 并裁切首尾；不得从第 0 块顺序解密到起点，也不得返回 multipart ranges。EPA1/EPC1 magic/长度/序号/ref/总大小/尾随不等映射既有 `PLATFORM.CRYPTO.CIPHERTEXT_FORMAT_INVALID`；AAD/tag 不等映射 `PLATFORM.CRYPTO.AAD_MISMATCH`；key 已不可恢复或载体错误映射 `PLATFORM.CRYPTO.DECRYPT_FAILED`，任何失败都不返回部分正文。

第三段，事务 B：先完成 EPA1 header/record/EPC1/ref 的 strict readback，并校验重算的明文大小/hash 分别等于 `size_bytes/content_hash` 与上传声明；随后把 `attachment_versions.state` 由 `PENDING` 置 `AVAILABLE` 并写 `available_at`，更新 `attachment_objects.current_version_no`，把 `upload_sessions.state` 置 `COMMITTED`，写 `platform.attachment.published.v1` 到 Outbox，最后写审计事件。写入次序按判定二，审计是本事务的最后一次数据库写入。提交后异步删除 staging 分片。

staging 的加密取舍：分片以会话级临时密钥加密后落盘，而不是明文落盘。理由是恶意内容检查需要明文，若明文落盘则在检查与加密之间存在一段明文驻留窗口，规格第 6.5 章要求“附件在所属法人密钥域内加密存储”，明文窗口虽不在正式路径上但仍是同一台服务器上的可读副本。采用会话级密钥后，扫描路径改为流式解密到管道，明文不落盘。staging 目录的权限位换 NTFS ACL：断继承并显式设 DACL，只授 core-server 的服务虚拟账户 `NT SERVICE\ep-core`，不保留 `BUILTIN\Users` 一类的继承 ACE，也不设共用本地组；原 0700 在 ACL 上可精确表达，判据由三位八进制相等降为 ACL 集合判否，这是判据锐利度下降不是防护下降。删除路径按裁定 F-08 第 4.3 节第 3 条改写：本节的删除，含本节第三段提交后异步删除分片在内，在 NTFS 上不保证成功——他人持有该文件句柄且未带 `FILE_SHARE_DELETE` 时返回拒绝访问，而 Linux 的 unlink 总能成功——因此删除路径须带有限重试，重试用尽仍失败即登记一条失败记录并告警，不得静默吞掉，也不得把「会话终态后立即删除」写成必然成功。同一成因下，staging 目录与附件存储根目录必须列入杀毒排除，该项进部署记录。staging 不进入任何写出与备份范围。

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

本阶段只交付上述 trait 与两个 DTO，`apps/core-server/src/wiring/` 与 `apps/job-worker/src/wiring/` 两个目录下的全部文件中都不出现该端口的注入行，实现类型 `OpsDisposalService`（位于 `crates/platform/obs/src/disposal.rs`，只由 ops 专用路径与专用账号触发）由阶段 14 交付并与注入行同批落地。原空实现连同原阶段 14 占位注释一并删除，理由是一个返回成功的空壳会把一次未接线的物理删除记成一次已完成的处置，而处置回执是对外可出具的凭证。`DisposalPort` 是裁定通则第三条列明的三项例外之一，因此按例外档处理而不是整条推迟：本阶段注册处置受理路由 `POST /api/v1/platform/disposals`，该路由随附件上传流水线属 3b-2 批；本阶段至阶段 13 之间的物理删除请求一律以 `PLATFORM.DISPOSAL.NOT_DELIVERED` 直接拒绝，分类 `BUSINESS_CONFLICT`，HTTP 409，`retryable` 为假，该码由本阶段登记进 `docs/error-codes.md` 的 `PLATFORM` 段；拒绝的同时经阶段 2 的 `DegradationLedger` 开一条 `kind` 取 `PORT_NOT_IMPLEMENTED`、`subject` 取 `DisposalPort` 的降级窗口，界面与健康端点显式呈现该能力未交付，指标 `ep_degradation_windows_open` 自动计数，阶段 14 注入 `OpsDisposalService` 后关窗。不以不注册路由返回 404 替代该降级窗口的理由是规格第 12.4 章要求处置请求的传播与结论生成可验证的处置清单，404 会使本阶段至阶段 13 之间的处置请求没有任何登记与留痕。

恶意内容检查的唯一实现口径如下。`ContentInspector` 有两个内建实现：`TYPE_SNIFF` 按魔数识别真实类型并与声明类型比对，不一致且不在允许的等价集合内即 `REJECT`；`STRUCTURE` 检查可执行文件头、OOXML 与 ODF 中的宏与外部引用、PDF 中的 JavaScript 与自动动作、归档炸弹（压缩比与展开深度上限）。首版基础产品不交付 CLAMD、病毒引擎或病毒库，不存在第三个内建实现，也不存在待定 socket、自动重新生效或隐藏配置分支。

病毒扫描模式是部署必答项，唯一取值为 `NONE` 与 `CUSTOMER_ICAP`。`NONE` 模式在两个内建检查器均为 `PASS` 后追加一条 `inspector=VIRUS_ICAP, verdict=SKIPPED, detail=MODE_NONE` 的证据并允许发布；同时以 `DegradationKind::VIRUS_SCANNER_NOT_AVAILABLE`、全局 scope、subject `VirusScan` 打开不可抑制降级窗口，健康端点、运维中心、交付说明与客户合同必须显示“平台未提供病毒防护”。`CUSTOMER_ICAP` 模式由 core-server 以 `NT SERVICE\ep-core` 连接 `\\.\pipe\ep-integ`，经下文唯一分块协议把附件明文流式交给 integration-gateway；只有 integration-gateway 到客户自管扫描器的一跳允许回环 ICAP TCP。ICAP URL 只允许 `icap` scheme 且主机必须是 IP 字面量 `127.0.0.1` 或 `[::1]`，禁止主机名、DNS、系统代理、协议代理、重定向和非回环地址，不新增产品监听端口，明文不得离开该服务器。返回 `PASS` 才允许发布；恶意命中记 `REJECT`，超时、协议错误或扫描器不可用记 `ERROR`，两者均保持附件 `QUARANTINED`，按共享八步退避重试且不得自动退回 `NONE`、不得人工绕过。配置且健康时关闭同一降级窗口；运行中失联时立即重开。基础产品只实现 ICAP 客户端，不安装、许可、更新或运维客户的病毒引擎与病毒库。

`ep-integ` 管道协议冻结如下。管道 server 身份是 `NT SERVICE\ep-integ`，DACL 客户端 ACE 只含 `NT SERVICE\ep-worker`、`NT SERVICE\ep-core`、`NT SERVICE\ep-ops`；server 在读取任何应用字节前执行 `ImpersonateNamedPipeClient`→`OpenThreadToken` 校验服务 SID/账户，并在所有分支 `RevertToSelf`，PID 只作审计关联。worker 只可调用 `push.dispatch.v1`、`esign.request.submit.v1`、`esign.status.get.v1`，并只在同一已关联的签章双工连接接收 gateway 反向发送的 `esign_file.begin.v1`、`esign_file.chunk.v1`、`esign_file.end.v1`、`esign_file.abort.v1`；core 只可调用 `virus_scan.begin.v1`、`virus_scan.chunk.v1`、`virus_scan.end.v1`、`virus_scan.abort.v1`；ops 只可调用 `health.get.v1`、`metrics.snapshot.v1`。client 以 `SECURITY_SQOS_PRESENT|SECURITY_IDENTIFICATION` 打开管道并在发送前核验 server 进程 token。`reject_remote_clients=true`；仅 bootstrap 首实例取 `first_pipe_instance(true)`，后续/补位实例取 `false`。普通操作沿用 4 字节大端长度前缀加 JSON，整帧不超过 1 MiB；上述十三个 operation 是完整封闭集合，不实现通配白名单或同义 operation。

integration-gateway 运行期数据库能力固定为零：不持 `ep_app_rw` 或其他数据库凭据、不建连接池、不链接 `ep-platform-outbox`、不持 KMS 或平台业务文件目录权限。推送、签章、病毒扫描只返回清洗后的稳定码和结果；签章文件使用上述四个 `esign_file` operation 与同一 `BoundedChunkStreamV1` 反向流给 job-worker，整批逐件执行「临时加密对象→长度/SHA-256/TYPE_SNIFF/STRUCTURE→`NONE|CUSTOMER_ICAP`→签章验签→数据库确认/发布」。仅整批全部 `PUBLISHED` 才建附件及签章关联并允许合同转 `SIGNED`；任一步失败均使对象保持 `QUARANTINED`，合同不得转 `SIGNED`。gateway 只保留一块与固定协议状态，不落盘、不写附件元数据。

病毒流最大 5 GiB。begin 为 `{request_id, attachment_object_id, total_len, content_sha256}`，request_id 是每次尝试新建的 UUIDv7，total_len 取 0..5368709120，哈希是 32 字节 SHA-256 的 64 位小写十六进制；chunk 为 `{request_id, seq, data_b64, chunk_sha256}`，seq 从 0 连续，解码后每块 1..524288 字节且最后一块可小于上限，空文件不发 chunk；end 为 `{request_id, next_seq, total_len, content_sha256}`。每块必须收到包含同 request_id 与 ack_seq 的 ACK 后才可发送下一块，最多一块在途；integration-gateway 只持单块、滚动哈希与 ICAP 协议固定开销，正文不落盘。块 ACK 超时 10 秒、空闲超时 30 秒、会话绝对上限 3600 秒；乱序、重复、缺块、累计长度/块哈希/最终哈希不符、调用方取消或任一超时均立即执行 `virus_scan.abort.v1`，关闭 ICAP 会话并清零缓冲。失败重试必须使用新的 request_id，从 begin 重来，不续接旧会话。

检查未通过的处置：版本行置 `QUARANTINED`，不可被任何单据引用、不可下载，保留 `quarantine_retention_days`（固定默认 90）后由处置流程删除；安全管理员可在保留期内查看扫描证据并选择提前删除或重新扫描，不提供恢复为可用的人工绕过。U-L-03 据此关闭。

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

守卫条件表达式：本阶段只交付最小求值器，支持字段引用（`vars.x`、`instance.state`）、比较（六种）、逻辑（与或非）、集合成员、空判定，以及一个不超过 12 个函数的白名单（长度、上取整、日期加减等）。表达式无副作用、无循环、求值步数上限 1000，超限返回 `VALIDATION`。该求值器只服务于流程守卫条件，不是 `RuleEvaluator` 的实现。完整的声明式规则引擎与受限 WASM 计算不在本阶段范围，本阶段只保证接口位点存在：`ep_platform_flow::port::RuleEvaluator` 与 `ep_platform_flow::port::WasmComputePort` 两个 trait 定义在 `ep-platform-flow`，按裁定 B-05，其实现类型分别为 `AstRuleEvaluator`（位于 `crates/platform/meta/src/rule/`，装配进 core-server）与 `PluginHostWasmCompute`（跨进程实现，位于 `crates/adapter/ipc/`，装配进 core-server 与 job-worker；plugin-host 侧的进程内执行实现为 `WasmtimeComponentCompute`，位于 `crates/adapter/wasm/`，两个 adapter crate 互不依赖，见裁定 H-02），两者均由阶段 13b 交付。两者与 `DisposalPort` 同属裁定通则第三条列明的三项例外，一律按例外档处理：本阶段至阶段 13b 之间两个 wiring 目录下的全部文件中都不出现这两个端口的注入行，`NoopRuleEvaluator` 与 `NoopWasmComputePort` 两个空实现不再存在；流程守卫命中这两项能力时经阶段 2 的 `DegradationLedger` 开一条 `kind` 取 `PORT_NOT_IMPLEMENTED`、`subject` 分别取 `RuleEvaluator` 与 `WasmComputePort` 的降级窗口，界面与健康端点显式呈现该能力未交付，并返回可重试错误或直接拒绝，不静默按成功路径放行，阶段 13b 注入实现后关窗。端点 `POST /api/v1/platform/rule-evaluations/actions/evaluate` 属阶段 13b，本阶段不建第二条求值路径。

#### 3.4.9 错误分类与重试

分类到 HTTP 的映射按基线第 5.5 节固定，不增不减。按裁定 C-24，`PLATFORM.IDEMPOTENCY.KEY_REQUIRED`、`PLATFORM.IDEMPOTENCY.PAYLOAD_MISMATCH`、`PLATFORM.CAPACITY.CONCURRENCY_LIMIT`、`PLATFORM.CONCURRENCY.STALE_VERSION`、`PLATFORM.AUTHZ.NOT_FOUND_OR_DENIED`、`PLATFORM.AUTHZ.OBJECT_FORBIDDEN`、`PLATFORM.DB.MIGRATION_WINDOW_CLOSED` 七个码由阶段 1 登记在 `crates/foundation/src/error/codes.rs` 与 `docs/error-codes.md` 的 `PLATFORM` 段，本阶段只引用，登记冲突由 CI 判负。本阶段实现四件事。

其一，`AppError` 到响应封套的统一映射中间件，位于 core-server 的 HTTP 层，只做一次翻译，各 crate 的 `Error` 到 `AppError` 的映射写在各自 `error.rs`。

其二，`incident_no` 的生成：格式 `ERR-<YYYYMMDD>-<6 位法人内流水>`，由 `ep-platform-sequence` 以 `scope_kind = 'DOCUMENT'`、`type_code = 'ERR'` 分配。事故编号必须在错误路径上可分配，因此该分配走独立短事务，不参与已回滚的业务事务；分配失败时退化为 `ERR-<YYYYMMDD>-<trace_id 前 12 位>`，保证错误响应永远有关联编号，规格第 15.1 章要求“每个错误包含关联编号”。

其三，数据库重试：序列化失败 40001 与死锁 40P01 在 `ep-adapter-db-pg` 的工作单元层重试 3 次，退避 50、150、450 毫秒，且只对尚未产生任何外部可见副作用的事务重试。“外部可见副作用”在本阶段的判定为：该事务尚未调用过任何 `ep-adapter-file` 的写入方法、尚未调用过 integration-gateway。重试次数计入阶段 2 注册并填充的 `ep_db_tx_retries_total`（counter，标签 `pool` 与 `sqlstate`），按裁定 C-21 本阶段不登记 `ep_tx_retry_total`。重试耗尽使用基线唯一码 `PLATFORM.DB.SERIALIZATION_RETRY_EXHAUSTED`，分类 `INFRASTRUCTURE`，`retryable = true`。

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

写入路径只有一条：job-worker 的索引消费者按法人轮转消费 Outbox 事件，在业务事务提交之后调用 `upsert` 与 `remove`。业务事务内不得出现这两个方法的调用，理由是基线第 10.3 节禁止事务内做文件正文读写，索引写入是文件写入。`xtask archcheck` 断言 `SearchIndexPort` 的调用点只出现在 job-worker 装配的消费者模块中，core-server 的用例路径上出现即构建失败。消费失败按 Outbox 八档安排八次重试，首投加八次重试均失败后转死信，不影响业务事务。

索引按法人分区，物理路径 `C:\EP\search\<legal_entity_id>\`，落在按裁定 F-08 第 4.3 节第 4 条取短名的安装根之下（默认 `C:\EP`），不放在深层路径下。同条要求的最坏路径长度留证由本阶段出具一次并记入部署记录，被算的是两条路径：本目录，与第 3.4.7 节附件三段式的 `<legal_entity_id>/<security_level>/<yyyy>/<mm>/<version_id>`，各段按其最大取值宽度在该根下实算最坏长度。理由是 `LongPathsEnabled` 是全机注册表值，按该裁定第零节授权边界第 2 条不得要求改客户机器的系统设置；本仓自己的文件访问由 Rust 标准库对绝对路径的 `\\?\` 前缀转换兜住，但随产品交付的 `pg_*.exe` 与客户侧的备份代理不保证如此。分区是行级安全在索引侧的等价物：查询必须带 `legal_entity_id`，跨法人查询按法人逐轮发起，不做跨分区合并查询。`SearchQuery.max_security_level` 取自 `SecurityContext.clearance_level`，不接受调用方传参；命中结果仍按数据库侧的可见性复核一次，索引不作为授权判据，与规格第 7.9 章派生存储安全继承一致。

本阶段不交付任何业务对象的检索文档投影函数。投影由各业务阶段按 `SearchDocument` 结构提供，本阶段只提供合成对象的投影用于自测。

#### 3.4.11 模块许可与生命周期（3b 段）

`license_trusted_signer_subjects` 是可识别 identity roster，不是撤销状态。每次 CAB 签发新 deployment manifest 前，必须在变更门关关闭期间扫描当前数据库全部 RELEASED special inner+outer 历史引用 token，新 signed roster 必须是该 exact set 的超集；任一已引用 token 被删除即轮换失败，不得释放 manifest。保留已撤销 token 只保证历史可识别，不会重新授权：验签仍先按 CRL prerequisite 分类为 REVOKED；新 artifact 同时要求 token 在 roster 且整链 ACTIVE。真正移除已引用 token 只能通过可信整库回退到尚无该引用的状态，或创建新 deployment，不存在原地清历史分支。

F-56 CMS 链首版冻结为低成本窄 DER/RFC profile，不锁某个 crate 或私有 API；实际实现版本只由 `Cargo.lock` 与 SBOM 固定。所有 license chain certificate 一律拒绝 `nameConstraints|certificatePolicies|policyMappings|policyConstraints|inhibitAnyPolicy`。leaf extension 闭集恰为必需 SKI、AKI、KU、EKU 与可选 BC：KU 只含 `digitalSignature`，EKU 只含 `codeSigning`，BC 只能 absent 或 `CA=false`。每张 CA（intermediate 与 anchor）的 extension 闭集恰为 SKI、AKI、critical BC 与 critical KU：`BC.CA=true`且 pathLen 必须实际执行，KU 恰含 `keyCertSign+cRLSign`。所有未列 certificate extension 不论 critical 与否一律拒绝。每份完整 base CRL 的 extension 只允许且必须有 AKI 与 CRLNumber；拒绝 IDP、delta CRL indicator、freshest CRL 与任何其他 CRL extension，所有 revoked-certificate entry extension 均拒绝。这一闭集替代“库默认 policy 通过”或模糊 critical-extension 判断，并与本节算法/时间/唯一链/CRL prerequisite 共同失败关闭。

initial-governance 审计不允许自由形状。全部 bootstrap evidence 投影摘要共用唯一函数 `projection_digest(domain,dto)=SHA-256(ASCII(domain)||0x00||JCS(dto))`，每个 typed root 的 `schema_version` 都是 JSON number `1`、`purpose` 逐字等于 domain。`platform.bootstrap.initial_governance.v1` typed audit ABI 的完整 envelope 固定 `event_id=<事务内预分配 UUIDv7>`、`legal_entity_id=<signed governance_legal_entity_id>`、`actor_user_id=SYSTEM_PRINCIPAL_ID` 且命中本事务已建立的同法人 ACTIVE SYSTEM grant、`actor_device_id=null`、`action='INITIAL_GOVERNANCE_BOOTSTRAPPED'`、`object_type='platform.initial_governance'`、`object_id=bootstrap_id`、`object_version=1`、`before=null`、`after=<下述 exact root>`、`reason=null`、`approval_ref=null`、`reauth_ref=null`、`client='system'`、`occurred_at=committed_at`；`event_day/seq/prev_hash/hash` 只由 `AuditWriter` 既有分段链算法派生。receipt 的 `audit_event_id/committed_at` 必须与它相等。audit `after` unknown/missing key 失败的 exact root 为 `{schema_version:1,purpose:"EP-INITIAL-GOVERNANCE-AUDIT-V1",bootstrap_id,deployment_id,bootstrap_body_sha256,bootstrap_authorization_registry_sha256,initial_license_archive_sha256,deployment_manifest_sha256,database_bootstrap_projection,database_bootstrap_projection_sha256,receipt_body_sha256,schema_manifest_sha256,ep_migrate_pe_sha256,committed_at,status:"COMMITTED"}`。

`database_bootstrap_projection` 的 domain 恰为 `EP-INITIAL-GOVERNANCE-DATABASE-PROJECTION-V1`，root exact 为 `{schema_version,purpose,legal_entity,key_domain,operators,legal_entity_grants,roles,role_permission_pairs,user_role_grants,approval_chains}`；`approval_chains` 是 F-56 冻结的 37 项复数集合，不接受 singular 别名。各 child 形状、null 语义、排序与 exact-set 逐字采用 F-56 首装冻结的同名 database projection ABI，不得另造第二套。`database_bootstrap_projection_sha256=projection_digest("EP-INITIAL-GOVERNANCE-DATABASE-PROJECTION-V1",database_bootstrap_projection)`；`bootstrap_authorization_registry_sha256` 同样只对 F-56 冻结的 canonical authorization registry DTO 求 domain-separated digest。所有数组按该 ABI 的 wire bytes canonical 排序；`receipt_body_sha256=SHA-256(receipt exact JCS bytes)` 不自包含，预分配 event id 与冻结 committed_at 使 receipt bytes 可在 audit INSERT 前唯一重建，不存在 digest 循环。after 与 projection 只绑定 receipt 引用 ids/digests/mapping，严禁密码、PHC/verifier、credential secret、证书正文或其他秘密。

CRL-revoked signer 的 DISABLE 窄恢复路径必须以具名 typed audit 闭合。除本次 recovery item 自身唯一 `platform.config_special.accepted.v1` 外，同一 audit terminal batch 还必须恰写一条 append-only `action='MODULE_SIGNER_REVOKED_DISABLED'`。写 batch 前预分配两个互异的新 UUIDv7，唯一链顺序为先写 recovery event、再写 accepted event，且 accepted event 必须为该 batch 最后一条；same-byte 幂等回放只返回既有结果，不分配、不追加也不重排。两事件共享冻结治理法人、同一次 execute 的受信 `SecurityContext.actor_user_id/actor_device_id/client` 与 `config_packages.approval_ref`，两者 `reason/reauth_ref` 均为 null。recovery event 的完整 envelope 其余固定 `object_type='platform.module_registrations'`、`object_id=<锁定 current row id>`、`object_version=after.row_version`、`before=<下述完整 current projection DTO>`、`after=<下段 recovery metadata DTO>`、`occurred_at=after.disabled_at`；accepted event 的其余 envelope 逐字采用本阶段同名冻结。两事件的 `event_day/seq/prev_hash/hash` 只由 `AuditWriter` 按该顺序链式派生。该 audit 的 `before` 是更新前完整 strict DTO `{schema_version:1,purpose:"EP-F56-CURRENT-MODULE-PROJECTION-V1",id,module_code,display_name,row_version,install_state,package_id,package_code,package_version,package_payload_sha256,package_signature_cms_sha256,package_signer_subject,package_signed_at,module_contract_version,module_contract_sha256,min_platform_version,max_platform_version_exclusive,released_on,source_config_package_id,source_config_item_id,installed_at,state_changed_at,enabled_at,disabled_at,last_transition_reason}`；`schema_version` 是 JSON number `1`，SemVer 是 strict object/null，digest 64 lowerhex，time 是 UTC whole-second。

audit `after` 只是 recovery metadata 闭集 `{schema_version:1,purpose:"EP-MODULE-SIGNER-REVOKED-DISABLED-V1",module_code,previous_source_config_package_id,previous_source_config_item_id,recovery_config_package_id,recovery_config_item_id,before_projection_sha256,after_projection_sha256,disabled_at,reason_sha256}`，其 `schema_version` 同样是 JSON number `1`。四个 source/recovery id 分别逐字取更新前 current row 与本次 RELEASED DISABLE item；`reason_sha256=SHA-256(ASCII("EP-MODULE-DISABLE-REASON-V1")||0x00||UTF-8(recovery item reason))`。投影摘要唯一函数为 `SHA-256(ASCII("EP-F56-CURRENT-MODULE-PROJECTION-V1")||0x00||JCS(dto))`；`before_projection_sha256` 必须从 audit.before exact DTO 重算，after DTO 只能由 before 做 checked `row_version+1`、`install_state=INSTALLED_DISABLED`、`state_changed_at=disabled_at=occurred_at`、`last_transition_reason=recovery item reason` 的唯一变换派生，并保留 previous source 两列与全部旧 inner/package 投影；再以同一函数求 `after_projection_sha256`。row version 溢出、id/version/time、reason、任一 digest 或 accepted event 不闭合均整事务回滚。Stage 14 只能用该 action、审计 hash chain、before/after 投影摘要与 accepted event 派生 recovery peer，禁止按相同 inner/package id 或最近时间猜选。

F-56 覆盖裁定 A-05 的旧短表、旧四态和旧模块边。唯一运行时契约落在 `ep-platform-license`：

```rust
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum ModuleState { NotInstalled, InstalledEnabled, InstalledDisabled }
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum LicenseStatus { Active, ExpiringSoon, GracePeriod, Restricted }
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum LicenseRestrictionReason {
    NotYetValid, ExpiredBeyondGrace, Revoked, SignatureInvalid, NoCurrentGrant,
}
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum LicenseKindV1 { Perpetual, Subscription }
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum EntitlementCodeV1 { F55LocalAi, F55Mcp }
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum ModulePackageActionV1 { Install, Enable, Disable, Upgrade, RollbackVersion }

pub struct LicenseEvaluationV1 {
    pub status: LicenseStatus,
    pub restriction_reason: Option<LicenseRestrictionReason>,
    pub trusted_now: DateTime<Utc>,
}

pub trait ModuleLicenseQuery: Send + Sync {
    fn module_state(&self, module: ModuleCode) -> Result<ModuleState, AppError>;
    fn license_evaluation(&self) -> Result<LicenseEvaluationV1, AppError>;
    fn module_is_currently_licensed(&self, module: ModuleCode, legal_entity_id: Uuid) -> Result<bool, AppError>;
    fn entitlement_is_currently_licensed(&self, entitlement: EntitlementCodeV1, legal_entity_id: Uuid) -> Result<bool, AppError>;
    fn feature_is_enabled(&self, feature_code: &str, legal_entity_id: Uuid) -> Result<bool, AppError>;
}
```

五方法语义不可互换：`module_state()` 只返回 `module_registrations.install_state` 的原始管理投影，不代表业务可用；`module_is_currently_licensed()` 是模块 effective-runtime admission，只有 raw state=`INSTALLED_ENABLED`，current module 的 outer/inner/source/accepted digest 与 `product-modules.v1.jcs` 全部有效，且同一 current grant 有效、含目标 module、请求法人命中 scope 时才 `Ok(true)`。合法的未安装/停用/未授权/范围外等负态返回 `Ok(false)`；结构、IO、摘要、source、catalog 歧义或复验异常返回 `Err(AppError)`，调用者必须失败关闭而不能把异常当可用或静默缓存 false。`feature_is_enabled()` 必须先通过 feature row 与 owner module 的上述 effective gate；`module_state()` 不得被业务守卫直接用作授权。

`ModuleCode` 是阶段 1 冻结的 15 值枚举，本阶段只消费。两类 `after_spec` 形状不同且不可互换：`LICENSE_GRANT` 恰为 `SignedBusinessArtifactV1<LicenseArtifactPayloadV1>`；`MODULE_PACKAGE` 恰为 `ModulePackageItemV1 { schema_version:1, action, reason, artifact: SignedBusinessArtifactV1<ModulePackageManifestV1> }`，签名封套位于 `artifact` 字段，不允许把整个 module item 当成通用 envelope。两者都是 strict JSON、UTF-8 无 BOM、unknown/duplicate field 拒绝且规范体不超过 1,048,576 bytes；`Sha256Digest` 的 JSON wire 恰为 `[0-9a-f]{64}` ASCII lowerhex，解码后数据库 `bytea` 只存 32 raw bytes，拒绝大写、`0x`、base64、数组或别名。每个 signed artifact 都要求 `payload_sha256=SHA-256(JCS(payload))`，`signature_cms_b64url` 只收 canonical base64url-no-pad，解码后是最大 1,048,576 bytes 的 detached CMS，detached content 恰为 `JCS(payload)`。SignerInfo 只接受 ECDSA P-256/SHA-256（AlgorithmIdentifier parameters absent）或 RSA-PSS/SHA-256（modulus 至少 3072、MGF1-SHA256、saltLength=32、trailerField=1），leaf 必须有 DigitalSignature KeyUsage 与 Code Signing EKU；链中每张证书与每份 CRL 自身的签名 AlgorithmIdentifier 也只允许 ECDSA-with-SHA256（issuer key=P-256、parameters absent）或同一 exact RSA-PSS，拒绝 SHA-1、RSA PKCS#1 v1.5、NULL/默认/隐式参数及其他算法。`signer_subject` 安全 wire 恰为 `spki-sha256:<64 lowerhex>`，摘要输入是 signer certificate exact DER SubjectPublicKeyInfo，必须逐字命中签名 `DeploymentManifestV1.license_trusted_signer_subjects` 唯一 roster；本地 `release.trusted_signer_subjects` 只能作空值或与该 roster 顺序一致的 exact-equal 断言，不参与授权。X.509 display DN 仅供界面/审计显示，不参与 JCS、身份比较或授权。验签只用固定 release root；禁止当前用户/本机任意根存储、联网补链、DEV/file key、命令行公钥和临时根。外层配置包签名与内层 CMS 必须独立通过，原始 license envelope 或完整 module item 原样保存在 `after_spec`，表 25/26 的 signature 列保存对应 artifact 的内层 CMS exact bytes。

签名状态从唯一完整链导出，不只看 leaf。`signed_time` 对 outer 取 manifest 的 RFC 3339 `signed_at`，对 inner 取 payload/manifest 的 RFC 3339 `issued_at`；CMS ASN.1 `signingTime` 必须与它语义上是同一 UTC whole-second instant，不能比较文本 bytes，DER 在 1950..2049 年只用 UTCTime、其余只用 GeneralizedTime，全部 Z-only、含秒、无小数、无 offset。non-anchor 是 leaf 加全部 intermediate：每张都必须在 signed_time 落入自身有效期；anchor 必须在 signed_time 有效并通过自签/CA/KeyUsage/critical-extension 检查，trusted_now 后 anchor 自身过期不触发 RETIRED，但从 bundle 移除/替换或形成多链立即 UNTRUSTED。

整条 non-anchor 链的 `REVOKED > ACTIVE > RETIRED > UNTRUSTED` 优先级只在全链 CRL prerequisite 成功后适用。结构、唯一链与 signed_time 验证后，先为每个实际签发 non-anchor 的 issuer 建立唯一 global-highest 且覆盖 trusted_now 的合法完整 base CRL；任一 issuer 缺失、过期、尚未生效、最高号同号冲突或 CRL 非法，整链立即 UNTRUSTED，不得继续扫描其他 issuer serial，也不得进入任何 CRL recovery。只有 issuer 全集成功才扫描全部 serial：任一命中整链 REVOKED；零命中且每张 non-anchor 当前有效才 ACTIVE；零命中、全部 signed_time 有效、当前至少一张过期且无 not-yet-valid，且首次 ACTIVE 接受/source/digest/signature 自洽，才 RETIRED并仅复验既有 RELEASED current/history。新 import/release outer 与首次 GRANT/REVOCATION/INSTALL/UPGRADE inner 只接 ACTIVE；既有合法 current/history 可 ACTIVE 或 RETIRED-nonrevoked。

离线 CRL 也无实现分支：从 CMS leaf 经 bundle intermediate 到 anchor 必须恰一条有效链；链上每个实际签发 leaf/intermediate 的 issuer 都必须在 bundle 内有 X.509 v2 完整 base CRL，CRL issuer DER Name/AKI 分别匹配 issuer certificate 的 subject DER Name/SKI，签名有效，并带 CRLNumber 与 nextUpdate。对每 issuer 先独立选择 `thisUpdate<=trusted_now<=nextUpdate` 且 CRLNumber global-highest 的唯一一份；最高号缺失/尚未生效/过期、同号内容冲突、无覆盖项、delta/indirect/removeFromCRL 或 unknown critical extension 令 prerequisite 整体失败并按 SIGNER_NOT_TRUSTED，绝不退旧 CRL。仅全部 issuer 选择成功后才扫描 serial；不读 CDP、不查 OCSP、不联网或软失败。

许可 payload 是 `Grant|Revoke` tagged union。Grant 恰含 `schema_version=1`、`purpose='EP-LICENSE-GRANT-V1'`、`grant_id`、`license_no`、`deployment_id`、`governance_legal_entity_id`、`issued_to`、UTC 秒精度 `issued_at`、kind、`valid_from/valid_to/maintenance_valid_to`、法人 scope/ids、三项 usage limit、module codes、`F55_LOCAL_AI|F55_MCP` entitlement 和 `supersedes_grant_id`；Revoke 恰含 `schema_version=1`、`purpose='EP-LICENSE-REVOCATION-V1'`、`revocation_id/deployment_id/grant_id/license_no/issued_at/reason_code`。数组按 wire bytes 排序去重，除 entitlement 可空外均非空；ALL 要求 entity ids 为空，LIST 要求 1..1024 个且必须包含治理法人；三个 limit 各 1..1,000,000。SUBSCRIPTION 要求 `valid_to>=valid_from` 且 maintenance 等于 valid_to；PERPETUAL 要求 valid_to 为空，maintenance 为空或不早于 valid_from。零 current 是合法的未供给/恢复状态；current slot 只保证至多一张。首张有效 grant 无前驱且接受事务提交后恰有一张 current；此后续期必须在同一持锁事务直接替换 current，撤销命中 current 后保留该 current 行并立即受限。每次 GRANT/REVOCATION apply 都服从下文 whole-transaction 第一条业务 SQL、锁内重读、`pre_import_trusted_now` 和新 grant `last_trusted_at` 初值的唯一算法；因此零 current 的并发首发也不得退化成“先查无行再各自插入”。deployment id 必须逐字等于签名部署清单与 Stage 14 current deployment 证据，候选 `issued_at>pre_import_trusted_now+5min`、签名/链/治理法人/范围/投影/直接后继/撤销对象任一不符均整项零写入拒绝。

每次 `ModuleLicenseQuery` 调用都读取同一 repeatable-read current grant 快照，从表列重建 payload，复验 digest、CMS、当前离线信任根、deployment 与法人 scope，并按表 26 后 `TrustedClockV1` 公式只计算、不写行；无 current、多个 current、解析/验签异常或投影不等均失败关闭，不得任选历史行或沿用缓存布尔值。`license_evaluation()` 必须从这一个快照一次性返回 `LicenseEvaluationV1 { status, restriction_reason, trusted_now }`：ACTIVE/EXPIRING_SOON/GRACE_PERIOD 的 reason 必为空，RESTRICTED 恰有一个封闭原因；ServerAdmin、告警和运行时守卫都不得分三次查询再拼接，避免状态、原因与可信时间撕裂。`last_trusted_at` 只由 readiness、推进 special package 的受控关口与 target cadence 不超过 240 秒的 checkpoint 在固定 advisory-lock/CAS 事务推进；它与 current grant digest 一并写不可篡改审计，但不持久化 usage snapshot。行内 `trust_bundle_sha256` 只作接受时溯源，不参与“必须等于当前 bundle”的判定；当前 bundle 必须匹配签名部署清单，计划 CAB 轮换及非计划磁盘漂移的失败关闭算法以表 26 后段为唯一口径。

状态边界以 `trusted_date` 判定：SUBSCRIPTION 在 `valid_from` 前为 `RESTRICTED/NOT_YET_VALID`；`trusted_date < valid_to-60 days` 为 ACTIVE；从 `valid_to-60 days`（含）至 `valid_to`（含）为 EXPIRING_SOON；到期后第 1..30 个自然日为 GRACE_PERIOD；第 31 日起为 `RESTRICTED/EXPIRED_BEYOND_GRACE`。PERPETUAL 从 valid_from 起为 ACTIVE，maintenance 到期不改变运行态；两种许可一旦被已验签撤销命中都立即 `RESTRICTED/REVOKED`，签名失效或无 current 分别为 `SIGNATURE_INVALID|NO_CURRENT_GRANT`。`ACTIVE|EXPIRING_SOON|GRACE_PERIOD` 均属当前有效，GracePeriod 全功能可用但扩大告警。

RESTRICTED 的允许写闭集除既有查询、报表、审计、备份、导出、身份安全处置、合规更正/删除/销毁与在途 Outbox/Saga 收敛外，只增加两条自恢复通道：其一，`LICENSE_GRANT` 的 import → 九套 autotest → submit → 仅 `client=win|mac` 的 approve/reject → outer sign → RELEASE order/execute 全链；其二，仅 action=DISABLE 的 `MODULE_PACKAGE` 同一全链。无 current、签名失效、过期或撤销均不得拦这两条；MODULE_PACKAGE 的 INSTALL/ENABLE/UPGRADE/ROLLBACK_VERSION 不在恢复闭集。其他常规业务写入、业务审批、集成出站和新自动化任务一律返回 `PLATFORM.LICENSE.RESTRICTED`，不得落成通用权限拒绝或静默只读。

许可准入的唯一 owner 是 `crates/platform/license/src/admission.rs`，其 ABI 固定如下；effect 的 wire 值逐 variant 取 `SCREAMING_SNAKE_CASE`，首版不得增删改名：

```rust
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum LicenseAdmissionEffectV1 {
    BusinessWrite,
    BusinessApproval,
    IntegrationOutbound,
    AutomationStart,
    ReadReportAuditBackupExport,
    IdentitySecurityDisposition,
    ComplianceDisposition,
    InFlightConvergence,
    LicenseGrantRecovery,
    ModuleDisableRecovery,
}

pub struct LicenseAdmissionRequestV1 {
    pub effect: LicenseAdmissionEffectV1,
    pub legal_entity_id: Option<Uuid>,
}

pub enum LicenseAdmissionBindingV1 {
    Fixed(LicenseAdmissionEffectV1),
    ConfigRelease { fallback_effect: LicenseAdmissionEffectV1 },
    McpInbound,
}

pub trait LicenseAdmissionGate: Send + Sync {
    fn admit(&self, request: &LicenseAdmissionRequestV1) -> Result<LicenseEvaluationV1, AppError>;
}
```

`admit` 只从一次 `license_evaluation()` 的同一 current 快照作决定并在放行时原样返回该 evaluation。全局 Restricted 时，`BusinessWrite|BusinessApproval|IntegrationOutbound|AutomationStart` 全部返回 `PLATFORM.LICENSE.RESTRICTED`；后六种 effect 是闭合集，其中读/报表/审计/备份/导出、身份安全处置、合规处置、严格在途收敛及两条许可恢复能力按各自既有权限继续。全局为 ACTIVE、EXPIRING_SOON 或 GRACE_PERIOD 时，前四种普通 effect 只要操作存在已鉴权目标法人，就必须以 `Some(target_legal_entity_id)` 验证签名 LIST scope；不命中同样以 `PLATFORM.LICENSE.RESTRICTED` 零写入，但不得把全局 `LicenseStatus` 改成 Restricted、不得生成新的 `LicenseRestrictionReason`，也不得污染 ServerAdmin 部署级状态。`legal_entity_id` 只能由受信会话/对象上下文或 worker 载荷绑定取得，真正无目标法人的部署级命令才取 `None`；已有目标法人的 route/job/event/owner/IPC 若传 `None` 必须由 binding/上下文一致性检查失败关闭，不能借 null 绕过 scope，且不接受调用方用任意 body 字段覆盖。范围外拒绝不拦查询、报表、审计、备份或导出，也不改变许可恢复闭集；module/entitlement/feature 的逐法人 false 与准入必须来自同一 current 快照。

`LicenseAdmissionBindingV1::ConfigRelease` 只允许用于配置包 `import|run-autotest|submit|approve|reject|sign|create-release-order|execute` 八类操作。解析器必须在事务内锁定 package 与唯一 item（import 则绑定本次不可变上传 bytes），按 F-56 strict shape 识别 `LICENSE_GRANT` 的 Grant/Revoke 为 `LicenseGrantRecovery`、只识别 `MODULE_PACKAGE` 且 `action=DISABLE` 为 `ModuleDisableRecovery`；任何普通项、其他模块动作或 strict 解析失败都取注册时的 fallback。approve/reject 的 fallback 固定 `BusinessApproval`，其余六类固定 `BusinessWrite`，不得由 URL 参数、客户端或包内自报 effect。`McpInbound` 只从已验签 manifest binding 的 `ActionClass` 映射：Read/Export→`ReadReportAuditBackupExport`，Approve→`BusinessApproval`，Write/Submit→`BusinessWrite`；manifest 未验签、无 binding 或未知 ActionClass 先失败关闭，不得猜测 effect。

所有外部 HTTP `/api/v1`、`/portal`、`/mcp` 路由的注册行都必须把原 `(CapabilityDomain,ActionClass)` 扩成 `(CapabilityDomain,ActionClass,LicenseAdmissionBindingV1)`，不得在 handler 内临时选 effect。认证前置入口不借用普通 route 默认值：凭证验证成功之后、会话签发之前固定调用 `Fixed(IdentitySecurityDisposition)`，使 Restricted 下仍可登录并执行安全处置，同时未验证凭证不能探测许可。core-server 与 job-worker 对每个 scheduler、Outbox dispatcher、approval owner callback 和出站 IPC operation 另设非 HTTP binding registry；配置发布 owner callback 沿用上面的 `ConfigRelease` strict target，其他项必须逐条显式登记且没有 fallback 默认值。

`InFlightConvergence` 只覆盖外部副作用已经发生之后的回执落库、终态推进、取消，或保证不产生任何新外部/业务副作用的补偿；它绝不允许首次外发、重试外发、新 claim、新业务效果或启动另一自动化。尚为 PENDING/DISPATCHING 的外发首次与重试始终登记 `IntegrationOutbound`，Restricted 时保留耐久队列、不领取也不派发；不得改标为 convergence 绕过。`apps/core-server/src/wiring/` 与 `apps/job-worker/src/wiring/` 中装配的 binding 集合必须分别与实际 route/job/event/owner/IPC 注册表 exact equal，缺项、多项、重复项、错误 binding 或同一 operation 多 effect 都失败关闭。

永久许可的 `maintenance_valid_to` 只控制新产品、模块、安全补丁和连接器更新；manifest `released_on` 越界拒绝安装/升级，已安装能力和数据不停止，签名撤销与紧急安全撤下不受该日期限制。ServerAdmin 每次读取和每日 monitor 都在同一 repeatable-read snapshot 实时计算三项 usage：`legal_entity_limit` 统计 `legal_entities.is_active=true`；`named_user_limit` 统计 `account_kind<>'SYSTEM'` 且 `status in ('ACTIVE','LOCKED','SUSPENDED')`；`registered_device_limit` 统计 `user_devices.status in ('PENDING','ACTIVE')`。每日 monitor 只刷新 metrics 并在越限/恢复边缘发既有告警；不新增 usage 表、日终持久化快照、月度签名商业申报、联网遥测或向发行方发送数据。授权管理员只能通过既有导出取得当时点报表并走普通导出审计，不能把它称作连续日终/月报。超限不阻断创建；module/entitlement scope 才是硬门。F-55 `currently_licensed` 在前三态为 true、Restricted 为 false；`purchased` 只从仍能按当前 bundle 验签且未标记 `HISTORICAL_SIGNER_REVOKED` 的 current/history grant 得出，不放行业务。

声明式模块 manifest 恰含 `schema_version=1`、`purpose='EP-MODULE-PACKAGE-V1'`、package identity 与三段 u16 semver、module code、UTC 秒精度 issued_at、released_on、平台半开兼容区间、module contract version/digest、`data_on_disable='RETAIN'`、`package_kind='DECLARATIVE_BUILTIN_MODULE'`；动作项恰含 `schema_version=1`、五值 action、1..1000 UTF-8 bytes reason 与签名 artifact。契约摘要必须等于签名产品 manifest 内该模块编译期摘要，当前平台版本必须落在区间。包只能声明统一产品中已编译存在的模块；任何 DLL、EXE、script、SQL、WASM、container、附件、hook、任意路径、URL 或 capability grant 字段都按 unknown-field 整项拒绝。

“签名产品 manifest”唯一指待签发布目录的 `target/release-package/product-modules.v1.jcs` 与安装后固定 `C:\EP\product-modules.v1.jcs`，最大 262,144 bytes、UTF-8 无 BOM、strict RFC 8785 JCS。DTO exact 为 product_version 加按 wire 排序的 15 行 modules；每行只有 `module_code,module_contract_version,module_contract_sha256,module_dependencies`，其中 version wire 为 1..=2,147,483,647 的 JSON integer，digest JSON 为 64 lowerhex/内存 raw32，dependencies 排序去重、只指 15 值闭集、不得自依赖且全图 DAG。

contract digest 不得再来自手写不透明常量。仓库必须恰有 `contracts/modules/{mdm,crm,cpq,clm,sales,procure,inventory,costing,project,service,finance,ledger,invoice,portal,reporting}.contract.v1.jcs` 15 个 descriptor，每个至多 262,144 bytes、UTF-8 无 BOM、RFC 8785 JCS exact bytes，DTO exact 为 `ModuleContractDescriptorV1 { schema_version:1, purpose:"EP-MODULE-CONTRACT-V1", module_code, module_contract_version:1..=2147483647, module_dependencies, abi_entries }`。Rust 字段仍用 u32，但 descriptor、product manifest、module package、parser 与数据库有效域统一 1..=i32::MAX，持久化前必须 checked conversion，禁止截断/cast/wrap；文件名与 module code 一一对应。dependencies 按 wire 排序去重、不得自依赖且全图 DAG；`abi_entries` 为 1..4096 项，按 `(kind wire,code UTF-8 bytes)` 排序且组合唯一，kind exact 为 `COMMAND|QUERY|EVENT|JOB|PERMISSION`，code 匹配 `[a-z][a-z0-9_.-]{0,127}`，每项只含 `kind,code,schema_sha256`。

每个 ABI schema 的唯一前像是 `contracts/modules/<wire>/schemas/<64-lowerhex>.schema.v1.jcs` exact bytes；文件至多 65,536 bytes、strict RFC 8785 JCS，root `$schema` 逐字为 `https://json-schema.org/draft/2020-12/schema`，`$ref` 只允许同文档 `#` fragment，禁止网络、文件或外部引用；文件名、entry digest 与重算 SHA-256 必须三者相等。`module_contract_sha256=SHA-256(descriptor exact bytes)`，version 逐字取 descriptor；descriptor 任一 byte、dependency、ABI 或 schema digest 改变都必须严格增加 version，同 version 不得换 digest。

每个 `ep-contract-<module>` 的 `MODULE_CONTRACT_VERSION`、`MODULE_CONTRACT_SHA256` 与 `MODULE_ABI_REGISTRY` 都由 descriptor 生成；`cargo xtask module-contracts verify` 对 compiled public command/query/event/job/permission registry 与 descriptor entries 做双向 exact-set，并重算全部 schema、descriptor 与 DAG，缺、多、重、漂移一律构建失败。`product-modules.v1.jcs` 只从这 15 个已验证 descriptor 的 version/digest/dependencies 和工作区 canonical product version 生成后 strict 回读；不得从环境变量、数据库、模块包、第二份 dependency registry 或手写 Rust 常量取值。

该文件是 `MANIFEST.sha256` closed roster 的必有 regular file，其 digest 同时受签名 manifest 与产品 Authenticode CAB 覆盖。安装器以 safe handle copy/flush/atomic publish/readback；core/worker 每次模块动作及运行门从 `C:\EP` fixed-root safe-handle resolver 打开，拒绝 reparse/ADS/hardlink/path drift，先核对 digest 命中已验签 `MANIFEST.sha256` 再 strict parse。Stage 14 product manifest projection 固定包含 exact file digest、product_version、15 行 contract/dependency digest 与 DAG 结论；这是既有部署证据投影，不为 F-56 新增表、列、迁移或第二份产品目录。

全部 RELEASED MODULE_PACKAGE history 还必须保持两套 identity 一一映射：同一 `package_id` 只能对应同一份 exact inner artifact；同一 `(module_code,package_code,package_version)` 只能对应同一个 `package_id` 与同一 exact inner。ENABLE/DISABLE/ROLLBACK_VERSION 重复带回相同 inner 合法；不同 payload/digest/signature/signer bytes 冒用任一 identity 时，在 release 锁内以 `PLATFORM.CONFIG_PACKAGE.SPECIAL_ITEM_SHAPE_INVALID` 整项拒绝。093300 的同一 deferred F-56 graph 在 COMMIT 扫描 RELEASED history 强制该规则，不新增历史表或运行时任选分支。

动作签名状态逐边冻结：首次 INSTALL 与指向新 artifact 的 UPGRADE 要求 inner signer、该 action 的 special outer signer 都为 ACTIVE；ENABLE/DISABLE 必须复用既有 RELEASED inner exact artifact，该 inner 可为 RETIRED 但不得 REVOKED，且本次新 action outer 必须 ACTIVE；ROLLBACK_VERSION 只能精确引用既有 RELEASED historical artifact，inner 可 RETIRED 但不得 REVOKED，新的 rollback action outer 仍须 ACTIVE。所有 action 都禁止 RETIRED outer 接受新 release；current inner 和/或 current source outer REVOKED 只允许下述 DISABLE 窄逃生口，不能借给其他动作。

模块状态机的合法动作只有下列五条：

| 当前态 | 动作 | 结果 | 守卫 |
|---|---|---|---|
| NOT_INSTALLED | INSTALL | INSTALLED_DISABLED | 部署级动作，无法人参数；current 有效许可同时含目标与依赖闭包全部 module codes；manifest、平台、契约与维护权通过；依赖不要求已安装或已启用 |
| INSTALLED_DISABLED | ENABLE | INSTALLED_ENABLED | 部署级动作，无法人参数；artifact identity 与已安装投影完全相等；current 有效许可同时含目标与依赖闭包全部 module codes，且每个依赖都已为 `INSTALLED_ENABLED`；模块自检通过 |
| INSTALLED_ENABLED | DISABLE | INSTALLED_DISABLED | 不要求当前许可或维护期，故 Restricted 中也允许；但 special item 的已验签 artifact identity 必须与当前安装包逐字段相等；先撤路由/写入，再排空在途，停止定时器与新 Outbox dispatch |
| INSTALLED_DISABLED | UPGRADE | INSTALLED_DISABLED | 部署级动作；current 有效许可含目标/依赖，目标 semver 严格更高，manifest 兼容且维护权有效 |
| INSTALLED_DISABLED | ROLLBACK_VERSION | INSTALLED_DISABLED | 部署级动作；current 有效许可含目标/依赖，目标为历史已验签且仍兼容版本，必须发起新的显式审批，不是通用配置回退 |

不存在 `NOT_INSTALLED→INSTALLED_ENABLED`、启用态升级、卸载/DELETE、direct SQL 或把降版本伪装 UPGRADE 的边。模块安装态与动作都是部署级，不接收 `legal_entity_id`；法人 scope 只在逐法人业务请求的 `module_is_currently_licensed`/entitlement/feature 查询中判定。非法边返回 `PLATFORM.MODULE.TRANSITION_INVALID`；授权不足返回 `PLATFORM.MODULE.LICENSE_REQUIRED`；签名、契约或兼容不符返回 `PLATFORM.MODULE.PACKAGE_INVALID_OR_INCOMPATIBLE`。DISABLE 的“无条件”只表示不要求 current 许可或维护期，不代表可用任意旧包：special item 必须携带当前安装时那份 `SignedBusinessArtifactV1<ModulePackageManifestV1>` 的 exact bytes，module code 与 package id/code/version/payload digest/signature/signer/signed-at/contract/兼容区间/released-on 必须逐字段等于当前安装投影，否则零写入拒绝。

若当前安装 package 的 inner signer 和/或其 current RELEASED source special outer signer 被当前 bundle CRL 明确标为 REVOKED，该模块的业务写、审批、自动化与外发运行门立即关闭，旧 package 不再作任何当前正向证明，且 inner/outer signer 已 REVOKED 的历史 package 永不再作 ENABLE/ROLLBACK_VERSION 证据或 rollback candidate。唯一停用逃生口仍使用上述旧 `SignedBusinessArtifact` exact bytes，不重新签 inner：由 ACTIVE signer 只对新的 F-56 special outer `manifest.toml` 签 detached CMS，并创建 `action=DISABLE`。该窄例外仅在旧 inner、旧 source outer、不可变 source item/接受摘要及其 payload/digest/signature/current projection 全部自洽，失败类别只能是旧 inner 和/或旧 source outer `CRL_REVOKED`，且未撤销的另一层为 ACTIVE 或 RETIRED-nonrevoked 时成立；任一 bad digest/signature/source、断链或不能唯一分类的损坏均拒绝并要求可信恢复。它只允许 DISABLE，仍须完整九套测试、双人审批、新 outer+旧两层证据的边界复核、独占排空与审计；不得据此 INSTALL、ENABLE、UPGRADE、ROLLBACK_VERSION，也不得把 ACTIVE outer signer 冒充为旧 inner identity。停用提交后，若部署许可仍有效，只能再以 inner signer 与 special outer signer 都为 ACTIVE、semver 严格更高且其余 UPGRADE 守卫全通过的新包执行正常 UPGRADE，替换 revoked current projection；旧包自此只作版本/审计历史，不能成为任何正向证明。

`ModuleOperationGate` 的 key 固定为 `hashtextextended('platform-module:' || ModuleCode wire,0)`。业务写事务在读取业务行前取得目标 module 的 transaction-level shared lock，worker 在真实领取/派发前以专用连接取得目标 module 的 session-level shared lock并在 finally 释放；持锁后调用 effective module gate，递归验证签名产品 DAG 的全部依赖、许可、inner/outer/source/接受摘要与产品目录，任一依赖负态不读 payload、不产生业务或外部效果。DISABLE 为安全且低成本固定按 15 个 ModuleCode wire 顺序取得全部 15 把 transaction-level exclusive lock，总 `lock_timeout=30s`，锁齐后重读全图但只改变目标 module；因此任一目标业务 shared holder 都能阻止并发停用任一依赖，新 shared 请求在等待中的 DISABLE 后排队。ENABLE 在一个事务按目标加传递依赖的全局 wire 顺序取锁，目标为 exclusive、依赖为 shared，锁齐后重验依赖均 effective/INSTALLED_ENABLED；INSTALL/UPGRADE/ROLLBACK_VERSION 仍取目标 exclusive。30 秒是整组锁总预算而不是每把重置，超时整笔返回 `PLATFORM.MODULE.IN_FLIGHT_DRAIN_TIMEOUT`、零状态/幂等/Outbox/审计部分变化。

通过 DISABLE 后它属于 Restricted 自恢复闭集：固定先撤 UI/HTTP/写端口或置只读，再禁止新定时任务与该模块的新 Outbox dispatch，在上述全 15 exclusive 锁内按时间投影写 INSTALLED_DISABLED、幂等结果与审计终结批并提交；所有业务表、附件、审计、配置包、包 identity 与许可历史均保留，授权查询、报表、审计、备份、导出和合规处置继续。重新 ENABLE 依上述目标-exclusive/依赖-shared 规则逐次重验内外签名、current 有效许可中的目标/依赖闭包、每个依赖均为 `INSTALLED_ENABLED`、契约与数据 schema，失败时保持 disabled。

过滤点两处，都在 job-worker：定时器扫描取件后按实例所属定义的模块段解析 `ModuleCode`，Outbox 投递取件后按 `event_type` 的模块段解析 `ModuleCode`，并从耐久载荷取得受信目标法人；真实领取/派发前依上段取得目标 shared lock并调用 `module_is_currently_licensed(module,legal_entity_id)`，由该 effective gate 递归检查依赖。`Ok(false)` 时条目保持 `PENDING` 且不累加 `attempts`，`Err` 时失败关闭并告警，绝不退化为 raw `module_state`；模块及依赖恢复后自动投递。这落实规格第 5.6 章模块停用后停止定时任务与对外事件，且不把停用误判为投递失败。

许可状态与模块开关不进启动自检。基线第 7.3 节的 `license-and-modules-consistent` 整项删除，阶段 1 注册的 Pending 项一并撤销，该项已不在基线第 7.3 节现行命名集合中，见第 3.12.1 节偏离五。运行期模块、feature 与 entitlement 每次只经上述 trait 判定；不满足时执行精确受限后果并写许可告警/审计/ServerAdmin 状态，不借用任一 `DegradationKind` 开窗。第 3.7 节新增的 `license-admission-registry-consistent` 只比较编译期/装配注册集合，不读取 current grant、许可状态或模块开关，故不恢复已删除的业务自检。系统进程在 Restricted、无 current 或签名失效时仍可启动，避免把可恢复商业状态误做整企业启动闸门。

Stage 3b 完成全部数据投影、解析/验签、可信时间、并发续期、撤销与五条动作的单元/真实 PostgreSQL 集成测试；复用配置包 API 的内外签名、九套自动测试、双人审批、ServerAdmin 组合和停用再启用端到端验收由阶段 13b 补齐，最终共同进入 `RG-LICENSE-MODULE-LIFECYCLE-GREEN`。

#### 3.4.12 配置发布的内容项端口与最小发布通道

`LICENSE_GRANT|MODULE_PACKAGE` special 包的 `RELEASED` 是永久终态。首次 RELEASE 后不得进入普通配置包的 `SUPERSEDED|ROLLED_BACK`，也不参与后续普通 lineage 自动 supersede；每份新 grant/revoke/module action 都新建另一份仍为 RELEASED 的单项包，多份 special RELEASED 同时存在是唯一正确历史形状。current/history/superseded grant 与 current module 只由 `license_grants/module_registrations` 投影及 source FK 表达；任何 special 置 SUPERSEDED/ROLLED_BACK、RELEASED 摘要为空、非 RELEASED 摘要非空或清摘要，都必须由 093300 deferred graph 在 COMMIT 拒绝。

3a 段先交付 `crates/platform/release/src/port/config_item.rs` 的下列 trait、`ItemKind` 类型、`ConfigPackageItem` DTO 与 `ConfigItemApplierRegistry`；3b 按 F-56 把 Rust 枚举/`ItemKind::ALL` 与数据库 CHECK 同步扩为阶段快照 18 项。trait 方法签名与阶段 13 计划第 4.6 节逐字一致，其中 `Tx` 取自 `ep_foundation::port::tx`：

```rust
pub trait ConfigItemApplier: Send + Sync {
    fn item_kind(&self) -> ItemKind;
    fn validate(&self, item: &ConfigPackageItem, ctx: &SecurityContext) -> Result<(), AppError>;
    fn apply(&self, tx: &mut dyn Tx, item: &ConfigPackageItem, ctx: &SecurityContext) -> Result<(), AppError>;
    fn revert(&self, tx: &mut dyn Tx, item: &ConfigPackageItem, ctx: &SecurityContext) -> Result<(), AppError>;
    fn requires_derived_store_rebuild(&self, item: &ConfigPackageItem) -> bool;
}
```

`validate` 没有 `Tx`，因此语义只允许对传入 DTO 做 pure、deterministic 的 syntax/shape 校验；不得查询数据库、KMS、文件、当前许可/模块状态或任何可变外部事实，也不得把其成功当作发布授权。`apply` 才是权威写入边界：F-56 两个 applier 只能在下述全局锁序已建立后，从数据库持久化的 exact package/item bytes 重新执行 signature/trust/current/source/dependency/governance 与业务守卫，只有 locked `apply` 全部通过才可提交；事务外 safe-parse 或先前 `validate` 的结论一律不能替代锁内复验。

Stage 3 的 18 项固定顺序为 CUSTOM_OBJECT、CUSTOM_FIELD、CUSTOM_RELATION、CUSTOM_INDEX、CUSTOM_VIEW、UI_LAYOUT、FLOW_DEFINITION、AUTHZ_ROLE、AUTHZ_POLICY、AUTHZ_FIELD_GRANT、REPORT_DEFINITION、METRIC_DEFINITION、DASHBOARD_DEFINITION、PRINT_TEMPLATE、NOTIFY_RULE、RULE、LICENSE_GRANT、MODULE_PACKAGE。阶段 13b 的既定 090500 在尾部追加 MCP_CONNECTOR、MCP_MANIFEST_VERSION，形成终态 20，且同批更新 Rust `ItemKind::ALL`；两个阶段各自保证 Rust 与数据库 CHECK 相等。`ConfigPackageItem` 字段为 `id`、`security_level`、`config_package_id`、`item_kind`、`item_code`、`change_kind`、`applies_to_legal_entity_ids`、`before_spec`、`after_spec`、`item_hash`、`sort_no`、`accepted_trust_bundle_sha256`。注册表只按 kind 注册/查找并装配于两个 wiring；3a 仍无表、无用例、不依赖身份与授权。

全平台 `item_hash` 保持兼容并补齐 REMOVE：ADD/MODIFY 固定为 `SHA-256(JCS(after_spec))`，REMOVE 固定为 `SHA-256(JCS(before_spec))`，保存 64 位 lowerhex；被选 spec 必须非 null，任何路径都禁止对 null 求摘要。import、autotest、submit、`actions/sign` 与 RELEASE execute 每次按 `change_kind` 从数据库列重算；kind/code/change/sort/scope 与相应 before/after 空值形状由包 manifest 和表约束绑定，不另造 digest 列。MODULE_PACKAGE 的 action/reason 已在 after_spec 内，因此 ADD 的摘要覆盖二者。

F-56 special `.epcfg` 是单卷 ZIP32/`STORE` archive，file hard cap 为 4,193,900 bytes；exact root entry 集合恰为 `manifest.toml`、`item.jcs`、`outer-signature.p7s`。禁止 ZIP64、加密、data descriptor、extra field、archive comment、目录项、重复名/大小写碰撞、绝对/反斜杠/`.`/`..` 路径、symlink/hardlink/reparse 属性、尾随数据和嵌套 archive；三个 entry 都是 regular file，DOS 时间固定 `1980-01-01 00:00:00`，CRC 与 central/local size/offset 必须和实际 bytes 一致，三 entry 的固定 ZIP header/directory overhead 恰为 330 bytes。`item.jcs` 最大 2,882,850 bytes，是唯一 item 的 `after_spec` 经 RFC 8785 JCS 后的 exact bytes，故 special `item_hash=SHA-256(item.jcs exact bytes)` lowerhex；它与 262,144-byte manifest、1,048,576-byte outer CMS、330-byte ZIP overhead 的最大值相加恰为 archive 上限。special 固定 ADD/before null，不存在空 after。

special 导入错误分类没有第二套解释：archive/ZIP/TOML/item JSON/base64 的语法、长度上限、CRC、entry 集或 container metadata 任一失败，统一 `PLATFORM.REQUEST.INVALID_PAYLOAD`（HTTP 400、不可重试、零落库）；typed item 重算 hash 不等返回 `PLATFORM.CONFIG_PACKAGE.ITEM_HASH_MISMATCH`；CMS 签名/摘要/属性的密码学失败返回 `PLATFORM.CONFIG_PACKAGE.SIGNATURE_INVALID`；证书链、CRL、EKU、root 或 signer subject 不受信返回 `PLATFORM.CONFIG_PACKAGE.SIGNER_NOT_TRUSTED`；strict DTO 已完成后的 special 业务 shape/metadata/governance 偏离才返回 `PLATFORM.CONFIG_PACKAGE.SPECIAL_ITEM_SHAPE_INVALID`。`PLATFORM.MODULE.PACKAGE_INVALID_OR_INCOMPATIBLE` 只能在签名与信任均通过后用于 version、contract、maintenance、history identity 或兼容性失败，不得吞并前述错误。

`manifest.toml` 最大 262,144 bytes，是 UTF-8 无 BOM、LF-only、末尾恰一 LF 的 canonical TOML；root 与唯一 `[[items]]` 的键、顺序和值逐字采用 F-56，item 固定指向 `item.jcs` 且携带同一 kind/code/change/sort/scope/null-before/hash。import 后及 `actions/sign`/RELEASE 均须由保存列通过同一 canonical writer 重建 exact manifest bytes，禁止依赖通用 TOML reserialize 或 staging 文件；`content_hash=SHA-256(manifest.toml exact bytes)`。`outer-signature.p7s` 是最大 1,048,576 bytes、恰一 SignerInfo 的 DER detached CMS，detached content 恰为 manifest exact bytes，`messageDigest=content_hash`，signed attributes 闭集为 `contentType=data,messageDigest,signingTime`；ASN.1 signingTime 与 manifest RFC 3339 `signed_at` 必须语义上为同一 UTC whole-second instant，并遵守上文 UTCTime/GeneralizedTime DER 闭集。special outer 新导入及每个推进关口都必须为 ACTIVE，并与 inner 一样只信固定 `license-roots.p7b`，但 outer 与 inner 独立验签，普通包的部署 KMS outer verifier 又是第三条独立路径。

special parser 在 JSON/TOML 转义解码完成后递归检查每个最终会进入 PostgreSQL text/jsonb 的 string，统一禁止 U+0000；该检查必须早于 package/item 持久化、staging 之外文件写入、审计、Outbox 或任何业务副作用。manifest/item 内业务 string 的直接 NUL、JSON `\u0000` 或 TOML basic-string `\u0000` 一律映射 `PLATFORM.CONFIG_PACKAGE.SPECIAL_ITEM_SHAPE_INVALID` 且零落库；容器本身语法不合法仍使用既有 container/parser 错误，不能把两层错误混为一类。canonical writer 也拒绝内存中含 U+0000 的值，防止绕过 import parser。

本阶段实现四个 applier：`FlowDefinitionApplier`（`ep-platform-flow`，3b-1）、`NotifyRuleApplier`（`ep-platform-notify`，3b-2）、`LicenseGrantApplier` 与 `ModulePackageApplier`（均在 `ep-platform-license`，3b-2）。终态其余 16 个的属主固定为阶段 4 三个 AUTHZ、阶段 11 四个 reporting、阶段 13b 六个 CUSTOM/UI 加 RULE、阶段 13c 两个 MCP；总数恰为 20。查不到实现的 `item_kind` 整包拒绝发布，错误码 `PLATFORM.CONFIG_RELEASE_ORDER.APPLIER_NOT_REGISTERED`，分类 `BUSINESS_CONFLICT`。

`LICENSE_GRANT` 与 `MODULE_PACKAGE` 是 F-56 特殊单项包：`source=IMPORTED`、`item_count=1`、唯一 item 的 `change_kind=ADD`、`before_spec=null`、`after_spec` 非空且 `applies_to_legal_entity_ids=[]`；仍须跑完整九套自动测试、`CONFIG_RELEASE` 双人职责分离审批、外层签名和发布执行。任一形状不符整包以 `PLATFORM.CONFIG_PACKAGE.SPECIAL_ITEM_SHAPE_INVALID`（409、不可重试）拒绝；只能创建 `action=RELEASE` 的发布单，对该包创建通用 `ROLLBACK` 以 `PLATFORM.CONFIG_RELEASE_ORDER.NON_ROLLBACKABLE_ITEM`（409、不可重试）拒绝。两类 applier 的 `revert` 不在合法编排图中；若被直接误调也返回同一 NON_ROLLBACKABLE 错误且零写入。发布后只能用新的已签名许可/撤销或模块动作单项包显式继承，禁止 UPDATE/DELETE 历史内容项、删除旧包、通用 revert 或数据库回滚抹除历史。

所有可能推进 F-56 special 的事务采用唯一全局锁序：先执行 `pre_idempotency_lock=LICENSE_CURRENT_EXCLUSIVE`；仅 ordinary execute 随后取得 `platform_meta.config_release_mutex FOR UPDATE`，special execute 不取 mutex 且跳过 DDL 段一；然后才按 `(config_package_id,release_order_id,item.sort_no,item.id)` canonical tuple 锁 package/order/item rows，再按 ModuleCode wire bytes 升序取得所需 module shared/exclusive locks，最后在锁内重读 current/history/source/dependency、重算可信时间与全部守卫并写 projection/package/order/Outbox/audit。ordinary execute 的连接 1 持有 license/mutex 至 COMMIT。该规则逐字覆盖 import、autotest、submit、approve、sign、create-release-order 与 execute；这些动作不得先 `try_begin`、claim、查 package、锁 mutex/package/order 或领取 task 再由 applier 补取 license lock。import 可在事务前 safe-parse archive 以识别 special，但该结果非权威，事务内仍从候选并最终持久化的 exact bytes 复验。GRANT、REVOCATION 与 MODULE_PACKAGE applier 可幂等重取同一 transaction-level exclusive lock；“第一句”始终指 whole transaction。reject 在进入事务前由 typed branch 固定 `NONE`，不推进 artifact、可信时间或 projection，只锁自身 package/flow rows 闭合同一 immutable content hash；不得在无锁事务查包后改判。

special RELEASE execute 遵循上述全局锁序，在同一事务取得 package/order/唯一 item 及所需 module locks 后，复验 exact archive 投影、outer、inner、item/content hash 与当前部署清单 bundle 摘要；全部通过后才允许把该 item 的 `accepted_trust_bundle_sha256` 首次从 null 写为此次实际 bundle 的 32-byte SHA-256，并在同一事务执行 applier 投影、把 package/order 置 RELEASED/SUCCEEDED。三部分任一失败全部回滚，禁止先写摘要、先发布或事后补投影；已非空的幂等重放只接受 same-byte 已完成事实，不得覆盖或清空。普通 item 始终为 null；grant 投影的 `trust_bundle_sha256` 写同一值，revocation 和 module action 不复制新列，只由各自不可删除的 source item 提供接受证据。

每次 special 首次 RELEASE 还必须在同一事务、所有投影与 RELEASED/SUCCEEDED 写入之后以 audit terminal batch 精确写唯一事件 `action='platform.config_special.accepted.v1'`；完整 envelope 还固定 `event_id` 为本次 RELEASE terminal batch 前预分配的新 UUIDv7，`legal_entity_id` 为冻结治理法人，`actor_user_id/actor_device_id/client` 逐字取 execute 的受信 `SecurityContext`，`object_type='platform.config_package_items'`，`object_id=config_item_id`，`object_version` 为 terminal item row_version，`before=null`，`after=上述 payload`，`reason=null`，`approval_ref=config_packages.approval_ref`，`reauth_ref=null`，`occurred_at=accepted_trusted_now`；`event_day/seq/prev_hash/hash` 只由 AuditWriter 既有分段链算法派生；same-byte 幂等回放只回放既有结果，不追加第二条。payload 是 unknown/missing key 均失败的 strict JCS 闭集：`{schema_version:1,purpose:"EP-CONFIG-SPECIAL-ACCEPTED-V1",config_package_id,config_item_id,artifact_kind,artifact_id,artifact_action,accepted_trusted_now,accepted_trust_bundle_sha256,inner_signer_subject,inner_chain_sha256,inner_trust_state,outer_signer_subject,outer_chain_sha256,outer_trust_state,payload_sha256,item_hash,content_hash,source_projection_sha256}`。`artifact_kind` 仅 `LICENSE_GRANT|LICENSE_REVOCATION|MODULE_PACKAGE`，`artifact_id` 分别为 grant/revocation/package UUID；前两类 `artifact_action=null`，模块类取五个 ModuleAction wire 之一。outer state 必须为 `ACTIVE`；inner state 只允许 `ACTIVE|RETIRED_NONREVOKED|REVOKED_AS_DISABLE_TARGET`，后两值分别仅用于已接受 inner 的合法复用与唯一 CRL-DISABLE 窄路径，新 GRANT/REVOCATION/INSTALL/UPGRADE 必须 ACTIVE。

`inner_chain_sha256/outer_chain_sha256` 唯一算法为 `SHA-256(ASCII("EP-CMS-CHAIN-V1") || 0x00 || 对 leaf→intermediate→anchor 每张 exact DER 依次追加 u32be(length) || DER)`；signer token 必须等于该链 leaf exact DER SPKI token。`source_projection_sha256=SHA-256(ASCII("EP-CONFIG-SPECIAL-SOURCE-PROJECTION-V1") || 0x00 || JCS(dto))`，其中 terminal `dto` 闭集恰为 `{schema_version:1,purpose:"EP-CONFIG-SPECIAL-SOURCE-PROJECTION-V1",config_package_id,package_no,source:"IMPORTED",status:"RELEASED",content_hash,outer_signature_sha256,outer_signer_subject,outer_signed_at,config_item_id,item_kind,item_code,change_kind:"ADD",sort_no:1,applies_to_legal_entity_ids:[],before_spec_sha256:null,after_spec_sha256,item_hash,accepted_trust_bundle_sha256}`；`outer_signature_sha256` 对保存的 DER outer bytes 求摘要，`after_spec_sha256=SHA-256(item.jcs exact bytes)`。UUID 小写 canonical、时间 UTC whole-second、全部 JSON digest 64 lowerhex；`accepted_trusted_now` 取本次持锁 TrustedClock。审计 hash chain、payload、数据库 terminal projection、accepted bundle bytes 与 inner/outer exact CMS 必须可互相重算，任一不等整笔回滚。

阶段 13 的 strict multipart 导入不新建路径：仍调用 `POST /api/v1/platform/config-packages/actions/import`。Win/Mac 原有 `application/json {attachment_object_id}` 保持兼容；同一路径的 `multipart/form-data` 形态对受信入口已认证的 Win、Mac 与 ServerAdmin 三类客户端开放，并且只有这一 media type 获得编译期 route-local 4,194,304-byte body limit。两种形态共用 `lowcode.config_package.import`、同一个 `ConfigRelease` binding、幂等作用域和审计，不因 multipart 获得第二套权限或通用文件能力。请求 `Content-Type` 恰为未加引号的 `multipart/form-data; boundary=<token>`；token 为 1..70 ASCII bytes 且只含 HTTP-token 安全子集 `[A-Za-z0-9'._+-]`，其余字符一律拒绝。无 preamble/epilogue、CRLF-only。恰一个名为 `package` 的 file part，header 顺序/bytes 固定为 `Content-Disposition: form-data; name="package"; filename="<filename>"\r\n`、`Content-Type: application/vnd.enterprise-platform.epcfg+zip\r\n` 后接空行；filename 匹配 `[A-Za-z0-9][A-Za-z0-9._-]{0,121}\.epcfg`，为 7..128 ASCII bytes，零额外 header/part/form field；结尾恰为 CRLF、closing boundary、CRLF。framing size 因而恰为 `136 + 2*boundary_len + filename_len`、最大 404 bytes，archive/file hard cap 为 4,193,900 bytes。

`Content-Length` 必填、为规范十进制 `1..=4,194,304`，并逐字等于 `framing_size+archive_size`；缺失、非法、为零、超限或任何 `Transfer-Encoding` 都在读取正文/创建临时对象前拒绝。流式读取同时以 4,194,304-byte body 与 4,193,900-byte file 两个硬截止拒绝短读/长读。长度、boundary、framing、header、part、MIME、filename、扩展名或 archive 上限任一不符，全都只返回既有 `PLATFORM.REQUEST.INVALID_PAYLOAD`，HTTP 400、`retryable=false` 且配置包零落库；不得新增 413、同义错误码或泄漏解析器/文件系统错误。该窄例外不得抬高其他路由或全局 1 MiB body limit。Restricted/零 current 时，multipart 先完成严格传输解析，随后 import 的 exclusive 事务从持久化候选 exact bytes 重验；只有唯一 `LICENSE_GRANT` 才映射 `LicenseGrantRecovery`，`MODULE_PACKAGE:DISABLE` 仍按既有 strict target 映射 `ModuleDisableRecovery`，普通包与其他模块动作统一拒绝。通用 attachment init/part/complete 路由永不因此获得恢复权限，所以首张许可可由已认证 Win/Mac 直接 multipart 导入而不依赖先上传附件。

既有 `GET /api/v1/platform/client-bootstrap?client=server_admin` 只增加始终存在的可空 `license_module_admin`：仅已认证 ServerAdmin 会话且具备 `lowcode.config_package.view` 时非空；权限不足、未认证或任一非 `server_admin` client 都逐字为 JSON null，不得省略或沿用旧缓存。非空对象所有键始终存在，exact 字段为 `license_no_masked,license_kind,license_status,restriction_reason,valid_from,valid_to,maintenance_valid_to,last_trusted_at,usage,module_codes,entitlement_codes,modules`；前八项中不可用的 Option 必须序列化为 JSON null，绝不省略。status/reason/trusted time 来自一次 `license_evaluation()`。`usage` exact object 恰有 `legal_entities,named_users,registered_devices` 三键，每项的 `limit,current,over_limit` 三键始终存在且形状为 `{limit:u32|null,current:u64,over_limit:bool|null}`；`limit=null` 时 `over_limit=null`，否则严格等于 `current>limit`。`modules` 恰 15 行并按 module wire 排序，每行 `module_code,display_name,install_state,package_trust_status,package_code,package_version,state_changed_at` 七键始终存在，后三项不可用时为 JSON null；`package_version` 非空时复用 strict `SemVerV1` object。`package_trust_status` 闭集恰为 `NOT_INSTALLED|TRUSTED|SIGNER_REVOKED|INVALID`，从 current bundle 对 source item/投影重算，不能以 `INSTALLED_ENABLED` 冒充 effective runtime 可用。可信 `license_no` 至少四个 Unicode scalar 时 `license_no_masked="****"+最后四个 scalar`，不足四个时固定 `"****"`，不得泄漏原长度。零 current 固定 `RESTRICTED/NO_CURRENT_GRANT`，其余许可身份/日期/可信时间为 null、code 集为空、usage 仍返回三个实际 current 但 limit/over_limit 为 null；SIGNATURE_INVALID 也不得显示未受信 current 的身份、日期、code 或 limit。两个 code 集与 modules 均按 wire bytes 排序；unknown key、任一 missing key 或把 null 与省略互换都是 OpenAPI/序列化契约失败。所有分支都不含 signature/payload/source/path/key/secret 或原始 license_no。

3b 段的六态发布状态机按裁定 A-27 以 PRD 第 10.4.1 节的十一态为唯一出处，本阶段实现其中六态：Draft 到 PendingApproval（提交审批）、PendingApproval 到 Approved/Rejected（只由下述 owner callback 迁移）、Approved 到 Released（签名并执行发布单）、Released 到 RolledBack（回退发布单）。差异审查由 `GET /api/v1/platform/config-packages/{id}/diff` 端点承载，不单列为状态。非法迁移返回 `BUSINESS_CONFLICT` 与 `PLATFORM.CONFIG_PACKAGE.*` 的对应码，不静默忽略。其余五态 PendingAutotest、TestFailed、TestPassed、SignedPendingRelease、Superseded 由阶段 13b 补齐，扩展只放宽 `ck_config_packages_status`，不改写任何既有行。

配置发布审批的具名场景固定为阶段 4 `ApprovalScenarioCode::ConfigRelease`（持久化 `CONFIG_RELEASE`），默认链节点固定 `ROLE:SECURITY_ADMIN`。普通包提交事务按普通规则；special submit 先取 whole-transaction exclusive，再从首发候选或首次 RELEASED history 派生 `governance_context_id`，要求当前受信 session/operator 对该法人具 `lowcode.config_package.submit`，若请求头带法人则必须相等，且包此前 `approval_legal_entity_id` 必须为空；随后按 canonical tuple 锁包/项、重算 item/content hash、取 content_version 并调用唯一 `ApprovalChainResolver::resolve_active_chain`。解析成功后同一事务才首次写 `approval_legal_entity_id=governance_context_id` 与 scenario/submitted/approval/chain/content 全套证据、把包置 PENDING_APPROVAL、创建流程实例与首任务、执行幂等 finish/通知/审计终结。派生不唯一、授权/请求头不符、审批列被预填、缺链、多链、空节点/展开或自审都整笔失败，包保持 TEST_PASSED，流程、任务、通知、Outbox、审计零新增。

approve/reject 路由仅把 `{task_id,decision,note}` 交给标准 `CompleteProcessTask`；处理器自身禁止更新配置包。typed command 在进入事务前冻结分支：approve 选 `LICENSE_CURRENT_EXCLUSIVE`，在 `try_begin`、读取/领取 task 或锁任何 package/instance/task 前先取得 exclusive；reject 选 `NONE`，不得先查包再从 approve 改判。流程引擎在同一任务事务调用唯一 `ConfigReleaseApprovalCallback`，按既定 canonical rows 锁包/实例/任务，验证同一 package id、approval_ref、法人、`CONFIG_RELEASE`、发起人、chain id/version/digest、当前 content_version/hash、节点顺序与结论人非 submitted_by且属于冻结审批集合，才由 callback 原子写 APPROVED+approved_by/at 或 REJECTED+rejected_by/at/reason。任一不符整笔回滚，任务未完成、包仍 PENDING_APPROVAL并产生安全告警；状态不得由 HTTP handler、通用 Outbox 消费者或人工 SQL 改写。

外层签名与验签分两条且不得混用。普通配置包的 `actions/sign` 才使用部署 release key：`EP__RELEASE__SIGNING_KEY_REF` 的 secret ref 在本次命令只解析一次为 immutable/versioned `KeyRef`；随后以 `ep_foundation::port::kms::KmsSigningKeyIdentityResolver` 对该 exact ref 取得 `SigningKeyIdentityV1 { key_ref, spki_sha256 }` before，以同一 ref 调 `KmsBackend::sign(content_hash exact bytes)`，再以同一 ref 调 `KmsBackend::verify` 且结果必须为 true，最后再次 resolve identity after 并要求与 before 逐字相等。`SigningKeyIdentityV1::signer_subject()` 是 canonical token 的唯一生成者，固定输出 `spki-sha256:<64 lowerhex>`；SPKI 摘要必须来自该不可变 KeyRef 所指签名公钥的 exact DER SubjectPublicKeyInfo，resolver 不得返回或导出私钥。只有整条链成功，才在锁定包的同一数据库事务持久化 canonical `signature_key_ref`、该 token、signature、signed_at 并推进状态；任一 ref 漂移、identity 漂移、不可解引用或 verify=false 都零状态推进。`KmsBackend` 既有六方法不变，Builtin/HSM adapter 同时实现独立只读 resolver；密钥轮换只改变后续签名选择，不回填历史。私钥始终在载体内，`ep-platform-release` 不依赖 `ep-adapter-kms`。

F-56 imported special package 则在 import 时先经 special publisher outer verifier 通过，并把发行方 outer `signature`、`signer_subject`、`signed_at` exact bytes/值写入既有列；包从落库起不可改。批准后的 `actions/sign` 按 change-kind 算法重算 item hash、由保存列经同一 canonical writer 重建 manifest exact bytes并重算 content hash，再复验并逐字保留这组三值，然后推进 `SIGNED_PENDING_RELEASE` 和写审计；不得调用部署 KMS `sign` 或 `KmsSigningKeyIdentityResolver`，不得覆盖或重新编码发行方签名。任一 item/hash/manifest 不符都整包拒绝。

F-56 artifact 的内层 detached CMS 是第三个独立判据：它不走普通外层 KMS verifier，只能使用固定 `C:\ProgramData\EnterprisePlatform\trust\license-roots.p7b` 的 exact DER PKCS#7 根/中间证书与离线 CRL bundle，按第 3.4.11 节复验 JCS digest、算法、EKU、subject、链与撤销；禁止 Windows 任意根、联网补链或临时根。普通外层成功、special 外层复验成功和内层 CMS 成功三者互不替代，任一失败均不得推进状态或写部分投影。

发布执行：普通包在一个 `READ COMMITTED` 事务内按 `sort_no` 升序调用 pure `validate` 与 `apply`；special execute 则先完成上文 license→package/order/item→module 的唯一锁序，再由 `apply` 从持久化 exact bytes 重跑全部权威守卫，绝不执行“先锁 package、后由 applier 取 license”的反序。两者都在同一事务把发布单置 `SUCCEEDED`、配置包置 `RELEASED`，写 Outbox 事件 `platform.config_release.released.v1`，最后写审计事件，次序按判定二。普通包回退按 `sort_no` 逆序调用 `revert`，以 `before_spec` 恢复，同样单事务且同样以审计收尾；特殊单项包在受理发布单时已被排除 ROLLBACK，绝不进入该循环。任一 applier 的 `requires_derived_store_rebuild` 为真时，本阶段只把该判定结果写入事件载荷，派生存储重建的传播段由阶段 13b 实现。

本阶段不交付自动测试编排、编辑锁、停机窗口排队与在线 DDL，这四项与十一态生命周期一并由阶段 13b 扩展。

#### 3.4.13 合同终止影响面平台（3b-2 批）

`ep-platform-impact` 的唯一公开契约冻结如下；字段使用 foundation 的中立 ID、模块码、事务与安全上下文，不引入业务 crate 类型。

```rust
pub struct ImpactSource {
    pub source_module: ModuleCode,
    pub source_doc_id: Uuid,
    pub source_doc_version: i64,
    pub source_event_id: Uuid,
    pub source_event_type: String,
    pub reason: String,
}
pub struct ImpactItemDraft {
    pub target_module: ModuleCode,
    pub target_doc_id: Uuid,
    pub target_doc_no: Option<String>,
    pub target_doc_line_no: Option<i32>,
    pub disposition_kind: ImpactDispositionKind,
    pub security_level: SecurityLevel,
    pub data_scope_tags: Vec<DataScopeTag>,
}
pub struct ManualImpactDecision {
    pub decision_code: String,
    pub decision_reason: String,
    pub decision_result_doc_id: Option<Uuid>,
}
pub enum ImpactDisposeOutcome {
    Completed { reason: String },
    AlreadySatisfied { reason: String },
    NeedsManualDecision { reason: String },
}
pub trait ImpactRule: Send + Sync {
    fn code(&self) -> &'static str;
    fn upstream_event_type(&self) -> &'static str;
    fn target_module(&self) -> ModuleCode;
    fn assess(&self, tx: &mut dyn Tx, ctx: &SecurityContext,
              source: &ImpactSource) -> Result<Vec<ImpactItemDraft>, AppError>;
    fn dispose(&self, tx: &mut dyn Tx, ctx: &SecurityContext,
               item: &ImpactDispositionItem,
               decision: Option<&ManualImpactDecision>)
               -> Result<ImpactDisposeOutcome, AppError>;
}
```

`ImpactItemDraft.target_doc_id: Uuid` 只承载真实目标；目标为空的 PENDING/`NO_APPLICABLE_TARGET` 目录行由 `ImpactAssessor` 按目录直接构造，不把空目标伪装成 UUID 哨兵，也不放宽该 DTO 为 `Option<Uuid>`。

`ImpactRegistry` 的可注册 code 全集只取 `docs/impact-catalog.md` 七码；目录外、重复 code、code 与 `target_module`/上游事件不符均在装配时失败。目录条数恒为 7，真实注册数按阶段退出点分别为 0、3、4、6、7。`cargo xtask configdoc --check-impact-catalog` 比对 code、顺序、属主阶段、目标模块、管理角色、允许的 `decision_code` 与结果 id 形状；`xtask archcheck` 同时以真实负样例证明业务规则不能依赖别模块 `ep-app-*`/`ep-domain-*`，各模块仓储仍由 `db-pg-one-schema-per-file` 拦截跨 schema SQL。

查询契约唯一名为 `ImpactAssessmentQuery`，供阶段 6 的 `GET /api/v1/clm/contracts/{id}/impact-assessment` 使用：

```rust
pub struct ImpactAssessmentView {
    pub assessment: ImpactAssessmentSummary,
    pub items: Vec<ImpactDispositionItemView>,
}
pub trait ImpactAssessmentQuery: Send + Sync {
    fn by_source(&self, tx: &mut dyn Tx, ctx: &SecurityContext,
                 source_module: ModuleCode, source_doc_id: Uuid,
                 source_doc_version: i64, source_event_type: &str)
                 -> Result<Option<ImpactAssessmentView>, AppError>;
}
```

视图按目录顺序、目标 id UUID bytes、目标行号排序；每个人工项同时返回该 code 的封闭允许集、`decision_result_doc_id` 必填/必空形状、`process_task_id` 与当前三决策字段。无批次返回 `Ok(None)`，无权与不存在由调用方统一映射为 `PLATFORM.AUTHZ.NOT_FOUND_OR_DENIED`。

唯一消费者名为 `platform.impact_assess`，只消费 `clm.contract.terminated.v1`。它先以 `(consumer,event_id)` 写 `platform_msg.inbox_consumptions`；同一事务锁定或幂等取得按来源唯一的批次，逐目录类别调用已注册规则的 `assess` 并建立真实项，未注册类别建立一个目标为空的 PENDING 占位项。批次与全部项、inbox 行、审计在同一事务提交；重复事件返回既有批次，不扩增任何项。已接线规则返回空集合时，该类别以目标三字段为空、`attempts=0`、无租约/错误/人工字段且 `outcome_reason='NO_APPLICABLE_TARGET'` 的 DONE 目录项表达，不以“没有行”逃过闭合判据；该行计入 `item_done`，使批次仍能以 `item_total=item_done` 唯一闭合。

`ImpactAssessor` 另按法人轮转领取到期 PENDING 项：批量 20，`FOR UPDATE SKIP LOCKED`，租约 60 秒、每 20 秒续租；同一项一次只由一个 worker 处置。占位项在其真实规则注册后先于普通项重跑 assess：无目标则在同一事务清除任何瞬时租约并写成上述目标为空的 DONE 终态，有目标则按目录唯一约束展开并删除原占位，重复扫描不增行。真实项每次使用一个 `READ COMMITTED` 事务，把同一 `&mut dyn Tx`、`SecurityContext` 与锁后项目传入规则；规则须在本事务内锁定并重读业务目标，验证同法人、来源关联与当前状态。

三态结果语义唯一如下：

- `Completed` 与 `AlreadySatisfied`：稳定 `reason` 清洗后非空，项目置 DONE、写 `outcome_reason`，批次计数同事务重算；`AlreadySatisfied` 不补写业务事实或第二条审计。
- `NeedsManualDecision`：不是失败，不增加 attempts、不退避、不进死信。平台同事务把 `disposition_kind` 置 `MANUAL_DECISION`、state 保持 PENDING，按目录固定的 `SALES_MANAGER|PROCURE_MANAGER|FINANCE_MANAGER|PROJECT_MANAGER` 创建或幂等复用待办并回填 `process_task_id`。
- 意外基础设施或可重试领域失败：失败计数加一，按 1 秒、5 秒、30 秒、2 分钟、10 分钟、30 分钟、1 小时、2 小时八档设置 `available_at`；首投加八次重试仍失败时 `attempts=9`、项目置 DEAD、批次置 FAILED。事务以来源事件 id 向 `platform_msg.dead_letters` `INSERT ... ON CONFLICT (source_event_id) DO NOTHING` 写一个批次级告警信封，并在新轮次发通知；通用死信 replay 对这类已消费事件返回 `PLATFORM.DEAD_LETTER.STATE_INVALID`，details 指向本批次 replay 端点。

人工项使用流程定义 `clm.contract_termination_disposition`。每个处置项各建一个实例，业务对象为该 item id，candidate role 只取目录映射，assignee 初始为空；节点为 `HUMAN_TASK` 并登记一条 SLA timer，超时天数取 `impact.manual_item.sla_days`（默认 5、范围 1..=30），流程实例最长 365 天。到期只写 `sla_breached_at`、通知与审计，不自动代选。流程实例进入 MANUAL_INTERVENTION 或自身结束均不直接改变项目状态；流程任务只是待办载体，项目推进只经下列人工决定命令。

人工决定端点锁定 item 与目标，先按目录验证码、非空理由、结果 id 的必填/必空形状，再调用 `ImpactRule::dispose(..., Some(&decision))` 复核具体对象。不得解析理由文本或只读取 process task outcome。合法 `Completed|AlreadySatisfied` 时先逐字持久化 `decision_code`、`decision_reason`、`decision_result_doc_id` 与决定人/时间，再置 DONE；`NeedsManualDecision`、错码、错形、异法人、异对象、状态错配均保持 PENDING，三决策字段全为空，不消耗重试预算。错误文案只返回已登记码；通用输入错误用 `PLATFORM.REQUEST.INVALID_PAYLOAD`，对象越权用 `PLATFORM.AUTHZ.OBJECT_FORBIDDEN`。

批次闭合只允许 `item_done=item_total`、全部项目 DONE 且 `item_dead=0` 一条路径；`NO_APPLICABLE_TARGET` 目录终态按一条 DONE 计入两侧计数。为避免平台直接写业务 schema，`ep-platform-impact` 另定义中立 `ImpactSourceCompletionPort::complete(tx,ctx,source,assessment_id,item_total)`，按 `(source_module,source_event_type)` 注册；阶段 6 的 `ep-app-clm` 为合同终止实现真实端口，在调用方同一事务锁定合同、确认仍为 TERMINATING、推进 TERMINATED、写审计并恰一次发布 `clm.contract.termination_completed.v1`。缺失或重复 completion port 时装配失败，平台不得直接 SQL 写 clm，也不得把批次先置 DONE 后异步补合同。完成端口成功后同事务把批次置 DONE；任一 PENDING 或 DEAD 均不调用。

replay 路径固定为 `POST /api/v1/platform/impact-assessments/{id}/actions/replay`：仅 FAILED 且至少一项 DEAD 可受理，要求 `platform.impact_assessment.replay`、记名、`X-Reauth-Token` 与幂等键；同事务把 DEAD 项置 PENDING、attempts 归零、可用时间置当前、清租约与最新错误，把批次置 RUNNING 并重算计数，不新建批次或项目。其他状态返回 `PLATFORM.IMPACT_ASSESSMENT.REPLAY_NOT_ALLOWED`。手工决定路径为 `POST /api/v1/platform/impact-disposition-items/{id}/actions/decide`，权限 `platform.impact_disposition.decide`，仅候选角色成员可用并要求 `row_version` 与幂等键。

---

### 3.5 API 契约

全部端点前缀 `/api/v1/platform`，门户侧为 `/api/v1/portal/...` 由 portal-gateway 转发。请求头集合、封套、分页、排序、过滤、幂等键、版本化一律按基线第 5 章，本节不重复，只给各端点的差异项。

权限项名称形如 `platform.<resource>.<action>`，判定由阶段 4 的 ep-platform-authz 承担；本阶段负责在每个端点上声明所需权限项并注册到权限项目录。
能力域码、动作类别与许可准入绑定按裁定 A-20 和第 3.4.11 节一起声明。外部路由注册处必须与路由同行一次性给出 `(CapabilityDomain, ActionClass, LicenseAdmissionBindingV1)` 三元组，不再为每个用例另写 `<USECASE_SCREAMING>_DOMAIN` 与 `<USECASE_SCREAMING>_ACTION` 两个常量，也不得在 handler 中补一个未登记的许可判断；`crates/platform/flow/src/capability.rs` 只保留前两类的类型别名与路由对照，许可 binding 的唯一类型 owner 仍是 `crates/platform/license/src/admission.rs`。`/api/v1/platform/` 下的全部路由能力域取 `CapabilityDomain::PlatformAdminLowcodeOps`，第 3.5.2 节三个 `/api/v1/portal/` 端点的能力域取 `CapabilityDomain::PortalSupplierWeb`。动作类别的取值规则为：只读查询取 `Read`；`process-tasks/{id}/actions/complete`、`config-packages/{id}/actions/approve` 与 `config-packages/{id}/actions/reject` 三个审批结论端点取 `Approve`；`POST /api/v1/platform/push-registrations`、`POST /api/v1/platform/config-packages` 与 `POST /api/v1/platform/disposals` 三个创建端点取 `Write`；其余 `actions/` 端点与分片 `PUT` 取 `Submit`；本阶段无导出路由，不出现 `Export`。许可 binding 默认按 ActionClass 的封闭映射登记为 Read/Export→`Fixed(ReadReportAuditBackupExport)`、Approve→`Fixed(BusinessApproval)`、Write/Submit→`Fixed(BusinessWrite)`；只有第 3.4.11 节列名的配置发布八类操作可登记 `ConfigRelease`，MCP inbound 才可登记 `McpInbound`，认证前置入口固定按该节时序登记身份安全处置。第 3.4.6 节 integration-gateway 的 `push.dispatch.v1` 是 `ep-integ` 出站 IPC operation 而非 HTTP route，不声明 CapabilityDomain/ActionClass，但必须在非 HTTP registry 登记 `Fixed(IntegrationOutbound)`。`xtask configdoc` 断言每个外部 `/api/v1`、`/portal`、`/mcp` 路由都能解析到恰一个三元组，`xtask archcheck` 断言非 HTTP registry 覆盖实际 operation，缺失、多余或重复均使构建失败。

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
| POST /api/v1/platform/disposals | 处置受理，请求 `{disposal_plan_id, scope, object_refs, approval_ref, second_approver_id, reauth_ref}`；本阶段至阶段 13 之间一律以 `PLATFORM.DISPOSAL.NOT_DELIVERED` 拒绝并开一条降级窗口，阶段 14 注入 `OpsDisposalService` 后放行 | 必填 | platform.disposal.execute |

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

`complete` 的守卫条件三项：任务处于 `PENDING` 或 `CLAIMED`；当前用户是受理人或候选角色成员，否则 `PLATFORM.PROCESS_TASK.NOT_ASSIGNED`；当前用户不是 `initiator_user_id`，否则使用全局唯一码 `PLATFORM.SOD.SELF_APPROVAL_FORBIDDEN`。第三项对应规格第 12.2 章“申请人不可自审”，判定输入由本阶段提供，判定实现由授权层执行。

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
| POST /api/v1/platform/config-packages/{id}/actions/submit-for-approval | `{note}`；以 `CONFIG_RELEASE` 解析阶段 4 共用链，包、审批证据、流程实例与首任务同事务从 Draft 到 PendingApproval；缺链/并列/空节点/自审均零写入失败关闭 | 必填 | lowcode.config_package.submit |
| POST /api/v1/platform/config-packages/{id}/actions/approve | `{task_id,note}`；仅完成标准流程任务，handler 不改包，包只能由同事务 `ConfigReleaseApprovalCallback` 迁到 Approved | 必填 | lowcode.config_package.approve |
| POST /api/v1/platform/config-packages/{id}/actions/reject | `{task_id,reason}`；仅完成标准流程任务，handler 不改包，包只能由同事务 callback 迁到 Rejected | 必填 | lowcode.config_package.approve |
| POST /api/v1/platform/config-packages/{id}/actions/sign | 签名，已签名重复调用返回既有签名 | 必填 | lowcode.config_package.sign |
| POST /api/v1/platform/config-release-orders | 建发布单或回退单 | 必填 | lowcode.config_release.submit |
| POST /api/v1/platform/config-release-orders/{id}/actions/execute | 执行，重复执行返回同一回执 | 必填 | lowcode.config_release.execute |
| GET /api/v1/platform/config-release-orders/{id} | 发布单详情含逐项落地结果 | 不适用 | lowcode.config_release.view |

`actions/run-autotest`、`config-edit-locks` 与停机窗口排队三组端点属阶段 13b，本阶段不实现。

#### 3.5.9 模块许可（3b 段）

本阶段不新增对外端点，`ModuleLicenseQuery` 是唯一取用入口。许可凭证与模块安装态在本阶段经迁移种子写入，运行期的许可导入与模块启停入口由阶段 13b 与阶段 14 承载，本阶段不预先占用路径。

#### 3.5.10 影响面处置（3b-2 批）

| 方法与路径 | 说明 | 幂等 | 权限项 | 主要错误码 |
|---|---|---|---|---|
| GET /api/v1/platform/impact-assessments/{id} | 批次、目录顺序的实项/占位项、计数、待办与允许决策形状 | 不适用 | platform.impact_assessment.read | PLATFORM.AUTHZ.NOT_FOUND_OR_DENIED |
| POST /api/v1/platform/impact-assessments/{id}/actions/replay | 仅 FAILED 且存在 DEAD 项；记名、重新认证；复位同批 DEAD 项而不新建批次/项目 | 必填 | platform.impact_assessment.replay | PLATFORM.IMPACT_ASSESSMENT.REPLAY_NOT_ALLOWED、PLATFORM.AUTHZ.REAUTH_REQUIRED |
| POST /api/v1/platform/impact-disposition-items/{id}/actions/decide | 请求 `{decision_code,decision_reason,decision_result_doc_id,row_version}`；只允许目录候选角色成员 | 必填 | platform.impact_disposition.decide | PLATFORM.REQUEST.INVALID_PAYLOAD、PLATFORM.CONCURRENCY.STALE_VERSION、PLATFORM.AUTHZ.OBJECT_FORBIDDEN |

合同按来源查询不另开平台 HTTP 路径；阶段 6 的合同端点只调用 `ImpactAssessmentQuery::by_source`。人工决定成功回放首次响应；同幂等键不同载荷按全局幂等规则拒绝。错误响应、审计与通知不得回显目标正文、未清洗理由或底层异常。

---

### 3.6 并发与事务边界

#### 3.6.1 事务清单

| 事务 | 隔离级别 | 内含写入 | 锁 | 预算 |
|---|---|---|---|---|
| 业务命令事务（core-server 用例） | READ COMMITTED | 业务表、取号行、Outbox、站内通知、幂等键、审计事件与段行 | BusinessWrite/Approval/Automation/Outbound 的第一业务 SQL 先取 `LICENSE_CURRENT_SHARED`，再 admission/聚合/序列/审计锁 | 5 秒；`statement_timeout` 10 秒，`lock_timeout` 3 秒 |
| Outbox 取件事务 | READ COMMITTED | `outbox_events` 的 status/locked 列 | 第一业务 SQL 取 transaction-level license shared、重验 admission，再 `FOR UPDATE SKIP LOCKED`；Restricted 不 claim | 1 秒 |
| Outbox 消费事务 | READ COMMITTED | 处理器副作用、`inbox_consumptions`、`outbox_events` 置 DONE | 新副作用短事务取 shared；外部调用另以专用连接持 session license shared+module shared，数据库事务在调用前结束 | 按处理器，上限 30 秒 |
| 锚定阶段 A | READ COMMITTED | `audit_anchors` 插入、段行 last_anchor_seq | 段行排他锁 | 1 秒 |
| 锚定阶段 B/C | 无事务或短事务 | `audit_anchors` 更新 | 条件更新 | KMS 与磁盘超时各 10 秒 |
| 链验证 | REPEATABLE READ | 只读，`audit_verifications` 更新在独立短事务 | 无 | 单段上限 5 分钟 |
| 流程单步事务 | READ COMMITTED | 实例行、`process_steps`、任务或定时器、Outbox、审计 | 普通 BusinessApproval 先取 shared；special approve 取 exclusive；typed reject NONE；随后实例 `FOR UPDATE` | 5 秒 |
| 定时器触发事务 | READ COMMITTED | `process_timers` 状态、实例 `next_wake_at` | AutomationStart 第一业务 SQL 取 shared，再 `FOR UPDATE SKIP LOCKED` 与 admission | 1 秒 |
| 补偿单条事务 | READ COMMITTED | `process_compensations`、实例状态、审计 | 实例行 `FOR UPDATE` | 5 秒 |
| 附件事务 A | READ COMMITTED | `attachment_versions` PENDING 行 | 无 | 1 秒 |
| 附件事务 B | READ COMMITTED | 版本置 AVAILABLE、对象版本号、Outbox、审计 | 对象行乐观锁 | 2 秒 |
| 保留期清理事务 | READ COMMITTED | 按批 DELETE，单批不超过 1000 行 | 无 | 30 秒 |
| 配置包提交审批/任务结论回调 | READ COMMITTED | 包审批证据、流程实例/步骤/任务、幂等 finish、同事务通知命令、审计终结批；approve/reject handler 不直接写包 | submit/approve 固定先取 `LICENSE_CURRENT_EXCLUSIVE`，再 `try_begin` 并按 canonical tuple 锁包/项/流程行；typed reject 固定 `NONE` 且不得查包后改判；普通包经过共享配置入口也遵守相同选择 | 5 秒 |
| 配置发布段二事务（3b 段） | READ COMMITTED | 按 `sort_no` 升序的 applier `apply`、发布单与配置包状态、Outbox、审计 | execute 共享入口对 ordinary/special 都先取 license exclusive；随后 `(package,order,sort_no,item_id)` rows→ModuleCode wire-order locks，ordinary 再沿用发布互斥，whole transaction 总 deadline 30 秒 | 30 秒 |

只读分析池的 `statement_timeout` 60 秒、`work_mem` 64 MB、`temp_file_limit` 2 GB，job-worker 池 `statement_timeout` 300 秒，ops 池 5 秒，取值全部按基线第 10.3 节，本阶段不改。

#### 3.6.2 锁顺序与死锁防范

本阶段引入三类会被多个事务同时争用的行：编号序列行、审计段行、流程实例行。凡 binding 要求 shared/exclusive，license advisory 必须先于下列全部行锁；其后的统一顺序为：业务聚合行，编号序列行，流程实例行，审计段行。审计段行排在最后，与判定二一致。跨自然日的多段写入按 `event_day` 升序加锁。该顺序写入 `ep-adapter-db-pg` 的工作单元文档并由代码评审逐条核对；违反顺序的代码在集成测试的死锁注入用例中会被检出。

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
| EP__PLATFORM__AUDIT__EVIDENCE_DIR | path | C:\EP\audit-evidence | job-worker |
| EP__PLATFORM__AUDIT__VERIFY_MAX_DAYS | u16 | 92 | core-server, job-worker |
| EP__PLATFORM__AUDIT__QUERY_MAX_DAYS | u16 | 366 | core-server |
| EP__PLATFORM__FILE__ROOT_DIR | path | C:\EP\files\published | core-server, job-worker |
| EP__PLATFORM__FILE__STAGING_DIR | path | C:\EP\files\staging | core-server |
| EP__PLATFORM__FILE__MAX_OBJECT_BYTES | u64 | 5368709120 | core-server |
| EP__PLATFORM__FILE__PART_BYTES | u32 | 8388608 | core-server |
| EP__PLATFORM__FILE__SESSION_TTL_HOURS | u16 | 24 | core-server, job-worker |
| EP__PLATFORM__FILE__MAX_CONCURRENT_UPLOADS_PER_USER | u8 | 3 | core-server |
| EP__PLATFORM__FILE__MAX_CONCURRENT_UPLOADS_GLOBAL | u8 | 6 | core-server |
| EP__PLATFORM__FILE__UPLOAD_BANDWIDTH_BYTES_PER_SEC | u64 | 52428800 | core-server |
| EP__PLATFORM__FILE__DOWNLOAD_BANDWIDTH_BYTES_PER_SEC | u64 | 52428800 | core-server |
| EP__PLATFORM__FILE__FREE_SPACE_MIN_BYTES | u64 | 107374182400 | core-server |
| EP__PLATFORM__FILE__VIRUS_SCAN__MODE | enum | 无默认；取值仅 `NONE`、`CUSTOMER_ICAP`，部署必须显式填写 | core-server, integration-gateway, ops-agent |
| EP__PLATFORM__FILE__VIRUS_SCAN__ICAP_URL | url | 无默认；`CUSTOMER_ICAP` 时必填，scheme 必须为 `icap` 且 host 只能是 `127.0.0.1` 或 `[::1]`；`NONE` 时必须为空 | integration-gateway, ops-agent |
| EP__PLATFORM__FILE__SCAN__TIMEOUT_SECONDS | u32 | 120 | core-server |
| EP__PLATFORM__FILE__SCAN__MAX_ARCHIVE_RATIO | u32 | 200 | core-server |
| EP__PLATFORM__FILE__SCAN__MAX_ARCHIVE_DEPTH | u8 | 4 | core-server |
| EP__PLATFORM__FILE__QUARANTINE_RETENTION_DAYS | u16 | 90 | job-worker |
| EP__PLATFORM__NOTIFY__RETENTION_DAYS | u16 | 180 | job-worker |
| EP__PLATFORM__NOTIFY__UNREAD_CAP_PER_USER | u32 | 2000 | core-server |
| EP__PLATFORM__NOTIFY__SYNC_FANOUT_MAX | u16 | 200 | core-server |
| EP__PLATFORM__NOTIFY__PUSH_ENABLED | bool | false | core-server, job-worker |
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
| EP__IMPACT__MANUAL_ITEM__SLA_DAYS | u16 | 5，范围 1..=30 | core-server, job-worker |
| EP__PLATFORM__SEARCH__ROOT_DIR | path | C:\EP\search | core-server, job-worker |
| EP__PLATFORM__RETRY__SERIALIZATION_MAX_ATTEMPTS | u8 | 3 | 全部持库进程 |
| EP__PLATFORM__RETRY__SERIALIZATION_BACKOFF_MS | Vec\<u32\> | [50,150,450] | 全部持库进程 |
| EP__PLATFORM__RETRY__CIRCUIT_FAILURE_THRESHOLD | u8 | 5 | job-worker, integration-gateway |
| EP__PLATFORM__RETRY__CIRCUIT_OPEN_SECONDS | u32 | 30 | job-worker, integration-gateway |
| EP__PLATFORM__RETRY__CIRCUIT_HALF_OPEN_PROBES | u8 | 1 | job-worker, integration-gateway |

运行期可变的业务参数不进配置文件，按基线第 7.1 节：提醒规则的触发对象、触发日期字段、提前量、重复策略与接收人解析方式，通知模板的标题与正文，流程定义，全部存事务数据库并经配置发布通道签名发布。发布通道按裁定 A-27 由本阶段 3b 段交付，见第 3.4.12 节；流程定义与提醒规则、通知模板的落地分别经 `FlowDefinitionApplier` 与 `NotifyRuleApplier`。

启动自检增量：按裁定 C-25，自检项一律以注册名标识，不用序号，注册表为阶段 1 的 `SelfCheckRegistry`，报告按注册顺序输出且基线项在前。自检项分阻断与降级两级，`severity` 取值域固定为 `Blocking` 与 `Degrading`，登记为本阶段新增决定第九项的一部分；阻断级失败以退出码 78 退出，降级级失败不阻止启动，改为登记一条 `platform_ops.degradation_windows` 并经阶段 2 的 `DegradationLedger` 出降级信号与告警。`--check` 模式对两级一律严格，任一项 FAILED 或 DEGRADED 均非零退出，闸门落在部署与升级前置，不落在进程启动。本阶段在基线第 7.3 节的命名项之外追加六项：`audit-evidence-store-writable`，审计证据存储目录可写——按裁定 F-08 第 4.3 节第 2 条，可写一项以实建探针文件再删判定，不得以「能否建子目录」代替「能否建文件」，在本平台两者是不同的权限位，以建子目录代判会假阳性——且其 NTFS ACL 不对本进程之外的账户授予覆盖与删除、对 archive-writer 的服务虚拟账户存在显式 Deny `DELETE` 与 `FILE_WRITE_DATA` 两条 ACE，剩余空间不低于阈值，阻断级；`audit-signing-key-usable`，签名私钥可解引用且可完成一次自签自验，阻断级；`attachment-store-ready`，附件存储根目录、staging 目录与检索索引根目录存在、三者的 NTFS ACL 已断继承、且除本进程的服务虚拟账户与 SYSTEM 与 Administrators 外不存在其他授权 ACE（原「权限位正确」随权限位换 NTFS ACL 改写，判据由三位八进制相等降为 ACL 集合判否，这是判据锐利度下降不是防护下降）。**判据只取本进程可自行读出的 ACL 事实，不引用部署记录**——`platform_ops.deployment_records` 由阶段 14 的迁移建立，本阶段运行时该表不存在，引用它会使这条阻断级自检在本阶段恒假、剩余空间不低于 `FREE_SPACE_MIN_BYTES`，阻断级；`event-catalog-consistent`，事件目录中登记的事件类型与代码中注册的处理器无缺漏无多余，降级级，检出不一致时停止派发未登记的事件类型并持续告警，其余投递照常；`impact-registry-consistent`，按当前已安装且启用模块核对目录内应存在的真实规则，并核对每个启用上游 `(source_module,source_event_type)` 恰有一个真实 `ImpactSourceCompletionPort`，目录外/重复/缺失/Noop 任一成立即阻断级。该项在进程启动与模块启用事务提交前都执行；失败时启动退出 78 或模块启用整笔回滚并返回 `PLATFORM.IMPACT_COMPLETION_PORT.NOT_REGISTERED`，现有 RUNNING 批次保持不变且不得被伪闭合；`license-admission-registry-consistent`，分别比较 `apps/core-server/src/wiring/` 与 `apps/job-worker/src/wiring/` 的许可 binding 集合和各自实际 route/job/event/approval-owner/outbound-IPC 注册集合，要求 exact equal，缺项、多项、重复、错 binding 或同一 operation 多 effect 任一成立即阻断级并以退出码 78 结束。该项只读静态描述与装配元数据，不查询 `license_grants`、模块状态或任何业务行；`xtask configdoc` 与 `xtask archcheck` 复用同一比较器并在构建时对相同差异失败。前三项、impact 项与许可准入注册项判读的是目录、NTFS ACL、剩余空间、密钥与静态装配注册表，不判读业务内容，故留在阻断级；前三项属 3b-1 批，impact 与许可准入注册项属 3b-2，impact 须早于阶段 6。基线项 `license-and-modules-consistent` 由本阶段整项删除而不是换成实现，见第 3.4.11 节与第 3.12.1 节偏离五。第 3.11.1 节风险九的保留期大小关系是两个配置键的比较，折叠进 `config-parsed` 的配置校验，不另设自检项。

---

### 3.8 测试计划

#### 3.8.1 单元测试覆盖的分支

编号：位数扩展的临界（999999 到 1000000）、类型码校验的四个长度边界、档案类与单据类的 `period_key` 取值、人工指定的拒绝路径、格式化补零、解析回读的往返一致。

审计：段首条的全零前序哈希、多条同事务事件的链式串接、跨自然日分组与升序加锁的顺序生成、JCS 规范化对键序与数值表示的稳定性（含 `1.10` 与 `1.1` 在字符串承载下可区分的断言）、`seq` 空洞不判为断裂、锚定触发条件的两个分支（时间到与条数到）及“无新事件即不创建”、A/B 之间轮换当前签名键仍只使用 anchor 已冻结 key_ref、锚点八档 `available_at` 与第九次失败、FAILED 按签名有无恢复到两个正确状态、验证器的五类失败定位。

Outbox：信封必填项的逐字段缺失、`security_level` 与 `data_scope_tags` 缺失的拒绝、八段退避序列的逐次取值、首投加八次重试的第 9 次失败转死信、`DISPATCHING` 租约过期回收、幂等键三态判定的九种组合（三种 `state` 乘三种 `request_hash` 关系）。

文件：上传会话状态机的全部合法迁移与全部非法迁移、分片重传的哈希一致与不一致、总哈希不符的拒绝、三个内置检查器各自的 PASS 与 REJECT 分支、归档炸弹的比例与深度两个上限、三段式落盘在四个崩溃点上的收敛判定。另逐字节覆盖 EPA1 的 24-byte header、1 MiB 边界前后、1/2/5120 块、末块整除与不整除、record offset 公式、EPC1 `plaintext+51`、序号缺/重/乱、ref 漂移、尾随/截断，以及一次 resolve 后在中途轮换仍全部块使用同一 canonical ref；`CurrentForWrite` 不能读历史，`ExactRef` 不能选 current 代替所指 key。

通知：模板变量白名单外的变量拒绝渲染、无权字段的替代文案、`dedupe_key` 的去重、未读上限触发时新通知仍写入、推送载荷在默认配置下不含业务字段。

流程：实例状态机的全部合法迁移与全部非法迁移、步骤幂等键的构造、守卫条件求值器的算子矩阵与步数上限、补偿逆序的顺序断言、三项运行约束各自的超限分支、版本迁移的可迁与不可迁判定；审批命令快照覆盖 PENDING 的三条合法终态边、三个终态的全部非法出边、状态形状、密文字段逐列不可变、摘要长度与变量白名单拒绝。

影响面：目录外/重复 code 拒绝、七条目录与 0/3/4/6/7 注册数分离、占位展开、三态 outcome、稳定原因码清洗、八档重试与第九次失败 DEAD、租约回收、批次唯一闭合判据、`ManualImpactDecision` 三字段形状、四类管理角色映射、人工项不增加 attempts、SLA 到点只提醒，以及 completion port 缺失/重复时失败关闭。

错误：五类分类到 HTTP 与 `retryable` 的映射矩阵、`incident_no` 的正常与退化两条生成路径、不可见记录统一返回 404 的四个端点。

领域属性测试（proptest），本阶段登记四组不变量。其一，对任意事件序列，链追加后按 `seq` 升序重算哈希恒等于存储值。其二，对任意并发取号序列，产出的编号集合无重复且无空号。其三，对任意投递与崩溃序列，每个事件的副作用恰好发生一次。其四，对任意步骤与补偿序列，补偿执行顺序恒为已完成步骤的严格逆序。这四组是规格第 17.2 章“领域属性测试”在本阶段的落点；基线第 8.1 节要求的借贷平衡、库存守恒、核销守恒、移动加权平均、价差拆分五组属业务模块阶段，本阶段不承担。

#### 3.8.2 集成测试场景清单

全部用真实 PostgreSQL 16，每用例独占一库，库名 `ep_test_<nanoid>`，用例结束即删库。合成聚合建在测试库内的临时 schema `test_synth`，由测试自行创建，不进 `db/migrations`，因此不违反“任何阶段不得新增 schema”。

行级安全，独立测试目标 `tests/rls_matrix` 的本阶段部分：对本阶段 27 张带 `legal_entity_id` 的表逐表调用阶段 1 提供的八个断言函数 `assert_read`、`assert_write`、`assert_update`、`assert_delete`、`assert_aggregate`、`assert_sort`、`assert_report_projection`、`assert_error_leak`，另覆盖会话变量缺失时的默认拒绝、会话归还后变量已清空、跨法人查询按法人逐轮而非 `OR` 展开三项；对六张部署级表断言其不建策略且可见性不随 `app.legal_entity_id` 变化。按裁定 C-05，断言函数骨架由阶段 1 提供、数据库侧策略断言与复制角色入口借用由阶段 2 提供、33 组完整矩阵与发布门禁项 `RG-RLS-MATRIX-GREEN` 由阶段 4 提供，本阶段只调用，不实现同名函数，也不承担该门禁项的判定。

Outbox 可靠投递，对应规格第 7.3 章必含项：至少一次投递、重复投递去重（同一事件强制投递 5 次）、崩溃恢复后不丢不重（在取件后投递前、投递后写 `inbox_consumptions` 前、写后置 `DONE` 前三个点终止 job-worker）。

影响面平台使用 testkit 的三个真实合成规则与一个真实合成 completion port，不注入 Noop：同一来源事件投递五次只建一个批次；七类目录中三类展开实项、四类为 PENDING 占位；一项 `Completed`、一项 `AlreadySatisfied`、一项从自动分支锁后漂移为 `NeedsManualDecision`，逐项断言同事务、锁后复核、HUMAN_TASK 幂等、SLA timer 与计数。四类人工目录形状逐码覆盖空理由、错码、结果 id 缺失/多余/异对象与状态错配，失败均保持 PENDING 且三字段为空。注入可重试失败时八个 `available_at` 档位全部可达，第九次失败置 DEAD/批次 FAILED；记名 replay 复位原行而不增行。

completion port 契约单独做正反例：全部项 DONE 前调用次数为零，最后一项闭合时业务合成对象推进与批次 DONE 在同一事务且完成事件恰一次；缺失、重复或 Noop 端口使 `impact-registry-consistent` 失败，进程退出 78 或模块启用事务回滚，并返回 `PLATFORM.IMPACT_COMPLETION_PORT.NOT_REGISTERED`。再构造直接依赖别模块 app/domain 的规则与仓储跨 schema SQL 两个负样例，分别由架构依赖门和 `db-pg-one-schema-per-file` 判红。

审计链，对应规格第 17.2 章“审计链与不可变存储测试”：20 并发写入 5000 条事件后链验证全通过；跨自然日边界的段切换；分别在阶段 A 提交后、B 外部签名后、B 提交后、C 文件写后、C 数据库提交后终止进程并跨重启恢复，原 anchor id 不变且最终只一份证据；两个 worker 并发及陈旧 row_version 只能一方推进，阶段 B/C 重放均不得覆盖终态；KMS 或证据存储连续九次失败后精确进入 FAILED/死信，再经记名 replay 按 signature 有无恢复到 PENDING_SIGN/SIGNED 并成功；成功时 anchor EVIDENCED 与 segment 的两个成功水位同事务推进，失败或回滚时二者均不推进；所有 segment/anchor 条件更新均通过 `assert_row_version_bump()`；证据文件被外部改写后验证报告定位到该锚定；证据路径的覆盖与删除尝试被应用层拒绝并写审计；`audit_events` 上的 `UPDATE` 与 `DELETE` 被 `assert_append_only` 触发器、数据库权限与 CI 的 SQL 静态检查三重拒绝；`outbox_events` 与 `dead_letters` 上白名单之外的列更新被 `assert_immutable_columns` 触发器拒绝，白名单之内的投递控制列与处置列更新正常通过。

流程引擎认证套件的阶段必过项，对应规格第 17.2 章：崩溃恢复（在步骤执行前、执行中、提交后、补偿过程中随机终止 job-worker 各不少于 20 次）；重复投递不少于 3 次时业务效果、外发事件与审计记录只产生一次；定时器幂等与可重放（进程重启、模拟升级重启、从备份恢复三种重放场景下不漏触发不重复触发）；流程定义版本升级（运行中实例继续用旧版本，新实例用新版本，显式迁移与回退各一次）；补偿正确性（逆序、幂等重试、部分失败后进人工任务队列并告警，用合成聚合与合成不变量验证）。

审批命令快照另跑真实 KMS 与 PostgreSQL 集成场景：实例与快照同事务创建，一实例第二张快照由唯一键拒绝；数据库、审计、Outbox、task payload 与 `process_instances.variables` 全面扫描均不存在命令明文或同名 `command` 列；换法人读取/更新均被 RLS 拒绝；篡改密文、key ref、两个摘要或 owner/scenario/action/schema_version 任一列被不可变触发器拒绝；合法批准只解密一次，在同一事务创建合成业务对象并把快照转 CONSUMED、写入与真实对象一致的 type/id/doc_no，再由 approval_ref 经实例与快照反查该对象。PENDING/REJECTED/EXPIRED 任一结果字段非空、CONSUMED 缺 type/id、转终态后改写结果、业务写成功但快照结果更新失败四类反例均整体回滚；失败、拒绝、过期三支零业务效果。第 39 号登记行逐值比对并执行 `db/checks/11`，断言无盲索引列。

附件：5 GB 文件的分片上传与断点续传（中断在 30%、60%、95% 三点）；并发上限触发；总哈希不符拒绝；检查未通过进隔离；三段式落盘在四个崩溃点的收敛；同一内容两次上传产生两个独立物理副本（规格第 6.5 章明确不做去重）；按法人密钥域与密级子域产生不同密文。轮换竞态必须证明上传开始只 resolve 一次、事务 A 后轮换不改变任何 EPA1 record 的 data-key ref，进程崩溃后以 `ExactRef(dek_ref)` 续传且旧 RETIRING/RETIRED key 可读，伪 ref、跨法人/domain/purpose/scope 与 REVOKED/CORRUPT 失败关闭。下载分别覆盖无 Range、三种合法 single-range、越界 end、超长 suffix，以及 whitespace/leading-zero/overflow/逗号多段/空项/start 越界/suffix=0；断言只解密相交块、206/416 与全部 headers 精确，任何块失败零部分正文。

附件制品身份：系统目录中可查得 `ux_attachment_versions_le_id` 与 `ux_attachment_versions_artifact_identity` 的精确列序；一条 AVAILABLE 版本可作为后续制品父行，改为 SUPERSEDED 后引用仍成立；PENDING、QUARANTINED、FAILED 三态以及错法人、错哈希或错大小的直接 SQL 子行均被真实复合外键拒绝；尝试把已被制品引用的父版本从 AVAILABLE/SUPERSEDED 改入不可发布状态同样被外键拒绝。

通知：站内通知在业务事务回滚时一并回滚；推送出口不可用时站内通知照常送达；接收人对关联单据无权限时正文不含无权字段；接收人已停用时写入 `RECIPIENT_INACTIVE` 并创建分配给同法人 `SECURITY_ADMIN` 的人工任务，原待办保持阻塞。安全管理员只能改派给当前有效且具备原节点审批能力的用户，改派前后主体、原因与时间写审计；申请人不可借改派成为本人事项审批人。U-B-16 据此关闭。

死信：转死信、重投成功、重投再失败、丢弃需双人审批、丢弃时申请人不可自审、按法人与会计期间的可枚举统计。

混沌与故障注入，五类：依赖服务超时（KMS 超时，以及 `CUSTOMER_ICAP` 模式下同机 ICAP 扫描器超时；后者必须留下 `ERROR` 结果、隔离附件、打开 `VIRUS_SCANNER_NOT_AVAILABLE` 窗口并在恢复后关闭，绝不回退 `NONE`）、连接池与内存资源耗尽（把读写池打满并验证 job-worker 池不被挤占）、消息积压（灌入 50 万条 Outbox 条目并验证取件不退化为顺序扫描）、磁盘写满（附件写入与证据写出各触发一次并验证按 `INFRASTRUCTURE` 返回且不产生半截元数据）、进程崩溃后重启恢复（core-server 与 job-worker 各强制终止 20 次）。预期行为一律为按规格第 15.1 章返回可重试或明确失败、不产生数据不一致、故障移除后 5 分钟内自愈。

契约测试：流程实例状态的 `same_transaction` 与 `outbox_eventual` 两条路径，各自验证业务状态、审计与 Outbox 仍在同一事务内提交。
端口缺位的降级窗口：`DisposalPort`、`RuleEvaluator` 与 `WasmComputePort` 三个端口在两个 wiring 目录下无任何注入行时，一次物理删除请求以 `PLATFORM.DISPOSAL.NOT_DELIVERED` 被拒且 HTTP 状态为 409、不可重试，一次命中规则求值与一次命中受限 WASM 计算的流程守卫各按可重试错误或直接拒绝返回；三种情形各产生一条 `kind` 取 `PORT_NOT_IMPLEMENTED`、`subject` 分别取三个端口完整类型名的降级窗口，三条窗口在同一 `kind` 下同时打开且互不覆盖，`ep_degradation_windows_open` 计数为三，健康端点逐项呈现该能力未交付。

许可与配置发布：先证明零 current 合法并映射 `RESTRICTED/NO_CURRENT_GRANT`，首张有效 grant 接受后才恰一 current；永久/订阅分别覆盖未来生效、60 天边界、到期后第 1/30/31 日、撤销立即受限、系统时钟倒拨，并断言一次 `license_evaluation()` 返回的 status/reason/trusted_now 来自同一 current 快照、有效态 reason 为空而 Restricted 恰一原因。`TrustedClockV1` 另以可控 wall/monotonic clock 验证启动 anchor、同进程不降、普通 query 零写入、readiness 与 import/autotest/submit/approve/sign/order/execute（reject 除外）都先取固定 advisory xact lock 后 CAS 推进，以及 job-worker target cadence 不超过 240 秒，以 `license-trusted-time:v1:<deployment>:<slot_utc>` 按 `floor(unix/240)*240` 的同 slot 至多一条；覆盖 checkpoint 与续期/撤销竞态、缺 checkpoint/偏差超 300 秒不可抑制告警、崩溃跨重启小于 300 秒诚实窗口和错误前跳只能可信备份恢复。两连接分别覆盖并发首发、同一前驱并发续期及续期/撤销竞态，逐次证明 whole special transaction 在 `BEGIN/SET LOCAL` 后第一条业务 SQL 取得固定 advisory xact lock、再按 canonical tuple/module wire-order 取锁并重读后恰一候选提交，输家只返回 SPECIAL_ITEM_SHAPE_INVALID；applier 重取同锁幂等，reject 独立证明零 license lock。新 grant 另断言 `last_trusted_at=max(pre_import_trusted_now,candidate.issued_at)` 且候选不参与 pre-import 值。计划 CAB bundle 轮换正例证明签名部署清单同步更新，exact set 恰为全部 RELEASED grant/revoke/module special item，逐项留旧接受摘要、新验证摘要、对象 id、outer/inner/总结果且零回填，并与 current license/module 投影交叉后重新开门；current grant/current revocation 失败进入 `RESTRICTED/SIGNATURE_INVALID`，当前模块失败只关该模块运行门，历史 CRL REVOKED 被隔离并排除 purchased/rollback/正向证明但不倒推有效 current，其他历史坏签名/source/digest 只保持变更门与共同 release gate 关闭。另验证 current inner 和/或 source outer 唯一 CRL 失败时可由 inner+outer 都 ACTIVE 的直接后继 grant 恢复，而其他损坏不可借道；未更新清单的磁盘漂移仍使 current 失败关闭。内外签名篡改、wrong deployment、错 scope、数组乱序/重复、unknown field、DEV root、链/撤销失败均零状态变更。special 正例还要证明 import 到 actions/sign 的发行方 outer signature/signer/signed-at exact 不变、部署 KMS sign 调用为零，inner 只读固定 license-roots.p7b；普通包 actions/sign 仍调用部署 KMS。模块五条合法动作与全部非法边逐条验证，动作 DTO 无 `legal_entity_id`，INSTALL/ENABLE/UPGRADE/ROLLBACK_VERSION 分别证明 current 有效 grant 同时覆盖目标与依赖闭包；INSTALL 只到 disabled 且不要求依赖已启用，ENABLE 则要求每个依赖已为 INSTALLED_ENABLED，enabled 不可升级、降版本只可新审批 `ROLLBACK_VERSION`、不存在卸载；逐法人 scope 只影响运行时授权。DISABLE 在 Restricted 中仍可完成全链，但旧包/异包 artifact identity 不等时零写入拒绝；当前 inner 和/或 current source outer signer 被 CRL REVOKED 时，正例必须是 action=DISABLE、携带当前旧 SignedBusinessArtifact exact bytes、旧 inner/source outer/接受/source/payload/digest/signature/projection 自洽、未撤销层为 ACTIVE 或 RETIRED-nonrevoked，且只有新 special outer 由 ACTIVE signer 签发；任一重签 inner、其他动作、坏 digest/signature/source/chain 或不能唯一分类都拒绝。五个动作逐条验证 installed/state-changed/enabled/disabled 的唯一投影规则与升级/降版不抹历史时点。通过 DISABLE 后 UI/写入/定时器/新 Outbox 停止，既有在途排空，查询/报表/审计/备份/导出与数据 checksum 保持，再启用重验并恢复。许可 Restricted 时进程仍启动；对五个 `LicenseRestrictionReason` 逐项证明 LICENSE_GRANT 全恢复链和 MODULE_PACKAGE/DISABLE 全恢复链放行，其他常规写/审批统一 `PLATFORM.LICENSE.RESTRICTED`，不开借名降级窗口。另以全局 ACTIVE 的 LIST grant 对范围外法人发普通写，断言 `LicenseAdmissionGate` 返回同一码且零写入，而全局 status/reason 不变、查询/报表/导出继续可用；把已有目标法人的请求伪装成 `legal_entity_id=None` 必须失败关闭。三项用量分别用 `legal_entities.is_active=true`、非 SYSTEM 的 ACTIVE/LOCKED/SUSPENDED 账号、PENDING/ACTIVE 设备证明精确计数，超限只告警，模块与 AI/MCP scope 不足硬拒绝。配置包六态和 `CONFIG_RELEASE` 审批证据继续按原正反矩阵验证；另外对两类精确 after_spec 形状逐项制造非 IMPORTED、非单项、带 before、外层法人覆盖、非 ADD、把 module item 误解析成 generic envelope、内层篡改及通用 ROLLBACK，断言分别以 F-56 稳定码整包零写入拒绝。`ConfigItemApplier::validate` 另以 fake DB/KMS/file/current port 断言调用次数全零，只有 locked `apply` 从 persisted exact bytes 重验才可提交。普通包仍按逆序 `revert`，`FlowDefinitionApplier` 与 `NotifyRuleApplier` 各跑发布/回退；`LicenseGrantApplier` 与 `ModulePackageApplier` 只跑显式新动作继承，永不调用通用 revert。

F-56 文件与持久化负矩阵还须分别覆盖 ADD/MODIFY 对 after_spec、REMOVE 对 before_spec 的 JCS/lowerhex 重算及对 null 求摘要拒绝；special 的 item.jcs 只等于 after_spec exact JCS，kind/code/change/sort/scope/before-null/hash 任一篡改都由 signed canonical manifest 复验拒绝。ZIP64、非 STORE、缺/多/重名 entry、路径/属性/时间/CRC/offset/尾随数据、非 canonical manifest/item、outer detached content/SignerInfo/attributes 不符均在落库前拒绝。RELEASE 事务对 ordinary/special 未发布/已发布分别证明摘要 null/null/32，故障点证明摘要、投影、package/order 全部同提交或全回滚，第二次异值写、清空与 direct SQL 破坏均由 093300 的 deferred 约束在 COMMIT 拒绝；grant 摘要必须等于 source item，revocation/module 不存在复制摘要列。

治理与首装矩阵必须证明：090100 fresh 库恰有 0601..0615 的 15 行 NOT_INSTALLED catalog，任一 id/code/name/null-shape 漂移或第 16 行使门禁失败；首张 grant 的治理法人已存在且 active、LIST 必含、首张 RELEASE 后同 deployment 不可换值，治理法人停用/删除失败；首张与后继 special 的 approval 法人分别从候选/首次 RELEASED history 派生，伪 request header/UI/环境变量、损坏或多份首次 history 全拒。`ep-migrate apply` 三参数只在 fresh production/常驻 readiness 前可用且只接受 canonical lowercase deployment UUID 子目录 `C:\ProgramData\EnterprisePlatform\evidence\stage14\initial-governance\<lowercase-deployment-id>\` 固定根、固定两输入名/receipt 名、exact DACL/safe-handle/CREATE_NEW/flush/readback；license archive 只验证不代发布。fresh 前置逐表断言零法人、SYSTEM 单例账号、零 credential/device/authz/key-domain/license/config 与 exact SYSTEM/deployment/migration seed。bootstrap 两 CMS、签入 `key_domain_id`、两人员/设备/凭据、两角色 exact permission/default chain、关闭 echo 的双密码、exact `/kek/1` locator 的 PROVISIONING 数据库提交、无 KMS/data-key 调用、审计/非秘密 receipt/PE digest/hash-chain 交叉及全部事务故障点均有正反例；receipt sidecar 与 ep-migrate KMS 调用次数必须为零。随后对 `KmsKeyMaterialProvisioner` 三方法注入成功、外部 material 成功后数据库提交前崩溃、16 tuple 任一 readback 失败、purpose/scope 缺/多/重复、orphan 隔离与重启 resume，证明只用同 key_domain_id、PROVISIONING 失败关闭 readiness，四 purpose×四 scope exact 16 条 wrapped DEK 与 domain ACTIVE 同事务提交且有 activation audit；A-04 每次只轮换一个 purpose 的四 scope。同 input digest 且仅漏 receipt 时只补 receipt；已有 receipt、不同 digest、任一非 fresh 前置或第二次执行退出 78 且零业务写，命令清单不增加子命令。

093300 history/并发矩阵另覆盖：同 `package_id` 冒用不同 inner、同 `(module,code,semver)` 冒用不同 package/inner 在 release 锁内及 COMMIT 均拒，重复 exact inner 用于 ENABLE/DISABLE/ROLLBACK_VERSION 可过。业务/worker 取得目标 shared 后递归 effective 检查依赖；ENABLE 按全局 wire 顺序取目标 exclusive+依赖 shared；DISABLE 按 wire 顺序取全部 15 把 exclusive、总 30 秒且只改目标。制造 package/order/item→license、module→package 或 ModuleCode 逆序并发，必须由 recording Tx/静态锁序检查在执行反序 SQL 前失败；合法并发不得死锁。依赖停用竞态、15 锁中任一超时、PENDING/DISPATCHING 重试和崩溃断连，必须无部分状态/Outbox/审计，raw `module_state` 永不参与业务放行。CAB 轮换证据逐 item 必须分别含 outer 结论、inner 结论和总结果；current grant/revocation 的任一 inner/source-outer 失败才改全局状态，module 的任一 inner/source-outer 失败只关自身。current 的 inner 和/或 outer CRL 撤销恢复候选必须 inner+outer ACTIVE 且 deployment/governance/direct-successor 全同，其他损坏不得借路。

签名/目录负矩阵还须覆盖 `Sha256Digest` 只收 64 lowerhex→DB raw32、SPKI token 与 display DN 不可互换；对 leaf+0/1/多 intermediate 构造整条 non-anchor 链，逐项断言状态优先级 REVOKED>ACTIVE>RETIRED>UNTRUSTED、每张证书 signed_time/current validity、首次 ACTIVE 接受证据、anchor signed_time 有效但 trusted_now 后过期不触发 RETIRED，以及 anchor 移除/替换/多链 UNTRUSTED。新 inner/outer 只接 ACTIVE，既有 current/history 每层可 ACTIVE 或 RETIRED-nonrevoked。CMS signingTime 与 RFC3339 issued_at/signed_at 只按 UTC whole second 语义相等；1949/1950/2049/2050 边界分别验证 GeneralizedTime/UTCTime，拒绝无秒、小数、offset、错 DER kind。SignerInfo、证书与 CRL 逐项覆盖 ECDSA-P256/SHA256 no-params 和 RSA-PSS exact 正例，拒绝 SHA1/PKCS1v1.5/NULL/隐式/default/错 MGF/salt/trailer。为每个实际 non-anchor issuer 构造完整 base CRL 的 AKI/SKI、签名、CRLNumber、nextUpdate 正例，并逐项拒绝缺失/过期/同号冲突/delta/indirect/removeFromCRL/unknown-critical/OCSP或联网软失败；intermediate serial 命中必须使整链 REVOKED。15 个 descriptor 的路径/strict JCS/DTO/262,144-byte cap、每模块 schema 的 65,536-byte cap/JSON Schema 2020-12/仅 `#` refs、ABI kind/code/排序/唯一与 schema filename-entry-rehash 三等全部正反覆盖；descriptor 任一 byte/依赖/ABI/schema digest 变化却未升 version、同 version 换 digest、compiled registry 缺/多/重复、第二 dependency registry 均使 `cargo xtask module-contracts verify` 失败。`product-modules.v1.jcs` 只能由 verified descriptors 生成并覆盖 MANIFEST/safe-handle readback；`module_state` 保持 raw，而 effective query 对合法负态 Ok(false)、结构/IO/digest/source/catalog/DAG 歧义 Err，feature 不能绕 owner module gate。

普通配置包签名另在 Builtin/HSM 两载体各跑一组正例，证明 secret ref 仅解析一次、before/after resolver 都命中同一 immutable/versioned KeyRef、token 的 32-byte 摘要确实来自该 key 的 exact DER SPKI、sign 后同 ref verify=true，且 canonical key ref/token/signature/signed_at 与状态同事务提交。轮换恰夹在两次 resolve、假 token、错 ref、不可解引用、verify=false、resolver 返回不一致 KeyRef/SPKI 各使零状态/审计部分推进；F-56 special 的 KMS sign/verify/identity-resolver 调用数均为零。

Win/Mac/ServerAdmin strict multipart 边界测试精确覆盖三个受信 ClientKind 正例，以及 boundary 0/1/70/71、合法字符全集与空格/双引号/括号/逗号/斜杠/冒号/等号/问号非法字符、filename 6/7/128/129/非ASCII、header顺序/exact MIME/CRLF/额外part，断言 `framing=136+2B+F<=404`、body cap=4,194,304、archive cap=4,193,900；4,193,901-byte file、4,194,305-byte body、Content-Length 不等 framing+file 与任何 Transfer-Encoding 均在读 body/建 staging 前以 HTTP400/nonretry `PLATFORM.REQUEST.INVALID_PAYLOAD` 且零落库拒绝。OpenAPI 还须证明 Win/Mac 同时保留 JSON attachment 形态，iOS/Android/portal/ops/mcp 均不能调用 multipart。Restricted/NO_CURRENT_GRANT 下，以已认证 Win/Mac multipart 导入首张 LICENSE_GRANT 成功进入后续恢复链；同状态的通用 attachment upload、普通包和 MODULE_PACKAGE 非 DISABLE 全以 `PLATFORM.LICENSE.RESTRICTED` 零写入拒绝，MODULE_PACKAGE:DISABLE 仅经 strict target 保留窄恢复路径。

ServerAdmin bootstrap 的 OpenAPI/serializer 矩阵还须逐一制造顶层 `license_module_admin` 缺键、非空对象任一 12 键缺失/未知、usage 三键或子项三键缺失/未知、modules 行少于或多于 15、七键缺失/未知，以及把任一 unavailable Option 的 JSON null 改成省略或反向改形；全部在 contract test 判红。正例逐分支断言键始终存在、三个实时 current 始终是数值、三个 limit/over_limit 联动 null、module 三个可空字段显式 null，且 code/module 排序与 license 号码遮罩 exact。

F-56 新增必跑负矩阵：recording Tx 对普通 shared、special/checkpoint exclusive 与 typed reject NONE 分支分别断言第一业务 SQL、`try_begin/query/claim` 前置关系、shared 并行及 exclusive 排空；Outbox claim 用 transaction shared、外发用专用 session shared+module shared 且外部调用期间零数据库事务。checkpoint 对 0/1/>1 audit 行、同 slot 不同 trusted_now、零 current 首发与 current CAS 分别验证，既有 append-only payload 更新次数必须为零。special 接受事件逐键重算 chain domain digest 与 source projection domain digest，覆盖 module 三种 inner trust state/action 组合、missing/unknown/重复事件和 same-byte replay。DRAFT→TEST_PASSED 逐态断言 approval 法人为空，submit 才写 derived id；伪预填、错请求头、operator 无治理法人授权与 approve/reject 事务内改判全拒。parser 覆盖直接 NUL、JSON/TOML escape NUL 与相邻合法 Unicode，并逐层验证 INVALID_PAYLOAD、ITEM_HASH_MISMATCH、SIGNATURE_INVALID、SIGNER_NOT_TRUSTED、SPECIAL_ITEM_SHAPE_INVALID、MODULE.PACKAGE_INVALID_OR_INCOMPATIBLE 不串码。

CRL prerequisite 专项构造两 issuer 链：一个 issuer 的 serial 明确 revoked、另一个 issuer 的 global-highest CRL 缺失/非法时，最终必须是 UNTRUSTED/SIGNER_NOT_TRUSTED，不得短路成 REVOKED 或进入恢复路径；只有把第二份 CRL 修复为合法覆盖项后才允许扫描 serial 并得出 REVOKED。

密钥/首装 fresh matrix 还须证明 data-key version 0/65536 与 ref 前导零拒绝、1/65535 和 EPC1 u16 正例，current=65535 的轮换稳定返回 `PLATFORM.KEY_DOMAIN.TRANSITION_INVALID` 且不溢出/回绕；KEK/wrap-KEK 的 1/i32::MAX 正例与 0/i32::MAX+1 在 cast 前拒绝。三条 `user_legal_entity_grants` 的 id/mapping/date/null/granted_by 与两条角色绑定的复合 FK exact。`platform.key_domain.activated.v1` 分别跑 STANDARD/null 与 INITIAL_GOVERNANCE/non-null，逐项核 KEK fingerprint、16 项 purpose/scope 顺序、wrapped digest、同事务状态推进；零域仅 NOT_PROVISIONED，已 PROVISIONING 后所有供给失败只 KEY_UNAVAILABLE。

许可准入注册：对十个 `LicenseAdmissionEffectV1` 分别跑全局有效/Restricted 与 LIST 命中/不命中的矩阵，断言前四类在 Restricted 下统一拒绝、六类闭集不越界，范围外普通 effect 只拒绝当前请求且不改全局 evaluation。配置发布八类操作分别以普通包、grant、revoke、module disable、其他 module action、malformed special 验证 strict target 与 fallback；MCP 以已验签 manifest 的五个 ActionClass 验证唯一映射，并对未验签/缺 binding 拒绝。认证测试固定观察到“凭证已验证、会话尚未签发”之间恰一次 `IdentitySecurityDisposition`。Outbox PENDING、DISPATCHING 首发/重试在 Restricted 下保持队列且零派发；只有注入“外部副作用已发生”的回执/终态/取消/无新副作用补偿可用 `InFlightConvergence`，把新 claim、新外发或新业务效果误标 convergence 必须失败。两个 wiring 分别制造 binding 缺项、多项、重复、错 effect 和同一 operation 多 effect，Blocking 自检、`xtask configdoc` 与 `xtask archcheck` 全部判红；精确相等时三者全绿，检查期间数据库许可查询次数为零。Stage 14 收件的两个具名证据固定为 `license_admission_registry_exact_set` 与 `license_admission_negative_matrix`，不得用泛称报告替代。

全文检索（3b 段）：索引写入只在 job-worker 的消费者中发生，core-server 的用例路径上不出现 `SearchIndexPort` 调用且由 `xtask archcheck` 判负；同一文档重复投递只产生一份索引条目；删除事件后查询不再命中；两个法人的索引分区互不可见；查询按 `SecurityContext.clearance_level` 过滤后仍对命中结果做数据库侧可见性复核。

#### 3.8.3 端到端用例

本阶段无业务模块，端到端用例跑在合成模块上，共八条。

E2E-1 合成单据全链路：创建合成单据（取号、写 Outbox、写站内通知、写审计，审计末位按判定二）、把合成命令加密为唯一 `approval_command_snapshots` 行并只将 snapshot id 放入流程变量、启动审批流程、审批人认领并完成（带重新认证令牌）、owner 校验证据后解密一次，在同一事务消费快照并写 `result_object_type/result_object_id/result_doc_no`、流程完成；客户端持 approval_ref 可经流程实例与唯一快照取得执行结果定位，Outbox 消费产生下游合成效果，审计链验证通过。全程反向扫描数据库与日志不得命中命令明文。覆盖第 3.1 节平台能力八项中的六项，即 Outbox、幂等键、单据编号、审计哈希链、站内通知与流程引擎。

E2E-2 附件全链路：init-upload、分片上传含一次中断续传、complete、检查通过、下载、新增第二个版本、旧版本物理文件仍存在、写删除标记后不可下载但历史审计仍可查。

E2E-3 补偿全链路：三步流程在第三步失败，逆序补偿前两步，第一步补偿重试耗尽，实例进 `MANUAL_INTERVENTION`，人工任务生成，站内通知送达，人工处置后实例取消，全过程审计轨迹可按实例查询完整执行轨迹与补偿轨迹。

E2E-4 定时器与提醒：配置一条提醒规则，定时器触发产生站内通知，进程重启后不重复触发，从备份恢复后不漏触发。

E2E-5 死信与人工修复：注入一个恒失败的消费处理器，事件走完八段退避进死信，站内通知送达责任人，重投仍失败，双人审批后丢弃，全过程写审计。

E2E-6 配置发布最小通道（3b 段）：从空库引导法人默认链，创建含一个 `FLOW_DEFINITION` 项与一个 `NOTIFY_RULE` 项的配置包，经差异审查后以 `CONFIG_RELEASE` 提交；先证明缺少 `SECURITY_ADMIN` 自然人时以 `NODE_HAS_NO_APPROVER` 零写入拒绝，再绑定一名非申请人，由 approve 路径只完成任务、callback 校验包/摘要/版本/链/非自审后批准。随后签名、执行发布，`platform_flow.process_definitions` 与 `platform_msg.notification_templates` 各新增一个已发布版本，`platform.config_release.released.v1` 写出，站内通知送达配置管理员；再回退发布单，两个定义按 `before_spec` 恢复，全过程审计轨迹完整。

E2E-7 影响面闭合正向：用 testkit 的七条真实合成规则建立一份十一目标批次；两个自动结果与四类人工项的每个允许决策形状逐项完成，重复消费、重复 dispose 与重复人工提交各三次。最后一项前合成来源保持 TERMINATING，最后一项后真实 `ImpactSourceCompletionPort` 与批次在同一事务完成，完成事件恰一条；`ImpactAssessmentQuery::by_source` 返回目录序、目标、待办与已存三决策字段一致。

E2E-8 影响面闭合反向与恢复：分别留一项 PENDING、提交错码/错 result id、令一项首投加八次重试均失败至 DEAD，三种情况下断言批次/来源绝不闭合。SLA 到点只通知不代选；DEAD 时批次 FAILED，记名 replay 后复用原批次与原项目继续。另在缺失 completion port 的装配中断言启动/模块启用失败关闭且零 Noop；补回端口与合法人工决定后仅闭合一次。

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

下列 36 项全部达成才算本阶段完成，每项都可由 CI 产物或测试报告客观判定。第 29 项是 3a 段的独立闸门，必须在阶段 4 开工前达成；第 32 项是 3b-1 批的独立闸门，必须在 T0 开跑前达成；第 34、35 项是阶段 6 开工前的 3b-2 硬闸门；第 36 项是任何高保密审批命令消费者启用前的硬闸门；其余各项在 3b 段结束时判定。

1. 39 个迁移文件在空库上按全库中央分配的 14 位版本号全序执行成功，每个文件的 `-- rollback:` 段可执行或已注明只能用备份回退；3a、3b 与阶段 4 的迁移在合并环境中依赖顺序单调，两个 impact slug 与两个 approval snapshot slug 均全局唯一，后者按建表后登记排列。
2. 33 张新表中，27 张带 `legal_entity_id` 的表全部 `ENABLE` 且 `FORCE` 行级安全，策略按统一模板生成，`tests/rls_matrix` 的本阶段部分八类全通过；六张部署级表按裁定 A-05 与 A-27 不带 `legal_entity_id`、不建策略，且已断言其可见性不随 `app.legal_entity_id` 变化。
3. 运行期账号 `ep_app_rw` 在本阶段表上无 DDL、无策略管理权限，`--check` 的 `rls-enabled-and-forced` 与 `runtime-role-privileges-bounded` 两项通过。
4. `--check` 的十五个命名项（基线第 7.3 节现行十项中除 `offsite-sink-requirements` 外的九项，加本阶段六项 `audit-evidence-store-writable`、`audit-signing-key-usable`、`attachment-store-ready`、`event-catalog-consistent`、`impact-registry-consistent`、`license-admission-registry-consistent`）在部署环境上全部通过并输出结构化报告，报告逐项给出 `Blocking` 或 `Degrading` 级别；`--check` 对 FAILED 与 DEGRADED 一律非零退出，`event-catalog-consistent` 在注入不一致时不阻止进程启动而是写出一条降级窗口，`impact-registry-consistent` 失败则以 78 退出且不执行任何影响项，`license-admission-registry-consistent` 对两个 wiring 任一缺/多/重复/错绑定同样以 78 退出且不读取许可业务状态。`offsite-sink-requirements` 按基线第 12 节通则第六条单列：该项在阶段 1 已整条推迟，其被测输入即落点判定与 `DegradationLedger` 登记均不在本阶段交付，因此本阶段不注册该项、不为其输出通过结论，也不计入上述十五项；重新计入的触发谓词是该项已出现在 `SelfCheckRegistry` 的注册清单中，由判定工具自身观测，不以阶段号翻牌。
5. Outbox 可靠投递三组测试全通过，含至少一次投递、重复投递去重、崩溃恢复不丢不重。
6. 幂等键三态判定的九种组合全通过，重复请求回带 `Idempotent-Replay: true`。
7. 编号并发取号 10 万次无重号、无空号，位数扩展临界通过。
8. 审计链：20 并发写入 5000 条后链验证全通过；跨日段切换正确；证据文件外部改写后验证报告定位到该锚定。
9. 锚定：在持续写入负载下 `ep_audit_anchor_age_seconds` 的最大值不超过 900 秒，KMS 中断 5 分钟后自动恢复锚定。
10. 审计证据路径的覆盖与删除尝试被应用层拒绝并写入审计证据，`ep-adapter-file` 的 `published` 与 `evidence` 命名空间在类型层面不暴露删除与覆盖方法（由编译期断言测试证明）。
11. 流程引擎认证套件的五组阶段必过项全部通过，通过结论写入验收证据。规格第 9.1 章的不达标预案未被触发。
12. 附件三段式落盘在四个崩溃点上全部收敛，无孤儿文件、无元数据在而正文不在；每个 AVAILABLE/SUPERSEDED 版本的 EPA1 全部 record 都与唯一 `dek_ref`、法人/domain/ATTACHMENT scope、对象/版本/总明文长度 AAD 一致，轮换不能造成同版本混 key，历史 Range 可按常量 offset 只解密相交块。病毒扫描两种部署分支均通过：NONE 只在两个内建检查 PASS 后发布并持续打开不可抑制窗口，CUSTOMER_ICAP 对 CLEAN 发布、对 INFECTED/超时/不可达/非法响应均隔离且不回退 NONE，非回环地址、主机名与重定向配置全部被启动校验拒绝。
13. 5 GB 单文件分片上传与三点断点续传成功，同一内容两次上传产生两个独立物理副本。
14. 站内通知在推送出口完全不可用的部署形态下照常送达，E2E-1 与 E2E-4 通过。
15. 死信的转入、重投、双人审批丢弃三条路径全通过，按法人与会计期间的枚举查询不走顺序扫描。
16. 五类混沌场景全部通过，故障移除后 5 分钟内自愈，进程崩溃后未完成任务自动恢复且已确认事务零丢失。
17. 八条后端 E2E 全通过。
18. 附录 A.1 的三个本阶段度量项在 20 并发负载模型下 P95 不超过 2 秒，样本各不少于 200，单次运行错误率不超过 0.1%。
19. 六条关键语句的 `EXPLAIN` 证据不含顺序扫描。
20. 覆盖率：本阶段代码行覆盖率不低于 85%，新增与修改代码不低于 80%，工作区整体不低于 80%。
21. `docs/error-codes.md`、`docs/event-catalog.md`、`docs/metrics-catalog.md`、`docs/data-dictionary.md`、`docs/impact-catalog.md` 五份登记与对应代码常量/注册表由 CI 校验一致，无重复码、无未登记项；影响目录恰七条，阶段 3 真实规则注册数恰为 0。
22. 本阶段的五项偏离与十二项新增决定已回写共享技术基线，并经平台架构负责人签字。
23. 模块许可：090100/090200 具备 F-56 终态列与 CHECK/唯一键，090100 原子种入固定 0601..0615 的 15 行 NOT_INSTALLED catalog；093300 在登记六张部署级表的同时补齐六条同包来源 FK、接受摘要/治理法人/历史 package identity 的五表 deferred 图。`ModuleLicenseQuery` 仍恰五个 `Result` 方法，`license_evaluation()` 在同一快照原子返回 status/reason/trusted_now，`module_state` 只读 raw，而 effective module/feature admission 对合法负态 Ok(false)、结构/IO/digest/source/catalog/DAG 歧义 Err。`TrustedClockV1` 的启动 anchor、wall+monotonic 公式、query 零写、readiness/special 关口/target cadence 不超过 240 秒 checkpoint 的同锁 CAS、240-second slot、告警和小于 300 秒诚实窗口均有正反测试；零/首张 current、许可边界、固定 lock 并发、首次 RELEASE 摘要、全部 RELEASED special exact-set CAB 重验/投影交叉/结果分流、非计划漂移、SPKI/64lowerhex/raw32、整条 non-anchor 链 ACTIVE/RETIRED/REVOKED/UNTRUSTED、ASN.1/RFC3339 同秒、证书/CRL 算法闭集、完整 base CRL 选择、15 descriptor/schema/compiled-registry exact set 和 `product-modules.v1.jcs` safe-handle/DAG 同样全绿。首张 grant 冻结 active 治理法人，后继/scope/approval/deactivation/history 约束及 fresh-production `ep-migrate apply` 三参数固定 evidence root 首装、双 CMS/双人员设备凭据、signed key_domain_id、PROVISIONING→ACTIVE、角色权限/默认链、审计/非秘密 receipt/零 sidecar、仅漏 receipt 的同 digest 崩溃恢复均验收。三项 usage 只在 repeatable-read 实时计数并产 metrics/边缘告警，零 usage 表/日终/月报/联网遥测。模块动作 DTO 无法人参数；INSTALL 不要求依赖启用，ENABLE 要求全依赖 enabled；业务/worker shared、ENABLE target-exclusive/dependency-shared、DISABLE 全 15 exclusive/总 30 秒的并发矩阵通过。逐动作 signer 状态固定，current module inner/source-outer CRL 撤销不改变 LicenseStatus，唯一 DISABLE 逃生口保留旧 inner exact bytes且只由 ACTIVE signer 签新 outer，停用后仅双 ACTIVE、更高 semver 的正常 UPGRADE 可替换。五种 Restricted 原因不阻两条恢复链；scope 不匹配统一拒绝且 None 旁路关闭。Stage 3b 证据进入共同 gate；Stage 14 必须同时收到 `license_admission_registry_exact_set`、`license_admission_negative_matrix` 与两个 wiring exact-equal inventory/digest，任一缺失不得宣称全局绿。
24. 最小配置发布通道：三张配置表建立，六态状态机、ADD/MODIFY-after 与 REMOVE-before hash、Stage3 Rust/DB18、四个 applier/wiring exact set 全绿。F-56 `.epcfg` 的 4,193,900-byte ZIP32/STORE 三 entry与330 overhead、2,882,850-byte after-spec-only item、262,144-byte canonical/rebuild manifest、1,048,576-byte detached outer CMS 正反矩阵通过；multipart 的 boundary/filename/header/MIME/CRLF、`136+2B+F<=404`、4,194,304 body与4,193,900 file 双 cap 全闭合。两种 exact after_spec、三 verifier、SPKI signer、special outer exact 保留且零 KMS sign、首次 RELEASE `accepted_trust_bundle_sha256 null→32` 同事务投影、不可变 deferred 图和 NON_ROLLBACKABLE guard 均有正反测试，通用回退绝不调用 special `revert`。
25. 全文检索：`SearchIndexPort` 与 `SearchQueryPort` 在 `ep-adapter-search` 上实现，索引按法人分区落在 `C:\EP\search\<legal_entity_id>\`，`xtask archcheck` 断言业务事务路径上不出现索引写调用，两个法人的分区互不可见。
26. 本阶段全部外部 HTTP 路由，即 `/api/v1/platform/` 各段与第 3.5.2 节三个 `/api/v1/portal/` 端点，均按裁定 A-20、第 3.5 节与 F-56 在注册处声明唯一 `(CapabilityDomain, ActionClass, LicenseAdmissionBindingV1)` 三元组；取值取自 `ep_foundation` 的前两枚举与 `crates/platform/license/src/admission.rs` 的 binding，`crates/platform/flow/src/capability.rs` 中不存在按用例命名的成对常量。认证前置 gate 位于凭证验证后/会话签发前；`push.dispatch.v1` 不声明 HTTP 前两元组而在非 HTTP registry 精确登记 `Fixed(IntegrationOutbound)`。两个 wiring 的 route/job/event/approval-owner/outbound-IPC 实际集合与 binding registry exact equal，`xtask configdoc`、`xtask archcheck` 和 Blocking 自检的正例及缺项/多项/重复/错绑定负例全通过。
27. `DisposalPort` 的 trait 与两个 DTO 已定义在 `crates/platform/file/src/port/disposal.rs`，`RuleEvaluator` 与 `WasmComputePort` 两个 trait 已定义在 `ep-platform-flow`，三者在 `apps/core-server/src/wiring/` 与 `apps/job-worker/src/wiring/` 两个目录下的全部文件中都不出现注入行；阶段 1 随 xtask 交付的 archcheck 规则 `unwired-absent` 在这两个目录上零命中，其前缀集合为 `Noop`、`Stub`、`Fake`、`Dummy` 四类，负样例由阶段 1 提供，本阶段只调用不重复定义；处置受理路由已注册，本阶段至阶段 13 之间的物理删除请求以 `PLATFORM.DISPOSAL.NOT_DELIVERED` 拒绝并开一条 `subject` 取 `DisposalPort` 的 `PORT_NOT_IMPLEMENTED` 降级窗口，`RuleEvaluator` 与 `WasmComputePort` 的能力缺位各开一条 `subject` 取该端口名的同类窗口，三条窗口可同时打开；三者的实现与注入行分别由阶段 13b 与阶段 14 交付并在注入后关窗。
28. 附件的幂等收敛任务在四个崩溃点上收敛，且不产生任何对账差异事项、不实现 `ReconCheck`、不依赖 `ep-platform-recon`。
29. 3a 段闸门：`platform_msg.idempotency_keys` 与 `IdempotencyStore` 实现、`crates/platform/release/src/port/config_item.rs` 端口与注册表两项已完成并通过各自单元测试；3a 段排在阶段 4 之前，`ep-platform-identity` 与 `ep-platform-authz` 两个 crate 此时尚未建立，故这两项所在 crate 的 `Cargo.toml` 中不存在指向它们的依赖项，本条按其 `Cargo.toml` 直读判定，不另立按 crate 逐项比对期望依赖清单的 `cargo metadata` 自检脚本。
30. 按裁定 B-02，`platform_core.append_only_registry` 中存在 `platform_audit.audit_events`、`platform_msg.outbox_events` 与 `platform_msg.dead_letters` 三行登记，`mode` 与 `mutable_columns` 按第 3.3.7 节的取值表逐项一致，三张表上的 `assert_append_only` 与 `assert_immutable_columns` 触发器已按登记挂接，`xtask sqlcheck` 执行 `db/checks/append_only_consistency.sql` 返回零行。
31. 按裁定 A-28，第 34 号迁移执行后 `platform_core.sensitive_field_registry` 中存在 `platform_msg.push_registrations` 的 `token` 一行，`is_field_encrypted` 为真、`blind_index` 为 `EXACT`、`blind_index_column` 为 `token_bidx`、`mask_style` 为 `FULL`、`normalization` 为 `NONE`，物理表上存在 `token_enc bytea` 且不存在同名明文列 `token`，阶段 2 的 `db/checks/11` 返回零行。
32. 3b-1 批闸门：判定四列出的六个 T0 切片在一次连贯执行中成立，即一次取号、一次审计追加与段行链接、一次 Outbox 写入与消费、一条同事务写入的站内通知、一个单审批节点流程实例从创建经人工任务完成到结束、一次经最小发布通道把该流程定义发布到 `platform_flow.process_definitions`，六项在同一测试进程内按上述次序跑通并留下可按实例查询的完整审计轨迹；该闸门不判定附件、检索、推送、定时器、补偿、许可、死信、混沌与任何性能度量项，也不要求 `ep-datagen` 的基准数据集。
33. 按基线第 3.8 节的正向登记制，第 35 号迁移执行后 `platform_core.unpoliced_table_registry` 中本阶段六张不带法人列的部署级表各有一行登记，六行的 `admission_basis` 均为 `SAME_FOR_ALL_ENTITIES`，`isolation_entry` 与 `matrix_case_id` 两列非空且 `matrix_case_id` 可在 `tests/rls_matrix` 中命中第 3.8.2 节所设的对应用例，`db/checks` 的第十三项（`db/checks/13_unpoliced_registry.sql`，由 `xtask sqlcheck` 执行）返回零行。
34. 影响面平台本体完整：两张表及其 CHECK、索引、复合外键与 RLS 全部存在；`ImpactRule`、`ImpactRegistry`、`ImpactAssessor`、`ImpactAssessmentQuery::by_source`、`ManualImpactDecision`、三态结果与七条目录常量逐字符合第 3.4.13 节；`platform.impact_assess` 是 `clm.contract.terminated.v1` 唯一消费者，同一事件重放五次仍只有一个批次。三个真实合成规则产实项、另四类产 PENDING 占位，注册数为 0 的生产装配中不存在 Noop/Stub/Fake/Dummy 规则。
35. 影响面闭合与失败关闭完整：E2E-7/8 全绿，四类人工角色与目录决策码/结果 id 形状逐项正反验证；`NeedsManualDecision` 不增 attempts，SLA 到点只提醒；八档全部可达且第九次失败 DEAD，replay 复用原行；PENDING、人工语义错误、DEAD 三支都阻止闭合。`ImpactSourceCompletionPort` 以 `(source_module,source_event_type)` 唯一注册，全部项 DONE 时与批次同事务调用；缺失、重复或替身在启动和模块启用前均失败关闭并返回 `PLATFORM.IMPACT_COMPLETION_PORT.NOT_REGISTERED`，平台无任何直写业务 schema 的 SQL。
36. 高保密审批命令快照完整：第 38、39 号迁移、表 33、`docs/data-dictionary/platform_flow.md` 与敏感登记行逐字一致；一实例一快照复合唯一键、同法人真实复合外键、RLS、四态/结果定位形状、单向状态触发器与密文字段不可变均有正反测试。CONSUMED 必须在业务对象同一事务固化 type/id 与可空 doc_no，并可由 approval_ref 稳定反查；其他三态结果全空，终态不可改。登记行固定为逻辑 `command`、密级 30、字段级加密、FULL 掩码、NONE 盲索引，`db/checks/11` 返回零行；任一数据库列、流程变量、任务/消息/审计载荷或普通日志出现命令明文，或 CONSUMED 以外产生业务效果，本条失败。

---

### 3.10 与规格和 PRD 的对应

#### 3.10.1 规格条目

| 规格条目 | 本阶段实现内容 |
|---|---|
| 第 5.1 章 Outbox、幂等、编号、通知、文件引用 | 全部四项能力；通知只交付站内通知与移动推送两条渠道，其余九条渠道不实现 |
| 第 5.1 章 低代码流程、审批、定时器、补偿和 SLA | 持久化流程引擎运行时；表单、规则表达式完整能力与 WASM 计算不在本阶段 |
| 第 5.1 章 审计与变更留痕 | 审计事件模型、哈希链、分段签名、链验证工具、审计查询端点 |
| 第 6.5 章 大文件 | 单文件上限 5 GB、分片上传、断点续传、带宽限制、法人密钥域与密级子域加密、不做去重、发布前类型识别与结构型恶意内容检查；病毒扫描按部署必答的 `NONE/CUSTOMER_ICAP` 唯一分支执行，未接入时以不可抑制降级窗口和诚实披露收口 |
| 第 7.2 章 已过账分录、库存流水、审批证据与审计证据不可覆盖 | 三张仅追加表按 `append_only_registry` 登记挂接 `assert_append_only` 与 `assert_immutable_columns` 触发器、不授予 DELETE、`ep-adapter-file` 的不可删除不可覆盖命名空间、CI 的 SQL 静态检查 |
| 第 7.3 章 Outbox 可靠投递测试项 | 三组测试作为数据库认证套件的输入 |
| 第 7.5 章 文件、分析与归档 | 事务库只存对象 ID、版本、哈希、大小、类型、密级、密钥引用与业务关联；上传五段流程；应用级不可变五项要求中的前四项；审计证据与附件使用独立存储路径与独立保留策略 |
| 第 7.7 章 法人行级隔离 | 本阶段 24 张表的统一策略模板；后台扫描按法人逐轮 |
| 第 7.8 章 密钥域 | 附件按法人密钥域与密级子域以固定 1 MiB EPA1/EPC1 分块加密，同一版本只绑定一个 pinned data-key ref；推送令牌按字段级密钥加密且 `KmsBackend::unwrap` 直接返回业务明文，`token_bidx` 只取 scoped selector `platform_msg.push_registrations.token@30` 的完整 32 bytes，承担同一法人密钥域下的查重而不承担唯一性，密文不进索引与唯一约束，该列按裁定 A-28 在 `platform_core.sensitive_field_registry` 登记一行 |
| 第 7.9 章 派生存储安全继承 | Outbox 信封强制携带来源对象 ID、版本、法人 ID、密级与数据范围标签，缺失即拒绝入队；内置检索索引按法人分区，索引文档携带密级与数据范围标签，查询按 `SecurityContext.clearance_level` 过滤且不作为授权判据 |
| 第 5.6 章 模块生命周期 | F-56 永久/订阅许可、四态与可信时间、module/entitlement 硬门、三张平台表、终态 `ModuleLicenseQuery`、签名声明式模块包与五条合法动作；模块停用保留全部数据并停止 UI/写入/定时器/新事件派发 |
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
| 10.5.2 提醒事项清单 | 九类提醒事项的 `notice_type` 枚举与模板（PRD 十类撤下「许可临期与宽限期告警」）；其中审批待办、审批结果、高风险操作待审批、流程时限四类由本阶段的流程引擎直接触发；合同到期、对账差异、关账被拒、死信与人工任务、许可临期、运维降级六类由本阶段提供写入接口，触发源在各自阶段 |
| 10.5.3 提醒规则的配置 | 定时器承载，触发与执行幂等且可重放，重启与从备份恢复后不漏不重 |
| 10.5.4 通知中心的用户操作 | 查看、跳转、标记已读、与审批待办的关系四项；跳转后仍按权限判定，无权时不展示单据内容 |
| 10.5.5 异常 | 无权限时正文不含无权字段；推送不可用时不重试到其他渠道；接收人停用时写 `RECIPIENT_INACTIVE`，由同法人 `SECURITY_ADMIN` 人工改派给具备原能力的有效用户并完整审计 |
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

风险三，`jsonb` 的数值规范化会破坏哈希可重算性。控制手段是：没有具名 typed-audit ABI 时，`before/after` 的业务 decimal/integer 使用 canonical 十进制 JSON string，并以 `1.10` 与 `1.1` 的差异测试保护前像；具名 strict DTO 则逐字段遵循其冻结 wire type，不能被通用字符串规则覆盖，尤其三个 F-56 typed audit 的 `schema_version` 必须是 JSON number `1`。残余风险是业务模块开发者绕过对应 DTO 直接序列化数值，必须由 `AuditRecorder` 的区分型入参和 schema 测试在编译/测试门拦截。

风险四，staging 明文窗口。已采用会话级临时密钥消除，但会话密钥与数据密钥同在一台服务器上，掌握操作系统权限者仍可解密。这是规格第 7.5 章与第 13.1 章已声明的残余风险的一部分，不额外承诺。

风险五，基础产品的两个内建检查器只能拦住类型不符与已知结构特征，无病毒库即无法拦住已知恶意样本。`NONE` 模式始终打开不可抑制降级窗口并写入交付说明与合同，不得表述为已提供病毒防护；`CUSTOMER_ICAP` 模式的实际防护能力、病毒库更新与误报漏报由客户选择的同机扫描器决定，平台只保证未取得 `PASS` 就不发布附件。

风险六，移动推送外部通道可能不可用。进程归属已经冻结为 job-worker 编排与脱敏载荷组装、integration-gateway 唯一出网投递，不存在实现分支；部署没有 APNs/FCM/厂商通道或投递失败时，`push_enabled = false` 或送达状态记 `FAILED`，只剩站内通知，不影响任何必须送达的提醒，按规格第 5.7 章如实披露移动端到达能力缺失。

风险七，本阶段无业务模块，全部验收在合成聚合上完成，存在“合成场景通过而真实场景不通过”的风险。控制手段是合成聚合必须覆盖真实业务的三个特征：多表写入、跨模块契约调用、带会计期间归属的事件；并在业务模块阶段回归本阶段的流程引擎认证套件。

风险八，Outbox 表在基准数据集下达到 200 万行，取件语句的索引选择性依赖 `status` 的分布。若 `DONE` 条目清理不及时，`status = 'PENDING'` 的扫描可能退化。控制手段是保留期清理作为必跑的后台任务并进入连续两个执行窗口未完成即告警的口径，以及 `EXPLAIN` 证据作为退出条件。

风险九，保留期清理的数据库权限已按基线第 3.6 节收口：`ep_app_rw` 只对逐表具名的清理白名单获得表级 DELETE，其他表仍由权限与 SQL 静态检查拒绝，不存在 schema 级授权或待决偏离。清理顺序必须保证 `inbox_consumptions` 的保留期严格长于 `outbox_events` 的 `DONE` 保留期，否则先清消费记录再重放已清 Outbox 条目会重复产生副作用；本阶段取 60 天与 30 天，差值 30 天，该大小关系是两个配置键的比较，作为 `config-parsed` 的一条配置校验实现，不另设自检项、不另占一个注册名。

风险十，3b 段的范围因裁定 A-05、A-07、A-19、A-27 与 F-56 扩大到许可、检索与配置发布三项，工期与评审面随之扩大。控制手段有二。其一，许可/模块只交付终态表投影、内层验签、状态机、唯一查询契约和两个特殊 applier，不另建端点、进程或可执行模块载体；检索只交付端口、适配与消费者；配置发布只交付六态与四个 applier，不交付自动测试编排、编辑锁与在线 DDL。其二，许可与检索放进 3b-2 批，关键路径上只留 3b-1。残余风险是阶段 13b 必须在既定 090500 中把 config item Rust/CHECK 从 Stage 3 同序 18 追加两个 MCP 项到终态 20并完成特殊单项包全链；该扩展按基线第 3.9 节在线 DDL 约束执行并持有迁移窗口，不得修改 Stage 3 历史迁移。

风险十一，3a 与 3b 拆段后迁移号段跨越阶段 4。若排期变动使 3b 早于阶段 4 落地，第 2 至 34 号的时间戳必须一并前移，否则会出现已应用版本号大于待应用版本号的乱序。控制手段是把号段与阶段顺序的对应关系写入第 3.3.1 节，并在 CI 中断言本阶段 3b 号段严格大于阶段 4 的最大版本号。

#### 3.11.2 为后续阶段预留的扩展点

`AuditRecorder` 端口：业务模块只调用 `record(tx, ctx, action, object, before, after, reason, approval_ref, reauth_ref)`，不接触链与段。

`NumberAllocator` 端口：业务模块只声明类型码与作用域，不接触序列表。

`OutboxWriter` 与消费者注册表：新模块只需登记事件类型与处理器，不改调度代码。

`ObjectStore` 的三个命名空间与 `DisposalPort`：阶段 14 以 `OpsDisposalService` 实现 `DisposalPort` 并同批写入两个 wiring 目录的注入行，不改存储适配；本阶段不预置任何注入，处置受理路由已注册并以 `PORT_NOT_IMPLEMENTED` 降级窗口承载能力缺位，扩展点是一个 trait 加一条可关闭的窗口，而不是一个已接线的空壳。

`ConfigItemApplier` 与 `ConfigItemApplierRegistry`：阶段 4、11、13b 与 13c 只需实现各自 applier 并在两个 wiring 注册；Rust 终态闭集为 20。普通项复用发布、差异、签名、审批与回退；F-56 两类特殊项只复用发布链并禁止通用回退，后续不得再以“可扩展”绕开闭集、CHECK 与签名裁定。

`SearchIndexPort` 与 `SearchQueryPort`：业务阶段只需按 `SearchDocument` 结构提供投影函数，不接触索引分区与写入时机。

`ModuleLicenseQuery`：各阶段只读该 trait 的五个 `Result` 方法判定模块状态、许可 evaluation、module/entitlement 与 feature；status/reason/trusted_now 只能取一次 `license_evaluation()` 返回的同一 `LicenseEvaluationV1`，不得拆查。调用方不直接读许可表、不缓存布尔值，也不另定义 F-55 许可 payload。

`ContentInspector`：基础产品只有 `TYPE_SNIFF` 与 `STRUCTURE` 两个内建实现；客户病毒引擎的唯一接入面是 integration-gateway 的同机 ICAP 客户端与 `NONE/CUSTOMER_ICAP` 配置，不允许新增 CLAMD socket、远端明文扫描或另一套扫描端口。

`RuleEvaluator` 与 `WasmComputePort`：阶段 13b 的接入位点已存在，实现类型分别为 `AstRuleEvaluator` 与 `PluginHostWasmCompute`；本阶段只提供流程守卫条件的最小求值器，不占用这两个端口，两者在本阶段至阶段 13b 之间不注入任何实现，能力缺位按 `subject` 取端口名的 `PORT_NOT_IMPLEMENTED` 降级窗口承载，阶段 13b 注入实现后关窗。

`ChannelDispatcher`：通知渠道的抽象接口已按两条渠道实现，后续版本恢复统一消息中心时新增渠道不改通知写入侧。

`v_attachment_watermark_inputs` 与 `v_evidence_write_inputs`：归档与备份阶段的唯一接缝。

`ix_outbox_events_le_period_status` 与 `ix_dead_letters_le_period_state`：关账阶段判定受理前提的取数入口。

`flow.state_persistence` 开关：规格第 9.1 章不达标预案切换到外部流程编排平台时，`outbox_eventual` 路径即为该预案的落点，本阶段已实现并已测试。

---

### 3.12 对共享技术基线的偏离项与本阶段新增决定

#### 3.12.1 已批准并同步的基线修订，共五项

以下五项均已写回共享技术基线，是开发直接采用的现行口径；“偏离”只描述发现时的旧差异，不再代表等待批准、签字或二次选择。

修订一，integration-gateway 的外部通信归属。旧基线把 integration-gateway 定为“首版唯一的对外出网进程，只承载电子签章一类出口”，同时把“站内通知与推送投递”列为 job-worker 职责。现行处理是：推送的编排与脱敏载荷组装留在 job-worker，实际投递只由 integration-gateway 执行；integration-gateway 承载电子签章、可选移动推送与可选同机客户 ICAP 病毒扫描三类外部通信。ICAP 目标只能是回环地址，属于文件安全依赖而不是业务外部系统；产品侧只新增 `\\.\pipe\ep-integ` 的具名 operation 与适配器，不新增内部 HTTP 端点。gateway 的数据库连接、Outbox 消费、KMS 和业务文件权限均为零，资源单位与系统账户不变；推送/签章效果由 job-worker、病毒扫描结果由 core-server 在各自权威事务落库。规格第 15.1 章的 `EXTERNAL_SYSTEM` 错误分类首版仍仅指电子签章，推送与 ICAP 失败不进入该业务错误率口径。

修订二，二进制正文通道。基线第 5.1 节已登记唯一例外：附件正文的分片上传与下载使用 `application/octet-stream`，路径限于 `/api/v1/platform/attachments/uploads/{session_id}/parts/{part_no}` 与 `/api/v1/platform/attachments/{id}/versions/{version_no}/content` 两类，其余一律 JSON。影响范围仅限这两类路径。

修订三，分片上传的幂等键落库豁免。分片 PUT 单次上传可产生 640 次写请求，为每次写一行幂等键在 7 天保留期内产生无谓膨胀。基线第 5.4 节现已把该具名端点冻结为唯一落库豁免：请求仍带 `Idempotency-Key`，数据库以 `(legal_entity_id, session_id, part_no)` 和 `part_hash` 判定同内容回放或异内容冲突；其他端点不得类推。

修订四，保留期清理的 `DELETE` 范围。基线第 3.6 节已登记七类清理对象与永不清理清单，并冻结 `inbox_consumptions` 保留期严格长于 `DONE` Outbox 保留期、父子逆序分批删除和正反 SQL 静态检查；未列名对象不得清理。
修订五，删除基线第 7.3 节的自检项 `license-and-modules-consistent`，并为自检项引入 `Blocking` 与 `Degrading` 两级取值域。该项判读运行期可变商业数据，若失败即拒绝启动，会使 F-56 的 GracePeriod、Restricted 与身份安全处置不可达。许可判据由第 3.4.11 节 `ModuleLicenseQuery` 的逐次验签状态机、用量审计和业务硬门承接，不借用 `DegradationKind`；`--check` 对现存两级自检一律非零退出，进程启动只被 `Blocking` 阻止。

#### 3.12.2 已同步的三项基线澄清

澄清一，工作单元内的写入顺序。共享技术基线第 10.3 节已经冻结为保存聚合、写 Outbox、最后写审计：审计段行是全局串行化点，其排他锁持有到事务提交，所以审计写入必须是工作单元闭包内最后一次数据库写入；任何阶段不得在其后再发起 Outbox 入队、投影回填、站内通知或幂等结果回写。本阶段直接采用该现行顺序，不存在旧示例分支。

澄清二，仅追加表的 `reverses_id` 与可更新状态列。共享技术基线第 4 节现已冻结为：`reverses_id` 按该表有无业务冲销或更正语义逐表判定，有的必须带并写明指向对象，没有的一律不得带，不为满足列约定保留恒为 NULL 的列。据此本阶段八张仅追加表中只有 `process_compensations` 带该列并指向被补偿的 `process_steps.id`；`audit_events`、`outbox_events`、`dead_letters`、`process_steps`、`inbox_consumptions`、`scan_results` 与 `upload_parts` 七张不带。`audit_events` 的列集按基线第 9.4 节固定；`outbox_events` 与 `dead_letters` 不带 `row_version`、`updated_at`、`updated_by`，其状态只按第 3.3.7 节可变列白名单条件更新。

澄清三，`audit_events` 的基线索引。基线第 3.10 节已登记该表的唯一时间列例外：因固定列集只有 `occurred_at` 而无 `created_at`，以 `ix_audit_events_le_occurred` 覆盖 `(legal_entity_id, occurred_at, id)`，不为套模板新增列。

#### 3.12.3 本阶段新增决定，共十二项，直接构成开发契约

下列决定均已批准；已适合抽成跨阶段通则的部分写入基线，其余由本节作为阶段 3 的唯一权威，不等待阶段结束再选择或签字。

一，`ep-foundation` 新增 `resilience` 与 `canonical` 两个模块。二，平台端点的路径模块段固定为 `platform`，事件类型的模块段固定为 `platform`。三，索引名超过 63 字节时的确定性缩短规则。四，审计哈希输入采用 RFC 8785 JCS；无具名 typed-audit ABI 时 `before/after` 中的业务 decimal/integer 以 canonical 十进制 JSON string 承载，具名 strict DTO 则必须逐字段遵循其冻结 wire type，其中 `CONFIG_SPECIAL_ACCEPTED`、`LICENSE_TRUSTED_TIME_CHECKPOINT`、`MODULE_SIGNER_REVOKED_DISABLED` 的 `schema_version` 都是 JSON number `1`。五，`audit_events.client` 增加 `system` 取值。六，`idempotency_keys` 增加 `state` 列与三态判定，其中并发在途一路不占用 `IdempotencyOutcome` 的变体，以 `Err(PLATFORM.IDEMPOTENCY.IN_PROGRESS)` 返回，与裁定 C-07 冻结的三个返回值并存。七，站内通知在业务事务内同步写入，不经 Outbox。八，审计链验证工具的入口只有 API 加后台任务，不交付 CLI 子命令。九，启动自检增加六个命名项。十，`ep-adapter-file` 划分 `published`、`staging`、`evidence` 三个命名空间，删除方法只在 `staging` 上存在。十一，按裁定 A-05 与 A-27，`platform_core` 的三张许可表与 `platform_meta` 的三张配置表由本阶段 3b 段建立，阶段 13b 只做列扩展与状态扩展，本阶段不建 `ep-platform-meta` 的任何自定义对象表。十二，按裁定 A-07，`ep_foundation::port::search` 的类型体与两个 trait 由本阶段补齐，阶段 1 只建空文件；索引写入只允许出现在 job-worker 的消费者中，由 `xtask archcheck` 断言。

#### 3.12.4 原业务待决事项的冻结值

本阶段不被任何业务决定阻塞。下列六项已冻结为首版值；未来变更走签名配置或正式规格修订，切换代价仍按表中记录评估。

| 未决编号 | 事项 | 本阶段冻结取值 |
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

依赖一，工程基线（阶段 1）：Cargo workspace 骨架、`rust-toolchain.toml`；`ep-foundation` 的 `Id`、`Money`、`Clock`、`IdGen`、`Rng`、`SecurityLevel`、`AppError`、`ErrorCode`、`DomainEvent` 信封，按裁定 A-03 冻结的 20 字段 `SecurityContext`、四个配套枚举与两个构造函数，按裁定 A-01 冻结的 `port::tx` 三件套 `Tx`、`SnapshotCtx`、`UnitOfWork`（两个方法 `transact` 与 `snapshot_transact`）与 `id::marker` 的 22 个标记类型，按裁定 A-02 冻结的 `SYSTEM_PRINCIPAL_ID` 与 `SYSTEM_DEVICE_ID`，按裁定 A-05 冻结的 `ModuleCode`，按裁定 A-20 冻结的 `CapabilityDomain` 与 `ActionClass`，以及 `port::search` 的空模块文件；按裁定 C-24 由阶段 1 登记的七个平台错误码；按裁定 C-07 的 `IdempotencyKeyHeaderGuard`；按裁定 C-05 的 `tests/rls_matrix` CI 目标与八个断言函数；按裁定 C-23 注册的两个数据库连接池指标；`ep-adapter-db-pg` 的连接池、会话变量注入与归还清除钩子，`tools/ep-migrate` 单一全局自建 Runner 的骨架与 `db/migrations/` 下二十四个空目录，CI 的依赖方向自检与 SQL 静态检查，`ep-testkit` 与 `ep-datagen` 骨架。缺失则本阶段无法开工，无临时替代。

依赖二，密钥与密码（阶段 2）：端口 `ep_foundation::port::kms` 的 `KmsBackend` 仍恰含信封加密 `wrap/unwrap`、ECDSA P-256 `sign/verify`、盲索引 `derive_blind_key` 与 `BlindIndex` 六方法；同模块另有只读 `KmsSigningKeyIdentityResolver` 和值型 `SigningKeyIdentityV1 { key_ref: KeyRef, spki_sha256: [u8;32] }`，其 `signer_subject()` 是 `spki-sha256:<64 lowerhex>` 的唯一生成方法。密钥域供给另由三方法 `KmsKeyMaterialProvisioner::{ensure_kek,generate_detached_data_key,readback_wrapped_data_key}` 承担，ACTIVE 域必须具备 FIELD/BLIND_INDEX/ATTACHMENT/ARCHIVE × 10/20/30/40 exact 16-row ACTIVE 矩阵；WrappedDataKey/Readback/DataKeyHandle/DataKeyRef 与 generate 参数的 data-key version 统一为非零 `u16`，DB 范围 1..65535，保持 EPC1 header 的 2-byte u16 布局，current=65535 再轮换以 TRANSITION_INVALID 失败。KEK/kek_version/wrap_kek_version 的 Rust 域可为 u32，但 SQL int 持久域统一 1..=2,147,483,647，adapter/DTO 必须在 cast 前拒绝更大值。正文流式加密另由恰三方法的 `KmsPinnedDataKeyBackend::{resolve_data_key,wrap_with_data_key,unwrap_with_data_key}` 承担，selector 只取 `CurrentForWrite|ExactRef(DataKeyRefV1)`，`DataKeyRefV1` 与 handle 的只读 `canonical_ref()` wire 唯一为 `data-key://<lowercase-data-key-uuid>#<u16无前导零>`，不暴露秘密。`BuiltinKmsBackend` 与 `HsmKmsBackend` 同时实现上述端口，resolver 只从 immutable/versioned KeyRef 指向的 exact DER SPKI 求摘要且永不返回私钥或明文 DEK。每法人数据加密密钥域与密级子域同属阶段 2。本阶段的普通配置包签名按第 3.4.12 节执行 identity-before/sign/verify/identity-after 闭环；`push_registrations.token_bidx` 只取 `derive_blind_key(legal_entity_id,"platform_msg.push_registrations.token@30",plaintext)` 的完整 32-byte 派生值，裸 FQN 拒绝；附件逐块只用 pinned 端口。所有调用只依赖 foundation 端口，不依赖 `ep-adapter-kms`，按裁定 F-04。阶段 2 排在本阶段之前，本条不留任何桩路径，第 3.9 节退出条件的第 9、10、12、13 项一律在真实载体实现上判定。

依赖三，法人与组织（阶段 2）：`ep-platform-tenancy::LegalEntityDirectory::list_active`（供后台按法人轮转）与其返回的 `LegalEntityRef`，编号格式的法人段取该结构的 `entity_no`（2 位数字）。缺失时以固定两法人的测试夹具替代，退出条件不受影响，但编号格式的法人段无法在真实数据上验收。

依赖四，身份（阶段 4）：`ep-platform-identity` 的用户目录、会话令牌校验、设备登记、用户停用状态、`X-Reauth-Token` 的签发与校验。缺失则通知接收人解析、推送设备绑定、人工任务的重新认证三处只能用桩。

依赖五，授权（阶段 4）：`ep-platform-authz` 的权限项注册与判定入口、职责分离判定（申请人不可自审）、字段级与密级过滤。缺失则全部端点的权限声明只能登记不能生效，第 3.9 节退出条件第 2 项的“报表投影”与“错误信息泄漏”两类无法判定。

依赖六，元数据（阶段 13b）：`ep-platform-meta` 的自定义对象注册与六个 `CUSTOM_` 前缀的 applier，用于让附件、流程、审计、检索对自定义对象自动生效，规格第 7.4 章要求“自定义对象自动获得权限、流程、审计、搜索、API 和报表能力”。缺失不阻塞本阶段核心能力，但该自动生效的验收推迟到阶段 13b。

依赖七，配置发布与模块许可：两项均按裁定 A-05、A-19、A-27 与 F-56 前移到本阶段。3a 交付端口与注册表，3b 交付最小发布通道、同序 Rust/CHECK `ItemKind=18`、许可/模块本体及两个特殊 applier，见第 3.4.11 至 3.4.12 节。阶段 13b 只补十一态、九套自动测试、编辑锁，并在既定 090500 同批追加 MCP 两项使 Rust/CHECK=20，再完成特殊单项包生成/只读校验与 ServerAdmin 既有 API 组合；阶段 13c 只消费 entitlement。模块停用再启用和签名许可全链验收顺延到阶段 13b，最终共同通过 `RG-LICENSE-MODULE-LIFECYCLE-GREEN`。

依赖八，可观测性（阶段 2）：`ep-platform-obs` 的日志字段约定与指标注册表，含按裁定 C-21 由阶段 2 注册并填充的 `ep_db_tx_retries_total`；本阶段三个例外端口的缺位窗口与降级级自检项一律经阶段 2 的 `platform_ops.degradation_windows` 与 `DegradationLedger` 登记，`kind` 取 `PORT_NOT_IMPLEMENTED`，`subject` 取端口或能力的完整类型名，该列与在 `kind`、`subject`、两个 scope 列及开窗状态上的唯一约束由阶段 2 提供。本阶段产出第 3.3.5 节的四个数据源视图，并注册七条指标：基线第 9.2 节固定清单中的 `ep_outbox_pending_events`、`ep_outbox_dispatch_attempts_total`、`ep_dead_letters_open` 与 `ep_audit_anchor_age_seconds` 四条，本阶段是这四条的唯一注册方与填充方；以及第 3.4.3 节的 `ep_audit_evidence_write_failures_total`、第 3.4.8 节的 `ep_flow_instances_manual_intervention` 与第 3.11.1 节的 `ep_audit_segment_lock_wait_seconds` 三条，这三条按基线第 12 节先回写基线第 9.2 节的指标清单再实现。本阶段不注册这七条之外的任何指标；`ep_archive_write_lag_seconds` 与 `ep_attachment_write_lag_seconds` 由写出进程所在阶段注册，本阶段只按第 3.3.6 节提供两个只读输入视图。台账条目的登记由运维中心承担。

依赖九，归档与备份（阶段 14）：archive-writer 的审计证据与附件正文向服务器之外落点的写出，backup-writer 的每日全量备份。本阶段只提供两个只读输入视图，不实现写出；按裁定 C-27，archive-writer 对审计证据目录只有组只读权限，证据文件与段根签名由本阶段的 job-worker 产生。规格第 13.3 章的 RPO 判定不在本阶段范围。

依赖十，电子签章（阶段 6）：`ep-adapter-esign` 按裁定 A-25 由阶段 6 交付，目录 `crates/adapter/esign/`，装配进 integration-gateway。本阶段交付熔断器与重试组件供其复用，`EXTERNAL_SYSTEM` 错误分类的唯一来源在阶段 6 落地，真实沙箱的通过记录在阶段 14 的认证清单中判定。

依赖十一，业务决策：U-K-04、U-K-05、U-K-06、U-L-03、U-L-04、U-A-11 六项，本阶段已给冻结取值，不构成阻塞。
