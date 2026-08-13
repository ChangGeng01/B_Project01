-- db/checks/11_sensitive_field_encryption.sql
-- 阶段 2 计划 §3.9 第 11 项（裁定 A-28）：敏感字段物理列集断言。
-- 通过判据：返回 0 行。执行方：ep-migrate check。
-- is_field_encrypted 取真的登记项：物理表必须存在 <column>_enc bytea 与
-- <column>_key_ref text，且不保留同名明文列 <column>；
-- 取假的登记项：只断言三元组命中 information_schema 中的实际列。
select r.schema_name, r.table_name, r.column_name,
       'encrypted field must have <col>_enc bytea and <col>_key_ref text, and must not keep plaintext column' as problem
from platform_core.sensitive_field_registry r
where r.is_field_encrypted
  and (not exists (select 1 from pg_attribute a
                   join pg_class c on c.oid = a.attrelid
                   join pg_namespace n on n.oid = c.relnamespace
                   where n.nspname = r.schema_name and c.relname = r.table_name
                     and a.attname = r.column_name || '_enc'
                     and a.attnum > 0 and not a.attisdropped
                     and format_type(a.atttypid, null) = 'bytea')
    or not exists (select 1 from pg_attribute a
                   join pg_class c on c.oid = a.attrelid
                   join pg_namespace n on n.oid = c.relnamespace
                   where n.nspname = r.schema_name and c.relname = r.table_name
                     and a.attname = r.column_name || '_key_ref'
                     and a.attnum > 0 and not a.attisdropped
                     and format_type(a.atttypid, null) = 'text')
    or exists (select 1 from pg_attribute a
               join pg_class c on c.oid = a.attrelid
               join pg_namespace n on n.oid = c.relnamespace
               where n.nspname = r.schema_name and c.relname = r.table_name
                 and a.attname = r.column_name
                 and a.attnum > 0 and not a.attisdropped))
union all
select r.schema_name, r.table_name, r.column_name,
       'plaintext registry row does not hit an actual column'
from platform_core.sensitive_field_registry r
where not r.is_field_encrypted
  and not exists (select 1 from pg_attribute a
                  join pg_class c on c.oid = a.attrelid
                  join pg_namespace n on n.oid = c.relnamespace
                  where n.nspname = r.schema_name and c.relname = r.table_name
                    and a.attname = r.column_name
                    and a.attnum > 0 and not a.attisdropped)
order by 1, 2, 3;
