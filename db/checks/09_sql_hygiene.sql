-- db/checks/09_sql_hygiene.sql
-- 阶段 2 计划 §3.9 第 09 项 / §3.8：SQL 卫生断言。
-- 通过判据：返回 0 行。执行方：ep-migrate check。
-- 一、函数体内不得出现 current_date（全库用 business_today() 代替）；
-- 二、全部外键必须 ON DELETE RESTRICT（含禁 CASCADE）；
-- 三、跨 schema 外键必须取 §3.8 复合形式：本地 (legal_entity_id, <ref>_id)
--     指向被引用表 (legal_entity_id, id)。
-- DDL 文本中的 CASCADE 与 varchar 等由 xtask sqlcheck 静态拦截。
select n.nspname as schema_name, p.proname as object_name,
       'function body contains current_date' as problem
from pg_proc p
join pg_namespace n on n.oid = p.pronamespace
where n.nspname in (
    'platform_core','platform_authz','platform_meta','platform_flow',
    'platform_audit','platform_msg','platform_file','platform_ops','ext',
    'mdm','crm','cpq','clm','sales','procure','inventory','costing',
    'project','service','finance','ledger','invoice','portal','reporting')
  and p.prosrc ~ '\mcurrent_date\M'
union all
select n.nspname, con.conname,
       'foreign key on delete is not restrict (' ||
       case con.confdeltype when 'a' then 'no action' when 'c' then 'cascade'
            when 'n' then 'set null' when 'd' then 'set default' end || ')'
from pg_constraint con
join pg_class c on c.oid = con.conrelid
join pg_namespace n on n.oid = c.relnamespace
where con.contype = 'f'
  and con.confdeltype <> 'r'
  and n.nspname in (
    'platform_core','platform_authz','platform_meta','platform_flow',
    'platform_audit','platform_msg','platform_file','platform_ops','ext',
    'mdm','crm','cpq','clm','sales','procure','inventory','costing',
    'project','service','finance','ledger','invoice','portal','reporting')
union all
select n.nspname, con.conname,
       'cross-schema foreign key must take composite form (legal_entity_id, <ref>_id) -> (legal_entity_id, id)'
from pg_constraint con
join pg_class c on c.oid = con.conrelid
join pg_namespace n on n.oid = c.relnamespace
join pg_class rc on rc.oid = con.confrelid
join pg_namespace rn on rn.oid = rc.relnamespace
where con.contype = 'f'
  and n.nspname <> rn.nspname
  and n.nspname in (
    'platform_core','platform_authz','platform_meta','platform_flow',
    'platform_audit','platform_msg','platform_file','platform_ops','ext',
    'mdm','crm','cpq','clm','sales','procure','inventory','costing',
    'project','service','finance','ledger','invoice','portal','reporting')
  and rn.nspname in (
    'platform_core','platform_authz','platform_meta','platform_flow',
    'platform_audit','platform_msg','platform_file','platform_ops','ext',
    'mdm','crm','cpq','clm','sales','procure','inventory','costing',
    'project','service','finance','ledger','invoice','portal','reporting')
  and not (
    array_length(con.conkey, 1) = 2
    and (select attname from pg_attribute
         where attrelid = con.conrelid and attnum = con.conkey[1]) = 'legal_entity_id'
    and (select attname from pg_attribute
         where attrelid = con.conrelid and attnum = con.conkey[2]) like '%\_id' escape '\'
    and (select attname from pg_attribute
         where attrelid = con.confrelid and attnum = con.confkey[1]) = 'legal_entity_id'
    and (select attname from pg_attribute
         where attrelid = con.confrelid and attnum = con.confkey[2]) = 'id')
order by 1, 2;
