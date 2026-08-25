> **F-57 状态：`HISTORICAL_DO_NOT_EXECUTE`。** 只复用经营事实一致性；当前范围是经营分录/试算/子账对账/经营期间，不是法定账簿、税务、工资或法定年结。

## 阶段 9：财务内核一 —— 总账与期间

> **F-50/F-51/Stage 14 范围修订。** ledger 最终为 14 张表/18 个本目录迁移/13 张法人 RLS 表、34 个 HTTP 端点、9 个 Ledger 事件、36 个 Ledger 自有码，`VoucherSourceKind` 19 项、单据类型码 5 项（新增 `CORR`）、审计动作 15 项；`posting_trigger_event_types` 仍为 13 项且是 14 表中唯一不带 `legal_entity_id` 的表。36 的构成为既有 32 加 F-50 的 4 个；重新认证与自审拒绝传播两个 `PLATFORM` 码，不为其建立 LEDGER 占位码。新增更正凭证头行和 `post_correction`，资金 `post_reversal` 只接受 finance 计算的受控动态拆分；Stage 14 的 `V20261023092600__platform_ops_harden_data_migration_evidence_graph.sql` 再追加专用 `HISTORICAL_MIGRATION` 来源及完整镜像图，不增加 ledger 表、本目录迁移、HTTP、事件或错误码。正文旧 12/16/8、10 张 RLS、31 个端点、8 个事件、34/38 个错误码、旧采购退货三来源、“更正不实现”与“无条件镜像原凭证”均被替代；精确任务见 F-50 实施计划、本计划第 9.4.1—9.4.3 节与 Stage 14 §4.12.1。

### 9.0 本阶段的边界、口径来源与阅读方式

本阶段建设 ledger 模块码下的全部原生能力：会计科目表、期初余额、事件科目对应关系、凭证模型、由规格第 5.2 章事件-分录表驱动的记账引擎、会计期间与可入账期间、记账日期与顺延入账、期间关账的受理前提与关账前强制校验的编排、年度损益结转、试算平衡与三张账表查询。

本阶段不定义任何借贷方向、取价、价差拆分、匹配与核销规则。上述规则一律按规格第 5.2 章财务规则条目的事件-分录表及其后的七个规则块执行。本计划在需要时按事件名称或规则块名称指向该处，不复述其内容。凡本计划出现分录相关表述，一律限于承载结构、映射表的形状、校验与幂等，不涉及规则本身。

本阶段不建设应收应付台账、发票台账、库存台账与成本归集，四者分别归阶段 10、阶段 10、阶段 8 与阶段 11。子账侧取数在关账勾稽中一律经 ep-contract-finance 的 ReconciliationItemQuery 按法人与会计期间取十项勾稽的子账侧合计，结构为 ReconciliationItemView，该 trait 由阶段 10 定义；阶段 8 在 ep-contract-inventory 定义的 StockValueSubledgerBalancePort 与阶段 7 在 ep-contract-procure 定义的 GrniSubledgerBalancePort 是阶段 10 内部组装该结果的手段，本阶段不直接调用。十项中的存货与已收货未收票两项，其子账侧实现体分别由阶段 8 的 InventorySubledgerBalanceQuery 与阶段 7 的 GrniSubledgerBalanceQuery 各自在本模块 contract 的端口上实现，阶段 10 只注入，其余八项取自阶段 10 自有表、不经这两个端口。本阶段只提供总账侧余额。本阶段也不建交付确认单，该单据按 A-09 归阶段 6 的 sales schema，本阶段只提供其收入与成本腿所调用的过账端口。

取值优先级按共享技术基线第 0 节：规格第 13.1、13.3、13.4、7.7 章最高，其次规格其余各章，其次 PRD，最后共享技术基线。本计划中标注为本阶段新增决定与偏离项的条目集中在第 9.12 节，评审时按该节逐条核对。文中出现的按裁定 A-nn、B-nn、C-nn 一律只是决策出处标注，取值以本计划正文与共享技术基线为准，二者不一致时以正文为准，任何取值都不得以裁定表为唯一出处。
#### 9.0.1 本阶段的两段拆分

本阶段按总览第 3.3 节的拆环结论切成 9a 与 9b 两段，落在裁定通则第四条固定的顺序 1 → 2 → 3a → 4 → 3b-1 → T0 → 5 → 9a → 8 → 6 → 7 → 10 → 11 → 9b → 14 上，9a 排在阶段 8 之前、9b 排在阶段 11 之后。阶段 3b-2 不在这条链上，其各项按阶段 3 计划第 3.0 节判定四的下游拉动点排在 T0 之后、各自拉动阶段之前；阶段 12 在阶段 10 之后与阶段 11 并行，阶段 13 在阶段 12 之后与阶段 9b 并行，两者都不在本阶段的前置或后继链上。本计划各节凡涉及分段处一律按下列分工判读。

9a 段交付：ep-contract-ledger、ep-domain-ledger、ep-app-ledger 与 ep-platform-recon 四个 crate；ledger.accounts、ledger.accounting_periods、ledger.event_account_bindings、ledger.opening_balance_batches、ledger.opening_balance_batch_lines、ledger.vouchers、ledger.voucher_lines、ledger.correction_vouchers、ledger.correction_voucher_lines、ledger.account_period_balances、ledger.posting_trigger_event_types 十一张表与 ledger.v_account_period_balances、ledger.v_pending_posting_backlog 两个视图；platform_core 下对账的三张表；AccountingPeriodResolver、PostingPort 与 TotalAccountBalanceProvider 三个初始对外契约；对账框架本体、分批执行器与每日对账调度；总账期初余额通道；受治理数据集视图与 platform_core.append_only_registry 的登记行；本模块四端界面中科目表、凭证、更正凭证与账表的部分。F-50 同批增量再向既有 `ep-contract-ledger` 增加 `F50LockSlicePort` 与 `CrossModuleLockCoordinator` 两个契约及 `f50_lock` DTO，不新增 crate、进程、表或 HTTP；完整首版终态为五个 trait。

9b 段交付：ledger.close_serialization_slots、ledger.period_close_requests、ledger.year_end_closings 三张表；关账请求状态机、受理与在途写事务等待、快照建立、关账前强制校验的编排与四类校验项的注册；年度损益结转；期间关闭时的下一期间期初固化；黄金业务闭环十四步的整体端到端用例 testkit/scenarios/golden_loop_14_steps.rs；本模块四端界面中关账发起跟进与年结发起的部分。

四类关账前强制校验项在 9b 段实现并向 ReconRegistry 注册，9a 段只交付对账框架本体与调度，不注册本模块的校验项。ReconCheck 的注册方按裁定 A-06 固定为阶段 7、8、9b、11 四个，校验项数依次为六、二、四、三共十五个，一律在 apps/job-worker/src/wiring/ 目录下经 ReconRegistry::register 注册，其中 9b 段的四个即本段自带的四类。阶段 10 不注册任何 ReconCheck：其原定的 FIN_CROSS_MODULE_LINK 是纯存在性项，跨 schema 单目标引用改建真实外键后已按 A-06 整条删除；该阶段的十个勾稽项由本段第三个校验项经 ep-contract-finance 的 ReconciliationItemQuery 取子账侧合计，不由阶段 10 自行注册。

#### 9.0.2 本阶段在 T0 贯通线上的最小切片

T0 是插在阶段 3b-1 结束之后、阶段 5 全量开工之前的一条贯通线，其前置为阶段 1、2、3a、4 与 3b-1，在总览第 3.4 节的十五个环节中居第六环。T0 不新增任何范围，只从阶段 5、6、9a、10、11 各取最小切片，判据是一份合同从建单走到管理层看到一个数。本阶段在 T0 中只贡献两样东西：一个会计期间与一张凭证。

本阶段贡献的切片逐项如下：ledger.accounts 与手工建立的少量科目；ledger.event_account_bindings 与该条路径用到的科目角色绑定；ledger.accounting_periods 与该法人的首个 OPEN 期间，该期间由 AccountingPeriodResolver::resolve 第二步的零期间分支在首次过账的同一业务事务内按记账日期所属自然月建立，不经任何端点也不经测试夹具预置；AccountingPeriodResolver::resolve 的第一至第三步且含第二步的零期间分支，顺延的第四第五步在 T0 上走不到；PostingPort::post 与 JOURNAL_MAP 中 SALES_INVOICE_ISSUED 与 RECEIPT_REGISTERED 两个来源类型的行；ledger.vouchers、ledger.voucher_lines 与 ledger.account_period_balances 三张表及其同事务写入；只读端点 GET /api/v1/ledger/account-balances，供 T0 判据读取 ledger 侧当期收入科目的贷方净发生额，与阶段 11 那张收入报表上的数做差额为零的对照，阶段 11 收入卡自身的取数来自其 costing 侧归集台账，不经本端点。

本阶段不进入 T0 的部分：期初余额批次、顺延入账、关账全链路、年度损益结转、对账框架、账表的其余端点、四端界面、性能度量，以及 9b 段全部内容。T0 只用 ep-datagen 最小样本，不要求 scale 数据集，不要求分支覆盖，只要求桌面端。

迁移落法：T0 段执行 db/migrations/ledger/ 的第 1、2、3、6、7、10、14、15 号共八个文件。第 14 号必须与第 15 号同批执行：第 15 号建立的 ledger.v_pending_posting_backlog 连接第 14 号建立的 ledger.posting_trigger_event_types，第 14 号不在 T0 内时该视图在空库上建不起来，且第 15 号会永久取得早于第 14 号的版本号，使版本号必须晚于其全部被引用对象这条判据在 9a 全量段之后的空库回归上同样失败。第 14 号在 T0 段只是一张不带法人列的空登记表，T0 不跑关账受理，v_pending_posting_backlog 在零行登记表上恒返回零，纳入 T0 无附带成本。第 16 号种子迁移不进 T0：该文件由 xtask configdoc 从 docs/event-catalog.md 的 produces_voucher 列生成并逐字比对，T0 时点事件目录尚未补全，此时落库的行数与 9a 全量段的生成结果不符，只能靠改写一个已应用的迁移文件才能让比对通过。其余十个文件中，第 4、5、8、9、16、17、18 号在 9a 全量段追加，第 11、12、13 号按第 9.3 节表下的分段说明在 9b 段追加。全部文件的版本号按各自执行日期取，因此 T0 段的版本号早于 9a 全量段，全局版本序仍单调递增，不产生乱序到达。

T0 通过之后，本阶段其余部分一律改为在这条已贯通的骨架上加厚，不再承担首次贯通的职责：9a 全量段加厚的是期初余额、顺延入账、期初固化、对账框架与账表；9b 段加厚的是关账与年结。第 9.9 节的退出条件不因 T0 增减，仍在 9a 与 9b 各自结束时判定；T0 自身的判据落在 T0 那条线上，本阶段不重复登记。


### 9.1 交付物清单

本阶段结束时，下列东西存在且可运行。

一是四个新增 crate 并可编译通过：ep-contract-ledger、ep-domain-ledger、ep-app-ledger 与 ep-platform-recon，加上 ep-adapter-db-pg 中新增的 ledger 与 recon 两组仓储实现文件组。

二是 db/migrations/ledger/ 下的 18 个迁移文件可在空库上离线执行完成，并可在全局唯一的迁移历史表 platform_core.schema_history 上查得版本；执行后 ledger schema 存在 14 张表与 2 个视图，除 `posting_trigger_event_types` 外的 13 张表全部带法人列并已 ENABLE 与 FORCE 行级安全。另有 db/migrations/platform_core/ 下的 3 个迁移文件建立对账框架的三张表，按 A-06 同属本阶段 9a 段。

三是记账引擎可用：任一业务模块的用例在其事务内经 ep-contract-ledger 的 PostingPort 提交一次过账输入，同事务内生成一张借贷平衡的总账凭证与其分录行、增量更新科目余额，并把审计与 `ledger.voucher.posted.v1` 登记为事务终结待写项；调用方完成全部业务/子账/投影后，按幂等 finish、Outbox、同事务通知、审计终结批的顺序刷新。映射表以编译期常量表存在，键为来源类型与计量项两项的一维查表；规格第 5.2 章的十类事件按分录集合展开为 17 个既有来源类型，F-50 另加受控 `CORRECTION`，Stage 14 再加只供迁移 writer 使用且不进 `JOURNAL_MAP` 的 `HISTORICAL_MIGRATION`，最终 19 个来源类型均有具名测试。

四是会计期间可用：该法人的首个期间由首次过账在同一业务事务内按记账日期所属自然月建立并置为打开，其后各期间按自然月自动建立并置为打开，job-worker 上的定时任务提前建立下一期间；期间归属解析函数可用，含零期间时建立首期、顺延入账与顺延目标不存在时的自动建立。

五是期间关账可用：从发起、重新认证、审批、受理前提判定、受理、等待在途写事务、建立快照、分批执行关账前强制校验，到四种结束方式，全链路在应用内可达，无需线下动作。

六是年度损益结转可用：可在年度末次期间为可入账期间时执行，可重复执行，结转凭证按事件-分录表之外的期末处理块生成。

七是账表查询可用：科目余额表、总账、明细账、试算平衡、会计恒等取数五个只读端点，可按会计期间字段与按原始业务日期两条路径检索，顺延入账的凭证在两条路径上均可查得。

八是文档产物：`docs/error-codes.md` 的 LEDGER 段最终为 36 个具名错误码，其中 32 个本阶段既有码加 F-50 的 4 个更正/资金冲正码；重新认证与自审拒绝传播 `PLATFORM.AUTHZ.REAUTH_REQUIRED` 与 `PLATFORM.SOD.SELF_APPROVAL_FORBIDDEN`，不再计入 LEDGER 自有码。`docs/event-catalog.md` 的 ledger 段最终为 9 个事件；`docs/data-dictionary/ledger.md` 登记 14 张表与 2 个视图，`docs/data-dictionary/platform_core.md` 增补对账三张表的数据字典；`docs/adr/` 新增 3 篇本阶段决定的 ADR。

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

下表文件名是由 `docs/migration-catalog.md` 冻结的精确文件名，不是示例。版本号一律为 `V<YYYYMMDDHHMMSS>` 十四位、全局唯一，并由 xtask sqlcheck 断言；实施不得自行换号、使用伪时间戳或十二位写法。

| 序 | 文件名 | 内容 |
|---|---|---|
| 1 | V20261015090000__ledger_create_accounts.sql | 建 ledger.accounts |
| 2 | V20261015090100__ledger_create_accounting_periods.sql | 建 ledger.accounting_periods；内联自然月一致性、法人内年月/起日唯一与 OPEN/CLOSED 关闭证据形状约束 |
| 3 | V20261015090200__ledger_create_event_account_bindings.sql | 建 ledger.event_account_bindings |
| 4 | V20261015090300__ledger_create_opening_balance_batches.sql | 建 ledger.opening_balance_batches，含 CONFIRMED 条件借贷平衡 CHECK |
| 5 | V20261015090400__ledger_create_opening_balance_batch_lines.sql | 建 ledger.opening_balance_batch_lines；建立非空/头行合计延迟约束与 CONFIRMED 头行明细不可变守卫 |
| 6 | V20261015090500__ledger_create_vouchers.sql | 建 ledger.vouchers；显式建立 `UNIQUE (legal_entity_id,id)`、普通 `UNIQUE (legal_entity_id,reverses_id)` 与同法人 `reverses_id` 自外键，保证一张原资金凭证至多冲正一次 |
| 7 | V20261015090600__ledger_create_voucher_lines.sql | 建 ledger.voucher_lines；显式建立 `UNIQUE (legal_entity_id,id)`、`UNIQUE (legal_entity_id,voucher_id,id)`、普通 `UNIQUE (legal_entity_id,reverses_id)` 与同法人 `reverses_id` 自外键，并建立 `DEFERRABLE INITIALLY DEFERRED` 的通用头行图及资金冲正经济镜像触发器 |
| 8 | V20261015090700__ledger_create_correction_vouchers.sql | 建 ledger.correction_vouchers；引用同法人已过账原凭证，头记录受控更正原因、状态与 `CORR` 单据号 |
| 9 | V20261015090800__ledger_create_correction_voucher_lines.sql | 建 ledger.correction_voucher_lines；逐行引用原凭证行及唯一生成凭证行，冻结 pair 形状、逐行镜像与累计更正上限 |
| 10 | V20261015090900__ledger_create_account_period_balances.sql | 建 ledger.account_period_balances |
| 11 | V20261020091900__ledger_create_close_serialization_slots.sql | 建 ledger.close_serialization_slots；除 `active_close_request_id` 外建立由其空性唯一推导的 stored generated `active_slot_key` 及三列证据候选键；9b 段在阶段 11 完成后执行 |
| 12 | V20261020092000__ledger_create_period_close_requests.sql | 建 ledger.period_close_requests、独立主动取消 reauth/approval/actor 证据及两个 stored generated evidence key；用四条双向长复合 `DEFERRABLE INITIALLY DEFERRED` FK 与三表延迟约束触发器闭合 request↔active slot、PASSED request↔CLOSED period 状态证据图，并以提交点图替换第 2 号迁移的 9a 临时关闭形状 CHECK |
| 13 | V20261020092100__ledger_create_year_end_closings.sql | 建 ledger.year_end_closings；增加执行前非零损益科目数/净余额与失败终结证据、期间身份即时守卫，并建立年结头、0/1/2 张受控凭证、末期期间及锁后余额的 `assert_year_end_closing_graph_consistent()` 延迟图 |
| 14 | V20261015091300__ledger_create_posting_trigger_event_types.sql | 建 ledger.posting_trigger_event_types，并在同一文件内按基线第 3.8 节的正向登记制向 platform_core.unpoliced_table_registry 插入本表一行，五列体例照抄阶段 4 第 29 号迁移，admission_basis 取 SAME_FOR_ALL_ENTITIES，隔离承接入口填第 9.3.12 节的 ledger.v_pending_posting_backlog。该行随本文件一并落地而不另起回填迁移，理由是本文件在 T0 段即执行，登记行缺失时 db/checks 第十三项在 T0 的空库上就返回非零行 |
| 15 | V20261015091400__ledger_create_ledger_views.sql | 建 ledger.v_account_period_balances 与 ledger.v_pending_posting_backlog |
| 16 | V20261015091500__ledger_backfill_posting_trigger_event_types.sql | 一次写全 ledger.posting_trigger_event_types 的 13 行，每行只填 event_type，清单见第 9.3.11 节；本文件由 xtask configdoc 从 docs/event-catalog.md 的 produces_voucher 列生成，CI 比对生成结果与仓库中的本文件是否逐字一致，不一致即构建失败；业务阶段不再追加任何回填迁移 |
| 17 | V20261015091600__ledger_create_dataset_views.sql | 按 A-18 重建 ledger.v_account_period_balances 使其输出 legal_entity_id、security_level、data_scope_tags 三列，并 GRANT SELECT ON ledger.v_account_period_balances TO ep_analyst_ro |
| 18 | V20261015091700__ledger_backfill_append_only_registry.sql | 按 B-02 向 platform_core.append_only_registry 登记 ledger.vouchers、ledger.voucher_lines、ledger.correction_vouchers、ledger.correction_voucher_lines 与 platform_core.recon_runs 五行，五行的 mode 一律取 APPEND_ONLY、mutable_columns 取空数组。文件内先插五行登记，再依次调用 platform_core.attach_table_guards('ledger','vouchers')、('ledger','voucher_lines')、('ledger','correction_vouchers')、('ledger','correction_voucher_lines')、('platform_core','recon_runs')，顺序不得颠倒，挂接函数读登记表取可变列白名单，先挂接后登记取不到 mutable_columns。第 1 至 15 号迁移一律不调用 attach_table_guards，这五张仅追加表的触发器只在本文件内挂接；platform_core.recon_runs 的建表迁移在 db/migrations/platform_core/ 目录且版本号早于本文件，本文件执行时该表已存在，跨目录挂接可行。本文件同时写入 ledger 与 platform_core 两个 schema 的登记对象，其主要创建对象是 ledger 四张仅追加表的登记行与触发器，故放在 db/migrations/ledger/ 目录下，正确性由本文件版本号晚于其全部被引用对象保证，并由空库全量执行验证 |

每个文件头部按基线第 3.9 节写 `-- rollback:` 段。普通建表类的回退语句为 drop table；第 15 号的回退为 drop view；第 16 号为按 event_type 删除本次插入的 13 行；第 17 号为按第 15 号的定义重建视图并 REVOKE SELECT ON ledger.v_account_period_balances FROM ep_analyst_ro；第 18 号为按 schema_name 与 table_name 删除本次登记的五行，并 drop 该五张表上由本文件挂接的 assert_append_only 触发器。第 6 至 9 号文件另注明其中的 REVOKE 语句无法安全逆向，回退须用升级前备份。第 12、13 号的循环证据图不得用一句 `DROP TABLE ... CASCADE` 回退；其精确逆序、空事实预检与恢复对象见第 9.3.9.2、9.3.10.2 节。

上表第 11、12、13 号三个文件属 9b 段，其余十五个属 9a 段。两段之间隔着阶段 8 至 11，因此 9b 段三个文件使用 `20261020091900` 至 `20261020092100` 的冻结版本号，严格排在阶段 11 最后一个 `V20261020091800` 迁移之后；ledger 目录内的相对次序仍按上表。第 2 号表上的 `closed_by_close_request_id` 与第 11 号表上的 `active_close_request_id` 在 9a/第 11 号完成时保持可空且暂不带目标外键；第 12 号目标表建立后不补两条只能证明“id 存在”的短 FK，而一次补齐第 9.3.9.1 节四条双向长证据 FK。四条均 `ON DELETE RESTRICT DEFERRABLE INITIALLY DEFERRED`，因此同一事务内 request、period、slot 三行可任意顺序更新，只有最终图在 `SET CONSTRAINTS ALL IMMEDIATE` 或 COMMIT 判定。

对账框架的三张表按 A-06 建在 platform_core schema，迁移文件放在 db/migrations/platform_core/，精确文件名依次为 `V20261015090110__platform_core_create_recon_check_definitions.sql`、`V20261015090120__platform_core_create_recon_runs.sql`、`V20261015090130__platform_core_create_recon_discrepancies.sql`，三者同属 9a 段且均早于第 18 号登记迁移。列按 A-06 的定义：recon_check_definitions 不带法人列，其准入判据是行集合与法人无关，行由制品决定并在本部署内对全部法人取值相同，隔离承接入口是对账执行器按法人逐轮遍历，该表按正向登记制登记到 platform_core.unpoliced_table_registry，不建行级策略，本计划不再使用全局配置字典这一类名；该登记行由 `platform_core_create_recon_check_definitions` 迁移在同一文件内插入，五列体例照抄阶段 4 第 29 号迁移，admission_basis 取 SAME_FOR_ALL_ENTITIES，隔离承接入口填本句前述的对账执行器按法人逐轮遍历；该表不含 statement_sha256 与 signed_statement_ref 两列，理由见第 9.4.7 节；recon_runs 与 recon_discrepancies 带 legal_entity_id 并按基线第 3.8 节模板建策略；recon_runs 为仅追加表且没有反向冲销语义，故不设 `reverses_id`；recon_discrepancies 为可更新表并带 row_version；`recon_discrepancies(legal_entity_id,recon_run_id)` 指向 `recon_runs(legal_entity_id,id)`，两表的 `(legal_entity_id,accounting_period_id)` 均指向 `ledger.accounting_periods(legal_entity_id,id)`，全部建立真实复合外键并取 `ON DELETE RESTRICT`。被引用表必须显式具备对应 `UNIQUE (legal_entity_id,id)` 候选键。

本阶段不建 CROSS_MODULE_LINK 校验项，依据为裁定 A-06 与总览 R14：所有单目标跨 schema 引用均建真实外键，只有封闭多态引用与两类明确白名单不建外键。`ledger.vouchers.(source_document_type,source_document_id)` 是封闭多态引用，由过账入口按登记对象类型校验；`release_package_id` 与 `approval_ref` 分别属于发布包引用和审批实例引用白名单，由各自受控入口保证。9b 段自带并注册的四个校验项按第 9.4.7 节无一取 CROSS_MODULE_LINK。

公共列在下列各表中一律按基线第 4 节的顺序排列，即 id、legal_entity_id、security_level、data_scope_tags、row_version、created_at、created_by、updated_at、updated_by。仅追加表按基线同节去掉 row_version、updated_at、updated_by；只有存在真实反向链的表才另带 `reverses_id` 并建立同法人自外键，本阶段仅 `ledger.vouchers` 与 `ledger.voucher_lines` 使用该列。更正凭证两表用命名明确的原凭证/原行引用表达更正链，`platform_core.recon_runs` 没有反向冲销语义，三者均不得为了公共形状添加恒空 `reverses_id`。为节省篇幅，下表只列公共列之外的列，并在每表注明其归类。

除不带法人列的登记表以及下文点名的发布包、审批实例和封闭多态白名单外，本阶段所有引用均为数据库真实外键。双方都带 `legal_entity_id` 时，一律用 `(legal_entity_id, ref_id) -> target(legal_entity_id, id)` 复合外键并取 `ON DELETE RESTRICT`，目标表显式提供 `UNIQUE (legal_entity_id,id)` 候选键；以下简写为“fk”的同 schema 引用也适用本句，不得退化为只连 `id` 的单列外键。

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
| period_code | text | 否 | 无 | 形如 202608；`ck_accounting_periods_natural_month` 强制等于 `to_char(make_date(fiscal_year,period_no,1),'YYYYMM')` |
| fiscal_year | smallint | 否 | 无 | ck_accounting_periods_fiscal_year 取 1900 至 9999 |
| period_no | smallint | 否 | 无 | ck_accounting_periods_period_no 取 1 至 12 |
| start_date | date | 否 | 无 | `ck_accounting_periods_natural_month` 强制等于 `make_date(fiscal_year,period_no,1)` |
| end_date | date | 否 | 无 | 同一 CHECK 强制等于 `(make_date(fiscal_year,period_no,1)+interval '1 month'-interval '1 day')::date` |
| status | text | 否 | OPEN | ck_accounting_periods_status 取 OPEN、CLOSED；9a 临时形状与 9b 最终跨表证据图见下文 |
| is_fiscal_year_last | boolean | 否 | false | 同一自然月 CHECK 强制等于 `(period_no=12)` |
| closed_at | timestamptz | 是 | 无 | OPEN 时为空、CLOSED 时非空 |
| closed_by_close_request_id | uuid | 是 | 无 | 触发关闭的关账请求；9b 最终以含期间 id、请求 id 与关闭时点的双向长证据 FK 证明，不建只证明 id 存在的短 FK |

第 2 号迁移首次建表即建立一个 NULL-safe 的 `ck_accounting_periods_natural_month`，用同一 AND 表达式逐值约束 `start_date=make_date(fiscal_year,period_no,1)`、`end_date=(make_date(fiscal_year,period_no,1)+interval '1 month'-interval '1 day')::date`、`period_code=to_char(make_date(fiscal_year,period_no,1),'YYYYMM')`、`is_fiscal_year_last=(period_no=12)`；不得只保留六位格式或起止先后 CHECK。9a 尚无关账请求表时，第 2 号迁移临时建立 `ck_accounting_periods_close_shape`，表达 `(status='OPEN' AND closed_at IS NULL AND closed_by_close_request_id IS NULL) OR (status='CLOSED' AND closed_at IS NOT NULL AND closed_by_close_request_id IS NOT NULL)`；第 12 号迁移在安装最终三表证据图的同一 DDL 事务中删除该即时 CHECK，改由 `DEFERRABLE INITIALLY DEFERRED` 图在提交点证明同一形状及请求终态，避免有效关闭必须把三表按某一种语句顺序写入。自然月索引与候选键为 pk_accounting_periods、ux_accounting_periods_legal_entity_id_period_code、`UNIQUE(legal_entity_id,fiscal_year,period_no)`、`UNIQUE(legal_entity_id,start_date)`、ix_accounting_periods_legal_entity_id_created_at；第 12 号另加第 9.3.9.1 节关闭证据候选键。严格自然月形状加两项唯一键使同法人两个期间不可能重叠；不保留会与 start_date 唯一键重复的普通 start_date 索引。

#### 9.3.3 ledger.event_account_bindings

归类为业务表。

| 列 | 类型 | 可空 | 默认 | 约束与说明 |
|---|---|---|---|---|
| account_role | text | 否 | 无 | ck_event_account_bindings_role 取第 9.4.2 节固定的 17 个角色 |
| account_id | uuid | 否 | 无 | fk_event_account_bindings_accounts，ON DELETE RESTRICT |
| release_package_id | uuid | 是 | 无 | 该绑定由哪个配置发布包发布；属于精确命名的发布包白名单，不建外键，由配置发布入口校验 |

索引：pk_event_account_bindings、ux_event_account_bindings_legal_entity_id_account_role、ix_event_account_bindings_legal_entity_id_created_at。

#### 9.3.4 ledger.opening_balance_batches 与 ledger.opening_balance_batch_lines

头表归类为单据类，另加 doc_no 与 status，类型码 OBB。

头表列：accounting_period_id uuid 非空（建账首期）、source text 非空 ck 取 MANUAL、MIGRATION_BATCH、migration_batch_no text 可空 ck 长度不超过 64、total_debit_amount numeric(18,2) 非空、total_credit_amount numeric(18,2) 非空、status text 非空 ck 取 DRAFT、PENDING_APPROVAL、CONFIRMED、REJECTED、confirmed_at timestamptz 可空、approval_ref uuid 可空。第 4 号迁移建立 `ck_opening_balance_batches_confirmed_balanced`，精确表达式为 `status <> 'CONFIRMED' OR total_debit_amount = total_credit_amount`；不得再写成 CONFIRMED 时跳过校验。头金额与行合计属于跨表谓词，由第 5 号迁移的延迟约束承担。

行表列：opening_balance_batch_id uuid 非空并以 `(legal_entity_id,opening_balance_batch_id)` 真实复合外键指向头表 `(legal_entity_id,id) ON DELETE RESTRICT`、line_no smallint 非空、account_id uuid 非空并同法人指向 accounts、debit_amount numeric(18,2) 非空默认 0、credit_amount numeric(18,2) 非空默认 0。`ck_opening_balance_batch_lines_one_side` 只允许 `(debit_amount>0 AND credit_amount=0) OR (credit_amount>0 AND debit_amount=0)`，不得让双零行通过。

索引：ux_opening_balance_batches_legal_entity_id_doc_no、ix_opening_balance_batches_legal_entity_id_created_at、ux_opening_balance_batch_lines_batch_id_line_no、ux_opening_balance_batch_lines_batch_id_account_id、ix_opening_balance_batch_lines_legal_entity_id_created_at。

第 5 号迁移在头、行两表建立同一套 `DEFERRABLE INITIALLY DEFERRED` 约束触发器。事务提交时，`PENDING_APPROVAL|CONFIRMED` 批次必须至少一行，`SUM(lines.debit_amount)=head.total_debit_amount`、`SUM(lines.credit_amount)=head.total_credit_amount` 且两项相等；聚合以 batch 的法人复合键定位，空集不得用 `COALESCE(0)` 冒充合法零行。头、行可任意先后写入，但在 COMMIT 或显式 `SET CONSTRAINTS ALL IMMEDIATE` 时必须闭合。

同迁移另建即时不可变守卫：`OLD.status='CONFIRMED'` 的头禁止 UPDATE/DELETE；任何父头已为 CONFIRMED 的行禁止 INSERT/UPDATE/DELETE。由 PENDING_APPROVAL 到 CONFIRMED 的唯一胜者可在同一事务完成状态更新与期初投影，提交后源批次及其行永久冻结；不得靠“改成另一组同额行”绕过。运行期角色没有物理 DELETE 期初批次的业务路径。

#### 9.3.5 ledger.vouchers

归类为仅追加表，另加 doc_no，类型码 GV，不带 row_version、updated_at、updated_by，带 reverses_id。

| 列 | 类型 | 可空 | 默认 | 约束与说明 |
|---|---|---|---|---|
| doc_no | text | 否 | 无 | 凭证号，按基线第 11.1 节格式 |
| accounting_period_id | uuid | 否 | 无 | fk_vouchers_accounting_periods，凭证落入哪个会计期间的唯一依据 |
| business_date | date | 否 | 无 | 原始业务日期，取该业务事件的记账日期 |
| deferred_from_period_id | uuid | 是 | 无 | 非空即表示该凭证发生过顺延，fk_vouchers_accounting_periods_deferred |
| source_kind | text | 否 | 无 | ck_vouchers_source_kind 取第 9.4.1 节固定的 19 个来源类型；第 19 项由 Stage 14 的 092600 迁移追加 |
| source_sequence_no | smallint | 否 | 1 | `ck_vouchers_source_sequence` 精确允许：普通来源与 CORRECTION 只取 1；`YEAR_END_PL_CLOSING` 取 1 或 2；`HISTORICAL_MIGRATION` 的 APPLY 根取 1、REVERSE 镜像取 2 |
| source_document_type | text | 否 | 无 | 与来源 id 组成封闭多态来源判别，目标集合由过账登记表固定 |
| source_document_id | uuid | 否 | 无 | 与 type 组成具名封闭多态来源；过账事务按登记目标校验同法人，不建伪外键 |
| source_document_no | text | 否 | 无 | 来源单据编号，冗余存储以支持不回表检索 |
| source_event_id | uuid | 是 | 无 | 触发过账的业务事件标识 |
| total_debit_amount | numeric(18,2) | 否 | 无 | ck_vouchers_balanced 约束等于 total_credit_amount |
| total_credit_amount | numeric(18,2) | 否 | 无 | 同上 |
| line_count | smallint | 否 | 无 | ck_vouchers_line_count 大于等于 2 |
| reverses_id | uuid | 是 | 无 | 本凭证冲销的凭证，`(legal_entity_id,reverses_id)` 真实自外键指向本表 `(legal_entity_id,id)`；只有受控 `post_reversal` 生成的资金冲正和 `reverse_migrated_historical_voucher` 生成的历史迁移镜像非空，普通 `post`、历史 APPLY 根、受控更正与年结凭证均为空 |

候选键与索引：`UNIQUE (legal_entity_id,id)`；普通 `UNIQUE (legal_entity_id,reverses_id)` 允许多个 NULL、但任一非空原凭证 id 只能出现一次，数据库层保证一张原资金凭证或历史迁移 APPLY 根至多生成一张冲正头；pk_vouchers、ux_vouchers_legal_entity_id_doc_no、ix_vouchers_legal_entity_id_created_at、ux_vouchers_legal_entity_id_source_kind_source_document_id_source_sequence_no、ix_vouchers_legal_entity_id_accounting_period_id、ix_vouchers_legal_entity_id_business_date、ix_vouchers_legal_entity_id_source_document_id。

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
| reverses_id | uuid | 是 | 无 | 本行冲销的分录行，`(legal_entity_id,reverses_id)` 真实自外键指向本表 `(legal_entity_id,id)` |

候选键与索引：`UNIQUE (legal_entity_id,id)`、`UNIQUE (legal_entity_id,voucher_id,id)`；普通 `UNIQUE (legal_entity_id,reverses_id)` 允许多个 NULL，但任一非空父分录行只能被一条资金冲正或历史迁移镜像行完整覆盖；pk_voucher_lines、ux_voucher_lines_voucher_id_line_no、ix_voucher_lines_legal_entity_id_created_at、ix_voucher_lines_legal_entity_id_account_id_accounting_period_id、ix_voucher_lines_legal_entity_id_accounting_period_id_account_id、ix_voucher_lines_legal_entity_id_business_date。

第 7 号迁移建立具名 `DEFERRABLE INITIALLY DEFERRED` 通用凭证图约束触发器，头和行任意先后插入都在事务提交时按凭证重读整图。所有凭证都必须满足 `count(lines)=head.line_count`、借方行金额合计等于 `head.total_debit_amount`、贷方行金额合计等于 `head.total_credit_amount`，每行 `accounting_period_id/business_date` 与头逐值相等；因此头部自己写成“平衡”但行图不平、行数错误或行期间漂移仍无法提交。

在通用图上只允许三种 NULL-safe 形状。第一种是非冲正头 `reverses_id IS NULL` 且该头全部行 `reverses_id IS NULL`；`HISTORICAL_MIGRATION` 在此形状下还必须 `source_sequence_no=1`。第二种是受控资金冲正头和全部行 `reverses_id IS NOT NULL`：父头 `reverses_id IS NULL`，`source_kind` 只取 `RECEIPT_REGISTERED|PAYMENT_REGISTERED|CUSTOMER_REFUND|SUPPLIER_REFUND`，子头 source_kind 与父头相同、`source_document_type='CASH_DOCUMENT_REVERSAL'`，子借方总额等于父贷方总额、子贷方总额等于父借方总额，且父头全部行被一一完整覆盖。第三种只允许 Stage 14 的 `reverse_migrated_historical_voucher`：父头必须是未冲正的 `HISTORICAL_MIGRATION/DATA_MIGRATION_RECORD`、`source_sequence_no=1`，子头必须 `source_kind='HISTORICAL_MIGRATION'`、`source_sequence_no=2`，且 `source_document_type/id/no` 与父头逐值相等；子头日期和期间取本次反向执行结果，借贷总额互换。

第二、三种的每条子行均须让 `reverses_id` 指向头所指父凭证内恰一父行，完整复制 `line_no/account_id/account_role/measure_key/amount` 并严格反转 `direction`；父图每行恰被覆盖一次，不得少行、多行、部分金额、换科目/角色/计量项或把同法人另一凭证/同父凭证另一行错配。头与行两项普通 UNIQUE 分别保证同一原凭证和原分录行至多冲正一次，延迟触发器保证至少完整覆盖一次；错 kind/type/sequence/source tuple、父头本身已冲正、原图不完整或任一镜像不等均整笔回滚。

历史来源还必须双向命中 Stage 14 权威证据，不能只写一张形似合法的凭证：sequence 1 头必须由同法人 `data_migration_records.id=source_document_id` 的 APPLY receipt 以 `target_object_type='ledger.vouchers'、target_id=本头 id` 唯一指向，record 的 `module_code/object_type` 必须为 `ledger/historical_voucher`、预留 target id 必须等于本头且 batch_no 等于 source_document_no；sequence 2 头必须由同记录 REVERSE receipt 唯一指向、该 receipt 指回前述 APPLY receipt，并有 Stage 14 R0。反向头 `business_date` 必须等于 `platform_core.business_day(REVERSE receipt.owner_effect_at)`，期间必须是包含该日期的 resolver 结果；APPLY 日期仍取迁移记录内容。Stage 14 的 `V20261023092600__platform_ops_harden_data_migration_evidence_graph.sql` 必须先扩展 `ck_vouchers_source_kind`/`ck_vouchers_source_sequence`，再 `CREATE OR REPLACE ledger.assert_voucher_graph_consistent()` 为上述第三形状与双向 receipt 证明安装静态分支；receipt/record 侧由 `platform_ops.assert_data_migration_evidence_graph_consistent()` 反向验证目标投影。不得复制第二套宽松 trigger，也不得让 092600 在任一方向仍可孤立时启用迁移 writer。应用锁内校验只作第二道防线，不替代数据库约束。

同样执行 REVOKE UPDATE, DELETE ON ledger.voucher_lines FROM ep_app_rw。

#### 9.3.6.1 ledger.correction_vouchers 与 ledger.correction_voucher_lines

两表均为仅追加事实并带法人 RLS，不带 `row_version/updated_at/updated_by`，运行期角色一律撤销 UPDATE 与 DELETE。头表是 `CORR` 单据，业务列固定为 `doc_no text not null`、`source_voucher_id uuid not null`、`reason text not null`、`posting_date date not null`、`accounting_period_id uuid not null`、`deferred_from_period_id uuid null`、`generated_voucher_id uuid not null`、`reauth_ref uuid not null`、`approval_ref uuid not null`、`posted_at timestamptz not null`。`(legal_entity_id,source_voucher_id)` 与 `(legal_entity_id,generated_voucher_id)` 均为指向 `ledger.vouchers(legal_entity_id,id)` 的真实复合外键；`generated_voucher_id` 唯一，`doc_no` 法人内唯一。

行表不是让调用方自由拼借贷的接口，而是 `post_correction` 自动展开的成对证据。业务列固定为 `correction_voucher_id uuid not null`、`pair_no smallint not null`、`line_role text not null`（`REVERSE_ORIGINAL|TARGET`）、`line_no smallint not null`、`source_voucher_line_id uuid not null`、`generated_voucher_line_id uuid not null`、`account_id uuid not null`、`account_role text not null`、`direction text not null`、`amount numeric(18,2) not null`、`memo text null`。`source_voucher_line_id` 与 `generated_voucher_line_id` 均以 `(legal_entity_id,id)` 真实复合外键指向 `voucher_lines(legal_entity_id,id) ON DELETE RESTRICT`。唯一约束为 `(legal_entity_id,correction_voucher_id,line_no)`、`(legal_entity_id,correction_voucher_id,pair_no,line_role)` 与 `(legal_entity_id,generated_voucher_line_id)`，每条更正证据与生成凭证行一一对应。

同一 `(correction_voucher_id,pair_no)` 在提交时必须恰有两行、金额相同、方向相反且引用同一原凭证行：`REVERSE_ORIGINAL` 复制原行 account/account_role、反转方向，生成凭证行 measure_key 固定为 `correction_reverse_original`；`TARGET` 使用命令中获准且不同于原角色的目标角色及其当时绑定科目、保持原方向，生成凭证行 measure_key 固定为 `correction_target`。证据行的 line_no/account/account_role/direction/amount 必须与 `generated_voucher_line_id` 指向的行逐值相等；每个生成行必须属于头的 `generated_voucher_id`，而该生成头必须为 `source_kind=CORRECTION`、`source_document_type='CORRECTION_VOUCHER'`、source_document_id/no 等于 CORR 头、business_date/accounting_period_id/deferred_from_period_id 等于 CORR 的 posting_date/期间。生成凭证的每一行恰有一条证据、证据也不得指向图外行，故 `line_count=2*pair_count` 且不存在额外自由腿。

第 9 号迁移在 correction 头、行与生成 voucher/line 上建立具名 `DEFERRABLE INITIALLY DEFERRED` 图约束触发器。触发器锁读 source voucher line，校验它属于 CORR 头的 `source_voucher_id`，并只按同一原行的 `REVERSE_ORIGINAL.amount` 计算历史累计，要求累计不超过原行金额；TARGET 不重复计入上限。首版严格限制为成本同侧重分类：源角色与目标角色都只允许 `MAIN_OPERATING_COST|DIRECT_EXPENSE_COST` 且必须不同，因收入侧只有 `MAIN_OPERATING_REVENUE` 一个可归集角色，所以收入凭证行不得进入本入口。父属、pair、生成行镜像、同侧角色矩阵、累计上限或头期间任一不符都在 COMMIT 整笔回滚；应用事务末重读只作第二道防线。

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

列：`active_close_request_id uuid null`；`active_slot_key smallint GENERATED ALWAYS AS (CASE WHEN active_close_request_id IS NULL THEN NULL::smallint ELSE 1::smallint END) STORED`。第 11 号迁移建立 `uq_close_serialization_slots_active_evidence UNIQUE(legal_entity_id,active_slot_key,active_close_request_id)`，供第 12 号反向证据 FK 引用；普通 `ux_close_serialization_slots_legal_entity_id` 仍保证每法人至多一行。不存在部分索引，也不使用一个可由应用伪造的 active boolean。slot 行在法人第一次发起期间关账或年结请求时以 `INSERT ... ON CONFLICT(legal_entity_id) DO NOTHING` 建立；关账受理、结论/取消回调与年结执行再按法人锁该唯一行。

索引：pk_close_serialization_slots、ux_close_serialization_slots_legal_entity_id、uq_close_serialization_slots_active_evidence、ix_close_serialization_slots_legal_entity_id_created_at。

#### 9.3.9 ledger.period_close_requests

归类为单据类，类型码 PCR。

| 列 | 类型 | 可空 | 默认 | 约束与说明 |
|---|---|---|---|---|
| doc_no | text | 否 | 无 | 关账请求编号 |
| accounting_period_id | uuid | 否 | 无 | `(legal_entity_id,accounting_period_id)` 真实复合外键指向 ledger.accounting_periods |
| status | text | 否 | 无 | ck 取第 9.4.5 节的 9 个状态 |
| reauth_ref | uuid | 是 | 无 | 真实单列外键指向 `platform_core.reauth_challenges(id)`；写入事务校验挑战所属用户、法人、待签摘要与有效期 |
| approval_ref | uuid | 是 | 无 | 精确审批实例引用，属于具名平台证明白名单；受控事务校验法人、场景与终态，不建伪外键 |
| approved_by | uuid | 是 | 无 | `(legal_entity_id,approved_by)` 真实复合外键指向 `platform_authz.user_legal_entity_grants(legal_entity_id,user_id)`；ck_period_close_requests_no_self_approval 约束不等于 created_by |
| accepted_at | timestamptz | 是 | 无 | 受理时点 |
| inflight_xids | text[] | 否 | '{}' | 受理时点登记的在途写事务标识集合 |
| inflight_wait_completed_at | timestamptz | 是 | 无 | 在途写事务等待结束时点 |
| snapshot_id | text | 是 | 无 | 导出快照标识 |
| snapshot_established_at | timestamptz | 是 | 无 | 快照建立时点 |
| conclusion | text | 是 | 无 | ck 取 PASSED、DISCREPANCY、INCOMPLETE、CANCELLED |
| concluded_at | timestamptz | 是 | 无 | 结论产生时点 |
| refusal_reasons | jsonb | 是 | 无 | 受理被拒时逐项载明未满足的前提项与其当前取值 |
| completed_batch_count | integer | 否 | 0 | 已完成批次数；`ck_period_close_requests_completed_batch_count CHECK (completed_batch_count >= 0)` |
| termination_cause | text | 是 | 无 | ck 取 BATCH_TIMEOUT、RESOURCE_LIMIT、PROCESS_EXIT、CONNECTION_RECYCLED、SNAPSHOT_INVALID |
| cancellation_reauth_ref | uuid | 是 | 无 | 主动取消独立重新认证证据；`fk_period_close_requests_cancellation_reauth` 真实单列 FK 指向 `platform_core.reauth_challenges(id) ON DELETE RESTRICT`，不得复用或覆盖原 `reauth_ref` |
| cancellation_approval_ref | uuid | 是 | 无 | 主动取消独立审批实例；受控回调校验法人、既有 `LEDGER_PERIOD_CLOSE` 场景、`action=CANCEL`、终态与申请人/审批人分离，属于具名平台证明白名单，不建伪 FK，也不得复用原 `approval_ref` |
| cancelled_by | uuid | 是 | 无 | 发起主动取消的操作者；`fk_period_close_requests_cancelled_by_grant` 以 `(legal_entity_id,cancelled_by)` 指向 `platform_authz.user_legal_entity_grants(legal_entity_id,user_id) ON DELETE RESTRICT` |
| active_slot_key | smallint | 生成 | 无 | stored generated；status 属于 ACCEPTED、VALIDATING 时为 1，其余状态为 NULL，应用不可写 |
| passed_accounting_period_id | uuid | 生成 | 无 | stored generated；status=PASSED 时等于 accounting_period_id，其余为 NULL，应用不可写 |

索引：pk_period_close_requests、ux_period_close_requests_legal_entity_id_doc_no、ix_period_close_requests_legal_entity_id_created_at、ix_period_close_requests_legal_entity_id_accounting_period_id。

`active_slot_key` 的精确生成式为 `CASE WHEN status IN ('ACCEPTED','VALIDATING') THEN 1::smallint ELSE NULL::smallint END`；当前九态没有 `WAITING_INFLIGHT/SNAPSHOTTING` 两个额外状态，T2 登记在途集合、T3 等待及快照建立前的请求仍为 ACCEPTED，因此整个已受理未结束窗口都被该 key 覆盖。`passed_accounting_period_id` 的精确生成式为 `CASE WHEN status='PASSED' THEN accounting_period_id ELSE NULL::uuid END`。

##### 9.3.9.1 三表状态证据图

第 12 号迁移建立四个普通、非部分候选键：请求侧 `uq_period_close_requests_active_evidence UNIQUE(legal_entity_id,active_slot_key,id)` 与 `uq_period_close_requests_passed_evidence UNIQUE(legal_entity_id,passed_accounting_period_id,id,concluded_at)`；slot 侧沿用第 11 号的 `uq_close_serialization_slots_active_evidence`；期间侧 `uq_accounting_periods_close_evidence UNIQUE(legal_entity_id,id,closed_by_close_request_id,closed_at)`。候选键包含本身已唯一的 id 是为冻结 FK 列序，不得删短或改成部分唯一索引。

四条双向长 FK 逐字冻结如下，全部使用 PostgreSQL 默认 `MATCH SIMPLE`，并取 `ON DELETE RESTRICT DEFERRABLE INITIALLY DEFERRED`：

| 约束 | 来源列 | 目标列 |
|---|---|---|
| `fk_close_slots_active_request_evidence` | slot `(legal_entity_id,active_slot_key,active_close_request_id)` | request `(legal_entity_id,active_slot_key,id)` |
| `fk_period_close_requests_active_slot_evidence` | request `(legal_entity_id,active_slot_key,id)` | slot `(legal_entity_id,active_slot_key,active_close_request_id)` |
| `fk_accounting_periods_passed_request_evidence` | period `(legal_entity_id,id,closed_by_close_request_id,closed_at)` | request `(legal_entity_id,passed_accounting_period_id,id,concluded_at)` |
| `fk_period_close_requests_closed_period_evidence` | request `(legal_entity_id,passed_accounting_period_id,id,concluded_at)` | period `(legal_entity_id,id,closed_by_close_request_id,closed_at)` |

前两条使任一 ACCEPTED/VALIDATING 请求必须且只能是本法人 slot 当前指针，同时非空 slot 只能指向这两个 active 状态；两个 request 短暂同时变 active、旧请求变 terminal、slot 换指针可按任意语句顺序完成，最终仍只有一个 request 能与单一 slot 三元组互相匹配。后两条使 CLOSED 期间只能指向同法人、同一 `accounting_period_id` 的 PASSED 请求，并强制 `request.concluded_at=period.closed_at`；任一 PASSED 请求也必须被该期间反向指回。OPEN 的两个关闭证据全空；PASSED 之外的请求因 generated period key 为空不能充当关闭证据。默认 MATCH SIMPLE 是为让非 active/非 PASSED 行跳过对应 FK，完整空性由下述延迟图判定，不得擅改为 MATCH FULL。

第 12 号迁移创建 owner 为 `ep_mod_ledger`、`SECURITY DEFINER SET search_path=pg_catalog,ledger` 的 `ledger.assert_period_close_state_graph_consistent()`，并在 accounting_periods、period_close_requests、close_serialization_slots 三表分别挂 `ct_accounting_periods_close_state_graph`、`ct_period_close_requests_state_graph`、`ct_close_serialization_slots_state_graph`，均为 `AFTER INSERT OR UPDATE OR DELETE FOR EACH ROW DEFERRABLE INITIALLY DEFERRED` constraint trigger。函数统一按“legal_entity_id 对应 slot 行 → accounting_period_id → request_id UUID bytes”固定顺序锁读最终图；任一 request 均要求同法人 slot 行已存在，即使其尚未 active。函数拒绝单向链接、错法人/错期间/错时点、active request 无 slot、slot 指 terminal、PASSED 无 CLOSED period、CLOSED 指 CANCELLED/FAILED/另一期间，以及下列逐态证据形状；重复触发只重复只读断言，不产生写副作用。直接 SQL 即使绕开应用的显式 slot 锁，提交点也会取得同一串行化行，不能与年结或另一关账图交错成双终态。

逐态提交形状为：

- PENDING_APPROVAL：`reauth_ref/approval_ref` 非空；approved、accepted、wait、snapshot、conclusion、concluded、refusal、termination 与三项 cancellation 证据全空，inflight_xids 为空且 completed_batch_count=0。
- APPROVAL_REJECTED：保留 reauth/approval 证明；approved 与全部执行/结论/cancellation 证据为空，inflight_xids 为空且 completed_batch_count=0。
- ACCEPTANCE_REFUSED：reauth/approval/approved_by 非空，`refusal_reasons` 必须为非空 JSON array；accepted、wait、snapshot、conclusion、concluded、termination 与三项 cancellation 证据为空，inflight_xids 为空且 completed_batch_count=0。
- ACCEPTED：reauth/approval/approved_by/accepted_at 非空；snapshot、conclusion、concluded、refusal、termination 与三项 cancellation 证据为空，completed_batch_count=0；inflight_xids 可为空或为 T2 已冻结集合，inflight_wait_completed_at 可空。
- VALIDATING：reauth/approval/approved_by/accepted_at/inflight_wait_completed_at/snapshot_id/snapshot_established_at 非空，conclusion、concluded、refusal、termination 与三项 cancellation 证据为空，completed_batch_count>=0。
- PASSED 与 FAILED_DISCREPANCY：前述验证证据全部非空，`conclusion` 分别严格为 PASSED、DISCREPANCY，concluded_at 非空，refusal/termination/cancellation 证据为空；只有 PASSED 进入期间关闭双向图。
- FAILED_INCOMPLETE：验证证据非空，`conclusion=INCOMPLETE`、concluded_at 与 termination_cause 非空，refusal/cancellation 证据为空。
- CANCELLED：`conclusion=CANCELLED`、concluded_at、cancellation_reauth_ref、cancellation_approval_ref、cancelled_by 全部非空，refusal/termination 为空；三项取消证据来自同一批准的取消动作，且 `cancellation_reauth_ref<>reauth_ref`、`cancellation_approval_ref<>approval_ref`，原 `reauth_ref/approval_ref` 保留且逐值不变。它可保留取消前已形成的合法 PENDING、ACCEPTED 或 VALIDATING 证据前缀，但 `accepted_at IS NULL` 时 inflight 必为空且 wait/snapshot 为空，wait 为空时 snapshot 为空，snapshot_id 与 snapshot_established_at 必须同空同非空，snapshot 为空时 completed_batch_count 必为 0；事件的 cancelled_at 唯一取 concluded_at，不另加可漂移列。

所有状态都要求 completed_batch_count>=0、snapshot 两字段同空同非空；存在 accepted/wait/snapshot/concluded 时按 `accepted_at<=inflight_wait_completed_at<=snapshot_established_at<=concluded_at` 的已出现前缀保持单调。另建两个即时非 constraint trigger `guard_period_close_request_transition`、`guard_accounting_period_transition`，分别调用同名的 `ledger.guard_period_close_request_transition()`、`ledger.guard_accounting_period_transition()`：请求 INSERT 只能 PENDING_APPROVAL，UPDATE 只允许第 9.4.5 节列出的边且终态业务证据不可再改，DELETE 一律拒绝；期间 INSERT 只能 OPEN，status 只允许 OPEN→CLOSED，CLOSED 不可重开。即时守卫只判状态边与身份不可变，不判尚未写齐的跨表证据；因此同一合法事务可先写 request、period 或 slot 中任一行，再由延迟图在提交点统一裁决。

##### 9.3.9.2 回退与 catalog 证据

9b rollback 的唯一合法逆序是第 13 号 → 第 12 号 → 第 11 号。第 12 号 down 首先以 pg_catalog 断言第 13 号的 year_end 表、图 trigger 与函数均已不存在；任一仍在即失败关闭，禁止拆掉其依赖的 slot/request 图。随后取得三表维护锁并执行空事实预检：`period_close_requests` 必须为空、全部 `accounting_periods.closed_by_close_request_id` 与 `close_serialization_slots.active_close_request_id` 必须为空；任一不满足即失败关闭，要求从升级前备份恢复，禁止丢弃既有关账证据。预检通过后严格按“删除三表 constraint trigger 与两个即时 guard trigger → 删除 `assert_period_close_state_graph_consistent()` 及两个 guard 函数，共三个函数 → 删除 accounting_periods/slot 指向 request 的外向 FK → 删除 accounting_periods 证据候选键 → `DROP TABLE ledger.period_close_requests RESTRICT` → 恢复第 2 号精确 `ck_accounting_periods_close_shape`”执行。request 自身两条反向 FK、generated 列和候选键随表删除；不得先 drop request、使用 CASCADE，或在还有事实时只留下悬空 UUID。

第 11 号 down 只可在第 12、13 号表/trigger/函数已全部不存在后执行；它先锁 close_serialization_slots 并断言所有 active_close_request_id 均为空，再以 `DROP TABLE ledger.close_serialization_slots RESTRICT` 删除 slot、generated key 与候选键。slot 中只剩可由后续首次请求重建的空串行化行，因此无需伪造业务清理；若仍有 active 指针、incoming dependency 或任一后置图对象，down 必须失败且不删任何行。第 11、12 号分别执行空库 up/down/up 后，对象签名与 generated 表达式逐字一致。

pg_catalog 门禁逐条核对：两个 generated 列及 slot generated 列 `pg_attribute.attgenerated='s'` 且 `pg_get_expr(adbin,adrelid)` 等于冻结 CASE；四条状态长 FK 的来源/目标列序逐项相等、`confdeltype='r'`、`condeferrable=true`、`condeferred=true`，cancellation_reauth/cancelled_by 两条证据 FK 的目标表与列序正确且 `confdeltype='r'`；四个候选键 `pg_index.indpred IS NULL`；三条 constraint trigger 的 `tgconstraint<>0/tgdeferrable/tginitdeferred` 全真且指向同一函数；两个即时 guard 非 constraint trigger；完整 up 后 `ck_accounting_periods_close_shape` 不存在，第 12 号 down 后它恢复且上述状态/取消 FK、三图 trigger、request 表不存在，第 11 号 down 后 slot 表及其 generated/candidate key 不存在。空库严格按 11→12→13 up、13→12→11 down、再 up 后对象签名逐字一致。

#### 9.3.10 ledger.year_end_closings

归类为单据类，类型码 YEC。

列：doc_no、fiscal_year smallint 非空、accounting_period_id uuid 非空且以同法人复合外键指向 accounting_periods、status text 非空 ck 取 PENDING_APPROVAL、APPROVED、EXECUTED、REJECTED、FAILED、sequence_no smallint 非空（同一年度内第几次结转）、reauth_ref uuid 可空且以真实单列外键指向 `platform_core.reauth_challenges(id)` 并在写入事务校验用户/法人/摘要/有效期、approval_ref uuid 可空且属于审批实例白名单、approved_by uuid 可空且以 `(legal_entity_id,approved_by)` 真实复合外键指向 `platform_authz.user_legal_entity_grants(legal_entity_id,user_id)` 并 ck 不等于 created_by、pl_carry_voucher_id uuid 可空、retained_earnings_voucher_id uuid 可空、executed_at timestamptz 可空、`failure_code text null`、`concluded_at timestamptz null`、`profit_loss_nonzero_account_count_before integer null`、`profit_loss_net_balance_before_amount numeric(18,2) null`。`ck_year_end_closings_failure_code` 的精确表达式为 `failure_code IS NULL OR failure_code IN ('PERIOD_NOT_POSTABLE','ROLE_UNBOUND')`，`ck_year_end_closings_nonzero_count` 为 `profit_loss_nonzero_account_count_before IS NULL OR profit_loss_nonzero_account_count_before >= 0`；`fk_year_end_closings_pl_carry_voucher` 从 `(legal_entity_id,pl_carry_voucher_id)` 指向 `vouchers(legal_entity_id,id)`，`fk_year_end_closings_retained_earnings_voucher` 从 `(legal_entity_id,retained_earnings_voucher_id)` 指向同一目标，两者均为 `ON DELETE RESTRICT DEFERRABLE INITIALLY DEFERRED`，允许凭证与 closing 任意先后插入。

索引：pk_year_end_closings、ux_year_end_closings_legal_entity_id_doc_no、ux_year_end_closings_legal_entity_id_fiscal_year_sequence_no、ix_year_end_closings_legal_entity_id_created_at。

##### 9.3.10.1 年结终态、凭证与余额证据图

第 13 号迁移创建 `ledger.assert_year_end_closing_graph_consistent()`，在 year_end_closings、vouchers、voucher_lines 三表分别挂 `ct_year_end_closings_graph/ct_year_end_vouchers_graph/ct_year_end_voucher_lines_graph`；三者都是 `AFTER INSERT OR UPDATE OR DELETE FOR EACH ROW DEFERRABLE INITIALLY DEFERRED` constraint trigger。voucher trigger 只在 OLD/NEW 任一侧 `source_kind='YEAR_END_PL_CLOSING'` 时进入图校验；voucher_lines 本身没有 source_kind，line trigger 必须用 OLD/NEW.voucher_id 锁读同法人 voucher 头后判定，禁止在 line 上引用一个不存在的伪列写 WHEN。函数 owner 为 ep_mod_ledger，`SECURITY DEFINER SET search_path=pg_catalog,ledger`，按“法人 close_serialization_slots 行 → accounting_period_id → account_id UUID bytes → source_sequence_no → line_no”固定顺序锁读本次 closing 的 slot、期间、科目、余额、两张凭证及行；任一 closing 均要求同法人 slot 行已存在，终态转移必须持有它。函数只验证本次状态转入/本次 YEAR_END 凭证，不因日后同期间新增普通业务凭证而重验历史 closing。第 13 号另在 accounting_periods 建即时非 constraint trigger `guard_year_end_accounting_period_identity`，调用同名 `ledger.guard_year_end_accounting_period_identity()`；一旦存在引用该期间的 year_end_closings，`fiscal_year/period_no/is_fiscal_year_last/start_date/end_date` 全部不可改。它不替换、不改写第 12 号的 `guard_accounting_period_transition()`，所以两个迁移可各自精确回退。

所有 closing 都必须满足 `fiscal_year=accounting_periods.fiscal_year` 且目标期间 `is_fiscal_year_last=true`、`period_no=12`；错年度或非末期在 INSERT 的延迟图提交点拒绝，`NOT_FISCAL_YEAR_LAST_PERIOD` 仍是创建前的应用错误，不进入已建单 FAILED。五态提交形状冻结为：

- PENDING_APPROVAL：reauth_ref、approval_ref 非空；approved_by、两个 voucher、executed_at、failure_code、concluded_at 与两个执行前控制字段全空。
- APPROVED：再要求非申请人的 approved_by 非空；全部执行/失败字段仍为空。
- REJECTED：只保留 reauth/approval 及审批拒绝状态，approved_by 与全部执行/失败证据为空。
- FAILED：保留 reauth/approval/approved_by，`failure_code` 与 concluded_at 非空；executed_at、两个 voucher 及两个执行前控制字段全空。只有 APPROVED 后锁后重验得到 `PERIOD_NOT_POSTABLE|ROLE_UNBOUND` 才可进入；基础设施、超时、进程或事务失败整笔回滚并保留 APPROVED 供 worker 重试/死信，不伪造第三个 failure code。
- EXECUTED：reauth/approval/approved_by、executed_at、`profit_loss_nonzero_account_count_before` 与 `profit_loss_net_balance_before_amount` 非空，failure_code/concluded_at 为空；count=0 当且仅当 net=0 且两个 voucher 都为空，count>0 当且仅当 pl_carry_voucher_id 非空，retained_earnings_voucher_id 非空当且仅当 net 不等于 0，两个非空 voucher id 必须不同。因此 count>0/net=0 的合法形状是第一张非空、第二张为空，不生成零金额凭证。

图函数只在 APPROVED→FAILED/EXECUTED 的本次终态转移上判定锁后条件：EXECUTED 必须看到期间仍为 OPEN，且本法人 close_serialization_slots.active_close_request_id 为空，也即本法人任何期间均不存在 status 属于 ACCEPTED/VALIDATING 的关账请求；`failure_code=PERIOD_NOT_POSTABLE` 必须看到期间非 OPEN 或该 slot 非空；`failure_code=ROLE_UNBOUND` 必须看到期间仍为 OPEN、该 slot 为空，但按当前非零损益余额推导出的本次所需 `PROFIT_THIS_YEAR`/`RETAINED_EARNINGS_UNDISTRIBUTED` 任一绑定缺失、停用或不可过账。两种条件同时出现时优先固化 PERIOD_NOT_POSTABLE。这样不能用合法枚举伪造一个并未发生的 FAILED，也不能在另一期间已占 active slot 时偷跑年结；后续期间关闭或重新绑定不重验历史终态。

两个执行前控制字段不是 HTTP/Excel/插件输入：worker 在锁定该期间全部 PROFIT_LOSS 与 PROFIT_THIS_YEAR 余额行后、写凭证前于同一事务冻结；延迟图以最终余额和本 closing 的凭证行反推 pre-image 逐值复核。第一张存在时必须恰含 count 个执行前非零 PROFIT_LOSS 科目的完整反向清零来源腿，每个科目一腿、金额等于其绝对余额、方向相反，除此只允许在 net 非零时有一条使 PROFIT_THIS_YEAR 产生该 signed net 的平衡腿；提交后这些 PROFIT_LOSS 余额全为零。第二张存在时恰两腿，以 `abs(net)` 反向清零 PROFIT_THIS_YEAR 并等额转入 RETAINED_EARNINGS_UNDISTRIBUTED；提交后 PROFIT_THIS_YEAR 为零。图同时按 voucher_lines 聚合复核本次 account_period_balances 增量，不能只信两个控制字段或余额投影。count>0/net=0 时第一张只由多个损益科目反向腿自行平衡，第二张必须为空。

每张存在的凭证头必须逐值满足：`source_kind='YEAR_END_PL_CLOSING'`、`source_document_type='YEAR_END_CLOSING'`、source_document_id=closing.id、source_document_no=closing.doc_no、source_event_id/reverses_id/deferred_from_period_id 全空、accounting_period_id=closing.accounting_period_id、business_date=period.end_date；pl_carry 的 source_sequence_no=1，retained earnings 的 source_sequence_no=2。反向检查同样成立：任一 YEAR_END_PL_CLOSING voucher 必须由 source_document_id 指向的同法人 EXECUTED closing 在正确槽位引用，禁止孤立、互换 sequence、复用另一 closing 凭证或自由附腿。封闭多态 document type 因此唯一增加 `YEAR_END_CLOSING`，普通 PostingPort 仍拒绝该 source kind。

即时非 constraint trigger `guard_year_end_closing_transition` 调用同名 `ledger.guard_year_end_closing_transition()`，只允许 INSERT=PENDING_APPROVAL、PENDING_APPROVAL→APPROVED|REJECTED、APPROVED→EXECUTED|FAILED，拒绝 DELETE、终态业务证据修改与跳态；逐态字段、凭证、余额和期间关系全部留给提交点图，从而 closing 与已满足普通头行 FK 的 voucher/余额事实可在同一事务中任意排序。

##### 9.3.10.2 年结 rollback 与 catalog 证据

第 13 号 rollback 先锁 year_end_closings/vouchers 并要求 year_end_closings 无行、也不存在 `source_kind='YEAR_END_PL_CLOSING'` 的 voucher；否则失败关闭并要求从升级前备份恢复。空事实时按“删除 year_end_closings/vouchers/voucher_lines 三条 constraint trigger，以及 closing transition/accounting period identity 两个即时 guard → 删除图函数及两个 guard 函数，共三个函数 → 删除两个指向 vouchers 的 FK → `DROP TABLE ledger.year_end_closings RESTRICT`”执行，不用 CASCADE，也不遗留 YEAR_END 凭证；第 12 号 accounting period transition guard 始终保留。pg_catalog 门禁证明两个 voucher FK 的来源/目标列序正确、`confdeltype='r'`、`condeferrable=true`、`condeferred=true`，三条图 trigger 的 `tgconstraint<>0/tgdeferrable/tginitdeferred` 全真且调用 `assert_year_end_closing_graph_consistent()`，两个即时 guard 的 `tgconstraint=0` 且函数各自唯一，failure/count CHECK 的枚举与比较表达式逐字相等；down 后 year_end 表、两 FK、五 trigger、三个函数均不存在，而第 12 号 guard 仍在，空库 up/down/up 后对象签名一致。

#### 9.3.11 ledger.posting_trigger_event_types

本表不带 legal_entity_id。准入判据是其行集合与法人无关：它只登记本版本会产生凭证的事件类型名，取值由制品决定，在本部署内对全部法人相同。隔离承接入口是第 9.3.12 节的 ledger.v_pending_posting_backlog，该视图的取数一律受调用方的 app.legal_entity_id 约束。本表按正向登记制登记到 platform_core.unpoliced_table_registry，不建行级策略；该登记行由第 14 号建表迁移在同一文件内插入，五列体例照抄阶段 4 第 29 号迁移，admission_basis 取 SAME_FOR_ALL_ENTITIES，隔离承接入口即上句所述的 ledger.v_pending_posting_backlog。

列：id uuid 主键、event_type text 非空唯一、created_at、created_by。原有的 ledger_event_kind 与 registered_by_module 两列删除，理由是来源类型已由 JOURNAL_MAP 的键唯一确定，本表只需承载会产生凭证的事件类型集合，供第 9.3.12 节的视图连接。

索引：pk_posting_trigger_event_types、ux_posting_trigger_event_types_event_type。
本表不设任何运行期断言接口。原 PostingTriggerRegistry::assert_registered 与错误码 LEDGER.POSTING_TRIGGER_EVENT_TYPE.REGISTRY_MISMATCH 一并删除，阶段 6、7、10 三处启动自检追加项与其退出码 78 路径随之取消。理由是种子行与编译期常量随同一制品发布，不存在分叉的可能，该断言只是给八个进程多加一条整机拒绝启动的路径，而这台服务器没有备节点。登记表一致性的承接方定死为两条，本阶段不另设第三条：一是 xtask configdoc 从 docs/event-catalog.md 的 produces_voucher 列生成第 16 号迁移并在 CI 中与仓库中的文件逐字比对，不一致即构建失败；二是阶段 3b 的 event-catalog-consistent 自检项，该项不通过时停止派发未登记的事件类型。关账受理前提仍为第 9.4.5 节的两条，不因本表新增第三条前提。按 A-21，登记表与全部 13 行登记行归本阶段 9a 段，业务阶段不再追加任何回填迁移。

唯一约束落在 event_type 上，一行即一个会产生凭证的事件类型。第 16 号种子迁移一次写全 13 行，清单见下表。原先按 ledger_event_kind 各写一行、并为 YEAR_END_PL_CLOSING 保留一行空 event_type 的写法删除：年度损益结转与 F-50 更正凭证均由人在应用内发起，不经业务事件驱动，本表不登记它们；阶段 8 的库存事件不产生凭证，本表也没有它的行。下表的阶段列表示该事件由哪个阶段产生，不表示由哪个阶段写登记行；各业务阶段一律不新增回填迁移，也不在启动时比对。

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

ledger.v_account_period_balances：按法人、科目、会计期间输出期初余额、本期借方发生额、本期贷方发生额、期末余额。期初取数规则为 is_opening_fixed 为真时取 opening_balance_amount，为假时取该科目最近一个已固化期间的期初加上该期间起至目标期间前一期的发生额净额。该视图对 ledger.accounts 与 ledger.accounting_periods 做交叉连接后左连 ledger.account_period_balances，使无发生额的启用科目在科目余额表中仍出现。视图不使用物化视图，首版不使用函数索引与部分索引。该视图同时是 A-18 的受治理数据集，dataset code 为 ledger_account_period_balances，grain 为 SNAPSHOT，输出列另含 legal_entity_id、security_level 与 data_scope_tags 三列，并按第 17 号迁移 GRANT SELECT 给 ep_analyst_ro，列名与类型签名与 reporting.dataset_fields 的登记一致，由阶段 11 的 reporting-dataset-signature-matched 自检项校验。

ledger.v_pending_posting_backlog：按法人与会计期间输出待消费过账条目数与未修复死信条数。受理前提二的判定语句按 C-28 在阶段 4、9、10 三处逐字一致，即：该法人该期间内，platform_msg.outbox_events 中 status 属于 PENDING 或 DISPATCHING、posting_date 落在该期间起止之间、且 event_type 命中 ledger.posting_trigger_event_types 的条目数为零，且 platform_msg.dead_letters 中 state 属于 OPEN 或 REPAIRING、同样命中该注册表的条数为零。posting_date 为空的平台事件一律不计入，理由是它们不产生凭证。该视图是规格第 10.2 章受理前提二的可枚举依据。全部凭证一律与业务事件同事务生成，Outbox 只承载派生、通知、检索与报表数据集，本阶段不存在异步过账路径。

### 9.4 领域模型与关键算法

#### 9.4.1 凭证来源类型

ep-domain-ledger 定义 VoucherSourceKind 枚举，最终 19 个取值。前 17 个取值不按规格第 5.2 章事件-分录表的行数取，而按分录集合取，判据固定为一句：两种情形的科目角色集合不相交，或同一角色方向相反，才拆成两个取值；一种情形只是另一种少几条腿的，不拆，由计量项缺省承担。据此十类事件展开为 16 个业务来源类型，加上期末处理动作共 17 个；F-50 增加受控 `CORRECTION`，只承接“源业务与资金事实均正确、仅已过账科目归类错误”的更正凭证；Stage 14 的 092600 再增加受控 `HISTORICAL_MIGRATION`，只承接已批准历史迁移记录的平衡凭证及其一次完整镜像。后三类专用来源均不提供自由分录。

`DELIVERY_CONFIRMED`、`SALES_INVOICE_ISSUED`、`SALES_INVOICE_REVERSED`、`PURCHASE_INVOICE_INVENTORY_REVERSED`、`PURCHASE_INVOICE_DIRECT_EXPENSE_REVERSED`、`PURCHASE_INVOICE_LINKED_RETURN_REVERSED`、`RECEIPT_REGISTERED`、`PAYMENT_REGISTERED`、`CUSTOMER_REFUND`、`SUPPLIER_REFUND`、`PURCHASE_RECEIPT`、`PURCHASE_INVOICE_INVENTORY`、`PURCHASE_INVOICE_DIRECT_EXPENSE`、`SALES_RETURN`、`PURCHASE_RETURN_INVENTORY`、`OVERBILLING_WRITTEN_OFF`、`YEAR_END_PL_CLOSING`、`CORRECTION`、`HISTORICAL_MIGRATION`。

与原 11 个取值的差别固定为六组。一是销项红字独立为 `SALES_INVOICE_REVERSED`；进项红字按物料独立更正、直接费用更正、链接实物退货三种分录集合拆为三个来源，链接实物退货的红字绝不写库存腿。二是原 `REFUND_REGISTERED` 拆为客户退款与供应商返款，原 `PURCHASE_INVOICE` 拆为物料类与直接费用类。三是物料采购退货无论未开票、已开票或同单混合，一律只生成一张 `PURCHASE_RETURN_INVENTORY` 物理退货凭证；直运/直接费用退货只由 `PURCHASE_INVOICE_DIRECT_EXPENSE_REVERSED` 红字凭证承载，不生成物理库存凭证。四是 `OVERBILLING_WRITTEN_OFF` 承接超量开票路径三，路径一与路径二仍由宿主来源的计量项表达；资金单据冲正不占来源枚举，按第 9.5.9 节 `post_reversal` 处理。五是 `CORRECTION` 只能由受控 `post_correction` 生成。六是 `HISTORICAL_MIGRATION` 只由 `ep-app-ledger` 的 Stage 14 migration writer 生成，APPLY/REVERSE 分别固定 sequence 1/2 与同一迁移记录 source tuple；普通业务、HTTP、Excel、插件和 MCP 均不可调用。

原写的任何阶段不得新增取值一句删除，替换为可执行约束：普通映射来源的新增提交必须同批包含 `JOURNAL_MAP` 对应行、`ck_vouchers_source_kind` 迁移及覆盖全部计量项组合的借贷平衡属性测试；受控特殊来源必须同批包含 CHECK、唯一专用入口、普通入口拒绝测试、数据库最终图与正反属性/集成测试，且不得伪造空 `JOURNAL_MAP` 行。任一集合缺项即 CI 失败。F-50 的 `CORRECTION` 与 Stage 14 的 `HISTORICAL_MIGRATION` 均按后一路径闭合；U-H-08 的手工自由凭证仍不实现，任何专用入口不得被复用为自由凭证。

`HISTORICAL_MIGRATION` 的封闭来源 tuple 唯一为 `source_document_type='DATA_MIGRATION_RECORD'`、`source_document_id=platform_ops.data_migration_records.id`、`source_document_no=platform_ops.data_migration_batches.batch_no`、`source_event_id IS NULL`。APPLY 取 sequence 1、`reverses_id IS NULL`；REVERSE 取同一 type/id/no、sequence 2、`reverses_id=APPLY voucher id`。092600 的 Stage 14 静态投影从同法人已锁定 record/batch 验证 tuple 与 target reservation，ledger 不为封闭多态来源建立伪跨 schema FK。普通 `PostingPort::post/post_reversal/post_correction`、HTTP、Excel、插件和 MCP 必须逐一拒绝该 source kind。

每张来源单据必须恒定映射到唯一取值。采购退货不再保存 `resolved_source_kind`：有实物库存出库的采购退货恒为 `PURCHASE_RETURN_INVENTORY`，其已开票段另由所链接红字单据生成 `PURCHASE_INVOICE_LINKED_RETURN_REVERSED`；直运或直接费用退货只触发 `PURCHASE_INVOICE_DIRECT_EXPENSE_REVERSED`。来源类型只由本次凭证承担的事实决定，不读取随后可能变化的“是否已开票”状态。

#### 9.4.2 科目角色

ep-contract-ledger 定义 AccountRole 枚举，17 个取值，是事件科目对应关系配置的唯一可配置面。角色本身由平台固定并随版本冻结，客户只能把角色绑定到本法人科目表中的具体科目。

ACCOUNTS_RECEIVABLE_UNBILLED、ACCOUNTS_RECEIVABLE、ADVANCE_FROM_CUSTOMER、MAIN_OPERATING_REVENUE、MAIN_OPERATING_COST、INVENTORY、ACCOUNTS_PAYABLE、ACCOUNTS_PAYABLE_ACCRUED、ADVANCE_TO_SUPPLIER、OVERBILLING_SUSPENSE、TAX_PAYABLE_OUTPUT、TAX_PAYABLE_INPUT、BANK_DEPOSIT、CASH_ON_HAND、DIRECT_EXPENSE_COST、PROFIT_THIS_YEAR、RETAINED_EARNINGS_UNDISTRIBUTED。

各角色对应规格第 5.2 章事件-分录表中出现的科目名称，一一对照关系写入 docs/data-dictionary/ledger.md，本计划不复述。

#### 9.4.3 计量项与事件到分录的映射表

本阶段采取的分层是：金额的计算归产生该业务事件的模块，借贷方向与科目角色归 ledger。理由是移动加权平均单价、暂估回冲金额、价差拆分与退货回冲取价一律由规格第 5.2 章的规则块在库存与采购侧维护，且必须与数量账、金额账同源同事务写入，ledger 无法也不应重算；而规格第 5.2 章要求内置固定的业务事件到分录映射，该映射的内容正是方向与科目角色。该分层是本阶段新增决定。按 C-13，取价一律归阶段 8，本阶段不自行取价，ledger 侧不提供任何取价方法，只做分录映射与借贷平衡；出入库取价与价差拆分的入口分别是 ep-contract-inventory 的 InventoryPostingPort 与 InventoryVariancePort，由调用方在同一事务内先取得金额再作为计量项传入。

`ep-contract-ledger` 以如下 DTO 定义普通过账输入；这些类型只依赖 `ep-foundation` 与本 crate，不依赖 costing 的领域类型。

```rust
pub struct SourceDocumentRef { pub object_type: String, pub id: uuid::Uuid, pub doc_no: String }
pub struct BackdateAuthorization { pub reauth_ref: uuid::Uuid, pub approval_ref: uuid::Uuid }

pub enum CaptureKind {
    CostInventoryCogs,
    CostDirectExpense,
    CostPostingVariance(CaptureVarianceReason),
    RevenueDeliveryOrder,
    RevenueDeliveryMilestone,
    RevenueSalesReturn,
}
pub enum CaptureVarianceReason {
    EstimatePriceDiffIssued,
    PurchaseReturnDiff,
    RedLetterDiff,
    OverInvoiceToCost,
}
pub enum CaptureDetailGrain { Head, Line }
pub enum CaptureParentRequirement {
    NewRootOnly,
    ReverseCurrentLiveOnly,
    NewRootForPositiveReverseCurrentLiveForNegative,
}
pub enum CapturePolicy {
    None,
    Required {
        capture_kind: CaptureKind,
        detail_grain: CaptureDetailGrain,
        parent_requirement: CaptureParentRequirement,
    },
}
pub struct PostingDimensionSnapshot {
    pub contract_id: Option<uuid::Uuid>,
    pub sales_order_id: Option<uuid::Uuid>,
    pub sales_order_line_id: Option<uuid::Uuid>,
    pub customer_id: Option<uuid::Uuid>,
    pub project_id: Option<uuid::Uuid>,
    pub product_id: Option<uuid::Uuid>,
    pub material_id: Option<uuid::Uuid>,
    pub warehouse_id: Option<uuid::Uuid>,
}
pub struct PostingAttribution {
    pub source_document_line_id: uuid::Uuid,
    pub measure_key: MeasureKey,
    pub amount: Money, // 严格大于 0；ledger 按最终凭证腿方向决定捕获正负号
    pub capture_kind: CaptureKind,
    pub dimensions: PostingDimensionSnapshot,
    pub reverses_capture_entry_id: Option<uuid::Uuid>,
}
pub struct PostingInput {
    pub source_kind: VoucherSourceKind,
    pub posting_date: NaiveDate,
    pub backdate_authorization: Option<BackdateAuthorization>,
    pub source_document: SourceDocumentRef,
    pub source_event_id: Option<uuid::Uuid>,
    pub measures: Vec<(MeasureKey, Money)>,
    pub attributions: Vec<PostingAttribution>,
}
```

登记日由服务端事务时钟取得，不接受客户端字段。同一请求内 `MeasureKey` 必须唯一；允许有符号的差额键仍使用 `Money` 的现行有符号表示，普通非差额键由映射校验为非负。资金来源拆为 `bank_amount` 与 `cash_on_hand_amount`，要求恰一大于零；禁止用单一 `cash_amount` 让 ledger 在运行期猜科目。普通过账的 `source_sequence_no` 由 ledger 服务端恒定写 `1`，不属于 `PostingInput`、HTTP 或任何 consumer DTO；需要多凭证的同一业务动作必须使用不同的、可追溯的来源单据 id 或受控专用入口，consumer 不得伪造序号。完整枚举、每个来源的必填/可选/互斥集合与平衡方程登记在 `docs/data-dictionary/ledger.md`。

归集归因是普通过账契约的必填组成，而不是阶段 11 猜测来源行的旁路。`JOURNAL_MAP` 在每条 `JournalLeg` 上逐腿冻结 `CapturePolicy`：成本或收入角色的腿必须且只能为 `Required { capture_kind,detail_grain,parent_requirement }`，其他角色的腿必须为 `None`。调用方必须逐业务行提交 attribution。按 `(measure_key,capture_kind)` 分组后，归因金额之和须精确等于该策略所对应分录腿的计量项绝对值，类型与父引用形状须与规则一致；漏项、多项、零/负金额、错类型、错父、非成本/收入键携归因均整笔拒绝。`NewRootOnly` 强制父为空，`ReverseCurrentLiveOnly` 强制父为当前 live capture，`NewRootForPositiveReverseCurrentLiveForNegative` 对正计量强制父为空、对负计量强制当前 live 父。`source_document_line_id` 只有规则明确为 `Head` 时才可取全零 UUID，`Line` 必须为真实业务行 id。ledger 为每个 measure leg 单独生成凭证行，不跨 `measure_key` 合并；写入凭证行后，以最终借贷方向把成本借方/贷方分别转换为正/负、收入贷方/借方分别转换为正/负，再把 `voucher_id/voucher_line_id/account_id` 与归因快照交给 `CostCaptureService` 或 `RevenueCaptureService`。任一归因或捕获失败与凭证、余额、审计、Outbox 同事务回滚。

F-51 所称 `LEDGER_BACKDATE` 是应用常量名，唯一映射到 `platform_authz.permission_items` 的 `code='ledger.backdate'`、`function_point='posting_backdate'`、`object_type='ledger.voucher'`、`allowed_actions=['UPDATE']`；授权器以 `(code, UPDATE)` 判定，不新增第七种 Action。该全局权限项由阶段 4 的 permission seed 登记，阶段 9 只引用。

原有的 branch 与 reverses_voucher_id 两个字段删除。branch 是第二根全局共用的轴，与来源类型相乘得到一百三十二格而其中合法的约十六格，哪些格合法这件事不进类型只进映射表的行，靠运行期校验兜底；来源类型按分录集合拆开之后它没有剩余职责。reverses_voucher_id 由第 9.5.9 节的 post_reversal 承担；销项与进项红字冲销各走自己的来源类型经映射生成，其追溯链落在单据侧，即红字发票指向原发票，凭证侧不再第二次表达，ledger.vouchers.reverses_id 与 ledger.voucher_lines.reverses_id 两列此后只由 post_reversal 写入。

ep-domain-ledger 定义编译期常量 `rule::journal_map::JOURNAL_MAP: &[JournalRule]`。精确形状为 `JournalRule { source_kind,measure_key,requiredness,legs: &'static [JournalLeg] }` 与 `JournalLeg { account_role,direction,capture_policy: CapturePolicy }`；这三个字段和策略枚举不得在实现侧另建短版。数据库和 CI 强制每个 `(source_kind, measure_key)` 只有一条规则，每条规则含 1..n 个分录腿；一个计量项的借贷两腿必须处于同一 `legs` 数组，不得用重复键表达。完整规则以 `docs/data-dictionary/ledger.md` 的冻结表为唯一机器实现输入。

映射算法固定为四步。

第一步，先拒绝请求内重复 `MeasureKey`，再按 `source_kind` 取出全部 `JournalRule`；出现未登记计量项、缺失必填计量项、违反互斥组或平衡方程，统一返回 VALIDATION 与 `LEDGER.POSTING.MEASURE_INVALID`。控制总额若不产生分录不得放入 `measures`。

第二步，符号归一。计量项金额允许为负，正值按表中方向入账，负值取绝对值按相反方向入账。该规则使 numeric(18,2) 的 amount 列恒为正、方向恒为二值，同时不改变任何净额。金额为零的计量项不生成分录行。

第三步，逐规则展开 `legs`，把 `AccountRole` 经该法人的 `event_account_bindings` 解析为 `account_id`；角色未绑定返回 BUSINESS_CONFLICT 与 `LEDGER.EVENT_ACCOUNT_BINDING.ROLE_UNBOUND`，绑定到已停用科目返回 `LEDGER.EVENT_ACCOUNT_BINDING.ACCOUNT_INACTIVE`，绑定到存在下级科目的一级科目返回 `LEDGER.ACCOUNT.NOT_POSTABLE`。同一角色在一次映射中多次出现时不合并行，逐条保留 `measure_key` 追溯。

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
| OPEN | CLOSED | 关账请求的关账前强制校验通过 | 同一提交图中存在唯一 `status=PASSED/conclusion=PASSED` 请求，且双向长 FK 把其 id、期间 id、concluded_at 与本期间 closed_by/closed_at 逐值锁定 |
| CLOSED | 无 | 首版不做反结账 | 无入口 |

可入账期间是派生条件，不是第三种状态：status = OPEN 且不存在 accounting_period_id 指向它、status 属于 ACCEPTED 或 VALIDATING 的关账请求。

期间归属解析算法 resolve_accounting_period(le, posting_date)，在业务事务内执行，输出 ResolvedPeriod。该类型定义在 ep-contract-ledger，三个业务字段私有，consumer 只能经只读 getter 取得 accounting_period_id、accounting_period_seq 与 deferred_from_period_id；唯一构造入口 `from_resolver_parts` 是给 `ep-app-ledger` 实现 resolver 使用的隐藏 implementation SPI，`xtask archcheck ledger-resolved-period-construction` 固定断言全工作区只有 `crates/application/ledger/` 可以调用，其他模块直接构造即失败。accounting_period_seq 由该期间的 fiscal_year 与 period_no 导出为该法人内的单调序号，不新增列。原先由各调用方自行计算并传入 accounting_period_seq 的做法取消，理由是十六项对账一律按 accounting_period_id 比金额，seq 写错时对账全绿，而收发存汇总与期末库存价值表按 seq 取数会静默错期，且其承载表是仅追加表不可更正。

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
| PENDING_APPROVAL | ACCEPTANCE_REFUSED | 审批通过后受理前提任一不成立 | 见下文两项前提；写 approved_by 与非空 refusal_reasons，不占 slot |
| PENDING_APPROVAL | ACCEPTED | 审批通过且两项前提全部成立 | slot 行锁内写 approved_by/accepted_at，并让 request↔slot 双向证据同时成立 |
| ACCEPTED | VALIDATING | 在途写事务等待结束且快照建立成功 | inflight_xids 全部完成 |
| ACCEPTED 或 VALIDATING | CANCELLED | 独立主动取消审批通过后的受控回调 | conclusion 原为空；写 cancellation_reauth_ref/cancellation_approval_ref/cancelled_by，保留原请求 reauth/approval/approved_by；同一事务解除本请求的 slot 双向证据 |
| PENDING_APPROVAL | CANCELLED | 独立主动取消审批通过后的受控回调 | 原关账审批尚无结论也不构成豁免；仍须写三项独立取消证据，原 reauth/approval_ref 不变，approved_by/accepted/wait/snapshot 仍为空且不占 slot |
| VALIDATING | PASSED | 全部校验项通过 | 同一快照内；同一事务关闭该期间、写相同 concluded/closed 时点并释放 slot |
| VALIDATING | FAILED_DISCREPANCY | 任一校验项差额非零 | 同一快照内；期间保持 OPEN并释放 slot |
| VALIDATING | FAILED_INCOMPLETE | 五类终止成因之一 | 同一快照内；期间保持 OPEN并释放 slot |

受理前提逐项判定，全部成立才受理。

前提一：该期间 status = OPEN；该期间是该法人 start_date 最小的 OPEN 期间；该法人的 close_serialization_slots.active_close_request_id 为空。

前提二：该法人该期间内，platform_msg.outbox_events 中 status 属于 PENDING 或 DISPATCHING、posting_date 落在该期间起止之间、且 event_type 命中 ledger.posting_trigger_event_types 的条目数为零，且 platform_msg.dead_letters 中 state 属于 OPEN 或 REPAIRING、同样命中该注册表的条数为零。posting_date 为空的平台事件一律不计入。该判定的可枚举依据为 ledger.v_pending_posting_backlog，两侧不为零时分别按 LEDGER.PERIOD_CLOSE_REQUEST.PENDING_POSTING_BACKLOG 与 LEDGER.PERIOD_CLOSE_REQUEST.UNREPAIRED_DEAD_LETTERS 载明。

任一不成立时置 ACCEPTANCE_REFUSED，把未满足项与其当前取值写入 refusal_reasons，经 ep-platform-recon 与 ep-platform-obs 的端口生成关账受理被拒事项并记入运维中心，期间状态不变，其过账、查询与报表不受影响，发起次数不设上限。同一法人同一期间连续两次受理被拒时按规格第 15.3 章告警，并按 A-26 经 ep-platform-obs 的 DegradationLedger::open 登记暴露窗口，至该期间完成关账时经 DegradationLedger::close 关闭。本阶段只调用该台账，不自建第二套窗口表；DegradationKind 的取值由阶段 2 定义并由后续阶段扩展，本阶段不复述其取值数。

年度末次期间的损益归零不在受理前判定，由关账前强制校验在同一快照上判定。

九态是唯一状态集合；在途集合登记、等待与快照导出前不增设 WAITING_INFLIGHT/SNAPSHOTTING 影子状态，请求从 ACCEPTED 一直保持到快照建立成功后一次转 VALIDATING。因此 generated active_slot_key 恰覆盖 ACCEPTED/VALIDATING。所有结论事务以一个数据库事务更新 request/period/slot；三行与各证据字段可用任意 SQL 语句顺序写入，只有 legal transition guard 即时判边，最终状态形状与双向关系统一由第 9.3.9.1 节延迟图判定。

#### 9.4.6 受理与在途写事务等待的次序

这是本阶段最需要严密论证的一处。次序固定为四步，任一步顺序颠倒即产生正确性缺口。

第一步，受理事务 T1：先确保 close_serialization_slots 该法人唯一行存在，再执行 SELECT ... FOR UPDATE，判定两项前提；通过时写请求 ACCEPTED/approved_by/accepted_at 与 slot.active_close_request_id，使双向 active evidence 在提交点成立，写 `ledger.period_close.accepted.v1` 的 Outbox 条目，最后写审计终结批并提交。请求与 slot 两条 UPDATE 顺序不影响提交；漏任一侧由延迟图整笔拒绝。

第二步，在途集合登记事务 T2：T1 提交之后立即开启，执行 `SELECT pg_snapshot_xip(pg_current_snapshot())`，把结果去掉自身写入 `period_close_requests.inflight_xids`，提交。其正确性前提不是 PostgreSQL 会自动给所有活跃事务分配 XID，而是第 9.5.9 节 exact resolver 强制每条可过账事务在读取期间前先执行 `SELECT pg_current_xact_id()`；没有顶层 XID 的只读事务本来不会出现在 xip，因此该语句是本算法不可删除的一部分。

必须是先提交 T1 再取快照，理由如下。任何一次期间归属解析先取得顶层 XID，再在后续语句快照上读 accounting_periods 与 period_close_requests；隔离级别为 READ COMMITTED，每条语句取新快照。若期间读取发生在 T1 提交之后，它一定看到 ACCEPTED，因此该期间不再是可入账期间，其凭证顺延到后续期间。若期间读取发生在 T1 提交之前，其已分配 XID 的事务在 T2 取快照时要么仍在途，从而落入 inflight_xids 并被第三步等待覆盖；要么已提交，从而其凭证在第四步的快照中可见。三种情形穷尽，因此受理之后不再产生落入该期间的新凭证，且快照覆盖该法人该期间的全部凭证。任何绕开 AccountingPeriodResolver 的过账写入同时由 archcheck 与期间/凭证图触发器拒绝，不能形成第四种情形。

第三步，等待：按配置的轮询间隔重复取 pg_current_snapshot()，对 inflight_xids 中每个 xid 判定 xid 小于 pg_snapshot_xmax(current) 且不在 pg_snapshot_xip(current) 中，全部成立即等待结束，写 inflight_wait_completed_at。该判定不依赖 pg_stat_activity，因此不需要给运行期账号授予 pg_read_all_stats 或 pg_monitor，不放大运行期账号权限。该集合在 T2 一次取定，此后只减不增，每笔事务终将提交或回滚，因此等待必然结束，不设时限、不自动解除。等待期间不冻结任何写入。等待超过 ledger.close.inflight_wait_warn_seconds 时只告警不终止。

第四步，快照建立：在 job-worker 池上开启一个 REPEATABLE READ 只读事务，执行 SELECT pg_export_snapshot() 取得 snapshot_id 并保持该事务打开；各批工作连接在自身事务开始时执行 SET TRANSACTION SNAPSHOT '<snapshot_id>'。在另一条连接上把请求置为 VALIDATING 并写 snapshot_established_at，不在快照事务内写，理由是快照事务须保持只读且长期打开；slot 继续指向该请求且 generated active_slot_key 仍为 1。按 A-01 与 C-03，只读快照事务的唯一入口是 ep-foundation 的 UnitOfWork::snapshot_transact，其向执行体传入的 SnapshotCtx 的 snapshot_id() 即 pg_export_snapshot 的返回值、taken_at() 即 snapshot_established_at，逐批传递的就是该 SnapshotCtx。

#### 9.4.7 关账前强制校验的编排

ep-platform-recon 的本体按 A-06 由本阶段 9a 段提供：crate、platform_core 下的三张表、ReconCheck 与 ReconRegistry 与 ReconExecutor 三个契约、BatchWindow 与 ReconRunOutcome、快照传递、单批时限、单查询内存与临时空间上限、差异事项与校验未完成事项的模型，以及 job-worker 内的每日对账调度，全部在本阶段落地。执行器按基线第 3.8 节逐法人遍历，法人清单取 ep-platform-tenancy 的 LegalEntityDirectory::list_active，每轮只在单一法人上设置 app.legal_entity_id，快照经 UnitOfWork::snapshot_transact 导出并以 SnapshotCtx 逐批传递。本阶段自带的四类校验项各实现一个 ReconCheck 并在 job-worker 的 wiring 中经 ReconRegistry::register 注册，实现与注册同属 9b 段。内部对账不另定义第二个上下文类型：`ReconExecutor` 是除枚举定义处外唯一出现 `SystemPurpose::Reconciliation` 的文件，在每轮法人开始时调用 `SecurityContext::system(legal_entity_id, request_id, trace_id, SystemPurpose::Reconciliation)`，任务结束与连接归还前销毁；job-worker wiring 只取得并调用 `ReconExecutor::run`，不暴露上下文构造入口。`reconciliation-context-confined` archcheck 对其他出现点构建失败；对账仓储在取连接前再校验 `AccountKind::System + Some(Reconciliation)`，失败返回 `PLATFORM.AUTHZ.OBJECT_FORBIDDEN` 且不取连接。规格第 7.7 章所要的这条路径不构成越权通道，其保证改为静态封闭性，不再用签名语句集：ReconExecutor 只按 ReconRegistry 中已注册的 ReconCheck 实现分发，不接受任何语句文本入参，校验语句一律是各 ep-app-* crate 内的编译期常量，完整性由阶段 1 的制品签名链与客户侧验签承担，并由 archcheck 断言 ep-platform-recon 与各 ReconCheck 实现体内不出现字符串拼接 SQL 与动态语句执行入口。platform_core.recon_check_definitions 的 statement_sha256 与 signed_statement_ref 两列删除，每次运行改写制品版本号与制品签名摘要到 recon_runs 与审计事件，外部审计问某次关账跑的是哪一版校验由这两项唯一回答。唯一保留为硬约束的是输出边界：recon_discrepancies.subject_ref 的允许键固定为勾稽项标识、法人、会计期间、凭证号、仓库、物料、批次、科目、单据编号与账户内部标识十项，写入时校验，出现白名单以外的键直接拒绝并按规格第 15.3 章告警。blocks_period_close 返回真的校验项即 is_blocking_period_close 为真的登记项，构成关账前强制校验的范围。

本阶段自带并注册的校验项四类。

一是会计借贷平衡：逐张凭证核对 total_debit_amount 等于 total_credit_amount 且等于其分录行按方向的合计；按该法人该期间核对借贷合计相等。差额非零生成勾稽类差异事项，载明勾稽项、法人、会计期间、子账侧金额、总账侧金额与差额。

二是年度末次期间损益逐科目归零：只在 is_fiscal_year_last 为真的期间执行，在同一快照上要求该期间 `category=PROFIT_LOSS` 的每个科目期末余额都等于零，即非零科目数必须为零；只看借正贷负净合计为零不通过，因为多个非零科目可互相抵消。每个非零科目生成一条差异事项，载明法人、会计期间、account_id 与该科目 signed 期末余额，不载明子账侧金额，与勾稽类差异事项区分。年中期间不设该要求。

三是总账侧余额提供者：ep-contract-ledger 暴露 TotalAccountBalanceProvider trait，按 (法人, 会计期间, AccountRole) 返回该科目在快照上的余额。子账与总账勾稽的比较由本阶段的 ReconExecutor 驱动，子账侧一律经 ep-contract-finance 的 ReconciliationItemQuery 按法人与会计期间取十项勾稽的子账侧合计，结构为 ReconciliationItemView，该 trait 由阶段 10 定义；阶段 8 在 ep-contract-inventory 定义的 StockValueSubledgerBalancePort 与阶段 7 在 ep-contract-procure 定义的 GrniSubledgerBalancePort 是阶段 10 内部组装该结果的手段，本阶段的 ReconCheck 不直接调用它们。十项中的存货与已收货未收票两项，其子账侧实现体分别由阶段 8 的 InventorySubledgerBalanceQuery 与阶段 7 的 GrniSubledgerBalanceQuery 各自在本模块 contract 的端口上实现，阶段 10 只注入，其余八项取自阶段 10 自有表、不经这两个端口。本阶段不定义总账侧接口之外的任何东西。

四是科目余额一致性：核对 account_period_balances 的本期发生额与按 voucher_lines 在同一期间的聚合相等，期初已固化的核对与上一期间期末相等。该项是本阶段引入增量余额表所必须的自检，属本阶段新增决定。

校验未完成一律按未通过处理：单批执行时限触发终止、单查询内存或临时空间上限触发终止、执行进程异常退出、连接被回收、快照失效五类之一发生时，置 FAILED_INCOMPLETE，写 termination_cause 与 completed_batch_count，生成校验未完成事项，按规格第 15.3 章即时告警，并按 A-26 经 ep-platform-obs 的 DegradationLedger::open 登记降级与暴露窗口、成因解除后经 close 关闭，该次关账请求结束，期间保持打开。不得按通过处理，也不得以未生成勾稽类差异事项为由放行。

#### 9.4.8 年度损益结转

前提：该期间 is_fiscal_year_last 为真；该期间为可入账期间；发起人具备 ledger.year_end_closing.request；按规格第 12.1 章财务过账一类完成重新认证；按第 12.2 章完成审批且申请人不可自审。本法人 close_serialization_slots.active_close_request_id 非空时，不论该 active request 指向本期间还是另一期间都不执行，返回 BUSINESS_CONFLICT 与 LEDGER.YEAR_END_CLOSING.PERIOD_NOT_POSTABLE。

算法五步。第一步，先确保并 `FOR UPDATE` 锁住该法人 close_serialization_slots 唯一行，再锁目标期间，随后按 account_id 固定顺序锁定执行时点该期间全部 category=PROFIT_LOSS 的余额行及 PROFIT_THIS_YEAR 余额行；此锁序与关账受理/结论共享同一 slot 串行化点，锁持有至提交，故年结通过检查后不可能并发新受理一个关账请求。锁内重新验证期间可入账、末期与角色绑定，计算并冻结 `profit_loss_nonzero_account_count_before` 和借正贷负口径的 `profit_loss_net_balance_before_amount`，调用方不得提交。第二步，count>0 时生成第一张结转凭证：source_kind=YEAR_END_PL_CLOSING、source_document_type=YEAR_END_CLOSING、source_document_id/no 为本 closing、source_sequence_no=1，把每个非零损益科目完整反向清零；只有 net 非零时才增加一条 PROFIT_THIS_YEAR 平衡腿。count=0 时不生成第一张。第三步，net 非零时生成第二张 source_sequence_no=2 的两腿凭证，以绝对额反向清零 PROFIT_THIS_YEAR 并转入 RETAINED_EARNINGS_UNDISTRIBUTED；net=0 时不生成第二张，即使 count>0 也不生成零金额凭证。第四步，所有存在的凭证 business_date 固定为期间 end_date，accounting_period_id 固定为本期间，deferred/source_event/reverses 全空。第五步，写 EXECUTED、executed_at、两个控制字段与条件存在的凭证引用；第 9.3.10.1 节延迟图从最终凭证腿和余额反推执行前控制值，任一不等整笔回滚。

可重复执行：每次执行创建一行新的 year_end_closings，sequence_no 递增，因此 ux_vouchers 的四列唯一键不冲突。本次结转之后又有凭证落入该期间时可再执行一次。

边界条件分三形：count=0 时 net 必为 0、两张凭证均空；count>0 且 net=0 时仍必须生成逐科目清零的第一张，第二张为空；count>0 且 net 非零时两张均存在且 id 不同。第一张是否存在只由非零损益科目数决定，不能只看可被多个科目互相抵消的净额。该期间关账之后到达、记账日期属于该年度的业务事件按顺延记入下一年度期间，其损益进入下一年度的本年利润，首版不做追溯重述。

APPROVED 后执行时若锁后发现期间已不可入账或本法人 slot 非空，置 FAILED、failure_code=PERIOD_NOT_POSTABLE；只有期间仍为 OPEN、slot 为空而任一必需角色未绑定时才置 FAILED、failure_code=ROLE_UNBOUND。两支都写 concluded_at=服务器时点且不写凭证、余额或 executed_at；其它基础设施/事务异常回滚并保持 APPROVED，由 worker 原任务重试或死信，不把基础设施原因塞入 failure_code。NOT_FISCAL_YEAR_LAST_PERIOD 在创建前拒绝，数据库末期图也使其无法作为已建单终态存在。

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
| PATCH /api/v1/ledger/accounts/{id} | 修改 name 与 is_active。请求体必带 row_version。category、balance_direction、parent_account_id 在该科目已产生凭证后不可改，按 U-H-03 冻结取值 |
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

手工录入与规格第 7.10 章迁移批次导入两条路径的互斥按 U-H-04 冻结取值：手工录入只允许在该法人尚无任何凭证且尚无已确认的迁移批次期初时执行，要求借贷合计平衡，需审批。
按 A-24，首版不设独立的数据迁移阶段，本节的 POST /api/v1/ledger/opening-balance-batches 与 /{id}/actions/confirm 是总账期初余额的唯一落点。应收应付预收预付期初与资金账户期初归阶段 10，库存期初归阶段 8 的 MIGRATION_STOCK_ADJUSTMENT 来源类型。四个通道的写入一律不生成凭证，期初对应的总账侧由本节的期初余额批次承担，两侧的平衡由 finance 的勾稽视图在首个会计期间校验。


#### 9.5.3 事件科目对应关系

| 方法与路径 | 说明 |
|---|---|
| GET /api/v1/ledger/event-account-bindings | 列出 17 个角色及其当前绑定，未绑定的角色以 account_id 为 null 呈现 |
| PUT /api/v1/ledger/event-account-bindings/{account_role} | 绑定或改绑。请求体 account_id 与 row_version。权限 ledger.event_account_binding.manage |
| POST /api/v1/ledger/event-account-bindings/actions/check-completeness | 返回未绑定角色清单与绑定到停用科目的角色清单，供建账验收与运维中心查看，不供任何启动自检使用 |

变更经 ep-platform-release 的配置发布通道发布，按基线第 7.1 节运行期可变业务参数的口径。该通道按 A-27 由阶段 3a 交付端口、阶段 3b 交付最小发布通道，本阶段只作为使用方接入，不自建第二套发布路径。F-51 已确认 U-H-06 的冻结值：需 `FINANCE_MANAGER` 审批，不额外重新认证。变更对已生成凭证无影响，因凭证行固化 account_id。

#### 9.5.4 凭证查询

| 方法与路径 | 说明 |
|---|---|
| GET /api/v1/ledger/vouchers | 列表。过滤白名单 accounting_period_id、business_date、doc_no、source_kind、source_document_no、account_id、amount 区间、deferred_from_period_id、created_by。business_date 与 accounting_period_id 是规格要求的两条检索路径，其余按 U-H-09 冻结取值提供 |
| GET /api/v1/ledger/vouchers/{id} | 详情，含分录行、来源单据引用、两个日期与顺延标注 |
| GET /api/v1/ledger/vouchers/{id}/lines | 分录行 |

响应中每张凭证一律带 accounting_period_id、business_date 与 is_deferred 三项，is_deferred 由 deferred_from_period_id 非空导出；按原始业务日期检索时结果标注该凭证实际落入的会计期间。按 U-H-10 冻结取值，过账接口的响应也回带这三项，使提交回执可即时告知顺延。

F-50 另增加以下三个受控更正凭证端点，计入本阶段 34 个 HTTP 端点：

| 方法与路径 | 说明 |
|---|---|
| POST /api/v1/ledger/correction-vouchers | 引用一张已过账原凭证，提交原因和逐原凭证行的受控更正意图；必带 `Idempotency-Key` 与 `X-Reauth-Token`，不得提交自由科目或任意借贷行；只返回 `202 ApprovalSubmission { approval_ref,scenario=LEDGER_CORRECTION_VOUCHER,requested_action=POST,status=PENDING_APPROVAL }`，此时不得创建 CORR 单据、号码、余额、归集条目或凭证 |
| GET /api/v1/ledger/correction-vouchers | 按来源凭证、记账期间、日期和发起人查询，默认 `created_at desc, id desc` |
| GET /api/v1/ledger/correction-vouchers/{id} | 返回更正单、受控行、原凭证引用与生成凭证引用；无权或不存在统一返回 `PLATFORM.AUTHZ.NOT_FOUND_OR_DENIED` |

本入口只处理“源业务事实与资金事实都正确、仅科目归类错误”。发票事实错误走作废/红字，资金事实错误走资金单据冲正；无来源自由分录返回 `LEDGER.CORRECTION_VOUCHER.ENTRY_NOT_ALLOWED`。

更正凭证固定复用阶段 3 的 `platform_flow.approval_command_snapshots`，不得给仅追加终态表增加伪 DRAFT。出厂审批场景码为 `LEDGER_CORRECTION_VOUCHER`，默认单节点 `FINANCE_MANAGER`；链缺失、并列 active 版本、节点为空或节点展开为空均 fail-closed，申请人不可自审。提交事务先校验权限、重新认证证据、来源凭证可见且已过账、请求形状和静态角色白名单，再把下列 V1 payload 以法人 FIELD/密级 30 密钥做 AES-256-GCM 信封加密；明文只存在于有界内存，流程 `variables` 只放 owner/scenario/action/snapshot id 和来源凭证 id。提交只创建流程实例、任务和密文快照并完成 HTTP 幂等记录，返回 202；不得预占 CORR 号码或写 ledger/costing 业务表。

```rust
pub struct LedgerCorrectionCaptureAllocationV1 {
    pub source_capture_entry_id: uuid::Uuid, // 只能是 costing.cost_entries 当前 live 条目
    pub amount: Money,                        // > 0
}
pub struct LedgerCorrectionApprovalLineV1 {
    pub source_voucher_line_id: uuid::Uuid,
    pub target_account_role: AccountRole,
    pub amount: Money,                 // > 0
    pub memo: Option<String>,          // 清洗后 <= 500
    pub source_capture_allocations: Vec<LedgerCorrectionCaptureAllocationV1>,
    // 非空、entry id 唯一、合计严格等于 amount
}
pub struct LedgerCorrectionApprovalPayloadV1 {
    pub source_voucher_id: uuid::Uuid,
    pub reason: String,                // 清洗后 1..=500
    pub posting_date: chrono::NaiveDate,
    pub reauth_ref: uuid::Uuid,
    pub requester_user_id: uuid::Uuid,
    pub requester_device_id: String,
    pub lines: Vec<LedgerCorrectionApprovalLineV1>, // 1..=200，source line 唯一
}
pub struct LedgerApprovalCommandEnvelopeV1 {
    pub schema_version: u16,           // 恒为 1
    pub scenario: ApprovalScenarioCode, // LEDGER_CORRECTION_VOUCHER
    pub action: String,                // 恒为 "POST"
    pub legal_entity_id: Id<LegalEntity>,
    pub idempotency_key: uuid::Uuid,
    pub payload: LedgerCorrectionApprovalPayloadV1,
}
```

提交时先经 costing 查询同一 `source_voucher_line_id` 的可见当前 live 成本条目：只有一个候选时服务端可把整行 amount 规范化为这一条 allocation；存在多个候选时请求必须逐项明确分配，禁止按 UUID、比例或“当前余额”暗自猜测。规范化后的 allocations 必须非空、id 唯一、全属该原凭证行且合计严格等于行 amount；审批详情逐项展示来源业务行与合同/订单/项目/产品等维度摘要。完整 allocations 连同其显示摘要哈希进入上述加密 payload、请求摘要与审批待签内容；审计只保存 entry id、金额和维度摘要哈希，不保存额外敏感明文。

审批通过回调是唯一过账入口。它在新事务锁定流程实例与 snapshot，校验同法人、scenario/action/schema、密文摘要、流程确已 APPROVED、最终审批人不是 requester、reauth 仍对应相同请求摘要；解密一次后重新锁读原凭证/原行及 payload 中逐字相同的成本 allocation，重验它们仍为同凭证行的当前 live 条目且开放额足够，绝不重算、换 entry id 或把金额自动改投另一来源行/维度。随后重新解析期间和补记授权，生成 CORR id/号码并构造第 9.5.9 节 `CorrectionPostingInput`，其中 allocations 逐项复制自批准 payload、`approval_ref=process_instance_id`、`source_event_id=None`。只调用一次 `post_correction`；只有 `Posted` 合法，`IdempotentReplay|Skipped` 在 snapshot 仍 PENDING 时均视为孤立图并整笔失败。终态更正头行、生成凭证、余额、归集条目、snapshot `CONSUMED + result_object_type='LEDGER_CORRECTION_VOUCHER'/result_object_id/result_doc_no`、幂等 finish、Outbox、通知与最终审计按统一 audit-last 后缀同事务提交。任一版本、金额、角色、allocation 开放额、期间、认证或摘要变化均保持 snapshot PENDING，返回稳定冲突并要求按当前事实重新提交，零 ledger/costing 写；驳回/撤回/过期只把 snapshot 置 `REJECTED|EXPIRED`，结果引用为空。相同回调重放读取 CONSUMED 结果引用返回既有更正视图，不再过账。

界面不提供凭证修改与删除入口；对 vouchers 与 voucher_lines 的任何写请求返回 PERMISSION_DENIED 与 LEDGER.VOUCHER.IMMUTABLE。

#### 9.5.5 账表与试算平衡

| 方法与路径 | 说明 |
|---|---|
| GET /api/v1/ledger/account-balances | 科目余额表。必填 accounting_period_id。输出按科目的期初余额、本期借方发生额、本期贷方发生额、期末余额。对应规格附录 A.1 的月度科目余额表 |
| GET /api/v1/ledger/general-ledger | 总账。必填 accounting_period_id，可选 account_id。按科目按期间给出发生额与余额 |
| GET /api/v1/ledger/subsidiary-ledger | 明细账。必填 account_id 与 accounting_period_id 或 business_date 区间之一，下钻到逐笔分录行，每行同时展示会计期间字段与原始业务日期，并带凭证与来源单据引用。超过 10000 行深偏移时按基线第 5.3 节切换为键集分页 |
| GET /api/v1/ledger/trial-balance | 试算平衡。必填 accounting_period_id。输出借方合计、贷方合计与差额，差额为零即通过。按 U-H-11 冻结取值分期初、发生额、期末三段各给一对合计 |
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
| GET /api/v1/ledger/period-close-requests/{id} | 详情。含 refusal_reasons、completed_batch_count、termination_cause、四种结束方式的结论与关联事项引用；CANCELLED 另回带 cancellation_reauth_ref、cancellation_approval_ref、cancelled_by，cancelled_at 取 concluded_at |
| POST /api/v1/ledger/period-close-requests/{id}/actions/cancel | 提交独立主动取消审批。必带 `Idempotency-Key` 与 X-Reauth-Token，守卫为 conclusion 为空；返回 `202 ApprovalSubmission`，此时 request/status/slot 均不变，批准回调才写 CANCELLED。取消的 reauth/approval 不得复用原关账申请证据 |

审批动作本身由 ep-platform-flow 的通用审批端点承载，本阶段不新建审批端点。

同步等待上限按基线第 11.6 节 8 秒，关账为后台任务，发起端点立即返回请求回执，进度经 GET 详情与站内通知回执可见。关账窗口不预设固定上限，界面不得给出固定完成时限的承诺。

错误码：LEDGER.PERIOD_CLOSE_REQUEST.PERIOD_ALREADY_CLOSED、LEDGER.PERIOD_CLOSE_REQUEST.NOT_EARLIEST_OPEN_PERIOD、LEDGER.PERIOD_CLOSE_REQUEST.ANOTHER_REQUEST_IN_PROGRESS、LEDGER.PERIOD_CLOSE_REQUEST.PENDING_POSTING_BACKLOG、LEDGER.PERIOD_CLOSE_REQUEST.UNREPAIRED_DEAD_LETTERS、LEDGER.PERIOD_CLOSE_REQUEST.VALIDATION_DISCREPANCY、LEDGER.PERIOD_CLOSE_REQUEST.VALIDATION_INCOMPLETE、LEDGER.PERIOD_CLOSE_REQUEST.CONCLUSION_ALREADY_PRODUCED、`PLATFORM.SOD.SELF_APPROVAL_FORBIDDEN`、`PLATFORM.AUTHZ.REAUTH_REQUIRED`。前五个是受理被拒原因并写入 refusal_reasons；第六、七个分别对应两类校验终态，第八个只用于取消/结论竞态，三者不得伪装为受理拒绝。八个 ledger 码均为 BUSINESS_CONFLICT；重新认证与自审拒绝传播平台码，不得再登记 ledger 私有别名。

#### 9.5.8 年度损益结转

| 方法与路径 | 说明 |
|---|---|
| POST /api/v1/ledger/year-end-closings | 发起。必带 X-Reauth-Token。请求体 accounting_period_id。权限 ledger.year_end_closing.request |
| GET /api/v1/ledger/year-end-closings | 列表，展示历次结转的五态、sequence_no、executed_at 或 failure_code/concluded_at |
| GET /api/v1/ledger/year-end-closings/{id} | 详情；EXECUTED 回带两个执行前控制字段与 0/1/2 张条件凭证引用，FAILED 回带闭集 failure_code/concluded_at，其他状态的两组终态字段为空 |

错误码：LEDGER.YEAR_END_CLOSING.NOT_FISCAL_YEAR_LAST_PERIOD、LEDGER.YEAR_END_CLOSING.PERIOD_NOT_POSTABLE、LEDGER.YEAR_END_CLOSING.ROLE_UNBOUND。F-51 已确认 U-H-15 的冻结值：年结使用独立审批链，并与期末结账同属财务过账类高风险操作。发起事务在请求、重新认证/审批引用与审计原子落库时写 `ledger.year_end_closing.requested.v1`；执行事务只有在第 9.3.10.1 节三种 EXECUTED 形状之一完整提交时才写 `ledger.year_end_closing.executed.v1`，payload 携两个执行前控制字段，两个 voucher id 均按条件可空。FAILED 不伪装 executed 事件，只写终态审计/通知与既有错误码；两类事件均不得以未命名 Outbox 条目代替。

#### 9.5.9 模块内契约

ep-contract-ledger 暴露给其他模块的 trait 完整首版为五个，均不经 HTTP；前三个由 9a 初始交付，后两个由 F-50 Task 1A 在不改变 crate 边界的前提下同批追加。

AccountingPeriodResolver 与 PostingOutcome 的 exact ABI 冻结如下；这是全卷唯一可抄定义，其他阶段只能引用，不能省略字段或另写同名简化类型。

```rust
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ResolvedPeriod {
    accounting_period_id: Id<AccountingPeriod>,
    accounting_period_seq: i32,
    deferred_from_period_id: Option<Id<AccountingPeriod>>,
}
impl ResolvedPeriod {
    #[doc(hidden)]
    pub fn from_resolver_parts(
        accounting_period_id: Id<AccountingPeriod>,
        accounting_period_seq: i32,
        deferred_from_period_id: Option<Id<AccountingPeriod>>,
    ) -> Self {
        Self { accounting_period_id, accounting_period_seq, deferred_from_period_id }
    }
    pub fn accounting_period_id(&self) -> Id<AccountingPeriod> {
        self.accounting_period_id.clone()
    }
    pub fn accounting_period_seq(&self) -> i32 { self.accounting_period_seq }
    pub fn deferred_from_period_id(&self) -> Option<Id<AccountingPeriod>> {
        self.deferred_from_period_id.clone()
    }
}

#[async_trait::async_trait]
pub trait AccountingPeriodResolver: Send + Sync {
    async fn resolve(
        &self,
        tx: &mut dyn Tx,
        ctx: &SecurityContext,
        legal_entity_id: Id<LegalEntity>,
        posting_date: NaiveDate,
    ) -> Result<ResolvedPeriod, AppError>;
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum PostingOutcome {
    Posted {
        voucher_id: uuid::Uuid,
        doc_no: String,
        accounting_period_id: Id<AccountingPeriod>,
        deferred_from_period_id: Option<Id<AccountingPeriod>>,
    },
    IdempotentReplay {
        voucher_id: uuid::Uuid,
        doc_no: String,
        accounting_period_id: Id<AccountingPeriod>,
        deferred_from_period_id: Option<Id<AccountingPeriod>>,
    },
    Skipped {
        accounting_period_id: Id<AccountingPeriod>,
        deferred_from_period_id: Option<Id<AccountingPeriod>>,
    },
}
```

`resolve` 在同一 `&mut dyn Tx` 内记忆化，同一事务内重复调用返回逐字段相等的值。首次非记忆化调用的第一条数据库语句固定为 `SELECT pg_current_xact_id()`，强制给该顶层事务分配 XID，然后才读取 `accounting_periods/period_close_requests`；不得以“当前只读”省略，也不得在读取期间后补调。这样任何已看到旧 OPEN 状态但尚未写凭证的过账事务都会进入后续 `pg_snapshot_xip` 集合。重复调用只读事务内 memo，不再次执行该语句。`from_resolver_parts` 虽因实现 crate 边界必须为 public，仍不是 consumer API；除 `ep-app-ledger` 外调用它由上述 archcheck 直接拒绝。三个 getter 返回拥有值，调用方不得从日期、Clock 或本地 SQL 另算 id/seq。

PostingPort：`post(tx, ctx, PostingInput)` 返回上述 `PostingOutcome`，并在内部把 `source_sequence_no=1` 写入凭证候选键。同一 `(legal_entity_id, source_kind, source_document_id, 1)` 已存在非零凭证时返回 `IdempotentReplay`，不重复写余额、审计或 Outbox；首次调用却得到 `IdempotentReplay`，或 owner 单据尚未终态却命中孤立凭证，均按内部不变量整事务失败。`Skipped` 只允许所有经规则校验后的会计效果绝对值均为零，仍回带实际期间与顺延来源，但不生成 voucher/doc_no/余额/ledger Outbox；owner 必须把 voucher 引用建模为可空且仅在其全部会计效果为零时接受 `Skipped`，业务终态、子账事实、owner Outbox 与幂等结果照常同事务落库，后续重放由 owner 的幂等终态返回而不得再次调用 PostingPort。

同一 trait 另有 `post_reversal(tx, ctx, source: CashDocumentRef, split: CashReversalPostingSplit)` 与 `post_correction(tx, ctx, input: CorrectionPostingInput)`，二者同样返回 `PostingOutcome`。精确 DTO 如下。

```rust
pub enum OriginalCashDocumentType { Receipt, Payment, CustomerRefund, SupplierRefund }
pub struct CashDocumentRef {
    pub reversal: SourceDocumentRef, // object_type 必须为 CASH_DOCUMENT_REVERSAL
    pub original_doc_type: OriginalCashDocumentType,
    pub original_doc_id: uuid::Uuid,
    pub original_voucher_id: uuid::Uuid,
    pub posting_date: NaiveDate,
    pub backdate_authorization: Option<BackdateAuthorization>,
    pub source_event_id: Option<uuid::Uuid>,
}
pub struct CashReversalPostingSplit { pub ar_ap_amount: Money, pub advance_amount: Money }

pub struct CorrectionCaptureAllocation {
    pub source_capture_entry_id: uuid::Uuid,
    pub amount: Money, // 严格大于 0；首版只能引用 costing.cost_entries
}
pub struct CorrectionReclassification {
    pub source_voucher_line_id: uuid::Uuid,
    pub target_account_role: AccountRole,
    pub amount: Money,
    pub memo: Option<String>,
    pub source_capture_allocations: Vec<CorrectionCaptureAllocation>,
}
pub struct CorrectionPostingInput {
    pub correction: SourceDocumentRef, // object_type=CORRECTION_VOUCHER，id/doc_no 为 CORR 单据
    pub source_voucher_id: uuid::Uuid,
    pub posting_date: NaiveDate,
    pub backdate_authorization: Option<BackdateAuthorization>,
    pub source_event_id: Option<uuid::Uuid>,
    pub reason: String,
    pub reauth_ref: uuid::Uuid,
    pub approval_ref: uuid::Uuid,
    pub lines: Vec<CorrectionReclassification>, // 非空，source_voucher_line_id 唯一
}

#[async_trait::async_trait]
pub trait PostingPort: Send + Sync {
    async fn post(
        &self,
        tx: &mut dyn Tx,
        ctx: &SecurityContext,
        input: PostingInput,
    ) -> Result<PostingOutcome, AppError>;

    async fn post_reversal(
        &self,
        tx: &mut dyn Tx,
        ctx: &SecurityContext,
        source: CashDocumentRef,
        split: CashReversalPostingSplit,
    ) -> Result<PostingOutcome, AppError>;

    async fn post_correction(
        &self,
        tx: &mut dyn Tx,
        ctx: &SecurityContext,
        input: CorrectionPostingInput,
    ) -> Result<PostingOutcome, AppError>;
}

#[async_trait::async_trait]
pub trait TotalAccountBalanceProvider: Send + Sync {
    async fn balance(
        &self,
        snapshot: &dyn SnapshotCtx,
        legal_entity_id: Id<LegalEntity>,
        accounting_period_id: Id<AccountingPeriod>,
        account_role: AccountRole,
    ) -> Result<Money, AppError>;
}
```

资金冲正中，finance 必须在取得 F-50 全局锁并完成台账分流后构造两项 split；HTTP、Excel、插件和人工界面不得提交拆分金额或科目。ledger 按 `original_voucher_id` 锁读并校验同法人原凭证，其 `source_document_id` 等于 `original_doc_id` 且 `source_kind` 与四类原单一一对应；拆分两项非负且合计严格等于原资金腿。新凭证复制原四类之一的 `source_kind`，但 `source_document_type/id/no` 必须取本次 `reversal` 单据，`reverses_id=original_voucher_id`，所以幂等唯一键与原凭证不碰撞；生成行逐一 `reverses_id` 指向原资金/往来/advance 分录行，并完整复制 line_no、account、account_role、measure_key 与 amount，只反转 direction，行期间和日期取新头。reversal id/doc_no/posting_date 缺失、原凭证错单/错法人/已冲正或来源不匹配均整笔拒绝；第 9.3.6 节普通 UNIQUE 与延迟镜像触发器再从数据库层拒绝第二次冲正、部分覆盖或自由改腿。

受控更正首版只允许同法人已过账原凭证在成本侧 `MAIN_OPERATING_COST↔DIRECT_EXPENSE_COST` 双向重分类；`MAIN_OPERATING_REVENUE` 因没有第二个收入角色明确不可达，禁止资金、税、存货、往来、预收预付、GRNI、超量挂账、年结与留存收益角色。ledger 为每个 `CorrectionReclassification` 自动生成第 9.3.6.1 节的一对反向原行/目标行，不接受调用方提交方向、account_id 或自由分录；两条生成 voucher line 的 id 分别固化进证据行 `generated_voucher_line_id`，并在锁内按原凭证行校验历史累计上限。`source_capture_allocations` 必须非空、无重复且合计严格等于本行 amount；每个 id 必须是原 `source_voucher_line_id` 的现存 `costing.cost_entries` 当前 live 条目，收入归集 id、已完全反向条目、错凭证行或目标仍为同一成本角色都拒绝。ledger 在写两条生成凭证行后把对应行 id 和 allocations 交阶段 11 的成本捕获服务：逐原归集条目锁定，累计反向不超过其未更正余额，先生成指向源 live 条目的反向成本条目，再生成指向该反向条目的目标成本条目；两者 `source_document_type=CORRECTION_VOUCHER`，维度只复制原条目，公开 API 不接受维度。分配缺失、错侧、错凭证行、金额不等或累计超限均与凭证、余额、审计、Outbox 同事务回滚。三种方法共享同一期间解析、幂等、余额、审计与 Outbox 原子写入规则。

Stage 14 的历史凭证不扩展公开 `PostingPort`，也不新增 HTTP。`ep-app-ledger::migration` 模块只定义下面一个模块私有 trait 与三个模块私有 struct；均不写 `pub`/`pub(crate)`，`LedgerMigrationWriter` 是唯一实现者且只由同模块的 `MigrationModuleWriter for LedgerMigrationWriter` 调用，因此 Rust 可见性直接禁止其他 crate 或同 crate 兄弟模块复用，不新增 archcheck 规则：

```rust
struct HistoricalMigrationLineInput {
    pub account_id: uuid::Uuid,
    pub account_role: AccountRole,
    pub direction: Direction,
    pub amount: Money,
}
struct HistoricalMigrationPostingInput {
    pub data_migration_record_id: uuid::Uuid,
    pub target_voucher_id: uuid::Uuid,
    pub batch_no: String,
    pub posting_date: NaiveDate,
    pub lines: Vec<HistoricalMigrationLineInput>,
}
struct HistoricalMigrationReversalInput {
    pub data_migration_record_id: uuid::Uuid,
    pub target_voucher_id: uuid::Uuid,
    pub original_voucher_id: uuid::Uuid,
}

#[async_trait::async_trait]
trait HistoricalMigrationPostingPrivate {
    async fn post_historical_migration(
        &self,
        tx: &mut dyn Tx,
        ctx: &SecurityContext,
        input: HistoricalMigrationPostingInput,
    ) -> Result<PostingOutcome, AppError>;

    async fn reverse_migrated_historical_voucher(
        &self,
        tx: &mut dyn Tx,
        ctx: &SecurityContext,
        input: HistoricalMigrationReversalInput,
    ) -> Result<PostingOutcome, AppError>;
}
```

APPLY 逐值使用 Stage 14 VALIDATED 记录预留的 UUIDv7 `target_voucher_id`，不另生成根 id；`data_migration_record_id/batch_no` 形成上节唯一 source tuple，登记日/事件分别固定为数据库事务时钟与 NULL，`posting_date` 经普通 `AccountingPeriodResolver` 解析。ledger 模块的迁移记录必须按 `(posting_date ASC,record_seq ASC)` 应用；若记录早于本法人已存在的最早期间仍按既有日期错误拒绝，禁止私设“隐形历史期间”。`lines` 至少两项；每项 amount 严格大于零，account_id 全集不得重复，account_role 必须在锁内恰好绑定该同法人启用且可过账 account。实现按 `account_id UUID bytes ASC` 稳定排序并服务端生成连续 `line_no=1..N` 和 UUIDv7 行 id，`measure_key` 逐行固定为 `historical_migration`；借贷合计必须相等且大于零，不生成 costing/revenue attribution，也不接受 memo、自由 measure、source event、sequence、doc_no、期间、科目或方向以外的隐藏字段。

REVERSE 锁读 `original_voucher_id` 及完整行图，要求它就是同法人、同 `data_migration_record_id` 的未冲正 sequence 1 历史 APPLY 根；`target_voucher_id` 必须是本次新 UUIDv7 且尚不存在。反向日期只取数据库事务时钟所在自然日，不接受迁移工具字段，期间经普通 resolver 解析；source type/id/no 全复制父头、sequence 固定 2。实现逐父行生成一个 UUIDv7 子行并复制 line/account/role/measure/amount、反转 direction、line.reverses_id 指父行，头.reverses_id 指父头；不重新读取角色绑定、不重算/合并金额。两方法的凭证、余额、既有 `ledger.voucher.posted.v1`、Stage 14 R0、writer receipt、迁移记录状态、审计与 Outbox 必须在同一 UnitOfWork 提交，任何一步失败零部分事实。

TotalAccountBalanceProvider：以上代码块是唯一 exact 签名，供 ep-platform-recon 在同一 `SnapshotCtx` 下按法人、期间与科目角色取总账侧余额；不得改用 `&mut dyn Tx`、省略期间或返回 HTTP/数据库行类型。

F50LockSlicePort 与 CrossModuleLockCoordinator：唯一 exact ABI、二十类锁类别、全部 key/plan DTO、规范化、owner 映射、lease→reload→proof 两段证明、HMAC/TxId/法人/子集校验及 40001 整事务重试均以 `docs/superpowers/specs/2026-08-21-f50-financial-consistency-design.md` 第 10.0.1 节的 Rust 代码块为唯一可抄定义，本节不另建简化版。实现类型 `LedgerF50LockCoordinator` 固定在 `crates/application/ledger/src/f50_lock.rs`；它组合 finance、procure、invoice、inventory 各自 application crate 的 `F50LockSlicePort`。`lock_all` 只要求本次非空类别所对应的 owner 恰有一份，空类别不要求 owner，因此阶段 8/6/7 可按实际非空切片施工与测试，绝不为后续模块注入替身；阶段 10 四 owner 交齐后，`apps/core-server/src/wiring/f50_lock.rs` 与 `apps/job-worker/src/wiring/f50_lock.rs` 的独立最终发布 `--check` 才要求四槽各恰一个，缺一、重复或 owner 不符均使完整首版 F-50 写入口不注册。协调器只编排 owner contract，不直接对其他 schema 写 SQL；集合漂移的固定 SQLSTATE 40001 由 `ep-domain-ledger` 的私有 `F50LockEpochRepo` 与 db-pg 实现承载，不暴露为业务 contract。四个业务 contract 的 mutator 仅接收 foundation 的 `TransactionLockProof`，所以没有任何 contract→contract 依赖。

交付确认的调用形态按 A-09 固定：交付确认单归阶段 6 的 sales schema，其 confirm_delivery 用例在同一事务内按库存腿、过渡科目腿、凭证腿的次序调用三个端口，凭证腿即 PostingPort::post，`source_kind=DELIVERY_CONFIRMED`，measures 只有 `revenue_amount` 与存在库存成本行时的 `cogs_amount`；前者逐交付行提交 `RevenueDeliveryOrder/Line/NewRootOnly` attribution，后者逐库存交付行提交 `CostInventoryCogs/Line/NewRootOnly` attribution，不传控制总额或任何分支参数。会计期间由 AccountingPeriodResolver::resolve 在该事务最前解析一次，库存腿与过渡科目腿复用同一个 ResolvedPeriod。本阶段只提供这些端口，不建交付确认单，不编排三腿次序。


五个 trait 的方法签名只使用 ep-foundation 与 ep-contract-ledger 自身的类型，不出现数据库行类型与 HTTP 类型，ResolvedPeriod 与 F-50 业务锁计划定义在 ep-contract-ledger 而不下沉到 ep-foundation；foundation 只持业务无关的 `TransactionLockProof` 不透明载体。事务句柄为 ep_foundation::port::Tx，快照上下文为 ep_foundation::port::SnapshotCtx；业务写方法一律以 `&mut dyn Tx`、快照余额方法一律以 `&dyn SnapshotCtx`，按 A-01 由阶段 1 冻结，本阶段不另定义同名类型。`tests/trybuild/stage9_contracts/` 必须编译四个 consumer 正例并拒绝省略 `ctx`、用 SnapshotCtx 调写端口、用 Tx 调快照端口、构造私有 ResolvedPeriod 字段、漏传 split/input、把 Skipped 当无字段 unit variant、漏掉 PostingInput 七字段中的任一字段或尝试填写不存在的 `source_sequence_no`；recording-port 正例逐字段断言 `backdate_authorization/source_event_id/attributions`，成本或收入腿漏 attribution 的运行期契约测试必须返回 `LEDGER.POSTING.MEASURE_INVALID`。`ResolvedPeriod` 代码块另由 doctest 编译，防止 exact ABI 出现只有分号没有函数体的伪实现。

### 9.6 并发与事务边界

#### 9.6.1 过账事务

一个业务事件一个事务，凭证与业务状态、子账条目、审计事件、Outbox 条目在同一事务内写入。事务内禁止外部 HTTP 调用、文件正文读写、通知发送与长时计算；同事务通知只持久化提交后投递所需的命令。隔离级别 READ COMMITTED。事务预算按基线第 10.3 节：业务事务不超过 5 秒，读写池 statement_timeout 10 秒、lock_timeout 3 秒、idle_in_transaction_session_timeout 15 秒。

事务内的固定次序：解析期间、映射分录、插入凭证、插入分录行、按 `account_id` 升序更新余额、完成成本/收入捕获与调用方同步投影、执行全部事务末守恒断言、幂等 `finish`、刷新 Outbox、写同事务通知命令、审计终结批。固定次序是防死锁与防止审计后补写的共同硬约束。每条分录行的 `accounting_period_id` 与 `business_date` 等于所属凭证的对应值这一断言必须在幂等、Outbox 与审计之前执行，不成立即回滚。这两列是冗余副本，保留它们是为了明细账与余额校验不回表，一行断言买到的保证与删列相同，且不动任何索引与查询。

PostingPort 只立即写凭证、余额与同步捕获事实；`ledger.voucher.posted.v1` 和该次过账审计只登记到事务级待刷新集合，不得在端口返回前直接执行 Outbox 或 audit 表 SQL。事务所有者在所有跨模块写回完成后按上述固定后缀统一刷新。`AuditWriter::append_terminal` 封印 `Tx`，其后任何 SQL `query/execute`、仓储写入或跨模块端口调用都必须以内置不变量错误拒绝并令整个事务回滚，唯一允许的后继动作是 `UnitOfWork::commit`。共享跨模块契约测试 `audit_terminal_seals_tx` 以采购过账与受控更正为夹具：审计后分别尝试余额更新、`CostCaptureService`、Outbox 刷新与来源模块写回，均应失败、审计后零新增行、事务整体回滚；正常路径由 recording transaction 断言审计批是 commit 前最后一批数据库执行。

幂等：HTTP 层由 Idempotency-Key 与 platform_msg.idempotency_keys 承担，幂等键写入与业务写入同事务；过账层另由 ux_vouchers 的四列唯一键承担，唯一冲突转为 IdempotentReplay。两层幂等不互相替代：前者防同一请求重复提交，后者防同一业务事件经不同路径重复过账。

与 Outbox 的关系：本阶段只产出以下 9 个具名事件，名称与 `docs/event-catalog.md` 的阶段 9 行逐项一致，不允许再写匿名的“Outbox 条目”占位：`ledger.voucher.posted.v1`、`ledger.correction_voucher.posted.v1`、`ledger.period_close.requested.v1`、`ledger.period_close.accepted.v1`、`ledger.period_close.acceptance_rejected.v1`、`ledger.period_close.concluded.v1`、`ledger.period_close.cancelled.v1`、`ledger.year_end_closing.requested.v1`、`ledger.year_end_closing.executed.v1`。最后一项只对应 EXECUTED，固定携 `profit_loss_nonzero_account_count_before/profit_loss_net_balance_before_amount` 与两个条件可空 voucher id，使 count=0、count>0/net=0、count>0/net≠0 三形可区分；FAILED 不产出 executed 事件。本阶段不消费其他模块的业务事件用于过账；ledger 的下游消费者为报表与经营指标模块，其消费幂等由 platform_msg.inbox_consumptions 承担。

序列化失败 40001 与死锁 40P01 由数据访问层统一重试 3 次，退避 50、150、450 毫秒，只对尚未产生任何外部可见副作用的事务重试。过账事务无外部副作用，可重试。

#### 9.6.2 关账的事务边界

关账跨多个事务，逐个列出。

T1 发起事务：先 `INSERT ... ON CONFLICT(legal_entity_id) DO NOTHING` 确保空 slot 行，再写请求行、提交审批任务、执行幂等 `finish`、写 `ledger.period_close.requested.v1` 与同事务通知命令、最后写审计终结批。交互式，在 core-server。

T2 受理事务：slot 行 FOR UPDATE，判定两项前提；全部通过时置 ACCEPTED 并写 `ledger.period_close.accepted.v1`，任一不通过时固化拒绝事项并写 `ledger.period_close.acceptance_rejected.v1`；两支都在 Outbox/同事务通知命令之后以审计终结批收口。在 job-worker。slot 行锁是同一法人同一时点只允许一个已受理未结束关账请求的唯一保证，不使用部分唯一索引，理由是基线第 3.10 节禁止部分索引。

T3 在途集合登记事务：取 pg_snapshot_xip 写入请求行。必须在 T2 提交之后开启。

T4 等待：无事务，按轮询间隔取 pg_current_snapshot() 判定。

T5 快照持有事务：REPEATABLE READ 只读，经 UnitOfWork::snapshot_transact 开启并向执行体传入 SnapshotCtx，执行 pg_export_snapshot 并保持打开直到全部批次结束。该连接的 idle_in_transaction_session_timeout 必须为 0，见第 9.12 节偏离登记。

T6..Tn 批次事务：各自 READ COMMITTED 开启后立即 SET TRANSACTION SNAPSHOT，只读。生产/关账模式分别按 `ledger.close.batch_timeout_seconds`、`ledger.close.batch_work_mem`、`ledger.close.batch_temp_file_limit` 设置 statement_timeout、work_mem、temp_file_limit；恢复验收与生产恢复模式改用三个对应的 `recovery_mode_*` 键。分批行数同样按当前模式从 `batch_size` 或 `recovery_mode_batch_size` 读取。每次 run 启动时把四项值快照进运行记录，进行中的 run 不受热更影响，与只读分析池的同名上限分别取值。

Tn+1 结论事务：slot 行 FOR UPDATE；PASSED 分支写 request.status/conclusion/concluded_at、period.status/closed_by/closed_at、下一期间期初固化与 slot 释放，FAILED_DISCREPANCY/FAILED_INCOMPLETE 分支只写请求结论并释放 slot、期间保持 OPEN。request/period/slot 三行的 SQL 顺序不冻结，但 PASSED 的 concluded_at 与 closed_at 取同一个预先求值的服务器时点；第 9.3.9.1 节双向 FK与延迟图在提交点统一证明。随后写 `ledger.period_close.concluded.v1` 与同事务通知命令，最后写审计终结批。

取消分两事务：提交事务只以既有 `LEDGER_PERIOD_CLOSE` 场景和 `action=CANCEL` 建立独立审批实例及其加密待签快照并返回 202，request/status/slot 不变；批准回调事务锁 flow/snapshot/request 与 slot，重验 conclusion 为空、独立 reauth 摘要、法人/场景/动作、申请人与审批人分离后，写 CANCELLED/conclusion/concluded_at/cancellation_reauth_ref/cancellation_approval_ref/cancelled_by，并在同一事务解除指向本请求的 slot，期间不变。原关账请求的 reauth_ref/approval_ref/approved_by 逐值不改；同事务先写 `ledger.period_close.cancelled.v1` 与通知命令，最后写审计终结批。request 与 slot 可任意先后，漏释放、释放错请求、只写合法枚举却漏任一取消证据均在提交点失败。取消与结论并发时由 slot 行锁串行化，先到先得；结论已提交则取消回调返回 BUSINESS_CONFLICT 与 LEDGER.PERIOD_CLOSE_REQUEST.CONCLUSION_ALREADY_PRODUCED。驳回、撤回或过期只终结取消审批 snapshot，不改变关账请求；相同提交幂等键与相同批准回调均返回同一 approval_ref/最终 CANCELLED 对象。该处理次序是 U-H-13 的冻结取值。

失败重试与补偿：受理判定、等待、快照建立与批次执行均可重放，重放前先读请求当前状态做守卫，不产生重复副作用。批次执行失败按第 9.4.7 节的五类成因置 FAILED_INCOMPLETE，不自动重试本次请求；解除成因后由人重新发起，本阶段不设自动重发起，理由是关账属高风险操作，平台不自行解除任何状态、不存在无发起人的自动动作。

年结发起事务先以同一 `INSERT ... ON CONFLICT DO NOTHING` 确保空 slot 行，再写 PENDING_APPROVAL closing、审批任务、事件与审计。审批通过后的执行是另一个单事务：按“法人 close slot → 期间 → account UUID bytes”固定锁序取得串行化点、期间、全部损益/本年利润余额与所需科目绑定，冻结两个执行前控制字段，再按条件写 0/1/2 张凭证及余额，最后写 EXECUTED 与事件；closing、voucher 头行、余额和状态可任意语句顺序落库，由 `assert_year_end_closing_graph_consistent()` 在提交点判定。两种确定性业务失败只写 FAILED/failure_code/concluded_at 与审计/通知，不写凭证或余额；基础设施失败整事务回滚、closing 仍为 APPROVED。

#### 9.6.3 余额行的并发

同一 (科目, 期间) 行在并发过账下会串行。按附录 A.4 的 20 并发与 15% 提交占比、5 至 15 秒思考时间，提交速率约为每秒 0.3 笔，热点行竞争可忽略。更新写为无条件增量，不存在丢失更新。等待锁超过 lock_timeout 3 秒时返回 INFRASTRUCTURE 与限流类错误码并可重试。

#### 9.6.4 RLS 与安全上下文

全部读写经统一数据访问层，安全上下文在连接取用时写入 app.legal_entity_id 等四个会话变量，归还前逐项设回空串。关账校验按规格第 7.7 章的内部对账系统用途执行，确切判据为同一个 `SecurityContext` 的 `account_kind=System` 且 `system_purpose=Some(Reconciliation)`；按法人逐轮遍历，每轮只写单一法人，不建立跨法人会话，不绕过行级策略。该上下文不调用字段投影器，理由是规格第 17.3 章的判据本身就是该法人的全量合计；这是一句事实描述而不是属性访问控制豁免，本阶段不在规格第 12.2 章的豁免清单上交叉登记。本阶段的校验语句输出列只含勾稽项标识、法人、会计期间、科目、凭证号与金额合计，不含任何行内敏感字段，该边界由第 9.4.7 节的 subject_ref 键集白名单在写入侧强制。

### 9.7 配置项

全部新增键在 EP__LEDGER__ 前缀下，结构体开启 deny_unknown_fields。生效方式一律为启动时读取；标注为可热更的两项经机密与配置版本变更在下次取用时生效，不需重启。

| 键 | 类型 | 默认值 | 生效方式 | 说明 |
|---|---|---|---|---|
| ledger.period.auto_create_lead_days | u16 | 7 | 启动 | 提前多少天建立下一自然月期间 |
| ledger.period.fiscal_year_last_period_no | u8 | 12 | 启动 | 年度末次期间的期号，按 U-H-12 冻结取值固定为自然年 12 月 |
| ledger.posting.max_lines_per_voucher | u16 | 500 | 启动 | 单张凭证分录行上限 |
| ledger.close.inflight_wait_poll_interval_ms | u32 | 500 | 取用 | 在途写事务等待的轮询间隔 |
| ledger.close.inflight_wait_warn_seconds | u32 | 300 | 取用 | 等待超时只告警不终止的阈值 |
| ledger.close.batch_size | u32 | 20000 | 取用 | 单批处理的分录行或科目行数 |
| ledger.close.batch_timeout_seconds | u32 | 120 | 取用 | 单批执行时限，触发终止即校验未完成 |
| ledger.close.batch_work_mem | string | "256MB" | 取用 | 批次连接的单查询内存上限 |
| ledger.close.batch_temp_file_limit | string | "4GB" | 取用 | 批次连接的临时空间上限 |
| ledger.close.recovery_mode_batch_size | u32，1000–20000 | 5000 | 取用 | 恢复模式下的分批规模，与常规取值分别冻结 |
| ledger.close.recovery_mode_batch_timeout_seconds | u32，60–900 | 300 | 取用 | 恢复模式下的单批执行时限，禁止 0 或无限 |
| ledger.close.recovery_mode_batch_work_mem | string，64MB–512MB | "128MB" | 取用 | 恢复模式批次连接的单查询内存硬上限 |
| ledger.close.recovery_mode_batch_temp_file_limit | string，512MB–8GB | "2GB" | 取用 | 恢复模式批次连接的临时空间硬上限 |

`ledger.close.batch_*` 四项与 `ledger.close.recovery_mode_batch_*` 四项是两套独立的已批准开发值。“取用”严格表示新 run 创建时读取并把四项快照进运行记录，不是 SIGHUP、目录监听或对运行中 run 热改。恢复模式四项对应环境变量依次为 `EP__LEDGER__CLOSE__RECOVERY_MODE_BATCH_SIZE`、`EP__LEDGER__CLOSE__RECOVERY_MODE_BATCH_TIMEOUT_SECONDS`、`EP__LEDGER__CLOSE__RECOVERY_MODE_BATCH_WORK_MEM`、`EP__LEDGER__CLOSE__RECOVERY_MODE_BATCH_TEMP_FILE_LIMIT`；不得缺省成无上限，也不得复用生产四项。配置解析失败、越界、单位非法，或启动自检无法在 PostgreSQL 会话上成功 `SET LOCAL` 任一上限时，job-worker 的 `config-parsed`/配置自检以退出码 78 失败，不启动恢复任务。运行中 statement_timeout 触发时写 `termination_cause=BATCH_TIMEOUT`；work_mem/temp_file_limit 或数据库资源限制触发时写 `termination_cause=RESOURCE_LIMIT`；两者均把 run 置 `FAILED_INCOMPLETE`，对外返回既有 `LEDGER.PERIOD_CLOSE_REQUEST.VALIDATION_INCOMPLETE`，生成未完成事项、告警和降级窗口，不自动放宽、禁用上限或重试为成功。

阶段 14 的附录 A.4 认证与附录 A.6 恢复演练分别验证生产四项和恢复四项。恢复演练从第一次运行起就使用上述有界默认值，不存在“不施加单批时限或资源上限”的暖机支路；两次演练均须记录四项配置、实际各批行数、耗时、内存与临时空间峰值，并在 4 小时 RTO 内全部通过。需要调整时只能在上述范围内经签名配置发布形成新版本，再从头重跑两次；任一四项变化立即使旧演练证据失效。证据未形成时能力状态为 `UNVERIFIED` 并阻止发布，不阻止按这组契约开发。客户实际数据量超出附录 A.3 基准时按相同受控流程在该部署上重取并重演，结论写入部署记录；超出允许范围不得现场放宽，须提升硬件或另立正式规格变更。

运行期可变的业务参数不进配置文件：事件科目对应关系、审批链、科目类别枚举一律存事务数据库并经配置发布通道发布。

启动自检：本阶段不新增任何启动自检项，也不在启动路径上判读任何业务数据行。基线第 7.3 节的 current-period-open 项整项删除，理由是它自述缺失时按规格第 5.2 章自动建立，那是一次写操作而不是闸门，且会让多个进程在自检阶段并发写 ledger 表。原拟挂在其下的两条子判定各自下沉。第一条即每个法人存在当前自然月的打开会计期间，下沉到过账路径，由第 9.4.4 节第二步的零期间分支、第五步的顺延目标自动建立与 job-worker 的提前建立定时任务三者共同承担，其中该法人的首个会计期间定死由第二步的零期间分支在首次过账的同一业务事务内建立，三者都在业务事务内或后台任务内完成，缺失不再影响任何进程启动。第二条即每个法人的 17 个科目角色全部已绑定且绑定到启用科目，下沉到取用点，由第 9.4.3 节第三步的 ROLE_UNBOUND 与 ACCOUNT_INACTIVE 阻断该类事件提交，由第 9.5.3 节的 check-completeness 端点供建账验收与运维中心查看，未绑定按规格第 15.3 章告警并经 ep-platform-obs 的 DegradationLedger::open 登记降级窗口、绑定后关闭。经此，任何一条业务数据不符都不会放大成八个进程集体拒绝启动，而未绑定的后果仍然是显式的、可查询的、会告警的。

### 9.8 测试计划

覆盖率门槛：本阶段全部代码属规格第 17.3 章强制不变量相关代码，行覆盖率不低于 85%；新增与修改代码不低于 80%；工作区整体不低于 80%。工具为 cargo-llvm-cov，CI 上以 --fail-under-lines 强制，路径规则写入 codecov.toml 的 crates/domain/ledger、crates/application/ledger、crates/contract/ledger 三条。不允许长期跳过用例。

#### 9.8.1 单元测试

映射表：遍历每条 `JournalRule`，断言 `(source_kind, MeasureKey)` 唯一、`legs` 非空、每个来源的必填与可选集合不相交、请求键唯一、互斥组和平衡方程全部成立；每个成本/收入腿恰有一个 `CapturePolicy::Required`，其他腿恰为 `None`，策略的 kind/grain/parent 与字典逐腿相等。逐一生成 `docs/data-dictionary/ledger.md` 列出的全部合法组合并断言借贷平衡；再对每个 Required 策略生成缺归因、错 kind、错粒度、父缺失/多余/非 live、合计差一分与非归集腿伪归因负例。`YEAR_END_PL_CLOSING`、`CORRECTION` 与 `HISTORICAL_MIGRATION` 必须被普通 `post` 拒绝，三者分别只走受控年结、`post_correction` 与 crate-private migration writer；静态断言 `HISTORICAL_MIGRATION` 在 `JOURNAL_MAP` 零行，防止伪造空规则。

符号归一：正金额、负金额、零金额三个分支；负金额翻转方向后净额不变；零金额不生成行。

映射失败分支：未登记的 MeasureKey、缺失的必填 MeasureKey、角色未绑定、绑定到停用科目、绑定到有下级的一级科目、行数超上限、借贷不平、行数小于 2、全部为零。

期间归属：记账日期晚于服务器自然日、该法人零期间时首次过账建立首期、该法人已有期间而记账日期早于其最早期间起始日、落在可入账期间、落在已关闭期间、落在有已受理未结束关账请求的期间、顺延目标不存在需自动建立、顺延目标跨年度，共 8 个分支。另以真实授权链覆盖补记四个负正例：无 `LEDGER_BACKDATE`、无重新认证、无 `FINANCE_MANAGER` 审批分别拒绝，三项齐备时成功且审计五元组完整；客户端伪造登记日或证明引用必须拒绝。

关账状态机：逐条遍历第 9.4.5 节的转移表，断言每条合法转移成功、每条非法转移返回 BUSINESS_CONFLICT；断言 conclusion 一经非空即不可再变。PENDING_APPROVAL、ACCEPTED、VALIDATING 三个前缀各覆盖一次独立取消批准回调，断言三项 cancellation 证据全有、原 reauth/approval/approved_by 逐值不改；漏项、复用原 ref、非 CANCELLED 偷带取消证据均失败。

年结算法：至少覆盖损益余额全零（count=0/net=0，零凭证）、单科目非零（count>0/net!=0，两凭证）、多个损益科目逐项非零但净额为零（count>0/net=0，仅第一张）、多个损益科目净额非零、正负两种 net、PROFIT_THIS_YEAR/RETAINED_EARNINGS_UNDISTRIBUTED 角色未绑定、重复执行、非年度末次期间、期间非可入账。逐分支断言两个 server-frozen 控制字段、0/1/2 张凭证槽位、source_sequence_no 与余额终值；PERIOD_NOT_POSTABLE/ROLE_UNBOUND 只生成 FAILED/failure_code/concluded_at，基础设施失败仍为 APPROVED。

余额推演：期初已固化与未固化两条路径；跨已关闭期间递推；损益类跨年归零。

#### 9.8.2 领域属性测试

按基线第 8.1 节，本阶段承担五组不变量中的借贷平衡一组，另自设三组。工具为 proptest，最小用例数 1024。

一是借贷平衡：对任意合法的普通 `(source_kind,measures)` 组合，映射产出的凭证借方合计等于贷方合计；另对任意至少两行、account 唯一、正金额且借贷相等的历史迁移输入，专用入口输出恰同额平衡图，对任意不平、重复 account、未绑定/停用/不可过账科目、零负金额、少于两行输入均拒绝。生成合法 APPLY 后的专用 REVERSE 必须逐腿完整反向且净额为零；任意删腿、添腿、换腿、错 source tuple 或第二次反向都不能形成合法最终图。新增普通来源按映射三件套扩展本组；新增专用来源按 CHECK/唯一入口/数据库图/正反测试四件套扩展，缺失即 CI 失败。

二是期间归属收敛：对任意 (期间集合状态, 合法 posting_date)，resolve 的返回期间一定处于 OPEN 且不存在已受理未结束的关账请求，且其 start_date 大于等于 posting_date 所属期间的 start_date。

三是余额可重放：对任意凭证序列，增量维护的 account_period_balances 与按 voucher_lines 全量聚合的结果逐科目逐期间相等。

四是符号归一保净额：对任意计量项金额序列，归一前后按科目角色的净额相等。

#### 9.8.3 集成测试

使用真实 PostgreSQL 16，每用例独占一个 ep_test_<nanoid> 数据库，用例结束即删库。禁止内存库与 mock。测试数据一律经 ep-testkit 构造器，禁止手写 INSERT。时间经 FixedClock 注入，禁止 sleep。

场景清单。

一是 RLS 与越权：ledger 的 13 张带法人列的表与 platform_core 下 recon 的 2 张带法人列的表在读取、写入、更新、删除、聚合、排序、报表投影与错误信息泄漏八类上不越权；两个复制角色与内部对账系统用途的五个入口借用测试。八类断言复用 testkit/src/rls_matrix.rs 中由阶段 1 提供的 assert_read、assert_write、assert_update、assert_delete、assert_aggregate、assert_sort、assert_report_projection、assert_error_leak 八个函数，入口借用复用阶段 2 提供的 assert_replication_role_containment 与 assert_recon_context_borrow，本阶段不实现同名函数；完整矩阵与发布门禁项 RG-RLS-MATRIX-GREEN 归阶段 4。对账部分须同时证明：五个外部入口均不能调用 `ReconExecutor::run`；除定义与 executor 外直写 `SystemPurpose::Reconciliation` 的负样例由 archcheck 拒绝；`General`、`None` 与非 System 三类上下文调用对账仓储均返回 `PLATFORM.AUTHZ.OBJECT_FORBIDDEN` 且连接池取用计数不增加；两个法人逐轮结果互不混入。该组属 tests/rls_matrix。

二是不可覆盖与凭证图/受控经济镜像：以 ep_app_rw 对 ledger.vouchers、ledger.voucher_lines、ledger.correction_vouchers 与 ledger.correction_voucher_lines 执行 UPDATE 与 DELETE 均被数据库拒绝。真实 PostgreSQL 正例覆盖普通凭证头行 count/sum/period/date 完整、四类资金凭证各一次完整反向，以及 `HISTORICAL_MIGRATION` APPLY/REVERSE 各一次；两类反向都逐父行证明 account/role/measure/amount/line_no 相同、direction 相反。既有资金直 SQL 负例保持不变；历史迁移另逐项构造：普通入口偷写该 kind、type 非 `DATA_MIGRATION_RECORD`、id/no 与同法人 record/batch 不等、source_event 非空、APPLY sequence 非 1 或带父、REVERSE sequence 非 2/换 type/id/no/父非历史 APPLY/父已被冲、反向日期或期间伪装成父值、少/多/重复/跨凭证父行、只冲部分金额、换 account/account_role/measure_key/line_no、方向未反转、第二次反向。除即时 UNIQUE/CHECK 负例外，其余必须让普通 FK 命中后在 `SET CONSTRAINTS ALL IMMEDIATE` 或 COMMIT 由延迟约束拒绝，并证明凭证、余额、writer receipt、迁移记录、R0、Outbox 零部分写入；合法历史图按头先/行先两种 SQL 顺序均可提交。

三是过账幂等：同一 (source_kind, source_document_id) 连续提交 3 次以上，凭证只产生一次、余额只增一次、审计只写一次、Outbox 只写一条。

四是期间自然月、自动建立并发与关账三表状态证据图：并发 10 个过账事务同时触发同一期间的自动建立，最终只产生一行，其余走 ON CONFLICT 分支；另一条并发用例针对零期间分支，在该法人无任何期间时并发 10 个首次过账事务，断言最终只产生一行期间、全部凭证落入该期间。以 direct SQL 分别构造错月首、错月末、`period_code` 与年月不符、非 12 月标末期/12 月未标末期、同法人重复年月、同法人重复起日，由第 2 号迁移仍在的自然月 CHECK/UNIQUE 拒绝；同一套正例写入连续 36 个自然月，断言无重叠、12 月标志逐值正确。

本场景再用真实三表与 `SET CONSTRAINTS ALL IMMEDIATE`/COMMIT 覆盖最终证据图。正例把 ACCEPTED、VALIDATING 与 PASSED/CLOSED 三种最终图分别按 request→slot、slot→request、request→period→slot、period→request→slot 等语句排列提交，证明任意同事务写序均可；OPEN 期间无关闭证据、空 slot 与全部非 active/非 PASSED 请求也可提交。九个状态逐态跑完整字段形状。负例逐项覆盖：任一 non-active request 缺同法人 slot、错法人或错期间 request；OPEN 偷带 closed_at/closed_by 任一项；CLOSED 缺任一关闭证据；CLOSED 指 CANCELLED、FAILED_DISCREPANCY、FAILED_INCOMPLETE 或另一期间；PASSED 无反向 CLOSED period、concluded_at 与 closed_at 差一微秒；active request 无 slot、slot 单向指 request、request 单向指 slot、slot 指 terminal、两个 active request 争同一 slot、terminal 仍占 slot；PENDING_APPROVAL、APPROVAL_REJECTED、ACCEPTANCE_REFUSED、ACCEPTED、VALIDATING 与四种结论态任一必需字段缺失、多出后续阶段字段、snapshot 两字段半空、completed_batch_count<0、时间前缀逆序；CANCELLED 三项独立证据半空、任一 ref 等于原申请 ref、cancelled_by 跨法人，以及非 CANCELLED 状态偷带任一取消证据。每个负例必须让短 ID 存在以绕过普通存在性检查，再由长 FK 或三表延迟图拒绝，并证明期间、请求、slot、期初固化、审计与 Outbox 零部分写入。

五是受理前提逐项：期间已关闭、非最早打开期间、已有已受理未结束请求、待消费过账非零、未修复死信非零，共 5 个用例，各自断言 refusal_reasons 载明未满足项与其当前取值、期间状态不变、请求可重新发起。

六是连续两次受理被拒的告警与暴露窗口记录，及该期间关账完成后暴露窗口消除。

七是关账受理与在途过账事务的交叠，必须覆盖“解析时尚未写入”的危险窗口：事务 A 执行 `BEGIN` 后不做任何写入，首次调用 resolver；用 SQL 记录器断言其第一条数据库语句恰为 `SELECT pg_current_xact_id()`，随后才读取待关 OPEN 期间。事务 A 停在解析完成、凭证尚未写入的位置；事务 B 完成受理并提交，事务 C（T2）取得的 `pg_snapshot_xip(pg_current_snapshot())` 必须含事务 A 的顶层 XID。此后事务 A 才写凭证并提交，断言等待阶段在 A 结束前不能完成、A 的凭证仍落入待关期间且进入本次冻结快照。另设两个控制反例：resolver 若把分配 XID 放到读取期间之后，或完全省略该语句，契约/SQL 序列测试必须失败；同一事务第二次调用 resolver 只命中 memo，不重复执行分配 XID 或期间查询。

八是顺延入账：在一次关账请求受理之后、本次关账产生结论之前提交一笔记账日期属于该待关闭期间的业务事件，断言提交成功、凭证记入其后最早的可入账期间、保留原始业务日期、deferred_from_period_id 非空、不进入本次快照、不改变本次关账结论、按两条路径均可检索、同一业务事件的子账条目与该凭证落入同一会计期间、顺延前后借贷与取价均不变。

九是年度末次期间损益归零与年结提交图：由受理时点在途、受理后方可见的写事务使该期间损益余额非零，断言在同一快照上逐非零科目检出、本次关账被拦截、差异事项逐项载明 account_id 与 signed 余额、期间保持打开；专设两个非零损益科目净和为零的反例，证明净合计为零仍不能关账。再执行一次结转后重新发起并通过。年结正例逐一覆盖 count=0/net=0 的零凭证、count>0/net=0 的单凭证、net>0 与 net<0 的双凭证，以及“closing 先、凭证/余额后”和“凭证/余额先、closing 后”两种跨图写序，均在 COMMIT 通过并逐值核对 frozen controls、余额与 source 头；voucher line 仍遵守通用头行 FK 的写序，不把本图误写成放宽普通 FK。

同组 direct SQL 负例逐项覆盖：closing 缺同法人 slot；五态字段 shape 错误；failure_code 为空、越过 `PERIOD_NOT_POSTABLE|ROLE_UNBOUND` 闭集、FAILED 偷带执行或凭证证据，以及期间仍可入账且 slot 为空却伪造 PERIOD_NOT_POSTABLE、角色完整却伪造 ROLE_UNBOUND、两条件同时成立却写低优先级 ROLE_UNBOUND；非末期、fiscal_year 不同或期间错法人，以及 closing 已引用后成组篡改期间年月身份字段；EXECUTED 时期间已 CLOSED，或本法人 slot 指向同期间/另一期间的任一 active close request；count/net/0-1-2 凭证槽位任一不符；两个槽位同 id、孤立 YEAR_END 凭证、sequence 互换或重复；source_kind/document_type/document id/no、event/reverses/deferred、期间、end_date 任一错值；第一张少/多/重复一个 P&L 来源腿、金额或方向错误、count 与非零来源腿数不等、净额与 PROFIT_THIS_YEAR 腿不等；第二张不是 abs(net)、未清零 PROFIT_THIS_YEAR、未等额落 RETAINED_EARNINGS_UNDISTRIBUTED；控制字段与由最终余额及本次凭证腿反推的 pre-image 差一分。所有关系负例先写齐普通 FK 目标，再由即时 identity guard 或 `assert_year_end_closing_graph_consistent()` 在提交点拒绝，且 closing、凭证、余额、审计、事件零部分写入。另证明基础设施/事务错误整笔回滚后仍为 APPROVED，不生成 FAILED 或第三种 failure_code。

十是校验未完成：在测试配置允许的最小值上构造必然超过 `ledger.close.batch_timeout_seconds` 的批次，断言置 FAILED_INCOMPLETE、生成校验未完成事项且不载明差额字段、告警触发、计入暴露窗口、期间保持打开；再分别构造恢复模式 statement_timeout 与 temp_file_limit 触发支，断言映射为 BATCH_TIMEOUT 与 RESOURCE_LIMIT，且系统不改成无上限、不自动重试成功。解除成因后重新发起并在本执行窗口内通过。另以 999/20001、59/901、63MB/513MB、511MB/9GB 和非法单位作为配置负例，断言 job-worker 以 78 拒启。

十一是取消审批与结论并发：PENDING_APPROVAL、ACCEPTED、VALIDATING 三种来源状态分别提交 `LEDGER_PERIOD_CLOSE/action=CANCEL`，断言提交阶段只产生审批/snapshot 且 request/slot 不变，批准回调才以三项独立证据进入 CANCELLED；原申请 reauth/approval/approved_by 不变，`ledger.period_close.cancelled.v1` 的 cancellation_reauth_ref/cancellation_approval_ref/cancelled_by 来自终态行且 cancelled_at 等于 concluded_at，驳回/撤回/过期与回调重放均无重复或半状态。再同时执行批准回调与校验结论落库，断言二者之一成功、另一按 slot 行锁次序返回明确错误、状态唯一；复用原关账 approval_ref/reauth_ref 的负例在提交点失败。

十二是关账通过后已关闭期间拒绝任何凭证写入，且下一期间的期初已固化并等于本期期末。

十三是试算平衡与会计恒等：注入一张人工构造的不平凭证不可能（受 CHECK 阻断），改为在快照上注入余额行差异，断言校验检出并拦截关账。

十四是期初余额批次：借贷不平拒绝、已有凭证拒绝、重复科目拒绝、确认后进入首期期初列；另用直 SQL 覆盖 CONFIRMED 头金额不等、PENDING_APPROVAL/CONFIRMED 零行、行合计与头任一侧不等、双零行、跨法人父行五类负例，普通 FK 可命中的跨表反例在 `SET CONSTRAINTS ALL IMMEDIATE`/COMMIT 失败且期初投影零部分更新。确认后头 UPDATE/DELETE 与其行 INSERT/UPDATE/DELETE 全由即时不可变守卫拒绝。
十五是受治理数据集视图：以 ep_analyst_ro 连接可 SELECT ledger.v_account_period_balances，该角色对 ledger 的任何基表无读写权限；视图输出含 legal_entity_id、security_level、data_scope_tags 三列且列名与类型签名与 reporting.dataset_fields 中 dataset code 为 ledger_account_period_balances 的登记行逐列相等；跨法人取数被行级策略拦截。本场景只做上述静态比对，不调用阶段 11 的 reporting-dataset-signature-matched 自检项，该自检项按基线第 12 节通则第六条与 E-17 同一档位整条推迟到阶段 11。

十六是受控更正审批命令：提交后只存在 flow 实例与加密 snapshot，HTTP 为 202，ledger/costing/号码均零新增；默认 FINANCE_MANAGER 通过后回调恰生成一张 CORR 终态、一张平衡凭证和完整归集更正并写 CONSUMED 结果引用。正例只跑 `MAIN_OPERATING_COST→DIRECT_EXPENSE_COST` 与反向两种；收入角色、收入 capture id、同角色目标与所有非成本角色必须在审批提交、回调重验和数据库提交三层均拒绝。覆盖申请人自审、链缺失、空节点/空展开、驳回、过期、密文或 AAD 摘要不符、审批期间原行可更正余额变化、补记授权失效、回调在 post_correction/snapshot/Outbox/audit 四点失败及重复回调；所有负例零部分业务事实，相同 HTTP 幂等键与相同回调均返回同一 approval_ref/最终对象。另以直 SQL 构造 pair 缺腿/多腿、两腿不同 source/amount、方向不反、角色相同或越界、生成行属于另一 generated voucher、同一生成行重复映射、证据与生成行 account/role/direction/amount/line_no/measure_key 不等、生成凭证多自由腿及累计超原行，全部在 UNIQUE/CHECK 或延迟提交点拒绝且 CORR/凭证/余额/归集零部分写入。

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

以下 24 条全部达成才算本阶段完成，逐条可客观判定。

E-1 四个新增 crate 在 cargo build --workspace 与 cargo clippy --workspace -- -D warnings 下无告警通过；CI 的依赖方向自检脚本对 ledger 的六条断言（清单以第 9.2 节依赖方向自检段为准，阶段 11 追加 ep-contract-costing 后由该阶段同步更新）与对 ep-platform-recon 的一条断言全部通过；单文件不超过 800 行、单函数不超过 50 行、嵌套不超过 4 层的检查通过。

E-2 db/migrations/ledger/ 的 18 个迁移与 db/migrations/platform_core/ 的 3 个对账迁移在空库上离线执行成功，且在含 36 个期间基准数据集的库上重放成功；第 2 号迁移的自然月 CHECK 与法人内年月、起日候选键均可由系统目录查得，9b 全量 up 后其临时关闭形状 CHECK 已由第 12 号迁移删除。第 5、7、9 号迁移的期初余额图、通用凭证/资金镜像图和 CORR pair/生成行图，以及第 12 号的三表关闭证据图、第 13 号的年结终态/凭证/余额图，均可查得为 `DEFERRABLE INITIALLY DEFERRED`，对应 generated key、长 FK、CHECK 与即时守卫存在且表达式/列序逐字等于第 9.3 节；Stage 14 的 092600 应用后，source kind/sequence CHECK 及通用图函数必须逐字扩展为第 19 个 `HISTORICAL_MIGRATION`、APPLY/REVERSE 双向 receipt 与完整镜像分支，且普通凭证分支签名不变。本阶段两张不带法人列的表 ledger.posting_trigger_event_types 与 platform_core.recon_check_definitions 在 platform_core.unpoliced_table_registry 中各有一行登记，且 db/checks 的第十三项返回零行。每个文件的 -- rollback: 段存在；第 12、13 号各在空事实下 up/down/up 签名一致、在有证据事实时 down 失败关闭且不遗留半图；092600 只在历史迁移 voucher/receipt/record/R0 全空时允许撤回该分支，否则在任何 DROP/ALTER 前失败。在线变更边界内的操作实测锁持有不超过 5 秒。

E-3 ledger schema 的 13 张带法人列的表与 platform_core 下 recon 的 2 张带法人列的表全部 ENABLE 且 FORCE 行级安全，策略名与模板一致；启动自检的 rls-enabled-and-forced 项在含本阶段表的库上通过。

E-4 tests/rls_matrix 中 ledger 的八类越权用例与五个入口借用用例全部通过。

E-5 以 ep_app_rw 对 ledger.vouchers、ledger.voucher_lines、ledger.correction_vouchers 与 ledger.correction_voucher_lines 的 UPDATE 与 DELETE 被数据库拒绝，用例留证；会计期间错月/重复、关账请求九态字段形状及 CANCELLED 独立 reauth/approval/actor 证据、request↔active slot 与 PASSED request↔CLOSED period 双向证据、普通凭证 count/sum/period/date、资金冲正单次完整经济镜像、HISTORICAL_MIGRATION source tuple/sequence/receipt 双向图与单次完整镜像、CORR pair 与生成 voucher line 一一映射/累计上限、期初批次非空合计和确认后不可变、年结五态/末期/0-1-2 凭证/source/逐科目清零/控制字段反推的全部 direct-SQL 正反例通过。合法关闭、年结与历史迁移图至少各以两种相反 SQL 写序提交成功；所有提交点负例均证明零请求、slot、期间关闭、凭证、余额、期初投影、归集、writer receipt、迁移记录、R0、审计与 Outbox 部分写入。

E-6 JOURNAL_MAP 覆盖规格第 5.2 章事件-分录表十类事件的全部分录集合，逐条与该表比对的核对清单由财务负责人或其代表签署，签署件归档到 docs/reviews/。

E-7 四组领域属性测试各不少于 1024 个用例通过；第一组同时覆盖 16 个 map-backed 来源与 `HISTORICAL_MIGRATION` 专用 APPLY/REVERSE 的平衡、镜像和非法输入闭集，后三个特殊来源不以空 `JOURNAL_MAP` 行冒充覆盖。

E-8 十六组集成测试场景全部通过，其中第十六组证明更正凭证在审批前零业务写、审批后单事务终态落图、驳回/过期/竞态/回调重放均无重复或部分事实。

E-9 关账受理与在途写事务交叠、顺延入账、年度末次期间损益归零、校验未完成四个用例通过，且四者的断言逐项对应规格第 10.2 章与第 17.2 章的原文判据。

E-10 关账前强制校验的四类校验项各实现一个 ReconCheck 并经 ReconRegistry::register 注册成功，注入借贷差异与损益非零两类差异后差异事项生成且可追溯、本次关账被拦截、差异清零后重新发起正常受理并通过。

E-11 年度损益结转的 count=0/net=0 零凭证、count>0/net=0 单凭证、net 正/负双凭证三类形状均可执行且可重复执行；两个执行前控制字段可由凭证腿与余额反推，第一张逐非零损益科目完整清零，第二张仅在 net 非零时以 abs(net) 清零本年利润。结转后该期间全部损益类科目及 PROFIT_THIS_YEAR 余额为零，会计恒等在结转前后两种取数口径下均成立；两类确定性 FAILED 与基础设施失败保持 APPROVED 的分支逐项通过。

E-12 覆盖率门槛达成：ledger 三个 crate 行覆盖率不低于 85%，新增与修改代码不低于 80%。

E-13 总账凭证过账 P95 不超过 3 秒、月度科目余额表 P95 不超过 10 秒，各不少于 200 次样本；七个查询的 EXPLAIN 证据无顺序扫描。

E-14 `docs/error-codes.md` 的 36 个 LEDGER 错误码与 `crates/contract/ledger` 的错误码常量表逐条对齐，无重复码，无仅文档侧或仅代码侧存在的码；`docs/event-catalog.md` 的 9 个 ledger 事件、`docs/data-dictionary/ledger.md` 的 14 张表与 2 个视图、`docs/data-dictionary/platform_core.md` 中对账三张表全部登记，CI 的一致性校验通过。

E-15 新增的 4 个指标在 ops-agent 的 127.0.0.1:9101 上可抓取，标签基数符合基线第 9.2 节纪律。

E-16 桌面端五个场景与移动端两个查看场景的端到端用例通过；移动端写入操作按替代路径验证转桌面端完成。
E-17 受治理数据集视图 ledger.v_account_period_balances 已发布，dataset code 为 ledger_account_period_balances、grain 为 SNAPSHOT，输出含 legal_entity_id、security_level、data_scope_tags 三列，已 GRANT SELECT 给 ep_analyst_ro，其列名与类型签名与 reporting.dataset_fields 中 dataset code 为 ledger_account_period_balances 的登记行逐列相等；本条以该登记行与本阶段交付的视图定义直接对读判定，是静态比对，不调用任何启动自检项。原写的由阶段 11 的 reporting-dataset-signature-matched 自检项校验通过一句，按基线第 12 节通则第六条整条推迟到阶段 11，由阶段 11 的退出条件承担；其重新生效的触发谓词为该自检项已按基线第 7.3 节注册进 SelfCheckRegistry 并可被 --check 模式实际执行，该谓词由判定工具自身可观测，不写成阶段号，也不需要任何人工翻牌动作。本条的达成与否只取决于上述静态比对，不因该自检项尚未交付而恒真或恒不可满足。

E-18 本模块在规格第 6.2 章能力矩阵中取值为完整或简化的能力域，其四端界面已实现并通过 Playwright 与 tauri-driver 的桌面用例、XCUITest 与 Espresso 的移动用例；取值为 VIEW_ONLY 的能力域只实现只读视图；取值为 NOT_APPLICABLE 的不实现入口。

E-19 本阶段全部路由的能力域码与动作类别常量已声明在 crates/contract/ledger/src/capability.rs，xtask configdoc 通过。

E-20 ep-platform-recon 本体已交付并可被其他阶段使用：crate、platform_core 下三张表、ReconCheck 与 ReconRegistry 与 ReconExecutor 三个契约、job-worker 内的分批执行器与每日对账调度齐备；ReconCheck 注册表封闭、无运行期语句入口，archcheck 断言通过，差异事项 subject_ref 的键集白名单在写入侧生效；以内置校验项在两个法人上跑通一次每日对账，差异事项与校验未完成事项均落库且可追溯。

E-21 platform_core.append_only_registry 中 ledger.vouchers、ledger.voucher_lines、ledger.correction_vouchers、ledger.correction_voucher_lines 与 platform_core.recon_runs 五行已登记，五行的 mode 均为 APPEND_ONLY、mutable_columns 均为空数组，与表上的仅追加约束一致，db/checks/append_only_consistency.sql 经 xtask sqlcheck 通过。

E-22 单据类型码 OBB、GV、PCR、YEC、CORR 已登记在 docs/data-dictionary.md 的单据类型码一节，且与 ep-platform-sequence 的常量表逐项一致，xtask configdoc --check-doc-type-codes 通过。

E-23 testkit/scenarios/golden_loop_14_steps.rs 在 ep-datagen 默认 scale 数据集上一次跑通，覆盖规格第 8 章第 1 至 14 步与规格第 17.2 章十五类必测分支，规格第 17.3 章强制不变量在该用例上自动校验通过，Windows 与 macOS 两端的桌面走查同批通过，执行记录纳入发布证据包。
E-24 规格第 21.4 章要求的专业签字已取得并留档：会计与税务在本阶段签字，签字人资格证据随版本留档；签字缺失或不通过时本阶段不得退出，整改后重新测试并重新签字，不得以未记录的方式豁免（规格第 22 章第 12 条）。本条由裁定 F-42 新增，此前四份计划的退出条件中无任何签字项。


### 9.10 与规格和 PRD 的对应

#### 9.10.1 规格条目

| 规格章节 | 本阶段实现的条目 |
|---|---|
| 第 5.2 章 财务规则 | 每法人单一账簿、本位币人民币；会计科目表最多两级含编码、名称、类别、借贷方向、启用状态与期初余额录入；内置固定的十类业务事件到分录映射；科目对应关系由管理员配置；首版不设库存出入库事件类别 |
| 第 5.2 章 总账功能与期末处理 | 凭证生成、科目余额表、总账与明细账查询、试算平衡、月度期间开闭、年度损益结转；红字冲销与更正凭证只追加不覆盖；损益类年中保留累计余额 |
| 第 5.2 章 业务事件的记账日期 | 默认取业务单据日期且不得晚于登记时点自然日；早于服务端登记日即补记，必须具备 `LEDGER_BACKDATE` 权限、重新认证并经 `FINANCE_MANAGER` 审批；只能落开放期间，关闭期间沿既有顺延规则；审计记录原日期、登记时间、修改人、reauth_ref 与 approval_ref |
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

写审计的动作清单固定 15 个，写入 platform_audit.audit_events，与业务变更同事务且一律作为 commit 前最后一批数据库执行：账户创建、账户修改、账户停用、账户启用、事件科目绑定变更、期初余额批次提交、期初余额批次确认、关账请求发起、关账请求受理、关账请求受理被拒、关账请求结论、关账请求主动取消、年结发起、年结执行、更正凭证过账。高风险操作另在 reauth_ref 与 approval_ref 列记录认证方式、待签内容摘要、时间与设备。期间自动建立按平台产生的状态迁移写审计，actor 取 foundation::SYSTEM_PRINCIPAL_ID、设备标识取 foundation::SYSTEM_DEVICE_ID，安全上下文由 `SecurityContext::system(..., SystemPurpose::General)` 构造；每日对账与关账前校验则仅由 ReconExecutor 构造 `Reconciliation`，两者按 A-02 与 A-03 由阶段 1 冻结。业务用例默认以业务单据日期填 `posting_date`；服务端事务日早于该日期时按既有未来日期错误拒绝，`posting_date` 早于服务端登记日时必须先经授权器断言 `LEDGER_BACKDATE`、重新认证与 `FINANCE_MANAGER` 审批，并把 `BackdateAuthorization` 传给 PostingPort。ledger 对补记缺证明一律拒绝，调用方审计事件必须携带原业务日期、服务端登记时间、修改人、reauth_ref 与 approval_ref；客户端不得提交登记日或伪造证明引用。

新增指标 4 个，登记入基线第 9.2 节：ep_ledger_posting_duration_seconds 直方图、标签 source_kind；ep_ledger_deferred_vouchers_total 计数器、标签 legal_entity_id；ep_ledger_open_periods 仪表、标签 legal_entity_id；ep_ledger_period_close_window_seconds 直方图、标签 legal_entity_id 与 conclusion。基线已有的 ep_period_close_rejected_total 与 ep_recon_* 由本阶段填充取值。禁止把 doc_no 与 trace_id 作为标签。

日志字段按基线第 9.1 节固定集合，span 名为 ledger.<usecase>。禁止进入日志的内容按同节，凭证金额不属敏感字段但来源单据的行内敏感字段不得随错误上下文外泄。

追踪采样：关账与对账任务、期末结账与财务过账两类高风险操作一律 100%，其余 10%。

### 9.12 本阶段新增决定、偏离基线项与假设

#### 9.12.1 本阶段新增决定，阶段结束时回写共享技术基线

一是金额计算与借贷映射的分层：金额归产生该业务事件或受控更正命令的模块，方向与科目角色归 ledger，普通事件接口为 PostingInput 的计量项。理由见第 9.4.3 节。同条另含映射键的唯一现行形状：JOURNAL_MAP 只以 `(source_kind,measure_key)` 为复合键，每键承载 1..n 个腿，不存在第三个分支轴或 PostingBranch 枚举；来源类型由原 11 个拆补为 17 个，F-50 增加受控 `CORRECTION`，Stage 14 再增加受控 `HISTORICAL_MIGRATION` 至最终 19 个。资金单据冲正由 `PostingPort::post_reversal` 接收 finance 锁内计算的两腿拆分，不进普通映射表；更正凭证由 `post_correction` 按原行累计上限生成；历史迁移凭证只由 crate-private migration writer 按已批准记录生成，亦不进普通映射表。普通来源遵守映射/CHECK/属性三件套，专用来源遵守 CHECK/唯一入口/数据库图/正反测试四件套。

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

冻结决定四（F-51 已关闭）：`DIRECT_EXPENSE_COST` 在总账侧只设一个科目角色；合同、订单、项目维度由成本归集模块按单据所带字段承载，不扩大 `AccountRole` 或 `event_account_bindings` 的键。该取值已是当前唯一实现口径，不再等待 U-H-05 回调，实现方无二次选择。

#### 9.12.4 F-50/F-51 冻结决定与未来正式变更代价

| 编号 | 与本阶段的关系 | 本阶段是否阻塞 | 冻结取值 | 切换代价 |
|---|---|---|---|---|
| U-H-01 科目类别枚举 | 试算平衡不需要，会计恒等与损益归零需要 | 不阻塞 | ASSET、LIABILITY、EQUITY、PROFIT_LOSS 四类，成本类因首版排除制造而不设 | 增删取值改 CHECK，用 NOT VALID 加 VALIDATE 在线完成；已有数据需按新类别重分类，代价为一次数据回填迁移 |
| U-H-02 科目编码规则 | 需要 | 不阻塞 | 一级 4 位数字、二级为一级加 3 位共 7 位、字符集只允许数字、长度上限 64、唯一性范围为法人 | 放宽字符集只改 CHECK，收紧则需回填 |
| U-H-03 科目使用与维护约束 | 需要 | 不阻塞 | 一级科目在其下已有二级科目时不可直接记账；类别、借贷方向、上级在已产生凭证后不可改；停用前置校验为无启用下级且未被角色绑定；新建默认启用 | 放宽为可改需增加历史凭证归属的迁移路径，代价高 |
| U-H-04 期初两条路径的分工 | 需要 | 不阻塞 | 手工录入只在该法人尚无任何凭证且无已确认迁移批次期初时可用，要求借贷平衡，需审批 | 改为可共存需增加两路径的冲突判定，代价中 |
| U-H-05 科目角色清单与未绑定行为 | F-51 已关闭 | 已关闭，不阻塞 | 17 个角色随版本冻结；`DIRECT_EXPENSE_COST` 保持单一角色，合同/订单/项目归集由成本模块承载；未绑定时在取用点阻断该类事件提交，并经 DegradationLedger 登记降级窗口与告警，不做任何启动判定；绑定到停用科目阻断提交 | 改为仅告警放行会使凭证生成时失败进入死信，代价高；细分角色需另立后续版本裁定，不是本阶段选择项 |
| U-H-06 绑定变更的治理 | 需要 | 不阻塞 | 经配置发布通道发布，需财务主管审批，不额外重新认证；对已生成凭证无影响 | 增加重新认证只改用例前置校验，代价低 |
| U-H-07 更正凭证入口 | F-50 已关闭；本阶段实现 | 不阻塞 | 只提供引用同法人已过账原凭证的受控更正凭证；固定科目角色、借贷平衡、逐原行累计不超额；不提供自由分录 | 若未来放开自由分录，必须另立裁定、权限与审计模型，不复用本入口 |
| U-H-08 手工凭证入口 | 本阶段不实现 | 同上 | 不提供入口 | 同上 |
| U-H-09 凭证检索条件集合 | 需要 | 不阻塞 | 在两条日期路径之外提供科目、金额区间、来源事件类型、来源单据号、制单人五项 | 增减过滤字段属向后兼容变更，代价低 |
| U-H-10 顺延提示形态 | 需要 | 不阻塞 | 过账响应与凭证详情一律回带 accounting_period_id、business_date 与 is_deferred 三项；账表与导出携带顺延标识 | 代价低 |
| U-H-11 账表查询口径 | 需要 | 不阻塞 | 三张账表支持单期间与期间区间；已关闭与打开期间在响应中以期间状态字段区分；试算平衡分期初、发生额、期末三段各给一对合计并提供按科目下钻 | 代价低 |
| U-H-12 期间建立口径 | 需要 | 不阻塞 | 会计年度为自然年，末次期间为 12 月期间，提前 7 天自动建立，执行主体为 job-worker 定时任务，建立失败按第 15.3 章告警并由顺延路径兜底补建，新法人首个期间只在首次过账时由第 9.4.4 节第二步的零期间分支按记账日期所属自然月建立，建账侧不另立口径 | 改为非自然年需在 accounting_periods 上引入年度起始月配置并重算 is_fiscal_year_last，代价中 |
| U-H-13 关账界面口径 | 需要 | 不阻塞 | 进度按 completed_batch_count 与总批次数呈现，刷新由前端轮询 GET 详情；取消可用窗口为 conclusion 为空；取消与结论并发按 slot 行锁先到先得 | 代价低 |
| U-H-14 三类事项的呈现位置 | 需要 | 不阻塞 | 三类事项同时进入运维中心与财务侧的关账请求详情，并按 ep-platform-notify 通知发起人与该法人的数据责任人 | 代价低 |
| U-H-15 年结的控制强度 | 需要 | 不阻塞 | 独立审批链，同为财务过账类高风险操作，重复执行按新单据处理并展示历次记录 | 改为与期末结账共用一条链只改审批链引用，代价低 |
| U-H-16 本年利润与未分配利润绑定 | 需要 | 不阻塞 | 由 PROFIT_THIS_YEAR 与 RETAINED_EARNINGS_UNDISTRIBUTED 两个角色承载；本阶段不随交付提供预置科目表模板 | 提供模板属新增数据文件，代价低 |
| U-H-17 记账日期的默认值与补记权限 | 需要 | 已关闭 | 默认业务单据日期；早于服务端登记日须 `LEDGER_BACKDATE`、重新认证及 `FINANCE_MANAGER` 审批，只落开放期间，关闭期间顺延；PostingInput 携带服务端产生的证明引用，审计记录完整五元组 | 已冻结；变更需同步全部 PostingPort 调用方与审计契约 |
| U-A-03 文本长度 | 需要 | 不阻塞 | 按基线第 11.2 节，编码 64、名称 200、备注与原因 2000 | 代价低 |
| U-A-07 科目类别是否允许管理员增删 | 需要 | 不阻塞 | 不允许增删，随版本冻结 | 允许增删需把 CHECK 改为引用配置字典，代价中 |
| U-A-08 期间关账与期初余额录入的默认审批链 | 需要 | 不阻塞 | 关账与年结为财务主管单节点审批，期初余额批次为财务主管单节点审批，一律申请人不可自审 | 代价低，审批链是运行期可配置数据 |

### 9.13 风险与预留

#### 9.13.1 技术风险

风险一：受理与在途写事务等待的次序若被后续重构改动，会在关账快照上留下不可见的缺口，其表现是偶发的期间数据不完整而非报错。控制手段是把第 9.4.6 节的四步次序写成 ADR 并在集成测试第七组用例上做次序断言，同时在 ep-app-ledger 的关账编排上以类型状态表达四步，使跳步无法编译。

风险二：pg_export_snapshot 依赖导出事务长期打开，长事务会拖住数据库的 xmin 推进，影响清理。控制手段是快照事务全程只读、只在校验期间打开，并把其持有时长记入 ep_ledger_period_close_window_seconds，超过 ledger.close.inflight_wait_warn_seconds 的十倍时按规格第 15.3 章告警。

风险三：增量余额表与 voucher_lines 之间可能因缺陷产生漂移，且漂移在报表上表现为正常数值。控制手段是把科目余额一致性列为关账前强制校验的第四类，并在每日校验中同样执行。

风险四：JOURNAL_MAP 的内容正确性无法由代码自证，只能由人对照规格第 5.2 章逐条核对。控制手段是 E-6 的签署核对清单，以及在测试中对每个 map-backed source_kind 断言其涉及的科目角色集合与该表一致。当前核对对象为 16 个普通映射来源下的 61 条唯一 `(source_kind,measure_key)` 规则，每条再核对 1..n 个腿；`YEAR_END_PL_CLOSING`、`CORRECTION`、`HISTORICAL_MIGRATION` 三个受控特殊来源必须在映射表零行并分别走专用测试，不得把它们压成伪规则，也不得把普通规则压回“每来源一行”而丢失多腿或 requiredness。

风险五：关账前强制校验的分批规模、单批时限与单查询资源上限在阶段 14 认证前只有冻结取值，客户实际数据量超出基准时可能反复判定为校验未完成而使关账无法通过。控制手段是规格第 10.2 章已给出重取方法，本阶段在配置上把六项做成可热更，并在校验未完成事项中载明触发的具体上限值以便现场重取。

风险六：顺延入账使期间数据不是严格的发生期口径，属规格第 21.20 章已登记的风险。本阶段的控制手段限于两条检索路径与顺延标识，不做追溯重述，界面不使用发生期一类措辞。
风险七：ep-platform-recon 本体在 9a 段交付，而其最重的使用者关账前强制校验编排在 9b 段，中间隔着阶段 8、6、7、10、11 五个阶段，其中阶段 7、8、11 三个陆续注册各自的 ReconCheck，阶段 6 与阶段 10 不注册。若本体的分批语义、快照传递与差异事项模型在此期间被各阶段各自变通，9b 段的关账编排会拿到互不一致的实现。控制手段是把 A-06 冻结的三个契约签名写进 CI 的接口快照断言，9a 段交付时即以内置校验项跑通每日调度的一次完整执行，各阶段注册后即刻纳入每日对账并在其退出条件上留证。


#### 9.13.2 为后续阶段预留的扩展点

一是 VoucherSourceKind 与 source_sequence_no 的组合已按 F-50 实装受控 `CORRECTION`，并由 Stage 14 的 092600 追加 `HISTORICAL_MIGRATION` sequence 1/2 与双向迁移证据图；U-H-08 的手工自由凭证不实现且不得复用任一专用入口。以后新增普通或专用来源分别按第 9.4.1 节对应的三件套或四件套同批约束执行，不再以不得新增封住，也不因此单独升主版本。

二是 TotalAccountBalanceProvider 是子账与总账勾稽的总账侧唯一入口，子账侧统一经阶段 10 定义的 ep-contract-finance 的 ReconciliationItemQuery 接入；inventory 与 procure 两个模块在阶段 8 与阶段 7 各在本模块的 ep-contract-* 中定义并实现本模块的子账余额端口，即 ep-contract-inventory 的 StockValueSubledgerBalancePort 与 ep-contract-procure 的 GrniSubledgerBalancePort，阶段 10 只注入，finance 与 invoice 两个模块的其余八项子账侧取自阶段 10 自有表、不经这两个端口，两条路径都不需改动本阶段代码。

三是 `AccountRole` 的 17 个取值在首版全部冻结，`DIRECT_EXPENSE_COST` 不预留角色限定符，合同、订单、项目归集只在成本模块表达。后续版本若确需按费用类别细分，必须另立裁定并同步迁移、绑定键、映射与历史兼容方案；本阶段不得提前扩展。

四是 ledger.posting_trigger_event_types 的 13 行由本阶段的种子迁移一次写全，受理前提二的判据在本阶段即已生效；该迁移由 xtask configdoc 从事件目录生成并由 CI 逐字比对，各业务模块既不追加回填迁移也不在启动时比对，不需改动本阶段代码。

五是 account_period_balances 已预留 is_opening_fixed，若后续版本恢复受控反结账，只需把固化位回退并重算，不需改表结构。

六是多账簿、辅助核算维度、过账模拟、合并抵消、多币种与汇兑损益一律不在本阶段留任何半成品字段，按规格第 5.7 章延期，避免留下不承载语义的空列。
