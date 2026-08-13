-- rollback: delete from platform_authz.permission_items where code in
-- rollback:   ('platform.user_accounts', 'platform.roles', 'platform.high_risk_requests',
-- rollback:    'platform.contract_effective', 'platform.payment', 'platform.invoice_issue',
-- rollback:    'platform.ledger_posting', 'platform.period_close', 'platform.sensitive_export');
-- db/migrations/platform_authz/V20261012112000__platform_authz_backfill_permission_item_seed.sql
-- 阶段 4 第 26 号迁移：权限项种子回填（04-identity-authz.md §3.5 第 26 号）。
-- 共 9 行，module_code 一律 platform：
--   一、platform 自身三个对象的管理权限项 3 行，object_type 即第 12 号迁移登记的
--       三个对象类型（platform.user_accounts、platform.roles、
--       platform.high_risk_requests）；
--   二、六类高风险操作各 1 行共 6 行，allowed_actions 恰好 SUBMIT 一个动作：
--       提交高风险请求单是该权限项唯一承载的动作，审批动作由审批链承担。
--       六类操作对应的业务对象尚未登记 object_scope_bindings，其 object_type
--       暂取操作码自身保持自洽，业务对象登记与其权限项细化归其所属阶段。
-- 种子行 id 取冻结编号 0301 至 0309；created_by 默认 SYSTEM_PRINCIPAL_ID。
-- 回退按 code 删除种子行（04:L225）。
set role ep_mod_platform_authz;
set lock_timeout = '5s';
set statement_timeout = '30min';

insert into platform_authz.permission_items
  (id, code, module_code, function_point, allowed_actions, object_type, description)
values
  ('00000000-0000-7000-8000-000000000301', 'platform.user_accounts', 'platform',
   '用户账号档案维护', array['VIEW', 'CREATE', 'UPDATE'], 'platform.user_accounts',
   '用户账号的建立、查看与生命周期维护'),
  ('00000000-0000-7000-8000-000000000302', 'platform.roles', 'platform',
   '角色档案维护', array['VIEW', 'CREATE', 'UPDATE'], 'platform.roles',
   '角色的建立、查看与生命周期维护'),
  ('00000000-0000-7000-8000-000000000303', 'platform.high_risk_requests', 'platform',
   '高风险请求单查看与审批', array['VIEW', 'APPROVE'], 'platform.high_risk_requests',
   '高风险请求单的查看与审批'),
  ('00000000-0000-7000-8000-000000000304', 'platform.contract_effective', 'platform',
   '高风险操作：合同生效', array['SUBMIT'], 'platform.contract_effective',
   '提交合同生效高风险请求'),
  ('00000000-0000-7000-8000-000000000305', 'platform.payment', 'platform',
   '高风险操作：资金支付', array['SUBMIT'], 'platform.payment',
   '提交资金支付高风险请求'),
  ('00000000-0000-7000-8000-000000000306', 'platform.invoice_issue', 'platform',
   '高风险操作：发票开具', array['SUBMIT'], 'platform.invoice_issue',
   '提交发票开具高风险请求'),
  ('00000000-0000-7000-8000-000000000307', 'platform.ledger_posting', 'platform',
   '高风险操作：总账过账', array['SUBMIT'], 'platform.ledger_posting',
   '提交总账过账高风险请求'),
  ('00000000-0000-7000-8000-000000000308', 'platform.period_close', 'platform',
   '高风险操作：期间关账', array['SUBMIT'], 'platform.period_close',
   '提交期间关账高风险请求'),
  ('00000000-0000-7000-8000-000000000309', 'platform.sensitive_export', 'platform',
   '高风险操作：敏感导出', array['SUBMIT'], 'platform.sensitive_export',
   '提交敏感导出高风险请求')
on conflict on constraint ux_permission_items_code do nothing;

reset role;
