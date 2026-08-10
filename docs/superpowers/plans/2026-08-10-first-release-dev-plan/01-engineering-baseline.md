## 阶段 1 工程基座与 CI

本阶段是全部后续阶段的地基。它不实现任何业务规则，不建任何业务表，不产生任何会计分录。凡涉及账务的内容一律指向规格第 5.2 章事件-分录表，本阶段不复述借贷与取价，也不预先实现其中任何一条规则。本阶段的判定标准只有一条：把共享技术基线里已经定死的每一条约定，变成可以在流水线上自动检出违反的机器判定，并交付一套可运行、可停机、可重启、可验签的空壳部署。

### 1. 本阶段的范围边界

在范围内的：Cargo workspace 与全部 crate 骨架、八个进程的空壳二进制、进程运行时装配、配置模型与启动自检框架、统一封套与错误映射、集群引导脚本的目录与执行顺序约定、迁移目录骨架、迁移静态检查、`tools/ep-migrate` CLI 骨架与退出码约定、ep-foundation 的跨阶段冻结类型与常量、连接池与会话变量注入清除、测试分层与覆盖率门禁、结构门禁与依赖方向门禁、容器与单机编排骨架、cgroup 资源限额取值与一次性部署校验、供应链门禁与可复现构建、制品与版本号、本地开发环境。

明确不在范围内的：集群引导五个脚本的内容、24 个 schema 与七个功能角色与 24 个属主角色、逐 schema 的默认权限、单一全局迁移 Runner 与其版本号断言、`ep-migrate` 五个子命令的实现，这五项按 C-01 与 C-02 归阶段 2；另有 Tauri 四端真机 PoC 与任何客户端代码，RLS 业务表，身份认证与授权判定，Outbox 消费与审计链，KMS 与信封加密，附件正文读写，电子签章对接，任何模块的领域模型。规格第 19 章的阶段 1 含四端 PoC 与安全密码抽象，本 14 阶段划分把它们分别归入客户端阶段与安全阶段，本阶段只交付它们的挂载点，不交付实现。
本阶段与 T0 贯通线的关系。T0 是在阶段 4 结束后、阶段 5 全量开工之前执行的一条最薄贯通线，判据是一条合同从建单走到管理层看到一个数，其业务切片取自阶段 5、6、9a、10、11。本阶段不贡献任何业务切片，只交付 T0 赖以判定的三样手段：`xtask e2e --profile=t0` 这一条独立的端到端目标、`ep-datagen` 的 `t0-min` 最小样本档、以及 `deploy/` 下一条命令起全栈的单机编排。T0 只要求桌面端可达，不要求 scale 数据集，不要求分支覆盖，不要求四端。本阶段其余交付物不因 T0 增删一项，阶段 5 至 11 一律改为在 T0 贯通后的骨架上加厚，M7 保留为全分支闭环而不再是首次贯通。


### 2. 交付物清单

本阶段结束时，下列东西必须存在且可运行，逐项可由他人在一台干净的 Linux 机器上复现。

| 编号 | 交付物 | 可运行的判定方式 |
|---|---|---|
| D-01 | 单一 Cargo workspace，含全部 crate 骨架，`cargo build --workspace --locked --offline` 成功 | 构建返回 0，无 warning |
| D-02 | 八个空壳进程二进制，各自可启动、可健康、可优雅停机 | `--check` 返回 0，健康端点返回 200，SIGTERM 后 30 秒内退出码 0 |
| D-03 | `tools/ep-migrate` CLI 骨架与退出码约定，五个子命令为 apply、status、check、gen-rls、open-window，子命令实现由阶段 2 交付 | 五个子命令的参数解析可运行，退出码 0 成功、2 参数错误、3 迁移窗口未打开、4 校验和不符、5 版本不一致、78 环境自检失败各有一个用例 |
| D-04 | 集群引导脚本的目录约定与执行顺序约定，文件名为 `db/bootstrap/00_database.sql`、`01_roles.sql`、`02_cluster_params.sql`、`03_role_defaults.sql`、`04_pg_hba.fragment`，脚本内容由阶段 2 交付 | 目录与文件名约定被 `xtask sqlcheck` 断言，自检项 rls-enabled-and-forced 与 runtime-role-privileges-bounded 的代码路径以测试库探针表为被测对象通过 |
| D-05 | 单机编排骨架，`deploy/` 下的 Podman Quadlet 与 Docker Compose 两套等价文件，含八个 slice 与配额 | 一条命令起全栈，`systemctl status` 全部 active |
| D-06 | `deploy/` 下八个 slice 的静态资源限额 drop-in 文件与一次性部署校验脚本 `scripts/verify-resource-limits.sh` | drop-in 取值与规格第 13.1 章配额表逐行一致，脚本在部署与升级时各执行一次并返回 0，任何进程的启动自检中不出现资源限额项 |
| D-07 | 一条绿色 CI 流水线，共 11 个阶段，全部门禁可离线执行 | 全量运行不超过 60 分钟，返回 0 |
| D-08 | 结构门禁工具 `xtask`，含 archcheck、sqlcheck、codecheck、errorcodes、eventcatalog、configdoc、coverage、sbom、sign、reproduce、e2e 十一个子命令 | 每条规则有一个故意违反的负样例，负样例必须失败 |
| D-09 | `ep-testkit` 测试夹具库、`ep-datagen` 数据集生成器骨架与其 `t0-min` 最小样本档、`xtask e2e --profile=t0` 目标 | 同一 seed 两次生成结果字节一致；`t0-min` 生成一个法人一个客户一个产品的最小样本；`--profile=t0` 作为独立目标可执行，本阶段用例集为空并返回 0 |
| D-10 | 覆盖率门禁，按路径分档强制 | 低于门槛即失败，有负样例证明 |
| D-11 | 制品与升级包，含八个进程镜像、迁移镜像、SBOM、签名、校验清单、回退说明 | 客户侧 `verify-release.sh` 在无网络环境下验签通过 |
| D-12 | 可复现构建证据 | 两次独立构建的二进制 SHA-256 与镜像 digest 全部相同 |
| D-13 | 本地开发环境，一条命令起 PostgreSQL 16 与全栈 | 新机器从零到跑通集成测试不超过 30 分钟 |
| D-14 | 文档骨架，含 ADR 目录、错误码表、事件目录、指标目录、配置参考、数据字典，其中数据字典含单据类型码一节 | 六份文件存在且被 CI 校验与代码一致，`xtask configdoc --check-doc-type-codes` 断言单据类型码一节与 `ep-platform-sequence` 的常量表逐项一致且无重复 |
| D-15 | 阶段 1 性能回归基线文件 | 五项取值有实测记录，后续阶段以此比对 |

### 3. crate 与进程归属

#### 3.1 本阶段建立的 crate

全部 crate 一次建齐目录与 `Cargo.toml`，其中本阶段写入实质代码的只有下表标注为实现的项，其余只建骨架，`lib.rs` 仅含 `pub use` 与一条编译期断言注释，不留 `todo!()`。理由是先把依赖方向门禁的判定面铺满，后续阶段新增文件不会绕过门禁。

| crate | 本阶段状态 | 装配进这些进程 |
|---|---|---|
| ep-foundation | 实现 | 全部八个 |
| ep-platform-runtime（本阶段新增，见第 13 节） | 实现 | 全部八个 |
| ep-platform-obs | 实现 | 全部八个 |
| ep-platform-tenancy、identity、authz、meta、flow、audit、outbox、sequence、notify、license、release、file、recon | 骨架 | 不装配 |
| ep-contract-<15 个模块> | 骨架 | 不装配 |
| ep-domain-<15 个模块> | 骨架 | 不装配 |
| ep-app-<15 个模块> | 骨架 | 不装配 |
| ep-adapter-db | 实现 | core-server、job-worker、integration-gateway、ops-agent |
| ep-adapter-db-pg | 实现 | 同上，且只在各进程的 wiring.rs 中出现 |
| ep-adapter-file、kms、queue、search、doc、esign、wasm | 骨架 | 不装配 |
| ep-adapter-ipc | 实现 | core-server、plugin-host、archive-writer、backup-writer |
| ep-testkit | 实现 | 仅 dev-dependencies |
| ep-datagen | 实现骨架 | 独立二进制，不属于八进程 |
骨架 crate 中有三处落点在本阶段就写死，后续阶段只补内容不改位置。第一处，`ep-foundation` 下的 `src/port/search.rs` 与 `src/port/doc.rs` 两个文件本阶段只建空文件并写模块注释，检索端口的类型与 trait 按 A-07 由阶段 3b 补齐，文档与打印端口按 A-08 由阶段 5 补齐。第二处，`ep-adapter-db` 提供 `PgUnitOfWork` 与 `PgTx` 两个实现类型的声明位，实现体落在 `ep-adapter-db-pg`。第三处，跨 crate 取具体事务句柄的唯一写法是 `tx.as_any_mut().downcast_mut::<PgTx>()`，`xtask archcheck` 断言 `crates/adapter/db-pg/` 之外的任何目录都不出现 `downcast_mut::<PgTx>`。

#### 3.2 本阶段八个进程各自的空壳内容

| 进程 | 本阶段实现的内容 | 本阶段不实现的内容 |
|---|---|---|
| core-server | 8080 HTTP 服务器与五个系统端点、`/run/ep/ipc/core.sock` IPC 服务端、rw 与 ro 两个池、并发闸门、同步等待上限、六项自检、优雅停机 | 任何业务路由、鉴权判定、幂等存储 |
| job-worker | 8081 健康与指标、任务调度器骨架与零个已注册任务、worker 池、200 毫秒到 2 秒的退避轮询空转 | Outbox 消费、通知投递、对账 |
| portal-gateway | 8090 HTTP、不建数据库连接、经回环调用 core-server 健康端点的上游探测、新建 trace 与 X-Correlation-Id | 门户业务页面、会话、脱敏投影 |
| integration-gateway | 8082 健康与指标、出网客户端骨架含超时退避熔断、出网白名单校验、独立池 5 | 电子签章协议、证据固化 |
| plugin-host | `/run/ep/ipc/plugin.sock` IPC 服务端、零数据库连接 | WASM 宿主；`wasmtime` 与 `wasmtime-wasi` 两个依赖本阶段一律不登记，也不留默认关闭的 feature 与编译缓存目录约定，由阶段 13b 在交付宿主时一次引入 |
| ops-agent | 9101 Prometheus 文本、9102 健康聚合、ep_ops_ro 池 2、按回环抓取其余七个进程的指标端点 | 运维台账读取、降级窗口 |
| archive-writer | 无监听、spool 目录、IPC 客户端、15 分钟周期心跳占位、core-server 不可用时落 spool 并在恢复后补写 | 事务日志归档、附件写出、审计证据写出 |
| backup-writer | 无监听、spool 目录、IPC 客户端、每日周期心跳占位 | 全量备份、校验、存量搬运 |

八个二进制 crate 名与进程名、systemd 单元名、cgroup slice 名一一对应，由 `xtask codecheck` 断言。archive-writer 与 backup-writer 在本阶段就不持有运行期应用账号，其配置结构体中根本不存在 db 段，配置里出现 db 段即启动失败，这是把规格第 7.7 章的账号边界前移到类型层。

### 4. 数据库变更

本阶段不新建任何业务表，也不交付集群引导脚本的内容与 24 个 schema。按 C-01 与 C-02，集群引导的五个文件、24 个 schema 与角色、逐 schema 的默认权限、单一全局迁移 Runner 与其版本号断言以及 `ep-migrate` 五个子命令的实现全部归阶段 2，阶段 2 是这些东西的唯一提供方。本阶段自己的数据库相关交付只有三类：迁移目录骨架、迁移历史表的参数固定与白名单、一张仅存在于测试库中的探针表，探针表不进生产迁移目录。

#### 4.1 集群引导与 schema 权限，本阶段只交付约定，内容归阶段 2

按 C-01，集群引导的五个文件 `db/bootstrap/00_database.sql`、`01_roles.sql`、`02_cluster_params.sql`、`03_role_defaults.sql`、`04_pg_hba.fragment`，以及 24 个 schema 的创建、七个功能角色与 24 个属主角色、逐 schema 的默认权限、public 权限回收、`postgresql.conf` 与 `pg_hba.conf` 模板的取值，全部由阶段 2 交付，阶段 2 计划第 3.1 与 3.2 节是这些内容的唯一出处。本阶段原拟的 `B001__cluster_roles.sql`、`B002__database.sql`、`B003__postgres_conf.sql` 三个文件名作废，不得再出现在任何目录与文档中。

本阶段在这一块只交付三件事。一是上述五个文件名与其执行顺序的约定，写入 `db/bootstrap/README.md` 并由 `xtask sqlcheck` 断言目录中不出现约定之外的文件名。二是 `xtask sqlcheck` 规则 SQL-020，即引导文件中不得出现任何口令字面量，口令由安装器从机密库读取后经 `ALTER ROLE ... PASSWORD` 单独注入，该规则本阶段实现并配负样例，被检对象由阶段 2 写入。三是本阶段拟定的 collation 取值、public schema 处置与实例参数取值，作为第 13 节的新增决定二与新增决定三继续有效，由阶段 2 在其脚本中落地。原属本阶段的运行期账号不授予 DELETE 一条，按 C-01 一并移交阶段 2，见第 13 节新增决定四。

#### 4.2 迁移目录骨架

按 C-01，本阶段不交付任何迁移文件。原拟的三个迁移文件连同其内容一并移交阶段 2，其中运行期账号在 22 个 schema 上不授予 DELETE 一条见第 13 节新增决定四。

本阶段只交付一件东西：`db/migrations/` 下 24 个空目录，目录名与基线第 1.2 节的 schema 名一一对应。目录只表达归属不表达先后，迁移执行顺序由阶段 2 交付的单一全局 Runner 按文件版本号全序排定，本阶段不交付任何顺序声明文件。

迁移文件必须以 `-- rollback:` 段开头这一纪律由 `xtask sqlcheck` 在本阶段实现并配负样例，被检对象自阶段 2 起产生。迁移历史表的存在性保证由 `ep-migrate apply` 承担，实现归阶段 2，本阶段只在 CLI 骨架中固定其 schema 与表名参数，使自检项 migration-version-matched 的比对在空库上同样成立。

#### 4.3 迁移历史表

表名 `platform_core.schema_history`，全库只有一张，由 refinery 0.8 在阶段 2 首次执行 `ep-migrate apply` 时创建，结构如下，任何阶段不改其结构。本阶段只做两件事：在 `ep-migrate` CLI 骨架中固定其 schema 与表名参数，以及把该表列入 `xtask sqlcheck` 的白名单。

| 列 | 类型 | 约束 |
|---|---|---|
| version | int4 | 主键 |
| name | varchar(255) | 非空 |
| applied_on | varchar(255) | 存 RFC3339 字符串 |
| checksum | varchar(255) | 非空 |

这四张列表由工具定义，不套用基线第 4 节的公共列，属工具自带元数据表，在 `xtask sqlcheck` 中列入白名单，白名单只有这一项。

#### 4.4 测试专用探针表，不进生产迁移目录

为了在本阶段就把基线第 3.8 节的 RLS 模板、第 4 节的公共列、第 3.7 节的乐观锁与第 3.10 节的索引命名全部跑通，`ep-testkit` 在每个临时测试库中创建 schema `ci_probe` 与下表。它不出现在 `db/migrations/` 下，不进任何交付制品，`xtask sqlcheck` 规则 SQL-030 断言 `ci_probe` 字样不出现在生产迁移目录中。
按 B-01，探针 schema 与探针表的建表函数一律带 `#[cfg(feature = "ci-probe")]`，Cargo feature 名固定为 `ci-probe`，在 `apps/core-server/Cargo.toml` 与 `testkit/Cargo.toml` 中声明且默认关闭。发布制品中不得出现该 feature 与探针表，判据由阶段 14 的发布门禁项 `RG-CI-PROBE-ABSENT` 承担，即发布制品的 `cargo tree -e features` 输出中不含 `ci-probe`，且镜像内不含符号 `api_v1_system_echo`。

| 列 | 类型 | 可空 | 默认 | 约束 |
|---|---|---|---|---|
| id | uuid | 否 | 无 | pk_probe_records |
| legal_entity_id | uuid | 否 | 无 | RLS 判据 |
| security_level | smallint | 否 | 20 | ck_probe_records_security_level check in (10,20,30,40) |
| data_scope_tags | text[] | 否 | '{}' | 无 |
| row_version | bigint | 否 | 1 | 无 |
| created_at | timestamptz | 否 | now() | 无 |
| created_by | uuid | 否 | 无 | 无 |
| updated_at | timestamptz | 否 | now() | 无 |
| updated_by | uuid | 否 | 无 | 无 |
| doc_no | text | 否 | 无 | ux_probe_records_legal_entity_id_doc_no |
| status | text | 否 | 无 | ck_probe_records_status check in ('DRAFT','EFFECTIVE','VOID') |
| note | text | 是 | 无 | ck_probe_records_note_len check length ≤ 2000 |

索引：`pk_probe_records`、`ix_probe_records_legal_entity_id_created_at`、`ux_probe_records_legal_entity_id_doc_no`，与基线第 3.10 节的三条基线索引一一对应。

RLS 策略按基线第 3.8 节模板原样生成，不写变体。

```sql
alter table ci_probe.probe_records enable row level security;
alter table ci_probe.probe_records force row level security;
create policy rls_probe_records_le on ci_probe.probe_records
  using (legal_entity_id = nullif(current_setting('app.legal_entity_id', true), '')::uuid)
  with check (legal_entity_id = nullif(current_setting('app.legal_entity_id', true), '')::uuid);
```

该表的唯一用途是让 `tests/rls_matrix` 的八类越权断言在阶段 1 就有被测对象，且让 RLS 模板生成器有一个可比对的黄金输出。业务表按此模板由后续阶段各自生成。

### 5. 领域模型与关键算法

本阶段没有业务领域模型。下列是 foundation 与 runtime 两层的核心类型、状态机与算法。

#### 5.1 foundation 的核心类型

| 类型 | 定义要点 | 边界条件 |
|---|---|---|
| Id\<T\> | uuid::Uuid 加 PhantomData\<T\>，serde 序列化为字符串，禁止不同 T 之间互转 | 解析失败返回 VALIDATION |
| Money | 内含 Decimal，构造时断言 scale 恰为 2，超出即构造失败 | 绝对值上限 10^16，超出返回错误而非截断 |
| Amount | 全精度中间值，不可直接写库，只能经 `to_money()` 一次性 round | round 策略固定 MidpointAwayFromZero，全工作区只有这一个 round 入口 |
| UnitPrice | scale 恰为 6 | 负值允许，零值允许 |
| Quantity | scale 恰为 6 | 负值允许，供冲销使用 |
| Rate | scale 恰为 6，语义为小数 | 13% 即 0.130000，构造时不接受 13 这样的百分数写法 |
| AccountingPeriodRef | Id\<AccountingPeriod\> 的别名与展示格式 | 本阶段只有类型，无期间逻辑 |
| SecurityLevel | 枚举 10/20/30/40，序列化为数字 | 未知取值反序列化失败 |
| SecurityContext | 按 A-03 冻结的 19 个字段，顺序为 user_id、account_kind、session_id、legal_entity_id、device_id、client、clearance_level、roles、duty_classes、department_scope、position_ids、project_scope、customer_scope、record_shares、data_scope_tags、snapshot_version、is_breakglass、request_id、trace_id，位于 `crates/foundation/src/security/context.rs` | 构造函数只有 `human` 与 `system` 两个，后者用 SYSTEM_PRINCIPAL_ID 与 SYSTEM_DEVICE_ID 填 user_id 与 device_id 且 account_kind 取 System；不提供任何 with_ 前缀的变换方法；字段不得增删改名 |
| AccountKind、ClientKind、DepartmentScope | `AccountKind { Human, System, Portal }`；`ClientKind { Win, Mac, Ios, Android, Portal, Ops }`，序列化取值与基线第 5.6 节 X-Client 头一一对应；`DepartmentScope { All, Subtree(Id<Department>), Explicit(Arc<[Id<Department>]>) }` | 未知取值反序列化失败 |
| DeviceId、RoleCode、DutyClass、RecordShare、DataScopeTag、RequestId、TraceId | 按 A-03 与第 13 节新增决定十二冻结，与 SecurityContext 同在 `crates/foundation/src/security/context.rs`：`DeviceId(Arc<str>)` 取长度 1 至 64 的 `[A-Za-z0-9_-]` 且可由 `&'static str` 无损构造；`RoleCode(Arc<str>)` 取长度 1 至 64 的 `[A-Z0-9_]`；`DutyClass { System, Data, Security, Audit, Key, Config }` 的序列化取值为 SYSTEM、DATA、SECURITY、AUDIT、KEY、CONFIG；`RecordShare { object_type: Arc<str>, object_id: uuid::Uuid, grant: RecordShareGrant }` 与 `RecordShareGrant { Read, Write }`，object_type 取 `<module>.<table>` 小写下划线形态并与事件信封 aggregate_type 同形；`DataScopeTag(Arc<str>)` 取 `<kind>:<value>` 形态，kind 为 `[a-z0-9_-]`、value 为 `[A-Za-z0-9_-]`，总长上限 128；`RequestId(Arc<str>)` 取长度 8 至 64 的 `[A-Za-z0-9_-]`，服务端自生成时取 UUIDv7 的无连字符十六进制；`TraceId(Arc<str>)` 取 32 位小写十六进制，与 W3C trace-context 的 trace-id 同形 | 七者只承载取值，不含任何判定逻辑，`RecordScope` 与 `RecordPredicate` 留在 ep-platform-authz 不前移；不合形态的字符串构造失败并返回 VALIDATION，未知枚举取值反序列化失败；`Arc<[DutyClass]>` 允许为空数组，`platform_authz.roles.duty_class` 为空的业务角色不产生条目，不设 None 变体；`DataScopeTag` 的序列化输出即公共列 `data_scope_tags text[]` 与事件信封 `data_scope_tags` 的元素形态，两处不得各自编解码 |
| AppError | code、category、message、details、retryable、incident_no、occurred_at、advice、source | Display 不输出 source 链，避免内部信息外泄 |
| DomainEvent | 基线第 6.1 节信封字段的强类型表达，payload 为泛型 | 信封字段增删会导致编译失败，事件目录不一致由 CI 检出 |
| Redacted\<T\> | Debug 与 Display 均输出 `***`，serde 序列化为 `"***"` | 任何 secrecy 之外的敏感值统一包这一层 |
| Tx、SnapshotCtx、UnitOfWork | 按 A-01 冻结，位于 `crates/foundation/src/port/tx.rs`。`Tx` 含 tx_id、isolation、legal_entity_id、as_any_mut 四个方法；`SnapshotCtx` 含 snapshot_id、taken_at、legal_entity_id、as_any 四个方法；`UnitOfWork` 只有 `transact` 与 `snapshot_transact` 两个方法；另含 `TxId` 与 `IsolationKind { ReadCommitted, RepeatableReadSnapshot }` | 契约层的跨模块方法签名一律写 `&mut dyn Tx`；`UnitOfWork` 不带池参数，一个实例在装配时绑定一个池；该 trait 含泛型方法不满足对象安全，application crate 对它取泛型参数 `U: UnitOfWork` 而不是 trait 对象；任何阶段不得改动这三者的签名 |
| id::marker | `crates/foundation/src/id/marker.rs`，22 个零大小标记类型，清单为 LegalEntity、UserAccount、Session、Department、Position、Project、Customer、Supplier、Material、Product、Warehouse、Contract、ContractLine、SalesOrder、SalesOrderLine、DeliveryConfirmation、DeliveryConfirmationLine、PurchaseOrder、GoodsReceiptLine、PurchaseInvoice、PurchaseInvoiceLine、AccountingPeriod | 无字段、无方法、无 trait 实现，只承载类型身份，供 `Id<T>` 在契约层表达跨模块引用；清单固定 22 项，任何阶段不得增删，见第 13 节偏离二 |
| SYSTEM_PRINCIPAL_ID、SYSTEM_DEVICE_ID | `crates/foundation/src/principal.rs`，取值分别为 `00000000-0000-7000-8000-000000000001` 与 `SYSTEM` | 取值符合 UUIDv7 的版本位与变体位校验且不可能与 IdGen 生成值碰撞；各阶段在种子迁移与系统上下文写 created_by 时一律引用该常量，不得自选取值 |
| ModuleCode | 按基线第 1.2 节 15 个模块码冻结的枚举，取值为 Mdm、Crm、Cpq、Clm、Sales、Procure、Inventory、Costing、Project、Service、Finance、Ledger、Invoice、Portal、Reporting | 未知取值反序列化失败；许可、对账、跨模块来源标注一律引用该枚举 |
| CapabilityDomain、ActionClass | `crates/foundation/src/capability.rs`，`CapabilityDomain` 18 项，序列化取值与阶段 13 第 4.4 节能力域码表的 18 个字符串逐字一致且顺序与该表序号一致；`ActionClass { Read, Write, Submit, Approve, Export }` | 本阶段只定义枚举，不做任何运行期判定；各阶段按裁定 A-20 的两类落点，在承载该路由处理器的 crate 的 `src/capability.rs` 中为每个用例声明 `<USECASE_SCREAMING>_DOMAIN` 与 `<USECASE_SCREAMING>_ACTION` 一对常量，业务模块的路由落 `crates/contract/<module>/src/capability.rs`，`/api/v1/platform/` 下的平台路由落 A-20 逐阶段指名的 platform crate 的 `src/capability.rs` 并一律取 `CapabilityDomain::PlatformAdminLowcodeOps`，不设第三类落点；`ci-probe` feature 门控的探针路由与 `/internal/v1/` 下不对四端暴露的内部端点不参与判定，不声明常量；`xtask configdoc` 断言每个 `/api/v1/` 路由都能解析到一对常量，缺失即构建失败 |

端口 trait：`Clock`（now 与 today_cn）、`IdGen`（new_id）、`Rng`（fill_bytes）、`IncidentNoGen`（next）。domain 层禁止绕过这四个端口，由 `xtask archcheck` 的符号禁令强制。另在 `crates/foundation/src/port/` 下建 `search.rs` 与 `doc.rs` 两个空文件，本阶段只写模块注释：按 A-07，`SearchDocument`、`SearchQuery`、`SearchHit`、`SearchIndexPort`、`SearchQueryPort` 由阶段 3b 补齐；按 A-08，`SheetSpec`、`ColumnSpec`、`CellValue`、`PrintLayout`、`SpreadsheetPort`、`DocTemplatePort`、`PdfRenderPort` 由阶段 5 补齐。两个文件的路径在本阶段固定，后续阶段只补内容不改位置。

#### 5.2 UUIDv7 生成算法

输入是 Clock 与 Rng。步骤：取当前 Unix 毫秒 t；若 t 大于上次生成的 last_t，则 seq 归零并设 last_t 等于 t；若 t 等于 last_t，则 seq 自增，seq 达到 4096 时自旋等待到下一毫秒；若 t 小于 last_t，即时钟回拨，则保持 last_t 不变并按 seq 自增继续生成，同时记 WARN 与指标，绝不生成时间戳倒退的 ID。输出为 48 位毫秒时间戳、4 位版本、12 位 seq、2 位变体、62 位随机。边界条件：单进程单毫秒上限 4096 个 ID，20 并发下远未触及；回拨超过 1000 毫秒时另触发自检项 9 的告警路径。

#### 5.3 关联编号生成算法

格式固定为 `ERR-YYYYMMDD-NNNNNN`，日期取 Asia/Shanghai 自然日。本阶段的 NNNNNN 由进程序号乘 100000 再加当日进程内序号对 100000 取模构成，进程序号固定为 core-server 为 1、job-worker 为 2、portal-gateway 为 3、integration-gateway 为 4、plugin-host 为 5、ops-agent 为 6、archive-writer 为 7、backup-writer 为 8。八个进程各占一个十万段，合计不超过六位。跨自然日归零。这样在没有共享序列表的阶段 1 也不会跨进程撞号，后续阶段可把实现替换为数据库序列而不改格式。

#### 5.4 进程生命周期状态机

这是本阶段唯一的状态机，也是 foundation 中 `state_machine!` 宏的第一个使用者。

状态：Init、Configuring、SelfChecking、Ready、Degraded、Draining、Stopped、Failed。

| 起始 | 事件 | 目标 | 守卫条件 |
|---|---|---|---|
| Init | Start | Configuring | 无 |
| Configuring | ConfigLoaded | SelfChecking | 配置解析成功且无未知键 |
| Configuring | ConfigInvalid | Failed | 以退出码 78 退出 |
| SelfChecking | AllPassed | Ready | 全部已注册的 Blocking 项通过，且无 Degrading 项未通过 |
| SelfChecking | PassedWithDegradation | Degraded | 全部 Blocking 项通过，且至少一个 Degrading 项未通过 |
| SelfChecking | AnyFailed | Failed | 任一 Blocking 项失败，以退出码 78 退出 |
| Ready | DegradationDetected | Degraded | 运行期检出降级条件 |
| Degraded | DegradationCleared | Ready | 条件消除 |
| Ready 或 Degraded | Sigterm | Draining | 停止接收新请求 |
| Draining | DrainComplete | Stopped | 在途请求归零或超过 drain 上限，退出码 0 |
| 任意 | Panic | Failed | 捕获后先写日志再退出，退出码 70 |

非法迁移一律返回 BUSINESS_CONFLICT 并记 ERROR，不 panic。Failed 状态下 systemd 以 `RestartPreventExitStatus=78` 不重启，避免配置错误导致重启风暴；退出码 70 允许重启。

#### 5.5 启动自检算法

自检项以注册表实现，注册表为 `SelfCheckRegistry`，位于 `crates/platform/runtime/src/selfcheck/registry.rs`，每项是一个 `SelfCheckItem { name, title, severity, run }`，name 为 kebab-case，severity 的取值域由本阶段定死为 Blocking 与 Degrading 两值，不设第三值。Blocking 项只判读二进制、环境、目录与数据库元数据，失败即以退出码 78 拒绝启动；Degrading 项失败不阻止启动，进程进入 Degraded 并由承接阶段登记降级窗口。任何阶段不得注册判读业务数据行的 Blocking 项，业务数据的一致性由规格第 10.2 章的对账组件与降级窗口承接，不由启动闸门承接。按 C-25，自检项一律按注册名标识，本计划与后续阶段都不再用序号称呼，基线第 7.3 节的十三项编号列表同步改为十项命名列表，见第 13 节新增决定十三。十个基线项中 `config-parsed`、`database-reachable`、`migration-version-matched`、`rls-enabled-and-forced`、`runtime-role-privileges-bounded`、`secrets-resolvable`、`file-store-writable`、`clock-skew-within-limit` 八项为 Blocking，`audit-chain-verifiable` 与 `offsite-sink-requirements` 两项为 Degrading。报告按注册顺序输出，基线十项在前，各阶段追加的命名项在后。本阶段实现其中六项，另三项以 Pending 登记，`offsite-sink-requirements` 本阶段不登记。

`config-parsed`，配置解析成功且无未知键，由 serde 的 deny_unknown_fields 与分层加载器返回。

`database-reachable`，数据库可达且服务端版本为 16.x，`timezone` 为 UTC，`max_connections` 不低于 52，`max_wal_senders` 不低于 4，`max_replication_slots` 不低于 3；不建库连接的四个进程 portal-gateway、plugin-host、archive-writer、backup-writer 对全部需要 SQL 会话的自检项一律跳过并标注 NotApplicable，不止本项，其中两个写出进程按规格第 7.7 章只持 REPLICATION 属性，任何 SQL 类自检项对它们都不成立，基线第 7.3 节所称十三项为全部进程共有一句同步作废。

`migration-version-matched`，迁移清单一致。算法：对全部 24 张历史表读出 (schema, version, name, checksum) 四元组，按 schema 升序再按 version 升序排序，逐条以 `\u{1F}` 分隔拼接后取 SHA-256，与编译期常量 `EP_MIGRATION_MANIFEST_SHA256` 比对，不一致即失败。该常量由 build.rs 在构建时对 `db/migrations/` 下全部文件做同样归一化（统一 LF、去行尾空白）后计算。任何进程都不执行迁移。本阶段 `db/migrations/` 下只有 24 个空目录，清单为空集，判定平凡通过，阶段 2 写入迁移文件后该项开始有实质内容。

`rls-enabled-and-forced`，全部带 legal_entity_id 列的表均已 ENABLE 且 FORCE 行级安全。算法：查 information_schema.columns 取出含该列的表集合，与 pg_class 的 relrowsecurity 与 relforcerowsecurity 比对，差集非空即失败；同时查 pg_roles 断言当前角色 rolbypassrls 与 rolsuper 均为假。本阶段生产库上该集合为空，判定平凡通过，被测对象为测试库中的 `ci_probe.probe_records`，业务表集合自阶段 2 起逐步建立。

`runtime-role-privileges-bounded`，运行期账号不具备 DDL、角色管理与策略管理权限。算法：对 24 个 schema 逐一 `has_schema_privilege(current_user, s, 'CREATE')` 必须为假，`rolcreaterole` 与 `rolcreatedb` 必须为假。本阶段 24 个 schema 由阶段 2 建立，代码路径在测试库上以等价授权验证。

`clock-skew-within-limit`，时钟偏差小于 1 秒。算法：调用 `adjtimex` 读取 `/proc` 暴露的时间同步状态，若 STA_UNSYNC 置位或 maxerror 超过配置阈值即失败。容器内需挂载 `/proc`，编排文件已保证。

三个 Pending 项的接管方固定如下。`secrets-resolvable`、`audit-chain-verifiable`、`file-store-writable` 由阶段 3b 实现，其中 `audit-chain-verifiable` 按 Degrading 登记。Pending 是如实上报的一种结论，不是空实现，本阶段不为任何未实现的自检项写返回成功的桩。`offsite-sink-requirements` 本阶段既不登记也不留 TODO 注释：按 A-26 该项未满足时要登记降级窗口，而 `DegradationLedger` 归阶段 2、落点判定归阶段 14，两者都不在本阶段，因此该项整条推迟，由阶段 14 在交付落点判定的同一批里连同 `DegradationLedger::open` 的调用一次登记为 Degrading 项。`license-and-modules-consistent` 与 `current-period-open` 两项整项删除，理由与承接方见第 13 节新增决定十三。

`--check` 模式按顺序执行全部注册项与 Pending 项，输出一份 JSON 报告到 stdout 后退出，不监听端口。报告结构为 `{ process, version, items: [{ name, title, outcome: PASSED|FAILED|DEGRADED|PENDING|NOT_APPLICABLE, detail }], overall }`。`--check` 的判定严于运行期：任一 FAILED 或 DEGRADED 均为非零退出，Pending 不计入成败。降级的闸门就落在这里与升级前置脚本上，不落在进程启动上。

#### 5.6 资源限额取值与一次性部署校验

本阶段不做配额生成器，也不产出 `quotas.generated.toml`。规格第 13.1 章的九行配额表以静态 drop-in 文件承载，放在 `deploy/` 下随八个 slice 一并交付，取值只有三类。第一类，每个 slice 一个 `MemoryMax`，按附录 D.2 的 BC-1 基线组合算定后写死，换机型只改这一列。第二类，backup-writer 一个 `IOMax` 硬上限，一个 MB/s 数字，压住每晚那个窗口。第三类，archive-writer 与 PostgreSQL 两个 slice 的 `IOWeight` 高于 backup-writer，保住 RPO 与第 16 章的时延线。`CPUWeight` 直接取规格第 13.1 章该行的份额百分数乘以 100，是相对份额，与机器规格无关，不参与任何计算。删除的是三样东西：按可分配量折算的生成算法、`min(份额×3, 40%)` 的突发上限算法、以及每个进程每次启动都比对一次的自检项。核对改由 `scripts/verify-resource-limits.sh` 在部署与升级时各执行一次，读 cgroup v2 目录下的实际取值与 drop-in 逐行比对，不一致即退出非零。理由是这台机器只服务 20 人，磁盘 IO 是唯一真正稀缺的资源，把 CPU 与内存的份额仲裁做成每进程的启动闸门，解决的是不存在的争用，换来的却是一处配置漂移导致八进程集体拒绝启动。

#### 5.7 可复现构建算法

固定 `SOURCE_DATE_EPOCH` 为该 Git 提交的 committer 时间；`RUSTFLAGS` 固定含 `--remap-path-prefix=$PWD=/build` 与 `--remap-path-prefix=$CARGO_HOME=/cargo`；构建目标固定 `x86_64-unknown-linux-musl` 静态链接；`cargo build --locked --offline --release`；镜像层用固定 mtime 与固定 uid/gid 打包，基础镜像为 scratch。校验方式是在两台不同主机或同一主机的两个不同路径下各构建一次，比对八个二进制的 SHA-256 与九个镜像的 digest，任一不等即失败，并用 diffoscope 输出差异供定位。

#### 5.8 覆盖率分档合并算法

cargo-llvm-cov 只支持全局阈值，因此本阶段自行实现分档。步骤：产出 lcov 后按文件路径匹配 `codecov.toml` 的路径规则，把每个文件归入 A 档（强制不变量相关与平台内核，门槛 85%）、B 档（其余，门槛 70%）与整体档（门槛 80%）；再对 `git diff --unified=0 <base>...HEAD` 的新增与修改行取交集算增量覆盖率（门槛 80%）。任一档不达标即失败，输出未达标文件清单与缺失行号。阶段 1 的 A 档路径规则只包含 `crates/foundation/`，其余 crate 随所属阶段追加。

### 6. API 契约

本阶段全部端点均为系统端点，不承载业务数据，全部只监听回环地址。反向代理站点配置必须显式拒绝 `/api/v1/system/` 与 `/portal/v1/system/` 两个前缀对外暴露，该拒绝规则随部署骨架交付并由 e2e 用例验证。

#### 6.1 core-server

| 方法与路径 | 请求 | 响应 data | 错误码 | 幂等语义 | 权限 |
|---|---|---|---|---|---|
| GET /api/v1/system/health | 无 | `{ status: "UP", process, version, started_at }` | 无 | 天然幂等 | 无，仅回环 |
| GET /api/v1/system/ready | 无 | `{ state: "READY"\|"DEGRADED", pending_items: n }` | PLATFORM.SYSTEM.NOT_READY，503 | 天然幂等 | 无，仅回环 |
| GET /api/v1/system/version | 无 | `{ version, git_commit, source_date_epoch, migration_manifest_sha256, api_major: 1 }` | 无 | 天然幂等 | 无，仅回环 |
| GET /api/v1/system/self-check | 无 | 第 5.5 节的报告结构 | PLATFORM.SYSTEM.NOT_READY，503 | 天然幂等 | 无，仅回环 |
| GET /api/v1/system/metrics | 无 | Prometheus 文本，非 JSON 封套 | 无 | 天然幂等 | 无，仅回环，供 ops-agent 抓取 |
| POST /api/v1/system/echo | `{ text: string, delay_ms?: int }` | `{ text, received_at }` | 见下 | 需 Idempotency-Key，本阶段只校验存在与格式，不做重放存储 | 仅在 feature `ci-probe` 下编译，发布构建不包含 |

echo 端点存在的唯一理由是让封套、错误映射、并发闸门、同步等待上限、请求头校验、追踪与日志七条横切链路在阶段 1 就有端到端用例。按 B-01，它由 `#[cfg(feature = "ci-probe")]` 保护，feature 名固定为 `ci-probe`，在 `apps/core-server/Cargo.toml` 与 `testkit/Cargo.toml` 中声明且默认关闭；`xtask codecheck` 断言发布 profile 不启用该 feature，e2e 用例断言发布镜像上该路径返回 404；发布制品层面的判定由阶段 14 的发布门禁项 `RG-CI-PROBE-ABSENT` 承担，判据为 `cargo tree -e features` 输出中不含 `ci-probe` 且镜像内不含符号 `api_v1_system_echo`。

本阶段登记的错误码全集如下，同步写入 `docs/error-codes.md` 与 `ep-foundation` 的 `error::codes`，两处由 CI 比对。

| 错误码 | category | HTTP | retryable | 触发条件 |
|---|---|---|---|---|
| PLATFORM.SYSTEM.NOT_READY | INFRASTRUCTURE | 503 | true | 进程未就绪或自检未通过 |
| PLATFORM.SYSTEM.SYNC_TIMEOUT | INFRASTRUCTURE | 503 | true | 同步等待超过 8 秒且尚无后台任务承接 |
| PLATFORM.SYSTEM.INTERNAL_ERROR | INFRASTRUCTURE | 503 | true | 未预期错误与 panic 捕获，消息为固定占位文案 |
| PLATFORM.REQUEST.INVALID_PAYLOAD | VALIDATION | 400 | false | JSON 解析失败或字段校验失败，details 定位到字段 |
| PLATFORM.REQUEST.HEADER_MISSING | VALIDATION | 400 | false | 固定请求头缺失或格式非法 |
| PLATFORM.ROUTE.NOT_FOUND | PERMISSION_DENIED | 404 | false | 路由不存在，与无权访问已存在记录同码同形态，避免存在性泄漏 |
| PLATFORM.IDEMPOTENCY.KEY_REQUIRED | VALIDATION | 400 | false | 写请求缺 Idempotency-Key |
| PLATFORM.CAPACITY.CONCURRENCY_LIMIT | INFRASTRUCTURE | 503 | true | 并发闸门等待超过 10 秒 |
| PLATFORM.IDEMPOTENCY.PAYLOAD_MISMATCH | BUSINESS_CONFLICT | 409 | false | 同一幂等键上的请求体哈希与首次调用不一致，本阶段只登记，返回方为阶段 3a |
| PLATFORM.CONCURRENCY.STALE_VERSION | BUSINESS_CONFLICT | 409 | false | 乐观锁版本过期，更新影响行数为零，本阶段只登记 |
| PLATFORM.AUTHZ.NOT_FOUND_OR_DENIED | PERMISSION_DENIED | 404 | false | 记录不存在与无权访问已存在记录同码同形态，避免存在性泄漏，本阶段只登记，返回方为阶段 4 |
| PLATFORM.AUTHZ.OBJECT_FORBIDDEN | PERMISSION_DENIED | 403 | false | 对象已对当前主体可见但该动作被拒，本阶段只登记，返回方为阶段 4 |
| PLATFORM.DB.MIGRATION_WINDOW_CLOSED | BUSINESS_CONFLICT | 409 | false | 未持有迁移窗口即执行在线变更，本阶段只登记，返回方为阶段 13b |

`message` 与 `advice` 在本阶段全部为占位简体中文文案，文案定稿依赖 U-A-06 决策，占位文案已满足规格第 15.1 章的四要素要求，不阻塞本阶段。CI 断言这十三条文案中不出现堆栈、SQL、主机名、进程名、表名与密钥字样。按 C-24，`PLATFORM.IDEMPOTENCY.KEY_REQUIRED`、`PLATFORM.CAPACITY.CONCURRENCY_LIMIT`、`PLATFORM.IDEMPOTENCY.PAYLOAD_MISMATCH`、`PLATFORM.CONCURRENCY.STALE_VERSION`、`PLATFORM.AUTHZ.NOT_FOUND_OR_DENIED`、`PLATFORM.AUTHZ.OBJECT_FORBIDDEN`、`PLATFORM.DB.MIGRATION_WINDOW_CLOSED` 七条一律由本阶段登记在 `crates/foundation/src/error/codes.rs` 与 `docs/error-codes.md` 的 PLATFORM 段，阶段 3a 与阶段 4 不得重复登记；其中后五条在本阶段只登记不返回。`PLATFORM.ROUTE.NOT_FOUND` 只用于路由不存在，记录级的存在性不泄漏一律用 `PLATFORM.AUTHZ.NOT_FOUND_OR_DENIED`，两者不得互换。

请求头校验在本阶段的口径：`X-Legal-Entity-Id`、`X-Device-Id`、`X-Client`、`Authorization` 四个头只校验存在性与格式（UUID 格式、枚举取值、Bearer 前缀与 43 位 base64url），不做任何真实校验，本阶段不定义 `AuthnPort` 与 `LegalEntityScopePort` 两个 trait，也不注入任何空实现：本阶段没有一条业务路由，头校验是一段无端口的纯格式校验，真实校验与其端口由阶段 4 在交付第一条判定时同批引入。这一点在 `docs/config-reference.md` 中明写为阶段 1 临时状态，防止误认为已具备鉴权。系统端点豁免这四个头，豁免清单在代码中是一张固定表，新增豁免路径需改这张表并触发 CODEOWNERS 中的安全审查。

#### 6.2 其余七个进程

| 进程 | 端点 | 说明 |
|---|---|---|
| job-worker | GET /healthz、/readyz、/metrics（8081） | 非封套，纯文本或最小 JSON |
| portal-gateway | GET /portal/v1/system/health、/portal/v1/system/metrics、/portal/v1/system/upstream（8090） | upstream 经回环调用 core 的 health，证明取数一律经 core；在 portal 侧新建 trace，不接受外部 traceparent，回带 X-Correlation-Id |
| integration-gateway | GET /healthz、/readyz、/metrics（8082） | 出网客户端只做一次对配置白名单的自检式解析，不发起真实请求 |
| plugin-host | 无 HTTP，仅 IPC | 见下 |
| ops-agent | GET /metrics（9101）、GET /healthz、/readyz（9102） | /metrics 聚合本机七个进程的指标端点，抓取失败按目标标记 up=0 |
| archive-writer、backup-writer | 无监听 | 仅 IPC 客户端与 spool |

#### 6.3 IPC 契约

承载为 Unix domain socket，路径 `/run/ep/ipc/core.sock` 与 `/run/ep/ipc/plugin.sock`，权限 0660，属主为对应系统账户，组为 ep。帧格式为 4 字节大端长度前缀加 JSON 体，单帧上限 1 MiB。

```json
{ "v": 1, "kind": "request", "id": "<uuidv7>", "method": "system.ping", "payload": {} }
{ "v": 1, "kind": "response", "id": "<同上>", "ok": true, "payload": { "process": "core-server", "version": "..." } }
{ "v": 1, "kind": "response", "id": "<同上>", "ok": false, "error": { "code": "...", "category": "...", "message": "..." } }
```

本阶段只实现 `system.ping` 与 `system.version` 两个方法。基线第 2 节规定的四类上报由阶段 14 在交付写出本体时连同其方法名一次定义，本阶段不预留方法名，也不在协议文档中占位。CI 只断言任何未实现的方法一律返回统一的未知方法错误而不是 panic，这条断言与方法名无关，不因后续新增方法而改动。

spool 行为：写出进程在 core-server 不可用时把待上报帧按一帧一行追加到 `/var/lib/ep/<proc>/spool/pending.jsonl`，恢复连接后按顺序补写并在成功后截断；spool 目录容量超过配置上限时丢弃最旧记录并记 ERROR，绝不阻塞写出。本阶段以心跳帧验证该路径。

### 7. 并发与事务边界

本阶段没有业务事务，但把事务与并发的全部约束以可执行的形式固定下来。

#### 7.1 工作单元

`ep-foundation` 定义 `UnitOfWork`，两个方法为 `transact` 与 `snapshot_transact`，`ep-adapter-db` 提供实现骨架，`ep-adapter-db-pg` 提供实现。按 A-01，事务句柄为 `ep_foundation::port::Tx`，快照上下文为 `ep_foundation::port::SnapshotCtx`，契约层的跨模块方法签名一律写 `&mut dyn Tx`，原先的 `TxHandle` 与 `transact_repeatable_read` 两个名字作废。本阶段的实现要点：`transact` 的隔离级别固定 READ COMMITTED；`snapshot_transact` 是只读快照事务的唯一入口，配合 `SET TRANSACTION SNAPSHOT` 使用，供后续的对账与关账前校验取用，两者是仅有的两个入口；`UnitOfWork` 不带池参数，一个实例在装配时绑定一个池；闭包返回后统一提交，返回 Err 统一回滚；闭包内不允许发起外部调用，由 `xtask archcheck` 对 `ep-app-*` 的符号禁令强制；跨 crate 取具体句柄只允许 `tx.as_any_mut().downcast_mut::<PgTx>()` 一种写法，且只允许出现在 `crates/adapter/db-pg/` 内。本阶段不定义 `AuditSink` 与 `OutboxSink` 两个 trait，也不在事务闭包内留空实现写入位。`UnitOfWork::transact` 的闭包签名已按 A-01 冻结，阶段 3a 交付审计与 Outbox 本体时在闭包内直接调用即可，事务边界本就不需要改动，不必先摆一个返回成功的桩。

#### 7.2 连接池

五个具名池，池参数与超时逐池固定。

| 池 | 归属进程 | 上限 | statement_timeout | lock_timeout | idle_in_transaction | 其他 |
|---|---|---|---|---|---|---|
| rw | core-server | 20 | 10s | 3s | 15s | 事务预算 5 秒由应用侧计时并告警 |
| ro | core-server | 10 | 60s | 3s | 15s | work_mem 64MB，temp_file_limit 2GB |
| worker | job-worker | 5 | 300s | 3s | 15s | 同一运行期读写账号 |
| integ | integration-gateway | 5 | 10s | 3s | 15s | 同一运行期读写账号 |
| ops | ops-agent | 2 | 5s | 3s | 15s | ep_ops_ro |

取用连接时执行四条 `select set_config('app.legal_entity_id'|'app.user_id'|'app.request_id'|'app.trace_id', $1, false)`，归还前逐项设回空串，不使用 DISCARD ALL。两处钩子实现在 `ep-adapter-db-pg` 的 `PgPoolFactory`，业务代码不得直接调用 set_config，由符号禁令强制。集成测试断言归还后的连接上四个变量均为空串，并断言在未设置法人变量时对 `ci_probe.probe_records` 的读、写、更新均返回零行或权限错误，即默认拒绝。
按 C-04，下列四个类型由本阶段在 `ep-adapter-db` 中定义，四者都不进 `ep-foundation`：`PoolKind { Rw, Ro, Worker, Integ, Ops }`；`SessionContext { legal_entity_id, user_id, request_id, trace_id }`；`RetryPolicy { max_attempts: u8, backoff_ms: [u16; 3], retryable_sqlstates: &'static [&'static str] }`；`ConnectionBudget { resident_max: u16, burst_max: u16, per_pool: [(PoolKind, u16); 5] }`。本阶段只交付类型定义与本节表中的逐池上限，`RetryPolicy` 与 `ConnectionBudget` 的取值以及校验脚本 `scripts/verify-connection-budget.sh` 归阶段 2，本阶段不在计划中声称提供该脚本。

#### 7.3 重试

只对尚未产生任何外部可见副作用的事务重试，触发条件为 SQLSTATE 40001 与 40P01，重试 3 次，退避 50、150、450 毫秒。按 C-21，事务重试的唯一指标名是 `ep_db_tx_retries_total`，类型为 counter，标签为 pool 与 sqlstate，由阶段 2 注册与填充；本阶段撤销原拟的 `ep_db_retries_total` 登记，也不登记任何同义指标，`docs/metrics-catalog.md` 的唯一性校验由本阶段的 `xtask` 实现。重试判定由 `RetryPolicy` 集中实现，业务代码不自行捕获这两个错误码，由 `xtask codecheck` 断言 `40001` 与 `40P01` 字面量只出现在该文件中。

#### 7.4 并发闸门与同步等待

并发闸门放在 core-server，理由是 portal-gateway 不建数据库连接且其取数一律经 core-server 的受控能力 API，core-server 是唯一的合流点，因此在 core 上限 20 即等于内部与门户合计上限 20。实现为 tower 的信号量层，许可数取配置值，等待超过 10 秒返回 503 与 PLATFORM.CAPACITY.CONCURRENCY_LIMIT，被拒事件计入 `ep_quota_throttled_total`。已获得许可的在途请求不受影响，不做静默降级。portal-gateway 侧另有一层按来源 IP 的限流，本阶段只交付参数与骨架。

同步等待上限 8 秒实现为 tower 的超时层，超时返回 PLATFORM.SYSTEM.SYNC_TIMEOUT。后台任务承接路径由任务阶段实现，本阶段在错误 advice 中写明该请求应改由后台任务表达。

#### 7.5 幂等

按 C-07，幂等分三段，本阶段只做第一段。本阶段的中间件名固定为 `IdempotencyKeyHeaderGuard`，只校验 `Idempotency-Key` 头存在且为合法 UUIDv7，不合法返回 `PLATFORM.IDEMPOTENCY.KEY_REQUIRED`，本阶段不做任何判等与重放存储。第二段是端口定义，`ep_adapter_db::port::IdempotencyStore` 及其 `try_begin` 与 `finish` 两个方法、`IdempotencyScope { legal_entity_id, user_id, endpoint, key }`，归阶段 2。第三段是 `platform_msg.idempotency_keys` 建表与重放实现，返回 `IdempotencyOutcome::FirstCall`、`Replay { status, body }` 或 `PayloadMismatch`，归阶段 3a。本阶段不建该表，三处不得各自判等。按 C-24，`PLATFORM.IDEMPOTENCY.PAYLOAD_MISMATCH` 由本阶段一并登记在错误码表中，但本阶段不可能返回它，返回方是阶段 3a。中间件已按最终形态分层，接入存储时只需注入一个 `IdempotencyStore` 实现。

#### 7.6 优雅停机与崩溃

收到 SIGTERM 后进入 Draining，停止接受新连接，等待在途请求完成，上限由配置控制默认 30 秒，超时后强制关闭并记 WARN，退出码仍为 0。systemd 的 TimeoutStopSec 取 45 秒。panic 由 catch_unwind 层捕获，先写一条含 trace_id 的 ERROR 日志，再返回 PLATFORM.SYSTEM.INTERNAL_ERROR，不中止进程；只有自检失败与配置错误才中止进程。

#### 7.7 与 Outbox 的关系

本阶段不写任何 Outbox 条目，也不消费，也不为消费预留任何钩子。`JobRegistry` 在本阶段只有注册与调度两件事，已注册任务为零个；至少一次投递与幂等消费的形态由阶段 3a 在交付 Outbox 消费时连同 `consumer_name` 与去重一次给出。理由是本阶段没有任何消费者，未被使用的钩子无从验证，只有维护成本没有判据。

### 8. 配置项

配置结构体开启 `deny_unknown_fields`，加载顺序按基线第 7.1 节五层。下表是本阶段新增的全部配置键，同步写入 `docs/config-reference.md`，由 `xtask configdoc` 断言代码与文档逐键一致，缺一即失败。生效方式一列中，启动表示改动后需重启，SIGHUP 表示可热加载，取用表示在下次取用时生效。

| 键 | 类型 | 默认值 | 生效方式 |
|---|---|---|---|
| http.bind_addr | string | 按进程固定，core 为 127.0.0.1:8080 | 启动 |
| http.max_body_bytes | u64 | 1048576 | 启动 |
| http.request_timeout_ms | u32 | 8000 | 启动 |
| http.shutdown_drain_ms | u32 | 30000 | 启动 |
| http.concurrency_limit | u16 | 20 | 启动 |
| http.concurrency_wait_ms | u32 | 10000 | 启动 |
| ipc.socket_path | path | /run/ep/ipc/<proc>.sock | 启动 |
| ipc.max_frame_bytes | u32 | 1048576 | 启动 |
| db.host、db.port、db.database | string、u16、string | 127.0.0.1、5432、ep | 启动 |
| db.user | string | ep_app_rw | 启动 |
| db.password_ref | string | secret://db/app_rw#1 | 取用 |
| db.pool.rw_max、ro_max、worker_max、integ_max、ops_max | u16 | 20、10、5、5、2 | 启动 |
| db.pool.acquire_timeout_ms | u32 | 3000 | 启动 |
| db.pool.max_lifetime_s、idle_timeout_s | u32 | 1800、300 | 启动 |
| db.timeout.<池>.statement_ms | u32 | 见第 7.2 节 | 启动 |
| db.timeout.<池>.lock_ms | u32 | 3000 | 启动 |
| db.timeout.<池>.idle_in_tx_ms | u32 | 15000 | 启动 |
| db.ro.work_mem_kb、temp_file_limit_kb | u32 | 65536、2097152 | 启动 |
| db.retry.max_attempts | u8 | 3 | 启动 |
| db.retry.backoff_ms | u32 数组 | [50,150,450] | 启动 |
| log.level | string | info | SIGHUP |
| log.debug_auto_off_minutes | u16 | 30 | SIGHUP |
| metrics.enabled | bool | true | 启动 |
| metrics.bind_addr | string | 按进程固定 | 启动 |
| trace.sample_ratio | f32 | 0.1 | SIGHUP |
| trace.otlp_enabled | bool | false | 启动 |
| trace.otlp_endpoint | string 可空 | null | 启动 |
| secrets.dir | path | /var/lib/ep/secrets | 取用 |
| secrets.provider | enum file、kms | file | 启动 |
| selfcheck.clock_skew_max_ms | u32 | 1000 | 启动 |
| runtime.worker_threads | u16 | 0，表示按 cgroup CPU 配额推导 | 启动 |
| runtime.blocking_threads | u16 | 32 | 启动 |
| egress.allowlist | string 数组 | 空 | SIGHUP |
| egress.connect_timeout_ms、request_timeout_ms | u32 | 3000、15000 | 启动 |
| egress.ca_bundle_path | path | /etc/ep/ca/esign-ca.pem | 取用 |
| egress.breaker.failure_threshold、open_ms、half_open_probes | u16、u32、u8 | 5、30000、1 | 启动 |
| spool.dir | path | /var/lib/ep/<proc>/spool | 启动 |
| spool.max_bytes | u64 | 268435456 | 启动 |
| portal.upstream_base_url | string | http://127.0.0.1:8080 | 启动 |
| portal.rate_limit_rps | u16 | 20 | SIGHUP |

不进配置文件的两类：一是运行期可变的业务参数，本阶段一条都不引入；二是机密，配置里只写 `secret://` 引用。本阶段的 `FileSecretProvider` 从 `secrets.dir` 读取权限 0600 的文件，不做信封加密，属临时实现，在 `docs/config-reference.md` 与 ADR 中显式标注，由密钥阶段替换为内置 KMS 或 HSM 解封。CI 断言 `SecretString` 未实现 Debug 与 Display，并断言配置结构体中任何名字含 password、secret、key、token 的字段类型必须是 `SecretString` 或 `SecretRef`。

### 9. 测试计划

#### 9.1 单元测试覆盖的分支

foundation：Money、UnitPrice、Quantity、Rate 四类的构造成功与失败分支（位数正确、位数过多、位数不足、非数字、超上限、负号、前导零）；Amount 到 Money 的舍入表，逐行写死期望值，至少覆盖 0.005、0.015、-0.005、-0.015、0.004999、2.675 六个中值与近中值样本，正负两侧均验证远离零；Decimal 加减在全精度下不提前舍入的断言；Id 的解析、显示、跨类型不可互转（编译期用 trybuild 断言不通过）；SecurityLevel 与 EventType 的反序列化拒绝未知取值；AppError 的 Display 不含 source 链；Redacted 与 SecretString 的三种输出路径均为掩码。

UUIDv7：同毫秒内序列递增、序列溢出自旋、时钟回拨不倒退、生成结果按字典序与时间序一致（1000 个样本排序后与生成顺序一致）。

关联编号：跨进程不撞号、跨日归零、进程内序号回绕。

状态机：全部 12 条合法迁移逐条通过；非法迁移的笛卡尔积逐条返回 BUSINESS_CONFLICT；不可达状态断言。

配置：分层覆盖顺序（内置默认、主配置、片段目录字典序、环境变量、命令行）逐层验证；未知键拒绝；类型错误的错误消息含键路径；EP__DB__POOL__RW_MAX 形式的双下划线映射。

自检：六项各自的通过与失败分支，Blocking 项失败时退出码 78；Degrading 项未通过时状态机进入 Degraded 而不退出，且 `--check` 退出码非零；Pending 项在报告中如实标注且不计入 overall 的成败。

覆盖率合并算法：三档判定各自的通过与不通过、增量行集合为空时的处理。

领域属性测试：本阶段以 proptest 建立框架并落三条与业务无关的属性，即 `to_money` 幂等（对已是 2 位的值再舍入不变）、Money 加法结合律与交换律、UUIDv7 单调性。规格第 17.2 章要求的借贷平衡、库存守恒、核销守恒、移动加权平均单价重算、价差拆分五组不变量属于财务与库存阶段，本阶段只交付 `proptest` 的策略工具与失败用例最小化配置，并在 `docs/adr/` 中登记这五组的挂载点。

#### 9.2 集成测试场景清单

全部集成测试使用真实 PostgreSQL 16，禁止内存库替代。每个用例独占一个数据库，命名 `ep_test_<nanoid>`，用例结束即删库；容器由 testcontainers 启动，若 CI 主机已有实例则复用并只建库。

| 编号 | 场景 | 判定 |
|---|---|---|
| IT-01 | ep-migrate CLI 骨架 | apply、status、check、gen-rls、open-window 五个子命令的参数解析通过，`status --format=manifest` 输出制品清单；空库上 24 个 schema 与 24 张历史表的存在性判定归阶段 2 的同名用例 |
| IT-02 | ep-migrate 退出码约定 | 0 成功、2 参数错误、3 迁移窗口未打开、4 校验和不符、5 版本不一致、78 环境自检失败六个分支各有一个用例 |
| IT-03 | 迁移清单哈希算法 | 归一化后的 SHA-256 与 build.rs 常量一致；篡改探针目录中的一个文件后比对失败 |
| IT-04 | 授权矩阵的判定路径 | 在测试库上以等价授权脚本验证 select/insert/update 允许而 DELETE 被拒，生产库 24 个 schema 的默认权限矩阵归阶段 2 的同名用例 |
| IT-05 | 运行期账号无 DDL | 在测试库的探针 schema 上 create table 被拒 |
| IT-06 | 会话变量注入与清除 | 取用后四个变量为期望值，归还后四个均为空串 |
| IT-07 | RLS 默认拒绝 | 未设法人变量时对探针表的读、写、更新、删除、聚合、排序均不可见或被拒 |
| IT-08 | RLS 跨法人矩阵 | 设法人 A 后读不到法人 B 的行；以法人 A 写入带法人 B 的行被 with check 拒绝；聚合与排序不泄漏行数与位次；错误消息不含被拒行内容 |
| IT-09 | FORCE RLS 对属主生效 | 以 ep_mod_ 属主身份同样受策略约束 |
| IT-10 | 乐观锁 | 版本正确时更新成功且 row_version 加一；版本过期时影响行数为 0 并映射为 PLATFORM 前缀的冲突错误形态（本阶段以探针表的通用实现验证） |
| IT-11 | 语句超时 | rw 池上超过 10 秒的语句被中断并映射为 INFRASTRUCTURE |
| IT-12 | 锁超时 | 3 秒未取得锁即失败 |
| IT-13 | 空闲事务超时 | 15 秒后会话被终止且连接池能自愈 |
| IT-14 | 序列化失败重试 | 人为构造 40001，断言重试 3 次、退避时长、指标计数与最终结果 |
| IT-15 | 死锁重试 | 人为构造 40P01，同上 |
| IT-16 | 事务闭包回滚 | 闭包返回 Err 时全部写入回滚 |
| IT-17 | 池上限 | rw 池并发 21 个请求时第 21 个走并发闸门而非耗尽连接 |
| IT-18 | 自检项 database-reachable、migration-version-matched、rls-enabled-and-forced、runtime-role-privileges-bounded | 逐项构造失败条件（版本不符、timezone 非 UTC、max_connections 不足、迁移清单不符、探针表未 FORCE RLS、账号具备 DDL）并断言退出码 78 与报告内容 |
| IT-19 | 封套一致性 | 成功与失败两种响应用 insta 快照固定字段集合与顺序 |
| IT-20 | 请求头校验 | 四个固定头逐个缺失与逐个格式错误共 8 个用例 |
| IT-21 | 幂等头 | 写请求缺 Idempotency-Key 返回 KEY_REQUIRED；带非 UUIDv7 时返回 VALIDATION |
| IT-22 | 并发闸门 | 21 并发下第 21 个等待并在 10 秒后返回 CONCURRENCY_LIMIT，`ep_quota_throttled_total` 加一 |
| IT-23 | 同步等待上限 | delay_ms 超过 8000 时返回 SYNC_TIMEOUT |
| IT-24 | panic 捕获 | 触发 panic 的探针路径返回 INTERNAL_ERROR 且进程仍存活 |
| IT-25 | 日志字段 | 每请求一条访问日志，17 个固定字段齐全，敏感值为掩码 |
| IT-26 | 指标端点 | 第 13 节新增决定五登记的六个指标名 `ep_build_info`、`ep_selfcheck_pending_items`、`ep_db_pool_connections`、`ep_db_statement_duration_seconds`、`ep_http_request_duration_seconds`、`ep_quota_throttled_total` 均存在，标签基数纪律断言（无 user_id、doc_no、trace_id 标签，route 为模板路径） |
| IT-27 | IPC | 帧编解码往返、超长帧拒绝、未知方法返回错误、socket 权限为 0660 与属主正确 |
| IT-28 | spool | core 不可用时落盘、恢复后补写、超上限丢最旧并记 ERROR |
| IT-29 | 优雅停机 | SIGTERM 后在途请求完成、新请求被拒、退出码 0、drain 超时路径 |
| IT-30 | 探针表模板一致性 | RLS 模板生成器输出与黄金文件逐字节一致 |
| IT-31 | collation 一致性 | 判定位在 `check` 子命令中就位并有一个负样例夹具，判据为 `pg_database` 的 `datcollate` 与 `datctype` 均为 `C` 且 `datlocprovider` 为 `c`；对生产库的实际比对归阶段 2，因引导脚本由阶段 2 交付 |

#### 9.3 端到端用例

E2E 在单机编排上跑，覆盖规格第 17.2 章中本阶段可达的部分，不涉及业务闭环 14 步。

| 编号 | 场景 | 判定 |
|---|---|---|
| E2E-01 | 一条命令起全栈 | PostgreSQL 加八个进程加一次性迁移容器全部达到 active，九个健康端点全绿 |
| E2E-02 | 全部进程 `--check` | 九份报告 overall 均为 PASSED 且退出码 0；构造任一 Degrading 项未通过时 overall 为 DEGRADED 且退出码非零 |
| E2E-03 | 迁移清单不一致时启动 | 自检项 migration-version-matched 失败，进程以 78 退出，systemd 不重启 |
| E2E-04 | 配置未知键 | 以 78 退出，stderr 含键路径 |
| E2E-05 | 资源限额取值 | 八个 slice 的实际 cgroup 取值与 `deploy/` 下 drop-in 文件逐行一致，`scripts/verify-resource-limits.sh` 返回 0，篡改一行后返回非零 |
| E2E-06 | 进程崩溃重启 | kill -9 core-server 后 systemd 重启并在 30 秒内重新就绪，其余进程不受影响 |
| E2E-07 | 优雅停机与整栈停止 | 全部退出码 0，无残留 socket 与 pid |
| E2E-08 | 系统端点不外泄 | 经反向代理访问 `/api/v1/system/` 与 `/portal/v1/system/` 返回 404 或 403 |
| E2E-09 | 发布构建无探针 | 发布镜像上 POST /api/v1/system/echo 返回 404 |
| E2E-10 | 制品验签 | 在断网机器上执行 verify-release.sh，签名与校验清单全部通过；篡改一个字节后失败 |
| E2E-11 | 可复现构建 | 两次构建的八个二进制 SHA-256 与九个镜像 digest 全等 |
| E2E-12 | 离线构建 | 断网环境下 `cargo build --locked --offline` 成功 |

#### 9.4 性能相关项

本阶段不参与规格第 16 章的通过线判定，只建立回归基线，写入 `docs/perf-baseline-stage1.md`，后续阶段以此比对是否劣化。

| 项 | 阶段 1 门槛 |
|---|---|
| CI 全量运行时长 | 不超过 60 分钟；增量不超过 25 分钟 |
| core-server 从进程启动到 ready | 不超过 3 秒，不含数据库启动 |
| 单个空壳进程常驻 RSS | 不超过 128 MiB |
| 单个进程镜像大小 | 不超过 40 MiB |
| GET /api/v1/system/health 本机 P95 | 不超过 20 毫秒，200 次采样 |

#### 9.5 覆盖率门槛

本阶段即启用最终门槛，不设过渡期。A 档路径 `crates/foundation/` 行覆盖率不低于 85%；其余代码不低于 70%；新增与修改代码不低于 80%；工作区整体不低于 80%。骨架 crate 因无代码不计入分母。`#[ignore]` 必须带 issue 编号注释，`xtask codecheck` 断言其存活不超过一个阶段（以注释中的阶段编号判定）。

#### 9.6 与规格第 17.2 与 17.3 章判据的对应

本阶段能对应到的判据有四类，其余判据在本阶段只交付判定框架不交付判定内容，此点在退出条件中显式承认。

一是单元测试与领域属性测试的覆盖率通过标准，本阶段以分档门禁完整实现并生效。二是法人行级隔离与越权测试集。按 C-05，`tests/rls_matrix` 分三段，三个阶段不得重复实现同名函数。本阶段交付该 CI 目标与八类断言的骨架并在探针表上实测，函数名固定为 `assert_read`、`assert_write`、`assert_update`、`assert_delete`、`assert_aggregate`、`assert_sort`、`assert_report_projection`、`assert_error_leak`，位于 `testkit/src/rls_matrix.rs`，业务表进入后按同一套断言扩展。阶段 2 只追加 `assert_recon_context_borrow` 一个函数，即内部对账系统安全上下文的入口借用，本阶段不为它建目标文件与失败占位。两个复制角色的入口借用断言不再设立：规格第 7.7 章自认三项遏制手段都不阻止持有本机操作系统权限者切换到写出进程账户并从本机建立流复制连接这条路径，为一条自认挡不住的通道再建一组测试与核对机制只扩大维护面，该边界的承载改为规格第 21.21 章的披露。阶段 4 追加 `matrix_32.rs` 的 32 组完整矩阵与发布门禁项 `RG-RLS-MATRIX-GREEN`。三是数据库适配认证套件中的迁移与锁两项的执行框架，本阶段固定迁移会话的 `lock_timeout = '5s'` 与 `statement_timeout = '30min'`，并提供在线变更耗时的实测夹具。四是混沌与故障注入六类中的进程崩溃后重启恢复一类，本阶段以 E2E-06 覆盖其可达部分，即重启后进程恢复与请求按第 15.1 章明确失败，未完成任务恢复与已确认事务零丢失属后续阶段。第 17.3 章的九项强制不变量在本阶段没有被测对象，一项都不声称通过。

### 10. 退出条件

下列每条都能由一条命令或一份自动产出的报告客观判定，全部达成才算本阶段完成。

1. `cargo build --workspace --locked --offline --release` 成功，零 warning，`-D warnings` 生效。
2. 每个 crate 的命名前缀、目录名与 `Cargo.toml` 中的 name 三处一致，由 archcheck 断言。不再断言 crate 清单与基线第 1.2 节逐项一致：逐项一致这条判据把 crate 边界变成必须走基线修订才能移动的冻结物，而真正要守的依赖方向由退出条件 3 的七条禁止项守住；基线第 1.2 与 1.3 节的两张表相应降为现状记录，增删 crate 走普通评审。`codecov.toml` 的分档路径规则按目录前缀表达，不与 crate 清单逐项对应，新增 crate 不会静默逃出覆盖率分档。
3. 依赖方向的七条禁止项各有一个负样例，负样例构建必须失败；正样例全部通过。
4. 八个二进制启动、就绪、优雅停机、崩溃重启四条路径在 E2E 中全绿。
5. `ep-migrate` 的五个子命令 apply、status、check、gen-rls、open-window 参数解析齐备，六个退出码各有一个用例；迁移清单哈希在探针目录上比对通过且篡改后失败；`db/migrations/` 下 24 个空目录存在，且目录中不含任何顺序声明文件。空库上 24 个 schema 与 `platform_core.schema_history` 一张历史表的存在性判定归阶段 2。
6. 六项已实现自检各自的通过与失败分支均有集成测试，自检项一律以注册名标识且各自带 Blocking 或 Degrading 一个档位；三项 Pending 项 `secrets-resolvable`、`audit-chain-verifiable`、`file-store-writable` 在报告中如实标注，且有一条 CI 断言保证未注册项数量只减不增。
7. 十三条错误码在 `docs/error-codes.md` 与代码常量表中一致，重复码或缺失码即构建失败，其中 C-24 列明的七条由本阶段独家登记。
8. 第 13 节新增决定五登记的六个指标名在指标端点上可见，其中 `ep_db_pool_connections` 与 `ep_db_statement_duration_seconds` 按 C-23 本阶段只注册不填充，判据为指标名存在而非有非零样本，标签基数纪律断言通过。
9. 全部配置键在 `docs/config-reference.md` 中有条目，代码与文档逐键一致。
10. 结构门禁十一个子命令各自有负样例，负样例必须失败。
11. SQL 静态检查的全部规则各有负样例，至少覆盖 DELETE 禁令、varchar 禁令、enum 禁令、current_date 禁令、跨 schema 外键禁令、ON DELETE CASCADE 禁令、rollback 注释缺失、公共列缺失与顺序错误、命名规范违反、迁移单一职责违反十项。
12. 覆盖率四档全部达标，且有一个人为降低覆盖率的负样例使流水线变红。
13. 两次独立构建产出完全相同的二进制哈希与镜像 digest。
14. SBOM 生成成功，`cargo deny` 与依赖漏洞扫描零严重与高危，许可证清单通过；`xtask sbom` 另含一个断言 SBOM 中不出现 `ep-bench` 与 `ep-release-gate` 两个包名的负样例，与阶段 14 的发布门禁项 `RG-TOOLS-EXCLUDED` 同名同判据。
15. 升级包结构完整，客户侧验签脚本在断网机器上通过，篡改后失败。
16. `deploy/` 下八个 slice 的静态资源限额 drop-in 取值与规格第 13.1 章配额表逐行一致，`scripts/verify-resource-limits.sh` 在部署后执行一次返回 0，篡改一行后返回非零，且任何进程的启动自检中不出现资源限额相关项。
17. 阶段 1 性能回归基线五项全部有实测记录并达标。
18. 六份文档骨架存在，ADR 至少含工具链冻结、collation 选型、musl 静态链接、CI 平台选型、新增 crate 五篇。
19. 源码仓库、制品与离线依赖仓库的加密备份脚本可执行，且完成一次恢复验证并留下记录。
20. 本计划第 13 节列出的偏离与新增决定全部回写共享技术基线，回写内容经评审通过。
21. ep-foundation 的跨阶段冻结项齐备且逐项与裁定一致：`port::tx` 的 `Tx`、`SnapshotCtx`、`UnitOfWork` 三者与 `TxId`、`IsolationKind`，`id::marker` 的 22 项标记类型，`principal` 的两个常量，`security::context` 的 19 个字段、三个配套枚举与七个字段类型 `DeviceId`、`RoleCode`、`DutyClass`、`RecordShare`、`DataScopeTag`、`RequestId`、`TraceId` 及其配套枚举 `RecordShareGrant`，`ModuleCode` 15 项，`CapabilityDomain` 18 项与 `ActionClass` 5 项；`crates/foundation/src/port/search.rs` 与 `doc.rs` 两个空模块文件存在；`xtask archcheck` 断言 `downcast_mut::<PgTx>` 只出现在 `crates/adapter/db-pg/`。
22. `testkit/src/rls_matrix.rs` 的八个断言函数存在，函数名与 C-05 逐字一致，且在探针表上全绿；本阶段不实现阶段 2 与阶段 4 的追加函数。
23. `docs/data-dictionary.md` 的单据类型码一节存在，`xtask configdoc --check-doc-type-codes` 通过，判据为该节与 `ep-platform-sequence` 的常量表逐项一致且无重复。
24. `docs/metrics-catalog.md` 的指标名唯一性校验在 `xtask` 中实现并通过，`ep_build_info`、`ep_selfcheck_pending_items`、`ep_db_pool_connections`、`ep_db_statement_duration_seconds`、`ep_http_request_duration_seconds`、`ep_quota_throttled_total` 六个指标已注册，`ep_db_retries_total` 与 `ep_tx_retry_total` 两个名字不出现在任何登记文件与代码中。
25. T0 贯通线的判定手段就位：`xtask e2e --profile=t0` 作为独立目标可执行并返回 0，`ep-datagen` 的 `t0-min` 样本档在同一 seed 下两次生成字节一致，`deploy/` 一条命令起全栈。本阶段不提供 T0 的任何业务切片，也不为 T0 声称任何业务判据。

### 11. 与规格和 PRD 的对应

| 规格章节 | 本阶段实现的条目 | 证据 |
|---|---|---|
| 第 4.1 章 | 模块化单体的代码隔离边界，以依赖方向门禁落实 | archcheck 报告与负样例 |
| 第 4.3 章 | workspace 七层边界与八个 apps 进程一一对应 | crate 清单断言、E2E-01 |
| 第 7.1 章 | 单实例单库的目录约定与迁移目录骨架；24 个 schema 与模块级属主角色的实际创建归阶段 2 | 迁移目录骨架、IT-01 |
| 第 7.3 章 | 认证套件中的迁移与锁两项的执行框架，PostgreSQL 16 版本门禁 | IT-18、迁移会话超时设置 |
| 第 7.4 章 | 在线变更边界的会话参数与耗时实测夹具 | 迁移工具参数与夹具用例 |
| 第 7.7 章 | 会话变量唯一判据、无 BYPASSRLS、连接归还清除；用途分账号与两个复制角色的建立归阶段 2 | IT-04 至 IT-09，角色与权限证据见阶段 2 |
| 第 12.3 章 | 密码算法选型的落地约束，即 TLS 1.3、SHA-256、ECDSA 三项在制品签名与哈希链参数上的取值 | 签名脚本与 ADR |
| 第 13.1 章 | 九行配额表落为八个 cgroup slice 的静态资源限额 drop-in，取值三类，由部署脚本一次核对 | D-06、E2E-05 |
| 第 13.2 章 | 单机容器编排、标准 OCI 容器、只依赖开放接口 | D-05、镜像清单 |
| 第 15.1 章 | 五类错误分类、四要素、存在性不泄漏、面向使用者文案 | 错误码表、IT-19 至 IT-21 |
| 第 16 章 | 20 并发上限的承载方式与连接池分池上限 | 第 7.2、7.4 节，IT-17、IT-22 |
| 第 17.1 章 | 本地 Git Monorepo 的目录结构、需求编号关联、角色分离与非自审的分支保护、仓库与制品加密备份 | 分支保护配置、CODEOWNERS、备份脚本与恢复记录 |
| 第 17.2 章 | 测试三层边界、覆盖率分档门槛、法人越权测试集骨架、进程崩溃重启一类 | 第 9 节全部 |
| 第 17.4 章 | SAST、依赖与密钥扫描、SBOM、制品签名、可复现构建、离线依赖仓库、客户侧验签 | 流水线阶段 7 与 8、E2E-10 至 E2E-12 |
| 第 18 章 | 单一版本线的版本号规则、升级包结构、回退说明、迁移逆向性标注 | D-11、迁移文件头 rollback 段 |
| 附录 A.3 | 基准数据集生成器的确定性与版本化骨架 | ep-datagen 与确定性用例 |
| 附录 D.2 | BC-1 基线组合的构建与运行形态落地 | 镜像与编排文件 |

| PRD 节 | 本阶段实现的条目 |
|---|---|
| 0.6 维护纪律 | 错误码表、事件目录、指标目录、配置参考四份登记文件与 CI 一致性校验 |
| 11.2 并发与规模上限 | 20 并发闸门与超限时的可见后果 |
| 11.3 响应时延与等待反馈 | 8 秒同步等待上限与超时提示路径 |
| 11.9 降级状态的用户可见性 | 自检项 11 的 Degraded 状态表达与就绪端点上的降级标识 |
| 11.10 错误与失败提示 | 四要素齐备的错误封套与不泄漏内部信息的断言 |
| 附录乙 U-A-03 | 文本长度以 text 加 CHECK 表达的生成器与探针表示例 |
| 附录乙 U-A-05 | 分页、排序、导出上限五项常量在 foundation 中集中定义 |
| 附录乙 U-A-06 | 错误码编制规则与占位文案，文案定稿待决策 |

### 12. 风险与预留

#### 12.1 已知技术风险

R-01，musl 静态链接的内存分配性能。musl 的默认分配器在多线程下弱于 glibc，20 并发下可能在性能阶段暴露。缓解是本阶段即引入 mimalloc 作为全局分配器并在性能基线中记录，若性能阶段判定不达标，回退方案是改用 glibc 加固定 digest 的 distroless 基础镜像，代价是可复现构建的验证面扩大到基础镜像，需重跑 E2E-11。

R-02，refinery 与 sqlx 双驱动共存。refinery 走 tokio-postgres，sqlx 自带驱动，两套驱动的行为差异会造成迁移与运行期对同一 DDL 的判断不一致。缓解是把 refinery 只链接进 `tools/ep-migrate`，八个运行期进程一律不链接 refinery，只用 sqlx 读历史表，由 archcheck 断言。

R-03，可复现构建的偶发不一致。来源通常是 build.rs 中引入的时间或路径、并行编译顺序、以及依赖中的过程宏产生非确定输出。缓解是 CI 阶段 8 强制两次构建比对，失败时用 diffoscope 定位并在 ADR 中记录已知不确定来源清单。

R-04，testcontainers 依赖容器运行时。CI 主机需 rootless podman，某些企业 Linux 上 cgroup v2 委派配置繁琐。缓解是测试夹具支持两种模式，容器模式与复用本机实例模式，二者用同一套建库与删库逻辑。

R-05，八个系统账户与 rootless 容器的 UID 映射。用户命名空间下容器内 UID 与宿主 UID 不同，会影响 socket 属主与 spool 目录权限。缓解是编排文件固定 `--userns=keep-id` 类映射并在 E2E 中断言 socket 与目录属主，另交付一份 UID 映射对照表进部署记录。

R-06，签名密钥在本阶段是软件密钥。正式签名要求硬件密码机与双人控制，本阶段用软件 ECDSA 密钥打通流程，存在把临时密钥误带入正式发布的风险。缓解是签名脚本对密钥来源做硬校验，非 HSM 来源的签名在制品元数据中标注 `signing_authority=dev` 并使发布流水线拒绝放行，只允许内部阶段制品使用。

R-07，复制槽的本机事务日志保留上限依赖实测。按 C-01，`max_slot_wal_keep_size` 的取值与其落地脚本 `db/bootstrap/02_cluster_params.sql` 均归阶段 2，取值按规格附录 A.3 的连续归档本机保留子项等量取 350GB，本阶段不自带取值，也不进本阶段的配置参考。缓解是规格第 7.3 章的两个断链用例实测本机 pg_wal 峰值并与该子项对照，实测速率使该取值不足以支撑部署记录约定的落点不可写时长时，按附录 A.3 同一构成上调该子项并重算容量下限，回填由归档阶段执行。

R-08，覆盖率门槛在骨架阶段可能诱发为覆盖而写的空测试。缓解是结构门禁与负样例制度并行，且 A 档只覆盖 foundation，其余按实际代码量分档。

R-09，CI 平台选型属本阶段新增决定，若客户或团队后续改用其他 CI，门禁逻辑不应绑定平台。缓解是全部门禁收敛到 `cargo xtask ci` 一个入口，CI 配置文件只负责调度，迁移平台的成本限于重写调度文件。

#### 12.2 为后续阶段预留的扩展点

自检注册表预留 `secrets-resolvable`、`audit-chain-verifiable`、`file-store-writable` 三项的注册位，注册函数签名与 severity 取值域已冻结，各阶段追加项一律以注册名加一个档位登记，不用序号。本阶段不预留任何返回成功的空实现：跨阶段端口按同批交付、整条推迟、改用降级窗口三者择一处置，本阶段一律取整条推迟，因此 `AuthnPort`、`AuthzPort`、`AuditSink`、`OutboxSink` 与 Outbox 消费的去重钩子在本阶段都不存在，由阶段 4 与阶段 3a 在交付判定与本体的同一批里引入。HTTP 中间件栈只留 `IdempotencyStore` 一个注入点，按 C-07 其端口定义归阶段 2、存储与重放实现归阶段 3a，本阶段的 `IdempotencyKeyHeaderGuard` 只校验请求头，不需要任何桩。`db/migrations/` 下 24 个目录已列齐，迁移执行顺序由阶段 2 交付的单一全局 Runner 按文件版本号全序排定，业务阶段只按版本号加文件，不存在任何顺序声明文件要改。错误码表、事件目录、指标目录、数据字典的单据类型码一节四份登记内容已建立并被 CI 校验，后续阶段先登记后实现的纪律有强制点。`ep-adapter-db` 与 `ep-adapter-db-pg` 的分层已就位，多库延期不影响抽象层的稳定性。feature `ci-probe` 提供一个不进发布的探针通道，后续阶段可复用于横切链路验证。

### 13. 偏离基线与本阶段新增决定

按基线第 0 节与第 12 节的要求，本节单列全部偏离项与新增决定，每项给出理由与影响范围，并同步提出基线修订。本阶段不接受只在实现里偏离。

偏离一，新增 crate `ep-platform-runtime`。基线第 1.2 节的平台底座清单没有承载进程运行时装配的 crate，而基线第 7.3 节已把 `SelfCheckRegistry` 的落点写死在 `crates/platform/runtime/src/selfcheck/registry.rs`，两处自相矛盾。若不新增，配置加载、自检注册表、信号处理、生命周期状态机、健康与就绪端点这一整套代码要在八个二进制里各写一份，与文件规模纪律和单一事实源冲突。该 crate 只承载进程生命周期状态机、分层配置加载、`SelfCheckRegistry`、健康与就绪端点、HTTP 服务器与中间件栈骨架，以及以 trait 表达的 IPC 服务端接口；IPC 的具体传输实现仍留在 `ep-adapter-ipc`，由 apps 在 `wiring.rs` 注入，因此本 crate 只依赖 foundation 与其他 platform，apps 依赖它，不改变任何既有依赖方向。影响范围只有一处：基线第 1.2 节的平台底座表增加 `ep-platform-runtime` 一行，职责列取上句。该表不补冻结措辞，也不再作为 archcheck 的比对面，理由见第 10 节退出条件 2。
偏离二，`ep-foundation` 承载 22 个实体标记类型。按 A-01，`crates/foundation/src/id/marker.rs` 集中声明跨模块被引用实体的零大小标记类型，清单固定 22 项。这是对基线第 1.3 节禁止 foundation 承载业务概念一条的一处受限例外，标记类型无字段、无方法、无 trait 实现，只承载类型身份，供 `Id<T>` 在契约层表达跨模块引用。不采用该例外的代价是每个 ep-contract crate 各自声明一份标记类型，同一实体在不同 crate 中的 `Id<T>` 互不相容，跨模块方法签名无法表达。影响范围是基线第 1.3 节增加一条受限例外，并注明清单为 22 项、任何阶段不得增删。

新增决定一，新增两个非交付或运维用途的 workspace 成员：`xtask` 是纯开发期工具，不进任何制品；`tools/ep-migrate` 是一次性运维工具，随制品交付，以 systemd 的 oneshot 单元在升级窗口内执行。二者都不是常驻进程，不监听端口，不属于八进程清单，不改变基线第 2 节。运行 `ep-migrate` 的操作系统账户为 `ep-migrate`，与八个进程账户互不复用，同属组 ep。影响范围有两处：基线第 1.1 节的目录布局改为两段，第一段是 workspace 成员路径，在既有八条之外增加 `/xtask/` 与 `/tools/<name>/`，第二段是非 workspace 成员的仓库目录，列 `/db/bootstrap/`、`/db/checks/`、`/deploy/`、`/scripts/`、`/clients/desktop/`、`/clients/mobile/` 六条，并在节末写明本两段即全部顶层目录，新增顶层目录必须先改本节；基线第 2 节的账户说明增加一条，`ep-migrate` 账户与八个进程账户互不复用且同属组 ep。

新增决定二，数据库建库参数固定为 `LOCALE_PROVIDER icu` 加 `ICU_LOCALE 'zh-Hans-CN'` 加 `LC_COLLATE 'C'` 与 `LC_CTYPE 'C'`，即默认排序取字节序、ICU 只作为按需显式指定 `COLLATE` 时的提供者，取值以阶段 2 的 `db/bootstrap/00_database.sql` 为准，本阶段不另行取值，并删除 public schema。基线第 3 节未覆盖排序与 public schema。理由是 C 排序只按字节比较，不随 glibc 或 ICU 的 collation 版本变化而改变，B-tree 索引不会因操作系统或 ICU 升级静默失效，与升级要求回退后数据一致性零差异一致；代价是中文按 UTF-8 字节序而不是拼音序排序，档案列表的中文排序不合阅读习惯，属首版已知边界，需要拼音序的场景由应用层以显式排序键表达，不改库级 collation；另一处后果是库排序为 C 时普通 B-tree 索引直接支持 like 前缀匹配走索引，各阶段不再另建 `text_pattern_ops` 操作符类索引。删除 public schema 是为了让基线第 3.2 节的全限定名约定没有例外出口。按 C-01，该决定的落地脚本由阶段 2 交付，其 `db/bootstrap/00_database.sql` 已按本决定写为 `CREATE DATABASE ep ENCODING 'UTF8' LC_COLLATE 'C' LC_CTYPE 'C' TEMPLATE template0` 并 `DROP SCHEMA public`，本阶段只保留决定本身与其基线回写。影响范围是基线第 3.1 节增加两条取值。

新增决定三，`shared_preload_libraries` 开启 `pg_stat_statements`。理由是附录 A.1 的查询证据与慢查询定位需要，开销可控。按 C-01，落地由阶段 2 的集群参数脚本交付。影响范围是基线第 3 节增加一条数据库实例参数。

新增决定四，运行期账号在 22 个 schema 上不授予 DELETE，仅在 platform_msg 与 platform_ops 上授予。按 C-01，该决定连同其迁移文件一并移交阶段 2，由阶段 2 计划第 12 节的偏离与新增决定接收并负责回写基线，本阶段不再承担其落地与回写。此处只保留决定的来源与理由：这是把基线第 3.6 节的软删除口径从 CI 静态检查升级为数据库强制，基线允许的两处清理即 platform_msg 的过期幂等键与 platform_ops 的过期指标快照通过对这两个 schema 单独授权保留，不放宽任何既有约束。

新增决定五，本阶段注册六个指标：`ep_build_info`（gauge，标签为 version 与 git_commit）、`ep_selfcheck_pending_items`（gauge，标签为 process）、`ep_db_pool_connections`（gauge，标签为 pool）、`ep_db_statement_duration_seconds`（histogram，标签为 pool 与 statement_kind）、`ep_http_request_duration_seconds`（histogram，桶为 0.05、0.1、0.25、0.5、1、2、3、5、10、30，标签为 route、method、status_class、client）、`ep_quota_throttled_total`（counter，标签为 route，取模板路径）。六者均在 `crates/platform/obs/src/metrics/registry.rs` 中由本阶段一次性注册，其中 `ep_db_pool_connections` 与 `ep_db_statement_duration_seconds` 按 C-23 由本阶段注册、由阶段 2 填充，其余四个由本阶段注册并填充，`ep_http_request_duration_seconds` 在本阶段的 HTTP 中间件栈中填充，`ep_quota_throttled_total` 在第 7.4 节的并发闸门中填充。按 C-21，事务重试指标统一为 `ep_db_tx_retries_total`，注册与填充均归阶段 2，本阶段原拟的 `ep_db_retries_total` 登记撤销。`docs/metrics-catalog.md` 的指标名唯一性校验在本阶段的 `xtask` 中实现。六者均不违反标签基数纪律。影响范围是基线第 9.2 节的基线指标清单只增加 `ep_build_info` 与 `ep_selfcheck_pending_items` 两项，另四项已在该节清单内，重复登记即构建失败，其注册方与填充方在 `docs/metrics-catalog.md` 中登记。

新增决定六，关联编号 `incident_no` 在没有共享序列的阶段以进程序号分段生成，格式与基线第 5.2 节示例一致。影响范围是基线第 5.2 节增加一条生成口径注记，并注明后续可替换为数据库序列而格式不变。

新增决定七，构建目标固定 `x86_64-unknown-linux-musl` 静态链接，运行基础镜像为 scratch，时区数据经 chrono-tz 编译进二进制，出网 TLS 用 rustls 并以配置指定 CA 文件。影响范围是基线新增一节交付形态取值，并与规格第 13.2 章的 OCI 容器要求一致。

新增决定八，CI 平台取内网自建 Forgejo 加 Woodpecker，全部门禁收敛到 `cargo xtask ci` 一个入口。影响范围是基线新增一条研发设施取值，且该取值不进入产品制品。
新增决定九，`ep-foundation` 的职责扩展。按 A-01、A-02、A-03、A-07、A-08 与 A-20，本阶段在 ep-foundation 中新增 `port::tx`、`id::marker`、`principal`、`security::context`、`capability` 五个模块，并建 `port::search` 与 `port::doc` 两个空模块。理由是这五类东西被三个以上阶段的契约层同时引用，若不前移，跨模块方法签名无法在契约层表达，系统主体与能力域码会在各阶段各写一份。影响范围有四处：基线第 1.2 节 ep-foundation 一行的职责描述增加 Tx、UnitOfWork、SnapshotCtx、id::marker、capability、port::search、port::doc 七项；基线第 4 节公共列表 created_by 一行的语义列写入 `00000000-0000-7000-8000-000000000001` 字面量；基线第 10.3 节在事务写法示例之后追加一句，只读快照事务的唯一入口是 `snapshot_transact`，配合 `SET TRANSACTION SNAPSHOT` 使用；基线第 12 节增加一条纪律，各阶段按裁定 A-20 的两类落点声明能力域码与动作类别常量，即业务模块的路由落 `crates/contract/<module>/src/capability.rs`，`/api/v1/platform/` 下的平台路由落 A-20 逐阶段指名的 platform crate 的 `src/capability.rs` 并一律取 `CapabilityDomain::PlatformAdminLowcodeOps`，`ci-probe` 门控的探针路由与 `/internal/v1/` 端点不参与判定也不声明常量，`xtask configdoc` 只断言每个 `/api/v1/` 路由。

新增决定十，启动自检项按注册名标识。按 C-25，自检项不再用序号称呼，注册表为 `SelfCheckRegistry`，注册项为 `SelfCheckItem { name, title, severity, run }`，name 为 kebab-case，基线十项的名字与档位见第 5.5 节，各阶段追加项按其阶段计划登记。理由是序号在多阶段追加时必然冲突，且已经出现同一序号在不同阶段指向不同项的情况。影响范围是基线第 7.3 节由编号列表改为命名列表。

新增决定十一，单据类型码的全局唯一登记表。按 C-26，`docs/data-dictionary.md` 增加单据类型码一节，本阶段建立该节与 CI 校验 `xtask configdoc --check-doc-type-codes`，判据为该节与 `ep-platform-sequence` 的常量表逐项一致且无重复；各类型码由其单据所在阶段登记，任何阶段不得新增未在该节登记的码。影响范围是基线第 11.1 节增加档案编码格式与类型码登记表的指引。

新增决定十二，`SecurityContext` 七个字段类型的形态。基线第 1.4 节的字段表只给出 `DeviceId`、`RoleCode`、`DutyClass`、`RecordShare`、`DataScopeTag`、`RequestId`、`TraceId` 七个类型名，其后的配套枚举一段只冻结了 `AccountKind`、`ClientKind`、`DepartmentScope` 三个枚举，七个类型的形态在规格、PRD、基线与裁定表中均无定义，而按 A-03 其交付方同为本阶段，不给形态则该结构体写不出可编译的定义。取值见第 5.1 节。理由与代价：`DutyClass` 的六个取值与阶段 4 的 `platform_authz.roles.duty_class` 列取值同源，互斥关系属该阶段的职责分离种子规则，不进枚举定义；`RecordShare` 只表达一条记录被显式共享给当前主体，不承载判定，`RecordScope` 与 `RecordPredicate` 留在 ep-platform-authz，否则判定语义前移进 foundation 会与基线第 1.3 节的分层冲突；`TraceId` 与 `RequestId` 的形态在基线中原本只有日志样例与请求头描述，本决定把它们写成唯一形态定义。影响范围有两处：基线第 1.4 节的配套枚举一段由三个枚举扩为三个枚举加上述七个字段类型与 `RecordShareGrant`；基线第 5.6 节的请求头一节写入 `X-Request-Id` 与 `X-Device-Id` 的形态，与本决定逐字一致。
新增决定十三，启动自检分两档并删除三项。`SelfCheckItem` 的 severity 取值域定死为 Blocking 与 Degrading 两值，第 5.4 节状态机的守卫由点名 `offsite-sink-requirements` 改为按档位判定，并写死一条禁令：任何阶段不得注册判读业务数据行的 Blocking 项。基线第 7.3 节的十三项删去三项，余十项。删 `license-and-modules-consistent`，理由是规格第 3.4 章明写平台不因许可状态停机、用量超上限不阻断业务、身份四项处置在任何许可状态下均可用，而以退出码 78 拒绝启动使规格设计的受限运行态整个不可达，承接方是规格第 3.4 章已有的四态机与阶段 3b 的 `ModuleLicenseQuery`；裁定 A-05 中阶段 1 登记 Pending 一句随之作废，按权威顺序规格高于裁定表。删 `current-period-open`，理由是该项缺失时按规格第 5.2 章自动建立期间，那是一次写操作而不是闸门，八个进程还会在自检阶段并发写 ledger 表，承接方下沉到阶段 9a 的过账路径。删 `cgroup-quota-matched`，理由与承接方见新增决定十四。`audit-chain-verifiable` 与 `offsite-sink-requirements` 两项定为 Degrading，理由是拒绝启动既修不好断链也补不上落点，而修复的唯一手段恰恰是人工介入，拒绝启动只会让这台没有备节点的服务器在最需要人操作的时候整体停摆。配置键 `selfcheck.pending_as_failure` 一并删除，Pending 一律不阻止启动，见假设二。影响范围是基线第 7.3 节由十三项编号列表改为十项命名列表并各带一个档位，且删去其中十三项为全部进程共有一句，改为不建库连接的四个进程对 SQL 类自检项一律标注 NotApplicable。

新增决定十四，删除 cgroup 配额生成器与配额清单，资源限额改为静态 drop-in 加一次性部署校验。取值三类见第 5.6 节，规格第 13.1 章的九行配额表仍逐行承载，只是承载物由生成结果换成随部署骨架交付的静态文件，核对由每进程每次启动一次换成部署与升级各一次，判定方为 `scripts/verify-resource-limits.sh`。理由是这台机器只有一台、并发上限 20，CPU 与内存不是稀缺资源，按可分配量折算与突发上限封顶两段算法解决的是不存在的争用，代价却是一个配置键、一份生成文件、一个自检项与一条八进程集体拒绝启动的路径；磁盘 IO 这一处真实稀缺由 backup-writer 的 `IOMax` 与两个 slice 的 `IOWeight` 次序表达。影响范围有两处：基线第 13 节的资源限额取值改为引用 `deploy/` 下的 drop-in 文件；配置键 `selfcheck.quota_manifest_path` 从配置参考中删除。

假设一，工具链版本。`rust-toolchain.toml` 的取值在本阶段首日由构建负责人按当日最新 stable 冻结并写入 ADR-0002，本计划以 1.86.0 表述仅为占位。冻结后不得单独升级，升级需另起变更并重跑可复现构建证据。这是假设而非既定事实，理由是版本号取决于冻结当日的上游发布状态。

假设二，本阶段允许 Pending 自检项存在且不阻止启动。规格第 7.3 节要求自检项失败即退出，但未规定尚未实现的项如何处置。本阶段把它定死为固定行为而不是开关：Pending 在报告中如实标注，不计入 overall 的成败，也不阻止启动，`selfcheck.pending_as_failure` 这个配置键随之删除，理由是把它置真会让阶段 1 至 13 的任何一个进程都起不来，它没有真实的取用者。一条 CI 断言保证 Pending 数量只减不增、在最后一个阶段归零。该假设一旦被认为不可接受，替代方案是让八个进程在阶段 1 就以 Degraded 启动，代价是降级状态在整个建设期一直为真，会淹没规格第 15.3 章的真实降级信号，因此不采用。

被阻塞情况的说明：本阶段不被任何业务决策阻塞。U-A-06 的错误文案未决只影响文案措辞，占位文案已满足规格第 15.1 章四要素；U-A-01 的编号规则未决不影响本阶段，编号器属后续阶段；U-A-03 与 U-A-05 已由基线第 11.2 与 11.5 节给出技术侧取值，本阶段照用；U-B-07 的记录级权限授予方式未决，本阶段按显式共享一条记录冻结 `RecordShare` 的形态并以此为临时取值，改判为按责任人、按创建人或按流程当前处理人只增加阶段 4 `ScopeCompiler` 的谓词分支，不改本结构体，改判为共享可再转授则在 `RecordShareGrant` 上增加一个变体，属加变体不改字段，由未知取值反序列化失败兜住。上述各条的切换代价均限于文案表、常量表与枚举变体的替换，不涉及数据库结构。
