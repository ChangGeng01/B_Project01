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

各阶段技术负责人先读第 3 节依赖图确认自己的前置与后继，再读第 4 节核对表中与本阶段相关的行，把该行的最终归属与确切标识符逐字落到本阶段计划的修订中，最后才进入本阶段计划正文。

评审人按第 4 节逐行核对。三类缺口的归属已全部裁定，每一行都给出最终归属与确切标识符，状态一律为已裁定。未把该行回写进相应阶段计划的即为评审阻塞项，该阶段不得进入编码。

### 1.3 与规格和 PRD 的关系

规格第 19 章把内部研发划分为四个可安装可回退可审计的候选阶段，本卷的十四个阶段是对这四个候选阶段的实施级分解，不改变规格的阶段划分，也不改变第 5 章的首版冻结目录。映射关系如下。

| 规格第 19 章阶段 | 对应本卷阶段 | 规格退出条件的落点 |
|---|---|---|
| 阶段 1 技术与契约验证 | 本卷阶段 1、2 | 四端 PoC 门槛表、数据库适配契约冻结、单机部署基线、密码提供者抽象层 |
| 阶段 2 平台内核 | 本卷阶段 3、4、13 | 流程引擎认证套件、许可状态机、配置发布与回退、六类高风险操作重新认证、按端脱敏与导出控制 |
| 阶段 3 黄金业务闭环与最小财务内核 | 本卷阶段 5 至 12 | 规格第 8 章闭环十四步、第 17.2 章财务内核必测分支、第 17.3 章强制不变量、经营驾驶舱四类指标 |
| 阶段 4 认证与发布硬化 | 本卷阶段 14 | PostgreSQL 16 认证套件、附录 A 性能与容量基线、附录 A.6 两项演练、渗透测试、等保三级自评 |

PRD 的作用是把规格的口径细化为可实现的字段与状态机。PRD 附录乙登记的未决事项在本卷中一律不代拍，只标注哪个阶段被阻塞、临时取值是什么、切换代价有多大。
### 1.4 归属裁定的四条通则

第 4 节的三类缺口已按归属裁定表逐条定死，下列四条通则对全部 67 条生效，各阶段不得再解释。

第一，权威顺序为规格、PRD、技术基线、阶段计划。本卷第 4 节原先给出的归属建议属阶段计划层，与技术基线冲突时以基线为准。裁定对第 4.1 节的 A-05、A-06、A-08、A-09、A-19、A-27 与第 4.2 节的 B-06 七条作了与原建议不同的结论，第 4 节已按裁定原样改写。

第二，模块归属的唯一判据是基线第 1.2 节的十五个模块码覆盖范围与基线第 1.3 节的一个仓储只访问自己模块的 schema。表落在哪个 schema，该 schema 对应的模块所在阶段就是该表的所有者，不存在甲阶段在乙模块的 schema 里建表这一形态。

第三，跨模块同步调用只有一种形态。调用方 ep-app-A 依赖被调方 ep-contract-B 的 trait，实现由被调方的 ep-app-B 提供，在 apps/core-server/src/wiring.rs 与 apps/job-worker/src/wiring.rs 注入。被调方阶段晚于调用方阶段时，调用方注入以 Noop 前缀命名的空实现并在该行加注释 TODO 加阶段号，被调方阶段替换该行，调用方阶段的相应验收项顺延到被调方阶段。

第四，调整后的阶段顺序固定为 1 → 2 → 3a → 4 → 3b → 5 → 9a → 8 → 6 → 7 → 10 → 11 → 9b → 13 → 14，阶段 12 在阶段 10 之后与阶段 11 并行。第 3 节的依赖矩阵、拆环、关键路径与并行建议全部以这条链为基准。

## 2. 十四阶段总表

| 阶段 | 名称 | 一句话目标 | 前置阶段 | 关键交付物 | 退出条件摘要 |
|---|---|---|---|---|---|
| 1 | 工程骨架与进程运行时 | 让八个进程能起来、能自检、能出制品 | 无 | Cargo workspace、ep-foundation（含 Tx 与 UnitOfWork、SecurityContext、SYSTEM_PRINCIPAL_ID、ModuleCode、CapabilityDomain 与 ActionClass、id::marker）、ep-platform-runtime 与命名自检注册表、ep-adapter-db 抽象、ep-adapter-ipc、ep-testkit、ep-datagen、xtask 门禁、部署骨架、制品与签名链路 | --check 模式的十三个命名自检项可运行且报告按注册顺序输出；八进程在一台服务器上启动并互通；依赖方向自检脚本在 CI 中生效 |
| 2 | 数据基座与密钥 | 把二十四个 schema、角色、行级隔离、组织架构与密钥域一次性做死 | 1 | 24 schema 与 24 属主角色、db/bootstrap 五个引导脚本、tools/ep-migrate 五个子命令、tenancy 五表、apply_le_rls 与 attach_table_guards、EPC1 密文信封、盲索引、迁移窗口、platform_ops.degradation_windows 与 DegradationLedger、连接预算脚本 | 全部带法人列的表 ENABLE 且 FORCE 行级安全；连接预算枚举与规格第 7.7 章一致；密钥域可开通可轮换；部门层级闭包在同一事务内全量重写并可查询 |
| 3 | 平台内核服务 | 把 Outbox、编号、审计链、附件、通知、流程引擎、许可、检索与最小发布通道九件事做完 | 1、2 | ep-platform-outbox/sequence/audit/file/notify/flow、ep-platform-license、ep-adapter-search、ep-platform-release 的 ConfigItemApplier 端口与最小发布通道、审计哈希链与段根签名、上传流水线、流程引擎与定时器补偿 | 流程引擎按规格第 17.2 章认证套件通过幂等重放崩溃恢复版本升级补偿四项；审计链可验证；死信重投与丢弃双人审批可用；模块许可状态机可判定；最小发布通道六态可发布可回退 |
| 4 | 身份与授权 | 让每一次访问都有主体、有法人、有判定、有留痕 | 1、2、3a | 身份九表与授权十六表、AccessDecider、ScopeCompiler、FieldProjector、ReauthGate、AdmissionGate、三个 AUTHZ 类 ConfigItemApplier、tests/rls_matrix 的 32 组矩阵 | 六类高风险操作的允许与拒绝两条路径通过；越权矩阵 32 组加五个入口借用全绿；职责分离与审批授权可拒绝 |
| 5 | 主数据与价目表 | 让客户供应商物料产品与价目表可建可批可停可导 | 1 至 4 | mdm 二十余表、cpq 价目表三表、MasterDataLookup、PriceResolver、ep-adapter-doc 与三个文档端口、导入导出台账 | 四类档案的建档审批生效变更停用启用闭环；导入导出往返验证；价目表多行命中有确定结论；本模块四端界面按规格第 6.2 章矩阵实现 |
| 6 | 合同与销售 | 打通合同到订单再到交付确认这一段闭环 | 5、9a、8 | clm 二十三表、sales 十六表（含交付确认单两表）、cpq 价格权限、签章编排、ep-adapter-esign、信用敞口查询 | 合同审批签章生效派生订单可跑通；续签与合并用例通过；交付确认单可建立可确认且库存腿与凭证腿生效；退货登记可建立与交付确认的关联；本模块四端界面按规格第 6.2 章矩阵实现 |
| 7 | 采购与供应商门户 | 打通采购订货收货退货付款申请与门户协同 | 5、9a、8、6 | procure 十五表（撤销供应商风险记录表后）、portal 六表、准入结论、收货入账分配、应付占用 | 门户四项协同用例闭环；超收与拒收路径可用；付款申请到付款登记的双向回写成立；本模块四端界面按规格第 6.2 章矩阵实现 |
| 8 | 库存与计价 | 把数量账与金额账做成同源且守恒 | 5、9a | 九张库存表、五个对外 trait（含 InventoryPostingPort 三方法与 AvailabilityQueryPort）、价差拆分、序列号状态 | 两账同源、数量守恒、结存非负三组断言通过；并发出库与移动加权平均重算通过；零结存残值可追溯；本模块四端界面按规格第 6.2 章矩阵实现 |
| 9 | 总账与关账 | 让每一笔业务事件都能落成凭证并能关账 | 5（9a）、8、10、11（9b） | ledger 十二表、ep-platform-recon 与 platform_core 对账三表、AccountingPeriodResolver、PostingPort、关账请求状态机、年度结转 | 试算平衡与会计恒等式成立；期间顺延入账可验证；关账受理前提与强制校验可拦截可修复可重发；对账框架可分批执行并落差异事项 |
| 10 | 发票与财务 | 把应收应付预收预付与资金腿做齐并勾稽 | 6、7、8、9a | invoice 十三表（含进项发票两表）、finance 二十六表、十项勾稽视图、核销守恒约束 | 分次到款分次付款作废红字冲销四类分支通过；十项勾稽差额为零；超量开票三条结清路径可用；三单匹配与暂估回冲可验证；本模块四端界面按规格第 6.2 章矩阵实现 |
| 11 | 成本归集与报表分析 | 让收入成本利润交付四类指标与总账三处一致 | 9a、10、8、6 | costing 两表与三视图、reporting 七表、受治理数据集目录、账龄分档唯一出处、四个报表类 ConfigItemApplier、渲染任务 | 收入成本利润与总账应收应付库存金额账差额为零；下钻等于合计加未分摊差异；五张常用报表验收；本模块四端界面按规格第 6.2 章矩阵实现 |
| 12 | 售后服务、项目与客户 360 | 把工单设备项目任务与客户视图接上闭环 | 5、6、7、10 | service 十表、project 五表、客户 360 五区块、退换修追溯 | 工单到销售退货的双向追溯可达；合同派生项目任务不重复；客户 360 区块降级可见 |
| 13 | 低代码、配置发布与四端客户端 | 让客户能自己扩对象、发配置、装客户端 | 1 至 12 | platform_meta 二十二表（其中配置包三表由阶段 3b 建立本阶段扩展）、ext 表生成模板、六个自定义类 ConfigItemApplier、配置包签名发布回退十一态、四端客户端壳与能力矩阵闸、WASM 插件宿主、白标与四端制品 | 配置隔离开发差异审查自动测试审批签名发布失败回退六步通过；自定义对象自动获得权限流程检索报表；本阶段不交付任何业务界面 |
| 14 | 运维中心、归档备份与发布门禁 | 让这台服务器可观测可备份可恢复可交付 | 1 至 13 | platform_ops 十九表与五视图（degradation_windows 由阶段 2 建立本阶段扩展）、ep-adapter-sink、ep-adapter-replication、OpsDisposalService、ep-bench、ep-release-gate、合规矩阵与手册 | 附录 A 性能容量基线达标；附录 A.6 两项演练达标；发布门禁含 RG-CI-PROBE-ABSENT 与 RG-TOOLS-EXCLUDED 全绿；等保三级自评除永久性不符合项外全部符合 |

## 3. 依赖图

### 3.1 依赖的三种强度

本卷把阶段之间的依赖分为三种，排期含义不同。

硬依赖指后继阶段没有前置阶段的产物就无法编译或无法建表，必须串行。

软依赖指后继阶段可以用桩或空实现先行开发，但退出条件必须等前置阶段到位后才能判定，允许并行开发串行验收。

反向依赖指前置阶段发布时该接口尚不存在，由后继阶段回头接入前置阶段留下的空壳，前置阶段的对应验收项顺延。

### 3.2 阶段间依赖矩阵

| 阶段 | 硬依赖 | 软依赖 | 反向依赖（由谁回头接入） |
|---|---|---|---|
| 1 | 无 | 无 | 2 补降级台账写入与 rls-enabled-and-forced 及 runtime-role-privileges-bounded 两项的被测对象；3a 补审计自检项；3b 补 license-and-modules-consistent 自检项；4 补认证与法人授权自检项；14 补落点自检项 |
| 2 | 1 | 无 | 3a 补审计写入；3b 补配置发布通道的接入；4 补重新认证与审批判定；9a 补 current-period-open 自检项；14 补 ep_replication_crosscheck_age_seconds 的指标注册 |
| 3 | 1、2 | 4（授权判定） | 13b 补 WasmComputePort 与 RuleEvaluator 的实现；14 补证据写出与 DisposalPort 的实现 |
| 4 | 1、2、3a | 3b（配置发布通道） | 11 与 13 补记录级谓词的模块登记 |
| 5 | 1、2、3a、4、3b | 无 | 6 补 ClmProductUsageProbe 与 SalesProductUsageProbe；8 补 InventoryMaterialUsageProbe；6、7、8、10、12 补 MasterReferenceCounter 与两个 TradeHistoryProvider；10 补 TaxRateOptionQuery 并撤销 MdmTaxRateStub |
| 6 | 5、9a、8 | 10（信用敞口与发票红冲状态） | 7 补 PurchaseRequisitionIntakePort 与 PurchaseReturnLinkPort；10 补 UnbilledArPort 过渡科目腿、ReceivableExposureQuery 与 InvoiceReversalStatusQuery |
| 7 | 5、9a、8、6 | 10（进项发票台账与应付） | 10 补进项发票登记与应付接入并包装 GrniSubledgerBalanceQuery；11 补成本退货标注 |
| 8 | 5、9a | 无 | 10 把 InventorySubledgerBalanceQuery 包装为 SubledgerBalanceProvider 并建两个勾稽视图；11 补 costing.stock_value_adjust 消费者 |
| 9a（科目、期间、凭证、过账端口、对账框架） | 5 | 无 | 6、7、10 回填 posting_trigger_event_types 的 event_type；7、8、10、11、13、14 补各自的 ReconCheck 实现与注册 |
| 9b（关账、年结、勾稽编排） | 8、10、11 | 无 | 14 补恢复验收模式 |
| 10 | 6、7、8、9a | 11（账龄分档） | 11 补账龄唯一出处迁移；12 补回款区块 |
| 11 | 9a、10、8、6 | 3b（配置发布通道） | 12 补 project.v_projects_dataset，在其发布前 reporting-dataset-signature-matched 按已登记未发布降级放行 |
| 12 | 5、6、7、10 | 无 | 无 |
| 13 | 1 至 12 | 无 | 无 |
| 14 | 1 至 13 | 无 | 无 |

### 3.3 两个必须拆开的循环

依赖矩阵中出现两处真实的环，不拆开则无法排期。

第一个环在阶段 3 与阶段 4 之间。阶段 4 需要阶段 3 的审计写入、Outbox、取号与流程引擎，阶段 3 需要阶段 4 的权限项注册与判定入口、职责分离判定与重新认证凭证。拆法是把阶段 3 切成两段。阶段 3a 交付 ep-platform-audit、ep-platform-outbox、ep-platform-sequence、幂等键表与 ep-platform-release 的 ConfigItemApplier 端口，这五者都不依赖授权判定，其中端口只是一个文件，无表无用例，因此不破坏拆环。阶段 4 在 3a 之上完整交付，含三个 AUTHZ 类 applier。阶段 3b 交付 ep-platform-flow、ep-platform-file、ep-platform-notify、ep-platform-license、ep-adapter-search 与 ep-platform-release 的最小发布通道，它们依赖授权判定。顺序为 2 → 3a → 4 → 3b → 5。

第二个环在阶段 8 与阶段 9 之间。阶段 8 需要阶段 9 的会计期间解析才能在库存流水上落 accounting_period_id，阶段 9 的关账勾稽需要阶段 8 的存货子账取数。拆法是把阶段 9 切成两段。阶段 9a 交付科目表、会计期间、凭证与凭证行、AccountingPeriodResolver、PostingPort 与 ep-platform-recon 对账框架本体，排在阶段 8 之前。阶段 9b 交付关账请求状态机、关账前强制校验编排与年度损益结转，排在阶段 11 之后。顺序为 5 → 9a → 8 → 6 → 7 → 10 → 11 → 9b。交付确认单按 A-09 归阶段 6，其库存腿与凭证腿分别取自阶段 8 与阶段 9a，因此阶段 6 必须排在两者之后，该约束与这条顺序一致，不产生新的环，唯一的反向依赖是阶段 10 回头替换过渡科目腿的空实现。

### 3.4 关键路径

关键路径是决定总工期的最长串行链，链上任一阶段延期直接顺延交付日期。

1 → 2 → 3a → 4 → 3b → 5 → 9a → 8 → 6 → 7 → 10 → 11 → 9b → 13 → 14

这条链上共十五个环节。阶段 3b 因承载许可、全文检索与最小发布通道进入关键路径，阶段 13 按裁定通则第四条固定排在阶段 9b 之后，也进入关键路径。不在关键路径上的只有阶段 12，它在阶段 10 之后与阶段 11 并行。阶段 13 仍是最脆弱的一环，它自己实现六个自定义类 ConfigItemApplier，另外九个分别由阶段 3b、4、11 交付，任一阶段漏做都会直接顺延阶段 13。

阶段 14 在关键路径末端，但其内部的性能基准、恢复演练与渗透测试是三条可并行的子链，其中恢复演练需要真实数据规模，只能在阶段 11 结束后才能开始。

### 3.5 可并行的阶段

下列组合内部无依赖，可以同时开工。

阶段 3b 与阶段 4 的后半段可并行，前提是 3b 只使用 4 的判定入口 trait 而不使用其实现。

阶段 5 与阶段 9a 可并行，两者都只依赖阶段 1 至 4，且不互相引用。这是排期上最有价值的一处并行，可省下一个阶段的时间。

阶段 6 与阶段 7 可在阶段 8 之后并行，代价是阶段 6 先注入 NoopPurchaseRequisitionIntakePort，等阶段 7 替换后再联调，直运退货勾稽的 NoopPurchaseReturnLinkPort 同此处理。

阶段 11 与阶段 12 在阶段 10 之后并行，两者有两处交叉：客户 360 的回款区块可用降级返回处理，project_projects 数据集由阶段 11 先播种目录、阶段 12 后建视图，其间自检项 reporting-dataset-signature-matched 按已登记未发布降级放行。

阶段 13 的客户端壳与制品链路可以从阶段 1 结束后就并行推进，业务界面已按 A-23 下沉到阶段 5 至 12，阶段 13 只留壳、能力矩阵闸、白标与制品。建议把阶段 13 拆成 13a 客户端与白标、13b 低代码与配置发布两条并行线，13b 依赖阶段 3b 的最小发布通道与阶段 11 的四个报表类 applier。

阶段 14 的 ep-bench 与 ep-release-gate 两个不随产品交付的 crate 可以从阶段 8 开始并行开发。

## 4. 跨阶段接口核对表

本节逐条比对十四个阶段声明的 needs 与 interfaces。核对方法是把每一条 needs 在全部 interfaces 中查找匹配项，匹配失败或匹配到多个即为缺口。三类缺口的归属已全部裁定，每一行给出最终归属与确切标识符，状态一律为已裁定。各阶段计划按确切标识符列逐字回写，未回写的行即为评审阻塞项，该阶段不得进入编码。名字、路径、列名、端点与事件名一律以本节为准，任何阶段不得另起一套，也不得重新解释归属。

### 4.1 A 类：有人需要但无人提供

| 编号 | 事项 | 需求方 | 现状 | 最终归属 | 确切标识符 | 状态 |
|---|---|---|---|---|---|---|
| A-01 | contract 层可用的不透明事务句柄与工作单元 | 阶段 7、8、9、13 | 阶段 1 只在 ep-adapter-db 提供 TxHandle。契约 crate 只可依赖 foundation，跨模块同事务调用的方法签名无法表达 | 阶段 1 | crates/foundation/src/port/tx.rs 冻结 Tx、SnapshotCtx、UnitOfWork 三个 trait 与 TxId、IsolationKind、BoxFuture；UnitOfWork 的两个方法为 transact 与 snapshot_transact 且不带池参数；契约层跨模块方法签名一律写 `&mut dyn Tx`；`downcast_mut::<PgTx>` 只允许出现在 crates/adapter/db-pg/ 内并由 xtask archcheck 断言；application crate 对 UnitOfWork 取泛型参数而非 trait 对象；配套 crates/foundation/src/id/marker.rs 冻结 22 项标记类型 | 已裁定 |
| A-02 | SYSTEM_PRINCIPAL_ID 的固定 UUID 取值 | 阶段 4 | 阶段 1 未列。公共列 created_by 在系统上下文写入固定系统主体 ID，取值未定则种子迁移无法写 | 阶段 1 | crates/foundation/src/principal.rs 的 SYSTEM_PRINCIPAL_ID 取 00000000-0000-7000-8000-000000000001，SYSTEM_DEVICE_ID 取 SYSTEM；由 SecurityContext::system 固定使用，并回写基线第 4 节 created_by 一行的语义列；任何阶段不得再自选取值 | 已裁定 |
| A-03 | SecurityContext 的完整字段集合 | 阶段 4、5、6、11 | 阶段 1 提供该类型但未定字段。阶段 4 列出十项必需字段 | 阶段 1 | crates/foundation/src/security/context.rs 按裁定顺序冻结 19 个字段，依次为 user_id、account_kind、session_id、legal_entity_id、device_id、client、clearance_level、roles、duty_classes、department_scope、position_ids、project_scope、customer_scope、record_shares、data_scope_tags、snapshot_version、is_breakglass、request_id、trace_id；配套枚举 AccountKind、ClientKind、DepartmentScope；构造入口只有 SecurityContext::human 与 SecurityContext::system，不提供任何 with_ 前缀的变换方法 | 已裁定 |
| A-04 | 集团、组织、部门、岗位四类表与部门层级闭包查询 | 阶段 4、5 | 阶段 2 只提供 platform_core.legal_entities。ep-platform-tenancy crate 无人交付本体 | 阶段 2 | platform_core.enterprise_groups、organizations、departments、positions、department_closures 五表，闭包表唯一约束 ux_department_closures_pair 在 ancestor_department_id 与 descendant_department_id 上；ep-platform-tenancy 交付 LegalEntityDirectory 与 DepartmentClosureQuery::descendant_ids；闭包在部门新增、改父、停用的同一事务内全量重写，不用递归 CTE 在线查询；阶段 4 的 department_id 与 position_id 外键目标写死为 platform_core.departments(id) 与 platform_core.positions(id) | 已裁定 |
| A-05 | ep-platform-license 模块许可与生命周期状态机 | 阶段 1（自检项 license-and-modules-consistent）、3、4、5、13 | 十四个阶段的 interfaces 均未提供该 crate | 阶段 3b 交付本体，阶段 13b 只补一条停用再启用的验收用例 | platform_core.module_registrations、license_grants、feature_flags 三表，均不带 legal_entity_id 且不建策略；ep-platform-license 的 ModuleLicenseQuery 三方法与 ModuleState、LicenseStatus；ModuleCode 十五项枚举由阶段 1 交付；阶段 1 把自检项 license-and-modules-consistent 标为 Pending，阶段 3b 换成实现；阶段 5 的启动自检读取 ModuleLicenseQuery::module_state | 已裁定 |
| A-06 | ep-platform-recon 对账框架本体与执行器 | 阶段 7、8、9、11、13、14 | 六个阶段都在向它注册语句集与校验项，无人交付 crate 本体、分批执行器、快照传递与系统安全上下文 | 阶段 9a | platform_core.recon_check_definitions、recon_runs、recon_discrepancies 三表，后两张带法人并按基线第 3.8 节建策略；ep-platform-recon 的 ReconCheck、ReconRegistry、ReconExecutor 与 BatchWindow、ReconRunOutcome；执行器逐法人遍历，只在单一法人上设置 app.legal_entity_id，快照由 UnitOfWork::snapshot_transact 导出并逐批传递；六个注册方一律实现 ReconCheck 并在 wiring 注册；阶段 3b 的附件孤儿收敛改称 job-worker 内的幂等收敛任务，不使用该框架 | 已裁定 |
| A-07 | ep-adapter-search 全文检索写入与查询 | 阶段 5、7、12、13 | 无人提供本体。基线第 1.2 节已登记该 crate | 阶段 3b，端口空文件由阶段 1 建 | crates/foundation/src/port/search.rs 的 SearchDocument、SearchQuery、SearchHit 与 SearchIndexPort、SearchQueryPort；索引按法人分区，物理路径 /var/lib/ep/search 下按 legal_entity_id 分目录；写入一律经 job-worker 消费 Outbox 事件触发，不在业务事务内调用；各阶段只产出 SearchDocument，不自建写入路径 | 已裁定 |
| A-08 | ep-adapter-doc Excel 与文档模板与 PDF 与打印排版 | 阶段 5、6、10、11、13 | 无人提供本体。阶段 11 的措辞是引用其既有能力 | 阶段 5，端口空文件由阶段 1 建 | crates/foundation/src/port/doc.rs 的 SpreadsheetPort（write_xlsx 与 read_xlsx）、DocTemplatePort::render、PdfRenderPort::render_pdf 与 SheetSpec、ColumnSpec、CellValue、PrintLayout；ep-adapter-doc 覆盖导入模板生成、错误行清单渲染与 XLSX 读写三项；阶段 6、10、11、13 只在这三个 trait 上增量并只产出 PrintLayout 取值，不新增 trait 也不自建渲染路径 | 已裁定 |
| A-09 | 交付确认单主体（表、用例、事件） | 阶段 6、8、10、11、12 | 阶段 6 认为在库存与交付阶段，阶段 8 认为在销售阶段，两边都不建表。规格第 8 章第 8 步与基线第 6.1 节的 sales.delivery.confirmed.v1 因此无归属 | 阶段 6 建表、建用例、发事件；阶段 8 提供库存腿端口；阶段 9a 提供收入与成本腿端口；阶段 10 提供过渡科目腿端口并反向替换阶段 6 的空实现 | sales.delivery_confirmations 与 sales.delivery_confirmation_lines 两表，类型码 DC，不设作废态，冲正一律经销售退货单；用例 create_delivery_confirmation.rs 与 confirm_delivery.rs，端点 POST /api/v1/sales/delivery-confirmations 与 POST /api/v1/sales/delivery-confirmations/{id}/actions/confirm-delivery；事件 sales.delivery.confirmed.v1，aggregate_type 取 sales.delivery_confirmations；确认事务内次序固定为 InventoryPostingPort::post_outbound、UnbilledArPort::record_on_delivery、PostingPort::post，会计期间由 AccountingPeriodResolver::resolve 在事务最前解析一次；sales.return_line_delivery_links 与 sales.delivery_schedules 的相关列改为同 schema 真实外键 | 已裁定 |
| A-10 | 进项发票台账与采购发票登记用例 | 阶段 7、8、10、11 | 阶段 7 认为在发票阶段，阶段 10 认为在采购阶段，两边都不建表。阶段 10 只提供两个只读投影端点 | 阶段 10 | invoice.purchase_invoices 与 invoice.purchase_invoice_lines 两表，类型码 PINV，唯一约束 ux_purchase_invoices_legal_entity_id_supplier_id_invoice_no；用例 register_purchase_invoice.rs 与端点 POST /api/v1/invoice/purchase-invoices；三单匹配在该用例内依次比对采购订单行、收货行与本次发票行；暂估回冲与价差拆分经 InventoryVariancePort::split_variance 取数，本阶段不自行取价；应付腿经本模块的 register_payable_on_purchase_invoice 用例；事件 invoice.purchase_invoice.registered.v1；阶段 7 的两处查询先注入空实现，相应验收顺延到 M7 | 已裁定 |
| A-11 | 进项红字发票登记端口与收货发票匹配查询端口 | 阶段 7 | 阶段 10 未提供 ReceiptInvoiceMatchQueryPort 与进项红冲登记端口 | 阶段 10，与 A-10 同批交付 | crates/contract/invoice/src/port/purchase.rs 的 ReceiptInvoiceMatchQueryPort（match_state 与 match_states）与 PurchaseCreditNotePort::register_credit_note，配套 DTO 为 ReceiptInvoiceMatchState、RegisterPurchaseCreditNote、PurchaseCreditNoteLine、PurchaseCreditNoteView；阶段 7 先注入 NoopReceiptInvoiceMatchQueryPort 与 NoopPurchaseCreditNotePort，阶段 10 替换；采购退货在采购发票已登记分支下调用 register_credit_note，红字发票由 invoice 模块登记 | 已裁定 |
| A-12 | ep-contract-inventory::AvailabilityQueryPort | 阶段 6 | 阶段 8 只提供 HTTP 端点 available-quantities，未提供 trait | 阶段 8，与 C-18 合并为同一 trait 的两个方法 | crates/contract/inventory/src/port/availability.rs 的 AvailabilityQueryPort，两个方法为 available 与 on_hand，配套 AvailabilityQuery 与 AvailabilityView；available 与端点 GET /api/v1/inventory/available-quantities 共用同一投影函数，reserved_quantity 按 U-G-01 的临时取值恒为零；阶段 7 的 StockAvailabilityQueryPort::on_hand 改指本 trait | 已裁定 |
| A-13 | MaterialUsageProbe 的实现 | 阶段 5 | 阶段 8 interfaces 未列 | 阶段 8 | 实现类型 InventoryMaterialUsageProbe 位于 crates/application/inventory/src/probe/material_usage.rs，实现 ep_contract_mdm::MaterialUsageProbe::has_stock_movement，取数为 inventory.stock_movements 上按 material_id 的存在性判定并命中索引 ix_stock_movements_legal_entity_id_material_id；阶段 5 的档案停用校验完整性验收顺延到阶段 8 | 已裁定 |
| A-14 | ProductUsageProbe 的实现 | 阶段 5 | 阶段 6 interfaces 未列 | 阶段 6，实现两份并在 wiring 取或 | ClmProductUsageProbe 位于 crates/application/clm/src/probe/product_usage.rs，取数为 clm.contract_lines 上 item_kind 为 PRODUCT 且所属合同状态为 EFFECTIVE；SalesProductUsageProbe 位于 crates/application/sales/src/probe/product_usage.rs，取数为 sales.sales_order_lines 上状态非 CANCELLED；组合类型 AnyProductUsageProbe 由阶段 5 在 ep-app-mdm 提供，任一为真即为真 | 已裁定 |
| A-15 | MasterReferenceCounter 与两个 TradeHistoryProvider 的实现 | 阶段 5 | 阶段 6、7、8、10、12 均未列为交付物 | 阶段 6、7、8、10、12 各实现本模块一份，聚合逻辑归阶段 5 | 十一个实现类型固定为 ClmReferenceCounter 与 SalesReferenceCounter 与 SalesTradeHistoryProviderImpl（阶段 6）、ProcureReferenceCounter 与 ProcureTradeHistoryProvider（阶段 7）、InventoryReferenceCounter（阶段 8）、InvoiceReferenceCounter 与 FinanceReferenceCounter 与 InvoiceSalesTradeHistoryProvider 与 InvoicePurchaseTradeHistoryProvider（阶段 10）、ServiceReferenceCounter（阶段 12），一律放在本模块 src/probe/ 下并注册到阶段 5 的 MasterReferenceCounterRegistry 与 TradeHistoryProviderRegistry；停用界面的计数覆盖模块由注册表实时枚举，未注册模块显式列为未覆盖；阶段 5 的完整性验收顺延到阶段 12 结束 | 已裁定 |
| A-16 | ep-contract-clm::ContractDerivationPlanQuery | 阶段 12 | 阶段 6 只提供 ContractQueryPort 与 ContractMilestonePort | 阶段 6 | crates/contract/clm/src/port/derivation.rs 的 ContractDerivationPlanQuery::derivation_plan 与 ContractDerivationPlan、ContractDerivationItem、ContractDerivationItemKind 四项；unique_key 取值规则固定为 contract_id、contract_version_no、item_kind 与 source_contract_line_id 或 milestone_no 四段以冒号连接，阶段 12 的派生任务以该键去重 | 已裁定 |
| A-17 | 销售退货单创建的命令端口与三类终态事件 | 阶段 12 | 阶段 6 提供表与 registered 事件，未提供创建命令 trait，也未提供到达终态、被作废、被驳回三类事件 | 阶段 6 | crates/contract/sales/src/port/sales_return.rs 的 SalesReturnCommandPort::create_sales_return 与 CreateSalesReturn、SalesReturnSourceRef、CreateSalesReturnLine、SalesReturnDeliveryLink、SalesReturnView；三个事件 sales.sales_return.closed.v1、sales.sales_return.cancelled.v1、sales.sales_return.rejected.v1 登记到 docs/event-catalog.md，既有的 registered 事件不变，阶段 6 的事件数由 14 增为 17；阶段 12 的状态机守卫按这三个事件名驱动 | 已裁定 |
| A-18 | 各模块的受治理数据集视图共十二个，连同 ledger 自备的一个合计十三个 | 阶段 11 | 只有 ledger.v_account_period_balances 由阶段 9a 提供。finance 两个、inventory 一个、clm 两个、sales 两个、invoice 一个、mdm 三个、project 一个均未列，原表把 invoice 的一个误记为 procure | 阶段 5、6、8、9a、10、12 各自发布本模块视图，阶段 11 只登记目录与消费 | mdm_customers 与 mdm_products 与 mdm_materials 对应 mdm.v_customers_dataset 等三视图归阶段 5；clm_contracts 与 clm_contract_delivery_milestones 与 sales_sales_orders 与 sales_order_delivery_batches 归阶段 6；inventory_stock_value_entries 归阶段 8；ledger_account_period_balances 归阶段 9a；invoice_purchase_invoices 对应 invoice.v_purchase_invoices_dataset 与 finance_receivable_ledger_entries 与 finance_payable_ledger_entries 归阶段 10；project_projects 归阶段 12。每个视图必须含 legal_entity_id、security_level、data_scope_tags 三列并在同一迁移中 GRANT SELECT 给 ep_analyst_ro，列签名由自检项 reporting-dataset-signature-matched 校验；阶段 7 不承担任何数据集视图 | 已裁定 |
| A-19 | ConfigItemApplier 的九个 item_kind 实现 | 阶段 13 | trait 定义在阶段 13，实现方阶段 3b、4、11 均未列为交付物，且时序倒挂 | trait 与注册表归阶段 3a，九个 applier 分派阶段 3b、4、11，六个自定义类归阶段 13b | crates/platform/release/src/port/config_item.rs 由阶段 3a 交付，含 ConfigItemApplier、ItemKind 十五项、ConfigPackageItem 与 ConfigItemApplierRegistry，方法签名中的 Tx 取自 ep-foundation；FlowDefinitionApplier 与 NotifyRuleApplier 归阶段 3b，AuthzRoleApplier 与 AuthzPolicyApplier 与 AuthzFieldGrantApplier 归阶段 4，ReportDefinitionApplier 与 MetricDefinitionApplier 与 DashboardDefinitionApplier 与 PrintTemplateApplier 归阶段 11，CUSTOM_ 与 UI_LAYOUT 六个归阶段 13b 的 ep-platform-meta | 已裁定 |
| A-20 | 各业务用例的能力域码与动作类别常量声明 | 阶段 13 | 十八项能力域码表与五项动作类别在阶段 13 才定义，各业务阶段无法提前声明 | 阶段 1 定义枚举并回写基线第 12 节，阶段 3b 至 12 各自声明常量，阶段 13 只做运行期判定 | crates/foundation/src/capability.rs 的 CapabilityDomain 十八项与 ActionClass 五项（Read、Write、Submit、Approve、Export），序列化取值与阶段 13 第 4.4 节能力域码表逐项一致；各模块在 `crates/contract/<module>/src/capability.rs` 中为每个用例声明 `<USECASE_SCREAMING>_DOMAIN` 与 `<USECASE_SCREAMING>_ACTION` 一对常量，xtask configdoc 断言每个 HTTP 路由都能解析到一对常量，缺失即构建失败 | 已裁定 |
| A-21 | 各模块把事件类型名登记到 ledger.posting_trigger_event_types | 阶段 9a | 阶段 6、7、8、10 均未列为交付动作 | 表与登记接口归阶段 9a，登记行归阶段 6、7、10，阶段 8 显式登记零行 | ledger.posting_trigger_event_types 与 ep_contract_ledger::PostingTriggerRegistry::register；阶段 6 回填 DELIVERY_CONFIRMED 与 SALES_RETURN，阶段 7 回填 PURCHASE_RECEIPT 与 PURCHASE_RETURN，阶段 10 回填其余八条含两条 INVOICE_REVERSED，YEAR_END_PL_CLOSING 一行的 event_type 保持为空；各阶段以 backfill 迁移做 UPDATE 定位 ledger_event_kind 并写入 event_type，不新增行；阶段 8 在计划中明写不登记任何行 | 已裁定 |
| A-22 | 处置流程对 DisposalPort 的实现 | 阶段 3b（预留桩） | 无人实现。规格要求物理删除只能由处置流程经专用路径与专用账号发起 | 阶段 14，与密钥销毁、备份保留期一并处理 | crates/platform/file/src/port/disposal.rs 的 DisposalPort 与 DisposalRequest、DisposalReceipt 由阶段 3b 定义；实现类型 OpsDisposalService 位于 crates/platform/obs/src/disposal.rs，只由 ops 专用路径与专用账号触发，执行前校验双人审批与重新认证凭证，执行后写审计并生成销毁证明；阶段 2、3、13 的物理删除路径一律指向该实现 | 已裁定 |
| A-23 | 各业务模块的四端界面 | 阶段 6、8、9、12、14 的 needs 均提到客户端阶段 | 阶段 13 只提供客户端壳、能力矩阵与制品，无任何业务界面 | 阶段 5 至 12 各自实现本模块界面 | 目录固定为 `clients/desktop/src/modules/<module>/` 与 `clients/mobile/src/modules/<module>/`，每模块一个；桌面用例用 Playwright 与 tauri-driver，移动用例用 XCUITest 与 Espresso；规格第 6.2 章能力矩阵取值为完整或简化的能力域必须实现四端界面，VIEW_ONLY 只实现只读视图，NOT_APPLICABLE 不实现入口；阶段 13 不交付任何业务界面，阶段 14 按各阶段交付情况汇总验收矩阵 | 已裁定 |
| A-24 | 期初与历史数据导入通道（应收应付预收预付、资金账户期初） | 阶段 8、9、10 的 needs 提到数据迁移阶段 | 十四个阶段中没有独立的数据迁移阶段 | 不设独立阶段，四个通道分归阶段 9a、10、8 | 总账期初经 ledger.opening_balance_batches 与端点 POST /api/v1/ledger/opening-balance-batches 及其 confirm 动作（阶段 9a）；应收应付预收预付期初经用例 import_opening_balances.rs 与端点 POST /api/v1/finance/opening-balances/actions/import，source_doc_type 取 MIGRATION_OPENING（阶段 10）；资金账户期初经 finance.cash_accounts.opening_balance 与 opening_balance_period_id，建档时一次录入且建档后不可修改（阶段 10）；库存期初经 MIGRATION_STOCK_ADJUSTMENT 与 MovementReason::MigrationOpening（阶段 8）；四个通道一律不生成凭证，两侧平衡由 finance 的八个勾稽视图在首个会计期间校验；各阶段计划中数据迁移阶段的措辞一律删除 | 已裁定 |
| A-25 | ep-adapter-esign crate 本体 | 阶段 3b、6 | 阶段 6 提供 integration-gateway 的两个内部端点，未登记 crate | 阶段 6，真实对接验证在阶段 14 | crate 名 ep-adapter-esign，目录 crates/adapter/esign/，只依赖 ep-foundation 与 ep_domain_clm::port::SignatureGateway，装配进 integration-gateway；内部端点 POST /internal/v1/esign/requests 与 GET /internal/v1/esign/requests/{external_request_id} 只监听 127.0.0.1:8082；契约测试 crates/adapter/esign/tests/contract_sandbox.rs 与 contract_stub.rs 共用同一组断言函数；阶段 14 提交沙箱通过记录或规格附录 B 允许的等效验证证据 | 已裁定 |
| A-26 | platform_ops 最小台账在阶段 14 之前的可用性 | 阶段 1、2、3、4、9、11、13 | 全部 platform_ops 表由阶段 14 提供，但阶段 1 的自检项 offsite-sink-requirements、阶段 2 的降级台账、阶段 11 的查询超限记录都更早需要 | 阶段 2 建表与写入端口，阶段 14 扩展为十九表五视图 | platform_ops.degradation_windows 与两条约束 ux_degradation_windows_kind_scope_closed 与 ck_degradation_windows_open_order，不带 legal_entity_id 且不建策略，带 scope_legal_entity_id 与 scope_accounting_period_id 两个可空标注列；ep-platform-obs 的 DegradationLedger 三方法 open、close、open_count；DegradationKind 初始两项 OFFSITE_SINK_NOT_CONFIGURED 与 WRITER_ROLE_CONTAINMENT_MISSING，阶段 14 扩展到十八类；指标 ep_degradation_windows_open 由阶段 2 注册并填充；阶段 1 早于阶段 2，只写 stderr 并留 TODO 注释 | 已裁定 |
| A-27 | ep-platform-release 配置发布通道在阶段 13 之前的可用性 | 阶段 2、3、5、6、7、9、10、11 | 八个阶段需要，阶段 13 才提供 | 端口归阶段 3a，最小发布通道归阶段 3b，扩展归阶段 13b | 阶段 3a 只交付 crates/platform/release/src/port/config_item.rs；阶段 3b 交付 platform_meta.config_packages 与 config_package_items 与 config_release_orders 三表、发布与回退用例、ConfigItemApplierRegistry 的运行期装配以及两个 applier，状态机只实现 Draft 与 PendingReview 与 PendingApproval 与 Approved 与 Released 与 RolledBack 六态，签名算法 ECDSA P-256；阶段 13b 建 config_item_apply_logs 与 config_edit_locks 并扩展为十一态，加入 PendingAutotest 与 TestPassed；阶段 2 不使用该通道，其敏感字段登记与密钥域配置直接经迁移与端点写入 | 已裁定 |
| A-28 | 字段元数据登记入口（把开户银行与银行账号密级设为 30） | 阶段 5 | 阶段 13 才提供 platform_meta，时序倒挂 | 登记表归阶段 2 与阶段 4，登记行归阶段 5 | 阶段 5 追加 mdm 的 backfill 迁移，向 platform_core.sensitive_field_registry 插入两行，schema_name 取 mdm，table_name 取 customers 与 suppliers，column_name 取 bank_name 与 bank_account_no，security_level 取 30，is_field_encrypted 取 true，blind_index_column 取 bank_account_no_bidx；字段级授权行经配置发布通道在阶段 5 之后写入 platform_authz.field_permissions，阶段 5 交付时按默认拒绝处理；阶段 5 不依赖 platform_meta，阶段 13 不承担该登记 | 已裁定 |

### 4.2 B 类：有人提供但无人使用

这一类不一定是错误，但每一条都要给出结论，要么找到使用者，要么明确它只服务于 CI 或运维。

| 编号 | 事项 | 提供方 | 现状 | 最终归属 | 确切标识符 | 状态 |
|---|---|---|---|---|---|---|
| B-01 | POST /api/v1/system/echo 与 ci_probe schema | 阶段 1 | 仅 CI 探针使用 | 阶段 1 提供，阶段 14 校验，保留 | Cargo feature 名固定为 ci-probe，在 apps/core-server/Cargo.toml 与 testkit/Cargo.toml 中声明且默认关闭，路由与 ci_probe.probe_records 的建表函数一律带 `#[cfg(feature = "ci-probe")]`；发布门禁项 RG-CI-PROBE-ABSENT 的判据为发布制品的 cargo tree -e features 输出不含 ci-probe 且镜像内不含符号 api_v1_system_echo | 已裁定 |
| B-02 | platform_core.append_only_registry | 阶段 2 | 阶段 8、9、10 有大量仅追加表，但三个阶段的 needs 均未提到要登记 | 登记表归阶段 2，登记行归阶段 8、9a、10 | 登记列为 schema_name、table_name、immutable_columns；阶段 8 登记 inventory.stock_movements 与 stock_qty_entries 与 stock_value_entries 与 variance_splits，阶段 9a 登记 ledger.vouchers 与 voucher_lines 与 general_vouchers，阶段 10 登记 finance 的 receivable_entries 与 payable_entries 与 advance_receipt_entries 与 advance_payment_entries 与 unbilled_ar_entries 与 overbilling_entries 与 cash_ledger_entries；检查脚本 db/checks/append_only_consistency.sql 由 xtask sqlcheck 执行 | 已裁定 |
| B-03 | platform_core.migration_windows 与 open-window 校验 | 阶段 2 | 唯一使用者是阶段 13 的在线 DDL 计划，阶段 13 needs 未提及 | 阶段 13b 显式接入 | 阶段 13b 的 DDL 执行段在开始前调用 ep_platform_release::MigrationWindowGuard::assert_open(tx)，该守卫由阶段 2 提供并列为对外可用组件；未持有窗口时返回 PLATFORM.DB.MIGRATION_WINDOW_CLOSED，HTTP 409，category 为 BUSINESS_CONFLICT | 已裁定 |
| B-04 | derive_blind_key 与 BlindIndex | 阶段 2 | 阶段 10 的银行账号查重使用，但其 needs 写的是哈希加盐 | 阶段 2 提供，阶段 10 使用 | 列名固定为 bank_account_no_bidx bytea，取值为 derive_blind_key 以 legal_entity_id 与 finance.cash_accounts.bank_account_no 与明文三参数派生，唯一约束 ux_cash_accounts_legal_entity_id_bank_account_no_bidx；mdm 的客户与供应商银行账号同名同构；阶段 10 的哈希加盐措辞作废，不得自建第二套哈希 | 已裁定 |
| B-05 | WasmComputePort 与 RuleEvaluator 端口 | 阶段 3b | 实现方是阶段 13 的 plugin-host 与规则求值端点，阶段 13 needs 未提及这两个端口 | 端口归阶段 3b，实现归阶段 13b | ep_platform_flow::port::WasmComputePort 的实现类型 PluginHostWasmCompute 位于 crates/adapter/wasm/ 并装配进 plugin-host；ep_platform_flow::port::RuleEvaluator 的实现类型 AstRuleEvaluator 位于 crates/platform/meta/src/rule/ 并装配进 core-server；端点 POST /api/v1/platform/rule-evaluations/actions/evaluate 只调用 AstRuleEvaluator，不新建求值路径 | 已裁定 |
| B-06 | ep-contract-service::EquipmentQuery | 阶段 12 | 无声明使用者 | 阶段 12 撤销 | 删除 crates/contract/service/src/port/equipment.rs；设备的跨模块可见性只保留三条路径，即 GET /api/v1/service/equipments 与其单条端点、全文检索索引中的 service.equipment_records 文档、阶段 12 自身的 EquipmentsSectionProvider；客户 360 的设备区块由 ep-app-service 自己的 Customer360SectionProvider 实现，报表侧一律经受治理数据集视图取数 | 已裁定 |
| B-07 | ep-contract-procure::PurchaseReturnLinkPort | 阶段 7 | 使用者是阶段 6 的直运销售退货勾稽，阶段 6 早于阶段 7 且未声明使用 | 阶段 7 提供与接入 | ep_contract_procure::PurchaseReturnLinkPort::link_drop_ship_return 接受 sales_return_id 与 DropShipReturnLine 清单并返回 PurchaseReturnLinkView；阶段 6 在 wiring 注入 NoopPurchaseReturnLinkPort 并把直运退货勾稽验收标为顺延，阶段 7 替换该行并在退出条件中加入端到端通过一条 | 已裁定 |
| B-08 | finance.v_recon_inventory 与 v_recon_grni 两个视图外壳 | 阶段 10 | 子账侧取数分别在阶段 8 与阶段 7，两阶段均未声明要接入 | 视图归阶段 10，子账侧查询函数归阶段 8 与阶段 7 | ep_contract_finance::ReconciliationItemQuery 与 SubledgerBalanceProvider::balance 由阶段 10 定义；实现类型 InventorySubledgerBalanceQuery 由阶段 8 提供该法人该期间的存货金额账合计，GrniSubledgerBalanceQuery 由阶段 7 提供已收货未收票暂估合计；两者的实现体由阶段 8 与阶段 7 各自以本模块查询函数形式先行交付，阶段 10 在交付时包装接线；阶段 7 与阶段 8 在退出条件中各加一条 | 已裁定 |
| B-09 | inventory.stock_value_adjusted.v1 | 阶段 8 | 声明消费者是报表数据集，阶段 11 needs 未提及 | 事件归阶段 8，消费者归阶段 11，保留该事件 | 消费者名固定为 costing.stock_value_adjust，位于 crates/application/costing/src/consumer/stock_value_adjust.rs，在 job-worker 注册，幂等由 platform_msg.inbox_consumptions 的 consumer 与 event_id 保证，副作用为向 costing.cost_entries 补记只影响金额账的调整对应的成本条目；阶段 8 计划中该事件的消费者由报表数据集改写为该名字 | 已裁定 |
| B-10 | ep-contract-mdm::SupplierSelfServiceCommand | 阶段 5 | 阶段 7 的门户 supplier-profile 使用，但其 needs 用的是另一套措辞 | 阶段 5 提供，阶段 7 使用 | ep_contract_mdm::SupplierSelfServiceCommand 两个方法固定为 submit_profile_change（接受 supplier_id 与 SupplierProfilePatch，返回 SupplierChangeRequestView）与 upload_qualification（接受 supplier_id 与 QualificationUpload）；阶段 7 门户 supplier-profile 一节的另一套措辞删除 | 已裁定 |
| B-11 | ep-bench 与 ep-release-gate | 阶段 14 | 不随产品交付 | 阶段 14，保留并排除出制品 | 两个 crate 位于 tools/bench/ 与 tools/release-gate/，不在 crates/ 下；发布门禁项 RG-TOOLS-EXCLUDED 的判据为 SBOM 中不含 ep-bench 与 ep-release-gate 两个包名；阶段 1 的 xtask sbom 增加同名断言的负样例 | 已裁定 |

### 4.3 C 类：同一事物被两个阶段都声称提供

这一类必须逐条二选一，不允许两边都做。

| 编号 | 事项 | 声称方 | 冲突点 | 最终归属 | 确切标识符 | 状态 |
|---|---|---|---|---|---|---|
| C-01 | 二十四个 schema、七个功能角色、二十四个属主角色、db/bootstrap 引导脚本、order.toml | 阶段 1 与阶段 2 | 两边都列为交付物，且阶段 1 只列三个迁移文件而阶段 2 列三十二个 | 阶段 2 | db/bootstrap 的 00_database.sql、01_roles.sql、02_cluster_params.sql、03_role_defaults.sql、04_pg_hba.fragment 五个文件名以阶段 2 为准，阶段 1 的 B001 与 B002 与 B003 三个文件名作废；db/migrations/order.toml 的二十四项顺序以阶段 2 第 3.3 节为准；阶段 1 只交付二十四个空目录与只含注释与顺序数组骨架的 order.toml，其自检项 rls-enabled-and-forced 与 runtime-role-privileges-bounded 在阶段 1 交付时以 ci_probe 探针表为被测对象；阶段 1 关于 DELETE 授权的决定移交阶段 2 | 已裁定 |
| C-02 | tools/ep-migrate CLI | 阶段 1 与阶段 2 | 子命令完全不同：阶段 1 为 migrate/verify/status/manifest，阶段 2 为 apply/status/check/gen-rls/open-window | 阶段 2，阶段 1 只交付骨架与退出码约定 | 子命令固定为 apply、status、check、gen-rls、open-window 五个；阶段 1 的 migrate 并入 apply，verify 并入 check，manifest 并入 status --format=manifest；退出码固定为 0 成功、2 参数错误、3 迁移窗口未打开、4 校验和不符、5 版本不一致、78 环境自检失败 | 已裁定 |
| C-03 | UnitOfWork 的事务方法名 | 阶段 1 与阶段 2 | 阶段 1 为 transact 与 transact_repeatable_read，阶段 2 为 transact 与 snapshot_transact | 阶段 1 定义，阶段 2 实现 | 两个方法为 transact 与 snapshot_transact，签名见 A-01，transact_repeatable_read 作废；ep-foundation 定义，ep-adapter-db 提供实现骨架，ep-adapter-db-pg 提供实现；基线第 10.3 节在示例之后追加一句，只读快照事务的唯一入口是 snapshot_transact，配合 SET TRANSACTION SNAPSHOT 使用 | 已裁定 |
| C-04 | PoolKind、RetryPolicy、SessionContext、ConnectionBudget | 阶段 1 与阶段 2 | 两边都在 ep-adapter-db 中定义 | 类型归阶段 1，取值与脚本归阶段 2 | 四个类型全部留在 ep-adapter-db 不进 ep-foundation；PoolKind 五项为 Rw、Ro、Worker、Integ、Ops，SessionContext 四字段为 legal_entity_id、user_id、request_id、trace_id；取值固定为 max_attempts 3、backoff_ms 50 与 150 与 450、retryable_sqlstates 40001 与 40P01、resident_max 42、burst_max 52；校验脚本 scripts/verify-connection-budget.sh 归阶段 2 | 已裁定 |
| C-05 | tests/rls_matrix | 阶段 1、2、4 | 三个阶段都声称提供 | 阶段 1、2、4 三段分工 | CI 目标名固定为 tests/rls_matrix；阶段 1 在 testkit/src/rls_matrix.rs 提供 assert_read、assert_write、assert_update、assert_delete、assert_aggregate、assert_sort、assert_report_projection、assert_error_leak 八个断言函数；阶段 2 追加 assert_replication_role_containment 与 assert_recon_context_borrow；阶段 4 追加 matrix_32.rs 与发布门禁项 RG-RLS-MATRIX-GREEN；三个阶段不得重复实现同名函数 | 已裁定 |
| C-06 | sensitive_field_registry | 阶段 2（platform_core）与阶段 4（platform_authz） | 同名表落在两个 schema | 阶段 2 | platform_core.sensitive_field_registry，列为 schema_name、table_name、column_name、security_level smallint、is_field_encrypted boolean、blind_index_column text、mask_style text，唯一约束 ux_sensitive_field_registry_schema_table_column；阶段 4 只引用不建表，其 platform_authz.sensitive_field_registry 的建表迁移删除，全部出现处改指 platform_core | 已裁定 |
| C-07 | 幂等键的三段职责 | 阶段 1（中间件）、阶段 2（IdempotencyStore 端口）、阶段 3a（表与重放实现） | 未冲突但未写清分工，容易做成三套 | 阶段 1 校验请求头，阶段 2 定义端口，阶段 3a 建表并实现重放 | 阶段 1 的中间件名固定为 IdempotencyKeyHeaderGuard，只校验 Idempotency-Key 头存在且为合法 UUIDv7，不合法返回 PLATFORM.IDEMPOTENCY.KEY_REQUIRED；阶段 2 定义 ep_adapter_db::port::IdempotencyStore 的 try_begin 与 finish 及 IdempotencyScope（legal_entity_id、user_id、endpoint、key）；阶段 3a 建 platform_msg.idempotency_keys 并返回 IdempotencyOutcome 的 FirstCall、Replay、PayloadMismatch 三态；三处不得各自判等 | 已裁定 |
| C-08 | 账龄分档 | 阶段 10（finance.aging_bucket_definitions）与阶段 11（reporting.aging_bucket_profiles 与 aging_bucket_lines） | 两套分档表 | 阶段 11 | 唯一出处为 reporting.aging_bucket_profiles 与 reporting.aging_bucket_lines，唯一取用入口为 ep_contract_reporting::AgingBucketQuery::buckets；阶段 10 的 finance.aging_bucket_definitions 在其计划中标注为临时表；阶段 11 交付两个迁移文件，一个把分档从 finance 迁入 reporting，一个删除 finance 侧表，两个文件分别放在 db/migrations/reporting/ 与 db/migrations/finance/ | 已裁定 |
| C-09 | 客户 360 | 阶段 5（/overview 与 CustomerPanelProvider）与阶段 12（/customer-360 与 Customer360SectionProvider） | 端点与契约各一套 | 阶段 12 | 唯一端点 GET /api/v1/crm/customers/{id}/customer-360，唯一契约 ep_contract_crm::Customer360SectionProvider；ep_contract_crm::CustomerPanelProvider 作废，阶段 5 直接实现 Customer360SectionProvider 并只挂载 mdm 自己的区块，不保留 /overview；阶段 12 接管后追加其余区块，不新增路径 | 已裁定 |
| C-10 | 供应商风险记录 | 阶段 5（mdm.supplier_risk_records）与阶段 7（procure.supplier_risk_records） | 两张同义表 | 风险记录归阶段 5，质量记录归阶段 7 | 保留 mdm.supplier_risk_records，撤销 procure.supplier_risk_records，保留 procure.supplier_quality_records；阶段 7 经阶段 5 提供的 ep_contract_mdm::SupplierRiskRecordPort 的 append 与 list 读写；阶段 7 计划第 3.2.3 节整节删除并顺延其后迁移序号 | 已裁定 |
| C-11 | 税率字典 | 阶段 5（mdm.classification_items 承载税率预置）与阶段 10（invoice.tax_rate_options） | 两处取值 | 阶段 10 | 唯一出处为 invoice.tax_rate_options，取用入口为 ep_contract_invoice::TaxRateOptionQuery 的 default_rate 与 list；阶段 5 的 classification_items 去掉税率一类，临时取值由桩类型 MdmTaxRateStub 承担；阶段 10 交付税率迁移文件并撤销该桩；阶段 6 取默认税率一律经 ep-contract-invoice，不经 ep-contract-mdm | 已裁定 |
| C-12 | 收货入账单价的固化位置 | 阶段 7（procure.goods_receipt_line_costings 含单价）与阶段 8（InventoryPricingLookupPort 按来源单据行回查单价） | 同一事实存两份，可能不一致 | 阶段 8 | 权威出处为 inventory.stock_value_entries.applied_unit_price；procure.goods_receipt_line_costings 删去单价列，只保留 goods_receipt_line_id、quantity、amount、allocation_kind、source_purchase_invoice_line_id；单价一律经 ep_contract_inventory::InventoryPricingLookupPort::original_unit_price_by_source_line 回查 | 已裁定 |
| C-13 | 取价职责的归属 | 阶段 7 期望 ledger 返回逐行入账分配与取价分支，阶段 8 由 inventory 承担全部取价三分支 | 两套取价实现 | 阶段 8 | 撤销阶段 7 的 PurchaseReceiptPostingPort 与 PurchaseReturnPostingPort 两个 needs；收货登记在同一事务内依次调用 InventoryPostingPort::post_inbound 与 PostingPort::post，采购退货依次调用 post_outbound 与 post，价差拆分调用 InventoryVariancePort::split_variance；ledger 侧不提供任何取价方法，阶段 7 与阶段 9 计划中各写一句不自行取价 | 已裁定 |
| C-14 | 信用敞口查询的三个名字 | 阶段 6 提供 sales::CreditExposureQueryPort，阶段 6 需要 finance::CustomerCreditExposurePort，阶段 10 提供 finance::CreditExposureQuery | 一个概念三个名字 | 对外入口归阶段 6，取数来源归阶段 10 | ep_contract_sales::CreditExposureQueryPort::exposure 返回 credit_limit、in_transit_amount、delivered_unbilled_amount、receivable_open_amount 与 available_amount；取数来源为 ep_contract_finance::ReceivableExposureQuery::exposure，只返回 receivable_open_amount 与 delivered_unbilled_amount；finance::CreditExposureQuery 与 finance::CustomerCreditExposurePort 两名作废；阶段 6 先注入 NoopReceivableExposureQuery，阶段 10 替换 | 已裁定 |
| C-15 | 应付查询端口命名 | 阶段 7 需要 PayableQueryPort 与 PayableStatementQueryPort，阶段 10 提供 PayableLedgerQuery 与 SupplierStatementQuery | 名字不一致 | 阶段 10 | ep_contract_finance::PayableLedgerQuery::open_balance 接受 purchase_invoice_id 返回 Money，ep_contract_finance::SupplierStatementQuery::statement 接受 supplier_id 与 PeriodRange 返回 SupplierStatementView；阶段 7 的 PayableQueryPort 与 PayableStatementQueryPort 作废，门户对账端点的取数改经 SupplierStatementQuery | 已裁定 |
| C-16 | 发票状态查询端口命名 | 阶段 6 需要 InvoiceStatusPort，阶段 10 提供 SalesInvoiceQuery 与 InvoiceReversalStatusQuery | 名字不一致 | 阶段 10 | ep_contract_invoice::SalesInvoiceQuery::by_sales_order_line 与 ep_contract_invoice::InvoiceReversalStatusQuery::is_fully_credit_noted；阶段 6 的 InvoiceStatusPort 作废，阶段 6 先注入空实现，其 SALES.SALES_RETURN.INVOICE_NOT_CREDIT_NOTED 判定顺延到 M7 | 已裁定 |
| C-17 | 采购需求派生端口命名 | 阶段 6 需要 PurchaseRequisitionDerivationPort，阶段 7 提供 PurchaseRequisitionIntakePort | 名字不一致 | 阶段 7 | ep_contract_procure::PurchaseRequisitionIntakePort::intake 接受 PurchaseRequisitionIntake（含 source_module、source_doc_id、source_doc_line_id、material_id、quantity、required_on、unique_key）返回 PurchaseRequisitionView；阶段 6 的 PurchaseRequisitionDerivationPort 作废并先注入 NoopPurchaseRequisitionIntakePort；阶段 12 的 project.project_task.requisition_requested.v1 下游同走该端口 | 已裁定 |
| C-18 | 库存过账端口命名 | 阶段 7 需要 StockInboundPort 与 StockOutboundPort 与 StockAvailabilityQueryPort，阶段 8 提供 InventoryPostingPort 的三个方法 | 名字与粒度均不一致 | 阶段 8 | ep_contract_inventory::InventoryPostingPort 三个方法固定为 post_inbound、post_outbound、find_movement_by_source；阶段 7 的 StockInboundPort 与 StockOutboundPort 与 StockAvailabilityQueryPort 三个名字作废，可用量由 AvailabilityQueryPort 承接；阶段 8 在计划第 5 节之后新增一小节列出五个 trait 的完整方法签名 | 已裁定 |
| C-19 | 合同派生项目任务的机制 | 阶段 6 需要 ProjectTaskDerivationPort（同步调用），阶段 12 通过消费 clm.contract.effective.v1 自行派生 | 同步与事件两套机制 | 阶段 12 消费，阶段 6 提供查询 | 撤销 ep_contract_project::ProjectTaskDerivationPort 与阶段 6 任何同步派生项目任务的措辞；阶段 12 的消费者名固定为 project.contract_derivation，消费 clm.contract.effective.v1，幂等键取 A-16 的 unique_key；阶段 6 改为提供 ContractDerivationPlanQuery | 已裁定 |
| C-20 | 收款计划的派生方 | 阶段 6 需要 finance::ReceivablePlanPort 派生收款计划，阶段 6 自己已有 clm.contract_payment_schedules | 同一事实两处 | 阶段 6 | 唯一表为 clm.contract_payment_schedules，撤销 ep_contract_finance::ReceivablePlanPort；阶段 10 的到款自动核销按合同收付款计划取数，经阶段 6 提供的 ep_contract_clm::ContractPaymentScheduleQuery::schedules | 已裁定 |
| C-21 | 事务重试指标名 | 阶段 1 的 ep_db_retries_total、阶段 2 的 ep_db_tx_retries_total、阶段 3a 的 ep_tx_retry_total | 三个名字指同一事物 | 注册与填充均归阶段 2 | ep_db_tx_retries_total，类型 counter，标签 pool（rw、ro、worker、integ、ops）与 sqlstate（40001、40P01）；阶段 1 的 ep_db_retries_total 与阶段 3a 的 ep_tx_retry_total 两个登记撤销；docs/metrics-catalog.md 由 CI 校验唯一性 | 已裁定 |
| C-22 | 复制交叉核对指标名 | 阶段 2 的 ep_db_replication_crosscheck_age_seconds 与阶段 14 的 ep_replication_crosscheck_age_seconds | 两个名字 | 注册归阶段 14，填充归阶段 2 | ep_replication_crosscheck_age_seconds，类型 gauge，标签 channel（archive、backup）；阶段 2 的 ep_db_replication_crosscheck_age_seconds 作废，其计划中该指标标注为由阶段 14 注册本阶段只填充 | 已裁定 |
| C-23 | 数据库连接池指标 | 阶段 1 与阶段 2 都登记 ep_db_pool_connections 与 ep_db_statement_duration_seconds | 重复登记 | 注册归阶段 1，填充归阶段 2 | ep_db_pool_connections（gauge，标签 pool）与 ep_db_statement_duration_seconds（histogram，标签 pool 与 statement_kind），两者在 crates/platform/obs/src/metrics/registry.rs 中由阶段 1 一次性注册；docs/metrics-catalog.md 的唯一性校验在阶段 1 的 xtask 中实现 | 已裁定 |
| C-24 | 错误码 PLATFORM.IDEMPOTENCY.KEY_REQUIRED 与 PLATFORM.CAPACITY.CONCURRENCY_LIMIT | 阶段 1 与阶段 3a、阶段 4 | 重复登记 | 阶段 1 | 两个常量位于 crates/foundation/src/error/codes.rs 并登记在 docs/error-codes.md 的 PLATFORM 段，KEY_REQUIRED 为 VALIDATION 与 HTTP 400 与不可重试，CONCURRENCY_LIMIT 为 INFRASTRUCTURE 与 HTTP 503 与可重试；同批由阶段 1 登记的还有 PLATFORM.IDEMPOTENCY.PAYLOAD_MISMATCH、PLATFORM.CONCURRENCY.STALE_VERSION、PLATFORM.AUTHZ.NOT_FOUND_OR_DENIED、PLATFORM.AUTHZ.OBJECT_FORBIDDEN、PLATFORM.DB.MIGRATION_WINDOW_CLOSED；阶段 3 与阶段 4 的清单中删去这七个 | 已裁定 |
| C-25 | 启动自检项的编号 | 阶段 3b 增四项无编号，阶段 4 称新增第 14 至 16 项，阶段 11 称新增第 14 项，阶段 13 称新增第 14 至 16 项 | 三个阶段抢同一批序号 | 阶段 1 定义注册表与十三个基线项名，各阶段追加命名项 | SelfCheckRegistry 位于 crates/platform/runtime/src/selfcheck/registry.rs，注册项为 SelfCheckItem 且 name 取 kebab-case；基线十三项为 config-parsed、database-reachable、migration-version-matched、rls-enabled-and-forced、runtime-role-privileges-bounded、secrets-resolvable、audit-chain-verifiable、file-store-writable、clock-skew-within-limit、cgroup-quota-matched、offsite-sink-requirements、license-and-modules-consistent、current-period-open；阶段 3b 追加 audit-evidence-store-writable 与 audit-signing-key-usable 与 attachment-store-ready 与 event-catalog-consistent，阶段 4 追加 duty-class-exclusivity 与 forbidden-permission-items-absent 与 authz-snapshot-loadable，阶段 5 追加 master-data-usage-probes-registered，阶段 11 追加 reporting-dataset-signature-matched，阶段 13 追加 client-capability-matrix-frozen 与 custom-object-ddl-consistent；报告按注册顺序输出，基线十三项在前，任何阶段不得再用序号称呼 | 已裁定 |
| C-26 | 单据类型码的全局唯一性 | 阶段 4、5、6、7、9、10、11、12 各自分配 | 阶段 7 的八类单据未分配类型码；其余七个阶段共分配三十余个码，无全局核对 | 登记文件归阶段 1，各码归其单据所在阶段 | docs/data-dictionary.md 的单据类型码一节为唯一登记处，CI 校验项名固定为 xtask configdoc --check-doc-type-codes，判据为该表与 ep-platform-sequence 的常量表逐项一致且无重复；全量四十一个码为阶段 4 的 BGA 与 HRR，阶段 5 的 CUST 与 SUPP 与 MATL 与 PROD 与 PRLS 与 MDCR 与 MDIB 与 MDEX，阶段 6 的 CT 与 SO 与 SR 与 DC，阶段 7 的 PR 与 PO 与 GR 与 RJ 与 PRT 与 PAYR 与 DN 与 SIU，阶段 9 的 OBB 与 GV 与 PCR 与 YEC，阶段 10 的 INVA 与 SINV 与 IRVS 与 RCPT 与 PAYM 与 RFND 与 CDRV 与 OBST 与 PINV，阶段 11 的 RT，阶段 12 的 EQ 与 CPL 与 WO 与 PRJ 与 PT；DC 与 PINV 是本次新增 | 已裁定 |
| C-27 | 审计证据目录的属主与写出者 | 阶段 3b 定义 /var/lib/ep/audit-evidence 属主 ep-worker，阶段 14 由 archive-writer 写出到落点 | 属主与写出者不同 | 写入归阶段 3b，写出归阶段 14，不冲突 | 目录 /var/lib/ep/audit-evidence，属主 ep-worker，组 ep，权限 0750；job-worker 写入证据文件并做段根签名，archive-writer 以组 ep 的只读权限读取并写出到服务器之外落点，不具备写入与删除权限；两个阶段各自在计划中补一句 | 已裁定 |
| C-28 | 关账受理前提二的统计口径 | 阶段 9a 的 v_pending_posting_backlog 统计待消费过账条目，阶段 10 第 0.1 节主张凭证在业务事务内同步生成 | 若凭证同事务生成，则不存在待过账队列，受理前提二失去判据 | 口径归阶段 9a，措辞在阶段 4、9、10 三处逐字一致 | 全部凭证一律与业务事件同事务生成，Outbox 只承载派生、通知、检索与报表数据集；受理前提二的判定语句固定为该法人该期间内 platform_msg.outbox_events 中 status 属于 PENDING 或 DISPATCHING 且 posting_date 落在该期间起止之间且 event_type 命中 ledger.posting_trigger_event_types 的条目数为零，且 platform_msg.dead_letters 中 state 属于 OPEN 或 REPAIRING 并同样命中该注册表的条数为零；posting_date 为空的平台事件不计入；视图名 ledger.v_pending_posting_backlog，错误码 LEDGER.PERIOD_CLOSE_REQUEST.PENDING_POSTING_BACKLOG 与 LEDGER.PERIOD_CLOSE_REQUEST.UNREPAIRED_DEAD_LETTERS | 已裁定 |

### 4.4 核对表的关闭方式

A 类二十八条中，A-01、A-02、A-03、A-09、A-10、A-19、A-20、A-26、A-27 九条是阻塞项，不关闭则相应阶段无法开工或黄金闭环无法连通，必须在阶段 1 计划定稿前回写完毕。其余十九条在各自阶段开工前回写。A-19 与 A-20 之所以进入阻塞清单，是因为 ConfigItemApplier 端口与能力域码枚举分别是阶段 4 与全部业务阶段的编译期前提。

B 类十一条不阻塞开工，但每一条都要在对应阶段的计划中给出一句结论，其中 B-06 的结论是撤销该 trait，阶段 12 的 crate 表须删去对应行。

C 类二十八条已全部二选一，落点见每一行的最终归属列与确切标识符列。其中十四条涉及命名，命名一旦落到代码里再改就是全仓改动，因此阶段 1 与阶段 2 定稿前必须先把这十四条的名字写进基线与四份登记文件。

## 5. 里程碑与验收节奏

十四个阶段划出十二个可演示节点。演示的判据是能在一台服务器上从客户端或接口跑完，不允许用测试夹具直接写库来构造前置数据。里程碑按阶段编号命名，其实际达成次序按第 3.4 节的链条，即 M1、M2、M3、M6、M5、M4、M7 之后 M8 与 M10 并行，随后 M9、M11、M12。

| 里程碑 | 时点 | 演示内容 | 判据 |
|---|---|---|---|
| M1 | 阶段 2 结束 | 空系统冷启动。八进程逐个启动、--check 报告逐项绿、迁移从零执行到最新、两个法人的行级隔离演示（同一 SQL 在两个法人上下文下返回不同结果集，无上下文时返回空） | 连接预算枚举与规格第 7.7 章一致；无 BYPASSRLS 角色；越权矩阵的数据库侧断言全绿 |
| M2 | 阶段 4 结束 | 平台内核闭环。登录并完成 MFA、登记设备、切换法人、发起一次高风险操作并被重新认证拦截、通过审批后执行、然后验证审计链 | 六类高风险操作的允许与拒绝两条路径均可演示；审计链验证工具对最近一段返回可验证；32 组越权矩阵全绿 |
| M3 | 阶段 5 结束 | 主数据闭环。Excel 导入 500 条客户、其中若干行报错并可下载错误清单、修正后重导、发起一次客户变更申请并审批生效、查看档案版本快照、停用一个物料并被引用校验阻断 | 导入导出往返一致；档案版本可比对；停用阻断给出具体阻断项 |
| M4 | 阶段 6 结束 | 合同到订单再到交付确认。录入合同并触发价格权限校验、走完四条审批链、经签章沙箱回传签署文件、合同生效后派生销售订单与分批交付行、修改合同并重新派生、建立交付确认单并确认一次 | 派生幂等（重复投递三次结果一致）；续签与合并各一次；交付确认的库存腿与凭证腿真实生效，过渡科目腿注入 NoopUnbilledArPort 且其净额断言顺延到 M7；信用敞口以 NoopReceivableExposureQuery 返回并明示为未完整；直运退货勾稽顺延到阶段 7 |
| M5 | 阶段 8 结束 | 库存内核。采购收货过账、移动加权平均单价重算、并发出库、结存归零出清、价差拆分的三种覆盖情形、序列号扫码校验 | ep-testkit 的两账同源、数量守恒、存货勾稽三组断言全绿；并发出库场景下单价重算无丢失更新 |
| M6 | 阶段 9a 结束 | 会计内核与对账框架。业务事件同事务生成凭证、试算平衡、会计恒等式、跨期记账日期的顺延入账并在凭证上标注 deferred_from_period_id、总账期初余额批次导入、对账框架的一次分批执行 | 借贷平衡属性测试通过；顺延凭证可按原始业务日期与按会计期间两条路径检索；ReconExecutor 在单一法人上下文下逐批执行并落 recon_runs 与 recon_discrepancies |
| M7 | 阶段 10 结束 | 黄金业务闭环首次全程贯通。规格第 8 章十四步中的第 1 至 11 步端到端跑完，含分批订货、分次到款、分次付款、发票红字冲销、销售退货、采购退货、信用超额转审批七种基础分支，其中第 5 步的进项发票登记与三单匹配、第 8 步的交付确认单三腿必须真实执行 | 十项勾稽差额为零；交付确认的过渡科目腿由本阶段替换空实现后净额断言成立；进项发票的暂估回冲与价差拆分经 InventoryVariancePort::split_variance 取数；发票红冲判定经 InvoiceReversalStatusQuery::is_fully_credit_noted；注入一笔差额后对账差异事项生成；核销守恒三条 CHECK 未被绕过。这是全卷最重要的一次演示 |
| M8 | 阶段 11 结束 | 管理层看数。经营驾驶舱出具收入成本交付利润四类指标、按期间客户产品合同订单下钻、应收账龄与应付账龄两张基础表、导出与像素级打印 | 收入成本利润与总账科目余额、应收应付台账、库存金额账三处差额为零；下钻合计加未分摊差异等于总额；五张常用报表验收；账龄分档取自 reporting.aging_bucket_profiles 且 finance 侧临时表已迁移并删除；受治理数据集的列签名与 reporting.dataset_fields 一致，project_projects 一个按已登记未发布降级并顺延到 M10 |
| M9 | 阶段 9b 结束 | 期末关账与年结。发起关账被受理前提拦截、修复后重新发起、受理后等待在途事务并建立快照、强制校验通过后关闭期间、年度末次期间的损益结转 | 关账全过程不冻结写入；其间到达的业务事件照常提交并顺延入账；注入差额后关账被拦截且可修复重发 |
| M10 | 阶段 12 结束 | 服务与视图。工单登记退换修、生成销售退货并双向追溯、设备台账与在保判定、合同派生项目任务、客户 360 五区块含降级区块 | 追溯双向可达；派生任务按唯一键不重复；某区块提供者不可用时返回 DEGRADED 而非整页失败 |
| M11 | 阶段 13 结束 | 客户端与定制。四端安装并登录、按能力矩阵展示与拒绝、建一个自定义对象并在线 DDL、打一个配置包并签名发布、注入一次失败后回退、白标构建两套品牌、停用一个模块再启用 | 配置六步（隔离开发、差异审查、自动测试、审批、签名发布、失败回退）全部可演示；在线 DDL 执行前经 MigrationWindowGuard::assert_open 校验迁移窗口；自定义对象自动获得权限流程检索报表；模块停用后其定时任务停止与对外事件停发且再启用后恢复；业务界面不在本阶段演示，已随阶段 5 至 12 各自验收 |
| M12 | 阶段 14 结束 | 交付验收。整机失效恢复演练、密钥恢复材料隔离恢复演练、备份自动校验、归档链断裂处置、经 OpsDisposalService 的一次处置执行、运维中心台账与诚实披露页 | 附录 A 性能容量基线在 20 并发负载模型下达标；附录 A.6 两项演练达标；发布门禁全绿，含 RG-CI-PROBE-ABSENT 与 RG-TOOLS-EXCLUDED 与 RG-RLS-MATRIX-GREEN；电子签章提交 contract_sandbox.rs 对真实沙箱的一次通过记录或规格附录 B 允许的等效验证证据 |

演示节奏的三条纪律。其一，M7 之前不允许对外承诺闭环，因为在阶段 10 之前发票与应收应付缺位，第 8 章第 6 至 10 步无法连通。其二，M5、M7、M8、M9 四次演示必须在 ep-datagen 的默认 scale 数据集上跑，不允许用小样本，因为规格附录 A.1 的度量结论只在该规模下成立。其三，判据中标注顺延的项一律在其顺延到的时点补验，不得默认通过：M3 的档案停用引用计数与两个使用探针分别顺延到阶段 6、阶段 8 与阶段 12 结束，M4 的过渡科目腿净额与发票红冲判定顺延到 M7、直运退货勾稽顺延到阶段 7，M8 的 project 数据集视图顺延到 M10。

## 6. 全局风险

### 6.1 结构性风险

R1 两处循环依赖。阶段 3 与阶段 4 之间、阶段 8 与阶段 9 之间各有一个真实的环。若不按第 3.3 节拆成 3a/3b 与 9a/9b，排期会在这两处反复回退。应对是把拆分写进阶段 3 与阶段 9 的计划头，并在 CI 的依赖方向自检中把 3a 与 3b 表达为两组 crate 集合，防止实现时越界。

R2 平台能力排期倒挂已按裁定前移。ep-platform-license、ep-platform-recon、ep-platform-release、ep-adapter-search、ep-adapter-doc、platform_ops 六项原先被排到很晚或无归属，而阶段 1 至 5 已经开始依赖它们。裁定后的落点是 platform_ops 最小台账归阶段 2，ep-platform-release 的端口归阶段 3a，许可与全文检索与最小发布通道归阶段 3b，ep-adapter-doc 归阶段 5，对账框架归阶段 9a。残余风险是反向依赖留下的空实现，应对是按裁定通则第三条统一以 Noop 前缀命名并在注入行标注 TODO 加阶段号，由 xtask 门禁统计空实现数量，每个阶段结束时把尚未被替换的空实现在验收报告中逐条列出。

R3 闭环枢纽单据的归属已定。交付确认单与进项发票是黄金闭环第 8 步与第 5 步的枢纽，原先被两个阶段互相推给对方。裁定按模块归属判据定死，交付确认单两表落在 sales schema 归阶段 6，进项发票台账两表落在 invoice schema 归阶段 10。残余风险是交付确认的过渡科目腿由阶段 10 反向替换，应对是把两个单据的用例写进 M7 判据，并在阶段 6 的退出条件中把过渡科目净额断言显式标为顺延到 M7。

R4 客户端界面已下沉到各业务阶段。原先十四个阶段的 interfaces 中没有任何业务界面，而规格第 6.2 章要求四端一致性矩阵按模块判定。裁定按 A-23 把界面下沉到阶段 5 至 12，目录固定为 clients/desktop/src/modules 与 clients/mobile/src/modules 下每模块一个，阶段 13 只保留壳、能力矩阵闸、白标与制品。残余风险是八个业务阶段各自的四端用例工作量被低估，应对是把这一条写进这八个阶段的退出条件，并在阶段 14 的验收矩阵中按模块汇总。

### 6.2 一致性风险

R5 取价职责已定死在 inventory。阶段 7 原先把取价放在 ledger，阶段 8 放在 inventory，取价直接决定规格第 5.2 章事件-分录表的金额与第 17.3 章守恒校验能否成立，两套实现必然产生尾差。裁定按 C-13 撤销阶段 7 的 PurchaseReceiptPostingPort 与 PurchaseReturnPostingPort，取价一律经 InventoryPostingPort 与 InventoryVariancePort，ledger 只做分录映射与借贷平衡，阶段 7 与阶段 9 的计划中各写一句不自行取价。

R6 命名多套并存已一次性裁决。第 4.3 节列出的十四条命名冲突涉及事务方法名、指标名、trait 名、端点路径、自检项编号与单据类型码，命名一旦进入代码，改动面是全仓。裁决结果已逐条写进第 4.3 节的确切标识符列，残余工作是在阶段 1 与阶段 2 定稿前把结果写入基线，并由 CI 校验 docs/error-codes.md、docs/event-catalog.md、docs/metrics-catalog.md、docs/data-dictionary.md 四份登记文件与代码常量表一致。

R7 关账受理前提的口径已统一。凭证同事务生成与待过账队列判据不能同时成立。裁定按 C-28 定死，全部凭证与业务事件同事务生成，Outbox 只承载派生、通知、检索与报表数据集，受理前提二的判定语句以第 4.3 节 C-28 行的原文为准，在阶段 4、9、10 三份计划中逐字一致，posting_date 为空的平台事件不计入，任何暗示存在异步过账路径的措辞一律删除。

### 6.3 工程与容量风险

R8 性能证据的规模不足。各阶段被要求提交附录 A.1 查询的 EXPLAIN 证据，但基准数据集要到十万级订单行、五十万级库存流水、一百五十万级会计分录、八百 GB 附件才有意义。前期阶段在小数据上出的执行计划，到阶段 14 认证时大概率不成立。应对是 ep-datagen 在阶段 1 就交付可用版本，各阶段的 EXPLAIN 证据一律在默认 scale 上采集，且阶段 14 的实测结论若与前期不符，回退责任落在原阶段而非阶段 14。

R9 连接预算触顶。常驻常规连接上限 42、峰值 52 是规格硬约束，而每个阶段都在增加后台任务、定时器与对账扫描。若各阶段各自申请连接，预算会在阶段 11 前后触顶。应对是把 scripts/verify-connection-budget.sh 提升为每个阶段的退出条件项，任何新增后台任务必须在既有池内排队而不是新建池。

R10 单机资源竞争。归档、备份、报表渲染、对账扫描、全文检索重建五类重负载都落在同一台服务器上，cgroup 配额一旦定死，某一类任务的窗口延长会挤压另一类。应对是把五类任务的时间窗在阶段 14 之前就排定并写入 docs/runbooks，且 archive-writer 与 backup-writer 的独立 slice 不得被任何阶段合并。

R11 电子签章是唯一外部依赖。阶段 6 的契约测试需要沙箱，阶段 14 需要一次真实对接或等效验证。沙箱不可用会同时卡住阶段 6 的退出条件与阶段 14 的认证。应对是阶段 6 同时交付 wiremock 打桩与真实沙箱两套测试，且在阶段 1 就确认沙箱账号可申请。

### 6.4 治理风险

R12 业务待决事项的叠加。PRD 附录乙的未决事项分布在 U-A 至 U-L 各组，十四个阶段全部使用了临时取值。单条切换代价都不大，但若在阶段 11 之后集中关闭，切换会同时触及数据、接口与界面三层。应对是按里程碑设三次决策截止点：M3 之前关闭 U-A 与 U-C 两组，M7 之前关闭 U-D 与 U-E 与 U-G 三组，M8 之前关闭 U-H 与 U-I 两组。每个阶段计划必须写明本阶段被哪些待决事项阻塞。

R13 覆盖率门槛与进度的冲突。强制不变量与平台内核代码要求 85%，工作区整体 80%，新增修改代码 80%。在十四个阶段连续推进时，最容易被牺牲的就是覆盖率。应对是把 cargo-llvm-cov 的 --fail-under-lines 设为硬门禁，且 #[ignore] 必须带 issue 编号并在下一阶段结束前清零，由 xtask coverage 统计并在阶段验收报告中列出。

R14 迁移顺序与跨模块逻辑引用。基线禁止跨 schema 外键，跨模块引用只由 application 校验并由对账周期核对。这意味着迁移顺序错误不会立即报错，而是在数据回填时才暴露。应对是 order.toml 的二十四项顺序在阶段 2 冻结后不得调整，任何阶段需要调整必须走基线修订，且每个阶段新增的跨模块逻辑引用必须同步实现一个 ReconCheck 并注册到 ep-platform-recon 的 ReconRegistry。同 schema 内的引用一律建真实外键，交付确认单与销售退货勾稽两处按 A-09 由逻辑引用改回真实外键。

R15 阶段 13 的关键路径效应。裁定后阶段 13 已在关键路径上，它自己实现六个自定义类 ConfigItemApplier，另外九个分别由阶段 3b、4、11 交付；能力域码十八项与动作类别五项由阶段 1 冻结，各业务阶段在自己的 ep-contract 中声明常量；受治理数据集十二个由阶段 5、6、8、10、12 各自发布。任何一个阶段漏做，阶段 13 都会直接顺延。应对是把这三类交付物写进阶段 3b、4 与阶段 5 至 12 每一个阶段的退出条件清单，并由 xtask 的 configdoc 与 archcheck 在每个阶段结束时校验完整性。
