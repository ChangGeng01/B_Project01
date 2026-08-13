-- rollback: drop table platform_authz.approval_chain_nodes;
-- db/migrations/platform_authz/V20261012110500__platform_authz_approval_chain_nodes.sql
-- 阶段 4 第 23 号迁移：审批链节点（04-identity-authz.md 表 3-22）。
-- approver_kind 三类：ROLE（该法人内持有该角色的有效用户）、POSITION（该岗位
-- 在岗用户）、DEPT_MANAGER（发起人所在部门链上的负责人）；ROLE 类经 role_code
-- 引用，POSITION 与 DEPT_MANAGER 类经 approver_ref 引用对应档案行。
-- 表上没有 allow_skip 一类列：越权跳过不是被校验拒绝的配置，而是根本没有
-- 承载它的字段（规格第 12.2 章「审批链不可越权跳过」在本设计中不是一个被
-- 校验的规则，而是一个不存在的能力）。
-- node_no 自 1 起连续无空洞、quorum 为正且不超过节点展开后的用户数，由第 4.5
-- 节静态校验纯函数在保存期与运行期执行，表上仅承载基本取值域约束。
-- 唯一索引全称 ux_approval_chain_nodes_legal_entity_id_approval_chain_id_node_no
-- 共 69 字节超过 63 字节上限，按列序缩写为
-- ux_approval_chain_nodes_le_id_approval_chain_id_node_no（55 字节）：
-- legal_entity_id 缩为 le_id，全称登记数据字典归集成任务。
-- 行级安全策略经 platform_core.apply_le_rls 生成（rls_approval_chain_nodes_le），不手写。
set role ep_mod_platform_authz;
set lock_timeout = '5s';
set statement_timeout = '30min';

create table if not exists platform_authz.approval_chain_nodes (
  id uuid not null,
  legal_entity_id uuid not null,
  security_level smallint not null default 20,
  data_scope_tags text[] not null default '{}',
  row_version bigint not null default 1,
  created_at timestamptz not null default now(),
  created_by uuid not null default '00000000-0000-7000-8000-000000000001',
  updated_at timestamptz not null default now(),
  updated_by uuid not null default '00000000-0000-7000-8000-000000000001',
  approval_chain_id uuid not null,
  node_no int not null,
  approver_kind text not null,
  approver_ref uuid null,
  role_code text null,
  quorum int not null default 1,
  timeout_hours int null,
  constraint pk_approval_chain_nodes primary key (id),
  constraint ck_approval_chain_nodes_approver_kind check (
    approver_kind in ('ROLE', 'POSITION', 'DEPT_MANAGER')),
  constraint ck_approval_chain_nodes_node_no check (node_no >= 1),
  constraint ck_approval_chain_nodes_quorum check (quorum >= 1),
  constraint ck_approval_chain_nodes_timeout check (
    timeout_hours is null or timeout_hours >= 1),
  constraint ck_approval_chain_nodes_role_code_len check (
    role_code is null or length(role_code) between 1 and 64)
);

-- 同链内节点号唯一（索引名为全称按列序缩写，见文件头注）。
create unique index if not exists ux_approval_chain_nodes_le_id_approval_chain_id_node_no
  on platform_authz.approval_chain_nodes (legal_entity_id, approval_chain_id, node_no);
-- 基线时间序索引。
create index if not exists ix_approval_chain_nodes_legal_entity_id_created_at
  on platform_authz.approval_chain_nodes (legal_entity_id, created_at);

-- 挂接乐观锁守卫与行级安全策略（rls_approval_chain_nodes_le），断言基线索引齐备。
select platform_core.attach_table_guards('platform_authz', 'approval_chain_nodes');
select platform_core.assert_baseline_indexes('platform_authz', 'approval_chain_nodes', false);

reset role;
