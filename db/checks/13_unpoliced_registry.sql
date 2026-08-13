-- db/checks/13_unpoliced_registry.sql
-- 阶段 2 计划 §3.9 第 13 项 / §3.8：未受行级策略表登记一致性断言。
-- 通过判据：返回 0 行。执行方：ep-migrate check。
-- 24 个项目 schema 下全部未启用行级安全的表必须与
-- platform_core.unpoliced_table_registry 的登记行逐行一致：
-- 多一张（UNREGISTERED_UNPOLICED_TABLE）或少一张（STALE_REGISTRY_ROW）即违规。
-- refinery 自建历史表 schema_history 不在登记范围。
with unpoliced as (
  select n.nspname, c.relname
  from pg_class c
  join pg_namespace n on n.oid = c.relnamespace
  where c.relkind = 'r'
    and not c.relrowsecurity
    and n.nspname in (
      'platform_core','platform_authz','platform_meta','platform_flow',
      'platform_audit','platform_msg','platform_file','platform_ops','ext',
      'mdm','crm','cpq','clm','sales','procure','inventory','costing',
      'project','service','finance','ledger','invoice','portal','reporting')
    and not (n.nspname = 'platform_core' and c.relname = 'schema_history')
),
reg as (
  select schema_name, table_name
  from platform_core.unpoliced_table_registry
)
select u.nspname as schema_name, u.relname as table_name,
       'UNREGISTERED_UNPOLICED_TABLE' as problem
from unpoliced u
where not exists (select 1 from reg r
                  where r.schema_name = u.nspname and r.table_name = u.relname)
union all
select r.schema_name, r.table_name, 'STALE_REGISTRY_ROW'
from reg r
where not exists (select 1 from unpoliced u
                  where u.nspname = r.schema_name and u.relname = r.table_name)
order by 3, 1, 2;
