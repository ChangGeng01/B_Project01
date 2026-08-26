# F-57 G0 Bootstrap Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Establish the development authority, single capability graph, feature-first architecture gate, signed generation shell, Windows topology contract, and honest L0/L1 evidence needed before any F-57 business implementation.

**Architecture:** Import the approved seeds once into `CapabilityGraphV1`, canonicalize it, and regenerate every semantic projection under a shared digest. Keep runtime topology and signed generation contracts in mature platform crates, while a Rust-owned `xtask f57` command selects due work and issues a machine-verifiable `G0_BOOTSTRAP_GREEN` receipt.

**Tech Stack:** Rust 2021, Serde/serde_json, SHA-256, PostgreSQL 16 static contracts, Cargo metadata, native Windows Server 2022 profiles, repository TSV/JSON/OpenAPI projections.

**Spec:** `docs/superpowers/specs/2026-08-23-f57-governed-automation-fabric-design.md`

**Status:** `READY_NOT_AUTHORIZED`; this is the first and only plan that may start after separate development authorization.

## Global Constraints

- This plan may start only after explicit development authorization; its present state is `READY_NOT_AUTHORIZED`.
- G0 creates no new F57/business migration and runs no production migration. Task 6 is the sole SQL exception: it corrects exactly three already-present, unpublished pre-release baseline drafts in place, proves the complete 69-file baseline on a disposable PostgreSQL 16 database, and does not claim Windows/P340/client/recovery certification.
- The 185 RequirementIDs, their TestIDs, EvidenceIDs, owner buckets, and activation buckets remain immutable.
- The five API seed families remain controlled import inputs until byte-stable round-trip succeeds; after that point their unchanged bytes become immutable `HISTORICAL_IMPORT_SNAPSHOT` evidence in the same commit. All live API projections then exist only under `docs/generated/f57/` and evolve from CapabilityGraph, never by rewriting the seed snapshots.
- A graph compile with duplicate owner, unknown reference, dependency cycle, missing schema, conflicting projection, or digest drift returns nonzero and produces no activatable output.
- New business code may only enter `crates/features/`; no new `contract/domain/application` triplet is allowed.
- Integration Gateway, Portal Gateway, Extension Host, backup target, and archive target have zero ordinary database credentials; especially `integration-gateway` changes from the historical SQL consumer to a typed-command-only gateway. The separately declared backup/archive writers may hold only their narrow replication/streaming identities and never a business SQL identity.
- Production topology is the closed plain-JCS logical-role declaration/certification chain authenticated transitively by the signed candidate and release certificate, not a signed topology wrapper or fixed process count. Runtime participants may be co-hosted only when trust level and isolation rules permit.
- Every F57 candidate-evidence signature is authorized by the one committed signed 89-row artifact-signer registry. The registry envelope itself is the sole bootstrap exception: its `signer_subject` must be the recomputed F-56 SPKI token and byte-equal verified `DeploymentManifestV1.manifest_signer_subject`, its leaf must additionally match the exact registry-authority DN, and its chain must verify to the deployment-pinned corporate root before any row lookup; DN alone, an ambient trusted certificate, or a wildcard never authorizes.
- Candidate-evidence keys are an explicit execution prerequisite, not a design gap or repository secret. G0-05 implements the sole self-hosted-or-existing trust coordinator/broker and may run all unit/fixture tests without live keys, but it cannot complete its real signed-registry step—and G0-06 cannot issue a signed green receipt—until the product-pinned deployment signing handle plus every generated evidence-role/TSA requirement is successfully prepared or imported, verified and installed. Absence returns `F57_EVIDENCE_TRUST_PREREQUISITE_MISSING` before any placeholder output or journal.
- All commands in this plan run offline after the repository toolchain is provisioned; dependency download is not part of a green gate.
- Every task runs in its own clean F-57 worktree. `cargo xtask f57 task begin|stage|verify-staged --task <task-id>` is the only staging interface; raw `git add`, globs, directory operands, and staging a path dirty before `task begin` are rejected. G0-01 alone uses `task stage --task G0-01 --bootstrap-clean-base <full-commit>` after its brand-new worktree first records an empty porcelain status and the full base commit outside the repository; the command re-proves that worktree/base/allowlist before staging.

---

## 1. Files and responsibilities

| Path | Responsibility |
|---|---|
| `crates/platform/capability-graph/src/model.rs` | Strict graph authoring types and exact enums |
| `crates/platform/capability-graph/src/compiler.rs` | Referential, owner, acyclic, schema, and carrier validation |
| `crates/platform/capability-graph/src/canonical.rs` | Stable sort, canonical JSON, generator identity, SHA-256 digest |
| `crates/platform/capability-graph/src/import.rs` | One-time import of the five current API seed families and 185 requirement bindings |
| `crates/platform/capability-graph/src/projection.rs` | In-memory deterministic projection set |
| `docs/capability-graph/f57-core.v1.json` | Sole reviewed authoring graph after import acceptance |
| `xtask/src/f57/registry.rs` | Strict seed parsing and 185-row delivery profile derivation |
| `xtask/src/f57/generate.rs` | Writes projections only through atomic compare/replace |
| `xtask/src/f57/verify.rs` | L0/L1/gate selection and no-diff checks |
| `xtask/src/f57/evidence.rs` | Gate receipt payload and digest binding |
| `crates/foundation/src/principal.rs` | Sole Rust owner of exact ten-kind `PrincipalKindV1` and full-identity `PrincipalRefV1` |
| `crates/foundation/src/identifier.rs` | Sole Rust owner/validator for strict UUID/digest/SPKI/path and all cross-stage identifier/closed-registry nominals |
| `crates/foundation/src/delivery.rs` | Sole Rust owner of deployment generation, distinct strict-positive objective revision, profile/TestID, and slice-probe ID/assertion nominals |
| `crates/foundation/src/client.rs` | Sole Rust owner of cross-stage client platform/stack/package/lifecycle-fixture nominals |
| `crates/foundation/src/evidence.rs` | Sole Rust owner of runner/candidate/artifact/result/probe-result/Fresh-PG-reference nominals |
| `crates/platform/runtime/src/evidence/object_store.rs` | Sole production-linkable port owner for immutable content-addressed evidence objects and private verified request/root tokens |
| `crates/platform/runtime/src/evidence/input_store.rs` | Sole production-linkable placement port for verified external signed-envelope inputs, with closed media/trust-policy contracts and no domain-verifier dependency |
| `crates/adapter/file/src/evidence_object_store.rs` | Sole filesystem implementation `FileEvidenceObjectStoreV1`; G0 lands the shared engine/tooling-root lane and G1-01 later adds the DATA_HDD authority-root lane without any production dependency on `xtask` |
| `crates/adapter/file/src/evidence_input_store.rs` | Sole filesystem implementation `FileEvidenceInputStoreV1`, sharing the same two staged root lanes and byte engine |
| `docs/evidence/f57-foundation.v1.schema.json` | Unique zero-import schema-DAG root and sole schema owner of all 36 reusable definitions |
| `docs/f57-feature-owner-registry.v1.tsv` | Exact 17-row business fact-owner/crate/schema/repository mapping |
| `docs/f57-platform-mechanism-registry.v1.tsv` | Exact 35-row platform mechanism/crate/authority-scope mapping |
| `docs/f57-task-staged-paths.v1.tsv` | Exact per-task staging allowlist and branch condition registry |
| `docs/f57-migration-baseline.v1.tsv` | Immutable 78-row pre-F57 baseline/absence partition and three draft preimage hashes |
| `docs/generated/f57/migration-apply-manifest.v1.json` | G0-generated deterministic 69-file baseline apply set; it never contains F57 rows and gates sign-bind its digest separately from reservations |
| `docs/f57-artifact-signer-registry.v1.json` | Sole signed 89-row F57 candidate-evidence signer registry and bootstrap trust narrowing input |
| `docs/schemas/f57-artifact-signer-registry.v1.schema.json` | Sole strict schema for the exact five-field registry payload, 89 rows and embedded two-role client-decision archive trust-anchor policy; it composes the foundation-owned detached envelope exactly once |
| `crates/platform/evidence-trust/` | Sole production-linkable candidate-evidence credential requirements, provider port, broker protocol and crash-safe provisioning/rotation coordinator |
| `crates/adapter/kms/src/f57_evidence_trust.rs` | Sole self-hosted OS-keystore/PIV and approved existing-enterprise-signer adapters; both expose the same row-bound verified signer handles |
| `apps/evidence-trust-tool/` | Sole prepare/verify/seal-registry/install/rotate maintenance ceremony CLI |
| `apps/evidence-signing-broker/` | Sole least-privilege fixed-endpoint candidate-evidence signing broker used by gates and platform runners |
| `crates/platform/runtime/src/topology.rs` | Sole Rust owner of the strict plain `RuntimeTopologyDeclarationV1`/`RuntimeTopologyCertificationV1` family plus pure deterministic builders/live declaration verifier; G0 emits no deployment declaration, G1-01 is the first production declaration caller, and G6 Task 14 alone authorizes/persists production certifications |
| `docs/evidence/f57-runtime-topology.v1.schema.json` | Sole schema owner for both plain topology roots, their nested topology vocabulary, two exact media bindings, and their foundation-only import edge |
| `crates/foundation/src/signature.rs` | Generic signed envelope, strict SPKI nominal boundary, stable generated artifact-type/descriptor registry, authorization binding/preparation API, and verifier-only `VerifiedArtifactTrustPolicyV1` plus `VerifiedSignedEnvelopeBytesV1` carrying the exact bytes/media/digest and that non-forgeable policy proof |
| `crates/platform/release/src/generation.rs` | Sole Rust nominal/wire owner of the signed manifest, signed reverse-plan and plain ACK generation family, private manifest proof, generation state machine, and immutable item refs |
| `docs/evidence/f57-generation.v1.schema.json` | Sole schema owner of the exact three generation roots with field counts `13/9/14` and one direct foundation import |
| `crates/platform/release/src/generation_approval.rs` | Sole Rust owner of the seven-field/three-row generation approval registry, bootstrap verifier, private registry proof, and approval-domain verifier |
| `docs/schemas/f57-generation-approval-registry.v1.schema.json` | Sole signed-root schema owner of the generation approval registry and its one direct foundation import |
| `crates/platform/release/src/carrier_contract.rs` | Sole early production-linkable Rust owner of the closed six-value `ReleaseCarrierRecipeIdV1`; G0 dispatcher imports it and G6 later activates mappings without redeclaration |
| `crates/platform/release/src/participant.rs` | Participant activation/ACK persistence behavior importing `GenerationParticipantV1`; it owns no wire nominal or schema |
| `crates/platform/release/src/pin.rs` | Sole portable artifact lease/persistent-reference wire owner |
| `crates/platform/gate-journal-contract/` | Sole production-linkable Rust owner of storage-root binding, complete gate-journal/header/record/checkpoint/prefix nominals, all thirteen reserved later delta variants, strict codec/transition validation, and append/checkpoint ports; foundation-only, with no filesystem or `xtask` dependency |
| `crates/adapter/file/src/gate_run_journal_store.rs` | Sole durable filesystem implementation of the shared journal append/checkpoint ports; both `xtask` and the Windows authority services inject it |

Generated files are committed review artifacts. `cargo xtask f57 graph generate --check` must reconstruct them without modifying bytes.

### Task 1: Freeze the 185-row delivery registry and F57 CLI

**Files:**
- Create: `xtask/src/f57/mod.rs`
- Create: `xtask/src/f57/cli.rs`
- Create: `xtask/src/f57/registry.rs`
- Create: `xtask/src/f57/evidence.rs`
- Create: `xtask/src/f57/verify.rs`
- Create: `xtask/tests/f57_registry.rs`
- Create: `xtask/tests/f57_cli.rs`
- Create: `crates/platform/release/src/carrier_contract.rs`
- Create: `crates/platform/release/tests/carrier_contract.rs`
- Create: `crates/platform/delivery-registry/Cargo.toml`
- Create: `crates/platform/delivery-registry/src/lib.rs`
- Create: `crates/platform/delivery-registry/src/registry.rs`
- Create: `crates/platform/delivery-registry/src/migration_closure.rs`
- Create: `crates/platform/delivery-registry/tests/registry.rs`
- Create: `crates/platform/delivery-registry/tests/migration_closure.rs`
- Create: `crates/platform/delivery-registry/tests/fixtures/migration-closure-identity-v1-golden.json`
- Modify: `crates/platform/release/src/lib.rs`
- Modify: `crates/platform/release/Cargo.toml`
- Create: `xtask/tests/fixtures/f57-gate-receipt-v1-golden.json`
- Create: `xtask/tests/fixtures/f57-objective-closure-distinct-reviewer-v1-golden.json`
- Create: `xtask/tests/fixtures/f57-requirement-evidence-binding-v1-golden.json`
- Create: `xtask/tests/fixtures/f57-slice-probe-evidence-binding-v1-golden.json`
- Create: `xtask/tests/fixtures/f57-final-l2-requirement-test-ids-v1-golden.jcs.json`
- Create: `xtask/tests/fixtures/f57-l3-requirement-test-ids-v1-golden.jcs.json`
- Create: `crates/foundation/src/delivery.rs`
- Create: `crates/foundation/src/principal.rs`
- Create: `crates/foundation/src/identifier.rs`
- Create: `crates/foundation/src/client.rs`
- Create: `crates/foundation/src/evidence.rs`
- Create: `crates/foundation/tests/f57_foundation_wire.rs`
- Create: `crates/foundation/tests/fixtures/f57-foundation-wire-v1-golden.json`
- Modify: `crates/foundation/src/lib.rs`
- Modify: `crates/foundation/Cargo.toml`
- Create: `docs/evidence/f57-foundation.v1.schema.json`
- Create: `docs/evidence/f57-gate-receipt.v1.schema.json`
- Create: `docs/evidence/f57-requirement-evidence-binding.v1.schema.json`
- Create: `docs/evidence/f57-slice-probe-evidence-binding.v1.schema.json`
- Create: `docs/f57-migration-reservations.v2.tsv`
- Create: `docs/f57-delivery-dag.v1.tsv`
- Create: `docs/f57-slice-probe-execution.v1.tsv`
- Create: `docs/f57-fresh-pg-check-registry.v1.tsv`
- Create: `docs/f57-feature-owner-registry.v1.tsv`
- Create: `docs/f57-platform-mechanism-registry.v1.tsv`
- Create: `docs/f57-task-staged-paths.v1.tsv`
- Modify: `xtask/src/main.rs`
- Modify: `xtask/Cargo.toml`
- Modify: `Cargo.toml`
- Modify: `Cargo.lock`
- Read: `docs/f57-task-ownership.seed.tsv`
- Read: `docs/f57-requirement-delivery-profile-overrides.v1.tsv`
- Read: `docs/f57-legacy-migration-disposition.seed.tsv`
- Read: `docs/f57-migration-baseline.v1.tsv`
- Read: `docs/migration-catalog.md`

**Interfaces:**
- Consumes: exact nine-column `docs/f57-task-ownership.seed.tsv`, the base activation-to-profile table in the master plan §4, exact override registry `docs/f57-requirement-delivery-profile-overrides.v1.tsv`, the immutable 78-row migration-baseline registry, the 310-row legacy disposition seed, and the 388-row migration catalog.
- Produces: the zero-import `f57-foundation.v1` schema and its strict direct-import-DAG/offline-closure validator; foundation-owned strict `UuidV1`, primitive/identifier/closed-registry vocabulary, principal/delivery/probe vocabulary, runner/candidate/artifact/result/probe-result/Fresh-PG-reference vocabulary, client vocabulary, and detached-envelope field set listed below; the early production-linkable sole-owner `crates/platform/release/src/carrier_contract.rs` with exact six-value `ReleaseCarrierRecipeIdV1`, allowing the Task-1 parser shell to compile without a local string/enum mirror; and the separate production-linkable sole owner `ep-platform-delivery-registry` for `DeliveryRegistryV1::load(root: &Path) -> Result<DeliveryRegistryV1, RegistryError>`, `DeliveryBindingV1` including `slice_probe_profiles_json`, the exact migration reservation table from the master plan §4.1, the exact 17-row `FeatureOwnerIdV1` registry from master §2.1, the exact 35-row `PlatformMechanismIdV1` registry from master §2.2, the immutable exact 42-row G0–G6 topology DAG from master §5, the master-derived exact 78-row slice-probe execution registry, the master-frozen exact 27-row current Fresh-PG check registry, the canonical test-only Final-L2/L3 Requirement TestID vector goldens derived from the same 185 rows, a complete task-path allowlist and strict baseline/catalog validators. `xtask/src/f57/registry.rs` is composition/CLI only and imports that crate; no production owner, generator, gate, or testkit target imports an xtask module. This task also produces the canonical signed `RequirementEvidenceBindingV1` and `SliceProbeEvidenceBindingV1` schema/golden wires, the six-row `GateCandidateBindingV1` gate/class relation, plain typed `GateReceiptRefV1 { gate, artifact }` prerequisite references, the embedded master `ObjectiveClosureBindingV1` family plus typed distinct-reviewer golden, and strict `cargo xtask f57` parsing. `ep-platform-release` declares direct workspace `serde` in Task 1 because `ReleaseCarrierRecipeIdV1` derives its traits here; relying on a transitive dependency is forbidden.

`ep-platform-delivery-registry::migration_closure` is also the sole production-linkable owner of the exact five-field `MigrationClosureIdentityV1 { schema_version,baseline_registry_sha256,baseline_apply_manifest_sha256,f57_reservation_manifest_sha256,legacy_seed_sha256 }` and its checked constructor. The constructor receives the four already typed registry/manifest values, fixes `schema_version=1`, recomputes each exact-byte digest, and exposes read-only accessors; neither foundation, release nor xtask may redeclare or field-construct it. The final-candidate owner imports this nominal through a direct Cargo edge, and owner/metadata/golden tests reject an xtask-owned copy, raw four-digest constructor, swapped input or second type owner.

- [ ] **Step 1: Write the failing registry tests.**

```rust
#[test]
fn real_registry_has_185_unique_complete_rows() {
    let registry = DeliveryRegistryV1::load(repo_root()).unwrap();
    assert_eq!(registry.rows().len(), 185);
    assert_eq!(registry.unique_requirement_count(), 185);
    assert!(registry.rows().iter().all(|row| {
        !row.capability_id.is_empty()
            && row.release_due_profile == DeliveryProfileV1::G6Release
    }));
}

#[test]
fn base_bucket_then_exact_override_resolves_delivery() {
    assert_eq!(base_first_due_profile("F57-01").unwrap(), DeliveryProfileV1::G0Bootstrap);
    assert_eq!(base_first_due_profile("F57-09").unwrap(), DeliveryProfileV1::G1AuthoritySpine);
    assert_eq!(base_first_due_profile("F57-17").unwrap(), DeliveryProfileV1::G5Integration);
    assert_eq!(base_first_due_profile("F57-18").unwrap(), DeliveryProfileV1::G5Integration);
    assert_eq!(base_first_due_profile("F57-19").unwrap(), DeliveryProfileV1::G4Ctc01);
    assert_eq!(base_first_due_profile("F57-24").unwrap(), DeliveryProfileV1::G6Release);
    assert!(base_first_due_profile("F57-26").is_err());

    let overrides = DeliveryOverrideRegistryV1::load(repo_root()).unwrap();
    assert_eq!(overrides.rows().len(), 57);
    assert_eq!(resolve_delivery("MDM-001", "F57-19", &overrides).unwrap().first_due_profile, DeliveryProfileV1::G5Integration);
    assert_eq!(resolve_delivery("CLM-001", "F57-19", &overrides).unwrap().first_due_profile, DeliveryProfileV1::G4Ctc01);
    assert_eq!(resolve_delivery("CLM-001", "F57-19", &overrides).unwrap().slice_probe_profiles, [DeliveryProfileV1::G2CtcData]);
    assert_eq!(resolve_delivery("CLI-001", "F57-18", &overrides).unwrap().slice_probe_profiles, [DeliveryProfileV1::G3ClientShell, DeliveryProfileV1::G4Ctc01]);
}

#[test]
fn requirement_capability_reference_is_stable() {
    assert_eq!(capability_id_for("GOV-001").unwrap(), "f57.req.gov-001");
    assert_eq!(capability_id_for("DEF-011").unwrap(), "f57.req.def-011");
}

#[test]
fn resolved_first_due_profile_counts_are_frozen() {
    assert_eq!(
        DeliveryRegistryV1::load(repo_root()).unwrap().first_due_counts(),
        [
            (DeliveryProfileV1::G0Bootstrap, 2),
            (DeliveryProfileV1::G1AuthoritySpine, 19),
            (DeliveryProfileV1::G2CtcData, 0),
            (DeliveryProfileV1::G3ClientShell, 0),
            (DeliveryProfileV1::G4Ctc01, 2),
            (DeliveryProfileV1::G5Integration, 126),
            (DeliveryProfileV1::G6Release, 36),
        ]
    );
}

#[test]
fn current_fresh_pg_registry_is_exact_and_old_seed_is_non_authoritative() {
    let registry = FreshPgCheckRegistryV1::load(repo_root()).unwrap();
    assert_eq!(registry.rows().len(), 27);
    assert_eq!(registry.sha256(), hex_sha256("76fed80fcf5f73a64c769cb37f7aadf2d217c813554acc893f7ca875004ce01a"));
    assert_eq!(registry.applicable(G1AuthoritySpine, 20261025090100).len(), 2);
    assert_eq!(registry.applicable(G5Integration, 20261025091930).len(), 20);
    assert_eq!(registry.applicable(G6Release, 20261025092530).len(), 27);
    assert_code(registry_with_duplicate_or_unknown_handler(), "F57_FRESH_PG_REGISTRY_INVALID");
    assert_code(runtime_attempt_to_load_old_fresh_pg_seed(), "F57_HISTORICAL_INPUT_NOT_EXECUTABLE");
}

#[test]
fn resolved_first_due_requirement_sets_are_frozen() {
    let registry = DeliveryRegistryV1::load(repo_root()).unwrap();
    assert_eq!(registry.first_due_sets(), expected_first_due_sets_from_master_section_4());
    assert_eq!(
        registry.canonical_first_due_map_sha256(),
        "a9547557f95a3a9892efa9f6751a0dd03accac65da344aa559a3203488fee086"
    );
    assert!(registry.first_due_ids(DeliveryProfileV1::G2CtcData).is_empty());
    assert!(registry.first_due_ids(DeliveryProfileV1::G3ClientShell).is_empty());
}

#[test]
fn final_release_requirement_partition_byte_goldens_are_exact_and_auxiliary_free() {
    let registry = DeliveryRegistryV1::load(repo_root()).unwrap();
    let final_l2 = registry.canonical_requirement_test_ids_through(
        DeliveryProfileV1::G5Integration,
    );
    let l3 = registry.canonical_requirement_test_ids_first_due_at(
        DeliveryProfileV1::G6Release,
    );
    let final_l2_jcs = canonical_json_bytes(&final_l2).unwrap();
    let l3_jcs = canonical_json_bytes(&l3).unwrap();

    assert_eq!(final_l2.len(), 149);
    assert_eq!(l3.len(), 36);
    assert_eq!(final_l2_jcs.as_slice(), include_bytes!(
        "fixtures/f57-final-l2-requirement-test-ids-v1-golden.jcs.json",
    ));
    assert_eq!(l3_jcs.as_slice(), include_bytes!(
        "fixtures/f57-l3-requirement-test-ids-v1-golden.jcs.json",
    ));
    assert_eq!(sha256(&final_l2_jcs), hex_sha256(
        "5ec5a8663b3763bcbf85d84438d6236e9a9680fc0749ad0ca3efa04f124a5a7a",
    ));
    assert_eq!(sha256(&l3_jcs), hex_sha256(
        "e7a2fae4fa1a3384c47592ffd0310b3b8717cb595bc5c4397f61e0be2cbc85df",
    ));
    assert!(final_l2.iter().all(|test_id| !l3.contains(test_id)));
    assert_eq!(canonical_union(&final_l2, &l3), registry.canonical_requirement_test_ids());
    assert_eq!(exact_release_carrier_auxiliary_test_ids().len(), 6);
    assert!(exact_release_carrier_auxiliary_test_ids().iter().all(|test_id| {
        !final_l2.contains(test_id) && !l3.contains(test_id)
    }));
}

#[test]
fn feature_owner_registry_is_the_exact_master_set() {
    let owners = FeatureOwnerRegistryV1::load(repo_root()).unwrap();
    assert_eq!(owners.rows(), expected_feature_owners_from_master_section_2_1());
    assert_eq!(owners.rows().len(), 17);
    assert!(owners.resolve("finance").is_err());
    assert!(owners.resolve("invoice").is_err());
    assert!(owners.resolve("operating-ledger").is_ok());
}

#[test]
fn platform_mechanism_registry_is_the_exact_master_set() {
    let owners = PlatformMechanismRegistryV1::load(repo_root()).unwrap();
    assert_eq!(owners.rows(), expected_platform_mechanisms_from_master_section_2_2());
    assert_eq!(owners.rows().len(), 30);
    assert!(owners.resolve("platform.identity").is_ok());
    assert!(owners.resolve("platform.flow").is_ok());
    assert!(owners.resolve("identity").is_err());
    assert!(owners.resolve("platform.unregistered").is_err());
}

#[test]
fn every_executable_task_has_a_closed_safe_staging_set() {
    let paths = TaskPathRegistryV1::load(repo_root()).unwrap();
    assert_eq!(paths.task_ids(), exact_task_ids_from_the_five_current_plans());
    assert!(paths.rows().iter().all(|row| row.path.is_repo_relative_normalized()));
    assert!(paths.rows().iter().all(|row| !row.path.has_glob_or_parent_segment()));
    assert!(paths.existing_tree_rows_are_exact_files());
}

#[test]
fn delivery_dag_is_the_exact_immutable_topology_not_a_status_ledger() {
    let dag = DeliveryDagV1::load(repo_root()).unwrap();
    assert_eq!(dag.header(), ["node_id", "requires_json", "produces_json", "migration_versions_json", "condition"]);
    assert_eq!(dag.rows(), expected_42_rows_from_master_section_5());
    assert_eq!(dag.rows().len(), 42);
    assert!(dag.is_acyclic());
    assert!(dag.has_no_status_column());
    assert!(dag.migrations_exact_join_reservations());
    assert!(dag.conditional_producers_are_mutually_exclusive());
}

#[test]
fn bootstrap_clean_base_option_is_closed_to_g0_01() {
    assert!(parse_f57(["task", "stage", "--task", "G0-01", "--bootstrap-clean-base", full_lowerhex_commit()]).is_ok());
    assert_parse_exit_2(["task", "stage", "--task", "G0-01"]);
    assert_parse_exit_2(["task", "stage", "--task", "G0-02", "--bootstrap-clean-base", full_lowerhex_commit()]);
    assert_parse_exit_2(["task", "stage", "--task", "G0-01", "--bootstrap-clean-base", "HEAD"]);
}

#[test]
fn client_gate_require_has_one_explicit_root_and_no_escape_or_extra_option() {
    assert!(parse_f57([
        "client-gate", "require",
        "--decision", "TAURI2_CERTIFIED",
        "--receipt", "target/f57/evidence/g5/client-stack-decision.v1.json",
        "--bundle-root", "target/f57/evidence",
    ]).is_ok());
    assert_parse_exit_2([
        "client-gate", "require",
        "--decision", "TAURI2_CERTIFIED",
        "--receipt", "target/f57/evidence/g5/client-stack-decision.v1.json",
    ]);
    assert_parse_exit_2([
        "client-gate", "require",
        "--decision", "TAURI2_CERTIFIED",
        "--receipt", "../client-stack-decision.v1.json",
        "--bundle-root", "target/f57/evidence",
    ]);
    assert_parse_exit_2([
        "client-gate", "require",
        "--decision", "TAURI2_CERTIFIED",
        "--receipt", "target/f57/evidence/g5/client-stack-decision.v1.json",
        "--bundle-root", "target/f57/evidence",
        "--unknown", "x",
    ]);
}

#[test]
fn conditional_task_paths_exact_match_stack_decision_wire() {
    assert!(task_condition("G5-02A", signed_stack_decision("TAURI2_CERTIFIED")).is_ok());
    assert!(task_condition("G5-02B", signed_stack_decision("FLUTTER_RUST_REQUIRED")).is_ok());
    assert_code(task_condition("G5-02B", signed_stack_decision("TAURI2_REJECTED")), "F57_TASK_CONDITION_UNKNOWN");
}

#[test]
fn annotated_file_actions_normalize_without_losing_path_or_condition() {
    assert_eq!(
        parse_file_action("Create (`NEW_TREE`): clients/workbench/android").unwrap(),
        normalized("Create", "clients/workbench/android", "NEW_TREE", "TASK_DEFAULT"),
    );
    assert_eq!(
        parse_file_action("Create (`GENERATED_MANIFEST_SET`): docs/generated/f57/projection-manifest.v1.json").unwrap(),
        normalized("Create", "docs/generated/f57/projection-manifest.v1.json", "GENERATED_MANIFEST_SET", "TASK_DEFAULT"),
    );
    assert_eq!(
        parse_file_action("Modify (`TAURI2_CERTIFIED`): clients/workbench/src/app/App.tsx").unwrap(),
        normalized("Modify", "clients/workbench/src/app/App.tsx", "EXACT_FILE", "TAURI2_CERTIFIED"),
    );
    assert_code(parse_file_action("Create (`UNKNOWN`): x"), "F57_TASK_PATH_ANNOTATION_UNKNOWN");
}

#[test]
fn requirement_evidence_wire_exactly_matches_all_185_registry_rows() {
    let registry = DeliveryRegistryV1::load(repo_root()).unwrap();
    assert!(registry.rows().iter().all(|row| row.evidence_schema == "RequirementEvidenceBindingV1"));
    let golden = RequirementEvidenceBindingV1::typed_load(include_bytes!("fixtures/f57-requirement-evidence-binding-v1-golden.json")).unwrap();
    assert_eq!(golden.payload.purpose, RequirementEvidencePurposeV1::RequirementEvidenceBinding);
    assert_eq!(golden.payload.requirement_bindings, registry.bindings_for(golden.payload.test_id));
    assert_eq!(generated_requirement_evidence_schema_bytes(), read("docs/evidence/f57-requirement-evidence-binding.v1.schema.json"));
}

#[test]
fn only_typed_pass_with_exact_test_run_and_bindings_satisfies_due() {
    for outcome in [RequirementTestOutcomeV1::Fail, RequirementTestOutcomeV1::NotCovered, RequirementTestOutcomeV1::NotDelivered] {
        assert!(!requirement_evidence_fixture(outcome).satisfies_due());
    }
    assert_code(evidence_with_empty_requirement_bindings(), "F57_REQUIREMENT_EVIDENCE_BINDING_EMPTY");
    assert_code(evidence_with_wrong_test_id(), "F57_REQUIREMENT_EVIDENCE_TEST_MISMATCH");
    assert_code(evidence_with_expiry_later_than_artifact(), "F57_REQUIREMENT_EVIDENCE_EXPIRY_EXTENSION");
}

#[test]
fn slice_probe_registry_and_wire_are_exact_and_never_satisfy_due() {
    let registry = DeliveryRegistryV1::load(repo_root()).unwrap();
    let probes = SliceProbeExecutionRegistryV1::load(repo_root()).unwrap();
    assert_eq!(probes.len(), 78);
    assert_eq!(probes.sha256(), "2e2ba21c33941c901867155458c78335793b800e069e496bd43a19a594a1995e");
    assert_eq!(probes.profile_counts(), [2, 6, 26, 5, 36, 3]);
    assert!(probes.test_ids().is_disjoint(&registry.test_ids()));
    assert_eq!(probes.rows(), registry.derive_slice_probe_execution_rows().unwrap());
    assert!(probes.rows().iter().all(|row| {
        row.test_id == test_id_for_probe(&row.parent_requirement_id, row.probe_profile)
            && row.test_id != row.parent_test_id
            && row.dispatch_binding_is_exact()
            && row.derived_contract_is_exact()
            && row.assertion_set() == [
                SliceProbeAssertionV1::ProbeCapabilityConforms,
                SliceProbeAssertionV1::ParentRequirementRemainsUnsatisfied,
            ]
            && !row.satisfies_due()
    }));
    assert_eq!(
        generated_slice_probe_schema_bytes(),
        read("docs/evidence/f57-slice-probe-evidence-binding.v1.schema.json"),
    );
    assert_code(probe_with_parent_test_id(), "F57_SLICE_PROBE_PARENT_TEST_REUSE");
    assert_code(probe_with_handler_supplied_pass(), "F57_SLICE_PROBE_ASSERTION_BYPASS");
    assert_code(probe_whose_parent_evaluator_passes(), "F57_SLICE_PROBE_PARENT_SATISFIED");
    assert_code(receipt_with_probe_in_test_results(), "F57_SLICE_PROBE_WRONG_RESULT_LANE");
}

#[test]
fn prerequisite_receipts_are_plain_typed_gate_refs_in_canonical_gate_order() {
    let receipt = gate_receipt_g4_golden();
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
    assert_code(receipt_with_signed_artifact_ref_wrapper(), "F57_GATE_PREREQUISITE_REF_TYPE_INVALID");
    assert_code(receipt_with_duplicate_or_wrong_run_gate_ref(), "F57_GATE_PREREQUISITE_SET_INVALID");
}

#[test]
fn legacy_migration_supersession_exact_joins_current_reservations() {
    let legacy = LegacyMigrationDispositionV1::load(repo_root()).unwrap();
    let reservations = MigrationReservationRegistryV2::load(repo_root()).unwrap();
    assert_eq!(legacy.rows().len(), 310);
    assert_eq!(legacy.sha256(), "06566ca354b6279391e5ec3a0152316a8eb38d1f10cb09dc23953370883c3196");
    assert_eq!(reservations.rows().len(), 47);
    assert_eq!(reservations.header(), [
        "version", "path", "gate", "owner_task", "origin", "status",
        "legacy_seed_sha256", "legacy_row_count", "mapping_closure_sha256",
    ]);
    assert_eq!(reservations.mapping_closure_sha256(), "3eb64294e9182b0e482aa16b66a08bd2b11335e114811ae9c80183532a3c27d0");
    assert_eq!(reservations.initial_exact_bytes_sha256(), "98270807d89b4f5d4ceadb2770f4148a338ab9a194294f78788a68cde4a9b742");
    assert_eq!(reservations.owner_map(), expected_47_version_owner_rows_from_master_section_4_1());
    assert!(reservations.rows().iter().all(|row| row.status == ReservationStatusV2::ReservedNotCreated));
    assert!(reservations.metadata_columns_are_constant());
    assert!(reservations.exact_joins_delivery_dag());
    assert_eq!(legacy.unique_replacement_paths().len(), 42);
    assert_eq!(reservations.net_new_f57_paths(), expected_five_net_new_paths_from_master());
    assert!(legacy.every_replacement_is_reserved());
}

#[test]
fn pre_f57_migration_partition_is_exact_and_physical_bytes_are_accounted() {
    let baseline = MigrationBaselineRegistryV1::load(repo_root()).unwrap();
    let legacy = LegacyMigrationDispositionV1::load(repo_root()).unwrap();
    let catalog = MigrationCatalogV1::load(repo_root()).unwrap();
    assert_eq!(baseline.sha256(), "52930d7ae32ee02ddda38199bcc144f5f6747fcfbe33e740741a0f21604ca8fd");
    assert_eq!(baseline.rows().len(), 78);
    assert_eq!(baseline.class_counts(), (66, 3, 7, 2));
    assert_eq!(baseline.physically_present_paths().len(), 69);
    assert!(baseline.exact_joins_catalog_partition(&catalog, &legacy));
    assert_eq!(catalog.rows().len(), 388);
    assert_eq!(catalog.executable_pre_f57_paths().len(), 69);
    assert!(baseline.immutable_files_match_source_hashes());
    match MigrationBaselineStateV1::detect(repo_root()).unwrap() {
        MigrationBaselineStateV1::PreimagePending => {
            assert!(!migration_apply_manifest_exists(repo_root()));
            assert!(baseline.rewrite_drafts_match_registered_preimages());
        }
        MigrationBaselineStateV1::PostimageApplied(manifest) => {
            assert!(manifest.verifies_against(&baseline));
            assert!(manifest.binds_all_three_preimages_and_target_contracts());
            assert!(baseline.rewrite_drafts_match_manifest_postimages(&manifest));
        }
    }
    assert!(baseline.absent_paths_remain_absent());
    assert!(legacy.all_paths_remain_absent());
}

#[test]
fn foundation_schema_is_the_unique_zero_import_root_and_nominal_owner() {
    let schema = strict_schema("docs/evidence/f57-foundation.v1.schema.json");
    assert!(schema.imports().is_empty());
    assert_eq!(schema.nominal_definitions(), [
        "UuidV1", "Sha256Digest", "SignerSpkiTokenV1", "RepositoryRelativePathV1",
        "CapabilityIdV1", "ErrorCodeV1", "RequirementIdV1", "EvidenceIdV1",
        "PlatformLaneV1", "CarrierIdV1", "ObjectiveKindV1", "ObligationIdV1",
        "FeatureOwnerIdV1", "PrincipalKindV1", "PrincipalRefV1", "GenerationNumberV1",
        "ObjectiveGenerationV1",
        "DeliveryProfileV1", "TestIdV1", "SliceProbeIdV1", "SliceProbeAssertionV1",
        "RunnerIdV1", "CandidateRunIdentityV1", "CandidateIdentityV1",
        "CandidateDataClassificationV1", "ArtifactRefV1", "TestResultRefV1",
        "SliceProbeResultRefV1", "CandidateBoundFreshPgReceiptRefPurposeV1",
        "CandidateBoundFreshPgReceiptRefV1", "ClientPlatformV1",
        "ClientStackKindV1", "ClientPackageIdV1", "ClientLifecycleFixtureRoleV1",
        "ClientLifecycleFixtureExpectedOutcomeV1",
        "SignedBusinessArtifactEnvelopeV1",
    ]);
    assert_eq!(PrincipalKindV1::wire_values(), [
        "USER", "GROUP", "TEAM", "PROJECT", "DEPARTMENT",
        "SERVICE", "AI", "PLUGIN", "CUSTOMER", "SUPPLIER",
    ]);
    assert_eq!(DeliveryProfileV1::wire_values(), [
        "G0_BOOTSTRAP", "G1_AUTHORITY_SPINE", "G2_CTC_DATA", "G3_CLIENT_SHELL",
        "G4_CTC01", "G5_INTEGRATION", "G6_RELEASE",
    ]);
    assert_eq!(foundation_owned_requirement_id_registry().len(), 185);
    assert_eq!(foundation_owned_test_id_registry().len(), 276);
    assert_eq!(foundation_owned_feature_owner_id_registry().len(), 17);
    assert_eq!(SliceProbeAssertionV1::wire_values(), [
        "PROBE_CAPABILITY_CONFORMS", "PARENT_REQUIREMENT_REMAINS_UNSATISFIED",
    ]);
    assert_eq!(CandidateDataClassificationV1::wire_values(), [
        "SYNTHETIC", "DEIDENTIFIED", "PRODUCTION_SIGNED_NO_BUSINESS_DATA",
    ]);
    assert_eq!(ClientPlatformV1::wire_values(), ["windows", "macos", "ios", "android"]);
    assert_eq!(ClientStackKindV1::wire_values(), ["tauri2", "flutter-rust"]);
    assert_eq!(ClientLifecycleFixtureRoleV1::wire_values(), [
        "UPGRADE_BASELINE", "REVOKED_PACKAGE", "DOWNGRADE_PACKAGE", "FAILED_UPDATE_PACKAGE",
    ]);
    assert_eq!(ClientLifecycleFixtureExpectedOutcomeV1::wire_values(), [
        "ACCEPTED_BASELINE", "REJECT_REVOKED_PACKAGE", "REJECT_DOWNGRADE",
        "FAILED_UPDATE_PRESERVES_TARGET",
    ]);
    assert_eq!(CandidateIdentityV1::field_names(), [
        "repository_tree_sha256", "git_commit", "cargo_lock_sha256",
        "capability_graph_sha256", "generator_version", "migration_manifest_sha256",
        "toolchain_manifest_sha256", "artifact_signer_registry_sha256",
    ]);
    assert_eq!(TestResultRefV1::field_names(), ["test_id", "artifact"]);
    assert_eq!(SliceProbeResultRefV1::field_names(), ["probe_id", "test_id", "artifact"]);
    assert_eq!(CandidateBoundFreshPgReceiptRefV1::field_names(), [
        "purpose", "artifact", "candidate_run", "profile", "through_version",
    ]);
    assert_eq!(CandidateBoundFreshPgReceiptRefPurposeV1::wire_values(), [
        "EP-F57-CANDIDATE-BOUND-FRESH-PG-RECEIPT-REF-V1",
    ]);
    assert!(all_foundation_identifier_grammars_and_closed_registries_are_exact());
    assert_code(uuid_upper_simple_braced_urn_whitespace_or_malformed_alias(), "F57_UUID_WIRE_INVALID");
    assert_eq!(foundation_wire_golden_bytes(), include_bytes!(
        "fixtures/f57-foundation-wire-v1-golden.json"
    ));
    assert!(schema.has_no_artifact_binding_signer_row_signed_payload_or_business_purpose());
    assert!(schema.only_purpose_is_plain_fresh_pg_receipt_ref_purpose());
    assert!(all_existing_f57_schemas_using_foundation_nominals_have_one_direct_exact_relative_ref());
    assert!(all_existing_signed_root_schemas_directly_import_and_compose_foundation_once());
    assert_eq!(strict_schema("docs/evidence/f57-requirement-evidence-binding.v1.schema.json").imports(), [
        "f57-foundation.v1.schema.json",
    ]);
    assert_eq!(strict_schema("docs/evidence/f57-slice-probe-evidence-binding.v1.schema.json").imports(), [
        "f57-foundation.v1.schema.json",
        "f57-requirement-evidence-binding.v1.schema.json",
    ]);
    assert_eq!(strict_schema("docs/evidence/f57-gate-receipt.v1.schema.json").imports(), [
        "f57-foundation.v1.schema.json",
    ]);
    assert_eq!(frozen_later_owner_edge_contract("f57-l2-candidate-evidence.schema.json"), [
        "f57-foundation.v1.schema.json",
        "f57-gate-receipt.v1.schema.json",
    ]);
    assert!(f57_schema_import_dag().is_acyclic_with_unique_zero_import_root(schema.id()));
    assert!(offline_closure_walk_fixture().visits_foundation_exactly_once_transitively());
    assert_code(foundation_with_reverse_or_network_ref(), "F57_SCHEMA_IMPORT_DAG_INVALID");
    assert_code(schema_using_foundation_nominal_only_transitively(), "F57_FOUNDATION_DIRECT_IMPORT_REQUIRED");
    assert_code(schema_copying_foundation_nominal(), "F57_FOUNDATION_NOMINAL_REDEFINED");
}

#[test]
fn objective_distinct_reviewer_binds_full_typed_identity_and_accepted_review() {
    let closure = distinct_reviewer_objective_closure_fixture();
    let (requested_by, reviewed_by, review_result) = closure.distinct_review().unwrap();
    assert_ne!((requested_by.kind, requested_by.id), (reviewed_by.kind, reviewed_by.id));
    assert!(review_result.typed_load_accepted_review().unwrap().exact_repeats(
        requested_by,
        reviewed_by,
        closure.objective_generation,
    ));
    assert!(review_result.proves_separately_authorized_reviewer());
    assert!(review_result.was_recorded_after_all_three_procurement_facts());
    assert_eq!(closure.exact_jcs_bytes(), include_bytes!(
        "fixtures/f57-objective-closure-distinct-reviewer-v1-golden.json"
    ));
    assert_code(closure_with_principal_hash_token(), "F57_OBJECTIVE_REVIEW_PRINCIPAL_TYPE_INVALID");
    assert_code(closure_with_kind_id_generation_or_result_drift(), "F57_OBJECTIVE_REVIEW_BINDING_MISMATCH");
}

#[test]
fn task1_carrier_parser_uses_the_sole_early_recipe_contract() {
    use ep_platform_release::carrier_contract::ReleaseCarrierRecipeIdV1;
    assert_eq!(ReleaseCarrierRecipeIdV1::wire_values(), [
        "WINDOWS_AUTHORITY_BUILD", "WINDOWS_SERVICE_INSTALL", "POSTGRES16_PITR",
        "BACKUP_RESTORE_CERTIFICATION", "P340_RELEASE72_HOUR", "POWER_SHUTDOWN",
    ]);
    assert!(f57_cli_parser_imports_recipe_type_without_local_string_table());
    assert_code(parse_carrier_recipe("windows-authority-build"), "F57_CARRIER_RECIPE_INVALID");
    assert_code(parse_known_but_not_delivered_carrier("POWER_SHUTDOWN"), "F57_CARRIER_NOT_DELIVERED");
}
```

- [ ] **Step 2: Run the narrow test and verify RED.**

Run: `cargo test -p ep-platform-release --test carrier_contract -- --nocapture`

Run: `cargo test -p ep-foundation --test f57_foundation_wire -- --nocapture && cargo test -p ep-xtask --test f57_registry --test f57_cli -- --nocapture && cargo test -p ep-platform-delivery-registry --test registry --test migration_closure -- --nocapture`

Expected: FAIL because the foundation wire/schema root, `xtask/src/f57/registry.rs`, and `DeliveryRegistryV1` do not exist.

- [ ] **Step 3: Implement strict parsing and profile derivation.**

```rust
// Defined first in crates/platform/release/src/carrier_contract.rs so the Task-1 CLI compiles.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd,
         serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ReleaseCarrierRecipeIdV1 {
    WindowsAuthorityBuild,
    WindowsServiceInstall,
    Postgres16Pitr,
    BackupRestoreCertification,
    P340Release72Hour,
    PowerShutdown,
}

// Defined once in crates/foundation/src/principal.rs.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum PrincipalKindV1 {
    User, Group, Team, Project, Department, Service, Ai, Plugin, Customer, Supplier,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct PrincipalRefV1 {
    pub kind: PrincipalKindV1,
    pub id: UuidV1,
}

// Defined once in crates/foundation/src/identifier.rs.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct UuidV1(uuid::Uuid); // private; serde accepts only lowercase 8-4-4-4-12 text

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(try_from = "String", into = "String")]
pub struct Sha256Digest([u8; 32]); // private; strict wire is 64 lowercase hexadecimal

#[derive(Clone, Debug, Eq, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(try_from = "String", into = "String")]
pub struct SignerSpkiTokenV1(String); // private; spki-sha256:<64-lowerhex>

#[derive(Clone, Debug, Eq, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(try_from = "String", into = "String")]
pub struct RepositoryRelativePathV1(String); // private; slash-normalized, no dot/parent/glob

#[derive(Clone, Debug, Eq, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(try_from = "String", into = "String")]
pub struct CapabilityIdV1(String); // private; lowercase dotted grammar, '-' after segment initial

#[derive(Clone, Debug, Eq, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(try_from = "String", into = "String")]
pub struct ErrorCodeV1(String); // private; uppercase dot-separated segments, '_' allowed, NONE reserved

#[derive(Clone, Debug, Eq, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(try_from = "String", into = "String")]
pub struct RequirementIdV1(String); // private; exact member of the 185-row delivery registry

#[derive(Clone, Debug, Eq, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(try_from = "String", into = "String")]
pub struct EvidenceIdV1(String); // private; [A-Z][A-Z0-9-]{2,95}

#[derive(Clone, Debug, Eq, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(try_from = "String", into = "String")]
pub struct PlatformLaneV1(String); // private; [a-z][a-z0-9-]{2,63}

#[derive(Clone, Debug, Eq, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(try_from = "String", into = "String")]
pub struct CarrierIdV1(String); // private; [a-z][a-z0-9-]{2,95}

#[derive(Clone, Debug, Eq, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(try_from = "String", into = "String")]
pub struct ObjectiveKindV1(String); // private; [A-Z][A-Z0-9_]{2,95}

#[derive(Clone, Debug, Eq, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(try_from = "String", into = "String")]
pub struct ObligationIdV1(String); // private; [A-Z][A-Z0-9_]{2,95}

#[derive(Clone, Debug, Eq, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(try_from = "String", into = "String")]
pub struct FeatureOwnerIdV1(String); // private; exact member of the 17-row owner registry

// Defined once in crates/foundation/src/delivery.rs.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize)]
pub struct GenerationNumberV1(u64); // private; 0 is legal only for client BOOTSTRAP

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize)]
pub struct ObjectiveGenerationV1(u64); // private; per-objective revision, 0 is always invalid

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum DeliveryProfileV1 {
    G0Bootstrap, G1AuthoritySpine, G2CtcData, G3ClientShell,
    G4Ctc01, G5Integration, G6Release,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(try_from = "String", into = "String")]
pub struct TestIdV1(String); // private; exact member of the closed 276-row executable registry

#[derive(Clone, Debug, Eq, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(try_from = "String", into = "String")]
pub struct SliceProbeIdV1(String); // private; canonical slice-probe ID

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum SliceProbeAssertionV1 {
    ProbeCapabilityConforms,
    ParentRequirementRemainsUnsatisfied,
}

// Defined once in crates/foundation/src/client.rs.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "lowercase")]
pub enum ClientPlatformV1 { Windows, Macos, Ios, Android }

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum ClientStackKindV1 { Tauri2, FlutterRust }

#[derive(Clone, Debug, Eq, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(try_from = "String", into = "String")]
pub struct ClientPackageIdV1(String); // private; 3..160 ASCII, [A-Za-z0-9][A-Za-z0-9._-]*

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ClientLifecycleFixtureRoleV1 {
    UpgradeBaseline, RevokedPackage, DowngradePackage, FailedUpdatePackage,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ClientLifecycleFixtureExpectedOutcomeV1 {
    AcceptedBaseline, RejectRevokedPackage, RejectDowngrade, FailedUpdatePreservesTarget,
}

// Defined once in crates/foundation/src/evidence.rs.
#[derive(Clone, Debug, Eq, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(try_from = "String", into = "String")]
pub struct RunnerIdV1(String); // private; [a-z][a-z0-9-]{2,95}

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct CandidateRunIdentityV1 {
    pub candidate_identity_sha256: Sha256Digest,
    pub gate_run_id: UuidV1,
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct CandidateIdentityV1 {
    pub repository_tree_sha256: Sha256Digest,
    pub git_commit: String,
    pub cargo_lock_sha256: Sha256Digest,
    pub capability_graph_sha256: Sha256Digest,
    pub generator_version: String,
    pub migration_manifest_sha256: Sha256Digest,
    pub toolchain_manifest_sha256: Sha256Digest,
    pub artifact_signer_registry_sha256: Sha256Digest,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum CandidateDataClassificationV1 {
    Synthetic, Deidentified, ProductionSignedNoBusinessData,
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct ArtifactRefV1 {
    pub uri: String,
    pub sha256: Sha256Digest,
    pub size_bytes: u64,
    pub media_type: String,
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct TestResultRefV1 {
    pub test_id: TestIdV1,
    pub artifact: ArtifactRefV1,
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct SliceProbeResultRefV1 {
    pub probe_id: SliceProbeIdV1,
    pub test_id: TestIdV1,
    pub artifact: ArtifactRefV1,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum CandidateBoundFreshPgReceiptRefPurposeV1 {
    #[serde(rename = "EP-F57-CANDIDATE-BOUND-FRESH-PG-RECEIPT-REF-V1")]
    CandidateBoundFreshPgReceiptRef,
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct CandidateBoundFreshPgReceiptRefV1 {
    pub purpose: CandidateBoundFreshPgReceiptRefPurposeV1,
    pub artifact: ArtifactRefV1,
    pub candidate_run: CandidateRunIdentityV1,
    pub profile: DeliveryProfileV1,
    pub through_version: u64,
}

// xtask imports the sole foundation definition; it never redeclares the enum.
pub use ep_foundation::DeliveryProfileV1;

pub fn capability_id_for(requirement_id: &str) -> Result<String, RegistryError> {
    let (prefix, number) = requirement_id
        .split_once('-')
        .ok_or_else(|| RegistryError::InvalidRequirement(requirement_id.to_owned()))?;
    if prefix.is_empty() || number.len() != 3 || !number.bytes().all(|b| b.is_ascii_digit()) {
        return Err(RegistryError::InvalidRequirement(requirement_id.to_owned()));
    }
    Ok(format!("f57.req.{}-{}", prefix.to_ascii_lowercase(), number))
}

pub fn base_first_due_profile(task: &str) -> Result<DeliveryProfileV1, RegistryError> {
    use DeliveryProfileV1::*;
    match task {
        "F57-01" => Ok(G0Bootstrap),
        "F57-02" | "F57-06" | "F57-08" | "F57-09" | "F57-10" | "F57-11" | "F57-12" => Ok(G1AuthoritySpine),
        "F57-16" => Ok(G3ClientShell),
        "F57-19" => Ok(G4Ctc01),
        "F57-13" | "F57-14" | "F57-15" | "F57-17" | "F57-18" | "F57-20" | "F57-21" | "F57-22" | "F57-23" => Ok(G5Integration),
        "F57-07" | "F57-24" | "F57-25" => Ok(G6Release),
        value => Err(RegistryError::UnknownActivationTask(value.to_owned())),
    }
}

pub fn resolve_delivery(
    requirement_id: &str,
    activation_task: &str,
    overrides: &DeliveryOverrideRegistryV1,
) -> Result<ResolvedDeliveryV1, RegistryError>;

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct GateReceiptRefV1 {
    pub gate: ProgramGateV1,
    pub artifact: ArtifactRefV1,
}

pub enum ObjectiveObservedStateV1 { Waiting, Closed, Reopened }
pub enum ProcurementClosureFactKindV1 {
    PurchaseInvoice,
    AccountsPayableRecognition,
    SupplierPaymentAllocation,
}
pub struct ObjectiveClosureEvidenceRefV1 {
    pub evidence_id: EvidenceIdV1,
    pub result: TestResultRefV1,
}
pub struct ProcurementClosureFactRefV1 {
    pub fact_kind: ProcurementClosureFactKindV1,
    pub owner_feature_id: FeatureOwnerIdV1,
    pub result: TestResultRefV1,
}
#[serde(tag = "review_kind", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ObjectiveClosureReviewBindingV1 {
    NotRequired,
    DistinctReviewer {
        requested_by: PrincipalRefV1,
        reviewed_by: PrincipalRefV1,
        review_result: TestResultRefV1,
    },
}
pub struct ObjectiveClosureBindingV1 {
    pub objective_kind: ObjectiveKindV1,
    pub state: ObjectiveObservedStateV1,
    pub objective_generation: ObjectiveGenerationV1,
    pub state_result: TestResultRefV1,
    pub open_obligation_ids: Vec<ObligationIdV1>,
    pub closure_evidence: Vec<ObjectiveClosureEvidenceRefV1>,
    pub procurement_facts: Vec<ProcurementClosureFactRefV1>,
    pub closure_review: ObjectiveClosureReviewBindingV1,
}
```

Create `docs/evidence/f57-foundation.v1.schema.json` first, before any other G0 schema. It is draft 2020-12, imports nothing, and has exactly 36 sole-owned definitions grouped as follows:

- primitives/identifiers: `UuidV1|Sha256Digest|SignerSpkiTokenV1|RepositoryRelativePathV1|CapabilityIdV1|ErrorCodeV1|RequirementIdV1|EvidenceIdV1|PlatformLaneV1|CarrierIdV1|ObjectiveKindV1|ObligationIdV1|FeatureOwnerIdV1`;
- principal/delivery/probe: `PrincipalKindV1|PrincipalRefV1|GenerationNumberV1|ObjectiveGenerationV1|DeliveryProfileV1|TestIdV1|SliceProbeIdV1|SliceProbeAssertionV1`;
- evidence references: `RunnerIdV1|CandidateRunIdentityV1|CandidateIdentityV1|CandidateDataClassificationV1|ArtifactRefV1|TestResultRefV1|SliceProbeResultRefV1|CandidateBoundFreshPgReceiptRefPurposeV1|CandidateBoundFreshPgReceiptRefV1`;
- cross-stage client: `ClientPlatformV1|ClientStackKindV1|ClientPackageIdV1|ClientLifecycleFixtureRoleV1|ClientLifecycleFixtureExpectedOutcomeV1`;
- reusable detached-CMS field set: `SignedBusinessArtifactEnvelopeV1 {payload,payload_sha256,signer_subject,signature_cms_b64url}`.

Rust ownership is equally singular: `identifier.rs` owns strict `UuidV1` plus the primitive/identifier group; `principal.rs` owns principal types; `delivery.rs` owns generation/profile/TestID/probe-ID/assertion; `client.rs` owns the five client nominals; `evidence.rs` owns runner/run/candidate/classification/ref types; `signature.rs` later owns signing behavior, not a second wire shape. `CandidateIdentityV1` is always the exact eight-field value, never a digest-only alias. Later crates import these definitions and never redeclare them.

The foundation schema has no artifact binding, signer-registry row, grant, policy, generation state machine, signed candidate/result payload, feature payload, business purpose, or import edge. Its plain result/reference helpers and sole Fresh-PG reference purpose authorize and sign nothing. Exact wire goldens cover all ten principal kinds; seven delivery profiles; 185 Requirement IDs; 276 TestIDs; 17 FeatureOwner IDs; both probe assertions; three data classifications; both result-ref shapes; the five-field Fresh-PG ref and exact purpose; complete client wire sets; package/path/runner and every identifier grammar/closed registry; full candidate-run and eight-field candidate identity; digest/token/unpadded-base64url and unknown-field boundaries. `UuidV1` is a private wrapper and accepts only 36-byte lowercase `8-4-4-4-12` text; uppercase, simple, braced, URN, whitespace and malformed aliases fail before a consuming field applies its nil/non-nil rule. Canonical CMS bytes are one complete DER value. Every signed-root schema directly imports foundation, composes the detached envelope exactly once while locally refining only `payload`, and closes the composed root with `unevaluatedProperties=false`. Every other F57 schema that uses a foundation nominal also carries one direct exact relative `$ref`; transitive-only use, copied primitive/client/delivery/principal/evidence/envelope definitions, absolute/network refs, a missing direct edge, or any edge out of foundation fail the schema-DAG validator. `f57-gate-receipt.v1.schema.json` uses exact same-directory refs such as `f57-foundation.v1.schema.json#/$defs/PrincipalRefV1`; every later G0 schema follows its path-correct exact relative form. The validator is rerun as schemas are added, and the final offline-schema closure must reach this unique root transitively exactly once without scanning or network resolution.

The core result/gate DAG is frozen, not inferred: `requirement-evidence-binding -> foundation`; `slice-probe-evidence-binding -> foundation + requirement-evidence-binding`, where the second edge reuses only `RequirementCandidateProvenanceV1`; `fresh-pg-evidence -> foundation`; `gate-receipt -> foundation`; and `gate-run-journal -> foundation`. The two gate schemas never import requirement-result, client, release or security-context. The later L2 owner is pre-registered with exactly `l2-candidate-evidence -> foundation + gate-receipt`, no result/client/release edge. Foundation owns generic result/probe/Fresh-PG ref shapes, while each signed result schema owns only its local payload/purpose/outcome family and directly composes the foundation envelope. A copied provenance/assertion/identifier/ref/envelope, missing direct foundation edge, extra result-to-gate/client/release edge, reverse edge or cycle fails the owner/DAG golden.

Reject BOM, CRLF, missing/extra columns, blank fields, duplicate RequirementID/TestID/EvidenceID, mismatched `T-F57-*` or `E-F57-*`, unknown task, unknown platform lane, and any row count other than 185. Parse the committed override registry with its exact four-column header and exactly 57 rows; reject unknown/duplicate RequirementID, a profile later than release, non-canonical/unsorted/duplicate JSON arrays, and any row that changes neither first due nor probe set. (F-63 ruling: the former "a probe at/after first due" rejection is removed — 11 of the frozen 78 probe pairs are deliberate two-stage proofs whose rationale says so verbatim; the rule contradicted the frozen counts `2/6/26/5/36/3` and initial hash it demands in this same sentence.) Exact-join all overrides to the 185-row source, deterministically derive the master's exact 78-row `docs/f57-slice-probe-execution.v1.tsv`, and require its initial byte hash `2e2ba21c33941c901867155458c78335793b800e069e496bd43a19a594a1995e`, profile counts `2/6/26/5/36/3`, six closed owner/target bindings and disjoint TestIDs. For every row also derive the exact `SliceProbeContractV1`: normalized contract/fixture/evidence IDs, SHA-256 of the exact rationale bytes, and assertion set `{PROBE_CAPABILITY_CONFORMS,PARENT_REQUIREMENT_REMAINS_UNSATISFIED}`. The generated graph must exact-contain that contract and one matching nonempty typed evidence requirement before the row can dispatch. The generic evaluator derives PASS only by evaluating both assertions; a handler-returned Boolean is not accepted. The generated requirement delivery view contains `slice_probe_profiles_json`; probes use only `SliceProbeEvidenceBindingV1`/`probe_results`, select narrow tests only, and never satisfy their parent due row.

`GateReceiptPayloadV1.prerequisite_receipts` is `Vec<GateReceiptRefV1>`, never `SignedArtifactRefV1` and never an untyped `ArtifactRefV1` vector. The complete earlier-gate set sorts uniquely by `(ProgramGateV1 ordinal, artifact.uri, artifact.sha256)`; each consumer resolves `artifact`, typed-loads a `GateReceiptV1`, exact-matches its payload gate and `CandidateRunIdentityV1`, verifies the registry-selected signer and terminal envelope-bound event, and rejects missing, extra, duplicate, wrong-gate, wrapper/double-signature or cross-run refs. G0 alone has an empty prerequisite vector. `GateReceiptPayloadV1.objective_closures` is also required, never omitted/defaulted: G0…G3 require exact `[]`; G4/G5 use the master-closed typed vectors delivered by their owning plans. `ObjectiveGenerationV1` is a strict-positive per-business-objective revision and is nominally distinct from deployment `GenerationNumberV1`: equal numeric values cannot cross the boundary, reopening/mutating an objective increments only its own revision, and deployment activation cannot synthesize a new objective revision. For `DISTINCT_REVIEWER`, `requested_by` and `reviewed_by` are server-derived `PrincipalRefV1` values and must differ on the complete `(kind,id)` tuple. `review_result` must typed-load the accepted review for the same `ObjectiveGenerationV1`, exact-repeat both typed principals, prove separate reviewer authorization, and be recorded only after the three canonical procurement facts. Caller hashes, truncated/pseudonymous equality tokens, result-only reviewer claims, deployment-generation substitution, or drift in either principal kind/ID, objective revision, fact prefix, or result binding fail closed. `docs/evidence/f57-gate-receipt.v1.schema.json` is the sole canonical schema owner for the receipt, plain ref and entire embedded objective-closure family, and directly imports the foundation principal/ref/envelope nominals; L2/release schemas import it by relative `$ref` and may not redefine these types. The ref wrapper and embedded closure relationships add no artifact kind or row to the 89-row signer registry.

The two release-partition fixtures are exact JCS arrays with no BOM, whitespace or trailing LF, generated only from the same resolved 185-row `DeliveryRegistryV1`. Final L2 is the canonical 149-ID `first_due_profile<=G5_INTEGRATION` vector with SHA-256 `5ec5a8663b3763bcbf85d84438d6236e9a9680fc0749ad0ca3efa04f124a5a7a`; L3 is the canonical 36-ID `first_due_profile=G6_RELEASE` vector with SHA-256 `e7a2fae4fa1a3384c47592ffd0310b3b8717cb595bc5c4397f61e0be2cbc85df`. They are mutually exclusive and their canonical union exact-equals all 185 Requirement TestIDs. The six auxiliary release-carrier TestIDs are excluded from both `test_results` vectors and are bindable by Final L2/L3 only through `carrier_refs`. These are test goldens beside the existing first-due registry golden, not a graph projection or a new projection family.

Create `docs/f57-migration-reservations.v2.tsv` byte-for-byte from master §4.1 with its exact nine-column header, 47 version/path/gate/owner/origin rows, repeated immutable metadata constants, and initial digest `98270807d89b4f5d4ceadb2770f4148a338ab9a194294f78788a68cde4a9b742`. Recompute the JCS mapping closure independently and require `3eb64294e9182b0e482aa16b66a08bd2b11335e114811ae9c80183532a3c27d0`; exact-join every owner/version to the immutable §5 DAG. The parser rejects quoting, BOM/CRLF, wrong order/count/header, a filename/version mismatch, differing repeated metadata, an unknown gate/task/origin/status, a `CREATED` row whose SQL is absent, an SQL file whose row is reserved, any non-owner transition, status rollback, mutation outside the owning task's exact row set, or a change to any non-status field. Candidate identity hashes the current exact TSV bytes, while the immutable mapping-closure digest deliberately excludes status.

Migration-baseline validation is a two-state closed machine so the permanent registry test remains valid before and after Task 6. `PREIMAGE_PENDING` is legal only while the apply manifest is absent, all three draft bytes exact-match their registered preimage hashes, all three catalog rows are `PLANNED`, and catalog cardinality is exactly `66 EXISTING + 322 PLANNED = 388`. `POSTIMAGE_APPLIED` is legal only when the deterministic apply manifest exact-binds the baseline-registry digest, all three preimage hashes, all three `target_contract_id` values and all three effective postimage hashes, the current draft bytes exact-match those postimages, all three catalog rows are `EXISTING`, and cardinality is exactly `69 EXISTING + 319 PLANNED = 388`. A manifest with any preimage still on disk, a postimage without the manifest, mixed pre/post bytes or catalog states, unknown postimage, partial three-row set, or invalid manifest is rejected. `gate g0` accepts only `POSTIMAGE_APPLIED` and signs/binds the manifest digest in its receipt.

Create `docs/f57-feature-owner-registry.v1.tsv` byte-for-byte from master §2.1 and `docs/f57-platform-mechanism-registry.v1.tsv` byte-for-byte from master §2.2. Their 17 and 35 IDs are the only legal `FeatureOwnerIdV1` and `PlatformMechanismIdV1` values; legacy schema/module aliases and crate-path inference are never accepted as owners. Create `docs/f57-task-staged-paths.v1.tsv` with exact header `task_id\tpath\tpath_kind\tcondition`. The task IDs are `G0-01..06`, `G1-01..07`, `G2-01..07`, `G3-01..02`, `G4-01..03`, `G5-01`, `G5-02A`, `G5-02B`, `G5-02C`, `G5-03..09`, and `G6-10..14`. `path_kind` is the closed set `EXACT_FILE|NEW_TREE|GENERATED_MANIFEST_SET`; `condition` is `ALWAYS|TAURI2_CERTIFIED|FLUTTER_RUST_REQUIRED`. The canonical annotations map mechanically: `Create (`NEW_TREE`)` means action `Create` plus kind `NEW_TREE` and retains the task's default condition; `Create|Modify (`TAURI2_CERTIFIED|FLUTTER_RUST_REQUIRED`)` means the named action, kind `EXACT_FILE`, and that condition; `Create|Regenerate (`GENERATED_MANIFEST_SET`)` means kind `GENERATED_MANIFEST_SET` and retains the task default. An unknown, combined, reordered, or action-incompatible annotation is rejected. Every G5-02A row has task-default `TAURI2_CERTIFIED`; every G5-02B row has task-default `FLUTTER_RUST_REQUIRED`; every other task defaults `ALWAYS`. The two selected-stack DLP rows in G5-02C and the exact selected-stack Workbench rows in G5-03 override that default with their matching branch condition, while all shared server/test/graph rows remain `ALWAYS`. Any conditional row requires the same verified signed stack-decision receipt; the obsolete token `TAURI2_REJECTED` is rejected. An existing tree may use only exact-file rows; `NEW_TREE` requires the registered root and all descendants to be absent at task-begin. `Create|Regenerate (GENERATED_MANIFEST_SET)` grants only the deterministic members listed by the staged projection manifest; during its first G0 creation, `task stage` independently regenerates the manifest from the staged graph/generator, exact-compares its self-declared 30 families and nested members, and only then trusts the set. At pre-commit that manifest is digest-verified, not signed; the clean-HEAD gate later sign-binds its digest. `Generate` is legal only for an exact member of such a family and therefore creates no independent staging authority. Only `Create|Modify|Regenerate|Expand`, manifest-owned `Generate`, and both source/destination of an explicit leaf-file `Move` become stageable rows; a directory move or directory operand is never legal. `Read|Consumes|Execute|Verify` entries never grant write or staging permission. Task 15 is evidence-only and has no staging row. Task 1 expands every comma/shorthand write entry in the five approved plans into normalized rows and freezes the resulting registry digest in the G0 receipt; missing task, unknown annotation/condition, overlapping conditional writer, unregistered leaf, absolute/parent/glob path, pre-task dirty path, or staged-set mismatch fails closed.

`f57` registers this complete closed grammar from G0:

- `task begin --task <task-id>`; `task stage --task <task-id> [--bootstrap-clean-base <full-commit>]`; `task verify-staged --task <task-id>`.
- `graph import-seeds`; `graph generate [--check]`; `fresh-pg --profile <DeliveryProfileV1> --through <baseline-or-reserved-version>`.
- `verify --level l0|l1 --changed-from <git-rev>`.
- `verify --level l2 --candidate <64-lowerhex> --candidate-manifest <path> --bundle-root <path> --run-journal <path> --out <path>`.
- `verify --level l3 --candidate <64-lowerhex> --candidate-manifest <absolute-path> --l2-evidence <absolute-path> --bundle-root <absolute-path> --run-journal <absolute-path> --out <absolute-path>`.
- `gate g0|g1|g2|g3 --bundle-root <path> --run-journal <path> --evidence-out <path>`.
- `gate g4|g5 --candidate-manifest <path> --l2-evidence <path> --bundle-root <path> --run-journal <path> --evidence-out <path>`.
- `gate g6 --candidate-manifest <absolute-path> --l2-evidence <absolute-path> --l3-evidence <absolute-path> --bundle-root <absolute-path> --run-journal <absolute-path> --evidence-out <absolute-path>`.
- `evidence verify --receipt <path> --bundle-root <path> [--expect-gate <gate>|--expect-type <type>] [--offline]`.
- `client-gate --stack tauri2 --candidate <git-rev> --storage-manifest <absolute-path> --deployment-manifest <absolute-path> --deployment-manifest-signature <absolute-path> --deployment-trust-bundle <absolute-path> --storage-trust-root <absolute-path> --storage-revocation <absolute-path> --storage-checkpoint <absolute-path> --bundle-root <path> --out <path>` for an absent architecture-attempt header; the same recovery command after that header exists forbids all seven bootstrap flags. `client-gate require --decision <kind> --receipt <path> --bundle-root <path>`.
- `client-gate validate-selected --selection-receipt <path> --candidate <git-rev> --integration --bundle-root <path> --run-journal <path> --out <path>`; for `--release` with no G6 header, replace `--integration` with `--release` and add exactly the same seven absolute storage/bootstrap flags above; after the G6 header exists every release re-entry forbids those flags.
- `client-build engineering --stack <tauri2|flutter-rust> --candidate <git-rev>`.
- `client-build --validation <path> --candidate <git-rev> --bundle-root <path> --run-journal <path> --out <path>`; evidence mode is read only from the signed validation.
- `candidate build --candidate <git-rev> [--client-artifacts <path>] --bundle-root <path> --run-journal <path> --out <path>`.
- `candidate freeze --candidate <git-rev> --client-artifacts <absolute-path> --bundle-root <absolute-path> --run-journal <absolute-path> --out <absolute-path>`.
- `carrier run --recipe <ReleaseCarrierRecipeIdV1> <--candidate <git-rev>|--candidate-manifest <absolute-path>> --bundle-root <absolute-path> --run-journal <absolute-path>`.

The mutually exclusive branches and mandatory options are part of the parser contract. The seven storage/bootstrap flags are one indivisible state-conditioned group in the exact source order shown: all seven are required exactly once before the relevant architecture/G6 header, and all seven are forbidden after it; a missing, duplicate, alias, relative path, wrong branch, partial group or eighth bootstrap option exits 2 before any write. The G0 parser shell can recognize and reject syntax while an undelivered owner returns 70; only the later G1-owned constructor turns the six trust paths plus manifest path into `ValidatedDataRootV1`. Every candidate/evidence-producing path and journal must be a descendant of its explicit bundle root; an external signed input is typed-verified and atomically materialized as `inputs/<exact-envelope-sha256>.json`. `client-gate require` likewise requires exactly one explicit `--bundle-root`; its receipt must resolve canonically below that root, and missing/duplicate root, receipt escape or extra option exits 2 before the fixed create-new repository copy. Every same-run explicit `--out`, `--candidate-manifest`, `--l2-evidence`, `--l3-evidence`, `--validation`, `--client-artifacts`, and `--evidence-out` must additionally equal the master `EvidenceEnvelopeStoreV1`/`CandidateManifestStoreV1` path derived from the explicit journal parent, immutable header profile and artifact kind; merely being in-root is insufficient. The initial stack decision is a static signed architecture input outside any candidate run and follows its separate architecture-decision attempt store below. `client-build engineering` is explicitly non-evidence: it has no `--out`, emits no signed artifact/report or consumable ref, and only returns build/test status plus ordinary ephemeral logs; candidate/evidence loaders reject its build-directory bytes. Only the validation-based `client-build` branch may emit `ClientArtifactSetV1`. `--offline` requires an absolute bundle root. `--through` accepts only the baseline endpoint `20261012113500` or an exact F57 reservation version; it never accepts a path, arbitrary number, or absent catalog row. `candidate build` always builds and hashes the current-HEAD authority artifact itself with the registered locked/offline toolchain. Without `--client-artifacts` it creates or resumes the explicit journal header's one unpredictable `gate_run_id`, builds the current-HEAD G4 Windows Workbench, and emits exactly `{windows-authority,windows-client}`. With `--client-artifacts` it accepts only a separately verified current-HEAD `ClientArtifactSetV1`, adopts that envelope's existing `gate_run_id` and journal, and emits exactly `{android-client,ios-client,macos-client,windows-authority,windows-client}`. It never accepts authority bytes or a caller-supplied run ID. `task begin` requires a dedicated clean task worktree and persists its HEAD/index/worktree/path digests under ignored `target/f57/task-state/`; `task stage` uses only the registry and snapshot; `task verify-staged` exact-checks the cached set and content. `candidate freeze` accepts no authority-artifact argv: it verifies and in-bundle-materializes the signed client artifact input, adopts the existing `gate_run_id` and journal, resolves the exact terminal `WINDOWS_AUTHORITY_BUILD` and `WINDOWS_SERVICE_INSTALL` chains from that journal, performs clean-HEAD candidate-bound Fresh-PG, and emits `ReleaseArtifactRefV1::WindowsAuthority { authority_artifact_set_ref }` pointing directly to the already signed `WindowsAuthorityArtifactSetV1` envelope; it never wraps or re-signs that authority set as `SignedArtifactRefV1`. `carrier run` accepts only compiled recipe IDs and never an arbitrary script, command, TestID, mode or extra argument. Commands whose owner task has not yet been delivered parse deterministically and return 70/`NOT_DELIVERED`; an unknown subcommand, missing/extra option—including any caller-supplied authority-artifact option—mixed branch, digest/manifest/journal/root mismatch, noncanonical store path, out-of-root path, relative final root, or DeliveryProfile/TargetGate confusion returns 2. Ownership is fixed: G0 Task 1 implements `task` and the parser shell; G0 Task 6 implements generic rehearsal Fresh-PG, internal candidate-bound mode, the durable journal, all three run output stores plus `JournalCheckpointStoreV1` and `EvidenceObjectStoreV1`, and generic carrier dispatcher; G5 Task 1 implements `client-gate` and both closed `client-build` branches; G4 Task 5 implements candidate construction; G6 Task 14 adds final `candidate freeze` and activates the six release-carrier recipes.

For `task stage`, `--bootstrap-clean-base` is mandatory only for `G0-01`, forbidden for every other task, and accepts exactly one 40-character lowercase hexadecimal commit that equals both the recorded new-worktree base and current `HEAD`. G0-01 has no legal `task begin`; every later task requires it. A symbolic ref, abbreviated SHA, dirty index at worktree creation, wrong worktree/task name, missing external base record, or any path outside the G0-01 allowlist exits 2 or fails closed before staging.

Create `verify.rs` here as the closed dispatcher shell used by the parser: it recognizes L0–L3 and G0–G6, returns 70 for every not-yet-delivered selector, and contains no passing placeholder. Task 3 extends its projection/facade drift checks and Task 6 implements L0/L1 and G0 issuance; those later tasks modify this file rather than create competing owners.

- [ ] **Step 4: Run tests and verify GREEN.**

Run: `cargo test -p ep-platform-release --test carrier_contract --all-targets --locked -- --nocapture`

Run: `cargo test -p ep-foundation --test f57_foundation_wire -- --nocapture && cargo test -p ep-xtask --test f57_registry --test f57_cli --all-targets --locked -- --nocapture && cargo test -p ep-platform-delivery-registry --test registry --test migration_closure --all-targets --locked -- --nocapture`

Expected: PASS with the exact six recipe wires compiled from `carrier_contract.rs`, uppercase/case/unknown/extra/branch parser negatives, exit `2` for invalid syntax and `70/F57_CARRIER_NOT_DELIVERED` for all six known recipes; the exact 36 foundation definitions, strict UUID aliases, complete identifier/closed registries, typed result/Fresh-PG refs, ten principal wires, seven delivery profiles, 276 TestIDs, three data classifications, complete client wire sets, one zero-import schema root/direct-import DAG, typed distinct-reviewer golden, 185 rows, and zero duplicate or unmapped values.

- [ ] **Step 5: Commit the registry boundary.**

```bash
cargo xtask f57 task stage --task G0-01 --bootstrap-clean-base <recorded-full-base-commit>
cargo xtask f57 task verify-staged --task G0-01
git commit -m "feat: freeze f57 delivery registry"
```

### Task 2: Build and import the single CapabilityGraph

**Files:**
- Create: `crates/platform/capability-graph/Cargo.toml`
- Create: `crates/platform/capability-graph/src/lib.rs`
- Create: `crates/platform/capability-graph/src/model.rs`
- Create: `crates/platform/capability-graph/src/compiler.rs`
- Create: `crates/platform/capability-graph/src/canonical.rs`
- Create: `crates/platform/capability-graph/src/semantic.rs`
- Create: `crates/platform/capability-graph/src/import.rs`
- Create: `crates/platform/capability-graph/tests/compiler.rs`
- Create: `crates/platform/capability-graph/tests/fixtures/graph-v1-golden.json`
- Create: `crates/platform/capability-graph/tests/fixtures/semantic-six-table-normalization-v1-golden.json`
- Create: `docs/schemas/f57-capability-graph.v1.schema.json`
- Create: `docs/schemas/f57-semantic-normalized-payloads.v1.schema.json`
- Create: `docs/schemas/f57-client-lifecycle-fixture-corpus.v1.schema.json`
- Create: `docs/capability-graph/f57-core.v1.json`
- Create: `testkit/fixtures/client-lifecycle/trust/android-fixture-root.der`
- Create: `testkit/fixtures/client-lifecycle/trust/ios-fixture-root.der`
- Create: `testkit/fixtures/client-lifecycle/trust/macos-fixture-root.der`
- Create: `testkit/fixtures/client-lifecycle/trust/windows-fixture-root.der`
- Create: `testkit/fixtures/client-lifecycle/android/baseline.apk`
- Create: `testkit/fixtures/client-lifecycle/android/revoked.apk`
- Create: `testkit/fixtures/client-lifecycle/android/downgrade.apk`
- Create: `testkit/fixtures/client-lifecycle/android/failed-update.apk`
- Create: `testkit/fixtures/client-lifecycle/ios/baseline.ipa`
- Create: `testkit/fixtures/client-lifecycle/ios/revoked.ipa`
- Create: `testkit/fixtures/client-lifecycle/ios/downgrade.ipa`
- Create: `testkit/fixtures/client-lifecycle/ios/failed-update.ipa`
- Create: `testkit/fixtures/client-lifecycle/macos/baseline.pkg`
- Create: `testkit/fixtures/client-lifecycle/macos/revoked.pkg`
- Create: `testkit/fixtures/client-lifecycle/macos/downgrade.pkg`
- Create: `testkit/fixtures/client-lifecycle/macos/failed-update.pkg`
- Create: `testkit/fixtures/client-lifecycle/windows/baseline.msi`
- Create: `testkit/fixtures/client-lifecycle/windows/revoked.msi`
- Create: `testkit/fixtures/client-lifecycle/windows/downgrade.msi`
- Create: `testkit/fixtures/client-lifecycle/windows/failed-update.msi`
- Create: `testkit/fixtures/client-lifecycle/fixture-corpus.v1.json`
- Modify: `Cargo.toml`
- Modify: `Cargo.lock`
- Modify: `xtask/src/f57/mod.rs`
- Create: `xtask/src/f57/generate.rs`

**Interfaces:**
- Consumes: the foundation-owned shared identifier/delivery/client nominals; the five API seeds; 185 delivery bindings; four isolated non-production native fixture build lanes whose private signing material remains outside the repository; business state-domain contract; and ADR-0025.
- Produces: the immutable reviewed 20-file lifecycle corpus (four public DER roots plus 16 native packages), strict plain `fixture-corpus.v1.json`, its sole schema, exact four-root/16-package typed CapabilityGraph vectors, the canonical `capability_package_slot_templates` vector, repository-reading `semantic_authoring_preflight(root, graph)`, the closed contract-kind validator/source-layout/codec registry and actual-six-table normalization golden, pure `compile(graph: CapabilityGraphV1, generator: GeneratorIdentityV1, delivery_registry: &DeliveryRegistryV1, mode: CompileModeV1) -> Result<CompiledCapabilityGraphV1, Vec<GraphErrorV1>>`, compiled typed copies of all three vectors, the flattened graph-owned semantic-contract vector, and canonical graph digest.

- [ ] **Step 1: Write compiler failure and determinism tests.**

```rust
#[test]
fn rejects_duplicate_owner_unknown_ref_and_cycle_together() {
    let graph = fixture_graph()
        .with_second_owner("f57.req.sal-001")
        .with_dependency("missing.capability")
        .with_cycle("a", "b");
    let errors = compile(graph, fixture_generator(), &fixture_delivery_registry(), CompileModeV1::BootstrapImport).unwrap_err();
    assert_eq!(errors.iter().map(|e| e.code.as_str()).collect::<Vec<_>>(), [
        "CAPABILITY_DUPLICATE_OWNER",
        "CAPABILITY_UNKNOWN_REFERENCE",
        "CAPABILITY_DEPENDENCY_CYCLE",
    ]);
}

#[test]
fn ordering_does_not_change_canonical_digest() {
    let a = compile(fixture_graph(), fixture_generator(), &fixture_delivery_registry(), CompileModeV1::BootstrapImport).unwrap();
    let b = compile(fixture_graph_reversed(), fixture_generator(), &fixture_delivery_registry(), CompileModeV1::BootstrapImport).unwrap();
    assert_eq!(a.graph_digest_sha256, b.graph_digest_sha256);
    assert_eq!(a.canonical_json, b.canonical_json);
}

#[test]
fn architecture_inputs_are_explicit_empty_then_exact_committed_singleton() {
    let bootstrap = compile(
        fixture_graph().with_architecture_inputs([]),
        fixture_generator(),
        &fixture_delivery_registry(),
        CompileModeV1::BootstrapImport,
    ).unwrap();
    assert!(bootstrap.architecture_inputs.is_empty());
    assert_code(
        compile(preselection_graph_with_stack_input(), fixture_generator(), &fixture_delivery_registry(), CompileModeV1::BootstrapImport),
        "CAPABILITY_ARCHITECTURE_INPUT_PRESELECTION_NONEMPTY",
    );

    let decision = committed_signed_stack_decision_fixture(ClientStackKindV1::Tauri2);
    let archive = committed_stack_decision_archive_manifest_fixture(&decision);
    let compiled = compile(
        complete_g5_graph_with_exact_architecture_input(&decision, &archive),
        fixture_generator(),
        &fixture_delivery_registry(),
        CompileModeV1::Activation { due_profile: DeliveryProfileV1::G5Integration },
    ).unwrap();
    assert_eq!(compiled.architecture_inputs, [ArchitectureInputBindingV1 {
        input_id: ArchitectureInputIdV1::ClientStackDecision,
        artifact_sha256: sha256(decision.exact_envelope_bytes()),
        archive_manifest_sha256: sha256(archive.exact_jcs_bytes()),
        media_type: "application/vnd.ep.f57-client-stack-decision-v1+json".to_owned(),
        selected_stack: ClientStackKindV1::Tauri2,
    }]);
    assert_code(g5_graph_with_decision_archive_digest_or_stack_drift(), "CAPABILITY_ARCHITECTURE_INPUT_DECISION_MISMATCH");
}

#[test]
fn g4_activation_accepts_complete_due_subgraph_and_seals_g5_nodes() {
    let compiled = compile(
        fixture_graph_with_complete_g4_and_imported_g5(),
        fixture_generator(),
        &fixture_delivery_registry(),
        CompileModeV1::Activation { due_profile: DeliveryProfileV1::G4Ctc01 },
    ).unwrap();
    assert!(compiled.active_capability_ids.contains("f57.req.clm-001"));
    assert_eq!(compiled.partition("f57.req.cli-001"), "DISABLED_NOT_CERTIFIED");
}

#[test]
fn activation_rejects_incomplete_due_binding_or_non_due_exposure() {
    assert_code(compile(g4_with_imported_due_node(), fixture_generator(), &fixture_delivery_registry(), g4_mode()), "CAPABILITY_DUE_NODE_NOT_ACTIVATION_READY");
    assert_code(compile(g4_with_g5_route_exposed(), fixture_generator(), &fixture_delivery_registry(), g4_mode()), "CAPABILITY_NOT_DUE_EXPOSED");
}

#[test]
fn graph_v1_schema_rust_and_golden_wire_are_identical() {
    let bytes = include_bytes!("fixtures/graph-v1-golden.json");
    let graph: CapabilityGraphV1 = strict_from_slice(bytes).unwrap();
    assert_eq!(graph.schema_version, 1);
    assert_eq!(graph.graph_id, "f57-core");
    assert!(graph.architecture_inputs.is_empty());
    assert!(graph.capability_package_slot_templates.is_empty());
    assert!(graph.semantic_table_anchors.is_empty());
    assert!(graph.semantic_row_schemas.is_empty());
    assert_eq!(graph.client_lifecycle_fixture_trust_roots.len(), 4);
    assert_eq!(graph.client_lifecycle_fixture_trust_roots.platforms(), [
        "android", "ios", "macos", "windows",
    ]);
    assert_eq!(graph.client_lifecycle_fixture_sources.len(), 16);
    assert!(graph.client_lifecycle_fixture_sources_are_exact_four_platform_by_four_role_matrix());
    assert!(graph.client_lifecycle_fixture_sources_have_exact_role_outcome_bijection());
    assert_eq!(canonical_json_bytes(&graph).unwrap(), bytes);
    assert_eq!(generated_json_schema_bytes(), read("docs/schemas/f57-capability-graph.v1.schema.json"));
    let schema = strict_schema("docs/schemas/f57-capability-graph.v1.schema.json");
    assert_eq!(schema.imports(), ["../evidence/f57-foundation.v1.schema.json"]);
    assert!(!schema.redefines_foundation_identifier_delivery_or_client_nominals());
    assert_code(strict_from_slice::<CapabilityGraphV1>(&with_unknown_field(bytes)), "CAPABILITY_GRAPH_UNKNOWN_FIELD");
    assert_code(strict_from_slice::<CapabilityGraphV1>(&with_raw_invalid_dependency(bytes)), "CAPABILITY_ID_INVALID");
    assert!(CapabilityIdV1::parse("f57.req.gov-001").is_ok());
    assert!(GraphSymbolV1::parse("gov-001").is_err());
    assert!(SchemaNameV1::parse("ControlHumanEffectDecisionRequestV1").is_ok());
    assert!(EvidenceIdV1::parse("E-F57-GOV-001").is_ok());
    assert!(PlatformLaneV1::parse("four-platform-client").is_ok());
    assert!(ErrorCodeV1::parse("PLATFORM.AUTHORITY.STALE_EPOCH").is_ok());
    assert!(ErrorCodeV1::parse("NONE").is_err());
    assert_code(graph_with_15_17_or_duplicate_fixture_sources(), "CAPABILITY_GRAPH_CLIENT_FIXTURE_SOURCE_SET_INVALID");
    assert_code(graph_with_fixture_role_outcome_or_order_drift(), "CAPABILITY_GRAPH_CLIENT_FIXTURE_SOURCE_BINDING_INVALID");
    assert_eq!(strict_graph_enum_vectors(), exact_graph_reachable_jcs_vectors_from_master_section_3());
    assert!(!generated_graph_schema_defines_runtime_topology_nominals());
}

#[test]
fn semantic_contracts_are_typed_self_contained_and_cycle_free() {
    let authored = semantic_authoring_preflight(
        repository_root(),
        fixture_graph_with_complete_business_semantics(),
    ).unwrap();
    assert_eq!(authored.registered_table_keys(), [
        "business_state_domain_registry_v1",
        "compensation_command_registry_v1",
        "objective_execution_registry_v1",
        "objective_trigger_closure_registry_v1",
        "termination_policy_registry_v1",
        "timeout_policy_registry_v1",
    ]);
    assert_eq!(authored.normalized_six_table_rows().len(), 89);
    assert!(authored.source_layouts_and_codecs_equal_master_six_table_registry());
    assert!(authored.normalized_rows_contain_no_opaque_policy_or_state_utf8());
    assert_eq!(
        canonical_json_bytes(authored.normalized_six_table_rows()).unwrap(),
        include_bytes!("fixtures/semantic-six-table-normalization-v1-golden.json"),
    );
    assert_eq!(
        generated_semantic_normalized_payloads_schema_bytes(),
        read("docs/schemas/f57-semantic-normalized-payloads.v1.schema.json"),
    );
    assert!(semantic_normalized_payload_schema_imports_graph_once_without_reverse_edge());
    let compiled = compile(
        authored.into_graph(),
        fixture_generator(),
        &fixture_delivery_registry(),
        CompileModeV1::Activation { due_profile: DeliveryProfileV1::G4Ctc01 },
    ).unwrap();
    assert!(compiled.semantic_table_anchors_are_sorted_unique_and_fully_referenced());
    assert!(compiled.semantic_row_schemas_are_sorted_unique_and_strict());
    assert!(compiled.semantic_contracts_are_sorted_unique_by_contract_id());
    assert!(compiled.semantic_contracts.iter().all(|contract| {
        contract.rows_are_sorted_unique()
            && contract.fields_are_sorted_unique()
            && contract.exact_row_count_matches()
            && compiled.row_schema_accepts_every_typed_value(contract)
            && compiled.provenance_resolves_once_and_matches_schema(contract)
            && contract.projection_path_matches_contract_id()
            && contract.projection_sha256 == sha256(canonical_json_bytes(
                &SemanticContractProjectionV1::from_binding(contract)
            ).unwrap())
    }));
    let expected_objective_kinds = F57_BUSINESS_OBJECTIVE_KIND_V1_EXACT
        .map(|wire| ObjectiveKindV1::parse(wire).unwrap());
    assert_eq!(compiled.objective_definition_kind_set(), expected_objective_kinds);
    for contract_kind in [
        SemanticContractKindV1::ObjectiveTriggerClosureRegistry,
        SemanticContractKindV1::ObjectiveExecutionRegistry,
        SemanticContractKindV1::CompensationCommandRegistry,
    ] {
        assert_eq!(compiled.objective_row_key_set(contract_kind), expected_objective_kinds);
    }
    let trigger_rows = compiled.single_contract_rows(
        SemanticContractKindV1::ObjectiveTriggerClosureRegistry
    ).unwrap();
    assert_eq!(
        compiled.timeout_policy_row_key_set(),
        trigger_rows.distinct_timeout_policy_id_set(),
    );
    assert_eq!(
        compiled.termination_policy_row_key_set(),
        trigger_rows.distinct_termination_policy_id_set(),
    );
    assert_eq!(
        compiled.workflow_definition_objective_kind_coverage_set(),
        expected_objective_kinds,
    );
    assert!(compiled.workflow_definitions_are_nonempty_for_every_objective_kind());
    assert!(compiled.workflow_definition_ids_are_global_unique_and_kind_correct());
    assert!(compiled.nested_policy_state_and_workflow_payloads_are_schema_bound_canonical_jcs());
    assert!(compiled.candidate_query_bindings_are_all_absent_or_all_present());
    assert!(compiled.objective_contract_ids_resolve_to_six_dedicated_kinds());
    assert!(compiled.all_semantic_references_resolve_once_in_allowed_contract_kind());
    assert!(compile_does_not_read_markdown_or_generated_semantic_contract_files());
    assert_code(
        semantic_authoring_preflight(repository_root(), graph_with_anchor_table_byte_drift()),
        "CAPABILITY_SEMANTIC_ANCHOR_SOURCE_DIGEST_MISMATCH",
    );
    assert_code(graph_with_unregistered_semantic_anchor(), "CAPABILITY_SEMANTIC_ANCHOR_UNRESOLVED");
    assert_code(graph_with_unknown_or_mismatched_row_schema(), "CAPABILITY_SEMANTIC_ROW_SCHEMA_UNRESOLVED");
    assert_code(graph_with_wrong_validator_for_contract_kind(), "CAPABILITY_SEMANTIC_VALIDATOR_KIND_MISMATCH");
    assert_code(graph_with_header_index_or_codec_drift(), "CAPABILITY_SEMANTIC_SOURCE_LAYOUT_MISMATCH");
    assert_code(graph_with_contract_table_missing_source_layout(), "CAPABILITY_SEMANTIC_SOURCE_LAYOUT_REQUIRED");
    assert_code(graph_with_graph_native_source_layout(), "CAPABILITY_SEMANTIC_SOURCE_LAYOUT_FORBIDDEN");
    assert_code(graph_with_opaque_nested_utf8_payload(), "CAPABILITY_SEMANTIC_STRUCTURED_PAYLOAD_REQUIRED");
    assert_code(graph_with_semantic_row_digest_drift(), "CAPABILITY_SEMANTIC_CONTRACT_DIGEST_MISMATCH");
    assert_code(graph_with_duplicate_contract_or_row_or_field(), "CAPABILITY_SEMANTIC_CONTRACT_DUPLICATE_ID");
    assert_code(graph_with_wrong_semantic_cell_kind(), "CAPABILITY_SEMANTIC_CONTRACT_ROW_SCHEMA_MISMATCH");
    assert_code(graph_with_wrong_workflow_provenance(), "CAPABILITY_SEMANTIC_PROVENANCE_KIND_INVALID");
    assert_code(graph_with_15_wrong_objective_kind_rows(), "CAPABILITY_OBJECTIVE_KIND_EXACT_SET_MISMATCH");
    assert_code(graph_with_timeout_or_termination_policy_key_gap(), "CAPABILITY_OBJECTIVE_POLICY_KEY_SET_MISMATCH");
    assert_code(graph_with_workflow_id_keyed_as_objective_or_wrong_kind(), "CAPABILITY_WORKFLOW_OBJECTIVE_COVERAGE_MISMATCH");
    assert_code(graph_with_partial_candidate_query_binding(), "CAPABILITY_CANDIDATE_QUERY_BINDING_INCOMPLETE");
    assert_code(graph_with_swapped_objective_contract_kinds(), "CAPABILITY_OBJECTIVE_CONTRACT_KIND_MISMATCH");
    assert_code(graph_with_unresolved_objective_or_authorization_reference(), "CAPABILITY_SEMANTIC_REFERENCE_UNRESOLVED");
}

#[test]
fn reviewed_native_fixture_corpus_is_exact_isolated_and_private_key_free() {
    let corpus = strict_fixture_corpus("testkit/fixtures/client-lifecycle/fixture-corpus.v1.json");
    assert_eq!(corpus.media_type(), "application/vnd.ep.f57-client-lifecycle-fixture-corpus-v1+json");
    assert_eq!(corpus.trust_roots.len(), 4);
    assert_eq!(corpus.packages.len(), 16);
    assert_eq!(corpus.trust_root_paths(), [
        "testkit/fixtures/client-lifecycle/trust/android-fixture-root.der",
        "testkit/fixtures/client-lifecycle/trust/ios-fixture-root.der",
        "testkit/fixtures/client-lifecycle/trust/macos-fixture-root.der",
        "testkit/fixtures/client-lifecycle/trust/windows-fixture-root.der",
    ]);
    assert_eq!(corpus.package_paths(), exact_16_native_fixture_paths());
    assert!(corpus.rows_have_nonzero_source_tree_toolchain_digests_and_closed_recipe_ids());
    assert!(corpus.package_ids_use_only_reserved_ep_f57_fixture_namespace());
    assert!(all_20_corpus_files_exact_match_manifest_digest_and_native_metadata());
    assert!(each_package_chain_terminates_only_at_its_platform_exact_der_root());
    assert!(graph_fixture_root_and_package_vectors_exact_equal_corpus());
    assert!(fixture_tree_contains_no_private_key_password_secret_or_production_root());
    let schema = strict_schema("docs/schemas/f57-client-lifecycle-fixture-corpus.v1.schema.json");
    assert_eq!(schema.imports(), ["../evidence/f57-foundation.v1.schema.json"]);
    assert!(schema.is_plain_reviewed_input_not_signed_or_offline_release_descriptor());
    assert!(!schema.redefines_foundation_nominals_or_imports_later_client_schemas());
    assert_code(corpus_with_3_or_5_roots_or_15_or_17_packages(), "F57_CLIENT_FIXTURE_CORPUS_CARDINALITY_INVALID");
    assert_code(corpus_with_alternate_ambient_or_production_root(), "F57_CLIENT_FIXTURE_TRUST_ROOT_INVALID");
    assert_code(corpus_with_private_key_or_locally_resigned_package(), "F57_CLIENT_FIXTURE_SECRET_OR_SIGNATURE_INVALID");
}
```

- [ ] **Step 2: Run the crate test and verify RED.**

Run: `cargo test -p ep-platform-capability-graph --test compiler -- --nocapture`

Expected: FAIL because package `ep-platform-capability-graph` does not exist.

- [ ] **Step 3: Implement strict graph types and compiler.**

Implement every `CapabilityGraphV1`-reachable graph type and enum byte-for-byte from master §3; this child plan may not shorten or widen that wire. It owns `CapabilityCarrierKindV1` but must not predeclare the later Task-5 topology-only `RuntimeCarrierV1|PersistenceClassV1|RuntimeParticipantV1|DatabaseConsumerV1` family. The key root/compiler types are:

```rust
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct CapabilityGraphV1 {
    pub purpose: CapabilityGraphPurposeV1,
    pub schema_version: u32,
    pub graph_id: String,
    pub graph_version: u64,
    pub architecture_inputs: Vec<ArchitectureInputBindingV1>,
    pub client_lifecycle_fixture_trust_roots: Vec<ClientLifecycleFixtureTrustRootSourceV1>,
    pub client_lifecycle_fixture_sources: Vec<ClientLifecycleFixtureSourceV1>,
    pub capability_package_slot_templates: Vec<CapabilityPackageGraphSlotV1>,
    pub semantic_table_anchors: Vec<SemanticTableAnchorV1>,
    pub semantic_row_schemas: Vec<SemanticRowSchemaV1>,
    pub capabilities: Vec<CapabilityNodeV1>,
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ArchitectureInputIdV1 { ClientStackDecision }

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct ArchitectureInputBindingV1 {
    pub input_id: ArchitectureInputIdV1,
    pub artifact_sha256: Sha256Digest,
    pub archive_manifest_sha256: Sha256Digest,
    pub media_type: String,
    pub selected_stack: ClientStackKindV1,
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct ClientLifecycleFixtureTrustRootSourceV1 {
    pub platform: ClientPlatformV1,
    pub source_path: RepositoryRelativePathV1,
    pub der_sha256: Sha256Digest,
    pub spki_token: SignerSpkiTokenV1,
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct ClientLifecycleFixtureSourceV1 {
    pub platform: ClientPlatformV1,
    pub role: ClientLifecycleFixtureRoleV1,
    pub source_path: RepositoryRelativePathV1,
    pub package_sha256: Sha256Digest,
    pub package_id: ClientPackageIdV1,
    pub package_version: String,
    pub package_signer_spki_token: SignerSpkiTokenV1,
    pub fixture_trust_root_der_sha256: Sha256Digest,
    pub expected_outcome: ClientLifecycleFixtureExpectedOutcomeV1,
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct CapabilityNodeV1 {
    pub capability_id: CapabilityIdV1,
    pub version: u64,
    pub owner: CapabilityOwnerRefV1,
    pub kind: CapabilityKindV1,
    pub dependencies: Vec<CapabilityIdV1>,
    pub semantics: CapabilitySemanticsV1,
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct GeneratorIdentityV1 {
    pub schema_version: u32,
    pub generator_version: String,
    pub generator_binary_sha256: Sha256Digest,
    pub graph_schema_sha256: Sha256Digest,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum CompileModeV1 {
    BootstrapImport,
    Activation { due_profile: DeliveryProfileV1 },
}

pub fn compile(
    graph: CapabilityGraphV1,
    generator: GeneratorIdentityV1,
    delivery_registry: &DeliveryRegistryV1,
    mode: CompileModeV1,
) -> Result<CompiledCapabilityGraphV1, Vec<GraphErrorV1>> {
    let order_errors = validate_canonical_order(&graph);
    if !order_errors.is_empty() { return Err(order_errors); }
    let errors = validate_all(&graph, delivery_registry, &mode);
    if !errors.is_empty() { return Err(errors); }
    let canonical_json = canonical_json_bytes(&graph)
        .map_err(|error| vec![GraphErrorV1::canonical(error)])?;
    let architecture_inputs = graph.architecture_inputs.clone();
    let client_lifecycle_fixture_trust_roots = graph.client_lifecycle_fixture_trust_roots.clone();
    let client_lifecycle_fixture_sources = graph.client_lifecycle_fixture_sources.clone();
    let capability_package_slot_templates = graph.capability_package_slot_templates.clone();
    let semantic_table_anchors = graph.semantic_table_anchors.clone();
    let semantic_row_schemas = graph.semantic_row_schemas.clone();
    let semantic_contracts = flatten_active_semantic_contracts(&graph, &mode)?;
    let partitions = activation_partitions(&graph, &mode)?;
    Ok(CompiledCapabilityGraphV1 {
        graph_digest_sha256: sha256(&canonical_json),
        generator_identity: generator,
        canonical_json,
        architecture_inputs,
        client_lifecycle_fixture_trust_roots,
        client_lifecycle_fixture_sources,
        capability_package_slot_templates,
        semantic_table_anchors,
        semantic_row_schemas,
        semantic_contracts,
        activation_profile: mode.activation_profile(),
        activation_eligible: mode.is_activation(),
        active_capability_ids: partitions.active_capability_ids,
        disabled_capabilities: partitions.disabled_capabilities,
    })
}
```

`compile` is a validator, never a normalizer: it rejects the first noncanonical or duplicate stable-interface vector before any semantic validation or digest calculation. The one-time importer and every later authoring tool may build into maps internally, but must call an explicit deterministic `canonicalize_for_authoring` before presenting a graph to `compile`; committed authoring bytes are then exact-compared with that canonical result. Silent sorting inside `compile`, accepting insertion/filesystem/database order, or producing a digest for noncanonical input is forbidden.

The repository-reading `semantic_authoring_preflight(root, graph)` and pure `compile` have separate, testable boundaries. Preflight fixed-loads only each registered `SemanticTableAnchorV1.document_path + table_key`, requires the unique exact BEGIN/END marker pair and LF-only table preimage defined in master §3, verifies its digest, exact-matches the complete header/index/codec vector, invokes the contract-kind's closed validator and exact-compares its canonical normalized typed rows to the graph binding. It rejects a moved/edited/ambiguous/CRLF table, a header alias/reorder, codec/validator mismatch, an opaque nested UTF-8 policy/state value and never reads a generated projection. The positive golden above must read the actual six marker pairs in `docs/superpowers/specs/2026-08-23-f57-business-execution-contract.md`, produce all `15+15+15+15+15+14=89` normalized rows and byte-match the independently reviewed normalization golden; synthetic fixtures alone cannot satisfy this test. `compile` opens no file: it proves root anchors and schemas are sorted/unique, each anchor's contract kind/schema join is valid, each active binding provenance follows the master closed rule, and every row key/field/value matches its resolvable strict schema and validator. It reconstructs only `SemanticContractProjectionV1`, exact-checks `projection_path`, `projection_sha256`, row count, containing-node owner, projection targets and all cross-contract references, then flattens the complete unique set by `contract_id`. It exact-compares Objective definitions plus trigger/closure, execution and compensation row-key sets to `F57_BUSINESS_OBJECTIVE_KIND_V1_EXACT`; timeout/termination row keys must instead equal the distinct policy IDs referenced from the trigger/closure rows, while workflow rows are keyed by `WorkflowDefinitionIdV1` and their normalized `objective_kind` coverage must equal the exact 15-kind set with at least one definition per kind. CandidateQuery's three optional fields must be all absent or all present, and the six Objective contract references must resolve to six dedicated non-interchangeable kinds. This separation closes the generated-input cycle, the unverified-Markdown-anchor gap and the former policy/workflow key-type contradiction.

`docs/schemas/f57-capability-graph.v1.schema.json` is generated from those exact Rust types, checked in, and treated as a golden artifact—not separately hand-maintained. It has exactly one direct import `../evidence/f57-foundation.v1.schema.json` for shared digest/path/capability/error/delivery/client nominals and may not copy any of them; foundation never imports graph. Root constants, identifier grammars, enum tags/wires, field sets including required `architecture_inputs`, `client_lifecycle_fixture_trust_roots`, `client_lifecycle_fixture_sources`, `capability_package_slot_templates`, `semantic_table_anchors` and `semantic_row_schemas`, the one-to-one contract-kind/validator registry, source layout/codec wires, `CanonicalJcsObjectUtf8V1`, sorting keys, and `CapabilityCarrierKindV1` are exactly master §3. CapabilityGraph owns `CapabilityPackageGraphSlotV1`, `SemanticTableAnchorV1` and `SemanticRowSchemaV1` and preserves those vectors byte-for-byte even while the initial bootstrap import starts with exact empty vectors; later activation-ready authoring may populate them only under graph-version amendment and complete reference validation. The later deployed-participant `RuntimeCarrierV1` is deliberately absent from this graph schema/source and is solely owned with the runtime-topology family in Task 5; graph parsing rejects a topology-carrier value where a capability carrier is required without defining a second enum. Both Rust deserialization and JSON Schema use `additionalProperties=false`; schema generation drift, a missing/direct-import definition, a copied topology nominal, or an importer output that fails either validator blocks G0-02.

`docs/schemas/f57-semantic-normalized-payloads.v1.schema.json` is the sole schema owner of the strict normalized `StateDomainDefinitionV1`, `TimeoutPolicyDefinitionV1` and `WorkflowDefinitionV1` object families consumed through `CANONICAL_JCS_OBJECT`. It imports the capability-graph schema exactly once for graph-owned nominals and has no reverse import. `StateDomainDefinitionV1` closes the exact fields named by business contract §14.6, with tagged `PERSISTED_LIFECYCLE|DERIVED_CLASSIFICATION`, typed transition/precedence/guard/invariant/reverse-trigger rows and no prose field. `TimeoutPolicyDefinitionV1` closes the model-tagged parameter union and exact fields in §8.2.2.1. `WorkflowDefinitionV1` closes §8.4's complete definition, step/edge/condition/compensation/upgrade/rollout types; arbitrary JSON, script/SQL/network expressions and untyped maps are unrepresentable. `semantic.rs` is the sole Rust owner of these DTOs and validators. The schema and Rust are generated/byte-golden together; a schema-name alias, opaque string, copied nominal, unknown field or graph↔payload schema cycle blocks G0-02.

G0-02 creates and reviews one immutable 20-file native lifecycle corpus before finalizing the graph: four public DER roots at the exact `testkit/fixtures/client-lifecycle/trust/{android,ios,macos,windows}-fixture-root.der` paths and the exact 16 package paths listed in Files. Four platform build lanes use dedicated non-production fixture roots and externally held ephemeral/private signing material; only public roots and final package bytes enter the repository. No private key, password, provisioning secret or production trust anchor may occur in the tree, manifest, logs or staged set. Package IDs use only reserved `ep.f57.fixture.*` test namespaces, and the public roots are trusted only in resettable conformance runners. Each lane records its nonzero source-tree/toolchain digest and closed reproducible-build recipe ID, then a clean verifier constructs a new isolated trust store from only that platform's exact DER root and native-parses all four packages. Ordinary G0/G5 runs never regenerate or resign this corpus; changing any byte/root/recipe is a deliberate graph-version amendment that replaces and reviews the complete affected closure.

`testkit/fixtures/client-lifecycle/fixture-corpus.v1.json` is strict plain JCS with exact media `application/vnd.ep.f57-client-lifecycle-fixture-corpus-v1+json`, four UTF-8 platform-sorted trust-root rows and 16 `(platform,role)`-sorted package rows. Root rows contain platform, fixed repository path, complete-DER digest and root SPKI token. Package rows contain platform/role/fixed path, byte digest, native package ID/version, leaf SPKI token, exact root-DER digest, expected outcome, source-tree/toolchain digests and reproducible recipe ID. `docs/schemas/f57-client-lifecycle-fixture-corpus.v1.schema.json` solely owns this reviewed input shape, directly imports foundation and imports no graph/later-client schema; the manifest is neither signed evidence nor an offline-release descriptor. Manifest/schema/path/media drift, copied foundation types, partial corpus, private material or production/ambient root use fails G0-02.

`client_lifecycle_fixture_trust_roots` is required at every graph stage with exactly four rows in order `android,ios,macos,windows`; each binds its fixed public DER path, exact digest and root SPKI. `client_lifecycle_fixture_sources` is likewise required with exactly 16 rows canonical-sorted by `(platform,role)`: one row for every platform × `UPGRADE_BASELINE|REVOKED_PACKAGE|DOWNGRADE_PACKAGE|FAILED_UPDATE_PACKAGE` pair. The outcome bijection is exact: `UPGRADE_BASELINE→ACCEPTED_BASELINE`, `REVOKED_PACKAGE→REJECT_REVOKED_PACKAGE`, `DOWNGRADE_PACKAGE→REJECT_DOWNGRADE`, and `FAILED_UPDATE_PACKAGE→FAILED_UPDATE_PRESERVES_TARGET`. Each fixture's `fixture_trust_root_der_sha256` equals its platform's sole root row, and its native chain terminates only at that root. `compile` rejects 3/5 roots, 15/17 packages, duplicate/missing/reordered row, another role/outcome/root mapping, invalid metadata or local nominal copy; it preserves both validated lifecycle vectors plus `capability_package_slot_templates` in `CompiledCapabilityGraphV1` and includes all three in canonical graph bytes/digest. Projection consumes only those typed vectors; it never re-parses `canonical_json` or discovers bytes/metadata/trust from filesystem, network, ambient trust, argv or environment.

The importer materializes the master §3 `ImportedContractV1` union with every original field of all five TSV families. Projection code is tested with those source paths made unavailable and must still reproduce their accepted bytes from the compiled graph alone. For explicitly registered machine tables, the owning task additionally converts the exact anchored Markdown rows into canonical `SemanticContractRowV1`/`SemanticContractFieldV1` values before authoring; the graph binding preserves provenance but the typed rows, not Markdown, are the compiled authority. Activation replaces incomplete semantics without deleting `imported_contract`; native nodes have no imported payload. The complete `GeneratorIdentityV1` survives compilation and every generated family exact-matches all three identity fields.

`CompileModeV1` is the exact set `BootstrapImport|Activation { due_profile }`. `BootstrapImport` permits only nodes carrying an exact legacy source binding and marks the compiled graph `activation_eligible=false`; it does not pretend imported API rows already contain full authorization, owner, lifecycle, objective, carrier, and evidence semantics.

`CapabilityGraphV1.architecture_inputs` is a required canonical vector, not an optional compatibility field. G0 import and every graph through pre-selection G5 construct it as exact `[]`; after the one committed stack decision and its complete historical archive exist, G5/G6 construct the exact singleton `{input_id=CLIENT_STACK_DECISION,artifact_sha256=<exact signed-decision envelope SHA-256>,archive_manifest_sha256=<exact ClientStackDecisionArchiveManifestV1 JCS SHA-256>,media_type=application/vnd.ep.f57-client-stack-decision-v1+json,selected_stack=<typed decision value>}`. G0 also constructs `capability_package_slot_templates=[]`; a later authored graph version may replace it with the canonical slot registry, never omit the field. `compile` validates canonical order and exact media, fixed-loads the committed decision plus `docs/decisions/f57-client-stack-decision-archive/archive-manifest.v1.json`, verifies the manifest's complete offline signed/TSA/revocation closure with no initial-bundle/global/network dependency, exact-matches both digests and selected stack, preserves both vectors in `CompiledCapabilityGraphV1`, and includes them in the canonical graph digest. Missing either architecture binding field, a decision-only/hash-only binding, a second input, pre-selection nonempty input, post-selection empty input, missing slot vector, incomplete archive or decision/archive/digest/stack drift fails closed.

`Activation { due_profile }` validates and activates only the profile-scoped due closure: every `RequirementBinding` whose `first_due_profile` exact-matches the supplied 185-row `DeliveryRegistryV1` and is `<=due_profile`, plus every dependency reachable from an enabled binding, must be fully activation-ready. A `SliceProbe` likewise exact-matches the registry's probe list and the deterministically derived contract/fixture/evidence/rationale/assertion closure; it must expose exactly one nonempty typed probe evidence requirement. `RequirementBinding` and `SliceProbe` nodes are bookkeeping owned exactly by `platform.capability-graph`; every semantic child carries its real registered feature/platform owner. Child due profiles are never authored: the compiler recomputes each as the minimum profile of every reachable incoming binding/probe. Any missing/extra/mismatched registry binding or imported/incomplete node in the selected closure fails. A later-profile Requirement remains in the same signed graph as `DISABLED_NOT_CERTIFIED { first_due_profile }`; it may not own an active route, menu, command, query, MCP tool, provider binding, automation trigger, or package activation. A registered `SliceProbe` child is a separate non-user-facing child capability and never activates its parent Requirement. The compiler emits stable `active_capability_ids` and `disabled_capabilities` partitions under the same graph digest. At G6 all 185 release-due rows must be complete; a deferred-boundary row is complete only when its typed seam and negative enable/claim evidence are activation-ready while the deferred implementation stays disabled.

Across the selected closure, Activation requires ID syntax, exact version, one owner, every schema/state/error/evidence/configuration/event/metric/impact/semantic reference, command/query/fact/data-object uniqueness, single fact writer, acyclic dependencies, allowed carrier, authorization completeness, deferred disablement, and 185 exact delivery references. Config IDs, ErrorCodes, EventTypes, MetricIDs, ImpactRuleIDs and SemanticContract IDs are closed stable identifiers owned by exactly one graph node; generated catalogs exact-join them and no legacy Markdown or runtime table can introduce a second value. Every lifecycle guard/invariant, authorization scope/condition/candidate query, and objective trigger/reopen/closure/responsibility/effect/timeout/termination/compensation/workflow reference resolves exactly once in an allowed contract kind; the complete business set exact-joins 15 ObjectiveKind rows. `validate_all` returns stable errors sorted by `(code, capability_id, path)`. Canonical arrays sort by the master plan's declared stable keys; state transitions sort exactly by `(from,to,action_id,guard_ids,invariant_ids)`. UI presentation order, where required, is owned by its generated UI-schema ordinal and is not inferred from graph array insertion order.

Implement `cargo xtask f57 graph import-seeds` as a one-time strict importer. Before authoring output, it fixed-loads the exact corpus manifest, 4 roots and 16 packages; verifies bytes, native metadata, leaf/root SPKI, isolated chain, root-digest join, reserved package namespace, recipe/source/toolchain bindings and absence of committed secrets; then exact-compares the resulting typed root/source vectors. It fails if `docs/capability-graph/f57-core.v1.json` already exists unless its canonical content is identical. Its constructor explicitly supplies `architecture_inputs: Vec::new()`, the exact four `client_lifecycle_fixture_trust_roots`, exact 16 `client_lifecycle_fixture_sources` and `capability_package_slot_templates: Vec::new()`; omission/default of any field is rejected by Serde/schema. The committed authoring graph contains all 185 `f57.req.*` nodes plus imported API/state/route children, records the source digest of every imported row, and remains non-activatable until each Requirement's exact first-due owner task—including G0 and G6—replaces that node and its required closure with complete activation semantics. Every imported Requirement binding and generated slice-probe bookkeeping node uses owner `platform.capability-graph`; its reachable semantic children use the exact feature/platform owner registry. The importer never derives a new owner from legacy `owner_task`. It stores no test result, package bytes or secret inside the graph.

- [ ] **Step 4: Import, compile twice, and prove byte stability.**

Run: `cargo xtask f57 graph import-seeds`

Expected: after native corpus preflight, creates `docs/capability-graph/f57-core.v1.json` with required `architecture_inputs=[]`, `capability_package_slot_templates=[]`, exact canonical four trust-root/16 package source rows, and 185 unique requirement capability references. The imported graph is reviewable but cannot be used as a runtime generation.

Run: `cargo test -p ep-platform-capability-graph --test compiler -- --nocapture`

Expected: PASS, including the 20-file/manifest isolated-trust corpus, corpus/graph direct foundation edges, exact four root and 16 package vectors preserved into the compiled graph, and G4 profile-scoped activation with later G5 nodes sealed `DISABLED_NOT_CERTIFIED`.

Run: `cargo xtask f57 graph import-seeds`

Expected: PASS with `unchanged`; no repository file changes.

- [ ] **Step 5: Commit graph authoring and compiler.**

```bash
cargo xtask f57 task stage --task G0-02
cargo xtask f57 task verify-staged --task G0-02
git commit -m "feat: establish f57 capability graph and reviewed fixture corpus"
```

### Task 3: Generate and guard every semantic projection

**Files:**
- Create: `crates/platform/capability-graph/src/projection.rs`
- Create: `crates/platform/capability-graph/tests/projections.rs`
- Create: `crates/platform/capability-graph/tests/fixtures/projection-manifest-v1-golden.json`
- Create: `crates/platform/capability-graph/tests/fixtures/client-platform-lifecycle-policy-v1-golden.json`
- Create: `crates/platform/capability-graph/tests/fixtures/semantic-contracts-manifest-v1-golden.json`
- Create: `docs/schemas/f57-projection-manifest.v1.schema.json`
- Create: `docs/schemas/f57-semantic-contracts-manifest.v1.schema.json`
- Create: `docs/generated/f57/requirement-delivery.tsv`
- Create: `docs/generated/f57/capability-index.tsv`
- Create: `docs/generated/f57/registry/config-catalog.v1.json`
- Create: `docs/generated/f57/registry/data-dictionary.v1.json`
- Create: `docs/generated/f57/registry/error-catalog.v1.json`
- Create: `docs/generated/f57/registry/event-catalog.v1.json`
- Create: `docs/generated/f57/registry/metrics-catalog.v1.json`
- Create: `docs/generated/f57/registry/impact-catalog.v1.json`
- Create: `docs/generated/f57/client-conformance-manifest.v1.json`
- Generate: `docs/generated/f57/client-platform-lifecycle-policy.v1.json`
- Create: `docs/generated/f57/semantic-contracts/manifest.v1.json`
- Generate: the exact `docs/generated/f57/semantic-contracts/<contract-id>.v1.json` member set from compiled graph bindings
- Create: `docs/generated/f57/rust/manifest.v1.json`
- Create: `docs/generated/f57/requirement-test-facades.v1.json`
- Create (`GENERATED_MANIFEST_SET`): `docs/generated/f57/projection-manifest.v1.json`
- Generate: the exact 22 `test_target_path` files and 185 `test_symbol` wrappers from `docs/f57-task-ownership.seed.tsv` (master plan §4.2)
- Create: `testkit/src/f57_cases/mod.rs`
- Create: `testkit/src/f57_cases/registry.rs`
- Generate: `testkit/src/f57_cases/generated_bindings.rs`
- Create: `xtask/src/f57/cases/mod.rs`
- Create: `xtask/src/f57/cases/g0.rs`
- Modify: `docs/capability-graph/f57-core.v1.json`
- Modify: `testkit/src/lib.rs`
- Modify: `testkit/Cargo.toml`
- Modify: `docs/openapi/README.md`
- Modify: `docs/superpowers/reviews/2026-08-23-f57-authority-supersession-register.md`
- Modify: `xtask/src/f57/generate.rs`
- Modify: `xtask/src/f57/verify.rs`
- Modify: `xtask/src/main.rs`
- Create: `xtask/src/f57/gate.rs`
- Read: `testkit/fixtures/client-lifecycle/fixture-corpus.v1.json`
- Read: `docs/schemas/f57-client-lifecycle-fixture-corpus.v1.schema.json`

**Interfaces:**
- Consumes: `CompiledCapabilityGraphV1`, including exact typed four-root/16-package lifecycle vectors, plus the fixed Task-2 corpus manifest and its exact 20 enumerated files for a read-only preflight.
- Produces: `project_all(compiled: &CompiledCapabilityGraphV1) -> Result<Vec<ProjectionArtifactV1>, ProjectionErrorV1>`, `RequirementCaseRegistryV1`, and manifest-bound generated bytes/facades.

- [ ] **Step 1: Write failing round-trip and drift tests.**

```rust
#[test]
fn imported_seeds_round_trip_before_authority_switch() {
    let projections = project_all(&compiled_real_graph()).unwrap();
    assert_eq!(projection(&projections, "api-discriminators.tsv"), read("docs/f57-api-discriminators.seed.tsv"));
    assert_eq!(projection(&projections, "api-component-shapes.tsv"), read("docs/f57-api-component-shapes.seed.tsv"));
    assert_eq!(projection(&projections, "api-component-state-domains.tsv"), read("docs/f57-api-component-state-domains.seed.tsv"));
    assert_eq!(projection(&projections, "api-state-domains.tsv"), read("docs/f57-api-state-domains.seed.tsv"));
    assert_eq!(projection(&projections, "api-direct-routes.tsv"), read("docs/f57-api-direct-routes.seed.tsv"));
}

#[test]
fn every_projection_binds_graph_and_generator_digest() {
    let projections = project_all(&compiled_real_graph()).unwrap();
    assert!(projections.iter().all(|p| p.graph_digest_sha256 == projections[0].graph_digest_sha256));
    assert!(projections.iter().all(|p| p.generator_identity == projections[0].generator_identity));
    assert_eq!(projections[0].generator_identity.generator_version, env!("CARGO_PKG_VERSION"));
}

#[test]
fn projection_manifest_wire_and_non_circular_exact_set_are_frozen() {
    let artifacts = project_all(&compiled_real_graph()).unwrap();
    let manifest = ProjectionManifestV1::from_artifacts(&artifacts).unwrap();
    assert_eq!(manifest.schema_version, 1);
    assert_eq!(manifest.families.len(), 30);
    assert_eq!(manifest.family_ids(), PROJECTION_IDS.sorted_by_wire_bytes());
    assert!(manifest.global_paths_are_unique());
    assert!(!manifest.contains_path("docs/generated/f57/projection-manifest.v1.json"));
    assert!(manifest.only_two_families_generate_outside_root());
    assert_eq!(manifest.multi_member_family_ids(), [
        "client-conformance-manifest.v1.json",
        "requirement-test-facades.v1.json",
        "rust/manifest.v1.json",
        "semantic-contracts/manifest.v1.json",
    ]);
    let lifecycle = manifest.family("client-conformance-manifest.v1.json").unwrap().members();
    assert_eq!(lifecycle.len(), 1);
    assert_eq!(lifecycle[0].output_path, "docs/generated/f57/client-platform-lifecycle-policy.v1.json");
    assert!(matches!(
        &lifecycle[0].owner,
        ProjectionOutputOwnerV1::Family(id)
            if id.as_str() == "client-conformance-manifest.v1.json"
    ));
    assert_eq!(lifecycle[0].media_type, "application/vnd.ep.f57-client-platform-lifecycle-policy-v1+json");
    assert_eq!(read(&lifecycle[0].output_path), include_bytes!(
        "fixtures/client-platform-lifecycle-policy-v1-golden.json"
    ));
    assert_eq!(canonical_json_bytes(&manifest).unwrap(), include_bytes!("fixtures/projection-manifest-v1-golden.json"));
    assert_eq!(generated_projection_manifest_schema_bytes(), read("docs/schemas/f57-projection-manifest.v1.schema.json"));
}

#[test]
fn semantic_contract_family_is_exact_projection_of_compiled_graph_rows() {
    let compiled = compiled_real_graph();
    let artifacts = project_all(&compiled).unwrap();
    let family = projection_family(&artifacts, "semantic-contracts/manifest.v1.json");
    let primary: SemanticContractsManifestV1 = strict_from_slice(family.primary().bytes()).unwrap();
    assert_eq!(primary.schema_version, 1);
    assert_eq!(primary.purpose, SemanticContractsManifestPurposeV1::SemanticContractsManifest);
    assert_eq!(primary.graph_digest_sha256, compiled.graph_digest_sha256);
    assert_eq!(primary.generator_identity, compiled.generator_identity);
    assert_eq!(
        primary.bindings,
        compiled.semantic_contracts.iter()
            .map(SemanticContractsManifestBindingV1::from)
            .collect::<Vec<_>>()
    );
    assert_eq!(family.members().len(), compiled.semantic_contracts.len());
    for contract in &compiled.semantic_contracts {
        let member = family.member_by_output_path(&contract.projection_path).unwrap();
        let expected_bytes = canonical_json_bytes(
            &SemanticContractProjectionV1::from_binding(contract)
        ).unwrap();
        assert_eq!(member.bytes(), expected_bytes);
        assert_eq!(member.descriptor.sha256, contract.projection_sha256);
        assert!(matches!(
            &member.descriptor.owner,
            ProjectionOutputOwnerV1::Family(id)
                if id.as_str() == "semantic-contracts/manifest.v1.json"
        ));
    }
    assert_eq!(family.primary().bytes(), include_bytes!(
        "fixtures/semantic-contracts-manifest-v1-golden.json"
    ));
    assert_eq!(
        generated_semantic_contracts_manifest_schema_bytes(),
        read("docs/schemas/f57-semantic-contracts-manifest.v1.schema.json")
    );
    assert!(project_all_does_not_read_business_markdown_or_generated_contract_files());
    assert_code(project_with_missing_extra_or_reordered_semantic_member(), "PROJECTION_SEMANTIC_MEMBER_SET_MISMATCH");
    assert_code(project_with_semantic_manifest_binding_drift(), "PROJECTION_SEMANTIC_MANIFEST_BINDING_MISMATCH");
}

#[test]
fn imported_api_projection_never_rereads_historical_seed_snapshots() {
    let compiled = compiled_import_graph_with_source_paths_unavailable();
    let projections = project_all(&compiled).unwrap();
    assert_eq!(all_five_import_projection_bytes(&projections), accepted_seed_preimages());
}

#[test]
fn requirement_facade_exact_set_is_frozen() {
    let manifest = generated_requirement_facades();
    assert_eq!(manifest.target_paths().len(), 22);
    assert_eq!(manifest.symbols().len(), 185);
    assert_eq!(manifest.requirement_ids().len(), 185);
    assert!(manifest.is_exact_projection_of("docs/f57-task-ownership.seed.tsv"));
}

#[test]
fn g0_due_handlers_are_real_and_future_handlers_fail_closed() {
    let registry = RequirementCaseRegistryV1::load_real().unwrap();
    assert_eq!(registry.delivered_for(DeliveryProfileV1::G0Bootstrap), ["GOV-010", "NFR-010"]);
    assert_eq!(registry.outcome("GOV-010"), TestOutcomeV1::Pass);
    assert_eq!(registry.outcome("NFR-010"), TestOutcomeV1::Pass);
    assert_eq!(registry.outcome("CRM-001"), TestOutcomeV1::NotDelivered);
}

#[test]
fn generated_handler_bindings_compile_and_exact_match_delivered_registry() {
    let registry = RequirementCaseRegistryV1::load_real().unwrap();
    assert_eq!(compiled_testkit_binding_ids(), registry.delivered_ids(TargetKindV1::RustTestkit));
    assert!(compiled_testkit_binding_ids().is_empty());
    assert_eq!(xtask_local_dispatch("GOV-010").unwrap(), TestOutcomeV1::Pass);
    assert_eq!(xtask_local_dispatch("NFR-010").unwrap(), TestOutcomeV1::Pass);
    assert_code(dispatch_generated("CRM-001"), "F57_REQUIREMENT_NOT_DELIVERED");
    assert_eq!(discovered_language_local_union(), registry.delivered_requirement_ids());
    assert!(language_local_discovered_sets_are_pairwise_disjoint());
}

#[test]
fn real_graph_activates_exactly_the_g0_due_closure() {
    let compiled = compile(
        real_graph(),
        real_generator(),
        &real_delivery_registry(),
        CompileModeV1::Activation { due_profile: DeliveryProfileV1::G0Bootstrap },
    ).unwrap();
    assert!(compiled.activation_eligible);
    assert!(compiled.has_active_requirement("GOV-010"));
    assert!(compiled.has_active_requirement("NFR-010"));
    assert_eq!(compiled.disabled_requirement_count(), 183);
    assert!(compiled.disabled_requirements_are_later_profile_not_certified());
}

#[test]
fn generated_openapi_and_machine_registry_authorities_are_exact() {
    assert_eq!(generated_openapi_paths(), [
        "docs/generated/f57/openapi/control-center.v1.yaml",
        "docs/generated/f57/openapi/employee-api.v1.yaml",
        "docs/generated/f57/openapi/portal.v1.yaml",
    ]);
    assert!(!path_exists("docs/openapi/control-center.v1.yaml"));
    assert!(!path_exists("docs/openapi/employee-api.v1.yaml"));
    assert_eq!(generated_machine_registry_paths().len(), 6);
    assert!(generated_machine_registries_exact_join_graph());
}

#[test]
fn client_conformance_dispatch_is_closed_and_stack_complete() {
    let manifest = generated_client_conformance_manifest();
    let policy = generated_client_platform_lifecycle_policy();
    assert_eq!(manifest.ids(), ["G3_SHELL", "G4_CTC_UI_API", "G5_FOUR_PLATFORM"]);
    assert_eq!(manifest.carriers().len(), 6);
    assert!(manifest.has_exactly_one_carrier_per_stack_and_id());
    assert!(manifest.recipe_ids_are_closed_enums_not_commands());
    assert_eq!(manifest.carrier_vectors(), [
        ("G3_SHELL", "flutter-rust", "FlutterRustG3Shell", ["clients/workbench/test/g3_shell_conformance_test.dart"]),
        ("G3_SHELL", "tauri2", "Tauri2G3Shell", ["clients/workbench/tests/g3-shell.conformance.test.ts"]),
        ("G4_CTC_UI_API", "flutter-rust", "FlutterRustG4CtcUiApi", ["clients/workbench/integration_test/ctc01_ui_api_test.dart"]),
        ("G4_CTC_UI_API", "tauri2", "Tauri2G4CtcUiApi", ["clients/workbench/e2e/ctc01.spec.ts"]),
        ("G5_FOUR_PLATFORM", "flutter-rust", "FlutterRustG5FourPlatform", ["clients/workbench/test/four_platform_contract_test.dart"]),
        ("G5_FOUR_PLATFORM", "tauri2", "Tauri2G5FourPlatform", ["clients/workbench/tests/four-platform-contract.test.ts"]),
    ]);
    assert!(manifest.carriers().iter().all(|row| row.delivery_state == ClientConformanceDeliveryStateV1::NotDelivered));
    assert_eq!(policy.platform_rows(), ["android", "ios", "macos", "windows"]);
    assert_eq!(policy.fixture_trust_root_descriptors().len(), 4);
    assert_eq!(policy.fixture_descriptors().len(), 16);
    assert!(policy.is_total_one_to_one_projection_of(
        compiled_real_graph().client_lifecycle_fixture_trust_roots,
        compiled_real_graph().client_lifecycle_fixture_sources,
    ));
    assert!(lifecycle_policy_generation_uses_no_filesystem_network_trust_store_argv_or_environment());
    assert!(projection_preflight_fixed_loads_manifest_and_exact_matches_all_20_reviewed_files());
    assert!(projection_preflight_uses_one_fresh_isolated_trust_store_per_platform_root());
}

#[test]
fn rust_owner_dto_family_owns_every_generated_rust_member() {
    let manifest = generated_rust_owner_manifest();
    assert!(manifest.members_are_sorted_unique_and_graph_bound());
    assert!(manifest.members().iter().all(|m| m.path == m.owner.generated_public_path()));
    assert!(manifest.members().iter().all(|m| m.path.ends_with("/src/public/generated.rs")));
}
```

- [ ] **Step 2: Run projection tests and verify RED.**

Run: `cargo test -p ep-platform-capability-graph --test projections -- --nocapture`

Expected: FAIL because `projection.rs` and generated manifests do not exist.

- [ ] **Step 3: Implement closed projection output.**

```rust
pub const PROJECTION_IDS: [&str; 30] = [
    "requirement-delivery.tsv",
    "capability-index.tsv",
    "api-discriminators.tsv",
    "api-component-shapes.tsv",
    "api-component-state-domains.tsv",
    "api-state-domains.tsv",
    "api-direct-routes.tsv",
    "registry/config-catalog.v1.json",
    "registry/data-dictionary.v1.json",
    "registry/error-catalog.v1.json",
    "registry/event-catalog.v1.json",
    "registry/metrics-catalog.v1.json",
    "registry/impact-catalog.v1.json",
    "openapi/control-center.v1.yaml",
    "openapi/employee-api.v1.yaml",
    "openapi/portal.v1.yaml",
    "policy/p340-certification-policy.v1.json",
    "typescript/index.ts",
    "typescript/types.ts",
    "typescript/client.ts",
    "typescript/manifest.v1.json",
    "ui/control-center.ui-schema.v1.json",
    "ui/employee-workbench.ui-schema.v1.json",
    "ui/portal.ui-schema.v1.json",
    "client-conformance-manifest.v1.json",
    "rust/manifest.v1.json",
    "semantic-contracts/manifest.v1.json",
    "authorization-catalog.json",
    "test-manifest.json",
    "requirement-test-facades.v1.json",
];

pub fn project_all(compiled: &CompiledCapabilityGraphV1) -> Result<Vec<ProjectionArtifactV1>, ProjectionErrorV1> {
    let mut artifacts = Vec::new();
    for family_id in PROJECTION_IDS {
        artifacts.extend(project_family(family_id, compiled)?);
    }
    artifacts.sort_by(|a, b| a.descriptor.output_path.cmp(&b.descriptor.output_path));
    reject_duplicate_output_paths(&artifacts)?;
    Ok(artifacts)
}
```

Implement the exact `ProjectionArtifactV1`, descriptor, owner, role, media, family and `ProjectionManifestV1` types from master §3. `ProjectionManifestV1::from_artifacts` exact-checks one shared graph digest and complete generator identity, then groups the sorted primary/member descriptors into precisely 30 closed families. It never includes output bytes or its own path/digest. Both the strict Rust wire and `docs/schemas/f57-projection-manifest.v1.schema.json` must accept the checked-in golden vector byte-for-byte and reject unknown fields, a 31st family, self-reference, duplicate path, wrong media/owner, upperhex digest, member-order drift or generator mismatch.

All repository semantic projections are rooted at `docs/generated/f57/`. The five import seed files remain byte-identical to their accepted G0 preimage and become immutable historical import snapshots during the authority-switch commit; they are not projection destinations and no later graph generation compares against or rewrites them. The three OpenAPI authorities are exactly `docs/generated/f57/openapi/control-center.v1.yaml`, `employee-api.v1.yaml`, and `portal.v1.yaml`. The old absent paths `docs/openapi/control-center.v1.yaml` and `docs/openapi/employee-api.v1.yaml` become permanent `SUPERSEDED_PLANNED_PATH` entries and must never be created; the existing `docs/openapi/*.yaml` files remain historical/current-subject inputs only. Task 3 atomically updates `docs/openapi/README.md` and the authority register to that exact state. The six generated registry catalogs are the only machine truth for config, data objects, error codes, events, metrics, and impact rules; the similarly named legacy markdown/data-dictionary files remain non-blocking inputs and cannot gate or extend implementation.

`requirement-test-facades.v1.json` is a projection family manifest whose members are the exact 22 repository paths in master §4.2 plus the compiled-discovery member `testkit/src/f57_cases/generated_bindings.rs`; these 23 members are generated outside `docs/generated/f57/`. The 22 canonical files contain wrappers only. `generated_bindings.rs` contains generated `#[path]` declarations and exact dispatch only, while the stable root `testkit/src/f57_cases/mod.rs` contains `mod generated_bindings;`; it never contains product assertions.

`rust/manifest.v1.json` is the second multi-member projection family. It contains the canonical-sorted exact members `{owner_id,path,sha256}` and binds each member to the graph and generator digests. Feature-owned Rust DTO output is written only to `crates/features/<owning-feature>/src/public/generated.rs` once that feature exists; platform-owned Rust DTO output is written only to the owning platform crate's `src/public/generated.rs`. `project_family("rust/manifest.v1.json", ...)` returns both that family manifest and the exact member bytes; no child task may hand-create a DTO, add an unmanifested generated member, or write a global DTO crate. Requirement facades and Rust remain the only two families permitted to generate outside `docs/generated/f57/`.

`client-conformance-manifest.v1.json` is the third multi-member projection family. Its primary projects exactly the six typed carriers from master §3 and stores closed recipe IDs, delivery state and exact source paths, never executable command text. Its exact sole member is `docs/generated/f57/client-platform-lifecycle-policy.v1.json`, owned by `FAMILY(client-conformance-manifest.v1.json)` with media `application/vnd.ep.f57-client-platform-lifecycle-policy-v1+json`; no other member or owner kind is legal. That member is plain JCS with `schema_version=1`, purpose `EP-F57-CLIENT-PLATFORM-LIFECYCLE-POLICY-V1`, `policy_id=F57_CLIENT_PLATFORM_LIFECYCLE_BASELINE_V1`, `policy_revision=1`, and the exact four UTF-8 platform-sorted lifecycle rows from master §3. Its four trust-root descriptors and 16 fixture descriptors are a total one-to-one transformation of the compiled graph's typed vectors, including exact root-DER digest/SPKI joins; policy generation may not read package bytes, filesystem, network, trust store, argv or environment. Before any policy byte is emitted, G0-03 preflight fixed-loads the one corpus manifest and only its exact 20 enumerated paths, exact-compares every graph row/digest/metadata/recipe binding, and native-verifies each leaf chain in a new isolated trust store containing only that platform's exact DER root. Missing/extra/alternate/ambient root, manifest/schema/media/path drift, secret/private material, partial corpus or local regenerated substitute fails before projection. The family primary, member descriptor/digest, graph digest and generator identity bind the policy without a new signer row. G0 writes all six carriers as `NOT_DELIVERED`; G3/G4 and exactly one G5 client branch update graph delivery state and regenerate the family. Before a stack decision, only the initial Tauri G3/G4 carrier may be invoked by its owning gate. After the signed decision, G5/G6 require the three selected-stack rows to be `DELIVERED`; a rejected tree is permanently `REJECTED_FIXTURE`.

`semantic-contracts/manifest.v1.json` is the fourth and only remaining multi-member family; the total is exactly 30, never 29 or 31. Its primary at `docs/generated/f57/semantic-contracts/manifest.v1.json` is strict `SemanticContractsManifestV1` JCS with `schema_version=1`, purpose `EP-F57-SEMANTIC-CONTRACTS-MANIFEST-V1`, the exact compiled graph/generator identities and canonical bindings sorted by `contract_id`. `docs/schemas/f57-semantic-contracts-manifest.v1.schema.json` owns only that primary envelope and references graph-owned semantic nominals; it may not copy or weaken them. Each member is the one-to-one `SemanticContractProjectionV1 {schema_version=1,contract_id,contract_kind,row_schema_id,rows}` at its binding's `projection_path`, with media `application/json`, digest equal to `projection_sha256`, and descriptor owner `FAMILY(semantic-contracts/manifest.v1.json)`. The primary binding exact-repeats each compiled contract's ID, kind, path, digest, schema, row count, business owner and projection-target vector. Member lookup is by exact path/ID, never positional zip. The projector consumes only the compiled typed vector; it never reads a Markdown source, existing generated member, database row or handwritten registry. Missing/extra/reordered members, path collision, primary/member binding drift, member/graph digest mismatch, schema/count mismatch or orphan semantic reference fails before any output is written.

Before generating, Task 3 replaces exactly the `GOV-010` and `NFR-010` requirement-binding nodes plus only their reachable G0 mechanism dependencies with complete `ActivationReady` semantics: registry integrity, projection drift, staging safety, signed-receipt schema, authorization, lifecycle, owner, carrier, and concrete evidence bindings. No later-profile requirement node is promoted. `compile(real_graph, real_generator, &real_delivery_registry, Activation { G0_BOOTSTRAP })` must pass with those two active and the other 183 sealed `DISABLED_NOT_CERTIFIED`; `BootstrapImport` remains available only for the historical byte-round-trip test and can never issue the G0 receipt.

Use LF, UTF-8 without BOM, stable field order, no locale sorting, and atomic temporary-file rename. `--check` writes nothing and prints every diff path. After the one-time round-trip passes, keep the five seed filenames and exact accepted digests solely for import audit, atomically change their authority-register status to `HISTORICAL_IMPORT_SNAPSHOT`, forbid any later byte change through `xtask f57 verify`, and never require a later live projection to equal those frozen snapshots.

Every generated Rust/TypeScript facade copies the seed symbol exactly and delegates only to a language-local registry adapter. The gate invokes due symbols by exact name; it never runs a whole mixed-profile target and interprets a future result as success. G0 implements concrete `GOV-010` and `NFR-010` handlers in `xtask/src/f57/cases/g0.rs`. `xtask/src/main.rs` registers the generated `f57check` module so those two exact functions compile and can be selected by the Rust-owned runner; generation fails if module registration or symbol discovery is absent. `testkit/src/f57_cases/generated_bindings.rs` covers only Rust testkit targets, never the xtask or TypeScript handler. The gate exact-joins the pairwise-disjoint xtask, testkit, and TypeScript discovered sets against the globally due IDs. All other descriptors are explicitly `NOT_DELIVERED` until their owning child task adds a concrete handler and updates the CapabilityGraph handler binding. Direct invocation of a non-due wrapper fails with `F57_REQUIREMENT_NOT_DELIVERED`; duplicate/missing/unknown/cross-language handler, `#[ignore]`, empty body, unconditional success, panic-placeholder, or catch-all registration fails generation. Child tasks modify handler modules only; they never edit a generated facade.

- [ ] **Step 4: Generate and verify no drift.**

Run: `cargo xtask f57 graph generate`

Expected: after exact 20-file corpus/isolated-trust preflight, all 30 projection families, including deterministic `policy/p340-certification-policy.v1.json`, the third multi-member client-conformance family with its sole root-bound `client-platform-lifecycle-policy.v1.json` member, the fourth semantic-contract family with the exact compiled-contract member set, the exact 22 facade members/185 symbols, the compiled-discovery binding member, the exact graph-owned Rust DTO members, and `docs/generated/f57/projection-manifest.v1.json` are written with one graph digest; the two superseded `docs/openapi` planned paths remain absent.

Run: `cargo xtask f57 graph generate --check`

Expected: PASS and `0 projection drift`.

Run: `cargo test -p ep-platform-capability-graph --test projections -- --nocapture`

Expected: PASS.

- [ ] **Step 5: Commit generated authority switch.**

```bash
cargo xtask f57 task stage --task G0-03
cargo xtask f57 task verify-staged --task G0-03
git commit -m "feat: generate capability graph projections"
```

### Task 4: Enforce feature-first boundaries and retire fixed process truth

**Files:**
- Create: `xtask/src/archcheck/features.rs`
- Create: `xtask/tests/f57_architecture.rs`
- Modify: `xtask/src/archcheck/mod.rs`
- Modify: `xtask/src/archcheck/deps.rs`
- Modify: `xtask/src/archcheck/frozen.rs`
- Modify: `crates/platform/runtime/src/process.rs`
- Modify: `apps/integration-gateway/src/config.rs`
- Modify: `apps/integration-gateway/src/main.rs`
- Modify: `Cargo.toml`

**Interfaces:**
- Consumes: Cargo metadata and the compiled topology/capability registries.
- Produces: architecture rules `feature-public-only`, `feature-one-owner`, `no-new-layer-triplet`, `non-authority-zero-database`, and `runtime-no-fixed-cardinality`.

- [ ] **Step 1: Write failing real-workspace architecture tests.**

```rust
#[test]
fn integration_gateway_has_no_database_capability() {
    let report = run_real_archcheck();
    assert_no_violation(&report, "non-authority-zero-database");
    assert!(!ProcessKind::IntegrationGateway.holds_sql_session());
    assert!(!ProcessKind::PortalGateway.holds_sql_session());
    assert!(!ProcessKind::ExtensionHost.holds_sql_session());
}

#[test]
fn new_business_crates_are_feature_first() {
    let fixture = fixture_workspace_with("crates/contract/new-domain/Cargo.toml");
    assert_violation(run_archcheck(&fixture), "no-new-layer-triplet");
}
```

- [ ] **Step 2: Run tests and verify RED on the historical gateway rule.**

Run: `cargo test -p ep-xtask --test f57_architecture -- --nocapture`

Expected: FAIL because `ProcessKind::IntegrationGateway.holds_sql_session()` is currently true and the feature rules do not exist.

- [ ] **Step 3: Implement graph-aware architecture rules.**

```rust
pub const F57_FEATURE_RULES: [&str; 5] = [
    "feature-public-only",
    "feature-one-owner",
    "no-new-layer-triplet",
    "non-authority-zero-database",
    "runtime-no-fixed-cardinality",
];
```

Set Integration Gateway, Portal Gateway, and Extension Host ordinary SQL capability to false; remove database URL/pool/KMS fields from their configs and require typed upstream contracts only. Permit the declared backup/archive writers only their separately measured replication/streaming identities. Permit existing layer-first crates as compatibility facades, but fail if a touched facade gains domain policy, repository SQL, or a new internal dependency. A feature may depend only on foundation, platform public APIs, adapters injected at composition, and another feature's `public` module.

- [ ] **Step 4: Run the architecture gate.**

Run: `cargo test -p ep-xtask --test f57_architecture -- --nocapture`

Expected: PASS.

Run: `cargo xtask archcheck`

Expected: PASS with the five F57 feature rules listed and 0 violations.

- [ ] **Step 5: Commit the physical-boundary gate.**

```bash
cargo xtask f57 task stage --task G0-04
cargo xtask f57 task verify-staged --task G0-04
git commit -m "refactor: enforce feature first boundaries"
```

### Task 5: Add signed-artifact vocabulary, generation, and Windows topology contracts

**Files:**
- Create: `crates/foundation/src/canonical_json.rs`
- Create: `crates/foundation/src/signature.rs`
- Create: `crates/foundation/src/validated_path.rs`
- Create: `crates/foundation/src/artifact_signer_registry.rs`
- Create: `crates/foundation/tests/signature.rs`
- Create: `crates/foundation/tests/validated_path.rs`
- Create: `docs/f57-artifact-signer-registry.v1.json`
- Create: `docs/schemas/f57-artifact-signer-registry.v1.schema.json`
- Create: `crates/platform/evidence-trust/Cargo.toml`
- Create: `crates/platform/evidence-trust/src/lib.rs`
- Create: `crates/platform/evidence-trust/src/model.rs`
- Create: `crates/platform/evidence-trust/src/provider.rs`
- Create: `crates/platform/evidence-trust/src/coordinator.rs`
- Create: `crates/platform/evidence-trust/src/broker_protocol.rs`
- Create: `crates/platform/evidence-trust/src/windows_broker_install.rs`
- Create: `crates/platform/evidence-trust/tests/provisioning.rs`
- Create: `crates/platform/evidence-trust/tests/broker_protocol.rs`
- Create: `crates/platform/evidence-trust/tests/windows_broker_install.rs`
- Create: `crates/platform/evidence-trust/tests/fixtures/f57-evidence-credential-requirements-v1-golden.json`
- Create: `crates/platform/evidence-trust/tests/fixtures/f57-evidence-signer-broker-windows-install-row-v1-golden.json`
- Create: `crates/adapter/kms/src/f57_evidence_trust.rs`
- Create: `crates/adapter/kms/tests/f57_evidence_trust.rs`
- Modify: `crates/adapter/kms/src/lib.rs`
- Modify: `crates/adapter/kms/Cargo.toml`
- Create: `apps/evidence-trust-tool/Cargo.toml`
- Create: `apps/evidence-trust-tool/src/main.rs`
- Create: `apps/evidence-trust-tool/src/ceremony.rs`
- Create: `apps/evidence-signing-broker/Cargo.toml`
- Create: `apps/evidence-signing-broker/src/main.rs`
- Create: `apps/evidence-signing-broker/src/service.rs`
- Create: `apps/evidence-signing-broker/src/windows_service.rs`
- Create: `apps/evidence-signing-broker/tests/composition.rs`
- Create: `apps/evidence-signing-broker/tests/windows_service.rs`
- Create: `docs/evidence/f57-evidence-signer-broker-windows-install-readback.v1.schema.json`
- Create: `docs/generated/f57/windows/evidence-signer-broker-install-row.v1.json`
- Create: `installer/windows/generated/EvidenceSignerBroker.wxi`
- Create: `crates/platform/powershell-trust/Cargo.toml`
- Create: `crates/platform/powershell-trust/src/lib.rs`
- Create: `crates/platform/powershell-trust/src/policy.rs`
- Create: `crates/platform/powershell-trust/src/descriptor.rs`
- Create: `crates/platform/powershell-trust/src/attempt.rs`
- Create: `crates/platform/powershell-trust/src/generated_registry.rs` (initial empty cumulative registry; generated only)
- Create: `crates/platform/powershell-trust/src/windows_executor.rs`
- Create: `crates/platform/powershell-trust/tests/policy.rs`
- Create: `crates/platform/powershell-trust/tests/attempt.rs`
- Create: `crates/platform/powershell-trust/tests/windows_executor.rs`
- Create: `apps/powershell-trust-tool/Cargo.toml`
- Create: `apps/powershell-trust-tool/src/main.rs`
- Create: `apps/powershell-trust-tool/src/sign.rs`
- Create: `apps/powershell-trust-tool/src/execute.rs`
- Create: `apps/powershell-trust-tool/src/attempt_store.rs`
- Create: `apps/powershell-trust-tool/tests/composition.rs`
- Create: `docs/security/f57-powershell-script-trust-policy.v1.json`
- Create: `docs/schemas/f57-powershell-script-trust-policy.v1.schema.json`
- Create: `docs/schemas/f57-powershell-script-descriptor.v1.schema.json`
- Create: `docs/generated/f57/powershell-script-registry.v1.json` (initial exact empty cumulative registry; generated only)
- Create: `crates/platform/powershell-trust/tests/fixtures/f57-powershell-script-trust-policy-v1-golden.json`
- Create: `crates/platform/powershell-trust/tests/fixtures/f57-powershell-script-registry-v1-golden.json`
- Create: `crates/platform/release/src/generation.rs`
- Create: `crates/platform/release/src/generation_approval.rs`
- Create: `crates/platform/release/src/participant.rs`
- Create: `crates/platform/release/src/pin.rs`
- Create: `crates/platform/release/tests/generation.rs`
- Create: `crates/platform/release/tests/generation_approval.rs`
- Create: `crates/platform/release/tests/fixtures/generation-manifest-v1.jcs.json`
- Create: `crates/platform/release/tests/fixtures/generation-reverse-plan-v1.jcs.json`
- Create: `crates/platform/release/tests/fixtures/generation-participant-ack-v1.jcs.json`
- Create: `crates/platform/release/tests/fixtures/generation-approval-registry-v1.jcs.json`
- Create: `crates/platform/release/tests/pin_wire.rs`
- Create: `crates/platform/release/tests/fixtures/artifact-pin-active-v1.jcs.json`
- Create: `crates/platform/runtime/src/topology.rs`
- Create: `crates/platform/runtime/src/windows/mod.rs`
- Create: `crates/platform/runtime/tests/topology.rs`
- Create: `crates/platform/runtime/tests/fixtures/runtime-topology-declaration-v1-golden.json`
- Create: `crates/platform/runtime/tests/fixtures/runtime-topology-certification-v1-golden.json`
- Create: `docs/evidence/f57-runtime-topology.v1.schema.json`
- Create: `docs/evidence/f57-generation.v1.schema.json`
- Create: `docs/schemas/f57-generation-approval-registry.v1.schema.json`
- Read: `docs/evidence/f57-foundation.v1.schema.json`
- Read: `docs/generated/f57/policy/p340-certification-policy.v1.json`
- Modify: `crates/platform/release/src/lib.rs`
- Modify: `crates/platform/release/Cargo.toml`
- Modify: `crates/foundation/src/lib.rs`
- Modify: `crates/foundation/Cargo.toml`
- Modify: `crates/platform/runtime/Cargo.toml`
- Modify: `Cargo.toml`
- Modify: `Cargo.lock`
- Modify: `crates/platform/runtime/src/lib.rs`
- Modify: `crates/platform/runtime/src/boot.rs`

**Interfaces:**
- Consumes: the G0-01 foundation nominals and zero-import foundation schema, the compiled-CapabilityGraph/generation/storage-manifest/P340-policy/current-readback type contracts needed to freeze deterministic topology builder tests, the signed-artifact verifier port, the product-pinned F-56 deployment trust/signing identity, and exactly one approved candidate-evidence signer provider mode. The recommended low-cost mode is self-hosted OS-keystore/PIV; connecting an existing enterprise signer is the equivalent optional branch. G0 consumes no verified deployment-specific storage manifest, DATA_HDD root, active generation, or production runtime readback; those inputs first become reachable in G1-01.
- Produces: foundation-owned `SignedBusinessArtifactV1<T>` whose schema shape composes `SignedBusinessArtifactEnvelopeV1` exactly once, private-field non-wire `VerifiedBusinessArtifactV1<T>` retaining the exact verified envelope bytes and verifier-only `VerifiedArtifactTrustPolicyV1`, closed private `ArtifactSigningErrorV1`, `ArtifactVerifier`, private `VerifiedCmsIssuanceWindowV1`, and the generated exhaustive issuance descriptor whose current-approved issued-at-only exact kind set is `SignedF57AuthorityStorageManifestV1|SignedGenerationManifestV1|SignedGenerationReversePlanV1`; private-field `ValidatedAbsolutePathV1` with its sole existing-canonical-absolute-file constructor; exact five-field `F57ArtifactSignerRegistryPayloadV1` including `ClientStackDecisionArchiveTrustAnchorPolicyV1`, its 89 rows, signed alias and bootstrap-verified lookup; the production-linkable `F57EvidenceTrustCoordinatorV1`, exact credential-requirement expansion, two provider modes, fixed authenticated broker protocol and crash-safe maintenance tool that together create or connect, verify, seal and install the real non-placeholder signer closure before the registry is committed; plus the sole Rust/schema three-root generation wire family `GenerationManifestV1|GenerationReversePlanV1|GenerationParticipantV1` with exact field counts `13/9/14`, item/participant/reverse-action vocabulary, private non-wire `VerifiedGenerationManifestV1`, and `GenerationStateV1`; the separately sole-owned seven-field `GenerationApprovalRegistryPayloadV1`, exact three five-field rows, private `VerifiedGenerationApprovalRegistryV1`, and `GenerationApprovalVerifierV1`; all four retention nominals `ArtifactPinStateV1|ArtifactPinLeaseV1|ArtifactPersistentReferenceKindV1|ArtifactPersistentReferenceV1`; and the sole Rust/schema topology family `RuntimeTopologyDeclarationV1|RuntimeTopologyCertificationV1` with pure deterministic `TopologyVerifier::build_declaration|build_certification`, private non-wire `VerifiedRuntimeTopologyDeclarationV1`, strict `TopologyVerifier::verify_declaration`, and frozen generation/approval/topology offline-descriptor contracts consumed later by G6. `generation.rs` owns the signed manifest, signed reverse plan and plain ACK nominals; `generation_approval.rs` owns only the approval registry and proof/verifier boundary; `participant.rs` imports the ACK nominal and owns behavior only. `GenerationManifestV1` and `GenerationReversePlanV1` are payload types whose only signed wires are their one `SignedBusinessArtifactV1<T>` envelopes; the ACK is strict plain JCS with `participant_apply_readback_ref` as its thirteenth field and `acknowledged_at_unix_ms` as its fourteenth field, and has no CMS/envelope. `ArtifactVerifier` may produce a generic proof only as the internal first stage. A generation consumer accepts only `VerifiedGenerationManifestV1` constructed by `GenerationApprovalVerifierV1` configured with private `VerifiedGenerationApprovalRegistryV1`, which itself can be constructed only after the product-pinned deployment trust and verified storage-manifest policy pin authenticate the fixed DATA_HDD registry path. G0 implements and fixture-tests these generation/approval/topology contracts but emits no production approval registry, generation manifest, reverse plan, participant ACK, deployment declaration or certification. Task 6—not Task 5—introduces the neutral gate-journal contract and its reference-only storage-root types because no Task-5 producer consumes them. After storage-manifest/DATA_HDD verification, G1-01 is the sole first production generation/declaration caller; G1-05 solely activates and persists participant ACKs. Expansion Task 14 owns the private release-layer authority and is the sole production certification caller after terminal P340 evidence. The foundation path nominal proves only canonical absolute file shape/existence; G1 alone adds storage trust/volume semantics. G0 owns only wire validation/live-at helpers; PostgreSQL retention transitions and aggregate reclamation remain G1-05.

Task 5 also delivers the one reusable Windows PowerShell trust boundary before any later plan may execute a `.ps1`. The low-cost default provisions a self-hosted CNG non-exportable code-signing leaf `CN=EP F57 PowerShell Code Signing,O=Enterprise Platform` under the product-pinned offline code-signing CA and a private-network RFC-3161 TSA `CN=EP F57 RFC3161 TSA,O=Enterprise Platform`; an existing enterprise code signer/TSA may be connected only through the same policy port and exact pinned trust fields. Private keys remain outside the application/data disks, ACL-bound to the release-signing identity, and never appear in a descriptor or backup. The public chain is installed into the approved Windows runner's machine `TrustedPublisher`/product trust stores through a two-person ceremony; the policy pins subject, issuer, SPKI SHA-256, Code Signing or Time Stamping EKU, validity, approved chain, offline CRL/OCSP snapshot digest and maximum revocation age. No public CA wildcard, Internet lookup, Trust-On-First-Use or user-store-only trust is accepted.

Every delivered script gets one strict `PowerShellScriptDescriptorV1` after its final edit, with exactly `schema_version,script_id,repo_path,signed_file_sha256,file_size_bytes,authenticode_content_digest_sha256,signer_subject_dn,signer_issuer_dn,signer_spki_sha256,signer_certificate_sha256,timestamp_subject_dn,timestamp_spki_sha256,rfc3161_gen_time_unix_ms,trust_policy_sha256`. The signing tool uses Windows SIP/CNG APIs from Rust, appends the Authenticode signature, obtains the approved RFC-3161 timestamp, fsyncs, reopens and verifies the final bytes, then emits that descriptor; a subsequent one-byte edit invalidates both signature and descriptor and requires an explicit re-sign. Descriptor verification independently recomputes the final whole-file SHA-256/size, Authenticode content digest, signer/chain/EKU/revocation state and RFC-3161 token against the pinned policy. G0 also creates the sole cumulative generator and its initially empty generated Rust/JSON registry. Every later script-owning task must, in the same commit and after final signing, atomically regenerate that registry and its independent literal fixture from all descriptors delivered through that task; the registry may only grow or replace the same script ID after its owned bytes change. The empty G0 registry rejects every execution. Task 14 does not create a second release registry: it performs the final regeneration and requires the exact 18-row transitive closure. No script is executable merely because it resides in the repository, and no task may execute its script before its descriptor is present in the current cumulative registry.

`powershell-trust-tool execute --script-id <closed-id> -- <registry-owned-arguments>` is the only allowed invocation boundary. It obtains the Windows directory through `GetWindowsDirectoryW`, opens `%SystemRoot%\System32\WindowsPowerShell\v1.0\powershell.exe` by component with no reparse point and delete/write sharing denied, resolves the final DOS/NT path and file identity, validates the protected Microsoft Authenticode/catalog identity, and holds that handle through child exit. It likewise opens the clean committed source root and descriptor-owned script path component-by-component with no reparse point and delete/write sharing denied. Before process creation, the attempt authority create-new appends and fsyncs an exact STARTED record and returns a private `VerifiedPowerShellExecutionStartedV1`; only then does the executor repeat file-ID, final-path, descriptor hash and Authenticode/RFC-3161 verification while holding the same handles and call `CreateProcessW` through a private `VerifiedWindowsCreateProcessInvocationV1`. That value fixes nonnull `lpApplicationName` to the verified final DOS path of the held host handle, nonnull `lpCurrentDirectory` to the verified final DOS path of the held clean-source-root handle, `bInheritHandles=FALSE`, `CREATE_UNICODE_ENVIRONMENT`, and one mutable NUL-terminated UTF-16 command-line buffer. The exact argv is `[canonical-host-path,"-NoProfile","-NonInteractive","-ExecutionPolicy","AllSigned","-File",canonical-absolute-script,...script-specific-argv]`; there is no shell/PATH/current-directory lookup and the caller can supply none of the fixed prefix. It verifies the effective machine policy is exactly `AllSigned`, the approved leaf/chain is present in the machine trust store, no weaker process/user policy won precedence, and host/script/source-root file identities remain unchanged after child creation and exit. A path alias, relative `-File`, caller command/script/prefix token, reparse component, alternate `powershell.exe`, stale descriptor, signer/TSA/revocation drift, policy other than effective AllSigned, replacement attempt, ambiguous Windows quoting, or STARTED-token mismatch fails before process creation. The tool exposes a separate ceremony-gated `sign` command but production dispatchers link only the execute library and cannot sign.

```rust
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct PowerShellScriptIdV1(String); // private; generated-registry values only

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct PowerShellScriptDescriptorV1 {
    pub schema_version: u32,
    pub script_id: String,
    pub repo_path: RepositoryRelativePathV1,
    pub signed_file_sha256: Sha256Digest,
    pub file_size_bytes: u64,
    pub authenticode_content_digest_sha256: Sha256Digest,
    pub signer_subject_dn: String,
    pub signer_issuer_dn: String,
    pub signer_spki_sha256: Sha256Digest,
    pub signer_certificate_sha256: Sha256Digest,
    pub timestamp_subject_dn: String,
    pub timestamp_spki_sha256: Sha256Digest,
    pub rfc3161_gen_time_unix_ms: i64,
    pub trust_policy_sha256: Sha256Digest,
}

pub struct VerifiedPowerShellScriptDescriptorV1 {
    value: PowerShellScriptDescriptorV1,
    descriptor_file_sha256: Sha256Digest,
}

pub trait PowerShellScriptTrustRegistryV1: Send + Sync {
    fn resolve(
        &self,
        script_id: &PowerShellScriptIdV1,
    ) -> Result<&VerifiedPowerShellScriptDescriptorV1, PowerShellTrustErrorV1>;
}

pub struct VerifiedWindowsCreateProcessInvocationV1 {
    /* private held handles + exact UTF-16 application/cwd/command-line buffers */
}
pub struct PreparedPowerShellExecutionV1 {
    /* private verified host/script/source-root handles + invocation */
}
pub struct PowerShellExecutionOutcomeV1 { /* private exit/readback */ }
pub struct VerifiedPowerShellExecutionStartedV1 { /* private exact durable binding */ }

#[derive(Clone, Copy, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum PowerShellExecutionAttemptStateV1 { Started, Completed, Unknown }

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PowerShellExecutionReconciliationV1 {
    CompletedExact,
    StillRunning,
    NoAuthoritativeOutcome,
    Conflict,
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct PowerShellExecutionAttemptRecordV1 {
    pub schema_version: u32,
    pub execution_attempt_id: UuidV1,
    pub idempotency_key_sha256: Sha256Digest,
    pub script_id: String,
    pub descriptor_file_sha256: Sha256Digest,
    pub script_specific_argv_sha256: Sha256Digest,
    pub host_file_identity_sha256: Sha256Digest,
    pub script_file_identity_sha256: Sha256Digest,
    pub source_root_file_identity_sha256: Sha256Digest,
    pub application_name_utf16le_sha256: Sha256Digest,
    pub current_directory_utf16le_sha256: Sha256Digest,
    pub command_line_utf16le_sha256: Sha256Digest,
    pub state: PowerShellExecutionAttemptStateV1,
    pub started_checkpoint_sha256: Sha256Digest,
    pub outcome_sha256: Option<Sha256Digest>,
}

pub trait PowerShellExecutionAttemptStoreV1: Send {
    fn begin_or_adopt_started(
        &mut self,
        prepared: &PreparedPowerShellExecutionV1,
        idempotency_key_sha256: Sha256Digest,
    ) -> Result<VerifiedPowerShellExecutionStartedV1, PowerShellTrustErrorV1>;
    fn complete_exact(
        &mut self,
        started: &VerifiedPowerShellExecutionStartedV1,
        outcome: &PowerShellExecutionOutcomeV1,
    ) -> Result<(), PowerShellTrustErrorV1>;
    fn mark_unknown_after_started(
        &mut self,
        started: &VerifiedPowerShellExecutionStartedV1,
    ) -> Result<(), PowerShellTrustErrorV1>;
    fn reconcile_started(
        &mut self,
        idempotency_key_sha256: Sha256Digest,
    ) -> Result<PowerShellExecutionReconciliationV1, PowerShellTrustErrorV1>;
}

pub trait PowerShellExecutionPortV1: Send + Sync {
    fn prepare_registered(
        &self,
        script_id: &PowerShellScriptIdV1,
        exact_registry_owned_arguments: &[String],
    ) -> Result<PreparedPowerShellExecutionV1, PowerShellTrustErrorV1>;

    fn execute_prepared_after_durable_started(
        &self,
        prepared: PreparedPowerShellExecutionV1,
        started: &VerifiedPowerShellExecutionStartedV1,
    ) -> Result<PowerShellExecutionOutcomeV1, PowerShellTrustErrorV1>;
}

pub enum PowerShellTrustErrorV1 {
    ScriptIdUnknown,
    DescriptorInvalid,
    PolicyMismatch,
    SignerOrTimestampInvalid,
    RevocationStateInvalid,
    TrustedPublisherInvalid,
    HostIdentityInvalid,
    PathOrReparseRejected,
    FileIdentityChanged,
    ArgumentShapeRejected,
    InvocationBindingInvalid,
    CommandLineRoundTripInvalid,
    EffectivePolicyNotAllSigned,
    AttemptConflict,
    AttemptAlreadyStarted,
    StartedBindingMismatch,
    ReconciliationRequired,
    ProcessCreateOrExitFailed,
}
```

`PowerShellExecutionAttemptStateV1` serializes exactly as `STARTED|COMPLETED|UNKNOWN`; the committed exact-record byte golden rejects Rust-case spellings and unknown fields. `VerifiedPowerShellScriptDescriptorV1`, `PowerShellScriptIdV1`, `VerifiedWindowsCreateProcessInvocationV1` and `VerifiedPowerShellExecutionStartedV1` have no public field/literal/string constructor. Only strict policy verification, the generated registry, the one Windows argv encoder/round-trip verifier, and `PowerShellExecutionAttemptStoreV1::begin_or_adopt_started` create them. The encoder rejects embedded NUL, then applies the frozen Windows CRT rule independently to every argv element: quote an empty value or one containing space, tab or quote; inside quotes double each backslash run immediately before a quote and add one escape backslash, double a trailing backslash run before the closing quote, preserve every other code unit, and join encoded arguments with exactly one U+0020. The mutable buffer is that exact UTF-16 sequence plus one terminal NUL and must contain at most 32,767 UTF-16 code units including the terminator. A separate parser must round-trip the produced buffer to the byte-for-byte logical argv before the value is verified. The STARTED record exact-binds the unpredictable attempt ID, derived idempotency key, script ID, descriptor-file digest, canonical script-specific argv digest, all three prepared file identities, and SHA-256 over the exact little-endian UTF-16 code units of nonnull `lpApplicationName`, nonnull `lpCurrentDirectory`, and the command-line buffer including its sole terminal NUL; the store fsyncs the record and checkpoint before minting the token. `execute_prepared_after_durable_started` consumes the non-cloneable prepared value and independently exact-matches every token field. A crash or response loss after STARTED may only adopt COMPLETED, durably mark UNKNOWN, or invoke the closed reconciler; it never starts the process again. The CLI derives its non-caller-selectable attempt store and idempotency key from `(script ID, descriptor digest, canonical argv, source-tree digest)`: a command carrying a verified `-BundleRoot/-RunJournal` uses that bundle's fixed `control/powershell-execution-attempts` lane, while engineering-only commands use the fixed clean-worktree `target/f57/powershell-engineering-attempts` lane. Candidate carriers implement the same port over their authenticated gate attempt/journal. Exact goldens cover empty, spaced, quoted, backslash-before-quote, trailing-backslash and non-BMP arguments plus the 32,767-unit boundary; embedded NUL, 32,768 units, wrong/nullable application name, wrong/nullable current directory, a parser mismatch, a second terminator, changed held-handle identity or any UTF-16 digest mismatch fails before `CreateProcessW`. No direct one-step execute overload, caller-supplied STARTED token, caller-selected attempt path, newest-attempt lookup, or retry-after-STARTED exists.

Task 5 declares the direct release-crate edges in this task, never retroactively: `ep-platform-release` keeps `ep-foundation|uuid|serde` from Task 1, adds workspace `sha2`, and adds dev-dependency `serde_json`; its generated implementation registry is compiled in all targets. At the G0 stage `release -> runtime` is absent. Expansion Task 14 is the sole later owner allowed to add that direct edge for topology certification and must update the locked metadata golden in the same change. Final metadata rejects `runtime -> release`, either lower crate importing the upper activation crate, path/git aliases, undeclared imports, cycles, and any release edge to adapters, apps or `xtask`.

The only four production decision-provider families are the disjoint `ArtifactSignerRegistryBootstrapAuthorizerV1`, `F57EvidenceSigningAuthorizerV1`, `GenerationApprovalRegistryBootstrapAuthorizerV1`, and `GenerationApprovalSigningAuthorizerV1`. The third owns only the signed generation-approval-registry payload under the pinned registry-authority SPKI/DN and its crash-safe maintenance operation; the fourth owns only the exact three child rows. Generated stable type-ID descriptors and cross-kind negatives make either bootstrap signer, candidate-evidence row or generation child row unusable in another family.

- [ ] **Step 1: Write failing state and topology tests.**

```rust
#[test]
fn generation_manifest_reverse_ack_owner_media_fields_and_envelope_digest_are_exact() {
    let schema = strict_schema("docs/evidence/f57-generation.v1.schema.json");
    assert_eq!(schema.imports(), ["f57-foundation.v1.schema.json"]);
    assert_eq!(schema.root_media_bindings(), [
        ("SignedBusinessArtifactV1<GenerationManifestV1>",
            "application/vnd.ep.f57-generation-manifest-v1+json"),
        ("SignedBusinessArtifactV1<GenerationReversePlanV1>",
            "application/vnd.ep.f57-generation-reverse-plan-v1+json"),
        ("GenerationParticipantV1",
            "application/vnd.ep.f57-generation-participant-ack-v1+json"),
    ]);
    assert!(schema.composes_foundation_detached_envelope_exactly_once_for_each_signed_root());
    assert!(schema.generation_participant_is_plain_internal_not_signed());
    assert_eq!(GenerationManifestV1::field_names(), [
        "schema_version", "purpose", "deployment_id", "authority_epoch",
        "generation_number", "previous_observed_generation_digest_sha256",
        "capability_graph_ref", "projection_manifest_ref", "storage_manifest_ref",
        "capacity_policy_definition_ref", "items", "required_participants",
        "issued_at_unix_ms",
    ]);
    assert_eq!(GenerationReversePlanV1::field_names(), [
        "schema_version", "purpose", "plan_id", "item_id", "action",
        "source_artifact_ref", "target_artifact_ref", "data_retention_mode",
        "issued_at_unix_ms",
    ]);
    assert_eq!(GenerationParticipantV1::field_names(), [
        "schema_version", "purpose", "deployment_id", "authority_epoch",
        "generation_number", "activation_attempt_id", "generation_manifest_ref",
        "generation_digest_sha256", "topology_declaration_ref", "participant_id",
        "participant_definition_sha256", "applied_item_set_sha256",
        "participant_apply_readback_ref",
        "acknowledged_at_unix_ms",
    ]);
    assert_eq!(GenerationManifestPurposeV1::wire_values(), [
        "EP-F57-GENERATION-MANIFEST-V1",
    ]);
    assert_eq!(GenerationReversePlanPurposeV1::wire_values(), [
        "EP-F57-GENERATION-REVERSE-PLAN-V1",
    ]);
    assert_eq!(GenerationParticipantPurposeV1::wire_values(), [
        "EP-F57-GENERATION-PARTICIPANT-ACK-V1",
    ]);
    assert_eq!(GenerationReverseActionV1::wire_values(), [
        "RESTORE_ARTIFACT", "DEACTIVATE_RETAIN_DATA", "NO_OP",
    ]);
    assert_eq!(GenerationDataRetentionModeV1::wire_values(), [
        "RETAIN_ALL_GENERATION_DATA",
    ]);
    assert_eq!(GenerationItemKindV1::wire_values(), [
        "CAPABILITY_GRAPH", "PROJECTION_SET", "MIGRATION_PLAN", "CAPABILITY_PACKAGE",
        "POLICY_BUNDLE", "UI_SCHEMA", "PROVIDER_MANIFEST", "RUNTIME_ARTIFACT",
    ]);

    let exact_envelope = read(
        "crates/platform/release/tests/fixtures/generation-manifest-v1.jcs.json");
    let generic_verified: VerifiedBusinessArtifactV1<GenerationManifestV1> =
        verify_generation_manifest_test_fixture(&exact_envelope).unwrap();
    assert_eq!(generic_verified.exact_envelope_jcs_bytes(), exact_envelope.as_slice());
    let payload_digest = generic_verified.payload_sha256();
    let verified_registry = fixture_verified_generation_approval_registry();
    let approval_registry_ref = verified_registry.artifact_ref().clone();
    let verifier = fixture_generation_approval_verifier(&verified_registry);
    let verified: VerifiedGenerationManifestV1 =
        verifier.verify_manifest(generic_verified).unwrap();
    let digest = verified.generation_digest_sha256();
    assert_eq!(digest, sha256(&exact_envelope));
    assert_eq!(verified.artifact_ref().sha256, digest);
    assert_eq!(verified.approval_registry_ref(), &approval_registry_ref);
    assert_eq!(verified.artifact_ref().media_type,
        "application/vnd.ep.f57-generation-manifest-v1+json");
    assert_ne!(digest, payload_digest);
    assert!(verified.payload().items_are_canonical_unique_and_all_have_reverse_plan());
    assert!(verified.payload().has_exactly_one_item(
        GenerationItemKindV1::CapabilityGraph,
        &verified.payload().capability_graph_ref,
    ));
    assert!(verified.payload().has_exactly_one_item(
        GenerationItemKindV1::ProjectionSet,
        &verified.payload().projection_manifest_ref,
    ));
    assert!(verified.payload().required_participants_are_canonical_graph_derivations());
    assert!(verified.payload().first_generation_iff_previous_observed_is_null());

    let reverse: SignedGenerationReversePlanV1 = strict_from_jcs(read(
        "crates/platform/release/tests/fixtures/generation-reverse-plan-v1.jcs.json")).unwrap();
    let verified_reverse = verifier.verify_reverse_plan(reverse).unwrap();
    assert!(verified_reverse.item_and_source_exact_match_manifest_item(verified.payload()));
    assert!(verified_reverse.action_target_and_retention_invariants_hold());

    let ack: GenerationParticipantV1 = strict_from_jcs(read(
        "crates/platform/release/tests/fixtures/generation-participant-ack-v1.jcs.json")).unwrap();
    assert_eq!(ack.generation_digest_sha256, digest);
    assert_eq!(ack.generation_manifest_ref.sha256, digest);
    assert!(ack.participant_definition_digest_matches_exact_runtime_participant_row());
    assert!(ack.applied_item_set_digest_matches_exact_required_subset());
    assert!(ack.participant_apply_readback_ref_exact_loads_same_attempt_successful_apply_readback());
    assert!(ack.acknowledged_at_is_after_same_attempt_durable_start_and_not_after_observed_commit());
    assert!(!f57_artifact_signer_registry().contains_artifact_kind("GENERATION_MANIFEST_V1"));
    assert!(!f57_artifact_signer_registry().contains_artifact_kind("GENERATION_REVERSE_PLAN_V1"));
    assert!(!f57_artifact_signer_registry().contains_artifact_kind("GENERATION_PARTICIPANT_V1"));
    assert!(!f57_artifact_signer_registry().contains_artifact_kind("GENERATION_APPROVAL_REGISTRY_V1"));
    assert!(g0_production_generation_reverse_approval_and_ack_call_sites().is_empty());
    assert_eq!(first_production_generation_caller(), "G1-01");
    assert_eq!(first_production_reverse_plan_caller(), "G1-01");
    assert_eq!(generation_creation_attempt_store_owner(), "G1-01");
    assert_eq!(generation_reverse_plan_authority_owner(), "G1-01");
    assert_eq!(participant_ack_persistence_owner(), "G1-05");
    assert_code(generation_digest_from_payload_or_graph_or_number(), "GENERATION_DIGEST_DOMAIN_INVALID");
    assert_code(double_wrapped_generation_manifest(), "GENERATION_ENVELOPE_SHAPE_INVALID");
    assert_code(activate_with_generic_generation_proof(), "GENERATION_APPROVAL_DOMAIN_REQUIRED");
    assert_code(approve_with_wrong_generation_approval_registry(), "GENERATION_APPROVAL_REGISTRY_MISMATCH");
    assert_code(approve_with_f57_evidence_signer_registry(), "GENERATION_APPROVAL_DOMAIN_REQUIRED");
    assert_code(manifest_with_missing_duplicate_or_mismatched_graph_projection_item(),
        "GENERATION_REQUIRED_ITEM_BINDING_INVALID");
    assert_code(reverse_plan_with_item_or_source_mismatch(), "GENERATION_REVERSE_PLAN_BINDING_MISMATCH");
    assert_code(reverse_restore_with_null_or_same_target(), "GENERATION_REVERSE_PLAN_TARGET_INVALID");
    assert_code(reverse_deactivate_with_target(), "GENERATION_REVERSE_PLAN_TARGET_INVALID");
    assert_code(reverse_noop_with_different_target_or_unsafe_kind(), "GENERATION_REVERSE_PLAN_TARGET_INVALID");
    assert_code(reverse_plan_with_nonretaining_mode(), "GENERATION_REVERSE_PLAN_RETENTION_INVALID");
    assert_code(generation_one_with_nonnull_previous_observed_digest(),
        "GENERATION_PREDECESSOR_INVALID");
    assert_code(later_generation_with_skipped_forked_or_desired_only_predecessor(),
        "GENERATION_PREDECESSOR_INVALID");
    assert_code(ack_with_other_attempt_digest_participant_or_item_set(), "GENERATION_ACK_BINDING_MISMATCH");
    assert_code(ack_with_missing_other_attempt_or_non_success_apply_readback_ref(),
        "GENERATION_ACK_APPLY_READBACK_MISMATCH");
    assert_code(ack_with_cms_signature_or_envelope(), "GENERATION_ACK_WIRE_INVALID");
    assert_code(ack_before_attempt_start_or_after_observed_commit(), "GENERATION_ACK_TIME_INVALID");
    assert_eq!(generation_offline_descriptor_contract(), (
        "docs/evidence/f57-generation.v1.schema.json",
        [
            ("SignedBusinessArtifactV1<GenerationManifestV1>",
             "application/vnd.ep.f57-generation-manifest-v1+json"),
            ("SignedBusinessArtifactV1<GenerationReversePlanV1>",
             "application/vnd.ep.f57-generation-reverse-plan-v1+json"),
            ("GenerationParticipantV1",
             "application/vnd.ep.f57-generation-participant-ack-v1+json"),
        ],
    ));
}

#[test]
fn generation_approval_registry_owner_pin_rows_and_bootstrap_are_exact() {
    let schema = strict_schema(
        "docs/schemas/f57-generation-approval-registry.v1.schema.json");
    assert_eq!(schema.imports(), ["../evidence/f57-foundation.v1.schema.json"]);
    assert_eq!(schema.root_media_bindings(), [
        ("GenerationApprovalRegistryV1",
            "application/vnd.ep.f57-generation-approval-registry-v1+json"),
    ]);
    assert!(schema.composes_foundation_detached_envelope_exactly_once());
    assert!(schema.closes_composed_root_with_unevaluated_properties_false());
    assert_eq!(GenerationApprovalRegistryPayloadV1::field_names(), [
        "schema_version", "purpose", "deployment_id", "revision", "rows",
        "issued_at_unix_ms", "expires_at_unix_ms",
    ]);
    assert_eq!(GenerationApprovalRegistryRowV1::field_names(), [
        "artifact_kind", "media_type", "signer_subject",
        "certificate_subject_dn", "validity_rule",
    ]);
    assert_eq!(GenerationApprovalRegistryPurposeV1::wire_values(), [
        "EP-F57-GENERATION-APPROVAL-REGISTRY-V1",
    ]);
    assert_eq!(GenerationApprovalArtifactKindV1::wire_values(), [
        "GENERATION_MANIFEST_V1", "GENERATION_REVERSE_PLAN_V1", "MIGRATION_PLAN_V1",
    ]);
    assert_eq!(GenerationApprovalValidityRuleV1::wire_values(), [
        "CURRENT_AT_VERIFICATION",
    ]);

    let registry = strict_signed_fixture(
        "crates/platform/release/tests/fixtures/generation-approval-registry-v1.jcs.json");
    assert_generation_approval_registry_exact_three_rows_media_spki_and_dns(&registry, [
        ("GENERATION_MANIFEST_V1",
         "application/vnd.ep.f57-generation-manifest-v1+json",
         "CN=EP Generation Manifest Authority,O=Enterprise Platform"),
        ("GENERATION_REVERSE_PLAN_V1",
         "application/vnd.ep.f57-generation-reverse-plan-v1+json",
         "CN=EP Generation Reverse Plan Authority,O=Enterprise Platform"),
        ("MIGRATION_PLAN_V1",
         "application/vnd.ep.f57-migration-plan-v1+json",
         "CN=EP Migration Plan Authority,O=Enterprise Platform"),
    ]);
    let storage = fixture_verified_storage_manifest_with_policy_ids([
        format!("generation-approval-registry-sha256:{}", sha256(registry.exact_bytes())),
    ]);
    let verified: VerifiedGenerationApprovalRegistryV1 =
        bootstrap_verify_generation_approval_registry(
            &storage,
            "generations/trust/generation-approval-registry.v1.json",
            product_pinned_deployment_trust_bundle(),
            registry,
        ).unwrap();
    assert_eq!(verified.payload().rows.len(), 3);
    assert_eq!(verified.payload().deployment_id, storage.deployment_id());
    assert_eq!(generation_trust_provisioning_owner(), "G1-01");
    assert_eq!(generation_trust_provider_role_count(), 4);
    assert_eq!(generation_trust_local_container_ids(), [
        "EP-F57-GENERATION-APPROVAL-REGISTRY-V1",
        "EP-F57-GENERATION-MANIFEST-V1",
        "EP-F57-GENERATION-REVERSE-PLAN-V1",
        "EP-F57-MIGRATION-PLAN-V1",
    ]);
    assert_generation_trust_adapters_are_exact_self_hosted_or_existing_enterprise_signer();
    assert_online_core_receives_only_manifest_and_reverse_plan_role_handles();
    assert_no_g0_production_generation_trust_key_registry_or_rotation_call_site();
    assert_code(registry_from_adjacent_path_or_ambient_windows_trust(),
        "GENERATION_APPROVAL_REGISTRY_BOOTSTRAP_INVALID");
    assert_code(registry_with_missing_duplicate_or_malformed_storage_policy_pin(),
        "GENERATION_APPROVAL_REGISTRY_PIN_INVALID");
    assert_code(registry_with_wrong_deployment_revision_time_row_media_dn_spki_or_validity(),
        "GENERATION_APPROVAL_REGISTRY_INVALID");
    assert_code(valid_old_registry_not_pinned_by_current_storage_manifest(),
        "GENERATION_APPROVAL_REGISTRY_PIN_MISMATCH");
    assert_code(registry_with_wildcard_default_self_root_or_f57_evidence_rows(),
        "GENERATION_APPROVAL_REGISTRY_INVALID");
    assert_eq!(generation_approval_offline_descriptor_contract(), (
        "docs/schemas/f57-generation-approval-registry.v1.schema.json",
        [("GenerationApprovalRegistryV1",
          "application/vnd.ep.f57-generation-approval-registry-v1+json")],
    ));
}

#[test]
fn generation_cannot_observe_without_exact_participant_acks() {
    let mut g = generation_fixture(["core", "worker"]);
    g.ack("core", g.digest()).unwrap();
    assert_eq!(g.observe().unwrap_err().code(), "GENERATION_ACK_SET_INCOMPLETE");
}

#[test]
fn later_generation_requires_exact_immediately_prior_durable_observed_envelope_digest() {
    let durable_observed = durable_observed_generation_fixture(7);
    let next = generation_manifest_fixture(8, Some(durable_observed.exact_envelope_digest()));
    assert!(validate_generation_predecessor(&next, &durable_observed).is_ok());
    assert_code(validate_generation_predecessor(
        &next.with_previous_digest(desired_only_generation_fixture(7).exact_envelope_digest()),
        &durable_observed,
    ), "GENERATION_PREDECESSOR_INVALID");
    assert_code(validate_generation_predecessor(
        &next.with_generation_number(9),
        &durable_observed,
    ), "GENERATION_PREDECESSOR_INVALID");
}

#[test]
fn topology_declaration_wire_media_owner_and_actual_readback_are_exact() {
    let graph = fixture_compiled_capability_graph();
    let build_facts = nonproduction_declaration_build_facts_fixture();
    let actual = actual_runtime();
    let declaration = TopologyVerifier::build_declaration(
        &graph,
        build_facts,
    ).unwrap();
    let bytes = read("crates/platform/runtime/tests/fixtures/runtime-topology-declaration-v1-golden.json");
    let artifact = content_addressed_ref(
        &bytes,
        "application/vnd.ep.f57-runtime-topology-declaration-v1+json",
    );
    assert_eq!(declaration.purpose, RuntimeTopologyDeclarationPurposeV1::RuntimeTopologyDeclaration);
    assert_eq!(declaration.hardware_profile_id.as_str(),
        "THINKSTATION_P340_I5_10500_32GB_256GB_SSD_1TB_HDD");
    assert_eq!(declaration.storage_profile_id.as_str(), "SINGLE_DISK_DEGRADED_PRODUCTION");
    assert_eq!(declaration.workload_profile_id.as_str(), "F57_P340_15_3_2_1__11_5_2_2_V1");
    assert_eq!(declaration.media_type(),
        "application/vnd.ep.f57-runtime-topology-declaration-v1+json");
    assert_eq!(canonical_jcs(&declaration), bytes);
    let verified = TopologyVerifier::verify_declaration(&artifact, &bytes, &fixture_readback_provider(actual)).unwrap();
    assert_eq!(verified.artifact_ref(), &artifact);
    assert_eq!(verified.declaration(), &declaration);
    assert!(declaration.participants_and_database_consumers_are_exact_graph_derivations(&graph));
    assert_code(
        verify_topology_fixture_with_database_consumer("integration-gateway"),
        "TOPOLOGY_DATABASE_CONSUMER_FORBIDDEN",
    );
    assert_code(TopologyVerifier::verify_declaration(
        &artifact, &bytes, &fixture_readback_provider(actual_runtime_with_drift())), "TOPOLOGY_ACTUAL_DRIFT");
    assert_code(TopologyVerifier::verify_declaration(
        &artifact.with_media_type("application/json"), &bytes,
        &fixture_readback_provider(actual_runtime())),
        "TOPOLOGY_DECLARATION_ARTIFACT_REF_MISMATCH");
    assert_code(TopologyVerifier::verify_declaration(
        &artifact, &noncanonical_or_digest_drift_bytes(&bytes),
        &fixture_readback_provider(actual_runtime())),
        "TOPOLOGY_DECLARATION_ARTIFACT_REF_MISMATCH");
    assert!(!declaration.contains_candidate_soak_capacity_or_certification_fields());
    assert!(!g0_gate_contract().contains_runtime_topology_declaration_ref_or_artifact());
    assert_code(g0_attempt_to_store_deployment_runtime_topology_declaration(),
        "F57_RUNTIME_TOPOLOGY_DECLARATION_NOT_DUE");
}

#[test]
fn topology_schema_owns_both_plain_roots_and_only_g6_may_authorize_production_certification() {
    let schema = strict_schema("docs/evidence/f57-runtime-topology.v1.schema.json");
    assert_eq!(generated_runtime_topology_schema_bytes(),
        read("docs/evidence/f57-runtime-topology.v1.schema.json"));
    assert_eq!(schema.imports(), ["f57-foundation.v1.schema.json"]);
    assert_eq!(schema.root_media_bindings(), [
        ("RuntimeTopologyDeclarationV1", "application/vnd.ep.f57-runtime-topology-declaration-v1+json"),
        ("RuntimeTopologyCertificationV1", "application/vnd.ep.f57-runtime-topology-certification-v1+json"),
    ]);
    assert_eq!(schema.purpose_wires(), [
        "EP-F57-RUNTIME-TOPOLOGY-DECLARATION-V1",
        "EP-F57-RUNTIME-TOPOLOGY-CERTIFICATION-V1",
    ]);
    assert_eq!(schema.declaration_field_names(), [
        "purpose", "schema_version", "deployment_id", "authority_epoch",
        "generation_digest_sha256", "capability_graph_digest_sha256",
        "hardware_profile_id", "storage_profile_id", "workload_profile_id",
        "storage_manifest_ref", "capacity_policy_definition_ref",
        "participants", "database_consumers",
    ]);
    assert_eq!(schema.certification_field_names(), [
        "schema_version", "purpose", "candidate_run", "candidate_manifest_ref",
        "topology_declaration_ref", "p340_soak_evidence_ref",
        "capacity_certificate_ref", "certified_host_fingerprint_sha256",
        "hardware_profile_id", "storage_profile_id", "workload_profile_id",
        "certified_concurrent_users", "certified_continuous_duration_seconds",
    ]);
    assert!(schema.both_roots_are_strict_plain_jcs_not_signed_envelopes());
    assert!(schema.owns_all_and_only_runtime_topology_nominals());
    assert!(!schema.imports_graph_release_p340_or_client_leaf_schema());
    assert!(!schema.defines_or_composes_signed_business_artifact_envelope());
    assert!(workspace_dependency_edge_exists(
        "ep-platform-runtime", "ep-platform-capability-graph"));
    assert!(!workspace_dependency_edge_exists(
        "ep-platform-capability-graph", "ep-platform-runtime"));
    assert!(workspace_dependency_dag_is_acyclic());
    let certification = TopologyVerifier::build_certification(
        fixture_candidate_run(),
        fixture_candidate_manifest_ref(),
        fixture_topology_declaration_ref(),
        fixture_p340_soak_evidence_ref(),
        fixture_capacity_certificate_ref(),
        fixture_certified_host_fingerprint(),
        fixture_hardware_profile_id(),
        fixture_storage_profile_id(),
        fixture_workload_profile_id(),
        20,
        259_200,
    ).unwrap();
    assert_eq!(certification.media_type(),
        "application/vnd.ep.f57-runtime-topology-certification-v1+json");
    assert_eq!(canonical_jcs(&certification), read(
        "crates/platform/runtime/tests/fixtures/runtime-topology-certification-v1-golden.json"
    ));
    assert_code(g0_attempt_to_authorize_or_store_production_topology_certification(),
        "F57_RUNTIME_TOPOLOGY_CERTIFICATION_NOT_DUE");
    assert_eq!(topology_offline_descriptor_contract(), TopologyOfflineDescriptorContractV1 {
        schema_path: "docs/evidence/f57-runtime-topology.v1.schema.json",
        root_media_bindings: [
            ("RuntimeTopologyDeclarationV1", "application/vnd.ep.f57-runtime-topology-declaration-v1+json"),
            ("RuntimeTopologyCertificationV1", "application/vnd.ep.f57-runtime-topology-certification-v1+json"),
        ],
    });
    assert_code(second_topology_schema_owner_or_signer_registry_row(), "F57_TOPOLOGY_OWNER_DUPLICATE");
}

#[test]
fn artifact_pin_wire_has_exact_lifecycle_and_live_at_semantics() {
    let lease = ArtifactPinLeaseV1::active(
        fixture_lease_id(),
        fixture_execution(),
        "core-server",
        fixture_generation_digest(),
        fixture_artifact_digest(),
        1_000,
        301_000,
    ).unwrap();
    assert!(lease.is_live_at(300_999));
    assert!(!lease.is_live_at(301_000));
    assert_eq!(lease.state, ArtifactPinStateV1::Active);
    assert!(ArtifactPinLeaseV1::from_jcs_with_unknown_field().is_err());
    assert_eq!(canonical_jcs(&lease), read("crates/platform/release/tests/fixtures/artifact-pin-active-v1.jcs.json"));
}

#[test]
fn signed_artifact_signer_registry_bootstrap_and_exact_rows_are_closed() {
    let source = read("docs/f57-artifact-signer-registry.v1.json");
    let verified = bootstrap_verify_f57_artifact_signer_registry(
        &source,
        deployment_pinned_corporate_root_fixture(),
    ).unwrap();
    assert_eq!(verified.payload().schema_version, 1);
    assert_eq!(verified.payload().purpose, F57ArtifactSignerRegistryPurposeV1::ArtifactSignerRegistry);
    assert_eq!(F57ArtifactSignerRegistryPayloadV1::field_names(), [
        "schema_version", "purpose", "rows",
        "client_stack_decision_archive_trust_anchor_policy", "issued_at_unix_ms",
    ]);
    assert_eq!(verified.media_type(), "application/vnd.ep.f57-artifact-signer-registry-v1+json");
    assert_eq!(verified.signer_subject(), spki_subject_token_of(deployment_manifest_signer_fixture()));
    assert_eq!(verified.signer_certificate_subject_dn(), "CN=EP F57 Artifact Signer Registry Authority,O=Enterprise Platform");
    assert_eq!(verified.rows().len(), 89);
    assert!(verified.rows_are_canonical_unique_by_artifact_kind_and_discriminator());
    assert!(verified.rows_have_exact_spki_tokens_and_certificate_dn_constraints());
    assert_eq!(verified.historical_validity_rows(), [
        ("CLIENT_STACK_DECISION_V1", "", F57SignerValidityRuleV1::ValidAtTrustedSigningTime),
    ]);
    assert!(verified.other_rows_use(F57SignerValidityRuleV1::CurrentAtVerification));
    let policy = &verified.payload().client_stack_decision_archive_trust_anchor_policy;
    assert_eq!(ClientStackDecisionArchiveTrustAnchorPolicyV1::field_names(), [
        "schema_version", "purpose", "roles",
    ]);
    assert_eq!(ClientStackDecisionArchiveTrustAnchorRoleV1::field_names(), [
        "chain_role", "root_certificate_sha256s",
    ]);
    assert_eq!(policy.schema_version, 1);
    assert_eq!(policy.purpose,
        ClientStackDecisionArchiveTrustAnchorPolicyPurposeV1::ClientStackDecisionArchiveTrustAnchorPolicy);
    assert_eq!(ClientStackDecisionArchiveCertificateChainRoleV1::wire_values(), [
        "DECISION_SIGNER", "TSA",
    ]);
    assert!(policy.has_exact_sorted_roles_with_nonempty_sorted_unique_der_root_digests([
        ClientStackDecisionArchiveCertificateChainRoleV1::DecisionSigner,
        ClientStackDecisionArchiveCertificateChainRoleV1::Tsa,
    ]));
    assert!(!verified.contains_wildcard_or_default());
    assert!(!verified.contains_artifact_kind("GATE_RECEIPT_REF_V1"));
    assert!(!verified.contains_pair("SIGNED_ARTIFACT_REF_V1", "windows-authority"));
    assert!(verified.contains_pair("WINDOWS_AUTHORITY_ARTIFACT_SET_V1", ""));
    assert!(release_authority_ref_golden().is_direct_windows_authority_artifact_set_ref());
    assert_eq!(generated_artifact_signer_registry_schema_bytes(), read("docs/schemas/f57-artifact-signer-registry.v1.schema.json"));
    let registry_schema = strict_schema("docs/schemas/f57-artifact-signer-registry.v1.schema.json");
    assert_eq!(registry_schema.imports(), ["../evidence/f57-foundation.v1.schema.json"]);
    assert!(registry_schema.composes_foundation_detached_envelope_exactly_once());
    assert!(registry_schema.owns_only_registry_payload_rows_archive_trust_policy_and_refinements());
    assert!(!registry_schema.redefines_foundation_nominals_or_envelope_fields());
    assert_code(bootstrap_registry_with_dn_as_signer_subject(), "F57_SIGNER_SUBJECT_NOT_SPKI_TOKEN");
    assert_code(bootstrap_registry_same_dn_different_spki(), "F57_SIGNER_REGISTRY_BOOTSTRAP_SUBJECT_MISMATCH");
    assert_code(registry_row_spki_or_dn_drift(), "F57_SIGNER_REGISTRY_INVALID");
    assert_code(registry_payload_missing_archive_policy(), "F57_SIGNER_REGISTRY_INVALID");
    assert_code(registry_with_extra_missing_duplicate_unsorted_or_empty_archive_policy_role(),
        "F57_SIGNER_REGISTRY_ARCHIVE_TRUST_POLICY_INVALID");
    assert_code(registry_with_archive_policy_root_digest_or_purpose_drift(),
        "F57_SIGNER_REGISTRY_ARCHIVE_TRUST_POLICY_INVALID");
    assert_code(unsigned_or_88_row_registry(), "F57_SIGNER_REGISTRY_INVALID");
}

#[test]
fn issued_at_only_current_configuration_issuance_descriptors_are_exact_and_exhaustive() {
    let rows = generated_business_artifact_issuance_descriptors()
        .rows_for(IssuanceCategoryV1::CurrentApprovedIssuedAtOnly);
    assert_eq!(rows.kind_names(), [
        "SignedF57AuthorityStorageManifestV1",
        "SignedGenerationManifestV1",
        "SignedGenerationReversePlanV1",
    ]);
    assert!(rows.all_use(F57SignerValidityRuleV1::CurrentAtVerification));
    assert!(rows.all_derive_checked_inclusive_window_from_issued_at(
        |issued_at_unix_ms| (issued_at_unix_ms, issued_at_unix_ms + 300_000)));
    assert_current_configuration_issuance_window_boundaries(
        signed_generation_manifest_fixture(), 1_000, [1_000, 301_000]);
    assert_current_configuration_issuance_window_boundaries(
        signed_generation_reverse_plan_fixture(), 2_000, [2_000, 302_000]);
    assert_code(current_configuration_signature_before_issued_or_after_five_minutes(),
        "F57_CMS_SIGNING_TIME_OUTSIDE_ISSUANCE_WINDOW");
    assert_code(current_configuration_issuance_window_i64_overflow(),
        "F57_CMS_ISSUANCE_WINDOW_OVERFLOW");
    assert_code(issuance_descriptor_with_overlapping_or_unknown_kind(),
        "F57_CMS_ISSUANCE_DESCRIPTOR_INVALID");
    assert_code(current_configuration_descriptor_using_static_none(),
        "F57_CMS_ISSUANCE_DESCRIPTOR_INVALID");
    assert_eq!(static_none_descriptor_kind_names(), ["F57ArtifactSignerRegistryV1"]);
}
```

```rust
#[test]
fn evidence_trust_expansion_provider_and_broker_are_exact_and_fail_closed() {
    let requirements = expand_f57_evidence_credential_requirements().unwrap();
    assert!(requirements.is_exact_bijection_over_all_89_registry_rows());
    assert!(requirements.has_exactly_one_registry_bootstrap_authority());
    assert!(requirements.has_exact_archive_tsa_requirement());
    assert_eq!(F57EvidenceTrustProviderModeV1::wire_values(), [
        "SELF_HOSTED_OS_KEYSTORE_PIV_V1",
        "EXISTING_ENTERPRISE_SIGNER_V1",
    ]);
    assert_eq!(broker_endpoint_for(HostOsV1::Windows),
        r"\\.\pipe\EnterprisePlatform\F57EvidenceSignerV1");
    assert_eq!(broker_endpoint_for(HostOsV1::Macos),
        "/var/run/enterprise-platform/f57-evidence-signer-v1.sock");
    assert_code(provision_with_missing_or_exportable_role(),
        "F57_EVIDENCE_TRUST_ROLE_INVALID");
    assert_code(sign_with_unregistered_row_runner_dn_or_payload(),
        "F57_EVIDENCE_SIGNING_REQUEST_REJECTED");
    assert_code(seal_after_torn_or_changed_ceremony(),
        "F57_EVIDENCE_TRUST_CEREMONY_CONFLICT");
}
```

```rust
#[test]
fn powershell_trust_policy_descriptor_and_fixed_executor_are_closed() {
    assert_exact_self_hosted_or_connected_signer_and_rfc3161_provider_modes();
    assert_descriptor_has_exact_14_fields_and_no_key_material();
    assert_sign_then_verify_round_trip_freezes_final_file_hash_content_digest_leaf_and_timestamp();
    assert_one_byte_post_signature_edit_is_rejected();
    assert_fixed_host_is_get_windows_directory_system32_windows_powershell_v1_and_microsoft_verified();
    assert_executor_holds_no_reparse_final_handles_with_write_delete_sharing_denied_through_exit();
    assert_executor_reverifies_same_file_ids_hashes_and_signatures_after_durable_started();
    assert_create_process_uses_verified_nonnull_application_and_source_root_current_directory();
    assert_windows_argv_utf16_round_trip_and_started_digests_are_exact();
    assert_argv_goldens_cover_empty_space_quote_backslash_trailing_slash_non_bmp_and_32767_limit();
    assert_effective_machine_execution_policy_is_exact_all_signed();
    assert_code(relative_script_path_or_path_host_lookup(), "F57_POWERSHELL_EXECUTION_PATH_FORBIDDEN");
    assert_code(untrusted_publisher_or_tsa_or_stale_revocation(), "F57_POWERSHELL_TRUST_INVALID");
    assert_code(caller_supplied_script_command_or_argument_shape(), "F57_POWERSHELL_DISPATCH_NOT_REGISTERED");
    assert_code(embedded_nul_or_32768_utf16_units(), "F57_POWERSHELL_ARGUMENT_SHAPE_REJECTED");
    assert_code(nullable_or_wrong_application_name_or_current_directory(), "F57_POWERSHELL_INVOCATION_BINDING_INVALID");
    assert_code(command_line_round_trip_or_started_utf16_digest_mismatch(), "F57_POWERSHELL_STARTED_BINDING_MISMATCH");
}
```

- [ ] **Step 2: Run narrow tests and verify RED.**

Run: `cargo test -p ep-platform-release --all-targets --locked --test generation --test generation_approval -- --nocapture`

Expected: FAIL because generation, reverse-plan, participant, generation-approval, and signed artifact-signer registry modules do not exist.

Run: `cargo test -p ep-platform-runtime --test topology -- --nocapture`

Expected: FAIL because topology modules do not exist.

Run: `cargo test -p ep-platform-evidence-trust -p ep-adapter-kms -p evidence-trust-tool -p evidence-signing-broker --all-targets`

Run on Windows: `cargo test -p ep-platform-powershell-trust -p powershell-trust-tool --all-targets`

Expected: FAIL because the pinned policy, strict script descriptor, CNG/SIP signing path, fixed-host executor and final-handle protections do not exist.

Expected: FAIL because the role-expansion owner, both provider modes, protected ceremony journal and fixed-endpoint broker do not exist.

- [ ] **Step 3: Implement exact state and topology validation.**

```rust
#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct SignedBusinessArtifactV1<T> {
    pub payload: T,
    pub payload_sha256: Sha256Digest,
    pub signer_subject: SignerSpkiTokenV1,
    pub signature_cms_b64url: String,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ArtifactSigningErrorV1 {
    SignerUnavailable,
    SignerSubjectMismatch,
    AuthorizationMismatch,
    SigningOperationConflict,
    DurableResultUnavailable,
    InvalidIssuanceWindow,
    AlgorithmRejected,
    CmsGenerationFailed,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(tag = "mode", rename_all = "SCREAMING_SNAKE_CASE", deny_unknown_fields)]
pub enum CmsIssuanceWindowProjectionV1 {
    Inclusive {
        not_before_unix_ms: i64,
        not_after_unix_ms: i64,
    },
    StaticNone,
}

pub struct VerifiedCmsIssuanceWindowV1 {
    projection: CmsIssuanceWindowProjectionV1,
}

pub struct ProviderSignerHandleIdV1(Box<[u8]>);

impl ProviderSignerHandleIdV1 {
    pub fn try_from_bytes(bytes: Vec<u8>) -> Result<Self, ArtifactSigningErrorV1> {
        if bytes.is_empty() || bytes.len() > 4096 {
            return Err(ArtifactSigningErrorV1::AuthorizationMismatch);
        }
        Ok(Self(bytes.into_boxed_slice()))
    }
    pub fn as_bytes(&self) -> &[u8] { &self.0 }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(transparent)]
pub struct CmsArtifactTypeIdV1(&'static str);

impl CmsArtifactTypeIdV1 {
    /* generated public associated constants for the exact legal signed payload set */
    pub fn as_str(self) -> &'static str { self.0 }
}

pub struct CmsArtifactDescriptorProjectionV1 {
    pub descriptor_ordinal: u16,
    pub artifact_type_id: CmsArtifactTypeIdV1,
    pub artifact_kind: &'static str,
    pub discriminator: &'static str,
    pub producer_role: &'static str,
    pub runner_id: Option<&'static str>,
    pub trust_domain: &'static str,
    pub issuance_rule: &'static str,
}

pub struct CmsArtifactDescriptorV1 {
    projection: CmsArtifactDescriptorProjectionV1,
    descriptor_sha256: Sha256Digest,
}

impl CmsArtifactDescriptorV1 {
    pub fn projection(&self) -> &CmsArtifactDescriptorProjectionV1 { &self.projection }
    pub fn descriptor_sha256(&self) -> Sha256Digest { self.descriptor_sha256 }
}

pub fn registered_cms_artifact_descriptor_v1(
    artifact_type_id: CmsArtifactTypeIdV1,
) -> Result<&'static CmsArtifactDescriptorV1, ArtifactSigningErrorV1>;

pub trait CmsSignableArtifactV1: serde::Serialize + Send + Sync + 'static {
    fn cms_artifact_type_id(&self) -> CmsArtifactTypeIdV1;
}

pub struct CmsAuthorizationDecisionV1 {
    pub signing_operation_id: UuidV1,
    pub issuance_window: CmsIssuanceWindowProjectionV1,
    pub signing_context_sha256: Sha256Digest,
    pub signer_role: String,
    pub signer_subject: SignerSpkiTokenV1,
    pub certificate_subject_dn: String,
    pub provider_handle: ProviderSignerHandleIdV1,
}

pub trait CmsAuthorizationDecisionProviderV1<T: CmsSignableArtifactV1>: Send + Sync {
    fn authorize(
        &self,
        payload: &T,
        descriptor: &CmsArtifactDescriptorV1,
        payload_sha256: Sha256Digest,
    ) -> Result<CmsAuthorizationDecisionV1, ArtifactSigningErrorV1>;
}

pub struct CmsAuthorizationBindingV1 {
    pub signing_operation_id: UuidV1,
    pub descriptor_sha256: Sha256Digest,
    pub payload_sha256: Sha256Digest,
    pub issuance_window_sha256: Sha256Digest,
    pub signing_context_sha256: Sha256Digest,
    pub signer_role: String,
    pub signer_subject: SignerSpkiTokenV1,
    pub certificate_subject_dn: String,
    pub provider_handle_sha256: Sha256Digest,
}

pub struct CmsProviderSigningRequestV1 {
    pub binding: CmsAuthorizationBindingV1,
    pub authorization_sha256: Sha256Digest,
    pub issuance_window: CmsIssuanceWindowProjectionV1,
    pub provider_handle: ProviderSignerHandleIdV1,
}

pub struct VerifiedCmsSigningAuthorizationV1 {
    provider_request: CmsProviderSigningRequestV1,
}

impl VerifiedCmsSigningAuthorizationV1 {
    pub fn provider_request(&self) -> &CmsProviderSigningRequestV1 {
        &self.provider_request
    }
}

pub struct VerifiedCmsSigningRequestV1<T: CmsSignableArtifactV1> {
    payload: T,
    descriptor: &'static CmsArtifactDescriptorV1,
    authorization: VerifiedCmsSigningAuthorizationV1,
}

pub fn prepare_cms_signing_request_v1<
    T: CmsSignableArtifactV1,
    A: CmsAuthorizationDecisionProviderV1<T>,
>(
    payload: T,
    authorizer: &A,
) -> Result<VerifiedCmsSigningRequestV1<T>, ArtifactSigningErrorV1>;

pub struct DetachedCmsProviderResultV1 {
    pub signer_subject: SignerSpkiTokenV1,
    pub signature_cms_b64url: String,
}

pub trait DetachedCmsSignerV1: Send + Sync {
    fn sign_authorized(
        &self,
        authorization: &VerifiedCmsSigningAuthorizationV1,
    ) -> Result<DetachedCmsProviderResultV1, ArtifactSigningErrorV1>;
}

pub fn sign_business_artifact_v1<T: CmsSignableArtifactV1>(
    signer: &impl DetachedCmsSignerV1,
    request: VerifiedCmsSigningRequestV1<T>,
) -> Result<SignedBusinessArtifactV1<T>, ArtifactSigningErrorV1>;

// Non-wire proof object. Only ArtifactVerifier may construct it after complete
// typed media/purpose/trust/time/digest/signature/JCS verification.
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(tag = "kind", rename_all = "SCREAMING_SNAKE_CASE", deny_unknown_fields)]
pub enum ArtifactTrustAuthorityBindingV1 {
    PinnedRootSet { root_set_sha256: Sha256Digest },
    SignedRegistry {
        registry_ref: ArtifactRefV1,
        registry_whole_envelope_sha256: Sha256Digest,
    },
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ArtifactTrustTimeRuleV1 {
    CurrentAtVerification,
    ValidAtTrustedSigningTime,
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct ArtifactTrustPolicyDescriptorV1 {
    schema_version: u32,
    trust_domain_id: String,
    authority_binding: ArtifactTrustAuthorityBindingV1,
    required_purpose: String,
    required_media_type: String,
    required_signer_subject: Option<SignerSpkiTokenV1>,
    required_certificate_subject_dn: Option<String>,
    validity_rule: ArtifactTrustTimeRuleV1,
    revocation_checkpoint_policy_sha256: Sha256Digest,
}

impl ArtifactTrustPolicyDescriptorV1 {
    pub fn trust_domain_id(&self) -> &str { &self.trust_domain_id }
    pub fn required_purpose(&self) -> &str { &self.required_purpose }
    pub fn required_media_type(&self) -> &str { &self.required_media_type }
    pub fn validity_rule(&self) -> ArtifactTrustTimeRuleV1 { self.validity_rule }
}

pub fn artifact_trust_policy_sha256_v1(
    descriptor: &ArtifactTrustPolicyDescriptorV1,
) -> Result<Sha256Digest, ArtifactSigningErrorV1>;

// Sole policy-identity preimage/helper used by ArtifactVerifier and the evidence-input
// contract resolver. The exact nine fields above are JCS-hashed with schema_version=1.
// No current clock, validation result, fetched chain, response, or mutable "now" enters it.

pub struct VerifiedArtifactTrustPolicyV1 {
    descriptor: ArtifactTrustPolicyDescriptorV1,
    policy_sha256: Sha256Digest,
}

impl VerifiedArtifactTrustPolicyV1 {
    pub fn descriptor(&self) -> &ArtifactTrustPolicyDescriptorV1 { &self.descriptor }
    pub fn policy_sha256(&self) -> Sha256Digest { self.policy_sha256 }
}

pub struct VerifiedArtifactTrustAuthorityV1 {
    /* private actual root/registry selection, leaf, chain, revocation and time receipt */
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ArtifactVerifierBootstrapProfileV1 {
    F57ProductDeployment,
}

struct EmbeddedProductDeploymentPinSetV1 {
    product_build_sha256: Sha256Digest,
    root_set_sha256: Sha256Digest,
    root_certificate_sha256s: &'static [Sha256Digest],
    required_manifest_purpose: &'static str,
    required_leaf_eku_oids: &'static [&'static str],
    rotation_policy_sha256: Sha256Digest,
}

// Private build-generated map covered by the independently signed product manifest
// and MANIFEST.sha256; no caller supplies or overrides these pins.
fn embedded_product_deployment_pin_set_v1(
    profile: ArtifactVerifierBootstrapProfileV1,
) -> &'static EmbeddedProductDeploymentPinSetV1;

pub struct ArtifactVerifierBootstrapInputV1<'a> {
    pub profile: ArtifactVerifierBootstrapProfileV1,
    pub exact_deployment_manifest_jcs_bytes: &'a [u8],
    pub exact_deployment_manifest_signature_der: &'a [u8],
    pub exact_deployment_trust_bundle_der: &'a [u8],
    pub exact_product_trust_root_der: &'a [u8],
    pub exact_revocation_bytes: &'a [u8],
    pub exact_checkpoint_bytes: &'a [u8],
}

pub enum ArtifactTrustAuthorityEvidenceV1<'a> {
    ConfiguredPinnedRoot {
        configured_policy_id: &'static str,
        exact_chain_der_leaf_to_root: &'a [Box<[u8]>],
        exact_revocation_bytes: &'a [u8],
        exact_checkpoint_bytes: &'a [u8],
    },
    SignedRegistrySelection {
        verified_registry_envelope: &'a VerifiedSignedEnvelopeBytesV1,
        registry_ref: &'a ArtifactRefV1,
        generated_selection_id: &'static str,
    },
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ArtifactVerificationErrorV1 {
    InvalidWire,
    ArtifactRefMismatch,
    DescriptorMismatch,
    TrustAuthorityInvalid,
    RegistrySelectionInvalid,
    SignatureRejected,
    SignerSubjectMismatch,
    CertificateSubjectMismatch,
    TimeInvalid,
    RevocationInvalid,
}

pub struct ArtifactVerifierV1 {
    /* private configured pins + offline CMS/chain/revocation engine + trusted-time source */
}

impl ArtifactVerifierV1 {
    pub fn bootstrap_product_pinned(
        input: ArtifactVerifierBootstrapInputV1<'_>,
    ) -> Result<Self, ArtifactVerificationErrorV1>;

    pub fn verify<T: CmsSignableArtifactV1 + serde::de::DeserializeOwned>(
        &self,
        exact_envelope_jcs_bytes: &[u8],
        expected_ref: &ArtifactRefV1,
        authority_evidence: ArtifactTrustAuthorityEvidenceV1<'_>,
    ) -> Result<VerifiedBusinessArtifactV1<T>, ArtifactVerificationErrorV1>;
}

pub struct VerifiedSignedEnvelopeBytesV1 {
    exact_envelope_jcs_bytes: Box<[u8]>,
    artifact_type_id: CmsArtifactTypeIdV1,
    media_type: String,
    whole_envelope_sha256: Sha256Digest,
    verified_trust_policy: VerifiedArtifactTrustPolicyV1,
}

impl VerifiedSignedEnvelopeBytesV1 {
    pub fn exact_envelope_jcs_bytes(&self) -> &[u8] { &self.exact_envelope_jcs_bytes }
    pub fn artifact_type_id(&self) -> CmsArtifactTypeIdV1 { self.artifact_type_id }
    pub fn media_type(&self) -> &str { &self.media_type }
    pub fn whole_envelope_sha256(&self) -> Sha256Digest { self.whole_envelope_sha256 }
    pub fn verified_trust_policy_sha256(&self) -> Sha256Digest {
        self.verified_trust_policy.policy_sha256()
    }
    pub fn verified_trust_policy(&self) -> &VerifiedArtifactTrustPolicyV1 {
        &self.verified_trust_policy
    }
}

pub struct VerifiedBusinessArtifactV1<T> {
    signed: SignedBusinessArtifactV1<T>,
    artifact_ref: ArtifactRefV1,
    verified_envelope_bytes: VerifiedSignedEnvelopeBytesV1,
}

impl<T> VerifiedBusinessArtifactV1<T> {
    pub fn payload(&self) -> &T { &self.signed.payload }
    pub fn payload_sha256(&self) -> Sha256Digest { self.signed.payload_sha256 }
    pub fn artifact_ref(&self) -> &ArtifactRefV1 { &self.artifact_ref }
    pub fn exact_envelope_jcs_bytes(&self) -> &[u8] {
        self.verified_envelope_bytes.exact_envelope_jcs_bytes()
    }
    pub fn verified_envelope_bytes(&self) -> &VerifiedSignedEnvelopeBytesV1 {
        &self.verified_envelope_bytes
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum F57SignerValidityRuleV1 {
    CurrentAtVerification,
    ValidAtTrustedSigningTime,
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct F57ArtifactSignerRegistryRowV1 {
    pub artifact_kind: String,
    pub discriminator: String,
    pub producer_role: String,
    pub runner_id: Option<RunnerIdV1>,
    pub signer_subject: SignerSpkiTokenV1,
    pub certificate_subject_dn: String,
    pub validity_rule: F57SignerValidityRuleV1,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ClientStackDecisionArchiveCertificateChainRoleV1 {
    DecisionSigner,
    Tsa,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum ClientStackDecisionArchiveTrustAnchorPolicyPurposeV1 {
    #[serde(rename = "EP-F57-CLIENT-STACK-DECISION-ARCHIVE-TRUST-ANCHOR-POLICY-V1")]
    ClientStackDecisionArchiveTrustAnchorPolicy,
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct ClientStackDecisionArchiveTrustAnchorRoleV1 {
    pub chain_role: ClientStackDecisionArchiveCertificateChainRoleV1,
    pub root_certificate_sha256s: Vec<Sha256Digest>,
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct ClientStackDecisionArchiveTrustAnchorPolicyV1 {
    pub schema_version: u32,
    pub purpose: ClientStackDecisionArchiveTrustAnchorPolicyPurposeV1,
    pub roles: Vec<ClientStackDecisionArchiveTrustAnchorRoleV1>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum F57ArtifactSignerRegistryPurposeV1 {
    #[serde(rename = "EP-F57-ARTIFACT-SIGNER-REGISTRY-V1")]
    ArtifactSignerRegistry,
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct F57ArtifactSignerRegistryPayloadV1 {
    pub schema_version: u32,
    pub purpose: F57ArtifactSignerRegistryPurposeV1,
    pub rows: Vec<F57ArtifactSignerRegistryRowV1>,
    pub client_stack_decision_archive_trust_anchor_policy:
        ClientStackDecisionArchiveTrustAnchorPolicyV1,
    pub issued_at_unix_ms: i64,
}

pub type F57ArtifactSignerRegistryV1 =
    SignedBusinessArtifactV1<F57ArtifactSignerRegistryPayloadV1>;

#[derive(Clone, Copy, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum GenerationManifestPurposeV1 {
    #[serde(rename = "EP-F57-GENERATION-MANIFEST-V1")]
    GenerationManifest,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum GenerationParticipantPurposeV1 {
    #[serde(rename = "EP-F57-GENERATION-PARTICIPANT-ACK-V1")]
    GenerationParticipantAck,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(try_from = "String", into = "String")]
pub struct GenerationItemIdV1(String); // private; [a-z][a-z0-9._-]{2,159}

#[derive(Clone, Copy, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum GenerationItemKindV1 {
    #[serde(rename = "CAPABILITY_GRAPH")]
    CapabilityGraph,
    #[serde(rename = "PROJECTION_SET")]
    ProjectionSet,
    #[serde(rename = "MIGRATION_PLAN")]
    MigrationPlan,
    #[serde(rename = "CAPABILITY_PACKAGE")]
    CapabilityPackage,
    #[serde(rename = "POLICY_BUNDLE")]
    PolicyBundle,
    #[serde(rename = "UI_SCHEMA")]
    UiSchema,
    #[serde(rename = "PROVIDER_MANIFEST")]
    ProviderManifest,
    #[serde(rename = "RUNTIME_ARTIFACT")]
    RuntimeArtifact,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum GenerationReversePlanPurposeV1 {
    #[serde(rename = "EP-F57-GENERATION-REVERSE-PLAN-V1")]
    GenerationReversePlan,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum GenerationReverseActionV1 {
    RestoreArtifact,
    DeactivateRetainData,
    NoOp,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum GenerationDataRetentionModeV1 {
    #[serde(rename = "RETAIN_ALL_GENERATION_DATA")]
    RetainAllGenerationData,
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct GenerationReversePlanV1 {
    pub schema_version: u32,
    pub purpose: GenerationReversePlanPurposeV1,
    pub plan_id: UuidV1,
    pub item_id: GenerationItemIdV1,
    pub action: GenerationReverseActionV1,
    pub source_artifact_ref: ArtifactRefV1,
    pub target_artifact_ref: Option<ArtifactRefV1>,
    pub data_retention_mode: GenerationDataRetentionModeV1,
    pub issued_at_unix_ms: i64,
}

pub type SignedGenerationReversePlanV1 =
    SignedBusinessArtifactV1<GenerationReversePlanV1>;

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct GenerationItemRefV1 {
    pub item_id: GenerationItemIdV1,
    pub item_kind: GenerationItemKindV1,
    pub artifact_ref: ArtifactRefV1,
    pub reverse_plan_ref: ArtifactRefV1,
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct GenerationParticipantRequirementV1 {
    pub participant_id: String,
    pub participant_definition_sha256: Sha256Digest,
    pub required_item_ids: Vec<GenerationItemIdV1>,
}

// Payload type only. The sole signed wire is SignedBusinessArtifactV1<GenerationManifestV1>.
#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct GenerationManifestV1 {
    pub schema_version: u32,
    pub purpose: GenerationManifestPurposeV1,
    pub deployment_id: UuidV1,
    pub authority_epoch: u64,
    pub generation_number: GenerationNumberV1,
    pub previous_observed_generation_digest_sha256: Option<Sha256Digest>,
    pub capability_graph_ref: ArtifactRefV1,
    pub projection_manifest_ref: ArtifactRefV1,
    pub storage_manifest_ref: ArtifactRefV1,
    pub capacity_policy_definition_ref: ArtifactRefV1,
    pub items: Vec<GenerationItemRefV1>,
    pub required_participants: Vec<GenerationParticipantRequirementV1>,
    pub issued_at_unix_ms: i64,
}

pub type SignedGenerationManifestV1 = SignedBusinessArtifactV1<GenerationManifestV1>;

#[derive(Clone, Copy, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum GenerationApprovalRegistryPurposeV1 {
    #[serde(rename = "EP-F57-GENERATION-APPROVAL-REGISTRY-V1")]
    GenerationApprovalRegistry,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum GenerationApprovalArtifactKindV1 {
    #[serde(rename = "GENERATION_MANIFEST_V1")]
    GenerationManifest,
    #[serde(rename = "GENERATION_REVERSE_PLAN_V1")]
    GenerationReversePlan,
    #[serde(rename = "MIGRATION_PLAN_V1")]
    MigrationPlan,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum GenerationApprovalValidityRuleV1 {
    #[serde(rename = "CURRENT_AT_VERIFICATION")]
    CurrentAtVerification,
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct GenerationApprovalRegistryRowV1 {
    pub artifact_kind: GenerationApprovalArtifactKindV1,
    pub media_type: String,
    pub signer_subject: SignerSpkiTokenV1,
    pub certificate_subject_dn: String,
    pub validity_rule: GenerationApprovalValidityRuleV1,
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct GenerationApprovalRegistryPayloadV1 {
    pub schema_version: u32,
    pub purpose: GenerationApprovalRegistryPurposeV1,
    pub deployment_id: UuidV1,
    pub revision: u64,
    pub rows: Vec<GenerationApprovalRegistryRowV1>,
    pub issued_at_unix_ms: i64,
    pub expires_at_unix_ms: i64,
}

pub type GenerationApprovalRegistryV1 =
    SignedBusinessArtifactV1<GenerationApprovalRegistryPayloadV1>;

// Private bootstrap proof: product-pinned deployment trust plus the exact
// storage-manifest digest pin are the only constructor authority.
pub struct VerifiedGenerationApprovalRegistryV1 {
    verified: VerifiedBusinessArtifactV1<GenerationApprovalRegistryPayloadV1>,
}

impl VerifiedGenerationApprovalRegistryV1 {
    pub fn payload(&self) -> &GenerationApprovalRegistryPayloadV1 {
        self.verified.payload()
    }
    pub fn artifact_ref(&self) -> &ArtifactRefV1 { self.verified.artifact_ref() }
}

// Private non-wire generation-approval proof. GenerationApprovalVerifierV1 is
// its sole constructor after the generic proof is checked by a verifier
// configured with VerifiedGenerationApprovalRegistryV1.
pub struct VerifiedGenerationManifestV1 {
    verified: VerifiedBusinessArtifactV1<GenerationManifestV1>,
    approval_registry_ref: ArtifactRefV1,
}

impl VerifiedGenerationManifestV1 {
    pub fn payload(&self) -> &GenerationManifestV1 { self.verified.payload() }
    pub fn artifact_ref(&self) -> &ArtifactRefV1 { self.verified.artifact_ref() }
    pub fn approval_registry_ref(&self) -> &ArtifactRefV1 { &self.approval_registry_ref }
    pub fn generation_digest_sha256(&self) -> Sha256Digest {
        sha256(self.verified.exact_envelope_jcs_bytes())
    }
}

pub struct VerifiedGenerationReversePlanV1 {
    verified: VerifiedBusinessArtifactV1<GenerationReversePlanV1>,
    approval_registry_ref: ArtifactRefV1,
}

impl VerifiedGenerationReversePlanV1 {
    pub fn payload(&self) -> &GenerationReversePlanV1 { self.verified.payload() }
    pub fn artifact_ref(&self) -> &ArtifactRefV1 { self.verified.artifact_ref() }
    pub fn approval_registry_ref(&self) -> &ArtifactRefV1 { &self.approval_registry_ref }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum GenerationApprovalVerificationErrorV1 {
    RegistryDeploymentMismatch,
    RegistryExpired,
    RegistryRowMissingOrDuplicate,
    ArtifactKindMismatch,
    ArtifactRefMismatch,
    MediaMismatch,
    SignerSubjectMismatch,
    CertificateSubjectMismatch,
    ValidityRuleMismatch,
    PayloadInvariantViolation,
}

pub struct GenerationApprovalVerifierV1<'a> {
    registry: &'a VerifiedGenerationApprovalRegistryV1,
}

impl<'a> GenerationApprovalVerifierV1<'a> {
    pub fn new(registry: &'a VerifiedGenerationApprovalRegistryV1) -> Self;

    pub fn verify_manifest(
        &self,
        generic: VerifiedBusinessArtifactV1<GenerationManifestV1>,
    ) -> Result<VerifiedGenerationManifestV1, GenerationApprovalVerificationErrorV1>;

    pub fn verify_reverse_plan(
        &self,
        generic: VerifiedBusinessArtifactV1<GenerationReversePlanV1>,
    ) -> Result<VerifiedGenerationReversePlanV1, GenerationApprovalVerificationErrorV1>;
}

// Plain authenticated-internal ACK persisted only by G1-05 after Service-SID IPC/readback.
#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct GenerationParticipantV1 {
    pub schema_version: u32,
    pub purpose: GenerationParticipantPurposeV1,
    pub deployment_id: UuidV1,
    pub authority_epoch: u64,
    pub generation_number: GenerationNumberV1,
    pub activation_attempt_id: UuidV1,
    pub generation_manifest_ref: ArtifactRefV1,
    pub generation_digest_sha256: Sha256Digest,
    pub topology_declaration_ref: ArtifactRefV1,
    pub participant_id: String,
    pub participant_definition_sha256: Sha256Digest,
    pub applied_item_set_sha256: Sha256Digest,
    pub participant_apply_readback_ref: ArtifactRefV1,
    pub acknowledged_at_unix_ms: i64,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum GenerationStateV1 {
    Draft, Compiled, Simulated, Approved, Signed, Predownloaded,
    Activating, Observed, RollingBack, RolledBack, Rejected,
}

pub fn transition_allowed(from: GenerationStateV1, to: GenerationStateV1) -> bool {
    use GenerationStateV1::*;
    matches!((from, to),
        (Draft, Compiled | Rejected)
        | (Compiled, Simulated | Rejected)
        | (Simulated, Approved | Rejected)
        | (Approved, Signed | Rejected)
        | (Signed, Predownloaded | Rejected)
        | (Predownloaded, Activating | Rejected)
        | (Activating, Observed | RollingBack)
        | (Observed, RollingBack)
        | (RollingBack, RolledBack))
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum RuntimeCarrierV1 {
    InProcess, WindowsService, JobObjectWorker, WasmSandbox, HyperVContainer,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum PersistenceClassV1 {
    None, EphemeralCache, AuthorityPostgres, AppendOnlyBackup, ArchiveOnly,
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct RuntimeParticipantV1 {
    pub participant_id: String,
    pub service_identity: String,
    pub binary_sha256: Sha256Digest,
    pub carrier: RuntimeCarrierV1,
    pub windows_service_sid: Option<String>,
    pub ipc_endpoint: String,
    pub ipc_dacl_sha256: Sha256Digest,
    pub dependency_participant_ids: Vec<String>,
    pub readiness_probe_ids: Vec<String>,
    pub resource_class: String,
    pub queue_class: String,
    pub allowed_persistence: PersistenceClassV1,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum DatabaseConnectionPrivilegeClassV1 {
    Normal,
    Reserved,
    Superuser,
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct DatabaseConsumerV1 {
    pub consumer_id: String,
    pub service_identity: String,
    pub database_role: String,
    pub purpose: String,
    pub connection_privilege_class: DatabaseConnectionPrivilegeClassV1,
    pub steady_pool_max: u32,
    pub peak_pool_max: u32,
    pub acquire_timeout_ms: u64,
    pub statement_timeout_ms: u64,
    pub capacity_budget_weight: u32,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(try_from = "String", into = "String")]
pub struct RuntimeHardwareProfileIdV1(String); // private; exact generated profile member
#[derive(Clone, Debug, Eq, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(try_from = "String", into = "String")]
pub struct RuntimeStorageProfileIdV1(String); // private; exact generated profile member
#[derive(Clone, Debug, Eq, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(try_from = "String", into = "String")]
pub struct RuntimeWorkloadProfileIdV1(String); // private; exact generated profile member

#[derive(Clone, Copy, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum RuntimeTopologyDeclarationPurposeV1 {
    #[serde(rename = "EP-F57-RUNTIME-TOPOLOGY-DECLARATION-V1")]
    RuntimeTopologyDeclaration,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum RuntimeTopologyCertificationPurposeV1 {
    #[serde(rename = "EP-F57-RUNTIME-TOPOLOGY-CERTIFICATION-V1")]
    RuntimeTopologyCertification,
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct RuntimeTopologyDeclarationV1 {
    pub purpose: RuntimeTopologyDeclarationPurposeV1,
    pub schema_version: u32,
    pub deployment_id: UuidV1,
    pub authority_epoch: u64,
    pub generation_digest_sha256: Sha256Digest,
    pub capability_graph_digest_sha256: Sha256Digest,
    pub hardware_profile_id: RuntimeHardwareProfileIdV1,
    pub storage_profile_id: RuntimeStorageProfileIdV1,
    pub workload_profile_id: RuntimeWorkloadProfileIdV1,
    pub storage_manifest_ref: ArtifactRefV1,
    pub capacity_policy_definition_ref: ArtifactRefV1,
    pub participants: Vec<RuntimeParticipantV1>,
    pub database_consumers: Vec<DatabaseConsumerV1>,
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct RuntimeTopologyCertificationV1 {
    pub schema_version: u32,
    pub purpose: RuntimeTopologyCertificationPurposeV1,
    pub candidate_run: CandidateRunIdentityV1,
    pub candidate_manifest_ref: ArtifactRefV1,
    pub topology_declaration_ref: ArtifactRefV1,
    pub p340_soak_evidence_ref: ArtifactRefV1,
    pub capacity_certificate_ref: ArtifactRefV1,
    pub certified_host_fingerprint_sha256: Sha256Digest,
    pub hardware_profile_id: RuntimeHardwareProfileIdV1,
    pub storage_profile_id: RuntimeStorageProfileIdV1,
    pub workload_profile_id: RuntimeWorkloadProfileIdV1,
    pub certified_concurrent_users: u16,
    pub certified_continuous_duration_seconds: u64,
}

// Private non-wire proof returned only after ref/bytes/schema/live-readback equality.
pub struct VerifiedRuntimeTopologyDeclarationV1 {
    declaration: RuntimeTopologyDeclarationV1,
    artifact_ref: ArtifactRefV1,
}

impl VerifiedRuntimeTopologyDeclarationV1 {
    pub fn declaration(&self) -> &RuntimeTopologyDeclarationV1 { &self.declaration }
    pub fn artifact_ref(&self) -> &ArtifactRefV1 { &self.artifact_ref }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ActualRuntimeReadbackV1 {
    deployment_id: UuidV1,
    authority_epoch: u64,
    generation_digest_sha256: Sha256Digest,
    capability_graph_digest_sha256: Sha256Digest,
    hardware_profile_id: RuntimeHardwareProfileIdV1,
    storage_profile_id: RuntimeStorageProfileIdV1,
    workload_profile_id: RuntimeWorkloadProfileIdV1,
    storage_manifest_ref: ArtifactRefV1,
    capacity_policy_definition_ref: ArtifactRefV1,
    participants: Vec<RuntimeParticipantV1>,
    database_consumers: Vec<DatabaseConsumerV1>,
}

mod runtime_topology_readback_sealed {
    pub trait Sealed {}
}

pub trait RuntimeTopologyReadbackProviderV1:
    runtime_topology_readback_sealed::Sealed + Send + Sync
{
    fn capture_exact(
        &self,
        expected: &RuntimeTopologyDeclarationV1,
    ) -> Result<ActualRuntimeReadbackV1, TopologyErrorV1>;
}

pub struct RuntimeTopologyReadbackCollectorV1 {
    /* private injected OS/service/IPC/storage/database readback ports; runtime is sole constructor */
}

// Pure non-wire values. G1-01 may form the production instance only after it has
// verified storage/DATA_HDD and the exact signed generation; G0 uses fixtures only.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RuntimeTopologyDeclarationBuildFactsV1 {
    pub deployment_id: UuidV1,
    pub authority_epoch: u64,
    pub generation_digest_sha256: Sha256Digest,
    pub storage_manifest_ref: ArtifactRefV1,
    pub capacity_policy_definition_ref: ArtifactRefV1,
}

pub enum TopologyErrorV1 {
    DeclarationBuildInputMismatch,
    DeclarationArtifactRefMismatch,
    WireInvalid,
    ProfileInvalid,
    DatabaseConsumerForbidden,
    ActualDrift,
    CertificationNotDue,
    CertificationBindingMismatch,
}

pub struct TopologyVerifier;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TopologyOfflineDescriptorContractV1 {
    pub schema_path: &'static str,
    pub root_media_bindings: [(&'static str, &'static str); 2],
}

pub const fn topology_offline_descriptor_contract() -> TopologyOfflineDescriptorContractV1 {
    TopologyOfflineDescriptorContractV1 {
        schema_path: "docs/evidence/f57-runtime-topology.v1.schema.json",
        root_media_bindings: [
            ("RuntimeTopologyDeclarationV1", "application/vnd.ep.f57-runtime-topology-declaration-v1+json"),
            ("RuntimeTopologyCertificationV1", "application/vnd.ep.f57-runtime-topology-certification-v1+json"),
        ],
    }
}

impl TopologyVerifier {
    pub fn build_declaration(
        graph: &CompiledCapabilityGraphV1,
        facts: RuntimeTopologyDeclarationBuildFactsV1,
    ) -> Result<RuntimeTopologyDeclarationV1, TopologyErrorV1> {
        if facts.authority_epoch == 0
            || !facts.storage_manifest_ref.has_exact_media(
                "application/vnd.ep.f57-authority-storage-manifest-v1+json")
            || !facts.capacity_policy_definition_ref.has_exact_media(
                "application/vnd.ep.f57-p340-certification-policy-definition-v1+json")
        {
            return Err(TopologyErrorV1::DeclarationBuildInputMismatch);
        }
        let declaration = RuntimeTopologyDeclarationV1 {
            purpose: RuntimeTopologyDeclarationPurposeV1::RuntimeTopologyDeclaration,
            schema_version: 1,
            deployment_id: facts.deployment_id,
            authority_epoch: facts.authority_epoch,
            generation_digest_sha256: facts.generation_digest_sha256,
            capability_graph_digest_sha256: graph.graph_digest_sha256,
            hardware_profile_id: current_runtime_hardware_profile_id(),
            storage_profile_id: current_runtime_storage_profile_id(),
            workload_profile_id: current_runtime_workload_profile_id(),
            storage_manifest_ref: facts.storage_manifest_ref,
            capacity_policy_definition_ref: facts.capacity_policy_definition_ref,
            participants: derive_canonical_runtime_participants(graph)?,
            database_consumers: derive_canonical_database_consumers(graph)?,
        };
        validate_declaration_policy_and_graph_derivation(&declaration, graph)?;
        Ok(declaration)
    }

    pub fn build_certification(
        candidate_run: CandidateRunIdentityV1,
        candidate_manifest_ref: ArtifactRefV1,
        topology_declaration_ref: ArtifactRefV1,
        p340_soak_evidence_ref: ArtifactRefV1,
        capacity_certificate_ref: ArtifactRefV1,
        certified_host_fingerprint_sha256: Sha256Digest,
        hardware_profile_id: RuntimeHardwareProfileIdV1,
        storage_profile_id: RuntimeStorageProfileIdV1,
        workload_profile_id: RuntimeWorkloadProfileIdV1,
        certified_concurrent_users: u16,
        certified_continuous_duration_seconds: u64,
    ) -> Result<RuntimeTopologyCertificationV1, TopologyErrorV1> {
        require_exact_topology_profiles(
            &hardware_profile_id,
            &storage_profile_id,
            &workload_profile_id,
        )?;
        if certified_concurrent_users != 20
            || certified_continuous_duration_seconds != 259_200
        {
            return Err(TopologyErrorV1::CertificationBindingMismatch);
        }
        Ok(RuntimeTopologyCertificationV1 {
            schema_version: 1,
            purpose: RuntimeTopologyCertificationPurposeV1::RuntimeTopologyCertification,
            candidate_run,
            candidate_manifest_ref,
            topology_declaration_ref,
            p340_soak_evidence_ref,
            capacity_certificate_ref,
            certified_host_fingerprint_sha256,
            hardware_profile_id,
            storage_profile_id,
            workload_profile_id,
            certified_concurrent_users,
            certified_continuous_duration_seconds,
        })
    }

    pub fn verify_declaration(
        artifact_ref: &ArtifactRefV1,
        exact_jcs_bytes: &[u8],
        readback: &impl RuntimeTopologyReadbackProviderV1,
    ) -> Result<VerifiedRuntimeTopologyDeclarationV1, TopologyErrorV1> {
        require_exact_media_size_digest_and_object_store_uri(
            artifact_ref,
            exact_jcs_bytes,
            "application/vnd.ep.f57-runtime-topology-declaration-v1+json",
        )?;
        let declaration: RuntimeTopologyDeclarationV1 = strict_from_jcs(exact_jcs_bytes)?;
        if canonical_jcs(&declaration) != exact_jcs_bytes { return Err(TopologyErrorV1::WireInvalid); }
        let actual = readback.capture_exact(&declaration)?;
        verify_declaration_equals_actual_and_policy(&declaration, &actual)?;
        Ok(VerifiedRuntimeTopologyDeclarationV1 {
            declaration,
            artifact_ref: artifact_ref.clone(),
        })
    }
}
```

`docs/evidence/f57-generation.v1.schema.json` and `crates/platform/release/src/generation.rs` are the sole schema/Rust nominal owner pair for the generation family; `participant.rs`, `activation_attempt.rs`, `ep-platform-generation-activation::{coordinator,verified_graph}` and their stores import those nominals and own behavior only. No retired `crates/platform/release/src/activation.rs` exists. The schema has exactly one direct import, same-directory `f57-foundation.v1.schema.json`; it imports no topology, CapabilityGraph, storage, G1, P340, release-evidence or later-stage schema and foundation has no reverse edge. It composes the foundation detached-CMS envelope exactly once for each of the signed manifest and signed reverse-plan roots and owns one separate strict plain ACK root. Exact purpose/media pairs are `EP-F57-GENERATION-MANIFEST-V1` / `application/vnd.ep.f57-generation-manifest-v1+json`, `EP-F57-GENERATION-REVERSE-PLAN-V1` / `application/vnd.ep.f57-generation-reverse-plan-v1+json`, and `EP-F57-GENERATION-PARTICIPANT-ACK-V1` / `application/vnd.ep.f57-generation-participant-ack-v1+json`; the three strict roots have exactly `13/9/14` fields. `GenerationManifestV1` and `GenerationReversePlanV1` are payloads, never signed-envelope aliases or nested wrappers; `GenerationParticipantV1` is plain JCS, never signed or wrapped.

`docs/schemas/f57-generation-approval-registry.v1.schema.json` and `crates/platform/release/src/generation_approval.rs` are the sole schema/Rust owners of the approval registry. The signed-root schema directly imports `../evidence/f57-foundation.v1.schema.json`, composes its detached-CMS envelope exactly once, narrows only `payload`, closes the composed draft-2020-12 root with `unevaluatedProperties=false`, and imports no generation/topology/storage/G1/P340/release schema. Its signed payload has exactly seven fields `{schema_version,purpose,deployment_id,revision,rows,issued_at_unix_ms,expires_at_unix_ms}`; purpose/media are `EP-F57-GENERATION-APPROVAL-REGISTRY-V1` / `application/vnd.ep.f57-generation-approval-registry-v1+json`. It has exactly three canonical unique five-field rows `{artifact_kind,media_type,signer_subject,certificate_subject_dn,validity_rule}` for `GENERATION_MANIFEST_V1`, `GENERATION_REVERSE_PLAN_V1` and `MIGRATION_PLAN_V1`; every row has its exact media, strict SPKI token, nonempty exact DN and `CURRENT_AT_VERIFICATION`, with no wildcard/default/other row.

Registry rows sort in that artifact-kind order. Their media are respectively `application/vnd.ep.f57-generation-manifest-v1+json`, `application/vnd.ep.f57-generation-reverse-plan-v1+json`, and `application/vnd.ep.f57-migration-plan-v1+json`; certificate DNs are respectively `CN=EP Generation Manifest Authority,O=Enterprise Platform`, `CN=EP Generation Reverse Plan Authority,O=Enterprise Platform`, and `CN=EP Migration Plan Authority,O=Enterprise Platform`. The registry envelope itself requires `CN=EP Generation Approval Registry Authority,O=Enterprise Platform`. Every `signer_subject` is the exact `spki-sha256:<64-lowerhex>` token recomputed from its provisioned nonexportable leaf; DN-as-subject, placeholder token, same-DN/different-key or same-key/different-DN substitution fails.

The registry envelope is bootstrap-verified only by the product-pinned deployment trust bundle plus exact registry-authority SPKI/DN. The verified storage manifest's canonical `policy_ids` contains exactly one `generation-approval-registry-sha256:<64-lowerhex>` member pinning the exact registry envelope at the fixed derived DATA_HDD path `generations/trust/generation-approval-registry.v1.json`. Deployment ID must match, revision is positive, `issued_at_unix_ms < expires_at_unix_ms`, and current chain/revocation plus the pinned digest are checked before the private `VerifiedGenerationApprovalRegistryV1` constructor is reachable. Rotation requires a newer signed storage manifest that pins new registry bytes. An adjacent path, valid-old registry, ambient Windows trust, self-root, missing/duplicate/malformed policy pin or copied 89-row evidence registry cannot authorize generation.

Production trust provisioning is deliberately outside G0. G1-01 solely owns `apps/recovery-tool/src/generation_trust.rs` and the private `crates/platform/runtime/src/secrets/generation_trust.rs::GenerationTrustProviderV1`, with exactly four disjoint roles: approval-registry, generation-manifest, generation-reverse-plan and migration-plan authority. Its only adapters are self-hosted Windows CNG/PIV or an approved existing enterprise signer under the identical registry/SPKI/DN/CMS contract. Neither may export a private key, invent a root, select a caller signer or send more than the approved digest to a remote signer. The local nonexportable container IDs are exactly `EP-F57-GENERATION-APPROVAL-REGISTRY-V1`, `EP-F57-GENERATION-MANIFEST-V1`, `EP-F57-GENERATION-REVERSE-PLAN-V1`, and `EP-F57-MIGRATION-PLAN-V1`; online core receives only manifest/reverse-plan role handles. Registry ceremony emits the digest for the independently signed storage policy pin and installs a mutually verified pair through a maintenance-locked fsynced rotation journal and immutable revision paths; crash recovery resumes that pair, never scans/selects latest or mutates active bytes in place. G0 owns only the types/validators/descriptor goldens and an architecture test proving zero production key/provision/install/rotation call site.

Only `GenerationApprovalVerifierV1` configured by that private registry proof may turn a generic verified manifest into private `VerifiedGenerationManifestV1`; every declaration, activation, digest and later release consumer accepts that domain-specific wrapper, never a generic proof, raw payload, digest or double-wrapped envelope. `schema_version=1`; deployment is nonnil, authority epoch and generation number are positive. `GenerationItemIdV1` accepts only 3–160 byte lowercase `[a-z][a-z0-9._-]{2,159}`. Items are canonical unique by `item_id`, use exactly the eight displayed kinds and typed-verify every forward artifact in its owning runtime trust domain. Each `reverse_plan_ref` typed-loads exact signed `GenerationReversePlanV1` under the approval registry's reverse-plan row and exact-matches its `item_id` and `source_artifact_ref` to the enclosing item. Reverse actions are exactly `RESTORE_ARTIFACT|DEACTIVATE_RETAIN_DATA|NO_OP` and retention is always `RETAIN_ALL_GENERATION_DATA`: RESTORE requires a distinct nonnull target, DEACTIVATE requires null, and NO_OP requires target byte-equal source and a compiled rollback policy that explicitly allows NO_OP for any unsafe item kind. The item set contains exactly one graph and one projection row whose refs byte-equal the manifest top-level refs. Required participants and each item subset are canonical unique complete compiled-graph derivations; `participant_definition_sha256=SHA256(JCS(the exact derived RuntimeParticipantV1 row))`. Caller-supplied sets/digests, unknown kind, wrong/media-unverified reverse ref, absent reverse plan, item/source mismatch or action/target/retention mismatch fails before manifest signing.

Generation numbering is server-only and positive. Generation `1` requires `previous_observed_generation_digest_sha256=null`; each later manifest repeats the exact complete signed-envelope digest of the immediately prior durable `OBSERVED` generation. A skip, fork, older/non-durable/desired-only predecessor, payload digest or caller digest fails before signing. The sole identity is `generation_digest_sha256=SHA256(the exact complete canonical-JCS bytes of SignedBusinessArtifactV1<GenerationManifestV1>)`, byte-equal to `generation_manifest_ref.sha256` and never the payload hash, graph hash, number, reserialized envelope or caller digest. `VerifiedGenerationManifestV1` retains the exact verified envelope bytes and approval-registry ref and exposes only read-only payload/ref/registry/digest accessors. Signing and verification use only the pinned generation approval registry, never the 89-row F57 candidate-evidence registry. G0 owns schema, strict parsers/validators, bootstrap and approval-verifier boundaries, the digest helper and non-production byte goldens, but has no production trust file, key, registry/generation/reverse-plan store, signing command or ACK write path.

Only after G1-01 has obtained private `ValidatedDataRootV1`, verified the signed storage manifest/DATA_HDD and its exact approval-registry pin, and compiled graph/projection/policy may its private authority create the first production generation. It verifies the pinned registry, typed-verifies item/reverse-plan refs and registry revision, create-new signs/stores/reloads generation `1`, obtains `VerifiedGenerationManifestV1` only through `GenerationApprovalVerifierV1`, computes the exact envelope digest, and only then constructs/stores the topology declaration. Later generation creation serializes against the immediately prior durable OBSERVED envelope. No generation manifest contains a topology declaration or certification ref, so the order has no cycle. G1-05 accepts the private manifest wrapper plus verified declaration/live topology before activation. For each required participant under one unpredictable durable `activation_attempt_id`, it authenticates Service-SID IPC and fresh complete actual readback, then alone constructs/persists one strict plain 14-field `GenerationParticipantV1`. The ACK exact-repeats deployment/epoch/number, generation ref/digest, declaration ref, participant ID/definition digest and `applied_item_set_sha256=SHA256(JCS(the canonical exact GenerationItemRefV1 subset named by its requirement row))`; `participant_apply_readback_ref` exact-loads the same-attempt successful apply readback, and the fourteenth/final field `acknowledged_at_unix_ms` is sampled after that durable readback and no later than the OBSERVED commit. It has no CMS field or envelope. OBSERVED moves in the same serialized transition only for the canonical exact required-participant set, one row per participant, all from that attempt and exact generation/declaration/digests. Missing, extra, duplicate, stale/mixed attempt, time/ref/item/readback drift, client field, signed ACK, failure-as-ACK or restart-minted ACK fails and retains/rolls back the prior observed generation.

The production reverse-plan path is likewise G1-01-only. Private `GenerationReversePlanAuthorityV1` derives one action per canonical item from the verified prior durable OBSERVED item set and compiled rollback policy; no caller supplies an action. An existing item restores its distinct prior artifact, a new safely deactivatable item uses `DEACTIVATE_RETAIN_DATA`, and `NO_OP` requires explicit compiled-policy proof or creation fails. Before any signature, DATA_HDD/final-handle-bound create-new `GenerationCreationAttemptStoreV1` freezes complete verified inputs, one unpredictable attempt ID, one plan ID per item, all plan/manifest issued times, the relocatable approval/storage/graph/projection/policy refs and deterministic private spool paths. That record coordinates recovery but is re-derived against verified inputs and never authorizes by itself. Each reverse plan is signed once to its spool, fsynced/reloaded, ingested through the authority-lane `EvidenceObjectStoreV1` and domain-verified from only `evidence-relative://bundle/objects/sha256/<whole-envelope-sha256>` before the manifest may reference it; the manifest follows the same spool → object → typed-reload protocol before the declaration object is built. Spool paths never enter a signed wire. The fixed DATA_HDD registry/storage sources are first verified then copied to authority-bundle `inputs/<whole-envelope-sha256>.json`; `VerifiedGenerationManifestV1` retains that relocatable registry ref, not the fixed source locator. Recovery adopts exact existing spool/object bytes, signs only a frozen absent stage and never scans, regenerates ID/time/action, re-signs, overwrites or selects another registry. G0's contract/architecture tests freeze this producer boundary, the exact object/input URI forms and every crash-cut invariant without implementing the producer. G6 final freeze must copy the complete generation/reverse-plan/ACK/declaration/dependency graph to identical relative locations in its own explicit bundle, typed-reload it there and pass an offline golden after the authority-generation root is unavailable.

`SignedBusinessArtifactV1<T>` is the only signature-bearing outer structure and must reuse the G0-01 foundation/F-56 four-field detached-CMS wire exactly: `payload`, `payload_sha256: Sha256Digest`, `signer_subject`, and `signature_cms_b64url`. Rust owns the generic implementation here, while `docs/evidence/f57-foundation.v1.schema.json` remains the sole schema owner of the envelope field set. `crates/foundation/src/signature.rs` also solely owns private-field, non-serialized `VerifiedBusinessArtifactV1<T>={signed,artifact_ref,exact_envelope_jcs_bytes}`. Only `ArtifactVerifier` may construct it after exact typed media/purpose, size, content-addressed URI, strict canonical JCS, payload digest, CMS chain/revocation/time and signer authorization verification; public access is read-only through `payload()`, `payload_sha256()`, `artifact_ref()` and `exact_envelope_jcs_bytes()`. Callers cannot fabricate it from a payload, digest, ref or reserialized envelope. Each signed-root schema imports the foundation root, composes the envelope once, refines only `payload` to its one local strict payload, and closes the composed root with `unevaluatedProperties=false`. `signer_subject` accepts only `spki-sha256:<64-lowerhex>` recomputed from the verified signer leaf's exact DER SPKI; a DN, SKI, serial, key name, upperhex or caller string is rejected. A human-readable leaf DN is derived only after verification and can satisfy a separate registry `certificate_subject_dn` constraint, never identity or authorization. Purpose is a required typed field inside `T` and is verified by the typed expectation; G0 must not introduce an outer purpose, raw key-id, DN-as-subject, or raw-signature variant. Graph, migration, package, provider, and generation payloads may not repeat signature, algorithm, key, or certificate fields.

The signing path is one-way: payload and digest exist before CMS generation, and no caller can construct `VerifiedCmsIssuanceWindowV1`. The generated exhaustive payload-kind descriptor derives the sole window and validity rule; unknown or overlapping kinds fail generation and a time-bearing payload can never select `static-none`. The current-approved intrinsic-`issued_at_unix_ms` category contains exactly `SignedF57AuthorityStorageManifestV1|SignedGenerationManifestV1|SignedGenerationReversePlanV1`. Each derives the checked inclusive window `[issued_at_unix_ms,issued_at_unix_ms+300000]` under checked `i128` and requires `CURRENT_AT_VERIFICATION`; both endpoints pass, one millisecond outside fails, and no CMS/TSA time is written back into the already hashed payload. The signer proves actual trusted CMS/RFC-3161 time lies in that window and the verifier repeats the same derivation. `F57ArtifactSignerRegistryV1` is the sole signed F57 `static-none` exception; no generation or storage payload may borrow it. G0's descriptor golden freezes all three exact kind names and generation manifest/reverse-plan behavior; G1-01's storage test instantiates the already reserved storage-manifest row without adding a fourth current-approved kind.

`crates/platform/runtime/src/topology.rs` and `docs/evidence/f57-runtime-topology.v1.schema.json` are the one Rust/schema owner pair for the replacement two-stage topology family; the former single-manifest nominal is forbidden and has no definition or use. The schema has exactly one import, same-directory `f57-foundation.v1.schema.json`, for UUID/digest/candidate-run/artifact refs. It owns the two purpose enums, two roots, three private profile IDs and their nested participant/database-carrier/persistence vocabulary; it imports no CapabilityGraph, release, P340 or client leaf schema and copies no foundation nominal. Both roots are strict plain JCS values, not `SignedBusinessArtifactV1`, and therefore add no signer-registry row. Their exact media are `application/vnd.ep.f57-runtime-topology-declaration-v1+json` and `application/vnd.ep.f57-runtime-topology-certification-v1+json`. The future offline schema closure must contain this one descriptor with both bindings; a second schema path/owner, signed wrapper, missing media, transitive-only foundation edge or reverse/later-stage import fails G0.

Task 5 adds exactly one Rust workspace dependency `ep-platform-runtime -> ep-platform-capability-graph` through `ep-platform-capability-graph.workspace = true` in `crates/platform/runtime/Cargo.toml`, then updates `Cargo.lock` in the same staged change. This source dependency is required only so the pure declaration builder can consume the complete `CompiledCapabilityGraphV1`; it does not add a JSON-Schema import. `ep-platform-capability-graph` has no dependency on runtime, release, storage, P340 or any later-stage crate, and the workspace dependency DAG/locked metadata golden rejects the reverse edge, a cycle, a path/git alias, a second graph crate, or lock drift.

G0 implements the pure deterministic `TopologyVerifier::build_declaration(graph:&CompiledCapabilityGraphV1,facts:RuntimeTopologyDeclarationBuildFactsV1) -> Result<RuntimeTopologyDeclarationV1,TopologyErrorV1>` contract and fixture-tests it without creating a deployment artifact. `RuntimeTopologyDeclarationBuildFactsV1` is a plain non-wire five-field carrier `{deployment_id,authority_epoch,generation_digest_sha256,storage_manifest_ref,capacity_policy_definition_ref}`; it is neither a verified wrapper nor activation authority. The builder depends only on G0/foundation/runtime-topology types: it takes the complete compiled graph—not a caller-supplied graph digest or ambient participant list—copies the graph digest, deterministically derives and canonical-sorts the full participant/dependency/probe and database-consumer sets, requires positive authority epoch plus exact storage-manifest/policy media, and forbids Integration Gateway and every other zero-credential participant from the database-consumer set. The three private profile IDs come only from their generated registries; the frozen current wires are respectively `THINKSTATION_P340_I5_10500_32GB_256GB_SSD_1TB_HDD`, `SINGLE_DISK_DEGRADED_PRODUCTION`, and `F57_P340_15_3_2_1__11_5_2_2_V1`. The declaration contains no candidate, P340 soak, capacity-certificate, certified-host or certification field, and current runtime readback is not a builder input. The G0 command/gate layer has no call or store path for this builder and its receipt contains no declaration ref. Only after `verify_authority_data_root_v1` returns the G1-owned private `ValidatedDataRootV1` does G1-01 become the sole first production caller: it forms the five facts from that verified manifest/DATA_HDD result, the exact signed generation selected for G1-05 and the fixed policy-definition ref, supplies the complete compiled graph, then create-new stores/reloads the canonical bytes in `EvidenceObjectStoreV1`. G1-05 separately calls `verify_declaration` with live readback and exact-matches graph/generation/storage/policy before activation. Architecture tests reject any G0 or non-G1-01 production call. Pre-release readiness may compare live state to this declaration, but declaration-only production activation is forbidden.

The only authority-facing read contract is `TopologyVerifier::verify_declaration(artifact_ref:&ArtifactRefV1, exact_jcs_bytes:&[u8], readback:&impl RuntimeTopologyReadbackProviderV1) -> Result<VerifiedRuntimeTopologyDeclarationV1,TopologyErrorV1>`. It first requires the declaration media, exact byte size and SHA-256 plus the `EvidenceObjectStoreV1` content-addressed URI, strict-loads the one declaration root, rejects any noncanonical JCS/unknown field/purpose/profile, then invokes the runtime-sealed collector inside the caller-held transition lock and byte-compares the complete fresh readback closure. `ActualRuntimeReadbackV1` fields are private; only `RuntimeTopologyReadbackCollectorV1` and runtime-owned test fixtures can construct it. Only that path constructs the private non-serializable wrapper, which exposes read-only declaration/ref accessors and no raw or unchecked constructor. A digest-only value, caller-built wrapper, generic media, another URI, valid JSON with noncanonical bytes, declaration without live equality, or certification bytes at this entry point fails closed. Authority/G1 plans must consume this exact wrapper name and API, never the retired single-manifest wrapper.

The same G0 owner provides only a pure `TopologyVerifier::build_certification` over foundation/runtime-topology scalars and refs plus strict certification parsing/field/profile invariants; its signature imports no P340, release, gate or later-stage verified type, and neither the G0 command layer nor G0–G5 has a production authorization/store path for it. Expansion Task 14 solely owns the private release-layer certification authority and is the only production caller: after the candidate-bound P340 carrier is terminal PASS and before POWER/Final L2, it typed-verifies the candidate, declaration, same soak and reachable capacity certificate, exact-matches all three profile IDs, certified host fingerprint, exactly `20` users and `259200` seconds, storage-manifest/DATA_HDD identity, P340 policy definition, candidate graph digest and active generation, and only then passes their plain values to the pure builder. Certification is stored create-new by digest in `EvidenceObjectStoreV1` and reached only through the release certificate; declaration is reached only through the candidate. The order is exactly `declaration -> candidate -> terminal P340 -> certification -> release certificate -> production activation`; a G0–G5 authorization/store attempt, pre-P340 production call, another terminal recipe, ref/profile/storage/policy/host/capacity drift, a topology signer row, or directory discovery fails closed.

`docs/f57-artifact-signer-registry.v1.json` is committed as the exact signed `F57ArtifactSignerRegistryV1`, not as an unsigned row seed or a generated runtime cache. Its strict schema has the one exact import `../evidence/f57-foundation.v1.schema.json`, composes the foundation-owned four-field signed envelope exactly once, and owns only the exact five-field local payload `{schema_version,purpose,rows,client_stack_decision_archive_trust_anchor_policy,issued_at_unix_ms}`, expanded rows, archive-policy family and registry refinements; it never redefines UUID/digest/SPKI/runner/ref/envelope shapes. The payload contains the master's fully expanded 89 scalar rows, sorted uniquely by `(artifact_kind,discriminator)`, with no `×`, `NONE`, wildcard, default, empty signer token/DN/role, or unknown runner. Each row contains `signer_subject=spki-sha256:<64-lowerhex>` recomputed from the exact provisioned role leaf and a separate exact `certificate_subject_dn`; no placeholder digest is legal. Exactly `CLIENT_STACK_DECISION_V1/""` uses `VALID_AT_TRUSTED_SIGNING_TIME`; the other 88 rows use `CURRENT_AT_VERIFICATION`. The embedded static `ClientStackDecisionArchiveTrustAnchorPolicyV1` has exactly `{schema_version=1,purpose=EP-F57-CLIENT-STACK-DECISION-ARCHIVE-TRUST-ANCHOR-POLICY-V1,roles}` and exactly two canonical rows `DECISION_SIGNER|TSA`, each with a nonempty sorted unique exact set of complete-DER root-certificate SHA-256 digests. It adds no registry-bootstrap anchor: G5 may extract only this already authenticated policy after the registry independently verifies under deployment trust, and neither the archive nor any payload lacking that mandatory field may omit/edit/self-authorize it. The four client and six carrier families exact-import their normative runner/DN constraints and resolve tokens from their provisioned leaves. `SIGNED_ARTIFACT_REF_V1/windows-authority` is deliberately absent: final release uses the internally tagged wire `{"artifact_kind":"WINDOWS_AUTHORITY","authority_artifact_set_ref":<ArtifactRefV1>}`, i.e. `ReleaseArtifactRefV1::WindowsAuthority { authority_artifact_set_ref }`, to point directly to already signed `WindowsAuthorityArtifactSetV1` bytes, never a second signed wrapper. The final total remains 89; plain `GateReceiptRefV1` is likewise not a signed artifact and adds no row.

Registry verification is two-stage and fail-closed. For the registry envelope only, `ArtifactVerifier` first uses the deployment-pinned corporate trust root, requires its envelope token to byte-equal verified `DeploymentManifestV1.manifest_signer_subject`, recomputes that token from the leaf, and additionally requires leaf DN `CN=EP F57 Artifact Signer Registry Authority,O=Enterprise Platform`; the registry cannot contribute its own root and the DN cannot authorize another key. Only the returned verified payload may authorize later signatures by exact `(artifact_kind,discriminator)` lookup. Each legal payload owner returns a generated stable `CmsArtifactTypeIdV1`; foundation resolves its private descriptor, and `prepare_cms_signing_request_v1` recomputes payload/descriptor/window/context/handle bindings before alone constructing the private request. The stable type ID and descriptor digest never depend on Rust `type_name`, module path or compiler. `sign_business_artifact_v1` consumes only that request and never caller-selected SPKI, DN, runner, window, operation ID or validity rule. The generated impl registry/architecture check rejects handwritten or reused type IDs, and the real release/evidence-trust/KMS adapter compile tests close the cross-crate path. Unsigned/source-schema drift, DN in the wire subject, another key with the same DN, the same key under a wrong DN, unexpanded/missing/duplicate row, dynamic JSON or runtime-domain artifact substitution fails before any F57 evidence signature is interpreted. `CandidateIdentityV1.artifact_signer_registry_sha256` is the SHA-256 of these exact signed source-envelope bytes.

The registry's trust material is implemented in this same task, before that source file can be completed. `crates/platform/evidence-trust` is the sole owner of `F57EvidenceCredentialRequirementV1`, the private `VerifiedEvidenceSignerHandleV1`, `F57EvidenceTrustProviderV1`, the fixed broker request/response protocol and `F57EvidenceTrustCoordinatorV1`. It expands the exact 89-row descriptor into a complete canonical mapping keyed by `(producer_role,runner_id,certificate_subject_dn)`, preserves every source `(artifact_kind,discriminator)` edge, and adds only the registry-bootstrap authority and archive TSA. The committed credential-requirements golden is generated from code and exact-compared; an implementation never hand-counts roles or reads signer tokens from the signed registry it is trying to create.

`crates/adapter/kms/src/f57_evidence_trust.rs` implements exactly `SELF_HOSTED_OS_KEYSTORE_PIV_V1|EXISTING_ENTERPRISE_SIGNER_V1`. The recommended self-hosted branch uses deterministic nonexportable key containers in Windows CNG/KSP, macOS Keychain/PIV or Linux PKCS#11/PIV and certificates chained to the offline product-pinned corporate issuer. The existing-enterprise branch imports exact certificate/chain/SPKI/DN identities and pins one mTLS signer plus one RFC-3161 TSA endpoint; it sends only the approved digest and closed context. Both verify EKU, key usage, private-key presence/nonexportability, OS ACL, authenticated runner identity, current chain/revocation and archive-root policy before returning an opaque row-bound handle. A provider cannot invent a root, choose a caller DN/runner/row, export a key or sign arbitrary bytes.

`apps/evidence-trust-tool` is the sole maintenance caller. One protected external state root, exclusive lease and fsynced length-framed journal freeze the ceremony ID, mode, deployment bootstrap, complete requirement digest, prior-registry digest for rotation, role locators/readbacks, issue time and output digest. Exact commands are `prepare`, `verify`, `seal-registry`, `install-broker`, and `rotate`; initial and rotation branches are disjoint, and every retry resumes the same ceremony. `seal-registry` fills all 89 SPKI tokens from verified handles, derives the two archive root sets, signs only through the product-pinned registry-bootstrap handle, writes the exact create-new repository path and typed-reloads it. `install-broker` atomically projects a sealed immutable session to the fixed local service/daemon configuration. A crash, half install, newest-file selection, changed time/role/ref, second output, old source overwrite or missing prerequisite never produces a usable active session.

`apps/evidence-signing-broker` is the only ordinary access to candidate-evidence private keys. Its endpoints are exactly `\\.\pipe\EnterprisePlatform\F57EvidenceSignerV1` on Windows and `/var/run/enterprise-platform/f57-evidence-signer-v1.sock` on macOS/Linux. A deterministic authorization key from the owning durable attempt/finalization makes the broker create-new allocate or adopt one CSPRNG `signing_operation_id` before provider access. Foundation hashes the exact tagged issuance window and provider-handle bytes, constructs the nine-field `CmsAuthorizationBindingV1`, and defines `authorization_sha256=SHA256(JCS(binding))`; the digest never includes itself. `INCLUSIVE` requires both endpoints with `not_before<=not_after`, while fieldless `STATIC_NONE` is legal only for the registry descriptor. The broker authenticates the OS caller, loads the sealed session/registry, verifies descriptor/row/runner/DN/SPKI/window/context plus every binding digest, and commits `(operation_id,authorization_digest)` before signing. Both self-hosted and existing-enterprise signer/TSA providers must query/adopt this tuple and return byte-identical CMS after response loss; field drift returns `SigningOperationConflict`, and a provider without durable adoption cannot be installed. It has no arbitrary signing, raw payload, key-path, endpoint, wildcard or fallback method. Self-hosted mode has zero network egress; enterprise mode's only egress is the session-pinned signer/TSA pair. Crash goldens cover before provider commit, after provider commit before broker receipt, after broker commit before caller receipt and after output storage. `xtask` and platform runners use the production-linkable broker client and never import maintenance coordinator or adapter key internals.

On Windows Server 2022 this task also owns the one immutable production service package/static row: service `EPF57EvidenceSignerBroker`, display name `Enterprise Platform F57 Evidence Signer Broker`, account `NT SERVICE\EPF57EvidenceSignerBroker`, executable `C:\Program Files\EnterprisePlatform\EvidenceTrust\f57-evidence-signing-broker.exe`, source entrypoint `apps/evidence-signing-broker/src/main.rs#windows_service_main`, argv `["--service-mode","windows-scm","--endpoint","F57EvidenceSignerV1"]`, `AUTO_START`, `UNRESTRICTED`, `RESTART_ON_FAILURE_MAX_THREE`, dependencies exactly `[CryptSvc,EventLog,KeyIso,RpcSs]`, privilege exactly `[SeChangeNotifyPrivilege]`, and the fixed named pipe above. It implements the converged master plan's exact service-object, executable, pipe and client-group descriptors: clients receive only concrete `0x00120183` pipe data rights, cannot create or replace a pipe instance, and no non-SYSTEM identity receives write/delete/DACL/owner rights over the executable. The signed package, generated static-row JSON and deterministic `EvidenceSignerBroker.wxi` are byte-equivalent; the later Windows installation task may only import those bytes and may not redefine the service.

`F57EvidenceSignerBrokerWindowsInstallReadbackV1` is the sole strict plain Windows readback under purpose/media `EP-F57-EVIDENCE-SIGNER-BROKER-WINDOWS-INSTALL-READBACK-V1` / `application/vnd.ep.f57-evidence-signer-broker-windows-install-readback-v1+json`. It closes the static-row digest; final-handle binary/AuthentiCode identity; parsed canonical ImagePath/argv; account, service and client-group SIDs; SID type, start/recovery/dependencies/privileges; canonical service/executable/pipe/group descriptors and hashes; exact client-group membership; registry/session and authenticated runtime challenge; PID/start key/held image; provider mode/readiness; storage-manifest/final-handle DATA_HDD state root and operation-journal closure; firewall/socket evidence; and `runtime_ssd_mutable_fallback_count=0`. Immutable executable/bootstrap bytes are reproducible Set A, while every mutable session, operation and committed CMS byte persists only below the verified DATA_HDD root plus authenticated off-host mirror. Before DATA_HDD verification the service may be RUNNING only as `WAITING_FOR_DATA_HDD` and returns `NOT_READY`; it never creates a mutable SSD fallback. The self-hosted mode proves an explicit program/service outbound block and zero sockets; enterprise mode permits only the sealed literal mTLS signer/TSA tuples with no DNS, proxy or redirect expansion. Early G0 evidence runners use a separately provisioned instance under this identical wire; Task 11 installs or exact-adopts the candidate-bound production-host instance, preventing a bootstrap dependency cycle.

- [ ] **Step 4: Run all state/topology tests.**

Run: `cargo test -p ep-platform-release --all-targets --locked --test generation --test generation_approval -- --nocapture`

Expected: PASS for the foundation-only three-root generation-schema edge; exact signed-manifest/signed-reverse-plan/plain-ACK media and `13/9/14` fields; exact purpose/item/action/retention wires; two payload-not-alias/single-envelope shapes; strict plain ACK whose `participant_apply_readback_ref` exact-loads the same-attempt successful apply readback and whose `acknowledged_at_unix_ms` follows that readback and no later than OBSERVED commit and no CMS/envelope; the separate foundation-only approval-registry schema with one signed seven-field payload and exact three five-field rows; product-pinned bootstrap plus the one storage-policy digest pin/fixed DATA_HDD path; exact SPKI/DN/media/current-validity rows and adjacent/ambient/old/wildcard/default/self-root negatives; generic-proof → registry-configured `VerifiedGenerationManifestV1` construction and generic/wrong-registry/89-row-registry substitution negatives; complete-envelope digest equality through the domain wrapper; reverse item/source and action/target/retention invariants; exactly one graph item/ref and one projection item/ref; generation-1 null predecessor and later exact immediately prior durable-OBSERVED predecessor; canonical graph-derived participants/item sets; ACK mismatch/time/restart negatives; zero G0 production approval-registry/generation/reverse-plan/ACK signer/store call site; every allowed/forbidden state edge; the four exact artifact-pin retention wires, lease live-at boundaries, persistent-reference release shape, strict storage-bootstrap/root-binding bytes, and no database reclamation policy in G0.

Run: `cargo test -p ep-foundation --test signature -- --nocapture`

Expected: PASS for canonical digest stability, wrong purpose/key/digest/signature rejection, all eight closed `ArtifactSigningErrorV1` branches, verifier-only private `VerifiedBusinessArtifactV1<T>` plus `VerifiedSignedEnvelopeBytesV1` construction with exact media/whole-envelope/non-forgeable trust-policy proof and raw construction/reserialization/cross-policy negatives; stable generated artifact-type IDs and descriptor lookup with module-refactor digest stability; the exact nine-field authorization binding and non-self-referential digest; checked provider-handle construction; legal downstream release payload preparation; unregistered/handwritten type-ID architecture failure; every descriptor/binding/window/handle mutation; equal inclusive boundary, inversion and static-none misuse; the exhaustive issuance descriptor including exactly the three current-approved issued-at-only kinds; exact F-56 SPKI-token wire, DN-as-subject and same-DN/different-key negatives; bootstrap-root/token/DN enforcement; strict schema equality; the direct tagged authority ref; the exact generation/approval offline descriptor contracts; and the canonical 89-row registry inside the exact five-field payload with one immutable two-role archive trust-anchor policy and all policy mutation negatives.

Run: `cargo test -p ep-platform-evidence-trust -p ep-adapter-kms -p evidence-trust-tool -p evidence-signing-broker --all-targets --locked`

Expected: PASS for the complete 89-row-to-credential bijection, separate registry-authority/TSA requirements, exact two provider modes, identical typed contract/validation/time/error semantics (with byte-identical CMS required only for the same frozen operation, never across provider modes), nonexportable/ACL/chain/revocation checks, fixed authenticated broker endpoints, typed authorization-only digest signing, durable operation allocation/query/adopt with byte-identical CMS across all four response-loss cuts, protected journal crash adoption, initial/rotation separation, rejection of a non-idempotent enterprise signer/TSA, and every partial/ambient/wildcard/arbitrary-payload/network-egress negative.

Run on the approved Windows Server 2022 runner: `cargo test -p ep-platform-evidence-trust --test windows_broker_install -p evidence-signing-broker --test windows_service --locked -- --nocapture`

Expected: PASS for the exact fixed service/static-row/schema/WiX/MSI-extraction projection; quoted ImagePath and canonical argv; service/executable/pipe/group DACLs and concrete `0x00120183` client mask; client-group exact membership; inability of a client to create the first/second/replacement pipe; final-handle/AuthentiCode/process-token/readiness challenge; DATA_HDD-only mutable state with zero SSD fallback; self-hosted zero-egress and enterprise literal signer/TSA allowlist; wrong account/SID/type/start/recovery/dependency/privilege/descriptor/member/path/media/root/session/provider/socket mutations; and byte-import compatibility with the later production-host installer.

Run on an approved Windows runner: `cargo test -p ep-platform-powershell-trust -p powershell-trust-tool --all-targets --locked`

Expected: PASS for exact policy/descriptor goldens, self-hosted and connected signer/TSA modes, nonexportable key/ACL and machine TrustedPublisher ceremony, offline chain/revocation/RFC-3161 verification, final signed-file hash/content-digest equality, effective AllSigned, fixed Microsoft host identity, clean-root/no-reparse/final-handle execution, nonnull verified `lpApplicationName`/`lpCurrentDirectory`, the unique Windows argv UTF-16 encoder/parser round trip and 32,767-code-unit boundary, STARTED-bound application/current-directory/command-line hashes, write/delete-share denial through exit, post-STARTED same-file revalidation, and every PATH/relative-file/reparse/replacement/policy/signer/timestamp/caller-argv/NUL/overflow/quoting negative.

Run on the approved signer host before staging the real registry: `cargo run -p evidence-trust-tool --locked -- prepare --mode SELF_HOSTED_OS_KEYSTORE_PIV_V1 --state-root <approved-absolute-protected-state-root> --deployment-manifest <absolute-file> --deployment-manifest-signature <absolute-file> --deployment-trust-bundle <absolute-file> --initial`

Run: `cargo run -p evidence-trust-tool --locked -- verify --state-root <same-root>`

Run: `cargo run -p evidence-trust-tool --locked -- seal-registry --state-root <same-root> --out docs/f57-artifact-signer-registry.v1.json`

Run: `cargo run -p evidence-trust-tool --locked -- install-broker --state-root <same-root>`

Expected: the recommended self-hosted ceremony create-new seals and typed-reloads the exact non-placeholder five-field/89-row signed registry, installs one matching broker session and exposes no private key. `--mode EXISTING_ENTERPRISE_SIGNER_V1 --provider-config <approved-absolute-file>` is the only alternative and must yield the same contract. A missing product-pinned deployment signing handle returns `F57_EVIDENCE_TRUST_PREREQUISITE_MISSING` with no registry or active session.

Run: `cargo test -p ep-foundation --test validated_path -- --nocapture`

Expected: PASS for canonical absolute existing-file construction and relative, absent, ADS, trailing-dot/space, normalization-alias, duplicate-separator and attempted direct-field construction negatives; no test may claim DATA_HDD or trust verification.

Run: `cargo test -p ep-platform-runtime --test topology -- --nocapture`

Expected: PASS for the exact declaration/certification fields, pure later-type-free deterministic builders, two purpose wires, two media bindings and byte goldens; the foundation-only schema edge; private three-profile registries; exact ArtifactRef/media/size/digest/JCS/live-readback construction of `VerifiedRuntimeTopologyDeclarationV1`; gateway-zero-database and every actual-drift negative; zero G0 deployment-declaration output, zero G0–G5 production-certification authority, zero signed wrapper/signer row/second owner; the required future offline-closure descriptor; and both production-path `NOT_DUE` guards.

Run: `cargo check --workspace --locked`

Expected: PASS with the one-way `ep-platform-runtime -> ep-platform-capability-graph` edge present in workspace metadata and `Cargo.lock`, no reverse edge/cycle, and zero lockfile rewrite.

- [ ] **Step 5: Commit contracts.**

```bash
cargo xtask f57 task stage --task G0-05
cargo xtask f57 task verify-staged --task G0-05
git commit -m "feat: add signing generation and topology contracts"
```

### Task 6: Implement L0/L1 selection and issue G0_BOOTSTRAP_GREEN

**Files:**
- Modify: `xtask/src/f57/cli.rs`
- Modify: `xtask/src/f57/evidence.rs`
- Modify: `xtask/src/f57/gate.rs`
- Modify: `xtask/src/f57/verify.rs`
- Create: `xtask/src/f57/run_journal.rs` (composition/reconciler registry only; no shared wire definitions)
- Create: `xtask/src/f57/run_artifact_store.rs`
- Create: `xtask/src/f57/evidence_field_bindings.rs`
- Create: `crates/platform/gate-journal-contract/Cargo.toml`
- Create: `crates/platform/gate-journal-contract/src/lib.rs`
- Create: `crates/platform/gate-journal-contract/src/storage_root_binding.rs`
- Create: `crates/platform/gate-journal-contract/src/journal.rs`
- Create: `crates/platform/gate-journal-contract/src/port.rs`
- Create: `crates/platform/gate-journal-contract/tests/wire.rs`
- Create: `crates/platform/gate-journal-contract/tests/transition.rs`
- Create: `crates/platform/gate-journal-contract/tests/fixtures/f57-storage-root-binding-v1-golden.json`
- Create: `crates/platform/runtime/src/evidence/mod.rs`
- Create: `crates/platform/runtime/src/evidence/placement.rs`
- Create: `crates/platform/runtime/src/evidence/object_store.rs`
- Create: `crates/platform/runtime/src/evidence/input_store.rs`
- Create: `crates/platform/runtime/tests/evidence_object_store_port.rs`
- Create: `crates/platform/runtime/tests/evidence_input_store_port.rs`
- Create: `crates/adapter/file/src/evidence_object_store.rs`
- Create: `crates/adapter/file/src/evidence_input_store.rs`
- Create: `crates/adapter/file/src/gate_run_journal_store.rs`
- Create: `crates/adapter/file/src/authority_storage_bootstrap_archive.rs`
- Create: `crates/adapter/file/tests/evidence_object_store.rs`
- Create: `crates/adapter/file/tests/evidence_input_store.rs`
- Create: `crates/adapter/file/tests/gate_run_journal_store.rs`
- Create: `crates/adapter/file/tests/authority_storage_bootstrap_archive.rs`
- Modify: `crates/platform/runtime/src/lib.rs`
- Modify: `crates/platform/runtime/Cargo.toml`
- Modify: `crates/adapter/file/src/lib.rs`
- Modify: `crates/adapter/file/Cargo.toml`
- Modify: `xtask/Cargo.toml`
- Modify: `Cargo.toml`
- Create: `docs/f57-evidence-placement-classes.v1.tsv`
- Create: `docs/f57-evidence-field-contracts.v1.tsv`
- Create: `docs/generated/f57/evidence-placement-classes.v1.json`
- Create: `docs/generated/f57/evidence-object-field-bindings.v1.json`
- Create: `docs/generated/f57/evidence-input-field-bindings.v1.json`
- Create: `docs/generated/f57/evidence-field-binding-manifest.v1.json`
- Create: `docs/generated/f57/evidence-contract-source-freeze.v1.json`
- Create: `crates/platform/runtime/src/evidence/generated_bindings.rs`
- Create: `xtask/tests/fixtures/f57-evidence-field-binding-registry-v1-golden.json`
- Create: `xtask/tests/fixtures/f57-evidence-placement-class-registry-v1-golden.json`
- Modify: `Cargo.lock`
- Create: `xtask/src/f57/carrier.rs`
- Create: `xtask/src/f57/client_conformance.rs`
- Create: `xtask/src/f57/fresh_pg.rs`
- Create: `xtask/tests/f57_levels.rs`
- Create: `xtask/tests/f57_fresh_pg.rs`
- Create: `xtask/tests/f57_run_journal.rs`
- Create: `xtask/tests/f57_client_conformance_dispatch.rs`
- Create: `xtask/tests/fixtures/f57-client-conformance-result-v1-golden.json`
- Create: `xtask/tests/fixtures/f57-candidate-bound-fresh-pg-evidence-v1-golden.json`
- Create: `testkit/src/f57_cases/probes/g0_bootstrap.rs`
- Create: `testkit/tests/f57_slice_probes_g0_bootstrap.rs`
- Create: `xtask/tests/fixtures/f57-gate-run-journal-v1-golden.jcs.jsonl`
- Create: `xtask/tests/fixtures/f57-gate-run-journal-checkpoint-v1-golden.json`
- Create: `xtask/tests/fixtures/f57-p340-qualification-journal-events-v1-golden.json`
- Create: `xtask/tests/fixtures/f57-run-artifact-store-descriptors-v1-golden.json`
- Create: `docs/evidence/f57-gate-run-journal.v1.schema.json`
- Create: `docs/evidence/f57-client-conformance-result.v1.schema.json`
- Create: `docs/evidence/f57-fresh-pg-evidence.schema.json`
- Create: `docs/evidence/f57-migration-apply-manifest.schema.json`
- Create: `docs/generated/f57/migration-apply-manifest.v1.json`
- Read: `docs/evidence/f57-foundation.v1.schema.json`
- Read: `docs/evidence/f57-generation.v1.schema.json`
- Read: `docs/schemas/f57-generation-approval-registry.v1.schema.json`
- Read: `docs/evidence/f57-runtime-topology.v1.schema.json`
- Read: `docs/evidence/f57-requirement-evidence-binding.v1.schema.json`
- Read: `docs/f57-fresh-pg-check-registry.v1.tsv`
- Read: `docs/f57-artifact-signer-registry.v1.json`
- Read: `docs/schemas/f57-artifact-signer-registry.v1.schema.json`
- Read: `crates/platform/evidence-trust/src/broker_protocol.rs`
- Modify: `db/migrations/platform_core/V20260901091500__platform_core_key_domains.sql`
- Modify: `db/migrations/platform_core/V20260901092000__platform_core_data_keys.sql`
- Modify: `db/migrations/platform_core/V20261012090500__platform_core_identity_user_credentials.sql`
- Modify: `docs/migration-catalog.md`
- Modify: `docs/superpowers/reviews/2026-08-23-f57-authority-supersession-register.md`
- Create: `.github/ci/f57-l0.ps1`
- Create: `.github/ci/f57-l1.ps1`
- Modify: `.github/workflows/ci.yml`
- Modify: `docs/ci-pipeline.md`
- Modify: `xtask/src/f57/mod.rs`
- Modify: `README.md`

**Interfaces:**
- Consumes: delivery registry, Cargo dependency graph, graph manifest, generator, archcheck, sqlcheck, test executors, the exact three-root `13/9/14` generation schema/parser/digest/reverse-contract bytes, the separate exact seven-field/three-row generation-approval-registry schema/bootstrap contract bytes, the exact two-root runtime-topology schema/builder/verifier-contract bytes, the exact signed 89-row artifact-signer registry, the exact 78-row baseline registry, 388-row catalog closure, three registered draft preimages/target contracts, 310-row legacy disposition seed, 47-row F57 reservation suffix, and the current 27-row Fresh-PG check registry. The old Fresh-PG task-profile seed is never a runtime input.
- Produces: the closed G0 dispatch outcome/error used by `verify`, generic `fresh-pg --profile <profile> --through <baseline-or-reserved-version>`, and one reusable signed/fsynced journal engine. `crates/platform/release/src/carrier_contract.rs` is the sole early production-linkable owner of the exact closed `ReleaseCarrierRecipeIdV1` values `WINDOWS_AUTHORITY_BUILD|WINDOWS_SERVICE_INSTALL|POSTGRES16_PITR|BACKUP_RESTORE_CERTIFICATION|P340_RELEASE72_HOUR|POWER_SHUTDOWN`; the G0 parser/dispatcher imports that type, returns `NOT_DELIVERED` for all six, and G6 Task 14 later supplies mappings without changing/redeclaring the enum. The neutral `ep-platform-gate-journal-contract` is the sole production-linkable Rust owner of `AuthorityStorageTrustBootstrapV1|G6VerifiedDataHddRootBindingV1`, all header/record/checkpoint/prefix nominals, `GateRunJournalReadPortV1`, the exact seven-variant `P340QualificationOperationKindV1`, `P340QualificationOutputRefV1`, five qualification variants, five POWER-continuation variants and three completion-finalization variants; all thirteen later delta variants are strict/reserved at G0 and remain `NOT_DELIVERED` until their owning task supplies semantics. `GateRunJournalReadPortV1` can load only one explicit persisted run and authenticate an exact supplied checkpoint/prefix before returning the exact terminal record for a supplied TestID; it has no latest/scan/path method. `xtask` only composes the shared codec/ports and reconcilers. The same task produces the three run output authorities, sequence-derived checkpoint store, production-linkable `EvidenceObjectStoreV1` and `EvidenceInputStoreV1` ports with their sole shared file adapters, and `AuthorityStorageBootstrapArchiveStoreV1` with exactly seven fixed `inputs/storage-root/*` rows (the signed storage-manifest envelope plus six bootstrap members). It also commits a separately reviewed placement-class seed and occurrence-binding seed, generated placement/object/input registries, runtime Rust registry, exact counts/digests/source-freeze manifest and independent byte goldens. Classes exist before containing roots; occurrence rows name only a class. The exact bootstrap accessors are `RUN_BOOTSTRAP_ARTIFACT_SIGNER_REGISTRY_V1|AUTHORITY_BOOTSTRAP_STORAGE_MANIFEST_V1|GENERATION_BOOTSTRAP_APPROVAL_REGISTRY_V1`. No generator implementation may be staged until the documentation-only source-freeze manifest records source row counts, class/object/input counts, source/output digests and two distinct reviewer identities; generator and later schema-reachability checks are independent consumers of that frozen source. Later tasks only activate/exact-check pre-registered classes/occurrences. The DATA_HDD constructors are deferred to G1-01 after its root type exists. It further produces exact signer-registry input materialization; closed release/client dispatchers initially returning `NOT_DELIVERED`; deterministic migration apply manifest and rebaseline; two G0 probe handlers; and the signer-registry-referenced, checkpoint-bound `G0_BOOTSTRAP_GREEN` receipt.

**Gate-journal read boundary (normative):** the `Produces` summary's phrase about returning a terminal record for a supplied TestID is superseded by the exact port below. G0 returns only an authenticated prefix for the exact supplied checkpoint. It accepts no TestID or domain type; an upper owner selects and verifies its own terminal record through the prefix's read-only accessors.

- [ ] **Step 1: Write failing level-selection tests.**

```rust
#[test]
fn l0_selects_changed_feature_and_graph_drift() {
    let selected = select(FixtureChangeSet::paths(["crates/features/customer-master/src/domain/customer.rs"]), EvidenceLevelV1::L0Developer).unwrap();
    assert!(selected.contains("capability-graph-no-diff"));
    assert!(selected.contains("feature:customer-master:unit-property"));
    assert!(!selected.contains("p340-72h"));
}

#[test]
fn release_carrier_recipe_contract_exists_before_the_dispatcher_and_is_closed() {
    use ep_platform_release::carrier_contract::ReleaseCarrierRecipeIdV1;
    assert_eq!(ReleaseCarrierRecipeIdV1::wire_values(), [
        "WINDOWS_AUTHORITY_BUILD",
        "WINDOWS_SERVICE_INSTALL",
        "POSTGRES16_PITR",
        "BACKUP_RESTORE_CERTIFICATION",
        "P340_RELEASE72_HOUR",
        "POWER_SHUTDOWN",
    ]);
    for recipe in ReleaseCarrierRecipeIdV1::all() {
        assert_code(g0_carrier_dispatch(*recipe), "F57_CARRIER_NOT_DELIVERED");
    }
    assert_code(parse_carrier_recipe("windows-authority-build"), "F57_CARRIER_RECIPE_INVALID");
    assert!(carrier_dispatcher_imports_release_contract_without_local_enum());
}

#[test]
fn missing_due_test_is_failure_not_skip() {
    let err = verify_fixture_with_missing_due_test().unwrap_err();
    assert_eq!(err.code(), "F57_DUE_TEST_NOT_DELIVERED");
}

#[test]
fn fresh_pg_requires_created_contiguous_reservations_and_clean_database() {
    assert_code(fresh_pg_fixture_with_reserved_not_created_row(), "F57_MIGRATION_NOT_DELIVERED");
    assert_code(fresh_pg_fixture_with_gap(), "F57_MIGRATION_SEQUENCE_GAP");
    assert_code(fresh_pg_fixture_with_preexisting_schema(), "F57_FRESH_PG_NOT_EMPTY");
}

#[test]
fn fresh_pg_always_applies_exact_signed_baseline_before_f57_suffix() {
    let result = run_fresh_pg(FreshPgInvocationV1::engineering(
        DeliveryProfileV1::G0Bootstrap,
        20261012113500,
    )).unwrap();
    assert_eq!(result.applied_baseline_count, 69);
    assert_eq!(result.applied_f57_count, 0);
    assert_eq!(result.applied_total_count, 69);
    assert_eq!(result.baseline_registry_sha256, hex_sha256("52930d7ae32ee02ddda38199bcc144f5f6747fcfbe33e740741a0f21604ca8fd"));
}

#[test]
fn fresh_pg_rejects_baseline_absence_preimage_postimage_and_order_drift() {
    assert_code(fresh_pg_fixture_with_absent_path_materialized(), "F57_MIGRATION_ABSENT_PATH_PRESENT");
    assert_code(fresh_pg_fixture_with_wrong_immutable_hash(), "F57_MIGRATION_BASELINE_HASH_DRIFT");
    assert_code(fresh_pg_fixture_with_uncorrected_draft_after_g0(), "F57_MIGRATION_DRAFT_NOT_REBASELINED");
    assert_code(fresh_pg_fixture_with_wrong_draft_postimage(), "F57_MIGRATION_DRAFT_TARGET_DRIFT");
    assert_code(fresh_pg_fixture_with_f57_before_baseline(), "F57_MIGRATION_APPLY_ORDER_INVALID");
}

#[test]
fn dirty_or_direct_fresh_pg_can_never_issue_candidate_evidence() {
    let result = run_fresh_pg(FreshPgInvocationV1::direct_from_dirty_fixture()).unwrap();
    assert_eq!(result.evidence_class, FreshPgEvidenceClassV1::EngineeringRehearsal);
    assert!(result.signed_receipt.is_none());
    assert_code(attempt_candidate_receipt_from_dirty_tree(), "F57_FRESH_PG_CANDIDATE_REQUIRES_CLEAN_HEAD");
}

#[test]
fn candidate_bound_fresh_pg_wire_and_single_run_ref_are_exact() {
    let signed = candidate_bound_fresh_pg_fixture().unwrap();
    let journal = candidate_bound_fresh_pg_journal_fixture();
    assert_eq!(signed.payload.evidence_class, FreshPgEvidenceClassV1::CandidateBound);
    assert_eq!(signed.payload.outcome, FreshPgEvidenceOutcomeV1::Pass);
    assert_eq!(signed.payload.postgres_major, 16);
    assert_eq!(signed.payload.execution_attempt_id, journal.candidate_bound_fresh_pg_execution_attempt_id());
    assert!(journal.candidate_bound_fresh_pg_start_context_refs().is_empty());
    assert_eq!(signed.payload.fresh_pg_check_registry_sha256, hex_sha256("76fed80fcf5f73a64c769cb37f7aadf2d217c813554acc893f7ca875004ce01a"));
    assert!(signed.payload.checks_exact_join_profile_registry());
    assert_eq!(
        signed.exact_bytes(),
        include_bytes!("fixtures/f57-candidate-bound-fresh-pg-evidence-v1-golden.json"),
    );
    assert_eq!(
        generated_fresh_pg_schema_bytes(),
        read("docs/evidence/f57-fresh-pg-evidence.schema.json"),
    );
    assert!(all_fresh_receipt_refs_in_run_name_same_envelope());
    assert_code(signed_engineering_rehearsal(), "F57_FRESH_PG_SIGNED_ENGINEERING_FORBIDDEN");
    assert_code(fresh_pg_with_missing_profile_check(), "F57_FRESH_PG_CHECK_SET_MISMATCH");
    assert_code(fresh_pg_with_later_same_profile_check(), "F57_FRESH_PG_CHECK_SET_MISMATCH");
    assert_code(fresh_pg_with_old_seed_task_or_argv(), "F57_HISTORICAL_INPUT_NOT_EXECUTABLE");
}

#[test]
fn g0_task6_schema_edges_and_client_conformance_wire_are_exact() {
    let fresh_pg = strict_schema("docs/evidence/f57-fresh-pg-evidence.schema.json");
    let journal = strict_schema("docs/evidence/f57-gate-run-journal.v1.schema.json");
    let conformance = strict_schema("docs/evidence/f57-client-conformance-result.v1.schema.json");
    assert_eq!(fresh_pg.imports(), ["f57-foundation.v1.schema.json"]);
    assert_eq!(journal.imports(), ["f57-foundation.v1.schema.json"]);
    assert_eq!(conformance.imports(), [
        "f57-foundation.v1.schema.json",
        "f57-requirement-evidence-binding.v1.schema.json",
    ]);
    assert!(fresh_pg.composes_foundation_envelope_exactly_once());
    assert!(conformance.composes_foundation_envelope_exactly_once());
    assert_eq!(conformance.external_defs_reused_from_requirement_evidence(), [
        "RequirementCandidateProvenanceV1",
    ]);
    assert!(!conformance.imports_schema("f57-client-common.v1.schema.json"));
    assert!(!fresh_pg.imports_result_gate_client_or_release_schema());
    assert!(!journal.imports_requirement_result_client_p340_or_release_schema());
    assert_eq!(conformance.media_type(),
        "application/vnd.ep.f57-client-conformance-result-v1+json");
    assert_eq!(ClientConformanceIdV1::wire_values(), [
        "G3_SHELL", "G4_CTC_UI_API", "G5_FOUR_PLATFORM",
    ]);
    assert_eq!(ClientConformanceResultPurposeV1::wire_values(), [
        "EP-F57-CLIENT-CONFORMANCE-RESULT-V1",
    ]);
    assert_eq!(ClientConformanceOutcomeV1::wire_values(), [
        "PASS", "FAIL", "NOT_DELIVERED",
    ]);
    assert_eq!(conformance.result_payload_field_names(), [
        "schema_version", "purpose", "conformance_id", "auxiliary_test_id",
        "stack", "recipe_id", "candidate_run", "execution_attempt_id",
        "provenance", "outcome", "evidence_refs", "issued_at_unix_ms",
        "expires_at_unix_ms",
    ]);
    assert_eq!(client_conformance_recipe_dispatch_table().len(), 6);
    assert!(client_conformance_recipe_dispatch_table().has_exactly_one_row_per_stack_and_id());
    assert!(client_conformance_recipe_dispatch_table().all_delivery_states_are_not_delivered());
    assert_eq!(client_conformance_wire_golden_bytes(), include_bytes!(
        "fixtures/f57-client-conformance-result-v1-golden.json"
    ));
    assert!(f57_schema_import_dag().is_acyclic());
    assert_code(conformance_with_client_common_import_or_copied_provenance(), "F57_SCHEMA_IMPORT_DAG_INVALID");
}

#[test]
fn journal_wire_state_machine_and_checkpoint_are_exact() {
    let journal = journal_fixture_with_started_completed_and_bound_candidate();
    assert!(journal.header().payload.storage_root_binding.is_none());
    assert_eq!(journal.first_record_sequence(), 1);
    assert!(journal.frames_match_len8_tab_jcs_lf());
    assert!(journal.hash_chain_and_signed_checkpoint_verify());
    assert_eq!(journal.bytes(), include_bytes!("fixtures/f57-gate-run-journal-v1-golden.jcs.jsonl"));
    assert_eq!(generated_journal_schema_bytes(), read("docs/evidence/f57-gate-run-journal.v1.schema.json"));
    let checkpoint = journal.latest_checkpoint().unwrap();
    let last_sequence = checkpoint.payload.last_sequence;
    let stored = journal_checkpoint_store_for_fixed_run().put(checkpoint).unwrap();
    assert_eq!(stored.relative_path(), format!(
        "runs/{}/checkpoints/{:020}.v1.json",
        journal.gate_run_id().hyphenated().to_string().to_ascii_lowercase(),
        last_sequence,
    ));
    assert_eq!(stored.exact_bytes(), include_bytes!("fixtures/f57-gate-run-journal-checkpoint-v1-golden.json"));
    assert_code(checkpoint_store_with_caller_path_or_unpadded_sequence(), "F57_JOURNAL_CHECKPOINT_PATH_NONCANONICAL");
    assert_code(checkpoint_whose_prefix_ends_at_other_sequence(), "F57_JOURNAL_CHECKPOINT_PREFIX_MISMATCH");
    let g6 = g6_header_fixture_with_verified_storage_root();
    assert!(g6.payload.storage_root_binding.as_ref().unwrap().validate_contract().is_ok());
    assert_code(non_g6_header_with_storage_root(), "F57_JOURNAL_STORAGE_ROOT_PROFILE_MISMATCH");
    assert_code(g6_header_without_storage_root(), "F57_JOURNAL_STORAGE_ROOT_REQUIRED");
    assert_code(g6_header_with_self_authorizing_or_unknown_bootstrap(), "F57_JOURNAL_STORAGE_TRUST_BOOTSTRAP_INVALID");
}

#[test]
fn gate_journal_sole_owns_all_p340_qualification_nominals_and_five_events() {
    assert_eq!(P340QualificationOperationKindV1::wire_values(), [
        "WINDOWS_PATCH_POLICY_ATTESTATION",
        "BITLOCKER_RECOVERY_CUSTODY_ATTESTATION",
        "UPS_POWER_WRITE_CACHE_POLICY",
        "HDD_FLUSH_POWER_CUT_VERIFICATION",
        "UPS_IDENTITY_READBACK",
        "SSD_CLEAN_REINSTALL_RESTORE",
        "MEMORY_TEST_READBACK",
    ]);
    let schema = strict_schema("docs/evidence/f57-gate-run-journal.v1.schema.json");
    assert_eq!(schema.p340_qualification_event_wires(), [
        "P340_QUALIFICATION_OPERATION_STARTED",
        "P340_QUALIFICATION_OPERATION_COMPLETED",
        "P340_QUALIFICATION_OPERATION_UNKNOWN",
        "P340_QUALIFICATION_OPERATION_RECONCILED",
        "P340_QUALIFICATION_CLOSURE_BOUND",
    ]);
    assert_eq!(schema.p340_qualification_event_field_sets(), expected_five_gate_owned_field_sets());
    assert_eq!(p340_qualification_journal_wire_golden_bytes(), include_bytes!(
        "fixtures/f57-p340-qualification-journal-events-v1-golden.json"
    ));
    assert_eq!(p340_qualification_closure_fixture().outputs.len(), 7);
    assert!(p340_qualification_closure_fixture().has_every_kind_once_in_ordinal_order());
    assert!(schema.has_exact_foundation_import());
    assert!(!schema.imports_p340_or_release_schema());
    assert!(!p340_qualification_vocabulary_adds_test_id_or_signer_row());
    assert_code(p340_qualification_closure_with_six_duplicate_or_reordered_outputs(), "F57_P340_QUALIFICATION_CLOSURE_INVALID");
    assert_code(g0_attempt_to_execute_p340_qualification(), "F57_CARRIER_NOT_DELIVERED");
}

#[test]
fn crash_recovery_never_starts_a_test_or_manifest_twice() {
    assert_code(journal_fixture_with_second_start(), "F57_JOURNAL_DUPLICATE_START");
    assert_code(journal_fixture_with_started_then_reconciled(), "F57_JOURNAL_TRANSITION_INVALID");
    assert_code(journal_fixture_with_completed_then_second_result(), "F57_JOURNAL_RESULT_ALREADY_TERMINAL");
    assert_code(journal_fixture_with_checkpoint_rollback(), "F57_JOURNAL_CHECKPOINT_NOT_EXTENSION");
    assert_code(candidate_fixture_with_conflicting_existing_output(), "F57_CANDIDATE_OUTPUT_CONFLICT");
    assert!(candidate_fixture_output_before_bound_recovers_without_resigning().is_ok());
    assert!(candidate_fixture_bound_before_response_is_idempotent().is_ok());
    assert!(candidate_fixture_finalization_started_then_first_write_uses_frozen_inputs().is_ok());
    assert!(candidate_fixture_output_before_reconciled_adopts_without_resigning().is_ok());
    assert_code(candidate_fixture_with_second_finalization_attempt(), "F57_CANDIDATE_FINALIZATION_ALREADY_STARTED");
}

#[test]
fn all_276_test_results_have_one_run_scoped_typed_output_and_crash_adoption() {
    let store = test_result_store_for_fixed_run();
    assert_eq!(store.descriptors().len(), 276);
    assert!(store.paths_are_injective_and_below_bundle_root());
    assert!(store.descriptor_counts_are([185, 78, 3, 4, 6]));
    for kind in all_five_test_result_envelope_kinds() {
        assert!(started_record_has_registered_start_context_exact_set(kind));
        assert!(result_output_before_terminal_is_reconciled_without_resigning(kind));
        assert!(result_terminal_before_response_returns_exact_existing_bytes(kind));
        assert_code(result_with_conflicting_existing_bytes(kind), "F57_TEST_RESULT_OUTPUT_CONFLICT");
        assert_code(result_with_wrong_media_or_schema(kind), "F57_TEST_RESULT_TYPE_MISMATCH");
    }
    assert_code(result_path_without_run_id(), "F57_TEST_RESULT_PATH_NONCANONICAL");
    assert_code(terminal_result_with_missing_output(), "F57_TEST_RESULT_OUTPUT_MISSING");
}

#[test]
fn two_process_race_has_one_durable_starter_and_zero_second_side_effect() {
    let race = race_two_processes_for_same_test_id();
    assert_eq!(race.started_count, 1);
    assert_eq!(race.physical_invocation_count, 1);
    assert_eq!(race.loser_error_code, "F57_JOURNAL_BUSY");
    assert_eq!(race.loser_exit_code, 1);
    assert!(crashed_owner_releases_os_lease_but_forces_reconciler_only());
}

#[test]
fn all_three_run_output_stores_and_checkpoint_store_are_create_new_and_recoverable() {
    let result_store = test_result_store_for_fixed_run();
    let envelope_store = evidence_envelope_store_for_fixed_run();
    let candidate_store = candidate_manifest_store_for_fixed_run();
    let checkpoint_store = journal_checkpoint_store_for_fixed_run();
    assert_eq!(result_store.descriptors().len(), 276);
    assert_eq!(envelope_store.descriptors().len(), 14);
    assert_eq!(candidate_store.descriptors().len(), 3);
    assert!(!envelope_store.descriptor_kinds().contains(
        &EvidenceEnvelopeKindV1::OfflineSchemaManifest,
    ));
    assert!(all_three_store_paths_equal_master_descriptor_golden());
    assert!(checkpoint_store.paths_are_sequence_derived_and_below_bundle_root());
    assert!(all_explicit_out_and_evidence_out_examples_exact_match_store());
    for kind in envelope_store.descriptor_kinds() {
        assert!(frozen_checkpoint_precedes_single_finalization_started(kind));
        assert!(output_before_bound_recovers_without_resigning(kind));
        assert!(bound_before_response_returns_exact_existing_bytes(kind));
        assert_code(conflicting_existing_output(kind), "F57_EVIDENCE_OUTPUT_CONFLICT");
    }
    for kind in all_candidate_manifest_kinds() {
        assert!(candidate_finalization_id_matches_started_bound_or_reconciled(kind));
        assert!(candidate_output_before_bound_recovers_without_resigning(kind));
        assert_code(candidate_at_noncanonical_path(kind), "F57_CANDIDATE_OUTPUT_NONCANONICAL");
    }
    assert_code(envelope_at_noncanonical_path(), "F57_EVIDENCE_OUTPUT_NONCANONICAL");
    assert_code(reusing_run_directory_for_second_header(), "F57_RUN_DIRECTORY_ALREADY_OWNED");
    assert!(bound_event_order_includes_single_fresh_pg_before_candidate_and_receipts());
}

#[test]
fn signed_signer_registry_materializes_once_and_binds_every_candidate_and_receipt() {
    let source = bootstrap_verified_artifact_signer_registry().unwrap();
    let materialized = materialize_artifact_signer_registry(&source, fixed_bundle_root()).unwrap();
    assert_eq!(materialized.uri, format!(
        "evidence-relative://bundle/inputs/{}.json",
        sha256(source.exact_signed_envelope_bytes()),
    ));
    assert_eq!(materialized.media_type, "application/vnd.ep.f57-artifact-signer-registry-v1+json");
    assert_eq!(read_ref_bytes(&materialized), source.exact_signed_envelope_bytes());
    assert!(all_candidate_identities_exact_match_registry_digest(&materialized));
    assert!(all_candidates_and_gate_receipts_exact_reference_registry(&materialized));
    assert_code(materialized_registry_with_source_byte_drift(), "F57_SIGNER_REGISTRY_MATERIALIZATION_MISMATCH");
}

#[test]
fn every_nested_artifact_ref_uses_the_content_addressed_object_store() {
    let object = object_store().put(fixed_bytes(), "application/vnd.ep.f57-test-fixture-v1+json").unwrap();
    assert_eq!(object.uri, format!("evidence-relative://bundle/objects/sha256/{}", object.sha256));
    assert!(object_store().put(fixed_bytes(), object.media_type.as_str()).unwrap().exact_eq(&object));
    assert_code(object_store_with_caller_path(), "F57_EVIDENCE_OBJECT_PATH_FORBIDDEN");
    assert_code(typed_consumer_with_object_media_alias(&object), "F57_EVIDENCE_OBJECT_MEDIA_MISMATCH");
    assert_code(object_store_symlink_escape(), "F57_EVIDENCE_OBJECT_PATH_ESCAPE");
    assert!(unreachable_object_is_ignored_without_directory_scan());
}

#[test]
fn gate_class_binding_relation_is_closed_and_journal_bound() {
    assert_eq!(valid_gate_class_binding_rows().len(), 6);
    assert!(all_valid_rows_round_trip_golden_schema());
    assert_code(cross_product_invalid_row(), "F57_GATE_BINDING_RELATION_INVALID");
    assert_code(receipt_without_latest_checkpoint(), "F57_GATE_JOURNAL_CHECKPOINT_MISSING");
    assert_code(receipt_without_artifact_signer_registry_ref(), "F57_GATE_SIGNER_REGISTRY_REF_MISSING");
    assert_code(receipt_with_signed_prerequisite_wrapper(), "F57_GATE_PREREQUISITE_REF_TYPE_INVALID");
}
```

- [ ] **Step 2: Run tests and verify RED.**

Run: `cargo test -p ep-platform-release --test carrier_contract -- --nocapture`

Run: `cargo test -p ep-xtask --test f57_levels --test f57_fresh_pg --test f57_run_journal --test f57_client_conformance_dispatch -- --nocapture`

Expected: FAIL because `verify`, the level selector, the journal engine, and its reserved P340 qualification vocabulary do not exist.

- [ ] **Step 3: Implement Rust-owned L0/L1 verdicts and thin CI adapters.**

```rust
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum EvidenceLevelV1 { L0Developer, L1PullRequest, L2IntegrationCandidate, L3ReleaseCertification }

use ep_platform_release::carrier_contract::ReleaseCarrierRecipeIdV1;

pub enum CarrierDispatchErrorV1 {
    NotDelivered { recipe: ReleaseCarrierRecipeIdV1 },
}

impl CarrierDispatchErrorV1 {
    pub fn code(&self) -> &'static str { "F57_CARRIER_NOT_DELIVERED" }
    pub fn exit_code(&self) -> i32 { 70 }
}

// Task 6 implements dispatch behavior; the enum already existed for the Task-1 parser.
pub fn dispatch_release_carrier(
    recipe: ReleaseCarrierRecipeIdV1,
) -> Result<std::convert::Infallible, CarrierDispatchErrorV1> {
    Err(CarrierDispatchErrorV1::NotDelivered { recipe })
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum P340QualificationOperationKindV1 {
    WindowsPatchPolicyAttestation,
    #[serde(rename = "BITLOCKER_RECOVERY_CUSTODY_ATTESTATION")]
    BitLockerRecoveryCustodyAttestation,
    UpsPowerWriteCachePolicy,
    HddFlushPowerCutVerification,
    UpsIdentityReadback,
    SsdCleanReinstallRestore,
    MemoryTestReadback,
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct P340QualificationOutputRefV1 {
    pub operation_kind: P340QualificationOperationKindV1,
    pub execution_attempt_id: UuidV1,
    pub output_ref: ArtifactRefV1,
}

// Sole owner: ep-platform-gate-journal-contract::port; private fields/accessors only.
pub struct VerifiedGateRunJournalPrefixV1 {
    header: VerifiedBusinessArtifactV1<GateRunJournalHeaderPayloadV1>,
    records: Vec<VerifiedBusinessArtifactV1<GateRunJournalRecordPayloadV1>>,
    checkpoint: VerifiedBusinessArtifactV1<GateRunJournalCheckpointPayloadV1>,
}

impl VerifiedGateRunJournalPrefixV1 {
    pub fn header(&self) -> &VerifiedBusinessArtifactV1<GateRunJournalHeaderPayloadV1> {
        &self.header
    }

    pub fn records(&self) -> &[VerifiedBusinessArtifactV1<GateRunJournalRecordPayloadV1>] {
        &self.records
    }

    pub fn checkpoint(&self) -> &VerifiedBusinessArtifactV1<GateRunJournalCheckpointPayloadV1> {
        &self.checkpoint
    }
}

pub enum GateRunJournalReadErrorV1 {
    NotFound,
    CheckpointInvalid,
    PrefixInvalid,
    HashChainInvalid,
    SignatureInvalid,
    CandidateRunMismatch,
    Io,
}

pub trait GateRunJournalReadPortV1: Send + Sync {
    fn load_exact_authenticated_prefix(
        &self,
        checkpoint: &VerifiedBusinessArtifactV1<GateRunJournalCheckpointPayloadV1>,
    ) -> Result<VerifiedGateRunJournalPrefixV1, GateRunJournalReadErrorV1>;
}

pub struct ValidatedCandidateManifestPathV1(/* private canonical existing local file */);
pub struct ValidatedL2EvidencePathV1(/* private canonical existing local file */);
pub struct ValidatedBundleRootV1(/* private explicit canonical evidence root */);
pub struct ValidatedRunJournalPathV1(/* private explicit canonical journal file */);
pub struct ValidatedEvidenceOutputPathV1(/* private exact derived output path */);

pub enum VerifyRequestV1 {
    L0Developer { changed_from: String },
    L1PullRequest { changed_from: String },
    L2IntegrationCandidate {
        candidate_manifest_sha256: Sha256Digest,
        candidate_manifest: ValidatedCandidateManifestPathV1,
        bundle_root: ValidatedBundleRootV1,
        run_journal: ValidatedRunJournalPathV1,
        evidence_out: ValidatedEvidenceOutputPathV1,
    },
    L3ReleaseCertification {
        candidate_manifest_sha256: Sha256Digest,
        candidate_manifest: ValidatedCandidateManifestPathV1,
        l2_evidence: ValidatedL2EvidencePathV1,
        bundle_root: ValidatedBundleRootV1,
        run_journal: ValidatedRunJournalPathV1,
        evidence_out: ValidatedEvidenceOutputPathV1,
    },
}

pub enum VerifyDispatchOutcomeV1 {
    Completed { level: EvidenceLevelV1 },
    NotDelivered { level: EvidenceLevelV1 },
}

pub enum VerifyDispatchErrorV1 {
    Syntax,
    PathValidation,
    Selection,
    Execution,
}

pub fn verify(
    request: VerifyRequestV1,
) -> Result<VerifyDispatchOutcomeV1, VerifyDispatchErrorV1> {
    dispatch_closed_verify_request(request)
}

pub enum TestOutcomeV1 { Pass, Fail, NotDelivered, NotCovered }

pub enum FreshPgEvidenceClassV1 { EngineeringRehearsal, CandidateBound }

impl TestOutcomeV1 {
    pub fn satisfies_due_requirement(&self) -> bool { matches!(self, Self::Pass) }
}
```

L0 runs format/lint, graph no-diff, archcheck, and touched unit/property tests. L1 adds affected dependents, due Fresh PostgreSQL profile, generated Rust/TypeScript/OpenAPI equality, and security static negatives. PowerShell only calls `cargo xtask f57 verify` and returns its exit code; workflow YAML contains no verdict logic. CLI parsing exact-maps `--changed-from` only to L0/L1. L2/L3 require the expected 64-lowerhex `--candidate` digest, explicit `--candidate-manifest`, `--bundle-root`, and the exact existing `--run-journal`; `--candidate` always means SHA-256 of the exact signed candidate-manifest envelope bytes, never `CandidateIdentityV1`. At G0 the parser constructs only the private lexical/path wrappers displayed above and returns `NotDelivered`/exit 70; it does not reference `TargetGateV1`, deserialize a future candidate/L2 type or construct `ValidatedCandidateManifestRefV1`. Once G4 lands `crates/platform/release/src/l2.rs`, that sole owner extends the dispatch branch: it typed-loads the explicit candidate file and journal below the root, validates purpose/signature/artifact exact-set/run identity/manifest-bound event, recomputes manifest and identity digests, exact-matches the former to argv, and only then constructs `ValidatedCandidateManifestRefV1` containing its L2-owned `TargetGateV1`. Both later branches require an explicit in-root evidence output and bind their latest signed journal checkpoint. L3 additionally requires the explicit `ValidatedL2EvidencePathV1`, typed-verifies that signed envelope and then constructs `final_l2_evidence_ref`; all L3 paths/root are absolute. No directory scan, default candidate/L2/journal/root path, digest inversion, argv-supplied run ID or G0 mirror of a future proof is allowed. Cross-branch/missing/extra options fail with exit 2.

Task 6 first exact-validates the immutable registry SHA-256 `52930d7ae32ee02ddda38199bcc144f5f6747fcfbe33e740741a0f21604ca8fd`, its class vector `66/3/7/2`, the complete 78+310=388 catalog join, all 66 immutable file hashes, all three registered stale-draft preimage hashes, and absence of the nine baseline-absent plus 310 legacy paths. Only then may it replace the three unpublished draft files in place according to their exact `docs/migration-catalog.md` target contracts. It generates `docs/generated/f57/migration-apply-manifest.v1.json` with exactly 69 baseline entries in canonical version order, path, apply class, effective postimage SHA-256, baseline-registry digest, legacy-seed digest, and baseline count. The manifest contains no F57 row or F57-reservation digest and is never regenerated by a child task. After an engineering Fresh-PG run proves those 69 files from an empty database, the same task changes exactly those three catalog rows from stale `PLANNED` to `EXISTING` and changes the catalog cardinality from `66 EXISTING + 322 PLANNED = 388` to `69 EXISTING + 319 PLANNED = 388`; postimage hashes remain solely in the apply manifest because the catalog has no hash column. It must not promote any absent or F57 row. The task stages the three SQL files, catalog, schema, and generated apply manifest atomically.

`310` is permanently the row count of `docs/f57-legacy-migration-disposition.seed.tsv`; `319` is only the post-rebaseline count of all catalog rows still in `PLANNED` status. Registry validation, CI messages, manifests and receipts must never label the latter as a legacy-seed count or substitute it for `legacy_row_count=310`.

The same atomic staged set changes the catalog's first status paragraph to exactly `F-57 Status: F57_BASELINE_REBASELINED` and states that the 69-file baseline is immutable, the three-draft exception is consumed, and any further baseline rewrite is forbidden. It also changes only the catalog row in the authority-supersession register from `REGISTRY_PENDING_REBASELINE` to `CURRENT_SUBJECT`, with the exact `69 EXISTING + 319 PLANNED` poststate and the apply-manifest binding. The two-state validator accepts the repository's current pending prestate before Task 6 and only this synchronized poststate after Task 6; a postimage catalog with a pending register/header, or vice versa, fails staging and G0.

Task 6 freezes its three new evidence-schema edges rather than inheriting them transitively. `docs/evidence/f57-fresh-pg-evidence.schema.json -> f57-foundation.v1.schema.json`; `docs/evidence/f57-gate-run-journal.v1.schema.json -> f57-foundation.v1.schema.json`; and `docs/evidence/f57-client-conformance-result.v1.schema.json -> f57-foundation.v1.schema.json + f57-requirement-evidence-binding.v1.schema.json`, with the latter importing the requirement owner only for `RequirementCandidateProvenanceV1`. The conformance root uses foundation-owned `ClientStackKindV1`, `TestIdV1`, `CandidateRunIdentityV1`, `UuidV1` and `ArtifactRefV1`, so it must not import the later G5-owned `f57-client-common.v1.schema.json`. Fresh-PG and conformance each directly compose the foundation envelope exactly once; the journal is the gate-owned signed/checkpoint root and imports no result/client/P340/release schema. An omitted direct foundation edge, copied provenance/shared nominal, client-common dependency, extra reverse edge or cycle fails the Task 6 DAG golden.

Before creating a run header or interpreting any F57 evidence signature, Task 6 invokes only foundation `ArtifactVerifierV1::bootstrap_product_pinned`; profile `F57ProductDeployment` resolves a private build-generated pin set covered by the independently signed product manifest/`MANIFEST.sha256`, and the supplied deployment/root bytes must exact-match that set, purpose, EKU, build and authorized rotation transition. It then verifies the committed signer-registry envelope against the resulting deployment trust plus exact registry-authority SPKI/DN, authenticates the fixed local `F57EvidenceSignerV1` broker, requires the sealed active-session registry digest to byte-equal those source bytes, and proves the current runner can resolve every due row. It resolves the pre-root placement class `RUN_BOOTSTRAP_ARTIFACT_SIGNER_REGISTRY_V1`, create-new materializes exact signed registry bytes at `inputs/<exact-envelope-sha256>.json`, fsyncs/reloads/finalizes the two-stage input proof, and constructs the sole registry ref/media. Existing exact bytes are adopted. An arbitrary bundled/self root, wrong deployment/build/profile, rotation bypass, forged descriptor, missing broker, session/source drift, unavailable role, alternate class/name, overwrite, path escape, unsigned bytes or ref/media/size mismatch fails before header creation. `CandidateIdentityV1.artifact_signer_registry_sha256`, every candidate registry ref and every gate receipt exact-match this copy. Each later signed artifact resolves its exact `(artifact_kind,discriminator)` row and submits only digest/verified issuance window/context; the signed registry remains the only lookup-free bootstrap envelope.

`fresh-pg` is implemented once here and never reimplemented by a child plan. It reads only the baseline registry, deterministic migration apply manifest, F57 reservation manifest, bound legacy-disposition seed, and current `docs/f57-fresh-pg-check-registry.v1.tsv`. It creates a disposable PostgreSQL 16 database using credentials supplied outside argv, proves the database is empty, applies the exact 69-file digest-locked baseline in strict version order, and only then applies every contiguous `CREATED` F57 reservation through the requested endpoint. At G0 the endpoint is the exact baseline terminus `20261012113500`; later endpoints are exact F57 reservation versions whose owning profile must equal the requested profile. It rejects a baseline/manifest/catalog/count/hash or current-check-registry drift, an absent path appearing, a missing baseline file, any preceding `RESERVED_NOT_CREATED`, gap, duplicate, wrong-schema file, legacy path on disk, unreserved SQL, unmapped legacy row, origin mismatch, or seed/closure digest drift.

Applicable checks use the master's exact two-predicate prefix rule over the 27-row registry, not every row sharing the requested profile. The compiled dispatcher accepts only each selected row's registered `handler_id`; it never executes registry text, arbitrary argv, or anything from `docs/f57-fresh-pg-task-profiles.seed.tsv`. Each result exact-repeats and row-hashes the selected registry row, uses the closed evidence contract, and requires PASS. An applicable handler whose task is not delivered returns 70; a non-applicable later task in the same profile is not invoked. The disposable database is destroyed only by the creator after result durability. It never targets production or accepts an arbitrary migration path. Candidate-bound gates additionally verify that their signed receipt exact-binds both apply-manifest and current-check-registry digests.

It has exactly two evidence classes. A direct task/pre-commit invocation, or any invocation while the worktree differs from committed `HEAD`, returns only an unsigned `ENGINEERING_REHEARSAL` result and is forbidden from writing a signed receipt. `CANDIDATE_BOUND` is available only through the internal gate/candidate API after proving a clean committed `HEAD`. Before creating or touching the disposable database, the journal durably appends `EVIDENCE_OPERATION_STARTED{artifact_kind=CANDIDATE_BOUND_FRESH_PG,execution_attempt_id=<one unpredictable UUID>,start_context_refs=[]}`. The payload repeats that immutable execution ID. It emits exactly the master `CandidateBoundFreshPgEvidenceV1`: candidate run, the exact aggregate migration-manifest/baseline-registry/apply-manifest/legacy-disposition/current-check-registry digests, profile, through-version, baseline/F57/total counts, PostgreSQL major 16, the complete applicable 27-registry prefix, PASS, precursor journal checkpoint and trusted validity window. The output is create-new/fsynced only at `<parent-of-explicit-run-journal>/fresh-pg/candidate-bound.v1.json`, the exact `EvidenceEnvelopeStoreV1` formula below the bundle root; it is typed-reloaded before `EVIDENCE_OPERATION_COMPLETED`. Recovery from STARTED first records `EVIDENCE_OPERATION_UNKNOWN`, then may adopt exact existing bytes or independently reconcile the original disposable-database identity through `EVIDENCE_OPERATION_RECONCILED`; it never generates another ID or reruns the database operation. All fresh gate receipts and the candidate in that run use the same plain typed ref. Binding an outer signed-candidate digest at this stage is circular and forbidden. The JSON schema and byte-golden require `evidence_class=CANDIDATE_BOUND`; a signed engineering result, dirty/direct result carrying candidate fields, missing/extra/failed/future check, historical-seed execution, registry-digest drift, duplicate per-run receipt, or candidate receipt without clean-HEAD attestation fails closed.

Task 6 creates `ep-platform-gate-journal-contract`, the sole production-linkable owner of every `GateRunJournalV1` wire, strict codec, transition validator, storage-root binding, append/checkpoint port, `JournalPrefixRefV1`, `PowerShutdownContinuationStatePrefixV1`, and P340 kind/output nominal. It is foundation-only. `crates/adapter/file/src/gate_run_journal_store.rs` is the sole durable filesystem engine; `xtask/src/f57/run_journal.rs` owns only CLI composition and the registered reconciler map, while later core-server services consume the same contract and file port. Neither side copies, re-exports or path-includes a wire, and no production crate depends on `xtask`. The neutral contract reserves and strict-parses at G0 the complete thirteen later delta variants—five P340 qualification, five POWER continuation and three carrier-completion finalization variants—while execution remains `NOT_DELIVERED` until Tasks 13/14 install the closed state-machine handlers. The schema remains the sole JSON owner and imports only foundation; it never imports P340 or release. Byte goldens freeze every field set, event wire, transition and cross-family negative before any later caller exists.

The same task owns the three run output authorities and separate checkpoint store. Runtime owns generated `EvidencePlacementClassV1` plus occurrence-to-class bindings and both production-linkable immutable-store ports. Object creation calls `resolve_class` before a containing root exists; a later occurrence calls `resolve` and must name the same class. For signed input, `resolve_class_location|resolve_location` yields only canonical class/locator/media; `load_exact_unverified` returns private `LoadedEvidenceInputV1`, whose bytes may be passed only to the owning ArtifactVerifier. After verification, `verify_trust` exact-matches the class's stable domain/purpose/media/validity rule against the verifier-derived nine-field descriptor and binds its dynamic policy digest; `finalize_loaded(loaded,contract,verified)` exact-compares ref/bytes/class/media and alone yields `VerifiedLoadedEvidenceInputV1` for semantic use. Materialization likewise requires verified bytes plus the full contract. Thus cold offline load has no proof-before-read cycle and another envelope's policy token cannot bless loaded bytes. Neither a generated static row nor a caller freezes a deployment-specific registry/root digest. The two file adapters share one create-new/adopt/fsync/reparse-safe engine and only private `ValidatedBundleRootV1` tooling constructors at G0; G1-01 adds distinct `ValidatedDataRootV1` constructors without changing ports. Goldens cover rotation, cross-deployment/registry, forged/reused proof, pre-finalization semantic use, absent/I/O/partial/conflict, class/digest/media drift and cold offline reconstruction.

The generated Rust registry freezes these two Task-14-facing APIs at G0; their return values retain the authoritative source row's `binding_ordinal` or `placement_class_ordinal`, so later code never invents or copies an ordinal:

```rust
pub mod generated_evidence_bindings {
    // Source key: containing_root_id=RELEASE_CANDIDATE_V1,
    // field_selector=/runtime_topology_declaration_ref, vector_role=NONE.
    pub fn release_candidate_runtime_topology_declaration_ref_v1(
    ) -> &'static EvidenceObjectFieldBindingV1;

    // Source key: placement class for plain RuntimeTopologyCertificationV1,
    // exact media application/vnd.ep.f57-runtime-topology-certification-v1+json.
    pub fn runtime_topology_certification_v1(
    ) -> &'static EvidencePlacementClassV1;
}
```

The source-freeze test exact-joins each accessor to one and only one TSV row and asserts its numeric ordinal, class ID, media and delivery task byte-equal that row. Renaming the accessor, selecting by free text, hand-writing an ordinal in G6 or resolving either object with a generic media contract fails compilation or the independent registry golden.

`AuthorityStorageBootstrapArchiveStoreV1` is the third, explicitly separate immutable placement lane. It derives exactly seven fixed paths: `inputs/storage-root/authority-storage-manifest.v1.json` plus `deployment-manifest.v1.json|deployment-manifest.p7s|deployment-trust-bundle.p7b|storage-trust-root.der|storage-revocation.bin|storage-checkpoint.bin` below the same directory. Its seven closed role/media descriptors use create-new/adopt/fsync/reload and crash-cut recovery; it accepts no eighth fixed row or caller filename. The storage-manifest envelope is domain-verified before placement, and the G6 header binds the fixed archive ref plus six bootstrap refs for historical boot authority. A final G6 bundle also needs the same envelope at its registered digest-named `EvidenceInputStoreV1` locator because immutable signed generation/declaration artifacts already reference that URI. This is the only dual-locator exception: both refs must have byte-equal media/size/digest/exact bytes, the fixed ref serves only header/bootstrap history, the digest ref serves only generation/declaration portability, offline traversal retains both paths while deduplicating content identity, and any third alias or cross-role substitution fails.

The seven descriptors are exact and shared by the archive/header/offline closure; extensions never infer media or parser:

| Role | Fixed filename | Exact media type | Exact parser / target contract |
|---|---|---|---|
| `AUTHORITY_STORAGE_MANIFEST` | `authority-storage-manifest.v1.json` | `application/vnd.ep.f57-authority-storage-manifest-v1+json` | `F57_AUTHORITY_STORAGE_MANIFEST_ENVELOPE_V1` |
| `DEPLOYMENT_MANIFEST` | `deployment-manifest.v1.json` | `application/vnd.ep.deployment-manifest-v1+json` | `F56_DEPLOYMENT_MANIFEST_JCS_V1` |
| `DEPLOYMENT_MANIFEST_SIGNATURE` | `deployment-manifest.p7s` | `application/pkcs7-signature` | `CMS_DETACHED_SIGNED_DATA_DER_V1` |
| `DEPLOYMENT_TRUST_BUNDLE` | `deployment-trust-bundle.p7b` | `application/pkcs7-mime` | `CMS_CERTIFICATE_SET_DER_V1` |
| `STORAGE_TRUST_ROOT` | `storage-trust-root.der` | `application/pkix-cert` | `X509_CERTIFICATE_DER_V1` |
| `STORAGE_REVOCATION` | `storage-revocation.bin` | `application/octet-stream` | `F57_STORAGE_REVOCATION_STATE_V1` |
| `STORAGE_CHECKPOINT` | `storage-checkpoint.bin` | `application/octet-stream` | `F57_STORAGE_ANTI_ROLLBACK_CHECKPOINT_V1` |

For an absent G6 header, the only pre-header immutable identities are the digest-named registry under `RUN_BOOTSTRAP_ARTIFACT_SIGNER_REGISTRY_V1` plus these seven archive rows—exactly eight. The creator may additionally use only its unique journal temporary. It must verify registry and broker-session equality first, verify the seven-file storage root second, create-new copy/fsync/reload all eight, and only then construct/sign the header. A crash adopts the same eight bytes; a ninth input, alternate registry class, partial group or header-before-reload fails. After the header exists, all seven CLI bootstrap flags are forbidden and recovery resolves only its authenticated registry/archive refs.

`docs/f57-evidence-placement-classes.v1.tsv` is the independently reviewed class authority with exact columns `placement_class_ordinal,placement_class_id,location_class,media_type,trust_domain_id,required_purpose,validity_rule,bootstrap_role,delivery_task`; object classes use exact `NONE` in all three trust columns. `docs/f57-evidence-field-contracts.v1.tsv` is the occurrence authority with exact columns `binding_ordinal,containing_root_id,field_selector,vector_role,placement_class_ordinal,delivery_task`. It has one canonical row for every in-bundle `ArtifactRefV1` occurrence in the complete master wire set except named run/checkpoint/offline/archive stores; every row exact-joins one class. Source review freezes complete rows before generator code, records source/class/object/input counts plus source/output/manifest SHA-256 values and two distinct reviewers in `evidence-contract-source-freeze.v1.json`, and forbids implementation from regenerating that approval object. Task 6 then generates placement/object/input JSON registries, runtime Rust tables and an output manifest; independent parsers compare them to the frozen source, while later task schemas exact-join occurrences before activation. Missing/extra class/selector, duplicate ordinal or root-pointer-role tuple, unregistered vector role, wrong class/lane/media/trust, bootstrap class borrowed by another type, count/digest drift or schema reachability mismatch fails L0/L1. Changing rows requires a master amendment and new reviewed source freeze, never an ad hoc child edit.

For every registered TestID, the engine durably appends `TEST_STARTED` with one CSPRNG `execution_attempt_id` and the registry-derived canonical `start_context_refs` before physical invocation. Ordinary Requirement/probe/client tests use exact `[]`; every release carrier, including P340, uses exact singleton `[staging_plan_ref]`, with all additional typed policy/capacity inputs reachable only through that signed plan. Result payload and every terminal/unknown/reconciled record repeat the same execution ID. The only legal paths are `ABSENT -> STARTED -> COMPLETED` or, after crash, `ABSENT -> STARTED -> UNKNOWN -> RECONCILED`; recovery invokes only the registered reconciler for that original attempt and never reruns or selects a favorable result. The journal-wide Windows-safe exclusive lease covers state read, STARTED durability, invocation, result-store durability, terminal append and checkpoint.

The run output stores remain disjoint and closed. `TestResultStoreV1` exact-builds the 276-row descriptor map and derives each result only as `runs/<gate-run-uuid>/test-results/<lowercase-TestID>.v1.json`. `EvidenceEnvelopeStoreV1` exact-builds the 14-row profile/kind map, while `CandidateManifestStoreV1` exact-builds the separate three-row candidate map, both relative to the canonical parent of the explicit `gate-run.jcs.jsonl`; same-run CLI paths and gate output directories must equal those formulas. `EvidenceEnvelopeKindV1::OfflineSchemaManifest` is intentionally absent from the 14-row map and later resolves only through the G6-owned `OfflineSchemaManifestStoreV1`. ObjectStore derives only registered immutable `objects/sha256/<digest>` locations; InputStore derives only registered verified signed-envelope `inputs/<whole-envelope-digest>.json` locations; the bootstrap archive derives exactly seven fixed `inputs/storage-root/*` paths. None accepts a caller-selected filename, scans, overwrites, aliases a lane, or infers media/trust policy.

Deterministic aggregates—client validation/artifact set, offline schema manifest through its later special store, L2/L3, G0…G5 receipts and release certificate—never use physical-operation STARTED records. After all constituent TestIDs/operations are terminal, the producer writes a checkpoint that freezes the complete input prefix, then durably appends exactly one `EVIDENCE_ENVELOPE_FINALIZATION_STARTED{artifact_kind,finalization_attempt_id,frozen_input_checkpoint_ref,issued_at_unix_ms,expires_at_unix_ms}` before first signing/write. Candidate manifests use the parallel exact five-field `CANDIDATE_MANIFEST_FINALIZATION_STARTED{candidate_kind,finalization_attempt_id,frozen_input_checkpoint_ref,generation_observed_selection_ref,issued_at_unix_ms}`; `generation_observed_selection_ref` is JSON `null` for G4/G5 and is a nonnull, already materialized exact selection record only for `FINAL_RELEASE`. Their payload repeats `finalization_attempt_id`; final release additionally repeats that nonnull selection ref. Bound or reconciled records repeat the same ID and exact store-derived candidate ref. Recovery may perform the first signing from frozen inputs or adopt existing exact bytes without re-signing; a second attempt, changed time/checkpoint/selection/ref, missing output after bound, or conflicting bytes quarantines and fails. A second header can never take over a run directory. G0's gate creates its explicit journal and follows CandidateBound Fresh-PG → due/probe results → checkpoint → receipt finalization → receipt bound; later tasks reuse this engine and may not add another execution index, checkpoint path or output store. `carrier run` is also implemented once here: its closed parser and dispatcher reject arbitrary commands and return 70 for all six registered release recipes until G6 Task 14 activates their compiled script/result/reconciler mappings.

- [ ] **Step 4: Run implementation tests and pre-commit L0/L1.**

Run: `cargo test -p ep-platform-gate-journal-contract --all-targets --locked -- --nocapture`

Run: `cargo test -p ep-xtask --all-targets --locked --test f57_levels --test f57_fresh_pg --test f57_run_journal --test f57_client_conformance_dispatch -- --nocapture`

Expected: PASS, including exact active-session/source-registry equality before the first journal signature, row/runner-bound authorized broker signing and missing/session-drift/unavailable-role failure before output; plus the neutral production-linkable journal owner, all thirteen reserved delta variants, exact seven P340 kinds/two helper nominals/five event field sets, foundation-only schema edge, storage-root wire, transition/byte goldens, and `NOT_DELIVERED` semantic guards.

Run: `cargo test -p ep-platform-runtime --all-targets --locked --test evidence_object_store_port --test evidence_input_store_port -- --nocapture`

Run: `cargo test -p ep-adapter-file --all-targets --locked --test evidence_object_store --test evidence_input_store --test gate_run_journal_store --test authority_storage_bootstrap_archive -- --nocapture`

Run: `cargo xtask f57 evidence-bindings generate --check`

Expected: PASS for both immutable-store ports, placement-class-before-occurrence creation, the three exact bootstrap class accessors, two-stage cold input load/finalization, reused-proof rejection, the exact seven-row role/filename/media/parser archive table and sole adapters; object/input/fixed-archive URI separation; exactly eight legal G6 pre-header immutable identities and header-after-reload ordering; create-new/adopt/fsync and absent/I/O/partial/conflict/path-escape negatives; verifier-only trust-policy proofs and rotation/cross-deployment/cross-registry rejection; independently frozen placement/object/input rows, counts and source/output/manifest digests; the private `ValidatedBundleRootV1` tooling lane; explicit absence of a premature `ValidatedDataRootV1` reference at G0; and workspace proof that no production crate depends on `xtask`. Authority-root constructors and cross-root parity remain G1-01 work.

Run: `cargo xtask f57 verify --level l0 --changed-from HEAD^`

Expected: PASS with a nonempty selected-test list.

Run: `cargo xtask f57 verify --level l1 --changed-from HEAD^`

Expected: PASS; any due Fresh PG prerequisite that is unavailable returns 3, never PASS.

Run: `cargo xtask f57 fresh-pg --profile G0_BOOTSTRAP --through 20261012113500`

Expected: unsigned engineering rehearsal PASS with exactly 69 baseline migrations, zero F57 migrations, 69 total, and no absent/superseded path applied.

- [ ] **Step 5: Commit G0 evidence selection.**

```bash
cargo xtask f57 task stage --task G0-06
cargo xtask f57 task verify-staged --task G0-06
git commit -m "ci: establish f57 l0 l1 bootstrap gate"
```

- [ ] **Step 6: Issue the gate only from clean committed HEAD.**

Run: `git status --porcelain=v1`

Expected: no output. A dirty or untracked source/generated file is a hard stop; gate evidence writes only below ignored `target/f57/evidence/`.

Run: `cargo xtask f57 verify --level l1 --changed-from HEAD^`

Run: `cargo xtask f57 gate g0 --bundle-root target/f57/evidence --run-journal target/f57/evidence/g0/gate-run.jcs.jsonl --evidence-out target/f57/evidence/g0`

Expected: the gate internally reruns candidate-bound Fresh-PG through the baseline endpoint and exits 0. Its `G0_BOOTSTRAP_GREEN` receipt binds committed `HEAD`, one repository tree digest, graph digest, generator version, the exact three-root `13/9/14` generation schema/parser/whole-envelope-digest/reverse-invariant contract digest plus the separate seven-field/three-row approval-registry/bootstrap contract digest—but no production registry, manifest, reverse plan or ACK ref—the two-stage runtime-topology schema/pure-builder/live-verifier contract digests—but no deployment declaration or certification ref—185-row delivery digest, frozen `first_due_map_sha256=a9547557f95a3a9892efa9f6751a0dd03accac65da344aa559a3203488fee086`, baseline-registry SHA-256 `52930d7ae32ee02ddda38199bcc144f5f6747fcfbe33e740741a0f21604ca8fd`, legacy-seed SHA-256 `06566ca354b6279391e5ec3a0152316a8eb38d1f10cb09dc23953370883c3196`, migration-apply-manifest digest, task-path-registry digest, and exact counts `baseline=69,f57=0,total=69`. It has exact `prerequisite_receipts=[]` and `objective_closures=[]`, references the in-bundle exact signed 89-row signer registry whose digest equals `CandidateIdentityV1.artifact_signer_registry_sha256`, and uses the checkpoint frozen by its one receipt-finalization record. The dedicated Fresh-PG reference carries `candidate_identity_sha256=sha256(JCS(CandidateIdentityV1))` and the same `gate_run_id`. No repository file changes occur after receipt issuance.

## G0 read-only completion check

Run these commands from a clean candidate tree:

```bash
cargo fmt --all -- --check
cargo xtask archcheck
cargo xtask sqlcheck
cargo xtask f57 graph generate --check
cargo test -p ep-platform-capability-graph --all-targets
cargo test -p ep-foundation --test signature
cargo test -p ep-platform-release --all-targets --locked --test generation --test generation_approval
cargo test -p ep-platform-gate-journal-contract --all-targets --locked
cargo test -p ep-platform-runtime --all-targets --locked --test topology --test evidence_object_store_port --test evidence_input_store_port
cargo test -p ep-adapter-file --all-targets --locked --test evidence_object_store --test evidence_input_store --test gate_run_journal_store --test authority_storage_bootstrap_archive
cargo test -p ep-xtask --all-targets --locked --test f57_registry --test f57_architecture --test f57_levels --test f57_fresh_pg --test f57_run_journal --test f57_client_conformance_dispatch
cargo xtask f57 evidence-bindings generate --check
cargo check --workspace --all-targets --locked
cargo xtask f57 evidence verify --receipt target/f57/evidence/g0/bootstrap-receipt.v1.json --bundle-root target/f57/evidence --expect-gate G0_BOOTSTRAP_GREEN
```

Expected final state: the already issued `G0_BOOTSTRAP_GREEN` receipt verifies without rerunning any gate or Requirement TestID. This state proves only that development authority and contracts are coherent. It does not prove business persistence, Windows installation, four clients, real CNG signing, P340 capacity, backup, recovery, or production readiness.
