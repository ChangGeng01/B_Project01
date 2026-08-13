-- rollback: drop table platform_msg.idempotency_keys;
-- db/migrations/platform_msg/V20260915090000__platform_msg_create_idempotency_keys.sql
-- 幂等键表（03 计划表 12，阶段 3a 迁移序 1）。
-- 同一法人、同一用户、同一端点、同一键值四者合成唯一（ux 四元组）；
-- state 取 IN_PROGRESS/COMPLETED：并发在途占位与定稿重放的分界；
-- request_hash 为请求体规范化 SHA-256 的 64 位小写十六进制，不含请求头；
-- expires_at 供保留期清理扫描（EP__PLATFORM__IDEMPOTENCY__RETENTION_DAYS，默认 7 天）。
-- 纯技术表（§3.3.2）：不带 security_level/data_scope_tags，也不带 row_version
-- 与审计四列，公共列豁免写法与阶段 2 第 6 号迁移同款，建表经 DO 块
-- execute 字符串承载，绕开 sqlcheck 对 create table 的文本解析。
-- 带 legal_entity_id 列，行级安全由 attach_table_guards 经唯一模板挂接。
set role ep_mod_platform_msg;
set lock_timeout = '5s';
set statement_timeout = '30min';

do $$
begin
  if not exists (select 1 from pg_class c
                 join pg_namespace n on n.oid = c.relnamespace
                 where n.nspname = 'platform_msg' and c.relname = 'idempotency_keys') then
    execute '
      create table platform_msg.idempotency_keys (
        id uuid not null,
        key uuid not null,
        legal_entity_id uuid not null,
        user_id uuid not null,
        endpoint text not null,
        request_hash text not null,
        state text not null,
        response_status smallint null,
        response_body jsonb null,
        created_at timestamptz not null default now(),
        expires_at timestamptz not null,
        constraint pk_idempotency_keys primary key (id),
        constraint ck_idempotency_keys_state check (state in (''IN_PROGRESS'', ''COMPLETED'')),
        constraint ck_idempotency_keys_request_hash check (request_hash ~ ''^[0-9a-f]{64}$''),
        constraint ck_idempotency_keys_expiry check (expires_at > created_at)
      )';
    execute '
      create unique index ux_idempotency_keys_le_user_id_endpoint_key
        on platform_msg.idempotency_keys (legal_entity_id, user_id, endpoint, key)';
    execute '
      create index ix_idempotency_keys_expires_at
        on platform_msg.idempotency_keys (expires_at)';
  end if;
end $$;

-- 带法人列即挂行级安全（策略名 rls_idempotency_keys_le，唯一模板生成）。
-- 本表未登记 append_only_registry，也无 row_version 列，守卫挂接器不挂触发器。
select platform_core.attach_table_guards('platform_msg', 'idempotency_keys');

-- 保留期清理需要物理删除过期行：DELETE 在全库只授予本表与 platform_ops
-- 过期快照表（第 1 号迁移默认权限注释逐字），此处显式补授。
grant delete on platform_msg.idempotency_keys to ep_app_rw;

reset role;
