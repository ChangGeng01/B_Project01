# ADR-0013 迁移执行器弃用 refinery 改自建语义兼容 Runner

- 状态：已接受
- 出处：阶段 2 计划 §3.3、§12 实施期偏离登记第一条；`tools/migrate/src/history.rs` 与 `apply.rs` 模块头逐字记录

## 背景

02 计划 §3.3 指定迁移工具为 refinery 0.8 系列，全库单一 Runner 与单一历史表 `platform_core.schema_history`；同计划 §3.4 与基线通则第五条又把迁移版本号冻结为 `V<YYYYMMDDHHMMSS>` 的 14 位时间戳。但 refinery 0.8.16 的 `Migration::unapplied` 把文件名版本号 `parse::<i32>()`，其历史表 DDL 为 `version INT4 PRIMARY KEY`，i32 上限约 2.1e9，而 14 位时间戳约 2.0e13，refinery Runner 会对全部迁移文件报 InvalidVersion，无法加载。计划 §3.3 与 §3.4 两条指定不可同时成立，这是 02 计划内部的结构性矛盾。

## 决定

经 leader 批准，`tools/ep-migrate` 不依赖 refinery，自建 refinery 语义兼容 Runner 承载全部迁移施加与版本台账，兼容点逐项锁定：

- 历史表同名四列 `version`、`name`、`applied_on`、`checksum`；`version` 由 INT4 放宽为 BIGINT 以容纳 14 位版本号，其余三列形态与 refinery postgres 驱动逐项一致，由 `HISTORY_TABLE_COLUMNS` 常量与单测锁定；
- 校验和沿用 refinery 同款 SipHasher13 依次喂 name、version、sql；因本项目版本号超 i32，按 i64 喂入——这是唯一已登记的算法偏离点；事务执行器与 `concurrent/` 非事务执行器共用 `migration_checksum`，两套执行器校验和严格一致；
- `applied_on` 存 RFC3339 文本、`checksum` 存 u64 十进制文本，与 refinery postgres 驱动的存取形态一致；
- 每个常规迁移一个事务（refinery 默认行为），`concurrent/` 目录走自动提交承载 `CREATE INDEX CONCURRENTLY`。

历史表由 Runner 自建，不由本项目建表迁移产生，因此不在 `unpoliced_table_registry` 登记与 `db/checks/13` 的范围内。

## 理由

选「自建兼容 Runner」而不是「坚持 refinery」：不采用本决定的代价是 14 位版本号整体作废，全部迁移文件无法被加载，等于推翻基线通则第五条。选「自建」而不是「改短版本号」：版本号是全局全序与真实时间的载体，改短即失去时间可读性与唯一性保证，且违反通则第五条，代价由全部建表阶段分担。自建 Runner 的代价（约 200 行，只做读文件、算校验和、执行、写历史四件事）换得执行路径全部在仓内可审，窗口守卫断言与 `concurrent/` 路径的定制不再受外部 crate 形态约束。

## 后果

正面：版本号空间、事务语义与非事务路径三者的组合完全由本仓控制，`ep-migrate` 五个子命令与六个退出码按 C-02 落码时无外部形态掣肘；历史表形态与 refinery 对齐，台账可被 refinery 生态工具读取（版本号列宽除外）。

负面（如实记录）：校验和喂 i64 而非 i32，使本仓台账与 refinery 对同一文件的校验和数值不一致，任何试图用 refinery 校验本仓台账的集成都会报校验和不符，该偏离已在 `history.rs` 模块头逐字记录；仓内长期多维护一份执行器实现。若未来 refinery 修复版本号宽度并裁定恢复依赖，须先以本模块单测作为历史表兼容判据逐项核对，再走变更登记。

## 影响范围

- `tools/migrate/src/history.rs`（文件模型、校验和与历史表的唯一出处）、`apply.rs`（apply 编排）、`concurrent.rs`（非事务执行器）；
- `platform_core.schema_history` 历史表的建表与读写路径；
- 02 计划 §12 实施期偏离登记第一条与第二条（校验和编译期对齐）。
