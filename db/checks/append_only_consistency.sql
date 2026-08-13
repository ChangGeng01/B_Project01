-- db/checks/append_only_consistency.sql
-- 阶段 2 计划 §3.9 附项（裁定 B-02）：仅追加登记与触发器一致性断言。
-- 通过判据：返回 0 行。执行方：xtask sqlcheck（不计入 ep-migrate check 十三项）。
-- platform_core.append_only_registry 的登记与物理表上实际挂接的触发器逐项一致：
-- APPEND_ONLY 登记须有 trg_<table>_append_only，IMMUTABLE_COLUMNS 登记须有
-- trg_<table>_immutable_columns；反向同名触发器必须有对应登记行。
-- 阶段 3b、7、8、9a、10 追加合计十四行登记后由本脚本兜底。
with reg as (
  select schema_name, table_name, mode
  from platform_core.append_only_registry
),
trg as (
  select n.nspname, c.relname, t.tgname
  from pg_trigger t
  join pg_class c on c.oid = t.tgrelid
  join pg_namespace n on n.oid = c.relnamespace
  where not t.tgisinternal
)
select r.schema_name, r.table_name, r.mode,
       'MISSING_TRIGGER_' ||
       case r.mode when 'APPEND_ONLY' then 'trg_' || r.table_name || '_append_only'
                   else 'trg_' || r.table_name || '_immutable_columns' end as problem
from reg r
where not exists (select 1 from trg t
                  where t.nspname = r.schema_name and t.relname = r.table_name
                    and t.tgname = case r.mode
                      when 'APPEND_ONLY' then 'trg_' || r.table_name || '_append_only'
                      else 'trg_' || r.table_name || '_immutable_columns' end)
union all
select t.nspname, t.relname, 'TRIGGER_WITHOUT_REGISTRY' as mode,
       'UNREGISTERED_TRIGGER_' || t.tgname
from trg t
where (t.tgname = 'trg_' || t.relname || '_append_only'
       and not exists (select 1 from reg r
                       where r.schema_name = t.nspname and r.table_name = t.relname
                         and r.mode = 'APPEND_ONLY'))
   or (t.tgname = 'trg_' || t.relname || '_immutable_columns'
       and not exists (select 1 from reg r
                       where r.schema_name = t.nspname and r.table_name = t.relname
                         and r.mode = 'IMMUTABLE_COLUMNS'))
order by 4, 1, 2;
