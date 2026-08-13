-- db/checks/02_rls_enabled.sql
-- 阶段 2 计划 §3.9 第 02 项：行级安全已启用且强制。
-- 通过判据：返回 0 行。执行方：ep-migrate check。
-- 凡带 legal_entity_id 列的表：relrowsecurity 与 relforcerowsecurity 均为 true，
-- 且策略数恰为 1（策略一律经 platform_core.apply_le_rls 生成）。
with le_tbls as (
  select n.nspname, c.oid, c.relname,
         c.relrowsecurity, c.relforcerowsecurity
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
)
select t.nspname as schema_name,
       t.relname as table_name,
       t.relrowsecurity,
       t.relforcerowsecurity,
       (select count(*) from pg_policy p where p.polrelid = t.oid) as policy_count
from le_tbls t
where not t.relrowsecurity
   or not t.relforcerowsecurity
   or (select count(*) from pg_policy p where p.polrelid = t.oid) <> 1
order by 1, 2;
