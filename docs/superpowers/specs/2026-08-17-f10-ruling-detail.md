# 裁定 F-10 的详本：剩余八条待裁的一次性处置

**方法**：四簇并行起草（影响面驱动机制、财务补偿、配置与权限、运维与验收），
每簇配一个默认判其不成立的**反方**证伪，最后收口。反方共报 81 条（必改 39、应改 38、登记即可 4），**凡标必改的一条不留原样**。

最终裁 **27 条**，撤下 **22 条**，仍须使用方拍板 **12 条**。

**本文是详本。摘要与两处前提更正见 `00c-gap-ruling.md` 的「### F-10」。**

---

## 一、结论提要

最要紧的一条：主控交办文假设「合同终止与发票红冲的下游处置都挂在影响面机制上」，这条不成立，必须现在改。红冲释放核销转预收（B 簇硬伤二）要求与红字凭证共用同一次期间解析、子账腿与总账腿落在同一事务（计划10:802、规格:368），而影响面机制是 Outbox 异步驱动（A 簇），异步做红冲会当场把子账腿甩到另一个期间。因此本轮把影响面机制只挂在合同终止一个上游事件上，发票红冲的下游处置在 register_invoice_reversal 同事务内直接完成，两条路互不相通。第二条：A-6「终止列为第七类高风险操作」本轮撤下——「六类」口径在 specs/ 下逐字命中 36 处（规格 8、PRD 25、盘点 2、AI 设计稿 1），其中规格:1330 与 :1433 是认证判定文本、规格:1835 是性能认证的负载构成（六类改七类要重标定发生频次并可能重跑 A.2 基线）、PRD:2889 与 :4510 两条待决项以「六类不含这三项」为立论前提；同时它的原判据「client_capability_values 行数仍为 72」直接撞使用方已表态的 F-09-3（00c:2659 逐字「由 18 个能力域乘 4 端共 72 格变为乘 5 端共 90 格」）。终止须经审批这半条本轮照裁（新增 chain_kind = TERMINATION），是否升为第七类高风险请单独拍板。第三条：本轮共裁 24 条，撤下 13 条，四份反方报告标的 22 条必改一条不留原样。第四条：三条机制骨架站得住并已修好——影响面处置台账（照抄正向派生编排与 recon_discrepancies）、往来台账改追加式加勾稽按期间累计、部分红冲加发票行明细同批交付。第五条：本轮所有裁定的落地都压在阶段 3、6、7、9b、10、12 六个阶段上，五类计数（表、迁移、错误码、事件、退出条件）必须入卷时逐份回去数，本卷已因凭记忆复述计数栽过三次，这是第四个高危点。

---

## 二、最终裁定（27 条）

### 规格级变更（13 条）

#### A-1 影响面处置台账：落库对象、生命周期与闭合判定

**裁定**：新增两张表落 platform_core，由阶段 3 的 3b-2 批交付，形状照抄两处已验证的现成机制，不另造。表一 impact_assessments（source_module、source_doc_id、source_doc_version、source_event_type、reason、status CHECK in RUNNING/DONE/FAILED、item_total、item_done、started_at、finished_at），唯一约束在 (legal_entity_id, source_module, source_doc_id, source_doc_version, source_event_type)。表二 impact_disposition_items（impact_assessment_id 同 schema 外键 ON DELETE RESTRICT、impact_rule_code、target_module、target_doc_id、target_doc_no、target_doc_line_no、disposition_kind CHECK in AUTO_CLOSE/AUTO_CANCEL/MANUAL_DECISION/INFORM_ONLY、state CHECK in PENDING/DISPATCHING/DONE/DEAD、attempts、available_at、last_error、idempotency_key、process_task_id、decided_by、decided_at、decision_reason），唯一约束在 (impact_assessment_id, impact_rule_code, coalesce(target_doc_id, impact_assessment_id), coalesce(target_doc_line_no, 0))。两表带 legal_entity_id 并按基线第 3.8 节建行级策略，不进 unpoliced_table_registry。

对反方四条必改的修正，逐条：
（一）撤销 WAIVED 状态。反方指出全簇未定义豁免走哪条审批链、approval_ref 从何而来。经核，WAIVED 本就是多余的：A-7 的三类 MANUAL_DECISION 项其完成手段本身就含「明确不处置并说明理由」，该结论以 state = DONE 加非空 decision_reason 表达即可，不需要第二条出口，也就不需要审批凭据。据此表上不设 approval_ref 列，新增表级 CHECK：disposition_kind <> 'MANUAL_DECISION' or state <> 'DONE' or decision_reason is not null。
（二）补 DEAD 的修复重放出口，照抄正向第 7 步。计划06:433 逐字「7. 人工修复后可重放该批次，重放按第 3.3 小节的两道唯一约束去重，不产生重复单据」。据此：批次 status = FAILED 时提供 POST /api/v1/platform/impact-assessments/{id}/actions/replay，把全部 DEAD 项的 attempts 归零、state 置 PENDING、批次回 RUNNING，去重由本条两道唯一约束保证。不补这一步，一项八次失败即合同永远停在 TERMINATING 且按 A-12 不能开票不能发货，比不做更糟。
（三）item_total 改为按目录算定，不按注册表算定。impact_disposition_items 的项集由编译期常量目录（A-2 的七条）确定：已注册规则调 assess 产出实项，未注册规则按目录建一条占位项（target_doc_id 与 target_doc_no 留空、state 恒 PENDING、不计入 item_done），逐字照抄正向计划06:429 对采购需求派生项的既有写法。该阶段接线后由该规则的 assess 重算：返回空则占位项置 DONE 并写 decision_reason「无适用对象」，否则展开为实项。这既消掉了反方指出的「未注册即不产出会让合同在四类未处置时直接进 TERMINATED」的真实窟窿，也不需要注入任何 ImpactRule 替身。
（四）闭合判定唯一且只有一条：item_done = item_total 且不存在 state = 'DEAD' 的项，此时批次置 DONE；item_done 只统计 state = 'DONE'。存在 DEAD 项时批次置 FAILED，上游对象保持处置中并在界面显示待人工修复。

平台 crate ep-platform-impact 三个契约名 ImpactRule / ImpactRegistry / ImpactAssessor，与裁定 A-06 的 ReconCheck 一一对位。生命周期四段：上游动作只写自身状态与一条 Outbox 事件；job-worker 的 platform.impact_assess 消费者以 inbox_consumptions 的 (consumer, event_id) 唯一约束保证只处理一次，在一个事务内建批次与全部项；此后每项一个独立事务调 dispose，Idempotency-Key 取该项 id，失败按基线第 6.2 节八档退避，八次全败置 DEAD 并写 dead_letters；闭合按上条。首版规则集冻结为七条，全部挂 clm.contract.terminated.v1 一个上游事件，其余上游事件（含发票红冲、合同变更、续签）本轮一律不接入。

**验收判据**：一、结构（机检）：迁移后两表存在且列与 CHECK 与本裁定逐字一致；db/checks 的未受策略表登记检查对两表返回零行。
二、闭合判据的反向断言（机检，防恒真的关键，四条中两条为否定断言）：造一份七类下游俱全的合同并发起终止后——①断言逐条规则的 assess 产出条数等于该规则的期望值（订单行 3、交付节点 2、交付确认 1、收款计划 2、采购需求 1、项目任务 1、销项发票 1，共 11 项），不得写成「item_total 等于各 assess 返回条数之和」这种把被测代码的构造方式当断言的恒真式；②任意一项停留在 PENDING 时断言合同不得到达终态；③把一个 MANUAL_DECISION 项在 decision_reason 为空时置 DONE，断言被表级 CHECK 拒绝；④补齐后断言批次置 DONE 且合同到达终态。
三、DEAD 重放（机检）：人为使一项八次失败后，断言批次为 FAILED、合同仍为 TERMINATING；调用 replay 后断言该项回到 PENDING、批次回 RUNNING，且重放不产生第二条同 (impact_assessment_id, impact_rule_code, target_doc_id, target_doc_line_no) 的项。
四、目录一致（机检）：xtask configdoc --check-impact-catalog 对 docs/impact-catalog.md 与编译期常量表逐字比对通过，比对形状照抄现有 xtask configdoc 对 docs/event-catalog.md 的比对；目录条数恒为七，与各阶段注册数无关。

**动到**：规格第 5.2 章 CLM 条目（规格:288）与第 8 章新增终止及其影响面闭合的语义；PRD 新增 3.5.5「影响面处置台账」一节；阶段 3 建两表、建 ep-platform-impact、建 platform.impact_assess 消费者与其 inbox_consumptions 幂等键、新增 replay 端点（落 3b-2 批）；阶段 6、7、10、12 各实现自己的 ImpactRule；00b 第 1.2 节 crate 清单与第 1.3 节依赖边；docs/data-dictionary.md 的 platform_core 节表条目数加二。连带：不得为本机制新增 crates/foundation/src/id/marker.rs 的标记类型（该清单按 A-01 冻结 22 项、由 xtask archcheck 的 foundation-frozen-items 逐项断言），target_doc_id 一律取裸 uuid 加 target_module: ModuleCode；计划06:524 端点响应体由「状态视图与已派生单据处置清单」改为「状态视图与终止评估批次 id」并新增 GET /api/v1/clm/contracts/{id}/impact-assessment；本裁定不动 A-06 的对账框架一个字，ReconCheck 注册项数仍为十五。

#### A-2 下游影响关系从哪来：取静态声明，否决运行期反查

**裁定**：取静态声明。影响关系由被影响模块在自己的 ep-app-<m> 内实现 ImpactRule 并在 apps/job-worker/src/wiring/ 注册，assess 与 dispose 只访问本模块 schema，确需外部事实时走 00b 第 1.3 节通道一的端口 trait。注册形状逐项照抄 A-06 的 ReconCheck，阶段 7、8、9b、11 已这么做了四轮。规则集是闭合枚举不是开放协议，恰七条，code 取值与条数写进新增的第五份登记文件 docs/impact-catalog.md。

否决运行期反查有三条硬证据：一，00b:118 逐字「禁止跨模块直接读写业务表」，一个扫全库外键找下游的组件过不了门禁；二，外键反查在最关键那一支上返回零行——procure.purchase_requisitions 对合同没有 uuid 引用列，只有一列文本 source_idempotency_key（计划07:588 逐字 CONTRACT:{contract_id}:{contract_line_id}:{contract_version}），而采购需求恰是这次被漏掉的两类之一；三，platform_audit.audit_events 的固定列（00b:667）无 correlation_id、request_id 与 trace_id，跨单据的链在审计里拼不出，Outbox 有保留期清理不能当追溯底本。

对反方两条必改的修正：
（一）机检面改指。反方指出 db-pg-one-schema-per-file 的判定面按 00b:118 限在 crates/adapter/db-pg/src 之内，而 ImpactRule 实现落在 ep-app-<m>，该规则判不到被测对象，原判据一恒真、判据二的负样例做不出来。改为两条真判得到的：其一，xtask archcheck 已有的 crate 依赖方向规则（00b:112 逐字「禁止 ep-domain-A 依赖 ep-domain-B、ep-app-B 或 ep-contract-B。跨模块只走 ep-app-A 依赖 ep-contract-B」），该规则以 cargo metadata 为输入、判定面覆盖 ep-app-<m>，负样例可构造；其二，ImpactRule 的取数一律经本模块仓储，仓储实现仍落 crates/adapter/db-pg/src，原 db-pg-one-schema-per-file 在那一层照常生效，这一层写成回归断言而不是本机制的专属门禁。
（二）覆盖面显式摘出规格第 8 章第 13 步。反方指出原判据要求覆盖第 13 步（记账与结账，其已发生事实是已过账凭证），而 A-7 明确不设凭证处置项，该判据恒假。据此本裁定把覆盖面写死为：PRD:1024-1028 的五类派生对象，加规格第 8 章第 7 步（交付）与第 8 步（开票）的两类已发生事实，合计七类，与七条规则一一对应；第 13 步的已过账凭证显式排除，理由与 A-7 同（更正手段只有红冲与更正凭证，红冲是第 3、7 两条的自动后果，单设凭证项会造出永远闭合不了的项）。

**验收判据**：一、机检：ImpactRegistry 可注册的 code 全集恰为 docs/impact-catalog.md 的七条且逐字一致（目录是编译期常量，与各阶段实际注册数无关，见 A-11 的逐阶段注册数判据）。
二、机检（否定断言）：构造一个直接依赖别模块 ep-app 或 ep-domain 的 ImpactRule 负样例，断言 xtask archcheck 的依赖方向规则报错；构造一个在本模块仓储实现里读别模块 schema 的负样例，断言 db-pg-one-schema-per-file 报错。两条负样例本身入 CI，否则门禁恒绿。
三、评审（登记入 00b 第 12.1 节 delegated 段）：七条规则对上述七类的逐条覆盖表，缺一不通过；举证格式按 00b 第 12 节通则第六条，承接方为 A-7 的七条 code。本条如实登记为不可机检：静态声明的漏报点是「某个模块该登记而没登记」，这句话写不出判据，本裁定不写它，只写可判定的替身（条数与目录逐字一致）加这一条评审。

**动到**：00b 第 1.2 节 crate 清单、第 1.3 节依赖边、第 12.1 节 delegated 段登记；新增 docs/impact-catalog.md；阶段 3（框架与登记）、阶段 6、7、10、12（各自实现与 wiring 注册）。连带：docs/ 下登记文件由四份增为五份，00-overview 第 323 行 R6 段逐字「由 CI 校验 docs/error-codes.md、docs/event-catalog.md、docs/metrics-catalog.md、docs/data-dictionary.md 四份登记文件与代码常量表一致」须同批加 docs/impact-catalog.md 并把「四份」改「五份」，各阶段退出条件里凡列举登记文件处一并加；盘点第五节场景二第 16 环引 00b:306 论证「无级联」时遗漏了 00b:307 的跨 schema 复合外键条款，该论证结论不变但入卷时应连带更正，避免日后据那半句反推出「本卷不建跨模块外键」。

#### A-3 「推着走完」是什么：三条驱动杠杆；明确不复用期末关账拦截

**裁定**：甲、状态阻断（主杠杆，新增）。上游对象在处置未闭合前停在显式的处置中状态，闭合才进终态；合同侧即 TERMINATING（A-5）。判据与正向计划06:348 守卫逐字同形。这是唯一真正推得动的杠杆——它把「没处理完」变成一个绕不过去的对象状态，而不是一张可以永远不看的清单。
乙、待办驱动（复用现成载体）。MANUAL_DECISION 项在 platform_flow.process_tasks 写一行 kind = 'HUMAN_TASK'，承载实例为新增流程定义 clm.contract_termination_disposition，impact_disposition_items.process_task_id 回写关联。
对反方两条应改的修正：其一，SLA 的触发机制改指真正的承载体。计划03:1059 逐字「SLA：以 kind = 'SLA' 的定时器表达，触发时不推进实例，只写 process_tasks.sla_breached_at 并产生一条流程时限提醒通知」——提醒由 platform_flow.process_timers 的 SLA 定时器到点触发，改 process_tasks.due_at 不会让任何东西发生。据此本裁定明写：该流程定义必须为每个人工决策节点登记一条 kind = 'SLA' 的定时器，超时时长取 EP__IMPACT__MANUAL_ITEM__SLA_DAYS（出厂默认 5 天），判据随之改为对定时器到点的断言。不写这一条，乙整条落空。其二，max_instance_duration_days 从收益栏移出并给出处置路径：影响面处置里有四条分支天然可能挂数周到数月（是否登记退货、是否红冲、ORDERED 需求怎么办、在制任务收尾还是取消），实例触及上限会按计划03:1065 置 MANUAL_INTERVENTION。本裁定明确：impact_disposition_items 的推进由 ImpactAssessor 驱动，不依赖流程实例状态，实例进入 MANUAL_INTERVENTION 不阻断任何处置项的 dispose，也不阻断合同离开 TERMINATING；process_task 只是待办载体不是闭合驱动。该流程定义的 max_instance_duration_days 取值登记为未决。
丙、前置阻断（复用并扩面），见 A-12。
明确不做：不把处置未闭合做成期末关账拦截项。三条理由——规格:1000 逐字「上述各项是该组件在首版的全部校验范围」，加第四类是改规格第 10.2 章的校验范围本身；00-overview:69 逐字「在计划层新增第三项受理前提是计划凌驾规格」，同一形状本卷已裁过一次；ReconCheck 的注册方与项数被 A-06 冻结为十五个且 category 已由三项收为两项，「处置未闭合」恰是存在性判据，塞进去等于把 A-06 刚做完的收窄推翻一遍。且不做也不漏：处置动作里真会动账的那几腿（红冲、退货、退款）本来就落在既有十五项勾稽的判定面内，差额非零照样拦关账。

**验收判据**：甲（机检）：任一处置项非终态时，重复调用终止端点不改变合同状态，且该合同不出现在已终止列表；全部闭合后迁到终态。
乙（机检）：一次 MANUAL_DECISION 项在 GET /api/v1/platform/process-tasks 的默认待办列表中对其受理人可见；把该人工任务节点的 SLA 定时器 fire_at 拨到过去并触发一次定时器扫描后，断言 process_tasks.sla_breached_at 非空且产出一条「流程时限提醒」通知。不得以修改 process_tasks.due_at 作为触发方式。
丙：见 A-12 的十条断言。
防恒真的反向判据（机检）：本次改动后断言 platform_core.recon_check_definitions 中不出现任何与影响面处置有关的 check code，且 ReconRegistry 注册项数仍恰为十五、category 仍恰为两个取值——用以证明「不复用关账拦截」真被执行，而不是嘴上说不做实现里偷偷加一项。
明确不设的判据：不写「所有未闭合处置项都会被人处理」这类句子，它不可判定。

**动到**：PRD 新增 3.5.5 节承载三条杠杆；PRD 第 3.6 节状态机随 A-5 改；阶段 3 新增流程定义模板 clm.contract_termination_disposition（含每个人工决策节点的 SLA 定时器登记）与配置键 EP__IMPACT__MANUAL_ITEM__SLA_DAYS；阶段 6 的终止用例接入待办。规格第 10.2 章一个字不动，这是本条的重点之一。连带：新增配置键须进 00b 的配置键登记与阶段 3 的配置项计数，其载体形态受盘点第七节第四档已登记的矛盾影响（11 个 EP__ 业务参数「启动时读取，变更需重启」对 00b:577「运行期可变的业务参数不进配置文件」），本裁定不解决那条，登记为未决；新增流程定义走既有配置发布通道，不触及阶段 13 发布链路；process_tasks.process_instance_id 是 not null，故必须新增流程定义而不是插孤儿行。

#### A-5 U-E-12 终止的状态机：新增 TERMINATING 态并补一条 EFFECTIVE 入边

**裁定**：一、合同状态集新增 TERMINATING（中文名「终止处置中」），非终态；clm.contracts.status 的 CHECK 同批加入。
二、新增四条边：1) IN_PERFORMANCE → TERMINATING，触发者为合同责任人，守卫为 TERMINATION 审批链结论通过、审批人不等于发起人、终止原因非空、乐观锁版本匹配；2) EFFECTIVE → TERMINATING（本裁定新增，卷宗原本没有），守卫在上条基础上追加 derivation_state = FAILED；3) TERMINATING → TERMINATED，触发者为系统，守卫为该合同的 impact_assessments.status = 'DONE' 且 item_done = item_total；4) TERMINATING → TERMINATING 自环，存在 DEAD 项时批次置 FAILED、合同保持 TERMINATING 并在界面显示待人工修复，与 PRD:1072 的已生效自环同形。
三、COMPLETED 不开终止入边（其定义是全部交付节点确认完成且全部收付款期次结清，无在途下游可处置）；DRAFT、PENDING_APPROVAL、PENDING_SIGNATURE 的退出仍走 VOID 与在审撤回，不改。
四、在审撤回沿用计划06:891 的取值，按平台审批链的撤回能力承载，不另设动作。
五、PRD:1074 的触发者由「待定」改为合同责任人、触发条件由「条件见开放问题」改为指向本裁定；PRD:1076 的「已派生单据的处置见开放问题」改为指向 A-7；PRD 附录乙 U-E-12 由「待决」改「已裁定」。
第二条第 2 款的理由必须说透：按计划06:891 现文只允许自 IN_PERFORMANCE 发起，而计划06:431 逐字规定派生失败的合同「保持 EFFECTIVE 并在界面显示待人工修复」——正向派生死在半路的合同永远到不了 IN_PERFORMANCE，于是永远没有终止出口，恰恰是最该被终止的那一类（建错了、派生炸了、下游一半在一半不在）在现有设计里无路可走，正中 PRD:4531 逐字「不决策则错误合同只能带着派生单据长期挂账」。补这条边只花一行状态表。
中间态而非一步到位，是本簇能不能推得动的关键：一步到位意味着合同在处置开始那一刻已是终态，此后清单闭不闭合对合同状态毫无影响，机制立刻退化成一张可以永远不看的清单，那就是现状。

**验收判据**：全部可机检。一、clm.contracts.status 的 CHECK 取值集合含 TERMINATING。二、逐边断言四条：从 IN_PERFORMANCE 发起成功；从 EFFECTIVE 且 derivation_state = FAILED 发起成功；从 EFFECTIVE 且 derivation_state = RUNNING 发起被拒并返回 CLM.CONTRACT.INVALID_STATE_TRANSITION；从 COMPLETED 发起被拒并返回同码。后两条为否定断言。三、闭合前断言：处置未闭合时 TERMINATING → TERMINATED 不发生，重复调用终止端点仍返回同一批次 id（幂等）。四、自环断言：人为把一项置 DEAD 后，批次为 FAILED、合同仍为 TERMINATING、界面呈现待人工修复。

**动到**：规格第 5.2 章 CLM 条目（规格:288）新增终止语义与其闭合要求；规格第 8 章不新增步骤（终止是例外路径不是闭环步骤），但在第 3 步后加一句反向指引；PRD 第 3.6 节状态机表新增四行、改 1074 与 1076 两行；PRD 附录乙 U-E-12 改「已裁定」；阶段 6 第 4.2 小节状态机表与 clm.contracts 的 status CHECK 及其由 Rust 类型导出的 serde 派生类型（计划06:329）。连带：计划06 第 11.3 小节 U-E-12 行整行重写，「仅允许自 IN_PERFORMANCE 发起」作废；凡按「合同已终止」判定的下游前置阻断谓词全部改为「已终止或终止处置中」，落点与例外见 A-12；clm.v_contracts_dataset 的 status 列取值域变了，须同步阶段 11 的 reporting.dataset_fields 登记与 reporting-dataset-signature-matched 自检的期望值；PRD:4624 的 U-J-13 与本条关联但不被本条关闭，处置见 A-7 连带一。

#### A-7 五类派生对象加两类已发生事实的逐类处置：七条影响面规则

**裁定**：首版 ImpactRegistry 目录恰七条，upstream_event_type() 全取 clm.contract.terminated.v1。逐条：
1. CLM_TERM_SALES_ORDER_LINE（ep-app-sales，阶段 6）：取该合同派生的 sales.sales_order_lines 中 status ∈ {OPEN, PARTIALLY_DELIVERED} 的行；delivered_quantity = 0 取 AUTO_CANCEL 置 CANCELLED，> 0 取 AUTO_CLOSE 置 CLOSED 并写关闭原因「合同终止 <合同编号>」与审计（守卫按计划06:357）。订单头：全部行零交付时置 CANCELLED，否则置 CLOSED。**按反方应改补一项处置面**：同批取消该订单行底下 status = PENDING 的 sales.delivery_schedules 分批交付行。理由是计划06:453 逐字「未拆分的订单行在派生时即建立一条分批交付行……因此系统中不存在没有分批交付行的订单行」，不取消则已终止合同会一直出现在交付逾期清单与看板里（规格第 8 章第 14 步逐字要求「交付取合同交付节点与订单分批交付的按期完成率和逾期清单」），与 A-8 第四条为提醒堵的是同一个洞。该处置落在本条规则内，不新增第八条规则。
2. CLM_TERM_MILESTONE（ep-app-clm，阶段 6）：取 clm.contract_milestones 中 status ∈ {PLANNED, ACTIVE}，AUTO_CANCEL 置 CANCELLED（取值已在计划06:181 的 CHECK 内，零 DDL 改动）。
3. CLM_TERM_DELIVERY_CONFIRMATION（ep-app-sales，阶段 6）：取该合同名下已 CONFIRMED 的交付确认单，MANUAL_DECISION，不自动动账；人工在既有两条路径里选一条并写进 decision_reason（登记销售退货单，或明确不退并说明理由）。系统不代选，因为交付确认单按 00c:421 逐字「不设作废态，冲正一律经销售退货单」。
4. CLM_TERM_PAYMENT_SCHEDULE（实现落 ep-app-clm，注册与验收随阶段 10）：取 clm.contract_payment_schedules 中未被开票占用的期次，是否已开票经阶段 10 新增只读端口 ep_contract_invoice::ReceiptPlanBillingQuery::billed_period_nos 判定；AUTO_CANCEL，期次 status 置 VOIDED 并写 void_reason（两列新增见 A-8）。实现落 clm 而注册随阶段 10，逐字照抄计划06:474 对 is_fully_credit_noted 的既有做法。
5. CLM_TERM_PURCHASE_REQUISITION（ep-app-procure，阶段 7）：**按反方必改扩取数范围**。原文只取 source_idempotency_key 以 CONTRACT: 为前缀者，会漏掉同一合同底下的另外两支——计划07:589-592 的四类来源键分别为 CONTRACT:、SALES_ORDER:、PROJECT_TASK:、STOCK_SHORTAGE:，而该合同派生的销售订单行可经 actions/raise-from-sales-order-line 起需求（SALES_ORDER: 键）、该合同派生的项目任务也可起需求（PROJECT_TASK: 键），两支都挂在这张合同底下。据此 assess 取数改为三支：CONTRACT:{contract_id}: 前缀者；SALES_ORDER: 键且该 sales_order_line 属本合同者；PROJECT_TASK: 键且该 project_task 的 source_contract_id 等于本合同者。status ∈ {PENDING, PARTIALLY_ORDERED} 取 AUTO_CLOSE 置 CLOSED（走计划07:487 已有的边，不新建边）；ORDERED 取 MANUAL_DECISION（该表守卫本就只允许采购员手工关闭 ORDERED，且已下达给供应商，是否协商取消只能人定）。
6. CLM_TERM_PROJECT_TASK（ep-app-project，阶段 12）：取 project.project_tasks 中 source_contract_id 等于该合同且 status ∈ {NOT_STARTED, IN_PROGRESS}；NOT_STARTED 取 AUTO_CANCEL 置 CANCELLED 且 cancel_reason 取「合同终止 <合同编号>」（守卫已在计划12:450，零 DDL 改动）；IN_PROGRESS 取 MANUAL_DECISION。
7. CLM_TERM_SALES_INVOICE（ep-app-invoice，阶段 10）：取该合同名下已开具且未被作废或全额红冲的销项发票，MANUAL_DECISION，不自动动账；人工在既有的作废与红字冲销两条路径里选一条或明确不冲，写进 decision_reason。系统不判定该用哪条，PRD:2411 已明写本系统不判定。
明确不设第八条「凭证处置项」，并把规格第 8 章第 13 步从覆盖面显式摘出：已过账凭证的更正手段只有红冲与更正凭证（规格:1443），红冲是第 3、7 两条动作的自动后果（按规格:311 的事件-分录表生成），单设凭证项会产生一个没有任何可用动作、永远闭合不了的项，会把闭合判定变成永假，比不设更坏。
「已发生的一律不回退」这句计划06:891 的原意保留；本裁定改的只有一点：不回退不等于不列出、不追踪、不推。

**验收判据**：一、机检：docs/impact-catalog.md 恰含上表七个 code 且与编译期常量表逐字比对通过，条数断言为七不多不少（目录条数，非注册数）。
二、机检（逐条注入，七对断言）：为每条 code 各造一个应命中对象与一个不应命中对象（例：已 CANCELLED 的项目任务、已 CONFIRMED 且已退货的交付确认、status = ORDERED 且已下达采购订单的需求、已被全额红冲的销项发票、经 SALES_ORDER: 键起且不属本合同的需求），断言 assess 只产出前者。每对里的「不应命中」为否定断言。第 5 条另加一条肯定断言：三支来源键各造一条应命中需求，三条全部产出。
三、机检（否定断言，且以表级 CHECK 为被测机制，见 A-1 第一条的 CHECK）：四条会产出 MANUAL_DECISION 项的分支（第 3 条、第 7 条恒为该类，第 5 条的 ORDERED 分支与第 6 条的 IN_PROGRESS 分支同样产出该类，共四条不是三条），其项在 decision_reason 为空时置 DONE 的写入被拒。
四、机检：第 1 条处置后断言该订单行底下不存在 status = PENDING 的 delivery_schedules 行，且该合同不出现在交付逾期清单取数结果中（否定断言）。
五、评审（登记 delegated）：七条对五类派生对象与规格第 8 章第 7、8 两步两类已发生事实的逐条覆盖表，缺一不通过；第 13 步显式排除并写明理由。

**动到**：PRD 新增 3.5.5 节承载本表；阶段 6 实现三条（1、2、3）；阶段 7 实现一条（5）；阶段 10 实现两条（4 的注册与 7）并新增只读端口 ReceiptPlanBillingQuery；阶段 12 实现一条（6）。四个阶段各自的 crate 清单、wiring 目录与退出条件同批改。连带：一、U-J-13 与本裁定正面冲突必须同批加限定语，计划12:887 的临时取值逐字「保留既有任务，不自动作废」须加「本取值只适用于合同变更或续签导致的派生计划变动；合同终止场景按 A-7 的 CLM_TERM_PROJECT_TASK 处置，不适用本取值」，PRD:4624 保持待决但加同一句；二、计划07:487 那条边的守卫措辞改为「来源单据作废或来源合同终止」，且**必须同批把该行后半句「来源作废由 Outbox 消费者触发并写审计」的触发方改为由 ImpactRule::dispose 触发**，只加六个字会把 A-9 第五条明令要防的双写原样留在计划里；三、阶段 10 新增的 ReceiptPlanBillingQuery 与计划10:1300 已登记的风险「clm 收款计划已开票金额的回写方式未定」落在同一处，本裁定只要一个只读查询不解决那条回写，阶段 10 须在同一小节说明两者关系；四、第 3、7 两条产生的销售退货与红冲会真实动账，动账后账不平由既有十五项勾稽照常拦关账，这是 A-3「不复用关账拦截也不漏」的具体落点。

#### B-1 U-H-07 更正凭证入口：开，且限定为必须有来源凭证的重分类更正

**裁定**：开入口。规格在五处把更正凭证当成已存在的手段向客户承诺（规格:361「红字冲销与更正凭证只追加不覆盖」、规格:187、规格:1656、规格:1443 第 19 章退出条件、规格:1709 第 22 章验收条目），PRD:2988 也已写成既有能力，而计划09:935 逐字「本阶段不实现／不提供入口」，PRD:4578 逐字「不决策则首版没有任何过账更正入口」——不开就是规格自相矛盾且对客户失信。
落地物：新增 ledger.correction_vouchers 与 ledger.correction_voucher_lines 两张表（单据类型码 CORR）；VoucherSourceKind 由 17 增至 18，新增 CORRECTION；PostingPort 新增 post_correction(tx, ctx, CorrectionInput)，不查 JOURNAL_MAP、分录行由入参给定，与既有 post_reversal 同属不经映射的第二类入口；source_sequence_no 恒为 1，ck_vouchers_source_sequence 不放开。
**按反方两条必改重写守卫**：
（一）来源引用由行级改为凭证级。原守卫二「每一行必须且只能引用一条已过账凭证行」加守卫三「对同一被引用行的累计更正金额不得超过该行金额」互相打死——最标准的两行重分类（借科目乙 X、贷科目甲 X）两行都得引用原凭证那条「借科目甲 X」，累计 2X > X 当场被拒，连它自己写的正向用例都过不去；而若允许随便指一条尚有余量的行，守卫三又不构成任何实质约束。改为：每张更正凭证必须且只能引用一张已过账凭证（source_voucher_id 非空、同法人），错误码 LEDGER.CORRECTION_VOUCHER.SOURCE_VOUCHER_REQUIRED；对同一被引用凭证的累计更正借方合计（含本次）不得超过该凭证的借方合计，错误码 LEDGER.CORRECTION_VOUCHER.AMOUNT_EXCEEDS_SOURCE_VOUCHER，按被引用凭证逐张独立计算不做链式回溯，更正凭证自身可被后续更正引用。
（二）勾稽守卫由「全局十项差额为零」改为「触及项不劣化」。原守卫五在唯一需要更正凭证的场景下恒拒：规格:1003 逐字「差异清零前不得关账」，差额已存在的那一刻正是要用更正凭证的时候，而原守卫对该场景下的任何更正凭证一律返回 RECONCILIATION_BROKEN，等于开了一个只在不需要它的时候可用的入口。改为：同事务内只重跑本次分录所触及科目角色对应的勾稽项，判据为该项差额的绝对值不大于提交前的值（不劣化），任一项劣化则整笔拒绝，错误码 LEDGER.CORRECTION_VOUCHER.RECONCILIATION_WORSENED，details 回带勾稽项、更正前差额、更正后差额。全局十项仍由既有每日对账与关账前强制校验兜底，不在提交口做。
（三）撤下期初余额批次这条来源（见 dropped），source_ref_kind 只留 VOUCHER。
其余守卫保留：借贷合计相等；科目取该法人 ledger.accounts 中启用且可直接记账者（一级科目在其下已有二级科目时不可直接记账，计划09:931 的 U-H-03 临时取值）。控制强度：属规格:1050 的财务过账高风险类，必须重新认证加审批，审批链取值随 C-3。期间按计划09 第 9.4.4 节解析，落已关闭期间时顺延，响应回带三项（U-H-10 口径）。端点 POST /api/v1/ledger/correction-vouchers（必带 X-Reauth-Token）、GET /{id}、GET 列表；桌面端可写，移动端只读。交付段落 9b，不落 9a、不进 T0（守卫需阶段 10 的子账侧接入）。
代价判定：计划09:935 那格「属破坏性变更，需升主版本」作废，按计划09:970 逐字「不需改表结构……也不因此单独升主版本」与计划09:333 的三者同批 CI 约束裁。

**验收判据**：阶段 9b 集成测试新增七条，一律真实 PostgreSQL 独占库，负向用例先 RED 后 GREEN 且各自断言具体错误码，只断言 4xx 判为未完成。正向 A：把一笔已过账的直接费用由科目甲重分类到科目乙，更正凭证过账后科目余额表两科目等额反向变动，原凭证行一字未变（由计划09:212 的 REVOKE UPDATE, DELETE ON ledger.vouchers 在库层保证）。正向 B：记账日期落已关闭期间时顺延，响应 is_deferred 为真且 deferred_from_period_id 非空。正向 C（本条修正的关键，改造前恒失败）：在已存在一项勾稽差额的状态下提交一张能把该差额改小的更正凭证 → 通过，且断言该项差额确实变小。负向 D：不引用来源凭证 → SOURCE_VOUCHER_REQUIRED。负向 E：对同一来源凭证累计更正超过其借方合计 → AMOUNT_EXCEEDS_SOURCE_VOUCHER，details 回带原凭证借方合计与累计值。负向 F：提交「借 应收账款 贷 主营业务收入」使应收项差额变大 → RECONCILIATION_WORSENED，details 中应收项更正后差额大于更正前。负向 G：借贷不平 → 拒绝。负向 H：未带 X-Reauth-Token 或审批未通过 → 拒绝。机检两条：xtask configdoc --check-doc-type-codes 对 CORR 通过；ck_vouchers_source_kind 取值集合逐字等于 18 项并与数据字典比对一致。

**动到**：规格第 5.2 章总账功能与期末处理块补一句，明写更正凭证的产生方式与来源凭证引用要求（本条自认要动规格，档位据此由原「计划级新增」上调）；PRD 第 7 节 2977 与 2988 两处改写、附录乙 U-H-07 改已决；阶段 9（第 9.3 节新增两张表与两个迁移、第 9.4.1 节取值 17 改 18、第 9.5.9 节新增 post_correction、第 9.5 节端点加三条、第 9.12.4 节 U-H-07 那格、第 9.13.2 节第一条、9b 交付清单）。**按反方必改补全阶段 9 的五处连带计数与登记**：计划09:41 与 09:793 的「12 张表与 2 个视图」改 14 张表、「16 个迁移文件」改 18；计划09:710 与其退出条件的「8 张带法人列的表」改 10，两张新表须建 rls_*_le 策略并进 tests/rls_matrix（撞裁定 C-05）；计划09:869 的「写审计的动作清单固定 14 个」改 15（更正凭证属高风险类须写审计）；00c 裁定 C-26 的全量类型码表（00c:1463 逐字「任何阶段不得新增未在此表登记的码」）阶段 9 行由 OBB、GV、PCR、YEC 增为五个含 CORR，登记文件归阶段 1、码归阶段 9（原文写落阶段 3 是错的）。另：计划09:333 的同批 CI 约束须补第二类，不经 JOURNAL_MAP 的来源类型其同批物为 CHECK 迁移加专用 Posting 方法加借贷平衡属性测试、无 JOURNAL_MAP 行，不补这条新增 CORRECTION 会当场撞 CI。

#### B-2 U-H-08 手工凭证入口：不开，且回写规格明写排除

**裁定**：不开。PRD:4579 给出的不开代价逐字是「无入口则期初调整与更正无路径」，这两条路径在本轮之后都已存在：期初侧有总账期初余额批次（计划09:509-513，需审批且只在该法人尚无任何凭证时可用）、阶段 10 的往来与预收预付期初导入、阶段 8 的库存期初三个通道；更正侧由 B-1 承载。所以 4579 陈述的代价已被消除，不开的成本降为零。反面代价则是实的且被计划自己点名：计划09:333 逐字「封条挡不住演进，只会让首版排除的调拨、盘点、领用与费用报销日后绕道手工凭证」，而这几项是首版范围冻结的一部分（规格:298 逐字「首版已排除调拨、盘点、领用和质检」）。
规格第 5.2 章补一句：首版总账凭证只由三个来源产生——十类业务事件按事件-分录表的固定映射、期末处理动作（年度损益结转）、更正凭证；不提供任何自由分录入口。PRD 第 7 节 2977 与附录乙 U-H-08 同步改为「不提供，已决」。
落地以两条可机检的封闭性约束加一条陈述承载（**按反方应改，把原第三条由「可机检」降为陈述并更正其失真判断**）：(a) ledger.vouchers 与 ledger.voucher_lines 的写入入口只有 PostingPort::post、post_reversal、post_correction 三个，由 xtask archcheck 断言二表的 INSERT 只出现在 ep-app-ledger 的这三个实现体内；(b) ck_vouchers_source_kind 的取值集合逐字等于 18 个已登记取值，由 xtask configdoc 与数据字典比对；(c) 陈述而非判据：post_correction 的每张凭证必须引用一张已过账凭证，因此不存在完全无来源的凭证；但本裁定不声称这能阻止有人用更正凭证变相记调拨或盘点的账——守卫只保证有来源，不保证语义正当，这一层由审批链与审计证据承担，不设机检，也不写成恒真门禁。原文把 (c) 说成「因此不存在无业务来源的凭证这条路径」是失真的，据 PRD:4579 的原始担心（绕过事件映射）它只削弱不消除。

**验收判据**：机检一：xtask archcheck 新增一条规则断言二表 INSERT 只在三个 Posting 方法实现体内；在测试夹具里插入第四处 INSERT，CI 必须失败——该负向夹具本身入 CI，否则这条门禁恒真。机检二：xtask configdoc 逐字比对 18 个取值与数据字典；人为加第 19 个取值而不同批加 CHECK 迁移与属性测试，CI 必须失败。评审判据：规格第 5.2 章与 PRD 第 7 节各出现一句明写排除的文本，且 PRD 附录乙 U-H-08 状态不再是「待决」，由评审逐字核对，二者缺一不通过。如实登记一处不可判定：见决策 (c)。

**动到**：规格第 5.2 章总账功能与期末处理块（PRD:4579 逐字要求「需回写规格」）；PRD 第 7 节 2977 与附录乙 U-H-08；阶段 9 第 9.12.4 节 U-H-08 那格；阶段 1（archcheck 新增规则与其负向夹具）；阶段 2 与阶段 3（configdoc 比对项）。连带：计划09:970 第一条「已为手工凭证与更正凭证留出位置」的措辞须改，只留更正凭证；本条与 B-1 是同一条决策的两半，必须同一批次落地不得先落一半。

#### B-3 U-D-02 资金单据冲正：采纳既有自建设计并做三处收紧

**裁定**：采纳计划10 第 4.7 节与第 3.2.13 节的既有设计为正式结论并回写规格与 PRD，同时三处收紧：
（一）语义收窄且可判——finance.cash_document_reversals 新增一列 reversal_reason_kind text 非空，CHECK 取值固定五项 NOT_ACTUALLY_RECEIVED、WRONG_AMOUNT、WRONG_PARTY、WRONG_ACCOUNT、WRONG_DATE，不设第六项，即界面上不存在「因发票红冲而冲正到款」这个选项。
（二）明写禁用场景——规格第 5.2 章与 PRD 第 6.12 节各新增一段：销项发票红字冲销时原到款已核销的，一律按 B-6 处理，不得用资金单据冲正解此场景。理由是计划09:595 逐字「post_reversal……语义为按原凭证逐行取反生成红字凭证」，逐行取反必然把银行存款腿一并贷回，而红冲已到款发票时钱还在本方账上，资金账当场与银行对账单不符——这不是次序没写清，是这条路根本不能走。
（三）控制强度落定——属规格:1050 的付款与财务过账高风险类，必须重新认证加审批，审批链取值随 C-3。
其余一字不动：表名、类型码 CDRV、status 锁死 REGISTERED、source_doc_type 取 RECEIPT/PAYMENT/REFUND、唯一约束保证一张资金单据只能被冲正一次、冲正单不可再被冲正、六步执行次序、凭证经 post_reversal 生成。PRD 附录乙 U-D-02 与计划10 第 0.4 节 F-14 两处状态由待决改已决。
**按反方应改更正依据出处**：本条与 B-1、B-2 三者同批的要求，其依据不是「PRD:4498 逐字要求三者一并决策」——PRD:4498 逐字只有「财务负责人，结论需与 U-H-07 更正凭证入口一并决策」，全句未提 U-H-08。三者同批的实际依据是 00c 庚一表原编号 7 行。B-1 与 B-2 的同款引用一并更正。

**验收判据**：落库判据：ck_cash_document_reversals_reason_kind 取值集合逐字等于五项，由 xtask configdoc 与数据字典比对；加第六项而不同批改文档即 CI 失败。用例判据（在计划10 第 8.3 节现有第 17 条基础上加三条）：一、冲正一笔已核销到款后，finance.receivable_settlement_links 出现 reverses_id 非空的反向行、原条目 open_amount 回增到冲正前加该行金额，十项勾稽差额仍为零；二、对同一资金单据第二次冲正 → 拒绝并断言 FINANCE.CASH_DOCUMENT_REVERSAL.SOURCE_ALREADY_REVERSED；三、对冲正单本身发起冲正 → 拒绝。评审判据：规格第 5.2 章与 PRD 第 6 节各出现一句明写「红冲已核销发票不走冲正」的文本，且 PRD 附录乙 U-D-02 与计划10 第 0.4 节 F-14 两处状态不再是待决。如实登记一处不可判定：reversal_reason_kind 的取值是否被如实选择机器判不了，只由审批与审计证据承担。

**动到**：阶段 10（第 3.2.13 节加一列与一条 CHECK、第 4.7 节加一段禁用场景、第 0.4 节 F-14 那格、第 3.6 节迁移清单第 14 号文件相应加列、第 5.4 节端点请求体加 reversal_reason_kind、第 8.3 节第 17 条加三条断言）；规格第 5.2 章财务规则条目新增一段（本条自认动规格，档位据此由原「实现级补充」上调）；PRD 第 6.12 节与附录乙 U-D-02。连带：与 B-6 互为前提须同批入卷，否则规格上会留下「红冲已核销发票该走哪条路」的空洞，那正是盘点断点四的成因；本条给的审批链取值须与 C-3 对齐不得出现两个取值；00c 庚一第 7 行随之标记为已裁。

#### B-4 U-D-09 允许部分金额红冲，销项与进项两个方向同时放开

**裁定**：允许部分金额红冲；作废（VOID）仍只允许全额且只允许一次。全额-only 与规格正文直接冲突：规格:311 逐字要求进项方向登记「供应商开具的价格调整红字发票，以及已登记进项发票的金额、税额更正」，价格调整红字发票按其性质就是部分金额，全额-only 之下这类发票根本登不进去，即计划层的一个临时取值把规格的一条已生效要求做没了。
逐项落地：(1) ck_sales_invoices_status 改为 ISSUED、VOIDED、PARTIALLY_RED_REVERSED、RED_REVERSED；ck_purchase_invoices_status 改为 REGISTERED、PARTIALLY_REVERSED、REVERSED。(2) 两张发票表各加累计列 reversed_net_amount（销项已存在于计划10:210，逐字「为 F-08 改判部分红冲预留」）、reversed_tax_amount、reversed_gross_amount，默认 0，CHECK 各不超过对应原值且 gross = net + tax；销项另加 rolled_back_ratio numeric(9,6) 默认 0，CHECK 不超过 issued_ratio。(3) 撤销唯一约束 ux_invoice_reversals_legal_entity_id_source_invoice_id——它在库层物理封死第二次红冲（计划10:242 那整段推理随之作废）；改由登记时对发票行 SELECT ... FOR UPDATE 后累加加上述 CHECK 兜底；作废互斥与作废只一次改由状态守卫承担，只有 status = ISSUED 且 reversed_net_amount = 0 才允许登记 VOID；防同一张红字发票重复登记新增唯一索引含 red_invoice_no（VOID 行该列为 NULL，PostgreSQL 下多个 NULL 不冲突，不需要部分索引）。(4) 金额关系替换计划10:63 的临时取值：0 < red_net_amount <= net_amount - reversed_net_amount，税额与价税合计同理；税额容差沿用 F-03；red_invoice_no 保持必填。(5) 比例回滚公式替换计划10:737：红冲时本次回增 = round(issued_ratio * red_net_amount / net_amount, 6)，本次红冲使 reversed_net_amount 达到 net_amount 时改取 issued_ratio - rolled_back_ratio 把尾差一次性归位；**按反方应改补 VOID 分支**：作废时本次回增一律取 issued_ratio - rolled_back_ratio（即全部回滚），不代入含 red_net_amount 的公式——该列按第 3.1.4 节在 VOID 行为空，代入即除空。(6) 累计开票比例校验取数改为 sum(issued_ratio - rolled_back_ratio)，范围含 ISSUED 与 PARTIALLY_RED_REVERSED，VOIDED 与 RED_REVERSED 因 rolled_back_ratio 等于 issued_ratio 自然贡献零，计划10:743 的特例句删除。(7) 下游一律按本次金额回滚：invoice_receipt_plan_links 反向行取本次分摊额且对同一 receipt_plan_line_id 的反向行合计不超过原正向行；finance.unbilled_ar_entries 的 DEBIT 行金额由计划10:796 的「原发票 net_amount」改为 red_net_amount；信用占用回退按本次 red_gross_amount；应收应付台账按 B-5 追加 red_gross_amount 的 REVERSAL 条目。(8) 申请单状态回退表三行守卫统一改为按 remaining_ratio 判定（原文引「第 686 行写法」，该行为空行，参照物不存在，改为按第 4.4 节 remaining_ratio 的定义直接表述）。(9) **按反方应改重新定义 SOURCE_ALREADY_REVERSED**：该码原由被撤销的唯一约束触发，现改为在 status ∈ {VOIDED, RED_REVERSED} 时触发，与 RED_AMOUNT_MISMATCH（金额超出剩余可冲）语义不重叠。

**验收判据**：阶段 10 集成测试新增六条，全部真实 PostgreSQL，后三条负向须先 RED 后 GREEN 并断言具体错误码。一、三次部分红冲把一张票冲完：reversed_net_amount 三次累加后精确等于 net_amount，状态两次为 PARTIALLY_RED_REVERSED、第三次迁到 RED_REVERSED。二、属性测试且不带容差的精确相等：对任意拆分序列恒有 remaining_ratio + sum(issued_ratio - rolled_back_ratio) = issue_ratio，且冲完时 rolled_back_ratio 精确等于 issued_ratio；VOID 路径单独一条，作废后该发票的 rolled_back_ratio 精确等于 issued_ratio。三、进项方向登记一张金额小于原票的价格调整红字发票成功，应付条目与存货或成本腿按 red_net_amount 冲减，十项勾稽差额为零——这条直接验规格:311 那句现在登不进去的要求。四、冲完后再冲 → 拒绝，断言 INVOICE.INVOICE_REVERSAL.RED_AMOUNT_MISMATCH，details 回带剩余可冲金额。五、对已部分红冲的票登记作废 → 拒绝，断言 INVOICE.INVOICE_REVERSAL.SOURCE_ALREADY_REVERSED。六、同一 red_invoice_no 对同一原票二次登记 → 拒绝，断言 INVOICE.INVOICE_REVERSAL.DUPLICATE_RED_INVOICE_NO。第四、五、六三条是撤销唯一约束之后「作废只允许一次」与「不得超冲」两条不变量的全部证明，不得只靠代码注释或文档声明。

**动到**：规格第 5.2 章红字冲销与作废事件的附加规则；PRD 第 6.5 节（6.5.2、6.5.3、6.5.5）、第 6.6 节、附录乙 U-D-09。**按反方必改补全阶段 10 与 PRD 的四处漏列落点**：PRD:2393 逐字「同一张发票的作废与红字冲销互斥，只允许其一，且只允许冲回一次」与 PRD 第 6.4.7 节销项发票状态机四行（PRD:2388-2391）；PRD:2417 逐字「原发票 | 是 | 只能选择状态为已开具的销项发票」（部分红冲后原发票为 PARTIALLY_RED_REVERSED，按此条再也选不出来）；计划10 第 4.2 节状态机（计划10:691 逐字「ISSUED 到 VOIDED、ISSUED 到 RED_REVERSED，两者互斥，各自只允许一次……由唯一约束兜底」）；计划10 第 5 节端点表（第 5.2 节请求体与错误码列）。另按反方更正阶段 10 的节号：索引落第 3.5 节不是第 3.4 节（第 3.4 节是 RLS 策略），新增表须按计划10:549 建 rls_<table>_le 策略并进裁定 C-05 的 tests/rls_matrix，「36 张表」这个数须同批改；第 3.3 节视图合计数同批改。连带：规格:311、PRD:2441、PRD:2411 三处「只允许冲回一次」必须同批改；计划10:242 整段推理作废须重写；与 B-8 同批交付，否则 PRD:373 与规格:312 的退货前置校验在首版仍不可判；与 B-5、B-6、B-7 同批，缺任一条部分红冲都落不了库。

#### B-6 硬伤二：已到款又不涉退货的红冲，改为释放核销转预收

**裁定**：红字冲销与作废登记时，若原发票的应收（应付）明细条目已被核销，在同一事务内先释放核销、再追加反向条目；释放额一律为 min(该条目 settled_amount, 本次 red_gross_amount)，按被释放的 finance.receivable_settlement_links.origin 分四路：origin = MANUAL → 追加 reverses_id 反向核销行、把该条目 settled_amount 减去释放额，同时新增一条 finance.advance_receipt_entries 预收条目，金额为释放额，会计期间取红字凭证的期间；origin = AUTO_ADVANCE → 同样追加反向核销行，但不新建预收条目，改为对 finance.advance_receipt_settlement_links 追加 reverses_id 反向行、把原预收条目的 open_amount 回增；origin = REFUND → 阻断提交，错误码 INVOICE.INVOICE_REVERSAL.SETTLED_BY_REFUND_REQUIRES_REVERSAL，details 列出该退款单并提示先冲正该退款单（钱已经出去了转不成预收）；origin = REVERSAL 的行不参与释放。
**按反方两条必改的修正**：
（一）补三处封闭 CHECK 的新取值，否则本裁定要写的行一行都落不了库：计划10:366 逐字「ck_advance_receipt_entries_source_type 取值 RECEIPT、MIGRATION_OPENING」（预付侧同构），新增取值 INVOICE_REVERSAL；计划10:489 的核销关系行 source_doc_type（RECEIPT、ADVANCE_RECEIPT、CUSTOMER_REFUND、CASH_DOC_REVERSAL）新增取值 INVOICE_REVERSAL，四张核销关系表同批；应收应付条目侧的 source_type 见 B-5。
（二）改规格:311 的借贷两栏而不是附加规则栏。规格:311 红字冲销与作废那一行现文借方栏为「销项方向：应交税费的销项税额；应收账款未开票过渡科目，恢复其余额。进项方向：应付账款」，贷方栏为「销项方向：应收账款。进项方向：应交税费的进项税额；存货或原归集的直接费用类成本科目」，两栏都没有预收账款与预付账款。本裁定新增的计量项 released_settlement_amount（销项映射借 ACCOUNTS_RECEIVABLE、贷 ADVANCE_FROM_CUSTOMER，进项映射借 ADVANCE_TO_SUPPLIER、贷 ACCOUNTS_PAYABLE）必须由规格:311 的借方与贷方两栏提供依据，附加规则栏加一句给不了 JOURNAL_MAP 新行依据，计划09 风险四那条「对每个 source_kind 断言其涉及的科目角色集合与该表一致」的核对当场不成立。金额为零时按计划09 第 9.4.3 节第二步不生成分录行，因此不新增 VoucherSourceKind 取值。
资金腿一律不动，银行存款与库存现金在本路径上零变动。后续路径：重开新票时按规格:306 逐字「后续开票时按同一合同的收付款计划自动核销预收账款」自动核销，不需人工搬迁（盘点断点五同时消失）；客户确要退钱的按规格:310 的预收分支登记客户退款。
之所以不能走「先冲正原到款」：计划09:595 逐字说明 post_reversal 按原凭证逐行取反，必然把银行存款腿一并贷回，而款项并未退还客户，资金账当场与银行对账单不符。

**验收判据**：阶段 10 集成测试新增四条加一条属性测试。一、已全额到款后全额红冲：该张凭证内应收账款借贷净额为零、预收账款增加等于价税合计、银行存款零变动；十项勾稽在红冲当期差额全部为零；随后重开同额新票，预收被自动核销、预收余额归零。二、已全额到款后部分红冲 30%：预收增加等于本次 red_gross_amount，该发票条目的 effective_open 等于零，十项勾稽差额为零。三、由预收自动核销而来的核销被红冲：不产生新预收条目，原预收条目 open_amount 回增，advance_receipt_settlement_links 出现反向行，预收台账合计与预收账款科目余额差额为零。四（负向，先 RED 后 GREEN）：该条目已被客户退款核销时红冲 → 拒绝并断言 SETTLED_BY_REFUND_REQUIRES_REVERSAL 且 details 中列出退款单号。属性测试：对任意「开票—分次到款—部分红冲」序列，银行存款科目余额恒等于资金流水台账余额合计，即本路径不写任何资金腿。

**动到**：规格第 5.2 章（规格:311 该行的借方与贷方两栏各新增一项，附加规则栏另加一句「已核销款项转预收或转预付」；退款事件规格:310 的两个分支适用条件随之改写）；PRD 第 6.5.3、6.9.2、6.11（预收台账条目的产生方式）、6.12.2；阶段 9（第 9.4.3 节两个来源类型各加一个计量项与其 JOURNAL_MAP 行，借贷平衡属性测试同批）；阶段 10（第 3.2.5 与 3.2.6 预收预付两表的 source_type 取值、四张核销关系表的 source_doc_type 取值、第 3.6 节迁移清单、第 4.9 节之前新增一节「红冲的核销释放」、第 6.1 节 register_invoice_reversal 事务内写入清单加三项、第 8.3 节）。连带：规格:310 的「原款项已核销至应收或应付且退货部分已完成红字冲销的冲减应收账款或应付账款」这一分支在销项与进项两个方向都变为不可达，须改写为按预收预付分支处理否则规格里留一条死分支；PRD:2726「关联退货单据 | 是」必填、PRD:2738、PRD:2773 三处必须同批改为条件必填——由红冲转入预收而发起的退款没有退货单，现行三处必填把这条路堵死；计划10:786 的可退上限「上界二，来自退货」须改为「存在关联退货单据时才生效」，「上界一，来自原款项」不变仍构成金额封顶；与 B-3、B-5、B-7 同批入卷。

#### C-1 第 4 条甲档：补偿动作的可配面（哪些环节允许哪种补偿、谁能批、留什么证据）

**裁定**：一、承载物：新建 platform_authz.compensation_policies，主键 (legal_entity_id, stage_code, action_code)，列恰为九列加公共列——stage_code、action_code、is_allowed、approval_chain_code（可空）、requires_reauth、requires_reason、requires_attachment、baseline_row_hash、lifecycle_state，多一列即视为越界。
二、发布：不新增 item_kind，复用已有的 AUTHZ_POLICY 与阶段 4 的 AuthzPolicyApplier，把它的落地表清单由四张扩为五张。选 AUTHZ_POLICY 而非新增 item_kind，是因为补偿策略的「谁能批」半边本来就落在 approval_chains 上，拆成两个内容项会使同一次变更跨两项而无法保证同批发布；且计划13:242 的 item_kind 是封闭 15 项由 CHECK 约束，新增必须改 CHECK，与计划13:1076 那句「不改本阶段任何表」自相矛盾。
三、**按反方必改，撤销 SHA-256 冻结机制**。原文照抄能力矩阵的「内置快照为运行期权威，不一致即拒绝一切写入并持续告警」，但那张 client_capability_values 是零可配面的全冻结表，而本条第五点同时声明客户可以收紧——客户第一次合法发布收紧包，表哈希立刻偏离基线，系统随即进入拒绝一切写入状态，收紧永远不生效且此后任何配置包都写不进这张表，原 criteria 二与四在同一台机器上不可能同时为真。改为：编译期常量 COMPENSATION_BASELINE 只作为出厂值与收紧判据的比较基准，不作为运行期权威；表是运行期权威；每次发布对每一行以基线同键行为基准跑 assert_narrowing；baseline_row_hash 列只记该行所依据的基线行哈希，用于识别基线随版本升级后需重新判定的行，不参与任何「拒绝写入」判定。
四、出厂基线 13 行（封闭）：SALES_INVOICE_ISSUED×INVOICE_VOID、SALES_INVOICE_ISSUED×INVOICE_CREDIT_NOTE、PURCHASE_INVOICE_REGISTERED×INVOICE_VOID、PURCHASE_INVOICE_REGISTERED×INVOICE_CREDIT_NOTE 四行 is_allowed=true、requires_reauth=true；DELIVERY_CONFIRMED×SALES_RETURN 与 GOODS_RECEIPT_POSTED×PURCHASE_RETURN 两行 is_allowed=true、requires_reauth=false；RECEIPT_REGISTERED×CASH_DOC_REVERSAL、PAYMENT_REGISTERED×CASH_DOC_REVERSAL、REFUND_REGISTERED×CASH_DOC_REVERSAL 三行 is_allowed=true、requires_reauth=true；OVERBILLING_WRITTEN_OFF×OVERBILLING_REVERSE_WRITE_OFF 一行 is_allowed=true、requires_reauth=true；VOUCHER_UPDATE_OR_DELETE、PERIOD_REOPEN 两行 is_allowed=false（前者因计划09:212 已在库权限层 REVOKE，后者因规格:366 逐字「首版不做反结账」）；VOUCHER_POSTED×CORRECTION_VOUCHER 一行按 B-1 改为 is_allowed=true、requires_reauth=true（原基线取 false 的依据是计划09:935「不提供入口」，该格已被 B-1 作废）。
五、可配面只有减法与加码，没有放宽面：客户只能把 is_allowed 由 true 改 false、把 approval_chain_code 换成节点更多的链、把三个 requires_* 由 false 改 true；基线 false 的行永远配不出 true，requires_reauth 永远配不成 false。
六、防止把账务硬约束配没了，三道：其一无承载物——表上不给事件到分录映射、是否生成凭证、是否写台账、会计期间、唯一约束是否生效、任何 allow_skip 一类列，照抄计划04:163 对审批链的同一手法（「越权跳过不是被校验拒绝的配置，而是根本没有承载它的字段」）；其二单调收紧——纯函数 assert_narrowing(baseline_row, candidate_row)，违反返回 PLATFORM.COMPENSATION_POLICY.WIDENING_FORBIDDEN；其三**按反方应改，不新增第 9 个 suite**——计划13:263 逐字「suite 取值封闭为 8 项」，计划13:444 逐字「SKIPPED 仅允许出现在该包不含对应 item_kind 时」，而本条挂在 AUTHZ_POLICY 上，任何只改审批链的包都含该 item_kind、不许 SKIPPED、只能空判 PASSED，该门禁在最常见的包上恒真。改为 assert_narrowing 在配置保存时执行一次、在运行期提交时再执行一次，形状逐字照抄计划04 第 4.5 节开篇「四类规则，全部在配置保存时执行一次、在运行期提交时再执行一次」（原文写成「保存期与发布期」是误引，因此丢掉了运行期那一道）；发布链路第三道由 AuthzPolicyApplier::apply 的前置校验承担，不动 suite 清单。
七、执行层位：策略求值一律在 ep-app-<m> 用例头部，领域层不读任何配置表（基线第 1.3 节只允许 ep-app-<m> 依赖 ep-platform-*）；领域侧的借贷平衡、核销守恒等硬不变量与其属性测试一字不动。ep-platform-authz 交付 CompensationPolicyQuery 一个端口，ep-app-invoice、sales、procure、finance、ledger 五个在各自补偿用例头部各调一次。
八、明确不设发布后账务回归门禁。

**验收判据**：一、落库（机检）：表列集与本裁定逐字一致，且不存在任何表达事件到分录映射、是否生成凭证、是否写台账、会计期间、唯一约束开关或 allow_skip 语义的列（列名白名单静态比对，白名单即上述九列加公共列）。
二、收紧（机检，正反各一）：一个只收紧的包（把某行 is_allowed 由 true 改 false）发布成功且发布后该动作被拒；一个放宽的包（把基线 false 的行改 true，或把 requires_reauth 改 false）在配置保存时即被拒并断言 PLATFORM.COMPENSATION_POLICY.WIDENING_FORBIDDEN。
三、运行期第二道（机检，否定断言）：绕过保存期校验直接把一行改成放宽形态写入库后，运行期提交对应补偿动作仍被拒并断言同码——用以证明两道判定确实各判一次，而不是只有保存期一道。
四、反向断言（机检，防冻结机制被偷偷加回来）：合法发布一次收紧包之后，该表仍可接受第二次合法发布，且系统不进入拒绝写入状态。

**动到**：规格第 9.1 章低代码能力清单（PRD:2965 逐字「映射本身不可由客户改写」、PRD:4036 逐字「不得改动财务模块的事件到分录映射」两处的边界随之改写，本条给出可配面即动该清单本身）；PRD 第 10.4 节；阶段 4（新增表、assert_narrowing、CompensationPolicyQuery、AuthzPolicyApplier 落地表清单四改五、第 4.5 节两道判定的同款登记）；阶段 6、7、9、10 各自补偿用例头部加一次求值；ep-datagen 的出厂基线 13 行。连带：本条不迁入任何 EP__ 配置项（见 dropped）；不新增 suite，计划13:263 与 :444 一字不动；出厂基线中 VOUCHER_POSTED×CORRECTION_VOUCHER 一行的取值随 B-1 同批确定，两条不得给出相反取值。

#### C-2 第 4 条乙档：可配的是前置条件的阈值参数，不是状态迁移本身

**裁定**：一、不可配面（编译期冻结，客户一个字改不了）的定义域，**按反方应改补第三支**：凡该迁移会触发 ledger.posting_trigger_event_types 十三行中任一事件者；凡该迁移会写 platform_core.append_only_registry 十四张表者；**凡该迁移会写 finance.receivable_entries、payable_entries、advance_receipt_entries、advance_payment_entries、overbilling_entries 五张往来台账者**。第三支不补就有一个真实缺口：裁定 B-02 已把这五张从 append_only_registry 删除（逐字「五张是带核销金额与状态机的可更新台账，登记为仅追加会在上线后拒绝正常核销写入，五行一并删除」），于是凡写这五张而不触发十三行中任一事件的迁移（核销、部分核销、超量开票挂账变更一类）在原定义域下既不落甲也不落乙，按原文就成了可配面——与「写台账的迁移一律编译期冻结」正面相反。
二、可配面：不改变状态迁移集合、不改变分录与台账写入的**前置条件阈值参数**，形态为数值或布尔，经配置发布通道下发，落业务参数表。
三、**按反方必改改判使用方举的那个例子**。「开票 7 天内允许作废」按原判据落在不可配面——发票作废走 invoice.sales_invoice.reversed.v1，该事件在 A-21 冻结的十三行内，原 criteria 一的 configurable-transition-disjoint 会把原 criteria 四那条验收用例直接判死。改判为：ISSUED → VOIDED 这条迁移本身编译期冻结、不可配；可配的只有它的一个前置条件阈值 EP__INVOICE__VOID__MAX_DAYS_AFTER_ISSUE（默认不限，取正整数时表示开具后 N 个自然日内方可作废）。使用方要的效果拿到了，迁移集合一格没动。
四、**撤下 xtask archcheck 的 configurable-transition-disjoint 规则**（见 dropped）：可配参数是经配置发布通道写进事务数据库的运行期行，posting_trigger_event_types 同样是数据库表，而 00b:120-122 逐字把 archcheck 的判定面限在 cargo metadata 与源码树，构建期读不到数据库也读不到尚未存在的客户配置包，这条规则按现有工具形态判不出真假。判定改落两处：配置保存期与运行期提交时各判一次（同 C-1 第六条的形状）。
五、可配参数的清单在首版封闭为一张登记表，新增参数须另裁，与「新增事件必须先登记再实现」同一纪律。

**验收判据**：一、机检（落库）：可配参数登记表的行集与编译期常量清单逐字比对通过，条数断言等于登记条数。
二、机检（正反各一，判定域正确性的关键）：把 EP__INVOICE__VOID__MAX_DAYS_AFTER_ISSUE 配为 7，断言第 8 天登记作废被拒、第 6 天通过；再构造一个试图把 ISSUED → VOIDED 这条迁移本身关掉或新增一条迁移的配置包，断言在配置保存期即被拒（否定断言）。
三、机检（不可配面定义域，三支各一条否定断言）：分别构造触发十三行事件的迁移、写 append_only_registry 十四张表的迁移、写五张往来台账的迁移各一个可配声明，三者在配置保存期均被拒。第三条是本裁定补的那一支的存在证明，不得省。
四、明确不设的判据：不写「同一响应体中不出现任何推荐路径或建议改走红冲的字段」这类句子——「此类键」没有封闭键名清单，机器判不出一个键名算不算推荐路径，且全卷没有任何端点产出过这类字段（PRD:2411 逐字「本系统不判定何时该用哪条」本就是已生效的弃权决定），该断言在任何实现下都为真，是标准的恒真门禁。

**动到**：规格第 9.1 章低代码能力清单（本条给出可配面即动该清单）；PRD 第 10.4 节；阶段 4 或阶段 3（可配参数登记表与两道判定的落点，按参数的属主模块分派）；阶段 10（EP__INVOICE__VOID__MAX_DAYS_AFTER_ISSUE 的前置校验分支与其用例）。连带：可配参数的载体形态受盘点第七节第四档已登记的矛盾影响（11 个 EP__ 业务参数「启动时读取，变更需重启」对 00b:577「运行期可变的业务参数不进配置文件」），本条要求的是运行期可变，因此该矛盾必须先解，登记为未决；本条不新增 item_kind、不改计划13:242 的封闭 15 项。

#### D-3 对外表述权限与诚实披露（原 D-13-2／D-13-3 修正合并后的裁定）

**裁定**：一、表述权限分三档，写入规格第 21 章新增的 21.22 节：第一档（可无条件使用）——对本卷已认证事实的陈述，例如单机形态、20 并发、备份保留 14 天；第二档（须有实测举证方可使用）——任何比较级表述，须附第三方或双方共同见证的实测报告并注明口径与样本；第三档（一律禁止）——「碾压」「行业模板」「实施顾问」「生态伙伴」四词及同类词作为承诺性表述，以及任何未经实测的比较级。三档的裁决权归产品负责人，法务前置结论为第二档的必要条件。
二、**第二档的实测举证口径本轮不定**（随 dropped 的 D-13-1 一并另裁）。在该口径定下来之前，第二档表述一律不得使用——即本轮的净效果是：不得对外宣称任何比较级优势。这是保守但可执行的，它现在就能防住过度承诺。
三、**禁用词的检查并入既有清单，不另造 CI 检查**（见 dropped）。计划14:568 逐字已有「交付、认证与验收材料经文本检查未出现高可用、零停机、自动切换、受控读取、法人隔离、等效、已满足、优先级隔离、资源隔离、性能保证十项禁用措辞」，把四个新词并入该清单，检查对象仍是交付、认证与验收材料。规格第 21.22 节本身作为规范性条文显式排除在检查对象之外——否则两条裁定落地即互相判违反（21.22 节必然出现「碾压」二字）。
四、诚实披露条目由八条改为十一条，新增的三条即本轮裁定直接产生的三类客户可感知限制：① 合同终止后处置未闭合前，合同停在「终止处置中」，期间不能开票、发货、改单，闭合时长取决于人工决策项的处理速度（A-3、A-12）；② 首版不做反结账、不提供手工凭证入口，已过账凭证的更正只能红冲或更正凭证追加，且更正凭证必须引用一张原凭证（B-1、B-2）；③ 备份保留期 14 天，超出保留期的历史版本不可恢复，恢复点粒度受落点形态影响（D-1）。措辞由产品负责人定稿，覆盖面以这三类为准。

**验收判据**：一、机检：计划14:568 的禁用措辞清单由十项改十四项，且该文本检查的对象集合不含规格第 21.22 节所在文件的该节（排除规则以节锚点表达，可判）。构造一份含「碾压」的交付材料样本，断言检查失败（否定断言，防清单加了但不生效）。
二、评审：规格第 21.22 节存在且三档划分与本裁定逐字一致；第二档在实测口径未定前不可用这一条写在节内。
三、机检：PRD 第 11.11 节小节数为十一，且计划14:568 退出条件第 16 项的计数同步改为十一；三处（PRD 小节数、计划 14 计数、客户合同模板条目数）逐字比对一致。
四、评审：新增三条的文本在产品界面可达处呈现（计划14:568 逐字要求「并在产品界面可达处呈现」），举证方式为贴出阶段 13 该页面的截图或文案键值表。

**动到**：规格第 21 章新增 21.22 节，第 21.4 章签字对象扩面；**按反方必改补全 PRD 与阶段 13 的连带**——PRD 第 11.11 节新增三小节（原裁定写「PRD：不动」是错的，计划14:568 逐字系于 PRD 第 11.11 节且该节 11.11.1 至 11.11.8 八小节俱在，PRD:4325、4329、4338、4352、4356、4360、4369、4373）；计划14:568 退出条件第 16 项的计数（该项在计划 14 的实际行号是 45，原文两处写成 44）；客户合同模板；阶段 13（界面可达处呈现的三条新文本）。连带：本条的第二档在 D-13-1 另裁之前恒为不可用，届时须回写本条；新增三条的内容直接取自 A-3/A-12、B-1/B-2、D-1 三处裁定，那三处若在入卷时被改，本条须同批改。

### 计划级新增（10 条）

#### A-8 卷宗硬冲突：「尚未开票的收款计划期次置作废」在现有 DDL 上落不了库

**裁定**：一、clm.contract_payment_schedules 新增两列：status text not null default 'ACTIVE' CHECK in ACTIVE, VOIDED；void_reason text；新增表级约束 ck_contract_payment_schedules_void_reason 表达 status <> 'VOIDED' or void_reason is not null。计划06:891 逐字要求「尚未开票的收款计划期次置作废」，而计划06:185 的列清单里既无 status 也无任何已开票金额列，这句话按现有 DDL 落不了库。
二、ep_contract_clm::ContractPaymentScheduleQuery::schedules 的返回 DTO 同批新增 status 一项。
三、阶段 10 的到款自动核销取数只取 status = 'ACTIVE' 的期次。
四、clm.v_contract_reminder_sources 的取数须排除 status = 'VOIDED' 的期次，以及合同状态为 TERMINATING 或 TERMINATED 的全部期次与交付节点。不加这条，已终止合同会永远按收付款计划到期日与交付节点日期继续发提醒（规格:288 逐字要求「按合同有效期、交付节点日期和收付款计划到期日生成提醒」），正是 U-E-12 自述的「带着派生单据长期挂账」在用户面前的样子。
五、「尚未开票」这个事实不在本表上，判定一律走 A-7 第 4 条的 ReceiptPlanBillingQuery，不得在 clm 侧另起一套开票金额镜像——那会造出第二套开票事实，与裁定 C-20「本表是收付款计划行的唯一出处」的纪律相悖，也会踩进计划10:1300 那条未定的回写风险。
六、本条只加 ACTIVE 与 VOIDED 两个取值，刻意不加 SETTLED（已结清是财务侧判定，加进来会诱使在 clm 侧维护第二套结清状态）。同段须写明该 VOIDED 是期次自身状态、不是合同作废（A-4 第一条的管辖面）。
对反方一条应改的修正：档位由「实现级补充」上调为「计划级新增」。本条做的四件事没有一件停在实现层——改建表 DDL 与其 CHECK 集、改一个被裁定 C-20 定为唯一跨模块出处的 trait 的返回 DTO（跨阶段契约变更）、改阶段 10 的到款自动核销取数口径、改一个直接承载规格:288 提醒要求的视图取数语义。

**验收判据**：全部可机检。一、迁移后该表有两列并有该 CHECK。二、否定断言：status = 'VOIDED' 且 void_reason is null 的写入被数据库拒绝。三、否定断言：一份已终止合同的 VOIDED 期次不出现在到期提醒取数结果里，也不出现在到款自动核销的候选集里；同一合同的交付节点同样不出现在提醒取数里。四、回归断言（防第三条被实现成一刀切）：未终止合同的 ACTIVE 期次在两处取数中照常出现。

**动到**：阶段 6 的计划06:185 建表迁移与计划06:353 的提醒视图；阶段 6 的 ContractPaymentScheduleQuery DTO；阶段 10 的计划10:248 取数口径。连带：一、DTO 形状变更是跨阶段契约变更，该 trait 按 C-20 是收付款计划行的唯一出处、阶段 10 是唯一消费方，改 DTO 必须与阶段 10 同批，不得阶段 6 单方面改完就走；二、docs/data-dictionary.md 该表列条目数加二，阶段 6 的表列计数须同批复核。

#### A-9 新增两个事件与其消费者；并更正「阶段 13 把事件类型冻结为十个」这一读法

**裁定**：一、事实更正（先做这条，否则后面的裁定会被一个不存在的约束卡住）。不存在「阶段 13 把事件类型冻结为十个」这回事。计划13:21 逐字是「docs/event-catalog.md 新增 10 个事件类型」，与同句并列的还有「新增 37 条错误码」「新增 19 张表条目」，是本阶段增量清单，与阶段 2 的 3 个、阶段 3 的 17 个、阶段 6 的 18 个、阶段 7 的 14 个、阶段 10 的 12 个并列，不是全卷上限。真正冻结为十的是另一件事：规格:298 的十类业务事件到分录的固定映射。合同终止不产生凭证，不进那张映射表，因此不撞。加事件在本卷的成本按 00b:522 逐字只是「新增事件必须先登记再实现」，不是禁止。
二、新增两个事件名，按 00b:522 的四段式与已完成时态：clm.contract.terminated.v1（IN_PERFORMANCE 或 EFFECTIVE 迁到 TERMINATING 时发出，payload 含 contract_id、contract_version_no、doc_no、terminate_reason、approval_ref、terminated_at，posting_date 与 accounting_period_id 取空）；clm.contract.termination_completed.v1（TERMINATING 迁到 TERMINATED 时发出，payload 含 contract_id、impact_assessment_id、item_total、completed_at，命名与既有 clm.contract.derivation_completed.v1 逐字同形）。
三、**按反方应改，把「占用九个预留名额」由结论降为待数**。计划06:612 逐字「本阶段的事件总数固定为 18……其余九个是合同与销售订单状态机的迁移事件，名称按基线第 6.1 节的四段式在实现前先登记入 docs/event-catalog.md」，但计划06:337-357 的合同状态机有 13 条边、销售订单状态机另有七条上下，九个名额本就是从二十条上下的迁移中选定的子集，不是闲置余量；A-5 又同批新增四条迁移边。本裁定因此不断言「事件总数 18 一字不改」，改为：两个事件名与其 payload 本轮裁定成立并须先登记再实现；阶段 6 事件总数是 18 还是 20，由入卷时回去把九个未命名名额的实际归属逐条数清后确定，二者择一，本裁定不代数。这正是本卷已犯三次的那类错，不在这里犯第四次。
四、两者不进 ledger.posting_trigger_event_types（该表按 A-21 共 13 行只登记会产生凭证的事件；合同类事件按计划06:608 逐字「不产生凭证，posting_date 与 accounting_period_id 为空」）。
五、消费者只增一个：job-worker 的 platform.impact_assess，消费 clm.contract.terminated.v1，动作是建批次与全部处置项。阶段 7 与阶段 12 不各建终止消费者——它们的动作已收在各自的 ImpactRule::dispose 里、由 ImpactAssessor 驱动。两套都建会出现处置项还是 PENDING 而需求单已被消费者关掉的不一致，闭合判定随之失真。

**验收判据**：一、机检（现有 CI 直接覆盖）：docs/event-catalog.md 的 clm 段含两个新事件且与代码常量表逐字一致，由 xtask configdoc 承担；阶段 3b 的 event-catalog-consistent 自检项同步覆盖。
二、机检：ledger.posting_trigger_event_types 的行数仍为 13（否定断言）。
三、算术核对（可评审且可当场验算，入卷时必做）：阶段 6 第 6.3 小节的命名表行数、文中「其余 N 个」与本阶段事件总数三者自洽；第 1 节与第 9 节仍不出现任何事件数字。必须逐字回去数，不得凭记忆复述。
四、机检（防双写）：job-worker wiring 中消费 clm.contract.terminated.v1 的消费者恰为一个。

**动到**：阶段 6 第 6.3 小节（命名表与其计数，见判据三）；docs/event-catalog.md 的 clm 段；阶段 3 新增 platform.impact_assess 消费者名与其 platform_msg.inbox_consumptions 幂等键。规格与 PRD 均不动。连带：本条的事实更正须回写，00c 入卷时明记「阶段 13 的 10 个事件类型是阶段增量不是全局上限」，防止同一误读在下一轮再次成为决策依据；若日后把影响面机制接到第二个上游事件（如发票红冲），那时才会真正面对阶段 10 的 12 个事件这个已定数。

#### A-10 验收：三条端到端用例加一条专门用来证伪的反向用例

**裁定**：新增四条用例，编号接现有序列。
E2E-6-08（阶段 6）：一份 IN_PERFORMANCE 合同发起终止 → TERMINATION 审批链通过 → 合同迁到 TERMINATING → 批次建立、三类已注册规则产出实项、另四类产出占位项、订单行与其分批交付行与交付节点自动闭合、交付确认项停在 MANUAL_DECISION 并在 GET /api/v1/platform/process-tasks 待办列表中对受理人可见 → 断言合同未到达 TERMINATED。
E2E-6-09（阶段 6）：从 EFFECTIVE 且 derivation_state = FAILED 的合同发起终止成功；从 EFFECTIVE 且 derivation_state = RUNNING 与从 COMPLETED 发起各被拒并返回 CLM.CONTRACT.INVALID_STATE_TRANSITION。
E2E-12-xx（阶段 12）：七类项俱全的合同终止全链闭合，合同到达 TERMINATED，clm.contract.termination_completed.v1 发出恰一次；重复调用终止端点 3 次只产生一个批次。
E2E-12-xx+1（反向用例，本条不得省，按反方修正后的形状）：故意把 CLM_TERM_PROJECT_TASK 的一项留在 PENDING，断言 ① 合同永不到达 TERMINATED、② 批次保持 RUNNING、③ 该项人工任务节点的 SLA 定时器到点后 sla_breached_at 非空并产出一条「流程时限提醒」；再把该项在 decision_reason 为空时置 DONE，断言写入被表级 CHECK 拒绝；补上 decision_reason 后合同才到达 TERMINATED。另加一段：把该项改为八次失败置 DEAD，断言批次为 FAILED、合同仍为 TERMINATING，调用 replay 后可继续推进。
现状是阶段 6 全文的测试与 E2E 段落里检索「终止」零命中，而同批交付的合同合并、续签、信用超额全都有用例（计划06:726、727）。终止在卷宗里有端点、有状态边、有一条临时处置规则，却没有一条用例证明它能跑通。

**验收判据**：四条用例在 CI 中全绿。另加一条评审判据专门针对第四条：该用例中必须至少存在三条形如 assert!(...is_err()) 或等效的否定断言，且这三条断言在被测代码的闭合判定被人为改成恒真时会失败。举证方式为在评审时贴出该用例源码与一次故意破坏闭合判定后的失败输出——这条评审判据本身可判定，它要求的是一次可复现的失败而不是一句承诺。
第四条用例存在的唯一目的是证伪：没有它，item_done = item_total 这个闭合判据必然恒真，因为测试里从来没有造出过一个未闭合的项，等式天然成立，门禁看起来全绿而实际什么都没判。

**动到**：阶段 6 第 8.3 小节测试清单与第 8.4 小节 E2E 表；阶段 12 对应小节。连带：阶段 6 与阶段 12 的用例编号与条数同批改；E2E 表新增行后阶段 6 第 8.4 小节开头「本阶段涉及的两行能力域为合同条款与电子签章、销售订单与履约」的表述不变（终止落在同一两行内），无需扩面。

#### A-11 跨阶段接线次序与未接线期间的表现

**裁定**：一、接线分四步，按已冻结的阶段链（00-overview:155「1 → 2 → 3a → 4 → 3b-1 → T0 → 5 → 9a → 8 → 6 → 7 → 10 → 11 → 9b → 14」，另按 00-overview:55「阶段 12 在阶段 10 之后与阶段 11 并行」）：阶段 3 建两表与 crate 与消费者 → 阶段 6 建动作、状态机、审批链取值、三条规则 → 阶段 7 一条 → 阶段 10 两条 → 阶段 12 一条。
二、**按反方两条必改重写未接线期间的表现**。原文两句互斥（「尚未注册的规则其处置项不产出」与「已产出但未接线的项恒 PENDING」），且前一句会开一个真实窟窿：阶段 6 到 12 之间 item_total 只算已注册的三到六条规则，合同可以在采购需求、项目任务、销项发票、收款计划期次一项未处置的情况下满足闭合判据直接进 TERMINATED。改为逐字照抄正向计划06:429 的既有写法：项集按 A-1 的编译期目录建立，未注册规则一律建一条占位项，state 恒 PENDING、target_doc_id 与 target_doc_no 留空、不计入 item_done，因此含该项的合同停在 TERMINATING、批次保持 RUNNING、界面按未接线呈现；对应阶段接线后由该规则的 assess 重算并推进。不得构造占位单号，不得把未接线项直接置 DONE，不得注入 ImpactRule 替身。这与计划12:455 的硬规则（跨模块同步调用的被调方必须与调用方同批到位，不存在先注入空实现再回头替换）完全相容——目录是常量声明不是实现。
三、**注册数判据改为逐阶段期望值表，不再写「恰为七」**。原文三处写死「ImpactRegistry 注册项数恰为七」，与本条的四阶段渐进接线直接对撞，在阶段 6 至 11 之间必然红，而实现方唯一出路是提前注册替身、又被本条明令禁止。改为：目录条数恒为七（编译期常量，全阶段可判）；注册数按阶段退出时的期望值断言，阶段 6 为 3、阶段 7 为 4、阶段 10 为 6、阶段 12 为 7。
四、终止动作的完整验收落在阶段 12，阶段 6 只验收自己的三条规则加状态机加审批链加台账建立。
五、终止端点在阶段 6 结束时即对外开放不隐藏：它已能正确把合同推进到 TERMINATING 并阻断下游新增业务（A-12），这一半价值在阶段 6 就成立；隐藏端点会让新增的四条状态边在三个阶段里成为死代码，等到阶段 12 再一次性点亮，风险反而更大。

**验收判据**：一、阶段 6 退出条件新增一项（机检加界面评审）：一次终止后批次建立、item_total 等于目录七条对应的项数、三类产出实项、另四类为占位项且 state = PENDING、合同停在 TERMINATING、界面按未接线呈现；同时断言 wiring 目录下不存在任何 ImpactRule 的替身实现，且 ImpactRegistry 注册数恰为 3。
二、阶段 7 与阶段 10 退出条件各新增一项：本阶段规则注册后注册数分别为 4 与 6，同一场景下对应类别的占位项被展开为实项并可闭合。
三、阶段 12 退出条件新增一项（机检）：注册数为 7，七类项全部产出、全部闭合、合同到达 TERMINATED、clm.contract.termination_completed.v1 发出恰一次。
四、否定断言（防偷懒，且因占位项确实存在而非恒真）：未注册规则对应的 impact_rule_code 在 impact_disposition_items 中存在行（占位项）且其中不存在任何 state = 'DONE' 的行。

**动到**：阶段 6、7、10、12 各自的退出条件与第 11.5 小节的接线登记；阶段 3 的交付物清单与目录常量表。连带：四份计划的退出条件条数各加一至二，四处计数须同批改并回去数，这是本卷计数失配的第四个高危点。

#### B-5 硬伤一：应收应付台账改追加式反向条目，三条守恒 CHECK 一字不改

**裁定**：计划10:349 逐字「is_reversed……红冲或作废登记后置 true 并把 original_amount 冲回」按现有三条 CHECK（ck_*_original_positive、ck_*_settled_le_original、ck_*_open_identity）三种读法全部撞死，而这三条 CHECK 是规格第 17.3 章核销守恒的落库形式不能为红冲让路。唯一出路是不改 CHECK、改写法，卷内已有正确先例：过渡科目子账用的就是追加行（计划10:796 逐字「追加一条 DEBIT 方向……的条目」）。
(1) 删除两张表的 is_reversed 列与计划10:349 那句说明。(2) 两表各新增三列：entry_kind text 非空 CHECK in INVOICE, REVERSAL；reverses_entry_id uuid 可空、同 schema 外键 ON DELETE RESTRICT、CHECK 表达 REVERSAL 时必须非空且 INVOICE 时必须为空；invoice_reversal_id uuid 可空。**按反方必改补一处封闭 CHECK 的取值**：计划10:341 逐字「ck_receivable_entries_source_type 取值 SALES_INVOICE、MIGRATION_OPENING」（应付侧同构），REVERSAL 行落不了库，据此 source_type 同批新增取值 INVOICE_REVERSAL，两张表各一处。(3) 红冲或作废登记追加一条 REVERSAL 条目：original_amount 取本次 red_gross_amount（恒大于 0 满足既有 CHECK）、settled_amount 为 0、open_amount 等于 original_amount、due_date 取原行、business_date 取冲销登记日期、accounting_period_id 与 deferred_from_period_id 取红字凭证的解析结果。(4) 新增视图 finance.v_receivable_open 与 v_payable_open，对每条 INVOICE 行输出 effective_open = open_amount - coalesce(该行全部 REVERSAL 子行的 open_amount 之和, 0)。(5) 核销候选集替换计划10:719 的「open_amount > 0 且 is_reversed = false」：改取 effective_open > 0，核销分配第 3 步的逐条上限由 open_amount 改为 effective_open。(6) 账龄基数（第 4.5 节与 PRD 6.9.3）由 open_amount 改为 effective_open，REVERSAL 行不单独进账龄。(7) 计划10:993 register_invoice_reversal 事务清单中的「冲回」改写为「追加 REVERSAL 条目并按 B-6 释放核销」。
effective_open 非负不是新加约束而是累计上限的推论，写成属性测试即可，不必加第四条 CHECK；追加式还顺带保住 PRD:2626 逐字「台账中不允许出现负数未核销余额」，每一行都为正，负号只出现在聚合口径里。

**验收判据**：落库判据：迁移后两表不存在 is_reversed 列，且三条既有守恒 CHECK 的名称与表达式逐字未变，由迁移测试断言；source_type 取值集合逐字等于三项。用例一：一张 110 的票、无到款、部分红冲 30 → 表中两行（INVOICE 110 与 REVERSAL 30），effective_open 为 80，账龄基数为 80，核销候选上限为 80。用例二（关键回归，先 RED 后 GREEN）：对上一步再到款 100 → 只核销 80、余 20 转预收；断言核销分配第 3 步取的是 effective_open 而不是 open_amount，取错会核销 110 并当场撞守恒。属性测试：对任意「开票—分次到款—多次部分红冲」序列恒有 effective_open >= 0，且 sum(effective_open) = sum(INVOICE.original) - sum(REVERSAL.original) - 净核销额。

**动到**：阶段 10（第 3.2.3 与 3.2.4 两张表的列与 CHECK 与 source_type 取值、第 3.3 节视图清单加两个并同批改该节合计数、第 3.4 节 RLS 策略与 3.5 节索引清单、第 3.6 节迁移清单、第 4.3 节核销分配第 1 与第 3 步、第 4.5 节账龄、第 6.1 节事务清单、第 8.2 与 8.3 节）；PRD 第 6.9.1 与 6.9.3（现行 6.9 全节没有一句写红冲，须补），并把 PRD:2431 的「见 6.9」改为实指。连带：计划10:719 与第 4.5 节两处口径必须同批改，漏一处即出现「按 open_amount 核销、按 effective_open 记账龄」的不一致，且这种不一致不会报错只会算错；finance.v_receivable_ledger_entries 与 v_payable_ledger_entries 两个受治理数据集视图（裁定 A-18）的列签名随之变化，须与阶段 11 的 reporting.dataset_fields 同批更新否则 reporting-dataset-signature-matched 自检会开降级窗口并关闭相关报表入口；阶段 11 的应收应付账龄两张基础表取数随之改；与 B-6、B-7 同批。

#### B-7 硬伤三：跨期红冲的子账腿与总账腿，勾稽视图改按期间累计

**裁定**：(1) 追加的 REVERSAL 条目其 accounting_period_id 与 deferred_from_period_id 一律取红字凭证在同一事务内的期间解析结果（计划10:802 的记忆化解析已保证凭证与全部子账条目共用同一次解析），因此同一事件的子账腿与总账腿落在同一期间，规格:368 逐字成立；原地改老行的写法一并废止。
(2) 四个勾稽视图的子账侧由当前值列改为按期间累计取数，**按反方必改逐视图分别给式，不写「同构改写」**——finance.advance_receipt_entries 与 advance_payment_entries 没有 entry_kind 列也不产生 REVERSAL 行，照抄含 entry_kind 的表达式落不了库：
· v_recon_receivable 与 v_recon_payable 的子账侧(P) = 对 receivable_entries（payable_entries）中 accounting_period_id <= P 的行求 sum(case when entry_kind = 'INVOICE' then original_amount else -original_amount end)，减去对应核销关系表中 accounting_period_id <= P 的行求 sum(settled_amount)，其中 reverses_id 非空的反向行按计划10:494 逐字「在聚合时按 reverses_id 抵消」参与，即两行相消。
· v_recon_advance_receipt 与 v_recon_advance_payment 的子账侧(P) = 对预收（预付）条目中 accounting_period_id <= P 的行求 sum(original_amount)，减去对应核销关系表中 accounting_period_id <= P 的行求 sum(settled_amount)（反向行同样按 reverses_id 抵消）。不引入 entry_kind。
这同时消解反方指出的口径打架：B-6 对 AUTO_ADVANCE 分支「把原预收条目的 open_amount 回增」只影响当前值列（核销候选与账龄两个当前时点口径），不进勾稽取数，因此不构成「回溯改写早期期间切片」；反向核销行本身已是追加式并按期间参与累计。
(3) 四张核销关系表各加 ix_*_legal_entity_id_accounting_period_id 承接按期间累计取数，落阶段 10 第 3.5 节索引（不是第 3.4 节，该节是 RLS 策略）。
(4) open_amount 与 settled_amount 两列保留，但只服务于核销候选集与账龄两个当前时点口径，不再是勾稽取数来源；这句写进阶段 10 第 3.3 节开头作为口径分工的明文。
规格:368 的最后一句已把目的写清：逐字「第 17.3 章按法人与会计期间逐项核对的子账与总账勾稽因此不因顺延产生跨期差额」。现行 sum(open_amount) 取当前值，任何后发生的核销或冲销都会回溯改写一个早期期间的切片，与总账侧「期末余额逐期累计」根本不同源；红冲只是把这个结构性错配暴露得最明显的一种。

**验收判据**：一、跨期用例（需两个打开期间，改造前必然失败，先 RED 后 GREEN）：M1 开票 110、M2 部分红冲 30。断言 M1 切片子账侧为 110 且与 M1 期末应收账款科目余额差额为零；M2 切片子账侧为 80 且与 M2 期末余额差额为零。改造前 M1 会算成 80。二、已关闭期间稳定性：M1 关账后在 M2 红冲，重跑 M1 的十项勾稽，全部差额仍为零且与关账当时的结果逐字一致。三、顺延用例：红字凭证因期间已关闭顺延到 M3 时，REVERSAL 条目的 accounting_period_id 等于 M3、deferred_from_period_id 非空，M3 两侧差额为零。四、预收专项（针对本条修正的分别给式）：一笔由预收自动核销而来的核销被跨期红冲后，v_recon_advance_receipt 在红冲当期与其后各期的差额均为零。五、属性测试：对任意事件序列与任意期间 P，四个视图的子账侧(P) 恒等于 TotalAccountBalanceProvider 给出的对应科目 P 期末余额。

**动到**：阶段 10（第 3.3 节四个视图取数式与一段口径分工说明、第 3.5 节索引清单加四条、第 3.6 节迁移清单、第 8.2 与 8.3 节）；阶段 9（第 9.4.7 节四类校验项中「按法人与会计期间比对子账侧合计与总账侧科目余额」的取数口径须与本条对齐）；规格第 10.2 章不改，PRD 第 6.13.2 不改，端点形态与每日及关账前口径均不动。连带：与 B-5、B-6 同批，单独落任何一条都不成立；裁定 F-07 已把「八个勾稽视图」改为十，本条只改其中四个的取数式不改数量；规格:1443 要求收入成本利润三项指标取数与三处一致，阶段 11 的经营指标一致性判据随应收应付台账口径改变须同批复核；盘点第 213 行「每日对账是否重跑已关闭期间」这个悬空问题由本条的累计口径消解，不再单独立规矩，也不动计划09:456 的执行器遍历写法。

#### B-8 销项发票的订单行与数量明细模型（部分红冲要真解决 PRD:373 的前提）

**裁定**：与部分红冲同批新增 invoice.sales_invoice_lines（sales_invoice_id 同 schema 外键、line_no、sales_order_id 与 sales_order_line_id 逻辑引用、quantity numeric(18,6) > 0、net_amount、tax_amount、gross_amount、reversed_quantity 默认 0、reversed_net_amount 默认 0，加公共列；CHECK reversed_quantity <= quantity、reversed_net_amount <= net_amount），并在 invoice.invoice_reversals 之下新增 invoice.invoice_reversal_lines（invoice_reversal_id、sales_invoice_line_id、red_quantity、red_net_amount、red_tax_amount、red_gross_amount）。头表金额恒等于行表合计，由领域守卫加属性测试保证。
**按反方必改重写 is_fully_credit_noted 的判定式**。原式「sum(quantity) - sum(reversed_quantity)，若剩余已开票数量小于入参 quantity 则返回已全额红冲」代入它自己的用例即阻断（5 台开票、红冲 1 台、退 1 台：4 < 1 不成立 → 返回未冲销清单 → 阻断），声称要打通的 PRD:373 在它自己的公式下仍走不通。改为按可退数量判定，签名一字不改：
可退数量 R_avail = (该订单行 delivered_quantity − returned_quantity) − (该订单行已开票数量 − 已红冲数量)，其中已开票数量与已红冲数量对该订单行的全部发票行求和。入参 quantity <= R_avail 时返回通过；否则返回未冲销的发票清单。
代入验证：5 台已交付已开票、红冲 1 台、已退 0 → R_avail = 5 − 0 − (5 − 1) = 1 ≥ 1 → 通过；再退第二台 → R_avail = 5 − 1 − 4 = 0 < 1 → 阻断。完全未开票时 R_avail = delivered − returned，与现行不需红冲的语义一致。该式只用 delivered_quantity 与 returned_quantity 两个既有列加行表两个累计列，不引入新概念。
部分红冲单独交付只能让它在账上成立，不能让规格:312 逐字「退货部分已开票的，必须先由红字冲销与作废事件按销项方向冲减销项税额与应收账款并恢复应收账款未开票过渡科目余额，再登记退货」与 PRD:373 那条路走通——承接这道判定的端口签名已被裁定 C-16 钉死为 (sales_order_line_id, quantity)，而这两个事实在现有数据模型上根本不存在。两条必须同批。计划10:57 自评行明细代价为「中」，本裁定不压低这个估计。

**验收判据**：一、端到端用例（M7 判据的直接落点，改造前无法实现）：一张覆盖五台设备的销项发票，对其中一台登记部分红冲，随后登记该台的销售退货 → 通过；对未冲销的另一台登记退货 → 阻断，断言 SALES.SALES_RETURN.INVOICE_NOT_CREDIT_NOTED 且响应列出待冲销的发票。二、属性测试：发票头表 net_amount、tax_amount、gross_amount 恒等于行表对应合计，reversed_* 同理。三、分摊判据：invoice_receipt_plan_links 与 finance.unbilled_ar_entries 的金额分摊在多行发票下仍与头表合计一致，差额为零。四、边界断言：完全未开票的订单行退货照常通过（防公式被实现成一律要求红冲）。如实登记一处不做：本条不要求发票行与交付确认行逐行匹配，只做到订单行与数量这一层；交付确认层仍按计划10 第 4.9 节逐字「不做与交付确认的逐笔匹配」的科目层面净额口径，本裁定不动它。

**动到**：阶段 10（第 0.4 节 F-02 与 F-06 两格、第 3.1.3 节头表、新增两张行表并按第 3.4 节建 rls_*_le 策略与进 C-05 矩阵、第 3.5 节索引、第 3.6 节迁移清单与「36 张表」计数、第 4.9 节分摊、第 5.2 节端点请求体加行数组、第 8.2 与 8.3 节）；阶段 6 第 4.12 节销售退货前置校验；PRD 第 6.4 与 6.5 节字段表加行明细；规格不改。**按反方应改更正一处对现状的描述**：阶段 6 侧不存在可改的空实现——计划06:476 逐字「该判定按裁定 C-16 不进 T0 切片，与阶段 10 的该 trait 按第 11.5 小节同批交付同批验收，本阶段不注入替身，承载该判定的退货登记分支整体落在第三批并在该批次当场成立」；真正要动的是第三批的同批交付口径与阶段 10 侧的实现体。同时登记一处卷内既存冲突：00c 裁定 C-16 的回写段写「阶段 6 先注入空实现、阶段 10 替换」，与计划06:476 本就打架，入卷时须一并更正为计划06:476 的口径。连带：计划10:57 的 F-02 那格作废、同格代价栏「中」保留不压低；计划10:59 的 F-04「剩余可开比例的计算基数为合同金额」本条不改，但行金额合计与比例基数两套口径并存须在第 4.4 节明写。

#### C-3 第 9 条 U-A-08：默认审批链与无链即拒

**裁定**：一、承载物：审批链一律落 platform_authz.approval_chains 与 approval_chain_nodes，clm.contract_types 的四个 approval_chain_*_code 由「逻辑引用 platform_flow 的流程定义键」改为「逻辑引用 platform_authz.approval_chains.code」，解析口径为该法人下 is_active 为真的最大 version_no（计划04:161 的唯一键含 version_no，只给裸 code 解析不到唯一行，此点必须写明）。
二、**「无链即拒」的作用面按反方必改收窄**：只作用于本裁定给出默认链的那些 scenario，不扩到规格:1041 的六类高风险操作全集。原文把范围写成六类而只给了其中一部分默认链，落地后出厂默认数据集上合同生效与开票没有链、敏感导出全线不可用，T0 贯通线（计划04:8 逐字点名四条单节点审批链，scenario 依次为合同生效、发票申请单、销项发票开具与资金账户建档）在建单与开票两步各断一次。
三、**到款登记从清单剔除**：计划10:68 逐字「到款登记不需重新认证也不需审批」，且该项是否需审批由配置项 EP__FINANCE__RECEIPT__REQUIRES_APPROVAL（默认 false）承载，属 F-13 与 U-D-14 的可配面，本条不动它，也不把它迁进 C-1 的表（见 dropped）。
四、**角色码按反方必改改为大写形态**：00b:219 逐字「RoleCode(Arc<str>)，取值为长度 1 至 64 的 [A-Z0-9_]，与 platform_authz.roles.code 逐字一致」，01:157 同款且由 archcheck 的 foundation-frozen-items 守。原文给的 finance.accountant 一类小写带点形态一律 RoleCode::parse 失败、写不进种子配置包。改为 FINANCE_ACCOUNTANT、FINANCE_MANAGER、SALES_MANAGER、PROCURE_MANAGER、OPS_DATA_OWNER 五个。同时登记一处卷内既存冲突：计划04:282 写的「字符集为小写字母、数字、下划线与点」与 00b:219 及 01:157 相反，按权威顺序以技术基线为准，计划04:282 须同批更正。
五、**不设 fail-open**：原文要求无链时 fail-open 并挂 DegradationLedger 开窗，但 DegradationKind 的唯一定义方按 00-overview:75 定死为阶段 2、终态清单唯一出处定死为阶段 14 的 18 项，现有取值没有一项对得上「审批链缺失」，误用 PORT_NOT_IMPLEMENTED 不成立；新增一个 kind 要同批改阶段 2 建表 CHECK 与阶段 14 的 18 项终态清单，与本条不成比例。且 ux_degradation_windows_kind_scope_closed 使同一 scenario 第二次 fail-open 插不进第二行，原判据只在第一次成立。统一为 fail-closed：无链或链节点展开为空一律拒绝提交。
六、默认链行须给全 code 与 name 两列，且其集合必须覆盖三处：计划04:8 点名的四条 T0 链、计划06:193 的 chain_kind 六值加 A-6a 新增的 TERMINATION 共七值、本裁定点名的财务类 scenario（发票作废与红冲、付款登记、退款登记、期间关账与年结、更正凭证、资金单据冲正）。具体 code 取值由阶段 4 与阶段 6 同批补齐，本裁定不代拟，但覆盖面由下列判据强制。

**验收判据**：一、机检（覆盖面，本条的核心判据，可判且非恒真）：ep-datagen 生成默认 scale 数据集后，对上述三处的每一个 scenario 逐个断言能解析到唯一 is_active 链且节点展开后用户集合非空；任一 scenario 解析不到或展开为空即失败。
二、机检（无链即拒，否定断言）：把某个已给默认链的 scenario 的链停用后，该动作提交被拒；断言拒绝路径上不产生任何 degradation_windows 行（证明确实是 fail-closed 而不是偷偷 fail-open）。
三、机检（角色码形态）：五个角色码经 RoleCode::parse 全部成功，且与 platform_authz.roles.code 的种子行逐字一致；构造一个小写带点的角色码负样例，断言 parse 失败。
四、机检（不扩面的反向断言）：规格:1041 六类中未被本裁定给出默认链的 scenario，其提交路径上不存在「无链即拒」分支——用以证明作用面确实被收窄，没有把 T0 贯通线掐断。

**动到**：阶段 4（12 行默认链的 code、name、节点、quorum、timeout_hours、is_active；计划04:282 的字符集表述更正；ep-datagen 生成默认数据集，计划04:29 与退出条件 04:743 同批改）；阶段 6（计划06:140 的四个 approval_chain_*_code 改指与其解析口径、chain_kind 七值与链集合的对应）；阶段 9、10（财务类 scenario 的链取值，与 B-1、B-3 对齐）；PRD 附录乙 U-A-08 改已裁定。连带：本条给出的取值是 B-1 更正凭证、B-3 资金单据冲正、A-6a 终止三处审批链取值的唯一出处，三处不得各给一套；不新增 DegradationKind，阶段 2 与阶段 14 一字不动。

#### D-1 备份保留期 D 与其治理（原 D-12-1／D-12-3／D-12-5 三条合并后的裁定）

**裁定**：一、保留期 D 认证取值 14 天，落库为 platform_ops.backup_retention_policies（id、security_level、data_scope_tags、retention_days、min_valid_generations、approval_ref、approver_id、second_approver_id、reauth_ref、effective_from、superseded_at、公共列）与 deployment_records.retention_days。
二、**区间守卫按反方必改重写**：原文一边写「上调超过 14 天允许」，一边给 CHECK (retention_days between 7 and 14)，数据库层直接拒收大于 14 的取值，上调路径不可达；且判据前后半句互斥。改为 CHECK (retention_days >= 7)，上限不进 CHECK，改由「不得超过该部署已完成演练所证明的取值」承载：retention_days 大于 14 时该部署必须有非空 drill_report_ref。
三、治理：保留期为单一取值，不按对象类型、模块、单据类型分列；变更须双人审批加重新认证，留 approval_ref、approver_id、second_approver_id、reauth_ref 四项证据；effective_from 与 superseded_at 表达版本，历史行不覆盖。
四、**例行回收的触发方按反方应改**：仍由 ops-agent 以 ops 专用账号按日发起，不由 job-worker 触发。计划14:331 逐字「触发面。只由 ops 专用路径与 ops 专用账号触发，不在 /api/v1/platform 前缀下对外暴露」，job-worker 自动构造 DisposalRequest 与该句正面冲突。落点在 DisposalRequest 的 BackupSets 范围内不变。落点长时间不可写时（规格:1209 归档通道暂停）例行回收持续被拒，该噪声按每日一条汇总告警处理，不逐次写审计。
五、**容量不足改为可记录、可告警，不设硬 CHECK**（见 dropped 的 ck_offsite_sinks_capacity）：offsite_sinks 增列 capacity_floor_bytes，容量不足时不拒收该行，改由 v_retention_status 输出 shortfall 并每日告警，同时按第 15.3 章开一条降级窗口，kind 取本轮新增的 BACKUP_RETENTION_WINDOW_SHORT、basis 取 CAPACITY_SHORT（见 D-2 的 basis 集合）。
六、W_day（事务日志生成速率）不另设第二套口径：规格:1841 逐字「按稳定段实测时长折算的事务日志生成速率……该速率是 A.3 连续归档本机保留子项取值依据的唯一来源」，本条直接引用该实测项，不在 A.4 新增同名必判必记项。
七、capacity_floor_bytes 的比对基准改为该部署实施方案登记的客户实际数据量，不绑 A.4 认证报告实测值。规格:1824 逐字「部署前由实施方按客户实际数据量完成容量核算，实际数据量超出本节取值时按同一构成重算容量下限并写入实施方案」，用 A.4 认证报告值逐字节比对每个部署等于要求所有部署容量下限相同，凡客户数据量不等于 A.3 基准的部署必判不通过。
八、代价照实说：D 由 7 天升到 14 天，客户落点要多约 2.6 TB，不是 2.1 TB——2.1 TB 只数了 7 代全量（7 × 300 GB），丢掉了多出来的 7 天事务日志归档、配置包与 1.15 余量；按公式代入 D=14 约 6877 GB、D=7 约 4285 GB，差 2592 GB。这个价签原样交给使用方。

**验收判据**：一、落库（机检）：backup_retention_policies 表存在且 CHECK 为 retention_days >= 7；retention_days > 14 且 drill_report_ref 为空的行写入被拒（否定断言）；retention_days = 7 与 = 14 均可写入（回归断言，防区间被实现成只允许一个值）。
二、单一取值（机检，按反方必改重写）：断言该表不存在任何按对象类型、模块或单据类型分列的列。列名黑名单静态比对，白名单口径明确排除 security_level 与 data_scope_tags 两个公共列——原判据写「无任何按对象类型、密级、法人或业务规则分列的列，多一列即判违反」，而该表自己的 DDL 里就有这两个按计划14:90/96/119 不可省的公共列，是一条对自己恒判违反的闸门。
三、治理（机检，否定断言）：缺少第二审批人或缺少 reauth_ref 的变更被拒；变更后原行 superseded_at 非空且行本身未被覆盖。
四、例行回收（机检）：由 ops 专用账号发起的日回收成功销毁早于窗口的备份集并留下销毁证明；由非 ops 账号或经 /api/v1/platform 前缀发起的同一请求被拒（否定断言）。
五、容量（机检）：capacity_floor_bytes 按公式与该部署实施方案登记的数据量重算一致（容差 0）；容量不足的落点行可正常写入且 v_retention_status 输出非零 shortfall 并开出一条降级窗口（本条是撤销硬 CHECK 之后「容量不足可被记录」这条性质的存在证明）。

**动到**：规格第 13.3 章新增一句（保留期取值与其治理）、第 13.4 章新增一条、第 22 章第 13 条追加半句；阶段 14（新增一张表与一个视图、offsite_sinks 增列、DisposalRequest 的日回收发起方、退出条件第 17 项补两句）。**按反方必改补全阶段 14 的四处计数与登记**：计划14:554 逐字「第 3 节的 17 张表……5 个视图与 21 个迁移文件全部落库」三个计数各加（18 张表、6 个视图、22 个迁移）；计划14:574 逐字「本阶段新建的 16 张 platform_ops 表在 platform_core.unpoliced_table_registry 中各有一行登记」计数加一并补该行的五列取值；计划14:178 逐字「platform_ops.v_ops_health……聚合上述四个视图的关键取值」改为五个；00b:603 逐字「追加项名与其 severity 在各阶段计划中登记，全量清单以总览第 4.3 节 C-25 行为唯一出处」，新增自检项须登记入总览 C-25 行。

#### D-2 备份保留窗口的自检项与降级窗口（原 D-12-4 修正后的存留部分）

**裁定**：一、新增自检项 backup-retention-window。**severity 按反方必改不得取会使 --check 非零退出的档**：计划14:452 逐字「任一项为 FAILED 或 DEGRADED 均以非零码退出，用于部署验收与升级前置校验」，计划14:553 逐字要求三个进程「报告中无 FAILED 也无 DEGRADED；并在生产配置下连续运行不少于 7 个自然日」。D 取 14 时任何新部署头 14 天必然 DEGRADED，交付日的部署验收 --check 必然非零码退出，阶段 14 退出条件第 1 项（只跑 7 天）永远不可满足。据此：该自检项的 severity 取 Informational（若卷内无此档，则该项不进 --check，改由 v_retention_status 视图加每日巡检告警承载），部署验收退出码不受其影响。
二、新增 DegradationKind 取值 BACKUP_RETENTION_WINDOW_SHORT，十八改十九。basis 集合取五项并**按反方应改给出优先级**（原四项定义互相包含，「四态互斥」按定义无法满足）：BOOTSTRAP（部署运行未满 D 天）> ARCHIVE_GAP（归档段序列有空洞）> ANCHOR_MISSING（不存在 verified_at ≤ now − D 的有效全量）> GENERATIONS_SHORT（有效全量代数低于 3）> CAPACITY_SHORT（落点容量低于下限，见 D-1 第五条）。同一时点只取优先级最高的一项。
三、**十八改十九的同批清单按反方更正**：补 00c:1899 逐字「逐个核过十八个取值：落点未配置、写出进程未投入运行、……」与计划14:572 逐字「platform_ops.degradation_windows 的 kind 取值已由阶段 2 的 3 个扩展至 18 个」两处；把原清单所称 00c:3084 改为 00c:3115、所称 00c:3165 改为 00c:3196；删去所称「13-clients-lowcode.md:1065」一处，该文件该行是 create index concurrently 的风险表行，原引句在该文件不成立。一份用来防计数失配的清单本身失配，必须先修好再用。
四、**A.6 演练那一支撤下**（见 dropped）：追加「恢复到最早全量之后 15 分钟」的一次演练撞规格:1864 逐字的 RPO 不超过 15 分钟，而原文枚举「一字不放宽」的项时恰好漏掉 RPO；规格:1867 逐字「两次均达标才判定通过」、规格:1853 逐字「演练结果是 A.5 唯一的可用性类发布判据」，该改动会把第 22 章第 7 条变成必不通过。

**验收判据**：一、部署验收（机检，本条修正的存在证明）：一个刚部署完成第 1 天的环境执行 ops-agent --check，退出码为 0；断言 backup-retention-window 项在报告中出现且其结论为 BOOTSTRAP，但不计入 FAILED 与 DEGRADED 两类。
二、basis 优先级（机检）：分别构造五种成因同时成立与两两同时成立的输入，断言产出的 basis 恰为优先级最高的那一项，不产生两条并存的窗口。
三、窗口生命周期（机检）：满 D 天且锚点齐备后该窗口自动关闭，v_retention_status 转 PASSED。
四、十八改十九（机检）：degradation_windows 的 kind CHECK 取值集合恰为 19 项且含 BACKUP_RETENTION_WINDOW_SHORT；上述四处文档计数与代码常量表逐字比对通过。

**动到**：阶段 2（degradation_windows 的 kind CHECK 建表迁移，A-26 冻结项，须与阶段 14 同批）；阶段 14（十八项终态清单改十九、自检项注册与其 severity 登记、v_retention_status 视图、basis 优先级实现与用例）；总览第 4.3 节 C-25 行（自检项全量清单唯一出处）；00c:1899 与其余三处计数文本。连带：本条动阶段 2 与阶段 14 两个已由 A-26 冻结的落点，档位按此计，不因「只加一个取值」而降档。

### 实现级补充（4 条）

#### A-4 词义：「作废」与「终止」不是一件事

**裁定**：一、在合同这一类对象上，VOID（已作废）只指生效前废弃，入边只有 DRAFT → VOID 与 REJECTED → VOID，守卫「无派生记录」，逐字保留不动。
二、已生效合同的退出动作一律叫「终止」，端点固定 POST /api/v1/clm/contracts/{id}/actions/terminate，状态固定 TERMINATING 与 TERMINATED。
三、界面文案、错误码文案、交付说明与销售话术中，禁止把已生效合同的退出称作「作废」；CLM.CONTRACT.* 错误码段不得新增任何含 VOID 字样且作用于已生效及其后状态的码。
四、使用方口头说的「合同作废」在本卷内一律按「终止」受理，本条即该翻译的唯一登记处。
对反方一条应改的修正：范围收窄。原文写成「在本卷内 VOID 只指生效前废弃」，会与卷内至少三处已生效对象上的作废撞车——计划10:193 销项发票 status 取值 ISSUED/VOIDED/RED_REVERSED、计划10:691「ISSUED 到 VOIDED……各自只允许一次」、其上位规格:311「同一张发票的作废与红字冲销互斥」，销项发票作废恰恰只对已开具的发票成立；A-8 新增的收款计划期次 VOIDED 同理。据此本条的管辖面明确限定为合同对象，发票、订单、收款计划期次上的 VOIDED 不在本条管辖，且 A-8 须在同段写明期次的 VOIDED 不是合同作废。

**验收判据**：一、机检（可判且非恒真）：阶段 6 的 clm.contracts 状态机中，目标状态为 VOID 的边恰为两条且源状态恰为 DRAFT 与 REJECTED；从 EFFECTIVE、IN_PERFORMANCE、TERMINATING、COMPLETED 任一状态发起 VOID 均返回 CLM.CONTRACT.INVALID_STATE_TRANSITION（四条否定断言）。
二、评审（如实降级并登记入 00b 第 12.1 节 delegated 段）：作用于 EFFECTIVE 及其后状态的合同动作，界面与错误码文案上没有一处用「作废」字样；举证方式为贴出阶段 6 错误码段与阶段 13 该页面的文案键值表逐处核对结论。原判据把「逐处核对作用对象」写成 grep 机检，实际 grep 判不出「作用对象处于哪个状态」这一维，按本卷通则第六条改为评审并登记，不留伪机检。

**动到**：PRD 第 3.6 节表后加一段词义说明；阶段 6 的文案与错误码；阶段 13 的客户端文案键值表；交付说明模板；00b 第 12.1 节 delegated 段加一行。连带：计划06 第 11.3 小节 U-E-12 行原文「尚未开票的收款计划期次置作废」中的「作废」指期次自身状态，按 A-8 改写为「置 VOIDED」并在同段写明这不是合同作废。

#### A-6a 终止的审批要求：新增 TERMINATION 审批链取值（第七类高风险那一半已撤下，见 dropped）

**裁定**：一、clm.contract_approvals.chain_kind 的 CHECK 取值由六个增为七个，新增 TERMINATION。这是计划06:891 那句「须经审批」在现有 DDL 上落地所必需的最小改动——计划06:193 的 chain_kind 是封闭 CHECK，六个取值（TERMS、DISCOUNT、PAYMENT、ATTACHMENT、CREDIT、EFFECTIVE）里没有终止链，不加这一个取值，那句话在数据库层就是落不了地的，这是卷宗内部的硬冲突不是留白。
二、该链必须含管理层节点，与合同生效链同形（规格:873 逐字「管理层是合同审批的必经节点」）；申请人不可自审由既有的规格:1062 与规格:1064 承载，不新增条款。
三、本轮不为终止追加重新认证要求，不动规格第 12.1 章的六类清单，不动 PRD 第 10.3 节，不动能力矩阵一格。理由与代价见 dropped 与 still_open。终止端点因此不带 X-Reauth-Token，其控制强度为「TERMINATION 审批链加管理层必经节点加申请人不可自审」。
四、本条明确不引用 client_capability_values 的行数作为任何判据——该数值按使用方已表态的 F-09-3（00c:2659 逐字「由 18 个能力域乘 4 端共 72 格变为乘 5 端共 90 格；client_capability_values 新增一列；逐格核对与验收矩阵随之扩面」）必然要变，任何断言它不变的判据都会与在先裁定对撞。终止落在「合同条款与电子签章」能力域，该域按规格:592 移动端取值为「简化」，终止随域取值，不新增能力域、不新增豁免条目、不重编冻结快照。

**验收判据**：全部可机检。一、DDL 断言：chain_kind 的 CHECK 取值恰为七个且含 TERMINATION。二、用例三条：无 TERMINATION 链或链节点展开为空时调终止端点被拒（否定断言）；审批人等于发起人时返回 PLATFORM.AUTHZ.SELF_APPROVAL_FORBIDDEN（否定断言）；审批链缺少管理层节点时保存该链被拒（否定断言）。三、反向断言（防偷偷升档）：终止端点的实现中不出现任何 X-Reauth-Token 校验分支，且 specs/ 下「六类」相关口径逐字未被改动——用以证明本条确实停在降级路径上，没有把 dropped 掉的那一半悄悄做进来。

**动到**：阶段 6 的 DDL（计划06:193 的 chain_kind CHECK 与其迁移）；阶段 6 端点表（计划06:524 追加 PLATFORM.AUTHZ.SELF_APPROVAL_FORBIDDEN 一码，该码为平台段由阶段 1 登记、按裁定 C-24 不计入阶段 6 条数，此点须在阶段 6 计划写明否则复核时会误以为漏登记）；阶段 4 的审批链场景清单（与 C-3 同批，见该条）。规格与 PRD 一字不动，这是本条降级后的全部代价面。

#### A-12 前置阻断的谓词扩面，以及不得阻断反向补偿动作这条必须写死的例外

**裁定**：一、凡按「合同已终止」判定的下游前置阻断，谓词一律改为「合同状态属于 TERMINATING 或 TERMINATED」。已知落点两处逐字：PRD:2289「合同状态校验：合同未生效、已终止的不允许提交发票申请。」与 PRD:2324「| 合同未生效或已终止 | 阻止提交，定位到合同字段 |」。不改就会开一个真实窗口：终止已发起、处置正在进行但合同还没到 TERMINATED，所有按「已终止」判定的阻断在这段时间里全部失效，正好在最混乱的那段时间还能继续开票发货，处置清单会边处置边变长。
二、阻断面在本裁定内穷举为五项，多一项不加：① 新开销项发票申请提交；② 新登记交付确认；③ 订单变更（sales.sales_order_changes）；④ 按该合同新建采购需求（四类来源中的合同派生与销售订单两支）；⑤ 合同修订 actions/amend。
三、必须写死的例外：反向补偿动作在 TERMINATING 与 TERMINATED 两个状态下一律不得被阻断，四类照常可提交——销售退货登记、发票作废与红字冲销登记、客户退款登记、采购退货登记。这四类恰恰是 MANUAL_DECISION 处置项唯一的完成手段；若有人出于「已终止合同不该再有任何写入」这种听起来很正确的直觉加一条通用守卫，闭合判定会瞬间变成永远不可满足，机制从「推着走完」退化成「卡死」，且卡死的表现是合同永远停在 TERMINATING，比不做还糟。
四、附带核对并补断言：销售退货的前置校验按计划06:474 只取「退货数量不超过该订单行 delivered_quantity - returned_quantity」，不读订单行状态，因此 A-7 把订单行置 CLOSED 或 CANCELLED 之后退货仍可登记。阶段 6 须补一条断言把这个「现在恰好没问题」的状态钉成被保护的性质，防止后续实现顺手加一条状态守卫把这条路堵死。

**验收判据**：十条断言，全部可机检（原文正文写「九条」而列出十条，本裁定统一为十条，并同批更正 A-3 丙对本条的引用）。
正向五条（否定断言）：五项阻断在 TERMINATING 状态下各被拒一次，错误码与已终止状态下一致。
反向四条（肯定断言，本条关键）：销售退货登记、发票红冲登记、客户退款登记、采购退货登记四类在 TERMINATING 状态下各成功一次。
回归一条：一条订单行被 A-7 置为 CLOSED 之后，针对该行的销售退货仍可登记成功。
反向四条与回归一条不得省——它们是唯一能证明本机制不会把自己卡死的断言。

**动到**：PRD:2289 与 PRD:2324 两处逐字，另在 PRD 新增 3.5.5 节内写入第二、三条的两张清单；阶段 6 与阶段 10 的守卫实现与用例。连带：一、第三条的例外清单必须落在 PRD 而不是只落在阶段计划——阶段计划是第三层权威，日后任何一轮复核只要读到 PRD 里「已终止合同不允许提交发票申请」而读不到例外，就会把这四条例外当成漏洞再堵一次，这是本簇唯一一处特意要求把结论写高一层的地方；二、第二条的五项是穷举不是举例，日后加第六项须另裁并同时核对它是否会挡住某个 MANUAL_DECISION 项的完成手段，这条核对纪律一并写入 PRD 3.5.5。

#### C-4 第 11 条：能力闸与重新认证的次序，以及豁免清单的边界

**裁定**：一、次序：能力闸在前，重新认证挑战在后。
二、豁免清单只含 POST /api/v1/platform/reauth-challenges 这一段（重新认证挑战的签发），其端别判定由计划04:512 的服务端判定按四类 operation_type 承担。
三、**按反方必改，/api/v1/platform/high-risk-requests* 整段路由不豁免，必须过能力闸**。整段豁免会把移动端审批放出来：规格:577 逐字「到款登记、付款登记、发票申请、发票开具登记与账龄对账在移动端只提供查询与查看，提交、审批与写入操作转桌面端完成」——审批被明写在转桌面端之列；而计划04:512 的服务端判定只作用于 reauth-challenges 的签发，管不到该路由上的 approve、withdraw、confirm_execution 等写动作（计划04:370 状态机的 IN_APPROVAL → APPROVED 等迁移），一旦不过能力闸就没有任何服务端端别判定拦它们，移动端可直接审批一张付款高风险单。同时规格第 6.2 章「同一功能同时落入两个能力域时以取值较低的所在行为准」在被整段豁免的路由上无处施加。
四、**四处自相矛盾统一为一处口径**：由于 reauth-challenges 被豁免、能力闸在该端点上永不触发，阶段 4 该段用例的期望错误码**保持不变**；原文既写「在阶段 13 交付时同批改为能力闸的错误码」又在 touches 写「保持不变」、criteria 三断言不变而 criteria 四断言已改齐，四处两两冲突，落码方无从下手。统一为不变，原「阶段 4 的该条用例在阶段 13 之后必然转红」这条理由连同其裁定一并作废。
五、**改写句的作用域写死为阶段 10 第 5 节全部 POST 与 PATCH 行**，不写「本节」也不写具体条数。原文照抄计划10:905 句中的「本节」二字，而该行落在第 5.5 节（台账、账龄与对账），该节 PATCH 为空集、原判据在空集上恒真；且规格:577 点名的到款登记、付款登记、发票申请、发票开具登记四类端点全在 5.1 至 5.4 节，不在改写句覆盖面内，本条要闭的正是这个缺口。条数不写死是为了不制造本卷第四次计数失配，判据按逐行驱动。

**验收判据**：一、机检（逐端点覆盖）：对阶段 10 第 5 节端点表中的每一个 POST 与 PATCH 行，逐个断言携带 X-Client: ios 时返回 PLATFORM.HIGH_RISK_REQUEST.CLIENT_NOT_ALLOWED 或其对应的能力闸错误码；用例由端点表驱动生成，新增端点自动纳入，不写死条数。
二、机检（豁免边界，否定断言）：POST /api/v1/platform/reauth-challenges 携带 X-Client: ios 时不被能力闸拦截（走计划04:512 的服务端 operation_type 判定）；/api/v1/platform/high-risk-requests 的 approve 动作携带 X-Client: ios 时被能力闸拦截。后者是本条修正的存在证明。
三、机检（次序）：一个既不满足能力闸又缺 X-Reauth-Token 的请求，返回的是能力闸错误码而不是 PLATFORM.REAUTH.TOKEN_REQUIRED。
四、回归断言：阶段 4 该段用例的期望错误码与改动前逐字一致，全绿。

**动到**：阶段 13（能力闸中间件的豁免清单与其判定次序）；阶段 10（第 5 节端点表的能力闸标注与用例生成）；阶段 4（该段用例期望值不变，仅在计划中写明本条已核过不需改）。连带：裁定 A-20 与计划04:433、计划13:512 三处须同批加同一句例外说明（原文引的计划13:511 是空行，实为 512 行）。

---

## 三、撤下的 22 条（各因反方哪一条必改）

- A-6b 终止列为第七类高风险操作（含重新认证要求）——撤下，进 still_open。两条必改：其一代价被压低到不可接受，「六类」口径在 specs/ 下逐字命中 36 处（规格 8 处、PRD 25 处、盘点 2 处、AI 分析设计稿 1 处），原 touches 只列 8 处，且其中三处不是枚举复述而是判定文本：规格:1330「合同生效、付款、开票、财务过账、期末结账和敏感数据导出六类高风险操作按第 12.1 章验证重新认证」是身份与访问控制测试的判定文本、规格:1433 是第 19 章退出条件、规格:1835 是性能认证的负载构成（逐字「审计事件由本节业务负载及其中第 12.1 章六类高风险操作的实际发生频次自然产生」，六类改七类要重标定发生频次并可能重跑 A.2 基线）；另有 PRD:2889 的 F-13 与 PRD:4510 的 U-D-14 两条待决项以「六类不含这三项」为立论前提。其二原判据「client_capability_values 行数仍为 72、冻结快照哈希与改动前一致」直接撞使用方已表态的 F-09-3（00c:2659 逐字「由 18 个能力域乘 4 端共 72 格变为乘 5 端共 90 格；client_capability_values 新增一列」「扩端要重编二进制并重跑逐格核对」）。原裁定第五条自己预留的降级路径已被采纳为 A-6a。

- A-1 的 WAIVED 状态与其 approval_ref——撤下，不进本轮。全簇未定义豁免一项处置走哪条审批链、审批人是谁、approval_ref 从哪个流程实例来，A-6a 只加了终止动作本身的链不是逐项豁免的链，落地后只能靠测试代码手工塞一个 UUID。且它本就多余：A-7 三类 MANUAL_DECISION 项的完成手段本身含「明确不处置并说明理由」，以 state = DONE 加非空 decision_reason 表达即可，不需要第二条出口。

- A-2 与 A-7 覆盖表中的规格第 8 章第 13 步（记账与结账，其已发生事实为已过账凭证）——从覆盖面显式摘出。原判据要求七条规则覆盖含第 13 步在内的三类已发生事实，而 A-7 同时明确不设第八条凭证处置项，该「缺一不通过」的评审判据按定义恒假。摘出理由与 A-7 同：更正手段只有红冲与更正凭证，红冲是第 3、7 两条的自动后果，单设凭证项会产生永远闭合不了的项。

- A-2 原两条机检判据（押在 xtask archcheck 的 db-pg-one-schema-per-file 上）——撤下并换判据。00b:118 逐字把该规则的判定面限在 crates/adapter/db-pg/src 之内，而 ImpactRule 实现落在 ep-app-<m>，该规则判不到被测对象：原 criteria 一恒真（无对象可判）、criteria 二要求的负样例做不出来（它不会报错）。替身见 A-2 修正后的判据。

- B-1 更正凭证的期初余额批次来源（source_ref_kind = OPENING_BALANCE_LINE）——撤下。原七条判据没有一条测这条来源；语义也未定义：计划09:518 逐字「四个通道的写入一律不生成凭证，期初对应的总账侧由本节的期初余额批次承担」，期初落在 ledger.account_period_balances 的 opening_balance_amount 上而不是凭证行，计划09:480 逐字「期初固化……固化一次即不再被推翻」，而更正凭证产生的是当期发生额改不动期初。按本卷通则第六条，写不出判据的要么撤下要么换替身，此处撤下。

- B-1 原守卫二（每一行必须且只能引用一条已过账凭证行）与守卫三（按被引用行逐行算累计上限）——撤下，换为凭证级引用与凭证借方合计上限。两条互相打死：最标准的两行重分类（借科目乙 X、贷科目甲 X）两行都得引用原凭证那条借科目甲 X，累计 2X > X 当场被拒，连它自己的正向用例都过不去；若允许随便指一条尚有余量的行，守卫三又不构成任何实质约束。

- B-1 原守卫五（同事务内重跑全局十项勾稽，任一项差额非零整笔拒绝）——撤下，换为「只重跑本次分录触及的勾稽项且判据为不劣化」。规格:1003 逐字「差异清零前不得关账」，差额已存在的那一刻正是唯一需要更正凭证的场景，原守卫在该场景下对任何更正凭证一律返回 RECONCILIATION_BROKEN，等于开了一个只在不需要它的时候可用的入口。

- C-1 的 SHA-256 冻结机制（内置快照为运行期权威、不一致即拒绝一切写入并持续告警）——撤下。它照抄的 client_capability_values 是零可配面的全冻结表，而 compensation_policies 声明可配：客户第一次合法发布收紧包，表哈希立刻偏离基线，系统随即拒绝一切写入，收紧永远不生效且此后任何配置包都写不进这张表；哈希比对也分不出「篡改」与「合法发布」，原 criteria 二与四不可能同时为真。

- C-1 把 EP__FINANCE__RECEIPT__REQUIRES_APPROVAL 与 EP__FINANCE__CASH_ACCOUNT__REQUIRES_APPROVAL 迁入 compensation_policies——撤下。迁入目标不存在（九列里没有任何表达「本动作是否需要审批」的布尔列，13 行基线里也没有 RECEIPT_REGISTER 与 CASH_ACCOUNT_MAINTAIN 两行，且资金账户档案维护不是补偿动作）；且会无声推翻计划10:68 的 F-13——该配置项默认 true，迁进一张只有减法面的表后客户再也关不掉它，F-13 给出的可配面被删而未在任何一处说明。

- C-1 新增第 9 个自动测试 suite COMPENSATION_POLICY——撤下。计划13:263 逐字「suite 取值封闭为 8 项」，计划13:444 逐字「SKIPPED 仅允许出现在该包不含对应 item_kind 时」；本条挂在 AUTHZ_POLICY 上，任何只改审批链或访问策略的包都含该 item_kind、不许 SKIPPED、只能空判 PASSED，该门禁在最常见的包上恒真。判定改落配置保存期与运行期两道。

- C-2 新增 xtask archcheck 规则 configurable-transition-disjoint——撤下。可配声明与 posting_trigger_event_types 都是运行期数据库内容，而 00b:120-122 逐字把 archcheck 的判定面限在 cargo metadata 与源码树（逐字「凡在 cargo metadata 之外另需调用图分析的断言，本基线不认其为已可判定」），构建期读不到数据库也读不到尚未存在的客户配置包，这条规则按现有工具形态判不出真假，其负样例也无从构造。

- C-2 把「开票 7 天内允许作废」作为可配的状态迁移——改判为阈值参数可配、迁移本身冻结。按 C-2 自己的判据，发票作废走 invoice.sales_invoice.reversed.v1，该事件在 A-21 冻结的十三行内，落在不可配面；原 criteria 一会把原 criteria 四那条验收用例直接判死。使用方要的效果由 EP__INVOICE__VOID__MAX_DAYS_AFTER_ISSUE 一个阈值参数拿到，迁移集合一格不动。

- C-3 的 fail-open 挂 DegradationLedger——撤下，统一为 fail-closed。现有 DegradationKind 没有一项对得上「审批链缺失」，PORT_NOT_IMPLEMENTED 按 00-overview:75 是「跨模块与平台能力缺位的唯一登记形态」属误用；新增 kind 要同批改阶段 2 建表 CHECK 与阶段 14 的 18 项终态清单，与本条不成比例；且 ux_degradation_windows_kind_scope_closed 使同一 scenario 第二次 fail-open 插不进第二行，原判据只在第一次成立。

- C-4 把 /api/v1/platform/high-risk-requests* 整段路由列入能力闸豁免清单——撤下。规格:577 逐字把审批列在移动端「转桌面端完成」之列，而计划04:512 的服务端判定只作用于 reauth-challenges 的签发，管不到该路由上的 approve、withdraw、confirm_execution 等写动作，整段豁免等于在移动端审批面上开一个口子，移动端可直接审批一张付款高风险单。

- D-12-1 的归档段销毁一支（保留自锚点全量 base_lsn 起的全部归档段、早于该 LSN 方可销毁，并把归档段序列连续无空洞写成守卫）——撤下，进 still_open。计划14:335 逐字「DisposalRequest.scope 取 AttachmentObjects、KeyDomain、BackupSets、ExtTables 四者之一」，归档段既不是 backup_sets 行也不属这四类，没有任何处置范围承载其销毁；platform_ops 内也没有归档段清单表（计划14:109 的 writeout_runs 只记 channel、period_seq 与字节数，无段身份与 LSN）。判据的被测输入没有提供方与交付阶段，直接违反 00c:1689 的通则第六条第一句。

- D-12-1 的第三段（丁）「保留策略对 ATTACHMENT_FULL 只允许清理 backup_sets 行本身」——撤下。与其第五段守卫（5）「kind 不为 ATTACHMENT_FULL」自相打架；且即使按（丁）执行，计划14:335 逐字「到达备份保留期的备份集销毁走 BackupSets，两者与附件正文一样必须把落点上的历史副本在同一次处置内一并覆盖，未一并覆盖的销毁证明不成立」，只清行不覆盖落点产不出成立的销毁证明，该路径在现行计划下不存在。

- D-12-2 的 ck_offsite_sinks_capacity 硬 CHECK——撤下，改为可记录加告警加降级窗口。该 CHECK 使「落点容量不足」这个真实状态在库里无法存在，其第六段整段处置连同「反解可支撑的最大 D」成为不可达代码；且它只对 media_type = 'NONE' 开例外，规格:1218 允许客户只提供离线介质，此类部署单盘容量低于下限时同样被拒收，误伤一条已生效的降级路径。

- D-12-4 给附录 A.6 整机失效恢复行追加「恢复到最早全量之后 15 分钟」这一次演练——撤下，进 still_open。规格:1864 该行判定标准逐字含「RPO 不超过 15 分钟」，而原文枚举「一字不放宽」的项时恰好漏掉 RPO；恢复到十几天前的目标点其恢复点距现在以天计，该次演练按现行判定标准必然 RPO 不达标；叠加规格:1867「两次均达标才判定通过」与规格:1853「演练结果是 A.5 唯一的可用性类发布判据」，该改动直接把第 22 章第 7 条变成必不通过。

- D-12-4 的演练门禁断言「两次演练的 backup_set_id 不同且其中一次等于该落点上 verified_at 最早的 DAILY_FULL 的 id」——撤下。「最早的 DAILY_FULL」随时间与回收任务变动，D-1 的回收会把早于锚点的全量销毁，同一份演练报告在两个时点会得出不同结论，判据不可复算不可复现，而发布门禁必须能在证据包采集时点稳定判真假。

- D-13-1 六项任务的效率验收（T = 0、A ≤ N_min + 8、T1–T5 四端各测一次、第 22 章新增第 16 条与门禁项 RG-TASK-ECONOMY）——整条撤下重裁，进 still_open。三条必改全是结构性的：其一「四端各测一次」与它自己引作依据的 PRD:4272 冲突，该行逐字把「发票申请与开具登记」「收付款登记与对账查看」列为移动端只查看写入转桌面端，T2 至 T5 四项任务在移动端不存在写入路径，等于给第 22 章新增一条永远判不过的必输条款；其二 A（键盘按键次数加点击次数）与 N_min（最小必要输入字段数）量纲不同，任一含文本录入的字段单独就要十来次击键，通过线在实践中恒假，而配套豁免条款在首版没有前一个版本因而恒真，必输与无牙两种形态都被通则第六条禁止；其三 T = 0 的判据实际测的是开发者自己打的来源标注，且枚举只有两个取值时「出现第三类即失败」永不成立，是恒真。

- D-13-3 新增一条独立 CI 文本检查（断言规格与 PRD 全文不出现四个词）——撤下，并入计划14:568 既有的十项禁用措辞清单。原文与同簇 D-13-2 必须同批落地却互相判违反（规格第 21.22 节必然出现「碾压」二字）；「作为承诺性表述」这一限定也不是文本检查能判的，只能退化为子串匹配（则恒假）或人工评审（则不是机检）。既有清单的覆盖面更大且已有承接方。

- 第 5 条冗余（是否采购第二台机器）——按主控前提，本轮一字不裁，归使用方。

---

## 四、仍须使用方拍板（12 条）

1. 终止是否列为第七类高风险操作（即是否要求重新认证）。本轮只裁了「须经 TERMINATION 审批链」这一半。升为第七类的完整代价：specs/ 下「六类」口径逐字命中 36 处须同批改（规格 8 处、PRD 25 处、盘点 2 处、AI 分析设计稿 1 处），其中规格:1330 是身份与访问控制测试的判定文本、规格:1433 是第 19 章退出条件、规格:1835 是性能认证的负载构成（六类改七类要重标定审计事件发生频次，可能使 A.2 的时延通过线需重测）、PRD:2889 的 F-13 与 PRD:4510 的 U-D-14 两条待决项以「六类不含这三项」为立论前提须连带复核。另须注意与使用方已表态的 F-09-3 的关系：能力矩阵按 00c:2659 已定要由 72 格变 90 格并重编二进制，终止的四端口径应在那次扩面里一并给，不要单独再动一次。若否掉，终止在移动端的可发起性改由规格第 6.2 章「合同条款与电子签章」行的『简化』取值单独承载，而『简化』没有逐动作定义，需再判一次。

2. MANUAL_DECISION 处置项的默认受理人解析（assignee_user_id 与 candidate_role_codes 取什么）。C-3 只给了财务类与 T0 类 scenario 的默认链，影响面处置这个触发源在计划03:928 的接收人解析表里没有对应行。受理人解析写错的后果是待办发给不该发的人或发不出去。须与 C-3 的默认链清单同批补齐。

3. EP__IMPACT__MANUAL_ITEM__SLA_DAYS 的取值与其载体形态，以及 clm.contract_termination_disposition 的 max_instance_duration_days 取值。默认 5 天是按 process_tasks 既有 SLA 机制推的，卷宗无出处。更麻烦的是载体本身有一处未决：盘点第七节第四档已登记「11 个 EP__ 业务参数现在是启动时读取、变更需重启」对 00b:577「运行期可变的业务参数不进配置文件」自相矛盾。C-2 的可配阈值参数（含 EP__INVOICE__VOID__MAX_DAYS_AFTER_ISSUE）同样卡在这条上——C-2 要求的是运行期可变，因此那条矛盾必须先解，本轮不代拍。

4. 影响面机制是否要同时挂到「合同变更」与「合同续签」两个上游事件上（U-J-13，PRD:4624）。本轮只挂合同终止一个，规则集冻结为七条。复用则 U-J-13 的临时取值「保留既有任务，不自动作废」要整条重议；不复用则要接受同一件事在系统里有两套处理形态。接第二个上游事件时才会真正面对事件预算问题（阶段 10 的 12 个事件是已定数）。这一刀比它看起来的贵，需要单独一轮。

5. 阶段 6 的事件总数是 18 还是 20。计划06:612 逐字「本阶段的事件总数固定为 18……其余九个是合同与销售订单状态机的迁移事件」，但计划06:337-357 的合同状态机有 13 条边、销售订单状态机另有七条上下，九个名额本就是从二十条上下的迁移中选定的子集不是闲置余量，而 A-5 又新增四条迁移边。入卷时须把九个未命名名额的实际归属逐条数清后确定，二者择一。这是本卷计数失配的高危点，不代数。

6. 进项方向是否存在「作废」这个动作。invoice.invoice_reversals 的 direction 取 OUTPUT、INPUT 而 reversal_type 取 VOID、RED_LETTER，即 INPUT + VOID 在数据模型上可表达；但 invoice.purchase_invoices 的 status 逐字只有 REGISTERED、REVERSED 两态，规格:311 进项方向那段也只写红字冲销与金额税额更正未提作废。B-4 给进项加了 PARTIALLY_REVERSED，但「进项能不能作废」先于本轮就已两边不一致，且属税务口径不是工程口径，卷宗无据可依。须财务负责人一句话，或由阶段 10 与规格同批把 INPUT + VOID 明确禁掉。

7. B-1 更正凭证守卫五（同事务内重跑本次触及的勾稽项）的执行时延代价。ep-platform-recon 现有本体是分批加快照的批处理形态（计划09 第 9.4.7 节：逐法人遍历、快照经 snapshot_transact 导出、有单批时限与单查询内存上限），压成一次同步提交内的调用是否落在可接受时延内，卷宗给不出判断依据——规格附录 A.4 的负载模型是稳态混合负载，盘点第 168 行已逐字登记它「没有任何一条描述红冲、反向凭证链、批量更正带来的写入洪峰」。判据能写死（不劣化），代价数不出来。若实测超时，退路是把触及项子集再收窄到单一勾稽项，但那要先有实测数据才能定口径。

8. 归档段销毁的承载。DisposalRequest 的 scope 按计划14:335 封闭为四类，归档段不属其中；platform_ops 内也没有归档段清单表。要么新增第五类 scope（连带计划14:335 与 14:569 的「四类处置范围」同批改），要么在备份保留窗口机制之外另设一条归档段生命周期。本轮撤下，须运维与阶段 14 的负责人拍板。

9. 附录 A.6 是否为「从较早的备份恢复」增加一次验证。本轮撤下的理由是它撞 A.6 的 RPO 15 分钟判定且 A.6 是发布门禁唯一的可用性类判据。若确要验证，两条路各有代价：一是修订 A.6 的 RPO 判定口径使其对该次演练不适用（属规格级变更，须同批改规格:1864 与 :1867）；二是在 A.6 之外另设一次不进 A.5 发布判据的例行验证演练（不动规格，但该演练结果不构成发布门禁）。二者择一，须使用方定。

10. D-13-1 六项任务的效率验收整条重定。可保留的替身是「同一任务在其规格第 6.2 章矩阵允许的端上的完成用时与步数上限」——端别范围须逐任务按矩阵裁剪（不能四端各测一次），量纲须统一（要么全用击键加点击并给出实测基线，要么全用字段数，不能两者相减），来源标注那条要么换成「该值在平台内已存在且当前用户有权读取」的真实查询判定要么撤下。重定之前 D-3 的第二档表述（任何比较级）一律不得使用。

11. 阶段 3、6、7、9、10、12、13、14 八份计划各自的交付物、错误码、表、迁移、事件、视图、退出条件七类计数在本轮全部裁定落地后的新数值。本轮改动横跨八个阶段，每类计数都要加。本卷已因「凭记忆复述一张表而不回去数」在计数与枚举失配上犯过三次（00c 附录戊、F-09 第五节自纠），本份裁定不给出具体新数值——给了就是第四次犯同一个错。正确做法是入卷时逐份文件回去数一遍并当场核对，本条登记这是一个必须做且高危的动作。

12. 第 5 条冗余（是否采购第二台机器）。按主控前提本轮一字不裁。提醒一句：它会回头推翻裁定 F-08 的单机形态与附录 A.3/A.4/A.6 的全部认证与演练口径，因此它越晚定，返工面越大；D-1 的备份保留期与 D-2 的降级窗口都建立在单机形态之上。

---

## 五、落地次序

先说一条必须纠正的前提：主控交办文假设「合同终止与发票红冲的下游处置都挂在影响面机制上，它是前置」，这一半不成立。发票红冲的下游处置（释放核销转预收、追加 REVERSAL 台账条目、四个勾稽视图的期间累计）按 B-6 与 B-7 必须与红字凭证在同一事务内完成并共用同一次期间解析（计划10:802、规格:368），而影响面机制是 Outbox 异步驱动，异步做红冲会当场把子账腿甩到与总账腿不同的期间。因此本轮：影响面机制只挂合同终止一个上游事件，是合同终止全部下游处置的前置；发票红冲的下游处置走 B 簇同事务路径，与影响面机制无前后依赖。两条路各自成链。

落地次序，按依赖分五股。

第一股（最先，阶段 3，是 A 簇其余全部的硬前置）：A-1 影响面处置台账两张表与 ep-platform-impact 与 platform.impact_assess 消费者 → A-2 的 docs/impact-catalog.md 七条目录常量（A-1 的 item_total 按目录算定，目录不定台账建不起来）→ A-9 的两个事件名先登记再实现（00b:522 的纪律，是消费者的前置）。A-1 未落地之前，A-3、A-5、A-7、A-10、A-11、A-12 一条都做不了。台账落 3b-2 批，且必须排在阶段 6 之前，这是排期约束不是范围约束，须由阶段 3 与阶段 6 的排期方确认。

第二股（阶段 6，可与第一股部分并行但不得早于其完成）：A-4 词义钉死 → A-5 状态机四条边 → A-6a 的 chain_kind 取值 → A-8 收款计划期次加列 → A-7 的三条规则 → A-12 前置阻断 → A-10 的两条阶段 6 用例。其中 A-4 必须在 A-5 之前——不先钉死词义，状态机命名会被反复重新解释；A-8 的 DTO 变更必须与阶段 10 同批，不得阶段 6 单方面改完就走。

第三股（B 簇财务补偿，内部次序不可颠倒）：B-5 台账改追加式（含三处封闭 CHECK 新增取值）→ B-7 四个勾稽视图改按期间累计 → B-6 红冲释放核销转预收（B-6 要写的行只有在 B-5 的 entry_kind 与 source_type 取值落库之后才落得进去，其预收腿的勾稽口径又依赖 B-7 的逐视图公式）→ B-4 部分红冲与 B-8 发票行明细**必须同批**（B-4 单独交付只能让部分红冲在账上成立，PRD:373 与规格:312 的退货前置校验仍不可判；B-8 的判定式又依赖 B-4 的 reversed_quantity 累计列）→ B-3 资金单据冲正与 B-6 同批入卷（否则规格上留下「红冲已核销发票该走哪条路」的空洞，那正是盘点断点四的成因）→ B-1 更正凭证落阶段 9b（其守卫要重跑触及的勾稽项，须阶段 10 的子账侧全部接入，故必须排在 B-5/B-6/B-7 之后）→ B-2 与 B-1 同批（同一条决策的两半，不得先落一半）。

第四股（C 簇，C-3 是多处的前置）：C-3 默认审批链先落（A-6a 的 TERMINATION 链、B-1 更正凭证的链、B-3 冲正的链三处取值的唯一出处，不先落这一条，三处会各给一套）→ C-1 补偿策略表（其 approval_chain_code 指向 C-3 的链，且其基线中 VOUCHER_POSTED×CORRECTION_VOUCHER 一行的取值随 B-1 确定）→ C-2 可配阈值参数（其载体形态卡在 00b:577 那条未决矛盾上，须先解）→ C-4 能力闸次序落阶段 13。

第五股（D 簇，与前四股无依赖，可全程并行）：D-1 保留期与其治理 → D-2 自检项与十八改十九（D-2 的降级窗口承载 D-1 第五条的容量不足记录，故 D-1 的列先落）→ D-3 表述权限与诚实披露**必须最后**：它新增的三条披露内容直接取自 A-3/A-12、B-1/B-2、D-1 三处裁定，那三处定稿之前 D-3 无法定稿。

跨股的两条硬约束：一、A-7 第 5 条的采购需求处置与 A-9 第五条同批执行——改计划07:487 守卫措辞时必须连触发方一起改为由 ImpactRule::dispose 驱动，只加「或来源合同终止」六个字会把双写原样留在计划里。二、A-11 的逐阶段注册数期望表（3 / 4 / 6 / 7）必须与四个阶段的退出条件同批写入，晚一个阶段写，那个阶段的门禁就是空的。

---

## 六、代价合计

按三档分说，不压低。

**第一档 规格级变更（动规格加 PRD 加二到四个阶段，以月计），共 12 条。** A-1 影响面处置台账（规格第 5.2 章 CLM 条目与第 8 章、PRD 新增 3.5.5 节、阶段 3/6/7/10/12 五个阶段）、A-2 静态声明（新增第五份登记文件，00-overview R6 段「四份」改「五份」并波及各阶段退出条件）、A-3 三条驱动杠杆、A-5 TERMINATING 态与四条边、A-7 七条影响面规则、B-1 更正凭证入口（规格第 5.2 章加句、阶段 9 五处计数与登记同批改、C-26 全量类型码表新增 CORR）、B-2 手工凭证明写排除（PRD:4579 逐字要求回写规格）、B-3 资金单据冲正（规格第 5.2 章新增一段禁用场景）、B-4 部分红冲（规格:311 的「只允许冲回一次」必须改，PRD 第 6.4.7 节状态机与 6.5、6.6 三节）、B-6 红冲释放核销转预收（规格:311 的借方与贷方两栏各新增一项，PRD 三节，规格:310 一条分支改写，PRD 三处必填改条件必填）、C-1 补偿策略可配面与 C-2 前置条件阈值可配（两条都动规格第 9.1 章的低代码能力清单本身，这是盘点第 59 行已判明的档位）、D-3 表述权限与诚实披露（规格第 21 章新增 21.22 节、第 21.4 章签字对象扩面、PRD 第 11.11 节新增三小节、阶段 13 与 14）。这一档的真实工期不是各条相加：B-4 与 B-8 必须同批、B-3 与 B-6 必须同批、B-1 与 B-2 必须同批，三组各自构成一个不可拆的交付批次，任何一组拆开都会在规格上留下自相矛盾的半句。

**第二档 计划级新增（规格不动或只补条，动一到两个阶段，以周计），共 9 条。** A-8 收款计划期次加两列（由原「实现级补充」上调：改建表 DDL、改一个被 C-20 定为唯一跨模块出处的 trait 的返回 DTO、改阶段 10 取数口径、改一个承载规格:288 提醒要求的视图）、A-9 事件与登记、A-10 四条用例、A-11 跨阶段接线次序（四份计划的退出条件各加一至二条）、B-5 台账改追加式、B-7 勾稽视图改按期间累计、B-8 发票行明细（计划10:57 自评「多行明细为中」，本裁定不压低这个估计）、C-3 默认审批链（含 ep-datagen 默认数据集与阶段 4 退出条件同批改）、D-1 保留期与其治理（含阶段 14 四处计数）、D-2 自检项与 DegradationKind 十八改十九（动阶段 2 建表 CHECK 与阶段 14 十八项终态清单两个已由 A-26 冻结的落点，不因「只加一个取值」而降档）。

**第三档 实现级补充（一句话到一条用例，以天计），共 4 条。** A-4 词义（PRD 加一段说明、阶段 6 与 13 的文案、00b delegated 段加一行）、A-6a 新增 chain_kind 取值（规格与 PRD 一字不动，这是降级后的全部代价面）、A-12 前置阻断谓词扩面（PRD 两处逐字加一段例外清单）、C-4 能力闸次序与豁免边界。

**三档之外必须单独说的三笔代价。** 其一，客户落点容量：保留期由 7 天升到 14 天，客户要多约 2.6 TB（按公式代入 D=14 约 6877 GB、D=7 约 4285 GB），不是原裁定说的 2.1 TB——那个数只算了 7 代全量，丢掉了多出来的 7 天事务日志归档、配置包与 1.15 余量。其二，八份阶段计划的七类计数（交付物、错误码、表、迁移、事件、视图、退出条件）在本轮全部落地后都要改，本份裁定刻意不给具体新数值，入卷时必须逐份回去数——本卷已因凭记忆复述计数栽过三次，这是第四个高危点，数错的代价是下一轮复核整段推倒。其三，撤下的 A-6b（终止升第七类高风险）如果使用方后续要，它单独一条就是第一档里最重的一条：36 处「六类」口径同批改，其中三处是认证判定文本与性能认证的负载构成，最坏情况要重跑 A.2 的时延基线测试。

---

## 七、反方报告全文（81 条）

| 档 | 打的哪一条 | 类型 | 问题 |
|---|---|---|---|
| 必改 | A-1（影响面处置台账）criteria 三、A-2（静态声明）criteria 一、A-7（七条规则）criteria 一 —— 与 A- | 内部自相矛盾 | 三条裁定各自写死机检「`ImpactRegistry` 注册项数恰为七」「条数断言为七，不多不少」，而同簇 A-11 逐字规定「阶段 3 建两表与 crate 与消费者 → 阶段 6 建动作、状态机、审批链取值、三条规则 → 阶段 7 一条 → 阶段 10 两条 → 阶段 12 一条」。按 A-11，阶段 6 结束时注册数是 3、阶段 7 是 4、阶段 10 是 6，只有阶段 12 之后才是 7。这条门禁在阶段 6 至 11 之间必然红，实现方唯一的出路是提前注册空实现——而 A-11 二又逐字禁止「不得注入替身实现」。两条一起执行不出来。参照物 A-06 之所以能写「合计十五个」，是因为它没有配一条要求每个阶段都成立的机检。 |
| 必改 | A-11（跨阶段接线次序）第二条与 criteria 四 | 内部自相矛盾 | 同一段里给了两条互斥的口径：「尚未注册的规则其处置项不产出」与「已产出但目标阶段未接线的项 `state` 恒为 `PENDING`、不计入 `item_done`」。若前者成立，后者永远没有实例——A-11 criteria 四「未注册规则对应的 `impact_rule_code` 在 `impact_disposition_items` 中不存在任何 `state = 'DONE'` 的行」就是恒真门禁（该 code 一行都不存在）。更要命的是前者会开一个真实的窗口：阶段 6 到阶段 12 之间，`item_total` 只算已注册的三到六条规则，闭合判据 `item_done = item_total` 于是可以在采购需求、项目任务、销项发票、收款计划期次一项都没处置的情况下满足，合同直接进 `TERMINATED`。A-11 声称的「照抄正向」并不成立：正向计划06:429 逐字是「采购需求派生项在阶段 7 接线之前 `status` 恒为 PENDING……且不计入 `item_done`，因此含该项的合同停在 EFFECTIVE」——正向是**照常产出项、只是不推进**，A-11 第一句把它改成了「干脆不产出」。E2E-6-08 之所以看起来没暴露这个洞，只是因为它的夹具恰好有一张交付确认单。 |
| 必改 | A-6（终止列为第七类高风险操作）criteria 三「漏改防护」 | 判据不可判 | 两条 grep 判据都不成立。其一漏判：A-6 自己点名要改的规格:937 逐字是「合同生效、付款、开票、财务过账、结账和敏感导出的相关流程不得依赖仅存在于内存的状态」，整句没有「六类」二字，`grep -c "六类高风险操作"` 与 `grep -c "敏感导出六类\｜敏感数据导出六类"` 都命中不了它——这条门禁号称「防七处只改了三处」，却恰恰判不出它自己列的其中一处。其二恒假：判据要求「在 specs/ 目录下返回 0」，而 specs/ 下还有对历史事实的记述，盘点:24 逐字「规格:1041 保留了原 AI 风险分级里的六类高风险操作（合同生效、付款、开票、财务过账、结账、敏感导出）」、盘点:41 同类，以及 2026-08-17-ai-analytics-shape-design.md:508 逐字「六类里的「敏感数据导出」是 AI 分析最可能落进去的那一类……清单本身封闭为六类」。要让 grep 返回 0，就得回头改写盘点与设计稿里对当时事实的记述，即为了让门禁变绿而篡改档案。 |
| 必改 | A-6（终止列为第七类高风险操作）touches 与代价档 | 连带漏列 | touches 只列了「规格:1041、规格:1050、规格:1064、规格:937 四处逐字；PRD 第 10.3 节标题、PRD:3964、PRD:3966 表新增一行、PRD:4099」，并把这称作「一次浅层的枚举扩容」。实际逐字命中的「六类」口径至少还有：规格:500「L3 的六类高风险操作清单在首版仍然适用」、规格:1330「合同生效、付款、开票、财务过账、期末结账和敏感数据导出六类高风险操作按第 12.1 章验证重新认证」、规格:1433「第 12.1 章六类高风险操作的重新认证与审批机制必须完成允许与拒绝两条路径的验证」、规格:1835「本节业务负载及其中第 12.1 章六类高风险操作的实际发生频次」，以及 PRD:119、291、534、1012、1615、2095、2191、2297、2339、2522、2762、2889、3053、3489、3894、3979、3989、4052、4510 共二十余处。其中规格:1330 与 1433 是**认证与退出条件的判定文本**（改了要重跑验证面），规格:1835 是**性能认证的负载构成**（六类改七类要重标定发生频次），PRD:2889 的 F-13 与 PRD:4510 的 U-D-14 逐字以「六类高风险操作不含这三项」为立论前提，改成七类要连带复核这两条待决项本身。这不是浅层扩容。 |
| 必改 | A-6（终止列为第七类高风险操作）criteria 四 | 与既有条款冲突 | 判据逐字「`client_capability_values` 行数仍为 72，二进制内置冻结快照的哈希与改动前一致」。但使用方已表态的 F-09-3（服务器端要独立 UI）在 00c:2659 的强制连带表里逐字写着「能力矩阵｜由 18 个能力域乘 4 端共 **72 格**变为乘 5 端共 **90 格**；`client_capability_values` 新增一列；逐格核对与验收矩阵随之扩面」，同表下一行「冻结机制｜能力矩阵是编译期冻结进二进制的常量……扩端要重编二进制并重跑逐格核对」，且该表冠以「本条触发的连带逐项登记，不得漏改」。A-6 把一个已被在先裁定判定为必须变的量写成了不变量断言，两条同时入卷必有一条要作废。 |
| 必改 | A-1（影响面处置台账）第三条生命周期 | 连带漏列 | 生命周期只有四段，DEAD 之后没有出口。逐字「八次全部失败置 `DEAD` 并写入 `platform_msg.dead_letters`」「存在 `DEAD` 项时批次置 `FAILED`」，而批次 `status` 的 CHECK 只有 RUNNING、DONE、FAILED 三值，闭合又要求 `status = 'DONE'`，全簇没有任何一处定义 FAILED 怎么回到 RUNNING、DEAD 项怎么重放。它照抄的正向恰恰有这一步：计划06:433 逐字「7. 人工修复后可重放该批次，重放按第 3.3 小节的两道唯一约束去重，不产生重复单据」。少这一步的后果不是少一个功能，而是把 A-3 甲的状态阻断变成死锁：只要有一项八次失败，合同永远停在 `TERMINATING`，按 A-12 又不能开票、不能发货、不能改单——A-12 自己的论证逐字说这种结局「比不做还糟」。 |
| 必改 | A-2（静态声明）criteria 一与二 | 判据不可判 | 两条判据都押在 `xtask archcheck` 的 `db-pg-one-schema-per-file` 上，但这条规则判不到被测对象。00b:118 逐字界定其判定面为「`ep-adapter-db-pg` 中的仓储实现按 schema 分文件……只在双引号字面量区间内取 `<schema>.<object>`，文件内出现自身 schema 之外的非 `v_` 对象即违反；`crates/adapter/db-pg/src` 下不落在任何 schema 目录内的文件，出现任何登记 schema 的对象同样违反」。而 A-2 自己规定 `ImpactRule` 的实现「落在被影响模块自己的 `ep-app-<m>`」，根本不在 `crates/adapter/db-pg/src` 之下。于是 criteria 一「对新增的七个实现文件全绿」恒真（无对象可判），criteria 二要求「构造一个跨 schema 读的 `ImpactRule` 负样例，断言 `db-pg-one-schema-per-file` 报错」必然做不出来——它不会报错。这条负样例本来是全簇唯一一条用来证明架构门禁在本机制上真生效的断言。 |
| 必改 | A-2 criteria 三 与 A-7 criteria 四（七条规则的逐条覆盖表） | 判据不可判 | 两处判据逐字相同：「七条规则对 PRD:1024-1028 五类派生对象与规格第 8 章第 7、8、13 步三类已发生事实的逐条覆盖表，缺一不通过」。规格第 8 章第 13 步是「记账与结账」，其已发生事实就是已过账凭证。而 A-7 在同一份裁定里明确「不设第八条「凭证处置项」」，理由逐字是「单设一个凭证项会产生一个没有任何可用动作的项，它永远闭合不了」。两条并存的结果是：覆盖表按定义永远缺第 13 步那一类，这条「缺一不通过」的评审判据恒假。要么把第 13 步从覆盖面里显式摘出并写明理由，要么改判第八条，二者必居其一。 |
| 必改 | U-H-07 更正凭证入口（守卫二与守卫三，以及「正向 A」判据） | 内部自相矛盾 | 守卫二逐字「每一行必须且只能引用一条已过账凭证行或一条已确认期初余额批次行」，守卫三逐字「对同一被引用行的累计更正金额（含本次）不得超过该行金额……按被引用行逐行独立计算」。而正向 A 判据逐字「把一笔已过账的直接费用金额由科目甲重分类到科目乙」，该更正凭证必然是两行（借科目乙 X、贷科目甲 X），两行都得各引用一条来源行，唯一自然的来源就是原凭证那条「借 科目甲 X」——对该来源行的累计更正额 = 2X > X，守卫三当场拒绝，正向 A 恒失败。裁定通篇没有给出「哪一行该引用哪条来源行」「来源行的科目是否须与更正行的科目相关」任何规则，于是守卫三同时是两种坏：随便指一条尚有余量的已过账行就能绕过（不构成任何实质约束），或按字面执行就打死最标准的两行重分类。 |
| 必改 | U-H-07 更正凭证入口（守卫五「同事务内重跑十项勾稽，任一项差额非零整笔拒绝」） | 判据不可判 | 守卫五判的是全局十项勾稽而不是本次分录触及的项。规格:1003 逐字「校验不通过时生成对账差异事项交数据责任人处理，差异清零前不得关账」——差额已存在的那一刻，正是唯一需要更正凭证的场景，而守卫五在该场景下对任何更正凭证一律返回 RECONCILIATION_BROKEN。裁定自陈开口的理由是 PRD:4578 逐字「不决策则首版没有任何过账更正入口」，守卫五恰恰把入口在唯一用得上的时候关死，成了「只在不需要它的时候可用」。unresolved 第二条只登记了「执行代价数不出来」，没登记这条语义死锁；且十项中的存货与已收货未收票两项子账侧来自阶段 8、7 的外部端口（计划10:517 起第 3.3 节），同步事务内是否可调也未登记。 |
| 必改 | U-H-07 更正凭证入口（touches 只写「第 9.3 节新增两张表与两个迁移文件」） | 连带漏列 | 阶段 9 至少四处计数与清单被漏。计划09:41 逐字「db/migrations/ledger/ 下的 16 个迁移文件……执行后 ledger schema 存在 12 张表与 2 个视图」；计划09:793（E-14）逐字「docs/data-dictionary/ledger.md 的 12 张表与 2 个视图……全部登记」；计划09:710 逐字「ledger 的 8 张带法人列的表……在读取、写入、更新、删除、聚合、排序、报表投影与错误信息泄漏八类上不越权」（新增两张带法人列的表须建 rls_*_le 策略并进 tests/rls_matrix，正撞裁定 C-05）与其退出条件 09:773；计划09:869 逐字「写审计的动作清单固定 14 个」，而本裁定把更正凭证定为规格:1050 的财务过账高风险类、必须重新认证加审批，审计清单必须由 14 改。本裁定在连带二里专门要求普查「17 改 18」这个数，却漏掉自己新增两张表引出的这四个数。 |
| 必改 | 盘点硬伤二「红冲释放核销转预收」与硬伤一「台账改追加式」（新写的子账行撞现有封闭 CHECK） | 连带漏列 | 三处封闭 CHECK 挡住本裁定要写的行，touches 一处也没列。其一，计划10:366 逐字「source_doc_type ｜ text ｜ 否 ｜ ck_advance_receipt_entries_source_type 取值 RECEIPT、MIGRATION_OPENING」（预付侧同构取 PAYMENT、MIGRATION_OPENING），而硬伤二要在红冲事务内「新增一条 finance.advance_receipt_entries 预收条目」，它既非到款也非期初导入，落不了库。其二，计划10:489 逐字核销关系行的「source_doc_type（RECEIPT、ADVANCE_RECEIPT、CUSTOMER_REFUND、CASH_DOC_REVERSAL）」没有发票冲销这个取值，红冲释放追加的反向核销行同样落不了库。其三，计划10:341 逐字「ck_receivable_entries_source_type 取值 SALES_INVOICE、MIGRATION_OPENING」，硬伤一新增的 entry_kind='REVERSAL' 行取哪个值裁定没写。连带还漏了 PRD 第 6.11 节（预收台账条目的产生方式，计划10 自陈「直接支撑 PRD 第 6.11.3 的两条校验」）与第 3.6 节迁移清单。 |
| 必改 | 盘点硬伤二「红冲释放核销转预收」（新增计量项 released_settlement_amount 与其 JOURNAL_MAP 行） | 代价被压低 | 规格:298 逐字「内置固定的业务事件到分录映射，覆盖下方事件-分录表列出的……十类事件」；计划09 第 9.4.3 节逐字「表的内容一律按规格第 5.2 章事件-分录表填写」；计划09 风险四逐字「在测试中对每个 source_kind 断言其涉及的科目角色集合与该表一致」。规格:311 红字冲销与作废那一行的借贷两栏现在没有预收账款、也没有预付账款。本裁定要新增「借 ACCOUNTS_RECEIVABLE、贷 ADVANCE_FROM_CUSTOMER」这条腿，touches 却只写「规格第 5.2 章（红字冲销与作废事件的附加规则新增「已核销款项转预收或转预付」一句）」——附加规则栏加一句不能给 JOURNAL_MAP 新行提供依据，风险四那条核对与断言当场不成立。要改的是规格:311 的借方与贷方两栏，不是附加规则栏。 |
| 必改 | 盘点硬伤三「勾稽视图改按期间累计」第 (2) 条「v_recon_payable、v_recon_advance_receipt、v_rec | 内部自相矛盾 | 硬伤一逐字只给「两张表各新增三列：entry_kind text 非空……」，且只加在 finance.receivable_entries 与 finance.payable_entries 上。finance.advance_receipt_entries（计划10 第 3.2.5 节列清单）与 advance_payment_entries 没有 entry_kind，也不产生 REVERSAL 行，硬伤三给的 sum(case when entry_kind = 'INVOICE' then original_amount else -original_amount end) 在这两个视图上照抄落不了库，「同构改写」四个字不可执行。更麻烦的是同簇打架：硬伤二对 origin = AUTO_ADVANCE 分支逐字「把原预收条目的 open_amount 回增」，这是原地改一行会计期间属于旧期间的老行，正是硬伤三判定为「回溯改写一个早期期间的切片」而必须废止的写法，两条裁定在预收这一腿上给出相反口径。 |
| 必改 | U-D-09 是否允许部分红冲（touches 与 knock_on 的落点清单） | 连带漏列 | 被本裁定直接推翻的条款有四处未列。PRD:2393 逐字「同一张发票的作废与红字冲销互斥，只允许其一，且只允许冲回一次」与 PRD:2388-2391 的四行状态表（PRD 第 6.4.7 节销项发票状态机）——touches 只写到「PRD 第 6.5 节」「第 6.6 节」；PRD:2417 逐字「原发票 ｜ 是 ｜ 只能选择状态为已开具的销项发票」，部分红冲后原发票是 PARTIALLY_RED_REVERSED，按此条再也选不出来；计划10:691 逐字「ISSUED 到 VOIDED、ISSUED 到 RED_REVERSED，两者互斥，各自只允许一次，VOIDED 与 RED_REVERSED 为终态。守卫为：invoice.invoice_reversals 中不存在以该发票为 source_invoice_id 的行，由唯一约束兜底」（计划10 第 4.2 节状态机，touches 只列到 4.4 与 4.9）；计划10:859 的端点请求体与错误码列（touches 完全没有阶段 10 第 5 节）。 |
| 必改 | 发票行明细裁定给出的 is_fully_credit_noted 实现式 | 内部自相矛盾 | decision 逐字「对该订单行的全部发票行求 sum(quantity) - sum(reversed_quantity)，若剩余已开票数量小于入参 quantity 则返回已全额红冲，否则返回未冲销的发票清单」。代入本裁定自己的 criteria 一逐字「一张覆盖五台设备的销项发票，对其中一台登记部分红冲，随后登记该台的销售退货 → 通过」：sum(quantity)=5、sum(reversed_quantity)=1、入参 quantity=1，4 < 1 不成立，于是返回未冲销的发票清单 → 阻断。该式没有把已退货数量纳入，凡是「部分红冲后退这一部分」的场景一律误阻断，即本裁定声称要打通的 PRD:373 与规格:312 那条路，在它自己的公式下仍然走不通。 |
| 必改 | 甲档 决策三 + 决策五 + criteria 二 + criteria 四 | 内部自相矛盾 | 决策三逐字要求 compensation_policies「常量为运行期权威，数据库表只是机器可读副本，全表按（stage_code, action_code, is_allowed, requires_reauth, requires_reason, requires_attachment）排序后取 SHA-256 与内置快照比对，不一致时以内置快照为判据继续运行、拒绝对该表的一切写入并持续告警」。这套写法照抄自 13-clients-lowcode.md:520「二进制内置的冻结快照本身就是权威……同时拒绝一切对该表的写入并持续告警」，但那张 client_capability_values 是零可配面的全冻结表，而本裁定第五条同时声明客户可以把 is_allowed 由 true 改 false、把 requires_* 由 false 改 true。两者不能并存：只要客户经 AuthzPolicyApplier 合法发布一次收紧包，表哈希立刻偏离内置基线，系统随即进入「拒绝一切写入 + 持续告警 + 以基线为运行期判据」状态——收紧永远不生效，且此后任何配置包都写不进这张表。criteria 二要求「只收紧的包，同一 suite outcome=PASSED 且发布成功、回退成功」，criteria 四要求「篡改 compensation_policies 一行后……对该表的写入被拒且告警已发」，哈希比对分不出「篡改」与「合法发布」，两条判据不可能同时为真。 |
| 必改 | 甲档 knock_on 二（EP__FINANCE__RECEIPT__REQUIRES_APPROVAL 与 EP__FINANCE__C | 内部自相矛盾 | 迁入目标不存在。决策一逐字把列集封死为九列加公共列「多一列即视为越界」，九列里没有任何表达「本动作是否需要审批」的布尔列；决策四把基线行集封死为 13 行且逐行点名，其中既没有 RECEIPT_REGISTER 行也没有 CASH_ACCOUNT_MAINTAIN 行——13 行里与到款有关的只有 RECEIPT_REGISTERED×CASH_DOC_REVERSAL（冲正到款单据），与「到款登记本身要不要审批」是两件事；资金账户档案维护更不是补偿动作。其次是无声推翻已生效裁定：10-ar-ap-invoice.md:68 逐字「到款审批与资金账户审批两项恢复为配置项 EP__FINANCE__RECEIPT__REQUIRES_APPROVAL 与 EP__FINANCE__CASH_ACCOUNT__REQUIRES_APPROVAL」，而后者默认值为 true（10-ar-ap-invoice.md 第 7 节配置项表逐字「EP__FINANCE__CASH_ACCOUNT__REQUIRES_APPROVAL ｜ bool ｜ true」）；迁进一张「只有减法面与加码面，没有放宽面」的表后，客户再也关不掉它，F-13 给出的可配面被删掉却未在任何一处说明。第三是与本簇自身打架：第 9 条决策四(3) 逐字「RECEIPT_REGISTER，出厂不建链，对应 EP__FINANCE__RECEIPT__REQUIRES_APPROVAL 默认 false」，把它当配置项保留，而第 9 条 knock_on 五又要求按甲档迁列并从配置文件删除。 |
| 必改 | 乙档 决策二(甲) 与 决策六 + criteria 一 与 criteria 四 | 内部自相矛盾 | 决策二(甲)把不可配面的判据定为「该迁移触发的事件名出现在 ledger.posting_trigger_event_types 的 13 行中」。00c-gap-ruling.md 的 A-21 把这 13 行逐行列死，其中含 `｜ 10 ｜ invoice.sales_invoice.reversed.v1 ｜ INVOICE_REVERSED ｜` 与 `｜ 10 ｜ invoice.purchase_invoice.reversed.v1 ｜ INVOICE_REVERSED ｜`。发票作废走的正是这个事件，因此 INVOICE_VOID 这条迁移按本裁定自己的判据落在不可配面。而决策六逐字把「开票 7 天内允许作废」判为「落在本档」，并要求给 INVOICE_VOID 的前置条件追加 now()-issue_date<=7 天，criteria 四还把它做成阶段 10 的验收用例。criteria 一的 configurable-transition-disjoint 规则（断言可配前置条件项对应的事件名不出现在 posting_trigger_event_types 中，出现即构建失败）会直接把 criteria 四判死。使用方举的唯一具体例子在本裁定内无处落地。 |
| 必改 | 乙档 决策三其一 + criteria 一（xtask archcheck 新增 configurable-transition-disjo | 判据不可判 | 被测对象不在 archcheck 的判定面里。可配前置条件按决策四是「以已有的声明式 AST 表达……承载与下发走已有规则表」，即经配置发布通道写进事务数据库的运行期行；posting_trigger_event_types 同样是 ledger schema 的数据库表（A-21 由阶段 9a 种子迁移写入）。而 00b-technical-baseline.md:120 逐字「另在 CI 中运行一段基于 `cargo metadata` 的自检脚本」、00b:122 逐字「凡在 `cargo metadata` 之外另需调用图分析的断言，本基线不认其为已可判定」，01-engineering-baseline.md:57 与 497、498 所列的 archcheck 规则一律以源码树与依赖图为输入。构建期的 archcheck 既读不到数据库，也读不到尚未存在的客户配置包。这条规则按现有工具形态判不出真假，criteria 一「配一条负样例，构建失败」也就无从构造。要么改判到发布期 suite 一侧，要么按通则第六条如实登记为不可判定。 |
| 必改 | 第 9 条 决策二(甲) + 决策四 + criteria 一/三 | 连带漏列 | 「无链即拒」的作用面比给出的默认链大。决策二(甲)逐字把范围写成「属规格:1041 六类高风险操作的……链缺失或链节点展开为空一律拒绝提交」，规格:1041 六类为合同生效、付款、开票、财务过账、结账和敏感导出。决策四的 12 行里没有合同生效、没有开票、没有敏感导出三类的链。04-identity-authz.md:8 逐字「T0 用到的身份数据只有……四条单节点审批链，四条的 scenario 依次为合同生效、发票申请单、销项发票开具与资金账户建档」，04:374 的守卫又逐字要求「已存在生效的审批链定义」。按本裁定落地后，出厂默认数据集上合同生效与开票没有链——T0 贯通线在建单与开票两步各断一次，敏感导出全线不可用。criteria 一 又把默认 scale 数据集钉成「本裁定第四条列出的 11 条链……链数与节点数逐行相等」，与 04:29 逐字「默认 scale 数据集生成 2 个法人、50 名命名用户、角色与授权集合」（不含任何链）以及 04:743 的退出条件都要同批改，touches 只写了「ep-datagen 生成」，没点这两行。 |
| 必改 | 第 9 条 决策二(甲) 与 决策四(3) 与 criteria 三 | 内部自相矛盾 | 到款登记被同一条裁定同时判成两种。决策二(甲)逐字「本条点名四项全部落在这一类」，而主控点名的第二项按 PRD:4442 的 U-A-08 原文是「到款与付款与退款登记」；按决策二读，到款登记无链即拒。但 10-ar-ap-invoice.md:68 逐字「到款登记不需重新认证也不需审批」，决策四(3) 自己也写「RECEIPT_REGISTER，出厂不建链」。两者叠加的结果是：出厂状态下到款登记既没有链又必须有链才放行，该路径直接死掉。criteria 三 又悄悄把被测项写成「发票作废与红冲、付款与退款登记、期间关账与年结、更正凭证」，把「到款」从四项里删了——判据与裁定正文对不上，正文的错误不会被任何用例照出来。 |
| 必改 | 第 9 条 决策三（五个业务角色码） | 与既有条款冲突 | finance.accountant、finance.manager、sales.manager、procure.manager、ops.data_owner 五个码违反已冻结的 RoleCode 形态。00b-technical-baseline.md:219 逐字「`RoleCode(Arc<str>)`，取值为长度 1 至 64 的 `[A-Z0-9_]`，与 `platform_authz.roles.code` 逐字一致」，01-engineering-baseline.md:157 同样逐字写「`RoleCode(Arc<str>)` 取长度 1 至 64 的 `[A-Z0-9_]`」，且 01:516 把它列进阶段 1 的跨阶段冻结项、由 archcheck 的 foundation-frozen-items 守。裁定括注「RoleCode 为小写字母、数字、下划线与点，长度 1 至 64」并称「字符集合规」，实际只对上 04-identity-authz.md:282 那一处相反写法（「字符集为小写字母、数字、下划线与点」），既未给行号，也未登记 00b/01 与 04 本就互相冲突这件事。按 00b 与 01（技术基线与冻结阶段，权威高于阶段 4 计划）读，这五个码一律 `RoleCode::parse` 失败，种子配置包写不进 platform_authz.roles.code。 |
| 必改 | 第 11 条 决策四 vs 决策五 vs touches vs criteria 三/四 | 内部自相矛盾 | 同一条阶段 4 用例被同一份裁定同时要求改与不改。决策四逐字「裁定：次序为能力闸在前，重新认证挑战在后；阶段 4 该用例的期望错误码在阶段 13 交付时同批改为能力闸的错误码，改动登记在阶段 13 的退出条件里」；决策五随即逐字把「/api/v1/platform/reauth-challenges」整段列进能力闸豁免清单，「这两段不走能力闸」——能力闸永不触发，期望错误码根本不该改，决策四那条裁定连同它的理由（「阶段 4 的该条用例在阶段 13 之后必然转红」）当场作废；touches 里写「阶段 4：……该段用例的期望错误码保持不变」；criteria 三断言 operation_type 取 Payment 时仍返回 PLATFORM.HIGH_RISK_REQUEST.CLIENT_NOT_ALLOWED（不变），criteria 四又断言「阶段 4 的期望值已随本裁定改齐」（已变），且 criteria 四自身前后半句就自相矛盾（「仍全绿」对「已改齐」）。四处两两冲突，落码方无从下手。 |
| 必改 | 第 11 条 决策五（能力闸路由豁免清单含 /api/v1/platform/high-risk-requests*） | 与既有条款冲突 | 整段豁免会把移动端审批放出来。规格 2026-07-19 第 577 行逐字「到款登记、付款登记、发票申请、发票开具登记与账龄对账在移动端只提供查询与查看，提交、审批与写入操作转桌面端完成」——审批被明写在转桌面端之列。04-identity-authz.md:512 的服务端判定只作用于 POST /api/v1/platform/reauth-challenges 的四类 operation_type 签发，管不到 high-risk-requests 路由上的 approve、withdraw、confirm_execution 等写动作（04:370 状态机的 IN_APPROVAL→APPROVED 等迁移）。这些动作一旦不过能力闸，就没有任何服务端端别判定拦它们，移动端可直接审批一张付款高风险单。同时，规格第 6.2 章逐字「同一功能同时落入两个能力域时，以取值较低的所在行为准」在被整段豁免的路由上无处施加——「审批待办与站内通知」四端为完整、「收付款登记与对账查看」移动端为仅查看，取低那一条正是靠能力闸生效的。裁定第六条据此宣称「服务端是权威在首版是真的」，实际是在移动端审批面上开了个口子。 |
| 必改 | 第 11 条 决策二（改写句）与 criteria 一（逐端点覆盖） | 判据不可判 | 作用域与端点数两处都不对。10-ar-ap-invoice.md:905 落在第 5.5 节（`#### 5.5 台账、账龄与对账` 在第 889 行，`#### 5.6 幂等语义` 在第 907 行）的末尾，句中的「本节」= 第 5.5 节，该节实有 3 个 POST、0 个 PATCH、8 个 GET；整个第 5 节（833 至 983 行）实有 19 个 POST、2 个 PATCH、17 个 GET。裁定的改写句照抄「本节」二字，criteria 一却按「本阶段第 5 节端点表中的全部 POST 与 PATCH 行（POST 已数出 20 个）」驱动——20 这个数既不是 3 也不是 19，来源不明。两处后果：按改写句字面读，规格:577 点名的到款登记、付款登记、发票申请、发票开具登记四类端点全在 5.1 至 5.4 节，不在改写句覆盖面内，本裁定要闭的正是这个缺口；按 5.5 节读，PATCH 是空集，「逐个断言 X-Client: ios 返回 403」在空集上恒真。 |
| 必改 | D-13-1 第二段「T1–T5 四端各测一次」与第 22 章新增第 16 条 | 判据不可判 | 裁定用 PRD:4272 排除了 T6 的移动端，却对 T2–T5 视而不见。PRD:4272 逐字：「移动端只提供查询与查看、写入转桌面端的能力域按规格第 6.2 章矩阵，至少含财务过账与期末结账、收付款登记与对账查看、发票申请与开具登记、报表与像素级打印、文档与附件协作、系统管理与低代码配置六类。」T2 发票申请提交、T3 发票开具登记、T4 红字冲销登记全部落在「发票申请与开具登记」，T5 到款登记并核销落在「收付款登记」。这四项任务在移动端不存在写入路径，「四端各测一次」的用例在移动端必然失败，而第 22 章第 16 条不通过即正式版不得完成。这正是同一条裁定 reasoning 里自称要避免的「必然失败的判据」，用的还是同一行证据。 |
| 必改 | D-13-1 第三段（b）「A ≤ N_min + 8」 | 判据不可判 | A 的定义逐字是「键盘按键次数加指点设备点击或轻触次数」，N_min 的定义逐字是「该任务的最小必要输入字段数」。两者量纲不同：一个数击键，一个数字段。任一含文本或日期录入的字段单独就要十来次击键，T1 销售建单按 N_min = 8 计只给 16 次击键加点击的总额度，包含导航、确认与提交在内，任何真实实现都过不了。这条通过线在实践中恒假，且它是第 22 章第 16 条与门禁项 RG-TASK-ECONOMY 的组成部分。配套的豁免条款「同一任务不得连续两个版本豁免」在首版没有前一个版本，该分支恒真，于是这条判据要么必输、要么无牙，两种形态都被通则第六条禁止。 |
| 必改 | D-13-1 criteria（a）「T = 0」的判定方式 | 判据不可判 | T 的定义是「其取值在本平台内已存在且该用户有权读取的字段个数」，而 criteria 判的是「逐输入控件断言其取值来源标注为 USER_NEW_FACT 或 SYSTEM_PREFILLED 二者之一，出现第三类即用例失败」。一是被测对象换了：判的是开发者自己打的标注，不是「该值是否已在平台内存在」，把一个转录字段标成 USER_NEW_FACT 即可通过；二是若来源标注确如裁定所写只有两个取值，「出现第三类」在枚举上不可能成立，整条断言恒真，正撞通则第六条第三句（00c:1691「亦不得以「计数照旧」或「两个空集合比对」的形态退化为恒真」）。 |
| 必改 | D-12-1 criteria（二）与 D-12-4 第三段的 backup-retention-window 自检项 | 与既有条款冲突 | 裁定给该自检项定 severity 取 Degrading，并把 DEGRADED_BOOTSTRAP（部署运行未满 D 天）列为正常态。但 14:452 逐字：「--check 模式执行本进程适用的全部已注册自检项并按注册顺序输出结构化报告后退出，任一项为 FAILED 或 DEGRADED 均以非零码退出，用于部署验收与升级前置校验」；14:553 逐字要求三个进程「报告中无 FAILED 也无 DEGRADED；并在生产配置下连续运行不少于 7 个自然日」。D 取 14 时，任何新部署头 14 天必然 DEGRADED，于是交付日的部署验收 --check 必然非零码退出，阶段 14 退出条件第 1 项（只跑 7 个自然日）永远不可满足。裁定的 touches 与 knock_on 对这两处一字未提。 |
| 必改 | D-12-1 criteria（三）挂在第 22 章第 7 条上的那句判据 | 判据不可判 | 该句为「该部署的 backup-retention-window 自检项在发布证据包采集时点为 PASSED 或 DEGRADED_BOOTSTRAP」。发布证据包在认证环境上采集，而按 14:553 该环境只连续运行 7 个自然日、按 D-12-1 锚点定义需运行满 D = 14 天才可能出现 PASSED，因此采集时点必为 DEGRADED_BOOTSTRAP，该判据在唯一被评估的环境里恒真通过。恒真门禁比没有门禁更坏，且本簇自己在 D-12-4 reasoning 里刚论证过「认证环境不会真的跑满 D 天」，却把这句留在了第 22 章上。 |
| 必改 | D-12-1 第一段、D-12-5 第三段与 D-12-2／D-12-5 criteria 的区间守卫 | 内部自相矛盾 | D-12-5 第三段逐字「上调（超过认证取值 14 天）：允许，但三条前置缺一不可」，D-12-1 第一段亦写「除非按 D-12-5 在该部署重做演练」。但 D-12-2 criteria（一）与 D-12-5 criteria（一）同时要求落库约束 ck_deployment_records_retention CHECK (retention_days between 7 and 14) 与 ck_retention_days CHECK (retention_days between 7 and 14)，数据库层直接拒收大于 14 的取值，上调路径不可达。同一处 D-12-5 criteria 还自相打架：既写「断言 deployment_records.retention_days ≤ 认证报告记录的认证取值」，又写「超出时须存在该部署的 drill_report_ref」——前半句成立则后半句永无触发机会。 |
| 必改 | D-12-2 criteria（一）ck_offsite_sinks_capacity 与第六段容量不足处置 | 内部自相矛盾 | criteria 要求 CHECK (media_type = 'NONE' or capacity_bytes >= capacity_floor_bytes)，第六段却要求「offsite_sinks.capacity_bytes < capacity_floor_bytes 时…按第 15.3 章开窗持续告警并书面告知客户」。容量不足的行被 CHECK 拒收，该状态在库里无法存在，第六段整段处置连同「反解可支撑的最大 D」成为不可达代码；同时它也让平台无法记录一个真实存在的客户落点，与 spec:1220「该状态不得静默通过」的口径相反。另外该 CHECK 只对 media_type = 'NONE' 开例外，14:96 逐字 media_type CHECK in ('ONLINE','OFFLINE','NONE')，spec:1218 允许客户只提供离线介质，此类部署的单盘容量低于 S_floor 时同样被拒收，误伤一条已生效的降级路径。此外 D-12-4 只新增了 BACKUP_RETENTION_WINDOW_SHORT 一个 kind、四个 basis（BOOTSTRAP／ANCHOR_MISSING／GENERATIONS_SHORT／ARCHIVE_GAP），没有一个能承载「落点容量不足」，第六段要求的开窗无 kind 可用。 |
| 必改 | D-12-3 criteria（三）作用域静态比对 | 内部自相矛盾 | criteria 逐字「断言 backup_retention_policies 表只有 retention_days 与 min_valid_generations 两个可配取值列，无任何按对象类型、密级、法人或业务规则分列的列；CI 以列清单静态比对，多一列即判违反」。而同一条裁定第二段给出的建表列是「id、security_level、data_scope_tags、retention_days、min_valid_generations、approval_ref、approver_id、second_approver_id、reauth_ref、effective_from、superseded_at、公共列」——security_level 正是密级列、data_scope_tags 正是范围标签列，且按 14:90／14:96／14:119 这两列是 platform_ops 全表的公共列，不可省。按字面执行，这条 CI 闸门对自己的 DDL 恒判违反，构建永远红；按宽松执行则无判据。 |
| 必改 | D-12-1 第三段（乙）与第五段守卫（4）的归档段销毁 | 发明了不存在的机制 | 裁定要求「保留自锚点全量的 base_lsn 起的全部归档段，早于该 LSN 的方可销毁」，并把「销毁后自 base_lsn(A) 起的归档段序列连续无空洞」写成守卫之一。但 14:335 逐字「DisposalRequest.scope 取 AttachmentObjects、KeyDomain、BackupSets、ExtTables 四者之一」，归档段既不是 backup_sets 行也不属这四类，没有任何处置范围承载其销毁；platform_ops 内也没有归档段清单表（14:109 的 writeout_runs 只记 channel 与 period_seq 与字节数，无段身份与 LSN）。criteria 把被测输入写成「落点上的归档段序列」而未给提供方与交付阶段，直接违反通则第六条第一句（00c:1689「必须在同处写明被测输入的提供方与交付阶段」）。要么新增第五类 scope（连带 14:335、14:569 的「四类处置范围」同批改），要么整条撤下，裁定两者都没写。 |
| 必改 | D-12-1 touches／D-12-3 touches 的阶段 14 连带 | 连带漏列 | 本簇新增一张表（backup_retention_policies）与一个视图（v_retention_status），却只列了「五视图改六视图」。漏了四处同批必改：14:554 逐字「第 3 节的 17 张表（其中 degradation_windows 由阶段 2 建立…）、5 个视图与 21 个迁移文件全部落库」（表数、视图数、迁移数三个计数）；14:574 逐字「本阶段新建的 16 张 platform_ops 表在 platform_core.unpoliced_table_registry 中各有一行登记，schema_name、table_name、admission_basis、isolation_entry 与 matrix_case_id 五列取值齐备」（计数加一行新登记）；14:178 逐字「platform_ops.v_ops_health：ops-agent 与门禁工装的单一入口，聚合上述四个视图的关键取值」（新增视图后为五个）；以及新增自检项的注册连带——00b:603 逐字「追加项名与其 severity 在各阶段计划中登记，全量清单以总览第 4.3 节 C-25 行为唯一出处」，总览 C-25 行未列入 touches。 |
| 必改 | D-12-4 第四段（甲）对附录 A.6 整机失效恢复行的追加 | 与既有条款冲突 | 裁定要求两次演练中至少一次「恢复目标点取落点上最早有效 DAILY_FULL 的 verified_at 之后 15 分钟处（而非最新恢复点）」，并声明「其余判定标准（RTO 4 小时、通过第 17.3 章全部强制不变量校验、附件元数据与正文逐条一致）一字不放宽」——枚举里恰好漏掉 RPO。而 spec:1864 该行判定标准逐字为「RTO 不超过 4 小时，RPO 不超过 15 分钟，恢复后通过第 17.3 章全部强制不变量校验，并逐条校验附件元数据与正文」，spec:1867 又写「RPO 判定以第 13.4 章约定的事务日志归档到该服务器之外的周期不超过 15 分钟为前提」，spec:1219 写「判定标准不因个别部署降级而放宽」。恢复到十几天前的目标点，其恢复点距现在以天计，该次演练按现行判定标准必然 RPO 不达标；再叠加 spec:1867「两次均达标才判定通过」与 spec:1853「发布门禁的可用性类判据只有一项：A.6 判定表的全部演练逐项达标」，这一改动直接把第 22 章第 7 条变成必不通过。要么同批修订 A.6 的 RPO 判定口径（属规格级变更，裁定未列），要么这一支撤下。 |
| 必改 | D-13-3 touches「PRD：不动」与 knock_on 第一段 | 连带漏列 | 裁定称「经核对，现行八条的具体内容未在计划14 内逐条列出，只有一个计数——这本身就是一处隐患：一个只有计数没有枚举的清单，改动时无法核对」，并把此列为建议不列为必改。事实相反：14:568 逐字「PRD 第 11.11 节八条诚实披露文本已进入交付说明与客户合同模板，并在产品界面可达处呈现」，PRD 第 11.11 节 11.11.1 至 11.11.8 八小节逐条俱在（PRD:4325、4329、4338、4352、4356、4360、4369、4373）。因此「八条改十一条」必须同批动 PRD 第 11.11 节（新增三小节）、14:568 退出条件第 16 项的计数、客户合同模板，以及「在产品界面可达处呈现」所对应的界面改动（阶段 13，不在裁定所列的单阶段范围内）。裁定 touches 写「阶段 14（诚实披露文本八条改十一条、CI 文本检查一条）。单阶段」，并在 D-13-2 与 D-13-3 两处写 PRD 不动，全部落空。附带：该项在计划 14 的实际行号是 14:45，裁定两处（D-12-2 touches 与 D-13-3 knock_on）都写成 14:44。 |
| 必改 | D-13-3 criteria（二）CI 文本检查 与 D-13-2 第一段规格 21.22 | 内部自相矛盾 | D-13-3 要求「CI 增一条文本检查，断言规格与 PRD 全文不出现「碾压」「行业模板」「实施顾问」「生态伙伴」四个词作为承诺性表述」，而同簇 D-13-2 要求在规格第 21 章新增 21.22 节，其中必须写明「可使用「体验碾压」这一表述，须两条同时成立」与三档表述权限——规格全文必将出现「碾压」。两条裁定必须同批落，落地即互相判违反。「作为承诺性表述」这一限定又不是文本检查能判定的，只能退化为子串匹配（则恒假）或人工评审（则不是机检）。另有一处现成承接方被漏掉：14:568 逐字已有「交付、认证与验收材料经文本检查未出现高可用、零停机、自动切换、受控读取、法人隔离、等效、已满足、优先级隔离、资源隔离、性能保证十项禁用措辞」，把新词并入这份既有清单比另造一条只覆盖两份文件的检查更省且覆盖面更大，裁定对该机制只字未提。 |
| 应改 | A-4（词义：作废与终止）第一条 | 与既有条款冲突 | 逐字「「作废」（`VOID` / 已作废）在本卷内只指生效前废弃」。这句话把范围写成了「本卷内」，而卷内至少两处作废作用于已生效对象：计划10:193 逐字「`ck_sales_invoices_status` 取值 ISSUED、VOIDED、RED_REVERSED」、计划10:691 逐字「ISSUED 到 VOIDED、ISSUED 到 RED_REVERSED，两者互斥，各自只允许一次」，其上位是规格:311「同一张发票的作废与红字冲销互斥」——销项发票作废恰恰只对**已开具**的发票成立。更直接的是 A-4 自己的连带项与同簇 A-8：A-4 knock_on 承认收款计划期次要「置 `VOIDED`」，A-8 为此加列，而那些期次是已生效合同派生出来的。第一条如果按字面入卷，会在下一轮复核时被拿来反推「发票作废写错了」。范围须收窄为「合同对象上的 VOID」。 |
| 应改 | A-4（词义：作废与终止）criteria | 判据不可判 | 两条判据都标为机检，实际都要人判语义。其一「`docs/error-codes.md` 的 CLM 段不出现同时命中「VOID」且作用对象为已生效及其后状态的码」——错误码登记文件里没有「作用对象处于哪个状态」这一维，grep 判不出后半个条件。其二「`grep` 在阶段 6 与阶段 13 的文案清单中对「作废」的每一处命中，其作用对象逐处核对为草稿或已驳回合同、发票、订单或收款计划期次之一」——「逐处核对」四个字本身就是评审动作。按本卷通则第六条，这两条要么改成可判定替身（例如把受管词表与其允许的对象类型做成一张登记文件再逐字比对），要么如实降为评审并登记入 00b 第 12.1 节 delegated 段。现状是把评审判据写成了机检。 |
| 应改 | A-1（WAIVED 的落库形状）与 A-6（只新增 TERMINATION 一个 chain_kind）、A-10（反向用例） | 连带漏列 | A-1 逐字要求「`WAIVED` 必须同时具备非空 `decision_reason` 与非空 `approval_ref`，由表级 CHECK 强制」，A-10 的反向用例逐字「补上 `approval_ref` 后合同才到达 `TERMINATED`」。但全簇没有任何一处定义豁免一项处置走哪条审批链、审批人是谁、`approval_ref` 从哪个流程实例来：A-6 只把 `chain_kind` 从六个加到七个（新增 `TERMINATION`），那是终止动作本身的链，不是逐项豁免的链。照抄来源 `recon_discrepancies` 的 `approval_ref` 在卷内也有其自己的审批入口。结果是 WAIVED 这条路在实现上无来源、在验收上只能靠测试代码手工塞一个 UUID，而它恰恰是全簇唯一一条「人决定不处置也算闭合」的出口。 |
| 应改 | A-3（三条驱动杠杆）乙 的 criteria | 判据不可判 | 判据逐字「把 `due_at` 拨到过去后 SLA 扫描产出一条「流程时限提醒」」。卷内的 SLA 不是这么触发的：计划03:1059 逐字「SLA：以 `kind = 'SLA'` 的定时器表达，触发时不推进实例，只写 `process_tasks.sla_breached_at` 并产生一条流程时限提醒通知，对应 PRD 第 10.5.2 节的「流程时限提醒」」。也就是说提醒由 `platform_flow.process_timers` 里一条 `kind='SLA'` 的定时器到点触发，改 `process_tasks.due_at` 不会让任何东西发生（`ix_process_tasks_le_state_due_at` 那条索引的括注「SLA 扫描」不是触发机制的定义）。按现判据写出来的用例会恒绿或恒红，取决于夹具怎么造，都判不出 SLA 是否真的接上了。裁定同时要补一句：流程定义 `clm.contract_termination_disposition` 必须为每个人工决策节点登记 SLA 定时器，否则 A-3 乙整条落空。 |
| 应改 | A-3（三条驱动杠杆）乙 关于 max_instance_duration_days 的处理 | 代价被压低 | A-3 把「实例触及 `max_instance_duration_days` 时按计划03:1065 置 `MANUAL_INTERVENTION` 并写 `LIMIT_EXCEEDED` 人工任务」列为「白拿的四样现成能力」之一。它其实是本机制的一个未解冲突而不是收益：影响面处置里三到四类是 `MANUAL_DECISION`（是否登记退货、是否红冲、`ORDERED` 需求怎么办、在制任务收尾还是取消），这些天然可能挂几周到几个月，而计划03:1065 逐字「三项超限即置 `MANUAL_INTERVENTION` 并写 `LIMIT_EXCEEDED` 人工任务」。实例进了 `MANUAL_INTERVENTION` 之后，挂在它下面的 `HUMAN_TASK` 还能不能被认领与完成、`impact_disposition_items` 怎么继续推进、合同能不能最终离开 `TERMINATING`，全簇一个字没写。这条要么给出取值与超限后的处置路径，要么如实登记为不确定，不能记在收益栏。 |
| 应改 | A-7 第 5 条 `CLM_TERM_PURCHASE_REQUISITION` 的 assess 取数范围 | 连带漏列 | assess 逐字只取「`source_idempotency_key` 以 `CONTRACT:{contract_id}:` 为前缀者」。计划07:589-592 的四类来源表逐字列出四种键：`CONTRACT:{contract_id}:{contract_line_id}:{contract_version}`、`SALES_ORDER:{sales_order_line_id}`、`PROJECT_TASK:{project_task_id}`、`STOCK_SHORTAGE:...`。同一张合同派生出的销售订单行上，采购员可经 `actions/raise-from-sales-order-line` 起需求（键为 `SALES_ORDER:`），合同派生的项目任务也可经 `project.project_task.requisition_requested.v1` 起需求（键为 `PROJECT_TASK:`），两支都挂在这张合同底下却都不会被 assess 找到。同簇 A-12 第二条自己承认要阻断的是「四类来源中的合同派生与销售订单两支」——A-12 认为销售订单支属于该合同，A-7 的取数却把它漏掉了。这正是 A-2 自己写的静态声明漏报点在本轮当场发生了一次。 |
| 应改 | A-7 第 1 条 `CLM_TERM_SALES_ORDER_LINE` 的处置面 | 连带漏列 | 只处置订单头与 `sales.sales_order_lines`，不动 `sales.delivery_schedules`。计划06:453 逐字「未拆分的订单行在派生时即建立一条分批交付行……因此系统中不存在没有分批交付行的订单行」，即每条被关闭的订单行底下必然还有 `status = PENDING` 的分批交付行；计划06:271 该表索引 `ix_delivery_schedules_legal_entity_id_promised_date_status` 的括注逐字是「交付指标的期间维度取数」，而规格第 8 章第 14 步逐字要求「交付取合同交付节点与订单分批交付的按期完成率和逾期清单」。不取消这些分批行，已终止合同会一直出现在逾期清单与交付看板里——这与 A-8 第四条为提醒专门堵的那个洞是同一类问题、同一句理由（A-8 逐字「合同都终止了还每月提醒收款到期」），只在收款侧堵了，交付侧漏了。 |
| 应改 | A-7 criteria 三 与 A-1 的 DDL | 判据不可判 | A-7 criteria 三逐字「三条 `MANUAL_DECISION` 规则的项，在 `decision_reason` 为空时直接置 `DONE` 的写入被拒」。A-1 的 `impact_disposition_items` DDL 里唯一与 `decision_reason` 有关的约束是「`WAIVED` 必须同时具备非空 `decision_reason` 与非空 `approval_ref`，由表级 CHECK 强制」，对 `DONE` 没有任何约束，表上也没有任何按 `disposition_kind` 分支的 CHECK（`disposition_kind` 与 `state` 是两个独立的 CHECK 列）。这条判据当前没有被测机制，写进去必然恒真。要么 A-1 同批加一条形如 `disposition_kind <> 'MANUAL_DECISION' or state not in ('DONE') or decision_reason is not null` 的表级 CHECK，要么撤下这条判据。顺带：该 criteria 说「三条 `MANUAL_DECISION` 规则」也数错了——按 A-7 自身规则集，第 3、7 两条恒为 `MANUAL_DECISION`，第 5 条的 `ORDERED` 分支与第 6 条的 `IN_PROGRESS` 分支同样产出 `MANUAL_DECISION` 项，是四条。 |
| 应改 | A-9（事件与登记）第三条「从计划06:612 已预留的九个未命名迁移事件里取两个」 | 代价被压低 | 计划06:612 逐字是「其余九个是合同与销售订单状态机的迁移事件」，A-9 据此断言可以「指名占用两个额度」，且「事件总数 18 一个字不用改，第 1 节与第 9 节的计数也不用动」。但卷内的迁移条数远多于九：计划06:337-351 的合同状态机表就有 13 条边（DRAFT→PENDING_APPROVAL、DRAFT→VOID、PENDING_APPROVAL→PENDING_SIGNATURE / REJECTED / DRAFT、REJECTED→DRAFT / VOID、PENDING_SIGNATURE→EFFECTIVE / REJECTED、EFFECTIVE→IN_PERFORMANCE、EFFECTIVE→EFFECTIVE、IN_PERFORMANCE→COMPLETED、IN_PERFORMANCE→TERMINATED），加上计划06:357 的订单状态机（PENDING_RELEASE→RELEASED、RELEASED→CANCELLED、PARTIALLY_DELIVERED→CLOSED、CHANGE_APPROVAL 进出等）还有七条上下。九个名额本来就是从二十条上下的迁移里选定的子集，不是闲置余量；A-9 既没有举证哪两个名额未被占用，又在同簇 A-5 新增四条迁移边的同时把可用名额从九减到七。这一刀的代价可能是「阶段 6 事件总数 18 要改」，那是要动第 1 节与第 9 节计数的，与本条自称的「最省钱的一处」相反。给不出举证时应按本卷通则登记为不确定。 |
| 应改 | A-8（收款计划期次加 status 与 void_reason）的 tier | 代价被压低 | 标为「实现级补充」，但它做的四件事没有一件停在实现层：一，改 计划06:185 的建表 DDL 与其 CHECK 集；二，改 `ep_contract_clm::ContractPaymentScheduleQuery::schedules` 的返回 DTO——该 trait 按裁定 C-20 是收付款计划行的唯一跨模块出处（计划10:248 逐字「收款计划行的取数唯一经 `ep_contract_clm::ContractPaymentScheduleQuery::schedules(tx, ctx, contract_id)`，按裁定 C-20 该查询由阶段 6 提供」），A-8 自己的 knock_on 也承认这是「跨阶段契约变更」；三，改 计划10 的到款自动核销取数口径；四，改 `clm.v_contract_reminder_sources` 的取数语义，而该视图直接承载规格:288 逐字要求的「按合同有效期、交付节点日期和收付款计划到期日生成提醒」。同簇里比它轻的 A-9（只加两个事件名、规格与 PRD 不动）标的是「计划级新增」，档位排序本身就不自洽。 |
| 应改 | A-1（影响面处置台账）criteria 二 第 ① 条 | 判据不可判 | 逐字「断言 `item_total` 等于七条规则 `assess` 返回条数之和」。按 A-1 第三条生命周期第 2 步，`item_total` 的定义就是「在一个事务内调用全部匹配的已注册 `ImpactRule::assess`，建立批次行与全部处置项，`item_total` 一次算定」——被断言的等式正是被测代码的构造方式，恒真。同款项里真正有判定力的是后半句「且每条规则至少产出一项」，那一条应保留，前半句应撤下或换成对**逐条规则期望条数**的断言（例：三条订单行、两条交付节点、一条采购需求……），否则这一格看起来有断言、实际什么都没判。 |
| 应改 | U-H-07 更正凭证入口（单据类型码 CORR） | 越权 | 00c 裁定 C-26 逐字「全量类型码表如下，任何阶段不得新增未在此表登记的码」，表中阶段 9 只有 OBB、GV、PCR、YEC。本裁定新增 CORR，却没有把 00c C-26 的全量码表列入 touches，等于在一条已生效裁定明写的封闭表外新增一个码。另 C-26 逐字「登记文件归阶段 1，各码归其单据所在阶段」，CORR 的单据落在阶段 9，而 touches 写的是「阶段 3（单据类型码 CORR 登记，裁定 C-26）」，落点错。 |
| 应改 | U-H-07（tier「计划级新增」）与 U-D-02（tier「实现级补充」） | 代价被压低 | 两条的 touches 都自认要动规格。U-H-07 的 touches 逐字「规格第 5.2 章总账功能与期末处理块补一句，明写更正凭证的产生方式与来源引用要求」，却标 tier「计划级新增」；U-D-02 的 decision 逐字「规格第 5.2 章与 PRD 第 6.12 节各新增一段」、touches 逐字「规格第 5.2 章财务规则条目（新增一段，明写冲正的适用与禁用场景）」，却标 tier「实现级补充」。同一簇内 U-H-08 只往规格第 5.2 章补一句排除条款就定为「规格级变更」。三条口径互相不一致，两条动规格的被标成更低档，代价被压低。 |
| 应改 | U-H-08 手工凭证入口（三条封闭性约束「全部可机检」之 (c)） | 判据不可判 | decision 逐字「落地以三条封闭性约束承载，全部可机检」，但 criteria 段只给了机检一（archcheck 断言 INSERT 只在三个方法内）与机检二（configdoc 比对 18 个取值），(c) 没有任何判据。且 (c) 的结论本身不成立：(c) 逐字「post_correction 的每一行必须有来源引用……因此不存在「无业务来源的凭证」这条路径」，而 U-H-07 的守卫二不要求来源行与更正行的科目有任何关系，post_correction 又逐字「不查 JOURNAL_MAP、分录行由入参给定」，正是 PRD:4579 逐字担心的「绕过事件映射」。裁定只登记了「不保证语义正当」，没登记 (c) 这句判断本身失真。 |
| 应改 | U-H-08 knock_on 三与 U-D-02 knock_on 三（「PRD:4498 逐字要求三者一并决策」） | 判据不可判 | PRD:4498 逐字只有「财务负责人，结论需与 U-H-07 更正凭证入口一并决策」，全句未提 U-H-08。PRD:4578 逐字也只说 U-H-07「与 U-D-02 是同一个缺口的两端」。两条裁定却各写「PRD:4498 逐字要求三者一并决策」。三者同批的要求实际来自 00c 庚一表原编号 7 行逐字「U-H-07 更正凭证入口与 U-H-08 手工凭证入口，须与 U-D-02 资金单据冲正一并决策（PRD:4498 逐字要求）——三者是一条决策，不得拆开」，即 00c 自己已经错引一次，本裁定照抄。结论可保留，但依据必须改指 00c 庚一，不得继续写「PRD:4498 逐字」。 |
| 应改 | U-D-09 criteria 的负向五、负向六，以及撤销唯一约束后的 SOURCE_ALREADY_REVERSED | 判据不可判 | criteria 逐字要求「后三条为负向用例须先 RED 后 GREEN 并断言具体错误码」，但负向五只写「断言状态守卫错误码」、负向六只写「断言唯一索引冲突转换后的错误码」，两条都没给码，判不了真假。同时 decision 第 (3) 条撤销 ux_invoice_reversals_legal_entity_id_source_invoice_id，而计划10:859 逐字登记的 INVOICE.INVOICE_REVERSAL.SOURCE_ALREADY_REVERSED 原本正是由该唯一约束触发；裁定既没有重新定义它的触发条件，也没有把它废止，落地后这个码要么恒不触发、要么与 RED_AMOUNT_MISMATCH 语义重叠。 |
| 应改 | 发票行明细裁定 touches「阶段 6（第 4.12 节销售退货前置校验由空实现改为真实调用）」 | 发明了不存在的机制 | 计划06:476 逐字「该判定按裁定 C-16 不进 T0 切片，与阶段 10 的该 trait 按第 11.5 小节同批交付同批验收，本阶段不注入替身，承载该判定的退货登记分支整体落在第三批并在该批次当场成立」——阶段 6 侧根本没有空实现可改。（00c C-16 回写段确实写「阶段 6 先注入空实现、阶段 10 替换」，与计划06:476 本就打架；裁定挑了与阶段计划相反的一处并当成现状，且没把这处冲突登记出来。）真正要动的是「第三批同批交付」的批次口径与阶段 10 侧的实现体。 |
| 应改 | 硬伤一、硬伤三、U-D-09、发票行明细四条裁定的阶段 10 落点（「第 3.4 节索引清单」） | 连带漏列 | 计划10:547 是「#### 3.4 RLS 策略」，计划10:553 才是「#### 3.5 索引」，四条裁定一律把索引写成「第 3.4 节索引清单」。后果不只是节号写错：新增的 invoice.sales_invoice_lines、invoice.invoice_reversal_lines 两张表（以及阶段 9 的两张更正凭证表）需要按计划10:549 逐字「上述 36 张表全部带 legal_entity_id，全部按共享基线第 3.8 节的统一模板生成策略……策略名一律 rls_<table>_le」建策略，而 RLS 一节既没被点名也没被改，「36 张表」这个数没改，裁定 C-05 的 tests/rls_matrix 用例集也没列。硬伤一新增的 finance.v_receivable_open 与 v_payable_open 两个视图同样没进第 3.3 节末尾的「合计 17 个」这个数。 |
| 应改 | U-D-09 第 (5) 条比例回滚公式对 VOID 的取值 | 判据不可判 | decision 逐字「本次回增 = round(issued_ratio * red_net_amount / net_amount, 6)」，而 red_net_amount 按计划10 第 3.1.4 节逐字是「RED_LETTER 时必填」，作废（VOID）行该列为空，公式在 VOID 上无定义、除法取空。第 (6) 条却又断言「VOIDED 与 RED_REVERSED 因 rolled_back_ratio 等于 issued_ratio 自然贡献零」——VOID 路径上 rolled_back_ratio 由哪条式子推到 issued_ratio，裁定没写。累计开票比例校验是 PRD:2430 与规格第 8 章第 7 步的判据落点，这个洞会直接反映成重开时的比例算错。 |
| 应改 | U-H-07 守卫二中的 source_ref_kind = OPENING_BALANCE_LINE 这条来源 | 判据不可判 | 七条集成测试判据没有一条测这条来源（正向 A 测直接费用重分类、正向 B 测顺延，负向 C 到 G 全是拒绝路径），即该分支在本裁定下无判据。语义也未定义：计划09:518 逐字「四个通道的写入一律不生成凭证，期初对应的总账侧由本节的期初余额批次承担」，期初余额落在 ledger.account_period_balances 的 opening_balance_amount 上而不是凭证行；计划09:480 逐字「期初固化：期间关闭的同一事务内……固化一次即不再被推翻」。更正凭证产生的是当期发生额，改不动期初，「引用一条已确认期初余额批次行」到底更正了什么、守卫三的「该行金额」取哪一列，一律未写。要么给判据，要么按本卷通则第六条撤下这条来源。 |
| 应改 | 甲档 决策二 与 决策六其三 | 内部自相矛盾 | 决策二逐字「整条配置包链路（Git 差异审查、自动测试、审批、ECDSA 签名、发布、回退）一字不改」，决策六其三随即把 platform_meta.config_autotest_runs 的 suite「由封闭 8 项扩为 9 项，新增 COMPENSATION_POLICY」，touches 也承认要改 13-clients-lowcode.md:263 的 ck_config_autotest_runs_suite、第 444 行与第 996 行。自动测试正是决策二列举的六段链路之一，「一字不改」当场不成立。这句话是本裁定用来论证代价可控的关键句，留着会让下游按「链路无改动」排期。 |
| 应改 | 甲档 决策六其三（第 9 个 suite COMPENSATION_POLICY 的判据） | 判据不可判 | suite 与 item_kind 的绑定使该 suite 在绝大多数包上恒为 PASSED。13-clients-lowcode.md:444 逐字「8 个 suite 的 outcome 全为 PASSED 或 SKIPPED，且 SKIPPED 仅允许出现在该包不含对应 item_kind 时」。本裁定不新增 item_kind，COMPENSATION_POLICY 挂在 AUTHZ_POLICY 上，而 AUTHZ_POLICY 按 04-identity-authz.md:423 覆盖 access_policies、sod_rules、approval_chains、approval_chain_nodes 四张表（本裁定再加第五张）。任何只改审批链或访问策略、不碰 compensation_policies 的 AUTHZ_POLICY 包——包括本簇第 9 条那 12 行默认链的包——都含该 item_kind，因此按 444 不许 SKIPPED，只能空判 PASSED。也就是说这道门禁在最常见的包上恒真，正撞通则第六条。 |
| 应改 | 乙档 决策一 与 决策二（边界的定义域） | 连带漏列 | 决策一声明不可配面包含「任何产生凭证或写台账与库存流水的迁移」，决策二用 posting_trigger_event_types 与 append_only_registry 两张表当定义域。但 00c 的 B-02 逐字把「原裁定给阶段 10 列的 finance.receivable_entries、finance.payable_entries、finance.advance_receipt_entries、finance.advance_payment_entries、finance.overbilling_entries 五张是带核销金额与状态机的可更新台账……五行一并删除」，append_only_registry 的十四行里根本没有应收、应付、预收、预付、超量开票五张往来台账。凡写这五张台账而不触发 13 行中任一事件的迁移（核销、部分核销、超量开票挂账变更一类）既不在甲判据内也不在乙判据内，按本裁定就是可配面——与决策一自己写的「写台账的迁移一律编译期冻结」正面相反。裁定未登记这个缺口，也未说明补法。 |
| 应改 | 乙档 criteria 四（时限样例的越线判据） | 判据不可判 | 逐字「同一响应体中不出现任何推荐路径或建议改走红冲的字段（断言响应键集合不含此类键）」。「此类键」没有封闭键名清单，机器判不出一个键名算不算「推荐路径」；而且全卷没有任何端点产出过这类字段（PRD:2411 逐字「本系统不判定何时该用哪条」本就是已生效的弃权决定），该断言在任何实现下都为真。这是一条标准的恒真门禁，正撞通则第六条，且裁定还把它写成「全部可机判」的五条之一。 |
| 应改 | 乙档 决策四（复用现成引擎，不造新引擎） | 代价被压低 | 「承载与下发走已有规则表」给不出行号。裁定 evidence 只引了 13-clients-lowcode.md 的 AST 上限与数值语义（实为 525、526 两行）与实现类型（531 行），没有任何一行指出规则 AST 存在哪张表、经哪个 item_kind 下发。13-clients-lowcode.md:242 逐字列出的 15 项 item_kind——CUSTOM_OBJECT、CUSTOM_FIELD、CUSTOM_RELATION、CUSTOM_INDEX、CUSTOM_VIEW、UI_LAYOUT、FLOW_DEFINITION、AUTHZ_ROLE、AUTHZ_POLICY、AUTHZ_FIELD_GRANT、REPORT_DEFINITION、METRIC_DEFINITION、DASHBOARD_DEFINITION、PRINT_TEMPLATE、NOTIFY_RULE——里没有任何一项承载脱离流程定义的单据前置条件规则。按 242 的封闭 CHECK，新增这类内容项必须同批改 ck_config_package_items_item_kind，即甲档 knock_on 一自己已经点破的那处。裁定却写「真正的新增只有一件事」，把新增一类内容项、新增一张（或指认一张）承载表这两项代价漏掉了。 |
| 应改 | 第 9 条 决策二(乙) + criteria 四 + tier「计划级新增」 | 代价被压低 | fail-open 挂 DegradationLedger 这一步没有可用取值，也开不出第二条窗口。00-overview.md:75 逐字把 DegradationKind「唯一定义方定死为阶段 2……终态清单的唯一出处定死为阶段 14」，取值为阶段 2 三项（OFFSITE_SINK_NOT_CONFIGURED、WRITER_NOT_IN_SERVICE、PORT_NOT_IMPLEMENTED）加阶段 14 扩到 18 项，由 kind 的 CHECK 收口；裁定通篇没有指名 fail-open 用哪个 kind，三项里没有一项对得上「审批链缺失」，PORT_NOT_IMPLEMENTED 按同段逐字是「跨模块与平台能力缺位的唯一登记形态」，用它属误用。真要新增一个 kind，就要同批改阶段 2 的建表 CHECK 与阶段 14 的 18 项终态清单，而 touches 只列了阶段 4、6、7、9、10 与 PRD，tier 却记「计划级新增」。另有一处判据不成立：同段逐字「唯一约束 ux_degradation_windows_kind_scope_closed 改为在 kind、subject、scope_legal_entity_id、scope_accounting_period_id 与开窗状态上」，同一 scenario 第二次 fail-open 插不进第二行，决策二(乙)的「每次 fail-open 必须经 DegradationLedger 开一条降级窗口」与 criteria 四的「degradation_windows 增一行」只在第一次成立。 |
| 应改 | 第 9 条 决策一 + 决策四 + knock_on 一 | 连带漏列 | 承载物改指之后，被指向的行不存在也不唯一。04-identity-authz.md:161 逐字给出 approval_chains 的索引为「pk、ux_approval_chains_legal_entity_id_code_version_no」，即唯一键含 version_no，一个裸 code 解析不到唯一行；裁定要求把 06-contract-sales.md:140 的「四个审批链编码是逻辑引用 platform_flow 的流程定义键」改为「逻辑引用 platform_authz.approval_chains.code」，却没说取哪个 version_no，也没说 is_active 为假的版本算不算命中。更直接的是决策四那 12 行只给了 scenario、节点、quorum、timeout_hours、is_active，没有给 code 与 name 两个必填列，且 12 行里没有任何一行对应 06-contract-sales.md:193 的 chain_kind 六值（TERMS、DISCOUNT、PAYMENT、ATTACHMENT、CREDIT、EFFECTIVE）；改指之后 clm.contract_types 的四个 approval_chain_*_code 指向一组不存在的行，合同审批出厂即无链——与上一条的六类高风险无链即拒叠加，EFFECTIVE 一类直接闭死。 |
| 应改 | D-12-4 knock_on 第一段「十八改十九」的同批清单 | 连带漏列 | 该清单自称「逐处点名，一处不得漏（本卷已因计数失配栽过三次）」「合计 10 处、11 个数字」，实测两漏两错一虚：漏 00c:1899 逐字「逐个核过十八个取值：落点未配置、写出进程未投入运行、…」；漏 14:572 逐字「platform_ops.degradation_windows 的 kind 取值已由阶段 2 的 3 个扩展至 18 个」；所称 00c:3084 实为 00c:3115（己-2 行），所称 00c:3165 实为 00c:3196（「十八类 kind 一项不动」）；所称「13-clients-lowcode.md:1065 写的是「不新增任何 DegradationKind 取值，见裁定 F-06」」在该文件不成立——13:1065 是 create index concurrently 的风险表行，该文件内相近的一句在 13:839 且措辞不同。一份用来防计数失配的清单本身就是失配的。 |
| 应改 | D-12-4 criteria（三）演练门禁断言 | 判据不可判 | 断言为「两次演练的 backup_set_id 不同，且其中一次等于该落点上 verified_at 最早的 DAILY_FULL 的 id」。「最早的 DAILY_FULL」是随时间与回收任务变动的量：D-12-1 的回收会把早于锚点的全量销毁，演练之后再判定时，当初那份最早全量可能已被删或已不是最早，同一份演练报告在两个时点会得出不同结论，判据不可复算、不可复现。发布门禁必须能在证据包采集时点稳定判真假，这条做不到。 |
| 应改 | D-12-4 criteria（二）「四态互斥，同一输入不得同时命中两态」 | 判据不可判 | 四个 basis 的定义互相包含：BOOTSTRAP 定义为「部署运行未满 D 天」，ANCHOR_MISSING 定义为「不存在 verified_at ≤ now − D 的有效全量备份」——部署运行未满 D 天时后者必然同时成立；GENERATIONS_SHORT（有效全量代数低于 3）在部署运行未满 3 天时同样与 BOOTSTRAP 同时成立。裁定没有给优先级规则，「四态互斥」这条断言按定义无法满足，用例必然失败。 |
| 应改 | D-12-3 第二段（3）例行回收由 job-worker 按日调度 | 与既有条款冲突 | 14:331 逐字「触发面。只由 ops 专用路径与 ops 专用账号触发，不在 /api/v1/platform 前缀下对外暴露，因此不进入第 5 节端点表。」裁定新增的日调度回收由 job-worker 自动构造 DisposalRequest，既不是 ops 专用路径也不是 ops 专用账号触发，与该句正面冲突；裁定 touches 只写了「新增一张表、一条日调度任务、DisposalRequest 增 origin 字段、前置校验增一个分支、五个用例、退出条件第 17 项补两句」，14:331 一字未列。同处还有第二个未列的连带：14:333 第四项「落点可写性判定为 Writable，否则返回 PLATFORM.OFFSITE_SINK.UNWRITABLE」，落点长时间不可写（spec:1209 归档通道暂停）时例行回收将持续被拒并持续写审计，裁定未说明该噪声如何处置。 |
| 应改 | 全簇 tier 标注（D-13-1、D-12-5、D-13-2、D-13-3） | 代价被压低 | 按盘点自定义的分档：:295 逐字「第二档 规格级变更（动规格加 PRD 加二到四个阶段，以月计）」，:297 逐字「第三档 计划级新增（规格不动或只补条，动一到两个阶段，以周计）」，:299 逐字「第四档 实现级补充（一句话到一条用例，以天计）」。D-13-1 自述动阶段 10、13、14 三个阶段并新增第 22 章第 16 条与一张冻结值表，落在第二档，却标「计划级新增」并自评「处在计划级新增的上沿」。D-12-5 标「实现级补充」，实际动规格第 13.3 章新增一句、第 13.4 章新增一条、第 22 章第 13 条追加半句，外加两张表的 CHECK 约束、门禁断言与同屏用例。D-13-2 标「实现级补充」，实际新增规格 21.22 一整节并扩 21.4 签字对象，另需第三方测评机构、两组各不少于 6 名受试者与法务前置结论。D-13-3 标「实现级补充」，实际连带 PRD 第 11.11 节新增三小节与界面呈现。四条一律低报一档。 |
| 应改 | D-12-2 第四段「不可算，须由附录 A.4 那次认证运行实测后回填」与 criteria（三） | 与既有条款冲突 | 裁定 reasoning 逐字称「规格全卷检索「事务日志生成速率」只在 A.3 出现一次，且逐字说「按 A.4 实测的事务日志生成速率折算」」，据此把 W_day 登记为规格未给取值、并作为 A.4「新增三个必判必记项」之一。实测 A.4 已经在管这件事：spec:1841 逐字「三项的实测周期分布、三项各自的实测写出字节量及其对比、按稳定段实测时长折算的事务日志生成速率、以及该次运行的附件新增字节数一并记入认证报告，该速率是 A.3 连续归档本机保留子项取值依据的唯一来源」。W_day 已有实测项与唯一来源，裁定另设一个同名必判必记项会造出第二套口径；「规格全卷未给取值」这一前提不成立，据以作出的「不可算、待回填」登记也随之要重写。 |
| 应改 | D-12-2 criteria（二）capacity_floor_bytes「容差 0」重算比对 | 与既有条款冲突 | criteria 要求「capacity_floor_bytes 非空且等于按公式与 A.4 认证报告实测值重算的结果（工装重算比对，容差 0）」。公式含 B_full 与 M_attach 两项，二者按客户实际数据量取值；而 spec:1824 逐字「部署前由实施方按客户实际数据量完成容量核算，实际数据量超出本节取值时按同一构成重算容量下限并写入实施方案」——正是 D-12-2 第五段（3）自称照抄的那一句。用 A.4 认证报告的实测值去逐字节比对每一个部署的 capacity_floor_bytes，等于要求所有部署的落点容量下限相同，与该句冲突；凡客户数据量不等于 A.3 基准的部署，门禁项 RG-RETENTION-POLICY-SET 必判不通过。 |
| 应改 | D-12-1 第三段（丁）与第五段守卫（5） | 内部自相矛盾 | （丁）逐字「保留策略对该 kind 只允许清理 backup_sets 行本身，不得触发任何落点侧对象销毁」，即允许对 ATTACHMENT_FULL 行做清理；守卫段却逐字「五条同时成立才允许销毁，任一不成立即拒绝执行并写审计…（5）kind 不为 ATTACHMENT_FULL」，即对该 kind 的任何销毁一律拒绝。两段打架。且即使按（丁）执行，14:335 逐字「到达备份保留期的备份集销毁走 BackupSets，两者与附件正文一样必须把落点上的历史副本在同一次处置内一并覆盖，未一并覆盖的销毁证明不成立」——只清行不覆盖落点的处置产不出成立的销毁证明，（丁）所设想的那条路径在现行计划下不存在。另，第四段「A 存在是 D 天窗口成立的充要条件」也与 D-12-4 把 ARCHIVE_GAP 列为独立 basis 相冲突：锚点在而归档段有空洞时窗口并不成立，A 存在只是必要条件，这句若原样写进规格第 13.4 章即是一条错的定义。 |
| 应改 | D-13-1 criteria（e）的 delegated 登记 与 D-13-2 criteria（二）的拒绝登记 | 内部自相矛盾 | 两者同为「产品负责人签署」的评审判据，处置相反：D-13-1 要求「在技术基线第 12.1 节 delegated 段登记一行，承接方写「产品负责人按录屏与计时原始数据判定」」，D-13-2 则以「登记会污染该段与 archcheck 输出的逐行比对契约」为由明令不登记。理由若成立，对 D-13-1 同样成立。而且 D-13-1 那一行过不了现行形制：00b:791 逐字「delegated 段登记已裁定不由工具执行的判据，属永久登记，每行必须点名承接的替身规则」——「产品负责人按录屏与计时原始数据判定」不是替身规则；00c:1694 逐字「两段与 archcheck 运行期输出逐行相等，多一条或少一条均判违反」意味着阶段 1 的 Rust 架构检查工具须在运行期打印一行关于新会计关账用时的登记，属阶段 1 改动，未列入 touches。 |
| 应改 | D-12-2 reasoning「从 7 天到 14 天，客户落点要多 2.1 TB」 | 代价被压低 | 该句紧接着写「这个价格必须原样交给使用方，不能压低」，但数本身是压低的。按同一条裁定第四段给出的公式与示例代入，D = 14 时约 6877 GB、D = 7 时约 4285 GB，差 2592 GB≈2.6 TB；即使不计 1.15 余量也是 5980 − 3726 = 2254 GB≈2.25 TB。2.1 TB 只数了 7 代全量（7 × 300 GB），把多出来的 7 天事务日志归档、配置包与余量整段丢掉了。交给使用方的价签应为约 2.6 TB。 |
| 应改 | 全簇 evidence 与 knock_on 的行号引用 | 发明了不存在的机制 | 多处「逐字原文」系在不含该文的行上，按纪律第二条「给不出原文的不算数」应逐处更正：D-12-4 两次把第 15.3 章台账「至少覆盖」系于 spec:1256（该行逐字为「- 业务流程、SLA、队列积压和用户体验监控。」），实际在 spec:1258；D-12-1 criteria 与 D-12-4 evidence 把通则第六条第三句系于 00c:1690（该行是第二句「被测输入的交付阶段晚于判据所在阶段的…」），实际在 00c:1691；D-13-2 criteria 把「两段与 archcheck 运行期输出逐行相等」系于 00c:1695，实际在 00c:1694；D-12-3 evidence 把 EP__ 业务参数一句系于盘点:301，实际在盘点:299；D-13-1 reasoning 与 knock_on 两次称「盘点第 297 行把它列为规格级变更」，:297 是第三档计划级新增行，影响面识别与驱动机制在 :295；D-12-2 touches 与 D-13-3 knock_on 把诚实披露八条系于 14:44，实际在 14:45；D-12-4 knock_on 引 13-clients-lowcode.md:1065 的那句在该文件不存在。 |
| 登记即可 | A-12（前置阻断的谓词扩面）criteria | 内部自相矛盾 | 同一段里正文逐字「九条断言，全部可机检」，随后列的是正向五条、反向四条、回归一条，末句逐字「合计十条」。同簇 A-3 丙又逐字引「见裁定 A-12 的九条断言」。九与十两个数字同时入卷，且被另一条裁定按九引用。本卷已因计数与枚举失配自纠三次（A-9 自己在 reasoning 里也点了这一条纪律），这里是第四次的现行犯。 |
| 登记即可 | A-7 第 5 条 knock_on 二 与 A-9 第五条 | 内部自相矛盾 | A-7 knock_on 二要求把 计划07:487 的守卫措辞由「来源单据作废」改为「来源单据作废或来源合同终止」，而该行守卫的后半句逐字是「来源作废由 Outbox 消费者触发并写审计」——保留这句就等于在阶段 7 保留一个由 Outbox 消费者驱动的关闭路径。A-9 第五条恰恰逐字禁止这件事：「阶段 7 与阶段 12 不各建一个终止消费者……同一件事只留一套触发，避免出现「消费者也关一遍、处置项也关一遍」的双写」。改措辞时必须连触发方一起改（改为由 `ImpactRule::dispose` 触发），只加「或来源合同终止」六个字会把 A-9 明令要防的双写原样留在计划里。 |
| 登记即可 | U-H-07 与 U-D-09 引用的三处行号 | 判据不可判 | 给不出原文的三处：其一，U-H-07 守卫四逐字「按计划09:929 的 U-H-03 临时取值」，而计划09:929 是「U-H-01 科目类别枚举」那一行，U-H-03 在计划09:931；其二，硬伤三 evidence 的「计划10:495 逐字」，计划10:495 是空行，该句原文在计划10:494；其三，U-D-09 第 (8) 条逐字「与该表已有的第 686 行写法一致」，计划10:686 是空行，该状态表最后一行是 685，全表没有一行是裁定所指的那种写法。第三处尤其要紧：它是「统一改为按 remaining_ratio 判定」这条指令的唯一参照样板，参照物不存在，指令就无法照办。 |
| 登记即可 | 全簇的行号引用 | 内部自相矛盾 | 五处行号与原文对不上，其中一处指向空行：乙档 evidence 写「13-clients-lowcode.md:526 逐字『规则以 AST 形式存储与下发……』」「:527 逐字『数值一律……』」，实为 525 与 526 两行；第 11 条 knock_on 一要求「A-20 与计划04:433、计划13:511 三处须同批加同一句例外说明」，13-clients-lowcode.md:511 是空行，该句实在 512 行；第 9 条 evidence 写「04-identity-authz.md:402 逐字『边界条件：节点展开后用户集合为空时拒绝保存……』」，实为 403 行；甲档 criteria 四写「照抄计划13:912 第 22 条用例的形状」，第 22 条用例实在 905 行。另有一处不是行号而是内容误引：甲档决策六其二写「同一份纯函数在配置保存期与配置发布期各判一次，形状照抄计划04 第 4.5 节四类规则的两处判法」，04-identity-authz.md 第 4.5 节开篇逐字为「四类规则，全部在配置保存时执行一次、在运行期提交时再执行一次」——两处是保存期与运行期，不是保存期与发布期，甲档因此丢掉了运行期那一道。按本卷纪律第二条，给不出对得上的行号的不算数。 |
