# ADR-0015 盲索引固定使用完整 32 字节，唯一性由业务约束决定

- 状态：已接受并冻结
- 出处：阶段 2 计划 §4.4、§12 假设三与 §11 风险 R-05；阶段 5、10 银行账号盲索引；`crates/foundation/src/port/kms.rs`
- 链接说明：文件名中的 `16-byte-truncation` 是早期草案留下的稳定链接名，不代表现行决定；不得据文件名实现 16 字节截断。

## 背景

受字段级加密的列不能直接参与过滤、排序或唯一约束，需要等值定位的敏感属性另建 `<col>_bidx bytea null`。早期草案同时出现半长截断类型、可变宽度配置，以及资金账户单列使用完整输出三套互斥口径；这会让类型、数据库约束与调用点无法形成单一契约，也会使部署配置改变持久化数据格式。

## 决定

`KmsBackend::derive_blind_key(legal_entity_id, column_fqn, plaintext)` 固定返回 `BlindIndex([u8; 32])`，其值是完整的 `HMAC-SHA256(blind_key, normalize(value))` 输出。参数名 `column_fqn` 因 ABI 保留，但现行合法值是 scoped selector `schema.table.column@<10|20|30|40>`；写入与查询必须从同一 sensitive-field/effective-level resolver 取 scope，裸 FQN 拒绝。调用方不得传入宽度、不得截断结果，也不得自建第二套哈希。

每个 `<col>_bidx bytea null` 必须带 `CHECK (<col>_bidx IS NULL OR octet_length(<col>_bidx) = 32)`；数据字典与 `db/checks/11` 必须登记并核验该约束。盲索引宽度不是配置项，不允许全局或逐字段覆盖。

是否建立唯一约束只由业务字段规则决定，与宽度正交。默认等值定位列建普通 btree；`finance.cash_accounts.bank_account_no_bidx` 因“同一法人内银行账号不重复”建立唯一约束，阶段 5 两张客户/供应商资料表没有该业务要求，只建普通索引。三者都使用相同的 32 字节类型与算法，不存在宽度例外。盲索引只支持等值；`PREFIX` 仍为预留且首版不放行。

## 理由

完整 32 字节保留 HMAC-SHA256 的全部输出强度，去掉截断碰撞面，并让 Rust 类型、数据库行格式、迁移检查和调用点只有一套可测试契约。它还避免配置变更造成同一列中新旧宽度混存。代价是相较早期半长截断草案，每个非空盲索引值及其索引项多占 16 字节；在本系统预计规模下，该成本低于双路径带来的实现复杂度与高保密场景中的碰撞风险。

唯一性保持按业务逐列表达，因为“可等值定位”不等于“业务值必须唯一”。这既不把所有敏感字段误设为唯一，也不需要用不同宽度表达业务差异。

## 后果

正面：`BlindIndex`、数据库列和所有调用点宽度一致；配置发布不再能改变持久化格式；资金账户与主数据资料的差异只剩可审计的业务唯一约束。

负面与残余风险：盲索引列和 btree 比早期 16 字节方案更宽；确定性等值盲索引仍会泄漏同一法人、同一字段内的相等关系与频次分布。该泄漏是可检索性固有取舍，继续通过 `sensitive_field_registry` 逐列登记、法人和列域密钥隔离、最小权限及不开放通用检索接口治理；改为 32 字节不会消除这一风险。

## 影响范围

- `ep_foundation::port::kms::BlindIndex([u8; 32])` 与 `KmsBackend::derive_blind_key`；
- 所有 `_bidx` 数据库列的 32 字节 CHECK、数据字典与 `db/checks/11`；
- 阶段 5 两张 mdm 资料表与阶段 10 `finance.cash_accounts`；
- 配置参考：不得登记盲索引宽度配置。
