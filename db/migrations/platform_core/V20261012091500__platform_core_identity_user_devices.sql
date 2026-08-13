-- rollback: 前置：按版本号逆序先回退 sessions（其 user_device_row_id 引用本表）；
-- rollback: drop table platform_core.user_devices;
-- db/migrations/platform_core/V20261012091500__platform_core_identity_user_devices.sql
-- 阶段 4 第 4 号迁移：设备登记（04-identity-authz.md 表 3-4）。
-- client 六值与基线第 5.6 节 X-Client 头、ClientKind 六变体一一对应；
-- restricted_legal_entity_id 非空表示该设备只能用于该法人，安全上下文建立时
-- 取用户授权集合与该限定的交集（规格第 7.7 章）。
-- device_id 与基线第 5.6 节 X-Device-Id 头同域，长度 1 至 64。
-- 本表不带 legal_entity_id 列、不建行级安全策略，登记行由第 29 号回填迁移写入。
-- 表经 DO 块创建，理由同阶段 2 第 3 号迁移：不带法人列的登记豁免表形态。
set role ep_mod_platform_core;
set lock_timeout = '5s';
set statement_timeout = '30min';

do $$
begin
  if not exists (select 1 from pg_class c
                 join pg_namespace n on n.oid = c.relnamespace
                 where n.nspname = 'platform_core' and c.relname = 'user_devices') then
    execute '
      create table platform_core.user_devices (
        id uuid not null,
        security_level smallint not null default 30,
        data_scope_tags text[] not null default ''{}'',
        row_version bigint not null default 1,
        created_at timestamptz not null default now(),
        created_by uuid not null default ''00000000-0000-7000-8000-000000000001'',
        updated_at timestamptz not null default now(),
        updated_by uuid not null default ''00000000-0000-7000-8000-000000000001'',
        user_id uuid not null,
        device_id text not null,
        client text not null,
        public_key bytea null,
        attestation_ref text null,
        restricted_legal_entity_id uuid null,
        status text not null,
        registered_at timestamptz not null default now(),
        revoked_at timestamptz null,
        last_seen_at timestamptz null,
        constraint pk_user_devices primary key (id),
        constraint ck_user_devices_device_id_len check (length(device_id) between 1 and 64),
        constraint ck_user_devices_client check (
          client in (''win'', ''mac'', ''ios'', ''android'', ''portal'', ''ops'')),
        constraint ck_user_devices_status check (
          status in (''PENDING'', ''ACTIVE'', ''REVOKED''))
      )';
  end if;
end $$;

-- 设备标识全库唯一；按用户定位在册设备。
create unique index if not exists ux_user_devices_device_id
  on platform_core.user_devices (device_id);
create index if not exists ix_user_devices_user_id_status
  on platform_core.user_devices (user_id, status);
-- 时间序索引：本表无 legal_entity_id 列，取建档时间。
create index if not exists ix_user_devices_created_at
  on platform_core.user_devices (created_at);

-- 挂接乐观锁守卫（无法人列，不建行级安全策略），并断言基线索引齐备。
select platform_core.attach_table_guards('platform_core', 'user_devices');
select platform_core.assert_baseline_indexes('platform_core', 'user_devices', false);

reset role;
