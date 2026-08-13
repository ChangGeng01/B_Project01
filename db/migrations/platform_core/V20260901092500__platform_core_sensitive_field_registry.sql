-- rollback: drop table platform_core.sensitive_field_registry;
-- db/migrations/platform_core/V20260901092500__platform_core_sensitive_field_registry.sql
-- 敏感字段清单（阶段 2 计划第 3.5 节表四）。
-- 业务列集按 C-06 冻结为十一列：schema_name、table_name、column_name、category、
-- security_level、is_field_encrypted、blind_index、blind_index_column、mask_style、
-- normalization、release_ref；approved_by 与 approved_at 两列按 C-06 撤销，不建。
-- 本阶段不预置任何行，阶段 5 按 A-28 以 backfill 迁移插入四行。
-- 本表不带 legal_entity_id 列、不建行级安全策略，按表十三登记（第 14 号迁移）。
-- 密级列即公共列第 2 位的 security_level（基线第 4 节），业务列集不再另建同名列；
-- ck_sensitive_field_registry_level 直接约束该公共列。
-- 表经 DO 块创建，理由同第 3 号迁移：不带法人列的登记豁免表形态。
set role ep_mod_platform_core;
set lock_timeout = '5s';
set statement_timeout = '30min';

do $$
begin
  if not exists (select 1 from pg_class c
                 join pg_namespace n on n.oid = c.relnamespace
                 where n.nspname = 'platform_core' and c.relname = 'sensitive_field_registry') then
    execute '
      create table platform_core.sensitive_field_registry (
        id uuid not null,
        security_level smallint not null default 20,
        data_scope_tags text[] not null default ''{}'',
        row_version bigint not null default 1,
        created_at timestamptz not null default now(),
        created_by uuid not null default ''00000000-0000-7000-8000-000000000001'',
        updated_at timestamptz not null default now(),
        updated_by uuid not null default ''00000000-0000-7000-8000-000000000001'',
        schema_name text not null,
        table_name text not null,
        column_name text not null,
        category text not null,
        is_field_encrypted boolean not null default false,
        blind_index text not null default ''NONE'',
        blind_index_column text null,
        mask_style text not null default ''NONE'',
        normalization text not null default ''TRIM_NFKC'',
        release_ref text not null,
        constraint pk_sensitive_field_registry primary key (id),
        constraint ux_sensitive_field_registry_schema_table_column
          unique (schema_name, table_name, column_name),
        constraint ck_sensitive_field_registry_category check (
          category in (''IDENTITY'', ''CONTACT'', ''ACCOUNT'', ''TAX_ID'', ''PAYMENT_TOKEN'', ''LEGAL'', ''HEALTH'')),
        constraint ck_sensitive_field_registry_level check (
          security_level in (10, 20, 30, 40)),
        constraint ck_sensitive_field_registry_bidx check (blind_index in (''NONE'', ''EXACT'')),
        constraint ck_sensitive_field_registry_normalization check (
          normalization in (''NONE'', ''TRIM_NFKC'', ''TRIM_NFKC_LOWER'', ''DIGITS_ONLY'')),
        constraint ck_sensitive_field_registry_release_ref check (
          release_ref like ''MIGRATION:%'' or release_ref like ''ENDPOINT:%'')
      )';
  end if;
end $$;

-- 挂接乐观锁守卫（无法人列，不建行级安全策略）。
-- 本表无 legal_entity_id 列，不建 ix_<table>_legal_entity_id_created_at，基线索引断言随之跳过。
select platform_core.attach_table_guards('platform_core', 'sensitive_field_registry');

reset role;
