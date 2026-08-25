# F-57 完整需求追踪矩阵

> 日期：2026-08-23（Australia/Melbourne）
> 状态：产品需求已批准、实现状态均为 `NOT_IMPLEMENTED`
> 来源：[管理软件基本需求.docx](../../介绍/管理软件基本需求.docx)、现行 PRD 未冲突细节、F-50 财务一致性与 F-57 总体设计
> 用途：把来源需求追踪到能力所有者和验收证据；本文件不宣称任何功能已经开发

> 2026-08-24 解释规则：本文件中的 `Task 1…25` 仅是 `F57-01…F57-25` 需求所有权桶，不再表示旧计划执行顺序。实际执行只依据 `2026-08-24-f57-converged-program.md` 及其四份子计划；G0 承接旧 Task 1 的机器登记职责。

## 1. 字段与状态

| 字段 | 含义 |
|---|---|
| ID | 稳定需求编号；测试、数据字典、接口和计划必须引用 |
| 来源 | `DOCX`、`PRD`、`F50`、`F55`、`F56` 或 `F57`；组合来源用 `/`，精确文件由下方来源定位表解析 |
| 处置 | `RETAINED`、`EXPANDED`、`SUPERSEDED`、`DEFERRED_WITH_INTERFACE`、`OUT_OF_SCOPE` |
| 所有者 | 唯一 owning capability；跨域协作仍只有一个事实所有者 |
| 验收 | 最低必须产生的自动测试、演算或部署证据 |

本文所有行的实现状态固定为 `NOT_IMPLEMENTED`。进入开发后只能逐行晋级为 `IN_PROGRESS`、`IMPLEMENTED_UNVERIFIED`、`VERIFIED`；没有验收证据不得标为 `VERIFIED`。

### 1.1 每行的完整追踪包络

为保持主矩阵易读，以下字段不在每张表重复展示，但对每一行都是强制且可机械推导的现行字段：

| 完整字段 | 现行取值规则 |
|---|---|
| `RequirementID` | 主矩阵 `ID` 原值 |
| `SourceClause[]` | §15 对每个 `RequirementID` 逐条列出的精确文件章节或原始功能段；不得用 ID 区间、通配符或“对应同名章节”代替 |
| `Supersedes[]` | §15 对每个 `RequirementID` 逐条列出的旧句/旧约束；没有旧限制时为 `NONE` |
| `RulingID[]` | §15 对每个 `RequirementID` 逐条列出的精确裁决 ID；没有裁决时为 `NONE` |
| `CapabilityOwner` | 主矩阵唯一`所有者`；它拥有该需求的命令、派生结果和验收，不因聚合其他域数据而取得其他域事实的写所有权 |
| `SourceDataOwner[]` | §15 对每个 `RequirementID` 逐条列出的事实来源 owner；token 只能取自主矩阵`所有者`集合，首 token 固定为 `CapabilityOwner`。仅消费自身数据时数组只有该 token；跨域时追加全部只读来源 owner。非 `CapabilityOwner` token 只能经公开 query/fact contract 读取，禁止直写或形成共同所有权 |
| `CommandEvent` | 稳定登记键为 `CMD-F57-<ID>` 与 `EVT-F57-<ID>-ACCEPTED`；只读/NFR 行使用 `QRY-F57-<ID>` 或 `EVID-F57-<ID>`，G0 登记真实类型名和路径 |
| `PermissionPolicy` | `CAP-F57-<ID>`；纯证据行固定为 `SYSTEM_EVIDENCE_ONLY` |
| `UISurface` | 依 §1.4 前缀映射；没有人机界面的内核行固定为 `AUTHORITY_INTERNAL` |
| `ConfigGeneration` | `REQUIRED`；只有不可变历史事实没有独立开关，仍记录执行时 generation |
| `TestID` | `T-F57-<ID>` |
| `EvidenceID` | `E-F57-<ID>` |
| `Status` | 当前全部 `NOT_IMPLEMENTED` |

收敛计划 G0 必须把这些稳定键写入数据、接口、权限、事件、错误、指标和迁移登记，并将真实代码/测试路径绑定到同一 ID；不得另造不回链本矩阵的名称。

### 1.2 来源定位

| 简码 | 精确来源 |
|---|---|
| `DOCX` | `docs/介绍/管理软件基本需求.docx` 客户原文；§15 使用其可识别的原始功能标题/需求短句定位 |
| `PRD` | `docs/superpowers/specs/2026-08-09-first-release-prd.md`；§15 使用明确章节或业务主题定位，仅保留未被 F-57 取代的字段/状态/规则 |
| `F50` | `docs/superpowers/specs/2026-08-21-f50-financial-consistency-design.md`；§15 使用明确的发票、资金、核销、冲销、期间或历史余额主题定位 |
| `F55` | `docs/superpowers/specs/2026-08-22-f55-approved-ai-mcp-server-admin-cloud-freeze.md` 仅保留 F-57 权威登记允许的隔离/证据意图 |
| `F56` | `docs/superpowers/specs/2026-08-22-f56-license-signed-module-package-freeze.md` 仅保留许可、内置模块许可信封和信任链 |
| `F57` | `docs/superpowers/specs/2026-08-23-f57-governed-automation-fabric-design.md` 及其明确纳入的 `2026-08-23-f57-business-execution-contract.md`、`2026-08-23-f57-client-lifecycle-security-contract.md` 对应主题章节 |

### 1.3 裁决索引

本节只登记可引用的裁决键，不再做范围映射。每个裁决究竟约束哪些需求，以 §15 的逐条 `RulingID[]` 为唯一机器可读关系。

| 裁决键 | 主题 |
|---|---|
| `RULING-AUTHORITY-01` | F-57 取代旧开发入口 |
| `RULING-AUTHZ-01` / `RULING-AUTHZ-02` | 动态能力授权、临时授权与改派 |
| `RULING-PKG-01` | 完整能力包取代旧闭集模块包 |
| `RULING-UX-01` / `RULING-PROCESS-01` | 双端 UI 与不冻结进程数 |
| `RULING-AI-01` / `RULING-MCP-01` | AI 阶段边界与 MCP 扩展 |
| `RULING-DATA-01` / `RULING-HW-01` / `RULING-HA-01` | HDD、恢复目标与单写暖备 |
| `RULING-POSTGRES-WIN-01` | Windows Server 2022 上 PostgreSQL 16 的安装、服务、连接预算与证据闭环 |
| `RULING-BACKUP-SAFEGUARD-01` | 独立备份拓扑签名信任、五态保护、自举与轮换门禁 |
| `RULING-UPS-01` | UPS provider、运行身份、状态与断电命令证据闭环 |
| `RULING-OFFLINE-01` / `RULING-FLOW-01` / `RULING-DB-01` | 离线、闭环自动化与关系结构定制 |
| `RULING-F10-01` | F-10 计数不再形成产品选择 |
| `RULING-BUSINESS-SCOPE-01` | 旧 97 项与当前业务范围的逐项提升规则 |
| `RULING-FIN-PERIOD-01` | 经营期间永久锁定与迟到事实顺延 |

### 1.4 UI 前缀映射

| 前缀 | 主 UI |
|---|---|
| `GOV/PKG/SEC/NFR/OPS` | 服务器控制中心；需要业务预览的部分同时下发 Workbench 签名 schema |
| `AUTH` | 服务器控制中心负责授权定义、模拟、发布与审计；Workbench 承载运行时裁剪、访问申请、委托和业务审批入口 |
| `MDM/CRM/CPQ/CLM/SAL/PROC/INV/FIN/SRV/PRJ/REP/AUT` | Workbench；对应外部参与者能力另走门户 |
| `PLT/CUS` | 服务器控制中心负责定义、模拟与发布；Workbench 负责搜索、通知、审批、附件及按签名 schema 使用和预览客户 UI |
| `CLI` | 四平台 Workbench |
| `POR` | 服务器控制中心管理 allowlist/gateway，客户/供应商门户承载外部参与者 UI |
| `INT/MCP/AI/IDP` | 服务器控制中心配置与证据；业务结果按权限回到 Workbench |
| `DBP` | 服务器控制中心与权威数据库内部运维面 |
| `DEF` | 无独立 UI；使用其 `Canonical RequirementID` 对应界面，未启用能力只显示边界和证据状态 |

## 2. 来源解释规则

| 冲突主题 | 现行解释 |
|---|---|
| 原 DOCX 的五类使用人员 | 保留为默认人物和工作台模板，不是写死岗位或授权边界 |
| 原 DOCX 的角色、模块、表单、字段权限 | 由 F-57 动态能力、作用域、条件、期限、设备和字段裁剪模型扩展覆盖 |
| 原 DOCX 的“数据库完整可定制” | 通过核心保护区与客户扩展区实现；禁止任意生产 SQL 和插件直写核心表 |
| 旧 PRD 的范围删减 | 旧延期项默认保持延期；只有本矩阵或 F-57 §11 明确逐项提升为 `RETAINED`、`EXPANDED` 或 `SUPERSEDED` 的能力才进入当前产品范围。未逐项提升的 HR、GRC、法务、商旅、ECM/CMS/GIS、PLM/PIM/QMS 等能力不得因本矩阵未重复列出而自动恢复 |
| 固定九进程和第五客户端 | 已由 F-57 的信任边界、服务器控制中心和四端 Workbench 取代 |
| 本地 AI 首发 | provider/权限/审计接口保留；具体本地模型为 `DEFERRED_WITH_INTERFACE` |
| F-56 模块包 | 仅是内置模块授权包；完整 `CAPABILITY_PACKAGE` 以 F-57 为准 |
| 法定总账、税务、工资 | 专业系统连接器；平台保留经营事实账、AR/AP、发票、资金、库存成本和对账 |
| 离线冲突 | 非敏感且可证明不冲突的字段可合并；金额、状态、权限、合同等必须人工处理 |
| HDD | `HDD_STRICT` 只约束权威节点：数据库、WAL、附件、索引、日志、临时文件、导出、pagefile/dump 和可关联客户的衍生数据全部覆盖；终端缓存及权威 SSD 的两个窄例外分别以 `CLI-004`、`SEC-011`、`SEC-012` 为准 |

## 3. 权威端与平台治理

| ID | 来源 | 要求 | 处置 | 所有者 | 验收 |
|---|---|---|---|---|---|
| GOV-001 | F57 | 每次业务操作由服务器重新鉴权并产生唯一权威结果 | EXPANDED | authority-kernel | 跨客户端同命令结果、越权和重放测试 |
| GOV-002 | F57 | 控制中心和全部权威服务仅运行于 Windows Server 2022 | EXPANDED | authority-runtime | Windows 实机服务与零 WSL 证据 |
| GOV-003 | F57 | 配置以签名、不可变、可回滚的发布代分发并协调 desired/observed state | EXPANDED | release-generation | 原子切换、漂移修复和回滚测试 |
| GOV-004 | F57 | 能力包声明所有权、依赖、兼容、权限、资源、测试和停用行为 | EXPANDED | capability-registry | manifest 正负例与所有权冲突测试 |
| GOV-005 | PRD/F56 | 许可支持永久、订阅、宽限、受限运行和模块生命周期 | RETAINED | license | 四态、续期、撤销、停用保留数据测试 |
| GOV-006 | DOCX | 名称、图标、颜色、客户端包和打印模板可白标 | RETAINED | branding | 签名白标代发布与回退测试 |
| GOV-007 | F57 | 数据、附件、审计、配置和客户包可按开放版本格式完整导出 | EXPANDED | portability | 空环境导入、校验和对账测试 |
| GOV-008 | F57 | 每次权威命令记录配置代、权限策略版本、工作流版本、能力包版本、客户端版本和幂等标识 | EXPANDED | authority-kernel | 缺版本拒绝、跨代、重放与审计关联测试 |
| GOV-009 | F57 | 当前状态、不可变业务事实、审计记录和 Outbox 事件必须在同一 PostgreSQL 事务原子提交 | EXPANDED | authority-kernel | 四写原子性、崩溃点与 Outbox 重放测试 |
| GOV-010 | F57 | 所有延期能力必须进入 exact registry，固定允许接口、禁止 route/module/menu/claim 和激活 ADR/证据；未激活时不可安装、不可见、不可调用、不可绕行、不可宣称 | EXPANDED | capability-registry | exact-set、禁用面、营销声明、低代码/插件绕行与激活门测试 |

## 4. 主数据、客户与商机

| ID | 来源 | 要求 | 处置 | 所有者 | 验收 |
|---|---|---|---|---|---|
| MDM-001 | PRD/F57 | 法人、组织、团队、用户、设备和身份提供者版本化 | EXPANDED | platform-core | 生命周期、法人隔离与历史引用测试 |
| MDM-002 | DOCX/PRD | 客户、供应商、物料、产品、单位、仓库和价目表有唯一所有者 | RETAINED | mdm | 唯一性、法人和 owner contract 测试 |
| MDM-003 | PRD | 主数据支持草稿、审批、启停和版本，业务事实不物理删除 | RETAINED | mdm | 状态机、引用阻断和归档测试 |
| MDM-004 | F57 | 编号、重复检测、合并及合并审计 | EXPANDED | mdm | 并发编号、重复和合并回滚测试 |
| MDM-005 | PRD | 历史交易引用历史快照，不因主数据更新而变化 | RETAINED | mdm | 版本快照回归测试 |
| MDM-006 | DOCX/F57 | Excel/CSV 使用同一权限、校验、审批、幂等和逐行错误 | EXPANDED | import-export | 错行、重复、并发变化和公式注入测试 |
| CRM-001 | DOCX/F57 | CRM 通过授权投影呈现 MDM 拥有的客户、联系人、地址与关联企业，呈现 invoice/finance 拥有的开票与信用事实；CRM 只拥有关系和交互视图，不复制或改写这些权威事实 | EXPANDED | crm | owner-command、跨域投影、字段范围、脱敏和禁止重复权威事实测试 |
| CRM-002 | DOCX/F57 | 客户 360 时间线包含合同、订单、资金、投诉、设备和服务 | EXPANDED | crm | 聚合授权与来源证据测试 |
| CRM-003 | PRD/F57 | CRM 拥有商机和追加式跟进事实；商机按 `DRAFT/QUALIFYING/SOLUTION/COMMERCIAL/WON/LOST/CANCELLED` 冻结状态机分阶段处理赢、输、取消和重开，并通过类型化命令把成交意向交给报价、合同或订单能力 | EXPANDED | crm | 字段/状态机、跟进历史、多人归属、重开、移交幂等与来源追踪测试 |
| CRM-004 | DOCX/F57 | CRM 提供重复、合并、风险和投诉的统一操作入口/投影；重复检测与合并命令由 mdm 拥有，投诉受理与处置事实由 service 拥有，CRM 不建立第二份客户或投诉权威记录 | EXPANDED | crm | owner 路由、合并冲突/审计、投诉联动和重复权威事实阻断测试 |
| CPQ-001 | PRD/F57 | CPQ 独立拥有报价及不可变版本、审批/签发/过期/撤回/接受状态和向合同或订单的 exact-once 转换；首版不接受部分接受，须生成新版本；CRM 只能通过公开命令消费报价结果 | EXPANDED | cpq | 全状态机、有效期边界、价格越权、并发版本、撤回/重复转换与 owner-contract 测试 |

## 5. 合同与销售

| ID | 来源 | 要求 | 处置 | 所有者 | 验收 |
|---|---|---|---|---|---|
| CLM-001 | DOCX | 合同记录关键条款、收付款计划和附件 | RETAINED | clm | 必填、版本、金额和附件测试 |
| CLM-002 | DOCX/PRD | 支持模板、条款库、评论、版本、签章及签署证据 | RETAINED | clm | 模板版本与签署文件验证测试 |
| CLM-003 | DOCX/F57 | 合同支持审批、会签和职责分离 | EXPANDED | clm | 自审拒绝、节点变化和重认证测试 |
| CLM-004 | DOCX/PRD | 合同生效幂等生成订单、采购需求、项目、收款和交付责任 | RETAINED | clm | 重复激活与同事务/Outbox 测试 |
| CLM-005 | PRD/F57 | 合同变更只改变未履行义务，已履行事实不可变 | EXPANDED | clm | 变更影响和历史事实测试 |
| CLM-006 | DOCX | 支持履约、提醒、续签、到期和合同合并 | RETAINED | clm | 计时、版本和合并测试 |
| CLM-007 | F57 | 合同终止形成影响处置清单，全部闭环后才能结束 | EXPANDED | clm | 下游影响、补偿和重开测试 |
| SAL-001 | DOCX/F57 | 订单来源闭集为合同版本、已批准报价版本或经审批人工权威，数据库 exact-one；无合同时冻结完整商业快照，三来源均形成交付、开票、收款、退换和售后依据 | EXPANDED | sales | 三来源 exact-one、权限/信用/审批、快照、全下游闭环和幂等测试 |
| SAL-002 | DOCX | 订单包含产品、数量、价格、税、交付日期、地址和类型 | RETAINED | sales | 类型与金额重算测试 |
| SAL-003 | DOCX/PRD | 提交校验价格权限、合同、库存、交期和信用 | RETAINED | sales | 竞态和锁序测试 |
| SAL-004 | DOCX/F57 | 变更、拆分、合并和取消版本化并按风险审批 | EXPANDED | sales | 版本、影响面和审批测试 |
| SAL-005 | DOCX/PRD | 支持部分交付、退货和换货 | RETAINED | sales | 分批、退换与财务库存勾稽测试 |
| SAL-006 | DOCX/PRD/F57 | 第一阶段必须完成 `STANDARD` 与 `DROP_SHIP` 两类销售闭环认证 | EXPANDED | sales | 两种状态机、库存/代发、退换货、财务勾稽与认证证据测试 |
| SAL-007 | PRD/F50 | 信用占用不重复计算并在履约、开票、收款、退货时释放重检 | RETAINED | finance | 并发额度与 gross exposure 测试 |
| SAL-008 | DOCX/PRD/F57 | `CONSIGNMENT`、`SUBSCRIPTION`、`LEASE` 完整闭环延期；当前交付类型化 provider seam，未逐类型认证不得启用或宣称可用 | DEFERRED_WITH_INTERFACE | sales | 三类禁用、provider contract、未知类型拒绝与未来兼容测试 |

## 6. 采购、供应商与库存

| ID | 来源 | 要求 | 处置 | 所有者 | 验收 |
|---|---|---|---|---|---|
| PROC-001 | DOCX/F57 | 采购需求来源闭集为合同、订单、项目、库存、人工或外部生产；合并/拆分保留逐来源分配且数量守恒，外部请求按 provider key 幂等 | EXPANDED | procure | 六来源、合并/拆分守恒、重复外部请求和来源追踪测试 |
| PROC-002 | DOCX | 采购需求支持合并、拆分和来源追踪 | RETAINED | procure | 数量守恒和追踪测试 |
| PROC-003 | DOCX/F57 | RFQ、供应商报价版本、统一口径评估、部分/多供应商授标和重新授标按冻结状态机执行；迟交/撤回、无报价、单一报价、平局和例外授标均有单义失败/审批结果 | EXPANDED | procure | 截止/版本、比价权限、数量守恒、SoD、例外/部分授标、供应商拒绝和重新授标测试 |
| PROC-004 | DOCX | 采购单支持审批、版本、分批订购和供应商确认 | RETAINED | procure | 状态机与供应商命令测试 |
| PROC-005 | DOCX/PRD | 收货支持多批次、超收、短收和拒收 | RETAINED | inventory | 数量、权限和重复收货测试 |
| PROC-006 | DOCX/PRD/F50 | 采购退货和代发追溯到需求、订单、库存和财务事实 | RETAINED | procure | 全链勾稽与补偿测试 |
| PROC-007 | DOCX/PRD | 支持付款申请和预付款 | RETAINED | finance | 上限、审批与冲正测试 |
| PROC-008 | DOCX | 供应商主档拥有资质、价格、交期、质量和风险；采购只通过供应商公开能力消费 | RETAINED | mdm | 版本、到期和评价测试 |
| PROC-009 | PRD/F57 | 采购域只通过 `POR-002` 的五项供应商门户白名单消费外部协作，不另建第二套门户事实 | RETAINED | procure | canonical owner、跨供应商/法人和白名单外命令拒绝测试 |
| INV-001 | PRD/F57 | 仓库和库存按法人隔离 | RETAINED | inventory | RLS 与复合外键测试 |
| INV-002 | PRD | 同时维护数量账、金额账和当前库存 | RETAINED | inventory | 重放与结存勾稽测试 |
| INV-003 | PRD | 支持批次、序列号和扫码追溯 | RETAINED | inventory | 唯一性与全链追踪测试 |
| INV-004 | PRD | 收货、发货、销售退货、采购退货和金额调整产生不可变事件 | RETAINED | inventory | 五类事件与更正链测试 |
| INV-005 | PRD | 禁止负库存；重复请求不重复移动 | RETAINED | inventory | 并发扣减与幂等测试 |
| INV-006 | PRD | 提供可用量计算和补货建议 | RETAINED | inventory | 计算口径与采购需求测试 |
| INV-007 | PRD | 保留库存估价、库存价值、收发存与期末价值报表 | RETAINED | costing | 金额账与报表对账测试 |

## 7. 经营财务

| ID | 来源 | 要求 | 处置 | 所有者 | 验收 |
|---|---|---|---|---|---|
| FIN-001 | DOCX/PRD | 维护现金和银行账户并保护敏感账号 | RETAINED | finance | 加密、末四位和重新认证测试 |
| FIN-002 | DOCX | 开票申请记录内容、比例、金额、日期并审批 | RETAINED | invoice | 累计比例与审批测试 |
| FIN-003 | DOCX/F50 | 销项和进项发票保存头、行、税额、号码和附件 | RETAINED | invoice | 多税率、号码唯一和金额汇总测试 |
| FIN-004 | F50 | 支持作废、红冲、部分红冲和重开 | RETAINED | invoice | 有效余额、LIFO 和历史切片测试 |
| FIN-005 | DOCX/F50 | 收付款可分次并与合同、订单、发票多对多核销 | RETAINED | finance | 多关系、并发上限与释放测试 |
| FIN-006 | F50 | 支持预收、预付及后续自动或人工分配 | RETAINED | finance | 挂账、分配与冲正测试 |
| FIN-007 | F50 | 支持客户退款、供应商返款和资金事实冲正 | RETAINED | finance | RELEASE 链与资金不重复测试 |
| FIN-008 | DOCX/PRD | 提供应收、应付和账龄 | RETAINED | finance | 历史期间与当前余额测试 |
| FIN-009 | F57 | 外部结果未知、重复回调和差异进入对账处置 | EXPANDED | recon | timeout-after-success 与重复回调测试 |
| FIN-010 | DOCX/F57 | 按合同、订单、客户、项目汇总现金流、成本和毛利 | EXPANDED | reporting | 指标来源和下钻测试 |
| FIN-011 | F50/F57 | 平台保留不可变、平衡的内部经营分录、经营科目映射、试算、子账对账和经营期间永久锁定；锁定后不得重开 | EXPANDED | ledger | 追加、更正、双分录、试算、永久锁定与绕过拒绝测试 |
| FIN-012 | F57 | 法定科目/凭证账簿、税务申报、工资和法定年结由类型化连接器对接专业系统 | DEFERRED_WITH_INTERFACE | integration | connector contract、失败、导出对账和内部事实不变测试 |
| FIN-013 | F50/F57 | 经营期间不等同法定会计期间；锁定后永不重开，迟到事实顺延记入下一开放经营期间并保留原业务日期、顺延依据和更正链 | EXPANDED | ledger | 永久锁定、迟到顺延、原日期保留、更正链和专业系统期间不一致测试 |

## 8. 售后、投诉、设备与项目

| ID | 来源 | 要求 | 处置 | 所有者 | 验收 |
|---|---|---|---|---|---|
| SRV-001 | DOCX | 投诉记录来源、分类、责任、受理和响应目标 | RETAINED | service | SLA、升级与法人测试 |
| SRV-002 | DOCX/PRD | 设备以序列号/批次关联客户、订单、合同和保修 | RETAINED | service | 唯一性和保修判定测试 |
| SRV-003 | DOCX/F57 | 工单类型闭集为安装、维修、巡检、保养和技术支持，共用冻结生命周期并按类型配置不可跳过的关闭证据 | EXPANDED | service | 五类型全状态机、等待/取消、证据 exact-set 与重开测试 |
| SRV-004 | F57 | 按能力、位置、负载、SLA、回避和职责分离自动派工 | EXPANDED | work-allocation | 解析、无候选、改派和撤权测试 |
| SRV-005 | DOCX/F57 | 记录时间线、附件、照片、扫码和客户签字 | EXPANDED | service | 移动、离线和证据测试 |
| SRV-006 | F57 | 工单记录配件预留/领用/退回/损耗、工时和费用；库存事实由库存拥有，经营成本由财务/成本所有者聚合，服务不得直写他域事实 | EXPANDED | service | 库存守恒、成本来源、审批/权限、取消与返还测试 |
| SRV-007 | DOCX/PRD | 退换修联动销售、库存和财务事实 | RETAINED | service | 组合链路与补偿测试 |
| SRV-008 | F57 | 记录根因、纠正措施、复发、满意度和回访；严重投诉/重复故障的 CAPA 与回访是关闭谓词，复发或证据撤回自动重开 | EXPANDED | service | CAPA/回访强制、复发、证据撤回、闭环和重新开启测试 |
| SRV-009 | F57 | 权益优先级固定为有效服务合同、保修、获批收费服务；周期维护按签名规则幂等生成，暂停/漏跑/时钟异常有补发或异常任务 | EXPANDED | service | 权益冲突、周期/续约、停用设备、漏跑、时钟异常和幂等生成测试 |
| SRV-010 | F57 | 只有业务闭环登记中该工单类型的义务和证据全部满足才能关闭；下游异常、退货、证据撤销或复发按登记重开，服务拥有工单事实，自动化只执行版本化闭环规则 | EXPANDED | service | closure registry exact predicate、合法人工终止与重开测试 |
| PRJ-001 | PRD | 项目包含里程碑、任务、责任人和日期 | RETAINED | project | 状态与任务测试 |
| PRJ-002 | DOCX/PRD | 项目关联合同并产生采购需求和交付证据 | RETAINED | project | 来源和交付联动测试 |
| PRJ-003 | F57 | 记录项目成本聚合、收款节点和风险；风险状态闭集为开放、缓解、监控、接受、关闭，`TRANSFER` 仅是 response strategy，风险转移可作为证据充分时的关闭原因；严重度驱动升级，验收和收款节点引用权威合同/财务事实 | EXPANDED | project | 财务聚合、风险状态/策略/升级、里程碑验收、收款节点和重开测试 |
| PRJ-004 | F57 | 完整 WBS、资源工时、预算变更和挣值分析暂缓 | DEFERRED_WITH_INTERFACE | project | 扩展契约兼容测试 |

## 9. 报表、自动化与动态权限

| ID | 来源 | 要求 | 处置 | 所有者 | 验收 |
|---|---|---|---|---|---|
| REP-001 | PRD | 提供收入、成本、利润和交付驾驶舱 | RETAINED | reporting | 来源、下钻和权限测试 |
| REP-002 | DOCX/F57 | 提供账龄、采购、服务、闭环和自动化质量报表；每个新增指标登记稳定 MetricID、公式 AST/版本、分子分母、时间窗、去重、迟到/重开/取消/Unknown 口径 | EXPANDED | reporting | 指标注册 exact-set、边界样本、历史重算和一致性测试 |
| REP-003 | F57 | 所有指标可解释公式并下钻到权限允许的来源证据；权限裁剪前后、无数据、Unknown 和未分摊差异不得产生误导 | EXPANDED | reporting | lineage、权限裁剪、无数据/Unknown 与证据下钻测试 |
| REP-004 | DOCX | 支持自定义指标、报表、仪表盘和打印模板 | RETAINED | reporting | 发布代与回滚测试 |
| AUT-001 | F57 | 自动化建模目标、义务、步骤、效果、证据、闭环和周期；每个业务 ObjectiveKind 必须登记触发、义务、责任、效果、证据、关闭、超时、补偿、终止和重开 exact contract | EXPANDED | automation | 业务闭环 registry、长链状态模型和逐链 predicate 测试 |
| AUT-002 | F57 | 支持检查点、幂等、重试、退避、租约、补偿和事故箱 | EXPANDED | automation | 断电、超时、抢占和恢复测试 |
| AUT-003 | F57 | 运行固定流程版本；升级时继续、补偿或重启 | EXPANDED | automation | 新旧版本并行和排空测试 |
| AUT-004 | F57 | 任务按能力动态找人，不依赖固定岗位；撤权/离职时精确分支为：`ASSIGNED` 未接受则终结原 assignment 并进入 `RESOLVING`，`ACCEPTED/IN_PROGRESS` 先写 checkpoint、阻止新 effect 并进入 `REASSIGNING`，`WAITING` 且外部效果 `Unknown` 时原 Objective 先进入 `RECONCILING`；全程保持 SLA、草稿裁剪和 SoD 历史，重新分配或升级不得扩大权限 | EXPANDED | work-allocation | 三分支、权限、负载、SLA 连续、草稿归属、SoD、无人候选与升级测试 |
| AUT-005 | F57 | 上游事实改变可传播影响并重新开启下游目标 | EXPANDED | automation | 退货、退款、撤回和重开测试 |
| AUT-006 | F57 | 流程发布前编译、模拟、故障注入、灰度和回滚 | EXPANDED | release-generation | release gate 测试 |
| AUT-007 | F57 | 外部效果 `Unknown` 的人工处置只能是 `CONFIRMED_SUCCEEDED`、`CONFIRMED_NOT_EXECUTED`、`CONFIRMED_COMPENSATED` 或 `UNRESOLVED_CONTAINED`；按风险要求独立证据、重新认证和 SoD，禁止单人制造成功事实，后续相反证据必须冲突、重开并追偿 | EXPANDED | recon | 四决策 exact-set、证据/双人、重复付款/发送、迟到回调冲突、重开和追偿测试 |
| AUTH-001 | DOCX/F57 | 主体、能力、范围、条件、期限、设备、金额和状态共同授权 | EXPANDED | authz | 决策表、属性和负向测试 |
| AUTH-002 | DOCX/F57 | 岗位、角色和五类人物仅是授权模板 | EXPANDED | authz | 无角色直接能力 grant 测试 |
| AUTH-003 | F57 | 支持临时授权、委托、自动过期和撤回 | EXPANDED | authz | delegation ceiling 和过期测试 |
| AUTH-004 | F57 | 支持职责分离、风险分级审批和受控破窗 | EXPANDED | authz | maker-checker 与 break-glass 测试 |
| AUTH-005 | DOCX/F57 | 每次服务操作重新鉴权并执行行/字段过滤 | EXPANDED | authz | RLS、projection 与撤权测试 |
| AUTH-006 | F57 | 权限可解释、可模拟并预览发布影响 | EXPANDED | authz | explanation 与 before/after diff 测试 |
| AUTH-007 | F57 | 高风险 exact-set 固定覆盖主数据合并、合同签署/签章/生效/重大变更/终止、价格/信用越权、订单取消、供应商准入/采购单发出/收货例外、库存调整、开票/红冲、付款/退款/银行账户、经营分录更正/期间锁定/迟到顺延例外、敏感导出、授权/委托/破窗、配置/包/schema/迁移、法律保留释放/数据处置、信任根/许可/provider/批准时间源、密钥/备份/恢复、远程支持授权和 authority 提升 | EXPANDED | authz | exact-set、风险等级、重新认证、双人/职责分离和遗漏拒绝测试 |

## 10. 客户端、连接器、AI 与能力包

| ID | 来源 | 要求 | 处置 | 所有者 | 验收 |
|---|---|---|---|---|---|
| CLI-001 | F57 | Workbench 以任务、目标、异常和闭环为首页 | EXPANDED | workbench | 跨模块任务场景测试 |
| CLI-002 | DOCX/F57 | Windows、macOS、iOS、Android 结果和权限等价、交互自适应 | EXPANDED | workbench | 四平台契约套件 |
| CLI-003 | F57 | 移动端重点支持审批、现场、扫码、拍照和离线草稿 | EXPANDED | workbench | 设备能力与弱网测试 |
| CLI-004 | F57 | 缓存最小加密、设备可撤销、高风险设备受限 | EXPANDED | device-security | 丢失、Root/Jailbreak 和撤销测试 |
| CLI-005 | F57 | 重连提交业务意图，服务器重验权限、规则、版本和幂等 | EXPANDED | sync | 双设备冲突与撤权测试 |
| CLI-006 | PRD | 桌面键盘、屏幕阅读、焦点和可访问性满足企业基线 | RETAINED | workbench | WCAG AA/键盘验收 |
| CLI-007 | F57 | Tauri 先过四平台硬门；失败整套回退 Flutter，禁止双栈 | EXPANDED | client-shell | PoC 决策证据 |
| CLI-008 | PRD/F57 | 四端包按客户和 audience 使用客户控制的签名/商店/MDM/离线仓库，绑定 digest、最低版本、撤销和更新回滚；移动端禁止动态下载可执行扩展，商店失败只能按合同记录差异后回退 Web/PWA | EXPANDED | client-distribution | 四平台签名、安装、更新、撤销、失窃证书、最低版本和 PWA 差异测试 |
| CLI-009 | F57 | Workbench 只通过签名 `employee_api_origin` 的版本化 HTTPS employee API 使用会话、命令、查询、任务流、文件和 schema；四端共用同一 IDL、错误、幂等、版本和兼容契约 | EXPANDED | employee-api | 四端 contract、重定向/代理/错 origin、会话撤销、游标、版本、分片和错误等价测试 |
| INT-001 | F57 | 数据库、文件、身份、邮件、通知、签章、杀毒、备份、密钥和监控使用类型化适配器 | EXPANDED | adapter-registry | provider contract suites |
| INT-002 | DOCX/F57 | 支持 REST、Webhook、MCP、Excel、Word、PDF 和 CSV | EXPANDED | integration | 格式/协议正负例测试 |
| INT-003 | F57 | 第一阶段必须认证本地文件、Office 格式、REST/Webhook/MCP、SMTP、AD/LDAP；企业微信、钉钉、飞书、M365、WPS、银行、税务和签章厂商作为签名能力包，逐 provider 取证后启用 | EXPANDED | integration | core provider exact-set、manifest、权限、失败与逐 provider conformance 测试 |
| INT-004 | F57 | 自建服务与已有服务实现同一契约并可替换 | EXPANDED | adapter-registry | provider swap 与 drain 测试 |
| INT-005 | F57 | 核心交易使用命令和事实；MCP 不成为数据库或交易权威 | EXPANDED | authority-kernel | 架构依赖与直连阻断测试 |
| AI-001 | F55/F57 | 模型、供应商、工具和提示可插拔；外部默认关闭 | EXPANDED | ai-governance | provider 与 deny-default 测试 |
| AI-002 | F57 | 外发最小化、脱敏、授权、审计；最高密级不得外发 | EXPANDED | ai-governance | field allowlist 与 egress 测试 |
| AI-003 | F57 | AI 只能通过能力执行获准低风险操作 | EXPANDED | ai-governance | 越权工具和审批测试 |
| AI-004 | F57 | 高风险必须审批；确定性流程不依赖 AI | EXPANDED | ai-governance | AI 失效主链继续测试 |
| AI-005 | F57 | 本地模型延期，接口、权限和审计契约立即冻结 | DEFERRED_WITH_INTERFACE | ai-provider | null provider 与 future compatibility 测试 |
| PKG-001 | F57 | 同时支持厂商信任根和客户信任根 | EXPANDED | package-trust | 双根、撤销和替换测试 |
| PKG-002 | F57 | 能力包以 closed-schema 声明权限、依赖、网络、资源、测试、SBOM、受控迁移和回滚；迁移只能由可信迁移器执行 | EXPANDED | capability-registry | manifest、SBOM、迁移权限/兼容/失败回滚正负例测试 |
| PKG-003 | F57 | WASM 与受 Job Object 约束的 Windows worker 可用；第一阶段必须实现 `HOST_CAPABILITY_CONDITIONAL` 的 Hyper-V-isolated Windows container adapter 与 conformance，但只有 host feature、nesting、容量和安全证据全通过才可激活，P340 32GB 默认禁用；内核仍走维护升级 | EXPANDED | package-runtime | WASM/worker lifecycle、container adapter conformance、证据门禁、默认禁用、排空和内核回滚测试 |
| PKG-004 | DOCX/F56/F57 | 安装、启停、升级始终保留数据、审计和导出 | RETAINED | package-runtime | 停用、重启和完整导出测试 |

## 11. 公共平台、定制、门户、身份与 MCP

| ID | 来源 | 要求 | 处置 | 所有者 | 验收 |
|---|---|---|---|---|---|
| PLT-001 | DOCX/F57 | 站内通知为核心能力；邮件及企业消息渠道通过可替换 provider，失败形成降级窗口且不丢业务责任 | EXPANDED | notify | 模板、收件范围、重复、渠道失败和补发测试 |
| PLT-002 | DOCX/F57 | 全局搜索只索引获准对象/字段，结果与聚合执行当前行/字段权限且撤权后及时消失 | EXPANDED | search | 越权推断、撤权、重建和 generation 测试 |
| PLT-003 | DOCX/F57 | 通用审批支持串行、并行会签、职责分离、SLA、升级、委托、超时和有证据关闭 | EXPANDED | approval | 多链、无人、超时、委托和重放测试 |
| PLT-004 | DOCX/F57 | 通用附件、评论、版本、模板和打印使用统一安全存储、扫描、签名、权限与保留策略 | EXPANDED | file | 隔离、病毒、版本、签名、保留和跨域引用测试 |
| CUS-001 | DOCX/F57 | 客户可定义对象、字段、关系、索引、编号、校验、搜索视图和规则；编译为 `ext` 真实关系结构，核心 schema 受保护 | EXPANDED | meta | 编译、编号并发、注入、锁、迁移、RLS 和 rollback 测试 |
| CUS-002 | DOCX/F57 | 客户可定义首页、表单、列表、菜单、通用视图、任务视图、看板和签名自适应 UI schema | EXPANDED | meta | 四端渲染、恶意 schema、权限、首页/通用视图和回退测试 |
| CUS-003 | DOCX/F57 | 客户可定义自动化、指标、报表和打印模板，全部版本化、模拟、审批和回滚 | EXPANDED | meta | compile/simulate/publish/rollback 测试 |
| CUS-004 | F57 | 数据、权限、流程、UI、报表、模板、provider 和包只能以同一不可变签名 generation 发布 | EXPANDED | release-generation | 混合代拒绝、desired/observed、崩溃和回滚测试 |
| POR-001 | F57 | 客户门户可查看报价/合同/订单/交付/发票/收款/设备/工单和批准文档，可确认交付/验收、提投诉/服务请求及补证；禁止修改金额、发起付款、权限和配置权威动作 | EXPANDED | portal | 客户隔离、exact allowlist、附件和高风险拒绝测试 |
| POR-002 | PRD/F57 | 供应商门户只允许采购单/交期确认、ASN、发票上传、对账查看和自有资料更新 | RETAINED | portal | 供应商隔离、五能力 exact-set 和越权测试 |
| POR-003 | F57 | 客户/供应商外部门户共用 audience-bound 邀请、主体和 party-binding 生命周期；每个 binding 精确绑定 audience、法人、customer/supplier party 与联系人，MFA、激活、暂停、恢复命令序列、设备/会话撤销、关系终止和 party 合并迁移均逐绑定授权，禁止 `RECOVERY_PENDING` 等未登记状态 | EXPANDED | portal-identity | 邀请重放、跨 audience/party/法人、联系人离职、关系终止、被盗账号、SUSPENDED 内恢复、合并迁移和即时撤销测试 |
| IDP-001 | F57 | 本地账号和双人破窗应急账号始终保留，外部身份不可使客户失去本地控制 | EXPANDED | identity | 外部故障、破窗、到期、轮换和审计测试 |
| IDP-002 | F57 | AD/LDAP 是第一阶段必须认证的企业身份 provider，映射不直接生成权限，仍由动态 grant 决定 | EXPANDED | identity | TLS、组映射、重复身份、禁用和撤权测试 |
| IDP-003 | F57 | 当前交付 OIDC/SAML 的统一签名 provider seam、provider-specific conformance harness 和禁用门；只有具体身份提供商包已安装且 metadata、issuer/audience、签名、重放、注销和驻留证据逐项通过后才启用，未安装/未取证保持关闭，不代表任何具体 IdP 已预装 | EXPANDED | identity | metadata、issuer/audience、签名、重放、注销、驻留和 provider-disabled 测试 |
| MCP-001 | F55/F57 | MCP manifest 采用 closed schema、签名、版本和 generation 绑定；调用只取得短期、调用级资源 handle | EXPANDED | provider | manifest、过期 token、旧代、撤权和 secret 不落盘测试 |
| MCP-002 | F57 | MCP 工具可扩展，但逐项声明能力、对象/字段、网络、文件、密钥、资源、风险和审批；默认全拒绝 | EXPANDED | provider | undeclared access、工具越权、资源耗尽和 egress 测试 |
| MCP-003 | F57 | MCP 写工具只能调用类型化业务命令；外部结果丢失进入 `Unknown` 并通过 provider 对账，不盲重试 | EXPANDED | provider | SQL/DB 阻断、重复请求、响应丢失和 reconcile 测试 |
| DBP-001 | F57 | 第一阶段只认证 PostgreSQL 16 为权威数据库；保留稳定 `AuthoritativeDatabaseProvider` seam，其他数据库只能作为外部数据源或类型化命令提供者。Task 11 sole owner/schema 固定五 strict root：19-field package lock、13-field install contract、4-field Event Log fixture、19-field Event Log coverage、17-field install readback；artifact set/scan contract/service-install evidence 逐层认证。`installed_files`↔SBOM/final-handle、防降级、九路径 SDDL→live DACL、四方 system identifier、typed `RUNNING` 与 SSD/HDD 分界 exact。GUC 固定 `max_connections=64\|reserved_connections=4\|superuser_reserved_connections=3`、safety=2；consumer `NORMAL\|RESERVED\|SUPERUSER` 分别校验五条预算与 role attributes，应用不能耗尽保留位。HBA 只证明 loopback `hostssl`+SCRAM；client `channel_binding=require`/协商由 authenticated probe 单独证明。`fsync_writethrough` 仅兼容性 pin；同文件 `fsync`/`fsync_writethrough` qualification 绑定卷/driver/cache，Task 15 再 exact-join P340 UPS/write-cache/flush/power-cut 才完成耐久性。日志 collector→stderr→HDD；Event Log coverage 闭合 provider registration、同 boot bookmark/record/time、零 clear/drop/gap、fixture ref/digest/complete execution 与零 token，缺失/截断/错配拒绝。禁 `initdb --waldir`、tablespace/reparse/trust/external CIDR/ambient override | EXPANDED | authority-database | 五 root 字段/schema/signature、package/SBOM/final-handle 双射与 anti-downgrade、SDDL/live DACL、SCM/start/RUNNING、四方 identifier、64/4/3/2 分类预算与 saturation、HBA/client-CB 分离、Event Log 完整覆盖负例、双 fsync 同文件资格及 Task-15 UPS/power-cut join、PITR、非 PG 权威启动拒绝、外部数据源隔离和 provider seam 测试 |
| OPS-001 | F57 | 服务器控制中心显示安全、备份、磁盘、容量、generation、包、自动化事故、恢复证书和永久降级状态 | EXPANDED | ops | 状态真实性、证据下钻、旧证书失效和无证据不绿测试 |

## 12. 安全、硬件与非功能

| ID | 来源 | 要求 | 处置 | 所有者 | 验收 |
|---|---|---|---|---|---|
| SEC-001 | F57 | Windows Server IPC 仅允许 first-instance、reject-remote 的 named pipe，DACL 精确限定对应 service SID 与 SYSTEM；双方校验 SID、进程、签名和 digest，frame 有界并绑定 nonce/epoch/generation 防重放，ACL 读回失败不得 ready；UEFI、外部启动、PXE 与 USB 介质窗口执行物理门禁，数据库不对客户端暴露 | EXPANDED | security | first-instance/remote 拒绝、DACL 读回、错 SID/进程/签名/digest、有界 frame、nonce/epoch/旧代/重放、UEFI/PXE/USB 窗口和数据库直连负向测试 |
| SEC-002 | F57 | 卷、字段、附件和备份加密，法人/用途独立密钥域 | EXPANDED | kms | 密钥隔离、轮换与恢复测试 |
| SEC-003 | F57 | 审计追加写、哈希验证并向服务器外写签名检查点 | EXPANDED | audit | 篡改检测与外部 checkpoint 测试 |
| SEC-004 | F57 | 勒索恢复同时具备服务器外连续只追加层、至少两块 distinct `media_id` 离线 HDD 和独立恢复材料。active-config 分别选择 signed `BackupTopologySigningTrustCurrentPointerV1` 与 current `BackupTopologyV1`；部署 bootstrap 固定独立 trust-manifest authority，pointer typed-load 唯一 `BackupTopologySigningTrustManifestV1`，manifest 固定 topology signer DN/SPKI/offline chain/revocation/checkpoint。私有 `BackupTopologyAuthorityV1` 只能由该 verified-current trust 构造，禁止 topology/storage/support/candidate/ambient、应用/备份恢复域或 ADR-0020 roster self-auth。topology exact-repeat trust refs、join current singleton-target `F57AuthorityStorageManifestV1` 与 cut/checkpoint/readback，并固定六角色、一台 off-host target、exact A/B、SPKI、live domains、双人 custody、保留和容量。clean install 仅允许空链 `INITIALIZING + INITIAL_POPULATION`；sequence 1 后进入 `BOOTSTRAPPING`，先完成 head 的 A/B 验证，再按 checked head 补足 minimum，闭合后才为 `HEALTHY/None`。current roots 只从 fresh `HEALTHY` 轮换，经单一 `TRANSITIONING` old-head+1 bridge，再以不得建 checkpoint 的 `BOOTSTRAPPING + CURRENT_ROOTS_ROTATION` 完成 A/B closure；健康前禁止第二轮换。全部 retained refs typed-load 为唯一连续 `BackupCheckpointV1` 链，current head exact-bind current trust/topology/storage tuple；四个非健康状态禁止 PITR、发布、恢复认证与启用。每次 install/checkpoint/PITR/activation/retry 都用新 challenge/session/object、<=300s expiry 和显式 checkpoint；closed support root 对 target 单签、介质 transition/观察双签，验证 mTLS、physical total/free、quota/reserve、partial/history、六角色权限负探针、一次性 just-written read、A/B disconnect/custody/health/no-recovery-material。未知即 `NON_SUPPRESSIBLE_RISK`。八条且仅八条介质边是 `BLANK→ENROLLED`、`ENROLLED→ACTIVE_APPEND`、`ACTIVE_APPEND→VERIFIED_DISCONNECTED`、`VERIFIED_DISCONNECTED→ROTATION_DUE`、`ROTATION_DUE→ACTIVE_APPEND`、`ACTIVE_APPEND→SEALED_VERIFIED`、`SEALED_VERIFIED→RETIRED_PENDING_DISPOSAL`、`RETIRED_PENDING_DISPOSAL→DESTROYED`；sealed 不可复写，destroyed 复用必须新 `media_id`/新 sequence-1 链 | EXPANDED | backup | independent trust pointer/manifest/topology signer、storage singleton join、initial-population bootstrap、current-roots single-bridge rotation、fresh preparation/explicit head、typed predecessor chain、A-B subset/latest/minimum/support closure、typed support kind/media/signer/quorum/domain、八边 sequence/predecessor/hash/sealed/destroyed、新 media-id、retention/capacity overflow、mTLS/freshness、六角色权限、partial flood/quota/reserve、两块介质/窗口外断开、污染点与洁净恢复测试 |
| SEC-005 | F57 | 离线更新经签名、SBOM、模拟、审批、原子切换和回滚 | EXPANDED | release | 伪造、降级和 rollback 测试 |
| SEC-006 | F57 | 默认零出站、无后门、无永久远程通道；支持会话按 `REQUESTED→APPROVED→READY→ACTIVE` 推进并以 `CLOSED/REVOKED/EXPIRED/FAILED_CONTAINED` 之一终结，绑定人员、工单、origin、对象/字段/动作和最长 4 小时，到期撤销并完整审计 | EXPANDED | support-security | 永久通道扫描、网络/字段/动作越界、MFA/SoD、到期/撤权、失败隔离、凭据回收和诊断脱敏测试 |
| SEC-007 | F57 | 版本化保留、法律保留、隐私处置和销毁按 hold>不可覆盖底线>合同/法规>普通保留优先级；双人释放，附件/备份 pin 传播，财务/审计不可改写，恢复后重放 tombstone 防数据复活 | EXPANDED | lifecycle | hold 优先级/阻断、双人释放、假名化/crypto-erasure、附件/备份传播、失败恢复、restore 后处置和证明测试 |
| SEC-008 | PRD/F57 | 附件先写 HDD quarantine 并冻结 digest、长度、类型、结构与展开预算；verdict 必须绑定 digest、引擎、definition version/time 和签名后原子 publish；definition age 必须不超过 72 小时，stale、`UNKNOWN`、`SKIPPED` 或 timeout 一律继续隔离 | RETAINED | file-security | EICAR、zip-bomb、polyglot、TOCTOU、digest/长度变化、过期 definition、UNKNOWN/SKIPPED/timeout、签名失败与原子 publish 测试 |
| SEC-009 | F57 | 客户持有根密钥控制权；客户凭据密文在 HDD 产品秘密库，只有不可导出 wrapping handle 位于 TPM/HSM/KMS | EXPANDED | kms | 客户撤钥、跨法人/用途隔离、SSD 凭据扫描、轮换和恢复测试 |
| SEC-010 | F57 | 审计至少记录 actor、时间、设备、认证、授权依据、配置代/包/流程版本、before/after、审批、AI/MCP/插件、导入导出、密钥、备份、恢复和结果证据 | EXPANDED | audit | 字段非空/适用性、链完整、敏感裁剪和跨域关联测试 |
| SEC-011 | F57 | 权威节点 SSD 上的 Windows Event Log 只允许稳定事件码和随机 incident ID，禁止客户值、对象 ID、客户正文哈希及任何可关联客户的载荷 | EXPANDED | audit | Event Log schema、内容扫描、对象关联与故障路径泄漏测试 |
| SEC-012 | F57 | OS SSD 的 BitLocker 使用互斥的 `TPM_ONLY_UNATTENDED` 或 `TPM_PIN_ATTENDED`，当前 P340 基线固定前者；PIN 模式不得宣称无人值守恢复且须单独验收告警/有人值守 RTO。DATA_HDD protector exact-set 固定 `{PUBLIC_KEY,RECOVERY_PASSWORD}` 且 Windows fixed-data auto-unlock=false；trusted boot 后仅由独立 restricted-LocalSystem/no-network `EPF57DataVolumeUnlockBroker` 验证九个 pre-HDD locator、证书策略/链、bootstrap authority、TPM NV 与目标卷，并以现有 TPM-backed/nonexportable key、固定 thumbprint、本机 WMI `UnlockWithCertificateThumbprint` 解锁。clean-SSD/TPM-loss 只能在 admission closed 下经服务器外 48 位 recovery password 双人八步仪式创建新 key/certificate/PUBLIC_KEY protector、提升 epoch/NV、普通重启验收后移除旧 protector；不得从公开 metadata 重建旧私钥。BitLocker 强制 software encryption/XTS-AES-256 与 100% 加密；SSD 仅可保存受界限约束且可重新登记的 TPM-bound binding 和非秘密 locator/trust metadata；两卷 recovery password 独立、服务器外、双人保管并与应用 vault/备份恢复分域；首发业务卷只认证 NTFS，并记录 GPT、cluster、logical/physical sector 参数 | EXPANDED | endpoint-security | OS 启动模式边界、protector exact-set、PUBLIC_KEY broker/九 locator/WMI/restricted-token/no-second-instance、fixed-data auto-unlock=false、ordinary reboot、clean-SSD 八步重新登记、公开材料不可重建旧私钥、software/XTS-AES-256、100% gate、NTFS allowlist、ReFS/FAT/exFAT 拒绝、GPT/cluster/sector、durable flush/power-loss、跨域材料、双人恢复与盗钥负向测试 |
| SEC-013 | F57 | Windows 安全时间必须使用批准的 W32Time source 并记录 offset 与 last-sync；持续时间使用 monotonic clock，时钟回退或时间不可用时高风险命令和配置/包/流程发布 fail-closed | EXPANDED | time-security | 时间源 allowlist、offset/last-sync 证据、回退/漂移/不可用、单调计时与 fail-closed 测试 |
| SEC-014 | F57 | pre-DB secret broker 将每个 DEK 分别 wrap 给日常 TPM/HSM operational recipient 与独立离线 recovery recipient；日常域不可调用恢复接口，恢复固定采用 `PIV_SHAMIR_2_OF_3_V1`：生成 3 份由不同保管人持有的 share，任意 2 份才能脱离原机洁净恢复，任何单一保管人不能恢复。两份信封的认证上下文均精确绑定 deployment、legal entity、purpose、data-key id/version、wrap-context generation、recipient 和 envelope version，短期 handle 绑定 call、recipient、generation；禁止通用应用主密钥文件和 WinCred 客户秘密 | EXPANDED | secret-broker | 两用途 wrap、3 份 share 的三种有效 2-of-3 组合、单份/错误 share 失败、七项认证绑定逐项错配、handle 重放/错接收者/旧代、SSD/WinCred 扫描与洁净跨机恢复测试 |
| SEC-015 | PRD/F57 | 受管原生端强制服务端裁剪、水印、导出审批及按载体可证明的打印/剪贴板/share/受管文件控制；不合规设备降级只读或拒绝高密级，浏览器对无法强制项明示尽力边界；不得宣称阻止外部相机 | EXPANDED | device-security | 受管/不合规/Root/Jailbreak、截图/录屏、剪贴板/share、打印、下载失效、浏览器披露和外部相机诚实性测试 |
| SEC-016 | F57 | 每个 backup set 使用独立 backup DEK 和 recovery-only `BackupKeyEnvelopeV1`，固定 `PIV_SHAMIR_2_OF_3_V1` 三份加密 share、任意两份跨洁净主机恢复；AAD、算法、recipient、generation、轮换和 KAT 以 ADR-0024 为准 | EXPANDED | backup-kms | 三组合/单份失败、AAD/算法/recipient 错配、KAT、轮换、丢失、跨版本和跨洁净主机恢复测试 |
| SEC-017 | F57 | 安全事件/漏洞按检测、分级、隔离、服务器外证据、根除、凭据/证书/密钥轮换、known-clean 恢复、业务对账和再放行闭环；受影响 deployment 由 SBOM/generation 三态匹配，未知不得显示绿色 | EXPANDED | security-incident | compromised authority、外部证据、轮换、污染最新点、SBOM 匹配、通知、对账与再放行测试 |
| NFR-001 | F57 | 20 名活跃用户是 Workbench、客户门户和供应商门户合计容量基线而非硬登录上限；Control Center 使用独立保留资源并与其同时加载，过载渐进限流 | EXPANDED | capacity | 15+3+2 人、Control Center、登录/重连/附件 burst、混合负载与第 21 人测试 |
| NFR-002 | F57 | `HDD_STRICT` 约束权威节点上所有承载或可关联客户的持久数据/衍生数据只落 HDD；customer deployment manifest 本体只存 HDD packages，SSD 仅允许非秘密客户公钥 trust anchor、sealed BitLocker metadata 与 Event Log 窄例外，并用 TPM NV 的 revision+digest 防 manifest 回滚；HDD 紧急空间预分配且 ACL 保护，普通服务/插件不得占用或删除，仅红线流程可受控释放且随后重建；WAL 或审计不可写时 authority fail-closed；路径取证使用 final handle 加 volume/device ID，覆盖 reparse、junction、mount、ADS、hardlink 与 TOCTOU；终端仅允许 `CLI-004` 的最小加密可撤销非权威缓存 | EXPANDED | storage-policy | manifest/HDD packages、SSD public anchor allowlist、TPM NV revision+digest 回滚、reserve 预分配/ACL/占用删除负例、红线释放与重建、WAL/审计不可写、final-handle volume/device、reparse/junction/mount/ADS/hardlink/TOCTOU、终端缓存边界和写入探针 |
| NFR-003 | F57 | P340 低资源档关闭本地 AI，重报表单并发、后台节流 | EXPANDED | capacity | 资源优先级与降级测试 |
| NFR-004 | F57 | 当前单 HDD 是降级生产；上线需服务器外备份、两块离线介质、UPS、恢复演练，以及独立 current topology-signing trust + topology + storage tuple 下显式 checkpoint 的 fresh `HEALTHY/None` safeguard；空链 `INITIALIZING`、补代/轮换 `BOOTSTRAPPING`、bridge `TRANSITIONING` 或未知风险都不能启用 | EXPANDED | ops | go-live gate、五态 fail-closed、自举 minimum/A-B closure、roots 单 bridge 轮换、fresh HEALTHY current-head/support 证据 |
| NFR-005 | F57 | UPS 与两层备份是当前上线门；唯一 UPS schema 拥有 exact 16/20/21/28-field `UpsAdapterManifestV1\|UpsStatusReadbackV1\|UpsOutletCycleCommandV1\|UpsOutletCycleCommandAckV1` 与分离 status/control ports；`ep-platform-release` 直接依赖 nominal `ep-platform-ups-contract`，`ep-authority-kernel` 直接组合该 contract 与唯一 Windows 实现 `ep-adapter-ups-windows`，并锁定 dependency golden。Windows standard carrier 只可监测且 UNKNOWN 保真，最高安全档必须用候选绑定 signed vendor adapter；实现仅在 `EPAuthorityControl`，manifest `implementation_binary_ref` exact-join candidate kernel 与 live held binary，`configuration_projection` 是唯一部署选择且其正 generation/JCS digest 在 identity/status/command/ACK 一致；USB/网络/credential 按 service SID 和 exact endpoint 最小授权。时间固定 5s poll/15s status/86400s self-test/30s ACK；供应商 `provider_operation_id` 必须是 1..128 字节 canonical ASCII `[A-Za-z0-9][A-Za-z0-9._:-]{0,127}`，在 ACK 前 durable-bind `(identity,command ID,digest)`，空/别名/改变/跨 command 均 UNKNOWN。同 identity+command ID+digest 只能 query/adopt byte-identical ACK，不同 digest conflict，未知不重发，boot change 前无 composite ACK 不补造；低电量按停止长任务、持久 checkpoint、PostgreSQL 安全停机、Windows 关机固定顺序执行。后续升级为 RAID1、64GB、增强备份设备和可选暖备 | EXPANDED | ops | `f57_ups_adapter_contract\|f57_ups_command_reconciliation`、两 UPS package Windows 全目标与 locked direct-dependency tests、standard/vendor carrier、UNKNOWN、implementation binary/config projection/service/transport/credential 漂移、provider operation canonical/durable/cross-command、同 ID adopt/conflict、response-loss/boot-change、实际断电/通信断开、900 秒 crossing、PG/Windows 顺序关机与再认证测试 |
| NFR-006 | F57 | 正式服务器原生支持 Windows Server 2022，不依赖 WSL | RETAINED | authority-runtime | 安装、服务和补丁测试 |
| NFR-007 | F57 | 容量档绑定硬件、数据规模、配置代和实测基准 | EXPANDED | capacity | profile 签名与失效测试 |
| NFR-008 | F57 | 暖备必须证明单写 fencing；任何实测 RPO/RTO 只能按 `NFR-014` 登记为候选 SLO，未连续取证不得认证 | EXPANDED | availability | 分区、旧主隔离、提升、证书绑定和未认证展示阻断测试 |
| NFR-009 | F57 | IaaS 使用同一产品包、客户控制且位于获准境内区域；manifest 记录 provider/region/tenant control/vTPM/media/cache/snapshot 证据，HDD 介质或处理位置不能证明时失败关闭，禁止托管数据库/KMS/队列偷换 | EXPANDED | carrier | 物理/IaaS exact carrier、region/vTPM/media/cache/snapshot/provider-root 与托管组件负向测试 |
| NFR-010 | PRD/F57 | 错误码稳定且有关联 ID，不暴露堆栈、对象或秘密 | RETAINED | error-registry | error contract 测试 |
| NFR-011 | F57 | 故障矩阵覆盖断电、满盘、坏盘、锁、重复、网络和插件 | EXPANDED | verification | fault-injection suite |
| NFR-012 | F57 | 必须从洁净 Windows Server 和备份恢复完整权威环境 | EXPANDED | recovery | bare-metal recovery drill |
| NFR-013 | F57 | Kafka、Redis、Elasticsearch、Kubernetes、Temporal 集群和独立分析仓库不得成为第一阶段运行必需；新增 provider 不得破坏单节点运行性 | EXPANDED | architecture | 零外部中间件启动、依赖缺失、provider 停用和单节点回归测试 |
| NFR-014 | F57 | 暖备与备份 RPO/RTO 只能是绑定硬件、数据量、配置代和软件版本的候选 SLO；首次上线至少一次洁净全量恢复，只可标初始验证；滚动 90 天内 3 次连续全量成功、窗口无失败才可认证，最长有效 90 天；首次成功前失败保持 UNVERIFIED 并保存失败证据，取得 INITIAL/CANDIDATE/CERTIFIED 后的失败或绑定变化进入 INVALIDATED，只有 CERTIFIED 到期进入 EXPIRED | EXPANDED | availability | 初始/候选/认证/过期/失效状态、三次连续、首次前失败、窗口失败、绑定变化及 UI/API/导出承诺阻断测试 |
| NFR-015 | F57 | 启动自检必须验证 OS/DATA_HDD protector exact-set、Secure Boot、PCR、九个 pre-HDD locator、证书策略/链、bootstrap authority、TPM NV、broker restricted token/WMI 权限、explicit-thumbprint PUBLIC_KEY unlock readback 与 `fixed_data_auto_unlock=false`；必须分别演练 ordinary broker reboot unlock、TPM/OS SSD 损坏后的双人八步 recovery-password 重新登记及 recovery password 被盗 | EXPANDED | security-verification | 冷启动/PCR 漂移、locator/policy/certificate/WMI/pipe 权限负例、普通重启、clean-SSD 八步新 key/certificate/protector + epoch/NV、旧 protector 移除顺序、盗钥与双人恢复演练 |
| NFR-016 | F57 | 首版交付 OS adapter seam、Windows Server 2022 安装/恢复证据、后继 LTSC 探针和签名迁移 playbook；Windows Server 2025/后继 LTSC 真实认证为 `DEF-010`，主流支持后新生产证书绑定补丁来源/风险接受/迁移排期，扩展支持结束前必须迁移 | DEFERRED_WITH_INTERFACE | authority-runtime | 2022 seam/playbook boundary、生命周期日期、补丁/风险/排期 gate、未来并行/原地迁移接口和误称已认证阻断测试 |
| NFR-017 | PRD/F57 | 首版生产数据、备份、日志、索引、审计、诊断、支持导出、provider 输入输出和可关联衍生数据只在中国大陆境内处理/持久化；地点未知、证据过期或跨境 endpoint 失败关闭 | EXPANDED | residency | deployment/provider/backup jurisdiction、境外 IaaS/日志/备份/支持、DNS/遥测最小化和证据过期测试 |
| NFR-018 | PRD/F57 | 首版唯一产品语言 `zh-CN`、经营币种 `CNY`、业务自然日/默认显示 `Asia/Shanghai`，时间戳 UTC 保存、持续时间 monotonic；多语言/币种/外汇/进出口/报关/信用证不可由低代码或插件启用 | RETAINED | localization | 文案/格式、CNY schema、UTC/Asia-Shanghai 边界、DST/跨日、禁用字段/能力和绕行负向测试 |

精确值补充：`DBP-001` 的日志投影固定为 `logging_collector=on`、`log_destination=stderr`、DATA_HDD `log_directory`、`log_filename=postgresql-%Y-%m-%d_%H%M%S.log`、`log_rotation_age=24h`、`log_rotation_size=100MB`、`log_truncate_on_rotation=off`、product event source `EnterprisePlatform.PostgreSQL16`、唯一 early fallback `PostgreSQL` 与 server eventlog destination off；零客户 token 必须由 typed coverage 证明两个 provider registration、同 boot start/end bookmark/record ID/time、零 clear/drop/unexplained gap、fixture ref/digest、expected=exercised>0、token=0、`coverage_complete=true`，而非单一 scalar。`SEC-004` 的 topology signer DN 固定为 `CN=EP F57 Backup Topology Authority,O=Enterprise Platform`，不可被候选 topology 或 support evidence 替代。

## 13. 明确的阶段边界

| ID | 能力 | 处置 | Canonical RequirementID | 所有者 | 当前必须交付的接口 | TestID |
|---|---|---|---|---|---|---|
| DEF-001 | 本地大模型实现 | DEFERRED_WITH_INTERFACE | `AI-005` | ai-provider | AI provider、授权、审计、资源和模型版本契约 | `T-F57-DEF-001` |
| DEF-002 | 完整 MRP/MES/高级排产 | DEFERRED_WITH_INTERFACE | `PROC-001` | procure | `ProcurementDemandProvider` 与外部生产连接器 | `T-F57-DEF-002` |
| DEF-003 | 大型 WMS/自动化立库 | DEFERRED_WITH_INTERFACE | `INV-001` | inventory | 库存命令、事件和仓储连接器 | `T-F57-DEF-003` |
| DEF-004 | 法定总账/税务/工资 | DEFERRED_WITH_INTERFACE | `FIN-012` | integration | 经营事实、导出、对账和专业系统连接器 | `T-F57-DEF-004` |
| DEF-005 | 完整 PPM/EVM | DEFERRED_WITH_INTERFACE | `PRJ-004` | project | 项目、里程碑、成本、收款节点和风险扩展契约 | `T-F57-DEF-005` |
| DEF-006 | 主主、双活、多写 | OUT_OF_SCOPE | `NFR-008` | availability | 单写 authority discovery、warm-standby fencing | `T-F57-DEF-006` |
| DEF-007 | PostgreSQL 外权威数据库 | DEFERRED_WITH_INTERFACE | `DBP-001` | authority-database | `AuthoritativeDatabaseProvider` seam；第一阶段仅 PG16 认证 | `T-F57-DEF-007` |
| DEF-008 | 任意原生 DLL 热注入 | OUT_OF_SCOPE | `PKG-003` | package-runtime | 签名内核维护升级及 WASM/Job Object worker；产品 container adapter 当前交付，仅具体 host activation 在证据不足时拒绝 | `T-F57-DEF-008` |
| DEF-009 | 寄售/订阅/租赁完整销售闭环 | DEFERRED_WITH_INTERFACE | `SAL-008` | sales | 三类类型化 sales provider seam 与禁用证据 | `T-F57-DEF-009` |
| DEF-010 | Windows Server 2025/后继 LTSC 实机认证 | DEFERRED_WITH_INTERFACE | `NFR-016` | authority-runtime | OS adapter seam、后继 LTSC 探针、签名迁移 playbook、补丁/风险/排期 gate；未来激活真实安装/迁移/恢复/回滚矩阵 | `T-F57-DEF-010` |
| DEF-011 | 通用 XML/SOAP/XSD 交换面 | DEFERRED_WITH_INTERFACE | `INT-002` | integration | 签名 provider codec seam；首版核心 exact-set 不含 XML，特定 XSD 逐 provider 认证且不得绕过 ImportProposal/typed command | `T-F57-DEF-011` |

## 14. 完整性结论

现行基线共有 **185** 个最终 `RequirementID`：**174** 个主需求和 **11** 个阶段边界需求。原始 DOCX 中的客户、合同、订单、采购、财务、投诉/售后、附件、权限、低代码、报表、品牌和插件要求均有稳定需求 ID；其 **32 个实质需求段**（不计标题、空段和新增状态说明）均在 §15 至少由一个 `RequirementID` 以精确 `DOCX ¶N` 直接引用，不再只依赖上级标题或旧 PRD 代指。现行 PRD 中更细的库存、发票、资金、核销、项目、设备和供应商门户规则被保留；客户端分发/协议、数据防外泄、生命周期、备份密钥、事件响应、驻留和本地化已提升为稳定需求；被 F-57 扩展或替代的旧限制已在本矩阵显式标记。

以下曾有歧义的范围已经冻结：

| 主题 | 现行答案 |
|---|---|
| 旧 97 项延期 | 默认继续延期；只有被 F-57 逐项提升的能力恢复 |
| 内部账 | 平衡经营分录、科目映射、试算、子账对账和经营期间控制当前交付；法定账簿/税务/工资/法定年结外接 |
| 销售类型 | `STANDARD`、`DROP_SHIP` 是第一阶段必须完成认证的目标；寄售、订阅、租赁只保留 provider seam |
| 客户门户 | 固定为 `POR-001` 查询/提交白名单，不含付款、金额、权限和配置权威动作 |
| 连接器 | 第一阶段必须认证 LocalFile、Office 格式、REST/Webhook/MCP、SMTP、AD/LDAP；证据通过前保持未认证/关闭，厂商连接器逐包取证 |
| 身份 | 本地账号/破窗和 AD/LDAP 是第一阶段认证目标；证据通过前不得宣称可用，OIDC/SAML 按 provider conformance 启用 |
| 高风险目录 | 以 `AUTH-007` exact-set 为准，新增类别必须先更新该需求和测试 |
| 平台定制 | 关系对象/字段/关系、UI、自动化、报表和模板都在当前范围，全部走签名 generation |
| 客户端技术 | Tauri 2 是默认实现，先执行硬门；证据失败只阻断客户端波次并整体改走 Flutter，不形成双栈 |

矩阵没有需要产品负责人继续选择的行。`DEFERRED_WITH_INTERFACE` 的实现边界和当前必须交付契约均已给出；现场证书、域名、provider 端点、证书主体、数据量、保留期和实测 RPO/RTO 属部署证据，缺失时对应能力保持关闭或未认证，不形成产品歧义。

## 15. 逐条来源、取代与裁决附表

本附表共有 **185** 行，与主矩阵及阶段边界中的最终 RequirementID 一一对应。五列按单个 ID 展开；`NONE` 仅用于没有旧句可取代或不依赖裁决，`SourceDataOwner[]` 永远非空。变更来源、取代、裁决或数据来源关系时必须同步更新本表。

| RequirementID | SourceClause[] | Supersedes[] | RulingID[] | SourceDataOwner[] |
|---|---|---|---|---|
| GOV-001 | [F57 §3.3「客户端权威规则」] | [NONE] | [RULING-AUTHORITY-01] | [authority-kernel; authz; release-generation] |
| GOV-002 | [F57 §3.2「Windows-only 服务器」] | [F55 §5「第五客户端」; F55 §2「固定九进程」] | [RULING-AUTHORITY-01; RULING-UX-01; RULING-PROCESS-01] | [authority-runtime] |
| GOV-003 | [F57 §5.1「签名配置代」] | [NONE] | [RULING-AUTHORITY-01] | [release-generation; capability-registry; authz; meta; automation; reporting; provider] |
| GOV-004 | [F57 §4.5「L3 业务能力包」] | [F56 §2.2「模块包能力闭集」] | [RULING-PKG-01] | [capability-registry] |
| GOV-005 | [PRD §0.4「产品与范围前提」; F56 §3「许可证终态」] | [NONE] | [NONE] | [license] |
| GOV-006 | [DOCX ¶62「品牌可定制」] | [NONE] | [NONE] | [branding] |
| GOV-007 | [F57 §2.2 F57-A13「完整可导出」] | [NONE] | [NONE] | [portability; platform-core; mdm; crm; cpq; clm; sales; procure; inventory; finance; invoice; ledger; service; project; reporting; automation; authz; file; audit; release-generation; capability-registry] |
| GOV-008 | [F57 §5.1「权威命令版本记录」] | [NONE] | [NONE] | [authority-kernel; release-generation; authz; automation; capability-registry; client-shell] |
| GOV-009 | [F57 §6.3「四项原子写」] | [NONE] | [NONE] | [authority-kernel] |
| GOV-010 | [F57 Client/Lifecycle §11「延期能力 exact registry」] | [旧延期口径「只按域名文字说明而无可执行禁用面」] | [RULING-BUSINESS-SCOPE-01] | [capability-registry] |
| MDM-001 | [PRD §2「主数据」; F57 §6.4「多法人」] | [NONE] | [NONE] | [platform-core] |
| MDM-002 | [DOCX ¶8「选择客户和产品」; DOCX ¶48「供应商资质、价格、交期、质量和风险」; PRD §2「主数据」] | [NONE] | [NONE] | [mdm] |
| MDM-003 | [PRD §2「主数据」] | [NONE] | [NONE] | [mdm] |
| MDM-004 | [F57 §6.2「关系结构编译器」; F57 Business Execution §2.1「客户、联系人和商机边界」] | [NONE] | [NONE] | [mdm] |
| MDM-005 | [PRD §2「主数据」] | [NONE] | [NONE] | [mdm] |
| MDM-006 | [DOCX ¶57「数据可定制」; F57 §9.3「Excel」] | [NONE] | [NONE] | [import-export; mdm] |
| CRM-001 | [DOCX ¶11「客户历史信息」; F57 Business Execution §2.1「客户、联系人和商机边界」] | [NONE] | [NONE] | [crm; mdm; finance; invoice] |
| CRM-002 | [DOCX ¶11「客户历史合同、回款、投诉、设备和服务」; F57 §10.1「统一时间线」] | [NONE] | [NONE] | [crm; clm; sales; finance; invoice; service] |
| CRM-003 | [PRD §1.6「首版不做什么」; F57 §11.1「CRM」; F57 Business Execution §2「CRM 商机与跟进」] | [PRD §1.6「不含商机报价」; 旧草案「CRM/CPQ 共用所有权」] | [RULING-BUSINESS-SCOPE-01] | [crm] |
| CRM-004 | [DOCX ¶11「客户历史信息」; F57 §11.1「CRM」; F57 Business Execution §2.1「客户、联系人和商机边界」] | [PRD §1.6「不含客户合并」] | [RULING-BUSINESS-SCOPE-01] | [crm; mdm; service] |
| CPQ-001 | [PRD §1.6「首版不做什么」; F57 §11.1「CRM/报价」; F57 Business Execution §3「CPQ 报价与版本」] | [PRD §1.6「不含商机报价」; 旧草案「CRM/CPQ 共用所有权」] | [RULING-BUSINESS-SCOPE-01] | [cpq; crm; mdm] |
| CLM-001 | [DOCX ¶34「合同管理」] | [NONE] | [NONE] | [clm] |
| CLM-002 | [DOCX ¶34「合同管理」; PRD §3「合同、订单与信用」] | [NONE] | [NONE] | [clm; file] |
| CLM-003 | [DOCX ¶9「合同审批不可越权跳过」; DOCX ¶34「合同管理」; F57 §11.1「合同」] | [NONE] | [NONE] | [clm; approval; authz] |
| CLM-004 | [DOCX ¶34「合同管理」; PRD §3「合同、订单与信用」] | [NONE] | [NONE] | [clm] |
| CLM-005 | [PRD §3「合同、订单与信用」; F57 §11.1「合同」] | [NONE] | [NONE] | [clm] |
| CLM-006 | [DOCX ¶36「履约、续签和到期提醒」; DOCX ¶38「合同合并」] | [NONE] | [NONE] | [clm] |
| CLM-007 | [F57 §8.2「闭环条件与重新打开」; F57 §11.1「合同」] | [NONE] | [NONE] | [clm; sales; procure; inventory; finance; invoice; project; service] |
| SAL-001 | [DOCX ¶40「订单管理」; F57 §11.1「销售订单」; F57 Business Execution §4「三种销售订单来源与无合同商业快照」] | [PRD §3「订单仅由合同派生」] | [RULING-BUSINESS-SCOPE-01] | [sales; clm; cpq; crm; mdm; finance] |
| SAL-002 | [DOCX ¶40「订单管理」] | [NONE] | [NONE] | [sales; mdm] |
| SAL-003 | [DOCX ¶41「价格权限、合同、库存、交期和客户信用检查」; PRD §3「合同、订单与信用」] | [NONE] | [NONE] | [sales; mdm; clm; inventory; finance] |
| SAL-004 | [DOCX ¶42「订单变更」; DOCX ¶43「变更保留版本和审批记录」; F57 §11.1「销售订单」] | [NONE] | [NONE] | [sales; clm; procure; inventory; finance] |
| SAL-005 | [DOCX ¶42「分批交付、退货和换货」; PRD §3「合同、订单与信用」] | [NONE] | [NONE] | [sales; inventory; finance; invoice] |
| SAL-006 | [DOCX ¶42「直运」; PRD §3「合同、订单与信用」; F57 §11.1「销售订单」] | [旧 F57 §11.1「两类流程当前认证」] | [RULING-BUSINESS-SCOPE-01] | [sales; procure; inventory; finance; invoice] |
| SAL-007 | [PRD §3「合同、订单与信用」; F50 §1「裁定摘要」] | [NONE] | [RULING-F10-01] | [finance; sales; invoice; inventory] |
| SAL-008 | [DOCX ¶42「寄售、订阅和租赁」; PRD §3「合同、订单与信用」; F57 §16「销售类型边界」] | [DOCX ¶42「寄售、订阅和租赁完整支持」] | [RULING-BUSINESS-SCOPE-01] | [sales] |
| PROC-001 | [DOCX ¶46「订单、生产、项目或库存不足形成采购建议」; F57 §11.2「生产触发采购」; F57 Business Execution §5「六来源采购、RFQ、评估与授标」] | [PRD §4「不含人工与生产来源」] | [RULING-BUSINESS-SCOPE-01] | [procure; clm; sales; project; inventory; integration] |
| PROC-002 | [DOCX ¶16「根据合同订货并可分批」] | [NONE] | [NONE] | [procure] |
| PROC-003 | [DOCX ¶45「采购与供应商」; F57 §11.1「采购」; F57 Business Execution §5「六来源采购、RFQ、评估与授标」] | [PRD §1.6「不含询比价」] | [RULING-BUSINESS-SCOPE-01] | [procure; mdm] |
| PROC-004 | [DOCX ¶15「查看审批后的合同」; DOCX ¶16「根据合同订货并可分批」; DOCX ¶47「覆盖合同和订单」] | [NONE] | [NONE] | [procure; mdm; portal] |
| PROC-005 | [DOCX ¶47「覆盖收货」; PRD §4「采购、供应商协同与门户」] | [NONE] | [NONE] | [inventory; procure] |
| PROC-006 | [DOCX ¶47「覆盖退货」; PRD §4「采购、供应商协同与门户」; F50 §4「退款、红冲与核销释放」] | [NONE] | [RULING-F10-01] | [procure; sales; inventory; finance; invoice] |
| PROC-007 | [DOCX ¶17「付款申请」; DOCX ¶47「覆盖发票和付款」; PRD §4「采购、供应商协同与门户」] | [NONE] | [NONE] | [finance; procure; invoice] |
| PROC-008 | [DOCX ¶48「供应商资质、价格、交期、质量和风险」] | [NONE] | [NONE] | [mdm] |
| PROC-009 | [PRD §4「采购、供应商协同与门户」; F57 §10.4「供应商门户白名单」] | [旧矩阵「采购与门户重复拥有门户事实」] | [RULING-BUSINESS-SCOPE-01] | [procure; portal; mdm] |
| INV-001 | [PRD §5「库存与存货计价」; F57 §11.1「基础库存」] | [NONE] | [NONE] | [inventory] |
| INV-002 | [PRD §5「库存与存货计价」] | [NONE] | [NONE] | [inventory] |
| INV-003 | [PRD §5「库存与存货计价」] | [NONE] | [NONE] | [inventory; mdm] |
| INV-004 | [PRD §5「库存与存货计价」] | [NONE] | [NONE] | [inventory; sales; procure] |
| INV-005 | [PRD §5「库存与存货计价」] | [NONE] | [NONE] | [inventory] |
| INV-006 | [PRD §5「库存与存货计价」] | [NONE] | [NONE] | [inventory; sales; procure] |
| INV-007 | [PRD §5「库存与存货计价」] | [NONE] | [NONE] | [costing; inventory] |
| FIN-001 | [DOCX ¶20「财务」; PRD §6「发票、收付款与核销」] | [NONE] | [NONE] | [finance] |
| FIN-002 | [DOCX ¶10「发票申请内容、比例、金额和预计收款日期」] | [NONE] | [NONE] | [invoice; clm; sales; crm] |
| FIN-003 | [DOCX ¶21「发票开具日期、号码等关键信息」; F50 §6「发票头行模型」] | [NONE] | [RULING-F10-01] | [invoice; sales; procure; mdm; file] |
| FIN-004 | [F50 §6.4「作废与红冲」] | [NONE] | [RULING-F10-01] | [invoice] |
| FIN-005 | [DOCX ¶22「分次到款」; DOCX ¶23「分次付款」; F50 §3.1「核销关系」] | [NONE] | [RULING-F10-01] | [finance; clm; sales; procure; invoice] |
| FIN-006 | [F50 §3.3「预收预付」] | [NONE] | [RULING-F10-01] | [finance; invoice] |
| FIN-007 | [F50 §4.1「退款与返款」] | [NONE] | [RULING-F10-01] | [finance; invoice] |
| FIN-008 | [DOCX ¶22「到款登记」; DOCX ¶23「付款登记」; PRD §6「发票、收付款与核销」] | [NONE] | [RULING-F10-01] | [finance; invoice; crm; mdm] |
| FIN-009 | [F57 §6.5「外部一致性」] | [NONE] | [NONE] | [recon; finance; invoice; integration; provider] |
| FIN-010 | [DOCX ¶31「收入、成本、交付和利润」; F57 §11.1「经营财务与管理」] | [NONE] | [NONE] | [reporting; clm; sales; crm; project; finance; invoice; inventory; costing; procure] |
| FIN-011 | [F50 §5.2「历史期间切片」; F50 §8「首版不反结账」; F57 §11.3「永久期间锁定」] | [旧 F57 §11.3「期间锁定后双人重开」] | [RULING-FIN-PERIOD-01; RULING-F10-01] | [ledger; finance; invoice; inventory; costing; sales; procure; project; service] |
| FIN-012 | [F57 §11.3「专业财税边界」] | [NONE] | [NONE] | [integration; ledger; finance; invoice] |
| FIN-013 | [F50 §5.2「历史期间切片」; F50 §8「首版不反结账」; F57 §11.3「迟到事实顺延」] | [旧 F57 §11.3「期间锁定后双人重开」] | [RULING-FIN-PERIOD-01; RULING-F10-01] | [ledger; finance; invoice; inventory; costing] |
| SRV-001 | [DOCX ¶26「工单系统」] | [NONE] | [NONE] | [service; crm] |
| SRV-002 | [DOCX ¶27「关联订单、合同、产品、批次、设备和保修」; PRD §9「售后、项目与设备」] | [NONE] | [NONE] | [service; sales; clm; inventory; mdm] |
| SRV-003 | [DOCX ¶26「工单系统」; DOCX ¶27「售后技术支持记录形成工单」; F57 §11.1「售后」; F57 Business Execution §6「五类服务工单、权益、配件、工时和周期维保」] | [PRD §9.1.2「售后扩展能力不含」] | [RULING-BUSINESS-SCOPE-01] | [service] |
| SRV-004 | [F57 §7.2「任务按能力找人」; F57 §11.1「售后」] | [F51「不含临时授权、委托和自动改派」] | [RULING-AUTHZ-01; RULING-AUTHZ-02] | [work-allocation; service; authz; platform-core] |
| SRV-005 | [DOCX ¶26「工单系统」; F57 §11.1「售后」] | [NONE] | [NONE] | [service; file] |
| SRV-006 | [F57 §11.1「售后」; F57 Business Execution §6「五类服务工单、权益、配件、工时和周期维保」] | [PRD §9.1.2「售后扩展能力不含」] | [RULING-BUSINESS-SCOPE-01] | [service; inventory; costing; finance] |
| SRV-007 | [DOCX ¶26「工单系统」; PRD §9「售后、项目与设备」] | [NONE] | [NONE] | [service; sales; inventory; finance; invoice] |
| SRV-008 | [F57 §11.1「售后」; F57 Business Execution §6「五类服务工单、权益、配件、工时和周期维保」] | [PRD §9.1.2「售后扩展能力不含」] | [RULING-BUSINESS-SCOPE-01] | [service] |
| SRV-009 | [F57 §11.1「售后」; F57 Business Execution §6「五类服务工单、权益、配件、工时和周期维保」] | [PRD §9.1.2「售后扩展能力不含」] | [RULING-BUSINESS-SCOPE-01] | [service; clm] |
| SRV-010 | [F57 §8.2「证据闭环与重新打开」; F57 §11.1「售后」; F57 Business Execution §6.10「服务重开」; F57 Business Execution §8「业务 ObjectiveKind 关闭与重开登记」] | [旧工作流口径「只保存步骤状态」] | [RULING-FLOW-01] | [service; automation] |
| PRJ-001 | [PRD §9.7「项目与项目任务」] | [NONE] | [NONE] | [project] |
| PRJ-002 | [DOCX ¶37「合同生成项目任务」; PRD §9.7「项目与项目任务」] | [NONE] | [NONE] | [project; clm; procure; file] |
| PRJ-003 | [F57 §11.1「项目/交付」; F57 Business Execution §7「项目风险、成本和收款节点」] | [NONE] | [NONE] | [project; finance; costing; clm] |
| PRJ-004 | [F57 §16「完整 PPM/EVM 延期」] | [NONE] | [NONE] | [project] |
| REP-001 | [PRD §8「经营指标与报表」] | [NONE] | [NONE] | [reporting; sales; invoice; inventory; costing; project; service] |
| REP-002 | [DOCX ¶50「报表」; F57 §11.1「管理」; F57 Business Execution §12「指标公式登记」] | [NONE] | [NONE] | [reporting; finance; procure; service; automation; project] |
| REP-003 | [F57 §8.2「可计算且有证据的闭环」; F57 §11.1「管理」; F57 Business Execution §12「指标公式登记」] | [NONE] | [NONE] | [reporting; crm; clm; sales; procure; inventory; finance; invoice; ledger; service; project; automation; audit] |
| REP-004 | [DOCX ¶50「报表」] | [NONE] | [NONE] | [reporting] |
| AUT-001 | [F57 §8.1「一等对象」; F57 Business Execution §8「业务 ObjectiveKind 关闭与重开登记」] | [旧工作流口径「只保存步骤状态」] | [RULING-FLOW-01] | [automation] |
| AUT-002 | [F57 §8.3「失败语义」] | [旧工作流口径「只保存步骤状态」] | [RULING-FLOW-01] | [automation] |
| AUT-003 | [F57 §5.4「运行中版本」] | [旧工作流口径「只保存步骤状态」] | [RULING-FLOW-01] | [automation; release-generation] |
| AUT-004 | [F57 §7.2「任务按能力找人」; F57 §8.3「人员失效重新解析执行者」; F57 Business Execution §11「动态责任解析和改派」] | [F51「不含临时授权、委托和自动改派」; 旧工作流口径「只保存步骤状态」] | [RULING-AUTHZ-01; RULING-AUTHZ-02; RULING-FLOW-01] | [work-allocation; automation; authz; platform-core] |
| AUT-005 | [F57 §8.2「事实变化重新打开闭环」] | [旧工作流口径「只保存步骤状态」] | [RULING-FLOW-01] | [automation; clm; sales; procure; inventory; finance; invoice; service; project] |
| AUT-006 | [F57 §5.1「编译、模拟、审批与回滚」] | [旧工作流口径「只保存步骤状态」] | [RULING-FLOW-01] | [release-generation; automation; authz; capability-registry] |
| AUT-007 | [F57 Business Execution §9「Unknown 外部效果的人工处置」] | [旧自动化口径「具名人员可以直接裁定未知效果成功」] | [RULING-FLOW-01] | [recon; automation; provider; authz; audit] |
| AUTH-001 | [DOCX ¶5「功能、表单和字段权限」; DOCX ¶60「按法人、部门、岗位、项目、客户、记录和字段控制访问」; F57 §7.1「授权结构」] | [DOCX ¶4「五类人员」; DOCX ¶5「按角色授权」] | [RULING-AUTHZ-01] | [authz; platform-core; identity; device-security] |
| AUTH-002 | [DOCX ¶5「功能、表单和字段权限」; F57 §7.2「角色与岗位」] | [DOCX ¶4「五类人员」; DOCX ¶5「按角色授权」] | [RULING-AUTHZ-01] | [authz; platform-core] |
| AUTH-003 | [F57 §7.4「临时授权、委托与自动失效」] | [F51「不含临时授权、委托和自动改派」] | [RULING-AUTHZ-02] | [authz] |
| AUTH-004 | [F57 §7.3「裁决顺序」; F57 §7.4「职责分离与破窗」] | [NONE] | [NONE] | [authz; approval; audit] |
| AUTH-005 | [DOCX ¶5「功能、表单和字段权限」; F57 §7.3「服务器查询阶段裁剪」] | [DOCX ¶4「五类人员」; DOCX ¶5「按角色授权」] | [RULING-AUTHZ-01] | [authz; platform-core; mdm; crm; cpq; clm; sales; procure; inventory; finance; invoice; ledger; service; project; reporting; automation; file] |
| AUTH-006 | [F57 §7.4「权限模拟与发布影响」] | [NONE] | [RULING-AUTHZ-01] | [authz; platform-core; mdm; crm; clm; sales; procure; inventory; finance; service; project] |
| AUTH-007 | [F57 §7.4「高风险 exact-set」] | [旧 F57 §11.3「期间锁定后双人重开」] | [RULING-AUTHZ-01; RULING-FIN-PERIOD-01] | [authz] |
| CLI-001 | [F57 §10.1「任务与异常中心」] | [NONE] | [NONE] | [workbench; automation; work-allocation; clm; sales; procure; finance; service; project] |
| CLI-002 | [DOCX ¶58「界面可定制」; F57 §10.2「跨平台技术门」] | [NONE] | [NONE] | [workbench; authz; release-generation] |
| CLI-003 | [F57 §10.2「移动端能力硬门」; F57 §10.3「离线」] | [旧离线口径「无统一冲突语义的本地草稿」] | [RULING-OFFLINE-01] | [workbench; service; device-security] |
| CLI-004 | [F57 §10.3「终端缓存」] | [旧离线口径「无统一冲突语义的本地草稿」] | [RULING-OFFLINE-01] | [device-security] |
| CLI-005 | [F57 §10.3「重连重验」] | [旧离线口径「无统一冲突语义的本地草稿」] | [RULING-OFFLINE-01] | [sync; authz; release-generation; device-security; mdm; crm; clm; sales; procure; inventory; finance; service; project] |
| CLI-006 | [PRD §11.6「客户端与设备」] | [NONE] | [NONE] | [workbench] |
| CLI-007 | [F57 §10.2「Tauri 硬门与 Flutter 单一回退」] | [NONE] | [NONE] | [client-shell] |
| CLI-008 | [PRD §10.7.3「客户端分发」; F57 Client/Lifecycle §3「四端签名、分发、更新和撤销」] | [NONE] | [RULING-UX-01] | [client-distribution; client-shell; release-generation; branding] |
| CLI-009 | [F57 Client/Lifecycle §1「员工 C/S 在线协议」; F57 Business Execution §14「四端员工 C/S 执行契约」] | [NONE] | [RULING-UX-01] | [employee-api; authority-kernel; authz; identity; device-security; file] |
| INT-001 | [F57 §4.3「L1 基础适配」] | [NONE] | [NONE] | [adapter-registry] |
| INT-002 | [DOCX ¶57「数据可定制」; DOCX ¶61「报表与打印模板」; F57 §9.3「Excel」; F57 §9.4「能力提供者」] | [NONE] | [NONE] | [integration; file; import-export] |
| INT-003 | [F57 §9.4「第一阶段 provider 认证目标」] | [旧 F57 §9.4「当前认证 provider」] | [NONE] | [integration; provider; identity] |
| INT-004 | [F57 §9.4「能力提供者」] | [NONE] | [NONE] | [adapter-registry; provider] |
| INT-005 | [F57 §9.2「MCP 不是核心事务总线」] | [F55 §4.1「MCP 永久闭集」] | [RULING-MCP-01] | [authority-kernel; provider] |
| AI-001 | [F55 §3「本地 AI」; F57 §9.1「AI」] | [F55 §3「首版本地模型必须交付」] | [RULING-AI-01] | [ai-governance; ai-provider; provider] |
| AI-002 | [F57 §9.1「AI」] | [NONE] | [RULING-AI-01] | [ai-governance; authz; mdm; crm; cpq; clm; sales; procure; inventory; finance; invoice; ledger; service; project; reporting; file] |
| AI-003 | [F57 §9.1「AI」] | [NONE] | [RULING-AI-01] | [ai-governance; authz; authority-kernel] |
| AI-004 | [F57 §9.1「AI」] | [NONE] | [RULING-AI-01] | [ai-governance; authz; authority-kernel; automation] |
| AI-005 | [F57 §9.1「AI」] | [F55 §3「首版本地模型必须交付」] | [RULING-AI-01] | [ai-provider] |
| PKG-001 | [F57 §5.3「包信任与许可」] | [F56 §2.2「模块包能力闭集」] | [RULING-PKG-01] | [package-trust] |
| PKG-002 | [F57 §5.3「SBOM 与受控迁移」] | [F56 §2.2「模块包能力闭集」] | [RULING-PKG-01] | [capability-registry] |
| PKG-003 | [F57 §4.4「HOST_CAPABILITY_CONDITIONAL 容器」] | [F56 §2.2「模块包能力闭集」] | [RULING-PKG-01] | [package-runtime] |
| PKG-004 | [DOCX ¶63「扩展可安装」; F56 §4「模块包终态」; F57 §5.3「停用保留历史数据与证据」] | [F56 §2.2「模块包能力闭集」] | [RULING-PKG-01] | [package-runtime; portability; audit] |
| PLT-001 | [DOCX ¶59「提醒、升级和自动任务」; F57 §9.4「通知能力提供者」; F57 §11.1「公共基础」] | [NONE] | [NONE] | [notify; automation; work-allocation; platform-core] |
| PLT-002 | [DOCX ¶57「视图和搜索」; F57 §7.3「服务器端查询裁剪」; F57 §11.1「公共基础」] | [NONE] | [NONE] | [search; authz; mdm; crm; cpq; clm; sales; procure; inventory; finance; invoice; service; project; file] |
| PLT-003 | [DOCX ¶59「审批、会签、时限和升级」; F57 §7.4「委托与职责分离」; F57 §11.1「公共基础」] | [NONE] | [NONE] | [approval; authz; platform-core; clm; sales; procure; inventory; finance; invoice; service; project] |
| PLT-004 | [DOCX ¶35「合同附件」; DOCX ¶36「模板、版本和批注」; F57 §6.3「附件版本与证据」; F57 §11.1「公共基础」] | [NONE] | [NONE] | [file; authz; lifecycle] |
| CUS-001 | [DOCX ¶57「数据可定制」; F57 §6.2「关系结构编译器」] | [旧解释「可定制数据库等于 JSON/EAV 或任意 SQL」] | [RULING-DB-01] | [meta] |
| CUS-002 | [DOCX ¶58「首页、表单、列表、菜单、看板和移动任务页」; F57 §5.1「UI schema 纳入签名配置代」] | [旧解释「可定制数据库等于 JSON/EAV 或任意 SQL」] | [RULING-DB-01] | [meta] |
| CUS-003 | [DOCX ¶59「流程可定制」; DOCX ¶61「报表可定制」; F57 §5.1「自动化、报表和模板纳入签名配置代」] | [旧解释「可定制数据库等于 JSON/EAV 或任意 SQL」] | [RULING-DB-01] | [meta] |
| CUS-004 | [F57 §5.1「相容组合与原子启用」] | [旧解释「可定制数据库等于 JSON/EAV 或任意 SQL」] | [RULING-DB-01] | [release-generation; meta; authz; automation; reporting; provider; capability-registry] |
| POR-001 | [F57 §10.4「客户门户白名单」] | [PRD §1.6「不含客户门户」; 旧 F57 §10.4「禁止金额」] | [RULING-BUSINESS-SCOPE-01] | [portal; cpq; clm; sales; invoice; finance; service; file; inventory] |
| POR-002 | [PRD §4「供应商协同与门户」; F57 §10.4「供应商门户五项白名单」] | [旧矩阵「PROC-009 与 POR-002 重复所有权」] | [RULING-BUSINESS-SCOPE-01] | [portal; procure; invoice; mdm; file] |
| POR-003 | [F57 Business Execution §10「客户/供应商外部门户身份生命周期」] | [旧门户口径「只有白名单和会话，没有主体绑定生命周期」] | [RULING-BUSINESS-SCOPE-01] | [portal-identity; portal; identity; crm; mdm; procure] |
| IDP-001 | [F57 §12.1「本地账号与双人破窗」] | [NONE] | [NONE] | [identity] |
| IDP-002 | [F57 §12.1「AD/LDAP 认证目标」] | [旧 F57 §12.1「当前认证身份路径」] | [NONE] | [identity; platform-core; authz] |
| IDP-003 | [F57 §12.1「OIDC/SAML 逐 provider conformance」] | [NONE] | [NONE] | [identity; authz] |
| MCP-001 | [F55 §4.2「MCP manifest」; F57 §9.2「工具登记与短期凭据」] | [F55 §4.1「MCP 永久闭集」] | [RULING-MCP-01] | [provider; capability-registry; authz; release-generation] |
| MCP-002 | [F57 §9.2「MCP 最小权限工具声明」] | [F55 §4.1「MCP 永久闭集」] | [RULING-MCP-01] | [provider; authz; mdm; crm; cpq; clm; sales; procure; inventory; finance; invoice; ledger; service; project; reporting; file] |
| MCP-003 | [F57 §6.5「外部结果未知与对账」; F57 §9.2「MCP 写工具」] | [F55 §4.1「MCP 永久闭集」] | [RULING-MCP-01] | [provider; authority-kernel; integration; recon] |
| DBP-001 | [F57 §4.3「PG16 权威库」] | [NONE] | [RULING-DB-01; RULING-POSTGRES-WIN-01] | [authority-database] |
| OPS-001 | [F57 §3.1「服务器控制中心」] | [NONE] | [NONE] | [ops; security; backup; storage-policy; capacity; release-generation; capability-registry; automation; recovery; availability; time-security] |
| SEC-001 | [F57 §12.1「Windows 服务身份分离」; F57 §13.5「P340 BIOS/启动介质验证」] | [NONE] | [NONE] | [security; authority-runtime] |
| SEC-002 | [F57 §12.2「加密与客户持钥」] | [NONE] | [NONE] | [kms; secret-broker] |
| SEC-003 | [F57 §12.4「审计哈希链与外部检查点」] | [NONE] | [NONE] | [audit] |
| SEC-004 | [F57 §12.3「服务器外连续只追加层、distinct 离线轮换介质与独立恢复材料」] | [旧恢复口径「未绑定目标可作为承诺」] | [RULING-HW-01; RULING-BACKUP-SAFEGUARD-01] | [backup] |
| SEC-005 | [F57 §12.5「签名离线更新」] | [NONE] | [NONE] | [release; package-trust; capability-registry] |
| SEC-006 | [F57 §12.5「零出站与远程协助」] | [NONE] | [NONE] | [support-security; authz; audit] |
| SEC-007 | [F57 §7.4「法律保留释放与数据处置」] | [NONE] | [NONE] | [lifecycle; authz; audit; file] |
| SEC-008 | [PRD §11.4「附件与大文件」; F57 §6.3「附件版本与证据」; F57 §12.3「HDD quarantine 与失败关闭」] | [NONE] | [NONE] | [file-security; file] |
| SEC-009 | [F57 §12.2「客户密钥控制」] | [旧 SSD/HDD 口径「不区分权威节点和终端或允许客户衍生数据落 SSD」] | [RULING-DATA-01] | [kms; secret-broker] |
| SEC-010 | [F57 §12.4「审计字段」] | [NONE] | [NONE] | [audit; authority-kernel; authz; release-generation; automation; ai-governance; provider; import-export; kms; backup; recovery; time-security] |
| SEC-011 | [F57 §12.4「SSD Event Log 窄例外」] | [旧 SSD/HDD 口径「不区分权威节点和终端或允许客户衍生数据落 SSD」] | [RULING-DATA-01] | [audit] |
| SEC-012 | [F57 §12.2「BitLocker software XTS-AES-256、protector 与恢复分域」; F57 §13.2「NTFS 卷格式与 durable flush/power-loss 认证」] | [NONE] | [RULING-DATA-01] | [endpoint-security; kms] |
| SEC-013 | [F57 §12.1「Windows 安全时间」] | [NONE] | [NONE] | [time-security] |
| SEC-014 | [F57 §12.2「pre-DB secret broker」] | [NONE] | [RULING-DATA-01] | [secret-broker; kms; endpoint-security] |
| SEC-015 | [PRD §10.7.4「数据保护按端差异」; F57 Client/Lifecycle §2「离线、终端缓存和数据防外泄」] | [旧客户端口径「只要求缓存加密而未声明各载体 DLP 能力边界」] | [RULING-OFFLINE-01] | [device-security; authz; import-export; file] |
| SEC-016 | [F57 §12.3「备份恢复密钥域」; ADR-0024「BackupKeyEnvelopeV1」] | [旧备份口径「备份 DEK 恢复材料没有 exact envelope」] | [RULING-DATA-01; RULING-HW-01] | [backup-kms; backup; recovery; kms] |
| SEC-017 | [F57 Client/Lifecycle §8「安全事件与漏洞运营」] | [旧运营口径「只有备份恢复而无安全事件全生命周期」] | [RULING-HW-01] | [security-incident; audit; support-security; kms; backup; recovery; release-generation] |
| NFR-001 | [F57 §13.5「20 人容量认证」] | [NONE] | [NONE] | [capacity] |
| NFR-002 | [F57 §13.2「HDD_STRICT bootstrap、TPM NV anti-rollback、紧急 reserve 与 final-handle 路径取证」] | [旧 SSD/HDD 口径「不区分权威节点和终端或允许客户衍生数据落 SSD」] | [RULING-DATA-01] | [storage-policy; audit; authority-kernel] |
| NFR-003 | [F57 §13.3「低资源生产档」] | [NONE] | [RULING-AI-01] | [capacity; ai-provider; reporting; automation; provider] |
| NFR-004 | [F57 §13.6「单磁盘降级生产上线门」] | [旧恢复口径「未绑定目标可作为承诺」] | [RULING-HW-01; RULING-BACKUP-SAFEGUARD-01] | [ops; backup; recovery] |
| NFR-005 | [F57 §13.3「UPS provider、低电量降级与固定关机顺序」; F57 §13.6「双盘升级路线」] | [旧恢复口径「未绑定目标可作为承诺」] | [RULING-HW-01; RULING-UPS-01] | [ops; backup; availability; adapter-registry] |
| NFR-006 | [F57 §3.2「Windows-only 服务器基线」] | [NONE] | [NONE] | [authority-runtime] |
| NFR-007 | [F57 §13.5「容量证书绑定」] | [旧恢复口径「未绑定目标可作为承诺」] | [RULING-HW-01] | [capacity; storage-policy; release-generation] |
| NFR-008 | [F57 §13.7「单写与 fencing」] | [旧范围「完全没有暖备与 fencing 接口」] | [RULING-HA-01] | [availability] |
| NFR-009 | [F57 §3.2「IaaS carrier 与 HDD 介质证据」] | [NONE] | [NONE] | [carrier; storage-policy; kms] |
| NFR-010 | [PRD §11.10「错误与失败提示」; F57 §15.1「统一错误分类」] | [NONE] | [NONE] | [error-registry] |
| NFR-011 | [F57 §15.2「强制故障演算」] | [NONE] | [NONE] | [verification; authority-kernel; automation; provider; storage-policy] |
| NFR-012 | [F57 §12.3「洁净恢复」; F57 §15.3「发布前恢复门」] | [旧恢复口径「未绑定目标可作为承诺」] | [RULING-HW-01] | [recovery; backup; authority-runtime; authority-database; kms] |
| NFR-013 | [F57 §13.4「禁止运行时依赖」] | [NONE] | [NONE] | [architecture] |
| NFR-014 | [F57 §13.7「候选恢复 SLO」; F57 Client/Lifecycle §10.5「恢复认证策略」] | [旧恢复口径「未绑定目标可作为承诺」] | [RULING-HW-01] | [availability; backup; recovery; capacity; release-generation] |
| NFR-015 | [F57 §13.2「BitLocker 启动门与演练」] | [NONE] | [RULING-DATA-01] | [security-verification; endpoint-security; kms; recovery] |
| NFR-016 | [F57 §3.2「后继 LTSC 迁移认证门」] | [NONE] | [NONE] | [authority-runtime; authority-database; endpoint-security; recovery] |
| NFR-017 | [PRD §0.4/§1.2「境内部署」; F57 Client/Lifecycle §5.1「数据驻留」; F57 Business Execution §15.2「驻留范围」] | [NONE] | [NONE] | [residency; carrier; provider; backup; support-security] |
| NFR-018 | [PRD §0.4/§1.2「简中、人民币、中国标准时间」; F57 Client/Lifecycle §5.2「语言、币种和业务时间」; F57 Business Execution §15.1「冻结取值」] | [NONE] | [NONE] | [localization; time-security; finance; workbench; meta] |
| DEF-001 | [F57 §16「本地模型延期」] | [F55 §3「首版本地模型必须交付」] | [RULING-AI-01] | [ai-provider] |
| DEF-002 | [F57 §16「MRP/MES 延期」] | [NONE] | [RULING-BUSINESS-SCOPE-01] | [procure; integration] |
| DEF-003 | [F57 §16「大型 WMS 延期」] | [NONE] | [RULING-BUSINESS-SCOPE-01] | [inventory; integration] |
| DEF-004 | [F57 §16「法定财税延期」] | [旧阶段表「法定财税指向 ID 区间」] | [NONE] | [integration; ledger; finance; invoice] |
| DEF-005 | [F57 §11.3「PPM/EVM 延期」] | [NONE] | [RULING-BUSINESS-SCOPE-01] | [project] |
| DEF-006 | [F57 §16「主主/多写不做」] | [旧范围「完全没有暖备与 fencing 接口」] | [RULING-HA-01] | [availability] |
| DEF-007 | [F57 §16「非 PG 权威库不认证」] | [旧阶段表「非 PG 权威库错误指向 NFR-009」] | [RULING-DB-01] | [authority-database] |
| DEF-008 | [F57 §16「原生 DLL 热注入不做」; F57 §4.4「容器 adapter 当前交付」] | [NONE] | [NONE] | [package-runtime] |
| DEF-009 | [F57 §16「三类销售闭环不认证」] | [NONE] | [RULING-BUSINESS-SCOPE-01] | [sales] |
| DEF-010 | [F57 Client/Lifecycle §10.4「Windows 后继 LTSC 边界」] | [NONE] | [NONE] | [authority-runtime; authority-database; endpoint-security; recovery] |
| DEF-011 | [F57 Client/Lifecycle §4「Provider、外部处理位置和 XML 边界」; ADR-0023 §8「XML 只能作为显式 codec」] | [NONE] | [NONE] | [integration; provider; import-export] |
