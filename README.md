# B_Project01

面向合同驱动型企业的私有化、可治理、可组合业务自动化平台。

## 当前状态

- 产品与架构：**F-57 Governed Automation Fabric** 已完成 2026-08-24 收敛并获用户批准，状态为 `DESIGN_READY`。
- 开发：五文件实施计划已经达到可直接执行的详细度，但用户尚未授权开始产品开发，状态为 `DEVELOPMENT_AUTHORIZATION_REQUIRED` / `IMPLEMENTATION_NOT_STARTED`。首次获得开发授权后只能从 G0 开始。
- 范围：现行闭集仍为 **185 个 RequirementID（174 个主需求 + 11 个延期边界）**；没有删除、合并或静默延期最终要求。
- 代码：仓库已有 Rust/PostgreSQL 骨架、早期平台逻辑和历史迁移；CapabilityGraph、F-57 权威主干、CTC-01、四端客户端、完整业务范围和发布认证均未实现。
- 生产：`PRODUCTION_NOT_READY`。现有 ThinkStation P340、单 1TB HDD 和 Windows Server 2022 只是候选硬件；在 UPS、服务器外只追加备份、两块离线轮换 HDD、分域恢复材料、洁净恢复硬件、72 小时容量和完整 L3 证据通过前，不得录入真实客户数据。
- 本轮只完成文档再基线；没有编写业务代码、创建 F-57 迁移、安装服务或改变生产环境。

## 唯一阅读与开发顺序

1. [F-57 总体设计](docs/superpowers/specs/2026-08-23-f57-governed-automation-fabric-design.md)
2. [F-57 业务执行契约](docs/superpowers/specs/2026-08-23-f57-business-execution-contract.md)
3. [F-57 客户端、生命周期与安全运营契约](docs/superpowers/specs/2026-08-23-f57-client-lifecycle-security-contract.md)
4. [185 项完整需求追踪](docs/superpowers/reviews/2026-08-23-f57-requirements-traceability.md)
5. [文档权威与取代登记](docs/superpowers/reviews/2026-08-23-f57-authority-supersession-register.md)
6. [ADR 索引](docs/adr/README.md)，其中 ADR-0019 至 ADR-0025 均为已接受的 F-57 技术决定
7. [Windows Server / P340 生产档案](docs/superpowers/specs/2026-08-23-f57-windows-p340-production-profile.md)
8. [仓库威胁模型](docs/threat-model.md)中的 F-57 现行部分
9. [收敛实施主计划](docs/superpowers/plans/2026-08-24-f57-converged-program.md)
10. [G0 启动计划](docs/superpowers/plans/2026-08-24-f57-g0-bootstrap-implementation.md)
11. [G1/G2 权威主干与 CTC 数据计划](docs/superpowers/plans/2026-08-24-f57-authority-spine-implementation.md)
12. [G3/G4 CTC-01 客户端闭环计划](docs/superpowers/plans/2026-08-24-f57-ctc01-implementation.md)
13. [G5/G6 扩展与最高安全发布计划](docs/superpowers/plans/2026-08-24-f57-expansion-release-implementation.md)
14. [开发就绪与全场景静态演算](docs/superpowers/reviews/2026-08-23-f57-development-readiness-verification.md)
15. [L0–L3 验证与发布证据契约](docs/ci-pipeline.md)

种子登记表是 G0 的受控导入输入，不是已实现 API 或已通过证据。G0 首次导入并通过逐字节往返后，五个 API 种子才转为 CapabilityGraph 的生成投影。配置、数据、错误、事件、指标、影响面和迁移目录在 G0 再基线前仍是受控历史输入。

## 禁止误用

- [2026-08-23 旧 25 项计划](docs/superpowers/plans/2026-08-23-f57-governed-automation-fabric-implementation.md)永久为 `HISTORICAL_DETAIL_INPUT`；可以提取细节，不能执行任务、迁移或旧门禁。
- `docs/superpowers/plans/2026-08-10-first-release-dev-plan/` 和 F-50/F-55/F-56 顶层实施计划不是并行执行队列。
- 旧文档中的 `Task 1…25` 只解释为 `F57-01…F57-25` 需求所有权桶；实际顺序、文件、迁移和门禁只由 2026-08-24 五文件计划集决定。
- F-55 本地模型实现已延期；当前阶段只保留 AI provider、权限、工具、隔离和审计契约。
- 产品介绍 DOCX 是 `CURRENT_SUMMARY_NON_NORMATIVE`，用于易懂介绍，不定义接口、范围、实现状态或验收。
- `deploy/` 中 Linux/systemd/Podman/Compose 内容是历史研究，不是 Windows Server 2022 生产入口。
- `DESIGN_READY` 只说明设计与计划可执行，不等于功能已实现；只有同一最终候选通过 L3，才能签发 `RELEASE_CERTIFIED`，而客户生产准入还需站点证据。
