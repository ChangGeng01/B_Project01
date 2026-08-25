# ADR-0005 CI 默认取 Forgejo、Woodpecker 与 Windows agent

- 状态：部分被 ADR-0022 取代；私有自建默认、薄适配器、Rust 唯一判定与 Authenticode 意图继续接受
- 取代说明：[ADR-0022](ADR-0022-f57-multi-lane-ci-and-windows-2022-authority.md) 取代本文单 Windows runner 与 Windows Server 2019 复核要求，现行 Windows 权威只接受 Windows Server 2022 并聚合 Apple/Android lane
- 出处：阶段 1 计划第 13 节新增决定八、裁定 F-08 Windows/CI 最终冻结
- 该取值不进入产品制品

## 背景

产品面向客户内网私有部署，源码、离线依赖、制品与签名材料不得因研发设施而被迫出网。服务端交付目标已冻结为 Windows Server 原生，因此默认执行器必须能在 Windows 环境运行同一套门禁。

门禁若分散在 CI YAML、shell 脚本和平台专有步骤中，本机与流水线会产生不同结论，更换平台也会复制判定逻辑。

## 决定

1. 默认 CI 平台取内网自建 Forgejo 加 Woodpecker，执行器取 Windows agent。
2. 全部门禁只由 `cargo xtask ci` 聚合和判定。它是阶段集合、顺序、退出码与最终结果的唯一真值。
3. Woodpecker 配置与任何备用平台配置都是薄适配器：只准备环境、选择 agent、注入已批准的 secret 引用并调用 `cargo xtask ci`，不得直接表达门禁逻辑。
4. 生产 Windows 制品必须通过 Authenticode 门禁；开发制品可用内部 ECDSA P-256，但必须标记为开发签名且不得发布。Authenticode 证书可由软件厂商或客户提供。

## 理由

Forgejo 与 Woodpecker 可在内网部署，资源占用较低，满足低成本与源码不出网的要求。Windows agent 与产品目标平台一致，可把 PE 构建、服务、DACL、命名管道和 Job Object 的判定留在实际承载环境中。

单一 Rust 入口使 CI 平台可替换：以后更换 Woodpecker 时只需重写环境适配，不重写判定逻辑。它也消除 bash、POSIX 可执行位、Linux 路径和 cgroup 对现行 Windows 门禁的隐式依赖。

## 后果

- `.github/ci/pipeline-stages.tsv`、`run-pipeline.sh` 与直接调用子门禁的 YAML 不再是权威来源；若保留，只能作为待迁移的历史文件或调用唯一入口的薄适配器。
- 默认 Windows agent 是实施路径，不再作为设计二选一。若首批实机验证失败，对应门禁保持非零并按 F-08 已写明的保守处置修复；不得自行切换平台或复制第二套门禁。
- 构建机仍是单点，其备份与恢复由运维阶段的恢复门禁承担。

## 证据状态

本 ADR 只冻结选择，不证明仓库已经完成迁移。F-08 原要求在 Windows Server 2022 上运行完整 `cargo xtask ci`，并在 Windows Server 2019 上完成同项复核；该 Windows Server 2019 复核要求已由 [ADR-0022](ADR-0022-f57-multi-lane-ci-and-windows-2022-authority.md) 取代，不再是现行放行条件。现行实施只接受 Windows Server 2022 权威证据并按 ADR-0022 聚合 Apple、Android 与签名 lane；F-57 登记的 Windows 验证、lane 聚合、PE 可复现构建和生产 Authenticode 在证据实际生成前一律不得宣称通过。
