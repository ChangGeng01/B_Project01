# F-50 财务一致性与发票模型收口设计

> 日期：2026-08-21
> 状态：对话方案已批准；书面复核与权威文档回写尚未完成
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
| F-10 B-3 的五种资金单据冲正原因 | 原因枚举保留；不得增加“因发票红冲而冲正到款/付款”；“必然逐行取反”的过宽记账语义由第 4.2 节动态拆分整体替代 |
| F-10 B-4 的分次部分红冲 | 分次部分红冲原则保留并扩展为销项、进项逐行累计；其中单一 `source_invoice_id`、单次冲销关系或 `reversed_*` 权威缓存等旧物理表达由第 6.4 节替代 |
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

## 4. 退款、红冲与核销释放

### 4.1 客户退款与供应商返款

客户退款与供应商返款不再生成 `origin = REFUND` 的正向核销行。处理顺序固定为：

1. 锁定退款单关联的原到款/付款、开放预收/预付条目及其核销根分组。
2. 退款额先消耗仍开放且可追溯到所选原款项的预收/预付。
3. 剩余退款额按第 4.4 节的同一 LIFO 次序，对所选原款项对应的有效核销根分组逐组追加 `RELEASE`。
4. 退款凭证记录真实资金腿；核销关系只表达往来余额变化，不另造资金腿。
5. 若退款额大于“可用预收/预付 + 所关联原款项的有效核销”，整笔拒绝。

守恒式为：

```text
refund_amount = advance_consumed + settlement_released
```

四种资金事实的分录方向固定为：

| 事实 | 借方 | 贷方 |
|---|---|---|
| 客户退款消耗预收 | 预收账款 | 银行存款/库存现金 |
| 客户退款释放应收核销 | 应收账款 | 银行存款/库存现金 |
| 供应商返款消耗预付 | 银行存款/库存现金 | 预付账款 |
| 供应商返款释放应付核销 | 银行存款/库存现金 | 应付账款 |

供应商返款完全镜像。资金单据冲正仍仅用于“款项实际未收到、金额、往来方、账户或日期登记错误”五类事实错误，不得用作发票红冲的前置业务步骤。

### 4.2 资金单据冲正按当前资金去向拆分

F-10 B-3 的“任何到款、付款、退款冲正都逐行取反原凭证”只在资金当前去向仍与原登记时相同的情况下成立。后续核销、红冲或新发票会改变去向，因此 F-50 将冲正统一为：银行或现金腿全额反向，往来与预收预付腿按锁后当前可追溯去向拆分；原凭证仍不可修改。

#### 4.2.1 到款与付款冲正

对原款项金额 `R`，计算所有直接来自该款项或经其预收/预付自动核销而形成的当前有效应收/应付核销净额 `S`，以及当前开放且可追溯到该款项的预收/预付 `V`。存在尚未冲正的下游退款/返款时先拒绝；否则必须满足 `R = S + V`，不等即整笔失败并报告追溯差额。

1. 对 `S` 涉及的全部核销根追加 `RELEASE`，每根最多释放当前 `root_net_settled`。
2. `DIRECT_CASH` 根在本路径不新建预收/预付，因为现金正被冲回。
3. `ADVANCE_AUTO` 根先恢复其原预收/预付，再与原本开放的 `V` 一并追加 `source_doc_type = CASH_DOC_REVERSAL` 的 `APPLY` 消耗，最终该原款项不留下开放预收/预付。
4. 客户到款冲正净分录为借应收 `S`、借预收 `V`、贷银行/现金 `R`；供应商付款冲正为借银行/现金 `R`、贷应付 `S`、贷预付 `V`。

例如“票 100 → 收 100 → 红冲 30 → 再冲正原收款”：锁后 `S = 70`、`V = 30`，最终应收 70、预收 0、银行净额 0；不能再释放 100，也不能把应收记成 100。

#### 4.2.2 退款与返款冲正

定义原退款中消耗预收/预付的金额为 `Y`、释放应收/应付核销的金额为 `X`：

1. 在对应预收/预付核销关系上追加 `RELEASE`，先恢复 `Y`。
2. 对每个正向主条目初始化递减容量 `capacity = locked_effective_open`，把该退款在此条目上的原 `RELEASE` 按 `settled_at DESC, id DESC` 排序；逐行取 `A_i = min(该 RELEASE 尚未反向的金额, capacity)`，追加引用原 `RELEASE` 的 `APPLY`，随后令 `capacity = capacity - A_i`。不得让每行各自读取同一个初始容量。
3. 每行剩余 `E_i = 原 RELEASE 尚未反向的金额 - A_i` 已无可核销的原票余额：原根为 `DIRECT_CASH` 时新增预收/预付，原根为 `ADVANCE_AUTO` 时恢复原预收/预付；二者都保留本次冲正单、原退款释放行与资金根的追溯。令 `A = sum(A_i)`、`E = sum(E_i)`。
4. 完成上述 `A` 的追加并在同一事务内重算 `effective_open` 后，对恢复或转入预收/预付的资金池 `Q = Y + E` 立即执行既有 `ADVANCE_AUTO` 候选与排序规则，向此时锁后的当前有效应收/应付自动核销 `Z`；只把 `V = Q - Z` 留作开放预收/预付。这些自动核销只写台账效果，不得另发第二张凭证，全部会计效果由第 4.2.3 节的一张净冲正凭证承载。
5. 必须满足 `X = A + E`、`Q = Y + E = Z + V`，从而 `Y + X = A + Z + V`；事务终态还必须全部 `effective_open >= 0`、`advance_open >= 0`。

客户退款冲正净分录为借银行/现金 `Y + X`、贷应收 `A + Z`、贷预收 `V`；供应商返款冲正为借应付 `A + Z`、借预付 `V`、贷银行/现金 `Y + X`。

两个顺序反例据此闭合：

- “票 100 → 收 100 → 退 30 → 全红 100 → 再冲正退款 30”：`A = 0`、`E = 30`、`Z = 0`、`V = 30`，最终应收 0、预收 100、银行净额 100。
- “预收 100 → 退 30 → 新开票 100 并自动核销 70 → 再冲正退款 30”：恢复的 `Y = 30` 立即自动核销，`Z = 30`、`V = 0`，最终应收 0、预收 0。

#### 4.2.3 finance 与 ledger 的封闭契约

finance 在取得第 10 节规定的全部锁后，计算唯一命令 `CashReversalPostingSplit { ar_ap_amount, advance_amount }`：到款/付款冲正取 `{S, V}`，退款/返款冲正取 `{A + Z, V}`。HTTP、Excel、插件与人工界面均不得提交这两个金额或科目；只有 finance 用例能构造该类型。

ledger 的 `post_reversal` 只接受原资金凭证、本次资金冲正单和上述封闭拆分，校验两金额非负、合计等于原资金腿、来源类型属于到款/付款/退款，按来源方向选择固定科目角色并原子生成一张冲正凭证。冲正产生的全部核销效果行、预收预付效果行与凭证共用同一次期间解析，记入本次冲正实际 `accounting_period_id`，原行期间不修改。finance 同事务写完台账后重跑受影响的应收/应付、预收/预付与资金勾稽，差额非零则整体回滚。该语义不新增凭证来源类别、不允许自由分录，并明确取代 F-10 B-3 与阶段 9 `post_reversal` 的“必然逐行取反”旧定义。

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
- 每行税额与金额先经过项目唯一的 F-03/U-D-05 税额校验与舍入策略；除第 6.4 节明列的纯税额结构性特例外，F-50 不另造第二套舍入模式或容差。行值确定后，头表按定标值直接求和，不二次舍入。
- 写请求与 Excel 模板只包含行级金额；服务端从行计算头表金额并在响应中返回。若旧客户端仍提交头表三项金额，按契约或模板版本不匹配拒绝，不得静默忽略。

登记服务在同一事务中写头与行并重读断言三项合计。数据库对每行实施金额 CHECK；跨头行合计由受控写入口、事务末校验和对账自检共同保证，不声称普通 CHECK 可以跨表求和。

### 6.2 销项发票

新增 `invoice.sales_invoice_lines`，除公共列外的业务列固定为：`sales_invoice_id`、`line_no`、`sales_order_id`、`sales_order_line_id`、`item_kind`、`item_id`、`uom_code`、`quantity`、`net_unit_price`、`tax_rate`、`net_amount`、`tax_amount`、`gross_amount`。发票头的开票内容可保留作整体摘要，行级商品或服务内容以行表为准。

### 6.3 进项发票

现有 `invoice.purchase_invoice_lines` 增加 `tax_rate` 与 `gross_amount`；`purchase_invoices` 删除头表 `tax_rate`。存货类与直接费用类仍按既有 `cost_kind` 分支，不因多税率改变成本归集边界。

进项、销项都允许多行、多税率。0% 税率是现有出厂税率档，税额必须允许为零。

### 6.4 作废与红字冲销

冲销头 `invoice.invoice_reversals` 不再使用无法同时建立两套真实外键的单列 `source_invoice_id`，改为 `source_sales_invoice_id` 与 `source_purchase_invoice_id` 两列；`direction = OUTPUT` 时前者非空、后者为空，`direction = INPUT` 时相反，由 XOR CHECK 与两套 `(legal_entity_id, source_*_invoice_id)` 复合外键共同保证。

新增统一的 `invoice.invoice_reversal_lines`。除 `invoice_reversal_id` 外，它冗余保存 `source_sales_invoice_id`、`source_purchase_invoice_id`，并分别配 `source_sales_invoice_line_id`、`source_purchase_invoice_line_id`；按冲销方向两组恰有一组同时非空。销项与进项原行表分别建立 `(legal_entity_id, sales_invoice_id, id)`、`(legal_entity_id, purchase_invoice_id, id)` 候选唯一键，冲销行再以对应三列 `MATCH FULL` 复合外键真实指向原行，保证活动组全填、非活动组全空，禁止部分 NULL 跳过外键；延迟约束触发器保证行上的原票 id 与 `invoice_reversals` 头所指原票逐字相同，从数据库层拒绝同法人错票挂行。

每个冲销行记录 `quantity_effect_kind = REDUCE | NONE`、`pricing_effect_kind = ORIGINAL_UNIT_PRICE | ADJUSTED`、本次冲销数量、`tax_rate`、不含税金额、税额与价税合计，税率必须等于所指原行税率。`invoice_reversal_id`、两个 effect kind、数量、税率及三项金额全部 `NOT NULL`；方向、来源 id 组、effect kind、数量与金额的组合 CHECK 必须逐项显式判断 NULL，不能让 SQL `UNKNOWN` 绕过约束：

- `REDUCE + ORIGINAL_UNIT_PRICE`：数量大于零，非末次金额按原行 `net_unit_price` 与项目唯一税额策略计算。只有该来源行此前从未出现 `pricing_effect_kind = ADJUSTED`，且本次按同一策略计算后恰好耗尽剩余数量时，才允许三项金额取“原行定标值减此前全部 `ORIGINAL_UNIT_PRICE` 冲销定标值”的确定性剩余额以吸收末次舍入尾差；该值必须等于服务端按唯一策略重算的预期尾差，不接受调用方自报。
- `REDUCE + ADJUSTED`：数量大于零，但金额还包含折让或价格更正；数量与金额分别占用各自剩余额度。
- `NONE + ADJUSTED`：数量为零，用于纯折让、金额或税额更正。
- `NONE + ORIGINAL_UNIT_PRICE`：无业务意义，CHECK 直接拒绝。

所有组合均要求不含税金额与税额大于等于零且至少一项大于零、价税合计大于零且等于前两项之和，因此纯税额更正可表达。普通红字行继续执行 F-03/U-D-05 的 `net × rate` 税额校验；仅 `NONE + ADJUSTED` 且 `net_amount = 0, tax_amount > 0` 的纯税额更正为结构性特例：税率仍复制原行、金额仍按项目唯一小数位与舍入规则定标、税额不得超过该原行的 `remaining_tax`，但不执行以零净额乘税率为基准的容差式。F-03/U-D-05 的回写必须显式登记这项特例，不能让通用容差校验把它误拒，也不能借此放宽其他行。冲销头三项金额由冲销行汇总，删除头表 `red_tax_rate`。

- 销项作废：系统按原销项发票全部行生成全额内部冲销行；仍只允许全额一次。
- 销项红票、进项红票：登记实际红字行，允许分次部分冲销。
- 每一原行分别计算 `remaining_quantity`、`remaining_net`、`remaining_tax`、`remaining_gross`，等于原值减已登记冲销行对应合计，四者都不得为负。
- 销项状态固定为 `ISSUED`、`PARTIALLY_RED_REVERSED`、`VOIDED`、`RED_REVERSED`；进项状态固定为 `REGISTERED`、`PARTIALLY_REVERSED`、`REVERSED`。除 `VOID` 外，全部原行三项剩余金额均为零时才进入全额红冲终态；已有任一冲销行但尚未全部归零时为部分红冲。状态在锁后推导并推进，终态不可回退。
- 红冲累计的权威来源只有已登记的冲销行聚合；不在蓝票头或蓝票行另存 `reversed_*` 权威列，既有规划中的同名缓存列全部撤销。
- 删除 `purchase_invoices.is_credit_note`，进项红票只走统一冲销入口。
- 删除不能表达多次部分红冲的 `purchase_invoices.reversed_by_id` 与“每张原票只能有一张冲销单”的唯一约束。
- 进项引用全在 `invoice` schema 内，不再错误指向 `procure`。

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

`legal_entity_id`、`invoice_medium`、`number_scheme`、`invoice_no`、`owner_type` 与 `owner_id` 全部 `NOT NULL`；`identifier_key` 为生成列，其结果也声明 `NOT NULL`。`UNIFIED_20` 的 CHECK 必须显式要求 `invoice_code IS NULL` 且 `invoice_no IS NOT NULL` 并匹配 20 位 ASCII 数字；`LEGACY_CODE_NUMBER` 必须显式要求 `invoice_code IS NOT NULL`、`invoice_no IS NOT NULL` 并分别匹配 10/12 位与 8 位 ASCII 数字。所有制式 CHECK 都写成 NULL-safe 条件，不依赖 SQL `UNKNOWN` 代替拒绝。

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

合同终止的影响面目录仍恰为七类业务对象，凭证不是第八类人工处置对象。合同终止产生的红冲、退货、退款等源业务动作自动生成后续凭证；资金冲正只纠正资金登记错误，更正凭证只纠正会计重分类，二者都不因合同终止自动产生。

回写必须覆盖 F-49 已定位的五处旧封闭句，以及同义残留：

- 规格第 8 章、第 19 章、第 22 章相关句；
- PRD 合同影响面与状态机相关句；
- PRD 到款更正、总账、诚实披露及附录乙；
- 阶段 9 的更正凭证两张表、`post_correction`、`CORR` 类型码、RLS、审计、端点和测试；
- 阶段 10 的资金冲正称谓与引用。

历史审计快照可以保留旧引文，但必须明确标注为历史，不得被当前计划引用。

## 9. 历史成交资料

历史成交默认只排除已经没有可供参考之成交部分的记录：已作废、已全额红冲、已全额退货。这里的“成交部分”是商品或服务数量与金额语义，不是财务台账的 `effective_open`。部分红冲与部分退货仍有有效部分，默认展示并明确标记状态，不得被“已红冲/已退货”筛选整体排除。

`TradeHistoryItem` 分开增加 `is_visible_by_default` 与 `is_selectable_as_price_source`，禁止用一个布尔值同时承担展示与取价资格：

| 当前事实 | 默认可见 | 可直接作为价格来源 |
|---|---:|---:|
| 未失效成交 | true | true |
| 仅按原单价减少数量的部分红冲、部分退货 | true | true |
| 含 `ADJUSTED` 的折让、价格或税额更正型部分红冲 | true | false |
| 已作废、已全额红冲、已全额退货 | false | false |

四个业务模块提供者负责按每个来源成交行的剩余数量、剩余金额、业务状态，以及冲销行的 `quantity_effect_kind` 与 `pricing_effect_kind` 映射这两个布尔值，MDM 聚合器不解析其他模块的状态字符串。只要该来源行出现任一仍有效的 `pricing_effect_kind = ADJUSTED` 更正，就不得直接作为价格来源；仅 `REDUCE + ORIGINAL_UNIT_PRICE` 的部分减少仍可沿用原单价。配置项统一为 `EP__MDM__TRADE_HISTORY__INCLUDE_INEFFECTIVE`，默认 `false`；打开后只改变终态记录的可见性，绝不把不可选记录变成可选。

部分红冲记录继续展示原成交数量、原单价与当前状态，不派生可能产生歧义的“净单价”。操作者显式选用记录时，服务端必须重新读取当前业务状态并重算 `is_selectable_as_price_source`，再执行当前价格权限与折扣校验，不能信任列表快照或客户端传回的布尔值；查询后才变为全红或发生金额型更正时必须拒绝选用。

## 10. 事务、并发与错误处理

1. 需要法定号码时，发票头、行、号码登记、应收应付条目、核销关系与总账凭证在同一数据库事务中完成；`VOID` 不写号码登记。
2. 所有资金与发票事务使用同一跨表锁序：原款项/退款单 → 原发票头 → 原发票行 → 应收/应付正向主条目 → 预收/预付条目 → 核销根 → 核销效果行 → 冲销行与累计。实现先收集全部候选对象 id，再按上述类别及每类 `id ASC` 统一加锁，锁后重载并校验依赖集合；若并发写入使相关对象集合发生变化，则整笔重试，不得在锁到一半时临时追加逆序对象。全部锁定后才按业务所需的 LIFO、容量与金额公式计算。
3. 号码占用依赖数据库唯一约束；唯一冲突统一映射为业务错误，不把另一张无权单据的属性写入响应。
4. 本事务内发现 `effective_open < 0`、`advance_open < 0`、根分组净核销越界、释放分配合计不等于 `L`、资金冲正拆分不守恒、头行金额不等或本次触及的子账总账勾稽不平时，整笔业务事务回滚，不静默截断。
5. 每日对账或关账重算发现既有历史期间切片不平时，新增差异事项、触发告警并阻断关账；不得声称能回滚已经提交的历史业务事务，也不得自动生成调账分录。
6. 已过账凭证、核销关系、冲销条目与号码登记均只追加或受限状态推进，不覆盖历史金额。

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
12. 任一根分组 `sum(APPLY) - sum(RELEASE)` 小于 0 或大于根金额、直接反向子行超出父行、或派生行跨条目/跨根：数据库或用例层拒绝。
13. 原到款/付款仍存在未冲正的下游退款/返款，或可追溯 `S + V` 不等于原金额：资金冲正拒绝并列出依赖或差额。

### 11.2 历史期间

14. `MIGRATION_OPENING` 应收、应付各 100：当前视图、账龄和首期历史切片均为 100，且可被正常核销；其 `C = 0`。
15. `MIGRATION_OPENING` 预收、预付各 100：`advance_open = 100`；核销 40 后为 60，释放 10 后为 70，始终满足上下界。
16. M1 开票 100、M2 收款 60、M3 退款 20：三期应收切片依次为 100、40、60；重跑 M1、M2 不受 M3 影响。另测 M1 开票 100、M2 收款 100、M3 红冲 30、M4 冲正原收款：四期应收依次为 100、0、0、70，预收依次为 0、0、30、0。
17. M1 预收 100、M2 自动核销 70、M3 释放 20：三期预收切片依次为 100、30、50；重跑早期结果不变。
18. 最新期间累计值分别等于当前应收 `sum(effective_open)`、应付 `sum(effective_open)`、预收 `sum(advance_open)` 与预付 `sum(advance_open)`；同一红冲数据经信用敞口、核销候选与上限、付款申请上限、客户/供应商门户、账龄、报表、Excel 和对账查询读取时逐项一致，不出现第二套经营余额。
19. 顺延事件只进入实际 `accounting_period_id` 对应切片，`business_date` 只影响检索和账龄。
20. 每日或关账重算故意制造历史差额时，生成差异事项并阻断关账，不改写历史业务行、不自动调账。

### 11.3 发票

21. 一张销项票含 13% 与 6% 两行，头表三项金额精确等于行合计，头表不存在税率且请求提交头表金额会被契约拒绝。
22. 一张进项票含 13% 与 0% 两行可登记，0% 行税额为 0；进项头同样没有税率。
23. `REDUCE + ORIGINAL_UNIT_PRICE` 严格按原单价计量且仅在此前无 `ADJUSTED` 时吸收可重算的末次尾差；先登记任一 `ADJUSTED` 后再把非尾差剩额伪装成末次原价行必须拒绝并要求改记 `ADJUSTED`。`REDUCE + ADJUSTED` 与 `NONE + ADJUSTED` 可表达折让或价格更正；`NONE + ORIGINAL_UNIT_PRICE` 被 CHECK 拒绝。纯税额行 `net = 0, tax > 0` 只走第 6.4 节特例，仍按原行税率、剩余税额上限和项目唯一小数位校验；绕过应用层把任一 effect kind、数量、税率或金额写成 NULL 也必须由数据库拒绝。
24. 只红冲一行后原票进入部分红冲；第二次可继续冲，但每个来源行的累计数量及三项金额分别不得超出原行；只有所有来源行三项金额都归零才进入全额红冲终态。
25. 两笔并发部分红冲单笔都合法但合计超额时只能一笔成功；`VOID`、红字冲销、退款与相关资金冲正并发时只能落成某一合法串行次序，且所有台账与总账勾稽差额为零，使用真实 PostgreSQL 验证锁序及集合变化重试。
26. 跨法人原票、跨法人原行、同法人但冲销行不属于冲销头原票、以及方向与两组 FK 不一致，绕过应用层直写数据库时也必须被复合 FK、XOR CHECK 或延迟约束触发器拒绝。
27. 20 位号码跨销项蓝票、进项蓝票、销项红票、进项红票重复时拒绝；并发登记只能一个事务成功。
28. 旧制号码相同但代码不同允许，代码与号码均相同时跨表拒绝；前导零完整保留。
29. `identifier_key` 只能由数据库生成；对必填键直写 NULL、制式与代码/号码不匹配、用 NULL 绕过唯一键均被 `NOT NULL` 或 NULL-safe CHECK 拒绝，registry owner 与业务头错配被延迟约束拒绝。
30. `VOID` 不生成号码登记；同一完整标识在另一法人可登记。
31. 普通用户不能直接查询号码登记表；无权用户收到的重复提示不泄露原记录，授权用户才得到业务单据链接。

### 11.4 更正入口与历史成交

32. 发票红冲、总账重分类、到款/付款/退款冲正三类入口的正向用例各自成功，源业务单据与来源凭证完整。
33. 跳过退货/终止源单据直接改票、用资金冲正冒充发票红冲、非资金单据提交资金冲正、无来源自由分录均被拒绝。
34. 原业务单据与原凭证在三类更正后逐字段不变，只新增可追溯记录；动态资金冲正仍只有资金冲正这一来源类别。
35. 四个 `TradeHistoryProvider` 对未失效、`REDUCE + ORIGINAL_UNIT_PRICE` 部分减少、任一 `ADJUSTED` 部分更正、全额失效四组状态，按来源行剩余数量/金额输出与第 9 节矩阵逐项一致。
36. 历史成交默认展示原单价数量型及金额调整型部分红冲并标明状态；前者仍可作为价格来源，任一含 `ADJUSTED` 的来源行可见但不可作为价格来源。
37. 查询时可选、提交选用前已全红或发生金额型更正：服务端重读后拒绝，不信任旧列表快照。
38. 打开 `EP__MDM__TRADE_HISTORY__INCLUDE_INEFFECTIVE` 后终态记录可见，但 `is_selectable_as_price_source` 仍为 false。

所有负向用例必须断言 `docs/error-codes.md` 中登记的精确错误码；只断言 4xx、数据库异常或“失败了”均不算通过。涉及并发、RLS、复合外键、生成列、延迟约束与期间切片的用例一律使用真实 PostgreSQL，不以内存替身代替。

## 12. 回写边界与完成定义

书面设计确认后，实施计划必须逐条列出以下规范性文件的修改，不得只改其中一层：

- `docs/superpowers/plans/2026-08-10-first-release-dev-plan/00c-gap-ruling.md`：登记 F-50，把 F-49 九条标为已关闭，并在 F-46 旧 `min` 公式与 F-48 仍含“资金冲正逐行取反”的句旁标注被 F-50 替代。
- `docs/superpowers/specs/2026-08-17-f10-ruling-detail.md`：在 B-2 标注凭证来源已由 F-48 从三类改为四类，在 B-3 至 B-8 的每个被替代句旁逐项标注 F-50，不让旧详本继续充当当前依据。
- `docs/superpowers/specs/2026-07-19-enterprise-private-operations-platform-design.md`：财务规则、发票、强制不变量、阶段验收与诚实披露。
- `docs/superpowers/specs/2026-08-09-first-release-prd.md`：历史成交、财务第 6 节、F-03/U-D-05 状态与纯税额结构性特例、附录乙及术语；只增加特例，不代替小数位、舍入模式与普通行容差待决项。
- `docs/superpowers/plans/2026-08-10-first-release-dev-plan/00-overview.md` 与 `02-data-foundation.md`：登记总数、阶段依赖、全局计数，并同步 T0 对象/迁移/`ep-datagen` 最小样本为“发票头 + 至少一行 + 号码登记”，保证首个可运行切片不再依赖旧单行头模型。
- `docs/superpowers/plans/2026-08-10-first-release-dev-plan/00e-appendix-b-full-sweep.md`、`00g-finance-judgments-needed.md` 与 `00h-b-cluster-blockers.md`：仅对仍会被当前文档引用的旧三来源、旧未决状态与被替代公式追加历史标记，不重写审计快照。
- `docs/superpowers/plans/2026-08-10-first-release-dev-plan/05-master-data.md`：`TradeHistoryItem` 两个资格字段、按来源行聚合规则与配置项。
- `docs/superpowers/plans/2026-08-10-first-release-dev-plan/06-contract-sales.md`：销售历史成交提供者、退货/红冲前置链、价格选用复核及信用敞口查询统一使用 `effective_open`。
- `docs/superpowers/plans/2026-08-10-first-release-dev-plan/07-procurement-portal.md`：采购历史成交提供者、进项引用、付款申请上限、应付查询与供应商门户余额统一使用 `effective_open`。
- `docs/superpowers/plans/2026-08-10-first-release-dev-plan/09-ledger-period.md`：更正凭证入口、`post_reversal` 动态拆分契约、计量项、截至期间的四类子账勾稽、已关闭期间重跑稳定性与阶段计数。
- `docs/superpowers/plans/2026-08-10-first-release-dev-plan/10-ar-ap-invoice.md`：数据表、DTO、端点、算法、当前/历史对账视图、全部查询与 Excel 导出、F-03/U-D-05 纯税额结构性特例、测试和风险。
- `docs/superpowers/plans/2026-08-10-first-release-dev-plan/11-cost-metrics-reporting.md`：账龄及报表由 `open_amount` 改为 `effective_open`、数据集签名与测试。
- `docs/superpowers/plans/2026-08-10-first-release-dev-plan/00f-f10-writeback-order.md`：被 F-48/F-50 替代项与剩余回写状态。
- `docs/data-dictionary.md`、`docs/config-reference.md`、`docs/error-codes.md`：新增枚举、表列、配置键与精确错误码。

完成必须同时满足：

1. 九条在裁定卷、规格、PRD 与阶段计划中均有唯一当前结论。
2. “退款核销后红冲只能拒绝”“释放额取 `min(已核销, 红字金额)`”“资金冲正必然逐行取反”“按 `reverses_id` 推断正负”“凭证只有三个来源”“进项头表单税率”“每张原票只能红冲一次”等旧口径在当前规范性文档中清零；历史引文必须带已替代标记。
3. 规格、PRD 与阶段计划中的表数、迁移数、RLS 数、类型码数、端点数、审计动作数与新增对象一致。
4. 信用敞口、核销候选与上限、付款申请上限、客户/供应商门户、账龄、报表、Excel 和对账查询在同一红冲用例下逐项返回相同 `effective_open`，不得留第二套经营余额；T0 数据生成回归能创建带至少一行及号码登记的最小发票。
5. 文档静态检查、引用检查与 Markdown 表格检查全部通过。

## 13. 非目标

本设计不引入税务平台直连、发票查验、勾选认证、纳税申报、多币种、反结账、手工自由凭证、万能发票物理表或通用事件溯源平台。上述边界仍按首版延期目录执行。

本设计不代替独立的 F-03/U-D-05 税额舍入模式与普通行容差决定；它只规定销项、进项和红字发票共用该项目唯一策略，并冻结第 6.4 节纯税额更正不适用 `net × rate` 容差式这一结构性特例。实施计划除回写该特例外，不得借 F-50 另行发明舍入模式或放宽普通行容差。

本设计也不声称软件规则替代企业财务负责人、税务专业人员或客户所在地主管税务机关的最终判断。编号制式与红字业务规则发生监管变化时，应通过受控版本升级更新验证配置与交付说明。
