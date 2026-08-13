# ADR-0017 运行期账号在业务 schema 上不授予 DELETE

- 状态：已接受
- 出处：阶段 2 计划 §3.1、§3.2、§12 新增决定五；`db/bootstrap/01_roles.sql`；`xtask sqlcheck` 规则 SQL-001

## 背景

基线第 3.6 节禁止业务数据物理删除，一律走状态机与仅追加口径。此前该禁令只由 CI 静态检查承载：运行期账号 `ep_app_rw` 若在数据库层持有 DELETE，任何绕过应用的写入路径（修数脚本、被入侵的会话、误用的运维语句）仍可物理删行，审计与对账看到的只是行消失。规格第 7.2 章的仅追加约束因此需要一个不依赖 CI 独自承担的数据库侧强制点。

## 决定

`ep_app_rw` 在全部业务 schema 上不授予 DELETE，强制点落在三处：一、`db/bootstrap/01_roles.sql` 的角色权限边界把该账号的写权限限定为 SELECT、INSERT、UPDATE，DELETE 只允许出现在 `platform_msg.idempotency_keys` 与 `platform_ops` 的过期快照表两类对象上；二、各 schema 建 schema 迁移的 `ALTER DEFAULT PRIVILEGES` 只授 `SELECT, INSERT, UPDATE ON TABLES TO ep_app_rw`，默认权限里不含 DELETE，此后属主角色新建的表自动继承该口径；三、`xtask sqlcheck` 规则 SQL-001 对迁移文件文本级断言业务 schema 上不得出现 DELETE 语句，只放行 `platform_msg` 与 `platform_ops` 两个 schema。两处例外的授予动作属各自表的建表阶段，不在本阶段预授。

## 理由

选「数据库权限强制」而不是「继续只靠 CI 静态检查」：不采用本决定的代价是禁令的覆盖面止于「经 CI 进入仓库的 SQL」，运行期账号一旦持有 DELETE，绕过应用的任何路径都不留拦截；权限收回后，误删尝试在数据库层即以权限错误失败，失败本身成为可观测信号。选「两个白名单例外」而不是「全库绝对禁止」：幂等键需要按保留期物理清除，过期快照表需要按窗口清理，两者都不是业务数据；不采用本决定的代价是为这两类对象另设专用账号或经迁移角色执行日常清理，反而扩大高权限账号的使用面。

## 后果

正面：仅追加约束从代码纪律升级为数据库权限，应用路径与绕过应用的路径同权；新增表的默认权限不含 DELETE，口径不依赖建表人记忆；SQL-001 使迁移层的 DELETE 意图在合入前即被拦下。

负面（如实记录）：任何确需物理删除的新场景都必须先论证并修改本决定的白名单，无法在用例里就地补一条 DELETE；误删排查的便利不复存在——这在设计意图之内。若白名单扩容，须同批回写基线第 3.1 节的角色权限边界列与本 ADR。

## 影响范围

- `db/bootstrap/01_roles.sql` 的 `ep_app_rw` 权限边界段；
- 全部 24 个 schema 的建 schema 迁移中的 `ALTER DEFAULT PRIVILEGES` 语句；
- `xtask/src/sqlcheck.rs` 规则 SQL-001 与其负样例；
- 基线第 3.1 节角色权限边界列（按 §12 新增决定五回写）。
