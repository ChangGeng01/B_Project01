## 本文件的地位

本文件自本次修订起降级为登记表，不构成权威。全卷权威链固定为四层：规格、PRD、技术基线、各阶段计划。本表与这四层冲突时一律以这四层为准，任何文件不得以本表为唯一出处，各阶段计划不得以本表编号代替正文定义。各阶段计划中的按裁定 X-nn 一律只是决策出处标注，取值以该计划与基线正文为准。

本表只承担三件事：登记 67 条缺口的最终归属、登记其确切标识符、登记作废名清单。各条目中只有结论、最终归属阶段、确切标识符三段属长期有效内容。各条目中的提供方要做什么、每个使用方要改什么、顺序约束三段，以及一切原方案作废、该措辞作废、其后编号顺延、总览第 N 节须改写一类句式，都是 2026-08-10 四次回写提交的施工期工单，回写已执行完毕，一律作废：不得引用、不得据以施工、不得据以判定评审阻塞，下一次修订本文件时逐条删除。文末的回写清单整节同此处理。

本表中一切以 Noop 为前缀的类型名、一切先注入空实现后反向替换的措辞、一切验收顺延的措辞，按下列通则第三条一律作废。不得新增第二张裁定表，不得让计划正文以裁定表为唯一出处，此二者列入评审清单。

## 五条通则

以下五条对全部 67 条登记项生效，各阶段不得再解释。通则属技术基线层，本登记表不得凌驾其上。

第一，权威顺序为规格、PRD、技术基线、阶段计划，本表在权威链之外。

第二，模块归属的唯一判据是基线第 1.2 节的 15 个模块码覆盖范围与基线第 1.3 节最后一条“禁止跨模块直接读写业务表，一个仓储只访问自己模块的 schema”。表落在哪个 schema，该 schema 对应的模块所在阶段就是该表的所有者，不存在“甲阶段在乙模块的 schema 里建表”这一形态。

第三，跨模块同步调用的被调方必须与调用方同批交付。被调方阶段晚于调用方阶段的，调用方本轮不做该调用，承载该调用的用例整体推到被调方所在批次；不得先注入空实现再回头替换，不得把验收顺延到被调方阶段。apps/core-server/src/wiring/ 与 apps/job-worker/src/wiring/ 两个目录下的全部文件中不得出现任何以 Noop、Stub、Fake、Dummy 为前缀的实现类型或注入行，由阶段 1 交付的 xtask archcheck 规则 unwired-absent 断言，出现即构建失败，该规则配一个故意违反的负样例，Unwired 一名撤销。唯一例外是规格把交付时点冻结在末期的三项平台能力，即 WasmComputePort、RuleEvaluator 与 DisposalPort，三者及其宿主进程 plugin-host 与承载 crate 一律保留：三者在其交付阶段之前不注入任何实现，改由 platform_ops.degradation_windows 承载，取值一律为阶段 2 定义的 DegradationKind 的 PORT_NOT_IMPLEMENTED 并由 subject 列记下该端口名，WASM_COMPUTE_NOT_DELIVERED、RULE_EVALUATOR_NOT_DELIVERED 与 DISPOSAL_NOT_DELIVERED 三个取值撤销，能力缺位时开一个降级窗口，界面与健康端点显式呈现该能力未交付，指标 ep_degradation_windows_open 自动计数；三者在能力缺位时返回可重试错误或直接拒绝，不得静默按成功路径放行，也不得以不注册路由返回 404 的形态替代该降级窗口。本条的完整裁定见总览第 1.5 节第六条至第八条与第十一条，本表只作登记。

第四，阶段顺序固定为：1 → 2 → 3a → 4 → T0 → 5 → 9a → 8 → 6 → 7 → 10 → 11 → 9b → 14，阶段 12 在阶段 10 之后与阶段 11 并行，阶段 13 在阶段 12 之后与阶段 9b 并行。T0 是插在阶段 4 与阶段 5 之间的最薄贯通线，不新增任何范围，只从阶段 5、6、9a、10、11 各取一个最小切片，其体量是这五个阶段各自最小子集的当量之和，定义见总览第 2 节总表 T0 行与第 5 节 MT0 行。本表全部顺序约束都以这条链为基准。

第五，db/migrations/order.toml 撤销。迁移文件放在其主要创建对象所属 schema 的目录下，二十四个目录保留；执行顺序由单一全局 Runner 按文件版本号排序，历史表合并为 platform_core.schema_history 一张，命名 V<YYYYMMDDHHMMSS> 且版本号取真实时间、全局唯一、严格递增，由 xtask sqlcheck 断言，正确性由空库全量执行在 CI 中验证。其后编号顺延这一整类连锁改动取消。

## 作废名清单

本清单是本表唯一长期有效的差分产物，作用是防止旧名被捡回来。任何阶段不得使用左列名字。

| 作废名 | 取代它的名字或去向 | 出处 |
|---|---|---|
| 全部以 Noop 为前缀的实现类型 | 撤销，跨模块调用同批交付或整只推迟；三项末期能力改由降级窗口承载 | 通则第三 |
| db/migrations/order.toml 与二十四项位次序 | 撤销，单一全局 Runner 按版本号排序 | 通则第五 |
| <schema>.refinery_schema_history 二十四张 | platform_core.schema_history 一张 | 通则第五 |
| transact_repeatable_read | UnitOfWork::snapshot_transact | C-03 |
| ep_db_retries_total、ep_tx_retry_total | ep_db_tx_retries_total | C-21 |
| ep_db_replication_crosscheck_age_seconds、ep_replication_crosscheck_age_seconds | 两者一并撤销，交叉核对折叠进保留量周期采样器 | C-22 |
| WRITER_ROLE_CONTAINMENT_MISSING | WRITER_NOT_IN_SERVICE，判据改为写出进程未运行或连续无上报 | A-26 |
| REPLICATION_CROSSCHECK_NO_RESULT | 撤销 | C-22 |
| ep_quota_throttled_total 与 platform_ops.quota_events | 撤销，按应用层限流与超时计入附录 A.2 错误率口径 | 基线第 2 节 |
| cgroup-quota-matched、license-and-modules-consistent、current-period-open | 三个自检项撤销 | C-25 |
| duty-class-exclusivity、forbidden-permission-items-absent、master-data-usage-probes-registered、client-capability-matrix-frozen | 四个自检项撤销，下沉为写入侧约束、模块启用前置校验或内置快照为准 | C-25 |
| inventory.stock_value_adjusted.v1 | inventory.stock_movement.value_adjusted.v1 | B-09 |
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
| procure.goods_receipt_line_costings 的单价列 | InventoryPricingLookupPort::original_unit_price_by_source_line | C-12 |
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

结论：按阶段 4 计划第 4.1 节的结构体加两个追踪字段一次性冻结，共 19 个字段，构造入口只有两个。

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

配套枚举同在 ep-foundation 冻结：`AccountKind { Human, System, Portal }`；`ClientKind { Win, Mac, Ios, Android, Portal, Ops }`，序列化取值与基线第 5.6 节 X-Client 头一一对应；`DepartmentScope { All, Subtree(Id<Department>), Explicit(Arc<[Id<Department>]>) }`。构造函数只有 `SecurityContext::human(..)` 与 `SecurityContext::system(legal_entity_id, request_id, trace_id)` 两个，后者用 A-02 的两个常量填 user_id 与 device_id，account_kind 取 System。不提供任何 with_ 前缀的变换方法。

第 18 与第 19 两个字段是本裁定的追加项，理由是基线第 3.8 节要求连接取用时写入 `app.request_id` 与 `app.trace_id` 两条会话变量，取数只能来自安全上下文。

提供方要做什么：阶段 1 在 ep-foundation 实现该结构体、三个配套枚举与基线第 1.4 节冻结的七个字段类型即 `DeviceId`、`RoleCode`、`DutyClass`、`RecordShare` 与 `RecordShareGrant`、`DataScopeTag`、`RequestId`、`TraceId`，写入阶段 1 计划第 5.1 节，替换现有的八字段简表；阶段 1 计划第 514 行退出条件第 21 条的验收面由“19 个字段与三个配套枚举”改为“19 个字段、三个配套枚举与七个字段类型”。七个类型的名字、形态与取值域以基线第 1.4 节为唯一出处，本表不复述。

每个使用方要改什么。阶段 4 计划第 4.1 节删去“由阶段 1 提供其骨架，本阶段补齐字段集合，见 needs”，改为“字段集合由阶段 1 按 A-03 冻结，本阶段只负责填充”。阶段 4 计划第 811 行的阻塞判定删除。阶段 5、6、11 凡引用 SecurityContext 字段的措辞一律按上表字段名书写。

`RecordShare` 的形状受 PRD 附录乙 U-B-07 记录级权限授予方式影响，该项待决，本表不代拍，按显式共享一条记录冻结为临时取值，决策人为产品负责人，截止点按总览 R12 的 M7。切换代价：改判为按责任人、按创建人或按流程当前处理人的，只增加阶段 4 `ScopeCompiler` 的谓词分支，不改本结构体；改判为共享可再转授的，在 `RecordShareGrant` 上增加变体，属加变体不改字段，旧取值由 serde 未知取值反序列化失败兜住。

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

结论：本体归阶段 9a，注册方固定为阶段 7、8、9b、10、11 五个，在其之后或按反向依赖接入。阶段 14 只调用 `ReconExecutor::run`，不注册任何 `ReconCheck`；阶段 13 全文没有跨模块逻辑引用，不实现也不注册 `ReconCheck`，从注册方清单中删除。原裁定所称的六个注册方作废，本条是该清单的唯一出处，其他文件一律引用不复述。总览 R14 不得再设与本条并列的注册义务，阶段 5、6、8、12 的跨模块逻辑引用不进入本清单，其写入时存在性校验由各阶段 application 层经对方模块契约承担。阶段 3b 的附件孤儿收敛任务不算对账，改写措辞，不使用该框架。

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

每个使用方要改什么。阶段 7 计划第 942 行与退出条件第 9 条保留登记语句的措辞，删去“既有的 REPEATABLE READ 快照”一语中的既有二字，改为“由 ep-platform-recon 提供的快照”。阶段 8 计划 D7 与退出条件第 12 条改为实现 `ReconCheck` 并在 wiring 注册。阶段 9 计划第 384 行删去“由 recon 提供”的转述，改为本阶段提供。阶段 11 计划第 60 与 577 行改为实现三个 `ReconCheck`。阶段 10 计划第 120 行的“注册对账取数语句集”改为实现一个 `ReconCheck` 并在 wiring 注册，其 `check_code` 固定为 `FIN_CROSS_MODULE_LINK`、`category` 取 `CROSS_MODULE_LINK`，一次覆盖本模块全部跨模块逻辑引用，不按引用逐条建校验项。阶段 9 计划第 20、44、942 三行的注册方名单一律改为阶段 7、8、9b、10、11，删去其中的阶段 6、13、14 与“均早于 9b”一语，9b 的四个校验项在本阶段内注册。五个注册方的校验项数固定为阶段 7 六个、阶段 8 两个、阶段 9b 四个、阶段 10 一个、阶段 11 三个，合计十六个，全部在 `apps/job-worker/src/wiring.rs` 中经 `ReconRegistry::register` 注册。阶段 13 计划不出现 `ReconCheck`、`ReconRegistry` 与 ep-platform-recon，本条对阶段 13 无落点；阶段 14 计划只保留 `ReconExecutor::run(ReconRunKind::RecoveryAcceptance)` 的调用，不出现注册措辞。阶段 3 计划第 3.0 节判定三与第 3.9 节把附件孤儿收敛改称“job-worker 内的幂等收敛任务”，明确不产生对账差异事项、不依赖 ep-platform-recon。

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

结论：交付确认单归 sales 模块，因此归阶段 6，不归阶段 8。总览第 4.1 节把表建在 sales schema 却让阶段 8 的 ep-app-inventory 去写，直接违反基线第 1.3 节最后一条“一个仓储只访问自己模块的 schema”，且 sales.delivery_schedules 与 sales.return_line_delivery_links 两张已在阶段 6 的表都需要指向交付确认单的同 schema 真实外键，按基线第 3.3 节这两处外键不能是逻辑引用。阶段 6 在调整后的顺序中排在阶段 8 与阶段 9a 之后，库存腿与凭证腿的端口在阶段 6 开工时均已存在，不构成倒挂。交付确认的功能定义出自规格第 8 章第 8 步与第 5.2 章事件-分录表，规格已把确认时点、直运分支、收入确认与销货成本结转、应收账款未开票过渡科目四项写死；PRD 附录乙 U-C-01 问的是该功能由 PRD 第 3 节还是第 5 节承载，属 PRD 内部的编排问题，不是技术取值待决项，按权威顺序规格高于 PRD，本条据规格直接落地，不受 U-C-01 阻塞。U-C-01 的承载节由产品负责人决定，本表不代拍，阶段 6 计划在未决事项表中登记该条及其切换代价，即改判由 PRD 第 5 节承载时两张表跨 schema 迁移、两处真实外键退回逻辑引用、事件 aggregate_type 与 payload 改名、类型码 DC 改登记模块、阶段 8 与 9a 与 10 三条腿调用方反转，属高代价。同批仍属 PRD 待决且本条不代拍的是 U-C-02 交付确认的操作者角色，阶段 6 只冻结能力常量不预置角色绑定。

最终归属阶段：阶段 6 建表、建用例、发事件；阶段 8 提供库存腿端口；阶段 9a 提供收入与成本腿端口；阶段 10 提供过渡科目腿端口并反向替换阶段 6 的空实现。

确切标识符。两张表落在 sales schema。

`sales.delivery_confirmations`，单据类，类型码 DC。列除公共列外为：`doc_no text not null`；`status text not null CHECK in ('DRAFT','CONFIRMED')`；`customer_id uuid not null`；`sales_order_id uuid not null`（同 schema 外键）；`posting_date date not null`；`warehouse_id uuid`；`is_drop_ship boolean not null default false`；`confirmed_at timestamptz`；`confirmed_by uuid`；`voucher_id uuid`（逻辑引用 ledger，确认时回填）；`remark text`。约束 `ux_delivery_confirmations_legal_entity_id_doc_no`；索引 `ix_delivery_confirmations_legal_entity_id_created_at`、`ix_delivery_confirmations_sales_order_id`、`ix_delivery_confirmations_legal_entity_id_posting_date`。不设作废态，冲正一律经销售退货单，理由是基线第 3.6 节禁止软删除且已过账分录只追加。本表不带 accounting_period_id，与阶段 6 计划第 781 行的偏离登记一致。

`sales.delivery_confirmation_lines`。列除公共列外为：`delivery_confirmation_id uuid not null`（同 schema 外键）；`line_no int not null`；`sales_order_line_id uuid not null`（同 schema 外键）；`delivery_schedule_id uuid not null`（同 schema 外键）；`item_kind text`；`item_id uuid not null`；`uom_code text not null`；`quantity numeric(18,6) not null`；`net_unit_price numeric(18,6) not null`；`tax_rate numeric(9,6) not null default 0`；`line_amount numeric(18,2) not null`；`line_amount_with_tax numeric(18,2) not null`；`warehouse_id uuid`；`batch_no text not null default '-'`；`serial_nos text[] not null default '{}'`；`cogs_amount numeric(18,2)`（确认时由库存腿回填）；`stock_movement_id uuid`（逻辑引用 inventory）。约束 `ux_delivery_confirmation_lines_confirmation_id_line_no`；索引 `ix_delivery_confirmation_lines_sales_order_line_id`。

用例名与端点：`crates/application/sales/src/usecase/create_delivery_confirmation.rs` 对应 `POST /api/v1/sales/delivery-confirmations`；`crates/application/sales/src/usecase/confirm_delivery.rs` 对应 `POST /api/v1/sales/delivery-confirmations/{id}/actions/confirm-delivery`；另有 `GET /api/v1/sales/delivery-confirmations` 与 `GET /api/v1/sales/delivery-confirmations/{id}`。

事件名：`sales.delivery.confirmed.v1`，aggregate_type 取 `sales.delivery_confirmations`，与基线第 6.1 节示例逐字一致。payload 字段固定为：`delivery_confirmation_id`、`doc_no`、`sales_order_id`、`customer_id`、`contract_id`、`is_drop_ship`、`voucher_id`、`lines`，其中 `lines` 每元素含 `delivery_confirmation_line_id`、`sales_order_line_id`、`delivery_schedule_id`、`item_kind`、`item_id`、`quantity`、`warehouse_id`、`batch_no`、`serial_nos`、`revenue_amount`、`cogs_amount`。信封的 `posting_date` 取单据 posting_date，`accounting_period_id` 取 PostingPort 返回值。

三腿的实现方与调用形态，全部在 confirm_delivery 的同一个事务内，次序固定为库存腿、过渡科目腿、凭证腿。

| 腿 | 实现阶段 | 调用 |
|---|---|---|
| 库存腿 | 阶段 8 | `ep_contract_inventory::InventoryPostingPort::post_outbound(tx, ctx, OutboundPosting { reason: MovementReason::DeliveryConfirmation, pricing: OutboundPricing::MovingAverage, source: SourceRef{ doc_type: DELIVERY_CONFIRMATION, .. }, lines })`，返回每行 cogs_amount 与 stock_movement_id；`is_drop_ship` 为真时整段跳过 |
| 过渡科目腿 | 阶段 10 | `ep_contract_finance::UnbilledArPort::record_on_delivery(tx, ctx, DeliveryUnbilledArCommand { delivery_confirmation_id, customer_id, posting_date, accounting_period_id, direction: DEBIT, net_amount })`，写 finance.unbilled_ar_entries |
| 收入与成本腿 | 阶段 9a | `ep_contract_ledger::PostingPort::post(tx, ctx, PostingInput { source_kind: VoucherSourceKind::DELIVERY_CONFIRMED, branch: DROP_SHIP 或 NON_DROP_SHIP, posting_date, source_document, measures })`，measures 含 revenue_amount、unbilled_receivable_amount、cogs_amount、inventory_release_amount 四项 |

会计期间由 `ep_contract_ledger::AccountingPeriodResolver::resolve` 在事务最前解析一次，库存腿与过渡科目腿复用其返回值。

提供方要做什么：阶段 6 在 `db/migrations/sales/` 追加两个迁移文件，slug 为 `sales_create_delivery_confirmations` 与 `sales_create_delivery_confirmation_lines`，排在 `sales.delivery_schedules` 之后、`sales.return_line_delivery_links` 之前；把 `sales.return_line_delivery_links` 的 `delivery_confirmation_id` 与 `delivery_confirmation_line_id` 由逻辑引用改为同 schema 真实外键 ON DELETE RESTRICT；`sales.delivery_schedules.delivery_confirmation_id` 与 `clm.contract_milestones.delivery_confirmation_id` 前者改为真实外键、后者保持逻辑引用（跨 schema）。写入阶段 6 计划第 3 节数据库变更、第 4 节算法、第 5 节 API 契约、第 8 节测试与第 9 节退出条件。

每个使用方要改什么。阶段 6 计划第 61 行的“交付确认回写”消费者保留，消费者名固定为 `sales.delivery_writeback`，消费自身发出的 `sales.delivery.confirmed.v1`。阶段 6 计划第 772 行的风险条整条删除。阶段 8 计划第 0 节与第 10.1 节保留“交付确认事件的库存侧算法”，删去任何暗示本阶段建单据的措辞，并在第 11.3 节明确 `SourceDocType::DELIVERY_CONFIRMATION` 由 sales 传入。阶段 10 计划第 815 行的 UnbilledArPort 使用方由“ep-app-sales、ep-app-inventory”收窄为“ep-app-sales”，并新增一条说明：阶段 6 先注入 `NoopUnbilledArPort`，阶段 10 替换。阶段 11 的成本下钻按 `sales.delivery_confirmation_lines` 取数，经 `costing.cost_entries` 的 `source_document_id` 与 `source_document_line_id` 跳转原单据；交付指标不直接读该基表，实际交付日期经受治理数据集 `clm_contract_delivery_milestones` 与 `sales_order_delivery_batches` 上的交付确认引用与确认日期列取得，理由是阶段 11 的 D-11-01 禁止分析 SQL 出现来源模块基表名，且 A-18 未为交付确认单登记数据集。阶段 12 计划第 204 行的 `delivery_confirmation_id` 与 `delivery_confirmation_line_id` 保持逻辑引用并注明来源表为 `sales.delivery_confirmations`。

顺序约束：阶段 6 排在阶段 8 与阶段 9a 之后，即 5 → 9a → 8 → 6，本条不产生倒挂。唯一的反向依赖是过渡科目腿由阶段 10 回头替换，阶段 6 的 E2E-6-09 与 E2E-6-10 中过渡科目净额的断言顺延到 M7。

### A-10 进项发票台账与采购发票登记用例

结论：进项发票台账归 invoice 模块，因此归阶段 10，与基线第 1.2 节“invoice 覆盖销项与进项发票台账”一致。阶段 7 不建表、不写台账。

最终归属阶段：阶段 10。

确切标识符。两张表落在 invoice schema。

`invoice.purchase_invoices`，单据类，类型码 PINV。列除公共列外为：`doc_no text not null`；`status text not null CHECK in ('REGISTERED','REVERSED')`；`supplier_id uuid not null`；`purchase_order_id uuid`（逻辑引用 procure）；`invoice_no text not null`（供应商发票号）；`invoice_date date not null`；`posting_date date not null`；`accounting_period_id uuid not null`；`deferred_from_period_id uuid`；`tax_rate numeric(9,6) not null`；`net_amount numeric(18,2) not null`；`tax_amount numeric(18,2) not null`；`gross_amount numeric(18,2) not null`；`cost_kind text not null CHECK in ('INVENTORY_TYPE','DIRECT_EXPENSE_TYPE')`；`is_credit_note boolean not null default false`；`reversed_by_id uuid`；`voucher_id uuid`。约束 `ux_purchase_invoices_legal_entity_id_doc_no`、`ux_purchase_invoices_legal_entity_id_supplier_id_invoice_no`；索引 `ix_purchase_invoices_legal_entity_id_created_at`、`ix_purchase_invoices_legal_entity_id_purchase_order_id`、`ix_purchase_invoices_legal_entity_id_posting_date`。

`invoice.purchase_invoice_lines`。列除公共列外为：`purchase_invoice_id uuid not null`（同 schema 外键）；`line_no int not null`；`purchase_order_line_id uuid`（逻辑引用 procure）；`goods_receipt_line_id uuid`（逻辑引用 procure）；`material_id uuid`；`quantity numeric(18,6) not null`；`net_unit_price numeric(18,6) not null`；`net_amount numeric(18,2) not null`；`tax_amount numeric(18,2) not null`；`accrual_reversal_amount numeric(18,2)`；`price_variance_amount numeric(18,2)`；`is_overbilling boolean not null default false`。约束 `ux_purchase_invoice_lines_invoice_id_line_no`；索引 `ix_purchase_invoice_lines_legal_entity_id_goods_receipt_line_id`。

用例名：`crates/application/invoice/src/usecase/register_purchase_invoice.rs`，端点 `POST /api/v1/invoice/purchase-invoices`，另有 `GET /api/v1/invoice/purchase-invoices` 与 `/{id}`。三单匹配在该用例内执行，依次比对采购订单行、收货行与本次发票行的数量与金额。暂估回冲与价差拆分经 `ep_contract_inventory::InventoryVariancePort::split_variance(tx, ctx, VarianceSplitCommand{..})` 取得尚有库存部分与已出库部分的金额，本阶段不自行取价。应付腿经本模块自身的 `register_payable_on_purchase_invoice` 用例写入。

事件名：`invoice.purchase_invoice.registered.v1`，aggregate_type 取 `invoice.purchase_invoices`，payload 含 `purchase_invoice_id`、`doc_no`、`supplier_id`、`purchase_order_id`、`cost_kind`、`net_amount`、`tax_amount`、`gross_amount`、`accrual_reversal_amount`、`price_variance_in_stock_amount`、`price_variance_released_amount`、`voucher_id`、`lines`。原有的 `invoice.purchase_invoice.reversed.v1` 保持不变。

提供方要做什么：阶段 10 在 `db/migrations/invoice/` 追加两个迁移文件，slug 为 `invoice_create_purchase_invoices` 与 `invoice_create_purchase_invoice_lines`，排在 `invoice.invoice_reversals` 之后；在 ep-platform-sequence 追加类型码 PINV；在事件目录与错误码表登记增量。写入阶段 10 计划第 3.1 节 invoice schema 表定义、第 4 节算法、第 5 节 API 契约、第 6 节并发场景与第 9 节退出条件。阶段 10 计划第 725 行的只读投影端点由“取数经 ep-contract-procure”改为“取数为本模块自有表”。

每个使用方要改什么。阶段 7 计划第 3 行“本阶段不实现采购发票登记……只按契约衔接”保留并加一句“进项发票台账两张表由阶段 10 在 invoice schema 建立”。阶段 7 计划第 244 行 `source_purchase_invoice_line_id` 与第 278 行 `purchase_invoice_line_id` 的逻辑引用目标写死为 `invoice.purchase_invoice_lines`。阶段 7 计划第 374 行 `accepted_purchase_invoice_id` 的目标写死为 `invoice.purchase_invoices`。阶段 8 的价差拆分入口保持 `InventoryVariancePort`，调用方由“采购或发票模块”收窄为 `ep-app-invoice`。阶段 11 计划第 3.5 节数据集 `procure_purchase_invoices` 的 source_view 由 `procure.v_purchase_invoices_dataset` 改为 `invoice.v_purchase_invoices_dataset`，dataset code 改为 `invoice_purchase_invoices`，提供方由采购阶段改为阶段 10，见 A-18。

顺序约束：阶段 7 在阶段 10 之前，阶段 7 对进项发票的两处查询按通则第三条注入空实现，由阶段 10 替换，阶段 7 的相应验收顺延到 M7。

### A-11 进项红字发票登记端口与收货发票匹配查询端口

结论：与 A-10 同批交付，两个端口都落在 ep-contract-invoice。

最终归属阶段：阶段 10。

确切标识符。

```rust
// crates/contract/invoice/src/port/purchase.rs
pub struct ReceiptInvoiceMatchState {
    pub goods_receipt_line_id: Id<GoodsReceiptLine>,
    pub is_invoice_registered: bool,
    pub matched_quantity: Quantity,
    pub matched_net_amount: Money,
    pub purchase_invoice_line_ids: Vec<Id<PurchaseInvoiceLine>>,
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

pub struct RegisterPurchaseCreditNote {
    pub supplier_id: Id<Supplier>,
    pub original_purchase_invoice_id: Id<PurchaseInvoice>,
    pub purchase_return_id: uuid::Uuid,
    pub posting_date: chrono::NaiveDate,
    pub lines: Vec<PurchaseCreditNoteLine>,
    pub is_for_overbilling_settlement: bool,
}
pub struct PurchaseCreditNoteLine {
    pub original_purchase_invoice_line_id: Id<PurchaseInvoiceLine>,
    pub goods_receipt_line_id: Option<Id<GoodsReceiptLine>>,
    pub quantity: Quantity,
    pub net_amount: Money,
    pub tax_amount: Money,
}
pub struct PurchaseCreditNoteView {
    pub purchase_invoice_id: Id<PurchaseInvoice>,
    pub doc_no: String,
    pub net_amount: Money,
    pub tax_amount: Money,
    pub gross_amount: Money,
    pub voucher_id: Option<uuid::Uuid>,
}

#[async_trait::async_trait]
pub trait PurchaseCreditNotePort: Send + Sync {
    async fn register_credit_note(&self, tx: &mut dyn Tx, ctx: &SecurityContext,
                                  cmd: RegisterPurchaseCreditNote)
        -> Result<PurchaseCreditNoteView, AppError>;
}
```

提供方要做什么：阶段 10 在 ep-contract-invoice 定义两个 trait 与四个 DTO，在 ep-app-invoice 实现，在两个 wiring.rs 注入。写入阶段 10 计划第 7 节模块内契约表，追加两行。

每个使用方要改什么。阶段 7 计划第 553 行的调用保持 `ReceiptInvoiceMatchQueryPort::match_state`，签名按上表补全参数。阶段 7 计划第 1096 行的假设 A3 改为“采购退货在采购发票已登记分支下调用 `PurchaseCreditNotePort::register_credit_note`，红字发票由 invoice 模块登记”，并删去“采购侧只提供退货数量、批次、关联收货行与退货日期”之后关于字段表的推测。阶段 7 在 wiring 注入 `NoopPurchaseCreditNotePort` 与 `NoopReceiptInvoiceMatchQueryPort`，阶段 10 替换。

顺序约束：与 A-10 相同，阶段 7 早于阶段 10，按通则第三条处理。

### A-12 ep-contract-inventory::AvailabilityQueryPort

结论：补 trait，与 C-18 合并为同一个 trait 的两个方法。

最终归属阶段：阶段 8。

确切标识符。

```rust
// crates/contract/inventory/src/port/availability.rs
pub struct AvailabilityQuery {
    pub legal_entity_id: Id<LegalEntity>,
    pub material_id: Id<Material>,
    pub warehouse_id: Option<Id<Warehouse>>,
    pub required_on: chrono::NaiveDate,
}
pub struct AvailabilityView {
    pub warehouse_id: Id<Warehouse>,
    pub on_hand_quantity: Quantity,
    pub reserved_quantity: Quantity,
    pub available_quantity: Quantity,
}

#[async_trait::async_trait]
pub trait AvailabilityQueryPort: Send + Sync {
    async fn available(&self, tx: &mut dyn Tx, ctx: &SecurityContext, q: AvailabilityQuery)
        -> Result<Vec<AvailabilityView>, AppError>;
    async fn on_hand(&self, tx: &mut dyn Tx, ctx: &SecurityContext,
                     legal_entity_id: Id<LegalEntity>, warehouse_id: Id<Warehouse>,
                     material_id: Id<Material>, batch_no: &str) -> Result<Quantity, AppError>;
}
```

`available` 与 HTTP 端点 A2 `GET /api/v1/inventory/available-quantities` 共用同一投影函数，`reserved_quantity` 按阶段 8 第 11.2 节 U-G-01 的临时取值恒为零。

提供方要做什么：阶段 8 在 ep-contract-inventory 增加该文件，在 ep-app-inventory 实现，在 wiring 注入。写入阶段 8 计划第 1 节 D1 的“四个对外 trait”改为五个，并在第 5 节 API 契约之后新增一小节列出五个 trait 的签名。

每个使用方要改什么。阶段 6 计划第 359 行保持调用 `AvailabilityQueryPort`，方法名写死为 `available`。阶段 7 计划第 555 行的 `StockAvailabilityQueryPort::on_hand` 改为 `AvailabilityQueryPort::on_hand`。

顺序约束：阶段 8 在阶段 6 与阶段 7 之前，无倒挂。

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

提供方要做什么：上表各阶段在自己的 ep-app-<module> 下增加 `src/probe/` 目录中的对应文件，并在两个 wiring.rs 注册到阶段 5 提供的注册表 `MasterReferenceCounterRegistry` 与 `TradeHistoryProviderRegistry`。各阶段在自己的第 9 节退出条件中增加一条“本模块的 MasterReferenceCounter 与 TradeHistoryProvider 已实现并注册”。

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

结论：阶段 6 补一个命令 trait 与三个事件。

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
    pub lines: Vec<CreateSalesReturnLine>,
}
pub struct SalesReturnSourceRef { pub source_module: ModuleCode, pub source_doc_type: String,
                                  pub source_doc_id: uuid::Uuid, pub source_doc_line_id: uuid::Uuid }
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
pub struct SalesReturnView { pub sales_return_id: uuid::Uuid, pub doc_no: String, pub status: String }

#[async_trait::async_trait]
pub trait SalesReturnCommandPort: Send + Sync {
    async fn create_sales_return(&self, tx: &mut dyn Tx, ctx: &SecurityContext,
                                 cmd: CreateSalesReturn) -> Result<SalesReturnView, AppError>;
}
```

三个事件登记到 `docs/event-catalog.md`：`sales.sales_return.closed.v1`（REGISTERED 迁到 CLOSED，payload 含 sales_return_id、doc_no、sales_order_id、source_ref、closed_at）；`sales.sales_return.cancelled.v1`（任一状态迁到 CANCELLED，payload 另含 cancel_reason）；`sales.sales_return.rejected.v1`（SUBMITTED 因审批驳回退回 DRAFT，payload 另含 reject_reason 与 approval_ref）。既有的 `sales.sales_return.registered.v1` 保持不变。

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

提供方要做什么：阶段 5、6、8、9a、10、12 各自在本模块迁移目录追加一个 `V…__<schema>_create_dataset_views.sql` 文件，并在本阶段退出条件中增加一条“本模块数据集视图已发布并授予 ep_analyst_ro，列签名已同步给阶段 11”。

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

提供方要做什么：阶段 9a 保留第 12 号建表迁移；第 14 号迁移 `V202611031005__ledger_backfill_posting_trigger_event_types.sql` 由“按十一类凭证来源各写一行且 event_type 留空”改为一次写全上表的 13 行并直接填入 `event_type` 与 `registered_by_module`，事件名逐字照抄上表，阶段 9a 不需要知道各业务模块的实现；该迁移的回退为按 `ledger_event_kind` 与 `event_type` 删除本次插入的 13 行。阶段 9a 另交付 `ep_contract_ledger::PostingTriggerRegistry::assert_registered(snapshot: &dyn SnapshotCtx, event_type: &str, kind: VoucherSourceKind, module: ModuleCode) -> Result<(), AppError>`，语义为只读断言：按 `event_type` 查种子行，缺行、`ledger_event_kind` 不符或 `registered_by_module` 不符一律返回 `AppError`，错误码取 `LEDGER.POSTING_TRIGGER_EVENT_TYPE.REGISTRY_MISMATCH`，分类 `BUSINESS_CONFLICT`、HTTP 409、不可重试，登记在阶段 9 计划的错误码表；该方法不写任何行，只供运行期启动自检比对，不供迁移调用，失败时进程以退出码 78 退出、不经 HTTP 返回。原写的幂等 upsert 语义与本条 13 行终态互斥，作废。写入阶段 9 计划第 9.3.11 节。

每个使用方要改什么。阶段 6、7、10 一律不新增 `backfill_posting_trigger_event_types` 迁移。阶段 7 计划第 3.3 节的第 24 号 `V202611030924__procure_backfill_posting_trigger_event_types.sql` 撤销，该编号由 B-02 追加的 `V202611030924__procure_backfill_append_only_registry.sql` 占用，其后编号不变，阶段 7 的迁移文件总数仍为三十一即三十个建表文件加第 24 号登记回填文件，第 433 行与第 895 行与第 1005 行的三十一一律保持不变；第 1005 行的退出条件保留三十一的计数，删去“且 `ledger.posting_trigger_event_types` 的两行 event_type 已置回空”半句，回退断言改为 `platform_core.append_only_registry` 中无本阶段残留登记行。阶段 10 计划删去 invoice 目录第 16 号 `V202611030965__invoice_backfill_posting_trigger_event_types.sql` 与 finance 目录第 24 号 `V202611031115__finance_backfill_posting_trigger_event_types.sql`，两个目录其后文件的编号与版本号一律不变，第 609 与 611 两行的写入与回退措辞整段删除，第 917 行之后的对照表保留为与种子行比对的清单。阶段 6 不新增该类文件，只在事件一节写一句“本阶段两个事件的登记行由阶段 9a 的种子迁移写入，本阶段只做运行期比对”。三个阶段改为在启动自检中经 `PostingTriggerRegistry::assert_registered` 对本模块事件做只读断言比对，缺行或 `ledger_event_kind` 或 `registered_by_module` 不符即以退出码 78 启动失败，全部“幂等 upsert”措辞一律删除。阶段 8 在第 6.4 节明确写一句“本阶段不向 ledger.posting_trigger_event_types 登记任何行，库存事件不独立产生凭证”。

顺序约束：登记行归阶段 9a 的 `db/migrations/ledger/` 种子迁移，按通则第五条不产生跨 schema 迁移，空库上按文件版本号全序执行成立。原方案把三个回填迁移放在 procure、invoice、finance 三个目录，而这三个回填文件的版本号都早于 `ledger.posting_trigger_event_types` 的建表迁移，空库上必然在该表建立之前执行并报 relation does not exist，整批迁移中断，该方案作废。

### A-22 处置流程对 DisposalPort 的实现

结论：实现归阶段 14，与密钥销毁、备份保留期一并处理。阶段 3b 至阶段 14 之间不注入任何实现，物理删除请求在此期间直接拒绝并开 DISPOSAL_NOT_DELIVERED 降级窗口，不得静默按成功路径放行。

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

结论：不设独立数据迁移阶段，按模块归属，三个通道各自落在已有阶段。

最终归属阶段：阶段 9a、阶段 10、阶段 8。

确切标识符。

| 通道 | 阶段 | 落点 |
|---|---|---|
| 总账期初余额 | 9a | 已有 `ledger.opening_balance_batches` 与 `ledger.opening_balance_batch_lines`，端点 `POST /api/v1/ledger/opening-balance-batches` 与 `/{id}/actions/confirm` |
| 应收应付预收预付期初 | 10 | 新增 `POST /api/v1/finance/opening-balances/actions/import`，请求体 `{ledger_side, accounting_period_id, rows[]}`，rows 写入 `finance.receivable_entries`、`finance.payable_entries`、`finance.advance_receipt_entries`、`finance.advance_payment_entries`，`source_doc_type` 取 `MIGRATION_OPENING` |
| 资金账户期初 | 10 | 已有 `finance.cash_accounts.opening_balance` 与 `opening_balance_period_id`，建档时一次录入，建档后不可修改 |
| 库存期初 | 8 | 已有 `MIGRATION_STOCK_ADJUSTMENT` 来源类型与 `MovementReason::MigrationOpening` |

四个通道的写入一律不生成凭证，期初对应的总账侧由 9a 的期初余额批次承担，两侧的平衡由 `finance.v_recon_*` 八个视图在首个会计期间校验。

提供方要做什么：阶段 10 增加一个用例文件 `crates/application/finance/src/usecase/import_opening_balances.rs` 与一个端点，写入阶段 10 计划第 5 节 API 契约与第 9 节退出条件。阶段 8 与阶段 9a 无新增，只在计划中把该通道标注为期初导入的唯一落点。

每个使用方要改什么。阶段 8、9、10 计划中凡出现“数据迁移阶段”的措辞一律删除，改为指向上表。

顺序约束：无倒挂。

### A-25 ep-adapter-esign crate 本体

结论：阶段 6 已在交付物中列出该 crate，本条只需补登记与替换契约测试的说明，真实对接验证在阶段 14。

最终归属阶段：阶段 6。

确切标识符：crate 名 `ep-adapter-esign`，目录 `crates/adapter/esign/`，只依赖 ep-foundation 与 `ep_domain_clm::port::SignatureGateway`，装配进 integration-gateway。内部端点固定为 `POST /internal/v1/esign/requests` 与 `GET /internal/v1/esign/requests/{external_request_id}`，只监听 127.0.0.1:8082。契约测试目标名固定为 `crates/adapter/esign/tests/contract_sandbox.rs`，wiremock 打桩目标名为 `crates/adapter/esign/tests/contract_stub.rs`，两套用例共用同一组断言函数。

提供方要做什么：阶段 6 在第 1 节交付物清单中把 crate 名写全，在第 8 节测试计划中把两套契约测试的文件名写死。

每个使用方要改什么。阶段 3 计划第 3.13 节依赖十一改为“`ep-adapter-esign` 由阶段 6 交付”。阶段 14 在其认证清单中增加一条“执行 `contract_sandbox.rs` 对真实沙箱的一次通过记录，或提交规格附录 B 允许的等效验证证据”。

顺序约束：阶段 6 早于阶段 14，无倒挂。

### A-26 platform_ops 最小台账的提前可用

结论：阶段 2 建 platform_ops schema 与 degradation_windows 一张表并提供写入端口，阶段 14 扩展为十九表五视图。

最终归属阶段：阶段 2。

确切标识符。表 `platform_ops.degradation_windows`，列与阶段 14 计划第 73 至 97 行的定义完全一致，不带 legal_entity_id，不建策略，带 `scope_legal_entity_id` 与 `scope_accounting_period_id` 两个可空标注列。阶段 2 只建表并交付两条约束 `ux_degradation_windows_kind_scope_closed` 与 `ck_degradation_windows_open_order`，其余两条 CHECK 与全部索引由阶段 14 追加。

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

`DegradationKind` 由阶段 2 定义为空枚举加五个初始取值 `OFFSITE_SINK_NOT_CONFIGURED`、`WRITER_NOT_IN_SERVICE`、`WASM_COMPUTE_NOT_DELIVERED`、`RULE_EVALUATOR_NOT_DELIVERED`、`DISPOSAL_NOT_DELIVERED`，阶段 14 扩展到二十一类。`WRITER_ROLE_CONTAINMENT_MISSING` 作废，其触发条件由遏制手段配置缺失改为客观事实即任一写出进程未运行或连续 N 个周期无上报，并取消不可关闭属性，`ck_degradation_windows_not_suppressible` 只护住 `OFFSITE_SINK_NOT_CONFIGURED`。指标 `ep_degradation_windows_open` 由阶段 2 注册并填充。

提供方要做什么：阶段 2 在第 3.4 节迁移表中把第 16 号 `platform_ops_create_schema` 之后追加一个 `V…__platform_ops_create_degradation_windows.sql`，在 ep-platform-obs 交付上述 trait 与 pg 实现。写入阶段 2 计划第 1 节交付物清单与第 3.5 节表定义。

每个使用方要改什么。阶段 1 计划的自检项 `offsite-sink-requirements` 在失败时调用 `DegradationLedger::open`，但阶段 1 早于阶段 2，因此阶段 1 只写 stderr 并留注释 `// TODO(stage-2): write degradation ledger`，阶段 2 补上。阶段 3 计划第 3.13 节依赖九删去 `platform_ops.degradation_windows` 一项。阶段 4、9、11、13 凡登记降级窗口的措辞改为调用 `DegradationLedger`。阶段 14 计划第 73 节明确本表由阶段 2 建立、本阶段只做扩展。

顺序约束：阶段 2 早于 3、4、9、11、13、14，倒挂解除。阶段 1 是唯一早于阶段 2 的使用方，按上述注释处理。

### A-27 ep-platform-release 配置发布通道的提前可用

结论：端口在阶段 3a，最小发布通道在阶段 3b，低代码全量与自动测试在阶段 13b。阶段 2 不使用该通道。

最终归属阶段：阶段 3a 交付端口，阶段 3b 交付最小通道，阶段 13b 扩展。

确切标识符。阶段 3b 交付的最小通道含三张表与一个状态机，表落在 platform_meta schema：`platform_meta.config_packages`、`platform_meta.config_package_items`、`platform_meta.config_release_orders`，列定义与阶段 13 计划第 3 节所列一致；platform_meta 下其余与配置发布相关的表一律归阶段 13b，本条不再逐张点名。原裁定用的表名 `config_item_apply_logs` 全库没有对应对象，其所指是阶段 13b 的 `platform_meta.config_release_steps`，该旧名作废，三份文件中不得再出现，也不再保留任何括注映射。发布状态机取 PRD 第 10.4.1 节的十一态为唯一出处：阶段 3b 实现其中六态 Draft、PendingApproval、Rejected、Approved、Released、RolledBack，差异审查由 `GET /api/v1/platform/config-packages/{id}/diff` 端点承载，不单列为状态，原裁定写的 PendingReview 在 PRD 中不存在，一并作废；阶段 13b 补齐其余五态 PendingAutotest、TestFailed、TestPassed、SignedPendingRelease、Superseded，六加五合计十一态，扩展只放宽 `ck_config_packages_status`，不改写任何既有行。签名算法固定为 ECDSA P-256，`item_hash` 算法与阶段 13 计划第 553 行一致。

提供方要做什么：阶段 3a 只交付 `crates/platform/release/src/port/config_item.rs`（见 A-19）。阶段 3b 交付三张表的迁移、发布与回退用例、`ConfigItemApplierRegistry` 的运行期装配以及两个 applier。写入阶段 3 计划第 3.1 节交付物清单，作为第 19 项与第 20 项。

每个使用方要改什么。阶段 2 计划中凡依赖配置发布通道的措辞改为“阶段 2 不使用发布通道，敏感字段登记与密钥域配置直接经迁移与端点写入，发布通道接入由阶段 3b 反向补齐”。阶段 5、6、7、9、10、11 的配置对象发布一律经阶段 3b 的通道，不自建第二套。阶段 13 计划把三张表标注为阶段 3b 已建、本阶段只做列扩展与状态扩展。

顺序约束：3a → 3b → 5 → …，全部使用方在其后，倒挂解除。

### A-28 字段元数据登记入口

结论：阶段 5 不依赖 platform_meta，改用阶段 2 的 `platform_core.sensitive_field_registry` 与阶段 4 的 `platform_authz.field_permissions` 两处承载。本条按权威顺序把规格的强制项与 PRD 的待决项分开裁定，权威链如下。规格第 7.8 章写明事务数据库使用信封加密、行内敏感字段按法人密钥域与密级使用字段级密钥属于强制项，并写明行内敏感字段由对象密级、字段密级或经产品负责人批准的敏感字段清单确定、至少覆盖身份与联系方式与账户与税号与支付认证令牌和法律或健康等高敏感属性。银行账号即该章明列的账户类属性，且本条已给该列赋 `security_level` 30，字段密级这一条判据本身即已成立，因此银行账号纳入行内敏感字段与对其做字段级加密两件事由规格直接强制，不是待决项，本条按规格取 `is_field_encrypted` 为真。PRD 附录乙 U-A-12 的原文只问三件事，即敏感字段清单是否包含开户银行与银行账号、这两列在列表与详情与导出三种场景的脱敏形态、导出是否触发重新认证，全文不出现加密二字；其中银行账号的纳入已被规格强制，剩余待决的只有开户银行是否同列、三场景脱敏形态、导出是否触发重新认证三问，决策人为安全负责人与产品负责人，本表不代拍，只给临时取值与切换代价。上一轮以 U-A-12 为由把银行账号退回明文，使规格第 7.8 章的强制项在首版落空，按权威顺序规格高于 PRD 也高于本表，该取值撤销；上一轮所称撤销的加密与否一项在 U-A-12 条文中并不存在，该撤销一并作废。任何阶段不得再据 U-A-12 把银行账号退回明文。

最终归属阶段：登记表归阶段 2 与阶段 4，登记行归各引入受保护列的模块阶段，首版共六行，即阶段 3b 一行、阶段 5 四行、阶段 10 一行。

确切标识符。阶段 5 在 `db/migrations/mdm/` 追加一个 `V…__mdm_backfill_sensitive_field_registry.sql`，向 `platform_core.sensitive_field_registry` 插入四行，不是两行：该表的唯一约束 `ux_sensitive_field_registry_schema_table_column` 落在三列上，两张表乘两列必然是四行。银行字段不在 `mdm.customers` 与 `mdm.suppliers` 上，这两个表名作废，实际落点是 `mdm.customer_invoice_profiles` 与 `mdm.supplier_payment_profiles`。四行逐列取值固定如下，列集按 C-06 冻结的十一列，缺一写不出 INSERT。

| schema_name | table_name | column_name | category | security_level | is_field_encrypted | blind_index | blind_index_column | mask_style | normalization | release_ref |
|---|---|---|---|---|---|---|---|---|---|---|
| mdm | customer_invoice_profiles | bank_name | ACCOUNT | 30 | false | NONE | 空 | NONE | TRIM_NFKC | `MIGRATION:<本迁移版本号>` |
| mdm | customer_invoice_profiles | bank_account_no | ACCOUNT | 30 | true | EXACT | bank_account_no_bidx | KEEP_LAST_4 | TRIM_NFKC | `MIGRATION:<本迁移版本号>` |
| mdm | supplier_payment_profiles | bank_name | ACCOUNT | 30 | false | NONE | 空 | NONE | TRIM_NFKC | `MIGRATION:<本迁移版本号>` |
| mdm | supplier_payment_profiles | bank_account_no | ACCOUNT | 30 | true | EXACT | bank_account_no_bidx | KEEP_LAST_4 | TRIM_NFKC | `MIGRATION:<本迁移版本号>` |

`column_name` 取逻辑列名，不带 `_enc` 后缀。`created_by` 取 `foundation::SYSTEM_PRINCIPAL_ID`。`bank_account_no` 两行的 `is_field_encrypted` 取 true，由规格第 7.8 章强制，不是临时取值；两张表的物理列相应为 `bank_account_no_enc bytea` 可空、`bank_account_no_key_ref text` 可空记录密钥标识与版本、`bank_account_no_tail text` 可空承载掩码保留的后四位、`bank_account_no_bidx bytea` 可空，一律不保留同名明文列 `bank_account_no`。`bank_name` 两行的 `is_field_encrypted` 取 false，物理列为 `bank_name text` 可空，这是 U-A-12 未决期间的临时取值。`db/checks/11` 按 `is_field_encrypted` 分支断言：取真的登记项断言物理表上存在 `<column_name>_enc` 列且类型为 `bytea` 且不存在同名明文列 `<column_name>`；取假的登记项只断言 `<schema_name>.<table_name>.<column_name>` 三元组在 `information_schema.columns` 中命中实际列，不施加 bytea 与 `_enc` 后缀断言。四行中除 `bank_account_no` 两行的存在由规格强制外，其余各行的存在本身与四行的 `security_level`、`mask_style` 均为 U-A-12 未决期间的临时取值，决策人为安全负责人与产品负责人；`category` 取 ACCOUNT、`normalization` 取 TRIM_NFKC、`release_ref` 取迁移版本号三列由本条固定，不属待决。盲索引按 B-04 建立，规格第 7.8 章禁止字段级密文直接用于唯一约束，盲索引是唯一的查重手段。

阶段 3b 的一行与阶段 10 的一行按同一列集给全。阶段 3b 在 `db/migrations/platform_msg/` 追加一个 `V…__platform_msg_backfill_sensitive_field_registry.sql`，与第 33 号同目录并排在其后，向 `platform_core.sensitive_field_registry` 插入一行，十一列逐列取值为：`schema_name` 取 platform_msg，`table_name` 取 push_registrations，`column_name` 取 token 即逻辑列名不带 `_enc`，`category` 取 PAYMENT_TOKEN，`security_level` 取 30，`is_field_encrypted` 取 true，`blind_index` 取 EXACT，`blind_index_column` 取 token_bidx，`mask_style` 取 FULL，`normalization` 取 NONE，`release_ref` 取 `MIGRATION:<本迁移版本号>`；`created_by` 取 `foundation::SYSTEM_PRINCIPAL_ID`，`-- rollback:` 段按 `schema_name` 与 `table_name` 删该行。`mask_style` 不得取 KEEP_LAST_4，该表没有 `token_tail` 列；`normalization` 不得取 TRIM_NFKC，推送令牌是大小写敏感的不透明串，规范化会改写 `derive_blind_key` 的入参。该行的依据是规格第 7.8 章把支付认证令牌列入行内敏感字段的最低覆盖面，属规格强制，不是临时取值。首版字段级加密列因此共四处而不是三处：阶段 4 计划第 387 行的“三处的 bank_account_no”改为“`mdm.customer_invoice_profiles` 与 `mdm.supplier_payment_profiles` 与 `finance.cash_accounts` 三处的 `bank_account_no`，加 `platform_msg.push_registrations.token`，共四处，前三处 `mask_style` 取 KEEP_LAST_4 由阶段 5 与阶段 10 交付，后一处取 FULL 由阶段 3b 交付”，同句“全库只有这一处解密位点”收窄为“经字段投影返回给用户的解密只有这一处”，推送令牌的解密由 job-worker 在投递链路上直接取用，不经 `FieldProjector`，也不受字段权限判定。阶段 3 计划第 3.9 节增加一条退出条件，即该行存在且 `is_field_encrypted` 为真、`blind_index_column` 为 `token_bidx`、`mask_style` 为 FULL，`db/checks/11` 返回零行。阶段 2 计划第 135 与 823 两行的登记行名单改为“全库共六行，阶段 3b 一行、阶段 5 四行、阶段 10 一行”。

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

确切标识符。登记行的列以阶段 2 实建的 `platform_core.append_only_registry` 为准，为 `schema_name`、`table_name`、`mode`、`mutable_columns`；`mode` 取 `APPEND_ONLY` 或 `IMMUTABLE_COLUMNS`，`mutable_columns` 是可变列白名单，取 `APPEND_ONLY` 时必须为空数组。原裁定写的 `immutable_columns` 列在该表上不存在，语义又与 `mutable_columns` 相反，该列名作废。登记方与登记行固定为十四行，如下表。

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
| 阶段 9a | platform_core.recon_runs | APPEND_ONLY | `'{}'` |
| 阶段 10 | finance.unbilled_ar_entries | APPEND_ONLY | `'{}'` |
| 阶段 10 | finance.cash_ledger_entries | APPEND_ONLY | `'{}'` |

阶段 7、8、9a、10 的十一行一律 `mode` 取 `APPEND_ONLY`、`mutable_columns` 取 `'{}'`，阶段 3b 的三行按上表逐表给定。死信的可变列必须取全五列，只取三列会让触发器拒绝写 `repaired_at` 与 `discard_reason`，修复完成与丢弃两条路径在上线后直接失败；`platform_audit.audit_segments` 有状态与锚定时间更新，登记为仅追加会拒绝锚定写入，不进本清单。原裁定给阶段 9a 列的 `ledger.general_vouchers` 全库没有同名对象，GV 是 `ledger.vouchers` 的单据类型码，该行删除；原裁定给阶段 10 列的 `finance.receivable_entries`、`finance.payable_entries`、`finance.advance_receipt_entries`、`finance.advance_payment_entries`、`finance.overbilling_entries` 五张是带核销金额与状态机的可更新台账，登记为仅追加会在上线后拒绝正常核销写入，五行一并删除。触发器按登记表挂接，`platform_core.attach_table_guards` 读登记表取可变列白名单，同一迁移内必须先插登记行再挂接触发器，顺序颠倒即取不到白名单；凡本条列出的表都必须有登记行，漏登记等于无强制。检查脚本名固定为 `db/checks/append_only_consistency.sql`，由 `xtask sqlcheck` 执行。

回写：阶段 3b、7、8、9a、10 各追加一个 `V…__<schema>_backfill_append_only_registry.sql`，并在各自退出条件中增加一条。阶段 3b 的一个是阶段 3 计划第 3.3.1 节迁移清单在第 32 号之后追加的第 33 号 `V2026110209xx__platform_msg_backfill_append_only_registry.sql`，属 3b 段；该文件同时登记 platform_audit 与 platform_msg 两个 schema 的表，其主要创建对象是 platform_msg 两张仅追加表的登记行与触发器，按通则第五条取 `db/migrations/platform_msg/` 目录；文件内先按上表插入三行登记，再依次调用 `platform_core.attach_table_guards('platform_audit','audit_events')`、`('platform_msg','outbox_events')`、`('platform_msg','dead_letters')`，顺序不得颠倒，回退段为删除该三行并 drop 对应触发器。阶段 9a 的一个同时登记 ledger 与 platform_core 两个 schema 的表，其主要创建对象是 ledger 两张仅追加表的登记行与触发器，按通则第五条放在 `db/migrations/ledger/` 目录下；`platform_core.recon_runs` 的建表迁移版本号早于本文件，该表在本文件执行时已存在，挂接可行。阶段 7 的一个占用第 24 号编号，见 A-21。五个登记方的写法一律与阶段 3b 的第 33 号相同，没有例外：文件内先按上表插入本阶段的登记行，再对每张表各调用一次 `platform_core.attach_table_guards('<schema>','<table>')`，顺序不得颠倒；`-- rollback:` 段一律为删除本次登记的行并 drop 该批表上对应的触发器，只删登记行不 drop 触发器在回退方向上同样使 `db/checks/append_only_consistency.sql` 判负。逐阶段的调用清单固定为：阶段 7 一次，取 `('procure','goods_receipt_line_costings')`；阶段 8 五次，依次取 `('inventory','stock_movements')`、`('inventory','stock_qty_entries')`、`('inventory','stock_value_entries')`、`('inventory','variance_splits')`、`('inventory','stock_movement_serials')`；阶段 9a 三次，依次取 `('ledger','vouchers')`、`('ledger','voucher_lines')`、`('platform_core','recon_runs')`；阶段 10 两次，依次取 `('finance','unbilled_ar_entries')`、`('finance','cash_ledger_entries')`。各阶段的建表迁移一律不调用 `attach_table_guards`，仅追加表的触发器只在本条的 backfill 文件内挂接。阶段 2 计划第 358 行死信的可变列白名单由三列改为五列，第 135、372、823 三行的登记方名单由阶段 7、8、9a、10 改为阶段 3b、7、8、9a、10。阶段 3 计划第 1536 行第 7.2 章一行的强制手段改为“三张仅追加表按 `append_only_registry` 登记挂接 `assert_append_only` 与 `assert_immutable_columns` 触发器、不授予 DELETE、`ep-adapter-file` 的不可删除不可覆盖命名空间、CI 的 SQL 静态检查”，并在第 9 节退出条件增加一条，即该三行登记存在且 mode 与可变列白名单按上表取值、触发器已挂接、`xtask sqlcheck` 执行 `db/checks/append_only_consistency.sql` 返回零行。阶段 8 计划第 294 行删去“本阶段按裁定只登记四行”与提请复核一语，改为五行；阶段 9 计划第 103 行删去关于 `ledger.general_vouchers` 的解释并改为三行；阶段 10 计划第 607 与 1198 行的“逐表给出 `immutable_columns`”改为“两行的 `mode` 取 `APPEND_ONLY`、`mutable_columns` 取 `'{}'`”。

### B-03 platform_core.migration_windows 与 open-window 校验

结论：阶段 13b 显式接入，在线 DDL 计划执行前必须持有迁移窗口。

最终归属阶段：阶段 13b。

确切标识符：端口 `ep_foundation::port::db::MigrationWindowGuard`，与 C-07 的 `IdempotencyStore` 同 crate 同模块，唯一方法为 `async fn assert_open(&self, tx: &mut dyn Tx) -> Result<(), AppError>`，由阶段 2 定义；唯一实现类型 `PgMigrationWindowGuard` 位于 `crates/adapter/db-pg/`，同为阶段 2 交付；在 `apps/core-server/src/wiring.rs` 与 `apps/job-worker/src/wiring.rs` 注入。阶段 13b 的在线 DDL 由 job-worker 的 DDL 执行器发起，窗口校验在把控制交给 ep-platform-release 的编排之前由该执行器调用注入实例的 `assert_open(tx)`；`ep-platform-release` 不引用该 trait，也不新增任何 adapter 方向的依赖。原裁定写的 `ep_platform_release::MigrationWindowGuard` 违反基线第 1.3 节“ep-platform-* 只可依赖 ep-foundation 与其他 ep-platform-*”，基线高于本表，该路径作废；阶段 3a 不承担再导出，本条对阶段 3 无落点。未持有窗口时返回 `PLATFORM.DB.MIGRATION_WINDOW_CLOSED`，HTTP 409，category 为 BUSINESS_CONFLICT。

回写：阶段 2 计划第 110 行改为上述端口与实现的落点并删去“由阶段 3a 建立 ep-platform-release crate 时以再导出方式暴露”一句，第 3.3 节把端口与实现列为对外可用组件，退出条件 E-17 改为“端口与 `PgMigrationWindowGuard` 实现均已交付且两个 wiring 已注入”；阶段 13 计划第 4.3 节 DDL 段第一步与第 895、984 三处去掉 `ep_platform_release::` 前缀，改为经装配注入的实例调用；阶段 3 计划一字不改。

### B-04 derive_blind_key 与 BlindIndex

结论：阶段 10 的银行账号查重改用盲索引，不自建第二套哈希。

最终归属阶段：阶段 10 使用，阶段 2 提供。

确切标识符：列名固定为 `bank_account_no_bidx bytea`，取值为 `derive_blind_key(legal_entity_id, 'finance.cash_accounts.bank_account_no', plaintext)`，唯一约束名 `ux_cash_accounts_legal_entity_id_bank_account_no_bidx`。同一约定在 mdm 的客户与供应商银行账号上同名同构，列名为 `bank_account_no_bidx`。

回写：阶段 10 计划中银行账号查重的措辞由“哈希加盐”改为“`derive_blind_key` 与 `BlindIndex`”；阶段 5 的对应列同样按此命名。

### B-05 WasmComputePort 与 RuleEvaluator 端口

结论：阶段 13b 显式实现这两个端口，不另起接口。阶段 3b 至阶段 13b 之间两个端口都不注入任何实现，rule-evaluations 端点在阶段 13b 之前不注册，能力缺位时开 WASM_COMPUTE_NOT_DELIVERED 或 RULE_EVALUATOR_NOT_DELIVERED 降级窗口并返回可重试错误，不得静默返回成功。

最终归属阶段：端口归阶段 3b，实现归阶段 13b。

确切标识符：`ep_platform_flow::port::WasmComputePort` 与 `ep_platform_flow::port::RuleEvaluator`，实现类型名固定为 `PluginHostWasmCompute`（位于 `crates/adapter/wasm/`，装配进 plugin-host）与 `AstRuleEvaluator`（位于 `crates/platform/meta/src/rule/`，装配进 core-server）。端点 `POST /api/v1/platform/rule-evaluations/actions/evaluate` 只调用 `AstRuleEvaluator`，不新建求值路径。

回写：阶段 13 计划第 4.5 节明确这两个实现类型名与其对应端口；阶段 3 计划把两个端口列入交付物清单并注明实现方为阶段 13b。

### B-06 ep-contract-service::EquipmentQuery

结论：撤销该 trait。理由是客户 360 的设备区块由 ep-app-service 自己的 `Customer360SectionProvider` 实现，不需要跨模块 trait；报表侧按阶段 11 的 D-11-01 一律经受治理数据集视图取数，不经 contract trait；低代码的设备引用经 HTTP 端点解析。

最终归属阶段：阶段 12 撤销。

确切标识符：删除 `crates/contract/service/src/port/equipment.rs`。设备的跨模块可见性只保留三条路径：`GET /api/v1/service/equipments` 与 `/{id}`、全文检索索引中的 `service.equipment_records` 文档、以及阶段 12 自身的 `EquipmentsSectionProvider`。

回写：阶段 12 计划第 2 节 crate 表删去 `EquipmentQuery`；第 9.3 节的三个读取方改为上述三条路径。

### B-07 ep-contract-procure::PurchaseReturnLinkPort

结论：阶段 7 反向接入阶段 6 留下的勾稽空位，阶段 6 的直运退货验收顺延到阶段 7。

最终归属阶段：阶段 7 提供与接入。

确切标识符：`ep_contract_procure::PurchaseReturnLinkPort::link_drop_ship_return(tx, ctx, sales_return_id: uuid::Uuid, lines: Vec<DropShipReturnLine>) -> Result<PurchaseReturnLinkView, AppError>`。阶段 6 在 wiring 注入 `NoopPurchaseReturnLinkPort`，阶段 7 替换。

回写：阶段 6 计划的直运退货用例增加该调用点与空实现注入；阶段 6 退出条件中直运退货勾稽一条标注为顺延到阶段 7；阶段 7 计划在退出条件中增加“已替换阶段 6 的空实现，直运退货勾稽端到端通过”。

### B-08 finance.v_recon_inventory 与 v_recon_grni 两个视图外壳

结论：阶段 10 自行反向接入，阶段 7 与阶段 8 必须提供子账侧查询实现。

最终归属阶段：视图归阶段 10，子账侧实现归阶段 8 与阶段 7。

确切标识符：`ep_contract_finance::ReconciliationItemQuery` 由阶段 10 定义；子账侧的两个实现类型名固定为 `InventorySubledgerBalanceQuery`（阶段 8，`ep-app-inventory`，返回该法人该期间的存货金额账合计）与 `GrniSubledgerBalanceQuery`（阶段 7，`ep-app-procure`，返回已收货未收票暂估合计）。两者均实现 `ep_contract_finance::SubledgerBalanceProvider::balance(snapshot: &dyn SnapshotCtx, legal_entity_id, accounting_period_id) -> Result<Money, AppError>`，该 trait 由阶段 10 定义，阶段 8 与阶段 7 在其阶段之前无法实现，因此按反向依赖由阶段 10 在交付时补齐两个实现的接线，实现体本身由阶段 8 与阶段 7 各自以本模块的查询函数形式先行交付并在阶段 10 包装。跨阶段的唯一取数入口是 `ep_contract_finance::ReconciliationItemQuery`，按法人与会计期间返回十项勾稽的子账侧合计，结构为 `ReconciliationItemView`；`SubledgerBalanceProvider` 只是阶段 10 内部组装该结果的手段，阶段 9b 的关账前强制校验与其 `ReconCheck` 一律调用 `ReconciliationItemQuery`，不直接调用 `SubledgerBalanceProvider`。十项中的存货与已收货未收票两项子账侧取自阶段 8 与阶段 7 的查询函数，其余八项取自阶段 10 自有表，不经该 trait。`ep-app-ledger` 为此在依赖方向自检清单中新增一条对 `ep-contract-finance` 的依赖，只用于 9b 段，符合基线第 1.3 节；`ep-platform-recon` 不依赖任何模块的 ep-contract-*，阶段 10 契约表中把该执行器列为使用方的措辞收窄为“由 ep-platform-recon 的执行器驱动阶段 9b 实现的 ReconCheck，不由其直接依赖”。

回写：阶段 8 与阶段 7 各在退出条件中增加一条“已提供本模块的子账侧余额查询函数，函数名与返回类型按 B-08 固定”；阶段 10 计划第 1131 行的措辞改为“两个实现由阶段 10 包装阶段 7 与阶段 8 提供的查询函数”。

### B-09 inventory.stock_movement.value_adjusted.v1

结论：保留该事件，按基线第 6.1 节的四段命名改名为 `inventory.stock_movement.value_adjusted.v1`，消费者固定为阶段 11 的成本同步消费者。

最终归属阶段：事件归阶段 8，消费者归阶段 11。

确切标识符：消费者名固定为 `costing.stock_value_adjust`，位于 `crates/application/costing/src/consumer/stock_value_adjust.rs`，在 job-worker 注册，幂等由 `platform_msg.inbox_consumptions(consumer, event_id)` 保证，副作用为向 `costing.cost_entries` 补记只影响金额账的调整对应的成本条目。

回写：原名 `inventory.stock_value_adjusted.v1` 只有三段，违反基线第 6.1 节“事件类型为四段”，基线高于本表，该名全库作废，任何阶段不得引用，也不得为此在 `docs/event-catalog.md` 中登记命名例外，处置逻辑与本表第 522 行对违规索引名的处置相同。aggregate 段取 `stock_movement`，与阶段 8 第 595 行已声明的 `aggregate_type` 即 `inventory.stock_movements` 同源；另一候选名 `inventory.stock_value.adjusted.v1` 会使 aggregate 段与 `aggregate_type` 不同源，予以排除。逐字替换的落点共六处：阶段 8 计划第 595 与 597 行，阶段 11 计划第 66、503、693、758 行。阶段 11 计划第 3 节新增该消费者并在退出条件中增加一条；阶段 8 计划第 6.4 节把该事件的消费者由“报表数据集”改写为 `costing.stock_value_adjust`。消费者名 `costing.stock_value_adjust` 与文件名 `stock_value_adjust.rs` 不是事件类型，不受四段规则约束，保持不动；阶段 8 计划第 29 行的“新增 2 条”与阶段 11 计划第 64 行的“新增 3 条”是条数，改名不改数。

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

确切标识符：`ep_adapter_db_pg::PoolKind { Rw, Ro, Worker, Integ, Ops }`；`ep_adapter_db_pg::SessionContext { legal_entity_id, user_id, request_id, trace_id }`；`ep_adapter_db_pg::RetryPolicy { max_attempts: u8, backoff_ms: [u16; 3], retryable_sqlstates: &'static [&'static str] }`；`ep_adapter_db_pg::ConnectionBudget { resident_max: u16, burst_max: u16, per_pool: [(PoolKind, u16); 5] }`。取值固定为 max_attempts 3、backoff_ms [50, 150, 450]、retryable_sqlstates ["40001", "40P01"]、resident_max 42、burst_max 52。校验脚本名 `scripts/verify-connection-budget.sh` 归阶段 2。

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

确切标识符：阶段 10 先建 `finance.aging_bucket_definitions` 作为临时表并在其计划中标注为临时；阶段 11 交付两个迁移文件，都放在 `db/migrations/reporting/`：第 13 号 `V202611031060__reporting_backfill_migrate_aging_buckets_from_finance.sql` 迁数据，第 14 号 `V202611031065__reporting_drop_finance_aging_bucket_definitions.sql` 删除 finance 侧临时表。删表文件不新建任何对象，按通则第五条随其成对的迁数据文件归 reporting 目录，两个文件同属一个 Runner，按版本号先迁后删自然成立。原方案把删表文件放在 `db/migrations/finance/`，其版本号与 finance 目录既有的 `V202611031065` 撞号，而全局版本号必须唯一且严格递增，该方案作废；阶段 11 为规避该顺序风险自加的标记行守卫一并删除，不再保留任何跨 Runner 的顺序断言。跨 schema 的 DROP 由 `ep_migrator` 执行，该角色已具备全部 `ep_mod_*` 成员资格。取用入口为 `ep_contract_reporting::AgingBucketQuery::buckets(tx, ctx, legal_entity_id, ledger_side) -> Result<Vec<AgingBucket>, AppError>`。

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

确切标识符：`invoice.tax_rate_options`，取用入口 `ep_contract_invoice::TaxRateOptionQuery::default_rate(tx, ctx, legal_entity_id, item_id: uuid::Uuid) -> Result<Rate, AppError>` 与 `list(tx, ctx, legal_entity_id) -> Result<Vec<TaxRateOption>, AppError>`。阶段 5 的 `mdm.classification_items` 去掉税率一类，阶段 10 之前的临时取值由阶段 5 的字典桩 `MdmTaxRateStub` 承担，阶段 10 交付时执行 `V…__invoice_backfill_migrate_tax_rates_from_mdm.sql` 迁移并删除桩。

回写：阶段 5 计划的分类项类别清单删去税率一类，并注明桩类型名与其撤销时点；阶段 6 计划取默认税率一律经 `ep_contract_invoice::TaxRateOptionQuery`，不经 `ep-contract-mdm`；阶段 10 计划增加该迁移文件。

### C-12 收货入账单价的固化位置

结论：权威出处归阶段 8 的 `inventory.stock_value_entries`，`procure.goods_receipt_line_costings` 只保留数量与金额的分配关系。

最终归属阶段：阶段 8。

确切标识符：从 `procure.goods_receipt_line_costings` 中删去单价列，保留 `goods_receipt_line_id`、`quantity`、`amount`、`allocation_kind`、`source_purchase_invoice_line_id`。单价一律经 `ep_contract_inventory::InventoryPricingLookupPort::original_unit_price_by_source_line(tx, ctx, source_doc_line_id: uuid::Uuid) -> Result<UnitPrice, AppError>` 回查，取数为 `inventory.stock_value_entries.applied_unit_price`。

回写：阶段 7 计划第 3.2.11 节删去单价列并在同节写明回查路径；阶段 8 计划第 11.3 节 E5 保持不变，端口名写死为 `InventoryPricingLookupPort::original_unit_price_by_source_line`。

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
    pub delivered_unbilled_amount: Money,
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
                                    pub delivered_unbilled_amount: Money }
#[async_trait::async_trait]
pub trait ReceivableExposureQuery: Send + Sync {
    async fn exposure(&self, tx: &mut dyn Tx, ctx: &SecurityContext, customer_id: Id<Customer>)
        -> Result<ReceivableExposureView, AppError>;
}
```

`finance::CreditExposureQuery` 与 `finance::CustomerCreditExposurePort` 两个名字作废。

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

确切标识符：`ep_contract_procure::PurchaseRequisitionIntakePort::intake(tx, ctx, cmd: PurchaseRequisitionIntake) -> Result<PurchaseRequisitionView, AppError>`，`PurchaseRequisitionIntake` 含 `source_module: ModuleCode`、`source_doc_id`、`source_doc_line_id`、`material_id`、`quantity`、`required_on`、`unique_key`。阶段 6 的 `PurchaseRequisitionDerivationPort` 作废。

回写：阶段 6 计划中该端口名的全部出现处改写，并注明阶段 6 先注入 `NoopPurchaseRequisitionIntakePort`；阶段 12 计划第 `project.project_task.requisition_requested.v1` 的下游也统一走该端口。

### C-18 库存过账端口命名

结论：统一取阶段 8 的 `InventoryPostingPort`，可用量查询另立 `AvailabilityQueryPort`。

最终归属阶段：阶段 8。

确切标识符：`ep_contract_inventory::InventoryPostingPort` 的三个方法固定为 `post_inbound(tx, ctx, InboundPosting) -> Result<InboundPostingResult, AppError>`、`post_outbound(tx, ctx, OutboundPosting) -> Result<OutboundPostingResult, AppError>`、`find_movement_by_source(tx, ctx, SourceRef) -> Result<Option<MovementResult>, AppError>`。阶段 7 的 `StockInboundPort`、`StockOutboundPort`、`StockAvailabilityQueryPort` 三个名字作废，第三个由 `AvailabilityQueryPort` 承接（见 A-12）。

回写：阶段 7 计划中三个端口名的全部出现处改写；阶段 8 计划在第 5 节之后新增一小节列出五个 trait 与其完整方法签名。

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

结论：统一为 `ep_replication_crosscheck_age_seconds`，注册与填充均归阶段 14。原裁定的注册归阶段 14、填充归阶段 2 把注册排在填充之后十二个阶段，在本计划的集中注册模型下阶段 2 取不到句柄；且交叉核对器的装配、结论表 `platform_ops.replication_crosscheck_runs`、无结论窗口与比对本体全部在阶段 14，阶段 2 没有 apps 装配点也产不出距上次核对结论的时长，该口径作废。

最终归属阶段：注册与填充均归阶段 14。阶段 2 只交付 `ep-adapter-db-pg` 的复制交叉核对取数函数与只读分析池中划出的独占连接，不触及该指标。

确切标识符：`ep_replication_crosscheck_age_seconds`，类型 gauge，标签 `channel`（取值 archive、backup）。阶段 2 的 `ep_db_replication_crosscheck_age_seconds` 作废。

回写：阶段 2 计划第 7.2 节删去该指标一行，指标表由五行变四行，与该数字相关的 D-13、E-12、E-18 三处一并由五个改为四个，第 658 行改写为“按 C-22，复制交叉核对指标统一为 `ep_replication_crosscheck_age_seconds` 并由阶段 14 一次性注册与填充，本阶段曾用的 `ep_db_replication_crosscheck_age_seconds` 作废，本阶段只交付其取数函数与只读分析池的独占连接，不登记也不填充”，第 2 节 crate 表 ep-platform-obs 一行的职责相应收窄为第 7.2 节四个指标中归本阶段的两项的注册与填充以及归阶段 1 注册的两项的填充；阶段 14 计划在指标清单中一次性登记该指标的注册与填充，第 350 行的填充方由阶段 2 改为本阶段，退出条件 19 改为该指标已由本阶段注册并填充且该条目由本阶段首次写入 `docs/metrics-catalog.md`、阶段 2 不写；总览第 3.2 节阶段 2 行的反向依赖删去该指标的注册一项。同源重复的两个配置键二取一：删去阶段 2 的 `EP__DB__REPL_CHECK__INTERVAL_S`，周期与语句超时统一取阶段 14 的 `EP__OPS__CROSSCHECK_PERIOD_SECONDS` 与 `EP__OPS__CROSSCHECK_STATEMENT_TIMEOUT_MS`，阶段 2 只保留连接侧的 `EP__DB__POOL__RO_REPL_CHECK_RESERVED`，其假设五改写为周期取值与上限 300 秒的判定归阶段 14 的配置键、本阶段只保证独占连接与其 5 秒专用语句超时。

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

结论：自检项改为按注册名标识，不用序号。基线第 7.3 节的十三项固定项一并改为命名项。

最终归属阶段：阶段 1 定义注册表与十三个基线项名，各阶段追加自己的命名项。

确切标识符：`SelfCheckRegistry` 位于 `crates/platform/runtime/src/selfcheck/registry.rs`，注册项为 `SelfCheckItem { name: &'static str, title: &'static str, severity: Severity, run: … }`，name 为 kebab-case。自检项分 Blocking 与 Degrading 两级，判读运行期可变业务数据行的一律取 Degrading 且不得作为启动失败条件，全量清单与分级以基线第 7.3 节与总览第 4.3 节 C-25 行为唯一出处，下表各阶段追加项按该出处重写。基线十项的名字固定为 config-parsed、database-reachable、migration-version-matched、rls-enabled-and-forced、runtime-role-privileges-bounded、secrets-resolvable、file-store-writable、clock-skew-within-limit、audit-chain-verifiable、offsite-sink-requirements，其中前八项取 Blocking、后两项取 Degrading；原列的十三项中 cgroup-quota-matched 与 license-and-modules-consistent 与 current-period-open 三项撤销。原十三项清单如下，仅供追溯，不再有效：`config-parsed`、`database-reachable`、`migration-version-matched`、`rls-enabled-and-forced`、`runtime-role-privileges-bounded`、`secrets-resolvable`、`audit-chain-verifiable`、`file-store-writable`、`clock-skew-within-limit`、`cgroup-quota-matched`、`offsite-sink-requirements`、`license-and-modules-consistent`、`current-period-open`。

各阶段追加项的名字固定如下，任何阶段不得再用序号称呼。

| 阶段 | 追加项名 |
|---|---|
| 3b | audit-evidence-store-writable、audit-signing-key-usable、attachment-store-ready、event-catalog-consistent |
| 4 | duty-class-exclusivity、forbidden-permission-items-absent、authz-snapshot-loadable |
| 5 | master-data-usage-probes-registered |
| 11 | reporting-dataset-signature-matched |
| 13 | client-capability-matrix-frozen、custom-object-ddl-consistent |

报告按注册顺序输出，注册顺序即上表顺序，基线十三项在前。

回写：基线第 7.3 节把十三项编号列表改为命名列表；阶段 1 计划第 5 节按名字重写；阶段 3、4、5、11、13 计划中所有“第 14 项”“第 14 至 16 项”的措辞按上表替换为项名。

### C-26 单据类型码的全局唯一性

结论：类型码统一登记在 `docs/data-dictionary.md` 的单据类型码一节，由 CI 校验唯一。阶段 7 补分配八个码，另按 A-09 与 A-10 追加两个码。

最终归属阶段：登记文件归阶段 1，各码归其单据所在阶段。

确切标识符。全量类型码表如下，任何阶段不得新增未在此表登记的码。

| 阶段 | 类型码 |
|---|---|
| 4 | BGA、HRR |
| 5 | CUST、SUPP、MATL、PROD、PRLS、MDCR、MDIB、MDEX |
| 6 | CT、SO、SR、DC |
| 7 | PR、PO、GR、RJ、PRT、PAYR、DN、SIU |
| 9 | OBB、GV、PCR、YEC |
| 10 | INVA、SINV、IRVS、RCPT、PAYM、RFND、CDRV、OBST、PINV |
| 11 | RT |
| 12 | EQ、CPL、WO、PRJ、PT |

DC 为交付确认单（A-09），PINV 为进项发票（A-10）。CI 校验项名固定为 `xtask configdoc --check-doc-type-codes`，判据为该表与 `ep-platform-sequence` 的常量表逐项一致且无重复。

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

另有两条跨文件的机械改写不再逐文件重复列出。其一，基线第 10.3 节已把工作单元内的写入次序定死为审计末位，各阶段计划的用例表与事务次序段中凡把审计写在 Outbox、站内通知或任何其他数据库写入之前的，一律调整为审计末位，涉及 03、04、05、06、07、09、11 七份计划；阶段 3 计划第 1665 行的澄清一与该条一并标为已回写基线，第 1469 行的取号、写审计、写 Outbox、写站内通知改为取号、写 Outbox、写站内通知、写审计，阶段 9 计划第 593 行按 account_id 升序更新余额的防死锁论证保留不动。其二，凡引用事件 `inventory.stock_value_adjusted.v1` 的一律按 B-09 改为 `inventory.stock_movement.value_adjusted.v1`。

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

落点：文首范围说明（删去不建设许可与配置发布两项）；第 3.1 节交付物清单（追加 ep-platform-license、ep-adapter-search、ep-platform-release 最小通道、ConfigItemApplier 端口，编号为第 18 至 21 项，即第 18 项全文检索、第 19 项 ConfigItemApplier 端口属 3a 段、第 20 项最小配置发布通道、第 21 项模块许可本体，并按 3a 与 3b 分段标注）；第 55 行的 `config_item_apply_logs` 改为 `config_release_steps`；第 770 与 1150 行的发布状态取值按 A-27 删去 PENDING_REVIEW 并补 REJECTED；本阶段不承担 `MigrationWindowGuard` 的再导出，B-03 对本文件无落点；第 3.2 节 crate 表；第 3.0 节判定三与附件收敛任务的措辞（不使用 recon）；第 3.9 节退出条件与自检项命名；第 3.12 节偏离项；第 3.13 节依赖清单（依赖二至依赖十一逐条按本裁定改写或删除）；第 3.3.1 节迁移清单在第 32 号之后按 B-02 追加第 33 号 `V2026110209xx__platform_msg_backfill_append_only_registry.sql`，属 3b 段，目录取 `db/migrations/platform_msg/`，文件内先插三行登记再依次挂接触发器；第 1536 行第 7.2 章一行的强制手段与第 9 节退出条件按 B-02 改写；第 609 行按 A-28 把 `token_ciphertext` 改名为 `token_enc` 并补 `token_key_ref text`。

### 04-identity-authz.md

改动缺口：A-02、A-03、A-04、A-19、C-05、C-06、C-24、C-25、C-28。

落点：第 3 节表清单（删除 platform_authz.sensitive_field_registry，第 150 行外键目标写死）；第 4.1 节 SecurityContext（改为引用阶段 1 冻结）；第 4 节新增三个 AUTHZ_ applier；第 5 节 API 契约；第 6 节 Outbox 与第 523 行受理前提口径；第 8 节测试计划的 rls_matrix 分工与 32 组矩阵；第 9 节退出条件（自检项改名、applier 一条、界面一条、能力域常量一条、MasterReferenceCounter 不适用）；第 144 行按 C-06 保留“该表不设 approved_by 与 approved_at 两列”并把批准留痕改为由 release_ref 承载；第 451 行按 C-06 把 `GET /api/v1/platform/sensitive-fields` 改为由阶段 2 交付、本阶段只调用不注册；能力域码常量落 `crates/platform/authz/src/capability.rs`；第 11 节末尾删去阻塞判定。

### 05-master-data.md

改动缺口：A-08、A-13、A-14、A-15、A-18、A-20、A-23、A-28、B-10、C-09、C-10、C-11。

落点：第 2 节 crate 表（ep-contract-crm 改为 Customer360SectionProvider，ep-adapter-doc 改为本阶段交付本体）；第 3 节迁移编号表（追加 sensitive_field_registry backfill 与 dataset views 两个文件）；第 4 节导入导出算法（三个 doc 端口）、分类项去掉税率一类、探针与计数器的实现方清单、可引用性判定；第 5 节 API 契约（/overview 改 /customer-360）；第 9 节退出条件（新增界面、数据集视图、能力域常量、税率桩撤销时点四条，另按 A-28 把敏感字段登记一条写实为四行且 bank_account_no 两行 is_field_encrypted 为真、bank_name 两行为假、db/checks/11 返回零行）；第 3 节第 25 号迁移按 A-28 的四行逐列取值改写；第 205 与 209 行的两张 profiles 表按 A-28 删去明文列 bank_account_no，新增 bank_account_no_enc bytea 与 bank_account_no_key_ref text 与 bank_account_no_tail text，保留 bank_account_no_bidx bytea 与明文列 bank_name text；第 12 节未决事项中 U-A-12 保持待决，待决范围写实为开户银行是否同列、三场景脱敏形态、导出是否触发重新认证三问，并写明银行账号的纳入与字段级加密按规格第 7.8 章强制落地、不在待决范围内。

### 06-contract-sales.md

改动缺口：A-09、A-14、A-15、A-16、A-17、A-18、A-20、A-21、A-23、A-25、B-07、C-11、C-14、C-16、C-17、C-19、C-20、C-26。

落点：第 1 节交付物清单（事件由 14 增为 18，删去错误码总数，追加交付确认单两表与 ep-adapter-esign 两套契约测试文件名）；第 2 节 crate 表（追加 ContractDerivationPlanQuery、ContractPaymentScheduleQuery、SalesReturnCommandPort，删去 ProjectTaskDerivationPort 与 ReceivablePlanPort）；第 3 节数据库变更（追加交付确认单两个迁移文件、把两处逻辑引用改真实外键、追加数据集视图一个文件；按 A-21 不再追加 posting_trigger backfill 文件，该两行登记由阶段 9a 的种子迁移写入）；第 4 节算法（新增交付确认三腿次序、退货前置校验端口改名、派生计划）；第 5 节 API 契约（新增四个交付确认端点）；第 6 节 Outbox 事件表（三个终态事件）；第 8 节测试计划；第 9 节退出条件（新增数据集视图、界面、能力域常量、探针与计数器、类型码 DC 五条，合计由十四条增为十九条，条数以本节实际编号为准）；第 10.2 节 PRD 节映射表末尾按 A-09 追加一行，说明交付确认在 PRD 无承载节、属附录乙 U-C-01、本阶段依据规格第 8 章第 8 步与第 5.2 章事件-分录表实现；第 11.3 节未决事项表按 A-09 增补 U-C-01 与 U-C-02 两行并逐行给出临时取值与切换代价；第 11 节风险（删去第 772 行整条，新增空实现替换清单）。

### 07-procurement-portal.md

改动缺口：A-01、A-06、A-10、A-11、A-15、A-18、A-20、A-21、A-23、B-07、B-08、B-10、C-10、C-12、C-13、C-15、C-17、C-18、C-26。

落点：第 0 节范围（补一句进项发票台账归阶段 10）；第 3.2.3 节整节删除并顺延迁移序号；第 3.2.11 节删去单价列；第 3.2.14 与 3.2.17 节的逻辑引用目标写死；第 3 节按 A-21 撤销第 24 号 posting_trigger backfill 文件，该编号由 B-02 追加的 append_only_registry backfill 文件占用并登记 procure.goods_receipt_line_costings 一行，其后编号不变，第 433、895、1005 三行的迁移文件总数三十一保持不变；各单据表 doc_no 行补类型码；第 4 节算法（端口名全部改写、不自行取价一句）；第 8.6 节对账语句登记改为实现 ReconCheck；第 9 节退出条件（新增界面、能力域常量、计数器与历史成交、GRNI 子账查询、类型码八个共五条）；第 11 节假设 A2 与 A3 改写。

### 08-inventory-costing.md

改动缺口：A-01、A-06、A-09（不建表）、A-12、A-13、A-15、A-18、A-20、A-21（零行）、A-23、B-02、B-08、B-09、C-12、C-13、C-18。

落点：第 0 节三条硬边界（补一句交付确认单由阶段 6 建立，本阶段只提供库存腿）；第 1 节交付物清单 D1 由四个 trait 改五个、第 31 行删去不交付界面一句；第 3 节追加 append_only_registry backfill 与 dataset view 两个迁移文件，其中 append_only_registry 按 B-02 登记五行且 mode 一律取 APPEND_ONLY、mutable_columns 取空数组；第 115 与 443 行按 A-13 把索引名改为 ix_stock_qty_entries_legal_entity_id_material_id 并删去命名例外说明；第 5 节之后新增一小节列出五个 trait 的完整签名；第 6.1 节事务句柄写实为 `&mut dyn Tx`；第 6.4 节补一句不登记 posting_trigger 行、并写明 stock_value_adjusted 的消费者名；第 9 节退出条件（新增界面、数据集视图、能力域常量、MaterialUsageProbe、ReferenceCounter、GRNI 之外的存货子账查询六条）；第 11.1 节 R2 删去总账未确认一句。

### 09-ledger-period.md

改动缺口：A-01、A-06、A-09（凭证腿）、A-18、A-20、A-21、A-23、A-24、B-02、C-13、C-28。

落点：第 9.1 节交付物清单（追加 ep-platform-recon 本体与三张表）；第 9.3 节数据库变更（追加 recon 三表与 append_only_registry backfill，后者按 B-02 登记 ledger.vouchers、ledger.voucher_lines 与 platform_core.recon_runs 三行）；第 99 与 101 行按 A-21 把第 14 号种子迁移改为一次写全 13 行并直接填入 event_type 与 registered_by_module；第 9.3.11 节追加 PostingTriggerRegistry 接口；第 9.3.12 节 v_pending_posting_backlog 的口径句子按 C-28 逐字改写；第 9.4.3 节补一句 ledger 不自行取价；第 9.5.9 节把事务句柄与快照上下文类型写死；第 9 节退出条件（新增数据集视图、界面、能力域常量、recon 本体四条）；第 9.3.11 节按 A-21 把 PostingTriggerRegistry 的方法改为 assert_registered 只读断言并写全签名、错误码与不写入语义，删去全部幂等 upsert 措辞；第 78 行的 ep-app-ledger 依赖枚举按 B-08 补入 ep-contract-finance 并注明只用于 9b 段、按阶段 11 的成本与收入捕获调用点补入 ep-contract-costing 并注明 CI 的 cargo metadata 断言清单由阶段 11 同批更新；第 9 与 438 与 949 三行的子账取数按 B-08 改为经 ReconciliationItemQuery，并写明十项中的八项取自阶段 10 自有表；9b 段第 9.8.4 节新增 testkit/scenarios/golden_loop_14_steps.rs 作为黄金业务闭环十四步整体端到端验收的唯一落点，覆盖规格第 8 章第 1 至 14 步与第 17.2 章十五类必测分支，第 9.9 节退出条件追加该用例在 ep-datagen 默认 scale 上一次跑通一条。

### 10-ar-ap-invoice.md

改动缺口：A-09（过渡科目腿）、A-10、A-11、A-15、A-18、A-20、A-21、A-23、A-24、B-02、B-04、B-08、C-08、C-11、C-14、C-15、C-16、C-26、C-28。

落点：第 0.1 节按 C-28 改写；第 3.1 节追加 invoice.purchase_invoices 与 invoice.purchase_invoice_lines 两表；第 3.2.1 节 aging_bucket_definitions 标注为临时；第 3 节追加期初导入、税率迁移、append_only backfill、dataset views 四个迁移文件，其中 append_only backfill 按 B-02 只登记 finance.unbilled_ar_entries 与 finance.cash_ledger_entries 两行；按 A-21 删去 invoice 与 finance 两个目录的 posting_trigger backfill 文件；按 C-08 账龄的迁入与删表两个文件均由阶段 11 在 reporting 目录提供，本阶段不提供；第 4 节新增采购发票登记算法与三单匹配；第 5 节 API 契约（新增采购发票三个端点与期初导入一个端点）；第 7 节模块内契约表（追加 ReceiptInvoiceMatchQueryPort、PurchaseCreditNotePort、TaxRateOptionQuery、SubledgerBalanceProvider，改名 ReceivableExposureQuery，UnbilledArPort 使用方收窄）；第 8 节事件表追加 invoice.purchase_invoice.registered.v1；第 9 节退出条件（新增界面、数据集视图、能力域常量、计数器与历史成交、类型码 PINV、盲索引六条，另按 A-28 增加一条，即 platform_core.sensitive_field_registry 中存在 finance.cash_accounts.bank_account_no 一行且 is_field_encrypted 为真、db/checks/11 返回零行）；第 63 行的 F-17 一行按 A-28 改标为 F-17 与 U-A-12 并按三问写实待决范围；第 294 行按 A-28 把 bank_account_no_cipher 改名为 bank_account_no_enc 并补 bank_account_no_key_ref text 一行；第 305 行删去登记由 platform_authz 承载一句，改为在 db/migrations/finance/ 追加一支 sensitive_field_registry backfill 迁移；第 917 与 1199 行按 A-21 把 PostingTriggerRegistry::register 的幂等 upsert 改为 assert_registered 只读断言。

### 11-cost-metrics-reporting.md

改动缺口：A-06、A-08、A-18、A-19、A-20、A-23、B-09、C-08、C-25、C-26。

落点：第 1 节 D-11-04 自检项改名为 reporting-dataset-signature-matched；第 3.5 节数据集种子表按 A-18 的十三行改写（procure 改 invoice）；第 3.3 节在 db/migrations/reporting/ 追加账龄迁入与删表两个文件，删去 finance 目录一节与标记行守卫；第 4 节新增四个报表类 ConfigItemApplier；第 4 节新增 costing.stock_value_adjust 消费者；第 5 节 API 契约；第 9 节退出条件（新增四个 applier、界面、能力域常量、三个 ReconCheck、账龄迁移五条）；第 437 行的交付卡取数按 A-09 保持不读 sales.delivery_confirmation_lines 基表，实际交付日期经 clm_contract_delivery_milestones 与 sales_order_delivery_batches 两个数据集取得；第 712 与 751 两行收窄为闭环第 14 步的指标一致性用例，不含第 12 步与期间关账，整条链路的贯通验收由阶段 9b 的 testkit/scenarios/golden_loop_14_steps.rs 承担。

### 12-service-project-asset.md

改动缺口：A-15、A-16、A-17、A-18、A-20、A-23、B-06、C-09、C-19、C-26。

落点：第 2 节 crate 表（删去 EquipmentQuery，Customer360SectionProvider 由新增改为扩充）；第 3 节追加 project dataset view 一个迁移文件；第 4.7 节派生消费者名写死为 project.contract_derivation；第 4 节退货登记改用 SalesReturnCommandPort 与三个终态事件；第 9.3.6 节三个读取方改写；第 9 节退出条件（新增界面、数据集视图、能力域常量、ServiceReferenceCounter 四条）；第 11 节 R-01 与 R-02 的缓解措辞改写；第 709 与 763 两行按阶段 9 计划第 728 行的唯一落点约定改写，指向阶段 9b 的 `testkit/scenarios/golden_loop_14_steps.rs`，本阶段只交付闭环第 12 步的用例片段与断言，整条链路的串接验收顺延到阶段 9b，第 1 节 D-10 交付物行的“闭环第 12 步的端到端用例”同步收窄为“闭环第 12 步的用例片段”。

### 13-clients-lowcode.md

改动缺口：A-05（只留验收）、A-19、A-20、A-23、B-03、B-05、C-25、C-26。

落点：第 2 节 crate 表（ep-platform-release 由本阶段新增改为阶段 3b 已建、本阶段扩展）；第 3 节三张 config 表标注为阶段 3b 已建、本阶段只做列与状态扩展，本阶段新建的四张表沿用 config_release_steps 等实际表名，第 102 行删去与 config_item_apply_logs 的括注映射；状态扩展按 A-27 改为在阶段 3b 六态之上补五态、只放宽 CHECK 不改写既有行，第 368 行的迁移删去对 PENDING_REVIEW 行的 UPDATE；第 4.3 节 DDL 段第一步改为调用经 job-worker 装配注入的 MigrationWindowGuard 实例的 assert_open，去掉 ep_platform_release 前缀；本阶段不实现也不注册任何 ReconCheck；第 4.4 节能力域码表改为引用 foundation::CapabilityDomain、判定算法第 1 条改写；第 4.5 节写明两个实现类型名；第 4.6 节写明端口由阶段 3a 提供、本阶段实现六个 applier；第 7 节自检项按名字改写；第 9 节退出条件（新增许可停用再启用一条，删去业务界面相关表述）。

### 14-ops-backup-release.md

改动缺口：A-06、A-18（无）、A-22、A-25、A-26、B-01、B-11、C-22、C-27。

落点：第 0 节偏离二保留并补一句 degradation_windows 由阶段 2 建立；第 3 节表清单把 degradation_windows 标为扩展而非新建；新增 OpsDisposalService 实现 DisposalPort 一节；归档写出一节补审计证据目录只读一句；指标清单登记 ep_replication_crosscheck_age_seconds；发布门禁项清单追加 RG-CI-PROBE-ABSENT 与 RG-TOOLS-EXCLUDED 两行；认证清单追加电子签章真实沙箱或等效验证一条；第 9 节退出条件同步。

## F 类 结构与判据裁定

本节登记本轮新增的两条裁定，编号取 F 类，不计入前文 67 条缺口。按通则第一条，两条的权威落点均在技术基线与各阶段计划正文，本节只作登记，正文取自裁定原件，结论文字未作改写。

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

三、第 1.3 节允许项五条与禁止项七条一字不改，`xtask archcheck` 零改动。规则名 `adapter-no-peer-adapter` 不变，`FORBIDDEN_RULES` 七项不变，deps.rs:280-286 现有负样例逐字保留——其中 `assert!(!violated("adapter-no-peer-adapter", vec![pkg("ep-adapter-db-pg", &["ep-foundation"])]))` 恰好就是裁定后 db-pg 的真实依赖形态。本裁定不新增任何受限例外、不新增任何门禁规则、不新增任何白名单。

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

按 00-overview.md:64 通则第一条，00c 是登记表、不构成权威层。本裁定的权威落点是 00b-technical-baseline.md 第 1.3、1.4、12 节与 01-engineering-baseline.md 第 10 节；00c 只登记结论与标识符。

#### 1. 事实认定（逐条有书证）

其一，必要性一条在阶段 1 恒不可判定。判据数的是跨 crate 源码引用计数，而 30 个 contract/platform crate 按 01-engineering-baseline.md:39「lib.rs 仅含 pub use 与一条编译期断言注释，不留 todo!()」为骨架，计数恒为零。仓库实现已如实承认：xtask/src/archcheck/source.rs:236-241 返回 Err，文案为「必要性判据不可判定：{} 个 ep-contract-* 与 ep-platform-* 全为骨架，无任何引用可数。」

其二，同一实现含一处静默放行，比 F3 本身更危险。source.rs:243 是无条件的 Ok(Vec::new())：只要任意一个 contract 或 platform crate 出现一行代码，本判据就从「诚实的不可判定」翻转为「零断言的通过」，从不真正数引用。该翻转在阶段 1 内必定发生——01-engineering-baseline.md:41-42 已把 ep-platform-runtime 与 ep-platform-obs 列为实现而非骨架。

其三，稳定性一条零实现。xtask/src/archcheck/mod.rs:52-58 的 evaluate 未调用任何稳定性检查。即 00b:118 的「都可机检」「由 xtask archcheck 断言上述两条判据」对应两处落空，不止必要性一处。

其四，id::marker 有两套互斥口径，必须同批合一。00b:168 写「增删按第 1.3 节的两条准入判据由 xtask archcheck 断言，不设冻结清单」；而 00c-gap-ruling.md:120（A-01 配套裁定）、00-overview.md:183、01-engineering-baseline.md:163、01-engineering-baseline.md:584 四处一致写「清单固定 22 项，任何阶段不得增删」并定性为受限例外。crates/foundation/src/id/marker.rs:5 的模块注释站在前者一侧。

#### 2. 裁定

**第一段，判据本体。** 第六条仍是两条准入判据，撤回的只是关于判定手段的两句自述。必要性一条降为评审判据，明写不由任何工具判定；稳定性一条明写为一半机检一半评审。工具不得再声称在判它不能判的东西。

**第二段，机检面不留空洞，四条替身接盘。** 必要性要防的真实动作只有一个——有人往 ep-foundation 里塞东西。该动作只有三条物理路径，逐条堵死：

1. `foundation-no-business/no-internal-dep`（已实现，deps.rs:160-172）：foundation 不依赖工作区内任何 crate。真正的业务概念几乎必然要引用别的类型，这道墙在依赖图上即可判。
2. `foundation-frozen-items`（已实现，frozen.rs:15-23、75-92）：堵「加进已有冻结项」。**本裁定同时把它由计数断言升为按名逐项断言**——现实现 count_unit_structs 只数数量，把 SalesOrder 改名为 Foo 仍是 22 个、静默通过，而本裁定把 22 项定为冻结清单，必须按名守。已核对 marker.rs:7-28 的 22 个名字与 00c:120 逐字一致，改判据不会立刻变红。
3. `foundation-module-registry`（新增）：堵「另开一个 foundation 顶层模块」。比对 crates/foundation/src/lib.rs 的顶层 pub mod 行与基线第 1.4 节登记的模块清单，逐行相等。当前实际为 capability、error、id、module、port、principal、security 七个。
4. `foundation-no-single-owner`（新增，取自方案 2）：堵「在已有模块内新增业务形状」这条前三条都漏的路径。取 crates/foundation/src/ 下每个 pub mod/struct/enum/trait/type 声明的模块路径段与条目名切词，与基线第 1.2 节十五个模块码词元求交，非空即违反；词元表由 xtask 从 crates/foundation/src/module.rs 的 ModuleCode 枚举体做文本解析，不 use ep_foundation，避免给 xtask 加一条对 ep-foundation 的依赖边；例外面写死为 crates/foundation/src/id/marker.rs 一个路径。**扫描面刻意不含 pub const/pub static/pub fn**：11-cost-metrics-reporting.md:97 的 36 个错误码常量落在 crates/foundation/src/error/codes.rs，段名本来就是模块码，纳入即 36 处误报。这是一条必要条件而非充要条件，如实标注（pub struct OrderDto 这类不带模块码词元的仍能过）。

四条合起来是净增而非净减：改前第六条在仓库里的真实判定力只有第 1 条，第 2 条只数数量，另两条不存在。

**第三段，id::marker 定为冻结清单制。** 四比一取多数，且 00c:120 是 A-01 源头、frozen.rs:16 已按 22 项实现。00b:168 的「不设冻结清单」作废。这不是放宽是收紧：冻结清单不允许任何增删，判据制允许通过判据新增。副产品是必要性判据在 00b:118 现文里唯一点名的适用对象随之变为空集——降一个适用对象为空的判据，不产生新敞口。

**第四段，可复用通则（本裁定真正的可复用部分）。** 在 00b 第 12 节新增通则第六条「判据可判定性与不可判定登记」。落点选 00b 第 12 节而非 00-overview 第 1.4 节，理由是后者标题与首句写死「五条通则」（00-overview.md:45、:47）且 00c:9、:11 另有一份副本，加第六条要连改四处计数并连带十四份文件中的「通则第 N 条」引用。通则四句：

- 凡写成「由 X 断言/由 CI 强制/由 --check 判定」的判据，必须在同处写明被测输入的提供方与交付阶段。
- 被测输入的交付阶段晚于判据所在阶段的，只有三种合法处置：整条推迟、换被测输入已存在的可判定替身、降为评审判据并登记；不得留第四种。
- 不可判定既不得表达为通过也不得表达为违反：工具须单列输出并以专用退出码结束，CI 不得把该退出码当作通过；亦不得以「计数照旧」或「两个空集合比对」的形态退化为恒真。
- **判据重新生效的触发谓词必须由判定工具自身可观测，不得写成阶段号或任何需要人工翻牌的动作**（取自方案 3；这是本项目历史失效模式的直接解药——offsite-sink-requirements 在 01:216 声明整条推迟，而 03:1525、04:724、06:777 三处下游仍按「十四项/十三项/十项全部通过」写，延后条款没有向下游传播）。

配套在 00b 新增 12.1 节登记表，**分两段**：delegated 段登记已裁定不由工具执行的判据（永久，必须点名承接的替身规则）；undecidable 段登记当前无法执行的判据（临时，**条目数由 CI 断言只减不增、并由阶段 14 发布门禁 RG-NO-UNDECIDABLE 断言归零**，形制照搬 01-engineering-baseline.md:614 对 Pending 自检项的同款纪律）。两段与 archcheck 运行期输出逐行相等，多一条或少一条均判违反。这条比对本身完全可判定，是「不得静默放行」的机械承接方。

**第五段，阶段 1 的 archcheck 行为定死为三态。** 机检面（四条替身）全绿则退出码 0，这是诚实的，因为工具不再声称在判必要性；必要性以 delegated 一行显式打印「不由本工具判定，承接方：评审举证 + foundation-module-registry + foundation-no-single-owner」；登记表比对不符则退出码 1；仍存在真正不可判定项时退出码 3 且明写「不可判定不等于通过」。三条路径各有唯一且不同的退出码，任何一条不能被读成另一条。

**第六段，普查八条按通则归档。** 04:724、03:1525、06:777 三处的自检项计数（十三/十四/十）必须同一批改完，否则会留下三套计数口径；09:798、09:737、10:1219 三处的「跑阶段 11 的自检项」整条推迟到阶段 11，其前半的列签名静态比对可判、保留；01:515、01:506 两条按「整条推迟」或「换判据」二择一，不得留恒真断言。

## 附录丙　阶段 1 实测引出的同类缺陷登记

本附录登记裁定 F-01 与 F-03 落地过程中，由三次同类缺陷普查查出的 22 条。
三条已裁定的（F-01 的 PgTx 声明位、F-02 的 adapter 互依、F-03 的必要性判据）不重复登记。

**本附录只登记，不构成裁定。** 每条都带 文件：行号 与逐字原文，可直接进入下一轮裁定。
登记而不修的理由：修这 22 条是另一轮工程，其中多条（如 G-01 的子账余额提供者、
H-01 的 platform 成环）改动面与 F-01 相当，混进本批会使本批无法验证。

| 编号 | 类别 | 严重度 | 落点 | 缺陷 |
|---|---|---|---|---|
| G-01 | 孤儿规则类 | blocking | `10-ar-ap-invoice.md:519` | B-08 子账余额提供者：trait 在 ep-contract-finance、类型在 ep-app-inventory/ep-app-procure、impl 被要求落在阶段 10 的 crate — 与 F1 同类且无任何合法落点 |
| G-02 | 孤儿规则类 | blocking | `01-engineering-baseline.md:56` | F1 原句在阶段 1 计划复述：PgUnitOfWork/PgTx 声明位在 ep-adapter-db、实现体落在 ep-adapter-db-pg |
| G-03 | 孤儿规则类 | blocking | `02-data-foundation.md:407` | F1 原句在阶段 2 计划正文复述：同一声明位/实现位分离 |
| G-04 | 孤儿规则类 | blocking | `02-data-foundation.md:51` | F1 原句在阶段 2 crate 职责表复述：ep-adapter-db 承载「实现声明位」 |
| G-05 | 孤儿规则类 | blocking | `00c-gap-ruling.md:122` | F1 原句在裁定册 A-01 的「提供方要做什么」复述，且与同一裁定的结论句自相矛盾 |
| G-06 | 孤儿规则类 | blocking | `00-overview.md:238` | C-03 的另一种措辞同样规定三 crate 分离：ep-foundation 定义、ep-adapter-db 提供实现骨架、ep-adapter-db-pg 提供实现 |
| H-01 | 依赖方向类 | blocking | `13-clients-lowcode.md:60` | ep-platform-meta 与 ep-platform-release 互为依赖，构成 Cargo 硬性循环，且触发 archcheck 的 platform-acyclic |
| H-02 | 依赖方向类 | blocking | `13-clients-lowcode.md:61` | ep-adapter-wasm 依赖 ep-adapter-ipc，与禁止项第五条互斥（与 F2 同构） |
| H-03 | 依赖方向类 | blocking | `03-platform-kernel.md:1177` | ep-platform-release 与 ep-platform-audit 直接取用 ep-adapter-kms，platform 反向依赖 adapter，与裁定 B-03 已作废的形态相同 |
| H-04 | 依赖方向类 | major | `01-engineering-baseline.md:577` | ep-platform-runtime 的 HTTP 中间件栈留 `ep_adapter_db::port::IdempotencyStore` 注入点，platform 反向依赖 adapter |
| H-05 | 依赖方向类 | major | `11-cost-metrics-reporting.md:623` | COSTING_INVENTORY_COGS_VS_STOCK_VALUE 由 ep-app-costing 实现却跨读 inventory schema，直接违反禁止项第七条 |
| H-06 | 依赖方向类 | major | `11-cost-metrics-reporting.md:22` | D-11-01 单方面把禁止项第七条重新界定为「只约束基表」，属阶段计划改写基线取值 |
| H-07 | 依赖方向类 | major | `14-ops-backup-release.md:73` | 归档与备份的 IPC 报文类型放进 ep-foundation，按禁止项第六条的必要性判据恒不可准入 |
| H-08 | 依赖方向类 | minor | `08-inventory-costing.md:51` | ep-app-inventory 的依赖清单遗漏 ep-contract-ledger，而同阶段的过账端口与子账总账勾稽都要用它 |
| H-09 | 依赖方向类 | minor | `03-platform-kernel.md:117` | ep-adapter-search 被阶段 3 声明「只依赖 ep-foundation」，却被阶段 5 与阶段 12 要求承载各模块档案的投影函数 |
| I-01 | 判据不可判定类 | blocking | `04-identity-authz.md:724` | 阶段 4 退出条件 2：--check 要求「十三个命名项全部通过」，其中三项分别由阶段 3b 与阶段 14 交付 |
| I-02 | 判据不可判定类 | blocking | `03-platform-kernel.md:1525` | 阶段 3 退出条件 4：--check 十四项「全部通过」且对 DEGRADED 非零退出，但其中 offsite-sink-requirements 由阶段 14 交付 |
| I-03 | 判据不可判定类 | blocking | `09-ledger-period.md:798` | 阶段 9a 退出条件 E-17：判据是「可由阶段 11 的 reporting-dataset-signature-matched 自检项校验通过」，该自检项由阶段 11 交付 |
| I-04 | 判据不可判定类 | blocking | `10-ar-ap-invoice.md:1219` | 阶段 10 退出条件 20：判据是「阶段 11 的 reporting-dataset-signature-matched 在三者上按降级口径校验通过」，阶段 10 早于阶段 11 |
| I-05 | 判据不可判定类 | major | `06-contract-sales.md:777` | 阶段 6 退出条件 3：要求基线第 7.3 节十项在 --check 上全部通过，同样含阶段 14 才交付的 offsite-sink-requirements |
| I-06 | 判据不可判定类 | major | `01-engineering-baseline.md:515` | 阶段 1 退出条件 23：xtask configdoc --check-doc-type-codes 要与 ep-platform-sequence 的常量表逐项比对，而该常量表由阶段 3a 交付 |
| I-07 | 判据不可判定类 | minor | `01-engineering-baseline.md:506` | 阶段 1 退出条件 14：要求为「SBOM 中不出现 ep-bench 与 ep-release-gate」配负样例，而这两个包由阶段 14 才创建 |


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

该 HTTP 中间件栈的落点由同一文件第 583 行写死在 ep-platform-runtime：「该 crate 只承载进程生命周期状态机、分层配置加载、`SelfCheckRegistry`、健康与就绪端点、HTTP 服务器与中间件栈骨架，以及以 trait 表达的 IPC 服务端接口……因此本 crate 只依赖 foundation 与其他 platform」。而 `IdempotencyStore` 的完整路径是 `ep_adapter_db::port::IdempotencyStore`（同文件第 331 行原文：「第二段是端口定义，`ep_adapter_db::port::IdempotencyStore` 及其 `try_begin` 与 `finish` 两个方法」；02-data-foundation.md:409 复述同一落点）。在 Rust 中，要在中间件栈里留一个以该 trait 为类型的注入点（字段、泛型约束或 `Arc<dyn IdempotencyStore>`），ep-platform-runtime 必须把 ep-adapter-db 列为依赖，第 583 行「只依赖 foundation 与其他 platform」当场不成立，也违反基线第 1.3 节允许项第二条加「其余一律禁止」。这是裁定 B-03 判过的同一条口子，只是那次落在 ep-platform-release、这次落在 ep-platform-runtime，没有对应裁定收口。附带一处同源矛盾：基线第 1.2 节（00b-technical-baseline.md:58）说 ep-platform-runtime 只放「以 trait 表达的服务器骨架。具体 HTTP 与 IPC 传输实现分别留在对应的 ep-adapter-*」，但第 1.2 节适配层清单十个 crate 里没有任何 HTTP adapter，而第 583 行又把「HTTP 服务器与中间件栈骨架」直接放进 runtime，两处口径不一。

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

