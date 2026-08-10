## 阶段 11：成本、指标与报表

本阶段把规格第 5.2 章成本归集与销货成本结转条目、第 5.5 章报表与经营驾驶舱预置指标两条条目，以及 PRD 第 8 节的四块用户可见能力落到可运行形态。凡涉及借贷、取价、价差拆分、暂估回冲、超量开票结清、退货回冲、期间归属与顺延，一律按规格第 5.2 章事件-分录表及其后各规则块执行，本计划只写捕获点、维度来源与守恒判据，不复述账务规则。

### 0. 前置判断与登记
#### 0.0 本阶段在贯通线 T0 中的最小切片

贯通线 T0 按通则第四条固定的十五环链 1 → 2 → 3a → 4 → 3b-1 → T0 → 5 → 9a → 8 → 6 → 7 → 10 → 11 → 9b → 14 排在第六环，即阶段 3b-1 批之后、阶段 5 全量开工之前，判据是一条合同从建单走到管理层看到一个数。本阶段向 T0 只贡献取数与展示侧的一条最薄路径，不贡献本节以外的任何内容，也不因 T0 新增任何范围。

贡献项四条。其一，costing.revenue_entries 一张表与 costing.v_revenue_entries_dataset 一个视图。其二，第 4.2 节收入捕获在 ep-app-ledger 过账用例内的调用点，只接一种收入来源，其触发事件与账务规则一律以规格第 5.2 章事件-分录表为准，本阶段不判定。其三，第 5.2 节 GET /api/v1/reporting/operating-metrics 只出收入一张卡，不出成本、利润、交付三张卡，不出三侧未分摊差异，meta 只留 as_of。其四，clients/desktop/src/modules/reporting 下承载该卡的一个桌面端页面。

T0 期间提前执行的迁移为第 3.3 节 db/migrations/costing/ 的四条与 db/migrations/reporting/ 的第 1、2、11、12 号四条，表、三个视图与数据集目录一次建齐，成本侧两表在 T0 期间为空表、costing.v_margin_dataset 因此只出 entry_side 为 REVENUE 的行、外部数据集目录行为未发布状态；本阶段核对这八条已生效，不重复执行，第 3.3 节其余迁移仍在本阶段执行。

T0 的通过判据只有一条：收入卡上该法人该会计期间的取值与该期间收入科目的贷方净发生额差额为零。取数用 ep-datagen 最小样本，不要求规格附录 A.3 的 scale 数据集，不要求任何分支覆盖，不要求移动端，不要求性能通过线。

不进 T0 的部分即本阶段其余全部内容，一律改为在这条已贯通的骨架上加厚：成本三类来源与其归集查询、四类指标与三层下钻、三侧未分摊差异、账龄两表、四类报表配置对象与其发布通道接入、统一前置查询服务与高级只读 SQL、导出与打印渲染、三个内部对账校验项、四端界面与性能取样。加厚不改变 T0 已确立的取数路径与恒等口径，只增加来源、维度与分支。第 9 节退出条件与 M8 仍是本阶段的完整验收，不因 T0 已通过而降低任何一条。

#### 0.1 相对共享技术基线的偏离与新增决定

| 编号 | 内容 | 理由 | 影响范围 |
|---|---|---|---|
| D-11-01 | reporting 的分析取数经 ep_analyst_ro 直接读取来源模块在其自身 schema 内发布的 v_ 受治理数据集视图，不逐条经 contract trait 往返。基线第 1.3 节的禁止跨模块直接读写业务表在本阶段被界定为只约束基表 | 规格第 5.5 章与第 16 章要求分析与经营报表在同一实例的独立只读角色上以聚合执行；在会计分录 150 万条的基准数据集上逐行往返无法满足附录 A.1 常用报表 P95 在 10 秒内 | 只读、只经 ep_analyst_ro、只经已登记数据集视图、SQL 中不得出现来源模块基表名、不得出现任何写语句，由 CI 的 SQL 静态检查与数据集注册表双重约束 |
| D-11-02 | 分析查询使用单个只读 REPEATABLE READ 事务。基线第 8.4 节只为内部对账与关账前校验规定该级别 | PRD 第 8.2.3 与第 8.3.2 节要求各维度合计加未分摊差异等于总额可由用户在结果表上直接验算。汇总行、未分摊差异行与总计行若分处不同快照，并发写入会使该恒等式在结果页上不成立 | 只影响 costing 与 reporting 的只读查询路径，不改变任何业务事务的隔离级别 |
| D-11-03 | reporting.datasets 与 reporting.dataset_fields 不带 legal_entity_id，按基线第 3.8 节的全局配置字典归类，不建行级策略 | 两表是随版本发布的内置数据集目录，不承载业务数据，对全部法人取值相同。加法人列会产生两份完全相同的目录与两套目录不一致的可能 | 两表禁止承载业务数据，禁止出现指向具体业务记录的列，由本阶段越权测试目标逐列核对 |
| D-11-04 | 启动自检新增命名项 reporting-dataset-signature-matched，severity 取 Degrading：已登记数据集的来源视图存在，且其列名与类型签名与 reporting.dataset_fields 一致；不一致不阻断启动，改为关闭该数据集及其依赖的报表对象的运行期入口，经阶段 2 交付的 ep_platform_obs::DegradationLedger 开降级窗口并按规格第 15.3 章告警。自检项按基线第 7.3 节注册进 SelfCheckRegistry，用注册名标识，不用序号称呼 | 来源视图缺失或列签名漂移会使已发布报表在运行期取数失败。PRD 第 8.6 节要求该场景不返回可能错位的结果，关闭入口即满足该要求；而以退出码 78 拒绝启动会把一个报表入口不可用放大为八进程集体停机，这台服务器没有备节点，且此时可行的修复动作恰恰被拒绝启动本身阻断 | core-server 与 job-worker 执行；--check 模式一并执行且 DEGRADED 与 FAILED 同样以非零退出，闸门落在部署与升级前置，不落在进程启动；来源视图尚未发布的目录行与列签名漂移共用这一条降级口径，不另设放行条款与解除时点 |
| D-11-05 | 新增 5 个指标、36 个错误码、3 个领域事件、13 个配置键，清单见第 3.6、第 5.6、第 6.5、第 7 节 | 基线第 12 节要求先登记再实现 | 阶段结束时回写基线第 5.5、第 6.1、第 7.1、第 9.2 节 |

#### 0.2 被 PRD 附录乙未决事项覆盖的部分与临时取值

本阶段被 13 条未决事项直接覆盖。13 条均不阻塞开工，理由是每一条都能以临时取值先行实现，决策只改取值不改本阶段的开工前提；其中 U-C-12、U-I-06 与 U-I-08 三条的反向切换属结构变更，代价在下表逐条写明并在第 11.1 节 R-7 复核。逐条给出临时取值与切换代价。

| 编号 | 临时取值 | 切换代价 |
|---|---|---|
| U-I-01 交付按期完成率口径 | 分母含期间内约定交付但尚未交付且尚未到期的节点与分批交付行；按期判定基准取实际交付确认日期小于等于约定交付日期，合同交付节点取节点确认日期；逾期天数自约定交付日期次日起算。指标卡固定标注分母中未到期项数 | 一处 SQL 谓词加一处展示常量，无结构变更，代价低 |
| U-I-02 毛利率零收入展示 | 收入为 0 时毛利率返回 null，界面呈现为不适用；收入不为 0 时一律按毛利除以收入计算并展示，含负值 | 一个纯函数分支，代价低 |
| U-I-03 默认法人、默认期间与行数上限 | 默认法人取该用户授权法人集合中的默认法人；默认期间取最近 3 个自然月对应的会计期间，与基线第 11.5 节一致；聚合结果硬上限 2000 行，下钻单页上限 200 行 | 三项均为配置键，改配置即可，代价低 |
| U-I-04 未分摊差异是否可下钻 | 可下钻。下钻端点与维度行下钻同一个，过滤条件为该维度列为空 | 若改为不可下钻，只需去掉一个入口，代价低 |
| U-I-05 被停用定义的引用表现 | 引用它的仪表盘组件保留占位并显示该报表已停用，不报错，也不展示最后一次结果 | 一处渲染分支，代价低 |
| U-I-06 是否允许缓存或物化 | 不缓存、不物化，每次实时取数。基线第 3.2 节已定首版不使用物化视图。结果头部固定展示取数时点，精确到秒 | 若改为允许缓存，需新增失效通道与失效事件，代价中，属结构变更，需在本节重新评估 |
| U-I-07 顺延导致数值变化的提示 | 结果头部固定展示取数时点；当该法人该期间存在 deferred_from_period_id 非空的凭证时，追加提示本期间存在顺延入账凭证 N 张，并指向规格第 21.20 章的口径说明 | 一处提示与一次计数查询，代价低 |
| U-I-08 跨法人是否合计 | 不合计。成本归集查询与经营驾驶舱的法人一律为必填单选，与 PRD 第 8.2.2 节一致；用户有多个法人权限时逐个法人分别出具 | 若改为允许合计，需按基线第 3.8 节逐法人设置会话变量后在应用侧相加，属新增用例，代价中 |
| U-I-09 自定义指标可引用范围 | 只能引用已登记受治理数据集的字段；不得引用未分摊差异桶、预置指标本身与总账科目余额数据集；自定义指标不进入规格第 22 章第 6 条一致性验收 | 放宽只需在数据集注册表上打开三个数据集的可引用标记，代价低 |
| U-I-10 存在未清零对账差异事项时的标记 | 指标卡级标记，粒度为法人加会计期间。该组合存在未清零对账差异事项时，收入、成本、利润三张卡加标记，交付卡不加 | 改粒度需改一次查询的分组键，代价低 |
| U-I-11 导出格式与是否一律触发高风险 | 报表与指标导出一律按敏感数据导出处理，需重新认证与审批；格式为 XLSX、CSV、PDF 三种；单次上限 50000 行，取自基线第 11.5 节 | 若改为按数据范围分级判定，需依赖 U-B-18 的敏感字段清单，属新增判定，代价中 |
| U-C-12 个人自用报表是否与企业级报表走同一发布通道 | 不区分个人与企业级，报表定义、自定义指标、仪表盘、打印模板四类对象一律按第 4.6 节四状态机提交审批，并经第 4.8 节的 ep-platform-release 通道跨环境发布，不设免审批旁路 | 若改为双通道，需新增个人自用对象类型与一条免审批发布路径，并同步改第 4.6 节状态机守卫、第 4.8 节四个 ConfigItemApplier 的 item_kind 映射与第 6.3 节配置对象生命周期的事务与权限判定，属结构变更，代价中 |
| U-I-12 状态名称、版本号与回退粒度 | 状态取 PRD 第 8.4.5 节的 DRAFT、PENDING_APPROVAL、PUBLISHED、DEACTIVATED 四个；版本号为整数 version_no 自 1 递增；回退粒度为对象版本级 | 与 U-C-12 同源，须与 PRD 第 10 节的配置发布通道一并决策，代价低 |

另有四条未决事项落在别的阶段但会改变本阶段的取数结果，本阶段按下列假定实现并在退出条件中留出核对项：U-D-11 账龄分档由本阶段承载配置结构，取值见第 3.4 节；U-D-12 到期日取值来源由财务阶段给出，本阶段只消费台账视图上的 due_date 列；U-C-09 已退货未冲回成本的置位方尚未决策，本阶段只提供 is_returned_not_reversed 列、CostReturnMarkPort 与按该标注的筛选，不指名调用方，第 8.5 节第十五类用例经该端口直接置位，该事项关闭后由其指定的阶段接入调用；U-F-06 直接费用类采购单据的合同订单项目字段是否至少必填其一直接决定未分摊差异桶的大小，本阶段不做假定，字段为空即归入未分摊差异。

#### 0.3 本阶段显式声明的假设

| 假设 | 理由 |
|---|---|
| A-1 成本科目集合与收入科目集合由 ledger 的事件科目对应关系配置给出，不由 costing 自行判定。ledger 在生成凭证时对每条落在这两个集合上的凭证行标注为成本腿或收入腿，并在同一事务内调用本阶段的捕获端口 | PRD 第 8.2.1 节要求三类成本之和与总账主营业务成本余额在法人、会计期间与科目合计层面可对平。若由 costing 自行判定科目归属，判定与凭证生成会成为两处口径，对平就不是构造性成立而是需要额外校验。规格第 7.2 章规定财务模块是分录的唯一权威写入者，科目判定留在 ledger 与该条一致 |
| A-2 成本归集条目与收入归集条目与凭证在同一数据库事务内写入，而不是经 Outbox 异步补写 | PRD 第 8.3.2 节的一致性口径要求差额为零，PRD 第 8.6 节要求出现差额时进入死信与人工修复而不是静默修正。异步补写会使正常运行期出现秒级差额，与该口径无法区分真实缺陷 |
| A-3 客户维度在捕获时由来源单据的合同或订单带出并落列，两者都为空时该条目的客户维度为空 | 直接费用类单据自带合同、订单与项目三个字段，不含客户字段。PRD 第 8.2.2 节的主维度五选一含客户，若不在捕获时固化，按客户下钻需要在查询期跨模块解析合同与订单的客户归属，无法在只读角色的单次聚合内完成 |
| A-4 收入、成本、利润三项与应收应付台账、库存金额账的一致性经规格第 17.3 章已有勾稽项传递判定，本阶段只直接比对指标与总账科目发生额，另加一条存货类成本合计与库存金额账出库金额合计的直接比对 | 规格第 22 章第 6 条要求三处一致。总账与应收应付台账、总账与库存金额账的勾稽已由规格第 17.3 章的子账与总账勾稽项承担并由内部对账组件按第 10.2 章执行。重复实现一次会产生第二套口径 |
| A-5 空维度一律以 NULL 表达，不使用基线第 11.4 节的哨兵值；只有参与唯一约束的 source_document_line_id 使用全零 UUID 哨兵 | 基线第 11.4 节的哨兵值理由是分组键中的 NULL 会把同一物料拆成两组。本阶段恰恰相反：未分摊差异的定义就是维度列为空的那一组，NULL 语义正是需要的语义。唯一约束上的 NULL 会使重复行不被拦截，因此只在该处用哨兵 |

### 1. 交付物清单

本阶段结束时下列内容可运行、可演示、可被自动化用例判定。

1. costing 模块的成本与收入归集台账：随凭证同事务写入的 costing.cost_entries 与 costing.revenue_entries 两张仅追加表，覆盖存货类销货成本、直接费用类成本、入账差异类成本三类来源与两类收入来源，含已退货未冲回成本标注。
2. 成本归集查询：按合同、订单、客户、项目、产品五个主维度之一的聚合结果，含收入、三类成本分列、成本合计、毛利六列与单列的未分摊差异行，并可下钻到交付确认单、直接费用类采购发票与入账差异事项，再跳转原单据。
3. 经营驾驶舱四类预置指标：收入、成本、利润、交付四张指标卡，三层下钻，收入侧、成本侧、利润侧三个未分摊差异桶，两类期间口径的界面披露数据，以及默认管理驾驶舱的开箱可用实例化。
4. 应收账龄与应付账龄两张基础表：可配置分档、按客户或供应商与单据的未核销余额账龄分布、区间行下钻到单据行。
5. 报表与指标自助定制：受治理数据集目录与字段目录、报表定义、企业自定义指标、仪表盘、像素级打印模板四类配置对象的统一生命周期与审批发布，含依赖登记与失效检测。
6. 统一前置查询服务的分析取数侧：数据集白名单、字段级与密级重写、记录级谓词注入、结果限量、高级只读 SQL 的解析与重写、超限终止到运维中心的记录。
7. 同实例只读角色的取数隔离：ep_analyst_ro 独立连接池的取用路径、会话参数、只读事务、语句超时与单查询资源上限的实测触发与错误映射。
8. 导出与打印渲染的后台任务：render_tasks 台账、job-worker 侧渲染、产物落为附件对象、站内通知回执、敏感导出的重新认证与审批留痕。
9. 内部对账新增三个子账与总账勾稽项：在 ep-app-costing 实现三个 ep_platform_recon::ReconCheck，经 ReconRegistry::register 在 apps/job-worker/src/wiring.rs 注册，进入每日校验与关账前强制校验。对账框架本体、platform_core 的三张对账表与 ReconExecutor 由阶段 9a 交付，本阶段不建框架，也不复述注册方清单与校验项总数，该清单的唯一出处是裁定 A-06。
10. 基准数据集扩展：ep-datagen 产出成本与收入归集条目、交付节点与分批交付、账龄可测的历史分布，规模与规格附录 A.3 一致。
11. 测试目标：tests/rls_matrix 的 costing 与 reporting 扩展、tests/analytics_isolation、tests/finance_metrics_consistency、四个 E2E 用例与六项常用报表性能用例。
12. 文档：docs/data-dictionary 的 costing 与 reporting 两节、docs/error-codes.md 新增 36 条、docs/event-catalog.md 新增 3 条、docs/adr 中 D-11-01 至 D-11-05 五份决策记录。
13. 四个报表类 ConfigItemApplier：ReportDefinitionApplier、MetricDefinitionApplier、DashboardDefinitionApplier、PrintTemplateApplier，位于 ep-app-reporting，实现阶段 3a 在 crates/platform/release/src/port/config_item.rs 交付的 ConfigItemApplier，注册进 ConfigItemApplierRegistry。
14. costing.stock_value_adjust 消费者：消费阶段 8 发出的 inventory.stock_movement.value_adjusted.v1，位于 crates/application/costing/src/consumer/stock_value_adjust.rs，在 job-worker 注册，向 costing.cost_entries 补记只影响金额账的调整对应的成本条目。
15. 本模块四端界面：costing 与 reporting 两个模块的桌面端与移动端界面，目录为 clients/desktop/src/modules/costing、clients/desktop/src/modules/reporting、clients/mobile/src/modules/costing、clients/mobile/src/modules/reporting。

### 2. crate 与进程归属

#### 2.1 新增 crate

| crate | 目录 | 职责 | 装配进程 |
|---|---|---|---|
| ep-contract-costing | crates/contract/costing | 成本与收入捕获命令、已退货未冲回标注命令、成本归集查询 DTO、CostCaptureService 与 RevenueCaptureService 与 CostReturnMarkPort 三个 trait，跨模块方法签名一律接受 ep_foundation::port::Tx 的 &mut dyn Tx；src/capability.rs 中各用例的能力域码与动作类别常量 | core-server |
| ep-domain-costing | crates/domain/costing | CostEntry 与 RevenueEntry 聚合、CostSource 与 RevenueSource 值对象、维度解析规则、未分摊差异判定规则、冲回配对规则 | core-server |
| ep-app-costing | crates/application/costing | 捕获用例、成本归集查询用例、下钻用例、标注用例、授权调用与审计写入；三个 ReconCheck 实现；src/consumer/stock_value_adjust.rs 库存金额调整消费者 | core-server、job-worker |
| ep-contract-reporting | crates/contract/reporting | 数据集描述符、报表类配置对象 DTO、经营指标查询 DTO、AgingBucketQuery trait（唯一方法 buckets(tx: &mut dyn Tx, ctx: &SecurityContext, legal_entity_id: Id<LegalEntity>, ledger_side) -> Result<Vec<AgingBucket>, AppError>）、RenderTaskPort trait；src/capability.rs 中各用例的能力域码与动作类别常量 | core-server、job-worker |
| ep-domain-reporting | crates/domain/reporting | ReportObject 聚合与四状态机、ReportSpec 值对象、指标表达式模型、账龄分档模型与分档算法、查询计划模型 | core-server、job-worker |
| ep-app-reporting | crates/application/reporting | 经营指标用例、账龄两表用例、配置对象生命周期用例、统一前置查询服务 query_facade、高级只读 SQL 解析与重写 sql_guard、渲染任务用例；四个 ConfigItemApplier 实现 ReportDefinitionApplier、MetricDefinitionApplier、DashboardDefinitionApplier、PrintTemplateApplier | core-server、job-worker |

#### 2.2 改动的既有 crate

| crate | 改动 | 进程 |
|---|---|---|
| ep-foundation | error::codes 新增 36 个常量；新增 Ratio 展示辅助与 UnallocatedBucket 类型 | 全部 |
| ep-adapter-db | 新增只读事务句柄类型 ReadOnlyTx 与只读池抽象，不含 PostgreSQL 专有语法 | 全部 |
| ep-adapter-db-pg | 新增 costing 与 reporting 两个 schema 的仓储实现；实现 ep_analyst_ro 只读池的取用钩子，含 SET TRANSACTION READ ONLY ISOLATION LEVEL REPEATABLE READ、statement_timeout、work_mem、temp_file_limit 与会话变量注入清除；实现聚合语句构造器 | core-server、job-worker |
| ep-adapter-doc | 在阶段 5 交付的 SpreadsheetPort、DocTemplatePort、PdfRenderPort 三个 trait 上增量实现像素级套打的 PrintLayout 取值与 CSV 写出，不新增 trait | job-worker |
| ep-platform-recon | 消费侧：本阶段不改其实现，只在 ep-app-costing 实现三个 ReconCheck 并经 ReconRegistry::register 注册；框架本体、三张表与 ReconExecutor 由阶段 9a 交付 | job-worker |
| ep-platform-obs | 注册 5 个新指标 | 全部 |
| ep-platform-authz | 消费侧扩展：本阶段调用其记录级谓词导出与字段级密级裁剪 API，不改其实现 | core-server |
| ep-platform-release | 消费侧：本阶段实现四个 ConfigItemApplier 并注册进阶段 3a 交付的 ConfigItemApplierRegistry；报表类配置对象的跨环境发布与回退一律经阶段 3b 交付的发布通道，本阶段不自建第二套 | core-server、job-worker |
| ep-app-ledger | 在其过账用例内追加成本与收入捕获的调用点，新增对 ep-contract-costing 的依赖；不依赖 ep-app-costing，两个捕获实现经 wiring 注入 | core-server |
| apps/core-server | wiring 注入本阶段交付的 CostCaptureService、RevenueCaptureService、CostReturnMarkPort 与 AgingBucketQuery 四个实现，其中成本与收入捕获的调用点由本阶段在 ledger 过账用例内追加，AgingBucketQuery 供 finance 台账查询用例取用，CostReturnMarkPort 的调用方按 PRD 附录乙 U-C-09 待决；新增第 5 节列出的全部路由；新增 severity 为 Degrading 的命名自检项 reporting-dataset-signature-matched | core-server |
| apps/job-worker | 新增渲染任务消费者、预置报表对象幂等播种任务、数据集依赖失效扫描任务、三个新校验项的调度 | job-worker |
| ep-testkit | 新增 CostEntryBuilder、RevenueEntryBuilder、ReportObjectBuilder、AgingProfileFixture、DeliveryMilestoneFixture | 测试 |
| ep-datagen | 新增成本与收入归集条目、交付节点与分批交付、账龄分布三组生成器 | 测试 |

#### 2.3 依赖方向自检

ep-domain-costing 只依赖 ep-foundation 与 ep-contract-costing。ep-app-costing 依赖 ep-foundation、ep-platform-*（含 ep-platform-recon 以实现三个 ReconCheck）、ep-domain-costing 与 ep-contract-ledger、ep-contract-clm、ep-contract-sales、ep-contract-procure、ep-contract-mdm、ep-contract-project 六个契约。ep-app-reporting 另依赖 ep-platform-release 以实现四个 ConfigItemApplier。ep-app-ledger 只新增对 ep-contract-costing 的依赖，实现在 apps/core-server/src/wiring.rs 注入，不产生 ep-app-ledger 对 ep-app-costing 的依赖。ep-app-finance 只新增对 ep-contract-reporting 的依赖以取用账龄分档，方向为 finance 到 reporting，与 reporting 到 finance 的数据集读取不构成 crate 级环，因为后者不经 crate 依赖而经数据库视图。按通则第三条，全卷不再有先注入空实现后反向替换这一形态，本阶段的三处跨模块接法固定如下。成本与收入捕获的调用点由本阶段在 ep-app-ledger 的过账用例内追加，与两个实现同批交付，两个 wiring.rs 直接注入真实实现。已退货未冲回成本的置位方按 PRD 附录乙 U-C-09 待决，本阶段只交付 CostReturnMarkPort 的实现与注册，不指名调用方。账龄分档按裁定 C-08 由阶段 10 先建临时表自行取数，本阶段迁移并删除该临时表后 finance 台账查询改经 AgingBucketQuery。三处均为同批交付，本阶段不产生任何顺延到后续阶段的验收项，也不接受任何早期阶段向本阶段派工的空实现。上述自检写入 CI 的 cargo metadata 断言脚本。

### 3. 数据库变更

全部新表按基线第 4 节的公共列排列顺序落列，下表只列出公共列之外的业务列。全部带 legal_entity_id 的表按基线第 3.8 节的模板启用并强制行级安全，策略名 rls_<table>_le，判据只有 app.legal_entity_id。全部索引按基线第 3.10 节命名，基线索引 pk_<table> 与 ix_<table>_legal_entity_id_created_at 逐表建立，下文不重复列出。

#### 3.1 schema costing

##### costing.cost_entries

仅追加表，不带 row_version、updated_at、updated_by，带 reverses_id。

| 列名 | 类型 | 可空 | 约束与说明 |
|---|---|---|---|
| reverses_id | uuid | 是 | 指向被本条冲回的成本归集条目，同 schema 外键 ON DELETE RESTRICT |
| business_date | date | 否 | 原始业务日期，取该业务事件的记账日期，按规格第 5.2 章 |
| accounting_period_id | uuid | 否 | 与该事件总账凭证共用同一会计期间字段，顺延时一并顺延 |
| source_type | text | 否 | ck_cost_entries_source_type，取值 INVENTORY_COGS、DIRECT_EXPENSE、POSTING_VARIANCE |
| variance_reason | text | 是 | ck_cost_entries_variance_reason，取值 ESTIMATE_PRICE_DIFF_ISSUED、PURCHASE_RETURN_DIFF、RED_LETTER_DIFF、OVER_INVOICE_TO_COST；与 source_type 为 POSTING_VARIANCE 互为充要 |
| voucher_id | uuid | 否 | 逻辑引用 ledger，不建外键 |
| voucher_line_id | uuid | 否 | 逻辑引用 ledger，不建外键 |
| account_id | uuid | 否 | 成本科目，逻辑引用 ledger |
| amount | numeric(18,2) | 否 | 借方为正，贷方冲回为负，ck_cost_entries_amount_nonzero |
| source_document_type | text | 否 | ck_cost_entries_source_document_type，取值 DELIVERY_CONFIRMATION、PURCHASE_INVOICE、PURCHASE_RETURN、RED_LETTER_INVOICE、OVER_INVOICE_SETTLEMENT、SALES_RETURN；取值为 DELIVERY_CONFIRMATION 时 source_document_id 与 source_document_line_id 指向阶段 6 按裁定 A-09 在 sales schema 交付的 sales.delivery_confirmations 与 sales.delivery_confirmation_lines，成本下钻按裁定 A-09 以这两张表为来源单据，并经这两列跳转原单据；下钻返回的明细行取自本模块的 costing.cost_entries，跳转由客户端按 jump_target 调用阶段 6 的交付确认查询端点完成，分析 SQL 中不出现这两个基表名，与 D-11-01 一致 |
| source_document_id | uuid | 否 | 逻辑引用来源模块 |
| source_document_line_id | uuid | 否 | 无明细行时写全零 UUID 哨兵，理由见 0.3 A-5 |
| contract_id | uuid | 是 | 归集维度 |
| sales_order_id | uuid | 是 | 归集维度 |
| sales_order_line_id | uuid | 是 | 下钻用 |
| customer_id | uuid | 是 | 归集维度，捕获时由合同或订单带出 |
| project_id | uuid | 是 | 归集维度 |
| product_id | uuid | 是 | 归集维度，ck_cost_entries_direct_no_product 强制 source_type 为 DIRECT_EXPENSE 时为空 |
| material_id | uuid | 是 | 下钻与库存交叉核对用 |
| warehouse_id | uuid | 是 | 存货类必填，与库存金额账交叉核对用 |
| is_returned_not_reversed | boolean | 否 | 默认 false，已退货未冲回成本标注 |
| return_mark_reason | text | 是 | ck 长度不超过 2000 |
| return_mark_approval_ref | text | 是 | 置位时的审批引用 |
| idempotency_key | uuid | 否 | 取自触发命令 |

约束与索引：
- ux_cost_entries_legal_entity_id_voucher_line_id_source_document_line_id，保证同一凭证行与同一来源明细行只捕获一次。
- ck_cost_entries_inventory_dims：source_type 为 INVENTORY_COGS 时 warehouse_id 与 material_id 非空。
- ix_cost_entries_legal_entity_id_accounting_period_id：聚合主路径。
- ix_cost_entries_legal_entity_id_contract_id_accounting_period_id。
- ix_cost_entries_legal_entity_id_sales_order_id_accounting_period_id。
- ix_cost_entries_legal_entity_id_customer_id_accounting_period_id。
- ix_cost_entries_legal_entity_id_project_id_accounting_period_id。
- ix_cost_entries_legal_entity_id_product_id_accounting_period_id。
- ix_cost_entries_legal_entity_id_voucher_id：对账与追溯。
- 五个维度索引均把维度列放在会计期间列之前，理由是查询模式固定为法人等值、维度等值、期间集合等值，把范围性最弱的列前置可使索引可用；期间范围在应用侧已展开为期间标识集合，不是范围扫描。

##### costing.revenue_entries

结构与 cost_entries 同构，差异如下。

| 列名 | 类型 | 可空 | 说明 |
|---|---|---|---|
| source_type | text | 否 | ck，取值 DELIVERY_ORDER、DELIVERY_MILESTONE、SALES_RETURN |
| amount | numeric(18,2) | 否 | 贷方确认收入为正，销售退货冲减为负 |
| source_document_type | text | 否 | ck，取值 DELIVERY_CONFIRMATION、SALES_RETURN |
| warehouse_id | 不设 | | 收入侧无仓库维度 |
| variance_reason | 不设 | | 收入侧无差异类 |
| is_returned_not_reversed | 不设 | | 该标注只在成本侧 |

约束与索引：ux_revenue_entries_legal_entity_id_voucher_line_id_source_document_line_id；ck_revenue_entries_milestone_dims 强制 source_type 为 DELIVERY_MILESTONE 时 sales_order_id、sales_order_line_id、product_id、material_id 四列全为空，这条 CHECK 直接把规格第 5.5 章收入侧未分摊差异的定义固化到数据库；索引集合与成本侧一一对应，去掉 warehouse 相关。

##### 视图

| 视图 | 内容 |
|---|---|
| costing.v_cost_entries_dataset | cost_entries 的受治理投影，含全部维度列、金额、期间、来源类型与来源单据引用，附 security_level 与 data_scope_tags |
| costing.v_revenue_entries_dataset | revenue_entries 的受治理投影 |
| costing.v_margin_dataset | 上两者的 UNION ALL，附 entry_side 列取值 REVENUE 与 COST，供驾驶舱利润指标一次聚合 |

三个视图 GRANT SELECT 给 ep_analyst_ro 与 ep_app_rw，不授予任何写权限，授权语句与建视图语句在同一迁移文件内执行。视图不带 SECURITY DEFINER，行级策略随基表生效。三个视图均包含 legal_entity_id、security_level、data_scope_tags 三列，列名与类型签名与 reporting.dataset_fields 的登记一致，由启动自检项 reporting-dataset-signature-matched 按 D-11-04 的降级口径校验。

#### 3.2 schema reporting

##### reporting.datasets

全局配置字典，不带 legal_entity_id、security_level、data_scope_tags，不建行级策略，按 D-11-03 登记。

| 列名 | 类型 | 可空 | 说明 |
|---|---|---|---|
| id | uuid | 否 | 主键 |
| code | text | 否 | ux_datasets_code，ck 长度不超过 64 |
| name | text | 否 | ck 长度不超过 200 |
| source_module | text | 否 | ck 取值为基线第 1.2 节的 15 个模块码之一 |
| source_view | text | 否 | schema 限定视图名，ck 必须以 source_module 或其平台 schema 前缀开头 |
| grain | text | 否 | ck 取值 ENTRY、DOCUMENT、DOCUMENT_LINE、SNAPSHOT |
| min_security_level | smallint | 否 | 默认 20 |
| is_drillable | boolean | 否 | 默认 true |
| is_metric_referenceable | boolean | 否 | 默认 true，自定义指标可否引用，对应 U-I-09 |
| row_version、created_at、created_by、updated_at、updated_by | | | 按基线第 4 节 |

##### reporting.dataset_fields

全局配置字典，同上豁免。

| 列名 | 类型 | 可空 | 说明 |
|---|---|---|---|
| dataset_id | uuid | 否 | 同 schema 外键 ON DELETE RESTRICT |
| field_code | text | 否 | ux_dataset_fields_dataset_id_field_code |
| data_type | text | 否 | ck 取值 UUID、TEXT、DATE、TIMESTAMPTZ、NUMERIC_2、NUMERIC_6、RATE、INT、BOOLEAN |
| field_role | text | 否 | ck 取值 KEY、DIMENSION、MEASURE、TIME、TAG |
| security_level | smallint | 否 | 字段级密级，未赋值时按数据集取值 |
| allowed_aggregations | text[] | 否 | 默认 '{}'，取值 SUM、COUNT、MIN、MAX、AVG |
| is_filterable、is_sortable、is_groupable | boolean | 否 | 默认 true |
| sort_no | int | 否 | |

##### reporting.report_objects

档案类，带 code、is_active、deactivated_at。

| 列名 | 类型 | 可空 | 说明 |
|---|---|---|---|
| code | text | 否 | ux_report_objects_legal_entity_id_code |
| name | text | 否 | ck 长度不超过 200 |
| object_kind | text | 否 | ck 取值 REPORT_DEFINITION、CUSTOM_METRIC、DASHBOARD、PRINT_TEMPLATE |
| bound_document_type | text | 是 | 仅 PRINT_TEMPLATE 使用，ck 强制其余三类为空 |
| is_preset | boolean | 否 | 默认 false，预置对象由 job-worker 幂等播种，不可删改只可停用 |
| description | text | 是 | ck 长度不超过 500 |

四类对象共用一张身份表的理由：PRD 第 8.4.5 节给出的是一套状态、一套审批人规则、一套权限继承；四张近乎相同的表会把行级策略、索引、迁移与状态机测试面扩大三倍，而它们之间没有任何字段差异需要数据库层区分。类型专有的结构差异全部落在版本行的 spec 上，由应用层按 object_kind 用不同的 schema 校验。

##### reporting.report_object_versions

| 列名 | 类型 | 可空 | 说明 |
|---|---|---|---|
| report_object_id | uuid | 否 | 同 schema 外键 ON DELETE RESTRICT |
| version_no | int | 否 | ux_report_object_versions_legal_entity_id_report_object_id_version_no |
| status | text | 否 | ck 取值 DRAFT、PENDING_APPROVAL、PUBLISHED、DEACTIVATED |
| spec | jsonb | 否 | 声明式定义，按 object_kind 由应用层用 JSON Schema 校验 |
| spec_hash | text | 否 | spec 的 SHA-256，供配置发布差异审查与签名 |
| submitted_by | uuid | 是 | |
| submitted_at | timestamptz | 是 | |
| approved_by | uuid | 是 | ck_report_object_versions_no_self_approval：approved_by 为空或 submitted_by 为空或两者不等 |
| approved_at | timestamptz | 是 | |
| approval_ref | text | 是 | 审批实例引用 |
| release_package_ref | text | 是 | 配置发布包引用，指向阶段 3b 交付的 platform_meta.config_packages |
| deactivated_at | timestamptz | 是 | |
| deactivated_by | uuid | 是 | |

索引：ix_report_object_versions_legal_entity_id_status；ix_report_object_versions_legal_entity_id_report_object_id_status。

##### reporting.report_object_publications

当前生效版本，一个对象一行。

| 列名 | 类型 | 可空 | 说明 |
|---|---|---|---|
| report_object_id | uuid | 否 | ux_report_object_publications_legal_entity_id_report_object_id |
| report_object_version_id | uuid | 否 | 同 schema 外键 |
| state | text | 否 | ck 取值 EFFECTIVE、SUSPENDED |
| published_at、published_by、approval_ref | | | |
| suspended_at、suspended_by、resumed_at、resumed_by | | 是 | |

不在 report_objects 上放 published_version_id 列的理由：那会与 versions 表构成互指外键，需要延迟约束；单独一行既表达当前生效版本，又天然承载停用与恢复的时点与操作者。

##### reporting.report_object_dependencies

| 列名 | 类型 | 可空 | 说明 |
|---|---|---|---|
| report_object_version_id | uuid | 否 | 同 schema 外键 |
| dependency_kind | text | 否 | ck 取值 DATASET、FIELD、METRIC、REPORT |
| dataset_code | text | 是 | dependency_kind 为 DATASET 或 FIELD 时非空 |
| field_code | text | 是 | dependency_kind 为 FIELD 时非空 |
| referenced_object_id | uuid | 是 | dependency_kind 为 METRIC 或 REPORT 时非空 |
| is_broken | boolean | 否 | 默认 false，由 job-worker 的失效扫描置位 |
| broken_reason | text | 是 | |

索引：ix_report_object_dependencies_legal_entity_id_dataset_code_field_code。

##### reporting.aging_bucket_profiles 与 reporting.aging_bucket_lines

profiles 为档案类，带 code、is_active、deactivated_at，另加 ledger_side（ck 取值 RECEIVABLE、PAYABLE、BOTH）、is_default（boolean）。lines 列为 aging_bucket_profile_id、sort_no int、bucket_code text、label text、from_days int 可空、to_days int 可空、includes_not_due boolean。

约束：ux_aging_bucket_lines_legal_entity_id_aging_bucket_profile_id_sort_no；ck_aging_bucket_lines_range 强制 from_days 与 to_days 同时为空只允许出现在 includes_not_due 为真的行上，且非空时 from_days 小于等于 to_days；分档的无缝无叠由应用层在保存时校验并返回 REPORTING.AGING_BUCKET_PROFILE.RANGE_GAP 或 RANGE_OVERLAP，不在数据库层表达，理由是跨行约束在 PostgreSQL 上只能靠触发器或排除约束，二者都超出基线第 7.4 章的公共能力基线。

账龄分档放在 reporting 而不是 finance 的理由：PRD 第 6.9.3 节的台账内账龄查询与第 8.3.4 节的两张预置基础表必须用同一套分档，否则同一笔余额在两处落入不同区间。把唯一出处放在 reporting，由 finance 经 ep-contract-reporting 的 AgingBucketQuery 读取，依赖方向为 ep-app-finance 到 ep-contract-reporting，不构成环。

##### reporting.render_tasks

单据类，带 doc_no 与 status。编号类型码 RT，按基线第 11.1 节由 ep-platform-sequence 取号。

| 列名 | 类型 | 可空 | 说明 |
|---|---|---|---|
| doc_no | text | 否 | ux_render_tasks_legal_entity_id_doc_no |
| status | text | 否 | ck 取值 QUEUED、RUNNING、SUCCEEDED、FAILED、CANCELLED、EXPIRED |
| task_kind | text | 否 | ck 取值 REPORT_EXPORT、DASHBOARD_EXPORT、COST_QUERY_EXPORT、AGING_EXPORT、PRINT_RENDER |
| report_object_version_id | uuid | 是 | 内置查询导出时为空 |
| request_spec | jsonb | 否 | 查询条件与列选择，含法人与期间 |
| output_format | text | 否 | ck 取值 XLSX、CSV、PDF |
| row_count | int | 是 | |
| attachment_object_id | uuid | 是 | 逻辑引用 platform_file.attachment_objects |
| reauth_ref | text | 是 | 敏感导出的重新认证凭证引用 |
| approval_ref | text | 是 | 敏感导出的审批引用 |
| requested_at、started_at、finished_at | timestamptz | | |
| expires_at | timestamptz | 否 | 产物保留到期时点 |
| last_error | text | 是 | |
| attempts | int | 否 | 默认 0 |

索引：ix_render_tasks_legal_entity_id_status_requested_at；ix_render_tasks_legal_entity_id_created_by_created_at。

#### 3.3 迁移编号与顺序

执行顺序按基线第 3.9 节由单一全局 Runner 依文件版本号全序排定，本阶段 reporting 目录下引用 costing 与 finance 对象的文件，其版本号一律晚于这些对象的建表迁移。

db/migrations/costing/
1. V202611030900__costing_create_cost_entries.sql
2. V202611030905__costing_create_revenue_entries.sql
3. V202611030910__costing_create_dataset_views.sql
4. V202611030915__costing_grant_analyst_ro.sql

db/migrations/reporting/
1. V202611031000__reporting_create_datasets.sql
2. V202611031005__reporting_create_dataset_fields.sql
3. V202611031010__reporting_create_report_objects.sql
4. V202611031015__reporting_create_report_object_versions.sql
5. V202611031020__reporting_create_report_object_publications.sql
6. V202611031025__reporting_create_report_object_dependencies.sql
7. V202611031030__reporting_create_aging_bucket_profiles.sql
8. V202611031035__reporting_create_aging_bucket_lines.sql
9. V202611031040__reporting_create_render_tasks.sql
10. V202611031045__reporting_grant_analyst_ro.sql
11. V202611031050__backfill_reporting_seed_datasets.sql
12. V202611031055__backfill_reporting_seed_dataset_fields.sql
13. V202611031060__reporting_backfill_migrate_aging_buckets_from_finance.sql
14. V202611031065__reporting_drop_finance_aging_bucket_definitions.sql

每个建表文件包含表、约束、索引与行级策略，不含任何数据写入；两个 seed backfill 文件只写数据集目录与字段目录，不写任何带法人的数据。全部文件头注释给出 rollback 段：建表类给出 DROP TABLE 逆向；两个 seed backfill 给出按 code 删除的逆向。带法人的预置对象即默认管理驾驶舱与默认账龄分档不在迁移中播种，由 job-worker 的幂等任务按法人补齐，理由是迁移执行时法人集合未知，且基线第 3.9 节要求迁移可离线执行、不得调用应用代码。第 13 号与第 14 号两个迁移按裁定 C-08 与通则第五条都放在 db/migrations/reporting/：第 13 号把阶段 10 临时建立的 finance.aging_bucket_definitions 的分档数据迁入 reporting.aging_bucket_profiles 与 reporting.aging_bucket_lines，第 14 号删除该临时表，两者均由本阶段提供。两者同属 reporting 这一个 Runner，按版本号先迁后删自然成立，不设标记行守卫，也不设任何跨 Runner 的顺序断言。跨 schema 的 DROP 由 ep_migrator 执行，该角色已具备全部 ep_mod_* 成员资格。第 14 号的 rollback 段只给出重建空表的逆向，不恢复数据。

#### 3.4 默认账龄分档取值

对应 U-D-11 的临时取值，播种为 is_preset 的 profile，code 为 AGING-DEFAULT-7，允许管理员经第 5.3 节的账龄分档端点在审批后修改。账龄分档不属阶段 3a 冻结的 15 个 item_kind，因此不进配置发布包，其修改只在本环境内生效，不经跨环境发布通道，也不另建第二套发布路径。

| sort_no | bucket_code | label | from_days | to_days | includes_not_due |
|---|---|---|---|---|---|
| 1 | NOT_DUE | 未到期 | 空 | 空 | true |
| 2 | D1_30 | 逾期 1 至 30 天 | 1 | 30 | false |
| 3 | D31_60 | 逾期 31 至 60 天 | 31 | 60 | false |
| 4 | D61_90 | 逾期 61 至 90 天 | 61 | 90 | false |
| 5 | D91_180 | 逾期 91 至 180 天 | 91 | 180 | false |
| 6 | D181_360 | 逾期 181 至 360 天 | 181 | 360 | false |
| 7 | D360_PLUS | 逾期 360 天以上 | 361 | 空 | false |

#### 3.5 受治理数据集目录种子

| dataset code | source_view | 提供阶段 | grain |
|---|---|---|---|
| costing_cost_entries | costing.v_cost_entries_dataset | 本阶段 | ENTRY |
| costing_revenue_entries | costing.v_revenue_entries_dataset | 本阶段 | ENTRY |
| costing_margin | costing.v_margin_dataset | 本阶段 | ENTRY |
| mdm_customers | mdm.v_customers_dataset | 5 | DOCUMENT |
| mdm_products | mdm.v_products_dataset | 5 | DOCUMENT |
| mdm_materials | mdm.v_materials_dataset | 5 | DOCUMENT |
| ledger_account_period_balances | ledger.v_account_period_balances | 9a | SNAPSHOT |
| inventory_stock_value_entries | inventory.v_stock_value_entries | 8 | ENTRY |
| clm_contracts | clm.v_contracts_dataset | 6 | DOCUMENT |
| clm_contract_delivery_milestones | clm.v_contract_delivery_milestones | 6 | DOCUMENT_LINE |
| sales_sales_orders | sales.v_sales_orders_dataset | 6 | DOCUMENT |
| sales_order_delivery_batches | sales.v_order_delivery_batches | 6 | DOCUMENT_LINE |
| invoice_purchase_invoices | invoice.v_purchase_invoices_dataset | 10 | DOCUMENT |
| finance_receivable_ledger_entries | finance.v_receivable_ledger_entries | 10 | ENTRY |
| finance_payable_ledger_entries | finance.v_payable_ledger_entries | 10 | ENTRY |
| project_projects | project.v_projects_dataset | 12 | DOCUMENT |

其中 ledger_account_period_balances 与 costing_margin 的 is_metric_referenceable 置为 false，对应 U-I-09 的临时取值。
外部数据集共 13 个，每个由拥有其基表的模块所在阶段发布，本阶段只登记目录与消费，不代建任何来源视图，也不承担任何其他模块的数据集视图。每个来源视图必须包含 legal_entity_id、security_level、data_scope_tags 三列，并在同一迁移中执行 GRANT SELECT ON 该视图 TO ep_analyst_ro，不授予 ep_app_rw 之外的任何写权限；列名与类型签名必须与 reporting.dataset_fields 的登记一致，由启动自检项 reporting-dataset-signature-matched 校验。

project_projects 的目录行由本阶段先播种、阶段 12 后建视图，其间该行按 D-11-04 的统一降级口径处理，即自检不阻断启动、该数据集与依赖它的报表对象入口关闭并开降级窗口，阶段 12 交付视图后窗口关闭。不为这一行另设专用的放行条款与解除时点，来源视图未发布与列签名漂移共用同一条口径。

#### 3.6 新增指标登记

| 指标 | 类型 | 标签 |
|---|---|---|
| ep_analytics_query_duration_seconds | histogram，桶沿用基线 ep_http_request_duration_seconds 的桶 | dataset、query_kind、legal_entity_id |
| ep_analytics_query_terminated_total | counter | reason 取值 statement_timeout、temp_file_limit、work_mem、result_limit、cancelled |
| ep_report_render_duration_seconds | histogram | task_kind、output_format |
| ep_report_render_queue_depth | gauge | 无 |
| ep_costing_entries_written_total | counter | side 取值 cost、revenue；source_type |

标签基数：dataset 上限 16，query_kind 上限 8，legal_entity_id 为 2，符合基线第 9.2 节的基数纪律。

### 4. 领域模型与关键算法

#### 4.1 核心类型

ep-domain-costing：

- CostEntry 聚合，字段与 costing.cost_entries 一一对应，构造函数只接受已校验的 CostCaptureCommand，不提供任何可变方法，冲回以新建带 reverses_id 的条目表达。
- CostSource 枚举：InventoryCogs { warehouse_id, material_id }、DirectExpense、PostingVariance(VarianceReason)。
- VarianceReason 枚举四值，与规格第 5.2 章成本归集条目列举的入账差异四类事项一一对应：暂估价差的已出库部分、采购发票已登记的采购退货差额、红字冲销与作废进项方向差额、超量开票经审批确认不再冲回时的转成本。
- RevenueSource 枚举：DeliveryOrder、DeliveryMilestone、SalesReturn。
- CollectionDimension 枚举五值：Contract、SalesOrder、Customer、Project、Product。
- DimensionSet 值对象，承载六个可空维度标识，提供 project_to(dimension) 返回 Option<Id>，None 即落入未分摊差异。

ep-domain-reporting：

- ReportObject 聚合与 ReportObjectVersion 实体，承载四状态机。
- ReportSpec 枚举四变体，分别对应报表定义、自定义指标、仪表盘、打印模板，每个变体有独立的结构与校验。
- MetricExpression：复用低代码声明式表达式的 AST，只允许字段引用、四则运算、聚合函数与条件表达式，不允许函数调用与子查询。
- AgingProfile 与 AgingBucket 值对象，提供 bucket_of(reference_date, due_date) -> BucketCode。
- QueryPlan 值对象：数据集、投影列、分组键、聚合项、过滤谓词、排序、限量六部分，是 query_facade 与 SQL 构造器之间唯一的中间表示，用户输入不进入 SQL 文本拼接。

#### 4.2 成本与收入捕获算法

触发点在 ledger 的过账用例内，与凭证写入同一事务；该调用点由本阶段在 ep-app-ledger 内追加，阶段 9a 不预留空实现，见第 2.3 节。

1. ledger 生成凭证后，把每条落在成本科目集合或收入科目集合上的凭证行连同该行对应的来源单据行标识与维度字段，组装为 CostCaptureCommand 或 RevenueCaptureCommand 列表，经注入的端口调用。
2. costing 侧对每条命令执行：校验 amount 非零；校验 source_type 与 variance_reason 的充要关系；校验存货类必带仓库与物料；校验直接费用类不得带产品；解析客户维度，取合同的客户，合同为空时取订单的客户，两者都空则为空；写入条目。
3. 幂等：插入语句为 INSERT ... ON CONFLICT (legal_entity_id, voucher_line_id, source_document_line_id) DO NOTHING RETURNING id，受影响行数为 0 时回读既有行并比对金额，金额不等返回 COSTING.COST_ENTRY.DUPLICATE_CAPTURE 并按业务冲突处理，金额相等即视为重复投递已捕获。
4. 冲回：销售退货、采购退货、红字冲销与作废产生的冲减腿同样以捕获命令进入，amount 为负，reverses_id 指向原条目；原条目不可定位时 reverses_id 为空，此时该条目仍进入合计，只是不成对，不影响任何守恒判据。
5. 边界条件：ledger 传入的科目不在成本或收入科目集合时返回 COSTING.COST_ENTRY.ACCOUNT_NOT_COST_ACCOUNT 或 REVENUE_ENTRY.ACCOUNT_NOT_REVENUE_ACCOUNT，整笔过账事务回滚，理由是捕获遗漏会直接破坏 PRD 第 8.2.1 节的对平；配置键 EP__COSTING__CAPTURE__REJECT_UNBOUND_COST_LEG 默认 false 时改为写 WARN 日志加计数并放行，只在实施期排障使用，生产默认拒绝该放行路径关闭。

顺延不改变本算法：条目的 accounting_period_id 直接取该事件凭证的会计期间字段，business_date 取记账日期，两者由 ledger 传入，costing 不自行判定期间，与规格第 5.2 章子账与凭证共用同一期间归属条款一致。

#### 4.3 成本归集查询算法

输入：法人（必填，取自 X-Legal-Entity-Id）、会计期间起止（必填）、主维度（必填五选一）、可选维度取值过滤、可选已退货未冲回筛选、分页与排序。

1. 经 ep-contract-ledger 把期间起止解析为期间标识集合 P，集合规模上限由 EP__REPORTING__ANALYTIC__MAX_PERIOD_SPAN 约束，默认 36，超出返回 COSTING.COST_ENTRY.PERIOD_RANGE_INVALID。使用集合而不是范围的理由是会计期间标识为 UUID，范围比较无业务语义。
2. 在 ep_analyst_ro 上开启单个只读 REPEATABLE READ 事务，一次事务内顺序执行四条语句：维度分组的成本三类分列与合计；维度分组的收入；未分摊差异的成本三类分列与合计（同一维度列 IS NULL）；总计行。
3. 恒等式由构造保证：分组集合与 IS NULL 集合互斥且并集为全集，因此各维度合计加未分摊差异恒等于总额。实现上不允许用两次独立查询相减得到未分摊差异。
4. 毛利等于收入减成本合计，按 numeric(18,2) 直接相减，不产生舍入。毛利率按基线第 3.5 节的 Rate 计算并 round 到 6 位，收入为 0 时返回 null。
5. 未分摊差异不进入 rows 数组，作为 data.unallocated 独立对象返回，并携带 legal_entity_id 与期间集合，从结构上使它不可能被排序进维度行，对应 PRD 第 8.2.3 节末行固定与不参与排序的要求。
6. 结果行数超过 EP__REPORTING__ANALYTIC__MAX_RESULT_ROWS 时返回 COSTING.COST_ENTRY.RESULT_TOO_LARGE 并在 advice 中给出可收窄的条件，不返回部分结果，对应 PRD 第 8.6 节。
7. 按产品下钻时的收窄：直接费用类条目的 product_id 由 CHECK 保证为空，因此自动落入未分摊差异；入账差异类按原维度归集后仍无产品字段的同样落入。无需任何额外分支代码，这是把口径固化到约束的收益。

#### 4.4 经营驾驶舱四类指标

三层结构固定为指标卡、维度汇总、单据行，下钻维度为期间、客户、产品、合同、订单五个，项目维度不在其内。

- 收入卡：Σ revenue_entries.amount，按法人与期间集合。
- 成本卡：Σ cost_entries.amount，三类来源可分列。
- 利润卡：收入减成本，毛利率按 4.3 第 4 条。
- 交付卡：不取任何凭证，取 clm_contract_delivery_milestones 与 sales_order_delivery_batches 两个数据集，期间维度取约定交付日期所属自然月，与金额类指标的会计期间字段不是同一口径。实际交付以交付确认单为准，其主体按裁定 A-09 由阶段 6 在 sales schema 交付，本阶段经上述两个数据集上的交付确认引用与确认日期列取得按期判定所需的实际交付日期，按 D-11-01 不在分析 SQL 中出现 sales.delivery_confirmation_lines 基表名。

未分摊差异三侧：

- 成本侧：cost_entries 中该下钻维度列为空的部分，按法人与会计期间聚合。
- 收入侧：revenue_entries 中 source_type 为 DELIVERY_MILESTONE 的部分，只在按产品与按订单下钻时成立；按客户、合同、期间下钻时该部分有维度值，不进入未分摊差异。
- 利润侧：同一法人同一会计期间的收入侧未分摊差异减成本侧未分摊差异。该值允许为负，实现上不施加任何 abs、max(0, x) 或隐藏分支，并由一条单元测试专门断言负值被原样返回与展示。
- 交付指标不设未分摊差异桶。按产品或订单下钻时分母与逾期清单同步收窄为订单分批交付部分，响应中固定返回 scope_narrowed 为 true 与 narrowed_reason，界面据此显式标注。

两类期间口径的界面披露：任一同时返回金额类与交付类指标的响应，meta 中固定携带 period_basis_note 对象，含 amount_basis 取值 VOUCHER_ACCOUNTING_PERIOD、delivery_basis 取值 PROMISED_DATE_NATURAL_MONTH 与 disclosure_ref 指向规格第 21.20 章，客户端不得省略渲染。

取数时点与顺延提示：meta 中固定携带 as_of，取只读事务开始时的服务器时间；deferred_voucher_count 经 ep-contract-ledger 取该法人该期间 deferred_from_period_id 非空的凭证张数，大于 0 时客户端展示顺延提示。

对账差异提示：meta 中固定携带 open_recon_discrepancies，取该法人该期间未清零的对账差异事项条数，大于 0 时收入、成本、利润三张卡加标记，交付卡不加，对应 U-I-10。

#### 4.5 账龄两表算法

1. 取 finance_receivable_ledger_entries 或 finance_payable_ledger_entries，过滤 open_amount 不等于 0。
2. 参考日为查询请求中的 as_of_date，缺省取服务器自然日，按基线第 3.4 节以 (now() AT TIME ZONE 'Asia/Shanghai')::date 取值。
3. 逾期天数等于 as_of_date 减 due_date；小于等于 0 落入 includes_not_due 的档；大于 0 按分档区间落档，区间为闭区间。
4. 账龄基数为 open_amount，不是原单据金额，对应规格第 17.3 章核销守恒与 PRD 第 6.9.3 节。
5. 账龄一律按原始业务日期与到期日计算，不按 accounting_period_id，因此存在顺延入账时账龄不变，这一点由一条集成测试专门断言。
6. 出现负的 open_amount 时不参与分档，单列为 anomaly 数组并附错误码提示，同时按规格第 15.2 章交由内部对账与死信处置，界面不静默修正。
7. 分档缺失或存在断档与重叠时拒绝出表，返回 REPORTING.AGING_BUCKET_PROFILE.RANGE_GAP 或 RANGE_OVERLAP。

#### 4.6 报表类配置对象状态机

状态：DRAFT、PENDING_APPROVAL、PUBLISHED、DEACTIVATED。

| 迁移 | 触发 | 守卫条件 |
|---|---|---|
| DRAFT 到 PENDING_APPROVAL | 设计者提交发布 | 提交人为该版本创建人；spec 通过按 object_kind 的 JSON Schema 校验；依赖登记全部可解析且 is_broken 为 false；引用字段在提交人的字段级权限与密级范围内可见 |
| PENDING_APPROVAL 到 PUBLISHED | 审批通过 | 审批人不为提交人；审批链按规格第 12.2 章不可越权跳过；通过后写入或更新 publications 行，原 PUBLISHED 版本置为 DEACTIVATED |
| PENDING_APPROVAL 到 DRAFT | 审批驳回 | 审批人不为提交人 |
| PUBLISHED 到 DRAFT | 修改已发布定义 | 不改变原 PUBLISHED 版本，新建 version_no 加一的 DRAFT 版本；原版本在新版本发布前继续生效，即 publications 行不变 |
| PUBLISHED 到 DEACTIVATED | 停用 | 操作者具备 reporting.report_object.administer；publications.state 置为 SUSPENDED |
| DEACTIVATED 到 PUBLISHED | 重新启用 | 同上权限；该版本的依赖登记重新校验通过；publications.state 置回 EFFECTIVE |

非法迁移返回 REPORTING.REPORT_OBJECT.INVALID_TRANSITION，自审返回 REPORTING.REPORT_OBJECT.SELF_APPROVAL_FORBIDDEN。

#### 4.7 统一前置查询服务与高级只读 SQL 守卫

拖拽式取数路径：请求到 QueryPlan 的转换只接受已登记数据集与已登记字段；字段级密级与权限由 ep-platform-authz 裁剪，无权字段既不进 SELECT，也不进 GROUP BY、ORDER BY 与分面计数；记录级谓词由 authz 导出为结构化谓词并注入 QueryPlan 的过滤部分；法人隔离不在 SQL 中表达，由行级策略承担；SQL 由 QueryPlan 生成，全部值以绑定参数传递，无字符串拼接。

高级只读 SQL 路径：用 sqlparser 解析为 AST，逐项白名单校验。允许：单条 Query 语句、SELECT、非递归 CTE、JOIN、GROUP BY、HAVING、ORDER BY、子查询、UNION ALL、字面量与绑定参数、白名单聚合函数与日期函数。拒绝：任何 DML 与 DDL、多语句、FOR UPDATE 与 FOR SHARE、递归 CTE、集合返回函数、pg_ 前缀与 information_schema 的任何对象、未登记的表或视图、COPY、DO、事务控制语句、以及任何在字段目录中不可见的列引用。校验通过后由重写层追加权限谓词与 LIMIT，超出 MAX_JOIN_COUNT 或 MAX_SUBQUERY_DEPTH 或 MAX_QUERY_BYTES 时拒绝。执行只在 ep_analyst_ro 上，不提供裸 SQL 执行端点，只能出现在报表定义版本的 spec 内，由设计器预览端点与报表运行端点两条路径执行，与规格第 8.4.1 节要求的设计器与运行时都不得直连一致。

超限终止：statement_timeout 触发映射为 REPORTING.ANALYTIC_QUERY.STATEMENT_TIMEOUT，temp_file_limit 与 work_mem 相关错误映射为 RESOURCE_LIMIT_EXCEEDED，两者 category 均为 INFRASTRUCTURE、HTTP 503、retryable 为 true，同时写 platform_ops 台账并计入 ep_analytics_query_terminated_total。不返回部分结果，对应 PRD 第 8.6 节。本阶段不登记降级窗口，超限终止只写运维中心的查询终止台账；若实施期需要以降级窗口表达某类分析取数的长期不可用，一律经阶段 2 交付的 ep_platform_obs::DegradationLedger 的 open 与 close，不自建第二套写入路径。

#### 4.8 报表类配置对象的发布通道接入与四个 ConfigItemApplier

报表类配置对象的跨环境发布与回退经阶段 3b 交付的 ep-platform-release 配置发布通道，本阶段不自建第二套通道，也不自建第二套签名与差异审查。本阶段在 ep-app-reporting 实现四个 ConfigItemApplier，所实现的 trait、ItemKind 枚举与 ConfigItemApplierRegistry 由阶段 3a 在 crates/platform/release/src/port/config_item.rs 交付；applier 接受通道传入的 tx 为 &mut dyn Tx，不自开事务，也不自行提交。

| item_kind | 实现类型 | 对应 report_objects.object_kind |
|---|---|---|
| REPORT_DEFINITION | ReportDefinitionApplier | REPORT_DEFINITION |
| METRIC_DEFINITION | MetricDefinitionApplier | CUSTOM_METRIC |
| DASHBOARD_DEFINITION | DashboardDefinitionApplier | DASHBOARD |
| PRINT_TEMPLATE | PrintTemplateApplier | PRINT_TEMPLATE |

item_kind 取值取自阶段 3a 冻结的 15 项枚举，report_objects.object_kind 的四个取值不改名，两者的映射只在 applier 内部转换，理由是 object_kind 是本模块表的 CHECK 取值，改名会牵动已发布对象的行数据与既有索引。

apply 的幂等：按 code 与 spec_hash 定位，已存在同 code 且 spec_hash 相同的 PUBLISHED 版本即跳过；不同即新建 version_no 加一的版本并写入或更新 publications 行。rollback 按包内记录的前一版本 version_no 回退 publications 行，不删除任何版本行，与第 4.6 节的四状态机共用同一套守卫，不另建状态迁移路径。

#### 4.9 库存金额调整事件的成本侧消费者

消费者名 costing.stock_value_adjust，位于 crates/application/costing/src/consumer/stock_value_adjust.rs，在 apps/job-worker 注册，消费阶段 8 发出的 inventory.stock_movement.value_adjusted.v1，幂等由 platform_msg.inbox_consumptions 的 consumer 与 event_id 保证。

副作用为向 costing.cost_entries 补记只影响金额账的调整对应的成本条目：voucher_id、voucher_line_id、account_id 与来源单据引用取事件载荷，写入语句与第 4.2 节第 3 条同为 INSERT ... ON CONFLICT (legal_entity_id, voucher_line_id, source_document_line_id) DO NOTHING，因此该凭证行若已由第 4.2 节的同事务捕获路径写入，本消费者即为空操作。载荷缺少凭证引用时不补记并写死信，理由是 cost_entries 的 voucher_id 与 voucher_line_id 为非空列，静默丢弃会使第 6.6 节的勾稽项在无提示的情况下产生差额。

该通道是收敛通道而不是常规通道：正常运行期应为空操作，非空补记计入 ep_costing_entries_written_total 并作为实施期观察项；两条路径写入同一集合，第 6.6 节三个勾稽项的判据不变，第 0.3 节 A-2 的同事务口径也不因该通道放宽。

### 5. API 契约

全部路径前缀 /api/v1，头集合按基线第 5.6 节。分析类查询一律为 GET，不需要 Idempotency-Key；写请求与导出任务为 POST，必须带 Idempotency-Key。响应封套按基线第 5.2 节。无权访问已存在记录一律 404 与 PLATFORM.AUTHZ.NOT_FOUND_OR_DENIED，对象类型完全无权 403 与 PLATFORM.AUTHZ.OBJECT_FORBIDDEN，法人未授权由平台在安全上下文建立阶段拒绝。

本节全部路由在 ep-contract-costing 与 ep-contract-reporting 的 src/capability.rs 中按用例声明一对常量，命名为 <USECASE_SCREAMING>_DOMAIN 与 <USECASE_SCREAMING>_ACTION，取值取自 ep-foundation 的 CapabilityDomain 与 ActionClass 两个枚举。本阶段全部路由的能力域码取 CapabilityDomain::ReportingReportPrint，动作类别按只读查询取 ActionClass::Read、导出与打印取 Export、设计与修改取 Write、提交发布取 Submit、审批与驳回取 Approve。xtask configdoc 断言每个 /api/v1/ 路由都能解析到一对常量，缺失即构建失败。

#### 5.1 成本归集

| 方法与路径 | 请求 | 响应 | 主要错误码 | 权限 |
|---|---|---|---|---|
| GET /api/v1/costing/cost-collections | 查询参数 period_from、period_to（必填，期间编码）、dimension（必填五选一）、filter[contract_id] 等维度过滤、filter[is_returned_not_reversed]=eq:true、page、page_size、sort | data.rows 为维度行数组，每行含 dimension_key、dimension_label、revenue_amount、cost_inventory_amount、cost_direct_amount、cost_variance_amount、cost_total_amount、gross_profit、gross_margin_rate；data.unallocated 为未分摊差异对象；data.total 为总计行；meta 含 as_of、deferred_voucher_count、open_recon_discrepancies 与分页 | COSTING.COST_ENTRY.PERIOD_RANGE_REQUIRED、PERIOD_RANGE_INVALID、DIMENSION_REQUIRED、DIMENSION_NOT_SUPPORTED、RESULT_TOO_LARGE、REPORTING.ANALYTIC_QUERY.STATEMENT_TIMEOUT | costing.cost_collection.read |
| GET /api/v1/costing/cost-collections/documents | 同上加 dimension_key（可为 null 表示下钻未分摊差异）、source_type 可选 | data 为单据行数组，含 source_type、source_document_type、source_document_id、doc_no、business_date、accounting_period_code、amount、is_returned_not_reversed、jump_target | 同上 | 同上 |
| GET /api/v1/costing/cost-entries/{id} | 无 | 单条条目全字段与其冲回链 | PLATFORM.AUTHZ.NOT_FOUND_OR_DENIED | 同上 |
| GET /api/v1/costing/revenue-entries/{id} | 无 | 同构 | 同上 | 同上 |
幂等语义：四个端点均为 GET，无副作用，重复请求返回同一快照上的等价结果，不使用幂等键。

#### 5.2 经营驾驶舱

| 方法与路径 | 说明 |
|---|---|
| GET /api/v1/reporting/operating-metrics | 返回四张指标卡。参数 period_from、period_to。响应 data.cards 数组，每张卡含 metric_code、value、unit、sub_values、unallocated、scope_note；meta 含 period_basis_note、as_of、deferred_voucher_count、open_recon_discrepancies |
| GET /api/v1/reporting/operating-metrics/{metric_code}/breakdowns | metric_code 取 revenue、cost、profit、delivery。参数 dimension 取 period、customer、product、contract、sales_order。响应 rows 与 unallocated 与 total 三段；delivery 指标的 unallocated 恒为 null，并在 dimension 为 product 或 sales_order 时返回 scope_narrowed 为 true |
| GET /api/v1/reporting/operating-metrics/{metric_code}/documents | 第三层，下钻到单据行并给出跳转目标 |
| GET /api/v1/reporting/aging-reports/{side} | side 取 receivable、payable。参数 as_of_date、profile_code 可选、group_by 取 customer、supplier、contract、sales_order、purchase_order。响应 rows 为分组行含各档金额与合计，anomalies 为负余额清单 |
| GET /api/v1/reporting/aging-reports/{side}/documents | 参数加 bucket_code 与分组键，返回单据行与跳转目标 |

错误码：REPORTING.OPERATING_METRIC.DIMENSION_NOT_SUPPORTED（如以 project 作为驾驶舱下钻维度）、REPORTING.OPERATING_METRIC.PERIOD_SCOPE_MISMATCH、REPORTING.AGING_BUCKET_PROFILE.NOT_FOUND、RANGE_GAP、RANGE_OVERLAP、REPORTING.ANALYTIC_QUERY.* 三条。权限：reporting.operating_metric.read 与 reporting.aging_report.read。

#### 5.3 报表类配置对象

| 方法与路径 | 幂等 | 权限 |
|---|---|---|
| GET /api/v1/reporting/report-objects | 无 | reporting.report_object.read |
| GET /api/v1/reporting/report-objects/{id} | 无 | 同上 |
| POST /api/v1/reporting/report-objects | Idempotency-Key 必填，四元组作用域 | reporting.report_object.design |
| GET /api/v1/reporting/report-objects/{id}/versions | 无 | reporting.report_object.read |
| POST /api/v1/reporting/report-objects/{id}/versions | 必填，从当前 PUBLISHED 派生 DRAFT | reporting.report_object.design |
| PATCH /api/v1/reporting/report-objects/{id}/versions/{version_no} | 必填，带 row_version 乐观锁 | 同上 |
| POST .../versions/{version_no}/actions/submit-for-approval | 必填 | 同上 |
| POST .../versions/{version_no}/actions/approve | 必填 | reporting.report_object.approve |
| POST .../versions/{version_no}/actions/reject | 必填 | 同上 |
| POST .../versions/{version_no}/actions/preview | 必填，设计器预览，含高级只读 SQL 的解析与执行，结果限量 | reporting.report_object.design |
| POST /api/v1/reporting/report-objects/{id}/actions/deactivate | 必填 | reporting.report_object.administer |
| POST /api/v1/reporting/report-objects/{id}/actions/reactivate | 必填 | 同上 |
| GET /api/v1/reporting/report-objects/{id}/results | 无，运行已发布报表定义 | reporting.report_object.read |
| GET /api/v1/reporting/datasets 与 /datasets/{code}/fields | 无 | reporting.report_object.design |
| GET/POST /api/v1/reporting/aging-bucket-profiles 与 /{id}/actions/activate、/actions/deactivate | POST 必填 | reporting.report_object.administer |

错误码：REPORTING.REPORT_OBJECT.INVALID_TRANSITION、SELF_APPROVAL_FORBIDDEN、DEPENDENCY_BROKEN、NOT_PUBLISHED、DEACTIVATED；REPORTING.REPORT_OBJECT_VERSION.SPEC_INVALID、EXPRESSION_PARSE_FAILED、ADVANCED_SQL_NOT_ALLOWED、ADVANCED_SQL_PARSE_FAILED、ADVANCED_SQL_LIMIT_EXCEEDED；REPORTING.DATASET.NOT_REGISTERED、REPORTING.DATASET_FIELD.NOT_VISIBLE、NOT_AGGREGATABLE。

表达式解析失败时 details 数组逐项给出表达式内的失败位置（行、列、片段），对应 PRD 第 8.6 节的定位要求。引用字段被删除或改名时返回 DEPENDENCY_BROKEN 并在 details 中列出失效字段，运行端点不返回任何结果。

#### 5.4 导出与打印渲染

| 方法与路径 | 说明 |
|---|---|
| POST /api/v1/reporting/render-tasks | 创建渲染任务。请求含 task_kind、report_object_id 可选、request_spec、output_format。Idempotency-Key 必填；X-Reauth-Token 必填，取值绑定 request_spec 的摘要；审批未通过时返回 202 与审批中状态。响应为任务回执含 doc_no 与 id |
| GET /api/v1/reporting/render-tasks/{id} | 查询任务状态 |
| GET /api/v1/reporting/render-tasks/{id}/content | 取产物，重定向到 platform_file 的受控下载路径；任务未完成返回 409，产物过期返回 REPORTING.RENDER_TASK.ARTIFACT_EXPIRED |
| POST /api/v1/reporting/render-tasks/{id}/actions/cancel | 取消排队中的任务 |

错误码：REPORTING.RENDER_TASK.ROW_LIMIT_EXCEEDED、FORMAT_NOT_SUPPORTED、CLIENT_NOT_SUPPORTED、ARTIFACT_EXPIRED。CLIENT_NOT_SUPPORTED 用于 X-Client 为 ios 或 android 且 task_kind 为 PRINT_RENDER 的情形，对应规格第 6.2 章移动端仅查看、打印转桌面端。

同步等待上限按基线第 11.6 节的 8 秒：全部导出与打印一律以后台任务表达，不提供同步导出；成本归集查询与驾驶舱查询在 8 秒内未返回时不自动转任务，而是按 statement_timeout 或 SYNC_BUDGET_MS 终止并提示收窄条件，理由是这类查询的正确响应是收窄范围而不是异步等待。

#### 5.5 版本化

四组端点均为新增，不涉及破坏性变更，主版本保持 v1。数据集目录与字段目录的枚举取值扩展按基线第 5.6 节，客户端必须容忍未知取值并按未知降级展示。

#### 5.6 新增错误码清单

COSTING.COST_ENTRY.PERIOD_RANGE_REQUIRED、PERIOD_RANGE_INVALID、DIMENSION_REQUIRED、DIMENSION_NOT_SUPPORTED、RESULT_TOO_LARGE、DUPLICATE_CAPTURE、ACCOUNT_NOT_COST_ACCOUNT、SOURCE_DIMENSION_CONFLICT、RETURN_MARK_NOT_APPLICABLE；COSTING.REVENUE_ENTRY.DUPLICATE_CAPTURE、ACCOUNT_NOT_REVENUE_ACCOUNT；REPORTING.DATASET.NOT_REGISTERED；REPORTING.DATASET_FIELD.NOT_VISIBLE、NOT_AGGREGATABLE；REPORTING.REPORT_OBJECT.INVALID_TRANSITION、SELF_APPROVAL_FORBIDDEN、DEPENDENCY_BROKEN、NOT_PUBLISHED、DEACTIVATED；REPORTING.REPORT_OBJECT_VERSION.SPEC_INVALID、EXPRESSION_PARSE_FAILED、ADVANCED_SQL_NOT_ALLOWED、ADVANCED_SQL_PARSE_FAILED、ADVANCED_SQL_LIMIT_EXCEEDED；REPORTING.ANALYTIC_QUERY.STATEMENT_TIMEOUT、RESOURCE_LIMIT_EXCEEDED、RESULT_TOO_LARGE；REPORTING.AGING_BUCKET_PROFILE.NOT_FOUND、RANGE_GAP、RANGE_OVERLAP；REPORTING.RENDER_TASK.ROW_LIMIT_EXCEEDED、FORMAT_NOT_SUPPORTED、CLIENT_NOT_SUPPORTED、ARTIFACT_EXPIRED；REPORTING.OPERATING_METRIC.DIMENSION_NOT_SUPPORTED、PERIOD_SCOPE_MISMATCH。共 36 条，全部登记进 docs/error-codes.md 与 ep-foundation 的 error::codes。

### 6. 并发与事务边界

#### 6.1 捕获路径

成本与收入归集条目与总账凭证、审计事件、Outbox 条目在同一数据库事务内写入，隔离级别 READ COMMITTED，事务由 ledger 的过账用例持有，costing 不另开事务。锁策略：仅追加表无行更新，唯一索引承担并发去重，不取显式锁。幂等键取自触发命令并落列。与 Outbox 的关系：捕获本身不产生 Outbox 条目，理由是没有消费方；驱动捕获的业务事件的 Outbox 条目由来源模块按既有链路写入。失败重试：唯一约束冲突按 4.2 第 3 条处理；序列化失败与死锁按基线第 8.4 节在数据访问层重试 3 次。补偿：捕获失败即整笔过账事务回滚，不存在只写凭证不写归集条目的中间态。

#### 6.2 查询路径

单个只读 REPEATABLE READ 事务，见 D-11-02。事务预算：ep_analyst_ro 池的 statement_timeout 60 秒、work_mem 64 MB、temp_file_limit 2 GB，按基线第 10.3 节，不由本阶段调整。会话变量在连接取用时写入、归还前清除，由连接池钩子统一实现。查询路径不写任何表，因此不需要幂等键，也不进入 Outbox。超时与资源上限触发时终止并映射错误，不重试，理由是同一查询重试只会再次超时并占用只读池的 10 个连接上限。

#### 6.3 配置对象生命周期

一个用例一个事务，READ COMMITTED，乐观锁 row_version，冲突映射 PLATFORM.CONCURRENCY.STALE_VERSION。发布时在同一事务内更新 versions.status、写入或更新 publications 行、写审计事件、写 Outbox 条目 reporting.report_object.published.v1。停用同理，事件为 reporting.report_object.deactivated.v1。审批链由 ep-platform-flow 承载，审批完成的回调与状态迁移在同一事务内完成。跨环境发布与回退经阶段 3b 的 ep-platform-release 通道，第 4.8 节的四个 ConfigItemApplier 在该通道的事务内被调用，接受通道传入的 &mut dyn Tx，不自开事务、不自行提交、不写第二份审计。

#### 6.4 渲染任务

core-server 在一个事务内取号、插入 render_tasks 行（status 为 QUEUED）、写审计、写 Outbox。job-worker 消费后以 UPDATE ... WHERE id = $1 AND status = 'QUEUED' 抢占并置 RUNNING，受影响行数为 0 即已被抢占，直接跳过。渲染过程在事务外执行，产物写入 platform_file 后再开一个短事务回填 attachment_object_id、row_count 与 SUCCEEDED，并写 Outbox 条目 reporting.render_task.completed.v1 驱动站内通知。失败按基线第 6.2 节的 8 次退避重试，全部失败置 DEAD 并把任务置 FAILED，last_error 落列。事务内禁止文件读写，渲染与写产物都在事务外，符合基线第 10.3 节。

#### 6.5 新增领域事件

| 事件类型 | 载荷要点 | 消费方 |
|---|---|---|
| reporting.report_object.published.v1 | report_object_id、object_kind、version_no、spec_hash、approval_ref | 审计检索、依赖失效扫描、配置发布通道 |
| reporting.report_object.deactivated.v1 | report_object_id、version_no、reason | 同上 |
| reporting.render_task.completed.v1 | render_task_id、doc_no、task_kind、output_format、row_count、attachment_object_id、outcome | 站内通知 |

三个事件的信封字段按基线第 6.1 节，security_level 与 data_scope_tags 取该配置对象或任务行的取值，posting_date 与 accounting_period_id 为 null，理由是这三类事件不产生过账，不进入关账受理前提的待消费过账条目计数。

#### 6.6 内部对账新增校验项

本阶段在 ep-app-costing 实现三个 ep_platform_recon::ReconCheck，经 ReconRegistry::register 在 apps/job-worker/src/wiring.rs 注册；对账框架本体、platform_core 的三张对账表与 ReconExecutor 由阶段 9a 交付，本阶段不建框架、不改其实现。三个实现的 code 返回下表校验项列的取值，category 三项一律取 SUBLEDGER_VS_LEDGER，blocks_period_close 三项均为 true。第三项虽跨 costing 与 inventory 两个 schema 取数，其判据是金额勾稽而不是引用完整性，归入 CROSS_MODULE_LINK 会使同一类别同时承载两种性质的判据。run_batch 接受执行器传入的 &dyn SnapshotCtx 与 BatchWindow，按规格第 10.2 章的分批与快照口径执行，进入每日校验与关账前强制校验，属子账与总账勾稽这一既有类别下的新增勾稽项，不新增校验类别。

| 校验项 | 子账侧 | 总账侧 | 判据 |
|---|---|---|---|
| COSTING_COST_VS_LEDGER | Σ costing.cost_entries.amount，按法人与会计期间 | 成本科目集合的当期借方净发生额 | 差额为零，且每条成本归集条目的 accounting_period_id 与其 voucher_id 所属凭证的会计期间相同；后一条谓词并入本项判据，不为期间一致另立校验项 |
| COSTING_REVENUE_VS_LEDGER | Σ costing.revenue_entries.amount，按法人与会计期间 | 收入科目集合的当期贷方净发生额 | 差额为零，且每条收入归集条目的 accounting_period_id 与其 voucher_id 所属凭证的会计期间相同；后一条谓词并入本项判据，不为期间一致另立校验项 |
| COSTING_INVENTORY_COGS_VS_STOCK_VALUE | Σ source_type 为 INVENTORY_COGS 的 amount | inventory_stock_value_entries 中出库方向的金额合计，按法人与会计期间 | 差额为零，对应规格第 22 章第 6 条的库存金额账一致 |

差异事项按规格第 10.2 章载明勾稽项、法人、会计期间、子账侧金额、总账侧金额与差额，落 platform_core.recon_discrepancies，差额不清零不得关账。三项由阶段 9a 的 ReconExecutor 调度，在 job-worker 自身连接池上执行，不使用只读分析池。

### 7. 配置项

全部键按基线第 7.1 节的 EP__ 前缀与双下划线层级，serde 开启 deny_unknown_fields。生效方式统一为启动期读取，修改后需重启对应进程；--check 模式一并校验取值域。

| 键 | 类型 | 默认值 | 说明 |
|---|---|---|---|
| EP__REPORTING__ANALYTIC__MAX_RESULT_ROWS | u32 | 2000 | 聚合结果硬上限，超出返回 RESULT_TOO_LARGE |
| EP__REPORTING__ANALYTIC__MAX_DRILL_PAGE_SIZE | u16 | 200 | 与基线第 5.3 节分页上限一致 |
| EP__REPORTING__ANALYTIC__MAX_PERIOD_SPAN | u16 | 36 | 与规格附录 A.3 的历史期间跨度一致 |
| EP__REPORTING__ANALYTIC__SYNC_BUDGET_MS | u32 | 8000 | 与基线第 11.6 节同步等待上限一致，先于 statement_timeout 触发以给出可收窄提示 |
| EP__REPORTING__ADVANCED_SQL__ENABLED | bool | true | 关闭后报表定义中含高级只读 SQL 的版本拒绝发布与运行 |
| EP__REPORTING__ADVANCED_SQL__MAX_QUERY_BYTES | u32 | 16384 | |
| EP__REPORTING__ADVANCED_SQL__MAX_JOIN_COUNT | u8 | 8 | |
| EP__REPORTING__ADVANCED_SQL__MAX_SUBQUERY_DEPTH | u8 | 4 | |
| EP__REPORTING__RENDER__MAX_CONCURRENCY | u8 | 2 | job-worker 内渲染并发，受 app-worker.slice 配额约束 |
| EP__REPORTING__RENDER__TIMEOUT_SECONDS | u32 | 300 | 与 job-worker 池 statement_timeout 对齐 |
| EP__REPORTING__RENDER__MAX_EXPORT_ROWS | u32 | 50000 | 与基线第 11.5 节一致 |
| EP__REPORTING__RENDER__ARTIFACT_TTL_DAYS | u16 | 7 | 产物保留期，到期置 EXPIRED 并按附件处置流程处理 |
| EP__COSTING__CAPTURE__REJECT_UNBOUND_COST_LEG | bool | false | 见 4.2 第 5 条；生产部署片段固定置 true |

不进配置文件的运行期业务参数：报表定义、自定义指标、仪表盘布局、打印模板、默认驾驶舱内容，一律存事务数据库并经阶段 3b 交付的 ep-platform-release 配置发布通道，按基线第 7.1 节；账龄分档同样存事务数据库，但按第 3.4 节不进配置发布包，只经本模块端点在审批后修改。

### 8. 测试计划

覆盖率门槛：本阶段的强制不变量相关代码，即成本与收入捕获、恒等式计算、未分摊差异三侧、账龄守恒与分档，行覆盖率不低于 85%；其余代码不低于 70%；新增与修改代码不低于 80%；工作区整体不低于 80%。工具 cargo-llvm-cov，路径规则写入 codecov.toml 的 crates/domain/costing、crates/application/costing、crates/domain/reporting、crates/application/reporting 四条。

#### 8.1 单元测试

- 维度解析：六个维度列的 32 组组合，含客户由合同带出、由订单带出、两者皆空三条分支。
- source_type 与 variance_reason 的充要关系四值全覆盖。
- 直接费用类不得带产品、存货类必带仓库与物料两条守卫的正反用例。
- 未分摊差异按五个主维度分别投影的判定，含收入侧只在产品与订单维度成立、在客户与合同维度不成立的对照用例。
- 利润侧未分摊差异为负时原样返回，不取绝对值、不置零、不隐藏，单独一条断言。
- 毛利率：收入为正、为零、为负三条分支，含 round 到 6 位的边界值。
- 账龄分档：未到期、边界日（逾期恰好 30 天与 31 天）、跨最末开区间、参考日等于到期日、负余额五条。
- 分档配置校验：断档、重叠、未到期档缺失、单档三条边界。
- 四状态机：六条合法迁移逐条，另加 12 条非法迁移逐条拒绝，自审拒绝单列。
- QueryPlan 生成：无权字段不进 SELECT、不进 GROUP BY、不进 ORDER BY、不进分面计数四条。
- 高级只读 SQL 守卫：允许清单 8 条各一例，拒绝清单 12 条各一例，含 pg_catalog 引用、递归 CTE、多语句、FOR UPDATE、未登记视图五条重点。
- 交付按期完成率：分母含未到期、按期、逾期已交付、逾期未交付四类样本的完成率与逾期天数。

#### 8.2 领域属性测试

用 proptest 生成随机条目集合，断言五组不变量：

1. 对任意主维度，各维度行金额合计加未分摊差异恒等于总额。
2. 对任意条目集合，成本三类分列之和恒等于成本合计。
3. 毛利恒等于收入减成本合计。
4. 利润侧未分摊差异恒等于收入侧减成本侧。
5. 账龄各档金额合计恒等于未核销余额合计，且每笔余额恰好落入一档。

前四组直接对应规格第 17.3 章会计借贷平衡与本阶段的下钻恒等要求，第五组对应核销守恒。

#### 8.3 集成测试

使用真实 PostgreSQL 16，每例独占一库。场景清单：

1. 捕获与凭证同事务：过账成功则条目存在，过账回滚则条目不存在，共 6 个事件类型各一例。
2. 重复投递：同一凭证行重复捕获 3 次只产生一条条目。
3. 顺延入账：受理关账后提交的业务事件，其条目的 accounting_period_id 与凭证一致，business_date 保留原记账日期，账龄不因顺延改变。
4. 三个新对账校验项：分别注入成本侧差额、收入侧差额、存货类与库存金额账差额，断言差异事项生成、可追溯、关账被拦截，清零后关账通过。
5. 只读隔离：以 ep_analyst_ro 连接尝试 INSERT、UPDATE、DELETE、CREATE 各一例，全部被拒；尝试读取未授予的表被拒。
6. 语句超时与资源上限实测触发：构造超过 60 秒的查询与超过 2 GB 临时空间的查询，断言终止、错误码、platform_ops 台账记录与指标计数，且不返回部分结果。
7. 结果限量：超过 MAX_RESULT_ROWS 时返回 RESULT_TOO_LARGE 而不是截断。
8. 数据集依赖失效：删除或改名一个来源视图的列，断言启动自检项 reporting-dataset-signature-matched 报 DEGRADED 而不是拒绝启动、进程照常提供其余功能、该数据集与依赖它的报表对象入口被关闭、降级窗口在 platform_ops 落行并告警，以及运行期依赖扫描把 is_broken 置位、运行端点返回 DEPENDENCY_BROKEN；另一例对来源视图尚未发布的 project_projects 断言走同一条降级路径，不走任何专用放行分支；第三例断言 --check 模式在同一库上以非零退出。
9. 配置对象生命周期：六条迁移的端到端，含审批人不为提交人、已发布版本在新草稿发布前继续生效、停用后仪表盘组件显示占位。
10. 渲染任务：排队、抢占、成功、失败重试到 DEAD、取消、产物过期六条。
11. 敏感导出：无 X-Reauth-Token 拒绝、审批未通过返回审批中、审批通过后产物可取。
12. 无数据与无权的区分：同一查询在无数据时返回 200 空结果，在无权时返回 403 或 404，两者提示文案与错误码不同。
13. 四个 ConfigItemApplier：经阶段 3b 的发布通道对四类对象各执行一次 apply 与一次 rollback，断言 apply 幂等（同 spec_hash 重放不新增版本行）、rollback 只回退 publications 行而不删除版本行、applier 全程使用通道传入的事务句柄且不自行提交。
14. costing.stock_value_adjust 消费者：同一 inventory.stock_movement.value_adjusted.v1 重复投递 3 次只补记一条成本条目；该凭证行已由同事务捕获路径写入时补记为空操作；载荷缺少凭证引用时进入死信而不是静默丢弃。
15. 账龄分档唯一出处迁移：在含 finance.aging_bucket_definitions 数据的库上执行两个迁移文件，断言两者在同一 reporting Runner 内按版本号顺序执行、分档逐档迁入 reporting 两表、finance 侧临时表已删除、finance 台账内账龄查询经 AgingBucketQuery::buckets 取到与预置表一致的分档。

#### 8.4 越权与隔离测试

tests/rls_matrix 扩展：对 costing.cost_entries、costing.revenue_entries 与 reporting 的七张带法人表，覆盖读取、写入、更新、删除、聚合、排序、报表投影与错误信息泄漏八类，另加两个复制角色与内部对账系统安全上下文的五个入口。

tests/analytics_isolation 新增测试目标，承担规格第 17.2 章派生存储越权与删除传播测试中的同实例只读角色部分：以跨法人与跨密级的安全上下文对全部已登记数据集发起检索、排序与分面计数，断言不返回无权数据，且排序位次与聚合值不间接暴露无权数据；断言来源记录更正后，报表结果在同一事务快照上一致。首版不做小计数抑制，该点在交付说明中明写。

#### 8.5 端到端测试

后端 E2E 用 Rust 集成测试直打 HTTP。四端 UI 由本阶段交付并测试：桌面端用 Playwright 与 tauri-driver，移动端用 XCUITest 与 Espresso；覆盖范围按规格第 6.2 章能力矩阵，取值为完整或简化的能力域跑完整用例，取值为 VIEW_ONLY 的只跑只读视图的查看与筛选，取值为 NOT_APPLICABLE 的不实现入口也不出用例。

对应规格第 17.2 章十五类必测分支中与本阶段直接相关的四类：

- 第四类直接费用类采购发票归集：无收货入库的服务类与费用类采购发票按单据自带字段直接归集，并进入按合同、订单、客户、项目的成本归集查询。
- 第七类直运订单闭环：全程不产生库存流水与库存金额账变动，成本按直接费用类归集并进入成本归集查询，收入、成本、利润指标包含该订单，存货侧勾稽取值不变。
- 第十三类超量开票转成本：经审批确认不再冲回并转当期主营业务成本后，当期毛利与成本归集查询同步变动。
- 第十五类直运订单退货：供应商不接受退回时该成本保留，并在成本归集查询中按原合同、订单、项目维度标注为已退货未冲回成本，可按该标注筛选，且带标注金额仍计入成本合计。

另加规格第 8 章闭环第 14 步的指标一致性用例，不含第 12 步与期间关账，整条链路的贯通验收由阶段 9b 的 testkit/scenarios/golden_loop_14_steps.rs 承担：断言收入、成本、利润三项取数与总账科目余额差额为零（法人、会计期间与科目合计层面），下钻合计加未分摊差异等于总额（客户、产品、合同、订单四个维度各一次），交付指标与合同交付节点、订单分批交付单据逐项一致（期间、客户、合同三个层面），按产品与订单下钻时交付指标只与订单分批交付单据逐项一致。该用例即规格第 22 章第 6 条的经营指标验收证据。

#### 8.6 性能测试

在规格附录 A.3 基准数据集与附录 A.4 负载模型下执行，20 并发，样本不少于 200 次，只取稳定段，同时施加连续归档写出、附件正文增量写出、审计证据写出与一次每日全量备份。

覆盖附录 A.1 常用报表清单中属本阶段的三项：应收账龄分析、应付账龄分析、经营看板的收入与成本与交付与利润。通过线 P95 在 10 秒内，同时记录 P99 与最大值。成本归集查询不在附录 A.1 清单内，按规格 PRD 第 11.3 节属临时分析查询，首版不设通过线，但仍记录取值作为观察项。

另附 EXPLAIN 证据：上述三项对应的全部查询在基准数据集上不得出现顺序扫描，逐条附 EXPLAIN (ANALYZE, BUFFERS) 输出，这是基线第 3.10 节的硬性要求。

#### 8.7 并发测试

- 同一凭证的重复捕获投递不少于 3 次，只产生一条条目。
- 关账受理与在途成本捕获事务的交叠：受理后到达的事件顺延入账，其条目与凭证落入同一期间，不进入本次校验快照，不改变关账结论。
- 同一报表对象版本的并发编辑，乐观锁冲突返回 409 并回带当前版本号与最后修改人。
- 只读池打满：11 个并发分析查询，第 11 个按池上限排队，超过 SYNC_BUDGET_MS 后返回明确失败而不是无限等待。
- 渲染任务的并发抢占：两个 job-worker 协程同时抢占同一任务，只有一个成功。

### 9. 退出条件

全部条目可由自动化用例或可复核证据判定。

1. 11 张新表、3 个视图、18 条迁移（4 条在 db/migrations/costing/，14 条在 db/migrations/reporting/）全部在空库与含基准数据集的库上执行通过，其中 costing 四条与 reporting 第 1、2、11、12 号四条已在贯通线 T0 期间执行，本阶段核对其已生效且不重复执行；每条迁移的 rollback 段在影子库上验证可执行；迁移会话锁持有不超过 5 秒、执行不超过 30 分钟。
2. 全部带法人的新表已 ENABLE 且 FORCE 行级安全，运行期账号不具备 BYPASSRLS 与 SUPERUSER；tests/rls_matrix 的 costing 与 reporting 扩展八类全绿。
3. 六个事件类型的成本与收入捕获与凭证同事务写入，重复投递 3 次只产生一条条目，过账回滚不留残条目。
4. 三个 ReconCheck 实现并经 ReconRegistry::register 注册成功，注入差额后差异事项在 platform_core.recon_discrepancies 生成且可追溯、关账被拦截，清零后关账通过；每日校验与关账前强制校验两条路径各验证一次。
5. 成本归集查询在五个主维度上各维度合计加未分摊差异等于总额，由属性测试与端到端用例双重断言；未分摊差异以独立对象返回，不出现在 rows 数组中。
6. 四类预置指标出具，三层下钻可达单据行并可跳转原单据；收入侧、成本侧、利润侧三个未分摊差异桶取值正确，利润侧负值原样返回；交付指标不设未分摊差异桶，按产品与订单下钻时返回 scope_narrowed。
7. 收入、成本、利润三项与总账科目余额在法人、会计期间与科目合计层面差额为零；存货类成本合计与库存金额账出库金额合计差额为零；交付指标与交付节点、分批交付单据逐项一致。
8. 两张账龄基础表出具，分档可配置，账龄按原始业务日期与到期日计算，顺延入账不改变账龄，负余额单列而不静默修正；分档断档与重叠被拒绝。
9. 报表定义、自定义指标、仪表盘、打印模板四类对象的六条状态迁移全部通过，自审被拒，已发布版本在新草稿发布前继续生效；依赖失效可检出并使运行端点拒绝返回结果。
10. 默认管理驾驶舱与默认账龄分档在两个法人上均由 job-worker 幂等播种成功，安装后不做任何配置即可查看。
11. 全部报表与指标取数经统一前置查询服务；不存在任何绕过该服务直连只读角色的代码路径，由 CI 断言 crates 中除 ep-adapter-db-pg 之外不出现只读池句柄的构造。
12. 高级只读 SQL 的允许清单 8 项与拒绝清单 12 项逐项验证通过；只在报表定义 spec 内可用，不存在裸 SQL 执行端点。
13. 语句超时与单查询资源上限实测触发，终止后返回明确失败、不返回部分结果、写入 platform_ops 台账并计入指标。
14. 导出与打印一律为后台任务，敏感导出经重新认证与审批，产物落为附件对象并有站内通知回执；移动端发起打印渲染被拒并提示转桌面端。
15. 附录 A.1 中属本阶段的三项常用报表 P95 在 10 秒内，样本不少于 200 次，含备份负载条件，EXPLAIN 证据显示无顺序扫描。
16. 覆盖率达到第 8 节门槛，无长期 ignore 用例。
17. docs/error-codes.md 新增 36 条、docs/event-catalog.md 新增 3 条、docs/data-dictionary 两节、五份 ADR 全部提交；docs/data-dictionary.md 的单据类型码一节含本阶段的 RT 一行且 xtask configdoc --check-doc-type-codes 通过；基线回写完成，含 5 个指标、13 个配置键、D-11-01 至 D-11-05 五条。
18. 规格第 17.2 章十五类必测分支中的第四、七、十三、十五类端到端通过，规格第 8 章闭环第 14 步的指标一致性用例通过，执行记录纳入发布证据包；闭环十四步整条链路的贯通验收由阶段 9b 的 testkit/scenarios/golden_loop_14_steps.rs 承担，不在本阶段判定。
19. 四个报表类 ConfigItemApplier 已实现并注册进阶段 3a 交付的 ConfigItemApplierRegistry；四类对象各经阶段 3b 的配置发布通道完成一次发布与一次回退，apply 幂等、rollback 不删除版本行。
20. costing 与 reporting 两个模块在规格第 6.2 章能力矩阵中取值为完整或简化的能力域，其四端界面已实现并通过 Playwright 与 tauri-driver 的桌面用例、XCUITest 与 Espresso 的移动用例；取值为 VIEW_ONLY 的能力域只实现只读视图；取值为 NOT_APPLICABLE 的不实现入口。
21. 本阶段全部路由的能力域码与动作类别常量已在 ep-contract-costing 与 ep-contract-reporting 的 src/capability.rs 声明，xtask configdoc 通过。
22. 三个 ReconCheck 的 code、category 与 blocks_period_close 取值与第 6.6 节一致，run_batch 在阶段 9a 提供的快照上分批执行，未完成批次以 UNFINISHED 上报而不是静默截断。
23. 账龄分档的唯一出处迁移完成：finance.aging_bucket_definitions 的数据已由 db/migrations/reporting/ 第 13 号迁移迁入 reporting.aging_bucket_profiles 与 reporting.aging_bucket_lines，该表已由同目录第 14 号迁移删除，finance 侧台账内账龄查询改经 AgingBucketQuery::buckets，两处分档逐档一致。
24. costing.stock_value_adjust 消费者已在 job-worker 注册，重复投递同一 inventory.stock_movement.value_adjusted.v1 只补记一条成本条目，幂等由 platform_msg.inbox_consumptions 保证，载荷缺凭证引用时进入死信。
25. 13 个外部数据集的目录行已登记且列签名与来源视图一致，启动自检项 reporting-dataset-signature-matched 在 core-server 与 job-worker 上通过；该自检项的 severity 为 Degrading，注入一次列签名漂移后进程照常启动、受影响数据集与依赖它的报表对象入口关闭、降级窗口开出并告警，同一库上 --check 以非零退出；project_projects 一行在阶段 12 交付视图前即处于该降级窗口内，视图交付后窗口关闭。
26. 贯通线 T0 的收入切片按第 0.0 节判据已在 T0 时点验收通过，本阶段核对加厚过程未改变该切片确立的取数路径与恒等口径：收入卡的取值与该法人该会计期间收入科目贷方净发生额差额仍为零，该断言在 ep-datagen 最小样本与规格附录 A.3 的 scale 数据集上各执行一次。

### 10. 与规格和 PRD 的对应

#### 10.1 规格条目

| 规格位置 | 本阶段实现的内容 |
|---|---|
| 第 5.2 章成本归集与销货成本结转条目 | 三类成本来源的捕获与归集、未分摊差异的两处定义、按合同订单客户项目产品的成本归集查询与下钻、直运订单成本由直接费用类承载、已退货未冲回成本标注、毛利按收入减成本 |
| 第 5.2 章财务规则条目总账功能与期末处理块 | 子账与凭证共用同一期间归属在成本与收入归集条目上的落实；账龄仍按原始业务日期与到期日计算 |
| 第 5.5 章报表与拖拽分析条目 | 报表设计器、仪表盘、企业自定义指标、像素级打印模板、权限继承、数据源为同一实例独立只读角色、经营口径成本三类之和与总账主营业务成本余额可对平 |
| 第 5.5 章经营驾驶舱预置指标与数据集条目 | 收入成本利润交付四类指标、五个下钻维度、三侧未分摊差异、两类期间口径、产品维度只对订单分批交付部分可用、默认管理驾驶舱与两张账龄基础表 |
| 第 7.7 章数据库账号与连接模型 | ep_analyst_ro 独立连接池的取用、会话变量注入与清除、不使用 SET ROLE、超限终止记入运维中心 |
| 第 7.9 章派生存储安全继承 | 同实例只读角色适用全部越权控制；查询期按法人记录字段密级过滤；调用方插件低代码不得直连；不得通过排序聚合分面计数间接暴露；只读角色不持有绕过前置服务的字段级密钥 |
| 第 8 章黄金业务闭环第 14 步与闭环验收 | 管理层看数的完整实现与三处一致的验收用例 |
| 第 9.1 章高级只读 SQL | 解析、权限重写、语句超时、资源上限与结果限量控制 |
| 第 10.2 章主系统规则 | 三个新勾稽项进入每日校验与关账前强制校验，差异事项拦截关账 |
| 第 16 章性能与容量 | 常用报表 P95 在 10 秒内的三项；分析负载与交易负载靠角色连接池与资源限额隔离，不表述为隔离保证 |
| 第 17.2 章财务内核测试 | 经营指标三处一致、下钻恒等、交付逐项一致，以及第四七十三十五四类必测分支 |
| 第 17.3 章强制不变量 | 应收应付核销守恒作为账龄基数；子账与总账勾稽新增三项；会计借贷平衡在捕获路径上不被破坏 |
| 第 21.20 章 | 两类期间口径差异与顺延导致数值变化的界面披露 |
| 第 22 章第 6 条 | 成本归集三类来源、驾驶舱四类指标与两张账龄表、下钻合计加未分摊差异等于总额的验收证据 |
| 附录 A.1 与 A.2 | 三项常用报表的度量与统计口径 |

#### 10.2 PRD 节点

| PRD 位置 | 本阶段实现的功能 |
|---|---|
| 8.1 | 七类角色的操作范围与权限继承 |
| 8.2.1 | 三类成本来源的承载单据、归集维度来源与金额出现时点 |
| 8.2.2 | 查询输入的三项必填、五个主维度、七列输出、两层下钻 |
| 8.2.3 | 未分摊差异的三条口径、恒等关系、末行固定与不参与排序 |
| 8.2.4 | 已退货未冲回成本的标注、筛选与计入成本合计 |
| 8.3.1 | 四类预置指标的口径、期间维度取数、下钻维度与未分摊差异 |
| 8.3.2 | 三层下钻、三侧未分摊差异、恒等关系、一致性口径 |
| 8.3.3 | 两类期间口径的界面披露、关账不冻结查询、取数时点与顺延提示 |
| 8.3.4 | 应收与应付账龄两张基础表及其下钻 |
| 8.3.5 | 默认管理驾驶舱开箱可用 |
| 8.4.1 至 8.4.4 | 报表设计器、企业自定义指标、仪表盘、像素级打印模板 |
| 8.4.5 | 报表类配置对象的四状态与六条流转、审批不可自审 |
| 8.5 | 权限继承、取数隔离、四端取值、时延、导出与打印的高风险控制、跨法人不合并 |
| 8.6 | 六类异常的系统处理与用户可见结果 |
| 8.7 | 首版不含清单，本阶段不实现其中任何一项 |
| 6.9.3 与 6.10.3 | 账龄分档的唯一出处与计算基数，由 finance 经契约取用 |
| 10.4.6 | 报表定制的能改与不能改边界、数据源、验证与回退 |
| 11.3 | 常用报表通过线与临时分析查询不设通过线的区分 |

### 11. 风险与预留

#### 11.1 技术风险

| 风险 | 影响 | 控制 |
|---|---|---|
| R-1 只读分析负载与交易负载在同一实例上抢占。规格第 13.1 章明确不表述为隔离保证 | 报表跑起来时交易时延劣化，可能击穿普通交易提交 P95 在 3 秒内 | 只读池上限 10 且与读写池合计不超过 30；statement_timeout 60 秒硬截断；聚合结果 2000 行上限；导出一律转 job-worker 并受 app-worker.slice 配额约束；性能测试必须在同时施加备份负载的条件下取样；该风险在交付说明中按规格第 21.19 章明写，不承诺隔离 |
| R-2 六个维度索引加在仅追加的大表上，写放大影响过账时延 | 交付确认、采购发票登记等提交类操作的 P95 上升 | 在基准数据集上先测捕获路径的单条插入耗时；若超出预算，退路是把三个低频维度索引（项目、产品、客户）合并为一个覆盖索引并在查询侧接受一次额外过滤，该退路不改变任何口径 |
| R-3 D-11-01 的跨模块只读投影使 reporting 的运行期正确性依赖 13 个外部数据集视图的列签名，其提供阶段为 5、6、8、9a、10、12 六个 | 来源模块改列会使报表在运行期失败 | 主控制是 CI 的视图列签名快照测试，来源模块改列即构建失败，挡在发布之前；运行期由启动自检项 reporting-dataset-signature-matched 按 D-11-04 降级处置，关闭受影响入口并开窗告警而不阻断启动，阶段 12 的 project_projects 在其视图交付前即处于该窗口内；视图被界定为契约级，变更走契约变更流程；各提供阶段按 A-18 在本阶段之前发布视图并同步列签名 |
| R-4 A-1 与 A-2 两条假设把捕获绑死在 ledger 的过账用例上，而该用例由阶段 9a 先行交付 | 本阶段要在别的模块的既有用例内追加调用点，可能与其事务边界或科目判定冲突，改不动即无法在同事务捕获 | 调用点、两个捕获实现与 wiring 注入由本阶段同批交付，见第 2.2 节 ep-app-ledger 一行、第 2.3 节与第 4.2 节；阶段 9a 不预留空实现，本阶段也不向其派工；接口形状由本阶段在 ep-contract-costing 自定，改动范围限于过账用例内的一段调用与一条 crate 依赖，依赖方向由 CI 的 cargo metadata 断言脚本守住；退路为 Outbox 异步补写，代价是正常运行期出现秒级差额，需要同时放宽 PRD 第 8.3.2 节的差额为零判定到关账时点判定，属口径变更，必须由财务负责人批准 |
| R-5 高级只读 SQL 的白名单可能被绕过，成为越权入口，对应规格第 21.7 章 | 报表成为越权读取通道 | AST 白名单而不是正则黑名单；执行只在受行级策略约束的 ep_analyst_ro 上，即使解析层被绕过，法人隔离仍由数据库承担；模糊测试对解析器施加不少于 10 万条变异输入；tests/analytics_isolation 属发布门禁项 |
| R-6 未分摊差异桶可能显著偏大，取决于 U-F-06 的决策 | 管理层下钻可用性下降，指标被质疑 | 本阶段在成本归集查询响应中固定返回 unallocated_ratio，实施期可据此度量；不做任何分摊，也不隐藏 |
| R-7 13 条未决事项若在阶段末仍未决策，交付的是临时取值 | 客户验收期返工 | 除 U-C-12 外的临时取值集中在配置键与两处纯函数中，第 0.2 节已逐条给出切换代价；U-C-12、U-I-06 与 U-I-08 三条属结构性，若决策与临时取值相反需要重新评估工期，其中 U-C-12 的临时取值落在第 4.6 节状态机与第 4.8 节四个 ConfigItemApplier 上 | 

#### 11.2 为后续阶段预留的扩展点

- 数据集注册表与字段目录是唯一的取数入口，后续恢复独立只读副本或其他派生存储后端时，只需新增 source_view 的解析方式与连接选择，报表定义与自定义指标无需改动。
- QueryPlan 是查询构造的唯一中间表示，后续引入缓存或物化（U-I-06 若翻转）时，失效键可直接取 QueryPlan 的哈希与其依赖数据集集合。
- ReportSpec 为四变体枚举加版本号，后续新增报表类型只增变体，状态机、审批、权限与发布通道不变。
- CostCaptureCommand 预留 extra_dimensions jsonb 字段但首版不写入、不索引、不查询，供后续恢复成本中心与责任中心核算时承载新维度而不改表结构；该字段在首版由 CHECK 强制为空对象，避免成为绕过范围收窄的入口。
- 账龄分档以 profile 加 lines 建模并按 ledger_side 区分，后续若财务侧要求应收与应付分别配置或按客户分级配置，只需增加 profile 行与一个绑定表，不改分档算法。
- 三个新对账校验项按阶段 9a 交付的 ReconCheck trait 与 ReconRegistry::register 实现与注册，后续新增勾稽项复用同一注册方式与 platform_core.recon_discrepancies 的差异事项模板。
- 渲染任务的 task_kind 为可扩展枚举，后续恢复报表订阅与定时分发（首版不含）时可复用同一任务台账与产物路径，只增调度来源。
