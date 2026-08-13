# `db/checks/` SQL 合规断言脚本

本目录是技术基线第 1.1 节第二段固定的顶层目录之一。

## 本目录的交付物

阶段 2 交付物 D-04 已全部就位：13 个编号断言脚本加一个不编号的 `append_only_consistency.sql`，
唯一出处是阶段 2 计划第 3.9 节。编号与断言内容一一绑定：

| 编号 | 文件 | 断言内容 |
|---|---|---|
| 01 | `01_common_columns.sql` | 公共列齐备（`migration_window_lock` 豁免） |
| 02 | `02_rls_enabled.sql` | 行级安全已启用且强制，策略数恰为 1 |
| 03 | `03_rls_conformance.sql` | 策略文本与 §3.6 模板全等 |
| 04 | `04_time_column_types.sql` | 时间列类型后缀（`_at`/`_date`/`_on`） |
| 05 | `05_numeric_precision.sql` | 数值精度后缀（金额/数量/单价/比率） |
| 06 | `06_naming.sql` | 命名前缀（pk/ux/ix/fk/ck/rls/sq） |
| 07 | `07_identifier_length.sql` | 无被静默截断的 63 字节标识符 |
| 08 | `08_no_forbidden_objects.sql` | 无 enum、函数索引、部分索引、JSON 路径索引 |
| 09 | `09_sql_hygiene.sql` | 无 current_date、全外键 RESTRICT、跨 schema 外键复合形式 |
| 10 | `10_baseline_indexes.sql` | 基线索引齐备 |
| 11 | `11_sensitive_field_encryption.sql` | 敏感字段加密分支的物理列集（A-28） |
| 12 | `12_collation_conformance.sql` | 排序规则一致性（ICU zh-Hans-CN） |
| 13 | `13_unpoliced_registry.sql` | 未受策略表登记逐行一致 |
| — | `append_only_consistency.sql` | 仅追加登记与触发器双向一致（B-02） |

每个脚本是一条（或一组 `UNION ALL` 连接的）查询，**返回 0 行即通过**，非 0 行即列出违规对象。

脚本在**活库**上执行，被测对象是 `pg_catalog` 中的实际对象；`xtask sqlcheck` 的静态规则检查的
是仓库里的 SQL 文本。两者判据不同、执行时机不同、承载方也不同，互为补充而非替代。

阶段 1 曾说明本阶段之前不建空文件占位的理由：编号与断言内容一一绑定，空脚本返回 0 行会与
「断言通过」在返回值上无法区分，那是一条静默返回成功的路径。

## 两条执行路径

| 脚本 | 执行方 | 通过判据 |
|---|---|---|
| 01 至 13 号编号脚本 | `ep-migrate check` | 每脚本返回 0 行 |
| `append_only_consistency.sql` | `xtask sqlcheck` | 返回 0 行 |

不编号的那一个不计入 `ep-migrate check` 的十三项。两条路径分开，是因为编号脚本断言的是迁移刚落成的
物理结构、须在迁移窗口内随迁移一并判定，而仅追加登记与触发器的一致性要在登记行由多个阶段陆续补齐
之后持续兜底，跟单次迁移窗口不同步。
