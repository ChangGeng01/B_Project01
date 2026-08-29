# MDM 数据字典（阶段 5 开发前冻结）

> **F-57 状态：`REGISTRY_PENDING_REBASELINE`。** 主数据字段可复用，但固定岗位、旧权限和旧阶段执行口径已被动态 capability/grant、generation 和唯一数据所有者规则取代。
>
> **激活/owner task：Task 3。** 本分册目前不是 F-57 实现权威；Task 3 完成再基线并显式激活前不得据此实施。

历史状态（F-57 下无效）：曾标为开发前契约，尚未执行迁移。阶段 5 计划 §3.2–§3.3 只保留为旧逐列来源；本分册保留旧 schema 清单和跨表规则。F-57 再基线首次落地时必须由数据库元数据生成逐列表格并在同一变更中替换本状态说明，不得把旧来源称为现行权威。

## 对象清单

mdm schema 固定 28 张表：

- 档案与字典：`uoms`、`classification_items`、`customers`、`suppliers`、`materials`、`products`、`warehouses`。
- 客户子表：`customer_contacts`、`customer_addresses`、`customer_invoice_profiles`。
- 供应商子表：`supplier_contacts`、`supplier_payment_profiles`、`supplier_qualifications`、`supplier_price_records`、`supplier_leadtime_records`、`supplier_risk_records`。
- 产品关联与治理：`product_material_links`、`change_requests`、`record_versions`、`import_batches`、`import_batch_rows`、`export_jobs`。
- 附件关联：`customer_attachments`、`supplier_attachments`、`supplier_qualification_attachments`、`supplier_risk_record_attachments`、`material_attachments`、`product_attachments`。

另发布三个只读治理视图：`v_customers_dataset`、`v_products_dataset`、`v_materials_dataset`。它们必须含 `legal_entity_id`、`security_level`、`data_scope_tags`，只授予 `ep_analyst_ro` 与既有应用读权限，不授予写权限。

## 冻结规则

- 全部 28 张表带 `legal_entity_id` 并启用、强制 RLS；本 schema 不登记无 RLS 例外。
- 同 schema 引用使用真实外键并 `ON DELETE RESTRICT`；业务用户列以 `(legal_entity_id,<user-column>)` 指向 `platform_authz.user_legal_entity_grants(legal_entity_id,user_id)`，附件列以 `(legal_entity_id,attachment_object_id)` 指向 `platform_file.attachment_objects(legal_entity_id,id)`，均为真实复合外键并 `ON DELETE RESTRICT`。公开契约另校验目标状态与业务范围，不替代外键。
- `classification_items` 固定九类，其中 F-51 七类出厂编码逐字取阶段 5 §3.3；税率不在本表，唯一来源是 `invoice.tax_rate_options`。
- `products.costing_mode` 只允许 `INVENTORY|DIRECT_EXPENSE`。恰有一条启用物料关联时必须为 `INVENTORY`；无启用物料关联的服务产品必须为 `DIRECT_EXPENSE`。
- `warehouses` 是仓库唯一权威；默认收货和默认发货各以 NULL 槽位加普通唯一索引保证每法人最多一个。`owner_user_id` 以 `(legal_entity_id,owner_user_id)` 真实复合外键指向 `platform_authz.user_legal_entity_grants(legal_entity_id,user_id)`。
- `customer_invoice_profiles` 与 `supplier_payment_profiles` 的 `bank_name`、`bank_account_no` 只以 `_enc + _key_ref` 物理列保存；账号另存 `tail` 与完整 32 字节盲索引，禁止同名明文列。
- `record_versions` 与 `import_batch_rows` 是仅追加表；不带 `row_version/updated_at/updated_by`，也不带无意义的 `reverses_id`。
- 六张附件关联表不物理删除，以 `is_active/deactivated_at` 解除关联。

## 编号类型码

`CUST`、`SUPP`、`MATL`、`PROD`、`WHSE`、`MDCR`、`MDIB`、`MDEX` 已在总册 §5.1 登记。档案编号不含年月段；三类单据编号含年月段。

## 一致性判据

首次迁移落地及以后每次结构变更都必须同时满足：数据库元数据、~~阶段 5 §3.2–§3.3~~（**F-68 更正**：该对照物属已被 F-57 取代的十四阶段，现行对照物是 **G0 生成的 data dictionary**）、本分册生成的逐列内容三者逐表逐列一致；`xtask configdoc`、RLS 矩阵和迁移快照任一不一致即**该次结构变更不得落地**（原写「阶段退出失败」，F-57 下无阶段退出时点，属恒假触发，F-68 改为按变更事件触发）。
