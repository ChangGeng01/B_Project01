-- rollback: delete from platform_authz.role_permission_grants
-- rollback:   where id::text like '00000000-0000-7000-8000-0000000005%';
-- rollback: delete from platform_authz.authz_config_versions
-- rollback:   where id::text like '00000000-0000-7000-8000-0000000004%f';
-- rollback: delete from platform_authz.roles
-- rollback:   where id::text like '00000000-0000-7000-8000-0000000004%'
-- rollback:     and code in ('SYSTEM_ADMIN', 'DATA_ADMIN', 'SECURITY_ADMIN',
-- rollback:                  'AUDIT_ADMIN', 'KEY_ADMIN', 'CONFIG_ADMIN', 'BUSINESS_MINIMAL');
-- db/migrations/platform_authz/V20261012112500__platform_authz_backfill_admin_duty_roles.sql
-- 阶段 4 第 27 号迁移：管理员职责角色种子回填（04-identity-authz.md §3.5 第 27 号）。
-- 对 platform_core.legal_entities 现存每一法人（按 entity_no 升序）写入：
--   一、七个角色：五类管理员 SYSTEM_ADMIN、DATA_ADMIN、SECURITY_ADMIN、
--       AUDIT_ADMIN、KEY_ADMIN（duty_class 对应 SYSTEM、DATA、SECURITY、AUDIT、KEY）、
--       CONFIG_ADMIN（duty_class 取 CONFIG，U-B-17 临时口径）与一个最小业务角色
--       BUSINESS_MINIMAL（duty_class 为空），lifecycle_state 一律 EFFECTIVE；
--       种子角色包只含五类管理员与一个最小业务角色，业务角色包留空（U-B-01/U-B-02）。
--   二、九行角色权限授予：SYSTEM_ADMIN 取 platform.user_accounts 与
--       platform.roles 各 VIEW、CREATE、UPDATE 六行；SECURITY_ADMIN 取
--       platform.high_risk_requests 的 VIEW、APPROVE 两行；AUDIT_ADMIN 取
--       platform.high_risk_requests 的 VIEW 一行。
--   三、一行 version_no 取 1、state 取 EFFECTIVE 的 authz_config_versions：
--       两个法人各一行，是本阶段运行期唯一的生效版本来源，启动自检
--       authz-snapshot-loadable 据此构造快照；checksum 按该法人本文件写入的
--       配置行规范化文本现算 sha256（经 pgcrypto 的 digest）。
-- 种子行 id 编号：角色 04 段、授予 05 段、版本 04 段末位 f，法人序号占十六位高位，
-- 循环法人下标保证各法人行间不冲突。法人表为空时本迁移为无害空操作，
-- 法人行由 ep-datagen 与后续装配写入后由配置发布通道补版本。
-- 依赖：digest 需要 pgcrypto；ep_migrator 具 CREATE ON DATABASE，可建 trusted 扩展。
-- 显式落位 platform_core：引导后 public schema 的 CREATE 只归库属主，ep_migrator
-- 的默认 search_path 里没有可建 schema，不指定会报 no schema has been selected
-- to create in；ep_migrator 是 ep_mod_platform_core 成员，可在该 schema 建扩展。
create extension if not exists pgcrypto schema platform_core;

-- 读取法人清单需要 platform_core 的跨 schema 选择权（schema USAGE 已在第 19 号迁移授予）。
set role ep_mod_platform_core;
grant select on platform_core.legal_entities to ep_mod_platform_authz;
reset role;

set role ep_mod_platform_authz;
set lock_timeout = '5s';
set statement_timeout = '30min';

do $$
declare
  v_le record;
  v_le_no int := 0;
  v_codes text[] := array['SYSTEM_ADMIN', 'DATA_ADMIN', 'SECURITY_ADMIN',
    'AUDIT_ADMIN', 'KEY_ADMIN', 'CONFIG_ADMIN', 'BUSINESS_MINIMAL'];
  v_names text[] := array['系统管理员', '数据管理员', '安全管理员',
    '审计管理员', '密钥管理员', '配置管理员', '最小业务角色'];
  v_duties text[] := array['SYSTEM', 'DATA', 'SECURITY', 'AUDIT', 'KEY', 'CONFIG'];
  v_grant_role text[] := array['SYSTEM_ADMIN', 'SYSTEM_ADMIN', 'SYSTEM_ADMIN',
    'SYSTEM_ADMIN', 'SYSTEM_ADMIN', 'SYSTEM_ADMIN',
    'SECURITY_ADMIN', 'SECURITY_ADMIN', 'AUDIT_ADMIN'];
  v_grant_item text[] := array['platform.user_accounts', 'platform.user_accounts',
    'platform.user_accounts', 'platform.roles', 'platform.roles', 'platform.roles',
    'platform.high_risk_requests', 'platform.high_risk_requests',
    'platform.high_risk_requests'];
  v_grant_action text[] := array['VIEW', 'CREATE', 'UPDATE', 'VIEW', 'CREATE',
    'UPDATE', 'VIEW', 'APPROVE', 'VIEW'];
  i int;
  v_role_id uuid;
  v_checksum_src text;
begin
  for v_le in
    select id from platform_core.legal_entities order by entity_no asc
  loop
    v_le_no := v_le_no + 1;
    v_checksum_src := 'authz_config_version=1;legal_entity=' || v_le.id::text;

    for i in 1..7 loop
      v_role_id := ('00000000-0000-7000-8000-0000000004'
        || lpad(to_hex(v_le_no * 16 + i), 2, '0'))::uuid;
      insert into platform_authz.roles
        (id, legal_entity_id, code, name, duty_class, is_portal_role,
         lifecycle_state, is_active)
      values
        (v_role_id, v_le.id, v_codes[i], v_names[i],
         case when i <= 6 then v_duties[i] else null end,
         false, 'EFFECTIVE', true)
      on conflict on constraint ux_roles_legal_entity_id_code do nothing;
      v_checksum_src := v_checksum_src || ';role=' || v_codes[i];
    end loop;

    for i in 1..9 loop
      insert into platform_authz.role_permission_grants
        (id, legal_entity_id, role_id, permission_item_code, action)
      values
        (('00000000-0000-7000-8000-0000000005'
          || lpad(to_hex(v_le_no * 16 + i), 2, '0'))::uuid,
         v_le.id,
         (select id from platform_authz.roles
           where legal_entity_id = v_le.id and code = v_grant_role[i]),
         v_grant_item[i], v_grant_action[i])
      on conflict (legal_entity_id, role_id, permission_item_code, action)
        do nothing;
      v_checksum_src := v_checksum_src || ';grant=' || v_grant_role[i]
        || ':' || v_grant_item[i] || ':' || v_grant_action[i];
    end loop;

    insert into platform_authz.authz_config_versions
      (id, legal_entity_id, version_no, state, checksum)
    values
      (('00000000-0000-7000-8000-0000000004'
        || lpad(to_hex(v_le_no * 16 + 15), 2, '0'))::uuid,
       v_le.id, 1, 'EFFECTIVE', platform_core.digest(v_checksum_src, 'sha256'))
    on conflict on constraint ux_authz_config_versions_legal_entity_id_version_no
      do nothing;
  end loop;
end $$;

reset role;
