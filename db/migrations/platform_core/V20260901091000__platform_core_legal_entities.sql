-- rollback: drop table platform_core.legal_entities;
-- rollback: 前置：先按版本号逆序回退第 9 号迁移追加的外键（或直接随本表删除一并消失），
-- rollback: 且确认无任何下游表引用法人行。
-- db/migrations/platform_core/V20260901091000__platform_core_legal_entities.sql
-- 法人注册表（阶段 2 计划第 3.5 节表一）。
-- 本表是法人维度隔离的根，自身不带 legal_entity_id 列、不建行级安全策略，
-- 按表十三登记于 unpoliced_table_registry（第 14 号迁移写入登记行）。
-- 公共列按基线第 4 节九件套中除 legal_entity_id 外的八列居首排列；
-- created_by/updated_by 默认值按 A-02 取 SYSTEM_PRINCIPAL_ID
-- 00000000-0000-7000-8000-000000000001。
-- group_id 列在本迁移建出但不建外键，集团表建出后由第 9 号迁移追加同 schema 真实外键。
-- 表经 DO 块创建：静态门禁 SQL-008 的九件套判据以带法人列的表为对象，
-- 不带法人列的登记豁免表统一以 DO 块形态建表，判据在 db/checks/01 按登记表核对。
set role ep_mod_platform_core;
set lock_timeout = '5s';
set statement_timeout = '30min';

do $$
begin
  if not exists (select 1 from pg_class c
                 join pg_namespace n on n.oid = c.relnamespace
                 where n.nspname = 'platform_core' and c.relname = 'legal_entities') then
    execute '
      create table platform_core.legal_entities (
        id uuid not null,
        security_level smallint not null default 20,
        data_scope_tags text[] not null default ''{}'',
        row_version bigint not null default 1,
        created_at timestamptz not null default now(),
        created_by uuid not null default ''00000000-0000-7000-8000-000000000001'',
        updated_at timestamptz not null default now(),
        updated_by uuid not null default ''00000000-0000-7000-8000-000000000001'',
        code text not null,
        entity_no text not null,
        name text not null,
        short_name text null,
        display_timezone text not null default ''Asia/Shanghai'',
        currency_code text not null default ''CNY'',
        is_active boolean not null default true,
        deactivated_at timestamptz null,
        group_id uuid null,
        constraint pk_legal_entities primary key (id),
        constraint ck_legal_entities_code_len check (length(code) between 1 and 64),
        constraint ck_legal_entities_entity_no_fmt check (entity_no ~ ''^[0-9]{2}$''),
        constraint ck_legal_entities_name_len check (length(name) between 1 and 200),
        constraint ck_legal_entities_short_name_len check (short_name is null or length(short_name) <= 64),
        constraint ck_legal_entities_tz check (display_timezone = ''Asia/Shanghai''),
        constraint ck_legal_entities_currency check (currency_code = ''CNY'')
      )';
  end if;
end $$;

-- 唯一索引：法人码与两位数字法人码全库唯一。
create unique index if not exists ux_legal_entities_code
  on platform_core.legal_entities (code);
create unique index if not exists ux_legal_entities_entity_no
  on platform_core.legal_entities (entity_no);
-- 基线时间序索引：本表无 legal_entity_id 列，按表一规格取 ix_created_at。
create index if not exists ix_legal_entities_created_at
  on platform_core.legal_entities (created_at);

-- 挂接乐观锁守卫（无法人列，不建行级安全策略），并断言基线索引齐备。
select platform_core.attach_table_guards('platform_core', 'legal_entities');
select platform_core.assert_baseline_indexes('platform_core', 'legal_entities', false);

reset role;
