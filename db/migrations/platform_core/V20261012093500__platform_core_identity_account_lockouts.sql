-- rollback: drop table platform_core.account_lockouts;
-- db/migrations/platform_core/V20261012093500__platform_core_identity_account_lockouts.sql
-- 阶段 4 第 8 号迁移：账号锁定计数（04-identity-authz.md 表 3-8，一人一行）。
-- 与计划表 3-8 的形态差异：计划写 user_id 为 pk，但 db/checks/01 对不带法人列的
-- 非仅追加表要求第 1 列为 id，故本表以 id 为 pk、user_id 建唯一索引
-- ux_account_lockouts_user_id 承载「一人一行」语义，行为等价。
-- 登录算法第 3 步对本表行加 FOR UPDATE 行锁（lock_timeout 3s）判定 locked_until。
-- 本表不带 legal_entity_id 列、不建行级安全策略，登记行由第 29 号回填迁移写入。
-- 表经 DO 块创建，理由同阶段 2 第 3 号迁移：不带法人列的登记豁免表形态。
set role ep_mod_platform_core;
set lock_timeout = '5s';
set statement_timeout = '30min';

do $$
begin
  if not exists (select 1 from pg_class c
                 join pg_namespace n on n.oid = c.relnamespace
                 where n.nspname = 'platform_core' and c.relname = 'account_lockouts') then
    execute '
      create table platform_core.account_lockouts (
        id uuid not null,
        security_level smallint not null default 30,
        data_scope_tags text[] not null default ''{}'',
        row_version bigint not null default 1,
        created_at timestamptz not null default now(),
        created_by uuid not null default ''00000000-0000-7000-8000-000000000001'',
        updated_at timestamptz not null default now(),
        updated_by uuid not null default ''00000000-0000-7000-8000-000000000001'',
        user_id uuid not null,
        failure_count int not null default 0,
        window_started_at timestamptz not null default now(),
        locked_until timestamptz null,
        last_failure_at timestamptz null,
        constraint pk_account_lockouts primary key (id),
        constraint ux_account_lockouts_user_id unique (user_id),
        constraint ck_account_lockouts_failure_count check (failure_count >= 0)
      )';
  end if;
end $$;

-- 到期解锁的清理任务按锁定截止时间扫描。
create index if not exists ix_account_lockouts_locked_until
  on platform_core.account_lockouts (locked_until);
-- 时间序索引：本表无 legal_entity_id 列，取建档时间。
create index if not exists ix_account_lockouts_created_at
  on platform_core.account_lockouts (created_at);

-- 挂接乐观锁守卫（无法人列，不建行级安全策略），并断言基线索引齐备。
select platform_core.attach_table_guards('platform_core', 'account_lockouts');
select platform_core.assert_baseline_indexes('platform_core', 'account_lockouts', false);

reset role;
