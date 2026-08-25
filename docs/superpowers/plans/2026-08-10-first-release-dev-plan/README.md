# 首版研发计划(14 阶段)

> **F-57 现行状态（2026-08-23）：`HISTORICAL`。** 本目录不再作为整体执行入口。它保留可复用的领域字段、迁移、财务和测试细节，但固定角色、固定九进程、旧 AI/MCP/ServerAdmin、旧模块包、旧存储/容量/恢复及旧客户端任务必须由 F-57 新计划取代。在新的 [F-57 实施计划](../2026-08-24-f57-converged-program.md) 获得用户单独批准前，不执行本目录任务。

本目录是《企业私有化可组合运营平台》首版的技术开发计划，共 14 个阶段。

历史快照状态：**本文曾在 2026-08-10 宣称“文档已冻结、可直接开发”，该声明现已撤销。** F-50 至 F-56 只保留为 F-57 明确引用的主题来源；任何任务、迁移或门禁只有进入 [F-57 实施计划](../2026-08-24-f57-converged-program.md) 才可执行。

阅读顺序：先读总览与技术基线，再按阶段号顺序读。技术基线是 14 个阶段的共享前提,
凡基线已给出取值的事项，各阶段直接引用，不重新决定。 数据库迁移的具体版本、文件名和路径统一见[数据库迁移目录](../../../migration-catalog.md)，阶段正文不得另行占号。

| 文件 | 内容 |
|---|---|
| [00-overview.md](00-overview.md) | 总览、十四阶段总表、依赖图、跨阶段接口核对表、里程碑、全局风险 |
| [00b-technical-baseline.md](00b-technical-baseline.md) | 共享技术基线:workspace 布局、进程清单、数据库约定、API 契约、事件、配置、测试门槛 |
| [00c-gap-ruling.md](00c-gap-ruling.md) | 跨阶段缺口归属裁定表：67 条缺口的最终归属、确切签名与回写清单 |
| [01-engineering-baseline.md](01-engineering-baseline.md) | 阶段 1:工程基座与 CI |
| [02-data-foundation.md](02-data-foundation.md) | 阶段 2:数据基座与隔离 |
| [03-platform-kernel.md](03-platform-kernel.md) | 阶段 3:平台内核 |
| [04-identity-authz.md](04-identity-authz.md) | 阶段 4:身份、认证与权限 |
| [05-master-data.md](05-master-data.md) | 阶段 5:主数据 |
| [06-contract-sales.md](06-contract-sales.md) | 阶段 6:合同与销售 |
| [07-procurement-portal.md](07-procurement-portal.md) | 阶段 7:采购、门户与收货 |
| [08-inventory-costing.md](08-inventory-costing.md) | 阶段 8:库存与存货计价 |
| [09-ledger-period.md](09-ledger-period.md) | 阶段 9:财务内核一：总账与期间 |
| [10-ar-ap-invoice.md](10-ar-ap-invoice.md) | 阶段 10:财务内核二：往来与发票 |
| [11-cost-metrics-reporting.md](11-cost-metrics-reporting.md) | 阶段 11:成本、指标与报表 |
| [12-service-project-asset.md](12-service-project-asset.md) | 阶段 12:售后、项目与设备 |
| [13-clients-lowcode.md](13-clients-lowcode.md) | 阶段 13:四端客户端与低代码 |
| [13c-local-ai-mcp-server-admin.md](13c-local-ai-mcp-server-admin.md) | 阶段 13c：本地 AI、双向 MCP、ServerAdmin 与云承载接入 |
| [14-ops-backup-release.md](14-ops-backup-release.md) | 阶段 14:运维、备份与发布硬化 |

## 权威来源

- 总体设计规格:`docs/superpowers/specs/2026-07-19-enterprise-private-operations-platform-design.md`
- 首版 PRD:`docs/superpowers/specs/2026-08-09-first-release-prd.md`
- F-50 财务一致性裁定:`docs/superpowers/specs/2026-08-21-f50-financial-consistency-design.md`
- F-51 开发就绪冻结:`docs/superpowers/specs/2026-08-21-f51-development-readiness-freeze.md`
- F-52 至 F-54 后续裁定:`00c-gap-ruling.md` 的对应段
- F-55 已批准范围冻结:`docs/superpowers/specs/2026-08-22-f55-approved-ai-mcp-server-admin-cloud-freeze.md`
- F-56 许可证与签名模块包终态冻结:`docs/superpowers/specs/2026-08-22-f56-license-signed-module-package-freeze.md`
- F-56 实施清单:`docs/superpowers/plans/2026-08-22-license-module-package-implementation.md`
- 单服务器与规模收窄口径:`docs/superpowers/reviews/2026-08-04-single-server-deployment-decisions.md`

冲突时以总体规格为基础权威；F-50 至 F-56 在各自明确范围内覆盖较早口径，且同范围较晚专项裁定优先；其后依次为 PRD 与十四阶段计划。账务规则一律以规格第 5.2 章及 F-50 的后续明示修订为准；F-55 范围按其 exact ABI、数据图、门禁和三份实施计划执行，许可证、签名模块包及 F-55 entitlement 的重叠面再由 F-56 原子替换，本计划不另造分支。
