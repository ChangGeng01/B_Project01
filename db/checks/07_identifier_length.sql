-- db/checks/07_identifier_length.sql
-- 阶段 2 计划 §3.9 第 07 项：标识符长度断言。
-- 通过判据：返回 0 行。执行方：ep-migrate check。
-- PostgreSQL 对超过 63 字节的标识符静默截断为 63 字节；本项目约定超长即按列序
-- 缩写并把全称登记数据字典，因此任何恰为 63 字节的对象名都视为疑似截断产物。
select o.schema_name, o.object_name, o.object_kind,
       octet_length(o.object_name) as name_bytes
from (
  select n.nspname as schema_name, c.relname as object_name,
         case c.relkind when 'r' then 'table' when 'i' then 'index'
                        when 'S' then 'sequence' else c.relkind::text end as object_kind
  from pg_class c
  join pg_namespace n on n.oid = c.relnamespace
  where n.nspname in (
      'platform_core','platform_authz','platform_meta','platform_flow',
      'platform_audit','platform_msg','platform_file','platform_ops','ext',
      'mdm','crm','cpq','clm','sales','procure','inventory','costing',
      'project','service','finance','ledger','invoice','portal','reporting')
    and c.relkind in ('r', 'i', 'S')
  union all
  select n.nspname, con.conname, 'constraint'
  from pg_constraint con
  join pg_class c on c.oid = con.conrelid
  join pg_namespace n on n.oid = c.relnamespace
  where n.nspname in (
      'platform_core','platform_authz','platform_meta','platform_flow',
      'platform_audit','platform_msg','platform_file','platform_ops','ext',
      'mdm','crm','cpq','clm','sales','procure','inventory','costing',
      'project','service','finance','ledger','invoice','portal','reporting')
  union all
  select n.nspname, p.polname, 'policy'
  from pg_policy p
  join pg_class c on c.oid = p.polrelid
  join pg_namespace n on n.oid = c.relnamespace
  where n.nspname in (
      'platform_core','platform_authz','platform_meta','platform_flow',
      'platform_audit','platform_msg','platform_file','platform_ops','ext',
      'mdm','crm','cpq','clm','sales','procure','inventory','costing',
      'project','service','finance','ledger','invoice','portal','reporting')
  union all
  select n.nspname, pr.proname, 'function'
  from pg_proc pr
  join pg_namespace n on n.oid = pr.pronamespace
  where n.nspname in (
      'platform_core','platform_authz','platform_meta','platform_flow',
      'platform_audit','platform_msg','platform_file','platform_ops','ext',
      'mdm','crm','cpq','clm','sales','procure','inventory','costing',
      'project','service','finance','ledger','invoice','portal','reporting')
) o
where octet_length(o.object_name) = 63
order by 1, 3, 2;
