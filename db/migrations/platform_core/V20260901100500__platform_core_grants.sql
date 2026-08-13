-- rollback: revoke select, insert, update on all tables in schema platform_core from ep_app_rw;
-- rollback: revoke select on all tables in schema platform_core from ep_analyst_ro;
-- db/migrations/platform_core/V20260901100500__platform_core_grants.sql
-- platform_core 全部对象的显式授权收口（阶段 2 计划第 3.4 节第 15 号迁移）。
-- 第 1 号迁移的默认权限只覆盖建表时点之后的对象语义，本迁移对既有对象显式授予，
-- 二者取值一致。默认权限与显式授予均不含 DELETE：仅追加口径在数据库侧强制，
-- DELETE 只在 platform_msg.idempotency_keys 与 platform_ops 过期快照表两处授予，
-- 均不在本 schema。GRANT 对已有权限是幂等的空操作。
set role ep_mod_platform_core;
set lock_timeout = '5s';
set statement_timeout = '30min';

grant select, insert, update on all tables in schema platform_core to ep_app_rw;
grant select on all tables in schema platform_core to ep_analyst_ro;

-- 全部 24 个属主角色在建表迁移末尾调用 platform_core.attach_table_guards 与
-- assert_baseline_indexes，跨 schema 调用要求本 schema 的 USAGE；
-- 函数本身的 EXECUTE 默认对 PUBLIC 开放，缺的只是 USAGE 一层。
do $$
declare
  s text;
begin
  for s in
    select unnest(array[
      'platform_core', 'platform_authz', 'platform_meta', 'platform_flow',
      'platform_audit', 'platform_msg', 'platform_file', 'platform_ops', 'ext',
      'mdm', 'crm', 'cpq', 'clm', 'sales', 'procure', 'inventory', 'costing',
      'project', 'service', 'finance', 'ledger', 'invoice', 'portal', 'reporting'])
  loop
    execute format('grant usage on schema platform_core to %I', 'ep_mod_' || s);
  end loop;
end $$;

-- 同上：attach_table_guards 以调用方权限读 append_only_registry 登记表；
-- 登记表创建迁移（V20260901093000）已授一次，此处对全 schema 属主角色再授，
-- GRANT 幂等，保证后续任何建表迁移的守卫挂接不因授权缺失而失败。
do $$
declare
  s text;
begin
  for s in
    select unnest(array[
      'platform_core', 'platform_authz', 'platform_meta', 'platform_flow',
      'platform_audit', 'platform_msg', 'platform_file', 'platform_ops', 'ext',
      'mdm', 'crm', 'cpq', 'clm', 'sales', 'procure', 'inventory', 'costing',
      'project', 'service', 'finance', 'ledger', 'invoice', 'portal', 'reporting'])
  loop
    execute format('grant select on platform_core.append_only_registry to %I', 'ep_mod_' || s);
  end loop;
end $$;

reset role;
