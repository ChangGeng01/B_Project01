# 事件目录

本文件是全部领域事件类型的唯一登记处。新增事件必须先登记再实现：先在本文件加行，再写产生该事件的代码。写入侧对未登记的 `event_type` 一律拒绝，不静默放行。

## 1. 事件类型命名

四段，形如 `<module>.<aggregate>.<past_participle>.v<major>`，全小写点分。例：`clm.contract.effective.v1`、`sales.delivery.confirmed.v1`、`inventory.receipt.posted.v1`、`ledger.voucher.posted.v1`、`finance.payment.registered.v1`。

三条硬约束：

- 事件名一律用已完成时态，禁止用命令式动词。`order.create` 是命令不是事件，`sales.order.created.v1` 才是。
- 模块段取技术基线第 1.2 节的模块码，或平台事件的 `platform`。
- 版本段是主版本。破坏性变更即删除或重命名字段、收紧校验、改变默认值语义、删除枚举取值，必须升主版本并作为新的一行登记，旧行保留。

## 2. 信封

载荷结构固定为信封加业务体，信封字段不得增删。任何事件的差异只在 `payload` 内。

```json
{
  "event_id": "0192f3a1-...-uuidv7",
  "event_type": "sales.delivery.confirmed.v1",
  "event_version": 1,
  "occurred_at": "2026-08-10T02:11:43.512Z",
  "legal_entity_id": "…",
  "aggregate_type": "sales.delivery_confirmations",
  "aggregate_id": "…",
  "aggregate_version": 7,
  "security_level": 20,
  "data_scope_tags": ["dept:sales", "project:P-2026-0007"],
  "posting_date": "2026-08-10",
  "accounting_period_id": "…",
  "correlation_id": "…",
  "causation_id": "…",
  "idempotency_key": "…",
  "actor": { "user_id": "…", "device_id": "…", "on_behalf_of": null },
  "payload": { }
}
```

三处形态与代码共用同一份编解码，不得各自实现：

- `security_level` 取 10、20、30、40 四值并序列化为数字，形态见 `crates/foundation/src/security/level.rs` 的 `SecurityLevel`；未知取值反序列化失败，不静默降级。
- `data_scope_tags` 的元素形态为 `<kind>:<value>`，kind 取 `[a-z0-9_-]`，value 取 `[A-Za-z0-9_-]`，总长上限 128，形态见同目录 `context.rs` 的 `DataScopeTag`。该形态与公共列 `data_scope_tags text[]` 的元素形态同源。
- `aggregate_type` 为 `<module>.<table>` 的小写下划线形态，与 `RecordShare::object_type` 同形。

`posting_date` 与 `accounting_period_id` 两项对不涉及过账的事件取空值，取空值不等于可以省略字段。

## 3. 登记表的组织方式

按模块段分节，节的顺序取技术基线第 1.2 节的模块码顺序，`platform` 段在最前。段名与下表逐字一致，`ModuleCode` 的 15 项取值见 `crates/foundation/src/module.rs`：

`platform`、`mdm`、`crm`、`cpq`、`clm`、`sales`、`procure`、`inventory`、`costing`、`project`、`service`、`finance`、`ledger`、`invoice`、`portal`、`reporting`。

每段一张表，列固定为：事件类型、聚合类型、触发时点、`payload` 关键字段、消费者、登记阶段。消费者一列写「暂无」时必须同时写明预期消费方所属阶段，不允许只写「暂无」。

## 4. 阶段 1 的登记内容

阶段 1 登记的事件数为 0。理由不是遗漏：阶段 1 不写任何 Outbox 条目，不消费任何事件，也不为消费预留钩子（阶段 1 计划第 7.7 节）；本阶段没有任何聚合，也没有任何消费者，登记一个无人产生也无人消费的事件只有维护成本没有判据。

因此本文件在阶段 1 结束时的状态是：命名规范、信封形态、组织方式与维护纪律四项已定，登记表为空。这是本文件的正常交付状态，不是待补内容。

后续阶段按其计划的交付物清单向对应段追加行，每一行都要同时满足本文件第 1 节的命名约束与第 2 节的信封约束。

## 5. 机器判定

- `xtask eventcatalog` 校验本文件与代码中事件类型登记的一致性。该子命令在阶段 1 尚未实现，当前以退出码 70 明确报「本阶段未交付」，不静默返回 0。
- 写入侧的运行期判定属阶段 3a：信封字段缺任一必填项即拒绝，`event_type` 未在本目录登记即拒绝。
- 启动自检项 `event-catalog-consistent` 由阶段 3b 注册，届时把本文件与运行期登记面的一致性变成启动时可判定的项。

## 6. 维护纪律

- 一个事件类型只能由一个阶段登记，重复登记即构建失败。
- 已登记的事件类型不得改名。语义变化时升主版本并新增一行，旧行保留并标注其停止产生的版本。
- 事件是对外契约。删除一个事件类型与删除一个 API 端点同级，须按破坏性变更处理。
