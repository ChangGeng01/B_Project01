# F-50 财务一致性与发票模型收口设计

> **F-57 状态：`CURRENT_SUBJECT_INPUT`。** 本文不是完整的现行平台权威；只有被 F-57 保留的财务不变量与裁定仍属规范性输入。旧 **F57-04/20/25** 只保留为需求所有权桶，实施顺序与门禁只由 [2026-08-24 收敛实施主计划](../plans/2026-08-24-f57-converged-program.md)及其四份依赖有序子计划定义；旧 [F-50 实施计划](../plans/2026-08-21-f50-financial-consistency-implementation.md) 已被替代。当前权威集合还包括 [F-57 总体设计](2026-08-23-f57-governed-automation-fabric-design.md)、[F-57 需求追踪矩阵](../reviews/2026-08-23-f57-requirements-traceability.md)与 [F-57 权威替代登记](../reviews/2026-08-23-f57-authority-supersession-register.md)。F-57 是设计/计划权威，不是产品已实现声明；本地模型实现延期。

> 日期：2026-08-21
> 状态：**F-50 时点历史状态：当时已批准并完成权威回写；当前只能作为 F-57 保留的财务不变量与裁定输入，不构成独立开发入口**
> 适用范围：首版中国大陆单法人账簿内的应收、应付、收付款、退款、发票、红字冲销、作废、账龄与关账勾稽
> 输入：F-49 留下的 9 条未决事项（`00c-gap-ruling.md` 第 7042—7050 行）

## 1. 目标与裁定摘要

本设计经书面复核并完成第 12 节回写后，一次关闭 F-49 的 9 条未决事项，目标是让以下四件事同时成立：

1. 资金真的流出或流入时，银行与现金腿不被虚构、重复或遗漏。
2. 红字冲销、退款、核销释放无论以何种合法顺序发生，最终账务结果相同。
3. 当前账龄与历史期间关账各自只有一个可复算口径，后续事件不改写已关闭期间。
4. 销项、进项、蓝票、红票在号码、头行金额、多税率与部分红冲上使用同一组不变量。

本设计采用“针对性修正”方案：保留现有业务模块边界与销项、进项两张物理头表，修正核销方向模型，补齐发票行与号码登记，并统一上位文档。它取代两种不采用的方案：

- 不采用“只改九处文字”：它不能解决退款后的真实业务出口、历史关账取数和并发号码重复。
- 不采用“重建通用财务事件引擎”：首版尚不需要一张万能单据表或通用事件溯源内核，成本与风险都过高。

九项结论如下：

| F-49 项 | F-50 结论 |
|---|---|
| 1 | 退款或返款是对原核销的 `RELEASE`，不是新的正向核销；不增加第六种资金冲正原因 |
| 2 | 红冲释放总额按有效未核销余额计算，多关系按固定 LIFO 分摊 |
| 3 | 核销候选、上限、账龄、信用与查询统一使用有效未核销余额 |
| 4 | 关账勾稽改为截至期间的追加事件切片，不使用后来变化的当前余额倒推历史 |
| 5 | 销项发票税率只在行上，头表三项金额为行汇总 |
| 6 | 进项发票同样支持多行、多税率及逐行红冲 |
| 7 | 发票法定完整标识经中央号码登记表实现跨业务表并发唯一 |
| 8 | 三类更正入口按错误原因唯一适用；四类凭证生成来源保持 F-48 口径 |
| 9 | 部分红冲、部分退货默认展示并标明状态；金额型更正可见但不可直接作为价格来源 |

## 2. 对既有裁定的继承与替代

| 既有裁定 | F-50 处理 |
|---|---|
| F-10 B-3 的五种资金单据冲正原因 | 原因枚举保留；不得增加“因发票红冲而冲正到款/付款”；“无条件镜像原凭证”的过宽记账语义由第 4.2 节动态拆分整体替代 |
| F-10 B-4 的分次部分红冲 | 分次部分红冲原则保留并扩展为销项、进项逐行累计；其中不分方向的单一原票引用、单次冲销关系或 `reversed_*` 权威缓存等旧物理表达由第 6.4 节替代 |
| F-10 B-5 的 `entry_kind = INVOICE/REVERSAL` | 正向主条目改名为 `ORIGINAL` 并明确包含蓝字发票与 `MIGRATION_OPENING`；冲销仍为 `REVERSAL` |
| F-10 B-6 的 `origin = REFUND` 拒绝分支 | 替代；退款改为 `RELEASE` 后该分支不再存在 |
| F-10/F-46 的 `min(已核销金额, 本次红字价税合计)` | 替代为第 4.3 节公式 |
| F-10 B-7 依赖 `reverses_id` 抵消的聚合 | 替代；四张核销关系表一律按显式 `effect_kind` 聚合，历史期间按第 5.2 节累计 |
| F-10 B-8 的销项多行 | 保留，并与进项行模型统一 |
| F-48 的四类凭证生成来源 | 保留；三类更正入口不得被误写成三类生成来源 |
| F-49 的 9 条未决 | 本设计经书面批准并完成第 12 节回写后全部关闭 |

F-10 详本、F-46、F-48、F-49 仍作为历史决策证据保留；被本设计替代的句子必须显式标注“已被 F-50 替代”，不得继续充当当前实现依据。

## 3. 统一术语与金额方向

### 3.1 核销关系

`finance.receivable_settlement_links`、`finance.payable_settlement_links`、`finance.advance_receipt_settlement_links` 与 `finance.advance_payment_settlement_links` 四张核销关系表统一使用以下方向语义；其中 `funding_origin` 只存在于前两张应收、应付核销关系表：

| 字段 | 取值与含义 |
|---|---|
| `effect_kind` | `APPLY` 增加净已核销金额；`RELEASE` 减少净已核销金额 |
| `funding_origin` | 仅应收、应付核销关系使用：`DIRECT_CASH` 表示到款/付款直接核销；`ADVANCE_AUTO` 表示预收/预付自动核销 |
| `source_doc_type`、`source_doc_id` | 表示本行由哪张到款、付款、退款、红冲、作废或冲正单产生 |
| `root_apply_id` | 指向本组最初的 `APPLY` 行；根行取自身 id |
| `reverses_id` | 指向本次直接反向的上一行，仅用于追溯，不再用于推断金额正负 |
| `settled_amount` | 始终为正数，方向只由 `effect_kind` 决定 |

既有 `origin = MANUAL/AUTO_ADVANCE/REFUND/REVERSAL` 作废。人工是否改过默认核销顺序仍记录在到款或付款单的 `is_manual_settlement_order` 与审计证据中，不借 `funding_origin` 表达。

每个 `root_apply_id` 分组必须满足：

```text
root_net_settled = sum(APPLY.settled_amount) - sum(RELEASE.settled_amount)
0 <= root_net_settled <= root_apply.settled_amount
```

对任一核销关系行，其直接反向子行的金额合计不得超过本行金额。根行的 `reverses_id` 为空；所有派生行必须填写 `reverses_id`，且 `effect_kind` 必须与所指行相反。应收、应付派生行还必须复制根行的 `funding_origin`。`reverses_id` 不得跨法人、跨台账侧、跨正向主条目或预收/预付条目、跨根分组，不得形成环。冲正一张退款单时会产生“反向的反向”，即新增 `APPLY` 并引用退款产生的 `RELEASE`，因此任何实现都不得再用 `reverses_id IS NULL/NOT NULL` 判断正负。

这些约束必须落在数据库，不给实现者“数据库或用例层二选一”的自由：每张关系表建立包含 `legal_entity_id`、所属主条目/预收预付条目 id、`root_apply_id` 与关系行 id 的候选唯一键及复合自引用外键；根行/派生行形态用 NULL-safe CHECK 固定；`DEFERRABLE INITIALLY DEFERRED` 约束触发器在提交前校验父子 effect 相反、同一法人/台账侧/所属条目/根、直接子行累计上限、根净额上下界与无环。应用服务在同一事务末重读断言只是第二道防线，不能替代数据库约束。

### 3.2 应收应付条目

对每个可核销正向主条目定义如下。正向主条目的 `entry_kind = ORIGINAL`，其 `source_doc_type` 可以是销项/进项蓝字发票，也可以是 `MIGRATION_OPENING`；冲销条目的 `entry_kind = REVERSAL`。期初条目没有发票冲销子行，因此其 `C = 0`。

```text
O = original_amount
S = sum(APPLY.settled_amount) - sum(RELEASE.settled_amount)
C = 指向该主条目的全部 entry_kind = REVERSAL 条目的 original_amount 合计
row_open = O - S
effective_open = row_open - C
```

必须始终满足：

```text
0 <= S <= O
row_open = O - S
0 <= effective_open <= row_open
```

冲销类条目只追加、不覆盖，强制 `settled_amount = 0`、`open_amount = original_amount`，且永不成为核销候选。正向主条目的 `settled_amount` 与 `open_amount` 可以作为事务内同步维护的查询投影，但权威事实是核销关系；二者必须与关系聚合结果相等。

“未核销余额”只在逐行守恒语境中指 `row_open`；凡涉及可继续核销、账龄、信用占用、门户、报表、客户 360 或供应商对账的经营语境，一律明确写 `effective_open`。

### 3.3 预收预付条目

预收、预付条目只保存正向创建额，不追加“反向预收/预付条目”。对每条预收或预付定义：

```text
net_consumed = sum(APPLY.settled_amount) - sum(RELEASE.settled_amount)
advance_open = original_amount - net_consumed
0 <= net_consumed <= original_amount
0 <= advance_open <= original_amount
```

条目的 `settled_amount` 与 `open_amount` 只是上述关系聚合的事务内投影，必须分别等于 `net_consumed` 与 `advance_open`。原到款/付款被资金冲正时，按第 4.2.1 节先释放可追溯的 `ADVANCE_AUTO` 核销，再把恢复额与原开放额统一追加 `source_doc_type = CASH_DOC_REVERSAL` 的 `APPLY` 消耗；退款/返款冲正恢复原退款消耗时追加 `RELEASE`，随后按第 4.2.2 节重新自动核销。任何路径都不得原地修改历史条目，也不得追加第二条“反向预收/预付条目”。此段取代 F-10 B-3/B-7 中旧口径。

### 3.4 普通开票时自动核销预收预付

销项发票开具与采购发票登记必须在创建应收/应付 `ORIGINAL` 主条目的同一事务内，按同一法人、往来方、合同及有效收付款计划，只消费可追溯到该合同资金根的开放预收/预付。候选预收/预付条目按 `created_at ASC, id ASC` 固定先入先出；逐条取 `min(remaining_effective_open, advance_open)`，不得人工改序，也不得跨合同或跨往来方借用。每个消费分段同时追加：

- 对预收/预付条目的 `APPLY` 效果行；
- 对本次应收/应付主条目的 `APPLY` 核销根行，`funding_origin = ADVANCE_AUTO`，并保留原预收/预付条目及其原到款/付款资金根引用。

自动核销总额记为 `A`，必须满足 `0 <= A <= min(本次主条目锁后 effective_open, 候选 advance_open 合计)`，且两侧效果行的分段、金额与资金根逐项一一对应。销项发票凭证在原开票基础腿上增加“借预收账款、贷应收账款 `A`”；采购发票凭证在原登记基础腿上增加“借应付账款、贷预付账款 `A`”。两条都使用计量项 `advance_auto_applied_amount`，由 `SALES_INVOICE_ISSUED` 与两个采购发票来源类型按各自固定角色映射；不新增事件、凭证来源类型或第二张凭证。

事务顺序固定为：按第 10 节收集并锁定本次发票、应收/应付主条目、候选预收/预付及其资金根；锁后重算候选与容量；写两侧 `APPLY` 效果；以同一个 `advance_auto_applied_amount = A` 生成本次发票的唯一凭证；重读应收/应付、预收/预付及总账勾稽，差额非零则整体回滚。各效果行与凭证共用本次发票实际 `accounting_period_id`，原预收/预付创建期间不修改。历史切片因此按发生期追加：发票期同时减少应收/应付与预收/预付，不回写原收付款期间。

本节是阶段 10 原“自动核销预收预付的分录腿待财务负责人回写”的唯一结论，开发前置由此关闭。

## 4. 退款、红冲与核销释放

### 4.1 客户退款与供应商返款

客户退款与供应商返款不再生成 `origin = REFUND` 的正向核销行。`finance.refund_source_payment_links` 除既有 `refund_id`、`source_doc_type`、`source_doc_id`、`linked_amount` 外，增加只读汇总投影 `advance_consumed_amount` 与 `settlement_released_amount`；权威事实仍是引用该来源链接的预收预付效果行和应收应付 `RELEASE` 行。每条由退款产生的效果行都必须保存 `refund_source_payment_link_id`，不得只保存整张退款 id。处理顺序固定为：

1. 锁定退款单关联的原到款/付款、开放预收/预付条目及其核销根分组。
2. 逐条来源链接处理：链接金额先消耗仍开放且可追溯到该链接所指原款项的预收/预付。
3. 该链接的剩余金额按第 4.4 节的同一 LIFO 次序，只对该原款项对应的有效核销根分组逐组追加 `RELEASE`；禁止把 A 原款项的链接金额释放到 B 原款项的资金根。
4. 退款凭证记录真实资金腿；核销关系只表达往来余额变化，不另造资金腿。
5. 若退款额大于“可用预收/预付 + 所关联原款项的有效核销”，整笔拒绝。

守恒式逐来源链接成立，并再汇总到整张退款：

```text
link.linked_amount
= link.advance_consumed_amount + link.settlement_released_amount

refund_amount = sum(link.linked_amount)
= advance_consumed + settlement_released
```

`advance_consumed_amount` 与 `settlement_released_amount` 必须由引用该 `refund_source_payment_link_id` 的效果行聚合得到并在事务末重读核对；不得接受客户端直接填汇总值。某一链接自己的可追溯余额不足时整笔退款失败，不能借用另一链接的剩余额度补足。

四种资金事实的分录方向固定为：

| 事实 | 借方 | 贷方 |
|---|---|---|
| 客户退款消耗预收 | 预收账款 | 银行存款/库存现金 |
| 客户退款释放应收核销 | 应收账款 | 银行存款/库存现金 |
| 供应商返款消耗预付 | 银行存款/库存现金 | 预付账款 |
| 供应商返款释放应付核销 | 银行存款/库存现金 | 应付账款 |

供应商返款完全镜像。资金单据冲正仍仅用于“款项实际未收到、金额、往来方、账户或日期登记错误”五类事实错误，不得用作发票红冲的前置业务步骤。

### 4.2 资金单据冲正按当前资金去向拆分

F-10 B-3 的“任何到款、付款、退款冲正都无条件镜像原凭证”只在资金当前去向仍与原登记时相同的情况下成立。后续核销、红冲或新发票会改变去向，因此 F-50 将冲正统一为：银行或现金腿全额反向，往来与预收预付腿按锁后当前可追溯去向拆分；原凭证仍不可修改。

#### 4.2.1 到款与付款冲正

对原款项金额 `R`，计算所有直接来自该款项或经其预收/预付自动核销而形成的当前有效应收/应付核销净额 `S`，以及当前开放且可追溯到该款项的预收/预付 `V`。存在尚未冲正的下游退款/返款时先拒绝；否则必须满足 `R = S + V`，不等即整笔失败并报告追溯差额。

1. 对 `S` 涉及的全部核销根追加 `RELEASE`，每根最多释放当前 `root_net_settled`。
2. `DIRECT_CASH` 根在本路径不新建预收/预付，因为现金正被冲回。
3. `ADVANCE_AUTO` 根先恢复其原预收/预付，再与原本开放的 `V` 一并追加 `source_doc_type = CASH_DOC_REVERSAL` 的 `APPLY` 消耗，最终该原款项不留下开放预收/预付。
4. 客户到款冲正净分录为借应收 `S`、借预收 `V`、贷银行/现金 `R`；供应商付款冲正为借银行/现金 `R`、贷应付 `S`、贷预付 `V`。

例如“票 100 → 收 100 → 红冲 30 → 再冲正原收款”：锁后 `S = 70`、`V = 30`，最终应收 70、预收 0、银行净额 0；不能再释放 100，也不能把应收记成 100。

#### 4.2.2 退款与返款冲正

退款冲正必须按每条 `refund_source_payment_link` 及其资金根独立计算，禁止把不同原到款/付款恢复出的资金合成一个无法追溯的池。对第 `j` 条来源链接，定义原退款中消耗该来源预收/预付的金额为 `Y_j`、释放该来源应收/应付核销的金额为 `X_j`：

1. 在对应预收/预付核销关系上追加 `RELEASE`，先恢复 `Y_j`；效果行保留该来源链接、原款项与原 advance entry。
2. 对每个正向主条目初始化递减容量 `capacity = locked_effective_open`，把该来源链接在此条目上的原 `RELEASE` 按 `settled_at DESC, id DESC` 排序；逐行取 `A_ji = min(该 RELEASE 尚未反向的金额, capacity)`，追加引用原 `RELEASE` 的 `APPLY`，随后令 `capacity = capacity - A_ji`。不得让每行各自读取同一个初始容量，也不得处理其他来源链接的行。
3. 每行剩余 `E_ji = 原 RELEASE 尚未反向的金额 - A_ji` 已无可核销的原票余额：原根为 `DIRECT_CASH` 时新增预收/预付，原根为 `ADVANCE_AUTO` 时恢复原预收/预付；二者都保留本次冲正单、来源链接、原退款释放行、原款项、原 advance entry 与资金根的追溯。令 `A_j = sum(A_ji)`、`E_j = sum(E_ji)`。
4. 完成该来源的 `A_j` 追加并在同一事务内重算 `effective_open` 后，只对该资金根恢复或转入的 `Q_j = Y_j + E_j` 执行既有 `ADVANCE_AUTO` 候选与排序规则，向此时锁后的当前有效应收/应付自动核销 `Z_j`；只把 `V_j = Q_j - Z_j` 留作开放预收/预付。所有新效果行继续保存同一来源链接与资金根；不同 `j` 不得合并 `Q_j`。这些自动核销只写台账效果，不得另发第二张凭证，全部会计效果由第 4.2.3 节的一张净冲正凭证承载。
5. 各来源链接必须分别满足 `X_j = A_j + E_j`、`Q_j = Y_j + E_j = Z_j + V_j`，从而 `Y_j + X_j = A_j + Z_j + V_j`；整单金额取各 `j` 求和。事务终态还必须全部 `effective_open >= 0`、`advance_open >= 0`。

来源链接的业务处理顺序固定为 `source_business_date DESC, source_doc_id DESC, refund_source_payment_link_id DESC`；这只是全部数据库锁已按第 10 节取得后的金额分配顺序，不能改变数据库加锁顺序。

整单令 `Y/X/A/Z/V` 分别为各来源链接同名字母之和。客户退款冲正净分录为借银行/现金 `Y + X`、贷应收 `A + Z`、贷预收 `V`；供应商返款冲正为借应付 `A + Z`、借预付 `V`、贷银行/现金 `Y + X`。

两个顺序反例据此闭合：

- “票 100 → 收 100 → 退 30 → 全红 100 → 再冲正退款 30”：`A = 0`、`E = 30`、`Z = 0`、`V = 30`，最终应收 0、预收 100、银行净额 100。
- “预收 100 → 退 30 → 新开票 100 并自动核销 70 → 再冲正退款 30”：恢复的 `Y = 30` 立即自动核销，`Z = 30`、`V = 0`，最终应收 0、预收 0。

#### 4.2.3 finance 与 ledger 的封闭契约

finance 在取得第 10 节规定的全部锁后，计算唯一命令 `CashReversalPostingSplit { ar_ap_amount, advance_amount }`：到款/付款冲正取 `{S, V}`，退款/返款冲正取 `{A + Z, V}`。HTTP、Excel、插件与人工界面均不得提交这两个金额或科目；只有 finance 用例能构造该类型。

ledger 的 `post_reversal` 只接受原资金凭证、本次资金冲正单和上述封闭拆分，校验两金额非负、合计等于原资金腿、来源类型属于到款/付款/退款，按来源方向选择固定科目角色并原子生成一张冲正凭证。冲正产生的全部核销效果行、预收预付效果行与凭证共用同一次期间解析，记入本次冲正实际 `accounting_period_id`，原行期间不修改。finance 同事务写完台账后重跑受影响的应收/应付、预收/预付与资金勾稽，差额非零则整体回滚。该语义不新增凭证来源类别、不允许自由分录，并明确取代 F-10 B-3 与阶段 9 `post_reversal` 的“无条件镜像原凭证”旧定义。

### 4.3 红冲需要释放的总额

在同一事务、同一锁快照内计算：

```text
L = max(0, current_reversal_gross - effective_open_before)
```

其中 `current_reversal_gross` 是本次红字价税合计；作废只允许未发生任何红冲的销项发票全额作废，因此代入原发票全额。提交前在锁后快照明确计算：

```text
remaining_reversible_gross = O - C
0 < current_reversal_gross <= remaining_reversible_gross
```

这里的“剩余可冲”不是 `effective_open`。由 `effective_open_before = O - S - C` 与上述上限可直接推出 `L <= S`。

该公式取代 `min(S, current_reversal_gross)`。例如原票 100、净已核销 60、有效未核销 40：本次红冲 30 时 `L = 0`；本次红冲 50 时才释放 10。旧公式会错误地产生预收并让已经结清的客户继续出现在应收账龄中。

处理完成后必须满足：

```text
effective_open_after
= effective_open_before + L - current_reversal_gross
= max(effective_open_before - current_reversal_gross, 0)
```

### 4.4 多关系释放次序

需要释放时，只处理仍有净余额的 `APPLY` 根分组，根分组的业务顺序固定为：

```text
root.settled_at DESC, root.id DESC
```

即后进先出，不按资金来源设置额外优先级、不按比例分摊，也不允许人工改序。每组可释放额等于该 `root_apply_id` 下 `APPLY` 合计减 `RELEASE` 合计；逐组取 `min(remaining_L, root_net_settled)`。组内再按仍有可反向余额的 `APPLY` 行之 `settled_at DESC, id DESC` 逐行追加 `RELEASE`，单行上限为该 `APPLY` 金额减去直接引用它的既有 `RELEASE` 合计。最终必须 `sum(take) = L`，否则整事务失败。

为避免死锁，数据库行锁严格遵循第 10 节的跨对象类别顺序，并在每一类别内按 `id ASC` 取得；全部锁定后才在内存中按上述 LIFO 顺序计算。

- `DIRECT_CASH` 被红冲释放：每个释放分段分别新增一条预收/预付条目，继承根行的原 `receipt_id/payment_id`，并保存 `source_settlement_root_id`；不得把不同资金根合并成一条失去来源的挂账。
- `ADVANCE_AUTO` 被红冲释放：恢复原预收/预付条目的可用余额。
- 退款或返款触发的 `RELEASE`：不产生上述两项，因为退款事件本身已经记录真实资金流出/流入。

红冲或作废凭证必须把 `released_settlement_amount = L` 作为独立计量项写入既有业务事件映射：销项基础腿贷应收 `current_reversal_gross`，释放附加腿借应收、贷预收 `L`；进项基础腿借应付 `current_reversal_gross`，释放附加腿借预付、贷应付 `L`。因此应收/应付净减少额均为 `current_reversal_gross - L`，银行存款与库存现金在本路径恒为零。台账释放与这两条附加分录必须同事务、同金额，不能只改一边。

本路径新增的冲销条目、核销 `RELEASE`、预收/预付创建或恢复效果行与凭证共用同一次期间解析并记入红冲/作废实际期间，所有原始条目的期间保持不变。

### 4.5 顺序无关性

同一业务事实的合法操作顺序必须得到同一终态。例如原票 100、收款 100、最终退款 30 且全额红冲：

- 先退款后红冲：退款先释放 30，红冲再释放 70，最终应收 0、预收 70。
- 先红冲后退款：红冲先形成预收 100，退款消耗 30，最终应收 0、预收 70。

两条路径银行资金净额相同，且不通过虚构资金冲正得到结果。供应商方向同构。

## 5. 当前余额、账龄与历史期间勾稽

### 5.1 当前经营视图

当前应收、应付查询与账龄以 `effective_open` 为唯一基数。子账当前余额为全部 `ORIGINAL` 正向主条目（蓝字发票与 `MIGRATION_OPENING`）的 `effective_open` 之和；冲销类条目不单独进入账龄。

当前预收、预付子账余额为全部业务挂账与 `MIGRATION_OPENING` 条目的 `advance_open` 之和，不读取后来被原地改写的历史创建额。

核销候选只包含：

- 条目类型为 `ORIGINAL`，来源是蓝字发票或 `MIGRATION_OPENING`；
- 锁后 `effective_open > 0`；
- 本次分配 `0 < allocation <= locked_effective_open`。

上述口径必须同步到到款、付款、信用占用、付款申请上限、客户 360、供应商门户、经营报表与 Excel 导出，不得保留第二套“当前未核销余额”。

### 5.2 历史期间切片

关账与历史报表不得读取今天的 `open_amount` 再按原始期间过滤。对期间 `P`，应收/应付子账余额按截至 `P` 的追加事件重算：

```text
AR/AP_subledger(P)
= sum(positive_main_entry.original_amount through P)
- sum(reversal_original_amount through P)
- sum(APPLY.settled_amount through P)
+ sum(RELEASE.settled_amount through P)
```

其中正向主条目是 `entry_kind = ORIGINAL` 的业务发票或 `MIGRATION_OPENING` 期初条目，冲销条目是 `entry_kind = REVERSAL` 的追加行。预收/预付余额分别按下式重算，`advance_entry` 包含业务挂账与 `MIGRATION_OPENING`：

```text
advance_subledger(P)
= sum(advance_entry.original_amount through P)
- sum(advance_settlement.APPLY.settled_amount through P)
+ sum(advance_settlement.RELEASE.settled_amount through P)
```

总账侧取同一法人、同一期间的期末余额，差额必须为零。

期间先后只经同一法人会计期间表的 `period_seq` 或起止日期判断，禁止比较 UUID 大小。必须满足：

1. 最新期间累计值等于当前经营视图合计。
2. `P` 之后发生的收款、退款、红冲或冲正不改变 `P` 的历史结果。
3. 顺延事件按实际 `accounting_period_id` 进入切片，`business_date` 只用于业务日期检索与账龄。

第 17.3 章的逐行守恒与子账总账勾稽必须分别写明“逐行原始余额”“当前有效余额”“截至期间余额”，不得再用一句“未核销余额”同时承担三种语义。

## 6. 销项与进项发票头行模型

### 6.1 物理边界

保留 `invoice.sales_invoices` 与 `invoice.purchase_invoices` 两张业务头表，不合并为万能发票表。Rust 侧共用 `InvoiceAmounts`、`InvoiceLineAmounts`、税额校验、头行汇总和号码登记组件。

本节新增或改造的发票头、发票行、冲销头、冲销行与号码登记表全部带 `legal_entity_id` 及基线公共列，启用法人 RLS；表间引用使用 `(legal_entity_id, id)` 复合外键，禁止跨法人挂接。写权限只授予发票应用服务角色，交互用户不得直接写表；读权限仍叠加单据权限、记录级范围与字段级权限。

两侧统一规则：

- 每张发票至少一行。
- 税率只在行上，头表没有单一 `tax_rate`。
- 头表保存 `net_amount`、`tax_amount`、`gross_amount` 三个只读汇总值。
- 蓝字销项、进项原票行 `net_amount > 0`、`tax_amount >= 0`、`gross_amount > 0`；冲销行改按第 6.4 节的组合约束，允许纯税额结构性特例。
- 行 `gross_amount = net_amount + tax_amount`。
- 每行税额与金额先经过下述唯一 F-03/U-D-05 税额校验与舍入策略；除第 6.4 节明列的纯税额结构性特例外，不存在第二套模式或容差。行值确定后，头表按定标值直接求和，不二次舍入。
- 写请求与 Excel 模板只包含行级金额；服务端从行计算头表金额并在响应中返回。若旧客户端仍提交头表三项金额，按契约或模板版本不匹配拒绝，不得静默忽略。

登记服务在同一事务中写头与行并重读断言三项合计。数据库对每行实施金额 CHECK；跨头行合计由受控写入口、事务末校验和对账自检共同保证，不声称普通 CHECK 可以跨表求和。

F-03/U-D-05 在本卷同批关闭，冻结为：所有人民币金额用十进制定点 `numeric(18,2)`/Rust `Decimal` 表达，税率用 `numeric(9,6)`，禁止二进制浮点参与业务计算；普通行的期望税额为 `round_half_up(net_amount × tax_rate, 2)`，恰逢半分时向绝对值增大的方向舍入。调用方提交的税额与期望税额之差绝对值不得超过 `EP__INVOICE__TAX__AMOUNT_TOLERANCE`，该配置为 `numeric(18,2)`、默认 `0.02`、允许区间 `0.00..=0.02`、启动时读取且变更需重启；超出返回精确税额容差错误。`gross_amount` 无容差，必须逐分等于 `net_amount + tax_amount`。红字金额仍保存正数并由冲销方向表达符号，因此使用同一算法；第 6.4 节的纯税额更正只豁免 `net × rate` 比较，不豁免两位小数、剩余税额上限与价税合计等式。

### 6.2 销项发票

新增 `invoice.sales_invoice_lines`，除公共列外的业务列固定为：`sales_invoice_id`、`line_no`、`sales_order_id`、`sales_order_line_id`、`item_kind`、`item_id`、`uom_code`、`quantity`、`net_unit_price`、`tax_rate`、`net_amount`、`tax_amount`、`gross_amount`。发票头的开票内容可保留作整体摘要，行级商品或服务内容以行表为准。

### 6.3 进项发票

现有 `invoice.purchase_invoice_lines` 增加 `tax_rate` 与 `gross_amount`；`purchase_invoices` 删除头表 `tax_rate`。存货类与直接费用类仍按既有 `cost_kind` 分支，不因多税率改变成本归集边界。

一张进项发票的全部行必须使用同一个 `cost_kind`；服务端从行推导并固化头值，混合 `INVENTORY_TYPE`/`DIRECT_EXPENSE_TYPE` 的请求或门户受理整单拒绝。该约束使一次登记唯一派生 `PURCHASE_INVOICE_INVENTORY` 或 `PURCHASE_INVOICE_DIRECT_EXPENSE`，调用方不得逐行选择凭证来源。进项行另固定四个 `NOT NULL DEFAULT 0` 的服务端结果字段：非负 `accrual_reversal_amount`、有符号 `price_variance_in_stock_amount`、有符号 `price_variance_released_amount`、非负 `overbilling_amount`；不再保留未拆分的总价差列或超量布尔标志。直接费用行四项全为零，物料行按 GRNI、库存价差和超量匹配结果写入。

进项、销项都允许多行、多税率。0% 税率是现有出厂税率档，税额必须允许为零。

### 6.4 作废与红字冲销

冲销头 `invoice.invoice_reversals` 不再使用无法同时建立两套真实外键的不分方向原票单列，改为 `source_sales_invoice_id` 与 `source_purchase_invoice_id` 两列；`direction = OUTPUT` 时前者非空、后者为空，`direction = INPUT` 时相反，由 XOR CHECK 与两套 `(legal_entity_id, source_*_invoice_id)` 复合外键共同保证。

冲销头另增加 `linked_purchase_return_id uuid NULL`。数据库以 NULL-safe CHECK 保证该字段非空时 `direction = INPUT AND reversal_kind = RED_LETTER` 必须为真；全部销项、`VOID` 与独立进项更正必须为空，由采购退货用例触发的进项红字（含物料、直接费用与直运）必须非空。字段是对 procure owner 的跨 schema 逻辑引用，不建立伪物理 FK，也不假定一张采购退货只能对应一张供应商红字。invoice owner 必须在同一写事务、统一锁序内经 procure owner 端口验证退货同法人、同供应商；物料类还要保证本次红字行恰好覆盖该退货的已开票分段，直接费用/直运类保证原成本归集链与累计可冲上限一致；不满足整笔回滚。

新增统一的 `invoice.invoice_reversal_lines`。除 `invoice_reversal_id` 外，它冗余保存 `source_sales_invoice_id`、`source_purchase_invoice_id`，并分别配 `source_sales_invoice_line_id`、`source_purchase_invoice_line_id`；按冲销方向两组恰有一组同时非空。销项与进项原行表分别建立 `(legal_entity_id, sales_invoice_id, id)`、`(legal_entity_id, purchase_invoice_id, id)` 候选唯一键，冲销行再以对应三列默认 `MATCH SIMPLE` 的复合外键真实指向原行。这里不得使用 `MATCH FULL`：共享的 `legal_entity_id` 永远非空，非活动方向会形成 `(非空, NULL, NULL)`，`MATCH FULL` 会把它误判为部分 NULL 而拒绝所有行。活动组全填、非活动组全空由显式 NULL-safe XOR/all-or-none CHECK 强制；延迟约束触发器再保证行上的原票 id 与 `invoice_reversals` 头所指原票逐字相同，从数据库层拒绝同法人错票挂行。

每个冲销行记录 `source_effect_seq`、`quantity_effect_kind = REDUCE | NONE`、`pricing_effect_kind = ORIGINAL_UNIT_PRICE | ADJUSTED`、本次冲销数量、`tax_rate`、不含税金额、税额与价税合计，税率必须等于所指原行税率。`invoice_reversal_id`、`source_effect_seq`、两个 effect kind、数量、税率及三项金额全部 `NOT NULL`；方向、来源 id 组、effect kind、数量与金额的组合 CHECK 必须逐项显式判断 NULL，不能让 SQL `UNKNOWN` 绕过约束。`source_effect_seq` 在每条销项或进项来源行内从 1 开始、无缺口严格递增，分别以 `(legal_entity_id,source_sales_invoice_line_id,source_effect_seq)` 与 `(legal_entity_id,source_purchase_invoice_line_id,source_effect_seq)` 唯一约束承载；非活动方向的 line id 为空，活动方向必被其中一个约束覆盖。invoice owner 必须先按第 10 节统一锁序锁来源行，再分配下一序号：

- `REDUCE + ORIGINAL_UNIT_PRICE`：数量大于零，非末次金额按原行 `net_unit_price` 与项目唯一税额策略计算。只有该来源行此前从未出现 `pricing_effect_kind = ADJUSTED`，且本次按同一策略计算后恰好耗尽剩余数量时，才允许三项金额取“原行定标值减此前全部 `ORIGINAL_UNIT_PRICE` 冲销定标值”的确定性剩余额以吸收末次舍入尾差；该值必须等于服务端按唯一策略重算的预期尾差，不接受调用方自报。
- `REDUCE + ADJUSTED`：数量大于零，但金额还包含折让或价格更正；数量与金额分别占用各自剩余额度。
- `NONE + ADJUSTED`：数量为零，用于纯折让、金额或税额更正。
- `NONE + ORIGINAL_UNIT_PRICE`：无业务意义，CHECK 直接拒绝。

跨行价量分类不是只靠服务端约定。`DEFERRABLE INITIALLY DEFERRED` 约束触发器在提交前锁定对应来源行，并按 `source_effect_seq` 重放该来源行全部冲销效果：序号必须恰为 `1..n`，四项累计均不得超过原行，每条非尾次 `ORIGINAL_UNIT_PRICE` 必须等于按原单价与唯一税额策略重算的标准金额。只有某行恰好耗尽剩余数量、此前序号没有任何 `ADJUSTED`、且三项金额分别等于原行定标值减此前全部 `ORIGINAL_UNIT_PRICE` 冲销定标值时，才允许它偏离标准金额吸收末次尾差；已有 `ADJUSTED` 后伪装尾差、重复/跳号或非末次偏差均在数据库层拒绝。应用服务锁后执行同一重放并映射业务错误，数据库触发器是不可绕过的最终约束。

所有组合均要求不含税金额与税额大于等于零且至少一项大于零、价税合计大于零且等于前两项之和，因此纯税额更正可表达。普通红字行继续执行 F-03/U-D-05 的 `net × rate` 税额校验；仅 `NONE + ADJUSTED` 且 `net_amount = 0, tax_amount > 0` 的纯税额更正为结构性特例：税率仍复制原行、金额仍按项目唯一小数位与舍入规则定标、税额不得超过该原行的 `remaining_tax`，但不执行以零净额乘税率为基准的容差式。F-03/U-D-05 的回写必须显式登记这项特例，不能让通用容差校验把它误拒，也不能借此放宽其他行。冲销头三项金额由冲销行汇总，删除头表 `red_tax_rate`。

- 销项作废：系统按原销项发票全部行生成全额内部冲销行；仍只允许全额一次。
- 销项红票、进项红票：登记实际红字行，允许分次部分冲销。
- 每一原行分别计算 `remaining_quantity`、`remaining_net`、`remaining_tax`、`remaining_gross`，等于原值减已登记冲销行对应合计，四者都不得为负。
- 销项状态固定为 `ISSUED`、`PARTIALLY_RED_REVERSED`、`VOIDED`、`RED_REVERSED`；进项状态固定为 `REGISTERED`、`PARTIALLY_REVERSED`、`REVERSED`。除 `VOID` 外，全部原行三项剩余金额均为零时才进入全额红冲终态；已有任一冲销行但尚未全部归零时为部分红冲。状态在锁后推导并推进，终态不可回退。
- 红冲累计的权威来源只有已登记的冲销行聚合；不在蓝票头或蓝票行另存 `reversed_*` 权威列，既有规划中的同名缓存列全部撤销。
- 删除 `purchase_invoices.is_credit_note`，进项红票只走统一冲销入口。
- 删除不能表达多次部分红冲的 `purchase_invoices.reversed_by_id` 与“每张原票只能有一张冲销单”的唯一约束。
- 进项引用全在 `invoice` schema 内，不再错误指向 `procure`。

进项红字与实物退货的分工固定为成对模型。只有物料类 `quantity_effect_kind = REDUCE` 的进项红字按原 `PURCHASE_INVOICE/DECREASE` 父效果追加 `PURCHASE_CREDIT_NOTE/INCREASE`，重开数量与 GRNI 金额由服务端按原父效果计算；`NONE + ADJUSTED` 不写 GRNI，`REDUCE + ADJUSTED` 也只按减少数量重开原暂估金额。原票为物料类且 `linked_purchase_return_id IS NOT NULL` 时，红字来源固定为 `PURCHASE_INVOICE_LINKED_RETURN_REVERSED` 且不得含库存腿，计量项使用 `gross_amount,input_tax_amount,grni_reopened_amount,linked_return_price_difference_amount,released_settlement_amount?`，其中 `linked_return_price_difference_amount = red_net_amount - grni_reopened_amount` 且 `red_net_amount` 是服务端控制总额、不是 MeasureKey。采购模块随后在同一事务逐条等量等额追加 `PURCHASE_RETURN/DECREASE`，并只以 `PURCHASE_RETURN_INVENTORY` 生成一张物理凭证：按原暂估借 GRNI、按锁后当前账面价值贷库存，账面差额经 `return_carrying_difference_amount` 进主营业务成本；部分退货按移动平均价，全数退清取退货前库存金额余额全额并使库存数量、金额与单价同时归零。两张凭证合计 GRNI 为零，库存只减少一次；未开票段直接消费原收货 GRNI，混合段仍只生成这一张物理凭证。原票为直接费用/直运类时来源固定为 `PURCHASE_INVOICE_DIRECT_EXPENSE_REVERSED`，红字只冲应付、进项税与原成本，不写 GRNI 或本方库存，也不生成 `PURCHASE_RETURN_INVENTORY`。

发票状态是票面金额/税额终态，不是库存或服务数量终态：某来源行 `remaining_net`、`remaining_tax`、`remaining_gross` 均为零后，该行已经没有可供成交取价的票面金额；即使一次 `NONE + ADJUSTED` 金额更正使审计字段 `remaining_quantity > 0`，原票在所有来源行三项金额归零后仍进入 `RED_REVERSED/REVERSED`。此时剩余数量只作原业务事实的审计展示，不代表仍有可引用成交价格；第 9 节必须返回 `false/false`。

### 6.5 HTTP、插件与 Excel 唯一写契约

四类入口共用以下封闭输入类型；字段名就是 OpenAPI、Rust contract crate、插件 schema 与 Excel 归一化后的名字，不允许各入口另造头金额或头税率版本：

```text
InvoiceIdentifierInput {
  invoice_medium,
  number_scheme,
  invoice_code?,
  invoice_no
}

SalesInvoiceLineInput {
  line_no,
  sales_order_id,
  sales_order_line_id,
  item_kind,
  item_id,
  uom_code,
  quantity,
  net_unit_price,
  tax_rate,
  net_amount,
  tax_amount,
  gross_amount
}

PurchaseInvoiceLineInput {
  line_no,
  purchase_order_id,
  purchase_order_line_id,
  goods_receipt_id?,
  goods_receipt_line_id?,
  cost_kind,
  item_id?,
  quantity,
  net_unit_price,
  tax_rate,
  net_amount,
  tax_amount,
  gross_amount
}

InvoiceReversalLineInput {
  source_sales_invoice_line_id?,
  source_purchase_invoice_line_id?,
  quantity_effect_kind,
  pricing_effect_kind,
  quantity,
  tax_rate,
  net_amount,
  tax_amount,
  gross_amount
}
```

阶段 7 与阶段 10 的内部采购红字端口只使用下面这一套 Rust 契约；它取代 `00c-gap-ruling.md` A-11 中旧的必填 `purchase_return_id`、旧 `PurchaseCreditNoteLine` 与旧 `PurchaseCreditNoteView`，旧块不得复制实现：

```rust
pub struct RegisterPurchaseCreditNote {
    pub supplier_id: Id<Supplier>,
    pub original_purchase_invoice_id: Id<PurchaseInvoice>,
    pub linked_purchase_return_id: Option<uuid::Uuid>,
    pub identifier: InvoiceIdentifierInput,
    pub posting_date: chrono::NaiveDate,
    pub expected_original_row_version: i64,
    pub is_for_overbilling_settlement: bool,
    pub lines: Vec<InvoiceReversalLineInput>,
}

pub struct PurchaseCreditNoteGrniReopen {
    pub invoice_reversal_line_id: Id<InvoiceReversalLine>,
    pub original_purchase_invoice_line_id: Id<PurchaseInvoiceLine>,
    pub goods_receipt_line_id: Id<GoodsReceiptLine>,
    pub grni_effect_id: uuid::Uuid,
    pub quantity: Quantity,
    pub amount: Money,
}

pub struct PurchaseCreditNoteView {
    pub invoice_reversal_id: Id<InvoiceReversal>,
    pub doc_no: String,
    pub net_amount: Money,
    pub tax_amount: Money,
    pub gross_amount: Money,
    pub linked_purchase_return_id: Option<uuid::Uuid>,
    pub grni_reopened_effects: Vec<PurchaseCreditNoteGrniReopen>,
    pub grni_reopened_amount: Money,
    pub voucher_id: uuid::Uuid,
    pub accounting_period_id: Id<AccountingPeriod>,
    pub accounting_period_seq: i32,
    pub deferred_from_period_id: Option<Id<AccountingPeriod>>,
}

#[async_trait::async_trait]
pub trait PurchaseCreditNotePort: Send + Sync {
    async fn register_credit_note(
        &self,
        tx: &mut dyn Tx,
        ctx: &SecurityContext,
        cmd: RegisterPurchaseCreditNote,
    ) -> Result<PurchaseCreditNoteView, AppError>;
}
```

`lines` 必须非空；本端口逐行要求 `source_purchase_invoice_line_id` 非空且 `source_sales_invoice_line_id` 为空。采购退货调用时 `linked_purchase_return_id` 必填并由 invoice owner 校验同法人、同供应商、同原票与当前退货；独立进项更正时必须为空。`expected_original_row_version` 是锁后比较的原进项发票并发版本，失配使用既有状态冲突错误码。阶段 7 已经解析期间时，本端口仍调用同一事务内记忆化的 `AccountingPeriodResolver`；返回的三项期间值必须与调用方先前结果逐值相等，不把 `ResolvedPeriod` 作为第二套可伪造命令字段。`voucher_id` 非空；无 GRNI 重开时集合为空且汇总为零，有重开时汇总必须等于各项 `amount` 之和。采购退货用例须收集同一 `linked_purchase_return_id` 下本次全部 `PurchaseCreditNoteView.voucher_id`，去重并按 UUID 升序写入 `procure.purchase_return.posted.v1.purchase_credit_note_voucher_ids[]`；物料物理退货凭证另写 `physical_return_voucher_id?`，不得压成一个无法表达混合分支的 voucher 字段。

`source_effect_seq` 是服务端在锁定来源行后分配的持久化顺序，不属于 `InvoiceReversalLineInput`，HTTP、插件、Excel 与内部采购退货命令均不得提交；响应可把它作为只读审计字段返回。

销项、进项蓝票登记头只携带业务头字段、`InvoiceIdentifierInput` 与非空 `lines[]`；公开 HTTP、插件与 Excel 的红字登记头只携带 `direction`、对应原票 id、`InvoiceIdentifierInput` 与非空 `lines[]`，不得接受 `linked_purchase_return_id`，因此这些入口只能登记独立更正。只有阶段 7 采购退货用例调用的内部 `PurchaseCreditNotePort::register_credit_note` 命令携带 `linked_purchase_return_id`，且物料、直接费用与直运三类调用均必填；该内部端口与采购退货共享事务，不能由路由直接暴露。作废命令只携带原销项票 id、作废原因、记账日期与并发版本，不接受号码或行金额，行由服务端按原票剩余额生成。所有公开请求头都不得出现 `tax_rate`、`net_amount/untaxed_amount`、`tax_amount`、`gross_amount`，进项输入还不得出现暂估、价差、超量或自动核销结果字段；响应必须返回服务端汇总，销项/进项均返回 `advance_auto_applied_amount`，进项另逐行及头级返回 `accrual_reversal_amount`、`price_variance_in_stock_amount`、`price_variance_released_amount`、`overbilling_amount`，冲销行返回只读 `source_effect_seq`，冲销头可返回只读链接 id。红字每行两种来源 line id 恰有一个非空且必须匹配头方向。

Excel 模板版本固定为 `sales-invoice-register-v2`、`purchase-invoice-register-v2` 与 `invoice-reversal-register-v2`。每行包含 `document_key`、`line_no`、该模板的头字段、号码字段与对应行字段；同一 `document_key` 的重复头字段必须逐字一致，行号必须从 1 起且组内唯一。旧模板版本或出现头金额/头税率列时整份文件按模板版本不支持拒绝；不能静默忽略旧列。解析器先归一成上述 contract 输入，再调用与 HTTP/插件相同的验证器与用例。

### 6.6 供应商门户上传与受理回写

`portal.supplier_invoice_uploads` 是待受理业务头，不是法定号码登记。删除上传头的单一 `tax_rate`，保留由行汇总出的只读 `net_amount`、`tax_amount`、`gross_amount`，并新增 `invoice_medium`、`number_scheme`、`invoice_code`、`invoice_no`。新增 `portal.supplier_invoice_upload_lines`，字段固定为 `supplier_invoice_upload_id`、`line_no`、`purchase_order_id`、`purchase_order_line_id`、`goods_receipt_id?`、`goods_receipt_line_id?`、`cost_kind`、`item_id?`、`quantity`、`net_unit_price`、`tax_rate`、`net_amount`、`tax_amount`、`gross_amount`；金额、税率、至少一行、头行汇总与 RLS 规则同第 6.1 节。

上传阶段只按同一供应商、同一法人、同一上传号码做防重复，不占用 `invoice.invoice_number_registry`，因为尚未形成正式进项发票。物理唯一性不使用基线禁止的部分索引：上传头增加数据库生成的 `identifier_key NOT NULL` 与 `active_identifier_slot`，后者在 `UPLOADED|ACCEPTED` 时等于前者、`RETURNED` 时为 NULL，并建立普通 `UNIQUE(legal_entity_id,supplier_id,active_identifier_slot)`；PostgreSQL 16 默认 `NULLS DISTINCT` 允许保留多张退回历史。状态形状 CHECK 强制 `UPLOADED` 无退回原因/正式票引用、`RETURNED` 只有非空白退回原因、`ACCEPTED` 只有正式进项票引用。`UPLOADED -> ACCEPTED` 时 invoice 用例在统一事务内重读上传头行、执行第 6.1/7 节全部验证、登记中央号码、创建进项发票头行，然后经下述端口原子回写。若号码此时已被任何正式蓝票或红票占用，受理失败且上传保持 `UPLOADED`。`UPLOADED -> RETURNED` 不占号码；退回后重新上传是新单据，不覆盖旧记录。

阶段 7 与阶段 10 之间的端口名与签名冻结为：

```rust
#[async_trait::async_trait]
pub trait SupplierInvoiceUploadWritebackPort: Send + Sync {
    async fn accept(
        &self,
        tx: &mut dyn Tx,
        ctx: &SecurityContext,
        upload_id: SupplierInvoiceUploadId,
        purchase_invoice_id: PurchaseInvoiceId,
    ) -> Result<(), AppError>;

    async fn return_upload(
        &self,
        tx: &mut dyn Tx,
        ctx: &SecurityContext,
        upload_id: SupplierInvoiceUploadId,
        reason: NonEmptyText,
        expected_row_version: i64,
    ) -> Result<(), AppError>;
}
```

端口由 `ep-app-portal` 实现，`ep-app-invoice` 在受理用例中调用；实现必须验证同法人、同供应商、状态仍为 `UPLOADED`，并分别推进 `ACCEPTED`/`RETURNED`。内部退回入口固定为 `POST /api/v1/portal/supplier-invoice-uploads/{id}/actions/return`，请求只含 `reason` 与 `row_version`，handler 必须把 `row_version` 原样映射到 `expected_row_version`；锁后状态或版本不等统一返回 `PORTAL.SUPPLIER_INVOICE_UPLOAD.STATE_CHANGED`，不得丢弃、覆盖或在 owner port 外另做一遍竞争性判断。供应商门户自身无接受权限。正式进项发票登记请求可携带一个 `supplier_invoice_upload_id`，携带时头行内容必须完全取自并匹配该上传，不接受客户端同时提交另一套行数据。

## 7. 发票号码与代码

### 7.1 编号制式与票面媒介分离

首版支持两种编号制式：

| `number_scheme` | 发票代码 | 发票号码 |
|---|---|---|
| `UNIFIED_20` | 必须为空 | 恰为 20 位 ASCII 数字 |
| `LEGACY_CODE_NUMBER` | 必填，10 或 12 位 ASCII 数字 | 恰为 8 位 ASCII 数字 |

另设 `invoice_medium = ELECTRONIC | PAPER`，不得再用电子/纸质直接推断代码是否必填。数电票采用 20 位号码并取消发票代码；旧制票使用代码加号码。该建模也兼容电子发票服务平台开具的纸质票使用新编号制式。

销项蓝票、进项蓝票以及 `RED_LETTER` 类型的销项红票、进项红票都必须引用一条 `invoice_number_registry_id`；`number_scheme`、`invoice_medium`、`invoice_code` 与 `invoice_no` 只在号码登记表保存一次，由受权视图与 DTO 关联返回，业务头表不复制第二份。`VOID` 是对原销项票的内部作废事件，不产生另一张法定发票，因此登记引用必须为空，也不占用新的号码。

外部依据：

- 国家税务总局辽宁省税务局《全面数字化的电子发票常见问题及解答》：数电票号码为 20 位并删除发票代码，且可按条件全额或部分开具红字数电票。
  <https://liaoning.chinatax.gov.cn/art/2023/4/25/art_99_102478.html>
- 国家税务总局《一文了解：纳税人怎样开具红字数电发票？》：全国推广后的红字数电发票可按规则全额或部分开具。
  <https://www.chinatax.gov.cn/chinatax/n810356/n3010387/c5236346/content.html>
- 国家税务总局浙江省税务局公告：电子发票服务平台开具的纸质专票与纸质普票也展示平台赋予的 20 位号码，证明票面媒介与编号制式不能绑定。
  <https://zhejiang.chinatax.gov.cn/art/2022/9/29/art_24105_564257.html>
- 国家税务总局关于普通发票代码、号码的文件及后续解读：旧制普通发票号码为 8 位，代码采用 12 位，历史 10 位代码仍可继续使用。
  <https://www.chinatax.gov.cn/n810341/n810765/n812193/n813008/c1203713/content.html>
  <https://www.chinatax.gov.cn/n810341/n810760/c2959369/content.html>

### 7.2 中央号码登记

新增小表 `invoice.invoice_number_registry`：

- `legal_entity_id`
- `invoice_medium`
- `number_scheme`
- `invoice_code`、`invoice_no`
- `identifier_key`（数据库生成列）
- `owner_type`：`SALES_BLUE`、`PURCHASE_BLUE`、`OUTPUT_RED`、`INPUT_RED`
- `owner_id`

`legal_entity_id`、`invoice_medium`、`number_scheme`、`invoice_no`、`owner_type` 与 `owner_id` 全部 `NOT NULL`；`identifier_key` 为生成列，其结果也声明 `NOT NULL`。数据库 CHECK 固定把 `invoice_medium` 限定为 `ELECTRONIC | PAPER`、`number_scheme` 限定为 `UNIFIED_20 | LEGACY_CODE_NUMBER`、`owner_type` 限定为 `SALES_BLUE | PURCHASE_BLUE | OUTPUT_RED | INPUT_RED`。`UNIFIED_20` 的制式 CHECK 必须显式要求 `invoice_code IS NULL` 且 `invoice_no IS NOT NULL` 并匹配 20 位 ASCII 数字；`LEGACY_CODE_NUMBER` 必须显式要求 `invoice_code IS NOT NULL`、`invoice_no IS NOT NULL` 并分别匹配 10/12 位与 8 位 ASCII 数字。全部枚举与制式 CHECK 都写成 NULL-safe 条件，不依赖 SQL `UNKNOWN` 代替拒绝。

`identifier_key` 的确定性格式为：

```text
UNIFIED_20:<invoice_no>
LEGACY_CODE_NUMBER:<invoice_code>:<invoice_no>
```

`identifier_key` 必须是 `GENERATED ALWAYS AS (...) STORED` 数据库生成列，调用方不得传入；生成表达式只读取同一行的 `number_scheme`、`invoice_code` 与 `invoice_no`。同一 CHECK 约束按第 7.1 节逐字验证两种制式，保证号码列与生成键不可能分叉。唯一约束固定为 `(legal_entity_id, identifier_key)`，并另设 `(legal_entity_id, owner_type, owner_id)` 唯一约束。同一法人内完整法定标识跨销项蓝票、进项蓝票、销项红票和进项红票全库唯一，不以方向、供应商、原发票或业务表缩小范围；不同法人可重复。

需要法定号码的登记先生成业务头 UUID，在同一事务中插入含该 `owner_id` 的号码登记行，再写业务头表。号码登记表先建立 `(legal_entity_id, id)` 候选唯一键；对应业务头的 `invoice_number_registry_id` 必须以 `(legal_entity_id, invoice_number_registry_id)` 复合外键指回该键并加唯一约束。数据库使用 `DEFERRABLE INITIALLY DEFERRED` 的约束触发器，在事务提交前双向核对 `owner_type`、`owner_id`、业务头 id 与登记引用；唯一写服务还须在事务末重读断言，且每日自检扫描零个孤儿、错配或多主引用。并发重复只能有一个事务成功，不采用“依次查询三张表再插入”的竞态方案。

号码与代码使用 `text` 保存并保留前导零。进入表前只裁剪首尾普通空白；内部空白、全角数字和非数字字符直接拒绝，不静默转换。号码登记表仅允许发票应用服务角色插入，按 `APPEND_ONLY` 登记并启用法人 RLS；不提供普通查询端点，任何号码展示都必须经所属业务单据的读权限。重复提示只有在操作者有权查看原记录时才返回链接，否则只返回通用重复错误，避免通过唯一约束泄露无权单据。

完成第 12 节权威回写后，PRD 第 6.16 节的 F-01 与附录乙 U-D-03 据此整体关闭，不再维持“一处部分关闭、另一处整条待决”的状态。

## 8. 已过账凭证的三类更正入口

凭证生成来源仍按 F-48 固定为四类：业务事件固定映射、年度损益结转、更正凭证、资金单据冲正凭证。这里的“三类”是更正业务入口，不是凭证来源数，也不是 `VoucherSourceKind` 细分枚举数。

唯一适用矩阵如下：

| 事实 | 唯一入口 | 禁止替代 |
|---|---|---|
| 发票票面事实因作废、折让、退货、服务中止等需要撤销或减额 | 有退货、终止等源业务动作时先登记源单据；需要改票时再走销项作废或发票红字冲销，凭证由这些事件自动生成 | 不得跳过源业务单据；不得使用资金冲正或总账更正凭证冒充发票事实 |
| 到款、付款、退款的金额、往来方、账户、日期或“实际未发生”登记错误，资金腿须一并取反 | 资金单据冲正 | 不得用发票红冲；不得只改总账而保留错误资金单 |
| 源业务事实与资金事实均正确，仅已过账凭证科目归类错误 | 引用原凭证的总账更正凭证 | 不得修改原凭证；不得改变源业务单据或资金腿 |

三条共同规则：原凭证不可修改或删除；首版不反结账；不提供手工自由分录。

总账更正凭证在首版进一步冻结为成本同侧矩阵 `MAIN_OPERATING_COST↔DIRECT_EXPENSE_COST`，收入侧只有一个可归集角色，故 `MAIN_OPERATING_REVENUE` 及任何跨成本/收入侧更正均不可达。上限按被引用原凭证行累计，不按整张凭证借方总额粗算。提交审批前须把每行金额落实到同一原凭证行的当前 live `costing.cost_entries`：单候选可自动补全，多候选必须显式分配；规范化后的 entry id 与金额清单进入加密审批快照。回调只能重验获批清单的同一组开放额，事实漂移时整笔拒绝并要求重提，不得自动换到另一来源行或维度。

合同终止的影响面目录仍恰为七类业务对象，凭证不是第八类人工处置对象。合同终止产生的红冲、退货、退款等源业务动作自动生成后续凭证；资金冲正只纠正资金登记错误，更正凭证只纠正会计重分类，二者都不因合同终止自动产生。

回写必须覆盖 F-49 已定位的五处旧封闭句，以及同义残留：

- 规格第 8 章、第 19 章、第 22 章相关句；
- PRD 合同影响面与状态机相关句；
- PRD 到款更正、总账、诚实披露及附录乙；
- 阶段 9 的更正凭证两张表、`post_correction`、`CORR` 类型码、RLS、审计、端点和测试；
- 阶段 10 的资金冲正称谓与引用。

历史审计快照可以保留旧引文，但必须明确标注为历史，不得被当前计划引用。

## 9. 历史成交资料

历史成交默认只排除已经没有可供参考之成交部分的记录：已作废、票面金额已全部红冲、已全额退货。这里的“成交部分”是来源业务行的剩余业务数量与剩余票面价税金额，不是财务台账的 `effective_open`。对发票来源行，`remaining_gross > 0` 才表示仍有金额对价；第 6.4 节因金额型更正而进入全红终态时，即使审计用 `remaining_quantity > 0` 也不得继续默认展示或取价。部分红冲与部分退货仍同时存在数量和金额有效部分时，默认展示并明确标记状态，不得被“已红冲/已退货”筛选整体排除。

`TradeHistoryItem` 分开增加 `is_visible_by_default` 与 `is_selectable_as_price_source`，禁止用一个布尔值同时承担展示与取价资格：

| 当前事实 | 默认可见 | 可直接作为价格来源 |
|---|---:|---:|
| 未失效，且至少一条来源行同时有剩余业务数量和剩余价税金额 | true | true |
| 仅按原单价减少数量的部分红冲、部分退货，且仍有数量与金额 | true | true |
| 含 `ADJUSTED` 的折让、价格或税额更正型部分红冲，且仍有金额对价 | true | false |
| 已作废、已全额退货，或票面三项金额已全部归零（即使审计剩余数量大于零） | false | false |

四个业务模块提供者负责按每个来源成交行的剩余业务数量、`remaining_net/tax/gross`、业务状态，以及冲销行的 `quantity_effect_kind` 与 `pricing_effect_kind` 映射这两个布尔值，MDM 聚合器不解析其他模块的状态字符串。聚合公式固定为：`is_visible_by_default = 非终态且至少一条来源行 remaining_business_quantity > 0 且 remaining_gross > 0`；`is_selectable_as_price_source = is_visible_by_default 且所选来源行从未出现仍有效的 ADJUSTED 且 remaining_business_quantity > 0`。仅 `REDUCE + ORIGINAL_UNIT_PRICE` 的部分减少仍可沿用原单价。配置项统一为 `EP__MDM__TRADE_HISTORY__INCLUDE_INEFFECTIVE`，默认 `false`；打开后只改变终态记录的可见性，绝不把不可选记录变成可选。

部分红冲记录继续展示原成交数量、原单价与当前状态，不派生可能产生歧义的“净单价”。操作者显式选用记录时，服务端必须重新读取当前业务状态并重算 `is_selectable_as_price_source`，再执行当前价格权限与折扣校验，不能信任列表快照或客户端传回的布尔值；查询后才变为全红或发生金额型更正时必须拒绝选用。

## 10. 事务、并发与错误处理

1. 需要法定号码时，发票头、行、号码登记、应收应付条目、核销关系与总账凭证在同一数据库事务中完成；`VOID` 不写号码登记。
   销项、进项原票登记先预生成发票 id 与 AR/AP `ORIGINAL` id；两对双向法人复合外键均为 `DEFERRABLE INITIALLY DEFERRED`。用例可先写引用未来原票 id 的 AR/AP 与自动核销效果并计算凭证计量项，但原票头必须等 `PostingPort::post` 返回非空 `voucher_id` 后才一次插入，且当次就同时具备非空 AR/AP id 与凭证 id。`IdempotentReplay` 只能返回已经完整存在且 id 全匹配的图，`Skipped` 非法；不得用可空列或事后 UPDATE 回填绕过该顺序。
2. 所有资金、发票、采购退货、GRNI 与库存联动事务使用同一跨表锁序：原款项/退款单 → 退款来源链接 → 采购退货单 → 原发票头 → 原发票行 → 收货行 → GRNI 根/效果 → 库存可用量 advisory/金额/覆盖/数量/序列号 → 应收/应付正向主条目 → 应付申请占用键/行 → 预收/预付条目 → 核销根 → 核销效果行 → 发票冲销累计 → 资金冲正累计。没有的类别直接跳过；实现先无锁收集全部候选对象 id/维度键，再按上述类别及每类冻结排序统一加锁（普通 id 取 `id ASC`，GRNI 取 `root_effect_id ASC,id ASC`，库存取第 10.0.1 节键序），锁后重载并校验依赖集合；若并发写入使相关对象集合发生变化，则整笔事务经既有 40001 重试策略重跑，不得在锁到一半时临时追加逆序对象。全部锁定并取得经验证的事务锁证明后，才可按业务所需的 LIFO、容量、GRNI、库存与金额公式计算或写任何业务事实。
3. 号码占用依赖数据库唯一约束；唯一冲突统一映射为业务错误，不把另一张无权单据的属性写入响应。
4. 本事务内发现 `effective_open < 0`、`advance_open < 0`、根分组净核销越界、释放分配合计不等于 `L`、资金冲正拆分不守恒、头行金额不等或本次触及的子账总账勾稽不平时，整笔业务事务回滚，不静默截断。
5. 每日对账或关账重算发现既有历史期间切片不平时，新增差异事项、触发告警并阻断关账；不得声称能回滚已经提交的历史业务事务，也不得自动生成调账分录。
6. 已过账凭证、核销关系、冲销条目与号码登记均只追加或受限状态推进，不覆盖历史金额。

### 10.0.1 跨模块锁协调器与事务锁证明

第 10 条第 2 项不是调用方可自行解释的伪代码。唯一实现落点固定为 `ep_contract_ledger::f50_lock` 的两个 trait 与 DTO、`ep-app-ledger` 的 `LedgerF50LockCoordinator`，以及四个 owner application crate 对锁切片 SPI 的实现。不得在 procure/invoice/finance/sales 用例中跨 schema 写 `SELECT ... FOR UPDATE`，不得各自复制一份锁序，也不得以“稍后由被调端口自行补锁”代替一次锁全集。

`ep-foundation` 只新增业务无关的 `ep_foundation::port::tx::TransactionLockProof` 不透明载体；四个业务 contract 的 mutator 都只引用这个 foundation 类型，因此不产生 contract→contract 依赖。F-50 类别、键、规则与协调器只存在于 owner `ep-contract-ledger`。精确契约如下，字段不得删减或换成字符串化 JSON：

```rust
pub enum F50LockOwner { Finance, Procure, Invoice, Inventory }
pub enum F50LockCategory {
    OriginalCashDocument,
    RefundSourceLink,
    PurchaseReturn,
    OriginalInvoice,
    OriginalInvoiceLine,
    GoodsReceiptLine,
    GrniEffect,
    InventorySourceDocument,
    InventoryAvailability,
    InventoryValueBalance,
    InventoryCoverageBalance,
    InventoryQuantityBalance,
    InventorySerial,
    ArApOriginalEntry,
    PayableReservation,
    AdvanceEntry,
    SettlementRoot,
    SettlementEffect,
    InvoiceReversalAccumulator,
    FinanceReversalAccumulator,
}

pub enum F50CashDocumentKind { Receipt, Payment, CustomerRefund, SupplierRefund }
pub enum F50InvoiceSide { Sales, Purchase }
pub enum F50LedgerSide { Receivable, Payable }
pub enum F50AdvanceSide { Receipt, Payment }
pub enum F50SettlementSide { Receivable, Payable, AdvanceReceipt, AdvancePayment }
pub enum F50InvoiceAccumulatorKind { SalesLine, PurchaseLine }
pub enum F50FinanceAccumulatorKind { CashDocument, RefundSource, Overbilling }
pub enum F50InventorySourceKind {
    PurchaseReceipt,
    PurchaseReturn,
    DeliveryConfirmation,
    SalesReturn,
    PurchaseInvoice,
    MigrationStockAdjustment,
}

pub struct F50CashDocumentKey { pub kind: F50CashDocumentKind, pub id: uuid::Uuid }
pub struct F50InvoiceKey { pub side: F50InvoiceSide, pub id: uuid::Uuid }
pub struct F50InvoiceLineKey {
    pub side: F50InvoiceSide,
    pub invoice_id: uuid::Uuid,
    pub line_id: uuid::Uuid,
}
pub struct F50GrniEffectKey { pub root_effect_id: uuid::Uuid, pub effect_id: uuid::Uuid }
pub struct F50InventorySourceKey {
    pub kind: F50InventorySourceKind,
    pub source_doc_id: uuid::Uuid,
}
pub struct F50InventoryValueKey { pub warehouse_id: uuid::Uuid, pub material_id: uuid::Uuid }
pub struct F50InventoryQuantityKey {
    pub warehouse_id: uuid::Uuid,
    pub material_id: uuid::Uuid,
    pub batch_no: String,
}
pub struct F50LedgerEntryKey { pub side: F50LedgerSide, pub id: uuid::Uuid }
pub struct F50AdvanceEntryKey { pub side: F50AdvanceSide, pub id: uuid::Uuid }
pub struct F50SettlementKey { pub side: F50SettlementSide, pub id: uuid::Uuid }
pub struct F50InvoiceAccumulatorKey {
    pub kind: F50InvoiceAccumulatorKind,
    pub source_line_id: uuid::Uuid,
}
pub struct F50FinanceAccumulatorKey {
    pub kind: F50FinanceAccumulatorKind,
    pub source_id: uuid::Uuid,
}

pub struct F50LockPlan {
    pub legal_entity_id: uuid::Uuid,
    pub cash_documents: Vec<F50CashDocumentKey>,
    pub refund_source_link_ids: Vec<uuid::Uuid>,
    pub purchase_return_ids: Vec<uuid::Uuid>,
    pub original_invoices: Vec<F50InvoiceKey>,
    pub original_invoice_lines: Vec<F50InvoiceLineKey>,
    pub goods_receipt_line_ids: Vec<uuid::Uuid>,
    pub grni_effects: Vec<F50GrniEffectKey>,
    pub inventory_sources: Vec<F50InventorySourceKey>,
    pub inventory_availability_keys: Vec<F50InventoryValueKey>,
    pub inventory_value_keys: Vec<F50InventoryValueKey>,
    pub inventory_coverage_keys: Vec<F50InventoryValueKey>,
    pub inventory_quantity_keys: Vec<F50InventoryQuantityKey>,
    pub inventory_serial_nos: Vec<String>,
    pub ar_ap_original_entries: Vec<F50LedgerEntryKey>,
    pub payable_reservation_purchase_invoice_ids: Vec<uuid::Uuid>,
    pub advance_entries: Vec<F50AdvanceEntryKey>,
    pub settlement_roots: Vec<F50SettlementKey>,
    pub settlement_effects: Vec<F50SettlementKey>,
    pub invoice_reversal_accumulators: Vec<F50InvoiceAccumulatorKey>,
    pub finance_reversal_accumulators: Vec<F50FinanceAccumulatorKey>,
}

pub struct F50LockLease { /* opaque authenticated bytes; not accepted by mutators */ }

#[async_trait::async_trait]
pub trait F50LockSlicePort: Send + Sync {
    fn owner(&self) -> F50LockOwner;
    async fn lock_category(
        &self,
        tx: &mut dyn Tx,
        ctx: &SecurityContext,
        category: F50LockCategory,
        plan: &F50LockPlan,
    ) -> Result<(), AppError>;
}

#[async_trait::async_trait]
pub trait CrossModuleLockCoordinator: Send + Sync {
    async fn lock_all(
        &self,
        tx: &mut dyn Tx,
        ctx: &SecurityContext,
        collected: &F50LockPlan,
    ) -> Result<F50LockLease, AppError>;

    async fn seal_after_reload(
        &self,
        tx: &mut dyn Tx,
        ctx: &SecurityContext,
        lease: F50LockLease,
        reloaded: &F50LockPlan,
    ) -> Result<TransactionLockProof, AppError>;

    async fn assert_covers(
        &self,
        tx: &mut dyn Tx,
        ctx: &SecurityContext,
        proof: &TransactionLockProof,
        required: &F50LockPlan,
    ) -> Result<(), AppError>;
}
```

`F50LockPlan` 的规范化是契约的一部分：全部 vector 先按其字段声明顺序升序排列再逐值去重；`batch_no` 与序列号先按各 owner 的强类型解析器校验，再使用其规范字符串；同一 key 以不同枚举 side/kind 出现不是重复。JCS 序列化规范化计划后取 SHA-256 得 `plan_digest`。`lock_all` 必须验证计划法人等于 `ctx.legal_entity_id`，先由非空类别推导本次 `required_owners`，再验证每个 required owner 在装配注册表中恰有一个 `F50LockSlicePort`；空类别不要求 owner、也不调用 owner。随后严格按 `F50LockCategory` 上述枚举顺序调用 owner。类别到 owner 的映射封闭为 Finance、Finance、Procure、Invoice、Invoice、Procure、Procure、Inventory×6、Finance、Procure、Finance、Finance、Finance、Invoice、Finance，不允许运行期配置。重复 owner、owner 自报与注册槽不符，或任一非空类别缺 owner，均在首条锁 SQL 前失败关闭。

阶段施工与完整发布使用两层装配判据，二者不得混写。阶段 8 的库存契约/owner 测试只注册非空计划所需的 Inventory，阶段 6 的销售库存腿同样只要求 Inventory，阶段 7 的收货与未开票退货按实际计划要求 Inventory/Procure；不得为尚未交付的 Finance/Invoice 注入 Noop、Stub 或空 slice。到阶段 10 四个 owner 均已交付后，`apps/core-server --check` 与 `apps/job-worker --check` 的独立静态装配门禁必须验证 Finance、Procure、Invoice、Inventory 四槽各恰一个，缺一、重复或错槽则完整首版制品不投入运行。这个最终发布门禁不改变 `lock_all` 的按计划最小 owner 判据。

调用方在任何业务计算或写入之前按固定五步执行：一，无锁通过 owner query port 收集 `collected`；二，调用 `lock_all` 得到只表示“已尝试锁定”的 `F50LockLease`；三，所有类别锁完后只读重载同一关系图，重建 `reloaded`；四，调用 `seal_after_reload`，只有规范化后的两个 plan 逐字相等才返回 `TransactionLockProof`；五，把该 proof 传给每个 owner mutator。集合漂移时 `seal_after_reload` 不返回 proof，而由 `PgF50LockEpochRepo` 执行固定的 SQLSTATE `40001` 中止本事务，复用全局 RetryPolicy 的 50/150/450 毫秒三次重试；三次仍漂移才按基础设施冲突返回，不把半锁事务继续执行。

proof 由 `LedgerF50LockCoordinator` 使用进程启动时生成的 32 字节随机 seal key 做 HMAC-SHA-256，认证载荷固定含版本 `F50_LOCK_V1`、`Tx::tx_id()`、法人、`plan_digest` 与 `sealed=true`。proof 不落库、不进日志、不进事件、不跨请求；进程崩溃时数据库事务回滚，旧 proof 随 tx id 失效。`assert_covers` 必须同时验证 HMAC、当前 tx id、当前法人、sealed 标志及 required plan 是已锁 plan 的逐类别子集；任一不成立返回内部不变量错误并整笔回滚。公开构造任意 bytes 不构成授权，因为没有 seal key 无法通过验证。所有 F-50 owner mutator（包括 GRNI、库存 post/split、AR/AP/预收预付、核销、退款、红冲、超量挂账与占用变化）在首条 SQL 前必须调用 `assert_covers`，缺 proof、传 lease、跨事务复用、少类别或少 key 均零业务写入；已经持有的行只锁后重读，不再补锁。

库存 owner 的六个类别在 F-50 大序内固定子序为：先按 `(kind,source_doc_id)` 对全部来源取 `inventory-source:` transaction advisory，以覆盖尚不存在的 movement 并保护 `(legal_entity_id,source_doc_type,source_doc_id)` 幂等根；再对全部 `(legal_entity_id,warehouse_id,material_id)` 取 `sales-availability:` advisory，随后依次锁 value balance、coverage balance、quantity balance、serial advisory/state。缺失余额允许在对应类别内按稳定键执行 `INSERT ... ON CONFLICT DO NOTHING` 的零余额锁脚手架后再锁；这是 coordinator 内部为锁住不存在键所必需的结构初始化，不是业务效果，除该受控初始化与 `AccountingPeriodResolver` 的零期间建立外，proof 前仍禁止任何业务事实、金额、数量、状态、号码、幂等终结或消息写入。库存 mutator 收到 proof 后不得再次取得来源 advisory 或余额锁。应付占用类别先对 `(legal_entity_id,purchase_invoice_id)` 取 `payable-reservation:` advisory，再锁已存在的 reservation 行，覆盖尚不存在行；任何增加 reservation 或降低对应 AP `effective_open` 的路径都必须包含该 key。

### 10.1 新增错误码封闭表

下表是 F-50 新增或替代错误的唯一集合；category 决定 HTTP 与 `retryable`，均沿用 `docs/error-codes.md` 第 1 节，其中 VALIDATION 为 `400/false`、BUSINESS_CONFLICT 为 `409/false`。实施前必须把下表逐行登记到 `docs/error-codes.md` 与 `ep-foundation` 常量表，端点、插件与 Excel 返回同一码。

| 错误码 | category | 精确触发条件 |
|---|---|---|
| `FINANCE.SETTLEMENT.EFFECT_INVALID` | VALIDATION | effect/root/reverses 形态、方向或来源枚举非法 |
| `FINANCE.SETTLEMENT.ROOT_INVARIANT_VIOLATED` | BUSINESS_CONFLICT | 跨条目/跨根/成环、父子上限或根净额上下界不成立 |
| `FINANCE.SETTLEMENT.EFFECTIVE_OPEN_CHANGED` | BUSINESS_CONFLICT | 锁后候选集合或有效未核销容量相对请求快照已变化 |
| `FINANCE.SETTLEMENT.AMOUNT_EXCEEDS_EFFECTIVE_OPEN` | BUSINESS_CONFLICT | 请求中的单笔核销金额超过锁后 `effective_open` |
| `FINANCE.SETTLEMENT.RELEASE_ALLOCATION_MISMATCH` | BUSINESS_CONFLICT | 红冲释放分段合计不等于锁后计算的 `L` |
| `FINANCE.REFUND.SOURCE_ALLOCATION_MISMATCH` | VALIDATION | 来源链接金额之和不等于退款额，或任一链接不满足逐来源守恒 |
| `FINANCE.REFUND.SOURCE_CAP_EXCEEDED` | BUSINESS_CONFLICT | 任一来源链接金额超过该原款项可追溯的预收预付与有效核销合计 |
| `FINANCE.CASH_DOCUMENT.DOWNSTREAM_REFUND_EXISTS` | BUSINESS_CONFLICT | 原到款/付款仍有未冲正退款或返款，禁止资金冲正 |
| `FINANCE.CASH_DOCUMENT.TRACEABILITY_MISMATCH` | BUSINESS_CONFLICT | 某原款项锁后 `R != S + V` 或退款冲正逐来源守恒不成立 |
| `FINANCE.CASH_DOCUMENT.POSTING_SPLIT_MISMATCH` | BUSINESS_CONFLICT | finance 计算的往来腿与预收预付腿不守恒或不等于原资金腿 |
| `FINANCE.RECONCILIATION.BALANCE_MISMATCH` | BUSINESS_CONFLICT | 本次写入后的子账/总账或当前/最新期间勾稽差额非零 |
| `INVOICE.INVOICE_LINE.HEAD_AMOUNT_FORBIDDEN` | VALIDATION | 写请求、插件或 Excel 提交头税率或头金额字段 |
| `INVOICE.INVOICE_LINE.TAX_AMOUNT_OUT_OF_TOLERANCE` | VALIDATION | 普通行税额与 half-up 期望值差额超过配置容差 |
| `INVOICE.INVOICE_LINE.AMOUNT_EQUATION_INVALID` | VALIDATION | 行价税合计不等或头行服务端汇总断言失败 |
| `INVOICE.INVOICE_REVERSAL.LINE_SOURCE_MISMATCH` | VALIDATION | 来源行两组 id 非恰一组、方向不符、跨票或跨法人 |
| `INVOICE.INVOICE_REVERSAL.EFFECT_KIND_INVALID` | VALIDATION | quantity/pricing effect 组合、纯税特例或末次尾差分类非法 |
| `INVOICE.INVOICE_REVERSAL.REMAINING_AMOUNT_EXCEEDED` | BUSINESS_CONFLICT | 任一来源行累计净额、税额或价税合计超过锁后剩余量 |
| `INVOICE.INVOICE_REVERSAL.REMAINING_QUANTITY_EXCEEDED` | BUSINESS_CONFLICT | 任一来源行累计冲销数量超过锁后剩余数量 |
| `INVOICE.INVOICE_REVERSAL.STATE_CHANGED` | BUSINESS_CONFLICT | 原票或来源行状态/版本在提交前变化，当前动作不再合法 |
| `INVOICE.INVOICE_NUMBER.FORMAT_INVALID` | VALIDATION | 编号制式、媒介、代码或号码不满足第 7 节封闭组合 |
| `INVOICE.INVOICE_NUMBER.DUPLICATED` | BUSINESS_CONFLICT | 同法人中央号码键已被任一蓝票或红票占用 |
| `INVOICE.INVOICE_NUMBER.OWNER_MISMATCH` | BUSINESS_CONFLICT | 登记 owner 与业务头方向、类型或 id 不一致 |
| `INVOICE.IMPORT.TEMPLATE_VERSION_UNSUPPORTED` | VALIDATION | 模板版本不是三个 v2 之一，或仍含头金额/头税率列 |
| `INVOICE.IMPORT.GROUP_HEADER_MISMATCH` | VALIDATION | 同一 `document_key` 的重复头字段不一致或行号重复 |
| `MDM.TRADE_HISTORY.PRICE_SOURCE_NO_LONGER_ELIGIBLE` | BUSINESS_CONFLICT | 列表后状态变化，提交时重算已不可作为价格来源 |
| `PORTAL.SUPPLIER_INVOICE_UPLOAD.DUPLICATED` | BUSINESS_CONFLICT | 同法人、同供应商、同规范化发票标识已有未作废上传记录 |
| `PORTAL.SUPPLIER_INVOICE_UPLOAD.STATE_CHANGED` | BUSINESS_CONFLICT | 上传记录不再为 `UPLOADED` 或 row_version 已变化 |
| `PORTAL.SUPPLIER_INVOICE_UPLOAD.CONTENT_MISMATCH` | BUSINESS_CONFLICT | 正式进项登记内容与锁后上传头行不一致 |
| `LEDGER.CASH_REVERSAL.SPLIT_INVALID` | BUSINESS_CONFLICT | ledger 收到非受控来源、负金额或两腿合计不等于原资金腿 |
| `LEDGER.CORRECTION_VOUCHER.SOURCE_NOT_POSTED` | BUSINESS_CONFLICT | 更正凭证引用的原凭证不存在、无权或未过账 |
| `LEDGER.CORRECTION_VOUCHER.AMOUNT_EXCEEDED` | BUSINESS_CONFLICT | 本次加历史累计更正金额超过原凭证对应行金额 |
| `LEDGER.CORRECTION_VOUCHER.ENTRY_NOT_ALLOWED` | VALIDATION | 更正请求试图改变资金/业务事实、使用自由科目或不平衡分录 |

泄漏规则同样冻结：VALIDATION 的 `details` 只含本请求字段路径；守恒错误只返回本单据内可见的安全金额与 `incident_no`，不返回表名、约束名或其他资金根；号码重复仅在当前主体有原单据读权限时返回业务链接，否则 `details` 为空；无权或不存在仍统一使用平台的 `NOT_FOUND_OR_DENIED`，不得用上述业务码暴露记录存在性。

## 11. 最小验收矩阵

### 11.1 核销与红冲

1. 原票 100、净核销 60、有效未核销 40，本次红冲 30：`L = 0`、红冲后 `effective_open = 10`、不产生预收，银行不变。
2. 同一前置本次红冲 50：`L = 10`、红冲后 `effective_open = 0`、预收增加 10，银行不变；总账与子账同时相等。
3. 原票 100、收款 100、先退款 30 后全红：最终应收 0、预收 70、银行净额 70。
4. 同一事实先全红后退款 30：终态与上一条逐项相同。
5. 原票 100、收款 100、退款 30、全红 100、再冲正退款：最终应收 0、预收 100、银行净额 100，四张台账与总账同时相等。
6. 预收 100、退款 30、新开票 100 并自动核销 70、再冲正退款：恢复资金自动核销剩余应收 30，最终应收 0、预收 0。
7. 原票 100、收款 100、红冲 30、再冲正原收款：只释放当前核销 70 并消耗可追溯预收 30，最终应收 70、预收 0、银行净额 0。
8. 第 5—7 条各跑供应商付款、预付与返款镜像，借贷方向相反但守恒式相同。
9. 退款后立即冲正退款、随后全红：最终应收 0、预收 100、银行净额 100，与第 5 条终态相同。
10. `DIRECT_CASH 40 + ADVANCE_AUTO 30 + DIRECT_CASH 30`，释放 50：按 LIFO 先释放最新 `DIRECT_CASH 30` 并新建可追溯预收 30，再释放 `ADVANCE_AUTO 20` 并恢复原预收 20。
11. 同一退款在同一主条目上有两个各 15 的 `RELEASE`，冲正时容量仅 10：递减容量使 `sum(A_i) = 10`，不得两行各取 10。
12. 绕过应用层直接向真实 PostgreSQL 写入下列每种非法行都必须被 CHECK、复合自引用外键或延迟约束触发器拒绝：根行 `root_apply_id != self`、派生行 effect 与父行同向、直接反向累计超父行、根净额小于 0 或大于根金额、跨法人/台账侧/所属条目/根，以及反向链成环；应用层另断言同一业务错误码。
13. 原到款/付款仍存在未冲正的下游退款/返款，或可追溯 `S + V` 不等于原金额：资金冲正拒绝并列出依赖或差额。
14. 一笔客户退款 70 关联收款 A 40 与收款 B 30：每条来源链接分别满足守恒，所有效果行保留对应 source link；冲正退款后再分别冲正 A、B 时都能独立复算 `R = S + V`。尝试把 A 的 10 分配到 B 根、把两个 `Q_j` 合池或只让整单金额守恒时必须拒绝；供应商镜像同测。

### 11.2 历史期间

15. `MIGRATION_OPENING` 应收、应付各 100：当前视图、账龄和首期历史切片均为 100，且可被正常核销；其 `C = 0`。
16. `MIGRATION_OPENING` 预收、预付各 100：`advance_open = 100`；核销 40 后为 60，释放 10 后为 70，始终满足上下界。
17. M1 开票 100、M2 收款 60、M3 退款 20：三期应收切片依次为 100、40、60；重跑 M1、M2 不受 M3 影响。另测 M1 开票 100、M2 收款 100、M3 红冲 30、M4 冲正原收款：四期应收依次为 100、0、0、70，预收依次为 0、0、30、0。
18. M1 预收 100、M2 销项开票 70 并自动核销、M3 释放 20：M2 唯一开票凭证在基础腿外严格增加借预收 70、贷应收 70，三期预收切片依次为 100、30、50；重跑早期结果不变。供应商镜像以采购发票 70 验证同一张凭证增加借应付 70、贷预付 70，且两侧效果行逐资金根一一对应。
19. 最新期间累计值分别等于当前应收 `sum(effective_open)`、应付 `sum(effective_open)`、预收 `sum(advance_open)` 与预付 `sum(advance_open)`；同一红冲数据经信用敞口、核销候选与上限、付款申请上限、客户/供应商门户、账龄、报表、Excel 和对账查询读取时逐项一致，不出现第二套经营余额。
20. 顺延事件只进入实际 `accounting_period_id` 对应切片，`business_date` 只影响检索和账龄。
21. 每日或关账重算故意制造历史差额时，生成差异事项并阻断关账，不改写历史业务行、不自动调账。

### 11.3 发票

22. 一张销项票含 13% 与 6% 两行，头表三项金额精确等于行合计，头表不存在税率且请求提交头表金额会被契约拒绝。
23. 一张同属 `INVENTORY_TYPE` 的进项票含 13% 与 0% 两行可登记，0% 行税额为 0；进项头没有税率，`cost_kind` 由行推导。同一请求或待受理上传混合两种 `cost_kind` 必须整单拒绝。请求不得带暂估/价差/超量/自动核销结果；响应逐行及头级返回四个服务端结果字段并返回 `advance_auto_applied_amount`，不得出现未拆分总价差或超量布尔权威值。
24. F-03/U-D-05 的十进制定点用例覆盖 `0.05 × 10% = 0.005` 按 half-up 得 `0.01`、税额差 `0.02` 接受而 `0.03` 拒绝、价税合计差一分即拒绝；`REDUCE + ORIGINAL_UNIT_PRICE` 严格按原单价计量且仅在此前无 `ADJUSTED` 时吸收可重算的末次尾差；先登记任一 `ADJUSTED` 后再把非尾差剩额伪装成末次原价行必须拒绝并要求改记 `ADJUSTED`。`REDUCE + ADJUSTED` 与 `NONE + ADJUSTED` 可表达折让或价格更正；`NONE + ORIGINAL_UNIT_PRICE` 被 CHECK 拒绝。纯税额行 `net = 0, tax > 0` 只走第 6.4 节特例，仍按原行税率、剩余税额上限和项目唯一小数位校验。绕过应用层直写真实 PostgreSQL 时，任一 effect kind、`source_effect_seq`、数量、税率或金额为 NULL，序号重复/跳号、非末次原价金额偏差，以及已有 `ADJUSTED` 后伪装末次尾差，均必须被 NOT NULL、唯一/CHECK 或延迟约束触发器拒绝。
25. 只红冲一行后原票进入部分红冲；第二次可继续冲，但每个来源行的累计数量及三项金额分别不得超出原行；只有所有来源行三项金额都归零才进入全额红冲终态。
26. 两笔并发部分红冲单笔都合法但合计超额时只能一笔成功；`VOID`、红字冲销、退款与相关资金冲正并发时只能落成某一合法串行次序，且所有台账与总账勾稽差额为零，使用真实 PostgreSQL 验证锁序及集合变化重试。
27. 跨法人原票、跨法人原行、同法人但冲销行不属于冲销头原票、以及方向与两组 FK 不一致，绕过应用层直写数据库时也必须被复合 FK、XOR CHECK 或延迟约束触发器拒绝。
28. 20 位号码跨销项蓝票、进项蓝票、销项红票、进项红票重复时拒绝；并发登记只能一个事务成功。
29. 旧制号码相同但代码不同允许，代码与号码均相同时跨表拒绝；前导零完整保留。
30. `identifier_key` 只能由数据库生成；对必填键直写 NULL、制式与代码/号码不匹配、用 NULL 绕过唯一键均被 `NOT NULL` 或 NULL-safe CHECK 拒绝，registry owner 与业务头错配被延迟约束拒绝。
31. `VOID` 不生成号码登记；同一完整标识在另一法人可登记。
32. 普通用户不能直接查询号码登记表；无权用户收到的重复提示不泄露原记录，授权用户才得到业务单据链接。
33. HTTP、插件及三个 v2 Excel 模板经同一验证器产生完全相同的头行模型和错误码；旧模板、头金额/头税率列、同组头字段不一致及重复行号逐项拒绝。
34. 两行多税率供应商上传被受理时，同一事务创建进项头行、中央号码与 `ACCEPTED` 回写；号码已占用或上传并发退回时事务整体失败且不留下半张正式发票。
35. 供应商上传退回后原记录保持 `RETURNED`，重传生成新记录；正式登记同时提交与上传不一致的第二套行内容时拒绝。
36. 物料收货暂估 100、采购发票 120 时，采购发票只借 GRNI 本金 100，差额 20 按在库/已出库拆分；链接实物退货执行 `+100,-100,+30,-30` 的 GRNI 链，内部红字命令带 `linked_purchase_return_id` 且不写库存，物理退货只生成一张 `PURCHASE_RETURN_INVENTORY`，最终 GRNI 为零、库存只减少一次。另测未开票、已开票、同单混合、零价数量根、正负两类差额、部分退货按锁后移动平均价以及全数退清后库存数量/金额/单价同时为零；任一公开 HTTP/插件/Excel 请求携带链接 id，或销项/VOID/独立进项更正在数据库落非空链接，或链接错法人/供应商/行覆盖、中途失败时，整笔拒绝且无半链。

### 11.4 更正入口与历史成交

37. 发票红冲、总账重分类、到款/付款/退款冲正三类入口的正向用例各自成功，源业务单据与来源凭证完整。
38. 跳过退货/终止源单据直接改票、用资金冲正冒充发票红冲、非资金单据提交资金冲正、无来源自由分录均被拒绝。
39. 原业务单据与原凭证在三类更正后逐字段不变，只新增可追溯记录；动态资金冲正仍只有资金冲正这一来源类别。
40. 四个 `TradeHistoryProvider` 对未失效、`REDUCE + ORIGINAL_UNIT_PRICE` 部分减少、任一 `ADJUSTED` 部分更正、全额失效四组状态，按来源行剩余数量/金额输出与第 9 节矩阵逐项一致。
41. 历史成交默认展示原单价数量型及金额调整型部分红冲并标明状态；前者仍可作为价格来源，任一含 `ADJUSTED` 的来源行可见但不可作为价格来源。
42. 查询时可选、提交选用前已全红或发生金额型更正：服务端重读后返回 `MDM.TRADE_HISTORY.PRICE_SOURCE_NO_LONGER_ELIGIBLE`，不信任旧列表快照。
43. 打开 `EP__MDM__TRADE_HISTORY__INCLUDE_INEFFECTIVE` 后终态记录可见，但 `is_selectable_as_price_source` 仍为 false。
44. `NONE + ADJUSTED` 把来源行三项票面金额冲为零但审计数量仍大于零时，原票进入全红终态，历史成交默认与取价资格均为 false；打开终态展示配置后只变为可见，取价仍为 false。
45. 第 10.1 节每个错误码至少一个端到端负例，精确断言 code/category/HTTP/retryable 与泄漏规则；数据库直写失败另由约束测试断言，不把原始 SQL 错误暴露给端点。

所有负向用例必须断言 `docs/error-codes.md` 中登记的精确错误码；只断言 4xx、数据库异常或“失败了”均不算通过。涉及并发、RLS、复合外键、生成列、延迟约束与期间切片的用例一律使用真实 PostgreSQL，不以内存替身代替。

## 12. 回写边界与完成定义

书面设计确认后，实施计划必须逐条列出以下规范性文件的修改，不得只改其中一层：

- `docs/superpowers/plans/2026-08-10-first-release-dev-plan/00c-gap-ruling.md`：登记 F-50，把 F-49 九条标为已关闭，并在 F-46 旧 `min` 公式与 F-48 仍含“资金冲正无条件镜像原凭证”的句旁标注被 F-50 替代。
- `docs/superpowers/specs/2026-08-17-f10-ruling-detail.md`：在 B-2 标注凭证来源已由 F-48 从三类改为四类，在 B-3 至 B-8 的每个被替代句旁逐项标注 F-50，不让旧详本继续充当当前依据。
- `docs/superpowers/specs/2026-07-19-enterprise-private-operations-platform-design.md`：财务规则、发票、强制不变量、阶段验收与诚实披露。
- `docs/superpowers/specs/2026-08-09-first-release-prd.md`：历史成交、财务第 6 节、F-03/U-D-05 的 half-up/两位金额/六位税率/`0.02` 最大容差结论与纯税额结构性特例、附录乙及术语；F-03/U-D-05 状态改为已关闭。
- `docs/superpowers/plans/2026-08-10-first-release-dev-plan/00-overview.md` 与 `02-data-foundation.md`：登记总数、阶段依赖、全局计数，并同步 T0 对象/迁移/`ep-datagen` 最小样本为“发票头 + 至少一行 + 号码登记”，保证首个可运行切片不再依赖旧单行头模型。
- `docs/superpowers/plans/2026-08-10-first-release-dev-plan/00e-appendix-b-full-sweep.md`、`00g-finance-judgments-needed.md` 与 `00h-b-cluster-blockers.md`：仅对仍会被当前文档引用的旧三来源、旧未决状态与被替代公式追加历史标记，不重写审计快照。
- `docs/superpowers/plans/2026-08-10-first-release-dev-plan/01-engineering-baseline.md`、`03-platform-kernel.md` 与 `04-identity-authz.md`：`archcheck` 只准 `post/post_reversal/post_correction` 写凭证；登记 `CORR` 单据类型码及配置文档；把已过账凭证纠错策略固定为更正凭证，不再只列旧资金冲正。
- `docs/superpowers/plans/2026-08-10-first-release-dev-plan/05-master-data.md`：`TradeHistoryItem` 两个资格字段、按来源行聚合规则与配置项。
- `docs/superpowers/plans/2026-08-10-first-release-dev-plan/06-contract-sales.md`：销售历史成交提供者、退货/红冲前置链、价格选用复核及信用敞口查询统一使用 `effective_open`。
- `docs/superpowers/plans/2026-08-10-first-release-dev-plan/07-procurement-portal.md`：采购历史成交提供者、进项引用、付款申请上限、应付查询与供应商门户余额统一使用 `effective_open`；供应商上传拆为头行、登记号码制式；GRNI 追加事实的 NULL-safe 根幂等、`effect_seq`、零金额数量根及采购退货成对效果；交付第 6.6 节冻结的回写端口及退回入口。
- `docs/superpowers/plans/2026-08-10-first-release-dev-plan/09-ledger-period.md`：更正凭证入口、`post_reversal` 动态拆分契约；`JournalRule` 一键多腿模型、18 个唯一来源、17 个角色、完整计量项/必填/互斥/平衡方程；截至期间的四类子账勾稽、已关闭期间重跑稳定性与阶段计数。
- `docs/superpowers/plans/2026-08-10-first-release-dev-plan/10-ar-ap-invoice.md`：数据表、DTO、端点、算法、当前/历史对账视图、全部查询与 Excel 导出、F-03/U-D-05 纯税额结构性特例、采购发票 GRNI 回冲/价差分录、链接实物退货红字与跨模块统一锁序、测试和风险。
- `docs/superpowers/plans/2026-08-10-first-release-dev-plan/11-cost-metrics-reporting.md`：账龄及报表由 `open_amount` 改为 `effective_open`、数据集签名与测试。
- `docs/superpowers/plans/2026-08-10-first-release-dev-plan/00f-f10-writeback-order.md`：被 F-48/F-50 替代项与剩余回写状态。
- `docs/data-dictionary.md` 及其 `docs/data-dictionary/ledger.md`、`finance.md`、`invoice.md`、`portal.md` 分卷，`docs/config-reference.md`、`docs/error-codes.md`、`docs/event-catalog.md`、`docs/openapi/ledger.v1.yaml`、`invoice.v1.yaml`、`finance.v1.yaml` 与 `portal.v1.yaml`：新增枚举、表列、端口、事件、配置键、精确错误码与唯一 v2 头行契约。
- `docs/superpowers/plans/2026-08-21-f50-financial-consistency-implementation.md`：作为 F-50 唯一开发执行计划，逐任务列出精确文件、迁移顺序、类型签名、失败/通过测试、静态门禁与提交边界；旧阶段计划中被替代段落只作历史基线。

完成必须同时满足：

1. 九条在裁定卷、规格、PRD 与阶段计划中均有唯一当前结论。
2. “退款核销后红冲只能拒绝”“释放额取 `min(已核销, 红字金额)`”“资金冲正无条件镜像原凭证”“按 `reverses_id` 推断正负”“凭证只有三个来源”“进项头表单税率”“每张原票只能红冲一次”等旧口径在当前规范性文档中清零；历史引文必须带已替代标记。
3. 规格、PRD 与阶段计划中的表数、迁移数、RLS 数、类型码数、端点数、审计动作数与新增对象一致；ledger 明确为 14 表/18 迁移/13 张法人 RLS，不能沿用旧 10 张计数。
4. 信用敞口、核销候选与上限、付款申请上限、客户/供应商门户、账龄、报表、Excel 和对账查询在同一红冲用例下逐项返回相同 `effective_open`，不得留第二套经营余额；T0 数据生成回归能创建带至少一行及号码登记的最小发票。
5. 第 6.5、6.6 节的 HTTP/插件/Excel/门户输入和回写端口在 contract、OpenAPI、阶段计划及实施计划中逐字段一致，不存在第二套头金额、单税率或未定端口名。
6. 第 10.1 节全部错误码已登记，所有负向验收可精确判定，且无权重复号码不泄漏原单据。
7. `JOURNAL_MAP` 的每个 `(source_kind,measure_key)` 唯一对应非空 `legs[]`，18 个来源的全部合法计量组合借贷平衡；采购暂估 100/发票净额 120、已开票退货成对效果、混合退货、零金额数量根、正负价差与并发重复根均有唯一可判定结果。
8. 文档静态检查、引用检查、错误码检查与 Markdown 表格检查全部通过。

## 13. 非目标

本设计不引入税务平台直连、发票查验、勾选认证、纳税申报、多币种、反结账、手工自由凭证、万能发票物理表或通用事件溯源平台。上述边界仍按首版延期目录执行。

本设计同时关闭为直接开发所必需的 F-03/U-D-05：小数位、舍入模式、普通行最大容差与纯税额结构性特例均以第 6.1、6.4 节为唯一口径。后续实现不得另行发明舍入模式、使用二进制浮点或把普通行容差放宽到 `0.02` 以上。

本设计也不声称软件规则替代企业财务负责人、税务专业人员或客户所在地主管税务机关的最终判断。编号制式与红字业务规则发生监管变化时，应通过受控版本升级更新验证配置与交付说明。
