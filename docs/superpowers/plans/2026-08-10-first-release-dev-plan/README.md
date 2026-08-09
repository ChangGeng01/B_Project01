# 首版研发计划(14 阶段)

本目录是《企业私有化可组合运营平台》首版的技术开发计划，共 14 个阶段。

阅读顺序：先读总览与技术基线，再按阶段号顺序读。技术基线是 14 个阶段的共享前提,
凡基线已给出取值的事项，各阶段直接引用，不重新决定。

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
| [14-ops-backup-release.md](14-ops-backup-release.md) | 阶段 14:运维、备份与发布硬化 |

## 权威来源

- 总体设计规格:`docs/superpowers/specs/2026-07-19-enterprise-private-operations-platform-design.md`
- 首版 PRD:`docs/superpowers/specs/2026-08-09-first-release-prd.md`
- 单服务器与规模收窄口径:`docs/superpowers/reviews/2026-08-04-single-server-deployment-decisions.md`

冲突时以规格为准，其次 PRD,最后本计划。账务规则一律以规格第 5.2 章事件-分录表为准，本计划不复述。
