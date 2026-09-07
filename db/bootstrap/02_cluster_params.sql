-- db/bootstrap/02_cluster_params.sql
-- 集群引导第 3 步，共 5 步。执行顺序：00 → 01 → 02 → 03 → 04，见本目录 README.md。
--
-- 安全边界：本文件只用于开发、测试和人工验证，不是 G6 Windows 生产安装器输入，生产环境
-- 禁止执行。G6 必须从签名配置投影生成精确 postgresql.conf，并要求 postgresql.auto.conf
-- 不存在或为空且无任何有效覆盖；不得用本文件的 ALTER SYSTEM 作为生产替代路径。
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
-- 非生产引导器在本文件执行完毕后触发一次实例重启。

-- 当前 P340 实现种子：应用侧四池常驻合计 37、临时 10、安全储备 5、峰值 52；
-- 服务端总上限固定 64，其中 migration 预留 4、recovery 超级用户预留 3，普通连接容量
-- 因而为 57，应用峰值 52 后还剩 5 个普通槽（64-4-3-52=5）。这 5 个服务端残余槽
-- 与应用峰值 52 内部的安全储备 5 是两个不同预算，不得合并或重复分配。F57 签名 provider
-- graph 的另一个两槽 margin 只由其完整 consumer exact-set 实测证明，不是本种子算出的残余。
-- 各权限级消费者与峰值预算由签名配置代逐项读回，不得把预留连接发给应用池。
-- ADR-0019 已明确这些数不是不可变产品真值；生产准入仍须把签名配置代的数据库消费者
-- exact set 与同硬件实测容量证书逐项读回，未知/重复消费者或证书超限均拒绝启动。
alter system set max_connections = 64;
alter system set reserved_connections = 4;
alter system set superuser_reserved_connections = 3;

-- 复制面：两个复制角色合计最多两路流复制，加两路余量取 4；
-- 复制槽取 3（归档槽 + 备用余量）。
alter system set max_wal_senders = 4;
alter system set max_replication_slots = 3;
alter system set wal_level = replica;

-- Windows Server 2022 / PostgreSQL 16 首版冻结值。平台不接受启用写缓存时不安全的
-- open_datasync，也不根据机器现场结果自动选择另一种方法。pg_test_fsync、驱动栈、缓存策略、
-- UPS、flush 与断电试验只形成生产资格证据；任一不通过就拒绝认证，不得静默改值。
alter system set wal_sync_method = 'fsync_writethrough';

-- 首版不依赖 posix_fadvise 平台能力，也不引入“锁定内存中的页”权限。
alter system set effective_io_concurrency = 0;
alter system set huge_pages = off;

-- 复制槽的本机事务日志保留上限。PostgreSQL 的 GB 单位是 2^30 bytes，因此本行恰为
-- 350 GiB。它只约束 replication-slot-retained live WAL；`archive.wal_spool_max_gb=350`
-- 只约束 archive staging，persistent-archiver-failure extra live WAL 另有独立 350 GiB
-- 故障桶。三项不得互借、合并或按“不会同时发生”抵销。
--
-- F-86 容量收束的现行权威已回写总体设计 §13.5.1；00c F-86 只保留历史裁定记录。
-- 任意回填或修改都必须在同一原子变更中同步 archive 配置、签名 P340 capacity profile、
-- live-WAL 300/650 GiB hold 与 350/700 GiB hard 阈值、<=30 s 采样、<50 GiB 反应余量
-- 以及 equality/one-byte-below/双故障/restart-held 测试；禁止只改本行。本行不因当前
-- 1 TB development disk 容量不足而偷降安全上限。
alter system set max_slot_wal_keep_size = '350GB';
alter system set wal_keep_size = 0;

-- 预加载扩展：语句级观测依赖 pg_stat_statements。
alter system set shared_preload_libraries = 'pg_stat_statements';

-- 全局锁超时兜底取 0（不限制），实际等待上限由各连接池的 after_connect 按池覆盖。
alter system set lock_timeout = 0;
