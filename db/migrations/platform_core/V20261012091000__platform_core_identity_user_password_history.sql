-- rollback: 前置：先删除本文件写入 append_only_registry 的登记行；
-- rollback: drop table platform_core.user_password_history;
-- db/migrations/platform_core/V20261012091000__platform_core_identity_user_password_history.sql
-- 阶段 4 第 3 号迁移：口令历史（04-identity-authz.md 表 3-3，仅追加）。
-- 仅追加表：不带 row_version、updated_at、updated_by，也不带 reverses_id——
-- 口令历史没有冲销或更正语义（第 12.1 节新增决定 5）。
-- 列序按 db/checks/01 对仅追加表的口径：id、security_level、data_scope_tags、
-- created_at、created_by 居首（表 3-3 正文列清单不含 security_level 与
-- data_scope_tags 两件公共列，按基线第 4 节同缺口径补齐并居前）。
-- 本表不带 legal_entity_id 列、不建行级安全策略，登记行由第 29 号回填迁移写入。
-- 表经 DO 块创建：SQL-008 九件套判据以带法人列的表为对象，仅追加表按阶段 2
-- 第 6 号迁移先例以 DO 块形态建表，公共列位置在 db/checks/01 按登记表核对；
-- 同文件先登记 append_only_registry（mode 取 APPEND_ONLY），使 check 01 的
-- 仅追加豁免与 attach_table_guards 的 trg_user_password_history_append_only
-- 挂接成立（登记与触发器一致性由 db/checks/append_only_consistency 兜底）。
set role ep_mod_platform_core;
set lock_timeout = '5s';
set statement_timeout = '30min';

do $$
begin
  if not exists (select 1 from pg_class c
                 join pg_namespace n on n.oid = c.relnamespace
                 where n.nspname = 'platform_core' and c.relname = 'user_password_history') then
    execute '
      create table platform_core.user_password_history (
        id uuid not null,
        security_level smallint not null default 40,
        data_scope_tags text[] not null default ''{}'',
        created_at timestamptz not null default now(),
        created_by uuid not null default ''00000000-0000-7000-8000-000000000001'',
        user_id uuid not null,
        verifier text not null,
        constraint pk_user_password_history primary key (id)
      )';
  end if;
end $$;

-- 按用户取口令历史时间序，供 history_size 五代的重复性校验。
create index if not exists ix_user_password_history_user_id_created_at
  on platform_core.user_password_history (user_id, created_at);

-- 仅追加登记：mode 取 APPEND_ONLY，mutable_columns 必须为空。
insert into platform_core.append_only_registry
  (id, schema_name, table_name, mode)
values
  ('00000000-0000-7000-8000-000000000101', 'platform_core', 'user_password_history',
   'APPEND_ONLY')
on conflict on constraint ux_append_only_registry_schema_table do nothing;

-- 挂接仅追加守卫（据登记挂 trg_user_password_history_append_only；无法人列不建策略）。
select platform_core.attach_table_guards('platform_core', 'user_password_history');

reset role;
