-- db/checks/05_numeric_precision.sql
-- 阶段 2 计划 §3.9 第 05 项：数值精度后缀断言（本阶段新增决定，计划第 12 节）。
-- 通过判据：返回 0 行。执行方：ep-migrate check。
-- 金额列 _amount 必须 numeric(18,2)；数量列 _qty 与单价列 _unit_price 必须
-- numeric(18,6)；比率与税率列 _rate 必须 numeric(9,6)。
select n.nspname as schema_name,
       c.relname as table_name,
       a.attname as column_name,
       format_type(a.atttypid, a.atttypmod) as actual_type,
       case
         when a.attname like '%\_amount' escape '\' then 'numeric(18,2)'
         when a.attname like '%\_rate' escape '\' then 'numeric(9,6)'
         else 'numeric(18,6)'
       end as expected_type
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
  and ((a.attname like '%\_amount' escape '\'
        and format_type(a.atttypid, a.atttypmod) <> 'numeric(18,2)')
    or (a.attname like '%\_qty' escape '\'
        and format_type(a.atttypid, a.atttypmod) <> 'numeric(18,6)')
    or (a.attname like '%\_unit\_price' escape '\'
        and format_type(a.atttypid, a.atttypmod) <> 'numeric(18,6)')
    or (a.attname like '%\_rate' escape '\'
        and format_type(a.atttypid, a.atttypmod) <> 'numeric(9,6)'))
order by 1, 2, 3;
