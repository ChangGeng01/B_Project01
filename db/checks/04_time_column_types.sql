-- db/checks/04_time_column_types.sql
-- 阶段 2 计划 §3.9 第 04 项：时间列类型后缀断言。
-- 通过判据：返回 0 行。执行方：ep-migrate check。
-- 后缀 _at 必须为 timestamptz；后缀 _date 与 _on 必须为 date（基线第 4 节）。
select n.nspname as schema_name,
       c.relname as table_name,
       a.attname as column_name,
       format_type(a.atttypid, a.atttypmod) as actual_type,
       case when a.attname like '%\_at' escape '\'
            then 'timestamp with time zone' else 'date' end as expected_type
from pg_class c
join pg_namespace n on n.oid = c.relnamespace
join pg_attribute a on a.attrelid = c.oid
where c.relkind = 'r'
  and n.nspname in (
    'platform_core','platform_authz','platform_meta','platform_flow',
    'platform_audit','platform_msg','platform_file','platform_ops','ext',
    'mdm','crm','cpq','clm','sales','procure','inventory','costing',
    'project','service','finance','ledger','invoice','portal','reporting')
  and not (n.nspname = 'platform_core' and c.relname = 'schema_history')
  and a.attnum > 0 and not a.attisdropped
  and ((a.attname like '%\_at' escape '\'
        and format_type(a.atttypid, a.atttypmod) <> 'timestamp with time zone')
    or (a.attname like '%\_date' escape '\'
        and format_type(a.atttypid, a.atttypmod) <> 'date')
    or (a.attname like '%\_on' escape '\'
        and format_type(a.atttypid, a.atttypmod) <> 'date'))
order by 1, 2, 3;
