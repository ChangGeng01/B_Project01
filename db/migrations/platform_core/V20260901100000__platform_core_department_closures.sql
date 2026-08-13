-- rollback: drop table platform_core.department_closures;
-- db/migrations/platform_core/V20260901100000__platform_core_department_closures.sql
-- 部门层级闭包表（阶段 2 计划第 3.5 节表十一、第 4.8 节维护契约）。
-- ancestor_department_id 与 descendant_department_id 均为同 schema 真实外键，
-- ON DELETE RESTRICT；depth 不小于 0（根部门自环行 depth = 0）。
-- 唯一约束 ux_department_closures_pair 在 (ancestor_department_id, descendant_department_id)，
-- 名称与列按裁定 A-04 冻结，不得改写。
-- ix_department_closures_le_id_descendant_id 按第 3.8 节缩写规则命名，
-- 全称为 ix_department_closures_legal_entity_id_descendant_department_id，
-- 登记在 docs/data-dictionary.md（由集成任务负责）。
-- 行级安全策略经 apply_le_rls 生成（rls_department_closures_le），不手写。
set role ep_mod_platform_core;
set lock_timeout = '5s';
set statement_timeout = '30min';

create table if not exists platform_core.department_closures (
  id uuid not null,
  legal_entity_id uuid not null,
  security_level smallint not null default 20,
  data_scope_tags text[] not null default '{}',
  row_version bigint not null default 1,
  created_at timestamptz not null default now(),
  created_by uuid not null default '00000000-0000-7000-8000-000000000001',
  updated_at timestamptz not null default now(),
  updated_by uuid not null default '00000000-0000-7000-8000-000000000001',
  ancestor_department_id uuid not null,
  descendant_department_id uuid not null,
  depth smallint not null,
  constraint pk_department_closures primary key (id),
  constraint fk_department_closures_ancestor foreign key (ancestor_department_id)
    references platform_core.departments (id) on delete restrict,
  constraint fk_department_closures_descendant foreign key (descendant_department_id)
    references platform_core.departments (id) on delete restrict,
  constraint ck_department_closures_depth check (depth >= 0),
  constraint ux_department_closures_pair
    unique (ancestor_department_id, descendant_department_id)
);

-- 基线时间序索引。
create index if not exists ix_department_closures_legal_entity_id_created_at
  on platform_core.department_closures (legal_entity_id, created_at);
-- 后代展开索引（缩写名，全称见文件头注释）。
create index if not exists ix_department_closures_le_id_descendant_id
  on platform_core.department_closures (legal_entity_id, descendant_department_id);

-- 挂接乐观锁守卫与行级安全策略（rls_department_closures_le），断言基线索引齐备。
select platform_core.attach_table_guards('platform_core', 'department_closures');
select platform_core.assert_baseline_indexes('platform_core', 'department_closures', false);

reset role;
