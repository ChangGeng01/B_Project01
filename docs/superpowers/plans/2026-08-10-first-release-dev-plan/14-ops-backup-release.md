> **F-57 状态：`SUPERSEDED_DO_NOT_EXECUTE`。** 本文只保留历史任务正文，现行工作由 F-57 **Tasks 24 and 25** 承接。当前权威集合为 [F-57 总体设计](../../specs/2026-08-23-f57-governed-automation-fabric-design.md)、[F-57 需求追踪矩阵](../../reviews/2026-08-23-f57-requirements-traceability.md)、[F-57 权威替代登记](../../reviews/2026-08-23-f57-authority-supersession-register.md)与 [F-57 实施计划](../2026-08-23-f57-governed-automation-fabric-implementation.md)；F-57 是设计/计划权威，不是产品已实现声明，本地模型实现延期。

## 阶段 14：运维、备份与发布硬化

> **F-51/F-52/F-53 开发冻结。** U-L-10、U-L-11 已采用本文现行取值关闭；复制核对复用 30 秒 WAL 采样器并恢复三态与连续无结论窗口，写入角色遏制改为独立 `writer-role-containment` Blocking 自检项。两工具自阶段 1 为非产品骨架、在本阶段才完成真实功能。F-53 又把旧系统历史迁移、首版离线补丁分发、支持套餐周期与病毒扫描部署形态收为唯一值。病毒扫描固定为 `NONE|CUSTOMER_ICAP`，基础产品不内置 CLAMD 或病毒库。上述事项均不再等待落码选择。

本阶段交付三件事。一是把规格第 13.3 章唯一的可用性类承诺即可恢复性，从纸面变成机器上持续运行、可被检出、可被诚实披露的机制。二是把第 17.2、17.5 章、附录 A、附录 D 与第 22 章的门禁判定变成可执行工装与可归档证据包。三是交付规格第 7.6、7.10 章的历史数据迁移编排与本地工具。本阶段不新增业务模块、不新增常驻进程、不新增 schema、不新增模块码、不新增错误分类、不新增平台 Outbox 事件类型，也不新增跨模块运行期依赖；为消除 F-55 的计划期反向依赖，本阶段只在内部拆成 14a0 通用证据基础、14a1-F56-adapter 与 14b 最终认证三个批次，三者的所有权仍完全属于阶段 14。新增的 `ep-data-migrate` 是随产品交付、按需运行、无监听端口且不直连目标数据库的一次性工具，不属于 F-55 后终态九个产品常驻进程。凡涉及账务的一律指向规格第 5.2 章事件-分录表与第 10.2 章，本文不复述借贷与取价。

现行交付口径固定为 Windows Server 原生：产品服务、数据库与客户主数据卷不进入 Hyper-V 客户机。唯一窄例外是 F-55 可选 `LOCAL_WINDOWS_HYPERV_CONTAINER`，它只为单次 MCP 插件调用建立短命 Hyper-V-isolated Windows utility VM，不承载产品服务、数据库或客户主数据卷；未取得宿主或 BC-2 nested virtualization 证据时只禁该 transport。CI 只以 `cargo xtask ci` 为权威入口，默认由 Forgejo 加 Woodpecker Windows agent 调用；生产 Windows 制品必须 Authenticode，开发内部制品可用 ECDSA P-256，证书可由软件厂商或客户提供。首版补丁分发唯一形态是生产 Authenticode 签名的离线补丁包及客户侧离线验签工具；本仓、本实例与首版交付范围均不建设厂商受控在线更新网关，未来如需在线网关须另立厂商侧项目和威胁模型，不得在本仓预留隐藏回传或下载通道。裁定 F-08 现有 17 项有效 Windows 测试是首批实施与发布证据门禁，不是设计待决；原编号 12 的 ICU 建库二选一已被 `libc/C/C` 冻结取代并撤销。本文件未声称其余实机测试已经执行或通过。

### 0. 偏离共享基线与本阶段新增决定

#### 0.0 Stage 14a0 / Stage 14a1-F56-adapter / Stage 14b 内部批次边界

固定依赖 DAG 为 **Stage 1/2 → Stage 14a0 generic evidence/migrations**，以及 **(Stage 14a0 + Stage 3b-2 F-56 types/runtime/table) → Stage 14a1-F56-adapter → Stage 13c entitlement wiring → Stage 14b final certification**；Stage 14a0 与 Stage 3b-2 可并行，不能把任一者伪造为另一者的串行前置，Stage 13c 其余任务仍按其自身 DAG 与全部前置执行。这只是阶段 14 的内部拆批，不改变阶段编号、模块归属或发布责任：14a0、14a1 与 14b 的制品、表、contract、证据框架和 release-gate 都由阶段 14 拥有，13c 只能消费并扩展已冻结接口。

Stage 14a0 在阶段 1、2 后前移，交付边界只含下列封闭内容：第 3.4 节由 Stage 14 拥有的 28 个 `V20261023...` 迁移文件及 catalog 登记（止于 `V20261023092600`）；其中 `V20261023090100__platform_ops_deployment_records.sql` 的表 1 基础形状；`DeploymentRecord` 当前版本模型、仓储与唯一选择器；服务器规格/备份事实的 contract 与 fixture；统一 bounded strict-JCS、typed opaque evidence ref、SHA-256 digest、P-256 low-S P1363 sidecar 校验原语；本节十五值 gate registry 与通用 result/index ABI；DegradationKind Rust 接收域由初始 3 项扩为终态 21 项但不提前声称数据库可写；不触碰 F-56 runtime/table 的 F-55 config/product projection、applicability 与禁用态 ABI；六项 F-55 gate 的 fail-closed registry 基础；以及 Stage 1 工具骨架上只会对缺失/无效证据返回非零的 `ep-release-gate` CLI 基础。为保持全局迁移版本单调，这 28 个 Stage 14 文件必须在任何 F-55 `V20261024...` 文件向可变共享数据库执行之前全部进入 catalog；14a0 对其做静态全序校验，待各被引用前置迁移齐备时再做 fresh-database 全量验证。这里的 Stage 14 roster=28 不能冒充全局 pre-F-55 roster：后者恰为 30 个 `V20261023...` 文件，还包含 Stage 6 拥有的 `V20261023092700/092800`，止于 `092800`。任何共享数据库都必须先按全局顺序执行这 30 个文件，再执行 F-55 九个 `V20261024...` 迁移；14b 不得事后补入或改号为更低版本。

Stage 14a1-F56-adapter 的唯一增量边界是：在 Stage 3b-2 已真实交付 F-56 types/runtime/table 后，实现 `f55_entitlement.rs` 的 current/history signed-grant query，以及依赖 F-56 的 license/module gate adapter 与 focused contract tests；它必须先于 Stage 13c entitlement wiring 完成。F-56 的端到端配置发布链、Stage 13b 终态 20 个 item kind、完整 pre-F-55 PostgreSQL 链和任何真实 gate PASS 都不属于 14a1 的提前交付，仍分别等待 Stage 13b、`PreF55DatabaseAdmissionV1` 与 Stage 14b。

Stage 14a0/14a1 都明确不交付 archive/backup/history-migration 的运行时完成态，不运行真实性能、恢复、渗透、AI 资源或 carrier 实机认证，不签署报告，不把任何 gate 判为通过，不组装可发布证据包，`ep-release-gate` 对缺失真实证据必须保持非零。Stage 3b/13b 按 F-56 交付许可与签名模块包终态；Stage 13c 只消费已到位的 F-56 与 Stage 14a0/14a1 接口，并交付 F-55 代码、九个后续迁移、六项能力 gate 实现、禁用态套件和候选 evidence，这些产物仍不是发布证据。Stage 14b 在阶段 1 至 12、13a、13b、13c 全部到位后，完成本文件其余运行时、真实探针/演练/签名认证和最终 evidence package，并且是唯一有权将共同许可 gate 与六项 F-55 gate 判为通过和允许发布的批次。14a0、14a1 或 13c 完成都不得形成安装、启用或发布的部分许可。

Stage 14a0/14a1 的文件与测试落点固定如下；未标 `14a1` 的行均属 14a0。表中“通过”只表示接口、静态全序或 fail-closed 行为通过，绝不表示真实发布 gate 已通过。

| 14a0/14a1 交付面 | 精确文件 | 精确测试与命令 |
|---|---|---|
| 28 文件与 catalog 冻结 | Create：第 3.4 节列出的 28 个精确 `db/migrations/**/V20261023...sql` 路径；Modify：`docs/migration-catalog.md`、`xtask/src/sqlcheck.rs`；Create：`xtask/tests/stage14a_migration_roster.rs` | `cargo test -p ep-xtask --test stage14a_migration_roster && cargo xtask sqlcheck`；断言 exact-set=28、文件非空、版本/owner/path 唯一、rollback 段存在、`V20261023090000` 仍为唯一兼容 no-op、`090350` 仍为唯一 concurrent 文件。此处不连接共享数据库 |
| deployment record 基础 | Create：`db/migrations/platform_ops/V20261023090100__platform_ops_deployment_records.sql`、`crates/platform/obs/src/deployment_record.rs`、`crates/adapter/db-pg/src/platform_ops/deployment_record.rs`；Modify：`crates/platform/obs/src/lib.rs`、`crates/adapter/db-pg/src/platform_ops/mod.rs`（只有该模块尚未从 crate root 导出时才同批改 `crates/adapter/db-pg/src/lib.rs`）；Create/Test：`crates/platform/obs/tests/stage14a_deployment_record.rs`、`crates/adapter/db-pg/tests/stage14a_deployment_record_pg.rs` | `cargo test -p ep-platform-obs --test stage14a_deployment_record && cargo test -p ep-adapter-db-pg --test stage14a_deployment_record_pg`；只在一次性数据库夹具验证表 1 基础形状、revision/superseded 哨兵、当前行唯一选择与无当前行/多当前行 fail closed，不构成 28 文件全链通过 |
| 通用证据 contract/fixture | Create：`crates/platform/obs/src/release_evidence.rs`、`crates/platform/obs/tests/stage14a_release_evidence.rs` 与 `crates/platform/obs/tests/fixtures/stage14a/`；Modify：`crates/platform/obs/src/lib.rs` | `cargo test -p ep-platform-obs --test stage14a_release_evidence`；覆盖 server-spec/backup fact、每份不超过 1048576 bytes 的无 BOM UTF-8 RFC 8785 strict-JCS、unknown/duplicate/noncanonical 拒绝、typed opaque ref、32-byte SHA-256、`Stage14EvidenceSignatureV1` 全局八 purpose、canonical base64url-no-pad 64-byte low-S P-256 P1363 与 signer/key-state 正反例；`SecretEvidenceSignatureV1` 保持独立 |
| 签名部署清单与首装证据 ABI（14a0 contract、14b collect） | Create：`crates/platform/obs/src/deployment_manifest.rs`、`crates/platform/obs/tests/stage14a_deployment_manifest.rs`、`tools/release-gate/src/deployment_manifest.rs`、`tools/release-gate/tests/deployment_manifest_evidence.rs`、`tools/release-gate/tests/initial_governance_evidence.rs` 与 `tools/release-gate/tests/fixtures/deployment/`；Modify：`crates/platform/obs/src/lib.rs`、`tools/release-gate/src/lib.rs`、`tools/release-gate/src/main.rs` | 14a0 运行 `cargo test -p ep-platform-obs --test stage14a_deployment_manifest && cargo test -p ep-release-gate --test deployment_manifest_evidence --test initial_governance_evidence`；逐项冻结下述 complete DER ContentInfo/SignedData v3、`[0] IMPLICIT` signedAttrs 与 universal SET OF 签名前像、degenerate SignedData v1 roots、manifest/sidecar/ref/DACL、管理员证书三列 roster、两个 bootstrap 登录/MFA、无 sidecar receipt、`action='platform.key_domain.activated.v1'` exact payload/16-row projection/audit hash-chain/manifest locator 对照，以及全部具名负例，但 fixture 不能产出真实 PASS。14b 才从固定安装路径、两个 secret recipient、数据库、审计链与首张 RELEASED grant 现场重算并签 `DeploymentManifestEvidenceV1`；任一 source 缺失或不等时命令非零 |
| DegradationKind 先扩 Rust 接收域 | Modify：`crates/platform/obs/src/degradation.rs`、`crates/platform/obs/src/lib.rs`、`crates/adapter/db-pg/src/platform_ops/degradation.rs`；Create/Test：`crates/platform/obs/tests/stage14a_degradation_contract.rs`、`crates/adapter/db-pg/tests/stage14a_degradation_binding.rs`；Create（迁移后 PG 验收）：`testkit/tests/stage14_degradation_pg.rs` | 14a0 先运行 `cargo test -p ep-platform-obs --test stage14a_degradation_contract && cargo test -p ep-adapter-db-pg --test stage14a_degradation_binding`，只断言 21 值 enum/serde/参数绑定闭集；不得把非初始 3 值写入只到 Stage 2 的数据库。完整 pre-F-55 链实际应用 `092500` 后才运行 `cargo test -p ep-testkit --test stage14_degradation_pg -- --nocapture`，逐项证明 21 值可写、未知值拒绝、五项不可抑制且 Rust/SQL 同序 |
| release-gate 基础、十五值 gate-result 与 F-55 applicability ABI（14a0） | Create：`tools/release-gate/src/lib.rs`、`tools/release-gate/src/registry.rs`、`tools/release-gate/src/gate_evidence.rs`、`tools/release-gate/src/f55.rs`、`tools/release-gate/tests/stage14_gate_evidence.rs`、`tools/release-gate/tests/stage14a_fail_closed.rs`、`tools/release-gate/tests/f55_applicability.rs`、`tools/release-gate/tests/fixtures/gates/`、`tools/release-gate/tests/fixtures/f55/`；Modify：`tools/release-gate/Cargo.toml`、`tools/release-gate/src/main.rs` | `cargo test -p ep-release-gate --test stage14_gate_evidence --test stage14a_fail_closed --test f55_applicability`；覆盖 exact 15 code、仅 `PASS` outcome、index/result/two sidecar、fixed root/ref、typed roster、同 run/deployment/build/closed-window、原子 staging，以及 raw path/reparse/ADS/hardlink/跨 run/孤儿文件/伪造 PASS；同时冻结三 F-55 projection、applicability、禁用态报告与 config/product 来源。共同 gate 缺失/非 PASS、F-56 source 缺席、六 code 重复/未知、坏 digest/signature/ref/freshness/组合均非零；14a0 不实现 entitlement query 或携带真实 PASS fixture |
| plaintext-secret 终态证据与 gate ABI（14a0） | Create：`tools/release-gate/src/secrets.rs`、`tools/release-gate/tests/secret_terminal_gate.rs`、`tools/release-gate/tests/fixtures/secrets/`；Modify：`tools/release-gate/src/lib.rs`、`tools/release-gate/src/registry.rs`、`tools/release-gate/src/main.rs` | `cargo test -p ep-release-gate --test secret_terminal_gate`；冻结 receipt/terminal evidence/独立签名与六个具名负例 ABI，只对 missing/invalid/nonzero 断言转绿；不读取生产 secret、不生成真实 receipt，也不把 `RG-PLAINTEXT-SECRETS-ABSENT` 判为 PASS |
| F-56 entitlement query（14a1） | Create：`crates/platform/license/src/f55_entitlement.rs`、`crates/platform/license/tests/f55_entitlement_evidence.rs`；Modify：`crates/platform/license/src/lib.rs` | 硬前置 Stage 3b-2 后运行 `cargo test -p ep-platform-license --test f55_entitlement_evidence`；只复用已经存在的 F-56 `LicenseGrantPayloadV1/EntitlementCodeV1/LicenseStatus` 与 runtime/table，覆盖 current/history exact projection、inner/special-outer 签名与行投影相等、current slot 零或一且禁止多 current、no-current 的 `grants=[]/current_grant=None/RESTRICTED/NO_CURRENT_GRANT`、`Active|ExpiringSoon|GracePeriod|Restricted`、RETIRED 仅复验已有 accepted item、历史 CRL 隔离不计 purchased、一个 `F55Mcp` 覆盖双方向及所有坏签名/坏 source；不得等待或伪装 Stage 13b 终态 20、配置发布端到端或真实 PASS |
| F-56 license/module lifecycle gate adapter（14a1） | Create：`tools/release-gate/src/license_module.rs`、`tools/release-gate/tests/license_module_lifecycle_gate.rs`、`tools/release-gate/tests/license_module_trust_hashes.rs`、`tools/release-gate/tests/license_module_trust_rotation.rs`、`tools/release-gate/tests/fixtures/license_module/`；Modify：`tools/release-gate/src/lib.rs`、`tools/release-gate/src/registry.rs`、`tools/release-gate/src/main.rs`；14b real-PG 复用：`testkit/tests/f55_integration_pg.rs` | 硬前置 Stage 3b-2 后运行 `cargo test -p ep-release-gate --test license_module_lifecycle_gate --test license_module_trust_hashes --test license_module_trust_rotation`；14a1 的 synthetic contract 测试冻结本节全部 domain/JCS DTO、`.epcfg`/CMS/p7b exactness、immutable accepted-bundle/acceptance-audit 交叉、global-highest-then-cover CRL、正确历史撤销 containment、append-only 首次建桶 checkpoint、U+0000 拒绝、状态相关 governance 法人和初始 role/action exact-set，并验证 current/history/current-slot、许可 current 与模块 current 的独立失效域及 `LicenseAdmissionGate` typed adapter。每个 current/history trust entry 必须分别输出并校验 `origin_config_item_id`、accepted inner 与 source outer 的 signer subject/state，按 whole non-anchor chain 得出 ACTIVE|RETIRED|REVOKED，UNTRUSTED 失败关闭，禁止合成单一 signer state。lifecycle manifest 还必须带 deployment-manifest 与 initial-governance 两个 exact child ref/digest，并验证签名部署清单、无 sidecar receipt、首装 DB/审计/grant/key-domain 与双 X509/MFA。共同 gate 的 exact roster 仍恰为 `license_module_lifecycle_matrix|license_admission_registry_exact_set|license_admission_negative_matrix|license_trust_rotation_exact_set` 四 code，缺任一、lifecycle/child/registry/trust digest 不等、负例未运行、层身份/状态合并或伪造 result 均非零。Stage 13b 配置包全链、终态 20、完整 pre-F-55 链与真实 PostgreSQL 的 `data_keys` CHECK/时间单调/wrapped-key 约束、data-key u16/EPC1/ref/row/65535 rotation 边界、KEK `1..=2147483647` checked conversion 及落库零副作用负例，只能在 14b 通过 `cargo test -p ep-testkit --test f55_integration_pg -- --nocapture` 与现场 collector 收口；14a1 不产生真实 `RG-LICENSE-MODULE-LIFECYCLE-GREEN=PASS` |
| 13c 前数据库准入交接 | Reuse：`testkit/tests/f55_integration_pg.rs`、`scripts/verify-release.ps1` 与 `xtask/src/ci.rs`；本行由 13c 按其 Task 3/5 创建或扩展，不在 14a0/14a1 复制第二份实现 | 在 13c Task 3 Step 3 之前先运行下述 `PreF55DatabaseAdmissionV1`；随后才可运行 `cargo xtask sqlcheck && cargo test -p ep-testkit --test f55_integration_pg -- --nocapture`。缺任一前置时保持非零，不以跳过或 `#[ignore]` 过渡 |

上述 `stage14a_deployment_manifest`、`deployment_manifest_evidence`、`initial_governance_evidence` 与 `license_module_trust_rotation` tests 还必须逐项覆盖 signed license signer roster 的 1/64 边界、0/65、非法 token、乱序/重复、inner/outer signer 不在 roster、local `[]` no-override、local 非空 exact-equal 与增删/替换失败、roster digest 四方相等，以及 CAB 同批 roster+bundle 正例和只换一侧/跨 batch/deployment/build 拼接负例；这些仍是 synthetic fail-closed contract，不得生成真实 PASS。

`PreF55DatabaseAdmissionV1` 是 Stage 14 拥有、13c 消费的执行前判据，不是可签名业务 DTO，也不新增数据库表。它只有同时满足以下六项才为真：一，catalog 与磁盘上第 3.4 节 28 个 Stage 14 路径 exact-set 相等且均非空；二，全局 catalog 中每个版本 `<20261024090000` 的文件都存在、checksum 与各自已批准值一致；三，`092400` 所需的 `platform_core.unpoliced_table_registry` 与矩阵 case、`092500` 所需的备份/归档/恢复表和 append-only/immutable/audit 基础、`092600` 所需的 flow/authz/reauth、Stage 8 `MIGRATION_HISTORY`、Stage 9 `HISTORICAL_MIGRATION` 以及第 4.12.1 节 25 个静态 owner projection 全部已由其拥有阶段真实交付，且 Stage 6 的 `V20261023092700`、`V20261023092800` 也已真实存在；四，一座全新的 PostgreSQL 16 一次按全局版本从零执行且只执行所有 `<20261024090000` 的已登记迁移，Stage 14 自有 28 文件的最后一条是 `V20261023092600`，而全局 pre-F-55 最后一条必须是 `V20261023092800`，并通过 catalog/checksum/rollback/static projection 断言；五，目标共享数据库尚未应用任何 `V20261024...` 且不存在“已应用较高版本、漏掉任一 `<20261024090000` 版本”形状；六，对该共享数据库只允许先通过本 admission 并按全局顺序补齐全部 `<20261024090000` 迁移（其中含 Stage 14 的 28 个 `V20261023...`，并在其后接续 Stage 6 的 `092700/092800`），随后才把九个 `V20261024...` 作为一个 F-55 批次执行，中途任一失败即整批停止且不得把部分 F-55 当成可用。若第五项失败，目标库直接判为不具备 F-55 升级资格；开发/测试库必须从规范迁移链重建，任何环境都禁止手工插 `schema_history`、事后执行低版本或改号绕过。

这一定义把“14a0 在阶段 1、2 后可完成”精确限定为文件/catalog、contract、focused disposable fixture 与静态/fail-closed 工装完成；14a1 则只在 Stage 3b-2 后接入 F-56，它们都不声称 28 个迁移此时已可在共享数据库执行。13c Task 1、Task 2 和 Task 3 的 red test/九个 SQL 候选编写按 13c 自身 DAG 开始；其 entitlement wiring 不得越过 14a1。`PreF55DatabaseAdmissionV1` 先在执行完整 `<20261024090000` 全局迁移链、末项为 `V20261023092800` 的 disposable fresh database 上转绿，随后 13c Task 3 才在同类 disposable database 接续九个 `V20261024...` 并完成整条全链绿验，最后才允许对满足第五、六项的共享数据库执行。由此既不把尚未实现的九个 F-55 SQL 反向变成 admission 自身前置，也不把阶段 6/8/9 与 25 个业务 projection 塞进 14a0 开工依赖，更不会让早期 13c 越过缺失的 pre-F-55 迁移先落 `V20261024...`。

偏离一，落点写出直接出网，不经 integration-gateway。基线第 2 节写 integration-gateway 是首版唯一对外出网进程。规格第 13.4 章认证的落点类型含客户对象存储，同章落点侧访问控制条规定写出侧凭据只由写出组件的系统账户持有、不复用于其他进程，第 7.7 章又逐项枚举了两个写出进程的凭据持有范围。因此对象存储落点的写出必须由 archive-writer 与 backup-writer 自身发起。本阶段把该句收窄为 integration-gateway 是首版唯一面向外部业务系统的出网进程，落点写出不在其内，并提出基线第 2 节修订。影响范围：需为 archive-writer 与 backup-writer 各加一条 Windows 防火墙按服务短名限定的出站规则，放开到落点的出向网络，目的地址集合固定为部署记录所载落点，不接受运行期变更。

偏离二，本阶段的部署级 platform_ops 台账表不带 legal_entity_id，也不建行级策略；历史数据迁移的六张表不适用这项偏离，必须带 legal_entity_id、ENABLE 且 FORCE 行级安全。基线第 3.8 节把 platform_ops 的机器级指标列为不带法人列的四类之一，第 4 节又要求每张业务表带 legal_entity_id。规格第 15.3 章要求台账同时覆盖两类按法人与会计期间归属的条目，即内部对账校验未完成与关账受理被拒。若给部署级台账加 legal_entity_id 并套第 3.8 节模板，部署级条目的法人列必然为空，行级策略下 NULL 比较结果为 NULL，这些条目对任何人都不可见，台账失效。本阶段取值：表 1 至表 17 的部署级台账不带 legal_entity_id，按各表定义使用 scope_legal_entity_id 与 scope_accounting_period_id 等可空的展示归属标注列；不建策略；读取侧可见性由 ABAC 在应用层按运维管理员、安全管理员与审计管理员三类角色判定。该偏离的准入判据是台账各行与法人无关，即其行要么在本部署内对全部法人取值相同，要么是部署自身的元数据；隔离承接入口是运维管理员、安全管理员与审计管理员三类角色的 ABAC 判定。表 18 至表 23 含来源记录键、差异、批准证据与迁移执行结果，属于法人业务数据，不得登记进 unpoliced_table_registry，按基线第 3.8 节的法人表模板建策略并进入 rls_matrix。原先援引的不带法人列的表只有四类这一封闭枚举已被三个阶段各自突破而作废，本节改按上述两项判据自证，不再援引该枚举。该偏离由阶段 2 先行落实最小台账：`platform_ops.degradation_windows` 首次建表只允许 `OFFSITE_SINK_NOT_CONFIGURED`、`WRITER_NOT_IN_SERVICE`、`PORT_NOT_IMPLEMENTED` 三个 kind，并交付 `ux_degradation_windows_kind_scope_closed` 与 `ck_degradation_windows_open_order` 两条约束；Stage 14a0 先把 Rust enum/serde/ledger 接收域由 3 扩为本文终态 21 以解除后续编译倒挂，但不向旧库写新增值；`V20261023092500` 在所有既有早期阶段已完成且 F-55 尚未执行时才把数据库 kind CHECK 从 3 扩为同序 21、把不可抑制闭集扩为 5，并与匹配 Rust binary 同一部署批发布，不重建表、不增删列。

偏离三，平台端点的路径模块段取 platform。基线第 5.1 节的 module 段只枚举了 15 个业务模块码，错误码段已允许 PLATFORM。本阶段取值：平台自身资源路径固定为 /api/v1/platform/<resource-plural>，事件类型的模块段同样允许取 platform，与错误码的 PLATFORM 段一一对应，并提出基线第 5.1 节与第 6.1 节修订。

偏离四，ops-agent 的两个端点不使用第 5.2 节封套。/metrics 输出 Prometheus 文本格式，/healthz 与 /readyz 输出精简 JSON，二者不带 success 与 error 字段，也不要求 Authorization 与 Idempotency-Key。理由是其消费方为 Prometheus，封套使之不可解析。原并列的 systemd 侧消费方即 sd_notify 就绪协议在本平台没有承载物，就绪的对外声明改由服务宿主自身的 SetServiceStatus(SERVICE_RUNNING) 承担，与本节两个端点无关；Prometheus 一侧不受影响，本偏离的结论即两个端点不使用第 5.2 节封套不变。二者只监听 127.0.0.1，不承载任何业务数据。

本阶段新增决定，基线未覆盖，阶段结束时回写基线：落点可写性判定的连续失败与连续成功阈值、三类写出的周期取值、部署级备份加密的算法与对象格式、恢复模式的触发方式、台账 kind 枚举、RPO 依据枚举的排序算法、写出进程本地暂存上限。逐项取值见第 4 节与第 7 节。

F-51 冻结事项：U-L-10 采用可见角色三类、入口在运维中心一级导航、导出格式为 JSON 与 CSV 两种；U-L-11 采用独立的“部署状态与已知限制”页面，客户确认留痕写审计。两项均已关闭，不再使用“临时取值”或“被阻塞”表述。
### 0.1 T0 贯通线、空实现硬规则与启动自检口径

T0 贯通线。阶段 3b-1 结束后、阶段 5 全量开工之前插入一条不新增任何范围的最薄贯通线 T0，从阶段 5、6、9a、10、11 各取最小切片，判据是一条合同从建单走到管理层看到一个数。固定业务主干关键路径为 1 → 2 → 3a → 4 → 3b-1 → T0 → 5 → 9a → 8 → 6 → 7 → 10 → 11 → 9b → 14b，共十五个环节，14b 是末环；F-55 的证据汇入支链精确为 1 → 2 → 14a0，以及 3b-2 → 14a1，再汇入 13c entitlement wiring → 14b。阶段 3b-2 不在业务主干链上，阶段 12 在阶段 10 之后与阶段 11 并行，13a 在阶段 1 后并行推进，13b 在阶段 3b 与阶段 11 后和 9b 并行。14b 最终发布验收同时等待 13a、13b、13c 和完整 pre-F-55 链，但不把这些终态前置错误提升为 14a0 开工门槛。本阶段不向 T0 贡献任何切片；T0 对本阶段的唯一影响是 14b 接手的是一条已被真实调用打通过的闭环。M7 保留为全分支闭环，14b 的 M12 仍为交付验收；14a0/14a1 都没有发布里程碑。

空实现的硬规则。原裁定通则第三条那套 Noop 空实现加 TODO 加验收顺延的通用机制整体删除，改为一条硬规则：跨模块同步调用的被调方必须与调用方同批交付；做不到就把该用例整条推迟到被调方所在批次；两者都不可行时才用降级窗口把缺席表达成台账事实。三者之外不允许任何形态的替身，也不允许任何返回零值、空集合、固定业务分支或恒定成功的实现。本阶段没有向后续阶段留下的注入点。DisposalPort 是通则第三条例外清单三项之一，例外档的落法就是降级窗口加直接拒绝，不走整条推迟：处置受理路由由阶段 3b 注册，阶段 3b 至阶段 13 之间 apps/core-server/src/wiring/ 与 apps/job-worker/src/wiring/ 两个目录内不出现该端口的任何替身也不出现任何注入行，物理删除请求一律以 PLATFORM.DISPOSAL.NOT_DELIVERED 直接拒绝，category 取 BUSINESS_CONFLICT，HTTP 409，不可重试，同时开一条 kind 取 PORT_NOT_IMPLEMENTED 且 subject 取 DisposalPort 的降级窗口，界面与健康端点显式呈现该能力未交付，指标 ep_degradation_windows_open 自动计数；本阶段是该端口的唯一实现方，注入 OpsDisposalService 后关闭该窗口。该规则由第 8.7 节的发布门禁项 RG-UNWIRED-ABSENT 强制，其判据提供方是阶段 1 随 xtask 交付并配负样例的 archcheck 规则 unwired-absent，断言对象是上述两个目录下的全部文件；原先那句由 xtask 门禁统计空实现数量在十一个子命令中无落点，一并删除。

启动自检的口径。启动自检不再充当数据一致性闸门。自检项按 severity 分 Blocking 与 Degrading 两档，判读运行期可变业务数据行的项一律不进 Blocking 档，闸门移到部署与升级前置的 --check。`offsite-sink-requirements` 保持一个整体 Degrading 项，只含服务器之外落点的七项判定，任一不满足只按降级状态启动并持续告警、记录暴露窗口。规格第 7.7 章两个专用角色的三项遏制手段拆为本阶段新增的 `writer-role-containment` Blocking 项，只对 archive-writer 与 backup-writer 适用并在建立任何复制连接前执行；其他进程一律 NotApplicable。任一遏制手段缺失时对应角色不得启用、写出进程以退出码 78 退出；core-server 以进程未投入运行这一客观事实开 `WRITER_NOT_IN_SERVICE`。角色已启用后的周期核对 `NO_RESULT` 属运行期状态，只走 `REPLICATION_CROSSCHECK_NO_RESULT`，不得反向触发该启动阻断项或停止写出进程。archive-writer 与 backup-writer 对全部 SQL 类自检项仍标 NotApplicable。逐项口径见第 7 节末。

---

### 1. 交付物清单

本阶段结束时下列东西存在且可运行。

1. archive-writer 可执行进程，承载三项写出：事务日志连续归档写出、附件正文向服务器之外落点的增量写出、审计证据存储向落点的写出，三项各自不超过 15 分钟周期，三项之间的先后由进程内部调度落实。事务日志的接收由本进程监管 PostgreSQL 16 自带的 pg_receivewal 完成，本进程不实现流复制协议，接收结果先落本机 WAL 暂存目录，再由本进程加密写出到落点。含附件正文写出点水位推进器、本地 spool 暂存与补写、落点可写性持续判定。审计证据目录 C:\EP\audit-evidence 的权限位换 NTFS ACL，不设共用本地组，逐账户列 ACE：只授 job-worker 的服务虚拟账户 NT SERVICE\ep-worker 写入，archive-writer 的服务虚拟账户 NT SERVICE\ep-archive 只读，并对后者显式 Deny DELETE 与 FILE_WRITE_DATA；本进程以该只读 ACE 读取并写出到服务器之外落点，对该目录只有读权限，不具备写入与删除权限，证据文件与段根签名由 job-worker 产生。
2. backup-writer 可执行进程，承载四项：每日全量基础备份（流式，本机只留暂存缓冲）、附件正文的存量引导搬运与每日全量写出、备份自动校验、归档链断裂后重建恢复基线的那一次全量基础备份。两次全量基础备份均由本进程监管 PostgreSQL 16 自带的 pg_basebackup 以 -X stream 完成，校验沿用 pg_verifybackup，本进程不实现流复制协议。另承载配置、证书、模块包、低代码规则包与基础设施定义的随日全量同批写出。另含恢复模式，承担整机失效恢复、密钥恢复材料隔离恢复与保留期尾端恢复三类演练的编排，第三类按裁定 F-11-4 新增；它与第一类共用同一条恢复路径，区别只在所选备份集不是最近一次 VERIFIED 的那一份而是保留期尾端的一份，不新增机制、不新增进程、不新增落点。
3. ops-agent 可执行进程，暴露 127.0.0.1:9101 指标端点与 127.0.0.1:9102 健康端点，以 ep_ops_ro 只读角色读取运维视图。
4. core-server 内的运维中心用例集与只读 API：降级与暴露窗口台账、两个 RPO 取值与依据、备份集与校验结论、归档通道状态、容量水位、部署记录、密钥恢复材料核验登记、恢复演练登记。
5. core-server 内的写出上报受理器：接收两个写出进程经 Windows 命名管道上报的七种报文，在同一事务内写 platform_ops 表与审计事件并按第 15.3 章开闭暴露窗口；本阶段新增 Outbox 事件固定为 0，不为部署状态写 Outbox。
6. ep-adapter-sink，落点适配层，三种认证落点类型的统一写入、读回、探针与吞吐实测。
7. 部署级备份加密实现，落在 ep_foundation::port::kms::KmsBackend 端口之上，载体实现由 ep-adapter-kms 提供，实例级密钥、信封加密、写出前施加、附件正文保持法人密钥域原密文不二次加密。
8. 归档通道状态机与断链处置器，含落点可写与不可写两支，含归档通道暂停终态。
9. 恢复编排与恢复点对齐算法，含附件元数据与正文逐条一致性校验的流式实现。
10. 性能与容量认证工装 ep-bench：负载生成器、必判必记项采集器、认证报告生成器。
11. 发布门禁工装 ep-release-gate：证据收集、按第 17.2 章通过标准与第 22 章十五条逐条判定、发布证据包组装。
12. 供应链安全流水线：SBOM、签名、可复现构建、离线依赖仓库、客户侧验签工具。
13. 等级保护三级控制项自评矩阵与四项永久性不符合项封闭清单，落在 docs/compliance/ 并由 CI 校验不得超出封闭清单。
14. 恢复手册、运维手册、部署记录模板与交付说明的诚实披露八条文本，落在 docs/runbooks/ 与 docs/delivery/。
15. OpsDisposalService，位于 crates/platform/obs/src/disposal.rs，实现阶段 3b 在 crates/platform/file/src/port/disposal.rs 定义的 DisposalPort，承担附件对象、密钥域、备份集与扩展表四类处置范围的执行，含密钥销毁与到达备份保留期的备份集处置，产出销毁证明；注入后关闭阶段 3b 起开着的 kind 取 PORT_NOT_IMPLEMENTED 且 subject 取 DisposalPort 的降级窗口。
16. **版本与补丁清单的本地导出**（按裁定 F-44 决定三）。结构化导出本实例的版本号、已装补丁清单
    及其安装时点、许可证状态与健康状态摘要，供人工携出交厂商登记。**本实例不建任何对外出站通道**——
    使用方已裁定不交付「厂商轻量部署管理通道」，改由本地导出加人工携出承接。
17. 对外表述逐条分档清单与产品负责人签字页。清单覆盖交付、认证、验收材料和客户合同的每一条对外表述，逐行记录 `statement_id`、材料路径与版本、材料摘要、表述原文、规格第 21.22 章第一/二/三档、使用结论、前提或证据引用、签字人与签字时点；签字页绑定整份清单及全部材料的摘要，材料或清单任一字节变化即失效并须重新判定、重新签字。本项由阶段 14 唯一承担，不顺延给销售、实施或发布后的人工补签。
18. `ep-data-migrate` 历史数据迁移工具与 core-server 内的迁移编排器。工具支持 XLSX/CSV、只读 ODBC 数据源、本地或 SMB 文件清单、经模板逐项白名单批准的 HTTPS API 四类来源；完成字段映射、清洗、显式安全属性赋值、完整试运行、分批推送、错误重跑、源系统只读冻结、增量追平、对账、切换与整批冲销计划。它只经 core-server 的受控迁移 API 调用各模块迁移写入者，不持目标数据库凭据、不直写任何业务表、不常驻、不监听、不把来源原文或凭据持久化进平台数据库。四类来源的规则统一保存为版本化签名模板，源记录以规范化 SHA-256 与来源键追溯。`ep-data-migrate` 与 DDL 工具 `ep-migrate` 是两个不同的二进制，命令、账号、窗口与错误码不得互用。
19. `ep-secretctl` 密钥引导、显式版本轮换与既有机密迁移的一次性本地工具，位于 `tools/ep-secretctl/`，随产品交付但不注册 Windows 服务、不监听端口、不持事务数据库凭据、不提供明文导出或降级命令。顶层子命令闭集恰为 `bootstrap|put|verify|migrate|finalize-migration|retire|inventory|wincred`；不存在顶层 `rotate`、`receipt-verify` 或别名，版本轮换由 `put` 准备新版本、签名配置显式切换与 `retire` 退役旧版本组成。它只接受阶段 2 冻结的 KMS/HSM 与 `EPS1` envelope contract；fresh install 与 Stage 1 legacy migration 分别产生第 8.7 节冻结的原始 receipt 集，Stage 14b 再以签名 `SecretTerminalEvidenceV1` 绑定 build、provider、输入/输出 inventory 与该 receipt 集，不向原始 receipt 塞入明文、明文摘要或 ADR-0007 闭集外字段。bootstrap、HSM、receipt 或 terminal evidence 验证失败即非零且没有 file/plaintext fallback。它与 `ep-data-migrate` 相同，必须进入产品 SBOM、依赖/安装包/密钥扫描、生产 Authenticode 验签及 Windows 两次可复现构建，不适用 `ep-bench`/`ep-release-gate` 工具排除规则。

---

### 2. crate 与进程归属

新增 crate。

| crate | 归属进程 | 职责 |
|---|---|---|
| ep-adapter-sink | archive-writer、backup-writer | 三类落点的写入、读回、列举、探针、吞吐实测；不含加密、不含业务语义 |
| ep-data-migrate | 一次性本地工具，随产品交付 | 位于 tools/data-migrate/；无监听端口，不注册 Windows 服务，不占用常驻连接或资源单位，不复用 DDL 工具 `ep-migrate` 的数据库账号。只读解析 XLSX/CSV、ODBC、文件清单和白名单 HTTPS API，把规范化记录按每块最多 1000 行且规范化 JSON 请求体不超过 524288 字节（两者先到）经员工 API HTTPS origin 推送；含 HTTP 封套的单请求仍不得超过 1 MiB。生产 PE 必须 Authenticode 并进入产品 SBOM |
| ep-secretctl | 一次性本地工具，随产品交付 | 位于 `tools/ep-secretctl/`；顶层子命令闭集恰为 `bootstrap|put|verify|migrate|finalize-migration|retire|inventory|wincred`，无 `rotate`/`receipt-verify` 别名；生产只认 `secrets.provider=kms` 和 `EPS1`，无监听、无常驻服务、无事务数据库凭据、无明文输出、无 file/HSM fallback；生产 PE 必须 Authenticode，并进入产品 SBOM、扫描与可复现构建 |
| ep-bench | 不随产品交付 | 位于 tools/bench/，自阶段 1 已是工作区非产品骨架且交付前固定返回 `EXIT_NOT_DELIVERED=70`；本阶段完成负载生成器与认证采集器，只有真实成功才返回 0；始终不进入发布制品与产品 SBOM |
| ep-release-gate | 不随产品交付 | 位于 tools/release-gate/，自阶段 1 已是工作区非产品骨架且交付前固定返回 `EXIT_NOT_DELIVERED=70`；本阶段完成门禁判定与证据包组装，只有真实成功才返回 0；始终不进入发布制品与产品 SBOM，`RG-TOOLS-EXCLUDED` 断言两个包名均不存在 |

改动 crate。

| crate | 归属进程 | 改动 |
|---|---|---|
| ep-platform-obs | core-server、job-worker、ops-agent | 复用阶段 2 已交付的 DegradationLedger 与初始 3 项 degradation_windows；Stage 14a0 先把 Rust enum/serde/ledger 接收域扩为终态 21 项，完整 pre-F-55 链执行 `V20261023092500` 后再让数据库 kind CHECK 接受同序 21 项并注册 `legal-entity-key-domain-coverage`，任何非初始值的真实 PG 写测在此前不得转绿；新增运维中心台账模型：RPO 依据判定、容量水位、部署记录；新增历史数据迁移编排、六张迁移台账表（含批准证据与 writer 回执）的仓储与迁移窗口状态机；新增 crates/platform/obs/src/disposal.rs 的 OpsDisposalService，实现阶段 3b 在 crates/platform/file/src/port/disposal.rs 定义的 DisposalPort；新增 crates/platform/obs/src/capability.rs 中本阶段各用例的能力域码与动作类别常量；注册并填充基线早已具名的 `ep_archive_write_lag_seconds`、`ep_attachment_write_lag_seconds`、`ep_backup_last_success_timestamp_seconds` 三项指标 |
| ep-platform-file | core-server | 新增附件写出范围查询端口，向 archive-writer 提供对象范围与元数据提交状态；不改动上传流水线状态机 |
| ep-platform-audit | core-server | 新增审计证据存储的写出范围查询端口，供 archive-writer 取段根与签名对象；审计链与分段签名本身不改 |
| ep-adapter-kms | archive-writer、backup-writer | 新增实例级部署备份加密密钥的解封与信封操作；端口 trait 为 ep_foundation::port::kms::KmsBackend，本 crate 只提供其载体实现；工作区内不存在 ep-platform-kms，该名作废 |
| ep-adapter-ipc | 全部 | 新增本阶段七种报文类型 |
| ep-platform-recon | job-worker | 本体、三张表、ReconCheck 与 ReconExecutor 由阶段 9a 交付；本阶段只新增恢复验收模式的调用入口与留证字段，调用形态为 ReconExecutor::run，run_kind 取 RECOVERY_ACCEPTANCE；本阶段不实现也不注册任何 ReconCheck，注册方清单见裁定 A-06 |
| apps/archive-writer、apps/backup-writer、apps/ops-agent | 同名 | 由骨架变为完整实现 |
| ep-app-mdm、ep-app-cpq、ep-app-clm、ep-app-sales、ep-app-procure、ep-app-inventory、ep-app-project、ep-app-service、ep-app-finance、ep-app-ledger、ep-app-invoice | core-server | 各模块在自己的 crate 实现 `ep_platform_obs::data_migration::MigrationModuleWriter`，只处理第 4.12 节登记给自己的 `module_code.object_type`；`validate` 纯校验且不落正式数据，`apply` 与 `plan_reversal/apply_reversal` 复用本模块唯一权威写入者。交易事实保持仅追加；可变根只能经具名 owner 命令形成当前 after-image，并同事务追加不可变 version/change/correction fact。模块之间不得直接调用或直写对方表；crm、costing、portal 与 reporting 的派生数据不直接迁移，按第 4.12 节重建 |
| apps/core-server | core-server | 新增运维中心用例、上报受理器、未知复制会话检出的装配，以及历史数据迁移 API、模块写入者注册表和同事务审计装配 |

依赖方向核对。ep-adapter-sink 只依赖 ep-foundation 与 ep-contract-*，不依赖 application，其重试与退避逻辑下沉 ep-foundation。ep-platform-obs 不依赖任何 domain 与 application。archive-writer 与 backup-writer 两个 apps 不依赖任何 ep-app-*，其与 core-server 的全部交互只经 ep-adapter-ipc 的报文类型，这七种报文类型定义在 ep-adapter-ipc 内，与本节改动表中 ep-adapter-ipc 一行是同一批，也与阶段 13 对 plugin 通道请求与响应类型的处置同形，ep-foundation 不新增 ipc 模块；这七种类型不得被任何 ep-platform-* 命名，ep-platform-runtime 侧的 IPC 服务端 trait 一律以泛型参数或字节切片表达，不出现其中任何一种，否则即构成 ep-platform-* 依赖 ep-adapter-*，由阶段 1 随 xtask 交付并配负样例的 archcheck 规则 platform-no-adapter 判红；core-server 侧对上报内容的落库与审计在 apps/core-server/src/wiring/ 处转换为 platform 类型。两者对 pg_receivewal 与 pg_basebackup 的监管只经进程启动、终止与退出码，不链接任何 PostgreSQL 客户端库。「启动」与「退出码」两半在本平台原样成立；「停止」这一半没有干净等价物：本平台没有跨进程投递 SIGINT 与 SIGTERM 的机制，只能取终止承载该子进程的作业对象，或向其投递控制台事件，二者都是强制终止，不是优雅停止，该结论写入本阶段风险节与交付说明（不冒用规格第 21.21 章的编号——该章讲的是备份角色绕过隔离与整簇副本，与本条不是同一件事），界面与文档不得把它表述为优雅停止或排空。

前置依赖。本阶段在调整后的阶段顺序中排在最后，下列前置件在本阶段开工前均已存在，本阶段不重复交付，也不向任何后续阶段留空实现。一，ep-foundation 的 SecurityContext、SYSTEM_PRINCIPAL_ID、SYSTEM_DEVICE_ID、CapabilityDomain 与 ActionClass 由阶段 1 提供。二，platform_ops schema、platform_ops.degradation_windows（含 subject 列与只允许 `OFFSITE_SINK_NOT_CONFIGURED`、`WRITER_NOT_IN_SERVICE`、`PORT_NOT_IMPLEMENTED` 的初始 3 项 kind CHECK）与 ep-platform-obs 的 DegradationLedger 由阶段 2 提供；阶段 2 只拥有这三项的持久化触发路径，并为 `legal-entity-key-domain-coverage` 只交付判读 provider、结构化结论与两个稳定错误，不注册、不调用 ledger。Stage 14a0 先扩 Rust enum/ledger 接收域至 21；完整 pre-F-55 链中的 `V20261023092500` 再扩 SQL CHECK 至同序 21 并把不可抑制闭集由 4 扩为 5；只有此后本阶段才把 provider 注册进 SelfCheckRegistry，并以 `LEGAL_ENTITY_KEY_DOMAIN_UNAVAILABLE` 按法人开闭，严禁提前写库或借用 `PORT_NOT_IMPLEMENTED`。三，crates/platform/file/src/port/disposal.rs 的 DisposalPort、DisposalRequest 与 DisposalReceipt 及其处置受理路由由阶段 3b 提供。四，ep-platform-recon 本体、三张表与 ReconExecutor 由阶段 9a 提供。五，ep-adapter-esign 与其 crates/adapter/esign/tests/contract_sandbox.rs 契约测试由阶段 6 提供，本阶段只执行并归档其对真实沙箱的通过记录。

---

### 3. 数据库变更

全部落在既有 platform_ops schema，不新增 schema。属主为 ep_mod_platform_ops，运行期读写由 ep_app_rw，ops-agent 只读由 ep_ops_ro。本节全部表的 created_by 与 updated_by 在系统上下文与种子迁移中一律取 ep-foundation 的 SYSTEM_PRINCIPAL_ID，即 00000000-0000-7000-8000-000000000001，不得自选其他字面量；两个写出进程经 IPC 上报产生的条目同样取该常量，理由是这两个进程不持有人类主体身份。

公共列约定。本阶段 platform_ops 表一律带 id uuid 主键（应用侧 UUIDv7）、security_level smallint not null default 20、data_scope_tags text[] not null default '{}'、created_at timestamptz not null default now()、created_by uuid not null。可更新表另带 row_version bigint not null default 1、updated_at timestamptz not null default now()、updated_by uuid not null。仅追加表不带 row_version 与 updated_*；只有存在业务冲销语义时才带 `reverses_id uuid null`，否则按总数据字典第 2 节不得保留恒空列。表 1 至表 17 不带 `legal_entity_id`、不建行级策略，理由见第 0 节偏离二；表 18 至表 23 必带 `legal_entity_id` 并 ENABLE、FORCE RLS，复合外键同带法人，不适用该偏离。时间列一律 timestamptz，日期列一律 date，金额与吞吐等数值按基线第 3.5 节取 numeric(18,6) 或 numeric(9,6)。文本列一律 text 加 CHECK 长度约束，取值上限按基线第 11.2 节。

活动行唯一性的统一写法。凡需要保证同一作用域下至多一条活动记录的表，一律用哨兵值而非部分索引：结束时间列取 timestamptz not null default 'infinity'，唯一约束建在作用域键加该列上。理由是基线第 3.10 节禁止部分索引，且该写法在同一语句内即可完成开与闭，不需要额外的指针表，也不触发基线第 3.6 节禁止的 DELETE。

#### 3.1 表清单

表 1 platform_ops.deployment_records，部署记录，版本行仅允许受控关闭哨兵。
列：id、security_level、data_scope_tags、revision bigint not null、server_spec jsonb not null（CPU 核数、内存、磁盘型号与容量）、disk_capacity_floor_bytes bigint not null、resource_quota_frozen_ref text not null（认证报告编号）、rto_hours numeric(9,6) not null default 4.000000、rto_reestimated boolean not null default false、rto_reestimation_basis text null（CHECK 长度不超过 2000）、shard_pickup_sla_hours int null、dual_control_authorizers jsonb not null default '[]'、waf_frontend_configured boolean not null、waf_attestation_at timestamptz null、virus_scan_mode text not null CHECK in ('NONE','CUSTOMER_ICAP')、virus_scan_icap_url text null、data_volume_within_baseline boolean not null、certification_report_ref text null、drill_report_ref text null、notes text null、superseded_at timestamptz not null default 'infinity'、created_at、created_by。
约束：pk_deployment_records；ux_deployment_records_superseded_at (superseded_at)；ux_deployment_records_revision (revision)；ck_deployment_records_rto_positive CHECK (rto_hours > 0)；ck_deployment_records_shard_sla CHECK (shard_pickup_sla_hours is null or shard_pickup_sla_hours > 0)；ck_deployment_records_virus_scan CHECK ((virus_scan_mode = 'NONE' and virus_scan_icap_url is null) or (virus_scan_mode = 'CUSTOMER_ICAP' and virus_scan_icap_url is not null))。
索引：ix_deployment_records_created_at。
说明：shard_pickup_sla_hours 为空即该部署未约定分片取件时限，按规格第 13.4 章不得在交付材料中宣称 4 小时 RTO，该结论由 v_rpo_status 与门禁工装同时读取。`virus_scan_mode` 是部署必答事实且没有数据库默认值；CUSTOMER_ICAP 的 URL 还须由应用校验为 `icap` scheme 与回环 IP 字面量，NONE 时本记录直接驱动不可抑制降级、健康展示及诚实披露。

表 2 platform_ops.offsite_sinks，服务器之外落点，版本行仅允许受控关闭哨兵。
列：id、security_level、data_scope_tags、sink_kind text not null CHECK in ('LOCAL_DIR','NFS_SMB_MOUNT','OBJECT_STORAGE')、root_ref text not null、media_type text not null CHECK in ('ONLINE','OFFLINE','NONE')、rotation_period_minutes int null、writability text not null CHECK in ('WRITABLE','UNWRITABLE','UNKNOWN')、writability_changed_at timestamptz not null、req_online boolean not null、req_auto_write boolean not null、req_failure_detectable boolean not null、access_control_attested boolean not null default false、access_control_attested_at timestamptz null、access_control_evidence_ref text null、writer_identity_ref text null、restore_identity_ref text null、disposal_identity_ref text null、append_only_attested boolean not null default false、append_only_attested_at timestamptz null、append_only_evidence_ref text null、append_only_probe_at timestamptz null、append_only_probe_result text not null default 'UNKNOWN' CHECK in ('PASS','FAIL','UNKNOWN')、readback_throughput_mibps numeric(18,6) null、write_throughput_mibps numeric(18,6) null、throughput_measured_at timestamptz null、superseded_at timestamptz not null default 'infinity'、公共列。三个 identity_ref 只存经清洗的账户/角色标识，不存凭据或 secret ref。
约束：ux_offsite_sinks_superseded_at；ck_offsite_sinks_offline_rotation CHECK (media_type <> 'OFFLINE' or rotation_period_minutes is not null)；ck_offsite_sinks_none_kind CHECK (media_type <> 'NONE' or (req_online = false and req_auto_write = false and req_failure_detectable = false))；ck_offsite_sinks_identity_separation CHECK (media_type = 'NONE' or (writer_identity_ref is not null and restore_identity_ref is not null and disposal_identity_ref is not null and writer_identity_ref <> restore_identity_ref and writer_identity_ref <> disposal_identity_ref and restore_identity_ref <> disposal_identity_ref))；ck_offsite_sinks_append_attestation CHECK ((append_only_attested = true and append_only_attested_at is not null and append_only_evidence_ref is not null and append_only_probe_result = 'PASS') or append_only_attested = false)。
索引：ix_offsite_sinks_created_at。
说明：media_type 取 NONE 表示客户未配置任何服务器之外落点，此时该部署没有 RPO 承诺。

表 3 platform_ops.degradation_windows，降级与暴露窗口台账，可更新。本表由阶段 2 按 A-26 建立相同列与初始 3 项 Rust/SQL kind 闭集。唯一顺序固定为：Stage 14a0 先扩 Rust enum/serde/ledger 接收域为下列终态 21 项；090300/090350 分别追加两条 CHECK/三个索引但不改 kind；完整 pre-F-55 链到 `V20261023092500` 时才在同一数据库事务把 kind CHECK 从 3 扩为 21、把不可抑制 CHECK 从 4 扩为 5；迁移成功后才注册 `legal-entity-key-domain-coverage` 并允许第 4 至 21 项真实写库。不重建表、不增删列，也不修改 Stage 2 历史迁移。
列：id、security_level、data_scope_tags、row_version、kind text not null CHECK in（下列 21 个取值）、subject text null（CHECK 长度不超过 200，承载端口或能力的完整类型名，由阶段 2 建表时给出）、scope_key text not null（CHECK 长度不超过 200）、scope_legal_entity_id uuid null、scope_accounting_period_id uuid null、basis text not null（CHECK 长度不超过 2000）、detail jsonb not null default '{}'、opened_at timestamptz not null、closed_at timestamptz not null default 'infinity'、closing_condition text not null（CHECK 长度不超过 2000）、is_suppressible boolean not null、suppressed_until timestamptz null、created_at、created_by、updated_at、updated_by。
kind 取值：OFFSITE_SINK_NOT_CONFIGURED、WRITER_NOT_IN_SERVICE、PORT_NOT_IMPLEMENTED、OFFSITE_SINK_OFFLINE_MEDIA_RPO_DEGRADED、WAL_ARCHIVE_WRITEOUT_OVERDUE_OR_FAILED、ATTACHMENT_INCREMENTAL_WRITEOUT_OVERDUE_OR_FAILED、ATTACHMENT_BOOTSTRAP_WINDOW_EXCEEDED、ATTACHMENT_RPO_NOT_YET_ACHIEVED、AUDIT_EVIDENCE_WRITEOUT_OVERDUE_OR_FAILED、PORTAL_WAF_NOT_CONFIGURED、AUDIT_ANCHOR_OVERDUE、OFFSITE_COPY_PROTECTION_MISSING、ARCHIVE_SLOT_RETENTION_WARNING、ARCHIVE_CHAIN_BROKEN、RECON_RUN_UNFINISHED、PERIOD_CLOSE_ACCEPTANCE_REJECTED、AUTHZ_SNAPSHOT_CHECKSUM_MISMATCH、CUSTOM_OBJECT_DDL_INCONSISTENT、REPLICATION_CROSSCHECK_NO_RESULT、VIRUS_SCANNER_NOT_AVAILABLE、LEGAL_ENTITY_KEY_DOMAIN_UNAVAILABLE，共 21 个。阶段 2 首次只放行前三项；Stage 14a0 让后续阶段可编译并用 mock/contract 测试表达其余 18 项，但 `V20261023092500` 之前任何真实数据库写入、集成测试或“已开窗”声明都必须失败。`PORT_NOT_IMPLEMENTED` 是跨模块与平台能力缺位的唯一登记形态，由缺位期间的调用方开窗、由被调方所在阶段注入实现后关窗，端口名记在 subject 列；AUTHZ_SNAPSHOT_CHECKSUM_MISMATCH 与 CUSTOM_OBJECT_DDL_INCONSISTENT 的 provider/触发契约可分别由阶段 4/13 实现，真实 PG 写测统一在 092500 后验收；REPLICATION_CROSSCHECK_NO_RESULT 由本阶段在连续第二个 `NO_RESULT` 时开窗；VIRUS_SCANNER_NOT_AVAILABLE 在模式为 NONE 时常开，在 CUSTOMER_ICAP 不可用时开、恢复时关；LEGAL_ENTITY_KEY_DOMAIN_UNAVAILABLE 在任一启用法人没有可解引用且状态有效的当前 key domain、KMS/HSM 解封失败或 EPS1 migration receipt 无法验证时按该法人开窗，恢复同一 provider、验证 receipt 并完成一次真实解封后自动关窗，严禁回退 file/plaintext provider。
约束：ux_degradation_windows_kind_scope_closed 是 PostgreSQL 16 的 `UNIQUE NULLS NOT DISTINCT (kind, subject, scope_legal_entity_id, scope_accounting_period_id, closed_at)`，与 ck_degradation_windows_open_order CHECK (closed_at > opened_at) 两条由阶段 2 建表时交付；`NULLS NOT DISTINCT` 不得省略，否则部署级窗口三个可空作用域会绕过唯一性。前者保证同一 kind 与同一 subject 在同一法人与会计期间作用域下至多一条活动条目，从而使同一 kind 下多个端口可同时开窗，本阶段不改写这两条；scope_key 保留为展示用的作用域说明列，不进该唯一约束。本阶段追加 ck_degradation_windows_le_required CHECK (kind not in ('RECON_RUN_UNFINISHED','PERIOD_CLOSE_ACCEPTANCE_REJECTED') or (scope_legal_entity_id is not null and scope_accounting_period_id is not null)) 与 ck_degradation_windows_not_suppressible CHECK (kind not in ('OFFSITE_SINK_NOT_CONFIGURED', 'OFFSITE_COPY_PROTECTION_MISSING', 'WRITER_NOT_IN_SERVICE', 'VIRUS_SCANNER_NOT_AVAILABLE', 'LEGAL_ENTITY_KEY_DOMAIN_UNAVAILABLE') or (is_suppressible = false and suppressed_until is null)) 两条，后者护住未配置落点、离站副本防删保护缺失、写出进程未投入运行、病毒扫描器不可用与法人密钥域不可用五类。五类均不得在维护窗口内人工静音，触发条件消除后由系统自动闭合；NONE 病毒扫描模式本身不具备关窗条件，只能经受控部署变更切到健康的 CUSTOMER_ICAP 后关窗，密钥域不可用只能经原 KMS/HSM 路径恢复并验证，不得由 fallback 关窗。
索引：全部索引由本阶段追加，即 ix_degradation_windows_kind_opened_at；ix_degradation_windows_closed_at_opened_at；ix_degradation_windows_scope_legal_entity_id_opened_at。
说明：归档通道暂停不单列 kind，按规格第 15.3 章含在 ARCHIVE_CHAIN_BROKEN 的同一个暴露窗口内，其 detail 内以 sub_state 取值 SUSPENDED 标注；这与规格“含落点持续不可写期间暂不重建复制槽的那一段”一致，窗口自断点起算，只在新的全量基础备份写出并通过自动校验时闭合。

表 4 platform_ops.writeout_runs，写出批次，仅追加。
列：id、security_level、data_scope_tags、channel text not null CHECK in ('WAL_ARCHIVE','ATTACHMENT_INCREMENTAL','ATTACHMENT_FULL','AUDIT_EVIDENCE','FULL_BACKUP','CONFIG_BUNDLE','ATTACHMENT_BOOTSTRAP')、writer_process text not null CHECK in ('archive-writer','backup-writer')、sink_id uuid not null、period_seq bigint not null、started_at timestamptz not null、finished_at timestamptz not null、outcome text not null CHECK in ('OK','FAILED','ABORTED')、bytes_written bigint not null default 0、object_count int not null default 0、failure_category text null CHECK in ('SINK_UNWRITABLE','ENCRYPTION','CHECKSUM','SOURCE_READ','QUOTA','OTHER')、last_error text null、report_id uuid not null、created_at、created_by。本表没有业务冲销语义，不带 `reverses_id`。
约束：ux_writeout_runs_report_id (report_id)，是 IPC 上报的幂等键；ux_writeout_runs_channel_period_seq (channel, period_seq)。
索引：ix_writeout_runs_channel_started_at；ix_writeout_runs_outcome_started_at。

表 5 platform_ops.attachment_watermarks，附件正文写出点水位，仅追加。
列：id、security_level、data_scope_tags、watermark_at timestamptz not null、pending_object_count int not null、oldest_pending_committed_at timestamptz null、bootstrap_state text not null CHECK in ('NOT_STARTED','RUNNING','DONE')、bootstrap_remaining_bytes bigint not null default 0、manifest_ref text not null、sink_id uuid not null、advanced_at timestamptz not null、report_id uuid not null、created_at、created_by。
约束：ux_attachment_watermarks_report_id。
索引：ix_attachment_watermarks_watermark_at；ix_attachment_watermarks_advanced_at。

表 6 platform_ops.backup_sets，备份集，可更新，走状态机。
列：id、security_level、data_scope_tags、row_version、kind text not null CHECK in ('DAILY_FULL','CHAIN_REBUILD_BASELINE','CONFIG_BUNDLE','ATTACHMENT_FULL')、state text not null CHECK in ('PLANNED','RUNNING','WRITTEN','VERIFIED','VERIFY_FAILED','ABORTED','DISPOSED')、sink_id uuid not null、writeout_run_id uuid null、started_at timestamptz null、written_at timestamptz null、verification_concluded_at timestamptz null、verified_at timestamptz null、aborted_at timestamptz null、disposed_at timestamptz null、disposed_from_state text null CHECK in ('VERIFIED','VERIFY_FAILED','ABORTED')、disposal_certificate_ref text null、bytes bigint null、base_lsn text null、backup_label_ref text null、manifest_ref text null、encryption_key_ref text not null、spill_peak_bytes bigint null、abort_reason text null CHECK in ('SPILL_LIMIT','SINK_UNWRITABLE','SOURCE_ERROR','SUPERSEDED')、公共列。
约束：第 3.1.1 节冻结七态逐态形状、单向边、写出回执及精确校验方法图；所有跨表谓词由 `platform_ops.assert_backup_evidence_graph_consistent()` 的 DEFERRABLE INITIALLY DEFERRED 约束触发器承担，不保留原先只有 `VERIFIED => verified_at` 的弱 CHECK。
索引：ix_backup_sets_kind_started_at；ix_backup_sets_state_started_at。

表 7 platform_ops.backup_runner_slot，备份串行槽，单行，可更新。
列：id uuid not null（固定常量）、current_backup_set_id uuid null、row_version、updated_at、updated_by、security_level、data_scope_tags、created_at、created_by。
约束：ck_backup_runner_slot_singleton CHECK (id = '00000000-0000-0000-0000-0000000000b1'::uuid)；`current_backup_set_id` 真实外键指向 `backup_sets(id) ON DELETE RESTRICT DEFERRABLE INITIALLY DEFERRED`，并由证据图强制“非空当且仅当所指备份处于 RUNNING”。
说明：每日全量备份与断链后重建基线备份的串行由该行的乐观锁保证，避免依赖单副本这一前提。

表 8 platform_ops.backup_verifications，备份自动校验结论，仅追加。
列：id、security_level、data_scope_tags、backup_set_id uuid not null、method text not null CHECK in ('MANIFEST_CHECKSUM','DECRYPT_READBACK','PG_VERIFYBACKUP','ATTACHMENT_CHECKSUM')、started_at timestamptz not null、finished_at timestamptz not null、outcome text not null CHECK in ('PASS','FAIL')、bytes_read bigint not null、mismatched_object_count int not null default 0、detail jsonb not null default '{}'、report_id uuid not null、created_at、created_by。
约束：`backup_set_id` 外键指向 `backup_sets(id) ON DELETE RESTRICT DEFERRABLE INITIALLY DEFERRED`；ux_backup_verifications_report_id；ux_backup_verifications_backup_method (backup_set_id,method)；结束不早于开始、两个计数非负且 PASS 必须 mismatched_object_count=0。索引：ix_backup_verifications_backup_set_id_started_at。

表 9 platform_ops.archive_channel，归档通道状态机，单行，可更新。
列：id uuid not null（固定常量）、state text not null CHECK in ('HEALTHY','RETENTION_WARNING','SLOT_INVALIDATED','REBUILDING','SUSPENDED')、slot_name text not null、slot_active boolean not null、confirmed_flush_lsn text null、broken_at timestamptz null、break_cause text null CHECK in ('SLOT_WAL_LIMIT','WRITER_STOPPED','WRITER_NOT_ADVANCING','SINK_UNWRITABLE')、rebuild_backup_set_id uuid null、restored_at timestamptz null、last_transition_id uuid null、replication_check_last_outcome text null、replication_check_last_at timestamptz null、replication_check_no_result_streak smallint not null default 0、replication_check_last_error_code text null、row_version、公共列。
约束：ck_archive_channel_singleton CHECK (id = '00000000-0000-0000-0000-0000000000a1'::uuid)；ck_archive_channel_broken CHECK (state not in ('SLOT_INVALIDATED','REBUILDING','SUSPENDED') or (broken_at is not null and break_cause is not null))；ck_archive_channel_replication_check_outcome CHECK (replication_check_last_outcome is null or replication_check_last_outcome in ('MATCHED','MISMATCHED','NO_RESULT'))；ck_archive_channel_replication_check_streak CHECK (replication_check_no_result_streak >= 0)。090900 的唯一 seed 精确为 id 固定常量、row_version=1、state=HEALTHY、slot_name=`ep_archive_slot`、slot_active=true、confirmed_flush_lsn/broken_at/break_cause/rebuild_backup_set_id/restored_at/last_transition_id/replication_check_last_outcome/replication_check_last_at/replication_check_last_error_code 全空、replication_check_no_result_streak=0；其余公共列取本节固定系统主体与数据库时钟。

表 10 platform_ops.archive_channel_transitions，通道版本证据，仅追加。
列：id、security_level、data_scope_tags、archive_channel_id uuid not null（固定取单例 id）、transition_kind text not null CHECK in ('STATE_CHANGE','OBSERVATION')、from_row_version bigint not null、to_row_version bigint not null、from_state text not null、to_state text not null、to_slot_name text not null、to_slot_active boolean not null、to_confirmed_flush_lsn text null、to_broken_at timestamptz null、to_break_cause text null CHECK in ('SLOT_WAL_LIMIT','WRITER_STOPPED','WRITER_NOT_ADVANCING','SINK_UNWRITABLE')、to_rebuild_backup_set_id uuid null、to_restored_at timestamptz null、to_replication_check_last_outcome text null CHECK in ('MATCHED','MISMATCHED','NO_RESULT')、to_replication_check_last_at timestamptz null、to_replication_check_no_result_streak smallint not null、to_replication_check_last_error_code text null、cause text not null、occurred_at timestamptz not null、detail jsonb not null default '{}'、report_id uuid not null、created_at、created_by。
约束：`to_row_version=from_row_version+1` 且 streak 非负；ux_archive_channel_transitions_report_id；ux_archive_channel_transitions_channel_to_version (archive_channel_id,to_row_version)；ux_archive_channel_transitions_channel_id_id (archive_channel_id,id)。`archive_channel` 以 `(id,last_transition_id) -> archive_channel_transitions(archive_channel_id,id) ON DELETE RESTRICT DEFERRABLE INITIALLY DEFERRED` 构成同单例反向指针；本表的 archive_channel_id 再 FK 到单例 id，`to_rebuild_backup_set_id` 非空时以真实 DEFERRABLE FK 指向备份集。每条证据保存 archive_channel 除 `last_transition_id` 与公共审计列外全部业务列的 after-image，不能只把可重放字段塞进无类型 detail；延迟图逐版本重放必须连续且最后一条 after-image 逐列等于当前单例。索引：ix_archive_channel_transitions_occurred_at。

表 11 platform_ops.replication_reports，写出进程的复制生命周期上报，仅追加。
列：id、security_level、data_scope_tags、writer_process text not null CHECK in ('archive-writer','backup-writer')、db_role text not null CHECK in ('ep_archiver','ep_backuper')、report_kind text not null CHECK in ('CONN_ESTABLISHED','CONN_CLOSED','SLOT_CREATED','SLOT_INVALIDATED','BASEBACKUP_STARTED','BASEBACKUP_FINISHED')、slot_name text null、backend_pid int null、occurred_at timestamptz not null、outcome text not null CHECK in ('OK','FAILED')、report_id uuid not null、spooled boolean not null default false、created_at、created_by。
约束：ux_replication_reports_report_id。索引：ix_replication_reports_occurred_at；ix_replication_reports_db_role_occurred_at。
说明：spooled 为真表示该条是 core-server 不可用期间在写出进程本地暂存后补写的，复制生命周期的时序一律按 occurred_at 而非写入时刻判读。

表 12 platform_ops.wal_retention_samples，复制槽本机保留量采样，可按期清理。
列：id、security_level、data_scope_tags、sampled_at timestamptz not null、slot_name text not null、retained_bytes bigint not null、max_slot_wal_keep_bytes bigint not null、retention_ratio numeric(9,6) not null、pg_wal_bytes bigint not null、created_at、created_by。
索引：ix_wal_retention_samples_sampled_at。保留 90 天，超期按基线第 3.6 节允许的过期指标快照清理路径删除。

表 13 platform_ops.capacity_samples，磁盘容量水位采样，可按期清理。
列：id、security_level、data_scope_tags、sampled_at、component text not null CHECK in ('ATTACHMENT_CURRENT','ATTACHMENT_HISTORY','DB_DATA','ARCHIVE_LOCAL','BASEBACKUP_SPILL','SEARCH_AND_TEMP')、used_bytes bigint not null、floor_bytes bigint not null、ratio numeric(9,6) not null、created_at、created_by。
索引：ix_capacity_samples_sampled_at。保留 400 天，覆盖年度容量复核。

表 14 platform_ops.key_recovery_materials，密钥恢复材料登记，可更新，不存材料本身。
列：id、security_level（固定 40）、data_scope_tags、row_version、material_kind text not null CHECK in ('TENANT_ROOT','LEGAL_ENTITY_KEY_DOMAIN','DEPLOYMENT_BACKUP_ENCRYPTION_KEY')、scope_ref text null、carrier text not null CHECK in ('BUILTIN_KMS','CUSTOMER_HSM')、shard_count smallint not null、shard_locations jsonb not null、dual_control_authorizers jsonb not null、last_verified_at timestamptz null、next_verification_due_on date not null、verification_method text not null、stored_with_protected_copy boolean not null default false、公共列。
约束：ck_key_recovery_materials_shards CHECK (shard_count >= 2)；ck_key_recovery_materials_not_colocated CHECK (stored_with_protected_copy = false)，落实规格第 13.4 章“不得与其保护的副本存放于同一落点”。
索引：ix_key_recovery_materials_next_verification_due_on。

表 15 platform_ops.key_recovery_verifications，核验结论，仅追加。
列：id、security_level、data_scope_tags、key_recovery_material_id uuid not null、performed_at、performed_by_party text not null CHECK in ('CUSTOMER_OPS','CUSTOMER_PER_CONTRACT')、outcome text not null CHECK in ('PASS','FAIL')、isolated_env_ref text not null、approval_ref text not null、report_ref text not null、created_at、created_by。
索引：ix_key_recovery_verifications_key_recovery_material_id_performed_at。

表 16 platform_ops.recovery_drills，恢复演练与真实恢复登记，可更新。
列：id、security_level、data_scope_tags、row_version、drill_kind text not null CHECK in ('WHOLE_MACHINE_RECOVERY','KEY_MATERIAL_ISOLATED_RECOVERY','PRODUCTION_RECOVERY')、backup_selection text not null CHECK in ('LATEST_VERIFIED','RETENTION_TAIL')、state text not null default 'RUNNING' CHECK in ('RUNNING','PASSED','FAILED')、attempt_no int not null、window_started_at timestamptz not null、window_ended_at timestamptz null、sink_id uuid not null、backup_set_id uuid not null、backup_verified_at_at_start timestamptz not null、retention_days_at_start smallint not null、sink_kind_at_drill text not null、readback_throughput_mibps numeric(18,6) null、rto_seconds bigint null、rpo_db_seconds bigint null、rpo_attachment_seconds bigint null、shard_pickup_seconds bigint null、attachment_check_total int null、attachment_check_failed int null、attachment_check_seconds bigint null、invariant_check_batches int null、invariant_check_max_batch_seconds bigint null、invariant_check_total_seconds bigint null、invariant_check_mem_peak_bytes bigint null、invariant_check_tempfile_peak_bytes bigint null、decrypt_seconds bigint null、outcome text null CHECK in ('PASS','FAIL')、failure_stage text null CHECK in ('READBACK','KEY_SHARD_PICKUP','DECRYPT','ATTACHMENT_CHECK','INVARIANT_CHECK','RPO_EVALUATION','RTO_EVALUATION','OTHER')、failure_code text null、report_ref text null、公共列。
约束：ck_recovery_drills_attempt CHECK (attempt_no >= 1)；retention_days_at_start >= 1；ux_recovery_drills_kind_selection_attempt (drill_kind,backup_selection,attempt_no) 对三类一律生效；`sink_id` 与 `backup_set_id` 分别真实外键指向 `offsite_sinks(id)` 与 `backup_sets(id) ON DELETE RESTRICT DEFERRABLE INITIALLY DEFERRED`。`attempt_no` 由服务按 `(drill_kind,backup_selection)` 单调分配；真实恢复同样逐次编号，不使用部分索引，也不从 UUID 推导整数。
索引：ix_recovery_drills_drill_kind_window_started_at。
说明：shard_pickup_seconds 单独留证且不计入 rto_seconds，按规格第 13.4 章与附录 A.6；attachment_check_seconds、decrypt_seconds 与 invariant_check_total_seconds 三项计入 rto_seconds。

表 17 platform_ops.alert_suppressions，告警抑制与静音，仅追加。
列：id、security_level、data_scope_tags、degradation_window_id uuid not null、action text not null CHECK in ('SUPPRESS','UNSUPPRESS')、acted_at timestamptz not null、acted_by uuid not null、until_at timestamptz null、reason text not null（CHECK 长度不超过 2000）、approval_ref text null、created_at、created_by。
索引：ix_alert_suppressions_degradation_window_id_acted_at。

表 18 platform_ops.data_migration_batches，历史数据迁移批次，可更新，带法人行级策略。
列：id、legal_entity_id、security_level（固定不低于 30）、data_scope_tags、row_version、batch_no text not null、source_kind text not null CHECK in ('XLSX_CSV','ODBC','FILE_MANIFEST','HTTPS_API')、source_system_ref text not null、source_schema_fingerprint bytea not null、source_readonly_test_ref text null、template_code text not null、template_version text not null、template_sha256 bytea not null、status text not null CHECK in ('DRAFT','APPROVED','TRIAL_RUNNING','TRIAL_FAILED','TRIAL_PASSED','SOURCE_FROZEN','APPLYING','DELTA_CATCHUP','RECONCILING','READY_FOR_CUTOVER','CUTOVER_COMPLETED','REVERSAL_PENDING','REVERSED','CANCELLED')、task_available_at timestamptz not null default now()、task_locked_by text null、task_locked_until timestamptz null、task_attempts int not null default 0、ledger_scope jsonb not null default '[]'、warehouse_scope jsonb not null default '[]'、required_reconciliation_keys jsonb not null、source_module_codes text[] not null、window_starts_at timestamptz not null、window_ends_at timestamptz not null、data_owner_id uuid not null、customer_finance_owner_id uuid not null、content_version bigint not null default 1、approval_content_hash bytea not null、current_run_no int not null default 0、trial_pass_count smallint not null default 0、trial_nonconvergent_count smallint not null default 0、source_frozen_at timestamptz null、source_readonly_evidence_ref text null、delta_watermark text null、source_manifest_sha256 bytea null、source_record_count bigint null、trial_report_ref text null、reconciliation_digest bytea null、final_reconciliation_report_ref text null、cutover_content_hash bytea null、cutover_at timestamptz null、reversal_batch_ref text null、reversal_reason text null、reversal_content_hash bytea null、cancelled_from_status text null CHECK in ('DRAFT','APPROVED','TRIAL_FAILED','TRIAL_PASSED')、cancelled_at timestamptz null、cancel_reason text null、公共列。旧 `data_owner_approval_ref`、`module_owner_approval_refs` 与 `cutover_decision_ref` 三列删除，批准事实只来自表 22。
约束：ux_data_migration_batches_le_batch_no (legal_entity_id,batch_no) 与 ux_data_migration_batches_le_id (legal_entity_id,id)；`(legal_entity_id,data_owner_id)` 与 `(legal_entity_id,customer_finance_owner_id)` 分别真实复合外键指向 `platform_authz.user_legal_entity_grants(legal_entity_id,user_id) ON DELETE RESTRICT`；template_sha256、source_schema_fingerprint、approval_content_hash、source_manifest_sha256、reconciliation_digest、cutover_content_hash、reversal_content_hash 七个 bytea 摘要非空时逐个固定 32 字节；content_version >= 1、current_run_no/task_attempts/trial 两计数均非负；窗口结束晚于开始；租约两列同空同非空；`source_module_codes` 必须非空、按数据库 C 排序、去重且仅取第 4.12 节 11 个模块码；`required_reconciliation_keys` 是按 `(check_kind,scope_key)` 排序去重的非空 JSON 数组且九类 check_kind 各至少一项。source_schema_fingerprint 从已验签模板的同名字段规范化为 SHA-256 后落库，运行期每次连接源端都重新计算并逐字节比较。`approval_content_hash` 固定为 RFC 8785 规范 JSON的 SHA-256，输入精确包含法人、批次 id/no、content_version、来源类型及清洗后的来源引用、source_schema_fingerprint、source_readonly_test_ref、模板 code/version/hash、两类 scope、required_reconciliation_keys、source_module_codes、窗口、data_owner_id 与 customer_finance_owner_id；DRAFT 中任一输入改变必须在同一 UPDATE 把 content_version 精确加一并重算 hash，进入 APPROVED 后这些输入逐列不可变。source_readonly_test_ref 在首次发起批准前由只读负测的不可变报告引用写入，APPROVED 及其后必非空。逐态证据形状与批准计数由第 3.1.2 节延迟图承担。
索引：ix_data_migration_batches_le_status_created_at；ix_data_migration_batches_window_ends_at；ix_data_migration_batches_task_claim (status, task_available_at, task_locked_until)。

表 19 platform_ops.data_migration_records，逐来源记录的错误队列与追溯台账，可更新，带法人行级策略；不保存来源原文、附件正文、访问凭据或可逆的来源定位值。
列：id、legal_entity_id、security_level、data_scope_tags、row_version、batch_id uuid not null、run_no int not null、chunk_no int not null、record_seq bigint not null、module_code text not null、object_type text not null、source_locator_sha256 bytea not null、source_record_sha256 bytea not null、mapped_security_level smallint null、mapped_key_domain_id uuid null、mapped_retention_policy_code text null、target_object_type text null、target_id uuid null、target_record_sha256 bytea null、apply_receipt_id uuid null、reversal_receipt_id uuid null、status text not null CHECK in ('QUEUED','VALIDATED','APPLIED','FAILED','REVERSED')、error_code text null、sanitized_error text null、applied_at timestamptz null、公共列。
约束：ux_data_migration_records_batch_run_object_source (legal_entity_id,batch_id,run_no,module_code,object_type,source_locator_sha256)；ux_data_migration_records_le_id (legal_entity_id,id)；ux_data_migration_records_le_batch_id (legal_entity_id,batch_id,id)；普通唯一键 `ux_data_migration_records_target_reservation (legal_entity_id,target_object_type,target_id)`（NULL 行不相互冲突）；`(legal_entity_id,batch_id) -> data_migration_batches(legal_entity_id,id)` 长复合外键。092600 为 `platform_core.key_domains` 补 `(legal_entity_id,id)` 候选键，并建立 `(legal_entity_id,mapped_key_domain_id) -> platform_core.key_domains(legal_entity_id,id) ON DELETE RESTRICT DEFERRABLE INITIALLY DEFERRED`；映射三列必须全空或全非空，非空时 mapped_security_level 只取 10/20/30/40、mapped_retention_policy_code 长度 1 至 64。run_no、chunk_no、record_seq 均大于 0，module_code/object_type 组合必须恰为第 4.12 节 25 个封闭值之一；三个 SHA-256 列各为 32 字节，target_record_sha256 允许空。第 3.1.2 节冻结五态形状与单向边，并以 `(legal_entity_id,batch_id,id,apply_receipt_id)` 与 `(legal_entity_id,batch_id,id,reversal_receipt_id)` 两条 nullable 长复合外键分别指向 `data_migration_writer_receipts(legal_entity_id,batch_id,record_id,id)` 候选键，均 ON DELETE RESTRICT、DEFERRABLE INITIALLY DEFERRED；不再仅凭任意 target 三元组把记录判为 APPLIED。`target_object_type,target_id` 是 VALIDATED 时由服务端按第 4.12.1 节 catalog 写入的目标根预留，不是 owner 写完后的自报结果：target type 必须等于 catalog 固定 relation，target id 必须为本次新生成 UUIDv7；状态转入 VALIDATED 的提交点，静态分支必须证明对应同法人 relation 尚无该 id。预留一旦形成逐列不可变，APPLY writer 必须使用该 id 建根，因此不需要给 25 张异构目标表伪造统一 migration_batch_id 列。
索引：ix_data_migration_records_le_batch_status_record_seq；ix_data_migration_records_le_batch_object_type；ix_data_migration_records_le_error_code。

表 20 platform_ops.data_migration_reconciliations，迁移对账结论，仅追加，带法人行级策略。
列：id、legal_entity_id、security_level、data_scope_tags、batch_id uuid not null、run_no int not null、check_kind text not null CHECK in ('COUNT','AMOUNT','RELATIONSHIP','ATTACHMENT','HASH','DEBIT_CREDIT_BALANCE','INVENTORY_CONSERVATION','OPENING_CONTINUITY','SECURITY_ASSIGNMENT')、scope_key text not null、source_value jsonb not null、target_value jsonb not null、difference_value jsonb not null、outcome text not null CHECK in ('PASS','FAIL','APPROVED_DIFFERENCE')、known_difference_id uuid null、report_ref text not null、checked_at timestamptz not null、公共列。
约束：ux_data_migration_reconciliations_batch_run_kind_scope (legal_entity_id,batch_id,run_no,check_kind,scope_key)；`(legal_entity_id,batch_id)` 指向批次，`(legal_entity_id,batch_id,known_difference_id)` 在非空时指向表 21 同批次差异，均 ON DELETE RESTRICT DEFERRABLE INITIALLY DEFERRED；APPROVED_DIFFERENCE 与差异引用同真同假；DEBIT_CREDIT_BALANCE、INVENTORY_CONSERVATION、OPENING_CONTINUITY 与 SECURITY_ASSIGNMENT 四类只能 PASS，不能豁免。
索引：ix_data_migration_reconciliations_le_batch_outcome。

表 21 platform_ops.data_migration_known_differences，规格第 7.10 章已知差异的审批台账，可更新并走封闭审批状态机，带法人行级策略。差异事实与当前决定留在本行，批准、拒绝与撤销的不可变证据只写表 22。
列：id、legal_entity_id、security_level、data_scope_tags、row_version、batch_id uuid not null、module_code text not null、category text not null CHECK in ('CLOSED_PERIOD_SOURCE_IMBALANCE_OR_INCOMPLETE','CLOSED_HISTORY_SETTLED_OR_CLOSED','NONCRITICAL_MISSING_HISTORY_DETAIL','NAMED_MIGRATION_BALANCING_ENTRY')、ledger_or_warehouse_scope text not null、source_document_scope text not null、amount numeric(18,2) null、quantity numeric(24,6) null、cause text not null、cannot_zero_reason text not null、proposal_ref text not null、data_owner_id uuid not null、module_owner_id uuid not null、finance_owner_id uuid not null、content_version bigint not null default 1、approval_content_hash bytea not null、decision text not null default 'PROPOSED' CHECK in ('PROPOSED','APPROVED','REJECTED','REVOKED')、decided_at timestamptz null、公共列。旧 `decision_ref` 与三项任意文本 approval_ref 删除。
约束：ux_data_migration_known_differences_le_id (legal_entity_id,id)；ux_data_migration_known_differences_le_batch_id (legal_entity_id,batch_id,id)；同法人 batch 长外键；`(legal_entity_id,data_owner_id)`、`(legal_entity_id,module_owner_id)`、`(legal_entity_id,finance_owner_id)` 三条真实复合外键分别指向 `platform_authz.user_legal_entity_grants(legal_entity_id,user_id) ON DELETE RESTRICT`；module_code 只取第 4.12 节 11 个模块码且必须属于所指批次的 source_module_codes；金额或数量至少一项非空；content_version >= 1 且 approval_content_hash 固定 32 字节。该 hash 是法人、批次、差异 id、content_version、module_code、category、两类 scope、amount/quantity、cause、cannot_zero_reason、proposal_ref 与三名 owner id 的 RFC 8785/SHA-256；PROPOSED 中任一输入改变必须在同一 UPDATE 把 content_version 精确加一并重算 hash，离开 PROPOSED 后输入不可变。状态只允许 PROPOSED → APPROVED|REJECTED 与 APPROVED → REVOKED；REJECTED、REVOKED 终态；逐态批准证据形状见第 3.1.2 节。
索引：ix_data_migration_known_differences_le_batch_decision。

表 22 platform_ops.data_migration_approval_evidences，迁移批准事实，仅追加，带法人行级策略。
列：id、legal_entity_id、security_level（固定不低于 30）、data_scope_tags、batch_id uuid not null、known_difference_id uuid null、subject_difference_id uuid GENERATED ALWAYS AS (coalesce(known_difference_id,'00000000-0000-0000-0000-000000000000'::uuid)) STORED、phase text not null CHECK in ('BATCH_APPROVAL','KNOWN_DIFFERENCE_DECISION','CUTOVER_APPROVAL','REVERSAL_APPROVAL')、decision text not null CHECK in ('APPROVED','REJECTED','REVOKED')、reauth_purpose text GENERATED ALWAYS AS (case when phase='KNOWN_DIFFERENCE_DECISION' and decision='REVOKED' then 'KNOWN_DIFFERENCE_REVOCATION' else phase end) STORED、approver_kind text not null CHECK in ('DATA_OWNER','MODULE_OWNER','FINANCE_OWNER','SECOND_APPROVER')、module_code text null、subject_module_code text GENERATED ALWAYS AS (coalesce(module_code,'-')) STORED、approver_role_id uuid not null、approver_role_code text not null、approver_role_grant_id uuid not null、approver_grant_effective_from date not null、approver_grant_effective_to_at_decision date null、content_version bigint not null、content_hash bytea not null、process_instance_id uuid not null、process_task_id uuid not null、reauth_challenge_id uuid not null、definition_id uuid not null、definition_code text not null、definition_version int not null、definition_hash text not null、submitted_by uuid not null、submitted_at timestamptz not null、decided_by uuid not null、decided_at timestamptz not null、reverses_evidence_id uuid null、created_at、created_by。
约束：ux_data_migration_approval_evidences_le_id (legal_entity_id,id)；ux_data_migration_approval_evidences_subject_role_decision `UNIQUE NULLS NOT DISTINCT (legal_entity_id,batch_id,phase,known_difference_id,content_version,approver_kind,module_code,decision)`；ux_data_migration_approval_evidences_reversal_parent (legal_entity_id,batch_id,phase,subject_difference_id,content_version,approver_kind,subject_module_code,id)。`(legal_entity_id,batch_id) -> data_migration_batches(legal_entity_id,id)`、`(legal_entity_id,batch_id,known_difference_id) -> data_migration_known_differences(legal_entity_id,batch_id,id)`、`(legal_entity_id,submitted_by) -> platform_authz.user_legal_entity_grants(legal_entity_id,user_id)`、`(legal_entity_id,decided_by) -> platform_authz.user_legal_entity_grants(legal_entity_id,user_id)` 与 `reauth_challenge_id -> platform_core.reauth_challenges(id)` 全部 ON DELETE RESTRICT。092600 还在 `platform_flow.process_instances` 建 `(legal_entity_id,id,definition_id,definition_code,definition_version)` 候选键、在 `platform_flow.process_tasks` 建 `(legal_entity_id,process_instance_id,id)` 候选键、在 `platform_flow.process_definitions` 建 `(legal_entity_id,id,code,version,definition_hash)` 候选键、在 `platform_authz.roles` 建 `(legal_entity_id,id,code)` 候选键，并在 `platform_authz.user_role_grants` 建 `(legal_entity_id,user_id,role_id,id,effective_from)` 候选键。表 22 的五条真实长 FK 精确为 `(legal_entity_id,process_instance_id,definition_id,definition_code,definition_version) -> platform_flow.process_instances(legal_entity_id,id,definition_id,definition_code,definition_version)`、`(legal_entity_id,process_instance_id,process_task_id) -> platform_flow.process_tasks(legal_entity_id,process_instance_id,id)`、`(legal_entity_id,definition_id,definition_code,definition_version,definition_hash) -> platform_flow.process_definitions(legal_entity_id,id,code,version,definition_hash)`、`(legal_entity_id,approver_role_id,approver_role_code) -> platform_authz.roles(legal_entity_id,id,code)` 与 `(legal_entity_id,decided_by,approver_role_id,approver_role_grant_id,approver_grant_effective_from) -> platform_authz.user_role_grants(legal_entity_id,user_id,role_id,id,effective_from)`；五条均 ON DELETE RESTRICT、DEFERRABLE INITIALLY DEFERRED，分别锁死实例定义、任务归属、发布定义快照、角色身份和作出决定者的角色授权。`approver_grant_effective_to_at_decision` 保存插证时授权行 effective_to 的快照，不参与父 FK，避免日后正常到期/撤权改写历史事实；受控函数与延迟图仍须在插入时逐列比对该快照并要求 `(decided_at AT TIME ZONE 'Asia/Shanghai')::date` 落在 `[effective_from,effective_to]`（空上界为无穷）。REVOKED 以 `(legal_entity_id,batch_id,phase,subject_difference_id,content_version,approver_kind,subject_module_code,reverses_evidence_id)` 真实长自 FK 指向上述候选键，避免 nullable 组合让 FK 静默跳过，延迟图再强制父 decision=APPROVED 与一一撤销。content_hash 固定 32 字节、definition_hash 为 64 位小写十六进制、content/definition version 为正、submitted_by <> decided_by；module_code 非空当且仅当 approver_kind=MODULE_OWNER；known_difference_id 非空当且仅当 phase=KNOWN_DIFFERENCE_DECISION；REVOKED 只允许该 phase 且必须有 reverses_evidence_id，其他行必须为空。

表 23 platform_ops.data_migration_writer_receipts，模块权威写入者效果回执，仅追加，带法人行级策略。
列：id、legal_entity_id、security_level（固定不低于 30）、data_scope_tags、batch_id uuid not null、record_id uuid not null、run_no int not null、module_code text not null、object_type text not null、effect_kind text not null CHECK in ('APPLY','REVERSE')、target_object_type text not null、target_id uuid not null、target_record_sha256 bytea not null、writer_contract_version int not null、effect_sha256 bytea not null、idempotency_key text not null、owner_effect_at timestamptz not null、reverses_receipt_id uuid null、created_at、created_by。
约束：ux_data_migration_writer_receipts_le_id (legal_entity_id,id)；ux_data_migration_writer_receipts_record_effect (legal_entity_id,record_id,effect_kind)；ux_data_migration_writer_receipts_le_batch_record_id (legal_entity_id,batch_id,record_id,id)；`(legal_entity_id,batch_id) -> data_migration_batches(legal_entity_id,id)`、`(legal_entity_id,batch_id,record_id) -> data_migration_records(legal_entity_id,batch_id,id)`，以及 `(legal_entity_id,batch_id,record_id,reverses_receipt_id) -> data_migration_writer_receipts(legal_entity_id,batch_id,record_id,id)` 三条真实 FK 全部 ON DELETE RESTRICT、DEFERRABLE INITIALLY DEFERRED。两个摘要固定 32 字节、writer_contract_version > 0；APPLY 的 reverses_receipt_id 必空，REVERSE 必非空且父 effect_kind=APPLY。两种 effect 的 batch/record/run/module/object 必须逐列等于所指记录；APPLY 的 target type/id 必须逐列等于 VALIDATED 时预留值，target hash 与 owner_effect_at 再逐列等于记录的 target_record_sha256 与 applied_at。REVERSE 的 target 只允许第 4.12.1 节逐对象冻结的三种封闭 owner effect：交易对象走既有通道产生的新取消/冲销/更正 fact；可变主数据取 catalog 具名不可变 version/change fact 并同时投影原根 after-image；仅 `procure.purchase_order_bundle`、`procure.payment_request_bundle`、`finance.cash_account_opening` 三支可把具名、独立的 `platform_audit.audit_events` owner change fact 作为 target，并同时投影原根 after-image。这三支 owner event 的 action、before/after、状态边、版本、根 id 与时点逐项固定，event_id 必须不同于 R0 的 receipt id；普通业务状态审计不自动满足该形状。三种 effect 均以 reverses_receipt_id 追到同记录 APPLY，并由 event_id=本 receipt id 的独立 R0 锁死 migration provenance；不得把普通 SQL 改状态、任意旧事实、同一审计行复用为 owner fact 与 R0，或孤立审计伪装为反向效果。`idempotency_key` 必须等于 `dm:v1:` 加 `SHA-256(RFC8785({legal_entity_id,batch_id,module_code,object_type,source_locator_sha256,effect_kind}))` 的 64 位小写 hex；UUID 取 RFC 9562 小写连字符字符串、source_locator_sha256 取 64 位小写 hex，因此同批同来源的跨 run 重放仍命中同一 key，APPLY 与 REVERSE 则因 effect_kind 分离。effect_sha256 精确为 `SHA-256(RFC8785({legal_entity_id,batch_id,record_id,run_no,module_code,object_type,effect_kind,target_object_type,target_id,target_record_sha256,writer_contract_version,idempotency_key,reverses_receipt_id}))`，UUID 及摘要沿用前述编码、整数取 JSON 整数、APPLY 的 reverses_receipt_id 取 JSON null，结果保存 32 个原始字节；第 4.12.1 节 25 行逐项生成静态 owner/target projection 分支，未登记、错属主、目标不存在、APPLY 未命中预留、REVERSE 通道/R0 不符或泛化 JSON 目标均拒绝。目标存在性与两个摘要均由受控数据库函数从实际目标重算，不能只信 `MigrationModuleWriter` 返回值；不要求 25 类异构业务根表新增同名 provenance 列。

表 18 至表 23 全部以 `legal_entity_id` 建标准 `le_isolation` 策略并 ENABLE、FORCE RLS；外键均同时带 `legal_entity_id`，不允许跨法人引用。六表的错误描述、来源系统引用与范围字段不得含连接串、口令、访问令牌、来源原文或未脱敏个人信息；明文来源只存在于客户控制的数据源及 `ep-data-migrate` 的有界内存缓冲，块发送完成立即清零。版本化模板只存仓库或客户批准的模板目录，数据库仅保存模板代码、版本与 SHA-256；模板必须由客户实施负责人签名，模板内容逐字段显式给出 `legal_entity_id`、`security_level`、`key_domain_id` 与 `retention_policy_code` 映射，缺任一项整批不得批准。

#### 3.1.1 P0-14A：备份、归档与恢复证据硬约束

迁移 `V20261023092500__platform_ops_harden_backup_evidence_graph.sql` 是本节唯一实现位点，必须一次完成下列事项，不得把任何一项延期为应用断言。

1. **真实引用与候选键。** `writeout_runs.sink_id`、`attachment_watermarks.sink_id`、`backup_sets.sink_id`、`recovery_drills.sink_id` 均指向 `offsite_sinks(id)`；`backup_sets.writeout_run_id` 指向 `writeout_runs(id)`；`backup_runner_slot.current_backup_set_id` 与 `backup_verifications.backup_set_id` 指向 `backup_sets(id)`；`archive_channel.rebuild_backup_set_id` 与 `archive_channel_transitions.to_rebuild_backup_set_id` 均指向 `backup_sets(id)`；`key_recovery_verifications.key_recovery_material_id` 指向 `key_recovery_materials(id)`；`recovery_drills.backup_set_id` 指向 `backup_sets(id)`；`alert_suppressions.degradation_window_id` 指向 `degradation_windows(id)`。全部为 `ON DELETE RESTRICT`，跨证据写序需要的外键一律 `DEFERRABLE INITIALLY DEFERRED`，不得以仓储层存在性检查代替。
2. **仅追加与版本行。** 向 `platform_core.append_only_registry` 登记 `writeout_runs`、`attachment_watermarks`、`backup_verifications`、`archive_channel_transitions`、`replication_reports`、`key_recovery_verifications`、`alert_suppressions` 七行，mode=`APPEND_ONLY`、mutable_columns=`'{}'`，逐表调用 `attach_table_guards`；运行角色的 UPDATE、DELETE 权限同时撤销。`deployment_records` 与 `offsite_sinks` 各登记 `IMMUTABLE_COLUMNS`，mutable_columns 精确为 `'{superseded_at}'`；追加 `UNIQUE(created_at)`，自定义延迟触发器强制任一事务提交后恰有一行 `superseded_at='infinity'`，历史活动行只能由 infinity 单向改为新活动行的 `created_at`，不得改回、提前闭合或无后继闭合；deployment 的新 revision 还必须等于旧 revision+1。两表 INSERT 仍只走受控版本写入函数。
3. **备份状态形状。** 下列“全空”只指 state-dependent 列，kind、sink_id、encryption_key_ref 与公共列始终按表定义非空。`PLANNED` 的 writeout_run_id、started_at、written_at、verification_concluded_at、verified_at、aborted_at、disposed_at、disposed_from_state、disposal_certificate_ref、bytes、base_lsn、backup_label_ref、manifest_ref、spill_peak_bytes、abort_reason 全空；`RUNNING` 仅 started_at 非空且恰由单例 slot 指向，其余上述列全空；`WRITTEN` 必有 started_at、written_at、writeout_run_id、bytes>=0、manifest_ref、spill_peak_bytes>=0，slot 已释放且四项校验时点/中止/处置字段全空；`VERIFIED` 继承 WRITTEN 形状并要求 `verified_at=verification_concluded_at`；`VERIFY_FAILED` 继承 WRITTEN 形状，要求 verification_concluded_at 非空而 verified_at 为空；`ABORTED` 只要求 started_at、aborted_at、writeout_run_id、spill_peak_bytes>=0、abort_reason 非空，其余 state-dependent 列全空；`DISPOSED` 必须逐列保存 `disposed_from_state` 所指 VERIFIED、VERIFY_FAILED 或 ABORTED 的原有完整形状，另要求 disposed_at 与 disposal_certificate_ref 非空。时序固定 `written_at>=started_at`、`verification_concluded_at>=written_at`、`aborted_at>=started_at`，disposed_at 不早于原结果态最后时点。数据库类 kind（DAILY_FULL、CHAIN_REBUILD_BASELINE）在 WRITTEN 及其后必须同时有 base_lsn 与 backup_label_ref；另两类必须同时为空。
4. **写出与校验闭图。** writeout run 只能在结束后一次插入：`finished_at>=started_at`，bytes/object_count 非负，OK 时 failure_category/last_error 同空，FAILED/ABORTED 时两者同非空。四类备份所指 run 的 writer_process 一律为 `backup-writer`；DAILY_FULL、CHAIN_REBUILD_BASELINE 的 writeout channel 精确为 FULL_BACKUP，CONFIG_BUNDLE 精确为 CONFIG_BUNDLE，ATTACHMENT_FULL 精确为 ATTACHMENT_FULL；WRITTEN/VERIFIED/VERIFY_FAILED 精确要求 `backup.sink_id=run.sink_id`、`backup.started_at=run.started_at`、`backup.written_at=run.finished_at`、`backup.bytes=run.bytes_written` 且 run.outcome=OK，ABORTED 精确要求同 sink、`backup.started_at=run.started_at`、`backup.aborted_at=run.finished_at` 且 run.outcome 为 FAILED 或 ABORTED。PLANNED、RUNNING、WRITTEN、ABORTED 必须零 verification 行；只有从 WRITTEN 结束校验的同一事务才一次插齐必需集合并转入 VERIFIED 或 VERIFY_FAILED：两个数据库类精确为 `{MANIFEST_CHECKSUM,DECRYPT_READBACK,PG_VERIFYBACKUP}`，ATTACHMENT_FULL 精确为 `{MANIFEST_CHECKSUM,DECRYPT_READBACK,ATTACHMENT_CHECKSUM}`，CONFIG_BUNDLE 精确为 `{MANIFEST_CHECKSUM,DECRYPT_READBACK}`，每种方法恰一行，缺项、额外项或重复项均拒绝。每行 `started_at>=backup.written_at`，`finished_at>=started_at`，backup.verification_concluded_at 必须等于该集合的最大 finished_at；全 PASS 才可 VERIFIED，且 verified_at 等于 concluded_at；任一 FAIL 必须 VERIFY_FAILED，且 VERIFY_FAILED 不得缺任何必需方法。这样允许先计算全部校验结果、再以任意父子写序原子提交，但不允许一张 WRITTEN 备份长期挂半套“已完成校验”证据。
5. **单向边与处置。** 只允许 `PLANNED→RUNNING→WRITTEN→VERIFIED|VERIFY_FAILED`、`RUNNING→ABORTED`、`VERIFIED|VERIFY_FAILED|ABORTED→DISPOSED`；PLANNED 只可在启动前改 kind/sink/encryption_key_ref 与公共更新证据，RUNNING、WRITTEN 不允许同态改写任一业务列，每次变化必须是上述合法边并一次写齐目标态证据；除处置边外三个结果态不可更新，DISPOSED 完全不可变。处置事务必须由 OpsDisposalService 在离站精确 key/版本删除并回读不存在后写证书引用；数据库行与历史校验不删除。`v_backup_last_success` 继续只取当前 state=VERIFIED，因此已销毁备份不会被误选为可恢复副本。
6. **归档证据链。** 单例初始行为 row_version=1、state=HEALTHY、last_transition_id 为空且无断链字段；从第二版起每个 row_version 恰有一条版本证据，`from_row_version/to_row_version` 连续、from_state 等于上一条 after-image 的 to_state，当前 last_transition_id 指向最后一条，最后一条全部 `to_*` after-image 与当前单例逐列相等。`STATE_CHANGE` 只能取第 4.2 节九个合法 from/to 对（“Healthy 或 RetentionWarning 到 SlotInvalidated”展开为两个不同对）；`OBSERVATION` 必须 from_state=to_state，slot_name/slot_active/broken_at/break_cause/rebuild_backup/restored_at 逐列等于上一 after-image，只允许 confirmed_flush_lsn 按 `pg_lsn` 非递减并更新复制核对四字段，因而 30 秒采样不会被错误逼成一次状态迁移。复制核对四字段的每版形状固定：outcome 为空时 at/error 同空且 streak=0；MATCHED/MISMATCHED 时 at 非空、streak=0、error 为空；NO_RESULT 时 at/error 非空，前一版同为 NO_RESULT 则 streak=前值+1，否则恰为 1。任一新核对结论的 at 必须严格大于前一非空 at；未产生新结论的 STATE_CHANGE 必须逐列保留四字段，不能借状态迁移重置 streak。SLOT_INVALIDATED 必有断点/原因且无恢复时点；REBUILDING 还必须指向 kind=CHAIN_REBUILD_BASELINE 且 state=RUNNING|WRITTEN 的备份；SUSPENDED 可在尚未创建基线时无该引用，已有引用时只能指向该类 ABORTED、VERIFY_FAILED，或之后处置但 `disposed_from_state in ('ABORTED','VERIFY_FAILED')` 的 DISPOSED 备份；恢复后的 HEALTHY/RETENTION_WARNING 必须保留断点、原因、基线引用、restored_at 四项，并让基线为 VERIFIED，或为之后处置但 `disposed_from_state=VERIFIED` 的 DISPOSED。初始健康形状则四项全空。数据库从固定初始行加全部 typed after-image 必须能重建每个历史 row_version；detail 只可补充说明，不能成为重放必需输入。
7. **恢复演练形状。** 插入 RUNNING 时所指备份必须在开始时为 VERIFIED 且 kind 只可 DAILY_FULL 或 CHAIN_REBUILD_BASELINE，`backup_verified_at_at_start=backup_sets.verified_at`，sink_id 与 sink_kind_at_drill 必须等于该备份及落点事实；以后备份可为 VERIFIED，或在演练开始之后由 VERIFIED 处置成 DISPOSED，不能改写历史判据。RUNNING 的结束、outcome、failure、report 与全部结果指标为空；只允许 RUNNING→PASSED|FAILED，两个终态都要求 `window_ended_at>=window_started_at`、相符 outcome 与 report_ref，终态逐列不可变。PASSED 的 failure_stage/failure_code 必须同空；WHOLE_MACHINE_RECOVERY 与 PRODUCTION_RECOVERY 的 PASSED 必须具备 readback、rto、shard-pickup、attachment、decrypt、invariant 六组指标，readback throughput>0，其余耗时/计数非负，attachment failed=0 且不超过 total、invariant batches>0，`rto_seconds >= attachment_check_seconds+decrypt_seconds+invariant_check_total_seconds` 且不超过 14400；两者在 LATEST_VERIFIED 时 rpo_db_seconds/rpo_attachment_seconds 成对必填、非负且分别不超过 900，在 RETENTION_TAIL 时成对为空。FAILED 的 failure_stage/failure_code 必须同非空且 failure_code 为已登记稳定错误码；WHOLE/PRODUCTION 只保留实际已完成的原子组：readback_throughput_mibps、rto_seconds、shard_pickup_seconds、decrypt_seconds 四个单值各自可空，附件的 total/failed/seconds 三列全空或全非空，不变量五列全空或全非空，RPO 两列全空或全非空且仅 LATEST_VERIFIED 可非空；任何已填值仍满足 throughput>0、其余非负、failed<=total、invariant batches>0，rto 非空时还须 `rto_seconds >= coalesce(attachment_check_seconds,0)+coalesce(decrypt_seconds,0)+coalesce(invariant_check_total_seconds,0)` 且不超过 14400，不得以零值冒充未执行的整组步骤。KEY_MATERIAL_ISOLATED_RECOVERY 只允许 LATEST_VERIFIED；PASSED 必须且只能填写 shard_pickup_seconds 与 decrypt_seconds 两项非负指标，FAILED 也只允许这两项按实际执行情况可空，且 decrypt_seconds 非空蕴含 shard_pickup_seconds 非空；其余 readback/rto/rpo/attachment/invariant 指标始终全空。RETENTION_TAIL 只允许 WHOLE_MACHINE_RECOVERY，两项 RPO 必须为空，并要求 `window_started_at-backup_verified_at_at_start >= (retention_days_at_start-1) days`。
8. **数据库执行面。** `platform_ops.assert_backup_evidence_graph_consistent()` 作为同一函数附着到 `writeout_runs`、`backup_sets`、`backup_runner_slot`、`backup_verifications`、`archive_channel`、`archive_channel_transitions`、`recovery_drills` 的 INSERT/UPDATE/DELETE `CONSTRAINT TRIGGER DEFERRABLE INITIALLY DEFERRED`；函数每次锁相关父行并按最终事务快照检查完整图。不得使用普通 AFTER trigger 冒充延迟约束，也不得只检查被改的一侧。

#### 3.1.2 P0-14B：历史迁移批准与 writer receipt 硬约束

迁移 `V20261023092600__platform_ops_harden_data_migration_evidence_graph.sql` 创建表 22、23，修改表 18 至表 21，并一次安装下列契约。

1. **批准流程、角色与重新认证 ABI。** 发布并只使用四个 `platform_flow.process_definitions.code`：`DATA_MIGRATION_DATA_OWNER_APPROVAL`、`DATA_MIGRATION_MODULE_OWNER_APPROVAL`、`DATA_MIGRATION_FINANCE_OWNER_APPROVAL`、`DATA_MIGRATION_REVERSAL_SECOND_APPROVAL`；它们是普通已发布流程定义，不扩展阶段 4 的 ApprovalScenarioCode。定义码按 approver_kind 唯一映射，SECOND_APPROVER 只可用第四码，其余三类各用同名责任人定义码。角色码同样是编译期全映射：DATA_OWNER=`OPS_DATA_OWNER`，FINANCE_OWNER=`FINANCE_MANAGER`，SECOND_APPROVER=`SECURITY_ADMIN`；MODULE_OWNER 按 module_code 唯一取 `mdm→MANAGEMENT_APPROVER`、`cpq→SALES_MANAGER`、`clm→MANAGEMENT_APPROVER`、`sales→SALES_MANAGER`、`procure→PROCURE_MANAGER`、`inventory→MANAGEMENT_APPROVER`、`ledger|finance|invoice→FINANCE_MANAGER`、`project|service→PROJECT_MANAGER`，不得配置替换或设兜底。表 22 的 definition 快照必须逐列等于实例及定义行；process task 必须以长 FK 属于同一实例、kind=APPROVAL、state=COMPLETED、approval_ref=本 evidence id、reauth_ref=本 evidence.reauth_challenge_id，且 candidate_role_codes 必须恰为上述单元素角色数组；task decision 对 APPROVED/REVOKED 为 APPROVED、对 REJECTED 为 REJECTED。reauth challenge 必须是 `HIGH_RISK_REAUTH+DATA_MIGRATION+CONSUMED`，user_id=submitted_by、consumed_at 非空且不晚于 instance.started_at；其 subject_digest 必须逐字等于 `SHA-256(RFC8785({operation_type:'DATA_MIGRATION',legal_entity_id,batch_id,known_difference_id,reauth_purpose,content_version,content_hash}))`，对象恰有这七个键，两个 UUID 值及非空 known_difference_id 均取 RFC 9562 小写连字符字符串，content_version 取 JSON 整数，content_hash 取 64 位小写十六进制字符串，known_difference_id 对批次三 phase 取 JSON null，最终摘要保存 32 个原始字节而非 hex 文本。reauth_purpose 对 BATCH/CUTOVER/REVERSAL 分别等于 phase，对已知差异初次决定为 KNOWN_DIFFERENCE_DECISION、撤销为 KNOWN_DIFFERENCE_REVOCATION，确保旧决定挑战不能重放为撤销；同一业务动作派生的多名 owner 流程可共享这一个已消费挑战，但不同 purpose/content version/hash 不得共享。evidence 的 submitted_by/submitted_at 精确取 task.initiator_user_id/instance.started_at，decided_by/decided_at 精确取非空 task.assignee_user_id/task.completed_at，created_by=decided_by；approver role/id/grant 与有效期快照只由函数从 decided_by 在决定日的同法人有效角色授权派生，角色须 `lifecycle_state=EFFECTIVE and is_active=true`，调用方不得自填；同一人同一角色若因历史数据存在多个覆盖决定日的授权行，唯一选择 `(effective_from desc,id asc)` 第一行。decision 只能由 task.decision 映射。实例必须 COMPLETED，业务对象对批次三 phase 取 `(platform_ops.data_migration_batch,batch_id)`，对差异 phase 取 `(platform_ops.data_migration_known_difference,known_difference_id)`。
2. **内容版本绑定，不复制审批命令。** 本场景审批后没有另一条待执行的业务命令，callback 的唯一效果是记录本 evidence 并由延迟图推进现有批次或差异状态，因此不创建、不引用 `approval_command_snapshots`，也不得伪造不属于 15 项 `ModuleCode` 的 owner_module=`platform`。每个实例的 variables 必须由相应定义的 JSON Schema 限为且只为 `{evidence_id,batch_id,known_difference_id,phase,approver_kind,module_code,content_version}` 七个非敏感路由键，其中两个可空键显式取 JSON null；不含 content hash、来源引用、差异内容、请求体或其摘要，重新认证引用只存在 task.reauth_ref 与表 22 真实 FK，不复制进 variables。实例启动事务先锁 batch/known difference，核对当前 content_version、重算 prospective content_hash、验证并消费上述 challenge，再预生成 evidence id；审批页面每次读取仍从受 RLS/ABAC 保护的当前业务行取内容，不从流程变量复制。callback 只能调用精确 ABI `platform_ops.record_data_migration_approval_evidence(p_legal_entity_id uuid,p_process_instance_id uuid,p_process_task_id uuid) RETURNS uuid`；函数以 task.approval_ref、task.reauth_ref 和实例 variables 派生 evidence id、reauth challenge id 与全部业务列，不接受 decision、content_hash、owner、definition、reauth 或时点作为调用参数。函数在同一事务再次锁行，要求路由七键、业务对象、实例、任务、定义长 FK 与重新认证 subject 全相符且当前 content_version 未变化，再从当前受保护列重算 content_hash 并插入 evidence，由同一延迟图推进状态。任一不符时零 evidence、零状态迁移；旧 content_version 或旧 reauth purpose 的流程即使后来完成也永不计数。该形态既不把高保密内容复制进流程表，也不扩展已冻结 ModuleCode。
3. **批次批准集合。** BATCH_APPROVAL 与 CUTOVER_APPROVAL 均恰有一条 DATA_OWNER、一条 FINANCE_OWNER，以及 `source_module_codes` 每码一条 MODULE_OWNER APPROVED；前两项 decided_by 分别等于 batch.data_owner_id/customer_finance_owner_id，每条模块 evidence 的 module_code 与覆盖集合一一对应，且决定者必须持有第 1 项静态映射的有效角色授权，集合外、错角色或重复角色均拒绝。BATCH_APPROVAL 绑定 approval_content_hash，CUTOVER_APPROVAL 绑定 cutover_content_hash。REVERSAL_APPROVAL 恰有 DATA_OWNER 与 SECOND_APPROVER 两条 APPROVED，二者不同人、分别持 `OPS_DATA_OWNER` 与 `SECURITY_ADMIN` 有效授权并绑定 reversal_content_hash。当前 content_version 有任何 REJECTED 时必须升版重提，旧版证据永不被新状态计数。
4. **差异决定集合。** 表 21 的 module_code 必须属于 batch.source_module_codes，MODULE_OWNER evidence 的 module_code 必须与其逐字相等。PROPOSED 无当前内容版本结论 evidence 且 decided_at 为空；APPROVED 恰有由行内 data/module/finance owner 各自作出的三条 APPROVED、分别持 `OPS_DATA_OWNER`/module 静态映射角色/`FINANCE_MANAGER` 有效授权且无 REJECTED，decided_at 等于三者最晚时点；REJECTED 至少有一条对应 owner 的 REJECTED、没有完整三方批准集，decided_at 等于当前内容版本全部批准/驳回 evidence 的最大 decided_at；REVOKED 必须对原三条 APPROVED 各有一条同 approver_kind、module_code、decided_by、content version/hash 的 REVOKED evidence，reverses_evidence_id 一一对应且不得二次撤销，差异行 decided_at 改为三条 REVOKED 的最大 decided_at。差异只能用于同一 batch；四个不可豁免 check_kind 永不接受 APPROVED_DIFFERENCE。
5. **逐记录、目标预留与回执双向图。** QUEUED 的三项映射、目标预留、目标 hash、两个 receipt 指针、错误与 applied_at 全空；VALIDATED 必须同时具备三项映射以及 catalog 固定的 `target_object_type` 与服务端新生成 `target_id`，但 `target_record_sha256`、两个 receipt、错误与 applied_at 仍为空。VALIDATED 的提交点由 25 个静态分支证明该同法人目标 relation 尚无该 id，普通唯一键又阻止任何两条迁移记录预留同一根；本态不允许同事务顺带创建目标。FAILED 必须有 error_code 与 sanitized_error、receipt 与 applied_at 为空：从 QUEUED 失败时映射和目标预留全空，从 VALIDATED 失败时完整保留映射与目标预留但 target hash 为空，不得出现半套；APPLIED 必须保留预留、填入数据库从实际根重算的 target_record_sha256、applied_at 与 apply_receipt_id，且无错误/reversal receipt；REVERSED 保留 APPLIED 全部证据并增加 reversal_receipt_id。只允许 `QUEUED→VALIDATED|FAILED`、`VALIDATED→APPLIED|FAILED`、`APPLIED→REVERSED`；FAILED、REVERSED 终态，修复后重跑必须在新 run_no 新建记录。映射和目标预留从 VALIDATED 起逐列不可变；映射 key domain 必须属同法人且为 ACTIVE，owner 的静态 APPLY 投影必须逐列证明最终根的 security_level 等于 mapped_security_level，且 bundle 内所有密文 key ref 属 mapped_key_domain_id、所有附件/记录采用 mapped_retention_policy_code 对应策略。APPLY receipt 与记录逐列同批次/run/module/object/预留目标/hash/time。REVERSE 的三种唯一形态由 catalog 逐项固定：交易对象指向既有取消、冲销或更正通道生成的新 owner fact；可变主数据指向具名不可变 version/change fact 并投影根 after-image；仅采购订单、付款申请、资金账户三根指向具名 owner audit change fact 并投影根 after-image。这三条 owner event 与 R0 必须是两个不同 event_id、同法人、同 occurred_at 的审计事实，owner action 与 before/after 状态版本图按 owner 阶段逐字固定，R0.after.owner_effect_id 指 owner event。全部形态都须在同事务另写 R0，其 `event_id` 精确等于 REVERSE receipt id、action=`DATA_MIGRATION_REVERSED`、object_type/id 指向原 APPLY 根，after 恰含 `{schema_version:1,data_migration_record_id,batch_id,apply_receipt_id,owner_effect_object_type,owner_effect_id}` 六键；函数逐键核实后才承认反向 provenance。记录的两个 nullable 长 FK 与 receipt 的 record FK 构成双向 DEFERRABLE 恰一图。
6. **writer receipt 产生点。** `MigrationModuleWriter::apply/apply_reversal` 的唯一权威写入者在调用方 UnitOfWork 内写完目标后只返回 `MigrationWriteEffect { receipt_id, target_object_type, target_id, writer_contract_version, idempotency_key, reverses_receipt_id }`；APPLY 必须逐值复用记录已预留的 target type/id，batch/run/module/object 从已锁定 record 派生，两个摘要与 owner_effect_at 不由调用者填写。业务追加记录、既有领域事件/Outbox、审计、receipt 与记录状态同事务提交。表 23 不向 ep_app_rw 授直接 INSERT；executor 只能调用精确 ABI `platform_ops.record_data_migration_writer_receipt(p_legal_entity_id uuid,p_receipt_id uuid,p_record_id uuid,p_effect_kind text,p_target_object_type text,p_target_id uuid,p_writer_contract_version int,p_idempotency_key text,p_reverses_receipt_id uuid) RETURNS uuid`。该 SECURITY DEFINER 函数锁 record，先按表 23 冻结公式从 record 与 batch 重算并常量时比较 p_idempotency_key，再按第 4.12.1 节 25 行 catalog 生成静态 `CASE` 分支，以固定 relation、关联列和排序键查询同法人目标，拒绝不存在目标、错 owner、APPLY 未使用预留根、REVERSE 未走 catalog 固定 owner 通道或错 idempotency key；随后由同一分支对目标规范投影计算 target_record_sha256，由函数按表 23 冻结 JSON 键集计算 effect_sha256。APPLY 的 owner_effect_at 取 `clock_timestamp()`；REVERSE 必须先存在 event_id=p_receipt_id 的上述不可变审计事件，owner_effect_at 精确取该事件 occurred_at。函数不信任调用方返回摘要，不运行动态 SQL，不查询 `information_schema`，也不要求异构业务根表新增同名 provenance 列。CI 从同一 25 行 catalog 生成 Rust owner 注册表、SQL 分支、relation allowlist、排序键与 target projection 快照并逐值比对，缺支、重复、错 owner、无法查询实际目标、关系名拼接或通用 JSON 目标分支均失败。
采购订单、付款申请、资金账户三支的 writer receipt 附加强约束属于第 6 项：REVERSE target 固定解析为 `platform_audit.audit_events.event_id=p_target_id` 的具名 owner event，再静态 join 原 APPLY 根与 `event_id=p_receipt_id` 的 R0。函数必须核两事件 id 不同、同法人同 occurred_at、固定 owner action/object/before/after/version/state edge、根最终 after-image，以及 R0 对 owner event 的精确引用。三支 before/after 的 `schema_version` 固定为 JSON number `1`，`row_version` 则按全库审计链规则固定为不带前导零的正十进制 JSON string；092600 不对不可信 JSON 做 bigint cast，而把该字符串与 root.row_version/object_version 的规范 `::text` 及真实变更分支的 `(root.row_version-1)::text` 逐字比较；JSON number、空串、负数、前导零和溢出字符串均因不等而拒绝。三支的 target canonical projection 固定为 `{owner_audit:row_v1(owner_event),<owner_root_after>:row_v1(root)}` 再加 R0，不接受调用方 JSON。APPLY 的 owner_effect_at 仍取数据库时钟；REVERSE 的 owner_effect_at 精确取 R0.occurred_at，也就等于 owner event.occurred_at。

7. **对账与切换图。** current_run_no 的记录行数必须恰等于 source_record_count 且全为 APPLIED，来源 locator 在该 run 内无重复；对账行 `(check_kind,scope_key)` 集合必须与 required_reconciliation_keys 完全相等。READY_FOR_CUTOVER 要求当前 run 无 FAILED、四类不可豁免项全 PASS、其余项只为 PASS 或关联同批次当前 APPROVED 差异的 APPROVED_DIFFERENCE，并以 `(check_kind,scope_key,source_value,target_value,difference_value,outcome,known_difference_id,report_ref)` 的排序规范 JSON 计算 reconciliation_digest。cutover_content_hash 精确为 `SHA-256(approval_content_hash,current_run_no,source_frozen_at,source_readonly_evidence_ref,delta_watermark,source_manifest_sha256,source_record_count,reconciliation_digest,final_reconciliation_report_ref,当前 APPROVED 差异 id/content_hash 排序集)`。
8. **批次状态形状。** 非 CANCELLED 的 cancelled_from_status/cancelled_at/cancel_reason 三列全空。DRAFT/APPROVED 的 current_run_no=0 且源冻结、对账、切换、冲销字段为空；DRAFT 的 source_readonly_test_ref 可空，APPROVED 及其后必须非空并具备当前 BATCH_APPROVAL 集，且当前 approval evidence 的 content_hash 已包含该引用与 source_schema_fingerprint。TRIAL_RUNNING/FAILED/PASSED 的 current_run_no>0 且 trial_report_ref 在结束态非空，PASSED 还要求 trial_pass_count>0；trial_nonconvergent_count 达到 2 时状态只能保持 TRIAL_FAILED 后转 CANCELLED，禁止再增 current_run_no。SOURCE_FROZEN 及其后必须成组具备 source_frozen_at/read-only evidence/delta watermark/manifest/count。APPLYING、DELTA_CATCHUP、RECONCILING 只持已冻结事实且切换字段为空；READY_FOR_CUTOVER 另要求第 7 项闭图、reconciliation digest/report 与 cutover content hash；CUTOVER_COMPLETED 及其后还要求完整 CUTOVER_APPROVAL、cutover_at 非空且租约已清；REVERSAL_PENDING/REVERSED 必须有 reversal_batch_ref/reason/content hash 与双人 REVERSAL_APPROVAL，REVERSAL_PENDING 的当前 run 允许 APPLIED 与已完成冲销的 REVERSED 混合以支持分块处理，REVERSED 时当前 run 必须全为 REVERSED 且每条同时保留 APPLY 与 REVERSE receipt。CANCELLED 必须同时具备 cancelled_from_status、cancelled_at 与长度 1 至 2000 的 cancel_reason，租约清空，并逐列保留其 DRAFT、APPROVED、TRIAL_FAILED 或 TRIAL_PASSED 前态的其余证据形状；不得已有源冻结/切换/冲销证据，且终态不可改。状态边仍取第 4.12 节九组，证据列不得先于其里程碑伪造；每次 trial/freeze/apply 的源端 schema 指纹都必须等于 batch.source_schema_fingerprint，post-freeze SOURCE_CHANGED 转 TRIAL_FAILED 时必须完整保留冻结五元组。
9. **延迟执行与不可变性。** `data_migration_reconciliations`、表 22、表 23 登记 APPEND_ONLY 并撤销 UPDATE/DELETE；表 21 只允许状态机列变化，离开 PROPOSED 后内容不可变；表 18、19 只允许本节状态边及相应里程碑列变化。`platform_ops.assert_data_migration_evidence_graph_consistent()` 附着表 18 至表 23 的 INSERT/UPDATE/DELETE `CONSTRAINT TRIGGER DEFERRABLE INITIALLY DEFERRED`，锁 batch 后检查最终快照；普通 FK 命中但错 batch、错 record、错流程实例、错内容版本、错 role/grant 或决定日授权无效仍必须由该图拒绝。角色授权的 effective_to 后续正常关闭不反向作废既有批准，历史有效区间以表 22 的不可变快照为准；修改角色/授权父行的 FK 身份列仍会被真实 FK 阻断。

#### 3.2 视图

- platform_ops.v_degradation_open：closed_at = 'infinity' 的全部条目，含 kind 与 subject、是否被抑制、抑制到期时间与是否可抑制。
- platform_ops.v_rpo_status：输出两行，target 取 DATABASE 与 ATTACHMENT，各行含 effective_seconds、basis、basis_source_kind、evidence_ref；判定算法见第 4.6 节。
- platform_ops.v_backup_last_success：按 kind 给出最近一次 VERIFIED 的备份集及其时间。
- platform_ops.v_capacity_current：六项组件的最近一次采样与占容量下限比。
- platform_ops.v_ops_health：ops-agent 与门禁工装的单一入口，聚合上述四个视图的关键取值。

#### 3.3 权限

ep_ops_ro 授予上述五个视图的 SELECT，不授予任何基表。ep_app_rw 只取得可更新表所需的 SELECT/INSERT/UPDATE 与仅追加普通台账的 SELECT/INSERT，所有基表 DELETE 一律撤销，APPEND_ONLY 表的 UPDATE/DELETE 一律撤销；表 22、23 的直接 INSERT 也撤销，分别只允许标准流程 callback 调用 `platform_ops.record_data_migration_approval_evidence(...)` 与 executor 调用 `platform_ops.record_data_migration_writer_receipt(...)` 两个受控 SECURITY DEFINER 函数写入。两函数固定 owner=`ep_mod_platform_ops`、函数属性固定 `SET search_path = pg_catalog, platform_ops, platform_flow, platform_authz`，先 `REVOKE ALL ON FUNCTION ... FROM PUBLIC` 再只向 ep_app_rw 授 EXECUTE；092600 只向 ep_mod_platform_ops 补这两函数所需的 platform_flow 三表、platform_core.reauth_challenges、user_legal_entity_grants、roles、user_role_grants、platform_core.key_domains 与 25 个静态目标投影列的 SELECT，不授写权限或通用 schema 写权。批准函数按一致顺序锁业务对象、流程实例/任务/定义、reauth challenge、角色与所选授权行，直到证据及状态同事务提交，避免审批完成、token 消费与撤权竞态。函数内先比对 `p_legal_entity_id=nullif(current_setting('app.legal_entity_id',true),'')::uuid`，任何空值或不等均拒绝，所有对象名带 schema，不允许调用方影响解析。访问表 18 至表 23 时仍必须注入 SecurityContext 并经 RLS 与 ABAC 双重判定。ep_analyst_ro 不授予 platform_ops 任何对象，理由是运维与迁移台账都不属于分析与报表取数范围。ep_archiver 与 ep_backuper 不授予 platform_ops 任何对象，两个写出进程一律经 IPC 上报，不直连。`ep-data-migrate` 不持任何数据库角色或连接串，只持由既有身份流程签发、绑定用户、设备、法人、批次且最多有效 10 分钟的一次性 API 会话文件；文件 ACL 只允许发起人，首次成功换取会话后立即删除，批次窗口关闭时服务端强制作废。

历史迁移 API 只使用一项最小权限，不以界面职责名称另造多项。092600 固定 seed `platform_authz.permission_items` 一行：`id='00000000-0000-7000-8000-000000000315'`、`code='platform.data_migration'`、`module_code='platform'`、`function_point='历史数据迁移批次管理'`、`allowed_actions=['VIEW','CREATE','UPDATE','SUBMIT']`、`object_type='platform.data_migration'`；再 seed `platform_authz.object_scope_bindings` 一行：`id='00000000-0000-7000-8000-000000000509'`、`object_type='platform.data_migration'`、`schema_name='platform_ops'`、`table_name='data_migration_batches'`、`owner_user_col=NULL`、`owning_dept_col=NULL`、`project_col=NULL`、`customer_col=NULL`、`security_level_col='security_level'`。所有批次、记录、对账、差异及 action route 的授权 object id 一律取 path 中的 batch id；子对象先以同法人长 FK 解析回 batch，再做这一次对象级判定，不能拿 record/difference/receipt id 绕过 batch scope。`VIEW/CREATE/UPDATE/SUBMIT` 分别覆盖只读、建 DRAFT、编辑/上传/执行、发起批准/切换/冲销；实际谁可执行仍由本节职责、职责分离、流程证据和 DATA_MIGRATION reauth 叠加收紧。两行均 `ON CONFLICT DO NOTHING` 后逐字段断言，固定 id、code/object_type 或任一字段已有但不等即令迁移失败，不覆盖存量；092600 不自动写任何 `role_permission_grants`，迁移执行人、数据责任人、模块责任人与财务责任人的授权只能经签名 authz 配置显式授予。

#### 3.4 迁移编号与顺序

目录 db/migrations/platform_ops/，迁移历史落在全局唯一的 platform_core.schema_history。执行顺序由单一全局 Runner 按文件版本号全序排定。

本节列出的 28 个文件全部属于 Stage 14a0 的迁移冻结批次：必须一次进入 catalog，不能只先登记 `deployment_records` 再在 F-55 九个 `V20261024...` 已进入共享数据库后补其余低版本。14a0 可在阶段 1、2 后编写和做静态全序检查；涉及后续 schema 的文件只在其全部前置迁移已存在时参加 fresh-database 全量执行。Stage 14b 只完成运行时和真实证据，不新增、重编号或延后执行任何未在 14a0 冻结的 `V20261023...` 文件。

依赖审计结论是“28 文件可早冻结，28 文件不可早落共享库”，两者不得混写。`092400` 消费阶段 2 的 `unpoliced_table_registry` 与阶段 4 冻结的矩阵 case；`092500` 消费本批全部备份/归档/恢复对象以及既有 append-only、immutable 与 audit 基础，并把 degradation kind 从初始 3 项扩为终态 21 项；`092600` 还消费 flow/authz/reauth、Stage 8 `MIGRATION_HISTORY`、Stage 9 `HISTORICAL_MIGRATION` 和第 4.12.1 节分布在各业务阶段的 25 个静态 owner projection。Stage 14 自有 28 文件到 `092600` 结束，但全局 pre-F-55 链还必须继续执行 Stage 6 的 `092700/092800`；F-55 首迁移 `V20261024090000` 明确依赖 `092800`。故 Stage 14a0 结束时只允许 catalog/static/focused disposable-fixture 结论为绿，禁止把 28 文件应用到任何长期存在或多人共用的可变数据库；13c 也只能先做 contract、red test 与九个 SQL 的原子候选。全链 fresh-database 绿验、共享开发库执行和任一依赖数据库完成态的 F-55 gate 均以第 0.0 节 `PreF55DatabaseAdmissionV1` 为硬前置。该前置是迁移 Runner 在执行第一条 `V20261024...` 前的单次 fail-closed admission，不是启动时自检，也不存在“先执行能跑的子集、以后补低版本”的支路。

1. V20261023090100__platform_ops_deployment_records.sql，该文件按第 3.1 节表 1 建表，配额冻结引用列直接以 resource_quota_frozen_ref 建立，不另出改名迁移。
2. V20261023090200__platform_ops_offsite_sinks.sql
3. V20261023090300__platform_ops_extend_degradation_windows.sql，常规事务迁移，只做 ALTER 并追加 ck_degradation_windows_le_required 与 ck_degradation_windows_not_suppressible 两条 CHECK；不得创建索引、不得改写 kind CHECK。后者第一版的不可抑制字面量闭集恰为 `OFFSITE_SINK_NOT_CONFIGURED|OFFSITE_COPY_PROTECTION_MISSING|WRITER_NOT_IN_SERVICE|VIRUS_SCANNER_NOT_AVAILABLE` 四项，即使后三项中尚未被初始 kind CHECK 放行的取值当前无行也不删去。迁移 slug 为保持全局迁移引用稳定继续保留 `extend_degradation_windows`。本表、subject 列、初始 3 项 kind CHECK 与两条既有约束均由阶段 2 建立；本文件不建表、不增删列、不提前放行其余 18 项。终态五项不可抑制闭集与 21 项 kind CHECK 统一由 `092500` 在全部早期写入方之后替换到位。
4. `concurrent/V20261023090350__platform_ops_add_degradation_window_indexes.sql`，独立非事务迁移，依次以 `CREATE INDEX CONCURRENTLY` 为既有 `platform_ops.degradation_windows` 建立第 3.1 节冻结的三个索引；文件不得混入 CHECK、列变更或其他事务 DDL。
5. V20261023090400__platform_ops_writeout_runs.sql
6. V20261023090500__platform_ops_attachment_watermarks.sql
7. V20261023090600__platform_ops_backup_sets.sql
8. V20261023090700__platform_ops_backup_runner_slot.sql；建表后同文件插入固定单例 `00000000-0000-0000-0000-0000000000b1`。
9. V20261023090800__platform_ops_backup_verifications.sql
10. V20261023090900__platform_ops_archive_channel.sql；建表后同文件按表 9 冻结的完整 seed 形状插入固定单例 `00000000-0000-0000-0000-0000000000a1`，不得只填 row_version/state 而依赖运行期补列。
11. V20261023091000__platform_ops_archive_channel_transitions.sql
12. V20261023091100__platform_ops_replication_reports.sql
13. V20261023091200__platform_ops_wal_retention_samples.sql
14. V20261023091300__platform_ops_capacity_samples.sql
15. V20261023091400__platform_ops_key_recovery_materials.sql
16. V20261023091500__platform_ops_key_recovery_verifications.sql
17. V20261023091600__platform_ops_recovery_drills.sql
18. V20261023091700__platform_ops_alert_suppressions.sql
19. V20261023091800__platform_ops_data_migration_batches.sql
20. V20261023091900__platform_ops_data_migration_records.sql
21. V20261023092000__platform_ops_data_migration_reconciliations.sql
22. V20261023092100__platform_ops_data_migration_known_differences.sql；第 19 至 22 号在建表文件内同批建立标准法人策略、ENABLE 与 FORCE RLS，不登记进 unpoliced_table_registry。
23. V20261023092200__platform_ops_views.sql
24. V20261023092300__platform_ops_grants_ops_ro.sql
25. V20261023090000__platform_ops_backfill_singletons.sql。该全局版本早于两张表的建表版本，保留 slug 但内容固定为带 rollback no-op 的兼容空迁移，绝不引用尚不存在的表；两条单例分别由第 8、10 号建表迁移原子插入。这样修复旧文本按清单序号误判执行序、实际全局 Runner 却按版本号先跑 090000 的空库必败矛盾。
26. V20261023092400__platform_core_backfill_stage14_unpoliced_table_registry.sql，落在 db/migrations/platform_core/ 目录下，其主要创建对象是 platform_core.unpoliced_table_registry 的登记行，按裁定通则第五条随主要创建对象所属 schema 归目录；版本号晚于本阶段全部基础建表迁移并早于两条硬化迁移。按基线第 3.8 节的正向登记制，向阶段 2 交付的该登记表写入本阶段新建且不带 `legal_entity_id` 的 16 张 platform_ops 表各一行，五列体例照抄阶段 4 第 29 号迁移，即 schema_name、table_name、admission_basis、isolation_entry 与 matrix_case_id。16 行的 admission_basis 一律取 ISOLATION_OR_DEPLOYMENT_METADATA，依据是第 0 节偏离二已自证的准入判据，即这些表记录的是部署自身的元数据而非任一法人的业务数据；isolation_entry 一律取第 5 节运维中心只读 API 按运维管理员、安全管理员与审计管理员三类角色的 ABAC 判定；matrix_case_id 取该入口在 tests/rls_matrix 中的用例标识。第 3.1 节表 3 的 degradation_windows 由阶段 2 建表并已含在阶段 2 登记的八行内，本文件不重复写入，以免触发 ux_unpoliced_table_registry_schema_table 两列唯一冲突；第 19 至 22 号新建的四张法人表及第 28 号新建的两张法人表都有 RLS，也不写本登记表。
27. V20261023092500__platform_ops_harden_backup_evidence_graph.sql，先验证现有 degradation_windows 行全部可映射到本文终态闭集，再在同一事务把 kind CHECK 从阶段 2 初始 3 项替换为本文精确 21 项、把 ck_degradation_windows_not_suppressible 替换为本文精确 5 项；同批落实第 3.1.1 节的列、外键、append-only/immutable 登记、状态与归档闭图、恢复演练形状、约束触发器和精确权限。对应 Rust `DegradationKind` 已由 Stage 14a0 contract 先从 3 项扩为同序 21 项以供后续代码编译，但直到本迁移成功前只允许 mock/参数绑定测试，不允许新增值真实写库；部署时必须让含 21 值 Rust binary 与本迁移同一批切换，不得占用别的版本号或发布 Rust/SQL 半完成态。
28. V20261023092600__platform_ops_harden_data_migration_evidence_graph.sql，创建表 22、23，修改表 18 至表 21，建立六表 RLS、目标预留、批准/receipt/对账/状态延迟图、25 个静态 projection 分支及受控写函数；同文件按第 3.3 节 seed 并逐字段断言 `platform.data_migration` permission/binding，不自动授角色；并按第 4.12.1 节前置扩展 Stage 8 的 MIGRATION_HISTORY tuple、Stage 9 的 HISTORICAL_MIGRATION 受控来源及其普通 SQL 镜像图。不得占用别的版本号。

每个文件头部带 -- rollback: 段。第 8 号与第 10 号两个单行表的回退为 DROP TABLE；第 3 号回退只删除本阶段追加的两条 CHECK，不删表、不改 kind 取值；第 4 号由 concurrent 非事务执行器按相反顺序 `DROP INDEX CONCURRENTLY` 删除三个索引；第 25 号正向与回退均为 no-op；第 26 号回退为按 schema_name 与 table_name 两列删除本阶段登记的 16 行，不触及阶段 2 登记的八行。第 27 号只有在七张仅追加表及 backup/archive/recovery 相关表均无运行证据、且 degradation_windows 中不存在任一新增 18 项的行时，才可撤销登记、触发器、外键与新增列并把五项不可抑制/21 项 kind CHECK 一起恢复为 090300 后形状；否则退出非零，严禁只回 SQL 不回 Rust 或留下 21→3 的反向解析漂移。第 28 号只有在表 18 至表 23 均为空时才可撤销触发器/函数、删除表 22/23 与新增列，否则退出非零。第 19 至 22 号只允许在尚无任何迁移批次行时回退 DROP；已有行时回退脚本必须退出非零，迁移数据的处置只能走规格第 12.4 章与 OpsDisposalService，不得以 schema 回退删除。第 4 号是唯一 `CREATE INDEX CONCURRENTLY` 文件且必须位于 `db/migrations/platform_ops/concurrent/`；其余新建空表索引随建表迁移使用普通 `CREATE INDEX`，所有常规事务文件都不得出现 `CONCURRENTLY`。迁移会话固定 lock_timeout 5s 与 statement_timeout 30min。

---

### 4. 领域模型与关键算法

#### 4.1 核心结构体与枚举

落在 ep-platform-obs 与两个新适配 crate。

- SinkDescriptor { kind: SinkKind, root: SinkRoot, credential_ref: SecretRef, media_type: MediaType }，SinkKind 取 LocalDir、NfsSmbMount、ObjectStorage，MediaType 取 Online、Offline、None。
- SinkWritability 取 Writable、Unwritable、Unknown。
- WriteoutChannel 取 WalArchive、AttachmentIncremental、AttachmentFull、AuditEvidence、FullBackup、ConfigBundle、AttachmentBootstrap。
- ArchiveChannelState 取 Healthy、RetentionWarning、SlotInvalidated、Rebuilding、Suspended。
- BreakCause 取 SlotWalLimit、WriterStopped、WriterNotAdvancing、SinkUnwritable。
- BackupSetState 取 Planned、Running、Written、Verified、VerifyFailed、Aborted、Disposed。
- DegradationKind 的初始闭集是阶段 2 首次落下的 OFFSITE_SINK_NOT_CONFIGURED、WRITER_NOT_IN_SERVICE、PORT_NOT_IMPLEMENTED 三项；终态闭集是第 3.1 节表 3 的 21 项。Stage 14a0 先在 `crates/platform/obs/src/degradation.rs` 与 PG adapter 绑定层扩 Rust 接收域；`V20261023092500` 后扩数据库 kind CHECK 至同序 21 项并把不可抑制闭集从 090300 的 4 项扩为终态 5 项；不自建第二套标记类型。只有迁移后 real-PG exact-set 测试全绿才允许运行时注册后 18 项触发方。
- RpoBasis 取 Default15Min、DegradedToMediaRotation、NoCommitment、BootstrapNotYetAchieved、ExposureWindowOpen、WriterNotInService、ArchiveChainBroken。
- AttachmentWatermark { at: DateTime<Utc>, pending: u32, oldest_pending_committed_at: Option<DateTime<Utc>>, manifest_ref: String }。
- RecoveryPoint { db_point: Lsn 与时间、attachment_point: 水位时刻、aligned: DateTime<Utc> }。
- EnvelopeHeader { magic, format_version: u16, alg: AeadAlg（固定 Aes256Gcm）, dbek_ref: KeyRef, nonce: [u8;12], aad: ObjectIdentity }。

#### 4.2 归档通道状态机

状态与守卫条件如下，全部迁移写入 archive_channel_transitions 并同事务写审计。

- Healthy 到 RetentionWarning：守卫为 retention_ratio 大于等于 0.60。动作为开 ARCHIVE_SLOT_RETENTION_WARNING 暴露窗口，只告警，按规格第 13.4 章不触发任何备份动作。
- RetentionWarning 到 Healthy：守卫为 retention_ratio 小于 0.55，取 0.05 迟滞带以免抖动。动作为闭窗口。
- Healthy 或 RetentionWarning 到 SlotInvalidated：守卫为四类成因任一成立，即数据库侧该槽被判失效（pg_replication_slots.wal_status 取 lost）、archive-writer 或其监管的 pg_receivewal 停止、确认位点长时间不推进（confirmed_flush_lsn 在两个写出周期内不前进）、落点持续不可写。第四类的判据在监管形态下由本机 WAL 暂存目录占用达到 EP__ARCHIVE__WAL_SPOOL_MAX_GB 表达，理由见第 4.3 节末。动作为开 ARCHIVE_CHAIN_BROKEN 暴露窗口，记 broken_at 与 break_cause，同时把事务数据库 RPO 依据切到 ArchiveChainBroken。
- SlotInvalidated 到 Rebuilding：守卫为落点可写性判定为 Writable。动作按顺序为删除已失效的复制槽、重建新槽、由 backup-writer 执行一次新的全量基础备份，该次备份与每日全量串行不并发。
- SlotInvalidated 到 Suspended：守卫为落点可写性判定为 Unwritable 且持续超过暂停阈值。动作为不重建复制槽、不执行全量基础备份、保持实例可写、本机事务日志不再因该槽堆积；持续告警并在台账 detail 内标注 sub_state 为 SUSPENDED；ARCHIVE_CHAIN_BROKEN 窗口不闭合。
- Suspended 到 Rebuilding：守卫为落点已恢复 Writable，且本次暂停的阻断原因已有新的成功 preflight/report_id；若所保留基线已是 ABORTED 或 VERIFY_FAILED，preflight 还必须针对该 abort_reason 或失败校验方法证明原阻断已消除。该迁移由平台自动执行，不需人工发起；新一次尝试必须创建新的 CHAIN_REBUILD_BASELINE，不能复用失败备份 id。
- Rebuilding 到 Healthy：守卫为该次基线备份写出到落点并通过自动校验，即对应 backup_sets 行进入 Verified。动作为闭 ARCHIVE_CHAIN_BROKEN 窗口，restored_at 置值。仅重建复制槽不触发该迁移。
- Rebuilding 到 Suspended：守卫为重建过程中落点再次转为不可写，或当前 CHAIN_REBUILD_BASELINE 进入 ABORTED/VERIFY_FAILED；后一支保留该失败备份 id 与对应 report_id，不能把失败行伪装成仍在运行。

边界条件三条。一，Suspended 是阻断原因未消除期间的稳态，没有无证据的自愈路径；客户修复落点或失败基线的根因并产生第一个成功 preflight 后，平台才自动走唯一出口，阻断仍在时不反复重建复制槽或执行基础备份；界面与台账文案不得在 preflight 通过前出现“正在恢复”一类表述。二，处于 SlotInvalidated、Rebuilding 与 Suspended 三态期间，v_rpo_status 的事务数据库行一律按 ArchiveChainBroken 展示，不得展示 15 分钟默认承诺。三，该状态机不因 archive-writer 重启而复位，状态持久化在数据库单行上，进程启动时先读该行再决定行为。

#### 4.3 落点可写性判定

判定不依赖人工发起，由写出组件按规格第 13.4 章三项最低要求中“写入失败可被平台检测”这一项持续执行。

算法。每 EP__SINK__PROBE_INTERVAL_SECONDS 执行一次探针：为本次探针生成不可复用的批次唯一 key，以 CREATE_NEW 语义写入小对象，再读回并逐字节比对；对象存储必须带 `If-None-Match: *`，Windows/SMB/NFS 目录必须使用等价的排他创建，已存在即失败，探针永不覆盖或删除旧对象。真实写出的成功与失败同样计入判定序列。连续 EP__SINK__UNWRITABLE_AFTER_FAILURES 次失败判为 Unwritable，连续 EP__SINK__WRITABLE_AFTER_SUCCESSES 次成功判为 Writable。判定翻转只更新 offsite_sinks 的 writability 与 platform_audit、指标，不新增 Outbox 事件。

离站副本防删保护。全部备份、归档、附件、水位与探针对象的 key 固定为 `<channel>/<deployment_id>/<batch_or_period_id>/<object_id>/<content_sha256>`；其中 batch_or_period_id 与 object_id 均不可复用，manifest 也生成自己的唯一版本 key。写出 API 只允许 CREATE_NEW，已有 key、条件写前置失败或服务端把 create 降格为覆盖时，该批失败并打开不可抑制的 `OFFSITE_COPY_PROTECTION_MISSING`，不得换用覆盖写重试。日常 writer 身份只可列举本部署前缀、创建新对象及完成校验所必需的读取，明确禁止删除、覆盖、重命名、版本清理、ACL/策略/生命周期管理和存储管理；恢复读取使用第二个平时封存的只读身份；到期处置使用第三个独立身份，只在双人审批和重新认证通过后临时解封，并按批准清单删除精确 key/版本，不得写新对象或管理策略。三身份不得共用账户、credential ref 或可相互代入的组。

后端唯一口径如下。

- OBJECT_STORAGE：IAM 显式拒绝 writer 的 DeleteObject、DeleteObjectVersion、覆盖写、重命名等价操作、BucketPolicy/ACL/Lifecycle/Versioning 管理与版本清理；写入必须携带 `If-None-Match: *`。平台以 writer 身份对已创建测试对象执行覆盖、删除和策略修改负向探针，三者都必须被拒。
- LOCAL_DIR 与 Windows/SMB：使用独立服务账户；共享与 NTFS DACL 对 writer 显式拒绝 DELETE、DELETE_CHILD、WRITE_DAC、WRITE_OWNER 与对既有文件的写入，适配器使用 `CreateFile(..., CREATE_NEW)`；除客户存储管理员外不得从继承 ACL 获得删除或改权能力。平台以 writer 身份实测覆盖、删除、重命名、chmod/chown/ACL 修改全部被拒。
- NFS：只有存储端 NFSv4 ACL 或等价机制能够把 ADD_FILE/创建新对象与 DELETE、DELETE_CHILD、WRITE_ACL、WRITE_OWNER、改属主和既有文件写入分离，并通过同一组负向探针时才合格。普通 POSIX/NFS 导出若“可写目录”同时使 writer 可删除、重命名或改权，即使写入正常也不满足保护门，必须打开 `OFFSITE_COPY_PROTECTION_MISSING`。

attestation 与窗口。部署配置缺任一身份、三个身份不互斥、服务端策略证据缺失，或任一删除/覆盖/重命名/改权/策略管理负向探针未被拒，均把 offsite_sinks 的 append_only_attested 置 false、append_only_probe_result 置 FAIL，并打开 `OFFSITE_COPY_PROTECTION_MISSING`。只有证据齐全且全部负向探针重新通过才写 PASS 并自动关窗；窗口不可抑制，且发布门失败。窗口只陈述副本防删保护缺失：连续写出、正常读回与获批恢复仍按实际能力运行并如实降级，不得因保护失败伪造“落点不可写”。本控制是最小权限的防删/防覆盖措施，不是 WORM、对象锁或不可变存储；客户存储管理员、云账户根管理员或另一台机器的本地管理员仍可绕过，交付说明和合同必须逐字披露该剩余风险。

暂停阈值。落点判为 Unwritable 起，若在 EP__ARCHIVE__SUSPEND_AFTER_MINUTES 内未恢复，则通道由 SlotInvalidated 转 Suspended。取值 30 分钟，是两个 15 分钟写出周期，理由是短于两个周期的不可写属正常抖动，不应立即宣布无恢复点。

边界条件。落点 media_type 为 None 时判定不执行，直接开 OFFSITE_SINK_NOT_CONFIGURED 窗口且该窗口不可抑制；media_type 为 Offline 时探针仍执行但结果不用于 RPO 判定，该部署的 RPO 依据固定为 DegradedToMediaRotation。落点不可写的背压形态由复制槽堆积改为本机 WAL 暂存目录堆积，理由是 pg_receivewal 以本地落盘为准推进确认位点，不把位点确认压到落点写出成功之后；暂存占用达到 EP__ARCHIVE__WAL_SPOOL_MAX_GB 即判归档链断裂并走第 4.2 节的 SlotInvalidated 分支。该改动对单机形态是净收益：pg_wal 不再因落点不可写而增长，数据库因复制槽滞留失去写入能力这条路径被移除，而落点未收到的事务日志在整机失效时本就不可用，RPO 口径不变。

#### 4.4 附件正文写出点水位推进算法

这是规格第 13.4 章附件与元数据恢复点对齐条的实现依据，也是附录 A.6 附件一致性判据成立的前提。

定义。水位 W 是一个时刻，满足在 W 之前提交的全部附件元数据，其对应正文都已完成向服务器之外落点的写出并通过校验。

步骤。
1. archive-writer 经 IPC 向 core-server 请求写出范围，入参为上次水位 W_prev 与上次已处理的最大元数据提交序，出参为一个按提交序升序的对象流，每项含 attachment_object_id、metadata_committed_at、content_ref、content_sha256、content_size、key_domain_ref。
2. archive-writer 维护 pending 集合，键为 attachment_object_id，值为该对象的元数据提交时刻与写出状态。
3. 对每个未写出对象，按其法人密钥域内的原密文原样写出到落点，不二次施加部署级备份加密（该密文已按规格第 7.5 章加密），写出后按 content_sha256 读回校验。
4. 每完成一批，令 T_min 为 pending 中尚未完成对象的最小 metadata_committed_at。若 pending 为空，W 推进到本批已知的最大 metadata_committed_at；否则 W 推进到 T_min 的前一微秒。
5. W 只增不减。W 与 pending 计数、oldest_pending_committed_at 一并写入落点上的水位 manifest 对象，并经 IPC 上报入 attachment_watermarks。
6. manifest 对象自身以部署级备份加密写出，理由是它含元数据提交时刻与对象标识；其内容必须只凭落点即可读出，不依赖已失效的原服务器。

边界条件。
- 引导窗口内，bootstrap_state 为 RUNNING，不产生 W，v_rpo_status 的附件行按 BootstrapNotYetAchieved 展示；引导完成后 W 自引导起点开始推进。
- 单个对象连续写出失败达到重试上限时，该对象保留在 pending 中，W 因此停滞，ATTACHMENT_INCREMENTAL_WRITEOUT_OVERDUE_OR_FAILED 窗口打开。这是有意行为：宁可水位停滞并暴露，也不跳过对象使恢复点上出现元数据在而正文不在。
- 首版无附件物理删除，已写删除标记的对象其正文仍需写出与保留，不从 pending 中剔除。
- core-server 不可用期间，archive-writer 不能取得新的写出范围，但已在 pending 中的对象继续写出，W 继续在已知范围内推进；上报进本地 spool，恢复后补写。

#### 4.5 恢复点对齐与整机失效恢复

恢复点对齐。W_db 取落点上已写出并通过校验的 WAL 归档所能支撑的最后一致时刻；W_att 取落点上水位 manifest 内的 W。恢复点 R 等于两者较早的一个，把事务数据库以 recovery_target_time 等于 R 回退到该点。该规则保证任一恢复点上元数据存在则正文必然存在。

整机失效恢复步骤，恢复模式下按规格第 13.1 章使用扣除操作系统预留后的全部可分配量。
1. 分片取回与双人控制完成，恢复材料在现场可用。此步不计入 RTO，但其实际耗时单独留证。
2. backup-writer 以 EP__BACKUP__MODE 取 restore 启动，读落点索引，解封部署级备份加密密钥。
3. 读出 W_att 与最近一次 Verified 的全量基础备份及其后的 WAL 归档链，计算 W_db 与 R。
4. 解密并展开基础备份，回放 WAL 至 R。
5. 与第 4 步并行，流式写入附件正文：每个对象在写入过程中计算 sha256，写完即经 IPC 上报 (attachment_object_id, sha256, size)。该实现满足附录 A.6 允许流式计算而不要求恢复后另跑全量读取的口径。
6. core-server 以恢复档配置启动，逐条比对上报的校验和与 platform_file 元数据记录，输出逐条比对结论、未通过条目清单与该校验实际耗时。任一条不满足即本次恢复或演练不达标。
7. 恢复审计证据存储、配置、证书、模块包、低代码规则包与基础设施定义。
8. job-worker 以恢复验收模式经阶段 9a 交付的 ep-platform-recon 的 ReconExecutor::run 执行规格第 17.3 章全部强制不变量校验，run_kind 取 RECOVERY_ACCEPTANCE，覆盖面宽于每日校验与关账前校验；分批规模、单批时限与单查询内存及临时空间上限按附录 A.6 演练实测冻结的恢复模式取值。
9. 重建归档与备份通道，产出新的基线备份并通过自动校验。
10. 汇总 rto_seconds、rpo_db_seconds、rpo_attachment_seconds、decrypt_seconds、attachment_check_seconds、invariant_check_* 六组取值写入 recovery_drills。

边界条件。W_att 缺失即返回 PLATFORM.RECOVERY.WATERMARK_UNAVAILABLE，恢复只能到 W_db 且必须在恢复报告中显式记录附件缺失范围，本次演练判定不达标。落点上只有本机副本时该演练不成立，判定为未达标。未经分片恢复材料解密的演练按未验证处理，不得以明文副本或原运行环境在线密钥完成。

#### 4.6 两个 RPO 取值与依据的判定算法

输入为 offsite_sinks 当前行、archive_channel 当前行、三类写出的最近一次结果与周期、attachment_watermarks 的 bootstrap_state、degradation_windows 中的活动条目、deployment_records 当前行。输出为两行，target 分别为 DATABASE 与 ATTACHMENT。

依据的严劣序，由劣到优固定为：NoCommitment、WriterNotInService、ArchiveChainBroken、BootstrapNotYetAchieved、DegradedToMediaRotation、ExposureWindowOpen、Default15Min。取值算法为对每个 target 收集其全部成立的依据，取严劣序中最靠前的一个作为展示依据。

各依据的成立条件。NoCommitment 在 media_type 为 None 时对两个 target 同时成立。WriterNotInService 在 WRITER_NOT_IN_SERVICE 窗口活动时对两个 target 同时成立，该窗口的触发条件是客观事实而非配置漏项，即任一写出进程未在运行或连续两个写出周期无上报。ArchiveChainBroken 在 archive_channel.state 属三个断链态之一时只对 DATABASE 成立。BootstrapNotYetAchieved 在 bootstrap_state 非 DONE 时只对 ATTACHMENT 成立。DegradedToMediaRotation 在 media_type 为 Offline 时对两个 target 同时成立，effective_seconds 取 rotation_period_minutes 乘 60。ExposureWindowOpen 在该 target 对应的写出超期或失败窗口活动时成立，effective_seconds 取当前时刻减该 target 最近一次 OK 的 writeout_runs.finished_at。Default15Min 在其余情形成立，effective_seconds 取 900。

对外披露取值按规格第 13.3 章取两者较大值。台账必须同时展示两行且各自标注依据，不得只展示较优的一个，也不得对任一方在降级或未达成状态下展示默认承诺值。台账取值与部署记录取值不一致时按较差一方展示。

#### 4.7 部署级备份加密

对象格式为 header 加密文加认证标签。header 明文可读，含 magic、format_version、alg 固定 AES-256-GCM、dbek_ref 含版本号、nonce 12 字节、aad 为对象身份三元组（channel、period_seq 或 backup_set_id、对象相对路径）。DEK 为每对象随机 32 字节，由 DBEK 以 AES-256-GCM 包裹后放入 header 的 wrapped_dek 字段，即信封加密。DBEK 为实例级，由部署方统一持有，载体只有内置 KMS 与客户自有硬件密码机两种，不属于任一法人密钥域。

施加范围。事务日志归档、每日全量备份、审计证据存储副本、配置与证书与模块包与低代码规则包与基础设施定义副本、附件水位 manifest，一律施加。附件正文保持其法人密钥域内的原密文原样写出，不重复施加。两类合起来使落点上不存在任何明文物理副本。

必须明写的结论。该加密只阻断落点侧的外部可读，不恢复副本上的法人隔离。同时持有 DBEK 与落点读取权限者可读到除行内敏感字段外的全部法人业务数据。该结论按规格第 21.21 章写入交付说明与客户合同，界面与文档不得使用受控读取、法人隔离、等效或已满足一类措辞。

#### 4.8 复制槽保留量判定与未知复制会话检出

保留量判定。ops-agent 与 core-server 分别按 EP__OPS__WAL_RETENTION_SAMPLE_PERIOD_SECONDS 采样 pg_replication_slots 的 safe_wal_size 与 wal_status，以及 pg_wal 目录占用，算出 retention_ratio 等于 retained_bytes 除以 max_slot_wal_keep_bytes，写入 wal_retention_samples。达到 0.60 触发第 4.2 节的 RetentionWarning 迁移。wal_status 取 lost 即判槽失效。规格明确该告警只提示保留量正在堆积，不触发任何备份动作，实现中不得挂接任何自动备份。

复制交叉核对复用上面这次 30 秒采样，不另建通道。两个写出进程建立复制连接时必须把 PostgreSQL `application_name` 分别设为 `archive-writer` 与 `backup-writer`。core-server 通过既有只读分析池在同一次采样读取 `pg_replication_slots` 与 `pg_stat_replication`：数据库侧槽集合是前者全部行的 `slot_name`，未登记逻辑槽也不忽略；会话集合是后者全部行的 `(pid, usename, application_name)`。报告侧只取 `platform_ops.replication_reports.outcome='OK'` 的行，对同一对象按 `(occurred_at, report_id)` 取最后一条：`SLOT_CREATED/SLOT_INVALIDATED` 以 `(writer_process, db_role, slot_name)` 恢复活动槽名集合，`CONN_ESTABLISHED/CONN_CLOSED` 以 `(writer_process, db_role, backend_pid)` 恢复 `(backend_pid, db_role, writer_process)` 活动会话集合。`spooled=true` 的补写仍以 `occurred_at` 排序。合法映射只有 `archive-writer↔ep_archiver` 与 `backup-writer↔ep_backuper`；交叉组合直接是 `MISMATCHED`。槽事件缺 `slot_name`、连接事件缺 `backend_pid`、三个输入中任一次查询未完整则是 `NO_RESULT`。

两侧槽名集合与会话三元组集合均精确一致为 `MATCHED`；数据库侧存在未上报槽/会话、报告侧存在数据库已不存在的活动对象、或出现非法进程/角色映射为 `MISMATCHED`；查询超时、错误、无权限或任一输入不完整为 `NO_RESULT`。`MATCHED` 与 `MISMATCHED` 都把 `archive_channel.replication_check_no_result_streak` 归零，后者同轮告警并按第 12.5 章审计；`NO_RESULT` 递增 streak，连续第二次时开 `REPLICATION_CROSSCHECK_NO_RESULT` 暴露窗口，写出进程照常运行，下一次 `MATCHED` 或 `MISMATCHED` 关闭窗口。每轮把结论、时点与清洗后的错误码写入 `archive_channel`，不存原始错误文本。

专用交叉核对子系统继续删除：不恢复 `platform_ops.replication_crosscheck_runs` 表与迁移、GET `/replication-crosschecks` 端点、`ep_replication_crosscheck_age_seconds` 指标、`EP__OPS__CROSSCHECK_PERIOD_SECONDS`、`EP__OPS__CROSSCHECK_STATEMENT_TIMEOUT_MS` 或独占连接；只恢复三态语义及其第十九个台账 kind。只读分析池交互式上限仍为 10。采样式检出只能覆盖跨过采样时点持续存在的未知槽与会话，起止都落在同一 30 秒周期内的连接可能漏检；该局限按第 21.21 章写入交付说明，不得表述为完整阻断或无遗漏检测。

#### 4.9 备份集状态机与暂存缓冲

迁移。Planned 到 Running 守卫为取得 backup_runner_slot 的乐观锁；Running 到 Written 守卫为流式写出完成且落点返回成功；Written 到 Verified 守卫为第 3.1.1 节按 kind 冻结的必需方法集合恰好一次且全 PASS；Written 到 VerifyFailed 守卫为该集合完整但任一方法 FAIL，动作为该备份不计入有效备份并告警；Running 到 Aborted 守卫为本机暂存缓冲占用达到 EP__BACKUP__SPILL_MAX_BYTES 或落点转不可写，动作为中止该次备份并告警，且不得挤占连续归档本机保留子项。Verified、VerifyFailed、Aborted 到 Disposed 是唯一处置边，只能在 OpsDisposalService 已取得销毁证明后执行；Disposed 不可再迁移。

暂存缓冲。按规格附录 A.3，本机不为全量基础备份预留可容纳整份的空间，backup-writer 以流式方式写出，本机只承载写出期间的暂存缓冲，缓冲占用达到子项取值时中止并告警。因此本机不承诺保留任何可直接读回的全量备份副本，整机失效恢复一律从落点副本进行。

备份自动校验。方法全集有四种：MANIFEST_CHECKSUM（对落点上每个对象逐个比对清单校验和）、DECRYPT_READBACK（读回并以恢复材料解密抽验固定比例的数据块，比例取 100% 于认证演练、取配置值于生产）、PG_VERIFYBACKUP（对基础备份的 backup_manifest 执行标准校验）、ATTACHMENT_CHECKSUM（对附件全量写出结果逐对象比对）；单个 kind 的精确必需子集以第 3.1.1 节为准，不把不适用方法伪造为 PASS。校验不建立到生产事务数据库实例的连接，不占用连接额度与复制槽。校验结论按规格第 13.4 章写入审计。

#### 4.10 处置执行与 DisposalPort 实现

端口由阶段 3b 在 crates/platform/file/src/port/disposal.rs 定义，含 DisposalRequest、DisposalReceipt 与 DisposalPort 三项，处置受理路由亦由阶段 3b 注册。本阶段提供其唯一实现 OpsDisposalService，位于 crates/platform/obs/src/disposal.rs，在 apps/core-server/src/wiring/ 与 apps/job-worker/src/wiring/ 两个目录内注入。阶段 3b 至阶段 13 之间该端口不注入任何实现，物理删除请求经该受理路由以 PLATFORM.DISPOSAL.NOT_DELIVERED 直接拒绝，category 取 BUSINESS_CONFLICT，HTTP 409，不可重试，同时有一条 kind 取 PORT_NOT_IMPLEMENTED 且 subject 取 DisposalPort 的降级窗口持续活动，本阶段注入实现后关闭该窗口；该错误码随阶段 3b 的受理路由登记，不在本文第 5 节的本阶段新增错误码清单内。阶段 2 的密钥销毁实际执行、阶段 3b 的附件与审计证据物理删除、阶段 13 的扩展对象物理删除路径一律指向该实现，各阶段不自建第二条销毁路径。

触发面。只由 ops 专用路径与 ops 专用账号触发，不在 /api/v1/platform 前缀下对外暴露，因此不进入第 5 节端点表。

执行前置，逐项校验，任一不成立即拒绝执行并写审计。一，DisposalRequest.approval_ref 对应的审批链已通过。二，DisposalRequest.second_approver_id 与申请人不同，落实双人控制，申请人不可自审。三，DisposalRequest.reauth_ref 为规格第 12.1 章要求的重新认证凭证且在有效期内。四，处置身份与 writer、restore 身份三者互斥，且仅在本次批准窗口临时解封；不满足时返回 `PLATFORM.SOD.SELF_APPROVAL_FORBIDDEN` 或 `PLATFORM.AUTHZ.REAUTH_REQUIRED`。五，落点可达且精确目标 key/版本存在，否则返回 PLATFORM.OFFSITE_SINK.UNWRITABLE；不要求 writer 身份具有删除权。

执行范围。DisposalRequest.scope 取 AttachmentObjects、KeyDomain、BackupSets、ExtTables 四者之一，object_refs 为该范围内的对象引用清单。密钥销毁走 KeyDomain，到达备份保留期的备份集销毁走 BackupSets，两者与附件正文一样必须由独立 disposal 身份把落点上的全部精确 key/版本在同一次处置内删除并逐项回读确认不存在；writer 与 restore 身份不得参与删除。任一目标遗漏、部分成功或身份复用时整批证明不成立并写失败审计，不得把“覆盖新空对象”当成销毁。

执行后置。同一事务内写 platform_audit.audit_events 并生成销毁证明对象，返回 DisposalReceipt，其 disposal_plan_id 回填请求取值、disposed_count 为实际处置对象数、certificate_ref 为销毁证明对象引用、executed_at 为执行完成时刻。

边界条件。处置不可逆，本阶段不提供撤销路径。处置执行不阻塞事务日志接收与附件正文写出，不改变归档通道状态机，也不闭合任何暴露窗口。

#### 4.11 本阶段的指标登记

本阶段注册且只注册基线早已具名的三项指标：`ep_archive_write_lag_seconds`、`ep_attachment_write_lag_seconds`、`ep_backup_last_success_timestamp_seconds`；类型、标签与填充语义以 `docs/metrics-catalog.md` 为唯一值。`ep_degradation_windows_open` 由阶段 2 注册并填充，本阶段不重复登记；其既有按活动 ledger 行与 kind 标签计数的实现，在 Stage 14a0 Rust 接收域和 `092500` SQL CHECK 同序扩成 21 项后自然覆盖新增 kind，不建立第二个指标或第二条填充路径。`ep_db_pool_connections` 与 `ep_db_statement_duration_seconds` 由阶段 1 注册，本阶段不重复登记。原裁定 C-22 归本阶段的 `ep_replication_crosscheck_age_seconds` 随第 4.8 节交叉核对子系统一并撤销，阶段 2 原先登记的 `ep_db_replication_crosscheck_age_seconds` 早已作废，两侧均不再登记，阶段 2 的取数函数与独占连接同时撤销。`docs/metrics-catalog.md` 的唯一性校验由阶段 1 的 xtask 执行，本阶段登记集合必须与上述三项精确相等。

#### 4.12 历史数据迁移工具、模块写入者与切换状态机

名字与安全边界先冻结。`ep-migrate` 只做数据库 DDL 与 schema 版本迁移，既有五个子命令与六个退出码不变；本节新工具固定名为 `ep-data-migrate`，只做客户历史数据迁移。它在 Windows Server 上由具备数据迁移权限的客户人员按需启动，不注册服务、不监听端口、不直连目标 PostgreSQL、不取得 `ep_migrator` 或 `ep_app_rw` 凭据。工具只从生产签名覆盖的部署清单 schema v1 读取 `employee_api_origin`，其值必须是无路径/查询/片段的 HTTPS origin；经第三方反向代理调用公开 `/api/v1/platform/data-migration-batches` 受控 API，强制验证证书链、SAN 主机名与清单 host，禁止重定向、系统代理、回环/localhost、直连 core-server:8080、命名管道或命令行/模板自填 URL。服务端再调用模块唯一写入者。来源连接一律只读：XLSX/CSV 只读打开，ODBC 要求 DSN 驱动报告只读事务且以一条写入负样例确认被源端拒绝，文件来源只读清单与正文，HTTPS API 只允许模板签名范围内的域名、端口、路径和字段且只发 GET；重定向到白名单外、TLS 校验失败或响应字段超集均拒绝。来源凭据只引用 Windows Credential Manager 中的条目名，不进入模板、命令行、日志、错误队列或数据库。

版本化模板固定为签名 TOML，schema 版本首版只取 `1`。顶层必含 `template_code`、`template_version`、`source_kind`、`source_schema_fingerprint`、`object_mappings[]`、`delta_watermark`、`reconciliation_rules[]` 与 `allowed_endpoint`（仅 HTTPS_API）；每个对象映射必含 `module_code`、`object_type`、来源主键、字段映射、清洗规则、必填校验以及 `legal_entity_id`、`security_level`、`key_domain_id`、`retention_policy_code` 四项显式赋值。清洗规则首版只允许 trim、大小写规范化、日期格式解析、十进制定标、枚举映射、空值替换、正则拒绝与静态查找表八种声明式操作，不允许脚本、宏、表达式求值、网络调用或任意 SQL。模板与旁签文件固定落在 `C:\EP\data-migration-templates\<template_code>\<template_version>.toml` 与同名 `.sig`；目录断开继承，仅客户实施负责人和 core-server 服务虚拟账户可读，只有客户实施负责人可写。该路径是产品协议常量，不新增配置键。模板由客户实施负责人签名，core-server 在批次创建时从该目录按代码与版本读取、以受信签名清单验签并比对请求摘要；同一 `template_code + template_version` 的 SHA-256 不得变化，变化必须升版本。数据库只保存代码、版本与摘要，不保存模板正文。完整试运行与正式执行分别记录同一模板摘要和源 schema fingerprint，不一致返回 `PLATFORM.DATA_MIGRATION.SOURCE_CHANGED`。

首版 `MigrationObjectKind` 取下列 25 个封闭值；代码值就是表内字符串，bundle 含其根记录、从属行、关系与附件，但不越过模块边界：

| module_code.object_type | 唯一写入者 |
|---|---|
| `mdm.customer_bundle`、`mdm.supplier_bundle`、`mdm.material_bundle`、`mdm.product_bundle`、`mdm.warehouse` | ep-app-mdm |
| `cpq.price_list_bundle` | ep-app-cpq |
| `clm.contract_bundle` | ep-app-clm |
| `sales.sales_order_bundle`、`sales.sales_return_bundle` | ep-app-sales |
| `procure.purchase_order_bundle`、`procure.goods_receipt_bundle`、`procure.purchase_return_bundle`、`procure.payment_request_bundle` | ep-app-procure |
| `inventory.stock_opening`、`inventory.stock_history` | ep-app-inventory；期初必须复用阶段 8 的 `MIGRATION_OPENING` |
| `ledger.opening_balance`、`ledger.historical_voucher` | ep-app-ledger；期初必须复用阶段 9a 的 opening-balance batch |
| `finance.open_items_opening`、`finance.cash_account_opening` | ep-app-finance；复用阶段 10 的应收、应付、预收、预付与资金账户期初通道 |
| `invoice.sales_invoice_bundle`、`invoice.purchase_invoice_bundle` | ep-app-invoice |
| `project.project_bundle` | ep-app-project，含项目任务与从属附件 |
| `service.customer_complaint_bundle`、`service.equipment_bundle`、`service.work_order_bundle` | ep-app-service；设备固定写 `source=MIGRATION` 与 `migration_batch_no` |

##### 4.12.1 静态 target projection catalog

本表同时是 `V20261023092600` 生成 25 个 SQL 分支、Rust writer 注册表与快照测试的唯一输入，不允许实现方再选 relation、根 id、子表、排序键或反向通道。记号固定如下。

- `A0` 是全部 APPLY 的同批 provenance：记录转 VALIDATED 时由服务端写 catalog 固定 `target_object_type` 与新 UUIDv7 `target_id`，延迟图证明对应同法人 relation 尚无该 id，且 `ux_data_migration_records_target_reservation` 保证全库迁移记录不重复预留；owner writer 只能用该 id 建根，receipt 再以同批长 FK 指回记录。除 catalog 明列的自然来源字段外，不给业务表添加通用 `migration_batch_id`、`migration_record_id` 或 JSON provenance 伪列。
- `row_v1(alias)` 固定为该静态分支所列 relation 的 `to_jsonb(alias)` 删除 `row_version,updated_at,updated_by` 三个运行期可变元数据键；`id,legal_entity_id,security_level,data_scope_tags,created_at,created_by` 与全部业务列必须保留。SQL 文件中 relation、alias、join 与 `ORDER BY` 全是生成后的字面量，不得把 relation 名作为参数、拼接动态 SQL、扫描 `information_schema` 或调用通用 JSON 目标函数。
- `set_v1(relation,predicate)` 固定以该行登记的 `id` UUID bytes 升序聚合 `row_v1`；空集精确为 `[]`。一个 bundle 的 APPLY 投影恰为 `{"schema_version":1,"object_kind":<literal>,"target_relation":<literal>,"root":row_v1(root),"children":{<catalog列出的relation:set_v1>}}`，键按 RFC 8785 规范化后取 SHA-256。根与子行的密文、key ref、attachment object id 均进入 hash，正文不进入；未在该行登记的 workflow、approval、audit、Outbox、派生缓存或共享余额表不得混入。
- `R0` 是全部 REVERSE 的强制 provenance：owner 既有变更、取消、冲销或更正命令与 REVERSE receipt 同事务写一条不可变 `platform_audit.audit_events`，其 `event_id=receipt.id`、`action='DATA_MIGRATION_REVERSED'`、`object_type/object_id` 指原 APPLY 根，`after` 恰为 `{schema_version:1,data_migration_record_id,batch_id,apply_receipt_id,owner_effect_object_type,owner_effect_id}` 六键。交易对象只能走下表具名 owner 通道并以新取消/冲销/更正 fact 为 target；可变主数据必须以具名 version/change fact 为 target并投影根 after-image。唯一例外是采购订单、付款申请、资金账户三根：它们以另一条具名 `platform_audit.audit_events` owner change fact 为 target并投影根 after-image；owner event 的 event_id 必须不同于 receipt/R0 id，且与 R0 同法人、同 occurred_at，R0.after.owner_effect_object_type 固定为 `platform_audit.audit_events`、owner_effect_id 固定为 owner event id。REVERSE 的 canonical projection 是该行登记的 owner effect 投影加 R0 事件的 `{event_id,action,object_type,object_id,after,occurred_at}`，owner_effect_at 精确取 occurred_at。普通 SQL 改状态、回删原记录、把任意旧事实挂到 receipt、复用 R0 充当 owner event，或只写普通审计而无匹配 owner 状态效果均拒绝。

| MigrationObjectKind | owner / writer | APPLY relation 与根 id | 同批 provenance | canonical APPLY projection；唯一 REVERSE owner 通道与 effect projection |
|---|---|---|---|---|
| `mdm.customer_bundle` | mdm / ep-app-mdm | `mdm.customers.id=target_id` | A0 | APPLY children：`customer_contacts.customer_id`、`customer_addresses.customer_id`、`customer_invoice_profiles.customer_id`、`customer_attachments.owner_id`。REVERSE：`reverse_migrated_master(CUSTOMER)` 复用 DEACTIVATION change request，target=`mdm.record_versions.id`；投影含该 version、其 `change_request_id` 行与 `customers` 根 after-image，再加 R0。 |
| `mdm.supplier_bundle` | mdm / ep-app-mdm | `mdm.suppliers.id=target_id` | A0 | APPLY children：`supplier_contacts.supplier_id`、`supplier_payment_profiles.supplier_id`、`supplier_qualifications.supplier_id`、`supplier_price_records.supplier_id`、`supplier_leadtime_records.supplier_id`、`supplier_risk_records.supplier_id`、`supplier_attachments.owner_id`；`supplier_qualification_attachments.owner_id` 取本根 qualification ids，`supplier_risk_record_attachments.owner_id` 取本根 risk ids。REVERSE 同上取 `SUPPLIER`，target/version projection 同构并加 R0。 |
| `mdm.material_bundle` | mdm / ep-app-mdm | `mdm.materials.id=target_id` | A0 | APPLY children：`material_attachments.owner_id`。REVERSE：`reverse_migrated_master(MATERIAL)`，target=`mdm.record_versions.id`，含 change request、root after-image、R0。 |
| `mdm.product_bundle` | mdm / ep-app-mdm | `mdm.products.id=target_id` | A0 | APPLY children：`product_material_links.product_id`、`product_attachments.owner_id`。REVERSE：`reverse_migrated_master(PRODUCT)`，target=`mdm.record_versions.id`，含 change request、root after-image、R0。 |
| `mdm.warehouse` | mdm / ep-app-mdm | `mdm.warehouses.id=target_id` | A0 | APPLY 无 children。REVERSE：`reverse_migrated_master(WAREHOUSE)` 只在既有非零库存/未结单据守卫通过后执行 DEACTIVATION，target=`mdm.record_versions.id`，含 change request、root after-image、R0。 |
| `cpq.price_list_bundle` | cpq / ep-app-cpq | `cpq.price_lists.id=target_id` | A0 | APPLY children：`price_list_lines.price_list_id`、`price_list_customer_links.price_list_id`。REVERSE：`reverse_migrated_master(PRICE_LIST)` 复用 mdm change request 与版本通道，target=`mdm.record_versions.id`，投影含 change request、`price_lists` 根 after-image 与 R0。 |
| `clm.contract_bundle` | clm / ep-app-clm | `clm.contracts.id=target_id` | A0 | APPLY children：`contract_lines.contract_id`、`contract_terms.contract_id`、`contract_milestones.contract_id`、`contract_obligations.contract_id`、`contract_payment_schedules.contract_id`、`contract_attachments.contract_id`、`contract_annotations.contract_id`、`contract_versions.contract_id`、`contract_merge_links` 中 source 或 target 等于根；审批、签章、validation 与 derivation evidence 排除。REVERSE：`reverse_migrated_contract` 对 DRAFT/REJECTED 走既有 VOID、对在制合同走既有 termination、已终态只追加 migration reversal version；target=`clm.contract_versions.id`，投影含该 version、合同根 after-image、R0，未闭合影响项时 plan 必须拒绝。 |
| `sales.sales_order_bundle` | sales / ep-app-sales | `sales.sales_orders.id=target_id` | A0；合同 id/version 必须已存在并与订单来源快照一致 | APPLY children：`sales_order_lines.sales_order_id`、`delivery_schedules.sales_order_id`、`delivery_confirmations.sales_order_id`、确认行经本根 confirmation ids、`sales_order_versions.sales_order_id`、`sales_order_changes.sales_order_id`、change lines 经本根 change ids；validation evidence 排除。REVERSE：`reverse_migrated_sales_order` 复用零交付 CANCEL 或有履约 CLOSE/order-change，已交付数量必须先经现有 sales-return 通道完整冲回；target=`sales.sales_order_changes.id`，投影含 change、change lines、由该命令生成的 sales returns 及 R0。 |
| `sales.sales_return_bundle` | sales / ep-app-sales | `sales.sales_returns.id=target_id` | A0；所引订单、交付行与 current-live capture 必须已存在 | APPLY children：`sales_return_lines.sales_return_id`、`return_line_delivery_links.sales_return_id`、`return_line_capture_allocations.sales_return_id`、`exchange_links` 经本根 return line ids。REVERSE：`reverse_migrated_sales_return` 对 DRAFT/SUBMITTED 走既有 CANCEL；REGISTERED/CLOSED 必须走现有补偿交付确认、库存出库与 ledger/costing current-live 反向链，target=`sales.delivery_confirmations.id`，投影含确认头行及 R0；不得把已登记退货改回草稿。 |
| `procure.purchase_order_bundle` | procure / ep-app-procure | `procure.purchase_orders.id=target_id` | A0 | APPLY children：`purchase_order_lines.purchase_order_id`、line batches 经本根 line ids、`purchase_order_payment_plans.purchase_order_id`、`purchase_order_attachments.owner_id`。REVERSE：`reverse_migrated_purchase_order` 按 Stage 7 §4.2.8 的逐态唯一 VOID/CLOSE/终态保持映射执行；新建独立 owner audit，action=`PROCURE_PURCHASE_ORDER_MIGRATION_REVERSED`、object=`procure.purchase_orders` 原 APPLY 根，before/after 各恰含 schema_version/row_version/status，真实变更版本+1、终态保持版本不变。receipt target 固定 `platform_audit.audit_events.event_id=receipt.target_id=owner_audit.event_id`，owner event 与 R0 id 不同且同法人同 occurred_at；R0.after 指回该 owner event。owner effect projection 精确为 `{owner_audit:row_v1(owner_audit),purchase_order_after:row_v1(purchase_orders)}` 再加 R0。已有收货、采购发票或付款占用未被对应 bundle 反向闭合时 plan 与 owner 守卫都拒绝。 |
| `procure.goods_receipt_bundle` | procure / ep-app-procure | `procure.goods_receipts.id=target_id` | A0；所引采购订单/行/批次必须存在 | APPLY children：`goods_receipt_lines.goods_receipt_id`、serials 与 costings 经本根 receipt line ids、`goods_receipt_attachments.owner_id`、`receipt_rejections.goods_receipt_id` 及其 attachments 经 rejection ids。REVERSE：`reverse_migrated_goods_receipt` 复用 PURCHASE_RETURN 命令完整冲数量、库存、GRNI 与凭证，target=`procure.purchase_returns.id`，投影含 return 头行/serials/附件及 R0。 |
| `procure.purchase_return_bundle` | procure / ep-app-procure | `procure.purchase_returns.id=target_id` | A0；原收货/成本 effect 与条件 credit note 必须存在 | APPLY children：`purchase_return_lines.purchase_return_id`、serials 经本根 line ids、`purchase_return_attachments.owner_id`，以及 `supplier_quality_records` 中 `source_type='PURCHASE_RETURN' and source_doc_id=root.id`。REVERSE：`reverse_migrated_purchase_return` 复用补偿 GOODS_RECEIPT 通道恢复数量/库存/GRNI，已开票段另经既有 invoice reversal 反向；target=`procure.goods_receipts.id`，投影含 receipt 头行/serials/costings 及 R0。 |
| `procure.payment_request_bundle` | procure / ep-app-procure | `procure.payment_requests.id=target_id` | A0 | APPLY children：`payment_request_lines.payment_request_id`、`payment_request_attachments.owner_id`；共享 `payable_reservations` 不进 hash，由 owner 守卫重算。REVERSE：`reverse_migrated_payment_request` 按 Stage 7 §4.2.8 的逐态唯一 VOID/WITHDRAW/CLOSE/终态保持映射并释放 reservation；FULLY_PAID 或未闭合付款效果拒绝。新建独立 owner audit，action=`PROCURE_PAYMENT_REQUEST_MIGRATION_REVERSED`、object=`procure.payment_requests` 原 APPLY 根，before/after 各恰含 schema_version/row_version/status。receipt target 固定 `platform_audit.audit_events.event_id=receipt.target_id=owner_audit.event_id`，owner event 与 R0 id 不同且同法人同 occurred_at；R0.after 指回 owner event。owner effect projection 精确为 `{owner_audit:row_v1(owner_audit),payment_request_after:row_v1(payment_requests)}` 再加 R0。 |
| `inventory.stock_opening` | inventory / ep-app-inventory | `inventory.stock_movements.id=target_id` | A0；自然来源固定 `IN/MIGRATION_OPENING/MIGRATION_STOCK_ADJUSTMENT/migration`，`source_doc_id=record.id`、`source_doc_no=batch.batch_no` | APPLY children：`stock_qty_entries.movement_id`、`stock_value_entries.movement_id`、`variance_splits.movement_id`、`stock_movement_serials.movement_id`；共享 balance/state 投影排除。REVERSE：`reverse_migrated_stock_movement` 追加 direction 反向、逐段金额/数量/序列镜像且 `reverses_movement_id` 指原 movement 的新 `stock_movements`，target 为该新 movement，投影同构并加 R0。 |
| `inventory.stock_history` | inventory / ep-app-inventory | `inventory.stock_movements.id=target_id` | A0；自然来源固定 direction=`IN,OUT,VALUE_ADJUST` 之一、`reason=MIGRATION_HISTORY`、`source_doc_type=MIGRATION_STOCK_HISTORY`、`source_module=migration`、`source_doc_id=record.id`、`source_doc_no=batch.batch_no` | APPLY/REVERSE children 与 stock opening 同构；金额行 pricing branch 固定 `MIGRATION_HISTORY`。REVERSE 同 `reverse_migrated_stock_movement`，target 为新反向 movement 并加 R0。092600 必须先按 Stage 8 同步段扩展 CHECK/Rust enum/定价与 direct-SQL tests。 |
| `ledger.opening_balance` | ledger / ep-app-ledger | `ledger.opening_balance_batches.id=target_id` | A0；`source='MIGRATION_BATCH'`、`migration_batch_no=batch.batch_no` | APPLY children：`opening_balance_batch_lines.opening_balance_batch_id`。REVERSE：`reverse_migrated_opening_balance` 走 ledger 受控迁移反向入口，按原行逐科目镜像生成 `ledger.vouchers` 及 lines，target 为新 voucher，投影含头行与 R0；不得 UPDATE 已 CONFIRMED opening batch。 |
| `ledger.historical_voucher` | ledger / ep-app-ledger | `ledger.vouchers.id=target_id` | A0；`source_kind='HISTORICAL_MIGRATION'`、`source_document_type='DATA_MIGRATION_RECORD'`、`source_document_id=record.id`、`source_document_no=batch.batch_no` | APPLY children：`voucher_lines.voucher_id`。REVERSE：`reverse_migrated_historical_voucher` 生成完整逐腿镜像、`reverses_id` 指原头行的 HISTORICAL_MIGRATION voucher，target 为新 voucher，投影含头行与 R0。092600 必须先按 Stage 9 同步段扩展第 19 个 source kind、专用受控入口、CHECK/镜像图与属性测试；普通 PostingPort 始终拒绝。 |
| `finance.open_items_opening` | finance / ep-app-finance | `target_object_type` 只可取 `finance.receivable_entries`、`finance.payable_entries`、`finance.advance_receipt_entries`、`finance.advance_payment_entries` 之一，根 `id=target_id` | A0；四表均 `source_doc_type='MIGRATION_OPENING'`；应收/应付业务引用全空，预收/预付另要求 `source_doc_id=record.id` | APPLY 无 children，hash 只含所选静态 relation 根。REVERSE：`reverse_migrated_open_item` 在同一所选台账追加 migration-opening 专用 reversal/effect，父 id 指原 opening entry、金额全额反向，target 为新 entry/effect，projection 为静态 relation 行加 R0；不得借 invoice/cash reversal 类型伪装。 |
| `finance.cash_account_opening` | finance / ep-app-finance | `finance.cash_accounts.id=target_id` | A0 | APPLY 无 children。REVERSE：`reverse_migrated_cash_account` 按 Stage 10 §4.2 复用停用守卫；active→inactive 时版本+1并写 deactivated_at，已 inactive 时状态/版本/时点保持，无未结资金事实才可执行。新建独立 owner audit，action=`FINANCE_CASH_ACCOUNT_MIGRATION_REVERSED`、object=`finance.cash_accounts` 原 APPLY 根，before/after 各恰含 schema_version/row_version/is_active/deactivated_at。receipt target 固定 `platform_audit.audit_events.event_id=receipt.target_id=owner_audit.event_id`，owner event 与 R0 id 不同且同法人同 occurred_at；R0.after 指回 owner event。owner effect projection 精确为 `{owner_audit:row_v1(owner_audit),cash_account_after:row_v1(cash_accounts)}` 再加 R0；不得删除账户或改历史 cash ledger。 |
| `invoice.sales_invoice_bundle` | invoice / ep-app-invoice | `invoice.sales_invoices.id=target_id` | A0；invoice application、合同/订单/计划必须先存在 | APPLY children：`sales_invoice_lines.sales_invoice_id`、`invoice_receipt_plan_links.sales_invoice_id`、根的 `invoice_number_registry_id` 行、根的 `invoice_application_id` 行及其 sales-order/receipt-plan links、`sales_invoice_attachments.owner_id`；共享 finance/ledger effects 排除并由 reconciliation 校验。REVERSE：既有 `create_invoice_reversal(OUTPUT)`，target=`invoice.invoice_reversals.id`，投影含 reversal lines、号码登记、附件及 R0。 |
| `invoice.purchase_invoice_bundle` | invoice / ep-app-invoice | `invoice.purchase_invoices.id=target_id` | A0；采购订单/收货/portal upload 必须先存在 | APPLY children：`purchase_invoice_lines.purchase_invoice_id`、根的 `invoice_number_registry_id` 行及 `purchase_invoice_attachments.owner_id`，附件按 `sort_no ASC,id UUID bytes ASC`。REVERSE：既有 `create_invoice_reversal(INPUT)`，target=`invoice.invoice_reversals.id`，投影含 reversal lines、号码登记、附件及 R0。具名采购附件表已冻结进 Stage 10 的 `V20261019091200__invoice_create_attachment_link_tables.sql`，不得漏出 hash或复用另一 owner 的附件关系。 |
| `project.project_bundle` | project / ep-app-project | `project.projects.id=target_id` | A0 | APPLY children：`project_tasks.project_id`、`project_attachments.owner_id`、task attachments 经本根 task ids、`project_task_purchase_requisition_links.project_task_id` 经本根 task ids。REVERSE：`reverse_migrated_project` 复用任务 CANCEL 与项目 CLOSED 守卫，始终新建 `project.project_migration_corrections`，mode 只取 CLOSE/RETAIN_CLOSED；receipt target 固定 `project.project_migration_corrections.id=receipt.target_id`，且 `correction.project_id=原 APPLY target_id`。owner effect 投影精确为 `{correction:row_v1(correction),project_after:row_v1(projects),task_after:set_v1(project.project_tasks,legal_entity_id=correction.legal_entity_id and project_id=correction.project_id)}` 再加 R0；set_v1 按 id UUID bytes 升序。存在 LINKED 采购需求对应对象未先 REVERSED 时 plan 拒绝。 |
| `service.customer_complaint_bundle` | service / ep-app-service | `service.customer_complaints.id=target_id` | A0 | APPLY children：`customer_complaint_attachments.owner_id`。REVERSE：`reverse_migrated_customer_complaint` 对 REGISTERED/PROCESSING 走既有 CANCEL，CLOSED/CANCELLED 保持根终态；两支都新建 `service.customer_complaint_migration_corrections`，mode 只取 CANCEL/RETAIN_TERMINAL。receipt target 固定 `service.customer_complaint_migration_corrections.id=receipt.target_id`，且 `correction.complaint_id=原 APPLY target_id`；owner effect 投影精确为 `{correction:row_v1(correction),complaint_after:row_v1(customer_complaints)}` 再加 R0。由本投诉升级的工单对象未先 REVERSED 时 plan 拒绝。 |
| `service.equipment_bundle` | service / ep-app-service | `service.equipment_records.id=target_id` | A0；自然来源固定 `source='MIGRATION'`、`migration_batch_no=batch.batch_no` | APPLY children：`equipment_attachments.owner_id`。REVERSE：`reverse_migrated_equipment` 在当前字典状态非终态时经既有入口置 RETURNED，当前已终态则保持原状态；两支都新建 `service.equipment_migration_corrections`，mode 只取 SET_RETURNED/RETAIN_TERMINAL。receipt target 固定 `service.equipment_migration_corrections.id=receipt.target_id`，且 `correction.equipment_record_id=原 APPLY target_id`；owner effect 投影精确为 `{correction:row_v1(correction),equipment_after:row_v1(equipment_records)}` 再加 R0。未结工单或序列占用依赖未先反向时 plan 拒绝；不得删除档案或改序列历史。 |
| `service.work_order_bundle` | service / ep-app-service | `service.work_orders.id=target_id` | A0 | APPLY children：`work_order_lines.work_order_id`、`work_order_logs.work_order_id`、`work_order_attachments.owner_id`、line attachments 经本根 line ids。REVERSE：`reverse_migrated_work_order` 对非终态走既有 CANCEL，对终态追加 `work_order_logs.entry_kind='CORRECTION'` 并以 reverses_id 指本次迁移产生的最后 ACTION；target 为新 correction log，投影含根 after-image、该 log 与 R0。 |

上述 25 行中 relation 名均以总数据字典为准。service 附件固定为 `equipment_attachments` 与 `work_order_line_attachments`，旧名 `equipment_record_attachments`/`work_order_log_attachments` 已从 Stage 12 清除；invoice 采购附件固定为 `purchase_invoice_attachments` 并由 Stage 10 的 091200 建表。project/service 三类 owner correction 固定为 `project_migration_corrections`、`customer_complaint_migration_corrections`、`equipment_migration_corrections`，随 Stage 12 的 090000/090700/090600 建表且以各自具名 effect id 为 REVERSE target；procure 两根与 finance cash account 不新增 correction 表，三者只允许上述三个固定 action 的独立 owner audit event 作为 target，且必须由 092600 静态分支同时证明根 after-image 和独立 R0，不能泛化为“任意 audit 即 effect”。`inventory.stock_history` 的 MIGRATION_HISTORY、`ledger.historical_voucher` 的 HISTORICAL_MIGRATION 及上述附件关系均已在 owner 阶段计划和直接数据字典冻结。092600 必须在 writer 启用前完成 inventory/ledger 两类 owner CHECK/图追补、三类 correction/R0 分支与三类 audit-target/R0 分支。25 分支生成门逐项对照这些现行名，缺一即失败，不得降级为无 provenance、无附件或不可反向。

`crm` 的客户 360、`costing` 的成本派生、`portal` 投影与 `reporting` 数据集不是源事实，不直接导入，正式写入完成后从上述权威记录重建；`MigrationObjectKind` 中不得出现这四个模块码。任何模板引用封闭值以外的对象，在批次进入 APPROVED 前以 `PLATFORM.DATA_MIGRATION.OBJECT_KIND_UNSUPPORTED` 拒绝，不得落到通用 JSON 表或低代码自定义对象兜底。扩展对象的历史导入不在首版范围，未来必须先为其对象类型登记具名模块写入者与对账规则。

模块端口固定定义在 `crates/platform/obs/src/data_migration/port.rs`，公开路径为 `ep_platform_obs::data_migration::MigrationModuleWriter`，只含中立结构，不含任一业务 crate 类型。它不放进 ep-foundation，因阶段 1 已冻结的四个 foundation port 模块不得增加；各 `ep-app-*` 依赖 ep-platform-obs 落在既有允许方向内。`MigrationModuleWriter` 的方法固定为 `validate`、`apply`、`reconcile_projection`、`plan_reversal` 与 `apply_reversal`：`validate` 纯校验且不得产生正式数据、文件对象、审计事件或 Outbox；`apply` 在调用方提供的 `UnitOfWork` 内复用本模块唯一权威写入者，业务记录、迁移记录、审计与 Outbox 同事务提交；`reconcile_projection` 只返回规范化计数、金额、数量、关系、附件和哈希摘要；两项 reversal 方法只能调用 catalog 具名 owner 更正/取消/冲销/版本命令。交易事实只能追加新反向 fact；可变根允许由该 owner 命令更新当前 after-image，但同事务必须追加 catalog 指定的不可变 version/change/correction fact。任何 writer 都不得以仓储通用 UPDATE 直接覆盖历史事实、物理 DELETE，或自建跨域通用 reversal 表。core-server 注册表按上述 25 个值逐项恰有一个实现，缺失、重复或实现 crate 与表中属主不符由 `apps/core-server/tests/migration_writer_registry.rs` 判红，不新增 archcheck 规则、不改变阶段 1 的规则清单。应用迁移与迁移对账任务都由 job-worker 经 `DataMigrationExecutor` 使用 `SecurityContext::system(legal_entity_id, request_id, trace_id, SystemPurpose::General)` 执行；这里的“迁移对账”是已批准迁移批次的工作流校验，不注册 `ReconCheck`、不新增 `ReconRunKind`、不写 platform_core 的三张内部对账表。阶段 14 对 ep-platform-recon 仍只调用既有 `ReconExecutor::run(RecoveryAcceptance)` 做恢复验收，A-06 的十五项注册表和 F-51 的 `Reconciliation` 构造封闭边界均不变。

`ep-data-migrate` 子命令固定为 `template-check`、`batch-create`、`trial`、`freeze-source`、`apply`、`reconcile`、`cutover`、`reverse`、`errors` 与 `status` 十个。退出码固定为 0 成功、2 参数或模板错误、3 批次状态或窗口不允许、4 来源 schema/manifest 摘要变化、5 存在记录校验错误、6 对账未通过、7 审批或重新认证不足、8 来源连接失败、78 本机环境或签名自检失败；不得与 `ep-migrate` 的退出码语义混用。每块最多 1000 行且规范化 JSON 请求体不超过 524288 字节，两项先到即封块；连同 HTTP 头与封套的单请求必须小于等于全局 1048576 字节门。单条规范化记录自身超过 524288 字节直接返回 `PLATFORM.DATA_MIGRATION.RECORD_TOO_LARGE`，不得为该路由放宽 body 门；大附件只在记录中提交已批准文件清单引用，正文走既有附件流水线。块带 `batch_id + run_no + chunk_no` 幂等键；工具只在内存中持有当前块，服务端成功或失败应答后都显式清零。错误导出只在本机把表 19 的来源定位摘要与当前读取的源记录重新关联，生成客户控制的加密文件；平台数据库与普通日志永不保存来源原文。

状态机与唯一守卫如下，全部迁移与拒绝都写 `platform_audit.audit_events`，并把模板、源 manifest、差异清单与切换决定的摘要纳入事件：

1. DRAFT → APPROVED：模板签名与 schema fingerprint 通过；时间窗口尚未结束；数据责任人、客户财务负责人已批准；本批涉及的每个模块各有模块责任人批准；证据逐项满足第 3.1.2 节且绑定当前 approval_content_hash；25 项封闭注册表覆盖模板全部对象；四项安全属性逐字段显式映射；来源连接只读负样例通过。任一不满足即不迁移。
2. APPROVED → TRIAL_RUNNING → TRIAL_PASSED/TRIAL_FAILED：`trial` 对完整源数据逐条调用 `validate`，在内存与摘要台账上构造投影并执行第 7.6、7.10 章全部九类对账，不调用 `apply`，因此不产生正式业务数据。校验失败记录进入表 19 的 FAILED 队列；修复来源或升版模板后以 `run_no+1` 重跑，历史运行不覆盖。正式执行前至少一轮完整试运行通过。
3. 两轮完整试运行仍未收敛时不得继续沿用原模板或原批次重试；原批次保持 TRIAL_FAILED，只能按第 9 项取消。数据责任人与客户财务负责人必须签署三选一决定：缩小首批范围、只迁期初与未结事项、历史明细留在源系统自行查询；前两项必须另建新 template_version 与新 batch_no，重新完成批次批准并从 run_no=1 试运行，不得修改已离开 DRAFT 的旧批次模板列。决定写交付说明与客户合同；它不是借贷平衡、库存守恒或不可豁免差异的豁免。
4. TRIAL_PASSED → SOURCE_FROZEN：客户在源系统启用只读冻结，`freeze-source` 读回源端只读证据、基准水位和源 manifest，三者摘要写审计。平台不能凭声明代替读回；无可机器读回的源系统须由数据责任人与源系统管理员双签证据，仍写同一事件。
5. SOURCE_FROZEN → APPLYING → DELTA_CATCHUP：按通过试运行的基准 manifest 分块调用 `apply`；一块一个数据库事务，任一行失败整块回滚，失败行入错误队列且不进入正式数据。基础批完成后从模板声明的单调 watermark 读到冻结点并追平；冻结后源端出现新写入或已处理来源记录摘要变化，整批转 TRIAL_FAILED 并返回 SOURCE_CHANGED，不允许继续切换。
6. DELTA_CATCHUP → RECONCILING → READY_FOR_CUTOVER：对正式目标执行九类对账。借贷平衡与库存守恒必须 PASS；切换基准日的期初余额、库存期初结存、未结应收应付与预收预付、未完成采购和销售订单差异必须为零；其余差异只有落入表 21 的四类封闭 category、经数据责任人、模块责任人与客户财务负责人三方批准并保持 `decision = 'APPROVED'`，再由表 20 关联后，才可记 APPROVED_DIFFERENCE。数量、金额、关系、附件、哈希与安全属性检查没有未批准 FAIL 时才进入 READY_FOR_CUTOVER；任何已关联差异在切换前转为 REVOKED，批次立即退回 RECONCILING，不得继续切换。
7. READY_FOR_CUTOVER → CUTOVER_COMPLETED：数据责任人、全部涉及模块责任人与客户财务负责人对 cutover_content_hash 共同批准，表 22 必须形成第 3.1.2 节的精确 CUTOVER_APPROVAL 证据集，请求携带有效 X-Reauth-Token；不接受任意文本决定引用。迁移窗口在同一事务关闭，25 项迁移写入口自动停用，切换事件不可篡改留证。重新开启必须新建批次并重新审批，不能把已完成批次改回 APPROVED。
8. CUTOVER_COMPLETED → REVERSAL_PENDING → REVERSED：只接受新的双人审批与 X-Reauth-Token；先汇总 25 个写入者的 reversal plan，任一模块不能完整计划时整批拒绝，不做部分冲销。执行时逐模块追加更正/冲销记录并保留 `migration_batch_no` 与反向批次引用；完成后重跑同一九类对账并留证。数据库台账、审计与原始迁移记录继续保留，REVERSED 不等于删除。
9. DRAFT、APPROVED、TRIAL_FAILED、TRIAL_PASSED 可转 CANCELLED；SOURCE_FROZEN 之后不得 CANCELLED，只能修复后继续或按 reversal 流程处理已经写入的部分。任何窗口到期、会话失效、来源摘要变化、已知差异批准被三方撤销、法人上下文不符或租约丢失都停止领取新块；已开事务回滚，陈旧 worker 不得写最终状态。批次与切换的已完成标准流程任务本身不可事后改写为“撤销”；内容变化必须递增 content_version 并重新走完整批准。

来源重复与任务租约不新增事件或外部队列。批次和逐记录行就是耐久任务载体；job-worker 只领取 `task_available_at <= clock_timestamp()` 且 `task_locked_until` 为空或已过期的批次，以 `FOR UPDATE SKIP LOCKED` 写入全局唯一 worker id、60 秒租约并把 `task_attempts` 加一，每 20 秒按批次 id、持有者与未过期租约条件续租，数据库当前时刻统一取 `clock_timestamp()`。续租或最终状态条件更新影响零行时立即回滚并停止该 worker，陈旧持有者不得覆盖新持有者；暂时性来源故障把 `task_available_at` 写为数据库当前时刻加共享八步退避并清租约，不占线程睡眠。逐记录唯一约束保证同一源键只产生一次正式效果，正式业务表上的模块幂等键再使用表 23 冻结的 `dm:v1:<sha256hex>`，其摘要不含 run_no，保证同一 batch/module/object/source_locator 的跨 run 重放仍命中同一 APPLY 或 REVERSE key。错误重跑只领取 FAILED 且来源摘要已经改变的记录；摘要未变时拒绝无效重跑。

#### 4.13 病毒扫描部署状态、健康与披露

首版不内置病毒库、不交付 CLAMD，也不建设病毒库在线或离线更新通道。部署记录与配置必须逐字一致地选 `NONE` 或 `CUSTOMER_ICAP`，没有默认值、自动探测或第三分支。`NONE` 在部署记录生效事务后立即以 `kind=VIRUS_SCANNER_NOT_AVAILABLE`、`subject=VirusScan`、全局 scope 打开不可抑制窗口；两个内建的类型与结构检查仍为附件发布硬前置，但 VIRUS_ICAP 结果固定为 `SKIPPED/MODE_NONE`。窗口在部署状态页和健康结论持续可见，交付说明与合同模板逐字写“平台未提供病毒防护”，不得用“具备恶意内容检查”替换或弱化该句。

`CUSTOMER_ICAP` 只允许 integration-gateway 连接客户自管、同机回环的 ICAP 扫描器。URL 只接受 `icap://127.0.0.1:<port>/<service>` 或 `icap://[::1]:<port>/<service>`；配置解析拒绝主机名、DNS、系统代理、重定向和非回环地址。core-server 经既有本机内部调用把当前解密流交给 integration-gateway，后者有界流式转发且不落盘；产品不新增 ICAP 监听口，明文不离开服务器。扫描器 CLEAN 才发布；INFECTED、超时、不可达、协议非法或未知响应都隔离附件并禁止引用、下载、发布，打开同一不可抑制窗口，绝不自动回退 NONE。只有下一次健康探测和真实扫描样本都成功时才关窗，既有隔离件仍须重新扫描，不能人工改为可用。

本口径是阶段 1“本机 IPC 不取回环 TCP”的唯一窄例外：例外主体只是 integration-gateway 的 ICAP 客户端，目的端只能是上述回环 IP；不扩大到 core-server 直连、产品自有监听、远端 ICAP 或其他协议。客户扫描器的引擎许可、病毒库更新、误报漏报与服务账户由客户负责；其产品、版本、服务名、回环端口、最近健康探测、测试样本结论与责任边界进入部署记录附件和发布证据包。

---

### 5. API 契约

统一前提。全部端点前缀 /api/v1/platform，请求头按基线第 5.6 节固定集合，写请求必带 Idempotency-Key，响应按基线第 5.2 节封套。分页、排序、过滤按基线第 5.3 节。权限一律按 ABAC 判定；运维台账主体角色取运维管理员、安全管理员、审计管理员三类。全部历史数据迁移端点统一判定第 3.3 节 `platform.data_migration`，动作映射固定为 GET→VIEW、POST 建批次/差异→CREATE、上传/试跑/冻结/应用/对账/取消→UPDATE、发起 approve/decide/revoke/cutover/reverse→SUBMIT，授权 object id 始终取 batch id；建批次尚无现存行时先以请求 legal_entity_id 和预生成 batch id 判定 CREATE，成功写入后同一 id 成为对象锚。权限通过后仍要求当前主体是批次记名的数据责任人、对应模块责任人、客户财务负责人或被其审批记录授权的迁移执行人，不因拿到 permission 自动成为任一职责。对当前上下文不可见的记录按基线第 5.5 节返回 404 与 PLATFORM.AUTHZ.NOT_FOUND_OR_DENIED。本节一般端点不要求 X-Reauth-Token；历史数据迁移的 approve、known-difference decide/revoke、cutover、reverse 五类动作例外，均按表中要求消费一次 operation_type=DATA_MIGRATION 的有效 X-Reauth-Token、绑定第 3.1.2 节 subject digest 并校验审批链，申请人不可自审；iOS/Android 请求重新认证挑战时与 Payment 同等前置拒绝。密钥恢复材料核验登记继续只要求既有审批链。处置执行不在本节端点内，其双人控制与重新认证凭证要求见第 4.10 节。

能力域码与动作类别按裁定 A-20 声明。本节全部路由逐用例声明一对常量，命名为用例名的全大写下划线形式后接 _DOMAIN 与 _ACTION，类型取阶段 1 在 ep-foundation 冻结的 CapabilityDomain 与 ActionClass，本阶段不自定义能力域码，也不重新定义这两个枚举。本节路由都在 /api/v1/platform 前缀下，常量一律声明在 crates/platform/obs/src/capability.rs，能力域一律取 CapabilityDomain::PlatformAdminLowcodeOps。动作类别按只读查询取 Read、部署记录导出取 Export、其余写端点取 Write。ops-agent 的三个端点与第 4.10 节的处置执行都不在 /api/v1 命名空间内，不参与该判定，不声明常量。xtask configdoc 断言每个 /api/v1/ 路由都能解析到一对常量，缺失即构建失败。

| 方法与路径 | 请求 | 响应 data | 主要错误码 | 幂等 | 权限 |
|---|---|---|---|---|---|
| GET /degradation-windows | filter[kind]、filter[subject]、filter[state]=open\|closed、filter[scope_legal_entity_id]、分页排序 | 台账条目数组，含 kind、subject、scope、basis、opened_at、closed_at、closing_condition、is_suppressible、suppressed_until | 无 | 读 | 三类角色 |
| GET /degradation-windows/{id} | 无 | 单条含 detail | PLATFORM.AUTHZ.NOT_FOUND_OR_DENIED | 读 | 三类角色 |
| POST /degradation-windows/{id}/actions/suppress | { until_at, reason } | 抑制结果 | PLATFORM.DEGRADATION_WINDOW.NOT_SUPPRESSIBLE 409、PLATFORM.DEGRADATION_WINDOW.ALREADY_CLOSED 409、PLATFORM.CONCURRENCY.STALE_VERSION 409 | 键相同重放返回首次结果 | 运维管理员与安全管理员 |
| POST /degradation-windows/{id}/actions/unsuppress | { reason } | 同上 | 同上 | 同上 | 同上 |
| GET /recovery-objectives | 无 | { rpo: [ {target, effective_seconds, basis, evidence_ref} x2 ], disclosed_rpo_seconds, rto: {hours, applicable, preconditions, shard_pickup_sla_hours} } | 无 | 读 | 三类角色 |
| GET /offsite-sinks | 无 | 当前落点与其判定结论 | 无 | 读 | 三类角色 |
| POST /offsite-sinks/actions/probe | 无 | 后台任务回执 | PLATFORM.OFFSITE_SINK.NOT_CONFIGURED 409 | 是 | 运维管理员 |
| POST /offsite-sinks/actions/measure-throughput | { direction: read\|write\|both } | 后台任务回执 | PLATFORM.OFFSITE_SINK.UNWRITABLE 503 | 是 | 运维管理员 |
| POST /offsite-sinks/actions/attest-access-control | { evidence_ref, writer_identity_ref, restore_identity_ref, disposal_identity_ref, backend_policy_digest, negative_probe_result, notes } | offsite_sinks 新哨兵版本与部署记录新版本；只有身份互斥、策略证据齐全且覆盖/删除/重命名/改权/策略负向探针全被拒才记 PASS | PLATFORM.OFFSITE_SINK.ACCESS_CONTROL_NOT_ATTESTED 409 | 同一证据摘要重放返回首次结果 | 安全管理员 |
| GET /archive-channel | 无 | 当前状态、broken_at、break_cause、sub_state、rebuild_backup_set_id | 无 | 读 | 三类角色 |
| POST /archive-channel/actions/reevaluate-sink | 无 | 后台任务回执 | 无 | 是 | 运维管理员 |
| GET /backup-sets | filter[kind]、filter[state]、分页 | 备份集数组 | 无 | 读 | 三类角色 |
| GET /backup-sets/{id} | 无 | 单条含校验结论数组 | 无 | 读 | 三类角色 |
| POST /backup-sets/actions/run-full | { kind: DAILY_FULL } | 后台任务回执 | PLATFORM.BACKUP_SET.CONCURRENT_RUN 409、PLATFORM.OFFSITE_SINK.UNWRITABLE 503 | 是 | 运维管理员 |
| GET /capacity | 无 | 六项组件水位与容量下限对照 | 无 | 读 | 三类角色 |
| GET /key-recovery-materials | 无 | 登记项数组，不含材料本身 | 无 | 读 | 安全管理员 |
| POST /key-recovery-materials/{id}/actions/record-verification | { performed_at, performed_by_party, outcome, isolated_env_ref, approval_ref, report_ref } | 核验记录与逾期窗口开闭结果；即使提交前已逾期也受理本次核验，不以错误响应阻断修复 | PLATFORM.SOD.SELF_APPROVAL_FORBIDDEN 409 | 是 | 安全管理员，需审批链 |
| GET /recovery-drills | filter[drill_kind]、分页 | 演练数组 | 无 | 读 | 三类角色 |
| POST /recovery-drills | { drill_kind,backup_selection,attempt_no,window_started_at,sink_id,backup_set_id,retention_days_at_start }；backup_verified_at_at_start 与 sink_kind_at_drill 由服务锁父行后复制，不接受客户端自填 | RUNNING 演练记录 | PLATFORM.RECOVERY_DRILL.DUPLICATE_ATTEMPT 409 | 是 | 运维管理员 |
| POST /recovery-drills/{id}/actions/record-result | `{outcome,report_ref,failure_stage,failure_code,readback_throughput_mibps,rto_seconds,rpo_db_seconds,rpo_attachment_seconds,shard_pickup_seconds,attachment_check_total,attachment_check_failed,attachment_check_seconds,invariant_check_batches,invariant_check_max_batch_seconds,invariant_check_total_seconds,invariant_check_mem_peak_bytes,invariant_check_tempfile_peak_bytes,decrypt_seconds}`；PASS 的 failure 两列空且指标完整，FAIL 的 failure 两列必填且未执行指标保持 null | 演练记录 | PLATFORM.RECOVERY.ATTACHMENT_CONTENT_MISSING 409、PLATFORM.RECOVERY.ATTACHMENT_CHECKSUM_MISMATCH 409 | 是 | 运维管理员 |
| GET /deployment-record | 无 | 部署记录当前版本 | 无 | 读 | 三类角色 |
| POST /deployment-record/actions/export | { format: json\|csv } | 后台任务回执 | 无 | 是 | 三类角色 |
| POST /data-migration-batches | `{batch_no,legal_entity_id,source_kind,source_system_ref,template_code,template_version,template_sha256,ledger_scope,warehouse_scope,window_starts_at,window_ends_at,data_owner_id,customer_finance_owner_id}`；source_schema_fingerprint、source_module_codes、required_reconciliation_keys 与 approval_content_hash 由已验签模板确定，调用方不得另填 | DRAFT 批次 | PLATFORM.DATA_MIGRATION.INVALID_TEMPLATE 400、PLATFORM.DB.LEGAL_ENTITY_MISMATCH 403 | 是 | 迁移执行人 |
| GET /data-migration-batches | 法人、状态、来源类型、分页 | 当前上下文可见批次数组 | 无 | 读 | 迁移四类主体；审计管理员只读 |
| GET /data-migration-batches/{id} | 无 | 批次、运行摘要、审批、对账与差异摘要，不含来源原文 | PLATFORM.AUTHZ.NOT_FOUND_OR_DENIED 404 | 读 | 同上 |
| POST /data-migration-batches/{id}/actions/approve | `{source_readonly_test_ref,content_version,row_version}`，content_version 是更新前期望值；服务验证只读负测报告后写入引用、把 content_version 精确加一、重算 approval_content_hash，再按静态角色映射创建只含七个路由键的标准流程，不接受 approval_ref 或 content hash | 流程引用；全部 callback 形成表 22 闭图后自动转 APPROVED | PLATFORM.DATA_MIGRATION.APPROVAL_INCOMPLETE 409、PLATFORM.DATA_MIGRATION.READ_ONLY_PROOF_FAILED 409、PLATFORM.DATA_MIGRATION.OBJECT_KIND_UNSUPPORTED 400 | 是；消费 X-Reauth-Token | 数据、财务与全部涉及模块责任人的审批齐备 |
| POST /data-migration-batches/{id}/actions/trial | `{source_schema_fingerprint,source_manifest_sha256,source_record_count,row_version}` | `{run_no,upload_session_id,expires_at}` 并转 TRIAL_RUNNING | PLATFORM.DATA_MIGRATION.STATE_CONFLICT 409、PLATFORM.DATA_MIGRATION.SOURCE_CHANGED 409 | 是 | 迁移执行人 |
| POST /data-migration-batches/{id}/runs/{run_no}/chunks | `{upload_session_id,phase:TRIAL\|BASE\|DELTA,chunk_no,watermark,records[]}`；最多 1000 行且规范化 JSON 体 ≤524288 字节，完整请求 ≤1 MiB | 逐行受理摘要 | PLATFORM.DATA_MIGRATION.CHUNK_TOO_LARGE 400、PLATFORM.DATA_MIGRATION.RECORD_TOO_LARGE 400、PLATFORM.DATA_MIGRATION.RECORD_INVALID 400、PLATFORM.DATA_MIGRATION.WINDOW_CLOSED 409 | `batch_id+run_no+phase+chunk_no` | 迁移执行人，签名部署清单员工 HTTPS origin 的一次性会话 |
| POST /data-migration-batches/{id}/runs/{run_no}/actions/finish-trial | `{final_manifest_sha256,final_watermark}` | TRIAL_PASSED 或 TRIAL_FAILED 与差异摘要 | PLATFORM.DATA_MIGRATION.RECONCILIATION_FAILED 409、PLATFORM.DATA_MIGRATION.SOURCE_CHANGED 409 | 是 | 迁移执行人 |
| POST /data-migration-batches/{id}/actions/freeze-source | `{readonly_evidence_ref,source_manifest_sha256,delta_watermark,row_version}` | SOURCE_FROZEN 批次 | PLATFORM.DATA_MIGRATION.READ_ONLY_PROOF_FAILED 409、PLATFORM.DATA_MIGRATION.SOURCE_CHANGED 409 | 是 | 数据责任人或迁移执行人 |
| POST /data-migration-batches/{id}/actions/start-apply | `{trial_run_no,row_version}` | `{run_no,upload_session_id,expires_at}` 并转 APPLYING | PLATFORM.DATA_MIGRATION.STATE_CONFLICT 409、PLATFORM.DATA_MIGRATION.WINDOW_CLOSED 409 | 是 | 迁移执行人 |
| POST /data-migration-batches/{id}/runs/{run_no}/actions/finish-apply | `{final_manifest_sha256,freeze_watermark}` | DELTA_CATCHUP 或 TRIAL_FAILED | PLATFORM.DATA_MIGRATION.RECORD_INVALID 400、PLATFORM.DATA_MIGRATION.SOURCE_CHANGED 409 | 是 | 迁移执行人 |
| POST /data-migration-batches/{id}/actions/reconcile | `{run_no,row_version}` | 后台任务回执并转 RECONCILING | PLATFORM.DATA_MIGRATION.RECONCILIATION_FAILED 409 | 是 | 迁移执行人；job-worker 调 DataMigrationExecutor，不注册 ReconCheck |
| GET /data-migration-batches/{id}/records | `filter[status]`、`filter[error_code]`、module_code、object_type、分页 | 仅摘要与清洗错误，不含来源原文 | 无 | 读 | 迁移四类主体；审计管理员只读 |
| GET /data-migration-batches/{id}/reconciliations | run_no、check_kind、outcome、分页 | 九类对账与已知差异关联 | 无 | 读 | 同上 |
| POST /data-migration-batches/{id}/known-differences | `{module_code,category,ledger_or_warehouse_scope,source_document_scope,amount,quantity,cause,cannot_zero_reason,proposal_ref,data_owner_id,module_owner_id,finance_owner_id}`，不含 decision/decided_at；module 必须属于批次，content version/hash 由服务生成 | PROPOSED 差异台账行 | PLATFORM.DATA_MIGRATION.KNOWN_DIFFERENCE_FORBIDDEN 409 | 是 | 数据责任人或迁移执行人 |
| POST /data-migration-batches/{id}/known-differences/{difference_id}/actions/decide | `{decision:APPROVED\|REJECTED,content_version,row_version}`；服务创建三方版本绑定流程，不接受任意引用或内容副本 | 流程引用；表 22 闭图后自动成为 APPROVED 或 REJECTED | PLATFORM.DATA_MIGRATION.KNOWN_DIFFERENCE_FORBIDDEN 409、PLATFORM.DATA_MIGRATION.APPROVAL_INCOMPLETE 409、PLATFORM.DATA_MIGRATION.STATE_CONFLICT 409 | 是；消费 X-Reauth-Token | 数据、模块、财务三方共同决定 |
| POST /data-migration-batches/{id}/known-differences/{difference_id}/actions/revoke | `{content_version,row_version}`；三方各自批准撤销并由 evidence.reverses_evidence_id 精确反向 | REVOKED 台账行；批次未切换时退回 RECONCILING | PLATFORM.DATA_MIGRATION.APPROVAL_INCOMPLETE 409、PLATFORM.DATA_MIGRATION.STATE_CONFLICT 409 | 是；消费 X-Reauth-Token | 数据、模块、财务三方共同撤销；CUTOVER_COMPLETED 后不可撤销 |
| POST /data-migration-batches/{id}/actions/cutover | `{final_reconciliation_report_ref,row_version}`；服务重算 digest/hash 并创建精确 owner 集流程，不接受 cutover_decision_ref | 流程引用；表 22 闭图且状态守卫成立后自动 CUTOVER_COMPLETED | PLATFORM.DATA_MIGRATION.CUTOVER_NOT_READY 409、PLATFORM.DATA_MIGRATION.APPROVAL_INCOMPLETE 409 | 是；消费 X-Reauth-Token | 数据、全部模块、财务三方共同签署 |
| POST /data-migration-batches/{id}/actions/reverse | `{second_approver_id,reason,row_version}`；second_approver_id 必须是与数据责任人不同且决定日持有效 SECURITY_ADMIN 授权的用户，服务创建 DATA_OWNER 与 SECOND_APPROVER 两条版本绑定批准流程，不接受 approval_ref | 流程引用；闭图且 25 writer 全部可计划后转 REVERSAL_PENDING | PLATFORM.DATA_MIGRATION.REVERSAL_NOT_PLANNABLE 409、PLATFORM.SOD.SELF_APPROVAL_FORBIDDEN 409 | 是；消费 X-Reauth-Token | 数据责任人与安全管理员 |
| POST /data-migration-batches/{id}/actions/cancel | `{reason,row_version}`；服务保存 cancelled_from_status、数据库时钟 cancelled_at 与原 reason，不只写审计 | CANCELLED 批次 | PLATFORM.DATA_MIGRATION.STATE_CONFLICT 409 | 是 | 数据责任人；只允许 DRAFT、APPROVED、TRIAL_FAILED、TRIAL_PASSED |

ops-agent 端点，不使用封套：GET http://127.0.0.1:9101/metrics 返回 Prometheus 文本；GET http://127.0.0.1:9102/healthz 返回进程存活；GET http://127.0.0.1:9102/readyz 返回本进程适用的基线自检项的结论摘要与当前降级条目数，摘要按注册名标识并按 severity 分 Blocking 与 Degrading 两档，不用序号。

本阶段新增的全部错误码，登记入 docs/error-codes.md 与 ep-foundation 的 error::codes：
PLATFORM.DEGRADATION_WINDOW.NOT_SUPPRESSIBLE、PLATFORM.DEGRADATION_WINDOW.ALREADY_CLOSED、PLATFORM.OFFSITE_SINK.NOT_CONFIGURED、PLATFORM.OFFSITE_SINK.UNWRITABLE、PLATFORM.OFFSITE_SINK.MEDIA_TYPE_OFFLINE、PLATFORM.OFFSITE_SINK.ACCESS_CONTROL_NOT_ATTESTED、PLATFORM.ARCHIVE_CHANNEL.SLOT_INVALIDATED、PLATFORM.ARCHIVE_CHANNEL.SUSPENDED、PLATFORM.BACKUP_SET.CONCURRENT_RUN、PLATFORM.BACKUP_SET.VERIFY_FAILED、PLATFORM.BACKUP_SET.SPILL_LIMIT_EXCEEDED、PLATFORM.BACKUP_ENCRYPTION.KEY_UNAVAILABLE、PLATFORM.KEY_RECOVERY_MATERIAL.SHARD_PICKUP_SLA_MISSING、PLATFORM.RECOVERY.WATERMARK_UNAVAILABLE、PLATFORM.RECOVERY.ATTACHMENT_CONTENT_MISSING、PLATFORM.RECOVERY.ATTACHMENT_CHECKSUM_MISMATCH、PLATFORM.RECOVERY_DRILL.DUPLICATE_ATTEMPT、PLATFORM.CAPACITY.DISK_WATERMARK_EXCEEDED；以及 PLATFORM.DATA_MIGRATION.INVALID_TEMPLATE、PLATFORM.DATA_MIGRATION.OBJECT_KIND_UNSUPPORTED、PLATFORM.DATA_MIGRATION.SOURCE_CHANGED、PLATFORM.DATA_MIGRATION.WINDOW_CLOSED、PLATFORM.DATA_MIGRATION.STATE_CONFLICT、PLATFORM.DATA_MIGRATION.APPROVAL_INCOMPLETE、PLATFORM.DATA_MIGRATION.READ_ONLY_PROOF_FAILED、PLATFORM.DATA_MIGRATION.CHUNK_TOO_LARGE、PLATFORM.DATA_MIGRATION.RECORD_INVALID、PLATFORM.DATA_MIGRATION.RECONCILIATION_FAILED、PLATFORM.DATA_MIGRATION.KNOWN_DIFFERENCE_FORBIDDEN、PLATFORM.DATA_MIGRATION.CUTOVER_NOT_READY、PLATFORM.DATA_MIGRATION.REVERSAL_NOT_PLANNABLE、PLATFORM.DATA_MIGRATION.SOURCE_CONNECTION_FAILED。分类归属：落点不可写、备份加密密钥不可用、磁盘水位与来源连接失败属 INFRASTRUCTURE 且 retryable 为真；INVALID_TEMPLATE、OBJECT_KIND_UNSUPPORTED、CHUNK_TOO_LARGE 与 RECORD_INVALID 属 VALIDATION 且 retryable 为假；其余属 BUSINESS_CONFLICT 且 retryable 为假。逾期核验通过台账窗口、健康结论和成功响应中的逾期状态呈现，不定义一个“返回 409 但同时成功登记”的伪错误码。

进程间接口报文承载于 Windows 命名管道 `\\.\pipe\ep-core`，帧格式按基线第 2 节的 4 字节大端长度前缀加 JSON 体，类型与 operation 唯一映射如下：
1. WriteoutResultReport → `ops.writeout_result.report.v1`。
2. VerificationConclusionReport → `ops.verification_conclusion.report.v1`。
3. FailureEventReport → `ops.failure_event.report.v1`。
4. ReplicationLifecycleReport → `ops.replication_lifecycle.report.v1`，对应复制连接建立与断开、复制槽建立与失效、全量基础备份起止，逐项记录角色、进程、起止时间与结果。
5. AttachmentWriteoutScopeQuery 与应答 → `ops.attachment_writeout_scope.query.v1`，由 core-server 提供附件写出对象范围与元数据提交状态。
6. AttachmentChecksumVerdictReport → `ops.attachment_checksum_verdict.report.v1`，用于恢复模式流式校验和上报。
7. BackupSlotAcquire 与 BackupSlotRelease → `ops.backup_slot.acquire.v1`、`ops.backup_slot.release.v1`，用于串行槽申请与释放。

`ep-core` server 身份为 `NT SERVICE\ep-core`。DACL 与服务端 token operation allowlist 固定为：`NT SERVICE\ep-archive` 只允许第 1、3、4、5 项；`NT SERVICE\ep-backup` 只允许第 1、2、3、4、6、7 项；`NT SERVICE\ep-ops` 只允许 `health.get.v1`、`metrics.snapshot.v1`，不得调用任何上报或范围请求。两写出身份交叉调用、ep-ops 调业务 operation、其他账户连接、客户端不核 server 账户均为必须失败的部署/集成负例。
全部报文带 report_id（UUIDv7），core-server 侧以对应表的 ux_*_report_id 唯一约束做幂等，不复用 platform_msg.idempotency_keys，理由是后者的作用域四元组含法人与端点，与部署级上报不匹配。

---

### 6. 并发与事务边界

事务边界。上报受理器一个报文一个事务，事务内完成两件事：写对应 platform_ops 表、写 platform_audit.audit_events；台账开闭与状态机迁移在同一事务内完成，不拆分。本阶段部署状态不写 `platform_msg.outbox_events`，新增事件为 0。隔离级别 READ COMMITTED。事务内禁止外部调用与文件正文读写，落点写出全部发生在写出进程内且在事务之外。

锁策略。archive_channel 与 backup_runner_slot 两个单行表的更新一律 SELECT ... FOR UPDATE 加乐观锁 row_version，受影响行数为 0 判版本冲突并返回 PLATFORM.CONCURRENCY.STALE_VERSION。degradation_windows 的开窗依赖阶段 2 交付的 ux_degradation_windows_kind_scope_closed 唯一约束，其列组为 kind、subject、两个作用域列与开窗状态，同一 kind 下 subject 不同的两条窗口互不冲突；重复开窗触发唯一冲突后转为读取既有活动条目并返回，即开窗天然幂等。台账的开与闭一律经阶段 2 在 ep-platform-obs 交付的 DegradationLedger 的 open 与 close，本阶段扩展其 kind 取值与实现，不另建第二条写入路径。

幂等键。IPC 报文以 report_id 唯一约束幂等；HTTP 写请求以 Idempotency-Key 按基线第 5.4 节幂等；备份与写出任务以 period_seq 与 backup_set_id 幂等，重复触发返回既有结果。

与 Outbox 的关系。阶段 14 新增平台事件类型固定为 **0**。部署级归档、备份、容量、恢复与降级状态只写 `platform_ops` 台账、`platform_audit.audit_events` 与既有指标，不进入面向业务消费者的 Outbox，也不为了满足事件信封的必填法人字段伪造“系统法人”。用户提醒由运维中心台账、健康结论与告警采集链呈现，不再建设一套未命名的平台事件。

历史数据迁移同样不新增第二套业务事件。模块 writer 的 `apply` 与 `apply_reversal` 复用该模块已经登记的领域事件，并把 `migration_batch_no` 放入既有来源引用；迁移批次状态、源冻结、对账、差异批准与切换决定只写 `platform_audit.audit_events`，不进入面向业务消费者的 Outbox。迁移分块一个块一个事务：业务追加记录、表 23 writer receipt、表 19 的 APPLIED/REVERSED 状态与双向 receipt 指针、既有领域事件、审计与 Outbox 同一提交；任一项失败全部回滚。附件正文复用阶段 3b 的暂存/提交协议，正文写成功但数据库事务回滚时由既有孤儿收敛任务处置，不在本阶段另建补偿表。

失败重试与补偿。落点写出失败按指数退避在写出进程内重试，退避序列取 5 秒、15 秒、45 秒、2 分钟、5 分钟，五次内未成功即计一次周期失败并上报；周期失败即开对应暴露窗口，不进入 Outbox 死信。Outbox 投递失败按基线第 6.2 节的八次退避序列，全部失败置 DEAD。core-server 不可用期间，两写出进程各用本服务配置中的 `EP__SPOOL__DIR` 与 `EP__SPOOL__MAX_BYTES`（默认 20 GiB），不存在 `EP__ARCHIVE__SPOOL_*` 或 backup 同义键。`WriteoutResultReport`、`VerificationConclusionReport`、`FailureEventReport`、`ReplicationLifecycleReport`、`AttachmentChecksumVerdictReport` 五类都是关键证据，只追加、flush、经 core 确认后截断，任何水位都不得删除或覆盖；范围查询和 slot acquire/release 需即时应答，不落 spool，core 不可用时不得开启新周期。只有可从本地 manifest 与落点对象清单确定重建的 heartbeat/progress 本地记录可按对象压缩为最新一条。软水位为上限减 64 MiB，保留区只写在途 critical 收尾；达到后继续 WAL 接收与当前写出，但停止新全量备份与附件周期、写 Windows Event Log。连接恢复后先按 `(occurred_at,report_id)` 重放，并让 core 打开不可抑制 `WRITER_NOT_IN_SERVICE`、subject=`<writer>:report-spool-exhausted`；重放完成且低于软水位才关窗。

必须覆盖的并发场景。一，落点转不可写与每日全量备份窗口重叠。二，复制槽失效与每日全量备份窗口重叠，验证在用流复制连接不超过 3、在用复制槽不超过 2。三，断链重建基线备份与每日全量备份的串行，验证不并发。四，同一 report_id 重复上报不少于 3 次，验证只产生一次效果。五，暴露窗口的并发开闭。六，core-server 重启期间写出进程持续写出且上报补写不丢。

---

### 7. 配置项

前缀 EP__，层级用双下划线，deny_unknown_fields。生效方式服从 `docs/config-reference.md` 的全局两档，只写“启动”或“取用”，不建设 SIGHUP 或目录监听入口：“启动”表示修改后须重启对应 Windows 服务；“取用”表示下一次读取该值时生效。原第三档“重判”不是新的加载方式，而是 `sink.kind`、`sink.root`、`sink.media_type` 三键的附加守卫：新值被业务使用前，必须重新执行落点判定并按附录 A.6 重做一次整机失效恢复演练；证据未完成时仍沿用旧的有效判定，不得仅因文件已改就宣称新值生效。

| 键 | 类型 | 默认值 | 生效 | 说明 |
|---|---|---|---|---|
| EP__SINK__KIND | 枚举 LOCAL_DIR/NFS_SMB_MOUNT/OBJECT_STORAGE | 无，必填 | 取用 | 变更后须重判；认证的三种落点类型之外不验收 |
| EP__SINK__ROOT | 字符串 | 无，必填 | 取用 | 变更后须重判；目录路径、挂载点或对象存储 URI |
| EP__SINK__CREDENTIAL_REF | 机密引用 | 无，必填 | 取用 | writer 身份，形如 secret://sink/writer#1；仅列举、CREATE_NEW 与必要校验读 |
| EP__SINK__RESTORE_CREDENTIAL_REF | 机密引用 | 无，必填 | 取用 | 独立只读恢复身份，平时封存，不得与 writer/disposal 共用 |
| EP__SINK__DISPOSAL_CREDENTIAL_REF | 机密引用 | 无，必填 | 取用 | 独立处置身份，仅双人审批与重新认证后临时解封，不得与 writer/restore 共用 |
| EP__SINK__MEDIA_TYPE | 枚举 ONLINE/OFFLINE/NONE | 无，必填 | 取用 | 变更后须重判；部署时判定结论，OFFLINE 即 RPO 降级 |
| EP__SINK__ROTATION_PERIOD_MINUTES | u32 | 空 | 取用 | MEDIA_TYPE 为 OFFLINE 时必填 |
| EP__SINK__PROBE_INTERVAL_SECONDS | u32 | 60 | 取用 | 可写性探针周期 |
| EP__SINK__UNWRITABLE_AFTER_FAILURES | u8 | 3 | 取用 | 本阶段新增决定 |
| EP__SINK__WRITABLE_AFTER_SUCCESSES | u8 | 2 | 取用 | 本阶段新增决定 |
| EP__SINK__READBACK_THROUGHPUT_MIN_MIBPS | u32 | 无，由认证报告冻结后填 | 启动 | 低于该值按第 13.3 章重估 RTO |
| EP__ARCHIVE__SLOT_NAME | 字符串 | ep_archive_slot | 启动 | 具名持久物理复制槽 |
| EP__ARCHIVE__WAL_SPOOL_MAX_GB | u32 | 350 | 启动 | 本机 WAL 暂存目录上限，占用附录 A.3 连续归档本机保留子项；数据库侧 max_slot_wal_keep_size 取同一上限，作为 pg_receivewal 停止时的兜底，由 database-reachable 自检核对；原键名 EP__ARCHIVE__MAX_SLOT_WAL_KEEP_GB 撤销 |
| EP__ARCHIVE__RETENTION_WARN_RATIO | numeric(9,6) | 0.600000 | 取用 | 第 13.4 章的 60% 告警阈值 |
| EP__ARCHIVE__WAL_WRITEOUT_PERIOD_SECONDS | u32 | 300 | 取用 | 15 分钟上限的三分之一，留两倍余量 |
| EP__ARCHIVE__ATTACHMENT_INCREMENTAL_PERIOD_SECONDS | u32 | 300 | 取用 | 同上 |
| EP__ARCHIVE__AUDIT_EVIDENCE_PERIOD_SECONDS | u32 | 300 | 取用 | 同上，与事务日志归档一致 |
| EP__ARCHIVE__SUSPEND_AFTER_MINUTES | u32 | 30 | 取用 | 落点不可写多久后转归档通道暂停 |
| EP__SPOOL__DIR | 路径 | 按服务模板：C:\EP\archive-writer\spool 或 C:\EP\backup-writer\spool | 启动 | 两 writer 的上报本地暂存，逐进程独立 |
| EP__SPOOL__MAX_BYTES | u64 | 21474836480 | 启动 | 两 writer 各自 20 GiB；软水位为减 64 MiB |
| EP__BACKUP__MODE | 枚举 normal/restore | normal | 启动 | 恢复模式触发方式，不新增命令行参数 |
| EP__BACKUP__RESTORE_PLAN_PATH | 路径 | 空 | 启动 | restore 模式必填 |
| EP__BACKUP__FULL_SCHEDULE | cron 表达式 | 0 1 * * * | 取用 | 每日全量备份窗口起点 |
| EP__BACKUP__ATTACHMENT_FULL_SCHEDULE | cron 表达式 | 0 3 * * * | 取用 | 附件正文每日全量写出 |
| EP__BACKUP__SPILL_MAX_BYTES | u64 | 53687091200 | 取用 | 50 GiB，附录 A.3 全量基础备份本机暂存子项 |
| EP__BACKUP__BOOTSTRAP_DEADLINE_HOURS | u32 | 无，必填 | 取用 | 引导窗口时限，由实施方估算并写入部署记录 |
| EP__BACKUP__VERIFY_DECRYPT_SAMPLE_RATIO | numeric(9,6) | 0.050000 | 取用 | 生产抽验比例，认证演练固定为 1.000000 |
| EP__BACKUP_ENCRYPTION__DBEK_REF | 机密引用 | 无，必填 | 取用 | 部署级备份加密密钥引用，含版本 |
| EP__BACKUP_ENCRYPTION__ALGORITHM | 枚举 | AES_256_GCM | 启动 | 首版只此一值 |
| EP__OPS__METRICS_LISTEN | socket 地址 | 127.0.0.1:9101 | 启动 | |
| EP__OPS__HEALTH_LISTEN | socket 地址 | 127.0.0.1:9102 | 启动 | |
| EP__OPS__WAL_RETENTION_SAMPLE_PERIOD_SECONDS | u32 | 30 | 取用 | 与附录 A.4 的 30 秒采样口径一致 |
| EP__OPS__CAPACITY_SAMPLE_PERIOD_SECONDS | u32 | 300 | 取用 | |
| EP__OPS__DISK_WATERMARK_RATIO | numeric(9,6) | 0.800000 | 取用 | 附录 A.3 的 80% 复核阈值 |
| EP__KEY_RECOVERY__VERIFICATION_INTERVAL_DAYS | u32 | 183 | 取用 | 每 6 个月核验 |
| EP__KEY_RECOVERY__SHARD_PICKUP_SLA_HOURS | u32 | 无，必填 | 取用 | 未填即不得宣称 4 小时 RTO |

历史数据迁移不新增 `EP__` 配置键。每块最多 1000 行且规范化 JSON 请求体最多 524288 字节、完整 HTTP 请求最多 1048576 字节、任务租约 60 秒、心跳 20 秒、一次性 API 会话 10 分钟、模板 schema 版本 1 与签名部署清单 schema 版本 1 都是首版协议常量，不允许客户用环境变量放宽；员工 API origin 只来自签名部署清单，来源地址、只读 DSN 名、Credential Manager 条目名、字段映射和 watermark 全在逐客户签名模板与批次审批内，不能变成进程级共享配置。支持套餐的版本与补丁状态回报周期同样不是软件配置：合同模板参数固定名 `PATCH_STATUS_REPORT_INTERVAL_DAYS`，允许 1 至 7 个自然日，未另选时默认 7；它属于发布前合同选择与签字门禁，不阻塞任何代码开始开发。

启动自检的本阶段落地分成两个具名项，每项只有一个 severity。

- `offsite-sink-requirements`（Degrading）精确实现七个判定：一，介质类型与落点类型判定存在；二，平台能以批次唯一 key 和 CREATE_NEW 自动写入并读回；三，写入失败可被平台检测；四，部署级备份加密密钥可解引用；五，writer、restore、disposal 三身份和三个 credential ref 均存在且两两互斥；六，落点侧防删/防覆盖策略证据已写部署记录，且 writer 的覆盖、删除、重命名、改权/ACL/policy 负向探针全部被拒；七，密钥恢复材料的分片取件时限已约定。第一项缺失开 `OFFSITE_SINK_NOT_CONFIGURED`；第二或第三项失败按实际写出窗口处理；第五或第六项失败开不可抑制的 `OFFSITE_COPY_PROTECTION_MISSING`；其余项按对应现行窗口如实降级。任一不满足都使 `--check` 非零并阻止发布，但服务按可用能力启动，连续归档、每日全量备份与获批恢复不得伪装成不可用；保护门通过也不得对外称 WORM 或不可变存储。
- `writer-role-containment`（Blocking）由本阶段新增，只对 archive-writer 与 backup-writer 适用并在任何复制连接建立前执行，其他进程返回 NotApplicable。它精确检查三项：角色 credential ref 对应的机密文件已断继承且 NTFS ACL 只授权对应服务虚拟账户，凭据不下发人类且不供其他进程；`ep_archiver` 与 `ep_backuper` 的 `pg_hba` 证明只含 127.0.0.1/32 与 ::1/128 回环放行；四类 IPC 上报路径及第 4.8 节周期交叉核对在部署配置中已就绪。任一项失败时对应角色不得启用，写出进程以退出码 78 退出；core-server 观察到进程未投入运行后开不可抑制的 `WRITER_NOT_IN_SERVICE`，basis 载明缺失项与起始时间。补齐并重新通过后角色才可启用。角色启用后的运行期 `NO_RESULT` 只开 `REPLICATION_CROSSCHECK_NO_RESULT`，不重跑或置失败本启动项，也不停止归档与备份。

`--check` 模式执行本进程适用的全部已注册项并按注册顺序输出结构化报告后退出，任一 FAILED 或 DEGRADED 都非零，用于部署与升级前置。两个写出进程只持 REPLICATION 属性连接，对全部 SQL 类自检项仍标 NotApplicable。

---

### 8. 测试计划

覆盖率门槛。本阶段全部 crate 属平台内核，行覆盖率不低于 85%；ep-bench 与 ep-release-gate 不进入发布制品，按其余代码 70% 计；新增与修改代码不低于 80%；工作区整体不低于 80%。工具 cargo-llvm-cov，CI 以 --fail-under-lines 强制，分档路径规则写入 codecov.toml。

#### 8.1 单元测试

- 水位推进算法：乱序提交序、单对象反复失败使水位停滞、pending 为空时的推进、引导期不产生水位、水位单调不减、对象删除标记不剔除。
- 归档通道状态机：九个合法 from/to 对逐项、迟滞带不抖动、Suspended 无自愈路径、进程重启后从持久状态恢复、三个断链态下 RPO 依据一律为 ArchiveChainBroken。
- 落点可写性判定：连续失败与连续成功阈值、MEDIA_TYPE 为 NONE 与 OFFLINE 的短路分支。
- 离站副本防删保护：key 批次唯一且不可复用、既有 key 条件写失败、三身份互斥、四类后端权限判定、任一负向探针未被拒即开不可抑制窗口、全部重验通过才关窗；不得把保护失败等同于落点不可写。
- RPO 依据判定：七种依据的成立条件、严劣序取值、两个 target 各自取值、对外披露取较大值、台账与部署记录不一致时取较差、任一降级态下不得输出 900。
- 备份集状态机：七个状态与其守卫，暂存缓冲触限中止，按 kind 精确方法集全 PASS 才 Verified、集合完整且任一 FAIL 才 VerifyFailed，三类终态只可一次处置为 Disposed。
- 信封加密：header 编解码、AAD 绑定对象身份、篡改密文与篡改 header 均校验失败、DBEK 版本切换后旧版本在轮换窗口内仍可解。
- 保留量判定：ratio 计算、0.60 触发、wal_status 取 lost 判失效。
- 复制交叉核对：MATCHED、MISMATCHED、NO_RESULT 三态全分支；连续第二个 NO_RESULT 开窗；随后 MATCHED 或 MISMATCHED 清零并关窗；错误只持久化清洗后的代码。
- 台账开闭：唯一约束下的幂等开窗、不可抑制 kind 的抑制被拒、抑制记名记时。
- 恢复点对齐：R 取两者较早、W_att 缺失分支。
- 历史迁移模板：签名与摘要、八种清洗操作、脚本/任意 SQL/白名单外网络全部拒绝、四项安全属性缺任一即失败、同版本不同摘要拒绝。
- 历史迁移状态机：第 4.12 节九组迁移与全部非法边；窗口到期、审批撤销、源摘要变化和租约丢失均停止；两轮不收敛必须产生三选一决定和新模板版本。
- 模块写入者注册表：25 个 `MigrationObjectKind` 逐项恰有一个属主，缺一、重复、错属主、向 crm/costing/portal/reporting 注册任一直接导入项均失败。
- 已知差异：四个可登记类别逐项通过；借贷平衡、库存守恒与四类不可豁免差异永远不能关联 APPROVED_DIFFERENCE。

#### 8.1.1 必跑 direct-SQL 负例

全部用普通 `ep_app_rw` 权限与真实 PostgreSQL 16 执行，并在每例显式 `SET CONSTRAINTS ALL IMMEDIATE` 或 COMMIT 触发延迟图；只验证 HTTP/仓储拒绝不算通过。

1. 对十张 APPEND_ONLY 表逐张 UPDATE、DELETE，及对 deployment_records/offsite_sinks 改 superseded_at 之外任一列、回开哨兵、无后继闭合，各自必须被数据库拒绝；表 22、23 的直接 INSERT 必须权限拒绝。
2. 插入不存在 sink/writeout/backup/material/window 的引用、slot 指错非 RUNNING 备份、两个 RUNNING 备份或 RUNNING 不占 slot，均必须失败。
3. 直接把 PLANNED 改 VERIFIED、缺写出回执、错 channel/sink/started/finished/bytes、未结束 writeout、OK 携失败字段或失败回执缺错误、在非校验终态偷插 verification、缺/多/重复校验方法、校验早于 written_at、concluded_at 不等最大 finished_at、含 FAIL 却 VERIFIED、全 PASS 却 VERIFY_FAILED，均在提交时失败；合法事务允许先写子后写父并提交成功，证明 DEFERRABLE 而非写序依赖。
4. 非法备份边、RUNNING/WRITTEN 同态改写 kind/sink/encryption 或任一执行证据、终态字段改写、无销毁证书处置、从 WRITTEN 直接 DISPOSED、处置后改回 VERIFIED，均失败。
5. archive_channel 缺版本证据、错 from/to version、typed after-image 与当前行不等、STATE_CHANGE 用非法边、OBSERVATION 偷改生命周期列/LSN 回退、核对 at 回拨或 outcome/at/streak/error 形状不符、NO_RESULT 连续计数跳号、STATE_CHANGE 偷清 streak、断链字段不全、Rebuilding 指日常备份或错误状态、SUSPENDED 指非失败基线或指 disposed_from_state=VERIFIED 的已处置基线、只重建槽即 Healthy、伪造断裂历史及 last_transition 指错行，均失败；MATCHED/MISMATCHED 归零、首个/连续 NO_RESULT、失败基线处置前后两种合法 SUSPENDED、九个合法 from/to 对和同态 OBSERVATION 分别提交成功，完整历史可从初始行重放到当前行。
6. recovery drill 指未 VERIFIED、CONFIG_BUNDLE/ATTACHMENT_FULL、错 sink/错 verified_at 的备份、RUNNING 预填结果、结束早于开始、PASSED 携 failure 或 WHOLE/PRODUCTION PASSED 缺完整指标、rto 小于已计入耗时之和/大于 14400、LATEST_VERIFIED 的 RPO 半组/任一值大于 900、attachment failed>0、FAILED 缺 failure/附件、不变量或 RPO 原子组只填半组、KEY_MATERIAL_ISOLATED 填入数据恢复/RPO 指标、decrypt 已填但 shard 为空或 PASSED 缺 shard/decrypt、RETENTION_TAIL 用错 drill kind/写 RPO/备份年龄不足，以及终态改写，均失败；三种 drill_kind 的合法 PASSED 与至少两个不同失败阶段的 FAILED 形状各提交成功，备份在演练开始后合法处置不破坏历史演练。
7. approval evidence 直接 INSERT 权限拒绝；受控函数遇到错法人/batch/difference、task 不属 instance、instance/definition 长 FK 不等、task 非完成、approval_ref 不等预生成 evidence id、reauth_ref 为空/错 challenge、challenge 未消费/非 DATA_MIGRATION/非 submitted_by、subject digest 错任一键、拿 KNOWN_DIFFERENCE_DECISION 挑战重放撤销、七键 variables 缺/多/错一键、审批人与申请人相同、definition hash/version 漂移、来源 schema 指纹漂移、APPROVED 缺只读负测引用或该引用未进 hash、模块码越界/差异 module 不属 batch/缺模块 owner、candidate_role_codes 非静态单元素映射、role id/code/grant 不配、授权未来生效/已过期、角色非 EFFECTIVE 或未启用、DRAFT/PROPOSED 改审批内容但 content_version 未精确加一、旧 content_version 重放，均失败；四个定义码与 approver_kind 的错映射同样失败。合法 APPROVED、REJECTED、REVOKED 各至少一例只从当前业务行重算 reauth/content hash并派生角色授权快照，数据库与流程表均不存在审批内容副本或伪造 owner_module=`platform`。
8. 记录 run/chunk/seq 非正、module/object 不在 25 闭集、三项映射只填半套、key domain 不存在/他法人/非 ACTIVE、QUEUED 偷填预留、VALIDATED 缺 target type/id、target type 不等 catalog、target id 非服务端 UUIDv7、预留时 owner relation 已有同法人根、两记录预留同一根、VALIDATED 同事务偷建根、从 VALIDATED 失败却清空半套预留、预留后改 type/id、APPLY 未使用预留 id、owner 目标安全属性与映射不等，以及直接把记录改 APPLIED/REVERSED 但缺 receipt，receipt 错 batch/record/run/module/object/target、重复 APPLY、REVERSE 指别条记录 APPLY、静态 25 分支取未登记 object kind 或错 owner，均失败；受控函数传不存在目标、他法人目标、错 idempotency key时拒绝，调用方伪造 target/effect digest 没有入参可用。REVERSE 另逐项拒绝：无 event_id=receipt id 的 R0、action/type/id 错、after 六键缺/多/任一值错、交易对象绕过 catalog 既有取消/冲销/更正通道、可变根没有 catalog 指定 version/change fact、任意旧 owner fact 或只写审计无 owner effect。合法 owner writer 的业务行、静态规范投影、R0、receipt、状态、事件/Outbox 任一注入失败时整块零写入。
   第 8 组对采购订单、付款申请、资金账户三条 audit-target 分支再逐项固定负例：owner event 与 R0 同 id、跨法人、occurred_at 不等、receipt target 仍指业务根、R0.owner_effect_id/type 不等、owner action/object/root/object_version/reason 不等、before/after 缺键或多键、row_version 写成 JSON number/空字符串/负数/前导零/溢出字符串、row_version/state edge 不合法、owner event.after 与最终根不等、保留态偷增版本、变更态未增版本、采购订单/付款申请的终态时间或 reason 与固定分支不符、资金账户 deactivated_at 形状不符，以及仅写 owner audit/R0 而未完成依赖守卫，均在 COMMIT 拒绝。三条合法分支的 target canonical projection 必须逐字为 catalog 指定的 owner_audit + root_after + R0，不能退化成一条通用 audit JSON 分支。
9. reconciliation 引用别批差异、四类不可豁免项取 APPROVED_DIFFERENCE、差异未 APPROVED/已 REVOKED、required key 缺失或多余、current run 记录数与 manifest 不等、含 FAILED 记录或 FAIL 对账时伪造 READY_FOR_CUTOVER，均失败。
10. 缺 BATCH/CUTOVER/REVERSAL 精确批准集、任一批准内容 hash 不等、租约未清、reconciliation digest 漂移、cutover 后撤销差异、CANCELLED 缺前态/时点/原因或伪造冻结后取消、当前 run 未全量转 REVERSED 或缺任一 REVERSE receipt 却置批次 REVERSED，均失败；六表在同事务任意合法写序均可提交。
11. 迁移结构证据在干净 PostgreSQL 16 库执行：应用 092500、092600 后从 `pg_constraint` 逐条断言本文全部 FK 的列序、父候选键、`confdeltype='r'`、`condeferrable=true`、`condeferred=true`，并断言 target reservation 唯一键列序；从 `pg_trigger`/`pg_constraint` 断言两个 graph function 只以 constraint trigger 附着到冻结表集，从 `pg_class` 断言六表 `relrowsecurity=true and relforcerowsecurity=true`，从 `pg_proc`/`pg_roles`/ACL 断言两函数 owner、精确参数/返回型、固定 search_path、PUBLIC 无权且仅 ep_app_rw 可 EXECUTE，并逐项比对 append-only/immutable registry 与表权限。逐字段断言 permission `...0315`、binding `...0509` 恰等第 3.3 节，零自动 role grant；同 id/code/object_type 任一错字段夹具必须令 092600 在其他改动前失败。另从 CHECK/静态快照断言 Stage 8 MIGRATION_HISTORY 三 direction tuple 与 pricing branch、Stage 9 第 19 个 HISTORICAL_MIGRATION source/专用入口/完整镜像图、25 行 relation/child/order/reverse channel 全部一致。空证据克隆必须能按 092600→092500 反序完整回退并再次前滚，schema_history 每版始终恰一行；任一图表写入最小合法证据后，两条 rollback 都必须在任何 DROP/REVOKE/DELETE 发生前退出非零，前后 catalog 快照逐字相等，证明失败回退零部分拆除。

#### 8.2 领域属性测试

用 proptest 生成随机的元数据提交与正文写出交错序列、迁移分块与重放序列，验证六条不变量。
1. 水位单调不减。
2. 对任一水位取值 W，在 W 之前提交的元数据集合是已完成写出对象集合的子集。
3. 对任一 (W_db, W_att)，取 R 等于较早者后，恢复点上元数据存在则正文必然存在。
4. 任意分块、乱序重送与 worker 接管下，同一 `(batch_no, object_type, source_locator_sha256)` 最多预留一个 target root、最多产生一次正式 APPLY 与一次 catalog 具名 REVERSE 效果。
5. 任一 QUEUED 记录无 target reservation；任一 VALIDATED 记录有唯一且尚不存在的同法人 catalog root reservation；FAILED 保留其合法前态的零套或完整套 reservation 且无 receipt；任一 APPLIED 记录的实际根 id 等于 reservation 并恰有一个同法人 APPLY receipt 与 32 字节目标摘要；任一 REVERSED 记录还恰有一个指回该 APPLY 的 REVERSE receipt、catalog owner effect 与 event_id=receipt id 的 R0 审计。
6. READY_FOR_CUTOVER 蕴含内部两项不变量 PASS、四类不可豁免差异为零、其余 FAIL 集为空或逐项关联三方批准的已知差异。
前三条直接对应附录 A.6 的附件一致性判据，也是规格第 13.4 章“不得出现元数据在、正文不在”的形式化表达；后三条对应规格第 7.10 章的幂等、错误隔离与切换守卫。

#### 8.3 集成测试

一律使用真实 PostgreSQL 16，每用例独占一个 ep_test_<nanoid> 库，用例结束即删库。落点用真实本地目录与真实 SMB 共享挂载（落点类型枚举 NFS_SMB_MOUNT 不改名，本平台的集成测试以 SMB 一支落实）；对象存储落点用本机 S3 兼容打桩，另提供一套契约测试跑客户对象存储沙箱。

场景清单。
1. 复制槽建立、监管的 pg_receivewal 接收 WAL 并推进确认位点、正常写出到落点、周期不超过 15 分钟；pg_receivewal 被外部杀死后由本进程重启并从既有槽续接，不丢段。
2. 两条堆积路径各一次。其一，本机 WAL 暂存触界：注入落点不可写并持续写入负载，验证暂存占用达 EP__ARCHIVE__WAL_SPOOL_MAX_GB 即判归档链断裂，其间 pg_wal 不因该槽增长、实例保持可写。其二，pg_receivewal 停止后的槽滞留：验证保留量到 60% 告警且不触发任何备份动作，到 max_slot_wal_keep_size 上限时数据库回收未确认日志使槽失效、实例保持可写。这一项同时是规格第 17.2 章混沌场景中磁盘写满一类的实现证据。
3. 归档链断裂两支：落点可写支走删槽、建槽、重建基线备份、自动校验、闭窗口的完整路径，验证仅重建复制槽不闭窗口；落点不可写支进入暂停态，验证不重建槽、不执行备份、实例可写、窗口不闭合、落点恢复后自动转入重建支。
4. 三类成因逐条：archive-writer 停止、archive-writer 长时间不推进确认位点、落点长时间不可写，验证均按断链处置而非只按进程重启处理。
5. 附件增量写出与水位推进，含大文件与接近 5 GB 单文件上限的对象。
6. 审计证据写出周期不超过 15 分钟，写出对象与段根签名一致。
7. 每日全量备份流式写出、暂存缓冲峰值、四种自动校验方法。
8. 断链重建基线备份与每日全量备份的串行，验证在用流复制连接不超过 3、在用复制槽不超过 2；该取值是 pg_basebackup 以 -X stream 形态占两条连接与一个临时槽后对规格第 7.7 章 backup-writer 一栏的重取，由本阶段回写该章。
9. 复制交叉核对三态：先断言两个复制连接的 `application_name` 分别是 `archive-writer`、`backup-writer`，且只接受 `archive-writer↔ep_archiver`、`backup-writer↔ep_backuper` 两个映射。数据库侧与按 `(occurred_at, report_id)` 重建的最新有效上报完全一致时产出 MATCHED；交叉角色映射、一个白名单外复制槽、一条非写出进程复制会话及一条报告侧幽灵记录时分别产出 MISMATCHED、在下一次 30 秒采样内告警并写审计；缺 `slot_name`或 `backend_pid`、三个输入任一不完整和连续两轮查询超时都产出 NO_RESULT，在连续第二轮打开 REPLICATION_CROSSCHECK_NO_RESULT，随后恢复为 MATCHED 时关窗。另以 `spooled=true` 且入库顺序与 `occurred_at` 相反的两行验证仍按发生时间重建。全程未使用独立连接、独立表、独立指标或独立配置键。
10. 部署级备份加密：落点上全部写出对象为密文，无恢复材料时无法读出任何业务数据，含未被字段级加密的明文业务表内容；以写出组件系统账户之外的身份读取落点被拒绝并告警。该项直接对应规格第 17.2 章数据保护控制与销毁证明测试的落点判据与第 22 章第 8 条。
11. 两个专用角色的越权测试：无法读取任何业务表、无法执行任何 DDL、无法从服务器之外建立连接、无法经界面与 API 借用。该项属发布门禁与第 7.3 章数据库认证套件必测项，并入 tests/rls_matrix 目标执行，断言经阶段 2 按 C-05 提供的 assert_replication_role_containment，本阶段不重复实现同名断言函数。
13. 时间点恢复：把库恢复到指定 R，验证数据一致。
14. core-server 不可用期间 WAL 接收与当前写出继续，上报进各自 spool；五类 critical 证据在软/硬水位下均不删除不覆盖，恢复后按发生时间补写不重不漏。软水位阻止新备份/附件周期、写 Windows Event Log，恢复后 `WRITER_NOT_IN_SERVICE` 不可抑制窗口先开后关；仅可重建 heartbeat/progress 允许合并，且重建结果与本地 manifest/落点清单一致。
15. 混沌与故障注入六类：依赖服务超时、连接池与内存资源耗尽、消息积压、系统时钟漂移、磁盘写满、进程崩溃后重启恢复；预期行为为核心交易按第 15.1 章返回可重试或明确失败、不产生数据不一致、故障移除后 5 分钟内自愈；进程崩溃场景另验证重启后未完成任务自动恢复、已确认事务零丢失。
16. 台账二十一类 kind 的开闭各一条，其中 RECON_RUN_UNFINISHED 与 PERIOD_CLOSE_ACCEPTANCE_REJECTED 两类由 ep-platform-recon 与 ledger 侧触发，PORT_NOT_IMPLEMENTED 一类由各调用方按端口开闭并以 subject 区分、本阶段以 DisposalPort 一支为样本，AUTHZ_SNAPSHOT_CHECKSUM_MISMATCH 与 CUSTOM_OBJECT_DDL_INCONSISTENT 两类分别由阶段 4 与阶段 13 触发，这五类本阶段只验证受理与展示；WRITER_NOT_IN_SERVICE 按写出进程停止与连续两周期无上报两条触发路径各验一次，REPLICATION_CROSSCHECK_NO_RESULT 按连续两个 NO_RESULT 开窗并由下一次有结论关窗；VIRUS_SCANNER_NOT_AVAILABLE 验 NONE 常开不可抑制，以及 CUSTOMER_ICAP 的 CLEAN 关窗、INFECTED/超时/不可达/非法响应隔离并开窗、不回退 NONE，另验主机名、重定向与非回环 URL 启动失败；LEGAL_ENTITY_KEY_DOMAIN_UNAVAILABLE 按无当前 key domain、KMS/HSM 解封失败、EPS1 migration receipt 失效三支开窗，并只在原 provider 恢复、receipt 有效且真实解封成功后关窗。再验一次同一 kind 下 subject 不同的两条窗口同时活动，唯一约束不误判为冲突。
17. 对外表述门禁正反用例：一份只含已认证事实且逐条带全前提的第一档清单经产品负责人签字后通过；分别注入一条第二档比较级、一条第三档禁用承诺性表述、一条未经实测比较级，以及 NONE 部署缺少“平台未提供病毒防护”逐字披露时均失败。CUSTOMER_ICAP 只有同机扫描器实测、版本与病毒库责任边界完整留证时才可描述为“已接入客户扫描器”，仍不得把客户扫描器效果表述为平台自带防护。首版发布值已固定为第二档零条、第三档零条，即使第二档附有报告也必须失败；未来若改变只能另立版本与正式裁定，不是本版预留分支。将已签材料改动一个字节或替换任一附件后，原签字必须失效，重新逐条判定并签字前门禁保持失败。
18. 四类迁移来源各走一次真实适配：XLSX 与 CSV 各一份；PostgreSQL 测试源经只读 ODBC DSN 读取且写入负样例被源端拒绝；本地与 SMB 文件清单各读一批附件；HTTPS API 以 TLS 沙箱读取并验证域名、端口、路径、方法、字段、重定向和凭据白名单，任一越界拒绝。四支都不把来源凭据或原文写入数据库与普通日志。
19. 完整试运行覆盖 25 个对象类型，逐条 `validate` 并执行九类对账，断言目标业务表、文件对象、既有业务领域事件与 Outbox 都零新增；迁移运行摘要、错误队列与状态审计按第 4.12 节正常增加。注入一条必然失败记录后它只进入表 19 错误队列和审计，不产生正式数据效果。修复来源后以新 run 重跑，旧运行证据不覆盖。
20. 正式基础批与增量追平覆盖 25 个对象类型；每条 VALIDATED 先产生 catalog 固定 relation 的唯一空根预留，正式 writer 逐项使用该预留 id。同块重发三次、乱序发送、core-server 重启与 worker 租约转移后，逐来源键、target reservation 与正式目标都恰有一次效果；已存在根、重复预留、错 relation、错 target id 逐项拒绝。任一块中间一行失败时整块业务写入、审计、receipt 与 Outbox 同时回滚。
21. 只读冻结与源变化：机器可读证据支、源管理员加数据责任人双签支各一条；冻结后新增来源行、修改既有行、schema fingerprint 变化和 manifest 变化四种情形均转 TRIAL_FAILED 并禁止切换，正常 watermark 追平支进入 RECONCILING。
22. 对账与已知差异：九种 check_kind 全覆盖；借贷不平与库存不守恒无论提交何种审批都失败；四个可登记类别各经三方审批一次后可关联；切换基准日的四类不可豁免差异各注入一条并逐条阻断；全部归零或合法批准后才进入 READY_FOR_CUTOVER。
23. 切换与关窗：缺任一模块责任人、数据责任人、客户财务负责人、最终报告、冻结证据、有效 X-Reauth-Token 任一项均拒绝；全齐后 CUTOVER_COMPLETED、窗口同事务关闭且迁移写入口失效，再次打开必须新建批次。
24. 整批冲销：25 个 writer 先按第 4.12.1 节逐行产出唯一 reversal plan 才执行；注入一个不能计划的模块时零模块产生冲销。完整支逐模块只走 catalog 具名 owner 变更/取消/冲销/更正通道，交易对象产生新反向 fact，可变主数据产生具名 version/change fact加根 after-image；project、complaint、equipment 三支还必须分别新建 `project_migration_corrections`、`customer_complaint_migration_corrections`、`equipment_migration_corrections` 并以新 fact id 为 target，改变态与终态 retain 各一例。采购订单、付款申请、资金账户三支各跑真实变更与终态/已停用保持分支，必须新建 action 分别为 `PROCURE_PURCHASE_ORDER_MIGRATION_REVERSED`、`PROCURE_PAYMENT_REQUEST_MIGRATION_REVERSED`、`FINANCE_CASH_ACCOUNT_MIGRATION_REVERSED` 的独立 owner audit，以其 event_id 为 receipt target，并让同 occurred_at 的独立 R0 指回该 event；逐项核 before/after、根最终状态/版本和 owner_audit+root_after+R0 projection。每条 REVERSE receipt 均有同 id 的 R0，target projection 由数据库静态分支重算。正反批次引用保留并重跑九类对账，原业务记录、迁移台账与审计均未删除；直接 UPDATE 历史交易、三类根 after-image 冒充 correction target、复用旧 correction、无 version 的根改写、任意旧 fact、owner/R0 复用、孤立审计和通用 reversal JSON 表逐项失败。
25. 法人隔离与最小权限：表 18 至表 23 六张迁移表执行读取、写入、更新、聚合、排序、错误泄漏八类 RLS 矩阵；跨法人批次、记录、对账、差异、批准证据和 writer receipt 均不可见。全部迁移路由逐项断言只用 `platform.data_migration`，GET/CREATE/UPDATE/SUBMIT 动作与 batch object id 映射精确；没有显式 role grant 时默认全部拒绝，授 VIEW 不得写、授 CREATE 不得执行、授 UPDATE 不得发起审批、授 SUBMIT 不得越过职责/流程/reauth，子对象 id 不得代替 batch id 绕过范围。`ep-data-migrate` PE 中无目标数据库驱动配置与连接串入口，源凭据不出 Credential Manager，上传会话首次换取即删本地文件且 10 分钟或关窗强制作废。
26. 离站防删保护逐后端实测：对象存储 writer 的覆盖、DeleteObject/DeleteObjectVersion 与策略/生命周期修改全被 IAM 显式拒绝且 `If-None-Match: *` 生效；Windows/SMB writer 的覆盖、删除、重命名、WRITE_DAC、WRITE_OWNER 全被 DACL 拒绝且 `CREATE_NEW` 生效；认证的 NFS 落点以服务端 NFSv4 ACL 证明 ADD_FILE 与 DELETE/DELETE_CHILD/WRITE_ACL/WRITE_OWNER 分离。另以普通 POSIX/NFS 可写目录作负样例，证明仍能写新文件但能删/改既有文件时 `offsite-sink-requirements` 为 DEGRADED、`OFFSITE_COPY_PROTECTION_MISSING` 不可抑制且发布门失败。三支均验证 writer、restore、disposal 凭据两两不同，writer 不能清空历史副本；disposal 身份仅在双人审批与重新认证后删除批准的精确 key/版本，部分删除不生成成功证明。

#### 8.4 端到端与演练

- 附录 A.6 整机失效恢复，至少两次，两次均达标。判定项逐条：RTO 不超过 4 小时（含解密耗时、附件逐条一致性校验耗时与第 17.3 章全部强制不变量校验耗时）、RPO 不超过 15 分钟且对事务数据库与附件正文同时成立、恢复后通过第 17.3 章全部强制不变量校验、每条附件元数据都能找到对应正文且正文校验和与元数据记录一致。落点固定为在线可写类型，取另一台机器上的目录或客户对象存储二者之一，落点类型与实测持续读回吞吐记入认证报告。抽样校验不成立，必须覆盖全部附件对象。
- 附录 A.6 密钥恢复材料隔离恢复，至少两次，两次均达标。在无原运行环境密钥的隔离环境中只装载备份的分片恢复材料，完成一次解密与恢复，覆盖客户自带密钥场景，恢复数据通过第 17.3 章强制不变量校验。
- 两类演练的分片取回与双人控制耗时单独留证并注明未计入 RTO 判定。
- 两类演练从第一次起均使用阶段 9 冻结的恢复模式四项有界默认值：单批 5000 行、单批 300 秒、单查询内存 128MB、临时空间 2GB；各须记录配置版本、各批实际行数与耗时分布、单批最大耗时、实测总耗时、单查询内存与临时空间占用峰值。任一触限即本次不达标；在允许范围内经签名配置调整后必须从头重跑两次，旧证据失效，两次中取较不利的一次作为发布证据。
- 附录 A.6 保留期尾端恢复，一次，按裁定 F-11-4 进发布门禁。判定项集合单列，只判四项：RTO 不超过 4 小时、数据完整性、恢复后通过第 17.3 章全部强制不变量校验、每条附件元数据都能找到对应正文且正文校验和与元数据记录一致。RPO 一项对该次演练不适用，理由同处写明：该次演练的恢复目标点由备份保留期决定、不由归档周期决定，按 15 分钟判它必然不达标。附录 A.6 的「两次均达标」一句对该次演练按其自身判定项集合判，不与整机失效恢复两次混判。
- 上一条的备份集判据取相对量、不取绝对天数：该次演练所用备份集的 verified_at 与该次演练开始时点的间隔不少于 D 减 1 天，且该备份集在演练开始时点仍处于有效保留期内——后半条取库内事实判定，即该备份集在演练开始时点仍存在、未被销毁、状态为已校验通过，**不再叠加任何以 D 为上界的折算**（保留期尾端那一份的年龄本就不小于 D，叠上界会把一次正确挑中锚点的演练判成不达标）。D 为备份保留期，规格第 13.4 章认证取值 14 天。两个量都在演练报告与 backup_sets、recovery_drills 两处取得，证据包采集时点即可算定、事后重算结果不变；它测的是保留期尾端那一份还能不能恢复这件事本身，而不是「哪一份最早」这个随回收任务漂移、不可复算的名字。D 由客户改小时该判据随 D 变，不改判据文本。该次演练的判定结论进入发布证据包，由 ep-release-gate 按附录 A.5 与第 22 章逐条判定，不新增门禁项标识。演练报告须载明所用备份集标识，未载明即本判据无取数落点，判定为不达标。该次演练在 platform_ops.recovery_drills 上的登记形态（drill_kind 取值与所用备份集的记录列）随 F-11-4 的其余部分与第 3.1 节表 16 同批收口，本节先给判定项集合与判据，收口完成前判据取数以演练报告为准，与本节其余演练报告的判读方式一致。
- 历史数据迁移端到端验收以一套包含主数据、合同、销售与采购未完成单据、库存和财务期初、历史凭证、投诉/工单/设备、项目任务及附件的客户化样本执行完整 `trial → freeze-source → apply → delta-catchup → reconcile → cutover`；切换后黄金闭环可继续处理未完成单据，全部 25 类来源可按批次追溯，随后在隔离副本执行一次整批冲销并通过同一九类对账。该验收不以 schema DDL 迁移、日常 Excel 导入或四条期初通道单独通过代替。
- 跨平台基础备份不可移植，按裁定 F-08 第 4.5 节第 4 条：既有 Linux 集群的 pg_basebackup 产物与其后的 WAL 归档链在本平台不可恢复。三条后果逐条落地。其一，由既有 Linux 集群割接到本平台只能走 pg_dump 与 pg_restore 的逻辑迁移，不得以基础备份加归档回放的方式割接。其二，本阶段全部恢复演练的实证记录必须在本平台重做，Linux 上跑出的演练记录一条都不能沿用。其三，演练的目标实例必须与源实例是同一 Windows 发行版，按第 8.5 节的被测机器口径即 Windows Server 2022。第 4.5 节的恢复点对齐与回放算法本身不受影响，改的只是备份从哪来这个前提。

#### 8.5 性能与容量认证

按附录 A.1 至 A.4 在 BC-1 基线组合上执行一次完整基线测试。数据集由 ep-datagen 按附录 A.3 产出并版本化冻结。

被测机器口径按裁定 F-08 第一节结论二与补裁己：BC-1 的操作系统列取 Windows Server 2022，本节全部实测与第 8.4 节全部演练一律在该版本上执行。目标版本区间为 Windows Server 2019 至 2022，认证取值冻结在 2022；2019 可在同一形态上运行，但不在首版认证组合内，也不在附录 D.3 的单维度替换清单内，在 2019 上取得的任何实测数据不进入本附录基线、不写入认证报告，也不得据以声明 2019 已认证；裁定第一节结论三所说的「在 2019 上做一次同项复核」，其对象是该裁定第十二节保留原编号后的 17 项有效机制实测（原编号 12 已撤销），**不是本节的整轮基线测试、也不是第 8.4 节的恢复演练**，本节不据此新增任何一轮复跑。该取值的对价——认证有效期覆盖到 2022 的扩展支持终点而不是 2019 的，日后需要 2019 背书须另立一次认证运行——按同一裁定写入交付说明，不得沉默。

必判项，任一不成立不得出具认证结论。
1. 三类写出周期均不超过 15 分钟。
2. 该次每日全量备份在业务负载稳定段内完整完成。
3. 附件正文每日全量写出按 800 GB 全量计完整完成，不得抽样，不得复用增量字节数。
4. 备份自动校验对该次实际产出的全量备份与附件全量写出结果完整完成。
5. 每日内部对账覆盖 2 个法人与 36 个会计期间完整完成，且其实测总耗时折算后落在一个自然日执行窗口内。
6. 附录 A.2 的全部时延通过线成立，且在备份窗口内的样本子集上同样成立，该子集每场景不少于 40 个样本。
7. 四个具名池的常驻连接峰值不超过 37、临时连接不超过 10、总硬峰值不超过 52，并始终保留 5 个安全余量；integration-gateway 数据库连接为 0。
8. 每场景样本不少于 200 次，单次运行错误率不超过 0.1%。

必记项。三项写出的周期分布与字节量对比、按稳定段折算的事务日志生成速率、附件新增字节数、备份起止时刻与传输字节数、两个写出进程实测的磁盘读写字节量与持续吞吐绝对值、备份窗口内外各自的 P95 与 P99 与最大值及其差异超过 30% 时的原因、对账起止与分批耗时分布与资源峰值、期间关账窗口的起止与结束方式与受理前提逐项判定结果与顺延入账凭证张数、复制连接与复制槽在备份窗口内外的峰值、pg_wal 实测峰值占用、磁盘五项实测占用与合计值、资源单位（具名 Job Object）的内存硬上限取值。

配额三列的记法按裁定 F-08 第 4.1 节与补裁乙、补裁壬及 F-55 收口，不按原 cgroup 口径记，也不记任何份额百分比。其一，按权重的磁盘 IO 份额一列在本平台无运行期承载，本节只记上面那项绝对字节量与持续吞吐；全量备份写出的磁盘 IO 绝对上限实现路径固定为部署侧静态限额文件、部署记录与 Windows 校验夹具，按补裁乙不进规格第 13.1 章配额表；实机证据形成前能力状态为 `UNVERIFIED`，本项不计入覆盖。其二，CPU 一列首版固定只作硬件标定与认证意图声明，不落运行期取值，也不存在实测后自动启用的当前版本支路。其三，内存硬上限一列是配额表在本平台唯一有运行期承载的一列，其承载分两类：F-55 后九个自研二进制各自所属的资源单位由服务宿主层在 ServiceMain 早期读取部署侧静态限额文件后创建或打开并自我指派，其中 `ai-inferer` 使用原“内置搜索索引”行改名后的独立 `APP_AI` 资源单位；PostgreSQL 16 与反向代理不链接该层，唯一实现路径是由运维代理（ops-agent）创建具名资源单位后以 AssignProcessToJobObject 指派，实机读回证据形成前这两行状态为 `UNVERIFIED`、不得记为已覆盖。AI 的独立内存硬上限与资源认证另按 F-55/阶段 13c 判定。其余两个 `UNVERIFIED` 能力的转绿谓词各自固定：backup-writer 绝对 IO 上限须由 Windows 夹具读回并证明实际限速与静态文件一致；PostgreSQL 16 与反向代理须由部署校验脚本从各自具名资源单位读回内存硬上限并与静态文件逐行一致。静态文件仅出现取值行不能替代运行期读回。谓词均由判定工具观测，不需要任何人选择实现方案。

现行解释：实现路径已经全部冻结。CPU 比例与按权重磁盘 IO 份额在首版固定不启用；PostgreSQL/反向代理指派与 backup-writer 绝对 IO 上限必须按上段主路径实现，但在 Windows 证据形成前能力状态固定为 `UNVERIFIED`、不计入覆盖、发布门禁保持非零。夹具在 Server 2022 主测并在 Server 2019 做同项复核；只有读回与行为证据通过后，后两项能力状态才可转为 `VERIFIED`，失败时保留实现、维持未验证和保守披露，不切换第二套实现。

期间关账为必测必记项而非达标项，不设通过线，但未按附录 A.4 实测即不得冻结 A.1 该项取值，也不得出具认证结论。

#### 8.6 安全与供应链测试

- SAST 取 clippy 全 lint 加 -D warnings、cargo-audit、cargo-deny、semgrep 规则集；DAST 对 core-server 与 portal-gateway 的 HTTP 面执行；模糊测试用 cargo-fuzz 覆盖信封解码、IPC 帧解码、manifest 解析三个解析面。
- 依赖、安装包与密钥三类扫描进 CI，其中安装包一项的被测对象是同一份安装包（MSI 或压缩包）及其内的 PE 二进制，密钥扫描覆盖全仓库历史。原「容器扫描」按裁定 F-08 第 4.4 节换被测对象：首版不产出容器镜像，该项原样留着即无被测对象，按通则第六条换成上述可判定替身，不留恒真项。
- 第三方渗透测试结论为严重与高危发现全部关闭，分级按第 17.4 章 CVSS v3.1 口径。
- 生产 MSI、PE 二进制及其他 Windows 可执行制品必须 Authenticode；证书可由软件厂商或客户提供。开发与内部制品可用 ECDSA P-256 打通清单和离线验签，但元数据必须标记为开发签名，发布门禁必须拒绝其进入生产。模块与插件继续按各自签名清单验签，不得以内部 ECDSA 冒充 Windows 生产签名。
- SBOM 取 CycloneDX 格式随每次构建产出；构建来源证明随离线包一并交付；可复现构建以固定 `rust-toolchain.toml`、`SOURCE_DATE_EPOCH`、`--remap-path-prefix` 与离线 vendor 目录实现，由 `cargo xtask ci` 第 8 阶段在 Windows agent 做两次独立构建比对。PE 字节一致性与三个 PostgreSQL `.exe` 的目标平台行为尚无本文件所见实机证据，因此对应阶段必须保持非零且不得声称通过；`.github/ci/pipeline-stages.tsv` 的历史状态不构成证据。
- `ep-data-migrate` 的 XLSX/CSV、ODBC、文件清单与 HTTPS 解析面加入 cargo-fuzz 与恶意样本库；至少覆盖压缩炸弹、实体或公式注入、畸形编码、超深嵌套、路径穿越、SMB 符号链接/重解析点、SSRF、DNS 重绑定、跨域重定向、响应字段超集、单记录/块超过 524288 字节、完整请求超过 1 MiB、伪造或篡改员工 API origin、回环/直连 8080 与证书主机名不符。该工具随产品交付，必须进入产品 SBOM、安装包扫描、Authenticode 验签与两次可复现构建；不得沿用 ep-bench/ep-release-gate 的工具排除规则。
- `ep-secretctl` 与运行时 secret-store 的 cargo-fuzz/恶意样本集合必须覆盖 `SecretRef` 与 `bootstrap://` 规范化、`EPS1` 头/长度/AAD/nonce/tag/尾随字节、DPAPI bootstrap entropy 与 recipient 交叉替换、HSM slot/PIN/object 不可用且不回退、`put→签名配置切换→retire` 显式轮换、legacy DACL/reparse/ADS/hardlink/UTF-8 trim 迁移、CREATE_NEW staging/原子发布/中断恢复、fresh/migration receipt 与 `SecretTerminalEvidenceV1` strict-JCS/签名/digest。每个解析面均含截断、超长、unknown field/flag/algorithm、错 deployment/recipient/ref/key-version、旧版本隐式 fallback、明文输入面与第 8.7 节六个具名发布负例；任一负例返回 0、泄露 secret/secret hash 或留下 legacy/quarantine/staging 即失败。该工具与 `ep-data-migrate` 同样进入产品 SBOM、依赖/安装包/密钥扫描、生产 Authenticode 验签与 Windows 两次可复现构建。
- 勒索恢复信任边界按 `docs/threat-model.md` 的当前仓库级模型执行：以已攻陷应用服务器且取得 writer 身份为前提，验证攻击者仍不能删除、覆盖、重命名历史副本或修改存储策略；同时保留客户存储管理员可绕过的负面结论，不得把本控制测试命名为 WORM 或不可变存储认证。

#### 8.7 发布门禁项清单

ep-release-gate 逐项判定，判定结论进入发布证据包，任一为否即不得发布。

| 门禁项 | 判据 | 判据提供方 |
|---|---|---|
| RG-CI-PROBE-ABSENT | 发布制品的 cargo tree -e features 输出中不含 ci-probe；符号半条按裁定 F-08 补裁申换被测对象——被测对象由 ELF 换 PE 之后，msvc 的 release 产物把内部函数名放进独立 PDB，「镜像内不含符号 api_v1_system_echo」在 PE 本体上恒真，该写法撤下，改判「PE 二进制的只读数据节中不出现路由字面量 /api/v1/system/echo」，负样例（开启 ci-probe feature 构建后断言该字面量在 PE 中出现）建议配但不作为该半条成立的必要条件——补裁申给的处置是「改判 PDB，或改判路由字面量；两条都不成立时如实登记该半条降级、只留依赖树一半」，把负样例写成必要条件会多带一件裁定没要求的构建产物，且负样例做不出时反而被迫退回登记降级。阶段 1 计划里同一门禁项的两处复述须同批改，不得只改本处 | 阶段 1 的 ci-probe feature 门控 |
| RG-TOOLS-EXCLUDED | SBOM 中不含 ep-bench 与 ep-release-gate 两个包名 | 本阶段 |
| RG-PLAINTEXT-SECRETS-ABSENT | 永久适用且不得 N/A：按本节 exact ABI 证明生产 effective config 只有 `secrets.provider=kms`；产品 feature graph、PE 闭集与 SBOM 均无 `legacy-file`/`FileSecretProvider`；声明的 `secret://` 只对应严格 `EPS1`，无 `EPC1`、明文、未知/旧信封、legacy、quarantine 或 staging；builtin 每 recipient 独立 DPAPI bootstrap 与 HSM no-fallback 实机探针成立；当前 fresh/migration 路径有匹配 build/deployment/inventory 的签名 receipt/terminal evidence；六个具名负例退出码均非零。任一来源缺失、空跑、自报布尔值或残留即失败 | Stage 2 的 ADR-0007 终态与 `ep-secretctl`；Stage 14a0 冻结 gate ABI，Stage 14b 在待发布产品及目标 Windows Server 2022 上取证、签名并判定 |
| RG-RLS-MATRIX-GREEN | tests/rls_matrix 的 32 组矩阵全部通过，另含两个复制角色的五个入口借用测试 | 阶段 4 |
| RG-UNWIRED-ABSENT | 发布制品源码树中 apps/core-server/src/wiring/ 与 apps/job-worker/src/wiring/ 两个目录下的全部文件中不出现 Noop、Stub、Fake、Dummy 四类前缀的实现类型或注入行，且无返回固定业务分支的占位类型 | 阶段 1 的 archcheck 规则 unwired-absent |
| RG-NO-UNDECIDABLE | 发布制品源码树上执行 `cargo xtask archcheck` 退出码为 0，且基线第 12.1 节 undecidable 段为空 | 阶段 1 的 archcheck 三态输出 |
| RG-OFFSITE-COPY-PROTECTED | offsite_sinks 当前行 append_only_attested=true 且负向探针为 PASS；三身份/凭据两两互斥；对象存储或 Windows/SMB/NFS 对应策略证据与覆盖、删除、重命名、改权/策略负例齐全；任一 `OFFSITE_COPY_PROTECTION_MISSING` 活动窗口即失败 | 本阶段 offsite-sink-requirements、部署记录与后端实测证据 |
| RG-EXTERNAL-CLAIMS-SIGNED | 交付、认证、验收材料与客户合同的全部对外表述均进入逐条清单并由产品负责人对清单和材料摘要签字：首版只有第一档可发布，且只含已认证事实并在同条列全前提；第二档固定为零，即使已有比较报告也不得放行；第三档固定为零，既不得以「碾压」「行业模板」「实施顾问」「生态伙伴」及同类词作承诺性表述，也不得出现任何未经实测的比较级。NONE 部署必须逐字含“平台未提供病毒防护”；CUSTOMER_ICAP 只能陈述已实测的客户扫描器接入事实和客户责任边界，不得写成平台内置。缺行、缺前提、缺签字、材料摘要不符或签字后材料变更任一发生即失败；未来若要开放比较级只能另立产品版本、规格裁定与证据门，不是本版条件分支 | 本阶段的对外表述清单、材料摘要清单与产品负责人签字页 |
| RG-LICENSE-MODULE-LIFECYCLE-GREEN | 永远适用且不得 N/A：F-56 第 8 节自动测试、真实 PostgreSQL、签名部署清单/首装 evidence、LICENSE_GRANT/MODULE_PACKAGE 内外签名与单项配置包审批/发布全链、current 零或一且禁止多 current、完整 history、四态与可信时间、续期/撤销、模块五条合法动作、停用保留数据全部通过；trust rotation exact-set 直接枚举全部 `RELEASED` special `config_package_items`（全部 grant、revocation、曾发布 module-package item），逐项要求 `accepted_trust_bundle_sha256` 为接受时 32-byte bundle 摘要、发布后不可变，普通或未发布 item 必须为空，grant 行 `trust_bundle_sha256` 与 source item 相等，再分别与 current grant/current revocation 和 15 行 current module projection 交叉；历史 CRL `REVOKED` 证据保留但隔离且不作 purchased/rollback/正向证明、不阻断另一个合法 ACTIVE|RETIRED existing current，其他历史 digest/signature/source/chain 异常仍失败。current grant/revocation 的信任失败才把 deployment-level `LicenseStatus` 置为 `Restricted/SignatureInvalid`；current module 的信任失败只关闭该 module 的 effective runtime 并在 ServerAdmin 显示 `package_trust_status=SIGNER_REVOKED|INVALID`，绝不反向改写 deployment `LicenseStatus`。current grant signer revoked 只接受 inner/outer 均 ACTIVE 的新 grant 逐字 supersedes current 恢复；current module signer revoked 只接受一个新 `DISABLE` special item 原样携带旧 inner artifact，由 ACTIVE signer 只签新 outer，旧 current 的 inner/source outer 可一层或两层被 CRL `REVOKED`，每个未撤销层只可 ACTIVE|RETIRED 且全部 accepted/source/payload/digest/signature 自洽，仅可作为停用目标、不得作正向许可证明；停用完成后只允许 inner/outer 均由 ACTIVE signer 签发、semver 严格更高且全守卫通过的全新 `UPGRADE` 替换。special outer+inner 共同锚定同一 `license-roots.p7b`，普通包 outer 仍独立使用部署 KMS；ServerAdmin 15 行 `package_trust_status` wire 闭集恰为 `NOT_INSTALLED|TRUSTED|SIGNER_REVOKED|INVALID`，安装/启用态不得冒充信任态；`LicenseAdmissionGate` 的 HTTP/MCP 与 core/worker 非 HTTP registry 对实际入口 exact-set 零缺/多/重复，shared guard 顺序/错误及 F-56 所列绕过负例全绿；全部形成同 run/build/deployment 的 Stage 14 签名证据。真实 PASS 是 AI/MCP applicability 的共同前置 | F-56；Stage 3b/13b 实现与候选证据，Stage 14b 真实复验、签名和最终判定 |
| RG-AI-CONTAINMENT-GREEN | 九条 `tests/ai_containment` 全绿且名字/数量精确匹配 | 阶段 13c 本地 AI 计划实现与夹具；Stage 14b 复验并归档 |
| RG-AI-RESOURCE-CERTIFIED | F-55 第 3.7 节联合负载、算定值、模型/硬件摘要与既有通过线全部有签名报告 | Stage 14b 在目标 Windows Server 2022 机器当场认证 |
| RG-MCP-CONFORMANCE-GREEN | pin 版本六方法正例与全部禁用方法负例全绿 | 阶段 13c MCP 计划实现与夹具；Stage 14b 复验并归档 |
| RG-MCP-CONTAINMENT-GREEN | grant、manifest、gateway、plugin-host、高风险禁区、凭据与 egress 收容全绿 | 阶段 13c MCP 计划实现与夹具；Stage 14b 复验并归档 |
| RG-SERVER-ADMIN-MATRIX-90-GREEN | 90 格逐格、hash、ClientKind/audit/metrics、无新进程端口全绿 | 阶段 13c ServerAdmin 计划与 Stage 14b 发布制品复验 |
| RG-DEPLOYMENT-CARRIER-EVIDENCE-GREEN | 所选 carrier 的 provider/region/vTPM/故障域与完整 Stage 14 证据齐全 | [ServerAdmin/carrier Task 6](../2026-08-22-server-admin-cloud-carrier-implementation.md#task-6-prove-carrier-equivalence-and-close-stage-14-evidence) 实现；Stage 14b 对所选 carrier 实机取证与签名 |

发布门禁先消费唯一签名部署清单，不能把配置、命令行或数据库自报值当作部署身份。发布目录中的产品侧信任根唯一为 `target/release-package/trust/deployment-roots.p7b`，安装后唯一 readback 路径为 `C:\ProgramData\EnterprisePlatform\trust\deployment-roots.p7b`；它必须逐字命中同一待发布 `MANIFEST.sha256` 与 Authenticode 安装 CAB。客户部署清单自身是部署后独立生成的 customer-specific 制品，固定安装路径恰为 `C:\ProgramData\EnterprisePlatform\deployment\deployment.manifest.v1.jcs` 与相邻 `deployment.manifest.v1.p7s`，**不得**进入产品 `MANIFEST.sha256`，否则形成 product-build digest 环。清单与签名都用 safe handle 读回并拒绝 UNC/device/reparse/ADS/hardlink/8.3 alias、case/path drift；其 strict ABI 如下。

```rust
pub struct CustomerSecurityAdminCertificateV1 {
    pub certificate_sha256: Sha256Digest,
    pub signer_subject: String, // exact spki-sha256:<64-lowerhex>
    pub subject_key_identifier_b64url: String, // canonical base64url-no-pad, decoded 1..=64 bytes
}

pub enum DeploymentManifestArtifactCodeV1 { // wire/顺序 exact
    ProductManifest,
    ProductManifestSignature,
    ProductSbom,
    CoreServer,
    EpMigrate,
    ProductModulesManifest,
    LicenseTrustBundle,
    DeploymentTrustBundle,
}

pub struct DeploymentManifestArtifactV1 {
    pub artifact_code: DeploymentManifestArtifactCodeV1,
    pub sha256: Sha256Digest,
}

pub struct LicenseTrustedSignerSubjectRegistryV1 {
    pub schema_version: u16, // exact JSON number 1
    pub purpose: String, // exact "EP-DEPLOYMENT-LICENSE-TRUSTED-SIGNER-SUBJECT-REGISTRY-V1"
    pub subjects: Vec<String>, // exact 1..=64，UTF-8 bytes 严格升序、唯一
}

pub struct DeploymentManifestV1 {
    pub schema_version: u16, // exact 1
    pub purpose: String, // exact "EP-DEPLOYMENT-MANIFEST-V1"
    pub manifest_id: Uuid,
    pub deployment_id: Uuid,
    pub deployment_record_revision: u64,
    pub product_version: SemVerV1,
    pub product_build_sha256: Sha256Digest,
    pub employee_api_origin: String,
    pub license_trust_bundle_sha256: Sha256Digest,
    pub license_trusted_signer_subjects: Vec<String>, // exact 1..=64 spki-sha256:<64-lowerhex>
    pub deployment_trust_bundle_sha256: Sha256Digest,
    pub x509_login_trust_anchor_ref: SecretRef,
    pub x509_login_trust_bundle_sha256: Sha256Digest,
    pub manifest_signer_subject: String, // exact spki-sha256:<64-lowerhex>
    pub customer_security_admin_certificates: Vec<CustomerSecurityAdminCertificateV1>, // exact 2..=16
    pub artifacts: Vec<DeploymentManifestArtifactV1>, // exact 8，enum 顺序
    pub issued_at: DateTime<Utc>,
}

pub struct DeploymentManifestEvidenceV1 {
    pub schema_version: u16, // exact 1
    pub stage14_run_id: Uuid,
    pub deployment_id: Uuid,
    pub deployment_record_revision: u64,
    pub product_build_sha256: Sha256Digest,
    pub manifest_id: Uuid,
    pub deployment_manifest_sha256: Sha256Digest,
    pub deployment_manifest_signature_sha256: Sha256Digest,
    pub license_trust_bundle_sha256: Sha256Digest,
    pub license_trusted_signer_subject_registry_sha256: Sha256Digest,
    pub deployment_trust_bundle_sha256: Sha256Digest,
    pub x509_login_trust_bundle_sha256: Sha256Digest,
    pub x509_login_ep_migrate_readback_sha256: Sha256Digest,
    pub x509_login_ep_core_readback_sha256: Sha256Digest,
    pub customer_security_admin_certificate_registry_sha256: Sha256Digest,
    pub installed_manifest_sd_sha256: Sha256Digest,
    pub installed_trust_bundle_sd_sha256: Sha256Digest,
    pub verification_transcript_sha256: Sha256Digest,
    pub observed_at: DateTime<Utc>,
}
```

`DeploymentManifestEvidenceV1.verification_transcript_sha256` 不是 stdout 或自由 JSON 的 direct hash。唯一前像使用 domain/purpose `EP-DEPLOYMENT-MANIFEST-VERIFICATION-TRANSCRIPT-V1` 与 `projection_digest`，root exact 为 `{schema_version,purpose,stage14_run_id,deployment_id,deployment_record_revision,product_build_sha256,manifest_id,deployment_manifest_sha256,deployment_manifest_signature_sha256,license_trust_bundle_sha256,license_trusted_signer_subject_registry_sha256,deployment_trust_bundle_sha256,x509_login_trust_bundle_sha256,x509_login_ep_migrate_readback_sha256,x509_login_ep_core_readback_sha256,customer_security_admin_certificate_registry_sha256,installed_manifest_sd_sha256,installed_trust_bundle_sd_sha256,checks,observed_at}`。除 `schema_version/purpose/checks` 外逐字段等于承载它的 evidence；`checks` 按此 enum 顺序恰九项且不得重复：`MANIFEST_JCS_CANONICAL|MANIFEST_CMS_EXACT|DEPLOYMENT_CHAIN_ACTIVE|DEPLOYMENT_CRL_GLOBAL_HIGHEST_COVERING|LICENSE_ROSTER_HISTORY_CONTAINED|X509_BUNDLE_RECIPIENTS_EQUAL|CUSTOMER_ADMIN_ROSTER_ACTIVE|INSTALLED_DACL_EXACT|ARTIFACT_DIGESTS_EXACT`。entry 不另带人工 outcome；只有 collector 已真实完成对应检查才可列入，因此 exact 九项本身就是闭集通过结论，任一检查失败直接不产 evidence。DTO、重算 digest 与 evidence 字段三者必须相等；unknown/missing/extra/reordered check 或把命令输出 hash 填入该字段均失败。

`DeploymentManifestV1` 是≤262144-byte、无 BOM UTF-8 RFC 8785 strict-JCS；所有字段与字段集 exact，unknown/duplicate/noncanonical 一律拒绝。`employee_api_origin` 只接受 DNS host 的 canonical HTTPS origin：无 userinfo/path/query/fragment，默认 443 必须省略，拒绝 IP literal、localhost、回环、重定向目标和直连 core-server:8080。`deployment_record_revision` 必须等于 `DeploymentRecordRepository` 选出的唯一 current revision；`product_version` 必须等于签名 product/modules projection；`product_build_sha256` 恰为 exact `MANIFEST.sha256` bytes digest。`artifacts` 按上列 enum 顺序恰八项，依次绑定 `MANIFEST.sha256`、`MANIFEST.sha256.sig`、`sbom.cdx.json`、`bin/core-server.exe`、`bin/ep-migrate.exe`、`product-modules.v1.jcs`、`trust/license-roots.p7b`、`trust/deployment-roots.p7b` exact bytes；前述 product/license/deployment 三个顶层 digest 必须分别等于对应 roster 项，禁止额外 artifact、第二目录或自由路径。

`license_trusted_signer_subjects` 是 F-56 inner 与 special outer 发行签名人的唯一授权输入：恰 1..=64 项，每项逐字匹配 `spki-sha256:[0-9a-f]{64}`，按 UTF-8 bytes 严格升序且唯一；每份 current/history/accepted special 的 inner 与 source outer signer token 都必须各自唯一命中该已验签、同 deployment/build 的 signed roster，不能仅凭 `license-roots.p7b` 成链而绕过授权。`license_trusted_signer_subject_registry_sha256` 唯一按 domain/purpose `EP-DEPLOYMENT-LICENSE-TRUSTED-SIGNER-SUBJECT-REGISTRY-V1` 对 exact DTO `{schema_version:1,purpose,subjects}` 调用本节 `projection_digest`；manifest、DeploymentManifestEvidence、InitialGovernanceEvidence 与 F56LicenseTrustRotationEvidence 四处 digest 必须相等。effective `release.trusted_signer_subjects` 只是一项本地漂移断言：`[]` 精确表示不覆盖并使用 signed roster，非空时必须先满足相同 canonical/sort/unique 规则，再与 signed roster 按项、顺序完全相等；不等时 readiness、special 运维与发布 gate 全部失败，绝不能增删或替换签名人。

signed roster 是可识别 signer 身份清单，不是“目前 ACTIVE signer”清单。安装任何新 deployment manifest 前，collector 必须从数据库全部永久 `RELEASED LICENSE_GRANT|MODULE_PACKAGE` special history 重建 inner 与 source outer 的 referenced-token exact-set，并要求该集合是新 manifest roster 的子集；trust chunks、released-special registry、roster projection 与数据库四方证明这一 containment。删除任何历史已引用 token、即使当前投影已不再使用它，也必须在安装/轮换前失败；该 token 只能随可信整库回退恢复，或在全新 deployment 中摆脱旧历史，不能在原 deployment 静默移除。保留旧 token 仅保留历史身份可识别性，不授予新发行权：CRL 的 REVOKED 判定优先，new artifact 仍须 roster membership 且 inner/outer whole chain 都为 ACTIVE。CAB 轮换必须同一离线批次同时签新 manifest roster 与 bundle，并先证明上述历史 referenced-set containment。

CAB 信任轮换必须在同一离线发布批次原子提供新的 `deployment.manifest.v1.jcs/.p7s`（含新 roster）与其 `license_trust_bundle_sha256` 所指 exact `license-roots.p7b`；两者的 deployment/build/revision/batch 必须同一，安装/readback 完成后才可重开 readiness/gate。若本地配置非空仍须与新 signed roster exact-equal。只换 bundle、只换 roster、跨批拼接、旧 manifest+新 bundle、新 manifest+旧 bundle或任一 signer 不在 roster 都必须具名非零失败；轮换不得回填历史 accepted digest，而由 trust evidence 同时绑定首次接受 bundle、新 validation bundle 与该 roster digest。

`customer_security_admin_certificates` 按 `signer_subject` UTF-8 bytes 升序，`certificate_sha256`、`signer_subject`、解码后的 SKI 三列分别唯一；每项三字段必须同时命中同一张 leaf：第一列是 exact DER certificate SHA-256 lowerhex，第二列是 exact DER SPKI SHA-256 token，第三列是 leaf SKI raw bytes 的 canonical base64url-no-pad。成员资格即该 exact leaf 同时按 `deployment-roots.p7b` 与登录 bundle 的唯一整链/最高 covering CRL 算法在本次验证时为 ACTIVE；不能只凭同 SPKI 的另一张证书、DN、serial、显示名或同一项里的任意两列判定。`x509_login_trust_anchor_ref` 必须逐字等于 effective `EP__AUTH__X509__TRUST_ANCHOR_REF` 的 canonical `SecretRef`；`ep-migrate` 以固定 `ep-migrate` recipient、core 以固定 `ep-core` recipient 经既有只读 resolver 解出的 exact bytes 都必须等于 `x509_login_trust_bundle_sha256`，两 recipient 的 readback digest 还必须彼此相等。该登录 bundle 同样是≤1048576-byte DER empty-content/zero-signer CMS CA+完整 base-CRL bag，不含 leaf、私钥、URL、脚本或可执行正文。

`deployment.manifest.v1.p7s` 是≤1048576-byte 的**单个完整 DER `ContentInfo`**，`contentType=signedData`、`content=[0] EXPLICIT SignedData` 且输入末尾零 trailing byte；`SignedData.version=3`，detached content 恰为 manifest exact JCS bytes，`encapContentInfo.eContentType=id-data` 且 `eContent` 缺省，`digestAlgorithms` 恰为只含一个 SHA-256 `AlgorithmIdentifier`（OID exact、parameters 缺省）的 DER SET。SignerInfo 恰一个、`version=3`、`sid` 逐字等于 leaf SKI，`digestAlgorithm` 同为 parameters 缺省的 SHA-256，unsigned attributes 缺省；signed attributes 必须存在，wire 是 `[0] IMPLICIT`，内容恰为按 DER 排序的 `contentType=id-data,messageDigest,signingTime` 三个 Attribute，每个 attrValues SET 恰一值，零重复/未知。实际签名 preimage 必须把该隐式字段的 content octets 重新包成 canonical DER universal `SET OF`（tag `0x31` + DER length + 原 content octets），不得直接签 `[0]` wire tag、不得改成库私有 SEQUENCE。`messageDigest` 等于 manifest digest；`signingTime` 与 `issued_at` 必须语义上是同一 UTC whole-second instant，1950..2049 只用 DER `UTCTime`、其余只用 `GeneralizedTime`，均为 Z-only、含秒、无小数/offset。CMS certificates 恰含 signer leaf 与形成唯一链所需的零至多个非自签 intermediate，按 DER SET 规范排序，不含 anchor、CRL、重复或无关证书；leaf 必须 DigitalSignature+CodeSigning，SPKI token 等于 `manifest_signer_subject`。SignerInfo、证书与 CRL 的签名算法及参数闭集只允许 ECDSA P-256/SHA-256（parameters absent）或 RSA-PSS/SHA-256（RSA modulus≥3072、hash=SHA256、MGF1-SHA256、saltLength=32、trailerField=1）；SHA-1、PKCS#1 v1.5、NULL/默认/隐式参数或其他组合全拒绝。

`deployment-roots.p7b` 是≤1048576-byte 的单个完整 DER `ContentInfo`：`contentType=signedData`、`content=[0] EXPLICIT SignedData`、输入末尾零 trailing byte；`SignedData.version=1`，`digestAlgorithms` 与 `signerInfos` 都是空 DER SET，`encapContentInfo.eContentType=id-data` 且 `eContent` 缺省，certificates 与 crls 分别按 DER SET 规范排序，零其他内容。这一唯一 degenerate SignedData 形状含 1..=64 CA 与 1..=256 完整 X.509 v2 base CRL，至少一张自签、自验、BasicConstraints CA=true 且 KeyUsage 同含 keyCertSign/cRLSign 的 anchor；其余只可为具同约束的 non-self-signed intermediate。证书 DER/SKI、CRL issuer+CRLNumber 唯一；不含 leaf、无关证书、delta/indirect/removeFromCRL。manifest CMS 必须形成恰一条从 leaf 经零或多个 intermediate 到当前 bundle anchor 的有效链；对每个实际 issuer 都先从全部结构/签名合法 base CRL 取全局最高 numeric CRLNumber、要求同号 DER 唯一，再要求该最高号覆盖 `DeploymentManifestEvidenceV1.observed_at`。只有所有 issuer 的 global-highest-then-cover 前置都成功后才扫描 serial；任一 issuer 缺失、最高号尚未生效/已过期/同号冲突都令整链 UNTRUSTED，禁止回退低号，也禁止先凭另一 issuer 的 serial hit 得出 REVOKED。此前置全绿后，链上任一 non-anchor serial 命中才拒绝。整条 non-anchor 链必须在 `issued_at` 均处于有效期，且在 evidence `observed_at` 仍全部 current-valid/无撤销，故本次 current manifest 状态必须为 ACTIVE；RETIRED/REVOKED/UNTRUSTED 均不能签发该 evidence。anchor 必须在 issued_at 有效，其 observed_at 过期本身不把链退休，但从当前 bundle 移除/替换或形成多链仍失败。零链、多链、约束/critical extension/CRL/算法任一失败都拒绝，绝不读取 Windows 任意根、联网补链或软失败。

安装目录 `C:\ProgramData\EnterprisePlatform\deployment\` 与 trust 文件 `C:\ProgramData\EnterprisePlatform\trust\deployment-roots.p7b` 的 owner 都必须为 `NT AUTHORITY\SYSTEM` 且 DACL 为 PROTECTED/关闭继承；显式 allow ACE exact 为 `NT AUTHORITY\SYSTEM`、`BUILTIN\Administrators`、`NT SERVICE\ep-ops` 各 FullControl，以及 `NT SERVICE\ep-core`、`NT SERVICE\ep-worker` 各 `FILE_GENERIC_READ|FILE_TRAVERSE|READ_CONTROL|SYNCHRONIZE`，其余包括 Users/Authenticated Users/Everyone/ep-ai/ep-integ/ep-plugin 均无 ACE，两个只读服务没有 write/delete/WRITE_DAC/WRITE_OWNER。deployment directory 的五条 ACE flags 恰为 `OBJECT_INHERIT_ACE|CONTAINER_INHERIT_ACE` 且无 INHERIT_ONLY/NO_PROPAGATE；JCS/p7s 只能继承这五条。trust file 自身是 PROTECTED explicit DACL，五条 ACE flags 均为 0。两份 SD digest 不是 Win32 opaque bytes 的随意 hash：`installed_manifest_sd_sha256` 是按 canonical path bytes 排序的 deployment directory、JCS、p7s 三对象 strict-JCS roster digest，`installed_trust_bundle_sd_sha256` 是 trust file 单对象 roster digest；每个对象 exact projection 为 `{canonical_path,owner_sid="S-1-5-18",dacl_protected,aces}`，directory/standalone trust file 的 `dacl_protected=true` 且 ACE origin=`EXPLICIT`，两个 manifest child file 的 `dacl_protected=false` 且 ACE origin=`INHERITED`。aces 按 SID bytes 排序且 item exact `{sid,access_profile,origin}`，profile 只取 `FULL_CONTROL|STAGE14_READ_ONLY`，分别逐字映射 `FILE_ALL_ACCESS` 与上述只读 mask。任何额外/deny/错 origin ACE、漏对象或 ACL 不等都失败。打包与 readback 的 `deployment-roots.p7b` exact bytes、digest 和安全描述符任一不等时失败。

Stage 14b 把安装后的 manifest 与 p7s exact-copy 到唯一证据根 `target/release-evidence/deployment/<lowercase-stage14-run-id>/`，顶层 exact-set 恰为 `deployment.manifest.v1.jcs`、`deployment.manifest.v1.p7s`、`deployment-manifest-evidence.v1.jcs`、`deployment-manifest-evidence.v1.jcs.sig.jcs`。evidence ref 唯一为 `ep-evidence://stage14/<same-run>/deployment/deployment-manifest-evidence/sha256/<digest>`；evidence 及三份 readback 绑定同一 run/deployment/current revision/product build/window，两个 x509 recipient 的 exact bytes 只在内存比较、不得复制进 evidence。sidecar 复用 `Stage14EvidenceSignatureV1`，purpose 唯一为 `DEPLOYMENT_MANIFEST_EVIDENCE_V1`。manifest 的独立 CMS 与该 evidence sidecar 两层都必须验证；任一层不能替代另一层。

fresh-production 首装的 source 根也不接受任意路径，唯一为 `C:\ProgramData\EnterprisePlatform\evidence\stage14\initial-governance\<lowercase-deployment-id>\`，其中 source exact-set 恰为 `bootstrap.jcs`、`license.epcfg`、`initial-governance.receipt.v1.jcs`；路径段必须逐字等于已验签 manifest、bootstrap、receipt、数据库与本次 evidence 的同一 `deployment_id`。目录 owner=`NT AUTHORITY\SYSTEM`、DACL PROTECTED；显式 allow ACE exact 为 SYSTEM/`BUILTIN\Administrators`/`NT SERVICE\ep-ops` 各 FullControl 与 `NT SERVICE\ep-core` 的 `FILE_GENERIC_READ|FILE_TRAVERSE|READ_CONTROL|SYNCHRONIZE`，其余包括 Users/Authenticated Users/Everyone/ep-worker/ep-ai/ep-integ/ep-plugin 无 ACE，ep-core 无 write/delete/WRITE_DAC/WRITE_OWNER。root directory 的四条 ACE flags 恰为 `OBJECT_INHERIT_ACE|CONTAINER_INHERIT_ACE` 且无 INHERIT_ONLY/NO_PROPAGATE，三个文件只能继承该四条。`source_root_sd_sha256` 按上一段同一 normalized SD projection 算法覆盖 root directory 加三文件的按 canonical path 排序 exact roster：root 为 `dacl_protected=true/EXPLICIT`，三 child 为 `false/INHERITED`；access profile 只取 `FULL_CONTROL|INITIAL_GOVERNANCE_READ_ONLY`，后者逐字映射本段 ep-core mask。任何文件不继承同一闭集都失败。三文件只以 fixed-root safe handle 打开并拒绝 UNC/device/reparse/ADS/hardlink/8.3/case/path drift；receipt 必须是≤1048576-byte、`CREATE_NEW`/flush/close/readback 的 F-56 exact unsigned strict-JCS，**不得存在或接受相邻 KMS/CMS/Stage14 sidecar**。

```rust
pub enum InitialGovernanceBootstrapRoleV1 { ConfigOperator, SecurityApprover } // wire exact CONFIG_OPERATOR|SECURITY_APPROVER
pub enum InitialGovernanceFinalKeyDomainStateV1 { Active } // wire exact ACTIVE

pub struct InitialGovernanceOperatorEvidenceV1 {
    pub bootstrap_role: InitialGovernanceBootstrapRoleV1,
    pub user_id: Uuid,
    pub device_id: Uuid,
    pub certificate_sha256: Sha256Digest,
    pub signer_subject: String,
    pub subject_key_identifier_b64url: String,
    pub x509_verifier: String, // exact cert-sha256:<64-lowerhex>
    pub x509_credential_handle_b64url: String, // exact leaf SKI raw bytes, canonical base64url-no-pad
    pub password_x509_sign_in_exit_code: i32, // exact 0
    pub complete_mfa_exit_code: i32, // exact 0
    pub authentication_audit_projection_sha256: Sha256Digest,
    pub authentication_transcript_sha256: Sha256Digest,
}

pub struct InitialGovernanceEvidenceV1 {
    pub schema_version: u16, // exact 1
    pub stage14_run_id: Uuid,
    pub deployment_id: Uuid,
    pub product_build_sha256: Sha256Digest,
    pub deployment_manifest_evidence_ref: OpaqueEvidenceRef,
    pub deployment_manifest_evidence_sha256: Sha256Digest,
    pub license_trusted_signer_subject_registry_sha256: Sha256Digest,
    pub customer_security_admin_certificate_registry_sha256: Sha256Digest,
    pub source_root_sd_sha256: Sha256Digest,
    pub bootstrap_id: Uuid,
    pub bootstrap_jcs_sha256: Sha256Digest,
    pub bootstrap_body_sha256: Sha256Digest,
    pub bootstrap_authorization_registry_sha256: Sha256Digest,
    pub initial_license_archive_sha256: Sha256Digest,
    pub initial_governance_receipt_sha256: Sha256Digest,
    pub initial_governance_audit_event_id: Uuid,
    pub initial_governance_audit_payload_sha256: Sha256Digest,
    pub initial_governance_audit_chain_hash_sha256: Sha256Digest,
    pub database_bootstrap_projection_sha256: Sha256Digest,
    pub schema_manifest_sha256: Sha256Digest,
    pub ep_migrate_pe_sha256: Sha256Digest,
    pub first_released_grant_id: Uuid,
    pub first_released_grant_source_config_package_id: Uuid,
    pub first_released_grant_source_config_item_id: Uuid,
    pub first_released_grant_projection_sha256: Sha256Digest,
    pub governance_legal_entity_id: Uuid,
    pub key_domain_id: Uuid,
    pub final_key_domain_state: InitialGovernanceFinalKeyDomainStateV1,
    pub key_domain_projection_sha256: Sha256Digest,
    pub key_domain_data_key_matrix_projection_sha256: Sha256Digest,
    pub activation_audit_event_id: Uuid,
    pub activation_audit_payload_sha256: Sha256Digest,
    pub activation_audit_chain_hash_sha256: Sha256Digest,
    pub operators: Vec<InitialGovernanceOperatorEvidenceV1>, // exact 2，enum 顺序
    pub observed_at: DateTime<Utc>,
}
```

上列 initial-governance 的六类 semantic digest 全部使用本节统一 `projection_digest(domain,dto)`，不是对数据库查询输出或大文件切片直接求 hash。六个 strict DTO 的 `schema_version` 固定为 JSON number `1`，`purpose` 必须逐字等于下列 domain；unknown/duplicate/missing key、string `"1"`、非 canonical UUID/digest/time/date、未按指定键排序或重复均拒绝。所有 Option key 必须存在且无值为 JSON null；raw bootstrap/receipt/CMS 与 audit `before/after` exact JCS bytes 仍取 direct-byte SHA-256，两个具名 transcript 字段则只按各自下文冻结的 domain DTO 调用 `projection_digest`。唯一闭集如下：

1. `customer_security_admin_certificate_registry_sha256` 使用 `EP-CUSTOMER-SECURITY-ADMIN-CERTIFICATE-REGISTRY-V1`，root exact `{schema_version,purpose,entries}`，entry exact `{certificate_sha256,signer_subject,subject_key_identifier_b64url}`；entries 恰为已验签 deployment manifest 的 `customer_security_admin_certificates` 2..=16 项，按 `signer_subject` UTF-8 bytes 排序，certificate/SPKI/SKI 三列分别唯一并逐值相等。`DeploymentManifestEvidenceV1` 与 `InitialGovernanceEvidenceV1` 的同名字段必须命中同一 root digest。
2. `bootstrap_authorization_registry_sha256` 使用 `EP-INITIAL-GOVERNANCE-AUTHORIZATION-REGISTRY-V1`，root exact `{schema_version,purpose,entries}`；entry exact `{bootstrap_role,user_id,device_id,signer_subject,certificate_sha256,subject_key_identifier_b64url,signature_cms_sha256}`，恰两项并按 `CONFIG_OPERATOR,SECURITY_APPROVER` enum 顺序。certificate 与 signature digest 分别是 leaf exact DER 与 decoded CMS exact DER 的 direct SHA-256；每项必须与 bootstrap body operator、authorization CMS 及 deployment certificate registry 的同一 distinct entry 逐值相等。
3. `database_bootstrap_projection_sha256` 使用 `EP-INITIAL-GOVERNANCE-DATABASE-PROJECTION-V1`，唯一前像是 F-56 `platform.bootstrap.initial_governance.v1` audit.after 内嵌的 `database_bootstrap_projection` exact object，root keys 恰为 `{schema_version,purpose,legal_entity,key_domain,operators,legal_entity_grants,roles,role_permission_pairs,user_role_grants,approval_chains}`。每个 child 的字段、literal、排序与数量逐字复用 F-56 initial-governance 冻结：一 active 法人；一同法人 PROVISIONING `LEGAL_ENTITY` key domain；两名按 bootstrap role 排序且仅含非秘密 credential metadata/Argon2 policy 的 operator；三条按 user UUID 排序的法人授权；两角色；八加二共十条 permission-action pair；两条 user-role grant；以及 `approval_chains`（字段名必须为复数，禁止 singular alias）按 Stage 4 `ApprovalScenarioCode::ALL` enum 顺序 exact 37 项及各自唯一 node。password、salt、PHC/verifier 或其 digest 禁止进入前像。audit.after 的 `database_bootstrap_projection_sha256`、evidence 字段与按 domain 重算值三者必须相等；`initial_governance_audit_payload_sha256` 是完整 audit.after exact JCS 的 direct SHA-256，`initial_governance_audit_chain_hash_sha256` 则按 append-only chain 重算，三者不得互换。
4. `key_domain_projection_sha256` 使用 `EP-INITIAL-GOVERNANCE-KEY-DOMAIN-PROJECTION-V1`，root exact `{schema_version,purpose,id,legal_entity_id,domain_kind,state,kek_ref,kek_version,provisioned_at,destroy_planned_at,destroyed_at,destroy_evidence_ref,security_level,data_scope_tags,row_version,created_at,created_by,updated_at,updated_by}`；`data_scope_tags` 按 UTF-8 bytes 排序去重。该 current row 必须为同一 signed id/法人、`LEGAL_ENTITY/ACTIVE`、canonical deployment locator、`kek_version=1`，ACTIVE 时间/空值形状与阶段 2 完全一致，并从 initial audit 内嵌的 PROVISIONING child 沿唯一 activation event 推导，不能另选第二域。本 DTO 的 `provisioned_at` 逐字等于 whole-second activation typed audit；但 PostgreSQL 公共列 `created_at/updated_at` 是统一时间通则的具名例外，wire 固定为 UTC RFC3339、恰六位小数 `YYYY-MM-DDTHH:MM:SS.ffffffZ`，即使微秒为零也不得省略或改成 whole-second；`row_version` 为 JSON number。
5. `key_domain_data_key_matrix_projection_sha256` 是**不可变首次 activation snapshot**，不是当前 ACTIVE 集合。它使用 `EP-INITIAL-GOVERNANCE-DATA-KEY-MATRIX-V1`，root exact `{schema_version,purpose,key_domain_id,legal_entity_id,activation_event_id,entries}`；entry 只含 `{data_key_id,purpose,security_level_scope,version,algorithm,wrapped_key_sha256,wrap_kek_version,activated_at}`。entries 从唯一 `action='platform.key_domain.activated.v1'` exact payload 重建，恰 16 行，按 `FIELD|BLIND_INDEX|ATTACHMENT|ARCHIVE` 后 10|20|30|40 排序，全部 `version=1,wrap_kek_version=1`、algorithm 映射正确；`wrapped_key_sha256` 是首次 activation 时非空 DB `wrapped_key` exact bytes 的 direct SHA-256。数据库同 id 行的 purpose/scope/version/algorithm/wrapped digest/wrap version/activated-at 不可变字段必须仍逐字相等；后续合法 rotation 使 version-1 行进入 RETIRING/RETIRED/DESTROYED、增加更高版本或推进 row_version/状态时间是允许的，collector 另按阶段 2 状态机验证，但这些当前 state/row_version/后继时间绝不进入或改写初始 snapshot digest。
6. 每名 operator 的字段名仍为 `authentication_audit_projection_sha256`，但其唯一可实现前像不依赖 Stage 4 未冻结的 audit action 字符串；它使用 `EP-INITIAL-GOVERNANCE-AUTHENTICATION-AUDIT-PROJECTION-V1` 对本次真实认证事务的既有三张表投影。root exact `{schema_version,purpose,bootstrap_role,user_id,device_id,login_attempts,mfa_challenge,session}`。collector 在测试前以 repeatable-read 记录该 user 的 `login_attempts.id` exact-set，随后为两个 operator 逐人串行完成 sign-in→MFA，再取 after exact-set；每人的集合差必须恰两行，0/1/>2 都使本 run 失败，禁止按 user/time 猜选。`login_attempts` 按 `(occurred_at,id)` 排序，entry exact `{id,user_id,login_name_hash,outcome,client,source_addr_sha256,occurred_at,created_by}`；两项 outcome 依次为 `MFA_REQUIRED|SUCCESS`，`login_name_hash` nested exact 为 `{length,sha256}`（length 是 DB bytes checked u32，sha256 是 exact bytes direct digest），`source_addr_sha256:Option<Sha256Digest>` key 永远存在，DB NULL 为 JSON null，非空为 `SHA-256(UTF-8(exact stored source_addr))`。`mfa_challenge` exact `{id,challenge_kind,user_id,session_id,user_device_row_id,default_legal_entity_id,operation_type,subject_digest,subject_summary_sha256,nonce_sha256,credential_kind_used,status,token_hash,issued_at,expires_at,verified_at,consumed_at,failure_count,row_version,created_at,created_by,updated_at,updated_by}`，固定 `SIGN_IN_MFA`、`session_id=null`、目标 user/device/法人、`operation_type=null`、`credential_kind_used=X509_CERT`、`status=CONSUMED`、`failure_count=0`；两个显式 `*_sha256` 分别对 DB subject_summary 的 RFC8785 JCS 与 nonce exact bytes 直接求 SHA-256，`subject_digest/token_hash` 两个既存 32-byte digest 按 lowerhex 输出。collector 只在内存中对 sign-in 返回的 `mfa_challenge` token 重算 SHA-256 并据此唯一定位该 row，随后销毁 token。`session` exact `{id,user_id,user_device_row_id,token_hash,active_legal_entity_id,client,issued_at,expires_at,idle_expires_at,last_seen_at,revoked_at,revoke_reason,is_breakglass,row_version,created_at,created_by,updated_at,updated_by}`，固定同 user/device/法人/client、未撤销、`is_breakglass=false`，其中 `token_hash` 也必须是既存 32-byte digest 的 64-lowerhex wire；complete-MFA 返回的 session token 同样只在内存重散列并唯一定位该 row后销毁，challenge 的 CONSUMED 与 session/login SUCCESS 必须同事务闭合。本三表投影内全部 PostgreSQL `timestamptz` 字段（包括 nullable timestamp 与公共 `created_at/updated_at`）是 whole-second 通则的具名例外：非空值统一输出 UTC RFC3339 恰六位小数 `YYYY-MM-DDTHH:MM:SS.ffffffZ`，即使微秒为零也不得省略；nullable 字段的 key 永远存在，DB NULL 只能是 JSON null。password/X509 原文、PHC、会话令牌、challenge nonce 原文与 source address 原文都不进入前像或落盘 transcript。三投影与 `password_x509_sign_in_exit_code=0/complete_mfa_exit_code=0` 必须逐值交叉；少/多 attempt、错 outcome、跨 user/device/entity、challenge/session token 非唯一、challenge 未消费、错误 credential kind、session 预存/撤销、timestamp 精度/wire 或摘要漂移均失败。

每名 operator 的 `authentication_transcript_sha256` 也不是 stdout 的自由 hash；唯一前像为 domain/purpose `EP-INITIAL-GOVERNANCE-AUTHENTICATION-TRANSCRIPT-V1` 的 exact DTO `{schema_version,purpose,bootstrap_role,user_id,device_id,password_x509_sign_in_exit_code,complete_mfa_exit_code,authentication_audit_projection_sha256}`，按 `projection_digest` 计算。两个 exit code 都必须是 JSON number `0`，最后一项等于上段三表 projection digest；DTO 不含 password、PHC、CMS、challenge/session token、source address或响应正文。operator evidence、该 DTO 与重算 digest 三者必须相等。

`first_released_grant_projection_sha256` 的唯一前像另行固定为 domain/purpose `EP-F56-FIRST-RELEASED-GRANT-PROJECTION-V1` 的 strict root `{schema_version,purpose,grant_id,payload,payload_sha256,inner_signature_cms_sha256,inner_signer_subject,accepted_trust_bundle_sha256,source_config_package_id,source_config_item_id}`。`payload` 是 F-56 `LicenseGrantPayloadV1` 全字段 exact object，必须为本 deployment 最早一张 RELEASED grant 且 `supersedes_grant_id=null`；payload/raw CMS digest、signer、accepted bundle、source package/item 与 DB row、对应 RELEASED special source、`action='platform.config_special.accepted.v1'` audit 及 trust entry 逐值相等。只 hash grant id、复用 current-grant projection、漏 source/accepted digest 或把后续 renewal 当 first 都失败。

上述六类 negative fixture 至少逐类变异一个 unknown/missing/duplicate key、排序、显式 null 与 JSON number wire，并分别证明摘要不可重算时 collector 非零；还必须证明 `approval_chains` 为 36/38 项、singular `approval_chain`、把 PHC/password verifier 纳入前像、operator 的 login attempt/challenge/session 缺重跨人或事务不闭合、matrix 非 16 行或 initial-governance/activation audit chain 漂移都不能形成 lifecycle/common PASS。

`InitialGovernanceEvidenceV1.license_trusted_signer_subject_registry_sha256` 必须逐字等于同 child 已验签 `DeploymentManifestV1` roster projection，并与首张 RELEASED grant 的 accepted inner/source outer 两个 signer token 分别做唯一成员核对；bootstrap archive、first-grant source、trust entry 与 manifest 不能跨 deployment/build/roster 拼接。roster 缺/空/乱序/重复、digest 漂移、任一 signer 不在 roster、只换 trust bundle 未同批签新 manifest或本地非空 assertion 不等，都必须在首装写入或 Stage 14 evidence 前非零失败。

initial-governance collector 必须 strict-parse F-56 exact receipt，并逐字核对 receipt、`platform.bootstrap.initial_governance.v1` 审计 payload/hash-chain、数据库 bootstrap projection、原始 bootstrap body/two-CMS authorization registry、原始 `license.epcfg` archive digest、最终最早一张 RELEASED grant 的 id/governance/source/projection，以及同一个 signed key domain 的唯一 `PROVISIONING→ACTIVE` 终结。密钥终结只接受阶段 2 冻结的 `action='platform.key_domain.activated.v1'` closed payload：`{schema_version:1,deployment_id,key_domain_id,legal_entity_id,activation_source,bootstrap_id,kek_ref,kek_version,kek_provider_fingerprint_sha256,data_keys,activated_at}`；这里必须为 `activation_source=INITIAL_GOVERNANCE` 且 `bootstrap_id` 等于 signed bootstrap，`data_keys` 恰为 `FIELD|BLIND_INDEX|ATTACHMENT|ARCHIVE × 10|20|30|40` 的固定顺序 16 行，每行 exact `{data_key_id,purpose,security_level_scope,version,algorithm,wrap_kek_version,wrapped_key_sha256}`，初始 `version=1`、算法、wrapped digest 与同事务数据库 projection/KMS readback逐字相等。所有 `data_keys.version` 与跨 crate/IPC/JSON 的 `DataKeyRef.version` 一律为 `u16`、有效域 `1..=65535`；EPC1 header 的 2-byte unsigned data-key version、附件/TOTP 持久引用、该 DTO 与 `data_keys` 行必须四者数值相等，禁止 `u32/i32` 截断、0、65536、前导零或 sentinel；当前 version=65535 时下一次 data-key rotation 必须以稳定边界错误拒绝且零写，禁止 checked-cast 遗漏、回绕到 0/1 或复用旧版本。`kek_version/wrap_kek_version` 则统一为 Rust `u32`、SQL `integer` 的共同可持久域 `1..=2147483647`，任何 0、负值、2147483648、溢出或 lossy cast 均拒绝；不得误套 data-key 的 u16 上限。collector 必须逐列核对 Stage 2 冻结的完整 activation envelope，并重算唯一 activation audit 的 payload digest与既有 append-only audit hash chain，要求 `activation_audit_chain_hash_sha256` 命中链上该 event，并以 `key_domain_data_key_matrix_projection_sha256` 绑定同一域的 exact 16-row DB projection；缺/多/重复、另一 transition、另一 event 或人工重建均失败。`kek_ref` 中的 deployment UUID 不能由普通 DB CHECK 冒充已证明，必须逐字等于已验签 `DeploymentManifestV1.deployment_id`，且 canonical locator 全值为 `kms://ep/v1/deploy/<lowercase-deployment-id>/domain/<lowercase-key-domain-id>/kek/1`。receipt 的 `schema_manifest_sha256/ep_migrate_pe_sha256` 还必须分别命中当前签名 schema manifest 与 `DeploymentManifestV1.artifacts[EP_MIGRATE]`。不得把后来的 grant、第二法人/密钥域、无审计补写或人工布尔值拼成首装证明。两个 operator entry 必须与 bootstrap 两项及 manifest 两个不同 certificate roster entry逐字段相等；X509 credential 的数据库 `verifier` 只能是 leaf exact DER 的 `cert-sha256:<64-lowerhex>`，`credential_handle` 只能是该 leaf SKI raw bytes，禁止 display DN/serial/SPKI-only 身份。Stage 14b 必须让两名 bootstrap 用户各自以 password+该 exact X509 leaf 完成 sign-in 与完整 MFA，并把零退出码、审计 projection 与 transcript digest 留证；password-only、X509-only、同证双人、同 SPKI 换 DER、错 SKI、登录 trust bundle 未唯一接受、ep-migrate/core resolver bytes 不等任一负例都必须非零。

collector 还必须逐列核对 initial-governance 完整 envelope：预分配 event id、signed 治理法人、该法人 SYSTEM grant、null device、固定 action/object/id/version、null before/reason/approval/reauth、exact after、system client 与 committed_at；链列只按 AuditWriter 重算。任一 actor/法人/null 列漂移即使 payload digest 与链在其自身字节上自洽，也不得形成 PASS。

initial-governance 输出根唯一为 `target/release-evidence/initial-governance/<lowercase-stage14-run-id>/`，顶层 exact-set 恰为一个≤1048576-byte `initial-governance-evidence.v1.jcs`，ref 唯一为 `ep-evidence://stage14/<same-run>/initial-governance/initial-governance-evidence/sha256/<digest>`；它不另加 sidecar/purpose，而由下述 `F56LicenseModuleLifecycleEvidenceV1` 的 exact child ref/digest 传递绑定，再由共同 gate 的已签 index 闭包。该 lifecycle manifest 同时必须带 `deployment_manifest_evidence_ref/sha256` 与 `initial_governance_evidence_ref/sha256`，两者与 lifecycle 十 case、trust/admission reports 均同 run/deployment/build/closed window；因此共同 gate 顶层 roster 仍恰为四项，不增加自由格式第五项。same-SPKI/different-DER、任一 roster 三列错配/重复/乱序、manifest/body/receipt/path deployment id 不等、登录 anchor ref/digest/recipient 不等、坏 CMS 属性/时间/算法/chain/CRL、错误 DACL、receipt sidecar、receipt/审计/DB/grant/key-domain 任一断链，以及只完成 password 或 X509 一因子，都是具名负例并阻止共同 PASS。

上述表格同时冻结全局 `Stage14GateCodeV1` 的 exact 15-value wire 闭集，按表中顺序恰为 `RG-CI-PROBE-ABSENT|RG-TOOLS-EXCLUDED|RG-PLAINTEXT-SECRETS-ABSENT|RG-RLS-MATRIX-GREEN|RG-UNWIRED-ABSENT|RG-NO-UNDECIDABLE|RG-OFFSITE-COPY-PROTECTED|RG-EXTERNAL-CLAIMS-SIGNED|RG-LICENSE-MODULE-LIFECYCLE-GREEN|RG-AI-CONTAINMENT-GREEN|RG-AI-RESOURCE-CERTIFIED|RG-MCP-CONFORMANCE-GREEN|RG-MCP-CONTAINMENT-GREEN|RG-SERVER-ADMIN-MATRIX-90-GREEN|RG-DEPLOYMENT-CARRIER-EVIDENCE-GREEN`，未知、第十六值、重复别名或大小写变体一律拒绝。门禁文件不再各自发明“结果 JSON”或只散列若干 digest；唯一通用 ABI 如下，字段名与字段集均为 exact，所有 UUID 为 lowercase canonical，所有 SHA-256 为 64 lowercase hex，时间为 UTC 秒精度。

```rust
pub enum Stage14GateCodeV1 { // wire 恰为上述十五个 RG-* 字面量
    RgCiProbeAbsent,
    RgToolsExcluded,
    RgPlaintextSecretsAbsent,
    RgRlsMatrixGreen,
    RgUnwiredAbsent,
    RgNoUndecidable,
    RgOffsiteCopyProtected,
    RgExternalClaimsSigned,
    RgLicenseModuleLifecycleGreen,
    RgAiContainmentGreen,
    RgAiResourceCertified,
    RgMcpConformanceGreen,
    RgMcpContainmentGreen,
    RgServerAdminMatrix90Green,
    RgDeploymentCarrierEvidenceGreen,
}

pub enum Stage14GateOutcomeV1 { Pass } // wire 唯一为 "PASS"

pub struct Stage14GateEvidenceEntryV1 {
    pub evidence_code: EvidenceCodeV1,
    pub evidence_ref: OpaqueEvidenceRef,
    pub evidence_sha256: Sha256Digest,
}

pub struct Stage14GateEvidenceIndexV1 {
    pub schema_version: u16, // exact 1
    pub gate_code: Stage14GateCodeV1,
    pub stage14_run_id: Uuid,
    pub deployment_id: Uuid,
    pub product_build_sha256: Sha256Digest,
    pub entries: Vec<Stage14GateEvidenceEntryV1>, // 1..=256，排序且唯一
    pub observed_at: DateTime<Utc>,
}

pub struct Stage14GateResultV1 {
    pub schema_version: u16, // exact 1
    pub gate_code: Stage14GateCodeV1,
    pub outcome: Stage14GateOutcomeV1,
    pub stage14_run_id: Uuid,
    pub deployment_id: Uuid,
    pub product_build_sha256: Sha256Digest,
    pub evidence_index_ref: OpaqueEvidenceRef,
    pub evidence_index_sha256: Sha256Digest,
    pub observed_at: DateTime<Utc>,
}
```

每项 gate 的最终根唯一为 `target/release-evidence/gates/<lowercase-stage14-run-id>/<lowercase-gate-code>/`，其中 `<lowercase-gate-code>` 是该 `RG-*` wire 值的 ASCII lowercase，目录内 final exact-set 恰为 `evidence-index.v1.jcs`、`evidence-index.v1.jcs.sig.jcs`、`gate-result.v1.jcs`、`gate-result.v1.jcs.sig.jcs`。result 内 index ref 唯一为 `ep-evidence://stage14/<lowercase-stage14-run-id>/gates/<lowercase-gate-code>/evidence-index/sha256/<64-lowerhex>`；ref 末段、`evidence_index_sha256` 与 exact index JCS bytes 的 SHA-256 必须三者相等。两份 sidecar 都复用下文同一个 `Stage14EvidenceSignatureV1`、preimage、deployment-key signer 与 key-state resolver，purpose 分别且只能为 `STAGE14_GATE_EVIDENCE_INDEX_V1`、`STAGE14_GATE_RESULT_V1`，各自 `evidence_sha256` 必须等于相邻 JCS 的 digest。

`EvidenceCodeV1` 的 wire 是 1..128 ASCII bytes、只匹配 `[a-z][a-z0-9_]{0,127}`；`OpaqueEvidenceRef` 在本 ABI 中为 1..2048 ASCII bytes。`entries` 按 `(evidence_code UTF-8 bytes,evidence_ref canonical bytes)` 升序、组合唯一；`evidence_code` 不是自由字符串，而必须命中 `tools/release-gate/src/registry.rs` 对该 gate 编译期冻结的 typed evidence exact roster。每个 registry 项只选择一个有界 strict-JCS typed parser 和一个已冻结的 `ep-evidence://stage14/<same-run>/.../sha256/<digest>` resolver；resolver 必须以安全 handle 解析并复验 ref 末段、entry digest 与 exact bytes，相同 `stage14_run_id/deployment_id/product_build_sha256` 及同一 closed run window 缺一不可。raw/absolute/relative filesystem path、未知 ref kind、自引用 gate index/result、escape、reparse point、ADS、hardlink、跨 run/build/deployment 或窗口外证据全部拒绝，不能把普通文件存在折算成 typed evidence。

`RG-LICENSE-MODULE-LIFECYCLE-GREEN` 的编译期 typed roster 恰为且只为 `evidence_code=license_module_lifecycle_matrix|license_admission_registry_exact_set|license_admission_negative_matrix|license_trust_rotation_exact_set` 四项；不得声称还有“F-56 第 8 节其他证据”却不登记 code/parser，也不得合成一个布尔值。四项 source 根唯一为 `target/release-evidence/license-module/<lowercase-stage14-run-id>/`，顶层文件恰为 `license-module-lifecycle.v1.jcs`、`license-admission-registry.v1.jcs`、`license-admission-negatives.v1.jcs`、`license-trust-rotation.v1.jcs`；lifecycle case 只允许位于 `lifecycle/<lowercase-snake-case-wire>.v1.jcs`，trust chunk 只允许位于 `trust-rotation-chunks/<ten-digit-zero-padded-1-based-u32-chunk-no>.v1.jcs`，被 RELEASED special item 实际引用的 accepted bundle exact bytes 只允许按 digest 去重位于 `accepted-trust-bundles/<64-lowerhex>.p7b`。四项顶层 ref 分别且只能为 `ep-evidence://stage14/<same-run>/license-module/license-module-lifecycle/sha256/<digest>`、`ep-evidence://stage14/<same-run>/license-module/license-admission-registry/sha256/<digest>`、`ep-evidence://stage14/<same-run>/license-module/license-admission-negatives/sha256/<digest>`、`ep-evidence://stage14/<same-run>/license-module/license-trust-rotation/sha256/<digest>`；lifecycle case ref 只能为 `ep-evidence://stage14/<same-run>/license-module/license-module-lifecycle-case/<case-wire>/sha256/<digest>`，chunk ref 只能为 `ep-evidence://stage14/<same-run>/license-module/license-trust-rotation-chunk/<ten-digit-zero-padded-chunk-no>/sha256/<digest>`，accepted bundle ref 只能为 `ep-evidence://stage14/<same-run>/license-module/license-accepted-trust-bundle/sha256/<digest>`。每个 lifecycle case、trust chunk、admission report 与 bundle 都是≤1048576 bytes；`license-trust-rotation.v1.jcs` 由受信 DB exact-set 流式生成并由 strict streaming parser 以 checked `u64` 累计，受部署证据卷容量约束但不得施加历史条目数或 256-chunk 业务总上限。顶层四份不增加 report-specific sidecar 或新 purpose，而是由共同 gate 的 `Stage14GateEvidenceIndexV1` entry digest 和已验签 index 逐项绑定，case/chunk/bundle 再由对应已绑定 manifest entry 传递绑定。`tools/release-gate/src/license_module.rs` 是四个 code/ref 及其 child ref 的唯一 strict parser/resolver；任何 raw path、自由 JSON、人工 entitlement 或未被下列 exact DTO 覆盖的 report 都不入 roster。

共同 collector 对每一份 special `.epcfg` 只接受 F-56 唯一 container，不能由 ZIP 库默认值补齐：archive 总长 `<=4,193,900`，单卷 ZIP32、三个 `STORE` regular-file entry，local-header 与 central-directory 顺序都逐字为 `manifest.toml,item.jcs,outer-signature.p7s`，零第四项。每个 local header 固定 `version-needed=20,flags=0,method=0,time=0x0000,date=0x0021,extra-length=0`；每个 central header 另固定 `version-made-by` raw u16=20、comment/disk/internal/external attributes 全零，并与 local 的 CRC-32、两项 size、name length、offset逐字相等；唯一 EOCD 的 disk 两值为 0、entry count 两值为 3、comment length=0 且 EOCD 结束即 EOF，固定总 overhead=330 bytes。拒绝 ZIP64、descriptor、加密、extra/comment、目录/重复/大小写碰撞/path traversal、symlink/hardlink/reparse、嵌套包与 trailing。special 的 `item.jcs` 恰为 after-spec RFC 8785 JCS exact bytes，`item_hash=SHA-256(item.jcs exact bytes)`；普通 item 保持 ADD/MODIFY 对 after-spec JCS、REMOVE 对 before-spec JCS 求 hash。inner 与 outer `.p7s` 都必须是单个完整 DER `ContentInfo`/`[0] EXPLICIT SignedData`、`SignedData.version=3`、零 trailing、detached content、SKI `SignerInfo.version=3`，signedAttrs wire 为 `[0] IMPLICIT` 且实际签名 preimage 为 canonical universal `SET OF`（`0x31`）；cert set 仅 leaf+必要 intermediate、无 root/CRL/多余证书。`license-roots.p7b` 只接受单个完整 DER degenerate `ContentInfo/SignedData.version=1`、空 digestAlgorithms/SignerInfos DER SET、empty content、按 DER SET 排序的 CA/base-CRL bag。任一 ContentInfo/raw-SignedData 二义性、错误 version/tag/preimage/order/header 或宽松解析都必须在落库前非零。

```rust
pub enum F56LicenseModuleLifecycleCaseV1 { // wire/path exact lowercase_snake_case
    LicenseTimeAndStatus,
    RenewalAndCurrentSignerRecovery,
    SpecialPackageEnvelopeAndSignature,
    UsageAndScope,
    ModuleTransitionsAndDrain,
    ModuleDisableRetentionAndReenable,
    SpecialPackageAndServerAdminImport,
    RestrictedEffectMatrix,
    F55EntitlementProjection,
    PostgresqlTerminalShape,
}

pub struct F56LicenseModuleLifecycleCaseEvidenceV1 {
    pub schema_version: u16, // exact 1
    pub stage14_run_id: Uuid,
    pub deployment_id: Uuid,
    pub product_build_sha256: Sha256Digest,
    pub case: F56LicenseModuleLifecycleCaseV1,
    pub test_binary_roster_sha256: Sha256Digest,
    pub test_binary_count: u16,
    pub command_roster_sha256: Sha256Digest,
    pub command_count: u16,
    pub fixture_registry_sha256: Sha256Digest,
    pub assertion_registry_sha256: Sha256Digest,
    pub assertion_count: u32,
    pub passed_count: u32,
    pub failed_count: u32,
    pub aggregate_exit_code: i32,
    pub environment_facts_sha256: Sha256Digest,
    pub execution_transcript_sha256: Sha256Digest,
    pub observed_at: DateTime<Utc>,
}

pub struct F56LicenseModuleLifecycleCaseRefV1 {
    pub case: F56LicenseModuleLifecycleCaseV1,
    pub case_ref: OpaqueEvidenceRef,
    pub case_sha256: Sha256Digest,
}

pub struct F56LicenseModuleLifecycleEvidenceV1 {
    pub schema_version: u16, // exact 1
    pub stage14_run_id: Uuid,
    pub deployment_id: Uuid,
    pub product_build_sha256: Sha256Digest,
    pub deployment_manifest_evidence_ref: OpaqueEvidenceRef,
    pub deployment_manifest_evidence_sha256: Sha256Digest,
    pub initial_governance_evidence_ref: OpaqueEvidenceRef,
    pub initial_governance_evidence_sha256: Sha256Digest,
    pub entries: Vec<F56LicenseModuleLifecycleCaseRefV1>, // exact 10，enum 顺序
    pub observed_at: DateTime<Utc>,
}

pub struct F56LicenseAdmissionRegistryEvidenceV1 {
    pub schema_version: u16, // exact 1
    pub stage14_run_id: Uuid,
    pub deployment_id: Uuid,
    pub product_build_sha256: Sha256Digest,
    pub core_binding_registry_sha256: Sha256Digest,
    pub core_actual_operation_registry_sha256: Sha256Digest,
    pub worker_binding_registry_sha256: Sha256Digest,
    pub worker_actual_operation_registry_sha256: Sha256Digest,
    pub core_binding_count: u32,
    pub worker_binding_count: u32,
    pub xtask_exact_set_report_sha256: Sha256Digest,
    pub blocking_selfcheck_report_sha256: Sha256Digest,
    pub observed_at: DateTime<Utc>,
}

pub enum F56LicenseAdmissionNegativeCaseV1 { // wire/顺序 exact
    MissingBinding,
    ExtraBinding,
    DuplicateBinding,
    WrongConfigReleaseResolver,
    WrongMcpInboundResolver,
    LegalEntityScopeBypass,
    FirstOrRetryAsInFlightConvergence,
    SharedGuardOrderOrErrorRewrite,
}

pub enum F56LicenseAdmissionObservedErrorV1 {
    PlatformLicenseRestricted, // wire exact "PLATFORM.LICENSE.RESTRICTED"
}

pub struct F56LicenseAdmissionNegativeResultV1 {
    pub case: F56LicenseAdmissionNegativeCaseV1,
    pub xtask_exit_code: Option<NonZeroI32>,
    pub blocking_selfcheck_exit_code: Option<NonZeroI32>,
    pub runtime_exit_code: Option<NonZeroI32>,
    pub observed_error_code: Option<F56LicenseAdmissionObservedErrorV1>,
    pub probe_report_sha256: Sha256Digest,
}

pub struct F56LicenseAdmissionNegativeEvidenceV1 {
    pub schema_version: u16, // exact 1
    pub stage14_run_id: Uuid,
    pub deployment_id: Uuid,
    pub product_build_sha256: Sha256Digest,
    pub entries: Vec<F56LicenseAdmissionNegativeResultV1>, // exact 8，enum 顺序
    pub observed_at: DateTime<Utc>,
}

pub enum F56TrustRotationArtifactKindV1 {
    LicenseGrant,
    LicenseRevocation,
    ModulePackage,
}

pub enum F56TrustRotationItemResultV1 {
    Trusted,
    HistoricalSignerRevoked,
    CurrentModuleSignerRevokedContained,
    ModuleSignerRevokedDisableAuthorization,
}

pub enum F56TrustRotationSignerStateV1 {
    Active,
    Retired,
    Revoked,
}

pub enum F56CurrentProjectionKindV1 {
    CurrentGrant,
    CurrentRevocation,
    CurrentModule,
}

pub struct F56TrustRotationItemEvidenceV1 {
    pub schema_version: u16, // exact 1
    pub purpose: String, // exact "EP-F56-TRUST-ROTATION-ITEM-V1"
    pub config_package_id: Uuid,
    pub config_item_id: Uuid,
    pub origin_config_item_id: Uuid,
    pub artifact_kind: F56TrustRotationArtifactKindV1,
    pub artifact_id: Uuid,
    pub accepted_at: DateTime<Utc>,
    pub acceptance_audit_event_id: Uuid,
    pub acceptance_audit_payload_sha256: Sha256Digest,
    pub acceptance_audit_chain_hash_sha256: Sha256Digest,
    pub accepted_trust_bundle_sha256: Sha256Digest,
    pub accepted_trust_bundle_evidence_ref: OpaqueEvidenceRef,
    pub validation_trust_bundle_sha256: Sha256Digest,
    pub source_projection_sha256: Sha256Digest,
    pub payload_sha256: Sha256Digest,
    pub inner_signature_cms_sha256: Sha256Digest,
    pub accepted_inner_signer_subject: String, // exact spki-sha256:<64-lowerhex>
    pub accepted_inner_signer_state: F56TrustRotationSignerStateV1, // 现行 validation bundle 下该 accepted inner 的状态
    pub accepted_inner_chain_sha256: Sha256Digest,
    pub validation_inner_chain_sha256: Sha256Digest,
    pub outer_manifest_sha256: Sha256Digest,
    pub outer_signature_cms_sha256: Sha256Digest,
    pub source_outer_signer_subject: String, // exact spki-sha256:<64-lowerhex>
    pub source_outer_signer_state: F56TrustRotationSignerStateV1, // 现行 validation bundle 下该 source outer 的状态
    pub accepted_outer_chain_sha256: Sha256Digest,
    pub validation_outer_chain_sha256: Sha256Digest,
    pub module_code: Option<ModuleCode>,
    pub module_action: Option<ModulePackageActionV1>,
    pub current_projection_kind: Option<F56CurrentProjectionKindV1>,
    pub current_projection_sha256: Option<Sha256Digest>,
    pub current_module_install_state: Option<ModuleState>,
    pub recovery_peer_config_item_id: Option<Uuid>,
    pub module_signer_revoked_disabled_audit_event_id: Option<Uuid>,
    pub module_signer_revoked_disabled_audit_payload_sha256: Option<Sha256Digest>,
    pub module_signer_revoked_disabled_audit_chain_hash_sha256: Option<Sha256Digest>,
    pub revoked_layer_crl_registry_sha256: Option<Sha256Digest>,
    pub result: F56TrustRotationItemResultV1,
}

pub struct F56TrustRotationChunkV1 {
    pub schema_version: u16, // exact 1
    pub stage14_run_id: Uuid,
    pub deployment_id: Uuid,
    pub product_build_sha256: Sha256Digest,
    pub chunk_no: u32,
    pub total_chunks: u32,
    pub entries: Vec<F56TrustRotationItemEvidenceV1>, // 1..=512
    pub observed_at: DateTime<Utc>,
}

pub struct F56TrustRotationChunkRefV1 {
    pub chunk_no: u32,
    pub entry_count: u16,
    pub chunk_ref: OpaqueEvidenceRef,
    pub chunk_sha256: Sha256Digest,
}

pub struct F56LicenseTrustRotationEvidenceV1 {
    pub schema_version: u16, // exact 1
    pub stage14_run_id: Uuid,
    pub deployment_id: Uuid,
    pub product_build_sha256: Sha256Digest,
    pub validation_trust_bundle_sha256: Sha256Digest,
    pub license_trusted_signer_subject_registry_sha256: Sha256Digest,
    pub trusted_now: DateTime<Utc>,
    pub complete_base_crl_registry_sha256: Sha256Digest,
    pub highest_covering_crl_registry_sha256: Sha256Digest,
    pub released_special_item_registry_sha256: Sha256Digest,
    pub ordinary_or_unreleased_null_registry_sha256: Sha256Digest,
    pub ordinary_or_unreleased_item_count: u64,
    pub current_license_projection_sha256: Sha256Digest,
    pub current_grant_projection_sha256: Option<Sha256Digest>,
    pub current_revocation_projection_sha256: Option<Sha256Digest>,
    pub current_license_status: LicenseStatus,
    pub current_license_restriction_reason: Option<LicenseRestrictionReason>,
    pub current_module_projection_registry_sha256: Sha256Digest,
    pub module_runtime_containment_report_sha256: Sha256Digest,
    pub trust_rotation_negative_matrix_sha256: Sha256Digest,
    pub total_entry_count: u64,
    pub chunks: Vec<F56TrustRotationChunkRefV1>, // zero entries => []；否则无 256-chunk 业务上限
    pub observed_at: DateTime<Utc>,
}
```

上列 trust digest 不再允许“排序 strict-JCS digest”这种未封口说法。统一派生摘要原语唯一为 `projection_digest(domain,dto)=SHA-256(ASCII(domain)||0x00||RFC8785_JCS(dto))`；每个由 `projection_digest` 计算的 DTO 根（不是已有 raw report/chunk 文件 ABI）的 `schema_version` 固定为 JSON number `1`（JSON string `"1"` 必须拒绝）、`purpose` 固定为与 `domain` 完全相同的 ASCII 字面量，strict parser 拒绝 unknown/duplicate/missing。UUID 只收 canonical lowercase，时间只收 RFC3339 UTC whole-second，digest 只收 64 lowerhex，enum 只收本节/F-56 既有 wire；所有 Option key **始终存在**，无值写 JSON `null`，空 Vec 写 `[]`。数组按下列键升序且组合唯一，排序比较 UUID raw 16 bytes、其他字符串 UTF-8 bytes；未明示可空的字段一律非空。只有 raw artifact/evidence bytes 直接取 `SHA-256(exact bytes)` 而不加 domain：payload/item JCS exact bytes、canonical `manifest.toml`、CMS DER、`.p7b` bundle、`.epcfg` archive、case/chunk/report JCS 文件、fixture/transcript。raw case/chunk/report 自身保持上列 struct 的既有字段集，不因其文件 digest 被强加 `purpose`；但该文件承载的下述具名 semantic `*_sha256` 仍必须对相应 exact DTO 调 `projection_digest`。因此同一 JCS 可同时具有用于 ref/文件完整性的 direct-byte SHA 与用于语义投影的 domain digest，两者不得互换。`EP-CMS-CHAIN-V1` 是唯一特例：`SHA-256(ASCII("EP-CMS-CHAIN-V1")||0x00||leaf→intermediate→anchor 每张 exact DER 的 u32-big-endian 长度和 DER bytes)`。

以下是各字段唯一可重算前像；花括号中的键集与顺序描述是 schema 而非可省略示例：

`license_trusted_signer_subject_registry_sha256` 逐字复用签名部署清单的 `EP-DEPLOYMENT-LICENSE-TRUSTED-SIGNER-SUBJECT-REGISTRY-V1` DTO/domain，subjects 必须与 `DeploymentManifestV1.license_trusted_signer_subjects` exact-equal；trust collector 的同名字段、manifest evidence 与 initial-governance child 三者都必须命中同一已验签 manifest。每个 entry 的 accepted inner 与 source outer subject 必须分别唯一命中 roster，全部 RELEASED history 的两类 referenced token exact-set 还必须是 roster 的子集；该 containment 由 chunks、released-special registry 与 DB exact-set 重算，不得只看 current projection。缺/空/乱序/重复、删除任一历史引用 token、local assertion 非空但不等、signer 不在 roster、roster/bundle 跨 deployment/build/batch 或 CAB 只轮换一侧都在链状态分类前失败；保留的旧 token 仍由 CRL 优先判态，不产生 ACTIVE 授权。

1. `source_projection_sha256` 的 domain/purpose 都是 `EP-CONFIG-SPECIAL-SOURCE-PROJECTION-V1`，DTO exact 为 `{schema_version,purpose,config_package_id,package_no,source,status,content_hash,outer_signature_sha256,outer_signer_subject,outer_signed_at,config_item_id,item_kind,item_code,change_kind,sort_no,applies_to_legal_entity_ids,before_spec_sha256,after_spec_sha256,item_hash,accepted_trust_bundle_sha256}`。special 固定 `source=IMPORTED,status=RELEASED,change_kind=ADD,sort_no=1,applies_to_legal_entity_ids=[]`；`content_hash=outer_manifest_sha256=SHA-256(manifest.toml exact bytes)`，`outer_signature_sha256=outer_signature_cms_sha256`，`before_spec_sha256=null`，`after_spec_sha256=item_hash=SHA-256(item.jcs exact bytes)`，其余逐列等于 terminal package/item row。
2. `released_special_item_registry_sha256` 使用 `EP-F56-RELEASED-SPECIAL-ITEM-REGISTRY-V1`，root exact `{schema_version,purpose,entries}`，entry exact `{artifact_kind,artifact_id,config_package_id,config_item_id,accepted_trust_bundle_sha256}`；按 `(artifact_kind wire,config_package_id,config_item_id)` 排序，与 chunks 的 tuple/count exact-set 相等。`LICENSE_GRANT|MODULE_PACKAGE` special source package 第一次进入 `RELEASED` 后永久保持 `RELEASED`，不参加 generic `RELEASED→SUPERSEDED`；多个 special RELEASED 同时存在是合法 history，current/history 身份只由 license current-slot/revocation 与 15 行 module projection决定。任何 special source 为 `SUPERSEDED` 都在建 registry 前直接失败，不能伪装成未发布项。`ordinary_or_unreleased_null_registry_sha256` 使用 `EP-F56-NULL-ACCEPTANCE-REGISTRY-V1`，root 同形，entry exact `{classification,config_package_id,config_item_id,item_kind,package_status,accepted_trust_bundle_sha256}`；`classification` 只取 `ORDINARY|UNRELEASED_SPECIAL`，末字段必须 JSON null，按 package/item UUID 排序；`UNRELEASED_SPECIAL` 只含从未 RELEASE 的合法前置/拒绝终态，不含 `SUPERSEDED`，entries 恰等于全库 ordinary item 加合法 unreleased special item，长度等于 `ordinary_or_unreleased_item_count`。
3. `current_grant_projection_sha256` 使用 `EP-F56-CURRENT-GRANT-PROJECTION-V1`，DTO exact `{schema_version,purpose,row_version,payload,payload_sha256,inner_signature_cms_sha256,inner_signer_subject,accepted_trust_bundle_sha256,source_config_package_id,source_config_item_id,current_slot,superseded_at,last_trusted_at}`；`payload` 是 F-56 `LicenseGrantPayloadV1` 全字段 exact object，scope UUID、module code、entitlement code 数组各按 wire 排序去重，current projection 固定 `current_slot=0,superseded_at=null`。`current_revocation_projection_sha256` 使用 `EP-F56-CURRENT-REVOCATION-PROJECTION-V1`，DTO exact `{schema_version,purpose,grant_id,grant_row_version,payload,payload_sha256,inner_signature_cms_sha256,inner_signer_subject,accepted_trust_bundle_sha256,source_config_package_id,source_config_item_id,revoked_at}`；`payload` 是 F-56 `LicenseRevocationPayloadV1` 全字段 exact object。`current_license_projection_sha256` 使用 `EP-F56-CURRENT-LICENSE-PROJECTION-V1`，DTO exact `{schema_version,purpose,current_grant_projection_sha256,current_revocation_projection_sha256,trusted_now,license_status,restriction_reason}`，两个 digest Option 与顶层字段逐字相等；零 current 的唯一形状为 `null,null,RESTRICTED,NO_CURRENT_GRANT`。
4. 每个 module row 的 digest 使用 `EP-F56-CURRENT-MODULE-PROJECTION-V1`，DTO exact 为 `{schema_version,purpose,id,module_code,display_name,row_version,install_state,package_id,package_code,package_version,package_payload_sha256,package_signature_cms_sha256,package_signer_subject,package_signed_at,module_contract_version,module_contract_sha256,min_platform_version,max_platform_version_exclusive,released_on,source_config_package_id,source_config_item_id,installed_at,state_changed_at,enabled_at,disabled_at,last_transition_reason}`；`package_version/min_platform_version/max_platform_version_exclusive` wire 都是 `Option<SemVerV1>` strict object（不是 dotted string），其中已安装行前两项非空、max 依合同可空；`module_contract_version` 虽由 Rust `u32` 承载，但证据、签名 source 与 PostgreSQL `integer` 的共同有效域固定为 `1..=2147483647`，数据库列不得改为 bigint，0、2147483648 与任何溢出/lossy cast 都在投影前拒绝。NOT_INSTALLED 的全部 package/source/version/time/reason Option 必须逐字为 null，其他状态按 F-56 terminal shape，并与唯一 source inner逐值相等。`current_module_projection_registry_sha256` 使用 `EP-F56-CURRENT-MODULE-PROJECTION-REGISTRY-V1`，root exact `{schema_version,purpose,entries}`，entry exact `{module_code,current_projection_sha256}`，恰 15 行、按 ModuleCode wire 排序；chunk 的 current-module digest 必须命中对应行，不能拿单行 digest 与 root digest比较。
5. `complete_base_crl_registry_sha256` 使用 `EP-F56-COMPLETE-BASE-CRL-REGISTRY-V1`，root exact `{schema_version,purpose,entries}`，entry exact `{issuer_subject_der_sha256,issuer_spki_sha256,issuer_subject_key_identifier_b64url,crl_number_decimal,this_update,next_update,crl_der_sha256,signature_algorithm}`；`signature_algorithm` exact enum 只有 `ECDSA_P256_SHA256|RSA_PSS_SHA256`，枚举 validation bundle 全部结构/签名合法 base CRL，CRLNumber 是非负无前导零十进制，按 `(issuer_subject_der_sha256,issuer_subject_key_identifier decoded bytes,numeric CRLNumber,this_update,crl_der_sha256)` 排序。选择算法固定为 **global-highest-then-cover**：对 inner/outer 唯一链的每个实际 issuer，都先从其全部结构/签名合法 base CRL 取全局 numeric CRLNumber 最大值并要求同号 DER 唯一，再要求该最高号满足 `thisUpdate<=trusted_now<=nextUpdate`；必须先让所有 issuer 的这一前置完整成功，任一缺失、最高号过期/尚未生效/冲突即令整项 UNTRUSTED，绝不回退低号、扫描任何 serial 或进入窄恢复。只有此前置全绿后才生成 `highest_covering_crl_registry_sha256` 并扫描 serial。该 registry 使用 `EP-F56-HIGHEST-COVERING-CRL-REGISTRY-V1`，root 同形，entry exact `{artifact_kind,config_package_id,config_item_id,layer,chain_position,certificate_der_sha256,certificate_serial_hex,issuer_subject_der_sha256,issuer_subject_key_identifier_b64url,selected_crl_der_sha256,crl_number_decimal,this_update,next_update,serial_revoked}`；`layer=INNER|OUTER`，`chain_position:u16` 固定 0=leaf 后逐级递增，每个 released entry 的每层每张 non-anchor 恰一行；`certificate_serial_hex` 是正整数最短 unsigned big-endian lowercase hex、禁止 leading `00`，按 artifact/package/item/layer/position 排序。单项 `revoked_layer_crl_registry_sha256` 是同 domain/root schema 仅保留该 item 且 `serial_revoked=true` 的 exact 子集摘要；两层均未撤销时必须为 null，任一层撤销时必须非空并零缺项。
6. `module_runtime_containment_report_sha256` 使用 `EP-F56-MODULE-RUNTIME-CONTAINMENT-V1`，root exact `{schema_version,purpose,entries}`，恰 15 个按 module wire 排序的 entry：`{module_code,install_state,package_trust_status,source_config_item_id,trust_rotation_result,raw_enabled,effective_runtime_allowed,expected_effective_runtime_allowed,read_export_probe,write_probe,approval_probe,automation_claim_probe,outbound_probe}`。`source_config_item_id:Option<Uuid>` 与 `trust_rotation_result:Option<F56TrustRotationItemResultV1>` 的 key 始终存在：NOT_INSTALLED 必须 `null/null`，两个已安装态必须都非空、source 命中 current module projection 且 result 等于同 source 的 trust entry。`package_trust_status` 只取 `NOT_INSTALLED|TRUSTED|SIGNER_REVOKED|INVALID`；probe exact `{outcome,exit_code,observed_error_code,transcript_sha256}`，outcome 只取 `ALLOWED|BLOCKED|NOT_APPLICABLE`，error Option 只取 null 或 `PLATFORM.LICENSE.RESTRICTED|PLATFORM.MODULE.LICENSE_REQUIRED|PLATFORM.MODULE.PACKAGE_INVALID_OR_INCOMPATIBLE`。ALLOWED 固定 exit 0/error null，BLOCKED 固定非零/非空，NOT_APPLICABLE 固定 exit 0/error null；actual 必须等于 expected。被 CRL 收容且 disabled 的模块必须 read/export=ALLOWED，四项副作用 probe 全 BLOCKED。
7. `trust_rotation_negative_matrix_sha256` 使用 `EP-F56-TRUST-ROTATION-NEGATIVE-MATRIX-V1`，root exact `{schema_version,purpose,entries}`，entry exact `{case,fixture_sha256,collector_exit_code,failure_stage,transcript_sha256}`，`collector_exit_code` 必须非零，`failure_stage` 只取 `SOURCE|ACCEPTANCE|CURRENT_PROJECTION|CHAIN|CRL|RECOVERY|CONTAINMENT|NULL_ACCEPTANCE`。case 按此 exact enum 顺序恰 30 项：`MISSING_ORIGIN|DUPLICATE_ORIGIN|INNER_OUTER_MERGED|SIGNER_NOT_IN_DEPLOYMENT_ROSTER|CURRENT_REVOKED_WITHOUT_RECOVERY|CURRENT_UNTRUSTED|NEW_CANDIDATE_RETIRED|NEW_CANDIDATE_REVOKED|MISSING_BASE_CRL|EXPIRED_CRL|DUPLICATE_HIGHEST_CRL|DELTA_CRL|INDIRECT_CRL|REMOVE_FROM_CRL|MISSING_REVOKED_LAYER_EVIDENCE|HISTORICAL_NON_CRL_DRIFT|HISTORICAL_CLASSIFICATION_MISMATCH|RECOVERY_SUPERSEDES_MISMATCH|RECOVERY_GOVERNANCE_MISMATCH|RECOVERY_PEER_MISSING|RECOVERY_PEER_NONUNIQUE|RECOVERY_AUDIT_PROJECTION_DRIFT|SPECIAL_RELEASED_SUPERSEDED|MODULE_STILL_ENABLED|ORDINARY_ACCEPTED_DIGEST_NON_NULL|UNRELEASED_ACCEPTED_DIGEST_NON_NULL|ACCEPTED_TRUST_BUNDLE_MISSING|ACCEPTED_TRUST_BUNDLE_DIGEST_MISMATCH|ACCEPTANCE_AUDIT_MISSING_OR_MISMATCH|RETIRED_WITHOUT_FIRST_ACTIVE_EVIDENCE`；缺、重、额外、零退出码或自由字符串均失败。`SIGNER_NOT_IN_DEPLOYMENT_ROSTER` 固定从一份 otherwise-valid 新 manifest roster 删除一个仍被某条 RELEASED history 的 inner 或 source outer 引用、且可由 bundle 唯一成链的 SPKI token；必须在 trust state 分类与 manifest 安装前以 `SOURCE` 非零失败，这同时是 historical referenced-set 不是 roster 子集的删除负例。`RECOVERY_PEER_MISSING` 固定删除唯一 `MODULE_SIGNER_REVOKED_DISABLED` action，`RECOVERY_PEER_NONUNIQUE` 固定追加第二条同 module/recovery tuple 的 action，`RECOVERY_AUDIT_PROJECTION_DRIFT` 固定逐一变异 payload、hash chain、audit before/after projection、object version、时间、四个 source/recovery id 或 reason digest；三者都必须在 peer 配对前非零失败。`SPECIAL_RELEASED_SUPERSEDED` 固定把一个已首次 RELEASE 的 `LICENSE_GRANT|MODULE_PACKAGE` source package 篡改为 `SUPERSEDED`，必须在 exact-set 分类前以 `SOURCE` 非零失败。`MISSING_BASE_CRL|EXPIRED_CRL|DUPLICATE_HIGHEST_CRL` 三个 golden fixture 都固定让一层存在可命中的 revoked serial、另一层分别出现缺 CRL/最高号过期/最高号同号 DER 冲突；三者必须在 serial scan 与 recovery 前得到整项 UNTRUSTED、`revoked_layer_crl_registry_sha256=null` 且 collector 非零，禁止误判 `HISTORICAL_SIGNER_REVOKED`。
8. `grant_trust_rotation_entry_sha256/revocation_trust_rotation_entry_sha256` 使用 `EP-F56-TRUST-ROTATION-ITEM-V1`，前像就是对应 `F56TrustRotationItemEvidenceV1` 的**全字段** exact object；因此 entry 自身固定带 `schema_version=1,purpose="EP-F56-TRUST-ROTATION-ITEM-V1"`，所有 Option 显式 null，任何省略字段、只摘要部分 signer/source 或对 chunk bytes 切片求 hash 都不等价。chunk 中的 object、按该 domain 重算的 entry digest 与 F-55 summary 三者必须逐字命中。

每个 trust entry 的 `accepted_at/acceptance_audit_*` 必须命中同一 source item 成功 RELEASE 事务追加的唯一 `action='platform.config_special.accepted.v1'`。这是 F-56 具名 typed audit ABI，不适用 Stage 3 对“无具名 ABI”的 numeric-string fallback；collector 必须先逐列核对 Stage 3 冻结的完整 acceptance envelope；其 closed payload exact 为 `{schema_version:1,purpose:"EP-CONFIG-SPECIAL-ACCEPTED-V1",config_package_id,config_item_id,artifact_kind,artifact_id,artifact_action,accepted_trusted_now,accepted_trust_bundle_sha256,inner_signer_subject,inner_chain_sha256,inner_trust_state,outer_signer_subject,outer_chain_sha256,outer_trust_state,payload_sha256,item_hash,content_hash,source_projection_sha256}`，其中 `schema_version` 必须是 JSON number `1`，string `"1"` 在审计解析前即拒绝并由 `ACCEPTANCE_AUDIT_MISSING_OR_MISMATCH` 负例证明。`accepted_at=accepted_trusted_now`，outer state=ACTIVE，inner state 只取 `ACTIVE|RETIRED_NONREVOKED|REVOKED_AS_DISABLE_TARGET` 的 F-56 合法形状。`accepted_inner_chain_sha256/accepted_outer_chain_sha256` 必须逐字等于该 immutable audit payload 的首次接受 chain digest；`validation_inner_chain_sha256/validation_outer_chain_sha256` 则按本次 validation bundle 与 `trusted_now` 重建当前唯一链，entry 的两个 current signer state 只由 validation chain 判定，禁止拿历史 accepted chain 冒充当前链。四个 chain digest 都使用上述 `EP-CMS-CHAIN-V1` 原语。`acceptance_audit_payload_sha256=SHA-256(payload exact JCS bytes)`，`acceptance_audit_chain_hash_sha256` 必须等于既有 append-only `AuditWriter` 对该 exact event 重算的链 hash；审计 payload、terminal DB source projection、item/manifest/two-CMS raw digest 与 entry 必须逐字交叉相等。

模块 signer 撤销后的窄停用还必须从同一 terminal batch 中唯一 `action='MODULE_SIGNER_REVOKED_DISABLED'` 的 append-only `AuditWriter` 事件重建 recovery peer，绝不按同 package、同 inner 或最近时间猜选。这同样是 F-56 具名 typed audit ABI，不适用 generic numeric-string fallback。

collector 必须证明该 batch 的唯一链顺序为 `MODULE_SIGNER_REVOKED_DISABLED` 在前、同 recovery item 的 `platform.config_special.accepted.v1` 在后且是 batch 最后一条；两事件 id 为两个互异 UUIDv7，共享冻结治理法人、同一 execute `SecurityContext` 与 package `approval_ref`，两者 `reason/reauth_ref` 均为 null。recovery event 的 object/id/version、完整 before/after 与 disabled time、accepted event 的完整 envelope 都须逐列命中 Stage 3；顺序颠倒、夹入第三事件、actor/device/client/approval 漂移均失败。

该 event 的 `before` 不是摘要占位，而是锁内更新前完整 `EP-F56-CURRENT-MODULE-PROJECTION-V1` typed DTO，exact keys 为 `{schema_version:1,purpose:"EP-F56-CURRENT-MODULE-PROJECTION-V1",id,module_code,display_name,row_version,install_state,package_id,package_code,package_version,package_payload_sha256,package_signature_cms_sha256,package_signer_subject,package_signed_at,module_contract_version,module_contract_sha256,min_platform_version,max_platform_version_exclusive,released_on,source_config_package_id,source_config_item_id,installed_at,state_changed_at,enabled_at,disabled_at,last_transition_reason}`。其中 `schema_version`、`row_version`、三个 SemVer 分量与 `module_contract_version` 都是 JSON number；`row_version` 固定 `1..=9223372036854775807`，contract version 固定 `1..=2147483647`，SemVer 是 strict object/null，其余 Option/null、lowerhex、UTC whole-second 规则沿用上段。string `"1"`、缺/多 key、越界或普通无具名审计 numeric-string fallback 均必须命中 `RECOVERY_AUDIT_PROJECTION_DRIFT` 非零负例。

event 的 `after` strict-JCS recovery payload exact 为 `{schema_version:1,purpose:"EP-MODULE-SIGNER-REVOKED-DISABLED-V1",module_code,previous_source_config_package_id,previous_source_config_item_id,recovery_config_package_id,recovery_config_item_id,before_projection_sha256,after_projection_sha256,disabled_at,reason_sha256}`，schema_version 同样只接受 JSON number `1`，unknown/missing/duplicate key 均失败。`before_projection_sha256` 必须直接从 audit `before` exact typed DTO 按 `EP-F56-CURRENT-MODULE-PROJECTION-V1` domain 重算；after DTO 只能由 before 作唯一确定变换：`row_version` checked `+1`、`install_state=INSTALLED_DISABLED`、`state_changed_at=disabled_at=event.after.disabled_at`、`last_transition_reason=recovery item reason`，其余每个 key（包括 previous source 两列与旧 inner/package 投影）逐字保留，再以同 domain 重算 `after_projection_sha256`。`reason_sha256=SHA-256(ASCII("EP-MODULE-DISABLE-REASON-V1")||0x00||UTF-8(recovery item reason))`。event envelope 固定 `object_type='platform.module_registrations'`、`object_id=before.id`、`object_version=after.row_version`、`occurred_at=after.disabled_at`；若停用仍为 current containment，数据库 current row 必须逐键等于派生 after DTO，若后来已有合法动作，则沿后续审计/投影链验证，不得要求现态倒退。

收容 pair 的两项必须携带同一非空 `module_signer_revoked_disabled_audit_event_id/payload_sha256/chain_hash_sha256`，其中 payload digest 是 event after exact JCS 的 direct SHA-256、chain digest 由既有审计链重算；旧 source entry 的 peer 只能取 after `recovery_config_item_id`，recovery entry 的 peer 只能取 after `previous_source_config_item_id`，两边 config package/item id、accepted event 也须逐字命中。非收容项这四个 recovery/audit Option 全部为 null；缺事件、多事件、before 前像缺失、审计链或任一 object-version/id/digest/time/reason/projection 漂移均不得认证为 PASS。

摘要不能代替首次接受时的 bundle bytes。每个 accepted digest 必须先从产品唯一不可变根 `C:\ProgramData\EnterprisePlatform\evidence\license-trust-bundles\<64-lowerhex>.p7b` 以 safe handle 读出；该根 owner SYSTEM、DACL PROTECTED，显式 inheritable allow ACE exact 为 SYSTEM/Administrators/`NT SERVICE\ep-ops` FullControl 与 `NT SERVICE\ep-core`/`NT SERVICE\ep-worker` 的 `FILE_GENERIC_READ|FILE_TRAVERSE|READ_CONTROL|SYNCHRONIZE`，其余无 ACE。文件只可 CREATE_NEW；同名只接受 exact bytes 相等，必须 flush/close/readback，拒绝覆盖、截短、删除、UNC/device/reparse/ADS/hardlink/8.3/case drift，并进入备份 exact-set。Stage 14b 将每个被 entry 引用的 unique bundle exact-copy 到上述 run 根的 `accepted-trust-bundles/<digest>.p7b`，要求 filename/ref末段、entry accepted digest、运行根文件 digest、Stage 14 copy digest 与 acceptance audit 五者相等；引用缺失、未引用多余文件或 validation bundle 冒充 accepted bundle均失败。

lifecycle manifest 的十项按 enum 顺序逐一对应 F-56 §8 第 1 至 10 项，不得拆并、缺失、重复或增加第十一项：许可时间/四态；续期与 current-signer 恢复；special `.epcfg`/两层签名；用量与 scope；模块状态边/依赖/排空；停用保留与再启用；special 包限制及 ServerAdmin import/只读审批；Restricted effect 矩阵；F-55 entitlement projection；PostgreSQL 终态。其 `deployment_manifest_evidence_ref/sha256` 与 `initial_governance_evidence_ref/sha256` 是两个必填 typed child，必须逐字命中本节固定 ref、exact bytes 与同 run/deployment/build/window；前者还必须验证独立 CMS 与 `DEPLOYMENT_MANIFEST_EVIDENCE_V1` sidecar，后者按无 sidecar 规则由 lifecycle digest 传递绑定。缺任一 child、把 receipt 本身误当 child、出现 receipt sidecar 或把 child 提升为共同 gate 第五个顶层 code 均失败。第一项的 assertion registry 必须精确包含 `TrustedClockV1` 在数据库连接后/public readiness 前以持久证据与 `system_utc_at_start` 建立 process anchor、OS monotonic 同进程不降，以及每个 special 推进点与 job-worker 目标间隔不超过 240 秒在同一 license advisory lock 内先按 deployment+240-second slot 耐久键查 checkpoint：零行只追加首行，已有一行只验真并复用，绝不 UPDATE 或追加第二行；current 的 `last_trusted_at` 另以 CAS 单调推进。还必须覆盖 checkpoint 与续期竞态串行重算、回拨后崩溃重启的未持久窗口严格小于 300 秒、连续 uptime 区间相邻成功 checkpoint 的 `trusted_now` 差值大于 300 秒即失败（小于等于 300 秒通过，保留 60 秒调度预算）、slot 映射/单值/hash-chain 与 trajectory 告警，以及错误前跳一经持久化只能从 Stage 14 可信备份完整恢复的正反例；不得用 daily-only job 或普通 wall clock 样例替代。每个 case file 的 `test_binary_count/command_count` 均为 1..=32，`assertion_count>0`、`passed_count=assertion_count`、`failed_count=0`、`aggregate_exit_code=0`；binary/command/fixture/assertion registry、环境事实与执行 transcript 都必须从本次待发布 build 的实际 run 重算。manifest entry 的 case/ref/digest 必须与 fixed case 文件三者相等并同 run/deployment/build/window；case file 不能只是 stdout 摘要或人工 PASS。F-56 §8 第 11 项由随后 admission 两 report 与 trust-rotation report共同覆盖，因此这四个顶层 evidence code 已是共同 gate 的完整 exact roster，不存在未登记的第五种证据。

其中 `license_time_and_status` 的 checkpoint 不能只报“240 秒 cadence 配置过”。collector 必须枚举并重算 append-only audit 中 `action=LICENSE_TRUSTED_TIME_CHECKPOINT` 的 exact after payload `{schema_version:1,purpose:"EP-LICENSE-TRUSTED-TIME-CHECKPOINT-V1",deployment_id,slot_utc,trusted_now,current_grant_id}`；这是 F-56 具名 typed audit ABI，`schema_version` 只接受 JSON number `1`，Stage 3 generic numeric-string fallback 不适用，string `"1"` 必须在 lifecycle negative fixture 中非零拒绝。`ensure_checkpoint` 在 special 业务 mutation 之前、持有同一 exclusive `platform-license-current` lock 的入口处只捕获一次 `trusted_now` 与当时唯一 current id/null，并由这一次 capture 同时派生 `slot_utc` 与候选 audit payload；terminal batch INSERT 只能消费该 immutable capture，禁止在 grant 换槽、撤销、模块状态或 `last_trusted_at` 已变化后重算 current id、时间或 slot。`slot_utc` 恰为 `floor(unix_seconds(captured_trusted_now)/240)*240` 转回 canonical RFC3339 UTC whole-second，耐久键恰为 `license-trusted-time:v1:<lowercase-deployment-id>:<slot_utc>`。checkpoint 是 append-only，绝不 UPDATE：同 slot 零行时本次事务追加恰一行；同 slot 已有一行时只验证其 action/purpose/deployment/slot、payload 与 audit hash-chain并复用，后续动作自己的 `trusted_now/current id` 可以随合法 current 推进而不同，绝不改写首行或追加第二行。current 存在时每个推进动作仍独立 CAS 单调提高 `last_trusted_at`；只有首次建 slot 时该 CAS 与新 checkpoint 同事务，故 `last_trusted_at` 可以高于本 slot checkpoint 的首次值。job-worker 调度目标间隔固定不超过 240 秒；collector 按服务 uptime 证明相邻成功 checkpoint 的 `trusted_now` 差值大于 300 秒即失败、小于等于 300 秒通过，240-second slot 为同一规则的一部分而非五分钟桶别名。collector 重算 slot exact-set/映射/单值、相邻 slot payload `trusted_now` 严格单调、startup `process_anchor_utc=max(bootstrap committed_at,current/history issued_at,accepted revocation issued_at,last_trusted_at,有效 checkpoint,system_utc_at_start)` 与服务 uptime；该观测门限不宣称 NTP 或 TPM。任一同 slot 多行/原行漂移、slot 映射错误、跨 slot 倒退、连续 uptime 区间差值>300 秒、terminal batch 二次捕获/后置重算、creation-time current id/锁断链、CAS 非单调、进程内下降、崩溃未持久窗口≥300 秒或错误前跳未经可信整库+审计恢复，都必须有具名 nonzero fixture，并使 lifecycle 与 common gate 非 PASS。

每条新 checkpoint 还须逐列命中完整 envelope：治理法人、同法人 SYSTEM actor、null device、`object_type='platform.license_trusted_time'`、`object_id=deployment_id`、null object version/before/reason/approval/reauth、system client、`occurred_at=captured_trusted_now` 与 exact after；链列只由 AuditWriter 重算。同 slot 复用不得出现新 event id。任一默认 object、数据库当前时间或 payload-only 验证均失败。

唯一运行期例外只供明确的 development/test profile：数据库同时满足零法人、零 current grant、零 bootstrap/initial-governance receipt 时，core 可在不创建 checkpoint 的情况下进入受限 readiness，但 `LicenseStatus/LicenseRestrictionReason` 固定为 `RESTRICTED/NO_CURRENT_GRANT`，job-worker 必须 dormant，不能领取业务/自动化任务、推进 special 状态或伪造 checkpoint。该例外不能出现在 production profile；生产 readiness 永远要求已验证 bootstrap/initial-governance 与当桶 append-only readiness checkpoint。Stage 14 collector 必须把零法人 bootstrap-less 形状固定判为 non-production-only：它没有 `InitialGovernanceEvidenceV1`、没有 production readiness checkpoint，因而永远不得生成/接受 `RG-LICENSE-MODULE-LIFECYCLE-GREEN` 的 result、任何其他 production Stage14 gate PASS 或发布证据包；把开发 readiness 当发布证据是具名非零负例。

`special_package_envelope_and_signature` 与 `postgresql_terminal_shape` 的 assertion/negative registry 还必须封闭下列五组，不得以一条 smoke test代替：

- 在 signature/hash 已通过后、任何 PostgreSQL 写入前，对所有会落入 `text/jsonb` 的 JSON/TOML String 解码值递归拒绝 U+0000；代表性 fixture 至少覆盖 manifest `package_no/name/signer_subject`、item `item_code`、grant `license_no/issued_to`、module `package_code/reason` 与 terminal source projection。每项固定返回 `PLATFORM.CONFIG_PACKAGE.SPECIAL_ITEM_SHAPE_INVALID`、package/item/audit/file projection 零写；尚未形成 typed DTO 的 ZIP/UTF-8/TOML 容器错误仍走既有容器/载荷码，禁止用 PostgreSQL SQLSTATE 或 authz 码替代。
- special import/DRAFT 的 `approval_legal_entity_id` 必须为 NULL；command 层派生 `governance_context_id` 的唯一优先序是：先取唯一 current grant 的治理法人；无 current 时取已完整验真的 initial-governance bootstrap；两者都无时，只在**首张 LICENSE_GRANT 的同一首次发行事务**取该候选 payload 的治理法人，其他命令/包种/事务绝不允许 candidate fallback。派生后先按 session authorization 判定，request header/UI/ServerAdmin/环境变量不得覆盖。submit 事务才写 `approval_legal_entity_id`，`PENDING_APPROVAL` 及以后必须非空且逐字等于上述唯一治理来源。DRAFT 非空、PENDING+ 为空/错法人、current/bootstrap/candidate 优先序被颠倒、非首发事务使用 candidate、来源不唯一或执行时漂移都固定为 `PLATFORM.CONFIG_PACKAGE.SPECIAL_ITEM_SHAPE_INVALID` 与零状态推进；纯 session/action 未授权仍保持既有 `PLATFORM.AUTHZ.*` 分层，不能伪装成 shape error。
- `pg_catalog` 必须通过 normalized `pg_get_constraintdef` 证明 `data_keys` 的 enum/purpose-algorithm 约束、`ck_data_keys_version` 恰含 `version>=1 AND version<=65535`、`ck_data_keys_state_shape` 四态空值形状、`activated_at<=retiring_at<=retired_at<=destroyed_at` 相应时间前缀、`wrap_kek_version>=1 AND wrap_kek_version<=2147483647` 与 `octet_length(wrapped_key)>0` 全部真实存在；KEK 当前版本所在 SQL `integer` 列也必须有同一 `1..=2147483647` CHECK，并与 Rust `u32` 的 checked conversion 一致。transaction fixture 对每一态的缺/多时间、倒序时间、data-key version 的 0/65536/前导零 ref、65535 后再轮换、KEK/wrap version 的 0/负值/2147483648/转换溢出、空 wrapped bytes 都必须拒绝且零写，并以 data-key version=1/65535 与 KEK version=1/2147483647 边界正例逐字核对 EPC1 2-byte header、附件/TOTP `DataKeyRef` u16 wire、DTO 与数据库行。fresh install 从零与 upgrade 后的既有库都运行同一 pg-catalog/写入负矩阵；任一仅 Rust 校验、`DataKeyRef` 被放宽为大于 u16、旧库漏 CHECK、EPC1/ref/row 不等、rotation 回绕或负例成功都阻止迁移/发布。
- 首装角色/权限只接受两份 exact pair registry：`F56_CONFIG_OPERATOR` 恰八对 `lowcode.config_package.view:VIEW|lowcode.config_package.import:CREATE|lowcode.config_package.autotest:UPDATE|lowcode.config_package.submit:SUBMIT|lowcode.config_package.sign:UPDATE|lowcode.config_release.view:VIEW|lowcode.config_release.submit:SUBMIT|lowcode.config_release.execute:UPDATE`；`SECURITY_ADMIN` 恰两对 `lowcode.config_package.view:VIEW|lowcode.config_package.approve:APPROVE`。按 `(permission_code UTF-8 bytes,action wire)` 排序去重，零额外/缺失/替代 action；数据库 role/permission/binding projection、bootstrap body/receipt、`CONFIG_RELEASE` chain 与两名不同用户逐项相等，CONFIG_OPERATOR 不能 APPROVE、SECURITY_ADMIN 不能 import/sign/execute。
- strict multipart config-package import 对已认证的 Windows、macOS 与 ServerAdmin 三入口使用同一个 handler/parser；deployment 为 `Restricted/NO_CURRENT_GRANT` 时，admission 只允许 exact `.epcfg` 在完整内外签名、来源、治理与 action 全链确认后映射为首张 `LICENSE_GRANT` 的首次发行恢复，普通附件上传 API 绝不因此开放。Stage 14 recovery/disabled matrix 必须至少含 Windows 与 macOS 的零-current multipart 首发成功、ServerAdmin 同 handler 等价成功，以及普通 config package、任一 MODULE_PACKAGE 非 `DISABLE` action、通用 attachment upload 和伪装 item kind 均稳定返回 `PLATFORM.LICENSE.RESTRICTED` 且 package/item/audit/file/attachment 零写；已有 current 时不得继续借首发例外，未经 strict 确认的请求在读取/落业务 payload 前失败关闭。

registry report 的两个 count 均须在 `1..=65535`，core/worker 各自 `binding_registry_sha256 == actual_operation_registry_sha256`，并由同一待发布 core/worker binary 的编译期 registry、`xtask` exact-set 与 Blocking selfcheck 三方重算，证明 HTTP/MCP route 及 job、event、approval-owner、outbound-operation 零缺失、零额外、零重复。negative report 的前五行必须同时具有非零 xtask 与 Blocking selfcheck exit、runtime/error 两字段为空；后三行必须具有非零 runtime exit、`observed_error_code=PLATFORM.LICENSE.RESTRICTED`，其余两个 exit 可空但不得为零。缺行、重复、额外 case、未执行 probe 或任一选填组形状不符均失败。

trust manifest/chunk 由 Stage 14b 直接枚举数据库中全部 `RELEASED LICENSE_GRANT`（grant 与 revocation）和全部 `RELEASED MODULE_PACKAGE` special item 生成，`released_special_item_registry_sha256` 是该 DB exact-set 的排序 strict-JCS digest；不得从 current projection 倒推历史。第一次 RELEASE 后所有这些 special source 永久保持 RELEASED，因此多份 RELEASED 是完整历史而不是重复 current；current/history 只从 license/module projection交叉得出，扫描到任一 `SUPERSEDED` special 必须直接非零失败。`ordinary_or_unreleased_null_registry_sha256/count` 同时覆盖全部普通 item 与从未 RELEASE 的合法未发布 special item并证明其 `accepted_trust_bundle_sha256 IS NULL`，不得吞入 SUPERSEDED special。`total_entry_count` 与跨块累计计数用 checked `u64`；`total_chunks:u32` 必须恰为 `ceil(total_entry_count/512)`（零 entry 时为 0），`chunk_no:u32` 从 1 连续到 total，文件名与 ref 中均是十位 zero-pad 十进制，禁止 0、缺号、重复、超过 `u32::MAX`、非十位或前导格式漂移。manifest ref 与 chunk header/digest/`entry_count:u16` 逐项相等；零 entry 当且仅当 chunks 为空，否则每块 1..=512、末块也不得为空，且不设 256-chunk 或其他历史条目业务上限。chunk entries 跨块按 `(artifact_kind wire,config_package_id UUID bytes,config_item_id UUID bytes)` 全局升序且组合唯一，并与 DB exact-set 零缺/多；`sum(entry_count)=total_entry_count` 必须 checked 无溢出。

每个 entry 的 accepted digest 必须等于 source item 的不可改 32-byte 值，validation digest 必须等于 manifest 顶层当前只读 `license-roots.p7b` digest，grant source 还须等于 grant 行 `trust_bundle_sha256`；其 accepted bundle ref/exact bytes、唯一接受审计 payload/hash-chain 与 inner/outer chain digest 还必须按上段逐项相等。`origin_config_item_id` 是 inner artifact 首次合法引入的唯一 RELEASED item：grant/revocation 及新 INSTALL/UPGRADE 等于本 entry 的 `config_item_id`，ENABLE/DISABLE/ROLLBACK_VERSION 则指向携带 exact same inner 的唯一 RELEASED INSTALL/UPGRADE origin；不存在、多于一个或内容不等都失败。`accepted_inner_signer_subject` 与 `source_outer_signer_subject` 必须分别从 origin inner CMS 与本 entry source outer CMS 重算且均为 `spki-sha256:<64-lowerhex>`，两个 state 也必须分别按当前 validation bundle 求值，禁止以一个 `signer_state`、聚合 subject 或 source item 状态替代。state 不是 leaf-only：对 inner/outer 各自唯一链的 leaf 加全部 non-anchor intermediate 统一求值，并先为两层所有实际 issuer 完成上段 global-highest-then-cover registry；任一 issuer 缺失、最高号过期/尚未生效/同号冲突即整项 `UNTRUSTED`，此时不得扫描任何 serial、不得产出 revoked-layer 子集或进入窄恢复。只有全部 issuer CRL 前置成功后，任一 non-anchor serial 命中才使相应层为 `REVOKED`；零命中且全部 non-anchor 当前有效才为 `ACTIVE`，零命中、全部在 signed-time 有效、origin/首次接受 exact 证据证明当时整层为 ACTIVE、当前至少一张 non-anchor 已过期且其余没有 not-yet-valid 才为 `RETIRED`。anchor 必须在 signed-time 有效、自签/CA/KeyUsage/critical-extension 合法；它在 `trusted_now` 后过期本身不把链降为 RETIRED，但从当前 bundle 移除/替换、形成零链/多链或任一结构/约束失败都为 `UNTRUSTED`。`UNTRUSTED` 不进入三值 wire enum，必须使证据生成失败。inner CMS `signingTime` 必须与 payload `issued_at`、outer CMS `signingTime` 必须与 manifest `signed_at` 各自语义上等于同一 UTC whole-second instant，且使用规范 DER `UTCTime|GeneralizedTime`；证书/CRL/SignerInfo 的 AlgorithmIdentifier 及参数都严格采用本节签名部署清单的 ECDSA-P256 或 RSA-PSS-SHA256 闭集。两个 CRL registry digest 分别绑定完整候选集与在全部 issuer 前置成功后生成的实际 highest-covering 选择集。RETIRED 只允许复验具有上述首次 ACTIVE exact 证据的既有 accepted current/history item，不能接受本次新 import/release。entry 的 `current_projection_kind/current_projection_sha256/current_module_install_state` 按 kind 合法成组：`CURRENT_GRANT|CURRENT_REVOCATION` 的 digest 必须分别等于 manifest 对应 Option 且 module state 为空；`CURRENT_MODULE` 的 digest 必须逐项命中由 15 个 `(module_code,current_projection_sha256)` 排序对重算的 `current_module_projection_registry_sha256`，不得拿单行 digest 与整表 digest 直接比较；非 current 的这三字段全空。`recovery_peer_config_item_id` 独立只允许下述两个模块收容 result 成组互指。正常既有 current 与非恢复 history 的两层 state 可各为 `ACTIVE|RETIRED` 且 result=`TRUSTED`；任何本次新 grant/revocation candidate 或新 INSTALL/UPGRADE 的 inner/outer 必须均为 `ACTIVE`。`HISTORICAL_SIGNER_REVOKED` 仅允许非 current 对象、两层至少一层为 `REVOKED`、每个未撤销层只能为 `ACTIVE|RETIRED`、不得有 UNTRUSTED，且 `revoked_layer_crl_registry_sha256=Some` 覆盖一至两层全部撤销证据，accepted/source/payload/digest/signature/首次接受 bundle bytes与审计全自洽；这是正确的 accepted-containment **包含态**，必须从 purchased、rollback 与正向证明排除，但其本身不是 FAIL，也不阻断一个独立 `TRUSTED` current。独立 current 的两层可以是 ACTIVE，或是非撤销 RETIRED 且具有同 origin/同 chain 的首次 ACTIVE exact 证据；其他共同 predicate 也全部通过时，common gate 可以 PASS。只有历史分类与实际 CRL containment 不等、缺失/错误 origin、把两层合并、正常 current 含 REVOKED/UNTRUSTED、新 candidate 任一层非 ACTIVE、撤销层漏 CRL、非撤销层不属 ACTIVE|RETIRED，或任何其他历史断链/漂移/损坏才必须命中 `trust_rotation_negative_matrix_sha256` 的具名非零 fixture，并阻止 final trust manifest/common result。

license inner/source-outer 的 accepted 与 validation path 还必须逐字执行 F-56 首版证书/CRL extension profile，不能交给不同 PKI 库默认解释。所有证书无论 critical 与否都拒绝 `nameConstraints|certificatePolicies|policyMappings|policyConstraints|inhibitAnyPolicy`。leaf extension exact-set 要求 noncritical SKI、noncritical AKI（仅 keyIdentifier 且等于 issuer SKI）、critical KU（唯一 digitalSignature）、noncritical EKU（唯一 codeSigning），BasicConstraints 只可缺省或 critical `CA=false,pathLen absent`；CA exact-set 要求 noncritical SKI/AKI、critical `CA=true` BasicConstraints（可空/非负 pathLen 且实际 enforce）和 critical KU（唯一 keyCertSign+cRLSign），禁止 EKU 与任何未列 extension。CRL extension exact-set 只允许 required noncritical AKI keyIdentifier 与 required noncritical CRLNumber，必须有 nextUpdate；禁止 IDP/delta/freshest、indirect、任何 revoked-entry extension/reason/removeFromCRL。Stage 14 fixture 必须在开发 verifier 前提供一套合法 whole-chain/base-CRL golden bytes，并逐一变异未列 extension、critical bit、KU/EKU、AKI/pathLen/CRL extension；实现 crate 不在文档内预选，但实际解析/验证依赖必须被当前 `Cargo.lock` 与产品 SBOM 精确固定，任何依赖升级都重跑全部 golden/negative 与共同 gate。

许可 current 允许零或一条：零 current 的 `current_grant_projection_sha256=None`、`current_revocation_projection_sha256=None` 与 `current_license_status/current_license_restriction_reason=RESTRICTED/NO_CURRENT_GRANT` 必须同时成立，多 current 失败；正常既有 current 的 inner/source outer 各可为 ACTIVE|RETIRED，但每个 RETIRED 都必须带 origin/首次接受时整条链 ACTIVE 的 exact 证明；有效三态 reason 为空、Restricted 恰有 F-56 单一 reason。current grant/revocation 的 CRL 恢复证据必须同时包含旧 current 后转成的 history entry 与新 current entry：旧 entry 的 inner/source outer 各自枚举 ACTIVE|RETIRED|REVOKED、至少一层 REVOKED、不得有 UNTRUSTED，且 row/source/payload/digest/signature 仍逐字自洽；新 grant 的 inner/source outer 必须均为 ACTIVE、同 deployment/governance，并逐字 direct-supersede 旧 current。任一非 CRL 失败、断链、双层状态被合并、候选一层非 ACTIVE 或 supersedes 漂移都不得借恢复链。模块 CRL 收容必须由一对互指 entry 唯一证明：旧 current package source 是 `CURRENT_MODULE_SIGNER_REVOKED_CONTAINED`，要求 `current_projection_kind=CURRENT_MODULE`、accepted inner/source outer 各自枚举 ACTIVE|RETIRED|REVOKED、至少一层 REVOKED、不得有 UNTRUSTED、安装态 `INSTALLED_DISABLED`、撤销层 CRL registry 与 `recovery_peer_config_item_id` 非空；新 recovery item 是 `MODULE_SIGNER_REVOKED_DISABLE_AUTHORIZATION`，要求 `module_action=DISABLE`、无 current projection、source outer=ACTIVE、accepted inner 与旧 entry 的 origin/subject/state/exact bytes 全相等、peer 反向指回旧 source，并由旧 entry 的 CRL registry 精确覆盖一层或两层全部撤销证据。两项 peer 与 current after row 只能由上段唯一 `MODULE_SIGNER_REVOKED_DISABLED` 事件、其审计 hash chain、before/after projection digest 和 accepted event 派生；after row 的 source 只与旧 inner/package 的 previous source 相等，`last_transition_reason` 必须等于 recovery item reason，禁止把 recovery item 写成 current source 或按相同 package/inner 推断 peer。两项其余 source/payload/digest/signature 均须自洽；它们只证明安全停用与 ACTIVE outer 授权，不证明任一已撤销层受信，也不得计入 purchased/rollback/正向模块证明。current module 仍 enabled、该 action 缺失/重复/漂移、该互指对缺失/不唯一、存在但非 `TRUSTED` 的正常 current grant/revocation、普通/未发布接受摘要非空，或 15 行 ServerAdmin `package_trust_status` 与现场重算 `NOT_INSTALLED|TRUSTED|SIGNER_REVOKED|INVALID` 不等时，均不得产生共同 PASS。停用后用全新 ACTIVE inner+outer、严格更高 semver 的 `UPGRADE` 替换时，新 item 必须是 `TRUSTED`；旧 package 与 recovery item 均只保留为隔离/处置历史。special outer 与 inner 都只用 `license-roots.p7b`，普通 config package outer 仍只用部署 KMS。三份顶层报告、全部 chunk、gate index/result 必须同 run/deployment/build/closed window；缺任一 code、registry/trust digest 不等、chunk 残缺、负例未真实运行或把诊断失败报告包装成 PASS，都不得产生共同 gate result。

collector 只能先在最终根的同父目录建立 `<lowercase-gate-code>.staging.<lowercase-uuidv7>`，用 `CREATE_NEW` 写四个文件，逐个 close、handle readback、strict parse、digest 与签名复验后，才在 final 目录不存在时原子 rename 发布；final 已存在必须失败并另开新 run，不得覆盖。任何 predicate 未通过、进程中断、文件缺失或校验失败都必须非零且不得产出或接受 `Stage14GateResultV1`：可控失败立即关闭句柄并清理 staging；崩溃遗留 staging 在下次运行先隔离/清理，且永远不能被 resolver 或打包器读取。`Stage14GateOutcomeV1` 没有 `FAIL|N/A|DISABLED|UNKNOWN` wire 值。第 8.7 节前九项永久 gate 与六项 F-55 disposition 中每一个 `RequiredPass` 都必须按此 ABI 产生同 run/deployment/build 的 exact index+result+两份 sidecar；`DisabledEvidence|NotInBuild` 禁止发布或接受对应 result，也禁止遗留孤儿 final index/sidecar，出现即整包失败。由此 release verifier 逐份验签，不再以“七个文件名/七个散落 SHA-256 数量正确”替代证据闭包。

`RG-PLAINTEXT-SECRETS-ABSENT` 的输入不接受人工布尔值。原始 `ep-secretctl` receipt 是 bounded strict-JCS，字段闭集如下；它继续遵守 ADR-0007 的“只含 deployment/run/tool binary digest、ref、recipient、envelope digest、status”限制，`receipt_kind/schema_version` 只是解析域标签，不携带路径、明文或明文摘要。fresh 分支只能由 `bootstrap/put/verify` 产生 `FRESH_INSTALL` 与逐项 `VERIFIED`，legacy 分支只能由 `migrate/finalize-migration/verify` 产生 `LEGACY_MIGRATION` 与逐项 `MIGRATED_FINALIZED_VERIFIED`；entries 按 `(recipient UTF-8 bytes,canonical_secret_ref UTF-8 bytes)` 排序去重且非空。collector 不接收 `--receipt-kind`，只从验签 receipt 的同质 kind/status 推导分支，混合 kind、空 entries 或 migration 未 finalize 均失败。

```rust
pub enum SecretCtlReceiptKindV1 { FreshInstall, LegacyMigration }
pub enum SecretCtlReceiptStatusV1 { Verified, MigratedFinalizedVerified }

pub struct SecretCtlReceiptEntryV1 {
    pub canonical_secret_ref: SecretRef,
    pub recipient: SecretRecipient,
    pub envelope_sha256: Sha256Digest,
    pub status: SecretCtlReceiptStatusV1,
}

pub struct SecretCtlReceiptV1 {
    pub schema_version: u16, // exact 1
    pub receipt_kind: SecretCtlReceiptKindV1,
    pub deployment_id: Uuid,
    pub run_id: Uuid,
    pub tool_binary_sha256: Sha256Digest,
    pub entries: Vec<SecretCtlReceiptEntryV1>,
}

pub enum SecretProviderV1 { Kms }
pub enum SecretKmsBackendV1 { Builtin, Hsm }

pub struct SecretTerminalEvidenceV1 {
    pub schema_version: u16, // exact 1
    pub stage14_run_id: Uuid,
    pub deployment_id: Uuid,
    pub product_build_sha256: Sha256Digest,
    pub observed_at: DateTime<Utc>,
    pub receipt_kind: SecretCtlReceiptKindV1,
    pub provider: SecretProviderV1,
    pub kms_backend: SecretKmsBackendV1,
    pub normalized_nonsecret_config_sha256: Sha256Digest,
    pub ep_secretctl_artifact_sha256: Sha256Digest,
    pub ep_secretctl_sbom_component_sha256: Sha256Digest,
    pub declared_ref_inventory_sha256: Sha256Digest,
    pub receipt_set_sha256: Sha256Digest,
    pub receipt_count: u32,
    pub eps1_inventory_sha256: Sha256Digest,
    pub eps1_object_count: u32,
    pub bootstrap_inventory_sha256: Sha256Digest,
    pub bootstrap_probe_evidence_sha256: Sha256Digest,
    pub hsm_no_fallback_probe_evidence_sha256: Option<Sha256Digest>,
    pub legacy_object_count: u32,
    pub quarantine_object_count: u32,
    pub staging_object_count: u32,
    pub negative_fixture_report_sha256: Sha256Digest,
}

pub enum SecretNegativeFixtureV1 {
    ProviderFileConfig,
    LegacyProviderLinked,
    NonEps1Envelope,
    LegacyQuarantineStagingResidual,
    BootstrapOrHsmFallback,
    ReceiptBindingInvalid,
}

pub struct SecretNegativeFixtureResultV1 {
    pub fixture: SecretNegativeFixtureV1,
    pub exit_code: NonZeroI32,
    pub stdout_sha256: Sha256Digest,
    pub stderr_sha256: Sha256Digest,
}
```

receipt 文件根固定为 `target/release-evidence/secrets/<lowercase-stage14-run-id>/receipts/`，文件名为其 strict-JCS bytes 的 SHA-256 lowerhex 加 `.receipt.v1.jcs`；terminal evidence 与六负例报告分别固定为同级 `secret-terminal-evidence.v1.jcs`、`secret-negative-fixtures.v1.jcs`。每一文件都带同名追加 `.sig.jcs` 的 `SecretEvidenceSignatureV1`，字段恰为 `schema_version,purpose,product_build_sha256,evidence_sha256,key_ref,key_version,signer_subject,signature_p1363_b64url`，purpose 闭集恰为 `SECRETCTL_RECEIPT_V1|SECRET_TERMINAL_EVIDENCE_V1|SECRET_NEGATIVE_FIXTURES_V1`；preimage 恰为 `SHA-256("EP-SECRET-RELEASE-EVIDENCE-V1\0" || ASCII(purpose) || 0x00 || product_build_sha256[32] || evidence_sha256[32])`，签名编码、low-S 与 key 状态规则同下述 Stage 14 evidence sidecar。每份 `SecretCtlReceiptV1.run_id` 必须逐字等于 `SecretTerminalEvidenceV1.stage14_run_id`；terminal evidence、全部 receipt、负例报告、effective config、产品 manifest/SBOM 与当前完整 deployment revision 必须同一 `product_build_sha256/deployment_id`，`observed_at` 位于同一 closed run window；不存在跨 run、跨 build 或跨 deployment 拼包。

Stage 14b collector 只从已安装服务 SCM `ImagePath` 所指同一待验配置链、待发布 `MANIFEST.sha256`/CycloneDX SBOM/生产 Authenticode PE、固定根 `C:\EP\secrets\`、`C:\EP\kms\bootstrap\` 与 `C:\EP\secrets-legacy-quarantine\`、原始 receipt 和现场探针生成上述 projection。`declared_ref_inventory_sha256` 是 strict loader 导出的全部规范非秘密 `secret://` 引用及其声明 recipient exact-set 的排序 JCS digest；`eps1_inventory_sha256` 是通过 handle 打开、拒绝 reparse/ADS/hardlink 后枚举的 `(recipient,canonical_secret_ref,envelope_sha256)` 排序 JCS digest，两者与 receipt entries 必须 exact-set 相等。`C:\EP\secrets\` 下只允许 `<recipient>\<sha256(canonical-secret-ref) lowerhex>.eps1`，每个对象均按 ADR-0007 完整解析、AAD 验证和真实解封；任何其他路径、magic、版本、算法、flag、尾随字节或解封失败都计入 legacy/staging 并使 gate 失败。quarantine 根必须不存在或为空；同目录 CREATE_NEW 暂存句柄/文件必须为零。production cargo feature graph、所有常驻产品 PE/伴随 metadata 与产品 SBOM 中，`legacy-file` feature、`FileSecretProvider` 类型/符号/组件/property 任一命中即失败；`ep-secretctl` 自身必须命中产品 manifest、SBOM component、生产 Authenticode 与两次可复现构建摘要。

`kms_backend=BUILTIN` 时，`bootstrap_inventory_sha256` 必须覆盖六个固定 recipient 各自关闭继承、ACE 只含该 recipient/SYSTEM/Administrators 的 DPAPI machine-scope KEK，交叉 recipient/blob/entropy 六类解封全部失败，`hsm_no_fallback_probe_evidence_sha256=None`；`kms_backend=HSM` 时每 recipient 的 nonextractable AES-256 object 与 DPAPI PIN bootstrap 均须现场 probe，强制 module/slot/PIN/object 各一项不可用时进程均以 78 退出、数据库连接次数为 0、builtin/file 解封调用次数为 0，并令该字段为这些结果的排序 JCS digest。fresh receipt 必须逐项匹配当前 declared/EPS1 inventory；migration receipt 还必须证明每个 legacy 输入已逐项变成相同 ref/recipient 的 EPS1、`finalize-migration` 已完成且 legacy/quarantine/staging 三计数均为 0。两分支都必须有 `receipt_count>0`、`provider=KMS`、三份 inventory/receipt digest 重算相等；不允许用空目录或零条 receipt 伪造终态。

六个负例 exact-set 恰为：`ProviderFileConfig` 把 effective provider 改为 `file`；`LegacyProviderLinked` 向常驻产品 feature graph、PE metadata 与 SBOM 各注入一次 `legacy-file/FileSecretProvider`；`NonEps1Envelope` 在规范目标路径放入一份 `EPC1`/错误 EPS1 magic 的对象；`LegacyQuarantineStagingResidual` 同时留下一个 legacy 路径对象、一棵 quarantine run 目录和一个未发布 CREATE_NEW staging 文件；`BootstrapOrHsmFallback` 对 BUILTIN 使用错 recipient/entropy，对 HSM 强制 module/slot/PIN/object 不可用并监测任何 builtin/file fallback；`ReceiptBindingInvalid` 只把一份有效 receipt 的 `product_build_sha256` sidecar binding 改成另一 build。`secret-negative-fixtures.v1.jcs` 必须按 enum 顺序恰有六行且六个 `exit_code` 均非零；缺行、重复、额外 fixture、未实际执行、任一退出 0 或把负例写成 PASS 文本都令永远适用的 gate 失败。

六项 F-55 gate 的 applicability 不再依赖未定义的“既有表示”，也不接受命令行 `--ai-purchased`、`--enabled`、环境变量自报或自由 JSON。唯一输入是下列三份 strict projection、当前完整 deployment revision 与禁用态报告；所有 JCS 均≤1048576 bytes、无 BOM UTF-8、RFC 8785 canonical，拒绝 unknown/duplicate field、非规范 number、非小写 UUID/hex 与越界字符串。F-56 source payload 的 `license_no` 1..128 与 `issued_to` 1..256 UTF-8 bytes 只在 source parser 验证；Stage 14 summary 只保留 `license_no_sha256`，不重复序列化二者。所有 F-56 派生的 `signer_subject` 虽保留字段名，safe wire 必须逐字为 `spki-sha256:<64-lowerhex>`；人类可读 X.509 名称不得进入证据身份比较。`key_ref` 只接受下段固定语法，ServerAdmin `bom-ref` 只接受固定字面量，其余 String 都由枚举或 opaque-ref 文法封闭；不得由实现方另选长度。`Sha256Digest` JSON wire 一律是 `[0-9a-f]{64}`，数据库 bytea 才使用对应 32 raw bytes，拒绝大写、`0x`、base64 或 array。三份 projection、applicability 和禁用态报告各带一个相邻 `Stage14EvidenceSignatureV1`，字段恰为 `schema_version,purpose,product_build_sha256,evidence_sha256,key_ref,key_version,signer_subject,signature_p1363_b64url`。该 sidecar 的全局 purpose 闭集恰为 `F55_ENTITLEMENT_SNAPSHOT_V1|F55_EFFECTIVE_CONFIG_SNAPSHOT_V1|F55_PRODUCT_MANIFEST_PROJECTION_V1|F55_APPLICABILITY_V1|F55_DISABLED_MODE_V1|STAGE14_GATE_EVIDENCE_INDEX_V1|STAGE14_GATE_RESULT_V1|DEPLOYMENT_MANIFEST_EVIDENCE_V1`；前五项是 F-55 context 子集，中间两项只签第 8.7 节通用 gate index/result，最后一项只签本节 `DeploymentManifestEvidenceV1`，三组不得互换。签名 preimage 恰为 `SHA-256("EP-STAGE14-EVIDENCE-V1\0" || ASCII(purpose) || 0x00 || product_build_sha256[32] || evidence_sha256[32])`；签名字符串只接受 canonical base64url-no-pad、解码恰 64-byte low-S P-256 P1363，key 必须是本次 deployment/build 的 current 或 retired-nonrevoked Stage 14 evidence key。`InitialGovernanceEvidenceV1` 没有第九个 purpose，只能由 lifecycle manifest child digest 传递绑定。上一段 `SecretEvidenceSignatureV1` 仍是独立类型、独立 `EP-SECRET-RELEASE-EVIDENCE-V1` preimage 和独立三 purpose 闭集；本次绝不把 gate/deployment purpose 加入 secret sidecar，也不允许两种 sidecar 互验。

这些 sidecar 与上一段 secret sidecar 的签名来源也不留实现选择：生产唯一 signer 是该 deployment 的 builtin KMS 或客户 HSM，通过 F-55 carrier 已冻结的同一 deployment-key signer/key-state resolver 调用既有 `KmsBackend::sign/verify`；本用途的 key ref 唯一形状为 `kms://<builtin|hsm>/deployment/<lowercase-deployment-id>/stage14-release-evidence#<key-version>`，`key_version` 为 1..2147483647 且必须逐字等于 ref fragment。私钥不导出、不落 release-evidence 目录，`ep-release-gate` 只组装 strict JCS/digest 并请求该 signer；生产不接受 DEV/file key、Windows 任意用户证书、命令行公钥或临时生成 key。`crates/platform/obs/src/release_evidence.rs` 提供唯一 signer/verifier contract，`tools/release-gate/src/f55.rs`、`tools/release-gate/src/gate_evidence.rs` 与 `tools/release-gate/src/secrets.rs` 只消费对应 contract；current 或 retired-nonrevoked 状态、deployment/purpose/version/subject 任一不等即失败，retired key 只验证其原绑定 build/run，revoked key 立即令原证据失效。

许可证与 entitlement 的唯一来源是已批准 F-56，不再保留 F-55 私有 payload、旧短表投影或临时 CMS 解释。Stage 14a1 在 Stage 3b-2 types/runtime/table 已存在后只新增只读 `F55EntitlementEvidenceQuery::snapshot(stage14_run_id,deployment_id,product_build_sha256,observed_at)`；实现由 `ep-platform-license` 对 `platform_core.license_grants` 的 F-56 current/history exact-set 逐行重建 `LicenseArtifactPayloadV1::Grant(LicenseGrantPayloadV1)`，核内层 detached CMS、special 外层单项 `LICENSE_GRANT` 配置包、部署绑定、source FK、行/payload 逐列相等、撤销证据与零或一 current slot，并逐项命中同 run 的 `F56LicenseTrustRotationEvidenceV1`。它不新增表/列、不定义第二个 entitlement enum，也不从十五个 `ModuleCode`、`feature_flags`、配置、环境变量或人工布尔值猜购买态。AI 只认 `EntitlementCodeV1::F55LocalAi`；`EntitlementCodeV1::F55Mcp` 同时覆盖入站与出站，不存在方向许可证。AI/MCP 是平台 capability，不映射、占用或伪造第十六个 `ModuleCode`；只有它们对具体业务对象执行动作时，才继续调用该对象在编译期 registry 中既有的 owner `ModuleCode` effective-runtime gate，entitlement 与 owner-module 两条判定互不替代。

query 必须返回全部 current/history grant 的下列有界 summary；`grants` 为 0..=65535，按 `grant_id` UUID bytes 排序、去重并与数据库 exact-set 相等。`is_current=true` 当且仅当源行 `current_slot=0 && superseded_at IS NULL`，全表允许零或一条、禁止多 current；历史行必须 `current_slot IS NULL && superseded_at IS NOT NULL`。snapshot 的 `current_grant` 是显式 Option：存在唯一 current 时必须指向该 summary 的 `grant_id` 与 exact strict-JCS digest，零 current 时必须为 `None`；空表合法形状恰为 `grants=[]/current_grant=None/license_status=RESTRICTED/license_restriction_reason=NO_CURRENT_GRANT` 且四个购买/当前许可布尔值全 false。零 current 但仍有 history 时同样 `current_grant=None/RESTRICTED/NO_CURRENT_GRANT`，两个 currently-licensed=false，purchased 仍只能从 TRUSTED history 重算。`grant_source_*` 必须非空并命中同一外层单项 special package；grant 与 revocation 各自的 origin、accepted-inner subject/state、source-outer subject/state 必须逐字段等于共同 trust entry，禁止用一个 signer state 合并两层。全部 revocation id/time/source/digest/signature/subject/state/accepted-digest/trust-result/entry-digest 字段必须成组全空或全非空。每个 summary 的 `trusted_now` 必须逐字等于同一 F-56 `TrustedClockV1` 对本次 repeatable-read snapshot 求得的时刻：启动持久/system anchor、OS monotonic、同进程不降与本次 capture 所属 240-second slot 的有效 append-only checkpoint 缺一不可；checkpoint 的首行值不要求等于同 slot 后续 query 的 `trusted_now`。不得改回单取本地 wall clock。`status/restriction_reason` 从该值重算：`Restricted` 恰有一项 reason，其余三态 reason 为空。正常既有 current 的 grant/revocation 两层 state 各可为 ACTIVE|RETIRED 且 `trust_rotation_result=TRUSTED`，其中 RETIRED 必须由同 origin/同 exact chain 首次接受时 ACTIVE 的不可改证据支持；本次新 grant/revocation candidate 的两层则必须均 ACTIVE。历史 `HISTORICAL_SIGNER_REVOKED` 只有在 accepted inner 和/或 source outer 至少一层被现行完整 base/highest-covering CRL 明确命中、每个未撤销层为 ACTIVE|RETIRED、无 UNTRUSTED 且 source/payload/digest/signature 仍自洽时可保留在 summary，但不得成为 current、purchased 或正向证明。除这一窄分类外，任何坏 signature/source/chain、多 current、current/history 形状错误、payload/行漂移、层状态合并或 trust entry 缺失都令整个 snapshot 非零，不能静默跳过一行。RETIRED signer 只允许复验已有 accepted current/history item，不接受本次新 import/release；anchor 在 trusted_now 过期本身不产生 RETIRED，anchor 被移除/多链或任何 UNTRUSTED 形状都令 query 失败。

```rust
pub struct F55LicenseGrantSummaryV1 {
    pub schema_version: u16, // exact 1
    pub purpose: String, // exact "EP-F55-LICENSE-GRANT-SUMMARY-V1"
    pub grant_id: Uuid,
    pub source_row_version: u64,
    pub license_no_sha256: Sha256Digest,
    pub license_kind: LicenseKindV1,
    pub issued_at: DateTime<Utc>,
    pub valid_from: NaiveDate,
    pub valid_to: Option<NaiveDate>,
    pub legal_entity_scope: LegalEntityScopeV1,
    pub legal_entity_ids: Vec<Uuid>,
    pub supersedes_grant_id: Option<Uuid>,
    pub superseded_at: Option<DateTime<Utc>>,
    pub is_current: bool,
    pub last_trusted_at: DateTime<Utc>,
    pub payload_sha256: Sha256Digest,
    pub grant_signature_cms_sha256: Sha256Digest,
    pub grant_origin_config_item_id: Uuid,
    pub grant_accepted_inner_signer_subject: String,
    pub grant_accepted_inner_signer_state: F56TrustRotationSignerStateV1,
    pub grant_source_outer_signer_subject: String,
    pub grant_source_outer_signer_state: F56TrustRotationSignerStateV1,
    pub revoked_at: Option<DateTime<Utc>>,
    pub revocation_id: Option<Uuid>,
    pub revocation_issued_at: Option<DateTime<Utc>>,
    pub revocation_payload_sha256: Option<Sha256Digest>,
    pub revocation_signature_cms_sha256: Option<Sha256Digest>,
    pub revocation_origin_config_item_id: Option<Uuid>,
    pub revocation_accepted_inner_signer_subject: Option<String>,
    pub revocation_accepted_inner_signer_state: Option<F56TrustRotationSignerStateV1>,
    pub revocation_source_outer_signer_subject: Option<String>,
    pub revocation_source_outer_signer_state: Option<F56TrustRotationSignerStateV1>,
    pub grant_source_config_package_id: Uuid,
    pub grant_source_config_item_id: Uuid,
    pub grant_source_accepted_trust_bundle_sha256: Sha256Digest,
    pub grant_trust_rotation_result: F56TrustRotationItemResultV1,
    pub grant_trust_rotation_entry_sha256: Sha256Digest,
    pub revocation_source_config_package_id: Option<Uuid>,
    pub revocation_source_config_item_id: Option<Uuid>,
    pub revocation_source_accepted_trust_bundle_sha256: Option<Sha256Digest>,
    pub revocation_trust_rotation_result: Option<F56TrustRotationItemResultV1>,
    pub revocation_trust_rotation_entry_sha256: Option<Sha256Digest>,
    pub trusted_now: DateTime<Utc>,
    pub status: LicenseStatus, // Active|ExpiringSoon|GracePeriod|Restricted
    pub restriction_reason: Option<LicenseRestrictionReason>,
    pub entitlement_codes: Vec<EntitlementCodeV1>,
}

pub struct F55CurrentGrantSummaryRefV1 {
    pub grant_id: Uuid,
    pub grant_summary_sha256: Sha256Digest,
}

pub struct F55EntitlementSnapshotV1 {
    pub schema_version: u16, // exact 1
    pub stage14_run_id: Uuid,
    pub product_build_sha256: Sha256Digest,
    pub deployment_id: Uuid,
    pub observed_at: DateTime<Utc>,
    pub license_trust_bundle_sha256: Sha256Digest,
    pub license_trust_rotation_evidence_ref: OpaqueEvidenceRef,
    pub license_trust_rotation_evidence_sha256: Sha256Digest,
    pub grants: Vec<F55LicenseGrantSummaryV1>, // 0..=65535，按 grant_id 排序唯一
    pub current_grant: Option<F55CurrentGrantSummaryRefV1>,
    pub license_status: LicenseStatus,
    pub license_restriction_reason: Option<LicenseRestrictionReason>,
    pub ai_purchased: bool,
    pub ai_currently_licensed: bool,
    pub mcp_purchased: bool,
    pub mcp_currently_licensed: bool,
}

pub struct F55EffectiveConfigSnapshotV1 {
    pub schema_version: u16, // exact 1
    pub stage14_run_id: Uuid,
    pub product_build_sha256: Sha256Digest,
    pub deployment_id: Uuid,
    pub observed_at: DateTime<Utc>,
    pub normalized_nonsecret_config_sha256: Sha256Digest,
    pub ai_enabled: bool,
    pub mcp_inbound_enabled: bool,
    pub mcp_outbound_enabled: bool,
}

pub enum F55ProductModuleDagConclusionV1 {
    Acyclic, // wire exact "ACYCLIC"
}

pub struct F55ProductModuleContractProjectionV1 {
    pub module_code: ModuleCode,
    pub module_contract_version: u32, // effective domain 1..=2147483647
    pub module_contract_sha256: Sha256Digest,
    pub module_dependencies_sha256: Sha256Digest,
}

pub struct F55ProductManifestProjectionV1 {
    pub schema_version: u16, // exact 1
    pub stage14_run_id: Uuid,
    pub product_build_sha256: Sha256Digest,
    pub deployment_id: Uuid,
    pub observed_at: DateTime<Utc>,
    pub signed_product_manifest_sha256: Sha256Digest,
    pub packaged_file_roster_sha256: Sha256Digest,
    pub product_sbom_sha256: Sha256Digest,
    pub core_server_artifact_sha256: Sha256Digest,
    pub product_modules_manifest_sha256: Sha256Digest,
    pub installed_product_modules_manifest_sha256: Sha256Digest,
    pub product_version: SemVerV1,
    pub product_modules: Vec<F55ProductModuleContractProjectionV1>, // exact 15
    pub product_module_dag_conclusion: F55ProductModuleDagConclusionV1,
    pub server_admin_asset_manifest_sha256: Option<Sha256Digest>,
    pub server_admin_asset_count: u32,
    pub server_admin_sbom_component_ref: Option<String>,
    pub server_admin_sbom_component_sha256: Option<Sha256Digest>,
    pub server_admin_included: bool,
}

pub struct F55ApplicabilityInputV1 {
    pub schema_version: u16, // exact 1
    pub stage14_run_id: Uuid,
    pub product_build_sha256: Sha256Digest,
    pub deployment_id: Uuid,
    pub deployment_record_revision: u64,
    pub deployment_carrier: DeploymentCarrier,
    pub observed_at: DateTime<Utc>,
    pub entitlement_snapshot_ref: OpaqueEvidenceRef,
    pub entitlement_snapshot_sha256: Sha256Digest,
    pub effective_config_snapshot_ref: OpaqueEvidenceRef,
    pub effective_config_snapshot_sha256: Sha256Digest,
    pub product_manifest_projection_ref: OpaqueEvidenceRef,
    pub product_manifest_projection_sha256: Sha256Digest,
    pub disabled_mode_report_ref: OpaqueEvidenceRef,
    pub disabled_mode_report_sha256: Sha256Digest,
}

pub enum F55GateCode {
    AiContainmentGreen,
    AiResourceCertified,
    McpConformanceGreen,
    McpContainmentGreen,
    ServerAdminMatrix90Green,
    DeploymentCarrierEvidenceGreen,
}

pub enum F55Capability { Ai, Mcp }

pub enum F55GateDisposition {
    RequiredPass,
    DisabledEvidence { capability: F55Capability },
    NotInBuild,
}

pub fn derive_f55_gate_dispositions(
    input: &F55ApplicabilityInputV1,
) -> Result<BTreeMap<F55GateCode, F55GateDisposition>, AppError>;
```

`F55CurrentGrantSummaryRefV1.grant_summary_sha256` 的唯一前像为对应 `F55LicenseGrantSummaryV1` **全字段** strict object，domain/purpose 都是 `EP-F55-LICENSE-GRANT-SUMMARY-V1`，按本节统一 `projection_digest` 计算；summary 已显式携带 `schema_version=1/purpose`，全部 revocation/日期/supersedes/restriction Option key 无值时仍写 JSON null，`legal_entity_ids` 按 UUID bytes、`entitlement_codes` 按 wire bytes 排序去重。snapshot 内该 summary object、其重算 digest 与 current ref 三者必须相等，不接受只 hash grant id/payload 或直接 hash summary 在大文件中的 byte slice。

三份 projection 的生成、来源、路径和时效同样封闭。Stage 14b 只运行 `ep-release-gate collect-f55-context --stage14-run-id <uuid> --deployment-id <uuid>`；两个 UUID 来自本次 evidence orchestrator，不设默认。工具先要求同一 build/deployment 的 `RG-LICENSE-MODULE-LIFECYCLE-GREEN` 真实 PASS，再在 `tools/release-gate/src/f55.rs` 中调用 `F55EntitlementEvidenceQuery` 读取 F-56 current/history exact-set，并要求 snapshot 内 trust-rotation ref/digest 逐字等于共同 gate index 的 `license_trust_rotation_exact_set` entry；任一行、inner/special-outer 签名、source FK、撤销、current-slot 或 trust entry 不合法均使整次采集失败。四个 purchase/currently-licensed 布尔值只能从 summary 重算：purchased 只认 current/history 中 `grant_trust_rotation_result=TRUSTED` 的 grant 含相应 `EntitlementCodeV1`，`HISTORICAL_SIGNER_REVOKED` 即使保留原 code 也必须排除；currently licensed 只在唯一 current grant 同时为 `TRUSTED`、含该 code 且状态为 `Active|ExpiringSoon|GracePeriod` 时成立，`Restricted` 一律为 false。一个 `F55Mcp` 同时决定两个 MCP 方向的许可事实；法人级业务请求仍由 F-56 `entitlement_is_currently_licensed(...,legal_entity_id)` 逐次执行 signed scope 判定，deployment-level snapshot 不替代该运行时守卫。AI/MCP 平台 capability 本身没有对应 `ModuleCode`，因此不得无条件调用虚构的“AI module/MCP module” gate；只有请求触及具体业务对象时，才必须在读取该对象 payload、推理使用其字段、动态写/批或外发前通过该对象真实 owner module 的 `ModuleOperationGate`/effective-runtime admission。owner module 失信只关闭该对象所属模块，绝不靠改写全局 LicenseStatus 实现。

`issued_at/last_trusted_at/trusted_now/observed_at` 均为 UTC 秒精度；PERPETUAL/SUBSCRIPTION 形状、60 天临期、30 天宽限、撤销立即 Restricted 与 future-issued 五分钟上限逐字复用 F-56。可信时间也只能复用其 `TrustedClockV1`：数据库连接后/public readiness 前以持久证据和 `system_utc_at_start` 建立 process anchor 并捕获 OS monotonic；每次 query/apply 取持久证据、单次 wall clock 与 `anchor+monotonic_elapsed` 的最大值；readiness、每个推进 special 的关口与 job-worker 目标间隔≤240 秒都在同一 license advisory lock 内按 `slot_utc=floor(unix_seconds/240)*240` 的 canonical UTC whole-second 耐久键执行 append-only checkpoint（零行追加首行、已有一行验真复用、永不 UPDATE/追加第二行），current 的 `last_trusted_at` 则独立 CAS 单调提高。`ensure_checkpoint` 入口在业务 mutation 前锁内单次捕获 trusted_now/current id，terminal batch 只能消费该 immutable capture。Stage 14 必须证明同进程不降、checkpoint/续期竞态、回拨后崩溃未持久窗口严格小于 300 秒、同 slot 首行值不随后续动作变化、`last_trusted_at` 可合法高于本 slot 首行、uptime 内相邻成功 checkpoint 的 trusted-now 差值≤300 秒，以及错误前跳只能经可信备份完整恢复；差值>300 秒、同 slot 多行、slot 映射错误或 terminal 重算均失败，不得降格为 daily-only 检查、单取 wall clock 或 direct SQL 回拨。该发布观测门限不宣称 NTP 或 TPM。F-56 special outer 与 inner CMS 的共同 release root 继续用既有包装路径 `target/release-package/trust/license-roots.p7b`，该兼容文件名不得解释为第二套许可证根；普通 config package outer 仍由部署 KMS 独立签验，绝不改读该文件。bundle exact bytes 必须被同目录 `MANIFEST.sha256` 精确覆盖；安装后 exact path 为 `C:\ProgramData\EnterprisePlatform\trust\license-roots.p7b`，两份 exact bytes SHA-256 必须相等并写入保留字段名 `license_trust_bundle_sha256`。special verifier 只接受锚定该只读共同 bundle、`signer_subject` 命中 `spki-sha256:<64-lowerhex>` allowlist、并按上述整条 non-anchor chain、完整 base CRL/每 issuer 覆盖 `trusted_now` 的最高 CRLNumber 与 exact AlgorithmIdentifier 算法得到 ACTIVE|RETIRED|REVOKED 的唯一 CMS chain；既有 accepted current/history 可为 ACTIVE|RETIRED，本次新 candidate/action 两层必须 ACTIVE，REVOKED 只走上述历史隔离或模块停用窄路径，UNTRUSTED 从不序列化且直接失败。不得读取 Windows 当前用户/本机任意根存储、联网补链或接受命令行替换 trust bundle。

同一命令对已安装 `ep-core` 的 SCM `ImagePath`、服务环境和其引用的主配置/片段目录按 `docs/config-reference.md` 五层覆盖顺序调用 `crates/platform/runtime/src/config/sections.rs` 的同一个 strict loader，只从构造成功的 `CoreConfig.ai.enabled`、`mcp.inbound_enabled`、`mcp.outbound_enabled` 形成三个布尔值；`normalized_nonsecret_config_sha256` 是该 loader 的完整规范非秘密导出（secret 仅保留规范 ref）JCS digest。命令行不得传这三个值，也不得直接读取调用者进程自己的 `EP__...`。它还须用 integration-gateway/plugin-host 的 SCM launch facts复核共享 MCP section 投影一致；任一服务 launch/config source 读不到、unknown key、解析失败或同键不等即不产出 snapshot。

产品投影只从同一待签发布目录的 `target/release-package/MANIFEST.sha256`、`MANIFEST.sha256.sig`、`signing-metadata.json`、`sbom.cdx.json`、清单覆盖的 `bin/core-server.exe`、同一 closed roster 内唯一 `target/release-package/product-modules.v1.jcs`，以及该 PE 内由 ServerAdmin `apps/core-server/build.rs` 生成的 immutable asset manifest 读取；先完成生产 Authenticode、manifest 签名和 closed-file-roster 复验，再投影。安装后还必须从固定 `C:\EP\product-modules.v1.jcs` 用 safe handle readback，拒绝 reparse/ADS/hardlink/path drift；发布目录与安装路径 exact bytes digest 必须相等并命中 `MANIFEST.sha256`，数据库、环境变量、ServerAdmin、MODULE_PACKAGE 或第二目录都不得提供替代目录。`product_modules_manifest_sha256` 与 `installed_product_modules_manifest_sha256` 必须相等；strict parser 投影 exact `product_version` 与按 ModuleCode wire 排序的 15 行 `{module_code,module_contract_version,module_contract_sha256,module_dependencies_sha256}`。每行 `module_contract_version` 的唯一有效域是 `1..=2147483647`：Rust `u32`、两份 manifest、当前 module projection、证据 DTO 与 PostgreSQL `integer` 必须逐值相等，数据库类型保持 integer；0、2147483648、JSON 越界或 checked conversion 失败均非零且无 projection。每个 dependency digest 必须用 domain/purpose=`EP-F55-MODULE-DEPENDENCIES-V1` 对 exact DTO `{schema_version:1,purpose:"EP-F55-MODULE-DEPENDENCIES-V1",module_code,dependencies}` 调统一 `projection_digest`；`dependencies` 按 ModuleCode wire 排序去重且可为空，禁止继续对裸 `Vec<ModuleCode>` 直接求 SHA-256。`product_module_dag_conclusion` 唯一为 `ACYCLIC`。`signed_product_manifest_sha256` 是 exact `MANIFEST.sha256` bytes digest；`packaged_file_roster_sha256` 是其逐行解析为 `(canonical_relative_path,file_sha256)`、按 path bytes 排序去重后的 strict JCS digest；产品 SBOM 只接受 CycloneDX，ServerAdmin component identity 唯一为 `bom-ref="urn:ep:component:server-admin"`。included=true 当且仅当 embedded asset manifest 存在、asset_count>0、两个 asset/SBOM Option 均为 Some、component ref 逐字等于该值且 component digest 与 SBOM 项重算一致；included=false 当且仅当 asset digest=None、count=0、两个 SBOM Option=None 且安装包 roster 没有任何 `/server-admin/` 独立资产。模块 manifest 缺失、不是 15 行、依赖不在闭集/自环/成环、contract 或任一 manifest/SBOM/PE/build digest 漂移均报错。

证据根唯一为 `target/release-evidence/f55/<lowercase-stage14-run-id>/`，五种 projection/input/report 文件名恰为 `entitlement-snapshot.v1.jcs`、`effective-config-snapshot.v1.jcs`、`product-manifest-projection.v1.jcs`、`applicability-input.v1.jcs`、`disabled-mode-report.v1.jcs`，各 sidecar 为同名追加 `.sig.jcs`。`F55ApplicabilityInputV1` 内四个 ref 的唯一语法为 `ep-evidence://stage14/<lowercase-stage14-run-id>/f55/<entitlement-snapshot|effective-config-snapshot|product-manifest-projection|disabled-mode-report>/sha256/<64-lowerhex>`；applicability 文件自身不自引用。entitlement snapshot 内 `license_trust_rotation_evidence_ref` 则必须逐字等于同 run 共同 gate index 的 `ep-evidence://stage14/<same-run>/license-module/license-trust-rotation/sha256/<digest>` entry，且其 ref 末段、snapshot digest 字段、共同 index entry digest 与 exact trust manifest bytes 四者相等。resolver 只映射上述固定根/文件名，拒绝 escape、reparse、ADS、hardlink、未知 kind 与 digest 不等。三份 projection、applicability、禁用态报告、当前完整 deployment revision、共同 gate trust evidence 与对应 CarrierEvidence 必须同一 `stage14_run_id/product_build_sha256/deployment_id`；三份 projection 与 applicability 的每个 `observed_at`、禁用态报告的 `started_at/completed_at` 都须落在同一 Stage 14 closed run window，且 applicability `observed_at` 与最早 projection/report 时点相差不超过 15 分钟。Stage 14a0 的 fixture 只测不依赖 F-56 runtime 的 ABI/签名/ref/freshness 与缺 source 非零；Stage 14a1 再测 signed-grant query/adapter 的 focused 正反例；两者均不产出可复用 PASS 文件。

`F55GateDisposition` 只存在于 `tools/release-gate/src/f55.rs` 的进程内判定，不序列化成 gate outcome，也不向 gate result 增加 `N/A`、`DISABLED` 或其他 wire token。verifier 在调用 `derive_f55_gate_dispositions` 前必须先按第 8.7 节通用 ABI 验证同一 `stage14_run_id/deployment_id/product_build_sha256` 的 `RG-LICENSE-MODULE-LIFECYCLE-GREEN` exact index、result 与两份 sidecar，且 result outcome 为唯一 `PASS`；该共同前置不进入六值 `F55GateCode`，也不改变三 projection 或 applicability DTO。随后 verifier 必须重新解析和验签三份 projection、disabled report 以及 `DeploymentRecordRepository` 选出的当前完整 revision；输入不再含可自报的 purchase/enabled/ServerAdmin 字段。任一 enabled=true 但对应 currently licensed=false、currently licensed=true 但 purchased=false、多于一个 current 或 current/history flag 形状非法、summary 状态/可信时间/签名/source 不合法、当前 deployment id/revision/carrier 不等、三份 projection 互绑不等、ServerAdmin 三源不等、任一 ref/digest/signature/freshness 不等都直接报错，不能降为禁用态；零 current 只有在 snapshot 精确满足 `current_grant=None/RESTRICTED/NO_CURRENT_GRANT`、相关 enable 全 false 且禁用态报告成立时走 `DisabledEvidence`，空库还必须额外满足 `grants=[]`，不能伪造 `RequiredPass`。

共同许可 gate 已通过后，派生函数的六项 F-55 闭集映射仍只有四条：一，从 entitlement/config projection 得到 `ai_purchased && ai_currently_licensed && ai_enabled` 时两项 AI code 均为 `RequiredPass`，否则只在 `ai_enabled=false` 时二者均为 `DisabledEvidence { Ai }`；二，从同一份 entitlement 中唯一的 `mcp_purchased/mcp_currently_licensed` 与 config 两方向开关得到 `mcp_purchased && mcp_currently_licensed && (mcp_inbound_enabled || mcp_outbound_enabled)` 时两项 MCP code 均为 `RequiredPass`，否则只在两个 enabled 都 false 时二者均为 `DisabledEvidence { Mcp }`；三，从 product projection 得到 server_admin_included 时矩阵 code 为 `RequiredPass`，否则为 `NotInBuild`；四，carrier code 永远为 `RequiredPass`，其 carrier 必须等于当前完整 deployment record 的二值枚举。每个 `RequiredPass` 必须在其固定 gate 根出现第 8.7 节 exact index、result 与两份 sidecar，result 只能为 `PASS`，四份文件、index 所引每份 typed evidence 及 applicability 必须匹配同一 run/build/deployment/closed window；只出现散落 digest、任一孤儿文件或跨 run 拼包均失败。`DisabledEvidence` 与 `NotInBuild` 不生成对应 final result/index/sidecar，若出现伪造 PASS 或孤儿 final index/sidecar反而视为输入形状错误并返回非零。`RG-LICENSE-MODULE-LIFECYCLE-GREEN` 自身永远要求上述 exact PASS pair，不参与 `DisabledEvidence|NotInBuild` 分支。

禁用分支唯一证据 ABI 为下列 strict `F55DisabledModeReportV1`，由 [阶段 13c Task 6](13c-local-ai-mcp-server-admin.md#task-6-bind-f-55-gates-for-stage-14b-without-weakening-disabled-mode) 的 `ep-f55-disabled-mode` 生成候选、Stage 14b 对发布制品重跑并签名。四类 retained fixture 必须各预置至少一条有效登记并由授权管理查询在禁用后读回，不能用空表把 retained 布尔值判真。

```rust
pub struct F55DisabledModeReportV1 {
    pub schema_version: u16, // exact 1
    pub stage14_run_id: Uuid,
    pub deployment_id: Uuid,
    pub product_build_sha256: Sha256Digest,
    pub effective_config_snapshot_sha256: Sha256Digest,
    pub disabled_suite_config_sha256: Sha256Digest,
    pub product_manifest_projection_sha256: Sha256Digest,
    pub suite_build_sha256: Sha256Digest,
    pub ai_enabled: bool,
    pub mcp_inbound_enabled: bool,
    pub mcp_outbound_enabled: bool,
    pub ai_business_http_route_count: u32,
    pub ai_pre_payload_fail_closed: bool,
    pub ai_inference_attempt_count: u64,
    pub mcp_business_http_route_count: u32,
    pub mcp_business_listener_count: u32,
    pub mcp_pre_payload_fail_closed: bool,
    pub mcp_egress_attempt_count: u64,
    pub mcp_local_child_process_count: u32,
    pub model_registration_retained: bool,
    pub connector_registration_retained: bool,
    pub manifest_registration_retained: bool,
    pub grant_registration_retained: bool,
    pub health_control_surfaces_only: bool,
    pub suite_exit_code: i32,
    pub started_at: DateTime<Utc>,
    pub completed_at: DateTime<Utc>,
}
```

该报告通过条件逐字段固定为：`stage14_run_id/deployment_id/product_build_sha256` 与 applicability、当前完整 deployment revision 和发布制品相等；`effective_config_snapshot_sha256/product_manifest_projection_sha256` 精确指向已验签 projection，`disabled_suite_config_sha256` 则是套件隔离配置经同一个 strict loader 导出的规范非秘密 JCS digest；三个 suite-local enabled 均 false；两个 route count、MCP listener count、AI inference/MCP egress attempt count 与 MCP child count 全为 0；两个 pre-payload fail-closed、四个 retained 与 `health_control_surfaces_only` 全为 true；`suite_exit_code=0`；时间为 UTC 秒精度且 `started_at < completed_at`。suite-local enabled 是禁用套件从隔离配置读回的测试进程事实，不是 applicability 购买态/启用态自报；当某 capability 走 `DisabledEvidence` 时，verifier 还须将对应 suite-local false 与 effective-config projection 的对应 false 逐项相等，另一 capability 可在产品部署中启用而不影响本套件用隔离配置证明统一安装包的禁用行为。统一安装基线下九个产品服务以及 `ep-ai|ep-integ|ep-plugin` 的 DACL health/control surface 可存在，但只能令 `health_control_surfaces_only=true`，不能折算为业务启用。基础产品每次发布都必须运行这套件，即使该客户本次购买并启用了 AI/MCP；报告缺失、格式错误、空 retained fixture、digest/signature/build 不等均非零。14a0 只冻结 ABI/登记六个名字并对真实证据缺席 fail closed，14a1 只接上 F-56 query/adapter，13c 只提供实现与候选 evidence；只有 14b 可形成最终 `RequiredPass` index/result 和允许发布，任何分支都没有部分发布许可。

---

### 9. 退出条件

全部为可客观判定项。

1. archive-writer、backup-writer、ops-agent 三个进程在 BC-1 基线组合上以 --check 通过本进程适用的全部已注册自检项，两个写出进程对 SQL 类自检项一律标 NotApplicable，报告中无 FAILED 也无 DEGRADED；并在生产配置下连续运行不少于 7 个自然日，期间三类写出周期无一次超过 15 分钟。
2. 第 3 节的 23 张表（其中 `degradation_windows` 由阶段 2 首次建表且 kind CHECK 只含初始 3 项；本阶段常规 `V20261023090300` 先追加两条 CHECK，`platform_ops/concurrent/V20261023090350` 独立追加三个并发索引，`V20261023092500` 最后把 kind CHECK 从 3 扩为终态 21、不可抑制闭集从 4 扩为 5；表 18 至表 23 为带法人 RLS 的迁移表）、5 个视图与 Stage 14 自有 28 个迁移文件全部落库，每个迁移带 rollback 段，platform_core.schema_history 与二进制期望版本一致；090000 是唯一兼容 no-op 且在两张单例建表前不引用未来对象，两条单例分别随 090700/090900 建表原子插入；静态扫描证明除 090350 外所有常规文件均无 `CONCURRENTLY`，新建空表索引均使用普通 `CREATE INDEX`。本条的 28 只指 Stage 14 roster；用于 F-55 的共享数据库还必须按第 0.0 节证明全局 30 个 `V20261023...` 已完整执行并止于 `092800`。
3. 归档通道状态机的九个合法 from/to 对在集成测试中逐项通过，另有 HEALTHY、RETENTION_WARNING、SLOT_INVALIDATED、REBUILDING、SUSPENDED 五态各一次合法 OBSERVATION；落点可写与不可写两支各完整走通一次，暂停态在落点恢复后自动转入重建且无需人工发起，全部历史版本可只凭初始行与 typed after-image 重建。
4. 台账二十一类 kind 的开闭各至少一条实证记录；OFFSITE_SINK_NOT_CONFIGURED、OFFSITE_COPY_PROTECTION_MISSING、WRITER_NOT_IN_SERVICE、VIRUS_SCANNER_NOT_AVAILABLE 与 LEGAL_ENTITY_KEY_DOMAIN_UNAVAILABLE 五类不可关闭告警在管理员尝试关闭或静音时均被拒并写审计；其余各类的抑制与静音记名记时写入审计；REPLICATION_CROSSCHECK_NO_RESULT 有连续第二次无结论开窗和下一次有结论关窗证据；病毒扫描有 NONE 常开、CUSTOMER_ICAP 健康关窗及故障重开证据；法人密钥域有未配置、KMS/HSM 解封失败、migration receipt 无效三支开窗，以及原 provider 恢复、receipt 有效且真实解封后关窗证据；同一 kind 下 subject 不同的两条窗口可同时活动且各自独立开闭。
5. v_rpo_status 在七种依据下各输出一次正确取值，且在任一降级、未达成或承诺不成立状态下均未输出 900。
6. 落点上的全部写出对象为密文并以批次唯一不可复用 key 及 CREATE_NEW/`If-None-Match: *` 写入；无恢复材料时无法从副本读出任何业务数据，含未被字段级加密的明文业务表内容。writer、restore、disposal 三身份和凭据两两互斥；writer 只能列举、创建新对象和必要校验读，覆盖、删除、重命名、改权及策略管理的逐后端负向探针全部被拒。对象存储 IAM deny、Windows/SMB DACL 或经认证 NFSv4 ACL 的策略摘要、账户标识、实测时点与证据引用已写 offsite_sinks 与部署记录；普通 POSIX/NFS 可写目录能删除或改写历史文件时本条失败。`OFFSITE_COPY_PROTECTION_MISSING` 无活动窗口；交付材料同时披露这不是 WORM，客户存储管理员仍可绕过。
7. 两个专用角色的越权测试在 tests/rls_matrix 目标内全部通过，断言取阶段 2 提供的 assert_replication_role_containment，本阶段无同名函数的第二份实现；独立的 `writer-role-containment` 在两个写出进程建立任何复制连接前执行，三项任一未落实时对应角色不启用、该进程以 78 退出、--check 非零，core-server 开载明缺失项的 WRITER_NOT_IN_SERVICE，补齐后角色与进程启用且窗口闭合；角色启用后的 NO_RESULT 不改变该自检项与进程状态；交付说明中已按第 21.21 章披露三项遏制都不阻止本机特权主体路径。
8. 复制交叉核对随 30 秒保留量采样持续执行：MATCHED、MISMATCHED、NO_RESULT 三态均有实证；白名单外复制槽、非写出进程会话与报告侧幽灵记录均在下一次采样产出 MISMATCHED 并告警审计；连续第二个 NO_RESULT 开 REPLICATION_CROSSCHECK_NO_RESULT，下一次 MATCHED 或 MISMATCHED 关窗。未建立独立连接、表、指标或配置键，只读分析池交互式上限仍为 10。
9. 附录 A.6 整机失效恢复与密钥恢复材料隔离恢复两类演练各执行两次且两次均达标，另按裁定 F-11-4 执行保留期尾端恢复一次并按第 8.4 节为其单列的判定项集合达标（该次不判 RPO），各次演练的报告齐备，其中附件元数据与正文的逐条比对结论、未通过条目清单与该校验实际耗时单独留证，恢复模式的不变量校验分批取值已冻结；保留期尾端恢复一次另判其备份集判据，即 recovery_drills 冻结的 backup_verified_at_at_start 与演练开始时点的间隔不少于 retention_days_at_start 减 1 天，且所指备份在演练开始时处于 VERIFIED（事后合法处置不改变该历史事实），不叠加以 D 为上界的折算；认证取 retention_days_at_start=14。该次演练的实证记录与其余各次一样必须在本平台重做，Linux 上跑出的记录一条都不能沿用。
10. 附录 A.1 至 A.4 的完整基线测试执行一次，第 8.5 节八项必判项全部成立，全部必记项已记入认证报告，服务器规格随该报告冻结并按规格第 13.1 章作为交付客户的服务器规格下限；客户服务器规格不低于认证报告所记规格时沿用该次认证结论，不重跑附录 A.4。资源单位（具名 Job Object）的首版路径已经冻结：按权重的磁盘 IO 份额整列不支持，CPU 比例只作硬件标定与认证意图声明且不落运行期取值；内存硬上限是配额表唯一运行期列，F-55 后九个自研二进制由服务宿主自我指派，`ai-inferer` 使用原“内置搜索索引”行改名后的独立 `APP_AI` 资源单位并另过 F-55 AI 资源门禁；PostgreSQL 16 与反向代理由 ops-agent 创建具名资源单位并调用 AssignProcessToJobObject；backup-writer 绝对 IO 上限落静态限额文件、部署记录和 Windows 读回夹具。PostgreSQL/反向代理指派与 backup-writer 限速两项在证据形成前状态为 `UNVERIFIED` 且本条不通过，不表示存在替代实现或等待选择。转为 `VERIFIED` 的唯一谓词分别是：部署校验脚本从 PostgreSQL 16/反向代理各自资源单位读回内存硬上限并与静态文件一致；Windows 夹具读回并证明 backup-writer 实际限速与静态文件一致。仅出现配置行、文档登记或命令存在均不得折算为通过。
11. 规格第 17.2 章十七类自动化测试的本阶段相关类型全部执行：混沌与故障注入六类、备份与完整恢复、审计链与不可变存储、数据保护控制与销毁证明、安全模糊与渗透。严重与高危缺陷全部关闭，中危缺陷登记并给出规避方案与责任人。
12. 覆盖率达标：平台内核与不变量相关代码不低于 85%，新增与修改代码不低于 80%，工作区整体不低于 80%，无带 issue 编号之外的 #[ignore]。
13. 等级保护三级控制项自评矩阵完成，除第 17.5 章登记的四项永久性不符合项外全部符合，其余不符合项均已关闭并经具备资质机构预评估；CI 校验不符合项条目未超出封闭清单。
14. 供应链安全各项齐备：SBOM、构建来源证明、离线依赖仓库、客户侧验签工具、生产 Authenticode 验签与 PE 可复现构建证据全部真实通过；随产品交付的 `ep-data-migrate` 与 `ep-secretctl` 都必须包含在产品 SBOM、依赖/安装包/密钥扫描、生产 Authenticode 与 Windows 两次可复现构建被测集合内。`cargo xtask ci` 第 7、8 阶段任一非零即本条不成立；历史 TSV 状态、文档登记或两次构建命令存在均不得折算成通过。
15. 只有 Stage 14b 可执行本条：ep-release-gate 对第 22 章十五条与第 17.2 章通过标准**逐条判定为通过**（原写「逐条产出判定结论」——只要求产出结论、不要求结论为通过，十五条中任一条判为不通过时本条仍成立，属恒真判据；裁定 F-42 改写）。第 8.7 节 RG-CI-PROBE-ABSENT、RG-TOOLS-EXCLUDED、RG-PLAINTEXT-SECRETS-ABSENT、RG-RLS-MATRIX-GREEN、RG-UNWIRED-ABSENT、RG-NO-UNDECIDABLE、RG-OFFSITE-COPY-PROTECTED、RG-EXTERNAL-CLAIMS-SIGNED 与 RG-LICENSE-MODULE-LIFECYCLE-GREEN 九项永远适用且必须取得真实 `PASS`，其中 plaintext-secret 与 license/module gate 永不得 N/A。只有共同许可 gate 的 exact `Stage14GateEvidenceIndexV1`、`Stage14GateResultV1` 与两份 sidecar 先对同一 run/build/deployment 验签通过，且共同 index 的 `license_module_lifecycle_matrix|license_admission_registry_exact_set|license_admission_negative_matrix|license_trust_rotation_exact_set` 四项 exact DTO/ref/case/chunk 全部解析，lifecycle 的 deployment-manifest/initial-governance 两个 typed child 已按 CMS/sidecar/DACL/source/ref/首装 DB-审计-grant-key-domain/双 X509-MFA 全链验真，current license/current module 独立结论与 ServerAdmin 15 行 trust status 一致，六项 F-55 code 才按本节验签后的 `F55ApplicabilityInputV1` 派生：AI 两项与 MCP 两项各自为 `RequiredPass` 时必须有对应 exact index/result/两份 sidecar，为 `DisabledEvidence` 时必须由同 run/build/deployment 的禁用态报告承接且不得出现对应 final index/result/sidecar；ServerAdmin 矩阵在签名产品投影 `server_admin_included=true` 时为 `RequiredPass`，否则只能为 `NotInBuild` 且不得出现对应 final index/result/sidecar；carrier evidence 永远为 `RequiredPass`。九项永久 gate 与每项 `RequiredPass` 都只接受第 8.7 节通用 ABI、唯一 `PASS` outcome、同一 closed window 和 typed evidence exact roster；任何包含 ServerAdmin 的 build 必须通过矩阵，每个 deployment 都必须通过 carrier。缺共同 gate、缺 projection、非法组合、过期/坏签名、缺 required pair、出现禁止/孤儿文件、跨 run/build/deployment 拼包或任一 typed evidence 不匹配均失败，不得以散落 digest 数量替代。发布证据包组装完成，含认证报告、演练报告、台账快照与暴露窗口记录、`RG-LICENSE-MODULE-LIFECYCLE-GREEN` 的签名 index/result、四份 F-56 typed report 及 lifecycle case/trust chunks、签名 `DeploymentManifestEvidenceV1` 与独立 deployment CMS/root/DACL readback、由 lifecycle 绑定的无 sidecar `InitialGovernanceEvidenceV1`、F-56 current-history 摘要、含签名 `product-modules.v1.jcs`/installed readback/product version/15 行 contract-dependency digest/DAG 结论的五份 F-55 projection/input/report、六项 disposition 摘要与仅对 `RequiredPass` 存在的 exact index/result、`SecretTerminalEvidenceV1`/receipt/六负例报告、离站副本三身份及防删负向探针证据、缺陷台账、渗透测试结论、等保自评结论、各业务阶段四端界面交付情况汇总矩阵、历史数据迁移完整演练与切换/冲销报告、支持套餐回报周期已选证据、病毒扫描模式与 CUSTOMER_ICAP 实测/责任边界证据、签字验收记录，以及逐条对外表述清单、全部受检材料的版本与摘要、产品负责人签字页；ep-bench 与 ep-release-gate 的真实成功命令均返回 0，未交付夹具仍返回 70，产品 SBOM 实物与注入两包名的负向夹具分别通过/失败。对外表述清单或任一受检材料在签字后发生变化时，原签字自动失效，重新判定并签字前本条不成立。Stage 14a0、14a1 或 13c 无权满足本条，也不得据候选 evidence、disabled/NotInBuild 分支或某一子集作部分发布。
16. PRD 第 11.11 节既有八条诚实披露文本及病毒扫描、离站副本防删边界新增披露已进入交付说明与客户合同模板，并在产品界面可达处呈现；NONE 必须逐字写“平台未提供病毒防护”，CUSTOMER_ICAP 必须写客户扫描器、病毒库更新、许可与误报漏报责任边界；离站副本必须逐字写“本控制不是 WORM 或不可变存储，客户存储管理员仍可删除或改写副本”。交付、认证与验收材料经文本检查未出现高可用、零停机、自动切换、受控读取、法人隔离、等效、已满足、优先级隔离、资源隔离、性能保证十项禁用措辞；同时按规格第 21.22 章逐条分档，第一档仅保留已认证事实且同条带全前提，第二档在当前版为零，第三档为零。第三档检查至少覆盖「碾压」「行业模板」「实施顾问」「生态伙伴」及同类承诺性表述和全部未经实测的比较级；本规格自身不属于文本检查对象，但交付、认证、验收材料与客户合同全部属于产品负责人逐条裁决和签字对象。
17. OpsDisposalService 已实现阶段 3b 定义的 DisposalPort 并在 core-server 与 job-worker 两个 wiring 目录内首次注入，阶段 3b 至阶段 13 的两个目录内均未出现该端口的任何替身与任何注入行；其间的物理删除请求经阶段 3b 注册的受理路由以 PLATFORM.DISPOSAL.NOT_DELIVERED 与 HTTP 409 被拒且不可重试，subject 取 DisposalPort 的 PORT_NOT_IMPLEMENTED 降级窗口全程活动并已在本阶段注入后关闭，该窗口的开闭两端各有一条实证记录；AttachmentObjects、KeyDomain、BackupSets、ExtTables 四类处置范围各有一次完整执行记录，销毁证明对象与审计条目齐备，落点侧历史副本由独立 disposal 身份在同一次处置内按批准清单删除精确 key/版本并逐项回读确认不存在；BackupSets 成功处置还必须在同一受理事务把备份行单向转 DISPOSED、保存 disposed_from_state/at/certificate_ref，且 v_backup_last_success 不再选中；writer、restore、disposal 三身份任一复用、任一目标遗漏或部分删除时证明不成立。缺审批链、缺第二审批人、缺重新认证凭证或处置身份未临时解封时执行被拒并写审计。
18. 电子签章的认证清单已补齐：crates/adapter/esign/tests/contract_sandbox.rs 对真实沙箱的一次通过记录已归档，或已提交规格附录 B 允许的等效验证证据。
19. 本阶段在 `docs/metrics-catalog.md` 注册并填充的指标集合精确等于 `ep_archive_write_lag_seconds`、`ep_attachment_write_lag_seconds`、`ep_backup_last_success_timestamp_seconds` 三项；`ep_degradation_windows_open`、`ep_db_pool_connections` 与 `ep_db_statement_duration_seconds` 只复用不重复登记；原裁定 C-22 的 `ep_replication_crosscheck_age_seconds` 已撤销，阶段 2 与本阶段均不登记。`docs/event-catalog.md` 同时明确本阶段新增平台事件为 0；部署状态只写 platform_ops、platform_audit 与已登记指标，没有虚构的“系统法人”，历史迁移只复用模块既有事件。
20. `platform_ops.degradation_windows` 的阶段边界已按唯一顺序成立：阶段 2 历史迁移与 Rust 只含初始 3 项；Stage 14a0 已在 `crates/platform/obs/src/degradation.rs`、`crates/adapter/db-pg/src/platform_ops/degradation.rs` 及 mock/contract 测试先把 enum/serde/ledger 参数域扩为同序 21 项而未向旧库写后 18 项；090300/090350 的两条 CHECK 与三个索引已建立；完整 pre-F-55 链执行 `V20261023092500` 后，数据库 kind CHECK 恰为同序 21 项、不可抑制闭集恰为含 LEGAL_ENTITY_KEY_DOMAIN_UNAVAILABLE 的 5 项，real-PG exact-set/unknown-value/五项不可抑制测试全绿。阶段 2 交付的列、`ux_degradation_windows_kind_scope_closed` 与 `ck_degradation_windows_open_order` 未被历史重写；任何 092500 前非初始值成功写库、Rust/SQL 集合或顺序不等都令本条失败。
21. 本阶段全部 /api/v1/ 路由的能力域码与动作类别常量已按 A-20 声明在 crates/platform/obs/src/capability.rs，能力域一律取 foundation::CapabilityDomain::PlatformAdminLowcodeOps，动作类别取 foundation::ActionClass，xtask configdoc 通过。
22. 本阶段新建且不带法人列的 16 张 platform_ops 表在 platform_core.unpoliced_table_registry 中各有一行登记，schema_name、table_name、admission_basis、isolation_entry 与 matrix_case_id 五列取值齐备，且 db/checks 的第十三项返回零行；阶段 2 登记的八行未被本阶段改写，degradation_windows 在该表中不产生第二行。表 18 至表 23 六张历史迁移表不在登记表中，均带 `legal_entity_id`、ENABLE 且 FORCE RLS，并在 tests/rls_matrix 的读、写、更新、聚合、排序、报表投影和错误泄漏矩阵上全绿；任一六表误入 unpoliced 登记或未启用策略，本条失败。
23. 规格第 21.4 章要求且归属阶段 14 的两类签字已取得并留档：安全专业签字须覆盖规格第 12.5 章审计链与威胁模型，签字人资格证据随版本留档；对外表述与宣传材料由产品负责人按规格第 21.22 章对交付、认证、验收材料和客户合同逐条分档裁决，并对逐条清单及全部材料摘要签字，产品负责人的任命或企业授权证据随版本留档。当前版第二档一律不得使用，第三档一律不得使用，第一档缺已认证事实依据或缺前提同样不得签为通过；清单漏项、签字缺失、签字不通过或签字后材料摘要变化时本阶段不得退出，整改或变更后须重新检查、重新测试并重新签字，不得以未记录方式豁免（规格第 22 章第 12 条）。本条明确把对外表述一类的建设、证据组装与签字责任冻结在阶段 14，不留给发布后补签；安全签字要求由裁定 F-42 引入，对外表述签字按规格第 21.4 与第 21.22 章在本轮补齐阶段归属。
24. 规格第 18 章四档时长上限已在附录 A.3 数据集与附录 A.4 拓扑下于 PostgreSQL 16 上各实测一次，
    并逐档判定为通过：常规功能版升级总时长 ≤ 4 小时且切换窗口 ≤ 10 分钟；紧急补丁 ≤ 2 小时且窗口 ≤ 5 分钟；
    回退到升级前直接版本 ≤ 2 小时且窗口 ≤ 10 分钟；经备份或影子表切换的回退 ≤ 8 小时且窗口 ≤ 20 分钟。
    计时自任务开始起算至规格第 17.3 章强制不变量与关键业务数据校验通过为止，含数据迁移、兼容测试与校验时间，
    不含灰度人工观察等待。**任一档超上限即本条不通过**，整改后重测，不得以「已产出实测数据」代替「判定为通过」。
25. 升级前定制兼容测试已运行，且**已实测其失败时确实阻止生产升级**：以一个必然失败的定制用例执行一次，
    确认放行被拒。只运行而不验证阻断效果的，本条不通过——那是恒真判据。
26. 每次升级与回退的证据包含规格第 18 章末条十一项要素齐备：版本跨度、迁移清单、兼容测试结果、
    灰度与切换过程、失败事务统计、升级与回退实测耗时及与时长上限的比对结论、切换窗口实测时长、恢复点、
    强制不变量结果、责任人签名，以及管理员/运维/审计/恢复四类文档的同步更新记录。**缺任一项本条不通过。**
27. 规格第 7.6 与第 7.10 章的历史数据迁移已按第 4.12 节完整交付并判定通过：`ep-data-migrate` 十个子命令、四类只读来源、签名模板 schema v1、25 个对象类型与逐项唯一 writer、25 行静态 relation/root/provenance/projection/reverse channel catalog、六张法人 RLS 台账表、九组状态迁移和全部 API 均存在；第 8.3 节第 18 至 25 项与第 8.4 节迁移端到端验收全部通过。至少一轮完整试运行覆盖全部 25 类且正式数据零效果；历史系统只读冻结、增量追平、正式切换与整批冲销四个动作各产出不可篡改审计证据；每个 VALIDATED 有唯一空根预留，每个 APPLIED 使用该根并有同事务 APPLY receipt，每个 REVERSED 有 catalog owner effect、同 id R0 与指回 APPLY 的 REVERSE receipt。批准、拒绝、撤销、切换与冲销只认表 22 的流程/任务/定义、角色/角色授权长 FK、决定时有效期快照、七键版本路由和服务端重算内容 hash 证据；借贷平衡与库存守恒差异为零，四类不可豁免差异为零，其余差异全部落四类封闭清单并经三方批准；数量、金额、关系、附件、哈希、安全属性赋值九项在迁移前后逐项一致；必然失败记录只进入错误队列且未进入正式数据。任一来源原文或凭据进入数据库/普通日志、工具取得目标数据库凭据、迁移接口在关窗后仍可写、25 个 writer 任一缺失/重复、任一 projection 使用动态 relation/通用 JSON 目标、任一交易反向绕过既有 owner 通道，或任一可变根既无具名不可变 version/change fact、又不是三条具名独立 audit-target owner effect 之一，本条失败；三条 audit-target 任一复用 R0、错 action/before/after/root/version/time 同样失败。本条是可执行验收项，不再登记为未交付缺口，也不再阻塞开始开发。
28. 版本与补丁清单的本地导出已实测：导出物含版本号、已装补丁及其安装时点、许可证状态与健康状态摘要，
    且**实测本实例不发起任何对外出站连接**（以网络侧观测为准，不以配置声明为准）。
    **本条同时是「不建部署管理通道」这一裁定的判负口**：若日后新增任何回传通道而本条未同批重裁，本条不通过。
29. 首版补丁分发已按唯一范围冻结并实测：发布物只含生产 Authenticode 签名的离线补丁包、摘要清单、SBOM、来源证明、离线验签工具、安装/回退说明与严重高危安装时限；本仓制品、部署脚本、配置、端点、DNS 访问与网络观测中均不存在厂商受控在线更新网关、自动下载器或隐藏回传。未来在线网关只能另立厂商侧项目，不得按本阶段扩展点补入。每份客户支持套餐或合同模板在发布前必须选择 `PATCH_STATUS_REPORT_INTERVAL_DAYS`，允许 1 至 7 个自然日、未另选默认 7，并写明客户人工携出第 16 项清单的义务与未按期回报时最高支持等级降为尽力而为级；该值是商业与签字门禁，不是代码开工门禁，缺选择按默认值成文后才能发布。
30. `V20261023092500` 已建立第 3.1.1 节全部真实 FK、七态备份/终态一次性完整校验集合/slot 双向图、STATE_CHANGE 与 OBSERVATION 共用的 typed after-image archive 版本图、恢复演练逐态形状及处置证据；七张 APPEND_ONLY 与两张 IMMUTABLE_COLUMNS 登记、物理 guard、权限撤销逐项一致，并在同一迁移把 degradation kind CHECK 从阶段 2 初始 3 项扩为本文同序 21 项、不可抑制闭集从 090300 的 4 项扩为含 `LEGAL_ENTITY_KEY_DOMAIN_UNAVAILABLE` 的终态 5 项。第 8.1.1 节第 1 至 6 组 direct-SQL 正反例、第 11 组 092500 catalog/rollback 证据及 `stage14_degradation_pg` 的 21 值/未知值/五项不可抑制实库断言全绿，`append_only_consistency.sql` 返回零行；缺任一 FK、触发器、登记、权限、可重放字段、pg_catalog 断言、degradation exact-set 或负例即不得退出。
31. `V20261023092600` 已建立表 22/23、六表 RLS、DATA_MIGRATION reauth challenge/subject digest 绑定、来源 schema 指纹与只读负测版本绑定、批准流程/任务/定义与角色/角色授权长 FK、决定时授权有效期快照、11 模块静态角色映射和七键内容版本图、VALIDATED 目标预留、记录↔writer receipt 双向图、第 4.12.1 节 25 个数据库静态 target/reverse projection 分支与 R0 审计绑定、required reconciliation keys 与 cutover/reversal 状态图；Stage 8 MIGRATION_HISTORY 与 Stage 9 HISTORICAL_MIGRATION owner 追补已同批闭合。三张 APPEND_ONLY 登记及两张受控写函数的精确签名、固定 search_path、PUBLIC 撤权和唯一 EXECUTE 授权成立；permission `...0315` 与 binding `...0509` 逐字段等于第 3.3 节且无自动 role grant。第 8.1.1 节第 7 至 10 组 direct-SQL 正反例、第 11 组 092600 catalog/rollback/permission 证据、25 分支静态生成一致性、任意合法写序提交测试与全事务回滚注入测试全绿。任一任意文本 approval ref 仍可决定状态、任一批准无匹配的已消费 DATA_MIGRATION reauth、任一审批人无决定时有效角色授权、任一旧内容版本可重放、任一 VALIDATED 无唯一空根预留、任一 APPLIED 未使用预留或无数据库核实 receipt、任一 REVERSED 无 catalog owner effect/R0/真实 receipt、或任一 READY/CUTOVER/REVERSED 可被单表 SQL 伪造时不得退出。

第 31 条对三条 audit-target 分支另作不可省略的机械判定：receipt.target_object_type/id 必须等于 `platform_audit.audit_events`/具名 owner event id，owner event 与 R0 分离、同法人同 occurred_at，三个 action、JSON exact key set、状态边、root row_version 及最终 after-image 全部逐项相等；任一分支无法由 092600 静态 SQL 查询、使用通用 JSON、或 direct-SQL 半图能够提交时，第 31 条不成立。

---

### 10. 与规格和 PRD 的对应

规格条目。
- 第 7.5 章：应用级不可变四项中的每份备份自动校验、至少一份备份落在服务器之外；审计证据存储与文件使用独立路径与独立保留策略；另承接一句诚实披露，即单机同机部署下备份、报表与对账在极端情况下仍可能影响交易时延，平台不提供隔离保证，该句进交付说明。
- 第 7.6 章与第 7.10 章：历史数据迁移由本阶段第 1 节交付物 18、第 3.1 节表 18 至表 23、第 4.12 节工具/模板/25 类 writer/状态机、第 5 节 API、第 8 节测试与退出条件 27、31 完整承接。四类来源、字段映射与八种清洗、试运行、错误修复、分批迁移、只读冻结、增量追平、九类对账、已知差异决策、批准证据、writer receipt、切换与整批冲销均有唯一实现和判据；DDL 工具 `ep-migrate` 与四条期初通道都不能代替本承接面。
- 第 7.7 章：两个写出进程的连接与复制槽枚举（按 pg_receivewal 与 pg_basebackup 形态重取为稳态一条连接一个槽、备份窗口内不超过三条连接两个槽）、本机 WAL 暂存上限、四类上报路径、未知复制槽与未知复制会话的检出、三项角色侧遏制手段、越权测试项；三项角色侧遏制手段按该章原文落地，部署期缺一不得启用该角色、两个写出进程随之不得投入运行，仅该章第三项自身写明的运行期例外保留，即角色已启用之后比对连续两个周期未产生比对结论时照常运行并持续告警；本阶段不回写删除该章任何一句。
- 第 7.8 章与第 12.3 章：部署级备份加密密钥为实例级、不属任一法人密钥域、载体只有内置 KMS 与客户自有硬件密码机；密钥恢复材料的分片、双人控制与每 6 个月核验。
- 第 12.5 章：审计证据存储向落点的写出周期不超过 15 分钟并自动校验；最近一次成功锚定时间在运维中心可见且超期告警。
- 第 13.1 章：恢复模式的资源档位；文件存储正文读写按发起进程计费。现行承载固定为 Windows 具名 Job Object 与 `deploy/` 静态限额文件，不做生成算法或按可分配量的运行期折算。内存硬上限是配额表首版唯一启用的运行期列，按 BC-1 算定绝对字节；内存软保底、按权重磁盘 IO 份额、CPU 比例与突发上限均不启用，也不得以最小工作集、`ReservationIops` 或 `MaxBandwidth` 冒充。F-55 后九个自研二进制由服务宿主在 `ServiceMain` 早期创建或打开资源单位并指派；`ai-inferer` 使用原“内置搜索索引”行改名后的独立 `APP_AI` 资源单位，其内存绝对值与认证按 F-55；内置搜索继续按实际调用它的既有进程归因，不再拥有独立行。PostgreSQL 16 与反向代理由 ops-agent 调用 `AssignProcessToJobObject` 指派，backup-writer 的绝对 IO 上限走同一静态限额与部署记录路径。PostgreSQL/反向代理指派与 backup-writer 限速是必须实现后取证的既定主路径，不是设计待决；F-08 证据未形成前只标“未验证”，失败时对应门禁保持非零并走已冻结保守处置。配额事件台账、保底击穿判定与 `cgroup-quota-matched` 自检项均已撤销。部署与升级各运行一次 `scripts/verify-resource-limits.ps1`，只用 Windows API、DACL 与 Job Object 读回核对；唯一 CI 判定是 `cargo xtask ci` 第 11 阶段，历史 TSV 不参与。单机同机部署缺少 CPU/IO 比例隔离的风险继续按规格第 21.19 章和本阶段第 11 节风险六如实披露。

  现行 CI 判定只看 `cargo xtask ci` 第 11 阶段；历史 `.github/ci/pipeline-stages.tsv` 状态不再生效。第 11 阶段须用 Windows API/PowerShell 或 Rust 夹具核对 DACL、具名 Job Object 与静态限额，17 项有效实机证据未完成前保持非零且不得宣称通过；18 个历史编号仅用于追溯，原编号 12 已撤销。
- 第 13.2 章：BC-1 部署适配，其操作系统列取 Windows Server 2022（Server 2019 可在同一形态上运行，但不在首版认证组合内，也不在附录 D.3 的单维度替换清单内）；编排取 Windows 服务控制管理器原生服务加 F-55 后九个二进制共用的一层服务宿主；交付形态取同一份安装包（MSI 或压缩包）加服务注册脚本，同一制品覆盖 2019 至 2022 两版。原「把产品部署进单机容器编排」与「以 OCI 容器交付产品」两项仍撤下；F-55 可选的逐次 MCP 插件 Hyper-V-isolated Windows utility VM 不承载产品服务、数据库或客户主数据卷，不构成产品容器交付。
- 第 13.3 章：RPO 与 RTO 两项目标及其全部前提、降级与不成立情形、以演练验收不以运行期统计值判定。
- 第 13.4 章：全部十九条备份要求逐条落地，含连续归档与时间点恢复、服务器之外落点、三类写出周期、复制槽保留量阈值与硬上限、归档链断裂两支处置、附件与元数据恢复点对齐、落点三项最低要求、写出组件、服务器之外副本保护、落点访问控制分层、落点回传吞吐单独度量、离线介质降级、未配置落点无承诺、附件每日全量写出、配置与证书与模块包与低代码规则包与基础设施定义单独备份、密钥恢复材料分离保管、每份备份自动校验、定期隔离恢复演练。
- 第 15.1 章与第 15.2 章：本阶段错误码的五类归属；备份与写出失败不静默忽略。
- 第 3.3 章与第 15.3 章的厂商部署管理通道：**首版不交付**（裁定 F-44 决定三）。
  规格措辞为许可性——第 123 行逐字「厂商**可以**提供」、第 1264 行逐字「客户**可选择**启用」，
  故不交付**不与规格冲突**，无须请求规格修订。第 17.4 章的替代路径逐字
  「未启用该通道的客户按支持套餐定期回报」由本阶段交付物 16 的本地导出与退出条件 28 承接；支持套餐参数按第 7 节与退出条件 29 固定为 `PATCH_STATUS_REPORT_INTERVAL_DAYS`，范围 1 至 7、默认 7。该参数是发布前合同模板选择与签字门禁，不是软件配置或代码开发待决。
- 第 15.3 章：运维中心全部登记项、两个 RPO 取值与依据枚举、告警持续可见、五类不可关闭告警固定为 OFFSITE_SINK_NOT_CONFIGURED、OFFSITE_COPY_PROTECTION_MISSING、WRITER_NOT_IN_SERVICE、VIRUS_SCANNER_NOT_AVAILABLE 与 LEGAL_ENTITY_KEY_DOMAIN_UNAVAILABLE，其余各类抑制与静音记名记时、台账快照与暴露窗口记录纳入交付验收。
- 第 16 章与附录 A.1 至 A.4：性能与容量认证的度量对象、统计口径、基准数据集、负载模型与全部必判必记项。
- 第 17.2 章：混沌与故障注入六类、备份与完整恢复、审计链与不可变存储、数据保护控制与销毁证明、安全模糊与渗透五类测试；发布缺陷门禁。
- 第 17.3 章：恢复后的全部强制不变量校验作为恢复验收口径的调用与留证。
- 第 17.4 章：供应链安全五项与严重高危分级口径；首版只提供签名离线补丁包，不建设厂商在线更新网关；未启用部署管理通道时用本地清单人工携出，支持套餐回报周期默认 7 天且最长 7 天。
- 第 17.5 章与附录 D：BC-1 单一基线组合、通过判据、认证报告、发布前置项与运行期项划分、四项永久性不符合项封闭清单。
- 附录 A.5 与 A.6：可恢复性判定方式与各类演练判定表全部判据。按裁定 F-11-4，演练为整机失效恢复两次、密钥恢复材料隔离恢复两次，另加保留期尾端恢复一次；该次演练的判定项集合单列，只判 RTO、数据完整性、第 17.3 章全部强制不变量、附件与元数据一致性四项，RPO 一项对其不适用，其备份集判据与新增次数同批进 A.5 的发布判据与本阶段第 9 条退出条件。按裁定 F-08 第一节结论二，全部演练的被测机器固定为 Windows Server 2022；按其第 4.5 节第 4 条，Linux 上的演练记录一条都不能沿用。
- 第 18 章：升级、版本与生命周期。四档时长上限与切换窗口按退出条件 24 逐档实测判定；
  升级前定制兼容测试及其失败阻断效果按退出条件 25 实测；逐次升级/回退证据包十一项要素按退出条件 26 校齐。
  首版唯一分发形态按退出条件 29 固定为生产 Authenticode 签名离线补丁包；受控在线更新网关归未来独立的厂商侧项目，不属于本仓、本实例或首版交付范围，不再等待表态，也不得据此在本仓预留下载/回传通道。
- 第 19 章阶段 4 与第 22 章：退出条件与十五条验收标准的门禁判定与证据包。
- 第 21.4 与第 21.22 章：对外表述与宣传材料的阶段归属固定为阶段 14；交付、认证、验收材料与客户合同逐条按三档裁决，产品负责人对逐条清单及材料摘要签字，第二档在当前版为零、第三档为零，第一档只允许已认证事实且须同条列全前提。该签字进入退出条件 15 与 23 的发布证据包，签字后材料变化即失效。
- 第 21.6、21.13、21.18、21.21 章：单点故障、补丁分发与漏洞响应台账、应用级不可变不等价、物理副本可被操作系统特权者读取四项风险的登记与披露；第 21.21 章另补一句，即未知复制会话检出的覆盖面已收窄为只覆盖持续存在的未知槽与未知会话。

PRD 条目。
- 第 10.6.2 节审计链验证工具：本阶段只提供其运维中心侧的锚定超期状态呈现，工具本体属审计阶段。
- 第 10.6.3 节运维中心的降级与暴露窗口台账：三条用户可见行为要求全部落地。
- 第 11.8 节计划内停机：本阶段提供窗口开始前完成一次通过校验的备份这一前置条件的自动判定。
- 第 11.9 节降级状态的用户可见性：六行规则逐行落地。
- 第 11.10 节：门户请求因资源不足失败的事件不再单列事件类型，统一按 portal-gateway 已有的应用层限流与超时路径计入附录 A.2 的错误率口径，PRD 该条中资源配额限流这一措辞由本阶段提出修订。
- 第 11.11 节诚实披露八条：八条文本与其在界面、部署记录、交付说明三处同时可见的要求。
- 第 11.12 节验收对应关系：六条指向的判据由本阶段的门禁工装逐条判定。
- 附录乙 U-L-10 与 U-L-11：F-51 已按本阶段推荐值冻结并关闭；现行实现值见本文第 1 节，不再标注临时取值或被阻塞状态。

---

### 11. 风险与预留

前三项历史缺口均已关闭，不再构成设计或开工阻断。其一，规格第 7.6、7.10 章的历史数据迁移曾与 DDL `ep-migrate` 同名混淆；现由第 4.12 节的独立 `ep-data-migrate`、25 类模块 writer、六张法人 RLS 台账表和退出条件 27、31 完整承接。A-24 的四条期初通道继续作为对应 writer 的底层唯一写入口，不再被误写为全部历史迁移能力。其二，首版明确不建设厂商受控在线更新网关，只交付签名离线补丁包；未来网关另立厂商侧项目。其三，支持套餐的回报义务已冻结为合同模板参数 `PATCH_STATUS_REPORT_INTERVAL_DAYS`，范围 1 至 7、默认 7；它只在发布前要求合同选择与签字，不要求开发者等待商业人员给值。

历史数据迁移仍有客户交付风险，但已不是方案缺口：源系统质量差时可能连续两轮不收敛。控制固定为第 4.12 节三选一决定、已知差异封闭清单和不可豁免差异零容差；选择缩小范围或只迁期初与未结事项时，遗留历史由客户在原系统自行查询，并把范围、责任与对切换后对账的影响写入交付说明与客户合同。实现方不得以该风险删减四类来源、25 类 writer 或九类对账的产品能力。

风险一，落点回传吞吐决定 RTO 是否成立，而落点由客户提供并运维。控制：部署时与每次更换落点后各实测一次持续读回与持续写入吞吐，写入部署记录；低于认证报告记录值时按第 13.3 章重估 RTO 并在该落点上重做一次整机失效恢复演练，未重做按未验证处理。风险不可消除，只能度量与披露。

风险二，附件正文 800 GB 的每日全量写出与 4 小时 RTO 同时成立的余量有限。控制：认证运行必须按 800 GB 全量计不得抽样；恢复演练的附件写入与校验和计算流式合并以免二次全量读取；实测超出 4 小时时只能上调 RTO 承诺值并同步修订规格与交付材料后重新演练，不得缩减校验范围或改按抽样。

风险三，未知复制会话检出折叠进保留量采样后，覆盖面由尽力检出进一步收窄为只覆盖持续存在的未知槽与未知会话，起止落在两次采样之间的连接检不出。控制：不为此重建任何独立核对通道，也不把它表述为检测手段；该局限与三项遏制手段都不阻止本机特权主体这一结论一并写入交付说明；真正的遏制在操作系统层访问控制与审计，不在应用层，该结论按第 21.21 章披露。接受该风险的理由是原先那套专用核对为一个自认挡不住唯一现实攻击者的边界占用了一条独占分析连接、一张表、一个指标、一个台账 kind 与一对配置键，其代价高于其检出增益。

风险四，归档通道暂停态没有平台侧自愈路径，客户长期不修复落点即长期无新恢复点。控制：告警不可由管理员关闭，台账依据固定为 ArchiveChainBroken，进入该状态起即书面告知客户并写入交付说明；界面文案禁止出现正在恢复一类表述。

风险五，本机不保留可直接读回的全量备份副本，落点与恢复材料同时不可用即无恢复路径。控制：恢复材料按分片、双人控制、每 6 个月核验且不得与其保护的副本同处一个落点，由 ck_key_recovery_materials_not_colocated 在数据库层强制。

风险六，备份、报表、对账与交易共用同一台服务器的磁盘与内存，平台不提供隔离保证，极端情况下备份窗口内交易时延仍可能被拉高。控制：资源单位（具名 Job Object）只落实内存硬上限，逐行按 BC-1 算定绝对字节并由部署校验脚本断言；按权重磁盘 IO 份额无承载，CPU 比例首版固定不启用，两者都不构成隔离保证。PostgreSQL 16 与反向代理的唯一实现路径为 ops-agent 创建资源单位并指派，实机证据形成前状态为 `UNVERIFIED`、不计入覆盖但不切换实现。磁盘 IO 一维归零使风险实质加重，第 13.3 章 RPO 不超过 15 分钟没有机制侧保证，只能由附录 A.4 实测；该降级与“无隔离保证”必须进入交付说明。真正判据是附录 A.2 时延线及其备份窗口子集；容量达到下限 80% 时告警并要求扩容或受控处置，未执行则写部署记录并书面告知客户。

风险七，对象存储落点使写出进程具备出网能力，扩大了攻击面。控制：出向策略的目的地址集合固定为部署记录所载落点，writer 凭据只由写出进程系统账户持有、不下发人类用户、不用于交互式登录、不复用于其他进程；restore 与 disposal 使用另外两个独立且平时封存的身份；该项纳入部署验收核对。

风险八，处置执行不可逆，且同一批对象分散在本机存储与服务器之外落点两侧，任一侧漏处置即销毁证明不成立。控制：OpsDisposalService 把落点侧对象纳入同一次处置范围并逐对象留证，disposed_count 与销毁证明对象引用一并写审计；双人控制、重新认证与第三个独立 disposal 身份在实现内强制，不设跳过开关；密钥销毁与到期备份集销毁一律走该实现，writer/restore 无删除权，其他阶段不得自建销毁路径；落点不可达时拒绝执行而不做部分处置。

风险九，应用服务器被勒索或 writer 凭据被盗后，攻击者可能尝试用合法写出身份清空历史副本。控制：所有对象采用批次唯一不可复用 key 与 CREATE_NEW/`If-None-Match: *`；writer 在对象存储 IAM、Windows/SMB DACL 或经认证的 NFSv4 ACL 上被明确拒绝覆盖、删除、重命名、改权、策略管理与版本清理，并由负向探针持续证明。任一后端做不到创建与删除权限分离时，即使仍可正常写入也打开不可抑制 `OFFSITE_COPY_PROTECTION_MISSING` 并阻止发布。该措施不是 WORM；客户存储管理员、云根账户或另一台机器本地管理员仍可绕过，这一剩余风险必须写入交付说明与客户合同。

为后续版本预留的扩展点，本阶段只留接口不实现。SinkKind 枚举预留经认证的不可变后端与异地在线不可变备份库两个取值位；ArchiveChannelState 预留多副本形态下的槽迁移态；recovery_drills.drill_kind 预留单故障域失效与区域灾难两类演练；部署级备份加密的 AeadAlg 预留商用密码算法位，其切换机制随第 12.3 章延期项恢复；degradation_windows.kind 的枚举以 CHECK 约束表达而非 PostgreSQL enum，按基线第 3.2 节可在线增补取值；ep-adapter-sink 的 trait 方法只用 foundation 类型，后续更换落点后端不触及上层。
