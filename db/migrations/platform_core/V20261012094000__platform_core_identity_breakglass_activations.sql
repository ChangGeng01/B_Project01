-- rollback: drop table platform_core.breakglass_activations;
-- db/migrations/platform_core/V20261012094000__platform_core_identity_breakglass_activations.sql
-- 阶段 4 第 9 号迁移：受控应急本地账号启用台账（04-identity-authz.md 表 3-9，单据类）。
-- doc_no 类型码 BGA；本表无 legal_entity_id 列，doc_no 唯一约束不带法人，
-- 是偏离一的连带项。allowed_action_set 由 CHECK 限定为规格第 12.1 章三类：
-- UNLOCK_OR_RESET_ADMIN、RESTORE_CONTROLLED_CONFIG_RELEASE、
-- TRIGGER_BACKUP_OR_RESTORE，不接受其他取值。
-- 启用四要素：expires_at 保证单次不超过 8 小时；approved_by 与 requested_by
-- 不同人且 approved_by 持 SECURITY 或 AUDIT 职责由端点校验；启用瞬间写
-- platform_ops 台账告警由端点完成。
-- 本表不带 legal_entity_id 列、不建行级安全策略，登记行由第 29 号回填迁移写入。
-- 表经 DO 块创建，理由同阶段 2 第 3 号迁移：不带法人列的登记豁免表形态。
set role ep_mod_platform_core;
set lock_timeout = '5s';
set statement_timeout = '30min';

do $$
begin
  if not exists (select 1 from pg_class c
                 join pg_namespace n on n.oid = c.relnamespace
                 where n.nspname = 'platform_core' and c.relname = 'breakglass_activations') then
    execute '
      create table platform_core.breakglass_activations (
        id uuid not null,
        security_level smallint not null default 40,
        data_scope_tags text[] not null default ''{}'',
        row_version bigint not null default 1,
        created_at timestamptz not null default now(),
        created_by uuid not null default ''00000000-0000-7000-8000-000000000001'',
        updated_at timestamptz not null default now(),
        updated_by uuid not null default ''00000000-0000-7000-8000-000000000001'',
        doc_no text not null,
        status text not null,
        user_id uuid not null,
        requested_by uuid not null,
        approved_by uuid null,
        reason text not null,
        approval_ref text null,
        allowed_action_set text[] not null,
        activated_at timestamptz null,
        expires_at timestamptz null,
        closed_at timestamptz null,
        rotated_at timestamptz null,
        rotation_result text null,
        constraint pk_breakglass_activations primary key (id),
        constraint ck_breakglass_activations_doc_no_len check (length(doc_no) between 1 and 64),
        constraint ck_breakglass_activations_status check (
          status in (''DRAFT'', ''PENDING_APPROVAL'', ''APPROVED'', ''ACTIVE'',
                     ''EXPIRED'', ''CLOSED'', ''REJECTED'')),
        constraint ck_breakglass_activations_reason_len check (length(reason) between 1 and 2000),
        constraint ck_breakglass_activations_actions check (
          cardinality(allowed_action_set) > 0 and allowed_action_set <@ array[
            ''UNLOCK_OR_RESET_ADMIN'',
            ''RESTORE_CONTROLLED_CONFIG_RELEASE'',
            ''TRIGGER_BACKUP_OR_RESTORE''])
      )';
  end if;
end $$;

-- 单据号全库唯一（无法人列，偏离连带项）；到期失效任务按状态与截止时间扫描。
create unique index if not exists ux_breakglass_activations_doc_no
  on platform_core.breakglass_activations (doc_no);
create index if not exists ix_breakglass_activations_status_expires_at
  on platform_core.breakglass_activations (status, expires_at);
-- 时间序索引：本表无 legal_entity_id 列，取建档时间。
create index if not exists ix_breakglass_activations_created_at
  on platform_core.breakglass_activations (created_at);

-- 挂接乐观锁守卫（无法人列，不建行级安全策略），并断言基线索引齐备。
select platform_core.attach_table_guards('platform_core', 'breakglass_activations');
select platform_core.assert_baseline_indexes('platform_core', 'breakglass_activations', false);

reset role;
