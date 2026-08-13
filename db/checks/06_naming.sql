-- db/checks/06_naming.sql
-- 阶段 2 计划 §3.9 第 06 项 / 基线 §3.10：命名前缀断言。
-- 通过判据：返回 0 行。执行方：ep-migrate check。
-- 主键 pk_、唯一 ux_、普通索引 ix_、外键 fk_、检查 ck_、策略 rls_、序列 sq_。
-- refinery 历史表 schema_history 及其附属对象不在本项目命名契约内，排除。
select n.nspname as schema_name, c.relname as table_name,
       con.conname as object_name, 'constraint' as object_kind,
       case con.contype when 'p' then 'pk_' when 'u' then 'ux_'
                        when 'f' then 'fk_' when 'c' then 'ck_' end as expected_prefix
from pg_constraint con
join pg_class c on c.oid = con.conrelid
join pg_namespace n on n.oid = c.relnamespace
where n.nspname in (
    'platform_core','platform_authz','platform_meta','platform_flow',
    'platform_audit','platform_msg','platform_file','platform_ops','ext',
    'mdm','crm','cpq','clm','sales','procure','inventory','costing',
    'project','service','finance','ledger','invoice','portal','reporting')
  and not (n.nspname = 'platform_core' and c.relname = 'schema_history')
  and ((con.contype = 'p' and con.conname not like 'pk\_%' escape '\')
    or (con.contype = 'u' and con.conname not like 'ux\_%' escape '\')
    or (con.contype = 'f' and con.conname not like 'fk\_%' escape '\')
    or (con.contype = 'c' and con.conname not like 'ck\_%' escape '\'))
union all
select n.nspname, ic.relname, ic.relname, 'index', 'pk_/ux_/ix_'
from pg_class ic
join pg_namespace n on n.oid = ic.relnamespace
where ic.relkind = 'i'
  and n.nspname in (
    'platform_core','platform_authz','platform_meta','platform_flow',
    'platform_audit','platform_msg','platform_file','platform_ops','ext',
    'mdm','crm','cpq','clm','sales','procure','inventory','costing',
    'project','service','finance','ledger','invoice','portal','reporting')
  and ic.relname not like 'schema_history%'
  and ic.relname not like 'pk\_%' escape '\'
  and ic.relname not like 'ux\_%' escape '\'
  and ic.relname not like 'ix\_%' escape '\'
union all
select n.nspname, c.relname, p.polname, 'policy', 'rls_'
from pg_policy p
join pg_class c on c.oid = p.polrelid
join pg_namespace n on n.oid = c.relnamespace
where n.nspname in (
    'platform_core','platform_authz','platform_meta','platform_flow',
    'platform_audit','platform_msg','platform_file','platform_ops','ext',
    'mdm','crm','cpq','clm','sales','procure','inventory','costing',
    'project','service','finance','ledger','invoice','portal','reporting')
  and p.polname not like 'rls\_%' escape '\'
union all
select n.nspname, c.relname, c.relname, 'sequence', 'sq_'
from pg_class c
join pg_namespace n on n.oid = c.relnamespace
where c.relkind = 'S'
  and n.nspname in (
    'platform_core','platform_authz','platform_meta','platform_flow',
    'platform_audit','platform_msg','platform_file','platform_ops','ext',
    'mdm','crm','cpq','clm','sales','procure','inventory','costing',
    'project','service','finance','ledger','invoice','portal','reporting')
  and c.relname not like 'schema_history%'
  and c.relname not like 'sq\_%' escape '\'
order by 1, 2, 3;
