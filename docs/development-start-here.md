# F-57 开发交接导航

> 状态：`CURRENT_SUMMARY_NON_NORMATIVE`；核查日期：2026-09-07（Australia/Melbourne）。
> 本文帮助开发者找到现行入口，不定义接口、验收或新任务，也不构成第六份实施计划。发生冲突时按[总体设计 §1.1](superpowers/specs/2026-08-23-f57-governed-automation-fabric-design.md)的权威顺序处理。

## 可以立即开始什么

开发授权已于 2026-08-27 获得，当前可以从 **G0-01** 开始实现。`READY_NOT_AUTHORIZED` 是收敛主计划 §7 保留到首张 G0 绿色回执之前的状态节点名；应连同 `DEVELOPMENT_AUTHORIZED=true`、`implementation_state=NOT_IMPLEMENTED` 阅读。它不要求再次索取已有开发授权，也不表示 G0 已完成。

当前仓库有 Rust/PostgreSQL 骨架与早期平台逻辑。F-57 CapabilityGraph、权威主干、完整客户端和发布认证尚未实现。准备外部执行器、测试签名材料和后续硬件可与 G0-01 并行；它们各自到期前必须就绪，不能用本地静态检查代替。

## 总体结构怎样落到代码

| 边界 | 唯一职责 | 开发时保持的约束 |
|---|---|---|
| CapabilityGraph | 描述能力、所有者、接口与生成投影 | 图是唯一编写源；Rust、OpenAPI、客户端和配置投影不能各维护一份定义 |
| `crates/features/` | 每项业务事实的唯一写入所有者 | 按功能组织，公开命令/授权视图/已提交事实跨能力协作；不跨 owner 直写表 |
| `crates/platform/` | 身份、授权、命令、代际、审计、备份等通用机制 | 机制不侵占业务事实；所有业务写入经 `CommandPipeline` 和授权事务边界 |
| `crates/adapter/` | PostgreSQL、文件、密钥、IPC 等外部能力 | I/O 与路径、权限、重试、失败语义在适配边界闭合，不能从客户端绕过 |
| `apps/` 与 authority-kernel | 进程装配、受控入口与 Windows 承载 | G1–G5 的应用装配暂居 core-server，G6-14 按逐叶迁移表移入 kernel，固定 launcher 只经版本化 ABI 调用 |
| Workbench、门户与外部工具 | 展示、提交意图、读取授权结果 | 无数据库凭据；执行时由服务端重新验证身份、权限、版本和前置条件 |

业务命令把事务内结果、审计/回执与耐久投递需求按契约原子提交；外部副作用在提交后经受控 provider 执行。外部结果为 `UNKNOWN` 时先核对和恢复，不盲目重试。上述边界由[总体设计](superpowers/specs/2026-08-23-f57-governed-automation-fabric-design.md)、[业务执行契约](superpowers/specs/2026-08-23-f57-business-execution-contract.md)与[ADR-0025](adr/ADR-0025-f57-capability-graph-and-feature-first-boundaries.md)定义，不表示现有目录已经完成迁移。

## 唯一执行计划与路线

入口是[收敛实施主计划](superpowers/plans/2026-08-24-f57-converged-program.md)，它与下列四个子计划组成唯一五文件执行计划集。任务依赖严格取主计划 §5 的 42 行 DAG，表格只帮助定位：

| 阶段 | 去哪里实施 | 完成后才能声称什么 |
|---|---|---|
| G0 | [Bootstrap，Task 1–6](superpowers/plans/2026-08-24-f57-g0-bootstrap-implementation.md) | 开发契约、生成投影、L0/L1 和首次 `G0_BOOTSTRAP_GREEN` 已验证 |
| G1 | [权威主干子计划，G1](superpowers/plans/2026-08-24-f57-authority-spine-implementation.md) | `G1_AUTHORITY_SPINE_GREEN` |
| G2 | [权威主干子计划，G2](superpowers/plans/2026-08-24-f57-authority-spine-implementation.md) | CTC 服务端持久化通过，`G2_CTC_DATA_GREEN` |
| G3 | [CTC-01 子计划，G3](superpowers/plans/2026-08-24-f57-ctc01-implementation.md) | 最小客户端壳通过，`G3_CLIENT_SHELL_GREEN` |
| G4 | [CTC-01 子计划，G4](superpowers/plans/2026-08-24-f57-ctc01-implementation.md) | 完整 CTC-01 开发切片通过，`DEV_SLICE_GREEN` |
| G5 | [扩展与发布子计划，Task 1–9](superpowers/plans/2026-08-24-f57-expansion-release-implementation.md) | 全范围及四平台集成通过，`INTEGRATION_GREEN` |
| G6 | [扩展与发布子计划，Task 10–15](superpowers/plans/2026-08-24-f57-expansion-release-implementation.md) | 冻结候选经 L3 才可 `RELEASE_CERTIFIED`；客户启用另需现场准入 |

G0-01…G0-06 串行。后续只有 DAG 明确独立的工作才能并行；F57 迁移的提交仍必须按版本递增，不能留下 `CREATED` 前缀缺口。旧 25-task 计划、早期分层业务计划与 Linux 部署目录均不是另一路执行入口。

## 开始前的本地核对

工具链版本取仓库 [rust-toolchain.toml](../rust-toolchain.toml)，目前为 Rust `1.95.0`，包含 `rustfmt`、`clippy`；依赖版本取 [Cargo.lock](../Cargo.lock)。先在受控准备阶段备齐工具链及离线依赖，再进入离线开发和验证。`.cargo/config.toml` 的 `cargo xtask` 别名自带 `--locked --offline`；普通 `cargo test/check` 仍须显式带这两个参数，或在已配置的离线执行环境运行。

以下命令在仓库根运行。命令存在、某一项通过和 F-57 门禁通过是不同结论：

| 当前可运行的命令 | 预期与用途 |
|---|---|
| `rustc --version`、`cargo --version`、`rustup component list --installed` | 核对本机工具链；2026-09-07 本机 Rust/Cargo 为 1.95.0，已列出 rustfmt/clippy；不证明 Windows 工具链可用 |
| `cargo metadata --locked --offline --format-version 1` | 验证完整依赖可离线解析；缺缓存必须补准备，不能把联网下载算作门禁通过 |
| `cargo fmt --all -- --check` | 格式检查；不构建或验证业务 |
| `bash .github/ci/verify-pipeline-commands.sh` | 2026-09-07 实测退出 `0`；仅核对旧流水线的命令可用性、离线参数和登记表，不执行 19 条门禁 |
| `bash .github/ci/compare-red-baseline.sh --gates-only` | 2026-09-07 实测退出 `3`；六个已测面与基线一致，但跳过 cargo-test，所以总体仍为“未覆盖” |
| `cargo xtask configdoc --check-doc-type-codes` | 当前登记预期退出 `3`，43 条文档码、0 条代码常量、1 项未覆盖；是独立检查，不计入默认 configdoc 的 304 项 |
| `cargo xtask f57 verify --level l0 --changed-from HEAD` | 2026-09-07 实测退出 `2`／未知 `f57`；尚未运行任何 F-57 检查或产生证据 |

[现行已登记基线](../.github/ci/known-red-baseline.tsv)的七个检查面如下。2026-09-07 本次逐面复测一致；完整比较器随后也已重跑 cargo-test，七面精确一致、外层退出 `0`。cargo-test 仍有三条已登记失败，完整记录见[本轮核验](superpowers/reviews/2026-09-07-windows-development-handoff-verification.md)：

| 判定面 | 退出码 | 登记不符数 |
|---|---:|---:|
| archcheck | 0 | 0 |
| sqlcheck | 0 | 0 |
| codecheck | 1 | 2 |
| errorcodes | 1 | 12 |
| configdoc | 1 | 304 |
| eventcatalog | 1 | 117 |
| cargo-test | 1（原始 Cargo 进程为 101） | 3 |

先按涉及的包和测试目标运行检查。完整 `cargo xtask ci` 会调度历史的构建、全工作区测试、签名、复现与端到端步骤，不适合作为 G0-01 的前置“全绿”要求。不得删除负例、降低计数或把登记红折算成 PASS；新增、收窄或状态变化须按 [CI §14.1](ci-pipeline.md#141-已登记基线红与新回归的区分f-68)复测并留证。

## G0-01 的首次自举顺序

完整文件清单、类型签名与测试以 [G0 Task 1](superpowers/plans/2026-08-24-f57-g0-bootstrap-implementation.md#task-1-freeze-the-185-row-delivery-registry-and-f57-cli) 为准；以下不是缩减交付范围：

1. 从已批准完整基线提交创建本任务专用的干净工作树。在编辑前证明 porcelain 状态为空，将完整基线提交和工作树记录保存到仓库外。G0-01 创建任务工具本身，因此此时没有合法的 `task begin` 调用。
2. 阅读 Task 1 的五份 `Read` 输入，按其 `Files` 清单建立新包骨架与失败测试。没有包时的 `package not found` 只是入口缺失；必须先有可构建测试骨架，才算预期的行为测试红灯。
3. 完成 Task 1 的 foundation 共享类型／schema、production-linkable delivery registry、release carrier 契约和 `xtask f57` CLI 壳。保留既有 `crates/foundation/src/principal.rs` 及 `SYSTEM_PRINCIPAL_ID`，按 Modify 扩展，不能覆盖旧内容。
4. 依 Task 1 的具名测试命令由红转绿，再验证 CLI 的真实进程出口：合法但未交付分支为 `NOT_DELIVERED`／`70`，非法语法为 `2`，两者都不产生门禁证据。L0/L1 调度器由 G0-06 才交付；G0-01 注册语法不等于完成 G0。
5. 用刚实现并已验证的自举命令暂存、核对后提交。G0-02 及以后才在编辑前调用 `task begin`，且不能再用 bootstrap 例外。

Task 1 首批落点可按职责定位；其中每一个精确文件仍须服从原计划清单：

| 职责 | 文件入口 |
|---|---|
| 共享类型与单一 schema 根 | `crates/foundation/src/{identifier,delivery,client,evidence}.rs`，扩展既有 `principal.rs`，`docs/evidence/f57-foundation.v1.schema.json` |
| 生产可链接登记表解析 | `crates/platform/delivery-registry/` 及其 registry／migration_closure 测试 |
| 六值 carrier 契约 | `crates/platform/release/src/carrier_contract.rs` 及其测试 |
| CLI 壳、测试与装配 | `xtask/src/f57/{mod,cli,registry,evidence,verify}.rs`，`xtask/tests/f57_registry.rs`、`f57_cli.rs`，修改 `xtask/src/main.rs` |
| 冻结登记与依赖 | Task 1 的七份新 TSV、全部具名 schema/golden，以及列出的 Cargo manifest／lock 文件 |

只有 Task 1 实现并通过具名测试后，才执行其既有提交序列：

```text
cargo xtask f57 task stage --task G0-01 --bootstrap-clean-base <recorded-full-base-commit>
cargo xtask f57 task verify-staged --task G0-01
git commit -m "feat: freeze f57 delivery registry"
```

占位值必须替换为此前记录的完整 40 位小写提交哈希，不能填 `HEAD` 或缩写。新工具必须重新证明干净基线、工作树身份与精确暂存集合；这里不允许绕过校验的原始目录级暂存。

## 外部条件何时必须准备好

| 最晚到期点 | 必须具备的条件 | 缺失时保留的边界 |
|---|---|---|
| G0-01 首次离线编译前 | 已固定工具链、离线 Cargo 依赖、干净工作树与仓库外基线记录 | 可以准备资料；缺依赖不能宣称编译或测试通过 |
| G0-02 原生 fixture corpus 定稿前 | Windows/macOS/iOS/Android 四条隔离的非生产原生格式解析通道、合成测试根和仓库外私钥材料，生成四份公有 DER 根及十六个固定格式语料；根仅用于内存解析，禁止把根或语料安装进 OS/分发信任库 | 图不能用空摘要、任意格式文件或未登记临时根完成；该语料仅验证格式与安全负例，不是 G5 四端产品或原生安装资格 |
| G0-05 实机检查与真实 registry 封存前 | 获批 Windows Server 2022 runner、Windows 签名与 PowerShell 信任环境、部署固定 signing handle、全部生成 evidence-role／TSA 条件及唯一 broker | 单元/fixture 测试可先做；缺真实材料不封存 registry，G0-06 不发签名绿色回执 |
| G0-06 Fresh-PG／L1／G0 发行前 | 可销毁 PostgreSQL 16 数据库；若接入 PR 自动验证，还需一次性无密钥、无制品、无内网权限的隔离自托管 runner，以及仓外保护规则 | 只运行工程演练；旧 D-07 runner 不能接收不可信 PR 或替代 L1 |
| G1-01 真实存储／代际路径验证前 | 已验证的部署与存储信任输入、DATA_HDD、对应 Windows 原生能力与保护配置 | 本地契约测试不构造假的生产 storage proof、generation 或 declaration |
| G5 架构分支与四端集成前 | 四平台原生构建／测试环境及本阶段规定的签名和生命周期证据 | 只按已验证决策选中一个客户端栈分支；不能凭偏好跳过决策 |
| G6 最终认证／客户启用前 | [P340 生产档案](superpowers/specs/2026-08-23-f57-windows-p340-production-profile.md)要求的合格 HDD、UPS、服务器外只追加备份、两块加密离线轮换盘、分域恢复材料、洁净恢复硬件和实际 Windows／四端／72 小时／完整 L3 证据 | 生产状态保持 `PRODUCTION_NOT_READY`；证书不自动打开业务入口，仍须客户现场准入 |

外部条件的精确提供者、信任形状、路径与失败码分别由 G0 Task 2／5／6、G1-01、G5 和 G6 任务持有。本文没有创建新服务命令、默认盘符或临时放行路径。

## 运行与耐久数据的交接边界

目标平台是 Windows Server 2022 原生服务。SSD 承载可重建的系统／程序运行材料；权威业务数据和计划规定的耐久状态必须经已验证的 DATA_HDD 路径写入。具体对象分级、存储配置和容量按 [Windows/P340 档案](superpowers/specs/2026-08-23-f57-windows-p340-production-profile.md)执行，不能把“运行在 SSD”解释为业务数据库、审计或其他耐久写入允许落 SSD。

原单块 1 TB HDD 不能取得生产资格。G6 必须验证替换后的合格硬件及全部保障措施；`deploy/` 的 Linux/systemd/容器入口和 `scripts/dev-up.ps1` 的 Linux 容器包装不提供 Windows 生产安装。`tools/release-gate/src/main.rs` 当前仍是打印 skeleton 并成功退出的历史骨架，不能作为任何发布证明；只有现行计划交付后的 F-57 门、同一最终候选和完整证据链才能产生有效结论。
