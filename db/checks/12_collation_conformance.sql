-- db/checks/12_collation_conformance.sql
-- 排序规则一致性断言（阶段 1 IT-31 判定位）。
-- 通过判据：返回 0 行。执行方：ep-migrate check。
--
-- 断言 ADR-0003 逐字要求的四项：`datlocprovider='c'`、`datcollate='C'`、`datctype='C'`、
-- `daticulocale IS NULL`。F-71 更正：本文件此前断言 `datlocprovider='i'`、
-- `daticulocale='zh-Hans-CN'`，即断言了裁定 00c 逐字「不得实现」的那一支。
--
-- 为什么不再比对 collation 版本：`datcollversion` 与
-- `pg_database_collation_actual_version()` 在 C 排序下都不参与版本追踪，恒为 NULL，
-- 比对它们是一条恒真判据。C 排序不受外部 collation 版本影响，正是选它的理由。
select d.datname as database_name,
       d.datlocprovider as loc_provider,
       d.datcollate as lc_collate,
       d.datctype as lc_ctype,
       d.daticulocale as icu_locale,
       'database collation deviates from ADR-0003' as problem
from pg_database d
where d.datname = current_database()
  and (d.datlocprovider <> 'c'
    or d.datcollate <> 'C'
    or d.datctype <> 'C'
    or d.daticulocale is not null);
