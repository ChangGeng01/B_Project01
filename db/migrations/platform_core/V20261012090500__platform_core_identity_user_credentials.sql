-- rollback: drop table platform_core.user_credentials;
-- db/migrations/platform_core/V20261012090500__platform_core_identity_user_credentials.sql
-- 阶段 4 第 2 号迁移：认证凭据（04-identity-authz.md 表 3-2）。
-- credential_kind 五值；PASSWORD 的 verifier 存 Argon2id 的 PHC 串，X509_CERT 存证书指纹；
-- TOTP 种子只存机密引用 secret://kms/totp/<user_id>#<ver>，种子本体在 KMS。
-- ck_user_credentials_material 按 kind 强制对应载体非空。
-- 本表不带 legal_entity_id 列、不建行级安全策略，登记行由第 29 号回填迁移写入。
-- 表经 DO 块创建，理由同阶段 2 第 3 号迁移：不带法人列的登记豁免表形态。
set role ep_mod_platform_core;
set lock_timeout = '5s';
set statement_timeout = '30min';

do $$
begin
  if not exists (select 1 from pg_class c
                 join pg_namespace n on n.oid = c.relnamespace
                 where n.nspname = 'platform_core' and c.relname = 'user_credentials') then
    execute '
      create table platform_core.user_credentials (
        id uuid not null,
        security_level smallint not null default 40,
        data_scope_tags text[] not null default ''{}'',
        row_version bigint not null default 1,
        created_at timestamptz not null default now(),
        created_by uuid not null default ''00000000-0000-7000-8000-000000000001'',
        updated_at timestamptz not null default now(),
        updated_by uuid not null default ''00000000-0000-7000-8000-000000000001'',
        user_id uuid not null,
        credential_kind text not null,
        verifier text null,
        public_key bytea null,
        credential_handle bytea null,
        secret_ref text null,
        sign_count bigint not null default 0,
        status text not null,
        activated_at timestamptz not null default now(),
        expires_at timestamptz null,
        last_used_at timestamptz null,
        revoked_at timestamptz null,
        constraint pk_user_credentials primary key (id),
        constraint fk_user_credentials_user_accounts foreign key (user_id)
          references platform_core.user_accounts (id) on delete restrict,
        constraint ck_user_credentials_kind check (
          credential_kind in (''PASSWORD'', ''TOTP'', ''WEBAUTHN_PLATFORM'',
                              ''WEBAUTHN_ROAMING'', ''X509_CERT'')),
        constraint ck_user_credentials_status check (
          status in (''ACTIVE'', ''SUSPENDED'', ''REVOKED'', ''EXPIRED'')),
        constraint ck_user_credentials_material check (
          (credential_kind in (''PASSWORD'', ''X509_CERT'') and verifier is not null)
          or (credential_kind in (''WEBAUTHN_PLATFORM'', ''WEBAUTHN_ROAMING'')
              and public_key is not null and credential_handle is not null)
          or (credential_kind = ''TOTP'' and secret_ref is not null))
      )';
  end if;
end $$;

-- 按用户定位凭据种类；WebAuthn credential id 全库唯一（NULL 不受唯一约束限制）。
create index if not exists ix_user_credentials_user_id_credential_kind
  on platform_core.user_credentials (user_id, credential_kind);
create unique index if not exists ux_user_credentials_credential_handle
  on platform_core.user_credentials (credential_handle);
-- 时间序索引：本表无 legal_entity_id 列，取建档时间。
create index if not exists ix_user_credentials_created_at
  on platform_core.user_credentials (created_at);

-- 挂接乐观锁守卫（无法人列，不建行级安全策略），并断言基线索引齐备。
select platform_core.attach_table_guards('platform_core', 'user_credentials');
select platform_core.assert_baseline_indexes('platform_core', 'user_credentials', false);

reset role;
