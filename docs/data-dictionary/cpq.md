# CPQ 数据字典（阶段 5–6 开发前冻结）

> **F-57 状态：`REGISTRY_PENDING_REBASELINE`。** 旧阶段冻结不再是执行入口；字段和定价不变量仅在与 F-57 动态权限、来源追踪和 generation 不冲突时复用。
>
> **激活/owner task：Task 3。** 本分册目前不是 F-57 实现权威；Task 3 完成再基线并显式激活前不得据此实施。

历史状态（F-57 下无效）：曾标为开发前契约，尚未执行迁移。阶段 5 计划 §3.4 与阶段 6 的 `cpq.price_authorities` 表定义只保留为旧逐列来源；F-57 再基线首次落地时必须由数据库元数据生成逐列表格并在同一变更中替换本状态说明。

## 对象清单

- `cpq.price_lists`：价目表档案，状态 `DRAFT|PENDING_APPROVAL|EFFECTIVE|EXPIRED|VOID`，类型码 `PRLS`。
- `cpq.price_list_lines`：产品、计量单位、含税标记、单价、底价与启用槽位。
- `cpq.price_list_customer_links`：指定客户范围与启用槽位。
- `cpq.price_authorities`：阶段 6 价格权限档案；主体层级只允许 `USER|POSITION|ROLE`，判定顺序固定 USER → POSITION → ROLE，三级均无命中时拒绝提交。

## 冻结规则

- 四张表全部带 `legal_entity_id` 并启用、强制 RLS。
- price list 到 lines/customer links 的同 schema 引用使用真实复合外键；line 的 `product_id/uom_id` 与 customer link 的 `customer_id` 分别以 `(legal_entity_id,<ref>)` 指向 `mdm.products/mdm.uoms/mdm.customers(legal_entity_id,id)` 的真实复合外键，均 `ON DELETE RESTRICT`。`MasterDataLookup` 仍在调用方事务内校验启用状态与业务范围，不替代外键。
- 活跃明细和客户范围都使用 NULL 槽位加普通唯一索引，不使用部分索引或函数索引。
- `floor_price` 为空或不高于 `unit_price`；批量取价单次最多 200 行，多命中必须显式返回全部命中供选择，不暗选一条。
- 价格权限不复制价目表金额，只保存主体、适用范围与可用价目表关系；不得建立第二套用户特价表。

## 编号类型码

`PRLS` 已在总册 §5.1 登记，按档案编码形态取号。

## 一致性判据

首次迁移落地及以后每次结构变更都必须同时满足数据库元数据、阶段 5 §3.4、阶段 6 的价格权限表定义和本分册逐项一致；固定单目标引用的未落实真实外键数、法人列遗漏数与无候选键目标数必须均为零。
