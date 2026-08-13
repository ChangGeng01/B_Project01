-- rollback: 回退只能用升级前备份或影子表回退。schema 一旦承载授权与后续阶段对象，
-- rollback: 逐一撤销的影响面超出本文件可安全逆向的范围，不提供 DROP 语句。
-- db/migrations/invoice/V20260901120000__invoice_create_schema.sql
-- 建 invoice schema，设属主为 ep_mod_invoice，授 USAGE 与默认权限（阶段 2 计划第 3.2 节）。
-- 默认权限不含 DELETE：仅追加口径在数据库侧强制，不依赖 CI 静态检查独自承担。
set role ep_mod_invoice;

-- 建 schema 需要库上的 CREATE 权限，只有会话角色 ep_migrator 持有该权限；
-- 此处先回到会话角色完成建 schema 与属主归位。
reset role;
create schema if not exists invoice;
alter schema invoice owner to ep_mod_invoice;

-- 三条固定授权：USAGE 给应用读写与只读分析两个角色；
-- 属主角色日后在本 schema 建的表，默认授予 ep_app_rw 三种 DML（无 DELETE）、
-- ep_analyst_ro 只读。
grant usage on schema invoice to ep_app_rw, ep_analyst_ro;
alter default privileges for role ep_mod_invoice in schema invoice
  grant select, insert, update on tables to ep_app_rw;
alter default privileges for role ep_mod_invoice in schema invoice
  grant select on tables to ep_analyst_ro;

reset role;
