# F-57 文档权威与取代登记

> 日期：2026-08-23（Australia/Melbourne）；审计更新：2026-08-26
> 状态：`CURRENT`；2026-08-24 收敛修订、ADR-0025 与五文件实施计划集已获用户批准
> 目的：让开发、测试和评审人员能够唯一判断“哪句话仍可执行”，而不需要按文件日期猜测

## 1. 状态定义

| 状态 | 含义 |
|---|---|
| `CURRENT` | 现行权威，可以直接作为对应主题的实现依据 |
| `CURRENT_SUBJECT` | 仅在明确主题内保持现行权威 |
| `CURRENT_PLAN_NOT_AUTHORIZED` | 现行可执行计划；内容已就绪，但尚未获得开始产品开发的授权 |
| `CURRENT_SUBJECT_IMPORT` | G0 前的受控导入输入；仍可审阅但不可手工扩展，G0 逐字节往返通过后转为 `HISTORICAL_IMPORT_SNAPSHOT` |
| `HISTORICAL_IMPORT_SNAPSHOT` | G0 已验收的不可变导入前像；仅供来源审计，不再是现行投影目标，后续 graph 演进不回写也不与其永久求等 |
| `GENERATED_PROJECTION` | 已由 CapabilityGraph 与登记输入确定性生成、绑定 graph digest/generator version 的现行机器投影；只能经 generator 更新 |
| `G0_PLANNED_REGISTRY` | 文件当前必须缺席，但路径、owner task 与 exact contract 已由现行计划冻结；只有获开发授权后的指定 G0 task 可创建 |
| `G0_GENERATED_AUTHORITY` | 当前必须缺席的 G0 生成权威；创建后只能由登记源确定性再生并由 gate 绑定 digest，禁止手工双写 |
| `HISTORICAL_DETAIL_INPUT` | 旧实施计划中的字段、测试和推理可以被现行计划引用，但原任务、顺序、迁移和门禁永久不可执行 |
| `CURRENT_SUBJECT_INPUT` | 仅提供被 F-57 明确保留的主题不变量与裁定；不能独立发出实施指令，也不能覆盖 F-57 |
| `CURRENT_SUMMARY_NON_NORMATIVE` | 现行易读摘要；只帮助理解，不定义范围、接口、状态或验收，任何冲突均以权威规范为准 |
| `PARTIALLY_SUPERSEDED` | 仍有可用细节，但冲突部分已被更高文档取代 |
| `REGISTRY_PENDING_REBASELINE` | 登记表结构可参考；新增开发前必须按 F-57 再基线 |
| `HISTORICAL` | 只用于决策追溯，不是实现入口 |
| `DEFERRED` | 已有设计或计划，但当前产品阶段明确不执行 |
| `SUPERSEDED_DO_NOT_EXECUTE` | 整份文件已被 F-57 替代，只供历史追溯；**不得作为任何实现入口** |
| `HISTORICAL_DO_NOT_EXECUTE` | 同 `HISTORICAL`，另显式禁止据以施工 |

> **本表原缺后两码。** 实测该两码挂在全仓 **25 个文件**的横幅上，而本表（唯一状态词表）内命中 0，导致同一份文件可同时持有横幅状态与本表状态两个不同取值（例如 `2026-08-21-f50-financial-consistency-implementation.md` 横幅为 `SUPERSEDED_DO_NOT_EXECUTE`、本表 `2026-08-21-f50-financial-consistency-implementation.md` 行给 `HISTORICAL`（原引第 131 行，因本次插行已漂移，改按文件名引用，F-62））。现补入定义；两者冲突时以本表为准。

本登记的文件状态是文件分类，不是项目开发状态。项目状态入口统一使用 `development_state=READY_NOT_AUTHORIZED`、`blocking_reason=DEVELOPMENT_AUTHORIZATION_REQUIRED`、`implementation_state=NOT_IMPLEMENTED` 和 `production_state=PRODUCTION_NOT_READY`；`CURRENT_PLAN_NOT_AUTHORIZED` 仅表示计划文件的分类，绝不构成开发授权。

逐文件冲突的唯一 precedence 由 F-57 总体设计 §1.1 持有；本登记只记录文件范围、取代关系和阅读/执行入口，不另设或重排权威顺序。历史文件中出现“当前”“冻结”“可直接开发”等原句，不改变本登记给出的分类。旧文档中的 `Task 1…25` 一律只解释为 `F57-01…F57-25` 需求所有权桶；实际执行节点、顺序、文件、迁移和门禁只由 2026-08-24 五文件计划集决定。

## 2. 文件级登记

| 文件 | 状态 | 现行范围 | 禁止用法 |
|---|---|---|---|
| `README.md` | `CURRENT` | 阅读导航及开发/生产状态入口；逐文件 precedence 仅链接至总体设计 §1.1 | 把导航编号当权威排序，或用旧 F-55/F-56/十四阶段入口覆盖 F-57 |
| `docs/介绍/管理软件基本需求.docx` | `CURRENT_SUBJECT` | 客户原始业务需求来源；人员分类只作人物模板 | 作为技术架构、固定岗位授权或范围优先级裁定 |
| `docs/介绍/企业一体化经营管理平台-产品介绍与功能大纲.docx` | `CURRENT_SUMMARY_NON_NORMATIVE` | 面向非技术读者的产品定位、业务术语和能力概览 | 作为规范、范围、接口、实现状态、性能/恢复承诺或验收证据；用介绍中的说法覆盖 F-57 |
| `2026-08-23-f57-governed-automation-fabric-design.md` | `CURRENT` | 产品、架构、权限、自动化、能力包、双端、存储、硬件、安全、商业与 2026-08-24 收敛修订 | 把文档批准解释为产品开发已授权或旧计划仍可执行 |
| `2026-08-23-f57-business-execution-contract.md` | `CURRENT_SUBJECT` | CRM、CPQ、合同、订单、采购、库存、财务、服务、项目、门户身份和业务闭环；新增 CTC-01 与 STANDARD/DROP_SHIP XOR | 扩大已批准范围，或覆盖总体信任、安全和阶段边界 |
| `2026-08-23-f57-client-lifecycle-security-contract.md` | `CURRENT_SUBJECT` | 员工 C/S API、渐进四端认证、终端 DLP、驻留/本地化、保留/支持/事件、便携导出和生产运营 exact 契约 | 把低档 Windows 切片说成四端、备份、UPS 或后继 LTSC 已实现/已认证 |
| `2026-08-23-f57-requirements-traceability.md` | `CURRENT` | 原始 DOCX、PRD 与 F-57 的 **185 项闭集（174 个主需求 + 11 个阶段边界）** 的功能追踪 | 不代替详细领域规则 |
| `docs/f57-task-ownership.seed.tsv` | `CURRENT_SUBJECT` | 185 个 RequirementID 到 owner task、activation task、稳定 TestID、测试目标/符号、EvidenceID、schema 和平台 lane 的逐项设计时绑定 | 把设计时绑定当作测试已存在、已运行或证据已通过；脱离追踪矩阵和实施计划修改 |
| `docs/f57-requirement-delivery-profile-overrides.v1.tsv` | `CURRENT_SUBJECT` | 57 个因旧所有权桶过宽而必须逐项改写 first-due 或登记早期切片探针的 Requirement；与主计划 §4 基础映射 exact-join 后生成 185 行交付视图 | 把 probe 当整项通过；仅凭旧 F57 桶推导交付时间；由实现者临场移动到期档位 |
| `docs/f57-migration-baseline.v1.tsv` | `CURRENT_SUBJECT` | pre-F57 78 行精确分区（66 immutable、3 rewrite、7 superseded absent、2 deferred absent），SHA-256=`52930d7ae32ee02ddda38199bcc144f5f6747fcfbe33e740741a0f21604ca8fd`；与 310-row legacy seed exact-join 为 388，G0 后可执行 baseline 恰为 69 | 改写登记、把 absent 行落盘、跳过三个 preimage/target/postimage 校验、或把文件本身当作 SQL 已修订/已应用 |
| `docs/f57-legacy-migration-disposition.seed.tsv` | `CURRENT_SUBJECT` | 310 个缺失旧 `PLANNED` 版本到 `SUPERSEDED_BY_F57_REBASELINE`、唯一聚合 owner task、现行 47-row reservation 中 42 个替代路径和映射规则的逐行绑定；2026-08-24 收敛重绑后 SHA-256=`06566ca354b6279391e5ec3a0152316a8eb38d1f10cb09dc23953370883c3196` | 把处置种子当作迁移已经执行/重分类；跳过 G0 exact-join；由开发者临场重分配或恢复旧 34-path replacement 集 |
| `docs/f57-feature-owner-registry.v1.tsv` | `G0_PLANNED_REGISTRY` | G0-01 只可按主计划 §2.1 创建 17-row exact `FeatureOwnerIdV1`/crate/schema/repository/fact-owner 登记 | 文件缺失时宣称已生成；从 schema 猜 owner；允许别名或双 writer |
| `docs/f57-platform-mechanism-registry.v1.tsv` | `G0_PLANNED_REGISTRY` | G0-01 只可按主计划 §2.2 创建 35-row exact `PlatformMechanismIdV1`/crate/mechanism-scope 登记；planned root 也须先登记 | 文件缺失时宣称已生成；从 crate 猜 owner；平台机制侵占 feature business fact 或使用未登记 owner |
| `docs/f57-task-staged-paths.v1.tsv` | `G0_PLANNED_REGISTRY` | G0-01 从五份现行计划精确展开每个 task 的可暂存路径、路径类型和技术分支条件 | 在文件缺失时绕过 snapshot staging；使用 raw `git add`、目录/glob、未登记或预先 dirty 路径 |
| `docs/f57-migration-reservations.v2.tsv` | `G0_PLANNED_REGISTRY` | G0-01 按主计划 §4.1 创建 exact 9-column/47-row F57 suffix；每行冻结 gate/owner/origin/映射 digest，42 legacy replacement + 5 net-new，仅 status 可由 owner task 原子单向转 `CREATED` | 把 reservation 当已创建 SQL；插入历史版本、跳号、重复 owner、修改非 status 字段或脱离 task commit 改状态 |
| `docs/f57-delivery-dag.v1.tsv` | `G0_PLANNED_REGISTRY` | G0-01 按主计划 §5 创建不可变 42-row topology DAG；只含 direct dependency/product/migration/condition，不含交付状态；状态仅由签名回执派生 | 后续 task 改 TSV 冒充推进、增加状态列、按旧 Task 数字推断执行顺序，或跳过 aggregate receipt |
| `docs/capability-graph/f57-core.v1.json` | `G0_PLANNED_REGISTRY` | G0 导入验收后成为唯一 CapabilityGraph authoring source；所有 Rust/OpenAPI/TS/UI/测试投影共享 digest | 文件缺失时宣称 graph 已存在；另建第二 schema/owner/UI 真值 |
| `docs/generated/f57/requirement-test-facades.v1.json` | `G0_GENERATED_AUTHORITY` | G0 确定性生成 22 canonical target、185 exact symbol 与 owner binding manifest | 手工创建 facade、umbrella pass、skip/ignore、或把文件存在当测试已执行 |
| `testkit/src/f57_cases/generated_bindings.rs` | `G0_GENERATED_AUTHORITY` | G0 生成 Rust handler 的 exact delivered binding；只引用已交付 concrete handler | 手工注册、catch-all、未交付 handler 返回 PASS 或绕过 language-local registry |
| `docs/generated/f57/test-manifest.json` | `G0_GENERATED_AUTHORITY` | G0 生成 TestID/target/symbol/language/handler exact join | 由 task prose 或 target-wide 运行替代 exact symbol ownership |
| `docs/generated/f57/projection-manifest.v1.json` | `G0_GENERATED_AUTHORITY` | G0 生成精确 30-family 非自引根清单（恰四个 multi-member，含 P340 policy 与 semantic contracts）；每个 primary/member 含 exact path/media/owner/digest，根绑定 graph digest 与完整 generator identity | 手工编辑投影、漏 regenerate、让 manifest 自哈希、隐藏动态成员或用不同 graph/generator 拼 gate |
| `docs/generated/f57/client-conformance-manifest.v1.json` | `G0_GENERATED_AUTHORITY` | CapabilityGraph 生成三项栈中立 conformance ID、Tauri/Flutter 六个 closed recipe、delivery state 与 exact source path；G5/G6 只运行签名选择栈 | 在 manifest 中放任意 shell、继续用被拒 Tauri fixture、缺失 Flutter G3/G4 carrier 或跨候选复用结果 |
| `docs/generated/f57/rust/manifest.v1.json` | `G0_GENERATED_AUTHORITY` | CapabilityGraph 生成所有 feature/platform owner Rust DTO member 的 owner/path/digest exact set；成员与 manifest 同步生成 | 手写 DTO、全局 DTO crate、未登记 member、客户端另造协议类型或漏 no-diff |
| `docs/generated/f57/migration-apply-manifest.v1.json` | `G0_GENERATED_AUTHORITY` | G0-06 在三草案 postimage 通过后一次性生成 canonical 69-file baseline apply set；自身不自签，由 clean-candidate receipt 签名绑定其 digest；F57 suffix 只从独立 reservation manifest 的连续 `CREATED` prefix 派生 | 把 manifest 当迁移已运行；要求其自签造成生成循环；向其中追加 F57 row；允许 pre/post 混态、任意 SQL 扫描或缺席路径进入 apply set |
| `docs/f57-fresh-pg-check-registry.v1.tsv` | `G0_GENERATED_AUTHORITY` | G0-01 按主计划冻结的 header、27 行和 SHA-256 `76fed80f…01a` 创建现行 Fresh-PG 检查闭集；适用项由 profile ordinal 与 activation through-version 双条件唯一选择，G0-06 只分派已登记的编译 handler | 从 profile 猜全量任务、执行未交付同档任务、运行登记文字/任意 argv、修改行而不重基线，或读取旧 seed 作为当前权威 |
| `docs/f57-api-discriminators.seed.tsv` | `CURRENT_SUBJECT_IMPORT` | 437 行 Control/Employee/Portal command/query 的 surface、owner task、introduced version、payload/result/error `$ref`、CAS mode 与 audience 唯一机器闭集；G0 无损导入后状态转为 `HISTORICAL_IMPORT_SNAPSHOT` | 脱离 G0 修改；用 inline 摘要、OpenAPI 单边或实现者临场命名覆盖；把登记当作 API 已实现；G0 后继续当 live projection 回写 |
| `docs/f57-api-component-shapes.seed.tsv` | `CURRENT_SUBJECT_IMPORT` | 638 行判别字组件形状、profile、参数、显式字段与 owner 的唯一机器闭集；规范化 Rust 路径由 G0 确定性投影产生，不是 seed 列；G0 无损导入后状态转为 `HISTORICAL_IMPORT_SNAPSHOT` | 由 OpenAPI、Rust 或客户端单边新增/改名字段；把组件登记当作 schema 已实现；伪造 seed 中不存在的 Rust 路径列；G0 后继续当 live projection 回写 |
| `docs/f57-api-component-state-domains.seed.tsv` | `CURRENT_SUBJECT_IMPORT` | 218 行 state/state-filter/nested-item 组件到唯一状态域和 owner 的机器绑定；G0 无损导入后状态转为 `HISTORICAL_IMPORT_SNAPSHOT` | 按 schema 名猜状态域；遗漏内嵌页 item；用通用 `STATE_CODE` 代替有限枚举；G0 后继续当 live projection 回写 |
| `docs/f57-api-state-domains.seed.tsv` | `CURRENT_SUBJECT_IMPORT` | 65 个 wire 状态域及其有限值闭集；语义 exact-join F-57 业务契约 §14.6 和其余现行领域图/派生规则；语义 `UNKNOWN` 仅存在于 `EFFECT_V1\|PAYMENT_V1\|REFUND_V1`；G0 无损导入后状态转为 `HISTORICAL_IMPORT_SNAPSHOT` | 在其他域自造 `UNKNOWN`、自造 `OTHER\|CUSTOM`、与领域状态分叉，或把展示枚举当作可直接写入的状态机；G0 后继续当 live projection 回写 |
| `docs/f57-api-direct-routes.seed.tsv` | `CURRENT_SUBJECT_IMPORT` | 47 行、11 列 Control/Employee/Portal 直连 HTTP 路由以及 111 个严格组件、security/profile/schema triple 与完整 error-code set 的机器闭集；G0 无损导入后状态转为 `HISTORICAL_IMPORT_SNAPSHOT` | 从 prose 猜 route；增加通配代理/隐含错误；把上传完成误当 PUBLISHED；覆盖共享组件 profile；G0 后继续当 live projection 回写 |
| `docs/f57-fresh-pg-task-profiles.seed.tsv` | `HISTORICAL_DETAIL_INPUT` | 旧 23-task database harness/PG catalog 检查思路，可供现行测试设计引用 | 执行旧 argv、让它覆盖 G0 `fresh-pg` 的 69-baseline + contiguous-F57 算法，或作为现行机器权威 |
| `docs/f57-ci-stage-registry.seed.tsv` | `HISTORICAL_DETAIL_INPUT` | 旧 11-stage 检查意图与结果 schema 参考 | 执行 `ci-stage` 旧命令、恢复第二套 stage/verdict、或覆盖现行 Rust `f57 verify/gate` |
| `docs/f57-ci-lane-task-profiles.seed.tsv` | `HISTORICAL_DETAIL_INPUT` | 旧 F57-01…25 lane 聚合思路与 native runner 需求参考 | 把 ownership bucket 当执行节点，或覆盖现行 delivery DAG/DeliveryProfile/candidate gate |
| 本登记 | `CURRENT` | 文件范围、取代关系与阅读/执行入口；逐文件 precedence 仅引用总体设计 §1.1 | 另造文档优先级、把文件分类当项目状态，或定义业务字段/算法 |
| `2026-08-23-f57-windows-p340-production-profile.md` | `CURRENT_SUBJECT` | Windows Server、SSD/HDD、P340、UPS、备份、容量与上线门；与总体设计冲突时总体设计优先 | 宣称未经实机证据的容量、RPO/RTO、HA 或生产认证 |
| `2026-08-24-f57-converged-program.md` | `CURRENT_PLAN_NOT_AUTHORIZED` | 唯一执行索引、DAG、迁移 exact-set、L0–L3 证据和阶段提升规则 | 在未获开发授权时执行，或跳过 G0 直接进入后续阶段 |
| `2026-08-24-f57-g0-bootstrap-implementation.md` | `CURRENT_PLAN_NOT_AUTHORIZED` | G0 能力图、生成、边界与 L0/L1 启动计划 | 把计划存在当成 G0 已通过 |
| `2026-08-24-f57-authority-spine-implementation.md` | `CURRENT_PLAN_NOT_AUTHORIZED` | G1 权威主干与 G2 CTC 数据实现计划 | 在 G0 receipt 之前执行，或把 G2 当成客户端切片已通过 |
| `2026-08-24-f57-ctc01-implementation.md` | `CURRENT_PLAN_NOT_AUTHORIZED` | G3/G4 双 UI、CTC-01 与 L2 开发切片计划 | 在 verified G2 same-candidate aggregate 之前执行、拼入早期 standalone G1 receipt，或把 `DEV_SLICE_GREEN` 当成发布认证 |
| `2026-08-24-f57-expansion-release-implementation.md` | `CURRENT_PLAN_NOT_AUTHORIZED` | G5 完整集成和 G6 最高安全发布认证计划 | 跳过客户端技术门、P340、备份、恢复或最终候选同一性 |
| `2026-08-23-f57-governed-automation-fabric-implementation.md` | `HISTORICAL_DETAIL_INPUT` | 保存 2026-08-23 的文件、接口、迁移、测试和证据细节，供现行计划提取；首页主动实施指令已移除，仅保留历史引用说明 | 执行任一旧 task/命令/迁移/门禁、按编号推导全局顺序，或把旧 25-task/11-stage aggregate 当成现行门禁 |
| `2026-08-23-f57-development-readiness-verification.md` | `CURRENT` | 现行静态演算、计划覆盖和开发/生产状态结论 | 把静态闭合当成代码、实机、恢复或生产认证已经通过 |
| `docs/ci-pipeline.md` | `CURRENT_SUBJECT` | F-57 Windows authority、Apple、Android 三执行器与签名证据聚合目标契约；尚未实现 | 把旧 Linux CI、任一单 lane 或文档存在本身当作发布通过 |
| `docs/threat-model.md` | `CURRENT_SUBJECT` | 仅 `Overview` 的 F-57 产品边界与 `F-57 增补威胁与强制控制` 为现行；两个 `HISTORICAL_NON_NORMATIVE_APPENDIX` 只作攻击故事来源 | 不得恢复附录中的固定九进程、本地模型首发、第五客户端、旧凭据/扫描/备份或声明式包全局限制 |
| `2026-08-21-f50-financial-consistency-design.md` | `CURRENT_SUBJECT_INPUT` | 仅提供 F-57 保留的发票、资金、核销、退款/返款、红冲、期间、历史余额和财务一致性不变量 | 独立发出实施指令，或定义 F-57 总体架构、权限基元与客户端形态 |
| `2026-08-09-first-release-prd.md` | `PARTIALLY_SUPERSEDED` | 与 F-57 不冲突的业务字段、状态、规则、异常和验收细节 | 固定岗位授权、旧 AI/延期、固定九进程、第五客户端、旧容量与恢复值 |
| `2026-07-19-enterprise-private-operations-platform-design.md` | `PARTIALLY_SUPERSEDED` | Rust/PostgreSQL/私有部署及未冲突领域细节 | 作为当前总体设计入口 |
| `2026-08-21-f51-development-readiness-freeze.md` | `PARTIALLY_SUPERSEDED` | 未冲突的具体业务默认值；RoleCode 仅是模板种子 | 把岗位/角色解释为唯一权限边界，或恢复不允许委托/临时授权的旧结论 |
| `2026-08-22-f55-approved-ai-mcp-server-admin-cloud-freeze.md` | `PARTIALLY_SUPERSEDED` | MCP 安全意图、Windows 隔离、未来客户自控 IaaS 的最小安全意图、可复用签名/证据规则；IaaS 当前仍是 `DEFERRED_WITH_INTERFACE` | 要求首发本地模型、固定九进程、第五客户端、旧能力闭集或把 IaaS 当作当前等价生产 carrier |
| `2026-08-22-f56-license-signed-module-package-freeze.md` | `PARTIALLY_SUPERSEDED` | 许可四态、离线许可、内置模块许可信封、停用保留数据、信任链 | 把内置模块许可包等同全部能力包，或禁止 F-57 的 WASM/受控容器/客户包 |
| `docs/adr/*` | `CURRENT_SUBJECT` | 与 F-57 不冲突且状态为接受的窄技术决定 | 反向覆盖 F-57；冲突 ADR 必须另立取代 ADR |
| [ADR-0019](../../adr/ADR-0019-f57-runtime-topology-and-measured-connection-budget.md) | `CURRENT_SUBJECT` | `ep-platform-runtime`、部署可变进程数、逐硬件/配置代 exact 连接消费者与实测预算、integration-gateway 零能力 | 恢复固定九进程、强制 `ai-inferer` 或把 `37+10+5=52` 当产品真值 |
| [ADR-0020](../../adr/ADR-0020-dual-recipient-data-key-recovery.md) | `CURRENT_SUBJECT` | data_keys operational/recovery 双 recipient 信封、`PIV_SHAMIR_2_OF_3_V1` 固定 2-of-3 离线恢复、数据库打开后的 exact-ref/readback | 恢复单一 wrapped DEK、2-of-2 或让日常服务调用 recovery |
| [ADR-0021](../../adr/ADR-0021-epb1-backup-envelope.md) | `CURRENT_SUBJECT` | 独立 `EPB1` 备份信封、每 backup-set DEK、recovery-only 解密与 ciphertext evidence | 把 backup 塞入 ADR-0014 `EPC1` 或给 writer/target 历史解密能力 |
| [ADR-0022](../../adr/ADR-0022-f57-multi-lane-ci-and-windows-2022-authority.md) | `CURRENT_SUBJECT` | Windows Server 2022、Apple、Android、签名聚合四 lane 与 Rust-owned verdict | 用单 Windows runner、Windows Server 2019 或任一单 lane 放行 |
| [ADR-0023](../../adr/ADR-0023-f57-provider-manifest-resource-grant.md) | `CURRENT_SUBJECT` | `ProviderManifestV1`、`ResourceGrantV1`、carrier、调用级权限交集、撤销/漂移和显式 XML codec 边界 | 以包级权限代替调用级 grant、引用未定义 `BC-2`、隐式启用 XML 或允许 provider 直连数据库 |
| [ADR-0024](../../adr/ADR-0024-f57-backup-key-envelope.md) | `CURRENT_SUBJECT` | 每 backup set 独立 recovery-only `BackupKeyEnvelopeV1`、2-of-3 加密 share、轮换、撤销和洁净主机互操作 | 复用生产 operational recipient/token/custodian、把 backup writer 当恢复者或用 ADR-0020 代替备份域信封 |
| [ADR-0025](../../adr/ADR-0025-f57-capability-graph-and-feature-first-boundaries.md) | `CURRENT_SUBJECT` | 单一 CapabilityGraph、feature-first crate 与 touched-feature 渐进迁移 | 未获开发授权时开始 crate 重排，或把逻辑层合并成无边界巨型 crate |
| `docs/config-reference.md` | `CURRENT_SUBJECT_INPUT` | 既有配置键、默认值与说明的历史输入；G0 后机器真值是 generated config catalog | 作为实现门、增加 F-57 key、或覆盖 CapabilityGraph/generated registry |
| `docs/data-dictionary.md` | `CURRENT_SUBJECT_INPUT` | 既有类型码、表字段与分册索引输入；G0 后机器真值是 generated data dictionary | 作为实现门或第二数据库 schema/owner 真值 |
| `docs/error-codes.md` | `CURRENT_SUBJECT_INPUT` | 既有错误分类和兼容语义输入；G0 后机器真值是 generated error catalog/OpenAPI | 禁止计划使用新 typed error；把旧表当 F-57 闭集；手工双写生成 ErrorCode |
| `docs/event-catalog.md` | `CURRENT_SUBJECT_INPUT` | 既有事件语义输入；G0 后机器真值是 generated event catalog | 作为实现门或绕过 capability fact/event owner |
| `docs/metrics-catalog.md` | `CURRENT_SUBJECT_INPUT` | 既有指标语义输入；G0 后机器真值是 generated metrics catalog | 作为实现门、手工增加未归属 metric 或宣称实测已通过 |
| `docs/impact-catalog.md` | `CURRENT_SUBJECT_INPUT` | 既有影响规则输入；G0 后机器真值是 generated impact catalog | 作为实现门或恢复旧七条闭集 |
| `docs/generated/f57/registry/config-catalog.v1.json` | `G0_GENERATED_AUTHORITY` | CapabilityGraph 中配置定义的唯一机器投影 | 文件缺失时宣称已生成、手工修改或从旧 config 文档新增值 |
| `docs/generated/f57/registry/data-dictionary.v1.json` | `G0_GENERATED_AUTHORITY` | CapabilityGraph data-object/field/type/owner 的唯一机器投影 | 用 EAV/任意 SQL/旧分册建立第二 schema 真值 |
| `docs/generated/f57/registry/error-catalog.v1.json` | `G0_GENERATED_AUTHORITY` | CapabilityGraph ErrorCode、schema、owner 和 surface 的唯一机器投影 | 手工自造未生成错误码或让旧 error 文档阻塞 typed plan |
| `docs/generated/f57/registry/event-catalog.v1.json` | `G0_GENERATED_AUTHORITY` | CapabilityGraph event/fact type 与单 writer 的唯一机器投影 | 双 owner、手工事件或把日志当事实 |
| `docs/generated/f57/registry/metrics-catalog.v1.json` | `G0_GENERATED_AUTHORITY` | CapabilityGraph metric ID/type/labels/owner 的唯一机器投影 | 高基数敏感 label、手工双写或文档存在即实测通过 |
| `docs/generated/f57/registry/impact-catalog.v1.json` | `G0_GENERATED_AUTHORITY` | CapabilityGraph impact rule/source/consumer 的唯一机器投影 | 模糊通配影响、旧七条闭集或未生成依赖 |
| `docs/migration-catalog.md` | `REGISTRY_PENDING_REBASELINE` | 388-row pre-F57 版本/状态/路径与碰撞保护；当前合法态 `66 EXISTING +322 PLANNED`。G0-06 必须与本登记原子切换为 `CURRENT_SUBJECT`/`F57_BASELINE_REBASELINED`、`69 EXISTING +319 PLANNED` 并绑定 apply manifest；三草案例外随后永久消耗 | 在 G0 前创建 F57 SQL；改写 66 immutable 历史；让三个草案、catalog、register 与 apply manifest 进入混合态；G0 后再次 rebaseline；恢复任一 319 absent SQL |
| `docs/openapi/README.md` | `CURRENT_SUBJECT_INPUT` | `docs/openapi/` 七个历史/主题输入与两个永久 absent superseded path 的兼容登记；显式指向 G0 generated OpenAPI authority | 把旧九路径登记当全局 API 权威、创建 superseded control/employee path 或按旧 Task16/18 激活 |
| `docs/openapi/ai-admin.v1.yaml` | `HISTORICAL` | F-55 AI 管理 surface 的历史输入 | 作为现行 Server Control Center 或已实现 API |
| `docs/openapi/ai-reporting.v1.yaml` | `DEFERRED` | 本地模型 API 历史输入 | 在本地模型延期期间激活或宣称已实现 |
| `docs/openapi/finance.v1.yaml` | `CURRENT_SUBJECT_INPUT` | F-50 财务不变量对应的历史 API 形状输入 | 作为完整 F-57 machine contract 或绕过 generated authority |
| `docs/openapi/invoice.v1.yaml` | `CURRENT_SUBJECT_INPUT` | F-50 发票不变量对应的历史 API 形状输入 | 作为完整 F-57 machine contract 或绕过 generated authority |
| `docs/openapi/ledger.v1.yaml` | `CURRENT_SUBJECT_INPUT` | F-50 经营账不变量对应的历史 API 形状输入 | 作为完整 F-57 machine contract 或绕过 generated authority |
| `docs/openapi/mcp-management.v1.yaml` | `HISTORICAL` | F-55 固定 MCP surface 的历史输入 | 覆盖 F-57 动态签名 manifest 能力模型或宣称已实现 |
| `docs/openapi/portal.v1.yaml` | `CURRENT_SUBJECT_INPUT` | F-50 门户不变量对应的历史 API 形状输入 | 作为完整 F-57 machine contract 或覆盖 generated portal authority |
| `docs/openapi/control-center.v1.yaml` | `HISTORICAL` | 永久缺席的旧 planned path；只追溯旧命名 | 创建、激活或让任何 router/client 依赖此路径 |
| `docs/openapi/employee-api.v1.yaml` | `HISTORICAL` | 永久缺席的旧 planned path；只追溯旧命名 | 创建、激活或让任何 router/client 依赖此路径 |
| `docs/generated/f57/openapi/control-center.v1.yaml` | `G0_GENERATED_AUTHORITY` | G0 CapabilityGraph 生成的唯一 Control API；后续仅 graph regenerate | 手工创建/编辑或从旧路径复制第二真值 |
| `docs/generated/f57/openapi/employee-api.v1.yaml` | `G0_GENERATED_AUTHORITY` | G0 CapabilityGraph 生成的唯一 Employee API；后续仅 graph regenerate | 手工创建/编辑或从旧路径复制第二真值 |
| `docs/generated/f57/openapi/portal.v1.yaml` | `G0_GENERATED_AUTHORITY` | G0 CapabilityGraph 生成的唯一 Portal API；后续仅 graph regenerate | 手工创建/编辑或让旧 portal input 直接激活 |
| `docs/data-dictionary/ai_mcp.md` | `CURRENT_SUBJECT_INPUT` | MCP/provider/carrier 未冲突字段；本地模型表只作延期输入 | 作为现行 machine schema、恢复本地模型或覆盖 generated dictionary |
| `docs/data-dictionary/clm_sales.md` | `CURRENT_SUBJECT_INPUT` | CLM/销售历史字段与经济守恒输入 | 作为实现门或覆盖 feature owner/generated dictionary |
| `docs/data-dictionary/cpq.md` | `CURRENT_SUBJECT_INPUT` | CPQ 历史字段与定价不变量输入 | 作为实现门或覆盖独立 CPQ owner |
| `docs/data-dictionary/finance.md` | `CURRENT_SUBJECT_INPUT` | F-50 资金、核销与冲销不变量输入 | 作为现行表真值或合并 receivable/payable owner |
| `docs/data-dictionary/invoice.md` | `CURRENT_SUBJECT_INPUT` | F-50 发票不变量输入 | 作为现行表真值或合并 sales/purchase invoice owner |
| `docs/data-dictionary/ledger.md` | `CURRENT_SUBJECT_INPUT` | 平衡经营分录、映射、试算、对账与经营期间输入 | 扩成法定账簿、作为实现门或覆盖 operating-ledger owner |
| `docs/data-dictionary/mdm.md` | `CURRENT_SUBJECT_INPUT` | 主数据历史字段输入 | 恢复固定岗位权限或覆盖 customer-master owner |
| `docs/data-dictionary/platform_audit.md` | `CURRENT_SUBJECT_INPUT` | 既有追加写与法人规则输入 | 作为现行 schema/事件闭集或绕过 generated registry |
| `docs/data-dictionary/platform_flow.md` | `CURRENT_SUBJECT_INPUT` | 既有流程状态机输入 | 把旧步骤表当完整 Objective/Effect/Evidence/Cycle 模型 |
| `docs/data-dictionary/portal.md` | `CURRENT_SUBJECT_INPUT` | 供应商门户历史字段输入 | 复制第二业务事实或覆盖 portal identity/experience owner |
| `docs/data-dictionary/procure.md` | `CURRENT_SUBJECT_INPUT` | GRNI 与采购守恒输入 | 作为实现门或覆盖 procurement/public facts |
| `2026-08-10-first-release-dev-plan/*` | `HISTORICAL` | 可复用的旧领域测试、迁移、字段和财务任务细节 | 作为整体执行入口，或直接执行涉及 F-57 冲突主题的任务 |
| `2026-08-21-f50-financial-consistency-implementation.md` | `HISTORICAL` | F-50 财务实现任务、测试和变更清单的演进来源；现行不变量由 F-50 设计输入并经 F-57 裁决，现行执行位置由 F-57 Tasks 4/20/25 冻结 | 作为独立执行队列、沿用旧任务编号/提交顺序，或覆盖 F-57 永久期间锁定与机器闭集 |
| `2026-08-22-license-module-package-implementation.md` | `HISTORICAL` | F-56 许可与内置模块包实现细节来源；现行许可兼容由 F-57 Tasks 1/13 和 F-56 未冲突规范承接 | 作为独立执行队列，或把内置模块许可信封等同 F-57 全部能力包 |
| `2026-08-22-mcp-extension-implementation.md` | `HISTORICAL` | F-55 固定 MCP surface、隔离和测试细节的演进来源；现行动态 manifest/grant/provider 执行由 F-57 Task 14 冻结 | 作为独立执行队列、恢复固定能力闭集或固定九 operation 为现行 API |
| `2026-08-22-server-admin-cloud-carrier-implementation.md` | `HISTORICAL` | F-55 ServerAdmin/云载体/Windows 隔离细节的演进来源；现行 Control Center、载体和生产证据由 F-57 Tasks 14/16/24/25 冻结 | 作为独立执行队列、恢复第五客户端或把历史云/部署证据当成已认证 |
| `deploy/*` 的 systemd/Podman/Compose | `HISTORICAL` | 旧 Linux 部署研究和资源假设追溯 | 作为 Windows Server 2022 生产或正式测试入口 |
| `2026-08-21-development-readiness-final-verification.md` | `HISTORICAL` | 旧 F-50 至 F-56 体系曾完成静态收口的证据 | 证明 F-57 已就绪 |
| `2026-08-17-f10-ruling-detail.md` | `HISTORICAL` | F-10 决策演进证据 | 作为财务实现或未决计数依据 |
| `00f-f10-writeback-order.md` | `HISTORICAL` | F-10 当时的回写批次证据 | 作为执行队列 |
| `2026-08-22-local-ai-implementation.md` | `DEFERRED` | 未来本地模型实现研究材料 | 当前执行；第一阶段只实现 provider、授权和隔离契约 |

### 2.1 F-57 之前的 spec/review 精确分组展开

下表中的 glob 是完整匹配规则；“精确匹配结果”列逐项列出本登记覆盖的每个结果。新增匹配文件必须先在本表增加单独一行并裁定状态，不能因命中 glob 自动继承状态。

| 精确 glob 规则 | 精确匹配结果 | 状态 |
|---|---|---|
| `docs/superpowers/specs/2026-07-*.md` | `docs/superpowers/specs/2026-07-19-enterprise-private-operations-platform-design.md` | `PARTIALLY_SUPERSEDED` |
| `docs/superpowers/specs/2026-08-09-*.md` | `docs/superpowers/specs/2026-08-09-first-release-prd.md` | `PARTIALLY_SUPERSEDED` |
| `docs/superpowers/specs/2026-08-14-*.md` | `docs/superpowers/specs/2026-08-14-new-requirements-gap-audit.md` | `HISTORICAL` |
| `docs/superpowers/specs/2026-08-17-*.md` | `docs/superpowers/specs/2026-08-17-ai-analytics-shape-design.md` | `HISTORICAL` |
| `docs/superpowers/specs/2026-08-17-*.md` | `docs/superpowers/specs/2026-08-17-f10-ruling-detail.md` | `HISTORICAL` |
| `docs/superpowers/specs/2026-08-21-*.md` | `docs/superpowers/specs/2026-08-21-f50-financial-consistency-design.md` | `CURRENT_SUBJECT_INPUT` |
| `docs/superpowers/specs/2026-08-21-*.md` | `docs/superpowers/specs/2026-08-21-f51-development-readiness-freeze.md` | `PARTIALLY_SUPERSEDED` |
| `docs/superpowers/specs/2026-08-22-*.md` | `docs/superpowers/specs/2026-08-22-f55-approved-ai-mcp-server-admin-cloud-freeze.md` | `PARTIALLY_SUPERSEDED` |
| `docs/superpowers/specs/2026-08-22-*.md` | `docs/superpowers/specs/2026-08-22-f56-license-signed-module-package-freeze.md` | `PARTIALLY_SUPERSEDED` |
| `docs/superpowers/reviews/2026-07-19-*.md` | `docs/superpowers/reviews/2026-07-19-enterprise-private-operations-platform-spec-review.md` | `HISTORICAL` |
| `docs/superpowers/reviews/2026-07-19-*.md` | `docs/superpowers/reviews/2026-07-19-enterprise-private-operations-platform-spec-review-2.md` | `HISTORICAL` |
| `docs/superpowers/reviews/2026-07-19-*.md` | `docs/superpowers/reviews/2026-07-19-enterprise-private-operations-platform-spec-review-3.md` | `HISTORICAL` |
| `docs/superpowers/reviews/2026-08-04-*.md` | `docs/superpowers/reviews/2026-08-04-first-release-scope-decisions.md` | `HISTORICAL` |
| `docs/superpowers/reviews/2026-08-04-*.md` | `docs/superpowers/reviews/2026-08-04-first-release-scope-map.md` | `HISTORICAL` |
| `docs/superpowers/reviews/2026-08-04-*.md` | `docs/superpowers/reviews/2026-08-04-ledger-event-journal-refactor.md` | `HISTORICAL` |
| `docs/superpowers/reviews/2026-08-04-*.md` | `docs/superpowers/reviews/2026-08-04-single-server-deployment-decisions.md` | `HISTORICAL` |
| `docs/superpowers/reviews/2026-08-04-*.md` | `docs/superpowers/reviews/2026-08-04-v2-verification-record.md` | `HISTORICAL` |
| `docs/superpowers/reviews/2026-08-09-*.md` | `docs/superpowers/reviews/2026-08-09-prd-consistency-report.md` | `HISTORICAL` |
| `docs/superpowers/reviews/2026-08-09-*.md` | `docs/superpowers/reviews/2026-08-09-single-server-verification-record.md` | `HISTORICAL` |
| `docs/superpowers/reviews/2026-08-21-*.md` | `docs/superpowers/reviews/2026-08-21-development-readiness-final-verification.md` | `HISTORICAL` |

## 3. 主题级唯一权威

| 主题 | 唯一权威或解释顺序 |
|---|---|
| 产品定位、总体架构 | F-57 |
| 权威节点与双端 UI | F-57 §3 |
| 五层能力、进程边界 | F-57 §4 |
| 配置代、能力包、热插拔 | F-57 §5；F-56 只补充内置模块许可和信任链 |
| 数据结构与客户自定义 | F-57 §6；PRD 只补详细业务字段 |
| 动态权限与任务分配 | F-57 §7；F-51 RoleCode 只作模板 |
| 耐久自动化 | F-57 §8 → F-57 业务执行契约的 Unknown 人工处置与逐闭环关闭谓词 |
| AI、MCP、Excel | F-57 §9 → ADR-0023；F-55 只补未冲突的隔离和协议细节 |
| 四端、离线、门户 | F-57 §10 → F-57 客户端、生命周期与安全运营执行契约 |
| 业务功能 | F-57 §11 → F-57 业务执行契约 → 需求追踪矩阵 → PRD 未冲突细节 |
| 业务范围提升 | `RULING-BUSINESS-SCOPE-01` → F-57 业务执行契约 → 需求追踪矩阵逐条处置 → PRD 未冲突细节 |
| 财务一致性 | F-57 范围边界与 `RULING-FIN-PERIOD-01` → F-50 未冲突详细不变量 → PRD 未冲突细节 |
| 安全、生命周期与勒索 | F-57 §12 → F-57 客户端、生命周期与安全运营执行契约 → `docs/threat-model.md` → ADR-0024 |
| Windows Server、P340、HDD | F-57 §13 → F-57 Windows/P340 生产档案 |
| PostgreSQL 16 Windows package lock、安装与读回 | F-57 汇合主计划 canonical Task 11 的 19-field package lock、13-field install contract、4-field Event Log fixture、19-field coverage、17-field install readback 五 strict root → expansion Task 11 实现工具与静态/演练闭合 → Task 15 同一 clean candidate 的安装、RUNNING 与实机 gate → Windows/P340 生产档案；完整 installed-file/SBOM 双射、防任意 different-lock adoption、unresolved SDDL→numeric SID→live DACL、四方 system identifier、`64/4/3` GUC 与 NORMAL/RESERVED/SUPERUSER 分类预算、HBA hostssl/SCRAM 与 client channel-binding probe 分离、collector/HDD 日志、Event Log 完整覆盖及双方法同文件 fsync qualification 均属于该链；`fsync_writethrough` 只作兼容性 pin，真实耐久性由 Task 15 exact-join P340 driver/cache/UPS/flush/power-cut。`docs/config-reference.md` 的 `db.*` 只作 G0 导入输入，不能改 package/service/path/security contract |
| 备份拓扑与防勒索现场保障 | F-57 汇合主计划的 `BackupTopologySigningTrustManifestV1\|BackupTopologySigningTrustCurrentPointerV1\|BackupTopologyAuthorityV1\|BackupTopologyV1\|StorageSafeguardReadbackV1\|StorageSafeguardSupportEvidenceV1\|BackupProtectionTransitionV1\|BackupCheckpointPreparation` → expansion Task 11 工具与状态机 → Task 14 生产组合/activation 绑定 → Task 15 同一 clean candidate 的现场执行 → Windows/P340 生产档案 → threat model；ADR-0021/0024 只分别补 ciphertext/key-envelope，不替代独立 current topology-signing trust、storage-manifest singleton target、`INITIALIZING→BOOTSTRAPPING→HEALTHY` 自举、`HEALTHY→TRANSITIONING→BOOTSTRAPPING→HEALTHY` roots 轮换、fresh preparation、typed retained chain/head、A/B exact lifecycle、权限/容量/现场读回 |
| UPS 适配器与跨重启断电幂等 | F-57 汇合主计划唯一 UPS common contract → expansion Task 13 冻结 common/P340 adapter、reconciliation 和对应测试 → Task 14 以 `ep-platform-release -> ep-platform-ups-contract` 及 Authority-kernel 对 contract/Windows adapter 的直接依赖接入 POWER 长链 → Task 15 在同一 clean frozen candidate 与真实 UPS 上执行 → Windows/P340 生产档案 → threat model；Task 14 不得复制或改写 Task 13 wire，Windows standard power status 只作监测输入，不能替代最高档 signed vendor adapter/control ACK |
| 商业产品化 | F-57 §14 |
| 错误、演算、发布门 | F-57 §15 → F-57 实施计划 |
| 开发就绪静态结论 | F-57 开发就绪与全场景静态演算；它不覆盖未来真实测试证据 |
| 许可 | F-57 产品约束 → F-56 许可四态与信任链 |
| 运行拓扑与数据库连接预算 | F-57 → ADR-0019；ADR-0001 只保留共享 runtime，ADR-0018 只保留 integration-gateway 零能力 |
| 客户 data-key 信封与恢复 | F-57 `SEC-014` → ADR-0020；ADR-0009 只补数据库元数据、exact-ref/cache/readback |
| 备份密码信封 | F-57 勒索恢复 → ADR-0021 `EPB1`；ADR-0014 `EPC1` 只补 FIELD/ATTACHMENT/ARCHIVE |
| 备份密钥恢复信封 | F-57 `SEC-016` → ADR-0024；ADR-0021 继续负责 EPB1 ciphertext envelope，ADR-0020 继续只负责在线数据 DEK |
| Provider manifest 与最小资源授权 | F-57 `MCP-001`、`MCP-002`、`PKG-003` → ADR-0023；包 manifest 只给 ceiling，运行调用还必须取得 `ResourceGrantV1` |
| 多平台 CI 与 Windows 权威 | F-57 → ADR-0022；ADR-0005 只补私有自建默认、薄适配器、Rust verdict 与 Authenticode 意图 |
| 需求到任务、测试和证据的逐项绑定 | F-57 需求追踪矩阵 → `docs/f57-task-ownership.seed.tsv` → F-57 实施计划；三者不证明实现或测试结果 |
| 旧迁移到 F-57 聚合替代的逐项绑定 | `docs/migration-catalog.md` → `docs/f57-legacy-migration-disposition.seed.tsv` → F-57 实施计划 Task 1；三者不证明目录已重分类或 SQL 已创建/执行 |

### 3.1 F-57 窄 ADR 取代链

| 新 ADR | 现行窄主题 | 被取代 ADR 的精确失效部分 | 继续有效的历史部分 |
|---|---|---|---|
| ADR-0019 | 进程数为部署细节；连接消费者/预算按签名代和硬件 exact 登记、实测 | ADR-0001 的固定九进程/强制 `ai-inferer`；ADR-0018 的固定四池与 `37+10+5=52` 产品冻结值 | ADR-0001 的 `ep-platform-runtime` 共享库；ADR-0018 的 integration-gateway DB/KMS/platform-file/Outbox 全零边界 |
| ADR-0020 | operational/recovery 双 recipient DEK，任一路径恢复同一 DEK，`PIV_SHAMIR_2_OF_3_V1` 固定 2-of-3 | ADR-0009 的单一 wrapped DEK 与“不需要另一套恢复协议”结论 | 数据库业务元数据真值，以及 DB open 后的 exact-ref、cache、readback 和 16-key matrix |
| ADR-0021 | 独立 EPB1 backup envelope | ADR-0014 若被解释为覆盖 backup 的任何说法 | ADR-0014 的 FIELD/ATTACHMENT/ARCHIVE EPC1 与 exact-ref/AAD 规则 |
| ADR-0022 | Windows Server 2022、Apple、Android、签名聚合四 lane | ADR-0005 的单 Windows runner 与 Windows Server 2019 证据要求 | 私有自建 Forgejo/Woodpecker 默认、薄 adapter、Rust-owned verdict 与 Authenticode 意图 |
| ADR-0023 | exact provider/carrier manifest、调用级 `ResourceGrantV1` 与显式 XML codec | 任何未定义 `BC-2` 容器门、包级宽权限可直接执行、隐式 XML 嗅探或 provider 直连数据库的旧解释 | F-55 的 MCP 隔离/审计意图与 F-56 的包签名信任链 |
| ADR-0024 | 每 backup set 独立 recovery-only key envelope、加密 2-of-3 shares 与跨洁净主机互操作 | 任何复用 ADR-0020 operational recipient/token/custodian 或只凭 EPB1 ciphertext 便宣称可恢复的解释 | ADR-0021 的 EPB1 ciphertext 格式与每 set DEK；ADR-0020 的在线数据 DEK 双 recipient 信封 |

## 4. 冲突裁决

### RULING-AUTHORITY-01：旧索引止于 F-56

- **旧句**：F-50 至 F-56 已是最高链，旧体系可直接开发。
- **裁决**：F-57 为现行最高设计；旧开发就绪只描述 2026-08-22 快照。
- **实现后果**：README 和计划入口必须先指向 F-57；旧十四阶段计划暂停整体执行。

### RULING-AUTHZ-01：固定角色与岗位

- **旧句**：RoleCode、岗位或五类角色是主要授权与任务分配边界。
- **裁决**：角色与岗位仅是种子模板和显示标签。实际授权由 principal、capability、scope、conditions、validity 和 delegation 动态裁决。
- **实现后果**：旧角色表可以保留为种子，但不得生成绕过动态引擎的快捷路径。

### RULING-AUTHZ-02：不支持临时授权、委托或自动改派

- **旧句**：首版不含委托代理、临时授权，节点无人时只允许人工固定角色改派。
- **裁决**：F-57 要求有期限授权、委托上限、自动失效、能力解析、重新分配和权限模拟。
- **实现后果**：所有改派仍重新验证能力，改派不扩大权限。

### RULING-PKG-01：模块包只能声明 15 个内置模块

- **旧句**：模块包不能携带 WASM、容器、迁移、UI 或连接器。
- **裁决**：F-56 的 `MODULE_PACKAGE` 继续作为内置模块许可信封；F-57 新增独立 `CAPABILITY_PACKAGE`，可以声明业务对象、流程、UI、报表、受控迁移、WASM 和受控容器扩展。第一阶段必须交付 `HOST_CAPABILITY_CONDITIONAL` 的 Hyper-V-isolated Windows container adapter 与 conformance，adapter 本身不延期；具体 host 未通过 feature/nesting/capacity/security 证据时不可激活，P340 32GB 默认禁用。
- **实现后果**：禁止任意 DLL、脚本和直接 SQL 仍成立；迁移由可信编译器执行；WASM 与 Job Object worker 可用，容器按 `PKG-003` 证据门启用。
- **F-57 当前显示目录**：15 个 UUID、`ModuleCode`、依赖/许可语义和基数保持 F-56 不变；仅将 `ledger` 的产品显示名窄覆盖为“经营分录与期间”，将 `portal` 窄覆盖为“客户与供应商门户”。“总账与结账”“供应商门户”是历史显示名，不得由 seed、界面、产品介绍或安装包恢复成当前名称。

### RULING-UX-01：ServerAdmin 是第五客户端

- **旧句**：ServerAdmin 是普通独立静态 SPA/第五客户端，Win/Mac 仍承载完整系统管理。
- **裁决**：服务器控制中心属于 Windows Server 权威节点；远程页面只是显示入口。四个平台上的 Workbench 只负责办公。

### RULING-PROCESS-01：固定九进程

- **旧句**：恰好九个产品常驻进程及其连接预算是产品冻结值。
- **裁决**：信任边界和能力契约是冻结值，进程数是部署实现细节。当前硬件允许模块化主体加必要隔离进程。
- **实现后果**：旧连接预算在 F-57 新拓扑完成实测前只能作历史参考。

### RULING-AI-01：首版本地模型必须交付

- **旧句**：本地分析模型和 `ai-inferer` 是首发硬范围。
- **裁决**：第一阶段只交付 AI provider、模型/工具/提示版本、授权、审计和隔离契约；本地模型实现延期。
- **实现后果**：旧本地 AI 实施计划状态为 `DEFERRED`，不得占用当前 P340 容量。

### RULING-MCP-01：MCP 能力闭集

- **旧句**：MCP 永久固定为 F-55 的少量方法和能力闭集。
- **裁决**：核心事务仍不得通过 MCP，但 MCP 工具可通过签名 manifest 扩展；每项工具受能力、字段、网络、文件、密钥、资源、审批和审计控制。

### RULING-DATA-01：SSD 可保存客户缓存或衍生数据

- **旧句**：部分 spool、缓存、日志、模型输入、临时文件或系统默认目录可落系统盘。
- **裁决**：`HDD_STRICT` 精确约束 authority node 上承载内容或可关联客户的持久数据和衍生数据；终端仍允许最小、加密、可撤销、非权威缓存。权威 SSD 除系统/程序/静态依赖/可重建模型外，只允许 OS-managed sealed BitLocker protector metadata，以及不含客户值、对象 ID 或客户正文哈希的固定事件码和随机 incident ID。
- **实现后果**：安装和启动必须扫描数据库、WAL、日志、索引、附件、导出、temp、spool、pagefile、dump、WinCred、服务 profile 和 Windows Event Log；BitLocker/recovery key 依 `SEC-012`、`NFR-015` 分域并演练；pre-DB secret broker 依 `SEC-014` 将 DEK 分别 wrap 给日常 operational recipient 与离线 `PIV_SHAMIR_2_OF_3_V1` recovery recipient（固定生成 3 份 share、任意 2 份重构）；两份信封按 ADR-0020 精确绑定 deployment、legal entity、purpose、data-key id/version、wrap-context generation、recipient 和 envelope version，短柄另绑定 call、recipient service 与 config generation 后才可完成洁净跨机恢复。

### RULING-HW-01：统一 RTO 不超过四小时

- **旧句**：不区分数据量和磁盘速度，统一给出完整恢复 RTO。
- **裁决**：暖备与备份 RPO/RTO 只能是绑定硬件、介质、数据量、配置代和软件版本的候选 SLO；连续演练达标并签发未过期证书前，不得认证或对外展示为承诺。

### RULING-HA-01：首版完全没有暖备接口

- **旧句**：HA 整体延期，架构不需要处理提升和 fencing。
- **裁决**：第一阶段仍为单权威写节点，但架构必须预留暖备；只有增加硬件后启用。必须 fencing，禁止双主。

### RULING-OFFLINE-01：离线只是本地草稿

- **旧句**：离线能力零散且缺少统一冲突语义。
- **裁决**：离线允许最小缓存、草稿和业务意图；重连后服务器重验。禁止离线权威付款、最终审批、合同生效、库存、权限或配置修改。

### RULING-FLOW-01：工作流只保存步骤状态

- **旧句**：流程以节点、审批和补偿为中心，没有目标、责任、效果、证据和重新开启闭环的一等模型。
- **裁决**：F-57 耐久引擎必须包含 objective、obligation、effect、evidence、closure、incident 和 cycle。

### RULING-DB-01：可定制数据库等于通用 JSON/EAV 或任意 SQL

- **裁决**：客户模型由可信编译器生成真正 PostgreSQL 结构；核心区受保护，客户不得执行生产 SQL。

### RULING-BUSINESS-SCOPE-01：旧 97 项延期与新业务基线的关系

- **旧句**：旧 PRD 曾排除或延期商机/报价、客户合并、非合同来源订单、人工/生产触发采购、询比价、客户门户、服务成本与周期维保等能力；也曾把供应商门户协作写成采购域和门户域两份近似事实。
- **裁决**：旧延期仍默认有效，只有需求矩阵中逐条标为 `RETAINED`、`EXPANDED` 或 `SUPERSEDED` 的能力进入当前产品范围。CRM 只拥有商机/跟进，CPQ 独立拥有报价；采购只消费 `POR-002` 的供应商门户白名单；`STANDARD`、`DROP_SHIP` 是第一阶段必须完成认证的销售类型，`CONSIGNMENT`、`SUBSCRIPTION`、`LEASE` 仅交付 provider seam。
- **实现后果**：以 `CRM-003`、`CRM-004`、`CPQ-001`、`SAL-001`、`SAL-006`、`SAL-008`、`PROC-001`、`PROC-003`、`PROC-009`、`SRV-003`、`SRV-006`、`SRV-008`、`SRV-009`、`POR-001`、`POR-002` 的独立所有者和处置为准；未逐条提升的 HR、GRC、法务、商旅、ECM/CMS/GIS、PLM/PIM/QMS 等不自动恢复。

### RULING-FIN-PERIOD-01：经营期间锁定后的处理

- **旧句**：F-50 的“禁止反结账”与 F-57 草案曾出现“期间锁定/双人重开”两种互斥语义。
- **裁决**：采用更强且单义的永久锁定：经营期间一旦锁定就永不重开。迟到事实记入下一开放经营期间，同时保留原业务日期、顺延依据、关联原事实和更正链；法定会计期间仍由专业系统负责。
- **实现后果**：`FIN-011`、`FIN-013` 和 `AUTH-007` 必须拒绝任何重开命令；迟到顺延例外仍属于高风险 exact-set，并产生双人审批和完整审计证据。

### RULING-F10-01：F-10 计数矛盾

- **现象**：历史详本同时出现“27/22/12”和“24/13”等不同批次计数。
- **裁决**：这些是历史批次/口径差异，不是九项之外的新未决。F-10 对实现已被 F-50 取代。
- **当前未决**：0。

### RULING-POSTGRES-WIN-01：数据库版本已选定但 Windows 安装可由现场自由决定

- **旧句**：只要使用 PostgreSQL 16、数据大致位于 HDD 即可，由安装员选择服务启动、WAL/temp/config/TLS 和监听方式。
- **裁决**：package lock、服务、账户、启动/恢复、ACL、依赖、路径、有效配置、TLS、网络和 pre-HDD 零进程由 sole-owner 五 strict root 固定：19-field package lock、13-field install contract、4-field Event Log fixture、19-field coverage、17-field install readback；artifact set、scan contract 与 service-install evidence 逐层认证。`installed_files` 必须与 package/SBOM 逐文件双射并以完整向量摘要封口；V1 只允许 clean install 或相同 lock 的幂等接管，任意不同已有 build——更旧或更新——都只能进入后继签名维护升级。九路径 unresolved SDDL template 在账户/服务创建后由同一证据解析 numeric SID，live canonical DACL 逐项读回；四个独立 system identifier 值全部相等，并与 cut/restore exact-join；runtime 只接受 typed `RUNNING`。GUC 固定 `max_connections=64|reserved_connections=4|superuser_reserved_connections=3` 与 safety=2；每个 consumer 的 NORMAL/RESERVED/SUPERUSER 类、五条预算和 role attributes 均读回，应用不得占保留位。HBA 只证明 loopback `hostssl`+SCRAM，client `channel_binding=require`/协商由独立 authenticated probe 证明。`wal_sync_method=fsync_writethrough` 只是兼容性 pin；同文件 `fsync`/`fsync_writethrough` qualification 绑定卷/driver/cache，Task 15 再与 P340 UPS/write-cache/flush/power-cut exact-join才构成生产耐久性。Event Log coverage 必须闭合两个 provider registration、同 boot bookmark/record/time、零 clear/drop/gap、fixture ref/digest/complete execution 与零 token。
- **实现后果**：不得新增 PostgreSQL installer/service-configuration PowerShell、独立 signer 或 backup component；已批准的 archive/PITR operational/test wrappers 保持 closed registry rows。现有 trusted installer 只解释已签 contract；different-lock build、installed-file/SBOM 差异、列表顺序/大小写/重复/缺多、ACL template/live mismatch、路径碰撞、identifier 漂移、非 RUNNING、64/4/3/2 或 privilege-class drift、HBA/client-probe 混淆、Event Log 证据缺失/截断/错配/token 命中、日志旁路、双 fsync qualification 或 Task-15 UPS/power-cut join 缺失/陈旧，均使 final-installed generation 不可达。

### RULING-BACKUP-SAFEGUARD-01：有异机目标和两块盘即可证明抗勒索

- **旧句**：off-host Boolean、两个介质 ID 和“不可覆盖”声明足以通过上线门。
- **裁决**：必须同时具备 active-config 分别选择的 signed `BackupTopologySigningTrustCurrentPointerV1` 与 signed topology、本 attempt 新鲜 strict safeguard readback，以及 topology-pinned target 单签/介质双签 support evidence。部署 bootstrap 固定独立 trust-manifest authority；current pointer typed-load 唯一 `BackupTopologySigningTrustManifestV1`，manifest 才固定 topology signer `CN=EP F57 Backup Topology Authority,O=Enterprise Platform` 的 DN/SPKI、offline chain、revocation 与 checkpoint。私有 `BackupTopologyAuthorityV1` 只能由该 verified-current trust 值构造；禁止 topology/storage/support evidence、candidate signer、ambient Windows root、应用/备份恢复域或 ADR-0020 recipient/share roster 自认证。current authority-storage manifest 的 `backup_target_ids` 在最高档只能是 `[continuous_target.target_id]`。证据还须证明六角色与 writer/target SPKI、target receipt 的实际 role-binding principal、live 凭据/故障/管理/保管/位置域、一次性写后读、六角色权限负探针、真实 total/free/quota/reserve、partial optionality、A/B media/serial/volume/GUID/physical-disk identity 均非空且互异，以及 exact 八边状态链/无恢复材料/物理断开/健康/exact 两人保管。
- **实现后果**：clean install 只能以空 head/retained/A-B、`INITIALIZING + INITIAL_POPULATION` 通过基础设施安装，不得 PITR、发布、恢复认证或生产；sequence 1 后进入 `BOOTSTRAPPING`，先闭合 A/B 复制，再由 checked current head 逐代补足 minimum，满足全部条件后才成为 `HEALTHY/None`。current trust/topology/storage roots 只能从 fresh `HEALTHY` 轮换：单一 CAS 进入 `TRANSITIONING`，只创建一个 old-head+1 bridge；随后 `BOOTSTRAPPING + CURRENT_ROOTS_ROTATION` 只允许 A/B 离线复制/验证，不得再建 checkpoint 或二次轮换，闭合后回到健康。所有 retained refs 必须 typed-load、按序且 previous-link 完整，current head exact-join current trust/topology/storage tuple；四个非健康状态均不授权 PITR、发布、恢复认证或 production activation。install、checkpoint、PITR、activation 分别 exact-bind binding-specific checkpoint，retry 使用新 challenge/session/object；任意 self-auth、旧 head、fork/cycle/gap、错误 ref kind/media/signer/projection、非法 A/B edge、destroyed media ID 复用或容量不等式漂移都失败。

### RULING-UPS-01：Windows 电池状态可以替代 UPS 控制契约

- **旧句**：能读到 AC/电池状态并能触发关机即可满足最高安全档。
- **裁决**：Windows standard carrier 使用空设备 profile/null status profile、UNKNOWN self-test 和仅代表 carrier/configuration 的逻辑 identity，只作监测且所有未知保持未知；最高档必须使用候选绑定 signed vendor adapter、非空设备 profile、status→signed runtime binding、24 小时内 provider-attested self-test、canonical device/network endpoint、typed command/ACK 和 command-ID query/adopt。manifest 的 `implementation_binary_ref` 必须 exact-load Authority kernel 实际持有的候选二进制，`configuration_projection` 固定 generation/device profile/outlet group/protected power path；status、command 和 ACK 逐字重复其摘要与 generation。厂商成功调度必须返回 canonical 1..128 ASCII `provider_operation_id`，并在 ACK 前与 adapter identity/command ID/command digest 耐久绑定，schedule/query/log 三方逐字一致。
- **实现后果**：status 固定 5 秒采集/15 秒有效，自检 86400 秒、内层 ACK/query-adopt 30 秒；adapter 在 provider 调用前耐久化同 boot/source 的 monotonic start marker，UTC 不参与授权。POWER 600 秒只作 User32/composite/preshutdown 对账。同 ID 不同 digest 冲突，provider operation ID 缺失/变化/跨 command 或未知状态均禁止重发，boot change 前缺 composite ACK 不得事后补造；adapter 不增加服务/vendor DLL/子进程，设备、网络和凭据只按 `EPAuthorityControl` 最小权限开放。覆盖顺序固定为 **Task 13（唯一 common/P340 contract、`f57_ups_adapter_contract` 与 `f57_ups_command_reconciliation`）→ Task 14（不改 wire，只用 release→UPS-contract 与 Authority-kernel→contract/Windows-adapter 直接依赖组合 POWER）→ Task 15（同一 clean frozen candidate 的真实硬件证据）**；较晚任务不能倒推修改较早合同，也不能把 Task 13 演练说成 POWER 或实机已通过。

## 5. 维护规则

1. 新裁定必须在本登记新增文件状态和主题影响，不得只修改某一正文。
2. 历史文档保留原文，但首页必须有状态提示；不得删除历史来伪造“一直没有冲突”。
3. 新增代码、表、API、错误码、事件、指标或配置前，必须先修改其 owning CapabilityGraph node 并确定性再生对应 machine catalog；旧 markdown/OpenAPI/data-dictionary 输入不得晋级为第二权威。数据库迁移另按 baseline/apply/reservation/catalog 两态门执行。
4. 若发现没有列入本登记的冲突，开发立即停止在该窄主题，先补裁决；不允许自行选择对自己方便的一句。
5. 现场认证值不是产品未决；缺证据时保持禁用、降级或未认证状态。
6. 需求与裁决的机器可读关系只允许登记在需求矩阵 §15 的逐条 `RequirementID | SourceClause[] | Supersedes[] | RulingID[]` 表；禁止用编号区间、通配符或主题级模糊映射替代。
