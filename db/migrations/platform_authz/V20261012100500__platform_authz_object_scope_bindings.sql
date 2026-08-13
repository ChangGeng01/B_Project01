-- rollback: 前置：先删除本文件写入的三行 platform 自身对象登记；
-- rollback: drop table platform_authz.object_scope_bindings;
-- db/migrations/platform_authz/V20261012100500__platform_authz_object_scope_bindings.sql
-- 阶段 4 第 12 号迁移：对象范围绑定（04-identity-authz.md 表 3-11）。
-- 记录级判定的落点：各业务模块在其阶段的 wiring 中登记自己对象的范围锚列，
-- 未登记的对象类型在记录级判定阶段一律拒绝，不默认放行。
-- 本阶段只登记 platform 自身的三个对象类型，同文件写入三行登记：
--   platform.user_accounts 锚列取 id（账号自身即责任人）；
--   platform.roles 无用户或部门锚列（角色是配置对象，记录级范围取 All）；
--   platform.high_risk_requests 锚列取 initiator_user_id（发起人为责任人）。
-- 与计划表 3-11 的形态差异：计划写 object_type 为 pk，但 db/checks/01 对不带
-- 法人列的非仅追加表要求第 1 列为 id，故本表以 id 为 pk、object_type 建唯一
-- 索引 ux_object_scope_bindings_object_type 承载自然键语义，行为等价。
-- 本表不带 legal_entity_id 列、不建行级安全策略，登记行由第 29 号回填迁移写入。
-- 表经 DO 块创建，理由同阶段 2 第 3 号迁移：不带法人列的登记豁免表形态。
set role ep_mod_platform_authz;
set lock_timeout = '5s';
set statement_timeout = '30min';

do $$
begin
  if not exists (select 1 from pg_class c
                 join pg_namespace n on n.oid = c.relnamespace
                 where n.nspname = 'platform_authz' and c.relname = 'object_scope_bindings') then
    execute '
      create table platform_authz.object_scope_bindings (
        id uuid not null,
        security_level smallint not null default 20,
        data_scope_tags text[] not null default ''{}'',
        row_version bigint not null default 1,
        created_at timestamptz not null default now(),
        created_by uuid not null default ''00000000-0000-7000-8000-000000000001'',
        updated_at timestamptz not null default now(),
        updated_by uuid not null default ''00000000-0000-7000-8000-000000000001'',
        object_type text not null,
        schema_name text not null,
        table_name text not null,
        owner_user_col text null,
        owning_dept_col text null,
        project_col text null,
        customer_col text null,
        security_level_col text not null default ''security_level'',
        constraint pk_object_scope_bindings primary key (id),
        constraint ux_object_scope_bindings_object_type unique (object_type),
        constraint ck_object_scope_bindings_object_type_len check (
          length(object_type) between 1 and 128)
      )';
  end if;
end $$;

-- 时间序索引：本表无 legal_entity_id 列，取建档时间。
create index if not exists ix_object_scope_bindings_created_at
  on platform_authz.object_scope_bindings (created_at);

-- platform 自身三对象的范围锚列登记。
insert into platform_authz.object_scope_bindings
  (id, object_type, schema_name, table_name, owner_user_col,
   owning_dept_col, project_col, customer_col)
values
  ('00000000-0000-7000-8000-000000000501', 'platform.user_accounts',
   'platform_core', 'user_accounts', 'id', null, null, null),
  ('00000000-0000-7000-8000-000000000502', 'platform.roles',
   'platform_authz', 'roles', null, null, null, null),
  ('00000000-0000-7000-8000-000000000503', 'platform.high_risk_requests',
   'platform_authz', 'high_risk_requests', 'initiator_user_id', null, null, null)
on conflict on constraint ux_object_scope_bindings_object_type do nothing;

-- 挂接乐观锁守卫（无法人列，不建行级安全策略），并断言基线索引齐备。
select platform_core.attach_table_guards('platform_authz', 'object_scope_bindings');
select platform_core.assert_baseline_indexes('platform_authz', 'object_scope_bindings', false);

reset role;
