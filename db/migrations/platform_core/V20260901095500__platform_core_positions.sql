-- rollback: drop table platform_core.positions;
-- db/migrations/platform_core/V20260901095500__platform_core_positions.sql
-- 岗位表（阶段 2 计划第 3.5 节表十）。
-- department_id 为同 schema 真实外键，ON DELETE RESTRICT；rank_no 大于 0。
-- 阶段 4 的 position_id 外键目标即 platform_core.positions(id)。
-- 行级安全策略经 apply_le_rls 生成（rls_positions_le），不手写。
set role ep_mod_platform_core;
set lock_timeout = '5s';
set statement_timeout = '30min';

create table if not exists platform_core.positions (
  id uuid not null,
  legal_entity_id uuid not null,
  security_level smallint not null default 20,
  data_scope_tags text[] not null default '{}',
  row_version bigint not null default 1,
  created_at timestamptz not null default now(),
  created_by uuid not null default '00000000-0000-7000-8000-000000000001',
  updated_at timestamptz not null default now(),
  updated_by uuid not null default '00000000-0000-7000-8000-000000000001',
  department_id uuid not null,
  code text not null,
  name text not null,
  rank_no smallint not null,
  is_active boolean not null default true,
  deactivated_at timestamptz null,
  constraint pk_positions primary key (id),
  constraint fk_positions_departments foreign key (department_id)
    references platform_core.departments (id) on delete restrict,
  constraint ck_positions_name_len check (length(name) between 1 and 200),
  constraint ck_positions_rank_no check (rank_no > 0)
);

-- 法人内岗位码唯一。
create unique index if not exists ux_positions_legal_entity_id_code
  on platform_core.positions (legal_entity_id, code);
-- 基线时间序索引。
create index if not exists ix_positions_legal_entity_id_created_at
  on platform_core.positions (legal_entity_id, created_at);

-- 挂接乐观锁守卫与行级安全策略（rls_positions_le），断言基线索引齐备。
select platform_core.attach_table_guards('platform_core', 'positions');
select platform_core.assert_baseline_indexes('platform_core', 'positions', false);

reset role;
