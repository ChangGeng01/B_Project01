-- rollback: drop table platform_core.organizations;
-- db/migrations/platform_core/V20260901094500__platform_core_organizations.sql
-- 组织表（阶段 2 计划第 3.5 节表八）。
-- org_kind 取 CORPORATION/BRANCH/DIVISION；parent_organization_id 自引用，
-- ON DELETE RESTRICT：有下级组织的组织不得删除。
-- 行级安全策略经 apply_le_rls 生成（rls_organizations_le），不手写。
set role ep_mod_platform_core;
set lock_timeout = '5s';
set statement_timeout = '30min';

create table if not exists platform_core.organizations (
  id uuid not null,
  legal_entity_id uuid not null,
  security_level smallint not null default 20,
  data_scope_tags text[] not null default '{}',
  row_version bigint not null default 1,
  created_at timestamptz not null default now(),
  created_by uuid not null default '00000000-0000-7000-8000-000000000001',
  updated_at timestamptz not null default now(),
  updated_by uuid not null default '00000000-0000-7000-8000-000000000001',
  code text not null,
  name text not null,
  org_kind text not null,
  parent_organization_id uuid null,
  is_active boolean not null default true,
  constraint pk_organizations primary key (id),
  constraint ck_organizations_name_len check (length(name) between 1 and 200),
  constraint ck_organizations_org_kind check (org_kind in ('CORPORATION', 'BRANCH', 'DIVISION')),
  constraint fk_organizations_parent foreign key (parent_organization_id)
    references platform_core.organizations (id) on delete restrict
);

-- 法人内组织码唯一。
create unique index if not exists ux_organizations_legal_entity_id_code
  on platform_core.organizations (legal_entity_id, code);
-- 基线时间序索引。
create index if not exists ix_organizations_legal_entity_id_created_at
  on platform_core.organizations (legal_entity_id, created_at);

-- 挂接乐观锁守卫与行级安全策略（rls_organizations_le），断言基线索引齐备。
select platform_core.attach_table_guards('platform_core', 'organizations');
select platform_core.assert_baseline_indexes('platform_core', 'organizations', false);

reset role;
