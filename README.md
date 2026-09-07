# B_Project01

面向合同驱动型企业的私有化、可治理、可组合业务自动化平台。

## 当前状态

- 文档完整度：**F-57 Governed Automation Fabric** 已完成 2026-08-24 收敛并获用户批准；`DESIGN_READY` 仅描述设计文档完整度，不是开发或实现状态。
- 开发状态字段：`development_state=READY_NOT_AUTHORIZED`；`blocking_reason=NONE`；`DEVELOPMENT_AUTHORIZED=true`（开发授权已于 2026-08-27 由使用方授予（逐字指令「需要 完全 可即刻开发 状态 最谨慎」，留证 00c F-65）；状态机节点名依 converged §7 保持至 G0_BOOTSTRAP_GREEN）；`implementation_state=NOT_IMPLEMENTED`。五文件计划已具备所需细度；**授权已于 2026-08-27 获得（F-65），开发从 G0 开始**。
- 范围：现行闭集仍为 **185 个 RequirementID（174 个主需求 + 11 个延期边界）**；没有删除、合并或静默延期最终要求。
- 代码：仓库已有 Rust/PostgreSQL 骨架、早期平台逻辑和历史迁移；CapabilityGraph、F-57 权威主干、CTC-01、四端客户端、完整业务范围和发布认证均未实现。
- 生产状态字段：`production_state=PRODUCTION_NOT_READY`。现有 ThinkStation P340 主机与 Windows Server 2022 可继续作为待认证候选，但原单块 1 TB HDD 的 profile 永久 `production_eligible=false`，不能靠补 UPS 或补证据转为权威生产盘，也不能复用为 RAID1 成员或备份盘；唯一低成本例外是被合格新盘替代、完成安全擦除并获处置批准后，最多作可随时丢弃的非权威暂存。必须先换装/新增一块物理 CMR 原始容量 `>=2,000,000,000,000` bytes 的 DATA_HDD；NTFS 权威卷以 `1,589,137,899,520` bytes 为名义 floor，并须满足 `volume_total_bytes >= max(1,589,137,899,520,1,481,763,717,120+107,374,182,400+measured_unclassifiable_filesystem_allocation_bytes)`。九桶/十四 capacity class/十四 canonical selector 只唯一覆盖产品管理 `data_root` 对象与登记 VSS extent；NTFS/BitLocker 元数据独立实测且不可借桶，普通卷外对象或解释不了的 allocation 一律失败关闭。随后 UPS、服务器外只追加备份、两块离线轮换 HDD、分域恢复材料、洁净恢复硬件、72 小时容量和完整 L3 证据仍须全部通过，才可录入真实客户数据。
- 本轮没有实现 F-57 业务能力、创建 F-57 业务迁移、安装服务或改变生产环境；只修复现有 Rust/CI/本地控制骨架的安全性与一致性缺陷，并同步完成文档再基线。

## 阅读导航（不定义权威排序）

逐文件冲突的唯一 precedence 由 [F-57 总体设计 §1.1](docs/superpowers/specs/2026-08-23-f57-governed-automation-fabric-design.md) 持有。本节只提供阅读导航；编号不构成优先级、实施顺序或授权。

- [F-57 总体设计](docs/superpowers/specs/2026-08-23-f57-governed-automation-fabric-design.md)
- [F-57 业务执行契约](docs/superpowers/specs/2026-08-23-f57-business-execution-contract.md)
- [F-57 客户端、生命周期与安全运营契约](docs/superpowers/specs/2026-08-23-f57-client-lifecycle-security-contract.md)
- [185 项完整需求追踪](docs/superpowers/reviews/2026-08-23-f57-requirements-traceability.md)
- [文档权威与取代登记](docs/superpowers/reviews/2026-08-23-f57-authority-supersession-register.md)
- [ADR 索引](docs/adr/README.md)，其中 ADR-0019 至 ADR-0025 均为已接受的 F-57 技术决定
- [Windows Server / P340 生产档案](docs/superpowers/specs/2026-08-23-f57-windows-p340-production-profile.md)
- [仓库威胁模型](docs/threat-model.md)中的 F-57 现行部分
- [收敛实施主计划](docs/superpowers/plans/2026-08-24-f57-converged-program.md) 与 G0、G1/G2、G3/G4、G5/G6 子计划
- [开发就绪与全场景静态演算](docs/superpowers/reviews/2026-08-23-f57-development-readiness-verification.md)
- [L0–L3 验证与发布证据契约](docs/ci-pipeline.md)

种子登记表是 G0 的受控导入输入，不是已实现 API 或已通过证据。G0 首次导入并通过逐字节往返后，五个 API 种子才转为 CapabilityGraph 的生成投影。配置、数据、错误、事件、指标、影响面和迁移目录在 G0 再基线前仍是受控历史输入。

## 禁止误用

- [2026-08-23 旧 25 项计划](docs/superpowers/plans/2026-08-23-f57-governed-automation-fabric-implementation.md)永久为 `HISTORICAL_DETAIL_INPUT`；可以提取细节，不能执行任务、迁移或旧门禁。
- `docs/superpowers/plans/2026-08-10-first-release-dev-plan/` 和 F-50/F-55/F-56 顶层实施计划不是并行执行队列。
- 旧文档中的 `Task 1…25` 只解释为 `F57-01…F57-25` 需求所有权桶；实际顺序、文件、迁移和门禁只由 2026-08-24 五文件计划集决定。
- F-55 本地模型实现已延期；当前阶段只保留 AI provider、权限、工具、隔离和审计契约。
- 产品介绍 DOCX 是 `CURRENT_SUMMARY_NON_NORMATIVE`，用于易懂介绍，不定义接口、范围、实现状态或验收。
- `deploy/` 中 Linux/systemd/Podman/Compose 内容是历史研究，不是 Windows Server 2022 生产入口。
- `DESIGN_READY` 只说明设计与计划可执行，不等于功能已实现；只有同一最终候选通过 L3，才能签发 `RELEASE_CERTIFIED`，而客户生产准入还需站点证据。
