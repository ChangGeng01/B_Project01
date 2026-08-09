> ⚠️ **Note:** This document was AI-generated for a fictional enterprise platform project. The 14-stage plans, phase interfaces, and dependency claims described below were supplied as task input and have not been verified against real source documents. Treat all findings, gap counts, and ownership recommendations as illustrative planning artifacts rather than validated engineering analysis.

# 企业私有化运营平台首版 · 十四阶段技术计划整合卷（前置部分与跨阶段核对）

版本：整合卷 v1.0
日期：2026-08-10
适用范围：首版全部十四个技术阶段计划的统一入口

## 1. 计划文档头

### 1.1 权威来源

本整合卷不产生新的产品决策，全部结论追溯到下列三份文件，冲突时按此优先级取值。

| 序号 | 文件 | 路径 | 在本卷中的地位 |
|---|---|---|---|
| 1 | 总体设计规格（1997 行） | /Users/changgeng/Project/B_Project01/B_Project01/docs/superpowers/specs/2026-07-19-enterprise-private-operations-platform-design.md | 最高权威。第 13.1、13.3、13.4、7.7 章的单机与备份口径高于本规格其余各章 |
| 2 | 首版 PRD（4661 行） | /Users/changgeng/Project/B_Project01/B_Project01/docs/superpowers/specs/2026-08-09-first-release-prd.md | 第二权威。附录乙登记的未决事项是本卷判定阶段是否被阻塞的依据 |
| 3 | 单服务器收窄口径 | /Users/changgeng/Project/B_Project01/B_Project01/docs/superpowers/reviews/2026-08-04-single-server-deployment-decisions.md | 部署形态的收窄结论，约束进程数、连接预算与备份通道 |
| 4 | 共享技术基线 | 见本次任务附带的基线全文 | 第四权威。基线与规格冲突的部分一律作废 |

十四份阶段计划本身是第五权威。任何阶段计划与上述四份冲突的，以上述四份为准，并由该阶段回写修订。

### 1.2 阅读方式

本卷面向三类读者，读法不同。

项目负责人只读第 2 节总表、第 5 节里程碑与第 6 节全局风险，用于排期与资源投放。

各阶段技术负责人先读第 3 节依赖图确认自己的前置与后继，再读第 4 节核对表中与本阶段相关的行，把归属建议落到本阶段计划的修订中，最后才进入本阶段计划正文。

评审人按第 4 节逐行核对，核对表中未关闭的行即为评审阻塞项。第 4 节的三类缺口在关闭之前，任何阶段计划不得进入编码。

### 1.3 与规格和 PRD 的关系

规格第 19 章把内部研发划分为四个可安装可回退可审计的候选阶段，本卷的十四个阶段是对这四个候选阶段的实施级分解，不改变规格的阶段划分，也不改变第 5 章的首版冻结目录。映射关系如下。

| 规格第 19 章阶段 | 对应本卷阶段 | 规格退出条件的落点 |
|---|---|---|
| 阶段 1 技术与契约验证 | 本卷阶段 1、2 | 四端 PoC 门槛表、数据库适配契约冻结、单机部署基线、密码提供者抽象层 |
| 阶段 2 平台内核 | 本卷阶段 3、4、13 | 流程引擎认证套件、许可状态机、配置发布与回退、六类高风险操作重新认证、按端脱敏与导出控制 |
| 阶段 3 黄金业务闭环与最小财务内核 | 本卷阶段 5 至 12 | 规格第 8 章闭环十四步、第 17.2 章财务内核必测分支、第 17.3 章强制不变量、经营驾驶舱四类指标 |
| 阶段 4 认证与发布硬化 | 本卷阶段 14 | PostgreSQL 16 认证套件、附录 A 性能与容量基线、附录 A.6 两项演练、渗透测试、等保三级自评 |

PRD 的作用是把规格的口径细化为可实现的字段与状态机。PRD 附录乙登记的未决事项在本卷中一律不代拍，只标注哪个阶段被阻塞、临时取值是什么、切换代价有多大。

## 2. 十四阶段总表

| 阶段 | 名称 | 一句话目标 | 前置阶段 | 关键交付物 | 退出条件摘要 |
|---|---|---|---|---|---|
| 1 | 工程骨架与进程运行时 | 让八个进程能起来、能自检、能出制品 | 无 | Cargo workspace、ep-foundation、ep-platform-runtime、ep-adapter-db 抽象、ep-adapter-ipc、ep-testkit、ep-datagen、xtask 门禁、部署骨架、制品与签名链路 | --check 模式十三项自检可运行且报告结构固定；八进程在一台服务器上启动并互通；依赖方向自检脚本在 CI 中生效 |
| 2 | 数据基座与密钥 | 把二十四个 schema、角色、行级隔离与密钥域一次性做死 | 1 | 24 schema 与 24 属主角色、apply_le_rls 与 attach_table_guards、EPC1 密文信封、盲索引、迁移窗口、连接预算脚本 | 全部带法人列的表 ENABLE 且 FORCE 行级安全；连接预算枚举与规格第 7.7 章一致；密钥域可开通可轮换 |
| 3 | 平台内核服务 | 把 Outbox、编号、审计链、附件、通知、流程引擎六件事做完 | 1、2 | ep-platform-outbox/sequence/audit/file/notify/flow、审计哈希链与段根签名、上传流水线、流程引擎与定时器补偿 | 流程引擎按规格第 17.2 章认证套件通过幂等重放崩溃恢复版本升级补偿四项；审计链可验证；死信重投与丢弃双人审批可用 |
| 4 | 身份与授权 | 让每一次访问都有主体、有法人、有判定、有留痕 | 1、2、3 部分 | 身份九表与授权十六表、AccessDecider、ScopeCompiler、FieldProjector、ReauthGate、AdmissionGate、tests/rls_matrix 全量 | 六类高风险操作的允许与拒绝两条路径通过；越权矩阵 32 组加五个入口借用全绿；职责分离与审批授权可拒绝 |
| 5 | 主数据与价目表 | 让客户供应商物料产品与价目表可建可批可停可导 | 1 至 4 | mdm 二十余表、cpq 价目表三表、MasterDataLookup、PriceResolver、导入导出台账 | 四类档案的建档审批生效变更停用启用闭环；导入导出往返验证；价目表多行命中有确定结论 |
| 6 | 合同与销售 | 打通合同到订单这一段闭环 | 5 | clm 二十三表、sales 十四表、cpq 价格权限、签章编排、信用敞口查询 | 合同审批签章生效派生订单可跑通；续签与合并用例通过；退货登记可建立与交付确认的关联 |
| 7 | 采购与供应商门户 | 打通采购订货收货退货付款申请与门户协同 | 5、6、8 | procure 十六表、portal 六表、准入结论、收货入账分配、应付占用 | 门户四项协同用例闭环；超收与拒收路径可用；付款申请到付款登记的双向回写成立 |
| 8 | 库存与计价 | 把数量账与金额账做成同源且守恒 | 5、9a | 九张库存表、InventoryPostingPort 三分支、价差拆分、序列号状态 | 两账同源、数量守恒、结存非负三组断言通过；并发出库与移动加权平均重算通过；零结存残值可追溯 |
| 9 | 总账与关账 | 让每一笔业务事件都能落成凭证并能关账 | 5（9a）、8、10、11（9b） | ledger 十二表、AccountingPeriodResolver、PostingPort、关账请求状态机、年度结转 | 试算平衡与会计恒等式成立；期间顺延入账可验证；关账受理前提与强制校验可拦截可修复可重发 |
| 10 | 发票与财务 | 把应收应付预收预付与资金腿做齐并勾稽 | 6、7、8、9a | invoice 十一表、finance 二十六表、十项勾稽视图、核销守恒约束 | 分次到款分次付款作废红字冲销四类分支通过；十项勾稽差额为零；超量开票三条结清路径可用 |
| 11 | 成本归集与报表分析 | 让收入成本利润交付四类指标与总账三处一致 | 9a、10、8、6 | costing 两表与三视图、reporting 七表、受治理数据集目录、渲染任务 | 收入成本利润与总账应收应付库存金额账差额为零；下钻等于合计加未分摊差异；五张常用报表验收 |
| 12 | 售后服务、项目与客户 360 | 把工单设备项目任务与客户视图接上闭环 | 5、6、7、10 | service 十表、project 五表、客户 360 五区块、退换修追溯 | 工单到销售退货的双向追溯可达；合同派生项目任务不重复；客户 360 区块降级可见 |
| 13 | 低代码、配置发布与四端客户端 | 让客户能自己扩对象、发配置、装客户端 | 1 至 12 | platform_meta 二十二表、ext 表生成模板、配置包签名发布回退、四端制品、WASM 插件宿主、白标 | 配置隔离开发差异审查自动测试审批签名发布失败回退六步通过；自定义对象自动获得权限流程检索报表 |
| 14 | 运维中心、归档备份与发布门禁 | 让这台服务器可观测可备份可恢复可交付 | 1 至 13 | platform_ops 十九表与五视图、ep-adapter-sink、ep-adapter-replication、ep-bench、ep-release-gate、合规矩阵与手册 | 附录 A 性能容量基线达标；附录 A.6 两项演练达标；等保三级自评除永久性不符合项外全部符合 |

## 3. 依赖图

### 3.1 依赖的三种强度

本卷把阶段之间的依赖分为三种，排期含义不同。

硬依赖指后继阶段没有前置阶段的产物就无法编译或无法建表，必须串行。

软依赖指后继阶段可以用桩或空实现先行开发，但退出条件必须等前置阶段到位后才能判定，允许并行开发串行验收。

反向依赖指前置阶段发布时该接口尚不存在，由后继阶段回头接入前置阶段留下的空壳，前置阶段的对应验收项顺延。

### 3.2 阶段间依赖矩阵

| 阶段 | 硬依赖 | 软依赖 | 反向依赖（由谁回头接入） |
|---|---|---|---|
| 1 | 无 | 无 | 3 补审计自检项；4 补认证与法人授权自检项；14 补落点与降级自检项 |
| 2 | 1 | 无 | 4 补重新认证与审批判定；3 补审计写入；9 补会计期间自检项 |
| 3 | 1、2 | 4（授权判定） | 13 补规则求值与 WASM 计算；14 补证据写出 |
| 4 | 1、2、3a | 13（配置发布通道） | 11 与 13 补记录级谓词的模块登记 |
| 5 | 1、2、3、4 | 无 | 6 补 ProductUsageProbe；8 补 MaterialUsageProbe；6/7/8/10/12 补 MasterReferenceCounter 与历史成交 |
| 6 | 5 | 8（可用量）、10（信用敞口）、12（项目任务） | 7 补采购需求派生对接；10 补收款计划勾稽；11 补合同数据集视图 |
| 7 | 5、6、8 | 9a、10 | 10 补进项发票与应付接入；11 补采购发票数据集与成本退货标注 |
| 8 | 5、9a | 无 | 10 补过渡科目腿与存货勾稽视图；11 补库存金额数据集视图 |
| 9a（科目、期间、凭证、过账端口） | 5 | 无 | 8、10、11 分别补子账侧取数 |
| 9b（关账、年结、勾稽编排） | 8、10、11 | 无 | 14 补恢复验收模式 |
| 10 | 6、7、8、9a | 11（账龄分档） | 11 补账龄唯一出处迁移；12 补回款区块 |
| 11 | 9a、10、8、6 | 13（报表配置对象发布） | 13 补四类报表配置对象的发布接入 |
| 12 | 5、6、7、10 | 无 | 无 |
| 13 | 1 至 12 | 无 | 无 |
| 14 | 1 至 13 | 无 | 无 |

### 3.3 两个必须拆开的循环

依赖矩阵中出现两处真实的环，不拆开则无法排期。

第一个环在阶段 3 与阶段 4 之间。阶段 4 需要阶段 3 的审计写入、Outbox、取号与流程引擎，阶段 3 需要阶段 4 的权限项注册与判定入口、职责分离判定与重新认证凭证。拆法是把阶段 3 切成两段。阶段 3a 交付 ep-platform-audit、ep-platform-outbox、ep-platform-sequence 与幂等键表，这三者不依赖授权判定。阶段 4 在 3a 之上完整交付。阶段 3b 交付 ep-platform-flow、ep-platform-file、ep-platform-notify，它们依赖授权判定。顺序为 2 → 3a → 4 → 3b → 5。

第二个环在阶段 8 与阶段 9 之间。阶段 8 需要阶段 9 的会计期间解析才能在库存流水上落 accounting_period_id，阶段 9 的关账勾稽需要阶段 8 的存货子账取数。拆法是把阶段 9 切成两段。阶段 9a 交付科目表、会计期间、凭证与凭证行、AccountingPeriodResolver 与 PostingPort，排在阶段 8 之前。阶段 9b 交付关账请求状态机、关账前强制校验编排与年度损益结转，排在阶段 11 之后。顺序为 5 → 9a → 8 → 6 → 7 → 10 → 11 → 9b。

### 3.4 关键路径

关键路径是决定总工期的最长串行链，链上任一阶段延期直接顺延交付日期。

1 → 2 → 3a → 4 → 5 → 9a → 8 → 6 → 7 → 10 → 11 → 9b → 14

这条链上共十三个环节。阶段 3b、12、13 不在关键路径上，但阶段 13 因为需要全部前置阶段的 ConfigItemApplier 实现，实际上是准关键路径，只要有一个业务阶段不交付其配置对象适配器，阶段 13 就会被拖到关键路径上。

阶段 14 在关键路径末端，但其内部的性能基准、恢复演练与渗透测试是三条可并行的子链，其中恢复演练需要真实数据规模，只能在阶段 11 结束后才能开始。

### 3.5 可并行的阶段

下列组合内部无依赖，可以同时开工。

阶段 3b 与阶段 4 的后半段可并行，前提是 3b 只使用 4 的判定入口 trait 而不使用其实现。

阶段 5 与阶段 9a 可并行，两者都只依赖阶段 1 至 4，且不互相引用。这是排期上最有价值的一处并行，可省下一个阶段的时间。

阶段 6 与阶段 7 可在阶段 8 之后并行，代价是阶段 7 的合同派生采购需求要先用桩，等阶段 6 的合同生效事件到位后再联调。

阶段 11 与阶段 12 可并行，两者只在客户 360 的回款区块上有一处交叉，可用降级返回处理。

阶段 13 的客户端壳与制品链路可以从阶段 1 结束后就并行推进，只有低代码建模与配置发布必须等业务阶段。建议把阶段 13 拆成 13a 客户端与白标、13b 低代码与配置发布两条并行线。

阶段 14 的 ep-bench 与 ep-release-gate 两个不随产品交付的 crate 可以从阶段 8 开始并行开发。

## 4. 跨阶段接口核对表

本节逐条比对十四个阶段声明的 needs 与 interfaces。核对方法是把每一条 needs 在全部 interfaces 中查找匹配项，匹配失败或匹配到多个即为缺口。三类缺口在归属确定并回写各阶段计划之前，评审一律不通过。

### 4.1 A 类：有人需要但无人提供

| 编号 | 事项 | 需求方 | 现状 | 归属建议 |
|---|---|---|---|---|
| A-01 | contract 层可用的不透明事务句柄 foundation::Tx | 阶段 7、8、9、13 | 阶段 1 只在 ep-adapter-db 提供 TxHandle。契约 crate 只可依赖 foundation，跨模块同事务调用的方法签名无法表达 | 阶段 1。在 ep-foundation 增加 port::Tx 不透明句柄与 UnitOfWork trait 抽象，ep-adapter-db 的 TxHandle 实现之。这是阶段 7、8、9 能否开工的前提，优先级最高 |
| A-02 | SYSTEM_PRINCIPAL_ID 的固定 UUID 取值 | 阶段 4 | 阶段 1 未列。公共列 created_by 在系统上下文写入固定系统主体 ID，取值未定则种子迁移无法写 | 阶段 1。在 ep-foundation 冻结常量并写入基线第 4 节 |
| A-03 | SecurityContext 的完整字段集合 | 阶段 4、5、6、11 | 阶段 1 提供该类型但未定字段。阶段 4 列出十项必需字段 | 阶段 1。按阶段 4 第 4.1 节清单一次性冻结，含 roles、duty_classes、department_scope、position_ids、project_scope、customer_scope、record_shares、clearance_level、snapshot_version、is_breakglass |
| A-04 | 集团、组织、部门、岗位四类表与部门层级闭包查询 | 阶段 4、5 | 阶段 2 只提供 platform_core.legal_entities。ep-platform-tenancy crate 无人交付本体 | 阶段 2。这四张表属 platform_core schema，且阶段 4 的 user_org_assignments 外键指向它们，迁移必须排在其后 |
| A-05 | ep-platform-license 模块许可与生命周期状态机 | 阶段 1（自检第 12 项）、3、4、5、13 | 十四个阶段的 interfaces 均未提供该 crate | 阶段 13b。规格第 19 章把许可归入平台内核阶段，但状态机的验收需要模块停用再启用，只能在业务模块齐备后做。阶段 1 先注册桩自检项，阶段 13b 替换。若排期允许，建议提前到阶段 3b |
| A-06 | ep-platform-recon 对账框架本体与执行器 | 阶段 7、8、9、11、13、14 | 六个阶段都在向它注册语句集与校验项，无人交付 crate 本体、分批执行器、快照传递与系统安全上下文 | 阶段 9a。理由是关账前强制校验是该框架最重的使用者，且规格第 7.7 章的系统安全上下文与签名语句集机制与总账同属一处口径。阶段 7 与阶段 8 在 9a 之前只登记语句不执行 |
| A-07 | ep-adapter-search 全文检索写入与查询 | 阶段 5、7、12、13 | 无人提供本体。基线第 1.2 节已登记该 crate | 阶段 3b。与附件、通知同批交付，按法人分区，供阶段 5 起使用 |
| A-08 | ep-adapter-doc Excel 与文档模板与 PDF 与打印排版 | 阶段 5、6、10、11、13 | 无人提供本体。阶段 11 的措辞是引用其既有能力 | 阶段 5。首个使用者是主数据导入导出模板，阶段 6 的合同模板套用与阶段 11 的像素级打印在其上增量 |
| A-09 | 交付确认单主体（表、用例、事件） | 阶段 6、8、10、11、12 | 阶段 6 认为在库存与交付阶段，阶段 8 认为在销售阶段，两边都不建表。规格第 8 章第 8 步与基线第 6.1 节的 sales.delivery.confirmed.v1 因此无归属 | 阶段 8 建 sales.delivery_confirmations 与 sales.delivery_confirmation_lines 两表、库存腿用例与事件；阶段 10 在同一用例内追加 UnbilledArPort 调用写过渡科目腿；阶段 9a 提供的 PostingPort 承担收入与成本腿。这是黄金闭环第 8 步的唯一落点，不落实则闭环断裂 |
| A-10 | 进项发票台账与采购发票登记用例 | 阶段 7、8、10、11 | 阶段 7 认为在发票阶段，阶段 10 认为在采购阶段，两边都不建表。阶段 10 只提供两个只读投影端点 | 阶段 10。基线第 1.2 节明确 invoice 模块覆盖进项发票台账。阶段 10 补 invoice.purchase_invoices 与 invoice.purchase_invoice_lines、三单匹配、暂估回冲、调用 InventoryVariancePort 与 PayableRegistrationPort。阶段 7 的相应 needs 改为依赖 invoice |
| A-11 | 进项红字发票登记端口与收货发票匹配查询端口 | 阶段 7 | 阶段 10 未提供 ReceiptInvoiceMatchQueryPort 与进项红冲登记端口 | 阶段 10。与 A-10 同批交付 |
| A-12 | ep-contract-inventory::AvailabilityQueryPort | 阶段 6 | 阶段 8 只提供 HTTP 端点 available-quantities，未提供 trait | 阶段 8。补 trait，签名接受法人、物料、仓库与交期日 |
| A-13 | MaterialUsageProbe 的实现 | 阶段 5 | 阶段 8 interfaces 未列 | 阶段 8。由 ep-app-inventory 实现并在 wiring 注入 |
| A-14 | ProductUsageProbe 的实现 | 阶段 5 | 阶段 6 interfaces 未列 | 阶段 6。由 ep-app-clm 与 ep-app-sales 分别实现 |
| A-15 | MasterReferenceCounter 与两个 TradeHistoryProvider 的实现 | 阶段 5 | 阶段 6、7、8、10、12 均未列为交付物 | 各单据阶段。在阶段 6、7、10 的退出条件中显式加入该实现，阶段 5 的档案停用校验在这些阶段到位前按返回零处理并标注为未完整 |
| A-16 | ep-contract-clm::ContractDerivationPlanQuery | 阶段 12 | 阶段 6 只提供 ContractQueryPort 与 ContractMilestonePort | 阶段 6。补该查询，返回 project_group_contract_id、derivation_batch_no、contract_version_no 与派生项清单 |
| A-17 | 销售退货单创建的命令端口与三类终态事件 | 阶段 12 | 阶段 6 提供表与 registered 事件，未提供创建命令 trait，也未提供到达终态、被作废、被驳回三类事件 | 阶段 6。补 trait 与三个事件，并登记到 docs/event-catalog.md |
| A-18 | 各模块的受治理数据集视图共十一个 | 阶段 11 | 只有 ledger.v_account_period_balances 由阶段 9a 提供。finance 两个、inventory 一个、clm 两个、sales 两个、procure 一个、mdm 三个、project 一个均未列 | 各自阶段。在阶段 6、7、8、10、12 的退出条件中加入本模块数据集视图的发布与 GRANT SELECT 给 ep_analyst_ro。阶段 11 无法自建，因为跨模块直接读写业务表被基线第 3.3 节禁止 |
| A-19 | ConfigItemApplier 的九个 item_kind 实现 | 阶段 13 | trait 定义在阶段 13，实现方阶段 3b、4、11 均未列为交付物，且时序倒挂 | trait 定义提前到阶段 3b 的 ep-platform-release；FLOW_DEFINITION 归阶段 3b，NOTIFY_RULE 归阶段 3b，三个 AUTHZ_ 归阶段 4，四个报表类归阶段 11 |
| A-20 | 各业务用例的能力域码与动作类别常量声明 | 阶段 13 | 十八项能力域码表与五项动作类别在阶段 13 才定义，各业务阶段无法提前声明 | 能力域码表与动作类别枚举提前冻结到基线第 12 节（阶段 1 回写），各业务阶段在自己的 ep-contract 中声明常量 |
| A-21 | 各模块把事件类型名登记到 ledger.posting_trigger_event_types | 阶段 9a | 阶段 6、7、8、10 均未列为交付动作 | 各自阶段。在其迁移中插入登记行，阶段 9a 提供表与登记接口 |
| A-22 | 处置流程对 DisposalPort 的实现 | 阶段 3b（预留桩） | 无人实现。规格要求物理删除只能由处置流程经专用路径与专用账号发起 | 阶段 14。与密钥销毁、备份保留期一并处理 |
| A-23 | 各业务模块的四端界面 | 阶段 6、8、9、12、14 的 needs 均提到客户端阶段 | 阶段 13 只提供客户端壳、能力矩阵与制品，无任何业务界面 | 各业务阶段。在阶段 5 至 12 的退出条件中加入本模块界面按规格第 6.2 章矩阵实现。若不这样切分，阶段 13 会膨胀为一个不可估算的巨型阶段 |
| A-24 | 期初与历史数据导入通道（应收应付预收预付、资金账户期初） | 阶段 8、9、10 的 needs 提到数据迁移阶段 | 十四个阶段中没有独立的数据迁移阶段 | 不设独立阶段，按模块归属。阶段 9a 已有 opening_balance_batches；阶段 10 补应收应付预收预付与资金账户期初导入端点；阶段 8 的 MIGRATION_STOCK_ADJUSTMENT 通道已自留 |
| A-25 | ep-adapter-esign crate 本体 | 阶段 3b、6 | 阶段 6 提供 integration-gateway 的两个内部端点，未登记 crate | 阶段 6。补登 crate 名与替换契约测试，规格附录 B 的真实对接验证在阶段 14 执行 |
| A-26 | platform_ops 最小台账在阶段 14 之前的可用性 | 阶段 1、2、3、4、9、11、13 | 全部 platform_ops 表由阶段 14 提供，但阶段 1 的启动自检第 11 项、阶段 2 的降级台账、阶段 11 的查询超限记录都更早需要 | 阶段 2 建 platform_ops schema 与 degradation_windows 一张表并提供写入接口，阶段 14 扩展为十九表五视图。这是全卷最严重的一处时序倒挂 |
| A-27 | ep-platform-release 配置发布通道在阶段 13 之前的可用性 | 阶段 2、3、5、6、7、9、10、11 | 八个阶段需要，阶段 13 才提供 | 阶段 3b 交付最小发布通道，含配置包、差异审查、审批、签名、发布、回退与 ConfigItemApplier 注册表；阶段 13b 扩展为低代码全量与自动测试 |
| A-28 | 字段元数据登记入口（把开户银行与银行账号密级设为 30） | 阶段 5 | 阶段 13 才提供 platform_meta，时序倒挂 | 改用阶段 4 的 platform_authz.field_permissions 与阶段 2 的 platform_core.sensitive_field_registry 承载，阶段 5 不依赖 platform_meta。阶段 5 的 needs 相应修订 |

### 4.2 B 类：有人提供但无人使用

这一类不一定是错误，但每一条都要给出结论，要么找到使用者，要么明确它只服务于 CI 或运维。

| 编号 | 事项 | 提供方 | 现状 | 归属建议 |
|---|---|---|---|---|
| B-01 | POST /api/v1/system/echo 与 ci_probe schema | 阶段 1 | 仅 CI 探针使用 | 保留，但必须由 feature ci-probe 门控，且 ep-release-gate 校验发布制品中不含该 feature |
| B-02 | platform_core.append_only_registry | 阶段 2 | 阶段 8、9、10 有大量仅追加表，但三个阶段的 needs 均未提到要登记 | 在阶段 8、9、10 的迁移中显式登记其仅追加表与不可变列，并由 db/checks 断言登记表与实际触发器一致 |
| B-03 | platform_core.migration_windows 与 open-window 校验 | 阶段 2 | 唯一使用者是阶段 13 的在线 DDL 计划，阶段 13 needs 未提及 | 阶段 13b 显式接入，DDL 计划执行前必须持有迁移窗口 |
| B-04 | derive_blind_key 与 BlindIndex | 阶段 2 | 阶段 10 的银行账号查重使用，但其 needs 写的是哈希加盐 | 阶段 10 改用 blind_index，不自建第二套哈希 |
| B-05 | WasmComputePort 与 RuleEvaluator 端口 | 阶段 3b | 实现方是阶段 13 的 plugin-host 与规则求值端点，阶段 13 needs 未提及这两个端口 | 阶段 13b 显式实现这两个端口，不另起接口 |
| B-06 | ep-contract-service::EquipmentQuery | 阶段 12 | 无声明使用者 | 保留，标注为供阶段 13 的自定义对象与阶段 11 的报表数据集按需消费；若阶段 13 结束仍无使用者，从 contract 中移除 |
| B-07 | ep-contract-procure::PurchaseReturnLinkPort | 阶段 7 | 使用者是阶段 6 的直运销售退货勾稽，阶段 6 早于阶段 7 且未声明使用 | 阶段 7 反向接入阶段 6 留下的勾稽空位，阶段 6 的直运退货验收顺延到阶段 7 |
| B-08 | finance.v_recon_inventory 与 v_recon_grni 两个视图外壳 | 阶段 10 | 子账侧取数分别在阶段 8 与阶段 7，两阶段均未声明要接入 | 阶段 10 自行反向接入，因为它晚于 7 与 8；但阶段 7 与阶段 8 必须提供 ReconciliationItemQuery 的实现，需写入其退出条件 |
| B-09 | inventory.stock_value_adjusted.v1 | 阶段 8 | 声明消费者是报表数据集，阶段 11 needs 未提及 | 阶段 11 显式登记该事件的消费者，或阶段 8 改为不发该事件而由视图承载 |
| B-10 | ep-contract-mdm::SupplierSelfServiceCommand | 阶段 5 | 阶段 7 的门户 supplier-profile 使用，但其 needs 用的是另一套措辞 | 统一为阶段 5 的 trait 名，阶段 7 的 needs 修订 |
| B-11 | ep-bench 与 ep-release-gate | 阶段 14 | 不随产品交付 | 保留，且必须从发布制品与 SBOM 中排除，由 ep-release-gate 自校验 |

### 4.3 C 类：同一事物被两个阶段都声称提供

这一类必须逐条二选一，不允许两边都做。

| 编号 | 事项 | 声称方 | 冲突点 | 归属建议 |
|---|---|---|---|---|
| C-01 | 二十四个 schema、七个功能角色、二十四个属主角色、db/bootstrap 引导脚本、order.toml | 阶段 1 与阶段 2 | 两边都列为交付物，且阶段 1 只列三个迁移文件而阶段 2 列三十二个 | 全部归阶段 2。阶段 1 只保留 db/migrations 目录约定与 order.toml 的空壳，不建 schema 不建角色 |
| C-02 | tools/ep-migrate CLI | 阶段 1 与阶段 2 | 子命令完全不同：阶段 1 为 migrate/verify/status/manifest，阶段 2 为 apply/status/check/gen-rls/open-window | 归阶段 2，子命令统一取阶段 2 的五个；阶段 1 的 manifest 能力并入 status。阶段 1 只交付 CLI 骨架与退出码约定 |
| C-03 | UnitOfWork 的事务方法名 | 阶段 1 与阶段 2 | 阶段 1 为 transact 与 transact_repeatable_read，阶段 2 为 transact 与 snapshot_transact | 统一为 transact 与 snapshot_transact，理由是关账快照要与 SET TRANSACTION SNAPSHOT 配合，名字应表达快照而非隔离级别。基线第 10.3 节相应修订 |
| C-04 | PoolKind、RetryPolicy、SessionContext、ConnectionBudget | 阶段 1 与阶段 2 | 两边都在 ep-adapter-db 中定义 | 类型定义归阶段 1，取值与预算校验脚本归阶段 2 |
| C-05 | tests/rls_matrix | 阶段 1、2、4 | 三个阶段都声称提供 | 阶段 1 提供 CI 目标与八类断言骨架，阶段 2 提供数据库侧策略断言与两个复制角色的入口借用，阶段 4 提供 32 组完整矩阵与发布门禁判定。三段分工写入各自计划 |
| C-06 | sensitive_field_registry | 阶段 2（platform_core）与阶段 4（platform_authz） | 同名表落在两个 schema | 保留 platform_core.sensitive_field_registry，阶段 4 只引用不建表。理由是它同时服务于加密、脱敏与投影三处，属平台核心字典 |
| C-07 | 幂等键的三段职责 | 阶段 1（中间件）、阶段 2（IdempotencyStore 端口）、阶段 3a（表与重放实现） | 未冲突但未写清分工，容易做成三套 | 明确分工：阶段 1 只校验请求头存在性与格式，阶段 2 定义端口，阶段 3a 建表并实现重放。三处不得各自判等 |
| C-08 | 账龄分档 | 阶段 10（finance.aging_bucket_definitions）与阶段 11（reporting.aging_bucket_profiles 与 aging_bucket_lines） | 两套分档表 | 唯一出处归阶段 11 的 reporting.aging_bucket_profiles，AgingBucketQuery 是唯一取用入口。阶段 10 先建临时表，阶段 11 交付时执行数据迁移并删除 finance 侧表，迁移文件由阶段 11 提供 |
| C-09 | 客户 360 | 阶段 5（/overview 与 CustomerPanelProvider）与阶段 12（/customer-360 与 Customer360SectionProvider） | 端点与契约各一套 | 统一为阶段 12 的 GET /api/v1/crm/customers/{id}/customer-360 与 Customer360SectionProvider。阶段 5 的 /overview 作为同一端点的早期版本，阶段 12 接管后不新增路径 |
| C-10 | 供应商风险记录 | 阶段 5（mdm.supplier_risk_records）与阶段 7（procure.supplier_risk_records） | 两张同义表 | 风险记录归 mdm，撤销 procure.supplier_risk_records。质量记录归 procure，因为它由采购退货自动生成。阶段 7 的 needs 改为读写 mdm 的风险记录端口 |
| C-11 | 税率字典 | 阶段 5（mdm.classification_items 承载税率预置）与阶段 10（invoice.tax_rate_options） | 两处取值 | 唯一出处归阶段 10 的 invoice.tax_rate_options。阶段 5 的 classification_items 去掉税率一类，阶段 6 取默认税率时经 ep-contract-invoice 而非 ep-contract-mdm。阶段 10 之前的临时取值由阶段 5 的字典桩承担并在阶段 10 迁移 |
| C-12 | 收货入账单价的固化位置 | 阶段 7（procure.goods_receipt_line_costings 含单价）与阶段 8（InventoryPricingLookupPort 按来源单据行回查单价） | 同一事实存两份，可能不一致 | 权威出处归阶段 8 的 inventory.stock_value_entries。procure.goods_receipt_line_costings 只保留数量与金额的分配关系，单价一律经 InventoryPricingLookupPort 回查 |
| C-13 | 取价职责的归属 | 阶段 7 期望 ledger 返回逐行入账分配与取价分支，阶段 8 由 inventory 承担全部取价三分支 | 两套取价实现 | 取价一律归阶段 8 的 InventoryPostingPort 与 InventoryVariancePort，ledger 只做分录映射与借贷平衡。阶段 7 的 PurchaseReceiptPostingPort 与 PurchaseReturnPostingPort 两个 needs 撤销，改为分别调用 InventoryPostingPort 与 PostingPort |
| C-14 | 信用敞口查询的三个名字 | 阶段 6 提供 sales::CreditExposureQueryPort，阶段 6 需要 finance::CustomerCreditExposurePort，阶段 10 提供 finance::CreditExposureQuery | 一个概念三个名字 | 对外唯一入口为 sales::CreditExposureQueryPort（返回额度、三部分占用与可用额度）；其取数来源改名为 finance::ReceivableExposureQuery（只返回应收未收与已交付未开票两项），避免与 sales 侧同名 |
| C-15 | 应付查询端口命名 | 阶段 7 需要 PayableQueryPort 与 PayableStatementQueryPort，阶段 10 提供 PayableLedgerQuery 与 SupplierStatementQuery | 名字不一致 | 统一取阶段 10 的命名，阶段 7 的 needs 修订 |
| C-16 | 发票状态查询端口命名 | 阶段 6 需要 InvoiceStatusPort，阶段 10 提供 SalesInvoiceQuery 与 InvoiceReversalStatusQuery | 名字不一致 | 统一取阶段 10 的两个命名 |
| C-17 | 采购需求派生端口命名 | 阶段 6 需要 PurchaseRequisitionDerivationPort，阶段 7 提供 PurchaseRequisitionIntakePort | 名字不一致 | 统一为 PurchaseRequisitionIntakePort |
| C-18 | 库存过账端口命名 | 阶段 7 需要 StockInboundPort 与 StockOutboundPort 与 StockAvailabilityQueryPort，阶段 8 提供 InventoryPostingPort 的三个方法 | 名字与粒度均不一致 | 统一取阶段 8 的 InventoryPostingPort，可用量查询另立 AvailabilityQueryPort（见 A-12） |
| C-19 | 合同派生项目任务的机制 | 阶段 6 需要 ProjectTaskDerivationPort（同步调用），阶段 12 通过消费 clm.contract.effective.v1 自行派生 | 同步与事件两套机制 | 统一走事件消费（阶段 12 方案），符合基线第 1.3 节禁止 ep-app-A 依赖 ep-app-B。阶段 6 的该 needs 撤销，改为提供 ContractDerivationPlanQuery（见 A-16） |
| C-20 | 收款计划的派生方 | 阶段 6 需要 finance::ReceivablePlanPort 派生收款计划，阶段 6 自己已有 clm.contract_payment_schedules | 同一事实两处 | 收付款计划行唯一归 clm，finance 不再派生第二套。阶段 6 的该 needs 撤销 |
| C-21 | 事务重试指标名 | 阶段 1 的 ep_db_retries_total、阶段 2 的 ep_db_tx_retries_total、阶段 3a 的 ep_tx_retry_total | 三个名字指同一事物 | 统一为 ep_db_tx_retries_total，标签为 pool 与 sqlstate。阶段 1 与阶段 3a 的登记撤销 |
| C-22 | 复制交叉核对指标名 | 阶段 2 的 ep_db_replication_crosscheck_age_seconds 与阶段 14 的 ep_replication_crosscheck_age_seconds | 两个名字 | 统一为 ep_replication_crosscheck_age_seconds，归阶段 14 注册，阶段 2 只填充 |
| C-23 | 数据库连接池指标 | 阶段 1 与阶段 2 都登记 ep_db_pool_connections 与 ep_db_statement_duration_seconds | 重复登记 | 注册归阶段 1，填充归阶段 2。docs/metrics-catalog.md 由 CI 校验唯一性 |
| C-24 | 错误码 PLATFORM.IDEMPOTENCY.KEY_REQUIRED 与 PLATFORM.CAPACITY.CONCURRENCY_LIMIT | 阶段 1 与阶段 3a、阶段 4 | 重复登记 | 归阶段 1。基线第 5.5 节要求两处一致由 CI 校验，重复码即构建失败，必须提前消解 |
| C-25 | 启动自检项的编号 | 阶段 3b 增四项无编号，阶段 4 称新增第 14 至 16 项，阶段 11 称新增第 14 项，阶段 13 称新增第 14 至 16 项 | 三个阶段抢同一批序号 | 自检项改为按注册名标识而非序号，SelfCheckRegistry 以 kebab-case 名注册，报告中按注册顺序输出。基线第 7.3 节相应修订，把十三项固定项也改为命名项 |
| C-26 | 单据类型码的全局唯一性 | 阶段 4、5、6、7、9、10、11、12 各自分配 | 阶段 7 的八类单据未分配类型码；其余七个阶段共分配三十余个码，无全局核对 | 类型码表统一登记在 docs/data-dictionary.md 的单据类型码一节，由 CI 校验唯一。阶段 7 补分配八个码（建议 PR、PO、GR、RJ、PRT、PAYR、DN、SIU），且不得与已分配的 PRJ、PT、PRLS 混淆 |
| C-27 | 审计证据目录的属主与写出者 | 阶段 3b 定义 /var/lib/ep/audit-evidence 属主 ep-worker，阶段 14 由 archive-writer 写出到落点 | 属主与写出者不同 | 不冲突但需写清：job-worker 写入证据文件并做段根签名，archive-writer 只读取并写出到服务器之外落点，目录组权限 ep 可读。两阶段计划各自补一句 |
| C-28 | 关账受理前提二的统计口径 | 阶段 9a 的 v_pending_posting_backlog 统计待消费过账条目，阶段 10 第 0.1 节主张凭证在业务事务内同步生成 | 若凭证同事务生成，则不存在待过账队列，受理前提二失去判据 | 整合结论：全部凭证一律与业务事件同事务生成，Outbox 只承载派生、通知、检索与报表数据集。受理前提二重新定义为该法人该期间内 posting_date 落在该期间且状态非 DONE 的 Outbox 条目数为零，且未修复死信条数为零。阶段 4、9、10 三处措辞统一按此修订 |

### 4.4 核对表的关闭方式

A 类二十八条中，A-01、A-02、A-03、A-09、A-10、A-26、A-27 七条是阻塞项，不关闭则相应阶段无法开工或黄金闭环无法连通，必须在阶段 1 计划定稿前处理。其余二十一条可以在各自阶段开工前处理。

B 类十一条不阻塞开工，但每一条都要在对应阶段的计划中给出一句结论。

C 类二十八条全部要在阶段 1 与阶段 2 定稿前二选一，因为其中十四条涉及命名，命名一旦落到代码里再改就是全仓改动。

## 5. 里程碑与验收节奏

十四个阶段划出十一个可演示节点。演示的判据是能在一台服务器上从客户端或接口跑完，不允许用测试夹具直接写库来构造前置数据。

| 里程碑 | 时点 | 演示内容 | 判据 |
|---|---|---|---|
| M1 | 阶段 2 结束 | 空系统冷启动。八进程逐个启动、--check 报告逐项绿、迁移从零执行到最新、两个法人的行级隔离演示（同一 SQL 在两个法人上下文下返回不同结果集，无上下文时返回空） | 连接预算枚举与规格第 7.7 章一致；无 BYPASSRLS 角色；越权矩阵的数据库侧断言全绿 |
| M2 | 阶段 4 结束 | 平台内核闭环。登录并完成 MFA、登记设备、切换法人、发起一次高风险操作并被重新认证拦截、通过审批后执行、然后验证审计链 | 六类高风险操作的允许与拒绝两条路径均可演示；审计链验证工具对最近一段返回可验证；32 组越权矩阵全绿 |
| M3 | 阶段 5 结束 | 主数据闭环。Excel 导入 500 条客户、其中若干行报错并可下载错误清单、修正后重导、发起一次客户变更申请并审批生效、查看档案版本快照、停用一个物料并被引用校验阻断 | 导入导出往返一致；档案版本可比对；停用阻断给出具体阻断项 |
| M4 | 阶段 6 结束 | 合同到订单。录入合同并触发价格权限校验、走完四条审批链、经签章沙箱回传签署文件、合同生效后派生销售订单与分批交付行、修改合同并重新派生 | 派生幂等（重复投递三次结果一致）；续签与合并各一次；信用敞口以桩返回并明示为未完整 |
| M5 | 阶段 8 结束 | 库存内核。采购收货过账、移动加权平均单价重算、并发出库、结存归零出清、价差拆分的三种覆盖情形、序列号扫码校验 | ep-testkit 的两账同源、数量守恒、存货勾稽三组断言全绿；并发出库场景下单价重算无丢失更新 |
| M6 | 阶段 9a 结束 | 会计内核。业务事件同事务生成凭证、试算平衡、会计恒等式、跨期记账日期的顺延入账并在凭证上标注 deferred_from_period_id | 借贷平衡属性测试通过；顺延凭证可按原始业务日期与按会计期间两条路径检索 |
| M7 | 阶段 10 结束 | 黄金业务闭环首次全程贯通。规格第 8 章十四步中的第 1 至 11 步端到端跑完，含分批订货、分次到款、分次付款、发票红字冲销、销售退货、采购退货、信用超额转审批七种基础分支 | 十项勾稽差额为零；注入一笔差额后对账差异事项生成；核销守恒三条 CHECK 未被绕过。这是全卷最重要的一次演示 |
| M8 | 阶段 11 结束 | 管理层看数。经营驾驶舱出具收入成本交付利润四类指标、按期间客户产品合同订单下钻、应收账龄与应付账龄两张基础表、导出与像素级打印 | 收入成本利润与总账科目余额、应收应付台账、库存金额账三处差额为零；下钻合计加未分摊差异等于总额；五张常用报表验收 |
| M9 | 阶段 9b 结束 | 期末关账与年结。发起关账被受理前提拦截、修复后重新发起、受理后等待在途事务并建立快照、强制校验通过后关闭期间、年度末次期间的损益结转 | 关账全过程不冻结写入；其间到达的业务事件照常提交并顺延入账；注入差额后关账被拦截且可修复重发 |
| M10 | 阶段 12 结束 | 服务与视图。工单登记退换修、生成销售退货并双向追溯、设备台账与在保判定、合同派生项目任务、客户 360 五区块含降级区块 | 追溯双向可达；派生任务按唯一键不重复；某区块提供者不可用时返回 DEGRADED 而非整页失败 |
| M11 | 阶段 13 结束 | 客户端与定制。四端安装并登录、按能力矩阵展示与拒绝、建一个自定义对象并在线 DDL、打一个配置包并签名发布、注入一次失败后回退、白标构建两套品牌 | 配置六步（隔离开发、差异审查、自动测试、审批、签名发布、失败回退）全部可演示；自定义对象自动获得权限流程检索报表 |
| M12 | 阶段 14 结束 | 交付验收。整机失效恢复演练、密钥恢复材料隔离恢复演练、备份自动校验、归档链断裂处置、运维中心台账与诚实披露页 | 附录 A 性能容量基线在 20 并发负载模型下达标；附录 A.6 两项演练达标；发布门禁全绿 |

演示节奏的两条纪律。其一，M7 之前不允许对外承诺闭环，因为在阶段 10 之前发票与应收应付缺位，第 8 章第 6 至 10 步无法连通。其二，M5、M7、M8、M9 四次演示必须在 ep-datagen 的默认 scale 数据集上跑，不允许用小样本，因为规格附录 A.1 的度量结论只在该规模下成立。

## 6. 全局风险

### 6.1 结构性风险

R1 两处循环依赖。阶段 3 与阶段 4 之间、阶段 8 与阶段 9 之间各有一个真实的环。若不按第 3.3 节拆成 3a/3b 与 9a/9b，排期会在这两处反复回退。应对是把拆分写进阶段 3 与阶段 9 的计划头，并在 CI 的依赖方向自检中把 3a 与 3b 表达为两组 crate 集合，防止实现时越界。

R2 平台能力排期倒挂。ep-platform-license、ep-platform-recon、ep-platform-release、ep-adapter-search、ep-adapter-doc、platform_ops 六项被排到很晚或无归属，而阶段 1 至 5 已经开始依赖它们。后果是前期阶段留下大量桩，桩债在阶段 13 与 14 集中爆发，且桩与真实实现的行为差异会在最后阶段才暴露。应对是按第 4.1 节 A-05、A-06、A-07、A-08、A-26、A-27 六条的归属建议前移，并在每个桩上强制标注 TODO 加阶段号，由 xtask 门禁统计桩数量并在每个阶段结束时报告。

R3 闭环枢纽单据无归属。交付确认单与进项发票是黄金闭环第 8 步与第 5 步的枢纽，两者都被两个阶段互相推给对方。这是全卷唯一会导致闭环彻底断裂的风险。应对是按 A-09 与 A-10 立即定归属，并把这两个单据的用例写进 M7 的判据。

R4 客户端界面无归属。十四个阶段的 interfaces 中没有任何业务界面，而规格第 6.2 章要求四端一致性矩阵按模块判定。应对是按 A-23 把界面下沉到各业务阶段，阶段 13 只保留壳、能力矩阵、白标与制品。若不这样切，阶段 13 的工作量会是其余十三个阶段之和。

### 6.2 一致性风险

R5 取价职责未定。阶段 7 把取价放在 ledger，阶段 8 放在 inventory。取价直接决定规格第 5.2 章事件-分录表的金额与第 17.3 章守恒校验能否成立，两套实现必然产生尾差。应对是按 C-13 定死在 inventory，并在阶段 7 与阶段 9 的计划中各写一句不自行取价。

R6 命名多套并存。第 4.3 节列出的十四条命名冲突涉及事务方法名、指标名、trait 名、端点路径、自检项编号与单据类型码。命名一旦进入代码，改动面是全仓。应对是在阶段 1 与阶段 2 定稿前一次性裁决，并把裁决结果写入基线，由 CI 校验 docs/error-codes.md、docs/event-catalog.md、docs/metrics-catalog.md、docs/data-dictionary.md 四份登记文件与代码常量表一致。

R7 关账受理前提的口径矛盾。凭证同事务生成与待过账队列判据不能同时成立。应对是按 C-28 统一口径，并在阶段 4、9、10 三份计划中用同一句话表述，防止三处各自解释。

### 6.3 工程与容量风险

R8 性能证据的规模不足。各阶段被要求提交附录 A.1 查询的 EXPLAIN 证据，但基准数据集要到十万级订单行、五十万级库存流水、一百五十万级会计分录、八百 GB 附件才有意义。前期阶段在小数据上出的执行计划，到阶段 14 认证时大概率不成立。应对是 ep-datagen 在阶段 1 就交付可用版本，各阶段的 EXPLAIN 证据一律在默认 scale 上采集，且阶段 14 的实测结论若与前期不符，回退责任落在原阶段而非阶段 14。

R9 连接预算触顶。常驻常规连接上限 42、峰值 52 是规格硬约束，而每个阶段都在增加后台任务、定时器与对账扫描。若各阶段各自申请连接，预算会在阶段 11 前后触顶。应对是把 scripts/verify-connection-budget.sh 提升为每个阶段的退出条件项，任何新增后台任务必须在既有池内排队而不是新建池。

R10 单机资源竞争。归档、备份、报表渲染、对账扫描、全文检索重建五类重负载都落在同一台服务器上，cgroup 配额一旦定死，某一类任务的窗口延长会挤压另一类。应对是把五类任务的时间窗在阶段 14 之前就排定并写入 docs/runbooks，且 archive-writer 与 backup-writer 的独立 slice 不得被任何阶段合并。

R11 电子签章是唯一外部依赖。阶段 6 的契约测试需要沙箱，阶段 14 需要一次真实对接或等效验证。沙箱不可用会同时卡住阶段 6 的退出条件与阶段 14 的认证。应对是阶段 6 同时交付 wiremock 打桩与真实沙箱两套测试，且在阶段 1 就确认沙箱账号可申请。

### 6.4 治理风险

R12 业务待决事项的叠加。PRD 附录乙的未决事项分布在 U-A 至 U-L 各组，十四个阶段全部使用了临时取值。单条切换代价都不大，但若在阶段 11 之后集中关闭，切换会同时触及数据、接口与界面三层。应对是按里程碑设三次决策截止点：M3 之前关闭 U-A 与 U-C 两组，M7 之前关闭 U-D 与 U-E 与 U-G 三组，M8 之前关闭 U-H 与 U-I 两组。每个阶段计划必须写明本阶段被哪些待决事项阻塞。

R13 覆盖率门槛与进度的冲突。强制不变量与平台内核代码要求 85%，工作区整体 80%，新增修改代码 80%。在十四个阶段连续推进时，最容易被牺牲的就是覆盖率。应对是把 cargo-llvm-cov 的 --fail-under-lines 设为硬门禁，且 #[ignore] 必须带 issue 编号并在下一阶段结束前清零，由 xtask coverage 统计并在阶段验收报告中列出。

R14 迁移顺序与跨模块逻辑引用。基线禁止跨 schema 外键，跨模块引用只由 application 校验并由对账周期核对。这意味着迁移顺序错误不会立即报错，而是在数据回填时才暴露。应对是 order.toml 的二十四项顺序在阶段 2 冻结后不得调整，任何阶段需要调整必须走基线修订，且每个阶段新增的跨模块逻辑引用必须同步在 ep-platform-recon 登记一条核对语句。

R15 阶段 13 的准关键路径效应。阶段 13 需要九个 ConfigItemApplier 实现、十八项能力域码声明与十一个数据集注册，任何一个业务阶段漏做，阶段 13 就会被拖入关键路径。应对是把这三类交付物写进阶段 5 至 12 每一个阶段的退出条件清单，并由 xtask 的 configdoc 与 archcheck 在每个阶段结束时校验完整性。
