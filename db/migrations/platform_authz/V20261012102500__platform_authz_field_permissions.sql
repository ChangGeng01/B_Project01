-- rollback: drop table platform_authz.field_permissions;
-- db/migrations/platform_authz/V20261012102500__platform_authz_field_permissions.sql
-- 阶段 4 第 16 号迁移：字段权限（04-identity-authz.md 表 3-15）。
-- visibility 四值 HIDDEN、MASKED、READ、WRITE；mask_style 三值 FULL、
-- KEEP_LAST_4、KEEP_DOMAIN（仅 MASKED 时使用，U-B-06 临时取值）。
-- 字段在 field_permissions 中无授权行时按默认拒绝处理，不进入响应键集合。
-- 唯一索引全称 ux_field_permissions_legal_entity_id_role_id_object_type_field_name
-- 共 65 字节超过 63 字节上限，按列序缩写为
-- ux_field_permissions_le_id_role_id_obj_type_field_name（54 字节）：
-- legal_entity_id 缩为 le_id、object_type 缩为 obj_type，全称登记数据字典归集成任务。
-- 行级安全策略经 platform_core.apply_le_rls 生成（rls_field_permissions_le），不手写。
set role ep_mod_platform_authz;
set lock_timeout = '5s';
set statement_timeout = '30min';

create table if not exists platform_authz.field_permissions (
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
  object_type text not null,
  field_name text not null,
  visibility text not null,
  mask_style text null,
  constraint pk_field_permissions primary key (id),
  constraint ck_field_permissions_object_type_len check (length(object_type) between 1 and 128),
  constraint ck_field_permissions_field_name_len check (length(field_name) between 1 and 128),
  constraint ck_field_permissions_visibility check (
    visibility in ('HIDDEN', 'MASKED', 'READ', 'WRITE')),
  constraint ck_field_permissions_mask_style check (
    mask_style is null or mask_style in ('FULL', 'KEEP_LAST_4', 'KEEP_DOMAIN'))
);

-- 同法人同角色同对象同字段唯一（索引名为全称按列序缩写，见文件头注）。
create unique index if not exists ux_field_permissions_le_id_role_id_obj_type_field_name
  on platform_authz.field_permissions (legal_entity_id, role_id, object_type, field_name);
-- 基线时间序索引。
create index if not exists ix_field_permissions_legal_entity_id_created_at
  on platform_authz.field_permissions (legal_entity_id, created_at);

-- 挂接乐观锁守卫与行级安全策略（rls_field_permissions_le），断言基线索引齐备。
select platform_core.attach_table_guards('platform_authz', 'field_permissions');
select platform_core.assert_baseline_indexes('platform_authz', 'field_permissions', false);

reset role;
