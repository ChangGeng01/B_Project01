-- rollback: drop table platform_authz.authz_config_versions;
-- db/migrations/platform_authz/V20261012111500__platform_authz_authz_config_versions.sql
-- 阶段 4 第 25 号迁移：授权配置版本（04-identity-authz.md 表 3-24）。
-- state 四态 DRAFT、STAGED、EFFECTIVE、ROLLED_BACK；同法人同时只有一个
-- EFFECTIVE，由第 5 节「配置版本生效」事务对该法人当前生效版本行 FOR UPDATE
-- 保证，表上不建部分唯一索引承载该不变量。
-- checksum 为该版本配置行的现算摘要（第 27 号回填迁移写入时按所写配置行现算）；
-- 快照重载的唯一路径是 core-server 按 EP__AUTHZ__SNAPSHOT__POLL_INTERVAL_MS
-- 轮询本表 EFFECTIVE 版本号。
-- release_bundle_ref 为阶段 3b 发布通道发布包的逻辑引用，本阶段写 null。
-- 行级安全策略经 platform_core.apply_le_rls 生成（rls_authz_config_versions_le），不手写。
set role ep_mod_platform_authz;
set lock_timeout = '5s';
set statement_timeout = '30min';

create table if not exists platform_authz.authz_config_versions (
  id uuid not null,
  legal_entity_id uuid not null,
  security_level smallint not null default 20,
  data_scope_tags text[] not null default '{}',
  row_version bigint not null default 1,
  created_at timestamptz not null default now(),
  created_by uuid not null default '00000000-0000-7000-8000-000000000001',
  updated_at timestamptz not null default now(),
  updated_by uuid not null default '00000000-0000-7000-8000-000000000001',
  version_no bigint not null,
  state text not null,
  release_bundle_ref uuid null,
  checksum bytea not null,
  published_by uuid null,
  published_at timestamptz null,
  constraint pk_authz_config_versions primary key (id),
  constraint ux_authz_config_versions_legal_entity_id_version_no
    unique (legal_entity_id, version_no),
  constraint ck_authz_config_versions_version_no check (version_no >= 1),
  constraint ck_authz_config_versions_state check (
    state in ('DRAFT', 'STAGED', 'EFFECTIVE', 'ROLLED_BACK'))
);

-- 按状态取生效版本（快照轮询路径）；基线时间序索引。
create index if not exists ix_authz_config_versions_legal_entity_id_state
  on platform_authz.authz_config_versions (legal_entity_id, state);
create index if not exists ix_authz_config_versions_legal_entity_id_created_at
  on platform_authz.authz_config_versions (legal_entity_id, created_at);

-- 挂接乐观锁守卫与行级安全策略（rls_authz_config_versions_le），断言基线索引齐备。
select platform_core.attach_table_guards('platform_authz', 'authz_config_versions');
select platform_core.assert_baseline_indexes('platform_authz', 'authz_config_versions', false);

reset role;
