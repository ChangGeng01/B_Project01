-- db/checks/12_collation_conformance.sql
-- 阶段 2 计划 §3.9 第 12 项：排序规则一致性断言（阶段 1 IT-31 判定位）。
-- 通过判据：返回 0 行。执行方：ep-migrate check。
-- 断言本库 datlocprovider 为 i、daticulocale 为 zh-Hans-CN、
-- datcollversion 与 pg_database_collation_actual_version(oid) 相等。
select d.datname as database_name,
       d.datlocprovider as loc_provider,
       d.daticulocale as icu_locale,
       d.datcollversion as recorded_collation_version,
       pg_database_collation_actual_version(d.oid) as actual_collation_version,
       'database collation deviates from stage 2 bootstrap contract' as problem
from pg_database d
where d.datname = current_database()
  and (d.datlocprovider <> 'i'
    or d.daticulocale is distinct from 'zh-Hans-CN'
    or d.datcollversion is distinct from pg_database_collation_actual_version(d.oid));
