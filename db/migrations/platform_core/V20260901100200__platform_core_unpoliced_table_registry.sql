-- rollback: drop table platform_core.unpoliced_table_registry;
-- rollback: 前置：db/checks/13 以本表为判据来源，删除后一致性断言失效。
-- db/migrations/platform_core/V20260901100200__platform_core_unpoliced_table_registry.sql
-- 未受行级策略表登记（阶段 2 计划第 3.5 节表十三）。
-- 正向规则：凡带 legal_entity_id 的表一律按第 3.6 节模板建策略；不带该列的表必须
-- 逐表登记本表一行，且 admission_basis 必须成立——该表的行要么在本部署内对全部法人
-- 取值相同（SAME_FOR_ALL_ENTITIES），要么是隔离机制自身或部署自身的元数据
-- （ISOLATION_OR_DEPLOYMENT_METADATA）。
-- 本迁移同文件写入本阶段八行登记，八行 admission_basis 均取
-- ISOLATION_OR_DEPLOYMENT_METADATA；其余阶段各自补齐各自的新增表。
-- 本表自身不带 legal_entity_id 列、不建行级安全策略，登记本表自身。
-- 表经 DO 块创建，理由同第 3 号迁移：不带法人列的登记豁免表形态。
set role ep_mod_platform_core;
set lock_timeout = '5s';
set statement_timeout = '30min';

do $$
begin
  if not exists (select 1 from pg_class c
                 join pg_namespace n on n.oid = c.relnamespace
                 where n.nspname = 'platform_core' and c.relname = 'unpoliced_table_registry') then
    execute '
      create table platform_core.unpoliced_table_registry (
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
        admission_basis text not null,
        isolation_entry text not null,
        matrix_case_id text not null,
        constraint pk_unpoliced_table_registry primary key (id),
        constraint ux_unpoliced_table_registry_schema_table unique (schema_name, table_name),
        constraint ck_unpoliced_table_registry_basis check (
          admission_basis in (''SAME_FOR_ALL_ENTITIES'', ''ISOLATION_OR_DEPLOYMENT_METADATA'')),
        constraint ck_unpoliced_table_registry_isolation_entry_len check (
          length(isolation_entry) between 1 and 200)
      )';
  end if;
end $$;

-- 唯一索引与时间序索引：本表无 legal_entity_id 列，按表十三规格取 ix_created_at。
create unique index if not exists ux_unpoliced_table_registry_schema_table
  on platform_core.unpoliced_table_registry (schema_name, table_name);
create index if not exists ix_unpoliced_table_registry_created_at
  on platform_core.unpoliced_table_registry (created_at);

-- 本阶段八行登记。isolation_entry 记载法人可见性所落的应用层入口，
-- matrix_case_id 取该入口在 tests/rls_matrix 中的用例标识；
-- 八行的准入判据均为隔离机制自身或部署自身的元数据。
insert into platform_core.unpoliced_table_registry
  (id, schema_name, table_name, admission_basis, isolation_entry, matrix_case_id)
values
  ('00000000-0000-7000-8000-000000000201', 'platform_core', 'legal_entities',
   'ISOLATION_OR_DEPLOYMENT_METADATA', '法人注册表为隔离机制自身元数据，可见性由法人目录读取契约承载', 'rls_matrix_unpoliced_legal_entities'),
  ('00000000-0000-7000-8000-000000000202', 'platform_core', 'enterprise_groups',
   'ISOLATION_OR_DEPLOYMENT_METADATA', '集团表为隔离机制自身元数据，可见性由法人目录读取契约承载', 'rls_matrix_unpoliced_enterprise_groups'),
  ('00000000-0000-7000-8000-000000000203', 'platform_core', 'sensitive_field_registry',
   'ISOLATION_OR_DEPLOYMENT_METADATA', '敏感字段清单为部署元数据，可见性由平台管理端点的能力判定承载', 'rls_matrix_unpoliced_sensitive_field_registry'),
  ('00000000-0000-7000-8000-000000000204', 'platform_core', 'append_only_registry',
   'ISOLATION_OR_DEPLOYMENT_METADATA', '仅追加登记表为部署元数据，可见性由平台管理端点的能力判定承载', 'rls_matrix_unpoliced_append_only_registry'),
  ('00000000-0000-7000-8000-000000000205', 'platform_core', 'migration_windows',
   'ISOLATION_OR_DEPLOYMENT_METADATA', '迁移窗口为部署元数据，可见性由迁移窗口管理端点的能力判定承载', 'rls_matrix_unpoliced_migration_windows'),
  ('00000000-0000-7000-8000-000000000206', 'platform_core', 'migration_window_lock',
   'ISOLATION_OR_DEPLOYMENT_METADATA', '迁移窗口锁为部署元数据，仅迁移编排路径触达', 'rls_matrix_unpoliced_migration_window_lock'),
  ('00000000-0000-7000-8000-000000000207', 'platform_core', 'unpoliced_table_registry',
   'ISOLATION_OR_DEPLOYMENT_METADATA', '登记表自身为部署元数据，可见性由平台管理端点的能力判定承载', 'rls_matrix_unpoliced_unpoliced_table_registry'),
  ('00000000-0000-7000-8000-000000000208', 'platform_ops', 'degradation_windows',
   'ISOLATION_OR_DEPLOYMENT_METADATA', '降级窗口台账为部署元数据，可见性由运维管理员、安全管理员与审计管理员三类角色的 ABAC 判定承载', 'rls_matrix_unpoliced_degradation_windows')
on conflict on constraint ux_unpoliced_table_registry_schema_table do nothing;

-- 挂接乐观锁守卫（无法人列，不建行级安全策略），并断言基线索引齐备。
select platform_core.attach_table_guards('platform_core', 'unpoliced_table_registry');
select platform_core.assert_baseline_indexes('platform_core', 'unpoliced_table_registry', false);

reset role;
