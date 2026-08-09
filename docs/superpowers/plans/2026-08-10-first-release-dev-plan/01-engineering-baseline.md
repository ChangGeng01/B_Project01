## 阶段 1 工程基座与 CI

本阶段是全部后续阶段的地基。它不实现任何业务规则，不建任何业务表，不产生任何会计分录。凡涉及账务的内容一律指向规格第 5.2 章事件-分录表，本阶段不复述借贷与取价，也不预先实现其中任何一条规则。本阶段的判定标准只有一条：把共享技术基线里已经定死的每一条约定，变成可以在流水线上自动检出违反的机器判定，并交付一套可运行、可停机、可重启、可验签的空壳部署。

### 1. 本阶段的范围边界

在范围内的：Cargo workspace 与全部 crate 骨架、八个进程的空壳二进制、进程运行时装配、配置模型与启动自检框架、统一封套与错误映射、数据库集群引导与 24 个 schema 及角色、迁移执行器与迁移静态检查、连接池与会话变量注入清除、测试分层与覆盖率门禁、结构门禁与依赖方向门禁、容器与单机编排骨架、cgroup 配额生成与核对、供应链门禁与可复现构建、制品与版本号、本地开发环境。

明确不在范围内的：Tauri 四端真机 PoC 与任何客户端代码，RLS 业务表，身份认证与授权判定，Outbox 消费与审计链，KMS 与信封加密，附件正文读写，电子签章对接，任何模块的领域模型。规格第 19 章的阶段 1 含四端 PoC 与安全密码抽象，本 14 阶段划分把它们分别归入客户端阶段与安全阶段，本阶段只交付它们的挂载点，不交付实现。

### 2. 交付物清单

本阶段结束时，下列东西必须存在且可运行，逐项可由他人在一台干净的 Linux 机器上复现。

| 编号 | 交付物 | 可运行的判定方式 |
|---|---|---|
| D-01 | 单一 Cargo workspace，含全部 crate 骨架，`cargo build --workspace --locked --offline` 成功 | 构建返回 0，无 warning |
| D-02 | 八个空壳进程二进制，各自可启动、可健康、可优雅停机 | `--check` 返回 0，健康端点返回 200，SIGTERM 后 30 秒内退出码 0 |
| D-03 | `tools/ep-migrate` 一次性迁移工具，含 migrate、verify、status、manifest 四个子命令 | 在空库上执行后 24 个 schema 与 24 个历史表存在 |
| D-04 | 数据库集群引导脚本，含角色、库、schema、默认权限、postgresql.conf 与 pg_hba.conf 模板 | 引导后运行期账号无 DDL、无 BYPASSRLS、无 SUPERUSER，自检项 4 与 5 通过 |
| D-05 | 单机编排骨架，Podman Quadlet 与 Docker Compose 两套等价文件，含八个 slice 与配额 | 一条命令起全栈，`systemctl status` 全部 active |
| D-06 | cgroup 配额生成器与配额清单文件 | 生成结果与规格第 13.1 章配额表逐行一致，自检项 10 通过 |
| D-07 | 一条绿色 CI 流水线，共 11 个阶段，全部门禁可离线执行 | 全量运行不超过 60 分钟，返回 0 |
| D-08 | 结构门禁工具 `xtask`，含 archcheck、sqlcheck、codecheck、errorcodes、eventcatalog、configdoc、coverage、sbom、sign、reproduce、e2e 十一个子命令 | 每条规则有一个故意违反的负样例，负样例必须失败 |
| D-09 | `ep-testkit` 测试夹具库与 `ep-datagen` 数据集生成器骨架 | 同一 seed 两次生成结果字节一致 |
| D-10 | 覆盖率门禁，按路径分档强制 | 低于门槛即失败，有负样例证明 |
| D-11 | 制品与升级包，含八个进程镜像、迁移镜像、SBOM、签名、校验清单、回退说明 | 客户侧 `verify-release.sh` 在无网络环境下验签通过 |
| D-12 | 可复现构建证据 | 两次独立构建的二进制 SHA-256 与镜像 digest 全部相同 |
| D-13 | 本地开发环境，一条命令起 PostgreSQL 16 与全栈 | 新机器从零到跑通集成测试不超过 30 分钟 |
| D-14 | 文档骨架，含 ADR 目录、错误码表、事件目录、指标目录、配置参考、数据字典 | 六份文件存在且被 CI 校验与代码一致 |
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

#### 3.2 本阶段八个进程各自的空壳内容

| 进程 | 本阶段实现的内容 | 本阶段不实现的内容 |
|---|---|---|
| core-server | 8080 HTTP 服务器与五个系统端点、`/run/ep/ipc/core.sock` IPC 服务端、rw 与 ro 两个池、并发闸门、同步等待上限、七项自检、优雅停机 | 任何业务路由、鉴权判定、幂等存储 |
| job-worker | 8081 健康与指标、任务调度器骨架与零个已注册任务、worker 池、200 毫秒到 2 秒的退避轮询空转 | Outbox 消费、通知投递、对账 |
| portal-gateway | 8090 HTTP、不建数据库连接、经回环调用 core-server 健康端点的上游探测、新建 trace 与 X-Correlation-Id | 门户业务页面、会话、脱敏投影 |
| integration-gateway | 8082 健康与指标、出网客户端骨架含超时退避熔断、出网白名单校验、独立池 5 | 电子签章协议、证据固化 |
| plugin-host | `/run/ep/ipc/plugin.sock` IPC 服务端、零数据库连接 | WASM 宿主，wasmtime 依赖登记但 feature 默认关闭 |
| ops-agent | 9101 Prometheus 文本、9102 健康聚合、ep_ops_ro 池 2、按回环抓取其余七个进程的指标端点 | 运维台账读取、降级窗口 |
| archive-writer | 无监听、spool 目录、IPC 客户端、15 分钟周期心跳占位、core-server 不可用时落 spool 并在恢复后补写 | 事务日志归档、附件写出、审计证据写出 |
| backup-writer | 无监听、spool 目录、IPC 客户端、每日周期心跳占位 | 全量备份、校验、存量搬运 |

八个二进制 crate 名与进程名、systemd 单元名、cgroup slice 名一一对应，由 `xtask codecheck` 断言。archive-writer 与 backup-writer 在本阶段就不持有运行期应用账号，其配置结构体中根本不存在 db 段，配置里出现 db 段即启动失败，这是把规格第 7.7 章的账号边界前移到类型层。

### 4. 数据库变更

本阶段不新建任何业务表。数据库变更只有三类：集群引导、24 个 schema 与权限、迁移历史表。另有一张仅存在于测试库中的探针表，不进生产迁移目录。

#### 4.1 集群引导，由安装器以超级用户执行一次，不经 refinery

文件位于 `db/bootstrap/`，按文件名顺序执行，全部幂等，可重复执行。文件中不得出现任何口令字面量，口令由安装器从机密库读取后经 `ALTER ROLE ... PASSWORD` 单独注入，`xtask sqlcheck` 规则 SQL-020 断言这一点。

B001__cluster_roles.sql，创建角色。

| 角色 | 属性 | 说明 |
|---|---|---|
| ep_app_rw | LOGIN NOINHERIT NOSUPERUSER NOCREATEDB NOCREATEROLE NOBYPASSRLS NOREPLICATION | 运行期读写 |
| ep_analyst_ro | 同上 | 只读分析 |
| ep_ops_ro | 同上 | 运维只读 |
| ep_migrator | NOLOGIN 平时，迁移窗口由安装器临时 `ALTER ROLE ep_migrator LOGIN` 并在回收时改回 | 迁移 DDL |
| ep_breakglass | NOLOGIN 平时 | 应急，启用与回收记入审计，审计落库由审计阶段承接 |
| ep_archiver | LOGIN REPLICATION NOSUPERUSER NOBYPASSRLS | 仅流复制 |
| ep_backuper | LOGIN REPLICATION NOSUPERUSER NOBYPASSRLS | 仅流复制 |
| ep_mod_<schema>，共 24 个 | NOLOGIN | 各 schema 与其对象的属主 |

另外执行 `GRANT ep_mod_<schema> TO ep_migrator`（24 条），使迁移账号可以以属主身份建对象；执行 `ALTER ROLE ep_app_rw SET search_path = ''` 与对 ep_analyst_ro、ep_ops_ro、ep_migrator 的同样设置，把基线第 3.2 节的全限定名要求变成数据库强制；执行 `ALTER ROLE <各角色> SET timezone = 'UTC'`。

B002__database.sql，创建库与库级设置。

```sql
create database ep with owner ep_mod_platform_core template template0
  encoding 'UTF8' locale_provider icu icu_locale 'zh-Hans-CN' locale 'zh-Hans-CN';
alter database ep set timezone to 'UTC';
revoke all on database ep from public;
grant connect on database ep to ep_app_rw, ep_analyst_ro, ep_ops_ro, ep_migrator;
-- ep_archiver 与 ep_backuper 不授予 connect，二者只经 replication 连接
drop schema if exists public cascade;
```

选用 ICU 作为排序提供者并固定 `zh-Hans-CN` 是本阶段新增决定，基线第 3 节未覆盖。理由是 glibc 版本变化会静默改变排序结果并使 B-tree 索引失效，而升级要求回退后数据一致性零差异；ICU 的 collation 版本可由 `pg_database.datcollversion` 检出，本阶段把它加入迁移工具的 verify 项。删除 public schema 是为了让基线第 3.2 节的全限定名约定没有例外出口。

B003__postgres_conf.sql 之外另交付 `deploy/postgres/postgresql.conf.tmpl` 与 `pg_hba.conf.tmpl`，本阶段固定下列取值。

| 参数 | 取值 | 依据 |
|---|---|---|
| max_connections | 100 | 基线第 2 节峰值 52 的余量 |
| max_wal_senders | 8 | 基线下限 4 的余量 |
| max_replication_slots | 6 | 基线下限 3 的余量 |
| wal_level | replica | 流复制归档 |
| max_slot_wal_keep_size | 20GB | 本机事务日志保留量硬上限，假设值，见第 12 节风险 R-07 |
| timezone | UTC | 基线第 3.4 节 |
| shared_preload_libraries | pg_stat_statements | 附录 A.1 的 EXPLAIN 与慢查询证据需要，本阶段新增决定 |
| default_transaction_isolation | read committed | 基线第 8.4 节 |
| lock_timeout、statement_timeout | 库级不设，由连接池按池设置 | 基线第 10.3 节 |

pg_hba.conf 只开三类条目：本机 unix socket 上的 scram-sha-256，回环 TCP 上的 scram-sha-256，以及 `local replication ep_archiver`、`local replication ep_backuper` 两条。不开任何非回环地址。

#### 4.2 迁移文件，由 ep-migrate 以 ep_migrator 执行

| 顺序 | 迁移编号与文件名 | 内容 |
|---|---|---|
| 1 | V202608120900__platform_core_create_schemas.sql | 创建 24 个 schema，`authorization ep_mod_<schema>`；对 ep_app_rw、ep_analyst_ro 授予各 schema 的 usage；对 ep_ops_ro 只授予 platform_ops 的 usage |
| 2 | V202608120910__platform_core_default_privileges.sql | 逐 schema 逐属主角色设置默认权限：对 ep_app_rw 授 select、insert、update，不授 delete；对 platform_msg 与 platform_ops 两个 schema 额外授 delete；对 ep_analyst_ro 授 select；对 ep_app_rw 授序列的 usage 与 select |
| 3 | V202608120920__platform_core_revoke_public.sql | 逐 schema 回收 public 的全部权限，回收 ep_app_rw 对 information_schema 与 pg_catalog 之外系统对象的默认可见性中可回收的部分 |

不给 ep_app_rw 授予 DELETE，是把基线第 3.6 节的软删除口径从 CI 静态检查升级为数据库强制。基线允许的两处清理（platform_msg 的过期幂等键、platform_ops 的过期指标快照）通过对这两个 schema 单独授权保留，其余 22 个 schema 上执行 DELETE 会直接报权限错误。

每个迁移文件头部必须有 `-- rollback:` 段，本阶段三个文件的回退说明分别是删除 schema（仅当无对象时）、恢复默认权限、恢复 public 授权，均可安全逆向。

其余 21 个 schema 在本阶段没有迁移文件。ep-migrate 在首次运行时对 24 个 schema 逐一确保历史表存在，使自检项 3 的比对在零迁移的 schema 上同样成立。

#### 4.3 迁移历史表

表名 `<schema>.refinery_schema_history`，共 24 张，由 refinery 0.8 创建，结构如下，本阶段不改其结构，只在 ep-migrate 中固定其 schema 与表名参数。

| 列 | 类型 | 约束 |
|---|---|---|
| version | int4 | 主键 |
| name | varchar(255) | 非空 |
| applied_on | varchar(255) | 存 RFC3339 字符串 |
| checksum | varchar(255) | 非空 |

这四张列表由工具定义，不套用基线第 4 节的公共列，属工具自带元数据表，在 `xtask sqlcheck` 中列入白名单，白名单只有这一项。

#### 4.4 测试专用探针表，不进生产迁移目录

为了在本阶段就把基线第 3.8 节的 RLS 模板、第 4 节的公共列、第 3.7 节的乐观锁与第 3.10 节的索引命名全部跑通，`ep-testkit` 在每个临时测试库中创建 schema `ci_probe` 与下表。它不出现在 `db/migrations/` 下，不进任何交付制品，`xtask sqlcheck` 规则 SQL-030 断言 `ci_probe` 字样不出现在生产迁移目录中。

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
| SecurityContext | legal_entity_id、user_id、device_id、client、request_id、trace_id、data_scope_tags、kind（Human 或 System） | 构造函数只有两个，人类上下文与系统上下文，无第三个入口 |
| AppError | code、category、message、details、retryable、incident_no、occurred_at、advice、source | Display 不输出 source 链，避免内部信息外泄 |
| DomainEvent | 基线第 6.1 节信封字段的强类型表达，payload 为泛型 | 信封字段增删会导致编译失败，事件目录不一致由 CI 检出 |
| Redacted\<T\> | Debug 与 Display 均输出 `***`，serde 序列化为 `"***"` | 任何 secrecy 之外的敏感值统一包这一层 |

端口 trait：`Clock`（now 与 today_cn）、`IdGen`（new_id）、`Rng`（fill_bytes）、`IncidentNoGen`（next）。domain 层禁止绕过这四个端口，由 `xtask archcheck` 的符号禁令强制。

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
| SelfChecking | AllPassed | Ready | 全部已注册自检项通过，且待注册项数量小于等于当前阶段允许上限 |
| SelfChecking | PassedWithDegradation | Degraded | 仅自检项 11 未满足 |
| SelfChecking | AnyFailed | Failed | 任一其他项失败，以退出码 78 退出 |
| Ready | DegradationDetected | Degraded | 运行期检出降级条件 |
| Degraded | DegradationCleared | Ready | 条件消除 |
| Ready 或 Degraded | Sigterm | Draining | 停止接收新请求 |
| Draining | DrainComplete | Stopped | 在途请求归零或超过 drain 上限，退出码 0 |
| 任意 | Panic | Failed | 捕获后先写日志再退出，退出码 70 |

非法迁移一律返回 BUSINESS_CONFLICT 并记 ERROR，不 panic。Failed 状态下 systemd 以 `RestartPreventExitStatus=78` 不重启，避免配置错误导致重启风暴；退出码 70 允许重启。

#### 5.5 启动自检算法

自检项以注册表实现，每项是一个 `SelfCheckItem { id, title, severity, run }`。本阶段实现基线第 7.3 节的第 1、2、3、4、5、9、10 共七项，其余六项以 Pending 登记。

第 1 项，配置解析成功且无未知键，由 serde 的 deny_unknown_fields 与分层加载器返回。

第 2 项，数据库可达且服务端版本为 16.x，`timezone` 为 UTC，`max_connections` 不低于 52，`max_wal_senders` 不低于 4，`max_replication_slots` 不低于 3；不建库连接的三个进程跳过本项并在报告中标注 NotApplicable。

第 3 项，迁移清单一致。算法：对全部 24 张历史表读出 (schema, version, name, checksum) 四元组，按 schema 升序再按 version 升序排序，逐条以 `\u{1F}` 分隔拼接后取 SHA-256，与编译期常量 `EP_MIGRATION_MANIFEST_SHA256` 比对，不一致即失败。该常量由 build.rs 在构建时对 `db/migrations/` 下全部文件做同样归一化（统一 LF、去行尾空白）后计算。任何进程都不执行迁移。

第 4 项，全部带 legal_entity_id 列的表均已 ENABLE 且 FORCE 行级安全。算法：查 information_schema.columns 取出含该列的表集合，与 pg_class 的 relrowsecurity 与 relforcerowsecurity 比对，差集非空即失败；同时查 pg_roles 断言当前角色 rolbypassrls 与 rolsuper 均为假。阶段 1 该集合为空，判定平凡通过，但代码路径与用例已就位。

第 5 项，运行期账号不具备 DDL、角色管理与策略管理权限。算法：对 24 个 schema 逐一 `has_schema_privilege(current_user, s, 'CREATE')` 必须为假，`rolcreaterole` 与 `rolcreatedb` 必须为假。

第 9 项，时钟偏差小于 1 秒。算法：调用 `adjtimex` 读取 `/proc` 暴露的时间同步状态，若 STA_UNSYNC 置位或 maxerror 超过配置阈值即失败。容器内需挂载 `/proc`，编排文件已保证。

第 10 项，cgroup 取值与配额清单一致。算法：读取本进程 cgroup v2 目录下的 cpu.weight、cpu.max、memory.low、memory.max、io.weight、io.max，与只读挂载的 `/etc/ep/quotas.generated.toml` 中本进程所属行比对，任一项不等即失败。PostgreSQL 与核心两行不设 cpu.max 与 io.max，比对时要求这两个文件的值为 max。

`--check` 模式按顺序执行全部注册项与 Pending 项，输出一份 JSON 报告到 stdout 后退出，不监听端口。报告结构为 `{ process, version, items: [{ id, title, outcome: PASSED|FAILED|DEGRADED|PENDING|NOT_APPLICABLE, detail }], overall }`。任一 FAILED 即退出码 78。

#### 5.6 cgroup 配额生成算法

输入是认证服务器的总 CPU 核数 C、总内存字节 M、总磁盘 IO 带宽 B。步骤：先扣除操作系统预留，可分配量为 (C×0.98, M×0.95, B×0.90)。对规格第 13.1 章配额表的九行逐行计算，其中 Rust 核心与集成网关一行对应 app-core.slice，反向代理与运维代理一行对应 app-edge.slice，Worker 对应 app-worker.slice，插件运行时对应 app-plugin.slice，公网门户对应 app-portal.slice，两个写出组件分别对应 app-archive.slice 与 app-backup.slice，PostgreSQL 与内置搜索索引各占一个 slice。cpu.weight 取该行份额百分数乘以 100，取整后不小于 1；memory.low 与 memory.max 同值，取可分配内存乘份额并向下取整到 MiB；io.weight 同 cpu.weight 的算法。突发上限：PostgreSQL 与核心两行不设 cpu.max 与 io.max；其余各行取 min(份额×3, 40%) 乘可分配量。边界条件：九行内存之和必须小于等于可分配内存，否则生成器报错退出；九行份额之和必须等于 100，否则报错；突发上限之和允许超过 100。输出为 systemd slice 的 drop-in 文件与 `quotas.generated.toml` 两份，后者供自检项 10 比对。

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

echo 端点存在的唯一理由是让封套、错误映射、并发闸门、同步等待上限、请求头校验、追踪与日志七条横切链路在阶段 1 就有端到端用例。它由 `#[cfg(feature = "ci-probe")]` 保护，`xtask codecheck` 断言发布 profile 不启用该 feature，e2e 用例断言发布镜像上该路径返回 404。

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

`message` 与 `advice` 在本阶段全部为占位简体中文文案，文案定稿依赖 U-A-06 决策，占位文案已满足规格第 15.1 章的四要素要求，不阻塞本阶段。CI 断言这八条文案中不出现堆栈、SQL、主机名、进程名、表名与密钥字样。

请求头校验在本阶段的口径：`X-Legal-Entity-Id`、`X-Device-Id`、`X-Client`、`Authorization` 四个头只校验存在性与格式（UUID 格式、枚举取值、Bearer 前缀与 43 位 base64url），不做任何真实校验，校验点以 trait `AuthnPort` 与 `LegalEntityScopePort` 的空实现占位，由身份阶段替换。这一点在 `docs/config-reference.md` 中明写为阶段 1 临时状态，防止误认为已具备鉴权。系统端点豁免这四个头，豁免清单在代码中是一张固定表，新增豁免路径需改这张表并触发 CODEOWNERS 中的安全审查。

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

本阶段只实现 `system.ping` 与 `system.version` 两个方法。基线第 2 节规定的四类上报（写出结果、校验结论、失败事件、连接与复制槽与基础备份起止）在方法命名空间中预留为 `archive.report.*`，本阶段不实现，只在协议文档中占位并由 CI 断言未实现方法返回统一的未知方法错误而不是 panic。

spool 行为：写出进程在 core-server 不可用时把待上报帧按一帧一行追加到 `/var/lib/ep/<proc>/spool/pending.jsonl`，恢复连接后按顺序补写并在成功后截断；spool 目录容量超过配置上限时丢弃最旧记录并记 ERROR，绝不阻塞写出。本阶段以心跳帧验证该路径。

### 7. 并发与事务边界

本阶段没有业务事务，但把事务与并发的全部约束以可执行的形式固定下来。

#### 7.1 工作单元

`ep-adapter-db` 定义 `UnitOfWork::transact(ctx, f)`，闭包内提供 `TxHandle`。本阶段的实现要点：隔离级别固定 READ COMMITTED，另提供 `transact_repeatable_read` 供后续的对账与关账前校验使用，两者是仅有的两个入口；闭包返回后统一提交，返回 Err 统一回滚；闭包内不允许发起外部调用，由 `xtask archcheck` 对 `ep-app-*` 的符号禁令强制。审计与 Outbox 的写入位以 `AuditSink` 与 `OutboxSink` 两个 trait 的空实现占位，保证后续阶段接入时事务边界不需要改动。

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

#### 7.3 重试

只对尚未产生任何外部可见副作用的事务重试，触发条件为 SQLSTATE 40001 与 40P01，重试 3 次，退避 50、150、450 毫秒，计入 `ep_db_retries_total`。重试判定由 `RetryPolicy` 集中实现，业务代码不自行捕获这两个错误码，由 `xtask codecheck` 断言 `40001` 与 `40P01` 字面量只出现在该文件中。

#### 7.4 并发闸门与同步等待

并发闸门放在 core-server，理由是 portal-gateway 不建数据库连接且其取数一律经 core-server 的受控能力 API，core-server 是唯一的合流点，因此在 core 上限 20 即等于内部与门户合计上限 20。实现为 tower 的信号量层，许可数取配置值，等待超过 10 秒返回 503 与 PLATFORM.CAPACITY.CONCURRENCY_LIMIT，被拒事件计入 `ep_quota_throttled_total`。已获得许可的在途请求不受影响，不做静默降级。portal-gateway 侧另有一层按来源 IP 的限流，本阶段只交付参数与骨架。

同步等待上限 8 秒实现为 tower 的超时层，超时返回 PLATFORM.SYSTEM.SYNC_TIMEOUT。后台任务承接路径由任务阶段实现，本阶段在错误 advice 中写明该请求应改由后台任务表达。

#### 7.5 幂等

本阶段只实现请求头存在性与格式校验，返回 PLATFORM.IDEMPOTENCY.KEY_REQUIRED。四元组作用域的存储、重放与 PAYLOAD_MISMATCH 判定由平台内核阶段在 `platform_msg.idempotency_keys` 上实现，本阶段不建该表，也不在错误码表中登记 PAYLOAD_MISMATCH，避免登记了却不可能被返回。中间件已按最终形态分层，接入存储时只需注入一个 `IdempotencyStore` 实现。

#### 7.6 优雅停机与崩溃

收到 SIGTERM 后进入 Draining，停止接受新连接，等待在途请求完成，上限由配置控制默认 30 秒，超时后强制关闭并记 WARN，退出码仍为 0。systemd 的 TimeoutStopSec 取 45 秒。panic 由 catch_unwind 层捕获，先写一条含 trace_id 的 ERROR 日志，再返回 PLATFORM.SYSTEM.INTERNAL_ERROR，不中止进程；只有自检失败与配置错误才中止进程。

#### 7.7 与 Outbox 的关系

本阶段不写任何 Outbox 条目，也不消费。job-worker 的调度器已按至少一次投递与幂等消费的形态分层：`JobRegistry` 注册的每个任务声明 `consumer_name`，执行前后各有一个可注入的去重钩子。这样 Outbox 阶段接入时不需要改动调度器结构。

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
| selfcheck.quota_manifest_path | path | /etc/ep/quotas.generated.toml | 启动 |
| selfcheck.pending_as_failure | bool | false | 启动 |
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

自检：七项各自的通过与失败分支，失败时退出码 78；Pending 项在 pending_as_failure 为真与假两种取值下的不同结论。

配额生成：份额之和不等于 100 报错、内存超可分配报错、weight 取整下界、突发上限封顶到 40%、两个不设上限的行输出 max。

覆盖率合并算法：三档判定各自的通过与不通过、增量行集合为空时的处理。

领域属性测试：本阶段以 proptest 建立框架并落三条与业务无关的属性，即 `to_money` 幂等（对已是 2 位的值再舍入不变）、Money 加法结合律与交换律、UUIDv7 单调性。规格第 17.2 章要求的借贷平衡、库存守恒、核销守恒、移动加权平均单价重算、价差拆分五组不变量属于财务与库存阶段，本阶段只交付 `proptest` 的策略工具与失败用例最小化配置，并在 `docs/adr/` 中登记这五组的挂载点。

#### 9.2 集成测试场景清单

全部集成测试使用真实 PostgreSQL 16，禁止内存库替代。每个用例独占一个数据库，命名 `ep_test_<nanoid>`，用例结束即删库；容器由 testcontainers 启动，若 CI 主机已有实例则复用并只建库。

| 编号 | 场景 | 判定 |
|---|---|---|
| IT-01 | 空库执行 ep-migrate migrate | 24 个 schema 与 24 张历史表存在，属主正确 |
| IT-02 | 重复执行 migrate | 幂等，无变更，退出码 0 |
| IT-03 | ep-migrate verify | 清单哈希与预期一致；篡改一个迁移文件后 verify 失败 |
| IT-04 | 默认权限矩阵 | ep_app_rw 可在 22 个业务与平台 schema 上 select/insert/update，DELETE 被拒；在 platform_msg 与 platform_ops 上 DELETE 允许 |
| IT-05 | 运行期账号无 DDL | 在任一 schema 上 create table 被拒 |
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
| IT-18 | 自检项 2 至 5 | 逐项构造失败条件（版本不符、timezone 非 UTC、max_connections 不足、迁移清单不符、账号具备 DDL）并断言退出码 78 与报告内容 |
| IT-19 | 封套一致性 | 成功与失败两种响应用 insta 快照固定字段集合与顺序 |
| IT-20 | 请求头校验 | 四个固定头逐个缺失与逐个格式错误共 8 个用例 |
| IT-21 | 幂等头 | 写请求缺 Idempotency-Key 返回 KEY_REQUIRED；带非 UUIDv7 时返回 VALIDATION |
| IT-22 | 并发闸门 | 21 并发下第 21 个等待并在 10 秒后返回 CONCURRENCY_LIMIT，指标加一 |
| IT-23 | 同步等待上限 | delay_ms 超过 8000 时返回 SYNC_TIMEOUT |
| IT-24 | panic 捕获 | 触发 panic 的探针路径返回 INTERNAL_ERROR 且进程仍存活 |
| IT-25 | 日志字段 | 每请求一条访问日志，17 个固定字段齐全，敏感值为掩码 |
| IT-26 | 指标端点 | 七个基线指标名存在，标签基数纪律断言（无 user_id、doc_no、trace_id 标签，route 为模板路径） |
| IT-27 | IPC | 帧编解码往返、超长帧拒绝、未知方法返回错误、socket 权限为 0660 与属主正确 |
| IT-28 | spool | core 不可用时落盘、恢复后补写、超上限丢最旧并记 ERROR |
| IT-29 | 优雅停机 | SIGTERM 后在途请求完成、新请求被拒、退出码 0、drain 超时路径 |
| IT-30 | 探针表模板一致性 | RLS 模板生成器输出与黄金文件逐字节一致 |
| IT-31 | ICU collation | 库的 datcollversion 与引导时记录一致，不一致时 verify 失败 |

#### 9.3 端到端用例

E2E 在单机编排上跑，覆盖规格第 17.2 章中本阶段可达的部分，不涉及业务闭环 14 步。

| 编号 | 场景 | 判定 |
|---|---|---|
| E2E-01 | 一条命令起全栈 | PostgreSQL 加八个进程加一次性迁移容器全部达到 active，九个健康端点全绿 |
| E2E-02 | 全部进程 `--check` | 九份报告 overall 为 PASSED 或 DEGRADED，退出码 0 |
| E2E-03 | 迁移未执行时启动 | 自检项 3 失败，进程以 78 退出，systemd 不重启 |
| E2E-04 | 配置未知键 | 以 78 退出，stderr 含键路径 |
| E2E-05 | cgroup 配额核对 | 八个 slice 的实际 cgroup 取值与配额清单逐行一致 |
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

一是单元测试与领域属性测试的覆盖率通过标准，本阶段以分档门禁完整实现并生效。二是法人行级隔离与越权测试集，本阶段以 `tests/rls_matrix` 交付读取、写入、更新、删除、聚合、排序、报表投影、错误信息泄漏八类断言的骨架与探针表上的实测，业务表进入后按同一套断言扩展；两个复制角色与内部对账系统安全上下文的五个入口借用测试在本阶段只建目标文件与失败占位，标注为待后续阶段填充。三是数据库适配认证套件中的迁移与锁两项的执行框架，本阶段固定迁移会话的 `lock_timeout = '5s'` 与 `statement_timeout = '30min'`，并提供在线变更耗时的实测夹具。四是混沌与故障注入六类中的进程崩溃后重启恢复一类，本阶段以 E2E-06 覆盖其可达部分，即重启后进程恢复与请求按第 15.1 章明确失败，未完成任务恢复与已确认事务零丢失属后续阶段。第 17.3 章的九项强制不变量在本阶段没有被测对象，一项都不声称通过。

### 10. 退出条件

下列每条都能由一条命令或一份自动产出的报告客观判定，全部达成才算本阶段完成。

1. `cargo build --workspace --locked --offline --release` 成功，零 warning，`-D warnings` 生效。
2. crate 清单与基线第 1.2 节逐项一致，命名前缀、目录名与 `Cargo.toml` 中的 name 三处一致，由 archcheck 断言。
3. 依赖方向的七条禁止项各有一个负样例，负样例构建必须失败；正样例全部通过。
4. 八个二进制启动、就绪、优雅停机、崩溃重启四条路径在 E2E 中全绿。
5. `ep-migrate migrate` 在空库上执行后 24 个 schema 与 24 张历史表存在，`verify` 通过，篡改后 `verify` 失败。
6. 七项自检各自的通过与失败分支均有集成测试；六项 Pending 项在报告中如实标注，且有一条 CI 断言保证未注册项数量只减不增。
7. 八条错误码在 `docs/error-codes.md` 与代码常量表中一致，重复码或缺失码即构建失败。
8. 七个指标在指标端点上可见，标签基数纪律断言通过。
9. 全部配置键在 `docs/config-reference.md` 中有条目，代码与文档逐键一致。
10. 结构门禁十一个子命令各自有负样例，负样例必须失败。
11. SQL 静态检查的全部规则各有负样例，至少覆盖 DELETE 禁令、varchar 禁令、enum 禁令、current_date 禁令、跨 schema 外键禁令、ON DELETE CASCADE 禁令、rollback 注释缺失、公共列缺失与顺序错误、命名规范违反、迁移单一职责违反十项。
12. 覆盖率四档全部达标，且有一个人为降低覆盖率的负样例使流水线变红。
13. 两次独立构建产出完全相同的二进制哈希与镜像 digest。
14. SBOM 生成成功，`cargo deny` 与依赖漏洞扫描零严重与高危，许可证清单通过。
15. 升级包结构完整，客户侧验签脚本在断网机器上通过，篡改后失败。
16. cgroup 配额生成结果与规格第 13.1 章配额表逐行一致，八个 slice 的实际取值与清单一致。
17. 阶段 1 性能回归基线五项全部有实测记录并达标。
18. 六份文档骨架存在，ADR 至少含工具链冻结、collation 选型、musl 静态链接、CI 平台选型、新增 crate 五篇。
19. 源码仓库、制品与离线依赖仓库的加密备份脚本可执行，且完成一次恢复验证并留下记录。
20. 本计划第 13 节列出的偏离与新增决定全部回写共享技术基线，回写内容经评审通过。

### 11. 与规格和 PRD 的对应

| 规格章节 | 本阶段实现的条目 | 证据 |
|---|---|---|
| 第 4.1 章 | 模块化单体的代码隔离边界，以依赖方向门禁落实 | archcheck 报告与负样例 |
| 第 4.3 章 | workspace 七层边界与八个 apps 进程一一对应 | crate 清单断言、E2E-01 |
| 第 7.1 章 | 单实例单库、24 个 schema 与模块级属主角色 | IT-01、IT-04 |
| 第 7.3 章 | 认证套件中的迁移与锁两项的执行框架，PostgreSQL 16 版本门禁 | IT-18、迁移会话超时设置 |
| 第 7.4 章 | 在线变更边界的会话参数与耗时实测夹具 | 迁移工具参数与夹具用例 |
| 第 7.7 章 | 用途分账号、会话变量唯一判据、无 BYPASSRLS、连接归还清除、两个复制角色不持业务权限 | IT-04 至 IT-09、B001 |
| 第 12.3 章 | 密码算法选型的落地约束，即 TLS 1.3、SHA-256、ECDSA 三项在制品签名与哈希链参数上的取值 | 签名脚本与 ADR |
| 第 13.1 章 | 九行配额表落为八个 cgroup slice 与配额清单，让路机制一的执行面 | D-06、E2E-05 |
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

R-07，`max_slot_wal_keep_size` 的 20GB 是假设值。规格第 7.3 章要求本机事务日志保留量有硬上限并在断链用例中实测峰值，实测值可能高于该假设。缓解是把该参数列为部署参数并在归档阶段按实测回填，本阶段在配置参考中标注为假设值。

R-08，覆盖率门槛在骨架阶段可能诱发为覆盖而写的空测试。缓解是结构门禁与负样例制度并行，且 A 档只覆盖 foundation，其余按实际代码量分档。

R-09，CI 平台选型属本阶段新增决定，若客户或团队后续改用其他 CI，门禁逻辑不应绑定平台。缓解是全部门禁收敛到 `cargo xtask ci` 一个入口，CI 配置文件只负责调度，迁移平台的成本限于重写调度文件。

#### 12.2 为后续阶段预留的扩展点

自检注册表预留第 6、7、8、11、12、13 项的注册位，注册函数签名已冻结。事务闭包内预留 `AuditSink` 与 `OutboxSink` 两个写入位，接入时不改事务边界。job-worker 的 `JobRegistry` 预留 consumer 去重钩子，Outbox 接入不改调度结构。HTTP 中间件栈预留 `AuthnPort`、`AuthzPort`、`IdempotencyStore` 三个注入点，身份与平台内核阶段直接替换空实现。IPC 方法命名空间预留 `archive.report.*` 四类上报。迁移目录与 `order.toml` 已列满 24 项，业务阶段只加文件不改顺序声明。错误码表、事件目录、指标目录三份登记文件已建立并被 CI 校验，后续阶段先登记后实现的纪律有强制点。`ep-adapter-db` 与 `ep-adapter-db-pg` 的分层已就位，多库延期不影响抽象层的稳定性。feature `ci-probe` 提供一个不进发布的探针通道，后续阶段可复用于横切链路验证。

### 13. 偏离基线与本阶段新增决定

按基线第 0 节与第 12 节的要求，本节单列全部偏离项与新增决定，每项给出理由与影响范围，并同步提出基线修订。本阶段不接受只在实现里偏离。

偏离一，新增 crate `ep-platform-runtime`。基线第 1.2 节的平台底座清单没有承载进程运行时装配的 crate。若不新增，配置加载、自检注册表、信号处理、HTTP 与 IPC 服务器骨架、健康与就绪端点这一整套代码要在八个二进制里各写一份，与文件规模纪律和单一事实源冲突。该 crate 归入 platform 层，只依赖 foundation 与其他 platform，apps 依赖它，不改变任何既有依赖方向。影响范围是基线第 1.2 节的平台底座表增加一行。

新增决定一，新增两个非交付或运维用途的 workspace 成员：`xtask` 是纯开发期工具，不进任何制品；`tools/ep-migrate` 是一次性运维工具，随制品交付，以 systemd 的 oneshot 单元在升级窗口内执行。二者都不是常驻进程，不监听端口，不属于八进程清单，不改变基线第 2 节。运行 `ep-migrate` 的操作系统账户为 `ep-migrate`，与八个进程账户互不复用，同属组 ep。影响范围是基线第 1.1 节的目录布局与第 2 节的账户说明各增加一条注记。

新增决定二，数据库 collation 提供者固定为 ICU 且 locale 为 `zh-Hans-CN`，并删除 public schema。基线第 3 节未覆盖排序与 public schema。理由见第 4.1 节。影响范围是基线第 3.1 节增加两条取值。

新增决定三，`shared_preload_libraries` 开启 `pg_stat_statements`。理由是附录 A.1 的查询证据与慢查询定位需要，开销可控。影响范围是基线第 3 节增加一条数据库实例参数。

新增决定四，运行期账号在 22 个 schema 上不授予 DELETE，仅在 platform_msg 与 platform_ops 上授予。这是把基线第 3.6 节的口径从静态检查升级为数据库强制，不放宽任何既有约束。影响范围是基线第 3.1 节角色表的权限边界一列增加说明。

新增决定五，新增三个指标：`ep_build_info`（gauge，标签为 version 与 git_commit）、`ep_db_retries_total`（counter，标签为 pool 与 sqlstate）、`ep_selfcheck_pending_items`（gauge，标签为 process）。三者均不违反标签基数纪律。影响范围是基线第 9.2 节的基线指标清单增加三项。

新增决定六，关联编号 `incident_no` 在没有共享序列的阶段以进程序号分段生成，格式与基线第 5.2 节示例一致。影响范围是基线第 5.2 节增加一条生成口径注记，并注明后续可替换为数据库序列而格式不变。

新增决定七，构建目标固定 `x86_64-unknown-linux-musl` 静态链接，运行基础镜像为 scratch，时区数据经 chrono-tz 编译进二进制，出网 TLS 用 rustls 并以配置指定 CA 文件。影响范围是基线新增一节交付形态取值，并与规格第 13.2 章的 OCI 容器要求一致。

新增决定八，CI 平台取内网自建 Forgejo 加 Woodpecker，全部门禁收敛到 `cargo xtask ci` 一个入口。影响范围是基线新增一条研发设施取值，且该取值不进入产品制品。

假设一，工具链版本。`rust-toolchain.toml` 的取值在本阶段首日由构建负责人按当日最新 stable 冻结并写入 ADR-0002，本计划以 1.86.0 表述仅为占位。冻结后不得单独升级，升级需另起变更并重跑可复现构建证据。这是假设而非既定事实，理由是版本号取决于冻结当日的上游发布状态。

假设二，`max_slot_wal_keep_size` 取 20GB。规格只要求存在硬上限并在断链用例中实测，未给数值。该值在归档阶段按实测回填。

假设三，本阶段允许 Pending 自检项存在且不阻止启动。规格第 7.3 节要求自检项失败即退出，但未规定尚未实现的项如何处置。本阶段以 `selfcheck.pending_as_failure` 表达，默认 false，并以一条 CI 断言保证 Pending 数量只减不增、在最后一个阶段归零。该假设一旦被认为不可接受，替代方案是让八个进程在阶段 1 就以 Degraded 启动，代价是降级状态在整个建设期一直为真，会淹没规格第 15.3 章的真实降级信号，因此不采用。

被阻塞情况的说明：本阶段不被任何业务决策阻塞。U-A-06 的错误文案未决只影响文案措辞，占位文案已满足规格第 15.1 章四要素；U-A-01 的编号规则未决不影响本阶段，编号器属后续阶段；U-A-03 与 U-A-05 已由基线第 11.2 与 11.5 节给出技术侧取值，本阶段照用。切换代价均限于文案表与常量表的替换，不涉及数据库结构。
