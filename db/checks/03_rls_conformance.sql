-- db/checks/03_rls_conformance.sql
-- 阶段 2 计划 §3.9 第 03 项 / §3.6：策略文本与模板全等。
-- 通过判据：返回 0 行。执行方：ep-migrate check。
-- 规范化：lower、去除 ::text 冗余转型、去除全部空白与括号后全等比较。
-- 模板（§3.6）：策略名 rls_<table>_le，using 与 with check 均为
--   legal_entity_id = nullif(current_setting('app.legal_entity_id', true), '')::uuid
with le_tbls as (
  select n.nspname, c.oid, c.relname
  from pg_class c
  join pg_namespace n on n.oid = c.relnamespace
  join pg_attribute a on a.attrelid = c.oid
  where c.relkind = 'r'
    and n.nspname in (
      'platform_core','platform_authz','platform_meta','platform_flow',
      'platform_audit','platform_msg','platform_file','platform_ops','ext',
      'mdm','crm','cpq','clm','sales','procure','inventory','costing',
      'project','service','finance','ledger','invoice','portal','reporting')
    and a.attnum > 0 and not a.attisdropped and a.attname = 'legal_entity_id'
),
pol as (
  select t.nspname, t.relname, p.polname,
         lower(regexp_replace(regexp_replace(coalesce(pg_get_expr(p.polqual, t.oid), ''),
                '::text', '', 'g'), '[\s()]', '', 'g')) as qual_norm,
         lower(regexp_replace(regexp_replace(coalesce(pg_get_expr(p.polwithcheck, t.oid), ''),
                '::text', '', 'g'), '[\s()]', '', 'g')) as check_norm
  from le_tbls t
  join pg_policy p on p.polrelid = t.oid
),
tmpl(x) as (values (
  'legal_entity_id=nullifcurrent_setting''app.legal_entity_id'',true,''''::uuid'))
select p.nspname as schema_name, p.relname as table_name, p.polname as policy_name,
       'unexpected policy name, expect rls_' || p.relname || '_le' as problem
from pol p
where p.polname <> 'rls_' || p.relname || '_le'
union all
select p.nspname, p.relname, p.polname, 'using clause deviates from template'
from pol p cross join tmpl
where p.qual_norm is distinct from tmpl.x
union all
select p.nspname, p.relname, p.polname, 'with check clause deviates from template'
from pol p cross join tmpl
where p.check_norm is distinct from tmpl.x
order by 1, 2, 3;
