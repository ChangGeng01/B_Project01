-- rollback: drop table platform_core.data_keys;
-- db/migrations/platform_core/V20260901092000__platform_core_data_keys.sql
-- 数据密钥表（阶段 2 计划第 3.5 节表三、第 4.3 节 EPC1 信封的 DEK 台账）。
-- purpose 取 FIELD/BLIND_INDEX/ATTACHMENT/ARCHIVE；algorithm 取 AES_256_GCM/HMAC_SHA256；
-- state 取 ACTIVE/RETIRING/RETIRED/DESTROYED。
-- 唯一约束 ux_data_keys_domain_purpose_scope_version 在
-- (key_domain_id, purpose, security_level_scope, version) 四列上：该名称 50 字节，
-- 未达 PostgreSQL 63 字节标识符上限，按全称保留（阶段 2 规格报告的缩写预案不触发）。
-- 首版不使用部分索引：取当前有效密钥按该 ux 前缀定位后 order by version desc limit 1。
-- security_level 默认 40（机密）。行级安全策略经 apply_le_rls 生成，不手写。
set role ep_mod_platform_core;
set lock_timeout = '5s';
set statement_timeout = '30min';

create table if not exists platform_core.data_keys (
  id uuid not null,
  legal_entity_id uuid not null,
  security_level smallint not null default 40,
  data_scope_tags text[] not null default '{}',
  row_version bigint not null default 1,
  created_at timestamptz not null default now(),
  created_by uuid not null default '00000000-0000-7000-8000-000000000001',
  updated_at timestamptz not null default now(),
  updated_by uuid not null default '00000000-0000-7000-8000-000000000001',
  key_domain_id uuid not null,
  purpose text not null,
  security_level_scope smallint not null,
  version int not null,
  algorithm text not null,
  wrapped_key bytea not null,
  wrap_kek_version int not null,
  state text not null,
  activated_at timestamptz not null,
  retiring_at timestamptz null,
  retired_at timestamptz null,
  destroyed_at timestamptz null,
  constraint pk_data_keys primary key (id),
  constraint fk_data_keys_key_domains foreign key (key_domain_id)
    references platform_core.key_domains (id) on delete restrict,
  constraint ck_data_keys_purpose check (purpose in ('FIELD', 'BLIND_INDEX', 'ATTACHMENT', 'ARCHIVE')),
  constraint ck_data_keys_level check (security_level_scope in (10, 20, 30, 40)),
  constraint ck_data_keys_version_pos check (version > 0),
  constraint ck_data_keys_alg check (algorithm in ('AES_256_GCM', 'HMAC_SHA256')),
  constraint ck_data_keys_state check (state in ('ACTIVE', 'RETIRING', 'RETIRED', 'DESTROYED')),
  constraint ux_data_keys_domain_purpose_scope_version
    unique (key_domain_id, purpose, security_level_scope, version)
);

-- 基线时间序索引。
create index if not exists ix_data_keys_legal_entity_id_created_at
  on platform_core.data_keys (legal_entity_id, created_at);

-- 挂接乐观锁守卫与行级安全策略（rls_data_keys_le），断言基线索引齐备。
select platform_core.attach_table_guards('platform_core', 'data_keys');
select platform_core.assert_baseline_indexes('platform_core', 'data_keys', false);

reset role;
