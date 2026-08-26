> **F-57 状态：`SUPERSEDED_DO_NOT_EXECUTE`。** 本文只保留历史任务正文；旧 **F57-08/09/10/16/17/18** 仅是需求所有权桶，不是现行 task 或执行顺序。当前权威集合为 [F-57 总体设计](../../specs/2026-08-23-f57-governed-automation-fabric-design.md)、[F-57 需求追踪矩阵](../../reviews/2026-08-23-f57-requirements-traceability.md)、[F-57 权威替代登记](../../reviews/2026-08-23-f57-authority-supersession-register.md)与 [2026-08-24 收敛实施主计划](../2026-08-24-f57-converged-program.md)；F-57 是设计/计划权威，不是产品已实现声明，本地模型实现延期。

## 阶段 13：四端客户端与低代码

> **F-55 后续扩展。** 本文件继续交付四端与 72 格基础矩阵；`13c-local-ai-mcp-server-admin.md` 在其后同批增补本地 AI、双向 MCP、ServerAdmin 第五列 18 格与客户自控境内 IaaS carrier，终态恰好 18×5=90 格、九个产品常驻进程。本文冲突的八进程/72 格“终态”措辞只描述 13a/13b 阶段切片。
>
> **F-52 开发冻结。** 阶段 13b 自动测试固定为九套；中立 SPI 归 `ep-platform-release`，四个属主 crate 实现，job-worker 经数据库租约队列执行。`config_packages` 是耐久派发载体，不新增事件或外部队列。本文旧“八套”或“执行落点待决”均已被替代，不再阻塞开发。
>
> **F-51 首版值冻结。** 本阶段原以“临时取值”登记的 U-K-01 至 U-K-08、U-L-06 与 U-A-05 均已按“全部采用推荐项”升级为首版规范值。以下只保留未来变更代价，不再表示待决或允许开发者二次选型。

本阶段承接规格第 6 章客户端与用户体验、第 7.4 章可定制数据库、第 9 章低代码与规则与模块发布、第 3.1 章与第 3.6 章白标与分发、第 12.4 章按端的数据保护控制，以及 PRD 第 8.4 节、第 10.4 节、第 10.7 节。本阶段不产生任何会计分录，也不新增任何账务口径，涉及账务的一律指向规格第 5.2 章事件-分录表，由财务阶段承担。
本阶段的交付边界按裁定 A-23 收窄：本阶段不交付任何业务界面；各业务模块的四端界面位于 `clients/desktop/src/modules/<module>/` 与 `clients/mobile/src/modules/<module>/`，由阶段 5 至阶段 12 各自交付，四端验收矩阵由阶段 14 汇总。

本阶段正式拆成两条可独立开工与验收的工作线，不再把“1 至 12 全部完成”作为统一开工前提：

| 工作线 | 硬依赖 | 内容与时点 |
|---|---|---|
| 13a 客户端、白标与制品 | 阶段 1 | 阶段 1 结束即启动客户端 Rust 核心、桌面/移动壳、白标与制品链。移动薄 PoC 前移到业务移动界面大规模投入之前；T0 所需桌面壳切片仍与阶段 3b-1、阶段 4 的能力闸接线同批完成 |
| 13b 低代码与配置发布 | 阶段 3b、阶段 11 | 在阶段 3b 最小发布通道与阶段 11 四个报表类 applier 到位后完成建模、DDL、发布回退、WASM 宿主与配置证据；可与阶段 9b 并行 |

13a 的薄 PoC 固定测试移动端切栈触发项：冷启动、列表滚动、中文输入三项完整判，交互时延只作取样，无障碍留第二批完整门槛表。任一可判阈值失败，只把移动 UI 从 Tauri 替换为 Flutter；客户端 Rust 核心九个 crate、服务端 Rust 核心、协议与数据模型不变。薄 PoC 只能产出切 Flutter 的否定结论，保留 Tauri 的肯定结论须待第二批完整门槛表通过或取得书面豁免。

本计划遵守共享技术基线。凡基线已定死的取值直接引用，不重新决定。本阶段新增的决定在第 12 节集中列出，需在阶段结束时回写基线。

---

### 1. 交付物清单

本阶段结束时，下列各项存在且可运行。

#### 1.1 服务端可运行物

1. core-server 内新增的低代码建模 API、扩展登记 API、客户端引导 API 与能力闸中间件，以及在阶段 3b 最小发布通道之上扩展的配置发布 API，全部经 `/api/v1/platform/...` 暴露，可用 `curl` 完成第 5 节全部端点的往返。
2. job-worker 内在阶段 3b 发布执行器之上扩展的 DDL 段编排、在线 DDL 执行器、九套配置自动测试执行器、派生存储重新打标任务、扩展自动停用巡检；发布传播仍由 Outbox 驱动，自动测试则只从 `config_packages` 的耐久租约行领取，不新增事件，可跑通一次含 DDL 的发布与一次回退。
3. plugin-host 进程从空壳变为可用宿主：可加载签名 WASM Component、按能力清单裁剪输入、按资源限额中止执行、把结果经 `\\.\pipe\ep-plugin` 命名管道返回给 core-server 与 job-worker，且该进程的数据库连接数恒为 0。
4. `platform_meta` 下 19 张新表与其迁移文件、回退说明、种子数据（先建立四端基础矩阵 18×4=72 行；阶段 13c 以 F-55 迁移追加 ServerAdmin 18 行并形成 90 格终态），以及对阶段 3b 已建的 `config_packages`、`config_package_items`、`config_release_orders` 三张表的列扩展与状态扩展。
5. `docs/error-codes.md` 新增 31 条具名错误码、`docs/event-catalog.md` 登记本阶段的具名事件类型、`docs/data-dictionary.md` 新增 19 张表条目，三处与代码常量表由 CI 校验一致。原“37 条”是未给出字面名的配额，已撤销；首版只实现本文与唯一登记表逐字可对账的 31 条。本阶段引用但由阶段 1 登记的 `PLATFORM.CONCURRENCY.STALE_VERSION`、`PLATFORM.AUTHZ.NOT_FOUND_OR_DENIED` 与 `PLATFORM.DB.MIGRATION_WINDOW_CLOSED` 三条不计入本阶段条数，见裁定 C-24。

#### 1.2 客户端可运行物

6. Windows 客户端安装包（msi 与便携 exe 各一）、macOS 客户端安装包（dmg，已签名并公证）、iOS 客户端（ipa，企业签名通道与商店通道各一份配置）、Android 客户端（aab 与 apk）。四份制品均由同一份 `brand.toml` 驱动产出，均可完成登录、设备登记、按能力矩阵渲染入口、离线草稿保存与恢复后提交。
7. 桌面端两个签名原生插件：打印机插件与 USB Key/智能卡插件，各自以独立子进程运行，含能力清单与本地登记表。
8. 客户端 Rust 核心 crate 集合（第 2 节列出），含本地加密缓存库、增量同步、声明式规则解释器、原生插件宿主、统一网络出口与证书链校验。
9. 四端共用的 React/TypeScript 界面壳与共用组件包，含浅色、深色、高对比度三套主题，全键盘与命令面板，WCAG AA 自动检查零严重问题，以及按能力矩阵裁剪入口的路由注册表。按裁定 A-23，本包不含任何业务模块界面，业务模块界面由阶段 5 至阶段 12 交付到 `clients/desktop/src/modules/<module>/` 与 `clients/mobile/src/modules/<module>/`。

#### 1.3 工具与流水线

10. `tools/epcfg`：配置包的打包、差异、签名、验签与离线导出导入命令行工具，可在开发与测试空间使用，也可在生产侧做只读校验。
11. `tools/epbrand`：白标构建流水线入口，输入 `brand.toml` 与源码 commit，输出四端未签名制品与其哈希清单，再调用签名步骤产出签名制品；同一输入两次构建的未签名制品哈希一致。
12. `tools/epplug`：WASM 插件 SDK 与打包签名工具，含一个示例插件（税额校验计算）与其能力清单。
13. 商店政策合规检查门禁脚本，四项检查项按规格第 3.6 章逐项判定，未通过即中断流水线。

#### 1.4 证据物

14. 13a 薄 PoC 报告与第二批附录 C.2 十二项门槛在 C.1 设备基线上的完整复测报告，均含全部原始测量数据；两批证据不得合并成一次并据此提前判定保留 Tauri。
15. 规格第 6.2 章能力等价矩阵 18 行、豁免清单每条替代路径的逐条核对表，含四端 E2E 执行证据；按裁定 A-23，业务闭环类用例的执行证据由阶段 5 至阶段 12 提交，本阶段只汇总，不自行执行。
16. 一次完整的配置发布与回退演练证据包：含差异审查记录、自动测试报告、审批与签名记录、执行耗时、锁持有时长、回退结果与审计链验证结论。

#### 1.5 与贯通线 T0 的关系

本阶段向 T0 贡献一个桌面壳最小切片，内容固定为下列五项，不含其余任何交付物：`/clients/desktop` 的 Tauri 2 桌面壳与其 Tauri IPC 命令表；`/clients/ui` 的路由注册表与浅色一套主题；`ep-client-core` 的会话、安全上下文、统一网络出口与证书链校验；core-server 的能力闸中间件，其判据取二进制内置的能力矩阵冻结快照，不读 `platform_meta.client_capability_values`；`GET /api/v1/platform/client-bootstrap` 的最小形态，只返回能力取值与品牌默认值。该切片的判据只有一条：T0 的那一条合同能在 Windows 桌面端建单，并在同一端看到 T0 的那张收入报表。

T0 不要求 13b 的低代码与配置发布，也不要求 13a 的完整四端制品、商店合规门禁、本地加密缓存与离线草稿与增量同步、桌面端两个原生插件、能力矩阵表与其冻结比对、引导下发台账、深色与高对比度两套主题。移动薄 PoC 属 13a 前移验证，不进入 T0 判据；T0 本身仍不要求四端、不要求 scale 数据集、不要求分支覆盖。

移动端两端按己-3/F-51 的裁定从上面这份清单中移出，拆成两段。`/clients/mobile` 的 Tauri 2 移动壳本体与其 iOS、Android 生命周期和后台任务适配，连同一份可在真机安装运行的开发构建，由 13a 在阶段 1 结束后立即开工，并在任何业务移动界面大规模投入前完成薄 PoC；它不移入 T0 之前的关键路径。阈值失败只把移动 UI 换成 Flutter，客户端 Rust 核心九个 crate、服务端 Rust 核心、协议与数据模型不变。四端正式制品、白标驱动、商店合规门禁与完整门槛复测留在第二批，排在阶段 11 之后。前移的理由与 T0 判据无关：裁定 A-23 在阶段 5 至阶段 12 各写死一条退出条件，要求本模块移动界面通过 XCUITest 与 Espresso 用例；薄 PoC 先冻结可行 UI 路线，业务阶段才可持续投入 `clients/mobile/src/modules/<module>/`。移动壳不属于第 1.5 节首段向 T0 贡献的那五项，也不进入 T0 判据，该段末句所定的那条唯一切片判据一字不改。

本阶段第二批的全部交付物在 T0 已贯通的骨架上加厚。原先由本阶段承担的首次贯通性质表述一律删除，M7 保留为全分支闭环判据，本阶段不重复承担。

---

### 2. crate 与进程归属

#### 2.1 服务端新增或改动的 crate

| crate | 归属进程 | 本阶段职责 | 依赖方向核对 |
|---|---|---|---|
| ep-platform-meta | core-server、job-worker | 自定义对象与字段与关系与索引与视图的建模、在线 DDL 计划与影响分析、界面布局、能力等价矩阵判定、声明式规则 AST 与解释器实现 `AstRuleEvaluator`、自定义对象向权限与流程与搜索与报表的注册端口、六个 CUSTOM_/UI_LAYOUT applier 与 `RuleApplier`；实现 `SCHEMA_VALIDATION`、`IMPACT_ANALYSIS`、`CAPABILITY_MATRIX`、`RULE_SEMANTICS` 四套自动测试 | 只依赖 ep-foundation 与其他 ep-platform-*；本阶段新增对 ep-platform-release 的单向边以实现自动测试 SPI；无 sqlx、无 reqwest |
| ep-platform-release | core-server、job-worker | 本 crate 由阶段 3b 按裁定 A-27/F-56 交付最小发布通道与同序 Rust/DB `ItemKind=18`，`ConfigItemApplier` 端口与注册表由阶段 3a 先立；本阶段扩展内容项差异算法、把发布状态机由六态补齐为十一态、在既定 090500 给 Rust 与 CHECK 同批追加两个 MCP 项到终态 20、实现两类特殊单项包 shape/NON_ROLLBACKABLE guard，并维持普通项升序 apply、逆序 revert；定义九套自动测试 SPI 与执行器，本 crate 不实现具体 suite | 依赖仍为 ep-foundation、ep-platform-audit、ep-platform-outbox 三项；本 crate一律不反向依赖任何 applier 属主，属主只单向依赖本 crate |
| ep-platform-authz | core-server、job-worker | 在阶段 4 既有授权模型上实现 `RLS_MATRIX`、`ROLE_PREVIEW`、`SOD_CHECK` 三套自动测试，不另立权限判定路径 | 本阶段只新增对 ep-platform-release 的单向依赖；既有依赖与表归属不变 |
| ep-platform-flow | core-server、job-worker | 在阶段 3b 既有流程模型上实现 `FLOW_SEMANTICS` 自动测试，不另立流程解释器 | 本阶段只新增对 ep-platform-release 的单向依赖；既有依赖与表归属不变 |
| ep-app-reporting | core-server、job-worker | 在阶段 11 既有受治理数据集与权限投影上实现 `REPORT_PERMISSION` 自动测试，不另建报表权限模型 | 本阶段新增对 ep-platform-release 的单向依赖；不反向依赖其他 application crate |
| ep-adapter-wasm | plugin-host | wasmtime Component 宿主、能力清单裁剪、燃料与内存与时限限额、编译缓存、宿主导入函数四件套；实现类型 `WasmtimeComponentCompute` 是阶段 3b 定义的 `ep_platform_flow::port::WasmComputePort` 的进程内执行实现，装配进 `apps/plugin-host`，见裁定 B-05 与 H-02 | adapter 层，不依赖 application，也不依赖任何其他 ep-adapter-*；其余依赖按基线第 1.3 节允许项第六条，本 crate 取 ep-foundation、`ep_platform_flow::port` 的端口 trait 与第三方 wasmtime |
| ep-adapter-ipc | plugin-host、core-server、job-worker | 复用基线第 2 节已定的帧格式，新增 plugin 通道的服务端与客户端；实现类型 `PluginHostWasmCompute` 是 `ep_platform_flow::port::WasmComputePort` 的跨进程实现，经 `\\.\pipe\ep-plugin` 的唯一业务 operation `wasm.execute.v1` 转发至 plugin-host，装配进 `apps/core-server/src/wiring/` 与 `apps/job-worker/src/wiring/` 两个目录，见裁定 B-05 与 H-02；plugin 通道的请求体与响应体即该端口的入参与出参类型，取自 `ep_platform_flow::port`，本 crate 不另立第二套 DTO；取消由 deadline 或断开当前调用表达，不另设 cancel operation | adapter 层，不依赖 application，也不依赖任何其他 ep-adapter-*；其余依赖按基线第 1.3 节允许项第六条，本 crate 依赖 ep-platform-runtime 以实现基线第 1.2 节所定的 IPC 服务端接口，依赖 ep-platform-flow 以实现 `WasmComputePort` |
| ep-foundation | 全部 | 本阶段不新增也不改动 foundation 类型：`Tx`、`SnapshotCtx`、`UnitOfWork` 由阶段 1 按裁定 A-01 在 `port::tx` 中冻结，`SecurityContext` 与 `ClientKind` 按裁定 A-03 冻结，`ModuleCode`、`CapabilityDomain`、`ActionClass` 按裁定 A-20 冻结，`Redacted<T>` 同由阶段 1 提供，本阶段只引用 | 不依赖工作区内任何 crate |
| ep-platform-obs | ops-agent | 注册本阶段 9 个新指标 | 只登记，不改结构 |

`ep-plugin` 管道 server 身份固定为 `NT SERVICE\ep-plugin`，`reject_remote_clients=true`；仅 bootstrap 首实例取 `first_pipe_instance(true)`，后续/补位实例取 `false`。DACL 只授 `NT SERVICE\ep-core`、`NT SERVICE\ep-worker` 与 `NT SERVICE\ep-ops`；server 在读取任何应用字节前执行 `ImpersonateNamedPipeClient`→`OpenThreadToken` 核对服务 SID/账户，并在所有分支 `RevertToSelf`，PID 只作审计关联：core/worker 只允许 `wasm.execute.v1`，ops 只允许 `health.get.v1|metrics.snapshot.v1`。client 以 Identification SQOS 打开，发送前核对 server 进程 token。其他账户、ops 执行业务 operation、未登记 operation、服务端账户不匹配均拒绝并审计；plugin-host 不监听 TCP。

本阶段不新增 platform crate（`ep-platform-release` 与 `ep-platform-license` 均由阶段 3b 交付，本阶段只在前者之上扩展），不新增业务模块 crate，不新增进程，不新增 schema，不新增错误分类。

#### 2.2 客户端 crate（不属九个产品常驻进程，位于 workspace 之外的独立 Cargo workspace）

客户端代码位于仓库 `/clients/`，为独立 Cargo workspace，与服务端 workspace 通过路径依赖复用 `ep-foundation`、`ep-contract-*` 与 `ep-platform-meta`。客户端不得依赖任何 `ep-app-*`、`ep-adapter-db*` 与 `ep-platform-outbox`。

| crate | 平台 | 职责 |
|---|---|---|
| ep-client-core | 四端 | 客户端核心装配：会话、安全上下文、统一网络出口、TLS 与证书链校验、错误封套解析、能力矩阵本地判定、引导数据缓存 |
| ep-client-cache | 四端 | 本地加密缓存库：SQLCipher 打开与密钥解封、表结构、增量同步、冲突处理、超期清除、标签随行 |
| ep-client-keystore | 四端 | 设备硬件保护密钥的封装与解封：Windows DPAPI 加 TPM、macOS Keychain 加 Secure Enclave、iOS Secure Enclave、Android Keystore |
| ep-client-rules | 四端 | 声明式规则解释器的客户端外壳，直接复用 ep-platform-meta 的 `rule` 模块，只对 `executable_on_client` 为真的规则求值 |
| ep-client-draft | 四端 | 离线草稿：本地保存、待中心校验标记、恢复连接后按业务模块正常提交端点重提交 |
| ep-client-sync | 四端 | 按记录版本与更新时间的增量拉取、法人切换、缓存失效 |
| ep-client-plughost | Windows、macOS | 桌面端原生插件宿主：验签、能力清单核对、子进程拉起、受限 IPC、崩溃与超时隔离、本地登记表 |
| ep-client-device | 四端 | 设备登记、远程注销响应、安全清除、生物识别调用、相机扫码（移动端）、MDM 合规状态读取 |
| ep-client-audit | 四端 | 客户端审计事件的本地暂存与向中心提交；客户端不建本地分段链，照抄规格第 12.5 章 |

外壳与界面：

- `/clients/desktop`：Tauri 2 桌面壳，Windows 与 macOS 共用，暴露 Tauri IPC 命令给 WebView；其 `src/modules/<module>/` 为各业务模块的桌面界面目录，按裁定 A-23 由阶段 5 至阶段 12 各自填入，本阶段只交付壳、路由注册表与目录约定。
- `/clients/mobile`：Tauri 2 移动壳，iOS 与 Android 共用，含各自的生命周期与后台任务适配；其 `src/modules/<module>/` 为各业务模块的移动界面目录，归属同上。
- `/clients/ui`：React 加 TypeScript 共用组件包，四端共用同一套组件、权限模型与流程模型，仅按端切换布局密度与入口可见性；本包不含任何业务模块界面。
- `/clients/plugins/printer`、`/clients/plugins/usbkey`：两个桌面端原生插件的独立可执行工程。

#### 2.3 进程侧连接与账号的变化

- core-server 与 job-worker 继续使用 `ep_app_rw`，连接池上限不变。
- 在线 DDL 执行需要 `ep_migrator`。落地方式为：job-worker 仅在存在一张已签名已批准且含 DDL 段的发布单时，临时建立一条 `ep_migrator` 连接，执行完毕立即关闭。该连接计入基线第 2 节“迁移与应急临时连接另计不超过 10”的额度，同时按规格第 7.7 章把该账号的启用与回收各写一条审计事件。任何其他路径不得建立 `ep_migrator` 连接。
- plugin-host 的数据库连接数为 0，本阶段以启动自检与集成测试双重断言。

---

### 3. 数据库变更

#### 3.1 总则

本阶段全部新表落在 `platform_meta` 一个 schema 内。理由是本阶段不得定义其他阶段拥有的 schema 的表，而品牌配置、客户端版本、扩展登记三组虽不属狭义元数据，但都是部署级配置对象，与自定义对象元数据同属配置发布链路的内容项，集中在一个 schema 内使模块隔离边界与迁移边界一致。

表分两类。

- 部署级配置表：不带 `legal_entity_id`，不带 `data_scope_tags`，不建行级策略；按基线第 3.8 节的正向登记制逐表在 `platform_core.unpoliced_table_registry` 登记一行，`admission_basis` 取 `SAME_FOR_ALL_ENTITIES`，即该表的行在本部署内对全部法人取值相同，`isolation_entry` 与 `matrix_case_id` 两列由第 3.4 节第 14 号迁移填写，未登记的表由 `db/checks` 第十三项判为违规而建不出来。其可见性由对象级权限判定。这一判断是本阶段的显式判断，理由是低代码配置在本部署内跨两个法人共用，为其编造一个法人列会制造第二套隔离口径，与基线第 4 节反对 `tenant_id` 的理由同构。
- 法人级运行台账表：带 `legal_entity_id`，按基线第 3.8 节模板建行级策略，列全。

全部表带基线第 4 节公共列，顺序按基线；仅追加表不带 `row_version`、`updated_at`、`updated_by`。`reverses_id` 不是仅追加表的通用列，只有正文定义了真实反向父链、且能建立目标外键与方向/累计约束的表才允许出现；本阶段的步骤、调用与下发审计表都没有这种父链，因而均不带该列。单目标引用一律建立真实外键；法人级目标使用 `(legal_entity_id,ref_id)` 复合外键，业务用户引用指向 `platform_authz.user_legal_entity_grants(legal_entity_id,user_id)`，全局身份/设备证据用单列外键并在写事务校验法人和用户归属。精确审批引用与动态 ext 关系元数据是封闭例外，不建伪外键。枚举一律 `text` 加 CHECK。时间列 `timestamptz`，日期列 `date`。主键 `uuid`，应用侧 UUIDv7。
按裁定 A-27，`platform_meta.config_packages`、`platform_meta.config_package_items` 与 `platform_meta.config_release_orders` 三张表由阶段 3b 随最小发布通道建立，第 3.2.10 至 3.2.12 节所列列定义即阶段 3b 的落地口径，本阶段只做列扩展与状态扩展，不重复建表；`config_release_steps`、`config_autotest_runs`、`config_edit_locks` 与 `config_release_mutex` 四张由本阶段建立，本阶段因此新建 19 张表。发布状态机以 PRD 第 10.4.1 节的十一态为唯一出处：阶段 3b 实现其中 Draft、PendingApproval、Rejected、Approved、Released、RolledBack 六态，差异审查由 `GET /api/v1/platform/config-packages/{id}/diff` 端点承载，不单列为状态；本阶段补齐其余 PendingAutotest、TestFailed、TestPassed、SignedPendingRelease、Superseded 五态，六加五合计十一态，扩展只放宽 `ck_config_packages_status`，不改写任何既有行。全部种子迁移与系统上下文写入的 `created_by` 一律取 `foundation::SYSTEM_PRINCIPAL_ID`，不得自选取值，见裁定 A-02。

#### 3.2 表定义

##### 3.2.1 platform_meta.custom_objects（部署级）

| 列 | 类型 | 可空 | 默认 | 说明 |
|---|---|---|---|---|
| id | uuid | 否 | 无 | 主键 |
| security_level | smallint | 否 | 20 | 对象级密级，取值 10、20、30、40 |
| code | text | 否 | 无 | 对象码，同时是 `ext` 下的物理表名 |
| name | text | 否 | 无 | 显示名 |
| physical_table_name | text | 否 | 无 | 固定为 `ext.` 加 code |
| is_document | boolean | 否 | false | 单据类为真，档案类为假 |
| doc_type_code | text | 是 | 无 | 单据类必填，2 至 4 位大写字母 |
| definition_version | bigint | 否 | 1 | 每次生效的定义版本，客户端引导按此判定缓存失效 |
| status | text | 否 | 'DRAFT' | DRAFT、PENDING_DDL、ACTIVE、DDL_FAILED、RETIRED |
| retired_at | timestamptz | 是 | 无 | 停用时间 |
| row_version | bigint | 否 | 1 | 乐观锁 |
| created_at / created_by / updated_at / updated_by | 按基线 | 否 | 按基线 | 公共列 |

约束与索引：

- `pk_custom_objects`。
- `ux_custom_objects_code`。
- `ix_custom_objects_status_created_at`。
- `ck_custom_objects_code_shape`：`code ~ '^[a-z][a-z0-9_]{2,62}s$'`，即小写 snake_case 且以 s 结尾，落实基线第 3.2 节表名复数。
- `ck_custom_objects_status`：取值集合。
- `ck_custom_objects_security_level`：取值 in (10,20,30,40)。
- `ck_custom_objects_doc_type`：`is_document = false and doc_type_code is null` 或 `is_document = true and doc_type_code ~ '^[A-Z]{2,4}$'`。
- 单据类型码的全局唯一性按裁定 C-26 由应用层校验：`doc_type_code` 必须与 `docs/data-dictionary.md` 单据类型码一节的全量表以及 `ep-platform-sequence` 的常量表逐项比对，重复即拒绝并返回 `PLATFORM.CUSTOM_OBJECT.DOC_TYPE_CODE_CONFLICT`，HTTP 409，category 为 BUSINESS_CONFLICT；`xtask configdoc --check-doc-type-codes` 同时覆盖内置码与已生效的自定义对象码两类。
- `ck_custom_objects_name_len`：`char_length(name) <= 200`，按基线第 11.2 节。
- 无行级策略。

##### 3.2.2 platform_meta.custom_fields（部署级）

| 列 | 类型 | 可空 | 说明 |
|---|---|---|---|
| id | uuid | 否 | 主键 |
| security_level | smallint | 否 | 表自身的密级，固定 20 |
| custom_object_id | uuid | 是 | 自定义对象上的字段时非空，同 schema 外键 ON DELETE RESTRICT |
| core_object_type | text | 是 | 核心对象上的自定义字段时非空，取值形如 `sales.sales_orders` |
| code | text | 否 | 字段码，同时是物理列名 |
| name | text | 否 | 显示名 |
| data_type | text | 否 | INTEGER、DECIMAL、FLOAT、BOOLEAN、STRING、TEXT、DATE、TIMESTAMP、ENUM、REFERENCE、JSON |
| is_nullable | boolean | 否 | 首版新增列一律为真，收紧非空进停机窗口 |
| max_length | int | 是 | STRING 与 TEXT 必填 |
| decimal_precision | smallint | 是 | DECIMAL 必填 |
| decimal_scale | smallint | 是 | DECIMAL 必填 |
| enum_values | text[] | 是 | ENUM 必填，取值大写 snake_case |
| reference_object_type | text | 是 | REFERENCE 必填；一旦版本发布即固定到一个目标对象，DDL 生成规则见第 3.3 节 |
| default_expr | text | 是 | 只接受常量字面量，不接受易失函数 |
| field_security_level | smallint | 是 | 空表示按所属对象取值 |
| physical_column_name | text | 否 | 与 code 同值 |
| definition_version | bigint | 否 | 定义版本 |
| status | text | 否 | DRAFT、PENDING_DDL、ACTIVE、DDL_FAILED、RETIRED |
| row_version、created_at、created_by、updated_at、updated_by | 按基线 | 否 | 公共列 |

约束与索引：

- `pk_custom_fields`、`fk_custom_fields_custom_objects`。
- `ux_custom_fields_object_code`：唯一约束在表达式上无法写为普通唯一索引，因此新增数据库生成列 `owner_key text GENERATED ALWAYS AS (CASE WHEN custom_object_id IS NOT NULL THEN 'custom:' || custom_object_id::text ELSE 'core:' || core_object_type END) STORED`，并在 `(owner_key, code)` 上建普通唯一索引 `ux_custom_fields_owner_key_code`。应用命令、仓储 INSERT/UPDATE 与导入格式均不得接收 `owner_key`；`custom:`/`core:` 前缀隔离 UUID 与核心类型两个命名空间。这样不使用函数索引且列值不可漂移，符合基线第 3.10 节。
- `ix_custom_fields_owner_key_created_at`。
- `ck_custom_fields_owner_exactly_one`：`custom_object_id is not null` 与 `core_object_type is not null` 恰有一个成立。
- `ck_custom_fields_type_params`：按 data_type 逐类校验参数组合非空或为空。
- `ck_custom_fields_decimal_bounds`：`decimal_precision <= 18 and decimal_scale <= 6`，与基线第 3.5 节的金额、单价、数量精度上界一致。
- `ck_custom_fields_code_shape`：`code ~ '^[a-z][a-z0-9_]{1,62}$'`，且不得命中保留列名集合（公共列九项加 `doc_no`、`status`、`code`、`is_active`、`deactivated_at`、`posting_date`、`business_date`、`accounting_period_id`、`reverses_id`），保留列名由应用层校验并返回 `PLATFORM.CUSTOM_OBJECT.RESERVED_NAME`。
- 无行级策略。

##### 3.2.3 platform_meta.custom_relations（部署级）

列：id、security_level、code、from_object_type text、to_object_type text、cardinality text（ONE_TO_ONE、ONE_TO_MANY、MANY_TO_MANY）、link_table_name text（多对多必填，形如 `ext.<a>_<b>_links`）、fk_column_name text（一对一与一对多必填）、on_delete text（固定 'RESTRICT'）、definition_version bigint、status text、公共列。

约束与索引：`pk_custom_relations`、`ux_custom_relations_code`、`ix_custom_relations_from_object_type_created_at`、`ck_custom_relations_cardinality`、`ck_custom_relations_on_delete`、`ck_custom_relations_shape`（按 cardinality 判定 link_table_name 与 fk_column_name 的必填组合）。无行级策略。

##### 3.2.4 platform_meta.custom_indexes（部署级）

列：id、security_level、code、target_object_type text、index_kind text（SINGLE、COMPOSITE、UNIQUE）、column_codes text[]、physical_index_name text、definition_version bigint、status text、公共列。

约束与索引：`pk_custom_indexes`、`ux_custom_indexes_physical_index_name`、`ix_custom_indexes_target_object_type_created_at`、`ck_custom_indexes_kind`、`ck_custom_indexes_columns_len`（`array_length(column_codes,1) between 1 and 4`，SINGLE 时必须为 1）。该 CHECK 集合就是规格第 7.4 章公共能力基线的落地，函数索引、局部索引与 JSON 路径索引在此不存在可表达的取值。无行级策略。

##### 3.2.5 platform_meta.custom_views（部署级）

列：id、security_level、code、name、base_object_type text、select_field_codes text[]、filter_spec jsonb、sort_spec jsonb、definition_version bigint、status text、公共列。

约束与索引：`pk_custom_views`、`ux_custom_views_code`、`ix_custom_views_base_object_type_created_at`。视图为逻辑视图，不建数据库视图对象，理由是数据库视图会绕过统一前置查询服务的字段级与密级过滤；查询在应用层按 filter_spec 组装并经同一权限路径执行。无行级策略。

##### 3.2.6 platform_meta.ddl_plans（部署级）

列：id、security_level、config_release_order_id uuid（同 schema 外键）、plan_no text、target_schema text（固定 'ext'）、execution_mode text（ONLINE、MAINTENANCE_WINDOW）、statement_count int、impact_index jsonb、impact_capacity jsonb、impact_performance jsonb、impact_security jsonb、impact_migration jsonb、lock_timeout_ms int（默认 5000）、statement_timeout_ms int（默认 1800000）、status text（PLANNED、EXECUTING、SUCCEEDED、FAILED、ROLLED_BACK、DEFERRED_TO_WINDOW）、started_at timestamptz、finished_at timestamptz、elapsed_ms int、max_lock_wait_ms int、failure_reason text、公共列。

约束与索引：`pk_ddl_plans`、`ux_ddl_plans_plan_no`、`fk_ddl_plans_config_release_orders`、`ix_ddl_plans_status_created_at`、`ck_ddl_plans_execution_mode`、`ck_ddl_plans_status`、`ck_ddl_plans_lock_timeout`（`lock_timeout_ms <= 5000`，落实规格第 7.4 章的 5 秒上限，配置不得调高）、`ck_ddl_plans_statement_timeout`（`statement_timeout_ms <= 1800000`）。五个影响分析列一律非空，对应 PRD 第 10.4.1 节“发布前的影响分析覆盖索引、容量、性能、安全与迁移五项”。无行级策略。

##### 3.2.7 platform_meta.ddl_plan_steps（部署级，仅追加）

列：id、security_level、ddl_plan_id uuid、seq int、sql_kind text（CREATE_TABLE、ADD_COLUMN、CREATE_INDEX_CONCURRENTLY、ENABLE_RLS、CREATE_POLICY、RELAX_CHECK、DROP_INDEX_CONCURRENTLY）、sql_text text、is_online boolean、started_at、finished_at、lock_wait_ms int、elapsed_ms int、outcome text（OK、TIMEOUT、ERROR、ROLLED_BACK、RETAINED_INACTIVE）、error_text text、created_at、created_by。普通配置包的 DDL 闭集不含 `DROP_COLUMN` 或 `DROP_TABLE`；该执行步骤没有可被反向事实引用的父链，安全补偿、保留为不可见物理结构与发布审计均以新的结果步骤表达，不设 `reverses_id`。

约束与索引：`pk_ddl_plan_steps`、`fk_ddl_plan_steps_ddl_plans`、`ux_ddl_plan_steps_plan_seq`、`ix_ddl_plan_steps_ddl_plan_id_created_at`、`ck_ddl_plan_steps_sql_kind`、`ck_ddl_plan_steps_outcome`。仅追加表，无 row_version。无行级策略。

##### 3.2.8 platform_meta.ui_layouts（部署级）

列：id、security_level、code、layout_kind text（FORM、LIST、HOME、MENU、BOARD）、target_object_type text（MENU 与 HOME 取固定值 `-`）、role_id uuid（空表示默认布局）、client_scope text（ALL、DESKTOP、MOBILE）、spec jsonb、definition_version bigint、status text（DRAFT、ACTIVE、SUPERSEDED、RETIRED）、公共列。

约束与索引：`pk_ui_layouts`、`ux_ui_layouts_kind_target_role_scope_version`（在 `(layout_kind, target_object_type, role_key, client_scope, definition_version)` 上，其中 `role_key text GENERATED ALWAYS AS (coalesce(role_id::text,'-')) STORED not null`；应用命令、仓储与导入格式不得接收该列）、`ix_ui_layouts_status_created_at`、`ck_ui_layouts_kind`、`ck_ui_layouts_client_scope`、`ck_ui_layouts_status`。生成列避免函数索引、NULL 分组歧义与应用伪造，理由与基线第 11.4 节空批次标识取 `'-'` 同构。无行级策略。

##### 3.2.9 platform_meta.client_capability_values（部署级）

规格第 6.2 章取值矩阵的机器可读副本，是运行期能力闸的唯一判据。

列：id、security_level、capability_domain text、client text（win、mac、ios、android）、value text（FULL、SIMPLIFIED、VIEW_ONLY、NOT_APPLICABLE）、exemption_ref text、alternative_path text、frozen_hash text、公共列。

约束与索引：`pk_client_capability_values`、`ux_client_capability_values_domain_client`、`ix_client_capability_values_client_created_at`、`ck_client_capability_values_client`、`ck_client_capability_values_value`、`ck_client_capability_values_exemption`（value 为 VIEW_ONLY 或 NOT_APPLICABLE 时 `alternative_path` 必须非空，落实规格第 6.2 章“标注为仅查看或不适用的取值必须在本清单中有对应条目说明替代路径”）。

种子数据由迁移文件 backfill 写入，18 个能力域乘 4 端共 72 行，取值逐格照抄规格第 6.2 章矩阵。能力域码表见第 4.4 节。无行级策略。

##### 3.2.10 platform_meta.config_packages（部署级）
本表由阶段 3b 按裁定 A-27 随最小发布通道建立，本节列定义即其落地口径；本阶段只做列扩展与状态扩展，追加 PENDING_AUTOTEST、TEST_FAILED、TEST_PASSED、SIGNED_PENDING_RELEASE、SUPERSEDED 五个状态取值与自动测试相关列，不重复建表，不改写任何既有行。

列：id、security_level、package_no text、name text、source text（IMPORTED、IN_PLACE）、git_ref text、manifest jsonb、content_hash text、`content_version bigint not null default 1`（只在包体或内容项变化时递增）、item_count int、signature bytea、signature_key_ref text、signer_subject text、signed_at timestamptz、min_platform_version text、status text、rejected_reason text、`approval_legal_entity_id uuid`、`approval_scenario text`、`submitted_by uuid`、`submitted_at timestamptz`、`approval_ref uuid`、`approval_chain_id uuid`、`approval_chain_version_no int`、`approval_definition_digest bytea`、`approval_content_version bigint`、`approval_content_hash text`、`approved_by uuid`、`approved_at timestamptz`、`rejected_by uuid`、`rejected_at timestamptz`、`active_autotest_batch_id uuid`、`autotest_attempts smallint not null default 0`、`autotest_available_at timestamptz`、`autotest_locked_by text`、`autotest_locked_until timestamptz`、`autotest_last_error text`、公共列。`signer_subject` 的安全持久化/wire 值固定 `spki-sha256:<64 lowerhex>`，显示 DN 只从证书派生且不参与身份；审批证据列由阶段 3b 的建表迁移一次建齐，阶段 13 只沿用；`approval_ref` 是流程实例证明，按基线白名单不建外键，其余法人和用户列建下述真实外键。

status 取值与 PRD 第 10.4.1 节状态表逐行对应：DRAFT（草稿）、PENDING_AUTOTEST（待自动测试）、TEST_FAILED（测试失败）、TEST_PASSED（测试通过）、PENDING_APPROVAL（待审批）、REJECTED（已驳回）、APPROVED（已批准）、SIGNED_PENDING_RELEASE（已签名待发布）、RELEASED（已发布）、ROLLED_BACK（已回退）、SUPERSEDED（已被替代）。

约束与索引：`pk_config_packages`、`ux_config_packages_package_no`、`ux_config_packages_content_hash`、`ix_config_packages_status_created_at`、`ix_config_packages_autotest_claim`（`status, autotest_available_at, autotest_locked_until, created_at`，不使用部分索引）、`fk_config_packages_approval_legal_entity`、`fk_config_packages_submitted_by_grant`、`fk_config_packages_approved_by_grant`、`fk_config_packages_rejected_by_grant`（后三项均以 `approval_legal_entity_id,user_id` 指向 `platform_authz.user_legal_entity_grants(legal_entity_id,user_id)`）、`ck_config_packages_status`、`ck_config_packages_source`、`ck_config_packages_item_count`（`item_count between 1 and 2000`）、`ck_config_packages_content_version`（`content_version >= 1`）、`ck_config_packages_autotest_attempts`（`autotest_attempts >= 0`）、`ck_config_packages_autotest_lease_pair`（`autotest_locked_by` 与 `autotest_locked_until` 同空或同非空）、`ck_config_packages_pending_autotest`（status 为 PENDING_AUTOTEST 时 `active_autotest_batch_id` 与 `autotest_available_at` 均非空；status 非 PENDING_AUTOTEST 时 `autotest_locked_by`、`autotest_locked_until` 与 `autotest_available_at` 均为空）、`ck_config_packages_approval_scenario`（非空时只能为 `CONFIG_RELEASE`）、`ck_config_packages_approval_shape`（进入 PENDING_APPROVAL 及其后各态时法人、场景、申请人、提交时间、approval_ref、链 id/版本/digest、审批内容版本/hash 全非空且 `approval_content_version=content_version and approval_content_hash=content_hash`；PENDING_APPROVAL 的两组结论列全空；REJECTED 只允许 rejected 三列非空；APPROVED 及其后只允许 approved 两列非空；此前各态审批列全空）、`ck_config_packages_no_self_approval`（`approved_by is null or approved_by <> submitted_by`）、`ck_config_packages_signed`（status 为 SIGNED_PENDING_RELEASE 及之后时 `signature`、`signer_subject`、`signed_at` 三列均非空）。包进入 TEST_PASSED（阶段 3 的六态则从提交审批开始）后，触发器禁止修改包体、`content_hash/content_version` 或内容项；状态只能由第 4.2 节的命名命令和审批回调迁移。无行级策略。

此外必须建立 `fk_config_packages_approval_chain`：`(approval_legal_entity_id,approval_chain_id) -> platform_authz.approval_chains(legal_entity_id,id) ON DELETE RESTRICT`；它与上述用户真实外键一起保证审批证据不跨法人。只有 `approval_ref` 继续属于平台证明无外键白名单。

##### 3.2.11 platform_meta.config_package_items（部署级）
本表由阶段 3b 按裁定 A-27 随最小发布通道建立，本节列定义即其落地口径，`item_hash` 算法与第 4.7 节一致；本阶段只做列扩展，不重复建表。

列：id、security_level、config_package_id uuid、item_kind text、item_code text、change_kind text（ADD、MODIFY、REMOVE）、applies_to_legal_entity_ids uuid[]（空数组表示全部法人）、before_spec jsonb、after_spec jsonb、item_hash text、sort_no int、`accepted_trust_bundle_sha256 bytea null`、公共列。该摘要列由 Stage 3 的首次建表迁移 090500 创建，不得由 Stage 13 的同尾号 ALTER 重复添加。

item_kind 终态封闭为 20 项，固定顺序为 CUSTOM_OBJECT、CUSTOM_FIELD、CUSTOM_RELATION、CUSTOM_INDEX、CUSTOM_VIEW、UI_LAYOUT、FLOW_DEFINITION、AUTHZ_ROLE、AUTHZ_POLICY、AUTHZ_FIELD_GRANT、REPORT_DEFINITION、METRIC_DEFINITION、DASHBOARD_DEFINITION、PRINT_TEMPLATE、NOTIFY_RULE、RULE、LICENSE_GRANT、MODULE_PACKAGE、MCP_CONNECTOR、MCP_MANIFEST_VERSION。Stage 3 Rust `ItemKind::ALL` 与首次 CHECK 已恰为前 18 项；本阶段的 `V20261022090500__platform_meta_alter_config_package.sql` 只在尾部追加两个 MCP 项，并同批把 Rust/DB 扩为终态 20，不得出现 Rust=20/DB=18 的瞬时发布版本，也不得另建重复 ALTER。两个 MCP applier 由 `ep-platform-mcp` 实现；两个 F-56 applier 已由 Stage 3b 的 `ep-platform-license` 实现；全部只接受 strict DTO/签名/审批/状态机，不提供任意 CRUD。PRINT_TEMPLATE 只产出 `ep_foundation::port::doc::PrintLayout`。

约束与索引：`pk_config_package_items`、`fk_config_package_items_config_packages`、`ux_config_package_items_pkg_kind_code`、供 F-56 同包来源 FK 使用的候选唯一键 `ux_config_package_items_package_id_id(config_package_id,id)`（只由既定 093300 后补，Stage 3 的 090500 建表不创建）、`ix_config_package_items_config_package_id_created_at`、`ck_config_package_items_item_kind`、`ck_config_package_items_change_kind`、`ck_config_package_items_specs`（ADD 时 `before_spec` 为空且 `after_spec` 非空，REMOVE 时相反，MODIFY 时两者均非空）、`ck_config_package_items_accepted_trust_len`（摘要为空或恰 32 bytes）。普通 item 的 accepted 摘要恒为 null；F-56 special 在所属 package 未 RELEASED 时必须为 null，RELEASED 时恰为 32 bytes。非空值不可修改或清空，跨 package/item/许可/模块投影的一致性由 093300 的 DEFERRABLE INITIALLY DEFERRED 约束触发器在提交时强制。无行级策略。

093300 的 F-56 deferred graph 分别以约束触发器挂到 `config_packages`、`config_package_items`、`module_registrations`、`license_grants` 与 `legal_entities` 五张表，均为 DEFERRABLE INITIALLY DEFERRED：普通 item 必须恒 null；special 的 package 非 RELEASED 时必须 null，RELEASED 时必须 32 bytes；任何已非空值发生改变或清空都在 COMMIT 拒绝；grant 投影的既有 `trust_bundle_sha256` 必须逐字等于 `grant_source_config_item_id` 所指 item 的 accepted 摘要；revocation 与每个 module action 的接受摘要只从其不可删除 RELEASED source item 读取，不另复制列。它还强制同 deployment 的最早 RELEASED grant 唯一冻结治理法人、全部后继相等且 LIST scope 必含该值、治理法人持续 active；special 在 `DRAFT|PENDING_AUTOTEST|TEST_FAILED|TEST_PASSED` 的 `approval_legal_entity_id` 必须为空，PENDING_APPROVAL 及以后才必须等于首张候选或首次 RELEASED history 唯一派生值。全部 RELEASED MODULE_PACKAGE history 的 `package_id` 与 `(module_code,package_code,semver)` 两套 identity 都只映射同一 exact inner。该迁移先添加 `UNIQUE(config_package_id,id)` 再创建六条来源 FK 与约束触发器；090500 不得提前创建父候选键或把摘要默认填为当前 bundle。

F-56 特殊单项包形状由导入、提交、签名和发布四个命名命令共同守卫：包必须 `source=IMPORTED,item_count=1`，唯一 item 必须为 `LICENSE_GRANT|MODULE_PACKAGE`、`change_kind=ADD,before_spec=null,after_spec!=null,applies_to_legal_entity_ids=[]`；`LICENSE_GRANT.after_spec` 恰为 `SignedBusinessArtifactV1<LicenseArtifactPayloadV1>`，`MODULE_PACKAGE.after_spec` 恰为 `ModulePackageItemV1 { schema_version:1, action, reason, artifact: SignedBusinessArtifactV1<ModulePackageManifestV1> }`，不得把整个 module item 当成同一泛型封套；两类包仍完整跑九套 autotest、`CONFIG_RELEASE` 非自审链、外层签名和各自 artifact 的内层 CMS。DRAFT 到 TEST_PASSED 的 approval 法人列始终为空；每个推进命令只在内存中从首张候选或首次 RELEASED grant history 派生 `governance_context_id`，要求当前受信 session/operator 对该法人有对应动作权限，请求头若存在只能相等。submit 同一事务才首次写 `approval_legal_entity_id=derived id`；PENDING_APPROVAL 及以后不得改变。首张 RELEASE 后永久冻结治理法人，后继 grant 必须同值且 LIST scope 必含它。派生不唯一、授权缺失、预填 approval 列、请求头/当前浏览法人/ServerAdmin UI/配置/环境变量覆盖或 history/source/signature 损坏均失败关闭。typed DTO 通过后的这些语义偏离返回 `PLATFORM.CONFIG_PACKAGE.SPECIAL_ITEM_SHAPE_INVALID`。部署在首张有效 grant 接受前允许零 current，数据库始终至多一张，首张接受事务提交后才恰一张；五个 Restricted reason 不得阻断 LICENSE_GRANT 恢复全链与 MODULE_PACKAGE:DISABLE 全链。ServerAdmin 不新增后端路由，只组合 import/diff/autotest/submit/sign/release-order/execute；审批结论只由 Win/Mac 待办完成。除此恢复闭集外，Restricted 下常规业务写统一返回 `PLATFORM.LICENSE.RESTRICTED`；LIST scope 不命中同样拒绝但不改全局 status/reason，查询/报表/导出保留，已有目标法人绝不能传 None 绕过。

首装 evidence 与本阶段向导不能混用。唯一首装命令仍是既有 `ep-migrate apply` 三参数，且只接受 canonical lowercase deployment UUID 子目录 `C:\ProgramData\EnterprisePlatform\evidence\stage14\initial-governance\<lowercase-deployment-id>\` 固定根的 `bootstrap.jcs`、`license.epcfg` 与固定输出 `initial-governance.receipt.v1.jcs`。目录 owner SYSTEM、DACL PROTECTED；显式 inheritable allow ACE 恰为 SYSTEM/BUILTIN\Administrators/`NT SERVICE\ep-ops` FullControl 与 `NT SERVICE\ep-core` 的 `FILE_GENERIC_READ|FILE_TRAVERSE|READ_CONTROL|SYNCHRONIZE`，其余服务与通用用户零 ACE，ep-core 无 write/delete/WRITE_DAC/WRITE_OWNER。safe-handle 拒绝 UNC/device/reparse/ADS/hardlink/8.3/case/path drift，receipt CREATE_NEW/flush/readback 且无 sidecar。bootstrap 事务只落 signed key-domain id、exact provider-independent `/kek/1` locator 与 PROVISIONING；core readiness 前由 `KeyDomainProvisioner` 经 `KmsKeyMaterialProvisioner` 三方法生成/回读四 purpose×四 scope exact 16 条 wrapped DEK 后，才同事务切 ACTIVE。ServerAdmin/Win/Mac 不得上传、生成、替换或下载这三份首装 evidence；本阶段只在共同 release gate 核对首装 receipt、审计、最终 ACTIVE key domain、16-tuple graph 与首张 RELEASED grant。

strict multipart 上传仍复用 `POST /api/v1/platform/config-packages/actions/import`：Win/Mac 的 `application/json {attachment_object_id}` 形态不变；同路径 `multipart/form-data` 形态对受信入口已认证的 Win、Mac 与 ServerAdmin 均开放，只给该 media type 设置编译期 route-local 4,194,304-byte body limit。两种形态共用 `lowcode.config_package.import`、同一 `ConfigRelease` binding、幂等作用域与审计；multipart 不赋予第二套权限、通用附件或任意文件能力。请求 `Content-Type` 恰为未加引号的 `multipart/form-data; boundary=<token>`，token 为 1..70 ASCII bytes 且只含 HTTP-token 安全子集 `[A-Za-z0-9'._+-]`；无 preamble/epilogue、CRLF-only。恰一个名为 `package` 的 file part，headers 顺序/bytes 固定为 `Content-Disposition: form-data; name="package"; filename="<filename>"\r\n`、`Content-Type: application/vnd.enterprise-platform.epcfg+zip\r\n` 后接空行；filename 匹配 `[A-Za-z0-9][A-Za-z0-9._-]{0,121}\.epcfg`，为 7..128 ASCII bytes；零额外 header/part/form field，末尾恰 CRLF、closing boundary、CRLF。framing size 恰为 `136+2*boundary_len+filename_len`、最大 404 bytes，archive/file hard cap=4,193,900 bytes。

`Content-Length` 必填、是规范十进制 `1..=4,194,304` 且逐字等于 `framing_size+archive_size`；缺失、非法、为零、超限或任何 `Transfer-Encoding` 均在读取 body/创建临时对象前拒绝。逐流同时以 4,194,304-byte body 与 4,193,900-byte file 硬截止并拒绝短读/长读。长度、boundary、framing、headers、part、MIME、filename、扩展名或 archive cap 任一不符，全都映射既有 `PLATFORM.REQUEST.INVALID_PAYLOAD`，HTTP 400、`retryable=false` 且配置包零落库；不得新增 413、同义错误码或透传 multipart/IO 错误。其他路由和全局 1 MiB body limit 完全不变。handler 只以 CREATE_NEW 将 file bytes 写入 `C:\ProgramData\EnterprisePlatform\staging\config-import\<request-id>.epcfg`，目录 owner SYSTEM、关闭继承，只有 SYSTEM/Administrators/`NT SERVICE\ep-core` 可管理；拒绝 UNC/device/reparse/ADS/hardlink，逐流计算 digest，并在成功或失败后都关闭句柄和删除 staging 文件。该文件不是 attachment 或通用文件能力，任何下载、列表或 API 都不可读取。Restricted/零 current 下先 strict parse，再由 import 的 exclusive transaction 持久化并从 exact bytes 重验；唯一 LICENSE_GRANT 才映射 `LicenseGrantRecovery`，MODULE_PACKAGE:DISABLE 仍按 strict target 映射 `ModuleDisableRecovery`，普通包与其他模块动作拒绝。通用 attachment init/part/complete 永不获得恢复权限，因此 Win/Mac 可直接 multipart 导入首张许可而不先创建会被 Restricted 阻断的 attachment。ServerAdmin 每次读取与每日 monitor 都在同一 repeatable-read snapshot 实时按唯一 SQL 谓词计算三项 usage：法人 `platform_core.legal_entities.is_active=true`；命名用户 `platform_core.user_accounts.account_kind<>'SYSTEM' AND status IN ('ACTIVE','LOCKED','SUSPENDED')`；设备 `platform_core.user_devices.status IN ('PENDING','ACTIVE')`。每日只刷新 metrics 并在越限/恢复边缘发既有告警；不新增 usage 表、日终持久化、月度签名申报、联网遥测或发行方上报，授权导出仅是受审计的时点报表。超限不阻断创建；module/entitlement scope 是硬门。

##### 3.2.12 platform_meta.config_release_orders（部署级）
本表由阶段 3b 按裁定 A-27 随最小发布通道建立，本节列定义即其落地口径；本阶段只做列扩展与状态扩展，不重复建表。

列：id、security_level、order_no text、config_package_id uuid、action text（RELEASE、ROLLBACK）、rollback_to_package_id uuid、execution_mode text（ONLINE、MAINTENANCE_WINDOW）、submitted_by uuid、approved_by uuid、approval_ref text、reauth_ref text、scheduled_window_start timestamptz、status text（SUBMITTED、APPROVED、REJECTED、QUEUED、EXECUTING、SUCCEEDED、FAILED、COMPENSATED、CANCELLED）、started_at、finished_at、elapsed_ms int、failure_reason text、公共列。

约束与索引：`pk_config_release_orders`、`ux_config_release_orders_order_no`、`fk_config_release_orders_config_packages`、`ix_config_release_orders_status_created_at`、`ck_config_release_orders_action`、`ck_config_release_orders_status`、`ck_config_release_orders_self_approval`（`approved_by is null or approved_by <> submitted_by`）、`ck_config_release_orders_rollback`（action 为 ROLLBACK 时 `rollback_to_package_id` 非空）。创建 ROLLBACK 单时必须锁定目标包并检查其 items；命中 `LICENSE_GRANT|MODULE_PACKAGE` 即返回 `PLATFORM.CONFIG_RELEASE_ORDER.NON_ROLLBACKABLE_ITEM`（409、不可重试）且零写入，不能以普通行 CHECK 代替这条跨表事务守卫。无行级策略。

##### 3.2.13 platform_meta.config_release_steps（部署级，仅追加）

列：id、security_level、config_release_order_id uuid、seq int、item_kind text、item_code text、applier text、phase text（DDL、METADATA、PROPAGATION）、outcome text（OK、FAILED、SKIPPED、COMPENSATED）、started_at、finished_at、elapsed_ms int、error_text text、created_at、created_by。补偿结果是同一发布单的另一条步骤事实，不构成逐行反向父链，故不设 `reverses_id`。

约束与索引：`pk_config_release_steps`、`fk_config_release_steps_config_release_orders`、`ux_config_release_steps_order_seq`、`ix_config_release_steps_config_release_order_id_created_at`、`ck_config_release_steps_phase`、`ck_config_release_steps_outcome`。无行级策略。

##### 3.2.14 platform_meta.config_autotest_runs（部署级）

列：id、security_level、config_package_id uuid、batch_id uuid not null、suite text、state text（QUEUED、RUNNING、FINISHED）、available_at timestamptz not null、outcome text null（PASSED、FAILED、SKIPPED；FINISHED 前必须为空）、started_at、finished_at、elapsed_ms int、failure_count smallint not null default 0、report jsonb、公共列。

suite 取值封闭为 9 项：SCHEMA_VALIDATION、IMPACT_ANALYSIS、RLS_MATRIX、ROLE_PREVIEW、FLOW_SEMANTICS、REPORT_PERMISSION、CAPABILITY_MATRIX、SOD_CHECK、RULE_SEMANTICS。第九项按 F-21 保留；`COMPENSATION_POLICY` 已由 F-10 撤销，不得进入枚举或注册表。

约束与索引：`pk_config_autotest_runs`、`fk_config_autotest_runs_config_packages`、`ux_config_autotest_runs_pkg_batch_suite`（`config_package_id, batch_id, suite`）、`ix_config_autotest_runs_due`（`config_package_id, batch_id, state, available_at`，不使用部分索引）、`ix_config_autotest_runs_config_package_id_created_at`、`ck_config_autotest_runs_suite`、`ck_config_autotest_runs_state`、`ck_config_autotest_runs_failure_count`（`failure_count between 0 and 9`）、`ck_config_autotest_runs_finished_outcome`（`state='FINISHED'` 当且仅当 outcome 与 finished_at 非空）。无行级策略。包行 `autotest_available_at` 只冗余当前 batch 未完成运行行的最小 `available_at` 供领取索引使用，逐套件到期判定只读本表，不得用包级值覆盖所有套件。

##### 3.2.15 platform_meta.config_edit_locks（部署级）

对应 PRD 附录乙 U-K-01 经 F-51 批准的首版冻结值。

列：id、security_level、item_kind text、item_code text、locked_by uuid、locked_at timestamptz、expires_at timestamptz、公共列。

约束与索引：`pk_config_edit_locks`、`ux_config_edit_locks_kind_code`、`ix_config_edit_locks_expires_at_created_at`、`ck_config_edit_locks_window`（`expires_at > locked_at`）。过期锁由 job-worker 的巡检任务按 `expires_at` 删除，这是基线第 3.6 节允许物理删除的两类之外的第三类，本阶段登记为对基线第 3.6 节的追加：低代码编辑锁属于短生命周期的协作锁，不承载任何业务事实，其清理与 `platform_msg` 的过期幂等键同类。无行级策略。

##### 3.2.16 platform_meta.config_release_mutex（部署级）

单行互斥表，用于串行化发布执行。基线第 3.10 节禁止部分索引，因此不能用带条件的唯一索引表达“同时只有一个执行中的发布单”。

列：id uuid（固定为全零 UUID）、security_level、holder_order_id uuid、acquired_at timestamptz、公共列。

约束与索引：`pk_config_release_mutex`、`ck_config_release_mutex_singleton`（`id = '00000000-0000-0000-0000-000000000000'::uuid`）。执行器以 `select ... for update` 取锁。无行级策略。

##### 3.2.17 platform_meta.brand_profiles（部署级）

对应 PRD 附录乙 U-K-07 经 F-51 批准的首版冻结值，把白标可配置项固定为下表列集。

列：id、security_level、code、product_name text、vendor_display_name text、app_identifier_win text、app_identifier_mac text、app_identifier_ios text、app_identifier_android text、logo_attachment_object_id uuid、splash_attachment_object_id uuid、login_background_attachment_object_id uuid、theme_primary_color text、theme_accent_color text、notify_template_set_code text、signing_identity_ref text、distribution_channel text（APP_STORE、ENTERPRISE_MDM）、store_policy_check_passed_at timestamptz、status text（DRAFT、ACTIVE、SUPERSEDED）、active_slot uuid null、公共列。`active_slot` 是服务端派生列，客户端不得提交；ACTIVE 时固定为全零 UUID，其余状态固定为空。

约束与索引：`pk_brand_profiles`、`ux_brand_profiles_code`、普通唯一键 `ux_brand_profiles_active_slot(active_slot)`、`ix_brand_profiles_status_created_at`、`ck_brand_profiles_distribution_channel`、`ck_brand_profiles_color`（`theme_primary_color ~ '^#[0-9A-Fa-f]{6}$'`，accent 同）、`ck_brand_profiles_status`、NULL-safe `ck_brand_profiles_active_slot`（`active_slot IS NOT DISTINCT FROM CASE WHEN status='ACTIVE' THEN '00000000-0000-0000-0000-000000000000'::uuid ELSE NULL END`）。因此数据库直写也不可能产生两个 ACTIVE，且不使用部分索引。状态机只允许 `DRAFT -> ACTIVE -> SUPERSEDED`；SUPERSEDED 不可恢复或修改，切换品牌必须新建 DRAFT。`signing_identity_ref` 只存 `secret://` 引用，不存密钥材料，照抄基线第 7.2 节。无行级策略。

##### 3.2.18 platform_meta.client_releases（部署级）

列：id、security_level、client text（win、mac、ios、android）、version text、build_no bigint、brand_profile_id uuid、distribution_channel text（从品牌配置冻结，客户端不可另选）、artifact_legal_entity_id uuid null、artifact_attachment_version_id uuid null、artifact_eligible boolean null、store_listing_uri text null、artifact_hash text、artifact_size_bytes bigint、min_supported_version text、is_forced_security_update boolean、rollout_percent smallint、rollout_legal_entity_ids uuid[]、rollout_department_ids uuid[]、released_at timestamptz、withdrawn_at timestamptz、status text（DRAFT、ROLLING_OUT、FULL、WITHDRAWN）、release_notes text、公共列。ENTERPRISE_MDM 发布把制品固定到 `platform_file.attachment_versions` 的一个不可漂移版本，`artifact_eligible` 固定为 true；APP_STORE 发布只固定商店详情 URI，不保存伪造的本地下载地址且该列为空。两种形态都保留构建流水线产出的 SHA-256 与字节数作为发布证据。

约束与索引：`pk_client_releases`、`ux_client_releases_client_version`、`fk_client_releases_brand_profiles`、`fk_client_releases_artifact_identity`（可空复合 FK `(artifact_legal_entity_id,artifact_attachment_version_id,artifact_hash,artifact_size_bytes,artifact_eligible) -> platform_file.attachment_versions(legal_entity_id,id,content_hash,size_bytes,artifact_eligible) ON DELETE RESTRICT`）、`ix_client_releases_status_created_at`、`ck_client_releases_client`、`ck_client_releases_distribution_channel`、`ck_client_releases_rollout_percent`（`between 0 and 100`）、`ck_client_releases_status`、`ck_client_releases_notes_len`（`char_length(release_notes) <= 2000`）、`ck_client_releases_artifact_hash`（64 位小写十六进制）、`ck_client_releases_artifact_size`（大于 0）、NULL-safe `ck_client_releases_locator_shape`：ENTERPRISE_MDM 时两项附件版本列、哈希、大小全非空、`artifact_eligible=true` 且 `store_listing_uri` 为空，APP_STORE 时两项附件版本列与 `artifact_eligible` 全空且 `store_listing_uri` 非空。真实复合外键使错法人、错版本、错哈希、错大小和不可发布父版本均无法写入。另给 `brand_profiles` 建 `(id,distribution_channel)` 候选唯一键并以复合 FK 固定发布行的 `distribution_channel` 必须等于其品牌配置，不能只靠应用复制。商店 URI 写入时必须是 HTTPS、无 userinfo/fragment，并按 client 命中封闭主机白名单；无法用 CHECK 表达的 URI 解析由同事务校验和直 SQL 约束触发器共同强制。无行级策略。

##### 3.2.19 platform_meta.extensions（部署级）

服务端 WASM 插件与桌面端原生插件共用一张登记表，落实规格第 9.3 章“签名、版本锁定、能力声明、最小权限授予和审计要求对以下形态一致”。

列：id、security_level、code、name、kind text（SERVER_WASM、DESKTOP_NATIVE）、version text、publisher_subject text、artifact_legal_entity_id uuid、artifact_attachment_version_id uuid、artifact_hash text、artifact_size_bytes bigint、artifact_eligible boolean not null default true、signature bytea、signature_verified_at timestamptz、capability_manifest jsonb、`manifest_hash text`、resource_limits jsonb、target_platforms text[]（DESKTOP_NATIVE 时取 win、mac 的子集）、`requested_grants jsonb`、`requested_grants_hash text`、status text（REGISTERED、PENDING_APPROVAL、APPROVED、REJECTED、ENABLED、DISABLED、REVOKED）、disabled_reason text、consecutive_failures int、`approval_legal_entity_id uuid`、`approval_scenario text`、`submitted_by uuid`、`submitted_at timestamptz`、`approval_ref uuid`、`approval_chain_id uuid`、`approval_chain_version_no int`、`approval_definition_digest bytea`、`approval_artifact_hash text`、`approval_manifest_hash text`、`approval_requested_grants_hash text`、`approved_by uuid`、`approved_at timestamptz`、`rejected_by uuid`、`rejected_at timestamptz`、`rejected_reason text`、公共列。登记时把上传对象的当时 CURRENT 版本解析成这两个不可变版本定位列并把 `artifact_eligible` 固定为 true；`manifest_hash` 是规范化 `capability_manifest` 的 SHA-256 小写十六进制。`request-approval` 把服务端规范化、按 `(capability,scope_key)` 排序且字段码排序去重的申请数组写入 `requested_grants`，其 SHA-256 写入 `requested_grants_hash`；两者不得由客户端提供摘要。后续同一附件对象发布新版本不会让已批准扩展静默换包。

约束与索引：`pk_extensions`、`ux_extensions_code_version`、候选唯一键 `ux_extensions_id_kind(id,kind)`、`ux_extensions_approval_identity(id,kind,approval_legal_entity_id,approval_ref)`、`fk_extensions_artifact_identity`（复合 FK `(artifact_legal_entity_id,artifact_attachment_version_id,artifact_hash,artifact_size_bytes,artifact_eligible) -> platform_file.attachment_versions(legal_entity_id,id,content_hash,size_bytes,artifact_eligible) ON DELETE RESTRICT`）、`fk_extensions_approval_legal_entity`、`fk_extensions_approval_chain`、`fk_extensions_submitted_by_grant`、`fk_extensions_approved_by_grant`、`fk_extensions_rejected_by_grant`（链与用户复合外键形状逐字复用 `config_packages`）、`ix_extensions_status_created_at`、`ck_extensions_kind`、`ck_extensions_status`、`ck_extensions_artifact_hash`、`ck_extensions_manifest_hash`、`ck_extensions_requested_grants_hash`（三种 hash 均为 64 位小写十六进制）、`ck_extensions_artifact_size`（大于 0）、`ck_extensions_artifact_eligible`（恒为 true）、`ck_extensions_consecutive_failures`（`>= 0`）、`ck_extensions_approval_scenario`（非空时只能为 `EXTENSION_ENABLE`）、`ck_extensions_no_self_approval`（`approved_by is null or approved_by <> submitted_by`）、`ck_extensions_disabled_reason`（status 为 DISABLED 时非空，其他状态为空）、`ck_extensions_approval_shape`：REGISTERED 时申请与审批证据全空；PENDING_APPROVAL、APPROVED、REJECTED、ENABLED、DISABLED、REVOKED 时法人、场景、申请人、提交时间、流程、链 id/版本/digest、三项审批摘要及 requested_grants/hash 全非空，且 `approval_artifact_hash=artifact_hash`、`approval_manifest_hash=manifest_hash`、`approval_requested_grants_hash=requested_grants_hash`；PENDING_APPROVAL 两组结论全空，REJECTED 只允许拒绝三列非空，APPROVED/ENABLED/DISABLED/REVOKED 只允许批准两列非空。真实复合外键使错法人、错版本、错哈希、错大小、不可发布父版本、错审批链或错审批主体均无法写入。

迁移同文件安装 `BEFORE UPDATE` 守卫 `platform_meta.assert_extension_identity_immutable()`：`code/version/kind/artifact_legal_entity_id/artifact_attachment_version_id/artifact_hash/artifact_size_bytes/artifact_eligible/publisher_subject/signature/capability_manifest/manifest_hash/resource_limits/target_platforms` 任一改变均拒绝；`requested_grants/requested_grants_hash` 只允许在 REGISTERED→PENDING_APPROVAL 的一次迁移中同时写入，此后不可变；审批证据只能由命名提交命令与 owner callback 写入。升级必须登记新的 `(code,version)`。REJECTED 与 REVOKED 均无出边；DISABLED 只允许重新 ENABLED 或 REVOKED；ENABLED 只允许 DISABLED 或 REVOKED；REGISTERED 只能进入 PENDING_APPROVAL，PENDING_APPROVAL 只能由 owner callback 进入 APPROVED 或 REJECTED，APPROVED 只能进入 ENABLED、DISABLED 或 REVOKED。无行级策略。

##### 3.2.20 platform_meta.extension_capability_grants（部署级）

列：id、security_level、extension_id uuid、extension_kind text、capability text、object_type text、field_codes text[]、`scope_key text not null`、`active_slot smallint null`、`approval_legal_entity_id uuid`、granted_by uuid、`approval_ref uuid`、granted_at timestamptz、revoked_at timestamptz、公共列。`extension_kind`、`approval_legal_entity_id` 与 `approval_ref` 均由父扩展冻结且客户端不得提交。`scope_key` 是服务端生成的规范键：READ_OBJECT_FIELDS 取规范化 object_type，其他能力固定取 `-`；`active_slot` 在有效授予时固定为 1，撤销时为空。

capability 取值封闭为 4 项，是首版扩展能力集的全部：READ_OBJECT_FIELDS（读取由调用方按本授予裁剪后传入的字段）、COMPUTE_ONLY（纯计算，无输入裁剪之外的任何能力）、DEVICE_PRINTER（桌面端打印机）、DEVICE_SMARTCARD（桌面端 USB Key 与智能卡）。网络、文件、密钥、数据库四类能力在此不存在可表达的取值，落实规格第 9.3 章“插件默认没有网络、文件、密钥或业务数据权限”。

约束与索引：`pk_extension_capability_grants`、复合 `fk_extension_capability_grants_extension_approval(extension_id,extension_kind,approval_legal_entity_id,approval_ref) -> extensions(id,kind,approval_legal_entity_id,approval_ref) ON DELETE RESTRICT`、`fk_extension_capability_grants_granted_by(approval_legal_entity_id,granted_by) -> platform_authz.user_legal_entity_grants(legal_entity_id,user_id) ON DELETE RESTRICT`、普通唯一键 `ux_extension_capability_grants_active(extension_id,capability,scope_key,active_slot)`、`ix_extension_capability_grants_extension_id_created_at`、`ck_extension_capability_grants_capability`、`ck_extension_capability_grants_device_kind`（DEVICE_PRINTER/DEVICE_SMARTCARD 时必须 `extension_kind='DESKTOP_NATIVE'`；其余两项允许两种 kind）、NULL-safe `ck_extension_capability_grants_scope_shape`（READ_OBJECT_FIELDS 必须有非空 object_type、`scope_key=object_type` 与非空、按字典序排序且去重后的 field_codes；其余三项的 object_type 为空、field_codes 为空数组且 `scope_key='-'`）、NULL-safe `ck_extension_capability_grants_active_slot`（`active_slot IS NOT DISTINCT FROM CASE WHEN revoked_at IS NULL THEN 1::smallint ELSE NULL END`）。复合父键同时固定 kind、审批法人和 `approval_ref`，普通唯一键借助非空 active_slot 保证同一扩展/能力/范围至多一条有效授予，历史撤销行因 NULL 可重复而保留且可重新授予。应用仍在同一事务复验并提供领域错误。无行级策略。

同一 `V20261022091000__platform_meta_extensions.sql` 还安装 `platform_meta.assert_extension_enable_graph_consistent()` 约束触发器，并以 `DEFERRABLE INITIALLY DEFERRED` 挂在 `extensions` 与 `extension_capability_grants` 的 INSERT/UPDATE/DELETE 上。提交点必须满足：ENABLED 的扩展已经 APPROVED 且其全部审批证据仍闭合；有效 grants 的规范化数组逐字等于 `requested_grants`、其重算摘要等于 `requested_grants_hash`，且每一项都被 `capability_manifest` 包含、没有额外或缺失；APPROVED/DISABLED 可保留同一组有效 grants，撤销任一有效 grant 的事务必须先把父扩展置 DISABLED；REJECTED 与 REVOKED 没有有效 grant；任一 grant 的 `approval_ref` 与批准人都必须等于父扩展的审批证据。这样 direct SQL 也不能制造“ENABLED 但无审批”“超 manifest 授权”“半套授权”或数据库状态与加载状态分裂。

##### 3.2.21 platform_meta.extension_invocations（法人级，仅追加）

列：id、legal_entity_id uuid、security_level、data_scope_tags text[]、extension_id uuid、caller_process text（core、worker）、caller_user_id uuid、caller_device_id uuid、entry_point text、input_hash text、output_hash text、fuel_consumed bigint、memory_peak_bytes bigint、duration_ms int、outcome text、error_text text、occurred_at timestamptz、created_at、created_by。调用审计没有冲销语义，不设 `reverses_id`。

outcome 取值：OK、TRAP、TIMEOUT、FUEL_EXHAUSTED、MEMORY_LIMIT、CAPABILITY_DENIED、HOST_ERROR、THROTTLED。

约束与索引：`pk_extension_invocations`；`extension_id` 单列真实外键指向部署级 `platform_meta.extensions(id)`；`(legal_entity_id,caller_user_id)` 真实复合外键指向 `platform_authz.user_legal_entity_grants(legal_entity_id,user_id)`；`caller_device_id` 单列真实外键指向 `platform_core.user_devices(id)`，写入事务另校验设备属于 caller_user_id 与当前法人；`ix_extension_invocations_legal_entity_id_created_at`、`ix_extension_invocations_extension_id_occurred_at`、`ck_extension_invocations_outcome`。

行级策略按基线第 3.8 节模板生成 `rls_extension_invocations_le`，并 `enable` 加 `force`。`error_text` 在写入前经 `foundation::Redacted` 处理，不得包含插件输入的明文。

##### 3.2.22 platform_meta.client_bootstrap_dispatches（法人级，仅追加）

落实规格第 7.4 章“自定义对象下发到客户端时随会话一并下发对象结构、字段密级、权限策略和声明式规则版本，下发范围可审计”。

列：id、legal_entity_id uuid、security_level、data_scope_tags text[]、user_id uuid、device_id uuid、client text、bootstrap_hash text、custom_object_codes text[]、rule_versions jsonb、ui_layout_versions jsonb、capability_snapshot_hash text、brand_profile_code text、dispatched_at timestamptz、created_at、created_by。下发审计没有冲销语义，不设 `reverses_id`。

约束与索引：`pk_client_bootstrap_dispatches`；`(legal_entity_id,user_id)` 真实复合外键指向 `platform_authz.user_legal_entity_grants(legal_entity_id,user_id)`；`device_id` 单列真实外键指向 `platform_core.user_devices(id)`，写入事务另校验设备属于 user_id 与当前法人；`brand_profile_code` 单列真实外键指向部署级 `platform_meta.brand_profiles(code)`；`ix_client_bootstrap_dispatches_legal_entity_id_created_at`、`ix_client_bootstrap_dispatches_user_id_dispatched_at`、`ck_client_bootstrap_dispatches_client`。行级策略按模板生成 `rls_client_bootstrap_dispatches_le`。

#### 3.3 ext schema 下自定义对象物理表的生成模板

每个自定义对象生成一张 `ext.<code>` 表，由迁移生成器统一产出，不允许手写变体。列顺序固定为基线第 4 节公共列在前、单据或档案专属列居中、自定义字段在后。

```sql
create table ext.<code> (
  id uuid not null,
  legal_entity_id uuid not null,
  security_level smallint not null default <对象密级>,
  data_scope_tags text[] not null default '{}',
  row_version bigint not null default 1,
  created_at timestamptz not null default now(),
  created_by uuid not null,
  updated_at timestamptz not null default now(),
  updated_by uuid not null,
  -- 单据类追加
  doc_no text not null,
  status text not null,
  -- 档案类追加
  code text not null,
  is_active boolean not null default true,
  deactivated_at timestamptz null,
  -- 自定义字段按 custom_fields 顺序追加，一律可空
  constraint pk_<code> primary key (id),
  constraint ux_<code>_legal_entity_id_id unique (legal_entity_id, id),
  constraint fk_<code>_created_by_grant foreign key (legal_entity_id, created_by)
    references platform_authz.user_legal_entity_grants (legal_entity_id, user_id) on delete restrict,
  constraint fk_<code>_updated_by_grant foreign key (legal_entity_id, updated_by)
    references platform_authz.user_legal_entity_grants (legal_entity_id, user_id) on delete restrict,
  constraint ck_<code>_status check (status in (...))
);
alter table ext.<code> enable row level security;
alter table ext.<code> force row level security;
create policy rls_<code>_le on ext.<code>
  using (legal_entity_id = nullif(current_setting('app.legal_entity_id', true), '')::uuid)
  with check (legal_entity_id = nullif(current_setting('app.legal_entity_id', true), '')::uuid);
create index ix_<code>_legal_entity_id_created_at on ext.<code> (legal_entity_id, created_at);
create unique index ux_<code>_legal_entity_id_doc_no on ext.<code> (legal_entity_id, doc_no); -- 单据类
```

自定义索引在此之后以 `create index concurrently` 追加。`REFERENCE` 字段或 ONE_TO_ONE/ONE_TO_MANY 关系只要固定指向 `ext.<target>`，生成器就必须产出 `(legal_entity_id,<reference_column>) REFERENCES ext.<target>(legal_entity_id,id) ON DELETE RESTRICT`，目标表的上述候选键不得省略；同一表自引用也使用该复合形状。多对多关系生成 `ext.<a>_<b>_links` 表，列为 `id`、`legal_entity_id`、公共列、`<a>_id`、`<b>_id`，并建立 `UNIQUE(legal_entity_id,id)`、`(legal_entity_id,<a>_id) -> ext.<a>(legal_entity_id,id)`、`(legal_entity_id,<b>_id) -> ext.<b>(legal_entity_id,id)` 两条真实复合外键及 created_by/updated_by 到授权表的两条真实复合外键，全部 `ON DELETE RESTRICT`；`ux_<a>_<b>_links_pair` 在 `(legal_entity_id,<a>_id,<b>_id)` 上。固定指向一个内置法人级对象的 REFERENCE 同样生成到其 `(legal_entity_id,id)` 的真实复合外键；只有目标在运行期按 ext 关系元数据动态解析、无法在发布版本冻结为单表的关系，才属于封闭 ext 例外并由 owner 同事务校验，不得把任意跨 schema 引用概括为“不建外键”。

#### 3.4 迁移编号与顺序

第 1 至 13 号迁移文件路径为 `db/migrations/platform_meta/`，第 14 号登记回填路径为 `db/migrations/platform_core/`；迁移历史统一落在全局唯一的 `platform_core.schema_history`。执行顺序由单一全局 Runner 按文件版本号全序排定，不存在任何模块顺序声明文件。下表版本、slug 与路径已由 `docs/migration-catalog.md` 全局冻结，实施必须逐字使用，不得在开工时重取时间戳、复用或重编号。

| 序 | 文件名 | 内容 |
|---|---|---|
| 1 | V20261022090000__platform_meta_custom_object_model.sql | custom_objects、custom_fields、custom_relations、custom_indexes、custom_views 五表 |
| 2 | V20261022090100__platform_meta_ddl_plan.sql | ddl_plans、ddl_plan_steps |
| 3 | V20261022090200__platform_meta_ui_layouts.sql | ui_layouts |
| 4 | V20261022090300__platform_meta_client_capability_values.sql | client_capability_values 建表 |
| 5 | V20261022090400__platform_meta_backfill_capability_matrix.sql | 72 行种子数据，逐格照抄规格第 6.2 章 |
| 6 | V20261022090500__platform_meta_alter_config_package.sql | 对阶段 3b 已建的 config_packages/items 做列扩展：只追加自动测试相关列、把 status 放宽为十一态，并在 Stage 3 同序 Rust/CHECK 18（含 `LICENSE_GRANT`、`MODULE_PACKAGE`）尾部追加 `MCP_CONNECTOR`、`MCP_MANIFEST_VERSION`，同批更新 Rust `ItemKind::ALL` 与 DB CHECK 到终态 20；不改写既有行，不重复审批列或 Stage3 `accepted_trust_bundle_sha256`，不创建只属于093300的父候选键，不另建低版本 ALTER |
| 7 | V20261022090600__platform_meta_config_release.sql | 对阶段 3b 已建的 config_release_orders 做列扩展与状态扩展（本阶段第 3.2.12 节列集相对最小通道的增量），新建 config_release_steps、config_autotest_runs、config_edit_locks、config_release_mutex 四表，并按下述 exact catalog 幂等 seed Stage 13 全部 30 个 permission_items 与 12 个 object_scope_bindings；不 seed role grants |
| 8 | V20261022090700__platform_meta_backfill_release_mutex_row.sql | 互斥表单行种子，`created_by` 取 `foundation::SYSTEM_PRINCIPAL_ID` |
| 9 | V20261022090800__platform_meta_brand_profiles.sql | brand_profiles |
| 10 | V20261022090900__platform_meta_client_releases.sql | client_releases |
| 11 | V20261022091000__platform_meta_extensions.sql | extensions、extension_capability_grants、不可变守卫与 `assert_extension_enable_graph_consistent()` 延迟图；审批证据、规范 grant scope 与活动槽一次建齐 |
| 12 | V20261022091100__platform_meta_extension_invocations.sql | extension_invocations 含 RLS |
| 13 | V20261022091200__platform_meta_client_bootstrap_dispatches.sql | client_bootstrap_dispatches 含 RLS |
| 14 | V20261022091300__platform_core_backfill_stage13_unpoliced_table_registry.sql | 按基线第 3.8 节的正向登记制，向阶段 2 交付的 `platform_core.unpoliced_table_registry` 写入本阶段新建的 17 张不带法人列的表各一行 |

090600 的 30 个固定 permission 按 code UTF-8 lexicographic 顺序占用连续 UUID `...0320`–`...0349`，exact catalog 如下。所有行另固定 `module_code='platform'`、`function_point=code`、`description=null`；`allowed_actions` 按全局动作顺序 canonical 保存。迁移逐行 `INSERT ... ON CONFLICT DO NOTHING` 后必须重读并断言 id/code/module/function/actions/object/description 全字段相等，任一漂移使迁移失败，绝不更新既有行或 seed 任一 `role_permission_grants`。

| id | code | allowed_actions | object_type |
|---|---|---|---|
| `00000000-0000-7000-8000-000000000320` | `brand.profile.manage` | `[CREATE,UPDATE]` | `platform.brand_profiles` |
| `00000000-0000-7000-8000-000000000321` | `brand.profile.view` | `[VIEW]` | `platform.brand_profiles` |
| `00000000-0000-7000-8000-000000000322` | `client.release.manage` | `[CREATE,UPDATE]` | `platform.client_releases` |
| `00000000-0000-7000-8000-000000000323` | `ext.extension.approve` | `[APPROVE]` | `platform.extensions` |
| `00000000-0000-7000-8000-000000000324` | `ext.extension.enable` | `[UPDATE]` | `platform.extensions` |
| `00000000-0000-7000-8000-000000000325` | `ext.extension.register` | `[CREATE,SUBMIT]` | `platform.extensions` |
| `00000000-0000-7000-8000-000000000326` | `ext.extension.view` | `[VIEW]` | `platform.extensions` |
| `00000000-0000-7000-8000-000000000327` | `lowcode.config.edit` | `[CREATE,UPDATE]` | `platform.config_edit_locks` |
| `00000000-0000-7000-8000-000000000328` | `lowcode.config_package.approve` | `[APPROVE]` | `platform.config_packages` |
| `00000000-0000-7000-8000-000000000329` | `lowcode.config_package.autotest` | `[UPDATE]` | `platform.config_packages` |
| `00000000-0000-7000-8000-000000000330` | `lowcode.config_package.create` | `[CREATE]` | `platform.config_packages` |
| `00000000-0000-7000-8000-000000000331` | `lowcode.config_package.import` | `[CREATE]` | `platform.config_packages` |
| `00000000-0000-7000-8000-000000000332` | `lowcode.config_package.sign` | `[UPDATE]` | `platform.config_packages` |
| `00000000-0000-7000-8000-000000000333` | `lowcode.config_package.submit` | `[SUBMIT]` | `platform.config_packages` |
| `00000000-0000-7000-8000-000000000334` | `lowcode.config_package.view` | `[VIEW]` | `platform.config_packages` |
| `00000000-0000-7000-8000-000000000335` | `lowcode.config_release.execute` | `[UPDATE]` | `platform.config_release_orders` |
| `00000000-0000-7000-8000-000000000336` | `lowcode.config_release.submit` | `[SUBMIT]` | `platform.config_release_orders` |
| `00000000-0000-7000-8000-000000000337` | `lowcode.config_release.view` | `[VIEW]` | `platform.config_release_orders` |
| `00000000-0000-7000-8000-000000000338` | `lowcode.custom_field.create` | `[CREATE]` | `platform.custom_fields` |
| `00000000-0000-7000-8000-000000000339` | `lowcode.custom_index.create` | `[CREATE]` | `platform.custom_indexes` |
| `00000000-0000-7000-8000-000000000340` | `lowcode.custom_object.create` | `[CREATE]` | `platform.custom_objects` |
| `00000000-0000-7000-8000-000000000341` | `lowcode.custom_object.modify` | `[UPDATE]` | `platform.custom_objects` |
| `00000000-0000-7000-8000-000000000342` | `lowcode.custom_object.plan_ddl` | `[SUBMIT]` | `platform.ddl_plans` |
| `00000000-0000-7000-8000-000000000343` | `lowcode.custom_object.retire` | `[UPDATE]` | `platform.custom_objects` |
| `00000000-0000-7000-8000-000000000344` | `lowcode.custom_object.view` | `[VIEW]` | `platform.custom_objects` |
| `00000000-0000-7000-8000-000000000345` | `lowcode.rule.evaluate` | `[VIEW]` | `platform.config_package_items` |
| `00000000-0000-7000-8000-000000000346` | `lowcode.ui_layout.create` | `[CREATE]` | `platform.ui_layouts` |
| `00000000-0000-7000-8000-000000000347` | `lowcode.ui_layout.modify` | `[UPDATE]` | `platform.ui_layouts` |
| `00000000-0000-7000-8000-000000000348` | `lowcode.ui_layout.preview` | `[VIEW]` | `platform.ui_layouts` |
| `00000000-0000-7000-8000-000000000349` | `lowcode.ui_layout.view` | `[VIEW]` | `platform.ui_layouts` |

同一迁移还按 object_type lexicographic 顺序占用连续 binding UUID `...0520`–`...0531`。12 行的 `owner_user_col,owning_dept_col,project_col,customer_col` 恰为 SQL NULL，`security_level_col='security_level'`；`schema_name='platform_meta'`，table 如下。`ON CONFLICT DO NOTHING` 后逐字段断言 id/object/schema/table/四锚/security 列全等，不能用缺 binding 的应用 fallback 代替。

| id | object_type | table_name |
|---|---|---|
| `00000000-0000-7000-8000-000000000520` | `platform.brand_profiles` | `brand_profiles` |
| `00000000-0000-7000-8000-000000000521` | `platform.client_releases` | `client_releases` |
| `00000000-0000-7000-8000-000000000522` | `platform.config_edit_locks` | `config_edit_locks` |
| `00000000-0000-7000-8000-000000000523` | `platform.config_package_items` | `config_package_items` |
| `00000000-0000-7000-8000-000000000524` | `platform.config_packages` | `config_packages` |
| `00000000-0000-7000-8000-000000000525` | `platform.config_release_orders` | `config_release_orders` |
| `00000000-0000-7000-8000-000000000526` | `platform.custom_fields` | `custom_fields` |
| `00000000-0000-7000-8000-000000000527` | `platform.custom_indexes` | `custom_indexes` |
| `00000000-0000-7000-8000-000000000528` | `platform.custom_objects` | `custom_objects` |
| `00000000-0000-7000-8000-000000000529` | `platform.ddl_plans` | `ddl_plans` |
| `00000000-0000-7000-8000-000000000530` | `platform.extensions` | `extensions` |
| `00000000-0000-7000-8000-000000000531` | `platform.ui_layouts` | `ui_layouts` |

第 14 号是回填文件，其主要创建对象是 `platform_core.unpoliced_table_registry` 的登记行，按裁定通则第五条落在 `db/migrations/platform_core/` 目录下，版本号晚于本阶段全部建表迁移，故列在最后；slug 以 `backfill_` 开头，回退说明为按 `schema_name` 与 `table_name` 两列删除本阶段登记的 17 行。五列体例照抄阶段 4 第 29 号迁移，即 schema、table、准入判据、隔离承接入口与 `rls_matrix` 用例标识五列按阶段 2 冻结的列集填写，准入判据一列 17 行一律取 `SAME_FOR_ALL_ENTITIES`，取值名以阶段 2 冻结的枚举为准。`config_packages`、`config_package_items` 与 `config_release_orders` 三张由阶段 3b 建表，其登记行按同一正向登记制随阶段 3b 的建表迁移插入，本阶段不重复登记，以免撞 `ux_unpoliced_table_registry_schema_table` 的两列唯一约束。本阶段新建的另两张表 `extension_invocations` 与 `client_bootstrap_dispatches` 带 `legal_entity_id` 并已建行级策略，不进本登记。

每个文件头部带 `-- rollback:` 段。本阶段不再追加任何 `ext` schema 级授权迁移：`ext` schema 本身、其属主角色以及对 `ep_app_rw` 与 `ep_analyst_ro` 的 USAGE 与表默认权限由阶段 2 按裁定 C-01 随二十四个 schema 一次建立，`ep_migrator` 在阶段 2 已被授予全部 `ep_mod_*` 角色成员资格，因而在 `ext` 下具备 DDL 权限；自定义对象表的数据读写授权由第 4.3 节的 DDL 计划在 `create policy` 之后逐表发出，不经默认权限，理由见第 4.3 节边界条件。

---

### 4. 领域模型与关键算法

#### 4.1 核心结构体与枚举

以下类型位于 `ep-platform-meta` 与 `ep-platform-release`，均只用 foundation 类型，不含任何 IO。凡已在 `ep-foundation` 冻结的类型本阶段一律引用不再定义，见裁定 A-01、A-03 与 A-20。

```rust
// ep-platform-meta::value
pub enum FieldDataType { Integer, Decimal { precision: u8, scale: u8 }, Float, Boolean,
    String { max_len: u32 }, Text { max_len: u32 }, Date, Timestamp,
    Enum { values: Vec<EnumValue> }, Reference { target: ObjectType }, Json }

pub enum IndexKind { Single, Composite, Unique }
pub enum Cardinality { OneToOne, OneToMany, ManyToMany }
pub enum CapabilityValue { Full, Simplified, ViewOnly, NotApplicable }
// ClientKind 由阶段 1 按裁定 A-03 冻结，CapabilityDomain 与 ActionClass 由阶段 1 按裁定 A-20 冻结，
// 三者均位于 ep-foundation，本阶段只 use 不再定义

// ep-platform-meta::model
pub struct CustomObject { id: Id, code: ObjectCode, security_level: SecurityLevel,
    is_document: bool, doc_type_code: Option<DocTypeCode>, fields: Vec<CustomField>,
    indexes: Vec<CustomIndex>, definition_version: u64, status: ObjectStatus }

// ep-platform-meta::rule
pub enum RuleExpr { Lit(RuleValue), FieldRef(FieldPath), LineAgg{ kind: AggKind, path: FieldPath },
    Cmp{ op: CmpOp, l: Box<RuleExpr>, r: Box<RuleExpr> },
    Logic{ op: LogicOp, args: Vec<RuleExpr> },
    Arith{ op: ArithOp, l: Box<RuleExpr>, r: Box<RuleExpr> },
    Between{ v: Box<RuleExpr>, lo: Box<RuleExpr>, hi: Box<RuleExpr> },
    InSet{ v: Box<RuleExpr>, set: Vec<RuleValue> },
    IsNull(Box<RuleExpr>), Today, PeriodOf(Box<RuleExpr>),
    WasmCall{ extension_code: String, entry_point: String, args: Vec<RuleExpr> } }

// ep-platform-release::model
pub enum PackageStatus { Draft, PendingAutotest, TestFailed, TestPassed, PendingApproval,
    Rejected, Approved, SignedPendingRelease, Released, RolledBack, Superseded }
pub enum ReleaseOrderStatus { Submitted, Approved, Rejected, Queued, Executing,
    Succeeded, Failed, Compensated, Cancelled }
pub enum ItemKind { /* 终态 20 项，固定顺序见 3.2.11；Stage 3 Rust/DB 前18含 LICENSE_GRANT/MODULE_PACKAGE，本阶段 090500 在尾部追加两个 MCP 项并同批更新 Rust/DB */ }
pub enum ChangeKind { Add, Modify, Remove }
pub enum AutotestSuiteId { SchemaValidation, ImpactAnalysis, RlsMatrix, RolePreview,
    FlowSemantics, ReportPermission, CapabilityMatrix, SodCheck, RuleSemantics }
pub trait ConfigAutotestSuite { fn id(&self) -> AutotestSuiteId; /* read-only run */ }
pub struct ConfigAutotestRegistry { /* exact set of nine trait objects */ }
pub trait ConfigAutotestExecutor { /* execute/recover one durable batch */ }
```

`AutotestSuiteId`、三个 SPI 与输入/报告 DTO 一律位于 `ep-platform-release`，不含 SQL、HTTP 或属主模型实现。九个实现仅在 `apps/job-worker/src/wiring/autotest.rs` 注册；启动断言注册表与上述九项是精确集合，缺项、重项、额外项任一成立均拒绝 job-worker 启动。属主映射为：meta 四套（SCHEMA_VALIDATION、IMPACT_ANALYSIS、CAPABILITY_MATRIX、RULE_SEMANTICS）、authz 三套（RLS_MATRIX、ROLE_PREVIEW、SOD_CHECK）、flow 一套、reporting 一套。依赖只从这四个属主指向 release，不形成 release 反向边。

#### 4.2 配置包状态机

状态与流转逐条对应 PRD 第 10.4.1 节的表，守卫条件如下。

| 起点 | 终点 | 触发 | 守卫条件 |
|---|---|---|---|
| Draft | PendingAutotest | 提交自动测试 | 包内容项数在 1 至 2000 之间；包体积不超过 64 MiB；每项 `item_hash` 按第 4.7 节 ADD/MODIFY-after、REMOVE-before 的 RFC 8785 JCS/SHA-256 lowerhex 重算一致且未对 null 求摘要 |
| PendingAutotest | TestPassed | 平台 | 当前 `active_autotest_batch_id` 的 9 个 suite 均为 FINISHED，outcome 全为 PASSED 或合法 SKIPPED |
| PendingAutotest | TestFailed | 平台 | 当前 `active_autotest_batch_id` 的 9 行全部 FINISHED，且至少一行 outcome 为 FAILED；不得在首个语义失败时提前跳转，以免丢失完整报告 |
| TestFailed | Draft | 配置管理员修改 | 持有该包全部内容项的编辑锁 |
| TestPassed | PendingApproval | 提交审批 | 包已锁定不可再改；`min_platform_version` 不高于当前版本；以 `CONFIG_RELEASE` 调用阶段 4 共用解析器得到唯一非空活动链，申请人不在任一节点展开集合 |
| PendingApproval | Approved | `ConfigReleaseApprovalCallback` 收到流程全部节点通过 | 回调逐项验证同一包、同一 `approval_ref`、`CONFIG_RELEASE`、链 id/版本/digest、当前 `content_version/content_hash` 与非自审；端点不得直接迁移 |
| PendingApproval | Rejected | `ConfigReleaseApprovalCallback` 收到任一节点驳回 | 与通过分支相同的绑定和非自审验证；端点不得直接迁移 |
| Approved | SignedPendingRelease | 平台签名 | 签名密钥可解引用；`content_hash` 与包实际内容一致 |
| SignedPendingRelease | Released | 执行发布单 | 执行模式与 `ddl_plans.execution_mode` 一致；若为 MAINTENANCE_WINDOW 则必须落在已登记的停机窗口内；互斥锁取得成功 |
| Released | RolledBack | 回退发布单 | 回退目标为上一 Released 包；该目标在保留窗口内（最近 10 个且不早于 180 天）；回退发布单本身已完成审批 |
| Released | Superseded | 后续版本发布 | **仅普通配置包**：同一普通 lineage 的旧包自动置位，与新包置 Released 同事务；包含 `LICENSE_GRANT\|MODULE_PACKAGE` 时该边不存在 |

非法迁移一律返回 `BUSINESS_CONFLICT` 与 `PLATFORM.CONFIG_PACKAGE.*` 的对应码，不静默忽略。

`LICENSE_GRANT|MODULE_PACKAGE` special 包的 `RELEASED` 是永久终态：首次 RELEASE 后不得进入 `SUPERSEDED|ROLLED_BACK`，也不参与上表普通 lineage 自动替代。后续 grant/revoke/module action 各自新建另一份仍为 RELEASED 的单项包，多份 special RELEASED 同时存在是正确历史形状；current/history/superseded grant 与 current module 只由 `license_grants/module_registrations` 投影及 source FK 表达，不得改 config package status 伪装。

`CONFIG_RELEASE` 是具名审批场景，默认链严格复用阶段 4 的 `ApprovalChainResolver::resolve_active_chain` 与 `ApprovalDefaultCatalog`，默认节点为 `ROLE:SECURITY_ADMIN`；本阶段不读取 `approval_chains` 自行挑链，也不复制角色映射。提交审批在一个事务内完成：special submit 先取第 4.6 节 `LICENSE_CURRENT_EXCLUSIVE`，从首发候选或首次 RELEASED grant history 派生 `governance_context_id`，证明当前受信 session/operator 对该法人有 `lowcode.config_package.submit`，请求头若带法人则要求相等，并断言此前 `approval_legal_entity_id` 为空；再按 canonical tuple 锁包/项、重算 content hash/version、解析冻结 chain。成功时同一事务首次写 `approval_legal_entity_id=governance_context_id` 与其余审批证据、把包置 PENDING_APPROVAL、创建流程实例/首任务、幂等 finish、通知与审计终结。派生/history 不唯一、授权或请求头不符、预填 approval 列、无链、多链、空节点/审批人、自审均整笔回滚，包仍 TEST_PASSED 且流程/任务/通知/Outbox/审计零新增。普通包也经共享 submit 入口先取 exclusive，但法人沿其普通受信请求上下文。

批准与驳回 HTTP 路径只是 `platform_flow.process_tasks` 完成命令的便利别名：请求必须带 `task_id`，处理器调用标准任务完成端口；它只写流程任务/步骤，不直接 `UPDATE config_packages`。typed command 在进入事务前固定分支：approve 选择第 4.6 节 `LICENSE_CURRENT_EXCLUSIVE`，在 `try_begin` 或锁任何 task/instance/package 前取得；reject 是唯一 ConfigRelease 写结论例外，固定 `NONE`，不得在无锁事务查询包后从 approve 改判或反向切换。流程引擎在同一事务的 owner callback 阶段调用唯一 `ConfigReleaseApprovalCallback`，再按既定顺序锁包、实例与任务并验证该任务属于包内 `approval_ref`。回调要求包仍为 PENDING_APPROVAL，实例 subject 是同一 package id，场景为 `CONFIG_RELEASE`，`approval_ref`、法人、发起人、chain id/version/definition digest 均与包证据相等，`content_version/content_hash` 仍等于提交快照，节点严格按 node_no 完成，结论操作者不是 submitted_by 且属于当前节点冻结集合。全部通过才由回调写 APPROVED、approved_by/approved_at；任一驳回才由回调写 REJECTED、rejected_by/rejected_at/rejected_reason。任一绑定、摘要、版本、顺序或自审校验失败都回滚任务完成与包迁移，包保持 PENDING_APPROVAL并产生安全告警；不得另设 Outbox 消费者事后“猜测”状态。

#### 4.2.1 九套自动测试与耐久派发

适用集合固定如下；`SKIPPED` 的唯一合法条件是配置包的 `ItemKind` 集合与该行集合交集为空，否则必须执行并产出 `PASSED` 或 `FAILED`。

| suite | 实现 crate | 适用 ItemKind |
|---|---|---|
| SCHEMA_VALIDATION | ep-platform-meta | 终态全部 20 种 |
| IMPACT_ANALYSIS | ep-platform-meta | 终态全部 20 种 |
| RLS_MATRIX | ep-platform-authz | CUSTOM_OBJECT、CUSTOM_FIELD、CUSTOM_RELATION、CUSTOM_VIEW、MCP_CONNECTOR、MCP_MANIFEST_VERSION |
| ROLE_PREVIEW | ep-platform-authz | AUTHZ_ROLE、AUTHZ_POLICY、AUTHZ_FIELD_GRANT |
| FLOW_SEMANTICS | ep-platform-flow | FLOW_DEFINITION |
| REPORT_PERMISSION | ep-app-reporting | REPORT_DEFINITION、METRIC_DEFINITION、DASHBOARD_DEFINITION、PRINT_TEMPLATE |
| CAPABILITY_MATRIX | ep-platform-meta | CUSTOM_OBJECT、CUSTOM_FIELD、UI_LAYOUT、RULE、MCP_CONNECTOR、MCP_MANIFEST_VERSION |
| SOD_CHECK | ep-platform-authz | AUTHZ_ROLE、AUTHZ_POLICY |
| RULE_SEMANTICS | ep-platform-meta | RULE |

受理算法固定为一个事务：以 `FOR UPDATE` 锁住 DRAFT 包，生成新的 batch UUID，把包置 `PENDING_AUTOTEST`，写 `active_autotest_batch_id`、`autotest_attempts=0`、立即可用的 `autotest_available_at` 并清空旧租约/错误，插入恰好九条 `QUEUED` 运行行，九行 `available_at` 与包级字段都取同一数据库当前时刻，执行幂等 `finish`，最后写审计终结批并提交；响应中的 `run_ids` 恰为这九行。无需且不得写自动测试 Outbox 事件，审计终结批之后只允许提交。

job-worker 以 `status='PENDING_AUTOTEST'`、包级 `autotest_available_at` 已到期、且租约为空或过期为条件，使用 `FOR UPDATE SKIP LOCKED` 领取一个 batch，设置 `autotest_locked_by/autotest_locked_until`；无任务时轮询间隔从 200 ms 退避至 2 s。所有到期、租约和退避比较的数据库当前时刻固定取 PostgreSQL `clock_timestamp()`，不得取 worker 操作系统时钟或事务起点固定的 `now()`。租约取值是 job-worker 内部冻结常量，不进部署配置：`AUTOTEST_LOCK_LEASE_SECONDS=60`、`AUTOTEST_LOCK_HEARTBEAT_SECONDS=20`。领取事务把 `autotest_attempts` 加一、写全局唯一 worker instance id 和 60 秒到期时间，同时把同 batch 遗留的 RUNNING 行恢复为 QUEUED、其 `available_at` 置数据库当前时刻，并把包级字段重算为未完成运行行最小值；崩溃重领后只运行该 batch 中 `state='QUEUED' AND available_at <= clock_timestamp()` 的行，FINISHED 行永不重跑，尚未到期的 QUEUED 行不得提前执行。执行 suite 期间每 20 秒条件续租；续租、run 行变更和包终态写入都必须同时匹配 package id、`status='PENDING_AUTOTEST'`、`active_autotest_batch_id`、`autotest_locked_by` 与未过期租约，影响零行即回滚当前只读事务并停止，陈旧 worker 不得覆盖新持有者。该任务构造 `SecurityContext::system(..., SystemPurpose::General)`，不得借用 `Reconciliation`。

`run-autotest` 受理、batch 领取/崩溃重领、每次 lease heartbeat、每条 run 的开始/终结写入与九行 FINISHED 后的 final aggregate 都是独立短写事务；每笔在查询 package/batch、`FOR UPDATE SKIP LOCKED`、修改 run 或重算 package 前无条件以第一业务 SQL 取得 `LICENSE_CURRENT_EXCLUSIVE` 并锁内重验 ConfigRelease admission。suite 实际执行的数据库访问是纯只读事务，可选 NONE；它不得 claim、续租、汇总或夹带状态写。recording transaction 负例必须让任一 worker 在 exclusive 前执行 package query/claim/lease 时当场失败，且没有依赖死锁或后置补锁。

每个 suite 独占一个只读事务：RLS_MATRIX 与 ROLE_PREVIEW 用 REPEATABLE READ，其余七套用 READ COMMITTED。语义断言失败立即把本套置 FINISHED/FAILED，不重试，但继续执行其余适用套件以形成完整报告。可重试基础设施错误不在 worker 线程内长时睡眠：首次失败后按 1 秒、5 秒、30 秒、2 分钟、10 分钟、30 分钟、1 小时、2 小时八步，把本行 `available_at` 写成数据库当前时刻加对应退避、置回 QUEUED、`failure_count` 加一，并把包的 `autotest_available_at` 重算为当前 batch 全部未完成行的最早到期值；当前持有者先继续其他已经到期的行，尚未到期的行不得执行，无已到期可执行行时清租约并返回轮询。八个时间点各对应一次重试；第八次重试仍失败时 `failure_count=9` 并把本行置 FINISHED/FAILED，不产生第九个退避。`autotest_last_error` 只保存最新清洗后基础设施错误，语义失败只写 suite `report`。九行全部 FINISHED 后才更新包：任一 FAILED 则 TEST_FAILED，否则全为 PASSED 或合法 SKIPPED 才进入 TEST_PASSED；终态更新同时清空 `autotest_locked_by`、`autotest_locked_until` 与 `autotest_available_at`。

#### 4.3 在线 DDL 计划生成与执行算法

输入为配置包中全部 CUSTOM_ 前缀内容项的差集，输出为一份 `ddl_plans` 与其有序 `ddl_plan_steps`。

步骤如下。

1. 归一化。把 ADD、MODIFY、REMOVE 三类内容项按目标对象聚合。ADD 与 MODIFY 参与目标物理结构差异；REMOVE 只生成元数据动作 `status=RETIRED`，从服务端 schema、查询注册表、客户端引导和 UI 中隐藏对象/字段/关系/索引/视图，但明确不进入物理结构差异，也不生成任何 DROP。普通配置包导入时若 REMOVE 的 after plan、manifest 或预生成 SQL 含 `DROP COLUMN`、`DROP TABLE` 或等价动态 SQL，整包按 VALIDATION 拒绝。
2. 基线校验。字段类型必须落在规格第 7.4 章的 11 种之内；索引类型必须落在 3 种之内；JSON 列不得建索引也不得设 CHECK 校验；对象级密级与字段级密级必须有值，字段级为空时按对象级取值，两者都为空即拒绝。任一项不通过返回 `VALIDATION`，计划不生成。
3. 语句映射与执行模式判定。

| 差异 | 生成语句 | 执行模式 |
|---|---|---|
| 新增对象 | create table、enable rls、force rls、create policy、逐表 grant、三条基线索引 | ONLINE |
| 新增可空列 | alter table add column，无默认或常量非易失默认 | ONLINE |
| 新增索引 | create index concurrently | ONLINE |
| 放宽长度 | drop constraint 旧 CHECK，add constraint 新 CHECK not valid，validate constraint | ONLINE |
| 新增多对多关系 | create table link，rls 五件套，唯一索引 | ONLINE |
| 收紧长度、改列类型、收紧非空、重建主键 | 对应非删除 DDL | MAINTENANCE_WINDOW |

计划整体的执行模式取其中最严者。含 MAINTENANCE_WINDOW 语句的计划在未登记停机窗口时返回 `PLATFORM.DDL_PLAN.REQUIRES_MAINTENANCE_WINDOW`。

4. 五项影响分析。索引项给出新增索引数、该对象索引总数与配额比对、按现有行数与平均行宽估算的索引体积；容量项给出新增列的行宽增量、`ext` 下对象总数与字段总数与配额比对、磁盘剩余量；性能项给出每条 `create index concurrently` 按现有行数与认证期实测吞吐外推的预计耗时与 30 分钟上限比对；安全项给出密级赋值核对结论、RLS 模板齐备结论、新增查询入口是否已纳入 RLS 矩阵测试的结论；迁移项给出逻辑回退方式，并对 ADD 明示“失败或回退时物理对象可能保留为不可见结构、数据不删”，对 REMOVE 明示“只退休元数据、物理行数与 checksum 不变”。五项写入 `ddl_plans` 的五个 jsonb 列，缺一不可。
5. 执行。DDL 段的第一步由 job-worker 的 DDL 执行器在把控制交给 `ep-platform-release` 的编排之前，调用经装配注入的 `ep_foundation::port::db::MigrationWindowGuard` 实例的 `assert_open(tx)`，该端口与其唯一实现 `PgMigrationWindowGuard` 均由阶段 2 交付并已在两个 wiring 注入，`ep-platform-release` 不引用该 trait，见裁定 B-03；未持有已打开的迁移窗口时不建立任何连接、不执行任何语句，返回 `PLATFORM.DB.MIGRATION_WINDOW_CLOSED`，HTTP 409，category 为 BUSINESS_CONFLICT，该错误码由阶段 1 登记，本阶段只引用。守卫通过后由 job-worker 建立一条 `ep_migrator` 连接，会话上执行 `set lock_timeout = '5s'` 与 `set statement_timeout = '30min'`。逐条语句在自动提交下执行，理由是 `create index concurrently` 不能在事务块内。每条语句执行前后各取一次 `clock_timestamp()`，把等待锁的时长与执行时长写入 `ddl_plan_steps`。
6. 失败与回退。任一语句失败时立即停止，只逆序撤销不会删除表、列或业务数据的结构：`create index concurrently` 可对应 `drop index concurrently`，新建策略/约束可在证明其撤销不改变行内容时撤销并恢复原 CHECK。已经成功的 `add column` 与 `create table` 绝不补成 `drop column/table`；它们保留物理结构，相关元数据置 DDL_FAILED 或 RETIRED、从 API/UI/查询注册表隔离，并追加 outcome=`RETAINED_INACTIVE` 的步骤事实。补偿前后对每张受影响物理表记录 row_count 与按主键排序的业务列 checksum，任一变化立即停止并转人工安全事件。安全补偿完成后计划置 ROLLED_BACK；若失败原因为 lock_timeout，计划另置 DEFERRED_TO_WINDOW 并把回退原因、操作对象与耗时写入审计。不存在用 DROP 伪装“回到起点”的补偿。
7. 元数据与 DDL 的一致化。DDL 无法与元数据写入同事务，因此采用两阶段：先在一个事务内把相关 `custom_objects` 与 `custom_fields` 置 PENDING_DDL，最后写审计终结批；执行 DDL；成功后在一个事务内置 ACTIVE、递增 `definition_version`、执行幂等 `finish`、写 Outbox/同事务通知命令、最后写审计终结批；失败后在一个事务内置 DDL_FAILED、写必要通知命令、最后写审计终结批。第 7 节按裁定 C-25 追加的自检项 `custom-object-ddl-consistent` 检出 ACTIVE 元数据而物理表缺失、或物理表存在而未开启行级安全的组合，该项按裁定 C-25 取 Degrading：检出即把相关 `custom_objects` 置 DDL_FAILED 并隔离其全部入口，经 `ep_platform_obs::DegradationLedger` 开一个 kind 取 `CUSTOM_OBJECT_DDL_INCONSISTENT` 的降级窗口并持续告警，进程照常启动，不以一次业务数据判读阻断九个进程。

边界条件：单次计划的语句数上限 200；同一时刻只允许一份 EXECUTING 的计划，由发布互斥锁保证；`ext` 表在 RLS 策略创建成功之前不对任何应用账号开放，`grant` 语句排在 `create policy` 之后。物理处置与普通发布完全分离：只有阶段 14 `DisposalPort` 的独立处置计划在双人审批、停机窗口、备份与处置清单都满足后才可能请求删除；阶段 13 的 DDL plan 类型、生成器、applier 与回退器均没有 DROP COLUMN/TABLE 变体或字符串拼接逃生口。

#### 4.4 能力等价矩阵与移动端豁免的运行期判定

能力域码由阶段 1 在 `ep-foundation` 的 `CapabilityDomain` 枚举中冻结，见裁定 A-20，本阶段不再定义。下表第 n 行的能力域码即该枚举第 n 个变体的序列化取值，与规格第 6.2 章矩阵 18 行一一对应，取值逐格照抄，不重述。

| 序 | 能力域码 | 规格第 6.2 章矩阵行 |
|---|---|---|
| 1 | crm.customer_360 | 客户档案与客户 360 查询 |
| 2 | clm.contract_esign | 合同条款与电子签章 |
| 3 | sales.order_fulfillment | 销售订单与履约 |
| 4 | procure.supplier_collab | 采购与供应商协同 |
| 5 | inventory.ledger_scan | 库存台账与收发扫码 |
| 6 | service.workorder_equipment | 售后工单与设备台账 |
| 7 | platform.approval_notify | 审批待办与站内通知 |
| 8 | project.task_milestone | 项目任务与交付节点 |
| 9 | mdm.master_data | MDM 主数据维护与审批 |
| 10 | platform.full_text_search | 全文检索 |
| 11 | ledger.posting_close | 财务过账与期末结账 |
| 12 | finance.settlement_view | 收付款登记与对账查看 |
| 13 | invoice.apply_issue | 发票申请与开具登记 |
| 14 | reporting.report_print | 报表与像素级打印 |
| 15 | platform.document_attachment | 文档与附件协作 |
| 16 | platform.admin_lowcode_ops | 系统管理、低代码配置与运维 |
| 17 | platform.extension_dynamic_code | 扩展插件与动态扩展代码 |
| 18 | portal.supplier_web | 外部门户 Web 形态：供应商 |

判定算法。

1. 各业务阶段按裁定 A-20 与阶段 3 第 3.4.11 节在实际外部路由注册行一次声明 `(CapabilityDomain,ActionClass,LicenseAdmissionBindingV1)` 三元组，不再创建 `<USECASE_SCREAMING>_DOMAIN`/`_ACTION` 成对常量或 handler 私有许可判断。本阶段的 `/api/v1/platform/` 与 `/api/v1/ext/` 都取 `CapabilityDomain::PlatformAdminLowcodeOps`；第 5.3 节配置包与发布单由 `crates/platform/release/src/capability.rs` 提供前两值，其余各段及 `/api/v1/ext/` 的展开路由由 `crates/platform/meta/src/capability.rs` 提供前两值，binding 类型只引用 `crates/platform/license/src/admission.rs`。普通路由的封闭映射为 Read/Export→`Fixed(ReadReportAuditBackupExport)`、Approve→`Fixed(BusinessApproval)`、Write/Submit→`Fixed(BusinessWrite)`；第 5.3 节只有 import/run-autotest/submit/approve/reject/sign/create-release-order/execute 八类注册 `ConfigRelease`，其中 approve/reject fallback=`BusinessApproval`、其余 fallback=`BusinessWrite`。`/api/v1/ext/{object-code}/{id}/actions/{verb}` 必须把每个已发布 verb 按其已冻结 ActionClass 展开成注册项，不得给动态 verb 一个默认 effect。`xtask configdoc` 断言每个外部 `/api/v1`、`/portal`、`/mcp` 路由都恰有一个三元组；缺、余、重复或错 binding 即构建失败。同一用例落入两个能力域时仍按取值较低的矩阵行判定。
2. core-server 的能力闸中间件在授权判定之前执行，读取受信入口核验后的 ClientKind，从 `platform_meta.client_capability_values` 取该能力域该端的取值。四端使用本阶段 72 格；`server_admin` 自阶段 13c 起使用 F-55 新增的第五列 18 格。`portal` 与 `ops` 不参与本判定；`mcp` 只能由 grant middleware 固定且不进入矩阵，按 manifest binding 与逐次权限检查判定。原始 `X-Client` 字符串本身不构成受信来源。配置包 import 路由仍属于 `PlatformAdminLowcodeOps`，权限与 capability 判定只做一次：受信 Win/Mac 可选原 JSON attachment 或 strict multipart，受信 ServerAdmin 只用 strict multipart；iOS/Android/portal/ops/mcp 均无 multipart 变体。媒体类型选择不得提升矩阵值或绕过 `lowcode.config_package.import`，Restricted 恢复也不放开通用 attachment capability。
3. 判定结果：
   - Full：放行。
   - Simplified：放行，但批量端点的单次上限按端下调，移动端 50 条，桌面端 200 条；超出返回 `VALIDATION`。业务对象、权限模型与流程结果不变，照抄规格第 6.2 章取值含义。
   - ViewOnly 且 action_class 不为 Read：拒绝，403 与 `PLATFORM.CLIENT_CAPABILITY.WRITE_NOT_AVAILABLE_ON_CLIENT`，`error.advice` 为该操作在桌面端完成的说明，`error.details` 携带 `alternative_path`，响应体 `data` 为空但响应头带 `X-Desktop-Handoff-Token`。
   - NotApplicable：返回 404 与 `PLATFORM.AUTHZ.NOT_FOUND_OR_DENIED`，不区分不存在与不承载，照抄基线第 5.5 节的存在性泄漏统一处理。
4. 桌面接续令牌是 U-K-08 经 F-51 批准的首版冻结值：一次性令牌，有效期 5 分钟，绑定用户、法人、目标能力域与目标单据标识，桌面端登录同一用户后凭该令牌拉起同一单据同一草稿。移动端界面同时提供“发送到桌面端继续”的入口，入口本身不构成写入。
5. 矩阵冻结比对：`client_capability_values` 全表按 `(capability_domain, client, value, exemption_ref)` 排序后计算 SHA-256，与二进制内置的冻结快照哈希比对。按裁定 C-25，`client-capability-matrix-frozen` 整项撤销，既不进注册表也不为其定任何 `DegradationKind` 取值：二进制内置的冻结快照本身就是权威，比对不一致时以内置快照为运行期判据继续运行，同时拒绝一切对该表的写入并持续告警，不阻断启动。规格第 6.2 章“本清单随本规格冻结”由内置快照承载，而不是由启动时的一次数据库判读承载。
6. 客户端侧同样按引导数据中的矩阵取值渲染入口，ViewOnly 的能力域不渲染提交、审批与写入入口。客户端隐藏不构成访问控制，服务端闸是唯一权威，照抄 PRD 第 10.4.3 节。

#### 4.5 声明式规则与移动端 WASM 豁免

1. 规则以 AST 形式存储与下发，无任何代码下发。AST 节点数上限 500，求值深度上限 32，超限返回 `PLATFORM.RULE.AST_LIMIT_EXCEEDED`。
2. 数值一律 `foundation::Money`、`UnitPrice`、`Quantity`、`Rate` 四类，中间值以 Decimal 全精度保留，只在产出最终判定值时按基线第 3.5 节 round，舍入策略 `MidpointAwayFromZero`。
3. 规则含 `WasmCall` 节点即 `requires_wasm` 为真。`executable_on_client` 的取值为 `!requires_wasm`，四端一致。
4. 移动端遇到 `requires_wasm` 为真的规则时不求值，单据保存为本地草稿并置 `pending_central_validation`，不产生正式业务记录也不产生正式会计分录，照抄规格第 6.2 章与第 6.3 章。恢复连接后按该业务模块的正常提交端点提交，中心执行全部规则并把“该单据曾以待中心校验草稿提交”写入审计。
5. 联网状态下客户端可调用 `POST /api/v1/platform/rule-evaluations/actions/evaluate` 获得与中心一致的预校验结果，该端点只读不写，不建立业务记录。
6. 桌面端同样不在本地执行 WASM 计算，首版 WASM 宿主只在服务端 plugin-host 中存在，照抄规格第 9.3 章“首版服务端只有这一种扩展形态”。
7. 实现类型按裁定 B-05 固定：规则求值实现类型为 `AstRuleEvaluator`，位于 `crates/platform/meta/src/rule/`，装配进 core-server，实现阶段 3b 定义的 `ep_platform_flow::port::RuleEvaluator`；WASM 计算的跨进程实现类型为 `PluginHostWasmCompute`，位于 `crates/adapter/ipc/`，装配进 `apps/core-server/src/wiring/` 与 `apps/job-worker/src/wiring/` 两个目录，实现阶段 3b 定义的 `ep_platform_flow::port::WasmComputePort`，其调用经 `\\.\pipe\ep-plugin` 命名管道转发至 plugin-host；plugin-host 侧的进程内执行实现类型为 `WasmtimeComponentCompute`，位于 `crates/adapter/wasm/`，装配进 `apps/plugin-host`，实现同一端口；`ep-adapter-wasm` 与 `ep-adapter-ipc` 互不依赖，见裁定 H-02。`POST /api/v1/platform/rule-evaluations/actions/evaluate` 只调用 `AstRuleEvaluator`，本阶段不新建第二条求值路径。

#### 4.6 配置发布执行与回退算法

发布执行分三段，段间不共享事务。

段一，DDL 段。仅当包含 CUSTOM_ 前缀内容项且存在结构差异时执行，按第 4.3 节。

段二，元数据与配置段。在一个 `READ COMMITTED` 事务内，按 `sort_no` 升序对每个内容项调用对应的 applier。`ConfigItemApplier` 端口、`ItemKind` 枚举、`ConfigPackageItem` DTO 与 `ConfigItemApplierRegistry` 由阶段 3a 在 `crates/platform/release/src/port/config_item.rs` 交付，见裁定 A-19；下列签名即该文件的内容，其中 `Tx` 取自 `ep-foundation` 的 `port::tx`，见裁定 A-01。本阶段只实现 applier，不改端口签名。

```rust
pub trait ConfigItemApplier: Send + Sync {
    fn item_kind(&self) -> ItemKind;
    fn validate(&self, item: &ConfigPackageItem, ctx: &SecurityContext) -> Result<(), AppError>;
    fn apply(&self, tx: &mut dyn Tx, item: &ConfigPackageItem, ctx: &SecurityContext) -> Result<(), AppError>;
    fn revert(&self, tx: &mut dyn Tx, item: &ConfigPackageItem, ctx: &SecurityContext) -> Result<(), AppError>;
    fn requires_derived_store_rebuild(&self, item: &ConfigPackageItem) -> bool;
}
```

`validate` 无 `Tx`，只允许 pure、deterministic 的 syntax/shape 校验；不得查数据库、KMS、文件、current license/module 或任何可变外部事实，其成功也不是发布授权。F-56 的 `apply` 只能在下述全局锁序建立后运行，并必须从数据库持久化的 exact package/item bytes 重新执行 signature/trust/current/source/dependency/governance 与业务守卫；只有 locked `apply` 全部通过才可提交，事务外 safe-parse 或先前 validate 结论都不具权威性。

终态 20 个 applier 的属主闭集如下：本阶段 `ep-platform-meta` 实现 CUSTOM_OBJECT、CUSTOM_FIELD、CUSTOM_RELATION、CUSTOM_INDEX、CUSTOM_VIEW、UI_LAYOUT 与 RULE 七个；Stage 3b 的 `ep-platform-flow`/`ep-platform-notify` 实现 FLOW_DEFINITION/NOTIFY_RULE，`ep-platform-license` 实现 LICENSE_GRANT/MODULE_PACKAGE，共四个；Stage 4 的 `ep-platform-authz` 实现三个 AUTHZ；Stage 11 的 `ep-app-reporting` 实现四个 reporting；Stage 13c 的 `ep-platform-mcp` 实现两个 MCP。总数 `7+4+3+4+2=20`，不得缺项、重项或注入额外 kind。全部实现在 `apps/core-server/src/wiring/` 与 `apps/job-worker/src/wiring/` 注册到同一 `ConfigItemApplierRegistry`；`ep-platform-release` 不反向依赖属主。未注册 kind 整包失败，不能跳过或部分发布。

F-56 的模块 catalog 不是稀疏默认值：fresh 090100 必须由 SYSTEM 原子种入恰 15 行、全部 `NOT_INSTALLED` 且 package/source/install/action-time 投影全空，身份固定如下；Stage 13 运行门与 ServerAdmin 显示都必须逐项核对，缺、多、重复、改名或第 16 行均失败关闭。

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
| `00000000-0000-7000-8000-000000000612` | `ledger` | 经营分录与期间 |
| `00000000-0000-7000-8000-000000000613` | `invoice` | 发票管理 |
| `00000000-0000-7000-8000-000000000614` | `portal` | 客户与供应商门户 |
| `00000000-0000-7000-8000-000000000615` | `reporting` | 报表与分析 |

第 4.6 节所有命令/作业共用同一 advisory key `hashtextextended('platform-license-current',0)` 与 `pre_idempotency_lock` 三值。会产生 `BusinessWrite|BusinessApproval|IntegrationOutbound|AutomationStart` 的普通 handler/job 固定 `LICENSE_CURRENT_SHARED`：`BEGIN/SET LOCAL` 后第一条业务 SQL 取 `pg_advisory_xact_lock_shared(...)`，再 `try_begin`、query/claim、row/module lock 与锁内 admission；shared 事务彼此并发。F-56 推进/current 替换/trusted-time checkpoint 固定 `LICENSE_CURRENT_EXCLUSIVE` 并在同一位置取 `pg_advisory_xact_lock(...)`；exclusive 等既有副作用事务排空，后续新请求排队后重验。纯读及冻结的 read/disposition/convergence 允许路径可 `NONE`；`LicenseGrantRecovery|ModuleDisableRecovery` 只决定 Restricted 准入，必须在共享配置入口的 exclusive transaction 内 strict 派生，绝不赋予 `NONE`。Outbox/worker claim 短事务取 shared 并重验；真正外发前用专用连接取得同 key session-level shared，再按 wire 顺序取 module session shared，重验后持到外部副作用/取消终结并 finally 反序释放，绝不跨外部调用持数据库事务。

所有可能推进 F-56 special 的事务采用唯一全局锁序：`BEGIN` 与 mandatory `SET LOCAL` 后，第一条业务 SQL 必须取得 `LICENSE_CURRENT_EXCLUSIVE`；随后才可 `try_begin` 或 query/claim。总序固定为 `LICENSE_CURRENT_EXCLUSIVE →（仅 ordinary execute）platform_meta.config_release_mutex FOR UPDATE → (config_package_id,release_order_id,item.sort_no,item.id) canonical rows → ModuleCode wire 顺序的 module locks`；ordinary execute 的连接 1 持有 license 与 mutex 至 COMMIT，special execute 不取 mutex 且跳过 DDL 段一。最后才在锁内重读 current/history/source/dependency 并写 projection/package/order/Outbox/audit。该规则逐字覆盖 import、autotest、submit、approve、sign、create-release-order、execute 及 autotest accept、worker batch claim、lease/heartbeat、final aggregate 的每个短事务；不得先查/claim/锁 package、order 或 mutex 再补取 license lock，普通配置包走共享入口也不例外。import 可在事务前 safe-parse archive，但结论非权威。九套 suite 的纯只读查询事务是唯一 autotest 只读例外。reject 在事务前由 typed branch 固定 `NONE`，不推进 artifact、可信时间或 projection，只锁自身 package/flow rows 闭合同一 immutable content hash；不得在无锁事务查包后改判。

GRANT、REVOCATION 与 MODULE_PACKAGE applier 的 `apply` 只可在上述事务内调用，并可幂等再次请求同一 transaction-level license lock；“第一条业务 SQL”指 whole transaction 的第一句，而非 applier 内较晚的第一句。取得全局锁后才重读全部 current/history 与已接受撤销，不得先查一条可能不存在的 current 行。锁内按 Stage 3 唯一 `TrustedClockV1` 公式求不含候选的 `pre_import_trusted_now`；新 grant 初值固定写 `last_trusted_at=max(pre_import_trusted_now,candidate.issued_at)`，revocation 则把命中 current 推进到三值最大。然后才重建 payload、验 CMS/当前信任根/deployment 并执行零 current 下首发、直接续期或撤销；并发首发、续期及续期/撤销竞态的输家在锁内重算后返回 `PLATFORM.CONFIG_PACKAGE.SPECIAL_ITEM_SHAPE_INVALID`，不得透出 SQLSTATE。

普通 `ModuleLicenseQuery` 只按“持久证据、当前 wall clock、进程启动 UTC anchor 加 OS monotonic elapsed”的三者最大值计算可信时间，不写数据库。core/worker public readiness 前，以及 import/autotest/submit/approve/sign/create-release-order/execute 每个推进 special package 状态的关口，都必须先取得同一 `LICENSE_CURRENT_EXCLUSIVE`，锁内重读 current 并 CAS 单调推进 `last_trusted_at` 后才执行原动作；typed reject 固定 NONE 且不推进。job-worker target cadence 不得超过 240 秒，checkpoint 同样取 exclusive；current 行只在严格增加时 CAS，append-only audit 绝不 UPDATE。有 uptime 证据时相邻持久 checkpoint gap 小于或等于 300 秒才 PASS，大于 300 秒必须不可抑制告警并 fail；wall/monotonic 偏差超过 300 秒同样失败。崩溃跨重启最多有小于 300 秒未持久观察窗口，产品说明不得宣传为 NTP/TPM 级防篡改；已持久化错误前跳只能随 Stage 14 可信备份整体恢复，不能自动回拨或 direct SQL 修正。

checkpoint audit 固定 `action='LICENSE_TRUSTED_TIME_CHECKPOINT'` 与 strict after `{schema_version:1,purpose:"EP-LICENSE-TRUSTED-TIME-CHECKPOINT-V1",deployment_id,slot_utc,trusted_now,current_grant_id}`，`schema_version` 是 JSON number `1`。`slot_utc=floor(unix_seconds(trusted_now)/240)*240` 的 canonical RFC 3339 UTC whole-second，分钟可为 `00/04/08/...`且跨小时仍按 Unix epoch，禁止五分钟取整。`ensure_checkpoint` 在同一 exclusive lock 内、任何业务 mutation 前一次性捕获 trusted_now/current grant id/current revocation id 并冻结 slot/payload snapshot；terminal AuditWriter 禁止在投影改变后重读/重算。exclusive 内只按 `(action,after->>'purpose',after->>'deployment_id',after->>'slot_utc')` 查询：0 行用冻结 snapshot 追加；1 行保留既有 exact bytes，只核定键、current id 形状与 hash chain；>1 或不等失败关闭。同 slot 后续动作复用既有 checkpoint，不要求 payload trusted_now 等于本次值；current.last_trusted_at 可独立 CAS。耐久键恰为 `license-trusted-time:v1:<lowercase-deployment-id>:<slot_utc>`，legal entity/actor/client 固定治理法人/SYSTEM grant/system；job target cadence 至多 240 秒且 special 也 ensure 当前 slot，持久 gap 必须小于或等于 300 秒。零 current 的首发可与必要 checkpoint 同事务，不自锁。

0 行分支才预分配新 UUIDv7 event id；1 行复用分支不分配、不写。新 checkpoint 的完整 envelope 固定 `legal_entity_id=<冻结治理法人>`、`actor_user_id=SYSTEM_PRINCIPAL_ID` 且命中同法人 ACTIVE SYSTEM grant、`actor_device_id=null`、`action='LICENSE_TRUSTED_TIME_CHECKPOINT'`、`object_type='platform.license_trusted_time'`、`object_id=deployment_id`、`object_version=null`、`before=null`、`after=<入口冻结 exact payload>`、`reason/approval_ref/reauth_ref=null`、`client='system'`、`occurred_at=<入口捕获 trusted_now>`；链列仅由 Stage 3 `AuditWriter` 派生。API、worker 与 collector 不得各自选择 object、数据库当前时间或 actor 默认值。

治理来源按 current grant→validated initial-governance audit/receipt→本次首张 GRANT candidate 优先。仅显式 non-production、bootstrap absent、zero-current、zero legal-entity 四项同时成立时 readiness 可不写 checkpoint，固定 Restricted/NoCurrent、worker dormant且永不产 Stage14 evidence；production 无豁免。后续首张 GRANT exclusive 事务要求 candidate 法人已存在且 operator 授权有效，并同事务创建首 checkpoint。

grant 行的 `trust_bundle_sha256` 必须等于其 grant source item 的接受时不可变摘要；revocation 与每个 module action 不另复制摘要，只从各自不可删除 RELEASED source item 读取。接受摘要不要求以后等于当前 bundle，计划轮换也绝不回填。当前 bundle 必须匹配签名部署清单；轮换只允许 CAB 维护操作同步更新两者、关闭许可/模块变更门，并枚举全部 RELEASED `LICENSE_GRANT` item（Grant 与 Revoke）和全部 RELEASED `MODULE_PACKAGE` item 的 exact set，以新 bundle 重验 exact persisted inner/outer artifact，再把 current grant/current revocation/current module projection 与 source/type/digest/identity 交叉核对。证据逐项保存旧接受摘要、新验证摘要、对象 id、outer 结论、inner 结论与总结果。

current grant 或命中 current 的 revocation 的 inner 和/或 source special outer 失败使全局保持 `RESTRICTED/SIGNATURE_INVALID`；当前安装模块的 inner 和/或 current source outer 失败只关闭该模块的业务写/审批/自动化/外发 effective runtime，绝不改变只由 current grant/revocation 导出的部署级 `LicenseStatus`。历史 inner 或 outer signer 被新 CRL 明确命中时记 `HISTORICAL_SIGNER_REVOKED`，保留并隔离，排除 `purchased`、rollback candidate 与正向证明，但不倒推另一份有效 current 为 Restricted；其他历史断链、source/digest/signature 漂移或结构损坏不改写独立有效 current，却使许可/模块变更门和共同 release gate 保持关闭，直至可信恢复。无 CAB 清单更新的磁盘或部署清单摘要漂移仍立即使 current 失败关闭。若唯一 current grant 或命中它的 revocation 的 inner 和/或 source outer 唯一失败为 CRL REVOKED，只有 row/source/payload/digest/signature bytes、outer bytes 与接受证据仍自洽时，才允许 inner+outer 都为 ACTIVE、同 deployment/治理法人且直接后继的 GRANT 在固定 advisory-lock 事务完成恢复；其他损坏不得借道。

`ModulePackageApplier` 执行的是部署级动作，命令 DTO 不得接收 `legal_entity_id`，且只允许五条合法边：INSTALL/ENABLE 及采用另一版本的 UPGRADE/ROLLBACK_VERSION 都必须从 current 有效 grant 证明目标 module 与依赖闭包的全部 module codes 已授权，并按动作逐次重验 manifest/产品契约/兼容性/适用维护权；INSTALL 只落 disabled，依赖无需已安装或启用，ENABLE 才额外要求每个依赖均为 `INSTALLED_ENABLED`。DISABLE 不要求 current 许可或维护期且在 Restricted 中仍可走完整恢复链，但 action item 必须携带当前安装旧 `SignedBusinessArtifact` exact bytes，identity/digest/signature/signer/source 必须与当前投影逐字段相等，不能拿任意旧包停用。

产品契约的唯一前像是仓库恰 15 个 `contracts/modules/<wire>.contract.v1.jcs` strict descriptor，每个至多 262,144 bytes，exact DTO 为 `ModuleContractDescriptorV1 { schema_version:1,purpose:"EP-MODULE-CONTRACT-V1",module_code,module_contract_version,module_dependencies,abi_entries }`。`module_contract_version` 的 Rust 字段保留 u32，但 descriptor/product manifest/module package/parser/DB 有效域统一 1..=2,147,483,647，入库前 checked conversion，2,147,483,648 及更大值拒绝且不 cast/wrap。依赖按 wire 排序去重、只指 15 值闭集且全图 DAG；ABI 项为 1..4096 个、按 `(kind,code)` 排序且组合唯一，kind 只取 `COMMAND|QUERY|EVENT|JOB|PERMISSION`，code 匹配 `[a-z][a-z0-9_.-]{0,127}`。每项 schema 唯一位于 `contracts/modules/<wire>/schemas/<64-lowerhex>.schema.v1.jcs`，至多 65,536 bytes、strict JCS JSON Schema 2020-12，只允许本文件 `#` fragment ref；文件名、entry digest 与重算值三等。contract digest 等于 descriptor exact bytes 的 SHA-256；任一 byte/dependency/ABI/schema digest 变化必须升 version，同 version 不得换 digest。`cargo xtask module-contracts verify` 对 descriptor、schema、compiled public registry 与生成的 `MODULE_*` 常量作双向 exact-set，禁止第二 dependency registry或手写摘要。

待签 `target/release-package/product-modules.v1.jcs` 与安装后 `C:\EP\product-modules.v1.jcs` 是同一至多 262,144-byte strict JCS，只能由上述 15 个已验证 descriptor 的 version/digest/dependencies 与 canonical product version 生成并 strict 回读。其 15 行按 wire 排序、全图 DAG，属于 `MANIFEST.sha256` closed roster 并受产品 Authenticode CAB 覆盖；安装/readback/每次动作都用 `C:\EP` fixed-root safe handle，拒绝 reparse/ADS/hardlink/path drift。Stage 14 projection 记录 exact file digest、版本、15 行 contract/dependency digest 与 DAG 结论，不新建 F-56 数据库对象。

全部 RELEASED MODULE_PACKAGE history 必须保持两套一一映射：同一 `package_id` 只能对应同一 exact inner artifact；同一 `(module_code,package_code,package_version)` 只能对应同一 `package_id` 与同一 exact inner。ENABLE/DISABLE/ROLLBACK_VERSION 重复带回 exact inner 合法；不同 payload/digest/signature/signer bytes 冒用任一 identity，在 release 锁内和 093300 deferred COMMIT 都以 `PLATFORM.CONFIG_PACKAGE.SPECIAL_ITEM_SHAPE_INVALID` 整项拒绝，不允许运行时任选历史。

`ModuleLicenseQuery::module_state` 仅供管理界面读取 raw install projection；业务运行唯一调用 `module_is_currently_licensed`，其 `Ok(true)` 要同时满足 raw INSTALLED_ENABLED、current module outer/inner/source/accepted digest/product catalog 有效、current grant 有效且含 module、目标法人命中 scope。合法未装/停用/未授权/范围外返回 `Ok(false)`；结构/IO/摘要/source/catalog 歧义返回 `Err`，调用者失败关闭。`feature_is_enabled` 除 feature row 外还必须通过 owner module 的同一 effective gate，不能以 raw state 或缓存布尔值替代。

每条动作的 signer 状态固定：首次 INSTALL 与新 UPGRADE 的 inner+新 outer 都必须 ACTIVE；ENABLE/DISABLE 复用既有 RELEASED inner exact artifact，inner 可 RETIRED 但不可 REVOKED，新 action outer 必须 ACTIVE；ROLLBACK_VERSION 只可精确引用既有 RELEASED history artifact，inner 可 RETIRED 但不可 REVOKED，新 outer 仍 ACTIVE。current inner 和/或 current source outer REVOKED 只进下述 DISABLE 窄逃生口。

若当前 package 的 inner signer 和/或 current RELEASED source special outer signer被当前 bundle CRL 明确标为 REVOKED，该模块运行门立即关闭，旧 package 不作当前正向证明，且任一层 signer 已 REVOKED 的历史 package 禁止 ENABLE/ROLLBACK_VERSION/rollback。唯一逃生口是不重签 inner：新 special item 仍为 `action=DISABLE` 且携带旧 SignedBusinessArtifact exact bytes，只由 ACTIVE signer 对新 canonical outer manifest 签 detached CMS；仅当旧 inner、旧 source outer、接受摘要/source/payload/digest/signature/projection 自洽，失败类别只能是旧 inner 和/或旧 source outer CRL REVOKED，且未撤销层为 ACTIVE 或 RETIRED-nonrevoked时放行。其他 action、重签 inner、bad digest/signature/source/chain 或不能唯一分类全部拒绝。该动作仍走完整审批、新 outer+旧两层证据边界复核、独占排空和审计，不把新 outer signer写进旧 inner identity。停用后若部署许可仍有效，只允许 inner 与 outer signer 都为 ACTIVE、semver 严格更高且正常 UPGRADE 守卫全通过的新 package 替换 revoked projection；旧 package 只保留作版本/审计历史，不得成为任何正向证明。

时间投影沿用 Stage 3 唯一规则：INSTALL 写同一 installed/state-changed/disabled 时点且 enabled 为空；ENABLE/DISABLE 各自更新最近时点并保留另一时点；UPGRADE/ROLLBACK_VERSION 只换包/source/reason/state-changed，不抹历史时点。法人 scope 只在逐法人运行时的 `module_is_currently_licensed`、entitlement、feature 与普通写 `LicenseAdmissionGate` 中判定，不得改变部署级安装态。两种 applier 均不得产生 DDL、路径、附件或可执行正文；其 `revert` 永不属于合法调用图。普通包维持下述三段算法，特殊单项包跳过段一、只执行段二的一个受控 apply，再写同一发布证据。

`ModuleOperationGate` 的 advisory key 固定为 `hashtextextended('platform-module:' || ModuleCode wire,0)`。业务事务在读业务行前取得 owner module transaction-level shared lock；worker 在读 payload/claim/dispatch 前以专用连接取得 session-level shared lock并在 finally 释放，随后调用 effective gate递归检查依赖。配置发布中的 module locks 永远位于 whole-transaction license lock 与 canonical package/order/item row locks 之后。INSTALL/UPGRADE/ROLLBACK_VERSION 取目标 exclusive；ENABLE 按全局 wire 顺序取目标 exclusive 与全部传递依赖 shared，锁齐后同事务重验依赖 raw/effective；DISABLE 按 15 个 ModuleCode wire 顺序取得全部 15 把 exclusive，在一个总计 30 秒 deadline 内锁齐但只修改目标。任一超时以 `PLATFORM.MODULE.IN_FLIGHT_DRAIN_TIMEOUT` 整笔回滚，状态、路由、幂等、Outbox 与审计零部分变化。

本阶段新增的 core-server/job-worker scheduler、Outbox dispatcher、`CONFIG_RELEASE` approval owner callback 与出站 IPC operation 全部逐项加入阶段 3 非 HTTP binding registry，没有“平台内部调用”默认豁免。配置 autotest 领取/运行、审批 owner callback 与 release-order execute 沿用 `ConfigRelease` strict target 并在锁内从唯一 item 得到 recovery 或 fallback；其他作业/事件/IPC 必须显式 `Fixed(effect)`。`InFlightConvergence` 只可用于已产生外部副作用后的回执落库、终态/取消或无新副作用补偿；配置发布传播、索引重建或状态收敛只有在已提交来源事实且不会产生新的外发/业务效果时才可登记该值。任何 PENDING/DISPATCHING 首次或重试外发仍是 `IntegrationOutbound`，Restricted 时保留队列不领取/不派发。Stage 13 完成时两个 wiring 的 binding 集合必须与实际 route/job/event/owner/IPC 集合 exact equal，并让同一 `license-admission-registry-consistent` Blocking 自检与 xtask 比对通过。

F-56 special 的 RELEASE execute 事务在 inner CMS/链/CRL、outer signature、item/content hash 与当前部署清单 bundle 摘要全部复验后，才允许把唯一 item 的 `accepted_trust_bundle_sha256` 从 null 写成此次实际 bundle 的 32-byte SHA-256；同一事务随即执行对应 applier 投影并把 package/order 置 RELEASED/SUCCEEDED。三步任一失败全部回滚，不能先写摘要、先发布包或事后补投影；已非空摘要的重放只接受 same-byte 已完成事实，不得覆盖。许可 grant 投影把同一值复制到既有 `license_grants.trust_bundle_sha256`，revocation 与 module action 不复制摘要，只由各自不可删除 source item 提供接受证据。

首次 RELEASE 同事务以 audit terminal batch 写唯一 `action='platform.config_special.accepted.v1'`；完整 envelope 还固定 `event_id` 为本次 RELEASE terminal batch 前预分配的新 UUIDv7，`legal_entity_id` 为冻结治理法人，`actor_user_id/actor_device_id/client` 逐字取 execute 的受信 `SecurityContext`，`object_type='platform.config_package_items'`，`object_id=config_item_id`，`object_version` 为 terminal item row_version，`before=null`，`after=上述 payload`，`reason=null`，`approval_ref=config_packages.approval_ref`，`reauth_ref=null`，`occurred_at=accepted_trusted_now`；`event_day/seq/prev_hash/hash` 只由 AuditWriter 既有分段链算法派生；same-byte 回放不得重复。payload strict-JCS 闭集为 `{schema_version:1,purpose:"EP-CONFIG-SPECIAL-ACCEPTED-V1",config_package_id,config_item_id,artifact_kind,artifact_id,artifact_action,accepted_trusted_now,accepted_trust_bundle_sha256,inner_signer_subject,inner_chain_sha256,inner_trust_state,outer_signer_subject,outer_chain_sha256,outer_trust_state,payload_sha256,item_hash,content_hash,source_projection_sha256}`；module action、inner `ACTIVE|RETIRED_NONREVOKED|REVOKED_AS_DISABLE_TARGET` 与 outer ACTIVE 的合法组合逐动作验证。两个 chain digest 只用 `SHA-256(ASCII("EP-CMS-CHAIN-V1")||0x00||leaf→anchor 每证书 u32be(length)||exact DER)`；source digest 只用 `SHA-256(ASCII("EP-CONFIG-SPECIAL-SOURCE-PROJECTION-V1")||0x00||JCS(dto))`，其中 dto exact 为 `{schema_version:1,purpose:"EP-CONFIG-SPECIAL-SOURCE-PROJECTION-V1",config_package_id,package_no,source:"IMPORTED",status:"RELEASED",content_hash,outer_signature_sha256,outer_signer_subject,outer_signed_at,config_item_id,item_kind,item_code,change_kind:"ADD",sort_no:1,applies_to_legal_entity_ids:[],before_spec_sha256:null,after_spec_sha256,item_hash,accepted_trust_bundle_sha256}`。所有摘要、exact CMS、terminal projection 与审计链必须可互相重算，任一 missing/unknown/不等使整笔回滚。

段二成功后在同一事务内按顺序写入：`config_release_orders` 置 SUCCEEDED、`config_packages` 置 RELEASED、幂等 `finish`、Outbox 事件 `platform.config_release.released.v1`、同事务通知命令、审计终结批。只有本次是普通配置包时，才在该事务把同一普通 lineage 的上一 RELEASED 包置 SUPERSEDED；special 自身永久保持 RELEASED，既不 supersede 任何包，也不被后续 special/普通包 supersede。许可 current 只由新签名 grant/revoke 动作继承，模块 identity 只由新签名 module action 继承，旧配置包与 item 永久保留。

段三，传播段。由 job-worker 消费 Outbox 事件执行，包含：任一 applier 的 `requires_derived_store_rebuild` 为真时按法人逐个重建内置搜索索引分区，重建经阶段 3b 按裁定 A-07 交付的 `ep-adapter-search` 执行，本阶段只按 `ep_foundation::port::search::SearchDocument` 产出投影并经 `SearchIndexPort` 写入，不自建第二条写入路径，重建期间该分区停止对外服务，重建后重放待处理的删除与更正事件并与来源做条数一致性校验与哈希抽样对账，照抄规格第 7.9 章；客户端引导数据版本号递增，使在线客户端在下一次引导时拉到新配置；站内通知按 PRD 第 10.5.2 节送达配置管理员。

失败与补偿：

- 段一失败，段二不执行，只执行第 4.3 节允许的无数据损失补偿；已成功新增的表/列保留为不可见物理结构，发布单置 FAILED，受影响元数据置 DDL_FAILED/RETIRED。
- 段二失败，事务回滚，段一同样只做无数据损失补偿；不得为追求物理结构回到起点而 DROP 已新增表/列，发布单置 FAILED。
- 段三失败，发布已生效，进入 Outbox 重试与死信，按基线第 6.2 节的 8 次退避；连续失败进入死信并按 PRD 第 10.5.2 节通知责任人；权限、密级或分区规则变更的传播未完成前，受影响范围的检索与报表入口不可用，照抄规格第 7.9 章与 PRD 第 10.4.5 节。

回退算法：

- 受理 ROLLBACK 时先锁目标包与 items；含 `LICENSE_GRANT|MODULE_PACKAGE` 立即以 `PLATFORM.CONFIG_RELEASE_ORDER.NON_ROLLBACKABLE_ITEM` 零写入拒绝，故以下通用回退仅适用于其余 18 类普通内容项。许可续期/撤销、模块停用/升级/显式 `ROLLBACK_VERSION` 都必须导入新的 F-56 签名特殊单项包并重新走九套测试与双人审批。
- 回退发布单以上一 RELEASED 包为目标，按 `sort_no` 逆序对当前包的内容项调用 `revert`，使用 `before_spec` 恢复逻辑配置；`revert` 的契约不承诺物理 schema 回到旧形状，更不允许删除表、列或业务行。
- 数据定制的普通 REMOVE 与回退取值（U-K-02 经 F-51 批准的首版冻结值）：REMOVE 及新增字段/对象的回退都只把元数据置 RETIRED，界面、API、查询注册表与客户端引导不再暴露该字段或对象，物理列、物理表及其中数据一律保留；发布前后与回退前后必须证明每张受影响表 row_count 和业务列 checksum 不变。规格第 7.2 章与第 7.5 章要求业务数据只追加不覆盖，回退不得删掉已录入的业务数据。物理删除只能由阶段 14 独立处置计划发起，经双人审批，并按裁定 A-22 经 `ep_platform_file::port::disposal::DisposalPort` 交由 `OpsDisposalService` 执行，走规格第 12.4 章的停机窗口、备份、处置清单与证明；普通 Config package 不得调用该端口，也没有 DROP COLUMN/TABLE 计划类型。
- 回退同样触发受影响派生存储的重新打标，照抄 PRD 第 10.4.5 节。
- 可回退版本数与时间窗（U-K-02 经 F-51 批准的首版冻结值）：保留最近 10 个 RELEASED 包，且发布时间不早于 180 天；超出范围的包置 SUPERSEDED 且不可作为回退目标，尝试回退到该包返回 `PLATFORM.CONFIG_RELEASE_ORDER.ROLLBACK_TARGET_EXPIRED`。

#### 4.7 配置包签名与验签算法

1. 打包：普通包 `manifest.toml` 含包码、名称、版本、`min_platform_version` 与内容项清单，每项至少绑定 `item_kind`、`item_code`、`change_kind`、`sort_no`、scope 与 `item_hash`。全平台 hash 算法固定为 ADD/MODIFY=`SHA-256(JCS(after_spec))`、REMOVE=`SHA-256(JCS(before_spec))`，输出 64 位 lowerhex；按 change kind 被选中的 spec 必须非 null，禁止对 null 求摘要。kind/code/change/sort/scope 与 spec 形状由已签 manifest、行 CHECK 和重算共同绑定；MODULE_PACKAGE action/reason 已在 after_spec 内。import、autotest、submit、sign、execute 每次都按这一规则重算。
2. `content_hash` 为 `manifest.toml` 字节流的 SHA-256。
3. 普通包外层签名：只有非 F-56 special 的普通配置包才进入部署 KMS 路径。`EP__RELEASE__SIGNING_KEY_REF` 的 secret ref 在一次 `actions/sign` 只解析一次为 immutable/versioned `KeyRef`；以 `ep_foundation::port::kms::KmsSigningKeyIdentityResolver` 对该 exact ref 取得 `SigningKeyIdentityV1 { key_ref, spki_sha256 }` before，以同 ref 调既有六方法 `KmsBackend::sign(content_hash exact bytes)`，再以同 ref 调 `verify` 且必须为 true，最后 resolve after 并要求与 before exact equal。`SigningKeyIdentityV1::signer_subject()` 是 canonical `spki-sha256:<64 lowerhex>` 唯一生成者；摘要只能来自该不可变 KeyRef 所指公钥 exact DER SubjectPublicKeyInfo，resolver 不返回私钥。整条链成功后才在锁定包的同一数据库事务写 canonical `signature_key_ref`、token、signature、signed_at、状态与审计；ref/identity 漂移、不可解引用或 verify=false 都零状态推进。Builtin/HSM adapter 同时实现独立 resolver，私钥不出载体；轮换只影响后续签名，不回填历史。
4. 导入与 special 外层签名：导入时对所有包逐项重算 `item_hash/content_hash`、验证 outer signature、核对 `signer_subject` 逐字命中签名 `DeploymentManifestV1.license_trusted_signer_subjects` 唯一 roster，并核对 `min_platform_version`。signed roster 恰含 1..64 个 `spki-sha256:<64 lowerhex>`，按 UTF-8 bytes 升序去重；`EP__RELEASE__TRUSTED_SIGNER_SUBJECTS` 空值只表示不覆盖，非空时必须与 signed roster 逐项 exact equal，绝不能扩大或缩减授权。F-56 signer token 输入是 leaf exact DER SPKI；display DN 只派生展示，不参与身份/JCS/授权。F-56 imported special 通过后把发行方 outer `signature` exact bytes、token 与 `signed_at` 原样写入既有三列，`signature_key_ref` 保持 null，且包从落库起不可修改；批准后的 `actions/sign` 再次重算 hash、调用 outer verifier 复验这组三值并原样保留，然后迁移 `SIGNED_PENDING_RELEASE` 和写审计，`KmsBackend::sign` 与 `KmsSigningKeyIdentityResolver` 调用次数都必须为零。任一验证不通过均不推进状态并返回既有 SIGNATURE_INVALID/SIGNER_NOT_TRUSTED/ITEM_HASH_MISMATCH。
5. 差异算法：两包内容项按 `(item_kind, item_code)` 对齐，逐项比对规范化后的 `after_spec`，输出新增、修改与删除三类，每项给出 before 与 after 的规范化 JSON；同一内容项在两包中完全一致时不进入差异。
6. F-56 artifact 的内层签名是独立 detached CMS，不替代 special 外层配置包签名，也不走普通外层 KMS verifier。`LICENSE_GRANT.after_spec` 原样保存 `SignedBusinessArtifactV1<LicenseArtifactPayloadV1>`；`MODULE_PACKAGE.after_spec` 原样保存带 action/reason 的 `ModulePackageItemV1`，只有其 `artifact` 字段是 `SignedBusinessArtifactV1<ModulePackageManifestV1>`。`Sha256Digest` JSON 只收 64 lowerhex、数据库只存 32 raw bytes。各自内层 strict JSON、RFC 8785 JCS、摘要、canonical base64url-no-pad、Code Signing EKU、SPKI token、离线链和撤销只能对固定 `C:\ProgramData\EnterprisePlatform\trust\license-roots.p7b` 验证，按 Stage 3 §3.4.11 独立通过；不得读取 Windows 任意根、联网补链或临时根，UI/API 不回显 signature 正文。CMS signingTime 与 outer RFC 3339 signed_at/inner RFC 3339 issued_at 必须语义上为同一 UTC whole-second instant：1950..2049 用 DER UTCTime，其他年份用 GeneralizedTime，Z-only、含秒、无小数/offset，禁止文本 bytes 比较。SignerInfo、每张证书与每份 CRL 的 signature AlgorithmIdentifier 只允许 ECDSA P-256/SHA-256 parameters absent 或 RSA-PSS/SHA-256（RSA≥3072、MGF1-SHA256、salt=32、trailer=1），拒绝 SHA-1、PKCS#1 v1.5、NULL/默认/隐式参数。
7. 状态看唯一完整链而不是只看 leaf。non-anchor 为 leaf+intermediates，每张必须在 signed_time 有效；anchor 必须在 signed_time 有效，trusted_now 后 anchor 自身过期不触发 RETIRED，但移除/替换/多链为 UNTRUSTED。`REVOKED>ACTIVE>RETIRED>UNTRUSTED` 只在全链 CRL prerequisite 成功后适用：先为每个实际 issuer 唯一选择 global-highest、覆盖 trusted_now、Name/AKI/SKI/签名/CRLNumber/nextUpdate 全合法的完整 base CRL；任一 issuer 缺失、尚未生效、过期、同最高号冲突、delta/indirect/removeFromCRL/unknown critical 或非法，整链立即 UNTRUSTED，不扫描其他 issuer serial、不进入 CRL recovery、不退旧 CRL。仅全集成功后扫描全部 serial，任一命中才 REVOKED；零命中且全当前有效为 ACTIVE；零命中、全 signed_time 有效、当前至少一张过期且无 not-yet-valid，并有首次 ACTIVE 接受/source/digest/signature 自洽证据才 RETIRED。新 import/release inner+outer 必须 ACTIVE；既有合法 current/history 可 ACTIVE 或 RETIRED-nonrevoked；不读 CDP/OCSP/网络。
8. F-56 special `.epcfg` 是总长至多 4,193,900 bytes 的单卷 ZIP32/STORE archive，exact root entries 恰为 `manifest.toml,item.jcs,outer-signature.p7s`，固定 ZIP overhead=330 bytes；禁止 ZIP64/encryption/data descriptor/extra/comment/directory/duplicate-or-case-collision/path escape/link-or-reparse/trailing/nested archive，DOS 时间、CRC 与 local/central size/offset 必须规范一致。`item.jcs` 上限 2,882,850 bytes，恰为唯一 ADD item 的 after_spec RFC 8785 JCS exact bytes；`manifest.toml` 上限 262,144 bytes，按 F-56 exact key order、LF-only、末尾一 LF 的 canonical writer 从保存列重建并与输入逐字相等；outer CMS 上限 1,048,576 bytes，是对 manifest exact bytes 的单 SignerInfo DER detached CMS，signed attributes 闭集和 signingTime/content hash 精确复验。special outer 与 inner 共享固定 release roots 但独立验签，普通 KMS outer 是第三条独立 verifier。

错误映射固定分层：archive/ZIP/TOML/item JSON/base64 语法、cap、CRC、entry/container metadata 失败统一 `PLATFORM.REQUEST.INVALID_PAYLOAD`/400/零落库；typed item hash 不等为 `PLATFORM.CONFIG_PACKAGE.ITEM_HASH_MISMATCH`；CMS 密码学失败为 `PLATFORM.CONFIG_PACKAGE.SIGNATURE_INVALID`；链/CRL/EKU/root/subject 失败为 `PLATFORM.CONFIG_PACKAGE.SIGNER_NOT_TRUSTED`；strict DTO 通过后的 special 业务 shape/metadata/governance 才为 `PLATFORM.CONFIG_PACKAGE.SPECIAL_ITEM_SHAPE_INVALID`。`PLATFORM.MODULE.PACKAGE_INVALID_OR_INCOMPATIBLE` 仅用于信任通过后的 version/contract/maintenance/history identity/compatibility。

special parser 必须在 JSON/TOML escape 解码后递归拒绝每个将落 PostgreSQL text/jsonb 的 string 中的 U+0000，并且早于 package/item、审计、Outbox 或其他业务副作用；manifest/item 内业务字段的直接 NUL、JSON `\u0000`、TOML basic-string `\u0000` 统一返回 `PLATFORM.CONFIG_PACKAGE.SPECIAL_ITEM_SHAPE_INVALID` 且零落库，容器语法错误仍沿既有外层 parser 错误。canonical writer 同样拒绝内存 U+0000，不能用 import 后重建绕过。

#### 4.8 服务端 WASM 插件沙箱算法

1. 登记与加载：register 在当前法人安全上下文内按 `attachment_object_id` 锁定附件对象，读取它的 `current_version_no` 对应版本并要求对象未标记删除、版本 `state='AVAILABLE'`、病毒扫描结论为 CLEAN；`artifact_legal_entity_id`、`artifact_attachment_version_id`、`artifact_hash` 与 `artifact_size_bytes` 全部从该锁内版本行派生，客户端不能提交或覆盖。事务内完成哈希、大小、manifest 与签名验证后才写 REGISTERED。加载 ENABLED 扩展时只按 `(artifact_legal_entity_id,artifact_attachment_version_id)` 读取这个固定版本，允许其因同对象发布了后续版本而处于 AVAILABLE 或 SUPERSEDED，但拒绝 PENDING、QUARANTINED、FAILED、正文不可读、哈希或大小不等；随后重算哈希、验签，并核对 `capability_manifest` 未声明超出当前有效 grants 的能力。任一不通过不加载并写审计，不得按哈希全库搜索、按对象 current 版本漂移或从存储路径猜测制品。
2. 编译：wasmtime Component Model 编译一次，按 `artifact_hash` 缓存到 `EP__PLUGIN__COMPILE_CACHE_DIR`；进程重启时从缓存恢复，缓存文件损坏即重新编译。
3. 实例化：每次调用建立独立 `Store`，设置燃料上限、内存上限、表元素上限、实例数上限；开启 epoch interruption，tick 为 100 毫秒，交易路径的执行时限 2000 毫秒，Worker 路径 30000 毫秒。
4. 宿主导入函数只有四个：`log(level, msg)` 写日志且经 `Redacted` 处理，`now()` 返回调用方传入的固定时刻，`get_input(name)` 只返回按 grants 裁剪后的输入字段，`emit_result(json)` 返回结果。没有网络、文件、随机数、环境变量、时钟与线程的导入项。随机数如需要由宿主按调用注入固定种子，保证同一输入可重放。
5. 输入裁剪：调用方在 core-server 或 job-worker 侧按该扩展的 `READ_OBJECT_FIELDS` 授予逐字段裁剪，未授予的字段不进入 IPC 报文。裁剪在统一安全上下文内执行，先按法人、记录、字段与密级过滤，再按授予裁剪。
6. 结果处置：`emit_result` 的 JSON 按调用点声明的结果模式校验，不符返回 `PLATFORM.EXTENSION.MANIFEST_MISMATCH`。
7. 失败分类与错误映射：

| 情形 | 记录 outcome | 分类 | 错误码 |
|---|---|---|---|
| 声明或调用了未授予的能力 | CAPABILITY_DENIED | PERMISSION_DENIED | PLATFORM.EXTENSION.CAPABILITY_DENIED |
| 扩展已停用 | 不产生调用 | PERMISSION_DENIED | PLATFORM.EXTENSION.DISABLED |
| 燃料耗尽、内存超限、执行超时 | FUEL_EXHAUSTED、MEMORY_LIMIT、TIMEOUT | INFRASTRUCTURE | PLATFORM.EXTENSION.RESOURCE_LIMIT_EXCEEDED |
| WASM trap | TRAP | INFRASTRUCTURE | PLATFORM.EXTENSION.RESOURCE_LIMIT_EXCEEDED |
| plugin-host 不可达或 IPC 失败 | HOST_ERROR | INFRASTRUCTURE | PLATFORM.EXTENSION.HOST_UNAVAILABLE |
| 并发实例数达到 `EP__PLUGIN__MAX_INSTANCES` 而调用被限流 | THROTTLED | INFRASTRUCTURE | PLATFORM.EXTENSION.HOST_UNAVAILABLE |

8. 自动停用：同一扩展同一入口连续失败达到 `EP__PLUGIN__AUTO_DISABLE_FAILURE_THRESHOLD`（默认 3）次时，把 `extensions.status` 置 DISABLED、`disabled_reason` 置具体原因，先写同事务通知命令，再写审计终结批，提交后按 PRD 第 10.5.2 节投递，照抄规格第 9.3 章“插件崩溃、超时或越权调用只影响该子进程，宿主记录事件并按策略停用该插件”。成功一次即把 `consecutive_failures` 归零。
9. 事务边界约束：插件调用一律发生在写事务之外。规则求值在开启事务之前完成，把求值结果作为命令输入带入事务，落实基线第 10.3 节“事务内禁止外部调用与长时计算”。

#### 4.9 桌面端原生插件沙箱算法

1. 本地登记表位于客户端加密缓存库，列为插件码、版本、签名主体、制品哈希、能力清单、状态。加载前逐项核对签名主体、版本、哈希与能力清单，任一不匹配不加载，照抄规格第 9.3 章。
2. 本地登记表内容与中心 `platform_meta.extensions` 按 `artifact_hash` 核对，不一致不加载并向中心上报。
3. 插件运行在独立子进程，与客户端核心通过受限 IPC 通信。IPC 承载在 Windows 命名管道与 macOS Unix domain socket 上，帧格式复用基线第 2 节的 4 字节大端长度前缀加 JSON 体。子进程不共享客户端进程内存、不接收本地缓存数据库密钥、不接收会话令牌。
4. 传入子进程的数据只有该次操作的最小必要数据，且已按字段权限脱敏；子进程的日志按同一脱敏规则限制。
5. 崩溃、超时或越权调用只影响该子进程；宿主记录事件、按策略停用该插件，并把插件加载、能力授予、设备访问与异常退出写入客户端审计，随设备健康状态上报中心。
6. 设备策略关闭原生插件加载时，本机打印机与 USB Key 能力停用并显式降级；同时按规格第 12.4 章把该端的强制控制降级为浏览器门户端口径，高密级内容改为只读预览并禁止下载；降级事件与降级范围记入客户端审计并按客户与设备登记，不改变能力矩阵的冻结取值。

#### 4.10 客户端本地缓存与同步算法

1. 本地库为 SQLCipher，密钥为设备硬件保护密钥解封出的数据密钥，绑定 TPM、Secure Enclave 或 Keystore。
2. 缓存内容只有当前用户按权限可访问的数据子集。每条缓存记录携带来源对象标识、版本、法人标识、密级与数据范围标签，与规格第 7.9 章派生存储的标签口径一致。这是本阶段的判断，理由是客户端缓存同样是来源数据的副本，需要同样的裁剪依据才能在离线态正确隐藏无权字段。
3. 增量同步按 `row_version` 与 `updated_at` 拉取。同一记录冲突时以中心版本为准，本地内容不覆盖中心记录。
4. 清除触发：退出登录、设备注销、缓存超期。超期取值为 U-L-06 经 F-51 批准的首版冻结值：桌面端 14 天、移动端 7 天，可由设备策略下调，下调值随策略下发并写审计。
5. 断网期只能保存本地草稿，不产生正式业务记录与正式会计分录；恢复连接后由中心重新校验并提交。
6. 移动端本地缓存范围与单文件上限可按设备存储、网络与安全策略下调，下调后的数值随设备策略下发并可审计，照抄规格第 6.2 章。

#### 4.11 白标构建与签名流水线算法

1. 输入：源码 commit、`Cargo.lock`、`rust-toolchain.toml`、`brand.toml`、目标端集合、版本号。
2. 品牌资源校验：产品名称长度不超过 200，Logo 与启动页按各端要求的尺寸与格式集合逐项校验，主题色为六位十六进制，应用标识按各端命名规则校验。任一不通过返回 `PLATFORM.BRAND_PROFILE.ASSET_INVALID`。
3. 商店政策合规检查门禁，四项逐条判定，照抄规格第 3.6 章：应用主体与账号归属一致、品牌与标识不冲突、隐私声明完整、资质材料齐备。未通过不得提交商店，返回 `PLATFORM.BRAND_PROFILE.STORE_POLICY_CHECK_FAILED`。
4. 可复现构建：固定 `SOURCE_DATE_EPOCH`、启用 `--remap-path-prefix` 去除构建路径、锁定全部依赖版本、在容器化构建环境内执行。同一输入两次构建产出的未签名制品哈希必须一致，该结论是规格第 3.2 章私有构建级支持判据的前提。
5. 签名：桌面端 Windows 用 Authenticode，macOS 用 codesign 加公证；iOS 与 Android 按分发路径选择证书。厂商托管的签名私钥保存在硬件密码机，按客户隔离密钥域，签名操作双人控制并单独审计，照抄规格第 3.1 章。应用商店分发一律使用客户自有账号与证书，厂商不在自有账号下集中发布多个客户的白标应用，照抄规格第 3.6 章。
6. 产出登记：ENTERPRISE_MDM 分发先把签名制品上传为附件，创建发布时按与扩展登记相同的锁内算法固定 `artifact_legal_entity_id + artifact_attachment_version_id`，哈希与大小只从该版本派生；APP_STORE 分发写经过客户端类型封闭主机白名单校验的 HTTPS `store_listing_uri`，哈希与大小取本次受信构建流水线的签名产出清单。两类都把制品哈希、SBOM、签名主体、版本写入发布证据；不允许只有哈希而没有可解析分发位置的发布行进入 ROLLING_OUT/FULL。
7. 灰度：`rollout_percent` 与 `rollout_legal_entity_ids`、`rollout_department_ids` 三项共同决定可见范围，可按法人、部门或用户逐步启用，照抄规格第 18 章。`is_forced_security_update` 为真时不受灰度比例约束，全量下发，客户端在更新完成前拒绝进入业务界面。

---

### 5. API 契约

全部端点遵守基线第 5 章：路径前缀 `/api/v1`，JSON 字段 snake_case，成功与失败封套按基线第 5.2 节，写请求必带 `Idempotency-Key`，请求头集合按基线第 5.6 节。平台侧路径段取 `platform`，该取值已由阶段 2 的九个平台路由按裁定 A-20 落定，本阶段沿用不再另议。自定义对象的通用数据端点路径段取 `ext`，与 schema 名一致，不新增模块码。

下表的权限要求列写权限项名，具体角色映射由权限阶段承担。每个端点都由同一实际路由注册行携带第 4.4 节的能力/动作/许可 binding 三元组，并在任何业务写入或副作用前调用阶段 3 唯一 `LicenseAdmissionGate`；客户端能力闸与许可闸互不替代。普通 GET/导出、Approve、其他写/提交分别用第 4.4 节的三个 Fixed 映射，第 5.3 节仅八类配置发布操作用 `ConfigRelease` strict target。全部端点在授权判定之前先过第 4.4 节的能力闸，能力域为 `platform.admin_lowcode_ops`；Windows/macOS/ServerAdmin 为完整，iOS/Android 为仅查看，因此下表全部写端点在 iOS 与 Android 上返回 403 与 `PLATFORM.CLIENT_CAPABILITY.WRITE_NOT_AVAILABLE_ON_CLIENT`。ServerAdmin 放行能力闸后仍须通过同一许可、对象权限、字段权限、职责分离、重新认证与审批，不取得隐式管理员权限；唯一窄例外是 F-56 特殊配置包的 `CONFIG_RELEASE` 审批结论只能由受信 `client=win|mac` 完成，ServerAdmin 对该待办与结论恒为只读。

#### 5.1 数据定制

| 方法与路径 | 请求 | 响应 data | 主要错误码 | 幂等语义 | 权限项 | Action |
|---|---|---|---|---|---|---|
| GET /api/v1/platform/custom-objects | 分页、排序、过滤按基线第 5.3 节 | 对象列表 | 无 | 只读 | lowcode.custom_object.view | VIEW |
| POST /api/v1/platform/custom-objects | `{code, name, security_level, is_document, doc_type_code, fields:[...]}` | 对象详情，status 为 DRAFT | PLATFORM.CUSTOM_OBJECT.RESERVED_NAME、PLATFORM.CUSTOM_OBJECT.TYPE_NOT_IN_BASELINE、PLATFORM.CUSTOM_OBJECT.SECURITY_LEVEL_REQUIRED、PLATFORM.CUSTOM_OBJECT.QUOTA_EXCEEDED | Idempotency-Key 四元组，重放返回首次结果 | lowcode.custom_object.create | CREATE |
| GET /api/v1/platform/custom-objects/{id} | 无 | 对象详情含字段、关系、索引、视图 | 404 统一按基线第 5.5 节 | 只读 | lowcode.custom_object.view | VIEW |
| PATCH /api/v1/platform/custom-objects/{id} | 局部字段，必带 `row_version` | 新版本 | PLATFORM.CONCURRENCY.STALE_VERSION、PLATFORM.CONFIG_EDIT_LOCK.HELD_BY_ANOTHER_USER | 乐观锁加幂等键 | lowcode.custom_object.modify | UPDATE |
| POST /api/v1/platform/custom-objects/{id}/actions/retire | `{reason}` | 对象详情，status 为 RETIRED | BUSINESS_CONFLICT 若存在引用 | 幂等键 | lowcode.custom_object.retire | UPDATE |
| POST /api/v1/platform/custom-fields | `{owner_kind, custom_object_id 或 core_object_type, code, name, data_type, ...}` | 字段详情 | 同上 | 幂等键 | lowcode.custom_field.create | CREATE |
| POST /api/v1/platform/custom-indexes | `{target_object_type, code, index_kind, column_codes}` | 索引详情 | PLATFORM.CUSTOM_OBJECT.INDEX_KIND_NOT_IN_BASELINE、PLATFORM.CUSTOM_OBJECT.QUOTA_EXCEEDED | 幂等键 | lowcode.custom_index.create | CREATE |
| POST /api/v1/platform/custom-objects/actions/plan-ddl | `{config_package_id}` | `{ddl_plan_id, execution_mode, statements:[...], impact:{index,capacity,performance,security,migration}}` | PLATFORM.DDL_PLAN.REQUIRES_MAINTENANCE_WINDOW | 幂等键，同一包重复调用返回同一计划 | lowcode.custom_object.plan_ddl | SUBMIT |
| POST /api/v1/platform/config-edit-locks | `{item_kind, item_code}` | `{lock_id, expires_at}` | PLATFORM.CONFIG_EDIT_LOCK.HELD_BY_ANOTHER_USER | 幂等键，同一用户重复取锁续期 | lowcode.config.edit | CREATE |
| DELETE /api/v1/platform/config-edit-locks/{id} | 无 | 空 | 404 | 幂等键 | lowcode.config.edit | UPDATE |

自定义对象的数据端点由平台按对象码自动注册，路径与形状固定：

| 方法与路径 | 说明 | 权限项 | Action |
|---|---|---|---|
| GET /api/v1/ext/{object-code} | 列表，分页排序过滤按基线第 5.3 节，默认排序单据类按 `created_at desc, id desc`，档案类按 `code asc` | `ext.object.<object-code>` | VIEW |
| POST /api/v1/ext/{object-code} | 新建，单据类由 `ep-platform-sequence` 取号，编号格式按基线第 11.1 节 | `ext.object.<object-code>` | CREATE |
| GET /api/v1/ext/{object-code}/{id} | 详情，字段按字段级权限与密级裁剪 | `ext.object.<object-code>` | VIEW |
| PATCH /api/v1/ext/{object-code}/{id} | 更新，必带 `row_version` | `ext.object.<object-code>` | UPDATE |
| POST /api/v1/ext/{object-code}/{id}/actions/{verb} | 状态机动作，verb 取值来自该对象的状态机定义 | `ext.object.<object-code>` | 状态机版本冻结的单一 `VIEW\|CREATE\|UPDATE\|SUBMIT\|APPROVE\|EXPORT`，不得运行时猜测 |

动态 ext 权限不依赖运维预建目录。自定义对象首次从 PENDING_DDL 进入 ACTIVE 的最终发布事务，必须在同一事务、对象路由可见前 `INSERT ... ON CONFLICT DO NOTHING` 并逐字段断言唯一 `permission_items` 行：`code='ext.object.'||custom_objects.code,module_code='platform',function_point=code,object_type='ext.'||custom_objects.code,description=null`，`allowed_actions` 恰为固定 CRUD 所需 `VIEW|CREATE|UPDATE` 加该版本全部 verb 显式声明动作后的 canonical 去重集合；同事务还写唯一 `object_scope_bindings` 行 `object_type='ext.'||code,schema_name='ext',table_name=code,owner_user_col='created_by',owning_dept_col/project_col/customer_col=null,security_level_col='security_level'`，并通过 `pg_catalog` 验证表与两个非空列真实存在。任一既有字段、动作集合、表或列漂移都整笔失败，路由不得可见。对象进入 RETIRED 时不删除或改写 permission/binding/历史 grant，但所有 ext 路由立即撤销；安装在 `role_permission_grants` 上的 Stage 13 guard 对 `ext.object.` 前缀的新 INSERT/UPDATE 要求对应 custom object 当前恰一 ACTIVE 且 action 仍在其冻结 allowed_actions，否则拒绝，因而退役后不能新授权。既有授权只留审计历史且不再产生运行权限；重建同码对象被 `ux_custom_objects_code` 阻断，不得借复用旧目录复活。

#### 5.2 界面定制

| 方法与路径 | 请求 | 响应 data | 主要错误码 | 幂等语义 | 权限项 | Action |
|---|---|---|---|---|---|---|
| GET /api/v1/platform/ui-layouts | 按 layout_kind、target_object_type、role_id、client_scope 过滤 | 布局列表 | 无 | 只读 | lowcode.ui_layout.view | VIEW |
| POST /api/v1/platform/ui-layouts | `{code, layout_kind, target_object_type, role_id, client_scope, spec}` | 布局详情 | PLATFORM.UI_LAYOUT.FIELD_HIDING_NOT_ACCESS_CONTROL、PLATFORM.UI_LAYOUT.CAPABILITY_VALUE_NOT_MODIFIABLE | 幂等键 | lowcode.ui_layout.create | CREATE |
| PATCH /api/v1/platform/ui-layouts/{id} | 局部字段加 `row_version` | 新版本 | PLATFORM.CONCURRENCY.STALE_VERSION | 乐观锁加幂等键 | lowcode.ui_layout.modify | UPDATE |
| POST /api/v1/platform/ui-layouts/actions/preview-as-role | `{layout_id, role_id, sample_record_id}` | `{rendered_spec, returned_fields, withheld_fields}` | 无 | 只读 | lowcode.ui_layout.preview | VIEW |

`preview-as-role` 的 `withheld_fields` 只返回字段码不返回值，用于核对无权字段确实不返回而非仅不显示，落实 PRD 第 10.4.3 节的验证要求。

保存布局时的校验：若 `spec` 中某字段被标为隐藏而该字段在该角色的字段权限中为可见，返回 `PLATFORM.UI_LAYOUT.FIELD_HIDING_NOT_ACCESS_CONTROL` 并指出该字段；若 `spec` 试图为某能力域提供高于矩阵取值的入口，返回 `PLATFORM.UI_LAYOUT.CAPABILITY_VALUE_NOT_MODIFIABLE`。

#### 5.3 配置发布与回退

本节端点中，`actions/import`、`actions/run-autotest` 与 `actions/sign` 三个端点及其权限项由本阶段按裁定 A-27 在阶段 3b 最小发布通道之上扩展交付；其余端点的路径、请求与错误码即阶段 3b 的落地口径。

| 方法与路径 | 请求 | 响应 data | 主要错误码 | 幂等语义 | 权限项 | Action |
|---|---|---|---|---|---|---|
| POST /api/v1/platform/config-packages | `{name, source:"IN_PLACE", items:[...]}` | 包详情，status 为 DRAFT | PLATFORM.CONFIG_PACKAGE.ITEM_LIMIT_EXCEEDED | 幂等键 | lowcode.config_package.create | CREATE |
| POST /api/v1/platform/config-packages/actions/import | 已认证 Win/Mac：`application/json {attachment_object_id}` 或 strict `multipart/form-data`；已认证 ServerAdmin：仅同一 strict multipart；multipart 恰一 `package` `.epcfg` file part | 包详情 | PLATFORM.REQUEST.INVALID_PAYLOAD（multipart/长度/shape，HTTP 400）、PLATFORM.CONFIG_PACKAGE.SPECIAL_ITEM_SHAPE_INVALID、PLATFORM.CONFIG_PACKAGE.SIGNATURE_INVALID、PLATFORM.CONFIG_PACKAGE.ITEM_HASH_MISMATCH、PLATFORM.CONFIG_PACKAGE.SIGNER_NOT_TRUSTED、PLATFORM.CONFIG_PACKAGE.PLATFORM_VERSION_TOO_LOW、PLATFORM.LICENSE.RESTRICTED | 幂等键，同一 `content_hash` 重复导入返回既有包；multipart 只使用本节 route-local 4 MiB 窄例外；Restricted 时锁内 strict LICENSE_GRANT/Module DISABLE 才进恢复分支 | lowcode.config_package.import | CREATE |
| GET /api/v1/platform/config-packages/{id} | 无 | 包详情含内容项摘要与各 suite 结论 | 404 | 只读 | lowcode.config_package.view | VIEW |
| GET /api/v1/platform/config-packages/{id}/diff | `?against={package_id}` | `{added:[...], modified:[...], removed:[...]}`，每项含 before 与 after 的规范化 JSON | 404 | 只读 | lowcode.config_package.view | VIEW |
| POST /api/v1/platform/config-packages/{id}/actions/run-autotest | 无 | `{run_ids:[...]}`，数组恰含九个运行行标识；同步返回受理回执，结果异步 | BUSINESS_CONFLICT 若状态不为 DRAFT | 幂等键 | lowcode.config_package.autotest | UPDATE |
| POST /api/v1/platform/config-packages/{id}/actions/submit-for-approval | `{note}` | `{package_id,status:"PENDING_APPROVAL",approval_ref}`；包、流程实例与首任务同事务建立 | PLATFORM.CONFIG_PACKAGE.AUTOTEST_NOT_PASSED、PLATFORM.APPROVAL.CHAIN_NOT_FOUND、PLATFORM.APPROVAL.ACTIVE_CHAIN_AMBIGUOUS、PLATFORM.APPROVAL.NODE_HAS_NO_APPROVER、PLATFORM.SOD.SELF_APPROVAL_FORBIDDEN | 幂等键 | lowcode.config_package.submit | SUBMIT |
| POST /api/v1/platform/config-packages/{id}/actions/approve | `{task_id,note}` | 标准流程任务完成回执；处理器自身不改包状态，包只由同事务 owner callback 迁移 | PLATFORM.SOD.SELF_APPROVAL_FORBIDDEN、BUSINESS_CONFLICT（任务与包/流程不匹配） | 幂等键 | lowcode.config_package.approve | APPROVE |
| POST /api/v1/platform/config-packages/{id}/actions/reject | `{task_id,reason}` | 标准流程任务完成回执；处理器自身不改包状态，包只由同事务 owner callback 迁移 | 同上 | 幂等键 | lowcode.config_package.approve | APPROVE |
| POST /api/v1/platform/config-packages/{id}/actions/sign | 无 | 包详情含 `signer_subject` 与 `signed_at`；F-56 special 返回 import 时保存的 exact 值 | 普通包为 INFRASTRUCTURE（部署签名密钥不可解引用）；special 为既有 SIGNATURE_INVALID/SIGNER_NOT_TRUSTED | 幂等键；普通包返回既有部署签名，special 只复验并逐字保留发行方 outer signature，绝不调用部署 KMS sign | lowcode.config_package.sign | UPDATE |
| POST /api/v1/platform/config-release-orders | `{config_package_id, action, rollback_to_package_id, execution_mode, scheduled_window_start}` | 发布单详情 | PLATFORM.CONFIG_RELEASE_ORDER.ROLLBACK_TARGET_EXPIRED、PLATFORM.DDL_PLAN.REQUIRES_MAINTENANCE_WINDOW | 幂等键 | lowcode.config_release.submit | SUBMIT |
| POST /api/v1/platform/config-release-orders/{id}/actions/execute | 无 | `{task_receipt_id}`，转后台任务 | PLATFORM.CONFIG_RELEASE_ORDER.CONCURRENT_RELEASE_IN_PROGRESS、PLATFORM.CONFIG_RELEASE_ORDER.DERIVED_STORE_REBUILD_REQUIRED、PLATFORM.DB.MIGRATION_WINDOW_CLOSED | 幂等键，重复执行返回同一回执 | lowcode.config_release.execute | UPDATE |
| POST /api/v1/platform/config-release-orders/{id}/actions/cancel | `{reason}` | 发布单详情 | BUSINESS_CONFLICT 若已进入 EXECUTING | 幂等键 | lowcode.config_release.execute | UPDATE |
| GET /api/v1/platform/config-release-orders/{id} | 无 | 发布单详情含逐步执行记录 | 404 | 只读 | lowcode.config_release.view | VIEW |

发布执行是长时操作，按基线第 11.6 节同步等待上限 8 秒，`actions/execute` 一律返回任务回执并转后台任务，完成后由站内通知送达。

approve/reject 两条路径不是第二套审批 API：它们与 `POST /api/v1/platform/process-tasks/{task_id}/actions/complete` 共用同一个 `CompleteProcessTask` 命令、幂等作用域和事务，只额外校验 URL 中 package id 与任务 subject 相同。任务完成后由本节命名 callback 决定包是否能迁移；客户端不得以任务端点 HTTP 200 推断包已批准，必须读取包状态。

F-56 特殊包另有不可绕过的客户端守卫：上述两个便利别名与通用 `process-tasks/{task_id}/actions/complete` 的 typed approve 分支固定 `LICENSE_CURRENT_EXCLUSIVE`，在 `try_begin`/读取 task 前取得；typed reject 分支按唯一例外固定 `NONE`，不得先查包后改判。随后才按各分支锁 task/instance/subject package 并读取其唯一 item。命中 `LICENSE_GRANT|MODULE_PACKAGE` 时，只接受受信入口已经核验出的 `ClientKind::Win|ClientKind::Mac`，`server_admin`、移动端、portal、ops、mcp 或伪造 `X-Client` 都在写 task/step/package、幂等完成记录或审计终结批前拒绝。ServerAdmin 只读展示待办和最终结论，不得通过通用任务完成端点绕过；该守卫不改变普通配置包的既有审批客户端矩阵。

`actions/run-autotest` 不走发布 Outbox：端点事务提交后的 `config_packages` 队列字段与九条 `config_autotest_runs` 即为完整受理事实、派发载体和崩溃恢复点。阶段 13 登记且只登记 `platform.custom_record.created.v1`、`platform.custom_record.updated.v1`、`platform.custom_record.state_changed.v1` 三项；旧“十项”没有其余七个可逐字对账的名称，已由 F-54 撤销，不得以未命名配额驱动实现。

#### 5.4 扩展登记与沙箱

| 方法与路径 | 请求 | 响应 data | 主要错误码 | 幂等语义 | 权限项 | Action |
|---|---|---|---|---|---|---|
| POST /api/v1/platform/extensions/actions/register | `{attachment_object_id, kind, code, version}` | 扩展详情含固定的 `artifact_attachment_version_id`、哈希、大小与解析出的能力清单 | PLATFORM.EXTENSION.SIGNATURE_INVALID、PLATFORM.EXTENSION.MANIFEST_MISMATCH | 幂等键，同一规范化登记命令返回既有登记；同 code/version 不同制品返回 payload mismatch | ext.extension.register | CREATE |
| POST /api/v1/platform/extensions/{id}/actions/request-approval | `{requested_capabilities:[...],note}` | `{extension_id,status:"PENDING_APPROVAL",approval_ref}`；扩展、流程实例与首任务同事务建立 | BUSINESS_CONFLICT、PLATFORM.APPROVAL.CHAIN_NOT_FOUND、PLATFORM.APPROVAL.ACTIVE_CHAIN_AMBIGUOUS、PLATFORM.APPROVAL.NODE_HAS_NO_APPROVER、PLATFORM.SOD.SELF_APPROVAL_FORBIDDEN、PLATFORM.EXTENSION.MANIFEST_MISMATCH | 幂等键 | ext.extension.register | SUBMIT |
| POST /api/v1/platform/extensions/{id}/actions/approve | `{task_id,note}` | 标准流程任务完成回执；路由自身不写扩展或 grant | PLATFORM.SOD.SELF_APPROVAL_FORBIDDEN、BUSINESS_CONFLICT（任务与扩展/流程不匹配） | 与标准任务完成命令共用幂等作用域 | ext.extension.approve | APPROVE |
| POST /api/v1/platform/extensions/{id}/actions/reject | `{task_id,reason}` | 标准流程任务完成回执；路由自身不写扩展或 grant | 同上 | 与标准任务完成命令共用幂等作用域 | ext.extension.approve | APPROVE |
| POST /api/v1/platform/extensions/{id}/actions/enable | 无 | 扩展详情含批准证据与有效授予清单 | PLATFORM.EXTENSION.SIGNATURE_INVALID、PLATFORM.EXTENSION.MANIFEST_MISMATCH、BUSINESS_CONFLICT | 幂等键 | ext.extension.enable | UPDATE |
| POST /api/v1/platform/extensions/{id}/actions/disable | `{reason}` | 扩展详情 | 无 | 幂等键 | ext.extension.enable | UPDATE |
| GET /api/v1/platform/extensions/{id}/invocations | 分页与时间范围过滤 | 调用流水，含 outcome、耗时、燃料与内存峰值 | 404 | 只读 | ext.extension.view | VIEW |
| POST /api/v1/platform/rule-evaluations/actions/evaluate | `{rule_code, rule_version, input}` | `{passed, message_code, details}` | PLATFORM.RULE.EXPRESSION_PARSE_FAILED、PLATFORM.RULE.AST_LIMIT_EXCEEDED、已登记的 `PLATFORM.EXTENSION.*` 精确码 | 幂等键，纯只读 | lowcode.rule.evaluate | VIEW |

`EXTENSION_ENABLE` 是阶段 4 的具名审批场景，默认链只经 `ApprovalChainResolver::resolve_active_chain` 解析且默认角色固定为 `SECURITY_ADMIN`。`request-approval` 锁扩展，要求状态 REGISTERED，重新验制品、签名、manifest，并把请求数组规范化为按 `(capability,scope_key)` 排序、READ_OBJECT_FIELDS 的字段码按字典序排序且去重的紧凑 JSON；逐项证明未超出 manifest 后服务端计算 `requested_grants_hash`。同一事务冻结 `approval_legal_entity_id/approval_scenario/submitted_by/submitted_at/approval_ref/approval_chain_id/approval_chain_version_no/approval_definition_digest/approval_artifact_hash/approval_manifest_hash/approval_requested_grants_hash`，建立流程实例与首任务，最后完成幂等、通知与审计；无链、多链、空节点、空审批人、自审或摘要不闭合均整笔回滚，扩展保持 REGISTERED 且流程、任务、grant、通知、Outbox、审计零新增。

approve/reject 与标准 `CompleteProcessTask` 命令、幂等作用域和事务共用一条路径，只多校验 URL 的 extension id 等于任务 subject；路由不得直接写 extensions 或 grants。流程引擎只在同一事务调用唯一 owner callback：

```rust
pub trait ExtensionEnableApprovalOwner: Send + Sync {
    fn complete(
        &self,
        tx: &mut dyn Tx,
        approval_ref: ApprovalRef,
        conclusion: ApprovalConclusion,
        actor: UserId,
        occurred_at: DateTime<Utc>,
    ) -> Result<(), AppError>;
}
```

`ExtensionEnableApprovalCallback` 是唯一实现与唯一写者。它按 id 锁扩展、流程实例、当前任务和该扩展全部 grants，逐项要求状态 PENDING_APPROVAL、subject/scenario/法人/发起人/approval_ref/chain id/version/digest 与冻结证据相等、三项当前摘要等于审批摘要、节点按 node_no 完成、actor 属于冻结审批集合且不等于 submitted_by。批准时它从 `requested_grants` 一次性写入全套有效 grants（kind、法人、approval_ref、granted_by 均由父证据派生），依赖延迟图在提交点证明“有效 grants = 请求快照 ⊆ manifest”，再原子写 APPROVED 与 approved_by/approved_at；驳回时不写 grant，只写 REJECTED 与拒绝结论。任一校验失败须连同任务完成一起回滚并发安全告警，不使用 Outbox 事后补授权。enable 只允许 APPROVED 或 DISABLED，重新验签并复验整张授权图后写 ENABLED；撤销 grant 的同一事务先写 DISABLED 与具体原因，再把该 grant 的 revoked_at 置位；REVOKED 是不可恢复终态，停用或撤销都保留制品、审批证据、grant 历史和业务数据。

#### 5.5 白标与客户端

| 方法与路径 | 请求 | 响应 data | 主要错误码 | 幂等语义 | 权限项 | Action |
|---|---|---|---|---|---|---|
| GET /api/v1/platform/brand-profiles/current | 无 | 唯一 ACTIVE 品牌配置，含产品名、Logo 与启动页的附件对象标识、主题色 | PLATFORM.BRAND_PROFILE.CURRENT_UNAVAILABLE（零行或检测到多行） | 只读，任何已认证用户可读；不任选一行 | 无需权限项 | — |
| GET /api/v1/platform/brand-profiles | 分页 | 品牌配置列表 | 无 | 只读 | brand.profile.view | VIEW |
| POST /api/v1/platform/brand-profiles | 全字段 | 品牌配置详情 | PLATFORM.BRAND_PROFILE.ASSET_INVALID、PLATFORM.BRAND_PROFILE.STORE_POLICY_CHECK_FAILED | 幂等键 | brand.profile.manage | CREATE |
| POST /api/v1/platform/brand-profiles/{id}/actions/activate | 无 | 品牌配置详情 | BUSINESS_CONFLICT、PLATFORM.BRAND_PROFILE.ASSET_INVALID | 幂等键 | brand.profile.manage | UPDATE |
| POST /api/v1/platform/client-releases | `{client,version,build_no,brand_profile_id,min_supported_version,is_forced_security_update,release_notes,locator:{kind:"ENTERPRISE_MDM_ARTIFACT",attachment_object_id}\|{kind:"APP_STORE_LISTING",store_listing_uri},artifact_hash?,artifact_size_bytes?}`；MDM 两项证据由服务端版本行覆盖，商店两项必须来自受信流水线清单 | 版本详情含固定 locator | VALIDATION、PLATFORM.BRAND_PROFILE.STORE_POLICY_CHECK_FAILED | 幂等键 | client.release.manage | CREATE |
| POST /api/v1/platform/client-releases/{id}/actions/roll-out | `{rollout_percent, rollout_legal_entity_ids, rollout_department_ids}` | 版本详情 | VALIDATION | 幂等键 | client.release.manage | UPDATE |
| POST /api/v1/platform/client-releases/{id}/actions/withdraw | `{reason}` | 版本详情 | 无 | 幂等键 | client.release.manage | UPDATE |
| GET /api/v1/platform/client-releases/check | `?client=&version=&build_no=` | `{action:"NONE" 或 "OPTIONAL" 或 "FORCED", target_version, release_notes, download_url}` | `PLATFORM.CLIENT_RELEASE.FORCED_UPDATE_REQUIRED` 只在已超出强制升级宽限的其他端点上返回 | 只读 | 无需权限项 | — |
| GET /api/v1/platform/client-releases/{id}/artifact | 无；必须携带已认证设备会话并通过该发布的灰度/法人/部门范围 | ENTERPRISE_MDM 固定附件版本的字节流，支持 Range；APP_STORE 返回 404 | 404、PLATFORM.AUTHZ.NOT_FOUND_OR_DENIED | 只读；永不暴露 storage_path | 无需额外权限项 | — |
| GET /api/v1/platform/client-capabilities | `?client=` | 该端 18 个能力域的取值与替代路径 | 无 | 只读 | 无需权限项 | — |
| GET /api/v1/platform/client-bootstrap | `?client=` | 见下 | 无；数据库矩阵与内置快照不同时以内置快照继续下发并拒绝矩阵写入 | 只读，但写一条 `client_bootstrap_dispatches` 与一条审计事件 | 无需权限项 | — |

白标激活是单一确定性事务。第一条锁语句固定为 `SELECT pg_advisory_xact_lock(hashtextextended('platform-meta:brand-profile-active',0))`，随后按 id 锁定目标 DRAFT 与当前 ACTIVE（如有），重新校验三项附件、签名引用及商店政策；目标已是唯一 ACTIVE 时零写入返回既有结果。否则先把旧 ACTIVE 置 SUPERSEDED 并清空 `active_slot`，再把目标置 ACTIVE 并写全零 `active_slot`，最后依次完成幂等结果、Outbox/通知与审计终结批后提交。SUPERSEDED 不可再激活。固定 advisory key 负责并发串行化，普通唯一键负责包括 direct SQL 在内的最终兜底；两个并发激活不同 DRAFT 必须形成一个确定串行序，提交后恰一 ACTIVE。`current` 查询读取至多两行：零行或异常多行一律返回 `PLATFORM.BRAND_PROFILE.CURRENT_UNAVAILABLE`、记录高优先级审计/告警且不下发任何默认品牌；它不以 `created_at`、code 或缓存任选一行。

`client-releases/check` 必须先完成已认证设备、client、法人/部门灰度与版本比较。ENTERPRISE_MDM 的 `download_url` 固定为本 API 的相对路径 `/api/v1/platform/client-releases/{id}/artifact`，内容端点再按发布行固定的版本读取并逐流校验哈希/总字节数；APP_STORE 的 `download_url` 恰为已校验的 `store_listing_uri`。WITHDRAWN、范围外、定位形状不合法或固定附件版本不可读时都不得返回下载地址；任何分支均不返回对象存储路径或可绕过设备会话的永久签名 URL。

`client-bootstrap` 的响应 data 结构：

```json
{
  "bootstrap_hash": "…",
  "capability_values": [{"capability_domain":"ledger.posting_close","value":"VIEW_ONLY","alternative_path":"…"}],
  "custom_objects": [{"code":"inspection_records","definition_version":7,"security_level":20,
    "fields":[{"code":"inspector","data_type":"STRING","max_length":200,"field_security_level":20}]}],
  "ui_layouts": [{"code":"…","layout_kind":"FORM","definition_version":3,"spec":{}}],
  "rules": [{"code":"…","version":5,"requires_wasm":false,"executable_on_client":true,"ast":{}}],
  "field_grants": [{"object_type":"ext.inspection_records","field_code":"inspector","can_read":true,"can_write":false}],
  "brand": {"product_name":"…","theme_primary_color":"#1A5FB4","logo_attachment_object_id":"…"},
  "device_policy": {"cache_ttl_days":7,"max_local_attachment_bytes":268435456,"native_plugin_enabled":false},
  "license_module_admin": null
}
```

`license_module_admin` 字段在所有响应中都存在。它只在已认证 `client=server_admin` 会话同时具备 `lowcode.config_package.view` 时非空；未认证、权限不足或任一其他 client 都逐字为 JSON null，不得省略、沿用上次非空缓存或返回部分对象。非空对象所有键始终存在，exact 字段为 `license_no_masked`、`license_kind`、`license_status`、`restriction_reason`、`valid_from`、`valid_to`、`maintenance_valid_to`、`last_trusted_at`、`usage`、`module_codes`、`entitlement_codes`、`modules`；前八项不可用的 Option 必须逐字序列化 JSON null，绝不省略。许可状态、原因和可信时间必须来自一次 `ModuleLicenseQuery::license_evaluation()` 的同一快照。`usage` exact object 恰有 `legal_entities,named_users,registered_devices` 三键，每项的 `limit,current,over_limit` 三键也始终存在，形状为 `{limit:u32|null,current:u64,over_limit:bool|null}`；`limit=null` 时 `over_limit=null`，否则严格等于 `current>limit`。即使没有可信 current grant 也返回三个实际 current，此时三个 limit/over_limit 都为 null。`modules` 恰有 15 行，每行 `module_code,display_name,install_state,package_trust_status,package_code,package_version,state_changed_at` 七键始终存在，后三项不可用时为 JSON null；`package_version` 非空时复用 strict `SemVerV1` object。`package_trust_status` exact 闭集为 `NOT_INSTALLED|TRUSTED|SIGNER_REVOKED|INVALID`，逐行从 current bundle 对 source item/投影重算，明确 install_state 不是 effective runtime 信任状态。可信 `license_no` 至少四个 Unicode scalar 时 `license_no_masked="****"+最后四个 scalar`，不足四个时固定 `"****"`，不得泄漏原长度。两个 code 集与 modules 均按 wire bytes 排序；unknown key、任一 missing key、把 null 改成省略或反向改形都属于 OpenAPI/序列化契约失败。

零 current 时 `license_status/restriction_reason` 固定为 `RESTRICTED/NO_CURRENT_GRANT`，其余许可身份、日期、可信时间都为 null，两个 code 集为空；`SIGNATURE_INVALID` 时同样不得显示来自未受信 current 行的身份、日期、code 或 limit。其他 Restricted 原因只显示已完整验签 current grant 的脱敏字段。两个 code 集与 15 行 modules 均按 wire bytes 排序；对象不含 signature、payload、source ref、path、key ref、secret 或原始 `license_no`。

该端点每次调用先写一条 `client_bootstrap_dispatches`，最后写一条审计终结事件并立即提交，落实规格第 7.4 章“下发范围可审计”；审计之后不得再执行数据库语句。响应缓存 TTL 由 `EP__CLIENT__BOOTSTRAP_CACHE_TTL_SECONDS` 控制，默认 300 秒；`definition_version` 或 `bootstrap_hash` 变化时客户端强制重取。

#### 5.6 版本化

本阶段全部端点为 v1。自定义对象端点的形状随对象定义变化，但路径与封套不变；新增字段属于向后兼容变更，不升主版本；删除或重命名字段由配置回退承担，不通过 API 版本表达。客户端必须容忍 `capability_values` 与 `item_kind` 出现未知取值并按未知降级处理，照抄基线第 5.6 节。

---

### 6. 并发与事务边界

#### 6.1 事务划分

| 操作 | 事务边界 | 隔离级别 | 锁策略 |
|---|---|---|---|
| 建模对象、字段、索引、视图、布局的增删改 | 一个用例一个事务 | READ COMMITTED | 第一业务 SQL 取 `LICENSE_CURRENT_SHARED`，再幂等/admission/行级乐观锁；编辑锁表按内容项粒度，TTL 1800 秒 |
| 配置包创建与内容项写入 | 一个事务 | READ COMMITTED | 普通 BusinessWrite 先取 `LICENSE_CURRENT_SHARED`，再 `try_begin` 与包行锁 |
| 配置包导入与验签 | archive 可在事务外 safe-parse，写入在一个事务内且事务外结论非权威 | READ COMMITTED | 共享 import 入口无条件先取 `LICENSE_CURRENT_EXCLUSIVE`，再持久化并从 exact bytes 重验；`ux_config_packages_content_hash` 承担去重 |
| 自动测试运行 | 受理、领取、lease/heartbeat、run 结果写入、最终汇总各为独立短事务；九个 suite 查询各自只读事务 | RLS_MATRIX 与 ROLE_PREVIEW 用 REPEATABLE READ，其余用 READ COMMITTED | 每个短写事务第一业务 SQL 重新取 `LICENSE_CURRENT_EXCLUSIVE`；suite 纯读取 NONE，不 claim/续租/汇总、不持包行锁 |
| 包签名 | 一个事务；普通包 KMS 签名调用在事务外完成后带入，special 只复验发行方签名 | READ COMMITTED | 共享 sign 入口无条件先取 `LICENSE_CURRENT_EXCLUSIVE`，再按 canonical tuple 锁行 |
| 发布单受理 | 一个事务 | READ COMMITTED | 共享入口无条件先取 `LICENSE_CURRENT_EXCLUSIVE`，再按 canonical tuple 锁 package/order/item |
| 发布协调与段二（连接 1） | ordinary 段一前开启 READ COMMITTED 事务，在 `BEGIN/SET LOCAL` 后先取 license advisory xact lock、再取 `config_release_mutex FOR UPDATE`、再锁 canonical package/order/item 与 module locks；F-56 special 同样先取 license lock，但不取 mutex 且跳过段一 | READ COMMITTED | 唯一总序为 license →（仅 ordinary）mutex → package/order/item → module；连接 1 持前置锁到 COMMIT，module locks 总 deadline 30 秒 |
| 发布执行段一（连接 2） | 每条 DDL 自动提交，不在事务块内 | 会话级 `lock_timeout=5s`、`statement_timeout=30min` | `create index concurrently` 不取表级排他锁；`add column` 取 ACCESS EXCLUSIVE 但受 5 秒超时约束 |
| 发布执行段三（传播） | 每个法人一个事务 | READ COMMITTED | 搜索索引分区重建期间该分区停止对外服务 |
| 扩展登记与能力授予 | 一个事务 | READ COMMITTED | 乐观锁 |
| 插件调用 | 不在任何写事务内 | 不适用 | 无数据库锁 |
| 调用流水写入 | 与调用点的业务事务同事务；无业务事务时单独一个事务 | READ COMMITTED | 仅追加 |

所有写用例采用同一收口顺序：先按各算法既定引用顺序完成业务事实、子账/凭证和同步投影，再执行幂等 `finish`，再刷新 Outbox，再写确需同事务落库的通知命令，最后调用 `AuditWriter::append_terminal` 批量落审计；不存在的类别跳过但后缀不得调换。`append_terminal` 封印 `Tx`，其后任何 SQL `query/execute`、仓储写入或跨模块端口调用都必须以内置不变量错误拒绝并令整个事务回滚，唯一允许的后继动作是 `UnitOfWork::commit`。共享跨模块契约测试 `audit_terminal_seals_tx` 以自动测试受理、配置发布段二和插件自动停用为夹具：审计后分别尝试元数据仓储、任一 `ConfigItemApplier`、Outbox 与通知写入，均应失败、审计后零新增行、事务整体回滚；正常路径由 recording transaction 断言审计批是 commit 前最后一批数据库执行。

#### 6.2 发布的串行化

`platform_meta.config_release_mutex` 是单行表，只由 ordinary execute 使用；F-56 special execute 不取该行锁并跳过 DDL 段一。ordinary 的连接 1（协调连接，`ep_app_rw`）在段一之前 `BEGIN/SET LOCAL`，第一条业务 SQL 先取 `LICENSE_CURRENT_EXCLUSIVE`，第二把锁才是 `select * from platform_meta.config_release_mutex where id = '000…0' for update`，随后按 canonical tuple 锁 package/order/item、再取 module locks，并把 license 与 mutex 一直持有到同一事务 COMMIT；连接 2（`ep_migrator`）只逐条自动提交 DDL。全部 DDL 成功后，连接 1 自己在仍持锁的同一事务中执行段二、幂等 `finish`、Outbox、同事务通知命令和审计终结批，然后 COMMIT，同时释放前述锁。一次执行尝试的数据库连接精确为这两条；禁止第三条连接执行段二、重取锁、代提交或“专门释放锁”。第二个并发 ordinary 发布请求在 `lock_timeout` 内取不到前置锁即返回 409 与 `PLATFORM.CONFIG_RELEASE_ORDER.CONCURRENT_RELEASE_IN_PROGRESS`。

异常收敛同样使用这两条连接。连接 2 的 DDL 超时或失败时立即停止，连接 1 记录安全补偿/保留结果与 FAILED 后提交释放；连接 2 断开时已提交语句以 `ddl_plan_steps + information_schema` 对账，当前未提交语句由 PostgreSQL 回滚。连接 1 断开会由 PostgreSQL 自动回滚并释放锁，执行器必须同步取消并关闭连接 2；若整个 worker 崩溃，两条 TCP 连接均关闭。重启后的新执行尝试可以重新建立一对连接，但须先按步骤事实与物理目录幂等对账，已成功 DDL 不重放，未知结论先隔离为 DDL_FAILED；这不属于在原尝试中引入第三连接。固定故障点为取锁后、DDL 前、DDL 自动提交前后、最后一条 DDL 后段二前、段二审计前和 COMMIT 前；任何点恢复后都只能出现一份发布效果、互斥行可再次取得、row_count/checksum 不变。

#### 6.3 幂等键与 Outbox

- HTTP 层：全部写端点必带 `Idempotency-Key`，作用域为法人、用户、端点、键值四元组，存储在 `platform_msg.idempotency_keys`，与业务写入同事务，照抄基线第 5.4 节。部署级配置端点的法人取该请求头 `X-Legal-Entity-Id` 的取值，即同一动作在不同法人上下文下发起视为不同的幂等作用域；这是本阶段的取值，理由是部署级配置对象无法人列但请求仍带法人上下文，若不纳入四元组会使两名分属不同法人的配置管理员的重放互相干扰。成功结果的 `finish` 必须晚于全部业务/投影，早于 Outbox、通知命令和审计终结批。
- Outbox：发布成功在段二事务完成幂等 `finish` 后写入 `platform.config_release.released.v1`，`idempotency_key` 取发布单标识；其后只允许同事务通知命令与审计终结批。段三消费端幂等由 `platform_msg.inbox_consumptions(consumer, event_id)` 唯一约束保证。
- 重试退避照抄基线第 6.2 节的 8 次；全部失败置 DEAD 并写死信，按 PRD 第 10.5.2 节通知责任人。
- 死信重投必须记名并写审计；丢弃需要双人审批。

#### 6.4 失败重试与补偿

- 数据库序列化失败 40001 与死锁 40P01 在数据访问层重试 3 次，退避 50、150、450 毫秒，只对尚未产生外部可见副作用的事务重试，照抄基线第 8.4 节。DDL 段不适用该重试，DDL 失败只走第 4.3 节的数据无损安全补偿。
- 段一只逆序撤销索引、策略或约束等经证明不会删表、删列、删行的结构；已新增表/列保留并隔离。段二的补偿是连接 1 自身事务回滚；跨段失败是段二未提交加同一安全补偿，不存在 DROP COLUMN/TABLE 伪补偿。安全补偿不完整时发布单置 COMPENSATED 并进入人工任务队列并告警，不得静默结束；进入该状态前后均核对受影响表 row_count 与业务列 checksum 不变。
- 插件调用失败不触发业务事务重试，由调用点按第 4.8 节的错误映射向上返回；连续失败达阈值自动停用该扩展。

#### 6.5 与规格第 10.2 章关账的关系

配置发布不产生会计分录，因此不进入关账受理前提的判定。但发布执行会占用磁盘 IO 与数据库连接，按规格第 13.1 章让路顺序属第 8 级“其余后台任务”；发布执行器在检测到期间关账请求已受理且尚未产生结论时暂停新的发布单执行，已在执行中的发布单继续执行至终态。这是本阶段的取值，理由是关账前的强制对账校验对同一快照敏感，DDL 在此期间执行会使校验语句的执行计划失效。

---

### 7. 配置项

全部键名前缀 `EP__`，层级用双下划线，结构体开启 `deny_unknown_fields`，照抄基线第 7.1 节。

| 键 | 类型 | 默认值 | 生效方式 |
|---|---|---|---|
| EP__CLIENT__BOOTSTRAP_CACHE_TTL_SECONDS | u32 | 300 | 下次引导生效 |
| EP__CLIENT__MOBILE_BATCH_MAX | u16 | 50 | 启动时生效 |
| EP__CLIENT__DESKTOP_BATCH_MAX | u16 | 200 | 启动时生效 |
| EP__CLIENT__HANDOFF_TOKEN_TTL_SECONDS | u32 | 300 | 启动时生效 |
| EP__CLIENT__CACHE_TTL_DAYS__DESKTOP | u16 | 14 | 下次引导生效 |
| EP__CLIENT__CACHE_TTL_DAYS__MOBILE | u16 | 7 | 下次引导生效 |
| EP__CLIENT__MAX_LOCAL_ATTACHMENT_BYTES__DESKTOP | u64 | 5368709120 | 下次引导生效 |
| EP__CLIENT__MAX_LOCAL_ATTACHMENT_BYTES__MOBILE | u64 | 268435456 | 下次引导生效 |
| EP__CLIENT__NATIVE_PLUGIN_ENABLED_DEFAULT | bool | true | 下次引导生效 |
| EP__LOWCODE__MAX_CUSTOM_OBJECTS | u16 | 200 | 启动时生效 |
| EP__LOWCODE__MAX_FIELDS_PER_OBJECT | u16 | 100 | 启动时生效 |
| EP__LOWCODE__MAX_INDEXES_PER_OBJECT | u8 | 5 | 启动时生效 |
| EP__LOWCODE__DDL_LOCK_TIMEOUT_MS | u32 | 5000 | 启动时生效，取值不得大于 5000 |
| EP__LOWCODE__DDL_STATEMENT_TIMEOUT_MS | u32 | 1800000 | 启动时生效，取值不得大于 1800000 |
| EP__LOWCODE__DDL_MAX_STATEMENTS_PER_PLAN | u16 | 200 | 启动时生效 |
| EP__LOWCODE__EDIT_LOCK_TTL_SECONDS | u32 | 1800 | 启动时生效 |
| EP__LOWCODE__RULE_MAX_AST_NODES | u16 | 500 | 启动时生效 |
| EP__LOWCODE__RULE_MAX_EVAL_DEPTH | u8 | 32 | 启动时生效 |
| EP__RELEASE__ROLLBACK_KEEP_PACKAGES | u8 | 10 | 启动时生效 |
| EP__RELEASE__ROLLBACK_MAX_AGE_DAYS | u16 | 180 | 启动时生效 |
| EP__RELEASE__PACKAGE_MAX_BYTES | u64 | 67108864 | 启动时生效 |
| EP__RELEASE__PACKAGE_MAX_ITEMS | u16 | 2000 | 启动时生效 |
| EP__RELEASE__SIGNING_KEY_REF | string | "secret://config/release_signing#1" | 取用时解析，轮换不重启 |
| EP__RELEASE__TRUSTED_SIGNER_SUBJECTS | list of string | 空 | 仅作 signed `DeploymentManifestV1.license_trusted_signer_subjects` 的可选 exact-equal 断言；空表示使用 signed roster 且不覆盖，非空必须长度/顺序/token 逐字相等，不等则 readiness/ops fail，绝不得增删 signer |
| EP__RELEASE__PAUSE_DURING_PERIOD_CLOSE | bool | true | 启动时生效 |
| EP__PLUGIN__MAX_INSTANCES | u16 | 8 | 启动时生效 |
| EP__PLUGIN__DEFAULT_FUEL | u64 | 200000000 | 启动时生效 |
| EP__PLUGIN__DEFAULT_MEMORY_BYTES | u64 | 67108864 | 启动时生效 |
| EP__PLUGIN__EPOCH_TICK_MS | u32 | 100 | 启动时生效 |
| EP__PLUGIN__CALL_TIMEOUT_MS__TRANSACTIONAL | u32 | 2000 | 启动时生效 |
| EP__PLUGIN__CALL_TIMEOUT_MS__WORKER | u32 | 30000 | 启动时生效 |
| EP__PLUGIN__COMPILE_CACHE_DIR | path | "C:\EP\plugin-host\cache" | 启动时生效 |
| EP__PLUGIN__AUTO_DISABLE_FAILURE_THRESHOLD | u8 | 3 | 启动时生效 |
| EP__PLUGIN__TRUSTED_SIGNER_SUBJECTS | list of string | 空 | 启动时生效，为空时不加载任何扩展 |
| EP__BRAND__ACTIVE_PROFILE_CODE | string | "default" | 下次引导生效 |

敏感项按基线第 7.2 节只写 `secret://` 引用，内存中以 `secrecy::SecretString` 包装。

启动自检的追加项按裁定 C-25 改为命名项，基线第 7.3 节的十项固定项一并按注册名标识，本阶段不再以序号称呼，需回写基线。按裁定 C-25，自检项分 Blocking 与 Degrading 两级，本阶段追加的一项为 Degrading 级：它判读的是数据库里的业务行，而这台服务器没有备节点，用一次数据判读阻断九个进程的启动是把可用性押在一次判读上。

- `custom-object-ddl-consistent`（Degrading）：`ext` schema 下全部表均已 `ENABLE` 且 `FORCE` 行级安全，且各自存在 `rls_<table>_le` 策略；`platform_meta.custom_objects` 中 status 为 ACTIVE 的每个对象在 `ext` 下均有对应物理表，反之不存在孤立物理表。不一致时把相关 `custom_objects` 置 DDL_FAILED 并隔离其全部入口，经 `ep_platform_obs::DegradationLedger` 的 `open` 与 `close` 登记一个 kind 取 `CUSTOM_OBJECT_DDL_INCONSISTENT` 的降级窗口并持续告警，不阻断启动；孤立物理表不隔离任何入口，只告警。

原拟追加的 `client-capability-matrix-frozen` 按裁定 C-25 整项撤销，不进注册表，也不为其定任何 `DegradationKind` 取值，其承载改为第 4.4 节第 5 条的二进制内置冻结快照。

裁定 C-25 为本阶段固定了上述一个项名，因此原有的另外两项校验不再作为启动自检项：status 为 ENABLED 的扩展其制品可读、哈希一致、签名可验、`capability_manifest` 未超出已授予能力，改由第 4.8 节第 1 条的加载路径逐次校验，不通过即不加载并写审计；当前生效的品牌配置引用的附件对象存在且可读，改由 `POST /api/v1/platform/brand-profiles/{id}/actions/activate` 用例在激活前校验，不通过返回 `PLATFORM.BRAND_PROFILE.ASSET_INVALID`。

`--check` 模式按 `SelfCheckRegistry` 的注册顺序一并执行全部注册项，基线十项在前，本阶段一项在后；`--check` 对 FAILED 与 DEGRADED 一律非零退出，因此部署与升级前置仍是严格闸门，进程启动不是。

---

### 8. 测试计划

#### 8.1 单元测试

覆盖分支逐项列出。

1. DDL 计划生成器：第 4.3 节第 3 步映射表六行差异各自的语句序列与执行模式判定；混合差异时整体执行模式取最严者；语句数超 200 时拒绝。
2. 基线校验器：11 种字段类型逐个通过、超出基线的类型逐个拒绝；3 种索引类型逐个通过、函数索引与局部索引与 JSON 路径索引的表达一律无法构造；JSON 列建索引拒绝；JSON 列设 CHECK 拒绝。
3. 密级校验：对象级为空拒绝；字段级为空时继承对象级；两者均为空拒绝。
4. 保留列名：公共列九项加第 3.2.2 节列出的九个专属列共十八个名字逐个拒绝。
5. 影响分析五项：各自在空差异、单表小差异、多表大差异三种输入下的输出结构完整性。
6. 配置包状态机：11 个状态、第 4.2 节表列出的 11 条合法迁移全部通过；任意两状态之间的非法迁移全部返回对应错误码；自审批拒绝。
7. 发布单状态机：9 个状态与其合法迁移；回退目标过期拒绝；回退目标非上一 RELEASED 包拒绝。
8. 签名与验签：正确签名通过；篡改 manifest 一个字节拒绝；篡改任一内容项一个字节拒绝；签名主体不在信任列表拒绝；`min_platform_version` 高于当前版本拒绝。
9. 差异算法：新增、修改、删除三类内容项的 diff 输出；同一内容项在两包中完全一致时不进入差异。
10. 能力矩阵判定：四个取值乘五个动作类别共二十种组合的判定结果；同一用例落入两个能力域时取较低者；未知端取值按未知降级。
11. 规则 AST 求值：每个节点类型的正常与边界输入；Decimal 精度按基线第 3.5 节，含四舍五入中值远离零的三组样例；节点数 500 与 501 的分界；深度 32 与 33 的分界；除零、空值比较、集合为空、区间上下界颠倒五类边界。
12. 插件能力裁剪：授予字段子集时输入报文只含子集；未授予任何 `READ_OBJECT_FIELDS` 时输入为空；声明超出授予的能力拒绝加载。
13. 插件资源限额判定：燃料、内存、时限三类各自的中止分类映射。
14. 品牌资源校验：产品名超长、颜色格式错误、应用标识不合规、Logo 尺寸不符四类各自拒绝。
15. 编号生成：自定义单据对象的编号格式按基线第 11.1 节，法人加类型加年月三元组独立自增，流水溢出扩位；`doc_type_code` 与 `docs/data-dictionary.md` 单据类型码一节全量表重复时按裁定 C-26 拒绝并返回 `PLATFORM.CUSTOM_OBJECT.DOC_TYPE_CODE_CONFLICT`。

#### 8.2 领域属性测试（proptest）

对应规格第 17.2 章的领域属性测试类型，本阶段贡献四组不变量。

1. 规则求值全域性：任意合法 AST 在任意合法输入上求值不 panic、不越界、不整数溢出，且求值节点数不超过 AST 节点数乘输入明细行数。
2. 元数据往返：任意合法对象定义序列化为配置包内容项再解析回来，与原定义相等。
3. 发布与回退的元数据段幂等性：任意合法配置包 apply 后 revert，`platform_meta` 中除 `definition_version`、审计与仅追加表之外的全部行与 apply 之前逐列相等。
4. DDL 计划数据无损收敛：对任意合法且不含 DROP COLUMN/TABLE 的语句序列，在任意前缀位置注入失败后执行安全补偿，所有既存表的 row_count 与按主键排序的业务列 checksum 恒等于执行前；索引、策略与约束恢复到安全可用形状，已新增表/列允许保留但必须在元数据中为 DDL_FAILED/RETIRED 且 API/UI/查询注册表不可见。生成的普通计划永不出现 DROP COLUMN/TABLE。

#### 8.3 集成测试（真实 PostgreSQL 16，禁用内存库与 mock）

每个用例独占一个数据库，按 `ep_test_<nanoid>` 建库，用例结束即删库。测试数据一律经 `ep-testkit` 构造器，禁止手写 INSERT。

| 序 | 场景 | 判据 |
|---|---|---|
| 1 | 新建自定义对象并发布 | `ext` 表存在；`enable` 与 `force` 行级安全均为真；`rls_<table>_le` 策略存在；三条基线索引存在 |
| 2 | 两法人越权矩阵 | 对该自定义对象执行读取、写入、更新、删除、聚合、排序、报表投影与错误信息泄漏八类，另覆盖两个复制角色与内部对账系统安全上下文的五个入口借用，全部不返回对方法人数据 |
| 3 | 新增可空列的锁持有 | 在 100 万行 `ext` 表上执行，`ddl_plan_steps.lock_wait_ms` 加执行时长不超过 5000 毫秒 |
| 4 | 新增索引的执行时长 | 在附录 A.3 基准数据集规模的 `ext` 表上 `create index concurrently` 不超过 30 分钟；建成后目标查询的 `EXPLAIN` 无顺序扫描 |
| 5 | 锁超时的安全收敛 | 人为持有冲突锁使 `add column` 超时，计划置 ROLLED_BACK 与 DEFERRED_TO_WINDOW；审计事件含原因、对象与耗时；受影响表 row_count/checksum 不变，执行记录不含 DROP COLUMN/TABLE，若此前已有自动提交新增结构则保留且入口隔离 |
| 6 | 元数据与 DDL 的一致化 | 在 DDL 执行成功后、元数据置 ACTIVE 之前杀死 job-worker，重启后启动自检项 `custom-object-ddl-consistent` 检出不一致，进程照常启动，相关 `custom_objects` 置 DDL_FAILED 且其全部入口被隔离，降级窗口已开；执行修复用例后该项通过且降级窗口关闭 |
| 7 | 导入包篡改与 special 外层签名保留 | 普通项篡改任一字节返回 `PLATFORM.CONFIG_PACKAGE.ITEM_HASH_MISMATCH` 且不落库；F-56 特殊项另分别篡改外层 item/hash/signature 与内层 payload/CMS，外层失败不能替代内层验证，任一失败均整包零状态变化。special 正例在 import 保存发行方 outer signature/signer/signed_at exact 值且 `signature_key_ref=null`，批准后 actions/sign 复验并推进但三值逐字不变、部署 `KmsBackend::sign` 调用为零；普通包 actions/sign 仍恰调用一次部署 KMS，内层只读固定 license-roots.p7b 而不读 Windows roots |
| 8 | 自动测试未通过阻止提交 | RLS_MATRIX 注入一条越权可读策略，suite 置 FAILED，提交发布单返回 `PLATFORM.CONFIG_PACKAGE.AUTOTEST_NOT_PASSED` |
| 9 | CONFIG_RELEASE 自审批与回调唯一写者 | submit 同事务持久化 scenario、申请人、approval_ref、链 id/版本/digest、内容 version/hash 并启动流程；提交人完成 approve 返回 `PLATFORM.SOD.SELF_APPROVAL_FORBIDDEN`。非申请人调用 approve/reject 时路由只完成对应 task，直接绕过 callback 更新包被仓储/状态机拒绝；合法 callback 才迁移包且 approved_by/rejected_by 记名 |
| 10 | 两连接并发发布 | 连接 1 持互斥行锁、连接 2 自动提交 DDL 时发起第二发布，后者返回 `PLATFORM.CONFIG_RELEASE_ORDER.CONCURRENT_RELEASE_IN_PROGRESS`；`pg_stat_activity` 与记录型连接工厂断言每次尝试精确两连接，段二与释放均由原连接 1 完成，从未建立第三连接，数据库结构与元数据无交叉污染 |
| 11 | 发布幂等 | 同一 `Idempotency-Key` 重复调用 execute，只产生一份 `config_release_steps`，第二次响应头带 `Idempotent-Replay: true` |
| 12 | 普通回退不删数据且特殊项不可回退 | 发布新增字段、录入 1000 行数据、回退，字段元数据置 RETIRED，API/UI 不再返回该字段，物理列与 1000 行数据仍可由受控 `ep_analyst_ro` 取证；回退前后 row_count/checksum 相等且无 DROP。对含 LICENSE_GRANT/MODULE_PACKAGE 的包创建 ROLLBACK 稳定返回 NON_ROLLBACKABLE，零 release order、零 applier 调用、历史包/item/投影不变 |
| 13 | 回退触发重新打标 | 权限类内容项回退后，搜索索引对应法人分区进入重建，重建期间检索入口返回 `PLATFORM.CONFIG_RELEASE_ORDER.DERIVED_STORE_REBUILD_REQUIRED`，重建完成后条数与来源一致 |
| 14 | 派生存储越权 | 以跨法人与跨密级安全上下文对自定义对象发起检索、排序与分面计数，均不返回无权数据 |
| 15 | 插件签名不符 | 篡改制品一个字节，enable 返回 `PLATFORM.EXTENSION.SIGNATURE_INVALID`；对已 ENABLED 的扩展篡改制品后按第 4.8 节第 1 条不加载并写审计，后续调用返回 `PLATFORM.EXTENSION.DISABLED` |
| 16 | 插件燃料耗尽 | 构造死循环插件，调用返回 `PLATFORM.EXTENSION.RESOURCE_LIMIT_EXCEEDED`，`extension_invocations.outcome` 为 FUEL_EXHAUSTED |
| 17 | 插件自动停用 | 同一入口连续 3 次失败，扩展置 DISABLED，第 4 次调用返回 `PLATFORM.EXTENSION.DISABLED`，审计与站内通知各一条 |
| 18 | 插件无 IO 能力 | 构造尝试打开套接字与文件的插件，编译期即因缺少导入项失败；宿主导入表断言只有四个函数 |
| 19 | plugin-host 零连接 | 断言 `pg_stat_activity` 中不存在来自 plugin-host 的连接；断言 plugin-host 的配置中不含数据库连接串 |
| 20 | 插件在事务外调用 | 静态检查 `ep-app-*` 的用例函数中不出现 plugin IPC 客户端符号；运行期断言插件调用发生时当前连接无活动事务 |
| 21 | 能力闸 | `X-Client: ios` 调用付款登记提交返回 403 与 `PLATFORM.CLIENT_CAPABILITY.WRITE_NOT_AVAILABLE_ON_CLIENT` 并带 `X-Desktop-Handoff-Token`；`X-Client: win` 放行；`X-Client: ios` 调用扩展登记返回 404 |
| 22 | 矩阵冻结 | 篡改 `client_capability_values` 一行，进程照常启动，运行期判定以内置快照为准，对该表的写入被拒绝且告警已发；`--check` 模式在同一状态下以非零退出 |
| 23 | 引导下发可审计与许可摘要不泄漏 | 调用 `client-bootstrap` 一次，`client_bootstrap_dispatches` 增一行含对象清单与规则版本，审计事件增一条；未认证、ServerAdmin 缺 view 权限及 win/mac/ios/android 分别断言字段存在且 `license_module_admin=null`。有权限 ServerAdmin 断言非空对象 12 键、usage 三键与每项三键、15 行模块及每行七键始终存在且排序，每行必有 `package_trust_status=NOT_INSTALLED\|TRUSTED\|SIGNER_REVOKED\|INVALID`；构造 raw INSTALLED_ENABLED 但 signer revoked/投影 invalid，断言 install_state 保持管理事实而 effective runtime 关闭。零 current 和 SIGNATURE_INVALID 逐项断言身份/日期/code/limit 的 null/空集合规则、三个 actual current 常在、limit/over_limit 联动 null，其他 Restricted 只显示已验签脱敏字段；status/reason/trusted time 来自一次 `license_evaluation`，package 三个不可用字段显式 null，号码遮罩 exact，响应不含 signature/payload/source/path/key/secret/原始 license_no。逐项删除/增加顶层、非空对象、usage 或 module key，并把 null 与省略互换，OpenAPI/serializer contract 必须全判红 |
| 24 | ep_migrator 连接的启用与回收 | 一次含 DDL 的发布产生恰好两条审计事件（启用与回收）；发布前后 `pg_stat_activity` 中 `ep_migrator` 连接数为 0 |
| 25 | 配额上限 | 对象数达 200、单对象字段数达 100、单对象索引数达 5 时，再新增返回 `PLATFORM.CUSTOM_OBJECT.QUOTA_EXCEEDED` |
| 26 | F-56 模块生命周期闭图 | 以部署级 DTO（断言无 `legal_entity_id`）跑五条合法边与所有非法态×动作；INSTALL/ENABLE/UPGRADE/ROLLBACK_VERSION 分别制造目标或依赖未获 current grant 授权并硬拒绝，INSTALL 只到 disabled 且依赖无需 enabled，ENABLE 则逐依赖要求 INSTALLED_ENABLED。首次 INSTALL/新 UPGRADE 的 inner+outer 必须 ACTIVE；ENABLE/DISABLE 复用 RELEASED inner（可 RETIRED、不可 REVOKED）且新 outer ACTIVE；ROLLBACK_VERSION 精确引用 RELEASED history inner（可 RETIRED、不可 REVOKED）且新 outer ACTIVE。Restricted 下 DISABLE 完整可用但旧 SignedBusinessArtifact 必须与 current exact；inner signer CRL REVOKED 时只允许 ACTIVE outer+旧 inner exact bytes 的 DISABLE，旧 accepted/source/payload/digest/signature 自洽且唯一失败为撤销，重签 inner/其他 action/bad digest-signature-source-chain 均拒。停用后仅 inner+outer ACTIVE、semver 更高的新 UPGRADE 可替换 revoked projection，旧包只留历史。逐动作时间投影、30 秒排空、数据保留均验证；`module_state` 只返回 raw，`module_is_currently_licensed` 对合法负态 Ok(false)、结构/IO/digest/source/catalog 歧义 Err，feature 还过 owner module effective gate |
| 27 | 编辑锁 | 两名配置管理员同时编辑同一对象，第二人返回 `PLATFORM.CONFIG_EDIT_LOCK.HELD_BY_ANOTHER_USER`；锁过期后可取得 |
| 28 | 关账期间暂停发布 | 受理一次关账请求后提交发布单执行，返回受理但排队；关账产生结论后自动继续执行 |
| 29 | 迁移窗口未打开 | 未登记打开的迁移窗口时执行含 DDL 段的发布，`MigrationWindowGuard::assert_open` 拒绝，返回 409 与 `PLATFORM.DB.MIGRATION_WINDOW_CLOSED`，全程 `pg_stat_activity` 中不出现 `ep_migrator` 连接；打开窗口后同一发布单执行成功 |
| 30 | 自动测试受理原子性 | 调用 `run-autotest` 后包与审计同事务置位，响应恰有九个 `run_id`，库内同 batch 恰有九条 QUEUED 且 suite 为精确集合；事件表零新增 |
| 31 | 自动测试租约恢复 | 验证 60 秒租约与 20 秒条件续租；worker 在第四套执行中崩溃，租约过期后另一 worker 以 `SKIP LOCKED` 领取同 batch，将遗留 RUNNING 行恢复为 QUEUED 并只补完未 FINISHED 行，已完成行不重跑；再让原 worker 尝试续租与写 run 终态，两次均影响零行且无法覆盖最终状态 |
| 32 | 语义失败不重试且报告完整 | 让 RULE_SEMANTICS 断言失败；该套只执行一次并置 FAILED，其余适用套件仍完成，包置 TEST_FAILED，报告包含九套终态或合法 SKIPPED |
| 33 | 基础设施重试耗尽与逐套件到期隔离 | 对两套在不同时间注入可重试基础设施错误；各自精确按 1s/5s/30s/2m/10m/30m/1h/2h 写运行行 `available_at`，包级 `autotest_available_at` 始终等于未完成行最小值，领取较早一套时不得提前运行较晚一套；每次清租约让 worker 返回轮询而非长时睡眠，其他已到期 suite 仍完成；第八次重试仍失败后该行以 `failure_count=9` 置 FINISHED/FAILED，九行都 FINISHED 后包才置 TEST_FAILED，错误已清洗，不写死信或新事件 |
| 34 | 注册表精确集合 | 分别构造缺项、重项、额外项，job-worker 均拒绝启动；九项精确注册时启动成功，`COMPENSATION_POLICY` 不可注册 |
| 35 | CONFIG_RELEASE 链失败关闭与内容绑定 | 依次制造无活动链、两条活动历史脏链、零节点、空角色、申请人落入节点，submit 分别返回 `CHAIN_NOT_FOUND`、`ACTIVE_CHAIN_AMBIGUOUS`、`NODE_HAS_NO_APPROVER`、`NODE_HAS_NO_APPROVER`、`SELF_APPROVAL_FORBIDDEN` 且包仍 TEST_PASSED、全域零写入；正常提交后分别篡改 package id、approval_ref、scenario、chain version/digest、content_version/hash、当前 task 顺序，approve/reject 均整体回滚，只有完全匹配且非自审的回调可迁移 |
| 36 | 两连接超时与崩溃恢复 | 在取锁后、首条 DDL 提交前后、末条 DDL 后段二前、段二审计前、COMMIT 前逐点杀连接 1、连接 2 或 worker；恢复前两连接均关闭，恢复尝试重新建立一对连接并依步骤事实/物理目录收敛，已提交 DDL 不重复，互斥锁可取得，发布效果至多一份且 row_count/checksum 不变；任一 attempt 从未出现第三连接 |
| 37 | 普通 REMOVE 与中途失败无损 | 对对象、字段各发一个 REMOVE，断言只置 RETIRED 并从 API/UI/查询注册表隐藏，物理表/列、行数与 checksum 不变；再让包含 CREATE TABLE、ADD COLUMN、CREATE INDEX 的计划在每个前缀失败，断言只撤销安全结构、已新增表/列保留隔离、无 DROP COLUMN/TABLE；直接导入带 DROP 或等价动态 SQL 的普通包在计划生成前拒绝。物理处置正例只由阶段 14 双人审批 `DisposalPort` 契约测试承担 |
| 38 | 扩展固定版本与跨表能力约束 | 登记时上传对象 current 版本为 V1，登记后把同对象 current 推进 V2，加载仍只读 V1 且哈希/大小一致；AVAILABLE→SUPERSEDED 后引用继续成立；PENDING/QUARANTINED/FAILED、错法人、错版本、错哈希/大小均由 `fk_extensions_artifact_identity` 在 direct SQL 层拒绝。直接 UPDATE kind、制品定位、哈希/大小、签名、manifest、资源限额或目标平台由 `assert_extension_identity_immutable` 拒绝；给 SERVER_WASM 插入 DEVICE_PRINTER/SMARTCARD grant、伪造 extension_kind 或给非字段能力填 object/fields 均由 CHECK/复合 FK 拒绝；`pg_catalog` 中父候选键、两个制品复合 FK、不可变触发器与列序逐项存在 |
| 39 | 白标激活串行化与单值读取 | 两连接同时激活不同 DRAFT，固定 advisory key 下两事务均正常结束并形成确定先后，最终恰一 ACTIVE、另一 SUPERSEDED；direct SQL 插第二个 ACTIVE 由普通唯一键拒绝。零 ACTIVE 时 current 返回 CURRENT_UNAVAILABLE 且无默认回退；临时禁用约束构造多 ACTIVE 的损坏库时同样 fail-closed、告警且不任选一行 |
| 40 | 客户端发布定位形状 | 对 ENTERPRISE_MDM 证明发布行固定精确附件版本，更新对象 current 不改变下载正文；对 APP_STORE 证明只返回该 client 白名单 HTTPS 商店 URI。两类 locator 混填、全空、错品牌 distribution_channel、非 HTTPS/userinfo/fragment/错主机、固定版本非可读态均在建发布或 roll-out 前拒绝 |
| 41 | 客户端制品下载授权 | MDM 发布范围内设备经相对 artifact 路由支持 Range 并取得哈希/大小一致的固定版本；跨法人、范围外、未认证、APP_STORE、WITHDRAWN 与伪造 release id 均不返回正文或 storage_path，check 也不返回可绕过会话的永久 URL |
| 42 | EXTENSION_ENABLE 审批与授权图 | 空链、多活动链、空节点、空角色与申请人自审均整笔失败且扩展保持 REGISTERED；正常提交冻结 artifact/manifest/requested-grants 三摘要与链版本。approve/reject 路由只完成标准任务，直接写 APPROVED、直接插 grant 或绕 callback 均失败；callback 批准后有效 grants 精确等于请求快照且不超 manifest。以 direct SQL 构造普通 FK 全命中但错 approval_ref、空 object 重复、超 manifest、少一项 grant、ENABLED 无批准证据、撤销 grant 未先 DISABLED、DISABLED 无 reason、REVOKED 回生均在 COMMIT 被约束拒绝；合法撤销后重授保留历史行且恰一 active_slot=1 |
| 43 | F-56 许可四态、可信时间、计量与恢复闭集 | 零/首张 current、五个 Restricted reason、PERPETUAL/SUBSCRIPTION 边界、撤销与倒拨全覆盖；`TrustedClockV1` 用可控 wall+monotonic 验证已验 hash-chain 的 bootstrap/checkpoint 启动 anchor、同进程不降、query 零写。readiness、special 与 target cadence 不超过 240 秒的 checkpoint 均在 `BEGIN/SET LOCAL` 后以第一条业务 SQL 取 `LICENSE_CURRENT_EXCLUSIVE`；`ensure_checkpoint` 在业务 mutation 前冻结 current grant/revocation/trusted_now snapshot，`slot_utc=floor(unix/240)*240`，同 slot 0 行追加、1 行保持 exact bytes 复用、>1 失败，terminal writer 不重算。uptime 下 gap<=300 秒 PASS、gap>300 秒 fail；typed reject 在事务前固定 NONE。普通 BusinessWrite/Approval/IntegrationOutbound/AutomationStart 取同 key shared 并发，exclusive 排空；Outbox claim 用 transaction shared，外发用专用 session shared+module shared 且不跨外部调用持 DB 事务。autotest 短事务全 exclusive，suite 纯读 NONE；所有反序 SQL 在执行前失败且合法并发无死锁。CAB 轮换扫全部 RELEASED special，signed signer roster 必须是全历史 inner+outer token exact superset，删引用 token 失败，保留 revoked token 仍由 CRL 得 REVOKED。窄 certificate/CRL extension profile、整链四态、算法/时间/CRL prerequisite 负矩阵全跑。治理法人、两恢复链、实时三计数、十 effect、scope/None、pre-auth、Win/Mac 审批与 InFlight 全闭合 |
| 44 | F-56 特殊单项包、ServerAdmin 上传与数据库终态 | 断言两种 exact after_spec、special shape/不可回退与八类 ConfigRelease binding；ADD/MODIFY-after、REMOVE-before hash/null 负例全过。`ConfigItemApplier::validate` 用 fake DB/KMS/file/current ports 证明调用数全零，只有全局锁内从 persisted exact bytes 重跑全部守卫的 apply 可提交。`.epcfg` 的 ZIP32/STORE 三 entry、330 overhead、各 cap、canonical manifest/item/CMS 全闭合；multipart 覆盖 boundary/filename/header/MIME/CRLF/零额外 part、双 cap、Content-Length/Transfer-Encoding/短长读并统一 INVALID_PAYLOAD 400。普通 KMS outer 逐项验证 secret ref 单次解析、immutable KeyRef、identity-before/sign/verify=true/identity-after、canonical SPKI token与同事务落库，Builtin/HSM 正例及轮换竞态、假 token/错 ref/verify=false 负例全过；special 的 KMS/resolver 调用均为零。fresh PostgreSQL 核对090100 固定0601..0615 catalog、090200治理列、090500 Rust/DB18+accepted且无父键、Stage13 090500 Rust/DB20且不重复列、093300唯一键/六FK/五表 deferred 图；接受摘要、治理法人与两套 package-history identity 均有 COMMIT 负例。15 descriptors/schemas/compiled registry/product manifest exact-set、safe-handle与 ModuleOperationGate 的业务/worker shared、ENABLE目标exclusive+依赖shared、DISABLE全15 exclusive/总30秒全测。首装固定 evidence root/DACL/safe-handle、signed key_domain_id、exact `/kek/1` locator、fresh exact前置、PROVISIONING→三方法生成/回读 exact 16 wrapped DEK→同事务 ACTIVE/activation audit、非秘密 receipt/零sidecar及只补漏 receipt 也进入共同 gate |

用例 43/44 的补充硬断言：普通副作用事务取 license-current shared、F-56/checkpoint 取 exclusive、typed reject NONE，验证 shared 并行/exclusive 排空、所有 `try_begin/query/claim` 前置顺序，以及 Outbox transaction-shared claim 与 session-shared+module-shared 外发期间零数据库事务。autotest accept/claim/heartbeat/final aggregate 每笔独立 exclusive，suite 纯读 NONE。checkpoint audit 覆盖完整 envelope、240-second slot 的 0/1/>1 行、same slot/different trusted_now、跨小时 Unix 取整、零 current 首发与 mutation 前 snapshot 冻结，已有 append-only payload UPDATE/终结重算次数必须为零。special acceptance event 与 `MODULE_SIGNER_REVOKED_DISABLED` 对完整 envelope、exact strict before/after、domain-separated projection digest、id/version/time/reason、same-byte replay 零重复逐项重算；recovery terminal batch 必须共享同一治理法人/execute SecurityContext/approval_ref、两事件 reason/reauth 均空，并按 recovery event 在前、accepted event 最后一条的唯一链顺序写入，颠倒、夹入第三事件或任一列漂移都失败。special RELEASED 多历史保留，转 SUPERSEDED/ROLLED_BACK 的 COMMIT 负例必须失败。DRAFT→TEST_PASSED approval 法人全空，submit 首次写 derived id；预填、伪请求头、operator 无治理法人授权、无锁事务改判均拒绝。直接/转义 U+0000、错误分类六层、data-key u16 边界、module contract i32 边界、bootstrap exact typed audit/三法人授权与 STANDARD/INITIAL activation exact payload 一并进共同 gate。CRL 两 issuer 负例固定为“一 issuer serial revoked + 另一 issuer CRL 缺失/非法”先得 UNTRUSTED，修复全部 prerequisite 后才得 REVOKED。

外部电子签章不在本阶段范围内，本阶段不使用 wiremock 打桩。

#### 8.4 端到端测试

桌面端用 Playwright 驱动桌面 WebView，用 tauri-driver 驱动桌面壳；移动端用 XCUITest 与 Espresso。

四端矩阵覆盖按规格第 6.2 章：取值为完整或简化的能力域在该端跑通端到端场景；取值为仅查看或不适用的能力域按豁免清单载明的替代路径验证。
按裁定 A-23，业务闭环类端到端用例随各业务阶段的四端界面交付：下表 E1 由阶段 9b 在其 `testkit/scenarios/golden_loop_14_steps.rs` 之上执行，E2 至 E6 由其所属业务阶段在自己的第 8 节测试计划中执行，其中 E5 跨六个仅查看能力域，由各能力域所属阶段分别执行，本阶段只交付其运行所需的客户端壳、路由注册表与能力闸，并对执行证据逐条汇总；E7 至 E12 属壳层、发布链路与白标制品，由本阶段自行执行，其中 E9 以本阶段的自定义对象单据为被测对象，不依赖任何业务模块界面。四端验收矩阵由阶段 14 汇总。

| 序 | 用例 | 端 | 判据 |
|---|---|---|---|
| E1 | 黄金业务闭环 14 步全程 | Windows、macOS | 整条链路的贯通验收由阶段 9b 的 `testkit/scenarios/golden_loop_14_steps.rs` 承担，本阶段只汇总其在 Windows 与 macOS 两端的走查证据；记录、校验、审计与结果与服务端集成测试一致 |
| E2 | 库存台账与收发扫码完整闭环 | 四端 | 移动端连续扫码 100 次识别率不低于 99%，单次识别不超过 1 秒 |
| E3 | 售后工单与设备台账完整闭环 | 四端 | 同一操作在任一端发起产生的记录与审计相同 |
| E4 | 审批待办与站内通知完整闭环 | 四端 | 站内通知在四端均可查看、跳转与标记，无权时按权限拒绝处理 |
| E5 | 六个仅查看能力域的写入入口缺失 | iOS、Android | 财务过账与期末结账、收付款登记与对账查看、发票申请与开具登记、报表与像素级打印、文档与附件协作、系统管理与低代码配置六类均不出现提交、审批与写入入口；进入写入路径时给出该操作在桌面端完成的说明并可发送到桌面端继续 |
| E6 | 含自定义对象与声明式规则的移动端场景 | iOS、Android | Rust 规则解释结果、字段级权限裁剪、审计结果与恢复连接后的中心重校验四者一致；含受限 WASM 计算的规则只能保存为待中心校验草稿，草稿在中心不产生业务记录 |
| E7 | 配置发布与回退全流程 | Windows | 差异审查、自动测试后以 `CONFIG_RELEASE` 解析默认 `SECURITY_ADMIN` 链；空链/空角色/自审均明确阻断，非申请人按节点顺序通过后包只由 callback 进入 APPROVED，再完成签名、两连接发布与回退。另 REMOVE 一个已有字段，界面/API 隐藏但管理员取证证明物理列、行数与 checksum 在发布、回退及一次中途故障前后均不变；全过程记名记时写审计 |
| E8 | 打印机与 USB Key 端到端 | Windows、macOS | 各一次成功；关闭原生插件加载后能力停用并显式降级，高密级内容改为只读预览并禁止下载，降级事件与范围记入客户端审计 |
| E9 | 断网草稿与恢复提交 | 四端 | 断网期无法完成审批、过账、开票与任何状态流转；草稿保存在本地加密缓存；恢复后由中心重新校验并提交；同一记录冲突以中心版本为准 |
| E10 | 白标制品四端启动 | 四端 | 应用图标、启动页、登录页与关于页显示 `brand.toml` 中的产品名与 Logo |
| E11 | 强制安全更新 | 四端 | `is_forced_security_update` 为真时客户端在更新完成前拒绝进入业务界面 |
| E12 | 读屏软件端到端下单 | 四端 | 各完成一次；WCAG AA 自动检查零严重问题 |
| E13 | 签名扩展安装、启停与升级 | Windows 管理端 | 登记制品后按 `EXTENSION_ENABLE` 走默认 `SECURITY_ADMIN` 链，审批页展示固定制品、manifest 与申请授权摘要；非申请人批准后才能启用。停用时运行数据、审批与 grant 历史均保留，再启用复验同一证据；升级登记新 `(code,version)` 并重新审批，旧版本可停用或撤销但数据不删除 |

#### 8.5 性能与容量

附录 C.2 十二项门槛在附录 C.1 设备基线上复测，每项以旧机型或中端机结果为准，通过线逐项照抄附录 C.2，本计划不重写数值。

本阶段另设三项，均为本阶段新增取值：

- 插件调用在交易路径上的 P95 不超过 50 毫秒。理由是它落在规格第 16 章普通交易提交 3 秒通过线之内，必须给业务逻辑留出余量。
- `client-bootstrap` 的 P95 不超过 2 秒，按规格第 16 章常规交互通过线。
- 一次含 20 个内容项且含 3 条 DDL 语句的发布在附录 A.3 基准数据集上的段一加段二总时长不超过 30 分钟，与规格第 7.4 章迁移执行上限一致。

附录 A.1 度量清单内涉及自定义对象的查询在基准数据集上不得出现顺序扫描，阶段计划提交对应查询的 `EXPLAIN` 证据。

#### 8.6 覆盖率门槛

- `ep-platform-meta`、`ep-platform-release`、`ep-platform-authz`、`ep-platform-flow`、`ep-adapter-wasm` 与 `ep-adapter-ipc` 属平台内核代码，行覆盖率不低于 85%；`ep-app-reporting` 的本阶段修改遵守新增/修改代码 80% 门槛。
- 客户端 Rust 核心 crate（`ep-client-core`、`ep-client-cache`、`ep-client-keystore`、`ep-client-plughost`、`ep-client-audit`）行覆盖率不低于 85%。这是本阶段在规格之上追加的取值，理由是这五个 crate 承担认证、安全策略、本地缓存加密与审计提交，属规格第 17.2 章“平台内核代码”的同类。
- 其余客户端 crate 与 `/clients/ui` 的 TypeScript 代码行覆盖率不低于 70%。
- 新增与修改代码行覆盖率不低于 80%。
- 工作区整体行覆盖率不低于 80%。
- 工具为 cargo-llvm-cov，CI 以 `--fail-under-lines` 强制；TypeScript 用 vitest 的 c8 覆盖率并同样在 CI 强制。`#[ignore]` 必须带 issue 编号且存活不超过本阶段。

#### 8.7 与规格第 17.2 章与第 17.3 章判据的对应

| 规格判据 | 本阶段对应测试 |
|---|---|
| 第 17.2 章 四端端到端测试 | E1 至 E12，按第 6.2 章矩阵确定各端场景范围；E6 直接对应“另执行一个含自定义对象与声明式规则的移动端场景” |
| 第 17.2 章 数据库适配认证中的自定义对象测试项与在线变更逐操作实测 | 集成 3、4、5、6；在线变更范围内操作的锁持有与执行时长不超过上限；新增可空列与新增索引达到在线能力底线 |
| 第 17.2 章 派生存储越权与删除更正传播测试 | 集成 13、14 |
| 第 17.2 章 模块生命周期测试 | 集成 26 |
| 第 17.2 章 身份与访问控制测试中的六类高风险操作四端口径一致 | E5、E7、E8，配合权限阶段的重新认证用例 |
| 第 17.2 章 数据保护控制与销毁证明测试的按端可执行范围 | E8、E9 |
| 第 17.3 章 权限不能跨法人、字段或密级越权 | 集成 2、8、13、14、21、22；`tests/rls_matrix` 独立测试目标追加自定义对象与自定义查询入口两类 |
| 第 17.3 章 审计链可验证 | 集成 5、17、23、24 产生的审计事件纳入审计链验证工具的抽样范围 |

---

### 9. 退出条件

下列各条全部达成才算本阶段完成，每条均可客观判定。

1. 四端制品各一份存在且可安装，均由同一份 `brand.toml` 产出，白标构建流水线的商店政策合规检查门禁四项在流水线日志中逐项显示通过。
2. 同一输入两次执行 `tools/epbrand` 产出的未签名制品哈希逐端一致。
3. 附录 C.2 十二项门槛在附录 C.1 设备基线上复测完成，通过或已获产品负责人书面批准豁免；全部原始测量数据进入证据包。
4. 本阶段先完成四端基础矩阵 18×4=72 格逐格核对：完整或简化格由所属业务阶段跑通端到端场景，本阶段汇总证据；仅查看或不适用格按替代路径验证。阶段 13c 再追加 ServerAdmin 18 格并对 90 格终态、hash 和 `Mcp` 非矩阵例外逐项验收；不得把本阶段切片的 72 格当作产品终态。
5. `platform_meta.client_capability_values` 的内容哈希与二进制内置冻结快照一致；人为篡改一行后进程照常启动，运行期判定以内置快照为准，对该表的写入被拒绝并告警，`--check` 模式在同一状态下非零退出；`client-capability-matrix-frozen` 按裁定 C-25 整项撤销，不作为启动自检项，也不为其定任何 `DegradationKind` 取值。
6. 本阶段纳入的 16 个普通可逆 kind 各有一次 apply 与一次 revert：`ep-platform-meta` 七个（五个 CUSTOM、UI_LAYOUT、RULE）、Stage 3b 两个（FLOW_DEFINITION、NOTIFY_RULE）、Stage 4 三个 AUTHZ、Stage 11 四个 reporting；两种 MCP kind 由 Stage 13c 按 F-55 验收。F-56 的 LICENSE_GRANT/MODULE_PACKAGE 各完成一次完整 RELEASE，但创建通用 ROLLBACK 必须稳定拒绝，不能为凑同形验收调用其 `revert`。
7. 一次含 3 条 DDL 语句的在线发布在基准数据集上完成，单条语句锁持有不超过 5 秒，计划总执行时长不超过 30 分钟，`ddl_plan_steps` 中逐条有实测的锁等待与执行时长。
8. 人为制造锁超时后计划安全收敛并转停机窗口，审计事件含回退原因、操作对象与耗时；不出现 DROP COLUMN/TABLE，既存业务 row_count/checksum 不变，已经自动提交的新增表/列允许保留但必须为 DDL_FAILED/RETIRED 且入口隔离。
9. 回退与普通 REMOVE 演练完成：按新增字段录入的业务数据在回退后仍可读出，字段元数据为 RETIRED，界面与 API 不再暴露该字段；对象和字段 REMOVE 同样只退休元数据，发布/回退/中途失败前后物理表列、行数与业务列 checksum 不变。任何物理删除只允许阶段 14 双人审批 `DisposalPort` 独立流程。
10. `tests/rls_matrix` 追加自定义对象与自定义查询入口两类后全部通过，八类越权面全覆盖。
11. plugin-host 的数据库连接数在全程为 0，宿主导入函数表只有四个函数，尝试网络与文件访问的插件在编译期即失败；`ep-plugin` 的账户×operation 矩阵逐格验证，只有 core/worker 的 `wasm.execute.v1` 与 ops 的健康/指标成功，其余组合、远程客户端及伪造 server 账户全部失败。
12. 插件连续失败自动停用生效，停用事件写入审计并经站内通知送达。
13. 桌面端打印机与 USB Key 各完成一次端到端验证；关闭原生插件加载后能力停用并显式降级，降级事件与范围记入客户端审计并按客户与设备登记；桌面端原生插件的子进程不共享客户端进程内存、不接收本地缓存数据库密钥、不接收会话令牌三条各有一次断言，传入子进程的报文只含按 `READ_OBJECT_FIELDS` 授予裁剪后的字段。
14. 服务端能力闸对移动端六个仅查看能力域的写入端点一律返回 403 与 `PLATFORM.CLIENT_CAPABILITY.WRITE_NOT_AVAILABLE_ON_CLIENT` 并带 `X-Desktop-Handoff-Token`，桌面接续令牌可拉起同一单据同一草稿；移动端界面上这六个能力域无提交、审批与写入入口一条按裁定 A-23 由各业务阶段随其界面验收，本阶段汇总其执行证据。
15. 含受限 WASM 计算的规则在移动端只能保存为待中心校验草稿，恢复连接后由中心重新校验并写入审计，该场景在 iOS 与 Android 各执行一次。
16. 覆盖率门槛按第 8.6 节逐项达成，CI 强制生效。
17. 本阶段新增的 31 条具名错误码、本阶段的具名事件类型、19 张表、唯一指标目录所列指标、唯一配置登记表所列配置项在 `docs/error-codes.md`、`docs/event-catalog.md`、`docs/data-dictionary.md`、`docs/metrics-catalog.md`、`docs/config-reference.md` 与代码常量表中登记齐备，CI 一致性校验通过；未具名的旧配额不构成实现范围。本阶段引用但由阶段 1 按裁定 C-24 登记的 `PLATFORM.CONCURRENCY.STALE_VERSION`、`PLATFORM.AUTHZ.NOT_FOUND_OR_DENIED` 与 `PLATFORM.DB.MIGRATION_WINDOW_CLOSED` 不计入本阶段条数。
18. 依赖方向自检脚本通过：客户端 crate 不依赖任何 `ep-app-*` 与 `ep-adapter-db*`；`ep-platform-meta` 与 `ep-platform-release` 不出现 sqlx、reqwest、`std::fs`、`std::net` 与 `SystemTime::now` 符号。`ep-platform-release` 的工作区内直接依赖在本阶段结束时恰为 ep-foundation、ep-platform-audit、ep-platform-outbox 三项，不含任何 suite 属主；ep-platform-meta、ep-platform-authz、ep-platform-flow 与 ep-app-reporting 对 `ep-platform-release` 的自动测试依赖均为单向。该断言按 F-05 通则甲-2 只约束本阶段结束时的快照，不封禁后续阶段在基线第 1.3 节允许项内增边。`xtask archcheck` 的 `platform-acyclic` 与 `platform-no-adapter` 两条规则全绿。
19. 一次普通配置发布与无损回退证据包、一次 LICENSE_GRANT RELEASE 和一次 MODULE_PACKAGE RELEASE 证据包均归档；三者都含差异、9-suite 报告、CONFIG_RELEASE 链/内容绑定、非自审任务、外层签名、执行与审计，特殊两包另含各自精确 after_spec 形状、内层 CMS、deployment/contract 复验和 NON_ROLLBACKABLE 负证据。普通包还必须归档同一 immutable KeyRef 的 identity-before/sign/verify/identity-after 与 canonical SPKI token 证据。零 current 首张 grant、五种 Restricted 原因下两条恢复闭集、治理法人冻结、首装 evidence/PROVISIONING→ACTIVE、ServerAdmin 只读审批/WinMac 结论与同路由 4 MiB 上传负矩阵都有证据；九套精确注册、合法 SKIPPED、语义失败不重试、基础设施八步重试耗尽与租约崩溃恢复均有对应证据。
20. 本阶段的偏离项与新增决定（第 12 节）已回写共享技术基线，基线更新经平台架构负责人确认。
21. `RG-LICENSE-MODULE-LIFECYCLE-GREEN` 的本阶段部分全绿：集成 23、26、43、44 覆盖 F-56 零/首张 current、同快照 evaluation、TrustedClock/readiness/special/checkpoint 同锁 CAS、全部 RELEASED special exact-set 轮换与分流、SPKI/digest/CRL/RETIRED、实时 usage 零持久化/遥测、scope/None、十 effect/pre-auth/ConfigRelease/MCP/InFlight、五 reason 两恢复链、治理法人、首装固定 evidence/PROVISIONING→ACTIVE、raw-vs-effective module/feature、15 descriptor/product目录 safe-handle/DAG、history identity、模块 shared/exclusive 锁与五动作 signer、revoked inner/source-outer DISABLE、数据保留、两种 after_spec/不可回退、special outer/inner/普通 KMS identity resolver 三路径、ZIP/archive/item/manifest/CMS/multipart 闭合，以及 090100/090200/两个090500/093300 的列/固定seed/CHECK/唯一键/FK/五表deferred图和 Rust/DB18→20。Stage 14 共同 gate 必须收到具名 `license_admission_registry_exact_set` 与 `license_admission_negative_matrix`，并附两个 wiring 的规范化 inventory/digest、exact-equal 正例与全部负例；F-55 AI/MCP 只从同一 signed grant 取 purchased/currently_licensed，最终签名证据或任一上述证据缺失时不得宣称全局绿。
22. 本阶段全部外部 `/api/v1/` 路由，即 `/api/v1/platform/` 与 `/api/v1/ext/` 两段，均在实际注册行声明唯一 `(CapabilityDomain,ActionClass,LicenseAdmissionBindingV1)` 三元组：第 5.3 节配置包与发布单由 `crates/platform/release/src/capability.rs` 提供前两值，其余各段与 `/api/v1/ext/` 的展开路由由 `crates/platform/meta/src/capability.rs` 提供前两值，binding 只取阶段 3 admission owner，能力域一律为 `CapabilityDomain::PlatformAdminLowcodeOps`。本阶段实际 route/job/event/approval-owner/outbound-IPC 集合与两个 wiring 的 binding registry exact equal，`license-admission-registry-consistent`、`xtask configdoc` 与 `xtask archcheck` 的正反测试通过；自定义单据对象的 `doc_type_code` 与 `docs/data-dictionary.md` 单据类型码一节全量表无重复，`xtask configdoc --check-doc-type-codes` 通过。
23. 含 DDL 段的发布在未打开迁移窗口时被经装配注入的 `ep_foundation::port::db::MigrationWindowGuard` 实例的 `assert_open` 拒绝并返回 409 与 `PLATFORM.DB.MIGRATION_WINDOW_CLOSED`，留有一次拒绝与一次放行的执行记录；`ep_platform_flow::port::RuleEvaluator` 与 `WasmComputePort` 的实现类型 `AstRuleEvaluator` 与 `PluginHostWasmCompute` 已在 `apps/core-server/src/wiring/` 与 `apps/job-worker/src/wiring/` 两个目录注册，plugin-host 侧注册 `WasmtimeComponentCompute`，规则求值端点只经 `AstRuleEvaluator`。
24. 本阶段向 T0 贡献的桌面壳最小切片按第 1.5 节的五项逐项交付，并在 T0 演练中完成一次判据走查：T0 的那一条合同在 Windows 桌面端建单，并在同一端看到 T0 的那张收入报表；该切片之外的本阶段交付物不参与 T0，也不因 T0 提前交付。
25. 本阶段新建的 17 张不带法人列的表在 `platform_core.unpoliced_table_registry` 中各有一行登记，`admission_basis` 均取 `SAME_FOR_ALL_ENTITIES`，且本阶段全部迁移执行完毕后 `db/checks` 第十三项返回零行；`config_packages`、`config_package_items` 与 `config_release_orders` 三张的登记行由阶段 3b 随其建表迁移承担，不在本条判定范围内。
26. `CONFIG_RELEASE` 提交严格复用阶段 4 共用解析器并默认 `SECURITY_ADMIN`：缺链、并列活动链、零/空节点与自审四类均稳定 fail-closed；approve/reject 路由只完成任务，包状态只由验证同一包、approval_ref、场景、链版本/digest、当前内容版本/hash、节点顺序和非自审的 callback 迁移。含 DDL 发布每次 attempt 精确使用协调连接加自动提交 DDL 连接两条，原协调连接自己完成段二并提交释放；六个故障点恢复测试全绿且无第三连接。

---

### 10. 与规格和 PRD 的对应

#### 10.1 规格条目

| 规格章节 | 本阶段实现的条目 |
|---|---|
| 3.1 私有化与白标 | 同一核心代码通过品牌配置、签名模块、低代码包与插件产生差异；独立产品名称、Logo、应用标识、证书与商店版本；白标构建、签名、灰度与发布流水线化；厂商托管签名私钥在硬件密码机内双人控制并单独审计 |
| 3.6 白标分发路径与签名身份归属 | 应用商店分发与企业签名/MDM 私有分发两条路径；商店政策合规检查门禁四项；被拒与超期的替代形态切换记录 |
| 5.1 平台内核 | 元数据、自定义对象、字段、关系、索引和视图；低代码表单与规则；配置发布；API、插件和连接器能力注册中的插件登记部分 |
| 6.1 技术路线 | Tauri 2 加 React/TypeScript；Rust 承担认证、安全策略、本地缓存数据库、同步、领域校验与设备能力；TypeScript 只承担界面；客户端全部网络 I/O 经 Rust 核心统一出口，TLS 通道与证书链校验由 Rust 核心执行 |
| 6.2 一致性与兼容 | 四端业务能力等价的运行期判定与豁免清单全部条目；18 行取值矩阵的机器可读落地与冻结校验；外设范围；后台常驻与长时任务；动态扩展代码；自定义对象与低代码规则的移动端豁免；本地缓存与文件；深度编辑；表格；财务与结账；收付款与对账；系统管理与配置发布；文档与 PDF 协作；分析与报表创作 |
| 6.3 本地缓存与设备 | 设备独立硬件保护密钥；本地缓存与凭据加密并绑定 TPM、Secure Enclave 或 Keystore；缓存只保存权限内子集；按记录版本与变更时间增量拉取；冲突以中心为准；断网只保存本地草稿；设备登记、远程注销与安全清除；缓存超期清除 |
| 6.4 文档、表格与设备 | 原生设备通过签名插件适配打印机与 USB Key/智能卡；移动端只使用随包静态签入的相机扫码；条码的企业自定义编码规则生成与识别 |
| 7.4 可定制数据库 | 自定义对象、字段、关系、校验、索引和视图；真实表加在线 DDL；不使用 EAV；公共能力基线的类型与索引限定；在线变更边界与运行期超限自动回退；对象级与字段级密级建模时赋值；随会话下发对象结构、字段密级、权限策略与规则版本且下发范围可审计；默认配额；发布前五项影响分析；声明式包进入 Git 支持差异审查与回退 |
| 7.9 派生存储安全继承 | 权限模型、密级规则或分区规则变更时受影响派生存储在变更生效前完成重建或重新打标；重建期间该分区停止对外服务；重建后重放与条数一致性校验 |
| 9.1 低代码能力 | 自定义对象、表单布局、列表列、首页、菜单与看板；声明式规则表达式与版本化规则；复杂计算调用受限 WASM 函数；不允许任意 JavaScript、Python、Shell 或本机程序进入核心环境 |
| 9.2 配置发布 | 开发测试与生产隔离；配置进入 Git 经差异审查、自动测试、审批与签名；验证失败阻止生产发布 |
| 9.3 模块与插件 | 签名 WASM Component；SDK 以 Rust 为主；插件默认无网络、文件、密钥与业务数据权限；必须声明能力、对象、字段与资源限额；权限经审批后最小授予；不能直连核心数据库也不能读取明文机密；扩展运行时三种形态；桌面端原生插件的九项安全边界 |
| 12.4 DLP 与隐私 | 桌面端与移动端的强制控制；关闭原生插件或设备不合规时的降级为门户端口径与只读预览禁止下载；降级事件与范围记入审计 |
| 13.1 正式拓扑 | plugin-host 与核心同机，按第 9.3 章承载服务端签名 WASM 组件；具名 Job Object 资源单位固定为 app-plugin，由服务宿主层在 `ServiceMain` 早期自我指派。首版运行期只启用内存硬上限并落 `JOB_OBJECT_LIMIT_JOB_MEMORY`；内存保底、CPU 比例/突发上限与按权重磁盘 IO 全部固定不启用，静态限额文件禁止出现这些字段，出现即配置校验失败，绝不自动翻牌。CPU 与磁盘数据只用于硬件标定和认证报告；未来若启用，必须另立正式版本裁定、配置 schema 与 Windows 实机证据，不能由文件出现值触发。插件过载仍只由第 4.8 节的燃料、内存、实例数与执行时限承担 |
| 15.3 运维中心 | 本阶段 9 个指标进入运维中心；插件调用被限流与被资源上限中止的事件记入运维中心，原列的配额触发限流一项保留删除，但理由不再是总览第 6.3 节 R10：己-1 的裁定所恢复的四类取值按裁定 F-08 第 4.1 节在本平台只剩内存硬上限一列有运行期承载——内存保底与磁盘 IO 份额两列删除、CPU 份额一列暂降为意图声明不落运行期取值、突发上限一列无承载——而内存硬上限触限的表现是分配失败返回错误、不是节流，同样不产生「配额触发限流」这一被测量，故该判据的资源单位侧在首版取值集合下不存在；结论不变，理由由「权重列只在争用时按比例分配」换成本平台四类取值的实际存活情况。另按该裁定做不到三，「保底份额被击穿」一类事件的三个被测量（节流计数、内存 low 事件、IO 排队时延）在本平台全无对应物，该类事件整体删除，本阶段不得换一组看似对应的 Windows 计数器凑数 |
| 17.2 自动化测试 | 四端端到端测试；数据库适配认证的自定义对象测试项与在线变更实测；派生存储越权与传播测试；模块生命周期测试 |
| 17.3 强制不变量 | 权限不能跨法人、字段或密级越权；审计链可验证 |
| 18 升级、版本与生命周期 | 客户端支持分批发布与强制安全更新 |
| 附录 C 四端客户端 PoC 量化门槛 | C.2 十二项门槛在 C.1 设备基线上的首版验收复测 |

#### 10.2 PRD 条目

| PRD 节 | 本阶段实现的功能 |
|---|---|
| 8.4.1 报表设计器 | 报表定义作为配置包内容项进入发布通道；本阶段不实现设计器本体与取数 |
| 8.4.2 企业自定义指标 | 指标表达式复用本阶段的声明式表达式 AST 与解释器 |
| 8.4.5 报表类配置对象的状态与流转 | 四类报表配置对象通过本阶段的 `ConfigItemApplier` 进入统一发布与回退闭环 |
| 10.2.2 配置对象与配置操作 | 权限配置的新版本进入本阶段的发布流程后生效；发布前执行法人越权测试集 |
| 10.2.4 权限拒绝的用户可见行为 | 能力闸的拒绝按规格第 15.1 章分类，不泄露存在性 |
| 10.3 六类高风险操作 | 移动端取值为仅查看的高风险操作不提供提交入口并显式说明在桌面端完成；桌面端 USB Key 与智能卡的重新认证由本阶段的原生插件承载取证接口 |
| 10.4.1 配置发布的通用生命周期 | 11 个状态与其流转、触发条件与触发人逐行落地；发布前五项影响分析；发布与回退全过程记名记时写入审计 |
| 10.4.2 数据定制 | 能改与不能改逐条；在线与停机边界；建模必填的对象级与字段级密级；发布前影响分析加自动测试；运行期超限自动回退并转停机窗口且记入审计；回退按声明式包差异审查后执行 |
| 10.4.3 界面定制 | 表单布局、列表列与列顺序、首页、菜单、看板；不得以隐藏字段替代字段级权限；不得改变四端取值矩阵；不得引入任意 JavaScript；按角色预览核对无权字段确实不返回；按上一已发布版本整体回退 |
| 10.4.4 流程定制 | 流程定义作为配置包内容项进入同一发布通道与版本回退；本阶段不实现流程引擎本体 |
| 10.4.5 权限定制 | 权限内容项进入同一发布通道；变更生效前完成受影响派生存储的重建或重新打标；发布前执行法人越权测试集且新增自定义对象与新增查询入口必须通过 |
| 10.4.6 报表定制 | 报表类内容项进入同一发布通道与整体回退，且不影响已导出的历史文件 |
| 10.4.7 五类定制的共同硬边界 | 不得跨模块直接读写业务表；不得取得事务业务库直连；系统管理与低代码配置在移动端只提供查看与告警处理 |
| 10.7.1 白标的可见范围 | 品牌配置项清单在 `brand_profiles` 中冻结为 U-K-07 经 F-51 批准的首版值 |
| 10.7.2 分发路径的可见差异 | 两条分发路径与切换记录 |
| 10.7.3 四端的用户可见差异 | 仅查看能力不出现写入入口并给出桌面端说明；同一操作在任一端结果相同；移动端不接受动态扩展代码；WASM 规则只能保存为待中心校验草稿；不承诺后台常驻；关闭原生插件后的降级；本地缓存与单文件上限可下调且可审计 |
| 10.7.4 数据保护控制按端的可见差异 | 三档控制强度与两类降级路径 |
| 11.5 离线、弱网与断网行为 | 断网只保存本地草稿；冲突以中心为准；缓存加密并绑定设备密钥；退出登录、设备注销或超期时清除 |
| 11.6 浏览器、客户端与设备要求 | 四端客户端形态；六类仅查看能力域；设备登记与远程注销；外设范围；WCAG AA 与主题与快捷键与命令面板与连续扫码；不做平板版式适配 |
| 附录乙 U-K-01 至 U-K-08、U-L-06 | F-51 已批准本阶段推荐值并冻结为首版规范，见第 11.3 节 |

---

### 11. 风险与预留

#### 11.1 技术风险

| 风险 | 影响 | 控制 | 触发后的处置 |
|---|---|---|---|
| Tauri 2 移动端成熟度不足，13a 薄 PoC 的可判阈值未通过 | 若发现过晚会扩大 UI 返工面 | 13a 在业务移动界面大规模投入前执行薄 PoC；UI 与 Rust 核心只经一层桥，Rust 核心九个 crate 的接口先冻结；薄 PoC 不得据部分测量给出“保留 Tauri”的肯定结论 | 只把移动 UI 切为 Flutter，并把 IPC 桥改为 FFI 桥及重做移动生命周期、推送、深链与平台插件适配；客户端 Rust 核心九个 crate、服务端 Rust 核心、协议、数据模型及已经通过的核心测试不动 |
| `create index concurrently` 在基准数据集上超过 30 分钟 | 新增索引失去在线变更能力，触及规格第 7.4 章的在线能力底线 | 影响分析的性能项在计划阶段外推并给出预警；单次计划语句数上限 200 | 该操作登记入停机窗口操作清单；若新增索引整体无法达到底线，按规格第 7.4 章交付说明必须明确降级为停机窗口变更，不得以在线 DDL 能力通过认证 |
| DDL 与元数据无法同事务导致的中间态 | 出现 ACTIVE 元数据而物理表缺失，或物理表存在而未开启行级安全 | 两阶段写入加启动自检项 `custom-object-ddl-consistent`；集成测试 6 专测该场景 | 进程照常启动，把相关对象置 DDL_FAILED 并隔离其全部入口，开一个 kind 取 `CUSTOM_OBJECT_DDL_INCONSISTENT` 的降级窗口并给出具体对象清单，由修复用例补建策略后关窗 |
| 插件执行占用同机资源影响交易时延 | 规格第 16 章 3 秒通过线受损 | plugin-host 独立资源单位 app-plugin 按基线第 2 节承载；首版只有内存硬上限落运行期取值，CPU 比例/突发、内存保底与按权重磁盘 IO 固定不启用，未来启用须另立版本裁定而非等待实测自动翻牌。单次调用按第 4.8 节设燃料、内存、实例数与 2000 毫秒交易路径时限，调用在事务外 | 并发实例数达到 `EP__PLUGIN__MAX_INSTANCES` 时限流其调用并记入运维中心；按第 4.8 节以 outcome `THROTTLED` 落一行 `platform_meta.extension_invocations`，向调用方返回 `PLATFORM.EXTENSION.HOST_UNAVAILABLE`（category `INFRASTRUCTURE`、HTTP 429、retryable 为真），事件记入运维中心并计入附录 A.2 的错误率口径；本阶段不为限流开降级窗口，也不新增任何 `DegradationKind` 取值，见裁定 F-06 |
| WASM 宿主自身的漏洞成为越权入口 | 对应规格第 21.7 章风险 | 宿主导入函数只有四个且无网络、文件、密钥与数据库；能力清单与最小权限授予；输入按字段权限裁剪后才进入 IPC；plugin-host 数据库连接数为 0 | 按规格第 3.3 章在本实例内停用该扩展，停用决定、影响范围与恢复条件记入审计 |
| 桌面端原生插件的子进程成为越权入口 | 对应规格第 21.7 章风险 | 子进程不共享客户端进程内存、不接收本地缓存密钥与会话令牌；传入报文按字段权限裁剪；签名主体、版本与哈希三项核对不通过即不加载 | 按规格第 3.3 章在本实例内停用该插件，停用决定、影响范围与恢复条件记入审计 |
| 白标维护矩阵膨胀 | 对应规格第 21.8 章风险 | 单一核心加配置化品牌；客户不维护长期核心代码分支；构建、签名、灰度全流水线化；可复现构建使制品哈希可核对 | 品牌配置项清单冻结在 `brand_profiles` 的列集内，新增可配置项必须先改该表并回写 U-K-07 决策 |
| 配置回退删掉已录入业务数据 | U-K-02 已由 F-51 批准为首版冻结值，不是开发待决 | 回退只停用元数据、不执行 DROP；最近 10 个已发布包且不早于 180 天的回退范围按第 4.6 节执行 | 首版不得由实现方改选。未来只有新的正式变更裁定才可调整；若裁定改为物理删除，须另建停机窗口计划与双人审批，并按裁定 A-22 经 `DisposalPort` 交由阶段 14 的 `OpsDisposalService` 按规格第 12.4 章处置清单承担 |
| 生产环境内的就地创作与规格第 9.2 章开发测试生产隔离的张力 | 审计与合规口径受质疑 | 生产内的 DRAFT 状态配置对运行期一律不可见，运行期只读取 ACTIVE 版本；差异审查以 Git 中的声明式包为准；就地式包在签名后内容不可再改 | 若客户或审计要求更严，收窄为只接受 IMPORTED 来源的包，把 `source` 的可选取值在配置中限定为 IMPORTED |

#### 11.2 为后续阶段预留的扩展点

- `ConfigItemApplier` 端口由阶段 3a 按裁定 A-19 交付，但首版 `item_kind` 是 F-56 冻结的终态 20 项闭集，不是运行时扩展点；实现方不得新增 kind、注入第 21 个 applier 或只改单侧枚举/CHECK。未来若更高版本正式裁定新增内容项，必须在同一发布批原子更新 Rust `ItemKind::ALL`、数据库 CHECK、具名迁移、migration catalog、全部 wiring 的 exact-set 断言、测试与文档，再复用既有发布链；不得修改历史迁移、用 unknown kind 旁路或把“实现 trait”当作授权。
- `CapabilityValue` 枚举与 `client_capability_values` 表结构支持新增能力域行与新增端列。恢复客户门户或经销商门户配套应用时，只需新增能力域行与新的 `client` 取值，不改判定算法。
- `extension_capability_grants.capability` 的取值集合封闭在 4 项。恢复服务端隔离容器形态或新增外设适配时，新增取值并同步扩展宿主导入函数表；宿主导入函数表的断言测试是新增能力必须同步修改的强制点。
- 客户端本地缓存的记录标签已按规格第 7.9 章口径携带，为后续恢复离线数据租约、租约到期锁定与撤销序列、离线草稿字段级合并预留了判定依据。
- `ep-client-plughost` 的能力清单与子进程 IPC 已与服务端 WASM 宿主共用同一份能力枚举与帧格式，为后续把桌面端插件形态统一到 WASM Component 预留了收敛路径。
- 白标构建流水线的可复现构建能力直接支撑规格第 3.2 章私有构建级源码许可的支持判据，后续开放该许可级别时不需要新建流水线。
- `ddl_plans` 的五项影响分析列为 jsonb，后续增加影响维度不需要迁移表结构。

#### 11.3 F-51 已批准的首版冻结值与未来变更代价

下列技术侧推荐值已由 F-51 的“全部采用推荐项”批准为首版规范，不再待决，也不允许实现方在开发中改选。未来若经新的正式裁定变更，切换代价如下。

| 编号 | 首版冻结值 | 未来正式变更代价 |
|---|---|---|
| U-K-01 | 配置包按整包发布；并发编辑按内容项粒度加编辑锁，TTL 1800 秒 | 改为按单对象发布需要拆分 `config_release_orders` 与互斥锁的粒度，影响第 6.2 节的串行化设计，工作量中等 |
| U-K-02 | 保留最近 10 个已发布包且不早于 180 天；数据定制回退只停用元数据不删数据 | 改为物理删除需要新增停机窗口计划与双人审批路径，并按裁定 A-22 经 `DisposalPort` 交由阶段 14 的 `OpsDisposalService` 接入规格第 12.4 章处置清单，工作量中等 |
| U-K-03 | 对象数 200、单对象字段数 100、单对象索引数 5 | 只改配置默认值，工作量小；但需重跑第 8.5 节的容量项 |
| U-K-05 | 移动推送正文不携带任何业务字段，只含事项类型与关联编号 | 若允许携带需新增按密级的正文裁剪，工作量小 |
| U-K-07 | 白标可配置项固定为 `brand_profiles` 的列集，含主题色、登录页背景与通知模板集 | 新增可配置项需要迁移该表并同步 `tools/epbrand`，工作量小 |
| U-K-08 | 仅查看能力的提示文案为该操作在桌面端完成的一句话说明；提供桌面接续令牌与深链跳转 | 只改文案与是否启用跳转，工作量小 |
| U-L-06 | 本地缓存有效期桌面端 14 天、移动端 7 天；草稿超期提示确认后清除 | 只改配置默认值，工作量小 |
| U-A-05 | 分页与排序与筛选期间按基线第 11.5 节，本阶段不另设 | 无 |

---

### 12. 本阶段对共享技术基线的新增决定与偏离项

按基线第 0 节，下列各项在基线中未覆盖或需要追加，本阶段显式登记，阶段结束时回写基线。

1. 客户端代码位置与 crate 命名。基线第 1.1 节只覆盖服务端 workspace。本阶段新增 `/clients/` 为独立 Cargo workspace，crate 前缀 `ep-client-`，通过路径依赖复用 `ep-foundation`、`ep-contract-*` 与 `ep-platform-meta`，禁止依赖 `ep-app-*` 与 `ep-adapter-db*`。edition 2021，禁止 nightly，与基线一致。桌面壳与移动壳分别位于 `/clients/desktop` 与 `/clients/mobile`，其 `src/modules/<module>/` 为业务模块界面目录，按裁定 A-23 由阶段 5 至阶段 12 各自交付，本阶段只建立目录约定与路由注册表。
2. 非常驻工具目录。基线第 2 节的九个产品进程是常驻进程清单。本阶段新增 `/tools/` 目录承载 `epcfg`、`epbrand`、`epplug` 三个一次性命令行工具，不属于进程清单，不占用系统账户与资源单位。
3. 客户端本地加密缓存库选型。取 SQLCipher，经 rusqlite 的 bundled-sqlcipher 特性引入。理由是附录 C.2 要求本地加密数据库随机读写吞吐不低于 20 MB/s、10 万行查询 P95 不超过 1 秒，纯 Rust 的嵌入式库在加密路径上尚无同等实测证据。该选型只作用于客户端，不触及基线第 3 节的服务端数据库约定。
4. 服务端 WASM 宿主选型。取 wasmtime 与 wasmtime-wasi，主版本在 workspace 根 `[workspace.dependencies]` 中锁定为 26 系列，只启用 Component Model，不启用任何 WASI 网络与文件能力。
5. 部署级配置表的登记。本阶段涉及的 20 张部署级表不带 `legal_entity_id` 与 `data_scope_tags`，不建行级策略，其余公共列齐备，按基线第 3.8 节的正向登记制逐表在 `platform_core.unpoliced_table_registry` 登记一行，`admission_basis` 一律取 `SAME_FOR_ALL_ENTITIES`。其中 17 张由本阶段建立，其登记行由第 3.4 节第 14 号迁移 `V20261022091300__platform_core_backfill_stage13_unpoliced_table_registry.sql` 一次写入；`config_packages`、`config_package_items` 与 `config_release_orders` 三张由阶段 3b 按裁定 A-27 建立，其登记行按同一正向登记制随阶段 3b 的建表迁移插入，本阶段不重复登记。理由见第 3.1 节。
6. 唯一约束中的确定性生成键。`custom_fields.owner_key` 由互斥的对象所有者生成，形态固定为 `custom:<uuid>` 或 `core:<type>`，不存在空值；`ui_layouts.role_key` 仅在 `role_id` 为空时由数据库生成 `'-'`。两列均为 `GENERATED ALWAYS ... STORED`，客户端和仓储不可写，普通唯一索引因而既无 NULL 歧义也不可被伪造。测试必须覆盖直接 SQL 企图写生成列被拒、同一 owner/code 重复被拒、`custom:` 与 `core:` 跨命名空间不碰撞。
7. 编辑锁的物理删除。基线第 3.6 节允许物理删除的表只有两类，本阶段追加第三类：`platform_meta.config_edit_locks` 的过期行由 job-worker 按 `expires_at` 清理。理由是它是短生命周期协作锁，不承载任何业务事实。
8. 平台侧 API 路径段取 `platform`，自定义对象数据端点路径段取 `ext`。两者都不新增模块码，`ext` 与 schema 名一致。
9. 自定义对象的领域事件统一为 `platform.custom_record.created.v1`、`platform.custom_record.updated.v1`、`platform.custom_record.state_changed.v1` 三个类型，具体对象由信封的 `aggregate_type` 承载为 `ext.<object_code>`。理由是不新增模块码。
10. 幂等作用域中法人维度对部署级端点的取值。取请求头 `X-Legal-Entity-Id` 的值，理由见第 6.3 节。
11. 启动自检项按裁定 C-25 改为按注册名标识，不再以序号称呼。本阶段按该裁定的注册顺序追加 `custom-object-ddl-consistent` 一项，按裁定 C-25 登记为 Degrading 级，判读结果不阻断进程启动，只按第 7 节写明的运行期后果处置并开一个 kind 取 `CUSTOM_OBJECT_DDL_INCONSISTENT` 的降级窗口，`--check` 对 DEGRADED 仍非零退出；该项一并覆盖基线项 `rls-enabled-and-forced` 对 `ext` schema 的扩展；`client-capability-matrix-frozen` 按裁定 C-25 整项撤销，不进注册表也不为其定任何 `DegradationKind` 取值；扩展制品校验与品牌附件校验不再作为启动自检项，改由第 4.8 节加载路径与品牌激活用例承担。自检项的 severity 取值域为 Blocking 与 Degrading 两值，需回写基线第 7.3 节。
12. 覆盖率门槛追加客户端 Rust 核心五个 crate 不低于 85%，TypeScript 界面包不低于 70%。
13. 关账受理期间暂停新发布单执行，取值见第 6.5 节。
14. 客户端本地缓存记录携带来源对象标识、版本、法人标识、密级与数据范围标签，与规格第 7.9 章派生存储同一口径。理由见第 4.10 节。

本阶段不偏离基线第 3.5 节的金额与数量精度、第 3.8 节的行级策略模板、第 5 章的封套与分页与幂等、第 6 章的事件与 Outbox、第 10.3 节的事务边界、第 10.4 节的分层边界。
