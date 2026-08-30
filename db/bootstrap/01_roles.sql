-- db/bootstrap/01_roles.sql
-- 集群引导第 2 步，共 5 步。执行顺序：00 → 01 → 02 → 03 → 04，见本目录 README.md。
--
-- 承载内容：七个功能角色与二十四个属主角色（ep_mod_<schema> × 24）的创建、
-- 属性重申与权限边界。逐项属性与权限边界的唯一出处是阶段 2 计划第 3.1 节的角色表。
--
-- 为什么角色不走迁移而落在引导路径：角色是簇级对象，而 ep_migrator 按技术基线第 3.1 节
-- 不具备角色管理权限，迁移路径在权限上执行不了这些语句。
--
-- 为什么排在 02、03、04 之前：02 的连接数上限要把超级用户预留之外的份额分给这些角色；
-- 03 的 ALTER ROLE ... SET 与 04 的认证行都按角色名寻址，角色不存在则语句直接报错。
--
-- 为什么不写口令：口令由安装器从机密库读取后经 ALTER ROLE ... PASSWORD 单独注入。
-- 本目录任何文件中出现口令字面量即违反 xtask sqlcheck 规则 SQL-020。
--
-- 可重复执行形态：CREATE ROLE 以 pg_roles 存在性检查守护；其后的 ALTER ROLE 一律重申
-- 计划属性，重复执行时把属性收敛回计划取值，不产生错误。

-- 一、建角色。存在性检查保证重复执行不报「角色已存在」。
do $$
declare
  s text;
begin
  -- 七个功能角色。
  if not exists (select 1 from pg_roles where rolname = 'ep_app_rw') then
    create role ep_app_rw;
  end if;
  if not exists (select 1 from pg_roles where rolname = 'ep_analyst_ro') then
    create role ep_analyst_ro;
  end if;
  if not exists (select 1 from pg_roles where rolname = 'ep_ops_ro') then
    create role ep_ops_ro;
  end if;
  if not exists (select 1 from pg_roles where rolname = 'ep_migrator') then
    create role ep_migrator;
  end if;
  if not exists (select 1 from pg_roles where rolname = 'ep_breakglass') then
    create role ep_breakglass;
  end if;
  if not exists (select 1 from pg_roles where rolname = 'ep_archiver') then
    create role ep_archiver;
  end if;
  if not exists (select 1 from pg_roles where rolname = 'ep_backuper') then
    create role ep_backuper;
  end if;
  -- 二十四个属主角色，与 24 个 schema 一一对应，NOLOGIN，仅归属与 DDL 边界。
  for s in
    select unnest(array[
      'platform_core', 'platform_authz', 'platform_meta', 'platform_flow',
      'platform_audit', 'platform_msg', 'platform_file', 'platform_ops', 'ext',
      'mdm', 'crm', 'cpq', 'clm', 'sales', 'procure', 'inventory', 'costing',
      'project', 'service', 'finance', 'ledger', 'invoice', 'portal', 'reporting'])
  loop
    if not exists (select 1 from pg_roles where rolname = 'ep_mod_' || s) then
      execute format('create role %I', 'ep_mod_' || s);
    end if;
  end loop;
end $$;

-- 二、属性重申。重复执行时把属性收敛回阶段 2 计划第 3.1 节的取值。
-- 应用读写角色：NOINHERIT 使其不自动继承被授予角色的权限；无 DDL、无角色管理、无策略管理。
alter role ep_app_rw login nosuperuser nocreatedb nocreaterole noreplication nobypassrls noinherit;
-- 只读分析角色：不授予 pg_read_all_stats，复制会话与复制槽的观察落在 ep_ops_ro。
alter role ep_analyst_ro login nosuperuser nobypassrls;
-- 运维只读角色：只对 platform_ops 的视图授 SELECT（视图出现时由对应迁移授予），加 pg_read_all_stats。
alter role ep_ops_ro login nosuperuser nobypassrls;
-- 迁移角色：只在迁移窗口内启用；CREATE ON DATABASE 与被授予全部 ep_mod_* 见下方权限边界。
alter role ep_migrator login nosuperuser nocreaterole;
-- 应急账号：NOLOGIN 为常态。启用时由运维流程执行
-- alter role ep_breakglass login valid until '<启用时刻 + 不超过 8 小时>';
-- 用后轮换口令并复位 NOLOGIN。启用口令同样不进本文件（SQL-020）。
alter role ep_breakglass nologin nosuperuser;
-- 两个复制角色：REPLICATION，无任何业务表权限，只能建复制连接。
alter role ep_archiver login replication nosuperuser nobypassrls;
alter role ep_backuper login replication nosuperuser nobypassrls;
-- 属主角色一律 NOLOGIN。
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
    execute format('alter role %I nologin', 'ep_mod_' || s);
  end loop;
end $$;

-- 三、权限边界。
-- 1) ep_migrator：库上的 CONNECT 与 CREATE（迁移建 schema 所需）与全部 ep_mod_* 成员资格
--    （使迁移文件的 SET ROLE ep_mod_<schema> 与其属主归位成立）；只在迁移窗口内启用。
--    00 已 REVOKE ALL FROM PUBLIC，CONNECT 必须显式授予，否则迁移账号连不进库。
grant connect, create on database ep to ep_migrator;
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
    -- GRANT 成员资格对已有成员是幂等的空操作。
    execute format('grant %I to ep_migrator', 'ep_mod_' || s);
  end loop;
end $$;

-- 2) 四个运行期角色的库级 CONNECT。
--    00 已 REVOKE ALL FROM PUBLIC，CONNECT 必须逐个显式授予，否则这些账号根本连不进库
--    ——引导脚本、全部迁移与十三项 check 仍会全绿、退出码 0，失败只在应用第一次建连时
--    以「握手阶段被拒」的形态出现，而 db 侧没有任何一项判据看得见它（F-78 补；
--    上一版只给 ep_migrator 授了 CONNECT，四个运行期角色全部遗漏）。
--    ep_breakglass 尤其不能漏：它存在的全部意义就是别的路都断了时还能进去。
grant connect on database ep to ep_app_rw, ep_analyst_ro, ep_ops_ro, ep_breakglass;

-- 3) ep_ops_ro：观察复制与集群统计所需的最小只读成员资格。
grant pg_read_all_stats to ep_ops_ro;

-- 4) 复制角色：无业务表权限，且不得连接业务库，仅复制连接（配合 04_pg_hba.fragment）。
revoke connect on database ep from ep_archiver, ep_backuper;
