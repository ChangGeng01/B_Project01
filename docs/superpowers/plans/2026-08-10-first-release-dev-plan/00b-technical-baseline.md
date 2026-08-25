> **F-57 状态：`SUPERSEDED_DO_NOT_EXECUTE`。** 本文件只供提取未冲突的旧技术细节；不得作为共享强制基线。唯一执行入口是 [F-57 实施计划](../2026-08-24-f57-converged-program.md)。

## 0. 本基线的地位与使用方式

本文件是 14 个阶段技术计划的共享前提。凡本文件已给出取值的事项，各阶段计划直接引用，不得重新决定、不得给出第二套取值、不得写“由实现方决定”。凡本文件未覆盖的事项，阶段计划可自行决定，但必须在计划中显式标注为本阶段新增决定，并在阶段结束时回写本基线。

取值来源原分三类：规格照抄值、本基线技术值和历史上曾需业务拍板的值。后者已由 F-51 或更早裁定全部冻结；正文保留的编号只用于追溯，不表示仍待决定。开发者不得重新选值，必须采用现行正文及 F-50 至 F-56 的冻结口径。

冲突时的优先级为：F-50 至 F-56 在各自明示范围内优先，且同范围内较晚裁定优先；其次是总体规格，再次是 PRD，最后是本基线。本基线与上位裁定冲突的部分一律作废。F-55 已把本文件的八进程、六值 ClientKind、三管道、72 格和仅物理机承载旧句更新为九进程、八值 ClientKind、四管道、90 格与两个等价 carrier；F-56 再把许可四态、声明式模块包和 F-55 entitlement 的重叠面更新为终态，不得按历史段落恢复旧计数或旧许可载体。

## 1. Rust workspace 布局与依赖方向

### 1.1 工作区根布局

仓库为单一 Cargo workspace。Cargo workspace 成员路径固定如下。

```
/Cargo.toml                     workspace 根，唯一的 [workspace.dependencies]
/rust-toolchain.toml            工具链版本唯一来源，阶段 1 冻结，其后不得单独升级
/crates/foundation/             ep-foundation
/crates/platform/<cap>/         ep-platform-<cap>
/crates/contract/<module>/      ep-contract-<module>
/crates/domain/<module>/        ep-domain-<module>
/crates/application/<module>/   ep-app-<module>
/crates/adapter/<name>/         ep-adapter-<name>
/apps/<process>/                九个产品常驻二进制，crate 名即进程名
/db/migrations/<schema>/        每 schema 一个迁移目录
/testkit/                       ep-testkit，测试夹具与构造器
/datagen/                       ep-datagen，基准数据集生成器
/docs/                          规格、PRD、ADR、数据字典、错误码表、事件目录
/xtask/                         ep-xtask，结构门禁与文档校验工具，只在开发期运行，不进制品
/tools/<name>/                  工具 crate，tools/ep-migrate 随制品交付，tools/bench 与 tools/release-gate 按 B-11 排除出制品
```

非 workspace 成员的仓库顶层目录固定如下：`/db/bootstrap/` 数据库引导脚本，`/db/checks/` SQL 断言脚本，`/scripts/` 运维与校验脚本，`/deploy/` 服务注册脚本与服务宿主层读取的静态限额文件，`/clients/desktop/` 与 `/clients/mobile/` 四端客户端源码，`/clients/server-admin/` 独立静态管理 SPA，`/clients/ui/` 共用设计组件，`/tests/` 跨 crate 端到端与安全收容测试。以上两段合起来即全部顶层目录，新增顶层目录必须先改本节。

crate 命名前缀统一为 `ep-`，crate 目录名不带前缀，`Cargo.toml` 中的 `name` 带前缀。二进制 crate 不带前缀，名字与进程名、Windows 服务名一一对应；与资源单位不构成一一对应，九个二进制落在八个产品资源单位内，对应关系见第 2 节。edition 固定 2021。禁止 nightly，禁止在成员 crate 中写版本号，成员一律 `dep.workspace = true`。

### 1.2 crate 清单与职责

平台底座。

| crate | 一句话职责 |
|---|---|
| ep-foundation | 稳定通用类型：Id、Money、Quantity、UnitPrice、Rate、AccountingPeriodRef、SecurityContext、SecurityLevel、AppError、ErrorCode、DomainEvent 信封、Clock 与 IdGen 端口；另含 `port::tx` 的 Tx、SnapshotCtx、UnitOfWork、TxId、IsolationKind，`id::marker` 的 22 个跨模块引用标记类型，`principal` 的 SYSTEM_PRINCIPAL_ID 与 SYSTEM_DEVICE_ID，模块码枚举 ModuleCode，`capability` 的 CapabilityDomain 与 ActionClass，以及 `port::search`、`port::doc`、`port::db`、`port::kms` 与阶段 2 新增的 `port::secret` 五个端口模块。上述类型的签名与取值见第 1.4 节。 |
| ep-platform-tenancy | 集团、法人、组织、部门、岗位，以及安全上下文的建立与法人授权集合校验。 |
| ep-platform-identity | 本地账号目录、口令与 MFA、会话、设备登记、高风险操作的重新认证凭证。 |
| ep-platform-authz | RBAC 与 ABAC 判定、字段级与密级过滤、职责分离、审批授权判定。 |
| ep-platform-meta | 元数据、自定义对象与字段、索引与视图、在线 DDL 计划与影响分析。 |
| ep-platform-flow | 持久化流程引擎、任务、定时器、SLA、补偿、流程定义版本。 |
| ep-platform-audit | 审计事件模型、哈希链与分段签名、链验证工具。 |
| ep-platform-outbox | Outbox 写入与消费、幂等键、死信、重投、投递统计。 |
| ep-platform-sequence | 单据编号与档案编码的生成与占号。 |
| ep-platform-notify | 站内通知与移动推送的模板、订阅、送达状态。 |
| ep-platform-license | 模块许可、功能开关、能力注册、模块生命周期状态机。 |
| ep-platform-release | 配置发布包、差异审查、签名、发布与回退。 |
| ep-platform-file | 附件对象元数据、版本、上传流水线状态机、正文引用。 |
| ep-platform-recon | 内部对账与强制不变量校验的语句集、分批与快照口径、差异事项模型。 |
| ep-platform-obs | 日志字段约定、指标注册表、追踪上下文、运维中心台账模型。 |
| ep-platform-runtime | 进程生命周期状态机、分层配置加载、第 7.3 节的 `SelfCheckRegistry`、健康与就绪端点，以及以 trait 表达的服务器骨架。HTTP 服务端骨架直接构建在第三方库上，工作区内既无也不新增 HTTP 系 ep-adapter-*；IPC 的具体传输实现留在 ep-adapter-ipc。两者一律由 apps 在 `apps/<proc>/src/wiring/` 目录下注入，本 crate 不依赖任何 ep-adapter-*。 |
| ep-platform-mcp | F-55 双向 MCP 的签名 manifest 校验、短期人类 grant、binding 解析、逐次授权、审计摘要与六方法闭集；不承载业务 command/query 实现。 |

本表是平台底座 crate 的现状记录，不是冻结清单。archcheck 不再对 crate 清单逐项比对，阶段 1 退出条件第 2 条中的该项断言撤销；crate 的增删走普通提交，只受第 1.3 节依赖方向七条禁止项约束，该七条仍由 archcheck 逐条断言并配负样例。其中第六条的机检面为 foundation-no-business（依赖边一侧，即 foundation 不依赖工作区内任何 crate）、foundation-frozen-items、foundation-marker-shape、foundation-module-registry、foundation-no-single-owner 五条规则，各配负样例；其必要性一条按第 12 节通则第六条降为评审判据并已登记入第 12.1 节，不计入本句的逐条断言。

契约、领域、应用三层按业务模块各一个 crate，模块码固定为下表 15 个，任何阶段不得新增模块码。

| 模块码 | 覆盖范围 |
|---|---|
| mdm | 客户、供应商、物料、产品、单位、地点、组织主数据。 |
| crm | 客户档案扩展与客户 360 视图。 |
| cpq | 价目表、价格权限校验、折扣及其审批。 |
| clm | 合同、模板、条款、修订、续签、履约、到期提醒、签章编排。 |
| sales | 销售订单与变更、分批交付、退换货、直运、寄售标记、订阅与租赁、客户信用额度校验。 |
| procure | 采购需求、采购订单与分批订货、收货、采购退货、付款申请、供应商档案与资质。 |
| inventory | 仓库、库存数量账、库存金额账、批次与序列号、移动加权平均计价。 |
| costing | 成本归集与销货成本结转、未分摊差异。 |
| project | 项目、项目任务、交付节点。 |
| service | 售后工单、客户投诉、退换修、设备台账与保修。 |
| finance | 应收应付台账、收付款与退款登记、核销、预收预付、银行与现金账户。 |
| ledger | 会计科目表、事件到分录映射、凭证、科目余额、会计期间与关账、年度损益结转。 |
| invoice | 销项与进项发票台账、申请与开具登记、作废与红字冲销、税额。 |
| portal | 供应商门户的受控能力用例与脱敏投影。 |
| reporting | 报表定义、仪表盘、经营驾驶舱预置指标与数据集、像素级打印模板。 |

各层职责：`ep-contract-<module>` 只放该模块对外公开的命令、查询、事件类型与 DTO，以及供其他模块调用的 trait；`ep-domain-<module>` 放聚合、值对象、领域服务、领域规则与业务端口 trait；`ep-app-<module>` 放用例、事务边界、授权调用、Outbox 与审计写入、跨模块协调。F-55 新增的 `ep-contract-ai` 与 `ep-contract-mcp` 是两个非业务模块的 transport/port 契约例外，不新增 `ModuleCode`、domain crate 或业务 schema；前者由 `ep-app-reporting` 使用，后者由 `ep-platform-mcp` 与 transport adapter 使用。

适配层。

| crate | 一句话职责 |
|---|---|
| ep-adapter-db-pg | 首版唯一交付并认证的 PostgreSQL 16 实现，含 RLS 会话变量注入与清除、流复制以外的全部 SQL。`PgTx`、`PgSnapshot`、`PgUnitOfWork`、`PgPoolFactory`、`PgMigrationWindowGuard` 与 `PgReadOnlyTx` 的声明与实现，`PoolKind`、`SessionContext`、`RetryPolicy`、`ConnectionBudget` 四个连接模型类型的定义与取值，`ScopePredicateRenderer`，以及公共能力基线到 PostgreSQL 类型与索引 DDL 的映射。 |
| ep-adapter-file | 本机文件存储实现，只提供写入新对象与读取，不提供覆盖与原地删除接口。 |
| ep-adapter-kms | 内置 KMS 与客户 HSM 两种载体的实现，含数据 KMS、字段级密钥与系统机密解封；`KmsBackend` 及其调用词汇定义在 `ep_foundation::port::kms`，`SecretUnsealer` 定义在独立的 `ep_foundation::port::secret`，均不在本 crate；只读 `SecretProvider/KmsSecretProvider` 落 `ep-platform-runtime`。数据 KMS common master 与六个 recipient 的 system-secret KEK 严格分离。 |
| ep-adapter-queue | 内置轻量队列，构建在 Outbox 表之上，不引入外部消息中间件。 |
| ep-adapter-search | 内置 Rust 全文检索索引的写入与查询，按法人分区。 |
| ep-adapter-doc | Excel 导入导出、文档模板套用、PDF 渲染与批注、像素级打印排版。 |
| ep-adapter-esign | 电子签章外部出口，首版唯一的外部系统适配。 |
| ep-adapter-wasm | 受限 WASM 计算与服务端插件的宿主接口。 |
| ep-adapter-ipc | 进程间接口的客户端与服务端，Windows 命名管道承载。 |
| ep-adapter-local-ai | F-55 签名模型包复验、只读加载、受约束解码与本地推理；禁止 DB、HTTP、网络与文件写 API。 |

### 1.3 依赖方向与禁止项

允许的依赖方向只有以下几条，其余一律禁止。

- ep-foundation 不依赖工作区内任何 crate。
- ep-platform-* 只可依赖 ep-foundation 与其他 ep-platform-*，且 platform 内部不得成环。
- ep-contract-<m> 只可依赖 ep-foundation。
- ep-domain-<m> 只可依赖 ep-foundation 与 ep-contract-<m>，即自身模块的契约。
- ep-app-<m> 可依赖 ep-foundation、ep-platform-*、ep-domain-<m>、ep-contract-<任意模块>。
- ep-adapter-* 可依赖 ep-foundation、ep-contract-*、以及 domain 与 platform 中的端口 trait，不得依赖 application。
- apps/* 可依赖全部，负责装配。

禁止的依赖方向逐条列出，评审时逐条核对。

- 禁止 ep-domain-A 依赖 ep-domain-B、ep-app-B 或 ep-contract-B。跨模块只走 ep-app-A 依赖 ep-contract-B。
- 禁止 ep-app-A 依赖 ep-app-B。模块间同步调用只能通过 ep-contract-B 中的 trait，实现在 apps 装配时注入。
- 禁止 ep-domain-* 与 ep-contract-* 依赖任何 adapter、sqlx、reqwest、tokio 的 IO 模块、std 的文件与网络 API。
- 禁止 ep-platform-* 依赖任何 domain 或 application。
- 禁止 adapter 之间互相依赖，共用逻辑下沉到 ep-foundation。
- 禁止 ep-foundation 承载业务概念。准入判据两条。必要性：被两个及以上 `ep-contract-*` 引用，或被 `ep-platform-*` 引用——该条为评审判据，不由任何工具判定，理由、举证格式与登记见第 12 节通则第六条与第 12.1 节。稳定性：不得承载任何会随业务政策变化的取值集合或规则方法，只允许类型身份与量纲原语——该条一半机检一半评审，机检面为 `xtask archcheck` 的 foundation-frozen-items，即冻结项的名字与项数不得随业务政策增删，其余属评审面。`crates/foundation/src/id/marker.rs` 是本条的唯一受限例外：其中的零大小标记类型无字段、无方法、无 trait 实现，只承载类型身份，供 `Id<T>` 在契约层表达跨模块引用；按裁定 A-01 冻结清单 22 项、任何阶段不得增删，不适用上述两条准入判据，其项数与名字由 `xtask archcheck` 的 foundation-frozen-items 按名逐项断言，改名与增删同样报错；上述无字段、无方法、无 trait 实现的形态由 foundation-marker-shape 断言。本条落在 archcheck 上的机检面为 foundation-no-business（依赖边一侧，即 foundation 不依赖工作区内任何 crate）、foundation-frozen-items、foundation-marker-shape、foundation-module-registry、foundation-no-single-owner 五条规则，必要性一条的举证格式与登记见第 12 节通则第六条与第 12.1 节。跨模块共享的业务形状不进 foundation，定义在拥有它的模块的 `ep-contract-*` 里作为 DTO，由可依赖任意模块契约的 `ep-app-*` 消费。
- 禁止跨模块直接读写业务表。跨模块取数只有两条通道，此外一律禁止。通道一，经被调方 `ep-contract-<m>` 中的端口 trait 取数，实现落在被调方的 `ep-app-<m>`，由 apps 在 `apps/core-server/src/wiring/` 与 `apps/job-worker/src/wiring/` 目录下注入；通道二，经被调方登记的受治理只读视图取数，视图名一律 `v_` 前缀，常规报表与经营看板一侧的取数连接取第 3.1 节的 `ep_analyst_ro` 只读角色。`ep-adapter-db-pg` 中的仓储实现按 schema 分文件，一个仓储只访问自己模块的 schema。本条的机检面为 `xtask archcheck` 的 db-pg-one-schema-per-file 一条规则并配负样例：按第 3.1 节登记的 24 个 schema 名逐名判定，不用前缀启发式，只在双引号字面量区间内取 `<schema>.<object>`，文件内出现自身 schema 之外的非 `v_` 对象即违反；`crates/adapter/db-pg/src` 下不落在任何 schema 目录内的文件，出现任何登记 schema 的对象同样违反。通道二中取数连接角色这一维不在该规则的判定面内——它判的是源码里的对象引用，判不出运行期连接取的是哪个角色；该维按第 12 节通则第六条降为评审判据并登记入第 12.1 节 delegated 段，承接方为阶段 11 的 reporting-dataset-signature-matched 启动自检加评审举证。

依赖方向由 CI 强制：`cargo deny` 检查许可与重复依赖，另在 CI 中运行一段基于 `cargo metadata` 的自检脚本，把上述禁止项表达为断言，违反即构建失败。本节允许项第二条「ep-platform-* 只可依赖 ep-foundation 与其他 ep-platform-*，且 platform 内部不得成环」的机检面为 `xtask archcheck` 的 platform-acyclic 与 platform-no-adapter 两条规则，各配负样例：前者判 platform 内部的依赖成环，后者判任一 ep-platform-* 依赖任一 ep-adapter-*。这两条落在允许项一侧，不并入本节禁止项七条，禁止项仍为七条、一字不改。第六条的必要性一条不在 `cargo metadata` 的依赖边判定面内——它数的是 foundation 模块被几个 crate 引用，属源码级判定；该条按第 12 节通则第六条降为评审判据并登记入第 12.1 节，其机检承接方为本节第六条点名的五条规则。

各阶段计划中的 crate 依赖枚举一律是该阶段结束时的快照，后续阶段可在本节允许项内增边，只需在该阶段的 crate 改动表写出增量并在提交说明中给出使用位，不回改先前阶段的枚举。据此，「按 crate 逐项比对期望依赖清单」这一形态整体撤销，其承接方是 `xtask archcheck` 的层位判定；「按 `cargo metadata` 断言某进程不链接某 crate」这一形态保留，被测输入是 `cargo metadata` 的输出，提供方为阶段 1，判据可判定。凡在 `cargo metadata` 之外另需调用图分析的断言，本基线不认其为已可判定：阶段 10 计划中 `finance.cash_ledger_entries` 只被四个用例的仓储写入一项，其调用图一侧的判据由阶段 10 同批给出，给不出即按第 12 节通则第六条的三档处置之一登记。

### 1.4 ep-foundation 冻结的跨阶段共享类型

本节各项由阶段 1 一次性冻结，签名与取值全阶段唯一。各阶段直接引用，不得改动签名、不得在阶段内另立同名类型、不得给出第二套取值。

ep-foundation 的顶层模块固定为下表七项。本表即 `xtask archcheck` 的 foundation-module-registry 规则的比对对象，与 `crates/foundation/src/lib.rs` 中的 `pub mod` 声明逐行相等，多一个少一个都判违反；新增或删除顶层模块必须先改本表并走基线修订，不得只改代码。模块内部的文件划分不在本表的判定面内。

| 顶层模块 | 落点 | 本节在该模块下冻结的内容 |
|---|---|---|
| capability | `crates/foundation/src/capability.rs` | `CapabilityDomain` 18 项与 `ActionClass` 5 项。 |
| error | `crates/foundation/src/error.rs` | `AppError` 与 `ErrorCode`；`AppError` 的字段构成见第 10.2 节，错误码与分类取值见第 5.5 节。 |
| id | `crates/foundation/src/id/` 下 `mod.rs` 与 `marker.rs` | `Id<T>`，以及冻结 22 项的跨模块引用标记类型。 |
| module | `crates/foundation/src/module.rs` | `ModuleCode`，取值与第 1.2 节的 15 个模块码一一对应。 |
| port | `crates/foundation/src/port/` 下 `tx.rs` 与 `db.rs`、`doc.rs`、`kms.rs`、`search.rs`、`secret.rs` | 事务与快照抽象；阶段 1 建四个空端口文件，阶段 2 在不增加 foundation 顶层模块的前提下新增独立 `secret.rs`，承载 `SecretUnsealer` 与强类型 secret/bootstrap ref。 |
| principal | `crates/foundation/src/principal.rs` | `SYSTEM_PRINCIPAL_ID` 与 `SYSTEM_DEVICE_ID`。 |
| security | `crates/foundation/src/security/` 下 `context.rs` 与 `level.rs` | `SecurityContext` 的 20 项字段、七个非通用字段类型与四个配套枚举，以及 `SecurityLevel`，其取值见第 4 节公共列 `security_level`。 |

事务与快照抽象位于 `crates/foundation/src/port/tx.rs`。契约层的跨模块方法签名一律写 `&mut dyn Tx`。

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

配套纪律四条。跨 crate 取具体句柄的唯一写法是 `tx.as_any_mut().downcast_mut::<PgTx>()`，该 downcast 只允许出现在 `crates/adapter/db-pg/` 内，由 `xtask archcheck` 的 downcast-pgtx-confined 规则在 `crates`、`apps`、`testkit`、`datagen`、`tools` 五个目录下逐文件扫描，断言这五个目录中 `crates/adapter/db-pg/` 之外的任何文件不出现 `downcast_mut::<PgTx>`；`xtask/` 自身因承载该规则的检索式常量而排除在扫描面外，仓库顶层的散落文件同样不在扫描面内，新增扫描目录须先改本节。UnitOfWork 不带池参数，一个实例在装配时绑定一个池，与第 10.3 节示例的两参数形态一致。application crate 对 UnitOfWork 取泛型参数 `U: UnitOfWork` 而不是 trait 对象，理由是该 trait 含泛型方法不满足对象安全。实现 `ep_foundation::port::*` 各模块中任一 trait 的具体类型，其声明位与实现位一律同处一个 crate，不得分离；`PgUnitOfWork` 与 `PgTx` 一律声明并实现在 ep-adapter-db-pg，`BuiltinKmsBackend` 与 `HsmKmsBackend` 一律声明并实现在 ep-adapter-kms。工作区内不存在名为 ep-adapter-db 的 crate。

跨模块引用的标记类型位于 `crates/foundation/src/id/marker.rs`，清单固定 22 项，任何阶段不得增删，由 `xtask archcheck` 的 foundation-frozen-items 规则按名逐项断言，其无字段、无方法、无 trait 实现的形态由 foundation-marker-shape 规则断言；本清单不适用第 1.3 节的两条准入判据，新增标记类型必须先改本节并走基线修订。清单如下：LegalEntity、UserAccount、Session、Department、Position、Project、Customer、Supplier、Material、Product、Warehouse、Contract、ContractLine、SalesOrder、SalesOrderLine、DeliveryConfirmation、DeliveryConfirmationLine、PurchaseOrder、GoodsReceiptLine、PurchaseInvoice、PurchaseInvoiceLine、AccountingPeriod。

系统主体常量位于 `crates/foundation/src/principal.rs`。

```rust
pub const SYSTEM_PRINCIPAL_ID: uuid::Uuid =
    uuid::uuid!("00000000-0000-7000-8000-000000000001");
pub const SYSTEM_SESSION_ID: uuid::Uuid =
    uuid::uuid!("00000000-0000-7000-8000-000000000002");
pub const SYSTEM_DEVICE_ID: &str = "SYSTEM";
```

取值选用全零前缀加版本位 7 与变体位 8 的保留形态，理由是它符合 UUIDv7 的版本与变体校验，同时不可能与 IdGen 生成的任何值碰撞。凡在种子迁移或系统上下文写 created_by 与 updated_by 的，一律引用该常量，不得另取字面量。

安全上下文位于 `crates/foundation/src/security/context.rs`，字段顺序即下表顺序，共 20 项，不得增删改名。

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

`SecurityContext::system(le, request, trace, purpose)` 的 20 字段映射不留默认分支：`user_id=SYSTEM_PRINCIPAL_ID`、`account_kind=System`、`session_id=SYSTEM_SESSION_ID`、`legal_entity_id=le`、`device_id=SYSTEM_DEVICE_ID`、`client=ClientKind::Ops`、`clearance_level=序列化数值 10 的最低 SecurityLevel`、`roles=[]`、`duty_classes=[]`、`department_scope=DepartmentScope::Explicit([])`、`position_ids=[]`、`project_scope=[]`、`customer_scope=[]`、`record_shares=[]`、`data_scope_tags=[]`、`snapshot_version=0`、`is_breakglass=false`、`request_id=request`、`trace_id=trace`、`system_purpose=Some(purpose)`。`SYSTEM_PRINCIPAL_ID` 对应阶段 4 种子的唯一 SYSTEM account 行；`SYSTEM_SESSION_ID` 只是非空上下文哨兵，`platform_core.sessions` 不建对应行，不得用于 reauth、续期或人类会话查询。人类 `AuthorizationService`/会话/重认证流水线遇 `AccountKind::System` 一律失败关闭；只有编译期封闭 executor/port 可按 `SystemPurpose` 接受，且仍受单法人 RLS。审计在 `account_kind=System` 时写 `audit_events.client='system'`；`system` 不是 ClientKind 变体。负例覆盖哨兵 session 查库/重认证、System 走人类授权、空范围被替换为 All、自填角色/密级/client 与跨法人调用，均返回 `PLATFORM.AUTHZ.OBJECT_FORBIDDEN` 且不取业务连接。

配套枚举同在 ep-foundation 冻结：`AccountKind { Human, System, Portal }`；`ClientKind { Win, Mac, Ios, Android, Portal, Ops, ServerAdmin, Mcp }`，序列化值固定为 `win|mac|ios|android|portal|ops|server_admin|mcp`；`DepartmentScope { All, Subtree(Id<Department>), Explicit(Arc<[Id<Department>]>) }`；`SystemPurpose { General, Reconciliation }`。普通员工 `X-Client` 可取前七值；`mcp` 只能由 `/mcp` grant middleware 固定，外部自填无效，并复用 grant 来源设备，不新增 `user_devices.client=mcp`。构造函数只有 `SecurityContext::human(..)` 与 `SecurityContext::system(legal_entity_id, request_id, trace_id, purpose)` 两个：前者把 `system_purpose` 固定为 `None`；后者用上面两个常量填 user_id 与 device_id、account_kind 取 System，并把 `system_purpose` 固定为 `Some(purpose)`。`AccountKind` 非 System 与 `system_purpose.is_some()`、或 `AccountKind::System` 与 `system_purpose.is_none()` 均为构造失败；不提供任何 with_ 前缀的变换方法。第 18 与第 19 两个字段的存在理由是第 3.8 节要求连接取用时写入 `app.request_id` 与 `app.trace_id` 两条会话变量，取数只能来自安全上下文；第 20 个字段把普通系统任务与免裁剪的内部对账任务做成不可混淆的类型事实。

`SystemPurpose::Reconciliation` 的构造面是静态封闭的：除枚举定义所在的 `crates/foundation/src/security/context.rs` 外，该变体只允许在 `crates/platform/recon/src/executor.rs` 出现，由 `xtask archcheck` 的 `reconciliation-context-confined` 规则扫描 `crates/`、`apps/`、`testkit/`、`datagen/`、`tools/` 并逐文件断言；测试必须经 `ReconExecutor` 驱动，不得在夹具中直造该变体。`ReconExecutor` 逐法人调用 `SecurityContext::system(..., SystemPurpose::Reconciliation)`，公开调用方只调用 `ReconExecutor::run`；每日对账与关账前校验的路由只装配在 `apps/job-worker/src/wiring/`，core-server、界面、API、报表、低代码与插件均无该入口。对账仓储在取连接前同时断言 `account_kind == AccountKind::System` 与 `system_purpose == Some(SystemPurpose::Reconciliation)`，不成立即返回 `PLATFORM.AUTHZ.OBJECT_FORBIDDEN`，不取连接、不写业务数据且不泄露对象存在性。普通系统任务一律传 `SystemPurpose::General`。不定义 `ReconContext`、不检查调用栈、不以 panic 充当授权判定。

上表中的七个非通用字段类型同在 `crates/foundation/src/security/context.rs` 冻结，任何阶段不得改名、改形态或另立第二处定义。`DeviceId(Arc<str>)`，取值为长度 1 至 64 的 `[A-Za-z0-9_-]`，必须能由 `&'static str` 无损构造，`SecurityContext::system` 即以 `SYSTEM_DEVICE_ID` 填该字段。`RoleCode(Arc<str>)`，取值为长度 1 至 64 的 `[A-Z0-9_]`，与 `platform_authz.roles.code` 逐字一致。`DutyClass { System, Data, Security, Audit, Key, Config }`，序列化取值依次为 SYSTEM、DATA、SECURITY、AUDIT、KEY、CONFIG，与 `platform_authz.roles.duty_class` 的六个字符串逐字一致；该列为空的业务角色不产生任何项，`Arc<[DutyClass]>` 允许为空数组，不设 None 变体，职责分离的两两互斥关系是种子规则行的内容，不进本枚举。`RecordShare { object_type: Arc<str>, object_id: uuid::Uuid, grant: RecordShareGrant }` 与 `RecordShareGrant { Read, Write }`，`object_type` 与第 6.1 节事件信封的 `aggregate_type` 同形，即 `<module>.<table>` 的小写下划线形态；本结构体只表达一条具体记录被显式共享给当前主体这一事实，不含任何判定语义，记录范围的编译结果与谓词类型留在 ep-platform-authz，不前移进本 crate。`DataScopeTag(Arc<str>)`，形态为 `<kind>:<value>`，kind 取 `[a-z0-9_-]`，value 取 `[A-Za-z0-9_-]`，总长上限 128；其 `Display` 与 serde 输出即为第 4 节公共列 `data_scope_tags text[]` 的元素形态与第 6.1 节事件信封 `data_scope_tags` 的元素形态，两处不得各自编解码。`RequestId(Arc<str>)`，取值为长度 8 至 64 的 `[A-Za-z0-9_-]`，服务端按第 5.6 节自生成时取 UUIDv7 的无连字符小写十六进制。`TraceId(Arc<str>)`，取值为 32 位小写十六进制，与 W3C trace-context 的 trace-id 同形。

模块码枚举 `ModuleCode` 按第 1.2 节的 15 个模块码冻结，取值为 Mdm、Crm、Cpq、Clm、Sales、Procure、Inventory、Costing、Project、Service、Finance、Ledger、Invoice、Portal、Reporting。

能力域码与动作类别位于 `crates/foundation/src/capability.rs`。

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

`CapabilityDomain` 的序列化取值逐一为阶段 13 计划第 4.4 节表中的 18 个能力域码字符串，顺序与该表序号一致。`ActionClass` 的五项与该节判定算法的 ViewOnly 分支配套，ViewOnly 只放行 Read。各阶段为每个用例声明常量的纪律见第 12 节。

四个端口模块的位置与补齐时点固定。`crates/foundation/src/port/db.rs` 由阶段 1 建空文件，阶段 2 按 C-07 与 B-03 补齐 `IdempotencyStore`、`IdempotencyScope`、`IdempotencyOutcome` 与 `MigrationWindowGuard`，并补齐规格第 7.4 章公共能力基线的字段类型与索引种类的能力描述，阶段 11 补齐只读事务端口 `ReadOnlyTx`；实现一律落在 ep-adapter-db-pg，本模块不声明任何 `Pg` 前缀的具体类型。`crates/foundation/src/port/search.rs` 由阶段 1 建空文件，阶段 3b 补齐 SearchDocument、SearchQuery、SearchHit 与 SearchIndexPort、SearchQueryPort，实现落在 ep-adapter-search，索引按法人分区，写入一律经 job-worker 消费 Outbox 事件触发，不在业务事务内调用。`crates/foundation/src/port/doc.rs` 由阶段 1 建空文件，阶段 5 补齐 SheetSpec、ColumnSpec、CellValue、PrintLayout 与 SpreadsheetPort、DocTemplatePort、PdfRenderPort，实现落在 ep-adapter-doc，其后各阶段只在这三个 trait 上增量，不新增渲染接口。`crates/foundation/src/port/kms.rs` 由阶段 1 建空文件，阶段 2 一次补齐冻结六方法 `KmsBackend`、独立 `KmsSigningKeyIdentityResolver`、独立三方法 `KmsKeyMaterialProvisioner`、独立三方法 `KmsPinnedDataKeyBackend` 及阶段 2 第 4.1 节逐项冻结的全部 strong value/exact wire；F-56 后仍不得给 `KmsBackend` 增第七个方法。实现一律落在 ep-adapter-kms，本模块不声明任何载体类型，也不声明任何 `Builtin` 或 `Hsm` 前缀的具体类型。`derive_blind_key` 的三个参数与返回类型均冻结：返回 `BlindIndex([u8; 32])`，对应数据库盲索引列必须为 32 字节；宽度不是配置项，阶段 5、10 不得另设截断或例外路径。pinned 正文新写只用 `CurrentForWrite`，历史读与中断续传只用持久 `DataKeyRefV1` 的 `ExactRef`；见阶段 2 计划第 4.1、4.4 节与第 12 节假设三。

## 2. 进程清单

九个产品常驻进程，与规格第 4.3 章及 F-55 的 apps 清单一一对应。新增或合并进程须先修订本节并写明其信任边界或资源理由，不设不得新增也不得合并的封条。

本节的九个进程是规格第 4.3 章 apps 清单的固定项。其中 plugin-host 承载签名 WASM Component 和 F-55 的本地签名 MCP 子进程；ai-inferer 只承载本地分析 AI。任何阶段计划不得删除 plugin-host 或 ai-inferer 及其服务账户、资源单位、命名管道和 adapter；八进程或把 AI 合并进 core-server 都是被禁止的第二套取值。

| 进程 | 职责 | 监听 | 数据库连接 | 服务虚拟账户 | 资源单位 |
|---|---|---|---|---|---|
| core-server | 全部领域命令与查询、事务、规则与工作流协调、四端 API、供应商门户的受控能力 API、交易路径上的附件正文读写、对写出进程上报内容的审计落库。 | `127.0.0.1:8080` 只作第三方反向代理 upstream；产品进程业务 IPC 只走 `\\.\pipe\ep-core` | 运行期读写池上限 20，只读分析池上限 10，合计不超过 30 | NT SERVICE\ep-core | 资源单位 app-core |
| job-worker | Outbox 消费、站内通知、移动推送编排与脱敏载荷组装、报表与文档渲染、批处理、归档与派生存储传播、内部对账与不变量校验、死信重投；不直接对外出网。 | 127.0.0.1:8081 仅健康与指标 | 独立池上限 5，使用同一运行期读写账号 | NT SERVICE\ep-worker | 资源单位 app-worker |
| portal-gateway | 承载公网供应商门户站点，做会话、限流、脱敏投影的呈现层，五项能力全部经 `\\.\pipe\ep-core` 调 core-server，不持数据库或文件存储凭据。 | `127.0.0.1:8090` 只作第三方反向代理 upstream；业务 IPC 是 `ep-core` 客户端 | 0，不建立事务数据库连接 | NT SERVICE\ep-portal | 资源单位 app-portal |
| integration-gateway | 唯一对外出网进程，承载电子签章、可选移动推送、客户同机 ICAP 与 F-55 远端无状态 Streamable HTTP MCP；逐次验签 manifest/origin/SPKI/schema，不消费 Outbox、不落业务效果。 | 无 TCP 监听；产品侧只监听 `\\.\pipe\ep-integ` | 0；不持数据库凭据、KMS 或业务文件目录权限 | NT SERVICE\ep-integ | 资源单位 app-core |
| plugin-host | 服务端签名 WASM Component 与 F-55 本地签名 LPAC stdio/可选 Windows Hyper-V-isolated container MCP 的受控宿主；utility VM 仅承载单次插件调用，按声明能力与资源限额执行。 | `\\.\pipe\ep-plugin` | 0 | NT SERVICE\ep-plugin | 资源单位 app-plugin |
| ops-agent | 运维中心的采集与暴露：指标端点、健康端点、降级与暴露窗口台账的读取。 | 127.0.0.1:9101 指标，127.0.0.1:9102 健康 | 专用只读角色池上限 2 | NT SERVICE\ep-ops | 资源单位 app-edge |
| archive-writer | 事务日志连续归档、附件正文向服务器之外落点的增量写出、审计证据存储的写出，三项各自不超过 15 分钟周期。 | 无 | 1 个常驻流复制连接与 1 个复制槽，不建常规连接 | NT SERVICE\ep-archive | 资源单位 app-archive |
| backup-writer | 每日全量备份、附件正文存量引导搬运与每日全量写出、备份自动校验、归档链断裂后的重建基线备份。 | 无 | 备份窗口内不超过 1 个流复制连接，窗口外为 0 | NT SERVICE\ep-backup | 资源单位 app-backup |
| ai-inferer | F-55 本地签名模型包的只读加载与单数据集 QueryPlan 推理；不取数、不见结果、不写业务事实。 | `\\.\pipe\ep-ai` | 0；无数据库凭据、网络 token、KMS 或文件写权限 | NT SERVICE\ep-ai | 资源单位 app-ai |

进程侧的固定约束，照抄规格。

- 常驻常规连接精确上限为 37（core rw 20 + ro 10、job-worker 5、ops-agent 2；integration-gateway 为 0），迁移与应急临时连接另计不超过 10，并发连接硬峰值仍不超过 52；余下 5 条是不可分配给任何常驻池的连接安全余量，不得用来恢复 gateway 或新增池。`max_wal_senders` 不低于 4，`max_replication_slots` 不低于 3。
- core-server 与 integration-gateway 是两个进程，但同处资源单位 app-core，单位内不再细分配额；九个自研二进制因此落在八个产品资源单位内，加 PostgreSQL 一个共九个。
- archive-writer 与 backup-writer 是两个独立进程与两个独立资源单位，其内存硬上限各自独立；CPU 比例与磁盘 IO 份额首版固定不启用，不存在实测或静态文件出现取值后自动生效的分支，二者之间不构成 CPU 或磁盘 IO 预算隔离。未来启用必须另立产品版本与正式裁定。两者不持有运行期应用账号，不读业务表。
- 九个进程各注册一个 Windows 服务，由 Windows 服务控制管理器承载启停、依赖顺序与崩溃重启，九个二进制共用一层服务宿主；全部组件以同一份安装包加服务注册脚本交付。资源单位为具名 Job Object，九个自研二进制由服务宿主层在 `ServiceMain` 早期自我指派；PostgreSQL 16 与反向代理由 ops-agent 指派。九个资源单位与规格第 13.1 章九行一一对应，只有 core-server 与 integration-gateway 共用 app-core、ops-agent 与第三方反向代理共用 app-edge；F-55 已把原无承载的搜索行唯一改为 app-ai，搜索按实际调用进程归因。AI 内存硬上限固定为 `floor(0.095 × CERTIFIED_HOST_RAM_BYTES)`，其余行仍按规格算定。CPU/磁盘份额只作认证意图，内存硬上限是唯一运行期配额列；八级让路次序不构成跨进程运行期保证。
> **资源单位现行冻结。** 首版 CPU 比例与 CPU 突发上限仅作硬件标定/认证意图，不写入静态限额文件、不调用 Job Object CPU rate API；按权重磁盘 IO 份额同样固定不启用。内存硬上限是配额表唯一运行期列。未来启用 CPU 或磁盘份额必须另立产品版本、规格裁定、Windows 实测和发布证据门。Job Object 名称由 deployment_id 确定性推导，格式固定为 `Global\EP_<D>_<S>`：`D` 是部署 UUID 的 32 位十六进制，移除连字符并转大写；`S` 精确取 `APP_CORE|APP_WORKER|APP_PORTAL|APP_PLUGIN|APP_EDGE|APP_ARCHIVE|APP_BACKUP|APP_AI|APP_DB`。core-server 与 integration-gateway 共 `APP_CORE`，job-worker=`APP_WORKER`，portal-gateway=`APP_PORTAL`，plugin-host=`APP_PLUGIN`，ops-agent 与第三方反向代理共 `APP_EDGE`，archive-writer=`APP_ARCHIVE`，backup-writer=`APP_BACKUP`，ai-inferer=`APP_AI`，PostgreSQL 16=`APP_DB`。deployment_id 非规范 UUID、全零、推导名与 `deploy/resource-limits.toml` 不一致、同机另一安装已占用同 deployment_id 但安装根或制品摘要不同，均在服务启动前失败关闭；同部署同 suffix 的多个成员打开同一对象是唯一允许的复用。suffix 封闭且 deployment_id 安装时唯一，不提供可配置名称。

- 九个进程各以自己的服务虚拟账户 `NT SERVICE\<服务名>` 运行，互不复用，各自带每服务 SID；不设共用本地组，进程之间的授权一律在对象的 NTFS ACL 与命名管道 DACL 上逐账户列 ACE。落点写出凭据只由 archive-writer 与 backup-writer 的虚拟账户持有；`ep-ai` 无网络 token、数据库/KMS/文件写权限。`tools/ep-migrate` 不注册为 Windows 服务，另有独立普通本地账户 `ep-migrate`，与上述九个不复用，只在迁移窗口内使用。

进程间接口，本基线取值。产品业务命令、查询与正文传输只使用 Windows DACL 命名管道（`tokio::net::windows::named_pipe`），帧为 4 字节大端长度前缀加 JSON，不使用回环 TCP。每个 server generation 仅 bootstrap 首实例取 `first_pipe_instance(true)`，同进程后续/补位实例一律取 `false`；客户端处理 `ERROR_PIPE_BUSY` 并每次重连重新核验对端。唯一窄例外是 integration-gateway 作为客户端连接客户自管同机 ICAP，只允许 IP 字面量 `127.0.0.1|[::1]`，禁止 DNS、代理、重定向和非回环地址，不新增产品监听端口，明文不落盘。管道名可被本地进程抢占的残余风险以首实例 fail-closed、客户端发送前核验 server token 和交付披露处理。archive-writer/backup-writer 上报与 spool 只按下文的精确 operation、七类报文及 critical/reconstructible 规则实现，不保留「四类且满后丢最旧」的旧口径。

> **产品内 IPC 现行封闭清单。** 上段通则中的任何旧 HTTP 示例均不得实现。产品进程之间的业务命令、查询与正文传输只走四条 DACL 命名管道；server 在读取应用字节前冒充客户端并以线程 token 的服务 SID/账户校验逐项 operation allowlist，PID 只作审计关联；客户端在发送前校验 server 进程 token。实现清单不得使用 `portal.*`、`virus_scan.*`、`esign_file.*` 或其他通配模式。
>
> - `\\.\pipe\ep-core` 的 server 是 `NT SERVICE\ep-core`。`ep-portal` 只可调用 `portal.session.sign_in.v1`、`portal.session.sign_out.v1`、`portal.identity.me.v1`、`portal.order_confirm.v1`、`portal.delivery_notice.v1`、`portal.invoice_upload.begin.v1`、`portal.invoice_upload.chunk.v1`、`portal.invoice_upload.end.v1`、`portal.invoice_upload.abort.v1`、`portal.settlement_query.v1`、`portal.profile_maintain.v1`；前三项是身份操作，后八个 operation 承载五项门户业务能力。`ep-archive` 只可调用 `ops.attachment_writeout_scope.query.v1`、`ops.writeout_result.report.v1`、`ops.failure_event.report.v1`、`ops.replication_lifecycle.report.v1`。`ep-backup` 只可调用 `ops.writeout_result.report.v1`、`ops.verification_conclusion.report.v1`、`ops.failure_event.report.v1`、`ops.replication_lifecycle.report.v1`、`ops.attachment_checksum_verdict.report.v1`、`ops.backup_slot.acquire.v1`、`ops.backup_slot.release.v1`。`ep-ops` 只可调用 `health.get.v1`、`metrics.snapshot.v1`、`ops.signed_artifact.install_receipt.v1`。
> - `\\.\pipe\ep-integ` 的 server 是 `NT SERVICE\ep-integ`。`ep-worker` 只可调用 `push.dispatch.v1`、`esign.request.submit.v1`、`esign.status.get.v1`、`mcp.remote.exchange.v1`，并只在同一已关联的签章双工连接接收 `esign_file.begin.v1`、`esign_file.chunk.v1`、`esign_file.end.v1`、`esign_file.abort.v1` 反向流；`ep-core` 只可调用 `virus_scan.begin.v1`、`virus_scan.chunk.v1`、`virus_scan.end.v1`、`virus_scan.abort.v1`、`mcp.remote.exchange.v1`；`ep-ops` 只可调用 `health.get.v1`、`metrics.snapshot.v1`。
> - `\\.\pipe\ep-plugin` 的 server 是 `NT SERVICE\ep-plugin`。`ep-core` 与 `ep-worker` 只可调用 `wasm.execute.v1`、`mcp.local.exchange.v1`；`ep-ops` 只可调用 `health.get.v1`、`metrics.snapshot.v1`。取消由 deadline 或断开当前调用表达，不另设 cancel operation。
> - `\\.\pipe\ep-ai` 的 server 是 `NT SERVICE\ep-ai`。`ep-core` 只可调用 `ai.query_plan.compose.v1`、`ai.model.activate.v1`、`ai.model.deactivate.v1`；`ep-ops` 只可调用 `health.get.v1`、`metrics.snapshot.v1`。总实例 51：compose 数据面 45（运行 15、排队 30）、core 模型控制面 2、ops 2，余 2 只作 accept/补位；四组不可互借。其他账户没有 ACE。
>
> 未列账户或账户调用未列 operation 一律拒绝并审计。core-server:8080 与 portal-gateway:8090 只作第三方反向代理 upstream；job-worker:8081、ops-agent:9101/9102 只暴露无业务数据的健康/指标并由部署 ACL/防火墙限定；integration-gateway 不监听 TCP。唯一回环 TCP 窄例外仍只是 integration-gateway 作为客户端连接客户同机 ICAP。

`ep-portal` 的服务账户认证只证明调用进程，不证明终端主体。门户管道请求固定为未受信 `PortalPipeRequest { opaque_session_token, requested_legal_entity_id, device_id, request_id }`，不存在可提交的 user/account/supplier/role/duty/data-scope/client 字段。core-server 从已核验的管道账户固定 `ClientKind::Portal`，重新验证 token、`account_kind=PORTAL`、设备、供应商绑定和授权法人集合后自行构造 `SecurityContext`；伪法人、内部 token、伪 device、多余主体字段或自填 client 必须拒绝并进入负例。

四条管道的抗占满边界是编译期常量，不设配置分支。`ep-core` 总实例 32、账户活跃连接上限 portal=20/archive=4/backup=4/ops=2；`ep-integ` 总实例 16、worker=8/core=4/ops=2；`ep-plugin` 总实例 12、core=4/worker=4/ops=2；`ep-ai` 总实例 51，compose core=45、control core=2、ops=2，余 2 只用于 accept/补位。各账户/用途额度互不借用；账户达到上限时，服务端完成身份核验后、读取任何应用帧前返回 `PLATFORM.IPC.CONCURRENCY_LIMIT`，AI compose 改返回 `AI.INFERENCE.CONCURRENCY_LIMIT`，断开并写安全审计。普通连接身份握手/首长度前缀/空闲/单调用绝对上限固定为 5/10/30/120 秒，半帧、慢帧、超限或断连立即清缓冲关闭；阶段 3 大正文仍按 10/30/3600 秒且 3600 秒流会话上限不被普通调用 120 秒截断。

每个 server generation 只在启动创建首实例时用 `first_pipe_instance(true)` 抢名并 fail-closed；同一持有首实例的服务进程创建后续或补位实例必须用 false。首实例句柄贯穿 listener 生命周期，断开后用同一句柄重新接受；异常丢失即整个服务退出交给 SCM 重启，不得靠 false 实例续命。客户端以 `SECURITY_SQOS_PRESENT|SECURITY_IDENTIFICATION` 打开管道。server 在读取任何应用字节前执行 `ImpersonateNamedPipeClient`、用 `OpenThreadToken` 核验允许的服务 SID/账户、并在所有分支 `RevertToSelf`；PID 只作审计关联。client 发送前用 server PID 与进程 token 核验预期服务账户，每次重连重新核验，发送失败不得转投未核验实例。

普通帧继续取 4 字节大端长度前缀加 JSON、整帧不超过 1 MiB。大正文统一使用 `BoundedChunkStreamV1` 状态机：begin 声明 UUIDv7 request_id、对象 id、总长度与 SHA-256；chunk 的 seq 从 0 连续、解码后每块不超过 524288 字节、块带 SHA-256；end 重复 next_seq、总长度与总哈希；abort 终止并清零缓冲。每块 ACK 后才允许下一块，最多一块在途，块 ACK/空闲/会话绝对超时固定 10 秒/30 秒/3600 秒；乱序、重复、缺块、长度或哈希不符立即 abort，重试用新 request_id。病毒扫描和签章结果文件上限均为 5368709120 字节，四个 operation 分别为 `virus_scan.begin.v1|virus_scan.chunk.v1|virus_scan.end.v1|virus_scan.abort.v1` 与 `esign_file.begin.v1|esign_file.chunk.v1|esign_file.end.v1|esign_file.abort.v1`；门户发票附件上限 52428800 字节，四个 operation 为 `portal.invoice_upload.begin.v1|portal.invoice_upload.chunk.v1|portal.invoice_upload.end.v1|portal.invoice_upload.abort.v1`。三者采用完全相同 DTO、状态机、背压与超时，不得另造协议。integration-gateway 不持数据库、KMS 或平台文件目录凭据，不消费 Outbox；push/e-sign/virus 只返回清洗结果。签章文件反向流给 job-worker 后，整批逐件进入「临时加密对象→长度/哈希/TYPE_SNIFF/STRUCTURE→按 `NONE|CUSTOMER_ICAP` 扫描→签章验签→数据库确认/发布」完整附件流水线。仅整批全部 `PUBLISHED` 才建签章关联并允许合同转 `SIGNED`；失败对象保持 `QUARANTINED`，合同不得转 `SIGNED`。

F-55 的两个 MCP exchange operation 另用 `McpExchangeChunkStreamV1`，只借用 1 MiB 普通 framing 与 512 KiB decoded chunk 上限，不复用上述大文件 DTO/3600 秒时限。它按 manifest/request/response 三段、逐块 ACK、连续序号和端到端 hash 承载 request 1 MiB、response 8 MiB，绝对时限固定 30 秒；不新增同义 operation 或落盘恢复。

## 3. 数据库约定

### 3.1 schema 划分与角色

一个数据库实例，库名 `ep`。schema 与模块一一对应，另设八个平台 schema 与一个低代码扩展 schema，共 24 个。

平台侧：`platform_core`、`platform_authz`、`platform_meta`、`platform_flow`、`platform_audit`、`platform_msg`、`platform_file`、`platform_ops`。业务侧：`mdm`、`crm`、`cpq`、`clm`、`sales`、`procure`、`inventory`、`costing`、`project`、`service`、`finance`、`ledger`、`invoice`、`portal`、`reporting`。低代码自定义对象的物理表一律建在 `ext`，不得建到业务 schema。

数据库角色，照抄规格第 7.7 章的用途分账号口径，本基线只给名字。

| 角色 | 用途 | 权限边界 |
|---|---|---|
| ep_app_rw | 运行期读写，只由 core-server、job-worker 经统一数据访问层持有 | 全部 schema 的表数据读写，无 DDL、无角色管理、无策略管理，非 SUPERUSER、非 BYPASSRLS；integration-gateway 明确不持有 |
| ep_analyst_ro | 只读分析池、常规报表与经营看板、高级只读 SQL | 只读，受 RLS 约束，独立连接上限与语句超时 |
| ep_ops_ro | ops-agent 专用 | 只读运维、健康与积压相关视图 |
| ep_migrator | 迁移 DDL 与自定义对象在线 DDL | 仅迁移窗口临时启用，启用与回收记入审计 |
| ep_breakglass | 受控应急本地账号 | 单次不超过 8 小时，用后轮换，独立审计 |
| ep_archiver | archive-writer 专用 | 仅 REPLICATION 属性，无任何业务表权限，仅本机连接 |
| ep_backuper | backup-writer 专用 | 仅 REPLICATION 属性，无任何业务表权限，仅本机连接 |
| ep_mod_<module> | 各模块 schema 与其对象的属主 | 仅归属与 DDL 边界，不用于运行期读写 |

不使用 `SET ROLE` 做隔离，连接生命周期内不切换角色，照抄规格。

### 3.2 表命名

- schema 已承担域，表名不再重复域前缀。表名 snake_case 复数，如 `sales.sales_orders`、`ledger.vouchers`。跨 schema 引用一律写全限定名。
- 明细行表命名为主表单数加 `_lines`，如 `sales.sales_order_lines`、`ledger.voucher_lines`。
- 台账类表用业务名加 `_entries`，如 `finance.receivable_entries`、`inventory.stock_qty_entries`、`inventory.stock_value_entries`。
- 多对多关联表命名为 `<a>_<b>_links`，如 `finance.settlement_invoice_links`。
- 附件关联表命名为 `<主表单数>_attachments`。
- 视图前缀 `v_`，物化视图前缀 `mv_`，首版不使用物化视图。
- 枚举一律不用 PostgreSQL enum 类型，改用 `text` 加 CHECK 约束，理由是 enum 的取值增删需要 DDL 且不可在线收窄，与规格第 7.4 章的在线变更边界冲突。取值一律大写 snake_case，如 `DRAFT`、`PENDING_APPROVAL`、`EFFECTIVE`。
- 布尔列以 `is_` 或 `has_` 开头。时间列以 `_at` 结尾且为 `timestamptz`，日期列以 `_date` 或 `_on` 结尾且为 `date`。

### 3.3 主键与外键

- 主键列名一律 `id`，类型 `uuid`，取值为应用侧生成的 UUIDv7。理由是时间有序使 B-tree 插入局部性好，且可在事务开始前生成以构造聚合内引用，避免往返取号。数据库侧不设默认值，缺失即为应用缺陷。
- 外键列名为被引用表单数加 `_id`，如 `customer_id`、`sales_order_line_id`。
- 同一 schema 内的引用建真实外键约束，`ON DELETE RESTRICT`，不使用级联删除。
- 跨 schema 即跨模块的引用凡目标单一的，一律建真实外键并 `ON DELETE RESTRICT`；默认外键做成复合形式 `(legal_entity_id, <ref>_id)` 指向被引用表的 `(legal_entity_id, id)` 唯一键，跨法人引用因此由数据库强制，不再只靠写入前校验。被引用对象的建表迁移版本号更晚而无法在建表语句中直接声明的少数引用，由一条版本号晚于两侧建表迁移的 `ALTER TABLE ADD CONSTRAINT` 补建，该迁移放在引用方所属 schema 的目录下。
- 用户与身份证据有两种固定物理形状，不得自行变体。业务表的负责人、审核人、受理人、处理人、确认人、操作者等用户列，以 `(legal_entity_id, <user_ref>)` 真实复合外键指向 `platform_authz.user_legal_entity_grants(legal_entity_id, user_id)`；`SYSTEM_PRINCIPAL_ID` 必须在每个法人各有一条永不物理删除的授权行。`platform_core.user_accounts`、`user_devices`、`sessions`、`reauth_challenges` 是不带 `legal_entity_id` 的全局身份证据表：引用其中会话、设备登记行或重新认证挑战的列仍必须建指向其主键的真实单列外键并 `ON DELETE RESTRICT`，同时由写用例在持锁事务内验证该证据归属当前用户与法人；这是一项外键形状例外，不是允许无外键逻辑引用。业务用户列不得直接外键到 `user_accounts`。
- 允许不建外键的引用是封闭白名单，只有四组。第一组是带判别列、可能指向两个及以上目标表的显式多态引用；当前仅包括各阶段逐表登记的业务对象、来源单据、品项、付款申请引用、资金来源/目标与影响面目标/结果组合，至少含 `ledger.vouchers`、`costing.cost_entries`、`inventory` 九张表的来源单据组合、阶段 7 的五组来源/引用组合、阶段 10 的品项与资金来源/目标组合，以及 F-54 的影响面 `target/result` 组合；不得仅因列名叫 `ref_id` 或 `source_id` 就自动归入本组。第二组是平台跨越型证明 `approval_ref` 与 `release_package_id`。第三组是阶段 13 `ext` 扩展对象与自定义字段指向业务表的元数据驱动引用。第四组只有 F-50 明确裁定的 `invoice.invoice_reversals.linked_purchase_return_id`。每个多态组合必须在所属阶段表定义中同时写出判别列、封闭目标集合、NULL-safe 形状 CHECK 与写入时同法人校验；缺任何一项即不在白名单。除此之外出现跨 schema 无外键引用即为违规。
- 模块隔离由本文件第 1.3 节末条的一个仓储只访问自己模块的 schema 保证，与外键无关；模块停用是应用层状态，不删数据，不与 `ON DELETE RESTRICT` 冲突。application 层的跨模块契约调用保留，定位由跨模块引用完整性的唯一保证降级为给出可读错误码与业务状态判定的前置校验，引用存在性由外键兜底；SQLSTATE 23503 在 ep-adapter-db-pg 一处统一映射为一个错误码并记录约束名，按应用缺陷处理并告警，不作为用户可恢复错误。
- 任何跨法人的引用一律禁止，写入前校验两侧 `legal_entity_id` 相等。
- 业务编号列：单据类为 `doc_no text`，档案类为 `code text`，唯一约束一律带法人，即 `(legal_entity_id, doc_no)`。

### 3.4 时间戳与时区

- 存储一律 `timestamptz`，实际存储为 UTC。数据库实例 `timezone = 'UTC'`，全部连接在取用时不改时区。
- 展示时区固定为 `Asia/Shanghai`，转换只发生在客户端与报表渲染层，服务端不做展示转换。
- 业务日期用 `date`，不带时区，语义是中国标准时间下的自然日。服务器自然日的取值一律用 `(now() AT TIME ZONE 'Asia/Shanghai')::date`，禁止用 `current_date`。
- Rust 侧 `timestamptz` 映射 `chrono::DateTime<chrono::Utc>`，`date` 映射 `chrono::NaiveDate`。禁止在 domain 层直接取当前时间，一律经 `foundation::Clock` 端口，测试注入 `FixedClock`。
- 会计相关的三个日期列名固定：业务事件登记单据上的记账日期为 `posting_date date`；凭证与子账条目上的原始业务日期为 `business_date date`；会计期间归属为 `accounting_period_id uuid`。顺延入账在凭证上另记 `deferred_from_period_id uuid null`，非空即表示该凭证发生过顺延，供界面标注与检索使用。

### 3.5 金额、数量与精度

本组取值同时约束库存侧与财务侧，是规格第 17.3 章守恒与勾稽校验能够成立的前提。税率可选值集合与账龄分档也已关闭：U-D-04 固定出厂六档税率并允许行级多税率，U-D-11 固定出厂六档账龄并在阶段 11 迁入按法人分套的权威表；本组不再留任何业务决策给实现方。

| 语义 | 数据库类型 | Rust 类型 | 说明 |
|---|---|---|---|
| 账面金额 | `numeric(18,2)` | `foundation::Money`，内含 `rust_decimal::Decimal` | 凡进入总账、子账、台账、发票与报表的金额一律 2 位小数，本位币固定人民币，不设币种列 |
| 单价 | `numeric(18,6)` | `foundation::UnitPrice` | 含价目表单价、采购不含税单价、暂估单价、移动加权平均单价 |
| 数量 | `numeric(18,6)` | `foundation::Quantity` | 含库存数量、订单行数量、发票数量 |
| 比例与税率 | `numeric(9,6)` | `foundation::Rate` | 取值域为小数，13% 存为 0.130000 |

舍入规则固定为四舍五入且中值远离零，Rust 侧 `Decimal::round_dp_with_strategy(2, MidpointAwayFromZero)`。计算链路上的中间值在内存中以 Decimal 全精度保留，只在写库前一次性 round。

尾差归属固定为三条，写入本基线即为全阶段口径。

- 出库金额等于移动加权平均单价乘出库数量并 round 到 2 位，数量账与金额账同源写入同一 round 后取值，因此库存金额账余额恒等于按 2 位累加的结果，与总账存货科目余额天然相等。
- 移动加权平均单价是派生值，取库存金额余额除以结存数量并 round 到 6 位，除不尽产生的尾差在仍有结存时留在金额余额中，不做单独调整分录；任何出库使该仓库该物料结存数量归零时，本次库存账面出库金额直接取锁前库存金额余额全额，金额余额与单价同时归零。物料采购退货同样采用这套当前账面价值：部分退货按移动加权平均单价计量，全部退清按余额全额；原收货暂估金额只用于 GRNI 消费，二者差额以有符号主营业务成本腿承接，不得在零结存下留下金额孤儿。
- 价差拆分中，尚有库存部分与已出库部分各自 round 到 2 位，两者之和与总差额的尾差一律计入已出库部分即当期主营业务成本，理由是该部分不再经过存货科目，尾差留在此处不会破坏存货金额账与数量账的一致性。

### 3.6 软删除口径

业务数据不做软删除。理由是规格第 7.2 章与第 7.5 章要求已过账分录、库存流水、审批证据与审计证据只追加不覆盖，软删除会引入“看不见但仍在”的第三态，使守恒校验的取数范围产生歧义。

- 业务单据的注销、作废、关闭一律走 `status` 状态机，不加 `deleted_at`。
- 档案类的停用用 `is_active boolean not null default true` 加 `deactivated_at timestamptz null`，停用不影响历史引用。
- 只有两类对象允许删除标记：`platform_file.attachment_objects` 的 `deleted_at`，以及低代码配置对象的 `retired_at`。物理删除只能由处置流程经专用路径与专用账号发起。
- 任何 `DELETE` 语句在业务 schema 上被禁止，由 CI 的 SQL 静态检查拦截。按期物理清理的封闭白名单只有：`platform_msg.idempotency_keys` 的过期行、`platform_msg.outbox_events` 的 `DONE` 行、`platform_msg.inbox_consumptions`、超过保留期且已读的 `platform_msg.notifications` 及其 `platform_msg.notification_deliveries`、超过保留期且已结束的 `platform_flow.process_instances` 及其 `process_steps` 与 `process_timers`、终态 `platform_file.upload_sessions` 及其 `upload_parts`，以及 `platform_ops` 的过期指标快照。`inbox_consumptions` 的保留期必须严格长于对应 `DONE` Outbox 的保留期，父子表按外键逆序分批删除；`audit_events`、`audit_segments`、`audit_anchors`、`dead_letters`、`attachment_objects`、`attachment_versions`、`scan_results` 与 `process_compensations` 永不进入本白名单。新增清理对象必须先修订本条、数据字典与正反 SQL 静态检查，不得以“平台表”作为无限兜底类。

### 3.7 乐观锁

- 每张可更新的业务表带 `row_version bigint not null default 1`。
- 更新一律写为 `UPDATE ... SET ..., row_version = row_version + 1, updated_at = now(), updated_by = $u WHERE id = $1 AND row_version = $2`，受影响行数为 0 即判定版本冲突。
- 冲突映射为错误分类 `BUSINESS_CONFLICT`，错误码 `PLATFORM.CONCURRENCY.STALE_VERSION`，HTTP 409，响应中回带当前版本号与最后修改人。
- 不使用 `xmin` 做版本，理由是它不随 HOT 更新语义稳定暴露给应用，也无法在离线客户端上比较。
- 仅追加表不带 `row_version`。

### 3.8 RLS 策略的统一写法

会话变量固定为 `app.legal_entity_id`，且它是行级策略的唯一判据，照抄规格。其余上下文变量 `app.user_id`、`app.request_id`、`app.trace_id` 只用于审计与日志，不参与策略判定。

每张带 `legal_entity_id` 的表按同一模板生成，模板由迁移生成器统一产出，不允许手写变体。

```sql
alter table <schema>.<table> enable row level security;
alter table <schema>.<table> force row level security;
create policy rls_<table>_le on <schema>.<table>
  using (legal_entity_id = nullif(current_setting('app.legal_entity_id', true), '')::uuid)
  with check (legal_entity_id = nullif(current_setting('app.legal_entity_id', true), '')::uuid);
```

变量缺失时 `current_setting` 返回 NULL，比较结果为 NULL，行不可见也不可写，即默认拒绝。连接取用时执行 `select set_config('app.legal_entity_id', $1, false)` 等四条设置，连接归还前逐项设回空串，禁止用 `DISCARD ALL`，理由是它会清掉预备语句缓存。

不设任何 `BYPASSRLS` 角色。跨法人查询按授权法人集合逐个法人设置变量后分别查询再在应用侧合并，不使用绕过策略，也不使用 `OR` 展开法人列表。内部对账系统安全上下文按法人逐轮遍历，同样只写单一法人，照抄规格。

凡带 `legal_entity_id` 的表一律按上述模板建策略。不带该列的表必须同时满足并逐表登记两件事：其一准入判据，该表的行集合与法人无关，即行要么在本部署内对全部法人取值相同，要么是隔离机制自身或部署自身的元数据；其二隔离承接点，写明该表的法人可见性落在哪个应用层入口以及该入口的判据来源。登记落在 `platform_core.unpoliced_table_registry`，由阶段 2 建，与 `sensitive_field_registry` 同构，列含 schema、table、准入判据取值、隔离承接入口、rls_matrix 用例标识。`db/checks/` 的编号断言由十二项扩为十三项，第十三项断言 `pg_class` 中所有未启用行级安全的本项目表与该登记表逐行一致，多一张少一张即返回非零行，由 `ep-migrate check` 执行，未登记的表建不出来。不带 `legal_entity_id` 的表只有四类这一封闭枚举撤销，其中全局配置字典这个无定义的无限容量兜底类名一并撤销；迁移历史由迁移工具自建，不在本项目建表范围内。发布门禁项 RG-RLS-MATRIX-GREEN 的判据由 32 组矩阵全部通过改为登记表行数与 rls_matrix 中承接入口用例数相等且全绿，阶段 4 交付的 matrix_32.rs 保留为其中一段。

### 3.9 迁移工具与迁移命名

迁移工具固定为仓内 `tools/ep-migrate` 自建 Runner，不依赖 refinery；原因是冻结的 14 位时间戳版本号超出 refinery 0.8 的 `INT4` 版本空间，详见 ADR-0013。全库只有一个 Runner 与一张 `platform_core.schema_history`，常规迁移逐文件单事务执行，`concurrent/` 目录逐文件自动提交执行；两条路径共用同一解析、校验和、缺失/分歧判定与历史写入实现。历史表 `version` 固定为 `BIGINT`；发布制品发现缺失或分歧立即失败，开发库只允许尚未施加的新版本按全局时间戳顺序到达，不允许改写已登记历史。 具体版本、文件名、归属目录与 EXISTING/PLANNED 状态只以[数据库迁移目录](../../../migration-catalog.md)为准。

- 迁移文件路径 `db/migrations/<schema>/`，24 个目录保留，历史表只有 `platform_core.schema_history` 一张。
- 文件命名 `V<YYYYMMDDHHMMSS>__<schema>_<slug>.sql`，版本号必须是真实时间且全局唯一、严格递增，由 `xtask sqlcheck` 断言；slug 为小写 snake_case 动词短语，如 `V<YYYYMMDDHHMMSS>__sales_add_order_line_delivery_batch.sql`。伪时间戳当序号用不能提供插入免疫，既有的重复版本号与非法分位一律按本条重取。
- 每个迁移文件头固定 `SET ROLE ep_mod_<schema>;`，使第 3.1 节的属主与默认权限在实际执行者为 `ep_migrator` 时仍然成立；第 3.1 节不使用 `SET ROLE` 做隔离一句只约束运行期连接，不约束迁移窗口。
- 一个文件只做一件事，不得在同一文件里既建表又回填数据。数据回填单独成文件，命名 slug 以 `backfill_` 开头。
- 迁移一律可离线执行，禁止在迁移中调用应用代码。
- 执行顺序由单一全局 Runner 按文件版本号排序，`db/migrations/order.toml` 撤销。跨 schema 的迁移放在其主要创建对象所属的 schema 目录下，正确性由版本号必须晚于其全部被引用对象保证，并由空库全量执行在 CI 中验证；其后编号顺延这一整类连锁改动随之取消。
- 每个迁移必须成对提供回退说明，写在文件头注释的 `-- rollback:` 段中；无法安全逆向的迁移必须注明只能用升级前备份或影子表回退，与规格第 18 章一致。
- 在线变更边界照抄规格第 7.4 章：新增表、新增可空列、新增索引、放宽长度可在线执行；新建空表的主键、唯一约束与索引随建表事务使用普通 `CREATE INDEX`，只有向已经可能承载存量数据的表追加索引才必须拆成 `concurrent/` 下的独立非事务文件并使用 `CREATE INDEX CONCURRENTLY`，不得把 `CONCURRENTLY` 混入常规事务迁移。改列类型、收紧非空、重建主键进停机窗口。单次在线变更锁持有上限 5 秒，迁移执行上限 30 分钟，迁移会话固定 `SET lock_timeout = '5s'` 与 `SET statement_timeout = '30min'`。

### 3.10 索引命名与基线索引

- 主键 `pk_<table>`，唯一 `ux_<table>_<col…>`，普通 `ix_<table>_<col…>`，外键 `fk_<table>_<ref_table>`，检查 `ck_<table>_<rule>`，策略 `rls_<table>_le`，序列 `sq_<table>_<col>`。
- 每张业务表的基线索引固定三条：`pk_<table>`；`ix_<table>_legal_entity_id_created_at`，用于列表默认排序与法人内扫描；单据类另加 `ux_<table>_legal_entity_id_doc_no`。唯一列名例外是 `platform_audit.audit_events`：该表按第 9.4 节没有 `created_at`，以 `ix_audit_events_le_occurred` 覆盖 `(legal_entity_id, occurred_at, id)`，不得为套模板新增第二个时间列。
- 首版不使用函数索引、部分索引与 JSON 路径索引，照抄规格第 7.4 章的公共能力基线。
- 全表扫描的容忍上限：任何进入附录 A.1 度量清单的查询在基准数据集上不得出现顺序扫描，阶段计划必须给出对应查询的 `EXPLAIN` 证据。

## 4. 公共列

每张业务表必须有下列列，顺序也按此排列，便于评审逐表核对。

| 列名 | 类型 | 可空 | 默认 | 语义 |
|---|---|---|---|---|
| id | uuid | 否 | 无，应用侧 UUIDv7 | 主键 |
| legal_entity_id | uuid | 否 | 无 | 法人，RLS 唯一判据，任何跨法人引用禁止 |
| security_level | smallint | 否 | 20 | 密级，取值 10 公开、20 内部、30 保密、40 机密；字段级密级未赋值时按所属对象取值 |
| data_scope_tags | text[] | 否 | '{}' | 数据范围标签，派生存储与归档层必须随事件携带 |
| row_version | bigint | 否 | 1 | 乐观锁版本 |
| created_at | timestamptz | 否 | now() | 创建时间，UTC |
| created_by | uuid | 否 | 无 | 创建人用户 ID，系统上下文一律写入 `foundation::SYSTEM_PRINCIPAL_ID`，字面量为 00000000-0000-7000-8000-000000000001 |
| updated_at | timestamptz | 否 | now() | 最后更新时间，UTC |
| updated_by | uuid | 否 | 无 | 最后更新人用户 ID |

补充规则。

- 单据类表另加 `doc_no text not null` 与 `status text not null`，`status` 带 CHECK 约束枚举该单据状态机的全部取值。
- 档案类表另加 `code text not null` 与 `is_active boolean not null default true`、`deactivated_at timestamptz null`。
- 只有凭证、子账权威条目及明确承担期间归属的会计事实表同时带 `business_date date` 与 `accounting_period_id uuid`；来源业务登记单只带其权威业务日期或 `posting_date date`，在正式入账事务内经 `AccountingPeriodResolver` 唯一解析期间，不为套公共列提前复制 `accounting_period_id`。逐表取值规则见第 3.4 节与所属阶段数据字典。
- 仅追加表不带 `row_version`、`updated_at`、`updated_by`。是否带 `reverses_id uuid null` 由该表有无业务冲销或更正语义决定：有的必须带，并在表定义处写明它指向哪张表的哪条记录；没有的一律不得带，不得为满足列约定而保留一个恒为 NULL 的该列。每张仅追加表由所属阶段在其表定义处逐表写明取舍与理由，本节不再列举表名；新增仅追加表按第 12 节纪律先登记再实现。`platform_audit.audit_events` 的列集以第 9.4 节为准，本节不另给它加列。
- 不设承载部署客户隔离的 `tenant_id` 或 `deployment_customer_id` 列。规格第 7.1 章规定每个部署客户一个独立事务数据库实例，不能再造第二套部署隔离口径；同一法人内引用 CRM 业务客户的外键列 `customer_id` 合法，并须按复合外键规则同时带 `legal_entity_id`。
- 附件引用不落在业务表列上，一律经 `<主表单数>_attachments` 关联表，列为 `owner_id`、`attachment_object_id`、`purpose`、`sort_no` 与公共列。

## 5. API 契约

### 5.1 传输与路径

- 对外传输为 HTTP/1.1 与 HTTP/2，TLS 1.3 由反向代理终结；产品进程间业务 IPC 只走 DACL 命名管道，不走回环 HTTP/TCP。除附件正文外，对外载荷固定为 `application/json; charset=utf-8`，字符集 UTF-8；附件分片上传 `/api/v1/platform/attachments/uploads/{session_id}/parts/{part_no}` 与附件版本正文下载 `/api/v1/platform/attachments/{id}/versions/{version_no}/content` 两类路径固定使用 `application/octet-stream`，这是唯一对外二进制正文例外。首版不提供 gRPC 与 GraphQL 对外接口。
- 员工四端路径前缀 `/api/v1`，供应商门户对外路径前缀 `/portal/v1`。portal-gateway 把门户路由映射到 `\\.\pipe\ep-core` 的具名 operation 并调用同一个 core 用例，不在产品进程之间重新发起 `/api/v1/portal/...` HTTP 请求。
- 资源路径形如 `/api/v1/<module>/<resource-plural>` 与 `/api/v1/<module>/<resource-plural>/{id}`，路径段小写，多词用连字符，如 `/api/v1/sales/sales-orders/{id}/lines`。
- 非 CRUD 的领域命令一律写为 `POST /api/v1/<module>/<resource-plural>/{id}/actions/<verb>`，verb 用连字符小写动词，如 `actions/submit-for-approval`、`actions/confirm-delivery`、`actions/void`。不使用动词化的资源名，也不使用查询参数表达动作。
- 批量操作写为 `POST /api/v1/<module>/<resource-plural>/actions/<verb>-batch`，单次上限 200 条，超出返回 `VALIDATION`。
- JSON 字段一律 snake_case，与数据库列名、Rust 结构体字段名保持同一套命名，理由是三层同名可消除映射表这一类长期错误来源。serde 不使用 rename_all。

### 5.2 封套

成功响应。

```json
{
  "success": true,
  "data": { },
  "error": null,
  "meta": { },
  "trace_id": "4bf92f3577b34da6a3ce929d0e0e4736"
}
```

失败响应。

```json
{
  "success": false,
  "data": null,
  "error": {
    "code": "SALES.SALES_ORDER.CREDIT_LIMIT_EXCEEDED",
    "category": "BUSINESS_CONFLICT",
    "message": "客户可用信用额度不足，本次占用金额超出可用额度。",
    "details": [{ "field": "lines[2].amount", "reason": "OVER_LIMIT", "value": null }],
    "retryable": false,
    "incident_no": "ERR-20260810-000123",
    "occurred_at": "2026-08-10T02:11:43.512Z",
    "advice": "调整订单金额，或提交信用超额审批。"
  },
  "meta": null,
  "trace_id": "4bf92f3577b34da6a3ce929d0e0e4736"
}
```

`incident_no`、`occurred_at`、`retryable`、`advice` 四项对应规格第 15.1 章要求的关联编号、发生时间、可否重试与处理建议，缺一不可。`message` 与 `advice` 是面向使用者的简体中文，禁止出现堆栈、SQL、内部主机名、进程名、表名与密钥。`details` 只在 `VALIDATION` 与 `BUSINESS_CONFLICT` 下非空。

### 5.3 分页、排序与过滤

- 分页参数 `page` 从 1 起，`page_size` 默认 20，上限 200。响应 `meta` 为 `{"page":1,"page_size":20,"total":123,"total_pages":7}`。
- `page * page_size` 超过 10000 时服务端拒绝并要求改用键集分页，参数为 `cursor` 与 `page_size`，`meta` 返回 `next_cursor`。理由是 20 并发下深偏移扫描仍会击穿规格第 16 章的报表通过线。
- 排序参数 `sort=<field>:asc|desc`，可多段用逗号分隔，字段必须在该端点声明的白名单内，否则 `VALIDATION`。默认排序固定为：单据与台账按 `created_at desc, id desc`，档案按 `code asc`，账表按 `accounting_period_id asc, doc_no asc`。
- 过滤参数 `filter[<field>]=<op>:<value>`，`op` 取 `eq`、`ne`、`gt`、`gte`、`lt`、`lte`、`in`、`like`、`between`、`isnull` 十种，`in` 与 `between` 的值用逗号分隔。`like` 只允许前缀匹配，服务端自动加尾部通配符，不接受用户输入的通配符，理由是避免前导通配导致全表扫描。
- 单据与台账列表的默认筛选期间固定为最近 3 个自然月，用户可放宽；导出上限 50000 行，超出必须转异步任务并由站内通知回执。

### 5.4 幂等键

- 除下述固定认证前矩阵与 F-55 一次性 secret 端点外，全部 POST、PUT、PATCH、DELETE 请求必须带 `Idempotency-Key` 头，取值为客户端生成的 UUIDv7，缺失即返回 `VALIDATION` 与错误码 `PLATFORM.IDEMPOTENCY.KEY_REQUIRED`。
- 幂等作用域为四元组：法人、用户、端点、键值。存储表 `platform_msg.idempotency_keys`，列含 `key`、`legal_entity_id`、`user_id`、`endpoint`、`request_hash`、`response_status`、`response_body`、`created_at`、`expires_at`，保留 7 天，唯一约束在四元组上。
- 重复请求且 `request_hash` 相同时返回首次结果，并带响应头 `Idempotent-Replay: true`。键相同而 `request_hash` 不同时返回 409 与 `PLATFORM.IDEMPOTENCY.PAYLOAD_MISMATCH`。
- 幂等键的写入与业务写入在同一数据库事务内，不使用外部缓存。
- 唯一落库豁免是附件分片 PUT `/api/v1/platform/attachments/uploads/{session_id}/parts/{part_no}`：它仍必须带合法 `Idempotency-Key`，但服务端以数据库唯一键 `(legal_entity_id, session_id, part_no)` 加 `part_hash` 作为自然幂等事实，不写 `platform_msg.idempotency_keys`；同一分片号同哈希回放首次结果，不同哈希返回 409 与 `PLATFORM.IDEMPOTENCY.PAYLOAD_MISMATCH`。除这一具名端点外，不得以“自然幂等”为由新增豁免。
- 认证前写端点的唯一头豁免由不可配置的 `PRE_AUTH_ENDPOINTS` 逐头矩阵承载：`POST /api/v1/platform/sessions/actions/sign-in`、`POST /api/v1/platform/sessions/actions/complete-mfa` 与 `POST /api/v1/portal/sessions/actions/sign-in` 均豁免 `Authorization`、`X-Legal-Entity-Id`、`Idempotency-Key`。complete-mfa 以数据库中一次性挑战令牌的条件消费防重放，不把新会话令牌原文写入通用幂等响应缓存；成功响应丢失后重新开始登录。除这三个具名端点外不得新增**认证前**免幂等写端点。
- 已认证的一次性 secret 响应闭集 `SENSITIVE_ONE_TIME_RESPONSE_ENDPOINTS` 首版恰一项：`POST /api/v1/platform/mcp-human-grants/actions/issue`。该端点禁止 `Idempotency-Key`、不写通用 response cache；携带该头以既有 `PLATFORM.REQUEST.INVALID_PAYLOAD` 和字段级原因拒绝，明文 token 只返回一次，响应丢失后撤销/过期再签发。它不是 `PRE_AUTH_ENDPOINTS`、不豁免 Authorization/法人/CSRF/权限，也不能类推到其他写 API。

### 5.5 错误码结构与分类

分类枚举固定五类，与规格第 15.1 章一一对应，不得增删。

| category | 含义 | HTTP | retryable |
|---|---|---|---|
| VALIDATION | 输入校验错误，定位到字段 | 400 | false |
| BUSINESS_CONFLICT | 业务冲突，含版本冲突、状态机非法迁移、守恒与勾稽不成立 | 409 | false |
| PERMISSION_DENIED | 权限或策略拒绝 | 403，无权访问已存在记录时统一 404 | false |
| EXTERNAL_SYSTEM | 外部依赖或受控远端 MCP 故障；首版外部业务主系统类别仍仅电子签章 | 502 | true |
| INFRASTRUCTURE | 基础设施故障，含数据库不可用、磁盘写满、限流 | 503，限流 429 | true |

错误码为三段点分大写，形如 `<MODULE>.<RESOURCE>.<REASON>`，模块段取第 1.2 节的模块码或 `PLATFORM`，资源段取表名的单数大写，原因段为动宾短语。全量错误码集中登记在 `docs/error-codes.md` 与 `ep-foundation` 的 `error::codes` 常量表，两处由 CI 校验一致，重复码即构建失败。

存在性泄漏的统一处理：对当前安全上下文不可见的记录，读、写、删一律返回 404 与 `PLATFORM.AUTHZ.NOT_FOUND_OR_DENIED`，不区分不存在与无权，理由是规格第 15.1 章要求权限拒绝不泄露无权数据。只有当前用户对该对象类型完全无权时才返回 403 与 `PLATFORM.AUTHZ.OBJECT_FORBIDDEN`。

F-56 的 `LicenseAdmissionGate` 是全部入口共享的前置判定，不由各业务 handler 自行计算日期或重复映射错误。HTTP 顺序固定为 route/header/body-size guard → session/device/法人/权限 → strict payload parse 或受控对象类型查找 → 许可证 effect 判定 → 幂等占位与业务事务；认证前身份入口在凭证验证成功后、签发会话前判定。scheduler/Outbox/审批执行器在领取或产生新外部副作用前调用同一判定。effect exact-set 是 F-56 的十值 `BusinessWrite|BusinessApproval|IntegrationOutbound|AutomationStart|ReadReportAuditBackupExport|IdentitySecurityDisposition|ComplianceDisposition|InFlightConvergence|LicenseGrantRecovery|ModuleDisableRecovery`：前四类在全局 `RESTRICTED`，或有效 LIST scope 未包含本次已鉴权目标法人时，以 `PLATFORM.LICENSE.RESTRICTED=BUSINESS_CONFLICT/409/false` 拒绝，后者不改变部署级全局状态；中间四类在两种情况下都允许；最后两类只承载 F-56 明列的 `LICENSE_GRANT` 全恢复链与 `MODULE_PACKAGE:DISABLE` 全链，仍经原权限、签名、审批与审计后允许。InFlightConvergence 不允许首次/重试外发、领取新任务或任何新受限副作用。不能用 HTTP method 猜 effect：全部外部 route 与 core/worker 的 job/event/approval-owner/outbound-operation 必须用 F-56 `Fixed|ConfigRelease|McpInbound` binding 显式登记，实际入口集合与 admission registry 缺/多/重复均由 xtask 和只读静态 Blocking 自检失败；该自检不读取 current license，故不与 Restricted 可启动冲突。该码属于所有适用 operation 继承的 shared admission error，不要求在每个 route-local OpenAPI `x-error-codes` 重复列出；局部列表仍只描述进入 handler 后的精确错误闭集，catalog 校验必须把 shared 与 local 两层合并后验证，且禁止局部用其他码改写受限运行。

### 5.6 鉴权头与版本化

请求头固定集合。

| 头 | 必填 | 说明 |
|---|---|---|
| Authorization | 是 | `Bearer <opaque-session-token>`，不透明会话令牌，长度 43 的 base64url。不使用 JWT，理由是高风险操作要求即时撤销与设备绑定，自包含令牌无法在单机集中撤销点之外做到 |
| X-Legal-Entity-Id | 是 | 调用方声明的目标法人，服务端必须对照该用户与设备的授权法人集合校验后才写入安全上下文，校验失败直接拒绝 |
| X-Device-Id | 是 | 已登记设备标识，未登记设备拒绝访问业务数据 |
| X-Client | 是 | 普通 HTTP 取值 `win`、`mac`、`ios`、`android`、`portal`、`ops`、`server_admin`；`mcp` 只由 `/mcp` grant middleware 固定，外部自填无效 |
| X-Request-Id | 否 | 缺失时服务端生成，一律在响应头回显 |
| Idempotency-Key | 写请求必填 | 见第 5.4 节 |
| X-Reauth-Token | 高风险操作必填 | 重新认证凭证，单次有效，签发后 5 分钟过期，绑定待签内容摘要 |
| traceparent | 否 | W3C 追踪上下文，缺失时服务端新建 |

响应头固定回带 `X-Request-Id`、`X-Trace-Id`，弃用接口另带 `Deprecation` 与 `Sunset`。

上表必填规则的唯一认证头例外是前述 `PRE_AUTH_ENDPOINTS` 矩阵加 `GET /api/v1/platform/identity/me/legal-entities`：后者已经认证，只豁免 `X-Legal-Entity-Id`。矩阵按完整 method/path 精确匹配，不接受前缀、通配符或配置扩展；四项之外的端点缺任一必填头均直接拒绝。

版本化策略：URL 承载主版本。向后兼容的变更即新增可选请求字段、新增响应字段、新增枚举取值的接收侧，不升版本；破坏性变更即删除或重命名字段、收紧校验、改变默认值语义、删除枚举取值，必须升主版本。同时在线的主版本不超过三个，与规格第 10.3 章的当前版本及前两个主版本一致。枚举取值扩展时，客户端必须容忍未知取值并按“未知”降级展示，不得报错。

## 6. 领域事件与 Outbox

### 6.1 事件命名与载荷

事件类型为四段，形如 `<module>.<aggregate>.<past_participle>.v<major>`，全小写点分，如 `clm.contract.effective.v1`、`sales.delivery.confirmed.v1`、`inventory.stock_movement.posted.v1`、`ledger.voucher.posted.v1`、`finance.payment.registered.v1`。事件名一律用已完成时态，禁止用命令式动词。全量事件登记在 `docs/event-catalog.md`，新增事件必须先登记再实现。

载荷结构固定为信封加业务体，信封字段不得增删。

```json
{
  "event_id": "0192f3a1-...-uuidv7",
  "event_type": "sales.delivery.confirmed.v1",
  "event_version": 1,
  "occurred_at": "2026-08-10T02:11:43.512Z",
  "legal_entity_id": "…",
  "aggregate_type": "sales.delivery_confirmations",
  "aggregate_id": "…",
  "aggregate_version": 7,
  "security_level": 20,
  "data_scope_tags": ["dept:sales", "project:P-2026-0007"],
  "posting_date": "2026-08-10",
  "accounting_period_id": "…",
  "correlation_id": "…",
  "causation_id": "…",
  "idempotency_key": "…",
  "actor": { "user_id": "…", "device_id": "…", "on_behalf_of": null },
  "payload": { }
}
```

`security_level` 与 `data_scope_tags` 是规格第 7.9 章派生存储写入的必备标签，缺失的事件不得写入派生存储。`posting_date` 与 `accounting_period_id` 是规格第 5.2 章要求“记账日期随 Outbox 条目一并可读”的落点，同时是关账受理前提中“待消费过账条目数为零”的可枚举依据。

`payload` 中的字段一律 snake_case，只放该事件的最小必要数据，禁止携带行内敏感字段的明文，需要时只放引用 ID。

### 6.2 Outbox 表与投递

- 表 `platform_msg.outbox_events`，列为信封各字段加 `payload jsonb`、`status text`、`attempts int`、`available_at timestamptz`、`locked_by text`、`locked_until timestamptz`、`last_error text`、`created_at`。`status` 取 `PENDING`、`DISPATCHING`、`DONE`、`DEAD`。
- 业务状态、审计事件与 Outbox 条目写入同一数据库事务，照抄规格。禁止在事务提交前发起任何外部调用。
- 取件语句固定为 `... where status = 'PENDING' and available_at <= now() order by available_at, event_id for update skip locked limit 100`，批量 100，轮询间隔 200 毫秒，无待处理时退避到 2 秒。
- 投递语义为至少一次。消费端幂等由 `platform_msg.inbox_consumptions(consumer, event_id)` 的唯一约束保证，消费副作用与该行插入同事务。
- 重试退避固定为 1 秒、5 秒、30 秒、2 分钟、10 分钟、30 分钟、1 小时、2 小时，共 8 次，全部失败后置为 `DEAD` 并写入死信。
- 死信表 `platform_msg.dead_letters`，列含原事件全量信封与载荷、`failure_category`、`last_error`、`first_failed_at`、`state`（`OPEN`、`REPAIRING`、`REPAIRED`、`DISCARDED`）、`repaired_by`、`approval_ref`。重投必须记名并写入审计；丢弃需要双人审批。
- 死信按 `legal_entity_id` 与 `posting_date` 可枚举，直接支撑规格第 10.2 章关账受理的两项前提判定。
- 单副本下定时器天然唯一触发，但触发与执行仍必须幂等且可重放，实现不得把单副本当作前提，照抄规格。定时器条目同样带幂等键，重放时按幂等键去重。

## 7. 配置模型

### 7.1 来源与优先级

优先级由高到低固定五层，同层内后加载覆盖先加载。

1. 命令行参数，只接受 `--config <path>`、`--check`、`--version` 三个，不接受任何业务参数。
2. 环境变量，前缀 `EP__`，层级用双下划线，如 `EP__DB__POOL__RW_MAX=20`。
3. 部署片段目录 `C:\EP\config\config.d\*.toml`，按文件名字典序加载。
4. 主配置 `C:\EP\config\config.toml`。
5. 二进制内置默认值。

配置结构体用 serde 反序列化并开启 `deny_unknown_fields`，未知键一律拒绝启动，理由是拼错的配置键静默失效是私有化交付里最难排查的一类故障。

运行期可变的业务参数不进配置文件：审批链、账龄分档、提醒提前量、枚举字典、报表定义、低代码规则一律存事务数据库并经配置发布通道签名发布，照抄规格第 9.2 章。

### 7.2 敏感配置的载体

- 数据库口令、备份加密密钥、审计签名私钥与 TLS 私钥一律不出现在配置文件与环境变量中，配置只写严格版本引用 `secret://<domain>/<name>#<version>`。电子签章及 F-55 MCP connector 的零 KMS 进程凭据是唯一 `wincred://` 例外；KMS 自举前的 HSM PIN 只用 `bootstrap://`。三种 ref 是不同强类型，互不接受。
- 生产 `secrets.provider` 闭集与默认值均只有 `kms`；Stage 1 `FileSecretProvider` 仅属历史与受控迁移输入，不进入常驻产品二进制。`SecretProvider` 只做 ref/recipient/版本/载体定位，`SecretUnsealer` 才返回明文；`KmsBackend` 继续负责原数据 KMS 与字段/附件密钥，不得把三者合成一个无边界接口。终态 ABI、`EPS1` 二进制信封、AAD、大小、轮换、迁移和 release gate 以 ADR-0007 为唯一出处。
- builtin 数据 KMS 的 `C:\EP\kms\master.key` 只由 core-server 与 job-worker 使用，不得扩 ACL 给系统机密 recipient。系统机密库的六个 recipient `ep-core|ep-worker|ep-ops|ep-archive|ep-backup|ep-migrate` 各有一个独立 32-byte KEK，以 DPAPI machine scope 和绑定 deployment/recipient/key-version 的 additional entropy 封装在固定 bootstrap 路径；HSM 后端则每 recipient 使用独立 object。`secret://` 信封落在 `C:\EP\secrets\<recipient>\<sha256(ref)>.eps1`，目录与文件断继承且只授对应服务、SYSTEM、Administrators，进程启动时读回 owner/DACL/reparse/ADS/hardlink/regular-file 事实并失败关闭。云 KMS 首版不支持。
- `ep-secretctl` 是唯一 writer 与唯一 legacy reader，顶层命令闭集固定为 ADR-0007 的八项并进入 SBOM；`put` 只用无回显本机 console 二次确认，拒绝重定向 stdin/argv/env/file，只有 `migrate` 可读受控 legacy 明文文件。运行时进程只读；legacy、quarantine 或 staging 任一残留都阻止生产发布。
- 内存 secret 使用 `secrecy::SecretBytes` 或 `SecretString`，不得实现 Clone、Debug、Display 或 Serialize，禁止进入日志、错误、指标、审计或 receipt；所有临时 buffer 每条路径显式 zeroize。
- 轮换必须显式改签配置引用，运行时不监听目录、不自动追随 `latest`：新版本先 put/verify，签名配置切到明确 `#version`，全 recipient ACK 后才 retire 旧版本，最短兼容窗口 24 小时。

### 7.3 启动自检

进程启动时按序执行下列自检。每项带 severity，取值域只有 Blocking 与 Degrading 两个。Blocking 项失败即以退出码 78 退出，并把失败项写入 stderr 与 `platform_ops` 台账（数据库不可用时只写 stderr）；Degrading 项失败不阻止启动，改为经 `ep-platform-obs` 的 `DegradationLedger` 开一个降级窗口、按规格第 15.3 章持续告警、并在健康端点显式呈现该项未通过，每个 Degrading 项必须在下方各自写明运行期后果。判读运行期可变业务数据行的自检项一律取 Degrading，不得作为启动失败条件，理由是这台服务器没有备节点，用一次数据不一致换九个产品进程全部拒绝启动，是把可修复的账务偏差升级为整机停机。取 Blocking 的只有判读配置、二进制、迁移版本、密钥与目录一类装配正确性的项。`--check` 模式不适用本条降级，Blocking 与 Degrading 任一不通过均以非零码退出；部署与升级的闸门落在 `--check`，不落在进程启动。

自检项按注册名标识，不用序号称呼。注册表为 `SelfCheckRegistry`，位于 `crates/platform/runtime/src/selfcheck/registry.rs`，注册项为 `SelfCheckItem { name, title, severity, run }`，name 为 kebab-case，由阶段 1 交付。下列十项是基线项，注册顺序即执行顺序，报告按注册顺序输出，基线十项在前。不持有常规数据库连接的进程对全部 SQL 类自检项一律标 `NotApplicable`、不判成败，终态精确闭集为 `portal-gateway|integration-gateway|plugin-host|archive-writer|backup-writer|ai-inferer` 六个；其中 `ai-inferer` 由 F-55 Task 4 同批加入，非 SQL 的配置、机密、模型包、目录与资源自检仍照常执行。全部进程共有这一说法撤销。

- `config-parsed`：配置解析成功且无未知键。
- `database-reachable`：事务数据库可达，服务端版本为 PostgreSQL 16.x，`timezone` 为 UTC，`max_connections` 不低于第 2 节的峰值 52，`max_wal_senders` 不低于 4，`max_replication_slots` 不低于 3。
- `migration-version-matched`：迁移历史版本与二进制期望版本一致，不一致即拒绝启动，任何进程都不得在启动时自动执行迁移。
- `rls-enabled-and-forced`：全部带法人列的表均已 `ENABLE` 且 `FORCE` 行级安全，且运行期账号不具备 `BYPASSRLS` 与 `SUPERUSER`。
- `runtime-role-privileges-bounded`：运行期账号不具备 DDL、角色管理与策略管理权限。
- `secrets-resolvable`：只判当前进程适用的本地 ref、bootstrap、recipient 隔离、信封和 KMS/HSM 解封路径，取 Blocking；零数据库进程仍执行此项中适用的非 SQL 判定，不得因 SQL 自检 N/A 而跳过。
- `audit-chain-verifiable`（Degrading）：审计链最近一段可读，最近一次段根签名可验证，最近锚定时间在约定间隔内。不通过时登记暴露窗口并持续告警后继续启动，理由是拒绝启动不能修复断链，而修复的唯一手段恰是人工介入。
- `file-store-writable`：文件存储路径可写、不具备覆盖与原地删除权限、剩余空间不低于阈值。
- `clock-skew-within-limit`：系统时钟与授时源偏差小于 1 秒。
- `offsite-sink-requirements`（Degrading）：服务器之外落点的三项最低要求判定。不满足时不阻止启动，但以降级状态启动，并按规格第 15.3 章持续告警、记录暴露窗口、按依据枚举展示该部署当前的 RPO，暴露窗口经 `ep-platform-obs` 的 `DegradationLedger` 登记。

各阶段追加的自检项同样为 kebab-case 注册名，注册顺序排在基线十项之后，追加项名与其 severity 在各阶段计划中登记，全量清单以总览第 4.3 节 C-25 行为唯一出处。任何阶段不得新增取 Blocking 且判读业务数据行的自检项；凡能表达为写入侧约束的，一律下沉到写入路径，不在每次启动复判。任何阶段不得再以序号称呼自检项。

法人数据密钥域覆盖率不再混入 `secrets-resolvable`。阶段 2 交付独立 `legal-entity-key-domain-coverage` 的 Degrading 判读算法、trait provider 与结构化结论；它只报告缺失法人及运行期后果，只有目标法人完全没有 `key_domains` 行时返回 `PLATFORM.KEY_DOMAIN.NOT_PROVISIONED`，一旦存在 `PROVISIONING|ACTIVE` 行而 KMS/KEK/DEK/readback/16-key 矩阵或 activation audit 不可用/不一致则返回 `PLATFORM.KEY_DOMAIN.KEY_UNAVAILABLE`。由于不可改的 `V20260901104500` 与 Stage 2 Rust 枚举只有三个初始 kind，阶段 2 不注册该检查，也不调用 `DegradationLedger::open`，更不得临时复用 `PORT_NOT_IMPLEMENTED`。Stage 14a0 先把 Rust 接受域、mock 与 contract fixture 扩为终态 21 项但不向旧三值数据库写新 kind；只有 Stage 14 自有 28-file roster 与 Stage 6 后续两项组成的全局 pre-F55 30-file chain 全部具备、可按版本作为同一不可拆批次执行时，才执行其中 `V20261023092500` 把数据库 CHECK 扩为同序 21 项并部署匹配 Rust，随后注册本 provider 并真实开/关 `LEGAL_ENTITY_KEY_DOMAIN_UNAVAILABLE`；不得提前单独 apply 092500，Stage 14b 只形成真实证据/最终发布判定。该终态 kind 不可抑制。零 SQL 进程对该独立检查为 NotApplicable，但不影响上一项非 SQL 机密检查。

`--check` 模式只执行已注册的全部自检项并按注册顺序输出结构化报告后退出，不监听任何端口，用于部署验收与升级前置校验。报告按注册名索引，不按序号索引。

## 8. 测试分层与门槛

### 8.1 三层边界与工具

- 单元测试：位于被测 crate 内，`#[cfg(test)]`。边界是不触网、不触库、不触文件系统、不取真实时间。覆盖纯函数、值对象、状态机、分录映射、取价与拆分规则。工具为 cargo test、rstest 参数化、insta 快照、proptest 领域属性测试。领域属性测试是规格第 17.2 章的独立测试类型，至少覆盖借贷平衡、库存守恒、核销守恒、移动加权平均单价重算、价差拆分五组不变量。
- 集成测试：位于各 crate 的 `tests/` 与 `apps/*/tests/`。边界是使用真实 PostgreSQL 16，禁止用内存库或 mock 替代数据库，理由是 RLS、隔离级别与并发行为无法被模拟。每个测试用例独占一个数据库，用 testcontainers 启动或复用本机实例并按 `ep_test_<nanoid>` 建库，用例结束即删库。外部电子签章用 wiremock 打桩，且必须同时提供一套契约测试跑真实沙箱。
- 端到端测试：覆盖规格第 8 章闭环 14 步与第 17.2 章财务内核的十五类必测分支。后端 E2E 用 Rust 集成测试直接打 HTTP 接口，四端 UI 用 Playwright 驱动桌面 WebView 与门户 Web，用 tauri-driver 驱动桌面壳，移动端用 XCUITest 与 Espresso 只跑规格第 6.2 章矩阵中取值为完整或简化的场景。

### 8.2 覆盖率门槛

- 强制不变量相关代码与平台内核代码行覆盖率不低于 85%，其余代码不低于 70%，新增与修改代码不低于 80%，照抄规格第 17.2 章。
- 工作区整体行覆盖率不低于 80%，是本基线在规格之上追加的下限，理由是规格的分档在极端配比下可能整体低于 80%。
- 工具为 cargo-llvm-cov，CI 上以 `--fail-under-lines` 强制。分档阈值由 `codecov.toml` 中的路径规则表达，规则按目录前缀给出，即 `crates/foundation/**`、`crates/platform/**`、`crates/contract/**`、`crates/application/**`、`crates/adapter/**` 五条，不与 crate 清单逐项对应，新增 crate 自动落入所属分档。
- 不允许长期跳过或屏蔽用例，`#[ignore]` 必须带 issue 编号注释且存活不超过一个阶段。

### 8.3 测试数据构造

- 统一用 `ep-testkit` 的构造器，形如 `LegalEntityFixture::two()`、`ContractBuilder::new().with_lines(..).effective()`。禁止在测试里手写 INSERT 语句拼数据，理由是绕过用例路径构造出的数据往往违反不变量，会掩盖真实缺陷。
- 基准数据集由 `ep-datagen` 产出，接受 `--seed` 与 `--scale`，默认 scale 对应规格附录 A.3 取值：法人 2 个、命名用户 50 名、客户与供应商与物料各 5000 条、销售与采购订单行各 10 万条、库存流水 50 万条、会计分录 150 万条、附件对象 10 万个约 800 GB、期间跨度 36 个会计期间。生成器版本化并随认证结论冻结，规模不同的实测结果不作为认证结论。
- 时间一律经 `Clock` 端口注入，测试用 `FixedClock`，禁止在测试中 sleep 等待时间推进。
- 金额与数量的期望值在测试中写死为字面量，不得由被测代码反算。

### 8.4 并发与事务测试

- 业务事务隔离级别固定 `READ COMMITTED`；内部对账与关账前强制校验固定用单个 `REPEATABLE READ` 事务或由其导出的快照，照抄规格第 10.2 章。该快照的唯一取用入口是 `UnitOfWork::snapshot_transact`，见第 10.3 节。
- 序列化失败 40001 与死锁 40P01 统一在数据访问层重试 3 次，退避 50、150、450 毫秒，且只对尚未产生任何外部可见副作用的事务重试。重试次数记入指标 `ep_db_tx_retries_total`，标签为 pool 与 sqlstate，见第 9.2 节。
- 必测的并发场景固定六组：同一单据的乐观锁冲突、同一物料的并发出库与移动加权平均单价重算、同一采购订单的并发发票匹配与暂估回冲、同一客户的并发下单与信用额度占用、关账受理与在途写事务的交叠、Outbox 同一事件的重复投递不少于 3 次。
- 法人越权测试集是独立测试目标 `tests/rls_matrix`，覆盖读取、写入、更新、删除、聚合、排序、报表投影与错误信息泄漏八类，另覆盖两个复制角色与内部对账系统安全上下文的五个入口借用测试，属发布门禁项。
- 事务边界的静态检查：CI 校验 `ep-domain-*` 不出现 sqlx 符号，`ep-app-*` 的用例函数中不出现 reqwest 与文件写入符号。

## 9. 可观测性

### 9.1 日志

- 格式为 JSON Lines，一行一事件，写本地文件并自轮转。原取值「输出到 stdout，由 systemd 收集」在本平台没有承载物：Windows 服务控制管理器起的服务不继承控制台，stdout 没有采集方。本条是「不自建日志平台」一句的降级，不是换个词把那句留着：本平台自建的是日志落地与轮转一层，它带来轮转、磁盘配额与并发写入三个新失效面；不自建的只剩检索、聚合与告警三项，这三项仍不自建。不取 Windows 事件日志一支，理由是下一条的 18 个固定字段在该载体上只能塞进消息字符串，其机检性质变弱。落点在 `C:\EP\` 之下，具体子目录、轮转策略与保留量本基线不取值，随日志落地与轮转一层在阶段 1 一并定，定后按第 0 节回写本节。
- 字段固定集合，缺失即视为实现缺陷：`ts`（RFC3339 UTC 微秒）、`level`、`target`、`msg`、`process`、`version`、`trace_id`、`span_id`、`request_id`、`legal_entity_id`、`user_id`、`device_id`、`module`、`operation`、`duration_ms`、`outcome`（`ok` 或 `error`）、`error_code`、`error_category`。
- 级别语义：ERROR 表示需要人工介入；WARN 表示已降级或已重试；INFO 表示关键状态迁移与每请求一条访问日志；DEBUG 默认关闭，只能由运维临时开启且自动 30 分钟后关闭。
- 禁止进入日志的内容：会话令牌、密钥与机密、口令、银行账号、身份证号、税号、附件正文、行内敏感字段明文、完整 SQL 参数。敏感值一律经 `foundation::Redacted<T>` 包装并序列化为 `"***"`。
- 实现为 tracing 加 tracing-subscriber 的 JSON 层，span 名与 `operation` 同名，命名为 `<module>.<usecase>`。

### 9.2 指标

- 由 ops-agent 在 127.0.0.1:9101 暴露 Prometheus 文本格式，仅内网可达，可对接客户已有的 Prometheus 与 Grafana。
- 命名 `ep_<subsystem>_<metric>_<unit>`。固定的基线指标：`ep_http_request_duration_seconds`（直方图，桶为 0.05、0.1、0.25、0.5、1、2、3、5、10、30，标签 route、method、status_class、client）、`ep_db_pool_connections`（gauge，标签 pool 取 rw、ro、worker、integ、ops）、`ep_db_statement_duration_seconds`（直方图，标签 pool 与 statement_kind）、`ep_db_tx_retries_total`（counter，标签 pool 取 rw、ro、worker、integ、ops，标签 sqlstate 取 40001、40P01）、`ep_outbox_pending_events`、`ep_outbox_dispatch_attempts_total`、`ep_dead_letters_open`、`ep_archive_write_lag_seconds`、`ep_attachment_write_lag_seconds`、`ep_audit_anchor_age_seconds`、`ep_backup_last_success_timestamp_seconds`、`ep_recon_run_duration_seconds`、`ep_recon_unfinished_total`、`ep_period_close_rejected_total`、`ep_degradation_windows_open`、`ep_build_info`（gauge，标签 version 与 git_commit）、`ep_selfcheck_pending_items`（gauge，标签 process）。指标名全量登记在 `docs/metrics-catalog.md`，唯一性由 CI 校验，同一指标只能由一个阶段注册，重复登记即构建失败。
- 标签基数纪律：禁止把 `user_id`、`doc_no`、`trace_id` 作为标签；`legal_entity_id` 允许，因为首版只有 2 个法人；`route` 用模板路径而非实例路径。
- 规格第 15.3 章要求的降级与暴露窗口台账既进数据库表 `platform_ops.degradation_windows`，也各出一个 gauge，两处不得只有其一。

### 9.3 追踪

- 采用 W3C traceparent，128 位 trace id。默认只把 trace_id 与 span_id 记入日志，不外发。客户已部署 OpenTelemetry 收集器时可开启 OTLP 导出，默认关闭。
- 采样固定为：错误请求、`HighRiskOperation` 七个枚举值（六类业务高风险操作加 `DATA_MIGRATION`）、关账与对账任务一律 100%，其余 10%；trace sampler fixture 必须逐项断言这七个值均为 100%，不得只覆盖六类业务操作。
- 门户请求在 portal-gateway 新建 trace，并把公网侧的关联标识放入 `X-Correlation-Id`，不接受外部传入的 traceparent，理由是公网可控性不足，外部注入的追踪上下文会污染内部链路。

### 9.4 审计事件与业务日志的区分

这是硬边界，各阶段不得自行放宽。

- 审计事件是法律与合规证据：与业务变更同一数据库事务写入 `platform_audit.audit_events`，进入按法人与自然日的哈希链，每 5 分钟或每 1000 条对段根哈希做一次非对称签名并立即写入审计证据存储，只追加不覆盖，按 15 分钟周期写出到服务器之外落点。
- 运行日志是排障材料：按第 9.1 章写本地文件并自轮转，可丢弃，不构成证据，不得用于替代审计。
- 判定规则：凡属于“谁在何时对哪条记录做了什么”，以及审批、授权变更、重新认证、敏感导出、配置发布、密钥使用、应急账号启用、迁移账号启用与回收、两个写出进程的连接与复制槽与基础备份起止，一律写审计。其余写日志。同一事实不得只落日志。
- `platform_audit.audit_events` 的固定列：`event_id`、`legal_entity_id`、`event_day`（Asia/Shanghai 自然日，分段键）、`seq`（数据库 bigserial，段内链序的唯一串行化点，核心不持有链状态）、`prev_hash`、`hash`（SHA-256）、`actor_user_id`、`actor_device_id`、`action`、`object_type`、`object_id`、`object_version`、`before`（jsonb，敏感字段掩码）、`after`、`reason`、`approval_ref`、`reauth_ref`、`client`、`occurred_at`。客户端不建本地分段链，其事件提交到中心后由中心按同一序列写入。

## 10. 代码组织约定

### 10.1 crate 内目录

领域 crate 固定目录。

```
src/lib.rs        只做 pub use 再导出，不含逻辑
src/model/        聚合根与实体，一个聚合一个文件
src/value/        值对象
src/service/      领域服务，跨聚合的纯逻辑
src/rule/         业务规则与不变量断言
src/port/         业务端口 trait，仓储与外部能力
src/error.rs      本 crate 的 Error 枚举
```

应用 crate 固定目录。

```
src/lib.rs
src/usecase/<verb>_<noun>.rs   一个用例一个文件，含入参、出参、执行体
src/authz.rs                   本模块的授权判定入口
src/tx.rs                      工作单元的使用与事务边界
src/projection/                投影与查询模型的组装
```

文件规模纪律：单文件 200 至 400 行为常态，硬上限 800 行；函数硬上限 50 行；嵌套深度不超过 4 层，超过用早返回拆解。超限即在 CI 中失败。

### 10.2 错误类型

- 每个库 crate 定义自己的 `Error` 枚举，用 thiserror 派生，变体带足够上下文，禁止 `Error::Other(String)` 这类兜底变体。
- 库 crate 禁止依赖 anyhow，只有 `apps/*/src/main.rs` 与测试可以使用。
- 边界转换：`ep-app-*` 的用例返回 `Result<T, AppError>`，`AppError` 定义在 foundation，携带 `code`、`category`、`message`、`details`、`retryable`、`source`。各层 Error 到 AppError 的映射写在各 crate 的 `error.rs`，一处映射一次，禁止在 HTTP 层做二次翻译。
- 禁止在非测试代码中使用 `unwrap`、`expect`、`panic!`、数组越界索引与整数溢出运算，由 clippy 的对应 lint 以 `-D warnings` 强制。确实无法继续的不变量违背，先写审计事件再中止当前请求，不中止进程。
- 错误消息中的用户可见文案与错误码一一对应，集中在 `docs/error-codes.md`，代码里只引用常量，不内联中文字符串。

### 10.3 事务边界的表达方式

- 事务只在 application 层出现，唯一写法是工作单元闭包。

```rust
let result = uow.transact(ctx, |tx| async move {
    let order = repo.load_for_update(tx, order_id).await?;
    let events = order.confirm_delivery(cmd, clock.now())?;
    repo.save(tx, &order).await?;
    outbox.enqueue(tx, &events).await?;
    audit.record(tx, ctx, &events).await?;
    Ok(order.into_view())
}).await?;
```

- 审计写入必须是工作单元闭包内的最后一次数据库写入，理由是审计段行是全局串行化点且其排他锁持有到事务提交。任何阶段不得在审计写入之后再发起任何数据库写入，包括 Outbox 入队、投影回填与同事务内的站内通知写入。上面示例的次序即唯一合法次序。
- 工作单元的唯一定义是 `ep_foundation::port::UnitOfWork`，方法只有两个：读写事务用 `transact`，只读快照事务用 `snapshot_transact`，后者配合 `SET TRANSACTION SNAPSHOT` 使用，签名见第 1.4 节。任何阶段不得新增第三个方法，也不得使用 `transact_repeatable_read` 一类的旧名。
- 一个用例一个事务。禁止在一个 HTTP 请求内开启多个写事务，需要多步的一律拆用例并由 Outbox 串接。
- 事务内禁止：外部 HTTP 调用、文件正文读写、发送通知、长时计算、等待用户输入。
- 事务预算固定：业务事务不超过 5 秒，`statement_timeout` 在读写池上设 10 秒，`lock_timeout` 设 3 秒，`idle_in_transaction_session_timeout` 设 15 秒。只读分析池 `statement_timeout` 60 秒、`work_mem` 64 MB、`temp_file_limit` 2 GB。job-worker 池 `statement_timeout` 300 秒。ops 池 5 秒。迁移账号 30 分钟。
- 安全上下文在连接取用时写入会话变量，归还前清除，由连接池的 `after_connect` 与 `before_return` 钩子统一实现，业务代码不得直接调用 `set_config`。

### 10.4 领域层与适配层的分界

- domain 只依赖 foundation 与自身 contract。domain 内禁止出现 sqlx、reqwest、tokio 的 IO 模块、`std::fs`、`std::net`、`SystemTime::now`、`rand`。当前时间经 `Clock` 端口，标识符经 `IdGen` 端口，随机数经 `Rng` 端口。
- 一切 IO 表达为 domain/port 中的 trait，实现在 adapter。trait 方法只用 domain 类型与 foundation 类型，不得出现数据库行类型与 HTTP 类型。
- application 负责四件事且只负责这四件事：授权判定的调用、事务边界、领域调用的编排、审计与 Outbox 的写入。业务规则不得写在 application，查询组装不得写在 domain。
- adapter 负责映射与协议，不得包含业务分支。凡是 adapter 里出现 `if` 判断业务状态，即为分层错误。
- 装配只在 `apps/<proc>/src/wiring/` 目录下发生，按模块一个文件，构造具体实现并注入 trait 对象。该目录之外任何地方不得 `use ep_adapter_db_pg::...`，该目录之内不得出现业务分支。`xtask archcheck` 另断言 `apps/core-server/src/wiring/` 与 `apps/job-worker/src/wiring/` 两个目录下的全部文件中不出现任何以 Noop、Stub、Fake、Dummy 为前缀的实现类型或注入行，出现即构建失败。该规则名为 unwired-absent，由阶段 1 随 `xtask` 一并交付，在阶段 1 的 archcheck 规则段与退出条件中各单列一条，不并入依赖方向七条禁止项，并配一个故意违反的负样例，负样例构建必须失败。前缀集合就是这四类，`Unwired` 一名撤销；阶段 14 的发布门禁项 RG-UNWIRED-ABSENT 的扫描面同为发布制品源码树中这两个目录下的全部文件，其判据提供方一列为阶段 1 的该 archcheck 规则。各阶段计划一律按这两个目录书写装配落点，不再出现单文件措辞。

## 11. 本基线一并定死的若干全局取值

下列事项曾因规格未定义而登记在 PRD 附录乙，现已全部冻结为本节取值。开发直接采用，不等待实施方选择；未来变更须走正式规格修订并同步数据、接口和验收，不能只改实现。

### 11.1 编号规则（对应 U-A-01、U-A-02）

- 单据编号格式 `<类型码>-<法人码>-<YYYYMM>-<6 位流水>`，如 `SO-01-202608-000123`。类型码为 2 至 4 位大写字母，法人码为 2 位数字，流水按法人、类型、年月三元组独立自增，位数不足补零，溢出时位数自动扩展为 7 位。
- 单据编号不允许人工指定。档案编码允许人工指定，也可按规则自动生成，唯一性范围为法人加对象类型；按规则自动生成时沿用上一条的编号格式与取号机制，前缀取同一张类型码登记表中的类型码，人工指定的档案编码只校验唯一性与第 11.2 节的文本长度，不校验格式。
- 类型码全量登记在 `docs/data-dictionary.md` 的单据类型码一节，单据类与档案类共用同一张表，全局唯一。新增类型码必须先登记再实现，登记项与 `ep-platform-sequence` 的常量表由 CI 项 `xtask configdoc --check-doc-type-codes` 逐项比对，缺失、重复或不一致即构建失败。
- 取号在业务事务内完成，实现为 `platform_core.number_sequences` 表上的 `update ... returning`，回滚即退号，因此不产生空号。承载模块为 `ep-platform-sequence`，落点为 PRD 第 10 节。

### 11.2 文本长度（对应 U-A-03）

列类型一律 `text` 加 CHECK 长度约束，不用 `varchar(n)`，理由是放宽长度属于在线变更范围内的操作，改 CHECK 比改类型代价低。默认上限：编码 64、名称 200、简述 500、备注与原因与说明 2000、地址 500、电子邮箱 320、电话 32、条款正文与富文本 1 MB。超限在 API 层返回 `VALIDATION`，不在数据库层静默截断。

### 11.3 权限求值顺序（对应 U-B-05）

策略默认拒绝。求值顺序固定为：先判定法人授权，再判定对象级权限，再判定记录级权限，再判定字段级权限与密级。同一主体命中多条策略且允许与拒绝并存时，显式拒绝优先。理由是默认拒绝的体系里，只有拒绝优先才能使新增一条角色不会意外放大既有授权，也才能使法人越权测试集有确定结论。

### 11.4 空批次标识（对应 U-G-05）

未启用批次管理的物料，其批次列取固定值 `'-'` 而非 NULL，界面展示为“无批次”。理由是该列同时是库存台账、收发存汇总与规格第 17.3 章守恒校验的分组键，NULL 在分组与唯一约束中的语义会把同一物料拆成两组。序列号列同理，未启用时取 `'-'`。

### 11.5 列表默认值（对应 U-A-05）

默认分页 20、上限 200、导出上限 50000、默认筛选期间最近 3 个自然月、默认排序按第 5.3 节。列表默认列由各模块在 PRD 中给出，本基线只固定这五项跨模块取值。

### 11.6 会话与并发（对应 U-L-01、U-L-02）

- 会话令牌有效期 8 小时，滑动续期，空闲 30 分钟失效；单用户同时活跃会话上限 3 个，超出时最早的会话失效并写入审计。
- 最近 60 秒内有请求的不同用户数超过 20 人时，不拒绝登录、不排队、不拒绝写入；只记录指标、发出告警并把超限区间标记为性能 SLA 不适用，管理端每 5 秒刷新。单用户最多 3 个有效会话，第 4 个建立时撤销最早会话。该活跃用户规模口径与按瞬时 HTTP 请求数保护进程资源的并发闸门彼此独立，二者不得共用计数器或拒绝条件。
- 普通业务同步等待上限 8 秒，超过即转为后台任务并返回任务回执，任务完成由站内通知送达。关账、批量导入、报表导出一律按后台任务表达。编译期具名例外只有 F-55 AI compose（Tower 122 秒/内部 120 秒、独立 45-slot）和 `POST /mcp`（Tower 32 秒/协议 30 秒、独立公平全局 16-slot→connector 4-slot）；两者不占普通 20-slot，普通业务 route 不得同步等待 MCP，也不得新增第三种例外。

## 12. 各阶段必须遵守的落地纪律

- 新增一张表、一个接口、一个事件、一个错误码、一个指标之前，先在本基线对应章节登记，再实现。阶段计划中出现未登记的以上五类，评审时按不通过处理。
- 各阶段在本阶段的路由注册处一次性给出 `(CapabilityDomain, ActionClass)` 元组，取值分别为第 1.4 节两个枚举的成员，例如交付确认路由注册为 `(CapabilityDomain::SalesOrderFulfillment, ActionClass::Submit)`。不再按用例声明 `<USECASE_SCREAMING>_DOMAIN` 与 `<USECASE_SCREAMING>_ACTION` 一对常量，也不再按阶段分散到十三个 crate 的 `src/capability.rs`；`/api/v1/platform/` 下的平台路由一律取 `CapabilityDomain::PlatformAdminLowcodeOps`。`ci-probe` feature 门控的探针路由与 `/internal/v1/` 下不对四端暴露的内部端点不参与判定。`xtask configdoc` 断言每个 `/api/v1/` 路由都能解析到一个元组，缺失即构建失败。任何阶段不得在阶段内重新定义能力域码，客户端能力矩阵的运行期判定只读这两个枚举。
- 任何阶段不得新增进程、不得新增 schema、不得新增模块码、不得新增错误分类、不得新增依赖方向。
- 任何阶段不得引入第二套命名风格、第二套封套、第二套分页参数、第二套幂等机制。
- 凡阶段计划需要偏离本基线，必须在计划中单列一节写明偏离项、理由与影响范围，并同步提出本基线的修订，不得只在实现里偏离。
- 规格与 PRD 的引用一律写章节号，不写“见规格”。本基线现行正文不允许再标注“留待业务决策”；若后续正式变更引入新决策，必须先完成裁定与权威回写，未冻结前不得进入实现。

本节另立通则一条，编号第六，供各阶段计划与各门禁工具引用。

**通则第六条，判据可判定性与不可判定登记。**

- 凡写成「由 X 断言」「由 CI 强制」「由 `--check` 判定」的判据，必须在同处写明被测输入的提供方与交付阶段。
- 被测输入的交付阶段晚于判据所在阶段的，只有三种合法处置：整条推迟、换一个被测输入已存在的可判定替身、降为评审判据并登记；不得留第四种。
- 不可判定既不得表达为通过也不得表达为违反：工具须单列输出并以专用退出码结束，CI 不得把该退出码当作通过；亦不得以「计数照旧」或「两个空集合比对」的形态退化为恒真。
- 判据重新生效的触发谓词必须由判定工具自身可观测，不得写成阶段号，也不得写成任何需要人工翻牌的动作。
- 凡降为评审判据的，提交必须在说明中按 文件：行号 给出举证，缺举证的提交评审时按不通过处理。

### 12.1 不可判定与降级判据登记表

本表是上一条通则的机械承接方，分 delegated 与 undecidable 两段，各五列，列序与列数不得改，两段不得合并成一张表，也不得增列第六列。两段由阶段 1 交付的 `xtask archcheck` 规则 undecidable-registry-matched 逐行读取，与该工具运行期输出的 delegated、undecidable 两个集合逐行比对，多一条或少一条均判违反并以退出码 1 结束；该比对的被测输入只有本表与工具自身的输出两项，均在阶段 1 内已存在，因此完全可判定。`xtask archcheck` 的退出码定死为三态：机检面全绿为 0，有违反或本表比对不符为 1，仍存在真正不可判定项为 3；三者互不合并，CI 不得把 3 读作通过。

delegated 段登记已裁定不由工具执行的判据，属永久登记，每行必须点名承接的替身规则。

| 判据名 | 所在文件与小节 | 理由 | 承接方 | 重新生效或删除条件 |
|---|---|---|---|---|
| foundation-no-business/necessity | 00b-technical-baseline.md 第 1.3 节禁止项第六条 | 判据数的是跨 crate 源码引用计数，而 `ep-contract-*` 与 `ep-platform-*` 在骨架期恒为零，工具无引用可数 | 不由本工具判定。承接方：评审举证，加 foundation-no-business/no-internal-dep、foundation-frozen-items、foundation-marker-shape、foundation-module-registry、foundation-no-single-owner 五条替身 | 永久降级，不因阶段推进而恢复；删除本行须先撤销裁定 F-03 |
| db-pg-one-schema-per-file/analyst-ro-connection | 00b-technical-baseline.md 第 1.3 节禁止项第七条 | 判据要断言的是运行期取数连接取哪个数据库角色，而 db-pg-one-schema-per-file 只在源码的双引号字面量里取 `<schema>.<object>`，任何静态规则都读不出运行期连接的角色 | 不由本工具判定。承接方：阶段 11 的 reporting-dataset-signature-matched 启动自检，加评审举证 | 永久降级，不因阶段推进而恢复；删除本行须先撤销裁定 F-05 |

undecidable 段登记当前无法执行的判据，属临时登记。本段当前为空，一行也没有。新增一行必须同时给出重新生效或删除条件，且该条件的触发谓词必须由 `xtask archcheck` 自身可观测，不得写成阶段号。本段的条目数由一条 CI 断言保证只减不增并配负样例，并由阶段 14 的发布门禁项 RG-NO-UNDECIDABLE 在发布制品源码树上断言归零；该纪律与阶段 1 计划第 13 节假设二对 Pending 自检项的只减不增与最后一个阶段归零是同一形态，不另立第二套。

| 判据名 | 所在文件与小节 | 理由 | 承接方 | 重新生效或删除条件 |
|---|---|---|---|---|
