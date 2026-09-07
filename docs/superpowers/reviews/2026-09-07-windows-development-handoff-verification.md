# Windows 与整体开发交接核验（2026-09-07）

> 文件分类：`CURRENT_SUMMARY_NON_NORMATIVE`。本记录说明本轮文档修订和实际验证，不定义新的架构权威、执行计划或发布等级。
> 用户明确选择：“完善总体架构、配置和实施计划，达到可立即开发的状态”；完成后上传现有 GitHub 审阅分支。
> 基础提交：`61ff5b6ec30b0013e5bf1233da86110888659012`；分支：`docs/spec-review-revisions`；远端：`origin/docs/spec-review-revisions`。

## 交付与状态

本轮交付以[开发起步指南](../../development-start-here.md)和[Windows/SSD/HDD 交接指南](../../windows-server-storage-handoff.md)为阅读入口；正式实施仍只有 2026-08-24 五文件计划。两份指南都标记为非规范导航，并已在权威登记中登记。

- 现有授权允许从 G0-01 开始；`DEVELOPMENT_AUTHORIZED=true`。
- `development_state=READY_NOT_AUTHORIZED` 是现行状态机保留到 G0 首次绿色回执前的节点名，不表示需要再次取得已有授权。
- `implementation_state=NOT_IMPLEMENTED`、`production_state=PRODUCTION_NOT_READY` 保持不变。
- 所有修改均为 Markdown；没有修改 Rust、SQL、依赖、执行脚本、业务迁移、安装器或生产环境。
- 原有 185 项需求、五文件执行入口、冻结种子和迁移基线不缩减；本轮不制造 L0–L3 回执。

## 发现与修订

| 问题 | 本轮处理 | 实际意义 |
|---|---|---|
| 顶层设计/配置/威胁模型允许 pagefile 与 crash dump 写 HDD，详细计划却要求禁用 | 总体设计 §13.2.1 明确收编八行 Windows 持久政策；同步 profile、配置参考、威胁模型与导航 | 开发者不会对 SSD/HDD 使用两套相反规则 |
| HDD 延迟解锁与启动 pagefile 有依赖冲突 | 六类文件在两卷均禁用；绑定 32/30/24/16 GiB 与 72 小时无页面文件认证 | 内存不足时失败并重认证，不能靠开启页面文件绕过 |
| G1-01 启动摘要先读尚锁定 HDD 上的 manifest | 改为 SSD 信任/九 locator → trusted boot/TPM → 独立 broker 解锁及读回 → HDD manifest/vault → 数据库 | 消除锁定数据卷的自举循环 |
| 每个任务都要求编辑前 task begin，但 G0-01 创建命令本身 | 主计划 §8 明确沿用 G0-01 干净基线/仓外记录/bootstrap-clean-base 特例；G6-15 保持 evidence-only | 开发从可执行的第一步开始，后续仍受精确暂存和快照约束 |
| 文档将尚不存在的 F57 CLI 写成当前返回 70 | 区分当前未知命令/2 与 G0-01 必须实现的 NOT_DELIVERED/70，补真实进程出口和零证据验收 | 不把计划命令或参数错误误认为已完成检查 |
| G6-14 要迁移整个 Authority，但冻结文件清单缺迁移端点 | 补 47 个逐叶 Move，明确 4 个保留文件、构建/依赖/模块/测试迁移与 ABI 边界 | 后续实现不需要越过 G0 已冻结 staging allowlist |
| AMT 可现场启用与首版枚举只有 DISABLED_UNCONFIGURED 冲突 | profile 明确首版必须关闭且未配置，未来启用须新 graph/profile 与独立认证 | 不会靠现场选择扩张首版管理面 |
| 固定测试包被要求与客户真包同身份，且平台原生签名与产品证据签名混用 | 将 4 根/16 包语料限定为格式与非生产负例；真实包输入嵌入既有生命周期证据，分别验证原生签名和产品 CMS wrapper | 不把 fixture 根当系统安装信任，不要求 Apple 平台私钥签产品自定义证据；冻结需求与 signer/投影类别不扩张 |
| 开发入口散落且旧 Linux/TOML 容易被误用 | 增加两份中文导航、总体职责表、故障矩阵、任务 owner 与真实命令边界 | Windows 配置有明确来源，SSD 程序与 HDD 数据职责可交接 |

客户端协调修订进一步明确：当前真包与四个实际源包先固定原生身份、整数版本及合法签名连续性；`native_inputs` 仅嵌入既有 PACKAGE_SIGNATURE/UPGRADE/REVOCATION，三处逐字匹配，不新增 JSON 根、签名角色或第 33 个解析类别。产品 wrapper 角色保持原产品 DN，原生渠道 pin 独立审定并保留历史版本；候选不能自己选择信任根。平台升级测试使用独立重置实例，既验证错误传输拒绝，也验证真实原生安装请求因受控空间不足失败后保留可启动版本。实际平台结果是既有 UPGRADE schema 的封闭嵌入类型，不使用未定义的日志 blob 冒充证明。原生链的撤销与有效时间依所需 CRL/OCSP 原始 DER 证据验证，缺失或 soft-fail 不放行；Android 已登记的自签证书只适用明确的无 CA 撤销服务边界。完整规则及官方依据见[客户端契约 §3.4](../specs/2026-08-23-f57-client-lifecycle-security-contract.md#34-解析语料原生生命周期输入与签名边界)。

独立复核还修正两处新增导航表述：拓扑是经验证的 plain JCS，不是独立 signed topology；G6-10 只交付静态清单/投影/fixtures，生产签名构建、安装及动态读回归 Task 15，相关 collector/tooling 先在 Task 14 交付。

最终候选经过架构、Windows/存储、开发就绪三个方向的独立复核；在本轮文档范围内未发现遗留 Critical/Important 问题。真实原生升级失败结果、撤销证据和签名边界已完成交叉复核；此结论不替代未来 G5/G6 的实现与实机验收。

## 本地验证证据

本轮没有修改可执行源码，因此以文档一致性、现有编译和登记失败对比确认交付。历史[深度收敛核验](2026-09-07-deep-release-convergence-verification.md)的 25 项结果不冒充本轮重新执行。

| 实际运行项 | 结果与范围 |
|---|---|
| `git diff --check` | 通过；最终候选再次核对通过 |
| `cargo fmt --all -- --check` | 通过 |
| `cargo check --workspace --all-features --locked --offline` | 退出 0；现有全特性代码可编译，不代表文档中的未来类型已实现 |
| `bash .github/ci/tests/run-negative.sh` | 退出 0；49 条负例符合预期，无法构造 0，异常结果 0 |
| `bash scripts/tests/dev-controls-negative.sh` | 退出 0；Unix/dash 测试替身和 PowerShell 静态顺序检查通过，没有启动真实容器 |
| `bash .github/ci/verify-pipeline-commands.sh` | 退出 0；11 阶段/19 条命令的可用性、离线约束和文档表一致；不代表这 19 条均已执行 |
| `bash .github/ci/compare-red-baseline.sh` | 初轮和最终候选均完整执行、退出 0；七个面与登记基线精确相等，包含全工作区测试 |
| `cargo xtask f57 verify --level l0 --changed-from HEAD` | 当前实测未知 f57／退出 2；未执行 F57 检查、未产生证据 |

完整比较器不是发布门，退出 0 只证明未比登记基线新增失败：

| 检查面 | 登记 / 实测退出类别与不符数 |
|---|---|
| archcheck | `0/0` |
| sqlcheck | `0/0` |
| codecheck | `1/2` |
| errorcodes | `1/12` |
| configdoc | `1/304` |
| eventcatalog | `1/117` |
| cargo-test | `1/3`；原始 Cargo 测试进程为 101 |

三条已登记测试失败仍是 `codecheck::negative_samples::the_repository_itself_passes`、`configdoc::negative_samples::the_repository_passes`、`eventcatalog::negative_samples::the_repository_is_clean_with_registered_events`。本轮未将其关闭，也不声称“全部测试通过”。

静态复核还重新计算九桶合计 `1,481,763,717,120` bytes、不可借用 100 GiB 和名义容量 `1,589,137,899,520` bytes；验证 60 GiB 子项、32/30/24/16 GiB、37/52/57 连接口径、八行/六类政策、47 个唯一迁移端点、冻结输入不变及新增链接。另核对三种 readback 字段数 `16/13/13`、native_inputs 的 `12` 字段、渠道 pin 的 `6` 字段与证书 pin 的 `4` 字段均与唯一声明相等。上述只是文档、契约和算术检查；未来 Rust 类型与平台用例仍未实现，没有测量真实磁盘、RAM、IOPS 或恢复时间。

## 官方资料复核

以下查询日期为 2026-09-07，仅用于检查外部事实；产品配置仍需经过现行签名投影与实际认证。

- Microsoft 的 [Windows Server 版本信息 ISO 日期表](https://learn.microsoft.com/en-us/windows/release-health/windows-server-release-info)确认 Windows Server 2022 主流支持截至 2026-10-13、扩展支持截至 2031-10-14；与 profile 的现有日期一致。生命周期页面的本地化/时区显示可能跨日，因此同时核对 ISO 表。保留 2022 当前认证目标，后继 LTSC 的迁移仍需独立认证，不能无条件换成 2025。
- [PostgreSQL 版本政策](https://www.postgresql.org/support/versioning/)当日列出 16 的当前 minor 为 16.15、最终支持日期 2028-11-09；此查询不自动更新已固定的 package lock。实际选版、升级和服务停机必须经现行维护与认证流程。
- [PostgreSQL 16 WAL 参数](https://www.postgresql.org/docs/16/runtime-config-wal.html)说明关闭 fsync 会带来断电/系统崩溃下不可恢复损坏风险。SSD 运行/HDD 数据方案不靠关闭持久性保障提速，是否达标仍取真实 flush、断电和恢复证据。
- [Microsoft 页面文件说明](https://learn.microsoft.com/en-us/troubleshoot/windows-client/performance/introduction-to-the-page-file)解释 commit limit/committed bytes 与页面文件关系。本项目无页文件阈值来自自己的最高安全 profile，是必须实测的工程约束，不能解释为 Microsoft 对所有 Windows 主机建议禁用页面文件。

## 外部未覆盖与交付边界

没有执行 Windows Server/P340 实机安装、PowerShell 运行时、真实 PostgreSQL 六项测试、UPS 断电、真实 HDD/VSS、72 小时负载、备份恢复、原生客户端签名/升级或最终 L3。现有六项 PostgreSQL 测试保持外部忽略状态；这些条件不能用 macOS 编译或文档静态复核替代。

上传只覆盖本轮项目文档，目标为 `origin/docs/spec-review-revisions`，不强推、不合并主分支、不创建发布、不部署。原始本地检查日志与临时校验脚本在仓库外，不含生产数据，也不纳入交付。完成上传以本地 HEAD、远端查询结果及跟踪分支相等、工作区干净为准；最终交付消息给出对应提交。
