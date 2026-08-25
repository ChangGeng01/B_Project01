# ADR-0003 数据库默认排序固定为 libc/C 字节序

- 状态：已接受
- 出处：阶段 1 计划第 13 节新增决定二；技术基线第 3.1 节、第 3.2 节
- 落地阶段：阶段 2（`db/bootstrap/00_database.sql`）

## 背景

技术基线第 3 节没有覆盖两件事：库级排序规则取什么，以及 public schema 是否保留。这两件事都必须在建库脚本写出之前定死，因为建库参数一经使用即写入数据库，此后只能重建库才能更改。

排序规则不是显示层偏好。B-tree 索引的物理顺序由建库时的 collation 决定；若 collation 随操作系统 glibc 或 ICU 库的版本变化而变化，已有索引会在升级后静默失效，表现为查询漏行而不是报错。规格对升级的要求是回退后数据一致性零差异，静默失效的索引与这条要求直接冲突。

## 决定

建库参数固定为 `LOCALE_PROVIDER libc`、`LC_COLLATE 'C'` 与 `LC_CTYPE 'C'`，默认排序严格按 UTF-8 字节值；首版不要求 PostgreSQL 构建带 ICU，也不建立 ICU collation。同时删除 public schema。

落地脚本按裁定 C-01 由阶段 2 交付，其 `db/bootstrap/00_database.sql` 写为 `CREATE DATABASE ep ENCODING 'UTF8' LOCALE_PROVIDER libc LC_COLLATE 'C' LC_CTYPE 'C' TEMPLATE template0` 并执行 `DROP SCHEMA public`。阶段 1 只保留本决定本身，不另行取值，也不产出任何 `db/` 下的文件。

此前版本同时写“默认字节序”和 `LOCALE_PROVIDER icu + ICU_LOCALE 'zh-Hans-CN'`，两者不能同时作为数据库默认排序真值：数据库默认 collation 由数据库 locale provider 决定。现统一采用 libc/C；验收必须断言 `pg_database.datlocprovider='c'`、`datcollate='C'`、`datctype='C'`、`daticulocale IS NULL`。脚本是本决定的落地物，不得反向覆盖本决定。

## 理由

C 排序只按字节比较，不引用任何外部 collation 版本，因此操作系统或 ICU 升级不会改变已有索引的物理顺序。代价是中文按 UTF-8 字节序而不是拼音序排序，档案列表的中文排序不合阅读习惯。这属首版已知边界：需要拼音序的场景由应用层以显式排序键表达，不改库级 collation。

删除 public schema 是为了让技术基线第 3.2 节的全限定名约定没有例外出口。保留 public 的代价是任何忘写 schema 名的对象都会落进它并通过测试，直到某次 `search_path` 变化才暴露。

## 后果

正面：库排序为 C 时普通 B-tree 索引直接支持 `like` 前缀匹配走索引，各阶段不再另建 `text_pattern_ops` 操作符类索引，索引数量与迁移量都随之减少。

负面：任何需要中文阅读序的列表都必须携带由应用层生成并持久化的显式 `sort_key`；首版禁止在业务 SQL 中依赖操作系统或 ICU 的隐式中文 collation。遗漏 `sort_key` 时只会得到字节序，因此相关列表必须在契约测试中断言排序键。

## 影响范围

技术基线第 3.1 节增加两条取值，即上述建库参数与 public schema 的删除。
