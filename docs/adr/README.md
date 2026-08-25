# 架构决策记录（ADR）

本目录是本仓全部架构决策的唯一登记处。技术基线第 1.1 节把 `/docs/` 登记为规格、PRD、ADR、数据字典、错误码表与事件目录的落点，本目录即其中的 ADR 一项。

## 一条记录什么时候该写

只登记满足下面任一条的决定：

- 改变了目录布局、crate 边界或依赖方向；
- 冻结了一个此后不得单独改动的取值（工具链版本、数据库排序规则、构建目标三类）；
- 在共享技术基线之外新增或偏离了一条约定；
- 引入了一处已知的临时实现，其替换方在后续阶段。

不登记的：单个函数的写法、可由代码本身读出的事实、尚未做出的决定。第四类如果只是「以后再说」，应留在阶段计划的风险一节，不占 ADR 编号。

## 编号与文件名

文件名形如 `ADR-<四位序号>-<英文短横线标题>.md`，序号一经分配不再回收，被推翻的记录保留原文并把状态改为「已被 ADR-XXXX 取代」。`ADR-0002` 的编号由 `rust-toolchain.toml` 首行的注释直接引用，改号即会使该引用失效。

## 固定结构

每篇按背景、决定、理由、后果、影响范围五段写。理由一段必须写明不采用该决定的代价，只写好处的记录不算完成。

## 现有记录

| 编号 | 标题 | 状态 | 出处 |
|---|---|---|---|
| [ADR-0001](ADR-0001-new-crate-ep-platform-runtime.md) | 新增 crate `ep-platform-runtime` | 部分被 ADR-0019 取代；共享库决定仍接受 | 阶段 1 计划第 13 节偏离一 |
| [ADR-0002](ADR-0002-toolchain-freeze.md) | 工具链版本冻结 | 已接受 | 阶段 1 计划第 13 节假设一 |
| [ADR-0003](ADR-0003-database-collation.md) | 数据库默认排序固定为 libc/C 字节序 | 已接受 | 阶段 1 计划第 13 节新增决定二 |
| [ADR-0004](ADR-0004-musl-static-linking.md) | 历史 Linux musl/scratch 构建决定（已由 Windows 原生部署取代） | 已取代 | 阶段 1 计划第 13 节新增决定七、F-51 |
| [ADR-0005](ADR-0005-ci-platform.md) | Forgejo + Woodpecker Windows agent；现行由 F-57 Rust command family 唯一判定，平台只作薄适配 | 部分被 ADR-0022 与 F-57 现行流水线取代 | 阶段 1 计划第 13 节新增决定八、F-51 |
| [ADR-0006](ADR-0006-domain-invariant-property-tests.md) | 五组领域不变量属性测试的挂载点 | 已接受 | 阶段 1 计划第 9 节领域属性测试一段 |
| [ADR-0007](ADR-0007-file-secret-provider-interim.md) | `FileSecretProvider` 为阶段 1 临时实现；生产终态为 `KmsSecretProvider` | 已取代 | 阶段 1 计划第 8 节末段、阶段 2 §4.3a |
| [ADR-0008](ADR-0008-five-named-pools-budget-exit-78.md) | 历史五具名连接池与启动预算求和校验违例退 78 | 已被 ADR-0018 取代 | 阶段 2 计划第 7.2 节与裁定 C-04 |
| [ADR-0009](ADR-0009-kms-registry-db-as-truth.md) | 内置 KMS 内存注册表不持久化，重启后以数据库为基准重建 | 部分被 ADR-0020 取代 | 阶段 2 任务 #12、#14 偏离登记第十二条；阶段 3b 不接管 |
| [ADR-0010](ADR-0010-migration-window-singleton-lock.md) | 迁移窗口并发开窗以单例锁表串行化 | 已接受 | 阶段 2 计划第 3.5 节表六 |
| [ADR-0011](ADR-0011-endpoint-context-interim.md) | 端点上下文头推导的阶段 2 临时实现；现行客户端闭集由阶段 4 与 F-55 接管 | 已关闭 | 阶段 2 任务 #14 偏离登记第五条、阶段 4、F-55 |
| [ADR-0012](ADR-0012-axum-query-feature.md) | workspace axum 依赖条目新增 `query` feature | 已接受 | 阶段 2 任务 #14 偏离登记第十三条 |
| [ADR-0013](ADR-0013-migration-runner-refinery-replacement.md) | 迁移执行器弃用 refinery 改自建语义兼容 Runner | 已接受 | 阶段 2 计划 §3.3 与 §12 实施期偏离登记第一条 |
| [ADR-0014](ADR-0014-cipher-envelope-aad-binding.md) | 密文自描述信封头与 AAD 三段拼接 | 已接受 | 阶段 2 计划 §4.3 与 §12 新增决定四 |
| [ADR-0015](ADR-0015-blind-index-16-byte-truncation.md) | 盲索引固定使用完整 32 字节，唯一性由业务约束决定 | 已接受并冻结 | 阶段 2 计划 §4.4 与 §12 假设三 |
| [ADR-0016](ADR-0016-legal-entities-unpoliced.md) | `legal_entities` 不建行级安全策略并登记于未受策略表登记表 | 已接受 | 阶段 2 计划 §3.5 表一与 §12 偏离项第一条 |
| [ADR-0017](ADR-0017-no-delete-grant-on-business-schemas.md) | 运行期账号在业务 schema 上不授予 DELETE | 已接受 | 阶段 2 计划 §12 新增决定五 |
| [ADR-0018](ADR-0018-integration-gateway-zero-database.md) | 四具名常驻池、分层连接预算与 `integration-gateway` 零数据库边界 | 部分被 ADR-0019 取代；网关零边界仍冻结 | 2026-08-21 数据库架构收口 |
| [ADR-0019](ADR-0019-f57-runtime-topology-and-measured-connection-budget.md) | F-57 运行拓扑与实测连接预算 | 已接受 | F-57 `RULING-PROCESS-01` |
| [ADR-0020](ADR-0020-dual-recipient-data-key-recovery.md) | 双 recipient 数据密钥恢复 | 已接受 | F-57 `SEC-014` 与威胁模型 |
| [ADR-0021](ADR-0021-epb1-backup-envelope.md) | EPB1 备份信封 | 已接受 | F-57 勒索恢复契约 |
| [ADR-0022](ADR-0022-f57-multi-lane-ci-and-windows-2022-authority.md) | F-57 多 lane CI 与 Windows Server 2022 权威 | 已接受 | F-57 多平台发布契约 |
| [ADR-0023](ADR-0023-f57-provider-manifest-resource-grant.md) | F-57 Provider Manifest 与 Resource Grant | 已接受 | F-57 provider、MCP、能力包权限与 carrier 契约 |
| [ADR-0024](ADR-0024-f57-backup-key-envelope.md) | F-57 BackupKeyEnvelopeV1 | 已接受 | F-57 每备份集 recovery-only 密钥信封契约 |
| [ADR-0025](ADR-0025-f57-capability-graph-and-feature-first-boundaries.md) | F-57 单一能力图与 feature-first 边界 | 已接受 | F-57 2026-08-24 架构收敛修订 |

阶段 1 退出条件 18 要求本目录至少含工具链冻结、collation 选型、部署构建决定、CI 平台选型、新增 crate 五篇，即上表的 ADR-0002、ADR-0003、ADR-0004、ADR-0005 与 ADR-0001。ADR-0004 仅保留 Linux 方案的历史追溯，现行构建与部署以 F-51 的 Windows 原生决定及 ADR-0022 的 F-57 多 lane 流水线为准，不得把 musl/scratch 当作当前退出条件。ADR-0006 与 ADR-0007 是阶段 1 计划正文另行点名要求写入 ADR 的两项，一并在本阶段登记。ADR-0008 至 ADR-0012 五篇由阶段 2 任务 #14 登记：0010、0012 分别是基线外新增约定与依赖能力面变更，0008 只保留当时五池决定的历史追溯；ADR-0009 的数据库元数据/exact-ref/cache/readback 部分仍有效，但单一 wrapped DEK 已由 ADR-0020 取代。ADR-0013 至 ADR-0017 五篇由阶段 2 任务 #17 按 D-13/E-12 点名补齐：0013 与 0016 对应基线外偏离（实施期偏离登记第一条、偏离项第一条），0014、0015、0017 对应 §12 新增决定四、假设三与新增决定五；其中 0014 的 EPC1 仍只覆盖字段、附件和归档，备份改用 ADR-0021 EPB1，0015 的现行契约固定为完整 32 字节且无宽度配置。F-57 新增 ADR-0019 至 ADR-0024：0019 让进程数成为部署细节并要求逐硬件/代实测连接预算，同时保留 `ep-platform-runtime` 和 `integration-gateway` 零边界；0020 冻结 operational/recovery 双 recipient DEK；0021 冻结独立 EPB1 备份信封；0022 冻结 Windows Server 2022、Apple、Android 与签名聚合四 lane；0023 冻结 provider/carrier manifest、调用级最小 `ResourceGrantV1` 与通用 XML 禁用边界；0024 在 ADR-0021 之上冻结每 backup set 的 recovery-only `BackupKeyEnvelopeV1`、三份加密 share 与跨洁净主机恢复。历史 `37 + 10 + 5 = 52` 只作测量种子。
