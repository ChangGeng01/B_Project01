# 影响面规则目录

> **F-67 注**：本文的「阶段退出点」列（3/6/7/10/12）、「实现属主」列（`ep-app-*`）与人工角色列均按旧十四阶段与旧模块口径写成——F-57 下这些退出点不再发生、`ep-app-*` 别名被业务契约 §0.2 禁作 owner、固定岗位由 RoleCode 模板种子取代；**本文在 G0 生成 impact-catalog.v1.json 时只作语义种子导入，上述三列须映射到 F-57 语义，不得照字面接线**。第 :66 的「阶段退出时注册数」验收在 F-57 下无触发时点，同此处置。

> **F-57 状态：`REGISTRY_PENDING_REBASELINE`。** 旧七条合同终止规则只作可复用种子，不再是完整影响面闭集；F-57 的变更传播、objective 重开、退货/退款/撤回/权限/包/配置代影响必须在实施计划 Task 1 登记后执行。

本文件是 `ep-platform-impact` 编译期目录常量的唯一文档登记源。首版目录固定为七条，且七条的 `upstream_event_type` 均为 `clm.contract.terminated.v1`。`cargo xtask configdoc --check-impact-catalog` 必须逐项比对本文件与代码常量；目录条数恒为 7，不能用尚未接线、空实现、Noop 或直接 DONE 的规则凑注册数。

## 1. 接线与占位规则

| 阶段退出点 | 真实注册数 | 本阶段新增 |
|---|---:|---|
| 阶段 3 | 0 | 平台、两张台账表、目录常量、唯一消费者与人工处置通道 |
| 阶段 6 | 3 | `CLM_TERM_SALES_ORDER_LINE`、`CLM_TERM_MILESTONE`、`CLM_TERM_DELIVERY_CONFIRMATION` |
| 阶段 7 | 4 | `CLM_TERM_PURCHASE_REQUISITION` |
| 阶段 10 | 6 | `CLM_TERM_RECEIPT_PLAN`、`CLM_TERM_SALES_INVOICE` |
| 阶段 12 | 7 | `CLM_TERM_PROJECT_TASK` |

每次建立评估批次时都按本目录七个类别建立项。尚未真实注册的类别建立一条占位项：`target_doc_id`、`target_doc_no`、`target_doc_line_no` 均为空，`state=PENDING`，不计入 `item_done`；对应阶段接线后由真实规则重新 `assess`。返回空集合时，同一目录行以目标三字段为空、`state=DONE`、`outcome_reason=NO_APPLICABLE_TARGET` 闭合并计入 `item_done`；有对象时以实际目标项替换。两种目标为空形状都固定 `attempts=0` 且无租约、错误、流程或人工决定字段；除该终态外，其他 DONE/DEAD/DISPATCHING 行必须有真实 target id。未接线的 PENDING 占位项不得提前 DONE，也不得触发重试或死信。

## 2. 七条唯一规则

| 顺序 | impact_rule_code | 实现属主 / 阶段 | target_module | 目标与 assess 取数 | 初始处置 | 人工角色 | 允许的 decision_code 与 decision_result_doc_id 形状 |
|---:|---|---|---|---|---|---|---|
| 1 | `CLM_TERM_SALES_ORDER_LINE` | `ep-app-sales` / 6 | `SALES` | 同法人、同来源合同且状态为 `OPEN\|PARTIALLY_DELIVERED` 的 `sales.sales_order_lines` | 锁后零交付 `AUTO_CANCEL`，已有交付 `AUTO_CLOSE`；同事务取消其全部 PENDING 分批交付行 | 无 | 无人工决策码；三个决策字段始终为空 |
| 2 | `CLM_TERM_MILESTONE` | `ep-app-clm` / 6 | `CLM` | 同法人、同合同且状态为 `PLANNED\|ACTIVE` 的 `clm.contract_milestones` | `AUTO_CANCEL` | 无 | 无人工决策码；三个决策字段始终为空 |
| 3 | `CLM_TERM_DELIVERY_CONFIRMATION` | `ep-app-sales` / 6 | `SALES` | 同法人、同合同、状态为 `CONFIRMED` 且尚未被有效销售退货全额覆盖的交付确认单 | `MANUAL_DECISION` | `SALES_MANAGER` | `RETURN_REGISTERED`：结果 id 必填，指向同法人、状态 `REGISTERED\|CLOSED` 且经退货行关联本交付确认单的销售退货单；`NO_RETURN`：结果 id 必须为空。两码理由均非空 |
| 4 | `CLM_TERM_PURCHASE_REQUISITION` | `ep-app-procure` / 7 | `PROCURE` | 同法人、同合同三支来源（`CONTRACT`、属于该合同的 `SALES_ORDER`、来源任务属于该合同的 `PROJECT_TASK`）且非 CLOSED 的采购需求 | `PENDING\|PARTIALLY_ORDERED` 为 `AUTO_CLOSE`；`ORDERED` 为 `MANUAL_DECISION` | `PROCURE_MANAGER` | `CLOSE_ORDERED_REQUISITION`：结果 id 必填且严格等于目标 `procure.purchase_requisitions.id`，锁后目标须为 `CLOSED`；`KEEP_ORDERED_REQUISITION`：结果 id 同样必填且等于目标 id，锁后目标仍为 `ORDERED`。两码理由均非空 |
| 5 | `CLM_TERM_RECEIPT_PLAN` | `ep-app-clm` / 10 | `CLM` | 同法人、同合同、状态 `ACTIVE` 且未被有效销项发票占用的合同收款计划期次 | `AUTO_CANCEL`，实际置 `VOIDED` | 无 | 无人工决策码；三个决策字段始终为空 |
| 6 | `CLM_TERM_SALES_INVOICE` | `ep-app-invoice` / 10 | `INVOICE` | 同法人、同合同且状态为 `ISSUED\|PARTIALLY_RED_REVERSED` 的销项发票 | `MANUAL_DECISION` | `FINANCE_MANAGER` | `VOID_SALES_INVOICE`：结果 id 必填，指向 `source_sales_invoice_id=目标`、`direction=OUTPUT`、`reversal_kind=VOID` 的 `invoice.invoice_reversals.id`，目标锁后 `VOIDED`；`RED_LETTER_SALES_INVOICE`：结果 id 必填，指向同目标的 OUTPUT+RED_LETTER 冲销登记且累计已全额红冲；`KEEP_SALES_INVOICE`：结果 id 必须为空，目标仍为 `ISSUED\|PARTIALLY_RED_REVERSED`。三码理由均非空 |
| 7 | `CLM_TERM_PROJECT_TASK` | `ep-app-project` / 12 | `PROJECT` | 同法人、`source_contract_id` 等于该合同且状态为 `NOT_STARTED\|IN_PROGRESS` 的 `project.project_tasks` | `NOT_STARTED` 为 `AUTO_CANCEL`；`IN_PROGRESS` 或锁后漂移到 IN_PROGRESS 为 `MANUAL_DECISION` | `PROJECT_MANAGER` | `PROJECT_TASK_COMPLETED`：结果 id 必填且等于目标任务 id，锁后目标须为 `COMPLETED`；`PROJECT_TASK_CANCELLED`：结果 id 必填且等于目标任务 id，锁后目标须为 `CANCELLED`。两码理由均非空 |

`ImpactRule::assess` 与 `ImpactRule::dispose` 都只能经本模块仓储访问本模块 schema；需要外部事实时只能调用已登记的 `ep-contract-*` 只读端口。`dispose` 必须使用调用方传入的同一个 `&mut dyn Tx` 和 `SecurityContext`，锁后重新验证法人、来源关系、目标关系与当前状态，不得信任 assess 时的快照。目标不存在、异法人、异来源或关系不成立属于受控失败，不能伪装成 `AlreadySatisfied`。

## 3. 人工决策数据契约

唯一命令为：

```rust
pub struct ManualImpactDecision {
    pub decision_code: String,
    pub decision_reason: String,
    pub decision_result_doc_id: Option<Uuid>,
}
```

平台先按本目录验证允许码、清洗后非空理由与结果 id 的必填/必空形状，再由业务规则在同一事务锁定目标并验证具体对象语义。不得解析 `decision_reason` 猜分支，不得只把结论存进流程任务 outcome。人工项合法闭合时，平台先把三字段原值持久化，再置 `state=DONE`；形状或业务语义错误时返回 `PLATFORM.REQUEST.INVALID_PAYLOAD` 或规则既有的受控错误，三字段不落库且项目保持 `PENDING`。

除 `MANUAL_DECISION` 外的所有项，`decision_code`、`decision_reason`、`decision_result_doc_id` 必须同时为空。人工项在 PENDING 时三字段也必须为空；人工项 DONE 时 code、reason 必填，result id 按上表逐码判定。流程任务只是待办与 SLA 载体，不是事实来源，也不替代三字段。

## 4. 统一执行结果与失败语义

```rust
pub enum ImpactDisposeOutcome {
    Completed { reason: String },
    AlreadySatisfied { reason: String },
    NeedsManualDecision { reason: String },
}
```

三个 `reason` 都是非空稳定原因码，只允许清洗后的安全文案进入审计与 API。`Completed` 与 `AlreadySatisfied` 使项进入 DONE；`NeedsManualDecision` 不是失败，不增加 `attempts`、不退避、不进死信，平台在同一事务把项改为 `MANUAL_DECISION/PENDING`、按本目录角色创建或幂等复用一个 `HUMAN_TASK` 并回填 `process_task_id`。

意外基础设施或可重试领域失败才进入八档退避；首次执行加八次重试仍失败时置 DEAD、批次置 FAILED。人工命令的码、理由、结果 id 或锁后状态不合法只保持 PENDING，不消耗重试预算。批次的唯一闭合条件为全部项目 DONE、`item_done=item_total` 且不存在 DEAD；平台才把批次置 DONE，并经合同属主的完成端口推进合同到 TERMINATED、恰一次发布 `clm.contract.termination_completed.v1`。任何一项 PENDING 或 DEAD 都不得闭合。

## 5. 机械验收

1. `cargo xtask configdoc --check-impact-catalog` 对七个 code、顺序、上游事件、目标模块、属主阶段、人工角色、允许决策码与结果 id 形状逐项一致，差异为零。
2. `ImpactRegistry` 拒绝目录外 code 与重复 code；阶段 6/7/10/12 退出时真实注册数分别为 3/4/6/7，阶段 3 为 0。
3. 七条规则各有应命中与不应命中目标；三支采购来源分别有正例。异法人、异来源、错关系与锁后状态漂移均有负例。
4. 四类人工分支逐码覆盖空理由、错码、结果 id 缺失/多余/异对象、目标状态错配；失败时项目仍 PENDING 且三字段为空。
5. 未接线占位项确实存在并保持 PENDING；不得以不存在该行形成恒真测试。注册真实规则后占位展开或以目标为空的 `NO_APPLICABLE_TARGET` DONE 行闭合，不产生重复目标项；后者计入 `item_done`，七类都无适用目标时仍满足 `item_total=item_done=7`。
6. 同一处置项重放三次不重复写业务事实、审计、HUMAN_TASK 或终态事件；`NeedsManualDecision` 不增加 attempts。
7. PENDING、DEAD、人工语义错误三类场景分别阻止批次和合同闭合；合法补齐或记名 replay 后才可继续，终态事件仍恰一次。
8. PostgreSQL 直写负例覆盖目标为空却为 DISPATCHING/DEAD、目标为空 DONE 但原因不等于 `NO_APPLICABLE_TARGET`、两种目标为空形状 attempts 非零或带租约/错误/流程/人工字段、目标非空却伪装目录空终态；全部由 CHECK 拒绝。正例覆盖 PENDING 占位转无适用目标 DONE，并断言 `item_total=item_done` 后才调用来源完成端口。
