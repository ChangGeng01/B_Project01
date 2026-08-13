-- rollback: drop table platform_core.sessions;
-- db/migrations/platform_core/V20261012092000__platform_core_identity_sessions.sql
-- 阶段 4 第 5 号迁移：会话（04-identity-authz.md 表 3-5）。
-- 令牌为 32 字节随机串 base64url 编码，仅 SHA-256 摘要入库，明文只在响应出现一次；
-- 令牌不用 JWT（基线第 5.6 节）。user_device_row_id 引用 user_devices.id 并建
-- 同 schema 真实外键；active_legal_entity_id 记录会话当前法人。
-- 本表不带 legal_entity_id 列、不建行级安全策略，登记行由第 29 号回填迁移写入。
-- 表经 DO 块创建，理由同阶段 2 第 3 号迁移：不带法人列的登记豁免表形态。
set role ep_mod_platform_core;
set lock_timeout = '5s';
set statement_timeout = '30min';

do $$
begin
  if not exists (select 1 from pg_class c
                 join pg_namespace n on n.oid = c.relnamespace
                 where n.nspname = 'platform_core' and c.relname = 'sessions') then
    execute '
      create table platform_core.sessions (
        id uuid not null,
        security_level smallint not null default 30,
        data_scope_tags text[] not null default ''{}'',
        row_version bigint not null default 1,
        created_at timestamptz not null default now(),
        created_by uuid not null default ''00000000-0000-7000-8000-000000000001'',
        updated_at timestamptz not null default now(),
        updated_by uuid not null default ''00000000-0000-7000-8000-000000000001'',
        user_id uuid not null,
        user_device_row_id uuid not null,
        token_hash bytea not null,
        active_legal_entity_id uuid not null,
        client text not null,
        issued_at timestamptz not null default now(),
        expires_at timestamptz not null,
        idle_expires_at timestamptz not null,
        last_seen_at timestamptz not null default now(),
        revoked_at timestamptz null,
        revoke_reason text null,
        is_breakglass boolean not null default false,
        constraint pk_sessions primary key (id),
        constraint fk_sessions_user_devices foreign key (user_device_row_id)
          references platform_core.user_devices (id) on delete restrict,
        constraint ck_sessions_client check (
          client in (''win'', ''mac'', ''ios'', ''android'', ''portal'', ''ops'')),
        constraint ck_sessions_revoke_reason_len check (
          revoke_reason is null or length(revoke_reason) <= 128)
      )';
  end if;
end $$;

-- 认证路径按令牌摘要定位会话；按用户清理过期会话；批量滑动续期按最近活跃时间。
create unique index if not exists ux_sessions_token_hash
  on platform_core.sessions (token_hash);
create index if not exists ix_sessions_user_id_expires_at
  on platform_core.sessions (user_id, expires_at);
create index if not exists ix_sessions_last_seen_at
  on platform_core.sessions (last_seen_at);
-- 时间序索引：本表无 legal_entity_id 列，取建档时间。
create index if not exists ix_sessions_created_at
  on platform_core.sessions (created_at);

-- 挂接乐观锁守卫（无法人列，不建行级安全策略），并断言基线索引齐备。
select platform_core.attach_table_guards('platform_core', 'sessions');
select platform_core.assert_baseline_indexes('platform_core', 'sessions', false);

reset role;
