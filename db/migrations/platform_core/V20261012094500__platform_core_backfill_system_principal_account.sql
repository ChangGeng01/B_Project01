-- rollback: delete from platform_core.user_accounts
-- rollback:   where id = '00000000-0000-7000-8000-000000000001';
-- db/migrations/platform_core/V20261012094500__platform_core_backfill_system_principal_account.sql
-- 阶段 4 第 10 号迁移：系统主体账号种子（04-identity-authz.md 第 3.5 节第 10 号）。
-- 写入 00000000-0000-7000-8000-000000000001 的系统主体账号行：account_kind 取
-- SYSTEM，login_name 取 system，status 取 ACTIVE，无凭据。该取值即 ep-foundation
-- 的 SYSTEM_PRINCIPAL_ID 常量，按 A-02 由阶段 1 冻结，不得自选其他值。
-- home_legal_entity_id 取占位常量 00000000-0000-7000-8000-000000000000：
-- 种子阶段尚无法人行，该列只用于审计事件分段与默认法人、不参与访问判定且
-- 不建外键，占位取值不产生任何判定后果。
set role ep_mod_platform_core;
set lock_timeout = '5s';
set statement_timeout = '30min';

insert into platform_core.user_accounts
  (id, account_kind, login_name, employee_no, display_name,
   home_legal_entity_id, clearance_level, status, is_mfa_required,
   activated_on, security_level)
values
  ('00000000-0000-7000-8000-000000000001', 'SYSTEM', 'system', null, '系统主体',
   '00000000-0000-7000-8000-000000000000', 20, 'ACTIVE', false,
   '2026-07-01', 30)
on conflict (login_name) do nothing;

reset role;
