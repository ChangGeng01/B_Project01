-- db/bootstrap/00_database.sql
-- 集群引导第 1 步，共 5 步。执行顺序：00 → 01 → 02 → 03 → 04，见本目录 README.md。
--
-- 承载内容：建库、库级会话默认值、库上 PUBLIC 权限的回收、public schema 的处置。
-- 取值唯一出处是阶段 2 计划第 3.1 节；建库的排序规则提供者取 ICU 且 locale 取
-- zh-Hans-CN，是阶段 1 新增决定二的落地，本阶段只执行不改其取值。
-- LC_COLLATE 与 LC_CTYPE 取 C 只作语法兜底（PostgreSQL 16 在 LOCALE_PROVIDER icu 下
-- 仍要求给出两者且必须以 template0 为模板），字符串比较与排序一律由 ICU 承担；
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
         'create database %I encoding %L locale_provider icu icu_locale %L lc_collate %L lc_ctype %L template %I',
         'ep', 'UTF8', 'zh-Hans-CN', 'C', 'C', 'template0')
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
