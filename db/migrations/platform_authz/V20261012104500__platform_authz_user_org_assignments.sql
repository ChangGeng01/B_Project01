-- rollback: 前置：drop index if exists platform_core.ux_departments_legal_entity_id_id;
-- rollback: 前置：drop index if exists platform_core.ux_positions_legal_entity_id_id;
-- rollback: drop table platform_authz.user_org_assignments;
-- db/migrations/platform_authz/V20261012104500__platform_authz_user_org_assignments.sql
-- 阶段 4 第 19 号迁移：用户组织归属（04-identity-authz.md 表 3-18）。
-- department_id 与 position_id 的外键目标按 A-04 写死为
-- platform_core.departments 与 platform_core.positions，外键在本迁移建立。
-- 跨 schema 复合外键前置：PostgreSQL 要求被引用列集有唯一约束，阶段 2 的
-- departments 与 positions 只有 pk(id)，缺 (legal_entity_id, id) 唯一约束，
-- 故本文件先以 ep_mod_platform_core 身份经 DO 块补齐两条唯一索引
-- （DO 块内不产生 sqlcheck 可见的 CREATE 对象，主要创建对象仍属
-- platform_authz，不触 SQL-010），并授予 ep_mod_platform_authz 对两张表
-- 的 REFERENCES 与 SELECT 及 schema 的 USAGE；随后切回属主角色建表。
-- 行级安全策略经 platform_core.apply_le_rls 生成
-- （rls_user_org_assignments_le），不手写。

-- 一、复合外键前置：补齐被引用唯一约束并授权（在 platform_core 属主角色下执行）。
set role ep_mod_platform_core;
set lock_timeout = '5s';
set statement_timeout = '30min';

do $$
begin
  if not exists (select 1 from pg_class c
                 join pg_namespace n on n.oid = c.relnamespace
                 where n.nspname = 'platform_core'
                   and c.relname = 'ux_departments_legal_entity_id_id') then
    execute 'create unique index ux_departments_legal_entity_id_id
      on platform_core.departments (legal_entity_id, id)';
  end if;
  if not exists (select 1 from pg_class c
                 join pg_namespace n on n.oid = c.relnamespace
                 where n.nspname = 'platform_core'
                   and c.relname = 'ux_positions_legal_entity_id_id') then
    execute 'create unique index ux_positions_legal_entity_id_id
      on platform_core.positions (legal_entity_id, id)';
  end if;
end $$;

grant usage on schema platform_core to ep_mod_platform_authz;
grant select, references on platform_core.departments to ep_mod_platform_authz;
grant select, references on platform_core.positions to ep_mod_platform_authz;

reset role;

-- 二、建表：跨 schema 复合外键 (legal_entity_id, department_id/position_id)
-- 指向 (legal_entity_id, id)，ON DELETE RESTRICT（基线第 3.3 节）。
set role ep_mod_platform_authz;
set lock_timeout = '5s';
set statement_timeout = '30min';

create table if not exists platform_authz.user_org_assignments (
  id uuid not null,
  legal_entity_id uuid not null,
  security_level smallint not null default 20,
  data_scope_tags text[] not null default '{}',
  row_version bigint not null default 1,
  created_at timestamptz not null default now(),
  created_by uuid not null default '00000000-0000-7000-8000-000000000001',
  updated_at timestamptz not null default now(),
  updated_by uuid not null default '00000000-0000-7000-8000-000000000001',
  user_id uuid not null,
  department_id uuid not null,
  position_id uuid not null,
  effective_from date not null,
  effective_to date null,
  constraint pk_user_org_assignments primary key (id),
  constraint fk_user_org_assignments_departments
    foreign key (legal_entity_id, department_id)
    references platform_core.departments (legal_entity_id, id) on delete restrict,
  constraint fk_user_org_assignments_positions
    foreign key (legal_entity_id, position_id)
    references platform_core.positions (legal_entity_id, id) on delete restrict
);

-- 按用户取组织归属；按部门展开在岗人员；基线时间序索引。
create index if not exists ix_user_org_assignments_legal_entity_id_user_id
  on platform_authz.user_org_assignments (legal_entity_id, user_id);
create index if not exists ix_user_org_assignments_legal_entity_id_department_id
  on platform_authz.user_org_assignments (legal_entity_id, department_id);
create index if not exists ix_user_org_assignments_legal_entity_id_created_at
  on platform_authz.user_org_assignments (legal_entity_id, created_at);

-- 挂接乐观锁守卫与行级安全策略（rls_user_org_assignments_le），断言基线索引齐备。
select platform_core.attach_table_guards('platform_authz', 'user_org_assignments');
select platform_core.assert_baseline_indexes('platform_authz', 'user_org_assignments', false);

reset role;
