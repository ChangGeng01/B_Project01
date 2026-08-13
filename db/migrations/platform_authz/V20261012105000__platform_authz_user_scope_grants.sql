-- rollback: drop table platform_authz.user_scope_grants;
-- db/migrations/platform_authz/V20261012105000__platform_authz_user_scope_grants.sql
-- 阶段 4 第 20 号迁移：用户范围授予（04-identity-authz.md 表 3-19）。
-- scope_kind 取 PROJECT、CUSTOMER、RECORD 三类；RECORD 时 object_type 必填，
-- 其余两类 object_type 为空。
-- can_reshare 固定为 false 且带 CHECK 约束限定为 false：PRD 附录乙 U-B-07
-- 「共享可否再转授」尚未决策，首版按不可转授实现，一旦决策放开只需放宽该 CHECK。
-- 唯一索引全称 ux_user_scope_grants_legal_entity_id_user_id_scope_kind_scope_ref_id
-- 共 69 字节超过 63 字节上限，按列序缩写为
-- ux_user_scope_grants_le_id_user_id_scope_kind_scope_ref_id（59 字节）：
-- legal_entity_id 缩为 le_id，全称登记数据字典归集成任务。
-- 行级安全策略经 platform_core.apply_le_rls 生成（rls_user_scope_grants_le），不手写。
set role ep_mod_platform_authz;
set lock_timeout = '5s';
set statement_timeout = '30min';

create table if not exists platform_authz.user_scope_grants (
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
  scope_kind text not null,
  object_type text null,
  scope_ref_id uuid not null,
  can_reshare boolean not null default false,
  granted_by uuid not null,
  effective_from date not null,
  effective_to date null,
  constraint pk_user_scope_grants primary key (id),
  constraint ck_user_scope_grants_scope_kind check (
    scope_kind in ('PROJECT', 'CUSTOMER', 'RECORD')),
  constraint ck_user_scope_grants_record_object_type check (
    scope_kind <> 'RECORD' or object_type is not null),
  constraint ck_user_scope_grants_can_reshare check (can_reshare = false),
  constraint ck_user_scope_grants_object_type_len check (
    object_type is null or length(object_type) between 1 and 128)
);

-- 按用户按范围类型取授予集合（索引名取计划 3-19 指定形态）。
create index if not exists ix_user_scope_grants_legal_entity_id_user_id_scope_kind
  on platform_authz.user_scope_grants (legal_entity_id, user_id, scope_kind);
-- 同法人同用户同范围类型同目标唯一（索引名为全称按列序缩写，见文件头注）。
create unique index if not exists ux_user_scope_grants_le_id_user_id_scope_kind_scope_ref_id
  on platform_authz.user_scope_grants (legal_entity_id, user_id, scope_kind, scope_ref_id);
-- 基线时间序索引。
create index if not exists ix_user_scope_grants_legal_entity_id_created_at
  on platform_authz.user_scope_grants (legal_entity_id, created_at);

-- 挂接乐观锁守卫与行级安全策略（rls_user_scope_grants_le），断言基线索引齐备。
select platform_core.attach_table_guards('platform_authz', 'user_scope_grants');
select platform_core.assert_baseline_indexes('platform_authz', 'user_scope_grants', false);

reset role;
