-- rollback: drop table platform_authz.user_legal_entity_grants;
-- db/migrations/platform_authz/V20261012103500__platform_authz_user_legal_entity_grants.sql
-- 阶段 4 第 17 号迁移：用户法人授权（04-identity-authz.md 表 3-16）。
-- 这是全系统唯一决定「某用户能不能进某法人」的表；它自身受策略约束，
-- 因此法人 A 的管理员无法看到也无法写入法人 B 的授权行。
-- 九张身份主体表的法人可见性内联落点：任何列出用户的查询一律与本表内联。
-- granted_from 与 granted_to 为生效日期区间，granted_to 为空表示长期有效。
-- 行级安全策略经 platform_core.apply_le_rls 生成
-- （rls_user_legal_entity_grants_le），不手写。
set role ep_mod_platform_authz;
set lock_timeout = '5s';
set statement_timeout = '30min';

create table if not exists platform_authz.user_legal_entity_grants (
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
  granted_from date not null,
  granted_to date null,
  granted_by uuid not null,
  constraint pk_user_legal_entity_grants primary key (id),
  constraint ux_user_legal_entity_grants_legal_entity_id_user_id
    unique (legal_entity_id, user_id)
);

-- 基线时间序索引。
create index if not exists ix_user_legal_entity_grants_legal_entity_id_created_at
  on platform_authz.user_legal_entity_grants (legal_entity_id, created_at);

-- 挂接乐观锁守卫与行级安全策略（rls_user_legal_entity_grants_le），断言基线索引齐备。
select platform_core.attach_table_guards('platform_authz', 'user_legal_entity_grants');
select platform_core.assert_baseline_indexes('platform_authz', 'user_legal_entity_grants', false);

reset role;
