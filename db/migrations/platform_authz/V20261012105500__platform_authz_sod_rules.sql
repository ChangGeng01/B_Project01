-- rollback: drop table platform_authz.sod_rules;
-- db/migrations/platform_authz/V20261012105500__platform_authz_sod_rules.sql
-- 阶段 4 第 21 号迁移：职责分离规则（04-identity-authz.md 表 3-20）。
-- rule_kind 四类：DUTY_EXCLUSION（职责互斥）、ROLE_EXCLUSION（角色互斥对）、
-- SELF_APPROVAL（自审批禁止）、CHAIN_SKIP（节点连续性与 quorum 合法性）。
-- 四类规则在配置保存时执行一次、运行期提交时再执行一次，两次用同一份纯函数。
-- enforcement 默认 BLOCK；message_code 指向 docs/error-codes.md 中的错误码，
-- 用于满足 PRD 第 10.2.2 节「异常提示需指出被拒绝的具体规则名称」。
-- left_ref 与 right_ref 为规则两端引用的文本形态（职责类或角色码）。
-- 行级安全策略经 platform_core.apply_le_rls 生成（rls_sod_rules_le），不手写。
set role ep_mod_platform_authz;
set lock_timeout = '5s';
set statement_timeout = '30min';

create table if not exists platform_authz.sod_rules (
  id uuid not null,
  legal_entity_id uuid not null,
  security_level smallint not null default 20,
  data_scope_tags text[] not null default '{}',
  row_version bigint not null default 1,
  created_at timestamptz not null default now(),
  created_by uuid not null default '00000000-0000-7000-8000-000000000001',
  updated_at timestamptz not null default now(),
  updated_by uuid not null default '00000000-0000-7000-8000-000000000001',
  rule_code text not null,
  rule_kind text not null,
  left_ref text null,
  right_ref text null,
  enforcement text not null default 'BLOCK',
  message_code text not null,
  constraint pk_sod_rules primary key (id),
  constraint ux_sod_rules_legal_entity_id_rule_code unique (legal_entity_id, rule_code),
  constraint ck_sod_rules_rule_kind check (
    rule_kind in ('DUTY_EXCLUSION', 'ROLE_EXCLUSION', 'SELF_APPROVAL', 'CHAIN_SKIP')),
  constraint ck_sod_rules_rule_code_len check (length(rule_code) between 1 and 128),
  constraint ck_sod_rules_message_code_len check (length(message_code) between 1 and 128)
);

-- 基线时间序索引。
create index if not exists ix_sod_rules_legal_entity_id_created_at
  on platform_authz.sod_rules (legal_entity_id, created_at);

-- 挂接乐观锁守卫与行级安全策略（rls_sod_rules_le），断言基线索引齐备。
select platform_core.attach_table_guards('platform_authz', 'sod_rules');
select platform_core.assert_baseline_indexes('platform_authz', 'sod_rules', false);

reset role;
