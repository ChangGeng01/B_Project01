-- rollback: drop table platform_authz.approval_chains;
-- db/migrations/platform_authz/V20261012110000__platform_authz_approval_chains.sql
-- 阶段 4 第 22 号迁移：审批链定义（04-identity-authz.md 表 3-21，档案类）。
-- scenario 的取值域包含六类高风险操作码，也包含业务模块登记的场景码，
-- 故不收敛为封闭枚举；T0 四条单节点审批链由 ep-datagen 最小样本生成。
-- version_no 为同一 code 下的版本序号，自 1 起；lifecycle_state 沿用档案类
-- 五态生命周期（与 roles 同域）。
-- 行级安全策略经 platform_core.apply_le_rls 生成（rls_approval_chains_le），不手写。
set role ep_mod_platform_authz;
set lock_timeout = '5s';
set statement_timeout = '30min';

create table if not exists platform_authz.approval_chains (
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
  scenario text not null,
  version_no int not null default 1,
  lifecycle_state text not null,
  is_active boolean not null default true,
  deactivated_at timestamptz null,
  constraint pk_approval_chains primary key (id),
  constraint ux_approval_chains_legal_entity_id_code_version_no
    unique (legal_entity_id, code, version_no),
  constraint ck_approval_chains_code_len check (length(code) between 1 and 64),
  constraint ck_approval_chains_code_fmt check (code ~ '^[A-Z][A-Z0-9_]{0,63}$'),
  constraint ck_approval_chains_name_len check (length(name) between 1 and 200),
  constraint ck_approval_chains_scenario_len check (length(scenario) between 1 and 128),
  constraint ck_approval_chains_version_no check (version_no >= 1),
  constraint ck_approval_chains_lifecycle_state check (
    lifecycle_state in ('DRAFT', 'PENDING_RELEASE', 'EFFECTIVE', 'SUPERSEDED', 'RETIRED'))
);

-- 基线时间序索引。
create index if not exists ix_approval_chains_legal_entity_id_created_at
  on platform_authz.approval_chains (legal_entity_id, created_at);

-- 挂接乐观锁守卫与行级安全策略（rls_approval_chains_le），断言基线索引齐备。
select platform_core.attach_table_guards('platform_authz', 'approval_chains');
select platform_core.assert_baseline_indexes('platform_authz', 'approval_chains', false);

reset role;
