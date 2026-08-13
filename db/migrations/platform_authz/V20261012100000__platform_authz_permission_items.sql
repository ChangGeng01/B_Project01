-- rollback: drop table platform_authz.permission_items;
-- db/migrations/platform_authz/V20261012100000__platform_authz_permission_items.sql
-- 阶段 4 第 11 号迁移：权限项注册表（04-identity-authz.md 表 3-10）。
-- code 形如 sales.sales_order；allowed_actions 子集取自 PRD 第 10.2.2 节恰好
-- 六个动作 VIEW、CREATE、UPDATE、SUBMIT、APPROVE、EXPORT，不多不少。
-- ck_permission_items_forbidden_codes 拒绝写入以 platform.legal_entity_isolation
-- 与 platform.direct_db_access 两个前缀开头的 code：关闭或修改法人隔离机制与
-- 事务业务库直连两类权限项写不进这张表；该约束替代原拟的同名启动自检项。
-- 本表不带 legal_entity_id 列、不建行级安全策略：行在本部署内对两个法人取值
-- 相同，登记行由第 29 号回填迁移写入 unpoliced_table_registry。
-- 与计划表 3-10 的形态差异：计划写 code 为 pk，但 db/checks/01 对不带法人列的
-- 非仅追加表要求第 1 列为 id，故本表以 id 为 pk、code 建唯一索引
-- ux_permission_items_code 承载自然键语义，行为等价。
-- 表经 DO 块创建，理由同阶段 2 第 3 号迁移：不带法人列的登记豁免表形态。
set role ep_mod_platform_authz;
set lock_timeout = '5s';
set statement_timeout = '30min';

do $$
begin
  if not exists (select 1 from pg_class c
                 join pg_namespace n on n.oid = c.relnamespace
                 where n.nspname = 'platform_authz' and c.relname = 'permission_items') then
    execute '
      create table platform_authz.permission_items (
        id uuid not null,
        security_level smallint not null default 20,
        data_scope_tags text[] not null default ''{}'',
        row_version bigint not null default 1,
        created_at timestamptz not null default now(),
        created_by uuid not null default ''00000000-0000-7000-8000-000000000001'',
        updated_at timestamptz not null default now(),
        updated_by uuid not null default ''00000000-0000-7000-8000-000000000001'',
        code text not null,
        module_code text not null,
        function_point text not null,
        allowed_actions text[] not null,
        object_type text not null,
        description text null,
        constraint pk_permission_items primary key (id),
        constraint ux_permission_items_code unique (code),
        constraint ck_permission_items_code_fmt check (
          code ~ ''^[a-z][a-z0-9_]*(\.[a-z][a-z0-9_]*)+$'' and length(code) <= 128),
        constraint ck_permission_items_module_code_len check (
          length(module_code) between 1 and 64),
        constraint ck_permission_items_function_point_len check (
          length(function_point) between 1 and 128),
        constraint ck_permission_items_object_type_len check (
          length(object_type) between 1 and 128),
        constraint ck_permission_items_allowed_actions check (
          cardinality(allowed_actions) > 0 and allowed_actions <@ array[
            ''VIEW'', ''CREATE'', ''UPDATE'', ''SUBMIT'', ''APPROVE'', ''EXPORT'']),
        constraint ck_permission_items_forbidden_codes check (
          code not like ''platform.legal_entity_isolation%''
          and code not like ''platform.direct_db_access%'')
      )';
  end if;
end $$;

-- 按模块检索权限项。
create index if not exists ix_permission_items_module_code
  on platform_authz.permission_items (module_code);
-- 时间序索引：本表无 legal_entity_id 列，取建档时间。
create index if not exists ix_permission_items_created_at
  on platform_authz.permission_items (created_at);

-- 挂接乐观锁守卫（无法人列，不建行级安全策略），并断言基线索引齐备。
select platform_core.attach_table_guards('platform_authz', 'permission_items');
select platform_core.assert_baseline_indexes('platform_authz', 'permission_items', false);

reset role;
