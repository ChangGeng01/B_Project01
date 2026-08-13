-- db/checks/10_baseline_indexes.sql
-- 阶段 2 计划 §3.9 第 10 项 / 基线 §3.10：基线索引齐备断言。
-- 通过判据：返回 0 行。执行方：ep-migrate check。
-- 每张业务表必须有 pk_<table>；带 legal_entity_id 列的表还必须有
-- ix_<table>_legal_entity_id_created_at。单据类的 ux_<table>_legal_entity_id_doc_no
-- 无法从目录机械识别单据表，由建表迁移末尾的
-- platform_core.assert_baseline_indexes(p_is_document = true) 在迁移时点断言。
-- 豁免：platform_msg.idempotency_keys 的索引清单按 03 计划表 12 逐字冻结
-- （pk + ux_idempotency_keys_le_user_id_endpoint_key + ix_…_expires_at），
-- ux 首列即 legal_entity_id，法人前缀覆盖已由唯一索引承载；与基线 §3.10
-- 的 ix_<table>_legal_entity_id_created_at 要求的冲突裁定已正式登记于
-- 02 计划第 12 节偏离登记十五，豁免以该登记为唯一出处，不扩散。
with tbls as (
  select n.nspname, c.relname,
         exists (select 1 from pg_attribute a
                 where a.attrelid = c.oid and a.attnum > 0 and not a.attisdropped
                   and a.attname = 'legal_entity_id') as has_le
  from pg_class c
  join pg_namespace n on n.oid = c.relnamespace
  where c.relkind = 'r'
    and n.nspname in (
      'platform_core','platform_authz','platform_meta','platform_flow',
      'platform_audit','platform_msg','platform_file','platform_ops','ext',
      'mdm','crm','cpq','clm','sales','procure','inventory','costing',
      'project','service','finance','ledger','invoice','portal','reporting')
    and not (n.nspname = 'platform_core' and c.relname = 'schema_history')
)
select t.nspname as schema_name, t.relname as table_name,
       'missing index pk_' || t.relname as problem
from tbls t
where not exists (select 1 from pg_indexes i
                  where i.schemaname = t.nspname and i.tablename = t.relname
                    and i.indexname = 'pk_' || t.relname)
union all
select t.nspname, t.relname,
       'missing index ix_' || t.relname || '_legal_entity_id_created_at'
from tbls t
where t.has_le
  and not (t.nspname = 'platform_msg' and t.relname = 'idempotency_keys')
  and not exists (select 1 from pg_indexes i
                  where i.schemaname = t.nspname and i.tablename = t.relname
                    and i.indexname = 'ix_' || t.relname || '_legal_entity_id_created_at')
order by 1, 2, 3;
