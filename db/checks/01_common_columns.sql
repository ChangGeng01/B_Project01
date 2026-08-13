-- db/checks/01_common_columns.sql
-- 阶段 2 计划 §3.9 第 01 项：公共列齐备断言。
-- 通过判据：返回 0 行。执行方：ep-migrate check。
-- 判据与基线第 4 节、sqlcheck SQL-008 同口径：
--   带 legal_entity_id 列的表，九件公共列必须齐备且按序占据第 1 至第 9 列；
--   不带 legal_entity_id 列的表，其余八件必须齐备且按序占据第 1 至第 8 列；
--   在 append_only_registry 登记为 APPEND_ONLY 的表可整体省略
--   row_version、updated_at、updated_by 三列（同缺才合法，缺一即违规）。
-- 豁免（逐个具名，均写明计划出处，不扩散）：
--   platform_core.schema_history：refinery 自建历史表，非本项目建表；
--   platform_core.migration_window_lock：计划 §3.5 表六明示豁免（登记于
--     unpoliced_table_registry 且行数由 check (id = 1) 固定为一行）；
--   platform_ops.degradation_windows：02 计划表十二按 A-26 以阶段 14 计划列清单
--     为准，本阶段只建表与两条约束，列位序不归公共列口径（已登记
--     unpoliced_table_registry，rls_matrix_unpoliced_degradation_windows）；
--   platform_msg.idempotency_keys：03 计划表 12 逐字冻结列集，§3.3.2 明示
--     纯技术表不带 security_level/data_scope_tags 与审计列，属后位计划对
--     公共列适用口径的逐表修订；与本节九列口径的冲突裁定已正式登记于
--     02 计划第 12 节偏离登记十五，豁免以该登记为唯一出处，不扩散。
with tbls as (
  select n.nspname, c.oid, c.relname,
         exists (select 1 from pg_attribute x
                 where x.attrelid = c.oid and x.attnum > 0 and not x.attisdropped
                   and x.attname = 'legal_entity_id') as has_le,
         exists (select 1 from platform_core.append_only_registry r
                 where r.schema_name = n.nspname and r.table_name = c.relname
                   and r.mode = 'APPEND_ONLY') as is_append_only
  from pg_class c
  join pg_namespace n on n.oid = c.relnamespace
  where c.relkind = 'r'
    and n.nspname in (
      'platform_core','platform_authz','platform_meta','platform_flow',
      'platform_audit','platform_msg','platform_file','platform_ops','ext',
      'mdm','crm','cpq','clm','sales','procure','inventory','costing',
      'project','service','finance','ledger','invoice','portal','reporting')
    and not (n.nspname = 'platform_core'
             and c.relname in ('schema_history', 'migration_window_lock'))
    and not (n.nspname = 'platform_ops' and c.relname = 'degradation_windows')
    and not (n.nspname = 'platform_msg' and c.relname = 'idempotency_keys')
),
expected as (
  select t.nspname, t.oid, t.relname, e.pos, e.col
  from tbls t
  cross join (values (1,'id'),(2,'legal_entity_id'),(3,'security_level'),
                   (4,'data_scope_tags'),(5,'row_version'),(6,'created_at'),
                   (7,'created_by'),(8,'updated_at'),(9,'updated_by')) as e(pos,col)
  where t.has_le and not t.is_append_only
  union all
  select t.nspname, t.oid, t.relname, e.pos, e.col
  from tbls t
  cross join (values (1,'id'),(2,'legal_entity_id'),(3,'security_level'),
                   (4,'data_scope_tags'),(5,'created_at'),(6,'created_by')) as e(pos,col)
  where t.has_le and t.is_append_only
  union all
  select t.nspname, t.oid, t.relname, e.pos, e.col
  from tbls t
  cross join (values (1,'id'),(2,'security_level'),(3,'data_scope_tags'),
                   (4,'row_version'),(5,'created_at'),(6,'created_by'),
                   (7,'updated_at'),(8,'updated_by')) as e(pos,col)
  where not t.has_le and not t.is_append_only
  union all
  select t.nspname, t.oid, t.relname, e.pos, e.col
  from tbls t
  cross join (values (1,'id'),(2,'security_level'),(3,'data_scope_tags'),
                   (4,'created_at'),(5,'created_by')) as e(pos,col)
  where not t.has_le and t.is_append_only
)
select e.nspname as schema_name,
       e.relname as table_name,
       e.pos as position,
       e.col as expected_column,
       a.attname as actual_column
from expected e
left join pg_attribute a
       on a.attrelid = e.oid and a.attnum = e.pos and not a.attisdropped
where a.attname is distinct from e.col
order by e.nspname, e.relname, e.pos;
