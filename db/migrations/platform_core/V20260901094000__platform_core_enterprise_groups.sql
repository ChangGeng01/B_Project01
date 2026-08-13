-- rollback: alter table platform_core.legal_entities
-- rollback:   drop constraint if exists fk_legal_entities_enterprise_groups;
-- rollback: drop table platform_core.enterprise_groups;
-- db/migrations/platform_core/V20260901094000__platform_core_enterprise_groups.sql
-- 集团表（阶段 2 计划第 3.5 节表七），并为 legal_entities.group_id 追加同 schema 真实外键。
-- 外键必须晚于两侧建表迁移，故以 ALTER TABLE ADD CONSTRAINT 在本迁移补建（第 3.8 节），
-- ON DELETE RESTRICT：有法人挂靠的集团不得删除。
-- 本表不带 legal_entity_id 列、不建行级安全策略，按表十三登记（第 14 号迁移）。
-- 表经 DO 块创建，理由同第 3 号迁移：不带法人列的登记豁免表形态。
set role ep_mod_platform_core;
set lock_timeout = '5s';
set statement_timeout = '30min';

do $$
begin
  if not exists (select 1 from pg_class c
                 join pg_namespace n on n.oid = c.relnamespace
                 where n.nspname = 'platform_core' and c.relname = 'enterprise_groups') then
    execute '
      create table platform_core.enterprise_groups (
        id uuid not null,
        security_level smallint not null default 20,
        data_scope_tags text[] not null default ''{}'',
        row_version bigint not null default 1,
        created_at timestamptz not null default now(),
        created_by uuid not null default ''00000000-0000-7000-8000-000000000001'',
        updated_at timestamptz not null default now(),
        updated_by uuid not null default ''00000000-0000-7000-8000-000000000001'',
        code text not null,
        name text not null,
        is_active boolean not null default true,
        deactivated_at timestamptz null,
        constraint pk_enterprise_groups primary key (id),
        constraint ck_enterprise_groups_code_len check (length(code) between 1 and 64),
        constraint ck_enterprise_groups_name_len check (length(name) between 1 and 200)
      )';
  end if;

  -- 为第 3 号迁移建出的 legal_entities.group_id 追加同 schema 真实外键。
  if not exists (select 1 from pg_constraint
                 where conname = 'fk_legal_entities_enterprise_groups') then
    execute '
      alter table platform_core.legal_entities
        add constraint fk_legal_entities_enterprise_groups
        foreign key (group_id) references platform_core.enterprise_groups (id)
        on delete restrict';
  end if;
end $$;

-- 唯一索引：集团码全库唯一。
create unique index if not exists ux_enterprise_groups_code
  on platform_core.enterprise_groups (code);
-- 基线时间序索引：本表无 legal_entity_id 列，按表七规格取 ix_created_at。
create index if not exists ix_enterprise_groups_created_at
  on platform_core.enterprise_groups (created_at);

-- 挂接乐观锁守卫（无法人列，不建行级安全策略），并断言基线索引齐备。
select platform_core.attach_table_guards('platform_core', 'enterprise_groups');
select platform_core.assert_baseline_indexes('platform_core', 'enterprise_groups', false);

reset role;
