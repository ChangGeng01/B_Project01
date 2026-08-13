# ADR-0016 `legal_entities` 不建行级安全策略并登记于未受策略表登记表

- 状态：已接受
- 出处：阶段 2 计划 §3.5 表一、§12 偏离项第一条；迁移 `V20260901100200__platform_core_unpoliced_table_registry.sql`

## 背景

行级安全策略的判据列是 `legal_entity_id`，凡带该列的表一律按第 3.6 节模板建策略。但 `platform_core.legal_entities` 是法人注册表本身：它是「法人」这一隔离维度的来源，不存在属于自己的法人归属列；给一张用来枚举法人的表加上以法人标识为判据的策略，会使「枚举法人」这一前置操作在策略之下无法进行——任何会话在取得法人上下文之前读不到法人清单，而法人上下文恰恰要从该表推导。

## 决定

`legal_entities` 不带 `legal_entity_id` 列、不建行级安全策略，并按正向规则在 `platform_core.unpoliced_table_registry` 登记一行：`admission_basis` 取 `ISOLATION_OR_DEPLOYMENT_METADATA`（隔离机制自身的元数据），`isolation_entry` 记录其法人可见性落在法人目录读取契约（`LegalEntityDirectory`）的应用层承接，`matrix_case_id` 取 `rls_matrix_unpoliced_legal_entities` 挂载越权矩阵用例。该登记行随建表阶段的建表迁移插入，缺行即 `db/checks/13` 返回非零行而迁移不通过。本决定同时是基线第 3.8 节「不带法人列的表只有四类」封闭枚举删除后的首个登记实例，按 §12 偏离项第一条回写基线。

## 理由

选「不建策略加逐表登记」而不是「为法人表虚构归属列」：不采用本决定的代价是引入一个语义上不存在的归属关系（法人归属于哪个法人？），且该虚构列会把法人目录这一被全部会话依赖的读取路径变成策略死锁点。选「登记表加机械判据」而不是「按原封闭枚举豁免」：原枚举中的全局配置字典一类无定义、容量无限，已被多个阶段各自归类，事实上不封闭；不采用本决定的代价是豁免口径继续靠主观判断，而 `admission_basis` 两值判据（对全部法人取值相同，或是隔离与部署自身的元数据）可机械核对，`db/checks/13` 把漏登记直接拦在建表路径上。

## 后果

正面：法人枚举这一前置操作在任何会话上下文下都可用；豁免不再是隐式约定，每张未受策略表都有一行可审的登记与其越权矩阵用例标识；新出现的无归属列的表只有登记一条出路。

负面（如实记录）：该表可被任一运行期会话读到全部法人的标识与名称（首版 2 个法人），法人可见性的裁剪由租户与身份阶段（阶段 4）在应用层按用户授权法人集合执行，数据库层不提供该裁剪；若阶段 4 的应用层裁剪缺位，本表的全量可读即成为既成暴露面，该边界已写入越权矩阵用例。

## 影响范围

- `db/migrations/platform_core/V20260901091000__platform_core_legal_entities.sql`（不建策略的建表方）；
- `db/migrations/platform_core/V20260901100200__platform_core_unpoliced_table_registry.sql`（登记行）；
- `db/checks/13_unpoliced_registry.sql` 与 `tests/rls_matrix` 的 `rls_matrix_unpoliced_legal_entities` 用例；
- 基线第 3.8 节（封闭枚举删除，正向规则回写）。
