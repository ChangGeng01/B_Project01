-- rollback: drop table platform_authz.user_role_grants;
-- db/migrations/platform_authz/V20261012104000__platform_authz_user_role_grants.sql
-- 阶段 4 第 18 号迁移：用户角色授予（04-identity-authz.md 表 3-17）。
-- effective_from 与 effective_to 为生效日期区间，权限按生效日期切换；
-- 调岗改权与离职停用通过撤销该用户全部会话使其立即生效。
-- 用户维度授权集合在会话建立时读取一次并冻结在 SecurityContext，会话有效期
-- 内不重读。
-- 唯一索引全称 ux_user_role_grants_legal_entity_id_user_id_role_id_effective_from
-- 共 66 字节超过 63 字节上限，按列序缩写为
-- ux_user_role_grants_le_id_user_id_role_id_eff_from（50 字节）：
-- legal_entity_id 缩为 le_id、effective_from 缩为 eff_from，
-- 全称登记数据字典归集成任务。
-- 行级安全策略经 platform_core.apply_le_rls 生成（rls_user_role_grants_le），不手写。
set role ep_mod_platform_authz;
set lock_timeout = '5s';
set statement_timeout = '30min';

create table if not exists platform_authz.user_role_grants (
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
  role_id uuid not null,
  effective_from date not null,
  effective_to date null,
  granted_by uuid not null,
  constraint pk_user_role_grants primary key (id)
);

-- 同法人同用户同角色同生效起始日唯一（索引名为全称按列序缩写，见文件头注）。
create unique index if not exists ux_user_role_grants_le_id_user_id_role_id_eff_from
  on platform_authz.user_role_grants (legal_entity_id, user_id, role_id, effective_from);
-- 按用户取角色集合；基线时间序索引。
create index if not exists ix_user_role_grants_legal_entity_id_user_id
  on platform_authz.user_role_grants (legal_entity_id, user_id);
create index if not exists ix_user_role_grants_legal_entity_id_created_at
  on platform_authz.user_role_grants (legal_entity_id, created_at);

-- 挂接乐观锁守卫与行级安全策略（rls_user_role_grants_le），断言基线索引齐备。
select platform_core.attach_table_guards('platform_authz', 'user_role_grants');
select platform_core.assert_baseline_indexes('platform_authz', 'user_role_grants', false);

reset role;
