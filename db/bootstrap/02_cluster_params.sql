-- db/bootstrap/02_cluster_params.sql
-- 集群引导第 3 步，共 5 步。执行顺序：00 → 01 → 02 → 03 → 04，见本目录 README.md。
--
-- 承载内容：实例级参数取值，含连接数与超级用户预留、预写日志级别、复制槽与发送进程上限、
-- 复制槽的本机事务日志保留上限、预加载扩展、全局锁超时兜底值。
-- 取值来自阶段 2 计划第 3.1 节（承阶段 1 新增决定三与规格附录 A.3），本阶段只执行不改其取值。
--
-- 为什么排在 01 之后：连接数上限的分配以 01 建出的角色为对象。
-- 为什么排在 03 之前：本文件定实例级默认值，03 定角色级覆盖值，两层的层次关系要在脚本
-- 顺序上读得出来，否则后续排查一个取值来自哪一层就得跨文件回溯。
--
-- 可重复执行形态：ALTER SYSTEM 覆盖同值即幂等。多数取值需重启生效，
-- 安装器在本文件执行完毕后触发一次实例重启。

-- 连接预算：应用侧五池常驻合计 42、峰值上限 52；
-- 64 = 52（下限假设）+ 12（超级用户预留与波动），其中超级用户预留固定 3。
alter system set max_connections = 64;
alter system set superuser_reserved_connections = 3;

-- 复制面：两个复制角色合计最多两路流复制，加两路余量取 4；
-- 复制槽取 3（归档槽 + 备用余量）。
alter system set max_wal_senders = 4;
alter system set max_replication_slots = 3;
alter system set wal_level = replica;

-- 复制槽的本机事务日志保留上限：等于附录 A.3 连续归档本机保留子项，不得高于。
-- 按 R-07，该取值做成单一变量；实测回填由归档阶段执行，回填只改本行。
alter system set max_slot_wal_keep_size = '350GB';
alter system set wal_keep_size = 0;

-- 预加载扩展：语句级观测依赖 pg_stat_statements。
alter system set shared_preload_libraries = 'pg_stat_statements';

-- 全局锁超时兜底取 0（不限制），实际等待上限由各连接池的 after_connect 按池覆盖。
alter system set lock_timeout = 0;
