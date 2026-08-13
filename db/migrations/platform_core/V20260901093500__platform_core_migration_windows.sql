-- rollback: drop table platform_core.migration_windows;
-- rollback: drop table platform_core.migration_window_lock;
-- db/migrations/platform_core/V20260901093500__platform_core_migration_windows.sql
-- 迁移窗口与单例锁表（阶段 2 计划第 3.5 节表六）。
-- state 取 OPEN/CLOSED；approval_ref 承载双人审批引用，缺失即不可开窗；
-- expires_at 必须晚于 opened_at；close_kind 取 MANUAL/EXPIRED/FAILED。
-- 同一时刻至多一个 OPEN 窗口：由对 migration_window_lock 的 SELECT ... FOR UPDATE
-- 串行化，不用部分唯一索引（基线第 3.10 节禁部分索引）。
-- 锁表只有 id smallint 一列，不带公共列也不带行版本，行数由 check (id = 1) 固定为一行，
-- db/checks/01 对该表豁免；本迁移同时插入该唯一锁行，开窗流程据此加行锁。
-- 两表均不带 legal_entity_id 列、不建行级安全策略，按表十三登记（第 14 号迁移）。
-- 表经 DO 块创建，理由同第 3 号迁移：不带法人列的登记豁免表形态。
set role ep_mod_platform_core;
set lock_timeout = '5s';
set statement_timeout = '30min';

do $$
begin
  if not exists (select 1 from pg_class c
                 join pg_namespace n on n.oid = c.relnamespace
                 where n.nspname = 'platform_core' and c.relname = 'migration_windows') then
    execute '
      create table platform_core.migration_windows (
        id uuid not null,
        security_level smallint not null default 20,
        data_scope_tags text[] not null default ''{}'',
        row_version bigint not null default 1,
        created_at timestamptz not null default now(),
        created_by uuid not null default ''00000000-0000-7000-8000-000000000001'',
        updated_at timestamptz not null default now(),
        updated_by uuid not null default ''00000000-0000-7000-8000-000000000001'',
        state text not null,
        approval_ref text not null,
        reason text not null,
        opened_by uuid not null,
        opened_at timestamptz not null,
        expires_at timestamptz not null,
        closed_by uuid null,
        closed_at timestamptz null,
        close_kind text null,
        applied_versions text[] not null default ''{}'',
        constraint pk_migration_windows primary key (id),
        constraint ck_migration_windows_state check (state in (''OPEN'', ''CLOSED'')),
        constraint ck_migration_windows_reason_len check (length(reason) <= 2000),
        constraint ck_migration_windows_expiry check (expires_at > opened_at),
        constraint ck_migration_windows_close_kind check (
          close_kind is null or close_kind in (''MANUAL'', ''EXPIRED'', ''FAILED''))
      )';
  end if;

  if not exists (select 1 from pg_class c
                 join pg_namespace n on n.oid = c.relnamespace
                 where n.nspname = 'platform_core' and c.relname = 'migration_window_lock') then
    execute '
      create table platform_core.migration_window_lock (
        id smallint not null,
        constraint pk_migration_window_lock primary key (id),
        constraint ck_migration_window_lock_singleton check (id = 1)
      )';
  end if;
end $$;

-- 单例锁行：开窗流程对该行取 SELECT ... FOR UPDATE 串行化，行必须存在。
insert into platform_core.migration_window_lock (id) values (1)
on conflict (id) do nothing;

-- 挂接乐观锁守卫（无法人列，不建行级安全策略）。
select platform_core.attach_table_guards('platform_core', 'migration_windows');

reset role;
