-- rollback: drop table platform_authz.role_permission_grants;
-- db/migrations/platform_authz/V20261012101500__platform_authz_role_permission_grants.sql
-- 阶段 4 第 14 号迁移：角色权限授予（04-identity-authz.md 表 3-13）。
-- action 为六动作之一：VIEW、CREATE、UPDATE、SUBMIT、APPROVE、EXPORT。
-- 唯一索引全称 ux_role_permission_grants_legal_entity_id_role_id_permission_item_code_action
-- 共 80 字节超过 63 字节上限，按列序缩写为
-- ux_role_permission_grants_le_id_role_id_perm_item_code_action（61 字节）：
-- legal_entity_id 缩为 le_id、permission_item_code 缩为 perm_item_code，
-- 全称登记数据字典归集成任务。
-- 行级安全策略经 platform_core.apply_le_rls 生成（rls_role_permission_grants_le），不手写。
set role ep_mod_platform_authz;
set lock_timeout = '5s';
set statement_timeout = '30min';

create table if not exists platform_authz.role_permission_grants (
  id uuid not null,
  legal_entity_id uuid not null,
  security_level smallint not null default 20,
  data_scope_tags text[] not null default '{}',
  row_version bigint not null default 1,
  created_at timestamptz not null default now(),
  created_by uuid not null default '00000000-0000-7000-8000-000000000001',
  updated_at timestamptz not null default now(),
  updated_by uuid not null default '00000000-0000-7000-8000-000000000001',
  role_id uuid not null,
  permission_item_code text not null,
  action text not null,
  constraint pk_role_permission_grants primary key (id),
  constraint ck_role_permission_grants_action check (
    action in ('VIEW', 'CREATE', 'UPDATE', 'SUBMIT', 'APPROVE', 'EXPORT')),
  constraint ck_role_permission_grants_code_len check (
    length(permission_item_code) between 1 and 128)
);

-- 同法人同角色同权限项同动作唯一（索引名为全称按列序缩写，见文件头注）。
create unique index if not exists ux_role_permission_grants_le_id_role_id_perm_item_code_action
  on platform_authz.role_permission_grants (legal_entity_id, role_id, permission_item_code, action);
-- 基线时间序索引。
create index if not exists ix_role_permission_grants_legal_entity_id_created_at
  on platform_authz.role_permission_grants (legal_entity_id, created_at);

-- 挂接乐观锁守卫与行级安全策略（rls_role_permission_grants_le），断言基线索引齐备。
select platform_core.attach_table_guards('platform_authz', 'role_permission_grants');
select platform_core.assert_baseline_indexes('platform_authz', 'role_permission_grants', false);

reset role;
