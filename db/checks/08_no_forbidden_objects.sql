-- db/checks/08_no_forbidden_objects.sql
-- 阶段 2 计划 §3.9 第 08 项 / 基线 §3.10：禁用对象断言。
-- 通过判据：返回 0 行。执行方：ep-migrate check。
-- 禁 PostgreSQL enum 类型、函数索引（表达式索引）、部分索引与 JSON 路径索引；
-- DDL 层面的 CASCADE 与 varchar 由 xtask sqlcheck 静态拦截，不在活库判定范围。
select n.nspname as schema_name, t.typname as object_name, 'enum type' as problem
from pg_type t
join pg_namespace n on n.oid = t.typnamespace
where t.typtype = 'e'
  and n.nspname in (
    'platform_core','platform_authz','platform_meta','platform_flow',
    'platform_audit','platform_msg','platform_file','platform_ops','ext',
    'mdm','crm','cpq','clm','sales','procure','inventory','costing',
    'project','service','finance','ledger','invoice','portal','reporting')
union all
select n.nspname, ic.relname, 'expression index'
from pg_index i
join pg_class ic on ic.oid = i.indexrelid
join pg_namespace n on n.oid = ic.relnamespace
where i.indexprs is not null
  and n.nspname in (
    'platform_core','platform_authz','platform_meta','platform_flow',
    'platform_audit','platform_msg','platform_file','platform_ops','ext',
    'mdm','crm','cpq','clm','sales','procure','inventory','costing',
    'project','service','finance','ledger','invoice','portal','reporting')
union all
select n.nspname, ic.relname, 'partial index'
from pg_index i
join pg_class ic on ic.oid = i.indexrelid
join pg_namespace n on n.oid = ic.relnamespace
where i.indpred is not null
  and n.nspname in (
    'platform_core','platform_authz','platform_meta','platform_flow',
    'platform_audit','platform_msg','platform_file','platform_ops','ext',
    'mdm','crm','cpq','clm','sales','procure','inventory','costing',
    'project','service','finance','ledger','invoice','portal','reporting')
union all
select n.nspname, ic.relname, 'jsonb path ops index'
from pg_index i
join pg_class ic on ic.oid = i.indexrelid
join pg_namespace n on n.oid = ic.relnamespace
where n.nspname in (
    'platform_core','platform_authz','platform_meta','platform_flow',
    'platform_audit','platform_msg','platform_file','platform_ops','ext',
    'mdm','crm','cpq','clm','sales','procure','inventory','costing',
    'project','service','finance','ledger','invoice','portal','reporting')
  and exists (select 1 from unnest(i.indclass) as k(opclass_oid)
              join pg_opclass oc on oc.oid = k.opclass_oid
              where oc.opcname = 'jsonb_path_ops')
order by 1, 2;
