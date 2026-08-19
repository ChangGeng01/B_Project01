-- db/checks/11_sensitive_field_encryption.sql
-- 阶段 2 计划 §3.9 第 11 项（裁定 A-28）：敏感字段物理列集断言。
-- 通过判据：返回 0 行。执行方：ep-migrate check。
-- 三支，登记侧两支加目录侧一支，与 13_unpoliced_registry.sql 同为双向判据：
-- 一、is_field_encrypted 取真的登记项：物理表必须存在 <column>_enc bytea 与
--     <column>_key_ref text，且不保留同名明文列 <column>；
-- 二、取假的登记项：只断言三元组命中 information_schema 中的实际列；
-- 三、UNREGISTERED_ENCRYPTED_COLUMN（本次补）：物理上存在 <base>_enc bytea 却没有
--     对应的 is_field_encrypted 登记行。缺这一支时，「有人加了加密列但忘了登记」
--     这一类错永远不会被本脚本看见——正是 13 号靠 UNREGISTERED_UNPOLICED_TABLE
--     兜住的那一向。
--
-- 今天本脚本恒返 0 行，成因是**两侧皆空**而不是缺哪一支：
-- platform_core.sensitive_field_registry 零种子行（阶段 4 计划逐字「本阶段对该表
-- 只有读取路径」），迁移里 <col>_enc bytea 与 <col>_key_ref text 各 0 处。
-- 登记侧的生产方已排期（阶段 3b、5、10 各有回填并进了退出条件），届时两侧都会有取值。
-- 另注：本脚本属 ep-migrate check 的十三项，而该命令今天没有任何自动调用方（附录辛第 27 条）。
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
union all
select n.nspname, c.relname,
       left(a.attname, length(a.attname) - 4),
       'UNREGISTERED_ENCRYPTED_COLUMN'
from pg_attribute a
join pg_class c on c.oid = a.attrelid
join pg_namespace n on n.oid = c.relnamespace
where c.relkind = 'r'
  and a.attnum > 0 and not a.attisdropped
  and a.attname like '%\_enc'
  and format_type(a.atttypid, null) = 'bytea'
  and n.nspname in (
    'platform_core','platform_authz','platform_meta','platform_flow',
    'platform_audit','platform_msg','platform_file','platform_ops','ext',
    'mdm','crm','cpq','clm','sales','procure','inventory','costing',
    'project','service','finance','ledger','invoice','portal','reporting')
  and not exists (select 1 from platform_core.sensitive_field_registry r
                  where r.schema_name = n.nspname
                    and r.table_name = c.relname
                    and r.column_name = left(a.attname, length(a.attname) - 4)
                    and r.is_field_encrypted)
order by 1, 2, 3;
