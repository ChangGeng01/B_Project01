-- rollback: drop table platform_authz.high_risk_requests;
-- db/migrations/platform_authz/V20261012111000__platform_authz_high_risk_requests.sql
-- 阶段 4 第 24 号迁移：高风险请求单（04-identity-authz.md 表 3-23，单据类）。
-- doc_no 类型码 HRR；status 十一态为第 4.4 节状态机逐字取值；operation_type
-- 六类取阶段 1 冻结的 SCREAMING_SNAKE 操作码。
-- approval_instance_ref 为流程引擎实例的逻辑引用，跨平台组件不建外键；
-- reauth_challenge_id 引用 platform_core.reauth_challenges（身份侧九表不带法人列，
-- 跨 schema 引用不建外键，保持身份表登记制形态）。
-- 本表为单据类：assert_baseline_indexes 第三参取 true，断言
-- ux_high_risk_requests_legal_entity_id_doc_no 齐备。
-- 行级安全策略经 platform_core.apply_le_rls 生成（rls_high_risk_requests_le），不手写。
set role ep_mod_platform_authz;
set lock_timeout = '5s';
set statement_timeout = '30min';

create table if not exists platform_authz.high_risk_requests (
  id uuid not null,
  legal_entity_id uuid not null,
  security_level smallint not null default 20,
  data_scope_tags text[] not null default '{}',
  row_version bigint not null default 1,
  created_at timestamptz not null default now(),
  created_by uuid not null default '00000000-0000-7000-8000-000000000001',
  updated_at timestamptz not null default now(),
  updated_by uuid not null default '00000000-0000-7000-8000-000000000001',
  doc_no text not null,
  status text not null,
  operation_type text not null,
  subject_object_type text not null,
  subject_object_id uuid not null,
  subject_digest bytea not null,
  reauth_challenge_id uuid null,
  approval_chain_id uuid not null,
  approval_instance_ref uuid null,
  initiator_user_id uuid not null,
  initiator_device_id text not null,
  submitted_at timestamptz not null,
  decided_at timestamptz null,
  executed_at timestamptz null,
  execution_ref uuid null,
  reject_reason text null,
  constraint pk_high_risk_requests primary key (id),
  constraint ux_high_risk_requests_legal_entity_id_doc_no
    unique (legal_entity_id, doc_no),
  constraint ck_high_risk_requests_doc_no_len check (length(doc_no) between 1 and 64),
  constraint ck_high_risk_requests_status check (
    status in (
      'PENDING_INITIATION', 'PENDING_REAUTH', 'REAUTH_FAILED', 'LOCKED',
      'REAUTH_PASSED', 'IN_APPROVAL', 'APPROVED', 'REJECTED', 'WITHDRAWN',
      'ABANDONED', 'EXECUTED')),
  constraint ck_high_risk_requests_operation_type check (
    operation_type in (
      'CONTRACT_EFFECTIVE', 'PAYMENT', 'INVOICE_ISSUE', 'LEDGER_POSTING',
      'PERIOD_CLOSE', 'SENSITIVE_EXPORT')),
  constraint ck_high_risk_requests_subject_object_type_len check (
    length(subject_object_type) between 1 and 128),
  constraint ck_high_risk_requests_initiator_device_id_len check (
    length(initiator_device_id) between 1 and 64)
);

-- 基线时间序索引；按状态与操作类型检索开放单据
-- （索引名 ix_high_risk_requests_legal_entity_id_status_operation_type，59 字节）。
create index if not exists ix_high_risk_requests_legal_entity_id_created_at
  on platform_authz.high_risk_requests (legal_entity_id, created_at);
create index if not exists ix_high_risk_requests_legal_entity_id_status_operation_type
  on platform_authz.high_risk_requests (legal_entity_id, status, operation_type);

-- 挂接乐观锁守卫与行级安全策略（rls_high_risk_requests_le），
-- 单据类断言（含 ux_high_risk_requests_legal_entity_id_doc_no）。
select platform_core.attach_table_guards('platform_authz', 'high_risk_requests');
select platform_core.assert_baseline_indexes('platform_authz', 'high_risk_requests', true);

reset role;
