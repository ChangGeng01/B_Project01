## 阶段 9：财务内核一 —— 总账与期间

### 9.0 本阶段的边界、口径来源与阅读方式

本阶段建设 ledger 模块码下的全部原生能力：会计科目表、期初余额、事件科目对应关系、凭证模型、由规格第 5.2 章事件-分录表驱动的记账引擎、会计期间与可入账期间、记账日期与顺延入账、期间关账的受理前提与关账前强制校验的编排、年度损益结转、试算平衡与三张账表查询。

本阶段不定义任何借贷方向、取价、价差拆分、匹配与核销规则。上述规则一律按规格第 5.2 章财务规则条目的事件-分录表及其后的七个规则块执行。本计划在需要时按事件名称或规则块名称指向该处，不复述其内容。凡本计划出现分录相关表述，一律限于承载结构、映射表的形状、校验与幂等，不涉及规则本身。

本阶段不建设应收应付台账、发票台账、库存台账与成本归集，四者分别归阶段 10、阶段 10、阶段 8 与阶段 11。子账侧取数在关账勾稽中一律经 ep-contract-finance 的 ReconciliationItemQuery 按法人与会计期间取十项勾稽的子账侧合计，结构为 ReconciliationItemView，该 trait 由阶段 10 定义；阶段 8 在 ep-contract-inventory 定义的 StockValueSubledgerBalancePort 与阶段 7 在 ep-contract-procure 定义的 GrniSubledgerBalancePort 是阶段 10 内部组装该结果的手段，本阶段不直接调用。十项中的存货与已收货未收票两项，其子账侧实现体分别由阶段 8 的 InventorySubledgerBalanceQuery 与阶段 7 的 GrniSubledgerBalanceQuery 各自在本模块 contract 的端口上实现，阶段 10 只注入，其余八项取自阶段 10 自有表、不经这两个端口。本阶段只提供总账侧余额。本阶段也不建交付确认单，该单据按 A-09 归阶段 6 的 sales schema，本阶段只提供其收入与成本腿所调用的过账端口。

取值优先级按共享技术基线第 0 节：规格第 13.1、13.3、13.4、7.7 章最高，其次规格其余各章，其次 PRD，最后共享技术基线。本计划中标注为本阶段新增决定与偏离项的条目集中在第 9.12 节，评审时按该节逐条核对。文中出现的按裁定 A-nn、B-nn、C-nn 一律只是决策出处标注，取值以本计划正文与共享技术基线为准，二者不一致时以正文为准，任何取值都不得以裁定表为唯一出处。
#### 9.0.1 本阶段的两段拆分

本阶段按总览第 3.3 节的拆环结论切成 9a 与 9b 两段，落在裁定通则第四条固定的顺序 1 → 2 → 3a → 4 → 3b-1 → T0 → 5 → 9a → 8 → 6 → 7 → 10 → 11 → 9b → 14 上，9a 排在阶段 8 之前、9b 排在阶段 11 之后。阶段 3b-2 不在这条链上，其各项按阶段 3 计划第 3.0 节判定四的下游拉动点排在 T0 之后、各自拉动阶段之前；阶段 12 在阶段 10 之后与阶段 11 并行，阶段 13 在阶段 12 之后与阶段 9b 并行，两者都不在本阶段的前置或后继链上。本计划各节凡涉及分段处一律按下列分工判读。

9a 段交付：ep-contract-ledger、ep-domain-ledger、ep-app-ledger 与 ep-platform-recon 四个 crate；ledger.accounts、ledger.accounting_periods、ledger.event_account_bindings、ledger.opening_balance_batches、ledger.opening_balance_batch_lines、ledger.vouchers、ledger.voucher_lines、ledger.account_period_balances、ledger.posting_trigger_event_types 九张表与 ledger.v_account_period_balances、ledger.v_pending_posting_backlog 两个视图；platform_core 下对账的三张表；AccountingPeriodResolver、PostingPort 与 TotalAccountBalanceProvider 三个对外契约；对账框架本体、分批执行器与每日对账调度；总账期初余额通道；受治理数据集视图与 platform_core.append_only_registry 的登记行；本模块四端界面中科目表、凭证与账表的部分。

9b 段交付：ledger.close_serialization_slots、ledger.period_close_requests、ledger.year_end_closings 三张表；关账请求状态机、受理与在途写事务等待、快照建立、关账前强制校验的编排与四类校验项的注册；年度损益结转；期间关闭时的下一期间期初固化；黄金业务闭环十四步的整体端到端用例 testkit/scenarios/golden_loop_14_steps.rs；本模块四端界面中关账发起跟进与年结发起的部分。

四类关账前强制校验项在 9b 段实现并向 ReconRegistry 注册，9a 段只交付对账框架本体与调度，不注册本模块的校验项。ReconCheck 的注册方按裁定 A-06 固定为阶段 7、8、9b、11 四个，校验项数依次为六、二、四、三共十五个，一律在 apps/job-worker/src/wiring/ 目录下经 ReconRegistry::register 注册，其中 9b 段的四个即本段自带的四类。阶段 10 不注册任何 ReconCheck：其原定的 FIN_CROSS_MODULE_LINK 是纯存在性项，跨 schema 单目标引用改建真实外键后已按 A-06 整条删除；该阶段的十个勾稽项由本段第三个校验项经 ep-contract-finance 的 ReconciliationItemQuery 取子账侧合计，不由阶段 10 自行注册。

#### 9.0.2 本阶段在 T0 贯通线上的最小切片

T0 是插在阶段 3b-1 结束之后、阶段 5 全量开工之前的一条贯通线，其前置为阶段 1、2、3a、4 与 3b-1，在总览第 3.4 节的十五个环节中居第六环。T0 不新增任何范围，只从阶段 5、6、9a、10、11 各取最小切片，判据是一份合同从建单走到管理层看到一个数。本阶段在 T0 中只贡献两样东西：一个会计期间与一张凭证。

本阶段贡献的切片逐项如下：ledger.accounts 与手工建立的少量科目；ledger.event_account_bindings 与该条路径用到的科目角色绑定；ledger.accounting_periods 与该法人的首个 OPEN 期间，该期间由 AccountingPeriodResolver::resolve 第二步的零期间分支在首次过账的同一业务事务内按记账日期所属自然月建立，不经任何端点也不经测试夹具预置；AccountingPeriodResolver::resolve 的第一至第三步且含第二步的零期间分支，顺延的第四第五步在 T0 上走不到；PostingPort::post 与 JOURNAL_MAP 中 SALES_INVOICE_ISSUED 与 RECEIPT_REGISTERED 两个来源类型的行；ledger.vouchers、ledger.voucher_lines 与 ledger.account_period_balances 三张表及其同事务写入；只读端点 GET /api/v1/ledger/account-balances，供 T0 判据读取 ledger 侧当期收入科目的贷方净发生额，与阶段 11 那张收入报表上的数做差额为零的对照，阶段 11 收入卡自身的取数来自其 costing 侧归集台账，不经本端点。

本阶段不进入 T0 的部分：期初余额批次、顺延入账、关账全链路、年度损益结转、对账框架、账表的其余端点、四端界面、性能度量，以及 9b 段全部内容。T0 只用 ep-datagen 最小样本，不要求 scale 数据集，不要求分支覆盖，只要求桌面端。

迁移落法：T0 段执行 db/migrations/ledger/ 的第 1、2、3、6、7、8、12 号与第 13 号共八个文件。第 12 号必须与第 13 号同批执行：第 13 号建立的 ledger.v_pending_posting_backlog 连接第 12 号建立的 ledger.posting_trigger_event_types，第 12 号不在 T0 内时该视图在空库上建不起来，且第 13 号会永久取得早于第 12 号的版本号，使版本号必须晚于其全部被引用对象这条判据在 9a 全量段之后的空库回归上同样失败。第 12 号在 T0 段只是一张不带法人列的空登记表，T0 不跑关账受理，v_pending_posting_backlog 在零行登记表上恒返回零，纳入 T0 无附带成本。第 14 号种子迁移不进 T0：该文件由 xtask configdoc 从 docs/event-catalog.md 的 produces_voucher 列生成并逐字比对，T0 时点事件目录尚未补全，此时落库的行数与 9a 全量段的生成结果不符，只能靠改写一个已应用的迁移文件才能让比对通过。其余八个文件中，第 4、5、14、15、16 号在 9a 全量段追加，第 9、10、11 号按第 9.3 节表下的分段说明在 9b 段追加。全部文件的版本号按各自执行日期取，因此 T0 段的版本号早于 9a 全量段，全局版本序仍单调递增，不产生乱序到达。

T0 通过之后，本阶段其余部分一律改为在这条已贯通的骨架上加厚，不再承担首次贯通的职责：9a 全量段加厚的是期初余额、顺延入账、期初固化、对账框架与账表；9b 段加厚的是关账与年结。第 9.9 节的退出条件不因 T0 增减，仍在 9a 与 9b 各自结束时判定；T0 自身的判据落在 T0 那条线上，本阶段不重复登记。


### 9.1 交付物清单

本阶段结束时，下列东西存在且可运行。

一是四个新增 crate 并可编译通过：ep-contract-ledger、ep-domain-ledger、ep-app-ledger 与 ep-platform-recon，加上 ep-adapter-db-pg 中新增的 ledger 与 recon 两组仓储实现文件组。

二是 db/migrations/ledger/ 下的 16 个迁移文件可在空库上离线执行完成，并可在全局唯一的迁移历史表 platform_core.schema_history 上查得版本；执行后 ledger schema 存在 12 张表与 2 个视图，全部带法人列的表均已 ENABLE 与 FORCE 行级安全。另有 db/migrations/platform_core/ 下的 3 个迁移文件建立对账框架的三张表，按 A-06 同属本阶段 9a 段。

三是记账引擎可用：任一业务模块的用例在其事务内经 ep-contract-ledger 的 PostingPort 提交一次过账输入，同事务内生成一张借贷平衡的总账凭证与其分录行、增量更新科目余额、写入审计事件、写入一条 ledger.voucher.posted.v1 的 Outbox 条目。映射表以编译期常量表形式存在，键为来源类型与计量项两项的一维查表，规格第 5.2 章的十类事件按分录集合展开为 17 个来源类型，可被单元测试与领域属性测试逐条遍历。

四是会计期间可用：该法人的首个期间由首次过账在同一业务事务内按记账日期所属自然月建立并置为打开，其后各期间按自然月自动建立并置为打开，job-worker 上的定时任务提前建立下一期间；期间归属解析函数可用，含零期间时建立首期、顺延入账与顺延目标不存在时的自动建立。

五是期间关账可用：从发起、重新认证、审批、受理前提判定、受理、等待在途写事务、建立快照、分批执行关账前强制校验，到四种结束方式，全链路在应用内可达，无需线下动作。

六是年度损益结转可用：可在年度末次期间为可入账期间时执行，可重复执行，结转凭证按事件-分录表之外的期末处理块生成。

七是账表查询可用：科目余额表、总账、明细账、试算平衡、会计恒等取数五个只读端点，可按会计期间字段与按原始业务日期两条路径检索，顺延入账的凭证在两条路径上均可查得。

八是文档产物：docs/error-codes.md 新增 LEDGER 段共 34 个错误码，比原定的 36 个少两个，一个是两类计量项错误码合并为 LEDGER.POSTING.MEASURE_INVALID，另一个是随过账事件登记表的运行期断言一并删除的 LEDGER.POSTING_TRIGGER_EVENT_TYPE.REGISTRY_MISMATCH；docs/event-catalog.md 新增 ledger 段共 8 个事件；docs/data-dictionary/ledger.md 新增 12 张表的数据字典，docs/data-dictionary/platform_core.md 增补对账三张表的数据字典；docs/adr/ 新增 3 篇本阶段决定的 ADR。

九是测试产物：ep-domain-ledger 与 ep-app-ledger 的单元测试与领域属性测试、crates/application/ledger/tests 下的集成测试、tests/rls_matrix 中新增的 ledger 越权用例、apps/core-server/tests 下的关账与顺延入账端到端用例、9b 段的 testkit/scenarios/golden_loop_14_steps.rs，以及 A.1 度量清单中总账凭证过账与月度科目余额表两项的 EXPLAIN 证据文件。
十是对账框架本体：ep-platform-recon crate、platform_core.recon_check_definitions 与 platform_core.recon_runs 与 platform_core.recon_discrepancies 三张表、ReconCheck 与 ReconRegistry 与 ReconExecutor 三个契约、job-worker 内的分批执行器与每日对账调度、差异事项 subject_ref 的键集白名单校验。按 A-06 该本体归本阶段 9a 段，注册方为阶段 7、8、9b、11 四个，各自在其上实现自己的 ReconCheck，不另起对账框架。

十一是本模块的四端界面：clients/desktop/src/modules/ledger/ 与 clients/mobile/src/modules/ledger/ 两个目录，按 A-23 由本阶段交付，阶段 13 只提供客户端壳、路由注册表与能力矩阵闸。

十二是受治理数据集视图：ledger.v_account_period_balances 按 A-18 输出 legal_entity_id、security_level、data_scope_tags 三列，dataset code 为 ledger_account_period_balances，grain 为 SNAPSHOT，并已 GRANT SELECT 给 ep_analyst_ro。

十三是能力域码与动作类别常量：crates/contract/ledger/src/capability.rs 中为每个用例声明一对常量，按 A-20 供 xtask configdoc 解析。


### 9.2 crate 与进程归属

新增 crate 四个，均按基线第 1.1 节的路径与命名。

| crate | 路径 | 职责 | 装配进入的进程 |
|---|---|---|---|
| ep-contract-ledger | crates/contract/ledger | 对外公开的命令、查询、事件类型、DTO，以及供其他模块调用的 trait，只依赖 ep-foundation | 被 core-server 与 job-worker 装配，且被其他模块的 ep-app-* 依赖 |
| ep-domain-ledger | crates/domain/ledger | 科目、期间、凭证、关账请求、年结四个聚合，事件到分录的编译期映射表，期间归属算法，余额推演，业务端口 trait | core-server、job-worker |
| ep-app-ledger | crates/application/ledger | 用例、事务边界、授权调用、审计与 Outbox 写入、关账编排、账表投影组装 | core-server、job-worker |
| ep-platform-recon | crates/platform/recon | 对账框架本体：ReconCheck 与 ReconRegistry 与 ReconExecutor 三个契约、BatchWindow 与 ReconRunOutcome、差异事项与校验未完成事项模型、按法人逐轮遍历的分批执行器与每日调度 | core-server、job-worker |

改动 crate 两个。

ep-adapter-db-pg 新增 src/repo/ledger/ 与 src/repo/recon/ 两个目录。前者按表分文件实现 ep-domain-ledger 的仓储端口，只访问 ledger schema；后者实现 ep-platform-recon 的仓储端口，只访问 platform_core 下对账的三张表。两者均不访问其他模块 schema，由 CI 的分层自检断言。

apps/core-server/src/wiring/ 与 apps/job-worker/src/wiring/ 两个目录下新增 ledger 的具体实现注入，含把 ep-app-ledger 的 PostingPort 实现与 AccountingPeriodResolver 实现注入到其他模块的用例构造器。job-worker 的 wiring 另装配 ReconRegistry 与 ReconExecutor，各阶段的 ReconCheck 实现一律在该处经 ReconRegistry::register 注册。除这两个目录外任何地方不得 use ep_adapter_db_pg。

进程归属逐项如下。

core-server 承载：科目表维护、期初余额、事件科目对应关系、凭证与账表查询、关账请求的发起与主动取消、年度损益结转的发起、以及由业务模块用例同事务调用的记账引擎。

job-worker 承载：期间自动建立定时任务、ep-platform-recon 的分批执行器与每日对账调度、关账受理前提判定、受理、在途写事务等待、快照建立、关账前强制校验的分批执行与结论落库、年度损益结转审批通过后的执行、以及科目余额固化。

本阶段不新增进程，不新增 schema，不新增模块码，不新增错误分类，不新增依赖方向。

依赖方向自检：ep-domain-ledger 只依赖 ep-foundation 与 ep-contract-ledger；ep-app-ledger 依赖 ep-foundation、ep-platform-authz、ep-platform-audit、ep-platform-outbox、ep-platform-sequence、ep-platform-flow、ep-platform-recon、ep-platform-release、ep-platform-notify、ep-platform-obs、ep-domain-ledger、ep-contract-ledger 与 ep-contract-finance，其中 ep-contract-finance 一项只用于 9b 段消费 ReconciliationItemQuery，按基线第 1.3 节 ep-app-<模块> 可依赖任意模块的 ep-contract-* 成立；ep-app-ledger 不依赖任何其他模块的 ep-app-*，也不依赖其他模块的 ep-domain-*。ep-platform-recon 只依赖 ep-foundation、ep-platform-obs 与 ep-platform-tenancy，不依赖任何模块的 ep-contract-*、ep-domain-* 与 ep-app-*，各模块的 ReconCheck 实现落在其自身的 ep-app-* 中并在 wiring 注册，该方向由同一自检脚本断言。其他模块经 ep-contract-ledger 的 trait 反向调用本阶段，实现在 wiring 注入。阶段 11 按其第 2.2 节与第 4.2 节在本阶段的过账用例内追加成本与收入捕获的调用点，并为 ep-app-ledger 新增一条对 ep-contract-costing 的依赖；该调用点按同批交付处理，即它与阶段 11 的实现同批加入代码，在此之前根本不存在于代码中，本阶段没有任何未接线端口注入点，不注入任何空实现，届时该依赖随阶段 11 的实现同批加入，依赖方向由 xtask archcheck 的七条禁止项按层位断言，不存在也不新建按 crate 逐项比对的期望依赖清单；本段的依赖枚举一律是本阶段结束时的快照，后续阶段可在基线第 1.3 节允许项内增边，不回改本段（裁定 F-05 通则甲-2 与甲-3）。

### 9.3 数据库变更

全部对象建在 ledger schema，属主为 ep_mod_ledger，运行期由 ep_app_rw 读写。迁移目录 db/migrations/ledger/，迁移历史落在全局唯一的 platform_core.schema_history。执行顺序由单一全局 Runner 按文件名版本号排序决定，db/migrations/order.toml 与其中的目录位次一并删除，本阶段不声明任何目录之间的先后，跨目录的先后一律由版本号本身表达。

下表的时间戳示例只固定相对顺序与 slug。实际文件名一律取 V<YYYYMMDDHHMMSS> 十四位真实执行时间，全局版本号唯一且严格递增，由 xtask sqlcheck 断言；伪时间戳与十二位写法不得进入仓库。

| 序 | 文件名 | 内容 |
|---|---|---|
| 1 | V202611030900__ledger_create_accounts.sql | 建 ledger.accounts |
| 2 | V202611030905__ledger_create_accounting_periods.sql | 建 ledger.accounting_periods |
| 3 | V202611030910__ledger_create_event_account_bindings.sql | 建 ledger.event_account_bindings |
| 4 | V202611030915__ledger_create_opening_balance_batches.sql | 建 ledger.opening_balance_batches |
| 5 | V202611030920__ledger_create_opening_balance_batch_lines.sql | 建 ledger.opening_balance_batch_lines |
| 6 | V202611030925__ledger_create_vouchers.sql | 建 ledger.vouchers |
| 7 | V202611030930__ledger_create_voucher_lines.sql | 建 ledger.voucher_lines |
| 8 | V202611030935__ledger_create_account_period_balances.sql | 建 ledger.account_period_balances |
| 9 | V202611030940__ledger_create_close_serialization_slots.sql | 建 ledger.close_serialization_slots |
| 10 | V202611030945__ledger_create_period_close_requests.sql | 建 ledger.period_close_requests |
| 11 | V202611030950__ledger_create_year_end_closings.sql | 建 ledger.year_end_closings |
| 12 | V202611030955__ledger_create_posting_trigger_event_types.sql | 建 ledger.posting_trigger_event_types |
| 13 | V202611031000__ledger_create_ledger_views.sql | 建 ledger.v_account_period_balances 与 ledger.v_pending_posting_backlog |
| 14 | V202611031005__ledger_backfill_posting_trigger_event_types.sql | 一次写全 ledger.posting_trigger_event_types 的 13 行，每行只填 event_type，清单见第 9.3.11 节；本文件由 xtask configdoc 从 docs/event-catalog.md 的 produces_voucher 列生成，CI 比对生成结果与仓库中的本文件是否逐字一致，不一致即构建失败；业务阶段不再追加任何回填迁移 |
| 15 | V202611031010__ledger_create_dataset_views.sql | 按 A-18 重建 ledger.v_account_period_balances 使其输出 legal_entity_id、security_level、data_scope_tags 三列，并 GRANT SELECT ON ledger.v_account_period_balances TO ep_analyst_ro |
| 16 | V202611031015__ledger_backfill_append_only_registry.sql | 按 B-02 向 platform_core.append_only_registry 登记 ledger.vouchers、ledger.voucher_lines 与 platform_core.recon_runs 三行，三行的 mode 一律取 APPEND_ONLY、mutable_columns 取空数组。文件内先插三行登记，再依次调用 platform_core.attach_table_guards('ledger','vouchers')、('ledger','voucher_lines')、('platform_core','recon_runs')，顺序不得颠倒，挂接函数读登记表取可变列白名单，先挂接后登记取不到 mutable_columns。第 1 至 13 号迁移一律不调用 attach_table_guards，这三张仅追加表的触发器只在本文件内挂接；platform_core.recon_runs 的建表迁移在 db/migrations/platform_core/ 目录且版本号早于本文件，本文件执行时该表已存在，跨目录挂接可行。本文件同时写入 ledger 与 platform_core 两个 schema 的登记对象，其主要创建对象是 ledger 两张仅追加表的登记行与触发器，故放在 db/migrations/ledger/ 目录下，正确性由本文件版本号晚于其全部被引用对象保证，并由空库全量执行验证 |

每个文件头部按基线第 3.9 节写 -- rollback: 段。建表类的回退语句为 drop table；第 13 号的回退为 drop view；第 14 号为按 event_type 删除本次插入的 13 行；第 15 号为按第 13 号的定义重建视图并 REVOKE SELECT ON ledger.v_account_period_balances FROM ep_analyst_ro；第 16 号为按 schema_name 与 table_name 删除本次登记的三行，并 drop 该三张表上由本文件挂接的 assert_append_only 触发器。第 6、7 号文件另注明其中的 REVOKE 语句无法安全逆向，回退须用升级前备份。

上表第 9、10、11 号三个文件属 9b 段，其余十三个属 9a 段。两段之间隔着阶段 8 至 11，因此 9b 段三个文件的编号按其执行日期取，排在这些阶段的迁移之后，ledger 目录内的相对次序仍按上表。第 2 号表上的 closed_by_close_request_id 外键随第 10 号文件在 9b 段补建，该列在 9a 段保持可空且无外键。

对账框架的三张表按 A-06 建在 platform_core schema，迁移文件放在 db/migrations/platform_core/，slug 依次为 platform_core_create_recon_check_definitions、platform_core_create_recon_runs、platform_core_create_recon_discrepancies，三者同属 9a 段。列按 A-06 的定义：recon_check_definitions 不带法人列，其准入判据是行集合与法人无关，行由制品决定并在本部署内对全部法人取值相同，隔离承接入口是对账执行器按法人逐轮遍历，该表按正向登记制登记到 platform_core.unpoliced_table_registry，不建行级策略，本计划不再使用全局配置字典这一类名；该表不含 statement_sha256 与 signed_statement_ref 两列，理由见第 9.4.7 节；recon_runs 与 recon_discrepancies 带 legal_entity_id 并按基线第 3.8 节模板建策略；recon_runs 为仅追加表，recon_discrepancies 为可更新表并带 row_version；recon_discrepancies.recon_run_id 为同 schema 真实外键，两表的 accounting_period_id 为跨 schema 逻辑引用不建外键，理由是基线第 3.3 节的模块规则本身禁止跨 schema 外键，与目录归属无关。

本阶段的跨模块逻辑引用不建 CROSS_MODULE_LINK 校验项，依据为裁定 A-06 与总览 R14，这是首版的已知边界。ledger.vouchers 的 source_document_type 与 source_document_id 由过账入口在同一事务内与来源单据行一并写入，其存在性不依赖写入时查证；release_package_id 与 approval_ref 两类平台侧逻辑引用分别由配置发布入口与审批入口保证。9b 段自带并注册的四个校验项按第 9.4.7 节无一取 CROSS_MODULE_LINK。

公共列在下列各表中一律按基线第 4 节的顺序排列，即 id、legal_entity_id、security_level、data_scope_tags、row_version、created_at、created_by、updated_at、updated_by。仅追加表按基线同节去掉 row_version、updated_at、updated_by，改带 reverses_id。为节省篇幅，下表只列公共列之外的列，并在每表注明其归类。

#### 9.3.1 ledger.accounts

归类为档案类，另加 code 与 is_active、deactivated_at。

| 列 | 类型 | 可空 | 默认 | 约束与说明 |
|---|---|---|---|---|
| code | text | 否 | 无 | ck_accounts_code_len 长度 1 至 64；ck_accounts_code_charset 只允许数字 |
| name | text | 否 | 无 | ck_accounts_name_len 长度 1 至 200 |
| category | text | 否 | 无 | ck_accounts_category 取 ASSET、LIABILITY、EQUITY、PROFIT_LOSS |
| balance_direction | text | 否 | 无 | ck_accounts_balance_direction 取 DEBIT、CREDIT |
| account_level | smallint | 否 | 无 | ck_accounts_level 取 1 或 2 |
| parent_account_id | uuid | 是 | 无 | fk_accounts_accounts 指向本表 id，ON DELETE RESTRICT；ck_accounts_parent_presence 约束 account_level = 2 时非空、account_level = 1 时为空 |
| is_postable | boolean | 否 | true | 是否可直接记账 |
| is_active | boolean | 否 | true | 启用状态 |
| deactivated_at | timestamptz | 是 | 无 | 停用时间 |

索引：pk_accounts、ux_accounts_legal_entity_id_code、ix_accounts_legal_entity_id_created_at、ix_accounts_legal_entity_id_parent_account_id、ix_accounts_legal_entity_id_category。

RLS：按基线第 3.8 节模板生成 rls_accounts_le。以下各带法人列的表同此，不再逐表重复。

不设跨 schema 外键，本表被引用方均在 ledger schema 内，故 parent 与 voucher_lines 的引用建真实外键。

#### 9.3.2 ledger.accounting_periods

归类为业务表，另加 status。本表不是单据，不设 doc_no，属本阶段新增决定，理由见第 9.12 节。

| 列 | 类型 | 可空 | 默认 | 约束与说明 |
|---|---|---|---|---|
| period_code | text | 否 | 无 | 形如 202608，ck_accounting_periods_code_format 六位数字 |
| fiscal_year | smallint | 否 | 无 | ck_accounting_periods_fiscal_year 取 1900 至 9999 |
| period_no | smallint | 否 | 无 | ck_accounting_periods_period_no 取 1 至 12 |
| start_date | date | 否 | 无 | 自然月首日 |
| end_date | date | 否 | 无 | 自然月末日，ck_accounting_periods_date_order 约束 start_date 小于等于 end_date |
| status | text | 否 | OPEN | ck_accounting_periods_status 取 OPEN、CLOSED |
| is_fiscal_year_last | boolean | 否 | false | 是否年度末次期间 |
| closed_at | timestamptz | 是 | 无 | 关闭时间 |
| closed_by_close_request_id | uuid | 是 | 无 | 触发关闭的关账请求，fk_accounting_periods_period_close_requests 在第 10 号迁移中补建 |

索引：pk_accounting_periods、ux_accounting_periods_legal_entity_id_period_code、ix_accounting_periods_legal_entity_id_created_at、ix_accounting_periods_legal_entity_id_start_date。

#### 9.3.3 ledger.event_account_bindings

归类为业务表。

| 列 | 类型 | 可空 | 默认 | 约束与说明 |
|---|---|---|---|---|
| account_role | text | 否 | 无 | ck_event_account_bindings_role 取第 9.4.2 节固定的 17 个角色 |
| account_id | uuid | 否 | 无 | fk_event_account_bindings_accounts，ON DELETE RESTRICT |
| release_package_id | uuid | 是 | 无 | 该绑定由哪个配置发布包发布，逻辑引用 platform_meta.config_packages，不建跨 schema 外键 |

索引：pk_event_account_bindings、ux_event_account_bindings_legal_entity_id_account_role、ix_event_account_bindings_legal_entity_id_created_at。

#### 9.3.4 ledger.opening_balance_batches 与 ledger.opening_balance_batch_lines

头表归类为单据类，另加 doc_no 与 status，类型码 OBB。

头表列：accounting_period_id uuid 非空（建账首期）、source text 非空 ck 取 MANUAL、MIGRATION_BATCH、migration_batch_no text 可空 ck 长度不超过 64、total_debit_amount numeric(18,2) 非空、total_credit_amount numeric(18,2) 非空、status text 非空 ck 取 DRAFT、PENDING_APPROVAL、CONFIRMED、REJECTED、confirmed_at timestamptz 可空、approval_ref uuid 可空。约束 ck_opening_balance_batches_balanced 在 status = CONFIRMED 时不校验，借贷平衡在应用层确认时校验，理由是 CHECK 无法表达条件依赖状态的跨列关系而不引入触发器。

行表列：opening_balance_batch_id uuid 非空 fk 指向头表 ON DELETE RESTRICT、line_no smallint 非空、account_id uuid 非空 fk 指向 accounts、debit_amount numeric(18,2) 非空默认 0 ck 大于等于 0、credit_amount numeric(18,2) 非空默认 0 ck 大于等于 0、ck_opening_balance_batch_lines_one_side 约束两者至少一个为 0。

索引：ux_opening_balance_batches_legal_entity_id_doc_no、ix_opening_balance_batches_legal_entity_id_created_at、ux_opening_balance_batch_lines_batch_id_line_no、ux_opening_balance_batch_lines_batch_id_account_id、ix_opening_balance_batch_lines_legal_entity_id_created_at。

#### 9.3.5 ledger.vouchers

归类为仅追加表，另加 doc_no，类型码 GV，不带 row_version、updated_at、updated_by，带 reverses_id。

| 列 | 类型 | 可空 | 默认 | 约束与说明 |
|---|---|---|---|---|
| doc_no | text | 否 | 无 | 凭证号，按基线第 11.1 节格式 |
| accounting_period_id | uuid | 否 | 无 | fk_vouchers_accounting_periods，凭证落入哪个会计期间的唯一依据 |
| business_date | date | 否 | 无 | 原始业务日期，取该业务事件的记账日期 |
| deferred_from_period_id | uuid | 是 | 无 | 非空即表示该凭证发生过顺延，fk_vouchers_accounting_periods_deferred |
| source_kind | text | 否 | 无 | ck_vouchers_source_kind 取第 9.4.1 节固定的 17 个来源类型 |
| source_sequence_no | smallint | 否 | 1 | ck_vouchers_source_sequence 约束取值为 1，或 source_kind = YEAR_END_PL_CLOSING |
| source_document_type | text | 否 | 无 | 来源单据对象类型，跨模块逻辑引用 |
| source_document_id | uuid | 否 | 无 | 来源单据标识，跨模块逻辑引用，不建外键 |
| source_document_no | text | 否 | 无 | 来源单据编号，冗余存储以支持不回表检索 |
| source_event_id | uuid | 是 | 无 | 触发过账的业务事件标识 |
| total_debit_amount | numeric(18,2) | 否 | 无 | ck_vouchers_balanced 约束等于 total_credit_amount |
| total_credit_amount | numeric(18,2) | 否 | 无 | 同上 |
| line_count | smallint | 否 | 无 | ck_vouchers_line_count 大于等于 2 |
| reverses_id | uuid | 是 | 无 | 本凭证冲销的凭证，fk_vouchers_vouchers |

索引：pk_vouchers、ux_vouchers_legal_entity_id_doc_no、ix_vouchers_legal_entity_id_created_at、ux_vouchers_legal_entity_id_source_kind_source_document_id_source_sequence_no、ix_vouchers_legal_entity_id_accounting_period_id、ix_vouchers_legal_entity_id_business_date、ix_vouchers_legal_entity_id_source_document_id。

不可覆盖的强制：本迁移末尾执行 REVOKE UPDATE, DELETE ON ledger.vouchers FROM ep_app_rw，使已过账凭证在数据库权限层不可覆盖，不依赖应用自律。

#### 9.3.6 ledger.voucher_lines

归类为仅追加表。

| 列 | 类型 | 可空 | 默认 | 约束与说明 |
|---|---|---|---|---|
| voucher_id | uuid | 否 | 无 | fk_voucher_lines_vouchers，ON DELETE RESTRICT |
| line_no | smallint | 否 | 无 | 行号，自 1 起 |
| account_id | uuid | 否 | 无 | fk_voucher_lines_accounts，ON DELETE RESTRICT |
| account_role | text | 否 | 无 | ck_voucher_lines_role 取 17 个角色，记录本行由哪个科目角色映射而来 |
| direction | text | 否 | 无 | ck_voucher_lines_direction 取 DEBIT、CREDIT |
| amount | numeric(18,2) | 否 | 无 | ck_voucher_lines_amount 大于 0 |
| measure_key | text | 否 | 无 | 本行来自哪个计量项，供追溯与测试断言 |
| accounting_period_id | uuid | 否 | 无 | 与所属凭证相同，冗余以支持明细账与余额校验不回表 |
| business_date | date | 否 | 无 | 与所属凭证相同，冗余 |
| reverses_id | uuid | 是 | 无 | 本行冲销的分录行 |

索引：pk_voucher_lines、ux_voucher_lines_voucher_id_line_no、ix_voucher_lines_legal_entity_id_created_at、ix_voucher_lines_legal_entity_id_account_id_accounting_period_id、ix_voucher_lines_legal_entity_id_accounting_period_id_account_id、ix_voucher_lines_legal_entity_id_business_date。

同样执行 REVOKE UPDATE, DELETE ON ledger.voucher_lines FROM ep_app_rw。

#### 9.3.7 ledger.account_period_balances

归类为业务表。

| 列 | 类型 | 可空 | 默认 | 约束与说明 |
|---|---|---|---|---|
| account_id | uuid | 否 | 无 | fk_account_period_balances_accounts |
| accounting_period_id | uuid | 否 | 无 | fk_account_period_balances_accounting_periods |
| opening_balance_amount | numeric(18,2) | 否 | 0 | 期初余额，正为借方余额、负为贷方余额 |
| is_opening_fixed | boolean | 否 | false | 期初是否已固化，上一期间关闭或期初余额批次确认时置真 |
| period_debit_amount | numeric(18,2) | 否 | 0 | 本期借方发生额 |
| period_credit_amount | numeric(18,2) | 否 | 0 | 本期贷方发生额 |

索引：pk_account_period_balances、ux_account_period_balances_legal_entity_id_account_id_accounting_period_id、ix_account_period_balances_legal_entity_id_created_at、ix_account_period_balances_legal_entity_id_accounting_period_id。

#### 9.3.8 ledger.close_serialization_slots

归类为业务表，每法人一行，用作同一法人同一时点只允许一个已受理未结束关账请求的串行化点。

列：active_close_request_id uuid 可空。索引：pk_close_serialization_slots、ux_close_serialization_slots_legal_entity_id、ix_close_serialization_slots_legal_entity_id_created_at。

#### 9.3.9 ledger.period_close_requests

归类为单据类，类型码 PCR。

| 列 | 类型 | 可空 | 默认 | 约束与说明 |
|---|---|---|---|---|
| doc_no | text | 否 | 无 | 关账请求编号 |
| accounting_period_id | uuid | 否 | 无 | fk_period_close_requests_accounting_periods |
| status | text | 否 | 无 | ck 取第 9.4.5 节的 9 个状态 |
| reauth_ref | uuid | 是 | 无 | 重新认证凭证引用 |
| approval_ref | uuid | 是 | 无 | 审批实例引用，逻辑引用 platform_flow |
| approved_by | uuid | 是 | 无 | 审批人，ck_period_close_requests_no_self_approval 约束不等于 created_by |
| accepted_at | timestamptz | 是 | 无 | 受理时点 |
| inflight_xids | text[] | 否 | '{}' | 受理时点登记的在途写事务标识集合 |
| inflight_wait_completed_at | timestamptz | 是 | 无 | 在途写事务等待结束时点 |
| snapshot_id | text | 是 | 无 | 导出快照标识 |
| snapshot_established_at | timestamptz | 是 | 无 | 快照建立时点 |
| conclusion | text | 是 | 无 | ck 取 PASSED、DISCREPANCY、INCOMPLETE、CANCELLED |
| concluded_at | timestamptz | 是 | 无 | 结论产生时点 |
| refusal_reasons | jsonb | 是 | 无 | 受理被拒时逐项载明未满足的前提项与其当前取值 |
| completed_batch_count | integer | 否 | 0 | 已完成批次数 |
| termination_cause | text | 是 | 无 | ck 取 BATCH_TIMEOUT、RESOURCE_LIMIT、PROCESS_EXIT、CONNECTION_RECYCLED、SNAPSHOT_INVALID |

索引：pk_period_close_requests、ux_period_close_requests_legal_entity_id_doc_no、ix_period_close_requests_legal_entity_id_created_at、ix_period_close_requests_legal_entity_id_accounting_period_id。

#### 9.3.10 ledger.year_end_closings

归类为单据类，类型码 YEC。

列：doc_no、fiscal_year smallint 非空、accounting_period_id uuid 非空 fk、status text 非空 ck 取 PENDING_APPROVAL、APPROVED、EXECUTED、REJECTED、FAILED、sequence_no smallint 非空（同一年度内第几次结转）、reauth_ref uuid 可空、approval_ref uuid 可空、approved_by uuid 可空 ck 不等于 created_by、pl_carry_voucher_id uuid 可空 fk 指向 vouchers、retained_earnings_voucher_id uuid 可空 fk 指向 vouchers、executed_at timestamptz 可空。

索引：pk_year_end_closings、ux_year_end_closings_legal_entity_id_doc_no、ux_year_end_closings_legal_entity_id_fiscal_year_sequence_no、ix_year_end_closings_legal_entity_id_created_at。

#### 9.3.11 ledger.posting_trigger_event_types

本表不带 legal_entity_id。准入判据是其行集合与法人无关：它只登记本版本会产生凭证的事件类型名，取值由制品决定，在本部署内对全部法人相同。隔离承接入口是第 9.3.12 节的 ledger.v_pending_posting_backlog，该视图的取数一律受调用方的 app.legal_entity_id 约束。本表按正向登记制登记到 platform_core.unpoliced_table_registry，不建行级策略。

列：id uuid 主键、event_type text 非空唯一、created_at、created_by。原有的 ledger_event_kind 与 registered_by_module 两列删除，理由是来源类型已由 JOURNAL_MAP 的键唯一确定，本表只需承载会产生凭证的事件类型集合，供第 9.3.12 节的视图连接。

索引：pk_posting_trigger_event_types、ux_posting_trigger_event_types_event_type。
本表不设任何运行期断言接口。原 PostingTriggerRegistry::assert_registered 与错误码 LEDGER.POSTING_TRIGGER_EVENT_TYPE.REGISTRY_MISMATCH 一并删除，阶段 6、7、10 三处启动自检追加项与其退出码 78 路径随之取消。理由是种子行与编译期常量随同一制品发布，不存在分叉的可能，该断言只是给八个进程多加一条整机拒绝启动的路径，而这台服务器没有备节点。登记表一致性的承接方定死为两条，本阶段不另设第三条：一是 xtask configdoc 从 docs/event-catalog.md 的 produces_voucher 列生成第 14 号迁移并在 CI 中与仓库中的文件逐字比对，不一致即构建失败；二是阶段 3b 的 event-catalog-consistent 自检项，该项不通过时停止派发未登记的事件类型。关账受理前提仍为第 9.4.5 节的两条，不因本表新增第三条前提。按 A-21，登记表与全部 13 行登记行归本阶段 9a 段，业务阶段不再追加任何回填迁移。

唯一约束落在 event_type 上，一行即一个会产生凭证的事件类型。第 14 号种子迁移一次写全 13 行，清单见下表。原先按 ledger_event_kind 各写一行、并为 YEAR_END_PL_CLOSING 保留一行空 event_type 的写法删除：年度损益结转由人在应用内发起，不经事件驱动，本表不登记它；阶段 8 的库存事件不产生凭证，本表也没有它的行。下表的阶段列表示该事件由哪个阶段产生，不表示由哪个阶段写登记行；各业务阶段一律不新增回填迁移，也不在启动时比对。

| 阶段 | event_type |
|---|---|
| 6 | sales.delivery.confirmed.v1 |
| 6 | sales.sales_return.registered.v1 |
| 7 | procure.goods_receipt.posted.v1 |
| 7 | procure.purchase_return.posted.v1 |
| 10 | invoice.sales_invoice.issued.v1 |
| 10 | invoice.purchase_invoice.registered.v1 |
| 10 | invoice.sales_invoice.reversed.v1 |
| 10 | invoice.purchase_invoice.reversed.v1 |
| 10 | finance.receipt.registered.v1 |
| 10 | finance.payment.registered.v1 |
| 10 | finance.refund.registered.v1 |
| 10 | finance.cash_document.reversed.v1 |
| 10 | finance.overbilling_entry.settled.v1 |


#### 9.3.12 两个视图

ledger.v_account_period_balances：按法人、科目、会计期间输出期初余额、本期借方发生额、本期贷方发生额、期末余额。期初取数规则为 is_opening_fixed 为真时取 opening_balance_amount，为假时取该科目最近一个已固化期间的期初加上该期间起至目标期间前一期的发生额净额。该视图对 ledger.accounts 与 ledger.accounting_periods 做交叉连接后左连 ledger.account_period_balances，使无发生额的启用科目在科目余额表中仍出现。视图不使用物化视图，首版不使用函数索引与部分索引。该视图同时是 A-18 的受治理数据集，dataset code 为 ledger_account_period_balances，grain 为 SNAPSHOT，输出列另含 legal_entity_id、security_level 与 data_scope_tags 三列，并按第 15 号迁移 GRANT SELECT 给 ep_analyst_ro，列名与类型签名与 reporting.dataset_fields 的登记一致，由阶段 11 的 reporting-dataset-signature-matched 自检项校验。

ledger.v_pending_posting_backlog：按法人与会计期间输出待消费过账条目数与未修复死信条数。受理前提二的判定语句按 C-28 在阶段 4、9、10 三处逐字一致，即：该法人该期间内，platform_msg.outbox_events 中 status 属于 PENDING 或 DISPATCHING、posting_date 落在该期间起止之间、且 event_type 命中 ledger.posting_trigger_event_types 的条目数为零，且 platform_msg.dead_letters 中 state 属于 OPEN 或 REPAIRING、同样命中该注册表的条数为零。posting_date 为空的平台事件一律不计入，理由是它们不产生凭证。该视图是规格第 10.2 章受理前提二的可枚举依据。全部凭证一律与业务事件同事务生成，Outbox 只承载派生、通知、检索与报表数据集，本阶段不存在异步过账路径。

### 9.4 领域模型与关键算法

#### 9.4.1 凭证来源类型

ep-domain-ledger 定义 VoucherSourceKind 枚举，17 个取值。取值不按规格第 5.2 章事件-分录表的行数取，而按分录集合取，判据固定为一句：两种情形的科目角色集合不相交，或同一角色方向相反，才拆成两个取值；一种情形只是另一种少几条腿的，不拆，由计量项缺省承担。据此十类事件展开为 16 个业务来源类型，加上期末处理动作共 17 个。

DELIVERY_CONFIRMED、SALES_INVOICE_ISSUED、SALES_INVOICE_REVERSED、PURCHASE_INVOICE_REVERSED、RECEIPT_REGISTERED、PAYMENT_REGISTERED、CUSTOMER_REFUND、SUPPLIER_REFUND、PURCHASE_RECEIPT、PURCHASE_INVOICE_INVENTORY、PURCHASE_INVOICE_DIRECT_EXPENSE、SALES_RETURN、PURCHASE_RETURN_UNBILLED、PURCHASE_RETURN_BILLED_INVENTORY、PURCHASE_RETURN_BILLED_DIRECT_EXPENSE、OVERBILLING_WRITTEN_OFF、YEAR_END_PL_CLOSING。

与原 11 个取值的差别有四处，一律是拆分或补齐，不改任何借贷与取价。一是拆：原 INVOICE_REVERSED 拆为销项与进项两个，原 REFUND_REGISTERED 拆为客户退款与供应商返款两个，原 PURCHASE_INVOICE 拆为物料类与直接费用类两个，原 PURCHASE_RETURN 拆为发票未登记、发票已登记存货口径、发票已登记直接费用口径三个。二是不拆：DELIVERY_CONFIRMED、SALES_RETURN 与 PURCHASE_RECEIPT 各保持一个取值，直运与非直运、路径一的反向匹配腿与暂估腿一律由计量项缺省表达，依据是第 9.4.3 节第二步已定金额为零的计量项不生成分录行。三是补：新增 OVERBILLING_WRITTEN_OFF 承接超量开票的路径三，它此前在 11 个取值里无处安放却独立产凭证，补上它是修复既有缺口而不是扩容；超量开票的路径一与路径二不设取值，改为各自宿主来源类型下的计量项 overbilling_settlement_amount。四是减：资金单据冲正不设取值，按第 9.5.9 节的 post_reversal 处理。

原写的任何阶段不得新增取值一句删除，替换为可执行约束：新增取值的提交必须同批包含 JOURNAL_MAP 的对应行、ck_vouchers_source_kind 的迁移，以及覆盖该取值全部计量项组合的借贷平衡属性测试，三者缺一 CI 失败。理由是取值落在 text 列的 CHECK 上，新增本就必须走迁移，迁移即天然的强制点；封条挡不住演进，只会让首版排除的调拨、盘点、领用与费用报销日后绕道手工凭证。手工凭证与更正凭证入口按 U-H-07 与 U-H-08 待决，本阶段仍不实现，见第 9.12 节的阻塞判定。

拆分后每张来源单据必须恒定映射到唯一取值。采购退货的三个取值由采购发票是否已登记这一可变状态派生，重放时重算可能得到不同取值，从而绕过 ux_vouchers 的四列唯一键造成重复过账，因此采购退货单上须落一列 resolved_source_kind，首次过账时固化，重放读该列不重算，该列归阶段 7。这是拆分带来的唯一新增负担，必须一并落地。

#### 9.4.2 科目角色

ep-contract-ledger 定义 AccountRole 枚举，17 个取值，是事件科目对应关系配置的唯一可配置面。角色本身由平台固定并随版本冻结，客户只能把角色绑定到本法人科目表中的具体科目。

ACCOUNTS_RECEIVABLE_UNBILLED、ACCOUNTS_RECEIVABLE、ADVANCE_FROM_CUSTOMER、MAIN_OPERATING_REVENUE、MAIN_OPERATING_COST、INVENTORY、ACCOUNTS_PAYABLE、ACCOUNTS_PAYABLE_ACCRUED、ADVANCE_TO_SUPPLIER、OVERBILLING_SUSPENSE、TAX_PAYABLE_OUTPUT、TAX_PAYABLE_INPUT、BANK_DEPOSIT、CASH_ON_HAND、DIRECT_EXPENSE_COST、PROFIT_THIS_YEAR、RETAINED_EARNINGS_UNDISTRIBUTED。

各角色对应规格第 5.2 章事件-分录表中出现的科目名称，一一对照关系写入 docs/data-dictionary/ledger.md，本计划不复述。

#### 9.4.3 计量项与事件到分录的映射表

本阶段采取的分层是：金额的计算归产生该业务事件的模块，借贷方向与科目角色归 ledger。理由是移动加权平均单价、暂估回冲金额、价差拆分与退货回冲取价一律由规格第 5.2 章的规则块在库存与采购侧维护，且必须与数量账、金额账同源同事务写入，ledger 无法也不应重算；而规格第 5.2 章要求内置固定的业务事件到分录映射，该映射的内容正是方向与科目角色。该分层是本阶段新增决定。按 C-13，取价一律归阶段 8，本阶段不自行取价，ledger 侧不提供任何取价方法，只做分录映射与借贷平衡；出入库取价与价差拆分的入口分别是 ep-contract-inventory 的 InventoryPostingPort 与 InventoryVariancePort，由调用方在同一事务内先取得金额再作为计量项传入。

ep-contract-ledger 定义 PostingInput：

- source_kind: VoucherSourceKind
- posting_date: NaiveDate
- source_document: SourceDocumentRef，含 object_type、id、doc_no
- source_event_id: Option<Id>
- measures: Vec<(MeasureKey, Money)>，MeasureKey 为固定枚举，取值与事件-分录表各腿的金额语义一一对应，如 revenue_amount、unbilled_receivable_amount、cogs_amount、inventory_release_amount、output_tax_amount、input_tax_amount、accrual_reversal_amount、price_variance_in_stock_amount、price_variance_released_amount、settlement_amount、advance_amount、overbilling_amount、overbilling_settlement_amount、cash_amount 等；完整清单登记在 docs/data-dictionary/ledger.md，随本阶段冻结

原有的 branch 与 reverses_voucher_id 两个字段删除。branch 是第二根全局共用的轴，与来源类型相乘得到一百三十二格而其中合法的约十六格，哪些格合法这件事不进类型只进映射表的行，靠运行期校验兜底；来源类型按分录集合拆开之后它没有剩余职责。reverses_voucher_id 由第 9.5.9 节的 post_reversal 承担；销项与进项红字冲销各走自己的来源类型经映射生成，其追溯链落在单据侧，即红字发票指向原发票，凭证侧不再第二次表达，ledger.vouchers.reverses_id 与 ledger.voucher_lines.reverses_id 两列此后只由 post_reversal 写入。

ep-domain-ledger 定义编译期常量映射表 rule::journal_map::JOURNAL_MAP，元素为四元组 (VoucherSourceKind, MeasureKey, AccountRole, Direction)，键为前两项、值为后两项，是一维查表。表的内容一律按规格第 5.2 章事件-分录表填写，本计划不复述其借贷。

映射算法固定为四步。

第一步，按 source_kind 取出该来源类型下的全部四元组，若 measures 中出现该来源类型未登记的 MeasureKey，或该来源类型必填的 MeasureKey 缺失，返回 VALIDATION 与 LEDGER.POSTING.MEASURE_INVALID。原 LEDGER.POSTING.MEASURE_NOT_APPLICABLE 与 LEDGER.POSTING.MEASURE_MISSING 两个错误码合并为该码，语义为该来源类型未登记此计量项或必填计量项缺失。

第二步，符号归一。计量项金额允许为负，正值按表中方向入账，负值取绝对值按相反方向入账。该规则使 numeric(18,2) 的 amount 列恒为正、方向恒为二值，同时不改变任何净额。金额为零的计量项不生成分录行。

第三步，把 AccountRole 经该法人的 event_account_bindings 解析为 account_id；角色未绑定返回 BUSINESS_CONFLICT 与 LEDGER.EVENT_ACCOUNT_BINDING.ROLE_UNBOUND，绑定到已停用科目返回 LEDGER.EVENT_ACCOUNT_BINDING.ACCOUNT_INACTIVE，绑定到存在下级科目的一级科目返回 LEDGER.ACCOUNT.NOT_POSTABLE。同一角色在一次映射中多次出现时不合并行，逐条生成，便于按 measure_key 追溯。

第四步，断言借方合计等于贷方合计，行数大于等于 2，不成立返回 BUSINESS_CONFLICT 与 LEDGER.VOUCHER.UNBALANCED。该断言与数据库上的 ck_vouchers_balanced 构成双重保证。

前向说明：阶段 11 将在凭证行生成之后、同一 &mut dyn Tx 内追加成本与收入捕获的调用点，见阶段 11 计划第 4.2 节。该调用点按同批交付处理，与阶段 11 的实现同批加入代码，此前不存在于代码中；本阶段不预留任何未接线端口，不注入任何空实现，映射算法本身不变。

边界条件：单张凭证行数上限由配置 ledger.posting.max_lines_per_voucher 约束，超出返回 BUSINESS_CONFLICT 与 LEDGER.VOUCHER.LINE_LIMIT_EXCEEDED；全部计量项金额为零时不生成凭证，PostingPort 返回 Skipped，调用方按无凭证处理，理由是零金额凭证既无账务意义又会污染试算平衡的凭证张数统计。

#### 9.4.4 会计期间与顺延入账

会计期间状态机只有两个状态，与规格第 5.2 章一致。

| 当前 | 目标 | 触发 | 守卫 |
|---|---|---|---|
| 尚未建立 | OPEN | 首次过账时该法人尚无任何期间，按记账日期所属自然月建立首个期间 | 该法人 ledger.accounting_periods 无任何行，且在同一业务事务内完成 |
| 尚未建立 | OPEN | 定时任务提前建立下一自然月期间 | 该法人不存在同 period_code 的期间，且该法人已存在至少一个期间时才计算下一自然月 |
| 尚未建立 | OPEN | 顺延取目标时不存在可入账期间，建立最晚期间之后紧邻的自然月期间 | 同上，且在同一业务事务内完成 |
| OPEN | CLOSED | 关账请求的关账前强制校验通过 | 该期间存在一个状态为 VALIDATING 且结论为 PASSED 的关账请求 |
| CLOSED | 无 | 首版不做反结账 | 无入口 |

可入账期间是派生条件，不是第三种状态：status = OPEN 且不存在 accounting_period_id 指向它、status 属于 ACCEPTED 或 VALIDATING 的关账请求。

期间归属解析算法 resolve_accounting_period(le, posting_date)，在业务事务内执行，输出 ResolvedPeriod。该类型定义在 ep-contract-ledger，字段私有、无公开构造函数，只能由 resolve 返回，含 accounting_period_id、accounting_period_seq 与 deferred_from_period_id 三项；accounting_period_seq 由该期间的 fiscal_year 与 period_no 导出为该法人内的单调序号，不新增列。原先由各调用方自行计算并传入 accounting_period_seq 的做法取消，理由是十六项对账一律按 accounting_period_id 比金额，seq 写错时对账全绿，而收发存汇总与期末库存价值表按 seq 取数会静默错期，且其承载表是仅追加表不可更正。

第一步，校验 posting_date 不晚于服务器自然日，取值为 (now() AT TIME ZONE 'Asia/Shanghai')::date，禁止使用 current_date。晚于则返回 VALIDATION 与 LEDGER.ACCOUNTING_PERIOD.POSTING_DATE_IN_FUTURE，定位到该字段。

第二步，取 posting_date 所属期间 P0，即 start_date 小于等于 posting_date 且 end_date 大于等于 posting_date 的那一行。P0 不存在时分两支判定。其一是零期间分支：该法人 ledger.accounting_periods 无任何行时，按 posting_date 所属自然月建立该期间并置 OPEN，period_code、fiscal_year、period_no、start_date、end_date 与 is_fiscal_year_last 按同一自然月口径导出，建立动作在同一业务事务内完成，并发安全沿用第五步已有写法，即唯一约束 ux_accounting_periods_legal_entity_id_period_code 加 INSERT ... ON CONFLICT DO NOTHING 再重读，建立后以该期间作为 P0 继续第三步；这是本阶段建立首个会计期间的唯一手段，属 9a 段交付并落在第 9.0.2 节的 T0 切片内，不经任何端点也不经测试夹具。其二是该法人已有期间的情形：返回 VALIDATION 与 LEDGER.ACCOUNTING_PERIOD.BEFORE_FIRST_PERIOD。后一支是本阶段的假设：规格保证记账日期不晚于登记时点自然日因此所属期间必已建立，但未覆盖该法人已有期间而记账日期早于其最早期间起始日的补记，本阶段按输入校验错误拒绝，理由是建账之前不存在该法人的账簿，允许写入会使期初余额的取数起点失去意义。

第三步，P0 为可入账期间则返回 (P0, null)。

第四步，否则取该法人当前最早的可入账期间 P1，按 start_date 升序第一条。存在则返回 (P1, P0)。

第五步，P1 不存在时，按该法人最晚期间之后紧邻的自然月建立新期间并置为 OPEN，返回 (新期间, P0)。并发安全由 ux_accounting_periods_legal_entity_id_period_code 加 INSERT ... ON CONFLICT DO NOTHING 再重读保证，不使用应用级锁。

不变量：因期间由早到晚顺序关账，且记账日期不得晚于登记时点自然日，顺延目标一律晚于 P0，顺延必然收敛，凭证不因期间归属被拒绝。该不变量作为领域属性测试的断言之一。

顺延的连带范围不再靠纪律保证。resolve 在同一 &mut dyn Tx 内记忆化，第二次调用返回同一个 ResolvedPeriod，一个事务里解析两次得到两个期间这条唯一会分叉的路径因此被消灭，原写的必须使用同一次 resolve 的返回值一句随之删除。ep-contract-ledger 的 AccountingPeriodResolver 仍是这一连带的唯一入口，各子账模块不得自行判定期间；跨模块边界按值传 ResolvedPeriod 的三项，由 xtask archcheck 断言 crates/application/ledger 与 ep-adapter-db-pg 的 ledger 仓储之外，任何模块的仓储写入 accounting_period_id 与 accounting_period_seq 的取值只能来自命令 DTO 的同名字段，不得来自 posting_date、Clock 或本地推导。子账条目的期间是否等于其来源凭证的期间，不另立校验项：CROSS_MODULE_LINK 一类已按 A-06 整体撤销，阶段 7 的六项一律取 INVARIANT、阶段 10 不注册任何 ReconCheck，该谓词的显式承接方只有阶段 11 的 COSTING_COST_VS_LEDGER 与 COSTING_REVENUE_VS_LEDGER 两项，已并入其判据；其余子账模块由本段首句所述 resolve 在同一 &mut dyn Tx 内的记忆化在结构上保证，不新增第十六项校验项。

顺延不改变任何取价与借贷，由第 9.4.3 节的映射算法保证：映射只读 source_kind 与 measures，不读期间。

#### 9.4.5 关账请求状态机

状态取值 9 个：PENDING_APPROVAL、APPROVAL_REJECTED、ACCEPTANCE_REFUSED、ACCEPTED、VALIDATING、PASSED、FAILED_DISCREPANCY、FAILED_INCOMPLETE、CANCELLED。

| 当前 | 目标 | 触发 | 守卫条件 |
|---|---|---|---|
| 无 | PENDING_APPROVAL | 财务会计发起 | 重新认证凭证有效且绑定本次待签内容摘要；期间存在且属该法人；调用方具备 ledger.period_close.request |
| PENDING_APPROVAL | APPROVAL_REJECTED | 审批驳回 | 审批人不等于发起人 |
| PENDING_APPROVAL | ACCEPTANCE_REFUSED | 审批通过后受理前提任一不成立 | 见下文两项前提 |
| PENDING_APPROVAL | ACCEPTED | 审批通过且两项前提全部成立 | slot 行锁内判定并置位 |
| ACCEPTED | VALIDATING | 在途写事务等待结束且快照建立成功 | inflight_xids 全部完成 |
| ACCEPTED 或 VALIDATING | CANCELLED | 操作者在尚未产生校验结论前主动取消 | 重新认证与审批通过；conclusion 为空 |
| PENDING_APPROVAL | CANCELLED | 同上 | 同上 |
| VALIDATING | PASSED | 全部校验项通过 | 同一快照内 |
| VALIDATING | FAILED_DISCREPANCY | 任一校验项差额非零 | 同上 |
| VALIDATING | FAILED_INCOMPLETE | 五类终止成因之一 | 同上 |

受理前提逐项判定，全部成立才受理。

前提一：该期间 status = OPEN；该期间是该法人 start_date 最小的 OPEN 期间；该法人的 close_serialization_slots.active_close_request_id 为空。

前提二：该法人该期间内，platform_msg.outbox_events 中 status 属于 PENDING 或 DISPATCHING、posting_date 落在该期间起止之间、且 event_type 命中 ledger.posting_trigger_event_types 的条目数为零，且 platform_msg.dead_letters 中 state 属于 OPEN 或 REPAIRING、同样命中该注册表的条数为零。posting_date 为空的平台事件一律不计入。该判定的可枚举依据为 ledger.v_pending_posting_backlog，两侧不为零时分别按 LEDGER.PERIOD_CLOSE_REQUEST.PENDING_POSTING_BACKLOG 与 LEDGER.PERIOD_CLOSE_REQUEST.UNREPAIRED_DEAD_LETTERS 载明。

任一不成立时置 ACCEPTANCE_REFUSED，把未满足项与其当前取值写入 refusal_reasons，经 ep-platform-recon 与 ep-platform-obs 的端口生成关账受理被拒事项并记入运维中心，期间状态不变，其过账、查询与报表不受影响，发起次数不设上限。同一法人同一期间连续两次受理被拒时按规格第 15.3 章告警，并按 A-26 经 ep-platform-obs 的 DegradationLedger::open 登记暴露窗口，至该期间完成关账时经 DegradationLedger::close 关闭。本阶段只调用该台账，不自建第二套窗口表；DegradationKind 的取值由阶段 2 定义并由后续阶段扩展，本阶段不复述其取值数。

年度末次期间的损益归零不在受理前判定，由关账前强制校验在同一快照上判定。

#### 9.4.6 受理与在途写事务等待的次序

这是本阶段最需要严密论证的一处。次序固定为四步，任一步顺序颠倒即产生正确性缺口。

第一步，受理事务 T1：对 close_serialization_slots 该法人行执行 SELECT ... FOR UPDATE，判定两项前提，置请求为 ACCEPTED、accepted_at、slot.active_close_request_id，写审计，写 ledger.period_close.accepted.v1 的 Outbox 条目，提交。

第二步，在途集合登记事务 T2：T1 提交之后立即开启，执行 select pg_snapshot_xip(pg_current_snapshot())，把结果去掉自身写入 period_close_requests.inflight_xids，提交。

必须是先提交 T1 再取快照，理由如下。任何一次期间归属解析都在某条语句的快照上读 accounting_periods 与 period_close_requests，隔离级别为 READ COMMITTED，每条语句取新快照。若该解析发生在 T1 提交之后，它一定看到 ACCEPTED，因此该期间不再是可入账期间，其凭证顺延到后续期间。若该解析发生在 T1 提交之前，其所在事务在 T2 取快照时要么仍在途，从而落入 inflight_xids 并被第三步等待覆盖；要么已提交，从而其凭证在第四步的快照中可见。三种情形穷尽，因此受理之后不再产生落入该期间的新凭证，且快照覆盖该法人该期间的全部凭证。

第三步，等待：按配置的轮询间隔重复取 pg_current_snapshot()，对 inflight_xids 中每个 xid 判定 xid 小于 pg_snapshot_xmax(current) 且不在 pg_snapshot_xip(current) 中，全部成立即等待结束，写 inflight_wait_completed_at。该判定不依赖 pg_stat_activity，因此不需要给运行期账号授予 pg_read_all_stats 或 pg_monitor，不放大运行期账号权限。该集合在 T2 一次取定，此后只减不增，每笔事务终将提交或回滚，因此等待必然结束，不设时限、不自动解除。等待期间不冻结任何写入。等待超过 ledger.close.inflight_wait_warn_seconds 时只告警不终止。

第四步，快照建立：在 job-worker 池上开启一个 REPEATABLE READ 只读事务，执行 SELECT pg_export_snapshot() 取得 snapshot_id 并保持该事务打开；各批工作连接在自身事务开始时执行 SET TRANSACTION SNAPSHOT '<snapshot_id>'。在另一条连接上把请求置为 VALIDATING 并写 snapshot_established_at，不在快照事务内写，理由是快照事务须保持只读且长期打开。按 A-01 与 C-03，只读快照事务的唯一入口是 ep-foundation 的 UnitOfWork::snapshot_transact，其向执行体传入的 SnapshotCtx 的 snapshot_id() 即 pg_export_snapshot 的返回值、taken_at() 即 snapshot_established_at，逐批传递的就是该 SnapshotCtx。

#### 9.4.7 关账前强制校验的编排

ep-platform-recon 的本体按 A-06 由本阶段 9a 段提供：crate、platform_core 下的三张表、ReconCheck 与 ReconRegistry 与 ReconExecutor 三个契约、BatchWindow 与 ReconRunOutcome、快照传递、单批时限、单查询内存与临时空间上限、差异事项与校验未完成事项的模型，以及 job-worker 内的每日对账调度，全部在本阶段落地。执行器按基线第 3.8 节逐法人遍历，法人清单取 ep-platform-tenancy 的 LegalEntityDirectory::list_active，每轮只在单一法人上设置 app.legal_entity_id，快照经 UnitOfWork::snapshot_transact 导出并以 SnapshotCtx 逐批传递。本阶段自带的四类校验项各实现一个 ReconCheck 并在 job-worker 的 wiring 中经 ReconRegistry::register 注册，实现与注册同属 9b 段。规格第 7.7 章所要的这条路径不构成越权通道，其保证改为静态封闭性，不再用签名语句集：ReconExecutor 只按 ReconRegistry 中已注册的 ReconCheck 实现分发，不接受任何语句文本入参，校验语句一律是各 ep-app-* crate 内的编译期常量，完整性由阶段 1 的制品签名链与客户侧验签承担，并由 archcheck 断言 ep-platform-recon 与各 ReconCheck 实现体内不出现字符串拼接 SQL 与动态语句执行入口。platform_core.recon_check_definitions 的 statement_sha256 与 signed_statement_ref 两列删除，每次运行改写制品版本号与制品签名摘要到 recon_runs 与审计事件，外部审计问某次关账跑的是哪一版校验由这两项唯一回答。唯一保留为硬约束的是输出边界：recon_discrepancies.subject_ref 的允许键固定为勾稽项标识、法人、会计期间、凭证号、仓库、物料、批次、科目、单据编号与账户内部标识十项，写入时校验，出现白名单以外的键直接拒绝并按规格第 15.3 章告警。blocks_period_close 返回真的校验项即 is_blocking_period_close 为真的登记项，构成关账前强制校验的范围。

本阶段自带并注册的校验项四类。

一是会计借贷平衡：逐张凭证核对 total_debit_amount 等于 total_credit_amount 且等于其分录行按方向的合计；按该法人该期间核对借贷合计相等。差额非零生成勾稽类差异事项，载明勾稽项、法人、会计期间、子账侧金额、总账侧金额与差额。

二是年度末次期间损益归零：只在 is_fiscal_year_last 为真的期间执行，在同一快照上核对该期间 category = PROFIT_LOSS 的科目期末余额合计为零。该项差异事项载明法人、会计期间与该期间损益类科目余额合计，不载明子账侧金额，与勾稽类差异事项区分。年中期间不设该要求。

三是总账侧余额提供者：ep-contract-ledger 暴露 TotalAccountBalanceProvider trait，按 (法人, 会计期间, AccountRole) 返回该科目在快照上的余额。子账与总账勾稽的比较由本阶段的 ReconExecutor 驱动，子账侧一律经 ep-contract-finance 的 ReconciliationItemQuery 按法人与会计期间取十项勾稽的子账侧合计，结构为 ReconciliationItemView，该 trait 由阶段 10 定义；阶段 8 在 ep-contract-inventory 定义的 StockValueSubledgerBalancePort 与阶段 7 在 ep-contract-procure 定义的 GrniSubledgerBalancePort 是阶段 10 内部组装该结果的手段，本阶段的 ReconCheck 不直接调用它们。十项中的存货与已收货未收票两项，其子账侧实现体分别由阶段 8 的 InventorySubledgerBalanceQuery 与阶段 7 的 GrniSubledgerBalanceQuery 各自在本模块 contract 的端口上实现，阶段 10 只注入，其余八项取自阶段 10 自有表、不经这两个端口。本阶段不定义总账侧接口之外的任何东西。

四是科目余额一致性：核对 account_period_balances 的本期发生额与按 voucher_lines 在同一期间的聚合相等，期初已固化的核对与上一期间期末相等。该项是本阶段引入增量余额表所必须的自检，属本阶段新增决定。

校验未完成一律按未通过处理：单批执行时限触发终止、单查询内存或临时空间上限触发终止、执行进程异常退出、连接被回收、快照失效五类之一发生时，置 FAILED_INCOMPLETE，写 termination_cause 与 completed_batch_count，生成校验未完成事项，按规格第 15.3 章即时告警，并按 A-26 经 ep-platform-obs 的 DegradationLedger::open 登记降级与暴露窗口、成因解除后经 close 关闭，该次关账请求结束，期间保持打开。不得按通过处理，也不得以未生成勾稽类差异事项为由放行。

#### 9.4.8 年度损益结转

前提：该期间 is_fiscal_year_last 为真；该期间为可入账期间；发起人具备 ledger.year_end_closing.request；按规格第 12.1 章财务过账一类完成重新认证；按第 12.2 章完成审批且申请人不可自审。该期间存在已受理未结束的关账请求时不执行，返回 BUSINESS_CONFLICT 与 LEDGER.YEAR_END_CLOSING.PERIOD_NOT_POSTABLE。

算法五步。第一步，取执行时点该法人该期间全部 category = PROFIT_LOSS 且期末余额非零的科目。第二步，生成第一张结转凭证，source_kind = YEAR_END_PL_CLOSING、source_sequence_no = 1，把上述余额结转至 PROFIT_THIS_YEAR 角色所绑定的科目，方向与科目余额相反，净额入 PROFIT_THIS_YEAR。第三步，生成第二张结转凭证，source_sequence_no = 2，把 PROFIT_THIS_YEAR 的余额结转至 RETAINED_EARNINGS_UNDISTRIBUTED。两张凭证的科目路径与分录一律按规格第 5.2 章总账功能与期末处理块，本计划不复述。第四步，两张凭证的记账日期一律取该期间 end_date，按第 9.4.4 节归属该期间，因该期间是可入账期间故不发生顺延。第五步，写 year_end_closings 的 executed_at 与两个凭证引用。

可重复执行：每次执行创建一行新的 year_end_closings，sequence_no 递增，因此 ux_vouchers 的四列唯一键不冲突。本次结转之后又有凭证落入该期间时可再执行一次。

边界条件：损益类科目全部余额为零时不生成凭证，year_end_closings 置为 EXECUTED 且两个凭证引用为空，理由是空凭证无账务意义且会使借贷平衡校验的凭证张数统计失真。该期间关账之后到达、记账日期属于该年度的业务事件按顺延记入下一年度期间，其损益进入下一年度的本年利润，首版不做追溯重述。

会计恒等取数：结转前按资产等于负债加所有者权益加本期损益取数，结转后按资产等于负债加所有者权益取数，两次取数均以会计期间字段划分的凭证集合为范围。该项不属规格第 10.2 章关账前强制校验的范围，由第 9.5 节的只读端点承载。

#### 9.4.9 科目余额的维护

增量维护，与凭证写入同事务。对每条分录行按 (legal_entity_id, account_id, accounting_period_id) 执行 INSERT ... ON CONFLICT DO UPDATE，SET period_debit_amount = period_debit_amount + $1、period_credit_amount = period_credit_amount + $2、updated_at = now()、updated_by = $u、row_version = row_version + 1。不使用乐观锁比较，理由与偏离登记见第 9.12 节。同一事务内按 account_id 升序更新，避免与并发过账事务形成死锁循环。

期初固化：期间关闭的同一事务内，把该期间各科目的期末余额写入下一期间行的 opening_balance_amount 并置 is_opening_fixed 为真；下一期间行不存在时创建。因期间由早到晚顺序关账、已关闭期间不再接受凭证写入，固化一次即不再被推翻。未固化期间的期初由 v_account_period_balances 按第 9.3.12 节的规则实时递推，递推深度等于打开期间数，通常为 1 至 3。

损益类科目不做特殊处理：年中保留累计余额由普通结转承担，年度末次期间结转后其期末为零，下一年度首期期初因此自然为零。

期初余额批次确认时，把各行写入建账首期的 opening_balance_amount 并置 is_opening_fixed 为真。

### 9.5 API 契约

统一约定：路径前缀 /api/v1/ledger；请求头按基线第 5.6 节固定集合；写请求必带 Idempotency-Key；封套按基线第 5.2 节；分页、排序与过滤按第 5.3 节，列表默认排序为单据与台账按 created_at desc, id desc、档案按 code asc、账表按 accounting_period_id asc, doc_no asc；错误分类只用基线第 5.5 节的五类；对当前安全上下文不可见的记录一律 404 与 PLATFORM.AUTHZ.NOT_FOUND_OR_DENIED。以下逐个端点只写差异部分。
本模块全部路由的能力域码按 A-20 取 foundation::CapabilityDomain::LedgerPostingClose，动作类别取 foundation::ActionClass 的 Read、Write、Submit、Approve、Export 五值之一，逐用例声明在 crates/contract/ledger/src/capability.rs，命名为 <USECASE_SCREAMING>_DOMAIN 与 <USECASE_SCREAMING>_ACTION，由 xtask configdoc 断言每个 /api/v1/ 路由都能解析到一对常量，缺失即构建失败。本阶段只声明常量，运行期判定归阶段 13。


#### 9.5.1 会计科目表

| 方法与路径 | 说明 |
|---|---|
| GET /api/v1/ledger/accounts | 列表。过滤白名单 code、name、category、balance_direction、account_level、is_active、parent_account_id。排序白名单 code、created_at |
| POST /api/v1/ledger/accounts | 新建。请求体 code、name、category、balance_direction、parent_account_id、is_active。响应 data 为科目视图。权限 ledger.account.manage |
| GET /api/v1/ledger/accounts/{id} | 详情 |
| PATCH /api/v1/ledger/accounts/{id} | 修改 name 与 is_active。请求体必带 row_version。category、balance_direction、parent_account_id 在该科目已产生凭证后不可改，按 U-H-03 临时取值 |
| POST /api/v1/ledger/accounts/{id}/actions/deactivate | 停用。守卫为该科目未被任何 event_account_bindings 引用且无下级启用科目 |
| POST /api/v1/ledger/accounts/{id}/actions/activate | 启用 |

错误码：LEDGER.ACCOUNT.CODE_DUPLICATED、LEDGER.ACCOUNT.LEVEL_EXCEEDED、LEDGER.ACCOUNT.PARENT_NOT_FOUND、LEDGER.ACCOUNT.PARENT_IS_LEVEL_TWO、LEDGER.ACCOUNT.CATEGORY_DIRECTION_MISMATCH、LEDGER.ACCOUNT.HAS_POSTED_VOUCHERS、LEDGER.ACCOUNT.BOUND_TO_EVENT_ROLE、LEDGER.ACCOUNT.HAS_ACTIVE_CHILDREN。版本冲突按基线映射为 PLATFORM.CONCURRENCY.STALE_VERSION。

#### 9.5.2 期初余额

| 方法与路径 | 说明 |
|---|---|
| POST /api/v1/ledger/opening-balance-batches | 新建批次，status = DRAFT |
| POST /api/v1/ledger/opening-balance-batches/{id}/actions/append-lines-batch | 追加行，单次上限 200 条，超出返回 VALIDATION |
| POST /api/v1/ledger/opening-balance-batches/{id}/actions/submit-for-approval | 提交审批。守卫为借贷合计相等且该法人尚无任何凭证 |
| POST /api/v1/ledger/opening-balance-batches/{id}/actions/confirm | 审批通过后确认，写入首期期初并固化 |
| GET /api/v1/ledger/opening-balance-batches 与 /{id} | 查询 |

错误码：LEDGER.OPENING_BALANCE_BATCH.UNBALANCED、LEDGER.OPENING_BALANCE_BATCH.VOUCHER_EXISTS、LEDGER.OPENING_BALANCE_BATCH.ACCOUNT_DUPLICATED、LEDGER.OPENING_BALANCE_BATCH.ALREADY_CONFIRMED。

手工录入与规格第 7.10 章迁移批次导入两条路径的互斥按 U-H-04 临时取值：手工录入只允许在该法人尚无任何凭证且尚无已确认的迁移批次期初时执行，要求借贷合计平衡，需审批。
按 A-24，首版不设独立的数据迁移阶段，本节的 POST /api/v1/ledger/opening-balance-batches 与 /{id}/actions/confirm 是总账期初余额的唯一落点。应收应付预收预付期初与资金账户期初归阶段 10，库存期初归阶段 8 的 MIGRATION_STOCK_ADJUSTMENT 来源类型。四个通道的写入一律不生成凭证，期初对应的总账侧由本节的期初余额批次承担，两侧的平衡由 finance 的勾稽视图在首个会计期间校验。


#### 9.5.3 事件科目对应关系

| 方法与路径 | 说明 |
|---|---|
| GET /api/v1/ledger/event-account-bindings | 列出 17 个角色及其当前绑定，未绑定的角色以 account_id 为 null 呈现 |
| PUT /api/v1/ledger/event-account-bindings/{account_role} | 绑定或改绑。请求体 account_id 与 row_version。权限 ledger.event_account_binding.manage |
| POST /api/v1/ledger/event-account-bindings/actions/check-completeness | 返回未绑定角色清单与绑定到停用科目的角色清单，供建账验收与运维中心查看，不供任何启动自检使用 |

变更经 ep-platform-release 的配置发布通道发布，按基线第 7.1 节运行期可变业务参数的口径。该通道按 A-27 由阶段 3a 交付端口、阶段 3b 交付最小发布通道，本阶段只作为使用方接入，不自建第二套发布路径。是否另设审批与重新认证按 U-H-06 待决，临时取值为需财务主管审批、不额外重新认证。变更对已生成凭证无影响，因凭证行固化 account_id。

#### 9.5.4 凭证查询

| 方法与路径 | 说明 |
|---|---|
| GET /api/v1/ledger/vouchers | 列表。过滤白名单 accounting_period_id、business_date、doc_no、source_kind、source_document_no、account_id、amount 区间、deferred_from_period_id、created_by。business_date 与 accounting_period_id 是规格要求的两条检索路径，其余按 U-H-09 临时取值提供 |
| GET /api/v1/ledger/vouchers/{id} | 详情，含分录行、来源单据引用、两个日期与顺延标注 |
| GET /api/v1/ledger/vouchers/{id}/lines | 分录行 |

响应中每张凭证一律带 accounting_period_id、business_date 与 is_deferred 三项，is_deferred 由 deferred_from_period_id 非空导出；按原始业务日期检索时结果标注该凭证实际落入的会计期间。按 U-H-10 临时取值，过账接口的响应也回带这三项，使提交回执可即时告知顺延。

界面不提供凭证修改与删除入口；对 vouchers 与 voucher_lines 的任何写请求返回 PERMISSION_DENIED 与 LEDGER.VOUCHER.IMMUTABLE。

#### 9.5.5 账表与试算平衡

| 方法与路径 | 说明 |
|---|---|
| GET /api/v1/ledger/account-balances | 科目余额表。必填 accounting_period_id。输出按科目的期初余额、本期借方发生额、本期贷方发生额、期末余额。对应规格附录 A.1 的月度科目余额表 |
| GET /api/v1/ledger/general-ledger | 总账。必填 accounting_period_id，可选 account_id。按科目按期间给出发生额与余额 |
| GET /api/v1/ledger/subsidiary-ledger | 明细账。必填 account_id 与 accounting_period_id 或 business_date 区间之一，下钻到逐笔分录行，每行同时展示会计期间字段与原始业务日期，并带凭证与来源单据引用。超过 10000 行深偏移时按基线第 5.3 节切换为键集分页 |
| GET /api/v1/ledger/trial-balance | 试算平衡。必填 accounting_period_id。输出借方合计、贷方合计与差额，差额为零即通过。按 U-H-11 临时取值分期初、发生额、期末三段各给一对合计 |
| GET /api/v1/ledger/accounting-equation | 会计恒等取数。必填 accounting_period_id，可选 mode 取 BEFORE_YEAR_END 与 AFTER_YEAR_END。输出四类合计与差额 |

全部只读端点的权限为 ledger.report.read，取数范围一律按法人与会计期间划分的凭证集合，跨法人一律默认拒绝。

#### 9.5.6 会计期间

| 方法与路径 | 说明 |
|---|---|
| GET /api/v1/ledger/accounting-periods | 列表。输出法人、会计期间、状态、当前是否为可入账期间、是否存在已受理未结束的关账请求、关闭时间与本次关账的发起人 |
| GET /api/v1/ledger/accounting-periods/{id} | 详情 |

本阶段不提供任何直接置位期间状态的端点，也不提供反结账与期间重开入口。首个会计期间同样不经端点建立，其唯一建立手段是第 9.4.4 节第二步的零期间分支，在首次过账的同一业务事务内完成，既不经端点也不经测试夹具写库。

#### 9.5.7 期间关账

| 方法与路径 | 说明 |
|---|---|
| POST /api/v1/ledger/period-close-requests | 发起。必带 X-Reauth-Token，其待签内容摘要绑定法人与 accounting_period_id。请求体 accounting_period_id。响应 202 与请求视图。权限 ledger.period_close.request |
| GET /api/v1/ledger/period-close-requests | 列表。过滤白名单 accounting_period_id、status、conclusion、created_by |
| GET /api/v1/ledger/period-close-requests/{id} | 详情。含 refusal_reasons、completed_batch_count、termination_cause、四种结束方式的结论与关联事项引用 |
| POST /api/v1/ledger/period-close-requests/{id}/actions/cancel | 主动取消。必带 X-Reauth-Token，需审批。守卫为 conclusion 为空 |

审批动作本身由 ep-platform-flow 的通用审批端点承载，本阶段不新建审批端点。

同步等待上限按基线第 11.6 节 8 秒，关账为后台任务，发起端点立即返回请求回执，进度经 GET 详情与站内通知回执可见。关账窗口不预设固定上限，界面不得给出固定完成时限的承诺。

错误码：LEDGER.PERIOD_CLOSE_REQUEST.PERIOD_ALREADY_CLOSED、LEDGER.PERIOD_CLOSE_REQUEST.NOT_EARLIEST_OPEN_PERIOD、LEDGER.PERIOD_CLOSE_REQUEST.ANOTHER_REQUEST_IN_PROGRESS、LEDGER.PERIOD_CLOSE_REQUEST.PENDING_POSTING_BACKLOG、LEDGER.PERIOD_CLOSE_REQUEST.UNREPAIRED_DEAD_LETTERS、LEDGER.PERIOD_CLOSE_REQUEST.VALIDATION_DISCREPANCY、LEDGER.PERIOD_CLOSE_REQUEST.VALIDATION_INCOMPLETE、LEDGER.PERIOD_CLOSE_REQUEST.CONCLUSION_ALREADY_PRODUCED、LEDGER.PERIOD_CLOSE_REQUEST.SELF_APPROVAL_FORBIDDEN、LEDGER.PERIOD_CLOSE_REQUEST.REAUTH_REQUIRED。前五个为受理被拒的具体原因，一律以 BUSINESS_CONFLICT 分类返回，并同时体现在 refusal_reasons 中。

#### 9.5.8 年度损益结转

| 方法与路径 | 说明 |
|---|---|
| POST /api/v1/ledger/year-end-closings | 发起。必带 X-Reauth-Token。请求体 accounting_period_id。权限 ledger.year_end_closing.request |
| GET /api/v1/ledger/year-end-closings | 列表，展示历次结转记录 |
| GET /api/v1/ledger/year-end-closings/{id} | 详情，含两张结转凭证引用 |

错误码：LEDGER.YEAR_END_CLOSING.NOT_FISCAL_YEAR_LAST_PERIOD、LEDGER.YEAR_END_CLOSING.PERIOD_NOT_POSTABLE、LEDGER.YEAR_END_CLOSING.ROLE_UNBOUND。是否与期末结账共用同一条审批链按 U-H-15 待决，临时取值为独立审批链、同为财务过账类高风险操作。

#### 9.5.9 模块内契约

ep-contract-ledger 暴露给其他模块的 trait 三个，不经 HTTP。

AccountingPeriodResolver：resolve(tx, ctx, legal_entity_id, posting_date) 返回 ResolvedPeriod，含 accounting_period_id、accounting_period_seq 与 deferred_from_period_id 三项。该方法在同一 &mut dyn Tx 内记忆化，同一事务内重复调用返回同一个值，子账写入方因此不必再靠纪律复用返回值。

PostingPort：post(tx, ctx, PostingInput) 返回 PostingOutcome，取值为 Posted{voucher_id, doc_no, accounting_period_id, deferred_from_period_id}、IdempotentReplay{同上}、Skipped。同一 (legal_entity_id, source_kind, source_document_id, source_sequence_no) 重复提交返回 IdempotentReplay，不重复写余额、不重复写审计、不重复写 Outbox。同一 trait 另有 post_reversal(tx, ctx, reverses_voucher_id, posting_date, source_document)，语义为按原凭证逐行取反生成红字凭证，不查 JOURNAL_MAP、不需要计量项、不需要来源类型入参，新凭证的 source_kind 沿用原凭证取值并写 reverses_id 与逐行的 reverses_id，幂等键与 post 相同。它承接的是分录恒等于原凭证取反的那一类事件，首版即资金单据冲正的到款、付款与退款三种来源。

TotalAccountBalanceProvider：balance(snapshot_ctx, legal_entity_id, accounting_period_id, account_role) 返回 Money，供 ep-platform-recon 在关账勾稽中取总账侧余额。

交付确认的调用形态按 A-09 固定：交付确认单归阶段 6 的 sales schema，其 confirm_delivery 用例在同一事务内按库存腿、过渡科目腿、凭证腿的次序调用三个端口，凭证腿即 PostingPort::post，PostingInput 的 source_kind 取 DELIVERY_CONFIRMED，measures 含 revenue_amount、unbilled_receivable_amount、cogs_amount、inventory_release_amount 四项，直运时后两项缺省因而不生成对应分录行，不再传任何分支参数；会计期间由 AccountingPeriodResolver::resolve 在该事务最前解析一次，库存腿与过渡科目腿复用同一个 ResolvedPeriod。本阶段只提供这两个端口，不建交付确认单，不编排三腿次序。


三个 trait 的方法签名只使用 ep-foundation 与 ep-contract-ledger 自身的类型，不出现数据库行类型与 HTTP 类型，ResolvedPeriod 定义在 ep-contract-ledger 而不下沉到 ep-foundation。事务句柄为 ep_foundation::port::Tx，快照上下文为 ep_foundation::port::SnapshotCtx，跨模块方法一律以 &mut dyn Tx 接收事务句柄，按 A-01 由阶段 1 冻结，本阶段不另定义同名类型。

### 9.6 并发与事务边界

#### 9.6.1 过账事务

一个业务事件一个事务，凭证与业务状态、子账条目、审计事件、Outbox 条目在同一事务内写入。事务内禁止外部 HTTP 调用、文件正文读写、通知发送与长时计算。隔离级别 READ COMMITTED。事务预算按基线第 10.3 节：业务事务不超过 5 秒，读写池 statement_timeout 10 秒、lock_timeout 3 秒、idle_in_transaction_session_timeout 15 秒。

事务内的固定次序：解析期间、映射分录、插入凭证、插入分录行、按 account_id 升序更新余额、写审计、写 Outbox。固定次序是防死锁的主要手段。次序末尾另加一条同事务断言：每条分录行的 accounting_period_id 与 business_date 等于所属凭证的对应值，不成立即回滚。这两列是冗余副本，保留它们是为了明细账与余额校验不回表，一行断言买到的保证与删列相同，且不动任何索引与查询。

幂等：HTTP 层由 Idempotency-Key 与 platform_msg.idempotency_keys 承担，幂等键写入与业务写入同事务；过账层另由 ux_vouchers 的四列唯一键承担，唯一冲突转为 IdempotentReplay。两层幂等不互相替代：前者防同一请求重复提交，后者防同一业务事件经不同路径重复过账。

与 Outbox 的关系：本阶段只产出 ledger.voucher.posted.v1 等 8 个事件，不消费其他模块的业务事件用于过账。ledger 的下游消费者为报表与经营指标模块，其消费幂等由 platform_msg.inbox_consumptions 承担。

序列化失败 40001 与死锁 40P01 由数据访问层统一重试 3 次，退避 50、150、450 毫秒，只对尚未产生任何外部可见副作用的事务重试。过账事务无外部副作用，可重试。

#### 9.6.2 关账的事务边界

关账跨多个事务，逐个列出。

T1 发起事务：写请求行、写审计、提交审批任务、写 Outbox。交互式，在 core-server。

T2 受理事务：slot 行 FOR UPDATE，判定两项前提，置 ACCEPTED，写审计与 Outbox。在 job-worker。slot 行锁是同一法人同一时点只允许一个已受理未结束关账请求的唯一保证，不使用部分唯一索引，理由是基线第 3.10 节禁止部分索引。

T3 在途集合登记事务：取 pg_snapshot_xip 写入请求行。必须在 T2 提交之后开启。

T4 等待：无事务，按轮询间隔取 pg_current_snapshot() 判定。

T5 快照持有事务：REPEATABLE READ 只读，经 UnitOfWork::snapshot_transact 开启并向执行体传入 SnapshotCtx，执行 pg_export_snapshot 并保持打开直到全部批次结束。该连接的 idle_in_transaction_session_timeout 必须为 0，见第 9.12 节偏离登记。

T6..Tn 批次事务：各自 READ COMMITTED 开启后立即 SET TRANSACTION SNAPSHOT，只读，statement_timeout 按 ledger.close.batch_timeout_seconds、work_mem 按 ledger.close.batch_work_mem、temp_file_limit 按 ledger.close.batch_temp_file_limit 单独设置，与只读分析池的同名上限分别取值。

Tn+1 结论事务：slot 行 FOR UPDATE，写结论与 concluded_at，通过时置期间为 CLOSED 并固化下一期间期初，释放 slot，写审计与 Outbox。

取消事务：slot 行 FOR UPDATE，守卫 conclusion 为空。取消与结论并发时由 slot 行锁串行化，先到先得；结论已提交则取消返回 BUSINESS_CONFLICT 与 LEDGER.PERIOD_CLOSE_REQUEST.CONCLUSION_ALREADY_PRODUCED。该处理次序是 U-H-13 的临时取值。

失败重试与补偿：受理判定、等待、快照建立与批次执行均可重放，重放前先读请求当前状态做守卫，不产生重复副作用。批次执行失败按第 9.4.7 节的五类成因置 FAILED_INCOMPLETE，不自动重试本次请求；解除成因后由人重新发起，本阶段不设自动重发起，理由是关账属高风险操作，平台不自行解除任何状态、不存在无发起人的自动动作。

#### 9.6.3 余额行的并发

同一 (科目, 期间) 行在并发过账下会串行。按附录 A.4 的 20 并发与 15% 提交占比、5 至 15 秒思考时间，提交速率约为每秒 0.3 笔，热点行竞争可忽略。更新写为无条件增量，不存在丢失更新。等待锁超过 lock_timeout 3 秒时返回 INFRASTRUCTURE 与限流类错误码并可重试。

#### 9.6.4 RLS 与安全上下文

全部读写经统一数据访问层，安全上下文在连接取用时写入 app.legal_entity_id 等四个会话变量，归还前逐项设回空串。关账校验按规格第 7.7 章的内部对账系统安全上下文执行，按法人逐轮遍历，每轮只写单一法人，不建立跨法人会话，不绕过行级策略。该上下文不调用字段投影器，理由是规格第 17.3 章的判据本身就是该法人的全量合计；这是一句事实描述而不是属性访问控制豁免，本阶段不在规格第 12.2 章的豁免清单上交叉登记。本阶段的校验语句输出列只含勾稽项标识、法人、会计期间、科目、凭证号与金额合计，不含任何行内敏感字段，该边界由第 9.4.7 节的 subject_ref 键集白名单在写入侧强制。

### 9.7 配置项

全部新增键在 EP__LEDGER__ 前缀下，结构体开启 deny_unknown_fields。生效方式一律为启动时读取；标注为可热更的两项经机密与配置版本变更在下次取用时生效，不需重启。

| 键 | 类型 | 默认值 | 生效方式 | 说明 |
|---|---|---|---|---|
| ledger.period.auto_create_lead_days | u16 | 7 | 启动 | 提前多少天建立下一自然月期间 |
| ledger.period.fiscal_year_last_period_no | u8 | 12 | 启动 | 年度末次期间的期号，按 U-H-12 临时取值固定为自然年 12 月 |
| ledger.posting.max_lines_per_voucher | u16 | 500 | 启动 | 单张凭证分录行上限 |
| ledger.close.inflight_wait_poll_interval_ms | u32 | 500 | 热更 | 在途写事务等待的轮询间隔 |
| ledger.close.inflight_wait_warn_seconds | u32 | 300 | 热更 | 等待超时只告警不终止的阈值 |
| ledger.close.batch_size | u32 | 20000 | 热更 | 单批处理的分录行或科目行数 |
| ledger.close.batch_timeout_seconds | u32 | 120 | 热更 | 单批执行时限，触发终止即校验未完成 |
| ledger.close.batch_work_mem | string | "256MB" | 热更 | 批次连接的单查询内存上限 |
| ledger.close.batch_temp_file_limit | string | "4GB" | 热更 | 批次连接的临时空间上限 |
| ledger.close.recovery_mode_batch_size | u32 | 5000 | 热更 | 恢复模式下的分批规模，与常规取值分别冻结 |
| ledger.close.recovery_mode_batch_timeout_seconds | u32 | 300 | 热更 | 恢复模式下的单批执行时限 |

后六项是规格第 10.2 章要求按附录 A.4 认证期实测取值并随认证报告冻结的三项及其恢复模式对应项，本阶段给出的默认值是待冻结前的临时取值，冻结在阶段 14 的认证运行中完成。客户实际数据量超出附录 A.3 基准时按同一取值方法在该部署上重取，重取结论写入部署记录。

运行期可变的业务参数不进配置文件：事件科目对应关系、审批链、科目类别枚举一律存事务数据库并经配置发布通道发布。

启动自检：本阶段不新增任何启动自检项，也不在启动路径上判读任何业务数据行。基线第 7.3 节的 current-period-open 项整项删除，理由是它自述缺失时按规格第 5.2 章自动建立，那是一次写操作而不是闸门，且会让多个进程在自检阶段并发写 ledger 表。原拟挂在其下的两条子判定各自下沉。第一条即每个法人存在当前自然月的打开会计期间，下沉到过账路径，由第 9.4.4 节第二步的零期间分支、第五步的顺延目标自动建立与 job-worker 的提前建立定时任务三者共同承担，其中该法人的首个会计期间定死由第二步的零期间分支在首次过账的同一业务事务内建立，三者都在业务事务内或后台任务内完成，缺失不再影响任何进程启动。第二条即每个法人的 17 个科目角色全部已绑定且绑定到启用科目，下沉到取用点，由第 9.4.3 节第三步的 ROLE_UNBOUND 与 ACCOUNT_INACTIVE 阻断该类事件提交，由第 9.5.3 节的 check-completeness 端点供建账验收与运维中心查看，未绑定按规格第 15.3 章告警并经 ep-platform-obs 的 DegradationLedger::open 登记降级窗口、绑定后关闭。经此，任何一条业务数据不符都不会放大成八个进程集体拒绝启动，而未绑定的后果仍然是显式的、可查询的、会告警的。

### 9.8 测试计划

覆盖率门槛：本阶段全部代码属规格第 17.3 章强制不变量相关代码，行覆盖率不低于 85%；新增与修改代码不低于 80%；工作区整体不低于 80%。工具为 cargo-llvm-cov，CI 上以 --fail-under-lines 强制，路径规则写入 codecov.toml 的 crates/domain/ledger、crates/application/ledger、crates/contract/ledger 三条。不允许长期跳过用例。

#### 9.8.1 单元测试

映射表：遍历 JOURNAL_MAP 的每一条四元组，断言不存在同一 (source_kind, MeasureKey) 的重复条目；断言每个 source_kind 的必填 MeasureKey 集合与可选集合不相交。原先对 source_kind 与 branch 的组合是否落在合法笛卡尔子集内的断言随分支轴删除而取消。

符号归一：正金额、负金额、零金额三个分支；负金额翻转方向后净额不变；零金额不生成行。

映射失败分支：未登记的 MeasureKey、缺失的必填 MeasureKey、角色未绑定、绑定到停用科目、绑定到有下级的一级科目、行数超上限、借贷不平、行数小于 2、全部为零。

期间归属：记账日期晚于服务器自然日、该法人零期间时首次过账建立首期、该法人已有期间而记账日期早于其最早期间起始日、落在可入账期间、落在已关闭期间、落在有已受理未结束关账请求的期间、顺延目标不存在需自动建立、顺延目标跨年度，共 8 个分支。

关账状态机：逐条遍历第 9.4.5 节的转移表，断言每条合法转移成功、每条非法转移返回 BUSINESS_CONFLICT；断言 conclusion 一经非空即不可再变。

年结算法：损益余额全零、单科目非零、多科目正负混合、本年利润角色未绑定、重复执行、非年度末次期间、期间非可入账，共 7 个分支。

余额推演：期初已固化与未固化两条路径；跨已关闭期间递推；损益类跨年归零。

#### 9.8.2 领域属性测试

按基线第 8.1 节，本阶段承担五组不变量中的借贷平衡一组，另自设三组。工具为 proptest，最小用例数 1024。

一是借贷平衡：对任意合法的 (source_kind, measures) 组合，映射产出的凭证借方合计等于贷方合计。新增 VoucherSourceKind 取值的提交必须同批扩展本组用例以覆盖该取值的全部计量项组合，缺失即 CI 失败。

二是期间归属收敛：对任意 (期间集合状态, 合法 posting_date)，resolve 的返回期间一定处于 OPEN 且不存在已受理未结束的关账请求，且其 start_date 大于等于 posting_date 所属期间的 start_date。

三是余额可重放：对任意凭证序列，增量维护的 account_period_balances 与按 voucher_lines 全量聚合的结果逐科目逐期间相等。

四是符号归一保净额：对任意计量项金额序列，归一前后按科目角色的净额相等。

#### 9.8.3 集成测试

使用真实 PostgreSQL 16，每用例独占一个 ep_test_<nanoid> 数据库，用例结束即删库。禁止内存库与 mock。测试数据一律经 ep-testkit 构造器，禁止手写 INSERT。时间经 FixedClock 注入，禁止 sleep。

场景清单。

一是 RLS 与越权：ledger 的 8 张带法人列的表与 platform_core 下 recon 的 2 张带法人列的表在读取、写入、更新、删除、聚合、排序、报表投影与错误信息泄漏八类上不越权；两个复制角色与内部对账系统安全上下文的五个入口借用测试。八类断言复用 testkit/src/rls_matrix.rs 中由阶段 1 提供的 assert_read、assert_write、assert_update、assert_delete、assert_aggregate、assert_sort、assert_report_projection、assert_error_leak 八个函数，入口借用复用阶段 2 提供的 assert_replication_role_containment 与 assert_recon_context_borrow，本阶段不实现同名函数；32 组完整矩阵与发布门禁项 RG-RLS-MATRIX-GREEN 归阶段 4。该组属 tests/rls_matrix。

二是不可覆盖：以 ep_app_rw 对 ledger.vouchers 与 ledger.voucher_lines 执行 UPDATE 与 DELETE 均被数据库拒绝。

三是过账幂等：同一 (source_kind, source_document_id) 连续提交 3 次以上，凭证只产生一次、余额只增一次、审计只写一次、Outbox 只写一条。

四是期间自动建立并发：并发 10 个过账事务同时触发同一期间的自动建立，最终只产生一行，其余走 ON CONFLICT 分支；另一条并发用例针对零期间分支，在该法人无任何期间时并发 10 个首次过账事务，断言最终只产生一行期间、全部凭证落入该期间。

五是受理前提逐项：期间已关闭、非最早打开期间、已有已受理未结束请求、待消费过账非零、未修复死信非零，共 5 个用例，各自断言 refusal_reasons 载明未满足项与其当前取值、期间状态不变、请求可重新发起。

六是连续两次受理被拒的告警与暴露窗口记录，及该期间关账完成后暴露窗口消除。

七是关账受理与在途写事务的交叠：在受理事务提交之前开启一个写事务并使其解析到待关期间，受理后该事务提交，断言其凭证落入该期间、被等待覆盖、进入本次快照。

八是顺延入账：在一次关账请求受理之后、本次关账产生结论之前提交一笔记账日期属于该待关闭期间的业务事件，断言提交成功、凭证记入其后最早的可入账期间、保留原始业务日期、deferred_from_period_id 非空、不进入本次快照、不改变本次关账结论、按两条路径均可检索、同一业务事件的子账条目与该凭证落入同一会计期间、顺延前后借贷与取价均不变。

九是年度末次期间损益归零：由受理时点在途、受理后方可见的写事务使该期间损益余额非零，断言在同一快照上检出、本次关账被拦截、差异事项载明该期间损益类科目余额合计、期间保持打开；再执行一次结转后重新发起并通过。

十是校验未完成：把 ledger.close.batch_timeout_seconds 调到极小使单批超时，断言置 FAILED_INCOMPLETE、生成校验未完成事项且不载明差额字段、告警触发、计入暴露窗口、期间保持打开；解除成因后重新发起并在本执行窗口内通过。

十一是取消与结论并发：同时发起取消与结论落库，断言二者之一成功、另一按 slot 行锁次序返回明确错误、状态唯一。

十二是关账通过后已关闭期间拒绝任何凭证写入，且下一期间的期初已固化并等于本期期末。

十三是试算平衡与会计恒等：注入一张人工构造的不平凭证不可能（受 CHECK 阻断），改为在快照上注入余额行差异，断言校验检出并拦截关账。

十四是期初余额批次：借贷不平拒绝、已有凭证拒绝、重复科目拒绝、确认后进入首期期初列。
十五是受治理数据集视图：以 ep_analyst_ro 连接可 SELECT ledger.v_account_period_balances，该角色对 ledger 的任何基表无读写权限；视图输出含 legal_entity_id、security_level、data_scope_tags 三列且列名与类型签名与 reporting.dataset_fields 中 dataset code 为 ledger_account_period_balances 的登记行逐列相等；跨法人取数被行级策略拦截。本场景只做上述静态比对，不调用阶段 11 的 reporting-dataset-signature-matched 自检项，该自检项按基线第 12 节通则第六条与 E-17 同一档位整条推迟到阶段 11。

#### 9.8.4 端到端测试

后端 E2E 用 Rust 集成测试直接打 HTTP 接口，覆盖规格第 8 章闭环第 13 步期间关账的完整链路：发起、重新认证、审批、受理、等待、快照、校验、通过、期间置为已关闭。

四端 UI 按规格第 6.2 章财务过账与期末结账能力域取值，Windows 与 macOS 为完整，由 Playwright 驱动桌面 WebView 与 tauri-driver 驱动桌面壳执行科目表维护、凭证查询、账表查询、关账发起与跟进、年结发起五个场景；iOS 与 Android 为仅查看，由 XCUITest 与 Espresso 只执行凭证与账表的查看场景，写入操作按清单载明的替代路径验证转桌面端完成。本模块的四端界面按 A-23 由本阶段交付，阶段 13 只提供客户端壳、路由注册表与能力矩阵闸，不交付本模块业务界面。

9b 段另交付黄金业务闭环十四步的整体端到端用例，落点唯一，为 testkit/scenarios/golden_loop_14_steps.rs，覆盖规格第 8 章第 1 至 14 步全程。本用例的判据是全分支闭环而不是首次贯通：首次贯通已由阶段 3b-1 之后的 T0 线完成，第 9.0.2 节的最小切片即本阶段在该线上的份额，本用例只负责把十五类必测分支与七种基础分支跑全。其中第 6、7、9、10、11 步复用阶段 10 已交付的 testkit/scenarios/stage10_ar_ap_closed_loop.rs 的步骤函数，不重写；第 5、8、11 步的库存侧断言引用阶段 8 提供的 ep-testkit 库存断言函数；第 12 步引用阶段 12 的 E2E-01 片段；第 14 步的指标断言引用阶段 11 的取数接口。判据按规格第 8 章闭环验收三条：全程在应用内完成，无外部或线下补齐环节；覆盖规格第 17.2 章十五类必测分支含七种基础分支；收入成本利润三处一致且差额为零，时延按规格第 16 章判定。该用例同时承担 Windows 与 macOS 两端的桌面走查，装置复用本节已交付的 Playwright 与 tauri-driver，因此本项由服务端用例与桌面两端走查两部分构成。阶段 8、阶段 11 与阶段 12 计划中凡指向整条合同闭环用例的措辞一律指向本文件名，不另建第二条链路用例。

#### 9.8.5 性能相关项

基准数据集由 ep-datagen 按附录 A.3 默认 scale 产出：法人 2 个、会计分录 150 万条、期间跨度 36 个。

度量项两个，取自附录 A.1。总账凭证过账按普通交易提交通过线 P95 在 3 秒内；月度科目余额表按常用报表通过线 P95 在 10 秒内。各不少于 200 次样本，只取负载稳定段。

EXPLAIN 证据：过账路径的期间解析、余额 upsert、凭证与分录行插入；科目余额表、总账、明细账、试算平衡四个查询。全部不得出现顺序扫描，证据文件提交到 docs/perf/ledger/。

期间关账窗口按附录 A.1 的非交互场景单列口径记录，不预设固定上限，不按秒级通过线判定；以校验未完成或主动取消结束的窗口只作记录，取值冻结须在同一稳定段内以校验通过或校验不通过结束的那一次窗口上取，冻结动作在阶段 14 完成。

#### 9.8.6 与规格判据的对应

规格第 17.2 章财务内核测试中由本阶段承担并可独立判定的判据：简易总账按固定映射生成凭证；每张凭证与每个会计期间的借贷合计相等；试算平衡通过；月度期间开闭通过；年度损益结转通过；期间关账与凭证会计期间归属的两项验收即顺延入账与年度末次期间损益归零。上述判据在本阶段的集成测试与 E2E 中逐条落为具名用例。

规格第 17.3 章强制不变量中由本阶段承担的：会计借贷平衡、会计恒等成立、已过账凭证不可覆盖，以及子账与总账勾稽的总账侧取数。库存数量守恒、存货金额账与数量账一致、应收应付核销守恒由其所属阶段承担，本阶段只提供总账侧余额与关账编排。

### 9.9 退出条件

以下 23 条全部达成才算本阶段完成，逐条可客观判定。

E-1 四个新增 crate 在 cargo build --workspace 与 cargo clippy --workspace -- -D warnings 下无告警通过；CI 的依赖方向自检脚本对 ledger 的六条断言（清单以第 9.2 节依赖方向自检段为准，阶段 11 追加 ep-contract-costing 后由该阶段同步更新）与对 ep-platform-recon 的一条断言全部通过；单文件不超过 800 行、单函数不超过 50 行、嵌套不超过 4 层的检查通过。

E-2 db/migrations/ledger/ 的 16 个迁移与 db/migrations/platform_core/ 的 3 个对账迁移在空库上离线执行成功，且在含 36 个期间基准数据集的库上重放成功；每个文件的 -- rollback: 段存在；在线变更边界内的操作实测锁持有不超过 5 秒。

E-3 ledger schema 的 8 张带法人列的表与 platform_core 下 recon 的 2 张带法人列的表全部 ENABLE 且 FORCE 行级安全，策略名与模板一致；启动自检的 rls-enabled-and-forced 项在含本阶段表的库上通过。

E-4 tests/rls_matrix 中 ledger 的八类越权用例与五个入口借用用例全部通过。

E-5 以 ep_app_rw 对 ledger.vouchers 与 ledger.voucher_lines 的 UPDATE 与 DELETE 被数据库拒绝，用例留证。

E-6 JOURNAL_MAP 覆盖规格第 5.2 章事件-分录表十类事件的全部分录集合，逐条与该表比对的核对清单由财务负责人或其代表签署，签署件归档到 docs/reviews/。

E-7 四组领域属性测试各不少于 1024 个用例通过。

E-8 十五组集成测试场景全部通过。

E-9 关账受理与在途写事务交叠、顺延入账、年度末次期间损益归零、校验未完成四个用例通过，且四者的断言逐项对应规格第 10.2 章与第 17.2 章的原文判据。

E-10 关账前强制校验的四类校验项各实现一个 ReconCheck 并经 ReconRegistry::register 注册成功，注入借贷差异与损益非零两类差异后差异事项生成且可追溯、本次关账被拦截、差异清零后重新发起正常受理并通过。

E-11 年度损益结转可执行、可重复执行，结转后该期间损益类科目余额为零，会计恒等在结转前后两种取数口径下均成立。

E-12 覆盖率门槛达成：ledger 三个 crate 行覆盖率不低于 85%，新增与修改代码不低于 80%。

E-13 总账凭证过账 P95 不超过 3 秒、月度科目余额表 P95 不超过 10 秒，各不少于 200 次样本；七个查询的 EXPLAIN 证据无顺序扫描。

E-14 docs/error-codes.md 的 LEDGER 段与 crates/contract/ledger 的错误码常量表逐条对齐，无重复码，无仅文档侧或仅代码侧存在的码；docs/event-catalog.md 的 8 个 ledger 事件、docs/data-dictionary/ledger.md 的 12 张表与 2 个视图、docs/data-dictionary/platform_core.md 中对账三张表全部登记，CI 的一致性校验通过。

E-15 新增的 4 个指标在 ops-agent 的 127.0.0.1:9101 上可抓取，标签基数符合基线第 9.2 节纪律。

E-16 桌面端五个场景与移动端两个查看场景的端到端用例通过；移动端写入操作按替代路径验证转桌面端完成。
E-17 受治理数据集视图 ledger.v_account_period_balances 已发布，dataset code 为 ledger_account_period_balances、grain 为 SNAPSHOT，输出含 legal_entity_id、security_level、data_scope_tags 三列，已 GRANT SELECT 给 ep_analyst_ro，其列名与类型签名与 reporting.dataset_fields 中 dataset code 为 ledger_account_period_balances 的登记行逐列相等；本条以该登记行与本阶段交付的视图定义直接对读判定，是静态比对，不调用任何启动自检项。原写的由阶段 11 的 reporting-dataset-signature-matched 自检项校验通过一句，按基线第 12 节通则第六条整条推迟到阶段 11，由阶段 11 的退出条件承担；其重新生效的触发谓词为该自检项已按基线第 7.3 节注册进 SelfCheckRegistry 并可被 --check 模式实际执行，该谓词由判定工具自身可观测，不写成阶段号，也不需要任何人工翻牌动作。本条的达成与否只取决于上述静态比对，不因该自检项尚未交付而恒真或恒不可满足。

E-18 本模块在规格第 6.2 章能力矩阵中取值为完整或简化的能力域，其四端界面已实现并通过 Playwright 与 tauri-driver 的桌面用例、XCUITest 与 Espresso 的移动用例；取值为 VIEW_ONLY 的能力域只实现只读视图；取值为 NOT_APPLICABLE 的不实现入口。

E-19 本阶段全部路由的能力域码与动作类别常量已声明在 crates/contract/ledger/src/capability.rs，xtask configdoc 通过。

E-20 ep-platform-recon 本体已交付并可被其他阶段使用：crate、platform_core 下三张表、ReconCheck 与 ReconRegistry 与 ReconExecutor 三个契约、job-worker 内的分批执行器与每日对账调度齐备；ReconCheck 注册表封闭、无运行期语句入口，archcheck 断言通过，差异事项 subject_ref 的键集白名单在写入侧生效；以内置校验项在两个法人上跑通一次每日对账，差异事项与校验未完成事项均落库且可追溯。

E-21 platform_core.append_only_registry 中 ledger.vouchers、ledger.voucher_lines 与 platform_core.recon_runs 三行已登记，三行的 mode 均为 APPEND_ONLY、mutable_columns 均为空数组，与表上的仅追加约束一致，db/checks/append_only_consistency.sql 经 xtask sqlcheck 通过。

E-22 单据类型码 OBB、GV、PCR、YEC 已登记在 docs/data-dictionary.md 的单据类型码一节，且与 ep-platform-sequence 的常量表逐项一致，xtask configdoc --check-doc-type-codes 通过。

E-23 testkit/scenarios/golden_loop_14_steps.rs 在 ep-datagen 默认 scale 数据集上一次跑通，覆盖规格第 8 章第 1 至 14 步与规格第 17.2 章十五类必测分支，规格第 17.3 章强制不变量在该用例上自动校验通过，Windows 与 macOS 两端的桌面走查同批通过，执行记录纳入发布证据包。


### 9.10 与规格和 PRD 的对应

#### 9.10.1 规格条目

| 规格章节 | 本阶段实现的条目 |
|---|---|
| 第 5.2 章 财务规则 | 每法人单一账簿、本位币人民币；会计科目表最多两级含编码、名称、类别、借贷方向、启用状态与期初余额录入；内置固定的十类业务事件到分录映射；科目对应关系由管理员配置；首版不设库存出入库事件类别 |
| 第 5.2 章 总账功能与期末处理 | 凭证生成、科目余额表、总账与明细账查询、试算平衡、月度期间开闭、年度损益结转；红字冲销与更正凭证只追加不覆盖；损益类年中保留累计余额 |
| 第 5.2 章 业务事件的记账日期 | 记账日期作为期间归属的唯一输入；不得晚于登记时点自然日；补记的记账日期随事件写入审计 |
| 第 5.2 章 凭证的会计期间归属 | 期间只有打开与已关闭两种状态；可入账期间为派生条件；会计期间字段在凭证生成时一次确定；顺延入账；期间按自然月连续建立、首个期间由首次过账在零期间时建立与顺延目标不存在时的自动建立 |
| 第 5.2 章 凭证的两个日期与检索 | 会计期间字段与原始业务日期一并写入凭证并进入总账与明细账；两条检索路径；按原始业务日期检索时标注实际落入的会计期间；已关闭期间不再接受凭证写入；顺延不回迁 |
| 第 5.2 章 顺延只改变期间归属 | 映射与取价与期间无关，由映射算法只读 source_kind 与 measures 保证 |
| 第 5.2 章 子账与凭证共用同一期间归属 | AccountingPeriodResolver 作为连带顺延的唯一入口 |
| 第 5.2 章 年度损益结转与顺延 | 结转凭证记账日期为末次期间期末日；只在该期间为可入账期间时执行；存在已受理未结束关账请求时不执行；每次结转对象为执行时点的损益余额 |
| 第 7.2 章 | 财务模块是正式会计分录与科目余额的唯一权威写入者；已过账分录只追加不覆盖 |
| 第 7.7 章 | 行级隔离以 app.legal_entity_id 为唯一判据；内部对账系统安全上下文的按法人逐轮遍历，其语句集封闭改由无运行期语句入口的静态断言承载，输出边界改由差异事项键集白名单承载；不使用 SET ROLE；连接归还前清除上下文 |
| 第 10.2 章 | 关账受理的两项前提、受理被拒事项与处置路径、受理后的执行次序、四种结束方式、内部对账的执行口径、校验未完成一律按未通过处理、执行窗口与完成要求 |
| 第 12.1 章 | 期末结账与财务过账两类高风险操作的重新认证 |
| 第 12.2 章 | 默认拒绝、申请人不可自审、审批链不可越权跳过 |
| 第 12.5 章 | 业务变更与审计事件同一事务写入；本阶段写审计的动作清单见第 9.11 节 |
| 第 15.1 章 | 五类错误分类与四项要素；权限拒绝不泄露存在性 |
| 第 15.2 章 | 借贷不平不得静默忽略，进入死信与人工修复 |
| 第 15.3 章 | 关账受理被拒、校验未完成的告警与降级及暴露窗口台账 |
| 第 16 章与附录 A.1 A.2 A.3 | 总账凭证过账与月度科目余额表两项度量、期间关账窗口的非交互单列口径、基准数据集规模 |
| 第 17.2 章 | 财务内核测试中总账、试算平衡、期间开闭、年度结转、顺延入账与损益归零六项判据 |
| 第 17.3 章 | 会计借贷平衡、会计恒等成立、已过账凭证不可覆盖、子账与总账勾稽的总账侧 |
| 第 21.20 章 | 顺延入账使期间数据不是严格发生期口径的披露；界面不使用发生期一类措辞 |

#### 9.10.2 PRD 条目

| PRD 节 | 本阶段实现的功能 |
|---|---|
| 7.1 | 角色与职责、每法人单一账簿、端上取值、财务模块唯一权威写入者 |
| 7.2.1 | 科目的建立与维护、七个输入字段与其校验、异常提示 |
| 7.2.2 | 期初余额录入与其进入科目余额表期初列 |
| 7.2.3 | 事件科目对应关系配置、映射不可改写、配置界面不出现库存出入库事件行 |
| 7.3 | 凭证查询、凭证上用户可见的四类内容、两条检索路径、已过账不可覆盖、界面不提供修改与删除入口 |
| 7.4.1 | 科目余额表 |
| 7.4.2 | 总账与明细账、往来明细不在本节承载、首版总账不含辅助核算维度 |
| 7.4.3 | 试算平衡的输入、判定、输出与异常 |
| 7.5 | 会计期间两种状态、可入账期间的派生判定、期间状态流转表、关账顺序、期间列表界面展示项 |
| 7.6.1 | 关账的发起与审批、重新认证、申请人不可自审 |
| 7.6.2 | 两项受理前提、不在受理前判定的内容 |
| 7.6.3 | 受理被拒事项、五条处置路径、发起次数不设上限、连续两次被拒的告警与暴露窗口 |
| 7.6.4 | 受理后的四步执行次序、关账前强制校验的四项范围、两个余额口径的差别 |
| 7.6.5 | 四种结束方式表、判定纪律、已顺延凭证不回迁 |
| 7.6.6 | 关账期间其他用户看到什么 |
| 7.7 | 年度损益结转的触发、操作者、执行前提、系统处理、三类典型异常与处置 |
| 7.8.1 至 7.8.6 | 记账日期规则、顺延触发的两种情形、两个日期、检索方式、连带与不连带范围、口径披露 |
| 7.9 | 八类情形的错误分类与处置 |
| 7.10 | 首版不含的五类，逐条在实现上不提供入口 |

本阶段不实现的相邻能力：应收应付台账与到款付款登记、销项与进项发票登记、库存台账与存货计价、经营驾驶舱与经营报表、审批链与重新认证的通用机制、运维中心的事项台账与告警，均见 needs。

### 9.11 审计事件与可观测性

写审计的动作清单固定 14 个，写入 platform_audit.audit_events，与业务变更同事务：账户创建、账户修改、账户停用、账户启用、事件科目绑定变更、期初余额批次提交、期初余额批次确认、关账请求发起、关账请求受理、关账请求受理被拒、关账请求结论、关账请求主动取消、年结发起、年结执行。高风险操作另在 reauth_ref 与 approval_ref 列记录认证方式、待签内容摘要、时间与设备。期间自动建立按平台产生的状态迁移写审计，actor 取 foundation::SYSTEM_PRINCIPAL_ID、设备标识取 foundation::SYSTEM_DEVICE_ID，安全上下文由 SecurityContext::system 构造，三者按 A-02 与 A-03 由阶段 1 冻结。补记即记账日期早于登记时点自然日的取值随该业务事件写入审计，由调用方在其审计事件中携带，本阶段在 PostingInput 中透出 posting_date 供其取用。

新增指标 4 个，登记入基线第 9.2 节：ep_ledger_posting_duration_seconds 直方图、标签 source_kind；ep_ledger_deferred_vouchers_total 计数器、标签 legal_entity_id；ep_ledger_open_periods 仪表、标签 legal_entity_id；ep_ledger_period_close_window_seconds 直方图、标签 legal_entity_id 与 conclusion。基线已有的 ep_period_close_rejected_total 与 ep_recon_* 由本阶段填充取值。禁止把 doc_no 与 trace_id 作为标签。

日志字段按基线第 9.1 节固定集合，span 名为 ledger.<usecase>。禁止进入日志的内容按同节，凭证金额不属敏感字段但来源单据的行内敏感字段不得随错误上下文外泄。

追踪采样：关账与对账任务、期末结账与财务过账两类高风险操作一律 100%，其余 10%。

### 9.12 本阶段新增决定、偏离基线项与假设

#### 9.12.1 本阶段新增决定，阶段结束时回写共享技术基线

一是金额计算与借贷映射的分层：金额归产生业务事件的模块，方向与科目角色归 ledger，接口为 PostingInput 的计量项。理由见第 9.4.3 节。同条另含映射键的形状：JOURNAL_MAP 由来源类型加分支加计量项的二维键改为来源类型加计量项的一维键，PostingBranch 枚举整体删除，来源类型按分录集合由 11 个拆为 17 个，资金单据冲正改由 PostingPort::post_reversal 按原凭证逐行取反生成而不进映射表，不得新增取值这条封条改为新增取值时映射行、CHECK 迁移与属性测试三者同批的 CI 约束。

二是过账与业务事件同事务，不经 Outbox 异步生成凭证。理由是规格第 10.2 章要求等待受理时点的在途写事务结束后快照即覆盖该期间全部凭证，只有同事务写入才使该论证成立；PRD 第 7.6.6 节与第 7.8.2 节的提交即生成凭证同样指向该形态。

三是受理与在途集合登记分两个事务、先提交受理再取快照的次序，见第 9.4.6 节的论证。

四是同一法人同一时点单一关账请求的串行化点为 ledger.close_serialization_slots 的行锁，不使用部分唯一索引。

五是 ledger.vouchers 与 ledger.voucher_lines 上对 ep_app_rw 撤销 UPDATE 与 DELETE。

六是 ledger.posting_trigger_event_types 退化为会产生凭证的事件类型集合单列表，13 行由本阶段 9a 段的种子迁移一次写全，该迁移由 xtask configdoc 从 docs/event-catalog.md 生成并由 CI 逐字比对；PostingTriggerRegistry 与其运行期断言删除，业务模块既不回填也不在启动时比对。

七是科目余额一致性自检作为关账前强制校验的第四类，是引入增量余额表所必须的自检。

八是四个新增指标与 11 个新增配置键。

九是启动自检不增反减：本阶段不新增任何启动自检项，基线第 7.3 节的 current-period-open 项整项删除，两条原拟子判定分别下沉到过账路径与取用点，见第 9.7 节。其中当前自然月打开会计期间一条的承接方定死为第 9.4.4 节第二步的零期间分支、第五步的顺延目标自动建立与 job-worker 的提前建立定时任务三者，首个会计期间由第二步的零期间分支建立，该分支属 9a 段交付并落在第 9.0.2 节的 T0 切片内。

十是 ep-platform-recon 框架本体归本阶段 9a 段，含 crate、platform_core 下三张表、三个契约、分批执行器与每日对账调度，按 A-06 其余阶段只在其上实现 ReconCheck，不另起第二套对账框架。

十一是本阶段按 9a 与 9b 两段交付，分段清单见第 9.0.1 节，两段之间隔着阶段 8、6、7、10、11。

#### 9.12.2 偏离共享技术基线的项，需同步修订基线

偏离一：ledger.account_period_balances 与 ledger.close_serialization_slots 保留 row_version 列并随更新自增，但更新语句不带 WHERE row_version = $2 的乐观锁比较。理由是余额行是派生汇总、不是用户编辑对象，无条件增量更新在行锁下不存在丢失更新；slot 行用途就是悲观串行化。影响范围限于这两张表，其余可更新表一律按基线第 3.7 节。

偏离二：ledger.vouchers 与 ledger.voucher_lines 是仅追加表却带 doc_no，且不设 status。理由是凭证需要编号以供检索与追溯，但生成即已过账、不存在状态机，设一个恒为单值的 status 列只会制造无意义的枚举。影响范围限于这两张表。

偏离三：ledger.accounting_periods 是业务表却带 status 而不带 doc_no。理由是会计期间不是单据、无编号需求，其唯一键为 (legal_entity_id, period_code)。

偏离四：快照持有连接的 idle_in_transaction_session_timeout 取 0。基线第 10.3 节只对读写池给出 15 秒，本阶段为 job-worker 池上承载快照的那一条连接取 0。理由是 pg_export_snapshot 要求导出事务在各批执行期间保持打开。影响范围限于该用途的连接，job-worker 池其余连接不变。

偏离五：关账前强制校验的批次连接单独设置 work_mem 与 temp_file_limit，与只读分析池的同名上限分别取值。该项在规格第 10.2 章有明文要求，基线第 10.3 节未覆盖，此处补齐。

#### 9.12.3 假设，规格与 PRD 未定义

假设一：该法人已有会计期间而记账日期早于其最早期间起始日的补记按 VALIDATION 拒绝。理由是建账之前不存在该法人的账簿，允许写入会使期初余额的取数起点失去意义。规格只保证记账日期不晚于登记时点自然日，未覆盖这一侧。该法人尚无任何期间的情形不属本假设，按第 9.4.4 节第二步的零期间分支建立首个期间。

假设二：凭证不使用负数金额，amount 恒为正，红字通过方向相反的追加凭证与 reverses_id 表达，计量项为负时按符号归一翻转方向。理由是负数金额会使借贷合计相等这一判据在实现上出现两种等价写法，进而使试算平衡与勾稽的取数出现歧义。

假设三：全部计量项金额为零时不生成凭证，年度损益结转在损益余额全零时不生成凭证。理由是零金额凭证无账务意义且会使借贷平衡校验的凭证张数统计失真。

假设四：DIRECT_EXPENSE_COST 在总账侧只设一个科目角色，按合同、订单、项目的归集由成本归集模块承载。规格第 5.2 章的直接费用类分录写的是按单据携带字段对应的成本科目，未定义该对应关系的配置形态，本阶段按单一角色处理并在 U-H-05 决策后回调。

#### 9.12.4 被待决事项阻塞的判定

| 编号 | 与本阶段的关系 | 本阶段是否阻塞 | 临时取值 | 切换代价 |
|---|---|---|---|---|
| U-H-01 科目类别枚举 | 试算平衡不需要，会计恒等与损益归零需要 | 不阻塞 | ASSET、LIABILITY、EQUITY、PROFIT_LOSS 四类，成本类因首版排除制造而不设 | 增删取值改 CHECK，用 NOT VALID 加 VALIDATE 在线完成；已有数据需按新类别重分类，代价为一次数据回填迁移 |
| U-H-02 科目编码规则 | 需要 | 不阻塞 | 一级 4 位数字、二级为一级加 3 位共 7 位、字符集只允许数字、长度上限 64、唯一性范围为法人 | 放宽字符集只改 CHECK，收紧则需回填 |
| U-H-03 科目使用与维护约束 | 需要 | 不阻塞 | 一级科目在其下已有二级科目时不可直接记账；类别、借贷方向、上级在已产生凭证后不可改；停用前置校验为无启用下级且未被角色绑定；新建默认启用 | 放宽为可改需增加历史凭证归属的迁移路径，代价高 |
| U-H-04 期初两条路径的分工 | 需要 | 不阻塞 | 手工录入只在该法人尚无任何凭证且无已确认迁移批次期初时可用，要求借贷平衡，需审批 | 改为可共存需增加两路径的冲突判定，代价中 |
| U-H-05 科目角色清单与未绑定行为 | 需要 | 不阻塞 | 17 个角色随版本冻结；未绑定时在取用点阻断该类事件提交，并经 DegradationLedger 登记降级窗口与告警，不做任何启动判定；绑定到停用科目阻断提交 | 改为仅告警放行会使凭证生成时失败进入死信，代价高，不建议 |
| U-H-06 绑定变更的治理 | 需要 | 不阻塞 | 经配置发布通道发布，需财务主管审批，不额外重新认证；对已生成凭证无影响 | 增加重新认证只改用例前置校验，代价低 |
| U-H-07 更正凭证入口 | 本阶段不实现 | 不阻塞本阶段，但使首版缺少过账更正路径 | 不提供入口 | 后续新增需在 VoucherSourceKind 上增一个来源类型并放开 source_sequence_no 的 CHECK，属破坏性变更，需升主版本 |
| U-H-08 手工凭证入口 | 本阶段不实现 | 同上 | 不提供入口 | 同上 |
| U-H-09 凭证检索条件集合 | 需要 | 不阻塞 | 在两条日期路径之外提供科目、金额区间、来源事件类型、来源单据号、制单人五项 | 增减过滤字段属向后兼容变更，代价低 |
| U-H-10 顺延提示形态 | 需要 | 不阻塞 | 过账响应与凭证详情一律回带 accounting_period_id、business_date 与 is_deferred 三项；账表与导出携带顺延标识 | 代价低 |
| U-H-11 账表查询口径 | 需要 | 不阻塞 | 三张账表支持单期间与期间区间；已关闭与打开期间在响应中以期间状态字段区分；试算平衡分期初、发生额、期末三段各给一对合计并提供按科目下钻 | 代价低 |
| U-H-12 期间建立口径 | 需要 | 不阻塞 | 会计年度为自然年，末次期间为 12 月期间，提前 7 天自动建立，执行主体为 job-worker 定时任务，建立失败按第 15.3 章告警并由顺延路径兜底补建，新法人首个期间只在首次过账时由第 9.4.4 节第二步的零期间分支按记账日期所属自然月建立，建账侧不另立口径 | 改为非自然年需在 accounting_periods 上引入年度起始月配置并重算 is_fiscal_year_last，代价中 |
| U-H-13 关账界面口径 | 需要 | 不阻塞 | 进度按 completed_batch_count 与总批次数呈现，刷新由前端轮询 GET 详情；取消可用窗口为 conclusion 为空；取消与结论并发按 slot 行锁先到先得 | 代价低 |
| U-H-14 三类事项的呈现位置 | 需要 | 不阻塞 | 三类事项同时进入运维中心与财务侧的关账请求详情，并按 ep-platform-notify 通知发起人与该法人的数据责任人 | 代价低 |
| U-H-15 年结的控制强度 | 需要 | 不阻塞 | 独立审批链，同为财务过账类高风险操作，重复执行按新单据处理并展示历次记录 | 改为与期末结账共用一条链只改审批链引用，代价低 |
| U-H-16 本年利润与未分配利润绑定 | 需要 | 不阻塞 | 由 PROFIT_THIS_YEAR 与 RETAINED_EARNINGS_UNDISTRIBUTED 两个角色承载；本阶段不随交付提供预置科目表模板 | 提供模板属新增数据文件，代价低 |
| U-H-17 记账日期的默认值与补记权限 | 部分需要 | 不阻塞 | 本阶段只在 PostingInput 上接收 posting_date 并做上界校验，默认值与可编辑性由各业务单据所属阶段决定；补记是否需额外权限按待决处理，本阶段不设 | 增加补记权限需在各业务模块的用例上加判定，本阶段无改动 |
| U-A-03 文本长度 | 需要 | 不阻塞 | 按基线第 11.2 节，编码 64、名称 200、备注与原因 2000 | 代价低 |
| U-A-07 科目类别是否允许管理员增删 | 需要 | 不阻塞 | 不允许增删，随版本冻结 | 允许增删需把 CHECK 改为引用配置字典，代价中 |
| U-A-08 期间关账与期初余额录入的默认审批链 | 需要 | 不阻塞 | 关账与年结为财务主管单节点审批，期初余额批次为财务主管单节点审批，一律申请人不可自审 | 代价低，审批链是运行期可配置数据 |

### 9.13 风险与预留

#### 9.13.1 技术风险

风险一：受理与在途写事务等待的次序若被后续重构改动，会在关账快照上留下不可见的缺口，其表现是偶发的期间数据不完整而非报错。控制手段是把第 9.4.6 节的四步次序写成 ADR 并在集成测试第七组用例上做次序断言，同时在 ep-app-ledger 的关账编排上以类型状态表达四步，使跳步无法编译。

风险二：pg_export_snapshot 依赖导出事务长期打开，长事务会拖住数据库的 xmin 推进，影响清理。控制手段是快照事务全程只读、只在校验期间打开，并把其持有时长记入 ep_ledger_period_close_window_seconds，超过 ledger.close.inflight_wait_warn_seconds 的十倍时按规格第 15.3 章告警。

风险三：增量余额表与 voucher_lines 之间可能因缺陷产生漂移，且漂移在报表上表现为正常数值。控制手段是把科目余额一致性列为关账前强制校验的第四类，并在每日校验中同样执行。

风险四：JOURNAL_MAP 的内容正确性无法由代码自证，只能由人对照规格第 5.2 章逐条核对。控制手段是 E-6 的签署核对清单，以及在测试中对每个 source_kind 断言其涉及的科目角色集合与该表一致。一维键使这项核对的对象由一百三十二格降为十七行，人工核对的可行性随之提高。

风险五：关账前强制校验的分批规模、单批时限与单查询资源上限在阶段 14 认证前只有临时取值，客户实际数据量超出基准时可能反复判定为校验未完成而使关账无法通过。控制手段是规格第 10.2 章已给出重取方法，本阶段在配置上把六项做成可热更，并在校验未完成事项中载明触发的具体上限值以便现场重取。

风险六：顺延入账使期间数据不是严格的发生期口径，属规格第 21.20 章已登记的风险。本阶段的控制手段限于两条检索路径与顺延标识，不做追溯重述，界面不使用发生期一类措辞。
风险七：ep-platform-recon 本体在 9a 段交付，而其最重的使用者关账前强制校验编排在 9b 段，中间隔着阶段 8、6、7、10、11 五个阶段，其中阶段 7、8、11 三个陆续注册各自的 ReconCheck，阶段 6 与阶段 10 不注册。若本体的分批语义、快照传递与差异事项模型在此期间被各阶段各自变通，9b 段的关账编排会拿到互不一致的实现。控制手段是把 A-06 冻结的三个契约签名写进 CI 的接口快照断言，9a 段交付时即以内置校验项跑通每日调度的一次完整执行，各阶段注册后即刻纳入每日对账并在其退出条件上留证。


#### 9.13.2 为后续阶段预留的扩展点

一是 VoucherSourceKind 与 source_sequence_no 的组合已为手工凭证与更正凭证留出位置，U-H-07 与 U-H-08 决策后新增来源类型即可，不需改表结构；新增按第 9.4.1 节的同批约束执行，即映射行、CHECK 迁移与借贷平衡属性测试三者同批，不再以不得新增封住，也不因此单独升主版本。

二是 TotalAccountBalanceProvider 是子账与总账勾稽的总账侧唯一入口，子账侧统一经阶段 10 定义的 ep-contract-finance 的 ReconciliationItemQuery 接入；inventory 与 procure 两个模块在阶段 8 与阶段 7 各在本模块的 ep-contract-* 中定义并实现本模块的子账余额端口，即 ep-contract-inventory 的 StockValueSubledgerBalancePort 与 ep-contract-procure 的 GrniSubledgerBalancePort，阶段 10 只注入，finance 与 invoice 两个模块的其余八项子账侧取自阶段 10 自有表、不经这两个端口，两条路径都不需改动本阶段代码。

三是 AccountRole 的 17 个取值中 DIRECT_EXPENSE_COST 预留了按费用类别细分的位置，U-H-05 决策后可扩展为角色加限定符的两段结构，event_account_bindings 的唯一键需相应扩展。

四是 ledger.posting_trigger_event_types 的 13 行由本阶段的种子迁移一次写全，受理前提二的判据在本阶段即已生效；该迁移由 xtask configdoc 从事件目录生成并由 CI 逐字比对，各业务模块既不追加回填迁移也不在启动时比对，不需改动本阶段代码。

五是 account_period_balances 已预留 is_opening_fixed，若后续版本恢复受控反结账，只需把固化位回退并重算，不需改表结构。

六是多账簿、辅助核算维度、过账模拟、合并抵消、多币种与汇兑损益一律不在本阶段留任何半成品字段，按规格第 5.7 章延期，避免留下不承载语义的空列。
