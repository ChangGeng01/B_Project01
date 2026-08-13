-- rollback: drop table platform_core.append_only_registry;
-- db/migrations/platform_core/V20260901093000__platform_core_append_only_registry.sql
-- 仅追加表登记（阶段 2 计划第 3.5 节表五，裁定 B-02 的登记表）。
-- mode 取 APPEND_ONLY 或 IMMUTABLE_COLUMNS；APPEND_ONLY 时 mutable_columns 必须为空。
-- 十四行登记由阶段 3b、7、8、9a、10 各自在本模块迁移中插入（B-02），本阶段只建表。
-- 登记与物理触发器的一致性由 db/checks/append_only_consistency.sql 兜底。
-- 本表不带 legal_entity_id 列、不建行级安全策略，按表十三登记（第 14 号迁移）。
-- 表经 DO 块创建，理由同第 3 号迁移：不带法人列的登记豁免表形态。
set role ep_mod_platform_core;
set lock_timeout = '5s';
set statement_timeout = '30min';

do $$
begin
  if not exists (select 1 from pg_class c
                 join pg_namespace n on n.oid = c.relnamespace
                 where n.nspname = 'platform_core' and c.relname = 'append_only_registry') then
    execute '
      create table platform_core.append_only_registry (
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
        mode text not null,
        mutable_columns text[] not null default ''{}'',
        constraint pk_append_only_registry primary key (id),
        constraint ux_append_only_registry_schema_table unique (schema_name, table_name),
        constraint ck_append_only_registry_mode check (
          mode in (''APPEND_ONLY'', ''IMMUTABLE_COLUMNS'')),
        constraint ck_append_only_registry_mutable check (
          mode <> ''APPEND_ONLY'' or mutable_columns = ''{}'')
      )';
  end if;
end $$;

-- 全部 24 个属主角色在建表迁移末尾以调用方权限调用 attach_table_guards，
-- 函数需读本登记表；本表自此刻起存在，须立即授权，否则 #8 起的建表迁移
-- 报 permission denied。授权收口迁移（V20260901100500）对全 schema 表再补一次。
do $$
declare
  s text;
begin
  for s in
    select unnest(array[
      'platform_core', 'platform_authz', 'platform_meta', 'platform_flow',
      'platform_audit', 'platform_msg', 'platform_file', 'platform_ops', 'ext',
      'mdm', 'crm', 'cpq', 'clm', 'sales', 'procure', 'inventory', 'costing',
      'project', 'service', 'finance', 'ledger', 'invoice', 'portal', 'reporting'])
  loop
    execute format('grant select on platform_core.append_only_registry to %I', 'ep_mod_' || s);
  end loop;
end $$;

-- 挂接乐观锁守卫（无法人列，不建行级安全策略）。
select platform_core.attach_table_guards('platform_core', 'append_only_registry');

reset role;
