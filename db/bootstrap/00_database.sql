-- db/bootstrap/00_database.sql
-- 集群引导第 1 步，共 5 步。执行顺序：00 → 01 → 02 → 03 → 04，见本目录 README.md。
--
-- 承载内容：建库、库级会话默认值、库上 PUBLIC 权限的回收、public schema 的处置。
-- 取值唯一出处是 ADR-0003：`LOCALE_PROVIDER libc`、`LC_COLLATE 'C'`、`LC_CTYPE 'C'`，
-- 默认排序严格按 UTF-8 字节值。本文件是该决定的落地物，不得反向覆盖它。
--
-- 为什么不是 ICU（F-71 更正，本行以上曾写 `locale_provider icu icu_locale 'zh-Hans-CN'`）：
-- B-tree 索引的物理顺序由建库时的 collation 决定。ICU 或 glibc 升级会改变比较规则，
-- 已建索引随即静默失效——表现是**查询漏行而不是报错**，与「升级回退后数据一致性零差异」
-- 直接冲突。C 排序只按字节比较，不引用任何外部 collation 版本，因此不受升级影响；
-- 首版也因此不要求 PostgreSQL 构建带 ICU。裁定 00c 逐字：旧 `LOCALE_PROVIDER icu`、
-- `ICU_LOCALE 'zh-Hans-CN'`「只作历史证据，**不得实现**」。
--
-- 代价是明码标价的：中文按字节序而不是拼音序，档案列表的中文排序不合阅读习惯。
-- 需要中文阅读序的列表由应用层生成并持久化显式 `sort_key` 列承担，不改库级 collation。
-- LC_CTYPE 'C' 的代价是 upper()/lower() 对非 ASCII 不做大小写映射，本系统不依赖该映射。
--
-- 为什么排在第 1 步：后续四步的对象要么建在本库内，要么以本库为授权范围；
-- 且建库语句须以 template0 为模板并独占一个会话，不能与其他语句共事务。
--
-- 为什么不写口令：口令由安装器从机密库读取后经 ALTER ROLE ... PASSWORD 单独注入。
-- 本目录任何文件中出现口令字面量即违反 xtask sqlcheck 规则 SQL-020。
--
-- 可重复执行形态：建库语句以「库存在性查询 + \gexec」守护，库已存在时查询返回零行、
-- 不执行任何建库；ALTER DATABASE 与 REVOKE 幂等；DROP SCHEMA 带 IF EXISTS。
-- 第二次执行退出码 0 且不产生变更。

-- 建库：仅当 ep 库尚不存在时，查询返回一条 CREATE DATABASE 语句交由 \gexec 执行。
-- 该语句必须独占会话、以 template0 为模板，不得放入 DO 块或多语句事务。
select format(
         'create database %I encoding %L locale_provider libc lc_collate %L lc_ctype %L template %I',
         'ep', 'UTF8', 'C', 'C', 'template0')
where not exists (select 1 from pg_database where datname = 'ep')
\gexec

-- 库级会话默认值：时区一律 UTC，事务隔离一律 read committed。
-- 两条 ALTER 覆盖同值时不产生行为变化，构成重复执行形态。
alter database ep set timezone = 'UTC';
alter database ep set default_transaction_isolation = 'read committed';

-- 回收 PUBLIC 在库上的全部权限：新连接默认不获得任何库级权限，
-- 后续权限只来自 01_roles.sql 的显式授予与迁移中的默认权限授予。
revoke all on database ep from public;

-- 以下两条作用于 ep 库内的 public schema，必须在连入 ep 之后执行。
\c ep

-- 先回收再删除：PUBLIC 不得在默认 schema 里建对象，本系统的对象一律落 24 个业务 schema。
-- 以存在性检查守护：重复执行时 schema 已不存在则整段跳过，不报错。
do $$
begin
  if exists (select 1 from pg_namespace where nspname = 'public') then
    revoke all on schema public from public;
    drop schema public;
  end if;
end $$;
