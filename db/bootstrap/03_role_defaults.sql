-- db/bootstrap/03_role_defaults.sql
-- 集群引导第 4 步，共 5 步。执行顺序：00 → 01 → 02 → 03 → 04，见本目录 README.md。
--
-- 承载内容：按角色固化的会话默认值兜底，含语句超时、锁超时、事务内空闲超时、
-- 工作内存与临时文件上限。取值唯一出处是阶段 2 计划第 3.1 节。
--
-- 为什么角色级还要再设一遍：角色级取值是兜底，连接池另在 after_connect 再设一次；
-- 两处一致由阶段 2 的集成测试断言。少了角色级兜底，任何绕过连接池建立的会话就没有超时约束。
--
-- 为什么排在 01 之后：ALTER ROLE ... SET 按角色名寻址，角色不存在则语句直接报错。
-- 为什么排在 04 之前：本文件是数据库侧的收尾，04 之后紧接的是 pg_hba.conf 合入与重载
-- 这一实例侧动作，把实例侧动作留在最后一步便于安装器分段与失败回退。
--
-- 可重复执行形态：ALTER ROLE ... SET 覆盖同值即幂等。

-- 应用读写角色：语句 10s / 锁 3s / 事务内空闲 15s。
alter role ep_app_rw set statement_timeout = '10s';
alter role ep_app_rw set lock_timeout = '3s';
alter role ep_app_rw set idle_in_transaction_session_timeout = '15s';

-- 只读分析角色：语句 60s，工作内存 64MB，临时文件上限 2GB。
alter role ep_analyst_ro set statement_timeout = '60s';
alter role ep_analyst_ro set work_mem = '64MB';
alter role ep_analyst_ro set temp_file_limit = '2GB';

-- 运维只读角色：语句 5s。
alter role ep_ops_ro set statement_timeout = '5s';

-- 迁移角色：语句 30min / 锁 5s，与迁移会话固定的 SET 取值一致。
alter role ep_migrator set statement_timeout = '30min';
alter role ep_migrator set lock_timeout = '5s';
