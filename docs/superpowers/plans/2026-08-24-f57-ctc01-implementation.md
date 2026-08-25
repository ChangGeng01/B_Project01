# F-57 CTC-01 Windows Vertical Slice Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Put the first real F-57 customer-to-contract-to-standard-order-to-procurement-to-delivery-to-invoice-to-cash loop through the Control Center, Windows Workbench, one authority, and PostgreSQL, then prove its failure semantics as `DEV_SLICE_GREEN`.

**Architecture:** G3 builds only the two minimum UI planes over generated Control and Employee contracts. G4 adds the smallest governed carrier set, connects the already-tested authority spine and feature handlers through real HTTPS and PostgreSQL, and runs an exact fault matrix; no mobile, portal, full offline, production backup, or P340 claim is pulled into this slice.

**Tech Stack:** Rust/Axum/PostgreSQL 16, generated OpenAPI/TypeScript/UI schema, Tauri 2 + React/TypeScript for the Windows technology slice, Playwright, PowerShell, local file quarantine, Excel/CSV proposal parsing, deterministic effect provider, read-only REST/MCP probe.

**Spec:** `docs/superpowers/specs/2026-08-23-f57-governed-automation-fabric-design.md`

**Status:** `READY_NOT_AUTHORIZED` / `BLOCKED_BY_G1_G2`; planning is complete, but implementation requires separate development authorization and one valid `G2_CTC_DATA_GREEN` aggregate whose payload embeds its same-candidate G0/G1/G2 closure. That aggregate authorizes task start only; G3 must execute its current-candidate predecessor set, while G4 exact-joins same-run L2 terminal records and starts only journal-`ABSENT` predecessor TestIDs on its own current run.

## Global Constraints

- This plan accepts only a verified `G2_CTC_DATA_GREEN` aggregate whose nested G0/G1/G2 references exact-match candidate identity, repository tree, graph, generator, baseline/apply/F57 migration manifests, toolchain, and `gate_run_id`. If G1 is displayed separately, it must be the exact G1 artifact referenced inside that G2 payload; a merely same-graph or earlier standalone G1 receipt is historical and cannot satisfy entry. After the tree changes, the aggregate is historical dependency evidence only; `gate g3` creates its explicit signed run journal and obtains a fresh G0…G3 set, while G4 uses one explicit journal to exact-join signed L2 terminal results and starts only missing predecessor TestIDs. A started TestID is never rerun.
- Every `fresh-pg` and G3/G4 gate applies the unchanged signed 69-file pre-F57 baseline before the contiguous `CREATED` F57 suffix. G3 ends at 19 F57 files (`88` total); G4 ends at 20 F57 files (`89` total). The 310 legacy and nine absent baseline paths remain absent.
- Only the 16 exact Employee API method/path pairs and generated Control API may be exposed; no generic object/table route exists.
- Control and Employee transports converge on the same `AuthorityCommandGatewayV1`; neither can set actor, legal entity, policy verdict, SoD verdict, authority epoch, or authenticated device.
- G3/G4 consume G0's canonical foundation nominals without redefining them. Every UUID-bearing run, candidate, command, journal, result, finalization, and recovery field uses the private strict `UuidV1` from `crates/foundation/src/identifier.rs`; raw `uuid::Uuid`, a second wrapper, and uppercase/simple/braced/URN/whitespace aliases are forbidden.
- G3/G4 consume the G1-03 security spine read-only. A Control or Employee adapter may construct only its private authenticated ingress product and call `AuthorityCommandGatewayV1`; it cannot raw-construct, serialize, or accept `VerifiedSecurityContextV1`, `SecurityContextEnvelopeV1`, issuer material, or a backend ticket. `PgUnitOfWork::begin_authorized(&VerifiedSecurityContextV1)` is the sole public database-transaction constructor, and every CTC repository operation stays inside the resulting `AuthorizedPgTx`.
- The minimum Workbench is online-only. `ClientIntentV1` is a versioned seam; no local authoritative business database or conflict engine is created here.
- CTC-01 uses `STANDARD`, `source=CONTRACT_VERSION`, and procurement source `SALES_ORDER`. `DROP_SHIP` and the six-source procurement matrix remain G5 work.
- A contract attachment is quarantined on HDD, digest-bound, scanned, and published before it can become evidence.
- G4 closes `CONTRACT_FULFILMENT`, `SALES_ORDER_FULFILMENT`, and `RECEIVABLE_COLLECTION` only.
- `PROCUREMENT_FULFILMENT` remains `WAITING`, with sole blocking obligation `PURCHASE_AP_CLOSED` and typed `ProcurementSettlementGapV1` showing all three false fields; this is a required success assertion, not an allowed failure.
- G4 recovery evidence is `SYNTHETIC_DEV_EVIDENCE`; it cannot satisfy backup, ransomware, clean-room, P340, or production requirements.
- macOS/iOS/Android probes may run independently, but their absence does not block this plan and their success does not create a distributable product claim.
- Normative Files-list expansion: every task below that creates a `db/migrations/**` file also modifies `docs/f57-migration-reservations.v2.tsv`, changes exactly its own reservation row to `CREATED`, and includes that registry in the same task-stage commit as the SQL. A pre-commit Fresh PG run is engineering rehearsal only; the clean-HEAD G3/G4 gate reruns the complete due prefix and issues the candidate-bound receipt. This rule applies even where a repeated Files line omits the registry path.
- Every task runs in its own clean F-57 worktree, begins with `cargo xtask f57 task begin --task <exact-id>`, and commits only after `task stage` plus `task verify-staged`. Raw `git add`, directory/glob staging, pre-task dirty paths, and cached-set drift are forbidden.
- G0 owns every canonical Requirement test facade. Task 4 creates only `testkit/src/f57_cases/g4/customer_contract_order.rs`, registers concrete handlers for exact symbols `t_f57_clm_001` and `t_f57_sal_002`, and regenerates the facade manifest. Those are the only Requirements first due at G4; G3 remains a probe-only shell gate. Neither task hand-edits `testkit/tests/f57_customer_contract_order.rs`.
- Before interpreting any F57 evidence signature, each G3/G4 run bootstrap-verifies and materializes the exact committed signed 89-row artifact-signer registry. Candidate identity, candidate manifest and every gate receipt exact-bind the same materialized registry ref; no ambient trusted-certificate fallback is allowed. The removed would-be row is `SIGNED_ARTIFACT_REF_V1/windows-authority`: final release uses the exact internally tagged wire `{"artifact_kind":"WINDOWS_AUTHORITY","authority_artifact_set_ref":<ArtifactRefV1>}` to reference the already signed `WindowsAuthorityArtifactSetV1` directly, and plain `GateReceiptRefV1` adds no row.
- G3/G4 consume, but never redefine, G0's two release-partition byte goldens: Final L2 is the 149 canonical Requirement TestIDs first due in G0…G5 with digest `5ec5a8663b3763bcbf85d84438d6236e9a9680fc0749ad0ca3efa04f124a5a7a`; L3 is the 36 first due in G6 with digest `e7a2fae4fa1a3384c47592ffd0310b3b8717cb595bc5c4397f61e0be2cbc85df`. They are disjoint and union to 185. The six release-carrier auxiliary TestIDs belong to neither partition and may appear only through `carrier_refs`.

---

## 1. Consumed generated contracts

The following are read-only outputs of the G0 graph compiler:

```text
docs/generated/f57/openapi/control-center.v1.yaml
docs/generated/f57/openapi/employee-api.v1.yaml
docs/generated/f57/typescript/index.ts
docs/generated/f57/typescript/types.ts
docs/generated/f57/typescript/client.ts
docs/generated/f57/typescript/manifest.v1.json
docs/generated/f57/ui/control-center.ui-schema.v1.json
docs/generated/f57/ui/employee-workbench.ui-schema.v1.json
```

The clients may wrap generated transport for authentication storage, retry presentation, and device integration, but may not copy or redefine business DTOs.

### Task 1: Build the minimum server-resident Control Center

**Files:**
- Create: `clients/control-center/package.json`
- Create: `clients/control-center/package-lock.json`
- Create: `clients/control-center/tsconfig.json`
- Create: `clients/control-center/vite.config.ts`
- Create: `clients/control-center/src/app/App.tsx`
- Create: `clients/control-center/src/api/authority.ts`
- Create: `clients/control-center/src/features/generations/GenerationDesk.tsx`
- Create: `clients/control-center/src/features/grants/GrantDesk.tsx`
- Create: `clients/control-center/src/features/automation/EffectDesk.tsx`
- Create: `clients/control-center/src/features/runtime/RuntimeStatus.tsx`
- Create: `clients/control-center/src/generated/manifest-link.ts`
- Create: `apps/core-server/src/platform/control_center.rs`
- Create: `apps/core-server/src/platform/static_assets.rs`
- Modify: `apps/core-server/src/platform/mod.rs`
- Create: `crates/adapter/db-pg/src/platform_meta/ui_schema_store.rs`
- Modify: `crates/adapter/db-pg/src/platform_meta/mod.rs`
- Modify: `crates/adapter/db-pg/src/lib.rs`
- Create: `apps/core-server/src/wiring/control_center.rs`
- Modify: `apps/core-server/src/wiring/mod.rs`
- Create: `apps/core-server/build.rs`
- Create: `db/migrations/platform_meta/V20261025091400__platform_meta_create_ui_schema_versions.sql`
- Create: `testkit/tests/f57_minimum_control_center.rs`
- Create: `testkit/tests/f57_control_contract_projection.rs`
- Modify: `apps/core-server/src/main.rs`
- Modify: `apps/core-server/Cargo.toml`
- Modify: `crates/adapter/db-pg/Cargo.toml`
- Modify: `testkit/Cargo.toml`
- Modify: `Cargo.lock`
- Modify: `docs/capability-graph/f57-core.v1.json`
- Regenerate (`GENERATED_MANIFEST_SET`): `docs/generated/f57/projection-manifest.v1.json`

**Interfaces:**
- Consumes: generated Control API/UI schema, `CommandPipeline`, generation store, authorization simulation, objective/effect query, topology/storage health.
- Produces: `control_center::router(state: AuthorityHttpStateV1) -> axum::Router` and a static web application with only generation, grant/simulation, effect decision, objective, and runtime health surfaces enabled.

- [ ] **Step 1: Write the failing transport and projection tests.**

```rust
#[tokio::test]
async fn control_command_cannot_assert_security_context() {
    let response = post_control_json("generation.activate", serde_json::json!({
        "actor_id": "00000000-0000-0000-0000-000000000001",
        "legal_entity_id": "00000000-0000-0000-0000-000000000002"
    })).await;
    assert_eq!(response.status(), 400);
    assert_eq!(response.error_code(), "CONTROL_ENVELOPE_UNKNOWN_FIELD");
}

#[test]
fn control_typescript_manifest_matches_graph_digest() {
    let manifest = generated_typescript_manifest();
    assert_eq!(manifest.graph_digest_sha256, compiled_graph_digest());
    assert_eq!(manifest.openapi_sha256, sha256_file("docs/generated/f57/openapi/control-center.v1.yaml"));
}
```

- [ ] **Step 2: Run tests and verify RED.**

Run: `cargo test -p ep-testkit --test f57_minimum_control_center --test f57_control_contract_projection -- --nocapture`

Expected: FAIL because the Control Center router and client do not exist.

- [ ] **Step 3: Implement the only Control transport and minimum UI.**

```rust
pub async fn execute_control_command(
    axum::extract::State(state): axum::extract::State<AuthorityHttpStateV1>,
    AuthenticatedControlContext(ingress_context): AuthenticatedControlContext,
    axum::Json(command): axum::Json<ControlCommandV1>,
) -> Result<axum::Json<ControlCommandSubmissionResultV1>, AppError> {
    let envelope = IngressCommandEnvelopeV1::project_from_control(command)?;
    let receipt = state.command_gateway.execute(&ingress_context, envelope).await?;
    Ok(axum::Json(ControlCommandSubmissionResultV1::project_exact(receipt)?))
}
```

`CommandReceiptV1` is the G1 internal persisted/idempotency wire and is forbidden from this public signature. `project_exact` is the generated exhaustive projection required by `docs/f57-api-direct-routes.seed.tsv`: it emits only `{correlation_id,authoritative_generation,audit_entry_id,value}`, where `value` is the command row's named closed result type. The generated route error mapper exposes only the seed row's allowlist; unknown internal errors become `PLATFORM.SYSTEM.NOT_READY`. OpenAPI/TypeScript goldens fail if the internal receipt, internal error enum, subject ref or audit hash leaks into Control contracts.

The browser bundle contains no database/KMS credential and no service-control command. Package, remote support, production restore, release signing, and destructive model operations render `DISABLED_NOT_CERTIFIED`. The migration stores signed UI schema version/digest and generation ref only; it stores no arbitrary JavaScript. `ui_schema_store` is its only SQL adapter and `wiring/control_center.rs` composes it through `AuthorizedPgTx`; the store, wiring, and platform module registrations must be reachable in the projection test.

- [ ] **Step 4: Generate, test, and build.**

Run: `cargo xtask f57 graph generate`

Expected: changed Control projection only for reviewed graph nodes.

Run: `cargo xtask f57 fresh-pg --profile G3_CLIENT_SHELL --through 20261025091400`

Expected: PASS on a clean PostgreSQL 16 database through the Control schema migration.

Run: `npm --prefix clients/control-center ci`

Expected: installs exclusively from the lock file and configured offline cache.

Run: `npm --prefix clients/control-center test -- --run`

Expected: PASS.

Run: `npm --prefix clients/control-center run build`

Expected: PASS with a deterministic static asset manifest.

Run: `cargo test -p core-server -p ep-adapter-db-pg -p ep-testkit --all-targets --locked --test f57_minimum_control_center --test f57_control_contract_projection -- --nocapture`

Expected: PASS.

- [ ] **Step 5: Commit the minimum control plane.**

```bash
cargo xtask f57 task stage --task G3-01
cargo xtask f57 task verify-staged --task G3-01
git commit -m "feat: add minimum governed control center"
```

### Task 2: Build the online Windows Workbench and Employee API

**Files:**
- Create: `clients/workbench/package.json`
- Create: `clients/workbench/package-lock.json`
- Create: `clients/workbench/tsconfig.json`
- Create: `clients/workbench/vite.config.ts`
- Create: `clients/workbench/src/app/App.tsx`
- Create: `clients/workbench/src/api/authority.ts`
- Create: `clients/workbench/src/tasks/TaskHome.tsx`
- Create: `clients/workbench/src/features/ctc01/Ctc01Flow.tsx`
- Create: `clients/workbench/src/schema/renderer.tsx`
- Create: `clients/workbench/src/security/endpoint.ts`
- Create: `clients/workbench/tests/g3-shell.conformance.test.ts`
- Create: `clients/workbench/src-tauri/Cargo.toml`
- Create: `clients/workbench/src-tauri/src/main.rs`
- Create: `clients/workbench/src-tauri/tauri.conf.json`
- Create: `apps/core-server/src/platform/employee_api.rs`
- Create: `apps/core-server/src/platform/client_sessions.rs`
- Create: `apps/core-server/src/platform/file_routes.rs`
- Modify: `apps/core-server/src/platform/mod.rs`
- Create: `crates/adapter/db-pg/src/platform_core/client_session_store.rs`
- Modify: `crates/adapter/db-pg/src/platform_core/mod.rs`
- Modify: `crates/adapter/db-pg/src/lib.rs`
- Create: `apps/core-server/src/wiring/employee.rs`
- Modify: `apps/core-server/src/wiring/mod.rs`
- Create: `db/migrations/platform_core/V20261025091500__platform_core_harden_employee_device_state_and_sessions.sql`
- Create: `testkit/tests/f57_windows_workbench_online.rs`
- Create: `testkit/tests/f57_employee_contract_projection.rs`
- Create: `testkit/tests/f57_g3_client_shell_gate.rs`
- Create: `testkit/src/f57_cases/probes/g3_client_shell.rs`
- Create: `testkit/tests/f57_slice_probes_g3_client_shell.rs`
- Create: `xtask/src/f57/g3.rs`
- Modify: `xtask/src/f57/client_conformance.rs`
- Modify: `apps/core-server/src/main.rs`
- Modify: `apps/core-server/Cargo.toml`
- Read/import unchanged: `crates/platform/authz/src/device_signature.rs`
- Read/import unchanged: `docs/schemas/f57-device-command-signature.v1.schema.json`
- Modify: `crates/adapter/db-pg/Cargo.toml`
- Modify: `testkit/Cargo.toml`
- Modify: `xtask/Cargo.toml`
- Modify: `Cargo.toml`
- Modify: `Cargo.lock`
- Modify: `xtask/src/f57/mod.rs`
- Modify: `docs/capability-graph/f57-core.v1.json`
- Regenerate (`GENERATED_MANIFEST_SET`): `docs/generated/f57/projection-manifest.v1.json`

**Interfaces:**
- Consumes: exact 16-route Employee API, G1-02's sole `DeviceCommandSignatureV1`/Ed25519 preimage contract, `ClientGenerationReportV1`, `ClientGenerationDirectiveV1`, generated TypeScript and UI schema.
- Produces: `employee_api::router(state: AuthorityHttpStateV1) -> axum::Router`, signed-schema renderer, Windows client shell, versioned but inactive `ClientIntentV1` seam, and current-candidate `G3_CLIENT_SHELL_GREEN`.

- [ ] **Step 1: Write failing route-closure and non-authority tests.**

```rust
#[test]
fn employee_route_set_is_exactly_the_contract_set() {
    assert_eq!(runtime_employee_routes(), contract_employee_routes());
    assert_eq!(runtime_employee_routes().len(), 16);
}

#[tokio::test]
async fn workbench_cache_never_satisfies_authoritative_query() {
    let client = offline_fixture_with_cached_customer();
    let result = client.authoritative_query("customer.get").await.unwrap_err();
    assert_eq!(result.code(), "AUTHORITY_UNAVAILABLE");
}

#[test]
fn g3_gate_requires_current_candidate_g0_g1_g2_conformance() {
    assert_code(g3_input_with_prior_tree_g2_only(), "GATE_PREREQUISITE_CANDIDATE_MISMATCH");
    assert!(g3_input_with_current_candidate_reruns().promote().is_ok());
}
```

- [ ] **Step 2: Run tests and verify RED.**

Run: `cargo test -p ep-testkit --test f57_windows_workbench_online --test f57_employee_contract_projection --test f57_g3_client_shell_gate -- --nocapture`

Expected: FAIL because Employee routes and Workbench do not exist.

- [ ] **Step 3: Implement online-only generated-schema Workbench.**

```typescript
export async function submitCommand(
  client: GeneratedEmployeeClient,
  envelope: EmployeeCommandEnvelopeV1,
): Promise<EmployeeCommandSubmissionResultV1> {
  const directive = await client.handshake(currentGenerationReport());
  if (directive.compatibility !== "COMPATIBLE") throw new GenerationDirectiveError(directive);
  return client.commands(envelope);
}
```

Endpoint storage contains only approved certificate pin, device key handle, session material, UI schema and bounded non-authoritative projection. `employee_api.rs` is the concrete owner of `EmployeeCommandIngressV1::submit_exact`: it accepts the exact canonical device-signed envelope bytes, rebuilds a private `VerifiedIngressContextV1`, and calls the unique `AuthorityCommandGatewayV1`; it cannot invoke `CommandPipeline` or a repository directly. `client_session_store` is the only SQL adapter for device/session state and `wiring/employee.rs` composes it through `AuthorizedPgTx`. `apps/core-server/src/platform/mod.rs` registers the Employee API, session, and file-route modules in this same task, and the projection test fails if the trait implementation, store, wiring, or any declared route is not compiled into the server. The contract attachment flow is upload-session → chunks → complete → quarantine; complete never means published. Unimplemented discriminators return `DISABLED_NOT_CERTIFIED`, not a generic mutation route. The same graph change moves only the Tauri `G3_SHELL` carrier from `NOT_DELIVERED` to `DELIVERED`, with exact source path `clients/workbench/tests/g3-shell.conformance.test.ts`; Tauri G4/G5 and all Flutter rows remain `NOT_DELIVERED`. The manifest-set regeneration exact-compares all six rows against master §3 before the G3 gate may dispatch the closed `Tauri2G3Shell` recipe.

The Employee adapter projects the internal receipt exactly to the seed-owned `{correlation_id,authoritative_generation,subject_version,audit_entry_id,value}` result before serialization; generated clients never import `CommandReceiptV1`. GET receipt remains the seed's strict `PENDING|COMPLETED` oneOf and is not this submission result. Contract goldens byte-compare both shapes, reject unknown fields and assert that Control/Employee/Portal projections cannot be interchanged.

- [ ] **Step 4: Generate and run Windows client contracts.**

Run: `cargo xtask f57 graph generate`

Expected: Employee, TypeScript, and Workbench schema projections update under one graph digest.

Run: `cargo xtask f57 fresh-pg --profile G3_CLIENT_SHELL --through 20261025091500`

Expected: PASS on a clean PostgreSQL 16 database through the Employee/session migration.

Run: `npm --prefix clients/workbench ci`

Expected: succeeds from lock/offline cache.

Run: `npm --prefix clients/workbench test -- --run`

Expected: PASS.

Run: `npm --prefix clients/workbench run build`

Expected: PASS.

Run: `cargo test -p core-server -p ep-adapter-db-pg -p ep-xtask -p ep-testkit --all-targets --locked --test f57_windows_workbench_online --test f57_employee_contract_projection --test f57_g3_client_shell_gate -- --nocapture`

Expected: PASS on the Windows Server 2022 G3 runner.

- [ ] **Step 5: Commit the minimum employee plane.**

```bash
cargo xtask f57 task stage --task G3-02
cargo xtask f57 task verify-staged --task G3-02
git commit -m "feat: add online Windows workbench"
```

- [ ] **Step 6: Issue G3 only from clean committed HEAD.**

Run: `git status --porcelain=v1`

Expected: no output.

Run: `cargo xtask f57 gate g3 --bundle-root target/f57/evidence --run-journal target/f57/evidence/g3/gate-run.jcs.jsonl --evidence-out target/f57/evidence/g3`

Expected: exit 0, reruns G0/G1/G2/G3 conformance on committed `HEAD`, and writes `target/f57/evidence/g3/client-shell-receipt.v1.json` with `gate=G3_CLIENT_SHELL_GREEN`. The receipt exact-references the run's materialized signed artifact-signer registry and is finalized from its frozen checkpoint before create-new write/bind. Receipt issuance changes no repository file.

### Task 3: Add the minimum governed CTC carriers

**Files:**
- Create: `crates/platform/provider/Cargo.toml`
- Create: `crates/platform/provider/src/lib.rs`
- Create: `crates/platform/provider/src/manifest.rs`
- Create: `crates/platform/provider/src/invocation.rs`
- Create: `crates/platform/import-export/Cargo.toml`
- Create: `crates/platform/import-export/src/lib.rs`
- Create: `crates/platform/import-export/src/proposal.rs`
- Create: `crates/platform/mcp/Cargo.toml`
- Create: `crates/platform/mcp/src/lib.rs`
- Create: `crates/platform/mcp/src/read_only_probe.rs`
- Create: `apps/integration-gateway/src/providers/mod.rs`
- Create: `apps/integration-gateway/src/providers/ctc01.rs`
- Create: `crates/adapter/db-pg/src/platform_meta/provider_store.rs`
- Modify: `crates/adapter/db-pg/src/platform_meta/mod.rs`
- Modify: `crates/adapter/db-pg/src/lib.rs`
- Create: `apps/core-server/src/wiring/provider.rs`
- Modify: `apps/core-server/src/wiring/mod.rs`
- Create: `db/migrations/platform_meta/V20261025091600__platform_meta_create_provider_manifests.sql`
- Create: `testkit/tests/f57_ctc01_carriers.rs`
- Create: `testkit/tests/f57_integration_gateway_no_sql.rs`
- Modify: `Cargo.toml`
- Modify: `Cargo.lock`
- Modify: `apps/core-server/Cargo.toml`
- Modify: `apps/integration-gateway/src/main.rs`
- Modify: `apps/integration-gateway/Cargo.toml`
- Modify: `testkit/Cargo.toml`
- Modify: `docs/capability-graph/f57-core.v1.json`
- Regenerate (`GENERATED_MANIFEST_SET`): `docs/generated/f57/projection-manifest.v1.json`

**Interfaces:**
- Consumes: ADR-0023 `ProviderManifestV1`, `ResourceGrantV1`, capability graph carrier definition, file quarantine, and command proposal port.
- Produces: local attachment carrier, `ImportProposalV1`, deterministic effect simulator, and read-only REST/MCP conformance probe.

- [ ] **Step 1: Write failing no-credential and grant-intersection tests.**

```rust
#[test]
fn integration_gateway_secret_request_set_excludes_authority_secrets() {
    let requested = integration_gateway_requested_secrets();
    assert!(!requested.iter().any(|s| matches!(s.kind, SecretKindV1::Database | SecretKindV1::Kms | SecretKindV1::AuthorityFileRoot)));
}

#[tokio::test]
async fn effect_simulator_denies_field_outside_invocation_intersection() {
    let outcome = invoke_with_field_not_in_grant().await.unwrap_err();
    assert_eq!(outcome.code(), "PROVIDER_FIELD_NOT_GRANTED");
}
```

- [ ] **Step 2: Run tests and verify RED.**

Run: `cargo test -p ep-testkit --test f57_ctc01_carriers --test f57_integration_gateway_no_sql -- --nocapture`

Expected: FAIL because provider/import-export/MCP crates do not exist.

- [ ] **Step 3: Implement the four-carrier exact-set.**

```rust
pub const CTC01_CARRIERS: [CarrierIdV1; 4] = [
    CarrierIdV1::LocalAttachment,
    CarrierIdV1::ExcelCsvImportProposal,
    CarrierIdV1::DeterministicEffectSimulator,
    CarrierIdV1::ReadOnlyRestMcpProbe,
];

pub fn effective_grant(
    package: &PermissionCeilingV1,
    provider: &ProviderManifestV1,
    deployment: &ResourceGrantV1,
    invocation: &InvocationGrantV1,
) -> Result<EffectiveInvocationGrantV1, GrantErrorV1> {
    intersect_four(package, provider, deployment, invocation)
}
```

Excel/CSV produces a validated proposal with row errors and byte digest; it never writes business tables. The MCP probe exposes read-only capability discovery and one synthetic query; write tools remain disabled. Provider success-after-timeout is represented by an injected deterministic scenario so Unknown/reconciliation can be proved. `provider_store` is the only SQL adapter for the manifest table and is composed only into the authority through `AuthorizedPgTx`; integration-gateway has no database dependency or credential. This task closes its own Cargo graph: `core-server` directly declares only the new `ep-platform-provider` plus its already used DB adapter; `integration-gateway` declares `ep-platform-provider|ep-platform-import-export|ep-platform-mcp` and no DB/KMS/file-authority crate; `ep-testkit` declares the three new library crates only for the typed focused tests. The core wiring module is registered and reachable in the same commit—never an unlinked source file or dependency deferred to a later task. The test writes a manifest through the authority command path, reads it back through the store, and proves the gateway consumes only its verified projection.

- [ ] **Step 4: Run carrier and architecture verification.**

Run: `cargo xtask f57 fresh-pg --profile G4_CTC01 --through 20261025091600`

Expected: PASS on a clean PostgreSQL 16 database through the minimum provider manifest migration.

Run: `cargo test -p ep-platform-provider -p ep-platform-import-export -p ep-platform-mcp --all-targets`

Expected: PASS.

Run: `cargo test -p core-server -p integration-gateway -p ep-testkit --all-targets --locked`

Expected: PASS with `core-server` provider wiring reachable, the gateway linked only to the three zero-database carrier crates, the focused tests importing their canonical nominals, and no missing/deferred/path/git Cargo edge or lockfile rewrite.

Run: `cargo test -p ep-testkit --test f57_ctc01_carriers --test f57_integration_gateway_no_sql -- --nocapture`

Expected: PASS.

Run: `cargo xtask archcheck`

Expected: PASS with Integration Gateway zero database/KMS/authority-file capability.

- [ ] **Step 5: Commit carriers.**

```bash
cargo xtask f57 task stage --task G4-01
cargo xtask f57 task verify-staged --task G4-01
git commit -m "feat: add minimum ctc carriers"
```

### Task 4: Connect the exact CTC-01 chain through UI, HTTPS, and PostgreSQL

**Files:**
- Create: `datagen/src/ctc01.rs`
- Create: `testkit/fixtures/ctc01/standard-contract-cycle.v1.json`
- Create: `testkit/tests/f57_ctc01_e2e.rs`
- Create: `testkit/src/f57_cases/g4/customer_contract_order.rs`
- Create: `clients/workbench/e2e/ctc01.spec.ts`
- Create: `clients/control-center/e2e/ctc01-governance.spec.ts`
- Modify: `clients/workbench/src/features/ctc01/Ctc01Flow.tsx`
- Modify: `apps/core-server/src/main.rs`
- Modify: `xtask/src/f57/client_conformance.rs`
- Create: `datagen/src/lib.rs`
- Modify: `datagen/Cargo.toml`
- Modify: `docs/capability-graph/f57-core.v1.json`
- Regenerate: `docs/generated/f57/requirement-test-facades.v1.json`
- Regenerate: `docs/generated/f57/test-manifest.json`
- Regenerate (`GENERATED_MANIFEST_SET`): `docs/generated/f57/projection-manifest.v1.json`
- Regenerate: `testkit/src/f57_cases/generated_bindings.rs`

**Interfaces:**
- Consumes: G2 public commands/facts, G3 clients, G4 carriers, one signed generation, Fresh PostgreSQL 16.
- Produces: one synthetic/de-identified exact chain and objective state evidence.

- [ ] **Step 1: Write the failing full-chain test using only external APIs.**

```rust
#[tokio::test]
async fn ctc01_closes_customer_side_and_honestly_waits_for_procurement_settlement() {
    let run = Ctc01Harness::fresh_pg16().through_http_and_windows_ui().run().await.unwrap();
    assert_eq!(run.objective("CONTRACT_FULFILMENT").state, "CLOSED");
    assert_eq!(run.objective("SALES_ORDER_FULFILMENT").state, "CLOSED");
    assert_eq!(run.objective("RECEIVABLE_COLLECTION").state, "CLOSED");
    let procurement = run.objective("PROCUREMENT_FULFILMENT");
    assert_eq!(procurement.state, "WAITING");
    assert_eq!(procurement.open_obligations, ["PURCHASE_AP_CLOSED"]);
    assert_eq!(procurement.settlement_gap, ProcurementSettlementGapV1 {
        purchase_invoice_recorded: false,
        payable_recognized: false,
        supplier_payment_settled: false,
    });
}
```

- [ ] **Step 2: Run the E2E test and verify RED.**

Run: `cargo test -p ep-testkit --test f57_ctc01_e2e -- --nocapture`

Expected: FAIL at the first missing UI/API-to-handler connection; the test may not be rewritten to call a repository directly.

- [ ] **Step 3: Wire commands and generated UI actions without adding a second business path.**

```rust
pub const CTC01_CHAIN: [&str; 10] = [
    "customer.create",
    "contract.version.activate",
    "sales_order.create_from_contract",
    "sales_order.release_standard",
    "procurement_demand.create_from_sales_order",
    "purchase_order.issue",
    "goods_receipt.record",
    "delivery_evidence.accept",
    "sales_invoice.issue",
    "cash_receipt.allocate",
];
```

Every action uses generated discriminator/payload, server CAS and idempotency. The fixture creates two legal entities so the same run can prove positive isolation and negative cross-entity access. Contract file bytes enter quarantine and only a clean, same-digest published version can satisfy contract evidence. The concrete G4 handler module proves `CLM-001` and `SAL-002`; regeneration changes their handler descriptors from `NOT_DELIVERED` to `DELIVERED` without changing either seed symbol or generated facade body. This graph change also moves only the Tauri `G4_CTC_UI_API` carrier to `DELIVERED`, bound to `clients/workbench/e2e/ctc01.spec.ts` and the closed `Tauri2G4CtcUiApi` recipe; it asserts that Tauri G3 is already delivered, leaves Tauri G5 and all Flutter rows unchanged, and regenerates the six-row conformance manifest through the same manifest-set authority.

- [ ] **Step 4: Run server and browser E2E.**

Run: `cargo test -p ep-testkit --test f57_ctc01_e2e -- --nocapture`

Expected: PASS with three closed objectives, one honestly waiting procurement objective, and no direct handler/repository shortcut.

Run: `npm --prefix clients/control-center run e2e`

Expected: PASS on the Windows G4 runner.

Run: `npm --prefix clients/workbench run e2e`

Expected: PASS on the same candidate and graph digest.

- [ ] **Step 5: Commit the connected slice.**

```bash
cargo xtask f57 task stage --task G4-02
cargo xtask f57 task verify-staged --task G4-02
git commit -m "feat: connect ctc01 Windows vertical slice"
```

### Task 5: Prove the exact fault matrix and issue DEV_SLICE_GREEN

**Files:**
- Create: `testkit/src/ctc01_faults.rs`
- Create: `testkit/tests/f57_ctc01_fault_matrix.rs`
- Create: `testkit/src/f57_cases/probes/g4_ctc01.rs`
- Create: `testkit/tests/f57_slice_probes_g4_ctc01.rs`
- Create: `xtask/src/f57/candidate.rs`
- Create: `xtask/src/f57/l2.rs`
- Create: `crates/platform/release/src/l2.rs`
- Create: `crates/platform/release/tests/l2.rs`
- Modify: `crates/platform/release/src/lib.rs`
- Modify: `crates/platform/release/Cargo.toml`
- Modify: `xtask/Cargo.toml`
- Modify: `Cargo.toml`
- Modify: `Cargo.lock`
- Read/import unchanged: `crates/foundation/src/signature.rs`
- Read/import unchanged: `crates/foundation/src/evidence.rs`
- Read/import unchanged: `crates/foundation/src/identifier.rs`
- Read/import unchanged: `crates/foundation/src/delivery.rs`
- Read/import unchanged: `xtask/src/f57/evidence.rs`
- Create: `scripts/windows/run-l2-candidate.ps1`
- Create after final signing: `scripts/windows/trust/F57_PS_RUN_L2_CANDIDATE_V1.authenticode.json`
- Regenerate after descriptor verification: `crates/platform/powershell-trust/src/generated_registry.rs`
- Regenerate after descriptor verification: `docs/generated/f57/powershell-script-registry.v1.json`
- Regenerate after descriptor verification: `crates/platform/powershell-trust/tests/fixtures/f57-powershell-script-registry-v1-golden.json`
- Read/import unchanged: `docs/evidence/f57-foundation.v1.schema.json`
- Read/import unchanged: `docs/evidence/f57-gate-receipt.v1.schema.json`
- Create: `docs/evidence/f57-integration-candidate.schema.json`
- Create: `docs/evidence/f57-l2-candidate-evidence.schema.json`
- Create: `xtask/tests/fixtures/f57-integration-candidate-development-v1-golden.json`
- Create: `xtask/tests/fixtures/f57-l2-development-slice-v1-golden.json`
- Read: `xtask/tests/fixtures/f57-final-l2-requirement-test-ids-v1-golden.jcs.json`
- Read: `xtask/tests/fixtures/f57-l3-requirement-test-ids-v1-golden.jcs.json`
- Modify: `testkit/src/lib.rs`
- Modify: `xtask/src/f57/cli.rs`
- Modify: `xtask/src/f57/mod.rs`
- Modify: `.github/workflows/ci.yml`

**Interfaces:**
- Consumes: immutable candidate identity including the exact signed-registry envelope digest, the materialized `artifact_signer_registry_ref`, real G3/G4 system, deterministic fault injection, due test registry, G0's exact 149/36 release-partition JCS goldens, the G0 `JournalCheckpointStoreV1`, and the three closed run output stores.
- Produces: `SignedIntegrationCandidateV1`, signed `L2CandidateEvidenceV1`, and current-candidate `DEV_SLICE_GREEN`. Bare `IntegrationCandidateV1` and `L2CandidateEvidencePayloadV1` exist only in memory immediately before the common signed-envelope constructor and are never persisted or supplied to a gate. `IntegrationCandidateV1` exact-includes the candidate-finalization attempt ID and signer-registry ref; `GateReceiptPayloadV1` exact-includes the same registry ref, uses `Vec<GateReceiptRefV1>` for plain typed prerequisites, and embeds the master `ObjectiveClosureBindingV1` vector. `G3_CLIENT_SHELL_GREEN` is already owned and issued by Task 2; the final signed `ReleaseCandidateV1` name is reserved exclusively for G6.
- Rust ownership remains one-way: `xtask/src/f57/candidate.rs` is the G4 tooling owner of the closed integration-candidate family and its constructor; production-linkable `crates/platform/release/src/l2.rs` is the sole Rust owner of the complete three-row L2 nominal/parser/verifier/builder family from G4 onward. `xtask/src/f57/l2.rs` only composes that authority and owns the one G4 receipt orchestration branch; later G5/G6 code imports the same release module and never redeclares or re-exports an L2 type. G0's `xtask/src/f57/evidence.rs` remains the Rust owner of `GateReceiptV1`, `GateReceiptPayloadV1`, `GateReceiptRefV1`, and the `ObjectiveClosureBindingV1` family. Task 5 imports those G0 types and foundation's `SignedBusinessArtifactV1`, strict `UuidV1`, delivery/TestID, candidate/run, artifact/result/probe-result, and Fresh-PG-reference nominals; it never modifies, shadows, copies, or introduces a second constructor for an imported type. The dependency direction is `xtask -> ep-platform-release::l2 -> foundation`; no production crate depends on xtask.
- Schema ownership and imports are exact: `docs/evidence/f57-gate-receipt.v1.schema.json` remains G0's sole schema owner of the receipt/ref/objective-closure family. `docs/evidence/f57-integration-candidate.schema.json` is the sole schema owner for both closed integration-candidate purposes and its only direct import is `f57-foundation.v1.schema.json`. `docs/evidence/f57-l2-candidate-evidence.schema.json` is the sole schema owner for the three legal L2 purpose/target/candidate rows and has exactly two direct imports: foundation plus one relative `$ref` to the G0 gate-receipt schema for the objective-closure family. Neither schema imports security context, requirement-result, client, or release schemas. Each signed root directly composes the foundation four-field envelope exactly once, refines only its local strict payload, closes with draft-2020-12 `unevaluatedProperties=false`, uses `deny_unknown_fields`, and copies no foundation or gate definition; G5/G6 add purpose-specific byte goldens without duplicating owners.

Before the first Windows invocation, this task finishes the script, uses the G0 ceremony-gated signer/TSA to emit `F57_PS_RUN_L2_CANDIDATE_V1.authenticode.json`, and verifies it through the fixed-host/final-handle executor. Any later task that changes the script must re-sign and replace this descriptor; an inherited stale descriptor is a hard failure.

- [ ] **Step 1: Write the failing 14-scenario matrix test.**

```rust
use std::collections::BTreeSet;

#[test]
fn ctc01_fault_registry_is_exact() {
    assert_eq!(CTC01_FAULTS, [
        "IDEMPOTENT_DOUBLE_CLICK", "STALE_CAS_ZERO_WRITE", "TWO_ENTITY_ISOLATION",
        "MID_EXECUTION_REVOCATION", "MAKER_CHECKER", "COMMIT_BOUNDARY_CRASH",
        "CHECKPOINT_RESTART", "PROVIDER_UNKNOWN_NO_RETRY", "RECONCILE_EXACT_ONCE",
        "HDD_YELLOW_DEGRADE", "BACKUP_LOAD_OVERLAP", "DB_FILE_RECOVERY_CUT",
        "QUARANTINE_DIGEST_PUBLICATION", "CASH_REVERSAL_REOPEN",
    ]);
}

#[test]
fn development_candidate_artifact_set_is_exact_and_current() {
    let candidate = build_development_candidate(clean_head()).unwrap();
    assert_eq!(candidate.artifact_ids(), ["windows-authority", "windows-client"]);
    assert!(candidate.all_artifacts_match_identity_and_toolchain());
    assert!(candidate.fresh_pg_receipt().typed_loads_single_run_g4_evidence());
    assert_eq!(candidate.payload.finalization_attempt_id, candidate.journal_candidate_finalization_attempt_id());
    assert_eq!(candidate.payload.artifact_signer_registry_ref, candidate.materialized_signer_registry_ref());
    assert_eq!(candidate.payload.identity.artifact_signer_registry_sha256, candidate.payload.artifact_signer_registry_ref.sha256);
    assert_code(candidate_without_authority(), "CANDIDATE_ARTIFACT_SET_INCOMPLETE");
    assert_code(candidate_with_duplicate_windows_client(), "CANDIDATE_ARTIFACT_DUPLICATE");
    assert_code(candidate_with_old_tree_authority(), "CANDIDATE_ARTIFACT_TREE_MISMATCH");
    assert_code(candidate_with_unbound_or_alternate_fresh_pg(), "CANDIDATE_FRESH_PG_REF_MISMATCH");
}

#[test]
fn development_candidate_and_l2_schema_goldens_are_exact() {
    assert_byte_golden::<SignedIntegrationCandidateV1>(
        "xtask/tests/fixtures/f57-integration-candidate-development-v1-golden.json",
    );
    assert_byte_golden::<L2CandidateEvidenceV1>(
        "xtask/tests/fixtures/f57-l2-development-slice-v1-golden.json",
    );
    assert_eq!(integration_candidate_schema_purposes(), ["DEVELOPMENT_SLICE", "INTEGRATION"]);
    assert_eq!(l2_schema_legal_rows(), master_three_l2_rows());
    assert_eq!(
        integration_candidate_schema_direct_imports().collect::<BTreeSet<_>>(),
        BTreeSet::from(["f57-foundation.v1.schema.json"]),
    );
    assert_eq!(
        l2_candidate_schema_direct_imports().collect::<BTreeSet<_>>(),
        BTreeSet::from([
            "f57-foundation.v1.schema.json",
            "f57-gate-receipt.v1.schema.json",
        ]),
    );
    assert!(candidate_and_l2_signed_roots_compose_foundation_envelope_exactly_once());
    assert!(candidate_and_l2_schemas_copy_no_imported_nominal());
    assert!(candidate_and_l2_uuid_fields_resolve_only_to_foundation_uuid_v1());
}

#[test]
fn ctc01_consumes_the_frozen_release_partition_without_reassignment() {
    let registry = DeliveryRegistryV1::load(repo_root()).unwrap();
    let final_l2 = strict_jcs_test_id_vector(include_bytes!(
        "../../xtask/tests/fixtures/f57-final-l2-requirement-test-ids-v1-golden.jcs.json",
    )).unwrap();
    let l3 = strict_jcs_test_id_vector(include_bytes!(
        "../../xtask/tests/fixtures/f57-l3-requirement-test-ids-v1-golden.jcs.json",
    )).unwrap();
    assert_eq!((final_l2.len(), sha256_jcs(&final_l2)), (
        149,
        hex_sha256("5ec5a8663b3763bcbf85d84438d6236e9a9680fc0749ad0ca3efa04f124a5a7a"),
    ));
    assert_eq!((l3.len(), sha256_jcs(&l3)), (
        36,
        hex_sha256("e7a2fae4fa1a3384c47592ffd0310b3b8717cb595bc5c4397f61e0be2cbc85df"),
    ));
    assert!(final_l2.iter().all(|test_id| !l3.contains(test_id)));
    assert_eq!(canonical_union(&final_l2, &l3), registry.canonical_requirement_test_ids());
    assert!(exact_release_carrier_auxiliary_test_ids().iter().all(|test_id| {
        !final_l2.contains(test_id) && !l3.contains(test_id)
    }));
    assert!(development_l2_fixture().payload.carrier_refs.is_empty());
}

#[test]
fn g4_receipt_prerequisites_and_signer_registry_binding_are_exact() {
    let receipt = issue_g4_receipt(g4_development_aggregate_fixture()).unwrap();
    assert_eq!(receipt.payload.prerequisite_receipts.iter().map(|r| r.gate).collect::<Vec<_>>(), [
        ProgramGateV1::G0BootstrapGreen,
        ProgramGateV1::G1AuthoritySpineGreen,
        ProgramGateV1::G2CtcDataGreen,
        ProgramGateV1::G3ClientShellGreen,
    ]);
    assert!(receipt.payload.prerequisite_receipts.windows(2).all(|pair| {
        gate_receipt_ref_sort_key(&pair[0]) < gate_receipt_ref_sort_key(&pair[1])
    }));
    assert!(receipt.payload.prerequisite_receipts.iter().all(|r| {
        let prerequisite = typed_load_gate_receipt(&r.artifact).unwrap();
        prerequisite.payload.gate == r.gate
            && prerequisite.payload.gate_run_id == receipt.payload.gate_run_id
            && prerequisite.payload.artifact_signer_registry_ref
                == receipt.payload.artifact_signer_registry_ref
            && prerequisite.verifies_registry_signer_and_terminal_bound_event()
    }));
    assert_eq!(receipt.payload.artifact_signer_registry_ref, materialized_signer_registry_ref());
    assert_exact_development_objective_closures(&receipt.payload.objective_closures);
    assert_code(g4_receipt_with_procurement_falsely_closed(), "F57_G4_PROCUREMENT_MUST_WAIT");
    assert_code(g4_receipt_with_missing_procurement_gap(), "F57_OBJECTIVE_CLOSURE_SET_INVALID");
    assert_code(g4_receipt_with_reopened_objective(), "F57_OBJECTIVE_STATE_INVALID");
    assert_code(g4_receipt_with_cross_run_objective_result(), "F57_OBJECTIVE_RESULT_RUN_MISMATCH");
    assert_code(g4_receipt_with_verified_registry_a_but_payload_ref_b(), "F57_SIGNER_REGISTRY_REF_MISMATCH");
    assert_code(g4_receipt_with_signed_artifact_ref_prerequisite(), "F57_GATE_PREREQUISITE_REF_TYPE_INVALID");
    assert_code(g4_receipt_with_cross_run_prerequisite(), "F57_GATE_PREREQUISITE_SET_INVALID");
}

#[test]
fn g4_objective_snapshot_is_selected_by_verified_candidate_context() {
    let development = issue_g4_receipt(g4_development_aggregate_fixture()).unwrap();
    let integration = issue_g4_receipt(g4_integration_aggregate_fixture()).unwrap();
    let final_release = issue_g4_receipt(g4_final_release_aggregate_fixture()).unwrap();
    assert_exact_development_objective_closures(&development.payload.objective_closures);
    assert_exact_closed_objective_closures(&integration.payload.objective_closures);
    assert_exact_closed_objective_closures(&final_release.payload.objective_closures);
    assert_code(g4_integration_with_development_l2_snapshot(), "F57_G4_OBJECTIVE_CONTEXT_MISMATCH");
    assert_code(g4_final_with_integration_l2_purpose(), "F57_G4_OBJECTIVE_CONTEXT_MISMATCH");
    assert_code(g4_closed_procurement_without_three_l2_facts(), "F57_G4_PROCUREMENT_FACT_SET_INVALID");
}
```

- [ ] **Step 2: Run the matrix and verify RED.**

Run: `cargo test -p ep-testkit --test f57_ctc01_fault_matrix -- --nocapture`

Expected: FAIL because fault injectors and L2 evidence do not exist.

- [ ] **Step 3: Implement each fault as an independently named test and Rust-owned L2 verdict.**

```rust
pub fn verified_g4_objective_closures(
    candidate: &VerifiedG4CandidateContextV1,
    l2: &L2CandidateEvidenceV1,
) -> Result<Vec<ObjectiveClosureBindingV1>, GateErrorV1> {
    require_l2_candidate_run_manifest_and_checkpoint_match(candidate, l2)?;
    match (candidate.context_kind(), l2.payload.purpose, l2.payload.target_gate) {
        (G4CandidateContextKindV1::DevelopmentSlice,
         L2EvidencePurposeV1::DevelopmentSlice,
         TargetGateV1::DevSliceGreen) => {
            require_exact_development_objective_closures(&l2.payload.objective_closures)?;
        }
        (G4CandidateContextKindV1::Integration,
         L2EvidencePurposeV1::Integration,
         TargetGateV1::IntegrationGreen) => {
            require_exact_closed_objective_closures_from_l2_results(l2)?;
        }
        (G4CandidateContextKindV1::FinalRelease,
         L2EvidencePurposeV1::FinalRelease,
         TargetGateV1::ReleaseCertified) => {
            require_exact_closed_objective_closures_from_l2_results(l2)?;
        }
        _ => return Err(GateErrorV1::ObjectiveContextMismatch),
    }
    Ok(l2.payload.objective_closures.clone())
}

pub fn issue_g4_receipt(input: G4AggregateInputV1) -> Result<GateReceiptV1, GateErrorV1> {
    require_exact_pass_set(&input.results, &CTC01_FAULTS)?;
    let due_results = exact_current_profile_due_results(&input, DeliveryProfileV1::G4Ctc01)?;
    let probe_results = exact_current_profile_probe_results(&input, DeliveryProfileV1::G4Ctc01)?;
    let verified_candidate = input.verified_candidate_context()?;
    let candidate_manifest_ref = verified_candidate.artifact_ref();
    let candidate_identity_sha256 = sha256_jcs(verified_candidate.identity())?;
    let gate_run_id = verified_candidate.gate_run_id();
    let fresh_pg_receipt = verified_candidate.fresh_pg_receipt().clone();
    input.require_candidate_l2_journal_and_receipts_match(
        &verified_candidate,
        candidate_identity_sha256,
        gate_run_id,
    )?;
    let verified_l2 = input.verified_l2_for_candidate_context(&verified_candidate)?;
    let objective_closures = verified_g4_objective_closures(&verified_candidate, &verified_l2)?;
    let finalization = input.verified_evidence_finalization(
        EvidenceEnvelopeKindV1::GateReceiptG4,
    )?;
    input.verified_artifact_signer_registry.require_signing_row(
        "GATE_RECEIPT_V1",
        "DEV_SLICE_GREEN",
        input.gate_signer,
    )?;
    let artifact_signer_registry_ref =
        input.verified_artifact_signer_registry.artifact_ref();
    require_exact_registry_identity_binding(
        verified_candidate.identity(),
        &artifact_signer_registry_ref,
        &input.verified_artifact_signer_registry,
    )?;
    let payload = GateReceiptPayloadV1 {
        schema_version: 1,
        purpose: GateReceiptPurposeV1::GateReceipt,
        gate: ProgramGateV1::DevSliceGreen,
        evidence_class: GateEvidenceClassV1::SyntheticDev,
        candidate_binding: GateCandidateBindingV1::SignedCandidate {
            candidate_manifest_ref,
            candidate_identity_sha256,
        },
        gate_run_id,
        prerequisite_receipts: canonical_prerequisite_gate_refs(&input)?,
        delivery_registry_sha256: input.delivery_registry_sha256,
        artifact_signer_registry_ref,
        first_due_map_sha256: input.first_due_map_sha256,
        due_result_set_sha256: canonical_due_result_set_sha256(&due_results)?,
        probe_result_set_sha256: canonical_probe_result_set_sha256(&probe_results)?,
        test_results: canonical_test_result_refs(&due_results)?,
        probe_results: canonical_probe_result_refs(&probe_results)?,
        objective_closures,
        run_journal_checkpoint_ref: finalization.frozen_input_checkpoint_ref,
        fresh_pg_receipt,
        issued_at_unix_ms: finalization.issued_at_unix_ms,
        expires_at_unix_ms: finalization.expires_at_unix_ms,
    };
    Ok(sign_business_artifact_v1(input.gate_signer, payload)?)
}

pub fn finalize_development_candidate(
    input: G4CandidateFinalizationInputV1,
) -> Result<SignedIntegrationCandidateV1, CandidateErrorV1> {
    let finalization = input.verified_candidate_finalization(
        CandidateManifestKindV1::IntegrationDevelopmentSlice,
    )?;
    input.verified_artifact_signer_registry.require_signing_row(
        "INTEGRATION_CANDIDATE_V1",
        "DEVELOPMENT_SLICE",
        input.candidate_signer,
    )?;
    require_exact_registry_identity_binding(
        &input.identity,
        &input.artifact_signer_registry_ref,
        &input.verified_artifact_signer_registry,
    )?;
    let payload = IntegrationCandidateV1 {
        schema_version: 1,
        purpose: IntegrationCandidatePurposeV1::DevelopmentSlice,
        gate_run_id: input.gate_run_id,
        finalization_attempt_id: finalization.finalization_attempt_id,
        identity: input.identity,
        fresh_pg_receipt: input.fresh_pg_receipt,
        precursor_journal_checkpoint_ref: finalization.frozen_input_checkpoint_ref,
        artifact_signer_registry_ref: input.artifact_signer_registry_ref,
        artifacts: canonical_development_artifacts(input.windows_authority, input.windows_client)?,
        data_classification: CandidateDataClassificationV1::Synthetic,
    };
    Ok(sign_business_artifact_v1(input.candidate_signer, payload)?)
}
```

Before construction, `issue_g4_receipt` recomputes `delivery_registry_sha256` and `first_due_map_sha256` from the same typed `DeliveryRegistryV1`, exact-matches them to every current-run prerequisite receipt, and requires the latter to equal the frozen `a9547557f95a3a9892efa9f6751a0dd03accac65da344aa559a3203488fee086`; it derives manifest ref, identity digest, run ID and Fresh-PG ref only from the typed-verified candidate context and exact-matches L2/journal/receipts. It derives `artifact_signer_registry_ref` only from the bootstrap-verified materialized registry, then exact-matches that ref to candidate identity; a second caller ref is not an input. `prerequisite_receipts` is exactly four plain `GateReceiptRefV1 { gate, artifact }` rows for G0…G3, sorted by `(gate ordinal,artifact.uri,artifact.sha256)`. Each artifact typed-loads the corresponding `GateReceiptV1` and exact-matches the declared gate, candidate run and the same materialized signer-registry ref, then verifies its registry-selected signer and terminal envelope-bound event; a `SignedArtifactRefV1` wrapper, bare artifact vector, duplicate, missing/extra gate or cross-run ref fails. This wrapper adds no signer-registry row.

Every G4 receipt contains exactly two canonical current-profile `test_results`, exactly 36 current-profile `probe_results`, and exactly four canonical `objective_closures`; only the verified candidate/L2 context selects their state. `DevelopmentSlice` candidate + Development L2 yields Contract/SalesOrder/Receivable `CLOSED` and Procurement `WAITING` with only `PURCHASE_AP_CLOSED` open, the exact three-false `ProcurementSettlementGapV1`, no procurement facts and `NOT_REQUIRED` review. Integration candidate + Integration L2 and final `ReleaseCandidateV1` + Final L2 each yield the byte-identical four-`CLOSED` vector already frozen by that L2. Their Procurement row has exactly the three typed facts and distinct-reviewer proof whose signed result refs are members of that bound L2's `test_results`; it never derives them from the G4 two-result due set or an earlier prerequisite receipt. A wrong candidate purpose, L2 purpose/target, WAITING/CLOSED snapshot or cross-input result fails `F57_G4_OBJECTIVE_CONTEXT_MISMATCH`. Every objective exact-matches the same candidate run and current generation; string-only status queries cannot construct it. The freshly emitted G0…G3 prerequisite receipts carry their own `2/19/0/0` due sets, `2/6/26/5` probe sets and `objective_closures=[]`, so the chain—not result-lane mixing—is the cumulative proof. Once all G0…G4 due/probe/objective work is terminal, `JournalCheckpointStoreV1` create-new writes the latest signed prefix at its exact sequence-derived path. The journal then durably appends one `EVIDENCE_ENVELOPE_FINALIZATION_STARTED` for `GateReceiptG4`; its frozen checkpoint, issue time and bounded expiry are the only scalars used by the constructor. Expiry is the minimum of the G4 policy cap and every consumed time-bearing prerequisite/result/probe/objective/Fresh-PG/checkpoint expiry plus the journal expiry; immutable candidate/static refs are excluded. The receipt includes the exact registry-derived `artifact_signer_registry_ref`, and its signer must match the registry's `GATE_RECEIPT_V1/DEV_SLICE_GREEN` row before signing. `ep-platform-release::l2` owns the sole L2 builder/verifier; `xtask/src/f57/l2.rs` owns only the `issue_g4_receipt` orchestration and G4 golden family. G5 Task 9 and G6 Task 15 must modify/call this composition branch and may not introduce a second G4 or L2 constructor. The scalar fields shown in `G4AggregateInputV1` are verified derivations, never caller authority. Crash-before-commit produces zero state/fact/audit/Outbox/receipt; crash-after-commit with response loss returns the same receipt. Unknown prohibits material retry and closure until an independent query produces exactly one reconciliation fact. HDD yellow pauses low-priority report/import work but never WAL, audit, receipt, or interactive save. The recovery cut restores PostgreSQL and attachment bytes to the same cut and is labeled synthetic.

Implement `cargo xtask f57 candidate build --candidate <git-rev> --bundle-root <path> --run-journal <path> --out <path>` here. It proves a clean commit and canonical paths, bootstrap-verifies the committed 89-row registry, materializes its exact signed envelope at `inputs/<sha256>.json`, and puts that digest in `CandidateIdentityV1` before atomically creating or resuming the signed journal header's one CSPRNG-generated nonnil foundation `UuidV1` `gate_run_id`. It invokes the registered locked/offline build internally for the current-HEAD Windows authority and G4 Windows Workbench and hashes their exact bytes; it accepts neither authority bytes nor a run ID from argv. CandidateBound Fresh-PG is the sole side-effecting evidence operation: before touching its disposable PostgreSQL database, the journal durably appends `EVIDENCE_OPERATION_STARTED{artifact_kind=CANDIDATE_BOUND_FRESH_PG,execution_attempt_id=<one CSPRNG-generated nonnil foundation UuidV1>,start_context_refs=[]}`. Its payload repeats that ID, is create-new written through `EvidenceEnvelopeStoreV1`, fsynced and typed-reloaded, then reaches `COMPLETED`; recovery goes through `UNKNOWN` and may only reconcile the original database/output into `RECONCILED`, never rerun it.

After authority/client/Fresh-PG precursor inputs are durable and terminal, `JournalCheckpointStoreV1` signs and stores the complete precursor checkpoint at `runs/<gate_run_id>/checkpoints/<20-digit-last_sequence>.v1.json`. The journal then appends exactly one five-field `CANDIDATE_MANIFEST_FINALIZATION_STARTED{candidate_kind=INTEGRATION_DEVELOPMENT_SLICE,finalization_attempt_id,frozen_input_checkpoint_ref,generation_observed_selection_ref=null,issued_at_unix_ms}`. `IntegrationCandidateV1.finalization_attempt_id` and `precursor_journal_checkpoint_ref` repeat that record; `artifact_signer_registry_ref` names the materialized registry and its digest exact-matches `identity.artifact_signer_registry_sha256`. The registry-selected `INTEGRATION_CANDIDATE_V1/DEVELOPMENT_SLICE` signer writes exactly `{windows-authority,windows-client}` through the G4 `CandidateManifestStoreV1` path `integration-candidate.v1.json`; only after fsync and typed reload may `CANDIDATE_MANIFEST_BOUND` repeat the same finalization ID and ref. Recovery may perform the first signing from frozen inputs or adopt exact unbound bytes through `CANDIDATE_MANIFEST_RECONCILED`; it never re-signs, changes frozen fields, generates another ID, scans, renames or overwrites.

L2 typed-loads that explicit bound candidate and journal, recomputes its exact envelope SHA and separate identity SHA, and `verify --level l2 --candidate` exact-matches only `candidate_manifest_ref.sha256`; source-building commands retain their separately frozen `<git-rev>` grammar. Every genuinely absent Windows/CTC/Workbench TestID gets one durable `TEST_STARTED{execution_attempt_id,start_context_refs=[]}` before invocation, writes its typed result only through `TestResultStoreV1`, and reaches `COMPLETED` or `UNKNOWN -> RECONCILED` with the same ID; a started TestID is never executed again. Its `L2CandidateEvidencePayloadV1.objective_closures` is the exact verified four-row vector selected by its legal purpose/target/candidate row, never omitted/default. After all L2 inputs are terminal, another stored checkpoint strictly extends the candidate precursor, then one `EVIDENCE_ENVELOPE_FINALIZATION_STARTED{artifact_kind=L2_DEVELOPMENT_SLICE,...}` freezes its checkpoint and times before `EvidenceEnvelopeStoreV1` writes/binds `L2CandidateEvidenceV1`. `gate g4` follows the same checkpoint/finalization/bind protocol for each freshly emitted G0…G4 receipt, typed-verifies candidate/L2/registry/run/objective equality, joins terminal L2 results, and starts only genuinely absent predecessor TestIDs. Thus the three run output stores remain disjoint—TestID results, aggregate envelopes, candidate manifests—while all checkpoint refs come only from `JournalCheckpointStoreV1`. Empty/duplicate/unknown artifacts, stale tree/graph/toolchain, wrong/missing registry, missing authority, unsigned payload, alternate Fresh-PG, missing/spoofed/reopened/cross-run objective row, noncanonical store path, second finalization, changed frozen time/checkpoint, conflicting output, duplicate/reexecuted TestID, bare L2 payload or earlier-tree prerequisite receipt fails closed. The optional four-client branch remains unavailable until G5; `candidate freeze` remains `NOT_DELIVERED`/70 until G6 Task 14 and, when delivered, must use `ReleaseArtifactRefV1::WindowsAuthority { authority_artifact_set_ref }` directly rather than a `SignedArtifactRefV1` wrapper or authority-artifact argv.

- [ ] **Step 4: Run pre-commit Fresh PG and fault verification.**

Run: `cargo xtask f57 fresh-pg --profile G4_CTC01 --through 20261025091600`

Expected: engineering rehearsal PASS through the exact 69-file baseline plus all 20 G0–G4 F57 suffix migrations (`89` total) on a clean PostgreSQL 16 database.

Run: `cargo test -p ep-testkit --test f57_ctc01_fault_matrix -- --nocapture`

Expected: 14/14 PASS. No L2 or gate receipt is issued from the dirty worktree.

- [ ] **Step 5: Commit L2 certification implementation.**

```bash
cargo xtask f57 task stage --task G4-03
cargo xtask f57 task verify-staged --task G4-03
git commit -m "test: certify ctc01 dev slice"
```

- [ ] **Step 6: Build L2 and issue G4 only from clean committed HEAD.**

Run: `git status --porcelain=v1`

Expected: no output.

Run on Windows through the G0 fixed-host/final-handle trust executor: `cargo run -p powershell-trust-tool --locked -- execute --script-id F57_PS_RUN_L2_CANDIDATE_V1 -- -TargetGate DEV_SLICE_GREEN -BundleRoot target/f57/evidence -RunJournal target/f57/evidence/g4/gate-run.jcs.jsonl`. The executor inserts the canonical absolute `-File`, `-NoProfile -NonInteractive -ExecutionPolicy AllSigned`, rejects any other argument shape, and requires the current same-ID descriptor generated after this task's final script edit.

Expected: calls `cargo xtask f57 candidate build --candidate HEAD --bundle-root target/f57/evidence --run-journal target/f57/evidence/g4/gate-run.jcs.jsonl --out target/f57/evidence/g4/integration-candidate.v1.json`; that command internally builds/hashes exactly the current-HEAD `windows-authority` and `windows-client` artifacts, creates or resumes the run ID, materializes the signed signer registry, completes the one Fresh-PG execution attempt, stores its precursor checkpoint, freezes one candidate `finalization_attempt_id`, and binds the manifest through `CandidateManifestStoreV1`. The script hashes the exact signed manifest bytes for `<candidate-manifest-sha256>` and separately verifies the embedded identity/registry digests, runs `cargo xtask f57 verify --level l2 --candidate <candidate-manifest-sha256> --candidate-manifest target/f57/evidence/g4/integration-candidate.v1.json --bundle-root target/f57/evidence --run-journal target/f57/evidence/g4/gate-run.jcs.jsonl --out target/f57/evidence/g4/l2-evidence.v1.json`, then invokes `cargo xtask f57 gate g4 --candidate-manifest target/f57/evidence/g4/integration-candidate.v1.json --l2-evidence target/f57/evidence/g4/l2-evidence.v1.json --bundle-root target/f57/evidence --run-journal target/f57/evidence/g4/gate-run.jcs.jsonl --evidence-out target/f57/evidence/g4`; L2 and the gate each store a frozen checkpoint, append one finalization record, write through `EvidenceEnvelopeStoreV1`, and bind the exact bytes. The gate exact-joins terminal L2-owned IDs and starts only absent G0…G4 IDs. It writes the typed L2 evidence plus `target/f57/evidence/g4/dev-slice-receipt.v1.json` without modifying the repository. Candidate, L2, receipt, TestID/result pairs, signer-registry ref and strictly extending checkpoints exact-match one `CandidateRunIdentityV1`.

Run: `cargo xtask f57 evidence verify --receipt target/f57/evidence/g4/dev-slice-receipt.v1.json --bundle-root target/f57/evidence --expect-gate DEV_SLICE_GREEN`

Expected: PASS with a fresh current-candidate G0/G1/G2/G3 plain typed `GateReceiptRefV1` prerequisite set and G4 aggregate sharing committed `HEAD`, graph, generator, baseline/apply/F57 migration manifests, exact counts `69+20=89`, toolchain, Windows client, authority digests, one materialized signed artifact-signer registry, and one `gate_run_id`.

## CTC-01 read-only completion check

```bash
cargo xtask f57 graph generate --check
cargo xtask archcheck
cargo xtask sqlcheck
cargo xtask f57 fresh-pg --profile G4_CTC01 --through 20261025091600
cargo test -p ep-testkit --test f57_minimum_control_center --test f57_windows_workbench_online --test f57_ctc01_carriers --test f57_ctc01_e2e --test f57_ctc01_fault_matrix
```

Then verify the already issued receipt without invoking L2 or G4 again:

`cargo xtask f57 evidence verify --receipt target/f57/evidence/g4/dev-slice-receipt.v1.json --bundle-root target/f57/evidence --expect-gate DEV_SLICE_GREEN`

Expected final state: the existing `DEV_SLICE_GREEN` receipt verifies read-only. The UI and chain are runnable on synthetic/de-identified data; no four-platform, portal, complete procurement settlement, production installer/signing, P340, backup, ransomware, or production claim is allowed.
