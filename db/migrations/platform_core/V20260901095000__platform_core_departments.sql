-- rollback: drop table platform_core.departments;
-- db/migrations/platform_core/V20260901095000__platform_core_departments.sql
-- 部门表（阶段 2 计划第 3.5 节表九）。
-- organization_id 与 parent_department_id 均为同 schema 真实外键，ON DELETE RESTRICT；
-- level_no 大于 0，与闭包表（第 13 号迁移）在同一事务内维护，写入契约见第 4.8 节。
-- 行级安全策略经 apply_le_rls 生成（rls_departments_le），不手写。
set role ep_mod_platform_core;
set lock_timeout = '5s';
set statement_timeout = '30min';

create table if not exists platform_core.departments (
  id uuid not null,
  legal_entity_id uuid not null,
  security_level smallint not null default 20,
  data_scope_tags text[] not null default '{}',
  row_version bigint not null default 1,
  created_at timestamptz not null default now(),
  created_by uuid not null default '00000000-0000-7000-8000-000000000001',
  updated_at timestamptz not null default now(),
  updated_by uuid not null default '00000000-0000-7000-8000-000000000001',
  organization_id uuid not null,
  code text not null,
  name text not null,
  parent_department_id uuid null,
  level_no smallint not null,
  is_active boolean not null default true,
  deactivated_at timestamptz null,
  constraint pk_departments primary key (id),
  constraint fk_departments_organizations foreign key (organization_id)
    references platform_core.organizations (id) on delete restrict,
  constraint fk_departments_parent foreign key (parent_department_id)
    references platform_core.departments (id) on delete restrict,
  constraint ck_departments_name_len check (length(name) between 1 and 200),
  constraint ck_departments_level_no check (level_no > 0)
);

-- 法人内部门码唯一。
create unique index if not exists ux_departments_legal_entity_id_code
  on platform_core.departments (legal_entity_id, code);
-- 基线时间序索引。
create index if not exists ix_departments_legal_entity_id_created_at
  on platform_core.departments (legal_entity_id, created_at);
-- 子部门定位索引：按上级部门在法人内展开。
create index if not exists ix_departments_legal_entity_id_parent_department_id
  on platform_core.departments (legal_entity_id, parent_department_id);

-- 挂接乐观锁守卫与行级安全策略（rls_departments_le），断言基线索引齐备。
select platform_core.attach_table_guards('platform_core', 'departments');
select platform_core.assert_baseline_indexes('platform_core', 'departments', false);

reset role;
