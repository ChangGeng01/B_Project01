-- rollback: delete from platform_authz.sod_rules
-- rollback:   where rule_kind = 'DUTY_EXCLUSION'
-- rollback:     and id::text like '00000000-0000-7000-8000-0000000006%';
-- db/migrations/platform_authz/V20261012113000__platform_authz_backfill_default_sod_rules.sql
-- 阶段 4 第 28 号迁移：默认职责分离规则回填（04-identity-authz.md §3.5 第 28 号、§4.5 第 1 条）。
-- 对 platform_core.legal_entities 现存每一法人写入 11 条 DUTY_EXCLUSION 规则：
--   一、SYSTEM、DATA、SECURITY、AUDIT、KEY 五类管理员职责两两互斥共 10 对
--       （照抄规格第 12.2 章）；
--   二、CONFIG 与 SECURITY 互斥 1 对（PRD 附录乙 U-B-17 待决的本阶段临时取值：
--       CONFIG 与 SECURITY 互斥、与其余四类可兼）。
-- enforcement 一律 BLOCK；message_code 一律 PLATFORM.SOD.DUTY_CONFLICT
-- （PRD 第 10.2.2 节「异常提示需指出被拒绝的具体规则名称」的承载）。
-- 互斥关系不进 DutyClass 枚举定义，只落在本表种子行（04:L282）。
-- 种子行 id 取 06 段，法人序号占十六位高位；法人表为空时为无害空操作。
-- 回退按种子 id 段删除本回填写入的规则行。
set role ep_mod_platform_authz;
set lock_timeout = '5s';
set statement_timeout = '30min';

do $$
declare
  v_le record;
  v_le_no int := 0;
  v_left text[] := array['SYSTEM', 'SYSTEM', 'SYSTEM', 'SYSTEM',
    'DATA', 'DATA', 'DATA', 'SECURITY', 'SECURITY', 'AUDIT', 'CONFIG'];
  v_right text[] := array['DATA', 'SECURITY', 'AUDIT', 'KEY',
    'SECURITY', 'AUDIT', 'KEY', 'AUDIT', 'KEY', 'KEY', 'SECURITY'];
  i int;
begin
  for v_le in
    select id from platform_core.legal_entities order by entity_no asc
  loop
    v_le_no := v_le_no + 1;

    -- 逐法人设置行级安全上下文。platform_authz.sod_rules 经
    -- platform_core.attach_table_guards 挂了 apply_le_rls，即 `force row level
    -- security` 加策略 `legal_entity_id = nullif(current_setting('app.legal_entity_id',
    -- true), '')::uuid`。FORCE 之下连表属主也受策略约束：不设该变量时
    -- current_setting 返回空串、nullif 得 NULL，WITH CHECK 恒不成立，下面的
    -- insert 会被拒。本迁移今天不失败只是因为全新引导时法人表为空、循环体一次
    -- 也不执行。第三个实参取 true：只在本事务内生效（F-80，与 112500 同批同病）。
    perform set_config('app.legal_entity_id', v_le.id::text, true);

    for i in 1..11 loop
      insert into platform_authz.sod_rules
        (id, legal_entity_id, rule_code, rule_kind, left_ref, right_ref,
         enforcement, message_code)
      values
        (('00000000-0000-7000-8000-0000000006'
          || lpad(to_hex(v_le_no * 16 + i), 2, '0'))::uuid,
         v_le.id,
         'DUTY_EXCLUSION_' || v_left[i] || '_' || v_right[i],
         'DUTY_EXCLUSION', v_left[i], v_right[i],
         'BLOCK', 'PLATFORM.SOD.DUTY_CONFLICT')
      on conflict on constraint ux_sod_rules_legal_entity_id_rule_code do nothing;
    end loop;
  end loop;
end $$;

reset role;
