> **F-57 状态：`SUPERSEDED_DO_NOT_EXECUTE`。** F-52–F-54 只保留历史裁决证据；F-57 权威登记和实施计划是当前唯一口径。

## 本文件的地位

本文件包含两类内容，必须分开解释：`F-52　最后五项内部开发阻断收口`、`F-53　阶段 14 历史迁移、补丁分发、支持套餐与病毒扫描部署收口`、`F-54　全局登记闭合与合同终止影响面平台补齐` 三个完整专项段，**按本文件首行横幅与 F-57 权威登记为历史裁决证据，不是现行施工口径**（本句原写「已批准的现行后续裁定与唯一实现口径」，与首行逐字「F-52–F-54 只保留历史裁决证据」相反，据首行更正）；其在各自明示范围内曾为已批准的后续裁定；若与更晚的 F-55/F-56 明示范围重叠，按同范围较晚裁定优先。除此三段外，本文件自本次修订起降级为 67 条历史登记表，不构成独立权威；一般权威链固定为规格、PRD、技术基线、各阶段计划，历史登记与这四层冲突时一律以后者为准。各阶段计划中的“按裁定 X-nn”只表示决策出处，取值以已回写的现行正文为准。

> **F-55 后续覆盖。** 本表及其 F-09/F-11/F-12/F-53 段中关于本地 AI、双向 MCP、ServerAdmin、72 格/八进程或云承载的旧条件和旧延期句，已由 `../../specs/2026-08-22-f55-approved-ai-mcp-server-admin-cloud-freeze.md` 覆盖，只作决策追溯；不得据此阻塞或分叉 F-55 实施。

> **F-56 后续覆盖。** 本表 A-05、A-19、F-52 及其他历史段中的许可短表、`Valid|ExpiringSoon|Expired|Revoked`、三方法 `ModuleLicenseQuery`、15/16 项 `ItemKind` 与相应 suite 计数，已由 `../../specs/2026-08-22-f56-license-signed-module-package-freeze.md` 原子替换。现行实现只采用 F-56 的签名 current/history grant、四态、五个 `Result` 方法、Stage 3 同序 18/Stage 13 终态 20，以及九套 suite 对终态 20 的回写；下文旧代码块和旧确切标识符只证明决策演进，不得据以施工。

除上述 F-52/F-53/F-54 三段（**现为历史裁决证据，见首行横幅**）外，历史登记部分只承担三件事：登记 67 条缺口的原归属、原确切标识符、作废名清单。结论、最终归属阶段和确切标识符都只作长期追溯；一旦落入 F-50 至 F-56 的明示覆盖面，现行取值必须改读后续裁定与已回写计划。其中提供方要做什么、每个使用方要改什么、顺序约束三段，以及一切原方案作废、该措辞作废、其后编号顺延、总览第 N 节须改写一类句式，都是 2026-08-10 四次回写提交的施工期工单，回写已执行完毕，一律作废：不得引用、不得据以施工、不得据以判定评审阻塞，下一次修订本文件时逐条删除。文末的历史回写清单整节同此处理。

本表中一切以 Noop 为前缀的类型名、一切先注入空实现后反向替换的措辞、一切验收顺延的措辞，按下列通则第三条一律作废。不得新增第二张裁定表，不得让计划正文以裁定表为唯一出处，此二者列入评审清单。

## 五条通则

以下五条对全部 67 条登记项生效，各阶段不得再解释。通则属技术基线层，本登记表不得凌驾其上。

第一，F-52/F-53/F-54 三段**按本文件首行横幅与 F-57 权威登记为历史裁决证据，不是现行施工口径**（本句原写「三个现行专项段」，与首行逐字「F-52–F-54 只保留历史裁决证据」相反，据首行更正）；其在各自明示范围内曾是后续裁定，F-55/F-56 在重叠范围内按较晚者优先；其余 67 条历史登记的权威顺序为规格、PRD、技术基线、阶段计划，历史登记本身在权威链之外。

第二，模块归属的唯一判据是基线第 1.2 节的 15 个模块码覆盖范围与基线第 1.3 节最后一条“禁止跨模块直接读写业务表，一个仓储只访问自己模块的 schema”。表落在哪个 schema，该 schema 对应的模块所在阶段就是该表的所有者，不存在“甲阶段在乙模块的 schema 里建表”这一形态。

第三，跨模块同步调用的被调方必须与调用方同批交付。被调方阶段晚于调用方阶段的，调用方本轮不做该调用，承载该调用的用例整体推到被调方所在批次；不得先注入空实现再回头替换，不得把验收顺延到被调方阶段。apps/core-server/src/wiring/ 与 apps/job-worker/src/wiring/ 两个目录下的全部文件中不得出现任何以 Noop、Stub、Fake、Dummy 为前缀的实现类型或注入行，由阶段 1 交付的 xtask archcheck 规则 unwired-absent 断言，出现即构建失败，该规则配一个故意违反的负样例，Unwired 一名撤销。唯一例外是规格把交付时点冻结在末期的三项平台能力，即 WasmComputePort、RuleEvaluator 与 DisposalPort，三者及其宿主进程 plugin-host 与承载 crate 一律保留：三者在其交付阶段之前不注入任何实现，改由 platform_ops.degradation_windows 承载，取值一律为阶段 2 定义的 DegradationKind 的 PORT_NOT_IMPLEMENTED 并由 subject 列记下该端口名，WASM_COMPUTE_NOT_DELIVERED、RULE_EVALUATOR_NOT_DELIVERED 与 DISPOSAL_NOT_DELIVERED 三个取值撤销，能力缺位时开一个降级窗口，界面与健康端点显式呈现该能力未交付，指标 ep_degradation_windows_open 自动计数；三者在能力缺位时返回可重试错误或直接拒绝，不得静默按成功路径放行，也不得以不注册路由返回 404 的形态替代该降级窗口。本条的完整裁定见总览第 1.5 节第六条至第八条与第十一条，本表只作登记。

第四，关键路径固定为：1 → 2 → 3a → 4 → 3b-1 → T0 → 5 → 9a → 8 → 6 → 7 → 10 → 11 → 9b → 14。阶段 12 在阶段 10 后与阶段 11 并行；13a 在阶段 1 后并行并前移移动薄 PoC；13b 在阶段 3b 与阶段 11 后和阶段 9b 并行；阶段 14 最终验收同时等待 13a 与 13b。T0 是插在阶段 3b-1 与阶段 5 之间的最薄贯通线，不新增任何范围，只从阶段 5、6、9a、10、11 各取一个最小切片，其体量是这五个阶段各自最小子集的当量之和，定义见总览第 2 节总表 T0 行与第 5 节 MT0 行。本表全部顺序约束都以这一口径为准。

第五，db/migrations/order.toml 撤销。迁移文件放在其主要创建对象所属 schema 的目录下，二十四个目录保留；执行顺序由单一全局 Runner 按文件版本号排序，历史表合并为 platform_core.schema_history 一张，命名 V<YYYYMMDDHHMMSS> 且版本号取真实时间、全局唯一、严格递增，由 xtask sqlcheck 断言，正确性由空库全量执行在 CI 中验证。其后编号顺延这一整类连锁改动取消。

## 作废名清单

本清单是本表唯一长期有效的差分产物，作用是防止旧名被捡回来。任何阶段不得使用左列名字。

| 作废名 | 取代它的名字或去向 | 出处 |
|---|---|---|
| 全部以 Noop 为前缀的实现类型 | 撤销，跨模块调用同批交付或整只推迟；三项末期能力改由降级窗口承载 | 通则第三 |
| WASM_COMPUTE_NOT_DELIVERED、RULE_EVALUATOR_NOT_DELIVERED、DISPOSAL_NOT_DELIVERED | 三个取值一并撤销，改取 `DegradationKind` 的 `PORT_NOT_IMPLEMENTED` 并由 `subject` 列记下该端口名 | 通则第三、A-26 |
| db/migrations/order.toml 与二十四项位次序 | 撤销，单一全局 Runner 按版本号排序 | 通则第五 |
| <schema>.refinery_schema_history 二十四张 | platform_core.schema_history 一张 | 通则第五 |
| transact_repeatable_read | UnitOfWork::snapshot_transact | C-03 |
| ep_db_retries_total、ep_tx_retry_total | ep_db_tx_retries_total | C-21 |
| ep_db_replication_crosscheck_age_seconds、ep_replication_crosscheck_age_seconds | 两者一并撤销，交叉核对折叠进保留量周期采样器 | C-22 |
| WRITER_ROLE_CONTAINMENT_MISSING | WRITER_NOT_IN_SERVICE，判据改为写出进程未运行或连续无上报 | A-26 |
| REPLICATION_CROSSCHECK_NO_RESULT | 保留；F-52 以共享 30 秒采样器的连续第二次 `NO_RESULT` 恢复，不恢复 C-22 已撤销的专用子系统 | F-52 部分替代 C-22 |
| ep_quota_throttled_total 与 platform_ops.quota_events | 撤销，按应用层限流与超时计入附录 A.2 错误率口径 | 基线第 2 节 |
| RESOURCE_QUOTA_EXPOSURE、BACKGROUND_TASK_WINDOW_MISSED | 两个取值一并撤销，无取代名 | 14-ops-backup-release.md 第 3 节 |
| cgroup-quota-matched、license-and-modules-consistent、current-period-open | 三个自检项撤销 | C-25 |
| duty-class-exclusivity、forbidden-permission-items-absent、master-data-usage-probes-registered、client-capability-matrix-frozen | 四个自检项撤销，下沉为写入侧约束、模块启用前置校验或内置快照为准 | C-25 |
| inventory.stock_value_adjusted.v1 | 撤销，不设替代事件 | B-09（F-54 复核） |
| finance::CreditExposureQuery、finance::CustomerCreditExposurePort | ep_contract_finance::ReceivableExposureQuery | C-14 |
| PayableQueryPort、PayableStatementQueryPort | PayableLedgerQuery、SupplierStatementQuery | C-15 |
| InvoiceStatusPort | SalesInvoiceQuery 与 InvoiceReversalStatusQuery | C-16 |
| PurchaseRequisitionDerivationPort | PurchaseRequisitionIntakePort | C-17 |
| StockInboundPort、StockOutboundPort、StockAvailabilityQueryPort | InventoryPostingPort 三方法与 AvailabilityQueryPort | C-18 |
| ProjectTaskDerivationPort | ContractDerivationPlanQuery 加事件消费 | C-19 |
| finance::ReceivablePlanPort | clm.contract_payment_schedules 与 ContractPaymentScheduleQuery | C-20 |
| ep_contract_crm::CustomerPanelProvider 与 /overview | Customer360SectionProvider 与 /customer-360 | C-09 |
| ep-contract-service::EquipmentQuery | 撤销，无替代 | B-06 |
| PurchaseReceiptPostingPort、PurchaseReturnPostingPort | 撤销，取价一律归 inventory | C-13 |
| procure.supplier_risk_records | mdm.supplier_risk_records | C-10 |
| procure.goods_receipt_line_costings 的单价列 | InventoryPricingLookupPort::priced_segments_by_source_line | C-12 |
| finance.aging_bucket_definitions | reporting.aging_bucket_profiles 与 aging_bucket_lines | C-08 |
| MdmTaxRateStub | 撤销，税率最小行与 TaxRateOptionQuery 由 T0 建立 | C-11 |
| platform_authz.sensitive_field_registry | platform_core.sensitive_field_registry | C-06 |
| sensitive_field_registry 的 approved_by 与 approved_at | release_ref | C-06 |
| recon_check_definitions 的 statement_sha256 与 signed_statement_ref | 撤销，改记制品版本号与制品签名摘要 | A-06 |
| ReconCategory 的 CROSS_MODULE_LINK 与 FIN_CROSS_MODULE_LINK | 撤销，跨 schema 单目标引用改建真实外键 | A-06 |
| config_item_apply_logs | config_release_steps | A-27 |
| 发布状态 PendingReview | 不存在，差异审查由 diff 端点承载 | A-27 |
| ep_platform_release::MigrationWindowGuard | ep_foundation::port::db::MigrationWindowGuard | B-03 |
| ledger.general_vouchers | 全库无同名对象，删除 | B-02 |
| append_only_registry 的 immutable_columns | mutable_columns | B-02 |
| inventory.stock_movements 作为 MaterialUsageProbe 取数 | inventory.stock_qty_entries | A-13 |
| B001、B002、B003 三个引导文件名 | db/bootstrap 的五个文件名 | C-01 |
| ep-migrate 的 migrate、verify、manifest 三个子命令 | apply、check、status --format=manifest | C-02 |
| <USECASE_SCREAMING>_DOMAIN 与 <USECASE_SCREAMING>_ACTION 常量对 | 路由注册处的 (CapabilityDomain, ActionClass) 元组 | A-20 |

## A 类 有人需要但无人提供

### A-01 契约层可用的不透明事务句柄与工作单元

结论：在 ep-foundation 中冻结 Tx、SnapshotCtx、UnitOfWork 三者，ep-adapter-db-pg 提供唯一实现，契约层的跨模块方法签名一律写 `&mut dyn Tx`。

最终归属阶段：阶段 1。

确切标识符。新增文件 `crates/foundation/src/port/tx.rs`，内容固定如下，任何阶段不得改动签名。

```rust
pub type BoxFuture<'a, T> = core::pin::Pin<Box<dyn core::future::Future<Output = T> + Send + 'a>>;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct TxId(pub uuid::Uuid);

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum IsolationKind { ReadCommitted, RepeatableReadSnapshot }

pub trait Tx: Send {
    fn tx_id(&self) -> TxId;
    fn isolation(&self) -> IsolationKind;
    fn legal_entity_id(&self) -> Id<LegalEntity>;
    fn as_any_mut(&mut self) -> &mut (dyn core::any::Any + Send);
}

pub trait SnapshotCtx: Sync {
    fn snapshot_id(&self) -> &str;
    fn taken_at(&self) -> chrono::DateTime<chrono::Utc>;
    fn legal_entity_id(&self) -> Id<LegalEntity>;
    fn as_any(&self) -> &(dyn core::any::Any + Sync);
}

#[async_trait::async_trait]
pub trait UnitOfWork: Send + Sync + 'static {
    async fn transact<T, F>(&self, ctx: &SecurityContext, body: F) -> Result<T, AppError>
    where
        T: Send + 'static,
        F: for<'t> FnOnce(&'t mut dyn Tx) -> BoxFuture<'t, Result<T, AppError>> + Send + 'static;

    async fn snapshot_transact<T, F>(&self, ctx: &SecurityContext, body: F) -> Result<T, AppError>
    where
        T: Send + 'static,
        F: for<'s> FnOnce(&'s dyn SnapshotCtx) -> BoxFuture<'s, Result<T, AppError>> + Send + 'static;
}
```

跨 crate 取具体句柄的唯一写法是 `tx.as_any_mut().downcast_mut::<PgTx>()`，该 downcast 只允许出现在 `crates/adapter/db-pg/` 内，由 `xtask archcheck` 断言其他目录不出现 `downcast_mut::<PgTx>`。

UnitOfWork 不带池参数，一个 UnitOfWork 实例在装配时绑定一个池，与基线第 10.3 节示例 `uow.transact(ctx, |tx| ...)` 的两参数形态一致。application crate 对 UnitOfWork 取泛型参数 `U: UnitOfWork` 而不是 trait 对象，理由是该 trait 含泛型方法不满足对象安全。

配套裁定：ep-foundation 新增 `crates/foundation/src/id/marker.rs`，集中声明跨模块被引用实体的零大小标记类型，清单固定为 22 项，任何阶段不得增删：LegalEntity、UserAccount、Session、Department、Position、Project、Customer、Supplier、Material、Product、Warehouse、Contract、ContractLine、SalesOrder、SalesOrderLine、DeliveryConfirmation、DeliveryConfirmationLine、PurchaseOrder、GoodsReceiptLine、PurchaseInvoice、PurchaseInvoiceLine、AccountingPeriod。这是对基线第 1.3 节“禁止 foundation 承载业务概念”的一处受限例外，标记类型无字段、无方法、无 trait 实现，只承载类型身份，供 `Id<T>` 在契约层表达跨模块引用。该例外的机检承接方为 `xtask archcheck` 的 foundation-frozen-items 规则，该规则按名逐项断言这 22 项标记类型并断言其单元结构体形态，缺名、多名、改名与形态不符一律报错。

提供方要做什么：阶段 1 在 ep-foundation 增加 `port::tx`、`id::marker` 与 `port::db` 三个模块，在 ep-adapter-db-pg 声明并实现 `PgUnitOfWork` 与 `PgTx`。写入阶段 1 计划第 5.1 节 foundation 核心类型表与第 7 节并发与事务边界。

每个使用方要改什么。阶段 7 计划第 1094 行的假设 A2 末句改为“总账与库存的契约端口按 A-01 接受 `&mut dyn Tx`，已由阶段 1 提供”。阶段 8 计划第 6.1 节把“接受调用方传入的事务句柄”写实为 `&mut dyn Tx`，第 11.1 节 R2 删除“尚未由总账阶段确认”一句。阶段 9 计划第 9.5.9 节末句“事务句柄类型取自 ep-foundation，见 needs”改为“事务句柄为 `ep_foundation::port::Tx`，快照上下文为 `ep_foundation::port::SnapshotCtx`”。阶段 13 计划第 4.6 节 ConfigItemApplier 的 `tx: &mut dyn Tx` 保持不变，改为在文首注明该类型来自 ep-foundation。

顺序约束：无倒挂，阶段 1 本就在最前。本条是阶段 7、8、9、13 开工的硬前提，必须在阶段 1 计划定稿前完成。

### A-02 SYSTEM_PRINCIPAL_ID 的固定取值

结论：冻结一个保留 UUID 作为系统主体，另冻结系统设备标识，两者写入 ep-foundation 常量表与基线第 4 节。

最终归属阶段：阶段 1。

确切标识符。`crates/foundation/src/principal.rs`：

```rust
pub const SYSTEM_PRINCIPAL_ID: uuid::Uuid =
    uuid::uuid!("00000000-0000-7000-8000-000000000001");
pub const SYSTEM_DEVICE_ID: &str = "SYSTEM";
```

取值选用全零前缀加版本位 7 与变体位 8 的保留形态，理由是它符合 UUIDv7 的版本与变体校验，同时不可能与 IdGen 生成的任何值碰撞。

提供方要做什么：阶段 1 在 ep-foundation 定义两个常量，并在 `SecurityContext::system()` 构造函数中固定使用它们；写入阶段 1 计划第 5.1 节。同时回写基线第 4 节公共列表 created_by 一行的语义列，把“固定的系统主体 ID”改为该字面量。

每个使用方要改什么。阶段 4 计划第 219 行改为“第 10 号迁移写入 `00000000-0000-7000-8000-000000000001` 的系统主体账号行”，删去“取值由阶段 1 提供，写入 needs”。阶段 9 计划第 779 行“actor 取系统主体 ID”改为“actor 取 `foundation::SYSTEM_PRINCIPAL_ID`”。阶段 2、3、5、8、10、11、12、13、14 凡在种子迁移或系统上下文写 created_by 的，一律引用该常量，不得再写 `'00000000-0000-0000-0000-000000000000'` 一类的自选值。

顺序约束：无倒挂。属阶段 1 定稿前必须关闭的阻塞项。

### A-03 SecurityContext 的完整字段集合

结论：按阶段 4 计划第 4.1 节的结构体加两个追踪字段与一个系统用途字段一次性冻结，共 20 个字段，构造入口只有两个；本条经开发就绪冻结补充后取下列现行值，旧 19 字段与三参数 `system` 签名作废。

最终归属阶段：阶段 1。

确切标识符。`crates/foundation/src/security/context.rs`，字段顺序即下表顺序，不得增删改名。

| 序 | 字段 | 类型 |
|---|---|---|
| 1 | user_id | Id\<UserAccount\> |
| 2 | account_kind | AccountKind |
| 3 | session_id | Id\<Session\> |
| 4 | legal_entity_id | Id\<LegalEntity\> |
| 5 | device_id | DeviceId |
| 6 | client | ClientKind |
| 7 | clearance_level | SecurityLevel |
| 8 | roles | Arc\<[RoleCode]\> |
| 9 | duty_classes | Arc\<[DutyClass]\> |
| 10 | department_scope | DepartmentScope |
| 11 | position_ids | Arc\<[Id\<Position\>]\> |
| 12 | project_scope | Arc\<[Id\<Project\>]\> |
| 13 | customer_scope | Arc\<[Id\<Customer\>]\> |
| 14 | record_shares | Arc\<[RecordShare]\> |
| 15 | data_scope_tags | Arc\<[DataScopeTag]\> |
| 16 | snapshot_version | u64 |
| 17 | is_breakglass | bool |
| 18 | request_id | RequestId |
| 19 | trace_id | TraceId |
| 20 | system_purpose | Option\<SystemPurpose\> |

配套枚举同在 ep-foundation 冻结：`AccountKind { Human, System, Portal }`；`ClientKind { Win, Mac, Ios, Android, Portal, Ops }`，序列化取值与基线第 5.6 节 X-Client 头一一对应；`DepartmentScope { All, Subtree(Id<Department>), Explicit(Arc<[Id<Department>]>) }`；`SystemPurpose { General, Reconciliation }`。构造函数只有 `SecurityContext::human(..)` 与 `SecurityContext::system(legal_entity_id, request_id, trace_id, purpose)` 两个：前者固定 `system_purpose=None`，后者用 A-02 的两个常量填 user_id 与 device_id、account_kind 取 System，并固定 `system_purpose=Some(purpose)`。`SystemPurpose::Reconciliation` 除枚举定义处外只允许在 `crates/platform/recon/src/executor.rs` 出现，由 `reconciliation-context-confined` archcheck 断言；普通系统任务传 `General`，不定义第三种 `ReconContext`。不提供任何 with_ 前缀的变换方法。

第 18 与第 19 两个字段是本裁定的追加项，理由是基线第 3.8 节要求连接取用时写入 `app.request_id` 与 `app.trace_id` 两条会话变量，取数只能来自安全上下文。

提供方要做什么：阶段 1 在 ep-foundation 实现该结构体、四个配套枚举与基线第 1.4 节冻结的七个字段类型即 `DeviceId`、`RoleCode`、`DutyClass`、`RecordShare` 与 `RecordShareGrant`、`DataScopeTag`、`RequestId`、`TraceId`，写入阶段 1 计划第 5.1 节，替换现有的八字段简表；阶段 1 退出条件的验收面为“20 个字段、四个配套枚举与七个字段类型”，并交付 `reconciliation-context-confined` 负例门禁。七个类型的名字、形态与取值域以基线第 1.4 节为唯一出处，本表不复述。

每个使用方要改什么。阶段 4 计划第 4.1 节删去“由阶段 1 提供其骨架，本阶段补齐字段集合，见 needs”，改为“字段集合由阶段 1 按 A-03 冻结，本阶段只负责填充”。阶段 4 计划第 811 行的阻塞判定删除。阶段 5、6、11 凡引用 SecurityContext 字段的措辞一律按上表字段名书写。

> **历史待决，已被 F-51 取代。** U-B-07 已关闭：`RecordShare { object_type, object_id, grant: RecordShareGrant }`，`RecordShareGrant { Read, Write }`；`can_reshare` 首版恒为 false 且不进结构。不得实现旧的两字段临时形态，未来变更只走正式规格流程。

顺序约束：无倒挂。属阶段 1 定稿前必须关闭的阻塞项。

### A-04 集团、组织、部门、岗位四类表与部门层级闭包

结论：四张表加一张闭包表全部落在 platform_core，由 ep-platform-tenancy 承载读取契约，阶段 2 交付。

最终归属阶段：阶段 2。

确切标识符。五张表与关键列如下，公共列按基线第 4 节。

| 表 | 关键列 |
|---|---|
| platform_core.enterprise_groups | code、name、is_active、deactivated_at；不带 legal_entity_id，属全局配置字典类，不建策略 |
| platform_core.organizations | legal_entity_id、code、name、org_kind（CORPORATION、BRANCH、DIVISION）、parent_organization_id、is_active |
| platform_core.departments | legal_entity_id、organization_id、code、name、parent_department_id、level_no smallint、is_active、deactivated_at |
| platform_core.positions | legal_entity_id、department_id、code、name、rank_no smallint、is_active、deactivated_at |
| platform_core.department_closures | legal_entity_id、ancestor_department_id、descendant_department_id、depth smallint；唯一约束 ux_department_closures_pair 在 (ancestor_department_id, descendant_department_id) |

契约 trait 落在 ep-platform-tenancy：

```rust
#[async_trait::async_trait]
pub trait LegalEntityDirectory: Send + Sync {
    async fn list_active(&self) -> Result<Vec<LegalEntityRef>, AppError>;
    async fn get(&self, id: Id<LegalEntity>) -> Result<LegalEntityRef, AppError>;
}
pub struct LegalEntityRef { pub id: Id<LegalEntity>, pub code: String,
                            pub entity_no: String, pub name: String, pub is_active: bool }

#[async_trait::async_trait]
pub trait DepartmentClosureQuery: Send + Sync {
    async fn descendant_ids(&self, tx: &mut dyn Tx, legal_entity_id: Id<LegalEntity>,
                            department_id: Id<Department>, max_depth: u8)
        -> Result<Vec<Id<Department>>, AppError>;
}
```

闭包表在部门新增、改父、停用的同一事务内全量重写该子树，不用递归 CTE 做在线查询，理由是基线第 3.10 节要求附录 A.1 度量查询不得出现顺序扫描。

提供方要做什么：阶段 2 在 `db/migrations/platform_core/` 追加五个迁移文件，文件名 slug 依次为 `platform_core_enterprise_groups`、`platform_core_organizations`、`platform_core_departments`、`platform_core_positions`、`platform_core_department_closures`，排在既有第 9 号 grants 文件之前；在 ep-platform-tenancy 交付两个 trait 与其 pg 实现。写入阶段 2 计划第 3.4 节迁移编号表与第 3.5 节表定义。

每个使用方要改什么。阶段 4 计划第 150 行把 department_id 与 position_id 的外键目标写死为 `platform_core.departments(id)` 与 `platform_core.positions(id)`，删去“表名以租户阶段实际交付名为准”。阶段 4 计划第 215 行的顺序说明保留。阶段 4 计划第 317 行的部门闭包编译改为经 `DepartmentClosureQuery::descendant_ids`。阶段 5 凡引用组织架构的措辞改为引用上表五个表名。阶段 3 计划第 3.13 节依赖三改为“`ep-platform-tenancy::LegalEntityDirectory::list_active`”。

顺序约束：无倒挂，阶段 2 在阶段 4 之前。

### A-05 ep-platform-license 模块许可与生命周期状态机

结论：本体前移到阶段 3b，不放在阶段 13b。理由是阶段 5 已经把“模块许可中 inventory 已启用而探针未注册即拒绝启动”写进自己的启动自检，阶段 3b 的定时器与 Outbox 投递又要按模块开关过滤，许可若晚于阶段 5 交付，这两处只能长期挂桩。停用再启用的端到端验收顺延到阶段 13b，本体不顺延。

最终归属阶段：阶段 3b 交付本体，阶段 13b 只补一条停用再启用的验收用例。

确切标识符。三张表落在 platform_core，均属全局配置字典类，不带 legal_entity_id，不建策略。

| 表 | 关键列 |
|---|---|
| platform_core.module_registrations | module_code text 唯一、display_name、install_state text CHECK in NOT_INSTALLED, INSTALLED_ENABLED, INSTALLED_DISABLED、installed_at、state_changed_at |
| platform_core.license_grants | license_no text 唯一、issued_to、valid_from date、valid_to date、named_user_limit int、module_codes text[]、signature bytea、revoked_at |
| platform_core.feature_flags | feature_code text 唯一、module_code、is_enabled bool、requires_license bool |

契约 trait 落在 ep-platform-license：

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

`ModuleCode` 是 ep-foundation 中按基线第 1.2 节 15 个模块码冻结的枚举，取值为 Mdm、Crm、Cpq、Clm、Sales、Procure、Inventory、Costing、Project、Service、Finance、Ledger、Invoice、Portal、Reporting，由阶段 1 交付。

提供方要做什么：阶段 3b 在 `db/migrations/platform_core/` 追加三个迁移，在 ep-platform-license 实现状态机与 trait，并把基线第 7.3 节的自检项 `license-and-modules-consistent` 从 Pending 换成实现。写入阶段 3 计划第 3.1 节交付物清单作为第 21 项，并写入第 3.2 节 crate 表，同时在阶段 3 计划头部的“本阶段不建设……许可”一句中删去许可二字。阶段 3 第 3.1 节四项追加物的编号固定为：第 18 项全文检索（A-07）、第 19 项 ConfigItemApplier 端口（A-19 与 A-27，属 3a 段）、第 20 项最小配置发布通道（A-27，属 3b 段）、第 21 项模块许可本体（本条，属 3b 段）。四项不得压缩到 18 至 20 三个编号内，原回写清单的“编号至第 20 项”是差一错误。

每个使用方要改什么。阶段 1 计划第 5 节把自检项 `license-and-modules-consistent` 标为 Pending 并注明由阶段 3b 替换。阶段 3 计划第 3.13 节依赖八整条删除。阶段 4、5 凡引用模块许可的措辞改为引用 `ModuleLicenseQuery`。阶段 5 计划第 340 行的启动自检增补项改为读取 `ModuleLicenseQuery::module_state`。阶段 13 计划删去许可本体的交付，只保留一条验收：某模块停用后其定时任务停止、对外事件停发、再启用后恢复。

顺序约束：3b 在 5 之前，链上无倒挂。总览第 4.1 节 A-05 行的“阶段 13b”须改为“阶段 3b”。

### A-06 ep-platform-recon 对账框架本体与执行器

结论：本体归阶段 9a，注册方固定为阶段 7、8、9b、11 四个，在其之后或按反向依赖接入。阶段 10 曾列为第五个注册方，其唯一一项 `FIN_CROSS_MODULE_LINK` 是纯存在性项，跨 schema 单目标引用改建复合真实外键后整条删除，阶段 10 自注册方清单退出。阶段 14 只调用 `ReconExecutor::run`，不注册任何 `ReconCheck`；阶段 13 不以引用存在性注册 `ReconCheck`，从注册方清单中删除。原裁定所称的六个注册方作废，本条是该清单的唯一出处，其他文件一律引用不复述。总览 R14 不得再设与本条并列的注册义务；阶段 5、6、8、12 的固定单目标跨模块引用均由同法人真实外键兜底，application 层经对方模块契约承担可引用状态与业务范围校验，二者都不进入 ReconCheck 清单。阶段 3b 的附件孤儿收敛任务不算对账，改写措辞，不使用该框架。

最终归属阶段：阶段 9a。

确切标识符。三张表，前一张属全局配置字典类不带法人，后两张带法人并按基线第 3.8 节模板建策略。

| 表 | 关键列 |
|---|---|
| platform_core.recon_check_definitions | check_code text 唯一、category text CHECK in SUBLEDGER_VS_LEDGER, INVARIANT、module_code text、is_blocking_period_close bool、registered_by_module text |
| platform_core.recon_runs | legal_entity_id、run_kind text CHECK in DAILY, PERIOD_CLOSE, RECOVERY_ACCEPTANCE、accounting_period_id、snapshot_id text、status text CHECK in RUNNING, COMPLETED, UNFINISHED, FAILED、batch_total int、batch_done int、started_at、finished_at；仅追加表 |
| platform_core.recon_discrepancies | legal_entity_id、recon_run_id、check_code、accounting_period_id、subject_ref jsonb、expected_amount numeric(18,2)、actual_amount numeric(18,2)、difference_amount numeric(18,2)、state text CHECK in OPEN, REPAIRING, REPAIRED, WAIVED、repaired_by、approval_ref；可更新表，带 row_version |

契约 trait 落在 ep-platform-recon：

```rust
#[async_trait::async_trait]
pub trait ReconCheck: Send + Sync {
    fn code(&self) -> &'static str;
    fn category(&self) -> ReconCategory;
    fn blocks_period_close(&self) -> bool;
    async fn run_batch(&self, snapshot: &dyn SnapshotCtx, legal_entity_id: Id<LegalEntity>,
                       accounting_period_id: Id<AccountingPeriod>, batch: BatchWindow)
        -> Result<Vec<ReconDiscrepancy>, AppError>;
}

pub struct ReconRegistry;
impl ReconRegistry { pub fn register(&mut self, check: std::sync::Arc<dyn ReconCheck>); }

#[async_trait::async_trait]
pub trait ReconExecutor: Send + Sync {
    async fn run(&self, run_kind: ReconRunKind, legal_entity_id: Id<LegalEntity>,
                 accounting_period_id: Id<AccountingPeriod>) -> Result<ReconRunOutcome, AppError>;
}

pub struct BatchWindow { pub batch_no: u32, pub batch_size: u32, pub offset: u64 }
pub struct ReconRunOutcome { pub run_id: Id<ReconRun>, pub status: ReconRunStatus,
                             pub discrepancy_count: u32, pub unfinished_check_codes: Vec<String> }
```

执行器按基线第 3.8 节逐法人遍历，只在单一法人上设置 `app.legal_entity_id`，快照由 `UnitOfWork::snapshot_transact` 导出并逐批传递。

提供方要做什么：阶段 9a 交付 ep-platform-recon crate、三张表的迁移（放在 `db/migrations/platform_core/`）、job-worker 内的执行器与每日调度、签名语句集校验。写入阶段 9 计划第 9.1 节交付物、第 9.3 节数据库变更与第 9.4 节关账前强制校验。

每个使用方要改什么。阶段 7 计划第 942 行与退出条件第 9 条保留登记语句的措辞，删去“既有的 REPEATABLE READ 快照”一语中的既有二字，改为“由 ep-platform-recon 提供的快照”。阶段 8 计划 D7 与退出条件第 12 条改为实现 `ReconCheck` 并在 wiring 注册。阶段 9 计划第 384 行删去“由 recon 提供”的转述，改为本阶段提供。阶段 11 计划第 60 与 577 行改为实现三个 `ReconCheck`。阶段 10 计划删去“注册对账取数语句集”与其后一切注册措辞，本阶段不实现也不注册任何 `ReconCheck`：`FIN_CROSS_MODULE_LINK` 与其 `category` 取值 `CROSS_MODULE_LINK` 一并撤销，引用存在性由复合真实外键强制，期间一致由 `AccountingPeriodResolver::resolve` 的同事务记忆化保证。阶段 9 计划第 20、44、942 三行的注册方名单一律改为阶段 7、8、9b、10、11，删去其中的阶段 6、13、14 与“均早于 9b”一语，9b 的四个校验项在本阶段内注册。四个注册方的校验项数固定为阶段 7 六个、阶段 8 两个、阶段 9b 四个、阶段 11 三个，合计十五个，全部经 `ReconRegistry::register` 在 job-worker 的 wiring 中注册。阶段 13 计划不出现 `ReconCheck`、`ReconRegistry` 与 ep-platform-recon，本条对阶段 13 无落点；阶段 14 计划只保留 `ReconExecutor::run(ReconRunKind::RecoveryAcceptance)` 的调用，不出现注册措辞。阶段 3 计划第 3.0 节判定三与第 3.9 节把附件孤儿收敛改称“job-worker 内的幂等收敛任务”，明确不产生对账差异事项、不依赖 ep-platform-recon。

顺序约束：9a 在 8、6、7、10、11、9b、13、14 之前，无倒挂。阶段 7 与阶段 8 在 9a 之后，因此不存在“只登记不执行”的过渡期，总览第 4.1 节 A-06 行末句“阶段 7 与阶段 8 在 9a 之前只登记语句不执行”删除。

### A-07 ep-adapter-search 全文检索写入与查询

结论：端口进 ep-foundation，适配实现归阶段 3b，阶段 5 起可用。

最终归属阶段：阶段 3b。

确切标识符。`crates/foundation/src/port/search.rs`：

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

索引按法人分区，物理路径 `/var/lib/ep/search/<legal_entity_id>/`。写入一律经 job-worker 消费 Outbox 事件触发，不在业务事务内调用，理由是基线第 10.3 节禁止事务内做文件正文读写。

提供方要做什么：阶段 1 在 `crates/foundation/src/port/` 下建空文件 `search.rs` 并只写模块注释；阶段 3b 补齐上述类型与 trait，交付 ep-adapter-search 实现与 job-worker 内的索引消费者。写入阶段 3 计划第 3.1 节交付物清单，作为第 18 项。

每个使用方要改什么。阶段 5 计划第 61 行改为“按 `SearchDocument` 结构定义四类档案与价目表的投影函数，写入方为 job-worker 的索引消费者”。阶段 7、12、13 凡提到检索文档投影的措辞一律改为“产出 `foundation::port::search::SearchDocument`”。

顺序约束：3b 在 5 之前，无倒挂。

### A-08 ep-adapter-doc Excel、文档模板、PDF 与打印排版

结论：端口进 ep-foundation 由阶段 5 补齐，适配实现归阶段 5，阶段 6 与阶段 11 在其上增量，不另起接口。

最终归属阶段：阶段 5。

确切标识符。`crates/foundation/src/port/doc.rs`：

```rust
pub struct SheetSpec { pub sheet_name: String, pub columns: Vec<ColumnSpec>, pub rows: Vec<Vec<CellValue>> }
pub struct ColumnSpec { pub field_code: String, pub header: String, pub required: bool, pub width: u16 }
pub enum CellValue { Text(String), Number(rust_decimal::Decimal), Date(chrono::NaiveDate), Empty }

#[async_trait::async_trait]
pub trait SpreadsheetPort: Send + Sync {
    async fn write_xlsx(&self, sheets: Vec<SheetSpec>, out_path: &std::path::Path) -> Result<(), AppError>;
    async fn read_xlsx(&self, in_path: &std::path::Path, expect: &[ColumnSpec])
        -> Result<Vec<Vec<CellValue>>, AppError>;
}
#[async_trait::async_trait]
pub trait DocTemplatePort: Send + Sync {
    async fn render(&self, template_ref: &str, model: serde_json::Value,
                    out_path: &std::path::Path) -> Result<(), AppError>;
}
#[async_trait::async_trait]
pub trait PdfRenderPort: Send + Sync {
    async fn render_pdf(&self, source: PdfSource, layout: PrintLayout,
                        out_path: &std::path::Path) -> Result<(), AppError>;
}
pub struct PrintLayout { pub page_size: String, pub margins_mm: [f32; 4],
                         pub offset_mm: [f32; 2], pub dpi: u16 }
```

提供方要做什么：阶段 1 建空文件 `crates/foundation/src/port/doc.rs`；阶段 5 补齐上述类型与三个 trait，交付 ep-adapter-doc 的实现，覆盖导入模板生成、错误行清单渲染、XLSX 读写三项。写入阶段 5 计划第 2 节 crate 表与第 4 节导入导出算法。

每个使用方要改什么。阶段 6 计划第 64 行改为“经 `DocTemplatePort::render` 与 `PdfRenderPort::render_pdf`，不新增接口”。阶段 10 的发票打印与阶段 11 计划第 85 行改为“在阶段 5 交付的三个 trait 上增量实现像素级套打的 `PrintLayout` 取值，不新增 trait”。阶段 13 的打印模板配置对象只产出 `PrintLayout` 取值，不自建渲染路径。

顺序约束：5 在 6、10、11、13 之前，无倒挂。总览第 4.1 节 A-08 行的“阶段 11 的措辞是引用其既有能力”一句现已成立。

### A-09 交付确认单主体

结论：交付确认单归 sales 模块，因此归阶段 6，不归阶段 8。总览第 4.1 节把表建在 sales schema 却让阶段 8 的 ep-app-inventory 去写，直接违反基线第 1.3 节最后一条“一个仓储只访问自己模块的 schema”，且 sales.delivery_schedules 与 sales.return_line_delivery_links 两张已在阶段 6 的表都需要指向交付确认单的同 schema 真实外键，按基线第 3.3 节这两处外键不能是逻辑引用。阶段 6 在调整后的顺序中排在阶段 8 与阶段 9a 之后，库存腿与凭证腿的端口在阶段 6 开工时均已存在，不构成倒挂。交付确认的功能定义出自规格第 8 章第 8 步与第 5.2 章事件-分录表，规格已把确认时点、直运分支、收入确认与销货成本结转、应收账款未开票过渡科目四项写死；PRD 附录乙 U-C-01 问的是该功能由 PRD 第 3 节还是第 5 节承载，属 PRD 内部的编排问题，不是技术取值待决项，按权威顺序规格高于 PRD，本条据规格直接落地，不受 U-C-01 阻塞。U-C-01 的承载节由产品负责人决定，本表不代拍，阶段 6 计划在未决事项表中登记该条及其切换代价，即改判由 PRD 第 5 节承载时两张表跨 schema 迁移、两处真实外键改由目标建成后的追补迁移继续保持同法人真实外键、事件 aggregate_type 与 payload 改名、类型码 DC 改登记模块、阶段 8 与 9a 与 10 三条腿调用方反转，属高代价。同批仍属 PRD 待决且本条不代拍的是 U-C-02 交付确认的操作者角色，阶段 6 只冻结能力常量不预置角色绑定。

最终归属阶段：阶段 6 建表、建用例、发事件；阶段 8 提供库存腿端口；阶段 9a 提供收入与成本腿端口；阶段 10 提供过渡科目腿端口并反向替换阶段 6 的空实现。

确切标识符。两张表落在 sales schema。

`sales.delivery_confirmations`，单据类，类型码 DC。列除公共列外为：`doc_no text not null`；`status text not null CHECK in ('DRAFT','CONFIRMED')`；`customer_id uuid not null`；`sales_order_id uuid not null`（同 schema 外键）；`posting_date date not null`；`warehouse_id uuid`；`is_drop_ship boolean not null default false`；`confirmed_at timestamptz`；`confirmed_by uuid`；`voucher_id uuid`（与法人组成真实复合外键指向 `ledger.vouchers(legal_entity_id,id)`，确认时同事务回填）；`remark text`。约束 `ux_delivery_confirmations_legal_entity_id_doc_no`；索引 `ix_delivery_confirmations_legal_entity_id_created_at`、`ix_delivery_confirmations_sales_order_id`、`ix_delivery_confirmations_legal_entity_id_posting_date`。不设作废态，冲正一律经销售退货单，理由是基线第 3.6 节禁止软删除且已过账分录只追加。本表不带 accounting_period_id，与阶段 6 计划第 11.2 节的偏离登记一致。

`sales.delivery_confirmation_lines`。列除公共列外为：`delivery_confirmation_id uuid not null`（同 schema 外键）；`line_no int not null`；`sales_order_line_id uuid not null`（同 schema 外键）；`delivery_schedule_id uuid not null`（同 schema 外键）；`item_kind text`；`item_id uuid not null`；`uom_code text not null`；`quantity numeric(18,6) not null`；`net_unit_price numeric(18,6) not null`；`tax_rate numeric(9,6) not null default 0`；`line_amount numeric(18,2) not null`；`line_amount_with_tax numeric(18,2) not null`；`warehouse_id uuid`；`batch_no text not null default '-'`；`serial_nos text[] not null default '{}'`；`cogs_amount numeric(18,2)`（确认时由库存腿回填）；`stock_movement_id uuid`（与法人组成真实复合外键指向 `inventory.stock_movements(legal_entity_id,id)`）。约束 `ux_delivery_confirmation_lines_confirmation_id_line_no`；索引 `ix_delivery_confirmation_lines_sales_order_line_id`。

用例名与端点：`crates/application/sales/src/usecase/create_delivery_confirmation.rs` 对应 `POST /api/v1/sales/delivery-confirmations`；`crates/application/sales/src/usecase/confirm_delivery.rs` 对应 `POST /api/v1/sales/delivery-confirmations/{id}/actions/confirm-delivery`；另有 `GET /api/v1/sales/delivery-confirmations` 与 `GET /api/v1/sales/delivery-confirmations/{id}`。

事件名：`sales.delivery.confirmed.v1`，aggregate_type 取 `sales.delivery_confirmations`，与基线第 6.1 节示例逐字一致。payload 字段固定为：`delivery_confirmation_id`、`doc_no`、`sales_order_id`、`customer_id`、`contract_id`、`is_drop_ship`、`voucher_id`、`lines`，其中 `lines` 每元素含 `delivery_confirmation_line_id`、`sales_order_line_id`、`delivery_schedule_id`、`item_kind`、`item_id`、`quantity`、`warehouse_id`、`batch_no`、`serial_nos`、`revenue_amount`、`cogs_amount`。信封的 `posting_date` 取单据 posting_date，`accounting_period_id` 取 PostingPort 返回值。

三腿的实现方与调用形态，全部在 confirm_delivery 的同一个事务内，现行次序固定为库存腿、凭证腿、过渡科目腿；旧的「过渡先于凭证」被 F-51 后续收口取代。

| 腿 | 实现阶段 | 调用 |
|---|---|---|
| 库存腿 | 阶段 8 | `ep_contract_inventory::InventoryPostingPort::post_outbound(tx, ctx, OutboundPosting { reason: MovementReason::DeliveryConfirmation, pricing: OutboundPricing::MovingAverage, source: SourceRef{ doc_type: DELIVERY_CONFIRMATION, .. }, lines })`，返回每行 cogs_amount 与 stock_movement_id；`is_drop_ship` 为真时整段跳过 |
| 过渡科目腿 | 阶段 10 | `record_on_delivery(tx,ctx,DeliveryUnbilledArCommand { delivery_confirmation_id,customer_id,posting_date,accounting_period_id,accounting_period_seq,deferred_from_period_id,voucher_id,direction:DEBIT,net_amount,gross_amount })`；完整 ResolvedPeriod 与 voucher_id 来自前两步，gross 取交付行价税合计，一次插入 APPEND_ONLY 行 |
| 收入与成本腿 | 阶段 9a | `ep_contract_ledger::PostingPort::post(tx, ctx, PostingInput { source_kind: VoucherSourceKind::DELIVERY_CONFIRMED, branch: DROP_SHIP 或 NON_DROP_SHIP, posting_date, source_document, measures })`，measures 含 revenue_amount、unbilled_receivable_amount、cogs_amount、inventory_release_amount 四项 |

会计期间由 `ep_contract_ledger::AccountingPeriodResolver::resolve` 在事务最前解析一次，库存腿与过渡科目腿复用其返回值。

提供方要做什么：阶段 6 在 `db/migrations/sales/` 追加两个迁移文件，slug 为 `sales_create_delivery_confirmations` 与 `sales_create_delivery_confirmation_lines`，排在 `sales.delivery_schedules` 之后、`sales.return_line_delivery_links` 之前；把 `sales.return_line_delivery_links` 的 `delivery_confirmation_id` 与 `delivery_confirmation_line_id` 由逻辑引用改为同 schema 真实外键 ON DELETE RESTRICT；`clm.contract_milestones.delivery_confirmation_id` 在 sales 目标建成后由 `V20261017093700__clm_add_cross_schema_foreign_keys.sql` 追补同法人复合真实外键。逐次交付只由 `sales.delivery_confirmation_lines.delivery_schedule_id` 的真实外键表达，`sales.delivery_schedules` 不再保存会被后一次交付覆盖的单值确认引用。写入阶段 6 计划第 3 节数据库变更、第 4 节算法、第 5 节 API 契约、第 8 节测试与第 9 节退出条件。

每个使用方要改什么。阶段 6 计划第 61 行的“交付确认回写”消费者保留，消费者名固定为 `sales.delivery_writeback`，消费自身发出的 `sales.delivery.confirmed.v1`。阶段 6 计划第 772 行的风险条整条删除。阶段 8 计划第 0 节与第 10.1 节保留“交付确认事件的库存侧算法”，删去任何暗示本阶段建单据的措辞，并在第 11.3 节明确 `SourceDocType::DELIVERY_CONFIRMATION` 由 sales 传入。阶段 10 计划第 815 行的 UnbilledArPort 使用方由“ep-app-sales、ep-app-inventory”收窄为“ep-app-sales”，并新增一条说明：阶段 6 先注入 `NoopUnbilledArPort`，阶段 10 替换。阶段 11 的成本下钻按 `sales.delivery_confirmation_lines` 取数，经 `costing.cost_entries` 的 `source_document_id` 与 `source_document_line_id` 跳转原单据；交付指标不直接读该基表，实际交付日期经受治理数据集 `clm_contract_delivery_milestones` 与 `sales_order_delivery_batches` 上的交付确认引用与确认日期列取得，理由是阶段 11 的 D-11-01 禁止分析 SQL 出现来源模块基表名，且 A-18 未为交付确认单登记数据集。阶段 12 的 `delivery_confirmation_id` 与 `delivery_confirmation_line_id` 使用带头行归属的同法人真实复合外键指向 `sales.delivery_confirmations` 与 `sales.delivery_confirmation_lines`。

顺序约束：阶段 6 排在阶段 8 与阶段 9a 之后，即 5 → 9a → 8 → 6，本条不产生倒挂。唯一的反向依赖是过渡科目腿由阶段 10 回头替换，阶段 6 的 E2E-6-09 与 E2E-6-10 中过渡科目净额的断言顺延到 M7。

### A-10 进项发票台账与采购发票登记用例

结论：进项发票台账归 invoice 模块，因此归阶段 10，与基线第 1.2 节“invoice 覆盖销项与进项发票台账”一致。阶段 7 不建表、不写台账。

最终归属阶段：阶段 10。

确切标识符。两张表落在 invoice schema。

`invoice.purchase_invoices`，单据类，类型码 PINV。列除公共列外为：`doc_no text not null`；`status text not null CHECK in ('REGISTERED','REVERSED')`；`supplier_id uuid not null`；`purchase_order_id uuid`（同法人复合真实外键指向 procure）；`invoice_no text not null`（供应商发票号）；`invoice_date date not null`；`posting_date date not null`；`accounting_period_id uuid not null`；`deferred_from_period_id uuid`；`tax_rate numeric(9,6) not null`；`net_amount numeric(18,2) not null`；`tax_amount numeric(18,2) not null`；`gross_amount numeric(18,2) not null`；`cost_kind text not null CHECK in ('INVENTORY_TYPE','DIRECT_EXPENSE_TYPE')`；`is_credit_note boolean not null default false`；`reversed_by_id uuid`；`voucher_id uuid`。约束 `ux_purchase_invoices_legal_entity_id_doc_no`、`ux_purchase_invoices_legal_entity_id_supplier_id_invoice_no`；索引 `ix_purchase_invoices_legal_entity_id_created_at`、`ix_purchase_invoices_legal_entity_id_purchase_order_id`、`ix_purchase_invoices_legal_entity_id_posting_date`。

`invoice.purchase_invoice_lines`。列除公共列外为：`purchase_invoice_id uuid not null`（同 schema 外键）；`line_no int not null`；`purchase_order_line_id uuid`（同法人复合真实外键指向 procure）；`goods_receipt_line_id uuid`（同法人复合真实外键指向 procure）；`material_id uuid`；`quantity numeric(18,6) not null`；`net_unit_price numeric(18,6) not null`；`net_amount numeric(18,2) not null`；`tax_amount numeric(18,2) not null`；`accrual_reversal_amount numeric(18,2)`；`price_variance_amount numeric(18,2)`；`is_overbilling boolean not null default false`。约束 `ux_purchase_invoice_lines_invoice_id_line_no`；索引 `ix_purchase_invoice_lines_legal_entity_id_goods_receipt_line_id`。

用例名：`crates/application/invoice/src/usecase/register_purchase_invoice.rs`，端点 `POST /api/v1/invoice/purchase-invoices`，另有 `GET /api/v1/invoice/purchase-invoices` 与 `/{id}`。三单匹配在该用例内执行，依次比对采购订单行、收货行与本次发票行的数量与金额。暂估回冲先经 `ep_contract_procure::GrniEffectWritebackPort::decrease_for_purchase_invoice` 在同一事务写 procure 的 `PURCHASE_INVOICE/DECREASE` 追加效果并取得服务端计算金额；发票净额与该金额之差再经 `ep_contract_inventory::InventoryVariancePort::split_variance(tx, ctx, VarianceSplitCommand{..})` 拆为尚有库存与已出库两部分。本阶段不自行取价，也不得从价差结果反推暂估回冲。应付腿经本模块自身的 `register_payable_on_purchase_invoice` 用例写入。

事件名：`invoice.purchase_invoice.registered.v1`，aggregate_type 取 `invoice.purchase_invoices`，payload 含 `purchase_invoice_id`、`doc_no`、`supplier_id`、`purchase_order_id`、`cost_kind`、`net_amount`、`tax_amount`、`gross_amount`、`accrual_reversal_amount`、`price_variance_in_stock_amount`、`price_variance_released_amount`、`voucher_id`、`lines`。原有的 `invoice.purchase_invoice.reversed.v1` 保持不变。

提供方要做什么：阶段 10 在 `db/migrations/invoice/` 追加两个迁移文件，slug 为 `invoice_create_purchase_invoices` 与 `invoice_create_purchase_invoice_lines`，排在 `invoice.invoice_reversals` 之后；在 ep-platform-sequence 追加类型码 PINV；在事件目录与错误码表登记增量。写入阶段 10 计划第 3.1 节 invoice schema 表定义、第 4 节算法、第 5 节 API 契约、第 6 节并发场景与第 9 节退出条件。阶段 10 计划第 725 行的只读投影端点由“取数经 ep-contract-procure”改为“取数为本模块自有表”。

每个使用方要改什么。阶段 7 计划第 3 行“本阶段不实现采购发票登记……只按契约衔接”保留并加一句“进项发票台账两张表由阶段 10 在 invoice schema 建立”。阶段 7 的 `source_purchase_invoice_line_id`、`purchase_invoice_line_id` 与 `accepted_purchase_invoice_id` 分别在 invoice 目标建成后的 Stage10 追补迁移中建立到 `invoice.purchase_invoice_lines`、`invoice.purchase_invoices` 的同法人真实外键，追补前相关写入口不启用。阶段 8 的价差拆分入口保持 `InventoryVariancePort`，调用方由“采购或发票模块”收窄为 `ep-app-invoice`。阶段 11 计划第 3.5 节数据集 `procure_purchase_invoices` 的 source_view 由 `procure.v_purchase_invoices_dataset` 改为 `invoice.v_purchase_invoices_dataset`，dataset code 改为 `invoice_purchase_invoices`，提供方由采购阶段改为阶段 10，见 A-18。

顺序约束：阶段 7 在阶段 10 之前，因此所有依赖进项发票目标的写入口保持未注册；不得注入空实现、固定成功或固定失败替身。阶段 10 同批交付真实查询实现、追补外键、注册入口并完成验收，不保留顺延项。

### A-11 进项红字发票登记端口与收货发票匹配查询端口

结论：与 A-10 同批交付，两个端口都落在 ep-contract-invoice。

最终归属阶段：阶段 10。

确切标识符。

```rust
// crates/contract/invoice/src/port/purchase.rs
pub struct ReceiptInvoiceMatchState {
    pub goods_receipt_line_id: Id<GoodsReceiptLine>,
    pub billed_returnable_quantity: Quantity,
    pub unbilled_returnable_quantity: Quantity,
    pub billed_returnable_net_amount: Money,
    pub purchase_invoice_line_ids: Vec<Id<PurchaseInvoiceLine>>, // posting_date,id 升序
}

#[async_trait::async_trait]
pub trait ReceiptInvoiceMatchQueryPort: Send + Sync {
    async fn match_state(&self, tx: &mut dyn Tx, ctx: &SecurityContext,
                         goods_receipt_line_id: Id<GoodsReceiptLine>)
        -> Result<ReceiptInvoiceMatchState, AppError>;
    async fn match_states(&self, tx: &mut dyn Tx, ctx: &SecurityContext,
                          goods_receipt_line_ids: &[Id<GoodsReceiptLine>])
        -> Result<Vec<ReceiptInvoiceMatchState>, AppError>;
}

```

**F-50 后续替代（现行）。** 本节原有的 `RegisterPurchaseCreditNote`、`PurchaseCreditNoteLine`、`PurchaseCreditNoteView` 与 `PurchaseCreditNotePort` Rust 代码块已经作废并从裁定册删除，禁止复制实现。采购红字唯一精确契约以 F-50 与阶段 10 现行正文为准：内部命令使用可空 `linked_purchase_return_id`，包含 F-50 冲销行输入、服务器计算的含税效果与过账/并发字段；返回逐项及汇总 GRNI reopen 效果。公开 HTTP、插件与 Excel 不接受该内部链接字段。

提供方要做什么：阶段 10 在 ep-contract-invoice 定义上面的 `ReceiptInvoiceMatchQueryPort` 及其 DTO，并按 F-50/阶段 10 的唯一精确契约定义 `PurchaseCreditNotePort`，在 ep-app-invoice 实现并在两个 wiring 目录注入。写入阶段 10 计划模块内契约表，追加两行。

每个使用方要改什么。阶段 7 的调用保持 `ReceiptInvoiceMatchQueryPort::match_state`，签名按上表补全参数；返回的是锁后仍可退的已开票/未开票数量，不返回头级布尔值。物料退货数量先消费 `unbilled_returnable_quantity`，不足部分才按 `purchase_invoice_line_ids` 的 `posting_date,id` 升序进入已开票段并调用 F-50/阶段 10 的 `PurchaseCreditNotePort::register_credit_note`；同一退货可同时包含两段，采购头不保存 `is_invoice_registered`。红字发票由 invoice 模块登记。阶段 10 真实实现尚未交付时，对应用例、路由与装配项均不注册；`NoopPurchaseCreditNotePort`、`NoopReceiptInvoiceMatchQueryPort`、空成功与占位返回全部禁止。

顺序约束：与 A-10 相同，阶段 7 早于阶段 10，按通则第三条处理。

### A-12 ep-contract-inventory::AvailabilityQueryPort

**F-51/阶段 8 后续替代（现行）。** 本节原有 `AvailabilityQuery`、`AvailabilityView` 与“`reserved_quantity` 恒为零”的代码块已经作废并从裁定册删除，禁止复制实现。唯一现行 DTO、trait、方法、归属、数量、组合锁与 A2 路由以阶段 8 第 5.1、11.2 节及 F-51 U-G-01 为准；可用量必须由阶段 6 的真实 `SalesAwareAvailabilityQuery` 组合结存与 CONFIRMED/RELEASED 未交付销售需求，不能回退为仅结存、零保留量、空 provider 或第二套 SQL。阶段 6/7/8 的接线与顺序均只按各阶段现行计划执行。

### A-13 MaterialUsageProbe 的实现

结论：阶段 8 实现并在 wiring 注入。

最终归属阶段：阶段 8。

确切标识符：`ep_contract_mdm::MaterialUsageProbe::has_stock_movement(&self, ctx: &SecurityContext, material_id: Id<Material>) -> Result<bool, AppError>`，实现类型名固定为 `InventoryMaterialUsageProbe`，位于 `crates/application/inventory/src/probe/material_usage.rs`，取数为 `inventory.stock_qty_entries` 上按 material_id 的数量流水存在性判定，命中索引 `ix_stock_qty_entries_legal_entity_id_material_id`，索引列为 `(legal_entity_id, material_id)`。原裁定写的取数表 `inventory.stock_movements` 不带 material_id 列，物料维度落在其明细表；原裁定写的索引名 `ix_stock_movements_legal_entity_id_material_id` 实际建在 `inventory.stock_qty_entries` 上，违反基线第 3.10 节的 `ix_<table>_<col…>` 规则，基线高于本表，两者一并作废，任何阶段不得再引用，也不得为此在数据字典中登记命名例外。`inventory.stock_value_entries` 中 qty_entry_id 为空的纯金额调整行不参与该判定。

提供方要做什么：阶段 8 增加该文件与 wiring 注入行，写入阶段 8 计划第 1 节交付物 D3 与第 9 节退出条件，新增一条“`InventoryMaterialUsageProbe` 已实现并注入，阶段 5 的启动自检模块项在 inventory 启用时通过”。

每个使用方要改什么。阶段 5 计划第 337 行删去“由 ep-app-inventory 实现”这一转述之外的不确定措辞，改为“实现类型 `InventoryMaterialUsageProbe` 由阶段 8 交付”。

顺序约束：阶段 5 早于阶段 8。阶段 5 交付时按其第 340 行的缺位行为处理，阶段 8 交付后自检项转为强制，阶段 5 的档案停用校验完整性验收顺延到阶段 8。

### A-14 ProductUsageProbe 的实现

结论：阶段 6 实现两份，在 wiring 中取或。

最终归属阶段：阶段 6。

确切标识符：`ep_contract_mdm::ProductUsageProbe::is_referenced_by_effective_sales(&self, ctx: &SecurityContext, product_id: Id<Product>) -> Result<bool, AppError>`。两个实现类型名固定为 `ClmProductUsageProbe`（`crates/application/clm/src/probe/product_usage.rs`，取数为 `clm.contract_lines` 上 item_kind 为 PRODUCT 且所属合同状态为 EFFECTIVE）与 `SalesProductUsageProbe`（`crates/application/sales/src/probe/product_usage.rs`，取数为 `sales.sales_order_lines` 上状态非 CANCELLED）。wiring 中用 `AnyProductUsageProbe(Vec<Arc<dyn ProductUsageProbe>>)` 组合，任一返回 true 即为 true，该组合类型由阶段 5 在 ep-app-mdm 中提供。

提供方要做什么：阶段 6 增加两个文件与 wiring 注入行，写入阶段 6 计划第 2 节 crate 表与第 9 节退出条件。

每个使用方要改什么。阶段 5 计划第 338 行改为“两个实现类型 `ClmProductUsageProbe` 与 `SalesProductUsageProbe` 由阶段 6 交付，组合类型 `AnyProductUsageProbe` 由本阶段提供”。

顺序约束：阶段 5 早于阶段 6，处理同 A-13。

### A-15 MasterReferenceCounter 与两个 TradeHistoryProvider 的实现

结论：按模块拆分到五个阶段，每个阶段实现自己模块的一份，实现类型名统一为 `<模块帕斯卡名>ReferenceCounter` 与 `<模块帕斯卡名>TradeHistoryProvider`。

最终归属阶段：阶段 6、7、8、10、12 各自实现本模块的一份，聚合逻辑归阶段 5。

确切标识符。

```rust
// ep-contract-mdm，由阶段 5 定义
#[async_trait::async_trait]
pub trait MasterReferenceCounter: Send + Sync {
    fn module_code(&self) -> ModuleCode;
    async fn count_open_documents(&self, ctx: &SecurityContext, object_kind: MasterObjectKind,
                                  object_id: uuid::Uuid) -> Result<u64, AppError>;
}
pub enum MasterObjectKind { Customer, Supplier, Material, Product, PriceList }

#[async_trait::async_trait]
pub trait SalesTradeHistoryProvider: Send + Sync {
    fn module_code(&self) -> ModuleCode;
    async fn recent(&self, ctx: &SecurityContext, customer_id: Id<Customer>,
                    item_id: uuid::Uuid, limit: u16) -> Result<Vec<TradeHistoryItem>, AppError>;
}
#[async_trait::async_trait]
pub trait PurchaseTradeHistoryProvider: Send + Sync {
    fn module_code(&self) -> ModuleCode;
    async fn recent(&self, ctx: &SecurityContext, supplier_id: Id<Supplier>,
                    item_id: uuid::Uuid, limit: u16) -> Result<Vec<TradeHistoryItem>, AppError>;
}
```

实现清单固定为下表，任何阶段不得少做也不得代做其他模块的一份。

| 实现类型 | 阶段 | crate | 覆盖单据 |
|---|---|---|---|
| ClmReferenceCounter | 6 | ep-app-clm | 未终态合同 |
| SalesReferenceCounter | 6 | ep-app-sales | 未终态销售订单、销售退货、交付确认 |
| ProcureReferenceCounter | 7 | ep-app-procure | 未终态采购需求、采购订单、收货、采购退货、付款申请 |
| InventoryReferenceCounter | 8 | ep-app-inventory | 非零结存的仓库物料批次组合数 |
| InvoiceReferenceCounter | 10 | ep-app-invoice | 未终态发票申请、未冲销发票 |
| FinanceReferenceCounter | 10 | ep-app-finance | 未核销应收应付、未消费预收预付 |
| ServiceReferenceCounter | 12 | ep-app-service | 未终态工单、投诉 |
| SalesTradeHistoryProviderImpl | 6 | ep-app-sales | 销售订单行与交付确认行 |
| InvoiceSalesTradeHistoryProvider | 10 | ep-app-invoice | 销项发票行 |
| ProcureTradeHistoryProvider | 7 | ep-app-procure | 采购订单行与收货行 |
| InvoicePurchaseTradeHistoryProvider | 10 | ep-app-invoice | 进项发票行 |

提供方要做什么：上表各阶段在自己的 ep-app-<module> 下增加 `src/probe/` 目录中的对应文件，并在两个 wiring 目录注册到阶段 5 提供的注册表 `MasterReferenceCounterRegistry` 与 `TradeHistoryProviderRegistry`。各阶段在自己的第 9 节退出条件中增加一条“本模块的 MasterReferenceCounter 与 TradeHistoryProvider 已实现并注册”。

每个使用方要改什么。阶段 5 计划第 442 与 448 行按上表改写实现方清单，并明确：停用界面展示的计数覆盖模块清单由注册表实时枚举，未注册模块显式列为未覆盖。

顺序约束：阶段 5 最早，其余阶段反向接入，阶段 5 的停用引用计数完整性验收顺延到阶段 12 结束。

### A-16 ep-contract-clm::ContractDerivationPlanQuery

结论：阶段 6 补该查询，阶段 12 消费。

最终归属阶段：阶段 6。

确切标识符。

```rust
// crates/contract/clm/src/port/derivation.rs
pub enum ContractDerivationItemKind { ProjectTask, DeliveryMilestone,
                                      PurchaseRequisitionLine, PaymentScheduleLine }
pub struct ContractDerivationItem {
    pub item_kind: ContractDerivationItemKind,
    pub unique_key: String,
    pub source_contract_line_id: Option<Id<ContractLine>>,
    pub milestone_no: Option<i32>,
    pub name: String,
    pub promised_date: Option<chrono::NaiveDate>,
    pub quantity: Option<Quantity>,
    pub owner_user_id: Option<Id<UserAccount>>,
}
pub struct ContractDerivationPlan {
    pub contract_id: Id<Contract>,
    pub contract_version_no: i32,
    pub derivation_batch_no: i32,
    pub project_group_contract_id: Option<Id<Contract>>,
    pub items: Vec<ContractDerivationItem>,
}

#[async_trait::async_trait]
pub trait ContractDerivationPlanQuery: Send + Sync {
    async fn derivation_plan(&self, tx: &mut dyn Tx, ctx: &SecurityContext,
                             contract_id: Id<Contract>, contract_version_no: i32)
        -> Result<ContractDerivationPlan, AppError>;
}
```

`unique_key` 的取值规则固定为 `<contract_id>:<contract_version_no>:<item_kind>:<source_contract_line_id 或 milestone_no>`，阶段 12 的派生任务以该键做唯一性去重，对应其“派生任务按唯一键不重复”的退出条件。

提供方要做什么：阶段 6 在 ep-contract-clm 增加该文件，在 ep-app-clm 实现，写入阶段 6 计划第 2 节 crate 表（把 `ContractDerivationCallbackPort` 一行改为并列列出 `ContractDerivationPlanQuery`）与第 4.7 节派生算法。

每个使用方要改什么。阶段 12 计划第 420 行与第 825 行 R-02 的措辞改为“接口形状按 A-16 已冻结”，删去“形态变化只改该处”的风险表述。

顺序约束：阶段 6 早于阶段 12，无倒挂。

### A-17 销售退货单创建命令端口与三类终态事件

结论：阶段 6 补一个命令 trait 与三个事件。下列形状已由 F-54 开发就绪复核取代本条早期的三字段返回与“任一状态可取消”描述，是首版现行唯一契约。

最终归属阶段：阶段 6。

确切标识符。

```rust
// crates/contract/sales/src/port/sales_return.rs
pub struct CreateSalesReturn {
    pub customer_id: Id<Customer>,
    pub sales_order_id: Id<SalesOrder>,
    pub return_reason: String,
    pub return_warehouse_id: Option<Id<Warehouse>>,
    pub posting_date: chrono::NaiveDate,
    pub source_ref: Option<SalesReturnSourceRef>,
    pub remark: Option<String>,
    pub allocation_mode: DeliveryAllocationMode,
    pub lines: Vec<CreateSalesReturnLine>,
}
pub struct SalesReturnSourceRef { pub source_module: ModuleCode, pub source_doc_type: String,
                                  pub source_doc_id: uuid::Uuid, pub source_doc_line_id: uuid::Uuid }
pub enum DeliveryAllocationMode { Manual, AutoFifo }
pub struct CreateSalesReturnLine {
    pub sales_order_line_id: Id<SalesOrderLine>,
    pub quantity: Quantity,
    pub batch_no: String,
    pub serial_nos: Vec<String>,
    pub delivery_links: Vec<SalesReturnDeliveryLink>,
}
pub struct SalesReturnDeliveryLink {
    pub delivery_confirmation_line_id: Id<DeliveryConfirmationLine>,
    pub quantity: Quantity,
    pub assigned_by: DeliveryLinkAssignedBy,   // Manual 或 AutoFifo
}
pub struct SalesReturnLineView {
    pub sales_return_line_id: Id<SalesReturnLine>,
    pub sales_order_line_id: Id<SalesOrderLine>,
    pub quantity: Quantity,
    pub delivery_links: Vec<SalesReturnDeliveryLink>,
}
pub struct SalesReturnView {
    pub sales_return_id: Id<SalesReturn>,
    pub doc_no: String,
    pub status: SalesReturnStatus,
    pub lines: Vec<SalesReturnLineView>,
}

#[async_trait::async_trait]
pub trait SalesReturnCommandPort: Send + Sync {
    async fn create_sales_return(&self, tx: &mut dyn Tx, ctx: &SecurityContext,
                                 cmd: CreateSalesReturn) -> Result<SalesReturnView, AppError>;
}
```

`allocation_mode` 的输入形状封闭。`Manual` 要求每个命令行的 `delivery_links` 非空、数量合计精确等于该退货行数量，且输入 `assigned_by` 每项都为 Manual；`AutoFifo` 要求所有命令行的 `delivery_links` 为空，由 sales owner 在同一事务锁定同法人、同销售订单行的可退交付确认行，按 `confirmed_at ASC, delivery_confirmation_line_id UUID bytes ASC` 依次分配，生成的每个 link 持久化 `assigned_by=AutoFifo`。两种模式都由 sales 重验累计可退数量，不接受调用方自报已占用量。`SalesReturnView.lines` 按 sales_return_line_id 升序，行内 delivery_links 按 delivery_confirmation_line_id 升序；创建调用方必须从返回行读取新 id，不得按“最新一行”查询或假设头 id 等于行 id。

`source_ref` 是可空整体，不是四个独立可空字段。阶段 6 的 `sales.sales_returns` 头表追加 `source_module text null`、`source_doc_type text null`、`source_doc_id uuid null`、`source_doc_line_id uuid null`，`ck_sales_returns_source_ref_shape` 强制四列全空或全非空；普通唯一键 `ux_sales_returns_le_source_ref` 建于 `(legal_entity_id,source_module,source_doc_type,source_doc_id,source_doc_line_id)`，利用 PostgreSQL 普通唯一键允许多组全 NULL 的语义，不使用基线禁止的部分唯一索引。同一来源重放时锁定并返回既有完整 `SalesReturnView`；若除 source_ref 外的规范化全量 `CreateSalesReturn` 命令与首次任一字段不一致，返回 `PLATFORM.IDEMPOTENCY.PAYLOAD_MISMATCH`，不得新建第二单。`remark` 清洗后长度不超过 2000；`source_doc_type` 长度不超过 64。该唯一键是售后消费者崩溃重试的业务兜底，不代替 HTTP 四元组幂等存档。

三个事件登记到 `docs/event-catalog.md`：`sales.sales_return.closed.v1`（仅 REGISTERED 迁到 CLOSED，payload 含 sales_return_id、doc_no、sales_order_id、source_ref、closed_at）；`sales.sales_return.cancelled.v1`（仅 DRAFT 或 SUBMITTED 迁到 CANCELLED，payload 另含 cancel_reason）；`sales.sales_return.rejected.v1`（SUBMITTED 因审批驳回退回 DRAFT，payload 另含 reject_reason 与 approval_ref）。REGISTERED 已存在追加型库存与会计事实，不可取消，只能进入 CLOSED；首版无已登记退货冲正入口。既有的 `sales.sales_return.registered.v1` 保持不变。

提供方要做什么：阶段 6 增加该文件与三个事件的写入位，写入阶段 6 计划第 2 节 crate 表、第 4 节退货算法、第 6 节 Outbox 事件表与第 9 节退出条件第 10 条的计数。阶段 6 的事件数固定为 18，构成为原有 14 个、本条新增的三个销售退货终态事件、A-09 迁入的 `sales.delivery.confirmed.v1` 一个；原裁定给的 17 只按本条推算，漏计了 A-09 迁入的这一个，17 与 14 两个数一并作废，阶段 6 计划第 1 节与第 9 节退出条件第 10 条一律写 18。错误码不再给总数：阶段 6 计划没有独立的错误码清单节，31 与 34 两个数都无出处，第 1 节与退出条件第 10 条一律删去错误码数字，改写为“本阶段第 5 节 API 契约表中出现的全部错误码已登记在 `docs/error-codes.md` 并与 `ep-foundation::error::codes` 一致，由 CI 校验”。

每个使用方要改什么。阶段 12 计划第 414 行的状态机守卫改为按上述三个事件名驱动，第 825 行 R-01 的缓解措辞改为“接口按 A-17 已冻结，testkit 的 `SalesReturnPortFake` 按该签名实现”。

顺序约束：阶段 6 早于阶段 12，无倒挂。

### A-18 各模块的受治理数据集视图

结论：外部提供的数据集共 12 个，不是 11 个。总览第 4.1 节的分项相加即为 12，计数须更正。每个视图由拥有其基表的模块所在阶段发布，阶段 11 只负责登记目录与消费。

最终归属阶段：见下表。

确切标识符。视图名、dataset code 与提供阶段固定如下，任何阶段不得改名。

| dataset code | 视图 | 提供阶段 | grain |
|---|---|---|---|
| mdm_customers | mdm.v_customers_dataset | 5 | DOCUMENT |
| mdm_products | mdm.v_products_dataset | 5 | DOCUMENT |
| mdm_materials | mdm.v_materials_dataset | 5 | DOCUMENT |
| ledger_account_period_balances | ledger.v_account_period_balances | 9a | SNAPSHOT |
| inventory_stock_value_entries | inventory.v_stock_value_entries | 8 | ENTRY |
| clm_contracts | clm.v_contracts_dataset | 6 | DOCUMENT |
| clm_contract_delivery_milestones | clm.v_contract_delivery_milestones | 6 | DOCUMENT_LINE |
| sales_sales_orders | sales.v_sales_orders_dataset | 6 | DOCUMENT |
| sales_order_delivery_batches | sales.v_order_delivery_batches | 6 | DOCUMENT_LINE |
| invoice_purchase_invoices | invoice.v_purchase_invoices_dataset | 10 | DOCUMENT |
| finance_receivable_ledger_entries | finance.v_receivable_ledger_entries | 10 | ENTRY |
| finance_payable_ledger_entries | finance.v_payable_ledger_entries | 10 | ENTRY |
| project_projects | project.v_projects_dataset | 12 | DOCUMENT |

每个视图必须包含 `legal_entity_id`、`security_level`、`data_scope_tags` 三列，并在同一迁移中执行 `GRANT SELECT ON <视图> TO ep_analyst_ro`，不授予 ep_app_rw 之外的任何写权限。视图的列名与类型签名必须与 `reporting.dataset_fields` 的登记一致，由阶段 11 的启动自检项 `reporting-dataset-signature-matched` 校验。

提供方要做什么：阶段 5、6、8、9a、10、12 各自在本模块迁移目录追加一个 迁移目录中各模块对应的 `*_create_dataset_views` PLANNED 路径 文件，并在本阶段退出条件中增加一条“本模块数据集视图已发布并授予 ep_analyst_ro，列签名已同步给阶段 11”。

每个使用方要改什么。阶段 11 计划第 3.5 节按上表改写种子表，把 `procure_purchase_invoices` 与 `procure.v_purchase_invoices_dataset` 整行替换为 `invoice_purchase_invoices` 与 `invoice.v_purchase_invoices_dataset`，提供方由采购阶段改为阶段 10。阶段 7 不再承担任何数据集视图，其反向依赖行中的“11 补采购发票数据集”删除。

顺序约束：阶段 12 晚于阶段 11，`project.v_projects_dataset` 与 `project_projects` 的目录行由阶段 11 先播种、阶段 12 后建视图，阶段 11 的启动自检项对尚未建立的视图按“已登记但未发布”降级放行，该降级只允许存在于阶段 12 结束前，阶段 12 结束后自检项转为强制。

### A-19 ConfigItemApplier 的九个 item_kind 实现

结论：trait 与注册表提前到阶段 3a，发布通道本体在阶段 3b，九个 applier 分派到阶段 3b、4、11。这里与总览的差别是 trait 提前到 3a 而不是 3b，理由是调整后的顺序为 3a → 4 → 3b，若 trait 落在 3b，阶段 4 的三个 AUTHZ_ applier 仍然倒挂。

最终归属阶段：见下表。

确切标识符。`crates/platform/release/src/port/config_item.rs` 由阶段 3a 交付，内容为 A-19 所需的 trait、`ItemKind` 枚举 15 项、`ConfigPackageItem` DTO 与 `ConfigItemApplierRegistry`，trait 方法签名与阶段 13 计划第 4.6 节所列逐字一致，其中 `Tx` 取自 ep-foundation。

| item_kind | 实现阶段 | 实现类型与位置 |
|---|---|---|
| FLOW_DEFINITION | 3b | ep-platform-flow 的 `FlowDefinitionApplier` |
| NOTIFY_RULE | 3b | ep-platform-notify 的 `NotifyRuleApplier` |
| AUTHZ_ROLE | 4 | ep-platform-authz 的 `AuthzRoleApplier` |
| AUTHZ_POLICY | 4 | ep-platform-authz 的 `AuthzPolicyApplier` |
| AUTHZ_FIELD_GRANT | 4 | ep-platform-authz 的 `AuthzFieldGrantApplier` |
| REPORT_DEFINITION | 11 | ep-app-reporting 的 `ReportDefinitionApplier` |
| METRIC_DEFINITION | 11 | ep-app-reporting 的 `MetricDefinitionApplier` |
| DASHBOARD_DEFINITION | 11 | ep-app-reporting 的 `DashboardDefinitionApplier` |
| PRINT_TEMPLATE | 11 | ep-app-reporting 的 `PrintTemplateApplier` |
| CUSTOM_OBJECT、CUSTOM_FIELD、CUSTOM_RELATION、CUSTOM_INDEX、CUSTOM_VIEW、UI_LAYOUT | 13b | ep-platform-meta 的六个 applier |

提供方要做什么：阶段 3a 只交付端口文件与注册表，无表、无用例、不依赖授权，因此不破坏 3a 与 4 的拆环。写入阶段 3 计划第 3.1 节交付物清单并明确标为 3a 段。阶段 3b、4、11、13b 各自在本阶段交付物清单与退出条件中列出自己的 applier 数量。

每个使用方要改什么。阶段 13 计划第 4.6 节的第一句改为“本端口由阶段 3a 提供，本阶段实现其中六个 applier”，第 532 行的九个 applier 归属按上表写死。阶段 4 与阶段 11 在自己的交付物清单中新增 applier 一项。

顺序约束：3a → 4 → 3b → 11 → 13b，全部实现方都晚于 trait 定义方，倒挂解除。

### A-20 能力域码与动作类别的提前冻结

结论：两个枚举提前到 ep-foundation，由阶段 1 交付并回写基线第 12 节，各阶段按本条确切标识符一段给出的两类落点，在承载该路由处理器的 crate 的 `src/capability.rs` 中为每个用例声明两个常量。

最终归属阶段：阶段 1 定义枚举并回写基线第 12 节，阶段 2、3b、4 与阶段 5 至 14 共十三个阶段各自在本条使用方清单为该阶段指名的落点声明常量，阶段 13 除按本条在 `crates/platform/meta/src/capability.rs` 与 `crates/platform/release/src/capability.rs` 声明自身两段路由的常量外，只做运行期判定，不重新定义能力域码。

确切标识符。`crates/foundation/src/capability.rs`：

```rust
pub enum CapabilityDomain {
    CrmCustomer360, ClmContractEsign, SalesOrderFulfillment, ProcureSupplierCollab,
    InventoryLedgerScan, ServiceWorkorderEquipment, PlatformApprovalNotify,
    ProjectTaskMilestone, MdmMasterData, PlatformFullTextSearch, LedgerPostingClose,
    FinanceSettlementView, InvoiceApplyIssue, ReportingReportPrint,
    PlatformDocumentAttachment, PlatformAdminLowcodeOps, PlatformExtensionDynamicCode,
    PortalSupplierWeb,
}
pub enum ActionClass { Read, Write, Submit, Approve, Export }
```

`CapabilityDomain` 的序列化取值逐一为阶段 13 计划第 4.4 节表中的 18 个能力域码字符串，顺序与该表序号一致。`ActionClass` 五项与阶段 13 第 4.4 节判定算法第 3 条的 ViewOnly 分支配套，ViewOnly 只放行 Read。

声明形态固定为：在承载该路由处理器的 crate 的 `src/capability.rs` 中，为每个用例声明一对常量，命名为 `<USECASE_SCREAMING>_DOMAIN` 与 `<USECASE_SCREAMING>_ACTION`，例如 `CONFIRM_DELIVERY_DOMAIN: CapabilityDomain = CapabilityDomain::SalesOrderFulfillment` 与 `CONFIRM_DELIVERY_ACTION: ActionClass = ActionClass::Submit`。落点只有两类，不设第三类：业务模块的路由落 `crates/contract/<module>/src/capability.rs`；`/api/v1/platform/` 下的平台路由落本条使用方清单中为该阶段指名的 platform crate 的 `src/capability.rs`，其能力域一律取 `CapabilityDomain::PlatformAdminLowcodeOps`。`ci-probe` feature 门控的探针路由与 `/internal/v1/` 下不对四端暴露的内部端点不参与判定，不声明常量。`xtask configdoc` 断言每个 `/api/v1/` 路由都能解析到一对常量，缺失即构建失败。

提供方要做什么：阶段 1 交付两个枚举并回写基线第 12 节，把该节的落地纪律改写为本条确切标识符一段给出的两类落点，即业务模块的路由落 `crates/contract/<module>/src/capability.rs`、`/api/v1/platform/` 下的平台路由落本条为该阶段指名的 platform crate 的 `src/capability.rs` 并一律取 `CapabilityDomain::PlatformAdminLowcodeOps`，同时把断言范围写死为每个 `/api/v1/` 路由，`ci-probe` feature 门控的探针路由与 `/internal/v1/` 下不对四端暴露的内部端点不参与判定、不声明常量。写入阶段 1 计划第 5.1 节与第 13 节，阶段 1 计划第 161 行与第 600 行新增决定九末句一并按两类落点与 `/api/v1/` 断言范围改写。各阶段计划中残留的“断言每个 HTTP 路由”与“断言每条 HTTP 路由”与“每个路由”措辞一律改为“断言每个 `/api/v1/` 路由”，使同一个工具只有一个断言范围。

每个使用方要改什么。使用方清单为阶段 2、3b、4、5、6、7、8、9、10、11、12、13、14 十三个，凡交付 `/api/v1/` 路由的阶段都在其内，原清单漏了阶段 2、13、14 三个。各阶段在退出条件中增加一条“本阶段全部路由的能力域码与动作类别常量已声明，`xtask configdoc` 通过”，并在该条中写明常量所在文件。平台路由的承载 crate 逐阶段指名如下：阶段 2 的九个平台路由落 `crates/platform/tenancy/src/capability.rs`，阶段 3b 落 `crates/platform/flow/src/capability.rs`，阶段 4 落 `crates/platform/authz/src/capability.rs`，阶段 13 落 `crates/platform/meta/src/capability.rs` 与 `crates/platform/release/src/capability.rs`，阶段 14 落 `crates/platform/obs/src/capability.rs`。阶段 6 的常量落 `crates/contract/clm/src/capability.rs`、`crates/contract/sales/src/capability.rs` 与阶段 5 已建的 `crates/contract/cpq/src/capability.rs` 三处，对 cpq 只追加不重定义，`/api/v1/cpq/price-authorities` 的能力域取 `CapabilityDomain::SalesOrderFulfillment`，阶段 6 的退出条件在原十四条上按各条裁定合计增为十九条，条数以阶段 6 计划第 9 节的实际编号为准。阶段 13 计划第 4.4 节第 1 条改为“常量由各业务阶段按 A-20 声明，本阶段只做判定”，第 469 行的能力域码表改为引用 `foundation::CapabilityDomain`，不再在阶段 13 内重新定义。

顺序约束：枚举在阶段 1，全部声明方在其后，倒挂解除。

### A-21 事件类型登记到 ledger.posting_trigger_event_types

结论：登记表、登记接口与全部 13 行登记行均归阶段 9a，业务阶段不再追加任何回填迁移。库存阶段登记零行，须显式写明。

最终归属阶段：阶段 9a，登记表、登记接口与 13 行登记行全部在内。

确切标识符。登记行全部由阶段 9a 的种子迁移一次写入，业务阶段不再追加任何回填迁移。`ledger.posting_trigger_event_types` 的终态为 13 行：`ledger_event_kind` 取阶段 9 计划第 9.4.1 节 `VoucherSourceKind` 的 11 个取值各一行，其中 INVOICE_REVERSED 与 REFUND_REGISTERED 各再加一行，合计 13 行；12 行的 `event_type` 取下表事件名，YEAR_END_PL_CLOSING 一行的 `event_type` 为空。`registered_by_module` 取下表阶段所对应的模块码。下表的阶段列表示该事件由哪个阶段产生，不表示由哪个阶段写登记行。

| 阶段 | event_type | ledger_event_kind |
|---|---|---|
| 6 | sales.delivery.confirmed.v1 | DELIVERY_CONFIRMED |
| 6 | sales.sales_return.registered.v1 | SALES_RETURN |
| 7 | procure.goods_receipt.posted.v1 | PURCHASE_RECEIPT |
| 7 | procure.purchase_return.posted.v1 | PURCHASE_RETURN |
| 10 | invoice.sales_invoice.issued.v1 | SALES_INVOICE_ISSUED |
| 10 | invoice.purchase_invoice.registered.v1 | PURCHASE_INVOICE |
| 10 | invoice.sales_invoice.reversed.v1 | INVOICE_REVERSED |
| 10 | invoice.purchase_invoice.reversed.v1 | INVOICE_REVERSED |
| 10 | finance.receipt.registered.v1 | RECEIPT_REGISTERED |
| 10 | finance.payment.registered.v1 | PAYMENT_REGISTERED |
| 10 | finance.refund.registered.v1 | REFUND_REGISTERED |
| 10 | finance.cash_document.reversed.v1 | REFUND_REGISTERED |
| 9b | 无 event_type，YEAR_END_PL_CLOSING 行由阶段 9a 的 backfill 保留 event_type 为空 | YEAR_END_PL_CLOSING |
| 8 | 零行 | 不适用 |

`ux_posting_trigger_event_types_event_type` 是唯一约束，`event_type` 为空的行不参与唯一性判定，因此一个 `ledger_event_kind` 可以有多行，INVOICE_REVERSED 与 REFUND_REGISTERED 各两行的 `event_type` 不同，不冲突。原裁定的“只 UPDATE 不新增行”与本表在算术上不可同时成立：种子只有 11 行而需要承载 12 个 event_type，该措辞作废，阶段 9 与阶段 10 计划中互斥的两套写法一并按本条统一。

提供方要做什么：阶段 9a 保留第 12 号建表迁移；第 14 号迁移 `V20261015091500__ledger_backfill_posting_trigger_event_types.sql` 由“按十一类凭证来源各写一行且 event_type 留空”改为一次写全上表的 13 行并直接填入 `event_type` 与 `registered_by_module`，事件名逐字照抄上表，阶段 9a 不需要知道各业务模块的实现；该迁移的回退为按 `ledger_event_kind` 与 `event_type` 删除本次插入的 13 行。阶段 9a 另交付 `ep_contract_ledger::PostingTriggerRegistry::assert_registered(snapshot: &dyn SnapshotCtx, event_type: &str, kind: VoucherSourceKind, module: ModuleCode) -> Result<(), AppError>`，语义为只读断言：按 `event_type` 查种子行，缺行、`ledger_event_kind` 不符或 `registered_by_module` 不符一律返回 `AppError`，错误码取 `LEDGER.POSTING_TRIGGER_EVENT_TYPE.REGISTRY_MISMATCH`，分类 `BUSINESS_CONFLICT`、HTTP 409、不可重试，登记在阶段 9 计划的错误码表；该方法不写任何行，只供运行期启动自检比对，不供迁移调用，失败时进程以退出码 78 退出、不经 HTTP 返回。原写的幂等 upsert 语义与本条 13 行终态互斥，作废。写入阶段 9 计划第 9.3.11 节。

每个使用方要改什么。阶段 6、7、10 一律不新增 `backfill_posting_trigger_event_types` 迁移。阶段 7 计划第 3.3 节的第 24 号 `WITHDRAWN__procure_backfill_posting_trigger_event_types.sql` 撤销，该编号由 B-02 追加的 `V20261018092300__procure_backfill_append_only_registry.sql` 占用，其后编号不变；本条裁定批次当时的阶段 7 迁移文件总数仍为三十一，即三十个建表文件加第 24 号登记回填文件。后续 F-50/F-51 又新增第 31 张表及 portal 目标建成后的真实外键追补，现行阶段 7 计数已经冻结为 33；再晚建的 invoice/project 外键追补分别计入阶段 10/12，精确版本与阶段归属只以 `docs/migration-catalog.md` 为准，不得继续把本段历史“三十一”当作施工计数。第 1005 行的退出条件在本条裁定批次保留三十一的计数，删去“且 `ledger.posting_trigger_event_types` 的两行 event_type 已置回空”半句，回退断言改为 `platform_core.append_only_registry` 中无本阶段残留登记行。阶段 10 计划删去 invoice 目录第 16 号 `WITHDRAWN__invoice_backfill_posting_trigger_event_types.sql` 与 finance 目录第 24 号 `WITHDRAWN__finance_backfill_posting_trigger_event_types.sql`，两个目录其后文件的编号与版本号一律不变，第 609 与 611 两行的写入与回退措辞整段删除，第 917 行之后的对照表保留为与种子行比对的清单。阶段 6 不新增该类文件，只在事件一节写一句“本阶段两个事件的登记行由阶段 9a 的种子迁移写入，本阶段只做运行期比对”。三个阶段改为在启动自检中经 `PostingTriggerRegistry::assert_registered` 对本模块事件做只读断言比对，缺行或 `ledger_event_kind` 或 `registered_by_module` 不符即以退出码 78 启动失败，全部“幂等 upsert”措辞一律删除。阶段 8 在第 6.4 节明确写一句“本阶段不向 ledger.posting_trigger_event_types 登记任何行，库存事件不独立产生凭证”。

顺序约束：登记行归阶段 9a 的 `db/migrations/ledger/` 种子迁移，按通则第五条不产生跨 schema 迁移，空库上按文件版本号全序执行成立。原方案把三个回填迁移放在 procure、invoice、finance 三个目录，而这三个回填文件的版本号都早于 `ledger.posting_trigger_event_types` 的建表迁移，空库上必然在该表建立之前执行并报 relation does not exist，整批迁移中断，该方案作废。

### A-22 处置流程对 DisposalPort 的实现

结论：实现归阶段 14，与密钥销毁、备份保留期一并处理。阶段 3b 至阶段 14 之间不注入任何实现，物理删除请求在此期间直接拒绝并开一个 kind 取 `PORT_NOT_IMPLEMENTED`、`subject` 取 `DisposalPort` 的降级窗口，不得静默按成功路径放行。

最终归属阶段：阶段 14。

确切标识符。trait 由阶段 3b 定义在 `crates/platform/file/src/port/disposal.rs`：

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

阶段 14 的实现类型名固定为 `OpsDisposalService`，位于 `crates/platform/obs/src/disposal.rs`，只由 ops 专用路径与专用账号触发，执行前校验双人审批与重新认证凭证，执行后写审计并生成销毁证明。

提供方要做什么：阶段 14 实现该 trait，在 wiring 注入，写入阶段 14 计划第 2 节 crate 表与退出条件。

每个使用方要改什么。阶段 3 计划第 613、870、1351 行的“实现由处置流程所在阶段交付”改为“实现由阶段 14 的 `OpsDisposalService` 交付”。阶段 2 计划第 377 行的“销毁的实际执行属处置流程，本阶段不实现”改为“经 `DisposalPort` 由阶段 14 执行”。阶段 13 计划第 547 与 1061 行的物理删除路径同样指向阶段 14。

顺序约束：阶段 14 在最后，全部使用方先留桩，无倒挂。

### A-23 各业务模块的四端界面

结论：界面下沉到各业务阶段，阶段 13 只保留壳、能力矩阵、白标与制品。

最终归属阶段：阶段 5 至 12 各自实现本模块界面。

确切标识符。界面交付物的命名与位置固定为 `clients/desktop/src/modules/<module>/`、`clients/mobile/src/modules/<module>/`，每个模块一个目录。每个业务阶段的退出条件中新增一条，措辞固定为：本模块在规格第 6.2 章能力矩阵中取值为完整或简化的能力域，其四端界面已实现并通过 Playwright 与 tauri-driver 的桌面用例、XCUITest 与 Espresso 的移动用例；取值为 VIEW_ONLY 的能力域只实现只读视图；取值为 NOT_APPLICABLE 的不实现入口。能力域到模块的映射按 A-20 的 `CapabilityDomain` 枚举。

提供方要做什么：阶段 5 至 12 各自在第 1 节交付物清单增加界面一项，在第 8 节测试计划增加四端 UI 用例，在第 9 节退出条件增加上述一条。阶段 8 计划第 31 行“本阶段不交付任何界面”整句删除并按上述改写。

每个使用方要改什么。阶段 13 计划的范围说明中明确“不交付任何业务界面”，其交付物只有客户端壳、路由注册表、能力矩阵闸、白标构建与四端制品。阶段 14 的验收矩阵按各业务阶段的界面交付情况汇总。

顺序约束：无倒挂。

### A-24 期初与历史数据导入通道

结论：不设独立数据迁移阶段，按模块归属，**四个**通道各自落在已有的**三个**阶段。
（原写「三个通道」，与其下表四行及次段自称「四个通道」不符——把通道数与阶段数混了；裁定 F-42 更正。）

最终归属阶段：阶段 9a、阶段 10、阶段 8。

确切标识符。

| 通道 | 阶段 | 落点 |
|---|---|---|
| 总账期初余额 | 9a | 已有 `ledger.opening_balance_batches` 与 `ledger.opening_balance_batch_lines`，端点 `POST /api/v1/ledger/opening-balance-batches` 与 `/{id}/actions/confirm` |
| 应收应付预收预付期初 | 10 | 新增 `POST /api/v1/finance/opening-balances/actions/import`，请求体 `{ledger_side, accounting_period_id, rows[]}`，rows 写入 `finance.receivable_entries`、`finance.payable_entries`、`finance.advance_receipt_entries`、`finance.advance_payment_entries`，`source_doc_type` 取 `MIGRATION_OPENING` |
| 资金账户期初 | 10 | 已有 `finance.cash_accounts.opening_balance` 与 `opening_balance_period_id`，建档时一次录入，建档后不可修改 |
| 库存期初 | 8 | 已有 `MIGRATION_STOCK_ADJUSTMENT` 来源类型与 `MovementReason::MigrationOpening` |

四个通道的写入一律不生成凭证，期初对应的总账侧由 9a 的期初余额批次承担，两侧的平衡由 `finance.v_recon_*` 十个视图在首个会计期间校验（本句按 F-07 改写，原「八个」作废）。

提供方要做什么：阶段 10 增加一个用例文件 `crates/application/finance/src/usecase/import_opening_balances.rs` 与一个端点，写入阶段 10 计划第 5 节 API 契约与第 9 节退出条件。阶段 8 与阶段 9a 无新增，只在计划中把该通道标注为期初导入的唯一落点。

每个使用方要改什么。阶段 8、9、10 计划中凡出现“数据迁移阶段”的措辞一律删除，改为指向上表。

顺序约束：无倒挂。

### A-25 ep-adapter-esign crate 本体

结论：阶段 6 已在交付物中列出该 crate，本条只需补登记与替换契约测试的说明，真实对接验证在阶段 14。

最终归属阶段：阶段 6。

确切标识符：crate 名 `ep-adapter-esign`，目录 `crates/adapter/esign/`，只依赖 ep-foundation 与 `ep_domain_clm::port::SignatureGateway`，装配进 integration-gateway。产品侧入口只允许 `NT SERVICE\ep-worker` 经 `\\.\pipe\ep-integ` 调用 `esign.request.submit.v1` 与 `esign.status.get.v1`；签章结果文件由 gateway 在同一双工连接按 `esign_file.begin.v1`、`esign_file.chunk.v1`、`esign_file.end.v1`、`esign_file.abort.v1` 反向流回。integration-gateway 不监听内部 TCP，不持数据库、KMS 或业务文件目录权限。契约测试目标名固定为 `crates/adapter/esign/tests/contract_sandbox.rs`，wiremock 打桩目标名为 `crates/adapter/esign/tests/contract_stub.rs`，两套用例共用同一组断言函数。

提供方要做什么：阶段 6 在第 1 节交付物清单中把 crate 名写全，在第 8 节测试计划中把两套契约测试的文件名写死。

每个使用方要改什么。阶段 3 计划第 3.13 节依赖十一改为“`ep-adapter-esign` 由阶段 6 交付”。阶段 14 在其认证清单中增加一条“执行 `contract_sandbox.rs` 对真实沙箱的一次通过记录，或提交规格附录 B 允许的等效验证证据”。

顺序约束：阶段 6 早于阶段 14，无倒挂。

### A-26 platform_ops 最小台账的提前可用

结论：阶段 2 建 platform_ops schema 与 degradation_windows 一张表并提供写入端口；阶段 14 第 3 节承接二十三表五视图，其中十七张是部署级台账，另六张是按法人隔离并 ENABLE、FORCE RLS 的历史数据迁移台账；阶段 13c 另建 `ai_model_packages`，故全仓 platform_ops 终态为二十四表五视图（十八张部署级表加六张法人 RLS 表）。

最终归属阶段：阶段 2。

确切标识符。表 `platform_ops.degradation_windows`，列与阶段 14 计划第 3.1 节表 3 的定义完全一致，不带 legal_entity_id，不建策略，带 `scope_legal_entity_id` 与 `scope_accounting_period_id` 两个可空标注列。阶段 2 只建表并交付两条约束 `ux_degradation_windows_kind_scope_closed` 与 `ck_degradation_windows_open_order`，其余两条 CHECK 与全部索引由阶段 14 追加；前一约束的唯一 SQL 形态是 PostgreSQL 16 的 `UNIQUE NULLS NOT DISTINCT (kind, subject, scope_legal_entity_id, scope_accounting_period_id, closed_at)`，不得用默认 NULL 语义替换。

写入端口落在 ep-platform-obs：

```rust
#[async_trait::async_trait]
pub trait DegradationLedger: Send + Sync {
    async fn open(&self, kind: DegradationKind, scope: DegradationScope, basis: &str)
        -> Result<uuid::Uuid, AppError>;
    async fn close(&self, kind: DegradationKind, scope: DegradationScope)
        -> Result<(), AppError>;
    async fn open_count(&self) -> Result<u64, AppError>;
}
```

`DegradationKind` 的可实施顺序以现有 SQL 与 F-55 终态裁定为准。阶段 2 的 `V20260901104500__platform_ops_create_degradation_windows.sql` 首次建表时，Rust 枚举与数据库 CHECK 都恰含 `OFFSITE_SINK_NOT_CONFIGURED`、`WRITER_NOT_IN_SERVICE`、`PORT_NOT_IMPLEMENTED` 三项，且阶段 2 只为这三项提供触发路径；不得把后续值倒灌进阶段 2。阶段 14 的 `V20261023092500__platform_ops_harden_backup_evidence_graph.sql` 才同批把 Rust 枚举与数据库 CHECK 从三项扩为终态 21 项，其中新增的第二十一项是 `LEGAL_ENTITY_KEY_DOMAIN_UNAVAILABLE`。`WASM_COMPUTE_NOT_DELIVERED`、`RULE_EVALUATOR_NOT_DELIVERED` 与 `DISPOSAL_NOT_DELIVERED` 三个取值按通则第三条撤销：端口在其交付阶段之前一律开 `PORT_NOT_IMPLEMENTED` 的降级窗口，并由 `subject` 列记下该端口名。`WRITER_ROLE_CONTAINMENT_MISSING` 作废，其触发条件由遏制手段配置缺失改为客观事实即任一写出进程未运行或连续两个写出周期无上报；终态不可关闭属性固定为五项，`ck_degradation_windows_not_suppressible` 护住 `OFFSITE_SINK_NOT_CONFIGURED`、`OFFSITE_COPY_PROTECTION_MISSING`、`WRITER_NOT_IN_SERVICE`、`VIRUS_SCANNER_NOT_AVAILABLE` 与 `LEGAL_ENTITY_KEY_DOMAIN_UNAVAILABLE`。其中 `OFFSITE_COPY_PROTECTION_MISSING` 表示 writer 可写但存储端未证明拒绝覆盖、删除、重命名或改权/策略，与落点不可写是两个独立事实。五项均随触发条件消除自动闭合，不允许人工静音。指标 `ep_degradation_windows_open` 由阶段 2 注册并填充。旧的“阶段 2 首次即落完整 20/21 项”“终态 20 项”与“四项不可抑制”均已被本段及 F-55 取代，不可实施。

提供方要做什么：阶段 2 在第 3.4 节迁移表中把第 16 号 `platform_ops_create_schema` 之后追加一个 `V20260901104500__platform_ops_create_degradation_windows.sql`，在 ep-platform-obs 交付上述 trait 与 pg 实现。写入阶段 2 计划第 1 节交付物清单与第 3.5 节表定义。

每个使用方要改什么。阶段 1 计划的自检项 `offsite-sink-requirements` 在失败时调用 `DegradationLedger::open`，但阶段 1 早于阶段 2，因此阶段 1 只写 stderr 并留注释 `// TODO(stage-2): write degradation ledger`，阶段 2 补上。阶段 3 计划第 3.13 节依赖九删去 `platform_ops.degradation_windows` 一项。阶段 4、9、11、13 凡登记降级窗口的措辞改为调用 `DegradationLedger`。阶段 14 计划第 73 节明确本表由阶段 2 建立、本阶段只做扩展。

顺序约束：阶段 2 早于 3、4、9、11、13、14，倒挂解除。阶段 1 是唯一早于阶段 2 的使用方，按上述注释处理。

### A-27 ep-platform-release 配置发布通道的提前可用

结论：端口在阶段 3a，最小发布通道在阶段 3b，低代码全量与自动测试在阶段 13b。阶段 2 不使用该通道。

最终归属阶段：阶段 3a 交付端口，阶段 3b 交付最小通道，阶段 13b 扩展。

确切标识符。阶段 3b 交付的最小通道含三张表与一个状态机，表落在 platform_meta schema：`platform_meta.config_packages`、`platform_meta.config_package_items`、`platform_meta.config_release_orders`，列定义与阶段 13 计划第 3 节所列一致；platform_meta 下其余与配置发布相关的表一律归阶段 13b，本条不再逐张点名。原裁定用的表名 `config_item_apply_logs` 全库没有对应对象，其所指是阶段 13b 的 `platform_meta.config_release_steps`，该旧名作废，三份文件中不得再出现，也不再保留任何括注映射。发布状态机取 PRD 第 10.4.1 节的十一态为唯一出处：阶段 3b 实现其中六态 Draft、PendingApproval、Rejected、Approved、Released、RolledBack，差异审查由 `GET /api/v1/platform/config-packages/{id}/diff` 端点承载，不单列为状态，原裁定写的 PendingReview 在 PRD 中不存在，一并作废；阶段 13b 补齐其余五态 PendingAutotest、TestFailed、TestPassed、SignedPendingRelease、Superseded，六加五合计十一态，扩展只放宽 `ck_config_packages_status`，不改写任何既有行。普通部署签名算法固定为 ECDSA P-256；`item_hash` 的现行唯一算法由 F-56 补齐为 ADD/MODIFY 取 `SHA-256(JCS(after_spec))`、REMOVE 取 `SHA-256(JCS(before_spec))`，两者均 lowerhex，禁止对 null 求摘要；外层 manifest 另签住 item kind/code/change/sort/scope 与该 digest。

提供方要做什么：阶段 3a 只交付 `crates/platform/release/src/port/config_item.rs`（见 A-19）。阶段 3b 交付三张表的迁移、发布与回退用例、`ConfigItemApplierRegistry` 的运行期装配以及两个 applier。写入阶段 3 计划第 3.1 节交付物清单，作为第 19 项与第 20 项。

每个使用方要改什么。阶段 2 计划中凡依赖配置发布通道的措辞改为“阶段 2 不使用发布通道，敏感字段登记与密钥域配置直接经迁移与端点写入，发布通道接入由阶段 3b 反向补齐”。阶段 5、6、7、9、10、11 的配置对象发布一律经阶段 3b 的通道，不自建第二套。阶段 13 计划把三张表标注为阶段 3b 已建、本阶段只做列扩展与状态扩展。

顺序约束：3a → 3b → 5 → …，全部使用方在其后，倒挂解除。

### A-28 字段元数据登记入口

结论：阶段 5 不依赖 platform_meta，改用阶段 2 的 `platform_core.sensitive_field_registry` 与阶段 4 的 `platform_authz.field_permissions` 两处承载。本条按权威顺序把规格的强制项与 PRD 的待决项分开裁定，权威链如下。规格第 7.8 章写明事务数据库使用信封加密、行内敏感字段按法人密钥域与密级使用字段级密钥属于强制项，并写明行内敏感字段由对象密级、字段密级或经产品负责人批准的敏感字段清单确定、至少覆盖身份与联系方式与账户与税号与支付认证令牌和法律或健康等高敏感属性。银行账号即该章明列的账户类属性，且本条已给该列赋 `security_level` 30，字段密级这一条判据本身即已成立，因此银行账号纳入行内敏感字段与对其做字段级加密两件事由规格直接强制，不是待决项，本条按规格取 `is_field_encrypted` 为真。PRD 附录乙 U-A-12 的原文只问三件事，即敏感字段清单是否包含开户银行与银行账号、这两列在列表与详情与导出三种场景的脱敏形态、导出是否触发重新认证，全文不出现加密二字；其中银行账号的纳入已被规格强制，剩余待决的只有开户银行是否同列、三场景脱敏形态、导出是否触发重新认证三问，决策人为安全负责人与产品负责人，本表不代拍，只给临时取值与切换代价。上一轮以 U-A-12 为由把银行账号退回明文，使规格第 7.8 章的强制项在首版落空，按权威顺序规格高于 PRD 也高于本表，该取值撤销；上一轮所称撤销的加密与否一项在 U-A-12 条文中并不存在，该撤销一并作废。任何阶段不得再据 U-A-12 把银行账号退回明文。

最终归属阶段：登记表归阶段 2 与阶段 4，登记行归各引入受保护列的模块阶段，首版共六行，即阶段 3b 一行、阶段 5 四行、阶段 10 一行。

确切标识符。阶段 5 在 `db/migrations/mdm/` 追加一个 `V20261014092500__mdm_backfill_sensitive_field_registry.sql`，向 `platform_core.sensitive_field_registry` 插入四行，不是两行：该表的唯一约束 `ux_sensitive_field_registry_schema_table_column` 落在三列上，两张表乘两列必然是四行。银行字段不在 `mdm.customers` 与 `mdm.suppliers` 上，这两个表名作废，实际落点是 `mdm.customer_invoice_profiles` 与 `mdm.supplier_payment_profiles`。四行逐列取值固定如下，列集按 C-06 冻结的十一列，缺一写不出 INSERT。

| schema_name | table_name | column_name | category | security_level | is_field_encrypted | blind_index | blind_index_column | mask_style | normalization | release_ref |
|---|---|---|---|---|---|---|---|---|---|---|
| mdm | customer_invoice_profiles | bank_name | ACCOUNT | 30 | false | NONE | 空 | NONE | TRIM_NFKC | `MIGRATION:<本迁移版本号>` |
| mdm | customer_invoice_profiles | bank_account_no | ACCOUNT | 30 | true | EXACT | bank_account_no_bidx | KEEP_LAST_4 | TRIM_NFKC | `MIGRATION:<本迁移版本号>` |
| mdm | supplier_payment_profiles | bank_name | ACCOUNT | 30 | false | NONE | 空 | NONE | TRIM_NFKC | `MIGRATION:<本迁移版本号>` |
| mdm | supplier_payment_profiles | bank_account_no | ACCOUNT | 30 | true | EXACT | bank_account_no_bidx | KEEP_LAST_4 | TRIM_NFKC | `MIGRATION:<本迁移版本号>` |

`column_name` 取逻辑列名，不带 `_enc` 后缀。`created_by` 取 `foundation::SYSTEM_PRINCIPAL_ID`。`bank_account_no` 两行的 `is_field_encrypted` 取 true，由规格第 7.8 章强制，不是临时取值；两张表的物理列相应为 `bank_account_no_enc bytea` 可空、`bank_account_no_key_ref text` 可空记录密钥标识与版本、`bank_account_no_tail text` 可空承载掩码保留的后四位、`bank_account_no_bidx bytea` 可空，一律不保留同名明文列 `bank_account_no`。`bank_name` 两行的 `is_field_encrypted` 取 false，物理列为 `bank_name text` 可空，这是 U-A-12 未决期间的临时取值。`db/checks/11` 按 `is_field_encrypted` 分支断言：取真的登记项断言物理表上存在 `<column_name>_enc` 列且类型为 `bytea` 且不存在同名明文列 `<column_name>`；取假的登记项只断言 `<schema_name>.<table_name>.<column_name>` 三元组在 `information_schema.columns` 中命中实际列，不施加 bytea 与 `_enc` 后缀断言。四行中除 `bank_account_no` 两行的存在由规格强制外，其余各行的存在本身与四行的 `security_level`、`mask_style` 均为 U-A-12 未决期间的临时取值，决策人为安全负责人与产品负责人；`category` 取 ACCOUNT、`normalization` 取 TRIM_NFKC、`release_ref` 取迁移版本号三列由本条固定，不属待决。盲索引按 B-04 建立，规格第 7.8 章禁止字段级密文直接用于唯一约束，盲索引是唯一的查重手段。

阶段 3b 的一行与阶段 10 的一行按同一列集给全。阶段 3b 在 `db/migrations/platform_msg/` 追加一个 `V20261013092600__platform_msg_backfill_sensitive_field_registry.sql`，与第 33 号同目录并排在其后，向 `platform_core.sensitive_field_registry` 插入一行，十一列逐列取值为：`schema_name` 取 platform_msg，`table_name` 取 push_registrations，`column_name` 取 token 即逻辑列名不带 `_enc`，`category` 取 PAYMENT_TOKEN，`security_level` 取 30，`is_field_encrypted` 取 true，`blind_index` 取 EXACT，`blind_index_column` 取 token_bidx，`mask_style` 取 FULL，`normalization` 取 NONE，`release_ref` 取 `MIGRATION:<本迁移版本号>`；`created_by` 取 `foundation::SYSTEM_PRINCIPAL_ID`，`-- rollback:` 段按 `schema_name` 与 `table_name` 删该行。`mask_style` 不得取 KEEP_LAST_4，该表没有 `token_tail` 列；`normalization` 不得取 TRIM_NFKC，推送令牌是大小写敏感的不透明串，规范化会改写 `derive_blind_key` 的入参。该行的依据是规格第 7.8 章把支付认证令牌列入行内敏感字段的最低覆盖面，属规格强制，不是临时取值。首版字段级加密列因此共四处而不是三处：阶段 4 计划第 387 行的“三处的 bank_account_no”改为“`mdm.customer_invoice_profiles` 与 `mdm.supplier_payment_profiles` 与 `finance.cash_accounts` 三处的 `bank_account_no`，加 `platform_msg.push_registrations.token`，共四处，前三处 `mask_style` 取 KEEP_LAST_4 由阶段 5 与阶段 10 交付，后一处取 FULL 由阶段 3b 交付”，同句“全库只有这一处解密位点”收窄为“经字段投影返回给用户的解密只有这一处”，推送令牌的解密由 job-worker 在投递链路上直接取用，不经 `FieldProjector`，也不受字段权限判定。阶段 3 计划第 3.9 节增加一条退出条件，即该行存在且 `is_field_encrypted` 为真、`blind_index_column` 为 `token_bidx`、`mask_style` 为 FULL，`db/checks/11` 返回零行。阶段 2 计划第 135 与 823 两行的登记行名单改为“全库共六行，阶段 3b 一行、阶段 5 四行、阶段 10 一行”。

U-A-12 三问的临时取值与切换代价如下，截止点按总览 R12 的 M3 之前关闭 U-A 组，不另设期限。第一问敏感字段清单是否包含开户银行与银行账号：银行账号由规格第 7.8 章强制纳入，不可撤销；开户银行的临时取值为纳入并登记两行，切换代价是删除或改写这两行，属数据行变更，不改代码也不改表。第二问列表与详情与导出三场景的脱敏形态：临时取值为 `bank_account_no` 两行的 `mask_style` 取 `KEEP_LAST_4` 且后四位取自 `bank_account_no_tail`、`bank_name` 两行取 `NONE`，三场景同形态，渲染一律经阶段 4 的 `FieldProjector`，切换代价是改这四行的 `mask_style`，不改代码。第三问导出是否触发重新认证：本表列不承载该判定，统一指向阶段 4 的重新认证判定函数，该函数对这四列判真，判据是列在清单内与密级 30 两条各自独立成立，切换代价限于该函数的判定入参配置，本条不在表列上再给第二套答案。若 U-A-12 决策为开户银行也做字段级加密，切换路径固定为一次变更内同时完成三件事，缺一 `db/checks/11` 必然判负：把 `bank_name` 两行的 `is_field_encrypted` 改为 true，把物理列改为 `bank_name_enc bytea` 并补 `bank_name_key_ref text`，删去同名明文列。阶段 4 的 `platform_authz.field_permissions` 由配置发布通道在阶段 5 之后写入对应的字段级授权行，阶段 5 交付时按默认拒绝处理。

提供方要做什么：阶段 5 追加该迁移文件，写入阶段 5 计划第 3 节迁移编号表与第 9 节退出条件，退出条件写实为“`platform_core.sensitive_field_registry` 中存在 `mdm.customer_invoice_profiles` 与 `mdm.supplier_payment_profiles` 的 `bank_name` 与 `bank_account_no` 共四行，`bank_account_no` 两行的 `is_field_encrypted` 为真、`bank_name` 两行为假，`db/checks/11` 返回零行”。阶段 5 计划第 205 与 209 行的两张 profiles 表列定义改为删去明文列 `bank_account_no`，新增 `bank_account_no_enc bytea`、`bank_account_no_key_ref text` 与 `bank_account_no_tail text`，保留 `bank_account_no_bidx bytea` 与明文列 `bank_name text`；第 900 行的 U-A-12 一行保持待决，待决范围写实为开户银行是否同列、三场景脱敏形态、导出是否触发重新认证三问，并写明银行账号的纳入与字段级加密按规格第 7.8 章强制落地、不在待决范围内，切换代价按本条三问逐问描述。

每个使用方要改什么。阶段 5 计划中凡出现 platform_meta 的依赖措辞一律删除。阶段 2 计划第 135 与 799 行的“插入客户与供应商两行”改为“插入 `mdm.customer_invoice_profiles` 与 `mdm.supplier_payment_profiles` 两表的四行”，“开户银行与银行账号是否纳入敏感字段清单尚待决策”一句收窄为“开户银行是否纳入敏感字段清单尚待决策，银行账号按规格第 7.8 章强制纳入并做字段级加密”；阶段 2 计划第 800 行的“四行 `is_field_encrypted` 在该事项决策前一律取假”改为“`bank_account_no` 两行取真、`bank_name` 两行取假，后者与四行的 `security_level` 与 `mask_style` 同为该事项决策前的临时取值”；阶段 2 计划第 370 行 `db/checks/` 第 11 项的断言文本改为按 `is_field_encrypted` 分支，取真的登记项断言物理表上存在 `<column_name>_enc` 列且类型为 `bytea` 且不存在同名明文列，取假的登记项只断言 `<schema_name>.<table_name>.<column_name>` 三元组在 `information_schema.columns` 中命中实际列。阶段 4 计划第 829 行的同类措辞按上述同步。字段级信封加密的物理列命名在全库只有一套，取 `<语义>_enc bytea` 加 `<语义>_key_ref text`，需要保留掩码尾数的再加 `<语义>_tail text`，需要查重的再加 `<语义>_bidx bytea`；阶段 10 计划第 294 行的 `bank_account_no_cipher` 改名为 `bank_account_no_enc` 并在其后补一行 `bank_account_no_key_ref text` 记录密钥标识与版本，`bank_account_no_tail` 与 `bank_account_no_bidx` 两列不动；阶段 10 计划第 305 行“字段级密级覆盖的登记由 platform_authz 承载”一句删除，改为在 `db/migrations/finance/` 追加一支 backfill 迁移向 `platform_core.sensitive_field_registry` 登记一行，取值为 schema_name 取 finance、table_name 取 cash_accounts、column_name 取 bank_account_no、category 取 ACCOUNT、security_level 取 30、is_field_encrypted 取 true、blind_index 取 EXACT、blind_index_column 取 bank_account_no_bidx、mask_style 取 KEEP_LAST_4、normalization 取 TRIM_NFKC、release_ref 取 `MIGRATION:<本迁移版本号>`，platform_authz 侧只写 `field_permissions` 的字段级授权行、不承载密级；阶段 10 计划第 63 行的 F-17 一行改标为“F-17 与 U-A-12”，待决范围按本条三问写实，不得写成已决，也不得把银行账号的加密写成待决。阶段 3 计划第 609 行的 `token_ciphertext` 改名为 `token_enc` 并补 `token_key_ref text`，全库不得同时存在 `_cipher` 与 `_ciphertext` 与 `_enc` 三套命名。阶段 12 计划第 13 节的同名约定与本条一致，不改。阶段 13 不承担该登记。

顺序约束：阶段 2 与阶段 4 均早于阶段 5，倒挂解除。

## B 类 有人提供但无人使用

### B-01 POST /api/v1/system/echo 与 ci_probe schema

结论：保留，由 feature 门控，发布制品中不得出现。

最终归属阶段：阶段 1 提供，阶段 14 校验。

确切标识符：Cargo feature 名固定为 `ci-probe`，在 `apps/core-server/Cargo.toml` 与 `testkit/Cargo.toml` 中声明，默认关闭。路由 `POST /api/v1/system/echo` 与 `ci_probe.probe_records` 的建表函数一律带 `#[cfg(feature = "ci-probe")]`。`ep-release-gate` 的校验项名固定为 `RG-CI-PROBE-ABSENT`，判据为发布制品的 `cargo tree -e features` 输出中不含 `ci-probe`，且镜像内不含符号 `api_v1_system_echo`。

回写：阶段 1 计划第 4.4 节与第 6 节各加一句 feature 门控说明；阶段 14 计划的发布门禁项清单增加 `RG-CI-PROBE-ABSENT` 一行。

### B-02 platform_core.append_only_registry

结论：阶段 3b、7、8、9a、10 在各自迁移中显式登记，由数据库检查断言登记与触发器一致。

最终归属阶段：登记表归阶段 2，登记行归阶段 3b、7、8、9a、10。

确切标识符。登记行的列以阶段 2 实建的 `platform_core.append_only_registry` 为准，为 `schema_name`、`table_name`、`mode`、`mutable_columns`；`mode` 取 `APPEND_ONLY` 或 `IMMUTABLE_COLUMNS`，`mutable_columns` 是可变列白名单，取 `APPEND_ONLY` 时必须为空数组。原裁定写的 `immutable_columns` 列在该表上不存在，语义又与 `mutable_columns` 相反，该列名作废。F-50 加入两张受控更正凭证证据表后，登记方与登记行终态固定为十六行，如下表。

| 登记方 | schema_name.table_name | mode | mutable_columns |
|---|---|---|---|
| 阶段 3b | platform_audit.audit_events | APPEND_ONLY | `'{}'` |
| 阶段 3b | platform_msg.outbox_events | IMMUTABLE_COLUMNS | `status`、`attempts`、`available_at`、`locked_by`、`locked_until`、`last_error` |
| 阶段 3b | platform_msg.dead_letters | IMMUTABLE_COLUMNS | `state`、`repaired_by`、`repaired_at`、`approval_ref`、`discard_reason` |
| 阶段 7 | procure.goods_receipt_line_costings | APPEND_ONLY | `'{}'` |
| 阶段 8 | inventory.stock_movements | APPEND_ONLY | `'{}'` |
| 阶段 8 | inventory.stock_qty_entries | APPEND_ONLY | `'{}'` |
| 阶段 8 | inventory.stock_value_entries | APPEND_ONLY | `'{}'` |
| 阶段 8 | inventory.variance_splits | APPEND_ONLY | `'{}'` |
| 阶段 8 | inventory.stock_movement_serials | APPEND_ONLY | `'{}'` |
| 阶段 9a | ledger.vouchers | APPEND_ONLY | `'{}'` |
| 阶段 9a | ledger.voucher_lines | APPEND_ONLY | `'{}'` |
| 阶段 9a | ledger.correction_vouchers | APPEND_ONLY | `'{}'` |
| 阶段 9a | ledger.correction_voucher_lines | APPEND_ONLY | `'{}'` |
| 阶段 9a | platform_core.recon_runs | APPEND_ONLY | `'{}'` |
| 阶段 10 | finance.unbilled_ar_entries | APPEND_ONLY | `'{}'` |
| 阶段 10 | finance.cash_ledger_entries | APPEND_ONLY | `'{}'` |

阶段 7、8、9a、10 的十三行一律 `mode` 取 `APPEND_ONLY`、`mutable_columns` 取 `'{}'`，阶段 3b 的三行按上表逐表给定；十六行中因此恰有十四行 APPEND_ONLY、两行 IMMUTABLE_COLUMNS。死信的可变列必须取全五列，只取三列会让触发器拒绝写 `repaired_at` 与 `discard_reason`，修复完成与丢弃两条路径在上线后直接失败；`platform_audit.audit_segments` 有状态与锚定时间更新，登记为仅追加会拒绝锚定写入，不进本清单。原裁定给阶段 9a 列的 `ledger.general_vouchers` 全库没有同名对象，GV 是 `ledger.vouchers` 的单据类型码，该行删除；原裁定给阶段 10 列的 `finance.receivable_entries`、`finance.payable_entries`、`finance.advance_receipt_entries`、`finance.advance_payment_entries`、`finance.overbilling_entries` 五张是带核销金额与状态机的可更新台账，登记为仅追加会在上线后拒绝正常核销写入，五行一并删除。触发器按登记表挂接，`platform_core.attach_table_guards` 读登记表取可变列白名单，同一迁移内必须先插登记行再挂接触发器，顺序颠倒即取不到白名单；凡本条列出的表都必须有登记行，漏登记等于无强制。检查脚本名固定为 `db/checks/append_only_consistency.sql`，由 `xtask sqlcheck` 执行。

回写：阶段 3b、7、8、9a、10 各有且只有一个对应的 `*_backfill_append_only_registry` PLANNED 迁移，并在各自退出条件核对登记、触发器与回退。阶段 3b 固定为 `db/migrations/platform_msg/V20261013092500__platform_msg_backfill_append_only_registry.sql`，先插入本节三行，再依次调用 `attach_table_guards` 处理 `platform_audit.audit_events`、`platform_msg.outbox_events`、`platform_msg.dead_letters`。阶段 7 固定处理 `procure.goods_receipt_line_costings` 一行；阶段 8 固定按 `inventory.stock_movements,stock_qty_entries,stock_value_entries,variance_splits,stock_movement_serials` 顺序处理五行；阶段 9a 固定按 `ledger.vouchers,ledger.voucher_lines,ledger.correction_vouchers,ledger.correction_voucher_lines,platform_core.recon_runs` 顺序处理五行，其迁移放在 `db/migrations/ledger/` 且晚于五张表的建表迁移；阶段 10 固定处理 `finance.unbilled_ar_entries,finance.cash_ledger_entries` 两行。每个文件都必须先插本阶段登记行，再逐表调用 `platform_core.attach_table_guards('<schema>','<table>')`；建表迁移本身不得提前调用。每个 `-- rollback:` 都先按安全顺序 drop 本批触发器再删除本批登记行，不能留下“登记/触发器只有一边”的状态。阶段 2 的死信白名单固定为五列；阶段 8 的登记数固定五行；阶段 9a 原 `ledger.general_vouchers` 三行旧口径被 F-50 的上述五行终态取代；阶段 10 两行均为 `APPEND_ONLY + '{}'`。`xtask sqlcheck` 执行 `db/checks/append_only_consistency.sql` 时必须对十六行逐项等值且返回零行。

### B-03 platform_core.migration_windows 与 open-window 校验

结论：阶段 13b 显式接入，在线 DDL 计划执行前必须持有迁移窗口。

最终归属阶段：阶段 13b。

确切标识符：端口 `ep_foundation::port::db::MigrationWindowGuard`，与 C-07 的 `IdempotencyStore` 同 crate 同模块，唯一方法为 `async fn assert_open(&self, tx: &mut dyn Tx) -> Result<(), AppError>`，由阶段 2 定义；唯一实现类型 `PgMigrationWindowGuard` 位于 `crates/adapter/db-pg/`，同为阶段 2 交付；在 `apps/core-server/src/wiring/` 与 `apps/job-worker/src/wiring/` 两个目录注入。阶段 13b 的在线 DDL 由 job-worker 的 DDL 执行器发起，窗口校验在把控制交给 ep-platform-release 的编排之前由该执行器调用注入实例的 `assert_open(tx)`；`ep-platform-release` 不引用该 trait，也不新增任何 adapter 方向的依赖。原裁定写的 `ep_platform_release::MigrationWindowGuard` 违反基线第 1.3 节“ep-platform-* 只可依赖 ep-foundation 与其他 ep-platform-*”，基线高于本表，该路径作废；阶段 3a 不承担再导出，本条对阶段 3 无落点。未持有窗口时返回 `PLATFORM.DB.MIGRATION_WINDOW_CLOSED`，HTTP 409，category 为 BUSINESS_CONFLICT。

回写：阶段 2 计划第 110 行改为上述端口与实现的落点并删去“由阶段 3a 建立 ep-platform-release crate 时以再导出方式暴露”一句，第 3.3 节把端口与实现列为对外可用组件，退出条件 E-17 改为“端口与 `PgMigrationWindowGuard` 实现均已交付且两个 wiring 已注入”；阶段 13 计划第 4.3 节 DDL 段第一步与第 895、984 三处去掉 `ep_platform_release::` 前缀，改为经装配注入的实例调用；阶段 3 计划一字不改。

### B-04 derive_blind_key 与 BlindIndex

结论：阶段 10 的银行账号查重改用盲索引，不自建第二套哈希。

最终归属阶段：阶段 10 使用，阶段 2 提供。

确切标识符：列名固定为 `bank_account_no_bidx bytea`，取值为 `derive_blind_key(legal_entity_id, 'finance.cash_accounts.bank_account_no@30', plaintext)`，其中 `@30` 是后位 F-56 密钥矩阵裁定补入的 scoped selector，裸 FQN 作废；唯一约束名 `ux_cash_accounts_legal_entity_id_bank_account_no_bidx`。同一约定在 mdm 的客户与供应商银行账号上同名同构，列名为 `bank_account_no_bidx`，各自 selector 同样以 `@30` 结尾。

回写：阶段 10 计划中银行账号查重的措辞由“哈希加盐”改为“`derive_blind_key` 与 `BlindIndex`”；阶段 5 的对应列同样按此命名。

### B-05 WasmComputePort 与 RuleEvaluator 端口

结论：阶段 13b 显式实现这两个端口，不另起接口。阶段 3b 至阶段 13b 之间两个端口都不注入任何实现，rule-evaluations 端点在阶段 13b 之前不注册，能力缺位时开一个 kind 取 `PORT_NOT_IMPLEMENTED`、`subject` 取 `WasmComputePort` 或 `RuleEvaluator` 的降级窗口并返回可重试错误，不得静默返回成功。

最终归属阶段：端口归阶段 3b，实现归阶段 13b。

确切标识符：`ep_platform_flow::port::WasmComputePort` 与 `ep_platform_flow::port::RuleEvaluator`。按 F-05 第 4 节 H-02，`WasmComputePort` 按进程边有两个实现类型：`WasmtimeComponentCompute` 位于 `crates/adapter/wasm/`，装配进 plugin-host，直接驱动 wasmtime Component 宿主；`PluginHostWasmCompute` 位于 `crates/adapter/ipc/`，装配进 core-server 与 job-worker，只经 plugin 通道的请求与响应类型把调用代理给 plugin-host，本身不链接 wasmtime。`RuleEvaluator` 的实现类型名固定为 `AstRuleEvaluator`（位于 `crates/platform/meta/src/rule/`，装配进 core-server）。端点 `POST /api/v1/platform/rule-evaluations/actions/evaluate` 只调用 `AstRuleEvaluator`，不新建求值路径。

回写：阶段 13 计划第 4.5 节明确这两个实现类型名与其对应端口；阶段 3 计划把两个端口列入交付物清单并注明实现方为阶段 13b。

### B-06 ep-contract-service::EquipmentQuery

结论：撤销该 trait。理由是客户 360 的设备区块由 ep-app-service 自己的 `Customer360SectionProvider` 实现，不需要跨模块 trait；报表侧按阶段 11 的 D-11-01 一律经受治理数据集视图取数，不经 contract trait；低代码的设备引用经 HTTP 端点解析。

最终归属阶段：阶段 12 撤销。

确切标识符：删除 `crates/contract/service/src/port/equipment.rs`。设备的跨模块可见性只保留三条路径：`GET /api/v1/service/equipments` 与 `/{id}`、全文检索索引中的 `service.equipment_records` 文档、以及阶段 12 自身的 `EquipmentsSectionProvider`。

回写：阶段 12 计划第 2 节 crate 表删去 `EquipmentQuery`；第 9.3 节的三个读取方改为上述三条路径。

### B-07 ep-contract-procure::PurchaseReturnLinkPort

结论：阶段 7 反向接入阶段 6 留下的勾稽空位，阶段 6 的直运退货验收顺延到阶段 7。

最终归属阶段：阶段 7 提供与接入。

确切标识符：`ep_contract_procure::PurchaseReturnLinkPort::link_drop_ship_return(tx, ctx, sales_return_id: Id<SalesReturn>, lines: Vec<DropShipReturnLine>) -> Result<PurchaseReturnLinkView, AppError>`；`DropShipReturnLine`、`PurchaseReturnLinkLineView` 与 `PurchaseReturnLinkView` 的唯一字段集取阶段 7 第 4.6.2 小节代码块。阶段 6 不写调用点、不注入任何实现；阶段 7 与真实 procure owner 同批把调用点接回阶段 6 的 `register_sales_return`，不存在先注入 Noop 再替换的形态。

回写：阶段 6 计划把直运退货勾稽整条推迟到阶段 7且不写调用点；阶段 7 计划交付真实端口、DTO、数据库祖先约束并首次接线，退出条件以直运退货勾稽端到端通过判定，不出现“替换空实现”。

### B-08 finance.v_recon_inventory 与 v_recon_grni 两个视图外壳

结论：视图归阶段 10；子账侧端口由阶段 8 与阶段 7 各自在本模块 contract crate 定义并由本模块 app crate 实现，阶段 10 只写注入行。

最终归属阶段：视图归阶段 10，子账侧实现归阶段 8 与阶段 7。

确切标识符（本段按 G-01 与 F-51 改写，原措辞作废）：跨阶段的唯一取数入口仍是 `ep_contract_finance::ReconciliationItemQuery`，由阶段 10 定义，按法人与会计期间返回十项勾稽的子账侧合计，结构为 `ReconciliationItemView`，阶段 9b 的关账前强制校验与其 `ReconCheck` 一律调用它。十项中的存货与已收货未收票两项，其子账侧各经被调方自己的 contract 端口取得：`ep_contract_inventory::StockValueSubledgerBalancePort`（阶段 8）与 `ep_contract_procure::GrniSubledgerBalancePort`（阶段 7）。两者签名统一为 `async fn balance(&self, snapshot: &dyn SnapshotCtx, legal_entity_id: Id<LegalEntity>, accounting_period_id: Id<AccountingPeriod>, accounting_period_seq: i32) -> Result<Money, AppError>`；id 只作同法人期间证据校验，累计顺序只用 seq，禁止比较 UUID。`InventorySubledgerBalanceQuery` 只读 inventory 的追加金额流水，`GrniSubledgerBalanceQuery` 只读 procure 的 `goods_receipt_line_costings` INCREASE/DECREASE 追加效果，均按 `accounting_period_seq <= target_seq` 返回截至期间累计，后续事件不得改变旧期间结果。两处均为 trait 外来、类型本地，`impl` 与类型同 crate，孤儿规则成立。`ep_contract_finance::SubledgerBalanceProvider` 撤销；不存在包装或占位实现。装配由阶段 10 的 `ep-app-finance` 承担，以两个 `Arc<dyn ...>` 注入点写入 core-server 与 job-worker wiring。其余八项取自阶段 10 自有表。`ep-app-ledger` 只在 9b 段依赖 `ep-contract-finance`；`ep-platform-recon` 只驱动 `ReconCheck` 而不直接依赖业务 contract。计数仍为 finance+invoice 自有 15 个 trait，inventory 与 procure 各自新增一个余额端口。U-G01-01 已由 F-51 关闭，不再存在落码时选择。

回写（本段按 G-01 改写，原措辞作废）：阶段 8 与阶段 7 各在退出条件中增加一条「已在本模块 `ep-contract-*` 内定义子账侧余额端口并由本模块 `ep-app-*` 实现，端口名、签名与实现类型名按 B-08 确切标识符段固定」；阶段 10 计划第 1131 行的措辞改为「两个实现分别由阶段 8 与阶段 7 交付，阶段 10 只在 `apps/core-server/src/wiring/` 与 `apps/job-worker/src/wiring/` 两个目录写注入行」。

### B-09 库存金额调整事件（F-54 复核后撤销）

历史结论曾要求把三段名改成 `inventory.stock_movement.value_adjusted.v1` 并由阶段 11 的 `costing.stock_value_adjust` 异步补记成本；该方案在 F-54 开发就绪复核中撤销，不再是实施指令。

最终归属阶段：事件归阶段 8，消费者归阶段 11。

现行唯一结论：`inventory.stock_value_adjusted.v1`、`inventory.stock_value.adjusted.v1`、`inventory.stock_movement.value_adjusted.v1` 与 `costing.stock_value_adjust` 均不得登记、产生、消费或实现。原因一是库存过账发生时 `PostingPort` 尚未生成非空的 `voucher_id/voucher_line_id`，旧 payload 在规定时点不可构造；原因二是 `PostingPort` 已在同一事务通过 `CostCaptureService` 捕获全部成本与价差凭证行，异步消费者正常情况下只能重复命中同一集合，异常情况下反而会制造跨事务差额。阶段 8 只保留 `inventory.stock_movement.posted.v1`，且其 payload 不含 `voucher_id`；阶段 11 删除该消费者、测试与退出条件。事件目录总数减一。

回写范围固定为事件目录、阶段 8 的事件交付数量与第 6.4 节、阶段 11 的交付物/crate/消费者/测试/退出条件、总览 B-09 与依赖格。所有旧名称只可出现在本历史裁定和事件目录的撤销说明中；其余现行规范出现即失败。

### B-10 ep-contract-mdm::SupplierSelfServiceCommand

结论：统一取阶段 5 的 trait 名，阶段 7 的措辞修订。

最终归属阶段：阶段 5 提供，阶段 7 使用。

确切标识符：`ep_contract_mdm::SupplierSelfServiceCommand`，方法固定为 `submit_profile_change(&self, tx: &mut dyn Tx, ctx: &SecurityContext, supplier_id: Id<Supplier>, patch: SupplierProfilePatch) -> Result<SupplierChangeRequestView, AppError>` 与 `upload_qualification(&self, tx, ctx, supplier_id, doc: QualificationUpload) -> Result<(), AppError>`。

回写：阶段 7 计划中门户 supplier-profile 一节的端口名一律改为 `SupplierSelfServiceCommand`，删去另一套措辞。

### B-11 ep-bench 与 ep-release-gate

结论：保留，从发布制品与 SBOM 中排除，由 ep-release-gate 自校验。

最终归属阶段：阶段 14。

确切标识符：两个 crate 位于 `tools/bench/` 与 `tools/release-gate/`，不在 `crates/` 下，不进 workspace 的默认 members 之外的任何制品清单。校验项名固定为 `RG-TOOLS-EXCLUDED`，判据为 SBOM 中不含 `ep-bench` 与 `ep-release-gate` 两个包名。

回写：阶段 14 计划的发布门禁项清单增加 `RG-TOOLS-EXCLUDED` 一行；阶段 1 的 `xtask sbom` 增加同名断言的负样例。

## C 类 同一事物被两个阶段都声称提供

### C-01 二十四个 schema、七个功能角色、二十四个属主角色、引导脚本与迁移框架

结论：全部归阶段 2，阶段 1 只保留目录约定与空壳。

最终归属阶段：阶段 2。

确切标识符：`db/bootstrap/00_database.sql`、`01_roles.sql`、`02_cluster_params.sql`、`03_role_defaults.sql`、`04_pg_hba.fragment` 五个文件名以阶段 2 的命名为准，阶段 1 的 `B001__cluster_roles.sql`、`B002__database.sql`、`B003__postgres_conf.sql` 三个文件名作废。单一全局迁移 Runner 与其版本号断言以阶段 2 第 3.3 节为准。阶段 1 只交付 `db/migrations/<schema>/` 二十四个空目录，不交付任何顺序声明文件。

阶段 1 计划第 4.1 节与第 4.2 节的三个迁移文件全部移交阶段 2，阶段 1 的自检项 `rls-enabled-and-forced` 与 `runtime-role-privileges-bounded` 保留但被测对象由阶段 2 建立，阶段 1 交付时以 `ci_probe` 探针表作为被测对象。

回写：阶段 1 计划第 2 节 D-04 改为“集群引导脚本的目录约定与执行顺序约定，脚本内容由阶段 2 交付”；第 4.1 与 4.2 节整体删除并指向阶段 2；第 13 节新增决定四关于 DELETE 授权的一条移交阶段 2。阶段 2 计划第 3.1 与 3.2 节保持不变，并在第 0 节明确本阶段是 schema 与角色的唯一提供方。

### C-02 tools/ep-migrate CLI

结论：归阶段 2，子命令取阶段 2 的五个，阶段 1 只交付骨架与退出码约定。

最终归属阶段：阶段 2。

确切标识符：子命令固定为 `apply`、`status`、`check`、`gen-rls`、`open-window` 五个。阶段 1 的 `migrate` 并入 `apply`，`verify` 并入 `check`，`manifest` 并入 `status`（`status --format=manifest` 输出制品清单）。退出码约定固定为 0 成功、2 参数错误、3 迁移窗口未打开、4 校验和不符、5 版本不一致、78 环境自检失败。

回写：阶段 1 计划第 2 节 D-03 改为“`tools/ep-migrate` CLI 骨架与退出码约定，五个子命令的实现由阶段 2 交付”；阶段 2 计划第 3.3 节把五个子命令与退出码表补全。

### C-03 UnitOfWork 的事务方法名

结论：统一为 `transact` 与 `snapshot_transact`，与 A-01 的签名一致。

最终归属阶段：阶段 1 定义，阶段 2 实现。

确切标识符：见 A-01 的 trait 定义。阶段 1 的 `transact_repeatable_read` 作废。

回写：阶段 1 计划第 7.1 节工作单元首句整句改写为 `ep-foundation` 定义 `UnitOfWork`，两个方法为 `transact` 与 `snapshot_transact`，`ep-adapter-db-pg` 提供唯一实现。基线第 10.3 节在示例之后追加一句：只读快照事务的唯一入口是 `snapshot_transact`，配合 `SET TRANSACTION SNAPSHOT` 使用。阶段 2 计划第 265 行的 `UnitOfWork trait，唯一方法 transact` 改为两个方法。

### C-04 PoolKind、RetryPolicy、SessionContext、ConnectionBudget

结论：类型定义归阶段 1，取值与预算校验脚本归阶段 2。四个类型全部留在 ep-adapter-db-pg，不进 ep-foundation。

最终归属阶段：类型归阶段 1，取值归阶段 2。

**ADR-0018 后续替代（现行）。** 原五池与 resident 42 的代码形状已经作废，禁止实现。精确类型为 `ep_adapter_db_pg::PoolKind { Rw, Ro, Worker, Ops }`；`ConnectionBudget` 的池数组长度为 4，常驻上限 37、临时上限 10、安全余量 5、硬峰值 52，integration-gateway 数据库连接为 0。`SessionContext` 与 `RetryPolicy` 其余字段和值保持本节冻结。现行 Windows 校验脚本固定为 `scripts/verify-connection-budget.ps1`；旧 `.sh` 名只作历史证据。

回写：阶段 1 计划第 7 节列出四个类型的定义与字段；阶段 2 计划第 265 行删去类型定义的表述，只保留取值与脚本。

### C-05 tests/rls_matrix

结论：三段分工，各阶段只做自己那一段。

最终归属阶段：阶段 1 提供 CI 目标与八类断言骨架，阶段 2 提供数据库侧策略断言与两个复制角色的入口借用，阶段 4 提供 32 组完整矩阵与发布门禁判定。

确切标识符：CI 目标名固定为 `tests/rls_matrix`，八类断言函数名固定为 `assert_read`、`assert_write`、`assert_update`、`assert_delete`、`assert_aggregate`、`assert_sort`、`assert_report_projection`、`assert_error_leak`，位于 `testkit/src/rls_matrix.rs`。阶段 2 追加 `assert_replication_role_containment` 与 `assert_recon_context_borrow` 两个函数。阶段 4 追加 `matrix_32.rs` 与发布门禁项名 `RG-RLS-MATRIX-GREEN`。

回写：三个阶段各自在计划第 8 节测试计划中写明本阶段承担的函数名清单，不得重复实现同名函数。

### C-06 sensitive_field_registry

结论：保留 `platform_core.sensitive_field_registry`，阶段 4 只引用不建表。

最终归属阶段：阶段 2。

确切标识符：`platform_core.sensitive_field_registry`，业务列集固定为十一列且本条即完整列集，公共列另按基线第 4 节：`schema_name text not null`、`table_name text not null`、`column_name text not null`（逻辑列名，不含 `_enc` 后缀）、`category text not null`、`security_level smallint not null`、`is_field_encrypted boolean not null default false`、`blind_index text not null default 'NONE'`、`blind_index_column text`、`mask_style text not null default 'NONE'`、`normalization text not null default 'TRIM_NFKC'`、`release_ref text not null`，唯一约束 `ux_sensitive_field_registry_schema_table_column` 在 `(schema_name, table_name, column_name)` 上。`approved_by` 与 `approved_at` 两列撤销，阶段 2 建表时不建这两列：这两列无来源可填，经迁移登记时只能以系统主体冒充产品负责人批准，规格第 12.2 章要求的批准留痕改由 `release_ref` 承载，经迁移登记时取 `MIGRATION:<迁移版本号>`，经端点登记时取 `ENDPOINT:<审批记录号>`。任何阶段不得写入本列集之外的列，也不得再声明本表另有附加列。

回写：阶段 4 计划中凡出现 `platform_authz.sensitive_field_registry` 的一律改为 `platform_core.sensitive_field_registry`，并删去对应的建表迁移；第 144 行“该表不设 `approved_by` 与 `approved_at` 两列”一句保留并成立，其后半句改为“批准留痕由 `release_ref` 承载”。阶段 2 计划第 3.5 节该表定义删去 `approved_by` 与 `approved_at` 两行，其余列按本条对齐。该表的唯一只读查询端点 `GET /api/v1/platform/sensitive-fields` 按 A-07 归阶段 2，契约以阶段 2 计划第 532 行为准；阶段 4 不注册该路由、不另写契约、不提供任何写入端点，第 451 行的“本阶段只提供”改为“该端点由阶段 2 交付，本阶段只调用”。

### C-07 幂等键的三段职责

结论：三段分工写死，三处不得各自判等。

最终归属阶段：阶段 1 校验请求头，阶段 2 定义端口，阶段 3a 建表并实现重放。

确切标识符：阶段 1 的中间件名固定为 `IdempotencyKeyHeaderGuard`，只校验 `Idempotency-Key` 头存在且为合法 UUIDv7，不合法返回 `PLATFORM.IDEMPOTENCY.KEY_REQUIRED`。阶段 2 定义 `ep_foundation::port::db::IdempotencyStore`，方法为 `try_begin(tx, scope: IdempotencyScope, request_hash: [u8; 32]) -> Result<IdempotencyOutcome, AppError>` 与 `finish(tx, scope, response_status: u16, response_body: &[u8]) -> Result<(), AppError>`，`IdempotencyScope { legal_entity_id, user_id, endpoint, key }`。阶段 3a 建 `platform_msg.idempotency_keys` 并实现该端口，返回 `IdempotencyOutcome::FirstCall` 或 `Replay { status, body }` 或 `PayloadMismatch`。

回写：三个阶段各自在计划中只描述自己那一段，删去对另外两段的描述。

### C-08 账龄分档

结论：唯一出处归阶段 11 的 `reporting.aging_bucket_profiles` 与 `reporting.aging_bucket_lines`，`AgingBucketQuery` 是唯一取用入口。

最终归属阶段：阶段 11。

确切标识符：阶段 10 先建 `finance.aging_bucket_definitions` 作为临时表并在其计划中标注为临时；阶段 11 交付两个迁移文件，都放在 `db/migrations/reporting/`：第 13 号 `V20261020091600__reporting_backfill_migrate_aging_buckets_from_finance.sql` 迁数据，第 14 号 `V20261020091700__reporting_drop_finance_aging_bucket_definitions.sql` 删除 finance 侧临时表。删表文件不新建任何对象，按通则第五条随其成对的迁数据文件归 reporting 目录，两个文件同属一个 Runner，按版本号先迁后删自然成立。原方案把删表文件放在 `db/migrations/finance/`，其版本号与 finance 目录既有的 `the withdrawn colliding finance reservation` 撞号，而全局版本号必须唯一且严格递增，该方案作废；阶段 11 为规避该顺序风险自加的标记行守卫一并删除，不再保留任何跨 Runner 的顺序断言。跨 schema 的 DROP 由 `ep_migrator` 执行，该角色已具备全部 `ep_mod_*` 成员资格。取用入口为 `ep_contract_reporting::AgingBucketQuery::buckets(tx, ctx, legal_entity_id, ledger_side) -> Result<Vec<AgingBucket>, AppError>`。

回写：阶段 10 计划第 3.2.1 节加一句“本表为临时表，阶段 11 交付后迁移并删除”；阶段 10 的账龄查询在阶段 11 到位后改经 `AgingBucketQuery`；阶段 11 计划第 3.3 节把两个迁移文件都列在 `db/migrations/reporting/` 目录下，删去 finance 目录一节与第 332 行的标记行守卫，第 694 行的第 15 条测试改为断言两个文件在同一 reporting Runner 内按版本号顺序执行且 finance 侧临时表已删除。

### C-09 客户 360

结论：统一为阶段 12 的端点与契约，阶段 5 的 `/overview` 作为同一端点的早期版本。

最终归属阶段：阶段 12。

确切标识符：唯一端点 `GET /api/v1/crm/customers/{id}/customer-360`；唯一契约 `ep_contract_crm::Customer360SectionProvider`。阶段 5 交付时该路径已启用，只挂载 mdm 自己的区块；阶段 12 接管后追加其余区块，不新增路径，不保留 `/overview`。`ep_contract_crm::CustomerPanelProvider` 作废，阶段 5 直接实现 `Customer360SectionProvider`。

回写：阶段 5 计划第 50 行的 `CustomerPanelProvider` 改为 `Customer360SectionProvider`，端点由 `/overview` 改为 `/customer-360`；阶段 12 计划第 60 行删去“新增”二字，改为“扩充阶段 5 已建立的 crm 契约”。

### C-10 供应商风险记录

结论：风险记录归 mdm，撤销 `procure.supplier_risk_records`；质量记录归 procure。

最终归属阶段：风险记录归阶段 5，质量记录归阶段 7。

确切标识符：保留 `mdm.supplier_risk_records`，撤销 `procure.supplier_risk_records`；保留 `procure.supplier_quality_records`。阶段 7 经 `ep_contract_mdm::SupplierRiskRecordPort::append(tx, ctx, supplier_id, record: SupplierRiskRecord) -> Result<(), AppError>` 与 `list(tx, ctx, supplier_id) -> Result<Vec<SupplierRiskRecord>, AppError>` 读写，该端口由阶段 5 提供。

回写：阶段 7 计划第 3.2.3 节整节删除，第 393 行的第 3 号迁移文件删除，后续迁移序号顺延；阶段 7 计划中风险记录的读写一律改为经 `SupplierRiskRecordPort`；阶段 5 计划在第 4 节增加该端口的定义。

### C-11 税率字典

结论：唯一出处归阶段 10 的 `invoice.tax_rate_options`。

最终归属阶段：阶段 10。

确切标识符：`invoice.tax_rate_options`，取用入口 `ep_contract_invoice::TaxRateOptionQuery::default_rate(tx, ctx, legal_entity_id, item_id: uuid::Uuid) -> Result<Rate, AppError>` 与 `list(tx, ctx, legal_entity_id) -> Result<Vec<TaxRateOption>, AppError>`。阶段 5 的 `mdm.classification_items` 去掉税率一类，阶段 10 之前的临时取值由阶段 5 的字典桩 `MdmTaxRateStub` 承担，阶段 10 交付时执行 `invoice_backfill_migrate_tax_rates_from_mdm`（已撤销、未分配版本）并删除桩的旧方案。

回写：阶段 5 计划的分类项类别清单删去税率一类，并注明桩类型名与其撤销时点；阶段 6 计划取默认税率一律经 `ep_contract_invoice::TaxRateOptionQuery`，不经 `ep-contract-mdm`；阶段 10 计划增加该迁移文件。

### C-12 收货入账单价的固化位置

结论：权威出处归阶段 8 的 `inventory.stock_value_entries`，`procure.goods_receipt_line_costings` 只保留数量与金额的分配关系。

最终归属阶段：阶段 8。

确切标识符（由 F-51 更新并经 mixed-pricing 红队收口）：`procure.goods_receipt_line_costings` 不存单价，保存 `goods_receipt_line_id`、`source_kind`、`source_doc_line_id`、`direction`、`quantity`、`amount`、`accounting_period_id`、`accounting_period_seq`、`posting_date`、`root_effect_id`、`reverses_id`，作为仅追加 GRNI 效果事实。`source_kind` 固定 `GOODS_RECEIPT|PURCHASE_RETURN|PURCHASE_INVOICE|PURCHASE_CREDIT_NOTE`，方向与根/父累计约束按阶段 7 第 3.2.10 节；原 `allocation_kind` 与单一 `source_purchase_invoice_line_id` 形态作废。库存取价一律经 `ep_contract_inventory::InventoryPricingLookupPort::priced_segments_by_source_line(tx,ctx,source_doc_type,source_doc_line_id) -> Result<Vec<PricedSegment>,AppError>` 回查 `inventory.stock_value_entries`，每段固定返回稳定键、来源行、IN/OUT、非负数量/金额业务幅值、`applied_unit_price` 与 `pricing_branch` 并按稳定键排序；同一收货行允许多段，不得任取或虚构单一单价。

回写：阶段 7 的 GRNI 表不加单价列，R-PROC-05 改为库存全段、GRNI 暂估段、超量结清段三方守恒；阶段 8 第 11.3 节 E5 端口名写死为 `InventoryPricingLookupPort::priced_segments_by_source_line`。销售交付行首版必须恰一 OUT 段，采购收货行允许多 IN 段。

### C-13 取价职责的归属

结论：取价一律归阶段 8，ledger 只做分录映射与借贷平衡。

最终归属阶段：阶段 8。

确切标识符：撤销阶段 7 的 `PurchaseReceiptPostingPort` 与 `PurchaseReturnPostingPort` 两个 needs。收货登记改为在同一事务内依次调用 `ep_contract_inventory::InventoryPostingPort::post_inbound` 与 `ep_contract_ledger::PostingPort::post`；采购退货改为依次调用 `InventoryPostingPort::post_outbound` 与 `PostingPort::post`；价差拆分调用 `ep_contract_inventory::InventoryVariancePort::split_variance`。ledger 侧不提供任何取价方法。

回写：阶段 7 计划中两个端口名的全部出现处按上述改写，并在第 0 节增加一句“本阶段不自行取价”；阶段 9 计划第 9.4.3 节的分层说明保留，并增加同一句。

### C-14 信用敞口查询的三个名字

结论：对外唯一入口为 `sales::CreditExposureQueryPort`，其取数来源改名为 `finance::ReceivableExposureQuery`。

最终归属阶段：阶段 6 提供对外入口，阶段 10 提供取数来源。

确切标识符。

```rust
// ep-contract-sales，阶段 6
pub struct CreditExposureView {
    pub customer_id: Id<Customer>,
    pub credit_limit: Money,
    pub in_transit_amount: Money,
    pub delivered_unbilled_gross_amount: Money,
    pub receivable_open_amount: Money,
    pub available_amount: Money,
}
#[async_trait::async_trait]
pub trait CreditExposureQueryPort: Send + Sync {
    async fn exposure(&self, tx: &mut dyn Tx, ctx: &SecurityContext, customer_id: Id<Customer>)
        -> Result<CreditExposureView, AppError>;
}

// ep-contract-finance，阶段 10
pub struct ReceivableExposureView { pub receivable_open_amount: Money,
                                    pub delivered_unbilled_gross_amount: Money }
#[async_trait::async_trait]
pub trait ReceivableExposureQuery: Send + Sync {
    async fn exposure(&self, tx: &mut dyn Tx, ctx: &SecurityContext, customer_id: Id<Customer>)
        -> Result<ReceivableExposureView, AppError>;
}
```

`finance::CreditExposureQuery` 与 `finance::CustomerCreditExposurePort` 两个名字作废。

U-E-03 现行补充：`finance.unbilled_ar_entries` 在原 `net_amount` 外必含 `gross_amount`，现有 `v_unbilled_ar_net` 同时返回 net_balance 供总账勾稽与 gross_balance 供信用；后者端口返回 `greatest(gross_balance,0)`。交付、开票、红冲/VOID、未开票销售退货的同步命令均携 net/gross；退货新增 `record_on_sales_return(...SalesReturnUnbilledArCommand{...,accounting_period_id,accounting_period_seq,deferred_from_period_id,voucher_id,net_amount,gross_amount})`。表数、迁移数、视图数和事件数不增。

回写：阶段 6 计划第 372 行的端口名改为 `ep_contract_finance::ReceivableExposureQuery`，并注明阶段 6 先注入 `NoopReceivableExposureQuery`；阶段 10 计划第 816 与 1130 行的端口名改为 `ReceivableExposureQuery` 并按上表收窄返回字段。

### C-15 应付查询端口命名

结论：统一取阶段 10 的命名。

最终归属阶段：阶段 10。

确切标识符：`ep_contract_finance::PayableLedgerQuery::open_balance(tx, ctx, purchase_invoice_id: Id<PurchaseInvoice>) -> Result<Money, AppError>` 与 `ep_contract_finance::SupplierStatementQuery::statement(tx, ctx, supplier_id: Id<Supplier>, period: PeriodRange) -> Result<SupplierStatementView, AppError>`。阶段 7 的 `PayableQueryPort` 与 `PayableStatementQueryPort` 作废。

回写：阶段 7 计划第 569 行的 `ep-contract-finance::PayableQueryPort::open_balance` 改为 `PayableLedgerQuery::open_balance`，门户对账端点的取数改为 `SupplierStatementQuery`。

### C-16 发票状态查询端口命名

结论：统一取阶段 10 的两个命名。

最终归属阶段：阶段 10。

确切标识符：`ep_contract_invoice::SalesInvoiceQuery::by_sales_order_line(tx, ctx, sales_order_line_id) -> Result<Vec<SalesInvoiceRef>, AppError>` 与 `ep_contract_invoice::InvoiceReversalStatusQuery::is_fully_credit_noted(tx, ctx, sales_order_line_id, quantity: Quantity) -> Result<CreditNoteStatus, AppError>`。阶段 6 的 `InvoiceStatusPort` 作废。

回写：阶段 6 计划第 424 行的端口名改为 `InvoiceReversalStatusQuery::is_fully_credit_noted`，并注明阶段 6 先注入空实现、阶段 10 替换，阶段 6 的 `SALES.SALES_RETURN.INVOICE_NOT_CREDIT_NOTED` 判定顺延到 M7。

### C-17 采购需求派生端口命名

结论：统一为 `PurchaseRequisitionIntakePort`。

最终归属阶段：阶段 7。

确切标识符：`ep_contract_procure::PurchaseRequisitionIntakePort::intake(tx, ctx, cmd: PurchaseRequisitionIntake) -> Result<PurchaseRequisitionView, AppError>`。F-54 开发就绪复核后的 exact DTO 为 `source_module: ModuleCode`、`source_doc_id: uuid::Uuid`、`source_doc_line_id: uuid::Uuid`、`source_contract_id: Option<Id<Contract>>`、`material_id: Id<Material>`、`quantity: Quantity`、`required_on: chrono::NaiveDate`、`unique_key: String`。CONTRACT、SALES_ORDER 两类必须给 source_contract_id；PROJECT_TASK 必须以 source_doc_id=project_id、source_doc_line_id=project_task_id，source_contract_id 可空；STOCK_SHORTAGE 必须为空。阶段 6 的 `PurchaseRequisitionDerivationPort` 作废。

回写：阶段 6 计划中该端口名的全部出现处改写；调用方与阶段 7 的真实实现不同批时整条用例不注册，不允许 `NoopPurchaseRequisitionIntakePort`。阶段 12 的 `project.project_task.requisition_requested.v1` 下游统一走该端口，来源幂等键固定为 `PROJECT_TASK:{project_task_id}`，不得拼入 HTTP Idempotency-Key；procure 对 PROJECT_TASK 固化必填 project_id，只在可空 source_contract_id 存在时固化 contract_id。

### C-18 库存过账端口命名

结论：统一取阶段 8 的 `InventoryPostingPort`，可用量查询另立 `AvailabilityQueryPort`。

最终归属阶段：阶段 8。

确切标识符：`ep_contract_inventory::InventoryPostingPort` 的三个方法固定为 `post_inbound(tx, ctx, InboundPosting) -> Result<InboundPostingResult, AppError>`、`post_outbound(tx, ctx, OutboundPosting) -> Result<OutboundPostingResult, AppError>`、`find_movement_by_source(tx, ctx, SourceRef) -> Result<Option<MovementResult>, AppError>`。阶段 7 的 `StockInboundPort`、`StockOutboundPort`、`StockAvailabilityQueryPort` 三个名字作废，第三个由 `AvailabilityQueryPort` 承接（见 A-12）。

回写：阶段 7 计划中三个端口名的全部出现处改写；阶段 8 计划在第 5 节之后新增一小节列出五个 trait 与其完整方法签名。附注（后续增量合并）：此处的“五个”只记录本条裁定当时的数量；其后按 G-01、F-05、F-51 U-G-01 与 U-F-02 增补契约，首版终态为十个，该小节即 08-inventory-costing.md 第 5.1 节“十个对外 trait 的完整签名”。

### C-19 合同派生项目任务的机制

结论：统一走事件消费，阶段 6 的同步调用需求撤销。

最终归属阶段：阶段 12 消费 `clm.contract.effective.v1` 自行派生，阶段 6 提供 `ContractDerivationPlanQuery`（见 A-16）。

确切标识符：撤销 `ep_contract_project::ProjectTaskDerivationPort`。阶段 12 的消费者名固定为 `project.contract_derivation`，幂等键为 A-16 定义的 `unique_key`。

回写：阶段 6 计划删去该 needs 与任何同步派生项目任务的措辞；阶段 12 计划第 420 行保留并写明消费者名。

### C-20 收款计划的派生方

结论：收付款计划行唯一归 clm，finance 不再派生第二套。

最终归属阶段：阶段 6。

确切标识符：唯一表为 `clm.contract_payment_schedules`。撤销 `ep_contract_finance::ReceivablePlanPort`。阶段 10 的到款自动核销按合同收付款计划取数，经 `ep_contract_clm::ContractPaymentScheduleQuery::schedules(tx, ctx, contract_id) -> Result<Vec<PaymentScheduleLine>, AppError>`，该查询由阶段 6 提供。

回写：阶段 6 计划删去该 needs，并在 `ep-contract-clm` 的 trait 清单中增加 `ContractPaymentScheduleQuery`；阶段 10 计划中收款计划勾稽的取数改为该查询。

### C-21 事务重试指标名

结论：统一为 `ep_db_tx_retries_total`，标签为 pool 与 sqlstate。

最终归属阶段：注册与填充均归阶段 2。

确切标识符：`ep_db_tx_retries_total`，类型 counter，标签 `pool`（取值 rw、ro、worker、integ、ops）与 `sqlstate`（取值 40001、40P01）。阶段 1 的 `ep_db_retries_total` 与阶段 3a 的 `ep_tx_retry_total` 两个登记撤销。

回写：阶段 1 与阶段 3 各删去对应的指标登记行；阶段 2 在第 9 节指标清单中登记该指标；`docs/metrics-catalog.md` 由 CI 校验唯一性。

### C-22 复制交叉核对指标名

结论（**本裁定已失效**，以下仅供追溯）：统一为 `ep_replication_crosscheck_age_seconds`，注册与填充均归阶段 14。原裁定的注册归阶段 14、填充归阶段 2 把注册排在填充之后十二个阶段，在本计划的集中注册模型下阶段 2 取不到句柄；且交叉核对器的装配、结论表 `platform_ops.replication_crosscheck_runs`、无结论窗口与比对本体全部在阶段 14，阶段 2 没有 apps 装配点也产不出距上次核对结论的时长，该口径作废。

最终归属阶段（**本裁定已失效**，见本条结论段的标注）：注册与填充均归阶段 14。阶段 2 只交付 `ep-adapter-db-pg` 的复制交叉核对取数函数与只读分析池中划出的独占连接，不触及该指标。

确切标识符（**本裁定已失效**：按 02-data-foundation.md:700，两个名字都不再注册也不填充，交叉核对折叠进保留量周期采样器；以下仅供追溯）：`ep_replication_crosscheck_age_seconds`，类型 gauge，标签 `channel`（取值 archive、backup）。阶段 2 的 `ep_db_replication_crosscheck_age_seconds` 作废。

历史回写说明（**整段已失效，不得照此实现**）：当时拟由阶段 2 删旧名、阶段 14 登记 `ep_replication_crosscheck_age_seconds`，并拟保留专属配置键与独占连接。F-52 已撤销这整组载体：两个指标名、两个专属配置键与独占连接均不登记，现行唯一值是复用 `EP__OPS__WAL_RETENTION_SAMPLE_PERIOD_SECONDS=30` 与既有只读分析池；精确比对、三态与落库列见 F-52 第四节。

### C-23 数据库连接池指标

结论：注册归阶段 1，填充归阶段 2。

最终归属阶段：注册归阶段 1。

确切标识符：`ep_db_pool_connections`（gauge，标签 pool）与 `ep_db_statement_duration_seconds`（histogram，标签 pool 与 statement_kind），两者在 `crates/platform/obs/src/metrics/registry.rs` 中由阶段 1 一次性注册。

回写：阶段 2 计划把这两个指标标注为“由阶段 1 注册，本阶段只填充”，删去重复登记；`docs/metrics-catalog.md` 的唯一性校验在阶段 1 的 `xtask` 中实现。

### C-24 两个平台错误码的登记归属

结论：`PLATFORM.IDEMPOTENCY.KEY_REQUIRED` 与 `PLATFORM.CAPACITY.CONCURRENCY_LIMIT` 归阶段 1，阶段 3a 与阶段 4 不得重复登记。

最终归属阶段：阶段 1。

确切标识符：两个常量位于 `crates/foundation/src/error/codes.rs`，同时登记在 `docs/error-codes.md` 的 PLATFORM 段。`PLATFORM.IDEMPOTENCY.KEY_REQUIRED` 分类 VALIDATION、HTTP 400、retryable false；`PLATFORM.CAPACITY.CONCURRENCY_LIMIT` 分类 INFRASTRUCTURE、HTTP 503、retryable true。同批由阶段 1 登记的还有 `PLATFORM.IDEMPOTENCY.PAYLOAD_MISMATCH`、`PLATFORM.CONCURRENCY.STALE_VERSION`、`PLATFORM.AUTHZ.NOT_FOUND_OR_DENIED`、`PLATFORM.AUTHZ.OBJECT_FORBIDDEN`、`PLATFORM.DB.MIGRATION_WINDOW_CLOSED` 五个。

回写：阶段 3 与阶段 4 的错误码清单中删去上述七个，并在计划中注明由阶段 1 登记。

### C-25 启动自检项的编号

结论：自检项改为按注册名标识，不用序号。裁定作出时基线第 7.3 节的十三项固定项一并改为命名项；其中三项已由本条撤销，现行基线项为十项。

最终归属阶段：阶段 1 定义注册表与十个基线项名，各阶段追加自己的命名项。

确切标识符：`SelfCheckRegistry` 位于 `crates/platform/runtime/src/selfcheck/registry.rs`，注册项为 `SelfCheckItem { name: &'static str, title: &'static str, severity: Severity, run: … }`，name 为 kebab-case。自检项分 Blocking 与 Degrading 两级，判读运行期可变业务数据行的一律取 Degrading 且不得作为启动失败条件，全量清单与分级以基线第 7.3 节与总览第 4.3 节 C-25 行为唯一出处，下表各阶段追加项按该出处重写。基线十项的名字固定为 config-parsed、database-reachable、migration-version-matched、rls-enabled-and-forced、runtime-role-privileges-bounded、secrets-resolvable、file-store-writable、clock-skew-within-limit、audit-chain-verifiable、offsite-sink-requirements，其中前八项取 Blocking、后两项取 Degrading；原列的十三项中 cgroup-quota-matched 与 license-and-modules-consistent 与 current-period-open 三项撤销。原十三项清单如下，仅供追溯，不再有效：`config-parsed`、`database-reachable`、`migration-version-matched`、`rls-enabled-and-forced`、`runtime-role-privileges-bounded`、`secrets-resolvable`、`audit-chain-verifiable`、`file-store-writable`、`clock-skew-within-limit`、`cgroup-quota-matched`、`offsite-sink-requirements`、`license-and-modules-consistent`、`current-period-open`。

各阶段追加项的名字固定如下，任何阶段不得再用序号称呼。

| 阶段 | 追加项名 |
|---|---|
| 3b | audit-evidence-store-writable、audit-signing-key-usable、attachment-store-ready、event-catalog-consistent |
| 4 | authz-snapshot-loadable |
| 11 | reporting-dataset-signature-matched |
| 13 | custom-object-ddl-consistent |

报告按注册顺序输出，注册顺序即上表顺序，基线十项在前。

回写：基线第 7.3 节把十三项编号列表改为命名列表；阶段 1 计划第 5 节按名字重写；阶段 3、4、5、11、13 计划中所有“第 14 项”“第 14 至 16 项”的措辞按上表替换为项名。

### C-26 单据类型码的全局唯一性

结论：类型码统一登记在 `docs/data-dictionary.md` 的单据类型码一节，由 CI 校验唯一。阶段 7 补分配八个码，另按 A-09 与 A-10 追加两个码。

最终归属阶段：登记文件归阶段 1，各码归其单据所在阶段。

确切标识符。全量类型码表如下，任何阶段不得新增未在此表登记的码。

| 阶段 | 类型码 |
|---|---|
| 4 | BGA、HRR |
| 5 | CUST、SUPP、MATL、PROD、PRLS、MDCR、MDIB、MDEX、WHSE |
| 6 | CT、SO、SR、DC |
| 7 | PR、PO、GR、RJ、PRT、PAYR、DN、SIU |
| 9 | OBB、GV、PCR、YEC、CORR |
| 10 | INVA、SINV、IRVS、RCPT、PAYM、RFND、CDRV、OBST、PINV |
| 11 | RT |
| 12 | EQ、CPL、WO、PRJ、PT |

DC 为交付确认单（A-09），PINV 为进项发票（A-10），CORR 为 F-50 新增的总账更正凭证，WHSE 为 F-51 新增的仓库档案。F-50、F-51 合并后的现行全集为 43 个；本表已吸收两卷增量。CI 校验项名固定为 `xtask configdoc --check-doc-type-codes`，判据为该表与 `ep-platform-sequence` 的常量表逐项一致且无重复。

回写：阶段 7 计划在第 3.2 节各单据表的 doc_no 行补上类型码；阶段 10 已登记 PINV；阶段 6 补登记 DC，即在 `sales.delivery_confirmations` 的 doc_no 行标注“由 ep-platform-sequence 生成，类型码 DC”，把第 809 行改为“合同类型码 CT、销售订单 SO、销售退货 SR、交付确认单 DC”，并在第 9 节退出条件中增加一条“四个单据类型码 CT、SO、SR、DC 已登记入 `docs/data-dictionary.md` 的单据类型码一节与 `ep-platform-sequence` 的常量表，`xtask configdoc --check-doc-type-codes` 通过”；阶段 1 在 `docs/data-dictionary.md` 建立该节与 CI 校验。

### C-27 审计证据目录的属主与写出者

结论：不冲突，写清分工。

最终归属阶段：写入归阶段 3b，写出归阶段 14。

确切标识符：目录 `/var/lib/ep/audit-evidence`，属主 `ep-worker`，组 `ep`，权限 0750。job-worker 写入证据文件并做段根签名；archive-writer 以组 `ep` 的只读权限读取并写出到服务器之外落点，不具备写入与删除权限。

回写：阶段 3 计划在附件与审计证据一节补一句“archive-writer 以组只读权限读取，本进程不写出”；阶段 14 计划在归档写出一节补一句“本进程对该目录只有读权限，证据文件与段根签名由 job-worker 产生”。

### C-28 关账受理前提二的统计口径

结论：全部凭证一律与业务事件同事务生成，Outbox 只承载派生、通知、检索与报表数据集。受理前提二重新定义。

最终归属阶段：口径归阶段 9a，措辞在阶段 4、9、10 三处统一。

确切标识符。受理前提二的判定语句固定为一句话，三处逐字一致：该法人该期间内，`platform_msg.outbox_events` 中 `status` 属于 PENDING 或 DISPATCHING、`posting_date` 落在该期间起止之间、且 `event_type` 命中 `ledger.posting_trigger_event_types` 的条目数为零，且 `platform_msg.dead_letters` 中 `state` 属于 OPEN 或 REPAIRING、同样命中该注册表的条数为零。`posting_date` 为空的平台事件一律不计入，理由是它们不产生凭证。视图名固定为 `ledger.v_pending_posting_backlog`，错误码固定为 `LEDGER.PERIOD_CLOSE_REQUEST.PENDING_POSTING_BACKLOG` 与 `LEDGER.PERIOD_CLOSE_REQUEST.UNREPAIRED_DEAD_LETTERS`。

回写：阶段 4 计划第 523 行按上述句子改写；阶段 9 计划第 9.3.12 节与第 9.4 节受理前提二按上述句子改写；阶段 10 计划第 0.1 节按上述句子改写，并删去任何暗示存在异步过账路径的措辞。

## 附录 施工期回写清单（整节作废）

本节是 2026-08-10 四次回写提交的工单，回写已执行完毕，全节自本次修订起作废：不得引用、不得据以施工、不得据以判定评审阻塞，下一次修订本文件时整节删除。本节所列各文件的落点与本次架构审计的处置冲突时，一律以本次处置为准。

另有两条跨文件的机械改写不再逐文件重复列出。其一，基线第 10.3 节已把工作单元内的写入次序定死为审计末位，各阶段计划的用例表与事务次序段中凡把审计写在 Outbox、站内通知或任何其他数据库写入之前的，一律调整为审计末位，涉及 03、04、05、06、07、09、11 七份计划；阶段 3 计划第 1665 行的澄清一与该条一并标为已回写基线，第 1469 行的取号、写审计、写 Outbox、写站内通知改为取号、写 Outbox、写站内通知、写审计，阶段 9 计划第 593 行按 account_id 升序更新余额的防死锁论证保留不动。其二，旧 B-09 曾要求把 `inventory.stock_value_adjusted.v1` 改名为 `inventory.stock_movement.value_adjusted.v1`；该机械改写现由 F-54 撤销，两名称及消费者一并删除，不得实施。

### 00-overview.md

改动缺口：A-05、A-06、A-08、A-09、A-18、A-19、A-27、B-06，另加全表格式统一。

落点：第 3.2 节依赖矩阵（阶段 7 反向依赖删去数据集一项，阶段 8 反向依赖改写为过渡科目腿由阶段 10 补、交付确认单归阶段 6）；第 3.3 节两个环的拆法（3a 增加 ep-platform-release 端口一项）；第 3.4 节关键路径（顺序改为 1 → 2 → 3a → 4 → 3b → 5 → 9a → 8 → 6 → 7 → 10 → 11 → 9b → 13 → 14）；第 4.1 节 A 类表逐行按本裁定表的归属列重写，A-05 由 13b 改 3b，A-06 删末句，A-08 保持阶段 5 并补端口位置，A-09 由阶段 8 改阶段 6，A-18 数量由十一改十二并把 procure 改 invoice，A-19 trait 由 3b 改 3a，A-27 端口由 3b 改 3a；第 4.2 节 B-06 由保留改撤销；第 4.4 节关闭方式（阻塞项清单增加 A-19 与 A-20）；第 6.1 节 R2、R3、R4 的应对措辞同步。

### 00b-technical-baseline.md

改动缺口：A-01、A-02、A-03、A-20、C-03、C-25。

落点：第 1.2 节 ep-foundation 一行的职责描述增加 Tx、UnitOfWork、SnapshotCtx、id::marker、capability、port::search、port::doc；第 4 节公共列表 created_by 一行写入 SYSTEM_PRINCIPAL_ID 字面量；第 7.3 节十三项自检改为命名项；第 10.3 节事务写法增加 snapshot_transact 一句；第 11.1 节增加档案编码格式与类型码登记表指引；第 12 节增加能力域码与动作类别的声明纪律一条。

### 01-engineering-baseline.md

改动缺口：A-01、A-02、A-03、A-07（空模块）、A-08（空模块）、A-20、A-26（注释）、B-01、C-01、C-02、C-03、C-04、C-05、C-07、C-21、C-23、C-24、C-25、C-26。

落点：第 2 节交付物清单 D-03、D-04、D-14；第 3.1 节 crate 表；第 4.1 与 4.2 节整体移交阶段 2；第 5.1 节 foundation 核心类型表（新增 Tx、SnapshotCtx、UnitOfWork、SYSTEM_PRINCIPAL_ID、SecurityContext 19 字段、ModuleCode、CapabilityDomain、ActionClass、id::marker 22 项）；第 5 节自检注册表改为命名项；第 6 节错误码登记表；第 7 节并发与事务边界；第 9 节测试计划的 rls_matrix 分工；第 13 节新增决定四移交阶段 2。

### 02-data-foundation.md

改动缺口：A-04、A-26、A-27（不使用发布通道）、B-02（登记表）、B-03（提供守卫）、B-04（提供盲索引）、C-01、C-02、C-03、C-04、C-05、C-06、C-07、C-21、C-22、C-23。

落点：第 1 节交付物清单（增加 tenancy 五表与 platform_ops.degradation_windows）；第 3.4 节迁移编号表（追加五个 tenancy 迁移与一个 platform_ops 迁移）；第 3.5 节表定义（追加五表）；第 4 节领域模型（追加 LegalEntityDirectory 与 DepartmentClosureQuery）；第 5 节 API 契约；第 6 节并发与事务边界（transact 与 snapshot_transact 两个方法）；第 7 节配置项；第 9 节退出条件；第 12 节偏离与新增决定（接收阶段 1 移交的 DELETE 授权决定）；第 3.5 节 `platform_core.sensitive_field_registry` 按 C-06 删去 `approved_by` 与 `approved_at` 两列并按十一列列集对齐；第 110 行与第 3.3 节按 B-03 改写 `MigrationWindowGuard` 的端口与实现落点并删去由阶段 3a 再导出一句；第 135 与 799 行按 A-28 改为四行并点明两张 profiles 表，第 800 行按 A-28 改为 `bank_account_no` 两行取真、`bank_name` 两行取假；第 370 行 `db/checks/` 第 11 项按 A-28 改为按 `is_field_encrypted` 分支断言；第 355 与 358 两行按 B-02 把死信的可变列白名单由三列改为五列，第 135、372、823 三行的 `append_only_registry` 登记方名单按 B-02 改为阶段 3b、7、8、9a、10；第 341 行按 C-25 把“运行期启动自检第 4 项”改为自检项 `rls-enabled-and-forced`；第 5 节九个平台路由按 A-20 在 `crates/platform/tenancy/src/capability.rs` 声明常量并增补一条退出条件。

### 03-platform-kernel.md

改动缺口：A-05、A-06（不使用）、A-07、A-19、A-22、A-27、A-28（密文列命名）、B-02（三行登记与触发器挂接）、B-05、C-07、C-21、C-24、C-25、C-27。

落点：文首范围说明（删去不建设许可与配置发布两项）；第 3.1 节交付物清单（追加 ep-platform-license、ep-adapter-search、ep-platform-release 最小通道、ConfigItemApplier 端口，编号为第 18 至 21 项，即第 18 项全文检索、第 19 项 ConfigItemApplier 端口属 3a 段、第 20 项最小配置发布通道、第 21 项模块许可本体，并按 3a 与 3b 分段标注）；第 55 行的 `config_item_apply_logs` 改为 `config_release_steps`；第 770 与 1150 行的发布状态取值按 A-27 删去 PENDING_REVIEW 并补 REJECTED；本阶段不承担 `MigrationWindowGuard` 的再导出，B-03 对本文件无落点；第 3.2 节 crate 表；第 3.0 节判定三与附件收敛任务的措辞（不使用 recon）；第 3.9 节退出条件与自检项命名；第 3.12 节偏离项；第 3.13 节依赖清单（依赖二至依赖十一逐条按本裁定改写或删除）；第 3.3.1 节迁移清单在第 32 号之后按 B-02 追加第 33 号 `V20261013092500__platform_msg_backfill_append_only_registry.sql`，属 3b 段，目录取 `db/migrations/platform_msg/`，文件内先插三行登记再依次挂接触发器；第 1536 行第 7.2 章一行的强制手段与第 9 节退出条件按 B-02 改写；第 609 行按 A-28 把 `token_ciphertext` 改名为 `token_enc` 并补 `token_key_ref text`。

### 04-identity-authz.md

改动缺口：A-02、A-03、A-04、A-19、C-05、C-06、C-24、C-25、C-28。

落点：第 3 节表清单（删除 platform_authz.sensitive_field_registry，第 150 行外键目标写死）；第 4.1 节 SecurityContext（改为引用阶段 1 冻结）；第 4 节新增三个 AUTHZ_ applier；第 5 节 API 契约；第 6 节 Outbox 与第 523 行受理前提口径；第 8 节测试计划的 rls_matrix 分工与 32 组矩阵；第 9 节退出条件（自检项改名、applier 一条、界面一条、能力域常量一条、MasterReferenceCounter 不适用）；第 144 行按 C-06 保留“该表不设 approved_by 与 approved_at 两列”并把批准留痕改为由 release_ref 承载；第 451 行按 C-06 把 `GET /api/v1/platform/sensitive-fields` 改为由阶段 2 交付、本阶段只调用不注册；能力域码常量落 `crates/platform/authz/src/capability.rs`；第 11 节末尾删去阻塞判定。

### 05-master-data.md

改动缺口：A-08、A-13、A-14、A-15、A-18、A-20、A-23、A-28、B-10、C-09、C-10、C-11。

落点：第 2 节 crate 表（ep-contract-crm 改为 Customer360SectionProvider，ep-adapter-doc 改为本阶段交付本体）；第 3 节迁移编号表（追加 sensitive_field_registry backfill 与 dataset views 两个文件）；第 4 节导入导出算法（三个 doc 端口）、分类项去掉税率一类、探针与计数器的实现方清单、可引用性判定；第 5 节 API 契约（/overview 改 /customer-360）；第 9 节退出条件（新增界面、数据集视图、能力域常量、税率桩撤销时点四条，另按 A-28 把敏感字段登记一条写实为四行且 bank_account_no 两行 is_field_encrypted 为真、bank_name 两行为假、db/checks/11 返回零行）；第 3 节第 25 号迁移按 A-28 的四行逐列取值改写；第 205 与 209 行的两张 profiles 表按 A-28 删去明文列 bank_account_no，新增 bank_account_no_enc bytea 与 bank_account_no_key_ref text 与 bank_account_no_tail text，保留 bank_account_no_bidx bytea 与明文列 bank_name text；第 12 节未决事项中 U-A-12 保持待决，待决范围写实为开户银行是否同列、三场景脱敏形态、导出是否触发重新认证三问，并写明银行账号的纳入与字段级加密按规格第 7.8 章强制落地、不在待决范围内。

### 06-contract-sales.md

改动缺口：A-09、A-14、A-15、A-16、A-17、A-18、A-20、A-21、A-23、A-25、B-07、C-11、C-14、C-16、C-17、C-19、C-20、C-26。

落点：第 1 节交付物清单（事件由 14 增为 18，删去错误码总数，追加交付确认单两表与 ep-adapter-esign 两套契约测试文件名）；第 2 节 crate 表（追加 ContractDerivationPlanQuery、ContractPaymentScheduleQuery、SalesReturnCommandPort，删去 ProjectTaskDerivationPort 与 ReceivablePlanPort）；第 3 节数据库变更（追加交付确认单两个迁移文件、把两处逻辑引用改真实外键、追加数据集视图一个文件；按 A-21 不再追加 posting_trigger backfill 文件，该两行登记由阶段 9a 的种子迁移写入）；第 4 节算法（新增交付确认三腿次序、退货前置校验端口改名、派生计划）；第 5 节 API 契约（新增四个交付确认端点）；第 6 节 Outbox 事件表（三个终态事件）；第 8 节测试计划；第 9 节退出条件（新增数据集视图、界面、能力域常量、探针与计数器、类型码 DC 五条，合计由十四条增为十九条，条数以本节实际编号为准）；第 10.2 节 PRD 节映射表末尾按 A-09 追加一行，说明交付确认在 PRD 无承载节、属附录乙 U-C-01、本阶段依据规格第 8 章第 8 步与第 5.2 章事件-分录表实现；第 11.3 节未决事项表按 A-09 增补 U-C-01 与 U-C-02 两行并逐行给出临时取值与切换代价；第 11 节风险（删去第 772 行整条，新增“入口暂不注册、待权威实现与依赖同批接线”的清单；禁止空实现占位）。

### 07-procurement-portal.md

改动缺口：A-01、A-06、A-10、A-11、A-15、A-18、A-20、A-21、A-23、B-07、B-08、B-10、C-10、C-12、C-13、C-15、C-17、C-18、C-26。

落点：第 0 节范围（补一句进项发票台账归阶段 10）；第 3.2.3 节整节删除并顺延迁移序号；第 3.2.11 节删去单价列；第 3.2.14 与 3.2.17 节的晚建 invoice 固定目标写死，并由 Stage10 精确追补迁移建立同法人真实外键；第 3 节按 A-21 撤销第 24 号 posting_trigger backfill 文件，该编号由 B-02 追加的 append_only_registry backfill 文件占用并登记 procure.goods_receipt_line_costings 一行，其后编号不变；本条回写批次当时要求第 433、895、1005 三行的迁移文件总数三十一保持不变，后续 F-50/F-51 已将现行阶段 7 计数增至 33，精确文件与阶段归属以 `docs/migration-catalog.md` 和阶段 7 第 3.4 节为准；各单据表 doc_no 行补类型码；第 4 节算法（端口名全部改写、不自行取价一句）；第 8.6 节对账语句登记改为实现 ReconCheck；第 9 节退出条件（新增界面、能力域常量、计数器与历史成交、GRNI 子账查询、类型码八个共五条）；第 11 节假设 A2 与 A3 改写。

### 08-inventory-costing.md

改动缺口：A-01、A-06、A-09（不建表）、A-12、A-13、A-15、A-18、A-20、A-21（零行）、A-23、B-02、B-08、B-09、C-12、C-13、C-18。

落点：第 0 节三条硬边界（补一句交付确认单由阶段 6 建立，本阶段只提供库存腿）；第 1 节交付物清单 D1 由四个 trait 改五个、第 31 行删去不交付界面一句；第 3 节追加 append_only_registry backfill 与 dataset view 两个迁移文件，其中 append_only_registry 按 B-02 登记五行且 mode 一律取 APPEND_ONLY、mutable_columns 取空数组；第 115 与 443 行按 A-13 把索引名改为 ix_stock_qty_entries_legal_entity_id_material_id 并删去命名例外说明；第 5 节之后新增一小节列出五个 trait 的完整签名；第 6.1 节事务句柄写实为 `&mut dyn Tx`；第 6.4 节补一句不登记 posting_trigger 行、并写明 stock_value_adjusted 的消费者名；第 9 节退出条件（新增界面、数据集视图、能力域常量、MaterialUsageProbe、ReferenceCounter、GRNI 之外的存货子账查询六条）；第 11.1 节 R2 删去总账未确认一句。
附注（后续增量合并，只针对上行的 trait 计数）：上行两处“五个”只记该批回写当时的数量；其后按 G-01、F-05、F-51 U-G-01 与 U-F-02 增补契约，首版终态为十个。08-inventory-costing.md 第 1 节 D1 与第 5.1 节已写明逐项名称、交付阶段与实现归属，上行历史数字不再作为施工指令。


### 09-ledger-period.md

改动缺口：A-01、A-06、A-09（凭证腿）、A-18、A-20、A-21、A-23、A-24、B-02、C-13、C-28。

落点：第 9.1 节交付物清单（追加 ep-platform-recon 本体与三张表）；第 9.3 节数据库变更（追加 recon 三表与 append_only_registry backfill，后者按 B-02 登记 ledger.vouchers、ledger.voucher_lines 与 platform_core.recon_runs 三行）；第 99 与 101 行按 A-21 把第 14 号种子迁移改为一次写全 13 行并直接填入 event_type 与 registered_by_module；第 9.3.11 节追加 PostingTriggerRegistry 接口；第 9.3.12 节 v_pending_posting_backlog 的口径句子按 C-28 逐字改写；第 9.4.3 节补一句 ledger 不自行取价；第 9.5.9 节把事务句柄与快照上下文类型写死；第 9 节退出条件（新增数据集视图、界面、能力域常量、recon 本体四条）；第 9.3.11 节按 A-21 把 PostingTriggerRegistry 的方法改为 assert_registered 只读断言并写全签名、错误码与不写入语义，删去全部幂等 upsert 措辞；第 78 行的 ep-app-ledger 依赖枚举按 B-08 补入 ep-contract-finance 并注明只用于 9b 段、按阶段 11 的成本与收入捕获调用点补入 ep-contract-costing 并注明 CI 的 cargo metadata 断言清单由阶段 11 同批更新；第 9 与 438 与 949 三行的子账取数按 B-08 改为经 ReconciliationItemQuery，并写明十项中的八项取自阶段 10 自有表；9b 段第 9.8.4 节新增 testkit/scenarios/golden_loop_14_steps.rs 作为黄金业务闭环十四步整体端到端验收的唯一落点，覆盖规格第 8 章第 1 至 14 步与第 17.2 章十五类必测分支，第 9.9 节退出条件追加该用例在 ep-datagen 默认 scale 上一次跑通一条。

### 10-ar-ap-invoice.md

改动缺口：A-09（过渡科目腿）、A-10、A-11、A-15、A-18、A-20、A-21、A-23、A-24、B-02、B-04、B-08、C-08、C-11、C-14、C-15、C-16、C-26、C-28。

落点：第 0.1 节按 C-28 改写；第 3.1 节追加 invoice.purchase_invoices 与 invoice.purchase_invoice_lines 两表；第 3.2.1 节 aging_bucket_definitions 标注为临时；第 3 节追加期初导入、税率迁移、append_only backfill、dataset views 四个迁移文件，其中 append_only backfill 按 B-02 只登记 finance.unbilled_ar_entries 与 finance.cash_ledger_entries 两行；按 A-21 删去 invoice 与 finance 两个目录的 posting_trigger backfill 文件；按 C-08 账龄的迁入与删表两个文件均由阶段 11 在 reporting 目录提供，本阶段不提供；第 4 节新增采购发票登记算法与三单匹配；第 5 节 API 契约（新增采购发票三个端点与期初导入一个端点）；第 7 节模块内契约表（追加 ReceiptInvoiceMatchQueryPort、PurchaseCreditNotePort、TaxRateOptionQuery、SubledgerBalanceProvider，改名 ReceivableExposureQuery，UnbilledArPort 使用方收窄）；第 8 节事件表追加 invoice.purchase_invoice.registered.v1；第 9 节退出条件（新增界面、数据集视图、能力域常量、计数器与历史成交、类型码 PINV、盲索引六条，另按 A-28 增加一条，即 platform_core.sensitive_field_registry 中存在 finance.cash_accounts.bank_account_no 一行且 is_field_encrypted 为真、db/checks/11 返回零行）；第 63 行的 F-17 一行按 A-28 改标为 F-17 与 U-A-12 并按三问写实待决范围；第 294 行按 A-28 把 bank_account_no_cipher 改名为 bank_account_no_enc 并补 bank_account_no_key_ref text 一行；第 305 行删去登记由 platform_authz 承载一句，改为在 db/migrations/finance/ 追加一支 sensitive_field_registry backfill 迁移；第 917 与 1199 行按 A-21 把 PostingTriggerRegistry::register 的幂等 upsert 改为 assert_registered 只读断言。

### 11-cost-metrics-reporting.md

改动缺口：A-06、A-08、A-18、A-19、A-20、A-23、B-09、C-08、C-25、C-26。

落点：第 1 节 D-11-04 自检项改名为 reporting-dataset-signature-matched；第 3.5 节数据集种子表按 A-18 的十三行改写（procure 改 invoice）；第 3.3 节在 db/migrations/reporting/ 追加账龄迁入与删表两个文件，删去 finance 目录一节与标记行守卫；第 4 节新增四个报表类 ConfigItemApplier；第 5 节 API 契约；第 9 节退出条件（新增四个 applier、界面、能力域常量、三个 ReconCheck、账龄迁移五条）；第 437 行的交付卡取数按 A-09 保持不读 sales.delivery_confirmation_lines 基表，实际交付日期经 clm_contract_delivery_milestones 与 sales_order_delivery_batches 两个数据集取得；第 712 与 751 两行收窄为闭环第 14 步的指标一致性用例，不含第 12 步与期间关账，整条链路的贯通验收由阶段 9b 的 testkit/scenarios/golden_loop_14_steps.rs 承担。旧“新增 `costing.stock_value_adjust` 消费者”已被 F-54 的 B-09 复核撤销，不得恢复。

### 12-service-project-asset.md

改动缺口：A-15、A-16、A-17、A-18、A-20、A-23、B-06、C-09、C-19、C-26。

落点：第 2 节 crate 表（删去 EquipmentQuery，Customer360SectionProvider 由新增改为扩充）；第 3 节追加 project dataset view 一个迁移文件；第 4.7 节派生消费者名写死为 project.contract_derivation；第 4 节退货登记改用 SalesReturnCommandPort 与三个终态事件；第 9.3.6 节三个读取方改写；第 9 节退出条件（新增界面、数据集视图、能力域常量、ServiceReferenceCounter 四条）；第 11 节 R-01 与 R-02 的缓解措辞改写；第 709 与 763 两行按阶段 9 计划第 728 行的唯一落点约定改写，指向阶段 9b 的 `testkit/scenarios/golden_loop_14_steps.rs`，本阶段只交付闭环第 12 步的用例片段与断言，整条链路的串接验收顺延到阶段 9b，第 1 节 D-10 交付物行的“闭环第 12 步的端到端用例”同步收窄为“闭环第 12 步的用例片段”。

### 13-clients-lowcode.md

改动缺口：A-05（只留验收）、A-19、A-20、A-23、B-03、B-05、C-25、C-26。

落点：第 2 节 crate 表（ep-platform-release 由本阶段新增改为阶段 3b 已建、本阶段扩展）；第 3 节三张 config 表标注为阶段 3b 已建、本阶段只做列与状态扩展，本阶段新建的四张表沿用 config_release_steps 等实际表名，第 102 行删去与 config_item_apply_logs 的括注映射；状态扩展按 A-27 改为在阶段 3b 六态之上补五态、只放宽 CHECK 不改写既有行，第 368 行的迁移删去对 PENDING_REVIEW 行的 UPDATE；第 4.3 节 DDL 段第一步改为调用经 job-worker 装配注入的 MigrationWindowGuard 实例的 assert_open，去掉 ep_platform_release 前缀；本阶段不实现也不注册任何 ReconCheck；第 4.4 节能力域码表改为引用 foundation::CapabilityDomain、判定算法第 1 条改写；第 4.5 节写明两个实现类型名；第 4.6 节写明端口由阶段 3a 提供、本阶段实现六个 applier；第 7 节自检项按名字改写；第 9 节退出条件（新增许可停用再启用一条，删去业务界面相关表述）。

### 14-ops-backup-release.md

改动缺口：A-06、A-18（无）、A-22、A-25、A-26、B-01、B-11、C-22、C-27。

历史落点清单（其中 C-22 指标一项已被 F-52 取代，不得照此实现）：第 0 节偏离二保留并补一句 degradation_windows 由阶段 2 建立；第 3 节表清单把 degradation_windows 标为扩展而非新建；新增 OpsDisposalService 实现 DisposalPort 一节；归档写出一节补审计证据目录只读一句；原拟登记 `ep_replication_crosscheck_age_seconds` 的动作撤销，现行口径是不登记该指标而复用 30 秒采样器；发布门禁项清单追加 RG-CI-PROBE-ABSENT 与 RG-TOOLS-EXCLUDED 两行；认证清单追加电子签章真实沙箱或等效验证一条；第 9 节退出条件同步。

## F 类 结构与判据裁定

本节登记已裁定的四条，编号取 F 类，不计入前文 67 条缺口。按通则第一条，四条的权威落点均在技术基线与各阶段计划正文，本节只作登记。F-01 与 F-03 的正文取自裁定原件，结论文字未作改写；F-04 与 F-05 由本轮新增，其正文按本轮交叉审查的逐条更正改写后登记，与裁定原件不一致处以本节为准。全卷不存在编号 F-02：早期以 F-02 指称的 adapter 互依一条即 F-01 的第二半（原争点 F2），凡引用 F-02 一律读作 F-01。

### F-01 事务实现类型的声明位与 db 系适配层的分层（合并裁定 F1 与 F2）

#### 争点

F1：基线第 1.4 节配套纪律第四条（00b-technical-baseline.md:166）原文「实现类型 `PgUnitOfWork` 与 `PgTx` 的声明位在 ep-adapter-db，实现落在 ep-adapter-db-pg。」在 Rust 中不可实现。`Tx` 属 ep-foundation、`PgTx` 属 ep-adapter-db，对 ep-adapter-db-pg 双双是外部类型，`impl Tx for PgTx` 触发 E0117 孤儿规则。

F2：ep-adapter-db-pg 依赖 ep-adapter-db 与第 1.3 节禁止项第五条（00b-technical-baseline.md:117）「禁止 adapter 之间互相依赖，共用逻辑下沉到 ep-foundation。」互斥，`xtask archcheck` 的 `adapter-no-peer-adapter` 规则实测报违规。

两者互相独立：只修 F1 仍留下 C-04 四个类型、C-07 `IdempotencyStore`、B-03 `MigrationWindowGuard`、阶段 11 `ReadOnlyTx`、公共能力基线类型映射五条独立的 db-pg → db 强制边；只修 F2 则孤儿规则不因加依赖而放松。

#### 结论

一、F1。凡实现 `ep_foundation::port::tx` 中 `Tx`、`SnapshotCtx`、`UnitOfWork` 三个 trait 的具体类型，其声明位与实现位一律同处一个 crate，不得分离。`PgUnitOfWork` 与 `PgTx` 一律声明并实现在 ep-adapter-db-pg。「声明位与实现位分离」这一说法在全卷作废。该条为通用条款，一并预防阶段 11 的 `ReadOnlyTx` 复刻同一错误。

二、F2。撤销 crate `ep-adapter-db`，按第五条自身给出的救济手段「共用逻辑下沉到 ep-foundation」执行，一分为二：

- 凡需要被 platform、contract、domain、application 命名的端口 trait 与数据库能力描述，下沉 `ep-foundation` 新增的 `port::db` 模块，与既有的 `port::tx`、`port::search`、`port::doc` 三个端口模块并列。落入本模块的有四项：C-07 的 `IdempotencyStore` 与 `IdempotencyScope`、`IdempotencyOutcome`；B-03 的 `MigrationWindowGuard`；阶段 11 的只读事务端口 `ReadOnlyTx`（本裁定一并钉死为 trait，不是具体类型）；规格第 7.4 章公共能力基线的字段类型与索引种类的能力描述。
- 凡只被实现方与装配方命名的具体类型、取值与 SQL 侧映射，全部落在 `ep-adapter-db-pg`：`PgTx`、`PgSnapshot`、`PgUnitOfWork`、`PgPoolFactory`、`PgMigrationWindowGuard`、`PgReadOnlyTx`、幂等存储的 pg 实现、C-04 的 `PoolKind` 与 `SessionContext` 与 `RetryPolicy` 与 `ConnectionBudget` 四个类型及其取值、`ScopePredicateRenderer`、公共能力基线到 PostgreSQL 类型与索引 DDL 的映射。C-04 的实质结论「四者不进 ep-foundation」完整保留，只改宿主 crate 名。

工作区内 db 系 adapter 从此只有 ep-adapter-db-pg 一个。

三、第 1.3 节允许项七条与禁止项七条一字不改，`xtask archcheck` 零改动。规则名 `adapter-no-peer-adapter` 不变，`FORBIDDEN_RULES` 七项不变，deps.rs:280-286 现有负样例逐字保留——其中 `assert!(!violated("adapter-no-peer-adapter", vec![pkg("ep-adapter-db-pg", &["ep-foundation"])]))` 恰好就是裁定后 db-pg 的真实依赖形态。本裁定不新增任何受限例外、不新增任何门禁规则、不新增任何白名单。

#### 裁定理由

第一，为什么不采「开例外」（方案 1 的形态例外与方案 3 的具名对例外）。二者都正确、都能编译、都能过门禁，但都是局部最优：用一个永久豁免口子换一条边的合法性，而把结构性成因原封保留。成因是「把端口 trait 停在一个 adapter crate 里」，这个成因在全卷已经复发三次——阶段 11 的 `ReadOnlyTx`（11-cost-metrics-reporting.md:98）、阶段 3a 的 `IdempotencyStore` 实现、阶段 13b 的 `MigrationWindowGuard` 接入。开例外之后这三处仍会逐一撞线，只是撞在豁免线内不再报警。

更关键的是本项目自己已经用同一形态失败过一次，且实测可查：A-01 标记类型例外的围栏，00c-gap-ruling.md:120 与 01-engineering-baseline.md:163 写的是「清单固定为 22 项，任何阶段不得增删」，而 00b-technical-baseline.md:118 与 :167 写的是「增删走普通提交……不设固定清单」「不设冻结清单」。同一处例外，两套互斥围栏，且把人工枚举换成「两条可机检判据」的那一次改动，正是本轮 F3 实测出的恒不可判定判据的来源。围栏没被拆掉，是被换成了量不出来的东西。方案 3 对这条教训的引用是准确的，但它给出的处置仍是再开一处例外并再加一层围栏（封闭枚举 + `adapter-abstraction-driver-free` 新规则 + 受限例外登记段），即为一条边新增三套机制。按评判标准第二条，这个交换不划算。

第二，为什么撤销 crate 不与规格冲突。规格第 4.3 章的引导句是「建议的代码边界：」（2026-07-19-enterprise-private-operations-platform-design.md:254），第 261 行的 adapter-db 一条落在该引导句之下。真正有强制力的是第 7.3 章第 678 行「保留数据库适配抽象层与公共能力基线的设计：业务代码只依赖抽象层，不直接依赖某一数据库的专有语法」。而按基线第 1.3 节允许项（00b:105 至 107），`ep-contract-<m>` 只可依赖 ep-foundation，`ep-domain-<m>` 只可依赖 ep-foundation 与自身契约，`ep-app-<m>` 可依赖 foundation、platform、自身 domain 与任意 contract——三条里都没有 adapter。也就是说业务代码从来就不被允许依赖 ep-adapter-db，规格第 678 行那句「业务代码只依赖抽象层」在原方案下一直是靠 `ep_foundation::port::tx` 满足的，ep-adapter-db 从来不在这条路径上。裁定后业务代码依赖 `ep_foundation::port::{tx, db}`，抽象层不但保留，而且第一次真正落在业务代码能依赖的那一层。公共能力基线同理：它的能力描述下沉 foundation 后可被 ep-platform-meta 直接命名（13-clients-lowcode.md:190 的 `ck_custom_indexes_kind` 一组 CHECK 就是它的落地），PostgreSQL 专有的类型与 DDL 映射留在 db-pg，正好是第 678 行要的分工。

第三，ep-adapter-db 的全部内容在现行第 1.3 节下没有一个合法消费者，这是层位判错而不是两个孤立缺陷。逐项核对：`PgTx`/`PgUnitOfWork`（00b:166）、C-04 四个类型（01:317，使用方是 01:316 的 `PgPoolFactory`）、公共能力基线映射（00b:88）、`ScopePredicateRenderer`（04:346）四项的消费方都是 db-pg，撞第五条；`MigrationWindowGuard`（02:116）与 `IdempotencyStore`（01:331）两项，前者消费方是 apps 与 db-pg，后者消费方是 ep-platform-runtime 的 HTTP 中间件栈（01:577「HTTP 中间件栈只留 `IdempotencyStore` 一个注入点」），而该 crate 在 01:583 自述「只依赖 foundation 与其他 platform」、基线 00b:104 也只允许这两类——这是一条与 F2 独立、至今未被发现的允许项违规，本裁定顺带修掉，开例外的两个方案则原样保留。它也不是 adapter：02:29 给 ep-adapter-db 的交付判据只有一句「端口 trait 与类型编译通过，不含任何 PostgreSQL 专有语法」，无 IO、无外部系统、无实现。全卷其余九个 adapter 的端口一律不停在第二个 adapter crate 里（search 与 doc 在 ep-foundation，file 在 ep-platform-file，wasm 在 ep-platform-flow，esign 在 ep-domain-clm，kms 在自己 crate 内），db 是唯一例外，也是唯一坏掉的那个。

第四，多库预留不被堵死。规格第 7.3 章第 677 行把 openGauss、达梦、人大金仓、OceanBase 登记为延期项。端口在 foundation 之后，第二个实现 crate 实现同一组 `ep_foundation::port::{tx, db}` 即可，天然合法；端口若留在 ep-adapter-db，第二个实现 crate 一出现就会撞上与 db-pg 一模一样的第五条。被推迟的只有跨实现共享的非端口辅助类型（`PoolKind`、`RetryPolicy`、`ConnectionBudget`）届时的一次提升，而规格第 678 行明令「任何章节与附录不得据此恢复多库交付基线或多库认证矩阵」，首版不为延期能力预付结构性成本与规格意图一致。

#### 从落选方案嫁接的三条

其一，F1 的处置写成通用条款而不是 db 专条（取自方案 1），并把「不得声明任何 `Pg` 前缀的具体类型」作为对侧约束（取自方案 3），两条合起来使阶段 11 的 `ReadOnlyTx` 不可能复刻 E0117。

其二，`ReadOnlyTx` 的形态在本裁定一并钉死为 trait（三个方案一致），具体类型 `PgReadOnlyTx` 归 db-pg。全卷仅 11:98 一处出现该名字，无第二处佐证，此为无外部佐证下的形态选择，列入残留风险。

其三，00c-gap-ruling.md:1194 的 C-03 回写指令按行号定位「阶段 1 计划第 333 行」，而被回写的正文实测在 01-engineering-baseline.md:302，已漂移 31 行（取自方案 3）。本轮把定位方式一并改为小节名锚点，不再用行号，否则下一轮按行号回写会打错位置。

#### 执行纪律

本裁定涉及的全部改动必须同批提交，尤其 00c 裁定册与 00-overview 第 4 节登记表必须与阶段计划同批。理由是实测证据：A-01 内部 00c:73「ep-adapter-db 只提供实现」与 00c:122「提供声明位，实现落在 ep-adapter-db-pg」两句互斥至今未被发现，正是因为前几轮改了阶段计划没改裁定册。虽然 00-overview.md:31 已把第 4 节与 00c 降为登记表、回写不作为评审阻塞判据，本条仍按硬要求执行。

### F-03 第 1.3 节禁止项第六条的必要性判据在阶段 1 恒不可判定

#### 0. 权威位

按 00-overview.md 第 1.4 节通则第一条，00c 是登记表、不构成权威层。本裁定的权威落点是 00b-technical-baseline.md 第 1.3、1.4、12 节与 01-engineering-baseline.md 第 10 节；00c 只登记结论与标识符。

#### 1. 事实认定（逐条有书证）

其一，必要性一条在阶段 1 恒不可判定。判据数的是跨 crate 源码引用计数，而 30 个 contract/platform crate 按 01-engineering-baseline.md:39「lib.rs 仅含 pub use 与一条编译期断言注释，不留 todo!()」为骨架，计数恒为零。仓库实现已如实承认：xtask/src/archcheck/source.rs:236-241 返回 Err，文案为「必要性判据不可判定：{} 个 ep-contract-* 与 ep-platform-* 全为骨架，无任何引用可数。」

其二，同一实现含一处静默放行，比 F3 本身更危险。source.rs:243 是无条件的 Ok(Vec::new())：只要任意一个 contract 或 platform crate 出现一行代码，本判据就从「诚实的不可判定」翻转为「零断言的通过」，从不真正数引用。该翻转在阶段 1 内必定发生——01-engineering-baseline.md:41-42 已把 ep-platform-runtime 与 ep-platform-obs 列为实现而非骨架。

其三，稳定性一条零实现。xtask/src/archcheck/mod.rs:52-58 的 evaluate 未调用任何稳定性检查。即 00b:118 的「都可机检」「由 xtask archcheck 断言上述两条判据」对应两处落空，不止必要性一处。

其四，id::marker 有两套互斥口径，必须同批合一。00b:168 写「增删按第 1.3 节的两条准入判据由 xtask archcheck 断言，不设冻结清单」；而 00c-gap-ruling.md:120（A-01 配套裁定）、00-overview.md:183、01-engineering-baseline.md:163、01-engineering-baseline.md:584 四处一致写「清单固定 22 项，任何阶段不得增删」并定性为受限例外。crates/foundation/src/id/marker.rs:5 的模块注释站在前者一侧。

#### 2. 裁定

**第一段，判据本体。** 第六条仍是两条准入判据，撤回的只是关于判定手段的两句自述。必要性一条降为评审判据，明写不由任何工具判定；稳定性一条明写为一半机检一半评审。工具不得再声称在判它不能判的东西。

**第二段，机检面不留空洞，五条替身接盘。** 必要性要防的真实动作只有一个——有人往 ep-foundation 里塞东西。该动作的物理路径逐条堵死如下：

1. `foundation-no-business/no-internal-dep`（已实现，deps.rs:160-172）：foundation 不依赖工作区内任何 crate。真正的业务概念几乎必然要引用别的类型，这道墙在依赖图上即可判。
2. `foundation-frozen-items`（已实现，frozen.rs:15-23、75-92）：堵「加进已有冻结项」。**本裁定同时把它由计数断言升为按名逐项断言**——现实现 count_unit_structs 只数数量，把 SalesOrder 改名为 Foo 仍是 22 个、静默通过，而本裁定把 22 项定为冻结清单，必须按名守。已核对 marker.rs:7-28 的 22 个名字与 00c:120 逐字一致，改判据不会立刻变红。
3. `foundation-module-registry`（新增）：堵「另开一个 foundation 顶层模块」。比对 crates/foundation/src/lib.rs 的顶层 pub mod 行与基线第 1.4 节登记的模块清单，逐行相等。当前实际为 capability、error、id、module、port、principal、security 七个。
4. `foundation-no-single-owner`（新增，取自方案 2）：堵「在已有模块内新增业务形状」这条前三条都漏的路径。取 crates/foundation/src/ 下每个 pub mod/struct/enum/trait/type 声明的模块路径段与条目名切词，与基线第 1.2 节十五个模块码词元求交，非空即违反；词元表由 xtask 从 crates/foundation/src/module.rs 的 ModuleCode 枚举体做文本解析，不 use ep_foundation，避免给 xtask 加一条对 ep-foundation 的依赖边；例外面写死为 crates/foundation/src/id/marker.rs 一个路径。**扫描面刻意不含 pub const/pub static/pub fn**：11-cost-metrics-reporting.md:97 的 36 个错误码常量落在 crates/foundation/src/error/codes.rs，段名本来就是模块码，纳入即 36 处误报。这是一条必要条件而非充要条件，如实标注（pub struct OrderDto 这类不带模块码词元的仍能过）。
5. `foundation-marker-shape`（新增）：堵「把冻结项本身改成带字段、带方法或带 trait 实现的业务形状」。`crates/foundation/src/id/marker.rs` 内只允许无字段、无方法、无 trait 实现的单元结构体，形态由本条独立断言（`frozen.rs::check_marker_shape`，规则名 `foundation-marker-shape`），与第 2 条分工：第 2 条守名字集合，本条守每一项的形态；读不到该文件即报违反，不得静默判通过。

五条合起来是净增而非净减：改前第六条在仓库里的真实判定力只有第 1 条，第 2 条只数数量，另三条不存在。

**第三段，id::marker 定为冻结清单制。** 四比一取多数，且 00c:120 是 A-01 源头、frozen.rs:16 已按 22 项实现。00b:168 的「不设冻结清单」作废。这不是放宽是收紧：冻结清单不允许任何增删，判据制允许通过判据新增。副产品是必要性判据在 00b:118 现文里唯一点名的适用对象随之变为空集——降一个适用对象为空的判据，不产生新敞口。

**第四段，可复用通则（本裁定真正的可复用部分）。** 在 00b 第 12 节新增通则第六条「判据可判定性与不可判定登记」。落点选 00b 第 12 节而非 00-overview 第 1.4 节，理由是后者标题与首句写死「五条通则」（00-overview.md:45、:47）且 00c:9、:11 另有一份副本，加第六条要连改四处计数并连带十四份文件中的「通则第 N 条」引用。通则四句：

- 凡写成「由 X 断言/由 CI 强制/由 --check 判定」的判据，必须在同处写明被测输入的提供方与交付阶段。
- 被测输入的交付阶段晚于判据所在阶段的，只有三种合法处置：整条推迟、换被测输入已存在的可判定替身、降为评审判据并登记；不得留第四种。
- 不可判定既不得表达为通过也不得表达为违反：工具须单列输出并以专用退出码结束，CI 不得把该退出码当作通过；亦不得以「计数照旧」或「两个空集合比对」的形态退化为恒真。
- **判据重新生效的触发谓词必须由判定工具自身可观测，不得写成阶段号或任何需要人工翻牌的动作**（取自方案 3；这是本项目历史失效模式的直接解药——offsite-sink-requirements 在 01:216 声明整条推迟，而 03:1525、04:724、06:777 三处下游仍按「十四项/十三项/十项全部通过」写，延后条款没有向下游传播）。

配套在 00b 新增 12.1 节登记表，**分两段**：delegated 段登记已裁定不由工具执行的判据（永久，必须点名承接的替身规则）；undecidable 段登记当前无法执行的判据（临时，**条目数由 CI 断言只减不增、并由阶段 14 发布门禁 RG-NO-UNDECIDABLE 断言归零**，形制照搬 01-engineering-baseline.md:614 对 Pending 自检项的同款纪律）。两段与 archcheck 运行期输出逐行相等，多一条或少一条均判违反。这条比对本身完全可判定，是「不得静默放行」的机械承接方。

**第五段，阶段 1 的 archcheck 行为定死为三态。** 机检面（五条替身）全绿则退出码 0，这是诚实的，因为工具不再声称在判必要性；必要性以 delegated 一行显式打印「不由本工具判定。承接方：评审举证，加 foundation-no-business/no-internal-dep、foundation-frozen-items、foundation-marker-shape、foundation-module-registry、foundation-no-single-owner 五条替身」；登记表比对不符则退出码 1；仍存在真正不可判定项时退出码 3 且明写「不可判定不等于通过」。三条路径各有唯一且不同的退出码，任何一条不能被读成另一条。

**第六段，普查八条按通则归档。** 04:724、03:1525、06:777 三处的自检项计数（十三/十四/十）必须同一批改完，否则会留下三套计数口径；09:798、09:737、10:1219 三处的「跑阶段 11 的自检项」整条推迟到阶段 11，其前半的列签名静态比对可判、保留；01:515、01:506 两条按「整条推迟」或「换判据」二择一，不得留恒真断言。

### F-04 KMS 能力的端口层位与 ep-platform-release 的依赖冻结（合并原 H-03 与 H-01）

#### 0. 权威位

按 00-overview.md 第 1.4 节通则第一条，00c 是登记表、不构成权威层。本裁定的权威落点是 00b-technical-baseline.md 第 1.2、1.3、1.4 节与阶段 1、2、3、4、12、13、14 计划正文；00c 与 00-overview 第 4 节只登记结论与标识符，冲突时正文胜出。

#### 1. 事实认定

其一，四个 ep-platform-\* crate 必须命名 KMS 接口，而该接口定义在一个 adapter 里。消费方逐条有书证：ep-platform-audit（03-platform-kernel.md:35「每 5 分钟或每 1000 条的 ECDSA P-256 段根签名」）、ep-platform-file（03:36 与 03:104）、ep-platform-notify（03:942「经阶段 2 的 `KmsBackend::unwrap` 解封该法人密钥域下的字段级密钥」）、ep-platform-release（03:1176「签名与验签……密钥经 `ep-adapter-kms` 取用」）。trait 定义位见 02-data-foundation.md:410「`ep-adapter-kms` 中：……`KmsBackend` trait（方法 `wrap`、`unwrap`、`derive_blind_key`、`health`）」。基线第 1.3 节允许项「ep-platform-\* 只可依赖 ep-foundation 与其他 ep-platform-\*，且 platform 内部不得成环」加同节「其余一律禁止」，四条边全被禁；而 03:120 自述「本阶段全部新增 crate……依赖只指向 `ep-foundation` 与其他 `ep-platform-*`」与上述四处互斥。

其二，消费面不止 platform 一层，因此落点不能是任何一个 platform crate。04-identity-authz.md:51 把 ep-adapter-kms 列入阶段 4 的改动既有 crate 表，而 04:44 同时断言 identity 与 authz「均不依赖任何 domain、application 与 adapter」；04:811 的 `SensitiveFieldDecryptor` 与 12-service-project-asset.md:355 的消费方在 application 层；14-ops-backup-release.md:38、:67 的消费方是两个 apps，这一侧合法。三侧（platform、application、apps）都能合法依赖的只有 ep-foundation。

其三，成因不止层位判错，还有端口面不完整。02:410 的 `KmsBackend` 只有四个方法，没有签名与验签；而 03:35 与 03:1176 要 ECDSA P-256 签名，13-clients-lowcode.md:569 明写「私钥由内置 KMS 或客户 HSM 持有，两种载体接口相同」。端口既无 `sign`，调用方就只剩把私钥材料取进本进程自签一条路，这既是那条非法依赖边的直接来源，也与 HSM 私钥不可导出的事实矛盾。03:87「若阶段 2 尚未交付签名接口，本阶段用其接口的桩实现开发」证明计划自己认为该接口应当存在。

其四，本条在本裁定落地前不被任何门禁挡下，已实测。在工作区副本上给 `crates/platform/release/Cargo.toml` 加一行 `ep-adapter-kms.workspace = true`，`cargo run -p ep-xtask -- archcheck` 时 16 条规则全部通过、退出码 0。原因是 deps.rs 的 `rule_platform_no_domain_or_app` 只把 `Layer::Domain` 与 `Layer::Application` 记为违规，`Layer::Adapter` 落进 `_ => None`，即该条允许项此前只有「platform 内部不成环」这一半被机检。

其五，工作区内不存在 `ep-platform-kms`，而 14:67 以该名开头写了一整行改动 crate。`crates/platform/` 下十五个目录为 audit、authz、file、flow、identity、license、meta、notify、obs、outbox、recon、release、runtime、sequence、tenancy，无 kms。

其六（原 H-01），ep-platform-meta 与 ep-platform-release 互为依赖，构成 Cargo 包级循环。`ConfigItemApplier` trait 与 `ConfigItemApplierRegistry` 定义在 ep-platform-release，阶段 13 计划把六个 CUSTOM\_ 与 UI\_LAYOUT 类 applier 实现放进 ep-platform-meta，实现外部 trait 即强制边 meta → release；而 13-clients-lowcode.md:60 的依赖列又写死 release → ep-platform-meta。两条边合起来 cargo 直接报 cyclic package dependency，工作区无法解析。03:122 逐字「无环，因为 `ep-platform-release` 不反向依赖 `ep-platform-flow` 与 `ep-platform-notify`」正是阶段 3 自己确立的反例纪律，阶段 13 把这条反向边加了回去。

#### 2. 裁定

**第一段，端口下沉，走允许项自身给出的救济手段。** KMS 能力的端口 trait 与其调用词汇下沉 `ep-foundation` 新增的 `port::kms` 模块（`crates/foundation/src/port/kms.rs`），与既有的 `port::tx`、`port::db`、`port::search`、`port::doc` 并列，形制照抄 F-01 对 `port::db` 的处置。F-04 当时冻结的最小端口面九项为：`KmsBackend` trait、`CipherText`、`KeyDomainId`、`BlindIndex`、`Aad`、`KeyRef`、`Signature`、`CipherEnvelope`、`KeyPurpose`；该历史计数不是终态封闭清单。F-56 后的现行实现必须再采用不改变六方法 ABI 的三个独立端口 `KmsSigningKeyIdentityResolver|KmsKeyMaterialProvisioner|KmsPinnedDataKeyBackend` 及阶段 2 第 4.1 节冻结的 signing/readback/pinned strong values，尤其 `DataKeyRefV1`、`DataKeySelectorV1` 与私有 `DataKeyHandleV1`；不得把这些能力塞成 `KmsBackend` 第七方法或继续按“总共九项”裁剪。阶段 1 只建空文件写模块注释，内容由阶段 2 一次补齐，与 `port::db` 同款；全部当前类型与 exact wire 以阶段 2 第 4.1 节为唯一施工清单。基线中两处「三个端口模块」的计数必须一并改为四个：00b-technical-baseline.md:43 的端口模块枚举与 00b:225 的「三个端口模块的位置与补齐时点固定」一句，后者并追加 `port::kms` 的补齐时点；阶段 1 计划中的空文件枚举同批由三个改四个，退出条件补 `port::kms` 的空模块存在性，否则 frozen.rs 断言四个文件而判据只写三个。

**第二段，端口面补齐 `sign` 与 `verify`，方法由四个增为六个。** 逐字签名固定如下：

```rust
// crates/foundation/src/port/kms.rs
#[async_trait::async_trait]
pub trait KmsBackend: Send + Sync + 'static {
    async fn wrap(&self, domain: KeyDomainId, purpose: KeyPurpose, aad: &Aad, plaintext: &[u8])
        -> Result<CipherEnvelope, AppError>;
    async fn unwrap(&self, domain: KeyDomainId, aad: &Aad, envelope: &CipherEnvelope)
        -> Result<Vec<u8>, AppError>;
    // 三参数形态保留；F-51 后 BlindIndex 返回宽度固定为完整 32 字节
    async fn derive_blind_key(&self, legal_entity_id: Id<LegalEntity>, column_fqn: &str, plaintext: &[u8])
        -> Result<BlindIndex, AppError>;
    async fn sign(&self, key: &KeyRef, payload: &[u8]) -> Result<Signature, AppError>;
    async fn verify(&self, key: &KeyRef, payload: &[u8], signature: &Signature) -> Result<bool, AppError>;
    async fn health(&self) -> Result<(), AppError>;
}
```

`derive_blind_key` 的三参数形态取自既有逐字原文（02:458、05:220、10:322）并继续保留。**历史结论已作废：F-04 作成时曾因 16 字节、完整 32 字节与可配置三套旧句互斥而暂不冻结返回宽度；F-51/F-52 已明确替代该结论，现行唯一值是完整 32 字节，`BlindIndex` 固定为 `[u8; 32]`，列、测试向量与跨法人派生全部同宽，不得实现 16 字节截断或 16/32 配置。U-F04-01/D-01 已关闭，不再等待落码选择。** `verify` 返回 `Result<bool, AppError>`，`false` 表示验签不通过，由调用方按 13:570「任一不通过置 REJECTED 并返回对应错误码」映射到其已登记的错误码，本裁定不新增任何错误码。签名算法在全卷已固定为 ECDSA P-256（03:35、03:1176、13:569），端口不带算法参数。该 trait 无泛型方法，对象安全，装配时以 `Arc<dyn KmsBackend>` 注入，落点为 `apps/core-server/src/wiring/` 与 `apps/job-worker/src/wiring/` 两个目录，阶段 14 的 archive-writer 与 backup-writer 两个 writer 各自在其进程入口注入；本裁定涉及的 `KmsBackend` 注入点只有这两个 wiring 目录与上述两个 writer 的进程入口，`apps/` 下其余进程的 wiring 目录一律不注入 `KmsBackend`。「密钥经 `ep-adapter-kms` 取用」这一说法在全卷作废：私钥与数据密钥材料一律不出载体，03:87 的「若阶段 2 尚未交付签名接口……桩实现」整句与 03:1710 的「缺失时以内存桩实现开发」一并删除。

**第三段，实现与密钥材料留在 ep-adapter-kms，crate 不撤销。** 两个载体实现类型 `BuiltinKmsBackend` 与 `HsmKmsBackend`（后者在 `hsm` feature 下）一律声明并实现在 ep-adapter-kms；`KeyDomain`（含 `domain_kind` 与阶段 2 第 4.2 节的四态）、`DataKey`、`BlindIndexKey` 三项不进端口，留在 ep-adapter-kms——它们是密钥材料与密钥域状态本身，端口存在的意义正是让这三样不出载体。与 F-01 不同，本 crate 不撤销：它有真实 IO、真实外部系统（HSM）与真实实现，工作区成员仍为 84。同时把 F-01 的通用条款适用面由三个 trait 名扩为端口模块全体：凡实现 `ep_foundation::port::*` 各模块中任一 trait 的具体类型，其声明位与实现位一律同处一个 crate，不得分离。该条款在全卷有三处逐字复述，**必须同批扩面**：00b-technical-baseline.md:165、01-engineering-baseline.md:586、00-overview.md:279，漏改任何一处即留下宽窄两套。扩面已逐个核对五个端口模块的实现方（port::tx → db-pg、port::db → db-pg、port::search → adapter-search、port::doc → adapter-doc、port::kms → adapter-kms），全部满足，不产生任何新违规。

**第四段，把允许项未被机检的那一半补上。** 新增 `xtask archcheck` 规则 `platform-no-adapter`：`Layer::Platform(_)` 的包依赖中出现 `Layer::Adapter(_)` 即违规。它与 `platform-acyclic` 同属允许项的机检面，**不进 `FORBIDDEN_RULES`**——禁止项仍是七条，规则名与顺序一字不改。配一个负样例 `pkg("ep-platform-release", &["ep-adapter-kms"])`，即本缺陷本身。落地后 archcheck 由 16 条增为 17 条，01-engineering-baseline.md:518 的「已判定规则共 14 条」同批改为「共 17 条」并逐项列出；该处此前已漏记 `foundation-marker-shape` 与 `undecidable-registry-matched` 两条（工具实测本已打印 16 条），属既有漂移，一并修。

**第五段，02:54 的 `crypto` 顶层模块作废，三项并入 `port::kms`。** 02:54 原文「只增三项：`crypto::CipherText`、`crypto::KeyDomainId`、`crypto::BlindIndex`」要求在 ep-foundation 新开第八个顶层模块，而 F-03 落地的 `foundation-module-registry` 把顶层模块冻结为七项（capability、error、id、module、port、principal、security），阶段 2 一落地即变红。三者本就是本端口的调用词汇，一并落 `port::kms`，并在 `crates/foundation/src/lib.rs` 按既有 `pub use` 惯例再导出，使 02:458 与 05:220 逐字写的 `foundation::BlindIndex` 继续成立。顶层模块数仍为七，登记表不动。

**第六段（原 H-01），ep-platform-release 的依赖冻结与 13b 编排归位。** 三条：其一，ep-platform-release 一律不反向依赖任何 `ConfigItemApplier` 属主 crate，03:122 的无环论证由点名 ep-platform-flow 与 ep-platform-notify 两个，推广到全部十五个 applier 属主（含 ep-platform-authz、ep-platform-meta、ep-app-reporting），并写明跨 crate 的执行编排一律落 `apps/*`。其二，03-platform-kernel.md:114 的 ep-platform-release 段句末追加「本 crate 的工作区内依赖在 3b 段止于 ep-foundation、ep-platform-audit、ep-platform-outbox 三项，阶段 13b 不再新增」；13-clients-lowcode.md:60 的依赖列去掉 `ep-platform-meta` 冻结为三项，职责列删「自动测试编排、DDL 段编排」，改为「自动测试结论的记录与守卫判定」，并补一句「本 crate 一律不反向依赖任何 `ConfigItemApplier` 属主 crate」。其三，阶段 13 退出条件 18 追加断言：**本阶段结束时** ep-platform-release 的工作区内直接依赖恰为三项，`platform-acyclic` 与 `platform-no-adapter` 全绿；该断言按 F-05 通则甲-2 只约束本阶段结束时点，不封禁后续阶段在允许项内增边。本段成立的前提是 `KmsBackend` 已按第一段下沉 foundation（否则 release 还要连 ep-adapter-kms，「恰为三项」当场为假），二者**必须同批提交**。

**第七段，允许项与禁止项一字不改，不新增受限例外、白名单或登记表行。** 第 12.1 节 delegated 与 undecidable 两段在本裁定内一行不加（F-05 另加一行，见下），archcheck 三态输出不变。必要性判据按 F-03 属评审判据，本项举证为：`port::kms` 被 ep-platform-audit、ep-platform-file、ep-platform-notify、ep-platform-release 四个 `ep-platform-*` 引用（03:35、03:36 与 03:104、03:942、03:1176），满足 00b:117「或被 `ep-platform-*` 引用」。另更正一处沿袭错值：00-overview.md:279 的 F-01 登记行把第 1.3 节允许项写成「五条」，实测 00b:102 至 :108 为**七条** bullet（禁止项 00b:112 至 :118 才是七条），同批改为七条。

#### 3. 裁定理由要点

落点候选四个，逐个核对后取端口下沉。（a）留在 ep-adapter-kms 即现状：四条 platform → adapter 边全违反允许项，且一旦某个 adapter 需要 KMS（如信封加密落在 ep-adapter-file）就撞禁止项第五条，与 F-01 的第二半同构。（b）新建 crate ep-platform-kms：能编译能过门禁，但工作区成员由 84 回升到 85，四个消费方分处四个 platform crate 会新增四条 platform → platform 边并直接逼近本裁定第六段刚拆掉的成环面，且它承载的东西无一是平台能力——没有表、没有用例、没有状态机，只有一个 trait 与八个数据类型，正是 F-01 判 ep-adapter-db「不是 adapter」时用的同一把尺子。（c）开例外：成因（端口停在 adapter crate 里）原封保留且已复发六处，F-01 已把这条路的账算过——本项目唯一一处既有受限例外的围栏已裂成两套互斥措辞。（d）端口下沉 ep-foundation：这是允许项自身给出的救济手段，把被依赖物挪进允许集合不是绕过规则而是执行规则，与 F-01 对 `port::db` 的处置逐字同形，且裁定后一条依赖边都不新增——非法的边不是被允许了，是根本不再产生。取（d）。

必须一并补 `sign` 与 `verify`：不补，release 与 audit 要做 ECDSA 签名就只能取私钥自签，被裁掉的依赖边会从后门原样长回来，13:569「私钥由内置 KMS 或客户 HSM 持有」在密码学上也不允许导出。必须一并加 `platform-no-adapter`：不加则按 F-03 通则第六条只剩「往 12.1 节 delegated 段永久加一行」这一档，那是净减；加规则是三档里唯一不产生永久降级、且能让缺陷当场变红的一档，成本是 deps.rs 内一个十余行函数加一个负样例，判定式复用 graph.rs 已有的层位判定，不引入新概念。**须如实说明：该规则在 F-01 落地后的判定面只覆盖原 H-03 一条**——原 H-04 的 `ep_adapter_db::port::IdempotencyStore` 已由 F-01 的端口下沉修掉（01:577 现文逐字只写 `IdempotencyStore`，无 `ep_adapter_db::` 前缀），附录丙 H-04 行的原措辞已过期，不得再据以宣称本规则「一次覆盖两条」。顺带更正 F-01 裁定理由第三段中的一句事实错误：「kms 在自己 crate 内」——它当时被当作健康样例列举，本裁定证明它是第二个坏掉的。

#### 4. 本裁定当时未含、现已由 F-52 闭合的两项

本段只保留历史成因，不再构成待决。F-52 已把阶段 13b 自动测试固定为九套：原八套加 F-21 的 `RULE_SEMANTICS`，中立 SPI 与适用映射归 `ep-platform-release`，四个属主 crate 实现并由 `apps/job-worker` 精确装配；无适用 item_kind 的套件才可 `SKIPPED`。F-52 同时冻结异步派发为 `config_packages` 的耐久领取字段加同一批次九条 `config_autotest_runs`，不新增事件。F-54 已把阶段 13 事件集合机械收口为三个具名 `platform.custom_record.*.v1` 事件；旧“十项”未给出其余七个名称，已撤销。D-02、D-03 均已关闭；F-04 不再留下另行补裁项。

### F-05 非阻塞批：依赖枚举口径、禁止项第七条判定面与原 H-02、H-04 至 H-09

#### 0. 权威位

按 00-overview.md 第 1.4 节通则第一条，00c 是登记表、不构成权威层。本裁定的权威落点是 00b-technical-baseline.md 第 1.2、1.3、12 节与阶段 1、3、5、7、8、9、10、11、12、13、14 计划正文。

#### 1. 共同成因

成因甲（原 H-04、H-08、H-09）：某个 crate 的落点或依赖被一处写成封闭枚举，被另一处的增量推翻，而全卷同时声称有一套「CI 按期望依赖清单逐 crate 比对」的门禁在守。实测该门禁在工作区里不存在：`xtask/src/archcheck/` 里没有任何一条规则读期望依赖清单，`cargo metadata` 的唯一消费者是 `xtask/src/graph.rs` 的层位图构建，判定一律按层位。成因乙（原 H-05、H-06）：禁止项第七条从未在基线上给出判定面，阶段 11 单边给了一套（D-11-01「只约束基表」），阶段 11 自己又越出这套；两条必须同批裁定，先定判定面 H-05 才有依据。成因丙（原 H-02、H-07）：跨进程通道的类型与实现无处安放时被就近塞进 ep-adapter-wasm 或 ep-foundation，而正解已写在计划自己的另一处。

#### 2. 通则甲　依赖枚举的效力口径

甲-1（复述基线 00b:113，不新增）：需要命名某模块具体类型或 DTO 的代码，落在拥有该类型的 crate 内；模块间同步调用只能通过 ep-contract-B 中的 trait，实现在 apps 装配时注入。「不属于任何模块」不构成把类型放进 `ep-foundation` 的理由——禁止项第六条的必要性判据是准入的唯一入口，按 F-03 它已降为评审判据，降级只改判定主体，不改判据取值。须注意 00b:113 本身不指定 trait 归属方是调用方还是被调方，该问题由 G-01 按模块归属判据另行裁定。

甲-2：各阶段计划中「ep-X 依赖 A、B、C」「只依赖 ep-foundation」一类枚举，一律解释为**该阶段结束时的快照**，不具跨阶段封闭效力。后续阶段可在基线第 1.3 节允许项内为既有 crate 增边，只需在该后续阶段的 crate 改动表里写出增量并在提交说明中给出使用位，不回改先前阶段的枚举。

甲-3（收窄后的撤销，与裁定原件不同，以本节为准）：撤销的只有「CI 的 `cargo metadata` 断言脚本**按 crate 逐项比对期望依赖清单**」这一形态，承接方是 `xtask archcheck` 已实现、已配负样例的七条禁止项加 `platform-acyclic` 与 `platform-no-adapter`，属 F-03 通则第六条三档处置中的第二档「换一个被测输入已存在的可判定替身」，不需要在第 12.1 节登记。**保留**「按 `cargo metadata` 断言某进程不链接某 crate」这一形态：它的被测输入就是 `cargo metadata` 的输出，提供方阶段 1 已存在，是可判定的具体谓词，一刀切撤销会制造两条无承接方的判据，正是 F-03 通则第六条要抓的形态。据此，02-data-foundation.md:60（portal-gateway 与 plugin-host 不链接 `ep-adapter-db-pg`，规格第 7.7 章两进程常驻连接数为零的编译期保证）与 10-ar-ap-invoice.md:1158（`finance.cash_ledger_entries` 只被四个用例的仓储写入）两处**不改**；其中 10:1158 依赖调用图分析的那一半在工作区内无任何工具承接，登记为待决项 U-F05-01，由阶段 10 在交付时给出判据或按通则第六条降为评审判据，本裁定在 00b:120 处标注一句「调用图一侧的判据由阶段 10 同批给出」。按上述收窄改写的落点为 03-platform-kernel.md:122、03:1549、07-procurement-portal.md:40、09-ledger-period.md:90、11-cost-metrics-reporting.md:112 与 11:825，其中 11:825 是裁定原件点名却漏改的一处。

#### 3. 通则乙　禁止项第七条的判定面回写基线

禁止项第七条改写为两条通道加一个机检面，D-11-01 的单边界定随之作废：

- 通道一（运行期读写路径的唯一通道）：跨模块取数经拥有方 `ep-contract-*` 的 trait，实现由拥有方的 `ep-app-*` 提供并在 apps 装配注入。
- 通道二（分析与报表路径的唯一通道）：拥有方在其自身 schema 内发布、并在 reporting 数据集注册表登记的 `v_` 受治理视图，只经 `ep_analyst_ro` 只读连接读取。
- 机检面（按 `xtask archcheck` 实际可判范围逐字写，不得写成工具判不出的形态）：规则 `db-pg-one-schema-per-file` 按基线第 3 节登记的 24 个 schema 名判定，只在双引号字面量区间内取词，自身 schema 由文件路径归属确定，判据为「文件内出现自身 schema 之外的非 `v_` 对象即违反」。规则名与规则条数不变。

承接方与阶段按 F-03 通则第六条写明：`v_` 前缀这一半由阶段 1 的 `xtask archcheck` 判定，被测输入是 db-pg 源码，阶段 1 已存在；「该视图确已登记且列签名一致」这一半由阶段 11 的启动自检项 `reporting-dataset-signature-matched` 判定，被测输入是数据集注册表与来源视图，届时提供方已存在，不构成不可判定项。**通道二「只经 `ep_analyst_ro` 只读连接」这一半在源码上不可观测，无机检承接方**，按通则第六条第三档降为评审判据，在基线第 12.1 节 delegated 段登记一行，承接方写「阶段 11 的 `reporting-dataset-signature-matched` 启动自检加评审举证」。这是本轮唯一一条新增登记行，undecidable 段仍为空。与之配套，11-cost-metrics-reporting.md:186 逐字「三个视图 GRANT SELECT 给 ep_analyst_ro 与 ep_app_rw」**必须同批**改为只授予 `ep_analyst_ro`，否则通道二的角色约束在阶段 11 当场落空，两句同时有效且互斥。

为什么不把 D-11-01 收紧回逐条经 contract trait 往返：11:22 给的理由（150 万条分录基准集、常用报表 P95 10 秒）是硬约束，且要改 13 个数据集与整条取数路径。为什么不把第七条整体放宽到只约束基表：那正是原 H-05 得以成立的口子，不走 `ep_analyst_ro`、不走视图的 rw 路径也会被放行。

#### 4. 逐条裁定

**H-02（blocking）——结论采纳（两个实现按进程边切开），穷举白名单撤销，理由第二条重写。** `WasmComputePort` 的实现按进程边一分为二：`WasmtimeComponentCompute` 落 `crates/adapter/wasm/`，归属进程收为 plugin-host，直接驱动 wasmtime Component 宿主；`PluginHostWasmCompute` 迁入 `crates/adapter/ipc/`，装配进 core-server 与 job-worker，只经 plugin 通道的请求与响应类型代理调用。依赖口径按下列一段写，**裁定原件第二段的穷举白名单整段撤销**：

> ep-adapter-wasm 与 ep-adapter-ipc 一律不依赖任何其他 `ep-adapter-*`；其余依赖按基线第 1.3 节允许项第六条，即可依赖 ep-foundation、ep-contract-\*，以及 domain 与 platform 中的端口 trait。ep-adapter-ipc 依赖 ep-platform-runtime 以实现基线第 1.2 节 ep-platform-runtime 一行所定的 IPC 服务端接口，依赖 ep-platform-flow 以实现 `WasmComputePort`；ep-adapter-wasm 依赖 ep-platform-flow 与 wasmtime。

撤销白名单的理由：00b-technical-baseline.md:58 逐字「……以及以 trait 表达的服务器骨架。具体 HTTP 与 IPC 传输实现分别留在对应的 ep-adapter-\*，由 apps 在 `apps/<proc>/src/wiring/` 目录下注入，本 crate 不依赖任何 ep-adapter-\*」与 01-engineering-baseline.md:583 同款口径，要求 ep-adapter-ipc `impl` ep-platform-runtime 的 IPC 服务端 trait，孤儿规则使该 impl 只能落在 ep-adapter-ipc 内，故其 `[dependencies]` 必含 ep-platform-runtime——与白名单不可同时成立；且白名单会把 H-07 的七种报文类型落点掐死。**理由第二条重写**（不再宣称「被逼出来的唯一合法宿主」）：落 ep-adapter-ipc 是既有形态（00b:58 与 01:583 已把「传输实现留 adapter、apps 注入」写死）的直接沿用；把跨进程代理放 ep-platform-flow 需为 runtime 新增一个 IPC **客户端** trait（全卷未定义），代价更大，故不取。内文两处更正：裁定原件写的 `F-02` 一律读作 `F-01`（F 类只有 F-01、F-03、F-04、F-05 四条，不存在 F-02）；`forbidden-std-io` 不是规则名，source.rs 复用的是 `domain-contract-no-io`。同批须做的两件事：13-clients-lowcode.md:947 的 85% 行覆盖率名单补入 `ep-adapter-ipc`（`PluginHostWasmCompute` 迁入后不得静默降门槛）；核 01-engineering-baseline.md:52 的 ep-adapter-ipc 装配进程列（现无 job-worker，而本条要求注入 job-worker 的 wiring 目录），若属阶段 1 时点口径则加限定语。本文件第 1092 行起的 B-05 确切标识符段同批改写。

**H-04（minor）——采纳，只改措辞，不动依赖边。** 依赖方向部分已由 F-01 的端口下沉修掉（01:577、01:330 现文已为 `ep_foundation::port::db::IdempotencyStore`，磁盘上 `crates/adapter/` 下无 db 目录）。残留的是 00b:58 承诺「具体 HTTP 与 IPC 传输实现分别留在对应的 ep-adapter-\*」，而第 1.2 节适配层清单九行没有任何 HTTP adapter，01:583 又把 HTTP 服务器与中间件栈骨架放进 runtime。裁定：**不新增 HTTP 系 adapter**，改 00b:58 与 01:583 使两处口径一致——HTTP 骨架直接构建在第三方 HTTP 库上（第三方库不是工作区 crate，不落在禁止项判定面内），IPC 的具体传输实现留在 `ep-adapter-ipc`，runtime 仍不依赖任何 `ep-adapter-*`。本文件附录丙 H-04 段的「第 1.2 节适配层清单十个 crate」一并改为九个。新增 HTTP adapter 的路线否掉：它只会被 apps 与 runtime 依赖，而 runtime 依赖它即构成 platform → adapter（与 F-04 同形），为一个措辞多造一个 crate 加一条违规边，不划算。

**H-05（major）——采纳处置，成因段重写。** 先纠正裁定原件的成因描述：11-cost-metrics-reporting.md:369 逐字「| inventory\_stock\_value\_entries | inventory.v\_stock\_value\_entries | 8 | ENTRY |」，即 11:628 引的是**已登记数据集名**，其来源本就是 `v_` 视图，「跨读 inventory schema 基表」这一成因**不成立**。真正的残余违反只在连接角色一维：11:630 逐字「三项由阶段 9a 的 ReconExecutor 调度，在 job-worker 自身连接池上执行，不使用只读分析池」——它走的不是通道二要求的 `ep_analyst_ro`。处置仍按通道一：`COSTING_INVENTORY_COGS_VS_STOCK_VALUE` 的实现方仍是 ep-app-costing（校验项数、`category`、`blocks_period_close` 均不变），但存货侧金额改经被调方的 contract 端口取得。端口命名与 G-01 同族：`ep_contract_inventory::StockValueOutboundPort`，落 `crates/contract/inventory/src/port/stock_value_outbound.rs`；实现类型 `InventoryStockValueOutboundQuery`，落 `crates/application/inventory/src/projection/stock_value_outbound.rs`；签名照抄同族形状 `(snapshot: &dyn SnapshotCtx, legal_entity_id, accounting_period_id) -> Result<Money, AppError>`，只是返回该期间出库方向的金额合计。trait 与实现由阶段 11 同批交付，在 `apps/job-worker/src/wiring/` 目录下注入，不改阶段 8 的交付物与退出条件；`ep-app-costing` 的依赖枚举补入 `ep-contract-inventory`（六个契约增为七个），落在允许项内。否掉的两条替代路线：(a) 继续走通道二直接读 `inventory.v_stock_value_entries`——现文正是这么做的，问题在于它跑在 job-worker 自身连接池而非 `ep_analyst_ro`；要让它合法就得给 rw 角色开受治理视图授权，而 11:186 现文「三个视图 GRANT SELECT 给 ep_analyst_ro 与 ep_app_rw」恰好就是这个口子，保留它等于让任何 app 都能用视图绕过 contract trait，是把一处局部问题换成一条全局口子（11:377 是数据集目录行、不是授权句，授权句在 11:380 与 11:186，裁定原件引 11:377 立论有误，一并更正）；(b) 把校验项移给 ep-app-inventory——它就得反过来读 costing schema，问题只是换了个方向。

**H-06（major）——采纳，基线回写，D-11-01 由「偏离」降为「已回写的新增决定」。** 按通则乙改写 00b:118，并把 11:22 的 D-11-01 行改写为「本阶段的分析取数按基线第 1.3 节禁止项第七条通道二执行，判定面已同批回写基线，本行不再是对基线的偏离」。**保留编号 D-11-01 不重排**：00c:439、00c:1104、11:135、11:448 四处引用它表达「分析 SQL 中不出现来源模块基表名」，该结论在改写后仍成立，删行会牵动四处并使 D-11-02 至 D-11-05 全部重排。此举净减一条例外：第七条从此只有一套取值且落在基线。措辞须与通则乙对 11:186 的同批改动协调，不得留两套授权口径。

**H-07（major）——采纳，删半句，落点取同一文件四行之上已经给出的那个。** 14-ops-backup-release.md:68 的改动 crate 表原文已写「| ep-adapter-ipc | 全部 | 新增本阶段七种报文类型 |」，与 14:73「放在 ep-foundation 的 ipc 模块下」互斥，且与阶段 13 对同一 crate 的处置一致（13:62「新增 plugin 通道的请求与响应类型」）。裁定：七种报文类型定义在 `ep-adapter-ipc`，`ep-foundation` 不新增 `ipc` 模块；14:73 删去该半句，改为写明约束——这些报文类型**不得被任何 `ep-platform-*` 命名**（否则构成 platform → adapter），core-server 侧对上报内容的审计落库在 `apps/core-server/src/wiring/` 处转换为 platform 类型。**须补一句**：ep-platform-runtime 侧的 IPC 服务端 trait 以泛型或字节切片表达，不命名这七种报文类型。该约束此前无机检承接方（`rule_platform_no_domain_or_app` 把 Adapter 落进 `_ => None`），F-04 的 `platform-no-adapter` 落地后有检，两条因此同向。依据其余两点不变：按甲-1 与禁止项第六条，这批类型的引用方只有 `ep-adapter-ipc` 与三个 apps，一个 `ep-contract-*` 都没有，也没有任何 `ep-platform-*`，必要性判据取值恒为假；落 ep-foundation 的顶层 `pub mod ipc` 会当场撞冻结的七项模块登记表，改塞 `port::ipc` 则与 00b:225 所定端口模块的位置与补齐时点固定（按 F-04 已改为四个且逐一点名）互斥，两条路都不通。

**H-08（minor）——采纳，补依赖，并按甲-2 给该枚举加限定语。** 08-inventory-costing.md:51 的「逐条自查」清单缺 `ep-contract-ledger`。缺陷成立：08:447 按裁定 A-06 固定阶段 8 实现两个 `ReconCheck`，其中一个 `category()` 取 `SUBLEDGER_VS_LEDGER`，其总账侧只能经 `ep_contract_ledger::TotalAccountBalanceProvider` 取得；改读 ledger schema 即落进第七条。编译成立已核实：**09-ledger-period.md:16 逐字「9a 段交付：……`AccountingPeriodResolver`、`PostingPort` 与 `TotalAccountBalanceProvider` 三个对外契约……」**，而 9a 在固定链上排在阶段 8 之前（08:54 逐字链「1 → 2 → 3a → 4 → 3b-1 → T0 → 5 → 9a → 8 → 6 → 7 → 10 → 11 → 9b → 14」），09:20 说的是四类**校验项**在 9b 实现，不是 trait 定义。裁定：08:51 补入 `ep-contract-ledger`，注明用途为 `TotalAccountBalanceProvider` 并注明「本阶段结束时的快照」。附带核实两点：ep-app-inventory 并不调用 `ep_contract_ledger::PostingPort::post`（08:12、08:571 明写调用方是采购、销售、发票模块），因此这条依赖的唯一理由是总账侧余额提供者；08:472「不在本模块内形成第二处存货科目余额取数口径」不被违反，因为余额来自 ledger 拥有的 trait 而不是本模块的第二处查询。09:90 末句关于「期望依赖清单」的表述按甲-3 改写。

**H-09（minor）——采纳，改的是投影函数的落点，不是 ep-adapter-search 的依赖集。** 03:116 与 03:122 的「`ep-adapter-search` 只依赖 `ep-foundation`」保留不动；改 05-master-data.md:79 与 12-service-project-asset.md:70：四类档案与价目表、五类服务与项目对象的 `SearchDocument` 投影函数落在拥有该 DTO 的模块的 `ep-app-*`（ep-app-mdm、ep-app-cpq、ep-app-service、ep-app-project），由 job-worker 的索引消费者调用后经 `SearchIndexPort` 写入。依据是甲-1，且这正是阶段 3 与总览已经写好的口径：03:58「本阶段不交付任何业务对象的检索文档投影函数，投影由各业务阶段按 `SearchDocument` 结构提供」、00-overview.md:189「各阶段只产出 SearchDocument，不自建写入路径」。否掉的替代路线是给 ep-adapter-search 加四条 contract 依赖：该边虽落在允许项内，但会让一个通用检索适配器随业务模块数线性扇入，且与两个阶段既有口径同时冲突。

#### 5. I-05、I-06、I-07 复核

三条均已在上一批清除，本裁定不含它们。I-05（06-contract-sales.md:777）已改为十项中的九项全部通过、`offsite-sink-requirements` 按阶段 1 计划整条推迟到阶段 14 并返回 `NOT_APPLICABLE`；其句末「按通则第六条取换判据一档」与 01-engineering-baseline.md:216 的「该项整条推迟」两套标签不一致，建议改为「自检项整条推迟，本条退出条件按通则第六条第二档换可判定替身」一句消歧，**是否必改标注为不确定**，不列为必改项。I-06（01:515）已改为「这一比对整条推迟到阶段 3a……本阶段只判该节存在」。I-07（01:506）已改为负样例一律以手写 SBOM 夹具构造、不因两包缺席而把断言留成恒真。

#### 6. 新增与净减

新增门禁规则 0 条（`platform-no-adapter` 计在 F-04 名下）、新增受限例外 0 条、新增白名单 0 条、新增错误码 0 个、新增 crate 0 个。**新增登记表行 1 行**：第 12.1 节 delegated 段的通道二角色约束一行，见通则乙；undecidable 段仍为空，阶段 1 archcheck 的通过态退出码仍为 0，不触发 RG-NO-UNDECIDABLE。净减两项：机制 −1（撤销「按 crate 逐项比对期望依赖清单」这一形态，收窄后只撤这一半，保留可判定的进程链接断言）；例外 −1（D-11-01 对第七条的单边界定作废）。新增的施工物属既有通道的常规用法，不计为机制：`ep_contract_inventory::StockValueOutboundPort` 一条 contract 端口，`ep-app-costing → ep-contract-inventory` 与 `ep-app-inventory → ep-contract-ledger` 两条落在允许项内的依赖边，`source.rs` 的 `schema_refs` 与 `one_schema_per_file` 一处判定实现改写（规则名与条数不变）。

## G 类 落位裁定

本节登记本轮新增的一条 G 类裁定，编号沿用附录丙的缺陷编号 G-01，不计入前文 67 条缺口。附录丙 G 类的其余五条（G-02 至 G-06）已由 F-01 一并处置，不在本节重复。按通则第一条，本裁定的权威落点在技术基线与阶段 7、8、9、10 计划正文，本节只作登记；其对 B-08 的修订以本文件 B-08 条目的确切标识符段为准。

### G-01 子账余额提供者的端口落位（修订裁定 B-08 的「确切标识符」段）

#### 争点

裁定 B-08 把 `SubledgerBalanceProvider` 定义在 `ep-contract-finance`（阶段 10 新增），把两个实现类型钉在 `ep-app-inventory`（阶段 8）与 `ep-app-procure`（阶段 7），又把两个 `impl` 判给阶段 10。trait 与类型对阶段 10 的任何 crate 双双是外部类型，`impl SubledgerBalanceProvider for InventorySubledgerBalanceQuery` 触发 E0117；而唯一可编译的落点（实现类型自己的 app crate）被计划自己排除，理由是阶段 8 与阶段 7 排在阶段 10 之前、届时 trait 尚不存在（08-inventory-costing.md:449「本阶段不依赖 ep-contract-finance」）。

#### 成因（本段按本轮复核重写，裁定原件的成因描述作废）

不是层位判错，也不是判据写错对象——七条禁止项一条也没被违反过，现行 18 条规则里没有孤儿规则面，`xtask archcheck` 对本条完全无感。也**不是**「端口停在了调用方的 contract crate」：A-13 与 A-15 的探针与登记表在全卷通行地由调用方持 trait，00b:113 逐字「模块间同步调用只能通过 ep-contract-B 中的 trait，实现在 apps 装配时注入」也不指定 B 必然是被调方，据此立论会与全卷既有形态互斥。

真正的成因是**端口的宿主 crate 的诞生阶段晚于实现方阶段**：`ep-contract-finance` 由阶段 10 新增，而两个实现方在阶段 8 与阶段 7。这一步之后全部困难都是派生的——实现方在其阶段无法实现，B-08 只好搬 `impl` 而不是搬 trait；搬到哪儿都不合法，因为孤儿规则要求 trait 与类型至少一头是本地的，阶段 10 两头皆外。

这一步同时撞穿三条已生效的约束，三条都写在计划自己身上：其一，本表通则第三条「跨模块同步调用的被调方必须与调用方同批交付。被调方阶段晚于调用方阶段的……」——按模块归属唯一判据，「该法人该期间的存货金额账合计」属 inventory、「已收货未收票暂估合计」属 procure，本条真实的被调方（8、7）本来就早于调用方（10），通则第三条根本不需要启用，是端口错位人为制造了一个「被调方在后」的假象；其二，08-inventory-costing.md:54 原文「本阶段在跨模块调用中一律是被调方在先的一侧……调用方阶段 6、阶段 7、阶段 10 各自接线并在其自身阶段完成该调用的验收」，B-08 让阶段 8 变成了阶段 10 的实现方，与这句自述互斥；其三，08:441 把该项与另外三项并列为「全部由其他阶段定义、本阶段实现」，而另外三项的定义方是阶段 5、阶段 5、阶段 9a，全部早于阶段 8，只有这一项的定义方在后——异常就摆在同一段里。

#### 结论

一、**撤销 `ep_contract_finance::SubledgerBalanceProvider`。** 该名全卷作废，任何阶段不得引用。计数按统一口径写：`ep-contract-finance` 的对外 trait 由 **11 个减为 10 个**，与 `ep-contract-invoice` 合计由 **16 个减为 15 个**；10-ar-ap-invoice.md:960 逐字「16 个 trait，定义在两个 contract crate 中」与 10:106 的合计口径须同批改到位，不得两处口径互斥。

二、**把端口移到被调方自己的 contract crate，一模块一个，与该模块既有端口同处一个 `port/` 目录。** 两个新端口，签名逐字如下，与被撤销的 `SubledgerBalanceProvider::balance` 语义与返回类型一致，只补齐 `&self` 与入参类型：

```rust
// crates/contract/inventory/src/port/subledger_balance.rs   阶段 8 定义
#[async_trait::async_trait]
pub trait StockValueSubledgerBalancePort: Send + Sync {
    async fn balance(&self, snapshot: &dyn SnapshotCtx,
                     legal_entity_id: Id<LegalEntity>,
                     accounting_period_id: Id<AccountingPeriod>,
                     accounting_period_seq: i32) -> Result<Money, AppError>;
}

// crates/contract/procure/src/port/subledger_balance.rs     阶段 7 定义
#[async_trait::async_trait]
pub trait GrniSubledgerBalancePort: Send + Sync {
    async fn balance(&self, snapshot: &dyn SnapshotCtx,
                     legal_entity_id: Id<LegalEntity>,
                     accounting_period_id: Id<AccountingPeriod>,
                     accounting_period_seq: i32) -> Result<Money, AppError>;
}
```

F-51 已把两个端口的累计顺序冻结为 `accounting_period_seq <= target_seq`；本段此前仅带期间 UUID 的历史签名已被上面的四参终态替代，禁止按 UUID 比较或省略 seq。

`SnapshotCtx`、`Id`、`LegalEntity`、`AccountingPeriod`、`Money`、`AppError` 六者全部取自 `ep-foundation`（`AccountingPeriod` 与 `LegalEntity` 在 00b:167 的 22 项标记清单内），两个 contract crate 仍只依赖 ep-foundation，禁止项第三条不受影响。加上【本轮 F-05 H-05】落在同一目录的 `StockValueOutboundPort`，`ep-contract-inventory` 的对外 trait 由 5 个增为 7 个，`ep-contract-procure` 由 5 个增为 6 个；00-overview.md:101、08:20、08:501 与 08:503 四处计数须一次改到位，不得分两批。

三、**实现类型名与位置不变，`impl` 与类型同 crate。** `InventorySubledgerBalanceQuery` 位于 `crates/application/inventory/src/projection/subledger_balance.rs`，由阶段 8 实现 `ep_contract_inventory::StockValueSubledgerBalancePort`；`GrniSubledgerBalanceQuery` 位于 `crates/application/procure/src/projection/subledger_balance.rs`（本裁定补钉该位置，原计划只给名字未给位置），由阶段 7 实现 `ep_contract_procure::GrniSubledgerBalancePort`。两处 trait 均为外部、类型为本地，孤儿规则成立。「以查询函数形式先行交付、由阶段 10 包装」这一说法在全卷作废——不存在包装，实现方直接实现；08:439 的小节标题「本阶段实现的外部 trait 与查询函数」同批删去「与查询函数」，10:1273 的「子账侧包装阶段 7 提供的查询函数」一并改写。

四、**装配与消费。** 阶段 10 的 `ep-app-finance` 依赖 `ep-contract-inventory` 与 `ep-contract-procure`（允许项明写 `ep-app-<m>` 可依赖任意 `ep-contract-*`），以 `Arc<dyn StockValueSubledgerBalancePort>` 与 `Arc<dyn GrniSubledgerBalancePort>` 两个注入点组装 `ReconciliationItemQuery` 十项中的两项，注入行由阶段 10 写入 `apps/core-server/src/wiring/` 与 `apps/job-worker/src/wiring/` 两个目录——这与 08:54「调用方……各自接线并在其自身阶段完成该调用的验收」一致，阶段 8 与阶段 7 不为调用方预留任何占位实现。B-08 其余结论全部保留：跨阶段唯一取数入口仍是 `ep_contract_finance::ReconciliationItemQuery`，阶段 9b 的关账前强制校验与其 `ReconCheck` 一律调用它，十项中的其余八项仍取自阶段 10 自有表。10-ar-ap-invoice.md:147 的排他句「跨模块只由 `ep-app-invoice` 依赖 `ep-contract-finance`，反向不成立」必须同批改写，不能只在句末追加；08-inventory-costing.md:53 的「procure、sales、invoice 三个模块的 application crate 依赖 ep-contract-inventory」须补 `finance`，裁定后它是第四个。

五、**不新增任何机制。** 不新增受限例外、不新增门禁规则、不新增白名单、不新增登记段，`xtask archcheck` 零改动，允许项七条与禁止项七条一字不改，`FORBIDDEN_RULES` 七项不变。裁定后涉及四条边：`ep-app-finance → ep-contract-inventory`、`ep-app-finance → ep-contract-procure`、`ep-app-inventory → ep-contract-inventory`、`ep-app-procure → ep-contract-procure`，其中 **`ep-app-finance → ep-contract-procure` 先于本裁定已存在**（10:994 逐字「付款申请单已付金额回写经 `ep-contract-procure` 写端口」），本裁定实际新增的只有三条，四条全部落在允许项内，`app-no-peer-app` 与 `domain-contract-no-io` 均不触发，platform 无关。

#### 裁定理由

缺陷的可修面只有三个：搬 impl、搬类型、搬 trait。搬 impl 是 B-08 已经走过的路，全工作区没有合法落点（放 ep-adapter-db-pg 也不行，允许项明写 adapter 不得依赖 application，两个类型不可命名）。搬类型（把两个实现类型挪进 ep-contract-finance 或阶段 10 的 app crate）等于把 inventory 与 procure 的取数逻辑搬进 finance，直接违反禁止项第七条，与 F-05 正在裁的 H-05 是同一个错。只剩搬 trait；trait 搬到哪里由「谁是被调方」决定，而被调方由本表通则第二条的模块归属唯一判据决定，判下来就是 inventory 与 procure。

**替代方案一：把 `ep-contract-finance` 的诞生提前到阶段 7 之前，让阶段 8 与阶段 7 直接实现原 trait。**（本段理由按本轮复核重写。）该方案确实正面对上了成因——宿主 crate 诞生太晚——技术上也可行、能编译。否掉的理由是另外三条：其一，它造出一个「归属阶段与创建阶段不同」的 crate，是全卷唯一一例，等于新增一条隐性机制；其二，它要求阶段 7 与阶段 8 冻结一个 finance 语义的签名，而 finance 的对账口径要到阶段 10 才成形，签名冻早了必然回改；其三，它保留了「被调方实现调用方的 trait」这一倒置形态，与 08:54 的自述、与本表通则第三条的方向感继续互斥。相形之下，把端口移到被调方 crate 后，宿主诞生阶段（8、7）天然早于调用方（10），这条时序矛盾自动消失，不需要为一个 crate 单独安排诞生时点。

**替代方案二（最强的竞争者）：只建一个 trait，放进 `ep-contract-ledger`。** 阶段 9a 早于 8、7、10，一个 trait 就能覆盖两个实现方，而且它与 `TotalAccountBalanceProvider` 恰好是子账侧与总账侧的对称一对，比本裁定少一个 trait。否掉的理由是一句硬话：09-ledger-period.md:458 结尾原文「本阶段不定义总账侧接口之外的任何东西。」——把子账侧端口塞进 ep-contract-ledger 需要先破这句围栏，即为一条边开一处例外；本项目的教训恰在于此，A-01 标记类型那处例外的围栏已经裂成两套互斥措辞。用「少一个 trait」换「多一处被破的围栏」不划算。须诚实说明：裁定原件另举的附带成本「该方案要求 ep-app-inventory 新增对 ep-contract-ledger 的依赖、需连改 08:51 的自查清单」**已不再成立**——F-05 的 H-08 已按另一理由（`TotalAccountBalanceProvider`）为 08:51 补入该依赖，故本方案的否决理由以 09:458 的围栏一条为准；同时 09:90 所依据的「CI 按期望依赖清单比对」形态已由 F-05 通则甲-3 撤销，不再构成任何一侧的论据。

**替代方案三：把 trait 下沉 ep-foundation。** 直接被禁止项第六条挡下——子账余额是业务概念，00b:117 明写跨模块共享的业务形状不进 foundation、定义在拥有它的模块的 `ep-contract-*` 里；且 F-03 之后必要性一条已降为评审判据，再往 foundation 塞东西正是 F-05 的 H-07 判掉的形态。不考虑。

**为什么多出一个 trait 是可接受的代价。** 净变化是 −1（finance）+2（inventory、procure）= +1 个 trait，换掉的是：一条不可编译的 impl 安排、一处「反向依赖」的特设措辞、一处与 08:54 自述的互斥、一处与通则第三条方向感的互斥。两个 trait 结构完全对称、签名逐字相同、各自与本模块既有端口同处一个 `port/` 目录。代价只在可扩展性口径上轻微下降：新增第三个子账来源时，要在该来源模块的 contract 里加一个同形端口，而不是「追加一个实现」（10:1308 需按此改写）——这是诚实的，新来源本来就是一个新模块的新被调方。

**与 F-01、F-03、F-04 的相容度。** 不冲突，也不援引它们没开的例外。本条的两个端口不在 F-01 通用条款按 trait 名限定的三个之内；F-04 已把该条款扩面到 `ep_foundation::port::*` 全体，本条的两个端口不在 `ep-foundation` 内，因此仍不受其覆盖。F-03 只动禁止项第六条与第 12 节，本条不碰第六条、不新增 delegated 或 undecidable 登记行，`undecidable-registry-matched` 的逐行比对不受影响。

#### 待决项

~~U-G01-01（已由 F-51 关闭）~~：GRNI 只读 procure 自有的 `goods_receipt_line_costings` 追加效果，按截至目标 `accounting_period_seq` 累计 `INCREASE.amount-DECREASE.amount`；阶段 10 只能经 `GrniEffectWritebackPort` 同事务追加发票/红字效果，不能跨 schema 反查或用今天的 `invoiced_quantity` 倒推历史。

### F-06　阶段 13 插件并发限流不开降级窗口

**争点（历史计数已由 F-55 终态裁定取代）。** 阶段 13 第 11.1 节技术风险表曾承诺「连续限流……登记降级窗口，
降级类别取阶段 14 冻结清单之一」。F-06 作成时终态清单为十八个 `DegradationKind`；F-52 为周期核对无结论新增第十九项 `REPLICATION_CROSSCHECK_NO_RESULT`，F-53 再为病毒扫描新增第二十项 `VIRUS_SCANNER_NOT_AVAILABLE`；这组 18→19→20 只记录历史演进，不是可实施的阶段顺序。现行实现必须由阶段 2 的三项起步，再由阶段 14 的 `V20261023092500` 扩为含 `LEGAL_ENTITY_KEY_DOMAIN_UNAVAILABLE` 的终态 21 项；21 项中仍没有任何限流或配额类。

**结论：删该句承诺，不新增取值。**

理由一，该句在现行取值域下**不可满足**。逐个核过终态 21 个取值：落点未配置、写出进程未投入运行、
端口未交付、三类写出超期、引导窗口超出、RPO 未达成、WAF 未配置、锚定超期、副本保护缺失、
归档槽保留告警、归档链断裂、对账未完成、关账受理被拒、授权快照校验和不符、
自定义对象 DDL 不一致、周期核对连续无结论、病毒扫描器不可用，加阶段 2 自身触发的三个取值。无一适用于「一次调用被限流」。
`PORT_NOT_IMPLEMENTED` 这条支路也不可用：阶段 2 已把它的适用面逐字封闭为
「只供 `WasmComputePort`、`RuleEvaluator` 与 `DisposalPort` 三项末期平台能力」，
挂上去是把已封闭的适用面重新打开，属新增例外。

理由二，删除后调用方并非无处承载。四件既有承载全部就位：
`platform_meta.extension_invocations` 的 outcome 取值 `THROTTLED` 逐笔落行；
`PLATFORM.EXTENSION.HOST_UNAVAILABLE` 按基线第 5 节为 `INFRASTRUCTURE`、HTTP 429、可重试；
阶段 13 第 10 节已写「插件调用被限流与被资源上限中止的事件记入运维中心」；
本文件作废名清单已就应用层限流指定去向为「计入附录 A.2 错误率口径」。

理由三，按 F-06 作成时的历史计数，要使原句可满足本应扩到第十九个取值；在 F-55 的终态清单上则须新增第二十二项，并同批改阶段 14 的 3→21 扩容迁移、终态 Rust 枚举与数据字典；阶段 2 首次建表的三项 CHECK 不得因此改写。另要**发明一个「连续限流」的阈值**——全卷无此阈值，
而同阶段第 4.8 节的自动停用有明确阈值配置项可作对照。按改动面取删除支。

**本裁定明确不主张的两件事。** 其一，不主张「降级窗口只用于部署面或配置面的持续状态」
这条通例——阶段 9 有两处反例：期间关账受理连续两次被拒开窗，
单查询资源上限触发终止即开窗。其二，阶段 11 第 4.7 节「超限终止不登记降级窗口」
只作为本卷已有同形态先例的存在性证明引用，不得据以推出通例。

**残留风险，如实登记。** 本条的分支选择不是唯一解。阶段 9 证明「连续 N 次即开窗」
在本卷是既有形态，「发明阈值」只是一个数而非一套机制，故增取值一支的代价被低估过。
本裁定按改动面与「无任何现存取值适用」两点取删除支；
**若产品侧认为插件过载须以窗口对外可见，本条应重开。**

新增机制 0 个，新增例外 0 条，新增 `DegradationKind` 取值 0 个，代码改动 0 处。

### F-07　A-24 的「八个勾稽视图」是旧值，改十

**争点。** A-24 写「两侧平衡由 finance 的八个勾稽视图在首个会计期间校验」，
而阶段 10 与本文件另两处均为十项勾稽。

**结论：五处「八」全部改「十」。** 不改成别的数，也不改成不带数的措辞。

理由。「八」是 B-08 生效前那次划分的残数：该划分原写「十个勾稽项对应十个对账视图，
本阶段建其中八个的完整实现，另两个只建外壳」，其后同批改为「全部完整实现」，
而 A-24 这一处未随改。此前怀疑「八」是刻意排除存货与已收货未收票两项
（其子账侧来自外部端口），核后不成立：期初通道的两侧平衡校验按阶段 10
已有的十项口径承接，无八项口径。

落点五处：总览 A-24 行、本文件 A-24 确切标识符段、阶段 10 第 4 节期初通道段、
阶段 10 退出条件 24 与退出条件 28。

新增机制 0 个，新增例外 0 条，代码改动 0 处。

### F-08　服务端交付目标改为 Windows Server 原生（2019 至 2022，认证基线冻结在 2022）

**裁定方向：规格让步（本轮由使用方授权改规格），计划与代码随之重取承载物；四个机制里有三条机制保证在本平台不再成立，一律删除而不换等价物。**

> **CPU/IO 比例机制的现行终局覆盖。** 本节形成时曾把 CPU 比例写成“待实测可上调”，该支路现已关闭：首版 CPU 比例与 CPU 突发上限只作硬件标定/认证意图，不写静态限额、不调用 Job Object CPU rate API；按权重磁盘 IO 份额固定不启用。后文所有“待实测可上调”“实测通过后恢复判据”或仍待产品负责人选择的字面只作 F-08 论证追溯，不是当前实现分支。未来启用必须另立产品版本、正式裁定、配置 schema 与 Windows 实机发布证据。

#### 零、使用方已定的四条前提与本裁定的授权边界

前提（不在本裁定的论证范围内，本裁定只承接）：

1. 服务端交付目标由「x86_64 加主流企业 Linux 发行版加 Docker Compose 或 Podman 加 systemd」改为 **Windows Server 原生**，不许 Linux 虚拟机层、不许 WSL、不许 Linux 容器。
2. 目标版本区间 **Windows Server 2019 至 2022**。
3. **认证基线 BC-1 的操作系统取值冻结在 Windows Server 2022**；2019 按规格现有的国产 Linux 同一形态处理——可以在同一形态上运行，但**不在首版认证组合内，也不在附录 D.3 的单维度替换清单内**。
4. 核心交易数据库 PostgreSQL 16 不变；客户端四端与供应商门户浏览器端不变。

授权边界（先划清，否则下面每一条都会被读大）。本轮授权覆盖的是**因平台改变而失去承载物的条款**：条款所依赖的内核机制在本平台不存在，因此该条款必须降级、换手段或宣告不再成立。本轮授权**不**覆盖三件事：

1. 不覆盖「因改动面大而删规格第 13.1 章配额表」。该问题现已随总体规格第 13.1 章的终局回写关闭：表保留为硬件标定与认证意图，首版运行期只承载内存硬上限；本裁定的处置是**保留该表、改变它的效力**（见 2.1），不是删表，也不存在等待产品负责人选择的当前分支。
2. 不覆盖任何要求修改客户机器全机系统设置的处置（典型是 `WaitToKillServiceTimeout` 与 `LongPathsEnabled` 两个注册表值）。凡处置落到「改客户机器的系统设置」，本裁定一律判为**做不到**，不判为「有条件可做」。
3. 不覆盖国产化替代路径与等级保护三级对外表述的产品决策，见第九节第 1 条。

#### 一、版本区间与认证冻结点单独带来的三条硬结论

这三条不是「Windows 与 Linux 有何不同」，而是「区间取 2019 至 2022、认证冻结在 2022」这一取值本身推出的，与机制替换无关，因此单列在最前。

**结论一：制品形态被定死为原生 Windows 服务，容器一支在本区间内不成立。** 进程隔离模式的 Windows 容器要求主机与容器基础镜像的版本相匹配，一份制品无法同时在 2019 与 2022 上以进程隔离运行；要覆盖区间就只能按版本各出一份镜像，与「同一制品、同一签名、同一 SBOM」的交付口径冲突。据此 `spec:1184` 逐字「全部组件以标准 OCI 容器交付」在本区间内**不成立**，2.2 取原生服务不是三选一里的偏好，是区间取值的结果。第九节第 1 条的「Windows 容器是否算原生」这一问因此**只剩理论意义**：即便使用方裁定它在范围内，它也过不了区间这一关，除非同时把区间收窄到单一版本。

**结论二：2019 上的实测数据不进认证报告。** 认证跑在 2022 上，2019 与国产 Linux 同一形态。据此：附录 A.4 的性能与容量基线、附录 A.6 的两类恢复演练、第 17.5 章的认证结论，其被测机器一律是 Server 2022；在 2019 上跑出的任何数字都不得写入认证报告，也不得据以声明 2019 已认证。若日后某客户需要 2019 的背书，须另立一次认证运行——这不是缺陷，是选 2022 的对价，换来的是认证有效期覆盖到 2022 的扩展支持终点而不是 2019 的。**该对价须写入交付说明，不得沉默。**

**结论三：区间内的机制可用面是齐的，区间不构成新的能力缺口。** 本裁定用到的 Windows 机制逐条核过其引入版本，全部早于 2019：Job Object 的 CPU 速率控制三模式（权重、硬上限、最小／最大速率）与 IO 速率控制均自 Server 2016 起提供；服务虚拟账户 `NT SERVICE\<名>` 与每服务 SID 自 Server 2008 R2 起提供；命名管道与其安全描述符自始提供。**因此第二节的九条「做不到」没有一条是「2019 上没有、2022 上有」，全部是 Windows 与 Linux 之间的差异**，收窄区间到 2022 一个版本也补不上任何一条。这一点很要紧：它意味着区间取值与机制裁定两件事互不牵扯，第三节的实测清单只需在 2022 上跑一遍，再在 2019 上做一次同项复核即可，不需要按版本各推一遍论证。

一处例外须点名：`\\.\pipe\` 名字空间的准入问题（做不到九）与 AF_UNIX 在 Server 2019 起于操作系统层可用这一事实**看似相关、实则无关**——tokio 的 `cfg_net_unix!` 门是 `#[cfg(all(unix, feature = "net"))]`，底层 mio 同样是 `#[cfg(unix)]`，`tokio::net::UnixStream` 在 Windows 目标上**不存在这个符号**，与操作系统是否支持 AF_UNIX 无关。要用它就得自建一套 IO 驱动，收益只是省掉改动。这一支**明确不取**，本条写下来是为了让「2019 起 Windows 也有 Unix socket」这个正确但无用的事实不再被重新提起。

#### 二、做不到的，先说（九条；每条给出它承接的条款与处置，不给等价物）

**做不到一：磁盘 IO 的按权重比例份额。整条不成立。**

承接条款：`spec:1135` 逐字「磁盘 IO 用 io.weight 表达份额、io.max 表达突发上限」；`spec:1137-1147` 配额表第三列九行；`spec:1150` 的借用与收敛语义在 IO 侧的部分；`spec:1157` 机制一在 IO 侧的两条含义；`spec:1160` 逐字「其份额之内的磁盘 IO 不向任何级别让路，含第 1 级，由该 cgroup 的 io.weight 落实」；`spec:1682` 复述同一句；己-1 第二节第 3 类的八个 `IOWeight`。

事实：Job Object 的 IO 速率控制给的是 `MaxIops`／`MaxBandwidth` 绝对上限与 `ReservationIops` 固定预留，是 cap 与 fixed reservation，**不是按权重的比例分配**，没有「其余组件空闲时借用、被借用方需要时按权重收敛」这一语义。（不确定：该机制在本地 NTFS 直连卷上的实际覆盖面与 `ReservationIops` 的保证强度，未实测，见第十节第 3 项。）另有一条独立于该不确定性的硬理由：规格该列是**百分比**，而 `ReservationIops` 是绝对 IOPS，两者之间的折算需要机器的总 IOPS，即机器相关值——己-1 第四节理由 2 已把「机器相关值不得进入两个权重列」定死，己-1 第二节又已删除「按可分配量折算的生成算法」。即便实测可靠，落地它也要复活一条刚被删掉的折算算法。

处置：**IO 份额一列在本平台无运行期承载，删除其机制保证**。`spec:1160` 的机制保证句整句删除，第 13.3 章 RPO 不超过 15 分钟的成立**完全押在机制四（附录 A.4 认证实测）**上；`spec:1682` 与 PRD 11.11 段的残余风险上界同批写严，明写该必要条件在本平台没有机制侧保证。八个 drop-in 的 `IOWeight` 一列**不迁移**。**明确禁止**：不得把 `ReservationIops` 写成 `io.weight` 的等价物，不得把 `MaxBandwidth` 写成「份额」。

**做不到二：`memory.low` 的回收保护下界。**

承接条款：`spec:1151` 逐字「内存的保底值与上限同值，即 memory.low 与 memory.max 取同一取值」；己-1 第二节第 1 类的八个 `MemoryLow`。

事实：Job Object 的 `JOB_OBJECT_LIMIT_JOB_MEMORY` 是提交内存硬上限，可承接 `MemoryMax`；`memory.low` 那种「内存压力下优先不回收」的软保底在 Windows 上没有对应物。`SetProcessWorkingSetSizeEx` 的最小工作集是工作集修剪下界，不是提交量保底，也不参与整机内存压力仲裁。

处置：删 `MemoryLow` 一列，`spec:1151` 改写为单值硬上限。**语义净损失小但不是零**：因该表保底与上限同值，被删掉的只是「压力下优先不回收」这半条；同时触限行为不同——`memory.max` 触限走内核终止进程，Job Object 触限是分配失败返回错误，`spec:1151` 上文「内存超售会触发内核终止进程」的理由句要一并改。**不得拿工作集下限冒充 `MemoryLow`。**

**做不到三：「保底份额被击穿」的三个观测量。判据整体消失。**

承接条款：`spec:1170` 逐字「在 cgroup 侧表现为 cpu.stat 的 throttled 计数上升、memory.events 的 low 事件或 io.stat 的排队时延超过阈值；两个条件同时成立才判定为击穿」；`spec:1684` 控制段的同口径句；第 15.3 章降级与暴露窗口台账的该类条目。

事实：Job Object 的查询接口提供基础记账与 IO 计数，没有节流计数、没有 low 事件（因为没有 `memory.low`）、没有 IO 排队时延。三个被测量全无对应物。

处置：**删除「保底份额被击穿」这一类事件**，第 15.3 章台账与 `spec:1684` 控制段只保留「配额触发限流」与「后台任务被让路延迟」两类，并在两处明写删除理由。按本卷附录戊四已有先例——把一个做不可靠的门禁写进退出条件，比没有门禁更坏——**不得换一组看似对应的 Windows 计数器凑数**。附带一条：己-1 第二节已在计划侧删掉该双条件判定，其删除理由（判据恒不命中）在本平台被一个更强的理由取代（被测量根本不存在），计划侧不需再动，规格侧现在才补上。

**做不到四：关机路径上超出 `WaitToKillServiceTimeout` 的排空。**

承接条款：`01:338` 逐字「上限由配置控制默认 30 秒……systemd 的 TimeoutStopSec 取 45 秒」；`deploy/ORCHESTRATION.md` 第六节第 3 条的 45 秒余量论证；D-02 判据（`01:20`）逐字「SIGTERM 后 30 秒内退出码 0」。

事实：显式停止路径（`sc stop`）上，服务只要持续抬 `dwCheckPoint`，服务控制管理器不强杀，30 秒排空成立，且比 systemd 宽松；**机器关机路径**上等待受 `HKLM\SYSTEM\CurrentControlSet\Control\WaitToKillServiceTimeout` 约束，该值远小于 30 秒且是全机注册表值（不确定：本区间两个版本的当前默认值，以及它是每服务预算还是全部服务的总预算，未核实，第十节第 9 项）。拉长它要改客户机器的系统设置，按第零节被本裁定排除。

处置：`01:338` 与 D-02 判据**拆成两条路径分别表述**：显式停止路径的 30 秒排空成立并可判；关机路径不成立，如实登记为交付说明中的一条降级。不得以「一般不会在关机时有在途请求」把它写掉。

**做不到五：按退出码取值分流重启（暂判做不到，附一条可翻案的实测）。**

承接条款：`01:197` 逐字「Failed 状态下 systemd 以 `RestartPreventExitStatus=78` 不重启，避免配置错误导致重启风暴；退出码 70 允许重启」；`01:193-195` 状态机三行；E2E-03（`01:459`）、E2E-04（`01:460`）。

事实：服务控制管理器的失败恢复只有一个布尔 `SERVICE_CONFIG_FAILURE_ACTIONS_FLAG` 加「是否报告了 `SERVICE_STOPPED`、报告码是否为 0」的二值判据，**没有按退出码取值的白名单或黑名单**（不确定：未逐项核实本区间的服务配置项全集，判定「无」基于我所知的接口面）。

处置：给出**一条主承载与一条降级备选**，并把选择权交给一次实测而不是交给措辞：

- 主承载：配置错误路径（78）由服务宿主正常报告 `SERVICE_STOPPED` 且 `dwWin32ExitCode` 取 78，管理器在默认布尔（假）下不触发恢复动作 → 不重启，正确；panic 路径（70）**不报告 `SERVICE_STOPPED`，直接以 70 终止进程**，管理器判为崩溃并按 `sc failure` 恢复 → 重启，正确。两个退出码对外仍可见（夹具经预先持有的进程句柄 `GetExitCodeProcess` 可断言真实码）。
- 该主承载成立的前提是一条未实测的事实：**服务进程未报告 `SERVICE_STOPPED` 即退出时，管理器判为崩溃并执行恢复动作**（第十节第 5 项）。
- 若实测不成立，本条即为**做不到**，按备选执行：保住 78／70 两个退出码的对外可见性，放弃管理器侧对 70 的自动重启，并把「panic 后不自动重启」如实写进交付说明。**两支都不得写成「Windows 上用 X 等价实现」。**

**做不到六：`Type=oneshot` 语义。**

承接条款：`01:592` 逐字「`tools/ep-migrate` 是一次性运维工具，随制品交付，以 systemd 的 oneshot 单元在升级窗口内执行」；`02:885` 的理由句逐字「理由是它不常驻、无 systemd 单元、无 cgroup slice」。

事实：服务控制管理器没有「起来做完就退出且算成功」的服务类型，也没有 `RemainAfterExit`。

处置：`ep-migrate` **不注册为服务**，由升级脚本以 `ep-migrate` 账户直接拉起并等其退出，退出码原样判定。`02:885` 的**结论不变**（它不是八进程之一、不常驻、独立账户，不构成基线第 12 节所禁止的新增进程），**理由句换**：「无 systemd 单元、无 cgroup slice」换成「不注册为 Windows 服务、不占用任何资源单位」。

**做不到七：`00b:641` 的「输出到 stdout，由 systemd 收集。不自建日志平台」。**

事实：服务控制管理器起的服务不继承控制台，stdout 无采集方。两条替代都带代价：写文件加自轮转等于自建了一个最小日志落地层（轮转、磁盘配额、并发写入是三个新失效面）；写 Windows 事件日志则 `00b:642` 的 18 个固定字段只能塞进消息字符串，其机检性质变弱（不确定：单条事件的大小上限与结构化字段支持程度，未核实，第十节第 11 项）。

处置：取**写本地文件加自轮转**，`00b:641` 整句重写，并**明写这是「不自建日志平台」一句的降级**：本平台自建的是日志落地与轮转一层，不含检索、聚合与告警。不得只换个词把这句留着。

**做不到八：「两套等价编排」这个交付物形态与其等价性核对。**

承接条款：D-05（`01:23`）逐字「`deploy/` 下的 Podman Quadlet 与 Docker Compose 两套等价文件，含八个 slice 与配额」；`scripts/verify-orchestration-equivalence.py` 十三条规则中以「两侧」为前提的六条；D-13（`01:31`）的 `scripts/dev-up.sh`／`dev-down.sh` 起的是同一份 `compose.yaml`。

事实：Windows 侧只有一套载体，**被比对的第二方不存在了**。这不是把脚本改写成 PowerShell 的问题。

处置：等价性核对这条判据**撤下**，`deploy/podman/` 下 9 个 `.container` 与 5 个 `.volume`、`deploy/compose/compose.yaml`、`scripts/verify-orchestration-equivalence.py` 与其负样例脚本一并失去对象。**不得造一个只有一侧的「等价性」脚本**。D-05 的判据改为「一条命令起全栈，`sc query` 九个服务全部 RUNNING」。附带损失一条如实登记：Compose 一侧的 `depends_on` 加 `condition: service_healthy` 是两套里唯一有**就绪门槛**的一支，`sc config depend=` 只有「依赖服务进入 RUNNING」，本次退化掉的正是较强的那一支，`deploy/ORCHESTRATION.md` 第五节已如实写过这个差别，现在它变成永久状态。

**做不到九（本次变更新开的一格，不是原有条款的降级）：命名管道名字空间没有创建侧准入控制。**

Linux 侧的遏制是目录权限：`/run/ep/ipc` 按 `00b:266` 设属主与组 `ep`，非该组进程连在该目录建一个 socket 文件都做不到。Windows 的 `\\.\pipe\` 是平坦名字空间，据我所知任何本地用户的任何进程都可以创建任何尚不存在的管道名，且没有受支持的手段对某个名字前缀施加创建侧 ACL。于是一个**非特权**本地用户可在服务启动前占住 `\\.\pipe\ep-core`，两个写出进程随后连上去的就是他，`00b:266` 枚举的四类上报落到他手里，真实的记名审计从此不再产生。

**这个攻击面的门槛低于规格第 21.18 章与第 7.7 章的口径**（那两处讲的是「持有该服务器操作系统权限者」），本条只要求非特权本地执行，必须**与 21.18 并列登记为一条新增残余风险，不得并入**。

能做的三件都不是等价物，如实这么写：`ServerOptions::first_pipe_instance(true)` 让名字被抢时服务**启动失败**（fail-closed，不是防护，但必设——不设的话第二个进程可为同一名字追加实例并分走一部分连接，比启动失败更坏）；客户端连上后经 `GetNamedPipeServerProcessId` 核对服务端账户（先连后核，有时序窗口）；其余进交付说明。（不确定：不能排除存在未知的 NPFS 根对象 ACL 或组策略手段；若使用方能找到，本条应撤回。第十节第 7 项专测此事。）

#### 三、本次变更顺带查出的四条现存缺陷（与裁定方向无关，无论如何都要改）

这四条不是「Windows 上做不到」，是**仓库里已经写错或漏写的东西，只不过要到 Windows 上才显形**。按本卷纪律，查出即改，不随裁定落地排期。

1. **应急账号在本平台被自己锁死。** `db/bootstrap/04_pg_hba.fragment:33-34` 实测只有一行 `local all ep_breakglass scram-sha-256`。Windows 版 PostgreSQL 无 Unix 域套接字，该行**不匹配任何连接**，应急账号在本平台完全无法登录——而它存在的全部意义就是别的路都断了的时候还能进去。须补 `host all ep_breakglass 127.0.0.1/32` 与 `::1/128` 两行。同一成因的还有两个复制角色的 `local replication` 放行行（`:22`、`:28`）。
2. **IPv6 回环被自己的 reject 行拒掉。** 同文件 `:23-25` 与 `:29-31` 放行了 `127.0.0.1/32` 却没有放行 `::1/128`，紧接着 `host ... ::/0 reject`。Windows 把 `localhost` 优先解析为 `::1`，凡以主机名 `localhost` 发起的复制连接会被自己的规则拒绝。**这一条在 Linux 上同样是缺陷**，只是 Linux 侧默认走 Unix 套接字把它盖住了。须为两个角色各补一行 `::1/128` 放行，排在 `::/0 reject` 之前。
3. **迁移校验和没有换行符护栏。** `tools/migrate/src/history.rs:53-58` 的 `migration_checksum` 直接对 SQL 正文求哈希，而全仓**没有 `.gitattributes`**。Windows 上一次 CRLF 转换就会让全部 69 个已应用迁移的校验和不符，`ep-migrate apply` 以退出码 4 拒绝启动。须建 `.gitattributes` 把 `*.sql` 与迁移目录钉死为 LF。**本条本轮即改，不等裁定落地**——它零风险、零语义变化，且是唯一一条会让「换台机器 clone 一次」就炸掉的。
4. **WAL 落盘方式曾落到不安全的默认值，现已关闭。** `db/bootstrap/02_cluster_params.sql` 的首版唯一值已冻结为 `wal_sync_method = 'fsync_writethrough'`，Windows 默认 `open_datasync` 不得使用；附录 A.4 只实测其性能代价与断电恢复证据，不再从多个方法中选值。

#### 四、逐机制裁定

##### 4.1 机制一　资源仲裁

**承载物：具名 Job Object，由服务宿主层在 `ServiceMain` 早期读取部署侧静态限额文件后创建或打开并自我指派。取值仍来自 `deploy/` 下的静态文件，不做生成算法。**

四类取值的存活情况（对照己-1 第二节逐条）：

| 己-1 的四类 | 本平台处置 | 依据 |
|---|---|---|
| 第 1 类　`MemoryMax` | **保留**，落 `JOB_OBJECT_LIMIT_JOB_MEMORY`，绝对字节按 BC-1 算定不变 | 唯一可原样落地的一列 |
| 第 1 类　`MemoryLow`（同值） | **删除** | 做不到二 |
| 第 2 类　`CPUWeight` | **固定为标定与认证意图声明，不落运行期取值**；首版不启用 | 现行终局覆盖 |
| 第 3 类　`IOWeight` | **删除** | 做不到一 |
| 第 4 类　backup-writer 的 `IOMax` | **保留待实测**，`IOMax` 是 MB/s 绝对值，与 `MaxBandwidth` 同形状，不需折算 | 第十节第 3 项 |

CPU 一列的历史模式比较保留如下，仅解释为何本版不启用，不提供自动翻牌条件：Job Object 的 CPU 速率控制有三个互斥模式——权重模式（取值域据文档口径为 1 至 9，最大可表达比值 9 比 1，而本表 44% 比 2% 需要 22 比 1，**表达不了**）、硬上限模式（百分之一百分点粒度，精度够但空闲容量不被借用，与 `spec:1150` 的借用语义相反）、以及**最小／最大速率模式**（`MinRate`／`MaxRate`，同为百分之一百分点粒度）。第三个模式即使经实测可用，也不得在首版自动落值或恢复判据；CPU 一列固定只作意图声明，未来启用须按本节开头的终局覆盖另立版本与裁定。

**「谁把进程放进资源单位」这一问的裁定。** 服务控制管理器没有 `Slice=` 的对应键（不确定：未核实服务配置项全集），`deploy/podman/core-server.container` 的 `Slice=app-core.slice` 一行没有落点。三条路里：新建一个常驻的 job 句柄持有者进程会触及 `00b` 第 12 节的新增进程禁令与八进程清单，不取；安装器创建具名 job 后退出会让该内核对象随最后一个句柄消失，不取；**取服务宿主层自我指派**。三条理由：不新增常驻进程；取值仍是部署侧静态文件，不构成 `00b:263` 逐字所禁的「配额生成算法」；具名 job 使运行期取值可被外部核对进程读回（前提是 DACL 授予 `JOB_OBJECT_QUERY`，第十节第 4 项），这是保住 E2E-05 那个替身被测对象的唯一办法。

同批补一句到 `00b:263`：**承载物由服务宿主层落实，不由编排层落实**。D-06 逐字「任何进程的启动自检中不出现资源限额项」**原样保留**——自我指派发生在服务宿主层，不是自检项注册表里的一项，`--check` 的九份报告一行不变。

##### 4.2 机制二　进程编排与生命周期

**承载物：Windows 服务控制管理器原生服务，八个进程各注册一个服务，外加一层八个二进制共用的服务宿主。不取 Windows 容器（第一节结论一），不取第三方服务包装器，不取任务计划程序（`ep-migrate` 除外可用其承接一次性执行）。**

理由三条：其一，第一节结论一已把容器一支排除在区间之外，管理器是唯一无争议落在线内的形态。其二，它是候选里唯一同时给到开机自启（`SERVICE_AUTO_START`）、崩溃重启（`sc failure` 三档动作加复位窗口）、依赖顺序（`sc config depend=`）、每服务独立无口令身份（虚拟账户 `NT SERVICE\<名>`，自带每服务 SID）、且允许长排空（显式停止路径上抬 checkpoint 即不被强杀）的机制。其三，它对现有 Rust 代码的改动面最小——真正 Unix 专有的只有 `crates/platform/runtime/src/shutdown.rs` 一个函数。第三方包装器（NSSM 一类）能补上退出码分流与 stdout 两个缺口，但用一个维护状态存疑的第三方常驻二进制去补两条可以如实披露的缺口不划算，且它会把「服务状态」这个判据变成隔一层的间接观测，**明确不取**。

配套四件，缺一不可：

1. 新增服务宿主层（`StartServiceCtrlDispatcher` 加 `ServiceMain` 加 `HandlerEx`，兼 4.1 的 Job Object 自我指派），八个二进制共用，并保留控制台直跑模式供开发与集成测试。`shutdown.rs:60-68` 的 `wait_for_signal` 按 `#[cfg(windows)]` 分叉为「等停止控制码，或控制台模式下等 `tokio::signal::windows` 的 ctrl_c／ctrl_break」。
2. 出向网络策略换承载：`14:7` 的「两个写出进程的 systemd 单元需放开到落点的出向网络策略」改为 Windows 防火墙按服务短名限定的出站规则，目的地址集合固定为部署记录所载落点这条约束不变。
3. `00b:264` 的八个系统账户互不复用改由虚拟账户 `NT SERVICE\<服务名>` 承接；**「同属组 `ep`」这一层直接不要**，只有八个进程，DACL 里逐账户列 ACE 即可，绕开「虚拟账户能否加入本地组」这个不确定项（第十节第 6 项）。`ep-migrate` 的独立账户保留为一个普通本地账户。
4. `14:13` 逐字「其消费方为 Prometheus 与 systemd」——`/readyz` 的 systemd 侧消费方（sd_notify 就绪协议）消失，换成服务宿主自身的 `SetServiceStatus(SERVICE_RUNNING)`；`/metrics` 的 Prometheus 消费方不受影响，该偏离的结论（两个端点不使用第 5.2 节封套）不变。

**判据强度的一处如实降级**：systemd 侧的退出码是内核经 waitpid 交给 PID 1 的客观事实，`sc query` 的 `WIN32_EXIT_CODE` 是服务自己经 `SetServiceStatus` 填的值。判据保留但须夹具补强——测试进程在发停止命令前先 `OpenProcess` 持住句柄，停机后 `GetExitCodeProcess` 断言真实退出码并与自报值双向比对。补强之后强度回到原位，不补强就是弱化。

**阶段 1 退出条件 4（`01:499`）逐字「八个二进制启动、就绪、优雅停机、崩溃重启四条路径在 E2E 中全绿」可以保住**，四条路径都有承载，只是各自判据要按本节重写。这是本次变更里唯一不需要撤下的顶层退出条件。

##### 4.3 机制三　IPC 与文件布局

**承载物：Windows 命名管道（`tokio::net::windows::named_pipe`）。不取 Windows 上的 AF_UNIX（第一节末），不取回环 TCP。**

回环 TCP 一支不取的理由：TCP 端口没有任何 ACL，本机任何用户的任何进程都能连，要恢复 0660 那层隔离得新加 mTLS 或共享口令（新机制、新密钥管理）；且它直接推翻 `00b:266` 逐字「不使用本机 TCP，理由是避免任何一个接口意外可从网络到达」。

命名管道一支**保住并加强了 `00b:266` 的那条理由**：`ServerOptions` 默认 `reject_remote_clients: true`，本机可达性是内核层的；DACL 可在创建时给定，表达力强于 0660（可以只授 archive-writer 与 backup-writer 两个账户，而不是对整个组 `ep` 打开），并消掉现有实现里先 `bind` 再 `set_permissions` 之间的竞态窗口；残留清理整块消失（`crates/adapter/ipc/src/server.rs:126-127` 的残留 socket 删除与 `:181` 的停机 `remove_file` 都不再需要）；另新得一项 Linux 侧没有的能力——`ImpersonateNamedPipeClient`／`GetNamedPipeClientProcessId` 可核对调用方身份。

两处必须新写、不是换名字的代码：**没有 listener 对象**（一个 `NamedPipeServer` 实例只服务一个连接，循环里要先建下一个实例再交出当前实例）；**客户端要处理 `ERROR_PIPE_BUSY`**（`client.rs:71-72` 逐字「每次调用一条新连接」，不加 BUSY 重试会把「core 在但忙」经 `forward.rs:90` 误报成「core 不可用」并落 spool）。**默认安全描述符不能用**，必须显式构造（不确定：默认 DACL 的确切 ACE 集合与版本差异，第十节第 10 项）。

文件布局：`/etc/ep` → `%ProgramData%\EP\config`，`/var/lib/ep` → `%ProgramData%\EP\`，`/run/ep/ipc` 随命名管道一起消失。权限位换 NTFS ACL，逐条：0600 机密库（`00b:582`）与 0700 staging（`03:974`）可精确表达；**0750 审计证据目录加 archive-writer 只读无写无删（`03:886`，裁定 C-27）在 ACL 上表达力更强**，可对该账户显式 Deny `DELETE` 与 `FILE_WRITE_DATA`，比靠组权限位凑只读更贴合裁定原文，这一处是净改善；0400 master.key（`02:674`、`02:855` R-04）的判据要软化为「除服务账户、SYSTEM、Administrators 外无其他授权 ACE」——**这是判据锐利度下降，不是防护下降**，R-04 逐字已承认「持有该服务器操作系统权限者可读取主密钥」，两者不要混为一谈。

**四条 Windows 特有的新增必做项，漏掉就是静默降级：**

1. `%ProgramData%` 的继承 ACL 默认对本机 `BUILTIN\Users` 可读，而现有代码在 `crates/adapter/ipc/src/spool.rs:64-69` 与 `server.rs:129-134` 两处**无条件建目录且完全不设权限**（Linux 侧靠父目录 mode 与 umask 兜住）。安装器必须断继承并显式设 DACL，进程启动时须像 masterkey 那样**核对目录 ACL**，不能只建不查。
2. `spool.rs:160-162` 逐字 `pub fn is_writable(dir: &Path) -> bool { std::fs::create_dir_all(dir).is_ok() }` 在本平台会假阳性——「可建子目录」与「可建文件」是两个不同的权限位。改成实建探针文件再删。阶段 14 的落点可写性持续判定同源，一并改。
3. `spool.rs:136-141` 的 `File::create` 重写与 `:114-124` 的 `remove_file` 截断，在本平台会被杀毒与备份代理的瞬时句柄打断，该错误经 `SpoolError::Io` 到 `forward.rs:94-96` 被归成 `ForwardOutcome::Lost`，而 `forward.rs:28-30` 对 Lost 的注释逐字是「连盘都落不下。这是最坏的一档」。改成先写临时文件再原子替换，加有限重试。同一成因适用于 `03:974` 的 staging「会话终态后立即删除」：Linux 的 unlink 总能成功，NTFS 上若他人持句柄且未带 `FILE_SHARE_DELETE` 则返回拒绝访问。删除路径要有重试与失败登记，且 staging 与附件目录**必须列入杀毒排除**，该项进部署清单。
4. 路径长度：`LongPathsEnabled` 是全机注册表值，按第零节排除。Rust 标准库在文件操作中对绝对路径做 `\\?\` 前缀转换，**本仓自己的文件访问不受 260 字符约束**；但随产品交付的 `pg_*.exe` 与客户侧的备份代理不保证如此。据此定一条硬约束：**安装根目录取短名**（默认 `C:\EP`，而不是把数据放在 `%ProgramData%\EP` 的深层路径下），并把 `03:1124` 的 `search\<legal_entity_id>\` 与附件三段式路径的最坏长度在该根下算一次留证。

##### 4.4 机制四　打包交付与认证

- **交付形态**：由 OCI 容器改为安装包（MSI 或压缩包）加服务注册脚本，理由见第一节结论一。`spec:1184` 不再成立。连带：`14:530` 的容器扫描一项、`14:532` 的容器签名一项换成安装包与 PE 二进制；`14:541` 的 `RG-CI-PROBE-ABSENT` 逐字「镜像内不含符号 api_v1_system_echo」把被测对象换成 PE 二进制（判据形态不变，仍可判）；`spec:1826` 逐字「插件运行时以第 9.3 章的受控容器承载服务端签名 WASM 组件」改措辞——WASM 运行时跨平台，真正的隔离来自 WASM 沙箱而非容器。
- **构建目标**：`xtask/src/reproduce.rs:4-5` 与 `:27` 的 `x86_64-unknown-linux-musl` 换为 `x86_64-pc-windows-msvc`。**musl 静态链接连带消失**，`01:561` 的风险 R-01（musl 内存分配性能）随之作废——这是本次变更里少见的净减一条风险。
- **可复现构建**：`14:533` 的两次构建比对产物哈希这条判据形态保留，但 **PE 二进制能否稳定字节一致未实测**（第十节第 8 项）。在实测结论出来之前，CI 阶段 8 `reproducible-build` 不得留在 `delivered`，须按 `.github/ci/pipeline-stages.tsv` 已有的 `delivered`／`undelivered` 机制处置。
- **认证基线**：附录 D.2 的 BC-1 行是被第 2.2、13.2、17.5 三章共同回指的取值来源，必须先改这里。操作系统列取 **Windows Server 2022**，部署形态与编排列取 4.2 的形态。**配额取值必须在本平台重新实测标定，不得沿用为 cgroup 标定的数字**；`spec:1826` 的「全部进程按第 13.1 章的资源配额与让路顺序配置 cgroup」同批改。Server 2019 按第零节前提 3 写入 D.2 说明段，形态与国产 Linux 那句逐字对齐——可运行、不在首版认证组合内、不在 D.3 单维度替换清单内。
- **签名体系**：生产 Windows 制品固定使用 Authenticode；开发与内部制品可使用内部 ECDSA P-256，但必须标记为开发签名且不得进入生产发布。Authenticode 证书可由软件厂商或客户提供，两种来源走同一签名接口、审计记录与客户侧验签门禁。
- **CI 平台**：默认取内网自建 Forgejo 加 Woodpecker Windows agent；全部门禁只由 `cargo xtask ci` 聚合和判定，CI 平台配置仅作薄适配器。现存 bash、POSIX 可执行位、Linux 路径与直接调用子门禁的脚本均不是现行权威，须在首批实施中迁移，不得据其存在声称 Windows CI 已通过。

##### 4.5 PostgreSQL 16 的口径差异（不属四机制，但载重）

数据库本身不换，但下列七条是操作系统换代带来的，必须与四机制同批处置：

1. **无 Unix 域套接字**：`pg_hba` 的 `local` 与 `peer` 不可用，本机连接一律 `host 127.0.0.1/32` 与 `::1/128` 加 SSPI 或 scram。规格第 7.7 章「只允许从本机建立复制连接」措辞平台中立，但须补一句平台口径。`14:452` 启动自检第八项里「`pg_hba` 只允许这两个角色从本机连接」这条 Blocking 断言的判据要重写（该断言目前尚无实现）。**这一条弱化了第 7.7 章三项遏制手段之一的载体，实测后无论结论如何都必须显式披露。**
2. **数据库 locale 已冻结，不再二选一**：PostgreSQL 16 建库参数固定为 `LOCALE_PROVIDER libc`、`LC_COLLATE 'C'`、`LC_CTYPE 'C'`，`db/bootstrap/00_database.sql` 必须逐字采用这三值。首版不依赖 ICU，不写 `ICU_LOCALE`，也不再按 Windows PostgreSQL 发行版是否带 ICU 分支。`db/checks/12_collation_conformance.sql` 固定断言 `datlocprovider='c'`、`datcollate='C'`、`datctype='C'`。
3. **旧 ICU 排序版本门禁已移除**：旧 `LOCALE_PROVIDER icu`、`ICU_LOCALE 'zh-Hans-CN'` 与「选定构建后再核实」只作历史证据，不得实现、不再进认证基线。默认字符串一律按 C 字节序排序；产品确需中文阅读序的界面或报表只能使用应用层显式持久化的 `sort_key` 排序，不得借数据库默认 collation 暗中恢复 ICU。
4. **跨平台基础备份不可移植**：既有 Linux 集群的 `pg_basebackup` 产物与其后的 WAL 归档链在 Windows 版上不可恢复。三条后果：割接只能走 `pg_dump`／`pg_restore` 逻辑迁移；阶段 14 全部恢复演练的实证记录必须在本平台重做；演练目标实例必须是同一 Windows 发行版。`14:285` 的回放本身成立，改的是「备份从哪来」的前提。
5. **无跨进程信号**：`14:73` 的两个写出进程「只经进程启停与退出码」监管 `pg_receivewal` 与 `pg_basebackup`，而 Windows 没有跨进程投递 SIGINT／SIGTERM 的机制，「停止」这一半没有干净等价物。须改为作业对象终止或控制台事件，并如实写明它不是优雅停止。
6. **服务账户不能是管理员**：`db/bootstrap/README.md:56` 的执行方式一节没有覆盖这一层，引导流程要多一步。
7. **三个参数已经显式冻结**：`wal_sync_method = 'fsync_writethrough'`、`effective_io_concurrency = 0`、`huge_pages = off`。首版不依赖 `posix_fadvise`，也不授予服务账户“锁定内存中的页”权限；认证只验证固定配置，不产生实现分支。至于“Windows 上 `shared_buffers` 不宜大”这一流传说法，PG 16 官方文档该页无 Windows 专属注记，本裁定不据此断言，以固定配置下的认证实测为发布证据。

#### 五、连带作废或需重裁的已生效裁定（逐条点名）

**己-1（规格第 13.1 章配额表的承载面、判据面与认证冻结口径）——重裁，不整条作废。**

| 己-1 的段 | 本裁定的处置 |
|---|---|
| 第一节　底账修正（九行对八 slice，第 9 行内置搜索索引无承载） | **全部保留**。该结论与操作系统无关，本平台仍是八个资源单位对八行，第 9 行仍不落、不加和、不拆分，八行权重之和低于 100 这条既定偏差在 CPU 一列存活期间照旧披露 |
| 第二节　承载面四类取值 | `MemoryMax` 留，`IOMax` 按现行绝对限额路径取证，`MemoryLow` 与 `IOWeight` 删（做不到二/一），`CPUWeight` 固定只作意图声明且首版不启用。承载物由 `deploy/systemd/system/*.slice.d/10-resource-limits.conf` 八个文件改为服务宿主层读取的静态限额文件 |
| 第三节　判据面 | `IOWeight` 与 `CPUWeight` 均无首版运行期被测对象，相关判据撤下且不得因实测自动恢复。内存及现行绝对限额的读回仍须采用具名 Job Object 并授予校验方 `JOB_OBJECT_QUERY`（第十节第 4 项） |
| 第四节　门户攻击面 | **推理保留，遏制面进一步缩小**。本平台按权重磁盘 IO 与 CPU 比例在首版均固定不启用，门户的跨进程资源侧遏制**只剩内存硬上限一维**；`07:1104` 的四项遏制第四项须按此改写，这是本次变更的第二处实质安全回退（第一处是做不到九） |
| 第五节　认证冻结口径（用 `spec:1826` 的「下限」语义） | **保留**。该口径与操作系统无关；但「两个权重列与机器无关，原样沿用」这半句因两列一删一待定而暂时无对象 |
| 第六节　越权自纠（`14:585` 撤销作废规格第 21.19 章那句） | **保留且更要紧**——第 21.19 章现在要承载更多的诚实披露，删它就是删一条对客户的义务 |
| 第七节　历史上曾归产品负责人的问题 | **现已关闭，不形成待决**；总体规格与阶段正文已按 Windows 终局值回写。其列明的七处同步修订纪律仍作为历史校验依据，见第六节 |
| 第八节　连带处置（己-7 的 T3-2 与 T3-4） | **需再核**：己-1 已标该两个标识符的具体落点不确定；本裁定改变了它们所依赖的承载面，改动方须在本裁定落地时一并核对 |

**其余已生效裁定的逐条判定：**

- **C-27（审计证据目录 0750，archive-writer 只读、不授予写入与删除）**：结论不变，**载体换 NTFS ACL 后表达力增强**，不作废。
- **C-25（`cgroup-quota-matched` 自检项整项撤销）**：不受影响，本裁定不重开 `01:201` 的 78 退出路径。
- **F-06（阶段 13 插件并发限流不开降级窗口）**：不受影响。本裁定不新增任何限流源；反而删掉一类事件（保底份额被击穿）。F-55 终态 21 类 kind 里仍没有任何限流或配额类。
- **F-05（含 H-07 的七种 IPC 报文类型落 `ep-adapter-ipc`）**：不受影响。报文类型与传输承载物无关，`frame.rs`／`message.rs`／`spool.rs`／`forward.rs` 四个文件的协议面本轮一行不动（`spool.rs` 另有第 4.3 节的三条落盘缺陷要改，与报文类型无关）。
- **F-01、F-03、F-04、F-07、G-01、A／B／C 三类的业务侧结论**：不受影响，一条不动。
- **附录戊四（为十四阶段总表建机检门禁的可行性评估，结论为不做）**：本裁定三次援引其先例（做不到三、做不到八、做不到五的备选支），不改其结论。

#### 六、规格与 PRD 的修订清单（逐条 文件:行号 与改法）

规格 `docs/superpowers/specs/2026-07-19-enterprise-private-operations-platform-design.md`：

| 行号 | 现状要点 | 改法 | 档 |
|---|---|---|---|
| 58 | 认证基线冻结为「主流企业 Linux 发行版、单机容器编排」，后半句是国产 Linux 延期表述 | **整句重写**。操作系统取 Windows Server 2022，编排取 4.2 形态；国产 Linux 那半句按第九节第 1 条处置，不得顺手删 | 需重写 |
| 68 | 「首版编排只认证单机容器编排」 | 重写为 4.2 形态；Kubernetes 延期项须**重新判断是否仍是同一个延期对象**（Windows 节点的 Kubernetes 与 Linux 节点不是同一件事） | 需重写 |
| 263 | 两个写出进程「以两个独立进程与两个独立 cgroup 运行」 | 进程拆分要求**保留**，`cgroup` 换成本平台的资源单位；同句「二者不共享 CPU、内存与磁盘 IO 预算」在磁盘 IO 侧按做不到一降级 | 需重写 |
| 1135 | 六个 cgroup 接口名逐项绑定 | 重写为 4.1 的四类存活情况；六个接口里只有 `cpu.max` 与 `memory.max` 有干净对应物，其余四个删 | 能力缺失 |
| 1137-1147 | 九行三列配额表本体 | **表本体保留**（作为硬件规格标定与认证实测的意图声明），**表下补一句**：本表在本平台不构成运行期机制取值，按权重磁盘 IO 与 CPU 比例首版固定无运行期承载；内存硬上限为唯一比例表运行期列 | 能力缺失 |
| 1150 | 借用与收敛语义、突发上限 | 磁盘 IO 侧整段删；CPU 侧按实测结论改写 | 能力缺失 |
| 1151 | 「memory.low 与 memory.max 取同一取值」，上文理由句「内存超售会触发内核终止进程」 | 改为单值硬上限；理由句改为触限时分配失败返回错误 | 需重写 |
| 1152 | 以「cgroup 无法在一个组件内部区分优先级」论证两个写出进程必须拆分 | 结论**保留且理由更强**（资源单位同样只作用于进程集合，粒度更粗），机制名换 | 需重写 |
| 1154 | 门户单列配额行，「与反向代理共用一个 cgroup 会取消该边界」 | 边界主张保留（虚拟账户加每服务 SID 承载得住系统账户边界），机制名换；同段门户资源侧遏制一句按己-1 第四节的更严版本削弱 | 需重写 |
| 1157 | 机制一整段以 `cpu.weight` 与 `io.weight` 的比例分配立论 | 机制一在磁盘 IO 侧**整体不成立**，退化为只剩「不得超出突发上限侵占盈余」半条；八级让路里所有标注「按机制一保证」的级别逐级重判 | 能力缺失 |
| 1159、1161、1162、1163 | 四处把级别归属写成 cgroup 归属 | 机制名替换加各级保证强度重判的组合，不是纯换词。机制三（PostgreSQL 侧的只读角色、语句超时、单查询资源上限、`maintenance_work_mem`）与操作系统无关，一字不动 | 需重写 |
| 1160 | 「其份额之内的磁盘 IO 不向任何级别让路……由该 cgroup 的 io.weight 落实」 | **整句删除**（本次后果最重的一处）。RPO 不超过 15 分钟完全押在机制四认证实测上 | 能力缺失 |
| 1170 | 击穿判定的三个 cgroup 统计项 | **该类事件整体删除**，只留限流与让路延迟两类，并写明删除理由 | 能力缺失 |
| 1181 | 「操作系统为主流企业 Linux 发行版」加国产 Linux 延期句 | 前半句取 Windows Server 2022 并写明 2019 的形态；后半句按第九节第 1 条处置 | 需重写 |
| 1182 | 「编排为单机容器编排，取值是 Docker Compose 或 Podman 加 systemd」 | **整条重写**为 4.2 形态，并明写这个二选一退化为单一形态，退化掉的正是较强的那一支（就绪门槛） | 能力缺失 |
| 1184 | 「全部组件以标准 OCI 容器交付」 | 重写为安装包加服务注册（第一节结论一） | 需重写 |
| 1185 | 「使用同一 Linux 平台」 | 纯措辞 | 需改措辞 |
| 1180、1186 | 「硬件为一台 x86_64 服务器」「不依赖任何云厂商专有服务」 | **不动** | 不受影响 |
| 1372 | 认证章的基线取值加国产 Linux 成套表述 | 同 58；另见第九节第 1 条对等级保护三级对外表述的提醒 | 需重写 |
| 1534 | 延期目录里以 Linux 基线为对照 | 同源同步改；Kubernetes 延期项复核 | 需重写 |
| 1682、1684 | 21.19 风险与控制段 | 风险段按做不到一把 RPO 的必要条件改写为无机制侧保证并把残余风险上界写严；控制段删击穿一类；**第 21.19 章全文保留**（己-1 第六节） | 能力缺失 |
| 1826 | 附录 A.4「全部进程按第 13.1 章配置 cgroup」、插件运行时以受控容器承载 | 机制名换；**配额取值须在本平台重新实测标定**；受控容器改措辞 | 需重写 |
| 1839 | 三项写出周期的必判项 | 判据**不动**（这是做不到一之后 RPO 的唯一落点，反而更吃重），但须与 1682 同批注明它现在是唯一落点 | 不受影响但吃重 |
| 1865 | 附录 A.6 两类恢复演练 | 判据不动，但被测机器按第一节结论二固定为 Server 2022，且按 4.5 第 4 条，Linux 上的演练记录一条都不能沿用 | 需重写 |
| 1956、1958 | 附录 D.2 的 BC-1 行与说明段 | **先改这里**，它是 2.2、13.2、17.5 三处的取值来源。操作系统列取 Windows Server 2022，部署形态与编排列换；说明段按第零节前提 3 为 Server 2019 写一句，形态与国产 Linux 那句逐字对齐。核心交易数据库列 PostgreSQL 16 **不动** | 需重写 |
| 1067、7.7 章 | 「只允许从本机建立复制连接」「其凭据由该服务器的操作系统层保护」 | 措辞平台中立，**但须补一句平台口径**，见 4.5 第 1 条 | 需重写 |

PRD `docs/superpowers/specs/2026-08-09-first-release-prd.md`（全文平台相关处极少，实测三处）：`4371` 复述第 13.1 章「让路次序不是运行期保证」，按 1682 同批改写，磁盘 IO 一维的表述跟着降级；`4417` 甲十六延期项里的国产化认证矩阵，按第九节第 1 条处置；`34` 逐字点名「国产 Linux 作为默认平台的旧表述」为作废取值，该句**保留**（它禁止的表述在新目标下仍不得出现），不必改。

**规格与 PRD 全文没有 SELinux、AppArmor、musl、Unix domain socket、硬编码 Unix 路径的字样**——实现侧的 14 处 Unix 绑定与 29 处硬编码路径在规格侧**没有对应条款**，不要去规格里找它们的落点。

#### 七、十四个阶段的分档

| 档 | 阶段 | 依据 |
|---|---|---|
| 整章重做 | **0 个** | 见下 |
| 大改 | 阶段 1、阶段 14 | 阶段 1：第 5.6 节与 D-02／D-05／D-06／D-11／D-13 五个交付物、E2E-01／03／05／06／07、IT-27／IT-29、退出条件 4 与 16、`01:197`／`01:287`／`01:338`／`01:592`／`01:604`，以及 `deploy/` 全目录与三个 `scripts/` 脚本。全卷受影响最重的一章。阶段 14：受影响 20 条，另加 4.5 的备份不可移植一条 |
| 小改 | 阶段 2、3、7、13 | 阶段 2：`02:89` 的 `pg_hba` 行、`02:674`／`02:855` 的 0400 判据、`02:885` 的理由句。阶段 3：`03:886`（C-27，载体换且增强）、`03:974`、`03:1004`（clamd 经本机 Unix socket 调用）、`03:1124`。阶段 7：`07:1104` 门户遏制第四项（实质安全回退，须显式披露）、`07:1057` 不动。阶段 13：己-1 已点名的三处插件配额措辞；**客户端四端主体一字不动**，`13:604` 逐字已写「IPC 承载在 Windows 命名管道与 macOS Unix domain socket 上」，服务端照此扩边有现成先例 |
| 不受影响 | 阶段 4、5、6、8、9、10、11、12 | 八个业务模块阶段，与操作系统无关，一条不动。唯一例外是 `04:711` Argon2id 参数实测门槛引用 `app-core.slice` 的 CPU 配额，纯改措辞 |

**阶段 14 是否整章重做——核实结论：不是，是大改。** 该章 630 行，逐条核过：受影响的 20 条**全部集中在承载物与判据**。而该章的**主体一条不动**：PostgreSQL 16、`pg_receivewal`／`pg_basebackup`／`pg_verifybackup`、复制槽与 `max_slot_wal_keep_size`、归档通道状态机、水位推进算法、恢复点对齐、部署级备份加密、混沌六类，全部与操作系统种类无关。把它判成「整章重做」会高估改动面并诱使人重写正确的算法。**真正接近整章重做的是阶段 1 的编排与配额半边，不是阶段 14。**

#### 八、代码改动面（实测，非估算）

全仓 292 个 `.rs` 文件。**带 Unix 绑定的只有 4 个文件、14 处**（本裁定落地前后各复测一次）：

- `crates/platform/runtime/src/shutdown.rs:60-68`：`tokio::signal::unix` 的 `SignalKind::terminate`／`interrupt`。该处在 Windows 上**必定编译失败**（确定）。改法：`#[cfg(windows)]` 分叉，形态最简单。
- `crates/adapter/kms/src/masterkey.rs`：4 处，且该文件**已有 `#[cfg(unix)]`／`#[cfg(not(unix))]` 分支形态现成**。要真写一份 Windows 实现（ACL 枚举与判否），**不得保留现有的 `#[cfg(not(unix))]` 拒启动分支**。
- `crates/adapter/ipc/src/client.rs`（2 处）与 `server.rs`（7 处）：**不是纯 `cfg` 能解决的**，要抽一层传输边界后各实现一次。

**IPC 的七个文件里有四个（`frame.rs`、`message.rs`、`spool.rs`、`forward.rs`）协议面一行不动**——`frame.rs` 已对 `AsyncWrite`／`AsyncRead` 泛型，这层现成。`server.rs` 的协议半边（`MethodTable`、`ServerError`、`serve_conn` 的帧循环）不动，只把 `serve_conn` 的入参改成泛型一行；生命周期半边（`bind`／`serve`）按平台各写一份。**关键设计约束：`bind()` 的返回类型不要泄露到 apps**——换成不透明的 `IpcListener`，`apps/core-server/src/main.rs:234` 与 `apps/plugin-host/src/main.rs:84` 的 `match ipc.bind()` 形状零改动，且保住「绑定失败要在宣告 serving 之前暴露」这条现有性质（`first_pipe_instance(true)` 正好让名字被占在这一步 fail-closed）。

**硬编码路径实测 29 处，分布在 10 个文件**，真默认值集中在 `crates/platform/runtime/src/config/sections.rs`（`:49`、`:193`、`:279`、`:644`、`:768`、`:784`、`:832`）、`config/mod.rs:37-38` 与 `crates/adapter/kms/src/cfg.rs`，其余是单元测试断言与文档注释，跟着机械改。**配置侧类型不需改**：`socket_path: PathBuf` 装得下 `\\.\pipe\ep-core`，该结构没有任何路径形态校验，只需改默认值并同步 `docs/config-reference.md:35`。

**另有五处非路径、非信号的平台绑定，各自单列**：`crates/platform/runtime/src/selfcheck/items/basic.rs:51-57` 时钟偏差自检读 Linux 的 `adjtimex`（非 Linux 平台如实报「未覆盖」——**该行为正确，本裁定不改它，但须在本平台给出一个真判据或让它永久停在未覆盖并登记**）；`lifecycle.rs:124` 把停机事件命名为 `Sigterm`（纯改名）；`lifecycle.rs:106-110` 退出码 78 绑 `RestartPreventExitStatus`（做不到五）；`crates/adapter/kms/src/hsm.rs:177`／`:188` PKCS#11 模块路径取 `.so`（换 `.dll`）；`config/sections.rs:667` 与 `boot.rs:113` 的 `worker_threads` 取 0 表示按 cgroup 配额推导（改为按 Job Object 或整机核数）。**`obs/log/mod.rs:89` 与 `db-pg/retry.rs:6` 登记的 SIGHUP 热生效手段，全仓本无 SIGHUP 处理器，本平台一并换成别的触发方式或如实登记为不支持。**

**新增（不能靠 `cfg` 解决的）**：服务宿主层（八个二进制共用，落 `crates/platform/runtime/` 内新模块，不新增 crate、不新增进程）；日志落地与轮转一层（做不到七）；命名管道的 DACL 构造与 `ERROR_PIPE_BUSY` 重试；spool 的原子替换与探针式可写性判定（后者是修本平台上的真缺陷，不算新机制）。

**估算**：新增约 600 至 800 行（服务宿主层为大头），改动约 100 行，apps 侧约 0 行，`deploy/` 与 `scripts/` 另计。IPC 侧的 CI 连带比预想小得多——全仓 `tests/` 下没有 IPC 集成测试，`.github/ci/` 与 `xtask/src/e2e.rs` 一次都没提 socket，非代码引用只有 `docs/config-reference.md:35` 一处。

**受影响的工具链**：`xtask/src/codecheck.rs` 的 slice 一维判据（现以 `deploy/systemd/system` 与 `deploy/podman/<p>.container` 的 `Slice=` 三处一致为判据）须重写；`00b:35` 逐字「名字与进程名、systemd 单元名、cgroup slice 名一一对应」把「systemd 单元名」换成「Windows 服务名」，结论不变、被测串换。CI 十一阶段中，阶段 11 `deploy-limits`（`scripts/verify-resource-limits.sh`，bash 加 `/sys/fs/cgroup`，`:388-391` 另用 `df -P` 与 `stat -c` 解析块设备主次号）须重写；**在 4.1 的实测出结论之前，该阶段按 `pipeline-stages.tsv` 已有机制标 `undelivered`，不删行**——这套机制现成，`verify-pipeline-commands.sh` 会真跑并核对退出码 70，用它比留一个半可靠的门禁好。

#### 九、诚实统计

- **新增机制 3 个**：服务宿主层（含 Job Object 自我指派）、本地日志落地与轮转、命名管道的 DACL 构造与忙重试。
- **新增常驻进程 0 个**，八进程清单不变，`00b` 第 12 节的新增进程禁令不破。
- **废掉的已生效裁定 0 条，重裁 1 条**（己-1）；**换载体不改结论 1 条**（C-27，且表达力增强）；**须由改动方再核 1 条**（己-7 的 T3-2 与 T3-4）。F-01、F-03、F-04、F-05、F-06、F-07、G-01、C-25 与 A／B／C 三类业务侧结论**一条不动**。
- **规格条款：不再成立 6 条**（1160 的机制保证、1170 的击穿判据、1157 在磁盘 IO 侧的机制一、1182 的编排取值、1184 的 OCI 交付、1151 的 `memory.low` 半条）；**降级 8 条**；**纯措辞约 10 条**；**PRD 2 条**。
- **计划判据：撤下 5 条**（D-05 的等价性核对、D-06 的两个权重列逐行相等、阶段 1 退出条件 16 的同一半、阶段 14 退出条件 10 的配额半边、E2E-07 的「无残留 socket」半条——后者在本平台恒真，管道实例随最后一个句柄由内核回收，按本卷先例恒真的门禁比没有门禁更坏）；**改写不撤下 6 条**（D-02、E2E-01、E2E-05、E2E-06、IT-27 第四项、IT-29）；**挂在实测上待定 1 条**（E2E-03）；**可原样保住的顶层退出条件 1 条**（阶段 1 退出条件 4）。
- **净减 1 条风险**：`01:561` 的 R-01（musl 静态链接内存分配性能）随构建目标更换而消失。
- **两处实质安全回退，必须写进交付说明，不得沉默**：其一，门户暴露面（`07:1104`）的第四项遏制在实测出结论前只剩内存硬上限一维；其二，命名管道名字空间无创建侧准入控制（做不到九），门槛低于第 21.18 章口径，须与其**并列**登记。另有两处判据强度下降（退出码由客观观测降为服务自报，须夹具补强；0400 判据由三位八进制相等降为 ACL 集合判否），**这两处是判据降级不是防护降级**，不要混算。
- **本次变更顺带查出的现存缺陷 4 条**（第三节），其中 1 条本轮即改。
- **认证基线要重取的**：附录 D.2 BC-1 行的操作系统列取 Server 2022、部署形态与编排列；附录 A.4 的配额取值全部在本平台重新实测标定；附录 A.6 两类恢复演练全部重做；附录 D.3 单维度替换清单里的部署形态与编排形态两个维度。

#### 十、执行次序与停写线

1. **先改附录 D.2 的 BC-1 行（`spec:1956`、`1958`）**。它是第 2.2、13.2、17.5 三章共同回指的取值来源，不先改这里，后面每一处都会指向一个已作废的取值。
2. **再改规格第 13.2 章（`1180-1187`）与第 13.1 章**（`1135`、`1137-1147` 表下注、`1150`、`1151`、`1152`、`1154`、`1157`、`1159-1163`、`1170`），这两章是全部承载物的源头；同批改 `58`、`68`、`263`、`1372`、`1534`、`1682`／`1684`、`1826`、`1865`，以及 PRD 的 `4371`、`4417`。**己-1 第七节列的七处必须同批**，漏任何一处都会留下指向已删机制的悬空引用。
3. **然后跑第十一节的实测清单**。这一步不产出文档，产出的是三个二值结论（CPU 一列能不能落、`IOMax` 能不能落、退出码分流能不能成立），它们各自决定上一步里三处的最终措辞——**因此第 2 步的这三处先写成待定，不要先写死再回改**。
4. **再改技术基线 `00b`**（`33`、`35`、`263`、`264`、`266`、`571`、`582`、`641`）与阶段 1 计划，最后是阶段 14、2、3、7、13 的小改。
5. **最后落码**：传输边界抽出 → `shutdown.rs` 分叉 → 服务宿主层 → 路径与 ACL → 日志落地 → `deploy/` 与 `scripts/` 重建 → CI 阶段 5 与 11 重写。

**第三节的四条现存缺陷不排在这条次序里**：第 3 条（`.gitattributes`）本轮即改；其余三条随阶段 2 的下一次触碰同批改，不等本裁定落地。

**停写线（哪一步之前不该继续往下写）。**

- **业务代码可以继续写，不必等**。全仓 292 个 `.rs` 只有 4 个文件带 Unix 绑定，阶段 3 的洞与阶段 4 至 12 的业务模块与本变更完全无关，停下来是纯浪费。
- **在第 5 步的「传输边界抽出」完成之前，不得再往 `crates/adapter/ipc/` 加报文类型**——F-05 H-07 裁定的七种归档与备份报文类型正待落，现在落进去就要在边界抽出时再动一遍。
- **在第 2 步完成之前，`deploy/` 下不得再新增任何文件**，`scripts/verify-orchestration-equivalence.py`、`verify-resource-limits.sh`、`dev-up.sh`／`dev-down.sh` 不得再投入修改工时——它们的被测对象正在消失。
- **在第 3 步的实测出结论之前，不得把任何配额取值写死进新的承载文件**；`01:24`／`:457`／`:507` 三处的判据文字**可以改写为待定形态，但不得写成任何依赖实测结论的确定判据**（本条经第十六节补裁巳更正，原文的全称禁令过宽，会把合法的降级动作一起挡住）。
- **在第 4 步完成之前，不得新增任何对 `tokio::net::unix`、`std::os::unix`、`PermissionsExt` 的调用点，也不得新增任何硬编码绝对路径**——现存的 14 处与 29 处已经量准，新增的每一处都要再量一次。
- `.github/` 下现存 CI 文件不再是权威；若保留，只能改造成准备环境后调用 `cargo xtask ci` 的薄适配器。任何平台适配器不得复制门禁逻辑。

#### 十一、原需另行裁定事项的闭合状态

1. **国产化替代路径与等级保护三级对外表述：已关闭。** 按第十三节保留为长期项，首版与当前开发只投 Windows，不建立 Linux 分支。
2. **Hyper-V 整机分区形态：已关闭；F-55 插件 utility VM 为唯一窄例外。** 产品服务仍取 Windows Server 原生服务，不把整个平台放入 Hyper-V 或任何虚拟机层；Job Object 能承接多少就按可验证能力承接，不能承接的能力如实降级。后续 F-55 §4.5 只为单次 `LOCAL_WINDOWS_HYPERV_CONTAINER` MCP 插件调用允许短命 Hyper-V-isolated utility VM，不承载产品服务、数据库或客户主数据卷，不能据此恢复整机虚拟化路线。
3. **Authenticode 签名：已关闭。** 生产 Windows 制品必须 Authenticode；内部开发制品可用 ECDSA P-256。证书可由软件厂商或客户提供。
4. **CI 平台取值：已关闭。** 默认 Forgejo 加 Woodpecker Windows agent，`cargo xtask ci` 是唯一入口，平台配置为薄适配器。

**已由第一节结论一消解、不再需要另行裁定的一条**：Windows 容器（进程隔离）是否在「原生」范围内——在 2019 至 2022 这个区间内它无论如何都过不了「一份制品」这一关，因此不必再问。若日后区间收窄到单一版本，本条重开。

#### 十二、Windows 首批实施验证门禁（17 项有效；保留原编号；不是设计待决）

本节不阻止按已冻结的 Windows 原生路径开始实施。全部项目须先在 Server 2022 上执行，再在 2019 上做同项复核，并形成机器版本、步骤、原始输出、结论和失败处置证据；未执行或证据不全一律不得宣称通过。任何失败只触发本节或前文已经写明的保守降级，并使对应 CI／发布门禁保持非零；不得自行切换 Linux、整机 Hyper-V、另一 CI 权威入口或第二套 Rust 核心。F-55 §4.5 的单次插件 Hyper-V utility VM 仅按其独立 gate 执行，不改变本条整机平台口径。

1. Job Object CPU 速率控制权重模式的取值域是否确为 1 至 9（决定 CPU 一列能否表达 44% 比 2%）。
2. **最小／最大速率模式（`MinRate`／`MaxRate`）的实际行为，以及 `MinRate` 之和低于 100% 时是否真给到保底**（决定 CPU 一列是降为意图声明还是逐行落地，**本清单价值最高的一项**）。
3. Job Object IO 速率控制在本地 NTFS 直连卷上的覆盖面，以及 `MaxBandwidth` 能否承接 backup-writer 的 `IOMax`。
4. 具名 Job Object 能否由外部核对进程经 DACL 授权的 `JOB_OBJECT_QUERY` 读回限额（决定 E2E-05 的替身能否保住），以及虚拟账户对具名 job 的 `JOB_OBJECT_ASSIGN_PROCESS` 与嵌套 job 下的自我指派是否可靠。
5. 服务进程未报告 `SERVICE_STOPPED` 即退出时，管理器是否判为崩溃并执行 `sc failure` 恢复动作（决定做不到五走主承载还是备选）。
6. 虚拟账户 `NT SERVICE\<名>` 能否加入本地组（若不能，`00b:264` 的组 `ep` 一层按 4.2 第 3 条直接不要）。
7. 以非管理员用户在服务启动前创建同名管道，观察服务是否失败、客户端连到谁（做不到九；若能证明存在受支持的创建侧准入控制，做不到九应撤回）。
8. PE 二进制在固定 `rust-toolchain.toml`、`SOURCE_DATE_EPOCH`、`--remap-path-prefix` 与离线 vendor 下能否稳定字节一致（决定 CI 阶段 8 的状态列）。
9. `WaitToKillServiceTimeout` 在本区间两版的当前默认值，以及它是每服务预算还是全部服务的总预算（做不到四的量化，不改变其定性）。
10. `C:\ProgramData` 与安装根的默认继承 ACE 集合，以及命名管道默认安全描述符的确切 ACE 集合（决定 4.3 第 1 条安装器断继承的范围与 DACL 构造的起点）。
11. Windows 事件日志单条事件的大小上限与结构化字段支持程度（若做不到七的处置日后改走事件日志一支）。
12. ~~**选定的 Windows 版 PostgreSQL 16 构建是否带 ICU**~~　**已撤销、不计入本节 17 项有效门禁。** locale 已按 4.5 第 2、3 条唯一冻结为 PostgreSQL 16 `LOCALE_PROVIDER libc`、`LC_COLLATE 'C'`、`LC_CTYPE 'C'`，首版不依赖 ICU；本行只保留原编号供历史追溯，不执行、不留证、不阻断任何开发或发布门禁。
13. `pg_hba` 在 Windows 版上 `local` 记录不可用这一点的确认，以及 `host 127.0.0.1/32` 加 `::1/128` 能否表达 `02:89` 的三个角色放行口径（4.5 第 1 条，须显式披露）。
14. 在目标 Windows Server 2022 上验证固定值 `wal_sync_method = 'fsync_writethrough'` 的性能代价、断电恢复与 `pg_test_fsync` 证据，并验证 `effective_io_concurrency = 0`、`huge_pages = off` 已生效；本项不再选择配置值。

15. ops-agent 的虚拟账户能否对 postgres 进程取得 `PROCESS_SET_QUOTA` 与 `PROCESS_TERMINATE`（补裁壬；不成立则 PostgreSQL 一行退回宣告无承载）。
16. 目标进程若已在某个 job 内，嵌套 job 能否叠加内存上限（区间两版均在 Server 2016 之后，接口面应具备，行为未测）。
17. 服务启动次序上 postgres 先于 ops-agent，指派前那个未受限窗口实测多长（**不得把该路径说成「启动即受限」**）。
18. **给数据库设内存硬上限本身的后果**：Job Object 触限是分配失败返回错误，PostgreSQL 在分配失败下是查询失败还是后端进程崩溃。若为崩溃，则该上限本身有害，宁可不设并如实宣告无承载——本项问的不是能不能设，是该不该设。

**第 2、5 与第 15 至 18 项的实现均按前文已列的主承载加保守失败支路开发；证据出具前，对应能力状态只能写“未验证”，不得写成已覆盖或已通过。设计路径本身不再写成待定。**

#### 十三、F-08-1 的使用方裁定（本轮补入，原挂庚一）

**使用方表态原文口径：国产化相关条目「保留项」，但主要开发投 Windows，不做 Linux 开发。**

据此逐条落实，三取一里取的是**保留为长期项**，不取「改写为互斥」，也不取「另立国产化服务端分支」。

**一、保留的是什么。** 第 2.2 章与第 5.7 章登记的国产 CPU、国产操作系统、国产中间件、
国产浏览器、国密密码设备与云的认证矩阵，**整体保留在延期目录内，不删除、不降为非目标、
不改为「本产品不支持」**。附录 D.3 的单维度替换清单同样不因本轮新增国产维度，
维持原状——国产 Linux 本来就不在 D.3 内。

**二、必须改的一句，改它不违反「保留」。** 规格现行文本三处（`1372`、`1534`、`1958`）
与 PRD `4417` 逐字写着国产 Linux **「可以在该形态上运行，但不在首版认证组合内」**。
服务端改 Windows Server 原生、且本轮明定零 Linux 开发之后，**这半句变成假陈述**：
首版产物是 Windows 服务与 PE 二进制，在任何 Linux 上都不可运行，不只是「未认证」。
把一句已知为假的能力陈述留在规格与交付材料里，比删掉它更坏。

因此该半句**必须改**，改法是把「可运行、未认证」换成「首版不可运行，其支持随国产化
认证矩阵整体延期」。**这不是取消承诺，是把承诺的时点说准**：延期项本体一字不动，
动的只是对「今天能不能跑」的事实描述。若日后重启国产化，重启的是同一条延期项。

**三、零 Linux 开发的效力。** 首版不投入任何 Linux 侧开发工时，具体含：
不维护 Linux 构建目标（`xtask/src/reproduce.rs` 的三元组按 F-08 第 4.4 节单值取
`x86_64-pc-windows-msvc`，不做双目标）、不维护 Linux 编排文件、不做跨平台兼容层、
不为保住 Linux 可移植性而否决任何 Windows 侧取值。**F-08 第 4.3 节要求的 `#[cfg(windows)]`
分叉是平台分叉不是双平台维护**：Unix 分支保留其现有实现即可，不为它新增测试、
不为它跑 CI、不因它阻塞任何 Windows 侧改动。

**四、如实登记重启成本。** 因第三条，国产化延期项的重启成本由「移植」变为「重做部署与
运维层」：届时要重新产出 Linux 编排、cgroup 承载、systemd 单元与 musl 或 glibc 构建，
即 F-08 本轮删掉的那一整层。该成本**写入第 5.7 章延期目录该条目的备注**，
不得让日后读者以为它仍是一次小改。

**五、连带提醒（不属本裁定，归产品负责人自行判断）。** 第 17.5 章的等级保护三级
对外表述与国产化替代的可达性相关；本轮既未改变等保三级的技术要求，也未改变
延期项本身，故本裁定不动第 17.5 章。但服务端平台换代这一事实是否需要向客户
另行说明，属商务判断，本卷不代拍。

**本条裁定后，附录庚一的 F-08-1 行撤销。**

#### 十四、落地过程中查出的八处漏裁（本轮补裁）

规格修订落地时逐条核对第六节工单，查出八处**第六节点了名却没给依据、或根本漏列**的位置。
按本卷纪律，写不出依据的改动不做——因此这八处当轮全部挂起，由本节补裁后才落地。
八处的共同成因是同一个：第六节那张表是按**行号**编的，而受影响的是**语义**，
语义的落点比行号多。这一点记在这里，下次再有平台级变更，工单要按语义编而不是按行号编。

**补裁甲　1150 的突发上限折算规则整条不成立，五处引用逐处判。**

规格原文「其余各行的突发上限取其份额的三倍并以可分配量的 40% 封顶」是一条**相对量折算**。
磁盘 IO 份额一列删除后被乘数消失，CPU 比例首版固定不启用，该规则**在本平台无被乘数、无承载，整条不成立**，
已随 1150 改写删除。这与己-1 第二节在计划侧删同一条算法的方向一致，不新增分歧。

其余引用「突发上限」之处按**它是承诺还是限制**分两类判：
凡把突发上限当**取值来源**引用的（即据它推出某组件能取得多少），随折算规则一并失去对象，须改写或删除；
凡只是**限制性表述**的（即不得超出突发上限），**保留**——限制在无承载物时自动无对象，
不产生虚假能力承诺，删它反而把一条约束变成沉默。八级让路第 8 级「插件运行时按机制一取得其外壳份额
并受其突发上限约束」属前者的前半、后者的后半，须按此拆开重判。

**补裁乙　backup-writer 的磁盘 IO 绝对突发上限不进第 13.1 章配额表。**

第 4.1 节判它「保留待实测」，第六节又要求 1150 磁盘 IO 侧整段删——两条并读会把它一起删掉。
本节收口：该项**保留，但不进配额表**。理由是配额表三列都是百分比，绝对值进表就要折算，
而折算已被己-1 与本裁定两次禁止。其落点是**部署侧静态限额文件与部署记录**，
规格只写「全量备份写出另有一个磁盘 IO 绝对突发上限，其取值不在本表内，运行期承载待实测」。
这样既不复活折算，也不把唯一还可能落地的磁盘 IO 手段丢掉。

**补裁丙　内存一列的绝对字节取值来源，须在表下注开一个例外分句。**

表下注写「本表在本平台不构成运行期机制取值」，而第 4.1 节又要求 `MemoryMax` 保留并落
`JOB_OBJECT_LIMIT_JOB_MEMORY`。两句并读，读者会问内存硬上限的数从哪来。
补一个例外分句：**内存一列是本表唯一在本平台有运行期承载的一列**，
其绝对字节按附录 D.2 的 BC-1 基线组合由该列百分数算定，不受本注限制。
这是己-1 第二节第 1 类的原有口径，本节只是把它在规格侧写明。

**补裁丁　「二者不共享 CPU、内存与磁盘 IO 预算」在 1152 不在 263，第六节记混了。**

实测该串全文只在 1152，263 整行只有进程与资源单位的拆分要求，没有任何预算表述。
第六节写给 263 的第二半指令**在 263 没有落点**，据此作废；
其实质意图（磁盘 IO 侧按做不到一降级）**改在 1152 执行**。同源的 1160「二者不共享磁盘 IO 预算」
同批降级为「二者之间不构成磁盘 IO 预算隔离」，内存上限各自独立一句保留。

**补裁戊　「权重」与「保底值」两个残留词：保留，登记待收口，不本轮删。**

「权重」两处（1157、1682，同一句话）与「保底值」一处（1684）在磁盘 IO 一维归零、
CPU 比例首版固定不启用之后都失去承载物。但三处**都是否定性或引用性表述**——
「序号在前不等于取得更多绝对量」是拒绝优先级承诺，「按第 13.1 章的份额、保底值与突发上限执行」
是对该章的交叉引用、随该章漂移。删它们会把限制变成沉默、把引用变成断链，两者都比留着更坏。
**本轮一字不动；现行解释固定为硬件标定与认证意图，不登记“实测后自动启用”的后续动作。**

**补裁己　第 17.5 章与附录 A.4 须各补一句被测机器口径。**

第一节结论二把附录 A.4、附录 A.6 与第 17.5 章三处并列要求被测机器一律 Server 2022，
第六节却只给了 A.6 的落点。补：**第 17.5 章补一句** 2019 上的实测数据不写入认证报告、
需要 2019 背书须另立一次认证运行；**附录 A.4 补一句**性能与容量基线的测试服务器操作系统
取值与 BC-1 一致，即 Server 2022。两处措辞与 A.6 已落的那句同口径，不各写各的。

**补裁庚　Kubernetes 延期项不再是同一个延期对象，须重新登记。**

第六节问了这一问但没给结论，本节给：**不是同一个对象。**
旧形态下首版产物是 OCI 容器，Kubernetes 适配包等于沿用同一份镜像再加一层编排清单；
新形态下首版产物是 Windows 服务与 PE 二进制，走 Kubernetes 只剩两条路——
用 Windows 容器（受第一节结论一的主机与镜像版本匹配约束，在 2019 至 2022 区间内一份制品盖不住两版），
或重新产出 Linux 制品（直接违反第十三节第三条的零 Linux 开发）。两条都不是原延期项所设想的那件事。

处置：第 5.7 章该延期项**改记为「Windows 节点的 Kubernetes 编排」**，
并注明其前置条件是目标版本区间先收窄到单一 Server 版本；原「Kubernetes 适配包」的
描述作废。第 68、1372、1534 三处同批改，不得一处改三处留。

**补裁辛　四处「容器」残留逐处判。**

| 位置 | 现状 | 处置 |
|---|---|---|
| 224（两处合一句） | 「需要使用受控容器，其中受控容器只适用于第 9.3 章的服务端 WASM 插件宿主」 | 与 1826 同源同义，**同批改为 WASM 沙箱**。第六节只列了 1826，属漏列 |
| 1151 | 「操作系统、容器运行时与文件系统缓存的最低预留为该服务器总量的 CPU 2%、内存 5% 与磁盘 IO 10%」 | 本平台没有容器运行时。**措辞改为「操作系统与文件系统缓存」，三个取值一字不动**——取值是整机预留，少一个消费方只会更宽松，不构成放宽承诺，因此不触及认证冻结量 |
| 1361、1362 | 供应链安全的「容器扫描」与「安装包、容器、模块和插件全部签名」 | 按第 4.4 节同口径，被测对象**换成安装包与 PE 二进制**。第六节只点了计划侧 `14:530`／`14:532`，规格侧属漏列 |
| 971 | 「首版不含服务端隔离容器形态」 | **不动**。该句在新平台仍为真，且它是排除项不是能力承诺 |

**本节八条补裁不新增机制、不新增进程、不改变 F-08 任何已有结论的方向，
只补它没写到的落点与依据。**

#### 十五、对抗性复核查出的七条（本轮补裁，其中一条修正第 4.1 节本身）

规格改完后做了一轮对抗性复核：五个视角各自独立找自相矛盾、悬空引用、虚假能力承诺与越权删除，
共报 43 条，每条再派一个默认判其不成立的反方去证伪，**证伪掉 32 条，存活 11 条，去重为 7 个**。
证伪率 74% 说明这类复核确有必要，也说明不加证伪直接采信会引入大量噪声。

**补裁壬（本节最重的一条，修正第 4.1 节的承载物裁定）　
服务宿主层覆盖不到第三方进程，PostgreSQL 一行不得就此判为无承载。**

事实认定。第 4.1 节裁定承载物是「服务宿主层自我指派进具名 Job Object」，而服务宿主层是自研代码，
八个二进制共用。**PostgreSQL 16 与反向代理不链接该层，无从自我指派。**
改前不存在这个缺口：cgroup 侧的 `Slice=` 能收编任何第三方服务，仓内实物
`deploy/systemd/system/app-db.slice.d/10-resource-limits.conf` 就是给 PostgreSQL 设的 `MemoryMax`。
而配额表内存一列是本平台唯一还有承载的一列，PostgreSQL 一行又是全表最大的一行（内存 48%）——
把它判成无承载，等于在唯一还能落地的一列上放弃最重要的一行。

第 4.1 节当时排除了两条路（安装器创建后退出、新建常驻句柄持有者），**但漏了第三条**：
**由 ops-agent 创建具名 Job Object 并以 `AssignProcessToJobObject` 把第三方进程指派进去。**
这条路同时满足第 4.1 节自己立的两个约束——ops-agent 是八进程之一，不新增常驻进程；
句柄由常驻进程持有，内核对象不会随最后一个句柄消失。**据此取这条路，不取「宣告无承载」。**

四项须实测，任一不成立即退回宣告无承载（并入附录庚五，编号接第十二节现有 14 项之后）：

| # | 测什么 | 不成立的后果 |
|---|---|---|
| 15 | ops-agent 的虚拟账户能否对 postgres 进程取得 `PROCESS_SET_QUOTA` 与 `PROCESS_TERMINATE` | 指派做不到，退回宣告无承载 |
| 16 | 目标进程若已在某个 job 内，嵌套 job 能否叠加内存上限（区间两版均为 Server 2016 之后，接口面应具备，行为未测） | 同上 |
| 17 | 服务启动次序上 postgres 先于 ops-agent，指派前存在一个未受限窗口，该窗口实测多长 | 窗口过长则须另想办法，且**不得把它说成「启动即受限」** |
| 18 | **给数据库设内存硬上限本身的后果**：Job Object 触限是分配失败返回错误，PostgreSQL 在分配失败下是查询失败还是后端进程崩溃 | 若为崩溃，则该上限本身有害，宁可不设并如实宣告无承载 |

第 18 项比前三项更要紧：它问的不是「能不能设」，而是「该不该设」。
本卷不接受一个会把数据库打崩的配额。

规格侧的落地：第 13.1 章承载句与表下注**各加一个限定分句**，写明该列的承载分两类——
八个自研二进制走自我指派，PostgreSQL 与反向代理走 ops-agent 指派且**待实测**；
在第 15 至 18 项出结论之前，这两行按待实测处置，不得写成已覆盖。

**补裁癸　1154 首句的保底承诺是本轮唯一漏改的正面许诺，须重判。**

规格逐字「每行的份额同时是该组件的保底值：其余组件全部满负荷时该组件仍应获得其份额」——
这正是做不到一与做不到二点名已无承载的那个含义，而同章 1159、1164、1165 三处都已按重判写成否定式。
四个视角独立报了同一处，成因是那条 bullet 的后半段被改写、首句因锚点落在后半而幸存。
按 1164、1165 已有的重判体例改写：承诺半句删，「不另设第二层保底」这半句是限制，保留。

**补裁子　1174「上表是生产运行期配额」与表下注正面冲突，收口。**

表下注已写「上表在本平台不构成运行期机制取值」并只为内存一列开例外，1174 却把整表重新宣告为
生产运行期配额。该行本轮未改（不在第六节行号表内），属漏收口，按表下注口径改写。

**补裁丑　1155 插件外壳的触限限流承诺无承载，按承诺与限制拆开。**

规格逐字「插件运行时整体触及突发上限时限流其调用，不挤占核心与数据库的份额」是一条运行期主动机制，
磁盘 IO 一维已删、CPU 比例首版固定不启用，无承载物。按补裁甲立的规则处置：承诺半句重判，限制半句保留。
另注：插件运行时自身的燃料上限、内存上限、实例数上限与执行时限（第 4.8 节）**与本条无关，一字不动**——
那是应用层闸门，不依赖操作系统机制。

**补裁寅　1159 的「权重」二字被越权删除，复原。**

补裁戊逐字判「『权重』两处（1157、1682，同一句话）……本轮一字不动」，
但落地批次早于补裁戊写出，1157（今 1159）那处已被改掉「而权重高」「而权重低」两处，
1682（今 1684）那处则原样保住。**被明令不动的两处只动了一处，两处同文的句子当场不一致。**
按补裁戊复原 1159，与 1684 恢复同文。

这一条记下来的价值大于它本身：**补裁写在落地之后，就管不住已经落地的东西**。
下次平台级变更，补裁要在分段落地之前出。

**补裁卯　1161 的「按上文」指向已被删除的句子，把被误删的事实句还回去。**

1161 逐字「本级两行按上文不设 CPU 与磁盘 IO 突发上限」，其所指的上文原句
「承载第 1 级负载的 PostgreSQL 16 与 Rust 核心与集成网关两行不设 CPU 与磁盘 IO 突发上限」
随折算规则一并被删。但**这半句不是折算规则，是一条关于表的事实**，属误删连带。
按补裁甲的承诺与限制之分，它既非承诺也非限制，是取值事实，应还回上文。

**补裁辰　PRD 甲十六仍写「Kubernetes 适配包」，与补裁庚改名后的规格四处不一致，同批改。**

补裁庚只点了规格三处，PRD 那处漏列。改为「Windows 节点的 Kubernetes 编排」，与规格同名。

**本节七条同样不新增机制、不新增进程；补裁壬修正第 4.1 节的一处覆盖面遗漏，
方向不变（仍是具名 Job Object，仍不新增常驻进程），只把落实者从一条扩为两条。**

#### 十六、第 4 步落地前的交叉核对（本轮补裁六条，其中一条更正第十节的停写线）

技术基线与阶段 1 计划的改写提案产出后，先做了三个视角的交叉核对（判据改完还判不判得出来、
计划有没有比规格承诺得多、有没有违反停写线），**报出 35 条，其中 10 条必改**。
这批提案因此**没有直接落盘**。下列六条补裁定完之后才落。

**补裁巳（更正第十节的停写线，本节最要紧的一条）　
停写线禁的是「写死取值」，不是「改成待定」。**

第十节停写线第四条逐字：「在第 3 步的实测出结论之前，不得把任何配额取值写进新的承载文件，
也不得改写 `01:24`／`:457`／`:507` 的判据文字。」

核对指出：第 4 步正在改写这三处，而第 3 步（庚五原十八行中的十七项有效实测）**一项都没跑，也跑不了**——
本方手上没有一台 Windows Server 2022。按该句字面，第 4 步永远不能开始。

**这是我写的那句话过宽了，本条更正它。** 立该线的意图是防「把一个未经实测的数写死进承载文件，
日后没人记得它是猜的」。把判据**改写成待定形态**不但不违反该意图，正是第十节第 3 步自己要求的
（逐字「因此第 2 步的这三处先写成待定，不要先写死再回改」）。两句原本就是一个意思，
是停写线那句漏了限定词。

更正后的停写线第四条：**在第 3 步的实测出结论之前，不得把任何配额取值写死进新的承载文件；
`01:24`／`:457`／`:507` 三处的判据文字可以改写为待定形态，但不得写成任何依赖实测结论的确定判据。**

记一条：**本卷的停写线一律要写清禁的是哪一种动作**。「不得改写某处文字」这种全称禁令
会把合法的降级动作一起挡住，下次不要再这么写。

**补裁午　「承载分两类」必须贯穿到底，四类取值段不得把两类合回一类。**

核对查出四处提案（`00b:263`、`01:221`、`01:511`、`01:535`）在前文抄了补裁壬的「该落实分两类」，
到四类取值那一段又写成「每个资源单位一个内存硬上限」「八个资源单位都有内存硬上限」，
**把规格 1149 明确排除在例外之外的两行一并算成已覆盖**，正犯补裁壬「不得写成已覆盖」那一句，
且同一条提案内自相矛盾。

处置：凡写内存硬上限覆盖面之处，一律带限定——**八个自研二进制的资源单位有内存硬上限；
PostgreSQL 16 与反向代理待实测，实测结论出具前不计入**。这条口径与规格 1149 逐字对齐，
计划不得比规格承诺得多。

**补裁未　计数与行序两处收口：八个二进制落在七个资源单位；配额表按行名指代不按行序。**

其一，八个自研二进制对应七个资源单位（`core-server` 与 `integration-gateway` 共用一个），
加 PostgreSQL 一个共八个。凡写「八个自研二进制各自的资源单位」的一律改「七个」，
或改用「各自所属的资源单位」这种不带数的写法。

其二，配额表「内置搜索索引」一行**位列第四不是第九**——本卷多处（含本裁定 己-1 第一节）
沿用「第 9 行」这个旧值。**本轮起一律按行名指代，不按行序**；已写的旧值不专门去改，
但不得再传播。这是附录戊登记的同一类计数失配，本卷自己又犯了一次。

其三，`00b:263` 提案里那句「分母缩小意味着两个资源单位欠配、其余六个超配」**整句删除**：
它是一条资源侧结论，而规格 1149、1152、1159 三处逐字禁止「据本句推出任何级间次序或资源侧结论」，
且它所依据的按权重归一化分配在本平台没有承载物。CPU 一列的缺口只影响硬件标定与认证意图声明。

**补裁申　四处恒真判据，按通则第六条换可判定替身。**

| 处 | 为什么恒真 | 换成什么 |
|---|---|---|
| 「静态限额文件中不出现 `MemoryLow` 与 `IOWeight` 两列」 | 这两个是 systemd 与 cgroup 的键名，新文件是本平台自定格式，**任何写法都不会出现** | 断言不出现任何按权重的磁盘 IO 份额列与任何内存软保底列，并**逐名列出做不到一与做不到二点名的三种冒充**：`ReservationIops` 冒充 `io.weight`、`MaxBandwidth` 写成份额、最小工作集冒充内存软保底；三种各配一个负样例 |
| D-07「十一个阶段中不出现退出码 1」 | `cargo build`／`clippy`／`test` 失败返回 101，`fmt --check` 返回 1，脚本返回 2 或 64——只禁 1 与 3 等于放过最常见的失败形态 | 按 `run-pipeline.sh` 自己的四类归类判：汇总里「不符」与「不可判定」各为 0 条、「未交付」恰为指定的两阶段、聚合退出码恰为该形态对应值 |
| `RG-CI-PROBE-ABSENT` 的符号半条 | 被测对象由 ELF 换 PE 后，`msvc` 的 release 产物把内部函数名放进独立 PDB，**PE 本体一般查不到该符号，无论探针是否编入都恒过** | 改判 PDB，或改判路由字面量 `/api/v1/system/echo` 是否出现在 PE 中；两条都不成立时如实登记该半条降级、只留依赖树一半 |
| E2E-07 的「全部退出码 0」被扩到含 PostgreSQL | PostgreSQL 停机走其自带包装器与 `pg_ctl`，退出码不由本平台代码决定，会因无关原因判红 | 退出码一项只判八个自研服务；PostgreSQL 一项只判 `sc query` 为 `STOPPED` |

**补裁酉　待定判据的复活谓词必须机器可观测，不得写「实测结论出具后」。**

核对指出：本批几乎每条待定判据的重新生效条件都写成「实测结论出具后按结论一次补回」，
而技术基线通则第六条第四句逐字要求「判据重新生效的触发谓词必须由判定工具自身可观测，
不得写成阶段号，也不得写成任何需要人工翻牌的动作」。**「实测结论出具后」正是人工翻牌。**

仓内已有正确体例可抄：`xtask/src/configdoc.rs` 的单据类型码判据逐字
「一旦该节登记表出现第一行就自动转为真判定」。

处置：每条待定判据配一个同款可观测谓词。本轮统一取——
**一旦 `deploy/` 下的静态限额文件出现对应取值行，该判据自动转为真判定**；
CI 侧则取 `pipeline-stages.tsv` 的状态列由 `undelivered` 变 `delivered` 即自动生效。
两者都是工具自己能读出来的事实，不需要任何人翻牌。

**补裁戌　核对查出的未覆盖落点，同批补齐，不留半改。**

`00b`：第 2 节进程清单表（表头含 `cgroup slice` 列，行内含 `app-core.slice` 等取值、
`ep-core` 等系统账户、两个 `.sock` 监听值）与紧随其后那句「core-server 与 integration-gateway
是两个进程但同处 `app-core.slice`」。

`01`：`63`、`67`（进程职责表里的两个 `.sock`）、`72`（`00b:35` 在阶段 1 的孪生句，
且它是 `xtask codecheck` 门禁的判据来源）、`213`（`clock-skew-within-limit` 读 `adjtimex` 与 `/proc`
——按第八节两支择一：给本平台一个真判据，或让它**永久停在未覆盖并登记**，不许不选）、
`356`／`377`／`384`／`386`／`391`（配置键默认值仍是 FHS 路径，与同批新取值正面打架）、
`460`（E2E-04，与 E2E-03 同因，退出码判据须同批补夹具，`stderr` 半条与做不到七同因）。

**半改比不改更坏**：同一张表里 E2E-03 补强、E2E-04 不补，同一份文件里 `00b:35` 改了、
`01:72` 没改，落地后就是两套取值并存。

**本节六条同样不新增机制、不新增进程。补裁巳更正本裁定第十节自己的一处过宽措辞；
其余五条是落地前的收口，全部在既有裁定的射程内。**

#### 十七、2026-08-21 Windows、CI、签名与客户端路线最终冻结

本节是 F-08 在时间上的最后裁定，覆盖本节之前及附录庚中仍把下列事项写成“待裁”“二选一”“落码前阻断”或“先不动”的旧句。旧句只保留追溯价值，不再构成现行实现约束。

1. **平台唯一。** 服务端取 Windows Server 2019 至 2022 原生服务，认证基线取 2022；不使用 Linux、WSL、Linux 容器、整机 Hyper-V 或第二套平台虚拟机层。Rust 服务端核心与协议只有一套。唯一例外是后续 F-55 §4.5 为受控 MCP 插件单次调用建立的短命 Hyper-V-isolated utility VM；它不是产品部署层。
2. **客户端 PoC 前移。** 己-3 的二选一取原选项一：阶段 13 正式拆为 13a 客户端与白标、13b 低代码与配置发布；13a 的移动薄 PoC 在业务移动界面大规模投入前执行。薄 PoC 的阈值失败只触发移动 UI 由 Tauri 换为 Flutter，客户端 Rust 核心九个 crate、服务端 Rust 核心、协议与数据模型不变。薄 PoC 只能提前产生切 Flutter 的否定结论；保留 Tauri 的肯定结论仍须第二批完整门槛表通过或取得书面豁免。
3. **签名唯一。** 生产 Windows 制品必须 Authenticode；开发与内部制品可使用内部 ECDSA P-256，但必须标记为开发签名且不得进入生产。Authenticode 证书可由软件厂商或客户提供，两种来源不得形成两套验签协议。
4. **CI 唯一。** 默认取内网 Forgejo 加 Woodpecker Windows agent；`cargo xtask ci` 是全部门禁的唯一入口和真值。任何 CI 平台文件都只是准备环境并调用该命令的薄适配器，不得承载或复制判定逻辑。
5. **证据状态唯一。** 第十二节有 17 项有效的首批实施验证门禁（原编号 12 已撤销，为追溯不重排其余编号），不是设计待决。先在 Windows Server 2022 执行，再在 2019 做同项复核；本裁定没有声称它们已执行或通过。未取得证据的能力标记为“未验证”，对应 CI／发布门禁保持非零；失败时只走本裁定预先写明的保守降级，不得自行改平台、改 CI 权威入口或改 Rust 核心。

据此，F-08-2、F-08-3、F-08-4 与己-3 的批次二选一全部关闭；附录庚一、庚二、庚五的相应行只作历史索引，不再占“未决”计数。

### F-09　使用方对新需求的三条裁定：人工闸门的读法、AI 走本地推理、服务器端独立 UI

**裁定方向：三条均由使用方直接表态，本裁定只作承接与影响面登记，不代拍任何未表态项。**

上游是 `docs/superpowers/specs/2026-08-14-new-requirements-gap-audit.md` 的第六节十三条待裁。
本轮表态覆盖其中第 1、3、10 三条，其余十条**仍未表态，不得按任一读法推进**。

#### 一、F-09-1　「所有确定项目明确需要人工手动」取**乙**读法

盘点把这句话拆成两种读法：**甲**为不许自动过账，**乙**为事实的登记必须人来做。
使用方裁**乙**。

据此逐条落实：

1. **财务内核一字不动。** 规格第 5.2 章十类事件按固定映射自动生成凭证这一机制**保留**，
   规格:664 的「财务模块是唯一权威写入者」、规格:298 的固定映射、PRD:381 三处**不改**。
   凡把这句话读成「要给每张凭证加人工确认」的，一律按本条驳回——那会推翻整个财务内核。
2. **人工闸门的落点是「事实的登记」，而这一层现状已满足**，不需要新条款：
   业务事件本来就由人登记，凭证是登记的后果。规格:1041 保留的六类高风险操作
   （合同生效、付款、开票、财务过账、结账、敏感导出）已有重新认证加审批，
   规格:1050-1051 已要求把认证方式、待签内容摘要、时间与设备写入审计证据。
3. **真正仍然成立的禁令是另一个形状**，本条不动它：全卷四处
   （规格:1632、926、1088、1260）写的是**值只能来自人工录入或固定规则，不能来自模型判断**。
   这四条在本轮裁定之后**继续有效**，且正是 F-09-2 与 F-09-4 的边界来源。
4. **净效果**：本条不产生任何新增条款、不动任何既有条款，**代价为零**。
   把它写进裁定是为了让「人工手动」这四个字日后不再被读成甲。

#### 二、F-09-2　AI 走**本地推理**，不调外部大模型 API

**这是一次范围解冻，属规格级变更。** 现状逐字：规格:45 把「首版不包含本地 AI、OCR、MCP、
向量检索与知识图谱」写在第 2.2 章**固定约束**里；规格:499 登记在第 5.7 章延期目录；
规格:1037-1039 的第 11 章**整章被掏空只留章号**；规格:1510 逐字要求
「恢复任何一项都需要重新经过范围冻结与规格修订，不得在首版通过低代码配置、插件或连接器变相实现」。

**取本地推理这一支的三条后果，逐条记明：**

1. **不撞「长期不实现」项。** 调外部大模型 API 撞的是首版对外出口白名单
   （首版对外只有电子签章一类）与完全离线要求；本地推理只撞延期项，**是较轻的一刀**。
2. **延期理由的基础已消失。** 第一轮收窄把 AI 判为延期项，依据逐字是
   「收束版全文未提及 AI、MCP、向量检索、知识图谱或 OCR」——理由是**客户没提**，
   不是做不了、也不是不该做。使用方现在提了，该理由不再成立。这使本次解冻在论证上成本很低。
3. **但代价不在论证上，在别处**，须一并承认：第 11 章要整章重建；
   十四阶段计划对 AI **零覆盖**（18 个文件逐词检索，命中全是标识符误命中），
   需新增至少一个阶段，无现成 crate、端口或表可挂；
   规格:1039 承诺「按原设计补齐」，但原设计的分级细节**已从现卷读不到**，
   须先找回被删前的版本，或重做一次设计——**这一点本裁定标为不确定**。
4. **模型文件的交付形态未裁。** 交付形态已按 F-08 改为安装包加服务注册脚本；
   模型权重是否进这个包、多大、可复现构建与 SBOM 与签名怎么算，**本轮不裁**，
   随 F-09-4 的形态一并定。

#### 三、F-09-3　服务器端要**独立 UI**，是第三个 UI 承载面

**这是新增一端，属规格级变更。** 现状：UI 面只有两个——四端企业客户端
（同一套组件包，只按端切布局密度与入口可见性）与供应商门户 Web；
系统管理、低代码配置与运维**不是**独立管理台，而是四端里的一个能力域
（规格:608 桌面完整、移动仅查看）；运维中心在实现上是一组 API 加 ops-agent，没有自己的界面。

据此，本条触发的连带**逐项登记，不得漏改**：

| 触发项 | 动什么 |
|---|---|
| 规格第 6 章 | 新增一端及其定位（权威面），并写明它与四端、与门户 Web 的关系 |
| `ClientKind` | 新增一个取值。注意 `ep-foundation` 的 `ClientKind` 是**冻结项**（技术基线第 1.4 节记 6 个取值，`xtask archcheck` 的 `foundation-frozen-items` 逐项计数），改它必须同批改基线与该门禁的期望值 |
| 能力矩阵 | 由 18 个能力域乘 4 端共 **72 格**变为乘 5 端共 **90 格**；`client_capability_values` 新增一列；逐格核对与验收矩阵随之扩面 |
| 冻结机制 | 能力矩阵是**编译期冻结进二进制的常量**，数据库表只是机器可读副本；扩端要重编二进制并重跑逐格核对 |
| 阶段 13 | 制品与用例扩面；该阶段已为「新增端列」留了扩展位，但那是按延期项留的 |

**本条与 F-09-4 的关系须先说清，否则会做错**：服务器端 UI 是权威面、面向管理员，
把数据分析 AI 放在它上面，等于**把最高权限与最不可控的输出放在一起**，是风险最高的放法。
**本裁定因此明确：AI 分析的可用性不由「在哪个端」决定，而由调用人的权限决定**——
每个用户在自己权限内可用，RLS 天然生效；不得做成「只有服务器端 UI 上的管理员能用」。

#### 四、F-09-4　数据库端的数据分析 AI：本轮只定边界，形态另裁

使用方要「数据库端数据分析 AI，尽可能精细化、颗粒度」。**形态由专项设计另出**，
本条只把**不可逾越的边界**先定死，防止形态设计跑偏：

1. **推理进程不得拥有自己的数据库读权限。** 它必须在**调用人的安全上下文**里工作。
   否则就是权限旁路——这是本平台上能打的最大的洞，且会使全部 RLS 断言失去意义。
   这条与 F-09-1 的乙读法同形：**AI 产出建议，事实的读取走既有权限通道**。
2. **现有 RLS 断言矩阵测不到 AI 的泄漏通道。** 裁定 C-05 的十个断言测的是 SQL 层的
   读、写、更新、删除、聚合、排序、报表投影与报错泄漏，**测不到「模型把它看到的东西转述出去」**。
   这是一条**新通道**，必须新增断言，且新断言要仿 C-05 的形状逐字冻结函数名。
   **在新断言写出来之前，本功能不得进入任何退出条件。**
3. **颗粒度与权限风险正相关。** 越细越贴近原始数据、越需要跨模块 join，
   而跨模块 join 正是 RLS 最容易被绕开的地方。因此「尽可能精细」**不是一个可以无限逼近的目标**，
   它有一个由权限模型决定的硬上限，该上限必须在形态设计里写明，不得含糊。
4. **本地推理是硬件决策，不是软件功能。** 它与 PostgreSQL 抢内存，而内存是 F-08 之后
   本平台**唯一还有运行期承载的一列**（且按补裁壬，PostgreSQL 那一行的承载本身还待实测）。
   规格第 13.1 章配额表九行里**没有推理这一行**；附录 A.4 的负载模型按 20 并发标定。
   **加一行推理，整张表与硬件标定要重做**，且要先回答一个采购问题：**要不要 GPU**。
   无 GPU 在 CPU 上跑推理，时延会直接撞第 16 章的 P95 通过线。
5. **规格:1632、926、1088、1260 四条禁令继续有效**：值不能来自模型判断。
   分析结论是**建议**，不是值；任何把模型输出直接写成账务事实的做法一律禁止。

**本条尚未裁定的**：形态（生成查询交既有通道执行／只在预授权数据集上工作／
分级混合）、颗粒度实际上限、新增断言的确切名字与判据、模型规模档位与硬件取值、
审计列集要不要动（技术基线第 9.4 节明写「不增不减」，动它本身需裁定）。
以上随专项设计一并出，出后回写本条。

#### 五、本裁定不覆盖的（盘点第六节十三条里仍未表态的十条）

盘点第六节原表 13 条，本轮表态覆盖第 1、3、10 三条，**余 10 条**，
编号沿用原表不重编：

| 原编号 | 仍在等什么 | 优先级 |
|---|---|---|
| 2 | **AI 的落点层级四档**：只读问答、生成建议草稿、调只读端口、调写端口。四档代价差一个数量级 | 高（见下方收窄说明） |
| 4 | **「深度配置能力」的落点在哪一层**：甲、哪些环节允许哪种补偿动作与谁能批（规格补条）；乙、单据状态机与流转前置条件可配（规格级变更）；丙、端能力粒度可配（与三层冻结条款正面冲突）。三档不能混着提 | **最高，词义澄清** |
| ~~5~~ | ~~**「冗余」指哪一种**~~ **已表态：不买第二台机器**，故「冗余」只取「坏了还能捞回来」一义。见下方 F-09-5 | **已决** |
| 6 | U-E-12 合同提前终止：是否允许、审批要求，以及终止后已派生的订单、收款计划、交付节点、采购需求、项目任务的处置口径 | 高 |
| 7 | U-H-07 更正凭证入口与 U-H-08 手工凭证入口，**须与 U-D-02 资金单据冲正一并决策**（PRD:4498 逐字要求）——三者是一条决策，不得拆开 | 高 |
| 8 | U-D-09 是否允许部分红冲 | 中 |
| 9 | U-A-08 全部默认审批链与审批人角色。证据侧已定，批准侧一条都没定，而清单逐字点名的四项正好覆盖全部补偿动作 | 中 |
| 11 | 移动端写入的拒绝点在服务端还是前端隐藏。这条便宜，但它决定「服务端是权威」在首版是真的还是半真的 | 低（代价小，但结论要紧） |
| 12 | 备份保留期与保留代数取值。销毁动作有实现方，判定何时到期的依据不存在——「一周前的恢复点还在不在」在规格层面未定义 | 中 |
| 13 | 「碾压」的可判定判据：选哪些代表任务、对谁测、赢多少算赢 | 低 |

**十条一条都不得按任一读法推进。** 第 4、5 两条是词义澄清，
不澄清后面所有讨论都会走偏，优先级最高。

**第 2 条经本轮表态被收窄但未定死，须单独说明**：使用方要的是「数据分析」，
而分析蕴含读数，故下限不低于**调只读端口**；F-09-1 的乙读法加四条模型判断禁令
已排除**调写端口**。剩下的真问题是**只读问答、生成建议草稿、调只读端口三者的边界**
——本裁定不代拍，随 F-09-4 的形态设计一并出。

**本节的一处自纠**：本裁定初稿在此处列了 11 项却写着「十条」，
且漏了原表第 2 条、又把原表第 7 条（三者一条决策）拆成三项。
两处错互相抵消，才使总数看着接近。这正是附录戊登记的同一类计数与枚举失配，
**本卷第三次犯**。成因也同源：凭记忆复述一张表，而不是回去数。
本节改为逐行带原编号的表，即为堵这一类。

### F-09-5　「冗余」取「坏了还能捞回来」一义；不买第二台机器

**使用方表态原文口径：不会买第二台机器。** 据此本条闭合，附录庚一与盘点第六节第 5 条一并撤销。

**一、确认这是什么决定，不是什么决定。** 这是一次**采购决策**，不是设计取舍。
其效果是：规格第 2.2 章的单服务器形态**保持不变**，裁定 F-08 的单机 Windows Server 原生形态
**不受冲击**，附录 A.3／A.4／A.6 的认证与演练口径**不重做**，阶段 1 与阶段 14 **不重写**。
**本条为本卷省下的是以季度计的返工与一次范围冻结**，这一点应如实记明，
它是本轮全部表态里唯一一条「什么都不用做」的裁定。

**二、「冗余」自此只有一义，全卷措辞据此收口。** 首版可承诺的是**坏了还能捞回来**，
现有五样：一份服务器之外的副本、数据库的连续归档与每日全量两条通道、
附件的 15 分钟增量与每日全量两条通道、密钥恢复材料分片存于至少两个物理地点、附件版本冗余。

**不可承诺的是坏了还能跑**：规格:1192 逐字「首版不承诺高可用，也不提供自动故障切换。
该服务器失效即停机」；规格:1377 已把「重要数据处理系统热冗余」与「关键计算设备硬件冗余」
登记为**等级保护三级永久性不符合项**。

**三、一条禁令原样保留并加强。** 规格:1377 明令**不得以服务器外备份、可恢复性目标、
恢复演练或反向代理站点隔离声称覆盖热冗余**。规格早就预判到有人会拿备份冒充冗余并提前堵死。
**本裁定重申该禁令，并把它扩到对外表述**：交付说明、合同与销售材料中
一律不得把「有备份」「有恢复演练」「RTO 4 小时」表述为「有冗余」或「高可用」。

**四、必须一并写进合同的一个数。** 规格:1610 要求 RTO 的对外承诺取
**硬件到位时长 + 分片取件时限 + 4 小时**三者之和，**不得只宣称 4 小时**。
不买第二台机器意味着「硬件到位时长」这一项由客户自备可用替换硬件决定，
该项不在平台可控范围内，须在合同里点名由客户承担。

**五、连带**：附录庚一该行撤销；盘点第六节第 5 条撤销；本条不新增机制、不新增进程、
不动任何既有条款，**代价为零**。

### F-10　剩余待裁的一次性处置：影响面驱动机制、财务补偿、配置与权限、运维与验收

**详本在 `docs/superpowers/specs/2026-08-17-f10-ruling-detail.md`（613 行）。本节只记摘要、
两处前提更正与本卷纪律相关的三条，正文不在此复述。**

**规模**：四簇并行起草，每簇配一个默认判其不成立的反方证伪。反方共报 81 条
（必改 39、应改 38、登记即可 4），**凡标必改的一条不留原样**。
最终裁 27 条（规格级 13、计划级 10、实现级 4），**撤下 22 条**，仍须使用方拍板 12 条。

#### 一、前提更正之一（要害）：影响面机制**不是**发票红冲的前置，两条路各自成链

本轮交办文假设「合同终止与发票红冲的下游处置都挂在影响面机制上，它是前置」。
**这一半不成立，本节更正。**

理由是硬的：红冲释放核销转预收要求与红字凭证**共用同一次期间解析、子账腿与总账腿落在同一事务**
（计划10:802、规格:368 逐字「同一业务事件产生的……台账条目，与该事件的总账凭证共用同一个
会计期间字段」）；而影响面机制是 **Outbox 异步驱动**。**异步做红冲会当场把子账腿甩到
与总账腿不同的期间**，直接违反规格:368。

据此定死：**影响面机制首版只挂 `clm.contract.terminated.v1` 一个上游事件**，规则集冻结为七条；
**发票红冲的下游处置在 `register_invoice_reversal` 同事务内直接完成**，两条路互不相通、
无前后依赖。日后要把红冲接入影响面机制，须先解决同事务与异步的矛盾，属另裁。

#### 二、历史前提更正之二（已被 F-54 的具名目录收口取代）

本段当时只证明“十个”不是全卷上限，未解决“十个中的其余七个没有名称”这一实现缺口。F-54 现行唯一值是：阶段 3 登记三个具名事件，阶段 13 登记三个具名事件；旧“阶段 3 十七个、阶段 13 十个”均撤销为未命名配额，不得照此实现。B-09 复核再撤销一条不可构造且重复的库存金额调整事件；全卷只以 `docs/event-catalog.md` 的 124 条具名行与代码常量集合相等为判据。

#### 三、机制骨架：不发明新形状，逐项照抄两处已验证的现成机制

影响面处置台账的形状**照抄正向派生编排与 `recon_discrepancies`**，不另造：
批次加逐项、双唯一约束、八档退避、死信、`WAIVED` 加 `approval_ref`。
**闭合判定只有一条且是计数式**——`item_done = item_total` 且不存在 `DEAD` 项，
与正向的守卫逐字同形，是数据库里查得出真假的谓词，不依赖任何人的判断。

**「推着走完」的主杠杆是状态阻断，不是待办清单**：上游对象在处置未闭合前停在显式的
处置中状态（合同侧即新增的 `TERMINATING`），闭合才进终态。
**它把「没处理完」变成一个绕不过去的对象状态，而不是一张可以永远不看的清单**——
这是本轮最要紧的一处设计取舍。

#### 四、撤下的 22 条里，有三条值得单记

1. **「终止列为第七类高风险操作」撤下**。「六类」口径在 `specs/` 下逐字命中 **36 处**，
   其中规格:1330 是身份与访问控制测试的**判定文本**、规格:1433 是第 19 章退出条件、
   规格:1835 是性能认证的**负载构成**（六类改七类要重标定审计事件发生频次，
   可能使附录 A.2 的时延通过线需重跑）。更直接的是它的原判据
   「`client_capability_values` 行数仍为 72」**当场撞使用方已表态的 F-09-3**（72 格改 90 格）。
   **须经 TERMINATION 审批链这一半本轮照裁**；是否升为第七类单列待表态。
2. **C-1 的 SHA-256 冻结机制撤下**。它照抄的是零可配面的全冻结表，而补偿策略表**声明可配**：
   客户第一次合法发布收紧包，表哈希立刻偏离基线，系统随即拒绝一切写入，
   **收紧永远不生效且此后任何配置包都写不进去**；哈希比对也分不出「篡改」与「合法发布」。
3. **D-13-3 的独立文本检查撤下**。它与同簇的 D-3 必须同批落地却**互相判违反**
   （规格新增的第 21.22 节必然出现「碾压」二字）；「作为承诺性表述」这一限定
   也不是文本检查能判的，只能退化为子串匹配（则恒假）或人工评审（则不是机检）。

#### 五、本卷纪律：第四个计数高危点，本轮**刻意不给数**

本轮改动横跨阶段 3、6、7、9b、10、12 六个阶段，
八份计划的交付物、错误码、表、迁移、事件、视图、退出条件**七类计数都要加**。

**本裁定不给出任何具体新数值。** 理由逐字照抄详本：
「本卷已因『凭记忆复述一张表而不回去数』在计数与枚举失配上犯过三次
（附录戊、F-09 第五节自纠），本份裁定不给出具体新数值——**给了就是第四次犯同一个错**。
正确做法是入卷时逐份文件回去数一遍并当场核对。」

同源的一处已登记：**阶段 6 的事件总数是 18 还是 20 本轮不代数**——
计划06:612 逐字「本阶段的事件总数固定为 18……其余九个是合同与销售订单状态机的迁移事件」，
而该状态机有 13 条边、销售订单另有七条上下，**九个名额本就是从二十条上下的迁移里选定的子集，
不是闲置余量**，而 A-5 又新增四条边。须逐条数清后确定。

#### 六、与已表态项的对齐

详本第四节把「第 5 条冗余」列为仍须拍板——**该条已由 F-09-5 表态闭合**（不买第二台机器），
详本成文早于该表态，以 F-09-5 为准。详本对它的提醒仍然成立且应记明：
**D-1 的备份保留期与 D-2 的降级窗口都建立在单机形态之上**。

### F-11　使用方对 F-10 四条待表态的裁定

四条均由使用方直接表态。本节承接并把各自的连带写全；其中第四条使用方选的是较贵的一档，
且该档在 F-10 撤下时被反方指出**判据不可复算**，本节一并把判据修好，否则门禁落不了地。

#### 一、F-11-1　终止：不改「六类」口径，单独给终止动作加一次重新认证

**表态**：不升为第七类高风险操作；「六类」那 36 处逐字口径**一字不动**；
但终止动作本身**在 TERMINATION 审批链之外另要求一次重新认证**。

据此定：

1. **终止的闸门是两道**：TERMINATION 审批链（审批人不得等于发起人、原因必填、乐观锁匹配）
   加**发起时的一次重新认证**。承载复用既有的 `POST /api/v1/platform/reauth-challenges`，
   不新建机制；次序按 F-10 的 C-4 裁定——**能力闸在前，重新认证挑战在后**。
2. **必须单独写清这个例外的理由，否则日后会被当成口径不一致。** 落点定在
   规格第 5.2 章的 CLM 条目（不是第 12.2 章的高风险操作枚举——**动那里就等于改六类**）。
   理由逐字写明：终止是**唯一一个会把已发生事实的下游处置整批打开**的动作，
   其影响面不由发起人独自决定，故在审批之外另加一次身份确认；
   **本例外不扩大第 12.2 章的六类枚举，也不适用于任何其他动作**。
3. **一处须核不须改**：规格:1835 的性能认证负载构成按「六类」标定审计事件发生频次。
   终止是低频动作（合同级、非日常），**本裁定判定其不改变该标定**，
   但入卷时须在附录 A.4 的负载模型里加一句注明该动作未计入频次基线及其理由，
   不得默默不提。
4. **代价**：规格第 5.2 章加一段、阶段 6 的终止端点加一次重新认证校验、
   对应加一条用例。**36 处「六类」口径零改动，性能基线不重跑。**

#### 二、F-11-2　数据分析 AI：本轮不定形态，先交第一步；「结果回不回模型」等实测

**表态**：先不定「结果不回模型」这个折扣接不接受，**先交第一步**。

据此定：

1. **第一步照原设计交，不含任何模型、不含 `ai-inferer`、不含一行推理代码**：
   按调用人权限二维裁出的字段目录投影器、结构化即席分析入口（人从裁剪后的目录里选）、
   校验器、`testkit/src/ai_containment.rs` 与 CI 目标 `tests/ai_containment` 的**四条断言**、
   发布门禁项 `RG-AI-CONTAINMENT-GREEN`。
2. **这一步不依赖本题**，也不依赖 GPU 采购、第九个进程、配额表改动与庚五第 15 至 18 项实测。
   它自身有价值：这组下拉框**今天不存在**——普通用户读不到字段目录，也没有即席取数入口。
3. **本题的重开谓词是三个可观测的数**，第一步交付后即可测，不需要任何人拍脑袋：
   一、按人裁剪后目录的 prefill 耗时；二、受约束解码下 QueryPlan 的合法率
   （不合法即重试，重试次数直接乘在时延上）；三、15 路并发下 KV cache 总量
   （不同权限的人**不得共用缓存**，故这是乘出来的不是共享的）。
   **三个数一出，「够不够用」不再是估算。**
4. **在本题定下来之前，不得写任何依赖「模型看得到结果」的代码或判据**，
   也不得在交付说明里把本功能表述为「会分析」。首版可表述的上限是「会写查询」。

#### 三、F-11-3　影响面机制：只挂合同终止，预留扩展点——但「预留」的定义要卡死

**表态**：首版只挂 `clm.contract.terminated.v1`，同时预留扩展点。

**「预留」在本卷只许是下面这一种，不许是别的**，这一条必须写死，否则它会变成提前造抽象：

| 算预留 | 不算预留 |
|---|---|
| `ImpactRule::upstream_event_type()` 本就是方法而非常量，天然支持多上游——**不需要为此新增任何抽象** | 为第二个上游事件新增任何接口层、分发层或配置层 |
| `docs/impact-catalog.md` 的目录结构按「上游事件 → 规则集」组织，首版只有一个上游、七条规则 | 为尚不存在的上游事件预先建空目录节 |
| 在 U-J-13 处登记「合同变更与续签是已知扩展点，接入须另裁」 | 修改 U-J-13 的现行临时取值 |

理由：F-10 详本已点名，**抽象对不对得等第二个上游真接上去才知道**。
现在按想象造的扩展层，接第二个事件时大概率要推倒重来，而且推倒时它已经有了用例和门禁。
**首版规则集仍冻结为七条，`ImpactRegistry` 注册项数恰为七这条机检判据不变。**

#### 四、F-11-4　「从较早的备份恢复」进发布门禁——并修好它不可复算的判据

**表态**：修订附录 A.6 的判定口径，使该次演练进发布门禁。这是规格级变更。

**F-10 撤下这一条时反方给了两个理由，表态解决了第一个，第二个仍在，本节修掉：**

1. **第一个理由（已由表态解决）**：从十几天前的恢复点恢复，其恢复点距今以天计，
   按 A.6 现行判定标准（含 RPO 不超过 15 分钟）**必然不达标**，
   叠加规格:1867「两次均达标才判定通过」会把发布卡死。
   **处置**：修订规格:1864，为该次演练**单列判定项集合**——
   只判 RTO 不超过 4 小时、数据完整性、第 17.3 章全部强制不变量、附件与元数据一致性四项，
   **RPO 一项对该次演练不适用**，并在同处写明不适用的理由（恢复目标点由保留期决定，
   不是由归档周期决定）。规格:1867 的「两次均达标」对该次演练按其自身判定项集合判。

2. **第二个理由（本节修）**：原判据是「该次演练所用备份集等于该落点上 `verified_at`
   最早的 `DAILY_FULL` 的 id」——**不可复算**。「最早的那一份」随时间推移与回收任务执行
   而变动，同一份演练报告在两个时点会得出不同结论，而发布门禁必须能在
   **证据包采集时点稳定判真假**。

   **改判据为**：该次演练所用备份集的 `verified_at` 与**演练开始时点**的间隔
   **不少于 D 减 1 天**（D 为保留期，认证取值 14，故为不少于 13 天），
   且该备份集在演练开始时点仍处于有效保留期内。
   **两个量都在演练报告里，采集时点即可算定，事后重算结果不变。**
   它测的是「保留期尾端那一份还能不能恢复」这件事本身，而不是「哪一份最早」这个会漂移的名字。

3. **连带**：规格:1864 与 :1867 同批改；附录 A.6 的演练次数由每类两次变为
   「整机失效恢复两次 + 保留期尾端恢复一次」，该新增次数须同批写入 A.5 的发布判据与
   阶段 14 的退出条件；D-1 的保留期 D 一旦由客户改小，该演练的间隔判据自动随 D 变，
   **不需要改判据文本**——这是本判据取相对量而非绝对天数的原因。

#### 五、四条的共同连带

本节四条**均不新增进程、不新增常驻资源单位**。计数影响按 F-10 第五节纪律
**本节同样不给具体新数值**：F-11-1 加一条用例与一段规格文本、F-11-4 加一次演练与一条退出条件，
入卷时随 F-10 那一批**逐份文件回去数**，不在此代数。

### F-12　使用方对第二批四条待表态的裁定

四条均由使用方直接表态。第二条破了全卷最硬的一条枚举，**其触发面本节逐处点名**；
其余三条代价小，但各有一处必须同批说清的东西。

#### 一、F-12-1　规格第 21.10 章整条复活，控制手段取「模型不产值，只产计划」

**表态**：复活，不取「只删假陈述」也不取「留到第二步」。

1. **先说为什么这一条必须动**：规格:1632 逐字「本条随第 11 章 MCP 与本地 AI 整章延期，
   首版不适用：首版没有模型推理与 OCR 抽取，金额、账户、税额与合同字段一律由人工录入
   或按固定规则计算」。**前半句的前提在 F-09-2 裁定本地推理之后当场不成立**——
   规格里躺着一句已知为假的陈述，这比缺一条风险条更坏。
2. **控制手段逐字定为**：模型不产出任何值，只产出一份查询计划；取数、裁剪、执行、渲染、
   叙述全部由确定性代码在调用人的安全上下文里完成。**这比原设计要用的闭包校验硬得多**：
   它把风险从「模型可能算错一个金额」降为「模型可能写错一条查询」，
   而后者有确定性校验器逐条挡（数据集码与字段码落在本轮目录投影内、聚合项落在白名单内、
   过滤与分组与排序三个开关为真、只引用一个数据集码）。
3. **规格:1632 后半句原样保留且更要紧**：金额、账户、税额与合同字段一律由人工录入或按
   固定规则计算——这正是 F-09-1 取乙读法之后**仍然成立的那四条禁令**的同一条，
   本裁定重申，不得因 AI 入场而松动。
4. **同批**：审计事件的客户端枚举与 `ClientKind` 须为 AI 调用留出取值——
   但**本轮不加**，见第二节末的落地时点约定。

#### 二、F-12-2　新增第九个常驻进程 `ai-inferer`——本节把它破掉的枚举逐处点名

**表态**：取新增常驻进程，不取按需拉起、不取跑在 `core-server` 内。

**方向认定合理，理由记明**：按需拉起每次要重新加载模型权重，冷启动直接撞第 16 章的
P95 通过线；跑在 `core-server` 内则推理撑爆内存会打掉它，而它与 `integration-gateway`
共用一个资源单位，那是电子签章的**唯一出网进程**——用一个可选功能去冒险一条必需通道，
不划算。**常驻独立进程是三条里唯一在时延与爆炸半径两侧都站得住的。**

**但它破的是全卷最硬的一条枚举，触发面逐处点名（实测计数，不代猜）：**

| 落点 | 现状 | 要怎么动 |
|---|---|---|
| 规格「八个进程」 | 7 处 | 逐处改，且须区分「八个业务进程」与「九个进程」两种语境 |
| 规格「八个二进制」 | 5 处 | 同上 |
| 规格「八个自研二进制」 | 2 处 | 尤其规格 13.1 表下注那句「该例外对八个自研二进制成立」——它是内存承载覆盖面的唯一限定句 |
| `00b` 三串合计 | 10 处 | 含第 2 节进程表加一行（服务虚拟账户、资源单位、监听、数据库连接四列都要填） |
| `00b` 的新增进程禁令 | 1 处 | **必须开一条具名例外，不得默默改数**——该禁令是本卷防「进程数悄悄膨胀」的唯一闸门 |
| `01` 三串合计 | 14 处 | 含 D-05 判据「`sc query` 九个服务全部 RUNNING」→ 十个 |
| `14` 三串合计 | 4 处 | |
| `xtask/src/codecheck.rs` | `EXPECTED_PROCESSES: usize = 8` | **这是机检常量**，改基线进程表而不改它，`codecheck` 当场报计数漂移 |
| 资源单位数 | 八个（七个自研 + PostgreSQL） | 变九个；配额表要加第十行 |

**配额表那一行的取值本轮不可拍**：按 F-08 补裁壬，第 15 至 18 项实测出结论前不得为
任何行写死内存取值，而第 18 项问的正是「给数据库设内存硬上限会不会把它打崩」——
**本卷不接受一个会把数据库打崩的配额**，同理也不接受一个把推理挤死或被推理挤死的配额。

**落地时点约定（本节最要紧的一条纪律）**：**本轮只记决定，不改任何枚举。**
理由是硬的——`apps/` 下实测只有八个目录，`codecheck` 以「基线进程表 ↔ `apps/` 目录集合
逐项相等」为判据；现在把进程表改成九行，`codecheck` 立刻报「基线登记了 ai-inferer，
`apps/` 下没有对应 crate」。**枚举与实物必须同批变**，因此上表全部改动挂在
「`apps/ai-inferer/` 落地」这一个机器可观测的谓词上，与该 crate 同批提交，一次改完。
在此之前，规格与计划里的「八」全部保持不动，**不得先改数后补物**。

#### 三、F-12-3　采购发票（进项方向）不支持作废，只走红字冲销

**表态**：不支持。与规格现有条文一致——该段只写了红字冲销与金额税额更正，未提作废。

1. 采购发票状态保持两态，不加态。
2. **数据模型上 `INPUT + VOID` 本来可表达，本裁定明确禁止使用该组合**，
   并要求在阶段 10 的表定义处加一条 CHECK 或注释把它挡住——
   **可表达而不许用的组合，必须在库层挡住，不能只靠文档约定**。
3. 顺带消掉一处潜在冲突：规格明写「本系统不判定何时该用作废、何时该用红冲」，
   若进项也开作废，这句话在进项侧会与「两条路径互斥且各自只允许一次」打架。
   不开作废，该冲突不存在。

#### 四、F-12-4　效率验收：步数入门禁，用时只留证

**表态**：两个量纲都测，**步数作为硬验收条件，用时同测但只记入报告、不作门禁**。

1. **步数**（点击数加击键数）可复算、不受机器快慢与操作者熟练度影响、样本需求小，
   适合做门禁；**用时**贴近体感、对外好讲，但受机器、网络、熟练度影响大，
   要出稳定结论须大样本并按新手与熟手分组，成本高一个量级，**不适合做门禁**。
   两者并用，各取所长。
2. **两条硬约束，来自 F-10 详本对该条的必改意见，不得违反**：
   - **端别范围须逐任务按第 6.2 章能力矩阵裁剪，不得四端各测一次**——
     有些任务在移动端本就只能查看、写入转桌面端，在那里测它没有对象。
   - **量纲必须统一**：步数一律取「点击数加击键数」，不得与「字段数」混用或相减。
     F-10 详本点名原稿正是两者相减，那不是一个可复算的量。
3. **基线要先实测再定上限**：本轮不给任何具体步数上限，
   须先在本平台测出自家基线，再据以定线。**不许拍一个数当门禁。**
4. **对外表述仍受 F-10 的 D-3 三档约束**：任何比较级表述须有实测举证；
   在本条的基线与上限定下来之前，**第二档表述一律不得使用**。

#### 五、本节四条的共同登记

F-12-1 属规格级变更（第 21.10 章重写）；F-12-2 属规格级变更但**本轮零改动、全部挂谓词**；
F-12-3 属实现级补充（一条 CHECK）；F-12-4 属规格第 22 章验收标准级新增，本轮只定量纲与
约束、不定取值。**四条均不给任何计数新值**，按 F-10 第五节纪律，入卷时逐份回去数。

### F-13　对账差异事项的处置归属与关账拦截口径（裁附录辛第 14 条）

**提问是「差异处置端点归哪个阶段」。裁定的第一句是：这个问题的前提不成立。**

辛-14 说「差异清零前不得关账」没有解除路径，因而一个期间只要出过一条差异就再也关不上。
该结论建立在一个未经裁定的假设上——**关账拦截读的是 `recon_discrepancies` 的累计行集**。
卷内从来不是这么写的。前提一改，处置端点这件事本身就不存在了。

#### 结论一　关账拦截的判据是**本次校验的校验项结论**，不是差异行的累计集合

阶段 9 计划第 9.4 节关账请求状态机逐字两行：

| VALIDATING | PASSED | **全部校验项通过** | 同一快照内 |
| VALIDATING | FAILED_DISCREPANCY | **任一校验项差额非零** | 同上 |

两个触发谓词的主语都是**本次执行的校验项**，不是差异表的行。
规格第 10.2 章逐字把四种结束方式一一绑到同一个主语：
「校验通过时该法人该会计期间置为已关闭……**校验不通过时**按上条生成对账差异事项并拦截关账；
**校验按下两条判定未完成时**生成校验未完成事项并拦截关账」。

反向的支撑更硬：四个注册方的退出条件与规格自身一律写「清零→重发起→通过」——
阶段 7「差额清零后关账可通过」、阶段 8「注入清零后校验通过」、阶段 9「差异清零后重新发起正常受理并通过」、
阶段 11「清零后关账通过」。**若闸门读累计行，这四条无一能通过**，
因为历史 `OPEN` 行不会因为数据修好而消失。

**据此固定判据**：该关账请求在 `VALIDATING` 阶段发起的那一次 `recon_runs`
（`run_kind = PERIOD_CLOSE`、同一快照）产出的、`is_blocking_period_close` 为真的校验项，
其差异条数为零。`DAILY` 运行产出的差异不进受理前提、不进校验结论，只作预警。
**历史差异行一律是台账，不参与任何判定。**

#### 结论二　解除路径本来就有，且是业务动作不是置态

规格第 10.2 章逐字：「后三种情形下该会计期间**保持打开**，其过账、查询与报表不受影响，
**按事项载明的内容修复后可重新发起关账请求**。」
这一句同时保证了两件事：修复写入永远可达（期间没关，不撞「已关闭期间不再接受任何凭证写入」），
以及重新发起可达。

「修复」是什么，PRD 有全卷唯一一处展开：「由数据责任人按修复路径**补登或冲正来源事件**」。
是往账上写业务条目，不是给差异行改状态。规格另给出一条被点名的解除路径，
逐字「经审批确认不再冲回，或……由供应商红字发票冲回」——同样是业务动作。

#### 结论三　首版不为对账差异事项提供任何写端点，因此没有端点要归属

PRD 逐字：「**对账视图不提供任何调整、抹平或忽略差额的操作入口。**」
这是全卷唯一一次谈到差异事项的「操作入口」，而它是一条禁令。
PRD 与规格对差异事项的写侧要求是**空集**。

据此不新增 `start-repair`／`waive`／`repair` 任何一类动作，
不新增 `platform.recon_discrepancy.*` 任何权限项。**指不到出处的端点是自造需求。**

#### 结论四　三个已处置取值首版无生产者；**不撤列**，但禁止任何判据依赖它们

`REPAIRING`、`REPAIRED`、`WAIVED` 与 `repaired_by`、`approval_ref` 在首版没有生产者。
`WAIVED` 另有一层问题：规格第 10.2 章与 PRD 第 7 节全文**没有「豁免」二字**，
它在首版连语义依据都没有；同类形态裁定 F-10 已在另一张表上判过一次
（撤销 `WAIVED` 与其 `approval_ref`，理由是「落地后只能靠测试代码手工塞一个 UUID」）。

**但本裁定不撤列**，三条理由：
一、`recon_discrepancies` 与 F-10 那张表形状不同——那是一张在同一份裁定里现画的处置台账，
本表是已落地的检出事实表，有迁移号、有退出条件；「设计期不落一列」与「从已落的表上删四列」不是同一种动作。
二、撤列必然连带把该表改为仅追加表，而那要动裁定 B-02 逐字冻结的**十四行**登记、
`db/checks/append_only_consistency.sql` 的兜底计数、阶段 9 的第 16 号迁移与其退出条件 E-21——
改动面远超本裁定标的。
三、**它会与尚未裁定的附录辛第 12 条抢同一张 backfill 迁移**：辛-12 的两条出路之一正是
把 `recon_runs` 从仅追加登记里撤出。在一个可能被撤销的登记上叠加是错的次序。

**改为**：三个取值与两列登记为首版不使用，**任何判据不得依赖它们**；
其去留与该表的仅追加登记随辛-12 同批单裁。

#### 结论五　读侧露出归**阶段 9b**，端口由阶段 9a 定义

PRD 逐字「差额不为零时，视图展示对应的对账差异事项及其可追溯链路」，
而全卷没有任何端点能解析那个「引用」——`api/v1/…recon` 零命中。这一条今天落空。

端口 `ReconDiscrepancyQueryPort` 由阶段 9a 定义（9a 是表的所有者），
落 `crates/platform/recon/src/port/discrepancy_query.rs`；
**消费、内联与验收整条落阶段 9b**，与四类校验项同批。

不落阶段 10 的理由是决定性的：固定链是「……→ 10 → 11 → **9b** → 14」，
而十项勾稽的差异行只有 9b 那一个 `ReconCheck` 能产出
（阶段 9 计划逐字「9a 段只交付对账框架本体与调度，**不注册本模块的校验项**」）。
阶段 10 在 9b 之前，它交付这个字段时表里不可能有任何一行——**字段恒空、判据恒真**，
正是本卷在清的那一类。阶段 10 的现有措辞一字不改，由 9b 改写为内联并在 9b 退出条件上留证。
该形态有先例：`DisposalPort` 的端点由阶段 3b 声明、阶段 14 注入实现后放行。

#### 本裁定同批要求的落码改动

`crates/platform/recon/src/gate.rs` 的 `CloseFacts.discrepancies` 现声明为
「该期间**全部**差异事项的状态」并以 `!is_settled()` 过滤计数。按结论一，该语义错了；
按结论四，`is_settled()` 在首版**恒假**，于是那个过滤**恒真**——
一条一旦出现就再也减不回零的计数，正是辛-14 观察到的死锁在代码里的样子。
改为承载「本次 `PERIOD_CLOSE` 运行产出的、阻断性校验项的差异条数」。

#### 不在本裁定内的两件事

一、**死信 `REPAIRED` 与 `DISCARDED` 两条边的触发方**。原拟一并判，
但阶段 3 计划的三个处置端点逐行都不含状态字，而基线把**死信重投**列为 job-worker 职责、
把**全部平台端点**列为 core-server 职责——「端点同步返回置态」被这两处正面顶掉。
按排除法只推得到目标态、推不到触发方。移出为**附录辛第 14a 条**，由阶段 3 单裁。

二、**已关闭期间之外的跨期差额**。附录乙的缺口审计另记了一类：
一周内已到款又不涉及退货的红冲会造出一个「关账拦得住、却没有解除路径」的勾稽差额，
其唯一可清路径要求一张登不进去的退货单。那一类不由本裁定解决，
它的成因在业务规则不在对账框架。

### F-14　`recon_runs` 的仅追加登记与 `FAILED` 取值（裁附录辛第 12、13 条，并了结 F-13 挂来的 `WAIVED`）

**两条争了半天的路都是错的，正解是裁定 B-02 自己的第三个取值。**

#### 结论一　登记 `mode` 由 `APPEND_ONLY` 改 `IMMUTABLE_COLUMNS`，可变列取三列

B-02 逐字：「`mode` 取 `APPEND_ONLY` 或 `IMMUTABLE_COLUMNS`」。同一张登记表里，
**凡带状态机的表用的都是后者**：`platform_msg.outbox_events` 的可变列取
`status`、`attempts`、`available_at`、`locked_by`、`locked_until`、`last_error`；
`platform_msg.dead_letters` 取 `state`、`repaired_by`、`repaired_at`、
`approval_ref`、`discard_reason`。

`recon_runs` 有 `RUNNING` 到终态的迁移加两个要推进的计数器，形状与 `outbox_events` 同族，
却被登记成 `APPEND_ONLY | '{}'`。**这是 B-02 内部的一处误用，不是 B-02 与 A-06 打架**——
辛-12 把它记成两条裁定互斥，记错了。

**据此改判**：该行 `mode` 取 `IMMUTABLE_COLUMNS`，
`mutable_columns` 取 `status`、`batch_done`、`finished_at`、`termination_cause` 四列。

这条改判的代价小到可以逐项数完：**十四行的行数不变**，
第 16 号迁移的三行登记与三次 `attach_table_guards` 调用一字不动，
`db/checks/append_only_consistency.sql` 的兜底计数不动，
`db/checks/01_common_columns.sql` 的三列同缺口径不受影响
（那条豁免只对 `APPEND_ONLY` 生效，`IMMUTABLE_COLUMNS` 的表照带公共列）。
改的是登记表**一行的两个取值**。

证据性不受损：`legal_entity_id`、`run_kind`、`accounting_period_id`、`snapshot_id`、
`started_at` 与制品标识两列都**不在**白名单里，`assert_immutable_columns` 会拒改。
A-06 逐字「外部审计问某次关账跑的是哪一版校验由这两项唯一回答」照旧成立。

**否掉的两条路，各自的死因**：

**(a) 保留 `APPEND_ONLY`、撤销 `RUNNING`、行在运行结束时一次插入——自伤。**
规格第 10.2 章把「**执行进程异常退出**」列为五类终止成因之一。
崩掉的进程写不了自己的终态行，于是 `PROCESS_EXIT` 成为取不到的取值——
**与用来判 `RUNNING` 死刑的是同一条罪名**。更糟的是崩溃不留行：
闸门读到的会是上一次 `DAILY` 运行的 `COMPLETED` 而放行，
一次崩掉的关账前校验被读成跑完了，这是「错了不会当场报错」。

**(b) 照 `audit_segments` 先例把该表从登记里撤出——留下一个更隐蔽的洞。**
`platform_core.attach_table_guards` 的三个分支依次判 `APPEND_ONLY`、
`IMMUTABLE_COLUMNS`、有无 `row_version`。撤登记之后前两个落空，
而 `recon_runs` 按 A-06 不带 `row_version`，第三个也落空——**该表一个守卫都没挂**，
`UPDATE` 与 `DELETE` 全放行。而 `append_only_consistency.sql` 判的是
「登记↔同名触发器」双向一致，**无登记无触发器恰好一致、返回零行绿灯**。
一张给关账当证据的表就这么变成谁都能改的表，而门禁是绿的。

`audit_segments` 那条先例也不可比：它一行等于一法人一自然日，
被当天每一条审计写入反复取锁推进，是可变游标，而证据另落在
`audit_events`、`audit_anchors` 与证据文件上；`recon_runs` 一行只由执行器写一次，
且它自己就是那个证据。

#### 结论二　撤销 `FAILED`，`status` 收为 `RUNNING`、`COMPLETED`、`UNFINISHED` 三值

规格第 10.2 章把五类终止成因**全部**归入「未完成」，
阶段 14 的十八个降级 kind 只有 `RECON_RUN_UNFINISHED`，
全卷 `FAILED` 只在 A-06 那一行 CHECK 里出现过。保留它就要一次配齐四件：
逐字的产生条件、关账请求状态机的一条新出边、降级承接方、以及改规格的
「只有四种结束方式」。四件今天一件都没有。

**归因改由新列 `termination_cause` 承担**：取值域同阶段 9 计划的五值
（`BATCH_TIMEOUT`、`RESOURCE_LIMIT`、`PROCESS_EXIT`、`CONNECTION_RECYCLED`、
`SNAPSHOT_INVALID`），`COMPLETED` 时必须为空、`UNFINISHED` 时必须非空。
补这一列另有一条独立理由：规格要求台账条目载明「已完成批次与终止原因」，
而 `termination_cause` 今天只长在 `ledger.period_close_requests` 上，
`DAILY` 与 `RECOVERY_ACCEPTANCE` 两类运行**无处可写终止原因**。

已落码的 `summarize_run` 那两条 `Failed` 产生条件的去处：
一、「注册表的阻断性校验项不足十五」**前移为起跑前闸门**——不起跑、不落行、
`run` 返回 `Err`。此时闸门侧 `latest_run` 为空报「尚未执行过对账」，
同时 `ReconRosterIncomplete` 报差几项，两条一起给出，比一个 `FAILED` 具体。
二、「一批都没派发出去就断了」并入 `UNFINISHED`，`unfinished_check_codes` 仍为空、
`termination_cause` 非空——已落码的那句「无从归因时不得把期望集整个当成未完成项」因此保住。
`validate_run_outcome` 的第一条不变量同批改为「`UNFINISHED` 要么列出没跑到底的检查项、
要么给出 `termination_cause`，二者至少其一，皆空即拒」。

#### 结论三　撤销 `WAIVED` 与 `approval_ref`（了结 F-13 挂来的一条）

规格与 PRD 对对账差异全文没有「豁免」语义，F-13 已查实；
裁定 F-10 已在另一张表上判过同形，理由逐字「落地后只能靠测试代码手工塞一个 UUID」。

F-13 当时不撤列给的三条理由，本裁定逐条处置：
理由一「本表已落地」——**撤回**，那是一条恒真判据：`db/` 下十四张登记表无一有迁移文件，
它区分不出标的；
理由二「撤列必然连带改仅追加」——不成立，撤的只是一个取值与一列，
`state` 仍留三值、表仍是可更新表；
理由三「会与辛-12 抢同一张 backfill 迁移」——**因本结论一取 `IMMUTABLE_COLUMNS` 而消失**，
第 16 号迁移一字不动。

**形式照 `key_domains.domain_kind` 的先例**：**收既有 CHECK 的取值域**，
不在旁边另加一条 CHECK。另加一条会让 A-06 段同一列一处写四值、一处只放行一值，
那正是附录丁 D-04 判过的「本文件 A-06 段自身打架」。

`REPAIRING`、`REPAIRED` 与 `repaired_by` **保留**：这两态在规格里有语义依据
（「按事项载明的内容修复后可重新发起关账」、「由数据责任人按修复路径补登或冲正来源事件」），
F-13 结论二的解除路径走的就是「修复」。`WAIVED` 是唯一一个在规格里连词都找不到的。

`recon_discrepancies` **不进登记**，维持 A-06 逐字「可更新表，带 `row_version`」：
次版要开的正是 `OPEN → REPAIRING → REPAIRED` 的置态路径，
登记为仅追加会在那一天拒绝它——与 B-02 删掉五张往来台账的理由同形。

#### 未了结的一件，明写

**崩掉的进程谁替它写终态行**，本裁定给不出答案。
`PROCESS_EXIT` 这一值要求有人在进程死后把那行 `RUNNING` 推到 `UNFINISHED`，
而全卷没有看门狗、没有超时清理。这一条在 `ledger.period_close_requests` 上同样存在
（阶段 9 计划要求五类之一发生时写 `termination_cause`，同样没说谁写），
**是卷内既有的缺口，不是本裁定新造的**，故不在此处强行指派承接方。
本裁定只保证：那行留着（结论一），归因有列可写（结论二），
而闸门读到 `RUNNING` 会拦住关账而不是放行。

### F-15　死信四态的触发方（裁附录辛第 14a 条）

**一条不对称:丢弃是决定,修复是结果。** 决定当场生效,结果由投递说话。

#### 结论一　四条合法边

| 从 | 到 | 触发方 | 进程与事务 | 写哪几列 |
|---|---|---|---|---|
| （无） | `OPEN` | 系统 | job-worker,第 8 次失败的转死信事务 | 全部信封与载荷列 |
| `OPEN` | `REPAIRING` | **人工**,`start-repair` | core-server,端点命令事务 | `state`、`repaired_by` |
| `OPEN` 或 `REPAIRING` | `REPAIRED` | **系统**,那个真正把副作用做成的事务 | 该消费事务本身,不经任何端点 | `state`、`repaired_at` |
| `OPEN` 或 `REPAIRING` | `DISCARDED` | **人工**,`discard` | core-server,端点命令事务 | `state`、`approval_ref`、`discard_reason` |

非法边:`REPAIRED` 与 `DISCARDED` 出发的任何边;`REPAIRING → OPEN`
（不设自动解除认领——那会把「谁在管这条」这个事实丢掉,而 E2E-5 从 `REPAIRING`
直接走到 `DISCARDED` 不需要它）。

#### 结论二　列到路径的映射由现行文本唯一确定

阶段 2 计划逐字:「死信的可变列白名单按 B-02 取 `state`、`repaired_by`、`repaired_at`、
`approval_ref`、`discard_reason` 五列,**少登记一列即在上线后拒绝修复完成与丢弃两条路径的写入**。」

三条路径的列子集因此唯一:`start-repair` 的「记名」（阶段 3 计划逐字
「`OPEN` 到 `REPAIRING`,记名」）写 `repaired_by`；`discard` 逐字「请求体必带
`approval_ref`」并写 `discard_reason`；剩下的 `repaired_at` 只能归「修复完成」。
**反证:若 `start-repair` 也写 `repaired_at`,撤掉那一列挂掉的就是认领而不是修复完成,
上面那句话自己就说错了。**

据此阶段 3 计划第 3.5.4 节的「三个处置**端点**分别写入这五列中的不同子集」
改为「三条处置**路径**」——修复完成那一路不是端点。B-02 的白名单取值与登记行数一字不改。

#### 结论三　`replay` 端点一列不写,只动 Outbox 行

`replay` 逐字只有「重投,记名并写审计」,基线逐字「重投必须记名并写入审计」——
两处都只要求记名与审计,都没要求置态。

E2E-5 逐字「注入一个恒失败的消费处理器,事件走完八段退避进死信,
站内通知送达责任人,**重投仍失败**,双人审批后丢弃」——**重投可以失败**。
端点若同步置 `REPAIRED`,这条验收的后半就无从发生:状态已经是 `REPAIRED`,
而 `REPAIRED → DISCARDED` 不是合法边。

重投的动作是把 Outbox 行置回可投递:`status='PENDING'`、`attempts=0`、
`available_at=now()`、清 `locked_by`/`locked_until`,`last_error` 留着当证据。
**`attempts` 必须归零**:`crates/platform/outbox/src/delivery.rs` 的 `judge`
以 `BACKOFF_SCHEDULE.get(attempts_before)` 取档,不归零则八档退避全部不可达,
而同文件逐字「**每一档都要被用到**,否则表里就有一个永远排不上的取值,那本身是缺陷」。

#### 结论四　受理前提取 Outbox 行的状态,不取去重表有无行

**死信分两类,重投的效果相反。**

**类甲**（消费事务内失败）:阶段 3 计划逐字「处理器的副作用与
`INSERT INTO platform_msg.inbox_consumptions (consumer, event_id) ...` **在同一事务内**」——
失败整事务回滚,去重行一并回滚。重投有效。

**类乙**（消费成功、子项独立事务失败）:阶段 6 计划先在去重表上占位并提交,
再逐字「每个派生项一个独立事务」。这类死信重投源事件时,`consumption.rs` 的
`judge(true)` 返回 `AlreadyConsumed`,副作用一次都不做,Outbox 行置 `DONE`——
**端点返回成功,什么都没发生,死信永远停在 `REPAIRING`。** 这是丙类。
类乙自己另有出口:阶段 6 计划逐字「人工修复后可重放该批次」,只是不走死信重投。

**据此三个端点一律以「该 `source_event_id` 的 Outbox 行存在且 `status='DEAD'`」
为受理前提**,不满足即返回 `PLATFORM.DEAD_LETTER.STATE_INVALID`
并在 `details` 指向该模块自己的批次重放入口,**不得静默成功**。

**判据为什么不取「去重表里有没有行」**:阶段 3 计划把 `inbox_consumptions`
列入按期清理清单,而 `dead_letters` 在**永不清理的清单**里。
拿去重行当判据会在保留期满的第一天静默失效——**一条随时间失效的判据**,
本卷在别处已清过同形（恒真判据的时间版本）。Outbox 行的状态不随时间失效:
类甲永久是 `DEAD`,类乙是 `DONE` 或行已清理,两种都影响零行。

#### 结论五　转死信的 INSERT 必须改成不冲突,否则 E2E-5 跑不通

阶段 3 计划逐字「第 8 次失败后事务内完成三件事:插入 `dead_letters`、
把 `outbox_events.status` 置 `DEAD`、写审计事件」,而 `ux_dead_letters_source_event_id`
是唯一索引。**重投再失败走到第八次时,那个 INSERT 撞唯一键,整个转死信事务被打掉,
`status` 也置不成 `DEAD`。** E2E-5 逐字要求的「重投仍失败,双人审批后丢弃」
在现有写法下走不到第二步。

改为 `INSERT ... ON CONFLICT (source_event_id) DO NOTHING`。行留在 `REPAIRING`,
`repaired_by` 留着上一个认领人,出口是再重投或丢弃。
运维侧的信号走通知的轮次位——阶段 3 计划逐字 `dedupe_key` 取
`<notice_type>:<source_kind>:<source_ref>:<轮次>`,重投再失败是新一轮次,不被去重挡掉。

#### 结论六　并发与一处会静默出错的交叉

三个端点一律在同一事务内**先取 `dead_letters` 行、后取 `outbox_events` 行**,
两次都用条件更新,任一影响零行即整事务回滚。次序固定以防死锁。
并发判定沿用阶段 3 计划逐字「`UPDATE ... WHERE id = $1 AND state = $expected`
的受影响行数判定,冲突映射为 `PLATFORM.DEAD_LETTER.STATE_INVALID`」。

这把锁序同时堵掉一处静默错误:`discard` 若不看 Outbox 行,
**一次在途的重投可以在丢弃提交之后照样把副作用做成**,
落成一条「已丢弃、其实做了」的记录,而关账照过。加上 `status='DEAD'` 这一条,
在途重投期间丢弃被拒。

#### 结论七　为什么不照 F-14 撤销 `REPAIRED`

F-14 撤 `FAILED` 与 `WAIVED` 的判别式是「**产生条件与接收方两头都缺**」。
`REPAIRED` 的接收方是齐的:裁定 C-28 的拦截集逐字把它排除在「未修复死信」之外,
`crates/platform/outbox/src/delivery.rs` 的 `is_settled()` 判它已了结,
`crates/platform/recon/src/gate.rs` 的 `unsettled_dead_letters` 吃这个口径。
**缺的只是生产者。接收方齐、生产者缺就补生产者;两头都缺才撤取值。**

不补的后果:死信的唯一出口只剩双人审批丢弃,于是每一条真修好的死信
都得被记成 `DISCARDED` 才能关账——**把一次成功的过账记成「已放弃」**。
那比死锁更坏。

### F-16　生产交付机的硬件基线与其后果（ThinkStation P340，单盘机械硬盘）

使用方给定的生产交付机与决定，本裁定不复议这些决定，只把后果算清、写实、登记：

- ThinkStation P340，i5-10500（6 核 12 线程），32 GB 内存
- 1 TB 机械硬盘 + 256 GB SSD，**SSD 不动**（只承载操作系统）
- **全部落 HDD，含 `pg_wal` 与审计证据**
- 人机并发上限 **10**（低于规格第 3 章的 20，按第 16 章「通过线不因并发下降而放松」不据此放宽任何取值）
- 附件正文 **50 GB/年**，约 3 万对象/年，单个上限 100 MB
- 首版单盘交付，第二块盘是绊线触发后的处置手段，不是初始配置

#### 结论一　落盘布局，`pg_wal` 与 PGDATA 同卷同轴显式登记

| 盘 | 承载 |
|---|---|
| SSD（C:） | Windows Server 本身。**不动** |
| HDD（D:） | 安装根 `D:\EP`、PGDATA（**含 `pg_wal`**）、附件正文三命名空间、检索索引、连续归档本机保留、全量备份暂存、审计证据文件 |

#### 结论二　安装根取 `D:\EP`，这是**补充 F-08 而不是改判**

F-08 逐字定的硬约束是「**安装根目录取短名**」，`C:\EP` 是括注里的**默认值**。
`D:\EP` 满足同一条硬约束，且与 `C:\EP` **等长**——
F-08 要求的「`search\<legal_entity_id>\` 与附件三段式路径的最坏长度在该根下算一次留证」
一个字都不用重算。F-08 全文不改。

**但连坐面不是换一个盘符。** 全仓写死 `C:\EP` 的行共 **28 处**，散在七个计划文件里。
逐处改成 `D:\EP` 是错的做法——同一份卷宗已经因此漂移过一次
（`00b` 写 `%ProgramData%\EP` 而 F-08 写 `C:\EP`，本轮修掉）。

**据此本裁定要求把安装根收成单一取值**：全部路径配置键的默认值改写为 `<安装根>\…`
的派生表达，安装根由 `std::env::current_exe()` 的**祖父目录**推出（二进制在 `<根>\bin\`）。
不走注册表——F-08 第零节逐字排除了「要求改客户机器的系统设置」，
那正是它排除 `LongPathsEnabled` 的理由。派生是纯进程内的，可脱库单测。

#### 结论三　容量按七项重算，并把自检闸门先扣掉

**先说一处此前被当成余量的硬闸门。** 阶段 3 计划的配置项
`EP__PLATFORM__FILE__FREE_SPACE_MIN_BYTES` 取 107374182400（**100 GiB**），
而同阶段把「剩余空间不低于 `FREE_SPACE_MIN_BYTES`」定为 `attachment-store-ready` 的
**阻断级**判据，逐字「阻断级失败以退出码 78 退出」。
**这 100 GiB 不是余量，是闸门**——五项占满盘的那一刻 core-server 拒绝启动。
必须先从预算里扣，否则就是一处「验收当天恒过、运行期才炸」。

**A.3 的第五项拆为两项。** 「内置搜索索引与恢复及升级所需临时空间」把两个不同的
缩放动因（索引随可检索文本量、临时空间随数据库规模）合成一个数，
还共用 A.3 逐字要求的「按上述五项分别测量」一次实测——实测值回来**拆不回两个用途**，
重算时连往哪个方向调都判不出来。本裁定强制拆开分别记账。

| 项 | 本部署取值 | 自变量与依据 |
|---|---|---|
| ① 附件正文当前版本 | 150 GiB | 50 GB/年 × 3 年（使用方给定） |
| ② 附件历史版本与待物删副本 | 60 GiB | ① × (v − 1)，v = 1.4 |
| ③ 事务数据库数据与索引 | 40 GiB | 业务行约 2.4 GiB + 审计与事件类约 37 GiB |
| ④a 连续归档本机保留 | 40 GiB | R_wal 约 4 GiB/日（10 并发）× T = 7 日 × 1.5 余量 |
| ④b 全量备份本机暂存 | 24 GiB | 5 分钟 × 80 MiB/s 流式缓冲 |
| ⑤a 内置搜索索引 | 8 GiB | 可检索文本量 × 膨胀 × 合并期双份 |
| ⑤b 恢复及升级临时空间 | 48 GiB | ③ × 1.2 |
| **小计** | **370 GiB** | |
| **自检阻断闸门** | **100 GiB** | 上述 `FREE_SPACE_MIN_BYTES`，阻断级 |
| **合计 / 1 TB 盘可寻址 930 GiB** | **470 GiB = 51%** | A.3 的 80% 阈值为 744 GiB |

**交付前提（写入部署记录，不是运维建议）**：附件正文不超过 **50 GB/年**。
该前提是一个**被监视的假设**：A.3 的 80% 阈值即绊线，触线按 A.3 逐字给的三条处置
（扩容 / 按第 12.4 章物理删除 / 把容量暴露写入部署记录并书面告知客户）。
假设不成立时是绊线响，不是静默撑爆。

#### 结论四　A.3 自身的两处问题，登记

**其一，A.3 的「事务数据库数据与索引约 300 GB」与它自己给的行数差两个数量级。**
按 A.3 逐字给的行数（主数据 5,000×3、订单行 10 万×2、库存流水 50 万、
会计分录 150 万、自定义对象 10 万行）实算，堆表约 878 MB，
加索引与膨胀余量约 **2.4 GiB**。那 300 GB 只能由审计事件、事件外发日志、
附件元数据版本与容量指标采样这类**A.3 未给行数、也未给保留上限**的表填满。
**该项是与本节行数不自洽的独立拍值**，且它是七项里唯一单调增长而卷内无保留上限的项，
本裁定要求它进部署记录的年度复核。

**其二，`v = 1.4` 无出处。** A.3 逐字「本节按每对象平均保留 1.4 个版本取值」，
全卷只此一处——不是实测、不是引用、不引任何章。登记为占位因子，
部署前按客户实际版本策略重取。附带一处算术：800 × 1.4 − 800 = 320，
A.3 写「约 300 GB」，向下取整少算 20 GiB。

#### 结论五　认证走另一台机器，但必须补 A.4 今天没有的介质维度

A.3 逐字「A.4 的测试服务器本地可用磁盘容量不得低于本节下限」，下限逐字「2 TB」。
这一条无条件、无向下路径，在任何时延项被测之前就已不成立——
P340 上**起不了跑**，不是跑得慢。

三条候选里两条走不通:**下调认证基线**撞 A.3 逐字「任何情况下不下调本节数据集」
（另有四处同向逐字）；**按第 17.5 章永久性不符合项带暴露交付**也不行，
该章逐字「永久性不符合项只来自上一条的封闭清单」，且其对象是等级保护三级的控制项，
附录 A 的容量与时延基线不是等保控制点。

**选定:在另一台 ≥2 TB 的机器上做认证，且认证机与交付机在介质维度必须同构**——
单轴机械盘、`pg_wal` 与 PGDATA 同卷同轴、审计证据同盘，只允许容量维度不同构。

**「介质同构」这条要求今天卷内不存在，必须补写进 A.4。** 规格逐字
「测试服务器的 CPU、内存与**磁盘规格**记入认证报告，作为交付客户的服务器规格下限」，
而**磁盘规格没有介质维度**——全文 `SSD`、`固态`、`机械硬盘`、`NVMe`、`IOPS` 命中数为 **0**。
不补写就是一处丙类:**认证在 SSD 上跑通，报告的磁盘规格栏只能写一个容量数字，
交付一台同容量机械盘机器时逐字对照不会失败。**

这一条的分量比时延更重，因为阶段 14 计划逐字已经说了:
「第 13.3 章 **RPO 不超过 15 分钟在本平台不再有机制侧保证，其成立完全押在附录 A.4 的
认证实测上**」——F-08 已把「其份额之内的磁盘 IO 不向任何级别让路」整句删除。
**若那次实测跑在与交付机不同介质的机器上，RPO 15 分钟就没有任何支撑物。**

#### 结论六　全量备份排期是交付前提，不是运维建议

规格第 13.1 章逐字「备份与连续归档在生产上又必然与业务负载同时运行」——
这句对连续归档成立，对每日全量**不成立**:全量备份的时间窗是可以排的。

单盘形态下它是**唯一**能挡住备份顺序读与 WAL 顺序写抢同一磁头的手段。
因此**每日全量备份必须排在营业时间之外**，写入部署记录当交付前提；
第二块盘到位后才降回排期建议。

#### 结论七　三项必测，与预先定好的绊线-处置对

第 16 章的三条时延通过线在「单轴机械盘、`pg_wal` 与 PGDATA 同卷」这一配置下
**卷内从未测过**。按通则「待实测不等于通过」，
**在实测结论出具之前不得写成已覆盖、不得据以推出任何资源侧结论。**

| 必测项 | 被测对象与判据 | 绊线 |
|---|---|---|
| WAL fsync 时延分布 | 单轴 HDD 上 `pg_wal` 的 fsync P50/P95/P99 | 记入认证报告，无独立绊线 |
| 提交时延 | 10 并发下普通交易提交 P95 | **> 3 秒** |
| 备份窗口内退化 | 全量备份运行期间的提交 P95 与三项写出周期 | **提交 P95 > 3 秒，或归档写出周期 > 15 分钟** |

**任一项越线，处置固定为「加一块 HDD 专给 `pg_wal`」**——不是改代码、不是放宽通过线、
不动 SSD。处置在越线之前就定好，免得届时临时找办法。

加盘后的迁移路径同批固定:`pg_wal` 迁到新盘并**保持排他**（该盘只放 `pg_wal`，
别的什么都不放——专用盘轴的全部价值来自排他性，
再放归档暂存或审计证据就又变成混合负载）；连续归档随之变为跨盘拷贝，
从 WAL 盘读、写到数据盘的归档目录，两边都不与自己争。

#### 未了结的一件，明写

**本部署的时延与 RPO 结论，在认证机备齐并按介质同构跑出结果之前一律不成立。**
本裁定把「介质同构」这条要求补进 A.4，但**认证机本身不在使用方已给的硬件之内**——
它是一台另需备齐的 ≥2 TB 单轴机械盘机器。没有它，本裁定的结论五、七都只是安排，
不是结论。这一条不在本裁定能解决的范围内，如实登记。

### F-17　永久授权与维护订阅两项首版不交付（裁附录辛第 7 条）

#### 结论一　范围:首版不交付。依据有三段,其中两段是规格自身的文本

**第一段,使用方明示。** 本部署为**自有企业内部使用**,不存在厂商与客户的商业关系,
永久授权与年度维护订阅两项不存在。规格第 1 章逐字
「本产品是一套面向**自有企业使用**、同时可向其他企业销售的私有化企业运营平台」——
两面都在规格内,本部署取前一面。

**第二段,规格自己已经把第 3.5 章的第三条行为抽空了。** 第 3.5 章逐字要求
「依赖法规口径的功能在界面和相关单据上显著标注所用版本及其**法规基准日期**,
并提示可能存在申报偏差风险」,而**同一章**逐字
「首版**不提供带生效日期的独立法规规则包**、按领域订阅的规则包与追溯重算」。
要标注的那个对象在首版没有可取的值——这是「取不到的取值」的规格层版本,
而且取消它的不是阶段计划,是同一章自己。

**第三段,「不再接收新版本与补丁」在首版没有可挂的闸门。** 第 3.5 章逐字
「平台不再接收新的功能版本与安全补丁,已交付功能所依据的法规口径冻结在当前已安装版本」。
而首版升级形态是同机停机切换加一份安装包,安装动作在任何平台业务进程之外;
规格另逐字「厂商不具备远程强制停机能力」。
**拒收这个动作既无平台内落点,也无厂商侧远程落点。**

第二、三段不依赖「阶段计划没建」——那是循环论证,计划的沉默正是本条要修的缺陷。

#### 结论二　**不建 `grant_type` 列**,按裁定 F-15 结论七的判别式

判别式逐字:产生条件与接收方**两头都缺**则撤取值;接收方齐、生产者缺则补生产者。

`PERPETUAL` **两头都缺**:

| 该有的 | 现状 |
|---|---|
| 凭证载体 | **无**。第 3.5 章通篇没有第 3.4 章那句「订阅许可证是一份签名文件,包含签发日期、到期日期、模块范围、法人范围和用量上限」的对应句 |
| 状态机的一条出边 | **无**。第 3.5 章没有状态集合、没有迁移条件、没有判定基准时间;且「已交付功能保持可用」与「受限运行」语义相反,不是同一枚举加第五值 |
| 后果承接方 | **无**。拒收无落点（结论一第三段）、标注无表无界面（结论五） |
| 改规格 | **未改**。第 3.2 章原文未动 |

**本裁定原先的候选是「CHECK 建齐两值、首版收为一值」,那违反本卷刚立的规矩,已改判。**
按判别式该撤不该留。

落码形式另有本仓先例:`crates/adapter/kms/src/material.rs` 逐字
「首版只放行 `LEGAL_ENTITY`,`GROUP_SHARED` 预留不放行（`ck_key_domains_kind`）,
**故本枚举不设第二变体**」,而 `enum DomainKind` 里确实只有一个变体。
**预留值在本仓的落法是不建变体,不是建了再收窄。**

另注:`platform_core.license_grants` 的建表迁移尚未落地（`db/` 下零命中），
所以「加一列」这个说法本身失真——它实际是「在一份还没写的建表 DDL 里预先写一个恒取同值的列」,
拿不到「既存列收窄」那层先例掩护。

#### 结论三　请求规格修订,第 3.5 章**全节保留不删**

- 第 3.2 章「同时支持永久授权与订阅授权」请求改为首版只支持订阅授权,并登记延期。
- 第 3.5 章**不删全节**,只在节首加一句首版不适用的定语。
  理由:该章那条界面与单据标注是**对客户的披露义务**,删章节等于删一条义务;
  且第 3.2、18、21、22 四处回指它。
- 登记落点取第 20 章排除清单与 PRD 的对应表,**不取第 5.7 章**:
  第 5.7 章现有条目的形状是「能力 / 模块 / 拓扑形态」的延期
  （其分组口径逐字为「单服务器形态」「20 并发规模」一类）,
  **一种商业授权形态不是那个粒度**。

#### 结论四　`classify_subscription` 的名字保留,但要改一句声明

名字保留。适用范围声明由「只吃订阅授权凭证」改为
「本部署唯一存在的授权形态」。

**同批明写一件事:已落码的许可四态机在本部署是一套不会被行使的机制。**
内部使用不存在「让自己公司的许可到期、然后进受限运行」这种情形。
它为规格第 1 章「同时可向其他企业销售」那一面保留,
**不是本部署的实效机制**——免得日后有人以为它在这里有效力,
据以推出任何运行期结论。

#### 结论五　法规标注那条义务**单独立,不随本裁定延期**（新登记辛-7a）

第 21.14 章逐字「**未持有有效维护订阅的客户**按第 3.5 章在界面与单据上
显著标注所用规则版本、生效日期与申报偏差风险」——
触发条件挂在维护订阅上,而维护订阅在本部署不存在。

**但它防的风险一点没变**:财税法规改了而系统没升级,
开票与申报的口径就是旧的,内部用谁也一样要知道。
商业那层（永久 vs 订阅）可以延期,这条实质义务不可以跟着走。

而且它另有一处独立缺口:**「法规基准日期」在十七份阶段计划加 PRD 里零覆盖**——
没有表存它、没有界面渲染它、没有阶段建它。这一处**独立于本裁定**,
即便首版就卖永久授权也照样缺。

故单独登记为**辛-7a**,交产品负责人定首版要不要做,不并入本条主结论。

#### 一处编号说明

本卷的 `F-` 是**文档内局部序号**:每一份阶段文件各有自己的 F-01 起序列
（阶段 10 到 F-21、阶段 07 到 F-14、阶段 01 与 03 与 13 各到 F-08，等等），
本文件的裁定序列是同一惯例的又一个实例。
**跨文档引用一律写「裁定 F-xx」**,不写裸号——裸号在不同文件里指不同的东西。

### F-18　附录辛第 8 条三件的处置：商业许可面首版不建，时钟可信性拆出辛-15

前提是裁定 F-17 结论四逐字：「已落码的许可四态机在本部署是一套**不会被行使的机制**……
它为规格第 1 章「同时可向其他企业销售」那一面保留,**不是本部署的实效机制**」。

#### 结论一　三件不并作一件

其二（临期与宽限期告警无触发源）与其三（凭证签名从生到死不被校验）**整件**落在
商业许可面,与 F-17 同向,首版不交付。

其一（可信时间的日落盘）**劈成两半**:许可判定基准那一半随商业面撤;
「**本机时刻是否可信**」那一半是独立缺口,与商业关系无关,拆出**辛-15**单独登记。

#### 结论二　一处定性要纠正:可信时间不是「不可构造」,是「恒真」

本裁定的登记原文（辛-8 其一）说它「整个不可构造」——**那把定性降了一级**。

三项取最晚值而其中两项无载体（表 26 上无「签发日期」列、无落盘时间戳列、
无「续期或撤销文件签发时间」列），该式退化为 `max(valid_from) = valid_from`,
一个**常量下界**:拦得住把时钟调到许可证生效日之前,
**拦不住调到生效日之后、到期日之前**。辛-8 自己的原文其实说对了
（逐字「拦不住的恰是「把系统时间调到签发日之后、到期日之前」这一手」），
是复述时降的级。

「不可构造」是第二类缺陷（取不到的取值），
「退化为恒可满足的常量下界」是**第一类（恒真的判据）**,后者更坏。
本裁定按后者定性,并因整表不建而一并消灭。

#### 结论三　分界线是「**要不要读一张本部署不建的表**」,不是「模块 vs 商业」

**活的**:`platform_core.module_registrations` 一张表、`ModuleLicenseQuery::module_state`
一个方法、`crates/platform/license/src/module.rs` 全模块。
它有三个**具名**消费方:job-worker 的两处过滤点（计划逐字
「`module_state` 不为 `InstalledEnabled` 时跳过该条,条目保持 `PENDING` 且不累加 `attempts`」）、
阶段 5 的探针判定、阶段 13 的集成测试 26。内部使用同样要停用与再启用模块。

**首版不建**:`platform_core.license_grants` 与 `platform_core.feature_flags` 两张表。

**`feature_flags` 站不住的实测依据**:`is_feature_enabled` 全仓七处命中,
**全部是** trait 声明（三处）、测试夹具、签名锁、以及本 crate 自陈的未覆盖段——
**零个具名调用方**。对比 `module_state` 有三个。
本裁定的原候选把 `feature_flags` 与 `module_registrations` 并列称「活的」,
**那只对一半**,按实测改判。

#### 结论四　零行返回什么是一道**被错问的题**

`license_status()` 不返回 `Result`,零行只能返回某个变体——于是有了
「取 `Valid` 还是取 `Revoked`」的选择题。两个答案都错:

- `Valid` 是「没有许可被判成有许可」,一个**不会当场报错**的错;
- `Revoked` 在本部署**没有任何消费方**,是一个**取不到的返回值**——
  而本卷正在清「取不到的取值」。

**两条纪律在这里冲突,解法是撤掉这道题:**
从 `ModuleLicenseQuery` 上**撤下 `license_status` 与 `is_feature_enabled`**,
trait 收为单方法 `module_state`。

这是对裁定 A-05 冻结签名的一次**改判**,理由:两个被撤方法都必须读一张本部署不建的表,
留着它们只有两条路——留一个会 panic 的占位（本卷明禁），
或者返回一个没有闸门可穿的值（上述冲突）。

**本裁定据此新立一条可复用判别式:
「fail-closed 的前提是存在一道会被真实请求穿过的闸门;无闸门时唯一合规处置是撤,不是关。」**

#### 结论五　纯函数全部保留,读表方法全部撤——同一条纪律的两面

判别式:一个已落码的判定物,**若它只吃入参、不需要本部署不存在的载体,保留**
（为可销售那一面保留契约与用例）;**若它必须读一张本部署不建的表,撤**。

据此保留:`classify_subscription`、`in_restricted_run`、`restricted_run_verdict`、
以及 `named_user_usage_verdict`。

**最后一个是本裁定对自身取证结论的一处反向纠正**:取证建议把 `usage.rs` 整模块撤,
但 `named_user_usage_verdict` 是一个只吃两个整数的纯函数,
按本节判别式该保留——与 `classify_subscription` 只吃 `LicenseFacts` 同形。
**判别式要一致地用,不能对一个纯函数破例。** 撤的只是读表方法。

#### 结论六　「复用制品签名链验许可证」这条路正面否掉

算法层面复用无障碍（全卷四处签名一律 ECDSA P-256）。但两处受信主体清单
（`EP__RELEASE__TRUSTED_SIGNER_SUBJECTS`、`EP__PLUGIN__TRUSTED_SIGNER_SUBJECTS`）
都不含许可,卷内从未为许可证写过对应的一句。

**而实质障碍更硬**:F-17 已裁本部署「不存在厂商与客户的商业关系」,
于是签发许可证的一方与构建这台机器制品的一方**是同一方**——
复用制品密钥验许可证是**自签自验**,
等于把一个从不校验的必填列换成一个**恒真的校验**,同类而不更好。
`signature bytea not null` 随表撤。

#### 结论七　notify 侧撤取值,不留「禁止依赖」

`notice_type` 的取值域由 CHECK 承载（计划逐字「ck 取值为 PRD 第 10.5.2 节的
十类提醒事项码」）。按 F-17 结论二自陈的规矩逐字
「**预留值在本仓的落法是不建变体,不是建了再收窄**」,
**撤下「许可临期与宽限期告警」一类,十类改九类**;
计划第 3.4.5 节的扇出规模标定改挂已冻结的规模基线,不再以这一类为支点。

附带一个好处:该类的接收人解析（计划逐字「许可宽限期取全体在职用户」）
是**全卷唯一「接收人是全体在职用户」的分支**。撤掉之后,
没有任何一类提醒的接收人是一个未定上界的集合,扇出标定反而更稳。

#### 结论八　规格不删句

规格第 3.2 章「许可证使用数字签名离线验证,不依赖持续联网」与第 3.4 章可信时间全段
**原文保留**,照 F-17 结论三的形式请求加一句本部署适用范围定语。
**规格是要求方,不是承接方**;本裁定不删要求,只登记本部署不交付。

### F-19　认证机与交付机介质不同构时的采信口径（补裁定 F-16）

使用方给定的两项事实,本裁定不复议,只把后果算清:

- **认证按 20 并发跑**（规格第 3 章冻结值，A.4 的负载模型逐字「按 15 名内部并发用户
  加 5 名供应商门户并发用户施加」）
- **认证机是 SSD,交付机是单轴机械盘**

#### 结论一　20 并发是唯一一处认证严于交付的维度,这是有利方向

实际部署的并发上限是 10,低于认证的 20。规格逐字「**时延通过线不因并发下降而放松**」,
所以按 20 认证不是放宽而是**收紧**:20 并发下过线,10 并发下必然更宽裕。
本裁定据此采信——**但仅限并发这一个维度**,见结论二。

#### 结论二　认证结论按维度拆开采信,不整体沿用

A.4 的四类必判项**全部受介质影响**:第 16 章的时延通过线、
每日全量备份与附件正文每日全量写出、每日内部对账跑完 2 个法人 36 个期间、
三项写出周期均不超过 15 分钟。在 SSD 上测出来的这四类,
**对一台单轴机械盘的交付机没有指示意义**——差的不是一个系数,是一个数量级
（fsync 约 0.1–0.5 毫秒对 5–15 毫秒），而且备份顺序读与 WAL 顺序写在单轴上抢同一磁头,
在 SSD 上根本不构成争用。

**可采信的维度**（与介质无关，SSD 机上认证的结论对本部署成立）：
功能类必测项、A.3 数据集能否装下、每日内部对账能否跑完全部 36 个期间、
备份能否完成、第 17.3 章强制不变量、权限与隔离矩阵、并发维度（见结论一）。

**不可采信的维度**（由介质决定，SSD 上的结论不适用于本部署）：
第 16 章三条时延通过线、三项写出周期（即第 13.3 章 RPO 不超过 15 分钟）、
全量备份与业务并发时的退化幅度。

#### 结论三　不可采信的三项在交付机上实测,且**必须明写它不是认证**

A.3 逐字「任何情况下不下调本节数据集」，而认证数据集的 800 GB 附件正文
在交付机的 900 GB 盘上放不下（F-16 已算）。所以交付机上的实测**只能按真实数据量做**——
它给得出这台机器的实际结论，**但不构成 A.4 的认证结论**。

这条形态在本卷有现成机制,不用新造:F-08 的庚五已把
「**等目标平台上的一次实测**」立为合法的第三种等待条件。
交付机实测按庚五的口径出具:写明测什么、在哪台机器上测、
两个可能结论各自决定哪一处文字。

#### 结论四　**必须补 A.4 的介质维度,否则本裁定自己会被架空**

规格逐字「测试服务器的 CPU、内存与**磁盘规格**记入认证报告,
作为交付客户的服务器规格下限」,而磁盘规格**没有介质维度**——
全文 `SSD`、`固态`、`机械硬盘`、`NVMe`、`IOPS` 命中数为 **0**。
阶段 14 计划逐字「**客户服务器规格不低于认证报告所记规格时沿用该次认证结论,
不重跑附录 A.4**」。

两条合起来:**这次 SSD 认证的报告只能记下一个容量数字,
而 HDD 交付机在容量维度上「不低于」它,沿用条款自动生效,
结论二的维度拆分被整个架空,而且逐字对照不会失败。**
这是第三类缺陷（错了不会当场报错）在流程层的形态。

据此请求规格 A.4 补两个字段并同批改一句:
- 认证报告的服务器规格增记 **`disk_medium`**（取值 `HDD_SINGLE_SPINDLE` / `SSD` / `MIXED`）
  与 **`wal_colocated`**（`pg_wal` 是否与 PGDATA 同卷同轴）两项;
- 阶段 14 的沿用条款改为「客户服务器规格**在容量与介质两个维度**均不低于认证报告所记时
  沿用该次认证结论」——介质维度不同构时,**只沿用结论二列举的可采信维度**。

#### 结论五　交付说明必须明写的两句,这是对客户的披露不是内部记录

一、**第 16 章的三条时延通过线在本部署未经认证覆盖**,
其成立以交付机实测为准;认证报告中的时延数据取自 SSD 机器,不适用于本部署。

二、**第 13.3 章 RPO 不超过 15 分钟在本部署不沿用认证结论。**
阶段 14 计划逐字已说「RPO 不超过 15 分钟在本平台**不再有机制侧保证**,
其成立完全押在附录 A.4 的认证实测上」（F-08 删掉了磁盘 IO 让路那一句之后）。
而该实测的介质与本部署不同——**于是本部署的 RPO 今天没有支撑物**,
只能由交付机实测给出，在实测出结论之前不得向客户承诺 15 分钟。

这两句按第 21 章残余风险披露的口径写入交付说明,不得沉默。

#### 结论六　三项必测的性质变了:从「验证预期」升为**交付前置**

F-16 结论七的三项必测与绊线-处置对**取值不变**（提交 P95 > 3 秒、
备份窗口内提交 P95 > 3 秒、归档写出周期 > 15 分钟，任一越线即加一块 HDD 专给 `pg_wal`）。

但性质变了:在 F-16 里它们是「跑一次看看,预期都会过」;
按本裁定,**它们是本部署这三条通过线与 RPO 唯一的结论来源**。
因此从「验证项」升为**交付前置**——不跑完这三项,
交付说明里那两句披露就没有可填的数,而结论五要求它们必须有数。

#### 一处仍无解的残余,明写

**本部署永远得不到一份介质同构的 A.4 认证结论**,除非日后备齐一台
≥2 TB 的单轴机械盘机器。交付机实测按庚五口径能给出这台机器的实际数值,
但它在规模上低于 A.3 的基准数据集,规格那句「任何情况下不下调本节数据集」堵着——
**两者合不成一份完整的认证**。这一条不是本裁定能解决的,如实登记。

### F-20　裁定附录辛第 10 条：两套表达式语言的一致性要求

本裁定的候选稿五条要点**全部被对抗性核查打掉**，且核查挖出的事实比原稿重要。
下面按纠正后的事实作判，不按登记时的描述作判。

#### 结论零　先更正辛-10 登记时的三处措辞，两处夸大、一处方向反了

**其一，「同一套低代码界面」无出处。** 全仓「流程设计器」**只命中一处**，
且是错误码的用户文案：docs/error-codes.md:128 逐字
「| PLATFORM.FLOW.GUARD_EXPRESSION_INVALID | 流程的条件表达式无法求值。|
**请在流程设计器中检查该条件的写法与所引用的字段。**|」——
**没有任何阶段计划交付这个设计器。**
守卫今天的真实录入面根本不是界面：它挂在 `FLOW_DEFINITION` 内容项上经配置包发布，
13:32 逐字交付「`tools/epcfg`：配置包的打包、差异、签名、验签与离线导出导入命令行工具」。
**守卫今天是写在 Git 里的一段文本。**

**其二，「同一个管理员」半有出处，且与卷内另一取值冲突。**
PRD:3907 逐字「| 配置管理员 | 低代码五类定制的编辑、提交、发布与回退发起 |」支持这一侧；
PRD:3253 逐字「| **报表设计者** | 使用报表设计器、**定义企业自定义指标**、
配置仪表盘与打印模板 |」是另一侧。这件事卷内已记为待决：
PRD:4456 逐字把「报表设计者」与「配置管理员」的岗位名称未对齐登记为 `U-B-02`「待决」。

**其三，方向反了：规格与 PRD 从头到尾只命名过一套。**
「声明式规则表达式」在规格全文命中 **1** 次（:921）、在 PRD 全文命中 **1** 次（:4051），
**没有任何一处把流程守卫条件单列为第二种语言**。切开它的是计划——
03-platform-kernel.md:1067 逐字「该求值器**只服务于流程守卫条件，
不是 `RuleEvaluator` 的实现**」。
所以「两套」不是规格造成的既成事实，是阶段 3 计划的一次切分。

#### 结论一　**本裁定的一致性矩阵整体悬空：声明式规则今天不可被创建**

这是本轮最重的一条，候选稿一字未提。
规则今天**能被求值、能被下发、能被审计，但不能被创建、不能被存储、不能被发布**：

- **无表。** 全卷 `rules` 仅 4 次命中，**无一是表定义**；
  唯一带 rule 字样的列是 13:327 的 `rule_versions jsonb`，
  它在 `client_bootstrap_dispatches` 上、存的是**版本号不是 AST**。
- **无内容项类别。** 13:242 逐字「item_kind 取值**封闭为 15 项**：
  CUSTOM_OBJECT、CUSTOM_FIELD、CUSTOM_RELATION、CUSTOM_INDEX、CUSTOM_VIEW、
  UI_LAYOUT、FLOW_DEFINITION、AUTHZ_ROLE、AUTHZ_POLICY、AUTHZ_FIELD_GRANT、
  REPORT_DEFINITION、METRIC_DEFINITION、DASHBOARD_DEFINITION、PRINT_TEMPLATE、
  NOTIFY_RULE。」——**没有 RULE。规则连配置包通道都没进。**
- **无 applier。** 全卷十个具名 `*Applier`（`FlowDefinitionApplier`、
  `MetricDefinitionApplier`、`NotifyRuleApplier`……）**没有规则类**。
- **无录入端点。** 全卷 `lowcode.rule` 命中 **1** 次，就是 13:706 的 `evaluate`。

而下发侧却是硬要求：13:325 逐字要求落实规格第 7.4 章
「自定义对象下发到客户端时随会话一并下发……**声明式规则版本**」，
规格 :921 逐字要求「声明式规则表达式与**版本化规则**」。

**一条谁都写不进去的规则，其「值语义与守卫一致」是一条恒真的判据**——
两侧永远不会分歧，因为一侧永远没有取值。
本裁定据此**撤回候选稿的十二维一致性矩阵**：在补上属主之前，那张矩阵
本身就是本卷正在清的第一类缺陷。

处置：**先补属主，再谈一致。** 登记为附录辛第 16 条（见下），
指派阶段 13b 同批交付：一张规则表、`item_kind` 增 `RULE`、一个 applier、一个录入端点。
在此之前，任何「两套语言值语义一致」的判据**不得记为已承接**。

#### 结论二　辛-10 的当事人换人：真正贴身的一对是「指标表达式 vs 高级只读 SQL」

候选稿把 `RULE.EXPRESSION_PARSE_FAILED` 当成「规则另有文本录入面」的证据——
**错**。13:525 逐字「规则以 AST 形式存储与下发，**无任何代码下发**」，
13:706 的请求体逐字 `{rule_code, rule_version, input}` 三个字段没有一个是表达式文本。
**规则侧今天没有 parse，也没有可 parse 的对象**（见结论六）。

而普查查出了一对**比辛-10 登记的那一对贴身得多**的：
11:497 逐字「用 **sqlparser 解析为 AST**，逐项白名单校验。允许：单条 Query 语句、
SELECT、非递归 CTE、JOIN、GROUP BY、HAVING、ORDER BY、子查询、UNION ALL、
字面量与绑定参数、白名单聚合函数与日期函数。」与 11:418 的 `MetricExpression`
**写在同一个字段里**——11:497 逐字「只能出现在**报表定义版本的 spec 内**」。

两者由同一个报表设计者写、进同一个 `spec`，而：
- SQL 那侧是 **PostgreSQL 的三值逻辑**（空参与比较得空，不是假也不是错）；
- 指标那侧**零口径**：11:642-655 的 13 个配置键里**没有任何指标表达式的上限**，
  11:663-700 的测试计划里**没有任何一条指标表达式用例**——
  而高级只读 SQL 有三个上限（`MAX_QUERY_BYTES`／`MAX_JOIN_COUNT`／`MAX_SUBQUERY_DEPTH`）
  与二十条用例（11:675 逐字「允许清单 8 条各一例，拒绝清单 12 条各一例」）。

**这一对没有任何可区分性判据，而它们同处一个字段。** 登记为附录辛第 17 条。

#### 结论三　普查：首版实际是**四套加一套半加一套零文法**，不是两套

| 语言 | 状态 |
|---|---|
| 流程守卫（中缀文本） | 已落码，64 条用例 |
| 声明式规则 AST | **不可被创建**（结论一） |
| 指标表达式 | **零上限、零用例**（结论二） |
| 高级只读 SQL | 三上限二十用例，最完整的一套 |
| `custom_fields.default_expr` | **半套且直通 DDL**：13:164 逐字「只接受常量字面量，不接受易失函数」，而 13:854 的基线校验器四项**不含它**——无解析方、无用例、无错误码 |
| 通知模板四列 | **零文法**：03:547-550 的 `title_template` 等四列，全仓「占位符」「插值」零命中，无转义规则、无未解析变量处置、无解析失败码，而它渲染进移动推送 |

后两项分别登记为附录辛第 18、19 条。

#### 结论四　空语义定口径：严格报错，但**作用域只及今天有被测对象的那一侧**

口径取守卫已落码的第三条，`expr/eval.rs:3-17` 逐字：
「**二值**（空参与比较判假）在 `not` 下自相矛盾：`vars.x` 缺失时 `vars.x > 10` 为假，
`not (vars.x > 10)` 为真——即「既不大于，也不是不大于」两条里必有一条被判成真。
**这是一条永远不会报错的错。**」

**但规则侧的口径不在本裁定今天能定的范围内。** `AstRuleEvaluator` 全仓源码零命中，
`crates/platform/meta/src/rule/` 目录不存在。通则第六条第二档要求替身的
「被测输入**已存在**」，这里做不到。因此规则侧走**第一档「整条推迟」**，
写进 13:863 第 11 项那一行——那一条今天逐字点了名却没写期望值，正是要补的地方。

**不把它记成本轮已承接。** 以尚未交付的被测对象充当替身，是通则第六条明禁的第四种。

#### 结论五　今天能冻结的是**期望表**，不是判据

用例集不是代码，是一张三列表：`守卫源文本 | 等价 AST | 期望裁决`。
今天写死入库，13b 只能对不对，**不能自选答案**。

裁决域必须**下降为封闭枚举**——`True | False | Rejected(NullOperand) |
Rejected(TypeMismatch) | Rejected(LimitExceeded)`，两侧各自映射，
**不比较 `at`、不比较文案**。理由：守卫的公开求值口是 `Result<bool, GuardError>`
且每个错误变体带 `at: usize` 源文本字节偏移（`expr/mod.rs:288` 逐字
`NullOperand { at: usize, what: &'static str }`），而规则**无源文本**，
`at` 没有对应物；13:706 列出的错误码只有解析期与限额期两个，
**没有任何求值期的空操作数或类型错误码**。不降维则用例永远写不出来。

必测的两条样本今天就能定：`9007199254740993 > 9007199254740992`
（`f64` 下两数不可区分、十进制定点下不等，只用字面量与比较节点即可，不需要算术）
与 `0.1` 对 `0.10`（`value.rs:174` 逐字要求
`equality_is_by_value_not_by_representation`）。

#### 结论六　`RULE.EXPRESSION_PARSE_FAILED` 判为第二类缺陷，删除

三条理由叠加：

1. **无被解析对象**，见结论二。
2. **码形不合规。** docs/error-codes.md:9 逐字「错误码为**三段**点分大写，
   形如 `<MODULE>.<RESOURCE>.<REASON>`」——`RULE.EXPRESSION_PARSE_FAILED` 只有两段，
   同格的 `RULE.AST_LIMIT_EXCEEDED` 同样两段，而 181 行之前的 13:525 写的是**三段**的
   `PLATFORM.RULE.AST_LIMIT_EXCEEDED`。**同一份文档里同一族码两个形状。**
3. **两条都未登记。** docs/error-codes.md 中这两个码命中数为 **0**，
   而 :5 逐字「新增错误码的顺序是**先登记后实现**」。

处置：13:706 删 `RULE.EXPRESSION_PARSE_FAILED`；`PLATFORM.RULE.AST_LIMIT_EXCEEDED`
补登 docs/error-codes.md。**这条码真正有 parse 对象的落点是
11:572 的 `REPORTING.REPORT_OBJECT_VERSION.EXPRESSION_PARSE_FAILED`，那一处不动。**

#### 结论七　合成一套 = **否**，但候选稿给的理由是错的，换两条

**依赖方向不堵，候选稿说反了。** 00b:102 逐字「ep-platform-\* 只可依赖 ep-foundation
与其他 ep-platform-\*」——`meta → flow` 在允许项内；且这条边**已被裁定 B-05 要求**
（13:531 逐字 `AstRuleEvaluator`「实现阶段 3b 定义的 `ep_platform_flow::port::RuleEvaluator`」）。

**「客户端传递依赖会拖进 outbox」这条也不成立。** 13:72 那句「依赖」无
「直接／传递」限定，而同文件需要限定时会明写（13:995 逐字「**工作区内直接依赖**」）；
判定工具也判不出传递闭包——`xtask/src/graph.rs` 逐字从 `cargo metadata --no-deps` 读入。
**而且按传递读法，`clients → meta → release → outbox` 在卷内已写死的边集上早就通了**，
合不合并表达式一件新事都没干。真按传递读，该裁的是 13:72 与 13:62/13:995 自相矛盾，
不是拿它堵表达式合并。

**真正成立的两条理由：**

一、**13:57 逐字「本阶段不新增 platform crate」**，封死「抽一个 `ep-platform-expr`
叶子 crate」这条依赖形状最干净的路。撤它要单裁，本裁定不撤。

二、**被求值对象的事实基础不同，合并即造假。**
`expr/value.rs:42-43` 逐字：「空。**键不存在与键存在但取值为空合并成同一个值**——
**变量没有 schema，本 crate 无从区分二者，硬要区分就是编一个自己不掌握的事实**。」
而规则侧的字段**有** schema（13:526 逐字「数值一律 `foundation::Money`、`UnitPrice`、
`Quantity`、`Rate` 四类」）。合成一套必然要一侧放弃 schema 或另一侧伪造 schema。
**这条理由在两种依赖读法下都成立，也不依赖任何门禁。**

改判为：**一套值语义、两套表达形态、两个独立实现**——但见结论一，
「一套值语义」今天还没有第二个当事人。

#### 结论八　三处「复用低代码 AST」的措辞降为待改，理由不是「架构不可能」

候选稿写「架构上不可能」，**引证指错了行**（00b:104 是契约层那条，
要引的在 00b:105），而且不成立：00b:106 逐字「ep-app-\<m\> 可依赖 ep-foundation、
**ep-platform-\***……」——把 `MetricExpression` 挪到 `ep-app-reporting` 这条边就合法，
且 11:112 逐字「ep-app-reporting 另依赖 ep-platform-release」，该形状阶段 11 已有。

成立的理由是另外两条：

一、**次序反的。** 00c:1510 的关键路径逐字「…→ 10 → **11** → 9b → **13** → 14」，
阶段 11 要复用的东西那时**还不存在**。

二、**能力集两个方向都对不上。** 11:418 逐字要「字段引用、四则运算、聚合函数与
**条件表达式**，**不允许函数调用**与子查询」；而 13:419-426 的 `RuleExpr` 十一个节点里
**没有任何条件／分支节点**（无 `If`、无 `Case`），它唯一的调用节点 `WasmCall`
**恰是 11:418 明令不允许的那一类**。

处置：三处「复用」改为措辞待定，文法属主归阶段 11，并同批要求 11 补
配置项上限与用例（见结论二：今天是零上限零用例）。

#### 结论九　E6 不得被计作「两套语言值语义一致」的承接方

13:930 的 E6 四者逐字是「Rust 规则解释结果、字段级权限裁剪、审计结果与
恢复连接后的中心重校验」——**后三者与规则模块复用无关**。
且它的第一分句在 `executable_on_client` 为真的真子集上是**同一份源码比自己**
（13:527 逐字「`executable_on_client` 的取值为 `!requires_wasm`」，
13:528 逐字「中心执行**全部规则**」）。

处置两句：**E6 对下发与部署有效，措辞一字不改；但不得计作值语义一致的承接方**——
那件事的承接方只有结论五的期望表。

**顺带记一条与辛-15 同源的**：`RuleExpr` 有 `Today` 与 `PeriodOf` 两个节点（13:425），
而 `Clock` 端口按辛-15 在代码里不存在；E6 的场景恰是断网草稿到恢复后中心重校验，
两次求值发生在不同机器不同时刻。**辛-15 的时钟缺口在这里有第二个落点。**

#### 残余：本裁定落地后仍无人承接的

- **A（最大）** 结论一那四项缺口在 13b 补齐之前，一致性矩阵整体不成立
- **B** 流程设计器**无交付阶段**——守卫写错时给用户的建议指向一个不存在的东西
- **C** 「指标表达式 vs 高级只读 SQL」同处一个 `spec` 字段而无可区分性判据
- **D** `default_expr` 直通 DDL 而无解析方
- **E** 通知模板四列零文法，且渲染进移动推送
- **F** 提醒规则**没有落点表**——03:1173 的 `NotifyRuleApplier` 明写要往那里落地，
  而全卷 `notify_rules`／`reminder_rules` 零命中
- **G** 本裁定降为评审判据的条目**登记落点未定**：00b:783 的第三档逐字要求
  「降为评审判据并登记」，00b:789 逐字「本表是上一条通则的**机械承接方**」，
  而本裁定没写它们登记进哪张表。**这一条须在下一轮补。**

#### 新登记的附录辛条目

| 编号 | 缺口 | 类 |
|---|---|---|
| 辛-16 | 声明式规则无表、无内容项类别、无 applier、无录入端点，却被要求版本化下发 | 排期项应撤（F-21 判乙已处置；F-28 结论二） |
| 辛-17 | 指标表达式与高级只读 SQL 同处 `report_object_version.spec` 而无可区分性判据 | **证伪**（F-28 结论二） |
| 辛-18 | `custom_fields.default_expr` 直通 DDL，无解析方、无用例、无错误码 | 待核（F-28 结论二） |
| 辛-19 | 通知模板四列零文法，渲染进移动推送 | **夸大**（F-28 结论二） |
| 辛-20 | 降为评审判据的条目无登记落点（通则第六条第三档的承接方未定） | **证伪**，撤号（F-24 结论七、F-28 结论二） |

### F-21　裁定附录辛第 16 条：声明式规则的属主

**主文：判乙——规则自立第 16 类 `item_kind = RULE`，按六件配齐；甲不成立，丙不成立。**

候选稿五条要点中四条被对抗性核查打掉，本裁定按纠正后的事实作判。
核查本身也有两处要纠正，见结论三。

#### 结论零　枢纽先结：甲不成立，但候选稿的「最强证据」选错了

甲＝「规则挂在自定义对象上随 `CUSTOM_OBJECT` 内容项走，`item_kind` 没有 RULE 是设计不是遗漏」。

候选稿把 13:729-733 的引导响应体立为最强证据——**那整段落在 ```` ```json ```` 围栏内
（13:725 起、13:737 闭），是示例块，而示例块的约束力本卷从未裁过。**
拿它当最强证据是自找翻案。换成三条**规范性正文**：

一、**`CustomObject` 的成员逐字列全，没有 rules。**
13:413 逐字「// ep-platform-meta::model」，13:414-416 逐字
「pub struct CustomObject { id: Id, code: ObjectCode, security_level: SecurityLevel,
is_document: bool, doc_type_code: Option<DocTypeCode>, fields: Vec<CustomField>,
indexes: Vec<CustomIndex>, definition_version: u64, status: ObjectStatus }」。
而 13:418 逐字「// ep-platform-meta::rule」把 `RuleExpr` 放进**另一个模块命名空间**。
同一页、同一份类型清单、两个模块。**甲要成立，规则必须是 `CustomObject` 的一个成员——
原文写死了它不是。**

二、**`item_kind` 与「定义对象表」在本卷一一配对。** 13:242 尾句逐字
「其中 FLOW_、AUTHZ_、REPORT_、METRIC_、DASHBOARD_、PRINT_、NOTIFY_ 七类的**定义对象表**
由流程、权限、报表、通知各自阶段拥有，本表只保存其序列化快照与哈希」。
甲主张规则无自己的类别，就等于主张它无自己的定义对象表；而 13:418 已给它单独的模块。

三、**规格自己把两者并列，不是嵌套。** 第 5.1 章 :275 逐字
「元数据、自定义对象、字段、关系、索引和视图」与 :276 逐字
「低代码表单、流程、**规则**、审批、定时器、补偿和 SLA」——
**规则与流程同条，与自定义对象不同条。** §9.1 的 :919/:920/:921 同样是三条并列。

甲的唯一依据是规格 :711「随会话一并下发对象结构、字段密级、权限策略和声明式规则版本」。
**那是一张混合下发清单，不是归属声明，而且它自己就能证伪甲**：
并列四项里的「权限策略」有明确的外来属主——13:551 逐字
「AUTHZ_ROLE 的 `AuthzRoleApplier`……由阶段 4 在 `ep-platform-authz` 实现」。
**同一张清单里已有一项被证明不属于自定义对象，「出现在这张清单里」就推不出「属于自定义对象」。**

**并且甲即便成立，缺口不缩小、只改名**：仍要补表、类别、applier、录入面四件，
只是标签换成 `CUSTOM_VALIDATION`，同时与规格 :275/:276、:919/:921 三处冲突。

#### 结论一　丙不成立，但候选稿的理由是错的，而且错得危险

候选稿写「`03:782`／`03:1173` 的 `APPLIER_NOT_REGISTERED` 是一道会被真实请求穿过的闸门」。
**两处都错：**

一、**引证错位。** `APPLIER_NOT_REGISTERED` 全仓仅命中 **1** 次，即 03:1173。
03:782 不含该码，它写的是机制句「未在 `ConfigItemApplierRegistry` 注册实现的 `item_kind`
由运行期校验拒绝发布，不靠 CHECK 拦截」。把机制句当成该码的第二处登记，是同一类引证错。

二、**实质前提为假：该闸门对规则类型上不可达。**
`crates/platform/release/src/port/config_item.rs:185` 逐字
`pub fn lookup(&self, kind: ItemKind) -> Option<&dyn ConfigItemApplier> {`——
**入参是枚举不是字符串**，`ItemKind` 十五变体里没有 `Rule`，
一个「规则内容项」连 `ConfigPackageItem` 都构造不出来，
根本走不到那道闸门前。

**这是循环论证：拿「闸门是空的」既论证撤、又论证补。** 该论证作废。

丙不成立的**正确**理由是判别式一（见结论二）：
规格 §5.7 延期目录与 §20 排除项**均不含**声明式规则；
PRD 首版不含清单里与规则相关的只有「由本地 AI 生成流程或规则草稿」，
而 PRD:4079 与规格 :926 两处逐字「草稿只能由人工创建」——
**人工创建规则草稿被明写在首版范围内。丙没有撤的资格。**

#### 结论二　新立两条判别式，其一与 F-18 合并

> **判别式一（撤 vs 补，与 F-18 合并成一条两分支的）：**
> 问同一个问题——**这道闸门今天会不会被真实请求穿过**。答否之后分两支：
> **能力已在、闸门缺位**（F-18 的形态）→ **撤**；
> **闸门已在、能力缺位**（本条的形态）→ **补**。
> 分辨两支靠**上位权威有没有点名该能力**，不靠闸门本身。
> **任何以「闸门是空的」同时论证撤与补的引用，一律无效。**
>
> 撤的资格不由阶段计划产生：撤的前提是上位权威点了它的名
> （规格第 5.7 章延期目录、第 20 章明确排除项、PRD 首版不含清单）。
> 上位权威把它写成首版强制项、只是阶段计划漏了承载物的，
> 唯一合规处置是补承载物；**计划不得以「我这里没有表」反向撤销上位权威的强制项。**

> **判别式二（自立一类 vs 挂靠既有类）：**
> 被挂靠物若**有独立的 code 与 version 轴**、且在下发台账列里与宿主**平级排开**，
> 则不得挂靠，必须自立一类。

套上去：13:706 的求值端点按 `{rule_code, rule_version}` 寻址且**不带对象码**；
13:327 的 `client_bootstrap_dispatches` 三个同形版本列里
`custom_object_codes` 对 `CUSTOM_OBJECT`、`ui_layout_versions` 对 `UI_LAYOUT` 各有 item_kind，
**只有 `rule_versions` 没有——这是遗漏的形状，不是设计的形状。**
规则不得挂靠 `CUSTOM_OBJECT`，也不得挂靠 `FLOW_DEFINITION`；**自立第 16 类。**

#### 结论三　判乙。但核查对拦路石读重了，本裁定按实测更正

核查主张增第 16 类要「**停用一条已生效裁定连带条款里的明文禁令**」，
指 00-overview.md:65 的「阶段 4 与第 4.1 节 A-19 行按十五项的写法不动」。

**实测该段全文是一次「撤销删除」，不是一条增项禁令。** :65 开头逐字：
「第一条，自定义对象、自定义字段与在线 DDL 三项及 ItemKind 的项数。**撤销该删除。**」
整段在把某次删除撤回，理由逐字「阶段计划是第五权威，**不得使规格与 PRD 的强制项失去承载**」；
「阶段 4 与第 4.1 节 A-19 行按十五项的写法不动」是**那次恢复的连带影响**——
意思是这次删除不要蔓延到阶段 4 与 A-19，**不是对将来增项的禁令**。

而 13:1076 逐字「`ConfigItemApplier` 端口由阶段 3a 按裁定 A-19 交付，
**其 `item_kind` 取值集合可扩展**」是明写允许。

**结论：增第 16 类不需要前置裁定。** 但**确实要回改 A-19 行的三个计数**——
00-overview.md:203 逐字「ConfigItemApplier 的**九个** item_kind 实现」
「**六个**自定义类归阶段 13b」，增项后为十个与七个，`ItemKind` 十五项为十六项。
那是一条已生效裁定行，**须明写回改，不得顺手带过**。

**同时撤掉核查引的一句原文：** 核查以 13:743「客户端必须容忍……`item_kind` 出现未知取值」
支撑「十五是恢复值不是冻结上限」。**实测这句不存在**——13:743 讲的是端点主版本
（「本阶段全部端点为 v1……新增字段属于向后兼容变更，不升主版本」）。
本裁定不用不存在的原文作支撑；13:1076 那句已经够了。

#### 结论四　13:1076 的末句是假的，同批判假

13:1076 末句逐字「发布链路、差异算法、签名、审批与回退全部复用，**不改本阶段任何表**」。
新增一类**必须**改 `ck_config_package_items_item_kind`（13:244 列出该约束），即必须改表。
此矛盾在 `2026-08-17-f10-ruling-detail.md:171` 已被独立查出一次。

改法：末句改为「不新建本阶段任何表，但须放宽 `ck_config_package_items_item_kind`，
并同批更新阶段 3a 的 `ItemKind::ALL`」。
03:1643 的同形句主语是「阶段 4、11 与 13b 只需实现自己的 applier」，指已有类别，那句成立，不改。

#### 结论五　六件配齐，逐件给静默风险

| 件 | 内容 | 今天有无门禁 | 处置 |
|---|---|---|---|
| 1 | `platform_meta.rules` 表 | **有真闸门**：`db/checks/13_unpoliced_registry.sql:4` 是**全库扫描式**，新表漏登记 `unpoliced_table_registry` 当场红；另有 01-12 共十条同为全库扫描 | 建表即可，**不需新建门禁**。候选稿写「数据库侧全无门禁」是错的 |
| 2 | `ItemKind` 增 `Rule` 变体 | **构建红**：`config_item.rs:45` 逐字 `pub const ALL: [ItemKind; 15] = [` 是定长数组；**测试红**：`:198` 逐字 `assert_eq!(ItemKind::ALL.len(), 15);` | 两条都是真闸门，但见下一行 |
| 3 | `RuleApplier` | **加了照绿**——见结论六 | 必须同批改断言方向 |
| 4 | 录入面 | 走 13:680 的通用配置包录入 `POST /api/v1/platform/config-packages` | **不新增专用写端点**，见结论七 |
| 5 | `RULE_SEMANTICS` 自动测试 suite | 历史八套缺少规则类会让规则包在零条规则语义被验证时通过 | **已由 F-52 闭合**：`RULE_SEMANTICS` 是最终第九套，执行落点、适用映射与 `SKIPPED` 语义均已冻结；D-02 不再是前置待决 |
| 6 | 一条计数机检 | **承接面现成且同形**：`xtask/src/archcheck/frozen.rs:19` 逐字 `const EXPECTED: [(&str, &str, usize); 8] = [`，里面已有 `:29`「("module.rs::ModuleCode", "模块码", 15)」与 `:30`「("capability.rs::CapabilityDomain", "能力域码", 18)」两条同形判据 | 加第 9 行。**但真实成本不是「加一条断言」**：`:35` 逐字 `let base = root.join("crates/foundation/src");`——该模块今天**只解析 foundation 一个 crate**，加 `ItemKind` 要放宽到跨 crate 解析 |

**件 6 的错误码回改一并纠正：不得复活 `RULE.EXPRESSION_PARSE_FAILED`。**
F-20 结论六已判它为第二类缺陷删除，且它是两段码、撞 docs/error-codes.md:9 的三段式规定。
只补 `PLATFORM.RULE.AST_LIMIT_EXCEEDED` 一条。
错误码是六件里唯一既不静默也不需要新建门禁的——`xtask/src/errorcodes.rs` 已承接。

#### 结论六　件 3 会静默通过，两处负向断言必须同批改成正向

`apps/job-worker/src/wiring/release.rs:35` 与 `apps/core-server/src/wiring/release.rs:49`
**都是负向断言** `registry.lookup(kind).is_none()`。

后果分两步，第二步更坏：
- 加第 16 个变体 `Rule` 时它未注册，`is_none()` 为真，**测试照绿**；
- 等 13b 真注册了 `RuleApplier`，core-server 那条才变红，
  而**最省事的改法是把 `"RULE"` 加进 `registered` 名单**——
  **这条测试就此从闸门退化成名单抄写。**

处置：同批把两条改成**正向断言**「本装配位注册的集合**恰等于**该阶段应交付的
`item_kind` 集合」。否则件 3 落地零门禁。

#### 结论七　件 4 判不补专用端点，但两方的先例都不能用

对抗核查在这一件上 1:1 分裂，两方各自援引的先例经核对**都不成立**：

- 反方拿 `CUSTOM_RELATION` 与 `CUSTOM_VIEW`（同 schema、同 crate、同阶段、零写端点）
  当「不需要专用端点」的先例——**不成立**：13:642 逐字「对象详情含字段、关系、索引、视图」，
  它们的录入面**挂在宿主对象上**，而规则**没有宿主**——
  这正是判别式二判它自立一类的同一组理由。
- 正方拿「五类定制里四类有专用端点、只有流程没有」——**也不成立**：
  那个缺席本卷从未裁过，**拿未覆盖当认可正是本卷禁的**。

判：走通用配置包录入，理由是该通道对全部 `item_kind` 一视同仁，
新增一类本就不需要新端点——这是 `ConfigItemApplier` 端口的设计意图本身。

**顺带纠正候选稿一处假话**：它写「规则是唯一一个只有『用』没有『管』的低代码类别」——
**假**。`lowcode.custom_relation.*` 与 `lowcode.custom_view.*` 全卷**零命中**，
`custom-relations`／`custom-views` 端点在全 docs/ 零命中。**它们比规则更彻底。**

#### 结论八　普查查出两处比辛-16 大的全域缺口，各自立案

**（甲）权限项没有生产者。**
消费方：13:634 逐字「下表的权限要求列写权限项名，具体角色映射由权限阶段承担」，
阶段 13 五张 API 表 30 个权限项串，全 15 份计划去重 229 个。
生产方：`platform_authz.permission_items` 的**唯一**种子迁移
`V20261012112000__platform_authz_backfill_permission_item_seed.sql` 注释逐字
「**共 9 行，module_code 一律 platform**」，且该迁移自己承认
「业务对象登记与其权限项细化**归其所属阶段**」——而所属阶段一份都没写。
**`permission_item` 这个串在 04 以外的十四份阶段计划零命中。** 登记为**辛-21**。

**（乙）`object_scope_bindings` 没有生产者，而消费口径是 fail-closed 全拒。**
04:139 逐字「各业务模块在其阶段的 wiring 中登记自己对象的范围锚列，
本阶段只登记 platform 自身的三个对象类型……**没有登记的对象类型在记录级判定阶段
一律拒绝，不默认放行**」。
而**该表在 15 份阶段计划中只出现在 04 一个文件里**（`grep -rl` 全卷唯一命中），
阶段 5–14 无一登记自己的对象。
**后果与辛-16 同形而覆盖面是全部业务模块的全部对象：所有列表与详情端点的
记录级判定恒为拒绝。这是「取不到的取值」在本卷最大的一处。** 登记为**辛-22**。

#### 残余

- **A** 已闭合：F-52 冻结 D-02，件 5 落入九套自动测试的 `RULE_SEMANTICS`
- **B** 辛-21 未结之前，任何新增权限项串（含件 4 若要加 `lowcode.rule.view`）
  都是往一个没有生产者的池子里再扔一个字符串，**六道门禁全绿**——本裁定据此**不加**该权限项
- **C** `FieldPath`、`AggKind`、`RuleValue`、`CmpOp` 四个类型在全 docs/ 只有 13:419/420/424
  三行本身，**零处定义**；件 1 建表要定死 `rule_code` 唯一性作用域时会撞上它
- **D** 三个例外端口（`RuleEvaluator`、`WasmComputePort`、`DisposalPort`）的 trait 今天
  在代码里都不存在，而 00-overview.md:77 要求「能力缺位时一律开」的三条降级窗口
  其 subject 要取的正是这三个类型名——**判据在、承接物不在**。登记为**辛-23**
- **E** `Guard::parse` 在发布期无承接方（`expr/mod.rs:31` 自陈），
  与件 5 是同一个洞的流程侧，`FLOW_SEMANTICS` suite 同样悬空，须与件 5 同批
- **F** 00b:770 逐字「新增一张表……之前，先在**本基线对应章节**登记」，
  而 00b 全文**没有表清单章节**——件 1 按现文无处可登

#### 新登记的附录辛条目

| 编号 | 缺口 | 类 |
|---|---|---|
| 辛-21 | 全卷 229 个权限项串无生产者，种子迁移只有 9 行且自陈「归其所属阶段」而无阶段承接 | **已撤销**（F-27 结论零） |
| 辛-22 | `object_scope_bindings` 只在阶段 4 出现，阶段 5–14 无一登记，而口径是「未登记一律拒绝」 | 已由 F-22 处置；成立但需收窄（F-28） |
| 辛-23 | 三个例外端口的 trait 不存在，而三条 `PORT_NOT_IMPLEMENTED` 降级窗口的 subject 要取其类型名 | 排期项应撤（F-24 结论七、F-28 结论二） |

### F-22　裁定附录辛第 22 条：`object_scope_bindings` 无生产方

> **历史证据边界：** 本节逐字引述与“无外键”等实测，描述的是 29 个校验和冻结的 **EXISTING** 迁移及当时源码，不是空库最终目标形状。直接开发须同时执行 `docs/migration-catalog.md` 的 **PLANNED** 追补迁移，并以阶段 4 现行表定义、真实外键总则和 `docs/data-dictionary.md` 为准；不得从本节历史证据反推出保留缺失约束。

候选稿五条要点**全部被对抗性核查打掉**。更要紧的是，**辛-22 登记稿本身有三处夸大、
一处方向说反了**。本裁定先更正登记，再作判。

#### 结论零　登记稿四处更正，其中一处是方向反了

登记稿逐字「后果与辛-16 同形而覆盖面是**全部业务模块的全部对象：所有列表与详情端点的
记录级判定恒为拒绝**。这是「取不到的取值」在本卷最大的一处。」——四处都要改：

**一、「所有列表与详情端点」今天一个都没有。** 16 个业务 schema 的迁移目录**各只有 1 个
文件**，业务表一张未建。这句在将来为真、在今天为假。

**二、方向反了，而且实情更严重。** 今天在线的 `/api/v1/platform/` 路由共 **33 条**，
经 `apps/core-server/src/main.rs` 的 `platform::middleware::authenticate` 层之后
**没有任何授权层**：`AccessDecider` 全仓命中 **5** 处、**全部在 `crates/platform/authz/`
crate 内部**（`lib.rs:25` 的 `pub use` 与 `decider.rs` 的定义、impl、两处测试夹具），
**零个生产调用点**；反向依赖复核同向——依赖 `ep-platform-authz` 的三个 crate 取的是
`AdmissionGate`、`sod::check_duty_exclusion`、`types::hex_*`、`applier::*`，**无一处取
`AccessDecider`**。
**所以今天的形态不是「未登记 → 一律拒绝」，是「根本不判」。** 登记稿把一个 fail-open 的
现状写成了 fail-closed 的现状。

**三、判定层级说错了。** 登记稿说是「记录级判定」。实现把它放在**阶段一之前的前置硬条件**：
`crates/platform/authz/src/decider.rs:76-83` 逐字
「// 前置硬条件：无角色直接拒；未登记对象类型拒范围绑定缺失。」
该检查只吃 `object_type` 一个入参，早于取法人分片、早于任何仓储取行。
这处夸大同时误导了半径：它让人以为拒绝逐条记录发生，实际是**整个对象类型一次性挡掉**。

**四、「本卷最大的一处」量级说反了。** 同一个 crate 里有一处更大且方向相反的：
`crates/platform/authz/src/field.rs:43-46` 的 `NoSensitiveFields` 是
`SensitiveFieldLookup` 的**唯一实现**，逐字恒返回 `None`；配 `field.rs:92-97` 逐字
「// 无授权行：非敏感字段原样透传；加密列不得透传密文。」——
**字段级在空登记表下是 fail-open 原样透传**。
**一个 fail-closed 的空登记表比一个 fail-open 的空登记表安全一个数量级。
登记稿把较安全的那一个封为「最大的一处」。**

#### 结论一　今天唯一可复算的实体缺陷，在阶段 4 自己的交付物内部

`db/migrations/platform_authz/` 下两份种子实测：`permission_items` 种子有 **9** 个
`object_type` 取值，`object_scope_bindings` 种子只有 **3** 行
（`platform.user_accounts`／`platform.roles`／`platform.high_risk_requests`）。
差集六个：`contract_effective`、`payment`、`invoice_issue`、`ledger_posting`、
`period_close`、`sensitive_export`。`permission_items.object_type` 上既无外键也无 CHECK。

登记稿只说「阶段 5–14 无一登记」，**没查出阶段 4 自己已经差 6 行**。这是低估，一并记。

#### 结论二　但这 6 行差集**不能当判据**，因为两侧同源

候选稿把 `permission_items.object_type` 立为「不由登记方产生的独立期望名册」，
以它为左集合做双向反连接。**该支点被那份迁移自己的注释逐字打掉**：
`V20261012112000__platform_authz_backfill_permission_item_seed.sql:13-14` 逐字
「六类操作对应的业务对象尚未登记 object_scope_bindings，其 object_type
**暂取操作码自身**保持自洽，业务对象登记与其权限项细化**归其所属阶段**」。

两层同时塌：其一，**两张表的生产权在同一句话里判给同一方**，不是独立来源；
其二，那 6 个 `object_type` 是**作者自陈的占位值**，不是任何权威点名的名册——
所谓「6 行差值」实为同一作者在同一批里给自己留的占位。
加上左集合自己也没有生产者（辛-21 已裁），该判据在两种情形下探测力都归零：
阶段 5–14 什么都不写 → 两侧同为空、差集空；要写 → 权限项行与 binding 行是**同一次编写动作**。

对照本卷唯一被证明有效的独立名册形态——`crates/platform/recon/src/registry.rs` 那条
逐字「这个数是**独立于注册表本身**的一个来源，正因如此它才有用：
注册表只装得下已经注册进来的东西，拿它自己的内容当期望值，**期望与实际必然相等、
差集恒空——那样的覆盖面判定是恒真的**」。**本条正是那种恒真。**

#### 结论三　计划与实现对判定分期的说法冲突，先裁这个

- **计划**：04:346 逐字「3. **阶段三**。按 object_type 从 object_scope_bindings 取范围锚列；
  未登记直接 Deny(ScopeBindingMissing)」，而 04:330 把阶段三定名为**记录级**。
- **实现**：`decider.rs:76-83` 把它提到**阶段一之前**，逐字注释「前置硬条件」。

**两种读法给出相反的对外形态**：按计划它是记录级判定的产物，落进 04:529
「用例把 None 与记录级判定失败映射到**同一个错误分支**，两条路径共用同一段构造代码，
**禁止分别构造**」的射程；按实现它不落进。

**判：以实现为准，同批改 04:346 的分期措辞。**
理由：前置位对同一未登记类型，存在的 id 与不存在的 id 输出**同一分支、同一构造、
同一短路深度**，记录存在性信号恒为零——**这正是 04:529 想保护的东西，前置位比记录级位
保护得更彻底**。

同批修一处已有互斥：docs/error-codes.md:21 逐字把 403 给「对该对象类型**完全无权**」，
而同文件 :40 该码的触发条件逐字写「对象**已对当前主体可见**但该动作被拒」。
以 :21 为准，:40 改写为覆盖两种形态。
**并明写：改 :40 是静默的**——`xtask/src/errorcodes.rs` 只比 `code`/`category`/`http`/
`retryable` 四列，触发条件是散文列，不在比对面内。

#### 结论四　承接方：三条候选否掉两条，第三条今天也没有消费方

**（甲）`ep-migrate check` 第 14 号脚本——否，三层静默。**
（a）加文件不改常量数组则**脚本永不执行**：`tools/migrate/src/checks.rs` 的
`numbered_checks_are_the_frozen_thirteen` 只断言常量自身，**从不读目录**；
（b）改数组撞 D-04 冻结物，要独立裁定；
（c）**决定性的一条：`ep-migrate` 在 `.github/`、`deploy/`、`scripts/` 全部零命中**，
唯一两处是 `scripts/verify-release.sh` 的二进制名单——**没有任何自动化会调用它**。

**（乙）启动期 Blocking 自检——否，但候选稿给的理由是错的。**
候选稿引 01:201／00b:588 说「禁止判读业务数据行的 Blocking 项」。
**那条禁令读错了**：01:201 的白名单逐字含「**数据库元数据**」，禁的是「业务数据行」；
而且**反例已在库里**——`apps/core-server/src/main.rs:141-146` 今天就注册了一个
`Severity::Blocking` 的 `authz-snapshot-loadable`，其 run 体读的正是 `object_bindings`。
**真正的两条理由是别的**：
一、`ext.*` 自定义对象由用户在**运行期**建出，其 `object_type` 在任何启动时刻**不可穷举**；
二、今天差 6 行，Blocking 一旦生效 **T0 起不来**。

**（丙）路由能力元组 + `configdoc`——形状对，但今天没有消费方。**
00b:751 逐字「各阶段在本阶段的路由注册处一次性给出 `(CapabilityDomain, ActionClass)` 元组……
`xtask configdoc` 断言每个 `/api/v1/` 路由都能解析到一个元组，**缺失即构建失败**」，
而 `configdoc` 是 CI 第 6 阶段的 `delivered` 项——**这是全卷唯一一条已在跑的覆盖面门禁**。
**但实测该元组今天被丢弃**：`apps/core-server/src/platform/identity.rs:526/530/534` 与
`identity_admin.rs:679` 四处逐字 `for (path, handler, _capability) in …`——
**下划线丢参**，全仓无消费方。

#### 结论五　因此本裁定**不给出一条今天就能红的承接方**，如实说

这不是判据可以推迟，是**承接方基础设施本身缺位**：全卷今天只有 CI 十一个阶段是自动的，
而部署期闸门（`db/checks` 十三项、八个二进制的 `--check`）**一条都不在其内**。

**处置：辛-22 的当事人换人。** 今天该裁的不是「登记表空」——
登记表空在判定层未接线之前**不产生任何后果**。
辛-22 降为**阶段 5 判定面接入时的前置条件**，并明写三条必须同批完成：
一、六行差集补齐（或明写那六个 `object_type` 是占位、不进判定链）；
二、`(CapabilityDomain, ActionClass)` 元组**停止用 `_` 丢弃**，接上 00b:751 的断言；
三、`DenyReason` 到错误码的映射同批交付——今天**九个变体全部无对外映射**，
`ScopeBindingMissing` 只有 `types.rs` 的指标标签 `"scope_binding_missing"` 一条线，
而该线的填充路径自陈未接（`wiring/authz.rs` 逐字「判定面接入时／（阶段 5+）经此端到端填充」）。

#### 结论六　普查查出一处**比辛-22 更彻底的缺陷：承接方是幻影**

`db/checks/append_only_consistency.sql` 有**七处文档逐字指名**由 `xtask sqlcheck` 执行
（db/README.md、db/checks/README.md、02:394、03:821、08:294、10:633，以及本文件 :1070
逐字「检查脚本名固定为 `db/checks/append_only_consistency.sql`，由 `xtask sqlcheck` 执行」）。

**实测：`xtask/Cargo.toml` 的 `[dependencies]` 只有 `serde` 与 `serde_json` 两条，
没有任何 postgres 客户端——它物理上不可能执行一条活库 SQL 断言。**
`append_only_consistency` 在 `xtask/` 下命中数为 **0**；
`xtask/src/sqlcheck.rs` 的判定面逐字只有 `db/migrations` 与 `db/bootstrap`，不含 `db/checks`。

**生产方齐全（阶段 3b/7/8/9a/10 各有 backfill 与退出条件）、消费方齐全（触发器）、
门禁是幻影。** 它证明「登记了承接方」与「承接方存在」在本卷已经脱钩过一次，无人发现。
登记为**辛-24**。

#### 残余

- **A** 部署期闸门**没有自动调用方**，这比辛-22 大——`db/checks` 十三项、
  `append_only_consistency`、八个二进制的 `--check` 全挂在它下面。登记为**辛-27**
- **B** `DenyReason` 九个变体全部无对外映射，本裁定只处理其中一个
- **C** `ScopePredicateRenderer` 在 `crates/` 零实现，`object_scope_bindings` 八个业务列里
  六个今天零读者——「登记值真伪」这类判据在渲染器交付前只能验列存在、验不了列语义
- **D** `ext.*` 自定义对象在任何静态判据下都不可穷举，三条候选路径全覆盖不到
- **E** 生产方形态与交付物已不一致：04:139 与该表迁移**两处逐字都写「各业务模块在其阶段的
  wiring 中登记」**，而那三行实际由迁移自己 insert
- **F** 辛-21 与辛-22 须同批，但依据要换——不是因为 `permission_items` 是判据
  （那条已由结论二打掉），而是因为**两者的生产方在同一句迁移注释里被判给同一方**，
  拆开裁会让两边各自等对方

#### 新登记的附录辛条目

| 编号 | 缺口 | 类 |
|---|---|---|
| 辛-24 | `append_only_consistency.sql` 的承接方是幻影：七处文档指名 `xtask sqlcheck`，而 xtask 无任何 postgres 客户端 | 成立（F-28） |
| 辛-25 | 字段级在空登记表下 fail-open 原样透传；敏感字段则静默 `continue`（无错误、无日志、无指标） | 成立但需收窄（F-28） |
| 辛-26 | 路由能力元组被 `_capability` 丢弃，33 条在线端点今天无能力判定且无任何东西会报出来 | 成立但需收窄（F-28） |
| 辛-27 | 部署期闸门无自动调用方：`ep-migrate` 与 `--check` 在 CI、deploy、scripts 全部零命中 | 已由 F-23 处置；成立（F-28） |

### F-23　裁定附录辛第 27 条：部署期闸门无自动调用方

候选稿五条要点**全部被对抗性核查打掉**，其中一条是我自己标为「优先级最高」的新发现——
它是错的，本裁定撤回。更要紧的是普查查出**同形态全卷 13 处**，最大的一处接的是**发布放行**。

#### 结论零　辛-27 登记稿两处更正

**一、「`ep-migrate` 与 `--check` 在 CI、deploy、scripts 全部零命中」——`ep-migrate` 不是零命中。**
它作为镜像名出现在 `scripts/verify-release.sh` 两处。
准确表述：**`--check` 零命中；`ep-migrate` 仅作镜像名出现，无任何执行点。**

**二、「比辛-22 大」成立，但理由要换。** 不是因为挂在它下面的东西多，
而是因为**同形态在全卷有 13 处**，且其中一处接的是发布放行（见结论五）。

#### 结论一　辛-27 是一个捆绑，拆成三件，承接方各不相同

**（一）`ep-migrate check`——卷内已有调用方名词，但有两个名字且都无交付物行。**
`01-engineering-baseline.md:592` 逐字「由**升级脚本**在升级窗口内以 `ep-migrate` 账户
直接拉起并等其退出，退出码原样判定」；而 `01-engineering-baseline.md:457` 逐字
「改由**起栈脚本**以其独立账户直接拉起并等其退出、按退出码原样判定」。
**两者是不是同一个，卷内零说明。** 且「升级脚本」四字在全 15 份计划里只出现两处，
**都是理由句里的一个名词，不是任何阶段的交付物行**。

处置：不新立承接方，但 F-08 第十节第 5 步须把它**升为具名交付物并统一命名**——
否则重建时会得到两个脚本或零个。

**（二）`db/checks`——不该推给部署期，但分母是 13 不是 14。**
这十三项要的不是「一套已起的部署」，是**一台跑过迁移的库**：它们读的
`unpoliced_table_registry`、`append_only_registry` 都由迁移建并回填。
第 14 项 `append_only_consistency.sql` **今天没有执行方**（辛-24），须先解那一条。

**但 CI 跑是必要不充分**，两条理由：第 12 号断言的被测对象是**建库参数**
（`db/bootstrap/00_database.sql` 的 locale 与 ICU），不是迁移产物；
部署库还会累积**在线 DDL 在运行期建出的 `ext.*` 对象**，CI 里永远没有。
故部署期那一道**不能省**，它是第二道而非唯一一道。

**（三）八个二进制的 `--check`——三件里唯一纯粹的缺口。** 无调用方，也无人工手册兜底。

#### 结论二　我标为「优先级最高」的那条新发现是错的，撤回

候选稿第三节写「`--check` 在无库环境返回 0，是第一类缺陷（恒真的判据），优先级最高」。
**两处都错：**

**一、触发条件说错了。** 真实条件不是「无库」，是「**SQL 探针未装配**」。
而 core-server 与 job-worker **已装配真探针**（`wiring/db.rs` 两处逐字 `Some(Arc::new(
FoundationProbeAdapter::new(`），且连接池是**惰性**的（`connect_lazy_with`）——
配置与机密齐备而库不在时，探针为 `Some` → `DatabaseReachable` 判
`Verdict::Fail("数据库不可达：…")` → Blocking → **非零退出**。
**最要紧的两个二进制行为是对的。**

**二、「Pending 不计入成败」是登记在案的取舍，不是疏漏。**
阶段 1 计划第 217 行定死该口径，第 620 行「假设二」写明了替代方案
（八进程以 Degraded 启动）与**不采用的理由**（会淹没规格第 15.3 章的真实降级信号），
并配了补偿控制（Pending 只减不增、最后阶段归零），第 614 行连
`selfcheck.pending_as_failure` 这个开关都明令删除。**是决定，不是缺陷。**

**但这条线索底下有一件真的，且不需要任何口径裁定：**
`crates/platform/runtime/src/process.rs:60-65` 把 `ProcessKind::IntegrationGateway` 与
`ProcessKind::OpsAgent` 列在 `holds_sql_session()` 返**真**的一支，
而两者的 `Cargo.toml` **都没有 `ep-adapter-db-pg`**（命中数 **0**），
`src/wiring/` 下只有 `mod.rs`、无 `db.rs`、无 `probes.rs`。
且 `apps/integration-gateway/src/wiring/mod.rs` 逐字
「与 core-server 同理：`ep-adapter-db-pg` 尚未提供 `SqlProbe` 实现」——
**core-server 今天已经注入了，这半句是假的。** 三者互相矛盾。登记为**辛-28**。

#### 结论三　甲（接编排面）撤销，但候选稿给的理由方向反了

候选稿写「加一个编排 unit 会当场打破等价并判红」。**两处都错：**

**一、「当场」把静态可能性写成了运行事实。** `verify-orchestration-equivalence.py`
**自己也没有自动调用方**——`deploy/ORCHESTRATION.md:203` 逐字自陈
「**两个脚本没有接进 CI。** 流水线定义在 `.github/` 下，不在本次交付的路径范围内。」

**二、更要命的是方向反了：那条判据对甲的落地物恒真。**
该脚本的扫描 glob 只有 `*.volume`／`*.container`／`*.slice`／`*.slice.d`，
**没有 `*.service`**。而甲的 systemd 落地物正是一个 `*.service`（`Type=oneshot` + `Before=`）——
**判据一声不响地通过。** 这比「判红」严重，它是本卷第一类缺陷在判定件上的形态。

甲撤销的**真理由**是两条：`00c:2263` 逐字停写线「在 CI 平台裁定出结论之前，
**不得改 `.github/` 下任何脚本的平台绑定**」，以及 `00c:2044` 已把等价性判据撤下、
两套编排失去对象。**往一个已被判定为「正在消失」的编排形态里接闸门，
等于把闸门接到一个不会交付的东西上。**

#### 结论四　乙可以劈成两半，今轮能落一半

候选稿说「加 `services:` 算不算平台绑定，须裁定人一句话定死」。
**不必裁——卷内已给了切分线。**
「平台绑定」在 `00c:2128` 与 `00c:5371` **两次以枚举方式自释**，点名四处具体位置
（执行器标签、离线 `CARGO_HOME`、`xtask/src/ci.rs` 固定拼 bash、可执行位判定），
**数据库服务不在其中**。而 `.github/workflows/ci.yml` 自身逐字
「无论最终取哪一条，要改的都只有本文件——判定逻辑全在 xtask 与
`.github/ci/pipeline-stages.tsv` 里，一行不动。」

**故：**
- **平台无关、今轮可落**：`.github/ci/pipeline-stages.tsv` 与 `docs/ci-pipeline.md`
  第 3 节同批加一行 db-checks 阶段。登记表不是脚本、不含平台绑定。
- **平台绑定、挂 CI 平台裁定**：库怎么起（`services:` 段或执行器镜像预置）只写在 `ci.yml` 里。

**须同批解决一条，否则乙是空的**：`crates/adapter/db-pg/tests/live_pg.rs` 逐字
「未设该变量时本文件全部用例**即刻返回**并在 stderr 留痕」，
而 `.github/` 下 `EP_TEST_PG_URL` 与 postgres **零命中**。
**加库而不同时把「未设即跳过」改成「未设即失败」，等于把绿色挪个位置。**

#### 结论五　普查：「造好了没接线」全卷 13 处，最大一处接的是**发布放行**

`tools/release-gate/src/main.rs` **全文 5 行**，逐字：

```rust
//! ep-release-gate — 工具 crate 骨架。

fn main() {
    println!("ep-release-gate skeleton");
}
```

返 0。而 `RG-RLS-MATRIX-GREEN`、`RG-CI-PROBE-ABSENT`、`RG-TOOLS-EXCLUDED`、
`RG-NO-UNDECIDABLE` **四个发布门禁项全部把判定委托给它**——
`testkit/tests/rls_matrix.rs:8-9` 逐字「判绿。绿判定属发布门禁项 `RG-RLS-MATRIX-GREEN`
（阶段 14 的 / ep-release-gate 逐项判定）。」

**完整链条：`rls_matrix` 全 Skipped → 绿判定委托给一个 println 骨架 → 骨架返 0。**
这比辛-27 描述的任何一项都彻底，且**它守的是发布放行**。登记为**辛-29**。

`tools/bench/src/main.rs` 同为 5 行骨架（`println!("ep-bench skeleton");`），
承接附录 A 的性能容量基线。登记为**辛-30**。

#### 结论六　一处「有调用方，但判据自己比自己」——比无调用方更坏

下列只是历史 Linux 实现缺陷证据；`verify-connection-budget.sh` 已被现行 `scripts/verify-connection-budget.ps1` 取代，不得恢复或被 CI 调用。历史 `scripts/verify-connection-budget.sh` 的 `spec_rows()` 是一个**写死八行的 heredoc**，
第二项把这八行加总，再与**同一文件里**的 `EXPECT_RESIDENT_SUM=42` 比——
**规格第 7.7 章一次都没被读到。**

且第四项只断言 `portal-gateway` 与 `plugin-host` **不**链接 `ep-adapter-db-pg`，
**没有反向断言** `integration-gateway` 与 `ops-agent` 链接了。
结合结论二的实测：**规格 42 条常驻连接里记在这两个进程头上的 7 条，
代码里没有任何池创建点，而这个判定件永远绿。** 登记为**辛-31**。

#### 一处须如实交代的

**CI 是真的**——`.github/workflows/ci.yml` 存在。**但它没有库**（`.github/` 下 postgres 零命中）。
本卷历轮我报的「六门禁全绿」是我**在本机逐条跑 `cargo run -p ep-xtask -- <gate>` 的结果**，
那六道判的都是静态面，不需要库；**它们的绿与 `db/checks` 十四项的状态无关**——
后者今天在任何地方都没有跑过。这一点此前各轮未加区分，在此讲明。

#### 残余

- **A** `spec:1410` 明列的「部署与升级回退手册」**全卷零认领**；
  `14:45` 点名的落点 `docs/runbooks/` 与 `docs/delivery/` **两个目录都不存在**
- **B** 「升级脚本」与「起栈脚本」是否同一个，卷内零说明
- **C** CI 平台裁定（`00c:2270` 第十一节第 4 条）未出，乙的另一半挂在它下面
- **D** `docs/ci-pipeline.md` 与 `run-pipeline.sh` 两处陈述已被 `xtask ci` 的交付推翻，
  一说登记表已作废、一说它是唯一真值，无判定件覆盖散文段
- **E** 本轮全部结论均为**静态读码，一次未跑**——
  「一台迁移过的空库足以让十三项产生真判定」这一论断仍未经真实执行验证

#### 新登记的附录辛条目

| 编号 | 缺口 | 类 |
|---|---|---|
| 辛-28 | `holds_sql_session()` 对两个进程返真，而两者不依赖 db 适配层、无 probes；且两处模块注释与代码事实相反 | 成立（F-28） |
| 辛-29 | `ep-release-gate` 是 5 行 println 骨架返 0，而四个 `RG-*` 发布门禁项全部委托给它 | **已撤销**（F-24） |
| 辛-30 | `ep-bench` 是 5 行骨架，承接附录 A 性能容量基线 | **已撤销**（F-24） |
| 辛-31 | 历史 `verify-connection-budget.sh` 的期望值与实际值同源（自己比自己），且缺反向断言；现行脚本为 `scripts/verify-connection-budget.ps1` | 成立但需收窄（F-28） |

### F-24　撤销辛-29 与辛-30，逐条撤回本人的夸大，另立两条真缺口与四条取证纪律

**本轮的主要产出不是新裁定，是撤销与更正。** 辛-29 是我上一轮立的条目，
经取证与对抗核查，它**不是缺陷，是排期项**；我给它的四句定性**逐句为假**。

#### 结论零　逐条撤回上一轮的夸大（本节效力高于结论本身）

**（一）「四个发布门禁项全部把判定委托给它」——三层都假。**

- **数目假。** `14:546` 那张表是**五项**，我整条漏了 `RG-UNWIRED-ABSENT`——
  而它恰是五项里承接最扎实的一项（`xtask/src/archcheck/source.rs` 的
  `unwired-absent` 规则已落码，且在 CI 第 3 阶段真跑）。
- **层级假，这是最要命的一处。** `14:546` 表头逐字 `| 门禁项 | 判据 | 判据提供方 |`，
  第三列五项的取值分别是「阶段 1 的 ci-probe feature 门控」「本阶段」「阶段 4」
  「阶段 1 的 archcheck 规则 unwired-absent」「阶段 1 的 archcheck 三态输出」——
  **无一是 ep-release-gate**。计划给它的角色是「逐项判定，判定结论进入发布证据包」，
  即**判定的汇总收口方**。**我把聚合层说成了判据层。**
- **半径假。** `14:574` 逐字它要判的是「第 22 章十五条与第 17.2 章通过标准逐条产出
  判定结论」**加**五个门禁项**加**证据包组装。五个门禁项只是其中条数最少的一类。
  **我同时把它的职责说小了、把性质说反了。**

**（二）「完整链条：`rls_matrix` 全 Skipped → 绿判定委托给一个 println 骨架 → 骨架返 0」
——链条断在第二环，第三环从未发生。**

`testkit/src/matrix_32.rs:201-205` 逐字：

```rust
    /// 全绿判据：零失败且零跳过。`RG-RLS-MATRIX-GREEN` 据此判定；
    /// 无活库时 skipped 非零，门禁如实不绿，不以 Skipped 顶过。
    pub fn is_green(self) -> bool {
        self.failed == 0 && self.skipped == 0
    }
```

配 `testkit/tests/rls_matrix.rs:75` 逐字 `assert_eq!(summary.passed, 0, "探针未接通时不得判过");`
与用例 `gate_verdict_is_not_green_without_a_probe`。
**今天这一项的判定结论是「不绿」，由 testkit 本地算出、由用例强制。没有委托动作。**

**（三）只引半句，把正面范例引成了缺陷证据。**
`testkit/tests/rls_matrix.rs:5-9` 全文逐字：「本阶段无活库：32 组矩阵与 4 项入口借用一律
Skipped，目标是**如实输出结构化报告并守住『未覆盖不等于通过』的纪律，而不是判绿**。
绿判定属发布门禁项 `RG-RLS-MATRIX-GREEN`（阶段 14 的 ep-release-gate 逐项判定）。」
我从「判绿。」起引。**这段注释是本卷纪律的正面范例——它明确拒绝判绿并点名了谁来判。
把它引成缺陷证据，是本轮最该认的一处错。**

**（四）「全卷此病最彻底的一处」——无实测支撑，同批撤。**

#### 结论一　判排期项，撤销辛-29；理由不沿用 F-23

四要件齐备：**阶段号**（14，`00-overview.md:230` 的 B-11）、**交付物行**（`14:42` 逐字
「11. 发布门禁工装 ep-release-gate：证据收集、按第 17.2 章通过标准与第 22 章十五条逐条判定、
发布证据包组装。」）、**验收判据**（`14:574` 退出条件 15）、**今天零消费方**。

零消费方三证：全仓 `release[-_]gate` 命中中，`xtask/` 内 6 处**全部是把它排除出制品的断言**
（`sbom.rs` 的 `EXCLUDED_PACKAGES`），无一是调用；`CARGO_BIN_EXE_ep-release-gate` 零命中；
不在 `reproduce.rs::BINARIES` 与 `sign.rs::IMAGES` 任何制品花名册里。

**且它与 F-23 结论一（一）的 `ep-migrate` 方向相反**：那件是「能力已在、调用方缺位」，
这件是「闸门已在、能力缺位」。**结论同向而理由不可复用——「与 F-23 同形」这个说法本身也不准。**

#### 结论二　辛-30（`ep-bench`）同批撤，但**分别撤、不合并**

同形证据是硬的（`14:41`／`14:42` 相邻两行、同一张 crate 表、同一条 B-11、
同为 `EXCLUDED_PACKAGES` 的两个成员、同为 5 行骨架）。
**但不合并**：辛-30 的验收判据落在 `14:512-533` 第 8.5 节的**八条编号判据**上，
与辛-29 的退出条件 15 不是一处。一句「两条同撤」会把第 8.5 节整节从视野里抹掉。

#### 结论三　骨架返 0 违反本仓自己的惯例，但并入 X-3，不单独立条

`xtask/src/main.rs:44-45` 逐字：

```rust
/// 未实现的子命令退出码。与「参数错误」的 2 区分，避免误读为通过。
const EXIT_NOT_DELIVERED: u8 = 70;
```

**F-52 作成前，`tools/release-gate` 与 `tools/bench` 是全仓仅有的两个「未交付却返 0」的可执行件；该现状已被后句的现行口径替代。**
但这与 **X-3** 是同一件事的两支。该历史分析现由 F-52 唯一收口：不删两个 crate；`tools/release-gate` 与 `tools/bench` 从阶段 1 即以非产品骨架存在并排除在产品 SBOM 外，阶段 14 真实能力交付前固定返回 `EXIT_NOT_DELIVERED = 70`，只有阶段 14 的真实命令成功才可返回 0。X-3 已关闭，不再保留分支。

#### 结论四　普查查出两处**比我登记的任何一条都大**

**（甲）`xtask configdoc` 的路由断言整段不存在，而它在 CI 里绿着。**

三处上位权威点名它：`00-overview.md:204` 逐字「xtask configdoc 断言每个 /api/v1/ 路由
都能解析到一个元组，缺失即构建失败」；`04:433` 同义；`14:576` 退出条件 21。

**实测：`xtask/src/configdoc.rs:1` 自陈逐字「共三段判据」**（配置键、指标名、单据类型码），
**`xtask/src/` 全目录 `api/v1` 命中数为 0**。而 `.github/ci/pipeline-stages.tsv` 第 6 阶段
逐字 `6	registry-docs	xtask	configdoc	delivered`。

**与辛-24 同形但更严重**：辛-24 的判据今天**没有被测输入**（活库不在），
本条的判据**有被测输入**（33 条路由都在）**却整段不存在**，
且既没进 `uncovered` 也没进 `deferred`——**它对外表现为「通过」。**

**而我上一轮登记辛-26 时，只写了「元组被 `_capability` 丢弃」这个现象，
把守它的门禁根本不存在这件事整个漏了。** 登记为**辛-32**。

**（乙）阶段 1 冻结的 foundation 值类型与端口全仓零实现。**

`crates/foundation/src/lib.rs` 的七个 `pub mod` 是
`capability`／`error`／`id`／`module`／`port`／`principal`／`security`——
**没有 money、没有 clock**。

- `Money` 在 `crates/foundation/` 下命中 **0 个文件**；
  全仓 `crates/` 下 `Money` 的**唯一**命中是**我自己写的一句文档注释**
  （`crates/platform/flow/src/expr/value.rs:6`）。
- `Clock` 的唯一命中是 `crates/adapter/kms/src/cache.rs` 的一个**无关本地别名**
  `pub type Clock = Arc<dyn Fn() -> Instant + Send + Sync>;`。
- `struct Money`／`trait Clock`／`trait IdGen`／`trait Rng` 四者全仓命中数合计 **0**。

而阶段 1 计划 `:148` 逐字「| Money | 内含 Decimal，构造时断言 scale 恰为 2，
超出即构造失败 | 绝对值上限 10^16，超出返回错误而非截断 |」，
`:397` 逐字给出它们的单元测试清单；`02:9` 逐字把它们列为阶段 2 的**前置**。

**门禁面同样漏空**：`xtask/src/archcheck/frozen.rs` 的 `EXPECTED` 八项**一项都不覆盖它们**。

**这是「登记了承接方、承接方不存在」在本卷最大的一处，且它是阶段 1 自己的交付物，
不是任何未来阶段的排期。** 登记为**辛-33**。
（阶段 1 是否已宣告闭合，仓内无状态文件可证，此处标为待裁定人确认。）

#### 结论五　我每轮都在说的一句话今天已不成立，当面更正

我历轮报「六门禁全绿」。**那六道我逐条跑过、各自返 0，这一半是真的。
但流水线自己的聚合器不这么判。**

`xtask/src/ci.rs:7` 逐字「返回 3 记为**存在不可判定项，不得当作通过**」，
`:160` 逐字 `judged_undecidable!(coverage, sbom, sign, reproduce);`。

**本轮实跑 `cargo run -p ep-xtask -- ci`，聚合退出码 = 3，不是 0。**

**此后各轮的验证口径改为：报六道静态门禁各自的退出码，并同时报 `xtask ci` 的聚合码，
不得只报前者。**

#### 结论六　方法论：连续五轮夸大是**取证方式**的问题，不是判断力的问题

被打掉的结论里**没有一条是推理链错**，全部是**取证只做了一半**。四种可复现的缺陷：

1. **只 grep 支持假设的那一侧，不 grep 会证伪自己的那一侧。**
   辛-26 我 grep 了 `_capability`（支持结论），没 grep `xtask/src/` 里有没有 `api/v1`——
   **一条命令就会当场发现更大的洞**（结论四甲）。
2. **引表不引表头、不数行数。** 五项读成四项，列名「判据提供方」没读。
3. **引注释从句中截起。** 从「判绿。」起引，把正面范例引成缺陷证据。
4. **用级别词而不给全集。**「最大」「最彻底」「唯一」。

**据此立四条取证纪律，效力及于本卷后续全部登记：**

> **一、「委托」不等于「提供」。** 凡引某张门禁／判据表登记缺陷，**须先读该表的列名**，
> 分清「判据提供方」与「判定汇总方」；把聚合层说成判据层的登记一律不成立。
>
> **二、登记前先数条目数。**
>
> **三、禁用「最大／最彻底／唯一」这类级别词**，除非同批给出全集清单与排序键。
> 替代写法：「在本轮实测的 N 处中排第一，全集清单见 X」。
>
> **四、每条登记须过「四要件核对」**：阶段号／交付物行／验收判据／今天零消费方——
> **四格全打勾即禁止登记为缺陷**，改写为验收判据。
> 辛-23、辛-29、辛-30 三条若走过这一格都不会被立案；辛-21、辛-24 会当场通过。

#### 结论七　据第四条纪律回溯复核已登记的十二条

| 编号 | 三分 | 要点 |
|---|---|---|
| 辛-15 | 真缺口 | 但当事人偏小——`Clock` 属阶段 1 冻结项，并入辛-33 一并看 |
| 辛-17 | **需复核，多半应撤** | `11:230` 有 `object_kind` 判别子、`11:244` 有逐变体 JSON Schema 校验、`11:760` 有退出条件。要撤须先正面论证「为什么不够」 |
| 辛-18 | 真缺口 | `default_expr` 全卷唯一一处，无解析方 |
| 辛-20 | 真缺口，**但当事人是裁定自身** | 00b 第 12.1 节登记表真实存在且已被 `archcheck` 读；缺的是 F-20 没写进去。应改记为落地动作，不占附录辛编号 |
| 辛-21 | 真缺口，**本批最硬** | 「权限项」在 05 至 12 与 14 **全部零命中**，无任何阶段被点名为生产者 |
| 辛-23 | **排期项，应撤** | `00-overview.md:77` 逐字给了实现方阶段号，四要件齐备 |
| 辛-24 | 真缺口 | 判定面无 `db/checks` |
| 辛-25 | **需复核，当事人要换** | 「fail-open」有登记且点名了替代承接方；真的那一半是敏感字段**被静默丢弃**（无错误、无日志、无指标） |
| 辛-26 | **需复核，且漏了更大的一半** | 「今天无能力判定」正是 `04:739` 退出条件 17 要的形态；漏的那一半是结论四甲 |
| 辛-28 | 真缺口，**但少了后果那一半** | 见结论八 |
| 辛-30 | **排期项，应撤** | 见结论二 |
| 辛-31 | 真缺口，**但我对它的描述有一处新造的夸大** | 见结论九 |

#### 结论八　F-23 结论二撤得过宽，部分恢复

我在 F-23 撤回了「`--check` 在无库环境返 0」这条发现。
**撤回时只验了 core-server 与 job-worker 两个装了真探针的，未验其余六个。**

对 `integration-gateway` 与 `ops-agent`，该发现**仍然成立**：
两者 `holds_sql_session()` 返真，而 `apps/integration-gateway/src/wiring/mod.rs` 逐字
`pub fn sql_probe() -> Option<Arc<dyn SqlProbe>> { None }` → 四项 SQL 自检全 `Pending`
→ Pending 不计成败 → **`--check` 返 0**。**补回辛-28 的当事人。**

#### 结论九　F-23 结论六对辛-31 的一句描述是新造的夸大，同批撤

我在 F-23 写辛-31「有真实调用方且今天真的返绿」。
**实测无调用方**：`verify-connection-budget` 在 `.github/`／`deploy/`／`scripts/` 下
除脚本自身的用法串外零命中；CI 第 11 阶段逐字跑的是
`scripts/verify-resource-limits.sh`，**是另一个脚本**。
**方向是高估现状健康度，与其余各处夸大方向相反，同批撤。**

#### 本轮未做到的，不得算作已核

- F-23 结论五那个「13 处」的底稿**本轮未穷举**，只新增了两处同形。
  按第三条纪律，**在给出全集清单之前不得再写这个数字**。
- 规格第 22 章十五条与第 17.2 章通过标准的承接状态**未逐条核**。
- `crates/platform/authz/` 十二个源文件只读了 `field.rs`，辛-25 的另一半未核。
- 辛-19 不在本轮复核清单内，未核。

#### 新登记的附录辛条目

| 编号 | 缺口 | 类 |
|---|---|---|
| 辛-32 | `xtask configdoc` 的 `/api/v1/` 路由能力元组断言**整段不存在**（自陈「共三段判据」，`api/v1` 零命中），而三处上位权威点名它、CI 第 6 阶段标 delivered，对外表现为「通过」 | 成立（F-28） |
| 辛-33 | 阶段 1 冻结的 `Money`／`UnitPrice`／`Quantity`／`Rate` 与 `Clock`／`IdGen`／`Rng` 全仓零实现，`frozen.rs` 的 `EXPECTED` 八项一项都不覆盖它们 | 已由 F-25 换当事人；成立（F-28） |

#### 撤销记录

| 编号 | 处置 | 理由 |
|---|---|---|
| 辛-29 | **撤销** | 排期项，四要件齐备；四句定性逐句为假 |
| 辛-30 | **撤销** | 排期项，四要件齐备；验收判据落在第 8.5 节，另行看管 |

### F-25　裁定附录辛第 33 条：当事人更换为已发生的绕过；并修补 F-24 第四条纪律

**辛-33 不撤销，但当事人整体更换**；同时**F-24 新立的「四要件核对」第四格判法有缺陷，
本裁定当场修补**——本条是那条纪律的第一次适用，修补比裁定本身重要。

#### 结论一　四要件核对逐格，第四格是假打勾

| 格 | 判 | 依据 |
|---|---|---|
| 阶段号 | 勾，**但与辛-29 有决定性不同** | 属主是**阶段 1**。辛-29 的属主是阶段 14（未开跑）；本条属主阶段 1 的**其余七个模块已交付**，且阶段 2（`db/migrations/` 实测 69 个 `.sql`）、3、3b、4 均已落码。`02:9` 逐字把这些项列为阶段 2 的**前置**——**这句今天为假，而以它为前置的阶段 2 已经跑完了。「还没轮到」在本条不成立。** |
| 交付物行 | 勾 | `01:148` 逐字「\| Money \| 内含 Decimal，构造时断言 scale 恰为 2，超出即构造失败 \| 绝对值上限 10^16，超出返回错误而非截断 \|」 |
| 验收判据 | 勾，**但这一格是空的** | `01:397`／`01:411` 在场，而全仓 `proptest`／`trybuild`／`to_money`／`FixedClock` 命中均为 **0**。**把一条缺陷改写成一条本身零实现的验收判据，是换抽屉，不是处置。** |
| 今天零消费方 | **假打勾** | 见结论二 |

#### 结论二　修补 F-24 第四条纪律的第四格判法（本轮最重要的产出）

`Money` 在 `crates/` 下命中恰 **2** 条，**两条都是注释**，`.rs` 正文使用为 0。
**这个读数是对的，推论是错的。**

**因为命中数为 0 恰恰是绕过的结果，不是「还没人要」的证据。**
用「符号命中数」当「消费方数」的代理量，在缺口已被绕过的场景下
**本身就是第二类缺陷（取不到的取值）——而且它出现在核对表自身。**

现场证据就在那两条注释里：`crates/platform/flow/src/expr/value.rs:5-7` 逐字
「基线第 3.5 节逐字把账面金额的 Rust 类型定为「`foundation::Money`，内含
`rust_decimal::Decimal`」。守卫表达式里最常见的一句就是 `vars.amount > 10000`——
它比较的正是那个金额，只能与它同源。」
**作者看着计划原文确认了自己是消费方，然后落了 `:46` 的 `Number(Decimal)`。**

> **修补后的第四格：不数符号命中，数「该语义今天有没有被另一套载体承载」。**
> 判法：先定出该项承载的**语义**（金额、当前时间、ID 生成、随机源），
> 再 grep 那个语义的**替代载体**；替代载体存在即为「有消费方」，四格不全勾。

**佐证：F-24 自己在辛-29 上的操作化本就不是数命中数**——
它逐字写「`xtask/` 内 6 处**全部是**把它排除出制品的断言……**无一是调用**」，
那是**按角色分类**。**纪律的原意本就如此，是我写纪律时把它写窄了。**

#### 结论三　当事人换成**八处**已发生的绕过

核查列了九处，对抗核查**移出一处**：`crates/platform/audit/src/jcs.rs` 把金额强制改为
字符串承载，是 JCS 规范化的正确做法（双精度确实承载不了 2 位小数金额），
即便 `Money` 存在也会这么写。**移出。**

另两处主张移出的**不成立，且被文件自己的原文证伪**：
- `recon/src/model.rs` **不是展示层**——`:174-179` 逐字「差异金额恰恰是
  **用来判断差异是否为零**的那个数——舍入让一个非零差异变成零，
  就等于把一条真差异放行进关账」。**用来判零的金额是域内取值。**
- `db-pg/src/conn.rs` **不是合理选型**——`02:9` 把阶段 2 对这些类型的动作写死为
  「补编解码」，而 `conn.rs` 是**另立一套**：`DbValue` 七个变体无金额变体，
  文档把金额排除在外、要求调用方自行以「分」传入，自定上限常量与断言函数。
  **「Money → 库内表示」这条编解码一条都没有。**

**金额语义 4 处，4 套互不相同的载体：**

| # | 落点 | 载体 |
|---|---|---|
| 1 | `crates/platform/flow/src/expr/value.rs:46` | 裸 `rust_decimal::Decimal`。`:5-7` 自陈看着计划原文在绕——**这一处是本裁定人自己写的** |
| 2 | `crates/platform/authz/src/reauth.rs:39` | `canonical_amount(cents: i128) -> String`，定长 20 字符、自定 scale=2 与整数 16 位——**是 `01:148` 的私版重实现** |
| 3 | `crates/adapter/db-pg/src/conn.rs:112` | `MONEY_MINOR_UNITS_MAX: i64`；`DbValue` 无金额变体 |
| 4 | `crates/platform/recon/src/model.rs:185-187` | 三个 `String` |

**端口语义 4 处：**

| # | 落点 | 形态 |
|---|---|---|
| 5 | `crates/platform/runtime/src/incident.rs:53-57` | `today_shanghai()` 直取墙钟——**这就是 `01:167` 的 `Clock::today_cn`**，改名、落 platform 层。全仓 `today_cn` 命中 **0** |
| 6 | `crates/adapter/kms/src/cache.rs:14-19` | `pub type Clock = Arc<dyn Fn() -> Instant + Send + Sync>;`——**对 F-24 把它记作「无关本地别名」这一定性，本裁定收窄一格**：它占用了冻结端口的名字，形态正是 `Clock` 端口本该有的可注入形态 |
| 7 | `crates/platform/runtime/src/incident.rs:14` | `pub struct IncidentNoGen`——`01:167` 要求它是 foundation 的**端口 trait**，实际是 platform 的具体结构体，落点错位 |
| 8 | `crates/adapter/kms/Cargo.toml:26-27` | 逐字「foundation 尚无 Rng 端口（port/ 目录只有 db/doc/kms/search/tx），信封 nonce 的随机源按任务裁定暂用 rand，**偏离已在任务汇报标注**」——**而「任务汇报」不是 00b 第 12.1 节，仓内两段登记表里都没有这条** |

**不给「169 处直取」那个数。** 对抗核查在这一点上站住了：三路取证给出的生产期折算
互差近四倍（`Utc::now` 一路说 56、一路说 15），**我没有逐处区分生产码与 `#[cfg(test)]`**。
按 F-24 纪律二与纪律三，**该数不写**，只留上表八处具名落点。

#### 结论四　`01:167` 的符号禁令四路全空，**且本轮是实测不是读码**

`01:167` 逐字「端口 trait：`Clock`（now 与 today_cn）、`IdGen`（new_id）、`Rng`（fill_bytes）、
`IncidentNoGen`（next）。domain 层禁止绕过这四个端口，
**由 `xtask archcheck` 的符号禁令强制**。」

`xtask/src/archcheck/source.rs:166-175` 唯一的符号级检索式是 `domain-contract-no-io`，
其 `NEEDLES` 恰 **6** 个：`std::fs`、`std::net`、`std::process`、`tokio::fs`、
`tokio::net`、`tokio::process`——**没有 `Utc::now`、没有 `rand`、没有任何一个端口名。**

**并且本裁定实跑了一次**：上表八处绕过全部在库的情况下，
`cargo run -p ep-xtask -- archcheck` **退出码 = 0**。
（对抗核查指出候选稿这句「没有任何门禁会发现」只读了码、未实测——该批评成立，本裁定补测。）

**这是与辛-32 同形的第三处幻影承接方。**

#### 结论五　另拆辛-34：门禁不是看不见缺口，**是反向锁死**

`00b:128` 逐字：「ep-foundation 的顶层模块固定为下表七项。本表即 `xtask archcheck` 的
foundation-module-registry 规则的比对对象，与 `crates/foundation/src/lib.rs` 中的
`pub mod` 声明**逐行相等，多一个少一个都判违反**；新增或删除顶层模块
**必须先改本表并走基线修订，不得只改代码**。」

而 `xtask/src/archcheck/foundation.rs` 真的去读那份 md（`REGISTRY_MARKER` 取的就是
00b 那句话）再与 `lib.rs` 的 `pub mod` 行逐行比。

**后果：今天谁去建 `crates/foundation/src/money.rs` 并加一行 `pub mod money;`，
`foundation-module-registry` 会当场判违反。门禁不是看不见缺口，是挡着修。**

配 `00b:43` 逐字「稳定通用类型：Id、Money、Quantity、UnitPrice、Rate、
AccountingPeriodRef、…Clock 与 IdGen 端口；**上述类型的签名与取值见第 1.4 节**。」——
而第 1.4 节内对这批项**没有任何签名与取值**。**一句从未成立的交叉引用。**

**这条独立于辛-33 成立**：即便辛-33 整条撤销、四格全打勾，它仍然为假。
故另立**辛-34**，不并入本条。

#### 结论六　登记数更正：缺口面是 12 项，不是 7 项

`01` 第 5.1 节表体实测 **19 行**，已交付 11、**零实现 8**：
`Money`、`Amount`、`UnitPrice`、`Quantity`、`Rate`、`AccountingPeriodRef`、
`DomainEvent`、`Redacted<T>`；加 `01:167` 的**4 个端口全缺**
（`IncidentNoGen` 以错落点的具体类型存在）。
**合计 12 项。辛-33 登记行只点了 7 项，漏登 `Amount`、`AccountingPeriodRef`、
`DomainEvent`、`Redacted<T>`、`IncidentNoGen` 五项。**（F-24 纪律二的又一次自查失败。）

#### 结论七　一处物理约束，一并记

`crates/foundation/Cargo.toml` 的 `[dependencies]` 实测四项：
`uuid`、`chrono`、`async-trait`、`serde`——**没有 `rust_decimal`**。
**今天在 foundation 里连写 `Money` 的原料都不在依赖表里。**

#### 本轮未做到的

- 「169 处直取」未逐处区分生产码与测试码，**该数不用**；八处具名落点是逐处核过的
- `crates/contract`／`domain`／`application` 各 15 个 crate 的空壳状态只做了抽样
- 阶段 1 是否已宣告闭合，**仓内无状态文件可证**——结论一格一的判断依据是
  「阶段 2/3/4 的交付物已在」这一间接证据，标为推断

#### 新登记与更正

| 编号 | 处置 |
|---|---|
| 辛-33 | **保留，当事人更换**为「八处已发生的绕过」＋「`01:167` 符号禁令四路全空（已实测 archcheck 返 0）」；缺口面由 7 项更正为 12 项 |
| 辛-34 | **新立**：`00b:43` 指向第 1.4 节的交叉引用从未成立；且 `00b:128` 与 `foundation-module-registry` 构成反向锁死——建 `pub mod money;` 会当场判违反 |

**并修补 F-24 第四条纪律**：四要件的第四格「今天零消费方」，
**判法改为数「该语义有没有被另一套载体承载」，不数符号命中。**

### F-26　裁定附录辛第 34 条：半二整体撤回（本人定性错误），半一当事人换人

#### 结论零　半二「反向锁死」**整体撤回**——这是本裁定人上一轮的定性错误

F-25 结论五逐字写过：「今天谁去建 `crates/foundation/src/money.rs` 并加一行
`pub mod money;`，`foundation-module-registry` 会当场判违反。**门禁不是看不见缺口，
是挡着修。**」

**那句在它自己举的那个落点上为真，但被推广成「门禁挡着修」，推广不成立。** 四条：

**一、我漏引了同一行的末句。** `00b:128` 末句逐字「**模块内部的文件划分不在本表的
判定面内。**」而 F-25 的引文停在「不得只改代码。」——**又是从句中截断**，
与 F-24 已经认过的第三类取证错误同型。**同一种错在认过之后又犯了一次。**

**二、工具只读一个文件。** `xtask/src/archcheck/foundation.rs:71` 逐字
`let path = root.join("crates/foundation/src/lib.rs");`——比的是 `lib.rs` 的
`pub mod` 行，**不递归目录**。

**三、仓内已有一条走通这条路的活先例。**
`crates/foundation/src/port/sensitive.rs` 在库，`crates/foundation/src/port/mod.rs:7`
逐字 `pub mod sensitive;`，而 `00b:136` 的 port 行**没有点名 sensitive**。
**它今天就在库里，而 `foundation-module-registry` 判通过。**

**四、`00b:128` 给的不是禁令，是次序。** 逐字「必须**先改本表并走基线修订**」——
一条明文可走的两步程序。同族条文另有四处（`00b:33`、`:179`、`:181`、`01:497`）。
**门禁挡住的是「只改码不改表」，这正是 doc-first 纪律的正确实现。
把纪律生效记成故障，是本裁定人的判读错误。**

**同批更正 F-25 结论七：「连原料都不在依赖表里」也错了。**
根 `Cargo.toml:32` 逐字 `rust_decimal = { version = "1", default-features = false,
features = ["std"] }`——**已在工作区依赖表**；`crates/platform/flow/Cargo.toml:15`
已 workspace 引用。缺的只是 `crates/foundation/Cargo.toml` 里那一行，
**而且加那一行不触发 archcheck 任何规则**。原句收窄为「不在 foundation 自己的依赖表里」。

#### 结论一　半一成立，但当事人从「那一句」换成「那一节」

**不取甲（收窄为「一句从未成立的交叉引用」）。** 理由：把 `:43` 删掉之后，
机检照样全绿、8 项照样无人补、§1.4 照样自称「冻结的跨阶段共享类型」。
**收窄到句子上，等于把判定面交给一个可以靠删句子规避的对象**——与「未覆盖 ≠ 通过」相悖。

**取丁：当事人是 §1.4 自身的覆盖面。**
`00b:124` 标题「### 1.4 ep-foundation 冻结的跨阶段共享类型」加 `:126` 逐字
「本节各项由阶段 1 一次性冻结，**签名与取值全阶段唯一**」是一条硬断言，
而节内（自数边界 124–240，共 117 行）对下列 **8 项零对应**：

**Money、Quantity、UnitPrice、Rate、AccountingPeriodRef、DomainEvent 信封、Clock、IdGen。**
其中 **7 项**在 124–240 行内**零字符出现**；`IdGen` 只在 `:191` 以一句理由说明顺带出现
（讲的是 `SYSTEM_PRINCIPAL_ID` 的取值选型，非签名非取值）。

`:43` 那句交叉引用作为该缺口的**表征面**写进成因，不单列。

#### 结论二　「内容缺失还是指错节号」——两侧分开答

- **取值一侧：多半在别处，属指错地方。** `00b:325` 逐字「| 账面金额 | `numeric(18,2)` |
  `foundation::Money`，内含 `rust_decimal::Decimal` |」，`:326`–`:328` 给
  UnitPrice／Quantity／Rate 的精度；DomainEvent 信封在 §6.1 的 JSON 块。
- **签名一侧：取不到。** `01:145` 表头逐字「| 类型 | **定义要点** | 边界条件 |」——
  给的是定义要点，不是签名；`:148` 那一行既未写元组结构体还是具名字段、
  未写 `Decimal` 的 crate 路径、未写构造函数名。
  **对照 §1.4 对 `Tx`／`SnapshotCtx`／`CapabilityDomain` 给的是可编译 Rust 代码块——
  两者不同档。**

故准确说法（不用级别词）：**取值多半可取但不在被指向处；可编译签名 6 项全卷取不到；
另 2 项（`Amount`、`AccountingPeriodRef` 的展示格式）连取值都取不到。**

**乙（笔误）证伪，两条：** 其一，git 全史 13 个版本核过——首版无该句，
自第二版引入起至今，节内命中数**恒为 1 行且恒是同一句**，**无「曾经成立后被搬走」这一支**；
其二，00b 跨文档引用一律写文档名（`:237`、`:239`、`:798`），裸写「第 N 节」指 00b 自身，
而 **00b 内不存在任何单一节号能让该句成立**——改节号修不好这一句。

#### 结论三　抽验结果：不是「全卷交叉引用无校验」那么大

00b 全文「见第 N 节／按第 N 节」命中 **23 处**，逐处核，**不成立 2 处**（`:43` 与 `:237`）。
**未抽验 0 处。不作全称判断。**

承接方面：`xtask` 子命令 **12** 个，以 00b／01 为判定面的规则 **3** 条，
**以 00b 内部交叉引用或 §1.4 覆盖面为判定面的规则 0 条**。
把 `:43` 的「第 1.4 节」改成任意节号或整句删掉，**判定结果不变**——属第三类缺陷。

#### 结论四　F-25 修补后的第四格，在文档类缺陷上的适用边界

本条的语义是「这批跨阶段共享类型有一个**受基线约束的冻结面**」。
唯一候选载体是 `01` §5.1，**不等价**，三条：
（a）列名是「定义要点」不是签名；
（b）它是阶段计划不是基线，而 `00b:3` 逐字要求「必须在计划中显式标注为本阶段新增决定，
并在阶段结束时**回写本基线**」——**回写未发生**；
（c）两面已漂移——`00b:701` 的 `AppError` 6 字段 vs `01:158` 的 9 字段，
而 `00b:133` 明写「`AppError` 的字段构成见第 10.2 节」。

> **补一条适用边界：第四格的「另一套载体」在文档类缺陷上读作
> 「同等约束力的第二份文档」，不是代码；约束力不同等的（计划 vs 基线）不算承载。**

#### 结论五　再更正一处历轮口径：`xtask` 不是六个子命令，是 **12** 个

`xtask/src/main.rs:29` 逐字 `const SUBCOMMANDS: [&str; 12] = [`。
我历轮说「六门禁」——那是**我选跑的六道**，不是 xtask 的全部。

**此后验证口径写全：六道静态门禁各自返 0（xtask 共 12 个子命令），
`xtask ci` 聚合退出码 3。**

#### 结论六　§1.4 登记表与实际文件的不一致 **7 处**，是 `:128` 末句实际生效的旁证

表点名而不存在 **1** 处（`crates/foundation/src/error.rs`，实际为 `error/mod.rs`
与 `error/codes.rs`）；实际存在而表未点名 **6** 处（`lib.rs`、`error/mod.rs`、
`error/codes.rs`、`port/mod.rs`、`port/sensitive.rs`、`security/mod.rs`）。
**七处同时在库而 `foundation-module-registry` 判通过。** 不计入辛-34 的缺口条目。

#### 待核（核查提出，本裁定未自行复核，不作已核）

- 核查称 `00c:1809`（H-07）判「两条路都不通」，其中「改塞 `port::ipc`」一条
  已被 `port/sensitive.rs` 证伪。**本裁定标为待核，下一轮处置。**

#### 辛-34 重述与更正

| 项 | 内容 |
|---|---|
| **撤回** | 半二「反向锁死」整条——本裁定人定性错误，成因是漏引 `00b:128` 末句 |
| **保留并换当事人** | 由「`00b:43` 那一句」改为「`00b` §1.4（124–240 行）的覆盖面漏 8 项」 |
| **连带更正** | F-25 结论七「连原料都不在依赖表里」→「不在 foundation 自己的依赖表里」 |
| **口径更正** | `xtask` 子命令 12 个；历轮「六门禁」指本人选跑的六道 |

### F-27　裁定附录辛第 21 条：原条三要素两错一对，整体撤回；另立辛-37 至辛-40

> **历史证据边界：** 本节对 `permission_items`、`role_permission_grants` 与应用校验器的逐字实测同样只证明 **EXISTING** 形状；目标 schema 的目录外键、候选键与编码闭集由阶段 4 追补迁移及现行数据字典冻结。本节保留作缺口来源与裁定过程，不得作为“继续不建外键”的实施口径。

#### 结论零　辛-21 原条**整体撤回**——三个构成要素错了两个

**要素一「全卷各阶段 API 表的『权限要求』列」——全卷没有一张表的列名叫这个。**
「权限要求」全卷命中 **3** 处：`10-ar-ap-invoice.md:913` 是**小节标题**
（逐字「#### 5.7 权限要求」）、`13-clients-lowcode.md:634` 是**正文**
（逐字「下表的权限要求列写权限项名，具体角色映射由权限阶段承担。」）、
第三处是 `00c:4192`，**即本人的登记稿**。
而 13 的真实表头逐字是「| 方法与路径 | 请求 | 响应 data | 主要错误码 | 幂等语义 | **权限项** |」。
实际列名六种（权限／权限项／权限对象与动作／权限动作／权限与幂等／幂等与权限），
列序落在第 3、4、5、6 列都有；**09 一份根本没有这一列**（8 张表表头逐字全是
「| 方法与路径 | 说明 |」，权限串内联写在说明格里）。
**我把正文的转述当成了列名**——F-24 纪律五（引表必读表头）在我自己立了它之后又犯一次。

**要素二「无任何阶段被点名为生产者」——被 `03:1189` 直接证伪。** 逐字：

> 权限项名称形如 `platform.<resource>.<action>`，判定由阶段 4 的 ep-platform-authz 承担；
> **本阶段负责在每个端点上声明所需权限项并注册到权限项目录。**

我那张宽词表（权限项｜权限点｜权限码｜permission_item｜perm_code｜权限标识）**整句漏掉了它**，
原因是这句的关键词是「**权限项目录**」而不是「权限项」加宾语。
**这是纪律一的一种新形态：宽词表也会整句漏，只要关键词落在词表之外。**

**要素三「229 个串」——复算不出来，作废。**
本裁定用可复算口径重数（取 16 份计划、按 markdown 表切分、表头命中六种列名之一者判为 API 表、
取该列数据行、剥反引号后取点分或冒号串）：
**表 33 张、数据行 294、串出现 199 次、整串去重 126、词干去重 50。**
229 在任何试过的口径下都不成立。**此后一律引这四个数并附口径，229 作废。**

「无门禁承接」这半句成立（见结论三）。**三要素两错一对，原条不是修补，是撤回。**

#### 结论一　最硬的一条新事实：126 个串与 `permission_items.code` **不是同一种东西**

- `permission_items.code` 是**两段**：`04:137` 逐字「code text pk（形如 sales.sales_order）」；
  `crates/platform/authz/src/types.rs:350` 注释逐字「权限项编码形态：`<module>.<table>`
  小写下划线两段。」动作装在 `allowed_actions text[]`，其 CHECK 冻结**恰六个**大写值
  （VIEW／CREATE／UPDATE／SUBMIT／APPROVE／EXPORT）。
- 而那 126 个串是三段点分（99 个）或冒号形（27 个）。

**实测：126 个串与 9 行种子 code 的交集为空；50 个词干与 9 行种子 code 的交集也为空。**
过 DB CHECK 的 97 个，**过应用侧 `check_permission_item_shape` 的 0 个**。

**所以「补生产者」这个动作，在形态口径统一之前根本不可执行。**

#### 结论二　当事人换人，新立两条

**辛-37：引用不存在权限项的授予被静默丢弃，而本该报此事的错误码判的是形态不是目录。**
四条实测：

| # | 实测 |
|---|---|
| a | `role_permission_grants.permission_item_code` **无外键**指向 `permission_items.code`——该迁移全文只挂 `ck_role_permission_grants_code_len check (length(permission_item_code) between 1 and 128)` |
| b | 保存期只调形态检查（`applier.rs:150-151`），**不触库** |
| c | `PLATFORM.AUTHZ.PERMISSION_ITEM_UNKNOWN` 的唯一发射点抛的消息逐字「权限项编码 {code} **形态非法**，需为 `<module>.<table>` 小写下划线」——**判形态不判目录**；而 `docs/error-codes.md:79` 逐字「授权配置引用的权限项码**不在权限项目录内**」、`:163` 逐字「权限配置引用了**不存在的权限项**」 |
| d | 装载期 `snapshot_query.rs:54` 逐字 `join platform_authz.permission_items pi on pi.code = g.permission_item_code`——**INNER JOIN**，错码授予**静默丢掉**，最终落到与「确实没授权」同一个 `DenyReason::ObjectForbidden`，无日志、无独立指标 |

**一条恒不触发的目录判据（第一类缺陷）＋一个错了不会当场报错的错（第三类缺陷）。**

**辛-38：权限项编码形态四处口径不一致。**
`03:1189` 三段 ／ `04:137` 两段 ／ DDL 正则 `(\.[a-z][a-z0-9_]*)+` ≥两段且不收连字符 ／
`types.rs:350` 及其实现恰两段。实测后果见结论一。

#### 结论三　「谁来生产」按通则第六条**降档为观察项**，不写没有承接的要求

25 个门禁位（`xtask` 12 个子命令 ＋ `db/checks/` 13 个编号项）逐个为否。
**xtask 全部源码只读 `00b` 与 `01` 两份计划——没有任何门禁读 03 到 14 的 API 表。**

而 `13_unpoliced_registry.sql` 之所以能承接，是因为期望名册由 `pg_catalog` **现算**、
两侧同域可比；**权限项的期望名册是 33 张 markdown 表里的自然语言字段，
今天没有任何机器能从它生成名册。缺的不是一条规则，是一个跨侧比对面。**

按纪律「不得写没有承接的要求」，此项**只登记为观察**，等辛-38 形态统一后再评估可否机器化。

#### 结论四　更正 F-22 的一处内部矛盾：**相邻两行互斥，七轮无人发现**

F-22 残余 **E** 逐字：「生产方形态与交付物已不一致：04:139 与该表迁移**两处**逐字都写
「各业务模块在其阶段的 wiring 中登记」」
F-22 残余 **F** 逐字：「…而是因为两者的生产方在**同一句**迁移注释里被判给同一方」

**E 说两处、F 说同一句，写在同一份裁定的相邻两行里。**

据此改判：**辛-21 与辛-22 分别处置**，三条理由：

一、**那句话的主语只覆盖 6 个。** 种子迁移 `:13-14` 整句逐字（不从句中截起）
「**六类操作对应的业务对象**尚未登记 object_scope_bindings，其 object_type 暂取操作码自身
保持自洽，业务对象登记与其权限项细化归其所属阶段。」——主语是那六类操作对应的业务对象。
辛-22 的半径是阶段 5–14 全部业务对象。**用一个 6 元交集论证两个几十元集合必须同批，
是把交集当并集。**

二、**载重不对称。** 删掉那句，辛-22 另有**两处**独立指派（`04:139` 与该表迁移 `:5`，
两处逐字都写「各业务模块在其阶段的 wiring 中登记自己对象的范围锚列」）毫发无损；
辛-21 只剩 `03:1189`，**而那句把生产方指给「阶段 3 自己」，不是「所属阶段」——
两句指派的是不同的方。**

三、E 已经把 F 打掉了。

#### 结论五　但交集那 6 个是真的同批，单独拆出

实测两处种子的字面量作差：`permission_items` 种子 9 个 `object_type`，
`object_scope_bindings` 种子 3 个且是前者的真子集，**差集恰 6**：
`platform.contract_effective`、`platform.invoice_issue`、`platform.ledger_posting`、
`platform.payment`、`platform.period_close`、`platform.sensitive_export`。

而 `decider.rs` 的前置硬条件查的正是后者——**这 6 个权限项在判定面接线之后，
由构造保证恒返回 `ScopeBindingMissing`**。判据可当场判（两处 SQL 的字面量集合作差），
承接方现成（种子里补 6 行，或明写这 6 个不进判定链）。登记为**辛-39**。

#### 结论六　同族一条：`db/checks` 目录里既有非恒真的双向判据，也有**恒真的单向判据**

`db/checks/11_sensitive_field_encryption.sql` 的 `from` 只有
`platform_core.sensitive_field_registry` **一张表**，方向单一；
而该表**零种子行**（全仓无 `insert into platform_core.sensitive_field_registry`）。
**零行进、零行出、恒过。**

F-22 表扬了 `13_unpoliced_registry` 的双向形态，**没有指出同目录下的 11 是反例**。
登记为**辛-40**。

#### 本轮未做到的，不得算作已核

- `13_unpoliced_registry` 的双向性我只用 `from` 模式 grep 过，**未逐句核其 `not exists` 子查询**——
  结论六里「13 是双向」这半句标为**待核**，「11 是单向且恒过」这半句是实测。
- 核查另称「04 第 9 节 22 条退出条件里至少 3 条在空集上恒过」，属静态推断、**未跑库**，标为待核。
- 126 个串的形态分类（三段 99／冒号 27）取自核查脚本，本裁定未逐串复核。

#### 撤销、新立与更正

| 编号 | 处置 |
|---|---|
| 辛-21 | **撤销**：列名转述、229 无口径、「无任何阶段被点名」被 `03:1189` 证伪 |
| 辛-37 | **新立**：授予引用不存在权限项被静默丢弃；`PERMISSION_ITEM_UNKNOWN` 判形态不判目录，与 error-codes 的描述不符 |
| 辛-38 | **新立**：权限项编码形态四处口径不一致，126 个串过应用侧校验 0 个 |
| 辛-39 | **新立**：6 个权限项的 `object_type` 不在 `object_scope_bindings` 内，接线后由构造保证恒 Deny |
| 辛-40 | **新立**：`db/checks/11` 单向且被测表零行，恒过 |
| F-22 残余 F | **更正**：与同节残余 E 互斥；辛-21 与辛-22 改为分别处置 |
| 计数口径 | 229 作废，改引「126 整串／50 词干／199 出现／294 行」并附本裁定口径 |

### F-28　全面核实：21 条在册条目逐条复核，两处动摇本人已落卷的裁定

本节不是新裁定，是一次**全面核实**的结论。口径：00c 内互异辛编号 40 个，
扣除已处置并回写的辛-1 至辛-14a（17 个）与已撤销的辛-21／29／30（3 个），得**在册 21 条**。
**辛-35、辛-36 从未存在**（00c 零命中、全仓零命中、`git log -S` 零命中）——
F-27 从辛-34 跳至辛-37，属本人编号错误，在此说明。

#### 结论零　两处动摇本人已落卷裁定的错误，先认

**（一）F-21 结论三：我判「这句不存在」的那句，就在同一行的后半。**

F-21 结论三逐字写过：「同时撤掉核查引的一句原文：核查以 13:743『客户端必须容忍……
`item_kind` 出现未知取值』支撑『十五是恢复值不是冻结上限』。**实测这句不存在**——
13:743 讲的是端点主版本。」

**实测 `13-clients-lowcode.md:743` 全行**：

> 本阶段全部端点为 v1。自定义对象端点的形状随对象定义变化，但路径与封套不变；
> 新增字段属于向后兼容变更，不升主版本；删除或重命名字段由配置回退承担，
> 不通过 API 版本表达。**客户端必须容忍 `capability_values` 与 `item_kind`
> 出现未知取值并按未知降级处理，照抄基线第 5.6 节。**

**我只读了这一行的前半（前半确实讲主版本），就判整句不存在。**
句中截断（纪律五）与只读支持自己一侧（纪律一）**两条同时犯**。

**后果比一处引证错更坏**：我用一个**不实的「实测」**去撤销核查提供的一条**正确证据**，
并据此给它记了一处错误。核查用那句论证「十五是恢复值不是冻结上限」——**那个论点是对的**。
**F-21 主结论（判乙、规则自立第 16 类）不动摇**，但该处撤销与所记的错误一并撤回，
恢复核查的引证。

**（二）F-15 的改写指令今天不可执行。**
F-15 把落点写成「阶段 3 计划**第 3.5.4 节**」，而被改的那句实际在 `03-platform-kernel.md:819`
（逐字「三处取值各有理由。`audit_events` 无任何更新路径，取 `APPEND_ONLY` 与空白名单。…」）；
`03:1237` 的 §3.5.4 逐字是「#### 3.5.4 Outbox 与死信」，**不含被改文字**。
改写指令的落点须改为 `03:819`。F-15 主结论不动摇。

#### 结论一　一处我不接受核查的结论，标为待核

核查称 F-16 结论表那一行「A.3 的 80% 阈值为 744 GiB」把基数从 A.3 原文的「本节容量下限
2 TB」换成了交付机可寻址容量 930 GiB，并称据此「已达 51%、逼近 80%」这一判断不成立。

> **【已更正，勿依此段】** 本段下述「未能自行复核」的结论**是错的**：`80%` 在规格里命中两处，
> 第二处 `1825` 就是 A.3 那一段，当时就在 grep 结果里而本人只读了该长行的可见前缀。
> 正确结论见 **F-31 结论零与结论一**（A.3 的 80% 基数是「本节容量下限」＝2 TB）与 **F-32**。

**本裁定未能自行复核**：在规格全文 grep `80%`，命中处为第 1324 行的**覆盖率**条款，
未命中容量阈值条款。**按纪律，不得因核查说了就写进裁定。**
**标为待核，下一轮专核 A.3 的容量阈值基数。**

#### 结论二　21 条分档（六档，合计 21 条）

| 档 | 条数 | 编号 |
|---|---|---|
| 成立 | **8** | 辛-24、27、28、32、33、37、38、40 |
| 成立但需收窄 | **7** | 辛-15、22、25、26、31、34、39 |
| 夸大 | **1** | 辛-19 |
| 证伪 | **2** | 辛-17、20 |
| 排期项应撤 | **2** | 辛-16、23 |
| 待核 | **1** | 辛-18 |

**逐条要点（只列判定非「成立」的 13 条）：**

- **辛-17 证伪。** 承重句「无可区分性判据」为假：`11:230` 有列级 CHECK 判别子、
  `11:235`／`11:244` 按 `object_kind` 逐变体 JSON Schema、`11:484` 同一校验在提交发布闸门上、
  `11:760` 有退出条件。**四条互相独立且都在必经路径上。** 四要件全勾。
- **辛-20 证伪。** 落点存在**且已被机器读**：`00b:789` 逐字「本表是上一条通则的**机械承接方**……
  多一条或少一条均判违反并以退出码 1 结束」，`xtask/src/archcheck/mod.rs:27` 的 `DELEGATED`
  两项与表内两行相等。另订正本人引证 `00b:783` → `00b:782`。
- **辛-19 夸大。**「零文法」当场为假：`crates/platform/notify/src/template.rs:52` 逐字
  「占位符形如 `{name}`」，`:16-22` 有三个 `RenderError` 变体，
  **且该文件早于本人登记一天入仓**。真正的残余是另一件（`03:586` 要求「模板变量的可用集合
  由模板声明」而表 16 无承载列），**须另立换当事人，不得记在本条名下**。
- **辛-16、辛-23 排期项应撤。** 辛-16 已由 F-21 判乙处置，条目结案；
  其中「无录入端点」一项撤回——`13:680` 通用配置包录入在场，F-21 结论七已正面裁过。
  辛-23 四要件全勾，改写为验收判据。
- **辛-18 待核。**「直通 DDL」撤回（无逐字依据，属推断）；那句写在表的**说明**列；
  今天 `custom_fields` 未建表、建字段端点请求体未点名该列——**无人能给它赋值，无可复算后果**。
- **辛-15、22、25、26、31、34、39 成立但需收窄**，逐条收窄点见下节账目。
  其中**辛-22、25、26、39 四条今日零后果**，后果发生条件同为
  「授权判定面接入第一个生产调用点」——**排期时应作为一个批次而非四件事**。

#### 结论三　账目须改的 17 项，其中两项越晚做越贵

**最贵的两项（已生效裁定未落到计划文件）：**

**其一**，`00-overview.md:203` 今天仍逐字「ConfigItemApplier 的**九个** item_kind 实现」
「**六个**自定义类归阶段 13b」「ItemKind **十五项**」；`00-overview.md:65` 仍逐字
「item_kind 的 CHECK 一次建齐十五项」。F-21 结论三已裁定改为十个／七个／十六项。
**不改的后果可复算：按现行文字施工会建成十五值 CHECK，事后为落实 F-21 必须改一条已应用迁移。**

**其二**，`13-clients-lowcode.md:242` 仍逐字「item_kind 取值封闭为 15 项」；
`13:1076` 末句仍是「不改本阶段任何表」，F-21 结论四已判该句为假。

**其余 15 项**分四类：撤销与已裁标记未回写 7 项（辛-21／29／30 的登记行今天仍以
「须走裁定」在册）；编号与体例 4 项——其中一项是**三张表丢了「类」列**，
致**辛-34、37、38、39、40 五条无类别字段，在任何按类别的盘点里隐形**；
计数与悬空引用 4 项——其中 `00c:6182` 逐字「其中第 15 至 18 项由**补裁壬**追加」，
而全卷 `壬` 仅此一处、**无补裁壬正文**。

#### 结论四　承重引证抽验：81 处中不通过 10 处

除结论零已认的两处外，另 8 处为口径或出处问题，主结论均不动摇，逐条已记入核实底稿。
其中 F-14「全卷 `FAILED` 只在 A-06 那一行 CHECK 里出现过」是**无限定全称**，
须改为「`recon_runs.status` 的取值域内」；F-16「全仓写死 `C:\EP` 的行共 28 处」
按行计 27、按出现次数计 29，**无一为 28**，须择一并附口径。

#### 结论五　一条十轮无人管的欠账

**辛-7a**（法规基准日期标注）在 F-18 至 F-27 **十轮零提及**，全卷仅 3 处命中，
**无任何一处认领**。须在下一轮明写处置或纳入排期。

#### 本次核实自身没做到的

- 结论一那条（F-16 的 80% 基数）**未能自行复核**，标为待核。
- 全部结论为**静态读码与文本比对**，未跑库、未执行任何 SQL 断言。
- 辛-25 的另一半（`crates/platform/authz/` 其余九个源文件）**自 F-24 起三轮未核**。
- H-07「改塞 `port::ipc` 那条路不通」是否已被 `port/sensitive.rs` 证伪，
  F-26 自标待核，**本轮仍未处置**。

#### 下一步建议的三件

1. **落账目 B 类两项**（F-21 到计划文件），理由是它随时间变贵，且已可复算。
2. **裁辛-39**——判据可当场判（两处种子字面量作差）、承接方现成，能立刻收口；
   并把辛-22、25、26 与它作为同一批次处理。
3. **专核 F-16 的 80% 基数**（结论一）与 **辛-7a**（结论五）。

### F-29　裁定附录辛第 39 条：当事人整体更换，并立第六条取证纪律

#### 结论零　撤回登记原文两句，并纠正本人自查里的一半

**（一）撤回「由构造保证恒返回 `ScopeBindingMissing`」。** 前件在卷内无依据。
该拒绝只在端点把这六个串**本身当 `object_type`** 传给判定器时触发
（`decider.rs:80-84` 的前置硬条件），而计划两处守卫条件写的都是**业务对象**：
`04:377` 逐字「发起人对**该对象**持有 SUBMIT 权限」、`04:503` 逐字「**该对象**的 Submit 权限」；
数据模型上二者是**两列**——`high_risk_requests` 的 `operation_type` 与 `subject_object_type`。
**我把一条计划从未画过的分支当成了唯一分支。**

**（二）我立案后的自查只对了一半。** 我说「那 6 个是权限项码不是对象类型」——
按码用确有两处（`identity/src/config.rs:167-176` 的 `HIGH_RISK_PERMISSION_ITEMS`
与 `user_grants.rs:47-57` 的 MFA 触发链）。**但由此推出的「`object_type` 这一列没有读者」是错的：**
`snapshot_query.rs:51-56` 逐字 `select r.code, pi.object_type, g.action …
join platform_authz.permission_items pi on pi.code = g.permission_item_code`——
**该列是判定快照授予键的第二元，是一条已接线的活读路径。**
同一个字符串在两条互不相干的链上各有身份，我推得太快。

#### 结论一　换上的当事人

**授予写库按 `permission_item_code`，判定读快照按 `object_type`，两者只由一次内联接换键；
六类高风险权限项因 `object_type` 取操作码自身，其授予行在快照里生成一批
任何合乎计划的调用都命中不到的键。**

三处逐字：换键处 `snapshot_query.rs:51-56`（上引）；落键处 `decider.rs:131-140` 逐字
「阶段二之二：角色 × 对象类型的授予集合包含所求动作」；
占位来源（整句，不从句中截起）`V20261012112000…:12-14` 逐字
「六类高风险操作各 1 行共 6 行，allowed_actions 恰好 SUBMIT 一个动作……
六类操作对应的业务对象尚未登记 object_scope_bindings，
**其 object_type 暂取操作码自身保持自洽**，业务对象登记与其权限项细化归其所属阶段。」

#### 结论二　后果分支写清，不得再用无理由的「恒 Deny」

- **(a) 端点传操作码本身** → 前置硬条件 `ScopeBindingMissing`。
- **(b) 端点按计划传业务对象类型** → 该业务对象今天未登记，同样落 `ScopeBindingMissing`，
  **但此时当事人是辛-22，不是本条**；待该业务对象登记后，授予键是 `(角色, "platform.payment")`
  而查的是 `(角色, 业务对象类型)`，落空走 `ObjectForbidden`。

**(b) 的后半段是推断**——卷内没有任何一处写出将来 `decide` 的实参形态，
`04:494` 只写「对给定 (user_id, object_type, object_id, action) 返回判定结论」，**由调用方传入**。

#### 结论三　F-27 给的第一条承接路径**不可执行，须删**

F-27 结论五逐字「承接方现成（种子里补 6 行，或明写这 6 个不进判定链）」。
**补 6 行走不通**：`object_scope_bindings` 的 `schema_name text not null` 与
`table_name text not null`，为 `platform.payment` 补一行须填出一个 schema 与表名，
而 `contract_effective`、`invoice_issue`、`ledger_posting`、`period_close` 是**动作名，
不占表名槽位**（三行种子实测都指向已建表）。

正确目标态由迁移自陈：**由所属阶段把这六行的 `object_type` 细化为真实业务对象类型，
与该业务对象的 binding 登记同批。**

#### 结论四　今天连授予行都没有，后果面比登记时更远

`V20261012112500…admin_duty_roles.sql` 的授予数组实测只含
`platform.user_accounts`、`platform.roles`、`platform.high_risk_requests` **各三次**——
**六类高风险项零授予行**。今天不但判定面零调用，连那批「命中不到的键」都还没生成。

#### 结论五　无承接，按通则第六条降档

`db/checks` 十三项（`NUMBERED_CHECKS` 全集）无一可承接，`grep -rl` 在该目录零命中。
**不提「新增第 14 项」作为要求**，三条理由：三处冻结须同批改；执行方无自动调用方（辛-27 在册）；
**更要紧的是断言会写不对目标态**——按结论三，正确动作是改那六行的 `object_type`，
而不是给 `object_scope_bindings` 补 6 行，照登记原文写断言会**把一个不该做的动作判为合规要求**。

处置：**辛-39 由「缺陷」降为「授权判定面接入时的前置条件」**，挂在辛-22 已定的同批之下，
在该阶段之前既不计入通过也不计入违反，不新增门禁项。

#### 结论六　F-28 的合批断言**不成立**，四条改两条

F-28 两处逐字称「辛-22、25、26、39 四条今日零后果，后果发生条件同为
『授权判定面接入第一个生产调用点』——排期时应作为一个批次」。**逐条复核后须改：**

- **辛-22 与辛-39 留在批内**——共用同一次联接、同一对表、同一批种子行，
  处置动作在同一批迁移里发生。
- **辛-25 出批**——其后果发生点在**字段投影**不在判定器（`decider.rs:49` 逐字
  「阶段四在端点侧经 FieldProjector 承接」），解锁物是 `04:495` 那条今天未注册的路由。
- **辛-26 出批，且「今日零后果」这一档也要改**——其现象今天就在线，
  而**这个形态正是本阶段退出条件所要的**（`04:739` 第 17 条）；
  解锁物在阶段 13 且排在判定**之前**（`13:513` 逐字「能力闸中间件在授权判定之前执行」），
  与「判定面接入」无先后依赖。

据此 `00c` 两处合批断言与「下一步建议」第 2 条一并改写。

#### 结论七　立**第六条取证纪律**：F-27 的错不属现有五条

逐条对照后确认属**新形态**：F-27 逐字取证覆盖了因果链的**两个端点**（A 列取值、B 列查表），
却把「两张表的同名列」当成了连接两端的**边**。同名是命名巧合，不是取值域相等。

> **六、断言一条因果边，逐字依据必须落在那条边上，不得以两个端点的逐字依据代替。**
> 凡结论含「恒 X」「由构造保证」「接线后必然」「A 导致 B」，须同时给出三份逐字：
> (a) 取值产生处；(b) 取值消费处；**(c) 把 (a) 的值送进 (b) 的那一处**。
> (c) 拿不出来时，结论只能写成条件句并整条标为推断，不得与 (a)(b) 同级书写，也不得据此定档。
>
> **6.1**：两张表的**同名列不构成 (c)**。须另给跨表咬合的逐字——外键、跨表 CHECK、
> 或同一条 SQL 里的 join／同一个函数实参——三者皆无时，须写明「两列今日互不咬合」，
> 而不是写「差集为 N」。
>
> **6.2**：(c) 若是函数调用，须给出调用点行号并注明是否在 `#[cfg(test)]` 之后；
> 全部调用点在测试内则 (c) 视为不存在。

**辛-39 的真实边其实存在**（`snapshot_query.rs:54` 那次 join），**F-27 恰好没找到它，
却找到了一条不存在的边**。按第六条办，F-27 当轮就会写对当事人。

#### 结论八　普查：半径比登记的大

`object_type text` 实测在 **6 张表**：`object_scope_bindings`、`permission_items`、
`field_permissions`、`access_policies`、`user_scope_grants`、`high_risk_requests`。
**六者之间零外键、零跨表 CHECK**——`platform_authz` 全目录的外键只有两条，
指向 `platform_core.departments` 与 `positions`。
而 `snapshot_query.rs` 有**四条独立读路**（grants 经 join、policies 直取、fields 直取、
bindings 直取）。**辛-39 只点了其中两张表。**

登记为**辛-41**：六张表共用 `object_type` 列名而无任何跨表咬合，四条读路各自取值。

#### 本轮未做到的

- 同族普查的 `schema_name` 与 `table_name` 两族（各 4 张表）**未核**。
- 结论二 (b) 的后半段是推断，已标明。
- 全部结论为静态读码与文本比对，**未跑库**。

### F-30　辛-22 定谳：本人自查被推翻，档位按第六条改判，另认一处伪造引文

#### 结论零　先认两处本人的错

**（一）我在本轮任务书里把复述当逐字引文。** 我写「F-28 逐字称『04:139 的指派在 05–14
十份计划中**零交付物行承接**（六种说法**全部零命中**）』」——实测
`grep "零交付物行\|六种说法"` 全卷命中 **0**。**那句话是我的复述，不是 F-28 的原文，
而我给它加了引号。**

这与 F-21 那次「实测这句不存在」是**同一类错的两个方向**：那次我否认了一句真实存在的原文，
这次我造出了一句不存在的原文。**两次都发生在引证环节，都改变了下游判断。**

**（二）我立案后的自查——「07:49 与 10:915 是对本表的认领」——被推翻。**
三条硬证：

1. **07 那一列的取值是「对象:动作」二元组。** 07:624 表头逐字
   「| 方法与路径 | 说明 | **权限对象与动作** | 主要错误码 |」，07:626 该列取值
   `procure.purchase_requisition:read`。而 `object_scope_bindings` 的 DDL
   **无任何动作列**（`allowed_actions` 命中 **0**），`permission_items` 有
   （命中 **4**）。**带动作的注册落不到本表。**
2. **10:141 逐字「注册 14 个对象类型**与 12 个动作**」**——同样与动作配对。
3. **10:915 那 14 个串是单数化的**（`invoice.invoice_application`、`finance.cash_account`），
   而本表三行种子是「schema 前缀＋真实表名（复数）」且 `schema_name`／`table_name` 两列填满；
   14 个里 `invoice.purchase_invoice_ledger`、`finance.advance_entry`、`finance.reconciliation`
   **三个根本填不出表名**——与 F-29 结论三对那六个占位串的判法同形。

**故 07 与 10 认领的是 `permission_items` 一侧，不是本表。我的自查作废。**

#### 结论一　F-28 那六个词的「零命中」**没有数错**，这是三次以来第一次

实测 `范围锚`／`范围绑定`／`scope_binding`／`ScopeBinding`／`锚列`／`登记自己对象`
在 05–14 十份中**各为 0**，`-i` 复算不变。
错的只是**由六个词外推到「零承接」这一步**——宽词表下确有 3 处形式上的交付物行／散文认领，
只是它们指向另一张表。**结论方向与结论本身不变，理据要换。**

#### 结论二　换上的理据比原来的强：十份计划写过 65 次「向登记表回填」，唯独没有这一张

实测 05–14 十份计划里三张同族登记表的命中：
`unpoliced_table_registry` **26** 次、`append_only_registry` **23** 次、
`sensitive_field_registry` **16** 次，合计 **65** 次；
**`object_scope_bindings` 命中 0 次。**

**十份计划完全知道怎么写一条「向某登记表回填 N 行」的交付物行——为三张表写了 65 次——
唯独没给本表写过一条。** 这比「六个词零命中」硬得多，因为它排除了「措辞不同」这个解释。

#### 结论三　一处我登记时完全没写的缺口：**阶段 4 自己的交付物也是零**

04:139 逐字「……本阶段只登记 platform 自身的三个对象类型……**并提供登记接口**，
业务对象的登记在其所属阶段完成。」

**那个「登记接口」不存在：** `crates/platform/authz/src/applier.rs` 对本表命中 **0**
（`AuthzConfigWriteStore` 的六个方法无一触及它，三个 applier 里无绑定 applier）；
全仓 `insert into platform_authz.object_scope_bindings` 命中 **1** 处，
就是**建表迁移自身**；无 `update`、无 `delete`、无端点。

**这一条是阶段 4 自己的交付物缺位，与 05–14 认不认领无关，也不受第六条 6.2 约束——
它是今天就能判的实体缺陷。**

#### 结论四　第六条 6.2 首次自适用：后果那一半必须写成条件句

`crates/platform/authz/src/decider.rs:158` 是 `#[cfg(test)]`，
而六处 `.decide(` 在 251／263／288／300／307／319——**全部在其后**。
按 6.2，**(c) 视为不存在**。

故辛-22 的后果面**改写为条件句，整条标推断**：

> **（推断）** 本表今天只有种子迁移写入的 **3** 行。**若**将来把 `AccessDecider` 接到
> 任一生产调用点，**则**凡 `object_type` 不在这 3 行之内的调用，在 `decider.rs:80-84`
> 的前置硬条件处返回 `Deny(ScopeBindingMissing)`。**前件今日不成立，本条今日不产生
> 可复算后果。**

**不得再写「未登记一律拒绝」这种陈述句**——那是把计划的规范句当成了今日的运行事实。

#### 结论五　档位与处置

辛-22 由「成立但需收窄」改为**两部分分列**：

| 部分 | 档 | 依据 |
|---|---|---|
| 阶段 4 的「登记接口」零交付 | **成立**（今日可判） | 结论三 |
| 「未登记则拒绝」的后果面 | **条件句，整条推断，不计入通过也不计入违反** | 结论四，第六条 6.2 |

**不撤号**（候选丁不成立）：结论三那一半今天就能判，不是排期项。

**F-22 结论五第一条须撤其括号外半句。** 它逐字写「一、六行差集补齐（或明写那六个
`object_type` 是占位、不进判定链）」，而 F-29 结论三已判「补 6 行走不通」——
**只留括号内半句。**

#### 结论六　不新增门禁项

`db/checks` 十三项无一可承接：本表的期望名册是「哪些业务对象会被端点暴露」，
这不在 `pg_catalog` 内，**两侧不同域**（F-27 已立此判据）。按纪律「不得写没有承接的要求」，
本条不附带任何限期动作。

#### 结论七　F-29 的合批依据须复核

F-29 结论六称辛-22 与辛-39「共用同一次联接、同一对表、同一批种子行」。
**按本轮结论三，辛-22 今天活的那一半（阶段 4 的登记接口零交付）与辛-39 无关**——
后者的当事人是 `permission_items` 与快照授予键的换键。
**合批依据只覆盖辛-22 的条件句那一半，不覆盖结论三那一半。** 据此改写。

#### 本轮未做到的

- 结论零（一）里「F-28 是否真写过近似的话」只复算了那六个词与两个短语，
  **未通读 F-28 全文比对**，标为待核。
- 07／10 认领指向 `permission_items` 一侧，此判定为**推断**——
  卷内无一处写出「对象类型注册」落哪张表。
- 全部结论为静态读码与文本比对，**未跑库**。

### F-31　专核两件：F-16 的 80% 基数（本人换了基数，且掩掉一处下限违反）与辛-7a 的真实状态

#### 结论零　先认第四次同类错

F-28 结论一说「在规格全文 grep `80%`，命中处为第 1324 行的覆盖率条款，**本裁定未能自行复核**」。
**实测 `80%` 在规格里命中两处：1324 与 1825，而 1825 就是 A.3 那一段。
它当时就在我的 grep 结果里，我只看了那一行被截断的前缀（讲规模取值）就把它略过了。**

这是**第四次同一类错**：F-21 只读 `13:743` 前半就判整句不存在；F-30 把复述当逐字引；
F-28 这次是把一条命中当成了不相关。**三次的机理相同——只读一行的可见部分就下结论。**

#### 结论一　F-16 那一行换了基数，核查说对了

A.3 逐字：「交付后由客户运维按第 15.3 章对该服务器本地磁盘水位设置阈值告警并每年复核一次容量：
**实测占用达到本节容量下限的 80% 时**，实施扩容或按第 12.4 章的处置流程发起物理删除……」

**基数是「本节容量下限」，而 A.3 逐字给出该下限：「本节数据集对应的服务器本地可用磁盘容量
下限为 2 TB」。**

- A.3 口径：80% × 2 TB = **1638 GiB**
- F-16 写的：**744 GiB** = 80% × 930 GiB ← **换成了交付机可寻址容量**
- 同一个 470 GiB：占 2 TB 为 **23%**，占 930 GiB 为 **51%**

**F-16 结论表那一行「A.3 的 80% 阈值为 744 GiB」与其后「已达 51%、逼近 80%」的框架，
是换基数的产物，撤回。**

#### 结论二　但换基数掩掉的东西比算错百分比严重得多

A.3 另有一句，F-16 通篇没有引：

> **「本节下限与该实测合计值取较大值，作为交付客户的磁盘规格下限。」**

即**交付客户的磁盘规格下限 = max(2 TB, 实测合计) ≥ 2 TB**。
而交付机可寻址 **930 GiB < 2 TB**。

**F-16 把一处「交付机不满足规格给的磁盘下限」重述成了一道「余量还有一半」的算术题。**
换基数不是算错一个数，是把违反下限改写成了留有余地。

**没有合法的下调口子**：A.3 逐字「部署前由实施方按客户实际数据量完成容量核算，
**实际数据量超出本节取值时**按同一构成重算容量下限并写入实施方案，
**任何情况下不下调本节数据集**」——该句只授权**向上**重算，
且明禁下调数据集；全节无一句授权按客户规模下调那 2 TB 下限。

#### 结论三　处置：这一条要交使用方定，不由本裁定替代

使用方已表态两条（本卷在案）：「硬盘空间无所谓，可以再加」与「当前只有一个 HDD，
以后不行才会加」。**但 A.3 把 2 TB 写成的是交付前置条件，不是「不够再加」。**
两者相抵，需使用方在下列三条里选一条，本裁定不替选：

| 选项 | 后果 |
|---|---|
| 甲　交付前把盘加到 ≥2 TB | 满足 A.3，无须任何豁免 |
| 乙　按实际数据量重算下限并写实施方案 | **须先改规格 A.3**——现行原文只授权向上重算，且「任何情况下不下调本节数据集」 |
| 丙　按现状交付并书面记录偏离 | 该偏离落在 A.3 的交付前置条件上，须写入交付说明与认证报告 |

**在使用方表态之前，F-16 的容量结论只保留「实测占用约 470 GiB」这一事实，
撤回其「51%／逼近 80%／余量充足」的全部判断。**

#### 结论四　辛-7a 的真实状态：不是「无人认领」，是**卡在一个我从没提出的问题上**

F-28 结论五称辛-7a「十轮零提及、**无任何一处认领**」。**后半句不准确。**

F-17 结论五逐字（`00c:3567`、`3581`）：「法规标注那条义务**单独立，不随本裁定延期**
（新登记辛-7a）……故单独登记为**辛-7a**，**交产品负责人定首版要不要做**，
不并入本条主结论。」

**它有明确归属——归使用方决定。十轮没动，是因为我一直没把这个问题提给使用方，
不是因为无人认领。这是我的遗漏，不是卷内的缺口。**

处置：辛-7a 的档由「须走裁定」改为**「待使用方表态」**，并在本轮提出（见下）。

#### 本轮未做到的

- 结论二关于「无合法下调口子」是对 A.3 全节的通读结论，**未通读第 13.1 章**（单机形态那一章）
  是否另有下调授权，标为待核。
- 未跑库；全部为静态读码与文本比对。

### F-32　磁盘下限偏离的处置（使用方表态：以后会加，当前加不了）

使用方对 F-31 结论三的三条选项表态：**「磁盘以后会加，但是当前加不了」**。
该表态落在选项丙（按现状交付并书面记录偏离），且带一条已定的补救路径。本裁定据此处置。

#### 结论一　偏离的准确表述

**交付机可寻址 930 GiB，低于规格 A.3 逐字给出的交付客户磁盘规格下限。**
A.3 那句逐字是「本节下限与该实测合计值取较大值，作为交付客户的磁盘规格下限」，
而「本节下限」逐字为「本节数据集对应的服务器本地可用磁盘容量下限为 2 TB」。

**缺口 1118 GiB。** 这是一处**交付前置条件**上的偏离，不是余量不足。

#### 结论二　偏离引入了一处第二类缺陷：A.3 的容量告警在这台机器上**触发不了**

A.3 逐字「交付后由客户运维按第 15.3 章对该服务器本地磁盘水位设置阈值告警并每年复核一次
容量：**实测占用达到本节容量下限的 80% 时**，实施扩容或按第 12.4 章的处置流程发起物理删除，
二者均未执行时把该部署的容量暴露写入部署记录并书面告知客户。」

算术：

| 项 | 取值 |
|---|---|
| A.3 告警阈值 = 80% × 2 TB | **1638 GiB** |
| 交付机整块盘可寻址 | **930 GiB** |
| 阈值减整盘 | **+708 GiB** |

**阈值高于整块盘 708 GiB，该告警在本机永不触发。**
连带地，A.3 为「二者均未执行」预备的那条兜底（写入部署记录并书面告知）也永不触发。
**这是本卷第二类缺陷——取不到的取值——由本偏离直接引入。**

#### 结论三　替代阈值：**744 GiB**。F-16 那个数是对的，但当时的理由是错的

按本机重定的容量复核阈值取 **80% × 930 GiB = 744 GiB**。

**这正是 F-16 结论表写下的数。** 但 F-16 把它标为「A.3 的 80% 阈值」——**那是错的**
（A.3 的阈值是 1638 GiB，见 F-31 结论一）。
**订正：744 GiB 不是 A.3 的阈值，是本偏离下按本机重定的替代阈值。数对，名分错，现予更名。**

#### 结论四　替代阈值**不能**由既有的 100 GiB 阻断闸门顶替

F-16 已定 `EP__PLATFORM__FILE__FREE_SPACE_MIN_BYTES` 取 100 GiB、阻断级
（即占用达 830 GiB 时 `attachment-store-ready` 自检失败）。**它不是 A.3 那件事的替代品**，
两条理由：

一、**意图不同。** A.3 的 80% 是**容量复核的提前量**，其处置逐字是「实施扩容或……发起物理删除」，
前提是还有时间行动；100 GiB 那道是**最后一刻的阻断**，触发时已经停机。
二、**间距不足。** 744 GiB 到 830 GiB 之间只有 86 GiB，按使用方给的 50 GB／年附件增量，
**不足两年**；而 A.3 要求的复核周期逐字是「每年复核一次」。

**故两道并存：744 GiB 为复核触发线（需人为设置水位告警），830 GiB 为阻断线（已在代码内）。**

#### 结论五　补救路径按使用方表态预先定死，不留「不行了再说」

A.3 的原生告警已失效（结论二），**故加盘的触发条件必须由本裁定预先写死，
不能依赖运行期的自然提醒**：

> **实测占用达到 744 GiB 时，实施加盘。** 该动作使用方已预先同意（表态逐字
> 「磁盘以后会加」），本裁定只定触发点，不再另行请示。
> 加盘目标按 A.3 原文的构成，补足至 ≥2 TB 后本偏离自动消灭。

#### 结论六　须写入的三处，缺一不可

本偏离落在交付前置条件上，按 A.3 自身的处置结构，须写入：

1. **部署记录**——A.3 逐字要求「把该部署的容量暴露写入部署记录」；
2. **交付说明**——本部署为内部使用，A.3 的「书面告知客户」在此即向使用方报备，
   与 F-19 结论五的两句披露并列；
3. **认证报告**——A.3 逐字「本节下限与该实测合计值取较大值，作为交付客户的磁盘规格下限」，
   该下限与实际交付规格的差额须随认证报告记录。

三处的内容同一：**交付规格 930 GiB、A.3 下限 2 TB、缺口 1118 GiB、
替代复核阈值 744 GiB、阻断线 830 GiB、加盘触发点 744 GiB。**

#### 结论七　对 F-16 的连带更正

F-16 结论表那一行及其后「已达 51%、逼近 80%、余量充足」的判断，**F-31 已撤回**。
本裁定补上撤回后的替代表述：

> 交付机可寻址 930 GiB，实测占用约 470 GiB（占本机 51%）。
> **本机低于 A.3 的 2 TB 交付下限 1118 GiB，属已记录的偏离（F-32）。**
> 复核触发线 744 GiB，阻断线 830 GiB，触及复核线即加盘。

#### 本轮未做到的

- 结论四第二条的「不足两年」用的是使用方给的 50 GB／年附件增量，
  **未计入事务数据库与归档的增长**，故是**乐观估计**，标为推断。
- F-31 结论未做到项仍在：「A.3 无合法下调口子」未通读第 13.1 章复核，标为待核。

### F-33　裁定辛-7a（使用方已表态「做」）：定形态、渲染点与归属

第一问使用方已答「要」。本裁定只定第二问，不再论证要不要做。

#### 结论零　「做」触到一个前提：**照规格原文实现，这条标注永不渲染**

规格第 3.5 章那条义务挂在一句引导句之下——`spec:182` 逐字「**维护订阅到期后的行为：**」，
`spec:185`、`spec:186` 是它下面的两个条目；`spec:1657` 逐字
「……**未持有有效维护订阅的客户**按第 3.5 章在界面与单据上显著标注所用规则版本、
生效日期与申报偏差风险。」

**两处触发条件都挂在维护订阅状态上。** 而 F-17 已裁永久授权与维护订阅两项首版不交付，
本部署为内部使用、无维护订阅概念。**故照规格原文实现，该分支在本部署恒假，标注永不渲染。**

**这是本卷第一类缺陷「恒真的判据」的镜像形态：恒假的触发。未覆盖 ≠ 通过，恒假亦然。**

据 F-17 结论五已立的理由（逐字「财税法规改了而系统没升级，开票与申报的口径就是旧的；
自有企业内部使用同样要知道自己算的是哪一版法规。删掉商业那层不该把这条实质义务一起带走」），
**触发条件改写为无条件常显**。

**这是改写规格原文，不是解释规格原文，在此明标。** 同批请求规格修订：
在 `spec:186` 那一条上加一句「本条不受维护订阅状态约束」，
与 F-17 结论三「第 3.5 章不删全节，只在节首加一句首版不适用的定语」并列处理。

#### 结论一　「所用版本」不必新造——规格同章自己给了定义

`spec:185` 逐字「平台不再接收新的功能版本与安全补丁，已交付功能所依据的法规口径
**冻结在当前已安装版本**。」——**第 3.5 章把「所用版本」直接定义成「当前已安装版本」**，
即平台制品版本。

而 `BuildInfo` 已落码（`crates/platform/runtime/src/process.rs`，含 `version`、
`git_commit`、`source_date_epoch` 等字段）。**这一半满足第 3.5 章，零新增承载物。**

`spec:179` 取消的逐字是「带生效日期的**独立法规规则包**、按领域订阅的规则包与追溯重算」——
取消的是那个**独立制品**，不是「版本」这个概念。两句不冲突。

#### 结论二　「法规基准日期」这一半没有自动取值源，只能是交付级人工声明

四个可能的自动源逐个否掉：制品版本（税率可经配置发布通道增删改、不随制品走，
用它冒充会给出**恒不报错的错值**）；`schema_history`（结构版本）；
`client_releases`（只覆盖四端客户端制品）；`tax_rate_options.updated_at`
（语义是**有人编辑那一行的时刻**，与法规生效基准日可差任意长）。

**故认定：「法规基准日期」在首版不是运行期数据，是一次交付级的人工声明。**
由本裁定新造一个编译期声明值承载。**这是裁定补规格空缺，来源明标。**

#### 结论三　最小形态：编译期声明值 ＋ 现成下发通道

**零新表、零新迁移、零新端点、零新配置键、零新 trait。**

`crates/platform/runtime/build.rs` 读构建期环境变量 `EP_REGULATORY_BASELINE_DATE`
（`YYYY-MM-DD`），经 `cargo:rustc-env` 注入；`BuildInfo` 加一个字段
`regulatory_baseline_date`；未注入取 `unknown`，沿用该文件已落码的惯例。

**选编译期的理由落在 build.rs 自己的注释上**，逐字「前者必须在构建时定死，
**运行期再算就等于让被测者自证**，因此落在 build.rs」——这正是「法规基准日不得被事后随手改」
所需要的性质。

**四条候选逐条驳回：**

| 候选 | 驳回理由 |
|---|---|
| 甲　配置项 | **运行期可改且不留痕**，而本条义务的实质是回答「你算的是哪一版法规」，一个谁都能改的字段防不住它要防的风险 |
| 乙　新建登记表 | 代价四道闸（sqlcheck 13 条规则、db/checks 八项目录扫描、`unpoliced_table_registry` 五列必填加一个 rls_matrix 用例、data-dictionary 先登记后实现），且阶段序倒挂——`platform_core` 由阶段 2/3b 建，早于阶段 10 的税率表 |
| 丙　原形 | 前半（编译期固化）成立并采纳；后半两处都断——「认证报告记录」无承接方（该报告 14 项必记全是性能容量实测量、零项软件版本类），「界面只读展示」被 `01:237` 逐字「全部只监听回环地址」堵死，且有 e2e 用例专门断言读不到 |
| 丁　挂既有承载物 | 五个逐个不合：`schema_history` 是结构版本；`ep_build_info` 指标端点仅回环；`client_releases` 只四端；`min_platform_version` 是包对平台的要求；`deployment_records` 形态最像但由阶段 14 建、且唯一读出口只给三类运维角色，**而本条要标注给的是用东西的人** |

#### 结论四　界面侧落得了（两处），单据侧首版**落不了**，不假装两侧都能落

**界面侧：**
- **阶段 13**——`client-bootstrap` 响应加一个 `regulatory` 顶层块（`baseline_date`、
  `platform_version`、`git_commit`），与既有的 `brand`、`device_policy` 同级。
  选它三条依据各落在自己那条边上：它是卷内**唯一横切四端的现成下发通道**；
  它已有下发台账与审计事件（`13:738` 逐字）；它在 T0 就有最小形态。
- **阶段 10**——invoice 开票界面显著位置一行文案。阶段 10 已有四端界面交付物行
  （按裁定 A-23 由本阶段而非阶段 13 交付）与对应退出条件。
- 另在「部署状态与已知限制页面」单列一节作总入口，**不进 PRD 11.11 的八条计数**。

**单据侧：首版落不了。** 三条硬依据：
一、**首版一张发票打印模板实例都不播种**——预置对象封闭为两类（默认管理驾驶舱与默认账龄分档），
   无任何发票打印模板；
二、`bound_document_type` 的**取值域全卷零定义**，17 份计划只有一行且未说能绑哪些单据类型码；
三、**阶段 10 的「发票打印」是一句无承接的散文**——§1 十六条交付物无一条含打印／渲染／模板／PDF，
   §9 三十三条退出条件对这四词零命中。**不做也不会有任何判据当场失败，正是第三类缺陷。**

**处置**：单据侧义务归**阶段 11**，形态收到最小一条——`render-tasks` 的 job-worker 侧渲染，
在绑定单据类型码属结论五四码时，把三项作为**固定页脚**写入产物；
**不经模板 spec、不加模板保存校验、不新增 trait**。
该因果边的逐字依据落在 `DocTemplatePort::render` 的 `model: serde_json::Value` 这一行本身
（自由结构，容得下三项），不是靠「阶段 10 有打印」与「阶段 5 有端口」两个端点推出来（纪律六）。

**须同批登记一句：T0 期间标注不可见**——T0 的 `client-bootstrap` 被限死为
「只返回能力取值与品牌默认值」。不写这一句，下一轮会被当成 T0 缺陷重开。

#### 结论五　「相关单据」全规格无定义，本裁定补四码

规格逐字只有「在界面和**相关单据**上显著标注」十四字，未给位置、未给任何可判定量、未给清单，
**「相关」二字全规格无定义——照原文验收是一条恒真判据**（任何一张单据都能被论证为不相关）。

补定四码：**SINV 销项发票、IRVS 冲销登记、PINV 进项发票、GV 记账凭证**。
前三码由规格事件-分录表中**过「应交税费」科目的四行**反推，GV 由 `spec:1657` 并列的
「会计核算」领域反推。合同与销售订单带税率但不过应交税费科目，不进清单。
**此清单是裁定补规格空缺，写入时须明标来源，不得表述为规格要求。**

#### 结论六　三处过度设计自我驳回

一、**驳回**「在 PRD 第 11.11 节加第九条」——那八条是**能力边界**，本条是**口径时点**，性质不同；
   加一条要同批改四处「八条」为「九条」，四处连带换一处渲染，不值。
二、**驳回**「新建运行期比对判据（基准日 vs 税率表 `max(updated_at)`）」——
   它要给税率查询 trait 加第三个方法（而卷内逐字限死两个方法、任何阶段不得另设税率桩），
   且 `updated_at` 语义是编辑时刻，**拿它比对本身就在混口径**。
三、**驳回**「在打印模板保存校验里加一条」——首版无预置模板实例，
   **该校验在首版被测输入为空集，是恒真判据**，按纪律不得以恒真形态立条。

#### 同批请求的规格修订（两处）

1. `spec:186` 加一句「本条不受维护订阅状态约束」（结论零）。
2. 第 3.5 章补「相关单据」的清单定义，或明写授权由实施方按裁定补（结论五）。

#### 本轮未做到的

- 结论五四码的反推依据取自规格事件-分录表，**未逐码回到各阶段计划核对其单据类型码是否已定义**，
  标为待核。
- 结论四单据侧的替代形态（render-tasks 固定页脚）**未核 `PrintLayout` 取值是否容得下页脚**，
  标为待核。
- 全部结论为静态读码与文本比对，**未跑库、未构建**。

### F-34　裁定辛-28：登记的「两处注释与代码事实相反」有一半是错的；真缺口今天不可落码

#### 结论零　先更正本人的登记

辛-28 登记逐字含「**且两处模块注释与代码事实相反**」。**这一半错了。**

实测：`ep-adapter-db-pg` 里 `impl SqlProbe` 命中 **0**；全仓两处 `impl SqlProbe` 分别在
`apps/core-server/src/wiring/probes.rs` 与 `apps/job-worker/src/wiring/probes.rs`，
是那两个 app **各自自建**的 `FoundationProbeAdapter`，建在 `PgDataFoundationCheck` 之上。

**所以「`ep-adapter-db-pg` 尚未提供 `SqlProbe` 实现」这句字面为真**，两处注释都写了它。
- `ops-agent` 那条**整句为真**，不必更正实质，只是措辞会误导（见结论三）。
- `integration-gateway` 那条里假的只有**「与 core-server 同理」**六个字——
  core-server 早已自建适配器并注入。

**我把「一个从句为假」写成了「两处注释与代码事实相反」。** 这是第六条纪律的邻近形态：
断言两个端点为假，实际只有连接两者的那半句为假。

#### 结论一　真缺口是另一件，且它今天不可落码

**两个进程被声明持有常规数据库连接、被连接预算记了 Integ5 与 Ops2 共 7 条，
而它们整个不依赖数据库适配层。**

实测成本：`core-server` 有 10 个 `ep-*` 依赖与完整 `DbAssembly`
（池、`PgDataFoundationCheck`、`FoundationProbeAdapter`）；
`ops-agent` 只有 3 个（`ep-foundation`、`ep-platform-runtime`、`ep-platform-obs`），
`integration-gateway` 同。给它们接探针，等于给两个今天完全没有数据库代码的进程
做整套 DB 装配（含 `ep_ops_ro` 角色的池），**那是阶段交付物，不是门禁修补**。

#### 结论二　考虑过并否掉的两条修法，逐条给否掉的理由

**（甲）把 `holds_sql_session()` 对这两个进程翻成假。** 否。它不是局部改动：
- `crates/platform/runtime/src/process.rs` 有一条冻结测试
  `exactly_four_processes_hold_sql_sessions` 断言恰是这四个；
- `crates/platform/runtime/src/selfcheck/secrets.rs` 用**同一个标志**门控机密自检，
  其文案逐字「不持有常规数据库连接**与密钥域**」——翻转会连带把机密自检也判成
  `NotApplicable`，而这两个进程有没有密钥域是另一个未答的问题。

**（乙）加一条 archcheck 规则「声明持 SQL 会话 ⟹ crate 必须依赖数据库适配层」。**
判据本身是对的、可机检，**但它今天会在这两个进程上判违反**，而按结论一我修不了。
加了只有两条路：让一道已交付的 CI 门禁红在一件今天关不掉的事上，
或者给它配一张恰好装下当前违反集的豁免名单——**后者是恒真判据**，本卷明禁。
**故不加。**

#### 结论三　落码只做能确证的那一处，其余按通则第六条第三档

**已落码**：两处模块注释更正。
- `integration-gateway`：删去「与 core-server 同理」，写明 core-server 早已自建并注入，
  本进程与它不同理、是**尚无任何数据库装配**。
- `ops-agent`：原句字面为真但会误导——**会让人以为「等适配层提供」就行，
  实际缺的是本进程自己的整套装配**。改为写明这一点。

**判据按通则第六条第三档降为评审判据并登记**：
「凡 `holds_sql_session()` 返真的进程，其 app crate 必须依赖数据库适配层并注入 `SqlProbe`」
——该判据在两个进程接库之前不具备可落地的机检形态，登记为评审判据，
提交时按 文件：行号 举证。

#### 结论四　连带确认辛-31 的另一半

F-27 曾记「规格 42 条常驻连接里记在这两个进程头上的 7 条，代码里无任何池创建点」。
**本轮独立复算确认**：两个 crate 的依赖表里没有任何数据库依赖，故无池创建点。
两条登记指的是同一处事实，**不重复计为两处缺口**。

#### 本轮未做到的

- 这两个进程**有没有密钥域**（结论二甲的连带问题）**未查**，标为待核。
- 「Pending 只减不增」是阶段 1 计划为该口径配的补偿控制，
  **本轮未核它今天有没有承接方**，标为待核。

### F-35　辛-40 处置：补上反向支，并更正本人对「恒过」成因的判断

#### 结论零　更正登记：今天恒过的成因是**两侧皆空**，不是单向

辛-40 登记逐字「`db/checks/11` **单向且**被测表零行，恒过」——**把两件事并成了一条**。

实测：`platform_core.sensitive_field_registry` 零种子行（阶段 4 计划逐字「本阶段对该表
只有读取路径」），而迁移里 `<col>_enc bytea` 与 `<col>_key_ref text` **各 0 处**。
**两侧都空，所以补不补反向支，今天都恒返 0 行。**

**「单向」是结构缺陷，「零行」是今天恒过的成因，两者独立。** 我原来的表述会让人以为
补上反向支就不恒过了——不会。

#### 结论一　反向支仍然该补，理由是它防的失效模式将来会到

补的是 `UNREGISTERED_ENCRYPTED_COLUMN`：物理上存在 `<base>_enc bytea`
却没有对应的 `is_field_encrypted` 登记行。**缺这一支，「有人加了加密列但忘了登记」
这一类错永远不会被本脚本看见**——正是 13 号靠 `UNREGISTERED_UNPOLICED_TABLE`
兜住的那一向。

登记侧的生产方**已排期**（阶段 3b、5、10 各有回填且都进了退出条件），
届时两侧都会有取值，本支即生效。schema 范围与 13 号逐项一致（24 个）。

#### 结论二　如实交代两条限制

一、**这份 SQL 我跑不了。** 本机无库，全部验证是结构级：三支、括号平衡、
   schema 列表与 13 号逐项相等。**未经一次真实执行。**
二、**本脚本属 `ep-migrate check` 的十三项，而该命令今天没有任何自动调用方**
   （附录辛第 27 条）。**改的是一份今天没人跑的文件。**

#### 结论三　档位

辛-40 由「成立」改为**「结构已补齐，恒过成因另属」**：
- 结构面（单向）**已处置**；
- 恒过面（两侧零行）**不是缺陷**——阶段 4 计划逐字要求本阶段该表只有读取路径，
  零行是计划要的状态，按四要件属排期项；
- 可执行性面归**辛-27**。

### F-36　辛-27 二裁：本轮立案前提被证伪；三件里两件按纪律四撤为验收判据；另立辛-42

#### 结论零　我开这一轮的前提是错的

我重开辛-27 的理由是：「SQL-031 证明活库判据可以有静态对应物，
故 db/checks 与 sqlcheck 两边**可能大面积重叠**」。**证伪。**

逐项判定（口径：两侧断言的**命题内容**是否相同，且**被测面**是否相当）：

| 档 | 项数 | 编号 |
|---|---|---|
| 实质覆盖 | **0** | — |
| 部分覆盖 | **5** | 01、04、06、08、09 |
| 零覆盖 | **8** | 02、03、05、07、10、11、12、13 |

0+5+8=13，与 `NUMBERED_CHECKS` 的十三项相等。

**SQL-031 不是先例。** 它判的是「调用点与登记行的**次序**」这一层迁移文本自洽，
**不是任何一条活库结构判据的静态对应物**。以它推出「活库判据可以普遍静态化」，
是**以特例推全称**——本卷第二次出现这个形态（第一次是 F-24 的「四个门禁项」）。

#### 结论一　辛-27 的半径确实被夸大，但不来自重叠面，来自**四要件从未逐件核**

按 F-23 已把辛-27 拆成的三件，逐件过纪律四：

| 件 | 四格 | 判 |
|---|---|---|
| 一、`ep-migrate check` 的具名调用方 | **2/4** | 缺陷成立 |
| 二、`db/checks` 十三项的执行 | **4/4** | **按纪律四禁止继续登记为缺陷**，改写为验收判据 |
| 三、八个二进制 `--check` 的调用方 | **4/4** | 同上 |

**辛-27 今天只剩第一件。** 而第一件的缺陷是「卷内给了两个名字（升级脚本、起栈脚本）
且都不是任何阶段的交付物行」——那正是 F-23 结论一已判的内容，**不需要第二轮裁定**。

**故本轮对辛-27 本身不产出新处置，只收窄其登记。**

#### 结论二　本轮真正的产出：一条「移交给零执行方」的因果边

实测：全仓 **40** 处 `create table`（含 1 处注释），其中**行首 19 处**、
**缩进在 DO 块 `execute '…'` 字符串内 20 处**。
而 sqlcheck 语句级规则的入口逐字是 `let rest = norm.strip_prefix("create table ")?;`
（`xtask/src/sqlcheck.rs:357`）——**只认行首**。
故 **SQL-003／005／008／009／010 五条语句级规则对那 20 张表一次也不触发。**

而迁移里有一句**自陈**，逐字：

> `db/migrations/platform_msg/V20260915090000__platform_msg_create_idempotency_keys.sql:10`
> 「-- execute 字符串承载，**绕开 sqlcheck 对 create table 的文本解析**」

**并把判据移交给 `db/checks/01`——而接收方今天零执行方。**

**两端与边本身各有逐字依据（纪律六满足）**：产生处（20 张表在 execute 内）、
消费处（sqlcheck 只认行首）、以及**把前者交给后者的那一句注释**。

**这与「部署期闸门无调用方」不是同一件事**：那件说的是闸门没人跑，
这件说的是**判据被有意从一道在跑的闸门移交给一道没人跑的闸门**。另立**辛-42**。

#### 结论三　三处收窄，不作全称判断

一、**「sqlcheck 完全看不见 DO 块」是错的。** 行级规则（SQL-001／002／004／006／030）
走 `line_rule`，**看得进** execute 字符串。准确表述是
「**五条语句级规则**对那 20 张表零触发」。

二、**「十份只读 pg_catalog」复核通过，但 12 号的表述要收窄**：
它读 `pg_database`，准确说法是「不读登记表、不读数据行、**也不读逐表目录元数据**」。
读登记表的是 3 份（01、11、13），10+3=13。

三、**「主体由迁移文件决定」不成立于 02、03、12、13**：
前三者的被测对象由 `apply_le_rls` 等函数**运行期动态生成**，12 的被测对象由
`db/bootstrap` 而非迁移决定。

#### 结论四　残差里最重的一条

**02 号**：新建一张带法人列的表，**漏写一行 `attach_table_guards` 调用** →
`apply` 不失败、CI 不红（sqlcheck 全文 `rls`／`rowsecurity` 命中 **1** 处且只判策略名前缀）、
**该表 RLS 从未 enable，对全部法人可读可写。**

**静态侧原理上判不了**：RLS 的启用发生在 `apply_le_rls` 的 `execute format` 内。
这一条不是「冗余的第二道」，是**唯一一道**。

#### 结论五　新立第七条纪律

本轮的错法是：拿一个特例（SQL-031）推出一条全称（活库判据可普遍静态化），
且没有先问「两侧被测对象是否相同」。据此立：

> **七、静态对应物不得被表述为活库判据的替代。**
> 凡以静态判据承接一条活库判据，须在同处写明三件：
> （a）两侧的**被测对象**各是什么；
> （b）**一个可复算的场景：静态过而活库不过**；
> （c）该场景是否已有别的承接方。
> **举不出 (b) 的，说明活库那一道是冗余的，应撤而不是并存；
> 举得出而 (c) 为空的，静态判据只能记为「部分承接」，不得记为承接。**

SQL-031 与 configdoc 第四段**已按此形态写过自陈边界**，回溯合规；
本条自今日起对后续一律生效。

#### 结论六　对辛-24、辛-40 的连带更正

那两条的「可执行性归辛-27」**须改**：按结论一，辛-27 的第二件已撤为验收判据，
故两条的可执行性面**同为验收判据，不是缺陷**。
两条各自的**结构面**（SQL-031 的部分承接、11 号补上的反向支）不受影响。

#### 本轮未做到的

- 逐项覆盖表的「部分覆盖」五项，其差异面取自核查逐条论证，
  **我只抽验了其中三项**（01 的白名单与三列剔除、06 的前缀入口、09 的 restrict 跨 schema 例外），
  另两项（04、08）未逐条复核，标为待核。
- 全部结论为静态读码与文本比对，**未跑库**。

### F-37　附录辛终局定谳（其一）：已裁 10 条，另更正账目 4 处

使用方要求「剩余部分一次性全裁完」。**本卷只落已实际裁出的 10 条，缺料 5 条另行补跑**——
代拟五条我没读过取证的裁定，正是本卷已被记录多次的那类错的放大版。

#### 结论零　定谳表（10 条）

| 编号 | 处置 | 今天可复算的后果 |
|---|---|---|
| 辛-15 | **当事人更换后保留** | **有**（见结论一） |
| 辛-16、17、19、20 | **撤销**（4 条） | 无 |
| 辛-18、23、25、31 | **降为验收判据**（4 条） | 无 |
| 辛-26 | **已承接结案** | 无 |

**本轮新立缺陷 0 条，升档 0 条。**

#### 结论一　辛-15 是十条里唯一保留的，但当事人整个换了

**原当事人撤回**：`platform_audit.audit_events` 今日未建表，
`from_occurred_at` 与 `verify_segment` 生产侧零调用——**后果指不出**。
`Clock` 端口那一半按 F-28 复核已并入辛-33。

**换上的当事人是 sign-in 锁定窗口链**，三份逐字齐（纪律六）：
产生处 `identity.rs:186` 的 `Utc::now()`；送入边 `login.rs:198/226-229` 的 `now` 参数；
消费处 `login.rs:459` 的 `locked_until > now`。

**后果一开始被写夸大了一级，攻击强制更正**：不是「被放行**登录成功**」——口令与 MFA 仍要过。
**真错答案在 `identity_sessions.rs:430-433`：上锁即把 `failure_count` 清零**，
故每回拨一次墙钟即重获整轮 `max_failures` 次尝试，**锁定这条速率限制整体作废**。

**缺陷形态同批更正**：由「无判据」改为「**判据在册但恒不生效**」——
四要件第三格原判 ✗ 只因我 grep 了「w32time／时间服务」两个词，
而落在词表外的三整句都在（`00b:600`、`03:53`、`14:499`）。**纪律一「宽词表整句漏」的又一例。**

#### 结论二　四条撤销，理由都是承重句当场为假

- **辛-16**：「无录入端点」为假——`13:680` 的通用配置包录入在场，F-21 结论七已正面裁过。
- **辛-17**：承重的**否定式全称句**「无可区分性判据」被四处在场判据证伪
  （`11:230` 列级 CHECK 判别子、`11:235`／`:244` 逐变体 JSON Schema、`11:484` 提交发布闸门、
  `11:497`／`:760` 把高级只读 SQL 圈死在报表定义 spec 内）。
- **辛-19**：「零文法」当场为假——`template.rs:52` 逐字「**占位符形如 `{name}`**。
  `allowed` 是该 `notice_type` 的变量白名单」，实测 **8** 条用例，
  **而该文件早于我的登记一天入仓**。
- **辛-20**：「无登记落点」为假——`00b:787` 登记表在场，
  `xtask/src/archcheck/mod.rs:27` 逐字 `pub const DELEGATED: [(&str, &str); 2] = [`
  且与表内两行逐行相等，`registry.rs` 读 md 比对、不符即退出码 1。

#### 结论三　辛-26 已承接结案，但我一处证伪侧 grep 本身是错的

声明面已由本卷落码的 `configdoc` 第四段承接并在 CI 第 6 阶段 `delivered`。

**更正**：我此前称「`X-Client` 与 `client_capability_values` 全仓零命中」——**前半为假**。
`apps/core-server/src/platform/middleware.rs:577` 逐字
`match header_of(req, "x-client").as_deref() {`——
**能力闸两半里「读取请求头 `X-Client`」这一半今天已经在线**（我用了大小写敏感的 grep）。
处置不变的理由是另一半仍缺：`client_capability_values` 实测命中 **0**。

**另一处计数更正**：登记的「四处 `_capability`」实测为 **6 处**——
另两处在 `platform/mod.rs`，**是本卷自己改路由时新增的**。

#### 结论四　账目更正 4 处

1. **F-28 结论一今天仍在产出错答案**——只读那三行的人会得到「规格里没有 A.3 的 80% 条款」
   这个错结论。**已在该段前加「已更正，勿依此段」标记**，指向 F-31 与 F-32。
   **这是本卷唯一一处卷内文本自身今天就在产出错答案的地方。**
2. **在册数由 21 改 23**：总表数据行 28，减合并行 1、减「从未存在」1、减已撤销 3 = 23。
   原「21」既与总表不符，其算式「40 − 17 − 3」也得 20、与自陈不等。
3. **六档小计不覆盖辛-41 与辛-42**——两条在表内有行而在小计里隐形，
   与 F-28 建总表时点名要消除的病相同。待补跑完成后一并重算。
4. **F-33「未做到」栏的承重物指错**：`PrintLayout` 应为 `DocTemplatePort::render` 的
   `model: serde_json::Value`——其结论四正文点的本来就是后者。

#### 结论五　欠账台账：10 件当场核完，4 处与原判不符

**三分**：今天能核完 **10 件**（已全部核完）、要跑库 **6 行**、要等阶段落码 **0 行**。

**与原判不符 4 处**：H-07「两条路都不通」应改为「一条撞冻结登记表、另一条须先改基线 `00b:239`」；
**04 第 9 节三条被测面为空**——F-27 定性为「未跑库，标为待核」**错**，
被测面是否为空**静态可判**，F-35 对 `sensitive_field_registry` 用的正是同一手法，
**同一推理形态在 F-27 名下待核、在 F-35 名下定论**；F-33 承重物指错；
F-36 称纪律七先例两处、**实为五处**。

**并明记一处口径问题**：「29 处」**不得表述为全集**——
十个「本轮未做到的」小节下条目共 27 个，含四关键词的只 14 个；**真实规模大于 29。**

#### 本轮未做到的

- **这不是一次「全裁完」。** 15 条里只裁出 10 条，**辛-32、37、38、41、42 五条材料未回**，
  已另行补跑。**按纪律三不代拟。**
- 辛-31 只收到处置档位与承重主张，**取证段未传入**，判据文本待补。
- 十条我做的是**抽样复核**：亲自实测的是在册行数、`template.rs` 的占位符与用例数、
  `DELEGATED` 长度、`x-client` 读取方、`client_capability_values` 计数五处。
- 全部结论为**静态读码与文本比对，未跑库**。

### F-38　附录辛终局定谳（其二）：补裁 5 条，并改判本人上一轮刚立的辛-42

F-37 只裁出 10 条，缺料 5 条本卷补齐。**终局定谳至此完成。**

#### 结论零　5 条分档

| 编号 | 处置 | 今天可复算的后果 |
|---|---|---|
| 辛-32 | **已承接结案** | 后果链**断在第一环**——判据已落码并挂进 `run()` |
| 辛-37、38、41、42 | **降为验收判据**（4 条） | 均指不出 |

**成立并保留 0 条。其中辛-42 是攻击改判**（原判「成立并保留」）。

#### 结论一　辛-42 改判：我上一轮的机制表述错了

F-36 结论二逐字写「sqlcheck 语句级规则的入口逐字是 `strip_prefix("create table ")`
——**只认行首**」。**错。**

`xtask/src/sqlcheck.rs:323` 逐字 `let norm = raw.split_whitespace().collect::<Vec<_>>().join(" ");`
——**`norm` 已把空白折成单空格，缩进早被剥掉**。故 `:357` 的
`strip_prefix("create table ")` 判的是**语句首词**，不是文件行首。
20 张表落空的**真因是切句器**（`:297` 逐字「按分号切句。单引号内的分号不切」）
把整个 DO 块并成一条 `do $$ begin …`。

**「只认行首」须改为「只认语句首词」。**

#### 结论二　更要紧的是方向反了：那 20 张里没有真违规

我举的反事实是「移到行首即报缺 `legal_entity_id`」。**那暴露的是 SQL-008 的假阳，不是漏判。**

`permission_items` 的豁免**有逐字登记**，三处：
迁移自身 `V20261012100000…:9-10` 逐字「本表不带 legal_entity_id 列、不建行级安全策略……
**登记行由第 29 号回填迁移写入 unpoliced_table_registry**」；
`db/checks/01_common_columns.sql:6` 逐字「不带 legal_entity_id 列的表，
其余八件必须齐备且按序占据第 1 至第 8 列」；
以及回填迁移里的实际登记行。**一致性由 `db/checks/13` 双向承担。**

**故 sqlcheck 对该表返 0 正是正确答案。** 我 F-36 里那句
「今天没有任何登记表记录哪 20 张表被豁免」**被证伪**——载体是 `unpoliced_table_registry`。

**另一处不得二次计入**：「接收方零执行方」这一半已由 F-36 结论一自己判为
「按纪律四禁止继续登记为缺陷」，不能在结论二里再算一次。

**半径也要收**：零触发的是**三条**（SQL-008／009／010），不是五条——
SQL-003 判 `create type`、SQL-005 扫全串 `references `，两者不属此族。

#### 结论三　其余四条降档的共同理由：后果指不出

- **辛-32**：判据已于 ad70e10 落码、挂进 `run()`、今日 **33 条**非系统路由为被测输入，**非恒真**。
  **附更正**：登记所引第三处上位权威「`14:576` 退出条件 21」在今日文件中**为假**
  （`14:576` 是退出条件 17，讲 `OpsDisposalService`），真出处是 `14:580`。
- **辛-37**：四条实测**逐条复验为真**，但两处种子字面量作差为空、无活库故唯一发射点不可达；
  且 `04:481` 所保的 PRD 条款**已由双载体承载**
  （`ck_permission_items_forbidden_codes` 与 `guard_permission_item_code`）。
- **辛-38**：四处口径**实为一处**真分歧——DDL 正则是两侧的**公共超集**而非第四套；
  发布执行路径未接通（`main.rs:187` 逐字「发布执行路径接通前在此显式持有，不以空实现顶位。」），
  126 个串今日只在 markdown 里。
- **辛-41**：零外键属实，但**「零跨表 CHECK」在 PostgreSQL 里恒真、不构成证据**
  （该数据库本就不支持跨表 CHECK）；而跨表咬合语义已由 `decider.rs:80-83` 在应用层承载。

#### 结论四　六档小计重算（F-37 结论四第 3 条的欠账）

终局后在册状态：**已承接结案 2**（辛-26、32）、**降为验收判据 8**（辛-18、23、25、31、37、38、41、42）、
**当事人更换后保留 1**（辛-15）、**已撤销 7**（辛-16、17、19、20、21、29、30）、
**已处置并回写 17**（辛-1 至辛-14a）、**从未存在 2**（辛-35、36）。

**其余仍在册的**：辛-22（两部分分列）、辛-24（部分承接）、辛-27（收窄为一件）、
辛-28（当事人收窄）、辛-33（当事人已换）、辛-34（半二已撤）、辛-39（当事人已换）、辛-40（结构已补）。

**终局结果：附录辛无一条停在「成立且无处置」。**

#### 结论五　本轮誊正的行号与计数（9 处，均不改处置）

其中三处值得点名：
- `.route(` 计数：辛-38 材料写「全仓 8 处」**为假**，今日 `apps/` 与 `crates/` 合计 **22 处、7 个文件**；
- `references` 计数：`platform_authz` 下 4 处命中里**两处是 `grant … references` 授权语句**，
  只有 2 处是外键子句；「零外键」结论不变，**口径须改**；
- `db/checks/01` 具名豁免为**四张**，攻击文写「三张」是排除 `schema_history` 后的数，须写明口径。

#### 本轮未做到的

- **未跑 cargo、未连活库**：辛-32 的反事实与辛-42 的规则触发方向**均为读码推断**。
- **辛-42 的「20 张逐张复算零真违规」未逐张复核**，只复核了计数与豁免登记的存在。
- F-37 的六处「要跑库才能核」欠账**仍未动**。

### F-39　首次活库执行：十四项断言全过，另实测出一条恒真的判据并当场修掉

**本机装有 PostgreSQL 16.14 且已在 5432 运行，库 `ep` 为完整迁移态**
（24 个 schema、40 张表、`schema_history` 69 行，与迁移目录 69 个 `.sql` 逐一对应）。
`db/checks` 自立卷以来**从未执行过**，本轮首次跑通。

#### 结论一　十四项断言首次实跑，全部返 0 行

`ep-migrate check` 十三项**逐项通过，退出码 0**；
被排除在十三项之外的 `append_only_consistency.sql` 单独以 `psql` 执行，**返 0 行**。

**这一条同时消掉本卷多处「未跑库」欠账**，其中三处点名：

| 原欠账 | 本轮实测结果 |
|---|---|
| F-35 结论二逐字「**这份 SQL 我跑不了。** 本机无库」 | 11 号**跑通**——我在 F-35 加的 `UNREGISTERED_ENCRYPTED_COLUMN` 反向支**语法正确、能执行** |
| F-38 未做到二「辛-42 的 20 张逐张零真违规**未逐张复核**」 | 01 号对**全部 40 张表**通过 → **那 20 张确无真违规**，由推断升为实测 |
| F-35 结论零「两侧皆空」为恒过成因 | 实测登记行 **0**、`_enc` 列 **0**，**两侧确为空** |

**另有一处此前静态判不了的，现已实证**：02／03／13 三项通过，
说明 `apply_le_rls` 在 `execute format` 里动态挂的 RLS 与策略**实际是对的**——
这正是 F-36 结论四点名「静态侧原理上判不了」的那一面。

#### 结论二　实测出一条恒真的判据，比 F-23 的警告更糟

F-23 曾逐字警告「加库而不同时把『未设 `EP_TEST_PG_URL` 即跳过』改成『未设即失败』，
等于把绿色挪个位置」。**本轮实测的形态比这更糟：**

`crates/adapter/db-pg/tests/live_pg.rs` 的五个用例，
**有库与无库两次运行，测试摘要都是 `ok. 5 passed`**，
差别只在耗时——**0.00s 对 0.50s**。

**即「真跑过」与「什么都没做」在结果上完全一样，而 CI 两种情况都看到绿。**
这是本卷第一类缺陷（恒真的判据），且它盖住的是**整个活库集成测试面**。

#### 结论三　已当场修掉：改为 `#[ignore]`

标准库测试框架只有 `passed`／`failed`／`ignored` 三种计数，
**其中只有 `ignored` 如实表达「本次没跑」**。故五个用例一律加 `#[ignore]`：

- 默认 `cargo test`：**`0 passed; 5 ignored`**——不再冒充通过；
- 有库时：`EP_TEST_PG_URL=… cargo test … -- --ignored` → **实测 `5 passed`，0.46s**。

运行期仍保留 `LiveDb::new()` 的 `None` 早退，用于「变量设了但连不上」的情形。

**这与本卷 `db/checks`／`--check` 的三态退出码（0 通过／1 违反／3 判定未做出）
是同一条纪律在测试框架上的落法：未覆盖不得表达为通过。**

#### 结论四　对辛-24、辛-27 的连带更新

- **辛-24**：其「活库那一半」**本轮已实际执行并返 0 行**。
  「幻影承接方」这一点**仍然成立**——`xtask sqlcheck` 依旧跑不了它（无 postgres 客户端），
  本轮是我用 `psql` 直接跑的，**不构成承接方**。
- **辛-27**：`ep-migrate check` **本轮被真实调用了一次**，证明该二进制可用；
  但「无**自动**调用方」不变——本轮是人工调用。

#### 本轮未做到的

- 活库是**开发机上的 `ep` 库**，不是认证机、不是交付机，**结论不外推到那两台**。
- 只跑了 `db/checks` 与 `live_pg`，**八个二进制的 `--check` 未跑**。
- 十四项全过是**在当前迁移集上**的结论；迁移集变化后须重跑。

### F-40　研发计划符合性审计：**不完全符合**，且审计自身只覆盖了 195/246

按使用方要求对 14 份阶段计划做全面审计。**结论先行：不完全符合，且「符合」这一判断
在证据上也不成立——不是因为发现了什么，而是因为有 60 节没看。**

#### 结论零　审计口径与实际覆盖面

| 口径 | 条数 |
|---|---|
| 规格二级节 | 102 |
| PRD 二级节 | 144 |
| 上位条目全集 | **246** |
| **本次实际审到** | **195**（规格 102 节＋9 个无子节章本体＋PRD 前 84 节） |
| **未审到** | **PRD 第 7–15 章共 60 节，零结论** |

**那 60 节不得按「未发现问题」解读。**

#### 结论一　五档分布（对抗核查后）

| 档 | 条数 |
|---|---|
| 已覆盖 | **158** |
| 仅有对应行无承接物 | **10** |
| 散文承诺无判据 | **1** |
| 未覆盖 | **22** |
| 明确排除 | **4** |
| 合计 | **195** |

**无承接方合计 32 条。** 三路原判 6 条「阻断」，**经对抗核查一条不剩**——降为高 3、中／低／无若干。
这个结果本身值得记：**审计方判「阻断」的门槛与本卷判「成立」的门槛一样偏松。**

#### 结论二　三条「高」，逐条实测确认

**高-1　规格 7.6 数据迁移整节无承接物。**
本裁定复算：`字段映射`／`增量追平`／`只读冻结` 在 14 份计划中**各 0 次命中**。
`05-master-data.md:20` 逐字「按裁定 A-24 不设独立数据迁移阶段」，
而 A-24 只落了总账、往来、资金账户、库存四条**期初**通道，**无一条对应 7.6**。
后果不静默：规格第 22 章第 8 条要求「数据迁移……完整可用」，会在发布门禁上判不通过。

**高-2　规格第 18 章升级与生命周期，四项无承接物。**
四档时长上限与切换窗口、升级前定制兼容测试、受控更新网关、逐次升级/回退证据包十一项构成。
**对抗核查补入了审计方漏查的部分承接**（`01:29` D-11 的回退说明、`01:510` 退出条件 15 的验签），
故「整章缺席」不成立，降为高。

**高-3　规格 21.4 三类专业签字无承接物。**
`威胁模型` 在 14 份计划中 **0 次命中**，而规格明写安全签字须覆盖它。
阶段 6/9/10/14 的退出条件里**无一条签字项**，而第 22 章第 12 条逐字
「签字缺失或不通过时正式版不得完成」。**补齐形态是四份计划各加一条退出条件行,零代码影响。**

#### 结论三　三处口径冲突，本裁定逐处实测确认

1. **许可临期窗口 30 vs 60。** 规格逐字「到期日前 **60** 天进入临期告警」，
   `03-platform-kernel.md:1149` 逐字「临期窗口取 **30** 天，属本阶段临时取值」。
   计划自标临时，但**没有任何退出条件会因取 30 而失败**——第三类缺陷。
2. **`LicenseStatus` 四态无宽限期态，而同一份计划有「许可宽限期告警」。**
   四态逐字「…不足临期窗口为 `ExpiringSoon`；否则 `Valid`」——**没有宽限期**；
   而 `03:930` 有「许可宽限期告警的 50 行」。**那条告警没有可触发它的状态。**
3. **裁定卷自身账目不自洽**：`00c:928` 逐字「…**三个**通道各自落在已有阶段」，
   而其下表列**四行**、次段自称「**四个**通道」。**十四份计划以此结论为承接依据。**

#### 结论四　一处回写债，本裁定当场查实

**F-33 已裁的辛-7a（法规基准日期标注）尚未回写进任何阶段计划**——
`法规基准` 在 14 份计划中 **0 次命中**。F-33 定的形态（构建期注入＋三处渲染点）
须回写进阶段 10、11、13 三份计划，否则那条裁定停在裁定卷里、不进施工面。

#### 结论五　还差什么才算符合（12 项）

**补交付物行与退出条件 7 项**：规格 7.6 数据迁移工具面；第 18 章四项；21.4 四份计划各加签字项；
7.10 迁移对账与差异登记；12.3 证书生命周期（或明确不取并给理由）；
3.4 可信时间／撤销文件／用量申报＋补齐宽限期态并把窗口改回 60 天；17.2 第十七类测试。

**补判据不新增功能 3 项**：`14:574` 退出条件 15 由「逐条产出判定结论」改为「逐条判定为通过」；
15.3 的 OTLP 与 SIEM 出口认领阶段；21.14 监测责任人与闭环演练进证据包，并清 F-33 的回写债。

**消一致性问题 2 项**：门户对账端点前缀两写二选一；`00c:928` 的「三个」改「四个」。

**以上 12 项全部落地后仍不能宣告符合**——还须补审 PRD 第 7–15 章 60 节。

#### 结论六　本次审计做不到的（八条，逐条如实）

1. **六路只收到三路**，PRD 第 7–15 章 60 节**零结论**。
2. **三路的对抗核查文本均中途截断**，**9 条缺口未经任何对抗核查**，严重度按原判照录。
3. **只抽验引证，未逐条重验**：本裁定逐字复核 32 处、双向计数 48 个关键词，
   三路提交的其余数百处引证**未逐条打开**。
4. **覆盖侧几乎没验**：158 条「已覆盖」只复核了 6 处承接物。
   **判「已覆盖」的门槛我在缺口侧执行了，在覆盖侧没有执行**——
   那 158 里可能混有「有交付物行但判据恒真」的条目。
5. **词表法仍会漏**：查出三路自报数据里一处计数错（`无障碍` 报 0、实测 1），
   **且我只能靠偶然复算发现**——纪律一的新形态在本次审计里同样发生了。
6. **四要件的第四件「今天零消费方」对全部 195 条一律未核**——那要读代码树统计调用点。
7. **静态对应物与活库判据的区分只做到文本层**：能判「写的是不是一条可判不通过的条件」，
   **判不了它在真实数据库上是否真会失败**。
8. **三路口径不可直接合并**，本表是重排后的结果，与三路自报数字不可逐一对上。

### F-41　符合性审计补审定谳：**不完全符合**（合并 255 个计数单位）

补审 PRD 第 7–15 章 60 节，并补做 F-40 自陈未做的覆盖侧抽验。**最终结论：不符合。**

#### 结论零　合并后的全量覆盖面

| 档 | F-40（195） | 本轮（60） | 合并（255） |
|---|---|---|---|
| 已覆盖 | 158 | 33 | **191** |
| 明确排除 | 4 | 3 | **7** |
| 仅有对应表行无承接物 | 10 | 6 | **16** |
| 有承接物但判据有缺陷 | 1 | 7 | **8** |
| 未覆盖 | 22 | 11 | **33** |

**无承接方合计 57 条，占 22.4%。**

口径说明：255 ≠ 246，差在 F-40 把 9 个无子节的规格章本体也计了条目。
**246 口径下的分档无法复算**——F-40 只给了五档合计数、**没有那 195 条的逐条清单**，
故只能给区间：已覆盖 149–158、无承接方 24–33。**这是账目限制，不是判断。**

#### 结论一　覆盖侧抽验:20 条里 4 条经不起重验

F-40 自陈「判已覆盖的高门槛在覆盖侧没有执行」。本轮补做:从 PRD 第 1–6 章分层抽 20 条，
**用判缺口的同一把尺重验**。

**结果 16 条经得起、4 条经不起**，失败形态**高度一致**：
PRD 2.1.3 系统管理员、4.5.1 收货操作者、4.9.2 第七条、5.4.3 扫码重复处理——
**四条全部属于「多子项的节里有一到两个子项只落在对应表行上」**。

**这不是个别失误，是登记方式的系统性问题**：对应表按**节**登记，
而承接物按**子项**存在；两者粒度不匹配时，**缺的那一子项在纸面上不可见**。

据此对 191 条「已覆盖」外推：约 **38 条**可能在同门槛下站不住。
**此为外推，不是复算**：抽样只取 PRD 第 1–6 章，**规格 102 节的已覆盖条目一条都没抽**。

#### 结论二　一条跨计划交接空档，两份各自读都完整

**PRD 10.3 高风险操作审批段**：
`04-identity-authz.md:506` 逐字「属开篇同批清单第一项，**随阶段 3b 一并交付，本阶段不注册该路由**」。
而实测 **`03-platform-kernel.md` 全文 `high_risk` 命中 0、「同批清单」命中 0**——
**接收方从未指名它接了什么。**

**这类缺陷在单份计划的自查里查不出来**：04 说「我移交了」，03 读起来也完整，
缺口在两份之间。

**须更正核查一处计数**：它称 03 退出条件节内 `高风险|重新认证|自审|审批` 命中为 0，
**我实测为 2**。承重证据改用「全文 `high_risk` 与「同批清单」各 0」——那两个是硬的。

#### 结论三　驳回核查一条错误发现：并发 20 不是回写债

核查列的第 10 项称「裁定 F-16 已把并发上限改判为 10，`00b:765`、`04:613`、`04:576`
三处仍取 20 未回写」。**这一条是错的，本裁定驳回。**

F-16 逐字记的是「人机并发上限 **10**（低于规格第 3 章的 20，
按第 16 章『通过线不因并发下降而放松』**不据此放宽任何**…）」；
F-19 结论一逐字「实际部署的并发上限是 10，低于认证的 20……
**按 20 认证不是放宽而是收紧**：20 并发下过线，10 并发下必然更宽裕」。

**计划取 20 是对的。没有回写债。** 核查把「实际部署 10」误读成「设计基准应改 10」。

#### 结论四　但该项的另一半是真的，且已自陈为待表态

`prd:4227` 逐字「实际并发或数据量超出上述基线时，业务不被阻断，**平台不拒绝登录**，
也不停止写入」；而 `00b:765` 逐字「并发达到 20 时新会话进入等待队列，
**等待超过 10 秒返回 503**」——**返 503 就是拒绝登录，与 PRD 相反。**

**但同一行末句已自陈**：逐字「这是技术侧的承载方式，
**是否改为不限制只记录留待产品负责人决策**」。

**故不判缺口，判「待使用方表态」**，与辛-7a 同档。**已提交使用方。**

#### 结论五　最终结论：不符合，三条独立理由

**一、57 条上位条目在计划侧指不出承接物**，其中 16 条**只有对应表行**——计划自称覆盖而实际未覆盖。
**二、覆盖侧的门槛没有执行到底**，抽验失败率 20%，且失败形态是系统性的（节 vs 子项粒度不匹配）。
**三、有缺口落在两份计划的交接处**，单份自查查不出来。

**「还差什么」合并去重后 25 项**，按三键排序（是否会使发布门禁判不通过 → 是否需新增功能 →
是否已被裁定登记）。其中**第 4、20、25 三项可当轮关闭且不新增任何功能**：
四份计划各加一条签字退出条件行、`14:574` 由「逐条产出判定结论」改「逐条判定为通过」、
以及五处账目改字。**其余 22 项需新增交付物或作出业务决策。**

#### 结论六　本轮做不到的（六条）

1. **第四路对抗核查只到两路**——第 11 章＋附录乙那 24 节的 14 条缺口**未经任何复核**。
   前两组的对抗核查查出 **8 处引证错**（含 1 处伪引证），按同密度推断第三组也应有若干处。**推断。**
2. **覆盖侧抽验只做 20 条**（占 191 的 10.5%），且**全取自 PRD**，
   那个 20% 的外推**对规格侧不成立**。
3. **抽验的 20 条里有 2 条我自己没核到底**，其中 PRD 3.10 第三条我按推断计为在场，**那是判断不是取证**。
4. **四要件第四件「今天零消费方」对全部 255 条一律未核**——与 F-40 同一处空白，**两轮都没补**。
5. **静态与活库的区分仍只到文本层**：能判「写得可判」，**判不了它真会判负**。
   这处空白已经吃过亏——提交 `d3db368` 记的正是「活库用例实测其『有库无库都报 5 passed』是恒真判据」。
6. **F-40 那 158 条的逐条清单不在仓库里**，只有合计数，
   故本轮**无法定位是哪 158 条**，只能重新抽样再外推。

### F-42　关闭 F-41 清单里可当轮关闭的三项，另留两处待核

F-41 的 25 项里有 3 项**不新增任何功能、当轮可关**。本卷执行，并如实说明哪几处没敢改。

#### 结论一　规格 21.4 专业签字：四份计划各加一条退出条件（F-41 第 4 项）

此前**四份计划的退出条件中无任何签字项**，而规格第 22 章第 12 条逐字
「签字缺失或不通过时正式版不得完成」。已加：

| 计划 | 条号 | 签字方 |
|---|---|---|
| 06 合同与销售 | 20 | 法务 |
| 09 总账与期间关账 | E-24 | 会计与税务 |
| 10 应收应付与发票 | 34 | 会计与税务 |
| 14 运维备份与发布门禁 | 23 | 安全（**须覆盖规格第 12.5 章审计链与威胁模型**） |

四条同款写明：签字人资格证据随版本留档；**签字缺失或不通过时本阶段不得退出**，
整改后重新测试并重新签字，**不得以未记录的方式豁免**。

**连带消掉一处零命中**：`威胁模型` 此前在 14 份计划中命中 **0**，现为 **1**。

#### 结论二　`14:574` 退出条件 15：由恒真判据改为可判负（F-41 第 20 项）

原文逐字「ep-release-gate 对第 22 章十五条与第 17.2 章通过标准**逐条产出判定结论**」——
**只要求产出结论、不要求结论为通过**，十五条中任一条判为不通过时该退出条件**仍成立**。
这是本卷第一类缺陷，且它守的是发布放行。

已改为「**逐条判定为通过**」，并在同处写明原措辞为何是恒真判据。

#### 结论三　账目改字三处，均逐处核准后才改（F-41 第 25 项）

| 处 | 原写 | 实测 | 依据 |
|---|---|---|---|
| `05:842` | 「表中 **14** 个场景」 | **13** | PRD 第 2.12 节表为 13 个数据行（15 行减表头与分隔） |
| `07:1078` | 「**七条**访问与数据约束」 | **八条** | PRD 第 4.9.2 节实为 8 条 |
| `00c:928` | 「**三个**通道」 | **四个通道、三个阶段** | 其下表 4 行数据，次句「最终归属阶段：9a、10、8」是 3 个阶段——**原文把通道数与阶段数混了** |

#### 结论四　两处**没敢改**，如实说明

F-41 第 25 项还点了两处，本卷**不改**，理由是**我数不出正确值**：

- **`12:731`「PRD 9.10 列出的**八类**必须留痕动作」**——实测 PRD 第 9.10 节只有
  **4 个**「- 」条目，**但我找不到任何读法能得出「八」**。
  可能是那 4 条各含多项，也可能计划引错节。**不能凭空改成 4**，标为待核。
- **`13:1047` 五类定制共同硬边界的条数**——F-41 称「三条实为五条」，
  而该行逐字是「| 10.4.7 五类定制的共同硬边界 | 不得跨模块直接读写业务表；…」，
  **我未定位到写「三条」的那一处**，无法核准。标为待核。

**这两处的处置本身就是本卷的纪律**：**没核准的数不改**——
改错一个数与原来错着，在后果上没有区别，而改过之后更难被发现。

#### 结论五　F-41 清单的剩余

25 项中本卷关闭 3 项，**剩 22 项**，全部需要**新增交付物或作出业务决策**，不在本卷范围。
另有 F-41 结论四那条**已提交使用方待表态**（并发到顶时排队返 503 vs 不限制只记录）。

#### 本轮未做到的

- 两处账目**未核准故未改**（结论四）。
- 新加的四条签字退出条件**本身没有承接方**——它们是退出条件，
  由人在阶段结束时判定，**没有任何机检会检查签字是否真的取得**。
  这与本卷其余「验收判据」同档，**不宣称已解决，只宣称已登记**。

### F-43　PRD 侧指空引用清零；规格 7.6／7.10 与第 18 章的零承接面补上判负口

> **历史审计快照。** 本节结论三、结论四及“本轮未做到”记录的是 F-43 作成时的缺口，不是现行实现口径。历史数据迁移、补丁分发与支持套餐周期已由 F-53 唯一收口；凡本节仍出现“今日必然判负”“不认领”“等待表态”或“尚未交付承接物”，均只解释当时成因，不得据此阻止开发或恢复旧分支。

本卷先修 PRD 自身能修的，再把 PRD 与规格的无承接条目落到计划侧。

#### 结论一　PRD 具名引用**指不到任何标题**者 15 处，已清零

PRD 第 93 行自订纪律逐字「一个术语只在一个节定义，其余节只写名称并**注明见 PRD 第某节**」。
实测：名称式引用 **110 处**，节号式仅 **3 处**。

**本裁定不批量改格式。** 110 处里 82 处的名称能解析到实际标题，改格式要我逐处解析 110 个名称到节号，
**错一个就是造了个看起来权威的错指针**——这与本卷两次因未核准的数栽跟头是同一类风险。

只修**解析不到任何标题**的：

| 原引用名称 | 处数 | 实际目标 | 核准依据 |
|---|---|---|---|
| 平台内核与权限／平台内核／平台与权限 | 10 | **第 10 节** | PRD 只有第 10 节「平台能力：权限、低代码定制、通知与审计」 |
| 平台内核与运维 | 1 | **第 10 节** | 运维中心在 10.6.3 |
| 经营报表与驾驶舱 | 1 | **第 8 节** | 实为「成本归集、经营指标与报表」 |
| 安全与数据保护 | 1 | **第 10 节** | **PRD 根本无此节**；其术语表第 119、123 行逐字已判「高风险操作、重新认证 → 第 10 节」 |

共 15 处。**核准依据全部取自 PRD 自身**——术语表已经在用节号，所以第 93 行那条纪律是真的、
术语表遵守了、正文没有。改后复核：具名指空 **0 处**，节号式引用由 3 处升至 23 处。

顺带纠正 F-41 的一处误判：F-41 称 `PRD:1838`「见 PRD 数据迁移一节」指向不存在的节，
**不成立**——`#### 2.11.2 历史数据迁移导入` 就在第 839 行，只是四级节。真问题是引用格式而非悬空。

#### 结论二　`03` 三处裁定回写欠账，加临期窗口，共五处

F-18 结论七已裁「**撤下「许可临期与宽限期告警」一类，十类改九类**」，
而计划至今三处仍按十类写。已改：

| 处 | 原 | 现 |
|---|---|---|
| `03:546` | `ck 取值为 PRD 10.5.2 的十类` | 九类，并写明有意不承接 PRD 哪一类 |
| `03:930` 接收人解析 | 含「许可宽限期取全体在职用户」 | 删去该分支 |
| `03:930` 扇出标定 | 「最大扇出为**许可宽限期告警**的 50 行」 | 改挂已冻结规模基线（命名用户上限 50） |
| `03:1593` 交付物行 | 十类枚举与模板 | 九类 |

第三处是关键：**扇出标定原本以一个已被撤下的类为支点**。改挂基线后，
按 F-18 自陈的好处，没有任何一类提醒的接收人是未定上界的集合。

第五处，**临期窗口 30 → 60**。规格第 3.4 章逐字「到期日前 **60** 天进入临期告警」，
计划取 30 并自称「切换只改配置」——**实测本仓不存在该配置键**（辛-6 已点此）。
既无配置可切、又无任何退出条件会因取 30 而失败，是第三类缺陷。取 60 与规格一致后该面消失。

**但必须记明：撤掉那条告警不等于关闭辛-5。** `LicenseStatus` 四态仍无宽限期态，
规格承诺的 30 天宽限期仍被 `Expired` 一个变体盖住，那是待业务决策项，不因本轮回写而关闭。

#### 历史结论三　规格 7.6／7.10 当时零承接，**现已被 F-53 完整替代**

实测 `字段映射`、`增量追平`、`只读冻结`、`迁移模板`、`哈希对账`、`分批迁移`、
`错误队列`、`期初余额衔接` 在 14 份计划中**各 0 次命中**。

**成因值得单记：计划里「迁移」一词几乎全指数据库 schema 迁移。**
`迁移窗口` 在 14 份计划中命中 **35 次**，指的是 `ep-migrate` 的 DDL 窗口，
**与规格第 7.10 章的历史数据迁移窗口同名不同物**。
按关键词计数会得到「覆盖充分」，**逐处核对则一处不是**。
这是第三类缺陷在文档侧的形态，也是我在本卷第六条纪律（因果边须有边上的逐字证据）的又一次适用：
**两个同名词不构成一条覆盖边。**

裁定 A-24「不设独立数据迁移阶段」只落四条**期初**通道，**无一条对应第 7.6 章的旧系统迁移**。
两者不是一回事：A-24 管期初余额录入，第 7.6 章管从历史系统搬数据。

**F-43 当时的处置**只是给 `14` 新增退出条件 27，让缺少承接物时必然判负。F-53 已把该判负口补成可实现承接，随后证据图加固又补齐两张台账：独立 `ep-data-migrate`、25 类模块 writer、六张法人 RLS 台账、完整试运行与源冻结、增量追平、对账、切换及整批冲销计划均冻结在阶段 14 第 4.12 节，退出条件 27、31 已是可执行验收而非“今日必然失败”。本段只保留同名“迁移”导致漏审的历史成因。

#### 历史结论四　规格第 18 章当时四项零承接，**现已被 F-53 收口**

实测 `切换窗口`／`定制兼容测试`／`受控更新网关`／`停机切换` 在 14 份计划中**各 0 次命中**
（此前统计的「计划数 1」是裁定卷 `00c` 自己，**它不是那 14 份**——记此以免再被自己的卷骗一次）。
`14` 第 10 节此前**无第 18 章的任何一行**。

新增退出条件 24–26：四档时长上限逐档实测判定为通过（任一档超限即不通过）；
定制兼容测试**须实测其失败时确实阻断放行**（只运行不验证阻断是恒真判据）；
证据包十一项要素缺任一项即不通过。

**F-43 当时未认领受控更新网关。** F-53 已取得使用方授权并冻结唯一范围：首版只交付生产 Authenticode 签名的离线补丁包和客户侧离线验签工具；本仓、本实例与首版均不建设厂商受控在线更新网关。未来网关须另立厂商侧项目与威胁模型，不得在本仓预留隐藏下载、回传或遥测通道。因此这里不再等待范围表态。

#### F-43 当轮未做到、现已关闭的历史清单

- PRD 附录乙 **167 条未决项**是 F-43 当时的历史计数；F-50、F-51 与 F-52 完成后现行真实未决为零，不得据该数字重开决策。
- 82 处能解析的名称式引用**未改为节号**，理由见结论一；
  这意味着 PRD 的引用**仍不可机检**，只是不再指空。
- 新增的四条退出条件里，24、26、27 在 F-43 当时只登记判负口；阶段 14 已补齐可执行工装、证据结构与明确签字责任。工装属于开发内容，专业签字与实机证据属于发布门禁，不阻止开始开发。

### F-44　使用方对 00d 三条卡阶段项的裁定，及其可落与不可落的部分

00d 第二轮增补的一档三条，使用方已直接表态。本卷记裁定、记落点，并如实记哪一条今天落不下去。

#### 决定一　主数据四类档案：**出厂四条默认审批链，各绑一个专属角色码**

使用方选「四类各一条链」。理由面本卷不代述，只记后果：
四类档案业务归口天然不同（客户归销售、供应商归采购、物料与产品归运营），
出厂一条通用链等于在出厂数据上先替客户做了一次权责合并。

**已落**：`05-master-data.md` 的 U-A-08 假定行由「出厂预置**一条**单节点审批链，
审批人取该对象类型的主数据审批人角色」改为四条，并写明原措辞**解析不到任何已冻结的 RoleCode**。

**落不下去的部分，如实记**：本决定的主落点是阶段 4 的默认链集合与角色码冻结表，
而**该落点今天不存在**——实测 F-10 的 C-3 冻结的五个角色码
（`FINANCE_ACCOUNTANT`、`FINANCE_MANAGER`、`SALES_MANAGER`、`PROCURE_MANAGER`、`OPS_DATA_OWNER`）
在 **14 份计划中命中 0、规格中 0、PRD 中 0**。**C-3 本身从未回写到任何地方。**
故决定一的阶段 4 侧须**与 C-3 的回写同批落**，已写入 `00f-f10-writeback-order.md`。

**残留待裁**：四个角色码各挂哪个岗位，本卷不代定——卷内唯一逐字依据只有 PRD:531 一句角色定义，
无可推导取值。

#### 决定二　终止影响面四类人工处置项：**按目标模块的管理者角色展开**

使用方选「按模块管理者展开」形态。**已落** `03-platform-kernel.md` 的接收人解析句，
补入此前缺失的触发源行：

| 处置项类别 | 候选角色 |
|---|---|
| 是否登记销售退货 | `SALES_MANAGER` |
| 已开销项发票作废还是红冲 | `FINANCE_MANAGER` |
| 已下达供应商的采购需求怎么办 | `PROCURE_MANAGER` |
| 在制项目任务收尾还是取消 | **`PROJECT_MANAGER`（本裁定新增）** |

四类一律**只展开候选角色、`assignee_user_id` 留空**，不在出厂数据里写死任何自然人——
与 `03:928` 既有解析口径同形，人员变动不改代码。

**`PROJECT_MANAGER` 是第六个角色码。** C-3 只冻结五个，其中无项目侧角色；
不补则「在制任务收尾还是取消」这一类**无人可派**，而 F-10 逐字
「卡死的表现是合同永远停在 `TERMINATING`，**比不做还糟**」。

**残留待裁**：该角色码挂哪个岗位，同决定一，本卷不代定。

#### 决定三　厂商部署管理通道：**不建通道，只做本地清单导出**

使用方选第三档（非本卷推荐的第二档，记此）。

**本裁定先核准一件事：不建**不与规格冲突**。** 三处措辞逐字均为许可性——
规格:123「厂商**可以**提供」、规格:1264「客户**可选择**启用」，
而规格:1651 明写替代路径「**未启用该通道的客户按支持套餐定期回报**」。
使用方选定的本地结构化导出是该替代路径的**更强形态**（结构化导出而非临时回报）。
**故无须请求规格修订**——这与本卷「规格是要求方不是承接方」的纪律一致：
规格没要求必须建，就不存在让步问题。

**已落** `14-ops-backup-release.md` 三处：交付物 16（版本与补丁清单的本地导出）、
退出条件 28、第 10 节对应关系补第 3.3 与 15.3 章一行。

退出条件 28 特意写成**可判负且带反向锁**：
「**实测本实例不发起任何对外出站连接**（以网络侧观测为准，**不以配置声明为准**）」，
并写明「若日后新增任何回传通道而本条未同批重裁，本条不通过」。

**F-44 当时未解决的一半现已由 F-53 关闭。** 支持套餐合同模板参数固定为 `PATCH_STATUS_REPORT_INTERVAL_DAYS`，允许 1 至 7 个自然日，未另选时默认 7。它是发布前由合同模板选择并签字的商业参数，不是环境变量、数据库配置或代码分支；合同尚未签字不阻止代码开发，发布时未选择则必须采用默认 7。

#### 本轮未做到的

- 决定一的阶段 4 侧**未落**，须与 C-3 回写同批，见结论一。
- 决定一、二各留一处**角色码到岗位的映射**未定，本卷不代定。
- 决定三的支持套餐条款在 F-44 当时未拟；F-53 已冻结参数名、范围、默认值和发布门禁责任，本项已关闭。
- 三条决定**都没有任何机检承接**：`03:928` 的解析表、`05` 的假定行、`14` 的退出条件 28
  都是文档与人工判定项，**本卷不宣称已实现，只宣称已裁定并已落文**。

### F-45　使用方对四条回写前置的裁定，含一次改规格授权

F-10 回写第二批卡在四条前置上，使用方已直接表态。本卷记裁定与落点。

#### 决定一　进项方向**不设作废登记**

**裁定**：采购发票（进项）不跟随 F-10 B-4 开放作废；**进项只有红字冲销，没有作废**。
理由由使用方给定，本卷只记后果：进项发票由供应商开具，**本方无权作废它**，
本方能登记的只有「供应商已开具红字发票」这一事实。

**本裁定对「分次部分红冲」的读法，明标以便纠正**：使用方所答只否定了作废一半，
未直接答部分红冲一半。本卷按**进项跟随 F-10 B-4 允许分次部分红冲**落文——
依据是提问本身就是「要不要跟销项一起开放分次部分红冲」，撤下的是作废而非红冲，
且供应商开具部分红字发票在实务中存在。**若读错，一句话即可改回。**

已落 PRD 八处：`1398` 模块职责、`2217` 角色与单据表、`2250` 高风险动作表、
`2526` 本节三件事、`2535` 节标题（`6.6.3 进项方向的红字冲销与作废登记` → `…的红字冲销登记`）、
`2538` 分录指向、`2539` 规则段整改、`2547` 价格调整路径。

**本裁定造出一个缺口，已登记不代拟**：供应商在开票当月**自行作废**了一张本方已登记的进项发票时，
**既无红字发票可登记、又不属本方登记错误**，本版无承接路径。
已新增附录乙 **U-D-19**（决策人财务负责人），并在 `6.6.3` 正文明标。
不处置的后果写在该条里：该场景只能靠账外处理，**进项税额转出与应付台账两侧都留下无凭据的差额**。

#### 决定二　一张发票**允许多税率**

**裁定**：每个行明细各自带税率。多行明细已由 F-10 B-8 裁为允许，本条补齐其税率一半。
出厂预置六档 13%／9%／6%／3%／1%／0%，**出厂后增删按裁定 A-27 经配置发布通道**（不随版本冻结）。

连带：`10-ar-ap-invoice.md:57` 逐字「一张发票**单税率、单行金额，不做多行明细**」
**已被本条与 F-10 B-8 两面作废**，须同批改——该处尚未改，记为本卷欠账。

#### 决定三　PRD 第 6.16 节的关闭体例：**描述列末尾加括注**

该节表头三列、**无状态列**，节前言逐字「本节不自行取值，统一由后续决策补齐」，
故关闭方式只有删行与加括注两种。使用方选加括注——与附录乙
「已关闭的事项保留在表内，注明关闭方式」同理，**保留历史可追溯**。

本轮按此体例关闭四条：`F-02`（本卷决定二）、`F-06`（F-10 B-8）、
`F-08`（F-10 B-4）、`F-14`（F-10 B-3）。**“该节余 17 条待决”只是本决定作成时的历史计数；F-50/F-51 已完成后续关闭，现行全卷真实未决为零。** 保留事项仍一律按加括注的体例追溯，不得据历史数字重开决策。

#### 决定四　备份代数：**规格硬要求由两代提高到三代**

**这一条是改规格正文，使用方已授权，照 F-08 那次的形式留证。**
（F-08 逐字「裁定方向：规格让步（**本轮由使用方授权改规格**）」；
本轮同形：**由使用方授权改规格第 13.4 章的备份代数取值**。）

已改三处：规格「至少存在**两代**校验通过的有效全量备份」→ 三代；
同句「不再是最后**两代**之一的备份集方可销毁」→ 三代；PRD 第 11 章同句同改。
复核：两份文档「两代」残留各 **0**。

**这条解掉的是什么**：F-10 的 D-2 把降级告警阈值定为「有效全量代数低于 3」，
而规格硬要求当时是两代——**一个恰好合规的部署会永久挂着一个降级窗口**。
这与 D-2 自己刚修掉的 BOOTSTRAP 缺陷是同一形态：D-2 逐字指出
「D 取 14 时任何新部署头 14 天必然 DEGRADED」，**它修了那一支，在这一支上复制了同一个错**。
硬要求提到三代后，告警阈值与硬底线对齐，该面消失。

代价如实记：**落点容量与保留成本上升约一代**，这是使用方选定此档时已知的。

#### 本轮未做到的

#### 补记（2026-08-20，第三批回写核出）：**决定一落 PRD 未落规格，本卷造出一处跨文档矛盾**

**这是本卷自己犯的错，如实记，不掩盖。**

落 F-45 决定一时只改了 PRD 八处，**没有查规格**。实测现状：

| 处 | 逐字 |
|---|---|
| `规格:297` | 「发票作废与红字冲销在应用内登记，**登记范围覆盖销项发票与进项发票**」 |
| `规格:311` 触发条件栏 | 「发票红字冲销或作废登记，**同时覆盖销项与进项两个方向**」 |
| `PRD:2538`（本卷刚落） | 「进项方向只取该事件的红字冲销一支，**作废一支在进项方向不成立**」 |

**规格说进项有作废，PRD 说没有。** 按权威顺序规格在上，则 PRD 那八处失效；
按裁定顺序 F-45 在后，则规格该改。**两读并存即无定论**——这正是本卷一路在他人稿件里判负的那一类。

**后果不止于文字**：`规格:297` 把「红字凭证按红字冲销与作废事件生成」直接挂在它允许的进项作废上，
而若按 PRD 把作废摘出进项方向，**规格允许登记的进项作废在事件-分录表里没有任何映射可用**——
一个可登记却生不出凭证的事件，进项税额与应付两侧无凭据。

**处置（已完成）**：使用方于同日授权改规格，本卷已改两处并复核：

| 处 | 原逐字 | 现逐字 |
|---|---|---|
| `规格:297` | 「发票作废与红字冲销在应用内登记，**登记范围覆盖销项发票与进项发票**」 | 「红字冲销……覆盖销项发票与进项发票；**作废只在销项方向成立**」，并写明理由与裁定号 |
| `规格:311` 触发条件栏 | 「发票红字冲销或作废登记，**同时覆盖销项与进项两个方向**」 | 「红字冲销登记，覆盖销项与进项两个方向；发票作废登记，**只在销项方向成立**」 |

复核：规格内两处旧措辞残留各 **0**，新增方向限定 **2** 处，与 PRD 的
「作废一支在进项方向不成立」一句对齐；规格破表行 **0**。
**决定一的规格与 PRD 两侧现已一致，该矛盾关闭。**

**留一条纪律给后来人**：本次错在「使用方裁的是业务规则，我只改了 PRD 就以为落完了」。
**业务规则的承载面不止一份文档**——凡改 PRD 的业务口径，必须同批 grep 规格是否有对应表述。
本卷此前一路在他人稿件里判负的跨文档矛盾，这次是自己造的，成因与那些一模一样。

#### 第三批回写（2026-08-20）：起草 21 条，通过 6 条，**实际落零**

| 批次 | 通过率 |
|---|---|
| 第二批 | 13 / 23 = **57%** |
| 第三批 | 6 / 21 = **29%** |

**通过率在掉，而这轮落零。** 通过的 6 条全部卡在不可拆组里：
`P-18`／`P-20` 通过而 `P-19` 被驳（工单逐字「三处必须同批」）；
`S-05` 通过而 `S-04` 被驳（`S-04` 只改了规格:311 借方栏、没改贷方栏，**当场借贷不平**——
这正是工单第 4 节高危十例里点名的那一条）。

**通过率下降有具体成因，值得记**：每落一批，后续每条要核对的已落文本面就更大。
第三批的核查员要同时核对前两批已落的 32 条**加上 F-45 改动的进项作废、多税率、备份代数**。
**剩下的条目恰恰是跨文档纠缠最深的那些。**

多轮起草—对抗核对这批条目**已呈递减**。第四轮预期通过率更低，
且每轮都可能像本轮一样，因一条被驳而整组落零。

**故本卷停止再跑起草轮**，改为把三轮驳回诊断里**真正需要财务判断**的部分提炼成
`00g-finance-judgments-needed.md`（九条）。判据是：逐条读驳回理由，
**缺的若是文字功夫则继续起草，缺的若是对这套账的判断则交出去**。
九条全部属后者——例如「作废一张已被核销的发票释放多少」，
F-10 给的公式含红字发票金额，而**作废没有红字发票**，第二个自变量不存在；
这不是措辞问题，是会计口径在裁定里就没定。

- ~~决定二的连带未落~~ —— **本轮已落**：`10-ar-ap-invoice.md:57` 已改为「允许多税率、允许多行明细」并写明原取值被两条裁定两面作废。
  **另核**：同文件 `:14` 的「单税率、单行金额」**不改**——该处逐字是「一张**最小**销项发票（数电、单税率、单行金额、不带影像附件）」，描述的是 T0 的**样例发票**、不是发票模型，允许多税率不要求最小切片必须用多税率。**差一点误改，记此。**
- **决定一对部分红冲的读法是本卷推定的**，非使用方直答，已在决定一明标。
- 四条决定**解掉的是回写前置，不等于回写本身完成**。`B-3+B-6`、`B-4+B-8` 两组
  仍缺 `P-14`…`P-23`、`P-30`…`P-37` 等条的逐字文本，
  而那些条在上一轮**被对抗核以钱账口径错误驳回**，不是缺决定，是缺定稿质量。

### F-46　使用方对 00g 九条财务判断的裁定

`00g-finance-judgments-needed.md` 的九条，使用方逐条表态。本卷记裁定，供 B 簇回写取用。

| # | 问题 | 裁定 |
|---|---|---|
| 一 | 作废一张已被核销的发票，释放多少 | **释放全额转预收** |
| 二 | 进项方向四路来源各怎么处置 | **与销项完全对称**（客户退款↔供应商返款） |
| 三 | 非退货退款的「客户或供应商」从哪来 | **由原款项单带出** |
| 四 | 已全额红冲后再登记，报哪个错 | **先判终态** |
| 五 | 多税率后发票头上的税率行 | **删掉头表税率行**，税额改各行汇总只读 |
| 六 | 红字发票号码唯一性范围 | **同一法人下全库唯一** |
| 七 | 「各自只允许一次」怎么改 | **作废只允许全额一次；红冲允许分次直至冲满** |
| 八 | 作废时各行明细的已红冲数量 | **视为各行全额已红冲** |
| 九 | 比例回滚口径写在哪 | **同一节内分列两式** |

#### 结论一　九条互相自洽，这一点已复核

**第一条与第八条是同一口径的两面**：作废按「整张冲回」处置——
金额侧释放全额、数量侧视为各行全额已红冲。两者若取值相反（例如释放全额但数量视为零），
会在可退数量与可退金额之间造出一个恒久差额。**本轮不存在该问题。**

**第六条与第四条相互加强**：红票号码全库唯一堵住「同一张红票挂两张原票」，
而「先判终态」保证已冲满的票不再接受任何登记——两道闸各守一侧。

#### 结论二　第一条填的是 F-10 自己的一个空

F-10 详本把释放规则挂在「红字冲销**与作废**登记时」，
却只给了 `min(该条目已核销金额, 本次红字发票价税合计)` 这一个公式；
其自查缺陷表又明写该列「在作废行**为空**，代入即除空」。
**同一份裁定的两句话在作废路径上对不上**——不是措辞问题，是那一格的会计口径当初就没定。
本条裁定填的正是这一格。

#### 结论三　第五条连带一处必须同批改

删掉头表税率行之后，**头表的「税额」行必须同批改为各行汇总的只读派生值**，
否则头上可手填税额、行上各自带税率，两者对不上时**没有任何判据会发现**（本卷第三类缺陷）。

#### 结论四　第二条的代价，如实记

「完全对称」意味着进项侧要新增三条业务口径（新增预付、回增原预付、供应商返款路径拒绝），
且 `6.11.3` 末段的挂账来源封闭列举**须同批为预付侧开口**——
此前已落的版本只为预收侧加了一条，**两侧不对称**，本条裁定后须补齐。

#### 本轮未做到的

- 九条只是**判断**，**回写本身尚未做**。B 簇两组约 20 条待据此重起草。
- 第九条属落点选择而非会计判断，本卷按使用方选定执行，**不另行论证**。
- 九条的选项由本卷按会计常规构造，**非从裁定原文读出**——
  这一点在提问时已向使用方明示，记此以免日后被误读为「裁定原有依据」。

### F-47　00h 四处堵点：解掉三处，第二处被一处**新发现的 F-10 内部矛盾**挡住

使用方指示解决 `00h-b-cluster-blockers.md` 的四处堵点。本卷解三处、卡一处，
另**新发现第五处**——它是 F-10 自身的内部矛盾，不是本卷造的。

#### 结论一　堵点一已解：`PRD:2895` 由无条件改条件

原逐字「退款单**必须**与原退货单据勾稽」，其下 `:2899` 逐字
「提交时校验……**不允许静默通过**」——**一条无条件提交闸**，
四轮起草都没人发现它，第四批对抗核才逮到。

已改为条件式：**因退货而发生的**退款仍须勾稽；**非因退货的退款不适用本项**，
其往来方与可退上限按 6.12 由原款项单带出。
**判别项取「有无关联退货单据」**，与 F-46 第三条同源，不另立判别。

#### 结论二　堵点三已解：规格 `:297` 与 `:880` 补入部分红冲的回滚口径

两处原文只写「作废与红字冲销同步回滚剩余可开比例」，
**分次部分红冲开放后该表述不再成立**——一次回滚多少，两条路径不同。

已按 F-46 第九条分列两式：**作废按已开比例减去已回滚比例**；
**分次部分红冲按本次红字金额占原发票金额的比例逐次回滚**。
`:880` 同批把「不因已作废或已冲销的发票被阻断」扩为
「不因已作废、**已全额冲销或已部分冲销**的发票被阻断」。

#### 结论三　堵点四已解：`PRD:2703` 限定辖域，**此处含一次解释，明标**

原逐字「红字冲销或作废登记**不修改**原应收明细条目，**而是**追加一条冲销类明细条目：…」——
无条件、无例外；而 B-6 的释放核销**就在红冲登记的同一事务内修改该条目的已核销金额**。
**三轮四次起草全栽在这一句。**

**本卷的读法**：该句的「不修改」由其后的「**而是**追加一条…」及所列字段限定辖域，
管的是**冲销的记录方式**（追加而非覆盖），不是「该条目任何字段永不变动」。
依据是 F-10 B-6 逐字「在同一事务内**先释放核销、再追加反向条目**」——
**两个动作，一前一后，不是同一个。**

已据此把「不修改」改写为「**不以覆盖原应收明细条目的方式表达冲销**——
其原单据金额、到期日、原始业务日期与账龄口径一律不变」，
并在同处括注辖域限定与释放核销的出处。

**这是一次解释，不是逐字**。原句没有明写辖域，本卷据 B-6 的动作次序推定。
**若使用方读法不同，改回一句话的事**，但那样 B-6 的 PRD 侧将无法落地。

#### 结论四　堵点二未解，因为**新发现第五处堵点：F-10 自身内部矛盾**

堵点二原判是「`P-21` 的前置是 `S-08`，工单把两者列为平行」。本卷起草 `S-08` 时查出更深一层：

| 处 | 逐字 |
|---|---|
| F-10 **B-2** | 总账凭证「**只由三个来源产生**——十类业务事件按事件-分录表的固定映射、期末处理动作、更正凭证」 |
| F-10 **B-3** | 资金单据冲正的「**凭证经 `post_reversal` 生成**」（按原凭证逐行取反） |

**实测 B-2 整段内「冲正」命中 0。** 即：同一份裁定，一半封闭了凭证来源为三个，
另一半引入了一个不在这三个里的新来源。

**这不是本卷造的**：`规格:363` 的三来源封闭句由本卷第一批忠实落 B-2 而来，
矛盾在裁定里**本来就潜伏着**。

**须使用方裁一刀**：资金单据冲正凭证是**第四个来源**（则 `规格:363` 须由三改四），
还是**归入十类事件映射的逆向应用**（则须在 363 明写该读法）。
**本卷不代裁**——两种读法对「首版凭证从哪来」这个封闭列举给出不同答案，
而该列举正是审计与勾稽的判定基准。

#### 本轮未做到的

- **堵点二未解**，须先裁第五处。
- 堵点四的解法**含一次解释**，已明标（结论三）。
- `PRD:2818` 的无条件勾稽**未动**——它是工单点名三处之一（`P-19` 的落点），
  属 B-6 组内条目，不在本卷「解堵点」的范围。
- 四处堵点解掉三处，**B 簇两组仍不能落**——组内条目本身尚未重起草。

### F-48　凭证来源由三个改四个，补入资金单据冲正凭证

F-47 结论四查出的第五处堵点——F-10 的 B-2 把凭证来源封闭为三个，
而同一份裁定的 B-3 引入了不在这三个里的第四个。使用方指示全面解决，本卷据下述依据裁定。

#### 结论一　裁为**第四个来源**，与更正凭证并列

**依据，两条，都可复算：**

一、**第三项本身就不是事件映射。** `规格:362` 逐字「更正凭证：由财务会计在总账内发起，
只用于有来源凭证的重分类更正，**不经事件-分录表**…」——
即三项里已有一项是「不走事件映射的更正机制」。资金单据冲正与它**结构平行**。

二、**「归入事件映射的逆向应用」这个读法站不住。** F-10 B-3 逐字「凭证经 `post_reversal` 生成」，
而 `post_reversal` 按原凭证**逐行取反**，**根本不查事件-分录表**——
与第三项不查表的理由完全相同。把它塞进第一项，等于说一个不查表的动作是「按表的固定映射」。

**故与更正凭证并列为第四项，不归入第一项。**

#### 结论二　落点四处，一处是**对客户的披露句**

| 处 | 改法 |
|---|---|
| `规格:363` | 「只由**三个**来源」→「只由**四个**来源」，补入资金单据冲正凭证并写明并列理由 |
| `PRD:3057` | 同句同改，并指向 6.7.6 |
| `PRD:4465` | 「更正只有**两条**路径」→**三条**（红字冲销、更正凭证、对产生该凭证的资金单据登记冲正单） |
| `PRD:4463` | 第 11.11.10 节**标题**同改 |

**后两处要紧的地方在于它们是第 11.11 节诚实披露**——对客户的公开陈述。
这两处由本卷第一批落 `P-54` 时写入，**当时忠实照 B-2 写了「两条路径」与「三个来源」**；
F-48 之后若不同批改，**披露的就是假信息**。

复核：两份文档「三个来源」残留各 **0**，「更正只有两条路径」全仓残留 **0**，破表行各 **0**。

#### 结论三　这处矛盾的来历，记清楚以免误伤

**不是本卷造的。** `规格:363` 的三来源封闭句由本卷第一批**忠实落 B-2** 而来，
矛盾在 F-10 里**本来就潜伏着**——B-2 整段内「冲正」命中 **0**，
它封闭列举时没有把同一份裁定 B-3 的产物计入。

**但本卷有一处该做未做**：第一批落 `P-54` 时把「两条路径」写进了诚实披露，
**当时没有交叉核对 B-3**。这与 F-45 那次「只改 PRD 没查规格」是同一类疏漏——
**落一条封闭列举时，必须把同一份裁定的其余条目过一遍，看有没有新增项。**

#### 本轮未做到的

- 本裁定只解掉凭证来源这一处；**堵点二（`P-21` 依赖 `S-08`）尚未落**，
  须由 B 簇整体改写承接。
- 「四个来源」是否需要连带改动阶段计划侧的凭证来源枚举，**本卷未查**。

### F-49　B 簇整体改写落地：52 条一次落齐，另留 9 条未决

四轮逐条起草（通过率 100%→57%→29%→13%，后两轮落零）之后，
按 `00h` 自己的诊断改做法：**一次整体改写，不再逐条起草**。本轮落 52 条。

#### 结论一　做法改了什么，以及它为什么管用

| | 前四轮 | 本轮 |
|---|---|---|
| 单位 | 约 20 条各自起草 | **两份独立全稿，各自看全局** |
| 对抗核 | 每条一名核查员 | **四个视角分开核整包**：内部自洽／指引落空／与已落冲突／追不到依据 |
| 定稿 | 逐条 accept/reject | 定稿人**逐条择优并当场修**，只留经得起四视角的 |
| 结果 | 后两轮落零 | **52 条一次落齐** |

两份全稿 38 与 35 条，四视角共查出 **61 与 38 处**问题，定稿留 52 条。

**关键在「先把语义链拉通再动笔」**：`释放核销 → 追加冲销条目 → 台账有效余额 → 账龄 →
预收挂账 → 退款` 这条链上各节互相引用，逐条起草时每个人只看得见自己那一节，
于是反复写出指向「一个还没写的节」或「口径相反的节」的悬空指引——**这正是前三轮的头号驳回理由。**

#### 结论二　前四轮的四个失败模式，本轮逐一实测已消除

| 失败模式 | 本轮实测 |
|---|---|
| `S-04` 只改借方栏，当场借贷不平 | 释放额在规格:311 出现 **4 次**（销项借贷各一、进项借贷各一），该行可判平 |
| `P-21` 把四路压成「一律转预收」 | 6.5.3 逐字「四路各不相同，**不得合并为一路**」，四路逐条分列；进项侧 6.6.3 同构 |
| `P-14`／`P-15` 与已落文本对撞 | 对撞源头「原条目的未核销余额一字不动」**残留 0**，本批同时改掉 |
| 悬空指引 | 定稿自查逐条 grep 现文核对每一处「按 X 节」，`open` 里剔除了三处追不到依据的 |

禁写措辞体检：`WAIVED`／`approval_ref`／SHA-256 冻结／fail-open 在两份文档**各命中 0**；
破表行各 **0**；全量 1112 通过；六道静态门禁各自返 0。

#### 结论三　9 条未决，其中一条又是 **F-10 内部矛盾**

| 1 | B-6 的 origin = REFUND 一路（销项客户退款／进项供应商返款核销）没有可走的出路，与 B-3 的五项冲正原因封闭列举互相封 |
| 2 | 一条应收（应付）明细条目上同时挂多条来源不同的核销关系时，释放额在各条核销关系之间的分摊次序未定 |
| 3 | B-5 第 (5) 条的核销候选与逐条上限由未核销余额改为有效未核销余额，其规格与 PRD 落点未被 B-5 的动到清单列出 |
| 4 | 规格第 17.3 章两句与已落的 PRD 6.9.1／6.9.3、以及本稿的 6.10.1／6.13.1 口径不一致 |
| 5 | 规格:297 与规格:880 把税率与不含税金额、税额、价税合计并列为销项发票开具登记的记录项，未区分头表与行明细 |
| 6 | 进项发票台账（6.6.2）仍是头表单税率、无行明细，与销项侧的多税率多行明细不对称 |
| 7 | 附录乙 U-D-03（发票号码唯一性约束范围）与 6.16 的 F-01 关闭进度不同步 |
| 8 | 「已过账凭证只以红字冲销或更正凭证追加更正」这一封闭列举在第 6、7 两节之外还有五处未同批 |
| 9 | PRD:810 的历史成交资料展示口径枚举「已作废、已红冲与已退货」未覆盖新增的部分红冲态 |

**第 1 条须单说**：F-10 的 **B-6** 对 `origin = REFUND` 一路逐字「提示**先冲正该退款单**」，
而同一份裁定的 **B-3** 逐字把冲正原因锁死五项并明写「**不设第六项**，
即界面上不存在『因发票红冲而冲正到款』这个选项」。
**被提示去走的那条路，提交不出来。** 且该退款款项确已付出，
逐行取反会把资金腿借回而钱并未回账——**犯的正是 B-3 自己用来否掉冲正的同一个毛病。**

本轮据此**只写「拒绝本次登记并列出该款项单据」，不写那条走不通的指引**。
须使用方裁：给冲正补第六项原因、另给该场景一条路径、还是照 F-45 决定一造出 `U-D-19` 的先例登记附录乙。

**这是本卷在 F-10 里查出的第三处内部矛盾**（前两处：B-2 三来源 vs B-3 冲正凭证；
B-6「与作废登记时」vs B-4「该列在 VOID 行为空」）。

#### 本轮未做到的

- 9 条未决全部登记，**未代拟**。其中第 3、4 两条涉及**是否授权改规格第 17.3 章与规格:306／:309**，
  不改则两份文档在同一勾稽项上给两个基准，而该项是关账前强制校验的判定依据。
- `PRD:810` 的历史成交口径枚举未覆盖新增的「部分红冲」态，随该未决项关闭时一并处理。
- B 簇落齐**不等于 F-10 回写完成**——A 簇、C 簇、D 簇的余项仍按 `00f` 台账。

### F-50　财务一致性与发票模型最终收口：F-49 九项全部关闭

> 裁定日期：2026-08-21
>
> 当前状态：已书面批准并冻结为开发口径；尚未开始业务代码实现。
>
> 唯一详细依据：`docs/superpowers/specs/2026-08-21-f50-financial-consistency-design.md`
>
> 唯一执行依据：`docs/superpowers/plans/2026-08-21-f50-financial-consistency-implementation.md`

F-49 上述“另留 9 条未决”是截至 F-49 作成时的历史状态；自 F-50 生效后九条不再属于附录乙待决，也不得继续阻塞阶段 9/10 开发。逐项关闭映射如下：

| F-49 项 | F-50 唯一结论 | 依据 |
|---:|---|---|
| 1 | 退款/返款不是正向核销，而是逐原款项来源链接追加 `RELEASE`；不增加第六种资金冲正原因 | F-50 §3.1、§4.1、§4.2 |
| 2 | 红冲释放额先按 `L = max(0, current_reversal_gross - effective_open_before)` 计算，再按根及组内固定 LIFO 分配 | F-50 §4.3、§4.4 |
| 3 | 所有经营消费者统一使用 `effective_open`；逐行守恒的 `row_open` 不得冒充可核销余额 | F-50 §3.2、§5.1 |
| 4 | 关账按截至期间的 ORIGINAL/REVERSAL/APPLY/RELEASE 追加事件切片重建，后续事件不改写历史 | F-50 §5.2 |
| 5 | 销项发票税率只在行，头三金额由行求和 | F-50 §6.1、§6.2 |
| 6 | 进项发票同样多行、多税率、逐来源行红冲 | F-50 §6.1、§6.3、§6.4 |
| 7 | 法定号码经 `invoice.invoice_number_registry` 在法人内跨四类蓝/红票并发唯一；F-01/U-D-03 同时关闭 | F-50 §7 |
| 8 | 发票事实、资金事实错误、纯总账重分类分别走发票冲销、资金单据冲正、更正凭证；凭证生成来源仍为四类 | F-50 §8 |
| 9 | 部分红冲/退货的历史成交分开返回默认可见与可取价资格；金额更正可见但不可直接取价 | F-50 §9 |

F-10 内部矛盾据此消除：B-3 的五种“错误资金登记”冲正原因原样保留；B-6 的 `origin = REFUND` 及“先冲正该退款单”路径整体废止。真实退款先消耗可追溯预收/预付，剩余部分对原 APPLY 根追加 RELEASE；红冲只处理仍有效的 APPLY 余额，因此不再需要一条提交不了且会虚构资金回流的第六原因。

历史替换标记：F-46 的 `min(已核销金额, 本次红字金额)` 已由 F-50 §4.3 替代；F-48 中“资金冲正凭证总是逐行取反”已由 F-50 §4.2 的动态去向拆分替代。F-48 的“四类凭证生成来源”仍有效。任何旧审计段落若保留上述原句，只能作为带本标记的历史证据，不是实现口径。

本裁定的开发门禁为 F-50 的 **45** 项验收、32 条精确错误码、权威优先级与实施计划；任何阶段计划中的旧单税率、单次红冲、`origin=REFUND`、按 `reverses_id` 推断方向、当前 `open_amount` 反推历史或无名门户回写端口，均视为已被替代。

### F-51　附录乙最后 47 项与开发口径冻结

> 裁定日期：2026-08-21
>
> 当前状态：**已批准并完成权威回写，文档可直接开发；本轮未启动新的业务功能实现，既有骨架与早期迁移不作为完整实现或认证证据。**
>
> 唯一详细依据：`docs/superpowers/specs/2026-08-21-f51-development-readiness-freeze.md`

F-51 关闭 00e 实测的 46 条真实待拍板事项，并对 `U-C-06` 作技术归属裁定，共 47 条。`00d-pending-decisions.md` 与 `00e-appendix-b-full-sweep.md` 自本裁定起只保留历史审计价值，不再是实现依据。F-10 C-3 对 U-A-08 的不完整覆盖由本裁定补齐；F-51 不改判 F-50，全部发票与财务一致性口径仍以 F-50 为准。

#### 一、47 项唯一现行清单

| 编号 | F-51 唯一现行值 |
|---|---|
| U-A-07 | 十一类业务字典经签名配置包可新增、改显示名与停用，已引用编码不可改删；科目类别固定为 `ASSET/LIABILITY/EQUITY/PROFIT_LOSS`；完整出厂编码见 F-51 规格 §3 |
| U-A-08 | 主数据五链分别绑定 `MDM_CUSTOMER_APPROVER`、`MDM_SUPPLIER_APPROVER`、`MDM_MATERIAL_APPROVER`、`MDM_PRODUCT_APPROVER`、`MDM_WAREHOUSE_APPROVER`；采购、销售、信用、财务、配置与报表链按 F-51 规格 §3，申请人不可自审、空链 fail-closed |
| U-A-09 | Excel/CSV 两遍处理：完整静态预校验失败零写入；通过后逐行独立事务，动态失败不回滚成功行；账号/主数据/发票/财务期初上限为 200/5000/2000/2000 |
| U-A-12 | 银行名与账号均密级 30、均加密；账号列表/详情默认末四位，完整查看重新认证并审计；包含任一字段的导出均为敏感导出 |
| U-B-01 | 随产品交付销售、采购、财务、技术、管理运营五个标准角色包；RoleCode 不可改、显示名可改、允许复制派生、不自动绑定自然人 |
| U-B-02 | 岗位显式映射到 U-B-01 RoleCode，不按中文名猜测；责任人、申请人、当前处理人是记录关系，不另建角色 |
| U-B-03 | 合同与发票申请的出厂管理节点为单节点 `ROLE:MANAGEMENT_APPROVER`；首版无金额分档、无部门上级链 |
| U-B-04 | `TECHNICIAN` 录安装调试维修证据并只读无价格合同摘要；最终交付由 `SALES_MANAGER` 或 `PROJECT_MANAGER`，技术角色不可确认收入或读价格/成本/毛利 |
| U-B-07 | 默认记录可见来源为责任人、当前流程处理人、显式共享；创建人无永久权；共享不可转授 |
| U-B-12 | 门户用户计入约 50 个启用命名用户池；同一身份只计一次；每“供应商 + 法人”最多 3 个启用账号 |
| U-B-15 | 审批链最多 10 节点；默认超时 24 小时、每 24 小时重复提醒；不自动升级、通过或驳回，只允许审计化人工转派 |
| U-B-18 | 含敏感字段、对象密级不低于 30、或行数不少于 1000 任一成立即敏感；审计导出始终敏感；XLSX/CSV/PDF，最多 50,000 行 |
| U-C-02 | 收货录入/过账仅 `WAREHOUSE_USER`；最终交付确认仅 `SALES_MANAGER` 或 `PROJECT_MANAGER` |
| U-C-03 | 仓管员是收货登记唯一操作者，采购员不得通过记录关系绕过能力检查 |
| U-C-06 | 仓库唯一归 `mdm`；阶段 5 建 `mdm.warehouses` 并维护审批，阶段 8 只实现 `WarehouseDeactivationCheckPort`；通用地点延期 |
| U-C-09 | 直运退货的“供应商拒绝接受退回”由 `PROCURE_MANAGER` 发起、`FINANCE_MANAGER` 重新认证审批后置位；不可直接撤销，只追加冲回/更正事实 |
| U-C-12 | 个人报表 `DRAFT/ACTIVE/RETIRED`，不走发布；企业报表 `DRAFT/PENDING_APPROVAL/PUBLISHED/DEACTIVATED`，走差异、审批、签名与版本回退 |
| U-D-01 | 无物料服务产品固定 `DIRECT_EXPENSE`；交付只确认收入、不走库存腿；成本只取实际直接费用，未到时毛利标暂估/缺失 |
| U-D-17 | 预付款每行恰引一张采购订单或合同；锁内累计已付、在审和本次不得超过付款计划授权额，无计划时不得超过价税合计 |
| U-D-18 | 生效供应商价自动带出并冻结快照；无价或超价允许但必填原因并置 `price_exception=true`，由既有 `PROCURE_MANAGER` 节点审批 |
| U-E-03 | 信用占用三桶全部按价税合计；已交付未开票查询必须返回含税值 |
| U-E-07 | 信用超额出厂为单节点 `FINANCE_MANAGER`，申请人不可自审 |
| U-F-02 | 补货阈值挂法人 + 仓库 + 物料，字段 `reorder_point/target_stock`；60 分钟扫描、默认关闭，建议量 `max(target_stock-available,0)` |
| U-F-10 | 批准阶段 7 门户白名单并按 F-50 把发票上传改为头 + 行；只开放本供应商、本法人数据，无通用查询、银行字段或导出 |
| U-F-11 | 首版仅邀请开通，自助注册默认关闭；许可证、签名配置、安全审批三者未同时满足不得启用 |
| U-G-01 | 可用量 = 结存 − 已确认/已下达未交付订单剩余量；不建持久预留表，订单确认时按法人仓库物料加锁重算 |
| U-G-07 | 序列号法人内全局唯一；库存来源以 `inventory.serial_states` 为权威，设备不另存第二份权威序列号 |
| U-H-16 | 交付可导入但不自动启用的 17 科目角色参考模板，明确本年利润与未分配利润绑定；按法人导入并由财务主管审批 |
| U-H-17 | 记账日期默认业务日；补记须 `LEDGER_BACKDATE`、重新认证与 `FINANCE_MANAGER` 审批，只能进开放期间 |
| U-H-18 | `OPS_DATA_OWNER` 按法人授予，可兼 `FINANCE_ACCOUNTANT`、不可兼 `FINANCE_MANAGER`，不得审批本人修复 |
| U-I-06 | 首版不缓存、不物化；同实例只读角色实时查询并返回精确到秒的 `data_as_of` |
| U-I-11 | 完全复用 U-B-18 的格式、50,000 行上限和敏感分级 |
| U-I-12 | 完全复用 U-C-12；个人按个人版本，企业按发布版本整体回退 |
| U-J-03 | 与 U-G-07 同值：法人内全局唯一，库存来源序列号以 `inventory.serial_states` 为权威 |
| U-J-04 | 工单六态固定；低代码只能加审批、提醒与时限，不得扩状态或迁移 |
| U-J-07 | 终态不原地重开；完成工单可创建带 `follow_up_of_work_order_id` 的返修跟进，取消工单只能新建独立单 |
| U-J-08 | `EXCHANGE` 强制退货行和替换发货行配对且客户/产品一致，两侧终态后才完成；只退用 RETURN，只补发走独立动作 |
| U-J-13 | 终态派生任务保留；失效未终态任务置 `derivation_stale=true` 并派项目负责人处置；新增义务生成新版本补充任务 |
| U-K-01 | 配置整包原子发布/回退；内容项编辑锁 TTL 1800 秒；首版无单对象发布 |
| U-K-07 | `brand_profiles` 现列集为首版最终范围；开发默认 `Enterprise Platform/Local Development/local.enterprise.platform.*` 且不可发布；生产必须客户正式品牌与签名 |
| U-K-09 | 提供本地上手清单；演示数据只在物理隔离 DEMO 配置档生成/重置，生产硬拒绝且不复制真实数据 |
| U-L-01 | 活跃并发为 60 秒内有请求的不同用户；超过 20 不拒绝业务，只告警并取消 SLA；管理端 5 秒刷新，单用户最多 3 会话 |
| U-L-05 | 默认 5 GB，可按模块/文件类型降低，有效值取最小；范围 1 MB–5 GB，门户硬上限 50 MB；调整走签名发布、安全审批与审计 |
| U-L-07 | 弱网为 5 请求中位 RTT >2 秒或 60 秒两次失败；离线为系统离线或连续 3 次失败；5 秒加密草稿，重连后写请求逐单确认 |
| U-L-08 | Chrome/Edge 当前及前 2 大版本、Safari 当前及前 1、Firefox 当前 ESR；只支持仍获安全更新的 OS，不支持版本硬拦截 |
| U-L-09 | 计划维护提前 7 天、24 小时通知，15 分钟倒计时；切换拒新提交、在途最多 5 分钟；草稿只恢复兼容版本且不自动提交 |
| U-L-12 | 派生读数返回秒级 `data_as_of/indexed_at`；延迟 >30 秒或未知显示非实时提示，超过 15 分钟进入运维告警 |

上述表恰为 47 行，不允许通过合并同源编号把任一编号从状态统计中删除。

#### 二、同批技术冻结

| 技术项 | F-51 唯一现行值 | 旧登记处置 |
|---|---|---|
| BlindIndex | 固定完整 32 字节；`derive_blind_key` 返回 `[u8;32]`，列、测试与跨法人派生同宽 | 附录庚 D-01 与 16 字节/可配置说法关闭 |
| 服务端形态 | Windows Server 原生部署，不使用整机 Hyper-V；F-55 仅允许单次 MCP 插件 Hyper-V utility VM | F-08-2 关闭，F-55 窄例外 |
| 客户端 PoC | 阶段 13a 在 iOS/Android 真机执行薄 PoC；可触发切离 Tauri，不能单独证明 Tauri 全表通过 | 己-3 关闭，完整全表仍在阶段 13 后续验收 |
| 生产签名 | Windows 生产 EXE、DLL、MSI、升级包必须 Authenticode 签名并在安装/升级前验证 | F-08-3 关闭 |
| CI | 唯一入口 `cargo xtask ci`；内网 Forgejo + Woodpecker，Windows agent 执行 | F-08-4 关闭 |
| GRNI | procure 建仅追加效果事实；阶段 10 经 `ep_contract_procure::GrniEffectWritebackPort` 同事务回写，查询只读 procure 自有表并按期间累计 | U-G01-01 关闭 |
| 对账上下文 | `SecurityContext` 第 20 字段为 `system_purpose: Option<SystemPurpose>`，枚举只含 `General/Reconciliation`；四参 `SecurityContext::system(legal_entity_id, request_id, trace_id, purpose)`；无 `ReconContext`；`Reconciliation` 除定义处只可在 `crates/platform/recon/src/executor.rs` 构造并由 `reconciliation-context-confined` 机检，job-worker 只调 `ReconExecutor::run`，越权用 `PLATFORM.AUTHZ.OBJECT_FORBIDDEN` | 戊-11 关闭 |
| F-50 | **45** 项验收、32 错误码、行级发票、中央号码、效果核销、`effective_open`、历史切片和门户受理均保留在首版 | F-51 不缩减、不替代 F-50 |

#### 三、只阻塞发布或专业签字，不阻塞编码

门户白名单与银行字段须安全签字；参考科目表及 U-D-01/U-D-17/U-D-18/U-E-03/U-H-17 须专业财务/采购签字；客户正式品牌、商店主体与 Authenticode 身份须在生产打包前提供；浏览器实际最低版本、维护窗口与通知属于每次发布/部署门禁；自助注册只有未来启用时才重走安全审批。上述门禁不得重新登记为开发待决项。

### F-52　最后五项内部开发阻断收口

> 裁定日期：2026-08-21
>
> 当前状态：**已批准并完成权威回写，文档可直接开发；本轮未启动新的业务功能实现，既有骨架与早期迁移不作为完整实现或认证证据。** D-02、D-03、X-3、X-1、X-2 均已有唯一实现口径。

本裁定不新增服务器、队列产品、数据库实例、常驻连接或产品进程。F-50 已关闭己-4、己-6，F-51 已关闭 D-01、戊-11；本节关闭剩余五项后，附录庚二的真实未决数为零。旧编号 `X1`、`X2` 自本裁定起统一写作 `X-1`、`X-2`。

#### 一、D-02：自动测试固定为九套，端口中立、实现归属主

1. `ep-platform-release` 定义中立 SPI：`AutotestSuiteId`、`ConfigAutotestSuite`、`ConfigAutotestRegistry`、`ConfigAutotestExecutor` 及输入/报告 DTO；`AutotestSuiteId` 的唯一取值为 `SCHEMA_VALIDATION`、`IMPACT_ANALYSIS`、`RLS_MATRIX`、`ROLE_PREVIEW`、`FLOW_SEMANTICS`、`REPORT_PERMISSION`、`CAPABILITY_MATRIX`、`SOD_CHECK`、`RULE_SEMANTICS`。F-21 的 `RULE_SEMANTICS` 保留；F-10 已撤销的 `COMPENSATION_POLICY` 不得恢复。
2. 实现归属固定为：`ep-platform-meta` 实现 `SCHEMA_VALIDATION`、`IMPACT_ANALYSIS`、`CAPABILITY_MATRIX`、`RULE_SEMANTICS`；`ep-platform-authz` 实现 `RLS_MATRIX`、`ROLE_PREVIEW`、`SOD_CHECK`；`ep-platform-flow` 实现 `FLOW_SEMANTICS`；`ep-app-reporting` 实现 `REPORT_PERMISSION`。四个属主 crate 单向依赖 `ep-platform-release`；`ep-platform-release` 不反向依赖任何属主 crate。九个实现只在 `apps/job-worker/src/wiring/autotest.rs` 以 trait object 注入；启动时按九项精确集合断言，缺项、重项或额外项均拒绝 job-worker 启动。
3. `SKIPPED` 只允许在配置包的 `ItemKind` 与该 suite 的适用集合交集为空时产生。按 F-56 终态回写，`SCHEMA_VALIDATION` 与 `IMPACT_ANALYSIS` 覆盖全部 20 种；`RLS_MATRIX` 覆盖 `CUSTOM_OBJECT/CUSTOM_FIELD/CUSTOM_RELATION/CUSTOM_VIEW/MCP_CONNECTOR/MCP_MANIFEST_VERSION`；`ROLE_PREVIEW` 覆盖三个 `AUTHZ_*`；`FLOW_SEMANTICS` 覆盖 `FLOW_DEFINITION`；`REPORT_PERMISSION` 覆盖四个报表类；`CAPABILITY_MATRIX` 覆盖 `CUSTOM_OBJECT/CUSTOM_FIELD/UI_LAYOUT/RULE/MCP_CONNECTOR/MCP_MANIFEST_VERSION`；`SOD_CHECK` 覆盖 `AUTHZ_ROLE/AUTHZ_POLICY`；`RULE_SEMANTICS` 只覆盖 `RULE`。其余适用 suite 必须产出 `PASSED` 或 `FAILED`；`LICENSE_GRANT|MODULE_PACKAGE` 除前两套外只能产生合法 `SKIPPED`，不能跳过前两套。
4. 每个 suite 使用自己的只读事务；`RLS_MATRIX` 与 `ROLE_PREVIEW` 用 `REPEATABLE READ`，其余七套用 `READ COMMITTED`。某套语义断言失败立即把该套标为 `FAILED`、不重试，但执行器继续运行其余适用套件以形成完整报告。可重试基础设施错误按共享八步退避重试；耗尽后把未完成套件标为 `FAILED`，报告只留清洗后的错误，包进入 `TEST_FAILED`。九套均为 `PASSED` 或合法 `SKIPPED` 时才进入 `TEST_PASSED`。

#### 二、D-03：数据库行直接承载耐久任务，不新增事件

1. `platform_meta.config_packages` 是队列载体，追加 `active_autotest_batch_id uuid`、`autotest_attempts smallint not null default 0`、`autotest_available_at timestamptz`、`autotest_locked_by text`、`autotest_locked_until timestamptz`、`autotest_last_error text`。包状态为 `PENDING_AUTOTEST` 时 batch 与到期时间必非空；离开该状态时租约与到期时间必为空；`locked_by/locked_until` 必须同空或同非空。`platform_meta.config_autotest_runs` 追加 `batch_id uuid not null`、`state`（`QUEUED/RUNNING/FINISHED`）与逐套件 `available_at timestamptz not null`；`failure_count smallint not null default 0` 且取 0 至 9，`outcome` 在 `FINISHED` 前可空，唯一约束改为 `(config_package_id, batch_id, suite)`。包级 `autotest_available_at` 只是当前 batch 所有未完成运行行 `available_at` 的最小值，用于领取索引；逐套件何时可重试只以运行行字段为准，不得从包级字段反推或让尚未到期的套件提前执行。
2. `POST .../actions/run-autotest` 在一个事务内锁住 `DRAFT` 包、生成 batch UUID、把包置 `PENDING_AUTOTEST` 并写队列字段、插入恰好九条 `QUEUED` 运行行（九行 `available_at` 与包级 `autotest_available_at` 均取同一数据库当前时刻）、写审计后提交，响应返回九个 `run_id`。
3. job-worker 轮询包级 `autotest_available_at` 已到期且租约为空或已过期的 `PENDING_AUTOTEST` 包，以 `FOR UPDATE SKIP LOCKED` 领取一个 batch；空闲轮询从 200 ms 退避到 2 s。所有到期、租约和退避比较的数据库当前时刻固定取 PostgreSQL `clock_timestamp()`，不得取 worker 操作系统时钟或事务起点固定的 `now()`。租约不再留给落码选择：`apps/job-worker` 内部常量 `AUTOTEST_LOCK_LEASE_SECONDS=60`、`AUTOTEST_LOCK_HEARTBEAT_SECONDS=20`，不新增配置键。每次成功领取使 `autotest_attempts` 加一，设置全局唯一的 worker instance id 与 60 秒租约，并在同一领取事务中把该 batch 的旧 `RUNNING` 行恢复为 `QUEUED`、把这些恢复行的 `available_at` 置数据库当前时刻，再把包级 `autotest_available_at` 重算为未完成运行行的最小值；`FINISHED` 行永不重跑。执行器只领取 `state='QUEUED' AND available_at <= clock_timestamp()` 的运行行。执行 suite 时每 20 秒按 package id、batch id、`PENDING_AUTOTEST`、`locked_by` 与未过期租约做条件续租；续租或任何 run/最终状态写入影响零行时，立即回滚当前只读事务并停止该 worker，陈旧 worker 不得覆盖新持有者。系统任务上下文固定使用 `SecurityContext::system(..., SystemPurpose::General)`，不得使用 `Reconciliation`。
4. 基础设施失败不占着 worker 线程睡眠：首次失败后依次以 1 秒、5 秒、30 秒、2 分钟、10 分钟、30 分钟、1 小时、2 小时把**该运行行**的 `available_at` 写成数据库当前时刻加对应退避、放回 `QUEUED`、`failure_count` 加一，并把包级 `autotest_available_at` 重算为当前 batch 全部未完成运行行 `available_at` 的最小值；继续执行已经到期的其他 suite，尚未到期的行不得执行。本轮无已到期可执行行时清租约并提交，由后续领取在包级最早到期时恢复。这八个时间点各对应一次重试；第八次重试仍失败时 `failure_count=9`，该行置 `FINISHED/FAILED`，不再产生第九个退避。`autotest_last_error` 只存最新清洗后基础设施错误，新 batch 受理时清空。语义失败不写本字段。
5. 包的终态更新只在同 batch 九行全部 `FINISHED` 后发生：任一 `FAILED` 则 `TEST_FAILED`，否则九行全为 `PASSED` 或合法 `SKIPPED` 才是 `TEST_PASSED`；更新同时清空租约与 `autotest_available_at`。不为此新增 Outbox、事件类型、表、进程或外部队列；阶段 13 的事件目录按 F-54 精确保持三个具名 `platform.custom_record.*.v1` 事件。数据库包行就是派发事实与恢复点。

#### 三、X-3：两个工具从阶段 1 占位，阶段 14 才交付

`tools/bench` 与 `tools/release-gate` 自阶段 1 起就是 workspace 成员和非产品工具骨架，始终排除在产品制品与产品 SBOM 之外。阶段 14 完成真实功能之前，调用任一工具必须以 `EXIT_NOT_DELIVERED = 70` 退出，禁止空壳返回 0。阶段 1 的 `xtask sbom` 正向用真实产品 SBOM 断言两个包名不存在，负向用人工注入包名的夹具断言门禁失败；阶段 14 完成功能后，只有真实命令成功才返回 0。两个工具可从阶段 8 并行实现，但功能交付、证据与发布责任仍归阶段 14。

#### 四、X-1：复用 30 秒采样器恢复三态结论与暴露窗口

1. 保留 C-22 对专用子系统的删除：不恢复 `replication_crosscheck_runs` 表、端点、指标、专属配置键、专属连接或新进程。core-server 复用 `EP__OPS__WAL_RETENTION_SAMPLE_PERIOD_SECONDS=30` 与既有只读分析池，在同次采样读取 `pg_replication_slots`、`pg_stat_replication`，并与 `platform_ops.replication_reports` 的最新有效状态比较。
2. 比对键不留给实现方自定。两个写出进程建立复制连接时必须把 PostgreSQL `application_name` 分别设为 `archive-writer`、`backup-writer`。数据库侧槽集合是 `pg_replication_slots` 全部行的 `slot_name`（未登记的逻辑槽也属异常），会话集合是 `pg_stat_replication` 全部行的 `(pid, usename, application_name)`。报告侧只用 `outcome='OK'` 行，按 `(occurred_at, report_id)` 确定同一对象最后一条：`SLOT_CREATED/SLOT_INVALIDATED` 以 `(writer_process, db_role, slot_name)` 得到活动槽名集合，`CONN_ESTABLISHED/CONN_CLOSED` 以 `(writer_process, db_role, backend_pid)` 得到 `(backend_pid, db_role, writer_process)` 活动会话集合。映射只允许 `archive-writer↔ep_archiver`、`backup-writer↔ep_backuper`；交叉组合是 `MISMATCHED`。槽事件缺 `slot_name`、连接事件缺 `backend_pid`、三个输入中任一次查询未完整，均是 `NO_RESULT`；`spooled=true` 不改变上述按 `occurred_at` 的顺序。
3. 每轮必须得到三态之一：两组槽名集合与两组会话三元组都精确一致为 `MATCHED`；任一侧有另一侧不存在的槽或会话、或出现非法进程/角色映射为 `MISMATCHED`；查询超时、错误、无权限或任一输入不完整为 `NO_RESULT`。`MATCHED/MISMATCHED` 把连续无结果计数归零；`MISMATCHED` 同轮告警并审计。连续第二个 `NO_RESULT` 打开 `REPLICATION_CROSSCHECK_NO_RESULT` 暴露窗口，写出进程与归档/备份继续运行；下一次 `MATCHED` 或 `MISMATCHED` 关闭该窗口。
4. 状态复用 `platform_ops.archive_channel`，追加 `replication_check_last_outcome text null`（CHECK 三态）、`replication_check_last_at timestamptz null`、`replication_check_no_result_streak smallint not null default 0`（CHECK 非负）、`replication_check_last_error_code text null`（只存清洗后的代码）。不新增表。`REPLICATION_CROSSCHECK_NO_RESULT` 是 F-52 冻结时的第十九项；F-53 当时追加的第二十项是无关的 `VIRUS_SCANNER_NOT_AVAILABLE`，这只是历史编号。F-55 终态再追加 `LEGAL_ENTITY_KEY_DOMAIN_UNAVAILABLE` 为第二十一项；实际落库必须保留阶段 2 首次建表的三项枚举/CHECK，并仅由阶段 14 的 `V20261023092500` 扩为完整 21 项，本节不新增第二套定义。旧的“F-53 后 20 项即终态、阶段 2 一次性落完整 20 项”不可实施。

#### 五、X-2：拆出写入角色启动阻断项，不混用 severity

`offsite-sink-requirements` 保持基线 `Degrading`，只含原前七项。阶段 14 另注册 `writer-role-containment`，severity 为 `Blocking`；它只适用于 archive-writer 与 backup-writer，并在建立任何复制连接前检查三项：凭据引用与 NTFS ACL 只授权对应服务虚拟账户且不得供人使用；`ep_archiver/ep_backuper` 的 `pg_hba` 只有回环放行证明；四类 IPC 上报路径与 X-1 周期核对已具备。其他进程返回 `NotApplicable`。任一项失败，该写入进程以 78 退出且不得启用对应角色；core-server 根据进程未投入运行打开不可抑制的 `WRITER_NOT_IN_SERVICE`，不新增 kind。角色启用后的运行期 `NO_RESULT` 只走 X-1，不反向触发该启动阻断项，也不停止写入进程。

#### 六、关闭与开发状态

| 编号 | 状态 | 唯一承接 |
|---|---|---|
| D-02 | 已关闭 | 九套 SPI、属主实现、精确注册与失败语义见本裁定第一节 |
| D-03 | 已关闭 | `config_packages` 耐久领取行与九条运行行见第二节；事件仍为十项 |
| X-3 | 已关闭 | 阶段 1 工具骨架返回 70，阶段 14 才允许真实成功返回 0 |
| X-1 | 已关闭 | 30 秒共享采样、三态、`archive_channel` 状态与第十九个 kind；F-53 后总数为 20 |
| X-2 | 已关闭 | 独立 `writer-role-containment` Blocking 自检项 |

以上五项已获得唯一实现值，不再等待落码选择。Windows 实机、Authenticode、真实沙箱、性能/恢复/渗透测试及专业签字仍是发布或认证门禁，不阻止按本裁定开始编码。

### F-53　阶段 14 历史迁移、补丁分发、支持套餐与病毒扫描部署收口

> 裁定日期：2026-08-21
>
> 当前状态：**已批准并完成权威回写，文档可直接开发；本轮未启动新的业务功能实现，既有骨架与早期迁移不作为完整实现或认证证据。** 本裁定关闭 F-43 结论三、结论四与 F-44 决定三留下的三个阶段 14 缺口，并登记同日追加批准的病毒扫描部署唯一口径。

本裁定优先保持低成本、高保密、单机 Windows Server 原生与纯本地业务运算。它不新增服务器、云依赖、常驻产品进程、外部队列、目标数据库直写账号或隐蔽出网通道。云服务器可以作为客户选择的部署机器或经批准的服务器之外备份落点，但不是首版在线更新、遥测或数据处理的必需依赖。

#### 一、首版补丁只走签名离线包，在线网关明确不在本仓范围

1. 首版补丁分发唯一形态为生产 Authenticode 签名的离线补丁包及客户侧离线验签工具；补丁包、清单、SBOM、签名摘要、兼容性结论、升级与回退证据进入同一发布证据包。
2. 本仓、本实例与首版交付范围均不建设厂商受控在线更新网关、自动下载器、回传代理或遥测隧道，也不得以“未来扩展点”为名预留未声明的域名、端口、计划任务或系统服务。客户实例的版本与补丁状态只做本地结构化导出，由人工携出。
3. 未来如需在线更新网关，必须另立厂商侧项目、独立威胁模型、数据流与密钥边界、运营责任及客户启用协议；该未来项目不属于本仓首版，也不构成本仓开发依赖。

#### 二、支持套餐冻结为发布前合同参数，不阻塞代码开发

1. 合同模板参数固定名为 `PATCH_STATUS_REPORT_INTERVAL_DAYS`，取值域为 1 至 7 个自然日，未另选时默认 7。参数控制未启用部署管理通道客户的版本与补丁状态人工回报周期。
2. 该参数是合同/支持套餐的商业选择，不是软件配置：不得新增环境变量、数据库列、功能开关或按套餐分叉的程序行为。软件只交付本地状态导出与离线补丁验签能力。
3. 代码可以在合同签字前直接开始开发。发布门禁只检查客户合同是否明确选择 1 至 7；未选择时合同模板必须落默认 7 并由有权人签字。支持等级、价格与服务响应承诺仍由合同模板选择，不改变代码路径。

#### 三、旧系统历史数据迁移由阶段 14 的本地工具与受控 API 完整承接

1. 新工具固定名 `ep-data-migrate`，位于 `tools/data-migrate/`，与 DDL 工具 `ep-migrate` 完全分离。它随产品交付、按需运行、不注册 Windows 服务、不监听端口、不持目标 PostgreSQL 凭据、不直写任何业务表；生产 PE 必须 Authenticode 并进入产品 SBOM。
2. 来源只允许四类：XLSX/CSV、只读 ODBC、本地或 SMB 文件清单、经签名模板逐项批准的 HTTPS GET API。来源凭据只引用 Windows Credential Manager 条目名；模板、命令行、日志、错误队列和数据库均不得保存凭据或来源原文。
3. 签名 TOML 模板 schema 版本固定为 1，只允许阶段 14 第 4.12 节列出的八类声明式清洗操作，并必须逐字段显式映射 `legal_entity_id`、`security_level`、`key_domain_id`、`retention_policy_code`；缺任一项整批不得批准。工具每块最多 1000 行且规范化 JSON 请求体不超过 524288 字节，两者先到即封块，含 HTTP 封套的单请求仍不超过全局 1 MiB；超大单记录拒绝，大附件只传既有附件流水线可消费的批准文件清单引用。工具只从签名部署清单读取员工 API 的唯一 HTTPS origin，经第三方反向代理使用最多有效 10 分钟的一次性会话调用公开迁移 API。必须校验证书链、证书主机名与清单 host，禁止重定向、回环地址、直连 core-server:8080、命名管道或命令行/模板自填 URL。
4. `MigrationObjectKind` 是阶段 14 第 4.12 节列出的 25 项封闭集合，归属 11 个现有 `ep-app-*` crate。中立端口固定为 `ep_platform_obs::data_migration::MigrationModuleWriter`，只含 `validate`、`apply`、`reconcile_projection`、`plan_reversal`、`apply_reversal` 五方法；各模块实现必须复用本模块唯一权威写入者。core-server 注册表按 25 项精确断言缺项、重项与错属主均失败。
5. F-53 首次冻结四张带 `legal_entity_id` 的迁移台账：`platform_ops.data_migration_batches`、`data_migration_records`、`data_migration_reconciliations`、`data_migration_known_differences`；后续证据图加固新增 `data_migration_approval_evidences` 与 `data_migration_writer_receipts`。现行六表全部 ENABLE、FORCE RLS，复合外键带法人，不进入 `unpoliced_table_registry`；连同十七张部署级台账，阶段 14 第 3 节为二十三表、五视图、二十八条迁移；再计入阶段 13c 的部署级 `ai_model_packages`，全仓 platform_ops 终态为二十四表五视图。`data_migration_known_differences` 以同一行的 `row_version` 承载 `PROPOSED → APPROVED|REJECTED`、`APPROVED → REVOKED`，每次变更写审计；决定事实进入批准证据表，writer 效果进入回执表，不建立缺少稳定聚合键的状态事件伪表。
6. 批次状态机唯一为 `DRAFT → APPROVED → TRIAL_RUNNING → TRIAL_FAILED|TRIAL_PASSED → SOURCE_FROZEN → APPLYING → DELTA_CATCHUP → RECONCILING → READY_FOR_CUTOVER → CUTOVER_COMPLETED`，并保留 `REVERSAL_PENDING → REVERSED` 与从允许状态进入 `CANCELLED` 的封闭分支。完整试运行不产生正式业务记录、文件对象、业务领域事件或 Outbox，但必须写迁移运行摘要与审计；正式写入按块在同一事务提交业务追加记录、迁移记录、既有领域事件、审计与 Outbox。阶段 14 的平台工作流与部署状态只写 `platform_ops`、`platform_audit` 和既有指标，新增平台 Outbox 事件固定为 **0**，不得为填事件信封伪造“系统法人”；历史迁移只复用各模块已登记领域事件。
7. 连续两次完整试运行仍不收敛时不得自动继续，只能由数据责任人与模块责任人作三选一决定：修订并重新签名模板、缩小迁移范围、改为只迁期初和未结事项。正式写入前必须取得源系统只读冻结证据；冻结后的变化只经 watermark 增量追平。切换前必须完成计数、金额、关系、附件、哈希及三类强制不变量对账；借贷平衡与库存守恒不得批准差异，其他差异只允许落入阶段 14 的封闭类别并经数据、模块、财务三方批准。
8. 整批冲销必须在正式应用前生成可执行计划；执行时只调用各模块现有的更正或冲销追加路径，不 UPDATE 覆盖、不 DELETE。迁移任务由 job-worker 的 `DataMigrationExecutor` 使用 `SecurityContext::system(..., SystemPurpose::General)` 执行；它不新增 `ReconContext`、`ReconCheck` 或 `ReconRunKind`，不扩大 F-51 对 `SystemPurpose::Reconciliation` 的封闭构造边界。

#### 四、病毒扫描只取 NONE 或同机 CUSTOMER_ICAP

1. 基础产品只内建 `TYPE_SNIFF` 与 `STRUCTURE` 两项检查，不交付 CLAMD、病毒引擎、病毒库或病毒库更新通道。部署必须显式填写 `virus_scan_mode=NONE|CUSTOMER_ICAP`，无默认值、自动探测或第三分支；原 `builtin_only`、`CLAMD_SOCKET`、插件模块码与远端扫描地址全部撤销。
2. NONE 时两个内建检查通过即可发布，但必须写 `VIRUS_ICAP/SKIPPED/MODE_NONE` 证据，持续打开不可抑制的 `VIRUS_SCANNER_NOT_AVAILABLE` 窗口，并在健康页、运维中心、交付说明与合同模板逐字写“平台未提供病毒防护”。该句与模式由产品负责人纳入对外表述清单并签字。
3. CUSTOMER_ICAP 的产品侧链路只允许 core-server 经 `\\.\pipe\ep-integ` 调用 `virus_scan.begin.v1`、`virus_scan.chunk.v1`、`virus_scan.end.v1`、`virus_scan.abort.v1`；普通帧与 `BoundedChunkStreamV1` 的分块、ACK、背压、长度、哈希和超时只取配置登记的唯一契约。integration-gateway 再作为客户端连接客户自管的同机回环 ICAP 扫描器，URL 条件必填，只允许 `icap://127.0.0.1:<port>/<service>` 或 `icap://[::1]:<port>/<service>`；禁止主机名、DNS、系统代理、重定向与非回环地址。明文有界流式转发、不在 integration-gateway 落盘、不离开服务器；gateway 不持数据库、KMS 或业务文件权限、不消费 Outbox，产品不新增 ICAP 或 HTTP 监听口。它是阶段 1 禁止本机回环 TCP 规则的唯一窄例外，不得扩到其他协议、进程或目的端。
4. CUSTOMER_ICAP 只有 CLEAN 才允许附件发布；INFECTED、超时、不可达、协议非法或未知响应一律隔离，禁止引用、下载和发布，打开同一不可抑制窗口，不得自动回退 NONE 或人工绕过。下一次健康探测和真实扫描样本都成功后才关窗，既有隔离件仍须重新扫描。客户扫描器的产品、版本、病毒库更新、许可与误报漏报责任进入部署证据、交付说明和合同。
5. `platform_ops.deployment_records` 增加 `virus_scan_mode` 与条件可空的 `virus_scan_icap_url`。F-53 当时把 F-52 的十九项追加 `VIRUS_SCANNER_NOT_AVAILABLE` 成为第二十项；该历史计数已被 F-55 取代，不可作为施工终态。现行终态由阶段 14 的 `V20261023092500` 在三项 Stage 2 基线上扩为 21 项，第二十一项为 `LEGAL_ENTITY_KEY_DOMAIN_UNAVAILABLE`；本项不新增平台 Outbox 事件、常驻产品进程、云依赖或外部数据处理。

#### 五、离站副本防删与勒索恢复信任边界

1. 全部备份、归档、附件、水位、manifest 与探针对象使用内容/批次唯一且不可复用的 key，只允许 CREATE_NEW；对象存储带 `If-None-Match: *`，目录后端采用等价排他创建。已有 key 即失败，writer 不得靠覆盖或复用 key 重试。
2. 落点凭据冻结为三套互斥身份：writer 只可列举、创建新对象和必要校验读，严禁删除、覆盖、重命名、版本清理、ACL/策略/生命周期管理；restore 是平时封存的独立只读身份；disposal 是第三身份，只在双人审批与重新认证后临时解封，并只删除批准清单中的精确 key/版本。三账户、credential ref 与可代入组必须两两隔离。
3. OBJECT_STORAGE 以 IAM 显式 deny 和覆盖/删除/策略修改负向探针验收；另一台 Windows/SMB 目录以独立服务账户、DACL deny DELETE、DELETE_CHILD、WRITE_DAC、WRITE_OWNER、既有文件写入以及 `CREATE_NEW` 负向探针验收；NFS 只有服务端 NFSv4 ACL 或等价机制能把 ADD_FILE 与 DELETE、DELETE_CHILD、WRITE_ACL、WRITE_OWNER、改属主和既有文件写入分离才合格。普通 POSIX/NFS 可写目录若 writer 同时可删除、重命名或改权，即使写入正常也不满足保护门。
4. `platform_ops.offsite_sinks` 登记三身份标识、append-only attestation 时点/证据和负向探针结论。配置缺项、身份复用、策略证据缺失或任一负向探针未被拒，均打开不可抑制 `OFFSITE_COPY_PROTECTION_MISSING`，`RG-OFFSITE-COPY-PROTECTED` 失败；只有全量重验通过才自动关窗。写出、读回和获批恢复继续按实际能力运行，不把保护缺失伪装成不可写。
5. 本控制不是 WORM、对象锁或不可变存储；客户存储管理员、云根账户或另一台机器本地管理员仍可绕过。交付说明与客户合同必须披露该剩余风险，对外表述签字不得使用“不可删除”“不可变副本”或同类承诺。

#### 六、权威回写与门禁分界

| 历史登记 | F-53 现行处置 |
|---|---|
| F-43 结论三“历史迁移只有判负口、今日必然失败” | 被阶段 14 第 4.12 节、测试计划和退出条件 27 完整替代；工装可直接开发 |
| F-43 结论四“受控更新网关不认领、等待范围表态” | 首版明确不建设；未来另立厂商侧项目，不是本仓依赖 |
| F-44 决定三“支持套餐条款未拟” | 参数名、1 至 7 范围、默认 7 与签字责任已冻结 |
| 00d 第 24 项“病毒扫描引擎待表态” | 不内置引擎；部署必答 NONE/CUSTOMER_ICAP，NONE 具名降级并披露，CUSTOMER_ICAP 失败隔离且不回退 |
| 离站副本访问控制只写“writer 可写、恢复可读” | 三身份隔离、不可复用 key、CREATE_NEW、逐后端 deny 与负向探针；保护缺失开不可抑制窗口并阻止发布 |
| A-24 只有四条期初通道 | 四条期初入口继续保留；旧系统完整迁移由阶段 14 编排并复用模块权威写入者 |
| A-26 与总览中的十七表五视图 | 阶段 14 第 3 节更新为二十三表五视图（十七张部署级台账加六张法人 RLS 迁移台账）；再计入阶段 13c 的 `ai_model_packages`，全仓 platform_ops 终态为二十四表五视图 |

实现可以据上述唯一值立即开始。不得以尚未取得客户迁移模板、客户源冻结证明、客户 ICAP 扫描器、支持合同签字、产品负责人对外表述签字、Windows 实机、生产 Authenticode 证书、真实沙箱或性能/恢复/渗透报告为由阻止编码；这些分别是客户实施、发布、认证或专业签字门禁，必须在相应试迁、附件发布、切换或发布前取得。客户没有 ICAP 时采用 NONE 并强制降级披露；客户选择 CUSTOMER_ICAP 后扫描器缺失或不可用只阻止附件发布，不恢复任何设计分支。

### F-54　全局登记闭合与合同终止影响面平台补齐

> 裁定日期：2026-08-21
>
> 当前状态：**已批准并完成权威回写，文档可直接开发；本轮未启动新的业务功能实现，既有骨架与早期迁移不作为完整实现或认证证据。** 本裁定不改变 F-10 已批准的七条业务处置含义，只补齐其在阶段 3 缺失的平台本体，并把全局登记从“有目录文件”收紧为“现行引用差集为零”。

#### 一、登记文件与现行引用唯一闭合

1. 五份权威登记固定为 `docs/error-codes.md`、`docs/event-catalog.md`、`docs/metrics-catalog.md`、`docs/data-dictionary.md`、`docs/impact-catalog.md`。代码常量、注册器、CHECK、阶段正文与五份登记逐项一致，不能以阶段自称总数或未命名配额替代具名项。
2. 阶段 3 旧“17 个事件”撤销为三个具名增量；阶段 13 旧“10 个事件”撤销为三个具名 `custom_record` 事件；阶段 14 新增平台 Outbox 事件固定为 0。事件目录只登记真实名字，不保留配额。
3. 环境变量与 dotted key 必须双向可追溯；现行 `EP__...` 引用（撤销键、通配前缀与历史裁定除外）不得缺登记。指标同理，现行 `ep_*` 指标引用（数据库角色、schema、crate、测试夹具与明确作废别名除外）不得缺目录。
4. 错误码只允许精确现行码；`SELF_APPROVAL` 旧名统一为 `PLATFORM.SOD.SELF_APPROVAL_FORBIDDEN`，`TOKEN_REQUIRED` 旧名统一为 `PLATFORM.AUTHZ.REAUTH_REQUIRED`。历史 F-10/F-50 旧码只作替代追溯，不得复活为第二套。CI 输出 referenced-minus-registered 与 registered-duplicate 两个差集，二者都必须为 0。

#### 二、阶段 3 一次交付完整影响面平台

1. 新增 `platform_core.impact_assessments` 与 `platform_core.impact_disposition_items` 两张法人 RLS 表；后者显式保存 `decision_code`、`decision_reason`、`decision_result_doc_id`，非人工项三字段全空，人工 DONE 时 code/reason 必填，result id 按目录逐码必填或必空。表、索引、CHECK 与列集以阶段 3 第 3.3.4 节和数据字典为唯一实现口径。
2. `ep-platform-impact` 交付 `ImpactRule`、`ImpactRegistry`、`ImpactAssessor`、`ImpactAssessmentQuery::by_source`、`ManualImpactDecision` 与 `ImpactDisposeOutcome::{Completed,AlreadySatisfied,NeedsManualDecision}`。`dispose` 接受调用方同一个 `&mut dyn Tx`、`SecurityContext` 与 item；规则必须锁后验证法人、来源关系、目标关系与状态。reason 是非空稳定码/清洗文案。
3. `docs/impact-catalog.md` 恰七条，全部上游固定为 `clm.contract.terminated.v1`；真实注册数按阶段 3/6/7/10/12 为 0/3/4/6/7。未接线类别确实建立目标为空、`attempts=0` 的 PENDING 占位项且不计入完成；真实规则接线后返回空集合时，同一目录行以目标为空、`outcome_reason=NO_APPLICABLE_TARGET` 的 DONE 终态闭合并计入 `item_done`。两种空目标形状都不得带租约、错误、流程或人工字段；除此之外的 DONE/DEAD/DISPATCHING 必须有真实目标。不得没有行、Noop 或在真实规则接线前直接 DONE。
4. 唯一消费者 `platform.impact_assess` 以 inbox 唯一键建批次与全部目录项。领取用数据库租约，首投失败后依次走八档退避，第八次重试仍失败时 failure count 为 9、项目 DEAD、批次 FAILED；`NeedsManualDecision` 不是失败，不增加 attempts。记名 replay 复位原批次/原项目，不新建第二批。
5. 人工项按 `target_module` 固定映射 `SALES_MANAGER|PROCURE_MANAGER|FINANCE_MANAGER|PROJECT_MANAGER`，创建或复用 `HUMAN_TASK`，SLA 默认 5 天、范围 1 至 30，流程实例最长 365 天。流程任务只承载待办和超时提醒，不是决策事实源；平台不得解析 reason 或只存 task outcome。
6. 人工命令统一为 `{decision_code,decision_reason,decision_result_doc_id}`。四类人工规则的允许码与对象形状只取 impact 目录；码/理由/结果 id/锁后状态任一错误时保持 PENDING、三字段不落库且不耗重试。`CLM_TERM_PROJECT_TASK` 使用 `PROJECT_TASK_COMPLETED|PROJECT_TASK_CANCELLED`；采购与销项票的码和结果对象语义取阶段 7、10 的现行冻结值。

#### 三、来源闭合禁止跨 schema 直写

平台不得直接更新 `clm.contracts`。`ep-platform-impact` 定义中立 `ImpactSourceCompletionPort`，注册唯一键为 `(source_module,source_event_type)`；阶段 6 的 `ep-app-clm::ContractTerminationCompletionPort` 以 `(CLM,clm.contract.terminated.v1)` 真实注册。全部项 DONE、无 DEAD 时，ImpactAssessor 在调用方同一事务调用该端口，锁定并复核合同，推进 TERMINATED、写审计、恰一次发 `clm.contract.termination_completed.v1`，随后批次 DONE。缺失、重复或替身时 `impact-registry-consistent` 在启动与模块启用提交前失败关闭，模块启用回滚并返回 `PLATFORM.IMPACT_COMPLETION_PORT.NOT_REGISTERED`；不得 Noop、不得先闭批后补合同。

#### 四、可直接开发与发布门边界

阶段 3 的两表、crate、唯一消费者、查询/人工/replay API、流程模板、租约重试、失败关闭、目录校验和真实 PostgreSQL 正反例是开发工作，立即可做。Windows 实机、生产 Authenticode、客户合同与专业签字仍只在各自发布/认证点判定，不是设计二选一；它们不能阻止阶段 3 至 12 按本裁定编码。迁移版本由全库中央分配器统一给出合法、全局唯一 14 位号，本裁定只冻结两个 slug 与先后依赖，不授权阶段自行抢号。

#### 五、阶段退出判据只校验当时可构造集合

1. 任一阶段的启动自检、登记比对与退出门只校验“截至该阶段已经注册、已经交付且由该阶段可构造夹具”的集合；未来阶段才提供的 offsite、reporting 或其他 selfcheck 在此前阶段必须明确为 `NOT_APPLICABLE` 或不进入当期集合，不能以未来能力未存在判本阶段失败。
2. `configdoc --check-doc-type-codes` 在阶段 1 只校验文档结构、重复与阶段 1 可构造夹具；阶段 3a 常量表交付后才启用逐值比对。阶段 1 的 SBOM 负例使用当期工作区可构造的同名测试夹具证明检测器有效；`ep-bench`、`ep-release-gate` 等未来包在其加入工作区的阶段再纳入真实包断言。
3. 附录丙 22 条已全部由 F-01、F-03、F-04、F-05、F-52 与本 F-54 关闭：G-01 归 F-03/G-01，G-02 至 G-06 归 F-01，H-01 归 F-04/F-52，H-02 至 H-09 归 F-05，I-01 至 I-07 归本节。现行未决为 0，历史严重度不得再解释为开发阻断。

### F-58　对 F-50…F-57 的审阅修复：权威层收口、门禁状态登记、口径对齐

使用方推入 F-50…F-57（172 文件、74250 行）后指示「全面修复」。审阅出 51 条，
本卷修可当轮修的，**并首次把门禁转红这件事登记下来**。

#### 结论一　门禁状态登记（**此前 172 个文件里一字未提**）

成因是**文档动了门禁没动**——本次更新中 `xtask/` 与全部 `.rs` 改动 **0 行**。
按 `00b:128` 逐字「必须先改本表并走基线修订，不得只改代码」，
**文档先行是本仓既定次序，门禁红本身不是缺陷；缺的是登记与转绿判据**。
照本仓最佳样板（`docs/metrics-catalog.md:136`）补齐承接方、转绿判据、禁止误读三样。

| 门禁 | 修前 | 修后 | 剩余不符 | 转绿判据 |
|---|---|---|---|---|
| `archcheck` | 0 | **0** | — | — |
| `sqlcheck` | 0 | **0** | — | — |
| `codecheck` | 1（3 处） | 1（**2 处**） | `ai-inferer` 无 crate、无 systemd 单元 | 阶段 13c 落 `apps/ai-inferer/` 与 `deploy/podman/ai-inferer.container` |
| `errorcodes` | 1（13 处） | 1（13 处） | 12 条文档已登记代码未实现，1 条取值冲突见结论三 | 各码所属阶段实现时 |
| `configdoc` | 1（312 处） | 1 | 配置键与指标两侧未对齐 | 首批实施按 `config-reference.md` 与 `metrics-catalog.md` 对齐 |
| `eventcatalog` | 1（117 处） | 1 | 事件已登记、无产生点 | 各事件所属阶段实现时 |

全量测试：**通过 1064、失败 5、忽略 5**（修前 1063／6／5，`codecheck` 常量修复净救回一个）。

**禁止误读，三句写死：**
1. 上述红色状态**不阻塞按现行文档开发**——这是文档先行的正常中间态。
2. **但不得声称任何一道登记漂移门禁已经验证通过**，也不得据此宣称文档与代码一致。
3. 失败用例中的 `the_repository_itself_passes` 一族，**其语义就是「本仓能过自己的闸」；
   它失败恰是闸在正常工作、如实报告仓库当前过不了**，
   **不得以「测试坏了」为由停用或改写该用例**。

#### 结论二　权威层四处自相矛盾已消除

| # | 原状 | 处置 |
|---|---|---|
| 1 | 规格 `:3` 逐字「旧「可直接开发」结论均**不得继续执行**」，而同文件 `:11` 原样写着「**可直接进入开发**」 | `:11` 改 `READY_NOT_AUTHORIZED`，与 `README:8` 同口径 |
| 2 | 规格 `:5`／`:7`／`:9` 标 F-50／F-55／F-56 为「（当前）」，PRD 同三处标「历史裁定」 | 规格三处对齐 PRD |
| 3 | `00c:1` 逐字「F-52–F-54 只保留历史裁决证据」，而 `:5`／`:11`／`:19` 写「现行专项段」「唯一实现口径」 | 三处对齐首行横幅 |
| 4 | `00b:9` 与 `00c:11` 是**仅有的两处逐层排序条款，却都只枚举到 F-56** | 补「**F-57 最高**」并写明原句照字面会得出 F-56 压过 F-57 |

**第 1、4 两条后果不是文字**：按权威顺序规格最高，实现方读 `:11` 即可主张已获开工授权。

#### 结论三　口径对齐三处，其中一处**改了又回退，如实记**

- **F-50 验收项数**：第 11 节实数 **45** 条，`00c` 两处写 44，已改。
  **差的第 45 项恰是「每个错误码至少一个端到端负例」**——照旧数施工少做的正是新错误码的负例保护。
- **门户发票上传**（`PRD:1852`）仍是头表带税率的单行模型，与同一份 PRD 第 6.6 节
  逐字「头表不设单一税率」及数据字典逐字「删除单一 `tax_rate`」相反，已改头＋行。
  **这是第三类缺陷**：按原文做的表单会带一个库里已不存在的字段，且不会当场报错。
- **`PLATFORM.AUTHN.RATE_LIMITED` 本卷改了代码 503→429，随即回退**。
  理由：`docs/error-codes.md:21` 逐字「INFRASTRUCTURE …… | **503，限流 429**」，看似代码错；
  但改后 `crates/foundation/src/error/codes.rs` 的不变量用例 `category_and_http_agree` 当场失败——
  该文件 `:203` 逐字 `Category::Infrastructure => &[503]`，**代码的分类模型把 Infrastructure 硬绑 503，
  不认「限流 429」这个例外**。
  **这不是改一个数能解的，是分类模型要不要开例外的设计判断**，本卷不代裁，登记为未决。
  **记此过程**：本卷先改后退，是那条不变量用例把错拦下的——**它按设计工作了**。

#### 结论四　两处登记缺失已补

- `threat-model.md` 自称以 F-57 为最高边界，其「规范依据」清单里 **F-57 命中 0**。已补四份 F-57 文件。
- `SUPERSEDED_DO_NOT_EXECUTE` 与 `HISTORICAL_DO_NOT_EXECUTE` 挂在全仓 **25 个文件**横幅上，
  而持有唯一状态词表的 F-57 权威登记里**两码命中 0**。已补入定义并写明冲突时以词表为准。

#### 结论五　顺带修掉一处 F-55 的回写欠账

F-55 把基线第 2 节改为九进程（新增 `ai-inferer`），
**但阶段 1 计划 `01:81` 仍逐字「八个二进制」——而 `codecheck` 正是引这句当判据出处**，
其常量也仍写死 8。三处已同批改九，报错文案改用常量以免再漂。

#### 本轮未做到的

- **审阅只覆盖权威层与门禁层**：五份 F-57 实施计划正文（约 3 万行）、`threat-model.md` 正文、
  7 份 OpenAPI（约 1 万行）、12 份数据字典分卷**均未逐条核**；`configdoc` 312 处只看了 8 条。
- `configdoc`／`eventcatalog` 的不符项**未逐条消化**，只做状态登记。
- `RATE_LIMITED` 的分类模型例外**未裁**（结论三）。
- **F-51／F-55／F-56／F-57 各自改规格的逐轮授权留证未找到**，证据不足未立条，记此供下轮核。

## 附录丙　阶段 1 实测同类缺陷历史追溯（22/22 已关闭）

本附录登记裁定 F-01 与 F-03 落地过程中，由三次同类缺陷普查查出的 22 条。
三条已裁定的（F-01 的 PgTx 声明位、F-01 的 adapter 互依、F-03 的必要性判据）不重复登记；全卷不存在编号 F-02，此处原写的 F-02 即 F-01 的第二半。

**本附录是历史追溯索引，不是现行待决表。** 22 条均已由上列裁定关闭，现行未决为 0；文件、旧行号、旧原文和历史严重度只解释缺陷来源，不得恢复旧实现或形成开发阻断。

| 编号 | 类别 | 历史严重度 / 现行状态 | 落点 | 缺陷 |
|---|---|---|---|---|
| G-01 | 孤儿规则类 | 历史 blocking；已关闭（F-03/G-01） | `10-ar-ap-invoice.md:519` | B-08 子账余额提供者：trait 在 ep-contract-finance、类型在 ep-app-inventory/ep-app-procure、impl 被要求落在阶段 10 的 crate — 与 F1 同类且无任何合法落点。**已裁定**，归属本文件「G 类 落位裁定」的 G-01：撤销 `ep_contract_finance::SubledgerBalanceProvider`，两个端口分别落 ep-contract-inventory 与 ep-contract-procure |
| G-02 | 孤儿规则类 | 历史 blocking；已关闭（F-01） | `01-engineering-baseline.md:56` | F1 原句在阶段 1 计划复述：PgUnitOfWork/PgTx 声明位在 ep-adapter-db、实现体落在 ep-adapter-db-pg |
| G-03 | 孤儿规则类 | 历史 blocking；已关闭（F-01） | `02-data-foundation.md:407` | F1 原句在阶段 2 计划正文复述：同一声明位/实现位分离 |
| G-04 | 孤儿规则类 | 历史 blocking；已关闭（F-01） | `02-data-foundation.md:51` | F1 原句在阶段 2 crate 职责表复述：ep-adapter-db 承载「实现声明位」 |
| G-05 | 孤儿规则类 | 历史 blocking；已关闭（F-01） | `00c-gap-ruling.md:122` | F1 原句在裁定册 A-01 的「提供方要做什么」复述，且与同一裁定的结论句自相矛盾 |
| G-06 | 孤儿规则类 | 历史 blocking；已关闭（F-01） | `00-overview.md:238` | C-03 的另一种措辞同样规定三 crate 分离：ep-foundation 定义、ep-adapter-db 提供实现骨架、ep-adapter-db-pg 提供实现 |
| H-01 | 依赖方向类 | 历史 blocking；已关闭（F-04/F-52） | `13-clients-lowcode.md:60` | ep-platform-meta 与 ep-platform-release 互为依赖，构成 Cargo 硬性循环，且触发 archcheck 的 platform-acyclic。**已裁定并闭合**：F-04 将 release 依赖冻结为三项、13b 编排归位 apps；F-52 又将最终九套 suite 的执行落点与数据库行派发载体冻结，已无另行补裁项 |
| H-02 | 依赖方向类 | 历史 blocking；已关闭（F-05） | `13-clients-lowcode.md:61` | ep-adapter-wasm 依赖 ep-adapter-ipc，与禁止项第五条互斥（与 F2 同构）。**已裁定**，归属 F-05 第 4 节 H-02：两个实现按进程边切开，`PluginHostWasmCompute` 迁入 ep-adapter-ipc，穷举白名单撤销 |
| H-03 | 依赖方向类 | 历史 blocking；已关闭（F-04） | `03-platform-kernel.md:1177` | ep-platform-release 与 ep-platform-audit 直接取用 ep-adapter-kms，platform 反向依赖 adapter，与裁定 B-03 已作废的形态相同。**已裁定**，即 F-04：端口下沉 `ep_foundation::port::kms`，新增机检规则 `platform-no-adapter` |
| H-04 | 依赖方向类 | 历史 major；已关闭（F-05） | `01-engineering-baseline.md:577` | 原登记措辞「HTTP 中间件栈留 `ep_adapter_db::port::IdempotencyStore` 注入点」**已过期**：该依赖边已由 F-01 的端口下沉修掉，01:577 现文逐字只写 `IdempotencyStore`，完整路径为 `ep_foundation::port::db::IdempotencyStore`。**已裁定**，归属 F-05 第 4 节 H-04：残留的只是 00b:58 与 01:583 的 HTTP 口径不一，只改措辞、不新增 HTTP 系 adapter |
| H-05 | 依赖方向类 | 历史 major；已关闭（F-05） | `11-cost-metrics-reporting.md:623` | COSTING_INVENTORY_COGS_VS_STOCK_VALUE 由 ep-app-costing 实现却跨读 inventory schema，直接违反禁止项第七条。**已裁定**，归属 F-05 第 4 节 H-05；成因经复核更正为连接角色一维（11:369 证明现文取的已是 `inventory.v_stock_value_entries`，11:630 证明它跑在 job-worker 自身连接池），处置仍为改经 `ep_contract_inventory::StockValueOutboundPort` |
| H-06 | 依赖方向类 | 历史 major；已关闭（F-05） | `11-cost-metrics-reporting.md:22` | D-11-01 单方面把禁止项第七条重新界定为「只约束基表」，属阶段计划改写基线取值。**已裁定**，归属 F-05 第 4 节 H-06 与通则乙：判定面回写基线，D-11-01 由偏离降为已回写决定、编号不重排 |
| H-07 | 依赖方向类 | 历史 major；已关闭（F-05） | `14-ops-backup-release.md:73` | 归档与备份的 IPC 报文类型放进 ep-foundation，按禁止项第六条的必要性判据恒不可准入。**已裁定**，归属 F-05 第 4 节 H-07：七种报文类型落 ep-adapter-ipc，且不得被任何 `ep-platform-*` 命名 |
| H-08 | 依赖方向类 | 历史 minor；已关闭（F-05） | `08-inventory-costing.md:51` | ep-app-inventory 的依赖清单遗漏 ep-contract-ledger，而同阶段的过账端口与子账总账勾稽都要用它。**已裁定**，归属 F-05 第 4 节 H-08：08:51 补入该依赖并注明为本阶段结束时的快照（09:16 逐字确认 `TotalAccountBalanceProvider` 属 9a 段交付，早于阶段 8） |
| H-09 | 依赖方向类 | 历史 minor；已关闭（F-05） | `03-platform-kernel.md:117` | ep-adapter-search 被阶段 3 声明「只依赖 ep-foundation」，却被阶段 5 与阶段 12 要求承载各模块档案的投影函数。**已裁定**，归属 F-05 第 4 节 H-09：投影函数落各模块 `ep-app-*`，03:116 与 03:122 的依赖集不动 |
| I-01 | 判据不可判定类 | 历史 blocking；已关闭（F-54） | `04-identity-authz.md:724` | 阶段 4 退出条件 2：--check 要求「十三个命名项全部通过」，其中三项分别由阶段 3b 与阶段 14 交付 |
| I-02 | 判据不可判定类 | 历史 blocking；已关闭（F-54） | `03-platform-kernel.md:1525` | 阶段 3 退出条件 4：--check 十四项「全部通过」且对 DEGRADED 非零退出，但其中 offsite-sink-requirements 由阶段 14 交付 |
| I-03 | 判据不可判定类 | 历史 blocking；已关闭（F-54） | `09-ledger-period.md:798` | 阶段 9a 退出条件 E-17：判据是「可由阶段 11 的 reporting-dataset-signature-matched 自检项校验通过」，该自检项由阶段 11 交付 |
| I-04 | 判据不可判定类 | 历史 blocking；已关闭（F-54） | `10-ar-ap-invoice.md:1219` | 阶段 10 退出条件 20：判据是「阶段 11 的 reporting-dataset-signature-matched 在三者上按降级口径校验通过」，阶段 10 早于阶段 11 |
| I-05 | 判据不可判定类 | 历史 major；已关闭（F-54） | `06-contract-sales.md:777` | 阶段 6 退出条件 3：要求基线第 7.3 节十项在 --check 上全部通过，同样含阶段 14 才交付的 offsite-sink-requirements |
| I-06 | 判据不可判定类 | 历史 major；已关闭（F-54） | `01-engineering-baseline.md:515` | 阶段 1 退出条件 23：xtask configdoc --check-doc-type-codes 要与 ep-platform-sequence 的常量表逐项比对，而该常量表由阶段 3a 交付 |
| I-07 | 判据不可判定类 | 历史 minor；已关闭（F-54） | `01-engineering-baseline.md:506` | 阶段 1 退出条件 14：要求为「SBOM 中不出现 ep-bench 与 ep-release-gate」配负样例，而这两个包由阶段 14 才创建 |


### G 类　孤儿规则类（6 条）

判据：声明位与实现位分离，或 trait 与类型分属两个 crate 而 impl 落在第三个，在 Rust 中不可实现。

**G-01**（blocking，置信度 high）　`10-ar-ap-invoice.md:519`

> 其中八项的子账侧取自本阶段自有表；存货与已收货未收票两项按裁定 B-08 由本阶段把阶段 8 与阶段 7 各自提供的查询函数包装为 `ep_contract_finance::SubledgerBalanceProvider` 的实现后接入，实现类型名固定为 `InventorySubledgerBalanceQuery` 与 `GrniSubledgerBalanceQuery`。

三方归属逐项核对：trait `SubledgerBalanceProvider` 属 ep-contract-finance（00c:1128「该 trait 由阶段 10 定义」，10:135 该 crate 为阶段 10 新增）；类型 `InventorySubledgerBalanceQuery` 属 ep-app-inventory（08:449「位于 `crates/application/inventory/src/projection/subledger_balance.rs`」，根 Cargo.toml 第 86 行 ep-app-inventory = crates/application/inventory）；类型 `GrniSubledgerBalanceQuery` 属 ep-app-procure（07:35 列在 ep-app-procure 职责内，07:1027「由阶段 10 在其 `SubledgerBalanceProvider` 上包装」）。impl 被要求落在阶段 10 的 crate（10:519「由本阶段把……包装为……的实现」；00c:1128「按反向依赖由阶段 10 在交付时补齐两个实现的接线……并在阶段 10 包装」），即 ep-app-finance/ep-app-invoice——既不拥有 trait 也不拥有类型，写出 `impl SubledgerBalanceProvider for InventorySubledgerBalanceQuery` 即 E0117，与 F1 同一机制。且本条比 F1 更硬：全工作区没有一个 crate 能合法承载这两个 impl。(a) 放 ep-app-finance/ep-app-invoice：孤儿规则不过，且基线 00b:114「禁止 ep-app-A 依赖 ep-app-B」使这两个类型根本不可命名；(b) 放 apps/core-server 装配处：apps 可依赖全部，但 apps crate 同样两头皆外，孤儿规则照样不过；(c) 放 ep-contract-finance（trait 本地，合法）：需 ep-contract-finance 依赖 ep-app-inventory 与 ep-app-procure，违反 00b:105「ep-contract-<m> 只可依赖 ep-foundation」；(d) 放 ep-app-inventory/ep-app-procure（类型本地，合法，且 00b:107 允许 ep-app-* 依赖任意 ep-contract-*）——这是唯一可编译的落点，但被计划自己排除：08:449「本阶段不依赖 ep-contract-finance」，且阶段 8、7 排在阶段 10 之前、届时 trait 尚不存在。对照 A-15 的做法（00c:634 表，十一个实现类型逐个落在自己模块的 ep-app-*，impl 与类型同 crate）可见本条是该模式的破例。连带落点：00c-gap-ruling.md:1128（裁定原文）、00-overview.md:131、132、225、10-ar-ap-invoice.md:108、1308、08-inventory-costing.md:449、770、07-procurement-portal.md:1027、09-ledger-period.md:9、458、972 共十二处均按同一措辞写死，须同批改。

**G-02**（blocking，置信度 high）　`01-engineering-baseline.md:56`

> 第二处，`ep-adapter-db` 提供 `PgUnitOfWork` 与 `PgTx` 两个实现类型的声明位，实现体落在 `ep-adapter-db-pg`。

与 F1 同一条安排的另一处落点，需与基线 00b:166 同批改。trait `Tx`/`SnapshotCtx`/`UnitOfWork` 属 ep-foundation（00b:127「事务与快照抽象位于 `crates/foundation/src/port/tx.rs`」）；类型 `PgUnitOfWork`/`PgTx` 按本句属 ep-adapter-db；impl 被要求落在 ep-adapter-db-pg。ep-adapter-db-pg 对 trait 与类型双双是外部 crate，`impl Tx for PgTx` 触发 E0117。本句是阶段 1 的「三处落点在本阶段就写死，后续阶段只补内容不改位置」之一，即最先被按字面执行的一处。

**G-03**（blocking，置信度 high）　`02-data-foundation.md:407`

> 本阶段在 `ep-adapter-db` 提供 `PgUnitOfWork` 与 `PgTx` 两个实现类型的声明位，实现落在 `ep-adapter-db-pg`。

trait 属 ep-foundation，类型按本句属 ep-adapter-db，impl 被要求落在 ep-adapter-db-pg，三者互不相同，E0117。阶段 2 是真正写这两个类型的阶段（02:30 D-06「`ep-adapter-db-pg` ……工作单元、重试、编解码全部实现」），因此本句是该安排的执行点，而不只是转述；同句还写死了 `downcast_mut::<PgTx>` 只允许出现在 `crates/adapter/db-pg/`，与类型声明位在 ep-adapter-db 相互绑死，两条必须一并改。

**G-04**（blocking，置信度 high）　`02-data-foundation.md:51`

> `PgUnitOfWork` 与 `PgTx` 对 `ep_foundation::port::tx` 三个 trait 的实现声明位、`port::IdempotencyStore` 端口定义、四个连接模型类型的取值、公共能力基线类型映射、重试判定

crate 职责表这一格把「对 ep_foundation 三个 trait 的实现」的声明位写进 ep-adapter-db，配套第 52 行把「编解码」等实现写进 ep-adapter-db-pg。trait 属 ep-foundation、类型属 ep-adapter-db、impl 属 ep-adapter-db-pg，仍是三 crate 分离，E0117。此处与 02:407 是同一安排的两处登记，只改正文不改本表会留下第二套取值。

**G-05**（blocking，置信度 high）　`00c-gap-ruling.md:122`

> 提供方要做什么：阶段 1 在 ep-foundation 增加 `port::tx` 与 `id::marker` 两个模块，在 ep-adapter-db 提供 `PgUnitOfWork` 与 `PgTx` 两个实现类型的声明位，实现落在 ep-adapter-db-pg。

裁定册是基线与各阶段计划的权威出处，本句是 F1 的源头登记：trait 属 ep-foundation、类型属 ep-adapter-db、impl 落 ep-adapter-db-pg，E0117。另需注意同一裁定 A-01 的结论句（00c:73）写的是「在 ep-foundation 中冻结 Tx、SnapshotCtx、UnitOfWork 三者，ep-adapter-db 只提供实现」——按结论句 impl 与类型同在 ep-adapter-db，孤儿规则可过；按第 122 行则不可过。同一条裁定内部两句互斥，任何按第 122 行执行的实现都编译不过，须一并裁定取哪一句。

**G-06**（blocking，置信度 high）　`00-overview.md:238`

> ep-foundation 定义，ep-adapter-db 提供实现骨架，ep-adapter-db-pg 提供实现

这是 F1 的第二种措辞，「实现骨架」在 Rust 中只能落成两种东西：impl 块本身，或被 impl 的具体类型。若落成 impl 块，则 impl 在 ep-adapter-db 而其中的具体类型要在 ep-adapter-db-pg，方法体无处安放；若落成具体类型（即 PgUnitOfWork/PgTx，与 00b:166 一致），则 trait 属 ep-foundation、类型属 ep-adapter-db、impl 属 ep-adapter-db-pg，E0117。两种读法都不可实现，且该措辞未指明是哪一种，实施方无法照办。同一措辞另见 01-engineering-baseline.md:302 与 00c-gap-ruling.md:1194（后者是要求把该句回写进阶段 1 计划第 333 行的指令），三处须同批改。


### H 类　依赖方向类（9 条）

判据：按基线第 1.3 节七条禁止项或五条允许项判为违规的 crate 依赖。

**H-01**（blocking，置信度 high）　`13-clients-lowcode.md:60`

> | ep-platform-release | core-server、job-worker | 本 crate 由阶段 3b 按裁定 A-27 交付最小发布通道，`ConfigItemApplier` 端口与 `ConfigItemApplierRegistry` 由阶段 3a 按裁定 A-19 交付；本阶段在其上扩展内容项差异算法、自动测试编排、把发布状态机由阶段 3b 的六态补齐为十一态、DDL 段编排与回退编排 | 依赖 ep-foundation、ep-platform-meta、ep-platform-audit、ep-platform-outbox |

同一张表的上一行（13-clients-lowcode.md:59）把「六个 CUSTOM_ 与 UI_LAYOUT 类 `ConfigItemApplier` 实现」放进 ep-platform-meta；而 `ConfigItemApplier` trait 与 `ConfigItemApplierRegistry` 定义在 ep-platform-release（03-platform-kernel.md:115 原文：「`ep-platform-release`：3a 段只含 `port::config_item` 一个模块，即 `ConfigItemApplier` trait、`ItemKind`、`ConfigPackageItem` 与 `ConfigItemApplierRegistry`」）。在 Rust 中实现一个外部 trait 必须把定义它的 crate 列为依赖，因此 ep-platform-meta → ep-platform-release 是强制边。本行又把 ep-platform-release → ep-platform-meta 写死。两条边合起来是 Cargo 包级循环，cargo 直接报 cyclic package dependency，工作区无法解析，连编译都进不去。本仓库 xtask/src/archcheck/deps.rs:175 的 `rule_platform_acyclic` 也会报「platform 内部成环」。这与阶段 3 自己确立的反例纪律正相反：03-platform-kernel.md:123 原文「无环，因为 `ep-platform-release` 不反向依赖 `ep-platform-flow` 与 `ep-platform-notify`，两个 applier 落在实现方 crate 内」——阶段 3 把 applier 放实现方 crate 并禁止 release 反向依赖，阶段 13 恰恰把这条反向边加了回去。

**H-02**（blocking，置信度 high）　`13-clients-lowcode.md:61`

> | ep-adapter-wasm | plugin-host、core-server、job-worker | wasmtime Component 宿主、能力清单裁剪、燃料与内存与时限限额、编译缓存、宿主导入函数四件套，实现类型 `PluginHostWasmCompute` 对应阶段 3b 定义的 `ep_platform_flow::port::WasmComputePort`，见裁定 B-05；core-server 与 job-worker 侧只编入其 IPC 客户端 | adapter 层，可依赖 foundation 与 platform/domain 的端口 trait，不依赖 application |

「core-server 与 job-worker 侧只编入其 IPC 客户端」把 plugin 通道的 IPC 客户端放在 ep-adapter-wasm 内——这也是这两个进程需要链接 ep-adapter-wasm 的唯一理由。而下一行（13-clients-lowcode.md:62）把该通道的帧格式与报文类型放在另一个 adapter：「| ep-adapter-ipc | plugin-host、core-server、job-worker | 复用基线第 2 节已定的帧格式，新增 plugin 通道的请求与响应类型 |」。客户端要构造并编解码这些请求响应类型，就必须依赖 ep-adapter-ipc，于是 ep-adapter-wasm → ep-adapter-ipc。这条边正是基线第 1.3 节禁止项第五条「禁止 adapter 之间互相依赖，共用逻辑下沉到 ep-foundation」所禁，本仓库 xtask/src/archcheck/deps.rs:140 的 `rule_adapter_no_peer_adapter` 会按与 F2 完全相同的形态报 [adapter-no-peer-adapter] ep-adapter-wasm — 依赖了同层 ep-adapter-ipc。本行右侧的「依赖方向核对」列只复述了允许项里的 foundation/platform/domain/application 四项，压根没提同层 adapter，所以这条自检按其自身写法也发现不了。

**H-03**（blocking，置信度 high）　`03-platform-kernel.md:1177`

> 签名与验签：签名算法固定为 ECDSA P-256，密钥经 `ep-adapter-kms` 取用；`item_hash` 为该项 `after_spec` 的 JSON 规范化序列化（键按字典序、无空白、UTF-8）后的 SHA-256 十六进制小写，与阶段 13 计划第 4.7 节一致；导入时逐项重算 `item_hash` 并比对，任一不符整包置拒绝。

该段落属 3b 段配置发布通道，即 ep-platform-release 的状态机「Approved 到 Released（签名并执行发布单）」。要「密钥经 ep-adapter-kms 取用」，ep-platform-release 必须命名 `KmsBackend` 这个 trait，而它定义在 ep-adapter-kms（02-data-foundation.md:411 原文列出「`KmsBackend` trait（方法 `wrap`、`unwrap`、`derive_blind_key`、`health`）」），因此产生 ep-platform-release → ep-adapter-kms 的 crate 边。同一文件第 88 行把这条关系写得更普遍：「`ep-adapter-kms` 由阶段 2 交付，本阶段只消费其接口，不改动其公开签名」——而阶段 3 新增的消费方全是 platform crate（release 的包签名、audit 的「每 5 分钟或每 1000 条的 ECDSA P-256 段根签名」见第 35 行、file 的「按法人密钥域与密级子域的信封加密落盘」见第 36 行）。基线第 1.3 节允许项「ep-platform-* 只可依赖 ep-foundation 与其他 ep-platform-*」加上「其余一律禁止」，这条边被禁。裁定 00c-gap-ruling.md:1078 已经用同一条理由作废过一次完全同构的路径：「原裁定写的 `ep_platform_release::MigrationWindowGuard` 违反基线第 1.3 节『ep-platform-* 只可依赖 ep-foundation 与其他 ep-platform-*』，基线高于本表，该路径作废」——但 kms 这条没有对应裁定，且阶段 3 自己第 121 行还反向断言「本阶段全部新增 crate 均为 `ep-platform-*` 与 `ep-adapter-*`，依赖只指向 `ep-foundation` 与其他 `ep-platform-*`」，与第 88、1177 行自相矛盾。注意 xtask/src/archcheck/deps.rs:123 的 `rule_platform_no_domain_or_app` 只拦 domain 与 application，拦不住 platform→adapter，因此这条不会被现有门禁挡下，只会在人工评审或 cargo 图上暴露。

**H-04**（major，置信度 high）　`01-engineering-baseline.md:577`

> HTTP 中间件栈只留 `IdempotencyStore` 一个注入点，按 C-07 其端口定义归阶段 2、存储与重放实现归阶段 3a，本阶段的 `IdempotencyKeyHeaderGuard` 只校验请求头，不需要任何桩。

该 HTTP 中间件栈的落点由同一文件第 583 行写死在 ep-platform-runtime：「该 crate 只承载进程生命周期状态机、分层配置加载、`SelfCheckRegistry`、健康与就绪端点、HTTP 服务器与中间件栈骨架，以及以 trait 表达的 IPC 服务端接口……因此本 crate 只依赖 foundation 与其他 platform」。而 `IdempotencyStore` 的完整路径是 `ep_adapter_db::port::IdempotencyStore`（同文件第 331 行原文：「第二段是端口定义，`ep_adapter_db::port::IdempotencyStore` 及其 `try_begin` 与 `finish` 两个方法」；02-data-foundation.md:409 复述同一落点）。在 Rust 中，要在中间件栈里留一个以该 trait 为类型的注入点（字段、泛型约束或 `Arc<dyn IdempotencyStore>`），ep-platform-runtime 必须把 ep-adapter-db 列为依赖，第 583 行「只依赖 foundation 与其他 platform」当场不成立，也违反基线第 1.3 节允许项第二条加「其余一律禁止」。这是裁定 B-03 判过的同一条口子，只是那次落在 ep-platform-release、这次落在 ep-platform-runtime，没有对应裁定收口。附带一处同源矛盾：基线第 1.2 节（00b-technical-baseline.md:58）说 ep-platform-runtime 只放「以 trait 表达的服务器骨架。具体 HTTP 与 IPC 传输实现分别留在对应的 ep-adapter-*」，但第 1.2 节适配层清单九个 crate（F-01 撤销 ep-adapter-db 后的实数）里没有任何 HTTP adapter，而第 583 行又把「HTTP 服务器与中间件栈骨架」直接放进 runtime，两处口径不一。

**H-05**（major，置信度 high）　`11-cost-metrics-reporting.md:623`

> 本阶段在 ep-app-costing 实现三个 ep_platform_recon::ReconCheck，经 ReconRegistry::register 在 apps/job-worker/src/wiring.rs 注册；对账框架本体、platform_core 的三张对账表与 ReconExecutor 由阶段 9a 交付，本阶段不建框架、不改其实现。三个实现的 code 返回下表校验项列的取值，category 三项一律取 SUBLEDGER_VS_LEDGER，blocks_period_close 三项均为 true。第三项虽跨 costing 与 inventory 两个 schema 取数，其判据是金额勾稽而不是引用完整性，归入 CROSS_MODULE_LINK 会使同一类别同时承载两种性质的判据。

「第三项虽跨 costing 与 inventory 两个 schema 取数」是对禁止项第七条「禁止跨模块直接读写业务表」的正面自认。第 629 行把总账侧取数写成「inventory_stock_value_entries 中出库方向的金额合计，按法人与会计期间」，其物理落点是阶段 8 的 `inventory.stock_value_entries`（08-inventory-costing.md:117「表 3，`inventory.stock_value_entries`，库存金额流水，仅追加」）。禁止项第七条的机检形态是「adapter-db-pg 中的仓储实现按 schema 分文件，一个仓储只访问自己模块的 schema」，因此承接这个 ReconCheck 取数的 costing 仓储文件里必然出现 `inventory.` 前缀的表名，按阶段 12 自己描述的同一条断言（12-service-project-asset.md:84「ep-adapter-db-pg 中 service 仓储只出现 service.* 表名，project 仓储只出现 project.* 表名，由 CI 的 SQL 静态检查断言」）当场判违规。它也落不进阶段 11 自己开的那个例外：第 631 行原文「三项由阶段 9a 的 ReconExecutor 调度，在 job-worker 自身连接池上执行，不使用只读分析池」，而 D-11-01 的豁免条件是「只读、只经 ep_analyst_ro、只经已登记数据集视图」，此项两条都不满足。

**H-06**（major，置信度 high）　`11-cost-metrics-reporting.md:22`

> | D-11-01 | reporting 的分析取数经 ep_analyst_ro 直接读取来源模块在其自身 schema 内发布的 v_ 受治理数据集视图，不逐条经 contract trait 往返。基线第 1.3 节的禁止跨模块直接读写业务表在本阶段被界定为只约束基表 | 规格第 5.5 章与第 16 章要求分析与经营报表在同一实例的独立只读角色上以聚合执行；在会计分录 150 万条的基准数据集上逐行往返无法满足附录 A.1 常用报表 P95 在 10 秒内 | 只读、只经 ep_analyst_ro、只经已登记数据集视图、SQL 中不得出现来源模块基表名、不得出现任何写语句，由 CI 的 SQL 静态检查与数据集注册表双重约束 |

基线第 1.3 节禁止项第七条的原文没有任何「基表」限定：「禁止跨模块直接读写业务表，adapter-db-pg 中的仓储实现按 schema 分文件，一个仓储只访问自己模块的 schema」。判据的落点是「schema」而不是「基表」——reporting 的取数 SQL 里出现的 `inventory.v_stock_value_entries`、`finance.v_*` 等视图名，其 schema 前缀属于来源模块，按原文机检形态照样判违规，把「不得出现来源模块基表名」换上去等于换掉了判据本身。而基线第 0 节写死「凡本文件已给出取值的事项，各阶段计划直接引用，不得重新决定、不得给出第二套取值」，本条恰是阶段计划对基线取值的重新决定；本条被声明为偏离项、但基线第 1.3 节至今没有对应回写，因此两份文件同时有效且互斥：按基线执行则 reporting 的 13 个跨模块数据集视图全部不可用（第 825 行 R-3 原文点明「reporting 的运行期正确性依赖 13 个外部数据集视图的列签名，其提供阶段为 5、6、8、9a、10、12 六个」），按本条执行则第七条在 reporting 上整体失效。

**H-07**（major，置信度 medium）　`14-ops-backup-release.md:73`

> archive-writer 与 backup-writer 两个 apps 不依赖任何 ep-app-*，其与 core-server 的全部交互只经 ep-adapter-ipc 的报文类型，报文类型定义在 ep-contract-portal 之外的独立位置，即放在 ep-foundation 的 ipc 模块下，理由是它跨越 platform 与 adapter 两侧且不属于任何业务模块

禁止项第六条把 foundation 的准入判据写成两条且都可机检，必要性一条是「被两个及以上 `ep-contract-*` 引用，或被 `ep-platform-*` 引用」。这批报文类型的引用方按本行自己的描述只有 ep-adapter-ipc 与 archive-writer/backup-writer/core-server 三个 apps——一个 `ep-contract-*` 都没有，也没有任何 `ep-platform-*`，必要性判据取值恒为假，`xtask archcheck` 的 foundation 必要性断言（xtask/src/archcheck/source.rs 的 `foundation_necessity`）会把该模块判为不必要，落地即构建失败。本行给出的理由「它跨越 platform 与 adapter 两侧」恰恰是判据判不出来的那一类，因为判据只认 contract 与 platform 的引用计数。另有一处同源冲突：阶段 13 把同一通道的类型放在了别处，13-clients-lowcode.md:62 原文「| ep-adapter-ipc | plugin-host、core-server、job-worker | 复用基线第 2 节已定的帧格式，新增 plugin 通道的请求与响应类型 |」，即同一个 ep-adapter-ipc 的报文类型被两个阶段分别安排进 ep-foundation 与 ep-adapter-ipc。

**H-08**（minor，置信度 medium）　`08-inventory-costing.md:51`

> 依赖方向按基线第 1.3 节，逐条自查如下。ep-domain-inventory 只依赖 ep-foundation 与 ep-contract-inventory。ep-app-inventory 依赖 ep-foundation、ep-platform-authz、ep-platform-audit、ep-platform-outbox、ep-platform-obs、ep-platform-recon、ep-domain-inventory、ep-contract-inventory、ep-contract-mdm。ep-app-inventory 不依赖任何其他模块的 application crate。

这是一份封闭枚举（「逐条自查」），而同一文件第 54 行原文说「ep-platform-recon 的对账框架、ep-contract-ledger 的过账端口与 ep-contract-mdm 的探针 trait 在本阶段开工时均已存在」，第 447 行又要求 ep-app-inventory 实现一个 category 取 `SUBLEDGER_VS_LEDGER` 的 ReconCheck（原文「存货项子账与总账勾稽，`category()` 取 `SUBLEDGER_VS_LEDGER`」）。这两项都必须命名 ep-contract-ledger 的类型（过账端口、以及 09-ledger-period.md:458 定义的总账侧 `TotalAccountBalanceProvider`），因此实际依赖集必然包含 ep-contract-ledger。按清单字面写 Cargo.toml 会缺依赖直接编译失败；而 09-ledger-period.md:90 明确 CI 是按「期望依赖清单」断言的（原文「届时 CI 的 cargo metadata 断言脚本中 ep-app-ledger 的期望依赖清单由阶段 11 同批更新」），补上依赖又会与本行的封闭枚举对不上。若为绕开而让该 ReconCheck 直接读 ledger schema 取总账侧余额，则落到禁止项第七条上，两条路都不通。

**H-09**（minor，置信度 medium）　`03-platform-kernel.md:117`

> `ep-adapter-search`（3b 段）：内置检索索引的按法人分区读写，实现 `ep_foundation::port::search` 的两个 trait。只依赖 `ep-foundation`，不依赖任何 `ep-platform-*`，索引根目录与分区路径见第 3.4.10 节。

阶段 5 把各模块的投影函数放进了这个 crate：05-master-data.md:79 原文「| ep-adapter-search | 按 ep_foundation::port::search::SearchDocument 结构定义四类档案与价目表的投影函数，写入方为 job-worker 的索引消费者……」。要把「四类档案与价目表」投影成 `SearchDocument`，函数签名必须接受 mdm 与 cpq 的档案 DTO，这些 DTO 按基线第 1.2 节只存在于 `ep-contract-mdm` 与 `ep-contract-cpq`，于是 ep-adapter-search 必须依赖这两个 contract crate，「只依赖 `ep-foundation`」这句当场不成立，按字面写 Cargo.toml 则投影函数无类型可写、编译不过。该边本身不违反七条禁止项（允许项明确「ep-adapter-* 可依赖 ep-foundation、ep-contract-*」），坏的是两个阶段对同一 crate 的依赖集给了互斥的两套取值，CI 的 cargo metadata 期望清单只能满足其一。


### I 类　判据不可判定类（7 条）

判据：判据依赖的被测输入由更晚的阶段交付，在判据所在阶段恒为空判或恒失败。

**I-01**（blocking，置信度 high）　`04-identity-authz.md:724`

> 2. core-server 与 job-worker 以 --check 模式退出码为 0，基线第 7.3 节的十三个命名项加本阶段的 authz-snapshot-loadable 一项全部通过，且 platform_ops.degradation_windows 中没有未关闭的 AUTHZ_SNAPSHOT_CHECKSUM_MISMATCH 窗口。

依赖的输入是 SelfCheckRegistry 中已注册且能返回 PASSED 的基线自检项。按通则第四条固定链 1 → 2 → 3a → 4 → 3b-1 → T0 → …，阶段 4 排在 3b 之前、阶段 14 之前。基线第 7.3 节现行十项中有三项在阶段 4 时点不存在：(a) offsite-sink-requirements —— 01-engineering-baseline.md:216 原文「`offsite-sink-requirements` 本阶段既不登记也不留 TODO 注释……该项整条推迟，由阶段 14 在交付落点判定的同一批里连同 `DegradationLedger::open` 的调用一次登记为 Degrading 项」，14-ops-backup-release.md:451 原文「基线第 7.3 节的 offsite-sink-requirements 项……由本阶段实现」；(b) audit-chain-verifiable 与 (c) file-store-writable —— 01-engineering-baseline.md:216 原文「`secrets-resolvable`、`audit-chain-verifiable`、`file-store-writable` 由阶段 3b 实现」，而 3b 整体排在阶段 4 之后（04-identity-authz.md:3 自述「这四项本体均由阶段 3b 交付，落在本阶段之后」）。未注册的项不可能「通过」，以 Pending 登记的项按 01-engineering-baseline.md:614「Pending……不计入 overall 的成败」也不构成「通过」。因此该条判据在阶段 4 恒不可满足。附带证据：条文写「十三个命名项」，而基线第 7.3 节经阶段 1 新增决定十三删去三项后现行只有十项，计数本身也停留在已作废的旧清单上，说明该条从未按现行注册表核对过。

**I-02**（blocking，置信度 high）　`03-platform-kernel.md:1525`

> 4. `--check` 的十四个命名项（基线第 7.3 节现行十项，加本阶段四项 `audit-evidence-store-writable`、`audit-signing-key-usable`、`attachment-store-ready`、`event-catalog-consistent`）在部署环境上全部通过并输出结构化报告，报告逐项给出 `Blocking` 或 `Degrading` 级别；`--check` 对 FAILED 与 DEGRADED 一律非零退出，`event-catalog-consistent` 在注入不一致时不阻止进程启动而是写出一条降级窗口。

「基线第 7.3 节现行十项」中的 offsite-sink-requirements 在阶段 3 时点尚未注册进 SelfCheckRegistry：01-engineering-baseline.md:216 明写阶段 1「既不登记也不留 TODO 注释……该项整条推迟」，14-ops-backup-release.md:451 明写该项「由本阶段实现，细化为八个子判定」。于是十四项这个计数在阶段 3 只可能凑出十三项，缺的那一项既不能判 PASSED 也不能判 FAILED。更强的一层：即便有人按阶段 2 的口径把它注册为返回 NOT_APPLICABLE（02-data-foundation.md:774 原文「`audit-chain-verifiable`、`file-store-writable` 与 `offsite-sink-requirements` 三项在其承担阶段交付前返回 `NOT_APPLICABLE`」），NOT_APPLICABLE 仍不是「通过」；而若按其真实语义判定，落点在阶段 14 之前根本不存在，该 Degrading 项必然 DEGRADED，本条自己又规定「`--check` 对 FAILED 与 DEGRADED 一律非零退出」，判据随即自相矛盾地恒失败。

**I-03**（blocking，置信度 high）　`09-ledger-period.md:798`

> E-17 受治理数据集视图 ledger.v_account_period_balances 已发布，dataset code 为 ledger_account_period_balances、grain 为 SNAPSHOT，输出含 legal_entity_id、security_level、data_scope_tags 三列，已 GRANT SELECT 给 ep_analyst_ro，列签名已同步给阶段 11 并可由其 reporting-dataset-signature-matched 自检项校验通过。

这条判据分两半，前半可判、后半不可判。前半「列签名与 reporting.dataset_fields 的登记比对」是可判的：11-cost-metrics-reporting.md:12 与 :328-339 说明 db/migrations/reporting/ 的第 1、2、11、12 号迁移（建 datasets、建 dataset_fields、两个 seed backfill）在 T0 期间提前执行，而 T0 是固定链第六环，早于 9a，所以登记表在 9a 时点已存在。后半不可判：判定手段 reporting-dataset-signature-matched 是一个启动自检项，其注册方是阶段 11（11-cost-metrics-reporting.md:106 原文「新增 severity 为 Degrading 的命名自检项 reporting-dataset-signature-matched」；00-overview.md:260 C-25 行原文「阶段 11 追加 reporting-dataset-signature-matched 取 Degrading」），且它明确不在阶段 11 的 T0 切片内 —— 11-cost-metrics-reporting.md:10 把 T0 贡献逐条限定为四项（revenue_entries 一表一视图、捕获调用点、只出收入卡的一个端点、一个桌面页面），并在 :8 声明「不贡献本节以外的任何内容」。9a 排在固定链 …T0 → 5 → 9a → 8 → 6 → 7 → 10 → 11… 的第八环，早于阶段 11 五个环节。于是 9a 结束时进程里根本没有这个自检项可跑，「校验通过」既无法为真也无法为假。同一缺陷在本文件 §9.8.3 集成测试第十五条（09-ledger-period.md:737）以被测口径重复了一次。

**I-04**（blocking，置信度 high）　`10-ar-ap-invoice.md:1219`

> 20. 本模块数据集视图 `invoice.v_purchase_invoices_dataset`、`finance.v_receivable_ledger_entries`、`finance.v_payable_ledger_entries` 已发布并授予 `ep_analyst_ro`，列签名已同步给阶段 11，阶段 11 的 `reporting-dataset-signature-matched` 在三者上按降级口径校验通过，即签名不符时关闭相关报表入口并开降级窗口而不阻断启动。

与 9a 的 E-17 同一根因，且更露骨：判据的执行主体被直呼为「阶段 11 的」自检项。阶段 10 在固定链上是第十二环、阶段 11 是第十三环，阶段 10 结束时该自检项尚未注册（注册方见 11-cost-metrics-reporting.md:106 与 00-overview.md:260 的 C-25 行；它也不在 11-cost-metrics-reporting.md:10 逐条列出的 T0 四项贡献之内）。判据后半句还要求观察到「签名不符时关闭相关报表入口并开降级窗口」这一运行期后果，而关闭入口的对象是阶段 11 的报表对象与运行端点，在阶段 10 同样不存在。因此本条在阶段 10 既跑不出通过、也跑不出不通过，属恒空判。

**I-05**（major，置信度 high）　`06-contract-sales.md:777`

> 3. `apps/core-server --check` 与 `apps/job-worker --check` 在基线第 7.3 节十项上全部通过并输出结构化报告，本阶段不追加任何启动自检项；本模块的 18 个事件与 `docs/event-catalog.md` 经 `xtask configdoc` 逐字比对通过。

与阶段 3、阶段 4 同一根因。阶段 6 在固定链上位于 …8 → 6 → 7 → 10 → 11 → 9b → 14，早于阶段 14。基线第 7.3 节十项之一的 offsite-sink-requirements 的被测对象是「服务器之外落点的三项最低要求判定」，落点判定与其 DegradationLedger 登记由阶段 14 交付（14-ops-backup-release.md:451），阶段 1 已声明不注册该项（01-engineering-baseline.md:216）。因此「十项全部通过」在阶段 6 恒不可达：要么第十项不在注册表里而无从判定，要么以 Degrading 身份判为不满足。注：同为基线十项的 file-store-writable 归属存疑（阶段 1 指给阶段 3b，但 03-platform-kernel.md:1424 的自检增量只追加了四个新名字、未点名接管该项），这一点我标注为不确定，本条只以 offsite-sink-requirements 立论。

**I-06**（major，置信度 high）　`01-engineering-baseline.md:515`

> 23. `docs/data-dictionary.md` 的单据类型码一节存在，`xtask configdoc --check-doc-type-codes` 通过，判据为该节与 `ep-platform-sequence` 的常量表逐项一致且无重复。

判据的比对对象之一是 ep-platform-sequence 的类型码常量表，它在阶段 1 不存在。01-engineering-baseline.md:46 把 sequence 连同其余十二个 platform crate 列为「骨架 / 不装配」，:39 进一步规定骨架 crate「`lib.rs` 仅含 `pub use` 与一条编译期断言注释，不留 `todo!()`」，即没有任何常量。该常量表的交付方是阶段 3a：03-platform-kernel.md:72 把 ep-platform-sequence 标为「新增」，:101 原文「`ep-platform-sequence`：`NumberAllocator` 端口、编号格式化与解析、类型码注册表、位数扩展算法」。比对的另一侧同样是空的 —— 00-overview.md:261 的 C-26 行把全量四十一个码逐个分给阶段 4、5、6、7、9、10、11、12 登记，阶段 1 一个码都不登记。于是这条 CI 项在阶段 1 要么因找不到常量表而无法运行，要么退化为两个空集合的比对而恒真，两种情形都不构成任何实质判定。它与 F3 是同一形态：判据的被测输入整体落在更晚的阶段。

**I-07**（minor，置信度 medium）　`01-engineering-baseline.md:506`

> 14. SBOM 生成成功，`cargo deny` 与依赖漏洞扫描零严重与高危，许可证清单通过；`xtask sbom` 另含一个断言 SBOM 中不出现 `ep-bench` 与 `ep-release-gate` 两个包名的负样例，与阶段 14 的发布门禁项 `RG-TOOLS-EXCLUDED` 同名同判据。

断言的被测形态是「工作区里存在这两个包且它们泄漏进了 SBOM」，而这两个包在阶段 1 根本不存在：14-ops-backup-release.md:41-42 把 ep-bench 与 ep-release-gate 列为阶段 14 的第 10、11 项交付物，:57-58 给出其落点 tools/bench/ 与 tools/release-gate/ 并注明「不进入发布制品与 SBOM」；阶段 1 的 crate 清单（01-engineering-baseline.md:41-55）不含这两项。正向断言在阶段 1 恒真而无信息量；退出条件另要求的负样例按定义必须构建失败，可它需要把这两个包真正拉进 SBOM 才能触发，阶段 1 无从构造。不确定之处：若允许用手写的 SBOM 夹具而非真实工作区成员来充当负样例，则该负样例可造，本条降为「正向断言恒真、无判定力」而非完全不可执行 —— 计划正文未说明允许哪一种，我按字面执行判定。

## 附录辛　阶段 3 落码过程中查出的口径不一致（登记，非裁定）

本附录登记阶段 3 各 crate 落码时查出的、卷内两处措辞对不上的地方。
**只登记不裁定**：每条都写明两处原文、实现当前取哪一种、以及改判时的改动面。
凡实现已按其中一种落码的，改判的代价一律控制在「一个函数加它的用例」之内——
这是本附录的收录门槛，够不上的应走正式裁定而不是登记。

---

#### 现状总表（本附录的权威状态索引，由裁定 F-28 建立）

**本表是附录辛全部条目现状的唯一权威来源。** 散落在各 F 裁定尾部的「新登记」小表只记
立案当时的状态，**不随后续裁定更新**；凡两者不一致，以本表为准。
建表原因见 F-28 结论三：三张裁定尾表丢了「类」列，致辛-34、37、38、39、40 在按类别的
盘点里隐形；另有 18 行登记行的类列停在立案时的「须走裁定」而实际状态已变（已同批改正）。

口径：00c 内互异辛编号 **40 个**；**辛-35、辛-36 从未存在**（00c、全仓、`git log -S`
三处零命中，F-27 从辛-34 跳至辛-37 属编号错误）。已处置并回写 17 个（辛-1 至辛-14a）、
已撤销 3 个（辛-21、29、30）、**在册 23 条**。
（本数由 F-37 更正：原写「21」与总表实际行数不符——总表数据行 28 行，减「辛-1 至辛-14a」合并行 1、减「辛-35、辛-36 从未存在」1、减已撤销 3，得 23。）

| 编号 | 现状 | 定谳裁定 |
|---|---|---|
| 辛-1 至 辛-14a（17 条） | 已处置并回写 | F-13 至 F-20 |
| 辛-15 | 当事人更换后保留：原当事人（audit_events）撤回，换为 sign-in 锁定窗口链——墙钟回拨即令锁定速率限制整体作废；形态由「无判据」更正为「判据在册但恒不生效」 | F-18 立案，F-37 定谳 |
| 辛-16 | **已撤销**：「无录入端点」当场为假（13:680 通用配置包录入在场），其余三项已由 F-21 全额承接并回写计划，四要件全勾 | F-21 判乙，F-37 撤号 |
| 辛-17 | **已撤销**：承重的否定式全称句被四处在场判据证伪 | F-28 结论二，F-37 撤号 |
| 辛-18 | 降为验收判据：今日 custom_fields 未建表、端点请求体未点名该列，无人能赋值 | F-28 立待核，F-37 定谳 |
| 辛-19 | **已撤销**：「零文法」证伪（template.rs:52 占位符文法、8 条用例，文件早于登记入仓）；残余当事人不同，改挂阶段 3b | F-28 结论二，F-37 撤号 |
| 辛-20 | **已撤销（撤号本轮执行）**：登记落点在场且已被 archcheck 机械读取 | F-24、F-28 判，F-37 执行 |
| 辛-21 | **已撤销** | F-27 结论零 |
| 辛-22 | 两部分分列：阶段 4 登记接口零交付＝成立；后果面＝条件句推断 | F-22、F-28、F-30 定谳 |
| 辛-23 | 降为验收判据（本轮执行两轮已判的应撤）：阶段号与退出条件均在场，今日零开窗方 | F-24、F-28 判，F-37 执行 |
| 辛-24 | **部分承接**：次序一致性由 SQL-031 静态判；活库那一半**已于 F-39 首次实跑并返 0 行**，但幻影承接方不变（xtask 仍跑不了它） | F-22 立案，SQL-031 落码，F-39 实跑 |
| 辛-25 | 降为验收判据（依据是今日后果指不出，非纪律四）：FieldProjector 零生产构造点，field-views 路由未注册 | F-22 立案，F-37 定谳 |
| 辛-26 | 已承接结案：声明面由 configdoc 第四段落码承接；运行期面改挂阶段 13 能力闸（x-client 读取方已在线，client_capability_values 仍零命中） | F-22 立案，F-37 结案 |
| 辛-27 | 收窄为一件（`ep-migrate check` 调用方）；另两件按纪律四撤为验收判据 | F-23，F-36 收窄 |
| 辛-28 | 成立但当事人收窄：注释那一半半错（F-34 结论零已更正）；真缺口是两进程无数据库装配，今日不可落码，判据降第三档 | F-23 立案，F-34 定谳 |
| 辛-29 | **已撤销** | F-24 |
| 辛-30 | **已撤销** | F-24 |
| 辛-31 | 降为验收判据（取证段待补）；反向断言那一半按 F-34 结论四并入辛-28，不重复计 | F-23 立案，F-37 降档 |
| 辛-32 | **已承接结案**：configdoc 第四段判据已于 ad70e10 落码并挂进 `run()`，今日 33 条非系统 `/api/v1/` 路由为被测输入、非恒真；登记所引「14:576」为假，应改引 `14:580` | F-24 立案，F-38 结案 |
| 辛-33 | 成立（当事人已换） | F-25，F-28 分档 |
| 辛-34 | 成立但需收窄（半二已撤） | F-26，F-28 分档 |
| 辛-35、辛-36 | **从未存在** | F-28 结论零 |
| 辛-37 | 降为验收判据：a/b/c/d 四条复验为真，但今日后果指不出（种子作差为空、无活库故唯一发射点不可达），且所保的 PRD 条款已由 `ck_permission_items_forbidden_codes` 与 `guard_permission_item_code` 双载体承载 | F-27 立案，F-38 降档 |
| 辛-38 | 降为验收判据：四处口径实为**一处**真分歧，DDL 正则是两侧公共超集而非第四套；发布执行路径未接通，126 个串今日只在 markdown | F-27 立案，F-38 降档 |
| 辛-39 | 成立但需收窄；当事人已换 | F-27 立案，F-29 换当事人 |
| 辛-40 | 结构已补齐（F-35）；恒过成因是两侧零行，属排期项；可执行性面按 F-36 结论六为验收判据 | F-27 立案，F-35 处置，F-36 更正 |
| 辛-41 | 降为验收判据：零外键属实，但「零跨表 CHECK」在 PostgreSQL 里**恒真、不构成证据**；跨表咬合已由 `decider.rs:80-83` 在应用层承载，而判定面今日零生产构造点 | F-29 结论八立案，F-38 降档 |
| 辛-42 | 降为验收判据（**攻击改判**，原「成立并保留」）：计数复算一致，但机制表述与承重后果均不成立——`strip_prefix` 判的是**语句首词**不是文件行首，真因是切句器把 DO 块并成一条；且 sqlcheck 对 `permission_items` 返 0 **正是正确答案**（豁免有逐字登记） | F-36 结论二立案，F-38 改判 |

**在册 21 条的六档小计**：成立 8（辛-24、27、28、32、33、37、38、40）；
成立但需收窄 7（辛-15、22、25、26、31、34、39）；夸大 1（辛-19）；证伪 2（辛-17、20）；
排期项应撤 2（辛-16、23）；待核 1（辛-18）。

**同批两条：辛-22 与辛-39**（同一对表、同一次换键、同一批种子行）。
**辛-25 与辛-26 由裁定 F-29 结论六移出该批**：辛-25 的解锁物是字段投影路由，
辛-26 的解锁物在阶段 13 的能力闸且排在判定**之前**，两者与「判定面接入」无先后依赖；
辛-26 的「今日零后果」一档同批改为「今日形态即阶段 4 退出条件 17 所要」。

**一条十轮无人认领**：辛-7a（法规基准日期标注）在 F-18 至 F-27 零提及，全卷仅 3 处命中。


### 辛-1　Outbox 的「共 8 次」是八次重试还是八次尝试

**两处原文。** 技术基线第 6.2 节逐字：「重试退避固定为 1 秒、5 秒、30 秒、2 分钟、
10 分钟、30 分钟、1 小时、2 小时，共 8 次，全部失败后置为 `DEAD` 并写入死信。」
阶段 6 计划逐字：「失败 `attempts + 1` 并按基线第 6.2 节的八档退避重排 `available_at`，
八次全部失败置 `DEAD` 并写入 `platform_msg.dead_letters`。」

前者的「共 8 次」修饰的是**重试**，后者按字面读像是**八次尝试**，两者差一次。

**实现当前取「八次重试」**，即首投加八次重试共九次投递，第九次失败才进死信。
理由不是从措辞上挑一个更像的，而是另一条更硬的约束：**退避表有 8 档，
取「八次尝试」会让最后一档（2 小时）永远排不上**——一个列在表里却永远用不到的取值
本身就是缺陷，本卷已在多处禁止这种形态（恒不命中的门禁、无被测对象的判据）。

**改判的改动面**：`crates/platform/outbox/src/delivery.rs` 的 `judge` 一个函数，
加它的三条用例（`every_backoff_tier_is_reachable`、`dead_letter_only_after_the_eighth_retry`、
`backoff_values_match_the_baseline_verbatim`）。其余代码不受影响。
若改判为「八次尝试」，须同批把基线第 6.2 节的退避表从八档减为七档，
否则缺陷仍在，只是换了个地方。

### 辛-2　JCS 数值精度：金额经规范化会被悄悄舍入

**这一条不是两处措辞不一致，是卷内没有任何一处写到。** 计划第 3.4.2 节逐字要求
「`bytea` 与 `uuid` 一律以字符串承载」，**没有点名数值**。

而 JCS（RFC 8785）的数值序列化走 ECMAScript 的 `Number::toString`，即 IEEE 754 双精度，
只能精确表示到 2^53（约 9.007×10^15）以内的整数；本卷的金额是 `numeric(18,2)`，
最大到 10^18 量级。**一个足够大的金额进 `before` 或 `after` 后，
其哈希算的是舍入后的值，而库里存的是原值。**

链验证当时不会报错——两侧用的是同一个舍入过程；但它从一开始就算错了，
且任何一方换实现即对不上。这是一条**不会报警的错**，比会报警的错难查得多。

**实现当前的处置**：`crates/platform/audit/src/jcs.rs` 在遇到无法精确往返的数值时
**返回错误而不是舍入**，小数一律拒绝。正确用法是把金额、数量、单价、比率四类
以字符串承载。

**建议**：把「数值一律以字符串承载」补进计划第 3.4.2 节的那句纪律里，
与 `bytea` 和 `uuid` 并列。本附录不代改计划，只登记。

### 辛-3　自定义对象码的长度：基线给 64，派生标识符只容得下 24

**两处原文。** 基线第 11.2 节逐字：文本列「默认上限：**编码 64**、名称 200……」，
`platform_meta.custom_objects.code` 是编码类列，按此得 64。
阶段 13 计划第 3.2 节逐字：对象码「**同时是 `ext` 下的物理表名**」，
`physical_table_name` 「固定为 `ext.` 加 code」；第 3.3 节的生成模板又从同一个 code
派生出 `pk_`、`ck_…_status`、`rls_…_le`、`ix_…_legal_entity_id_created_at`
与多对多的 `ux_<a>_<b>_links_pair`。

两处各自成立，合起来不成立：PostgreSQL 的 `NAMEDATALEN` 为 64、标识符可用 63 字节，
而 `ux_<a>_<b>_links_pair` 的长度是两个码之和加 15。**两个 64 长的码建一次多对多关系，
派生出的索引名是 143 字节。**

**这条错不会当场报错**，这是它值得登记的理由。PostgreSQL 对超长标识符的处置是
**截断到 63 字节并只发一条 NOTICE**，不是报错。于是 `platform_meta` 里记下的名字
与库里真实存在的名字不是同一个；日后按记录的名字去 drop 或重建，命中的是空集，
而调用方拿到的是「成功」。

**实现当前取 24**，由多对多那一路反推：`2n + 15 ≤ 63`。
单对象一路最紧的是 `ix_…_created_at`（`n + 30 ≤ 63`），给出 33——**取小者**。
漏掉多对多那一路会得出 33，而 33 在两个自定义对象建关系时派生出 81 字节，仍被截断。

**改判的改动面**：`crates/platform/meta/src/identifier.rs` 的 `MAX_OBJECT_CODE_LEN`
一个常量加它的四条用例。若改判为放宽上限，须同批改阶段 13 计划第 3.3 节的命名模板
（缩短派生后缀，或改为哈希后缀），否则截断仍在，只是换了个地方；
只改基线第 11.2 节的 64 不解决问题——64 正是溢出的那一侧。

### 辛-4　`ext.<code>` 不加引号，而本卷没有一处判保留字

**这一条不是两处措辞不一致，是卷内没有任何一处写到**，形同辛-2。

阶段 13 计划第 3.3 节的生成模板逐字 `create table ext.<code> (`，
标识符**不加引号**。而对象码由管理员在低代码界面上取。
一个等于 PostgreSQL 保留字的取值——`order`、`user`、`table`、`group`、`case`——
会让建表语句语法出错。这一条与辛-3 不同：**它当场报错**，因此危害小得多，
登记它是因为报错的时机太靠后（发布期执行 DDL 计划时），而管理员在建模界面上
拿不到任何提示，且此时该对象的字段、关系、布局可能都已配好。

**实现当前不判保留字。** 判它需要一份**完整**的保留字表；
一份不完整的黑名单正是本卷反复禁止的形态——它会让「查过了」这件事变成假的
（同类形态见通则第六条与本文件多处对恒真判据的处置）。
`crates/platform/meta/src/identifier.rs` 的模块文档已把这一未覆盖点明写出来，
不以「校验过了」的外观掩盖。

**处置有两条路，本附录不选**：一是把生成模板的 `ext.<code>` 及其全部派生名改为
加双引号，此后保留字不再是问题，代价是全库标识符大小写敏感性口径要一并交代；
二是引入完整保留字表（PostgreSQL 16 的 `pg_get_keywords()` 可导出，
但那是运行期取数，冻结进代码即需版本化）。前者改的是阶段 13 计划第 3.3 节一处模板，
后者新增一个常量表加校验分支。**两条路的代价都超出本附录「一个函数加它的用例」的收录门槛**，
故本条只登记现状与未覆盖面，正式处置须走裁定。

### 辛-5　许可状态：规格的四态与计划冻结的四个变体不是同一组

**两处原文。** 规格第 3.4 章逐字：「许可状态分为**生效、临期告警、宽限期、受限运行**四种」，
且「到期日后进入 30 天宽限期：**全部功能可用**，告警范围扩大到全体用户」，
「宽限期结束后进入受限运行：业务写入、审批、集成出站和自动化任务停止」。
阶段 3 计划第 3.4.11 节按裁定 A-05 冻结的枚举逐字：
`pub enum LicenseStatus { Valid, ExpiringSoon, Expired, Revoked }`。

两组各四个，但不是同一组四个。**`Expired` 一个变体盖住了规格的宽限期与受限运行两态**，
而这两态的后果相反、相隔 30 天。

**这一条的严重性不在「调用方判不出来」。** 同一份计划第 3.8.2 节的集成测试场景清单
已经把 `Expired` 的后果钉死：「许可过期与吊销各触发一次 `LicenseStatus` 由 `Valid`
迁到 `Expired` 与 `Revoked`，两种情形下……业务写入按规格第 3.4 章的受限运行处置」。
照这句的字面读，`Expired` 即受限运行——**于是规格承诺的 30 天宽限期整段消失，
一家二三十人的公司在到期次日就开不出单**。反过来，若把 `Expired` 一律当成宽限期，
受限运行这一态在整个系统里永远不发生。

注意计划那句自己写的是「按**规格第 3.4 章**的受限运行处置」——它把「什么是受限运行」
交回规格，而规格说受限运行始于宽限期结束。所以更可能的读法是那条验收用例的
`valid_to` 本就在 30 天之前，而不是计划在主张「到期即受限」。**两种读法都讲得通，
这正是要登记的理由。**

**实现当前的处置**：落两个函数。`classify_subscription` 照 A-05 冻结的四变体给状态值，
`in_restricted_run` 照规格第 3.4 章给后果（撤销立即受限；否则 `today - valid_to > 30`）。
两者在宽限期那 30 天里给出不同答案，并有一条用例专门钉住这个差异。
`LicenseStatus` 不提供任何 `blocks_business()` 之类的方法——
调用方拿不到「从状态值直接推出停不停写」的捷径。

**改判的改动面**：`crates/platform/license/src/status.rs` 的 `in_restricted_run`
一个函数，加它的三条用例（`expired_does_not_mean_restricted_during_the_grace_period`、
`nothing_before_expiry_is_restricted`、`revocation_short_circuits_everything`）。
若改判为「到期即受限」，须同批说明规格第 3.4 章的宽限期条款如何处置——
删掉这个函数而不动规格，等于让规格的一条正面承诺无声失效。

### 辛-6　临期窗口：规格 60 天，计划 30 天；而「切换只改配置」无配置可改

**两处原文。** 规格第 3.4 章逐字：「到期日前 **60 天**进入临期告警：全部功能可用，
管理员在控制台和登录后可见告警。」
阶段 3 计划第 3.4.11 节逐字：「……距 `valid_to` 不足临期窗口为 `ExpiringSoon`；
否则 `Valid`。**临期窗口取 30 天**，属本阶段临时取值，切换只改配置。」

到期前第 45 天这一天，按规格应有告警，按计划是 `Valid`、无告警。

**两个数的差别只落在告警提前量上**，不改变任何一天的停写判定——
规格明写临期告警期间「全部功能可用」。这一点必须写清楚，
否则容易把它误当成与辛-5 同级的问题。

**「切换只改配置」这句在本阶段没有承载物**：阶段 3 计划第 3.7 节的配置项全表
不存在任何 `EP__PLATFORM__LICENSE__*` 键，`docs/config-reference.md` 全文
`license` 零命中。这个数现在只能是一个 Rust 常量。

**实现当前取 30**，理由只有一条：计划是本阶段的落码依据且它自陈临时。
**不是因为裁定 A-05 定了 30**——A-05 全文不含任何天数，
落码文档里已明写不得把这个数挂到 A-05 名下。

**改判的改动面**：`EXPIRING_SOON_WINDOW_DAYS` 一个常量，
加 `boundary_closures_are_pinned` 与 `the_two_thirty_day_windows_are_separate_constants`
两条用例的期望值。宽限期那个 30 是另一个常量、另一处出处，改判时不连坐。

### 辛-7　永久授权与订阅授权在 `license_grants` 上不可区分　**已由裁定 F-17 处置**

规格第 3.2 章逐字「同时支持永久授权与订阅授权」；第 3.5 章「维护订阅到期后的行为」
第一条逐字「永久授权部分继续可用：**平台不因维护订阅到期而停机**，
已交付功能保持可用，历史数据访问与导出不受限制」。
而规格第 3.4 章的四态机挂在订阅授权上——节标题逐字「**订阅**许可生命周期与离线计量」，
开篇逐字「**订阅授权**在完全离线……环境下同样必须可运转」。

**阶段 3 计划表 26 的 `license_grants` 十列里没有区分二者的列**，
也没有维护订阅到期日这样的第二个日期列。于是一份永久授权的凭证进到状态判定里，
`valid_to` 过后一律得 `Expired`，调用方照辛-5 的读法进入受限运行——
与规格第 3.5 章「不因维护订阅到期而停机」正面相抵。

**实现当前的处置**：判定函数命名为 `classify_subscription`，
把适用范围写进名字与文档，并在 crate 文档的未覆盖段逐字登记本条。
不加 `grant_type` 入参——那一列不存在，要求调用方填它等于要求它编一个值。

**为什么不收进本附录的正常处置轨**：改判要加库列、加迁移、改判定入参与全部用例，
远超「一个函数加它的用例」的门槛。本条只登记现状与实现取的诚实形态，
正式处置须走裁定，并须同批回答一个更前置的问题：
**首版到底交不交付永久授权**（规格第 3.2 章说交付，阶段计划全卷没有一处为它建过任何东西）。

### 辛-7a　法规基准日期的界面与单据标注　**已定谳**（使用方表态「做」；形态、渲染点与归属见裁定 F-33）

由裁定 F-17 结论五从辛-7 拆出，**不随永久授权一起延期**。

规格第 3.5 章逐字「依赖法规口径的功能在界面和相关单据上显著标注所用版本及其
**法规基准日期**，并提示可能存在申报偏差风险」；第 21.14 章逐字
「未持有有效维护订阅的客户按第 3.5 章在界面与单据上显著标注所用规则版本、
生效日期与申报偏差风险」。

**「法规基准日期」在十七份阶段计划与 PRD 里零覆盖**：没有一张表存它、
没有一个界面渲染它、没有一个阶段建它。发票的税率选项表六列无生效日期无版本，
发票打印只调模板渲染与 PDF 渲染两个端口。

**为什么不能随辛-7 延期。** 第 21.14 章那处的触发条件挂在「未持有有效维护订阅的客户」上，
而本部署无维护订阅概念，条件不适用——但**它防的风险与商业形态无关**：
财税法规改了而系统没升级，开票与申报的口径就是旧的。自有企业内部使用同样要知道
自己算的是哪一版法规。删掉商业那层不该把这条实质义务一起带走。

**另一层：它独立于永久授权。** 即便首版就交付永久授权，这一条照样缺——
规格第 3.5 章同章逐字「首版不提供带生效日期的独立法规规则包」已经把
「所用版本及其法规基准日期」的取值来源取消了。**要标注的对象在首版不存在。**
所以本条真正要裁的是两问：首版要不要有一个「已安装法规口径版本」的登记物；
若要，它的界面与单据渲染点归哪个阶段。

### 辛-8　三件无下游承接方的事，登记备查　**已由裁定 F-18 处置**

以下三条不是「留给阶段 13b」——逐份下游计划核过，没有承接方：

**其一，可信时间的日落盘。** 规格第 3.4 章逐字要求判定基准取三者最晚值，
其中第二项是「平台本地许可状态存储中**按日落盘**的最近一次时间戳」。
`license_grants` 上没有这一列；阶段 3 计划第 3.2.4 节 job-worker 的任务清单
（封闭列举十二项）里也没有按日落盘的任务。后果是规格
「本地系统时间早于可信时间时按可信时间判定状态」这条**防时钟回拨的能力现在落空**——
拦不住的恰是「把系统时间调到签发日之后、到期日之前」这一手。
而规格第 17 章又把系统时钟漂移列为混沌注入的必测六类之一。

**其二，许可临期告警没有触发源。** 阶段 3 计划第 3.10.2 节的 PRD 条目追溯表把「许可临期」
一类的触发源指回本阶段，而本阶段按第 3.5.9 节「不新增对外端点」、
按本轮交付只有纯判定函数。`notify` 侧的写入接口已备，
计划第 3.4.5 节甚至用这条告警标定了扇出规模，`fanout.rs` 也为它写了用例——
**一个已被两处引为依据、却没有任何东西会产生它的通知**。
形态同「八档退避最后一档排不上」，只是发生在 crate 之间。

**其三，许可凭证的签名从生到死不被校验。** 表 26 的 `signature bytea not null`
是必填列，而规格第 3.2 章逐字「许可证使用数字签名离线验证，不依赖持续联网」
是正面能力承诺。全卷未给算法与密钥引用（对比审计侧 `SIGNATURE_ALGORITHM` 与
`SIGNING_KEY_REF` 两个配置键都在）；验签的自然落点是导入路径，而导入路径按其一同理无承接。
**一个必填却从不被校验的签名列，是恒真判据的一种变体。**

三条均已在 `crates/platform/license/src/lib.rs` 的未覆盖段逐条明写，
不以「校验过了」的外观掩盖。正式处置须走裁定。

### 辛-15　审计事件的日期没有任何时钟可信性判据，而唯一的检测项在本平台永久未覆盖　**须走裁定**

由裁定 F-18 结论一从辛-8 其一拆出。**与商业许可无关，内部使用同样暴露。**

**证据链五条，逐条实测：**

一、审计事件按法人与自然日分段，段键由 `crates/platform/audit/src/segment.rs` 的
`from_occurred_at(legal_entity_id, occurred_at: DateTime<Utc>)` 算出——
**一个纯换算，对 `occurred_at` 的时间源不加任何校验。**

二、`crates/platform/audit/src/chain.rs` 的 `verify_segment` **零时间断言**：
实测 grep `occurred_at|taken_at|Utc|DateTime` 在该文件内无任何命中，
它只比 `prev_hash` 与重算 `hash` 两件。

三、唯一的时钟异常检测在本平台**永久未覆盖**：
`crates/platform/runtime/src/selfcheck/items/basic.rs` 在非 Linux 下逐字返回
`SkewReading::Unavailable("本平台不提供 adjtimex，时钟偏差未覆盖")`，
而阶段 1 计划逐字已裁死「**该自检项在本平台永久停在「未覆盖」并就此登记**」。

四、**时钟回拨告警路由到那个永久未覆盖的项。** 阶段 1 计划逐字
「回拨超过 1000 毫秒时另触发自检项 9 的告警路径」，而自检项 9 正是
`clock-skew-within-limit`——即第三条那一项。**告警无处可落。**

五、`Clock` 端口**在代码里还不存在**：`crates/foundation/src/port/mod.rs` 全文
只有 `db`、`doc`、`kms`、`search`、`sensitive`、`tx` 六个 `pub mod`。

**后果。** NTP 误配、虚拟机快照回滚、主板电池耗尽任一发生，
审计事件被归到**它没有发生的那一天**，而链验证照常全绿——
这是第三类缺陷（错了不会当场报错）的教科书形态，
且它伤的是规格第 12.5 章那条不可篡改审计的证据价值。

**为什么不随辛-8 一起延期。** 辛-8 其一的被测对象是 `license_grants` 上不存在的一列，
本条的被测对象是 `audit_events.occurred_at` 与段键——**两列都存在、都必填、
F-18 之后照常写入**。商业许可撤掉不改变这一条一分一毫。

**可裁范围（本条不代裁，列出三问）：**
一、审计追加路径是否需要一条「本段已有最大 `occurred_at` 的单调下界」写入侧断言——
   它拦不住整体时钟漂移，但拦得住段内回拨，且是纯脱库可判定的；
二、`clock-skew-within-limit` 在本平台除「重新生效谓词」之外是否需要一个真判据
   （Windows 侧的可用替代是 `GetSystemTimeAdjustment` 与 `w32tm /stripchart`，
   本条不预设结论）；
三、全卷是否需要一条授时源的部署要求——实测「w32time」与「时间服务」
   在全卷除该自检项自身之外零命中。

### 辛-9　`instance.state` 的八个取值里，六个在求值时刻恒假

**出处两处，都在阶段 3 计划第 3.4.8 节内。** 一处逐字给出守卫可引用的字段：
「支持字段引用（`vars.x`、`instance.state`）」，未限定取值域。
另一处逐字给出单步事务的动作顺序：「加载实例行并 `FOR UPDATE`，
按 `definition_version` 加载定义，**求值守卫条件选出下一节点**，执行该节点，
写 `process_steps`，**更新实例状态**与 `next_wake_at`，写 Outbox，写审计。」

求值发生在**更新实例状态之前**，且在推进路径上。按同节的实例状态机表，
能走到推进的来源态只有 `RUNNING`（步骤成功且有后继）与 `WAITING`（唤醒，
且它先迁到 `RUNNING`）。于是另外六个取值写进守卫即是一条**永远不成立的分支**：
`CREATED` 只在首次派发之前存在而那一步无守卫；`COMPLETED`、`FAILED`、`CANCELLED`
三个终态按状态机表没有出边；`COMPENSATING` 走的是按步号降序的补偿路径，不经守卫；
`MANUAL_INTERVENTION` 恢复后先迁到 `RUNNING` 再推进。

这与本卷已栽过的「八档退避最后一档永远排不上」是同一形态，只是发生在取值域上。

**实现当前的处置**：**不在解析期拒绝那六个字面量**——计划只说它是一个字段引用、
没有限定取值域，替它限定就是自造规格。改为把事实暴露成一个可断言的公开面：
`ep_platform_flow::expr::GUARD_TIME_STATES`（两个取值）与
`Guard::unreachable_state_literals()`，让发布期的校验方能把它报给写守卫的人。

**改判的改动面**：`crates/platform/flow/src/expr/mod.rs` 的 `GUARD_TIME_STATES`
一个常量，加它的两条用例（`state_literals_that_can_never_hold_are_reported`、
`reachable_state_literals_are_not_reported`）。若改判为「解析期直接拒」，
改的是同一处加解析器一个分支。

### 辛-10　同一个管理员会写两套表达式语言，而全卷没有一处要求它们一致　**超出本附录门槛，须走裁定**

阶段 3 计划第 3.4.8 节逐字把本轮的求值器与规则引擎切开：
「该求值器只服务于流程守卫条件，**不是 `RuleEvaluator` 的实现**。」
阶段 13 计划另有一套：声明式规则的 AST 与解释器落在 `ep-platform-meta` 的 `rule` 模块，
客户端外壳 `ep-client-rules` 逐字「直接复用 ep-platform-meta 的 `rule` 模块」，
且第 8.4.2 节逐字「指标表达式复用本阶段的声明式表达式 AST 与解释器」。

于是首版最终会有**两套表达式语言**：本轮的中缀守卫语言（十进制定点、严格空语义、
单引号字符串、单段路径、三个函数）与阶段 13b 的声明式规则 AST（下发到四端、
驱动自定义规则与企业指标）。**两者由同一个管理员在同一套低代码界面上写**，
而全卷没有一处要求它们的语法、数值语义、空语义或函数白名单一致，
也没有一处禁止它们不一致。

这是本轮选择中缀文本而不是 JSON AST 所付的真实代价里最贵的一项
（选中缀的理由见 `crates/platform/flow/src/expr/lex.rs`：JSON AST 会把守卫自己的
数字字面量在解析那一刻变成 `f64`，阈值本身被污染）。两条路各有一处硬伤，
本附录不选，只把这件事登记出来。

**为什么不收进正常处置轨**：改判要动阶段 13 的规则 AST 设计与两处解释器，
远超「一个函数加它的用例」。正式处置须走裁定，并须同批回答一个更前置的问题：
**两套语言到底该合成一套，还是该在界面上明确分成两个不同的输入位**。

### 辛-11　守卫求值器的两条前提都没有承接方　**登记备查**

**其一，「本 crate 不引 `serde_json`」这条没有机检承接。**
它是守卫数值精度的全部前提：`variables` 在库里是 `jsonb`、PostgreSQL 用 numeric
精确存，而 `serde_json` 未开 `arbitrary_precision` 时**在 `from_str` 返回之前**
就已经把带小数点的字面量变成了 `f64`——求值器再怎么写也救不回来。
本轮把它做成类型事实（`ep-platform-flow` 的依赖只有 `ep-foundation`、
`rust_decimal` 与 `chrono`），但 `archcheck` 只判层位与环、
不按 crate 逐项比对期望依赖清单（阶段 3 计划第 2 节逐字「不另立按 crate 逐项比对
期望依赖清单的自检脚本」），**一次 `cargo add serde_json` 就能把它推翻而六道门禁全绿**。

实现侧留了一道退而求其次的探针：`GuardValue::number` 拒绝一切带 `e`／`E` 的取值。
`numeric(18,2)` 的文本输出从不带指数，一个带指数的取值进到这里本身就是上游走过
`f64` 的证据。它拦不住全部腐化（`f64` 在小量级上的 `to_string` 不带指数），
但它是本模块唯一一处能察觉那件事的地方。

**其二，发布期没有承接方调 `Guard::parse`。**
本轮把能在发布期判的全部前移到解析（文法、白名单函数名与元数、路径形态、
嵌套深度、源长度、数字字面量精度），但 `FlowDefinitionApplier::validate` 的签名
里没有任何一处要求它校验守卫。不接的话，一条语法错的守卫会一直躺到
某个实例走到那条边的那一刻才炸——而那时它炸在持 `FOR UPDATE` 行锁的单步事务里。

两条均已在 `crates/platform/flow/src/expr/mod.rs` 的未覆盖段逐条明写。

### 辛-12　`recon_runs` 既是仅追加表，又有 RUNNING 到终态的状态机　**已由裁定 F-14 处置**

**两处原文，都在本文件内。** 裁定 A-06 给这张表的列定义逐字含
「`status` text CHECK in RUNNING, COMPLETED, UNFINISHED, FAILED、`batch_total` int、
`batch_done` int、`started_at`、`finished_at`」——一个起始态加三个终态，
外加两个要边跑边推进的计数器与一个结束时间。
裁定 B-02 的登记表逐字把同一张表登记为「| 阶段 9a | `platform_core.recon_runs` |
APPEND_ONLY | `'{}'` |」，而同一条 B-02 逐字规定「`mutable_columns` 是可变列白名单，
取 `APPEND_ONLY` 时必须为空数组」。

阶段 2 计划逐字定义 `platform_core.assert_append_only()` 是
「BEFORE UPDATE OR DELETE 触发器函数，**一律 raise**」。
于是 `RUNNING` 到终态的那次更新**上线即被无条件拒绝**，
`batch_done` 与 `finished_at` 两列同理写不进去。

**后果是一个取不到的取值**：实现方只能绕过 `RUNNING` 直插终态，
`ReconRunStatus::Running` 与关账闸门里对应的 `CloseBlocker::ReconRunning`
整条成为死路径——包括它的文案「对账正在执行中，请等其结束」。

**同一条 B-02 的判据现成，却没有适用到这张表**：它在同一节里逐字用
「`platform_audit.audit_segments` 有状态与锚定时间更新，登记为仅追加会拒绝锚定写入，
不进本清单」判掉了另一张同形态的表。判据一致地适用，`recon_runs` 也该不进那份清单。

**实现当前的处置**：`ReconRunOutcome::running` 保留（A-06 的取值域里有它、
闸门对它有一条分支），但在其文档与 crate 的未覆盖段逐字写明它大概率取不到，
不让读者误以为那是一条活路径。

**改判的改动面**：本条不是「一个函数加它的用例」能了结的——
要么把 `recon_runs` 从仅追加登记里撤出（动 B-02 的表与阶段 9a 的迁移），
要么把 `RUNNING` 从 A-06 的 CHECK 里去掉（动取值域与闸门的一条分支）。
**须走裁定**，两条路各自的连带面在裁定时一并给出。

### 辛-13　`FAILED` 全卷没有产生条件，下游也没有接收方　**已由裁定 F-14 处置**

### 辛-14a　死信 `REPAIRED` 与 `DISCARDED` 的触发方　**已由裁定 F-15 处置**

由 F-13 结论末节移出、指定「由阶段 3 单裁」。F-15 的处置见该节，
另在核查中查出两处此前未登记的缺陷，一并由 F-15 承接：
类乙死信（消费成功后子项事务失败）的重投是静默空转，
以及转死信的 `INSERT` 在重投再失败时撞唯一键、使 E2E-5 那条验收路径走不通。

规格第 10.2 章逐字把五类终止成因——「单批执行时限触发终止、单查询内存或临时空间
上限触发终止、执行进程异常退出、连接被回收与快照失效五类」——**全部**归入未完成；
同章给一次关账的几种结束方式里没有「失败」这一种。
全卷 `FAILED` 只在裁定 A-06 那一行 CHECK 定义里出现过，再无第二处给它产生条件。
阶段 14 的降级 `kind` 取值域里只有 `RECON_RUN_UNFINISHED`，没有对应 `FAILED` 的一项——
即便产生了一次 `FAILED`，它的降级窗口该开哪一类也判不出来。

**实现当前的分界是本 crate 自定的**：`Unfinished` = 至少有一个阻断性校验项
没产生结论且**归因得到具体的 code**；`Failed` = 一个结论也没有、一个归因也给不出
（注册表为空，或运行在任何一批派发出去之前就断了）。
这条线切在**输出的可用性**上而不是成因上，因此不违逆规格对五类成因的归属，
且使四个取值全部可达。

注：`ReconRunStatus` 的这条分界注释在上一轮（`8774e62`）就已写进 `model.rs`，
当时没有登记出处，本条补登。

**改判的改动面**：`crates/platform/recon/src/executor.rs` 的 `summarize_run`
一个函数，加它的三条用例。但 `FAILED` 的降级承接方须由裁定一并指定，
否则改判之后它仍是一个没人接的取值。

### 辛-14　差异事项的三个已处置取值全卷没有生产者，「差异清零」因此无解除路径　**已由裁定 F-13 处置**

`platform_core.recon_discrepancies.state` 的取值域是
`OPEN`、`REPAIRING`、`REPAIRED`、`WAIVED` 四个。`OPEN` 由校验项产出，
另外三个是运维处置**之后**的态——而九个阶段计划里**没有任何一条**给出这三个迁移的
端点、用例或承接方。`repaired_by` 一词在全部计划文件里只在裁定 A-06 的表定义里
出现过一次。

规格第 10.2 章逐字「校验不通过时生成对账差异事项交数据责任人处理，
差异清零前不得关账」，阶段 3 计划的通知扇出逐字「对账差异取该法人的数据责任人」——
**两处都只到通知为止**；阶段 9 的待决项只说三类事项「同时进入运维中心与
财务侧的关账请求详情」，那是呈现不是处置。

**后果有三层**：`ReconDiscrepancy` 的 `validate_waiver` 守的规则今天没有任何调用方
能触发；关账闸门里 `!is_settled()` 的过滤在生产上等于「全部计数」，
因为差异永远停在 `OPEN`；于是「差异清零前不得关账」成了一条
**没有解除路径**的约束——一个期间只要出过一条差异就再也关不上。

这一条比三个 trait 的任何一处都更早该被点名。

**处置见裁定 F-13。** 该裁定走的是本条给的第二条出路，且推翻了本条的前提：
关账拦截读的从来不是累计行集，而是**本次校验的校验项结论**；
解除路径是规格逐字的「期间保持打开、按事项载明的内容修复后重新发起关账」，
是补登与冲正来源事件这类业务动作，不是给差异行置态。
因此首版不提供写端点，三个已处置取值登记为不使用而非撤列，
读侧露出归阶段 9b。死信侧的两条同形边移出为下一条。

## 附录丁　历史未裁登记（现已全部关闭）

本附录保留裁定 F-04 与 F-05 落地时的历史登记。丁一三项已由 F-51/F-52 关闭，丁二也已全部处置；旧“须另行处置”与“刻意不裁”只说明当时状态，不得作为当前开发阻断。

### 丁一　原文本矛盾（3 条，现已全部关闭）

| 编号 | 事项 | 矛盾所在 | 须由谁定 |
|---|---|---|---|
| ~~D-01~~ **已关闭，见 F-51** | `derive_blind_key` 的返回宽度 | BlindIndex 固定完整 32 字节 | 不再待定 |
| ~~D-02~~ **已关闭，见 F-52** | 阶段 13b 自动测试 suite 的执行落点 | 固定九套；中立 SPI 归 `ep-platform-release`，四个属主实现并在 job-worker 精确注册 | 不再待定 |
| ~~D-03~~ **已关闭，见 F-52** | 自动测试从 core-server 受理到 job-worker 执行的派发载体 | 复用 `config_packages` 队列字段与九条 `config_autotest_runs`，不新增事件 | 不再待定 |

### 丁二　本轮清单未点名，留待下一轮（5 条）　**已于其后一轮全部处置**

| 编号 | 事项 | 处置 |
|---|---|---|
| D-04 | A-06 的注册方与校验项计数有两套 | **原判有误，已裁定并回写。** 逐阶段实测点数为阶段 7 六项、阶段 8 两项、阶段 9b 四项、阶段 11 三项，合计十五，注册方四个。阶段 10 全文只有 `FIN_CROSS_MODULE_LINK` 一项，而该项的 `category` 取值 `CROSS_MODULE_LINK` 已被同一条 A-06 撤销（跨 schema 单目标引用改建复合真实外键），一个 category 不存在的校验项不能计入。决定性证据是本文件 A-06 段自身打架：标识符表把 `recon_check_definitions.category` 的 CHECK 写成两项，同节散文仍要求阶段 10 取第三个值——表已回写、散文漏写，漏写的一侧就是「五个/十六个」。共改八处复述，并顺带修两处连坐事实错误 |
| D-05 | 单文件 `wiring.rs` 措辞 | **成立，已改，范围远超登记。** 实测八个 apps 的 `src/` 下只有 `wiring/` 目录、不存在任何 `wiring.rs` 文件，故该措辞在全卷都与制品不符，不只是登记点名的一个文件。共改四十余处，并推翻三处此前明写「不逐处改写、按口径声明解释」的豁免条款——既然逐处改写，豁免即失效 |
| D-06 | 六处只写「KMS」不写 crate 名 | **成立，但登记的范围两头都不准，已按实测处置。** 点名的六处里只有两处属落点性表述；另四处分别讲超时预算、依赖不可用、超时注入与载体中断，指的是「KMS 作为外部依赖这件事」，补 crate 名反而把载体与端口混为一谈，不改。另在登记未点名处补三处落点性表述 |
| D-07 | 阶段 7 的对外 trait 计数 | **原判有误。** 全卷没有任何一处为 `ep-contract-procure` 写过 trait 计数，只有枚举（实数 6，与 F-05 一致），不存在可漂移的数字；登记写的落点小节也不准。但同形缺陷真实存在，落在 `ep-contract-inventory`：C-18 与 A-12 当时写下的「五个 trait」共四处，已一并改到位 |
| D-08 | 阶段 12 的 job-worker 职责列 | **确认不冲突，无需改动。** 发布 Outbox 事件与产出 `SearchDocument` 是两件事。保留为落码期核对项：索引消费者须调用 `ep-app-service` 与 `ep-app-project` 的投影函数，而非 `ep-adapter-search` 内的投影 |

### 丁三　本轮撤销的一条越权编辑

回写期间，阶段 11 的编辑方与技术基线的编辑方对同一处冲突各做了一半且方向相反：
基线一方把通则第七条通道二的角色约束收为「常规报表与经营看板一侧」并登记入第 12.1 节；
阶段 11 一方则把三个 costing 视图的 `GRANT SELECT` 由 `ep_analyst_ro` 与 `ep_app_rw`
收窄为只给 `ep_analyst_ro`。合成裁定明写「二者只能留一个」。

取基线一方，撤销阶段 11 的收窄。理由是后者并未消解真正的冲突——
第 186 行的三个视图属 costing，而第 630 行的 `ReconExecutor` 读的是
`inventory.v_stock_value_entries` 且跑在 job-worker 自身连接池上，
收窄 costing 视图的授权对该情形毫无作用；而撤销一个既有 `GRANT` 是运行期权限变更，
若「无用例读这三个视图」的判断有误，故障在编译期没有任何信号。

## 附录戊　计数与枚举失配的普查结果

本附录是清理附录丁二时同批做的同类普查的结果。判据：凡文中出现「N 个」「N 条」
「N 项」并且紧邻处有对应的枚举、表格或清单的，逐处数一遍枚举项数比对；
同一个量在两个及以上文件里各写一次的，比对取值。扫描面为 18 个计划文件全部。

共查出 12 条。**其中 10 条已在本批改完，2 条留待下一轮**，逐条如下。

### 戊一　已改（10 条）

| 编号 | 落点 | 原值 | 实测值 | 依据 |
|---|---|---|---|---|
| 戊-1 | `00-overview.md` 第 2 节阶段 4 行 | 授权十六表 | 十五表 | 阶段 4 第 3.3 节标题写「15 张」，逐表 3-10 至 3-24 实数 15；原第 16 张已按 C-06 移入 platform_core |
| 戊-2 | 同表阶段 7 行 | procure 十五表、portal 六表 | 二十三表、七表 | 阶段 7 五处自述「三十张表」，23+7=30 |
| 戊-3 | 同表阶段 10 行 | finance 二十六表 | 二十三表 | 迁移第 1 至 17 号实建 23 张，与该阶段三处「36 张表」互洽 |
| 戊-4 | 同表阶段 11 行 | reporting 七表 | 九表 | 第 3 节逐表实数 9，与退出条件的「11 张新表」「18 条迁移」互洽 |
| 戊-5 | 同表阶段 12 行 | service 十表 | 十三表 | 该阶段 D-04 逐字「service schema 的 13 张表」，D-09「18 张带法人表」 |
| 戊-6（历史，已被 F-53 及后续加固更新） | 同表阶段 14 行 | platform_ops 十九表 | F-53 前十七表；F-53 当时二十一表；现行全仓终态二十四表 | 当时第 3.1 节只有表 1 至表 17；F-53 先新增四张法人 RLS 历史迁移台账，后续证据图加固再增批准证据与 writer 回执两表，使阶段 14 第 3 节现行为表 1 至表 23；阶段 13c 的 `ai_model_packages` 是全仓第 24 表 |
| 戊-7 | `00-overview.md` F-03 行「最终归属」列 | 四条机检规则 | 五条 | 同一行的「确切标识符」列已写五条，行内自相矛盾；工具实测五条 |
| 戊-8 | `00b` 第 1.2、1.3、12 节三处 | 四条规则 | 五条 | `foundation-marker-shape` 已由 `foundation-frozen-items` 拆出为独立规则 |
| 戊-9 | `01` 退出条件 3 | 四条规则合成 | 五条 | 同上 |
| 戊-10 | `14` 第 1 节 | 八个子命令 | 十一个 | 阶段 1 的 D-08 与退出条件 10 两处逐字「十一个」并逐一枚举 |

配套：本批为 `foundation-module-registry` 与 `foundation-no-single-owner` 两条规则
补齐了规则级负样例（此前只有辅助函数的单元测试，规则本身没有负样例），
使「五条替身各配负样例」这句话为真而不只是声称。

### 戊二　留待下一轮（2 条）

| 编号 | 事项 | 为何本轮不裁 |
|---|---|---|
| 戊-11 **本轮判为不可判定，已移入庚二** | 阶段 4 的「入口借用测试」项数有三套互斥分解 | 基线第 8.4 节写「五个入口借用测试」指三个被测对象合计五项；阶段 2 拆成「复制角色 5 项 + 系统上下文 5 个入口」共十项；阶段 4 拆成「复制角色两项 + 对账上下文一项 + 只读角色两项」共五项且引入基线未提的两个只读角色。三套的被测对象都不同，不是数字统一问题，须先裁定被测对象是什么 |
| 戊-12 **已裁定，归属 F-07** | A-24 行的「八个勾稽视图」 | 同文件另两处与阶段 10 均写「十项勾稽」。「八个」可能是刻意指其中八项（存货与 GRNI 两项子账侧来自外部端口，与期初通道无关），也可能是十项的旧值。证据不足以定，标注不确定 |

### 戊三　一条成因

12 条里有 6 条落在同一处：`00-overview.md` 第 2 节十四阶段总表的「关键交付物」列。
该列是一份手抄摘要，抄的是各阶段计划第 1 与第 3 节的表数，而**没有任何机检承接方**。
各阶段计划自身的计数全部自洽——普查逐一数过阶段 2 至 14 的表数、迁移数、事件数、
错误码数、指标数，无一处内部失配。失配全部发生在抄写这一步。

按第 12 节通则第六条，这类摘要要么给判据，要么明写它不构成规范来源。
本轮只改数值，未给该列建判据，登记为下一轮事项。

### 戊四　为总表建机检门禁的可行性评估：结论为当前不可行

上一轮把总表六行的表数改正之后，本轮做了两件事：把剩下五行核完，
并专门评估能否给这一列建一条机检门禁。

**核完的结果**：阶段 5、6、9、13 四行相符，阶段 2 一行有问题。
连同上一轮的六行，十一行里七行与阶段计划不符。阶段 2 那一行的问题不是数字算错，
而是它写「tenancy 五表」——`tenancy` 根本不在二十四个 schema 之内，
它只是承载 crate `ep-platform-tenancy` 的名字，那五张表物理上在 `platform_core`；
且该行漏掉了本阶段绝大部分建表量。已按同表体例改为
「platform_core 十三表（含组织架构五表与迁移窗口单例锁表）与 platform_ops 一表」。

核阶段 2 时另查出两条本身的缺陷，一并修在阶段 2 计划里：

其一，该阶段四处按表号手数的计数一律少 1，根因是第 3.5 节表六的标题行里
声明了两张真实表——`migration_windows` 加单例锁表 `migration_window_lock`。
可机检的一侧（未受行级策略表的八行登记清单与 E-05 的第 13 项断言）把锁表算在内，
指向十四张；手数表号的四处指向十三张。取十四。

其二，`migration_window_lock` 只有 `id smallint` 一列，既无行版本也无审计列，
而 `db/checks` 第 01 项对全库断言公共列齐备且全文无任何豁免声明。
按现文执行，该项必返回非零行，E-05 恒不通过。已补豁免，
判据为该表登记在 `unpoliced_table_registry` 且行数由 `check (id = 1)` 固定为一行。

**门禁评估的结论：不可行，一行代码都不写。**

评估的对象是「用各阶段计划的迁移文件名清单反推建表数」。四个前提无一成立：

| 前提 | 实测 |
|---|---|
| 清单格式可解析 | 五种写法并存；阶段 2 有两行是花括号展开加**字面省略号**，正文里就写着 `…`，解析器只能放弃；阶段 6 根本没有清单，而它是表数最多的阶段之一 |
| 文件名可分建表与非建表 | 阶段 2、4、13、14 的迁移 slug 整段不带动词段，按 `_create_` 取集合命中零条 |
| 一文件恰一表 | 阶段 7 与阶段 11 恰好成立，阶段 10、12、13、14 不成立（阶段 13 有一文件五表） |
| 被测输入存在 | 全仓零个 `.sql` 文件、零个 `migrations` 目录 |

硬做的后果是把通则第六条禁止的三种退化形态占全：对阶段 2、4、6、13、14 恒不命中，
对阶段 7、11 给假绿灯，按真实文件面则是两个空集合比对恒真。
**它会把「这一列已被机检锁死」写进退出条件，而实际锁住的只有一半——比没有门禁更坏。**

另有两类结构性错配也说明这条路走不通：阶段 14 第 3 节现行二十三张表里含 `platform_ops.degradation_windows`，
其建表迁移在阶段 2 的清单内，任何按本阶段清单条数计数的门禁在阶段 14 至少恒少一、阶段 2 恒多一，
而这个偏差是设计使然不是缺陷；正则扫节还会把已撤销的文件名与占位名数进去，
这类污染方向恰好是掩盖漏表。

**按通则第六条取第一档「整条推迟」**，不占用第 12.1 节 undecidable 段。
重新评估的触发谓词写成工具自身可观测的文件系统谓词：
**`db/migrations/` 下出现至少一个 `.sql` 文件**。彼时被测输入变为真实 SQL 的
`CREATE TABLE` 语句，既不需要解析散文，也不需要先把三个阶段的逐表标记归一，
上述四个前提一次全部成立。在此之前，该列由本文件与 `00-overview.md` 两处
明写为阅读辅助、不构成规范来源，取值以各阶段计划为准。

历史更正：本节作成时，`00-overview.md` 的 A-26 行写「阶段 14 扩展为十九表五视图」，
而同文件阶段 14 行与阶段 14 计划退出条件 2 当时均为十七表五视图；十九是 C-22 撤销
`replication_crosscheck_runs` 之前的旧值。F-53 当时三处统一为二十一表五视图，来源是原十七表加四张法人 RLS 历史迁移台账；后续证据图加固新增两张法人 RLS 表，使阶段 14 第 3 节现行为二十三表五视图，再计入阶段 13c 的 `ai_model_packages` 后，全仓 platform_ops 终态为二十四表五视图。不得把本段任一历史数字当成现行终态。

## 附录己　全卷冲突审计的结果与未裁事项

本附录记七维度全卷冲突审计的结果。七个维度各一个查错员、各配一个专职反驳者，
合计报告 54 条，反驳后成立 49 条，去重后 38 条独立冲突。驳回率 9%，
没有任何一个维度被整体驳回。

### 己一　已处置（34 条）

代码侧 11 条与文档侧 23 条已在本批改完，逐条见提交说明。其中三条值得单记：

**其一，`testkit` 的八个 RLS 断言函数名与裁定 C-05 零重合。** C-05 早把八个名字
冻结为 `assert_read` 至 `assert_error_leak`，十处文档逐字一致，而代码用的是
按「八个断言函数」这个数目自编的另八个名。代码让步——阶段 1 退出条件 22 的判据
本身就是「与 C-05 逐字一致」，改文档等于把退出条件改成恒真。

**其二，`HumanContextInput` 的注释称 19 字段、实测 18。** 危害具体：按注释
「与上表一一对应」做机械映射，会把第 19 项 `account_kind` 补进入参，
打开从 HTTP 层伪造 `account_kind: System` 的口子。已改注释，并把这 18 字段
纳入 `foundation-frozen-items` 机检——此前它是冻结面上唯一无机检承接的计数。

**其三，四条规则声称配负样例而实际只测了辅助函数。** `unwired-absent`、
`db-pg-one-schema-per-file`、`foundation-frozen-items`、`foundation-marker-shape`
四条的规则入口零调用，而三处文档逐字要求「配一个故意违反的负样例」。
已补八个规则级负样例，并修掉最后一处「读不到文件即判通过」的路径。

审计另给出两条关于本卷自身的结论，记录备查：

> 代码的**行为**侧实测零冲突（依赖边、规则实现、字段计数），出问题的全在**说明**侧。
> 真正的风险不在代码写错，而在「代码声称被门禁看着、实际没有」。

> 裁定层报出的 12 条里 9 条落在本文件，且多数无判定后果——本文件已进入
> 维护成本高于信息价值的状态。下一轮宜把各条目「结论」「最终归属阶段」
> 「确切标识符」三段之外的部分整体标注为历史举证，而不是逐条追改。

### 己二　历史待裁登记（现行状态以 F-50 至 F-54、总体规格与已回写阶段正文为准）

| 编号 | 事项 | 为何本轮定不了 |
|---|---|---|
| 己-1 **历史裁定，已由后续 Windows 现行回写取代** | cgroup 九行三列配额表的存废 | 下文保留当时 Linux/cgroup 论证，仅作追溯，不是现行实现值。现行首版的跨进程比例配额只启用具名 Job Object 的内存硬上限；CPU 比例、CPU 突发上限与按权重磁盘 IO 份额固定不启用，静态限额文件出现这些比例字段即配置失败。未来启用必须另立产品版本与正式裁定，不存在待产品负责人选择的当前分支 |
| 己-2 **已裁定，归属 F-06；F-55 后计数同步** | 阶段 13 承诺的限流降级窗口取值 | 终态 21 个 `kind` 仍无任何限流或配额类取值；F-52 为周期核对无结论新增 `REPLICATION_CROSSCHECK_NO_RESULT`，F-53 为病毒扫描新增 `VIRUS_SCANNER_NOT_AVAILABLE`，F-55 再新增 `LEGAL_ENTITY_KEY_DOMAIN_UNAVAILABLE`，均不改变 F-06 删除限流降级承诺与继续撤销 `RESOURCE_QUOTA_EXPOSURE` 的结论 |
| 己-3 **已裁定，见 F-51 与正文后续更新** | 四端真机 PoC 首测的承接阶段 | 阶段 1 冻结门槛表；阶段 13a 前移移动薄 PoC。薄批只能产出切 Flutter 的否定结论，保留 Tauri 须完整全表通过；失败只替换移动 UI，Rust 核心九个 crate 不变 |
| 己-4 **已关闭，见 F-50** | 门户发票受理写端口的确切类型名与所属 crate | `ep_contract_portal::SupplierInvoiceUploadWritebackPort` 由 `ep-app-portal` 实现、`ep-app-invoice` 调用；不再待定 |
| 己-5 **已关闭，不需使用方表态** | 规格第 7.7 章「三项遏制手段缺一不得启用」的备选支路 | 逐字复核 `spec:790` 后本条不存在备选支路：该章的运行期例外只挂在第三项、只在角色已启用之后，且明写「不适用本条的停用后果」；阶段 14 主张的状态正是这一支，范围本就窄于原判所述。故不需修订规格第 7.7 与 21.21 两章、不需修订技术基线第 0 节的优先级条款、不需产品负责人与安全负责人表态，本行与附录庚一的对应行一并撤销。阶段 14 本体五处（`14:24`、`:452`、`:496`、`:559`、`:582`）已准确落实该区分，无须改动；`14:103`「本阶段不改其可抑制性」在改动由阶段 2 承担时仍成立，亦不改。关闭的前提是同批清除六处反转残留：`00-overview.md:259`（裁定 C-22 行）与 `00-overview.md:210`（裁定 A-26 行）**各部分撤销一个分句**、`02-data-foundation.md:689` 与 `:330`、`14-ops-backup-release.md:556` 与 `:590`。部分撤销两条已生效裁定各一分句的授权已由使用方给出，依据是技术基线第 0 节「本基线与规格冲突的部分一律作废」的优先级条款，且两个分句都与规格正面冲突并超出各自裁定的本题射程；两条裁定的其余内容一字不动，其状态列保留「已裁定」并加注「（本轮部分撤销一分句）」。`02:330` 等三处的依据是 `spec:1257` 自身「同样不可由管理员关闭」而非己-5 的方向，采超集口径：整个 `WRITER_NOT_IN_SERVICE` kind 不可抑制，代价是写出进程因日常维护停机时运维也无法静音该告警，窗口仍随条件消除自动闭合 |
| 己-6 **已关闭，见 F-50** | 门户发票 `UPLOADED → RETURNED` 由哪个端点承载 | 内部端点固定为 `/api/v1/portal/supplier-invoice-uploads/{id}/actions/return`，请求为 `reason,row_version`；不再待定 |
| X-1 **已关闭，见 F-52** | 周期核对三态与连续无结论的载体 | 复用 30 秒 WAL 保留量采样；三态与 streak 落 `archive_channel`，连续第二个 `NO_RESULT` 开 `REPLICATION_CROSSCHECK_NO_RESULT`；不恢复专用子系统 |
| X-2 **已关闭，见 F-52** | 写入角色遏制检查的 severity | 从 `offsite-sink-requirements` 拆成独立 `writer-role-containment` Blocking 项；其他进程 NotApplicable |
| 己-7 | 「本轮改的 T3-2 与 T3-4 依赖己-1 的裁定方向」 | 己-1 已裁为「计划让步、恢复规格口径」，这两条随之**回滚到「保留」一侧**，与该裁定的其余编辑同批处理。`T3-2` 与 `T3-4` 两个标识符经全仓检索只在本行出现，其余文件零命中，确切回滚落点须由改动方按其原工作清单核对——**不确定** |

本表只保留历史追溯。现行状态须与 F-50 至 F-54、总体规格和已回写阶段正文同读：其中全部真实待裁项均已有唯一值，不再等待表态或落码选择。

### 己-1 的历史裁定　规格第 13.1 章配额表的承载面、判据面与认证冻结口径

> **整节已被后续 Windows 现行回写取代，仅作论证追溯。** 本节以下关于 `MemoryLow`、`CPUWeight`、`IOWeight`、`IOMax`、cgroup/slice、恢复权重列或“仍待产品负责人决定”的命令式措辞一律不构成现行要求，也不形成待决。现行唯一值取总体规格第 13.1 章、技术基线第 2 节、阶段 1 第 5.6 节与阶段 14 第 8.5 节：首版跨进程比例配额只启用具名 Job Object 的内存硬上限；CPU 比例、CPU 突发上限与按权重磁盘 IO 份额固定不启用，静态限额文件出现这些比例字段即配置失败。其他绝对限额只按这些现行章节各自冻结的实现与证据状态解释。未来启用比例项须另立产品版本、正式裁定、配置 schema 与 Windows 实机发布证据。

**裁定方向：计划让步、恢复规格口径。不需使用方表态。**

#### 一、先修正底账：互斥不止一列，两侧都写不成立

- 规格第 13.1 章的配额表实测为九行三列。`00b:246` 的进程表实测八个进程映射到七个 app slice（core-server 与 integration-gateway 同处 `app-core.slice`），加 PostgreSQL 共八个 slice；`01:24`、`01:221`、`01:507` 三处一致写「八个 slice」。九行对八 slice，差的正是**内置搜索索引**一行（CPU 10%、内存 10%、IO 8%）——全仓检索该词只命中 `ep-adapter-search` 适配器与 `SearchIndexPort` 端口，它由 core-server 与 job-worker 进程内承载，无独立进程、无独立 slice、无 drop-in 承载物。
- `01:221` 同段内自相矛盾：先写「取值只有三类」，随即枚举了第四个键 `CPUWeight`；又写 `MemoryMax` 按附录 D.2 的 BC-1 基线组合算定后写死为绝对字节，而规格该列是「48%」「16%」这样的百分数，两者之间没有定义换算。因此 `01:24` 与 `01:507` 的「drop-in 取值与规格第 13.1 章配额表**逐行一致**」在行数与量纲两处同时无定义，是一条**写不出判据的退出条件**，这一支不能原样保留。
- 另一侧同样不成立：`00b:7` 逐字「本基线与规格冲突的部分一律作废」，据此 `00b:263` 的「CPU 一列整列删除」「也不进认证冻结范围」按基线自己这一句已经作废；`spec:1684` 更逐字写着「机器级资源配额与第 13.1 章的四类让路机制**首版按该章交付并认证**，本条登记的是其可执行程度的上界与由此产生的残余风险」——规格明确要求首版交付并认证该章配额，计划层删表无授权。

**让步方与落点：`00b:263`、`00-overview:333`、`07:1104`、`13:1022`、`13:1023`、`13:1065`、`14:562`、`14:585`、`14:624` 共九处让步，回到规格口径。** 原判自称「六处」，实测漏三处（`13:1023`、`14:562`、`14:624`），其中 `14:562` 是阶段 14 的退出条件第 10 项，漏改的后果不是措辞不齐，而是留一条与 `spec:1135`、`spec:1826` 正面冲突的验收线。另有阶段 1 的五处（`01:24`、`:221`、`:507`、`:531`、`:612`）属同一口径下的计数与判据修正，不算让步但须同批。阶段 1 的方向（表照抄、承载物换成静态 drop-in、不做生成算法、不做启动自检）保留，其「三类」计数与「逐行一致」判据由本裁定修正。理由的权重次序：标准 3（规格最高，且基线第 0 节自认作废）压过标准 4（改动面）；并且**恢复规格口径不需要产品负责人签字，删规格口径才需要**——这一不对称决定方向，也使本裁定不越过附录庚一原登记的决定人。

#### 二、承载面：四类取值，八行 drop-in

1. 每个 slice 一个 `MemoryMax` 与一个**同值**的 `MemoryLow`，取按附录 D.2 的 BC-1 基线组合由该表内存列算定的绝对字节。同值一条照抄 `spec:1151`「内存的保底值与上限同值，即 memory.low 与 memory.max 取同一取值」——两侧计划此前**一起漏了 `MemoryLow`**，这是本条唯一一处两侧都错的地方。
2. 每个 slice 一个 `CPUWeight`，取该行 CPU 份额百分数乘以 100。
3. 每个 slice 一个 `IOWeight`，取该行磁盘 IO 份额百分数乘以 100；archive-writer 与 PostgreSQL 因此天然高于 backup-writer。此项取代原「第三类」的特设次序约束，一条通则替掉一条特例。
4. backup-writer 另有一个 `IOMax` 硬上限（该键名是否为 systemd 侧的确切指令名未经实测，属既有文本沿用，落码时核实，本轮不改）。

两个权重列只表达八个 slice 之间的相对比例，与机器规格无关，不参与任何折算，**也不声称该 slice 取得整机的对应百分比**：同机的 `system.slice` 与规格第 13.1 章的操作系统预留不在这八行内，权重之和不构成整机分母。

**内置搜索索引一行的处置（本轮改判其理由）。** 该行**不落 drop-in、不加和、不拆分**，drop-in 是八行不是九行——处置动作维持。但原判援引的同形先例 `spec:1153`「运维代理的资源计入反向代理与运维代理一行，不单列配额行」**经全文逐字复核不存在**（`spec:1153` 实际写的是「文件存储的正文读写不单列配额，按发起该 IO 的进程计费」；「反向代理与运维代理」只是配额表第 6 行的合并行名，规格没有任何一句把它写成救济通则），该论证据此作废。改用的理由是：该行在首版无独立进程、无独立 slice、无 cgroup 承载物；加和会把 app-core 与 app-worker 的权重抬到规格没有写过的取值，拆分则需要一个规格未给出的分摊比例，两者都是新机制。**净损失如实披露，不得沉默**：八行权重之和不再是 100，CPU 侧分母降为 90、IO 侧降为 92，实际承载搜索索引的 app-core 与 app-worker 相对**欠配**，其余六个 slice 相对**超配**。该偏差与下述突发上限缺口一并写入 `00b:263`、`01` 第 5.6 节与交付说明。

删除项维持不变：按可分配量折算的生成算法、`min(份额×3, 40%)` 的突发上限算法、`cgroup-quota-matched` 自检项（裁定 C-25 已整项撤销，本裁定不重开 `01:201` 的 78 退出路径）、`platform_ops.quota_events`、保底份额击穿的两条件判定、`quotas.generated.toml`、`selfcheck.quota_manifest_path`。**但删除理由必须换**：现行理由「不构成运行期保证」是 `spec:1157` 自己已经承认的事，不能反过来当删计划文本的依据；正确理由是**判据恒不命中**——不设 `cpu.max` 时 `cpu.stat` 的 throttled 计数恒为 0，`io.stat` 的排队时延阈值随突发上限一并无来源，且 `spec:1170` 的第一个条件写的是「低于**上表所列份额**」这一整机百分比，而权重列不产生该被测量。本仓已两次裁定禁止恒不命中的门禁这一形态。

#### 三、判据面：撤下不可判定项，换可判定替身，不进 12.1 登记表

按第 12 节通则第六条的**第二种合法处置**（换一个被测输入已存在的可判定替身）：

- 可判定的一半：`CPUWeight` 与 `IOWeight` 两列与规格第 13.1 章对应行的百分数乘以 100 **逐行整数相等**——无换算、无行数缺口，八行对八行。
- 不可判定的一半：`MemoryMax`／`MemoryLow`／`IOMax` 与该表百分数之间无定义换算，因此**只**与 drop-in 自身及运行期 cgroup 比对，不与该表比对。这正是 `01:457` 的 E2E-05 已经在做的事，替身现成。

**不新增 12.1 登记行。** 本条走的是通则第六条三档处置中的第二档，不需要登记；而 12.1 两段由 `undecidable-registry-matched` 与 `xtask archcheck` 的运行期输出逐行比对，塞一条非 archcheck 规则的行会破坏该比对契约并连带改 xtask。另：`01:24`／`:457`／`:507` 的被测输入是 `deploy/` 与运行期 cgroup，`deploy/` 由阶段 1 自身的 D-05／D-06 交付，同阶段，不落入通则第六条禁止的「被测输入交付阶段晚于判据所在阶段」。须记明：`deploy/` 目录当前全仓不存在，本节的判据面在阶段 1 交付该目录之前无法实证，只能靠评审。

#### 四、门户攻击面：推理成立，但最小补法不是 `cpu.max`

**推理成立。** `cpu.weight` 只在竞争时约束份额；`spec:1150` 自己写明其余组件空闲时本组件可借用其空闲部分，被借用方一旦需要，内核在一个采样周期内把各方收敛回其权重份额。夜间洪泛正落在其余组件空闲的时段，被攻破的 portal-gateway 确可短时逼近整机 CPU 与磁盘 IO。**但补法不取 `cpu.max`／`io.max`，三条独立理由，任一成立即足够：**

1. `spec:1150` 突发上限的主语逐字是「其余各行」共七行。只给门户一行设，是规格里不存在的新例外；七行全设则等于把本裁定刚删掉的折算算法整体复活。
2. `cpu.max` 需知核数、`io.max` 需知设备号与带宽，两者都是**机器相关值**，加进去会破坏两个权重列「与机器无关」这一性质，并把 `spec:1135` 与 `spec:1684` 的现场调优禁令附着到最需要应急收紧的两个数上。
3. cgroup 节流**只延迟不失败**，产不出规格要求的「门户请求因配额限流而失败」的事件；且裁定 F-06 已定终态 21 类 kind 中无任何限流或配额类取值（F-52 新增项仅用于周期核对无结论，F-53 新增项仅用于病毒扫描器不可用，F-55 新增项仅用于法人密钥域不可用），加 cgroup 节流等于再造一个无 kind 可归的限流源。

**真正的最小补法就是第二节的第 2、3 类——恢复两个权重列，零新增机制。** 删 CPU 列不是「不管 CPU」：cgroup v2 的 `cpu.weight` 默认为 100，八个 slice 一个都不设，portal-gateway 与 PostgreSQL 就是 100:100，而规格给的是 3:44，删列把公网进程抬到与数据库同级，这是本条唯一一处实质安全回退。同理，`00b:263` 的「20 人一台机器上 CPU 不是稀缺资源」被证伪：20 是**人机并发上限**，而吃 CPU 的是备份压缩与加密、索引重建、报表渲染、WASM 执行、文档渲染，与人机并发无关；`spec:1684` 明写时延通过线在备份窗口内的样本子集上同样判定，而删列后 PostgreSQL 与 backup-writer 在备份窗口内正是 1:1 争 CPU。该理由整句删除。

**净损失如实披露。** 除 backup-writer 的 `IOMax` 外，突发上限一列在首版**无承载**；资源侧遏制只覆盖内存硬上限与两个权重份额。应用层限流器生效之前的 TLS 握手 CPU 消耗、以及来源分散的分布式洪泛这两条残余风险不消除，按 `spec:1373`（WAF 由客户提供运维、平台不验收其效果）**不得计入平台侧覆盖**。此项须写入 `07:1104` 的门户风险段与交付说明。

#### 五、认证冻结口径：不做相对／绝对二分，改用规格已有的「下限」语义

驳回「只冻相对列、不冻绝对列」：`spec:1135` 的冻结面含保底值，而 `spec:1151` 已把内存的保底值与上限定为同值，二分等于给风险最高的那一列开洞。口径改用 `spec:1826` 逐字的「作为交付客户的服务器规格**下限**」：

- 客户服务器规格**不低于**认证报告所记规格时，沿用该次认证结论，**不重跑附录 A.4**。
- 两个权重列与机器无关，**原样沿用**，一个数都不改。
- `MemoryMax`／`MemoryLow`／`IOMax` 由实施方按 BC-1 的**同一算定式**对全部八行**同批**重算并写入部署记录。因为是整机规格驱动的同批重算，不是 `spec:1135` 禁止的「**单方**调高**单一组件**的份额、保底值或突发上限」——那两个限定词正是本口径的合法性来源。
- 低于该下限的服务器不交付。
- 据此，`00b:263` 的「也不进认证冻结范围」与「硬件升级只改这三个数字、不重跑配额认证」两句撤销；`14:562` 的退出条件第 10 项与 `14:624` 的风险六控制列同口径改写，不得再出现「只保留三类调优取值」与「硬件变更只改这三个数字、不重跑配额认证」。

#### 六、越权自纠（与本裁定方向无关，无论如何都要执行）

`14:585` 逐字「规格第 21.19 章的风险条目随之作废，其诚实披露折叠进第 7.5 章」——阶段计划作废规格章节，直接违反标准 3；且 `spec:1684` 末句逐字「本条不是新增的延期项，不在第 5.7 章登记」，它连走延期通道消失都不行。第 21.19 章控制段承载的是写入交付说明与客户合同的披露义务，删它是删一条对客户的义务。**该句整句撤销，规格第 21.19 章全文保留。**

#### 七、历史上曾归产品负责人的问题（现已关闭，不形成待决）

本段记录当时「在规格未修订的前提下，计划侧该怎么写」的论证。该问题现已由后续 Windows 终局回写关闭：规格第 13.1 章的表保留为硬件标定与认证意图，首版 CPU 比例/突发与按权重磁盘 IO 固定不启用，不再等待产品负责人表态。若未来另立版本重新启用，`spec:1135`、`1150`、`1152`、`1157`、`1170`、`1826`、`1839` 七处仍须同批修订，漏任何一处都会留下悬空引用。

#### 八、连带处置

- 己-7：`T3-2` 与 `T3-4` 随本裁定回滚到「保留」一侧。两个标识符全仓只在己二表该行出现，具体落点须由改动方按其原工作清单核对——**不确定**。
- 裁定 F-06 不受影响：本裁定驳回门户突发上限一轴，未新增任何限流源；F-55 终态 21 类 kind 仍无任何限流或配额类。阶段 13 的原句按现行计划解释为「本阶段不因插件限流新增任何 `DegradationKind` 取值」。
- `13:1022`／`13:1065` 的插件运行时过载仍由第 4.8 节的燃料上限、内存上限、实例数上限与执行时限承担，plugin-host slice 只是恢复该行的四类静态取值，两套不并存为两个闸门；`13:1023`「配额触发限流一项」维持删除，但理由改为「该判据的 cgroup 侧被测量在首版取值集合下不存在」，不再声称因 R10 删表。
- `07:1057`「本阶段不定义任何 cgroup 配额与让路次序」是阶段作用域表述，与阶段 1 承载不冲突，**不改**。

### 己-3 的裁定　四端真机 PoC 首测的承接阶段

**裁定方向：承接阶段与批次均已冻结；薄首测前移，触发失败只替换移动 UI，Rust 核心不变。** 本节较早形成的二选一论证保留作追溯，现行选择以 F-08 第十七节为准。

#### 一、原定性推翻：这不是纯排期与资源决策

技术依赖链的两端已经把承接窗口夹死，「落哪个阶段」在技术上没有可选项。

1. **最早可测点**：被测物在阶段 1 不存在——`/clients/` 独立 Cargo workspace 与四端壳由 `13:1103` 新建；且规格附录 C.2 自身的两项判据在阶段 1 无可能成立——交互时延行引附录 A.1 十项常规交互清单，其中客户列表、客户详情、销售订单表单、库存可用量、审批任务列表、全文检索、附件列表、字段级受控只读视图分属阶段 3b、4、5、6、8；无障碍行 `spec:1925` 要求「四端各完成一次读屏软件端到端下单流程」，而全卷第一条真实的端到端下单链是 T0 的 MT0 判据。
2. **最晚有用发现点**：触发 Flutter 切栈的条件只有 `spec:1936` 逐字列出的五项（冷启动、列表滚动、交互时延、无障碍、中文输入），且全部限定在移动端；裁定 A-23 已把业务界面下沉到 `clients/mobile/src/modules/<module>/` 并在阶段 5 至 12 各写死一条移动用例退出条件，因此每晚一个业务阶段，切栈返工面就多一批模块目录。
3. **窗口只有一个点**：同时满足「壳已存在」「第一条端到端下单链已存在」「尚无任何业务阶段的移动界面」的批次，全卷只有阶段 13 第一批。

**结论：首测的承接阶段定为阶段 13a，不是阶段 1，也不是阶段 2。** 阶段 2 计划全文「四端」「客户端」「PoC」三词零命中，`00-overview.md` 第 1.3 节原先把它落在本卷阶段 1、2 是一处指向空集的事实性错误；`01-engineering-baseline.md` 第 1 节那句排除在技术上是对的。`spec:1905` 逐字「门槛表在阶段 1 启动前冻结」，冻结门槛表是纸面动作，与首测执行可分离，**门槛表冻结仍留本卷阶段 1**，改指阶段 13a 的只是首测执行。批次已按 F-08 第十七节冻结为薄首测前移。

#### 二、同批硬定的一条：移动壳最小切片前移，此事与 PoC 无关

`13:47` 把「移动端两端与其制品」整体排在阶段 11 之后，而裁定 A-23 为阶段 5 至 12 各写死一条逐字同形的退出条件（`05:781`、`06:790`、`07:1024`、`08:778`、`09:800`、`10:1219`、`11:768`、`12:769`），要求本模块移动界面通过 XCUITest 与 Espresso 用例；这八个阶段在固定链上没有一个排在阶段 11 之后，移动壳不存在时这八条退出条件**恒不可达**。方向由已生效裁定唯一确定，不构成取舍：A-23 逐字固定了那八条措辞，规格第 6.2 章的四端等价本就要求移动界面存在，故让步方是 `13:47`。

硬定：`/clients/mobile` 的移动壳本体与其 iOS、Android 生命周期与后台任务适配**不晚于阶段 5 退出条件的移动用例判定**可用——原判要求「排在阶段 5 全量开工之前」超出 A-23 的实际要求（A-23 只写死退出条件），卡在开工前是不必要地拉长 T0 前关键路径，按标准 4 放宽为退出前。四端制品、白标驱动与商店合规门禁仍留第二批。移动壳不属于第 1.5 节向 T0 贡献的五项，也不进入 T0 判据，`13:45`「该切片的判据只有一条」**一字不改**，本裁定不扩 T0。

#### 三、阶段 13 的风险缓解建立在不存在的前提上，后果有三条

（a）**判据性质必须拆开**：`13:39`、`13:938`、`13:978`、`13:1027` 四处原写「复测」。现按冻结路线拆为阶段 13a 的移动薄首测与阶段 13b/第二批的完整复测：薄首测只可触发切 Flutter，完整复测才可判保留 Tauri。

（b）**返工范围低估**：`13:1062` 处置列逐字抄自 `spec:545`，而该清单成文于 A-23 之前，不含被下沉的八个业务阶段的移动模块目录及其 XCUITest 与 Espresso 用例。本轮补入该项，不写具体目录数、不断言各模块界面的技术栈（全卷无逐字依据）。

（c）**控制列整格为空**：其两个分句同时悬空——阶段 1 明写不做 PoC；「冻结 Rust 核心接口语义」所指的客户端 crate 由阶段 13 自己新建，阶段 1 冻结的只是 `ep-foundation` 的服务端类型。

同类第四处是 `04-identity-authz.md:794`（原登记漏列），一并硬修：删去「阶段 1 的四端 PoC 若未覆盖」这个恒真悬空条件，改为无条件补测；USB Key 属桌面端外设，按附录 C.3 第四条不触发切栈。（a）（b）（c）与第二节的移动壳前移与批次二选一无关，两支下都成立，不因选支不同而回滚。

#### 四、「薄 PoC」这一支成立，但可判定面比原判小

成立的理由是范围收敛取自规格自身而非另定分档：`spec:1936` 已把可触发切栈的项枚举为五项，其余七项按附录 C.3 第三、四、五条一律不触发切栈，晚测不产生返工。可复用 T0 的**形态**——把一个做晚了会让前面全部白做的判定，前移到沉没成本接近零的那一点——但**不能复用其判据**：T0 判据只有一条，薄 PoC 不进入它；1 万行虚拟列表须用独立夹具构造，不得为此新增 `ep-datagen` 档位，也不得把 scale 数据集拖进 T0。

**本轮收窄两处（原判的两处过度声明作废）：**

- **无障碍项在阶段 13 第一批不可判。** `spec:1925` 逐字要求「四端各完成一次读屏软件端到端下单流程」，而该时点只有 `00-overview:97` 的桌面端一条下单链，按本裁定第一节该批次「尚无任何业务阶段的移动界面」，移动端读屏端到端下单流程**不存在**。原判「无障碍项因 MT0 恰好是一条真实的端到端下单链而可按 `spec:1925` 判定」不成立。故五项切栈触发项中**完整可判的只有冷启动、列表滚动、中文输入三项**，交互时延只能取样近似，无障碍不可判。
- **薄批只能产出否定结论。** `spec:1934`（附录 C.3 第一条）逐字「全部门槛项通过，或未通过项已获书面批准豁免时，客户端路线判定为 Tauri」。薄批只测五项，**只能产出切 Flutter 的否定结论，产不出判定为 Tauri 的肯定结论**；肯定结论须俟第二批全表通过，或未通过项获书面批准豁免。原判「据此判定客户端路线」只成立一半。

若选薄 PoC，其证据包必须显式标注交互时延的取样清单、声明无障碍项未判，并声明完整口径由第二批全表复测承担。

#### 五、批次选择已冻结：取选项一

| 选项 | 内容 | 代价 |
|---|---|---|
| **一　薄首测前移（已选）** | `spec:1936` 五项，只在 iOS 与 Android 两端，随阶段 13a 薄 PoC 测；据此可作出切 Flutter 的否定判定，判定为 Tauri 须俟第二批全表通过或书面批准豁免；其余七项留第二批全表复测 | 真机、企业签名身份与两款主流 MDM 是实施前置；交互时延只能取样近似，无障碍项在该时点不可判。阈值失败只替换移动 UI，客户端 Rust 核心九个 crate 不动 |
| ~~二　维持第二批全表首测~~ | ~~附录 C.2 十二项在阶段 13 第二批一次测全，据此判定路线~~ | ~~未选；保留作取舍追溯~~ |

选项一已经选定：阶段 1 只冻结门槛表与采购可行性，不做测量、不写客户端代码；阶段 13a 执行薄首测，第二批执行完整复测。四处证据描述按这一前后关系解释，不再存在未表态默认支。

#### 六、明确不做的

不给 PoC 建任何机检门禁（先例见附录戊四；PoC 是真机人工判定，判定人按规格固定为产品负责人，机器判不了）；不新建 PoC 专属阶段；不动 T0 判据与 `13:45` 的「固定为下列五项」；不动规格附录 C 的两次测量结构与判定人。代码侧零改动：84 个 crate、18 条 archcheck 规则、七条禁止项、33 个 xtask 测试一处不动，`/clients/` 在阶段 13 之前本就不存在。

## 附录庚　全部待决事项的合并索引

前面三个附录（丁、戊、己）各自登记过一批未裁事项，分散在三处。
本附录把它们合并成一张索引。F-50、F-51、F-52、F-53 生效后全部真实待决均已关闭；下表仅保留编号与关闭依据，开发不得再把历史“在等什么”当成阻断。
条目正文仍在原附录，本表只作关闭状态索引，不复述。

编号沿用原附录，不重编。合并索引不构成新裁定，唯一值以对应 F 类裁定与已回写阶段正文为准。

### 庚一　等使用方决定（0 条）

本节已清空。下表保留撤销线只作决策追溯，不再阻断开工。

| 编号 | 原登记 | 在等什么 | 谁能定 |
|---|---|---|---|
| ~~己-3~~ **已获表态，见 F-08 第十七节** | 附录己二「己-3 的裁定」一节 | ~~批次二选一~~ **已选薄首测前移；失败只替换移动 UI 为 Flutter，Rust 核心不变** | ~~排期决策方~~ 已决 |
| ~~F-08-1~~ **已获表态，本行撤销，见 F-08 第十三节** | 裁定 F-08 第十一节第 1 条 | ~~**国产化替代路径与等级保护三级对外表述**。服务端改 Windows Server 后，规格第 2.2、17.5 章登记的国产化认证矩阵在服务端一侧失去可达路径：国产 Linux 不再是「延期项」，而是与首版服务端平台互斥。规格 `1372`、`1534`、`1958` 与 PRD `4417` 四处的国产 Linux 表述**本轮原样挂起，不得顺手删除**——删掉等于悄悄取消一项对客户的能力承诺。三选一：改写为互斥、保留为长期项、另立国产化服务端分支。~~ **本轮已由使用方裁定为「保留为长期项 + 零 Linux 开发」，落 F-08 第十三节** | ~~产品负责人~~ 已决 |
| ~~F-08-2~~ **已获表态，见 F-08 第十七节** | 裁定 F-08 第十一节第 2 条 | ~~Hyper-V 二选一~~ **已取 Windows 原生，产品服务、数据库与客户主数据卷不进入 Hyper-V；2026-08-22 F-55 仅追加逐次 MCP 插件短命 Hyper-V-isolated Windows utility VM 窄例外** | ~~使用方~~ 已决 |
| ~~F-08-3~~ **已获表态，见 F-08 第十七节** | 裁定 F-08 第十一节第 3 条 | ~~是否需要 Authenticode~~ **生产必须 Authenticode；开发可内部 ECDSA；证书由厂商或客户提供** | ~~使用方~~ 已决 |

**己-7** 不入本表：它没有独立内容，纯粹随己-1 的方向回滚或保留。

### 庚二　原等被测输入或落码结果存在（0 条）

本节已清空。F-50 关闭己-4、己-6，F-51 关闭 D-01、戊-11，F-52 关闭 D-02、D-03、X-3、X-1、X-2，F-53 关闭阶段 14 历史迁移、补丁分发、支持套餐与病毒扫描部署缺口；其余撤销行此前已关闭。下表全部只作追溯。

| 编号 | 原登记 | 在等什么 |
|---|---|---|
| ~~D-01~~ **已裁定，见 F-51 同批技术冻结** | 附录丁一 | BlindIndex 固定完整 32 字节；`derive_blind_key`、列、测试与跨法人派生同宽，不再等待落码时选择 |
| ~~D-02~~ **已关闭，见 F-52** | 附录丁一 | 九套自动测试的 SPI、属主、适用集合、事务与失败语义已冻结 |
| ~~D-03~~ **已关闭，见 F-52** | 附录丁一 | `config_packages` 直接承载耐久任务，不新增事件，job-worker 租约领取已冻结 |
| ~~己-4~~ **已关闭，见 F-50** | 附录己二 | `SupplierInvoiceUploadWritebackPort` 归 `ep-contract-portal`，由 portal 实现、invoice 调用 |
| ~~己-6~~ **已关闭，见 F-50** | 附录己二 | 退回端点与 `reason,row_version` 请求已冻结 |
| ~~X-3~~ **已关闭，见 F-52** | 附录己二 | 两工具自阶段 1 为非产品骨架，阶段 14 交付前固定返回 70；真实/负向 SBOM 判据已冻结 |
| ~~X-4~~ **已裁定** | 附录己二 | `ep_quota_throttled_total` 已撤销，指标目录删除该行；代码侧注册表与填充点须在首批实施中同步删除，`cargo xtask configdoc` 未真实通过前不得宣称登记一致 |
| ~~戊-11~~ **已裁定，见 F-51 同批技术冻结** | 附录戊二 | `SecurityContext` 增加第 20 字段 `system_purpose`；`Reconciliation` 仅由 recon executor 构造并以 `reconciliation-context-confined` 机检，不再等待阶段 2 另定类型或入口 |
| ~~X-1~~ **已关闭，见 F-52** | 附录己二 | 复用 30 秒采样器，三态与 streak 落 `archive_channel`，第十九个 kind 承载连续 `NO_RESULT`；F-53 后总数为 20 |
| ~~F-08-4~~ **已裁定，见 F-08 第十七节** | 裁定 F-08 第十一节第 4 条 | 默认 Forgejo 加 Woodpecker Windows agent，`cargo xtask ci` 为唯一入口；实机运行属于首批实施证据，不再决定平台取值 |
| ~~X-2~~ **已关闭，见 F-52** | 附录己二 | 独立 `writer-role-containment` 为 Blocking；`offsite-sink-requirements` 保持 Degrading |

### 庚三　全部已裁定（含 F-50、F-51、F-52、F-53）

己-2 与戊-12 已裁，归属 F-06 与 F-07，正文见 F 类节。
服务端改 Windows Server 原生一事已裁，归属 F-08，正文见 F 类节；
该裁定原带的四条新待决事项现已全部闭合；庚一相关行与庚二 F-08-4 行仅保留撤销追溯。
庚五现有十七项有效首批实施验证门禁与证据清单；原编号 12 的 ICU 建库二选一已被 C 字节序冻结取代并撤销。这些门禁不再归类为设计待决。
D-01 与戊-11 已由 F-51 同批技术冻结关闭：前者固定完整 32 字节，后者固定
`SecurityContext.system_purpose`、四参 `system` 构造与 recon executor 唯一构造边界；两者不再计入庚二。
己-4 与己-6 已由 F-50 关闭。D-02、D-03、X-3、X-1、X-2 已由 F-52 关闭，自动测试、耐久派发、工具生命周期、周期核对与写入角色启动检查均有唯一实现口径。庚一与庚二真实未决合计为零。

### 庚四　这张索引的维护纪律

新增待决事项一律先进原附录，再在本表加一行；本表只增不改正文。
每条必须写明「在等什么」，且该条件要么是使用方的一次表态，
要么是仓库里可观测的事实（某文件出现、某 crate 有实质内容）。
**写不出等待条件的，不是待决事项，是没查够**——按第 12 节通则第六条的三档处置。

目标平台实测现作为实施证据条件，不再作为设计待决的第三种类型。门槛仍同样严格：必须写明测什么、机器版本、原始输出、结论与预先冻结的失败处置；缺一即未通过。

### 庚五　Windows 首批实施验证门禁（17 项有效，见裁定 F-08 第十二节）

本节不复述条目，只给索引与共同执行条件。十七项有效门禁逐条列在裁定 F-08 第十二节；为保持历史引用，该节保留已撤销的原编号 12，其余不重排，第 15 至 18 项仍是补裁壬追加的四项。有效门禁不阻止按冻结设计开始开发，但在证据形成前阻止对应能力被标为“已覆盖”或“已通过”。

执行环境固定为 **Windows Server 2022**（认证冻结点），并在 Windows Server 2019 上做同项复核（区间下沿）。两版结论不一致时，对应门禁保持非零，并按 F-08 已列的保守失败支路处理；不得自行切换 Linux、整机 Hyper-V 或另一套 CI/核心。F-55 §4.5 的可选插件 utility VM 只按其自身 Hyper-V gate 判定。

下列三项直接决定对应实现支路是否可启用；证据出具前一律使用已冻结的保守状态，不把它们写成设计待定：

| 项 | 决定什么 |
|---|---|
| 第 2 项　Job Object 最小／最大速率模式的实际行为 | 未验证前 CPU 一列只作意图声明；验证通过后才可逐行启用并恢复判据面 |
| 第 3 项　IO 速率控制在本地 NTFS 直连卷上的覆盖面 | backup-writer 的 `IOMax` 能否保留 |
| 第 5 项　服务未报告 `SERVICE_STOPPED` 即退出时是否被判为崩溃 | 退出码 78／70 的分流走主承载还是降级备选 |

第 18 项问的是「该不该设」而不是「能不能设」；不成立时主动放弃该配额并如实宣告无承载。其余有效项均可先实现测试夹具和主承载，但在真实执行前对应 CI／发布门禁必须保持非零。第 8 项约束 PE 可复现构建；原第 12 项已撤销，不得再以 ICU 建库证据要求重新激活。
