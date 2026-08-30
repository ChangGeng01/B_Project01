-- rollback: drop table platform_ops.degradation_windows;
-- rollback: 前置：阶段 3b 起的开窗调用方依赖本台账，回退前须确认无活动窗口。
-- db/migrations/platform_ops/V20260901104500__platform_ops_create_degradation_windows.sql
-- 降级窗口台账（阶段 2 计划第 3.5 节表十二，裁定 A-26），排在 platform_ops 建 schema 之后。
-- 列定义与阶段 14 计划第 3.1 节表 3 完全一致，阶段 14 只做 kind 取值扩展、
-- 追加两条 CHECK 与全部索引，不重建表、不增删列。
-- 本阶段交付的两条约束：
--   ux_degradation_windows_kind_scope_closed 建在 kind、subject、scope_legal_entity_id、
--     scope_accounting_period_id 与 closed_at 五者上，保证同一 kind 与同一 subject
--     在同一法人与会计期间作用域下至多一条活动条目；
--     **必须带 NULLS NOT DISTINCT（F-81）**：PostgreSQL 的唯一约束默认把 NULL 视作
--     互不相等，而 subject 与两个 scope 列都可空，三个 kind 初值
--     （OFFSITE_SINK_NOT_CONFIGURED / WRITER_NOT_IN_SERVICE / PORT_NOT_IMPLEMENTED）
--     又都是部署级、无法人无会计期间的窗口，正常写法就是 scope 两列取 NULL——
--     不带该修饰时本约束**在它本该生效的全部取值组合上完全不生效**：同一个端口
--     未实现窗口每触发一次就多一行活动记录，gauge 随之虚高，close() 又按
--     `is not distinct from` 一次关掉全部同类行，台账既不去重也不可对账，
--     行数随请求量无界增长。closed_at 非空且有 default 'infinity'，不受影响。
--   ck_degradation_windows_open_order 要求 closed_at 晚于 opened_at。
-- subject 为阶段 2 新建的可空列，承载开窗对象的完整类型名（端口名或平台能力名），
-- 使同一 kind 下的多个对象可同时开窗。两个 scope 列只作标注不作策略判据。
-- 本表不带 legal_entity_id、不建行级安全策略，按表十三登记（第 14 号迁移已写入登记行）；
-- kind 的 CHECK 首版只含阶段 2 定义的三个初始取值，其余取值由阶段 14 扩展。
-- 表经 DO 块创建，理由同 platform_core 各登记表：不带法人列的登记豁免表形态。
set role ep_mod_platform_ops;
set lock_timeout = '5s';
set statement_timeout = '30min';

do $$
begin
  if not exists (select 1 from pg_class c
                 join pg_namespace n on n.oid = c.relnamespace
                 where n.nspname = 'platform_ops' and c.relname = 'degradation_windows') then
    execute '
      create table platform_ops.degradation_windows (
        id uuid not null,
        security_level smallint not null default 20,
        data_scope_tags text[] not null default ''{}'',
        row_version bigint not null default 1,
        kind text not null,
        subject text null,
        scope_key text not null,
        scope_legal_entity_id uuid null,
        scope_accounting_period_id uuid null,
        basis text not null,
        detail jsonb not null default ''{}'',
        opened_at timestamptz not null,
        closed_at timestamptz not null default ''infinity'',
        closing_condition text not null,
        is_suppressible boolean not null,
        suppressed_until timestamptz null,
        created_at timestamptz not null default now(),
        created_by uuid not null default ''00000000-0000-7000-8000-000000000001'',
        updated_at timestamptz not null default now(),
        updated_by uuid not null default ''00000000-0000-7000-8000-000000000001'',
        constraint pk_degradation_windows primary key (id),
        constraint ck_degradation_windows_kind check (
          kind in (''OFFSITE_SINK_NOT_CONFIGURED'', ''WRITER_NOT_IN_SERVICE'', ''PORT_NOT_IMPLEMENTED'')),
        constraint ck_degradation_windows_subject_len check (subject is null or length(subject) <= 200),
        constraint ck_degradation_windows_scope_key_len check (length(scope_key) <= 200),
        constraint ck_degradation_windows_basis_len check (length(basis) <= 2000),
        constraint ck_degradation_windows_closing_condition_len check (length(closing_condition) <= 2000),
        constraint ck_degradation_windows_open_order check (closed_at > opened_at),
        constraint ux_degradation_windows_kind_scope_closed
          unique nulls not distinct
            (kind, subject, scope_legal_entity_id, scope_accounting_period_id, closed_at)
      )';
  end if;
end $$;

reset role;
