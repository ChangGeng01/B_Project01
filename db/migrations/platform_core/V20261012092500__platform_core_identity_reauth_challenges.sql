-- rollback: drop table platform_core.reauth_challenges;
-- db/migrations/platform_core/V20261012092500__platform_core_identity_reauth_challenges.sql
-- 阶段 4 第 6 号迁移：重新认证挑战（04-identity-authz.md 表 3-6）。
-- operation_type 六类高风险操作，落库取大写下划线形态，与全库枚举风格一致，
-- 对应 HighRiskOperation 六变体 ContractEffective、Payment、InvoiceIssue、
-- LedgerPosting、PeriodClose、SensitiveExport。
-- subject_digest 为服务端按五项规范化算法重算的 SHA-256，不采信客户端值；
-- subject_summary 为掩码后的展示结构。X-Reauth-Token 单次消费由
-- status='VERIFIED' 的条件更新保证。
-- 本表不带 legal_entity_id 列、不建行级安全策略，登记行由第 29 号回填迁移写入。
-- 表经 DO 块创建，理由同阶段 2 第 3 号迁移：不带法人列的登记豁免表形态。
set role ep_mod_platform_core;
set lock_timeout = '5s';
set statement_timeout = '30min';

do $$
begin
  if not exists (select 1 from pg_class c
                 join pg_namespace n on n.oid = c.relnamespace
                 where n.nspname = 'platform_core' and c.relname = 'reauth_challenges') then
    execute '
      create table platform_core.reauth_challenges (
        id uuid not null,
        security_level smallint not null default 30,
        data_scope_tags text[] not null default ''{}'',
        row_version bigint not null default 1,
        created_at timestamptz not null default now(),
        created_by uuid not null default ''00000000-0000-7000-8000-000000000001'',
        updated_at timestamptz not null default now(),
        updated_by uuid not null default ''00000000-0000-7000-8000-000000000001'',
        user_id uuid not null,
        session_id uuid not null,
        operation_type text not null,
        subject_digest bytea not null,
        subject_summary jsonb not null,
        nonce bytea not null,
        credential_kind_used text null,
        status text not null,
        token_hash bytea null,
        issued_at timestamptz not null default now(),
        expires_at timestamptz not null,
        verified_at timestamptz null,
        consumed_at timestamptz null,
        failure_count int not null default 0,
        constraint pk_reauth_challenges primary key (id),
        constraint ck_reauth_challenges_operation_type check (
          operation_type in (''CONTRACT_EFFECTIVE'', ''PAYMENT'', ''INVOICE_ISSUE'',
                             ''LEDGER_POSTING'', ''PERIOD_CLOSE'', ''SENSITIVE_EXPORT'')),
        constraint ck_reauth_challenges_status check (
          status in (''ISSUED'', ''VERIFIED'', ''CONSUMED'', ''FAILED'',
                     ''EXPIRED'', ''ABANDONED'')),
        constraint ck_reauth_challenges_failure_count check (failure_count >= 0)
      )';
  end if;
end $$;

-- 按令牌摘要定位挑战；按用户清理过期与失败挑战。
create unique index if not exists ux_reauth_challenges_token_hash
  on platform_core.reauth_challenges (token_hash);
create index if not exists ix_reauth_challenges_user_id_status_expires_at
  on platform_core.reauth_challenges (user_id, status, expires_at);
-- 时间序索引：本表无 legal_entity_id 列，取建档时间。
create index if not exists ix_reauth_challenges_created_at
  on platform_core.reauth_challenges (created_at);

-- 挂接乐观锁守卫（无法人列，不建行级安全策略），并断言基线索引齐备。
select platform_core.attach_table_guards('platform_core', 'reauth_challenges');
select platform_core.assert_baseline_indexes('platform_core', 'reauth_challenges', false);

reset role;
