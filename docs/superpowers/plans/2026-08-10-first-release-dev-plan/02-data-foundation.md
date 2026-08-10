## 阶段 2：数据基座与隔离

### 0. 本阶段边界与前置

本阶段交付全平台唯一的数据落点与隔离机制：一个 PostgreSQL 16 实例上的 24 个 schema、七个功能角色与二十四个属主角色、迁移框架、法人行级策略、密钥域与字段级信封加密、按用途分池的连接模型、时区与数值精度的两侧一致约定，以及可被机械判定的建表合规断言。本阶段另按 A-04 交付集团、组织、部门、岗位四类表与部门层级闭包表及其读取契约，按 A-26 交付 `platform_ops.degradation_windows` 最小台账与降级登记端口。本阶段不建任何业务模块的单据与档案表，不实现任何业务用例。

三条边界先写死。一是本阶段拥有 `platform_core` 内的十二张表与 `platform_ops.degradation_windows` 一张表，以及 24 个 schema 的创建与授权；按 C-01，本阶段是 24 个 schema、七个功能角色与二十四个属主角色的唯一提供方，阶段 1 只交付目录约定与空壳，任何其他阶段不得再建 schema 或角色；业务 15 个 schema 在本阶段是空 schema，只有属主、授权与合规断言。二是本阶段拥有的是机制不是策略内容：行级策略模板由本阶段产出并强制，判据取值 `app.legal_entity_id` 由安全上下文写入，而 `SecurityContext` 的字段集合按 A-03 由阶段 1 冻结、其填充与用户授权法人集合的判定属阶段 4；审批、重新认证、职责分离同理，本阶段只提供失败即拒的端口位点。三是凡规格与 PRD 未定义而本阶段必须假设的，一律在第 12 节显式登记。

前置：阶段 1 已冻结 `rust-toolchain.toml`、workspace 根 `Cargo.toml` 的 `[workspace.dependencies]`、CI 骨架与依赖方向自检脚本，并已交付 `ep-foundation` 的 `Id`、`Money`、`Quantity`、`UnitPrice`、`Rate`、`SecurityLevel`、`AppError`、`ErrorCode`、`Clock`、`IdGen`，以及按 A-01 冻结的 `port::tx` 三个 trait 与 `id::marker` 的 22 个标记类型、按 A-02 冻结的 `SYSTEM_PRINCIPAL_ID` 与 `SYSTEM_DEVICE_ID`、按 A-03 冻结的十九字段 `SecurityContext` 与其三个配套枚举、按 A-05 冻结的 `ModuleCode`、按 A-20 冻结的 `CapabilityDomain` 与 `ActionClass`。本阶段不重定义这些类型，只为其补 PostgreSQL 编解码。阶段 1 另已按裁定 F-01 建 `port::db` 空模块，本阶段在其中补齐 `IdempotencyStore` 与 `MigrationWindowGuard` 两个端口 trait 与公共能力基线的能力描述。

阶段 1 另按 C-01 与 C-02 向本阶段移交三项：`db/bootstrap/` 五个脚本的内容、单一全局迁移 Runner 与其版本号断言、`tools/ep-migrate` 五个子命令的实现。阶段 1 保留的是目录约定、CLI 骨架与退出码约定。

本阶段与 T0 贯通线的关系先写死。阶段顺序固定为 1 → 2 → 3a → 4 → 3b-1 → T0 → 5 → 9a → 8 → 6 → 7 → 10 → 11 → 9b → 14 共十五环，其中 T0 是第六环，插在阶段 3b-1 批结束后、阶段 5 全量开工之前，是一条不新增任何范围的最薄贯通线，从阶段 5、6、9a、10、11 各取最小切片，判据是一条合同从建单走到管理层看到一个数，只跑单法人、只跑桌面端、不要求分支覆盖、不要求 scale 数据集。本阶段整体位于 T0 之前，不拆分也不推迟，其对 T0 的贡献是 T0 赖以运行的全部数据落点，逐项为：24 个 schema 与二十四个属主角色、迁移框架与空库全量执行、法人行级策略与四条会话变量、公共列与四类数值精度、`UnitOfWork` 的 `transact` 与 `snapshot_transact`、`LegalEntityDirectory` 与 `DepartmentClosureQuery`、`DegradationLedger`，以及 `ep-datagen` 的 T0 最小样本档。本阶段交付物中按规模与分支排期的三项，即 `--scale small` 的 2 法人装载、越权矩阵第一块 16 组、第 8.6 节五条观察项，不是 T0 的前置，只是本阶段自身的退出条件。T0 通过后阶段 5 至 11 在这条已贯通的骨架上加厚，加厚期对本阶段的追加只有两类：三张登记表的登记行，以及 `db/migrations/<schema>/concurrent/` 下的索引，两类都不改本阶段已交付的机制，本阶段不因加厚而重开。

跨模块调用的处置规则一并写死，原先的空实现加验收顺延通则已删除。跨模块同步调用的被调方必须与调用方同批交付，否则调用方本轮不做该调用，承载该调用的用例整体推到被调方所在批次，不得先注入空实现再回头替换，也不得把验收顺延到被调方阶段；以 Noop、Stub、Fake、Dummy 为前缀的实现类型不得出现在 `apps/core-server/src/wiring/` 与 `apps/job-worker/src/wiring/` 两个目录下的全部文件中，由阶段 1 随 xtask 交付的 archcheck 规则 `unwired-absent` 断言，出现即构建失败。唯一例外是规格把交付时点冻结在末期的三项平台能力，即 `WasmComputePort`、`RuleEvaluator` 与 `DisposalPort`，三者在其交付阶段之前不注入任何实现，改由本阶段的 `platform_ops.degradation_windows` 承载，取值一律取本阶段 `DegradationKind` 的 `PORT_NOT_IMPLEMENTED` 并由 `subject` 列记下该端口名，能力缺位时返回可重试错误或直接拒绝，不得静默按成功路径放行，也不自建第二套标记。本阶段无任何空实现注入点：审批与重新认证的端口未装配时一律拒绝，见第 4.2 节与第 5 节 A-05；`ep-testkit` 的内存幂等实现只出现在测试装配中，并由 CI 断言其不进入 `apps/*` 的依赖图。

文中的按裁定 X-nn 与按通则第 n 条只是决策出处标注，取值以本计划与共享技术基线正文为准，裁定表不构成本计划之上的权威。

---

### 1. 交付物清单

| 编号 | 交付物 | 形态 | 可运行判据 |
|---|---|---|---|
| D-01 | `tools/ep-migrate` 迁移执行器 | 独立 CLI 二进制，非运行期进程 | `ep-migrate apply` 在空库上建成 24 个 schema 与本阶段全部对象；`ep-migrate status` 输出 `platform_core.schema_history` 的单一版本；重复执行退出码 0 且无变更 |
| D-02 | `db/bootstrap/` 集群引导脚本集 | 五个文件加一个 shell 包装，需超级用户执行一次 | 在裸 PostgreSQL 16 上建成数据库 `ep`、七个功能角色与二十四个属主角色、集群参数与 `pg_hba` 片段，可重复执行 |
| D-03 | `db/migrations/` 39 个迁移文件 | SQL | 按文件版本号全序执行成功，每文件含 `-- rollback:` 段；不交付任何顺序声明文件，正确性由空库全量执行验证 |
| D-04 | `db/checks/` 合规断言集 | 13 个编号断言脚本加 `append_only_consistency.sql` | 每脚本返回 0 行，非 0 行即列出违规对象；编号脚本由 `ep-migrate check` 执行，`append_only_consistency.sql` 按 B-02 由 `xtask sqlcheck` 执行 |
| D-05 | `ep-foundation` 的 `port::db` | 库模块 | `IdempotencyStore` 与 `MigrationWindowGuard` 两个端口 trait 与公共能力基线能力描述编译通过，不含任何 PostgreSQL 专有语法 |
| D-06 | `ep-adapter-db-pg` | 库 crate | 五池连接管理、会话变量注入与清除、工作单元、重试、编解码全部实现并被集成测试覆盖 |
| D-07 | `ep-adapter-kms` | 库 crate | 按裁定 F-04，`ep_foundation::port::kms` 的 `KmsBackend` 与八个词汇类型编译通过；本 crate 的 `BuiltinKmsBackend` 实现该 trait 六个方法并通过全部用例；`HsmKmsBackend` 在 `hsm` feature 下编译通过；端口 trait 与其调用词汇不在本 crate |
| D-08 | `ep-testkit` 数据库夹具 | 库 crate 增量 | `PgTestDb::new()` 按 `ep_test_<nanoid>` 独占建库，用例结束即删库 |
| D-09 | `ep-datagen` 骨架 | 二进制 crate | 接受 `--seed` 与 `--scale`，`--scale t0` 产出 T0 最小样本的平台部分，即 1 个法人及其组织架构最小行，`--scale small` 产出 2 个法人；公共列填充器就位，业务维度的最小样本由各模块在生成器注册点上追加 |
| D-10 | `tests/rls_matrix` 的本阶段那一段 | 独立集成测试目标的增量 | 16 组法人越权用例、5 项复制角色用例、5 个系统上下文入口用例全绿；按 C-05 本阶段追加 `assert_replication_role_containment` 与 `assert_recon_context_borrow` 两个函数 |
| D-11 | `scripts/verify-connection-budget.sh` | shell | 输出八进程连接枚举并与规格第 7.7 章逐项比对，不一致即非 0 退出 |
| D-12 | `tools/ep-explain-check` | CLI | 对给定 SQL 采集 `EXPLAIN (ANALYZE, BUFFERS)` 并在出现 Seq Scan 时报错，供后续阶段提交附录 A.1 证据 |
| D-13 | 文档增量 | Markdown | `docs/data-dictionary.md` 十三张表条目、`docs/error-codes.md` 20 个新错误码、`docs/event-catalog.md` 3 个事件、`docs/metrics-catalog.md` 四个指标条目、`docs/adr/` 五篇 ADR |
| D-14 | `ep-platform-tenancy` | 库 crate 加五个迁移 | 集团、组织、部门、岗位与部门层级闭包五张表建成；`LegalEntityDirectory` 与 `DepartmentClosureQuery` 两个 trait 及其 pg 实现编译通过并被集成测试覆盖 |
| D-15 | `ep-platform-obs` 降级台账最小实现 | 库 crate 加一个迁移 | `platform_ops.degradation_windows` 建成并带两条约束；`DegradationLedger` 三个方法可用；`ep_degradation_windows_open` 指标已注册并填充 |

五篇 ADR 分别是：为什么用 refinery 加自建非事务执行器承载 `CREATE INDEX CONCURRENTLY`；为什么密文自带信封头并以行标识入 AAD；为什么盲索引截断 16 字节且不建唯一约束；为什么法人注册表不建策略并登记在未受行级策略表登记表中；为什么运行期账号在业务 schema 上不授予 DELETE。

---

### 2. crate 与进程归属

新增 crate 四个，改动 crate 四个。`tools/ep-migrate` 的骨架与退出码约定按 C-02 由阶段 1 交付，本阶段补齐五个子命令的实现，因此计入改动而非新增。

| crate | 归属层 | 装配进程 | 本阶段职责 |
|---|---|---|---|
| `ep-adapter-db-pg` | adapter | 同上 | 唯一 PostgreSQL 16 实现：五池构建、`after_connect` 与 `after_release` 钩子、RLS 会话变量、编解码、迁移历史读取、SQLSTATE 23503 的统一错误映射；`PgTx` 与 `PgUnitOfWork` 的声明与实现、`UnitOfWork` 两个方法的唯一实现、重试执行体、四个连接模型类型的定义与取值、公共能力基线到 PostgreSQL 类型与索引的映射 |
| `ep-adapter-kms` | adapter | core-server、job-worker | 内置 KMS 与 HSM 两种载体的实现，即 `ep_foundation::port::kms::KmsBackend` 的两个实现类型 `BuiltinKmsBackend` 与 `HsmKmsBackend`，后者在 `hsm` feature 下；含信封加密、字段级密钥与盲索引密钥的派生与缓存、密钥材料与密钥域状态。端口 trait 与其调用词汇按裁定 F-04 不在本 crate，在 `ep_foundation::port::kms` |
| `tools/ep-migrate` | 工具二进制 | 不属八进程，只在迁移窗口内以 `ep_migrator` 运行 | 按 C-02 补齐 `apply`、`status`、`check`、`gen-rls`、`open-window` 五个子命令的实现与六个退出码 |
| `ep-foundation` | 底座 | 全部 | 只在阶段 1 已建的 `port::kms` 空文件内补齐端口面九项：`KmsBackend` trait 与 `CipherText`、`KeyDomainId`、`BlindIndex`、`Aad`、`KeyRef`、`Signature`、`CipherEnvelope`、`KeyPurpose` 八个词汇类型；三个密码学值类型经 `lib.rs` 按既有 `pub use` 惯例再导出。不新增顶层模块，`crypto::` 一套命名按裁定 F-04 作废，理由是顶层模块已被 archcheck 规则 `foundation-module-registry` 冻结为七项。不新增业务概念。必要性按基线第 12 节通则第六条在提交说明中逐项举证使用位 |
| `ep-testkit` | 测试 | 无 | 独占库夹具、法人夹具、安全上下文夹具、越权矩阵驱动器 |
| `ep-datagen` | 工具 | 无 | 规模参数框架与公共列填充器 |
| `ep-platform-tenancy` | platform | core-server、job-worker | 按 A-04 交付组织架构五张表的迁移，以及 `LegalEntityDirectory` 与 `DepartmentClosureQuery` 两个 trait 及其 pg 实现 |
| `ep-platform-obs` | platform | 全部 | 按 A-26 交付 `DegradationLedger` trait 与其 pg 实现、`DegradationKind` 的三个初始取值，以及第 7.2 节四个指标中 `ep_db_tx_retries_total` 与 `ep_degradation_windows_open` 两项的注册，与四个指标全部的填充 |

进程侧结论：portal-gateway 与 plugin-host 不链接 `ep-adapter-db-pg`，由 CI 的 `cargo metadata` 断言强制，这是规格第 7.7 章两进程常驻连接数为零的编译期保证，不依赖运行期配置。archive-writer 与 backup-writer 同样不链接该 crate，其复制角色凭据只由各自系统账户持有，本阶段只交付其数据库侧的角色、权限与 `pg_hba` 约束，进程实现属备份阶段；两个写出进程是否投入运行不受任何遏制手段配置项的判定影响。

---

### 3. 数据库变更

#### 3.1 集群引导（超级用户执行，不走迁移）

角色与集群参数是簇级对象，`ep_migrator` 不具备角色管理权限，因此单独成路径。按 C-01，`db/bootstrap/` 下的五个文件名是集群引导的唯一出处，阶段 1 曾列出的 `B001__cluster_roles.sql`、`B002__database.sql`、`B003__postgres_conf.sql` 三个文件名作废。

`db/bootstrap/00_database.sql`：`CREATE DATABASE ep ENCODING 'UTF8' LOCALE_PROVIDER icu ICU_LOCALE 'zh-Hans-CN' LC_COLLATE 'C' LC_CTYPE 'C' TEMPLATE template0`；`ALTER DATABASE ep SET timezone = 'UTC'`；`ALTER DATABASE ep SET default_transaction_isolation = 'read committed'`；`REVOKE ALL ON DATABASE ep FROM PUBLIC`；`REVOKE ALL ON SCHEMA public FROM PUBLIC`；`DROP SCHEMA public`。建库语句的排序规则提供者取 ICU 且 locale 取 `zh-Hans-CN`，这是阶段 1 新增决定二的落地，本阶段只执行不改其取值。PostgreSQL 16 在 `LOCALE_PROVIDER icu` 下仍要求给出 `LC_COLLATE` 与 `LC_CTYPE` 且必须以 `template0` 为模板，两者取 `C` 只作语法兜底，字符串比较与排序一律由 ICU 提供者承担；`LC_CTYPE 'C'` 的代价是 `upper()` 与 `lower()` 对非 ASCII 不做大小写映射，本系统的中文场景不依赖该映射，后续阶段不得据此把建库语句改回不带 ICU 提供者的写法。

`db/bootstrap/01_roles.sql`：建七个功能角色与二十四个属主角色，逐项属性如下。

| 角色 | 属性 | 权限边界 |
|---|---|---|
| `ep_app_rw` | LOGIN NOSUPERUSER NOCREATEDB NOCREATEROLE NOREPLICATION NOBYPASSRLS NOINHERIT | 全部 schema 的 SELECT、INSERT、UPDATE；DELETE 只在 `platform_msg.idempotency_keys` 与 `platform_ops` 的过期快照表上授予；无 DDL、无角色管理、无策略管理 |
| `ep_analyst_ro` | LOGIN NOSUPERUSER NOBYPASSRLS | 全部 schema 的 SELECT；不授予 `pg_read_all_stats`，复制会话与复制槽的观察落在 `ep_ops_ro` 与阶段 14 的保留量采样 |
| `ep_ops_ro` | LOGIN NOSUPERUSER NOBYPASSRLS | 只对 `platform_ops` 的视图授 SELECT，加 `pg_read_all_stats` |
| `ep_migrator` | LOGIN NOSUPERUSER NOCREATEROLE | `CREATE ON DATABASE ep`，并被授予全部 `ep_mod_*` 角色成员资格；只在迁移窗口内启用 |
| `ep_breakglass` | LOGIN NOSUPERUSER，`NOLOGIN` 为常态，启用时 `ALTER ROLE ... LOGIN VALID UNTIL` | 单次不超过 8 小时，用后轮换 |
| `ep_archiver` | LOGIN REPLICATION NOSUPERUSER NOBYPASSRLS | 无任何业务表权限；`REVOKE CONNECT ON DATABASE ep`；只能建复制连接 |
| `ep_backuper` | LOGIN REPLICATION NOSUPERUSER NOBYPASSRLS | 同上 |
| `ep_mod_<module>` × 24 | NOLOGIN | 各 schema 与其对象的属主，仅归属与 DDL 边界 |

`db/bootstrap/02_cluster_params.sql`：`max_connections = 64`（下限假设 52，留 12 为超级用户预留与波动，`superuser_reserved_connections = 3`）；`max_wal_senders = 4`；`max_replication_slots = 3`；`wal_level = replica`；`max_slot_wal_keep_size = '350GB'`（等于附录 A.3 连续归档本机保留子项，不得高于）；`wal_keep_size = 0`；`shared_preload_libraries = 'pg_stat_statements'`；`lock_timeout = 0` 全局默认由各池覆盖。

`db/bootstrap/03_role_defaults.sql`：按角色固化超时。`ALTER ROLE ep_app_rw SET statement_timeout='10s'`、`lock_timeout='3s'`、`idle_in_transaction_session_timeout='15s'`；`ep_analyst_ro` 取 `statement_timeout='60s'`、`work_mem='64MB'`、`temp_file_limit='2GB'`；`ep_ops_ro` 取 `5s`；`ep_migrator` 取 `statement_timeout='30min'`、`lock_timeout='5s'`。角色级取值是兜底，池级 `after_connect` 再设一次，两处一致由集成测试断言。

`db/bootstrap/04_pg_hba.fragment`：`ep_archiver` 与 `ep_backuper` 只放行 `local replication` 与 `host replication ... 127.0.0.1/32 scram-sha-256`，其余地址一律 reject；`ep_breakglass` 只放行 `local`。

#### 3.2 schema 与角色映射

24 个 schema：`platform_core`、`platform_authz`、`platform_meta`、`platform_flow`、`platform_audit`、`platform_msg`、`platform_file`、`platform_ops`、`ext`，以及 `mdm`、`crm`、`cpq`、`clm`、`sales`、`procure`、`inventory`、`costing`、`project`、`service`、`finance`、`ledger`、`invoice`、`portal`、`reporting`。每个 schema 的属主为同名 `ep_mod_<schema>` 角色。每个 schema 建成后立即执行三条：`GRANT USAGE ON SCHEMA <s> TO ep_app_rw, ep_analyst_ro`；`ALTER DEFAULT PRIVILEGES FOR ROLE ep_mod_<s> IN SCHEMA <s> GRANT SELECT, INSERT, UPDATE ON TABLES TO ep_app_rw`；`ALTER DEFAULT PRIVILEGES FOR ROLE ep_mod_<s> IN SCHEMA <s> GRANT SELECT ON TABLES TO ep_analyst_ro`。默认权限里不含 DELETE，这是规格第 7.2 章仅追加约束与基线第 3.6 节禁止 DELETE 的数据库侧强制，不依赖 CI 静态检查独自承担。

#### 3.3 迁移框架

工具为 refinery 0.8 系列，全库只有一个 Runner 与一张历史表 `platform_core.schema_history`。执行顺序由该 Runner 按文件版本号全序排定，不存在任何顺序声明文件，二十四个目录只表达归属不表达先后。迁移文件放在其主要创建对象所属 schema 的目录下，不以写入方所属模块为准；文件命名 `V<YYYYMMDDHHMMSS>__<schema>_<slug>.sql`，版本号取真实时间、全局唯一且严格递增，由 `xtask sqlcheck` 断言。正确性由每个文件的版本号必须晚于其全部被引用对象保证，空库上按文件版本号全序执行成功是各阶段退出条件的判据，任何阶段不得为迁就顺序而作废并改号既有迁移。

非事务迁移的处理：`CREATE INDEX CONCURRENTLY` 不能在事务块内执行，而 refinery 默认每个迁移一个事务。解法是把此类文件放在 `db/migrations/<schema>/concurrent/` 子目录，由 `ep-migrate` 的自建执行器以自动提交模式逐条执行，成功后按 refinery 历史表的同一结构手工插入一行（version、name、applied_on、checksum）。两条路径共用同一张历史表与同一套版本号空间，`ep-migrate status` 不区分来源。该执行器约 200 行，只做读文件、算校验和、执行、写历史四件事，风险点在于中途失败会留下失效索引，因此执行前先 `DROP INDEX IF EXISTS` 同名对象，执行后校验 `pg_index.indisvalid`，无效即报错并要求人工清理。

迁移会话固定 `SET lock_timeout = '5s'` 与 `SET statement_timeout = '30min'`。迁移窗口未打开时 `ep-migrate apply` 直接拒绝并返回 `PLATFORM.DB.MIGRATION_WINDOW_CLOSED`，该错误码按 C-24 由阶段 1 登记，本阶段首次实现。任何进程启动时不执行迁移，只校验版本一致。

按 C-02，`tools/ep-migrate` 的子命令固定为五个，阶段 1 的 `migrate` 并入 `apply`、`verify` 并入 `check`、`manifest` 并入 `status`。

| 子命令 | 用途 |
|---|---|
| `apply` | 按文件版本号全序执行迁移，含 `concurrent/` 子目录的非事务路径 |
| `status` | 输出 `platform_core.schema_history` 的单一版本，`--format=manifest` 输出制品清单 |
| `check` | 执行 `db/checks/` 的十三项编号合规断言，含第 12 项排序规则一致性与第 13 项未受行级策略表登记一致性 |
| `gen-rls` | 按第 3.6 节模板生成策略语句 |
| `open-window` | 开启迁移窗口，判据与第 5 节 A-09 端点一致 |

退出码约定固定为 0 成功、2 参数错误、3 迁移窗口未打开、4 校验和不符、5 版本不一致、78 环境自检失败。

按 B-03，迁移窗口的判定另以组件形态对外提供：端口为 `ep_foundation::port::db::MigrationWindowGuard`，与 C-07 的 `IdempotencyStore` 同 crate 同模块，唯一方法为 `async fn assert_open(&self, tx: &mut dyn Tx) -> Result<(), AppError>`，未持有 `OPEN` 窗口时返回 `PLATFORM.DB.MIGRATION_WINDOW_CLOSED`，HTTP 409，分类 BUSINESS_CONFLICT；唯一实现类型为 `PgMigrationWindowGuard`，位于 `crates/adapter/db-pg/`。端口与实现均由本阶段交付，并在 `apps/core-server/src/wiring.rs` 与 `apps/job-worker/src/wiring.rs` 注入。阶段 13b 的在线 DDL 由 job-worker 的 DDL 执行器发起，在把控制交给 ep-platform-release 的编排之前调用注入实例的 `assert_open(tx)`，`ep-platform-release` 不引用该 trait。

#### 3.4 迁移编号与顺序

| 序 | 文件名 | 内容 |
|---|---|---|
| 1 | `V202609010900__platform_core_create_schema.sql` | 建 schema、设属主、授 USAGE 与默认权限 |
| 2 | `V202609010905__platform_core_conventions.sql` | 六个约定函数与三个触发器函数 |
| 3 | `V202609010910__platform_core_legal_entities.sql` | 法人注册表 |
| 4 | `V202609010915__platform_core_key_domains.sql` | 密钥域表 |
| 5 | `V202609010920__platform_core_data_keys.sql` | 数据密钥表 |
| 6 | `V202609010925__platform_core_sensitive_field_registry.sql` | 敏感字段清单 |
| 7 | `V202609010930__platform_core_append_only_registry.sql` | 仅追加表登记 |
| 8 | `V202609010935__platform_core_migration_windows.sql` | 迁移窗口与单例锁表 |
| 9 | `V202609010940__platform_core_enterprise_groups.sql` | 集团表，并为 `legal_entities.group_id` 追加同 schema 真实外键 |
| 10 | `V202609010945__platform_core_organizations.sql` | 组织表 |
| 11 | `V202609010950__platform_core_departments.sql` | 部门表 |
| 12 | `V202609010955__platform_core_positions.sql` | 岗位表 |
| 13 | `V202609011000__platform_core_department_closures.sql` | 部门层级闭包表 |
| 14 | `V202609011002__platform_core_unpoliced_table_registry.sql` | 未受行级策略表登记，并在同一文件内写入本阶段八行登记 |
| 15 | `V202609011005__platform_core_grants.sql` | 本 schema 全部对象的显式授权收口 |
| 16 至 22 | `V2026090110{10,15,20,25,30,35,40}__platform_{authz,meta,flow,audit,msg,file,ops}_create_schema.sql` | 七个平台 schema 的建 schema 与授权 |
| 23 | `V202609011045__platform_ops_create_degradation_windows.sql` | 降级窗口台账，按 A-26 排在 `platform_ops` 建 schema 之后 |
| 24 | `V202609011050__ext_create_schema.sql` | 低代码扩展 schema |
| 25 至 39 | `V2026090111{00,05,10,15,20,25,30,35,40,45,50,55}__…` 与 `V2026090112{00,05,10}__…` | 15 个业务 schema 的建 schema 与授权，文件名 slug 为 `<schema>_create_schema` |

本阶段号段整体早于全部业务模块号段。理由是 15 个业务 schema 由本阶段第 25 至 39 号迁移建立，任一业务模块的建表迁移都引用其所属 schema，其版本号必须晚于本阶段对应的建 schema 迁移；本阶段自身不引用任何由后续阶段建立的对象，因此本阶段号段内部只需保持严格递增。各业务阶段的号段据此整体排在本阶段号段之后，且不得与本阶段号段重号，由 `xtask sqlcheck` 的版本号全局唯一且严格递增断言拦截。

合计 39 个迁移文件。本阶段不向 `platform_core.sensitive_field_registry` 预置任何行，阶段 5 按 A-28 以 `db/migrations/mdm/` 下的 backfill 迁移插入 `mdm.customer_invoice_profiles` 与 `mdm.supplier_payment_profiles` 两表的 `bank_name` 与 `bank_account_no` 共四行，其中 `bank_account_no` 两行的 `is_field_encrypted` 取真；`platform_core.append_only_registry` 的登记行按 B-02 合计十四行，由阶段 3b、阶段 7、阶段 8、阶段 9a 与阶段 10 各自在本模块迁移中插入，本阶段只建登记表与一致性检查脚本。`platform_core.unpoliced_table_registry` 的登记行由建表阶段随建表迁移插入，本阶段插入本阶段八张未受策略表的八行，其余由阶段 3b、阶段 4、阶段 11、阶段 13 与阶段 14 各自补齐，缺行即 `db/checks/13` 返回非零行而迁移不通过。

#### 3.5 本阶段自有表逐表定义

本阶段自有表十三张。表一至表十一与表十三都带基线第 4 节公共列，下表只列本表特有列与约束，公共列不重复；表十二 `platform_ops.degradation_windows` 的列定义按 A-26 以阶段 14 计划为准，本阶段只建表与两条约束。凡在迁移与系统上下文中写 `created_by` 与 `updated_by` 的路径，一律按 A-02 取 `ep_foundation::SYSTEM_PRINCIPAL_ID`，即 `00000000-0000-7000-8000-000000000001`，不得自选取值。

表一 `platform_core.legal_entities`（法人注册表，不带 `legal_entity_id`，不建策略，按表十三登记）

| 列 | 类型 | 约束 |
|---|---|---|
| `id` | uuid | `pk_legal_entities` |
| `group_id` | uuid null | `fk_legal_entities_enterprise_groups`，同 schema 真实外键，`ON DELETE RESTRICT`，在第 9 号迁移中追加；集团表见表七 |
| `code` | text not null | `ck_legal_entities_code_len` 长度 1 至 64；`ux_legal_entities_code` 唯一 |
| `entity_no` | text not null | 两位数字法人码，`ck_legal_entities_entity_no_fmt` 正则 `^[0-9]{2}$`；`ux_legal_entities_entity_no` 唯一；供编号阶段的单据编号法人段取用 |
| `name` | text not null | 长度 1 至 200 |
| `short_name` | text null | 长度不超过 64 |
| `display_timezone` | text not null default 'Asia/Shanghai' | `ck_legal_entities_tz` 只允许 `Asia/Shanghai` |
| `currency_code` | text not null default 'CNY' | `ck_legal_entities_currency` 只允许 `CNY` |
| `is_active` | boolean not null default true | 档案类 |
| `deactivated_at` | timestamptz null | |
| 公共列 | | `row_version`、`created_at`、`created_by`、`updated_at`、`updated_by` |

索引：`pk_legal_entities`、`ux_legal_entities_code`、`ux_legal_entities_entity_no`、`ix_legal_entities_created_at`。该表不建 `ix_<table>_legal_entity_id_created_at`，因为无该列。

表二 `platform_core.key_domains`

| 列 | 类型 | 约束 |
|---|---|---|
| `id` | uuid | `pk_key_domains` |
| `legal_entity_id` | uuid not null | RLS 判据 |
| `domain_kind` | text not null | `ck_key_domains_kind` 首版只放行 `LEGAL_ENTITY`，`GROUP_SHARED` 为后续预留且当前不放行 |
| `state` | text not null | `ck_key_domains_state` 取 `PROVISIONING`、`ACTIVE`、`DESTROY_PLANNED`、`DESTROYED` |
| `kek_ref` | text not null | 形如 `kms://builtin/le/<uuid>` 或 `kms://hsm/slot0/le/<uuid>` |
| `kek_version` | int not null default 1 | `ck_key_domains_kek_version_pos` 大于 0 |
| `provisioned_at` | timestamptz null | |
| `destroy_planned_at` | timestamptz null | |
| `destroyed_at` | timestamptz null | |
| `destroy_evidence_ref` | text null | 销毁证明的审计引用 |
| 公共列 | | `security_level` 默认 40，`data_scope_tags`、`row_version` 与四个审计列 |

索引：`pk_key_domains`、`ux_key_domains_legal_entity_id_domain_kind`、`ix_key_domains_legal_entity_id_created_at`。策略 `rls_key_domains_le`。

表三 `platform_core.data_keys`

| 列 | 类型 | 约束 |
|---|---|---|
| `id` | uuid | `pk_data_keys` |
| `legal_entity_id` | uuid not null | RLS 判据 |
| `key_domain_id` | uuid not null | `fk_data_keys_key_domains`，同 schema 真实外键，`ON DELETE RESTRICT` |
| `purpose` | text not null | `ck_data_keys_purpose` 取 `FIELD`、`BLIND_INDEX`、`ATTACHMENT`、`ARCHIVE` |
| `security_level_scope` | smallint not null | `ck_data_keys_level` 取 10、20、30、40 |
| `version` | int not null | 大于 0 |
| `algorithm` | text not null | `ck_data_keys_alg` 取 `AES_256_GCM` 或 `HMAC_SHA256` |
| `wrapped_key` | bytea not null | KEK 信封后的 DEK |
| `wrap_kek_version` | int not null | |
| `state` | text not null | `ck_data_keys_state` 取 `ACTIVE`、`RETIRING`、`RETIRED`、`DESTROYED` |
| `activated_at` | timestamptz not null | |
| `retiring_at`、`retired_at`、`destroyed_at` | timestamptz null | |
| 公共列 | | `security_level` 默认 40 |

索引：`pk_data_keys`、`ux_data_keys_domain_purpose_scope_version`（对应列 `key_domain_id, purpose, security_level_scope, version`，因 PostgreSQL 标识符 63 字节上限按列序缩写，全称登记在 `docs/data-dictionary.md`）、`ix_data_keys_legal_entity_id_created_at`。策略 `rls_data_keys_le`。首版不使用部分索引，取当前有效密钥的写法是按 `ux` 前缀定位后 `order by version desc limit 1`。

表四 `platform_core.sensitive_field_registry`（不带 `legal_entity_id`，不建策略，按表十三登记）

| 列 | 类型 | 约束 |
|---|---|---|
| `id` | uuid | `pk_sensitive_field_registry` |
| `schema_name`、`table_name`、`column_name` | text not null | `ux_sensitive_field_registry_schema_table_column` 三列唯一。`column_name` 是逻辑列名，不含 `_enc` 后缀 |
| `category` | text not null | `ck_..._category` 取 `IDENTITY`、`CONTACT`、`ACCOUNT`、`TAX_ID`、`PAYMENT_TOKEN`、`LEGAL`、`HEALTH`，对应规格第 7.8 章至少覆盖的六类加法律与健康 |
| `security_level` | smallint not null | 10、20、30、40；未赋值时按所属对象取值的规则由 `platform_core.effective_level()` 承载 |
| `blind_index` | text not null default 'NONE' | `ck_..._bidx` 首版只放行 `NONE` 与 `EXACT`，`PREFIX` 登记为预留且当前不放行 |
| `is_field_encrypted` | boolean not null default false | 该列在物理表上是否为信封密文；取 true 时物理列集按 A-28 为 `<column_name>_enc bytea` 与 `<column_name>_key_ref text`，需要保留掩码尾数的再加 `<column_name>_tail text`，需要查重的再加 `<column_name>_bidx bytea`，且不保留同名明文列，由 `db/checks/11` 断言 |
| `blind_index_column` | text null | 盲索引列名；`blind_index` 取 `NONE` 时为空，取 `EXACT` 时形如 `bank_account_no_bidx` |
| `mask_style` | text not null default 'NONE' | 掩码样式；取值语义由阶段 4 的字段级授权解释，本阶段只承载登记 |
| `normalization` | text not null default 'TRIM_NFKC' | 取 `NONE`、`TRIM_NFKC`、`TRIM_NFKC_LOWER`、`DIGITS_ONLY` |
| `release_ref` | text not null | 批准留痕与登记来源引用；按 A-27 本阶段不接入配置发布通道，经迁移登记时取 `MIGRATION:<迁移版本号>`，经端点登记时取 `ENDPOINT:<审批记录号>`，发布通道接入由阶段 3b 反向补齐 |

本表业务列集按 C-06 冻结为上表十一列，即 `schema_name`、`table_name`、`column_name`、`category`、`security_level`、`is_field_encrypted`、`blind_index`、`blind_index_column`、`mask_style`、`normalization`、`release_ref`，公共列另按基线第 4 节，唯一约束 `ux_sensitive_field_registry_schema_table_column` 在前三列上。`approved_by` 与 `approved_at` 两列按 C-06 撤销，本阶段建表时不建这两列，理由是这两列无来源可填，经迁移登记时只能以系统主体冒充产品负责人批准，规格第 12.2 章要求的批准留痕改由 `release_ref` 承载。任何阶段不得写入本列集之外的列，也不得再声明本表另有附加列。阶段 4 只引用本表不建表，其 `platform_authz.sensitive_field_registry` 一名作废。

表五 `platform_core.append_only_registry`（不带 `legal_entity_id`，不建策略，按表十三登记）

| 列 | 类型 | 约束 |
|---|---|---|
| `id` | uuid | `pk_append_only_registry` |
| `schema_name`、`table_name` | text not null | `ux_append_only_registry_schema_table` |
| `mode` | text not null | `ck_..._mode` 取 `APPEND_ONLY` 或 `IMMUTABLE_COLUMNS` |
| `mutable_columns` | text[] not null default '{}' | 仅 `IMMUTABLE_COLUMNS` 模式使用，`ck_..._mutable` 要求 `APPEND_ONLY` 时该数组为空 |

表六 `platform_core.migration_windows`（不带 `legal_entity_id`，不建策略，按表十三登记）加单例锁表 `platform_core.migration_window_lock(id smallint primary key check (id = 1))`，该锁表同样按表十三登记

| 列 | 类型 | 约束 |
|---|---|---|
| `id` | uuid | `pk_migration_windows` |
| `state` | text not null | `ck_migration_windows_state` 取 `OPEN`、`CLOSED` |
| `approval_ref` | text not null | 双人审批引用，缺失即不可开窗 |
| `reason` | text not null | 长度不超过 2000 |
| `opened_by`、`opened_at` | uuid、timestamptz not null | |
| `expires_at` | timestamptz not null | `ck_migration_windows_expiry` 要求晚于 `opened_at` |
| `closed_by`、`closed_at` | null | |
| `close_kind` | text null | `ck_..._close_kind` 取 `MANUAL`、`EXPIRED`、`FAILED` |
| `applied_versions` | text[] not null default '{}' | 本窗口实际应用的迁移版本 |

同一时刻至多一个 `OPEN` 窗口，由对 `migration_window_lock` 的 `SELECT ... FOR UPDATE` 串行化，不用部分唯一索引，因为基线第 3.10 节禁止部分索引。
表七 `platform_core.enterprise_groups`（集团表，不带 `legal_entity_id`，不建策略，按表十三登记）

| 列 | 类型 | 约束 |
|---|---|---|
| `id` | uuid | `pk_enterprise_groups` |
| `code` | text not null | `ux_enterprise_groups_code` 唯一；长度 1 至 64 |
| `name` | text not null | 长度 1 至 200 |
| `is_active` | boolean not null default true | 档案类 |
| `deactivated_at` | timestamptz null | |

索引：`pk_enterprise_groups`、`ux_enterprise_groups_code`、`ix_enterprise_groups_created_at`。该表不建 `ix_<table>_legal_entity_id_created_at`，因为无该列。

表八 `platform_core.organizations`

| 列 | 类型 | 约束 |
|---|---|---|
| `id` | uuid | `pk_organizations` |
| `legal_entity_id` | uuid not null | RLS 判据 |
| `code` | text not null | `ux_organizations_legal_entity_id_code` 唯一 |
| `name` | text not null | 长度 1 至 200 |
| `org_kind` | text not null | `ck_organizations_org_kind` 取 `CORPORATION`、`BRANCH`、`DIVISION` |
| `parent_organization_id` | uuid null | `fk_organizations_parent`，同 schema 真实外键，`ON DELETE RESTRICT` |
| `is_active` | boolean not null default true | 档案类 |

索引：`pk_organizations`、`ux_organizations_legal_entity_id_code`、`ix_organizations_legal_entity_id_created_at`。策略 `rls_organizations_le`。

表九 `platform_core.departments`

| 列 | 类型 | 约束 |
|---|---|---|
| `id` | uuid | `pk_departments` |
| `legal_entity_id` | uuid not null | RLS 判据 |
| `organization_id` | uuid not null | `fk_departments_organizations`，同 schema 真实外键，`ON DELETE RESTRICT` |
| `code` | text not null | `ux_departments_legal_entity_id_code` 唯一 |
| `name` | text not null | 长度 1 至 200 |
| `parent_department_id` | uuid null | `fk_departments_parent`，同 schema 真实外键，`ON DELETE RESTRICT` |
| `level_no` | smallint not null | `ck_departments_level_no` 大于 0；与闭包表在同一事务内维护 |
| `is_active` | boolean not null default true | 档案类 |
| `deactivated_at` | timestamptz null | |

索引：`pk_departments`、`ux_departments_legal_entity_id_code`、`ix_departments_legal_entity_id_created_at`、`ix_departments_legal_entity_id_parent_department_id`。策略 `rls_departments_le`。阶段 4 的 `department_id` 外键目标即 `platform_core.departments(id)`。

表十 `platform_core.positions`

| 列 | 类型 | 约束 |
|---|---|---|
| `id` | uuid | `pk_positions` |
| `legal_entity_id` | uuid not null | RLS 判据 |
| `department_id` | uuid not null | `fk_positions_departments`，同 schema 真实外键，`ON DELETE RESTRICT` |
| `code` | text not null | `ux_positions_legal_entity_id_code` 唯一 |
| `name` | text not null | 长度 1 至 200 |
| `rank_no` | smallint not null | `ck_positions_rank_no` 大于 0 |
| `is_active` | boolean not null default true | 档案类 |
| `deactivated_at` | timestamptz null | |

索引：`pk_positions`、`ux_positions_legal_entity_id_code`、`ix_positions_legal_entity_id_created_at`。策略 `rls_positions_le`。阶段 4 的 `position_id` 外键目标即 `platform_core.positions(id)`。

表十一 `platform_core.department_closures`

| 列 | 类型 | 约束 |
|---|---|---|
| `id` | uuid | `pk_department_closures` |
| `legal_entity_id` | uuid not null | RLS 判据 |
| `ancestor_department_id` | uuid not null | `fk_department_closures_ancestor`，同 schema 真实外键，`ON DELETE RESTRICT` |
| `descendant_department_id` | uuid not null | `fk_department_closures_descendant`，同 schema 真实外键，`ON DELETE RESTRICT` |
| `depth` | smallint not null | `ck_department_closures_depth` 不小于 0 |

唯一约束 `ux_department_closures_pair` 在 `(ancestor_department_id, descendant_department_id)`，名称与列按 A-04 冻结，本阶段不得改写。另有索引 `ix_department_closures_legal_entity_id_created_at` 与 `ix_department_closures_le_id_descendant_id`，后者按第 3.8 节缩写规则命名，全称登记在 `docs/data-dictionary.md`。策略 `rls_department_closures_le`。

表十二 `platform_ops.degradation_windows`（降级窗口台账，不带 `legal_entity_id`，不建策略，按表十三登记）

按 A-26，本表的列定义与阶段 14 计划的同表列清单一致，本阶段不复述也不改写，只建表并交付两条约束 `ux_degradation_windows_kind_scope_closed` 与 `ck_degradation_windows_open_order`，其余两条 CHECK 与全部索引由阶段 14 追加。本表带 `scope_legal_entity_id` 与 `scope_accounting_period_id` 两个可空标注列，两者只作标注不作策略判据。本阶段另建 `subject text` 可空列，承载开窗对象的完整类型名，即端口名或平台能力名，使同一 `kind` 下的多个对象可同时开窗，阶段 14 的列清单必须含该列。`ux_degradation_windows_kind_scope_closed` 因此建在 `kind`、`subject`、`scope_legal_entity_id`、`scope_accounting_period_id` 与开窗状态五者上，约束名不变。写入端口落在 `ep-platform-obs`。

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

`DegradationKind` 由本阶段定义，初始取值三个：`OFFSITE_SINK_NOT_CONFIGURED`、`WRITER_NOT_IN_SERVICE` 与 `PORT_NOT_IMPLEMENTED`。`WRITER_NOT_IN_SERVICE` 的触发条件是客观事实，即两个写出进程中任一未运行或连续若干周期无上报，不由任何遏制手段的配置是否齐备触发，且可关闭；本阶段曾用的 `WRITER_ROLE_CONTAINMENT_MISSING` 一名作废，`ck_degradation_windows_not_suppressible` 的不可抑制取值只保留 `OFFSITE_SINK_NOT_CONFIGURED` 一项。`PORT_NOT_IMPLEMENTED` 是跨模块与平台能力缺位的唯一登记形态，按第 0 节只供 `WasmComputePort`、`RuleEvaluator` 与 `DisposalPort` 三项末期平台能力在其交付阶段之前开窗使用，开窗时由 `subject` 列记下该端口的完整类型名，交付阶段注入实现后关窗，其窗口必须可关闭。本阶段是 `DegradationKind` 的唯一定义方，终态取值清单的唯一出处是阶段 14 计划，且必须是本阶段这三项的超集，任何阶段新增取值都须同批写入阶段 14 的取值表。`DegradationScope` 承载上述两个可空标注列与 `subject` 的组合。阶段 4、阶段 9、阶段 11、阶段 13 凡登记降级窗口的路径一律调用本端口，不自建第二套台账。

表十三 `platform_core.unpoliced_table_registry`（未受行级策略表登记，不带 `legal_entity_id`，不建策略，登记本表自身）

| 列 | 类型 | 约束 |
|---|---|---|
| `id` | uuid | `pk_unpoliced_table_registry` |
| `schema_name`、`table_name` | text not null | `ux_unpoliced_table_registry_schema_table` 两列唯一 |
| `admission_basis` | text not null | `ck_unpoliced_table_registry_basis` 取 `SAME_FOR_ALL_ENTITIES` 与 `ISOLATION_OR_DEPLOYMENT_METADATA` 两值 |
| `isolation_entry` | text not null | 该表的法人可见性所落的应用层入口，长度 1 至 200 |
| `matrix_case_id` | text not null | 该入口在 `tests/rls_matrix` 中的用例标识 |

索引：`pk_unpoliced_table_registry`、`ux_unpoliced_table_registry_schema_table`、`ix_unpoliced_table_registry_created_at`。该表不建 `ix_<table>_legal_entity_id_created_at`，因为无该列。本表取代基线第 3.8 节不带 `legal_entity_id` 的表只有四类这条封闭枚举，该枚举与其中的全局配置字典一类一并删除。正向规则是：凡带 `legal_entity_id` 的表一律按第 3.6 节模板建策略；不带该列的表必须逐表登记本表一行，且 `admission_basis` 必须成立，即该表的行要么在本部署内对全部法人取值相同，要么是隔离机制自身或部署自身的元数据。该判据可机械核对，取代原先主观的不承载任何业务数据。本阶段登记八行：`platform_core` 的 `legal_entities`、`enterprise_groups`、`sensitive_field_registry`、`append_only_registry`、`migration_windows`、`migration_window_lock`、`unpoliced_table_registry`，以及 `platform_ops.degradation_windows`，八行的 `admission_basis` 均取 `ISOLATION_OR_DEPLOYMENT_METADATA`。refinery 自建的历史表不由本项目建表，不在本登记与 `db/checks/13` 的范围内。

#### 3.6 RLS 策略模板与其强制方式

策略不允许手写。迁移里只调用一个函数，模板文本由函数内部拼出，全库唯一一份。

```sql
create or replace function platform_core.apply_le_rls(p_schema text, p_table text)
returns void language plpgsql as $$
begin
  execute format('alter table %I.%I enable row level security', p_schema, p_table);
  execute format('alter table %I.%I force row level security', p_schema, p_table);
  execute format(
    'create policy %I on %I.%I using (legal_entity_id = nullif(current_setting(''app.legal_entity_id'', true), '''')::uuid) '
    'with check (legal_entity_id = nullif(current_setting(''app.legal_entity_id'', true), '''')::uuid)',
    'rls_' || p_table || '_le', p_schema, p_table);
end $$;
```

强制手段有三层。第一层是 `db/checks/03_rls_conformance.sql`，把 `pg_policies` 的 `qual` 与 `with_check` 规范化后与规范文本全等比较，任何变体即报违规。第二层是 `db/checks/02_rls_enabled.sql`，断言凡有 `legal_entity_id` 列的表 `relrowsecurity` 与 `relforcerowsecurity` 均为 true，且策略数恰为 1。第三层是运行期启动自检项 `rls-enabled-and-forced`。

会话变量在连接取用时按固定顺序设置四条：`app.legal_entity_id`、`app.user_id`、`app.request_id`、`app.trace_id`，用 `select set_config($1, $2, false)`；归还前逐项设回空串。不使用 `DISCARD ALL`。变量缺失时 `current_setting(..., true)` 返回 NULL，比较结果为 NULL，行不可见也不可写，即默认拒绝。

#### 3.7 约定函数与触发器

| 对象 | 语义 |
|---|---|
| `platform_core.business_day(ts timestamptz) returns date` | `(ts AT TIME ZONE 'Asia/Shanghai')::date`，IMMUTABLE，可被单元测试直接喂固定时刻 |
| `platform_core.business_today() returns date` | `business_day(now())`，STABLE。全库禁止 `current_date`，由 `db/checks/09` 与 CI 静态检查双重拦截 |
| `platform_core.current_legal_entity() returns uuid` | 读会话变量，缺失返回 NULL |
| `platform_core.effective_level(obj smallint, fld smallint) returns smallint` | 字段级密级未赋值时取所属对象密级 |
| `platform_core.assert_row_version_bump()` | BEFORE UPDATE 触发器函数，`NEW.row_version <> OLD.row_version + 1` 即 raise |
| `platform_core.assert_append_only()` | BEFORE UPDATE OR DELETE 触发器函数，一律 raise |
| `platform_core.assert_immutable_columns()` | BEFORE UPDATE 触发器函数，比对 `append_only_registry.mutable_columns` 之外的列有变化即 raise |
| `platform_core.attach_table_guards(p_schema text, p_table text)` | 由各阶段迁移调用，按登记表自动挂接上述触发器并调用 `apply_le_rls` |

`assert_append_only` 用于裁定 B-02 终表中 `mode` 取 `APPEND_ONLY` 的十二张表，逐表清单以 B-02 为唯一出处，本阶段不复述；`assert_immutable_columns` 用于 Outbox 与死信，Outbox 的可变列白名单为 `status`、`attempts`、`available_at`、`locked_by`、`locked_until`、`last_error`，死信的可变列白名单按 B-02 取 `state`、`repaired_by`、`repaired_at`、`approval_ref`、`discard_reason` 五列，少登记一列即在上线后拒绝修复完成与丢弃两条路径的写入。这些表的定义与其登记行属阶段 3b、7、8、9a 与 10，本阶段只交付机制与登记表，并在集成测试中用合成表验证。

触发器成本在 20 并发下可忽略，其收益是把基线第 3.7 节的乐观锁写法与第 3.6 节的仅追加口径从代码纪律变成数据库约束，代码路径遗漏时立即失败而不是静默写坏。

#### 3.8 基础索引与命名

命名前缀固定 `pk_`、`ux_`、`ix_`、`fk_`、`ck_`、`rls_`、`sq_`，由 `db/checks/06_naming.sql` 断言。每张业务表的基线索引三条按基线第 3.10 节，由 `attach_table_guards` 的姊妹函数 `platform_core.assert_baseline_indexes(p_schema, p_table, p_is_document boolean)` 在迁移末尾断言，缺失即迁移失败。标识符超 63 字节时按列序缩写并强制在数据字典登记全称，由 `db/checks/07_identifier_length.sql` 断言无被 PostgreSQL 静默截断的对象。

金额、数量、单价、比率四类列的类型正确性无法从语义推断，因此本阶段固定列名后缀：金额列以 `_amount` 结尾且必须 `numeric(18,2)`；数量列以 `_qty` 结尾且必须 `numeric(18,6)`；单价列以 `_unit_price` 结尾且必须 `numeric(18,6)`；比率与税率列以 `_rate` 结尾且必须 `numeric(9,6)`。由 `db/checks/05_numeric_precision.sql` 断言。这是本阶段新增决定，见第 12 节。

跨 schema 引用的处置在本节一并写死，基线第 3.3 节禁止跨 schema 外键这条禁令删除。凡目标单一的跨 schema 引用一律建真实外键并 `ON DELETE RESTRICT`，取复合形式 `(legal_entity_id, <ref>_id)` 指向被引用表的 `(legal_entity_id, id)` 唯一键；被跨 schema 引用的表因此另建一条唯一键 `ux_<table>_legal_entity_id_id`，名称超过 63 字节时按本节缩写规则处理并在数据字典登记全称。这条同时把基线第 3.3 节任何跨法人引用一律禁止从写入前的应用层校验升级为数据库强制，各阶段写入前校验两侧 `legal_entity_id` 相等的散落实现随之删除。保留逻辑引用列的只有三类：`ledger.vouchers`、`costing.cost_entries` 与 `inventory` 九张表的多态来源单据引用；`approval_ref` 与 `release_package_id` 两个平台侧跨越型引用；`ext` 下扩展对象与自定义字段指向业务表的元数据驱动引用。三类之外出现的跨 schema 逻辑引用列即为违规。被引用对象的建表迁移版本号更晚而无法在建表语句中直接声明的少数引用，由一条版本号晚于两侧建表迁移的 `ALTER TABLE ADD CONSTRAINT` 补建，该迁移放在引用方所属 schema 的目录下。外键违例的 SQLSTATE 23503 在 `ep-adapter-db-pg` 一处统一映射为 `PLATFORM.DB.REFERENCED_ROW_MISSING`，`details` 定位到该外键列并记录约束名，同时按应用缺陷告警，理由是写入前的契约调用本应先行拒绝；跨模块契约调用因此降级定位为给出可读错误码与业务状态判定的前置校验，引用存在性由数据库兜底。

#### 3.9 合规断言清单

`db/checks/` 十三项编号断言，全部返回 0 行为通过：01 公共列齐备；02 RLS 已启用且强制；03 策略文本与模板全等；04 时间列类型（`_at` 为 timestamptz、`_date` 与 `_on` 为 date）；05 数值精度后缀；06 命名前缀；07 标识符长度；08 无 PostgreSQL enum 类型、无函数索引、无部分索引、无 JSON 路径索引；09 无 `current_date`、无 `ON DELETE CASCADE`，且全部外键为 `ON DELETE RESTRICT`、跨 schema 外键均取第 3.8 节的复合形式；10 基线索引齐备；11 按 `platform_core.sensitive_field_registry` 的 `is_field_encrypted` 分支断言，取真的登记项断言物理表上存在 `<column_name>_enc` 列且类型为 `bytea` 且不存在同名明文列 `<column_name>`，取假的登记项只断言 `<schema_name>.<table_name>.<column_name>` 三元组在 `information_schema.columns` 中命中实际列，不施加 `bytea` 与 `_enc` 后缀断言；12 排序规则一致性，脚本名 `db/checks/12_collation_conformance.sql`，断言 `pg_database` 中本库的 `datlocprovider` 为 `i`、`daticulocale` 为 `zh-Hans-CN`、且 `datcollversion` 与 `pg_database_collation_actual_version(oid)` 相等，任一项不符即返回该库一行，该项即阶段 1 计划第 447 行 IT-31 要求在 `check` 子命令中就位的判定位，其负样例夹具由阶段 1 提供，本阶段不重复交付；13 未受行级策略表登记一致性，脚本名 `db/checks/13_unpoliced_registry.sql`，断言本项目 24 个 schema 下全部未启用行级安全的表与 `platform_core.unpoliced_table_registry` 的登记行逐行一致，多一张或少一张即返回该表一行，refinery 自建的历史表除外。

另有一个不编号的脚本 `db/checks/append_only_consistency.sql`，按 B-02 断言 `platform_core.append_only_registry` 的登记与物理表上实际挂接的触发器逐项一致，由 `xtask sqlcheck` 执行，不计入 `ep-migrate check` 的十三项。阶段 3b、阶段 7、阶段 8、阶段 9a 与阶段 10 追加合计十四行登记后由该脚本兜底。

---

### 4. 领域模型与关键算法

本阶段不涉及任何账务分录。凡与会计相关的取值与借贷口径一律指向规格第 5.2 章事件-分录表，本阶段只保证其载体的精度、时区与不可覆盖性。

#### 4.1 核心类型

`ep-adapter-db-pg` 中的四个类型 `PoolKind`、`SessionContext`、`RetryPolicy`、`ConnectionBudget` 按 C-04 由阶段 1 定义且一律留在 `ep-adapter-db-pg`，不进 `ep-foundation`，本阶段只固定其取值：`PoolKind { Rw, Ro, Worker, Integ, Ops }`；`SessionContext { legal_entity_id, user_id, request_id, trace_id }`；`RetryPolicy` 取 `max_attempts` 为 3、`backoff_ms` 为 `[50, 150, 450]`、`retryable_sqlstates` 为 `["40001", "40P01"]`；`ConnectionBudget` 取 `resident_max` 为 42、`burst_max` 为 52，`per_pool` 五项取 Rw 20、Ro 10、Worker 5、Integ 5、Ops 2。预算校验脚本 `scripts/verify-connection-budget.sh` 由本阶段交付。

事务句柄与工作单元按 A-01 冻结在 `ep-foundation` 的 `port::tx`，本阶段不重定义：`Tx`、`SnapshotCtx` 与 `UnitOfWork` 三个 trait 的签名取自该模块，契约层的跨模块方法一律写 `&mut dyn Tx`；`UnitOfWork` 的两个方法按 C-03 为 `transact` 与 `snapshot_transact`，`transact_repeatable_read` 一名作废。本阶段在 `ep-adapter-db-pg` 声明并实现 `PgUnitOfWork` 与 `PgTx` 两个类型。跨 crate 取具体句柄的唯一写法是 `tx.as_any_mut().downcast_mut::<PgTx>()`，只允许出现在 `crates/adapter/db-pg/` 内，由 `xtask archcheck` 断言其他目录不出现 `downcast_mut::<PgTx>`。一个 `UnitOfWork` 实例在装配时绑定一个池，application crate 对其取泛型参数 `U: UnitOfWork` 而不是 trait 对象。

按 C-07，本阶段在 `ep_foundation::port::db` 中定义幂等存储端口，签名见第 6 节，其表与重放实现属阶段 3a，请求头校验属阶段 1。

按裁定 F-04，KMS 能力的端口 trait 与其调用词汇落 `ep_foundation::port::kms`，不落 `ep-adapter-kms`。该模块承载端口面九项：`KmsBackend` trait 与 `CipherText`、`KeyDomainId`、`BlindIndex`、`Aad`、`KeyRef`、`Signature`、`CipherEnvelope`、`KeyPurpose` 八个词汇类型，空文件由阶段 1 建，内容由本阶段补齐。`ep-adapter-kms` 中只留 `KeyDomain`（含 `domain_kind` 与第 4.2 节四态）、`DataKey`、`BlindIndexKey` 三个密钥材料与密钥域状态类型，以及 `BuiltinKmsBackend` 与 `HsmKmsBackend` 两个载体实现类型，两个实现类型的声明位与实现位同在本 crate。全卷不再出现密钥经 `ep-adapter-kms` 取用这一说法，私钥与数据密钥材料一律不出载体。

`KmsBackend` 的方法由四个增为六个，补 `sign` 与 `verify`；签名算法在全卷已固定为 ECDSA P-256，端口不带算法参数。该 trait 无泛型方法，对象安全，装配时以 `Arc<dyn KmsBackend>` 注入，注入点为 `apps/core-server/src/wiring/` 与 `apps/job-worker/src/wiring/` 两个目录。六个方法的形态如下。

```rust
// crates/foundation/src/port/kms.rs
#[async_trait::async_trait]
pub trait KmsBackend: Send + Sync + 'static {
    async fn wrap(&self, domain: KeyDomainId, purpose: KeyPurpose, aad: &Aad, plaintext: &[u8])
        -> Result<CipherEnvelope, AppError>;
    async fn unwrap(&self, domain: KeyDomainId, aad: &Aad, envelope: &CipherEnvelope)
        -> Result<Vec<u8>, AppError>;
    // derive_blind_key 只冻结三参数形态，返回宽度是待决项，见下段
    async fn derive_blind_key(&self, legal_entity_id: Id<LegalEntity>, column_fqn: &str,
                              plaintext: &[u8]) -> Result<BlindIndex, AppError>;
    async fn sign(&self, key: &KeyRef, payload: &[u8]) -> Result<Signature, AppError>;
    async fn verify(&self, key: &KeyRef, payload: &[u8], signature: &Signature)
        -> Result<bool, AppError>;
    async fn health(&self) -> Result<(), AppError>;
}
```

本代码块的冻结范围逐项写死，不得整块读成已冻结。`wrap`、`unwrap`、`sign`、`verify`、`health` 五个方法的参数与返回类型冻结，任何阶段不得改写；`verify` 取 `Result<bool, AppError>` 而不是 `Result<(), AppError>`，`false` 表示验签不通过，由调用方映射到其已登记的错误码，本阶段不因此新增错误码。`derive_blind_key` 只冻结 `legal_entity_id`、`column_fqn`、`plaintext` 三参数形态，该形态取自第 4.4 节与阶段 5、阶段 10 的既有逐字原文；其返回宽度不随本批冻结，理由是第 4.4 节与第 11 节假设三要求 `finance.cash_accounts` 走确需唯一路径时取完整 32 字节，第 4.1 节下一段的 `BlindIndex` 现取 `[u8; 16]`，第 7 节 `EP__CRYPTO__BLIND_INDEX__BYTES` 又允许取 16 或 32，三处不能同时为真。该返回类型因此是待决项，落码前由本阶段与阶段 5、阶段 10 同批定，定前任何阶段不得据本代码块把 16 字节当作已冻结结论。

`ep-foundation` 的 `port::kms` 增：`CipherText(Vec<u8>)`、`KeyDomainId(Uuid)` 与 `BlindIndex`，三者由 `crates/foundation/src/lib.rs` 按既有 `pub use` 惯例再导出，使第 4.4 节与阶段 5、阶段 10 逐字写的 `foundation::BlindIndex` 继续成立。三者均不实现 `Debug` 与 `Display` 的明文形态，`CipherText` 的 `Debug` 输出固定为 `CipherText(len=N)`。`BlindIndex` 现取 `[u8; 16]`，该宽度按上段是待决项，不随本批冻结。

#### 4.2 密钥域状态机

状态：`PROVISIONING`、`ACTIVE`、`DESTROY_PLANNED`、`DESTROYED`。

| 起点 | 终点 | 守卫条件 |
|---|---|---|
| 无 | `PROVISIONING` | 法人存在且 `is_active`；同法人同 `domain_kind` 不存在既有域 |
| `PROVISIONING` | `ACTIVE` | KEK 已在 KMS 中生成且 `kek_ref` 可解引用；四个 `purpose` 各已生成一把 `version = 1` 的 `ACTIVE` DEK |
| `ACTIVE` | `ACTIVE` | 轮换：新 DEK 版本置 `ACTIVE`，旧版本置 `RETIRING`。同一域同一 `purpose` 的轮换互斥，由事务级建议锁保证 |
| `ACTIVE` | `DESTROY_PLANNED` | 销毁前核验通过（见 4.6）且已完成双人审批与重新认证，两者由授权阶段的端口判定，端口未装配时一律拒绝 |
| `DESTROY_PLANNED` | `ACTIVE` | 核验结论被撤销或审批被驳回，允许回退，回退写审计 |
| `DESTROY_PLANNED` | `DESTROYED` | 全部 DEK 已置 `DESTROYED`，销毁证明三项齐备并已写入审计证据 |

非法迁移一律返回 `BUSINESS_CONFLICT`。`DESTROYED` 是终态，不可逆。

DEK 状态机：`ACTIVE` → `RETIRING`（有新版本，旧密文仍可解，新写入不再用它）→ `RETIRED`（重加密完成，无存量密文引用）→ `DESTROYED`（随密钥域销毁）。`RETIRING` 到 `RETIRED` 的判定依据是重加密任务上报的残留计数为零，任务本身属后续阶段，本阶段只提供状态与端口。

#### 4.3 信封加密算法

密文自描述，单列 `bytea`，布局如下。

| 偏移 | 长度 | 内容 |
|---|---|---|
| 0 | 4 | 魔数 `EPC1` |
| 4 | 1 | 算法标识，`0x01` 表示 AES-256-GCM |
| 5 | 16 | `data_keys.id` |
| 21 | 2 | DEK 版本，大端 u16 |
| 23 | 12 | 随机 nonce |
| 35 | n | 密文加 16 字节认证标签 |

AAD 固定为三段拼接：16 字节 `legal_entity_id` 大端、UTF-8 的 `schema.table.column`、16 字节所属行的 `id`。加密步骤：一、按 `(legal_entity_id, purpose = FIELD, effective_level)` 取当前 `ACTIVE` DEK，命中进程内缓存则直接用，否则读 `data_keys` 并经 KMS `unwrap` 解封；二、生成 12 字节随机 nonce，用 `Rng` 端口取随机数，禁止直接用 `rand`；三、AES-256-GCM 加密，AAD 如上；四、拼装信封。

解密步骤：校验魔数与长度，按 `dek_id` 与版本取 DEK，DEK 为 `DESTROYED` 即返回 `PLATFORM.CRYPTO.DECRYPT_FAILED` 并带 `incident_no`；AAD 由调用方按当前行重新构造，标签校验失败返回 `PLATFORM.CRYPTO.AAD_MISMATCH`。

边界条件：明文为空串时仍加密并产生非零长度密文，避免以长度区分空与非空；明文超过 1 MB 时拒绝并要求走附件通道；nonce 复用不可能发生，因为每次加密独立随机且同一 DEK 的写入量远低于 GCM 的安全上界，该上界与实际写入量的对照写入 ADR。

行标识入 AAD 的代价是密文不能在行之间搬运，因此更正记录必须重新加密而不是复制密文；收益是数据库层的整列复制或跨行拼接无法产生可解密的结果，这与规格第 7.8 章密文不得直接用于过滤排序聚合是同一方向的约束。

#### 4.4 盲索引算法

需要检索的敏感属性另建 `<col>_bidx bytea null` 列。取值为 `HMAC-SHA256(blind_key, normalize(value))` 的前 16 字节，`blind_key` 由 `(legal_entity_id, purpose = BLIND_INDEX, schema.table.column)` 派生，派生方式为 `HKDF-SHA256(dek, info = schema.table.column)`。归一化四种取值：`NONE` 原样；`TRIM_NFKC` 去首尾空白加 NFKC；`TRIM_NFKC_LOWER` 再转小写，用于电子邮箱；`DIGITS_ONLY` 只保留数字，用于电话与银行账号。

索引为普通 btree `ix_<table>_legal_entity_id_<col>_bidx`，不建唯一约束。理由是截断到 16 字节后碰撞概率虽极低但非零，唯一约束会把碰撞变成用户不可理解的写入失败；确需唯一时改用完整 32 字节并在数据字典标注。查询语义是先按盲索引取候选集，再在应用层解密逐条精确比对，因此碰撞只影响候选集大小，不影响正确性。

按 B-04，盲索引取值的唯一计算入口是 `KmsBackend::derive_blind_key(legal_entity_id, 'schema.table.column', plaintext)` 与 `foundation::BlindIndex`，阶段 5 与阶段 10 的银行账号查重一律经该入口，不得自建哈希加盐路径。银行账号盲索引列在 mdm 的客户与供应商、finance 的资金账户上列名相同且取值入口相同，列名统一为 `bank_account_no_bidx`，但唯一性与截断长度按各自表的要求分别取值，不是同构。`finance.cash_accounts` 建唯一约束 `ux_cash_accounts_legal_entity_id_bank_account_no_bidx`，按本节的确需唯一路径取完整 32 字节并在数据字典登记全称。`mdm.customer_invoice_profiles` 与 `mdm.supplier_payment_profiles` 两表的该列取本节默认的 16 字节截断，只建本节通式的普通 btree `ix_<table>_legal_entity_id_bank_account_no_bidx`，索引名超过 63 字节时按第 3.8 节缩写并在数据字典登记全称，一律不建唯一约束，依据是 PRD 第 6.2.2 节只对资金账户要求银行账号在同一法人内不重复，客户开票要素与供应商收付款信息无该要求。

盲索引只支持等值。前缀与范围检索首版不支持，`blind_index = 'PREFIX'` 在 CHECK 中不放行。

#### 4.5 会话变量生命周期与连接分池

`after_connect` 钩子执行三件事：设置本池的 `statement_timeout`、`lock_timeout`、`idle_in_transaction_session_timeout`、`work_mem`、`temp_file_limit`；设置 `application_name` 为 `<process>/<pool>`；把四条会话变量设为空串。`after_release` 钩子把四条会话变量逐项设回空串并断言当前无未结束事务。业务代码不得直接调用 `set_config`，由 CI 静态检查断言 `ep-app-*` 与 `ep-domain-*` 中不出现该符号。

八进程连接枚举与五池的对应如下，与规格第 7.7 章逐项一致。

| 进程 | Rw | Ro | Worker | Integ | Ops | 复制 | 合计常驻 |
|---|---|---|---|---|---|---|---|
| core-server | 20 | 10 | 0 | 0 | 0 | 0 | 30 |
| job-worker | 0 | 0 | 5 | 0 | 0 | 0 | 5 |
| integration-gateway | 0 | 0 | 0 | 5 | 0 | 0 | 5 |
| portal-gateway | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| plugin-host | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| ops-agent | 0 | 0 | 0 | 0 | 2 | 0 | 2 |
| archive-writer | 0 | 0 | 0 | 0 | 0 | 1 常驻加 1 复制槽 | 0 |
| backup-writer | 0 | 0 | 0 | 0 | 0 | 备份窗口内 1 | 0 |
| 合计 | | | | | | | 42 |

只读分析池的 10 个连接全部为交互式，不再从中划出任何独占连接。复制交叉核对子系统连同其独占连接、专用语句超时、配置键与指标一并删除，理由是它对唯一现实主体无效，持有本机操作系统权限者复制数据文件不会在两张统计视图中留下任何痕迹，而它试图覆盖的复制槽堆积风险已由阶段 14 周期读取 `pg_replication_slots` 的保留量判定覆盖。规格第 7.7 章第三项遏制手段的检出能力折叠进该次采样：在同一次采样中断言不出现白名单之外的复制槽，以及不出现不属于两个写出进程的复制会话，出现即按规格第 15.3 章告警并按第 12.5 章记入审计，不新增连接、配置键、表与指标。该折叠只覆盖持续存在的未知槽与会话，不表述为完整的检测手段，其局限写入交付说明。

`ConnectionBudget` 在进程启动时按配置求和，常驻超过 42 或峰值超过 52 即以退出码 78 拒绝启动。

#### 4.6 销毁前核验与销毁证明算法

规格第 7.8 章与第 12.4 章要求销毁证明逐项列出不可读范围、仍可读范围与补足措施。算法步骤：一、按 `sensitive_field_registry` 枚举该法人下全部受字段级密钥保护的列，形成不可读范围清单，逐列给出表名、列名、密级与预估行数；二、按 schema 枚举该法人下未受字段级加密的业务表，形成仍可读范围清单，并显式标注表空间密钥不属于任一法人密钥域因此这部分在数据文件层仍可读；三、对仍可读部分逐项要求填写补足措施，取值为物理删除、匿名化或继续保留，三选一且不可为空；四、核验该密钥域内数据无保留义务，核验相关备份与归档已按其保留要求处置或已具备可独立解密的恢复材料，核验人与核验结论写入报告；五、三项任一缺失即返回 `PLATFORM.KEY_DOMAIN.DESTROY_PRECHECK_FAILED`，不生成计划。

行数为预估值，取自 `pg_class.reltuples` 而非精确计数，理由是精确计数会在 20 并发下形成长扫描；该口径写入报告，不表述为精确值。

#### 4.7 事务重试算法

序列化失败 40001 与死锁 40P01 在数据访问层重试 3 次，退避 50、150、450 毫秒。只对尚未产生任何外部可见副作用的事务重试，判定依据是 `Tx` 上的 `side_effect_marker` 标志位：一旦用例调用了写外部文件、发起外部请求或写入不可回滚资源的端口，该标志置位，置位后不重试而直接返回 `PLATFORM.DB.SERIALIZATION_RETRY_EXHAUSTED`。重试次数进 `ep_db_tx_retries_total` 指标。
#### 4.8 组织架构读取契约与部门层级闭包

按 A-04，集团、组织、部门、岗位四类表与部门层级闭包表落在 `platform_core`，读取契约落在 `ep-platform-tenancy`。两个 trait 的签名逐字如下，任何阶段不得改写。

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

`Tx` 与 `Id<T>` 的标记类型按 A-01 取自 `ep-foundation`，其中 `LegalEntity` 与 `Department` 是该处 22 项标记清单中的两项。

闭包维护算法：部门新增、改父与停用三种写入，在同一事务内全量重写该部门为根的子树，先按 `ancestor_department_id` 删除该子树的既有行，再逐层插入，`depth` 自零起，同一事务内一并维护 `departments.level_no`。不使用递归 CTE 做在线查询，理由是基线第 3.10 节要求附录 A.1 度量查询不得出现顺序扫描。`max_depth` 取 0 时只返回本部门，取值超过实际深度时按实际深度截止。

使用方与顺序：阶段 3 枚举法人的唯一入口是 `LegalEntityDirectory::list_active`；阶段 4 的部门闭包编译经 `DepartmentClosureQuery::descendant_ids`，其 `department_id` 与 `position_id` 外键目标为表九与表十；阶段 5 及其后各阶段引用组织架构一律指向表七至表十一，不另建同义表。

---

### 5. API 契约

本阶段自有端点九个，全部在 `/api/v1/platform/` 下，由 core-server 承载。全部写请求必须带 `Idempotency-Key`，其请求头校验按 C-07 属阶段 1，存储表 `platform_msg.idempotency_keys` 与重放实现属阶段 3a，本阶段只定义端口，见第 6 节。全部端点遵循基线第 5.2 节封套，不重复描述。

| 序 | 方法与路径 | 用途 |
|---|---|---|
| A-01 | `GET /api/v1/platform/key-domains` | 列出当前法人的密钥域 |
| A-02 | `GET /api/v1/platform/key-domains/{id}` | 单个密钥域详情与其 DEK 版本摘要 |
| A-03 | `POST /api/v1/platform/key-domains/actions/provision` | 为当前法人建立密钥域，幂等 |
| A-04 | `POST /api/v1/platform/key-domains/{id}/actions/rotate` | 轮换指定 purpose 的 DEK |
| A-05 | `POST /api/v1/platform/key-domains/{id}/actions/plan-destroy` | 生成销毁前核验报告并置 `DESTROY_PLANNED` |
| A-06 | `POST /api/v1/platform/key-domains/{id}/actions/cancel-destroy` | 撤销销毁计划 |
| A-07 | `GET /api/v1/platform/sensitive-fields` | 敏感字段清单只读查询 |
| A-08 | `GET /api/v1/platform/migrations` | `platform_core.schema_history` 的版本视图 |
| A-09 | `POST /api/v1/platform/migrations/actions/open-window` 与 `.../actions/close-window` | 迁移窗口开闭 |

逐项契约要点如下。

A-01。请求：分页 `page`、`page_size`，排序白名单 `created_at`、`domain_kind`。响应 `data` 为数组，元素含 `id`、`domain_kind`、`state`、`kek_version`、`provisioned_at`、`active_key_count`。权限：对象级 `platform.key_domain` 的读。错误：无。

A-02。路径参数 `id`。响应增 `keys` 数组，元素含 `purpose`、`security_level_scope`、`version`、`state`、`activated_at`，不返回 `wrapped_key`，该字段在任何响应中都不出现。当前安全上下文不可见时返回 404 与 `PLATFORM.AUTHZ.NOT_FOUND_OR_DENIED`。

A-03。请求体 `{"domain_kind": "LEGAL_ENTITY"}`。幂等语义为业务幂等加键幂等双重：同法人同 `domain_kind` 已存在 `ACTIVE` 域时直接返回该域并置 `Idempotent-Replay: true`。响应 201 或 200。错误：`PLATFORM.KEY_DOMAIN.NOT_PROVISIONED` 用于 KMS 不可用，分类 `INFRASTRUCTURE`，503，可重试。权限：安全管理员。不需重新认证，理由是建立密钥域不改变既有数据的可读性。

A-04。请求体 `{"purpose": "FIELD"}`。同一域同一 purpose 并发轮换由事务级建议锁串行，第二个请求返回 409 与 `PLATFORM.KEY_DOMAIN.ROTATION_IN_PROGRESS`。响应返回新版本号与旧版本的 `RETIRING` 状态。权限：安全管理员，需重新认证，`X-Reauth-Token` 必填，绑定的待签内容摘要为 `域 ID 加 purpose 加当前版本号`。

A-05。请求体含 `remediation` 数组，每元素 `{schema_name, table_name, action}`，`action` 取 `PHYSICAL_DELETE`、`ANONYMIZE`、`RETAIN`。响应返回完整核验报告三段。错误：`PLATFORM.KEY_DOMAIN.DESTROY_PRECHECK_FAILED`，409，`details` 逐项给出缺失的表。权限：安全管理员发起，需重新认证并需双人审批，审批判定经端口调用阶段 4，端口未装配时一律 403 与 `PLATFORM.AUTHZ.OBJECT_FORBIDDEN`。本端点不执行销毁，销毁的实际执行按 A-22 经 `DisposalPort` 由阶段 14 的 `OpsDisposalService` 承担，本阶段不实现。

A-06。无请求体。仅在 `DESTROY_PLANNED` 状态可调用，否则 409。需重新认证。

A-07。查询参数 `filter[schema_name]`、`filter[table_name]`、`filter[category]`。响应不含任何样例值。权限：安全管理员或审计管理员只读。

A-08。响应 `data` 为数组，元素含 `schema_name`、`version`、`name`、`applied_on`、`checksum`、`applied_via`（取 `TRANSACTIONAL` 或 `CONCURRENT`）。`meta` 增 `expected_version_by_binary` 与 `is_consistent`。权限：系统管理员只读。

A-09。开窗请求体 `{"approval_ref": "...", "reason": "...", "ttl_minutes": 60}`，`ttl_minutes` 上限 240。响应返回窗口 ID 与 `expires_at`。已有 `OPEN` 窗口时 409。关窗请求体 `{"window_id": "..."}`。权限：系统管理员发起，需双人审批。开窗与关窗均写审计，审计写入端口由审计阶段提供。

统一约定：所有端点对当前安全上下文不可见的记录一律返回 404 与 `PLATFORM.AUTHZ.NOT_FOUND_OR_DENIED`；对该对象类型完全无权时返回 403 与 `PLATFORM.AUTHZ.OBJECT_FORBIDDEN`。`message` 与 `advice` 为简体中文，不出现 SQL、表名、密钥引用与进程名。

按 A-20，本阶段这九个平台路由在 `crates/platform/tenancy/src/capability.rs` 中各声明一对常量，命名为 `<USECASE_SCREAMING>_DOMAIN` 与 `<USECASE_SCREAMING>_ACTION`，能力域一律取 `CapabilityDomain::PlatformAdminLowcodeOps`，四个只读端点的动作类别取 `ActionClass::Read`，五个 `actions/` 端点取 `ActionClass::Submit`。两个枚举按 A-20 由阶段 1 在 `ep-foundation` 冻结，本阶段不重定义。`xtask configdoc` 断言每个 `/api/v1/` 路由都能解析到一对常量，缺失即构建失败。

下表 18 个错误码由本阶段新增，全部登记在 `docs/error-codes.md` 与 `ep-foundation` 的 `error::codes`。

| 错误码 | 分类 | HTTP |
|---|---|---|
| `PLATFORM.DB.RLS_CONTEXT_MISSING` | INFRASTRUCTURE | 503 |
| `PLATFORM.DB.LEGAL_ENTITY_MISMATCH` | PERMISSION_DENIED | 403 |
| `PLATFORM.DB.POOL_EXHAUSTED` | INFRASTRUCTURE | 503 |
| `PLATFORM.DB.STATEMENT_TIMEOUT` | INFRASTRUCTURE | 503 |
| `PLATFORM.DB.LOCK_TIMEOUT` | BUSINESS_CONFLICT | 409 |
| `PLATFORM.DB.SERIALIZATION_RETRY_EXHAUSTED` | INFRASTRUCTURE | 503 |
| `PLATFORM.DB.MIGRATION_VERSION_MISMATCH` | INFRASTRUCTURE | 503 |
| `PLATFORM.DB.MIGRATION_WINDOW_CONFLICT` | BUSINESS_CONFLICT | 409 |
| `PLATFORM.DB.WRITE_SCALE_VIOLATION` | VALIDATION | 400 |
| `PLATFORM.DB.APPEND_ONLY_VIOLATION` | BUSINESS_CONFLICT | 409 |
| `PLATFORM.DB.ROW_VERSION_NOT_BUMPED` | BUSINESS_CONFLICT | 409 |
| `PLATFORM.KEY_DOMAIN.NOT_PROVISIONED` | INFRASTRUCTURE | 503 |
| `PLATFORM.KEY_DOMAIN.KEY_UNAVAILABLE` | INFRASTRUCTURE | 503 |
| `PLATFORM.KEY_DOMAIN.ROTATION_IN_PROGRESS` | BUSINESS_CONFLICT | 409 |
| `PLATFORM.KEY_DOMAIN.DESTROY_PRECHECK_FAILED` | BUSINESS_CONFLICT | 409 |
| `PLATFORM.CRYPTO.DECRYPT_FAILED` | INFRASTRUCTURE | 503 |
| `PLATFORM.CRYPTO.AAD_MISMATCH` | BUSINESS_CONFLICT | 409 |
| `PLATFORM.DB.REFERENCED_ROW_MISSING` | VALIDATION | 400 |

另有四个按 C-24 由阶段 1 登记、在本阶段首次实现的码：`PLATFORM.CONCURRENCY.STALE_VERSION`、`PLATFORM.AUTHZ.NOT_FOUND_OR_DENIED`、`PLATFORM.AUTHZ.OBJECT_FORBIDDEN` 与 `PLATFORM.DB.MIGRATION_WINDOW_CLOSED`，本阶段只引用不登记。`PLATFORM.CRYPTO.CIPHERTEXT_FORMAT_INVALID` 与 `PLATFORM.SENSITIVE_FIELD.NOT_REGISTERED` 归入 VALIDATION，400，由本阶段一并登记，本阶段新增合计 20 个。

---

### 6. 并发与事务边界

事务只在 application 层出现，唯一写法是工作单元闭包。本阶段交付该闭包的实现，签名按 A-01 取自 `ep_foundation::port::tx`，与基线第 10.3 节一致，两个方法按 C-03 为 `transact` 与 `snapshot_transact`。`transact` 内部依次做四件事：从指定池取连接、按当前 `SecurityContext` 写四条会话变量、开启事务、执行闭包并按结果提交或回滚，最后归还连接前清除会话变量。`snapshot_transact` 是只读快照事务的唯一入口，内部在 `REPEATABLE READ` 下配合 `SET TRANSACTION SNAPSHOT` 使用，闭包收到的是 `&dyn SnapshotCtx` 而不是 `&mut dyn Tx`。

隔离级别：业务事务固定 `READ COMMITTED`。内部对账与关账前强制校验固定单个 `REPEATABLE READ` 事务或由其导出的快照，本阶段交付 `UnitOfWork::snapshot_transact` 这一入口与 `pg_export_snapshot` 的封装，对账框架本体与执行器按 A-06 属阶段 9a 的 `ep-platform-recon`。

锁策略：更新一律带 `row_version` 谓词，受影响行数为 0 即 `PLATFORM.CONCURRENCY.STALE_VERSION`；密钥域轮换用 `pg_advisory_xact_lock(hashtextextended('key_domain:'||id||':'||purpose, 0))`；迁移窗口开闭用 `migration_window_lock` 单行 `FOR UPDATE`；迁移会话 `lock_timeout` 5 秒，业务池 3 秒。

事务预算：业务事务不超过 5 秒；读写池 `statement_timeout` 10 秒；`idle_in_transaction_session_timeout` 15 秒；只读分析池 60 秒加 `work_mem` 64 MB 加 `temp_file_limit` 2 GB；job-worker 池 300 秒；ops 池 5 秒；迁移账号 30 分钟。事务内禁止外部 HTTP 调用、文件正文读写、发送通知、长时计算与等待用户输入，由 CI 静态检查断言 `ep-app-*` 用例函数中不出现 reqwest 与文件写入符号。

与 Outbox 的关系：本阶段不实现 Outbox，但交付其赖以成立的接缝，即同一个 `Tx` 句柄可被业务写入、审计写入与 Outbox 写入共享，三者因此天然同事务。`Tx` 不提供任何逃逸出事务的方法，也不提供裸连接访问。

幂等键：按 C-07，幂等键的职责分三段，本阶段只承担中间一段，即在 `ep_foundation::port::db` 中定义端口，不校验请求头，不建表，不判等。

```rust
// crates/foundation/src/port/db.rs
pub struct IdempotencyScope { pub legal_entity_id: Id<LegalEntity>, pub user_id: Id<UserAccount>,
                              pub endpoint: String, pub key: uuid::Uuid }

#[async_trait::async_trait]
pub trait IdempotencyStore: Send + Sync {
    async fn try_begin(&self, tx: &mut dyn Tx, scope: IdempotencyScope, request_hash: [u8; 32])
        -> Result<IdempotencyOutcome, AppError>;
    async fn finish(&self, tx: &mut dyn Tx, scope: IdempotencyScope,
                    response_status: u16, response_body: &[u8]) -> Result<(), AppError>;
}
```

请求头存在性与 UUIDv7 合法性由阶段 1 的 `IdempotencyKeyHeaderGuard` 校验；`platform_msg.idempotency_keys` 建表与 `IdempotencyOutcome` 的 `FirstCall`、`Replay`、`PayloadMismatch` 三种判定由阶段 3a 实现。本阶段在 `ep-testkit` 提供内存实现供集成测试使用，CI 断言该实现不得出现在 `apps/*` 的依赖图中。

失败重试与补偿：数据库层只做序列化失败与死锁的有限重试，不做业务补偿。密钥域 provision 与 rotate 的中途失败以状态机重入解决：provision 失败时域停在 `PROVISIONING`，再次调用从缺失的 DEK 处续做；rotate 失败时新版本行若已写入则处于 `ACTIVE` 而旧版本尚未置 `RETIRING`，再次调用按版本号收敛。两者都不产生外部副作用，因此可无限次重入。

必测并发场景在本阶段的可测部分有三组：同一行的乐观锁冲突；同一密钥域同一 purpose 的并发轮换；关账前强制校验所用快照与在途写事务的交叠，本阶段用合成表验证快照语义，业务语义留给阶段 9a 的对账与总账。其余三组涉及库存、发票与信用额度，按裁定通则第四条的阶段顺序分属阶段 8、阶段 10 与阶段 6。

---

### 7. 配置项

全部新增键在 `EP__` 前缀下，`deny_unknown_fields` 生效。

| 键 | 类型 | 默认值 | 生效方式 |
|---|---|---|---|
| `EP__DB__DSN` | string | 无，必填 | 只写机密引用，形如 `secret://db/app_rw#1`；重启生效 |
| `EP__DB__POOL__RW_MAX` | u32 | 20 | 重启生效 |
| `EP__DB__POOL__RO_MAX` | u32 | 10 | 重启生效 |
| `EP__DB__POOL__WORKER_MAX` | u32 | 5 | 重启生效 |
| `EP__DB__POOL__INTEG_MAX` | u32 | 5 | 重启生效 |
| `EP__DB__POOL__OPS_MAX` | u32 | 2 | 重启生效 |
| `EP__DB__POOL__ACQUIRE_TIMEOUT_MS` | u32 | 8000 | 重启生效，与基线第 11.6 节同步等待上限 8 秒对齐 |
| `EP__DB__BUDGET__RESIDENT_MAX` | u32 | 42 | 重启生效，启动自检据此判定 |
| `EP__DB__BUDGET__PEAK_MAX` | u32 | 52 | 重启生效 |
| `EP__DB__TIMEOUT__RW_STATEMENT_MS` | u32 | 10000 | 重启生效 |
| `EP__DB__TIMEOUT__RW_LOCK_MS` | u32 | 3000 | 重启生效 |
| `EP__DB__TIMEOUT__RW_IDLE_IN_TX_MS` | u32 | 15000 | 重启生效 |
| `EP__DB__TIMEOUT__RO_STATEMENT_MS` | u32 | 60000 | 重启生效 |
| `EP__DB__TIMEOUT__WORKER_STATEMENT_MS` | u32 | 300000 | 重启生效 |
| `EP__DB__TIMEOUT__OPS_STATEMENT_MS` | u32 | 5000 | 重启生效 |
| `EP__DB__RO__WORK_MEM` | string | `64MB` | 重启生效 |
| `EP__DB__RO__TEMP_FILE_LIMIT` | string | `2GB` | 重启生效 |
| `EP__DB__RETRY__MAX_ATTEMPTS` | u32 | 3 | 热生效 |
| `EP__DB__RETRY__BACKOFF_MS` | 数组 | `[50,150,450]` | 热生效 |
| `EP__DB__MIGRATION__EXPECTED_VERSIONS_PATH` | string | `/etc/ep/migration-versions.toml` | 重启生效，二进制期望版本清单 |
| `EP__KMS__BACKEND` | 枚举 | `builtin` | 取 `builtin` 或 `hsm`；重启生效 |
| `EP__KMS__BUILTIN__MASTER_KEY_PATH` | string | `/var/lib/ep/kms/master.key` | 重启生效，权限必须 0400 且属主为本进程账户，否则拒绝启动 |
| `EP__KMS__HSM__PKCS11_MODULE` | string | 空 | `hsm` 时必填 |
| `EP__KMS__HSM__SLOT` | u32 | 0 | |
| `EP__KMS__HSM__PIN_REF` | string | `secret://kms/hsm_pin#1` | 只写引用 |
| `EP__KMS__DEK_CACHE__MAX_ENTRIES` | u32 | 512 | 热生效 |
| `EP__KMS__DEK_CACHE__TTL_S` | u32 | 300 | 热生效 |
| `EP__CRYPTO__BLIND_INDEX__BYTES` | u32 | 16 | 重启生效，取值 16 或 32，是全库默认截断长度；按第 4.4 节走确需唯一例外路径的列在 `derive_blind_key` 调用点显式取完整 32 字节而不受该键影响，首版该例外只有 `finance.cash_accounts.bank_account_no` 一列 |
| `EP__MIGRATION__WINDOW_TTL_MAX_MIN` | u32 | 240 | 热生效 |

机密不进配置：数据库口令、KMS 主密钥、HSM PIN 一律写引用，解析到 `/var/lib/ep/secrets/`，内存中用 `secrecy::SecretString` 包装。机密轮换不需重启，进程在下次取用时使用新版本，旧版本保留一个轮换窗口。

#### 7.1 启动自检项

按 C-25，自检项一律以注册名标识，不用序号。自检项另分两档严重度，`SelfCheckItem` 的 `severity` 取 `Blocking` 与 `Degrading` 两值：`Blocking` 判读的一律是二进制、环境与目录，失败即以退出码 78 退出；`Degrading` 判读的是运行期可变的业务数据行，失败不阻断启动，改为经 `DegradationLedger::open` 登记窗口、持续告警，并按本节写明的运行期后果就地收窄该项相关的功能。`--check` 模式两档均以非零退出，闸门落在部署与升级前置，不落在进程启动。本阶段承担五项：`database-reachable` 判定数据库可达且版本为 16.x、`timezone` 为 UTC、`max_connections` 不低于 52、`max_wal_senders` 不低于 4、`max_replication_slots` 不低于 3，取 `Blocking`；`migration-version-matched` 判定迁移历史版本与二进制期望版本逐 schema 一致，取 `Blocking`；`rls-enabled-and-forced` 判定全部带法人列的表已 ENABLE 且 FORCE 行级安全且运行期账号不具备 BYPASSRLS 与 SUPERUSER，只读 `pg_class` 与 `pg_roles`，取 `Blocking`；`runtime-role-privileges-bounded` 判定运行期账号不具备 DDL、角色管理与策略管理权限，取 `Blocking`；`secrets-resolvable` 拆为两段，机密可解引用且 KMS 或 HSM 可用一段取 `Blocking`，每个法人的数据加密密钥域存在一段取 `Degrading`，后者失败时登记降级窗口并告警，该法人的字段级加密写入在取不到 `ACTIVE` DEK 时返回 `PLATFORM.KEY_DOMAIN.NOT_PROVISIONED`，其余法人与其余功能不受影响。后一段必须取 `Degrading`：建立密钥域的唯一入口是第 5 节 A-03 端点，由 core-server 承载，若缺域即拒绝启动则该端点永远不可达，形成自锁，而这台服务器没有备节点。上述五项在 portal-gateway、plugin-host、archive-writer 与 backup-writer 四个进程上一律返回 `NOT_APPLICABLE`，理由是前两者不链接 `ep-adapter-db-pg`，后两者只持复制连接、不执行任何 SQL，与基线第 7.3 节列出的四个进程逐项一致。自检 runner 与 `SelfCheckRegistry` 属阶段 1，本阶段以 `DataFoundationCheck` trait 提供这五项的实现并返回结构化结论，不追加任何新的自检项名。

另按 A-26，本阶段补上阶段 1 在 `offsite-sink-requirements` 一项中预留的 `// TODO(stage-2): write degradation ledger` 一行：该项失败时经 `DegradationLedger::open` 登记 `OFFSITE_SINK_NOT_CONFIGURED` 窗口，该窗口不可抑制。两个复制角色的遏制手段是否齐备不再进入任何自检项，也不再登记降级窗口，更不得决定两个写出进程是否投入运行；`WRITER_NOT_IN_SERVICE` 只在写出进程实际未运行或连续若干周期无上报时由阶段 14 登记。

#### 7.2 指标登记与填充分工

| 指标 | 类型 | 标签 | 注册方 | 填充方 |
|---|---|---|---|---|
| `ep_db_pool_connections` | gauge | `pool` | 阶段 1，位于 `crates/platform/obs/src/metrics/registry.rs` | 本阶段 |
| `ep_db_statement_duration_seconds` | histogram | `pool`、`statement_kind` | 阶段 1，同上 | 本阶段 |
| `ep_db_tx_retries_total` | counter | `pool` 取 rw、ro、worker、integ、ops；`sqlstate` 取 40001、40P01 | 本阶段 | 本阶段 |
| `ep_degradation_windows_open` | gauge | 无 | 本阶段 | 本阶段 |

按 C-21，阶段 1 的 `ep_db_retries_total` 与阶段 3a 的 `ep_tx_retry_total` 两个登记已撤销，本阶段不得再用这两个名字。复制交叉核对指标随该子系统一并删除，`ep_replication_crosscheck_age_seconds` 与本阶段曾用的 `ep_db_replication_crosscheck_age_seconds` 两个名字都不再注册也不填充，C-22 的裁定随之失效。按 C-23，两个连接池指标由阶段 1 一次性注册，本阶段只填充。`docs/metrics-catalog.md` 的唯一性校验由阶段 1 的 `xtask` 实现，本阶段的登记行必须通过该校验。

---

### 8. 测试计划

#### 8.1 单元测试

边界是不触库、不触网、不触文件系统、不取真实时间。15 组，逐组列出被覆盖的分支。

U-01 信封编解码：魔数错、长度不足、未知算法标识、nonce 截断、标签截断五类均返回 `CIPHERTEXT_FORMAT_INVALID`。
U-02 AAD 构造：法人不同、列名不同、行标识不同三类各自触发 `AAD_MISMATCH`。
U-03 归一化四种取值的等价类：空串、纯空白、前后空白、全角数字、大小写混合、含分隔符的电话与账号。
U-04 盲索引确定性：同输入恒等；不同归一化取值产生不同结果；截断长度按配置取 16 或 32。
U-05 `Money` 编解码往返：18 位整数部分上下界、负数、零、恰好 2 位小数。
U-06 舍入：`±0.005`、`±0.015`、`±2.675` 三组中值远离零；断言 `round(round(x,6),2)` 与 `round(x,2)` 在特定输入上不等，用于固化禁止二次舍入的结论。
U-07 `Quantity` 与 `UnitPrice` 的 6 位小数往返，`Rate` 的 `13%` 存为 `0.130000`。
U-08 会话变量拼装与清除序列的顺序与内容。
U-09 连接预算求和：常驻 42 边界、峰值 52 边界、单池越界三类。
U-10 重试判定：40001 与 40P01 重试；23505、23503、42501 不重试；副作用标志置位后不重试。
U-11 密钥域状态机 6 条合法迁移与 12 条非法迁移。
U-12 DEK 状态机 3 条合法与 6 条非法。
U-13 迁移窗口状态机含到期自动关闭与 `ttl_minutes` 上限。
U-14 迁移文件名解析：合法名、缺 schema 段、版本号非 12 位、slug 含大写四类；排序稳定性。
U-15 标识符长度检查与缩写规则。

#### 8.2 领域属性测试

P-01 加解密往返恒等：任意 0 至 65536 字节明文、任意合法 AAD。
P-02 `Money` 编解码往返恒等：`numeric(18,2)` 值域内任意 Decimal。
P-03 盲索引对同一归一化输入恒等，且对 10 万个随机输入在 16 字节截断下无碰撞（固定 seed，作为回归基线而非概率证明）。
P-04 舍入幂等：`round(round(x,2),2) == round(x,2)` 恒成立。
P-05 会话变量任意写入序列后，清除操作使四条变量全为空串。

规格第 17.2 章要求的领域属性测试五组不变量（借贷平衡、库存守恒、核销守恒、移动加权平均单价重算、价差拆分）在本阶段不具备被测对象，按模块归属分属阶段 9a（借贷平衡）、阶段 8（库存守恒、移动加权平均单价重算、价差拆分）与阶段 10（核销守恒）；本阶段交付的是其数值前提，即 P-02 与 P-04。这一点在测试计划中显式声明，不得被读成本阶段已覆盖该五组。

#### 8.3 集成测试

真实 PostgreSQL 16，每用例独占 `ep_test_<nanoid>` 库，用例结束即删库。禁止内存库替代。40 项，IT-15 随复制交叉核对删除后编号不重排。

引导与迁移：IT-01 引导脚本可重复执行且幂等；IT-02 24 个 schema 建成、属主正确、`public` 已删除；IT-03 迁移全量执行后 `platform_core.schema_history` 版本齐备；IT-04 迁移重复执行无变更；IT-05 每个迁移文件含 `-- rollback:` 段（静态加执行双重）；IT-06 `CREATE INDEX CONCURRENTLY` 走非事务执行器并正确写历史，中途失败留下的无效索引被检出；IT-07 迁移窗口关闭时 `apply` 被拒；IT-08 迁移会话的 `lock_timeout` 与 `statement_timeout` 实际生效。

角色与权限：IT-09 `ep_app_rw` 非 SUPERUSER、非 BYPASSRLS、无 DDL、无角色管理、无策略管理；IT-10 `ep_app_rw` 在业务 schema 上执行 DELETE 报权限错；IT-11 `ep_analyst_ro` 写入被拒且受 RLS 约束；IT-12 `ep_ops_ro` 只能读运维视图；IT-13 `ep_archiver` 与 `ep_backuper` 对任意业务表 SELECT 被拒、DDL 被拒、`CONNECT ON DATABASE ep` 被拒；IT-14 两个复制角色从非本机地址连接被拒（容器内以第二网络地址验证）。

RLS：IT-16 策略文本与模板全等；IT-17 会话变量缺失时读为 0 行、写违反 `with_check`；IT-18 连接归还后复用不残留上下文；IT-19 `force row level security` 对表属主同样生效；IT-20 跨法人写入被 `with_check` 拒绝；IT-21 跨法人聚合查询在单一法人会话下只返回本法人合计。

约束与合规：IT-22 `row_version` 未加一的 UPDATE 被触发器拒；IT-23 `APPEND_ONLY` 表的 UPDATE 与 DELETE 被拒；IT-24 `IMMUTABLE_COLUMNS` 表只放行白名单列；IT-25 全部外键为 `ON DELETE RESTRICT` 且无 `ON DELETE CASCADE`，合成表上的跨 schema 复合外键在引用不存在的行时返回 `PLATFORM.DB.REFERENCED_ROW_MISSING`，在两侧 `legal_entity_id` 不等时被数据库直接拒绝；IT-26 十三项合规断言全部返回 0 行，含未登记的未受策略表被第 13 项检出；IT-27 `numeric(18,2)` 收到 3 位小数时 Rust 侧先行拦截并返回 `WRITE_SCALE_VIOLATION`，而非交由数据库静默舍入。

时区与精度：IT-28 数据库 `timezone` 为 UTC，且 `pg_database` 中本库的 `datlocprovider` 为 `i`、`daticulocale` 为 `zh-Hans-CN`、`datcollversion` 与 `pg_database_collation_actual_version(oid)` 相等；IT-29 `business_day()` 在 UTC 15:59:59 与 16:00:00 两个时刻分别返回相邻的两个自然日；IT-30 `timestamptz` 与 `chrono::DateTime<Utc>`、`date` 与 `NaiveDate` 的往返。

密钥域与加密：IT-31 provision 幂等；IT-32 轮换后新版本 `ACTIVE`、旧版本 `RETIRING`、旧密文仍可解；IT-33 DEK 置 `DESTROYED` 后旧密文解密返回 `DECRYPT_FAILED` 且带 `incident_no`；IT-34 法人 A 的会话无法读取法人 B 的 `data_keys` 行，且即便持有密文也因 AAD 不匹配而无法解密；IT-35 敏感字段未登记即加密写入被拒；IT-36 销毁前核验报告三项齐备，缺 `remediation` 即失败。

连接与事务：IT-37 五池各自的 `statement_timeout`、`work_mem`、`temp_file_limit` 实际生效（用 `pg_sleep` 与大排序验证）；IT-38 序列化失败重试 3 次后返回 `SERIALIZATION_RETRY_EXHAUSTED` 且重试计数进指标；另在同一项内验证 `REPEATABLE READ` 快照跨批一致读与两事务同行更新的 409。
组织架构与降级台账：IT-39 五张组织架构表建成、四张带法人列的表策略齐备，部门改父后闭包表全量重写该子树且 `ux_department_closures_pair` 无重复行，`departments.level_no` 与闭包 `depth` 一致；IT-40 `DepartmentClosureQuery::descendant_ids` 在 `max_depth` 取 0、取 1、取超过实际深度三种取值下的返回集与闭包表一致，且 `EXPLAIN` 不出现顺序扫描；IT-41 `DegradationLedger` 的 `open` 与 `close` 幂等，同一 kind、subject 与 scope 的第二个未关闭窗口被 `ux_degradation_windows_kind_scope_closed` 拒绝，同一 kind 下 subject 不同的两个窗口可同时打开，`open_count` 与 `ep_degradation_windows_open` 取值一致。

#### 8.4 越权矩阵（发布门禁项）

独立测试目标 `tests/rls_matrix`，三块。按 C-05，八类断言函数 `assert_read`、`assert_write`、`assert_update`、`assert_delete`、`assert_aggregate`、`assert_sort`、`assert_report_projection`、`assert_error_leak` 的骨架与 CI 目标由阶段 1 提供在 `testkit/src/rls_matrix.rs`，本阶段以数据库侧策略为被测对象填充其用例，并追加 `assert_replication_role_containment` 与 `assert_recon_context_borrow` 两个函数；32 组完整矩阵 `matrix_32.rs` 与发布门禁项 `RG-RLS-MATRIX-GREEN` 由阶段 4 承担，该门禁的判据由 32 组全部通过改为 `platform_core.unpoliced_table_registry` 的登记行与本目标中承接入口用例逐行对应且全绿，`matrix_32.rs` 保留为其中一段而不再是计数依据。本阶段不实现上列之外的同名函数。

第一块，八类 × 2 法人共 16 组：读取、写入、更新、删除、聚合、排序、报表投影、错误信息泄漏。错误信息泄漏一项的判据是对不可见记录的读、写、删一律返回 404 与 `PLATFORM.AUTHZ.NOT_FOUND_OR_DENIED`，响应体中不出现对方法人的任何标识、计数或时间戳。

第二块，两个复制角色 5 项：无法读取任何业务表、无法执行任何 DDL、无法从该服务器之外建立连接、无法经界面借用、无法经 API 借用。后两项以 core-server 的路由表与依赖图断言实现，即不存在任何路径可构造出以这两个角色为身份的连接。该块由本阶段追加的 `assert_replication_role_containment` 承载，五项全部保留，因为它们是保留下来的角色权限、`pg_hba` 本机限制与凭据隔离三项控制的唯一验证方；遏制手段是否齐备不再登记任何降级窗口，也不作为两个写出进程投入运行的前置。

第三块，内部对账系统安全上下文 5 个入口：界面、API、报表、低代码、高级只读 SQL 均无法建立或借用。本阶段的实现方式是把该上下文的构造器设为 crate 内可见并只对 job-worker 装配路径开放，另加一条运行期断言：该上下文建立时校验调用栈来源标记，来源非内置对账任务即 panic 前先写审计再中止当前任务。该块由本阶段追加的 `assert_recon_context_borrow` 承载。本阶段不留签名语句集与其校验位点：封闭性改为静态判据，即该上下文不存在运行期 SQL 入口，执行器只按注册表分发已注册的 `ReconCheck` 实现、不接受语句文本入参，由 `xtask archcheck` 断言 `ep-platform-recon` 与各 `ReconCheck` 实现体内不出现字符串拼接 SQL 与动态语句执行入口；`ReconCheck` 实现本身按 A-06 由阶段 9a 提供。审计侧原先要记的语句集版本与签名摘要改记制品版本号与制品签名摘要，取自阶段 1 的制品元数据。

#### 8.5 部署级验收用例

本阶段无 UI，端到端改为部署级脚本化验收 6 项：DA-01 裸实例执行引导；DA-02 `ep-migrate apply` 全量迁移；DA-03 `ep-datagen --scale small` 装载 2 个法人，另以 `--scale t0` 装载 T0 最小样本的平台部分；DA-04 `ep-migrate check` 十三项断言全通过；DA-05 `rls_matrix` 全绿；DA-06 复制角色以 `pg_receivewal` 建立一次连接并创建复制槽，随后 `verify-connection-budget.sh` 输出与规格第 7.7 章的八进程枚举逐项一致。

#### 8.6 性能相关项

本阶段的取值是观察项，不是规格第 16 章的通过线，第 16 章判定在阶段 4。观察项五条：单字段加解密 P95 不高于 200 微秒；DEK 缓存在 20 并发下命中率不低于 99%；同一点查在策略开启与关闭两种情形下的耗时差不高于 5%；连接取用加会话变量设置的往返不高于 1 毫秒；空库全量迁移不高于 60 秒。超出即记入风险清单并在阶段 4 复测，不阻断本阶段退出。

另交付 `ep-explain-check` 并对本阶段自有的两条查询给出 `EXPLAIN` 证据：密钥域按法人列表、`data_keys` 取当前有效版本。两条均不得出现顺序扫描。附录 A.1 度量清单内查询的证据由各业务阶段提交，本阶段只提供采集工具。

#### 8.7 覆盖率门槛

本阶段全部代码属平台内核，行覆盖率不低于 85%；新增与修改代码不低于 80%；工作区整体不低于 80%。工具为 cargo-llvm-cov，CI 以 `--fail-under-lines` 强制，路径规则写入 `codecov.toml`。另加一条本阶段自定门槛：`ep-adapter-kms` 与 RLS 相关代码中，每个错误码至少有一个用例覆盖其返回路径，由测试名与错误码常量的映射脚本校验。`#[ignore]` 必须带 issue 编号且不得跨阶段存活。

---

### 9. 退出条件

全部可客观判定，逐条给出判定命令或产物。

E-01 `cargo build --workspace` 与 `cargo clippy --workspace --all-targets -- -D warnings` 通过；依赖方向自检脚本通过，含 portal-gateway 与 plugin-host 不依赖 `ep-adapter-db-pg` 这两条断言。
E-02 `db/bootstrap` 在裸实例上执行成功并可重复执行，第二次执行退出码 0 且无变更。
E-03 `ep-migrate apply` 在空库上成功，`ep-migrate status` 输出 `platform_core.schema_history` 的单一版本且与 `migration-versions.toml` 记录的期望版本一致。
E-04 `ep-migrate apply` 重复执行退出码 0 且无变更。
E-05 `ep-migrate check` 的 13 项编号合规断言全部返回 0 行，其中第 13 项以本阶段八张未受行级策略表的登记行为准，去掉任一行或另建一张未登记的未受策略表即返回非零行；`xtask sqlcheck` 执行的 `db/checks/append_only_consistency.sql` 同样返回 0 行。
E-06 `tests/rls_matrix` 三块共 26 组用例全绿，其中 `assert_replication_role_containment` 与 `assert_recon_context_borrow` 两个函数由本阶段实现，八类断言函数复用阶段 1 的骨架，本阶段不出现同名重复实现。
E-07 40 项集成测试全通过，测试结束后 `select datname from pg_database where datname like 'ep\_test\_%'` 返回 0 行。
E-08 覆盖率报告显示平台内核路径行覆盖不低于 85%，新增与修改代码不低于 80%。
E-09 2 个法人各自 provision 成功，每个域下 4 个 purpose 各有一把 `ACTIVE` DEK；对其中一个域执行一次轮换与一次销毁前核验，核验报告三项齐备。
E-10 `verify-connection-budget.sh` 输出与规格第 7.7 章的八进程枚举逐项一致，退出码 0。
E-11 第 7.1 节所列五项数据基座启动自检在 `--check` 模式下按注册名返回通过，其中四项标 `Blocking`、`secrets-resolvable` 的密钥域一段标 `Degrading` 并在缺域时以登记降级窗口而非退出码 78 收场；基线十项中的其余五项不由本阶段实现，其中已由阶段 1 交付的 `config-parsed` 与 `clock-skew-within-limit` 返回通过，`audit-chain-verifiable`、`file-store-writable` 与 `offsite-sink-requirements` 三项在其承担阶段交付前返回 `NOT_APPLICABLE` 并在报告中标注承担阶段；这五项在 portal-gateway、plugin-host、archive-writer 与 backup-writer 四个进程上同样返回 `NOT_APPLICABLE`；报告中不出现任何序号称呼。
E-12 `docs/data-dictionary.md` 含十三张表全部列条目与两处缩写标识符的全称，`docs/error-codes.md` 含本阶段新增的 20 个错误码且与 `ep-foundation::error::codes` 一致（CI 校验），`docs/event-catalog.md` 含 3 个事件，`docs/metrics-catalog.md` 含第 7.2 节四个指标条目，五篇 ADR 已提交。
E-13 第 12 节的偏离与新增决定已回写共享技术基线，评审记录存档。
E-14 代码审查与安全审查由独立角色完成，严重与高危发现全部关闭，符合规格第 17.1 章不得由同一执行角色自行批准的要求。
E-15 组织架构五张表建成并挂接策略与触发器，`LegalEntityDirectory` 与 `DepartmentClosureQuery` 两个 trait 已交付并可被阶段 3、阶段 4 与阶段 5 在 `wiring.rs` 中注入；IT-39 与 IT-40 通过。
E-16 `platform_ops.degradation_windows` 建成并带 `subject` 可空列与 `ux_degradation_windows_kind_scope_closed`、`ck_degradation_windows_open_order` 两条约束，其中前一条建在 `kind`、`subject`、`scope_legal_entity_id`、`scope_accounting_period_id` 与开窗状态五者上，`DegradationLedger` 的 `open`、`close`、`open_count` 三个方法可用，`DegradationKind` 的三个初始取值 `OFFSITE_SINK_NOT_CONFIGURED`、`WRITER_NOT_IN_SERVICE` 与 `PORT_NOT_IMPLEMENTED` 已定义且制品中不出现 `WRITER_ROLE_CONTAINMENT_MISSING` 一名，阶段 1 预留的 `// TODO(stage-2): write degradation ledger` 一行已补上。
E-17 `ep_foundation::port::db::MigrationWindowGuard` 端口与 `PgMigrationWindowGuard` 实现均已交付，`apps/core-server/src/wiring.rs` 与 `apps/job-worker/src/wiring.rs` 两处已注入，窗口关闭时 `assert_open` 返回 `PLATFORM.DB.MIGRATION_WINDOW_CLOSED`；`tools/ep-migrate` 的五个子命令与六个退出码与第 3.3 节逐项一致，阶段 1 的 `migrate`、`verify`、`manifest` 三个名字在本阶段制品中不存在。
E-18 `docs/metrics-catalog.md` 的唯一性校验通过，第 7.2 节四个指标的注册方与填充方与该文件一致，制品中不出现 `ep_db_retries_total`、`ep_tx_retry_total`、`ep_db_replication_crosscheck_age_seconds` 与 `ep_replication_crosscheck_age_seconds` 四个已作废的名字。
E-19 `ep_foundation::port::db::IdempotencyStore` 已按 C-07 定义并被内存实现覆盖，`platform_msg.idempotency_keys` 建表与重放判定不在本阶段交付物中，CI 断言本阶段无第二套判等实现。
E-20 本阶段全部路由的能力域码与动作类别常量已声明，常量位于 `crates/platform/tenancy/src/capability.rs`，`xtask configdoc` 通过。

---

### 10. 与规格和 PRD 的对应

规格条目逐条。

| 规格章节 | 本阶段实现的内容 |
|---|---|
| 7.1 事务数据 | 单实例单数据库；每模块独立 schema、数据库角色与迁移目录；时间以 UTC 存储、按中国标准时间展示；金额仅人民币 |
| 7.2 数据所有权与不变量 | 已过账分录、库存流水、审批证据、审计证据不可覆盖的数据库侧强制，即 `assert_append_only` 与不授予 DELETE |
| 7.3 数据库兼容 | 只交付并认证 PostgreSQL 16；抽象层与实现分离为 `ep_foundation::port::{tx, db}` 与 `ep-adapter-db-pg`，业务代码只依赖前者；认证套件中法人行级隔离与越权测试集本阶段交付并首次通过 |
| 7.4 可定制数据库 | 公共能力基线的字段类型与索引限制的断言（禁函数索引、部分索引、JSON 路径索引）；在线变更边界的迁移侧落实与 5 秒锁上限、30 分钟执行上限；`ext` schema 建立 |
| 7.7 法人行级隔离机制 | 安全上下文写入会话变量、策略以该变量为唯一判据、无 BYPASSRLS、连接归还前清除、按用途分账号、按用途分池、八进程连接枚举、两个复制角色的数据库侧遏制手段即无业务表权限与 `pg_hba` 只放行本机，第三项遏制的检出折叠进阶段 14 的复制槽保留量采样、本阶段不承载，内部对账系统安全上下文的封闭构造与越权测试 |
| 7.8 密钥域 | 每法人独立数据加密密钥域、密级子密钥、行内敏感字段信封加密、字段级密文不用于过滤排序聚合唯一约束、受治理盲索引、法人密钥销毁只影响本域、销毁证明三项 |
| 7.9 派生存储安全继承 | 只读角色仍受行级策略约束、只读角色不持有绕过前置查询服务的字段级密钥（`ep_analyst_ro` 不被授予任何 KMS 访问）、密级与数据范围标签作为公共列随事件携带的载体 |
| 12.2 授权 | 不设全能超级管理员的数据库侧落实；复制角色这一处已登记豁免的边界实现。内部对账系统安全上下文不再登记为第二处豁免，理由是它用同一个 `ep_app_rw`、受同一行级策略约束、不向任何主体返回业务行，与取整簇物理数据的复制角色不同量级 |
| 12.3 加密与密钥 | 存储信封加密、AES、SHA-256；密钥载体只有内置 KMS 与客户 HSM，云 KMS 不实现 |
| 12.4 DLP 与隐私 | 密钥销毁的前置核验与销毁证明覆盖范围与残留声明 |
| 12.5 审计 | 审计事件与业务变更同事务的接缝，即共享 `Tx`；`platform_audit` schema 建立与授权 |
| 13.1 正式拓扑 | 单实例、同机、数据库连接数不在配额节重复取值而按 7.7 分池上限执行 |
| 13.4 备份 | 复制角色、复制槽参数 `max_slot_wal_keep_size` 取 350 GB、`max_wal_senders` 4、`max_replication_slots` 3 的数据库侧配置 |
| 15.1 错误分类 | 五类分类的数据库侧映射与本阶段新增的 20 个错误码，另有四个按 C-24 由阶段 1 登记、本阶段首次实现，每条含关联编号、发生时间、可否重试与处理建议 |
| 17.2 自动化测试 | 数据库适配认证中的法人行级隔离与越权测试集；身份与访问控制测试中数据库账号边界部分；数据保护控制与销毁证明测试中密钥域销毁核验部分；派生存储越权测试中只读角色部分 |
| 17.3 强制不变量 | 权限不能跨法人、字段或密级越权一项由本阶段承担并通过；已过账凭证不可覆盖的机制由本阶段交付、其被测对象在阶段 3；其余各项本阶段只提供数值与时区前提 |
| 附录 A.3 | `ep-datagen` 的规模参数框架与 2 个法人、36 个会计期间跨度的维度预留 |

PRD 条目逐条。

| PRD 节 | 本阶段实现的内容 |
|---|---|
| 10.2 权限模型与配置 | 法人维度的数据库侧强制；求值顺序中第一步法人授权的落地判据 |
| 11.2 并发与规模上限 | 2 个法人、50 名命名用户的数据基座承载；连接预算不因超出基线而阻断写入，只影响时延通过线的适用性 |
| 11.3 响应时延与等待反馈 | 同步等待上限 8 秒对应连接取用超时 `ACQUIRE_TIMEOUT_MS` 默认 8000 |
| 11.9 降级状态的用户可见性 | 数据基座自检失败项的结构化结论输出，供运维中心台账取用 |
| 附录乙 U-A-03 | 文本长度上限以 `text` 加 CHECK 表达，取值按基线第 11.2 节；本阶段被该项的业务侧决策阻塞程度为零，因为改 CHECK 属在线变更范围 |
| 附录乙 U-A-04 | 数量、单价、金额、税率的小数位数与舍入规则的两侧一致实现；业务侧若另有决策，改动范围限于列类型与 `Money` 的 scale 常量，切换代价为一次停机窗口内的列类型变更 |
| 附录乙 U-A-12 与 6.16 的 F-17 | 开户银行是否纳入敏感字段清单尚待决策，银行账号按规格第 7.8 章强制纳入并做字段级加密；本阶段以 `sensitive_field_registry` 承载该决策，不预置任何行；被阻塞程度为零，因为登记是数据行变更而非代码变更。按 A-28，阶段 5 以 `db/migrations/mdm/` 下的 backfill 迁移插入 `mdm.customer_invoice_profiles` 与 `mdm.supplier_payment_profiles` 两表的 `bank_name` 与 `bank_account_no` 共四行，`bank_account_no` 两行的 `is_field_encrypted` 取真、`bank_name` 两行取假，后者与四行的 `security_level` 与 `mask_style` 同为该事项决策前的临时取值；按 A-27，本阶段不接入配置发布通道 |
| 附录乙 U-B-05 | 权限求值顺序中显式拒绝优先的结论，本阶段只落实第一步法人授权，其余四步属授权阶段 |

---

### 11. 风险与预留

R-01 refinery 0.8 对非事务迁移的支持不确定。若其无法按文件粒度关闭事务，则 `CREATE INDEX CONCURRENTLY` 必须走自建执行器。缓解是自建执行器已列入 D-01 的必交付范围而不是备选，两条路径共用同一历史表结构；残余风险是两套执行器的校验和算法必须严格一致，由 IT-06 断言。

R-02 `numeric(18,2)` 在插入更高精度时静默舍入。PostgreSQL 的舍入方向恰与本平台的中值远离零一致，因此不会产生方向性错误，但会掩盖代码路径上遗漏 round 的缺陷。缓解是 Rust 侧写前断言并返回 `WRITE_SCALE_VIOLATION`，由 IT-27 覆盖。残余风险是绕过统一数据访问层的直写路径，该路径本身已由角色权限与 CI 断言封堵。

R-03 法人枚举与行级策略的张力。内部对账、启动自检与密钥域巡检都需要枚举全部法人，而策略以单一法人为判据。本阶段的解法是 `legal_entities` 不建策略并按第 3.5 节表十三登记，准入判据取隔离机制自身的元数据，代价是任一持有运行期账号的会话都能读到 2 个法人的名称与编码。该残余风险登记在第 12 节，并在越权测试集中以明确用例固化其边界，即除该表的清单元数据外不得读到对方法人的任何业务数据。备选解法是 `SECURITY DEFINER` 函数，本阶段不采用，理由是它等价于引入一个受限的 BYPASSRLS 路径，与规格第 7.7 章不设绕过的口径冲突。

R-04 内置 KMS 主密钥的保护只到操作系统层。持有该服务器操作系统权限者可读取主密钥并解开全部法人密钥域。该结论与规格第 21.18 章对同类主体的口径一致，写入交付说明，不得表述为等效于硬件密码机。缓解是文件权限 0400、属主校验、启动自检拒绝不合规权限，以及主密钥使用记入审计。

R-05 盲索引的信息泄漏。等值盲索引会泄漏取值的相等关系与频次分布，持有数据库读权限者可据此做频次分析。该性质是可检索与保密的固有取舍，规格第 7.8 章已要求盲索引受治理，本阶段的治理手段是逐列登记与批准；残余风险写入 ADR 并在敏感字段清单的批准流程中向产品负责人明示。

R-06 触发器带来的写入开销。每张可更新业务表挂 `assert_row_version_bump`，每条 UPDATE 多一次 PL/pgSQL 调用。在 20 并发下预估影响低于 3%，由本阶段的观察项实测；若阶段 4 的容量测试显示该开销挤压普通交易提交 3 秒通过线，退路是把该触发器降级为仅在测试与预发布环境启用，生产改由 CI 静态检查与集成测试保证。该退路写入 ADR，不在本阶段执行。

R-07 复制槽的 `max_slot_wal_keep_size` 取值依赖附录 A.4 实测的事务日志生成速率，本阶段只能按附录 A.3 的 350 GB 上限取值。若阶段 4 实测速率使该取值不足以支撑部署记录约定的落点不可写时长，需按附录 A.3 同一构成上调该子项并重算容量下限。本阶段把该参数做成引导脚本的单一变量，便于一处修改。

R-08 幂等键的三段职责按 C-07 已经拆定，本阶段只定义端口，请求头校验属阶段 1，表与重放判定属阶段 3a，因此本阶段不再登记为被阻塞。残余风险是判等口径必须只在阶段 3a 一处实现，本阶段与阶段 1 都不得自行判等，由第 6 节的端口签名与 CI 依赖图断言约束；阶段 3a 交付后重跑本阶段四个写端点的用例。

为后续阶段预留的扩展点，逐项给出位置。一是 `attach_table_guards` 与 `assert_baseline_indexes` 两个函数，各业务阶段建表时调用一次即自动获得策略、触发器与索引断言，无需重复实现。二是 `sensitive_field_registry`、`append_only_registry` 与 `unpoliced_table_registry` 三张登记表，新增受保护列、仅追加表或不带法人列的表只是插入一行，不改代码；按 A-27 本阶段不接入配置发布通道，登记经迁移或端点直接写入，发布通道接入由阶段 3b 反向补齐，登记行分别由阶段 5（A-28）与阶段 3b、7、8、9a、10（B-02）插入，未受策略表的登记行由建表阶段随建表迁移插入。三是 `KmsBackend` trait 的 `hsm` 实现位点，客户提供硬件密码机时只切换配置；`derive_blind_key` 是盲索引取值的唯一计算入口，阶段 5 与阶段 10 按 B-04 直接取用。四是 `UnitOfWork::snapshot_transact`，供阶段 9a 的 `ep-platform-recon` 直接承接。五是 `db/migrations/<schema>/concurrent/` 目录，供后续阶段在有数据的表上加索引，其在线 DDL 执行段按 B-03 须先调用 `MigrationWindowGuard::assert_open`。六是 `ep-datagen` 的模块生成器注册点，T0 最小样本的业务部分由阶段 5 至阶段 11 在此追加。七是 `CipherEnvelope` 的算法标识字段预留 `0x02` 起的取值，供后续版本恢复商用密码档位时扩展，本阶段不实现也不验收。八是 `key_domains.domain_kind` 的 `GROUP_SHARED` 取值已在类型中存在但 CHECK 不放行，供后续版本恢复跨法人能力时放开。九是 `LegalEntityDirectory` 与 `DepartmentClosureQuery` 两个 trait 与 `DegradationLedger` 端口，阶段 3 至阶段 14 直接注入取用，不自建同义接口，其中 `DegradationKind` 的 `PORT_NOT_IMPLEMENTED` 是跨模块与平台能力缺位的唯一登记形态，开窗时由 `subject` 列记下该端口或平台能力的完整类型名。

---

### 12. 偏离基线、本阶段新增决定与显式假设

按基线第 0 节与第 12 节的纪律，逐条单列。

偏离项两条。

第一条，基线第 3.8 节把不带 `legal_entity_id` 的表列为封闭的四类，其中的全局配置字典一类无定义、容量无限，已被多个阶段各自归类，事实上不封闭。本阶段删掉该封闭枚举与全局配置字典这一类名，改为一条正向规则加一张登记表：凡带 `legal_entity_id` 的表一律按模板建策略；不带该列的表必须逐表登记 `platform_core.unpoliced_table_registry` 一行，给出可机械核对的准入判据与法人可见性的应用层承接入口，未登记的表由 `db/checks/13` 判为违规而建不出来。本阶段据此登记八行，其中 `platform_core.legal_entities` 的准入判据是隔离机制自身的元数据，把它纳入策略会使枚举法人这一前置操作无法进行；影响范围是该表可被任一运行期会话读到 2 个法人的标识与名称，其记录级与字段级裁剪由租户与身份阶段在应用层按用户授权法人集合执行。回写基线第 3.8 节。

第二条，基线第 3.3 节禁止跨 schema 外键。本阶段删掉该禁令，改为第 3.8 节的复合外键规则。理由是原禁令的两条依据都不成立：外键是被引用表上的声明式约束，不构成任何跨模块的读或写，模块隔离由仓储按 schema 分文件与依赖方向断言承载；模块停用按规格第 5.6 章只停界面入口、写入接口、定时任务与对外事件，全程无 DDL，外键不参与其中任何一步；迁移顺序已由文件版本号全序排定，外键只是把这条已有顺序变成机器可验证的。收益是十五个模块之间的引用完整性从应用层纪律变成写入瞬间的数据库强制，能同时兜住迁移回填、期初导入、`ep-datagen` 与修数脚本这类绕过应用的写入路径，并顺带删掉各阶段写入前校验两侧 `legal_entity_id` 相等的散落实现。回写基线第 3.3 节。

本阶段新增决定六条，均需回写基线。

一、金额、数量、单价、比率四类列的列名后缀固定为 `_amount`、`_qty`、`_unit_price`、`_rate`，理由是没有命名约定就无法机械断言基线第 3.5 节的精度取值，人工核对在 24 个 schema 上不可持续。回写基线第 3.2 节。

二、标识符超过 63 字节时按列序缩写并在数据字典登记全称，理由是 PostgreSQL 的硬性限制会静默截断从而产生两个同名对象。回写基线第 3.10 节。

三、迁移执行器为独立 CLI `tools/ep-migrate` 而不是八进程之一，理由是它不常驻、无 systemd 单元、无 cgroup slice、只在迁移窗口内以 `ep_migrator` 运行，因此不构成基线第 12 节所禁止的新增进程。回写基线第 1.1 节的目录布局。

四、密文自带信封头且以 `legal_entity_id`、`schema.table.column`、行标识三段作为 AAD，理由是把密文与其所在行绑定，使数据库层的整列复制无法产生可解密结果。回写基线第 7.2 节。

五、`ep_app_rw` 在业务 schema 上不授予 DELETE，只在 `platform_msg.idempotency_keys` 与 `platform_ops` 过期快照上授予，理由是把基线第 3.6 节的禁止从 CI 静态检查提升为数据库权限。该决定按 C-01 由阶段 1 第 13 节移交本阶段，本阶段是其唯一登记方，阶段 1 计划不再重复登记。回写基线第 3.1 节的角色权限边界列。

六、`platform_core` 承载密钥域、数据密钥、敏感字段清单、仅追加登记、未受行级策略表登记与迁移窗口六类平台元数据表，以及按 A-04 归入的集团、组织、部门、岗位与部门层级闭包五张组织架构表，理由是基线的八个平台 schema 中没有为密钥管理与租户组织单列 schema，而新增 schema 被基线第 12 节禁止。回写基线第 3.1 节的 schema 用途说明。

显式假设四条，规格与 PRD 均未定义。

假设一，内置 KMS 主密钥以 32 字节随机内容存放于 `/var/lib/ep/kms/master.key`，权限 0400，属主为使用它的进程系统账户，其自身不再二次加密，保护依赖操作系统层访问控制。理由是单机形态下不存在第二个可托管该密钥的可信部件，规格第 12.3 章的两种载体中内置 KMS 本身即是最终信任根。该假设写入交付说明。

假设二，DEK 按 `(密钥域, 用途, 密级子域)` 三元组划分，四个用途取 `FIELD`、`BLIND_INDEX`、`ATTACHMENT`、`ARCHIVE`。规格第 7.8 章只说密级、附件与归档可在法人密钥域内使用子密钥，未给划分维度。理由是这四类的轮换与销毁节奏不同，合并会使任一类的轮换牵动全部。

假设三，盲索引截断为 16 字节且默认不建唯一约束。规格第 7.8 章只要求受治理的盲索引，未给长度与唯一性口径。理由见第 4.4 节。按 B-04，`finance.cash_accounts` 的银行账号盲索引列 `bank_account_no_bidx` 需要唯一约束，走本假设已给出的例外路径，即取完整 32 字节并在数据字典登记；`mdm.customer_invoice_profiles` 与 `mdm.supplier_payment_profiles` 两表的同名列不建唯一约束，按本假设默认取 16 字节截断，依据是 PRD 第 6.2.2 节只对资金账户要求银行账号在同一法人内不重复，客户开票要素与供应商收付款信息无该要求。两处都不构成对本假设的推翻。

假设四，迁移窗口的默认存活时长为 60 分钟、上限 240 分钟。规格第 7.7 章只说迁移账号在迁移窗口内启用，未给窗口时长。取值依据是基线第 3.9 节的迁移执行上限 30 分钟加一倍余量，上限对齐规格第 12.1 章应急账号 8 小时的一半以示更严。

被业务决策阻塞的判定：本阶段无被阻塞项。U-A-03、U-A-04、U-A-12 与 F-17 四项虽与本阶段相关，但其载体分别是 CHECK 约束、列类型与登记表，三者的变更都在在线变更或登记行变更范围内，切换代价分别为一次在线迁移、一次停机窗口内的列类型变更与一次登记行变更；U-A-12 若决策为开户银行也做字段级加密，按 A-28 另需在同一次变更内改物理列并删去同名明文列，该代价落在阶段 5 而不落在本阶段。四项均不构成本阶段的开工前提。幂等键一项按 C-07 是三段分工而非阻塞，见第 6 节与 R-08。
