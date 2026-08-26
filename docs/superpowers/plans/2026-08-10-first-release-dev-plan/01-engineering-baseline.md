> **F-57 状态：`HISTORICAL_DO_NOT_EXECUTE`。** 可复用测试思想须经 F-57 Task 1 重置旧架构冻结、Windows 目标和登记表后再采用。

## 阶段 1 工程基座与 CI

> **F-50 增量。** `xtask archcheck` 的凭证写入边界最终只允许 ledger 内部 `post`、`post_reversal`、`post_correction` 三个受控入口；业务 crate、HTTP、Excel 与插件不得直接 INSERT/UPDATE 凭证表或构造更正分录。该规则与负样例按 F-50 实施计划交付，本阶段不实现财务业务。

> **F-55 后续扩展。** 本阶段交付并验收的是最初八个产品进程的工程空壳；F-55 在其后同批新增第九个 `ai-inferer`，并同步扩展制品、服务账户、资源单位、启动/健康、连接预算与发布验收清单。本文其余“八进程”措辞只描述阶段 1 当时的切片，不是终态；终态恰为九个产品常驻进程，精确增量以 `13c-local-ai-mcp-server-admin.md` 及三份 F-55 子计划为准。

本阶段是全部后续阶段的地基。它不实现任何业务规则，不建任何业务表，不产生任何会计分录。凡涉及账务的内容一律指向规格第 5.2 章事件-分录表，本阶段不复述借贷与取价，也不预先实现其中任何一条规则。本阶段的判定标准只有一条：把共享技术基线里已经定死的每一条约定，变成可以在流水线上自动检出违反的机器判定，并交付一套可运行、可停机、可重启、可验签的空壳部署。

### 1. 本阶段的范围边界

在范围内的：Cargo workspace 与全部 crate 骨架、八个进程的空壳二进制、进程运行时装配、配置模型与启动自检框架、统一封套与错误映射、集群引导脚本的目录与执行顺序约定、迁移目录骨架、迁移静态检查、`tools/ep-migrate` CLI 骨架与退出码约定、`tools/bench` 与 `tools/release-gate` 两个非产品工具骨架及其未交付退出码 70、ep-foundation 的跨阶段冻结类型与常量、连接池与会话变量注入清除、测试分层与覆盖率门禁、结构门禁与依赖方向门禁、Windows 服务注册与服务宿主、具名 Job Object 静态限额、DACL、命名管道、供应链门禁与可复现构建、制品与版本号、本地开发环境。

明确不在范围内的：集群引导五个脚本的内容、24 个 schema 与七个功能角色与 24 个属主角色、逐 schema 的默认权限、单一全局迁移 Runner 与其版本号断言、`ep-migrate` 五个子命令的实现，这五项按 C-01 与 C-02 归阶段 2；另有移动薄 PoC 的实际测量与任何客户端代码，RLS 业务表，身份认证与授权判定，Outbox 消费与审计链，KMS 与信封加密，附件正文读写，电子签章对接，任何模块的领域模型。阶段 1 只冻结 PoC 门槛表；客户端目录、夹具与薄 PoC 由阶段 13a 在本阶段结束后立即开工，业务移动界面大规模投入前完成。阈值失败只替换移动 UI 为 Flutter，客户端 Rust 核心九个 crate 与服务端 Rust 核心不变。

Windows/CI 最终冻结：产品服务、数据库与客户主数据卷只走 Windows Server 原生形态，不进入 Hyper-V 客户机；F-55 唯一允许的 Hyper-V 窄例外是可选、逐次 MCP 插件调用使用的短命 Hyper-V-isolated Windows utility VM，它不构成第二套服务端部署。配置安全承载为 NTFS DACL、进程间通信为命名管道、配置变更只在启动或下次取用时生效。默认 CI 为 Forgejo 加 Woodpecker Windows agent，全部门禁只由 `cargo xtask ci` 聚合。生产制品必须 Authenticode，开发内部制品可用 ECDSA P-256，证书可由软件厂商或客户提供。裁定 F-08 保留 18 个历史编号，原编号 12 已撤销；其余 17 项有效测试是首批实施证据门禁，不是设计待决，本文没有声称其已经执行或通过。
本阶段与 T0 贯通线的关系。按总览通则第四条的固定链 1 → 2 → 3a → 4 → 3b-1 → T0 → 5 → 9a → 8 → 6 → 7 → 10 → 11 → 9b → 14 共十五个环节，T0 是其中第六环，在阶段 3b-1 结束之后、阶段 5 全量开工之前执行，是一条最薄贯通线，判据是一条合同从建单走到管理层看到一个数，其业务切片取自阶段 5、6、9a、10、11。本阶段不贡献任何业务切片，只交付 T0 赖以判定的三样手段：`xtask e2e --profile=t0` 这一条独立的端到端目标、`ep-datagen` 的 `t0-min` 最小样本档、以及 `deploy/` 下一条命令起全栈的单机编排；这三样手段属本阶段本轮既有交付物，按总览第 2 节 T0 行的体量口径不计入 T0 的当量，也不从本阶段扣除。T0 只要求桌面端可达，不要求 scale 数据集，不要求分支覆盖，不要求四端。本阶段其余交付物不因 T0 增删一项，阶段 5 至 11 一律改为在 T0 贯通后的骨架上加厚，M7 保留为全分支闭环而不再是首次贯通。


### 2. 交付物清单

本阶段结束时，下列东西必须存在且可运行，逐项可由他人在一台干净的 Windows Server 2022 机器上复现。目标版本区间为 Windows Server 2019 至 2022，认证取值冻结在 2022；按裁定 F-08 第一节结论二，在 2019 上只做一次同项复核，其数据不写入认证报告，也不得据以声明 2019 已认证。

| 编号 | 交付物 | 可运行的判定方式 |
|---|---|---|
| D-01 | 单一 Cargo workspace，含全部 crate 骨架，`cargo build --workspace --locked --offline` 成功 | 构建返回 0，无 warning |
| D-02 | 八个空壳进程二进制，各自可启动、可健康、可优雅停机 | `--check` 返回 0，健康端点返回 200；停机按两条路径分别判，不合并成一条。显式停止路径（`sc stop`）：服务宿主持续抬 `dwCheckPoint`，服务控制管理器不强杀，30 秒内排空并以退出码 0 退出，该路径成立且可判。机器关机路径：等待受全机注册表值 `WaitToKillServiceTimeout` 约束，该值远小于 30 秒，拉长它须改客户机器的系统设置，按裁定 F-08 第零节授权边界第 2 条一律判为做不到，故 30 秒排空在该路径不成立，如实登记为交付说明中的一条降级，不得以「一般不会在关机时有在途请求」把它写掉；该值在本区间两版的默认值及其预算作用域只作为第十二节实机证据项，证据状态为 `UNVERIFIED`，只量化风险、不改变本条实现与定性。退出码的判定按该裁定第 4.2 节由夹具补强：测试进程在发停止命令前先 `OpenProcess` 持住句柄，停机后 `GetExitCodeProcess` 断言真实退出码，并与 `sc query` 的 `WIN32_EXIT_CODE` 自报值双向比对 |
| D-03 | `tools/ep-migrate` CLI 骨架与退出码约定，五个子命令为 apply、status、check、gen-rls、open-window，子命令实现由阶段 2 交付；F-56 只给既有 apply 增加三个必须成组出现的 fresh-production initial-governance 参数，不构成第六个子命令 | 五个子命令的参数解析可运行，退出码 0 成功、2 参数错误、3 迁移窗口未打开、4 校验和不符、5 版本不一致、78 环境自检失败各有一个用例；三参数的签名、自举与 receipt 语义只取 F-56，不在本阶段造占位实现 |
| D-04 | 集群引导脚本的目录约定与执行顺序约定，文件名为 `db/bootstrap/00_database.sql`、`01_roles.sql`、`02_cluster_params.sql`、`03_role_defaults.sql`、`04_pg_hba.fragment`，脚本内容由阶段 2 交付 | 目录与文件名约定被 `xtask sqlcheck` 断言，自检项 rls-enabled-and-forced 与 runtime-role-privileges-bounded 的代码路径以测试库探针表为被测对象通过 |
| D-05 | 单机编排骨架，`deploy/` 下的 Windows 服务注册脚本与八个资源单位的静态限额文件，随同一份安装包（MSI 或压缩包）交付；两套等价编排文件这一交付物形态在本平台不存在——只剩一套载体，被比对的第二方消失——故等价性核对这条判据按裁定 F-08 做不到八撤下，`deploy/podman/` 与 `deploy/compose/` 下的编排文件、`scripts/verify-orchestration-equivalence.py` 与其负样例脚本一并失去对象，不得造一个只有一侧的「等价性」脚本。附带损失一条如实登记：Compose 一侧的 `depends_on` 加 `condition: service_healthy` 是两套里唯一带就绪门槛的一支，`sc config depend=` 只表达被依赖服务进入 RUNNING、不表达就绪，本次退化掉的正是较强的那一支，且为永久状态 | 一条命令起全栈，`sc query` 九个服务全部 RUNNING |
| D-06 | `deploy/` 下八个具名 Job Object 静态限额文件与一次性部署校验工具 `scripts/verify-resource-limits.ps1` | 只使用 Windows API、DACL 与 Job Object 读回值，不读取 Linux/cgroup 路径；F-08 对应实机证据未生成前第 11 阶段保持非零，不得宣称通过；任何进程的启动自检中不出现资源限额项 |
| D-07 | 一条绿色 CI 流水线，共 11 个阶段，全部门禁可离线执行 | `cargo xtask ci` 是唯一入口与真值；默认 Forgejo 加 Woodpecker Windows agent 只作薄适配；全量不超过 60 分钟且十一个阶段全部真实返回 0，未交付、未覆盖或未实测均保持非零 |
| D-08 | 结构门禁工具 `xtask`，含 archcheck、sqlcheck、codecheck、errorcodes、eventcatalog、configdoc、coverage、sbom、sign、reproduce、e2e 十一个门禁子命令与唯一聚合入口 `ci` | 每条规则有一个故意违反的负样例，负样例必须失败；CI 平台不得绕过聚合入口直接编排子命令 |
| D-09 | `ep-testkit` 测试夹具库、`ep-datagen` 数据集生成器骨架与其 `t0-min` 最小样本档、`xtask e2e --profile=t0` 目标 | 同一 seed 两次生成结果字节一致；`t0-min` 生成一个法人一个客户一个产品的最小样本；`--profile=t0` 作为独立目标可执行，本阶段用例集为空并返回 0 |
| D-10 | 覆盖率门禁，按路径分档强制 | 低于门槛即失败，有负样例证明 |
| D-11 | 制品与升级包，含八个进程与 `ep-migrate` 的 PE 二进制、同一份 MSI 或压缩包、服务注册脚本、SBOM、签名、校验清单与回退说明，同一制品覆盖 Windows Server 2019 至 2022 | 客户侧 `verify-release.ps1` 在断网 Windows 机器上验签：生产制品必须 Authenticode，证书可由厂商或客户提供；内部开发制品可用 ECDSA P-256，但须标开发签名且发布门禁拒绝放行 |
| D-12 | 可复现构建证据 | 两次独立构建的八个 PE 二进制 SHA-256 全部相同；由 `cargo xtask ci` 第 8 阶段在 Windows agent 判定，实机证据生成前保持非零并标记未验证，不把文档冻结写成通过 |
| D-13 | 本地开发环境，一条命令起本机 PostgreSQL 16 与全栈；服务宿主以控制台模式运行 | `scripts/dev-up.ps1` 启动并等待就绪，`scripts/dev-down.ps1` 排空后停止；新机器从零到跑通集成测试不超过 30 分钟 |
| D-14 | 文档骨架，含 ADR 目录、错误码表、事件目录、指标目录、配置参考、数据字典，其中数据字典含单据类型码一节 | 六份文件存在且被 CI 校验与代码一致，其中单据类型码一节本阶段只判该节存在，`xtask configdoc --check-doc-type-codes` 与 `ep-platform-sequence` 常量表的逐项一致且无重复比对按第 10 节退出条件 23 推迟到阶段 3a |
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
| ep-adapter-db-pg | 实现 | 同上，且只在各进程的 `apps/<proc>/src/wiring/` 目录下出现 |
| ep-adapter-file、kms、queue、search、doc、esign、wasm | 骨架 | 不装配 |
| ep-adapter-ipc | 实现 | 八个产品进程均装配：core/integ/plugin 创建各自服务端，portal/worker/archive/backup/ops 及兼任客户端的 core 按逐账户 DACL 连接；阶段 1 只实现三条管道的健康/指标操作与未知操作失败关闭，后续阶段在同一适配器内补已冻结的具名业务 operation，不新增传输实现 |
| ep-testkit | 实现 | 仅 dev-dependencies |
| ep-datagen | 实现骨架 | 独立二进制，不属于八进程 |
| ep-bench、ep-release-gate | 非产品工具骨架 | 位于 `tools/bench/` 与 `tools/release-gate/`，是 workspace 成员但不装配进八进程、不进入产品制品或产品 SBOM；阶段 14 交付真实功能前调用一律返回 `EXIT_NOT_DELIVERED=70`，不得返回 0 |
骨架 crate 中有三处落点在本阶段就写死，后续阶段只补内容不改位置。第一处，`ep-foundation` 下的 `src/port/search.rs`、`src/port/doc.rs`、`src/port/db.rs` 与 `src/port/kms.rs` 四个文件本阶段只建空文件并写模块注释，检索端口的类型与 trait 按 A-07 由阶段 3b 补齐，文档与打印端口按 A-08 由阶段 5 补齐，`port::db` 的 `IdempotencyStore` 与 `MigrationWindowGuard` 按 C-07 与 B-03 由阶段 2 补齐、只读事务端口 `ReadOnlyTx` 由阶段 11 补齐；`port::kms` 由阶段 2 按裁定 F-04/F-56 一次补齐冻结六方法 `KmsBackend`、`KmsSigningKeyIdentityResolver`、`KmsKeyMaterialProvisioner`、`KmsPinnedDataKeyBackend` 及阶段 2 第 4.1 节的 strong values，两种载体的实现落 `ep-adapter-kms`，不得给六方法 ABI增第七项。第二处，`PgUnitOfWork` 与 `PgTx` 两个实现类型的声明与实现同在 `ep-adapter-db-pg`，工作区内不存在 ep-adapter-db。第三处，跨 crate 取具体事务句柄的唯一写法是 `tx.as_any_mut().downcast_mut::<PgTx>()`，`xtask archcheck` 在 `crates/`、`apps/`、`testkit/`、`datagen/`、`tools/` 五个目录上扫描，断言其中 `crates/adapter/db-pg/` 之外的任何文件都不出现 `downcast_mut::<PgTx>`；`xtask/` 自身与仓库顶层文件因承载该规则的检索式常量而不在扫描面内。

本阶段另交付一条 archcheck 规则，规则名 `unwired-absent`，承接总览通则第三条的空实现形态整体撤销。断言对象是 `apps/core-server/src/wiring/` 与 `apps/job-worker/src/wiring/` 两个目录下的全部文件，不是单个 `wiring.rs` 文件；判据是这些文件中不出现任何以 Noop、Stub、Fake、Dummy 为前缀的实现类型或注入行，出现即构建失败。前缀集合就是这四类，不设第五类。该规则随 `xtask` 在本阶段交付，并配一个故意违反的负样例，负样例构建必须失败；它在第 10 节单列一条退出条件，不并入退出条件 3 的依赖方向七条禁止项，其负样例也不计入那七条的负样例集。规格把交付时点冻结在末期的 WasmComputePort、RuleEvaluator 与 DisposalPort 三项平台能力不在此开例外，三者在其交付阶段之前本就不出现任何注入行，能力缺位改由降级窗口承载。阶段 14 的发布门禁项 `RG-UNWIRED-ABSENT` 的扫描面与本规则相同，其判据提供方即本规则。

#### 3.2 本阶段八个进程各自的空壳内容

| 进程 | 本阶段实现的内容 | 本阶段不实现的内容 |
|---|---|---|
| core-server | 8080 HTTP 服务器与五个系统端点、`\\.\pipe\ep-core` 命名管道 IPC 服务端、rw 与 ro 两个池、并发闸门、同步等待上限、六项自检、优雅停机 | 任何业务路由、鉴权判定、幂等存储 |
| job-worker | 8081 健康与指标、任务调度器骨架与零个已注册任务、worker 池、200 毫秒到 2 秒的退避轮询空转 | Outbox 消费、通知投递、对账 |
| portal-gateway | 8090 HTTP 只作第三方反向代理 upstream、不建数据库连接、以 `NT SERVICE\ep-portal` 经 `\\.\pipe\ep-core` 执行上游探测与后续五项受控能力、新建 trace 与 X-Correlation-Id | 门户业务页面、会话、脱敏投影 |
| integration-gateway | 无 TCP 监听；`\\.\pipe\ep-integ` 的健康/指标与业务 IPC server、出网客户端骨架含超时退避熔断、出网白名单校验；数据库/KMS/业务文件目录/Outbox 权限与连接均为零 | 电子签章协议与证据固化、移动推送协议适配、CUSTOMER_ICAP 客户端；分别由阶段 6、3b-2、3b-2 交付，业务效果由 worker/core 落库 |
| plugin-host | `\\.\pipe\ep-plugin` 命名管道 IPC 服务端、零数据库连接 | WASM 宿主；`wasmtime` 与 `wasmtime-wasi` 两个依赖本阶段一律不登记，也不留默认关闭的 feature 与编译缓存目录约定，由阶段 13b 在交付宿主时一次引入 |
| ops-agent | 9101 Prometheus 文本、9102 健康聚合、ep_ops_ro 池 2、按回环抓取其余七个进程的指标端点 | 运维台账读取、降级窗口 |
| archive-writer | 无监听、spool 目录、IPC 客户端、15 分钟周期心跳占位、core-server 不可用时落 spool 并在恢复后补写 | 事务日志归档、附件写出、审计证据写出 |
| backup-writer | 无监听、spool 目录、IPC 客户端、每日周期心跳占位 | 全量备份、校验、存量搬运 |

**九个**二进制 crate 名与进程名、Windows 服务名一一对应，由 `xtask codecheck` 断言（本句原写「八个」，F-55 已把技术基线第 2 节进程表更新为九进程并新增 `ai-inferer`，此处同批更正）；与资源单位不构成一一对应——core-server 与 integration-gateway 同处一个资源单位，八个二进制落在七个资源单位内，该维判据按裁定 F-08 第八节随 `codecheck` 重写时改为断言这一多对一关系。archive-writer 与 backup-writer 在本阶段就不持有运行期应用账号，其配置结构体中根本不存在 db 段，配置里出现 db 段即启动失败，这是把规格第 7.7 章的账号边界前移到类型层。

具名 Job Object 的名称在本阶段一次冻结，不留配置分支：`Global\EP_<deployment UUID去连字符并转32位大写十六进制>_<suffix>`，suffix 封闭为 `APP_CORE|APP_WORKER|APP_PORTAL|APP_PLUGIN|APP_EDGE|APP_ARCHIVE|APP_BACKUP|APP_DB`。core/integration 共 APP_CORE，job-worker、portal、plugin、archive、backup、PostgreSQL 分别取对应 suffix，ops-agent 与第三方反向代理共 APP_EDGE。非规范/全零 deployment_id、推导名与 `deploy/resource-limits.toml` 不一致或同机异安装复用同 deployment_id 均在启动前失败；名称不设配置键。首版该文件只承载内存硬上限；CPU 比例/突发上限与磁盘 IO 份额不写值、不自动启用，未来须新版本正式裁定。

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

表名 `platform_core.schema_history`，全库只有一张，由阶段 2 的 `ep-migrate` 自建 Runner 首次执行 `apply` 时创建，结构如下，任何阶段不改其结构。本阶段只做两件事：在 `ep-migrate` CLI 骨架中固定其 schema 与表名参数，以及把该表列入 `xtask sqlcheck` 的白名单。Runner 不依赖 refinery，原因与兼容边界见 ADR-0013。

| 列 | 类型 | 约束 |
|---|---|---|
| version | bigint | 主键，容纳 14 位时间戳版本号 |
| name | varchar(255) | 非空 |
| applied_on | varchar(255) | 存 RFC3339 字符串 |
| checksum | varchar(255) | 非空 |

这四列由工具定义，不套用基线第 4 节的公共列，属工具自带元数据表，在 `xtask sqlcheck` 中列入白名单，白名单只有这一项。

#### 4.4 测试专用探针表，不进生产迁移目录

为了在本阶段就把基线第 3.8 节的 RLS 模板、第 4 节的公共列、第 3.7 节的乐观锁与第 3.10 节的索引命名全部跑通，`ep-testkit` 在每个临时测试库中创建 schema `ci_probe` 与下表。它不出现在 `db/migrations/` 下，不进任何交付制品，`xtask sqlcheck` 规则 SQL-030 断言 `ci_probe` 字样不出现在生产迁移目录中。
按 B-01，探针 schema 与探针表的建表函数一律带 `#[cfg(feature = "ci-probe")]`，Cargo feature 名固定为 `ci-probe`，在 `apps/core-server/Cargo.toml` 与 `testkit/Cargo.toml` 中声明且默认关闭。发布制品中不得出现该 feature 与探针表，判据由阶段 14 的发布门禁项 `RG-CI-PROBE-ABSENT` 承担，即发布制品的 `cargo tree -e features` 输出中不含 `ci-probe`，且交付安装包内的八个 PE 二进制中不含符号 `api_v1_system_echo`。判据形态不变，只换被测对象：首版交付形态是同一份安装包加服务注册脚本，没有镜像这一层。

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
| SecurityContext | 按 A-03 冻结的 20 个字段，顺序为 user_id、account_kind、session_id、legal_entity_id、device_id、client、clearance_level、roles、duty_classes、department_scope、position_ids、project_scope、customer_scope、record_shares、data_scope_tags、snapshot_version、is_breakglass、request_id、trace_id、system_purpose，位于 `crates/foundation/src/security/context.rs` | `human` 固定 `system_purpose=None`；`system(le,request,trace,purpose)` 使用 SYSTEM_PRINCIPAL_ID/SYSTEM_SESSION_ID/SYSTEM_DEVICE_ID，account=System、client=Ops、clearance=10、全部列表空、department=Explicit([])、snapshot=0、breakglass=false、purpose=Some，其余三个值取入参；不提供 with_ 变换，字段不得增删改名 |
| AccountKind、ClientKind、DepartmentScope、SystemPurpose | `AccountKind { Human, System, Portal }`；`ClientKind { Win, Mac, Ios, Android, Portal, Ops, ServerAdmin, Mcp }`，序列化值为 `win\|mac\|ios\|android\|portal\|ops\|server_admin\|mcp`；`mcp` 只由 grant middleware 固定；`DepartmentScope { All, Subtree(Id<Department>), Explicit(Arc<[Id<Department>]>) }`；`SystemPurpose { General, Reconciliation }` | 未知取值反序列化失败；外部自填 mcp 失败；`Reconciliation` 只允许由 `ReconExecutor` 构造，archcheck 规则 `reconciliation-context-confined` 拒绝其他出现位置 |
| DeviceId、RoleCode、DutyClass、RecordShare、DataScopeTag、RequestId、TraceId | 按 A-03 与第 13 节新增决定十二冻结，与 SecurityContext 同在 `crates/foundation/src/security/context.rs`：`DeviceId(Arc<str>)` 取长度 1 至 64 的 `[A-Za-z0-9_-]` 且可由 `&'static str` 无损构造；`RoleCode(Arc<str>)` 取长度 1 至 64 的 `[A-Z0-9_]`；`DutyClass { System, Data, Security, Audit, Key, Config }` 的序列化取值为 SYSTEM、DATA、SECURITY、AUDIT、KEY、CONFIG；`RecordShare { object_type: Arc<str>, object_id: uuid::Uuid, grant: RecordShareGrant }` 与 `RecordShareGrant { Read, Write }`，object_type 取 `<module>.<table>` 小写下划线形态并与事件信封 aggregate_type 同形；`DataScopeTag(Arc<str>)` 取 `<kind>:<value>` 形态，kind 为 `[a-z0-9_-]`、value 为 `[A-Za-z0-9_-]`，总长上限 128；`RequestId(Arc<str>)` 取长度 8 至 64 的 `[A-Za-z0-9_-]`，服务端自生成时取 UUIDv7 的无连字符十六进制；`TraceId(Arc<str>)` 取 32 位小写十六进制，与 W3C trace-context 的 trace-id 同形 | 七者只承载取值，不含任何判定逻辑，`RecordScope` 与 `RecordPredicate` 留在 ep-platform-authz 不前移；不合形态的字符串构造失败并返回 VALIDATION，未知枚举取值反序列化失败；`Arc<[DutyClass]>` 允许为空数组，`platform_authz.roles.duty_class` 为空的业务角色不产生条目，不设 None 变体；`DataScopeTag` 的序列化输出即公共列 `data_scope_tags text[]` 与事件信封 `data_scope_tags` 的元素形态，两处不得各自编解码 |
| AppError | code、category、message、details、retryable、incident_no、occurred_at、advice、source | Display 不输出 source 链，避免内部信息外泄 |
| DomainEvent | 基线第 6.1 节信封字段的强类型表达，payload 为泛型 | 信封字段增删会导致编译失败，事件目录不一致由 CI 检出 |
| Redacted\<T\> | Debug 与 Display 均输出 `***`，serde 序列化为 `"***"` | 任何 secrecy 之外的敏感值统一包这一层 |
| Tx、SnapshotCtx、UnitOfWork、TransactionLockProof | 按 A-01 冻结，位于 `crates/foundation/src/port/tx.rs`。`Tx` 含 tx_id、isolation、legal_entity_id、as_any_mut 四个方法；`SnapshotCtx` 含 snapshot_id、taken_at、legal_entity_id、as_any 四个方法；`UnitOfWork` 只有 `transact` 与 `snapshot_transact` 两个方法；另含 `TxId`、`IsolationKind { ReadCommitted, RepeatableReadSnapshot }` 与业务无关的不透明 `TransactionLockProof` | 契约层的跨模块方法签名一律写 `&mut dyn Tx`；`UnitOfWork` 不带池参数，一个实例在装配时绑定一个池；该 trait 含泛型方法不满足对象安全，application crate 对它取泛型参数 `U: UnitOfWork` 而不是 trait 对象；任何阶段不得改动这三者的签名。`TransactionLockProof` 只有 `from_authenticated_bytes(Vec<u8>)` 与 `authenticated_bytes(&self)->&[u8]` 两个载体方法，自定义 `Debug` 永远输出 `<transaction-lock-proof:redacted>`，不实现 serde、Default 或公开字段；它不理解 F-50、模块、表、锁序或业务类别，真实性、事务绑定和覆盖范围由签发它的协调器校验 |
| id::marker | `crates/foundation/src/id/marker.rs`，22 个零大小标记类型，清单为 LegalEntity、UserAccount、Session、Department、Position、Project、Customer、Supplier、Material、Product、Warehouse、Contract、ContractLine、SalesOrder、SalesOrderLine、DeliveryConfirmation、DeliveryConfirmationLine、PurchaseOrder、GoodsReceiptLine、PurchaseInvoice、PurchaseInvoiceLine、AccountingPeriod | 无字段、无方法、无 trait 实现，只承载类型身份，供 `Id<T>` 在契约层表达跨模块引用；清单固定 22 项，任何阶段不得增删，见第 13 节偏离二；由 archcheck 的 `foundation-frozen-items` 按名逐项断言，改名与增删同样报错 |
| SYSTEM_PRINCIPAL_ID、SYSTEM_SESSION_ID、SYSTEM_DEVICE_ID | `crates/foundation/src/principal.rs`，取值分别为 `00000000-0000-7000-8000-000000000001`、`00000000-0000-7000-8000-000000000002`、`SYSTEM` | principal 对应唯一 SYSTEM account 种子行；session 只是上下文哨兵、sessions 表不建行且不可 reauth/续期；各阶段不得自选取值 |
| ModuleCode | 按基线第 1.2 节 15 个模块码冻结的枚举，取值为 Mdm、Crm、Cpq、Clm、Sales、Procure、Inventory、Costing、Project、Service、Finance、Ledger、Invoice、Portal、Reporting | 未知取值反序列化失败；许可、对账、跨模块来源标注一律引用该枚举 |
| CapabilityDomain、ActionClass | `crates/foundation/src/capability.rs`，`CapabilityDomain` 18 项，序列化取值与阶段 13 第 4.4 节能力域码表的 18 个字符串逐字一致且顺序与该表序号一致；`ActionClass { Read, Write, Submit, Approve, Export }` | 本阶段只定义枚举，不做任何运行期判定；各阶段按裁定 A-20 的两类落点，在承载该路由处理器的 crate 的 `src/capability.rs` 中为每个用例声明 `<USECASE_SCREAMING>_DOMAIN` 与 `<USECASE_SCREAMING>_ACTION` 一对常量，业务模块的路由落 `crates/contract/<module>/src/capability.rs`，`/api/v1/platform/` 下的平台路由落 A-20 逐阶段指名的 platform crate 的 `src/capability.rs` 并一律取 `CapabilityDomain::PlatformAdminLowcodeOps`，不设第三类落点；`ci-probe` feature 门控的探针路由与 `/internal/v1/` 下不对四端暴露的内部端点不参与判定，不声明常量；`xtask configdoc` 断言每个 `/api/v1/` 路由都能解析到一对常量，缺失即构建失败 |

端口 trait：`Clock`（now 与 today_cn）、`IdGen`（new_id）、`Rng`（fill_bytes）、`IncidentNoGen`（next）。domain 层禁止绕过这四个端口，由 `xtask archcheck` 的符号禁令强制。另在 `crates/foundation/src/port/` 下建 `search.rs`、`doc.rs`、`db.rs` 与 `kms.rs` 四个空文件，本阶段只写模块注释：按裁定 F-01，`IdempotencyStore`、`MigrationWindowGuard` 与公共能力基线的能力描述由阶段 2 补进 `db.rs`，`ReadOnlyTx` 由阶段 11 补齐；按 A-07，`SearchDocument`、`SearchQuery`、`SearchHit`、`SearchIndexPort`、`SearchQueryPort` 由阶段 3b 补齐；按 A-08，`SheetSpec`、`ColumnSpec`、`CellValue`、`PrintLayout`、`SpreadsheetPort`、`DocTemplatePort`、`PdfRenderPort` 由阶段 5 补齐；按裁定 F-04/F-56，阶段 2 在 `kms.rs` 补齐 `KmsBackend` 的 `wrap|unwrap|derive_blind_key|sign|verify|health` 六方法，并并列补齐 `KmsSigningKeyIdentityResolver`、`KmsKeyMaterialProvisioner`、`KmsPinnedDataKeyBackend` 三个独立端口及 strong values；内置 KMS 与客户 HSM 两种载体的实现类型落 `ep-adapter-kms`、本模块不声明任何载体类型。F-51 已将 `derive_blind_key` 的三参数形态与返回宽度一并冻结，`BlindIndex` 固定为完整 `[u8; 32]`；F-56 把历史 key wire 冻结为 `DataKeyRefV1`/`ExactRef`，新写只用 `CurrentForWrite`，不得截断、配置宽度或把 pinned 能力塞进六方法 ABI。四个文件的路径在本阶段固定，后续阶段只补内容不改位置。

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
| Ready 或 Degraded | StopRequested | Draining | 停止接收新请求 |
| Draining | DrainComplete | Stopped | 在途请求归零或超过 drain 上限，退出码 0 |
| 任意 | Panic | Failed | 捕获后先写日志再退出，退出码 70 |

非法迁移一律返回 BUSINESS_CONFLICT 并记 ERROR，不 panic。退出码 78 与 70 的取值本身保留，但 Windows 服务控制管理器没有 `RestartPreventExitStatus=78` 的按码白名单。唯一实现先落主承载：配置错误路径报告 `SERVICE_STOPPED` 且 `dwWin32ExitCode=78`，panic 路径不报告 `SERVICE_STOPPED`、直接以 70 终止并由 `sc failure` 恢复。裁定 F-08 第十二节第 5 项是该行为的首批实施验证门禁；证据形成前不得声称自动重启分流已通过。若实机不支持主承载，预定失败支路只保留 78／70 对外可见性、放弃 70 自动重启并写入交付说明，不另选平台。退出码用例须预先 `OpenProcess` 持住句柄，停机后以 `GetExitCodeProcess` 断言真实码并与服务自报值双向比对。

#### 5.5 启动自检算法

自检项以注册表实现，注册表为 `SelfCheckRegistry`，位于 `crates/platform/runtime/src/selfcheck/registry.rs`，每项是一个 `SelfCheckItem { name, title, severity, run }`，name 为 kebab-case，severity 的取值域由本阶段定死为 Blocking 与 Degrading 两值，不设第三值。Blocking 项只判读二进制、环境、目录与数据库元数据，失败即以退出码 78 拒绝启动；Degrading 项失败不阻止启动，进程进入 Degraded 并由承接阶段登记降级窗口。任何阶段不得注册判读业务数据行的 Blocking 项，业务数据的一致性由规格第 10.2 章的对账组件与降级窗口承接，不由启动闸门承接。按 C-25，自检项一律按注册名标识，本计划与后续阶段都不再用序号称呼，基线第 7.3 节的十三项编号列表同步改为十项命名列表，见第 13 节新增决定十三。十个基线项中 `config-parsed`、`database-reachable`、`migration-version-matched`、`rls-enabled-and-forced`、`runtime-role-privileges-bounded`、`secrets-resolvable`、`file-store-writable`、`clock-skew-within-limit` 八项为 Blocking，`audit-chain-verifiable` 与 `offsite-sink-requirements` 两项为 Degrading。报告按注册顺序输出，基线十项在前，各阶段追加的命名项在后。本阶段实现其中六项，另三项以 Pending 登记，`offsite-sink-requirements` 本阶段不登记。

`config-parsed`，配置解析成功且无未知键，由 serde 的 deny_unknown_fields 与分层加载器返回。

`database-reachable`，数据库可达且服务端版本为 16.x，`timezone` 为 UTC，`max_connections` 不低于 52，`max_wal_senders` 不低于 4，`max_replication_slots` 不低于 3；本阶段切片先覆盖不建库连接的五个进程 portal-gateway、integration-gateway、plugin-host、archive-writer、backup-writer，F-55 Task 4 再把 `ai-inferer` 加为终态第六个。该六进程对全部需要 SQL 会话的自检项一律跳过并标注 `NotApplicable`，不止本项；两个写出进程按规格第 7.7 章只持 REPLICATION 属性，任何 SQL 类自检项对它们都不成立。非 SQL 自检不因本规则跳过，基线第 7.3 节所称十三项为全部进程共有一句同步作废。

`migration-version-matched`，迁移清单一致。算法：从唯一的 `platform_core.schema_history` 读出 `(version, name, checksum)` 三元组并按 `version` 升序排序；构建期由同一个 `migration_manifest` 库对 `db/migrations/` 全部文件执行与 Runner 相同的路径解析、LF 归一化、行尾空白去除和校验和计算，得到期望三元组序列。运行期把两组规范序列逐项全等比较，并把期望序列的 SHA-256 固化为 `EP_MIGRATION_MANIFEST_SHA256` 供制品自检；缺行、多行、版本、名称或校验和任一不等即失败。任何进程都不执行迁移。本阶段 `db/migrations/` 下只有 24 个空目录，清单为空集，判定平凡通过，阶段 2 写入迁移文件后该项开始有实质内容。

`rls-enabled-and-forced`，全部带 legal_entity_id 列的表均已 ENABLE 且 FORCE 行级安全。算法：查 information_schema.columns 取出含该列的表集合，与 pg_class 的 relrowsecurity 与 relforcerowsecurity 比对，差集非空即失败；同时查 pg_roles 断言当前角色 rolbypassrls 与 rolsuper 均为假。本阶段生产库上该集合为空，判定平凡通过，被测对象为测试库中的 `ci_probe.probe_records`，业务表集合自阶段 2 起逐步建立。

`runtime-role-privileges-bounded`，运行期账号不具备 DDL、角色管理与策略管理权限。算法：对 24 个 schema 逐一 `has_schema_privilege(current_user, s, 'CREATE')` 必须为假，`rolcreaterole` 与 `rolcreatedb` 必须为假。本阶段 24 个 schema 由阶段 2 建立，代码路径在测试库上以等价授权验证。

`clock-skew-within-limit`，时钟偏差小于 1 秒。算法：本平台没有 `adjtimex` 与 `/proc`，该判据的被测对象不存在。按裁定 F-08 第八节两支择一，本阶段取第二支——**该自检项在本平台永久停在「未覆盖」并就此登记**，不换一个看似对应的 Windows 计数器凑数；未覆盖不等于通过，其失败分支在本平台无从构造，故退出条件 6 的「六项已实现自检各自的通过与失败分支均有集成测试」对本项不适用，该例外一并登记。本项的重新生效谓词是机器可观测的事实：一旦本平台出现可读的时间同步状态源并登记进自检项注册表，本项自动转为真判定。

三个 Pending 项的接管方固定如下。`secrets-resolvable` 的 `KmsSecretProvider`、bootstrap、recipient 隔离、信封与非 SQL 判定由阶段 2 实现；`audit-chain-verifiable` 与 `file-store-writable` 由阶段 3b 实现，其中 `audit-chain-verifiable` 按 Degrading 登记。法人数据密钥域覆盖率不是 `secrets-resolvable` 的子段：阶段 2 只交付独立 `legal-entity-key-domain-coverage` provider 与结构化结论，阶段 14a/14b 在终态 21-kind Rust/DB 接受域形成后才注册并接真实窗口。Pending 是如实上报的一种结论，不是空实现，本阶段不为任何未实现的自检项写返回成功的桩。`offsite-sink-requirements` 本阶段既不登记也不留 TODO 注释：按 A-26 该项未满足时要登记降级窗口，而 `DegradationLedger` 归阶段 2、落点判定归阶段 14，两者都不在本阶段，因此该项整条推迟，由阶段 14 在交付落点判定的同一批里连同 `DegradationLedger::open` 的调用一次登记为 Degrading 项。`license-and-modules-consistent` 与 `current-period-open` 两项整项删除，理由与承接方见第 13 节新增决定十三。

`--check` 模式按顺序执行全部注册项与 Pending 项，输出一份 JSON 报告到 stdout 后退出，不监听端口。报告结构为 `{ process, version, items: [{ name, title, outcome: PASSED|FAILED|DEGRADED|PENDING|NOT_APPLICABLE, detail }], overall }`。`--check` 的判定严于运行期：任一 FAILED 或 DEGRADED 均为非零退出，Pending 不计入成败。降级的闸门就落在这里与升级前置脚本上，不落在进程启动上。

#### 5.6 资源限额取值与一次性部署校验

本节以下历史段落中的“待实测／待定／重开”统一按证据状态读取，不再表示设计可二选一。首批唯一实现先交付具名 Job Object、DACL、静态限额与 Windows 校验夹具；八个自研进程启用已冻结的内存硬上限，CPU 比例与磁盘 IO 份额不启用，PostgreSQL/反向代理指派和 backup-writer 绝对 IO 上限先实现主路径但能力状态标“未验证”。只有 F-08 对应实机门禁通过后才可把该能力标为启用；失败即走前文保守降级并保持门禁非零。

本阶段不做配额生成器，也不产出 `quotas.generated.toml`。规格第 13.1 章配额表在本平台的唯一承载物是具名 Job Object 与 `deploy/` 静态限额文件，不做生成算法或按可分配量运行期折算。首版实现路径一次冻结：八个自研二进制由服务宿主在 `ServiceMain` 早期创建或打开资源单位并自我指派；PostgreSQL 16 与反向代理由 ops-agent 创建资源单位后调用 `AssignProcessToJobObject`，实机读回证据形成前状态为 `UNVERIFIED`，不计入覆盖但不切换实现。内存硬上限是配额表唯一启用的运行期列，落 `JOB_OBJECT_LIMIT_JOB_MEMORY`；绝对字节按 BC-1 算定。`MemoryLow` 整列删除，因为 Windows 最小工作集不等于提交量保底；`IOWeight` 整列删除，因为 Windows 的绝对限速/预留不能表达按权重借用与收敛；CPU 比例首版固定只作硬件标定与认证意图声明，不落运行期值，`MinRate/MaxRate` 不作为当前版本的实测后备分支。backup-writer 的全量备份写出另有一个 MB/s 绝对上限，按补裁乙不进配额百分比表，唯一落点为静态限额文件、部署记录与 Windows 读回夹具，行为证据形成前状态为 `UNVERIFIED`。两个未验证能力均须先实现主路径再取证；证据失败保持门禁非零与保守披露，不自动启用 CPU 或另造第二套机制。

删除的是三样东西：随进程交付的配额生成器与其 `quotas.generated.toml` 产物、`min(份额×3, 可分配量的 40%)` 的突发上限一列、以及每个进程每次启动都比对一次的自检项。核对改由 `scripts/verify-resource-limits.ps1` 在部署与升级时各执行一次，使用 Windows API 读回具名 Job Object 限额并核对 DACL 与静态限额文件，不一致即退出非零；`cargo xtask ci` 第 11 阶段调用该核对，17 项有效实机证据未完成前保持非零。任何 bash、`/sys/fs/cgroup` 或历史 TSV 状态均不是现行承载。把核对放在部署与升级时点，可避免一处配置漂移令八进程集体拒绝启动；但备份窗口仍须按认证时延线实测，不能以本机用户少为由免测。

三处偏差如实披露，不以加和或拆分手段掩盖。其一，规格第 13.1 章配额表共九行，内置搜索索引一行在首版没有独立进程，也没有独立资源单位，其份额不落静态限额文件、不加和到任何一行、也不按比例拆分给其余各行，因此八行的取值之和低于该表对应列的总和：CPU 标定一列合计 90，内存一列按百分数算定的绝对字节合计只相当于该表的 90%，余下 10% 成为未指派的机器余量。这里有一处与 cgroup 侧不同的后果须点名：本平台的内存承载是各自独立的绝对硬上限而不是按权重归一化的比例分配，实际承载搜索索引负载的 core-server 与 job-worker 两个资源单位不会因此自动分到那 10%，而是各自被自己的硬上限压住；CPU 一列因不落运行期取值，其缺口只影响硬件标定与认证意图声明。该偏差在首版接受，未来搜索索引具备独立进程与独立资源单位时须另立版本变更后补齐。其二，规格第 13.1 章的突发上限一列在首版没有承载物，且按裁定 F-08 补裁甲，「其余各行的突发上限取其份额的三倍并以可分配量的 40% 封顶」是一条相对量折算，磁盘 IO 一列已删、CPU 一列首版不启用，被乘数消失，整条不成立并已随规格改写删除；backup-writer 的磁盘 IO 绝对上限不是该列的实现，按补裁乙不进该配额表。其三，磁盘 IO 份额一列在本平台整列无运行期承载，第 13.3 章 RPO 不超过 15 分钟因此失去机制侧保证，完全押在附录 A.4 的认证实测上；这一条是实质降级而不是措辞变化，与前两处并列披露，不得沉默。

#### 5.7 可复现构建算法

`cargo xtask ci` 第 8 阶段把 `SOURCE_DATE_EPOCH` 固定为 Git 提交的 committer 时间，并用解析后的 Windows 工作目录与离线 Cargo 目录生成 `--remap-path-prefix`，不依赖 `$PWD`、`$CARGO_HOME` 或 bash。构建目标只取 `x86_64-pc-windows-msvc`，以锁文件和离线依赖仓库做两次发布构建，比对八个 PE 二进制的 SHA-256，任一不等即失败并输出差异。实机证据生成前该阶段保持非零并标“未验证”；历史流水线状态不得将其转绿。

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

echo 端点存在的唯一理由是让封套、错误映射、并发闸门、同步等待上限、请求头校验、追踪与日志七条横切链路在阶段 1 就有端到端用例。按 B-01，它由 `#[cfg(feature = "ci-probe")]` 保护，feature 名固定为 `ci-probe`，在 `apps/core-server/Cargo.toml` 与 `testkit/Cargo.toml` 中声明且默认关闭；`xtask codecheck` 断言发布 profile 不启用该 feature，e2e 用例断言按发布 profile 构建并安装后该路径返回 404；发布制品层面的判定由阶段 14 的发布门禁项 `RG-CI-PROBE-ABSENT` 承担，判据为 `cargo tree -e features` 输出中不含 `ci-probe` 且交付安装包内的八个 PE 二进制中不含符号 `api_v1_system_echo`。

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
| portal-gateway | GET /portal/v1/system/health、/portal/v1/system/metrics（8090） | 8090 只由第三方反向代理调用并只呈现 portal 本进程状态；core 可达性由 ops-agent 以 `ep-ops` 调 `ep-core` 的 `health.get.v1` 独立采集，不给 `ep-portal` 增加健康探测权限 |
| integration-gateway | 无 HTTP；`health.get.v1`、`metrics.snapshot.v1` 经 `\\.\pipe\ep-integ` | 出网客户端只做一次对配置白名单的自检式解析，不发起真实请求 |
| plugin-host | 无 HTTP，仅 IPC | 见下 |
| ops-agent | GET /metrics（9101）、GET /healthz、/readyz（9102） | /metrics 聚合本机七个进程的指标端点，抓取失败按目标标记 up=0 |
| archive-writer、backup-writer | 无监听 | 仅 IPC 客户端与 spool |

#### 6.3 IPC 契约

承载为 Windows 命名管道（`tokio::net::windows::named_pipe`），三条名字固定为 `\\.\pipe\ep-core`、`\\.\pipe\ep-integ`、`\\.\pipe\ep-plugin`；不取 AF_UNIX 或产品进程间回环 TCP。唯一窄例外不是产品 IPC：integration-gateway 可作为客户端连接客户自管的同机 ICAP，目标只能是 IP 字面量 `127.0.0.1` 或 `[::1]`，禁止主机名、DNS、代理、重定向与非回环地址，明文流式经过内存且不得落盘。三条管道用 `reject_remote_clients=true` 与显式逐账户 DACL，默认安全描述符禁止使用；原 Unix 权限、组与 socket 目录整体撤销。

每个 server generation 先创建一个带 `first_pipe_instance(true)` 的首实例以抢名并 fail-closed；成功后，同一持有首实例的服务进程创建后续或补位实例一律用 false。首实例句柄贯穿 listener 生命周期，断开后在同一句柄重新接受；异常丢失则整个服务退出并由 SCM 重启，不允许 false 实例继续提供服务。一个实例只服务一个连接，循环须先准备接受余量再交出当前实例。客户端处理 `ERROR_PIPE_BUSY`，且每次重连都重新核验服务端，不能把旧核验结果复用到新实例。

客户端必须以 `SECURITY_SQOS_PRESENT|SECURITY_IDENTIFICATION` 打开管道，禁止服务端借用调用方权限。服务端连接后、读取任何应用字节前调用 `ImpersonateNamedPipeClient`，用 `OpenThreadToken` 核验允许的服务 SID/账户，并在所有成功/失败分支立即 `RevertToSelf`；客户端 PID 只作审计关联，不作授权。客户端发送前经 server PID 与进程 token 核验预期服务账户，核验或发送失败即关闭；不得转投未核验实例。

抗占满常量固定为：ep-core 总实例 32，portal/archive/backup/ops 活跃上限 20/4/4/2；ep-integ 总实例 16，worker/core/ops 为 8/4/2；ep-plugin 总实例 12，core/worker/ops 为 4/4/2。各账户合计比总实例少 2，至少保留一个接受实例与一个轮换余量。超账户额度在身份核验后、读 body 前返回 `PLATFORM.IPC.CONCURRENCY_LIMIT` 并审计；流会话全程占额度。一账户占满不影响其他账户。普通身份握手、首长度前缀、空闲、单调用绝对上限为 5/10/30/120 秒；流协议为 10/30/3600 秒。半帧、慢帧、断连、超长帧均清缓冲并关闭。帧格式为 4 字节大端长度前缀加 JSON 体，单帧上限 1 MiB。

```json
{ "v": 1, "kind": "request", "id": "<uuidv7>", "operation": "health.get.v1", "payload": {} }
{ "v": 1, "kind": "response", "id": "<同上>", "ok": true, "payload": { "process": "core-server", "version": "..." } }
{ "v": 1, "kind": "response", "id": "<同上>", "ok": false, "error": { "code": "...", "category": "...", "message": "..." } }
```

本阶段在三条服务端管道都实现 `health.get.v1` 与 `metrics.snapshot.v1`，调用账户仅为 `NT SERVICE\ep-ops`；业务 operation 的字符串、账户矩阵与 DTO 可在契约表中冻结，但在其所属阶段前必须失败关闭，不得以成功空壳代替。CI 断言未列 operation、错误账户与通配白名单均被拒绝而不是 panic。

> **现行 IPC 清单，取代本节前文的历史枚举。** 产品管道固定为 `\\.\pipe\ep-core`、`\\.\pipe\ep-integ`、`\\.\pipe\ep-plugin`；server 分别为 `NT SERVICE\ep-core|ep-integ|ep-plugin`。server 在读取应用字节前执行 `ImpersonateNamedPipeClient`→`OpenThreadToken` 校验服务 SID/账户，并在所有分支 `RevertToSelf`，PID 只作审计关联；client 以 `SECURITY_SQOS_PRESENT|SECURITY_IDENTIFICATION` 打开并在发送前校验 server 进程 token。实现清单不得使用任何通配模式。
>
> - `ep-core` 客户端 ACE 只含 `ep-portal|ep-archive|ep-backup|ep-ops`。portal 身份三项为 `portal.session.sign_in.v1|portal.session.sign_out.v1|portal.identity.me.v1`；五项业务能力由 `portal.order_confirm.v1|portal.delivery_notice.v1|portal.invoice_upload.begin.v1|portal.invoice_upload.chunk.v1|portal.invoice_upload.end.v1|portal.invoice_upload.abort.v1|portal.settlement_query.v1|portal.profile_maintain.v1` 承载。archive 只可调用 `ops.attachment_writeout_scope.query.v1|ops.writeout_result.report.v1|ops.failure_event.report.v1|ops.replication_lifecycle.report.v1`。backup 只可调用 `ops.writeout_result.report.v1|ops.verification_conclusion.report.v1|ops.failure_event.report.v1|ops.replication_lifecycle.report.v1|ops.attachment_checksum_verdict.report.v1|ops.backup_slot.acquire.v1|ops.backup_slot.release.v1`。ops 只可调用 `health.get.v1|metrics.snapshot.v1`。
> - `ep-integ` 客户端 ACE 只含 `ep-worker|ep-core|ep-ops`。worker 只可调用 `push.dispatch.v1|esign.request.submit.v1|esign.status.get.v1` 并在同一已关联双工连接接收 `esign_file.begin.v1|esign_file.chunk.v1|esign_file.end.v1|esign_file.abort.v1`；core 只可调用 `virus_scan.begin.v1|virus_scan.chunk.v1|virus_scan.end.v1|virus_scan.abort.v1`；ops 只可调用 `health.get.v1|metrics.snapshot.v1`。
> - `ep-plugin` 客户端 ACE 只含 `ep-core|ep-worker|ep-ops`。core 与 worker 只可调用 `wasm.execute.v1`；ops 只可调用 `health.get.v1|metrics.snapshot.v1`；取消由 deadline 或断开当前调用表达，不设 cancel operation。
>
> 产品业务命令/数据一律走管道；core:8080 与 portal:8090 只作第三方反向代理 upstream，job-worker:8081 与 ops-agent:9101/9102 只作无业务数据健康/指标探测，integration-gateway 不监听 TCP。阶段 3 冻结的 `BoundedChunkStreamV1` 同时承载病毒 5 GiB、签章文件 5 GiB 与门户发票 50 MiB 正文，统一分块、逐块 ACK、单块在途、长度/哈希/超时/abort，不得另造 HTTP 上传或第二套流协议。唯一回环 TCP 窄例外仍只是 integration-gateway 作为客户端连接客户同机 ICAP。

spool 行为：archive-writer 与 backup-writer 在 core-server 不可用时把待上报帧追加到各自 `C:\EP\<proc>\spool\`，恢复后按 `(occurred_at, report_id)` 幂等补写并只在 core 确认入库后截断；两进程都只读取通用 `spool.dir`、`spool.max_bytes`，默认 20 GiB，不存在 archive/backup 专属同义键。五类 critical 报文 `WriteoutResult|VerificationConclusion|FailureEvent|ReplicationLifecycle|AttachmentChecksumVerdict` 永不删除、覆盖或静默丢弃；只有可由本地 manifest 与落点清单确定性重建的 `HEARTBEAT|PROGRESS_SNAPSHOT` 可按对象合并为最新一条。软水位固定为上限减 64 MiB，余量仅供在途 critical 收尾；达到软水位后继续 WAL 接收与当前写出，但停止新备份/附件周期并写 Windows Event Log，恢复后让 core 打开不可抑制 `WRITER_NOT_IN_SERVICE`（subject=`<writer>:report-spool-exhausted`），重放完成且回落后关闭。按裁定 F-08 第 4.3 节路径长度一条，安装根唯一为 `C:\EP`。安装器须断继承并显式设 DACL，启动时核对 ACL并实建探针文件；追加先 flush，再原子切换 manifest，句柄冲突有限重试。

### 7. 并发与事务边界

本阶段没有业务事务，但把事务与并发的全部约束以可执行的形式固定下来。

#### 7.1 工作单元

`ep-foundation` 定义 `UnitOfWork`，两个方法为 `transact` 与 `snapshot_transact`，`ep-adapter-db-pg` 提供唯一实现。按 A-01，事务句柄为 `ep_foundation::port::Tx`，快照上下文为 `ep_foundation::port::SnapshotCtx`，契约层的跨模块方法签名一律写 `&mut dyn Tx`，原先的 `TxHandle` 与 `transact_repeatable_read` 两个名字作废。本阶段的实现要点：`transact` 的隔离级别固定 READ COMMITTED；`snapshot_transact` 是只读快照事务的唯一入口，配合 `SET TRANSACTION SNAPSHOT` 使用，供后续的对账与关账前校验取用，两者是仅有的两个入口；`UnitOfWork` 不带池参数，一个实例在装配时绑定一个池；闭包返回后统一提交，返回 Err 统一回滚；闭包内不允许发起外部调用，由 `xtask archcheck` 对 `ep-app-*` 的符号禁令强制；跨 crate 取具体句柄只允许 `tx.as_any_mut().downcast_mut::<PgTx>()` 一种写法，且只允许出现在 `crates/adapter/db-pg/` 内。本阶段不定义 `AuditSink` 与 `OutboxSink` 两个 trait，也不在事务闭包内留空实现写入位。`UnitOfWork::transact` 的闭包签名已按 A-01 冻结，阶段 3a 交付审计与 Outbox 本体时在闭包内直接调用即可，事务边界本就不需要改动，不必先摆一个返回成功的桩。

#### 7.2 连接池

四个具名池，池参数与超时逐池固定；integration-gateway 不建池、不解析数据库配置。

| 池 | 归属进程 | 上限 | statement_timeout | lock_timeout | idle_in_transaction | 其他 |
|---|---|---|---|---|---|---|
| rw | core-server | 20 | 10s | 3s | 15s | 事务预算 5 秒由应用侧计时并告警 |
| ro | core-server | 10 | 60s | 3s | 15s | work_mem 64MB，temp_file_limit 2GB |
| worker | job-worker | 5 | 300s | 3s | 15s | 同一运行期读写账号 |
| ops | ops-agent | 2 | 5s | 3s | 15s | ep_ops_ro |

取用连接时执行四条 `select set_config('app.legal_entity_id'|'app.user_id'|'app.request_id'|'app.trace_id', $1, false)`，归还前逐项设回空串，不使用 DISCARD ALL。两处钩子实现在 `ep-adapter-db-pg` 的 `PgPoolFactory`，业务代码不得直接调用 set_config，由符号禁令强制。集成测试断言归还后的连接上四个变量均为空串，并断言在未设置法人变量时对 `ci_probe.probe_records` 的读、写、更新均返回零行或权限错误，即默认拒绝。
按 C-04，下列四个类型由本阶段在 `ep-adapter-db-pg` 中定义，四者都不进 `ep-foundation`：`PoolKind { Rw, Ro, Worker, Ops }`；`SessionContext { legal_entity_id, user_id, request_id, trace_id }`；`RetryPolicy { max_attempts: u8, backoff_ms: [u16; 3], retryable_sqlstates: &'static [&'static str] }`；`ConnectionBudget { resident_max: u16, temporary_max: u16, peak_max: u16, safety_headroom: u16, per_pool: [(PoolKind, u16); 4] }`。冻结值为常驻 37、迁移/应急临时 10、显式安全余量 5、硬峰值 52；四池明细 20+10+5+2=37，`37+10+5=52`。本阶段只交付类型定义与本节表中的逐池上限，`RetryPolicy` 与 `ConnectionBudget` 的 Windows 校验脚本 `scripts/verify-connection-budget.ps1` 归阶段 2，本阶段不在计划中声称提供该脚本。

#### 7.3 重试

只对尚未产生任何外部可见副作用的事务重试，触发条件为 SQLSTATE 40001 与 40P01，重试 3 次，退避 50、150、450 毫秒。按 C-21，事务重试的唯一指标名是 `ep_db_tx_retries_total`，类型为 counter，标签为 pool 与 sqlstate，由阶段 2 注册与填充；本阶段撤销原拟的 `ep_db_retries_total` 登记，也不登记任何同义指标，`docs/metrics-catalog.md` 的唯一性校验由本阶段的 `xtask` 实现。重试判定由 `RetryPolicy` 集中实现，业务代码不自行捕获这两个错误码，由 `xtask codecheck` 断言 `40001` 与 `40P01` 字面量只出现在该文件中。

#### 7.4 并发闸门与同步等待

并发闸门放在 core-server，理由是 portal-gateway 不建数据库连接且其取数一律经 core-server 的受控能力 API，core-server 是唯一的合流点，因此在 core 上限 20 即等于内部与门户合计上限 20。实现为 tower 的信号量层，许可数取配置值，等待超过 10 秒返回 503 与 `PLATFORM.CAPACITY.CONCURRENCY_LIMIT`，并写一条不含用户、单号或追踪号作为指标标签的结构化拒绝日志；旧 `ep_quota_throttled_total` 已撤销，不得注册或填充。已获得许可的在途请求不受影响，不做静默降级。portal-gateway 侧另有一层按来源 IP 的限流，本阶段只交付参数与骨架。

同步等待上限 8 秒实现为 tower 的超时层，超时返回 PLATFORM.SYSTEM.SYNC_TIMEOUT。后台任务承接路径由任务阶段实现，本阶段在错误 advice 中写明该请求应改由后台任务表达。

#### 7.5 幂等

按 C-07，幂等分三段，本阶段只做第一段。本阶段的中间件名固定为 `IdempotencyKeyHeaderGuard`，只校验 `Idempotency-Key` 头存在且为合法 UUIDv7，不合法返回 `PLATFORM.IDEMPOTENCY.KEY_REQUIRED`，本阶段不做任何判等与重放存储。第二段是端口定义，`ep_foundation::port::db::IdempotencyStore` 及其 `try_begin` 与 `finish` 两个方法、`IdempotencyScope { legal_entity_id, user_id, endpoint, key }`，归阶段 2。第三段是 `platform_msg.idempotency_keys` 建表与重放实现，返回 `IdempotencyOutcome::FirstCall`、`Replay { status, body }` 或 `PayloadMismatch`，归阶段 3a。本阶段不建该表，三处不得各自判等。按 C-24，`PLATFORM.IDEMPOTENCY.PAYLOAD_MISMATCH` 由本阶段一并登记在错误码表中，但本阶段不可能返回它，返回方是阶段 3a。中间件已按最终形态分层，接入存储时只需注入一个 `IdempotencyStore` 实现。

#### 7.6 优雅停机与崩溃

收到停止请求后进入 Draining，停止接受新连接，等待在途请求完成，上限由配置控制默认 30 秒，超时后强制关闭并记 WARN，退出码仍为 0。停止请求在本平台的承载是服务控制管理器投递的停止控制码，控制台直跑模式下是 Ctrl+C 或 Ctrl+Break 事件，原 SIGTERM 一路不存在。按裁定 F-08 做不到四，排空须按两条路径分别表述，不得合并为一句：显式停止路径（`sc stop`）上，服务只要持续抬 `dwCheckPoint` 就不被强杀，30 秒排空成立且可判；机器关机路径上的等待受全机注册表值 `WaitToKillServiceTimeout` 约束，该值远小于 30 秒，拉长它要改客户机器的系统设置，按该裁定第零节此类处置一律判为做不到，因此关机路径上的 30 秒排空不成立，如实登记为交付说明中的一条降级，不得以「一般不会在关机时有在途请求」把它写掉（该值在目标区间两版的当前默认值、以及它是每服务预算还是全部服务的总预算，见该裁定第十二节第 9 项，实测只量化该降级、不改变其定性）。原「systemd 的 `TimeoutStopSec` 取 45 秒」随 systemd 一并撤下，本平台不设第二个超时值，也不设任何自造的等价余量。panic 由 catch_unwind 层捕获，先写一条含 trace_id 的 ERROR 日志，再返回 PLATFORM.SYSTEM.INTERNAL_ERROR，不中止进程；只有自检失败与配置错误才中止进程。

#### 7.7 与 Outbox 的关系

本阶段不写任何 Outbox 条目，也不消费，也不为消费预留任何钩子。`JobRegistry` 在本阶段只有注册与调度两件事，已注册任务为零个；至少一次投递与幂等消费的形态由阶段 3a 在交付 Outbox 消费时连同 `consumer_name` 与去重一次给出。理由是本阶段没有任何消费者，未被使用的钩子无从验证，只有维护成本没有判据。

### 8. 配置项

配置结构体开启 `deny_unknown_fields`，加载顺序按基线第 7.1 节五层。下表是本阶段新增的全部配置键，同步写入 `docs/config-reference.md`，由 `xtask configdoc` 断言代码与文档逐键一致，缺一即失败。生效方式一列中，启动表示改动后需重启，取用表示在下次取用时生效。原第三档「SIGHUP 表示可热加载」本阶段撤下：本平台没有 SIGHUP，且按裁定 F-08 第八节实测，全仓从来没有 SIGHUP 处理器——该档在换平台之前就已经没有被测对象。本阶段不新增任何热加载触发机制：该裁定第九节把本次新增机制诚实统计为三个（服务宿主层、本地日志落地与轮转、命名管道的 DACL 构造与忙重试），另起一个触发器即越出该统计。因此本表原标 SIGHUP 的各键一律按启动生效，热加载能力如实登记为本阶段不交付；日后要交付须另立一条同时给出触发手段与被测对象的决定，不得只在本表改一个词。

| 键 | 类型 | 默认值 | 生效方式 |
|---|---|---|---|
| http.bind_addr | string | 按进程固定，core 为 127.0.0.1:8080 | 启动 |
| http.max_body_bytes | u64 | 1048576 | 启动 |
| http.request_timeout_ms | u32 | 8000 | 启动 |
| http.shutdown_drain_ms | u32 | 30000 | 启动 |
| http.concurrency_limit | u16 | 20 | 启动 |
| http.concurrency_wait_ms | u32 | 10000 | 启动 |
| ipc.socket_path | path | 服务端进程固定为 ep-core、ep-integ、ep-plugin 三值之一 | 启动 |
| ipc.max_frame_bytes | u32 | 1048576 | 启动 |
| db.host、db.port、db.database | string、u16、string | 127.0.0.1、5432、ep | 启动 |
| db.user | string | ep_app_rw | 启动 |
| db.password_ref | string | secret://db/app_rw#1 | 取用 |
| db.pool.rw_max、ro_max、worker_max、ops_max | u16 | 20、10、5、2 | 启动 |
| db.pool.acquire_timeout_ms | u32 | 8000 | 启动 |
| db.pool.max_lifetime_s、idle_timeout_s | u32 | 1800、300 | 启动 |
| db.timeout.<池>.statement_ms | u32 | 见第 7.2 节 | 启动 |
| db.timeout.<池>.lock_ms | u32 | 3000 | 启动 |
| db.timeout.<池>.idle_in_tx_ms | u32 | 15000 | 启动 |
| db.ro.work_mem_kb、temp_file_limit_kb | u32 | 65536、2097152 | 启动 |
| db.retry.max_attempts | u8 | 3 | 启动 |
| db.retry.backoff_ms | u32 数组 | [50,150,450] | 启动 |
| log.level | string | info | 启动 |
| log.debug_auto_off_minutes | u16 | 30 | 启动 |
| metrics.enabled | bool | true | 启动 |
| metrics.bind_addr | string | 按进程固定 | 启动 |
| trace.sample_ratio | f32 | 0.1 | 启动 |
| trace.otlp_enabled | bool | false | 启动 |
| trace.otlp_endpoint | string 可空 | null | 启动 |
| secrets.dir | path，首版固定值 | C:\EP\secrets | 启动；Stage 2 生产终态出现不同值以 78 拒绝，Stage 1 曾取用自定义目录只属历史迁移输入 |
| secrets.provider | enum kms | kms | 启动；生产闭集，Stage 1 file 只属历史迁移输入 |
| selfcheck.clock_skew_max_ms | u32 | 1000 | 启动 |
| runtime.worker_threads | u16 | 0，表示固定按整机可用核数推导；首版 CPU 比例不启用，因此始终不从资源单位份额推导 | 启动 |
| runtime.blocking_threads | u16 | 32 | 启动 |
| egress.allowlist | string 数组 | 空 | 启动 |
| egress.connect_timeout_ms、request_timeout_ms | u32 | 3000、15000 | 启动 |
| egress.ca_bundle_path | path | C:\EP\config\ca\esign-ca.pem | 取用 |
| egress.breaker.failure_threshold、open_ms、half_open_probes | u16、u32、u8 | 5、30000、1 | 启动 |
| spool.dir | path | C:\EP\<proc>\spool | 启动 |
| spool.max_bytes | u64 | 21474836480 | 启动 |
| portal.rate_limit_rps | u16 | 20 | 启动 |

不进配置文件的两类：一是运行期可变的业务参数，本阶段一条都不引入；二是机密，配置只写强类型引用。阶段 1 的 `FileSecretProvider`/直接文件 wiring 只属历史切片，当时没有形成可保留到生产的统一 provider 端口；阶段 2 必须在 `ep_foundation::port::secret` 新建 `SecretUnsealer`，在 `ep-platform-runtime` 新建 `SecretProvider/KmsSecretProvider`，生产配置从第一版可发布制品起只接受 `kms`。Stage 1 明文文件只能由签名 `ep-secretctl migrate` 在受控升级中读取，常驻二进制不链接该 reader。终态引用、recipient、bootstrap、信封、轮换与迁移按 ADR-0007；CI 断言 `SecretBytes|SecretString` 未实现 Clone、Debug、Display、Serialize，并断言配置结构体中任何名字含 password、secret、key、token 的字段只能是对应强类型 ref 或 secret wrapper。

上述 8000 ms/20-slot 只属于普通 HTTP route。F-55 的 AI compose 以编译期独立 45-slot、Tower 122000 ms/内部 120000 ms 运行；`POST /mcp` 以独立公平全局 16-slot→connector 4-slot、Tower 32000 ms/协议 30000 ms 运行；两者不读取、不占用或借用普通配置，其他 route 不得新增第三种例外。

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

全部集成测试使用真实 PostgreSQL 16，禁止内存库替代。每个用例独占一个数据库，命名 `ep_test_<nanoid>`，用例结束即删库；本平台不许 Linux 容器，testcontainers 一支随之不成立，实例一律取构建机与开发机上已安装的 Windows 版 PostgreSQL 16，由夹具连该实例建库与删库，不再有容器模式与复用本机实例模式两支。该实例的存在、服务端版本与建库参数是集成测试的外部前提，夹具启动时核对，不满足即明确失败，不静默跳过。

| 编号 | 场景 | 判定 |
|---|---|---|
| IT-01 | ep-migrate CLI 骨架 | apply、status、check、gen-rls、open-window 五个子命令的参数解析通过，`status --format=manifest` 输出制品清单；空库上 24 个 schema 与唯一 `platform_core.schema_history` 的存在性判定归阶段 2 的同名用例 |
| IT-02 | ep-migrate 退出码约定 | 0 成功、2 参数错误、3 迁移窗口未打开、4 校验和不符、5 版本不一致、78 环境自检失败六个分支各有一个用例 |
| IT-03 | 迁移清单哈希算法 | 归一化后的 SHA-256 与 build.rs 常量一致；篡改探针目录中的一个文件后比对失败 |
| IT-04 | 授权矩阵的判定路径 | 在测试库上以等价授权脚本验证 select/insert/update 允许、非基线第 3.6 节具名清理表上的 DELETE 被拒、具名清理表上的 DELETE 仅由 `ep_app_rw` 获得表级授权；生产库 24 个 schema 的默认权限矩阵归阶段 2 的同名用例 |
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
| IT-22 | 并发闸门 | 21 并发下第 21 个等待并在 10 秒后返回 CONCURRENCY_LIMIT；结构化拒绝日志存在，且不存在 `ep_quota_throttled_total` 指标 |
| IT-23 | 同步等待上限 | delay_ms 超过 8000 时返回 SYNC_TIMEOUT |
| IT-24 | panic 捕获 | 触发 panic 的探针路径返回 INTERNAL_ERROR 且进程仍存活 |
| IT-25 | 日志字段 | 每请求一条访问日志，17 个固定字段齐全，敏感值为掩码 |
| IT-26 | 指标端点 | 第 13 节新增决定五登记的五个指标名 `ep_build_info`、`ep_selfcheck_pending_items`、`ep_db_pool_connections`、`ep_db_statement_duration_seconds`、`ep_http_request_duration_seconds` 均存在，`ep_quota_throttled_total` 不存在；标签基数纪律断言（无 user_id、doc_no、trace_id 标签，route 为模板路径） |
| IT-27 | IPC | 帧往返、超长/未知 operation、DACL 封闭集、错账户×operation 全负例均通过；bootstrap 首实例以 true 创建、同 server 第二/补位实例以 false 成功，外部抢名失败，首句柄异常丢失导致整服务退出并由 SCM 重启；server 在读字节前冒充并以 thread token 核服务 SID、所有分支 RevertToSelf，PID 变化不影响授权；client 用 Identification SQOS 且每次重连重核 server token。三管道验证总实例 32/16/12、逐账户 cap 与保留接受槽；超 cap 在读 body 前返回 `PLATFORM.IPC.CONCURRENCY_LIMIT`，慢连接/半帧/断连与 5/10/30/120 秒普通超时、10/30/3600 秒流超时均清缓冲关闭；一账户占满不影响其他账户，`ERROR_PIPE_BUSY` 重试不落 spool |
| IT-28 | spool | core 不可用时两 writer 落盘、恢复后按发生时间幂等补写；五类 critical 在软/硬水位均零丢失，只有可重建进度可合并；达到软水位继续 WAL/当前写出但拒绝新备份与附件周期，写 Windows Event Log，恢复后不可抑制窗口开闭正确 |
| IT-29 | 优雅停机 | 服务停止控制码（控制台直跑模式下为 Ctrl+C 或 Ctrl+Break 事件）后在途请求完成、新请求被拒、退出码 0、drain 超时路径；退出码一项须在发停止命令前先 `OpenProcess` 持住句柄、停机后以 `GetExitCodeProcess` 断言，不以服务自报值单独判定。本用例只覆盖显式停止路径；机器关机路径的排空按第 7.6 节为做不到，不在本用例内，也不另设一条判不出结果的用例 |
| IT-30 | 探针表模板一致性 | RLS 模板生成器输出与黄金文件逐字节一致 |
| IT-31 | collation 一致性 | 判定位在 `check` 子命令中就位并有一个负样例夹具，判据为 `pg_database` 的 `datcollate` 与 `datctype` 均为 `C` 且 `datlocprovider` 为 `c`；对生产库的实际比对归阶段 2，因引导脚本由阶段 2 交付 |

#### 9.3 端到端用例

E2E 在单机编排上跑，覆盖规格第 17.2 章中本阶段可达的部分，不涉及业务闭环 14 步。

| 编号 | 场景 | 判定 |
|---|---|---|
| E2E-01 | 一条命令起全栈 | PostgreSQL 与八个进程共九个 Windows 服务经 `sc query` 全部为 RUNNING，九个健康端点全绿。`ep-migrate` 按裁定 F-08 做不到六不注册为 Windows 服务、不计入这九个，改由起栈脚本以其独立账户直接拉起并等其退出、按退出码原样判定，原「一次性迁移容器达到 active」半条随容器形态一并撤下 |
| E2E-02 | 全部进程 `--check` | 九份报告 overall 均为 PASSED 且退出码 0；构造任一 Degrading 项未通过时 overall 为 DEGRADED 且退出码非零 |
| E2E-03 | 迁移清单不一致时启动 | 自检项 migration-version-matched 失败，进程以 78 退出（该退出码经预先持有的进程句柄以 `GetExitCodeProcess` 断言，不取 `sc query` 的自报值）。首版不支持按退出码配置服务控制管理器的重启白名单或黑名单，故重启行为明确排除在本用例与产品承诺之外，不再作为待选分支；第十二节第 5 项只记录目标系统的实际行为证据，状态未形成时标 `UNVERIFIED`，不改变本条始终只判真实退出码的唯一实现口径 |
| E2E-04 | 配置未知键 | 以 78 退出（该退出码经预先持有的进程句柄以 `GetExitCodeProcess` 断言真实值，不取服务自报值；本阶段以控制台直跑模式起子进程取其退出码），键路径出现在本地日志文件中——服务控制管理器起的服务不继承控制台、stderr 无采集方，故 stderr 一项按做不到七改判日志落点 |
| E2E-05 | 资源限额取值 | `scripts/verify-resource-limits.ps1` 通过 Windows API 与 DACL 授权的 `JOB_OBJECT_QUERY` 读回具名 Job Object 的内存硬上限，与静态限额文件逐行一致；篡改一行必须失败。CPU 比例与磁盘 IO 份额无现行运行期承载，不冒充被测项；PostgreSQL/反向代理指派与 backup-writer 绝对 IO 上限按 F-08 首批实施门禁取证，未验证前本用例与 CI 第 11 阶段保持非零 |
| E2E-06 | 进程崩溃重启 | 强制终止 core-server 进程（本平台没有跨进程投递信号的机制，`kill -9` 无对应物，夹具取进程终止而不是停止请求）后，服务控制管理器按 `sc failure` 配置的恢复动作重启该服务并在 30 秒内重新就绪，其余八个服务不受影响。本条判的是管理器对「进程未报告 `SERVICE_STOPPED` 即消失」这一路径的恢复动作，与做不到五中按退出码取值的分流是两件事，本条不判分流 |
| E2E-07 | 优雅停机与整栈停止 | 九个服务全部退出码 0（经预先持有的进程句柄以 `GetExitCodeProcess` 断言，不取自报值），`sc query` 全部为 STOPPED 且无残留进程。原「无残留 socket」半条按裁定 F-08 第九节撤下：命名管道实例随最后一个句柄由内核回收，该判据在本平台恒真，按本卷先例恒真的门禁比没有门禁更坏，撤下而不换替身 |
| E2E-08 | 系统端点不外泄 | 经反向代理访问 `/api/v1/system/` 与 `/portal/v1/system/` 返回 404 或 403 |
| E2E-09 | 发布构建无探针 | 按发布 profile 构建并经安装包安装后，POST /api/v1/system/echo 返回 404 |
| E2E-10 | 制品验签 | 在断网 Windows 机器上执行 `verify-release.ps1`：生产制品 Authenticode、清单与证书来源记录全部通过；篡改一个字节或换成仅内部 ECDSA 签名后必须失败 |
| E2E-11 | 可复现构建 | 两次构建的八个 PE 二进制 SHA-256 全等；九个镜像 digest 一项随容器交付形态取消而撤下，不换等价物。PE 二进制能否稳定字节一致尚未在目标平台实测，实测结论出具前本用例与 CI 阶段 8 同批按未交付登记，照跑但不作为通过判据 |
| E2E-12 | 离线构建 | 断网环境下 `cargo build --locked --offline` 成功 |

#### 9.4 性能相关项

本阶段不参与规格第 16 章的通过线判定，只建立回归基线，写入 `docs/perf-baseline-stage1.md`，后续阶段以此比对是否劣化。

| 项 | 阶段 1 门槛 |
|---|---|
| CI 全量运行时长 | 不超过 60 分钟；增量不超过 25 分钟 |
| core-server 从进程启动到 ready | 不超过 3 秒，不含数据库启动 |
| 单个空壳进程常驻 RSS | 不超过 128 MiB |
| 单个进程 PE 二进制大小 | 不超过 40 MiB。被测对象由原 scratch 镜像换成 PE 二进制，门槛取值本轮沿用，随本节回归基线在目标平台首次实测后重取；安装包整体大小本轮不设门槛 |
| GET /api/v1/system/health 本机 P95 | 不超过 20 毫秒，200 次采样 |

#### 9.5 覆盖率门槛

本阶段即启用最终门槛，不设过渡期。A 档路径 `crates/foundation/` 行覆盖率不低于 85%；其余代码不低于 70%；新增与修改代码不低于 80%；工作区整体不低于 80%。骨架 crate 因无代码不计入分母。`#[ignore]` 必须带 issue 编号注释，`xtask codecheck` 断言其存活不超过一个阶段（以注释中的阶段编号判定）。

#### 9.6 与规格第 17.2 与 17.3 章判据的对应

本阶段能对应到的判据有四类，其余判据在本阶段只交付判定框架不交付判定内容，此点在退出条件中显式承认。

一是单元测试与领域属性测试的覆盖率通过标准，本阶段以分档门禁完整实现并生效。二是法人行级隔离与越权测试集。按 C-05，`tests/rls_matrix` 分三段，三个阶段不得重复实现同名函数。本阶段交付该 CI 目标与八类断言的骨架并在探针表上实测，函数名固定为 `assert_read`、`assert_write`、`assert_update`、`assert_delete`、`assert_aggregate`、`assert_sort`、`assert_report_projection`、`assert_error_leak`，位于 `testkit/src/rls_matrix.rs`，业务表进入后按同一套断言扩展。阶段 2 追加 `assert_replication_role_containment` 与 `assert_recon_context_borrow` 两个函数，前者是两个复制角色的入口借用遏制断言，后者是内部对账系统安全上下文的入口借用，本阶段不为二者建目标文件与失败占位。`assert_replication_role_containment` 是阶段 14 发布门禁项 `RG-RLS-MATRIX-GREEN` 的被测对象之一，本阶段无权单方撤销；规格第 7.7 章自认三项遏制手段都不阻止持有本机操作系统权限者切换到写出进程账户并从本机建立流复制连接这条路径，那是残余风险的如实披露、由规格第 21.21 章承载，不构成取消该断言的依据。阶段 4 追加 `matrix_32.rs` 的 32 组完整矩阵与发布门禁项 `RG-RLS-MATRIX-GREEN`。三是数据库适配认证套件中的迁移与锁两项的执行框架，本阶段固定迁移会话的 `lock_timeout = '5s'` 与 `statement_timeout = '30min'`，并提供在线变更耗时的实测夹具。四是混沌与故障注入六类中的进程崩溃后重启恢复一类，本阶段以 E2E-06 覆盖其可达部分，即重启后进程恢复与请求按第 15.1 章明确失败，未完成任务恢复与已确认事务零丢失属后续阶段。第 17.3 章的九项强制不变量在本阶段没有被测对象，一项都不声称通过。

### 10. 退出条件

下列每条都能由一条命令或一份自动产出的报告客观判定，全部达成才算本阶段完成。凡被测输入的交付阶段晚于本阶段的判据，按基线第 12 节通则第六条处置，不得以恒不可满足或恒真的形态留在本节。

1. `cargo build --workspace --locked --offline --release` 成功，零 warning，`-D warnings` 生效。
2. 每个 crate 的命名前缀、目录名与 `Cargo.toml` 中的 name 三处一致，由 archcheck 断言。不再断言 crate 清单与基线第 1.2 节逐项一致：逐项一致这条判据把 crate 边界变成必须走基线修订才能移动的冻结物，而真正要守的依赖方向由退出条件 3 的七条禁止项守住，foundation 一侧另由 `foundation-module-registry` 与 `foundation-no-single-owner` 两条规则单独接盘，新增 foundation 顶层模块必须同批改基线第 1.4 节；基线第 1.2 与 1.3 节的两张表相应降为现状记录，增删 crate 走普通评审。`codecov.toml` 的分档路径规则按目录前缀表达，不与 crate 清单逐项对应，新增 crate 不会静默逃出覆盖率分档。
3. 依赖方向的七条禁止项在 archcheck 上各有一个负样例，负样例构建必须失败，正样例全部通过；其中第六条 `foundation-no-business` 的机检面由 `foundation-no-business`、`foundation-frozen-items`、`foundation-marker-shape`、`foundation-module-registry`、`foundation-no-single-owner` 五条规则合成，负样例分别为 foundation 依赖工作区内任一 crate、`id::marker` 的 22 项标记类型改名或增删、`id::marker` 中的标记类型形态违反（出现字段、方法或 derive 的 trait 实现）、foundation 顶层模块清单与基线第 1.4 节的顶层模块登记表不符、在非 marker 文件中出现含模块码词元的条目（如 `pub struct SalesOrderDto` 或 `pub mod invoice;`），共五个，本条负样例总数因此由七个变为十一个；其必要性一条不由任何工具判定，也不产出负样例，理由与登记见基线第 12 节通则第六条与第 12.1 节。
4. 八个二进制启动、就绪、优雅停机、崩溃重启四条路径在 E2E 中全绿。
5. `ep-migrate` 的五个子命令 apply、status、check、gen-rls、open-window 参数解析齐备，六个退出码各有一个用例；迁移清单哈希在探针目录上比对通过且篡改后失败；`db/migrations/` 下 24 个空目录存在，且目录中不含任何顺序声明文件。空库上 24 个 schema 与 `platform_core.schema_history` 一张历史表的存在性判定归阶段 2。
6. 六项已实现自检各自的通过与失败分支均有集成测试，自检项一律以注册名标识且各自带 Blocking 或 Degrading 一个档位；三项 Pending 项 `secrets-resolvable`、`audit-chain-verifiable`、`file-store-writable` 在报告中如实标注，且有一条 CI 断言保证未注册项数量只减不增。
7. 十三条错误码在 `docs/error-codes.md` 与代码常量表中一致，重复码或缺失码即构建失败，其中 C-24 列明的七条由本阶段独家登记。
8. 第 13 节新增决定五登记的五个指标名在指标端点上可见，其中 `ep_db_pool_connections` 与 `ep_db_statement_duration_seconds` 按 C-23 本阶段只注册不填充，判据为指标名存在而非有非零样本；已撤销的 `ep_quota_throttled_total` 不存在，标签基数纪律断言通过。
9. 全部配置键在 `docs/config-reference.md` 中有条目，代码与文档逐键一致。
10. 结构门禁十一个子命令各自有负样例，负样例必须失败。
11. SQL 静态检查的全部规则各有负样例，至少覆盖 DELETE 封闭白名单、varchar 禁令、enum 禁令、current_date 禁令、跨 schema 外键形态、ON DELETE CASCADE 禁令、rollback 注释缺失、公共列缺失与顺序错误、命名规范违反、迁移单一职责违反十项。
12. 覆盖率四档全部达标，且有一个人为降低覆盖率的负样例使流水线变红。
13. 两次独立构建产出完全相同的八个 PE 二进制哈希；镜像 digest 一项随交付形态改为安装包加服务注册脚本而撤下，不换等价物。可复现构建实现路径固定为第 5.7 节算法，目标平台证据形成前能力状态为 `UNVERIFIED`，CI 阶段 8 与 E2E-11 保持非零且不得发布；这不是算法二选一或开发阻断，证据失败时仍按同一路径整改并重跑，只有八个哈希真实一致后本条才转为通过。
14. SBOM 生成成功，`cargo deny` 与依赖漏洞扫描零严重与高危，许可证清单通过。`ep-bench` 与 `ep-release-gate` 自本阶段起已是 workspace 中的非产品工具骨架：正向检查必须针对真实产品 SBOM 并断言两个包名均不存在；负向夹具人为注入任一包名并断言 `xtask sbom` 失败，判据与阶段 14 的 `RG-TOOLS-EXCLUDED` 相同。另分别调用两个骨架并断言均以 `EXIT_NOT_DELIVERED=70` 退出，禁止未实现主函数返回 0；阶段 14 完成真实功能后才允许成功命令返回 0。
15. 升级包结构完整，客户侧验签脚本在断网机器上通过，篡改后失败。
16. `deploy/` 下八个具名 Job Object 静态限额文件只承载已冻结的内存硬上限，不出现磁盘 IO 份额、内存软保底或 CPU 比例运行期取值；三类冒充各有负样例。`scripts/verify-resource-limits.ps1` 用 Windows API 与 DACL 读回并逐行比对，篡改即失败；`cargo xtask ci` 第 11 阶段只有在 Server 2022 主测与 Server 2019 同项复核证据齐全时才可返回 0。任何进程启动自检不重复资源限额核对。
17. 阶段 1 性能回归基线五项全部有实测记录并达标。
18. 六份文档骨架存在，ADR 至少含工具链冻结、collation 选型、构建目标与交付形态、CI 平台选型、新增 crate 五篇。其中构建目标与交付形态一篇取代原 musl 静态链接一篇：musl 静态链接随构建目标改为 `x86_64-pc-windows-msvc` 而作废，`docs/adr/ADR-0004-musl-static-linking.md` 按 ADR 惯例标为被取代而不删除，新篇记 Windows 构建目标、PE 二进制与安装包加服务注册脚本的交付形态。篇数仍为五篇。
19. 源码仓库、制品与离线依赖仓库的加密备份脚本可执行，且完成一次恢复验证并留下记录。
20. 本计划第 13 节列出的偏离与新增决定全部回写共享技术基线，回写内容经评审通过。
21. ep-foundation 的跨阶段冻结项齐备且逐项与裁定一致：`port::tx` 的 `Tx`、`SnapshotCtx`、`UnitOfWork` 三者与 `TxId`、`IsolationKind`、不透明且 Debug 脱敏的 `TransactionLockProof`，`id::marker` 的 22 项标记类型（由 archcheck 的 `foundation-frozen-items` 按名逐项断言，改名与增删同样报错），`principal` 的两个常量，`security::context` 的 20 个字段、四个配套枚举（含 `SystemPurpose`）与七个字段类型 `DeviceId`、`RoleCode`、`DutyClass`、`RecordShare`、`DataScopeTag`、`RequestId`、`TraceId` 及其配套枚举 `RecordShareGrant`，`ModuleCode` 15 项，`CapabilityDomain` 18 项与 `ActionClass` 5 项；`crates/foundation/src/port/search.rs`、`doc.rs`、`db.rs` 与 `kms.rs` 四个空模块文件存在（`port::kms` 按裁定 F-04 于本阶段建空模块，`KmsBackend` 由阶段 2 补齐）；`xtask archcheck` 在 `crates/`、`apps/`、`testkit/`、`datagen/`、`tools/` 五个扫描目录内断言 `downcast_mut::<PgTx>` 只出现在 `crates/adapter/db-pg/`，并以 `reconciliation-context-confined` 断言 `SystemPurpose::Reconciliation` 除定义处外只出现在 `crates/platform/recon/src/executor.rs`；`xtask/` 自身与仓库顶层文件不在扫描面内；`crates/foundation/src/lib.rs` 的顶层 `pub mod` 行与基线第 1.4 节的顶层模块登记表逐行相等，由 `foundation-module-registry` 读该表比对断言。`TransactionLockProof` 的必要性举证固定为 invoice、finance、procure、inventory 四个 contract crate 均需在不发生 contract→contract 依赖的前提下接收同一事务锁证明；稳定性举证固定为该类型不含业务枚举、规则方法或可序列化载荷，F-50 类别只存在于 owner 的 ledger contract。
22. `testkit/src/rls_matrix.rs` 的八个断言函数存在，函数名与 C-05 逐字一致，且在探针表上全绿；本阶段不实现阶段 2 与阶段 4 的追加函数。
23. `docs/data-dictionary.md` 的单据类型码一节存在，该节与 `ep-platform-sequence` 常量表逐项一致且无重复这一比对整条推迟到阶段 3a：该常量表由阶段 3a 交付，本阶段被测输入为空集，比对在本阶段恒真，按基线第 12 节通则第六条不得以恒真形态留在本节；`xtask configdoc --check-doc-type-codes` 的逐项比对自阶段 3a 起生效，本阶段只判该节存在。
24. `docs/metrics-catalog.md` 的指标名唯一性校验在 `xtask` 中实现并通过，`ep_build_info`、`ep_selfcheck_pending_items`、`ep_db_pool_connections`、`ep_db_statement_duration_seconds`、`ep_http_request_duration_seconds` 五个指标已注册，`ep_quota_throttled_total`、`ep_db_retries_total` 与 `ep_tx_retry_total` 三个撤销名不出现在任何登记文件与代码中。
25. T0 贯通线的判定手段就位：`xtask e2e --profile=t0` 作为独立目标可执行并返回 0，`ep-datagen` 的 `t0-min` 样本档在同一 seed 下两次生成字节一致，`deploy/` 一条命令起全栈。本阶段不提供 T0 的任何业务切片，也不为 T0 声称任何业务判据。
26. `xtask archcheck` 的 `unwired-absent` 规则就位并通过：`apps/core-server/src/wiring/` 与 `apps/job-worker/src/wiring/` 两个目录下的全部文件中不出现任何以 Noop、Stub、Fake、Dummy 为前缀的实现类型或注入行；该规则配一个故意违反的负样例，负样例构建必须失败。本条单列，不并入退出条件 3 的依赖方向七条禁止项。
27. `xtask archcheck` 的规则面与三态输出就位。已判定规则共 19 条：依赖方向的七条禁止项 `domain-no-cross-module`、`app-no-peer-app`、`domain-contract-no-io`、`platform-no-domain-or-app`、`adapter-no-peer-adapter`、`foundation-no-business`、`db-pg-one-schema-per-file`，加 `platform-acyclic`、`platform-no-adapter`、`postgres-driver-tooling-only`、`crate-naming-consistent`、`unwired-absent`、`downcast-pgtx-confined`、`foundation-frozen-items`、`foundation-module-registry`、`foundation-no-single-owner`、`foundation-marker-shape`、`undecidable-registry-matched`、`rule-roster-matched` 十二条。其中 `rule-roster-matched` 断言本条自身声称的条数与规则名与工具实际判定的一一相等——计数漂移在本项目已复发四次，该条把它变成可机检的。其中 `platform-no-adapter` 按裁定 F-04 新增，与 `platform-acyclic` 同为基线第 1.3 节允许项「`ep-platform-*` 只可依赖 `ep-foundation` 与其他 `ep-platform-*`」的机检面，各配一个故意违反的负样例；`postgres-driver-tooling-only` 为阶段 2 新增，断言裸驱动 tokio-postgres 只许出现在仓库目录 `tools/migrate` 的 `ep-migrate` Runner（refinery 版本空间冲突的处置经批准登记，见 ADR-0013 与 `history.rs` 模块头），同样不并入禁止项；`platform-no-adapter`、`postgres-driver-tooling-only`、`foundation-marker-shape` 与 `undecidable-registry-matched` 四条一律不并入禁止项，禁止项仍为七条且一字未改。退出码三态互不合并：机检面全绿为 0，有违反为 1，仍存在不可判定项为 3，CI 不得把 3 当作通过。基线第 1.3 节禁止项第六条的必要性一条由 delegated 段单列打印「不由本工具判定」并点名承接方，delegated 不参与退出码判定；本阶段 undecidable 段为空，因此本阶段 `cargo xtask archcheck` 的通过态退出码为 0。工具运行期输出的 delegated 与 undecidable 两段与基线第 12.1 节登记表逐行比对相等，多一条或少一条即本条不通过；undecidable 段的条目数按第 13 节假设二的同款纪律只减不增，并由阶段 14 的发布门禁项 `RG-NO-UNDECIDABLE` 断言归零。本条单列，不并入退出条件 3 的七条禁止项。

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
| 第 13.1 章 | 九行配额表中内存一列是本平台唯一有运行期承载的一列，八个自研二进制的对应行落为八个具名 Job Object（资源单位）的提交内存硬上限；CPU 一列首版固定只作硬件标定与认证意图声明，磁盘 IO 份额整列无运行期承载；PostgreSQL 16 与反向代理两行的唯一实现路径是由运维代理创建具名资源单位后指派，实机证据形成前状态为 `UNVERIFIED`；内置搜索索引一行在首版无独立进程与独立资源单位、不落静态限额文件，其缺口按第 5.6 节披露 | D-06、E2E-05 |
| 第 13.2 章 | 单机原生服务编排、同一份安装包加服务注册脚本、只依赖开放接口 | D-05、安装包与服务注册脚本清单 |
| 第 15.1 章 | 五类错误分类、四要素、存在性不泄漏、面向使用者文案 | 错误码表、IT-19 至 IT-21 |
| 第 16 章 | 20 并发上限的承载方式与连接池分池上限 | 第 7.2、7.4 节，IT-17、IT-22 |
| 第 17.1 章 | 本地 Git Monorepo 的目录结构、需求编号关联、角色分离与非自审的分支保护、仓库与制品加密备份 | 分支保护配置、CODEOWNERS、备份脚本与恢复记录 |
| 第 17.2 章 | 测试三层边界、覆盖率分档门槛、法人越权测试集骨架、进程崩溃重启一类 | 第 9 节全部 |
| 第 17.4 章 | SAST、依赖、安装包与密钥扫描、SBOM、制品签名、可复现构建、离线依赖仓库、客户侧验签 | 流水线阶段 7 与 8、E2E-10 至 E2E-12 |
| 第 18 章 | 单一版本线的版本号规则、升级包结构、回退说明、迁移逆向性标注 | D-11、迁移文件头 rollback 段 |
| 附录 A.3 | 基准数据集生成器的确定性与版本化骨架 | ep-datagen 与确定性用例 |
| 附录 D.2 | BC-1 基线组合的构建与运行形态落地 | 安装包与服务注册脚本 |

| PRD 节 | 本阶段实现的条目 |
|---|---|
| 0.6 维护纪律 | 错误码表、事件目录、指标目录、配置参考四份登记文件与 CI 一致性校验 |
| 11.2 并发与规模上限 | 20 并发闸门与超限时的可见后果 |
| 11.3 响应时延与等待反馈 | 8 秒同步等待上限与超时提示路径 |
| 11.9 降级状态的用户可见性 | 自检项 11 的 Degraded 状态表达与就绪端点上的降级标识 |
| 11.10 错误与失败提示 | 四要素齐备的错误封套与不泄漏内部信息的断言 |
| 附录乙 U-A-03 | 文本长度以 text 加 CHECK 表达的生成器与探针表示例 |
| 附录乙 U-A-05 | 分页、排序、导出上限五项常量在 foundation 中集中定义 |
| 附录乙 U-A-06 | 错误码编制与文案均以 `docs/error-codes.md` 当前登记为冻结值；每码必须具备关联编号、时间、可否重试与建议动作四要素 |

### 12. 风险与预留

#### 12.1 已知技术风险

R-01 撤销，本行原地留作撤销登记，编号不顺延。原条目是 musl 静态链接的内存分配性能，其成因随构建目标由 `x86_64-unknown-linux-musl` 改为 `x86_64-pc-windows-msvc` 一并消失；原缓解手段（引入 mimalloc 作为全局分配器）与原回退方案（改用 glibc 加固定 digest 的 distroless 基础镜像并重跑 E2E-11）随之一并撤销，本阶段不因本条要求任何全局分配器取值，也不再有基础镜像这一层。这是本次平台变更在本阶段唯一净减的一条风险，R-02 至 R-09 一条不动。

R-02，迁移工具的 tokio-postgres 与运行期 sqlx 双驱动共存。两套驱动的行为差异可能造成迁移与运行期对同一数据库错误的解释不一致；`tools/ep-migrate` 因自建 Runner 独占裸 tokio-postgres，八个运行期进程只用 sqlx，`postgres-driver-tooling-only` 断言该边界。迁移清单解析、校验和与历史表形态由共享 `migration_manifest` 库和 ADR-0013 的兼容测试锁定，不让两套驱动各自实现一遍。

R-03，可复现构建的偶发不一致。来源通常是 build.rs 引入时间或路径、并行编译顺序及过程宏的非确定输出。`cargo xtask ci` 第 8 阶段在 Windows agent 强制两次构建比对，失败时输出差异并记录已知来源；PE 实机证据尚未形成时该阶段保持非零，任何历史 TSV 状态不得折算成通过。

R-04，集成测试依赖构建机与开发机上已安装的 Windows 版 PostgreSQL 16 实例。本平台不许 Linux 容器，testcontainers 与 rootless podman 加 cgroup v2 委派一支整条不成立，原来的容器模式与复用本机实例模式两支退化为后一支；退化掉的是环境自带这一层——实例不再由夹具拉起，装没装、装的是哪个构建都变成外部前提。首版不依赖 ICU；夹具启动时必须核对 PostgreSQL 主版本与数据库 `libc/C/C` 建库参数，不满足即明确失败而不是静默跳过，建库与删库逻辑不变，用例仍各自独占一个库。

R-05，八个服务账户与文件对象的权限映射。用户命名空间与 rootless 容器的 UID 映射在本平台不存在，socket 属主一维随命名管道取代 Unix 域套接字而消失，被测对象由属主位换成 NTFS ACL：`%ProgramData%` 的继承 ACL 默认对本机 `BUILTIN\Users` 可读，而现有代码建 spool 与 IPC 目录时不设任何权限，Linux 侧靠父目录 mode 与 umask 兜住的那一层在本平台没有。缓解是安装器断继承并显式设 DACL、进程启动时核对目录 ACL 而不是只建不查；原交付的 UID 映射对照表撤销，改为一份目录与账户的 ACE 对照表进部署记录。虚拟账户能否加入本地组按裁定 F-08 登记为目标平台实测项，本阶段不依赖组这一层，逐账户列 ACE。

R-06，签名密钥在本阶段是软件密钥。正式签名要求硬件密码机与双人控制，本阶段用软件 ECDSA 密钥打通流程，存在把临时密钥误带入正式发布的风险。缓解是签名脚本对密钥来源做硬校验，非 HSM 来源的签名在制品元数据中标注 `signing_authority=dev` 并使发布流水线拒绝放行，只允许内部阶段制品使用。

R-07，复制槽的本机事务日志保留上限依赖实测。按 C-01，`max_slot_wal_keep_size` 的取值与其落地脚本 `db/bootstrap/02_cluster_params.sql` 均归阶段 2，取值按规格附录 A.3 的连续归档本机保留子项等量取 350GB，本阶段不自带取值，也不进本阶段的配置参考。缓解是规格第 7.3 章的两个断链用例实测本机 pg_wal 峰值并与该子项对照，实测速率使该取值不足以支撑部署记录约定的落点不可写时长时，按附录 A.3 同一构成上调该子项并重算容量下限，回填由归档阶段执行。

R-08，覆盖率门槛在骨架阶段可能诱发为覆盖而写的空测试。缓解是结构门禁与负样例制度并行，且 A 档只覆盖 foundation，其余按实际代码量分档。

R-09，CI 平台选型属本阶段新增决定，若客户或团队后续改用其他 CI，门禁逻辑不应绑定平台。缓解是全部门禁收敛到 `cargo xtask ci` 一个入口，CI 配置文件只负责调度，迁移平台的成本限于重写调度文件。

#### 12.2 为后续阶段预留的扩展点

自检注册表预留 `secrets-resolvable`、`audit-chain-verifiable`、`file-store-writable` 三项的注册位，注册函数签名与 severity 取值域已冻结，各阶段追加项一律以注册名加一个档位登记，不用序号。本阶段不预留任何返回成功的空实现：跨阶段端口按同批交付、整条推迟、改用降级窗口三者择一处置，本阶段一律取整条推迟；这一纪律由本阶段随 `xtask` 交付的 archcheck 规则 `unwired-absent` 在 `apps/core-server/src/wiring/` 与 `apps/job-worker/src/wiring/` 两个目录上强制，因此 `AuthnPort`、`AuthzPort`、`AuditSink`、`OutboxSink` 与 Outbox 消费的去重钩子在本阶段都不存在，由阶段 4 与阶段 3a 在交付判定与本体的同一批里引入。HTTP 中间件栈只留 `ep_foundation::port::db::IdempotencyStore` 一个注入点，按 C-07 其端口定义归阶段 2、存储与重放实现归阶段 3a，本阶段的 `IdempotencyKeyHeaderGuard` 只校验请求头，不需要任何桩。`db/migrations/` 下 24 个目录已列齐，迁移执行顺序由阶段 2 交付的单一全局 Runner 按文件版本号全序排定，业务阶段只按版本号加文件，不存在任何顺序声明文件要改。错误码表、事件目录、指标目录、数据字典的单据类型码一节四份登记内容已建立并被 CI 校验，后续阶段先登记后实现的纪律有强制点。`ep_foundation::port::{tx, db}` 与 `ep-adapter-db-pg` 的分层已就位，业务代码只依赖前者，多库延期不影响抽象层的稳定性。feature `ci-probe` 提供一个不进发布的探针通道，后续阶段可复用于横切链路验证。

### 13. 偏离基线与本阶段新增决定

按基线第 0 节与第 12 节的要求，本节单列全部偏离项与新增决定，每项给出理由与影响范围，并同步提出基线修订。本阶段不接受只在实现里偏离。

偏离一，新增 crate `ep-platform-runtime`。基线第 1.2 节的平台底座清单没有承载进程运行时装配的 crate，而基线第 7.3 节已把 `SelfCheckRegistry` 的落点写死在 `crates/platform/runtime/src/selfcheck/registry.rs`，两处自相矛盾。若不新增，配置加载、自检注册表、信号处理、生命周期状态机、健康与就绪端点这一整套代码要在八个二进制里各写一份，与文件规模纪律和单一事实源冲突。该 crate 只承载进程生命周期状态机、分层配置加载、`SelfCheckRegistry`、健康与就绪端点、HTTP 服务器与中间件栈骨架，以及以 trait 表达的 IPC 服务端接口；HTTP 服务器骨架直接构建在第三方 HTTP 库之上，工作区内既不存在也不新增任何 HTTP 系 `ep-adapter-*`；IPC 的具体传输实现仍留在 `ep-adapter-ipc`，由 apps 在 `apps/<proc>/src/wiring/` 目录下注入，因此本 crate 只依赖 foundation 与其他 platform，apps 依赖它，不改变任何既有依赖方向。影响范围只有一处：基线第 1.2 节的平台底座表增加 `ep-platform-runtime` 一行，职责列取上句。该表不补冻结措辞，也不再作为 archcheck 的比对面，理由见第 10 节退出条件 2。
偏离二，`ep-foundation` 承载 22 个实体标记类型。按 A-01，`crates/foundation/src/id/marker.rs` 集中声明跨模块被引用实体的零大小标记类型，清单固定 22 项。这是对基线第 1.3 节禁止 foundation 承载业务概念一条的一处受限例外，标记类型无字段、无方法、无 trait 实现，只承载类型身份，供 `Id<T>` 在契约层表达跨模块引用。不采用该例外的代价是每个 ep-contract crate 各自声明一份标记类型，同一实体在不同 crate 中的 `Id<T>` 互不相容，跨模块方法签名无法表达。影响范围已落在基线第 1.3 节第六条，该条写明本例外不适用两条准入判据、清单冻结 22 项、按名断言。

偏离三，第 3.1 节三处落点中的第二处在本阶段内改过一次位置。原落点是 `ep-adapter-db` 提供 `PgUnitOfWork` 与 `PgTx` 两个实现类型的声明位、实现体落在 `ep-adapter-db-pg`，该形态在 Rust 中不可实现：`Tx` 属 `ep-foundation`、`PgTx` 属 `ep-adapter-db`，对 `ep-adapter-db-pg` 双双是外部类型，`impl Tx for PgTx` 触发 E0117 孤儿规则；同一形态还要求 `ep-adapter-db-pg` 依赖 `ep-adapter-db`，与基线第 1.3 节禁止项第五条互斥。按裁定 F-01，crate `ep-adapter-db` 整个撤销，需要被 platform、contract、domain、application 命名的端口下沉为 `ep-foundation` 的 `port::db` 模块，`PgUnitOfWork` 与 `PgTx` 的声明与实现一律同处 `ep-adapter-db-pg`，工作区内 db 系 adapter 只剩 `ep-adapter-db-pg` 一个。本条登记的就是这一次位置变更：第 3.1 节「后续阶段只补内容不改位置」自本条起以新落点为准，旧落点不再是不可动的既定位置。影响范围有两处：基线第 1.2 节删去 `ep-adapter-db` 一行、`ep-foundation` 一行的职责描述增加 `port::db` 端口模块；基线第 1.4 节配套纪律第四条改为凡实现 `ep_foundation::port::*` 各模块中任一 trait 的具体类型，其声明位与实现位一律同处一个 crate，不得分离。该条按裁定 F-04 一次扩面到全部端口模块，既覆盖 `port::tx` 的 `Tx`、`SnapshotCtx`、`UnitOfWork`，也覆盖 `port::kms` 的 `KmsBackend`，其载体实现 `BuiltinKmsBackend` 与 `HsmKmsBackend` 的声明与实现同在 `ep-adapter-kms`；本句在技术基线第 1.4 节、本节与总览的 F-01 登记行三处逐字复述，扩面须三处同批改到位，不得留下宽窄两套。

新增决定一，新增五个非产品或运维用途的 workspace 成员：`xtask` 是纯开发期工具，不进任何制品；`tools/ep-migrate` 与签名 `tools/ep-secretctl` 是一次性运维工具，随制品交付；`tools/bench` 与 `tools/release-gate` 是不随产品交付的认证/发布工具骨架，自本阶段存在但到阶段 14 才完成，未交付期间固定退出 70。`ep-secretctl` 顶层命令闭集、明文输入与 WinCred 服务 current-token 协议以 ADR-0007 和配置参考第 5 节为准，进入 SBOM 但不注册服务、不监听端口、不连数据库。本平台的服务控制管理器没有起来做完就退出且算成功的服务类型，也没有 `RemainAfterExit`，因此 ep-migrate 不注册为 Windows 服务，由升级脚本在升级窗口内以 `ep-migrate` 账户直接拉起并等其退出，退出码原样判定。原以 systemd 的 oneshot 单元执行一句随平台变更作废，结论不变、理由换：它仍不常驻、不注册为 Windows 服务、不占用任何资源单位。五者都不是常驻进程，不监听端口，不属于八进程清单，不改变基线第 2 节；ep-migrate 与 ep-secretctl 随产品制品交付。运行 `ep-migrate` 的操作系统账户为 `ep-migrate`，按裁定 F-08 第 4.2 节保留为一个普通本地账户，与八个服务账户互不复用；组 `ep` 这一层在本平台不要，账户之间的隔离由逐账户列 ACE 的 DACL 表达，不由组权限位表达。影响范围有两处：基线第 1.1 节的目录布局改为两段，第一段是 workspace 成员路径，在既有八条之外增加 `/xtask/` 与 `/tools/<name>/`，第二段是非 workspace 成员的仓库目录，列 `/db/bootstrap/`、`/db/checks/`、`/deploy/`、`/scripts/`、`/clients/desktop/`、`/clients/mobile/` 六条，并在节末写明本两段即全部顶层目录，新增顶层目录必须先改本节；基线第 2 节的账户说明增加一条，`ep-migrate` 为普通本地账户，与八个服务账户互不复用，不设组 `ep`。

F-56 后续只给既有 `apply` 增加 `--initial-governance-bootstrap/--initial-license-package/--receipt-out` 三个必须同时出现的 fresh-production 参数；它在全迁移完成且常驻服务尚未 readiness 时执行一次性双签首装事务，不增加第六个子命令、退出码、服务、监听或常驻资源。strict body、无回显 console password、一次性前置、崩溃仅补 receipt 与禁止 `--force` 全部只取 F-56，本阶段不造临时实现。

新增决定二，数据库建库参数固定为 `LOCALE_PROVIDER libc`、`LC_COLLATE 'C'` 与 `LC_CTYPE 'C'`，默认排序严格取 UTF-8 字节序并删除 public schema；首版不依赖 ICU 构建，也不保留 ICU 可用性二选一。理由是 C 排序不随操作系统 locale 或 ICU 版本变化，B-tree 索引不会因语言库升级静默改变顺序；需要中文阅读序的列表由应用层生成并持久化显式 `sort_key`，不得依赖隐式数据库 collation。按 C-01，阶段 2 的 `db/bootstrap/00_database.sql` 唯一建库语句为 `CREATE DATABASE ep ENCODING 'UTF8' LOCALE_PROVIDER libc LC_COLLATE 'C' LC_CTYPE 'C' TEMPLATE template0`，随后 `DROP SCHEMA public`；验收断言 `datlocprovider='c'`、`datcollate='C'`、`datctype='C'`、`daticulocale IS NULL`。本段与 ADR-0003 是决定出处，脚本不得反向改值。

新增决定三，`shared_preload_libraries` 开启 `pg_stat_statements`。理由是附录 A.1 的查询证据与慢查询定位需要，开销可控。按 C-01，落地由阶段 2 的集群参数脚本交付。影响范围是基线第 3 节增加一条数据库实例参数。

新增决定四，运行期账号的默认表权限不含 DELETE；只对技术基线第 3.6 节逐表具名的清理白名单授予 `ep_app_rw` 表级 DELETE，绝不按 schema 整体授权。按 C-01，该决定连同其引导与迁移落点一并移交阶段 2，由阶段 2 负责默认权限；各白名单表的创建阶段在建表迁移中追加精确表级授权。此处只保留决定来源与理由：这是把软删除口径从 CI 静态检查升级为数据库强制，同时让已冻结的保留期任务可执行；未列名表仍由数据库权限与 SQL 静态检查共同拒绝。

新增决定五，本阶段注册五个指标：`ep_build_info`（gauge，标签为 version 与 git_commit）、`ep_selfcheck_pending_items`（gauge，标签为 process）、`ep_db_pool_connections`（gauge，标签为 pool）、`ep_db_statement_duration_seconds`（histogram，标签为 pool 与 statement_kind）、`ep_http_request_duration_seconds`（histogram，桶为 0.05、0.1、0.25、0.5、1、2、3、5、10、30，标签为 route、method、status_class、client）。五者均在 `crates/platform/obs/src/metrics/registry.rs` 中由本阶段一次性注册，其中 `ep_db_pool_connections` 与 `ep_db_statement_duration_seconds` 按 C-23 由本阶段注册、由阶段 2 填充，其余三个由本阶段注册并填充，`ep_http_request_duration_seconds` 在本阶段的 HTTP 中间件栈中填充。旧 `ep_quota_throttled_total` 已撤销且不得注册或填充；并发闸门只返回登记错误码并记录结构化日志。按 C-21，事务重试指标统一为 `ep_db_tx_retries_total`，注册与填充均归阶段 2，本阶段原拟的 `ep_db_retries_total` 登记撤销。`docs/metrics-catalog.md` 的指标名唯一性校验在本阶段的 `xtask` 中实现。五者均不违反标签基数纪律。影响范围是基线第 9.2 节的基线指标清单只增加 `ep_build_info` 与 `ep_selfcheck_pending_items` 两项，另三项已在该节清单内，重复登记即构建失败，其注册方与填充方在 `docs/metrics-catalog.md` 中登记。

新增决定六，关联编号 `incident_no` 在没有共享序列的阶段以进程序号分段生成，格式与基线第 5.2 节示例一致。影响范围是基线第 5.2 节增加一条生成口径注记，并注明后续可替换为数据库序列而格式不变。

新增决定七，构建目标固定 `x86_64-pc-windows-msvc`，只此一个三元组、不做双目标，时区数据经 chrono-tz 编译进二进制，出网 TLS 用 rustls 并以配置指定 CA 文件。原取值 `x86_64-unknown-linux-musl` 静态链接与 scratch 运行基础镜像两项一并撤销：服务端交付目标改为 Windows Server 原生后不再有 Linux 构建目标，也不再有容器基础镜像这一层，首版不投入任何 Linux 侧构建工时。影响范围是基线新增一节交付形态取值，取值为八个 PE 二进制随同一份安装包（MSI 或压缩包）加服务注册脚本交付、同一制品覆盖 Windows Server 2019 至 2022 两个版本，与规格第 13.2 章现文一致；原与规格第 13.2 章 OCI 容器要求一致一句随该章改写作废。可复现构建算法固定见第 5.7 节，PE 字节一致性的能力状态在实机证据形成前为 `UNVERIFIED`。

新增决定八，CI 平台取内网自建 Forgejo 加 Woodpecker Windows agent，全部门禁收敛到 `cargo xtask ci` 一个入口。该命令是阶段集合、顺序、失败分类与聚合退出码的唯一真值；Woodpecker 与任何备用平台配置都只是薄适配器，不得直接编排子门禁。该取值不进入产品制品。Windows agent 与 17 项有效实机门禁尚未在本文形成通过证据，未验证项必须保持非零，不得由 `.github/` 历史脚本或登记状态折算为通过；原编号 12 已撤销，18 个历史编号只用于追溯。
新增决定九，`ep-foundation` 的职责扩展。按 A-01、A-02、A-03、A-07、A-08 与 A-20，本阶段在 ep-foundation 中新增 `port::tx`、`id::marker`、`principal`、`security::context`、`capability` 五个模块，并建 `port::search`、`port::doc`、`port::db` 与 `port::kms` 四个空模块，其中 `port::db` 按裁定 F-01 承接原 ep-adapter-db 的端口 trait 与能力描述，`port::kms` 按裁定 F-04 承接 `KmsBackend` 与其调用词汇类型、由阶段 2 补齐。理由是这五类东西被三个以上阶段的契约层同时引用，若不前移，跨模块方法签名无法在契约层表达，系统主体与能力域码会在各阶段各写一份。影响范围有四处：基线第 1.2 节 ep-foundation 一行的职责描述增加 Tx、UnitOfWork、SnapshotCtx、id::marker、capability、port::search、port::doc、port::db、port::kms 九项；基线第 4 节公共列表 created_by 一行的语义列写入 `00000000-0000-7000-8000-000000000001` 字面量；基线第 10.3 节在事务写法示例之后追加一句，只读快照事务的唯一入口是 `snapshot_transact`，配合 `SET TRANSACTION SNAPSHOT` 使用；基线第 12 节增加一条纪律，各阶段按裁定 A-20 的两类落点声明能力域码与动作类别常量，即业务模块的路由落 `crates/contract/<module>/src/capability.rs`，`/api/v1/platform/` 下的平台路由落 A-20 逐阶段指名的 platform crate 的 `src/capability.rs` 并一律取 `CapabilityDomain::PlatformAdminLowcodeOps`，`ci-probe` 门控的探针路由与 `/internal/v1/` 端点不参与判定也不声明常量，`xtask configdoc` 只断言每个 `/api/v1/` 路由。

新增决定十，启动自检项按注册名标识。按 C-25，自检项不再用序号称呼，注册表为 `SelfCheckRegistry`，注册项为 `SelfCheckItem { name, title, severity, run }`，name 为 kebab-case，基线十项的名字与档位见第 5.5 节，各阶段追加项按其阶段计划登记。理由是序号在多阶段追加时必然冲突，且已经出现同一序号在不同阶段指向不同项的情况。影响范围是基线第 7.3 节由编号列表改为命名列表。

新增决定十一，单据类型码的全局唯一登记表。按 C-26，`docs/data-dictionary.md` 增加单据类型码一节，本阶段建立该节与 CI 校验 `xtask configdoc --check-doc-type-codes`，判据为该节与 `ep-platform-sequence` 的常量表逐项一致且无重复，而该常量表由阶段 3a 交付，故按基线第 12 节通则第六条该比对整条推迟到阶段 3a，本阶段只判该节存在；各类型码由其单据所在阶段登记，任何阶段不得新增未在该节登记的码。影响范围是基线第 11.1 节增加档案编码格式与类型码登记表的指引。

新增决定十二，`SecurityContext` 七个字段类型的形态。基线第 1.4 节的字段表只给出 `DeviceId`、`RoleCode`、`DutyClass`、`RecordShare`、`DataScopeTag`、`RequestId`、`TraceId` 七个类型名，其后的配套枚举一段只冻结了 `AccountKind`、`ClientKind`、`DepartmentScope` 三个枚举，七个类型的形态在规格、PRD、基线与裁定表中均无定义，而按 A-03 其交付方同为本阶段，不给形态则该结构体写不出可编译的定义。取值见第 5.1 节。理由与代价：`DutyClass` 的六个取值与阶段 4 的 `platform_authz.roles.duty_class` 列取值同源，互斥关系属该阶段的职责分离种子规则，不进枚举定义；`RecordShare` 只表达一条记录被显式共享给当前主体，不承载判定，`RecordScope` 与 `RecordPredicate` 留在 ep-platform-authz，否则判定语义前移进 foundation 会与基线第 1.3 节的分层冲突；`TraceId` 与 `RequestId` 的形态在基线中原本只有日志样例与请求头描述，本决定把它们写成唯一形态定义。影响范围有两处：基线第 1.4 节的配套枚举一段由三个枚举扩为三个枚举加上述七个字段类型与 `RecordShareGrant`；基线第 5.6 节的请求头一节写入 `X-Request-Id` 与 `X-Device-Id` 的形态，与本决定逐字一致。
新增决定十三，启动自检分两档并删除三项。`SelfCheckItem` 的 severity 取值域定死为 Blocking 与 Degrading 两值，第 5.4 节状态机的守卫由点名 `offsite-sink-requirements` 改为按档位判定，并写死一条禁令：任何阶段不得注册判读业务数据行的 Blocking 项。基线第 7.3 节的十三项删去三项，余十项。删 `license-and-modules-consistent`，理由是规格第 3.4 章明写平台不因许可状态停机、用量超上限不阻断业务、身份四项处置在任何许可状态下均可用，而以退出码 78 拒绝启动使规格设计的受限运行态整个不可达，承接方是规格第 3.4 章已有的四态机与阶段 3b 的 `ModuleLicenseQuery`；裁定 A-05 中阶段 1 登记 Pending 一句随之作废，按权威顺序规格高于裁定表。删 `current-period-open`，理由是该项缺失时按规格第 5.2 章自动建立期间，那是一次写操作而不是闸门，八个进程还会在自检阶段并发写 ledger 表；承接方定死为阶段 9a 的 `AccountingPeriodResolver::resolve` 第二步的零期间分支，即该法人 `ledger.accounting_periods` 无任何行时按 posting_date 所属自然月建立该期间并置 OPEN，在首次过账的同一业务事务内完成，该分支属阶段 9a 交付并落在阶段 9a 的 T0 切片内，本阶段不为该项保留任何注册位。删 `cgroup-quota-matched`，理由与承接方见新增决定十四。`audit-chain-verifiable` 与 `offsite-sink-requirements` 两项定为 Degrading，理由是拒绝启动既修不好断链也补不上落点，而修复的唯一手段恰恰是人工介入，拒绝启动只会让这台没有备节点的服务器在最需要人操作的时候整体停摆。配置键 `selfcheck.pending_as_failure` 一并删除，Pending 一律不阻止启动，见假设二。影响范围是基线第 7.3 节由十三项编号列表改为十项命名列表并各带一个档位，且删去其中十三项为全部进程共有一句，改为不建库连接的四个进程对 SQL 类自检项一律标注 NotApplicable。

新增决定十四，删除 cgroup 配额生成器与配额清单，资源限额改为静态限额文件加一次性部署校验；承载物固定为具名 Job Object。内存硬上限保留并落 `JOB_OBJECT_LIMIT_JOB_MEMORY`，是配额表唯一运行期列；`MemoryLow` 与 `IOWeight` 整列删除；CPU 比例首版固定不启用，不保留实测后自动重开分支。八个自研二进制由服务宿主在 `ServiceMain` 早期自我指派；PostgreSQL 16 与反向代理由 ops-agent 创建资源单位后调用 `AssignProcessToJobObject`；backup-writer 绝对 IO 上限落静态限额文件、部署记录与 Windows 读回夹具，按补裁乙不进配额表。后两项的实现路径已冻结，实机证据形成前状态为 `UNVERIFIED`，CI 第 11 阶段保持非零；核对只由 `scripts/verify-resource-limits.ps1` 在部署与升级时使用 Windows API、DACL 与具名 Job Object 完成，不读取历史 TSV。内置搜索索引无独立资源单位、突发上限折算无承载、CPU/IO 比例不支持三项均按第 5.6 节披露。影响范围仍为基线第 2 节的资源单位说明、删除 `quotas.generated.toml` 落点与删除 `selfcheck.quota_manifest_path` 配置键。

现行修正（覆盖上一段中的历史实现句）：部署核对文件固定为 `scripts/verify-resource-limits.ps1`，只使用 Windows API、DACL 与具名 Job Object；唯一 CI 状态来自 `cargo xtask ci` 第 11 阶段，不读取 `.github/ci/pipeline-stages.tsv`。F-08 的第 2、3、4、15 至 18 项均为首批实施证据门禁；未验证前使用第 5.6 节冻结的保守状态并保持非零，不再作为设计二选一。

假设一，工具链版本。`rust-toolchain.toml` 的取值在本阶段首日由构建负责人按当日最新 stable 冻结并写入 ADR-0002，本计划以 1.86.0 表述仅为占位。冻结后不得单独升级，升级需另起变更并重跑可复现构建证据。这是假设而非既定事实，理由是版本号取决于冻结当日的上游发布状态。

假设二，本阶段允许 Pending 自检项存在且不阻止启动。规格第 7.3 节要求自检项失败即退出，但未规定尚未实现的项如何处置。本阶段把它定死为固定行为而不是开关：Pending 在报告中如实标注，不计入 overall 的成败，也不阻止启动，`selfcheck.pending_as_failure` 这个配置键随之删除，理由是把它置真会让阶段 1 至 13 的任何一个进程都起不来，它没有真实的取用者。一条 CI 断言保证 Pending 数量只减不增、在最后一个阶段归零。同一纪律同样适用于基线第 12.1 节 undecidable 段的条目，两者共用只减不增与最后一个阶段归零这一形态，不另立第二套。该假设一旦被认为不可接受，替代方案是让八个进程在阶段 1 就以 Degraded 启动，代价是降级状态在整个建设期一直为真，会淹没规格第 15.3 章的真实降级信号，因此不采用。

被阻塞情况的说明：本阶段不被任何业务决策阻塞。U-A-06 的正式简体中文文案已由 F-51 冻结，U-A-01 的编号规则由后续所属阶段的现行冻结值承接，U-A-03 与 U-A-05 已由基线第 11.2 与 11.5 节给出唯一值。本阶段的 `RecordShare` 只表达显式共享这一种来源；F-51 U-B-07 已冻结默认可见来源为责任人、当前流程处理人和显式共享，创建人不因创建永久可见，共享不可再转授。阶段 4 `ScopeCompiler` 必须按该三类并集实现，不得保留另一套候选分支。
