-- rollback: delete from platform_core.unpoliced_table_registry
-- rollback:   where (schema_name, table_name) in (
-- rollback:     ('platform_core', 'user_accounts'),
-- rollback:     ('platform_core', 'user_credentials'),
-- rollback:     ('platform_core', 'user_password_history'),
-- rollback:     ('platform_core', 'user_devices'),
-- rollback:     ('platform_core', 'sessions'),
-- rollback:     ('platform_core', 'reauth_challenges'),
-- rollback:     ('platform_core', 'login_attempts'),
-- rollback:     ('platform_core', 'account_lockouts'),
-- rollback:     ('platform_core', 'breakglass_activations'),
-- rollback:     ('platform_authz', 'permission_items'),
-- rollback:     ('platform_authz', 'object_scope_bindings'));
-- db/migrations/platform_core/V20261012113500__platform_core_backfill_unpoliced_table_registry.sql
-- 阶段 4 第 29 号迁移：本阶段 11 张不带法人列的表的正向登记（基线第 3.8 节
-- 偏离一改写后的登记制，04-identity-authz.md 第 3.4、12.2 节）。
-- 版本号晚于本阶段全部建表迁移，故列在最后。准入判据：
--   platform_core 九张身份主体表取 ISOLATION_OR_DEPLOYMENT_METADATA——
--   隔离机制自身的元数据，用户是可被授权多个法人的主体，给其行贴单一法人
--   标签在语义上不成立；法人可见性落在任何列出用户的查询一律与受策略约束的
--   platform_authz.user_legal_entity_grants 内联这一条上；
--   platform_authz 的 permission_items 与 object_scope_bindings 两张取
--   SAME_FOR_ALL_ENTITIES——行在本部署内对两个法人取值相同，不返回任何与
--   法人相关的行，可见性由授权判定第二阶段的对象级判定承担。
-- matrix_case_id 取 tests/rls_matrix 中各表承接入口的用例标识；核验判据为
-- 每张表的可见性不随 app.legal_entity_id 变化。
set role ep_mod_platform_core;
set lock_timeout = '5s';
set statement_timeout = '30min';

insert into platform_core.unpoliced_table_registry
  (id, schema_name, table_name, admission_basis, isolation_entry, matrix_case_id)
values
  ('00000000-0000-7000-8000-000000000209', 'platform_core', 'user_accounts',
   'ISOLATION_OR_DEPLOYMENT_METADATA',
   '身份主体表为隔离机制自身元数据，法人可见性落在列出用户的查询一律与受策略约束的 platform_authz.user_legal_entity_grants 内联',
   'rls_matrix_unpoliced_user_accounts'),
  ('00000000-0000-7000-8000-000000000210', 'platform_core', 'user_credentials',
   'ISOLATION_OR_DEPLOYMENT_METADATA',
   '凭据表为隔离机制自身元数据，法人可见性落在列出用户的查询一律与受策略约束的 platform_authz.user_legal_entity_grants 内联',
   'rls_matrix_unpoliced_user_credentials'),
  ('00000000-0000-7000-8000-000000000211', 'platform_core', 'user_password_history',
   'ISOLATION_OR_DEPLOYMENT_METADATA',
   '口令历史为隔离机制自身元数据，法人可见性落在列出用户的查询一律与受策略约束的 platform_authz.user_legal_entity_grants 内联',
   'rls_matrix_unpoliced_user_password_history'),
  ('00000000-0000-7000-8000-000000000212', 'platform_core', 'user_devices',
   'ISOLATION_OR_DEPLOYMENT_METADATA',
   '设备表为隔离机制自身元数据，法人可见性落在列出用户的查询一律与受策略约束的 platform_authz.user_legal_entity_grants 内联',
   'rls_matrix_unpoliced_user_devices'),
  ('00000000-0000-7000-8000-000000000213', 'platform_core', 'sessions',
   'ISOLATION_OR_DEPLOYMENT_METADATA',
   '会话表为隔离机制自身元数据，法人可见性落在列出用户的查询一律与受策略约束的 platform_authz.user_legal_entity_grants 内联',
   'rls_matrix_unpoliced_sessions'),
  ('00000000-0000-7000-8000-000000000214', 'platform_core', 'reauth_challenges',
   'ISOLATION_OR_DEPLOYMENT_METADATA',
   '重新认证挑战表为隔离机制自身元数据，法人可见性落在列出用户的查询一律与受策略约束的 platform_authz.user_legal_entity_grants 内联',
   'rls_matrix_unpoliced_reauth_challenges'),
  ('00000000-0000-7000-8000-000000000215', 'platform_core', 'login_attempts',
   'ISOLATION_OR_DEPLOYMENT_METADATA',
   '登录尝试表为隔离机制自身元数据，法人可见性落在列出用户的查询一律与受策略约束的 platform_authz.user_legal_entity_grants 内联',
   'rls_matrix_unpoliced_login_attempts'),
  ('00000000-0000-7000-8000-000000000216', 'platform_core', 'account_lockouts',
   'ISOLATION_OR_DEPLOYMENT_METADATA',
   '账号锁定计数表为隔离机制自身元数据，法人可见性落在列出用户的查询一律与受策略约束的 platform_authz.user_legal_entity_grants 内联',
   'rls_matrix_unpoliced_account_lockouts'),
  ('00000000-0000-7000-8000-000000000217', 'platform_core', 'breakglass_activations',
   'ISOLATION_OR_DEPLOYMENT_METADATA',
   '应急账号启用台账为隔离机制自身元数据，法人可见性落在列出用户的查询一律与受策略约束的 platform_authz.user_legal_entity_grants 内联',
   'rls_matrix_unpoliced_breakglass_activations'),
  ('00000000-0000-7000-8000-000000000218', 'platform_authz', 'permission_items',
   'SAME_FOR_ALL_ENTITIES',
   '权限项注册表行在本部署内对两个法人取值相同，不返回与法人相关的行，可见性由授权判定第二阶段的对象级判定承担',
   'rls_matrix_unpoliced_permission_items'),
  ('00000000-0000-7000-8000-000000000219', 'platform_authz', 'object_scope_bindings',
   'SAME_FOR_ALL_ENTITIES',
   '对象范围绑定表行在本部署内对两个法人取值相同，不返回与法人相关的行，可见性由授权判定第二阶段的对象级判定承担',
   'rls_matrix_unpoliced_object_scope_bindings')
on conflict on constraint ux_unpoliced_table_registry_schema_table do nothing;

reset role;
