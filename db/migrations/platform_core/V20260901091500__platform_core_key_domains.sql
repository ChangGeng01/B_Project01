-- rollback: drop table platform_core.key_domains;
-- rollback: 前置：第 5 号迁移的 data_keys 外键引用本表，须先回退第 5 号迁移。
-- db/migrations/platform_core/V20260901091500__platform_core_key_domains.sql
-- 密钥域表（阶段 2 计划第 3.5 节表二、第 4.2 节状态机载体）。
-- domain_kind 首版只放行 LEGAL_ENTITY，GROUP_SHARED 为后续预留且当前不放行；
-- state 取 PROVISIONING/ACTIVE/DESTROY_PLANNED/DESTROYED，合法迁移六条见第 4.2 节。
-- kek_ref 形如 kms://builtin/le/<uuid> 或 kms://hsm/slot0/le/<uuid>。
-- security_level 默认 40（机密）。行级安全策略经 apply_le_rls 生成，不手写。
set role ep_mod_platform_core;
set lock_timeout = '5s';
set statement_timeout = '30min';

create table if not exists platform_core.key_domains (
  id uuid not null,
  legal_entity_id uuid not null,
  security_level smallint not null default 40,
  data_scope_tags text[] not null default '{}',
  row_version bigint not null default 1,
  created_at timestamptz not null default now(),
  created_by uuid not null default '00000000-0000-7000-8000-000000000001',
  updated_at timestamptz not null default now(),
  updated_by uuid not null default '00000000-0000-7000-8000-000000000001',
  domain_kind text not null,
  state text not null,
  kek_ref text not null,
  kek_version int not null default 1,
  provisioned_at timestamptz null,
  destroy_planned_at timestamptz null,
  destroyed_at timestamptz null,
  destroy_evidence_ref text null,
  constraint pk_key_domains primary key (id),
  constraint ck_key_domains_kind check (domain_kind in ('LEGAL_ENTITY')),
  constraint ck_key_domains_state check (state in ('PROVISIONING', 'ACTIVE', 'DESTROY_PLANNED', 'DESTROYED')),
  constraint ck_key_domains_kek_ref_fmt check (
    kek_ref ~ '^(kms://builtin/le/|kms://hsm/slot0/le/)[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}$'),
  constraint ck_key_domains_kek_version_pos check (kek_version > 0)
);

-- 同法人同 kind 至多一个域：唯一键即状态机准入判据的数据库侧强制。
create unique index if not exists ux_key_domains_legal_entity_id_domain_kind
  on platform_core.key_domains (legal_entity_id, domain_kind);
-- 基线时间序索引。
create index if not exists ix_key_domains_legal_entity_id_created_at
  on platform_core.key_domains (legal_entity_id, created_at);

-- 挂接乐观锁守卫与行级安全策略（rls_key_domains_le），断言基线索引齐备。
select platform_core.attach_table_guards('platform_core', 'key_domains');
select platform_core.assert_baseline_indexes('platform_core', 'key_domains', false);

reset role;
