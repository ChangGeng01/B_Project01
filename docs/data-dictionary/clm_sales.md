# clm / sales 承重经济图数据字典

> **F-57 状态：`REGISTRY_PENDING_REBASELINE`。** 经济守恒细节可复用；来源、dynamic authz、objective/obligation、generation，以及 `STANDARD`/`DROP_SHIP` 当前认证边界须先按 F-57 再基线。
>
> **激活/owner task：Task 3。** 本分册目前不是 F-57 实现权威；Task 3 完成再基线并显式激活前，只能作为历史字段与不变量输入。

历史状态（F-57 下无效）：本分册曾与总册、阶段 6 计划及迁移目录共同构成开发前冻结契约。这里只登记合同到销售履约链上承担金额、来源版本和累计交付守恒的承重列；其余列曾采用阶段 6 第 3 节。所有表的旧模型均带总册公共列、`ENABLE/FORCE RLS`、同法人候选键和 `ON DELETE RESTRICT` 真实外键。

## 1. `clm.contracts`

单据类。承重列为 `status`、`customer_id`、`version_no int not null`、`total_amount numeric(18,2) not null`、`total_amount_with_tax numeric(18,2) not null`。非 DRAFT 至少一条行；两项头额分别等于行额合计。

## 2. `clm.contract_lines`

| 列 | 类型 | 可空 | 语义 |
|---|---|---|---|
| contract_id | uuid | 否 | 合同头；候选键 `(legal_entity_id,contract_id,id)` |
| line_no | int | 否 | 合同内唯一行号 |
| quantity | numeric(18,6) | 否 | 正数 |
| net_unit_price | numeric(18,6) | 否 | 折后冻结单价 |
| is_tax_included | boolean | 否 | 单价是否含税，不得由下游猜测 |
| tax_rate | numeric(9,6) | 否 | 冻结税率 |
| line_amount | numeric(18,2) | 否 | 第 10 节累计净额函数在整行数量处的值 |
| line_amount_with_tax | numeric(18,2) | 否 | 第 10 节累计含税函数在整行数量处的值 |
| order_type | text | 否 | STANDARD/DROP_SHIP/CONSIGNMENT/SUBSCRIPTION/LEASE（F-65 按 F-57 业务执行契约 `:272`（该行属 §4）：历史 `NORMAL` 只允许在一次性再基线迁移中映射为 `STANDARD`，**不得作为现行数据库枚举**；本行原列 NORMAL） |
| cycle_unit/cycle_length | text/int | 是 | 订阅或租赁周期快照 |
| lease_from/lease_to | date/date | 是 | 租赁区间快照 |
| auto_renew | boolean | 否 | 自动续期快照 |

同一非 DRAFT 合同的 order/cycle/lease/auto_renew 逐值相同，因为一个合同版本只派生一张销售订单。

## 3. `clm.contract_payment_schedules`

ACTIVE 行只允许一种 basis。RATIO 行只有 ratio，合计精确等于 `1.000000`；AMOUNT 行只有 `amount/amount_with_tax`，两项合计分别等于合同头净额/含税额。VOIDED 行必须有 reason 且不计合计。PENDING_APPROVAL 及以后由延迟图强制闭合。

## 4. `clm.contract_versions`

仅追加。候选键 `(legal_entity_id,contract_id,version_no)`。`snapshot jsonb` 使用固定对象形状：`header`、按 line_no 排序的 `lines`、`terms`、`milestones`、`payment_schedules`、`attachments`；line 元素必须含 id、item/costing/material/uom/quantity/net_unit_price/is_tax_included/tax_rate/两项行额/交期/仓库/order/cycle/lease/auto_renew。`BEFORE INSERT` 守卫锁定并逐字段复核插入时权威表，之后 UPDATE/DELETE 永久拒绝；非 DRAFT 合同的当前 `contracts.version_no` 必须恰有一份仍与提交点权威头/行/期次相等的快照，历史快照不与当前行重比。

## 5. `sales.sales_orders`

单据类。`source_contract_id/source_contract_version_no` 以复合 FK 指向 `clm.contract_versions`；`customer_id`、order/cycle/lease/auto_renew 与两项头额必须等于该来源版本快照及订单行和。

## 6. `sales.sales_order_lines`

| 列 | 类型 | 可空 | 语义 |
|---|---|---|---|
| sales_order_id | uuid | 否 | 订单头 |
| source_contract_id | uuid | 否 | 必须等于订单头 |
| source_contract_version_no | int | 否 | 必须等于订单头 |
| source_contract_line_id | uuid | 否 | 长 FK 指向同合同 current line；延迟图再证明其在冻结版本 lines 中恰出现一次 |
| quantity | numeric(18,6) | 否 | 来源版本冻结量 |
| net_unit_price | numeric(18,6) | 否 | 来源版本冻结价 |
| is_tax_included | boolean | 否 | 来源版本冻结录入口径 |
| tax_rate | numeric(9,6) | 否 | 来源版本冻结税率 |
| line_amount/line_amount_with_tax | numeric(18,2) | 否 | 等于来源版本行额 |
| delivered_quantity | numeric(18,6) | 否 | 已确认交付事实之和 |
| open_amount_with_tax | numeric(18,2) | 否 | 非终态为行含税额减已确认交付含税额；CLOSED/CANCELLED 为 0 |

## 7. `sales.delivery_schedules`

每个订单行至少一条；`SUM(quantity)=sales_order_lines.quantity`。`delivered_quantity` 等于指向本 schedule 的 CONFIRMED delivery line 数量和。PENDING 允许未交完，DELIVERED 当且仅当累计量等于 quantity；CLOSED/CANCELLED 保留历史量。

## 8. `sales.delivery_confirmation_lines`

| 列 | 类型 | 可空 | 语义 |
|---|---|---|---|
| sales_order_line_id/delivery_schedule_id | uuid/uuid | 否 | 长复合 FK 证明同一订单头行分批 |
| quantity | numeric(18,6) | 否 | 本次确认量 |
| net_unit_price | numeric(18,6) | 否 | 等于订单行 |
| is_tax_included | boolean | 否 | 等于订单行 |
| tax_rate | numeric(9,6) | 否 | 等于订单行 |
| allocation_quantity_before | numeric(18,6) | 是 | DRAFT 为空；CONFIRMED 为同订单行锁后既有确认累计量 |
| line_amount/line_amount_with_tax | numeric(18,2) | 否 | 分别为 `cum(B+q)-cum(B)` |

同一订单行的 CONFIRMED 区间从 0 连续无洞无重叠，末端等于订单行 delivered_quantity。`sales.assert_sales_order_economic_graph_consistent()` 与交付图共同覆盖订单、行、分批、确认头和确认行的双向变化。

## 9. `sales.sales_return_lines`

`net_unit_price/is_tax_included/tax_rate` 逐值等于原订单行；退货金额按实际交付确认行累计区间分配，不按当前价格重算。

## 10. 唯一累计金额函数

税外价：`cum_net(x)=round2(x*net_unit_price)`，`cum_gross(x)=round2(cum_net(x)*(1+tax_rate))`。税内价：`cum_net(x)=round2(x*net_unit_price/(1+tax_rate))`，`cum_gross(x)=round2(x*net_unit_price)`。整行取 x=quantity；任何部分交付固定取区间差，不允许逐段独立舍入。

## 11. 迁移与数据库门禁

- `V20261023092700__clm_harden_contract_economic_graph.sql`：四表 DEFERRABLE INITIALLY DEFERRED 合同经济图。
- `V20261023092800__sales_harden_order_delivery_economic_graph.sql`：五表 DEFERRABLE INITIALLY DEFERRED 来源版本、分批、确认累计与 open amount 图。
- 两图的触发源覆盖父表、子表与反向事实表；所有普通 FK 命中但经济关系错误的负例必须在 COMMIT 失败。
