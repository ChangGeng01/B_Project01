-- rollback: 前置：按版本号逆序先回退 user_credentials、sessions 与第 10 号回填迁移；
-- rollback: drop table platform_core.user_accounts;
-- db/migrations/platform_core/V20261012090000__platform_core_identity_user_accounts.sql
-- 阶段 4 第 1 号迁移：员工账号目录（04-identity-authz.md 表 3-1，档案类）。
-- 本表不带 legal_entity_id 列、不建行级安全策略：home_legal_entity_id 只用于审计事件
-- 分段与默认法人，不参与访问判定；法人可见性由 platform_authz.user_legal_entity_grants
-- 内联承担（第 12.2 节偏离一），登记行由第 29 号回填迁移写入 unpoliced_table_registry。
-- 基线索引偏离连带项：ix_user_accounts_status_created_at 替代
-- ix_<table>_legal_entity_id_created_at。
-- 表经 DO 块创建，理由同阶段 2 第 3 号迁移：不带法人列的登记豁免表形态，
-- SQL-008 判据对不带法人列的表在 db/checks/01 按登记表核对。
set role ep_mod_platform_core;
set lock_timeout = '5s';
set statement_timeout = '30min';

do $$
begin
  if not exists (select 1 from pg_class c
                 join pg_namespace n on n.oid = c.relnamespace
                 where n.nspname = 'platform_core' and c.relname = 'user_accounts') then
    execute '
      create table platform_core.user_accounts (
        id uuid not null,
        security_level smallint not null default 30,
        data_scope_tags text[] not null default ''{}'',
        row_version bigint not null default 1,
        created_at timestamptz not null default now(),
        created_by uuid not null default ''00000000-0000-7000-8000-000000000001'',
        updated_at timestamptz not null default now(),
        updated_by uuid not null default ''00000000-0000-7000-8000-000000000001'',
        account_kind text not null,
        login_name text not null,
        employee_no text null,
        display_name text not null,
        home_legal_entity_id uuid not null,
        supplier_ref_id uuid null,
        clearance_level smallint not null default 20,
        status text not null,
        is_mfa_required boolean not null default false,
        activated_on date null,
        deactivated_at timestamptz null,
        last_login_at timestamptz null,
        constraint pk_user_accounts primary key (id),
        constraint ck_user_accounts_account_kind check (
          account_kind in (''EMPLOYEE'', ''PORTAL'', ''BREAKGLASS'', ''SYSTEM'')),
        constraint ck_user_accounts_login_name_len check (length(login_name) between 1 and 64),
        constraint ck_user_accounts_employee_no_len check (
          employee_no is null or length(employee_no) between 1 and 64),
        constraint ck_user_accounts_display_name_len check (length(display_name) between 1 and 200),
        constraint ck_user_accounts_clearance_level check (clearance_level in (10, 20, 30, 40)),
        constraint ck_user_accounts_status check (
          status in (''UNACTIVATED'', ''ACTIVE'', ''LOCKED'', ''SUSPENDED'', ''DEACTIVATED''))
      )';
  end if;
end $$;

-- 全局唯一登录名与工号（门户与系统账号工号为空，唯一索引对多个 NULL 不生效）。
create unique index if not exists ux_user_accounts_login_name
  on platform_core.user_accounts (login_name);
create unique index if not exists ux_user_accounts_employee_no
  on platform_core.user_accounts (employee_no);
-- 时间序索引：本表无 legal_entity_id 列，按表 3-1 规格取状态加建档时间。
create index if not exists ix_user_accounts_status_created_at
  on platform_core.user_accounts (status, created_at);

-- 挂接乐观锁守卫（无法人列，不建行级安全策略），并断言基线索引齐备。
select platform_core.attach_table_guards('platform_core', 'user_accounts');
select platform_core.assert_baseline_indexes('platform_core', 'user_accounts', false);

reset role;
