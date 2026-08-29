# ADR-0026 五组领域不变量属性测试的 F-57 承接

- 状态：已接受
- 出处：F-67 终扫对 ADR-0006 的逐字登记；[权威和取代登记](../superpowers/reviews/2026-08-23-f57-authority-supersession-register.md):92 逐字「`docs/adr/*` | `CURRENT_SUBJECT` | 与 F-57 不冲突且状态为接受的窄技术决定 | 反向覆盖 F-57；冲突 ADR 必须另立取代 ADR」
- 取代范围：只取代 [ADR-0006](ADR-0006-domain-invariant-property-tests.md) 第 17—27 行的整张挂载表（含「承接阶段」与「挂载点 crate」两列）及与该表绑定的两处阶段指称（:15 的「阶段 1 交付」一句、:27 的「阶段 1 不为任何一组预留」一句）；ADR-0006 的五组强制不变量义务本身、背景与理由两段的论证继续有效，其 :39 登记的「若后续裁定改变了某组不变量的归属阶段，本 ADR 须同批修订」在本篇落地后视为已履行

## 背景

ADR-0006 :19—25 用一张三列表把五组强制不变量指派给「阶段 9a／阶段 8／阶段 10」并各自钉在一个 `ep-domain-*` crate 上。该表的两列今天都已失效，且失效原因互不相同：

第一，「承接阶段」列指向的旧十四阶段已被 F-57 整体取代，不会再有任何一个阶段「退出」。ADR-0006 :39 自己写明「本表是按当前阶段划分写的」，并把「若后续裁定改变了某组不变量的归属阶段，本 ADR 须同批修订」登记为已知维护成本。该条件已经成立。

第二，「挂载点 crate」列指向的四个 `ep-domain-*` crate 与 [ADR-0025](ADR-0025-f57-capability-graph-and-feature-first-boundaries.md) 冲突。ADR-0025 :5 窄取代了「每个业务域默认拆成 contract/domain/application 三个顶层 crate」的物理结构，:34 逐字「禁止新增新的三 crate 空套件」，:36 逐字「现有 layer-first crate 可暂作兼容 facade，但不得新增业务规则」。[F-57 收敛实施主计划](../superpowers/plans/2026-08-24-f57-converged-program.md):74 同义复述为 existing layer-first business crates become read-only compatibility facades。把五组不变量的属性测试与其被测领域规则写进 `ep-domain-ledger`、`ep-domain-inventory`、`ep-domain-finance`、`ep-domain-costing`，正是往 facade 里新增业务规则。

这四个 crate 今天仍在树上且包名与 ADR-0006 逐字相同（`crates/domain/{ledger,inventory,finance,costing}/Cargo.toml` 的 `name = "ep-domain-*"`），每个 `src/lib.rs` 只有 6 行骨架注释；`crates/features/` 尚不存在。因此这不是一处「已经作废的历史文字」，而是一份仍能被照着执行、执行了就会违反 ADR-0025 的现行指令。

义务本身没有问题。五组不变量的来源是 2026-07-19 总体设计 §17.2「领域属性测试」（:1342—1345）在测试类型清单中的独立一项，其五组内容由 ADR-0006 :9 逐字固定为借贷平衡、库存守恒、核销守恒、移动加权平均单价重算、价差拆分。属性测试作为测试类型在现行权威中仍然在册：收敛主计划 :11461 的 `L0_DEVELOPER` 行逐字含 touched unit/property tests。要修的只有承接。

## 决定

### 一、五组义务原样保留

五组强制不变量继续是必交付项，一组不减、一组不合并、一组不降级为示例。ADR-0006 :9 的五组名称是本篇的唯一枚举源，本篇不重述、不改写、不新增第六组。

### 二、撤销 ADR-0006 的整张挂载表

ADR-0006 :19—25 的表自本篇起不再是任何交付的依据。任何计划、测试清单或提交说明都不得再引用「阶段 8／9a／10」作为某组不变量的承接，也不得再把 `ep-domain-*` 当作其挂载点。ADR-0006 :15 的「阶段 1 交付 `proptest` 的策略工具与……三条属性」与 :27 的「阶段 1 不为任何一组预留返回成功的空测试」两句，其中的阶段指称同时失效；两句表达的规则本身——先有工具再有业务组、不预留恒真的空测试与占位文件——由本篇第五条重新承接。

### 三、承接改挂 F-57 的 requirement→owner-task 映射

新的承接以三份现行权威逐层解析，不再经过任何阶段号：

- 需求行与 CapabilityOwner 取 [F-57 需求追踪矩阵](../superpowers/reviews/2026-08-23-f57-requirements-traceability.md)（§1.1 权威顺序第 4 位）；
- `owner_task`／`activation_task`／TestID 取 [`docs/f57-task-ownership.seed.tsv`](../f57-task-ownership.seed.tsv)（§1.1 权威顺序第 15 位），该 seed 的表头逐字为 `requirement_id owner_task activation_task test_id test_target_path test_symbol evidence_id evidence_schema platform_lane`；
- 交付波次取收敛主计划 :11247 的 delivery-profile 行，其逐字内容为 `| F57-13, F57-14, F57-15, F57-17, F57-18, F57-20, F57-21, F57-22, F57-23 | G5_INTEGRATION |`。

| 不变量组 | F-57 承接需求行 | CapabilityOwner | owner_task / activation_task | TestID |
|---|---|---|---|---|
| 会计借贷平衡 | `FIN-011`（矩阵 :190） | `ledger` | `F57-20` / `F57-20`（seed :64） | `T-F57-FIN-011` |
| 库存数量与金额守恒 | `INV-002`（矩阵 :169） | `inventory` | `F57-20` / `F57-20`（seed :48） | `T-F57-INV-002` |
| 应收应付核销守恒 | `FIN-005`（矩阵 :184） | `finance` | `F57-20` / `F57-20`（seed :58） | `T-F57-FIN-005` |
| 移动加权平均单价重算 | 矩阵中无逐字对应行；最近承接为 `INV-002`（矩阵 :169）与 `INV-007`（矩阵 :174） | `inventory` / `costing` | `F57-20` / `F57-20`（seed :48、:53） | `T-F57-INV-002`、`T-F57-INV-007` |
| 价差拆分 | 矩阵中无逐字对应行；最近承接为 `INV-007`（矩阵 :174） | `costing` | `F57-20` / `F57-20`（seed :53） | `T-F57-INV-007` |

**以下归类是本篇的判断，不是任何文件的逐字结论**（F-68 标注：这些 RequirementID 在矩阵与 seed 中各自独立成行，「从属于某组」是读验收列文字后的归并；若实现方认为每行证据须独立成立，按独立处理，本段不构成阻碍）。同 owner 的相邻证据行不另立不变量组，只在负例侧被同一组属性测试消费：`FIN-013`（矩阵 :192，seed :66）承接经营期间永久锁定与迟到顺延，属借贷平衡组的期间维度；`INV-004`（矩阵 :171，seed :50）与 `INV-005`（矩阵 :172，seed :51）承接五类不可变库存事件与禁止负库存，属库存守恒组的负例；`FIN-007`（矩阵 :186，seed :60）与 `FIN-008`（矩阵 :187，seed :61）承接冲正与应收应付账龄，属核销守恒组的释放侧。`SRV-006`（矩阵 :203，seed :72）虽在验收列逐字含「库存守恒」，但其 `owner_task`／`activation_task` 均为 `F57-21` 且 CapabilityOwner 为 `service`，是库存事实的消费方而非承载方；**「不得被当作库存守恒组的承接」这句禁令是本篇新写的，不是引用**（F-68 标注），其依据是矩阵逐字「库存事实由库存拥有……服务不得直写他域事实」。

### 四、后两组的挂载点当前为 UNRESOLVED，且必须如实登记

移动加权平均单价重算与价差拆分两组，在矩阵正文中没有逐字命中行。这两组的现行效力来自矩阵 §15 的精确来源绑定：`INV-002` 的 `SourceClause[]` 为 `[PRD §5「库存与存货计价」]`（矩阵 :418），`INV-007` 为同一章（矩阵 :423）；而首版 PRD（§1.1 权威顺序第 25 位）:1910 逐字「存货计价采用移动加权平均一种方法与单一成本层」，:1915 逐字「退货回冲的取价三分支与回冲后移动加权平均单价的重算」，:2113 逐字「价差拆分规则中判定为尚有库存的部分会调整存货金额并重算移动加权平均单价」。这条链符合 F-57 总体设计 §1.1 对未列文件的收编要求（以具名文件与具名章节精确绑定），因此两组义务成立。

但两组的**物理挂载点在现行权威中取不到值**，本篇不替使用方选一个：

- 收敛主计划 §2.1（:76—97）的 `FeatureOwnerIdV1` 是一份 exact 17 行闭集。其中没有 `costing` 行——`costing` 一词在该文件全文零命中；
- 该 17 行中也没有任何一行声明库存金额账／成本层／价差事实：`inventory-fulfilment`（:86）的 authoritative scope 逐字只到 `receipt/delivery/return and quantity/batch/serial facts`，`operating-ledger`（:87）只到 `internal operating entries, mappings, trial balance, and operating periods`，`purchase-invoicing`（:93）只到 `purchase-invoice facts and reversals`；
- 而矩阵 :174 把 `INV-007` 的唯一 CapabilityOwner 定为 `costing`。

因此这两组的挂载点状态固定为 `UNRESOLVED`，直到使用方以一次显式裁定关闭下列二选一：给 `FeatureOwnerIdV1` 增加一个承载库存金额／计价／价差事实的行，或把该事实范围逐字并入某个既有 feature owner 的 authoritative scope。在裁定落地之前：

- 不得把这两组挂到 `inventory-fulfilment`——那会越过 :86 逐字划定的 scope，而 §2.1 逐字要求「Sharing a physical schema never permits sharing a fact or repository writer」；
- 不得回落到 `ep-domain-costing`／`ep-domain-inventory`——那违反 ADR-0025 :34、:36；
- 任何声称这两组已交付的证据一律判为不合格，理由是挂载点未解析而非测试未通过。

前三组不受此限，其挂载点按第五条解析即可确定。

### 五、挂载点解析规则与转绿判据

挂载点不再由本篇写死一个 crate 名，改为一条可机械求值的解析式：某组不变量的属性测试，落在其 CapabilityOwner 对应 `FeatureOwnerIdV1` 行的 `crate_root` 内部（ADR-0025 :17 的 `public`/`domain`/`application`/测试模块结构中的测试模块），不得落在该行之外。据 §2.1 现表，前三组解析为：

| 不变量组 | 解析后的 `feature_owner_id` | `crate_root` |
|---|---|---|
| 会计借贷平衡 | `operating-ledger` | `crates/features/operating-ledger`（:87） |
| 库存数量与金额守恒 | `inventory-fulfilment` | `crates/features/inventory-fulfilment`（:86） |
| 应收应付核销守恒 | 客户侧 `receivable-cash`、供应商侧 `payable-cash` 各一组，不合并 | `crates/features/receivable-cash`（:94）、`crates/features/payable-cash`（:88） |

核销守恒**在挂载点上落到两处**而不是一处，是因为矩阵 :184 的 CapabilityOwner `finance` 在 §2.1 中被 :88 与 :94 两行分别拥有，且两行共用 `finance` 物理 schema 的事实不构成共享写者。**这是挂载点的一分为二，不是把不变量拆成两组**——第一条「一组不减、一组不合并、不新增第六组」仍然成立，五组枚举以 ADR-0006 为唯一源。**该拆分的依据是本篇对 §2.1 的解析，没有任何文件逐字写过它**（F-68 标注）。

转绿判据固定为下列四条，每条都必须能在不通过时当场失败：

1. **表内取值一致**：第三条表中五行的 `owner_task`、`activation_task`、`TestID` 与 `docs/f57-task-ownership.seed.tsv` 对应 `requirement_id` 行的同名字段逐字节相等；任一不等即失败。该 seed 是 §1.1 第 15 位权威且被 registry snapshot 以 `TASK_OWNERSHIP_SEED` 摘要保护，取值可取得、可比对。
2. **挂载点在范围内**：每组属性测试文件的路径前缀等于第五条解析出的 `crate_root`；落在其外、落在 `crates/domain/*`、或落在 `crates/features/` 之外即失败。
3. **不接受空测试与占位**：任何一组都不得以恒真断言、返回成功的空测试或占位文件充数（承接自 ADR-0006 :27 的原规则）；一组的属性测试若在其 `crate_root` 内不存在，判为该组未交付，而不是判为通过。
4. **UNRESOLVED 不得被静默跳过**：第四条列为 `UNRESOLVED` 的两组，其状态必须显式为未解析并阻断相关证据聚合；「未覆盖」不得计为「通过」。

本篇不规定属性测试框架的选型细节，只登记一项现行事实供后续批次消化：`proptest` 当前不在根 `Cargo.toml` 的 workspace 依赖中，`crates/` 下也无任何文件引用它（两处 grep 均零命中）。ADR-0006 :15 承诺的策略工具与三条框架自测属性（`to_money` 幂等、Money 加法结合律与交换律、UUIDv7 单调性）因此尚未落地。该缺口属实现进度，不属文档冲突，本篇只登记不裁定。

## 理由

把承接钉在 requirement→owner-task 映射而不是另选一组新阶段号，是因为 seed 是 §1.1 第 15 位的具名权威、有固定表头、有 185 行一一对应、并被 registry snapshot 摘要保护；阶段号在 F-57 之后没有任何一份现行文件为其背书。代价是本 ADR 的表从此依赖 seed：seed 若改行，本表须同批改，且第五条判据 1 会在不同步时立即失败——这正是把维护成本换成可当场报错的判据。

把挂载点写成解析式而不是写死 crate 名，是因为 ADR-0006 的老毛病恰恰是写死了一列名字，而名字所依附的物理结构被 ADR-0025 换掉了。代价是解析式依赖 §2.1 的 17 行 registry，registry 增删行会改变解析结果；换来的好处是 registry 改动会自动传导，不需要再立一篇取代 ADR。

把后两组诚实标成 `UNRESOLVED` 而不是就近挂到 `inventory-fulfilment`，代价是这两组在使用方裁定前无法开工，会拖住价差与计价相关的证据聚合。不这么做的代价更大：把库存金额与价差事实塞进一个 authoritative scope 逐字不含它们的 feature owner，会制造一处「错了不会当场报错」的越界——scope 检查会通过，因为没人声明过这个事实归谁，而这正是本仓判据要拦的第三类缺陷。

核销守恒拆成两组，代价是要写两套策略与两套断言；不拆的代价是让一组测试跨越 `receivable-cash` 与 `payable-cash` 两个事实 owner，把两个 owner 的守恒式混成一个，日后任一侧改动都会同时改到另一侧的判据。

## 后果

正面：五组义务从「挂在一张永不到期的阶段表上」变成挂在有 owner、有 TestID、有交付波次的现行映射上；两组取不到挂载点的情况被显式登记为 `UNRESOLVED` 而不是被就近安置；判据 1、2 都能在不成立时当场失败，不依赖人工比对。

代价与必须同批完成的登记动作：

1. ADR-0006 的状态行须改为「阶段挂载表已被 ADR-0026 取代；五组不变量义务仍接受」，其 :3 的 F-67 注中「该动作待 G0 批次，此处先登记」一句在本篇落地后失效，须同批删去或改为指向本篇；
2. `docs/adr/README.md` 现有记录表须新增本篇一行，并把 ADR-0006 行的状态列同步为上一条的措辞；该文件 :54 的叙述段亦须补一句说明本篇的窄取代范围；
3. 本篇不在 F-57 总体设计 §1.1 的恰 25 份具名清单内，与 ADR-0006 一样只经登记 :92 的 `docs/adr/*` 行取得 `CURRENT_SUBJECT` 效力；它不能反向覆盖 §1.1 中任何一份文件。若使用方希望本篇进入 25 份清单，须另行修订 F-57 总体设计 §1.1，属使用方裁定，本篇不代行；
4. 第四条的 `UNRESOLVED` 是一项待使用方裁定的未决项，须登记在案并在裁定落地后回改本篇第四、五条，而不是由实现者在代码里就近选一个 crate。

## 影响范围

- [ADR-0006](ADR-0006-domain-invariant-property-tests.md) :3、:15、:17—27 与 `docs/adr/README.md` 的现有记录表；
- [F-57 需求追踪矩阵](../superpowers/reviews/2026-08-23-f57-requirements-traceability.md) 的 `FIN-005`、`FIN-007`、`FIN-008`、`FIN-011`、`FIN-013`、`INV-002`、`INV-004`、`INV-005`、`INV-007` 九行及其 §15 精确来源；
- [`docs/f57-task-ownership.seed.tsv`](../f57-task-ownership.seed.tsv) 中上述九个 `requirement_id` 行的 `owner_task`/`activation_task`/`test_id` 字段；
- [F-57 收敛实施主计划](../superpowers/plans/2026-08-24-f57-converged-program.md) §2.1 的 `FeatureOwnerIdV1` 17 行 registry 与 `docs/f57-feature-owner-registry.v1.tsv` 的生成；
- [ADR-0025](ADR-0025-f57-capability-graph-and-feature-first-boundaries.md) 第七条的 touched-feature 渐进迁移与 facade 删除证据；
- `crates/domain/{ledger,inventory,finance,costing}` 四个 facade crate 的边界，与 `crates/features/{operating-ledger,inventory-fulfilment,receivable-cash,payable-cash}` 的测试模块；
- `cargo xtask archcheck` 的层位判定与 L0 lane 的 touched unit/property tests 选择。
