-- rollback: 前置：先删除本文件写入 append_only_registry 的登记行；
-- rollback: drop table platform_core.login_attempts;
-- db/migrations/platform_core/V20261012093000__platform_core_identity_login_attempts.sql
-- 阶段 4 第 7 号迁移：登录尝试（04-identity-authz.md 表 3-7，仅追加）。
-- 仅追加表：不带 row_version、updated_at、updated_by，也不带 reverses_id——
-- 登录尝试没有冲销或更正语义（表 3-7 定义处与第 12.1 节新增决定 5）。
-- login_name 以哈希存储：失败尝试中的登录名可能是攻击者构造的任意串，
-- 明文入库会把一张运行数据表变成半个外部输入落点。
-- 列序按 db/checks/01 对仅追加表的口径：id、security_level、data_scope_tags、
-- created_at、created_by 居首。
-- 本表不带 legal_entity_id 列、不建行级安全策略，登记行由第 29 号回填迁移写入。
-- 表经 DO 块创建，理由同第 3 号迁移：仅追加表形态，公共列位置在 db/checks/01
-- 按登记表核对；同文件先登记 append_only_registry（mode 取 APPEND_ONLY），
-- 使仅追加豁免与 attach_table_guards 的触发器挂接成立。
set role ep_mod_platform_core;
set lock_timeout = '5s';
set statement_timeout = '30min';

do $$
begin
  if not exists (select 1 from pg_class c
                 join pg_namespace n on n.oid = c.relnamespace
                 where n.nspname = 'platform_core' and c.relname = 'login_attempts') then
    execute '
      create table platform_core.login_attempts (
        id uuid not null,
        security_level smallint not null default 30,
        data_scope_tags text[] not null default ''{}'',
        created_at timestamptz not null default now(),
        created_by uuid not null default ''00000000-0000-7000-8000-000000000001'',
        user_id uuid null,
        login_name_hash bytea not null,
        outcome text not null,
        client text null,
        source_addr text null,
        occurred_at timestamptz not null,
        constraint pk_login_attempts primary key (id),
        constraint ck_login_attempts_outcome check (
          outcome in (''SUCCESS'', ''CREDENTIAL_INVALID'', ''ACCOUNT_LOCKED'',
                      ''ACCOUNT_INACTIVE'', ''MFA_REQUIRED'', ''MFA_INVALID'',
                      ''DEVICE_UNREGISTERED'', ''ADMISSION_REJECTED'')),
        constraint ck_login_attempts_source_addr_len check (
          source_addr is null or length(source_addr) <= 64)
      )';
  end if;
end $$;

-- 速率限制与过期清理按发生时间；按用户取失败序列供锁定窗口判定。
create index if not exists ix_login_attempts_occurred_at
  on platform_core.login_attempts (occurred_at);
create index if not exists ix_login_attempts_user_id_occurred_at
  on platform_core.login_attempts (user_id, occurred_at);

-- 仅追加登记：mode 取 APPEND_ONLY，mutable_columns 必须为空。
insert into platform_core.append_only_registry
  (id, schema_name, table_name, mode)
values
  ('00000000-0000-7000-8000-000000000102', 'platform_core', 'login_attempts',
   'APPEND_ONLY')
on conflict on constraint ux_append_only_registry_schema_table do nothing;

-- 挂接仅追加守卫（据登记挂 trg_login_attempts_append_only；无法人列不建策略）。
select platform_core.attach_table_guards('platform_core', 'login_attempts');

reset role;
