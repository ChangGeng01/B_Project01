-- rollback: 前置：按版本号逆序先回退引用本表的 role_permission_grants、
-- rollback: user_role_grants、approval_chain_nodes 与第 27 号回填迁移；
-- rollback: drop table platform_authz.roles;
-- db/migrations/platform_authz/V20261012101000__platform_authz_roles.sql
-- 阶段 4 第 13 号迁移：角色（04-identity-authz.md 表 3-12，档案类）。
-- 角色一律按法人建立，不做跨法人的全局角色：全局角色会立刻在这张表上制造
-- 一处需要绕过行级策略的读路径，而基线第 3.8 节不允许任何绕过。
-- code 字符集取阶段 1 冻结的 RoleCode 口径（大写字母、数字、下划线，长度
-- 1 至 64）：与计划 04:L282 的小写加点口径冲突时以冻结实现为准
-- （crates/foundation/src/security/context.rs 与 xtask archcheck frozen 断言）。
-- duty_class 六值 SYSTEM、DATA、SECURITY、AUDIT、KEY、CONFIG，业务角色为空。
-- 行级安全策略经 platform_core.apply_le_rls 生成（rls_roles_le），不手写。
set role ep_mod_platform_authz;
set lock_timeout = '5s';
set statement_timeout = '30min';

create table if not exists platform_authz.roles (
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
  duty_class text null,
  is_portal_role boolean not null default false,
  lifecycle_state text not null,
  retired_at timestamptz null,
  is_active boolean not null default true,
  deactivated_at timestamptz null,
  constraint pk_roles primary key (id),
  constraint ux_roles_legal_entity_id_code unique (legal_entity_id, code),
  constraint ck_roles_code_fmt check (code ~ '^[A-Z][A-Z0-9_]{0,63}$'),
  constraint ck_roles_name_len check (length(name) between 1 and 200),
  constraint ck_roles_duty_class check (
    duty_class is null or duty_class in (
      'SYSTEM', 'DATA', 'SECURITY', 'AUDIT', 'KEY', 'CONFIG')),
  constraint ck_roles_lifecycle_state check (
    lifecycle_state in ('DRAFT', 'PENDING_RELEASE', 'EFFECTIVE', 'SUPERSEDED', 'RETIRED'))
);

-- 基线时间序索引。
create index if not exists ix_roles_legal_entity_id_created_at
  on platform_authz.roles (legal_entity_id, created_at);

-- 挂接乐观锁守卫与行级安全策略（rls_roles_le），断言基线索引齐备。
select platform_core.attach_table_guards('platform_authz', 'roles');
select platform_core.assert_baseline_indexes('platform_authz', 'roles', false);

reset role;
