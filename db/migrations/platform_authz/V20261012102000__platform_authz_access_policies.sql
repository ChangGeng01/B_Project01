-- rollback: drop table platform_authz.access_policies;
-- db/migrations/platform_authz/V20261012102000__platform_authz_access_policies.sql
-- 阶段 4 第 15 号迁移：访问策略（04-identity-authz.md 表 3-14）。
-- role_id 为空表示适用全部角色；effect 取 ALLOW 或 DENY，显式拒绝优先。
-- condition 是受限的声明式结构，不是表达式语言：只允许对 department、
-- position、project、customer、security_level、data_scope_tag 六个属性做
-- in、not_in、lte、gte、has_tag 五种断言的合取，由 serde 强类型反序列化，
-- 不做字符串求值。空对象 '{}' 表示无附加条件。
-- 行级安全策略经 platform_core.apply_le_rls 生成（rls_access_policies_le），不手写。
set role ep_mod_platform_authz;
set lock_timeout = '5s';
set statement_timeout = '30min';

create table if not exists platform_authz.access_policies (
  id uuid not null,
  legal_entity_id uuid not null,
  security_level smallint not null default 20,
  data_scope_tags text[] not null default '{}',
  row_version bigint not null default 1,
  created_at timestamptz not null default now(),
  created_by uuid not null default '00000000-0000-7000-8000-000000000001',
  updated_at timestamptz not null default now(),
  updated_by uuid not null default '00000000-0000-7000-8000-000000000001',
  role_id uuid null,
  object_type text not null,
  effect text not null,
  priority int not null default 100,
  condition jsonb not null default '{}',
  lifecycle_state text not null,
  constraint pk_access_policies primary key (id),
  constraint ck_access_policies_object_type_len check (length(object_type) between 1 and 128),
  constraint ck_access_policies_effect check (effect in ('ALLOW', 'DENY')),
  constraint ck_access_policies_lifecycle_state check (
    lifecycle_state in ('DRAFT', 'PENDING_RELEASE', 'EFFECTIVE', 'SUPERSEDED', 'RETIRED'))
);

-- 对象级判定按法人、对象类型与效果检索；基线时间序索引。
create index if not exists ix_access_policies_legal_entity_id_object_type_effect
  on platform_authz.access_policies (legal_entity_id, object_type, effect);
create index if not exists ix_access_policies_legal_entity_id_created_at
  on platform_authz.access_policies (legal_entity_id, created_at);

-- 挂接乐观锁守卫与行级安全策略（rls_access_policies_le），断言基线索引齐备。
select platform_core.attach_table_guards('platform_authz', 'access_policies');
select platform_core.assert_baseline_indexes('platform_authz', 'access_policies', false);

reset role;
