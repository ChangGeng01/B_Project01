# 数据字典

本文件是全部业务表与全局码表的唯一登记处。表结构以迁移文件为准，本文件登记的是列语义、取值域与跨阶段共用的码表；两处不一致时以迁移文件为准并同批修正本文件。

## 1. 组织方式

按 schema 分节，每个 schema 一节，节内按表名字典序。每张表一张列表，列固定为：列名、类型、可空、默认、语义。

公共列在第 2 节统一给出，各表不重复列出，只列该表自有的列。表定义处必须写明该表属单据类、档案类、会计相关类还是仅追加类，因为这四类各有附加列约定。

阶段 1 不建任何业务表，因此本文件在阶段 1 结束时没有任何 schema 分节。这是本文件的正常交付状态：阶段 1 的范围边界明写不建任何业务表。已有的实质内容是第 2 至 5 节四项跨阶段共用的约定与码表。

## 2. 公共列

每张业务表必须有下列列，顺序也按此排列。

| 列名 | 类型 | 可空 | 默认 | 语义 |
|---|---|---|---|---|
| id | uuid | 否 | 无，应用侧 UUIDv7 | 主键 |
| legal_entity_id | uuid | 否 | 无 | 法人，RLS 唯一判据，任何跨法人引用禁止 |
| security_level | smallint | 否 | 20 | 密级，取值见第 3 节；字段级密级未赋值时按所属对象取值 |
| data_scope_tags | text[] | 否 | '{}' | 数据范围标签，元素形态见第 4 节 |
| row_version | bigint | 否 | 1 | 乐观锁版本 |
| created_at | timestamptz | 否 | now() | 创建时间，UTC |
| created_by | uuid | 否 | 无 | 创建人用户 ID |
| updated_at | timestamptz | 否 | now() | 最后更新时间，UTC |
| updated_by | uuid | 否 | 无 | 最后更新人用户 ID |

`created_by` 与 `updated_by` 在系统上下文下一律写入 `ep_foundation::principal::SYSTEM_PRINCIPAL_ID`，字面量为 `00000000-0000-7000-8000-000000000001`。该取值同时满足 UUIDv7 的版本位与变体位校验，且不可能与 ID 生成器产出的任何值碰撞，因此不需要另设一个「系统用户」记录来占位。

四类附加列：

- 单据类表另加 `doc_no text not null` 与 `status text not null`，`status` 带 CHECK 约束枚举该单据状态机的全部取值。
- 档案类表另加 `code text not null`、`is_active boolean not null default true`、`deactivated_at timestamptz null`。
- 会计相关表另加 `posting_date date` 或 `business_date date` 与 `accounting_period_id uuid`。
- 仅追加表不带 `row_version`、`updated_at`、`updated_by`；是否带 `reverses_id uuid null` 由该表有无业务冲销或更正语义决定，有的必须带并写明它指向哪张表的哪条记录，没有的一律不得带，不得为满足列约定而保留一个恒为 NULL 的该列。

不设 `tenant_id` 或 `customer_id` 列：客户隔离由「每个客户一个独立事务数据库实例」的部署形态承担，再加一列只会制造第二套隔离口径与两处越权测试面。

附件引用不落在业务表列上，一律经 `<主表单数>_attachments` 关联表，列为 `owner_id`、`attachment_object_id`、`purpose`、`sort_no` 与公共列。

## 3. 密级取值

`security_level` 为 smallint，四级取值如下。代码侧的对应物是 `crates/foundation/src/security/level.rs` 的 `SecurityLevel`，序列化为数字而不是变体名，与本表同源。

| 取值 | 名称 | 变体 |
|---|---|---|
| 10 | 公开 | `Public` |
| 20 | 内部 | `Internal` |
| 30 | 保密 | `Confidential` |
| 40 | 机密 | `Secret` |

未知取值一律反序列化失败，不静默降级为最低级。

## 4. 数据范围标签的元素形态

`data_scope_tags` 的每个元素形如 `<kind>:<value>`，kind 取 `[a-z0-9_-]`，value 取 `[A-Za-z0-9_-]`，总长上限 128，二者均不可为空。例：`dept:sales`、`project:P-2026-0007`。

该形态由 `crates/foundation/src/security/context.rs` 的 `DataScopeTag` 唯一实现，公共列与事件信封的同名字段共用它，两处不得各自编解码。

## 5. 单据类型码

本节是单据类型码与档案类型码的全局唯一登记表，单据类与档案类共用同一张表。类型码全局唯一，新增类型码必须先在本节登记再实现。

### 5.1 登记表

| 类型码 | 名称 | 类别 | 所属模块 | 登记阶段 |
|---|---|---|---|---|

本阶段登记条数为 0。理由不是遗漏：阶段 1 不引入任何单据与任何档案，一个类型码都没有被测对象。各类型码由其单据或档案所在阶段登记，任何阶段不得新增未在本节登记的码。

### 5.2 判定

代码侧的对应物是 `ep-platform-sequence` 的类型码常量表，判据是本节与该常量表逐项一致且无重复，由 CI 项 `xtask configdoc --check-doc-type-codes` 执行。

该常量表由阶段 3a 交付。在它存在之前，「逐项一致且无重复」的被测输入为空集，比对恒真；恒真的判据不作为通过判定，因此该逐项比对整条推迟到阶段 3a 生效。阶段 1 对本节只判一件事：本节存在。

## 6. 跨模块引用的实体标记类型

下列 22 项是被三个以上阶段的契约层同时引用的实体，集中声明在 `crates/foundation/src/id/marker.rs`，供 `Id<T>` 在契约层表达跨模块引用。清单冻结 22 项，改名与增删由 `xtask archcheck` 的 `foundation-frozen-items` 规则按名逐项断言。

标记类型无字段、无方法、无 trait 实现，只承载类型身份；它们不是表，本节登记它们是因为每一项都对应后续阶段的一张主表，逐项对齐可以防止同一实体在不同模块里落成两张表。

`LegalEntity`、`UserAccount`、`Session`、`Department`、`Position`、`Project`、`Customer`、`Supplier`、`Material`、`Product`、`Warehouse`、`Contract`、`ContractLine`、`SalesOrder`、`SalesOrderLine`、`DeliveryConfirmation`、`DeliveryConfirmationLine`、`PurchaseOrder`、`GoodsReceiptLine`、`PurchaseInvoice`、`PurchaseInvoiceLine`、`AccountingPeriod`。

## 7. 维护纪律

- 先登记后实现：新增表、新增列、新增类型码都是先改本文件再写迁移。
- 已登记的类型码不得改名，也不得回收后另作他用。
- 列语义变化按破坏性变更处理，与改 API 字段同级。
- 本文件的 schema 分节随各阶段的迁移一并提交，不允许迁移已合入而字典未更新。
